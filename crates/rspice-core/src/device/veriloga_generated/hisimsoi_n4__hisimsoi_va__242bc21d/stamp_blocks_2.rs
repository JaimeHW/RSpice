#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        locals: &mut StampLocals,
    ) {
        let (assign11130_e12835,) = {
    if (locals.var_guard109 != 0.0) {
        let (assign11130_e12833,) = {
            if (locals.var_flg_brk8 > 0.0) {
                (locals.var_flg_brk8,)
            } else {
                (locals.var_lp_s0,)
            }
        };
        (assign11130_e12833,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign11130_e12835;

        let assign11140_e12838: f64 = if locals.var_flg_conv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign11140_e12838;

        let (assign11150_e12844, assign11150_e12844_d_n0, assign11150_e12844_d_n2, assign11150_e12844_d_n6, assign11150_e12844_d_n7, assign11150_e12844_d_n10, assign11150_e12844_d_n11, assign11150_e12844_d_n12, assign11150_e12844_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard299 != 0.0)) {
        (locals.var_phi_s0_soi_ini, locals.var_phi_s0_soi_ini_dn0, locals.var_phi_s0_soi_ini_dn2, locals.var_phi_s0_soi_ini_dn6, locals.var_phi_s0_soi_ini_dn7, locals.var_phi_s0_soi_ini_dn10, locals.var_phi_s0_soi_ini_dn11, locals.var_phi_s0_soi_ini_dn12, locals.var_phi_s0_soi_ini_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign11150_e12844;
        locals.var_phi_s0_soi_dn0 = assign11150_e12844_d_n0;
        locals.var_phi_s0_soi_dn2 = assign11150_e12844_d_n2;
        locals.var_phi_s0_soi_dn6 = assign11150_e12844_d_n6;
        locals.var_phi_s0_soi_dn7 = assign11150_e12844_d_n7;
        locals.var_phi_s0_soi_dn10 = assign11150_e12844_d_n10;
        locals.var_phi_s0_soi_dn11 = assign11150_e12844_d_n11;
        locals.var_phi_s0_soi_dn12 = assign11150_e12844_d_n12;
        locals.var_phi_s0_soi_dn17 = assign11150_e12844_d_n17;

        let (assign11160_e12850, assign11160_e12850_d_n0, assign11160_e12850_d_n2, assign11160_e12850_d_n6, assign11160_e12850_d_n7, assign11160_e12850_d_n10, assign11160_e12850_d_n11, assign11160_e12850_d_n12, assign11160_e12850_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard299 != 0.0)) {
        (locals.var_phi_b0_soi_ini, locals.var_phi_b0_soi_ini_dn0, locals.var_phi_b0_soi_ini_dn2, locals.var_phi_b0_soi_ini_dn6, locals.var_phi_b0_soi_ini_dn7, locals.var_phi_b0_soi_ini_dn10, locals.var_phi_b0_soi_ini_dn11, locals.var_phi_b0_soi_ini_dn12, locals.var_phi_b0_soi_ini_dn17,)
    } else {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    }
};
        locals.var_phi_b0_soi = assign11160_e12850;
        locals.var_phi_b0_soi_dn0 = assign11160_e12850_d_n0;
        locals.var_phi_b0_soi_dn2 = assign11160_e12850_d_n2;
        locals.var_phi_b0_soi_dn6 = assign11160_e12850_d_n6;
        locals.var_phi_b0_soi_dn7 = assign11160_e12850_d_n7;
        locals.var_phi_b0_soi_dn10 = assign11160_e12850_d_n10;
        locals.var_phi_b0_soi_dn11 = assign11160_e12850_d_n11;
        locals.var_phi_b0_soi_dn12 = assign11160_e12850_d_n12;
        locals.var_phi_b0_soi_dn17 = assign11160_e12850_d_n17;

        let (assign11170_e12856, assign11170_e12856_d_n0, assign11170_e12856_d_n2, assign11170_e12856_d_n6, assign11170_e12856_d_n7, assign11170_e12856_d_n10, assign11170_e12856_d_n11, assign11170_e12856_d_n12, assign11170_e12856_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard299 != 0.0)) {
        (locals.var_phi_s0_bulk_ini, locals.var_phi_s0_bulk_ini_dn0, locals.var_phi_s0_bulk_ini_dn2, locals.var_phi_s0_bulk_ini_dn6, locals.var_phi_s0_bulk_ini_dn7, locals.var_phi_s0_bulk_ini_dn10, locals.var_phi_s0_bulk_ini_dn11, locals.var_phi_s0_bulk_ini_dn12, locals.var_phi_s0_bulk_ini_dn17,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign11170_e12856;
        locals.var_phi_s0_bulk_dn0 = assign11170_e12856_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign11170_e12856_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign11170_e12856_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign11170_e12856_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign11170_e12856_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign11170_e12856_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign11170_e12856_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign11170_e12856_d_n17;

        let (assign11180_e12860, assign11180_e12860_d_n0, assign11180_e12860_d_n2, assign11180_e12860_d_n6, assign11180_e12860_d_n7, assign11180_e12860_d_n10, assign11180_e12860_d_n11, assign11180_e12860_d_n12, assign11180_e12860_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign11180_e12860;
        locals.var_ps0_dn0 = assign11180_e12860_d_n0;
        locals.var_ps0_dn2 = assign11180_e12860_d_n2;
        locals.var_ps0_dn6 = assign11180_e12860_d_n6;
        locals.var_ps0_dn7 = assign11180_e12860_d_n7;
        locals.var_ps0_dn10 = assign11180_e12860_d_n10;
        locals.var_ps0_dn11 = assign11180_e12860_d_n11;
        locals.var_ps0_dn12 = assign11180_e12860_d_n12;
        locals.var_ps0_dn17 = assign11180_e12860_d_n17;

        let (assign11190_e12865, assign11190_e12865_d_n0, assign11190_e12865_d_n2, assign11190_e12865_d_n6, assign11190_e12865_d_n7, assign11190_e12865_d_n10, assign11190_e12865_d_n11, assign11190_e12865_d_n12, assign11190_e12865_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign11190_e12863: f64 = (-locals.var_q_n0);
        (assign11190_e12863, (-locals.var_q_n0_dn0), (-locals.var_q_n0_dn2), (-locals.var_q_n0_dn6), (-locals.var_q_n0_dn7), (-locals.var_q_n0_dn10), (-locals.var_q_n0_dn11), (-locals.var_q_n0_dn12), (-locals.var_q_n0_dn17),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign11190_e12865;
        locals.var_qn0_dn0 = assign11190_e12865_d_n0;
        locals.var_qn0_dn2 = assign11190_e12865_d_n2;
        locals.var_qn0_dn6 = assign11190_e12865_d_n6;
        locals.var_qn0_dn7 = assign11190_e12865_d_n7;
        locals.var_qn0_dn10 = assign11190_e12865_d_n10;
        locals.var_qn0_dn11 = assign11190_e12865_d_n11;
        locals.var_qn0_dn12 = assign11190_e12865_d_n12;
        locals.var_qn0_dn17 = assign11190_e12865_d_n17;

        let assign11200_e12868: f64 = if locals.var_qn0 <= 1e-50 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign11200_e12868;

        let (assign11210_e12874, assign11210_e12874_d_n0, assign11210_e12874_d_n2, assign11210_e12874_d_n6, assign11210_e12874_d_n7, assign11210_e12874_d_n10, assign11210_e12874_d_n11, assign11210_e12874_d_n12, assign11210_e12874_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard300 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign11210_e12874;
        locals.var_qn0_dn0 = assign11210_e12874_d_n0;
        locals.var_qn0_dn2 = assign11210_e12874_d_n2;
        locals.var_qn0_dn6 = assign11210_e12874_d_n6;
        locals.var_qn0_dn7 = assign11210_e12874_d_n7;
        locals.var_qn0_dn10 = assign11210_e12874_d_n10;
        locals.var_qn0_dn11 = assign11210_e12874_d_n11;
        locals.var_qn0_dn12 = assign11210_e12874_d_n12;
        locals.var_qn0_dn17 = assign11210_e12874_d_n17;

        let (assign11230_e12884, assign11230_e12884_d_n0, assign11230_e12884_d_n2, assign11230_e12884_d_n6, assign11230_e12884_d_n7, assign11230_e12884_d_n10, assign11230_e12884_d_n11, assign11230_e12884_d_n12, assign11230_e12884_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign11230_e12882: f64 = (locals.var_qn0 * locals.var_c_fox_inv);
        (assign11230_e12882, ((locals.var_qn0_dn0 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn2)), ((locals.var_qn0_dn6 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn7)), ((locals.var_qn0_dn10 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn10)), ((locals.var_qn0_dn11 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn11)), ((locals.var_qn0_dn12 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn12)), ((locals.var_qn0_dn17 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign11230_e12884;
        locals.var_vgvt_dn0 = assign11230_e12884_d_n0;
        locals.var_vgvt_dn2 = assign11230_e12884_d_n2;
        locals.var_vgvt_dn6 = assign11230_e12884_d_n6;
        locals.var_vgvt_dn7 = assign11230_e12884_d_n7;
        locals.var_vgvt_dn10 = assign11230_e12884_d_n10;
        locals.var_vgvt_dn11 = assign11230_e12884_d_n11;
        locals.var_vgvt_dn12 = assign11230_e12884_d_n12;
        locals.var_vgvt_dn17 = assign11230_e12884_d_n17;

        let assign11240_e12889: f64 = if ((locals.var_phi_s0_soi <= 0.0) && (locals.var_flg_skipacc != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard301 = assign11240_e12889;

        let (assign11260_e12904, assign11260_e12904_d_n0, assign11260_e12904_d_n2, assign11260_e12904_d_n6, assign11260_e12904_d_n7, assign11260_e12904_d_n10, assign11260_e12904_d_n11, assign11260_e12904_d_n12, assign11260_e12904_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign11260_e12900: f64 = (-locals.var_weffcv_nf);
        let assign11260_e12902: f64 = (assign11260_e12900 * locals.var_leff_cv);
        (assign11260_e12902, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign11260_e12904;
        locals.var_t0_dn0 = assign11260_e12904_d_n0;
        locals.var_t0_dn2 = assign11260_e12904_d_n2;
        locals.var_t0_dn6 = assign11260_e12904_d_n6;
        locals.var_t0_dn7 = assign11260_e12904_d_n7;
        locals.var_t0_dn10 = assign11260_e12904_d_n10;
        locals.var_t0_dn11 = assign11260_e12904_d_n11;
        locals.var_t0_dn12 = assign11260_e12904_d_n12;
        locals.var_t0_dn17 = assign11260_e12904_d_n17;

        let (assign11270_e12910, assign11270_e12910_d_n0, assign11270_e12910_d_n2, assign11270_e12910_d_n6, assign11270_e12910_d_n7, assign11270_e12910_d_n10, assign11270_e12910_d_n11, assign11270_e12910_d_n12, assign11270_e12910_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_q_s0_dep_ini, locals.var_q_s0_dep_ini_dn0, locals.var_q_s0_dep_ini_dn2, locals.var_q_s0_dep_ini_dn6, locals.var_q_s0_dep_ini_dn7, locals.var_q_s0_dep_ini_dn10, locals.var_q_s0_dep_ini_dn11, locals.var_q_s0_dep_ini_dn12, locals.var_q_s0_dep_ini_dn17,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn12, locals.var_q_sl_dep_dn17,)
    }
};
        locals.var_q_sl_dep = assign11270_e12910;
        locals.var_q_sl_dep_dn0 = assign11270_e12910_d_n0;
        locals.var_q_sl_dep_dn2 = assign11270_e12910_d_n2;
        locals.var_q_sl_dep_dn6 = assign11270_e12910_d_n6;
        locals.var_q_sl_dep_dn7 = assign11270_e12910_d_n7;
        locals.var_q_sl_dep_dn10 = assign11270_e12910_d_n10;
        locals.var_q_sl_dep_dn11 = assign11270_e12910_d_n11;
        locals.var_q_sl_dep_dn12 = assign11270_e12910_d_n12;
        locals.var_q_sl_dep_dn17 = assign11270_e12910_d_n17;

        let (assign11280_e12916, assign11280_e12916_d_n0, assign11280_e12916_d_n2, assign11280_e12916_d_n6, assign11280_e12916_d_n7, assign11280_e12916_d_n10, assign11280_e12916_d_n11, assign11280_e12916_d_n12, assign11280_e12916_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn12, locals.var_q_b0_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
        locals.var_q_bl_dep = assign11280_e12916;
        locals.var_q_bl_dep_dn0 = assign11280_e12916_d_n0;
        locals.var_q_bl_dep_dn2 = assign11280_e12916_d_n2;
        locals.var_q_bl_dep_dn6 = assign11280_e12916_d_n6;
        locals.var_q_bl_dep_dn7 = assign11280_e12916_d_n7;
        locals.var_q_bl_dep_dn10 = assign11280_e12916_d_n10;
        locals.var_q_bl_dep_dn11 = assign11280_e12916_d_n11;
        locals.var_q_bl_dep_dn12 = assign11280_e12916_d_n12;
        locals.var_q_bl_dep_dn17 = assign11280_e12916_d_n17;

        let (assign11290_e12924, assign11290_e12924_d_n0, assign11290_e12924_d_n2, assign11290_e12924_d_n6, assign11290_e12924_d_n7, assign11290_e12924_d_n10, assign11290_e12924_d_n11, assign11290_e12924_d_n12, assign11290_e12924_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign11290_e12922: f64 = (locals.var_q_sl_dep + locals.var_q_bl_dep);
        (assign11290_e12922, (locals.var_q_sl_dep_dn0 + locals.var_q_bl_dep_dn0), (locals.var_q_sl_dep_dn2 + locals.var_q_bl_dep_dn2), (locals.var_q_sl_dep_dn6 + locals.var_q_bl_dep_dn6), (locals.var_q_sl_dep_dn7 + locals.var_q_bl_dep_dn7), (locals.var_q_sl_dep_dn10 + locals.var_q_bl_dep_dn10), (locals.var_q_sl_dep_dn11 + locals.var_q_bl_dep_dn11), (locals.var_q_sl_dep_dn12 + locals.var_q_bl_dep_dn12), (locals.var_q_sl_dep_dn17 + locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
        locals.var_q_depl = assign11290_e12924;
        locals.var_q_depl_dn0 = assign11290_e12924_d_n0;
        locals.var_q_depl_dn2 = assign11290_e12924_d_n2;
        locals.var_q_depl_dn6 = assign11290_e12924_d_n6;
        locals.var_q_depl_dn7 = assign11290_e12924_d_n7;
        locals.var_q_depl_dn10 = assign11290_e12924_d_n10;
        locals.var_q_depl_dn11 = assign11290_e12924_d_n11;
        locals.var_q_depl_dn12 = assign11290_e12924_d_n12;
        locals.var_q_depl_dn17 = assign11290_e12924_d_n17;

        let (assign11300_e12935, assign11300_e12935_d_n0, assign11300_e12935_d_n2, assign11300_e12935_d_n6, assign11300_e12935_d_n7, assign11300_e12935_d_n10, assign11300_e12935_d_n11, assign11300_e12935_d_n12, assign11300_e12935_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign11300_e12929: f64 = (-0.5);
        let assign11300_e12932: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign11300_e12933: f64 = (assign11300_e12929 * assign11300_e12932);
        (assign11300_e12933, (assign11300_e12929 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (assign11300_e12929 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (assign11300_e12929 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (assign11300_e12929 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (assign11300_e12929 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (assign11300_e12929 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (assign11300_e12929 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (assign11300_e12929 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign11300_e12935;
        locals.var_qbu_dn0 = assign11300_e12935_d_n0;
        locals.var_qbu_dn2 = assign11300_e12935_d_n2;
        locals.var_qbu_dn6 = assign11300_e12935_d_n6;
        locals.var_qbu_dn7 = assign11300_e12935_d_n7;
        locals.var_qbu_dn10 = assign11300_e12935_d_n10;
        locals.var_qbu_dn11 = assign11300_e12935_d_n11;
        locals.var_qbu_dn12 = assign11300_e12935_d_n12;
        locals.var_qbu_dn17 = assign11300_e12935_d_n17;

        let (assign11310_e12943, assign11310_e12943_d_n0, assign11310_e12943_d_n2, assign11310_e12943_d_n6, assign11310_e12943_d_n7, assign11310_e12943_d_n10, assign11310_e12943_d_n11, assign11310_e12943_d_n12, assign11310_e12943_d_n13, assign11310_e12943_d_n15, assign11310_e12943_d_n16, assign11310_e12943_d_n17, assign11310_e12943_d_n18,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign11310_e12941: f64 = (locals.var_t0 * locals.var_qbu);
        (assign11310_e12941, ((locals.var_t0_dn0 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn0)), ((locals.var_t0_dn2 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn2)), ((locals.var_t0_dn6 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn6)), ((locals.var_t0_dn7 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn7)), ((locals.var_t0_dn10 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn10)), ((locals.var_t0_dn11 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn11)), ((locals.var_t0_dn12 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t0_dn17 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign11310_e12943;
        locals.var_qb_dn0 = assign11310_e12943_d_n0;
        locals.var_qb_dn2 = assign11310_e12943_d_n2;
        locals.var_qb_dn6 = assign11310_e12943_d_n6;
        locals.var_qb_dn7 = assign11310_e12943_d_n7;
        locals.var_qb_dn10 = assign11310_e12943_d_n10;
        locals.var_qb_dn11 = assign11310_e12943_d_n11;
        locals.var_qb_dn12 = assign11310_e12943_d_n12;
        locals.var_qb_dn13 = assign11310_e12943_d_n13;
        locals.var_qb_dn15 = assign11310_e12943_d_n15;
        locals.var_qb_dn16 = assign11310_e12943_d_n16;
        locals.var_qb_dn17 = assign11310_e12943_d_n17;
        locals.var_qb_dn18 = assign11310_e12943_d_n18;

        let (assign11320_e12951, assign11320_e12951_d_n0, assign11320_e12951_d_n2, assign11320_e12951_d_n6, assign11320_e12951_d_n7, assign11320_e12951_d_n10, assign11320_e12951_d_n11, assign11320_e12951_d_n12, assign11320_e12951_d_n13, assign11320_e12951_d_n15, assign11320_e12951_d_n16, assign11320_e12951_d_n17, assign11320_e12951_d_n18,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign11320_e12949: f64 = (locals.var_qb * 0.5);
        (assign11320_e12949, (locals.var_qb_dn0 * 0.5), (locals.var_qb_dn2 * 0.5), (locals.var_qb_dn6 * 0.5), (locals.var_qb_dn7 * 0.5), (locals.var_qb_dn10 * 0.5), (locals.var_qb_dn11 * 0.5), (locals.var_qb_dn12 * 0.5), (locals.var_qb_dn13 * 0.5), (locals.var_qb_dn15 * 0.5), (locals.var_qb_dn16 * 0.5), (locals.var_qb_dn17 * 0.5), (locals.var_qb_dn18 * 0.5),)
    } else {
        (locals.var_qd_fb, locals.var_qd_fb_dn0, locals.var_qd_fb_dn2, locals.var_qd_fb_dn6, locals.var_qd_fb_dn7, locals.var_qd_fb_dn10, locals.var_qd_fb_dn11, locals.var_qd_fb_dn12, locals.var_qd_fb_dn13, locals.var_qd_fb_dn15, locals.var_qd_fb_dn16, locals.var_qd_fb_dn17, locals.var_qd_fb_dn18,)
    }
};
        locals.var_qd_fb = assign11320_e12951;
        locals.var_qd_fb_dn0 = assign11320_e12951_d_n0;
        locals.var_qd_fb_dn2 = assign11320_e12951_d_n2;
        locals.var_qd_fb_dn6 = assign11320_e12951_d_n6;
        locals.var_qd_fb_dn7 = assign11320_e12951_d_n7;
        locals.var_qd_fb_dn10 = assign11320_e12951_d_n10;
        locals.var_qd_fb_dn11 = assign11320_e12951_d_n11;
        locals.var_qd_fb_dn12 = assign11320_e12951_d_n12;
        locals.var_qd_fb_dn13 = assign11320_e12951_d_n13;
        locals.var_qd_fb_dn15 = assign11320_e12951_d_n15;
        locals.var_qd_fb_dn16 = assign11320_e12951_d_n16;
        locals.var_qd_fb_dn17 = assign11320_e12951_d_n17;
        locals.var_qd_fb_dn18 = assign11320_e12951_d_n18;

        let (assign11330_e12961, assign11330_e12961_d_n0, assign11330_e12961_d_n2, assign11330_e12961_d_n6, assign11330_e12961_d_n7, assign11330_e12961_d_n10, assign11330_e12961_d_n11, assign11330_e12961_d_n12, assign11330_e12961_d_n13, assign11330_e12961_d_n15, assign11330_e12961_d_n16, assign11330_e12961_d_n17, assign11330_e12961_d_n18,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign11330_e12958: f64 = (1.0 - 0.5);
        let assign11330_e12959: f64 = (locals.var_qb * assign11330_e12958);
        (assign11330_e12959, (locals.var_qb_dn0 * assign11330_e12958), (locals.var_qb_dn2 * assign11330_e12958), (locals.var_qb_dn6 * assign11330_e12958), (locals.var_qb_dn7 * assign11330_e12958), (locals.var_qb_dn10 * assign11330_e12958), (locals.var_qb_dn11 * assign11330_e12958), (locals.var_qb_dn12 * assign11330_e12958), (locals.var_qb_dn13 * assign11330_e12958), (locals.var_qb_dn15 * assign11330_e12958), (locals.var_qb_dn16 * assign11330_e12958), (locals.var_qb_dn17 * assign11330_e12958), (locals.var_qb_dn18 * assign11330_e12958),)
    } else {
        (locals.var_qs_fb, locals.var_qs_fb_dn0, locals.var_qs_fb_dn2, locals.var_qs_fb_dn6, locals.var_qs_fb_dn7, locals.var_qs_fb_dn10, locals.var_qs_fb_dn11, locals.var_qs_fb_dn12, locals.var_qs_fb_dn13, locals.var_qs_fb_dn15, locals.var_qs_fb_dn16, locals.var_qs_fb_dn17, locals.var_qs_fb_dn18,)
    }
};
        locals.var_qs_fb = assign11330_e12961;
        locals.var_qs_fb_dn0 = assign11330_e12961_d_n0;
        locals.var_qs_fb_dn2 = assign11330_e12961_d_n2;
        locals.var_qs_fb_dn6 = assign11330_e12961_d_n6;
        locals.var_qs_fb_dn7 = assign11330_e12961_d_n7;
        locals.var_qs_fb_dn10 = assign11330_e12961_d_n10;
        locals.var_qs_fb_dn11 = assign11330_e12961_d_n11;
        locals.var_qs_fb_dn12 = assign11330_e12961_d_n12;
        locals.var_qs_fb_dn13 = assign11330_e12961_d_n13;
        locals.var_qs_fb_dn15 = assign11330_e12961_d_n15;
        locals.var_qs_fb_dn16 = assign11330_e12961_d_n16;
        locals.var_qs_fb_dn17 = assign11330_e12961_d_n17;
        locals.var_qs_fb_dn18 = assign11330_e12961_d_n18;

        let (assign11340_e12967, assign11340_e12967_d_n0, assign11340_e12967_d_n2, assign11340_e12967_d_n6, assign11340_e12967_d_n7, assign11340_e12967_d_n10, assign11340_e12967_d_n11, assign11340_e12967_d_n12, assign11340_e12967_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign11340_e12967;
        locals.var_qi_dn0 = assign11340_e12967_d_n0;
        locals.var_qi_dn2 = assign11340_e12967_d_n2;
        locals.var_qi_dn6 = assign11340_e12967_d_n6;
        locals.var_qi_dn7 = assign11340_e12967_d_n7;
        locals.var_qi_dn10 = assign11340_e12967_d_n10;
        locals.var_qi_dn11 = assign11340_e12967_d_n11;
        locals.var_qi_dn12 = assign11340_e12967_d_n12;
        locals.var_qi_dn17 = assign11340_e12967_d_n17;

        let (assign11350_e12977, assign11350_e12977_d_n0, assign11350_e12977_d_n2, assign11350_e12977_d_n6, assign11350_e12977_d_n7, assign11350_e12977_d_n10, assign11350_e12977_d_n11, assign11350_e12977_d_n12, assign11350_e12977_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign11350_e12973: f64 = (locals.var_q_s0_bulk * locals.var_leff_cv);
        let assign11350_e12975: f64 = (assign11350_e12973 * locals.var_weffcv_nf);
        (assign11350_e12975, ((locals.var_q_s0_bulk_dn0 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn2 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn6 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn7 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn10 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn11 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn12 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn17 * locals.var_leff_cv) * locals.var_weffcv_nf),)
    } else {
        (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12, locals.var_qsub_dn17,)
    }
};
        locals.var_qsub = assign11350_e12977;
        locals.var_qsub_dn0 = assign11350_e12977_d_n0;
        locals.var_qsub_dn2 = assign11350_e12977_d_n2;
        locals.var_qsub_dn6 = assign11350_e12977_d_n6;
        locals.var_qsub_dn7 = assign11350_e12977_d_n7;
        locals.var_qsub_dn10 = assign11350_e12977_d_n10;
        locals.var_qsub_dn11 = assign11350_e12977_d_n11;
        locals.var_qsub_dn12 = assign11350_e12977_d_n12;
        locals.var_qsub_dn17 = assign11350_e12977_d_n17;

        let (assign11360_e12983, assign11360_e12983_d_n0, assign11360_e12983_d_n2, assign11360_e12983_d_n6, assign11360_e12983_d_n7, assign11360_e12983_d_n10, assign11360_e12983_d_n11, assign11360_e12983_d_n12, assign11360_e12983_d_n13, assign11360_e12983_d_n15, assign11360_e12983_d_n16, assign11360_e12983_d_n17, assign11360_e12983_d_n18,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign11360_e12983;
        locals.var_qd_dn0 = assign11360_e12983_d_n0;
        locals.var_qd_dn2 = assign11360_e12983_d_n2;
        locals.var_qd_dn6 = assign11360_e12983_d_n6;
        locals.var_qd_dn7 = assign11360_e12983_d_n7;
        locals.var_qd_dn10 = assign11360_e12983_d_n10;
        locals.var_qd_dn11 = assign11360_e12983_d_n11;
        locals.var_qd_dn12 = assign11360_e12983_d_n12;
        locals.var_qd_dn13 = assign11360_e12983_d_n13;
        locals.var_qd_dn15 = assign11360_e12983_d_n15;
        locals.var_qd_dn16 = assign11360_e12983_d_n16;
        locals.var_qd_dn17 = assign11360_e12983_d_n17;
        locals.var_qd_dn18 = assign11360_e12983_d_n18;

        let (assign11370_e12989, assign11370_e12989_d_n0, assign11370_e12989_d_n2, assign11370_e12989_d_n6, assign11370_e12989_d_n7, assign11370_e12989_d_n10, assign11370_e12989_d_n11, assign11370_e12989_d_n12, assign11370_e12989_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign11370_e12989;
        locals.var_ids_dn0 = assign11370_e12989_d_n0;
        locals.var_ids_dn2 = assign11370_e12989_d_n2;
        locals.var_ids_dn6 = assign11370_e12989_d_n6;
        locals.var_ids_dn7 = assign11370_e12989_d_n7;
        locals.var_ids_dn10 = assign11370_e12989_d_n10;
        locals.var_ids_dn11 = assign11370_e12989_d_n11;
        locals.var_ids_dn12 = assign11370_e12989_d_n12;
        locals.var_ids_dn17 = assign11370_e12989_d_n17;

        let (assign11380_e12995, assign11380_e12995_d_n0, assign11380_e12995_d_n2, assign11380_e12995_d_n6, assign11380_e12995_d_n7, assign11380_e12995_d_n10, assign11380_e12995_d_n11, assign11380_e12995_d_n12, assign11380_e12995_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign11380_e12995;
        locals.var_vgvt_dn0 = assign11380_e12995_d_n0;
        locals.var_vgvt_dn2 = assign11380_e12995_d_n2;
        locals.var_vgvt_dn6 = assign11380_e12995_d_n6;
        locals.var_vgvt_dn7 = assign11380_e12995_d_n7;
        locals.var_vgvt_dn10 = assign11380_e12995_d_n10;
        locals.var_vgvt_dn11 = assign11380_e12995_d_n11;
        locals.var_vgvt_dn12 = assign11380_e12995_d_n12;
        locals.var_vgvt_dn17 = assign11380_e12995_d_n17;

        let (assign11390_e13001,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign11390_e13001;

        let (assign11400_e13007, assign11400_e13007_d_n0, assign11400_e13007_d_n2, assign11400_e13007_d_n6, assign11400_e13007_d_n7, assign11400_e13007_d_n10, assign11400_e13007_d_n11, assign11400_e13007_d_n12, assign11400_e13007_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign11400_e13007;
        locals.var_phi_sl_soi_dn0 = assign11400_e13007_d_n0;
        locals.var_phi_sl_soi_dn2 = assign11400_e13007_d_n2;
        locals.var_phi_sl_soi_dn6 = assign11400_e13007_d_n6;
        locals.var_phi_sl_soi_dn7 = assign11400_e13007_d_n7;
        locals.var_phi_sl_soi_dn10 = assign11400_e13007_d_n10;
        locals.var_phi_sl_soi_dn11 = assign11400_e13007_d_n11;
        locals.var_phi_sl_soi_dn12 = assign11400_e13007_d_n12;
        locals.var_phi_sl_soi_dn17 = assign11400_e13007_d_n17;

        let (assign11410_e13013, assign11410_e13013_d_n0, assign11410_e13013_d_n2, assign11410_e13013_d_n6, assign11410_e13013_d_n7, assign11410_e13013_d_n10, assign11410_e13013_d_n11, assign11410_e13013_d_n12, assign11410_e13013_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign11410_e13013;
        locals.var_phi_bl_soi_dn0 = assign11410_e13013_d_n0;
        locals.var_phi_bl_soi_dn2 = assign11410_e13013_d_n2;
        locals.var_phi_bl_soi_dn6 = assign11410_e13013_d_n6;
        locals.var_phi_bl_soi_dn7 = assign11410_e13013_d_n7;
        locals.var_phi_bl_soi_dn10 = assign11410_e13013_d_n10;
        locals.var_phi_bl_soi_dn11 = assign11410_e13013_d_n11;
        locals.var_phi_bl_soi_dn12 = assign11410_e13013_d_n12;
        locals.var_phi_bl_soi_dn17 = assign11410_e13013_d_n17;

        let (assign11420_e13019, assign11420_e13019_d_n0, assign11420_e13019_d_n2, assign11420_e13019_d_n6, assign11420_e13019_d_n7, assign11420_e13019_d_n10, assign11420_e13019_d_n11, assign11420_e13019_d_n12, assign11420_e13019_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign11420_e13019;
        locals.var_phi_sl_bulk_dn0 = assign11420_e13019_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign11420_e13019_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign11420_e13019_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign11420_e13019_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign11420_e13019_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign11420_e13019_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign11420_e13019_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign11420_e13019_d_n17;

        let (assign11430_e13025, assign11430_e13025_d_n0, assign11430_e13025_d_n2, assign11430_e13025_d_n6, assign11430_e13025_d_n7, assign11430_e13025_d_n10, assign11430_e13025_d_n11, assign11430_e13025_d_n12, assign11430_e13025_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_q_s0_bulk, locals.var_q_s0_bulk_dn0, locals.var_q_s0_bulk_dn2, locals.var_q_s0_bulk_dn6, locals.var_q_s0_bulk_dn7, locals.var_q_s0_bulk_dn10, locals.var_q_s0_bulk_dn11, locals.var_q_s0_bulk_dn12, locals.var_q_s0_bulk_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
        locals.var_q_sl_bulk = assign11430_e13025;
        locals.var_q_sl_bulk_dn0 = assign11430_e13025_d_n0;
        locals.var_q_sl_bulk_dn2 = assign11430_e13025_d_n2;
        locals.var_q_sl_bulk_dn6 = assign11430_e13025_d_n6;
        locals.var_q_sl_bulk_dn7 = assign11430_e13025_d_n7;
        locals.var_q_sl_bulk_dn10 = assign11430_e13025_d_n10;
        locals.var_q_sl_bulk_dn11 = assign11430_e13025_d_n11;
        locals.var_q_sl_bulk_dn12 = assign11430_e13025_d_n12;
        locals.var_q_sl_bulk_dn17 = assign11430_e13025_d_n17;

        let (assign11440_e13031, assign11440_e13031_d_n0, assign11440_e13031_d_n2, assign11440_e13031_d_n6, assign11440_e13031_d_n7, assign11440_e13031_d_n10, assign11440_e13031_d_n11, assign11440_e13031_d_n12, assign11440_e13031_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign11440_e13031;
        locals.var_psl_dn0 = assign11440_e13031_d_n0;
        locals.var_psl_dn2 = assign11440_e13031_d_n2;
        locals.var_psl_dn6 = assign11440_e13031_d_n6;
        locals.var_psl_dn7 = assign11440_e13031_d_n7;
        locals.var_psl_dn10 = assign11440_e13031_d_n10;
        locals.var_psl_dn11 = assign11440_e13031_d_n11;
        locals.var_psl_dn12 = assign11440_e13031_d_n12;
        locals.var_psl_dn17 = assign11440_e13031_d_n17;

        let (assign11450_e13037, assign11450_e13037_d_n0, assign11450_e13037_d_n2, assign11450_e13037_d_n6, assign11450_e13037_d_n7, assign11450_e13037_d_n10, assign11450_e13037_d_n11, assign11450_e13037_d_n12, assign11450_e13037_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign11450_e13037;
        locals.var_psdl_dn0 = assign11450_e13037_d_n0;
        locals.var_psdl_dn2 = assign11450_e13037_d_n2;
        locals.var_psdl_dn6 = assign11450_e13037_d_n6;
        locals.var_psdl_dn7 = assign11450_e13037_d_n7;
        locals.var_psdl_dn10 = assign11450_e13037_d_n10;
        locals.var_psdl_dn11 = assign11450_e13037_d_n11;
        locals.var_psdl_dn12 = assign11450_e13037_d_n12;
        locals.var_psdl_dn17 = assign11450_e13037_d_n17;

    }

    pub(super) fn stamp_transient_block_33(
        locals: &mut StampLocals,
    ) {
        let (assign11470_e13050, assign11470_e13050_d_n0, assign11470_e13050_d_n2, assign11470_e13050_d_n6, assign11470_e13050_d_n7, assign11470_e13050_d_n10, assign11470_e13050_d_n11, assign11470_e13050_d_n12, assign11470_e13050_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    }
};
        locals.var_vdsorg = assign11470_e13050;
        locals.var_vdsorg_dn0 = assign11470_e13050_d_n0;
        locals.var_vdsorg_dn2 = assign11470_e13050_d_n2;
        locals.var_vdsorg_dn6 = assign11470_e13050_d_n6;
        locals.var_vdsorg_dn7 = assign11470_e13050_d_n7;
        locals.var_vdsorg_dn10 = assign11470_e13050_d_n10;
        locals.var_vdsorg_dn11 = assign11470_e13050_d_n11;
        locals.var_vdsorg_dn12 = assign11470_e13050_d_n12;
        locals.var_vdsorg_dn17 = assign11470_e13050_d_n17;

        let (assign11480_e13057, assign11480_e13057_d_n0, assign11480_e13057_d_n2, assign11480_e13057_d_n6, assign11480_e13057_d_n7, assign11480_e13057_d_n10, assign11480_e13057_d_n11, assign11480_e13057_d_n12, assign11480_e13057_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11480_e13057;
        locals.var_t10_dn0 = assign11480_e13057_d_n0;
        locals.var_t10_dn2 = assign11480_e13057_d_n2;
        locals.var_t10_dn6 = assign11480_e13057_d_n6;
        locals.var_t10_dn7 = assign11480_e13057_d_n7;
        locals.var_t10_dn10 = assign11480_e13057_d_n10;
        locals.var_t10_dn11 = assign11480_e13057_d_n11;
        locals.var_t10_dn12 = assign11480_e13057_d_n12;
        locals.var_t10_dn17 = assign11480_e13057_d_n17;

        let (assign11490_e13068, assign11490_e13068_d_n0, assign11490_e13068_d_n2, assign11490_e13068_d_n6, assign11490_e13068_d_n7, assign11490_e13068_d_n10, assign11490_e13068_d_n11, assign11490_e13068_d_n12, assign11490_e13068_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11490_e13065: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign11490_e13066: f64 = (locals.var_qnsub_esi / assign11490_e13065);
        (assign11490_e13066, (((locals.var_qnsub_esi_dn0 * assign11490_e13065) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign11490_e13065 * assign11490_e13065)), (((locals.var_qnsub_esi_dn2 * assign11490_e13065) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign11490_e13065 * assign11490_e13065)), (((locals.var_qnsub_esi_dn6 * assign11490_e13065) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign11490_e13065 * assign11490_e13065)), (((locals.var_qnsub_esi_dn7 * assign11490_e13065) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign11490_e13065 * assign11490_e13065)), (((locals.var_qnsub_esi_dn10 * assign11490_e13065) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign11490_e13065 * assign11490_e13065)), (((locals.var_qnsub_esi_dn11 * assign11490_e13065) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign11490_e13065 * assign11490_e13065)), (((locals.var_qnsub_esi_dn12 * assign11490_e13065) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign11490_e13065 * assign11490_e13065)), (((locals.var_qnsub_esi_dn17 * assign11490_e13065) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign11490_e13065 * assign11490_e13065)),)
    } else {
        (locals.var_t2__blk303, locals.var_t2__blk303_dn0, locals.var_t2__blk303_dn2, locals.var_t2__blk303_dn6, locals.var_t2__blk303_dn7, locals.var_t2__blk303_dn10, locals.var_t2__blk303_dn11, locals.var_t2__blk303_dn12, locals.var_t2__blk303_dn17,)
    }
};
        locals.var_t2__blk303 = assign11490_e13068;
        locals.var_t2__blk303_dn0 = assign11490_e13068_d_n0;
        locals.var_t2__blk303_dn2 = assign11490_e13068_d_n2;
        locals.var_t2__blk303_dn6 = assign11490_e13068_d_n6;
        locals.var_t2__blk303_dn7 = assign11490_e13068_d_n7;
        locals.var_t2__blk303_dn10 = assign11490_e13068_d_n10;
        locals.var_t2__blk303_dn11 = assign11490_e13068_d_n11;
        locals.var_t2__blk303_dn12 = assign11490_e13068_d_n12;
        locals.var_t2__blk303_dn17 = assign11490_e13068_d_n17;

        let (assign11500_e13083, assign11500_e13083_d_n0, assign11500_e13083_d_n2, assign11500_e13083_d_n6, assign11500_e13083_d_n7, assign11500_e13083_d_n10, assign11500_e13083_d_n11, assign11500_e13083_d_n12, assign11500_e13083_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11500_e13076: f64 = (2.0 / locals.var_t2__blk303);
        let assign11500_e13079: f64 = (locals.var_vgp - locals.var_t10);
        let assign11500_e13080: f64 = (assign11500_e13076 * assign11500_e13079);
        let assign11500_e13081: f64 = (1.0 + assign11500_e13080);
        (assign11500_e13081, (((-((2.0 * locals.var_t2__blk303_dn0) / (locals.var_t2__blk303 * locals.var_t2__blk303))) * assign11500_e13079) + (assign11500_e13076 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), (((-((2.0 * locals.var_t2__blk303_dn2) / (locals.var_t2__blk303 * locals.var_t2__blk303))) * assign11500_e13079) + (assign11500_e13076 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), (((-((2.0 * locals.var_t2__blk303_dn6) / (locals.var_t2__blk303 * locals.var_t2__blk303))) * assign11500_e13079) + (assign11500_e13076 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), (((-((2.0 * locals.var_t2__blk303_dn7) / (locals.var_t2__blk303 * locals.var_t2__blk303))) * assign11500_e13079) + (assign11500_e13076 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), (((-((2.0 * locals.var_t2__blk303_dn10) / (locals.var_t2__blk303 * locals.var_t2__blk303))) * assign11500_e13079) + (assign11500_e13076 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), (((-((2.0 * locals.var_t2__blk303_dn11) / (locals.var_t2__blk303 * locals.var_t2__blk303))) * assign11500_e13079) + (assign11500_e13076 * (locals.var_vgp_dn11 - locals.var_t10_dn11))), (((-((2.0 * locals.var_t2__blk303_dn12) / (locals.var_t2__blk303 * locals.var_t2__blk303))) * assign11500_e13079) + (assign11500_e13076 * (locals.var_vgp_dn12 - locals.var_t10_dn12))), (((-((2.0 * locals.var_t2__blk303_dn17) / (locals.var_t2__blk303 * locals.var_t2__blk303))) * assign11500_e13079) + (assign11500_e13076 * (locals.var_vgp_dn17 - locals.var_t10_dn17))),)
    } else {
        (locals.var_t4__blk305, locals.var_t4__blk305_dn0, locals.var_t4__blk305_dn2, locals.var_t4__blk305_dn6, locals.var_t4__blk305_dn7, locals.var_t4__blk305_dn10, locals.var_t4__blk305_dn11, locals.var_t4__blk305_dn12, locals.var_t4__blk305_dn17,)
    }
};
        locals.var_t4__blk305 = assign11500_e13083;
        locals.var_t4__blk305_dn0 = assign11500_e13083_d_n0;
        locals.var_t4__blk305_dn2 = assign11500_e13083_d_n2;
        locals.var_t4__blk305_dn6 = assign11500_e13083_d_n6;
        locals.var_t4__blk305_dn7 = assign11500_e13083_d_n7;
        locals.var_t4__blk305_dn10 = assign11500_e13083_d_n10;
        locals.var_t4__blk305_dn11 = assign11500_e13083_d_n11;
        locals.var_t4__blk305_dn12 = assign11500_e13083_d_n12;
        locals.var_t4__blk305_dn17 = assign11500_e13083_d_n17;

        let (assign11510_e13094, assign11510_e13094_d_n0, assign11510_e13094_d_n2, assign11510_e13094_d_n6, assign11510_e13094_d_n7, assign11510_e13094_d_n10, assign11510_e13094_d_n11, assign11510_e13094_d_n12, assign11510_e13094_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11510_e13091: f64 = (2.0 / locals.var_t2__blk303);
        let assign11510_e13092: f64 = (1.0 + assign11510_e13091);
        (assign11510_e13092, (-((2.0 * locals.var_t2__blk303_dn0) / (locals.var_t2__blk303 * locals.var_t2__blk303))), (-((2.0 * locals.var_t2__blk303_dn2) / (locals.var_t2__blk303 * locals.var_t2__blk303))), (-((2.0 * locals.var_t2__blk303_dn6) / (locals.var_t2__blk303 * locals.var_t2__blk303))), (-((2.0 * locals.var_t2__blk303_dn7) / (locals.var_t2__blk303 * locals.var_t2__blk303))), (-((2.0 * locals.var_t2__blk303_dn10) / (locals.var_t2__blk303 * locals.var_t2__blk303))), (-((2.0 * locals.var_t2__blk303_dn11) / (locals.var_t2__blk303 * locals.var_t2__blk303))), (-((2.0 * locals.var_t2__blk303_dn12) / (locals.var_t2__blk303 * locals.var_t2__blk303))), (-((2.0 * locals.var_t2__blk303_dn17) / (locals.var_t2__blk303 * locals.var_t2__blk303))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign11510_e13094;
        locals.var_t5_dn0 = assign11510_e13094_d_n0;
        locals.var_t5_dn2 = assign11510_e13094_d_n2;
        locals.var_t5_dn6 = assign11510_e13094_d_n6;
        locals.var_t5_dn7 = assign11510_e13094_d_n7;
        locals.var_t5_dn10 = assign11510_e13094_d_n10;
        locals.var_t5_dn11 = assign11510_e13094_d_n11;
        locals.var_t5_dn12 = assign11510_e13094_d_n12;
        locals.var_t5_dn17 = assign11510_e13094_d_n17;

        let assign11520_e13098: f64 = locals.var_t5;
        let assign11520_e13103: f64 = if ((locals.var_t4__blk305 < assign11520_e13098) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard309 = assign11520_e13103;

        let (assign11530_e13116, assign11530_e13116_d_n0, assign11530_e13116_d_n2, assign11530_e13116_d_n6, assign11530_e13116_d_n7, assign11530_e13116_d_n10, assign11530_e13116_d_n11, assign11530_e13116_d_n12, assign11530_e13116_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11530_e13112: f64 = locals.var_t5;
        let assign11530_e13114: f64 = (assign11530_e13112 - locals.var_t4__blk305);
        (assign11530_e13114, (locals.var_t5_dn0 - locals.var_t4__blk305_dn0), (locals.var_t5_dn2 - locals.var_t4__blk305_dn2), (locals.var_t5_dn6 - locals.var_t4__blk305_dn6), (locals.var_t5_dn7 - locals.var_t4__blk305_dn7), (locals.var_t5_dn10 - locals.var_t4__blk305_dn10), (locals.var_t5_dn11 - locals.var_t4__blk305_dn11), (locals.var_t5_dn12 - locals.var_t4__blk305_dn12), (locals.var_t5_dn17 - locals.var_t4__blk305_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign11530_e13116;
        locals.var_tmf1_dn0 = assign11530_e13116_d_n0;
        locals.var_tmf1_dn2 = assign11530_e13116_d_n2;
        locals.var_tmf1_dn6 = assign11530_e13116_d_n6;
        locals.var_tmf1_dn7 = assign11530_e13116_d_n7;
        locals.var_tmf1_dn10 = assign11530_e13116_d_n10;
        locals.var_tmf1_dn11 = assign11530_e13116_d_n11;
        locals.var_tmf1_dn12 = assign11530_e13116_d_n12;
        locals.var_tmf1_dn17 = assign11530_e13116_d_n17;

        let (assign11540_e13127, assign11540_e13127_d_n0, assign11540_e13127_d_n2, assign11540_e13127_d_n6, assign11540_e13127_d_n7, assign11540_e13127_d_n10, assign11540_e13127_d_n11, assign11540_e13127_d_n12, assign11540_e13127_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11540_e13125: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign11540_e13125, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign11540_e13127;
        locals.var_x2_dn0 = assign11540_e13127_d_n0;
        locals.var_x2_dn2 = assign11540_e13127_d_n2;
        locals.var_x2_dn6 = assign11540_e13127_d_n6;
        locals.var_x2_dn7 = assign11540_e13127_d_n7;
        locals.var_x2_dn10 = assign11540_e13127_d_n10;
        locals.var_x2_dn11 = assign11540_e13127_d_n11;
        locals.var_x2_dn12 = assign11540_e13127_d_n12;
        locals.var_x2_dn17 = assign11540_e13127_d_n17;

        let (assign11550_e13138, assign11550_e13138_d_n0, assign11550_e13138_d_n2, assign11550_e13138_d_n6, assign11550_e13138_d_n7, assign11550_e13138_d_n10, assign11550_e13138_d_n11, assign11550_e13138_d_n12, assign11550_e13138_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11550_e13136: f64 = (locals.var_t5 * locals.var_t5);
        (assign11550_e13136, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn12 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn12)), ((locals.var_t5_dn17 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign11550_e13138;
        locals.var_xmax2_dn0 = assign11550_e13138_d_n0;
        locals.var_xmax2_dn2 = assign11550_e13138_d_n2;
        locals.var_xmax2_dn6 = assign11550_e13138_d_n6;
        locals.var_xmax2_dn7 = assign11550_e13138_d_n7;
        locals.var_xmax2_dn10 = assign11550_e13138_d_n10;
        locals.var_xmax2_dn11 = assign11550_e13138_d_n11;
        locals.var_xmax2_dn12 = assign11550_e13138_d_n12;
        locals.var_xmax2_dn17 = assign11550_e13138_d_n17;

        let (assign11560_e13147, assign11560_e13147_d_n0, assign11560_e13147_d_n2, assign11560_e13147_d_n6, assign11560_e13147_d_n7, assign11560_e13147_d_n10, assign11560_e13147_d_n11, assign11560_e13147_d_n12, assign11560_e13147_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11560_e13147;
        locals.var_xp_dn0 = assign11560_e13147_d_n0;
        locals.var_xp_dn2 = assign11560_e13147_d_n2;
        locals.var_xp_dn6 = assign11560_e13147_d_n6;
        locals.var_xp_dn7 = assign11560_e13147_d_n7;
        locals.var_xp_dn10 = assign11560_e13147_d_n10;
        locals.var_xp_dn11 = assign11560_e13147_d_n11;
        locals.var_xp_dn12 = assign11560_e13147_d_n12;
        locals.var_xp_dn17 = assign11560_e13147_d_n17;

        let (assign11570_e13156, assign11570_e13156_d_n0, assign11570_e13156_d_n2, assign11570_e13156_d_n6, assign11570_e13156_d_n7, assign11570_e13156_d_n10, assign11570_e13156_d_n11, assign11570_e13156_d_n12, assign11570_e13156_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11570_e13156;
        locals.var_xmp_dn0 = assign11570_e13156_d_n0;
        locals.var_xmp_dn2 = assign11570_e13156_d_n2;
        locals.var_xmp_dn6 = assign11570_e13156_d_n6;
        locals.var_xmp_dn7 = assign11570_e13156_d_n7;
        locals.var_xmp_dn10 = assign11570_e13156_d_n10;
        locals.var_xmp_dn11 = assign11570_e13156_d_n11;
        locals.var_xmp_dn12 = assign11570_e13156_d_n12;
        locals.var_xmp_dn17 = assign11570_e13156_d_n17;

        let (assign11580_e13165,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign11580_e13165;

        let (assign11590_e13174,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11590_e13174;

        let (assign11600_e13183, assign11600_e13183_d_n0, assign11600_e13183_d_n2, assign11600_e13183_d_n6, assign11600_e13183_d_n7, assign11600_e13183_d_n10, assign11600_e13183_d_n11, assign11600_e13183_d_n12, assign11600_e13183_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign11600_e13183;
        locals.var_arg_dn0 = assign11600_e13183_d_n0;
        locals.var_arg_dn2 = assign11600_e13183_d_n2;
        locals.var_arg_dn6 = assign11600_e13183_d_n6;
        locals.var_arg_dn7 = assign11600_e13183_d_n7;
        locals.var_arg_dn10 = assign11600_e13183_d_n10;
        locals.var_arg_dn11 = assign11600_e13183_d_n11;
        locals.var_arg_dn12 = assign11600_e13183_d_n12;
        locals.var_arg_dn17 = assign11600_e13183_d_n17;

        let (assign11610_e13192, assign11610_e13192_d_n0, assign11610_e13192_d_n2, assign11610_e13192_d_n6, assign11610_e13192_d_n7, assign11610_e13192_d_n10, assign11610_e13192_d_n11, assign11610_e13192_d_n12, assign11610_e13192_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11610_e13192;
        locals.var_dnm_dn0 = assign11610_e13192_d_n0;
        locals.var_dnm_dn2 = assign11610_e13192_d_n2;
        locals.var_dnm_dn6 = assign11610_e13192_d_n6;
        locals.var_dnm_dn7 = assign11610_e13192_d_n7;
        locals.var_dnm_dn10 = assign11610_e13192_d_n10;
        locals.var_dnm_dn11 = assign11610_e13192_d_n11;
        locals.var_dnm_dn12 = assign11610_e13192_d_n12;
        locals.var_dnm_dn17 = assign11610_e13192_d_n17;

        let (assign11620_e13203, assign11620_e13203_d_n0, assign11620_e13203_d_n2, assign11620_e13203_d_n6, assign11620_e13203_d_n7, assign11620_e13203_d_n10, assign11620_e13203_d_n11, assign11620_e13203_d_n12, assign11620_e13203_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11620_e13201: f64 = (locals.var_xp * locals.var_x2);
        (assign11620_e13201, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11620_e13203;
        locals.var_xp_dn0 = assign11620_e13203_d_n0;
        locals.var_xp_dn2 = assign11620_e13203_d_n2;
        locals.var_xp_dn6 = assign11620_e13203_d_n6;
        locals.var_xp_dn7 = assign11620_e13203_d_n7;
        locals.var_xp_dn10 = assign11620_e13203_d_n10;
        locals.var_xp_dn11 = assign11620_e13203_d_n11;
        locals.var_xp_dn12 = assign11620_e13203_d_n12;
        locals.var_xp_dn17 = assign11620_e13203_d_n17;

        let (assign11630_e13214, assign11630_e13214_d_n0, assign11630_e13214_d_n2, assign11630_e13214_d_n6, assign11630_e13214_d_n7, assign11630_e13214_d_n10, assign11630_e13214_d_n11, assign11630_e13214_d_n12, assign11630_e13214_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11630_e13212: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11630_e13212, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11630_e13214;
        locals.var_xmp_dn0 = assign11630_e13214_d_n0;
        locals.var_xmp_dn2 = assign11630_e13214_d_n2;
        locals.var_xmp_dn6 = assign11630_e13214_d_n6;
        locals.var_xmp_dn7 = assign11630_e13214_d_n7;
        locals.var_xmp_dn10 = assign11630_e13214_d_n10;
        locals.var_xmp_dn11 = assign11630_e13214_d_n11;
        locals.var_xmp_dn12 = assign11630_e13214_d_n12;
        locals.var_xmp_dn17 = assign11630_e13214_d_n17;

        let (assign11640_e13225, assign11640_e13225_d_n0, assign11640_e13225_d_n2, assign11640_e13225_d_n6, assign11640_e13225_d_n7, assign11640_e13225_d_n10, assign11640_e13225_d_n11, assign11640_e13225_d_n12, assign11640_e13225_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11640_e13223: f64 = (locals.var_xp * locals.var_x2);
        (assign11640_e13223, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11640_e13225;
        locals.var_xp_dn0 = assign11640_e13225_d_n0;
        locals.var_xp_dn2 = assign11640_e13225_d_n2;
        locals.var_xp_dn6 = assign11640_e13225_d_n6;
        locals.var_xp_dn7 = assign11640_e13225_d_n7;
        locals.var_xp_dn10 = assign11640_e13225_d_n10;
        locals.var_xp_dn11 = assign11640_e13225_d_n11;
        locals.var_xp_dn12 = assign11640_e13225_d_n12;
        locals.var_xp_dn17 = assign11640_e13225_d_n17;

        let (assign11650_e13236, assign11650_e13236_d_n0, assign11650_e13236_d_n2, assign11650_e13236_d_n6, assign11650_e13236_d_n7, assign11650_e13236_d_n10, assign11650_e13236_d_n11, assign11650_e13236_d_n12, assign11650_e13236_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11650_e13234: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11650_e13234, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11650_e13236;
        locals.var_xmp_dn0 = assign11650_e13236_d_n0;
        locals.var_xmp_dn2 = assign11650_e13236_d_n2;
        locals.var_xmp_dn6 = assign11650_e13236_d_n6;
        locals.var_xmp_dn7 = assign11650_e13236_d_n7;
        locals.var_xmp_dn10 = assign11650_e13236_d_n10;
        locals.var_xmp_dn11 = assign11650_e13236_d_n11;
        locals.var_xmp_dn12 = assign11650_e13236_d_n12;
        locals.var_xmp_dn17 = assign11650_e13236_d_n17;

        let (assign11660_e13247, assign11660_e13247_d_n0, assign11660_e13247_d_n2, assign11660_e13247_d_n6, assign11660_e13247_d_n7, assign11660_e13247_d_n10, assign11660_e13247_d_n11, assign11660_e13247_d_n12, assign11660_e13247_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11660_e13245: f64 = (locals.var_xp * locals.var_x2);
        (assign11660_e13245, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11660_e13247;
        locals.var_xp_dn0 = assign11660_e13247_d_n0;
        locals.var_xp_dn2 = assign11660_e13247_d_n2;
        locals.var_xp_dn6 = assign11660_e13247_d_n6;
        locals.var_xp_dn7 = assign11660_e13247_d_n7;
        locals.var_xp_dn10 = assign11660_e13247_d_n10;
        locals.var_xp_dn11 = assign11660_e13247_d_n11;
        locals.var_xp_dn12 = assign11660_e13247_d_n12;
        locals.var_xp_dn17 = assign11660_e13247_d_n17;

        let (assign11670_e13258, assign11670_e13258_d_n0, assign11670_e13258_d_n2, assign11670_e13258_d_n6, assign11670_e13258_d_n7, assign11670_e13258_d_n10, assign11670_e13258_d_n11, assign11670_e13258_d_n12, assign11670_e13258_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11670_e13256: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11670_e13256, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11670_e13258;
        locals.var_xmp_dn0 = assign11670_e13258_d_n0;
        locals.var_xmp_dn2 = assign11670_e13258_d_n2;
        locals.var_xmp_dn6 = assign11670_e13258_d_n6;
        locals.var_xmp_dn7 = assign11670_e13258_d_n7;
        locals.var_xmp_dn10 = assign11670_e13258_d_n10;
        locals.var_xmp_dn11 = assign11670_e13258_d_n11;
        locals.var_xmp_dn12 = assign11670_e13258_d_n12;
        locals.var_xmp_dn17 = assign11670_e13258_d_n17;

        let (assign11680_e13269, assign11680_e13269_d_n0, assign11680_e13269_d_n2, assign11680_e13269_d_n6, assign11680_e13269_d_n7, assign11680_e13269_d_n10, assign11680_e13269_d_n11, assign11680_e13269_d_n12, assign11680_e13269_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11680_e13267: f64 = (locals.var_xp * locals.var_x2);
        (assign11680_e13267, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11680_e13269;
        locals.var_xp_dn0 = assign11680_e13269_d_n0;
        locals.var_xp_dn2 = assign11680_e13269_d_n2;
        locals.var_xp_dn6 = assign11680_e13269_d_n6;
        locals.var_xp_dn7 = assign11680_e13269_d_n7;
        locals.var_xp_dn10 = assign11680_e13269_d_n10;
        locals.var_xp_dn11 = assign11680_e13269_d_n11;
        locals.var_xp_dn12 = assign11680_e13269_d_n12;
        locals.var_xp_dn17 = assign11680_e13269_d_n17;

        let (assign11690_e13280, assign11690_e13280_d_n0, assign11690_e13280_d_n2, assign11690_e13280_d_n6, assign11690_e13280_d_n7, assign11690_e13280_d_n10, assign11690_e13280_d_n11, assign11690_e13280_d_n12, assign11690_e13280_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11690_e13278: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11690_e13278, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11690_e13280;
        locals.var_xmp_dn0 = assign11690_e13280_d_n0;
        locals.var_xmp_dn2 = assign11690_e13280_d_n2;
        locals.var_xmp_dn6 = assign11690_e13280_d_n6;
        locals.var_xmp_dn7 = assign11690_e13280_d_n7;
        locals.var_xmp_dn10 = assign11690_e13280_d_n10;
        locals.var_xmp_dn11 = assign11690_e13280_d_n11;
        locals.var_xmp_dn12 = assign11690_e13280_d_n12;
        locals.var_xmp_dn17 = assign11690_e13280_d_n17;

        let (assign11700_e13291, assign11700_e13291_d_n0, assign11700_e13291_d_n2, assign11700_e13291_d_n6, assign11700_e13291_d_n7, assign11700_e13291_d_n10, assign11700_e13291_d_n11, assign11700_e13291_d_n12, assign11700_e13291_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11700_e13289: f64 = (locals.var_xp + locals.var_xmp);
        (assign11700_e13289, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign11700_e13291;
        locals.var_arg_dn0 = assign11700_e13291_d_n0;
        locals.var_arg_dn2 = assign11700_e13291_d_n2;
        locals.var_arg_dn6 = assign11700_e13291_d_n6;
        locals.var_arg_dn7 = assign11700_e13291_d_n7;
        locals.var_arg_dn10 = assign11700_e13291_d_n10;
        locals.var_arg_dn11 = assign11700_e13291_d_n11;
        locals.var_arg_dn12 = assign11700_e13291_d_n12;
        locals.var_arg_dn17 = assign11700_e13291_d_n17;

        let (assign11710_e13300, assign11710_e13300_d_n0, assign11710_e13300_d_n2, assign11710_e13300_d_n6, assign11710_e13300_d_n7, assign11710_e13300_d_n10, assign11710_e13300_d_n11, assign11710_e13300_d_n12, assign11710_e13300_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11710_e13300;
        locals.var_dnm_dn0 = assign11710_e13300_d_n0;
        locals.var_dnm_dn2 = assign11710_e13300_d_n2;
        locals.var_dnm_dn6 = assign11710_e13300_d_n6;
        locals.var_dnm_dn7 = assign11710_e13300_d_n7;
        locals.var_dnm_dn10 = assign11710_e13300_d_n10;
        locals.var_dnm_dn11 = assign11710_e13300_d_n11;
        locals.var_dnm_dn12 = assign11710_e13300_d_n12;
        locals.var_dnm_dn17 = assign11710_e13300_d_n17;

        let assign11720_e13315: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard310 = assign11720_e13315;

        let assign11730_e13318: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign11730_e13318;

        let (assign11740_e13331,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11740_e13331;

        let assign11750_e13334: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign11750_e13334;

        let (assign11760_e13350,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard312 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11760_e13350;

        let assign11770_e13353: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign11770_e13353;

        let (assign11780_e13372,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard313 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11780_e13372;

        let assign11790_e13375: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign11790_e13375;

        let (assign11800_e13397,) = {
    if ((((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard314 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11800_e13397;

        let (assign11810_e13408,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign11810_e13408;

        let mut assign11820_loop_guard: usize = 0;
        while {
            let assign11820_cond_e13420: f64 = if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign11820_cond_e13420 != 0.0
        } {
            assign11820_loop_guard += 1;
            assert!(assign11820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign11820_body0_e13432, assign11820_body0_e13432_d_n0, assign11820_body0_e13432_d_n2, assign11820_body0_e13432_d_n6, assign11820_body0_e13432_d_n7, assign11820_body0_e13432_d_n10, assign11820_body0_e13432_d_n11, assign11820_body0_e13432_d_n12, assign11820_body0_e13432_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign11820_body0_e13430: f64 = (locals.var_dnm).sqrt();
        (assign11820_body0_e13430, (locals.var_dnm_dn0 / (2.0 * assign11820_body0_e13430)), (locals.var_dnm_dn2 / (2.0 * assign11820_body0_e13430)), (locals.var_dnm_dn6 / (2.0 * assign11820_body0_e13430)), (locals.var_dnm_dn7 / (2.0 * assign11820_body0_e13430)), (locals.var_dnm_dn10 / (2.0 * assign11820_body0_e13430)), (locals.var_dnm_dn11 / (2.0 * assign11820_body0_e13430)), (locals.var_dnm_dn12 / (2.0 * assign11820_body0_e13430)), (locals.var_dnm_dn17 / (2.0 * assign11820_body0_e13430)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign11820_body0_e13432;
            locals.var_dnm_dn0 = assign11820_body0_e13432_d_n0;
            locals.var_dnm_dn2 = assign11820_body0_e13432_d_n2;
            locals.var_dnm_dn6 = assign11820_body0_e13432_d_n6;
            locals.var_dnm_dn7 = assign11820_body0_e13432_d_n7;
            locals.var_dnm_dn10 = assign11820_body0_e13432_d_n10;
            locals.var_dnm_dn11 = assign11820_body0_e13432_d_n11;
            locals.var_dnm_dn12 = assign11820_body0_e13432_d_n12;
            locals.var_dnm_dn17 = assign11820_body0_e13432_d_n17;
            let (assign11820_body1_e13445,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign11820_body1_e13443: f64 = (locals.var_m0 + 1.0);
        (assign11820_body1_e13443,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign11820_body1_e13445;
        }

    }

    pub(super) fn stamp_transient_block_34(
        locals: &mut StampLocals,
    ) {
        let (assign11830_e13463, assign11830_e13463_d_n0, assign11830_e13463_d_n2, assign11830_e13463_d_n6, assign11830_e13463_d_n7, assign11830_e13463_d_n10, assign11830_e13463_d_n11, assign11830_e13463_d_n12, assign11830_e13463_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 == 0.0)) {
        let assign11830_e13459: f64 = (2.0 * 4.0);
        let assign11830_e13460: f64 = (1.0 / assign11830_e13459);
        let assign11830_e13461: f64 = (locals.var_dnm).powf(assign11830_e13460);
        (assign11830_e13461, if 0.0 == 0.0 && ((assign11830_e13460) as f64).is_finite() && ((assign11830_e13460) as f64).fract() == 0.0 { if assign11830_e13460 == 0.0 { 0.0 } else { (assign11830_e13460 * ((locals.var_dnm).powf(assign11830_e13460 - 1.0) * locals.var_dnm_dn0)) } } else { (assign11830_e13461 * (assign11830_e13460 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11830_e13460) as f64).is_finite() && ((assign11830_e13460) as f64).fract() == 0.0 { if assign11830_e13460 == 0.0 { 0.0 } else { (assign11830_e13460 * ((locals.var_dnm).powf(assign11830_e13460 - 1.0) * locals.var_dnm_dn2)) } } else { (assign11830_e13461 * (assign11830_e13460 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11830_e13460) as f64).is_finite() && ((assign11830_e13460) as f64).fract() == 0.0 { if assign11830_e13460 == 0.0 { 0.0 } else { (assign11830_e13460 * ((locals.var_dnm).powf(assign11830_e13460 - 1.0) * locals.var_dnm_dn6)) } } else { (assign11830_e13461 * (assign11830_e13460 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11830_e13460) as f64).is_finite() && ((assign11830_e13460) as f64).fract() == 0.0 { if assign11830_e13460 == 0.0 { 0.0 } else { (assign11830_e13460 * ((locals.var_dnm).powf(assign11830_e13460 - 1.0) * locals.var_dnm_dn7)) } } else { (assign11830_e13461 * (assign11830_e13460 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11830_e13460) as f64).is_finite() && ((assign11830_e13460) as f64).fract() == 0.0 { if assign11830_e13460 == 0.0 { 0.0 } else { (assign11830_e13460 * ((locals.var_dnm).powf(assign11830_e13460 - 1.0) * locals.var_dnm_dn10)) } } else { (assign11830_e13461 * (assign11830_e13460 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11830_e13460) as f64).is_finite() && ((assign11830_e13460) as f64).fract() == 0.0 { if assign11830_e13460 == 0.0 { 0.0 } else { (assign11830_e13460 * ((locals.var_dnm).powf(assign11830_e13460 - 1.0) * locals.var_dnm_dn11)) } } else { (assign11830_e13461 * (assign11830_e13460 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11830_e13460) as f64).is_finite() && ((assign11830_e13460) as f64).fract() == 0.0 { if assign11830_e13460 == 0.0 { 0.0 } else { (assign11830_e13460 * ((locals.var_dnm).powf(assign11830_e13460 - 1.0) * locals.var_dnm_dn12)) } } else { (assign11830_e13461 * (assign11830_e13460 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11830_e13460) as f64).is_finite() && ((assign11830_e13460) as f64).fract() == 0.0 { if assign11830_e13460 == 0.0 { 0.0 } else { (assign11830_e13460 * ((locals.var_dnm).powf(assign11830_e13460 - 1.0) * locals.var_dnm_dn17)) } } else { (assign11830_e13461 * (assign11830_e13460 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11830_e13463;
        locals.var_dnm_dn0 = assign11830_e13463_d_n0;
        locals.var_dnm_dn2 = assign11830_e13463_d_n2;
        locals.var_dnm_dn6 = assign11830_e13463_d_n6;
        locals.var_dnm_dn7 = assign11830_e13463_d_n7;
        locals.var_dnm_dn10 = assign11830_e13463_d_n10;
        locals.var_dnm_dn11 = assign11830_e13463_d_n11;
        locals.var_dnm_dn12 = assign11830_e13463_d_n12;
        locals.var_dnm_dn17 = assign11830_e13463_d_n17;

        let (assign11840_e13474, assign11840_e13474_d_n0, assign11840_e13474_d_n2, assign11840_e13474_d_n6, assign11840_e13474_d_n7, assign11840_e13474_d_n10, assign11840_e13474_d_n11, assign11840_e13474_d_n12, assign11840_e13474_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11840_e13472: f64 = (1.0 / locals.var_dnm);
        (assign11840_e13472, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11840_e13474;
        locals.var_dnm_dn0 = assign11840_e13474_d_n0;
        locals.var_dnm_dn2 = assign11840_e13474_d_n2;
        locals.var_dnm_dn6 = assign11840_e13474_d_n6;
        locals.var_dnm_dn7 = assign11840_e13474_d_n7;
        locals.var_dnm_dn10 = assign11840_e13474_d_n10;
        locals.var_dnm_dn11 = assign11840_e13474_d_n11;
        locals.var_dnm_dn12 = assign11840_e13474_d_n12;
        locals.var_dnm_dn17 = assign11840_e13474_d_n17;

        let (assign11850_e13487, assign11850_e13487_d_n0, assign11850_e13487_d_n2, assign11850_e13487_d_n6, assign11850_e13487_d_n7, assign11850_e13487_d_n10, assign11850_e13487_d_n11, assign11850_e13487_d_n12, assign11850_e13487_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11850_e13483: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign11850_e13485: f64 = (assign11850_e13483 * locals.var_dnm);
        (assign11850_e13485, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign11850_e13483 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign11850_e13483 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign11850_e13483 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign11850_e13483 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign11850_e13483 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign11850_e13483 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn12)) * locals.var_dnm) + (assign11850_e13483 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn17)) * locals.var_dnm) + (assign11850_e13483 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign11850_e13487;
        locals.var_tmf0_dn0 = assign11850_e13487_d_n0;
        locals.var_tmf0_dn2 = assign11850_e13487_d_n2;
        locals.var_tmf0_dn6 = assign11850_e13487_d_n6;
        locals.var_tmf0_dn7 = assign11850_e13487_d_n7;
        locals.var_tmf0_dn10 = assign11850_e13487_d_n10;
        locals.var_tmf0_dn11 = assign11850_e13487_d_n11;
        locals.var_tmf0_dn12 = assign11850_e13487_d_n12;
        locals.var_tmf0_dn17 = assign11850_e13487_d_n17;

        let (assign11860_e13500, assign11860_e13500_d_n0, assign11860_e13500_d_n2, assign11860_e13500_d_n6, assign11860_e13500_d_n7, assign11860_e13500_d_n10, assign11860_e13500_d_n11, assign11860_e13500_d_n12, assign11860_e13500_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign11860_e13496: f64 = locals.var_t5;
        let assign11860_e13498: f64 = (assign11860_e13496 - locals.var_tmf0);
        (assign11860_e13498, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn12 - locals.var_tmf0_dn12), (locals.var_t5_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t4__blk305, locals.var_t4__blk305_dn0, locals.var_t4__blk305_dn2, locals.var_t4__blk305_dn6, locals.var_t4__blk305_dn7, locals.var_t4__blk305_dn10, locals.var_t4__blk305_dn11, locals.var_t4__blk305_dn12, locals.var_t4__blk305_dn17,)
    }
};
        locals.var_t4__blk305 = assign11860_e13500;
        locals.var_t4__blk305_dn0 = assign11860_e13500_d_n0;
        locals.var_t4__blk305_dn2 = assign11860_e13500_d_n2;
        locals.var_t4__blk305_dn6 = assign11860_e13500_d_n6;
        locals.var_t4__blk305_dn7 = assign11860_e13500_d_n7;
        locals.var_t4__blk305_dn10 = assign11860_e13500_d_n10;
        locals.var_t4__blk305_dn11 = assign11860_e13500_d_n11;
        locals.var_t4__blk305_dn12 = assign11860_e13500_d_n12;
        locals.var_t4__blk305_dn17 = assign11860_e13500_d_n17;

        let (assign11870_e13510, assign11870_e13510_d_n0, assign11870_e13510_d_n2, assign11870_e13510_d_n6, assign11870_e13510_d_n7, assign11870_e13510_d_n10, assign11870_e13510_d_n11, assign11870_e13510_d_n12, assign11870_e13510_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard309 == 0.0)) {
        (locals.var_t4__blk305, locals.var_t4__blk305_dn0, locals.var_t4__blk305_dn2, locals.var_t4__blk305_dn6, locals.var_t4__blk305_dn7, locals.var_t4__blk305_dn10, locals.var_t4__blk305_dn11, locals.var_t4__blk305_dn12, locals.var_t4__blk305_dn17,)
    } else {
        (locals.var_t4__blk305, locals.var_t4__blk305_dn0, locals.var_t4__blk305_dn2, locals.var_t4__blk305_dn6, locals.var_t4__blk305_dn7, locals.var_t4__blk305_dn10, locals.var_t4__blk305_dn11, locals.var_t4__blk305_dn12, locals.var_t4__blk305_dn17,)
    }
};
        locals.var_t4__blk305 = assign11870_e13510;
        locals.var_t4__blk305_dn0 = assign11870_e13510_d_n0;
        locals.var_t4__blk305_dn2 = assign11870_e13510_d_n2;
        locals.var_t4__blk305_dn6 = assign11870_e13510_d_n6;
        locals.var_t4__blk305_dn7 = assign11870_e13510_d_n7;
        locals.var_t4__blk305_dn10 = assign11870_e13510_d_n10;
        locals.var_t4__blk305_dn11 = assign11870_e13510_d_n11;
        locals.var_t4__blk305_dn12 = assign11870_e13510_d_n12;
        locals.var_t4__blk305_dn17 = assign11870_e13510_d_n17;

        let (assign11880_e13518, assign11880_e13518_d_n0, assign11880_e13518_d_n2, assign11880_e13518_d_n6, assign11880_e13518_d_n7, assign11880_e13518_d_n10, assign11880_e13518_d_n11, assign11880_e13518_d_n12, assign11880_e13518_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11880_e13516: f64 = (locals.var_t4__blk305).sqrt();
        (assign11880_e13516, (locals.var_t4__blk305_dn0 / (2.0 * assign11880_e13516)), (locals.var_t4__blk305_dn2 / (2.0 * assign11880_e13516)), (locals.var_t4__blk305_dn6 / (2.0 * assign11880_e13516)), (locals.var_t4__blk305_dn7 / (2.0 * assign11880_e13516)), (locals.var_t4__blk305_dn10 / (2.0 * assign11880_e13516)), (locals.var_t4__blk305_dn11 / (2.0 * assign11880_e13516)), (locals.var_t4__blk305_dn12 / (2.0 * assign11880_e13516)), (locals.var_t4__blk305_dn17 / (2.0 * assign11880_e13516)),)
    } else {
        (locals.var_t3__blk304, locals.var_t3__blk304_dn0, locals.var_t3__blk304_dn2, locals.var_t3__blk304_dn6, locals.var_t3__blk304_dn7, locals.var_t3__blk304_dn10, locals.var_t3__blk304_dn11, locals.var_t3__blk304_dn12, locals.var_t3__blk304_dn17,)
    }
};
        locals.var_t3__blk304 = assign11880_e13518;
        locals.var_t3__blk304_dn0 = assign11880_e13518_d_n0;
        locals.var_t3__blk304_dn2 = assign11880_e13518_d_n2;
        locals.var_t3__blk304_dn6 = assign11880_e13518_d_n6;
        locals.var_t3__blk304_dn7 = assign11880_e13518_d_n7;
        locals.var_t3__blk304_dn10 = assign11880_e13518_d_n10;
        locals.var_t3__blk304_dn11 = assign11880_e13518_d_n11;
        locals.var_t3__blk304_dn12 = assign11880_e13518_d_n12;
        locals.var_t3__blk304_dn17 = assign11880_e13518_d_n17;

        let (assign11890_e13531, assign11890_e13531_d_n0, assign11890_e13531_d_n2, assign11890_e13531_d_n6, assign11890_e13531_d_n7, assign11890_e13531_d_n10, assign11890_e13531_d_n11, assign11890_e13531_d_n12, assign11890_e13531_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11890_e13527: f64 = (1.0 - locals.var_t3__blk304);
        let assign11890_e13528: f64 = (locals.var_t2__blk303 * assign11890_e13527);
        let assign11890_e13529: f64 = (locals.var_vgp + assign11890_e13528);
        (assign11890_e13529, (locals.var_vgp_dn0 + ((locals.var_t2__blk303_dn0 * assign11890_e13527) + (locals.var_t2__blk303 * (-locals.var_t3__blk304_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2__blk303_dn2 * assign11890_e13527) + (locals.var_t2__blk303 * (-locals.var_t3__blk304_dn2)))), (locals.var_vgp_dn6 + ((locals.var_t2__blk303_dn6 * assign11890_e13527) + (locals.var_t2__blk303 * (-locals.var_t3__blk304_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2__blk303_dn7 * assign11890_e13527) + (locals.var_t2__blk303 * (-locals.var_t3__blk304_dn7)))), (locals.var_vgp_dn10 + ((locals.var_t2__blk303_dn10 * assign11890_e13527) + (locals.var_t2__blk303 * (-locals.var_t3__blk304_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2__blk303_dn11 * assign11890_e13527) + (locals.var_t2__blk303 * (-locals.var_t3__blk304_dn11)))), (locals.var_vgp_dn12 + ((locals.var_t2__blk303_dn12 * assign11890_e13527) + (locals.var_t2__blk303 * (-locals.var_t3__blk304_dn12)))), (locals.var_vgp_dn17 + ((locals.var_t2__blk303_dn17 * assign11890_e13527) + (locals.var_t2__blk303 * (-locals.var_t3__blk304_dn17)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11890_e13531;
        locals.var_t10_dn0 = assign11890_e13531_d_n0;
        locals.var_t10_dn2 = assign11890_e13531_d_n2;
        locals.var_t10_dn6 = assign11890_e13531_d_n6;
        locals.var_t10_dn7 = assign11890_e13531_d_n7;
        locals.var_t10_dn10 = assign11890_e13531_d_n10;
        locals.var_t10_dn11 = assign11890_e13531_d_n11;
        locals.var_t10_dn12 = assign11890_e13531_d_n12;
        locals.var_t10_dn17 = assign11890_e13531_d_n17;

        let (assign11900_e13547, assign11900_e13547_d_n0, assign11900_e13547_d_n2, assign11900_e13547_d_n6, assign11900_e13547_d_n7, assign11900_e13547_d_n10, assign11900_e13547_d_n11, assign11900_e13547_d_n12, assign11900_e13547_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11900_e13538: f64 = (locals.var_t10 * locals.var_t10);
        let assign11900_e13541: f64 = (4.0 * 0.01);
        let assign11900_e13543: f64 = (assign11900_e13541 * 0.01);
        let assign11900_e13544: f64 = (assign11900_e13538 + assign11900_e13543);
        let assign11900_e13545: f64 = (assign11900_e13544).sqrt();
        (assign11900_e13545, (((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)) / (2.0 * assign11900_e13545)), (((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)) / (2.0 * assign11900_e13545)), (((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)) / (2.0 * assign11900_e13545)), (((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)) / (2.0 * assign11900_e13545)), (((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)) / (2.0 * assign11900_e13545)), (((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)) / (2.0 * assign11900_e13545)), (((locals.var_t10_dn12 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn12)) / (2.0 * assign11900_e13545)), (((locals.var_t10_dn17 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn17)) / (2.0 * assign11900_e13545)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign11900_e13547;
        locals.var_tmf1_dn0 = assign11900_e13547_d_n0;
        locals.var_tmf1_dn2 = assign11900_e13547_d_n2;
        locals.var_tmf1_dn6 = assign11900_e13547_d_n6;
        locals.var_tmf1_dn7 = assign11900_e13547_d_n7;
        locals.var_tmf1_dn10 = assign11900_e13547_d_n10;
        locals.var_tmf1_dn11 = assign11900_e13547_d_n11;
        locals.var_tmf1_dn12 = assign11900_e13547_d_n12;
        locals.var_tmf1_dn17 = assign11900_e13547_d_n17;

        let (assign11910_e13562, assign11910_e13562_d_n0, assign11910_e13562_d_n2, assign11910_e13562_d_n6, assign11910_e13562_d_n7, assign11910_e13562_d_n10, assign11910_e13562_d_n11, assign11910_e13562_d_n12, assign11910_e13562_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11910_e13555: f64 = (locals.var_t10 + locals.var_tmf1);
        let assign11910_e13556: f64 = (0.5 * assign11910_e13555);
        let assign11910_e13559: f64 = (1e-10 * 0.01);
        let assign11910_e13560: f64 = (assign11910_e13556 + assign11910_e13559);
        (assign11910_e13560, (0.5 * (locals.var_t10_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t10_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t10_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t10_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t10_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t10_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t10_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t10_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11910_e13562;
        locals.var_t10_dn0 = assign11910_e13562_d_n0;
        locals.var_t10_dn2 = assign11910_e13562_d_n2;
        locals.var_t10_dn6 = assign11910_e13562_d_n6;
        locals.var_t10_dn7 = assign11910_e13562_d_n7;
        locals.var_t10_dn10 = assign11910_e13562_d_n10;
        locals.var_t10_dn11 = assign11910_e13562_d_n11;
        locals.var_t10_dn12 = assign11910_e13562_d_n12;
        locals.var_t10_dn17 = assign11910_e13562_d_n17;

        let assign11920_e13565: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign11920_e13565;

        let (assign11930_e13574, assign11930_e13574_d_n0, assign11930_e13574_d_n2, assign11930_e13574_d_n6, assign11930_e13574_d_n7, assign11930_e13574_d_n10, assign11930_e13574_d_n11, assign11930_e13574_d_n12, assign11930_e13574_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard315 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11930_e13574;
        locals.var_t10_dn0 = assign11930_e13574_d_n0;
        locals.var_t10_dn2 = assign11930_e13574_d_n2;
        locals.var_t10_dn6 = assign11930_e13574_d_n6;
        locals.var_t10_dn7 = assign11930_e13574_d_n7;
        locals.var_t10_dn10 = assign11930_e13574_d_n10;
        locals.var_t10_dn11 = assign11930_e13574_d_n11;
        locals.var_t10_dn12 = assign11930_e13574_d_n12;
        locals.var_t10_dn17 = assign11930_e13574_d_n17;

        let (assign11950_e13590, assign11950_e13590_d_n0, assign11950_e13590_d_n2, assign11950_e13590_d_n6, assign11950_e13590_d_n7, assign11950_e13590_d_n10, assign11950_e13590_d_n11, assign11950_e13590_d_n12, assign11950_e13590_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11950_e13588: f64 = (locals.var_vds / locals.var_t10);
        (assign11950_e13588, (((locals.var_vds_dn0 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn2 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn6 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn7 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn10 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn11 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn12 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn12)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn17 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn17)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1__blk302, locals.var_t1__blk302_dn0, locals.var_t1__blk302_dn2, locals.var_t1__blk302_dn6, locals.var_t1__blk302_dn7, locals.var_t1__blk302_dn10, locals.var_t1__blk302_dn11, locals.var_t1__blk302_dn12, locals.var_t1__blk302_dn17,)
    }
};
        locals.var_t1__blk302 = assign11950_e13590;
        locals.var_t1__blk302_dn0 = assign11950_e13590_d_n0;
        locals.var_t1__blk302_dn2 = assign11950_e13590_d_n2;
        locals.var_t1__blk302_dn6 = assign11950_e13590_d_n6;
        locals.var_t1__blk302_dn7 = assign11950_e13590_d_n7;
        locals.var_t1__blk302_dn10 = assign11950_e13590_d_n10;
        locals.var_t1__blk302_dn11 = assign11950_e13590_d_n11;
        locals.var_t1__blk302_dn12 = assign11950_e13590_d_n12;
        locals.var_t1__blk302_dn17 = assign11950_e13590_d_n17;

        let (assign11960_e13601, assign11960_e13601_d_n0, assign11960_e13601_d_n2, assign11960_e13601_d_n6, assign11960_e13601_d_n7, assign11960_e13601_d_n10, assign11960_e13601_d_n11, assign11960_e13601_d_n12, assign11960_e13601_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11960_e13598: f64 = (locals.var_ddlte - 1.0);
        let assign11960_e13599: f64 = (locals.var_t1__blk302).powf(assign11960_e13598);
        (assign11960_e13599, if 0.0 == 0.0 && ((assign11960_e13598) as f64).is_finite() && ((assign11960_e13598) as f64).fract() == 0.0 { if assign11960_e13598 == 0.0 { 0.0 } else { (assign11960_e13598 * ((locals.var_t1__blk302).powf(assign11960_e13598 - 1.0) * locals.var_t1__blk302_dn0)) } } else { (assign11960_e13599 * (assign11960_e13598 * (locals.var_t1__blk302_dn0 / locals.var_t1__blk302))) }, if 0.0 == 0.0 && ((assign11960_e13598) as f64).is_finite() && ((assign11960_e13598) as f64).fract() == 0.0 { if assign11960_e13598 == 0.0 { 0.0 } else { (assign11960_e13598 * ((locals.var_t1__blk302).powf(assign11960_e13598 - 1.0) * locals.var_t1__blk302_dn2)) } } else { (assign11960_e13599 * (assign11960_e13598 * (locals.var_t1__blk302_dn2 / locals.var_t1__blk302))) }, if 0.0 == 0.0 && ((assign11960_e13598) as f64).is_finite() && ((assign11960_e13598) as f64).fract() == 0.0 { if assign11960_e13598 == 0.0 { 0.0 } else { (assign11960_e13598 * ((locals.var_t1__blk302).powf(assign11960_e13598 - 1.0) * locals.var_t1__blk302_dn6)) } } else { (assign11960_e13599 * (assign11960_e13598 * (locals.var_t1__blk302_dn6 / locals.var_t1__blk302))) }, if 0.0 == 0.0 && ((assign11960_e13598) as f64).is_finite() && ((assign11960_e13598) as f64).fract() == 0.0 { if assign11960_e13598 == 0.0 { 0.0 } else { (assign11960_e13598 * ((locals.var_t1__blk302).powf(assign11960_e13598 - 1.0) * locals.var_t1__blk302_dn7)) } } else { (assign11960_e13599 * (assign11960_e13598 * (locals.var_t1__blk302_dn7 / locals.var_t1__blk302))) }, if 0.0 == 0.0 && ((assign11960_e13598) as f64).is_finite() && ((assign11960_e13598) as f64).fract() == 0.0 { if assign11960_e13598 == 0.0 { 0.0 } else { (assign11960_e13598 * ((locals.var_t1__blk302).powf(assign11960_e13598 - 1.0) * locals.var_t1__blk302_dn10)) } } else { (assign11960_e13599 * (assign11960_e13598 * (locals.var_t1__blk302_dn10 / locals.var_t1__blk302))) }, if 0.0 == 0.0 && ((assign11960_e13598) as f64).is_finite() && ((assign11960_e13598) as f64).fract() == 0.0 { if assign11960_e13598 == 0.0 { 0.0 } else { (assign11960_e13598 * ((locals.var_t1__blk302).powf(assign11960_e13598 - 1.0) * locals.var_t1__blk302_dn11)) } } else { (assign11960_e13599 * (assign11960_e13598 * (locals.var_t1__blk302_dn11 / locals.var_t1__blk302))) }, if 0.0 == 0.0 && ((assign11960_e13598) as f64).is_finite() && ((assign11960_e13598) as f64).fract() == 0.0 { if assign11960_e13598 == 0.0 { 0.0 } else { (assign11960_e13598 * ((locals.var_t1__blk302).powf(assign11960_e13598 - 1.0) * locals.var_t1__blk302_dn12)) } } else { (assign11960_e13599 * (assign11960_e13598 * (locals.var_t1__blk302_dn12 / locals.var_t1__blk302))) }, if 0.0 == 0.0 && ((assign11960_e13598) as f64).is_finite() && ((assign11960_e13598) as f64).fract() == 0.0 { if assign11960_e13598 == 0.0 { 0.0 } else { (assign11960_e13598 * ((locals.var_t1__blk302).powf(assign11960_e13598 - 1.0) * locals.var_t1__blk302_dn17)) } } else { (assign11960_e13599 * (assign11960_e13598 * (locals.var_t1__blk302_dn17 / locals.var_t1__blk302))) },)
    } else {
        (locals.var_t2__blk303, locals.var_t2__blk303_dn0, locals.var_t2__blk303_dn2, locals.var_t2__blk303_dn6, locals.var_t2__blk303_dn7, locals.var_t2__blk303_dn10, locals.var_t2__blk303_dn11, locals.var_t2__blk303_dn12, locals.var_t2__blk303_dn17,)
    }
};
        locals.var_t2__blk303 = assign11960_e13601;
        locals.var_t2__blk303_dn0 = assign11960_e13601_d_n0;
        locals.var_t2__blk303_dn2 = assign11960_e13601_d_n2;
        locals.var_t2__blk303_dn6 = assign11960_e13601_d_n6;
        locals.var_t2__blk303_dn7 = assign11960_e13601_d_n7;
        locals.var_t2__blk303_dn10 = assign11960_e13601_d_n10;
        locals.var_t2__blk303_dn11 = assign11960_e13601_d_n11;
        locals.var_t2__blk303_dn12 = assign11960_e13601_d_n12;
        locals.var_t2__blk303_dn17 = assign11960_e13601_d_n17;

        let (assign11970_e13610, assign11970_e13610_d_n0, assign11970_e13610_d_n2, assign11970_e13610_d_n6, assign11970_e13610_d_n7, assign11970_e13610_d_n10, assign11970_e13610_d_n11, assign11970_e13610_d_n12, assign11970_e13610_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11970_e13608: f64 = (locals.var_t2__blk303 * locals.var_t1__blk302);
        (assign11970_e13608, ((locals.var_t2__blk303_dn0 * locals.var_t1__blk302) + (locals.var_t2__blk303 * locals.var_t1__blk302_dn0)), ((locals.var_t2__blk303_dn2 * locals.var_t1__blk302) + (locals.var_t2__blk303 * locals.var_t1__blk302_dn2)), ((locals.var_t2__blk303_dn6 * locals.var_t1__blk302) + (locals.var_t2__blk303 * locals.var_t1__blk302_dn6)), ((locals.var_t2__blk303_dn7 * locals.var_t1__blk302) + (locals.var_t2__blk303 * locals.var_t1__blk302_dn7)), ((locals.var_t2__blk303_dn10 * locals.var_t1__blk302) + (locals.var_t2__blk303 * locals.var_t1__blk302_dn10)), ((locals.var_t2__blk303_dn11 * locals.var_t1__blk302) + (locals.var_t2__blk303 * locals.var_t1__blk302_dn11)), ((locals.var_t2__blk303_dn12 * locals.var_t1__blk302) + (locals.var_t2__blk303 * locals.var_t1__blk302_dn12)), ((locals.var_t2__blk303_dn17 * locals.var_t1__blk302) + (locals.var_t2__blk303 * locals.var_t1__blk302_dn17)),)
    } else {
        (locals.var_t7__blk307, locals.var_t7__blk307_dn0, locals.var_t7__blk307_dn2, locals.var_t7__blk307_dn6, locals.var_t7__blk307_dn7, locals.var_t7__blk307_dn10, locals.var_t7__blk307_dn11, locals.var_t7__blk307_dn12, locals.var_t7__blk307_dn17,)
    }
};
        locals.var_t7__blk307 = assign11970_e13610;
        locals.var_t7__blk307_dn0 = assign11970_e13610_d_n0;
        locals.var_t7__blk307_dn2 = assign11970_e13610_d_n2;
        locals.var_t7__blk307_dn6 = assign11970_e13610_d_n6;
        locals.var_t7__blk307_dn7 = assign11970_e13610_d_n7;
        locals.var_t7__blk307_dn10 = assign11970_e13610_d_n10;
        locals.var_t7__blk307_dn11 = assign11970_e13610_d_n11;
        locals.var_t7__blk307_dn12 = assign11970_e13610_d_n12;
        locals.var_t7__blk307_dn17 = assign11970_e13610_d_n17;

        let (assign11980_e13619, assign11980_e13619_d_n0, assign11980_e13619_d_n2, assign11980_e13619_d_n6, assign11980_e13619_d_n7, assign11980_e13619_d_n10, assign11980_e13619_d_n11, assign11980_e13619_d_n12, assign11980_e13619_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11980_e13617: f64 = (1.0 + locals.var_t7__blk307);
        (assign11980_e13617, locals.var_t7__blk307_dn0, locals.var_t7__blk307_dn2, locals.var_t7__blk307_dn6, locals.var_t7__blk307_dn7, locals.var_t7__blk307_dn10, locals.var_t7__blk307_dn11, locals.var_t7__blk307_dn12, locals.var_t7__blk307_dn17,)
    } else {
        (locals.var_t3__blk304, locals.var_t3__blk304_dn0, locals.var_t3__blk304_dn2, locals.var_t3__blk304_dn6, locals.var_t3__blk304_dn7, locals.var_t3__blk304_dn10, locals.var_t3__blk304_dn11, locals.var_t3__blk304_dn12, locals.var_t3__blk304_dn17,)
    }
};
        locals.var_t3__blk304 = assign11980_e13619;
        locals.var_t3__blk304_dn0 = assign11980_e13619_d_n0;
        locals.var_t3__blk304_dn2 = assign11980_e13619_d_n2;
        locals.var_t3__blk304_dn6 = assign11980_e13619_d_n6;
        locals.var_t3__blk304_dn7 = assign11980_e13619_d_n7;
        locals.var_t3__blk304_dn10 = assign11980_e13619_d_n10;
        locals.var_t3__blk304_dn11 = assign11980_e13619_d_n11;
        locals.var_t3__blk304_dn12 = assign11980_e13619_d_n12;
        locals.var_t3__blk304_dn17 = assign11980_e13619_d_n17;

        let (assign11990_e13632, assign11990_e13632_d_n0, assign11990_e13632_d_n2, assign11990_e13632_d_n6, assign11990_e13632_d_n7, assign11990_e13632_d_n10, assign11990_e13632_d_n11, assign11990_e13632_d_n12, assign11990_e13632_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign11990_e13627: f64 = (1.0 / locals.var_ddlte);
        let assign11990_e13629: f64 = (assign11990_e13627 - 1.0);
        let assign11990_e13630: f64 = (locals.var_t3__blk304).powf(assign11990_e13629);
        (assign11990_e13630, if 0.0 == 0.0 && ((assign11990_e13629) as f64).is_finite() && ((assign11990_e13629) as f64).fract() == 0.0 { if assign11990_e13629 == 0.0 { 0.0 } else { (assign11990_e13629 * ((locals.var_t3__blk304).powf(assign11990_e13629 - 1.0) * locals.var_t3__blk304_dn0)) } } else { (assign11990_e13630 * (assign11990_e13629 * (locals.var_t3__blk304_dn0 / locals.var_t3__blk304))) }, if 0.0 == 0.0 && ((assign11990_e13629) as f64).is_finite() && ((assign11990_e13629) as f64).fract() == 0.0 { if assign11990_e13629 == 0.0 { 0.0 } else { (assign11990_e13629 * ((locals.var_t3__blk304).powf(assign11990_e13629 - 1.0) * locals.var_t3__blk304_dn2)) } } else { (assign11990_e13630 * (assign11990_e13629 * (locals.var_t3__blk304_dn2 / locals.var_t3__blk304))) }, if 0.0 == 0.0 && ((assign11990_e13629) as f64).is_finite() && ((assign11990_e13629) as f64).fract() == 0.0 { if assign11990_e13629 == 0.0 { 0.0 } else { (assign11990_e13629 * ((locals.var_t3__blk304).powf(assign11990_e13629 - 1.0) * locals.var_t3__blk304_dn6)) } } else { (assign11990_e13630 * (assign11990_e13629 * (locals.var_t3__blk304_dn6 / locals.var_t3__blk304))) }, if 0.0 == 0.0 && ((assign11990_e13629) as f64).is_finite() && ((assign11990_e13629) as f64).fract() == 0.0 { if assign11990_e13629 == 0.0 { 0.0 } else { (assign11990_e13629 * ((locals.var_t3__blk304).powf(assign11990_e13629 - 1.0) * locals.var_t3__blk304_dn7)) } } else { (assign11990_e13630 * (assign11990_e13629 * (locals.var_t3__blk304_dn7 / locals.var_t3__blk304))) }, if 0.0 == 0.0 && ((assign11990_e13629) as f64).is_finite() && ((assign11990_e13629) as f64).fract() == 0.0 { if assign11990_e13629 == 0.0 { 0.0 } else { (assign11990_e13629 * ((locals.var_t3__blk304).powf(assign11990_e13629 - 1.0) * locals.var_t3__blk304_dn10)) } } else { (assign11990_e13630 * (assign11990_e13629 * (locals.var_t3__blk304_dn10 / locals.var_t3__blk304))) }, if 0.0 == 0.0 && ((assign11990_e13629) as f64).is_finite() && ((assign11990_e13629) as f64).fract() == 0.0 { if assign11990_e13629 == 0.0 { 0.0 } else { (assign11990_e13629 * ((locals.var_t3__blk304).powf(assign11990_e13629 - 1.0) * locals.var_t3__blk304_dn11)) } } else { (assign11990_e13630 * (assign11990_e13629 * (locals.var_t3__blk304_dn11 / locals.var_t3__blk304))) }, if 0.0 == 0.0 && ((assign11990_e13629) as f64).is_finite() && ((assign11990_e13629) as f64).fract() == 0.0 { if assign11990_e13629 == 0.0 { 0.0 } else { (assign11990_e13629 * ((locals.var_t3__blk304).powf(assign11990_e13629 - 1.0) * locals.var_t3__blk304_dn12)) } } else { (assign11990_e13630 * (assign11990_e13629 * (locals.var_t3__blk304_dn12 / locals.var_t3__blk304))) }, if 0.0 == 0.0 && ((assign11990_e13629) as f64).is_finite() && ((assign11990_e13629) as f64).fract() == 0.0 { if assign11990_e13629 == 0.0 { 0.0 } else { (assign11990_e13629 * ((locals.var_t3__blk304).powf(assign11990_e13629 - 1.0) * locals.var_t3__blk304_dn17)) } } else { (assign11990_e13630 * (assign11990_e13629 * (locals.var_t3__blk304_dn17 / locals.var_t3__blk304))) },)
    } else {
        (locals.var_t4__blk305, locals.var_t4__blk305_dn0, locals.var_t4__blk305_dn2, locals.var_t4__blk305_dn6, locals.var_t4__blk305_dn7, locals.var_t4__blk305_dn10, locals.var_t4__blk305_dn11, locals.var_t4__blk305_dn12, locals.var_t4__blk305_dn17,)
    }
};
        locals.var_t4__blk305 = assign11990_e13632;
        locals.var_t4__blk305_dn0 = assign11990_e13632_d_n0;
        locals.var_t4__blk305_dn2 = assign11990_e13632_d_n2;
        locals.var_t4__blk305_dn6 = assign11990_e13632_d_n6;
        locals.var_t4__blk305_dn7 = assign11990_e13632_d_n7;
        locals.var_t4__blk305_dn10 = assign11990_e13632_d_n10;
        locals.var_t4__blk305_dn11 = assign11990_e13632_d_n11;
        locals.var_t4__blk305_dn12 = assign11990_e13632_d_n12;
        locals.var_t4__blk305_dn17 = assign11990_e13632_d_n17;

        let (assign12000_e13641, assign12000_e13641_d_n0, assign12000_e13641_d_n2, assign12000_e13641_d_n6, assign12000_e13641_d_n7, assign12000_e13641_d_n10, assign12000_e13641_d_n11, assign12000_e13641_d_n12, assign12000_e13641_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12000_e13639: f64 = (locals.var_t4__blk305 * locals.var_t3__blk304);
        (assign12000_e13639, ((locals.var_t4__blk305_dn0 * locals.var_t3__blk304) + (locals.var_t4__blk305 * locals.var_t3__blk304_dn0)), ((locals.var_t4__blk305_dn2 * locals.var_t3__blk304) + (locals.var_t4__blk305 * locals.var_t3__blk304_dn2)), ((locals.var_t4__blk305_dn6 * locals.var_t3__blk304) + (locals.var_t4__blk305 * locals.var_t3__blk304_dn6)), ((locals.var_t4__blk305_dn7 * locals.var_t3__blk304) + (locals.var_t4__blk305 * locals.var_t3__blk304_dn7)), ((locals.var_t4__blk305_dn10 * locals.var_t3__blk304) + (locals.var_t4__blk305 * locals.var_t3__blk304_dn10)), ((locals.var_t4__blk305_dn11 * locals.var_t3__blk304) + (locals.var_t4__blk305 * locals.var_t3__blk304_dn11)), ((locals.var_t4__blk305_dn12 * locals.var_t3__blk304) + (locals.var_t4__blk305 * locals.var_t3__blk304_dn12)), ((locals.var_t4__blk305_dn17 * locals.var_t3__blk304) + (locals.var_t4__blk305 * locals.var_t3__blk304_dn17)),)
    } else {
        (locals.var_t6__blk306, locals.var_t6__blk306_dn0, locals.var_t6__blk306_dn2, locals.var_t6__blk306_dn6, locals.var_t6__blk306_dn7, locals.var_t6__blk306_dn10, locals.var_t6__blk306_dn11, locals.var_t6__blk306_dn12, locals.var_t6__blk306_dn17,)
    }
};
        locals.var_t6__blk306 = assign12000_e13641;
        locals.var_t6__blk306_dn0 = assign12000_e13641_d_n0;
        locals.var_t6__blk306_dn2 = assign12000_e13641_d_n2;
        locals.var_t6__blk306_dn6 = assign12000_e13641_d_n6;
        locals.var_t6__blk306_dn7 = assign12000_e13641_d_n7;
        locals.var_t6__blk306_dn10 = assign12000_e13641_d_n10;
        locals.var_t6__blk306_dn11 = assign12000_e13641_d_n11;
        locals.var_t6__blk306_dn12 = assign12000_e13641_d_n12;
        locals.var_t6__blk306_dn17 = assign12000_e13641_d_n17;

        let (assign12010_e13650, assign12010_e13650_d_n0, assign12010_e13650_d_n2, assign12010_e13650_d_n6, assign12010_e13650_d_n7, assign12010_e13650_d_n10, assign12010_e13650_d_n11, assign12010_e13650_d_n12, assign12010_e13650_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12010_e13648: f64 = (locals.var_vds / locals.var_t6__blk306);
        (assign12010_e13648, (((locals.var_vds_dn0 * locals.var_t6__blk306) - (locals.var_vds * locals.var_t6__blk306_dn0)) / (locals.var_t6__blk306 * locals.var_t6__blk306)), (((locals.var_vds_dn2 * locals.var_t6__blk306) - (locals.var_vds * locals.var_t6__blk306_dn2)) / (locals.var_t6__blk306 * locals.var_t6__blk306)), (((locals.var_vds_dn6 * locals.var_t6__blk306) - (locals.var_vds * locals.var_t6__blk306_dn6)) / (locals.var_t6__blk306 * locals.var_t6__blk306)), (((locals.var_vds_dn7 * locals.var_t6__blk306) - (locals.var_vds * locals.var_t6__blk306_dn7)) / (locals.var_t6__blk306 * locals.var_t6__blk306)), (((locals.var_vds_dn10 * locals.var_t6__blk306) - (locals.var_vds * locals.var_t6__blk306_dn10)) / (locals.var_t6__blk306 * locals.var_t6__blk306)), (((locals.var_vds_dn11 * locals.var_t6__blk306) - (locals.var_vds * locals.var_t6__blk306_dn11)) / (locals.var_t6__blk306 * locals.var_t6__blk306)), (((locals.var_vds_dn12 * locals.var_t6__blk306) - (locals.var_vds * locals.var_t6__blk306_dn12)) / (locals.var_t6__blk306 * locals.var_t6__blk306)), (((locals.var_vds_dn17 * locals.var_t6__blk306) - (locals.var_vds * locals.var_t6__blk306_dn17)) / (locals.var_t6__blk306 * locals.var_t6__blk306)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    }
};
        locals.var_vdseff = assign12010_e13650;
        locals.var_vdseff_dn0 = assign12010_e13650_d_n0;
        locals.var_vdseff_dn2 = assign12010_e13650_d_n2;
        locals.var_vdseff_dn6 = assign12010_e13650_d_n6;
        locals.var_vdseff_dn7 = assign12010_e13650_d_n7;
        locals.var_vdseff_dn10 = assign12010_e13650_d_n10;
        locals.var_vdseff_dn11 = assign12010_e13650_d_n11;
        locals.var_vdseff_dn12 = assign12010_e13650_d_n12;
        locals.var_vdseff_dn17 = assign12010_e13650_d_n17;

        let (assign12020_e13657, assign12020_e13657_d_n0, assign12020_e13657_d_n2, assign12020_e13657_d_n6, assign12020_e13657_d_n7, assign12020_e13657_d_n10, assign12020_e13657_d_n11, assign12020_e13657_d_n12, assign12020_e13657_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign12020_e13657;
        locals.var_vds_dn0 = assign12020_e13657_d_n0;
        locals.var_vds_dn2 = assign12020_e13657_d_n2;
        locals.var_vds_dn6 = assign12020_e13657_d_n6;
        locals.var_vds_dn7 = assign12020_e13657_d_n7;
        locals.var_vds_dn10 = assign12020_e13657_d_n10;
        locals.var_vds_dn11 = assign12020_e13657_d_n11;
        locals.var_vds_dn12 = assign12020_e13657_d_n12;
        locals.var_vds_dn17 = assign12020_e13657_d_n17;

        let assign12030_e13660: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign12030_e13660;

        let (assign12040_e13669, assign12040_e13669_d_n0, assign12040_e13669_d_n2, assign12040_e13669_d_n6, assign12040_e13669_d_n7, assign12040_e13669_d_n10, assign12040_e13669_d_n11, assign12040_e13669_d_n12, assign12040_e13669_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12040_e13669;
        locals.var_psl_dn0 = assign12040_e13669_d_n0;
        locals.var_psl_dn2 = assign12040_e13669_d_n2;
        locals.var_psl_dn6 = assign12040_e13669_d_n6;
        locals.var_psl_dn7 = assign12040_e13669_d_n7;
        locals.var_psl_dn10 = assign12040_e13669_d_n10;
        locals.var_psl_dn11 = assign12040_e13669_d_n11;
        locals.var_psl_dn12 = assign12040_e13669_d_n12;
        locals.var_psl_dn17 = assign12040_e13669_d_n17;

        let (assign12050_e13680, assign12050_e13680_d_n0, assign12050_e13680_d_n2, assign12050_e13680_d_n6, assign12050_e13680_d_n7, assign12050_e13680_d_n10, assign12050_e13680_d_n11, assign12050_e13680_d_n12, assign12050_e13680_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 != 0.0)) {
        let assign12050_e13678: f64 = (locals.var_psl - locals.var_ps0);
        (assign12050_e13678, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn12 - locals.var_ps0_dn12), (locals.var_psl_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign12050_e13680;
        locals.var_pds_dn0 = assign12050_e13680_d_n0;
        locals.var_pds_dn2 = assign12050_e13680_d_n2;
        locals.var_pds_dn6 = assign12050_e13680_d_n6;
        locals.var_pds_dn7 = assign12050_e13680_d_n7;
        locals.var_pds_dn10 = assign12050_e13680_d_n10;
        locals.var_pds_dn11 = assign12050_e13680_d_n11;
        locals.var_pds_dn12 = assign12050_e13680_d_n12;
        locals.var_pds_dn17 = assign12050_e13680_d_n17;

        let (assign12060_e13689, assign12060_e13689_d_n0, assign12060_e13689_d_n2, assign12060_e13689_d_n6, assign12060_e13689_d_n7, assign12060_e13689_d_n10, assign12060_e13689_d_n11, assign12060_e13689_d_n12, assign12060_e13689_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12060_e13689;
        locals.var_phi_sl_soi_dn0 = assign12060_e13689_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12060_e13689_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12060_e13689_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12060_e13689_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12060_e13689_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12060_e13689_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12060_e13689_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12060_e13689_d_n17;

        let (assign12070_e13698, assign12070_e13698_d_n0, assign12070_e13698_d_n2, assign12070_e13698_d_n6, assign12070_e13698_d_n7, assign12070_e13698_d_n10, assign12070_e13698_d_n11, assign12070_e13698_d_n12, assign12070_e13698_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 != 0.0)) {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12070_e13698;
        locals.var_phi_bl_soi_dn0 = assign12070_e13698_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12070_e13698_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12070_e13698_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12070_e13698_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12070_e13698_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12070_e13698_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12070_e13698_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12070_e13698_d_n17;

        let (assign12080_e13707, assign12080_e13707_d_n0, assign12080_e13707_d_n2, assign12080_e13707_d_n6, assign12080_e13707_d_n7, assign12080_e13707_d_n10, assign12080_e13707_d_n11, assign12080_e13707_d_n12, assign12080_e13707_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 != 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12080_e13707;
        locals.var_phi_sl_bulk_dn0 = assign12080_e13707_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12080_e13707_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12080_e13707_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12080_e13707_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12080_e13707_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12080_e13707_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12080_e13707_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12080_e13707_d_n17;

        let (assign12090_e13716,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign12090_e13716;

        let assign12110_e13728: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign12110_e13728;

        let (assign12120_e13740, assign12120_e13740_d_n0, assign12120_e13740_d_n2, assign12120_e13740_d_n6, assign12120_e13740_d_n7, assign12120_e13740_d_n10, assign12120_e13740_d_n11, assign12120_e13740_d_n12, assign12120_e13740_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 != 0.0)) {
        (locals.var_pssl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12120_e13740;
        locals.var_phi_sl_soi_dn0 = assign12120_e13740_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12120_e13740_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12120_e13740_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12120_e13740_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12120_e13740_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12120_e13740_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12120_e13740_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12120_e13740_d_n17;

        let (assign12130_e13752, assign12130_e13752_d_n0, assign12130_e13752_d_n2, assign12130_e13752_d_n6, assign12130_e13752_d_n7, assign12130_e13752_d_n10, assign12130_e13752_d_n11, assign12130_e13752_d_n12, assign12130_e13752_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 != 0.0)) {
        (locals.var_pbsl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12130_e13752;
        locals.var_phi_bl_soi_dn0 = assign12130_e13752_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12130_e13752_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12130_e13752_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12130_e13752_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12130_e13752_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12130_e13752_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12130_e13752_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12130_e13752_d_n17;

        let (assign12140_e13764, assign12140_e13764_d_n0, assign12140_e13764_d_n2, assign12140_e13764_d_n6, assign12140_e13764_d_n7, assign12140_e13764_d_n10, assign12140_e13764_d_n11, assign12140_e13764_d_n12, assign12140_e13764_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 != 0.0)) {
        (locals.var_psbl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12140_e13764;
        locals.var_phi_sl_bulk_dn0 = assign12140_e13764_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12140_e13764_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12140_e13764_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12140_e13764_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12140_e13764_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12140_e13764_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12140_e13764_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12140_e13764_d_n17;

    }

    pub(super) fn stamp_transient_block_35(
        locals: &mut StampLocals,
    ) {
        let (assign12160_e13803, assign12160_e13803_d_n0, assign12160_e13803_d_n2, assign12160_e13803_d_n6, assign12160_e13803_d_n7, assign12160_e13803_d_n10, assign12160_e13803_d_n11, assign12160_e13803_d_n12, assign12160_e13803_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign12160_e13794: f64 = (locals.var_psl_lim - locals.var_phi_s0_soi);
        let (assign12160_e13801, assign12160_e13801_d_n0, assign12160_e13801_d_n2, assign12160_e13801_d_n6, assign12160_e13801_d_n7, assign12160_e13801_d_n10, assign12160_e13801_d_n11, assign12160_e13801_d_n12, assign12160_e13801_d_n17,) = {
            if (assign12160_e13794 >= 0.0) {
                let assign12160_e13799: f64 = (locals.var_psl_lim - locals.var_phi_s0_soi);
                (assign12160_e13799, (locals.var_psl_lim_dn0 - locals.var_phi_s0_soi_dn0), (locals.var_psl_lim_dn2 - locals.var_phi_s0_soi_dn2), (locals.var_psl_lim_dn6 - locals.var_phi_s0_soi_dn6), (locals.var_psl_lim_dn7 - locals.var_phi_s0_soi_dn7), (locals.var_psl_lim_dn10 - locals.var_phi_s0_soi_dn10), (locals.var_psl_lim_dn11 - locals.var_phi_s0_soi_dn11), (locals.var_psl_lim_dn12 - locals.var_phi_s0_soi_dn12), (locals.var_psl_lim_dn17 - locals.var_phi_s0_soi_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12160_e13801, assign12160_e13801_d_n0, assign12160_e13801_d_n2, assign12160_e13801_d_n6, assign12160_e13801_d_n7, assign12160_e13801_d_n10, assign12160_e13801_d_n11, assign12160_e13801_d_n12, assign12160_e13801_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign12160_e13803;
        locals.var_pds_max_dn0 = assign12160_e13803_d_n0;
        locals.var_pds_max_dn2 = assign12160_e13803_d_n2;
        locals.var_pds_max_dn6 = assign12160_e13803_d_n6;
        locals.var_pds_max_dn7 = assign12160_e13803_d_n7;
        locals.var_pds_max_dn10 = assign12160_e13803_d_n10;
        locals.var_pds_max_dn11 = assign12160_e13803_d_n11;
        locals.var_pds_max_dn12 = assign12160_e13803_d_n12;
        locals.var_pds_max_dn17 = assign12160_e13803_d_n17;

        let (assign12170_e13824, assign12170_e13824_d_n0, assign12170_e13824_d_n2, assign12170_e13824_d_n6, assign12170_e13824_d_n7, assign12170_e13824_d_n10, assign12170_e13824_d_n11, assign12170_e13824_d_n12, assign12170_e13824_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign12170_e13816: f64 = (1.0 + 0.3);
        let assign12170_e13818: f64 = (assign12170_e13816 * locals.var_pds_max);
        let assign12170_e13820: f64 = (assign12170_e13818 - locals.var_vds);
        let assign12170_e13822: f64 = (assign12170_e13820 - 0.03);
        (assign12170_e13822, ((assign12170_e13816 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign12170_e13816 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign12170_e13816 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign12170_e13816 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign12170_e13816 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign12170_e13816 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign12170_e13816 * locals.var_pds_max_dn12) - locals.var_vds_dn12), ((assign12170_e13816 * locals.var_pds_max_dn17) - locals.var_vds_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12170_e13824;
        locals.var_tmf1_dn0 = assign12170_e13824_d_n0;
        locals.var_tmf1_dn2 = assign12170_e13824_d_n2;
        locals.var_tmf1_dn6 = assign12170_e13824_d_n6;
        locals.var_tmf1_dn7 = assign12170_e13824_d_n7;
        locals.var_tmf1_dn10 = assign12170_e13824_d_n10;
        locals.var_tmf1_dn11 = assign12170_e13824_d_n11;
        locals.var_tmf1_dn12 = assign12170_e13824_d_n12;
        locals.var_tmf1_dn17 = assign12170_e13824_d_n17;

        let (assign12180_e13845, assign12180_e13845_d_n0, assign12180_e13845_d_n2, assign12180_e13845_d_n6, assign12180_e13845_d_n7, assign12180_e13845_d_n10, assign12180_e13845_d_n11, assign12180_e13845_d_n12, assign12180_e13845_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign12180_e13838: f64 = (1.0 + 0.3);
        let assign12180_e13840: f64 = (assign12180_e13838 * locals.var_pds_max);
        let assign12180_e13841: f64 = (4.0 * assign12180_e13840);
        let assign12180_e13843: f64 = (assign12180_e13841 * 0.03);
        (assign12180_e13843, ((4.0 * (assign12180_e13838 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign12180_e13838 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign12180_e13838 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign12180_e13838 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign12180_e13838 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign12180_e13838 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign12180_e13838 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign12180_e13838 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12180_e13845;
        locals.var_tmf2_dn0 = assign12180_e13845_d_n0;
        locals.var_tmf2_dn2 = assign12180_e13845_d_n2;
        locals.var_tmf2_dn6 = assign12180_e13845_d_n6;
        locals.var_tmf2_dn7 = assign12180_e13845_d_n7;
        locals.var_tmf2_dn10 = assign12180_e13845_d_n10;
        locals.var_tmf2_dn11 = assign12180_e13845_d_n11;
        locals.var_tmf2_dn12 = assign12180_e13845_d_n12;
        locals.var_tmf2_dn17 = assign12180_e13845_d_n17;

        let (assign12190_e13864, assign12190_e13864_d_n0, assign12190_e13864_d_n2, assign12190_e13864_d_n6, assign12190_e13864_d_n7, assign12190_e13864_d_n10, assign12190_e13864_d_n11, assign12190_e13864_d_n12, assign12190_e13864_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let (assign12190_e13862, assign12190_e13862_d_n0, assign12190_e13862_d_n2, assign12190_e13862_d_n6, assign12190_e13862_d_n7, assign12190_e13862_d_n10, assign12190_e13862_d_n11, assign12190_e13862_d_n12, assign12190_e13862_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12190_e13861: f64 = (-locals.var_tmf2);
                (assign12190_e13861, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12190_e13862, assign12190_e13862_d_n0, assign12190_e13862_d_n2, assign12190_e13862_d_n6, assign12190_e13862_d_n7, assign12190_e13862_d_n10, assign12190_e13862_d_n11, assign12190_e13862_d_n12, assign12190_e13862_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12190_e13864;
        locals.var_tmf2_dn0 = assign12190_e13864_d_n0;
        locals.var_tmf2_dn2 = assign12190_e13864_d_n2;
        locals.var_tmf2_dn6 = assign12190_e13864_d_n6;
        locals.var_tmf2_dn7 = assign12190_e13864_d_n7;
        locals.var_tmf2_dn10 = assign12190_e13864_d_n10;
        locals.var_tmf2_dn11 = assign12190_e13864_d_n11;
        locals.var_tmf2_dn12 = assign12190_e13864_d_n12;
        locals.var_tmf2_dn17 = assign12190_e13864_d_n17;

        let (assign12200_e13882, assign12200_e13882_d_n0, assign12200_e13882_d_n2, assign12200_e13882_d_n6, assign12200_e13882_d_n7, assign12200_e13882_d_n10, assign12200_e13882_d_n11, assign12200_e13882_d_n12, assign12200_e13882_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign12200_e13877: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12200_e13879: f64 = (assign12200_e13877 + locals.var_tmf2);
        let assign12200_e13880: f64 = (assign12200_e13879).sqrt();
        (assign12200_e13880, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12200_e13880)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12200_e13880)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12200_e13880)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12200_e13880)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12200_e13880)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12200_e13880)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12200_e13880)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12200_e13880)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12200_e13882;
        locals.var_tmf2_dn0 = assign12200_e13882_d_n0;
        locals.var_tmf2_dn2 = assign12200_e13882_d_n2;
        locals.var_tmf2_dn6 = assign12200_e13882_d_n6;
        locals.var_tmf2_dn7 = assign12200_e13882_d_n7;
        locals.var_tmf2_dn10 = assign12200_e13882_d_n10;
        locals.var_tmf2_dn11 = assign12200_e13882_d_n11;
        locals.var_tmf2_dn12 = assign12200_e13882_d_n12;
        locals.var_tmf2_dn17 = assign12200_e13882_d_n17;

        let (assign12210_e13905, assign12210_e13905_d_n0, assign12210_e13905_d_n2, assign12210_e13905_d_n6, assign12210_e13905_d_n7, assign12210_e13905_d_n10, assign12210_e13905_d_n11, assign12210_e13905_d_n12, assign12210_e13905_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign12210_e13895: f64 = (1.0 + 0.3);
        let assign12210_e13897: f64 = (assign12210_e13895 * locals.var_pds_max);
        let assign12210_e13901: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12210_e13902: f64 = (0.5 * assign12210_e13901);
        let assign12210_e13903: f64 = (assign12210_e13897 - assign12210_e13902);
        (assign12210_e13903, ((assign12210_e13895 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign12210_e13895 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign12210_e13895 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign12210_e13895 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign12210_e13895 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign12210_e13895 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign12210_e13895 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign12210_e13895 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12210_e13905;
        locals.var_pds_ini_dn0 = assign12210_e13905_d_n0;
        locals.var_pds_ini_dn2 = assign12210_e13905_d_n2;
        locals.var_pds_ini_dn6 = assign12210_e13905_d_n6;
        locals.var_pds_ini_dn7 = assign12210_e13905_d_n7;
        locals.var_pds_ini_dn10 = assign12210_e13905_d_n10;
        locals.var_pds_ini_dn11 = assign12210_e13905_d_n11;
        locals.var_pds_ini_dn12 = assign12210_e13905_d_n12;
        locals.var_pds_ini_dn17 = assign12210_e13905_d_n17;

        let (assign12220_e13923, assign12220_e13923_d_n0, assign12220_e13923_d_n2, assign12220_e13923_d_n6, assign12220_e13923_d_n7, assign12220_e13923_d_n10, assign12220_e13923_d_n11, assign12220_e13923_d_n12, assign12220_e13923_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let (assign12220_e13921, assign12220_e13921_d_n0, assign12220_e13921_d_n2, assign12220_e13921_d_n6, assign12220_e13921_d_n7, assign12220_e13921_d_n10, assign12220_e13921_d_n11, assign12220_e13921_d_n12, assign12220_e13921_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign12220_e13921, assign12220_e13921_d_n0, assign12220_e13921_d_n2, assign12220_e13921_d_n6, assign12220_e13921_d_n7, assign12220_e13921_d_n10, assign12220_e13921_d_n11, assign12220_e13921_d_n12, assign12220_e13921_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12220_e13923;
        locals.var_pds_ini_dn0 = assign12220_e13923_d_n0;
        locals.var_pds_ini_dn2 = assign12220_e13923_d_n2;
        locals.var_pds_ini_dn6 = assign12220_e13923_d_n6;
        locals.var_pds_ini_dn7 = assign12220_e13923_d_n7;
        locals.var_pds_ini_dn10 = assign12220_e13923_d_n10;
        locals.var_pds_ini_dn11 = assign12220_e13923_d_n11;
        locals.var_pds_ini_dn12 = assign12220_e13923_d_n12;
        locals.var_pds_ini_dn17 = assign12220_e13923_d_n17;

        let assign12230_e13926: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign12230_e13926;

        let (assign12240_e13941, assign12240_e13941_d_n0, assign12240_e13941_d_n2, assign12240_e13941_d_n6, assign12240_e13941_d_n7, assign12240_e13941_d_n10, assign12240_e13941_d_n11, assign12240_e13941_d_n12, assign12240_e13941_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard318 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12240_e13941;
        locals.var_pds_ini_dn0 = assign12240_e13941_d_n0;
        locals.var_pds_ini_dn2 = assign12240_e13941_d_n2;
        locals.var_pds_ini_dn6 = assign12240_e13941_d_n6;
        locals.var_pds_ini_dn7 = assign12240_e13941_d_n7;
        locals.var_pds_ini_dn10 = assign12240_e13941_d_n10;
        locals.var_pds_ini_dn11 = assign12240_e13941_d_n11;
        locals.var_pds_ini_dn12 = assign12240_e13941_d_n12;
        locals.var_pds_ini_dn17 = assign12240_e13941_d_n17;

        let assign12250_e13944: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard319 = assign12250_e13944;

        let (assign12260_e13962, assign12260_e13962_d_n0, assign12260_e13962_d_n2, assign12260_e13962_d_n6, assign12260_e13962_d_n7, assign12260_e13962_d_n10, assign12260_e13962_d_n11, assign12260_e13962_d_n12, assign12260_e13962_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12260_e13962;
        locals.var_pds_ini_dn0 = assign12260_e13962_d_n0;
        locals.var_pds_ini_dn2 = assign12260_e13962_d_n2;
        locals.var_pds_ini_dn6 = assign12260_e13962_d_n6;
        locals.var_pds_ini_dn7 = assign12260_e13962_d_n7;
        locals.var_pds_ini_dn10 = assign12260_e13962_d_n10;
        locals.var_pds_ini_dn11 = assign12260_e13962_d_n11;
        locals.var_pds_ini_dn12 = assign12260_e13962_d_n12;
        locals.var_pds_ini_dn17 = assign12260_e13962_d_n17;

        let (assign12270_e13975, assign12270_e13975_d_n0, assign12270_e13975_d_n2, assign12270_e13975_d_n6, assign12270_e13975_d_n7, assign12270_e13975_d_n10, assign12270_e13975_d_n11, assign12270_e13975_d_n12, assign12270_e13975_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign12270_e13975;
        locals.var_pds_dn0 = assign12270_e13975_d_n0;
        locals.var_pds_dn2 = assign12270_e13975_d_n2;
        locals.var_pds_dn6 = assign12270_e13975_d_n6;
        locals.var_pds_dn7 = assign12270_e13975_d_n7;
        locals.var_pds_dn10 = assign12270_e13975_d_n10;
        locals.var_pds_dn11 = assign12270_e13975_d_n11;
        locals.var_pds_dn12 = assign12270_e13975_d_n12;
        locals.var_pds_dn17 = assign12270_e13975_d_n17;

        let (assign12280_e13990, assign12280_e13990_d_n0, assign12280_e13990_d_n2, assign12280_e13990_d_n6, assign12280_e13990_d_n7, assign12280_e13990_d_n10, assign12280_e13990_d_n11, assign12280_e13990_d_n12, assign12280_e13990_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign12280_e13988: f64 = (locals.var_phi_s0_soi + locals.var_pds);
        (assign12280_e13988, (locals.var_phi_s0_soi_dn0 + locals.var_pds_dn0), (locals.var_phi_s0_soi_dn2 + locals.var_pds_dn2), (locals.var_phi_s0_soi_dn6 + locals.var_pds_dn6), (locals.var_phi_s0_soi_dn7 + locals.var_pds_dn7), (locals.var_phi_s0_soi_dn10 + locals.var_pds_dn10), (locals.var_phi_s0_soi_dn11 + locals.var_pds_dn11), (locals.var_phi_s0_soi_dn12 + locals.var_pds_dn12), (locals.var_phi_s0_soi_dn17 + locals.var_pds_dn17),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12280_e13990;
        locals.var_psl_dn0 = assign12280_e13990_d_n0;
        locals.var_psl_dn2 = assign12280_e13990_d_n2;
        locals.var_psl_dn6 = assign12280_e13990_d_n6;
        locals.var_psl_dn7 = assign12280_e13990_d_n7;
        locals.var_psl_dn10 = assign12280_e13990_d_n10;
        locals.var_psl_dn11 = assign12280_e13990_d_n11;
        locals.var_psl_dn12 = assign12280_e13990_d_n12;
        locals.var_psl_dn17 = assign12280_e13990_d_n17;

        let (assign12290_e14003, assign12290_e14003_d_n0, assign12290_e14003_d_n2, assign12290_e14003_d_n6, assign12290_e14003_d_n7, assign12290_e14003_d_n10, assign12290_e14003_d_n11, assign12290_e14003_d_n12, assign12290_e14003_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12290_e14003;
        locals.var_phi_sl_soi_dn0 = assign12290_e14003_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12290_e14003_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12290_e14003_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12290_e14003_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12290_e14003_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12290_e14003_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12290_e14003_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12290_e14003_d_n17;

        let (assign12300_e14016, assign12300_e14016_d_n0, assign12300_e14016_d_n2, assign12300_e14016_d_n6, assign12300_e14016_d_n7, assign12300_e14016_d_n10, assign12300_e14016_d_n11, assign12300_e14016_d_n12, assign12300_e14016_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        (locals.var_phi_b_dep0, locals.var_phi_b_dep0_dn0, locals.var_phi_b_dep0_dn2, locals.var_phi_b_dep0_dn6, locals.var_phi_b_dep0_dn7, locals.var_phi_b_dep0_dn10, locals.var_phi_b_dep0_dn11, locals.var_phi_b_dep0_dn12, locals.var_phi_b_dep0_dn17,)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
        locals.var_phi_b_dep = assign12300_e14016;
        locals.var_phi_b_dep_dn0 = assign12300_e14016_d_n0;
        locals.var_phi_b_dep_dn2 = assign12300_e14016_d_n2;
        locals.var_phi_b_dep_dn6 = assign12300_e14016_d_n6;
        locals.var_phi_b_dep_dn7 = assign12300_e14016_d_n7;
        locals.var_phi_b_dep_dn10 = assign12300_e14016_d_n10;
        locals.var_phi_b_dep_dn11 = assign12300_e14016_d_n11;
        locals.var_phi_b_dep_dn12 = assign12300_e14016_d_n12;
        locals.var_phi_b_dep_dn17 = assign12300_e14016_d_n17;

        let (assign12310_e14035, assign12310_e14035_d_n10,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign12310_e14029: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign12310_e14031: f64 = (assign12310_e14029 * locals.var_c_box_fd_inv);
        let assign12310_e14033: f64 = (assign12310_e14031 * locals.var_c_box_fd_inv);
        (assign12310_e14033, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv),)
    } else {
        (locals.var_t0__blk320, locals.var_t0__blk320_dn10,)
    }
};
        locals.var_t0__blk320 = assign12310_e14035;
        locals.var_t0__blk320_dn10 = assign12310_e14035_d_n10;

        let assign12320_e14038: f64 = if locals.var_phi_sl_soi < locals.var_fd_end { 1.0 } else { 0.0 };
        locals.var_guard326 = assign12320_e14038;

        let (assign12330_e14054, assign12330_e14054_d_n0, assign12330_e14054_d_n2, assign12330_e14054_d_n6, assign12330_e14054_d_n7, assign12330_e14054_d_n10, assign12330_e14054_d_n11, assign12330_e14054_d_n12, assign12330_e14054_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign12330_e14052: f64 = (-locals.var_vbsbiz);
        (assign12330_e14052, (-locals.var_vbsbiz_dn0), (-locals.var_vbsbiz_dn2), (-locals.var_vbsbiz_dn6), (-locals.var_vbsbiz_dn7), (-locals.var_vbsbiz_dn10), (-locals.var_vbsbiz_dn11), (-locals.var_vbsbiz_dn12), (-locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk321, locals.var_t1__blk321_dn0, locals.var_t1__blk321_dn2, locals.var_t1__blk321_dn6, locals.var_t1__blk321_dn7, locals.var_t1__blk321_dn10, locals.var_t1__blk321_dn11, locals.var_t1__blk321_dn12, locals.var_t1__blk321_dn17,)
    }
};
        locals.var_t1__blk321 = assign12330_e14054;
        locals.var_t1__blk321_dn0 = assign12330_e14054_d_n0;
        locals.var_t1__blk321_dn2 = assign12330_e14054_d_n2;
        locals.var_t1__blk321_dn6 = assign12330_e14054_d_n6;
        locals.var_t1__blk321_dn7 = assign12330_e14054_d_n7;
        locals.var_t1__blk321_dn10 = assign12330_e14054_d_n10;
        locals.var_t1__blk321_dn11 = assign12330_e14054_d_n11;
        locals.var_t1__blk321_dn12 = assign12330_e14054_d_n12;
        locals.var_t1__blk321_dn17 = assign12330_e14054_d_n17;

        let (assign12340_e14091, assign12340_e14091_d_n0, assign12340_e14091_d_n2, assign12340_e14091_d_n6, assign12340_e14091_d_n7, assign12340_e14091_d_n10, assign12340_e14091_d_n11, assign12340_e14091_d_n12, assign12340_e14091_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign12340_e14069: f64 = (2.0 * locals.var_t1__blk321);
        let assign12340_e14072: f64 = (locals.var_t0__blk320 * locals.var_beta);
        let assign12340_e14073: f64 = (assign12340_e14069 + assign12340_e14072);
        let assign12340_e14076: f64 = (2.0 * locals.var_t1__blk321);
        let assign12340_e14079: f64 = (locals.var_t0__blk320 * locals.var_beta);
        let assign12340_e14080: f64 = (assign12340_e14076 + assign12340_e14079);
        let assign12340_e14081: f64 = (assign12340_e14073 * assign12340_e14080);
        let assign12340_e14085: f64 = (locals.var_t1__blk321 * locals.var_t1__blk321);
        let assign12340_e14087: f64 = (assign12340_e14085 + locals.var_t0__blk320);
        let assign12340_e14088: f64 = (4.0 * assign12340_e14087);
        let assign12340_e14089: f64 = (assign12340_e14081 - assign12340_e14088);
        (assign12340_e14089, ((((2.0 * locals.var_t1__blk321_dn0) * assign12340_e14080) + (assign12340_e14073 * (2.0 * locals.var_t1__blk321_dn0))) - (4.0 * ((locals.var_t1__blk321_dn0 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn0)))), ((((2.0 * locals.var_t1__blk321_dn2) * assign12340_e14080) + (assign12340_e14073 * (2.0 * locals.var_t1__blk321_dn2))) - (4.0 * ((locals.var_t1__blk321_dn2 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn2)))), ((((2.0 * locals.var_t1__blk321_dn6) * assign12340_e14080) + (assign12340_e14073 * (2.0 * locals.var_t1__blk321_dn6))) - (4.0 * ((locals.var_t1__blk321_dn6 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn6)))), ((((2.0 * locals.var_t1__blk321_dn7) * assign12340_e14080) + (assign12340_e14073 * (2.0 * locals.var_t1__blk321_dn7))) - (4.0 * ((locals.var_t1__blk321_dn7 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn7)))), (((((2.0 * locals.var_t1__blk321_dn10) + ((locals.var_t0__blk320_dn10 * locals.var_beta) + (locals.var_t0__blk320 * locals.var_beta_dn10))) * assign12340_e14080) + (assign12340_e14073 * ((2.0 * locals.var_t1__blk321_dn10) + ((locals.var_t0__blk320_dn10 * locals.var_beta) + (locals.var_t0__blk320 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk321_dn10 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn10)) + locals.var_t0__blk320_dn10))), ((((2.0 * locals.var_t1__blk321_dn11) * assign12340_e14080) + (assign12340_e14073 * (2.0 * locals.var_t1__blk321_dn11))) - (4.0 * ((locals.var_t1__blk321_dn11 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn11)))), ((((2.0 * locals.var_t1__blk321_dn12) * assign12340_e14080) + (assign12340_e14073 * (2.0 * locals.var_t1__blk321_dn12))) - (4.0 * ((locals.var_t1__blk321_dn12 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn12)))), ((((2.0 * locals.var_t1__blk321_dn17) * assign12340_e14080) + (assign12340_e14073 * (2.0 * locals.var_t1__blk321_dn17))) - (4.0 * ((locals.var_t1__blk321_dn17 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn17)))),)
    } else {
        (locals.var_t2__blk322, locals.var_t2__blk322_dn0, locals.var_t2__blk322_dn2, locals.var_t2__blk322_dn6, locals.var_t2__blk322_dn7, locals.var_t2__blk322_dn10, locals.var_t2__blk322_dn11, locals.var_t2__blk322_dn12, locals.var_t2__blk322_dn17,)
    }
};
        locals.var_t2__blk322 = assign12340_e14091;
        locals.var_t2__blk322_dn0 = assign12340_e14091_d_n0;
        locals.var_t2__blk322_dn2 = assign12340_e14091_d_n2;
        locals.var_t2__blk322_dn6 = assign12340_e14091_d_n6;
        locals.var_t2__blk322_dn7 = assign12340_e14091_d_n7;
        locals.var_t2__blk322_dn10 = assign12340_e14091_d_n10;
        locals.var_t2__blk322_dn11 = assign12340_e14091_d_n11;
        locals.var_t2__blk322_dn12 = assign12340_e14091_d_n12;
        locals.var_t2__blk322_dn17 = assign12340_e14091_d_n17;

        let (assign12350_e14115, assign12350_e14115_d_n0, assign12350_e14115_d_n2, assign12350_e14115_d_n6, assign12350_e14115_d_n7, assign12350_e14115_d_n10, assign12350_e14115_d_n11, assign12350_e14115_d_n12, assign12350_e14115_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign12350_e14107: f64 = (10.0 * 2.220446049250313e-16);
        let (assign12350_e14113, assign12350_e14113_d_n0, assign12350_e14113_d_n2, assign12350_e14113_d_n6, assign12350_e14113_d_n7, assign12350_e14113_d_n10, assign12350_e14113_d_n11, assign12350_e14113_d_n12, assign12350_e14113_d_n17,) = {
            if (locals.var_t2__blk322 >= assign12350_e14107) {
                (locals.var_t2__blk322, locals.var_t2__blk322_dn0, locals.var_t2__blk322_dn2, locals.var_t2__blk322_dn6, locals.var_t2__blk322_dn7, locals.var_t2__blk322_dn10, locals.var_t2__blk322_dn11, locals.var_t2__blk322_dn12, locals.var_t2__blk322_dn17,)
            } else {
                let assign12350_e14112: f64 = (10.0 * 2.220446049250313e-16);
                (assign12350_e14112, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12350_e14113, assign12350_e14113_d_n0, assign12350_e14113_d_n2, assign12350_e14113_d_n6, assign12350_e14113_d_n7, assign12350_e14113_d_n10, assign12350_e14113_d_n11, assign12350_e14113_d_n12, assign12350_e14113_d_n17,)
    } else {
        (locals.var_t2__blk322, locals.var_t2__blk322_dn0, locals.var_t2__blk322_dn2, locals.var_t2__blk322_dn6, locals.var_t2__blk322_dn7, locals.var_t2__blk322_dn10, locals.var_t2__blk322_dn11, locals.var_t2__blk322_dn12, locals.var_t2__blk322_dn17,)
    }
};
        locals.var_t2__blk322 = assign12350_e14115;
        locals.var_t2__blk322_dn0 = assign12350_e14115_d_n0;
        locals.var_t2__blk322_dn2 = assign12350_e14115_d_n2;
        locals.var_t2__blk322_dn6 = assign12350_e14115_d_n6;
        locals.var_t2__blk322_dn7 = assign12350_e14115_d_n7;
        locals.var_t2__blk322_dn10 = assign12350_e14115_d_n10;
        locals.var_t2__blk322_dn11 = assign12350_e14115_d_n11;
        locals.var_t2__blk322_dn12 = assign12350_e14115_d_n12;
        locals.var_t2__blk322_dn17 = assign12350_e14115_d_n17;

        let (assign12360_e14131, assign12360_e14131_d_n0, assign12360_e14131_d_n2, assign12360_e14131_d_n6, assign12360_e14131_d_n7, assign12360_e14131_d_n10, assign12360_e14131_d_n11, assign12360_e14131_d_n12, assign12360_e14131_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign12360_e14129: f64 = (locals.var_t2__blk322).sqrt();
        (assign12360_e14129, (locals.var_t2__blk322_dn0 / (2.0 * assign12360_e14129)), (locals.var_t2__blk322_dn2 / (2.0 * assign12360_e14129)), (locals.var_t2__blk322_dn6 / (2.0 * assign12360_e14129)), (locals.var_t2__blk322_dn7 / (2.0 * assign12360_e14129)), (locals.var_t2__blk322_dn10 / (2.0 * assign12360_e14129)), (locals.var_t2__blk322_dn11 / (2.0 * assign12360_e14129)), (locals.var_t2__blk322_dn12 / (2.0 * assign12360_e14129)), (locals.var_t2__blk322_dn17 / (2.0 * assign12360_e14129)),)
    } else {
        (locals.var_t2__blk322, locals.var_t2__blk322_dn0, locals.var_t2__blk322_dn2, locals.var_t2__blk322_dn6, locals.var_t2__blk322_dn7, locals.var_t2__blk322_dn10, locals.var_t2__blk322_dn11, locals.var_t2__blk322_dn12, locals.var_t2__blk322_dn17,)
    }
};
        locals.var_t2__blk322 = assign12360_e14131;
        locals.var_t2__blk322_dn0 = assign12360_e14131_d_n0;
        locals.var_t2__blk322_dn2 = assign12360_e14131_d_n2;
        locals.var_t2__blk322_dn6 = assign12360_e14131_d_n6;
        locals.var_t2__blk322_dn7 = assign12360_e14131_d_n7;
        locals.var_t2__blk322_dn10 = assign12360_e14131_d_n10;
        locals.var_t2__blk322_dn11 = assign12360_e14131_d_n11;
        locals.var_t2__blk322_dn12 = assign12360_e14131_d_n12;
        locals.var_t2__blk322_dn17 = assign12360_e14131_d_n17;

        let (assign12370_e14152, assign12370_e14152_d_n0, assign12370_e14152_d_n2, assign12370_e14152_d_n6, assign12370_e14152_d_n7, assign12370_e14152_d_n10, assign12370_e14152_d_n11, assign12370_e14152_d_n12, assign12370_e14152_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign12370_e14146: f64 = (2.0 * locals.var_t1__blk321);
        let assign12370_e14149: f64 = (locals.var_t0__blk320 * locals.var_beta);
        let assign12370_e14150: f64 = (assign12370_e14146 + assign12370_e14149);
        (assign12370_e14150, (2.0 * locals.var_t1__blk321_dn0), (2.0 * locals.var_t1__blk321_dn2), (2.0 * locals.var_t1__blk321_dn6), (2.0 * locals.var_t1__blk321_dn7), ((2.0 * locals.var_t1__blk321_dn10) + ((locals.var_t0__blk320_dn10 * locals.var_beta) + (locals.var_t0__blk320 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk321_dn11), (2.0 * locals.var_t1__blk321_dn12), (2.0 * locals.var_t1__blk321_dn17),)
    } else {
        (locals.var_t3__blk323, locals.var_t3__blk323_dn0, locals.var_t3__blk323_dn2, locals.var_t3__blk323_dn6, locals.var_t3__blk323_dn7, locals.var_t3__blk323_dn10, locals.var_t3__blk323_dn11, locals.var_t3__blk323_dn12, locals.var_t3__blk323_dn17,)
    }
};
        locals.var_t3__blk323 = assign12370_e14152;
        locals.var_t3__blk323_dn0 = assign12370_e14152_d_n0;
        locals.var_t3__blk323_dn2 = assign12370_e14152_d_n2;
        locals.var_t3__blk323_dn6 = assign12370_e14152_d_n6;
        locals.var_t3__blk323_dn7 = assign12370_e14152_d_n7;
        locals.var_t3__blk323_dn10 = assign12370_e14152_d_n10;
        locals.var_t3__blk323_dn11 = assign12370_e14152_d_n11;
        locals.var_t3__blk323_dn12 = assign12370_e14152_d_n12;
        locals.var_t3__blk323_dn17 = assign12370_e14152_d_n17;

        let (assign12380_e14171, assign12380_e14171_d_n0, assign12380_e14171_d_n2, assign12380_e14171_d_n6, assign12380_e14171_d_n7, assign12380_e14171_d_n10, assign12380_e14171_d_n11, assign12380_e14171_d_n12, assign12380_e14171_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign12380_e14167: f64 = (locals.var_t3__blk323 - locals.var_t2__blk322);
        let assign12380_e14169: f64 = (assign12380_e14167 / 2.0);
        (assign12380_e14169, ((locals.var_t3__blk323_dn0 - locals.var_t2__blk322_dn0) / 2.0), ((locals.var_t3__blk323_dn2 - locals.var_t2__blk322_dn2) / 2.0), ((locals.var_t3__blk323_dn6 - locals.var_t2__blk322_dn6) / 2.0), ((locals.var_t3__blk323_dn7 - locals.var_t2__blk322_dn7) / 2.0), ((locals.var_t3__blk323_dn10 - locals.var_t2__blk322_dn10) / 2.0), ((locals.var_t3__blk323_dn11 - locals.var_t2__blk322_dn11) / 2.0), ((locals.var_t3__blk323_dn12 - locals.var_t2__blk322_dn12) / 2.0), ((locals.var_t3__blk323_dn17 - locals.var_t2__blk322_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk324, locals.var_psb_inia__blk324_dn0, locals.var_psb_inia__blk324_dn2, locals.var_psb_inia__blk324_dn6, locals.var_psb_inia__blk324_dn7, locals.var_psb_inia__blk324_dn10, locals.var_psb_inia__blk324_dn11, locals.var_psb_inia__blk324_dn12, locals.var_psb_inia__blk324_dn17,)
    }
};
        locals.var_psb_inia__blk324 = assign12380_e14171;
        locals.var_psb_inia__blk324_dn0 = assign12380_e14171_d_n0;
        locals.var_psb_inia__blk324_dn2 = assign12380_e14171_d_n2;
        locals.var_psb_inia__blk324_dn6 = assign12380_e14171_d_n6;
        locals.var_psb_inia__blk324_dn7 = assign12380_e14171_d_n7;
        locals.var_psb_inia__blk324_dn10 = assign12380_e14171_d_n10;
        locals.var_psb_inia__blk324_dn11 = assign12380_e14171_d_n11;
        locals.var_psb_inia__blk324_dn12 = assign12380_e14171_d_n12;
        locals.var_psb_inia__blk324_dn17 = assign12380_e14171_d_n17;

        let (assign12390_e14199, assign12390_e14199_d_n0, assign12390_e14199_d_n2, assign12390_e14199_d_n6, assign12390_e14199_d_n7, assign12390_e14199_d_n10, assign12390_e14199_d_n11, assign12390_e14199_d_n12, assign12390_e14199_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign12390_e14186: f64 = (locals.var_t1__blk321 * locals.var_t1__blk321);
        let assign12390_e14188: f64 = (assign12390_e14186 / locals.var_t0__blk320);
        let assign12390_e14190: f64 = (assign12390_e14188 / locals.var_cnst1bulk);
        let assign12390_e14191: f64 = (assign12390_e14190).ln();
        let assign12390_e14195: f64 = (2.0 / locals.var_t1__blk321);
        let assign12390_e14196: f64 = (locals.var_beta + assign12390_e14195);
        let assign12390_e14197: f64 = (assign12390_e14191 / assign12390_e14196);
        (assign12390_e14197, ((((((((((locals.var_t1__blk321_dn0 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn0)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12390_e14188 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12390_e14190) * assign12390_e14196) - (assign12390_e14191 * (-((2.0 * locals.var_t1__blk321_dn0) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12390_e14196 * assign12390_e14196)), ((((((((((locals.var_t1__blk321_dn2 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn2)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12390_e14188 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12390_e14190) * assign12390_e14196) - (assign12390_e14191 * (-((2.0 * locals.var_t1__blk321_dn2) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12390_e14196 * assign12390_e14196)), ((((((((((locals.var_t1__blk321_dn6 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn6)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12390_e14188 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12390_e14190) * assign12390_e14196) - (assign12390_e14191 * (-((2.0 * locals.var_t1__blk321_dn6) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12390_e14196 * assign12390_e14196)), ((((((((((locals.var_t1__blk321_dn7 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn7)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12390_e14188 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12390_e14190) * assign12390_e14196) - (assign12390_e14191 * (-((2.0 * locals.var_t1__blk321_dn7) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12390_e14196 * assign12390_e14196)), ((((((((((((locals.var_t1__blk321_dn10 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn10)) * locals.var_t0__blk320) - (assign12390_e14186 * locals.var_t0__blk320_dn10)) / (locals.var_t0__blk320 * locals.var_t0__blk320)) * locals.var_cnst1bulk) - (assign12390_e14188 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12390_e14190) * assign12390_e14196) - (assign12390_e14191 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk321_dn10) / (locals.var_t1__blk321 * locals.var_t1__blk321)))))) / (assign12390_e14196 * assign12390_e14196)), ((((((((((locals.var_t1__blk321_dn11 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn11)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12390_e14188 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12390_e14190) * assign12390_e14196) - (assign12390_e14191 * (-((2.0 * locals.var_t1__blk321_dn11) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12390_e14196 * assign12390_e14196)), ((((((((((locals.var_t1__blk321_dn12 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn12)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12390_e14188 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12390_e14190) * assign12390_e14196) - (assign12390_e14191 * (-((2.0 * locals.var_t1__blk321_dn12) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12390_e14196 * assign12390_e14196)), ((((((((((locals.var_t1__blk321_dn17 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn17)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12390_e14188 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12390_e14190) * assign12390_e14196) - (assign12390_e14191 * (-((2.0 * locals.var_t1__blk321_dn17) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12390_e14196 * assign12390_e14196)),)
    } else {
        (locals.var_psb_inib__blk325, locals.var_psb_inib__blk325_dn0, locals.var_psb_inib__blk325_dn2, locals.var_psb_inib__blk325_dn6, locals.var_psb_inib__blk325_dn7, locals.var_psb_inib__blk325_dn10, locals.var_psb_inib__blk325_dn11, locals.var_psb_inib__blk325_dn12, locals.var_psb_inib__blk325_dn17,)
    }
};
        locals.var_psb_inib__blk325 = assign12390_e14199;
        locals.var_psb_inib__blk325_dn0 = assign12390_e14199_d_n0;
        locals.var_psb_inib__blk325_dn2 = assign12390_e14199_d_n2;
        locals.var_psb_inib__blk325_dn6 = assign12390_e14199_d_n6;
        locals.var_psb_inib__blk325_dn7 = assign12390_e14199_d_n7;
        locals.var_psb_inib__blk325_dn10 = assign12390_e14199_d_n10;
        locals.var_psb_inib__blk325_dn11 = assign12390_e14199_d_n11;
        locals.var_psb_inib__blk325_dn12 = assign12390_e14199_d_n12;
        locals.var_psb_inib__blk325_dn17 = assign12390_e14199_d_n17;

        let assign12400_e14202: f64 = if locals.var_psb_inia__blk324 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard327 = assign12400_e14202;

        let (assign12410_e14219, assign12410_e14219_d_n0, assign12410_e14219_d_n2, assign12410_e14219_d_n6, assign12410_e14219_d_n7, assign12410_e14219_d_n10, assign12410_e14219_d_n11, assign12410_e14219_d_n12, assign12410_e14219_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) && (locals.var_guard327 != 0.0)) {
        (locals.var_psb_inia__blk324, locals.var_psb_inia__blk324_dn0, locals.var_psb_inia__blk324_dn2, locals.var_psb_inia__blk324_dn6, locals.var_psb_inia__blk324_dn7, locals.var_psb_inia__blk324_dn10, locals.var_psb_inia__blk324_dn11, locals.var_psb_inia__blk324_dn12, locals.var_psb_inia__blk324_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12410_e14219;
        locals.var_phi_sl_bulk_dn0 = assign12410_e14219_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12410_e14219_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12410_e14219_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12410_e14219_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12410_e14219_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12410_e14219_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12410_e14219_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12410_e14219_d_n17;

        let (assign12420_e14241, assign12420_e14241_d_n0, assign12420_e14241_d_n2, assign12420_e14241_d_n6, assign12420_e14241_d_n7, assign12420_e14241_d_n10, assign12420_e14241_d_n11, assign12420_e14241_d_n12, assign12420_e14241_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) && (locals.var_guard327 == 0.0)) {
        let assign12420_e14237: f64 = (locals.var_psb_inib__blk325 - locals.var_psb_inia__blk324);
        let assign12420_e14239: f64 = (assign12420_e14237 - 0.0008);
        (assign12420_e14239, (locals.var_psb_inib__blk325_dn0 - locals.var_psb_inia__blk324_dn0), (locals.var_psb_inib__blk325_dn2 - locals.var_psb_inia__blk324_dn2), (locals.var_psb_inib__blk325_dn6 - locals.var_psb_inia__blk324_dn6), (locals.var_psb_inib__blk325_dn7 - locals.var_psb_inia__blk324_dn7), (locals.var_psb_inib__blk325_dn10 - locals.var_psb_inia__blk324_dn10), (locals.var_psb_inib__blk325_dn11 - locals.var_psb_inia__blk324_dn11), (locals.var_psb_inib__blk325_dn12 - locals.var_psb_inia__blk324_dn12), (locals.var_psb_inib__blk325_dn17 - locals.var_psb_inia__blk324_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12420_e14241;
        locals.var_tmf1_dn0 = assign12420_e14241_d_n0;
        locals.var_tmf1_dn2 = assign12420_e14241_d_n2;
        locals.var_tmf1_dn6 = assign12420_e14241_d_n6;
        locals.var_tmf1_dn7 = assign12420_e14241_d_n7;
        locals.var_tmf1_dn10 = assign12420_e14241_d_n10;
        locals.var_tmf1_dn11 = assign12420_e14241_d_n11;
        locals.var_tmf1_dn12 = assign12420_e14241_d_n12;
        locals.var_tmf1_dn17 = assign12420_e14241_d_n17;

        let (assign12430_e14263, assign12430_e14263_d_n0, assign12430_e14263_d_n2, assign12430_e14263_d_n6, assign12430_e14263_d_n7, assign12430_e14263_d_n10, assign12430_e14263_d_n11, assign12430_e14263_d_n12, assign12430_e14263_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) && (locals.var_guard327 == 0.0)) {
        let assign12430_e14259: f64 = (4.0 * locals.var_psb_inib__blk325);
        let assign12430_e14261: f64 = (assign12430_e14259 * 0.0008);
        (assign12430_e14261, ((4.0 * locals.var_psb_inib__blk325_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12430_e14263;
        locals.var_tmf2_dn0 = assign12430_e14263_d_n0;
        locals.var_tmf2_dn2 = assign12430_e14263_d_n2;
        locals.var_tmf2_dn6 = assign12430_e14263_d_n6;
        locals.var_tmf2_dn7 = assign12430_e14263_d_n7;
        locals.var_tmf2_dn10 = assign12430_e14263_d_n10;
        locals.var_tmf2_dn11 = assign12430_e14263_d_n11;
        locals.var_tmf2_dn12 = assign12430_e14263_d_n12;
        locals.var_tmf2_dn17 = assign12430_e14263_d_n17;

    }

    pub(super) fn stamp_transient_block_36(
        locals: &mut StampLocals,
    ) {
        let (assign12440_e14287, assign12440_e14287_d_n0, assign12440_e14287_d_n2, assign12440_e14287_d_n6, assign12440_e14287_d_n7, assign12440_e14287_d_n10, assign12440_e14287_d_n11, assign12440_e14287_d_n12, assign12440_e14287_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) && (locals.var_guard327 == 0.0)) {
        let (assign12440_e14285, assign12440_e14285_d_n0, assign12440_e14285_d_n2, assign12440_e14285_d_n6, assign12440_e14285_d_n7, assign12440_e14285_d_n10, assign12440_e14285_d_n11, assign12440_e14285_d_n12, assign12440_e14285_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12440_e14284: f64 = (-locals.var_tmf2);
                (assign12440_e14284, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12440_e14285, assign12440_e14285_d_n0, assign12440_e14285_d_n2, assign12440_e14285_d_n6, assign12440_e14285_d_n7, assign12440_e14285_d_n10, assign12440_e14285_d_n11, assign12440_e14285_d_n12, assign12440_e14285_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12440_e14287;
        locals.var_tmf2_dn0 = assign12440_e14287_d_n0;
        locals.var_tmf2_dn2 = assign12440_e14287_d_n2;
        locals.var_tmf2_dn6 = assign12440_e14287_d_n6;
        locals.var_tmf2_dn7 = assign12440_e14287_d_n7;
        locals.var_tmf2_dn10 = assign12440_e14287_d_n10;
        locals.var_tmf2_dn11 = assign12440_e14287_d_n11;
        locals.var_tmf2_dn12 = assign12440_e14287_d_n12;
        locals.var_tmf2_dn17 = assign12440_e14287_d_n17;

        let (assign12450_e14310, assign12450_e14310_d_n0, assign12450_e14310_d_n2, assign12450_e14310_d_n6, assign12450_e14310_d_n7, assign12450_e14310_d_n10, assign12450_e14310_d_n11, assign12450_e14310_d_n12, assign12450_e14310_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) && (locals.var_guard327 == 0.0)) {
        let assign12450_e14305: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12450_e14307: f64 = (assign12450_e14305 + locals.var_tmf2);
        let assign12450_e14308: f64 = (assign12450_e14307).sqrt();
        (assign12450_e14308, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12450_e14308)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12450_e14308)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12450_e14308)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12450_e14308)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12450_e14308)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12450_e14308)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12450_e14308)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12450_e14308)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12450_e14310;
        locals.var_tmf2_dn0 = assign12450_e14310_d_n0;
        locals.var_tmf2_dn2 = assign12450_e14310_d_n2;
        locals.var_tmf2_dn6 = assign12450_e14310_d_n6;
        locals.var_tmf2_dn7 = assign12450_e14310_d_n7;
        locals.var_tmf2_dn10 = assign12450_e14310_d_n10;
        locals.var_tmf2_dn11 = assign12450_e14310_d_n11;
        locals.var_tmf2_dn12 = assign12450_e14310_d_n12;
        locals.var_tmf2_dn17 = assign12450_e14310_d_n17;

        let (assign12460_e14334, assign12460_e14334_d_n0, assign12460_e14334_d_n2, assign12460_e14334_d_n6, assign12460_e14334_d_n7, assign12460_e14334_d_n10, assign12460_e14334_d_n11, assign12460_e14334_d_n12, assign12460_e14334_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 != 0.0)) && (locals.var_guard327 == 0.0)) {
        let assign12460_e14330: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12460_e14331: f64 = (0.5 * assign12460_e14330);
        let assign12460_e14332: f64 = (locals.var_psb_inib__blk325 - assign12460_e14331);
        (assign12460_e14332, (locals.var_psb_inib__blk325_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk325_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk325_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk325_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk325_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk325_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk325_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk325_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12460_e14334;
        locals.var_phi_sl_bulk_dn0 = assign12460_e14334_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12460_e14334_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12460_e14334_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12460_e14334_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12460_e14334_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12460_e14334_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12460_e14334_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12460_e14334_d_n17;

        let (assign12470_e14361, assign12470_e14361_d_n0, assign12470_e14361_d_n2, assign12470_e14361_d_n6, assign12470_e14361_d_n7, assign12470_e14361_d_n10, assign12470_e14361_d_n11, assign12470_e14361_d_n12, assign12470_e14361_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign12470_e14350: f64 = (locals.var_vbsbiz - locals.var_phi_sl_soi);
        let assign12470_e14353: f64 = (locals.var_q_fd_soi / 2.0);
        let assign12470_e14355: f64 = (assign12470_e14353 * locals.var_t_soi);
        let assign12470_e14357: f64 = (assign12470_e14355 / 1.034943e-10);
        let assign12470_e14358: f64 = (assign12470_e14350 - assign12470_e14357);
        let assign12470_e14359: f64 = (-assign12470_e14358);
        (assign12470_e14359, (-((locals.var_vbsbiz_dn0 - locals.var_phi_sl_soi_dn0) - (((locals.var_q_fd_soi_dn0 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn2 - locals.var_phi_sl_soi_dn2) - (((locals.var_q_fd_soi_dn2 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn6 - locals.var_phi_sl_soi_dn6) - (((locals.var_q_fd_soi_dn6 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn7 - locals.var_phi_sl_soi_dn7) - (((locals.var_q_fd_soi_dn7 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn10 - locals.var_phi_sl_soi_dn10) - (((locals.var_q_fd_soi_dn10 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn11 - locals.var_phi_sl_soi_dn11) - (((locals.var_q_fd_soi_dn11 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn12 - locals.var_phi_sl_soi_dn12) - (((locals.var_q_fd_soi_dn12 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn17 - locals.var_phi_sl_soi_dn17) - (((locals.var_q_fd_soi_dn17 / 2.0) * locals.var_t_soi) / 1.034943e-10))),)
    } else {
        (locals.var_t1__blk321, locals.var_t1__blk321_dn0, locals.var_t1__blk321_dn2, locals.var_t1__blk321_dn6, locals.var_t1__blk321_dn7, locals.var_t1__blk321_dn10, locals.var_t1__blk321_dn11, locals.var_t1__blk321_dn12, locals.var_t1__blk321_dn17,)
    }
};
        locals.var_t1__blk321 = assign12470_e14361;
        locals.var_t1__blk321_dn0 = assign12470_e14361_d_n0;
        locals.var_t1__blk321_dn2 = assign12470_e14361_d_n2;
        locals.var_t1__blk321_dn6 = assign12470_e14361_d_n6;
        locals.var_t1__blk321_dn7 = assign12470_e14361_d_n7;
        locals.var_t1__blk321_dn10 = assign12470_e14361_d_n10;
        locals.var_t1__blk321_dn11 = assign12470_e14361_d_n11;
        locals.var_t1__blk321_dn12 = assign12470_e14361_d_n12;
        locals.var_t1__blk321_dn17 = assign12470_e14361_d_n17;

        let (assign12480_e14399, assign12480_e14399_d_n0, assign12480_e14399_d_n2, assign12480_e14399_d_n6, assign12480_e14399_d_n7, assign12480_e14399_d_n10, assign12480_e14399_d_n11, assign12480_e14399_d_n12, assign12480_e14399_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign12480_e14377: f64 = (2.0 * locals.var_t1__blk321);
        let assign12480_e14380: f64 = (locals.var_t0__blk320 * locals.var_beta);
        let assign12480_e14381: f64 = (assign12480_e14377 + assign12480_e14380);
        let assign12480_e14384: f64 = (2.0 * locals.var_t1__blk321);
        let assign12480_e14387: f64 = (locals.var_t0__blk320 * locals.var_beta);
        let assign12480_e14388: f64 = (assign12480_e14384 + assign12480_e14387);
        let assign12480_e14389: f64 = (assign12480_e14381 * assign12480_e14388);
        let assign12480_e14393: f64 = (locals.var_t1__blk321 * locals.var_t1__blk321);
        let assign12480_e14395: f64 = (assign12480_e14393 + locals.var_t0__blk320);
        let assign12480_e14396: f64 = (4.0 * assign12480_e14395);
        let assign12480_e14397: f64 = (assign12480_e14389 - assign12480_e14396);
        (assign12480_e14397, ((((2.0 * locals.var_t1__blk321_dn0) * assign12480_e14388) + (assign12480_e14381 * (2.0 * locals.var_t1__blk321_dn0))) - (4.0 * ((locals.var_t1__blk321_dn0 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn0)))), ((((2.0 * locals.var_t1__blk321_dn2) * assign12480_e14388) + (assign12480_e14381 * (2.0 * locals.var_t1__blk321_dn2))) - (4.0 * ((locals.var_t1__blk321_dn2 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn2)))), ((((2.0 * locals.var_t1__blk321_dn6) * assign12480_e14388) + (assign12480_e14381 * (2.0 * locals.var_t1__blk321_dn6))) - (4.0 * ((locals.var_t1__blk321_dn6 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn6)))), ((((2.0 * locals.var_t1__blk321_dn7) * assign12480_e14388) + (assign12480_e14381 * (2.0 * locals.var_t1__blk321_dn7))) - (4.0 * ((locals.var_t1__blk321_dn7 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn7)))), (((((2.0 * locals.var_t1__blk321_dn10) + ((locals.var_t0__blk320_dn10 * locals.var_beta) + (locals.var_t0__blk320 * locals.var_beta_dn10))) * assign12480_e14388) + (assign12480_e14381 * ((2.0 * locals.var_t1__blk321_dn10) + ((locals.var_t0__blk320_dn10 * locals.var_beta) + (locals.var_t0__blk320 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk321_dn10 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn10)) + locals.var_t0__blk320_dn10))), ((((2.0 * locals.var_t1__blk321_dn11) * assign12480_e14388) + (assign12480_e14381 * (2.0 * locals.var_t1__blk321_dn11))) - (4.0 * ((locals.var_t1__blk321_dn11 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn11)))), ((((2.0 * locals.var_t1__blk321_dn12) * assign12480_e14388) + (assign12480_e14381 * (2.0 * locals.var_t1__blk321_dn12))) - (4.0 * ((locals.var_t1__blk321_dn12 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn12)))), ((((2.0 * locals.var_t1__blk321_dn17) * assign12480_e14388) + (assign12480_e14381 * (2.0 * locals.var_t1__blk321_dn17))) - (4.0 * ((locals.var_t1__blk321_dn17 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn17)))),)
    } else {
        (locals.var_t2__blk322, locals.var_t2__blk322_dn0, locals.var_t2__blk322_dn2, locals.var_t2__blk322_dn6, locals.var_t2__blk322_dn7, locals.var_t2__blk322_dn10, locals.var_t2__blk322_dn11, locals.var_t2__blk322_dn12, locals.var_t2__blk322_dn17,)
    }
};
        locals.var_t2__blk322 = assign12480_e14399;
        locals.var_t2__blk322_dn0 = assign12480_e14399_d_n0;
        locals.var_t2__blk322_dn2 = assign12480_e14399_d_n2;
        locals.var_t2__blk322_dn6 = assign12480_e14399_d_n6;
        locals.var_t2__blk322_dn7 = assign12480_e14399_d_n7;
        locals.var_t2__blk322_dn10 = assign12480_e14399_d_n10;
        locals.var_t2__blk322_dn11 = assign12480_e14399_d_n11;
        locals.var_t2__blk322_dn12 = assign12480_e14399_d_n12;
        locals.var_t2__blk322_dn17 = assign12480_e14399_d_n17;

        let (assign12490_e14424, assign12490_e14424_d_n0, assign12490_e14424_d_n2, assign12490_e14424_d_n6, assign12490_e14424_d_n7, assign12490_e14424_d_n10, assign12490_e14424_d_n11, assign12490_e14424_d_n12, assign12490_e14424_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign12490_e14416: f64 = (10.0 * 2.220446049250313e-16);
        let (assign12490_e14422, assign12490_e14422_d_n0, assign12490_e14422_d_n2, assign12490_e14422_d_n6, assign12490_e14422_d_n7, assign12490_e14422_d_n10, assign12490_e14422_d_n11, assign12490_e14422_d_n12, assign12490_e14422_d_n17,) = {
            if (locals.var_t2__blk322 >= assign12490_e14416) {
                (locals.var_t2__blk322, locals.var_t2__blk322_dn0, locals.var_t2__blk322_dn2, locals.var_t2__blk322_dn6, locals.var_t2__blk322_dn7, locals.var_t2__blk322_dn10, locals.var_t2__blk322_dn11, locals.var_t2__blk322_dn12, locals.var_t2__blk322_dn17,)
            } else {
                let assign12490_e14421: f64 = (10.0 * 2.220446049250313e-16);
                (assign12490_e14421, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12490_e14422, assign12490_e14422_d_n0, assign12490_e14422_d_n2, assign12490_e14422_d_n6, assign12490_e14422_d_n7, assign12490_e14422_d_n10, assign12490_e14422_d_n11, assign12490_e14422_d_n12, assign12490_e14422_d_n17,)
    } else {
        (locals.var_t2__blk322, locals.var_t2__blk322_dn0, locals.var_t2__blk322_dn2, locals.var_t2__blk322_dn6, locals.var_t2__blk322_dn7, locals.var_t2__blk322_dn10, locals.var_t2__blk322_dn11, locals.var_t2__blk322_dn12, locals.var_t2__blk322_dn17,)
    }
};
        locals.var_t2__blk322 = assign12490_e14424;
        locals.var_t2__blk322_dn0 = assign12490_e14424_d_n0;
        locals.var_t2__blk322_dn2 = assign12490_e14424_d_n2;
        locals.var_t2__blk322_dn6 = assign12490_e14424_d_n6;
        locals.var_t2__blk322_dn7 = assign12490_e14424_d_n7;
        locals.var_t2__blk322_dn10 = assign12490_e14424_d_n10;
        locals.var_t2__blk322_dn11 = assign12490_e14424_d_n11;
        locals.var_t2__blk322_dn12 = assign12490_e14424_d_n12;
        locals.var_t2__blk322_dn17 = assign12490_e14424_d_n17;

        let (assign12500_e14441, assign12500_e14441_d_n0, assign12500_e14441_d_n2, assign12500_e14441_d_n6, assign12500_e14441_d_n7, assign12500_e14441_d_n10, assign12500_e14441_d_n11, assign12500_e14441_d_n12, assign12500_e14441_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign12500_e14439: f64 = (locals.var_t2__blk322).sqrt();
        (assign12500_e14439, (locals.var_t2__blk322_dn0 / (2.0 * assign12500_e14439)), (locals.var_t2__blk322_dn2 / (2.0 * assign12500_e14439)), (locals.var_t2__blk322_dn6 / (2.0 * assign12500_e14439)), (locals.var_t2__blk322_dn7 / (2.0 * assign12500_e14439)), (locals.var_t2__blk322_dn10 / (2.0 * assign12500_e14439)), (locals.var_t2__blk322_dn11 / (2.0 * assign12500_e14439)), (locals.var_t2__blk322_dn12 / (2.0 * assign12500_e14439)), (locals.var_t2__blk322_dn17 / (2.0 * assign12500_e14439)),)
    } else {
        (locals.var_t2__blk322, locals.var_t2__blk322_dn0, locals.var_t2__blk322_dn2, locals.var_t2__blk322_dn6, locals.var_t2__blk322_dn7, locals.var_t2__blk322_dn10, locals.var_t2__blk322_dn11, locals.var_t2__blk322_dn12, locals.var_t2__blk322_dn17,)
    }
};
        locals.var_t2__blk322 = assign12500_e14441;
        locals.var_t2__blk322_dn0 = assign12500_e14441_d_n0;
        locals.var_t2__blk322_dn2 = assign12500_e14441_d_n2;
        locals.var_t2__blk322_dn6 = assign12500_e14441_d_n6;
        locals.var_t2__blk322_dn7 = assign12500_e14441_d_n7;
        locals.var_t2__blk322_dn10 = assign12500_e14441_d_n10;
        locals.var_t2__blk322_dn11 = assign12500_e14441_d_n11;
        locals.var_t2__blk322_dn12 = assign12500_e14441_d_n12;
        locals.var_t2__blk322_dn17 = assign12500_e14441_d_n17;

        let (assign12510_e14463, assign12510_e14463_d_n0, assign12510_e14463_d_n2, assign12510_e14463_d_n6, assign12510_e14463_d_n7, assign12510_e14463_d_n10, assign12510_e14463_d_n11, assign12510_e14463_d_n12, assign12510_e14463_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign12510_e14457: f64 = (2.0 * locals.var_t1__blk321);
        let assign12510_e14460: f64 = (locals.var_t0__blk320 * locals.var_beta);
        let assign12510_e14461: f64 = (assign12510_e14457 + assign12510_e14460);
        (assign12510_e14461, (2.0 * locals.var_t1__blk321_dn0), (2.0 * locals.var_t1__blk321_dn2), (2.0 * locals.var_t1__blk321_dn6), (2.0 * locals.var_t1__blk321_dn7), ((2.0 * locals.var_t1__blk321_dn10) + ((locals.var_t0__blk320_dn10 * locals.var_beta) + (locals.var_t0__blk320 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk321_dn11), (2.0 * locals.var_t1__blk321_dn12), (2.0 * locals.var_t1__blk321_dn17),)
    } else {
        (locals.var_t3__blk323, locals.var_t3__blk323_dn0, locals.var_t3__blk323_dn2, locals.var_t3__blk323_dn6, locals.var_t3__blk323_dn7, locals.var_t3__blk323_dn10, locals.var_t3__blk323_dn11, locals.var_t3__blk323_dn12, locals.var_t3__blk323_dn17,)
    }
};
        locals.var_t3__blk323 = assign12510_e14463;
        locals.var_t3__blk323_dn0 = assign12510_e14463_d_n0;
        locals.var_t3__blk323_dn2 = assign12510_e14463_d_n2;
        locals.var_t3__blk323_dn6 = assign12510_e14463_d_n6;
        locals.var_t3__blk323_dn7 = assign12510_e14463_d_n7;
        locals.var_t3__blk323_dn10 = assign12510_e14463_d_n10;
        locals.var_t3__blk323_dn11 = assign12510_e14463_d_n11;
        locals.var_t3__blk323_dn12 = assign12510_e14463_d_n12;
        locals.var_t3__blk323_dn17 = assign12510_e14463_d_n17;

        let (assign12520_e14483, assign12520_e14483_d_n0, assign12520_e14483_d_n2, assign12520_e14483_d_n6, assign12520_e14483_d_n7, assign12520_e14483_d_n10, assign12520_e14483_d_n11, assign12520_e14483_d_n12, assign12520_e14483_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign12520_e14479: f64 = (locals.var_t3__blk323 - locals.var_t2__blk322);
        let assign12520_e14481: f64 = (assign12520_e14479 / 2.0);
        (assign12520_e14481, ((locals.var_t3__blk323_dn0 - locals.var_t2__blk322_dn0) / 2.0), ((locals.var_t3__blk323_dn2 - locals.var_t2__blk322_dn2) / 2.0), ((locals.var_t3__blk323_dn6 - locals.var_t2__blk322_dn6) / 2.0), ((locals.var_t3__blk323_dn7 - locals.var_t2__blk322_dn7) / 2.0), ((locals.var_t3__blk323_dn10 - locals.var_t2__blk322_dn10) / 2.0), ((locals.var_t3__blk323_dn11 - locals.var_t2__blk322_dn11) / 2.0), ((locals.var_t3__blk323_dn12 - locals.var_t2__blk322_dn12) / 2.0), ((locals.var_t3__blk323_dn17 - locals.var_t2__blk322_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk324, locals.var_psb_inia__blk324_dn0, locals.var_psb_inia__blk324_dn2, locals.var_psb_inia__blk324_dn6, locals.var_psb_inia__blk324_dn7, locals.var_psb_inia__blk324_dn10, locals.var_psb_inia__blk324_dn11, locals.var_psb_inia__blk324_dn12, locals.var_psb_inia__blk324_dn17,)
    }
};
        locals.var_psb_inia__blk324 = assign12520_e14483;
        locals.var_psb_inia__blk324_dn0 = assign12520_e14483_d_n0;
        locals.var_psb_inia__blk324_dn2 = assign12520_e14483_d_n2;
        locals.var_psb_inia__blk324_dn6 = assign12520_e14483_d_n6;
        locals.var_psb_inia__blk324_dn7 = assign12520_e14483_d_n7;
        locals.var_psb_inia__blk324_dn10 = assign12520_e14483_d_n10;
        locals.var_psb_inia__blk324_dn11 = assign12520_e14483_d_n11;
        locals.var_psb_inia__blk324_dn12 = assign12520_e14483_d_n12;
        locals.var_psb_inia__blk324_dn17 = assign12520_e14483_d_n17;

        let (assign12530_e14512, assign12530_e14512_d_n0, assign12530_e14512_d_n2, assign12530_e14512_d_n6, assign12530_e14512_d_n7, assign12530_e14512_d_n10, assign12530_e14512_d_n11, assign12530_e14512_d_n12, assign12530_e14512_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign12530_e14499: f64 = (locals.var_t1__blk321 * locals.var_t1__blk321);
        let assign12530_e14501: f64 = (assign12530_e14499 / locals.var_t0__blk320);
        let assign12530_e14503: f64 = (assign12530_e14501 / locals.var_cnst1bulk);
        let assign12530_e14504: f64 = (assign12530_e14503).ln();
        let assign12530_e14508: f64 = (2.0 / locals.var_t1__blk321);
        let assign12530_e14509: f64 = (locals.var_beta + assign12530_e14508);
        let assign12530_e14510: f64 = (assign12530_e14504 / assign12530_e14509);
        (assign12530_e14510, ((((((((((locals.var_t1__blk321_dn0 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn0)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12530_e14501 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12530_e14503) * assign12530_e14509) - (assign12530_e14504 * (-((2.0 * locals.var_t1__blk321_dn0) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12530_e14509 * assign12530_e14509)), ((((((((((locals.var_t1__blk321_dn2 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn2)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12530_e14501 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12530_e14503) * assign12530_e14509) - (assign12530_e14504 * (-((2.0 * locals.var_t1__blk321_dn2) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12530_e14509 * assign12530_e14509)), ((((((((((locals.var_t1__blk321_dn6 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn6)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12530_e14501 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12530_e14503) * assign12530_e14509) - (assign12530_e14504 * (-((2.0 * locals.var_t1__blk321_dn6) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12530_e14509 * assign12530_e14509)), ((((((((((locals.var_t1__blk321_dn7 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn7)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12530_e14501 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12530_e14503) * assign12530_e14509) - (assign12530_e14504 * (-((2.0 * locals.var_t1__blk321_dn7) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12530_e14509 * assign12530_e14509)), ((((((((((((locals.var_t1__blk321_dn10 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn10)) * locals.var_t0__blk320) - (assign12530_e14499 * locals.var_t0__blk320_dn10)) / (locals.var_t0__blk320 * locals.var_t0__blk320)) * locals.var_cnst1bulk) - (assign12530_e14501 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12530_e14503) * assign12530_e14509) - (assign12530_e14504 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk321_dn10) / (locals.var_t1__blk321 * locals.var_t1__blk321)))))) / (assign12530_e14509 * assign12530_e14509)), ((((((((((locals.var_t1__blk321_dn11 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn11)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12530_e14501 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12530_e14503) * assign12530_e14509) - (assign12530_e14504 * (-((2.0 * locals.var_t1__blk321_dn11) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12530_e14509 * assign12530_e14509)), ((((((((((locals.var_t1__blk321_dn12 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn12)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12530_e14501 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12530_e14503) * assign12530_e14509) - (assign12530_e14504 * (-((2.0 * locals.var_t1__blk321_dn12) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12530_e14509 * assign12530_e14509)), ((((((((((locals.var_t1__blk321_dn17 * locals.var_t1__blk321) + (locals.var_t1__blk321 * locals.var_t1__blk321_dn17)) / locals.var_t0__blk320) * locals.var_cnst1bulk) - (assign12530_e14501 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12530_e14503) * assign12530_e14509) - (assign12530_e14504 * (-((2.0 * locals.var_t1__blk321_dn17) / (locals.var_t1__blk321 * locals.var_t1__blk321))))) / (assign12530_e14509 * assign12530_e14509)),)
    } else {
        (locals.var_psb_inib__blk325, locals.var_psb_inib__blk325_dn0, locals.var_psb_inib__blk325_dn2, locals.var_psb_inib__blk325_dn6, locals.var_psb_inib__blk325_dn7, locals.var_psb_inib__blk325_dn10, locals.var_psb_inib__blk325_dn11, locals.var_psb_inib__blk325_dn12, locals.var_psb_inib__blk325_dn17,)
    }
};
        locals.var_psb_inib__blk325 = assign12530_e14512;
        locals.var_psb_inib__blk325_dn0 = assign12530_e14512_d_n0;
        locals.var_psb_inib__blk325_dn2 = assign12530_e14512_d_n2;
        locals.var_psb_inib__blk325_dn6 = assign12530_e14512_d_n6;
        locals.var_psb_inib__blk325_dn7 = assign12530_e14512_d_n7;
        locals.var_psb_inib__blk325_dn10 = assign12530_e14512_d_n10;
        locals.var_psb_inib__blk325_dn11 = assign12530_e14512_d_n11;
        locals.var_psb_inib__blk325_dn12 = assign12530_e14512_d_n12;
        locals.var_psb_inib__blk325_dn17 = assign12530_e14512_d_n17;

        let assign12540_e14515: f64 = if locals.var_psb_inia__blk324 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard328 = assign12540_e14515;

        let (assign12550_e14533, assign12550_e14533_d_n0, assign12550_e14533_d_n2, assign12550_e14533_d_n6, assign12550_e14533_d_n7, assign12550_e14533_d_n10, assign12550_e14533_d_n11, assign12550_e14533_d_n12, assign12550_e14533_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard328 != 0.0)) {
        (locals.var_psb_inia__blk324, locals.var_psb_inia__blk324_dn0, locals.var_psb_inia__blk324_dn2, locals.var_psb_inia__blk324_dn6, locals.var_psb_inia__blk324_dn7, locals.var_psb_inia__blk324_dn10, locals.var_psb_inia__blk324_dn11, locals.var_psb_inia__blk324_dn12, locals.var_psb_inia__blk324_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12550_e14533;
        locals.var_phi_sl_bulk_dn0 = assign12550_e14533_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12550_e14533_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12550_e14533_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12550_e14533_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12550_e14533_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12550_e14533_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12550_e14533_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12550_e14533_d_n17;

        let (assign12560_e14556, assign12560_e14556_d_n0, assign12560_e14556_d_n2, assign12560_e14556_d_n6, assign12560_e14556_d_n7, assign12560_e14556_d_n10, assign12560_e14556_d_n11, assign12560_e14556_d_n12, assign12560_e14556_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12560_e14552: f64 = (locals.var_psb_inib__blk325 - locals.var_psb_inia__blk324);
        let assign12560_e14554: f64 = (assign12560_e14552 - 0.0008);
        (assign12560_e14554, (locals.var_psb_inib__blk325_dn0 - locals.var_psb_inia__blk324_dn0), (locals.var_psb_inib__blk325_dn2 - locals.var_psb_inia__blk324_dn2), (locals.var_psb_inib__blk325_dn6 - locals.var_psb_inia__blk324_dn6), (locals.var_psb_inib__blk325_dn7 - locals.var_psb_inia__blk324_dn7), (locals.var_psb_inib__blk325_dn10 - locals.var_psb_inia__blk324_dn10), (locals.var_psb_inib__blk325_dn11 - locals.var_psb_inia__blk324_dn11), (locals.var_psb_inib__blk325_dn12 - locals.var_psb_inia__blk324_dn12), (locals.var_psb_inib__blk325_dn17 - locals.var_psb_inia__blk324_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12560_e14556;
        locals.var_tmf1_dn0 = assign12560_e14556_d_n0;
        locals.var_tmf1_dn2 = assign12560_e14556_d_n2;
        locals.var_tmf1_dn6 = assign12560_e14556_d_n6;
        locals.var_tmf1_dn7 = assign12560_e14556_d_n7;
        locals.var_tmf1_dn10 = assign12560_e14556_d_n10;
        locals.var_tmf1_dn11 = assign12560_e14556_d_n11;
        locals.var_tmf1_dn12 = assign12560_e14556_d_n12;
        locals.var_tmf1_dn17 = assign12560_e14556_d_n17;

        let (assign12570_e14579, assign12570_e14579_d_n0, assign12570_e14579_d_n2, assign12570_e14579_d_n6, assign12570_e14579_d_n7, assign12570_e14579_d_n10, assign12570_e14579_d_n11, assign12570_e14579_d_n12, assign12570_e14579_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12570_e14575: f64 = (4.0 * locals.var_psb_inib__blk325);
        let assign12570_e14577: f64 = (assign12570_e14575 * 0.0008);
        (assign12570_e14577, ((4.0 * locals.var_psb_inib__blk325_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk325_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12570_e14579;
        locals.var_tmf2_dn0 = assign12570_e14579_d_n0;
        locals.var_tmf2_dn2 = assign12570_e14579_d_n2;
        locals.var_tmf2_dn6 = assign12570_e14579_d_n6;
        locals.var_tmf2_dn7 = assign12570_e14579_d_n7;
        locals.var_tmf2_dn10 = assign12570_e14579_d_n10;
        locals.var_tmf2_dn11 = assign12570_e14579_d_n11;
        locals.var_tmf2_dn12 = assign12570_e14579_d_n12;
        locals.var_tmf2_dn17 = assign12570_e14579_d_n17;

        let (assign12580_e14604, assign12580_e14604_d_n0, assign12580_e14604_d_n2, assign12580_e14604_d_n6, assign12580_e14604_d_n7, assign12580_e14604_d_n10, assign12580_e14604_d_n11, assign12580_e14604_d_n12, assign12580_e14604_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let (assign12580_e14602, assign12580_e14602_d_n0, assign12580_e14602_d_n2, assign12580_e14602_d_n6, assign12580_e14602_d_n7, assign12580_e14602_d_n10, assign12580_e14602_d_n11, assign12580_e14602_d_n12, assign12580_e14602_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12580_e14601: f64 = (-locals.var_tmf2);
                (assign12580_e14601, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12580_e14602, assign12580_e14602_d_n0, assign12580_e14602_d_n2, assign12580_e14602_d_n6, assign12580_e14602_d_n7, assign12580_e14602_d_n10, assign12580_e14602_d_n11, assign12580_e14602_d_n12, assign12580_e14602_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12580_e14604;
        locals.var_tmf2_dn0 = assign12580_e14604_d_n0;
        locals.var_tmf2_dn2 = assign12580_e14604_d_n2;
        locals.var_tmf2_dn6 = assign12580_e14604_d_n6;
        locals.var_tmf2_dn7 = assign12580_e14604_d_n7;
        locals.var_tmf2_dn10 = assign12580_e14604_d_n10;
        locals.var_tmf2_dn11 = assign12580_e14604_d_n11;
        locals.var_tmf2_dn12 = assign12580_e14604_d_n12;
        locals.var_tmf2_dn17 = assign12580_e14604_d_n17;

        let (assign12590_e14628, assign12590_e14628_d_n0, assign12590_e14628_d_n2, assign12590_e14628_d_n6, assign12590_e14628_d_n7, assign12590_e14628_d_n10, assign12590_e14628_d_n11, assign12590_e14628_d_n12, assign12590_e14628_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12590_e14623: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12590_e14625: f64 = (assign12590_e14623 + locals.var_tmf2);
        let assign12590_e14626: f64 = (assign12590_e14625).sqrt();
        (assign12590_e14626, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12590_e14626)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12590_e14626)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12590_e14626)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12590_e14626)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12590_e14626)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12590_e14626)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12590_e14626)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12590_e14626)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12590_e14628;
        locals.var_tmf2_dn0 = assign12590_e14628_d_n0;
        locals.var_tmf2_dn2 = assign12590_e14628_d_n2;
        locals.var_tmf2_dn6 = assign12590_e14628_d_n6;
        locals.var_tmf2_dn7 = assign12590_e14628_d_n7;
        locals.var_tmf2_dn10 = assign12590_e14628_d_n10;
        locals.var_tmf2_dn11 = assign12590_e14628_d_n11;
        locals.var_tmf2_dn12 = assign12590_e14628_d_n12;
        locals.var_tmf2_dn17 = assign12590_e14628_d_n17;

        let (assign12600_e14653, assign12600_e14653_d_n0, assign12600_e14653_d_n2, assign12600_e14653_d_n6, assign12600_e14653_d_n7, assign12600_e14653_d_n10, assign12600_e14653_d_n11, assign12600_e14653_d_n12, assign12600_e14653_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12600_e14649: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12600_e14650: f64 = (0.5 * assign12600_e14649);
        let assign12600_e14651: f64 = (locals.var_psb_inib__blk325 - assign12600_e14650);
        (assign12600_e14651, (locals.var_psb_inib__blk325_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk325_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk325_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk325_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk325_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk325_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk325_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk325_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12600_e14653;
        locals.var_phi_sl_bulk_dn0 = assign12600_e14653_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12600_e14653_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12600_e14653_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12600_e14653_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12600_e14653_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12600_e14653_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12600_e14653_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12600_e14653_d_n17;

        let (assign12610_e14674, assign12610_e14674_d_n0, assign12610_e14674_d_n2, assign12610_e14674_d_n6, assign12610_e14674_d_n7, assign12610_e14674_d_n10, assign12610_e14674_d_n11, assign12610_e14674_d_n12, assign12610_e14674_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign12610_e14666: f64 = (2.0 * 1.034943e-10);
        let assign12610_e14668: f64 = (assign12610_e14666 / 1.6021918e-19);
        let assign12610_e14670: f64 = (assign12610_e14668 * locals.var_phi_sl_soi);
        let assign12610_e14672: f64 = (assign12610_e14670 / locals.var_uc_nsubs);
        (assign12610_e14672, ((((assign12610_e14668 * locals.var_phi_sl_soi_dn0) * locals.var_uc_nsubs) - (assign12610_e14670 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12610_e14668 * locals.var_phi_sl_soi_dn2) * locals.var_uc_nsubs) - (assign12610_e14670 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12610_e14668 * locals.var_phi_sl_soi_dn6) * locals.var_uc_nsubs) - (assign12610_e14670 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12610_e14668 * locals.var_phi_sl_soi_dn7) * locals.var_uc_nsubs) - (assign12610_e14670 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12610_e14668 * locals.var_phi_sl_soi_dn10) * locals.var_uc_nsubs) - (assign12610_e14670 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12610_e14668 * locals.var_phi_sl_soi_dn11) * locals.var_uc_nsubs) - (assign12610_e14670 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12610_e14668 * locals.var_phi_sl_soi_dn12) * locals.var_uc_nsubs) - (assign12610_e14670 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12610_e14668 * locals.var_phi_sl_soi_dn17) * locals.var_uc_nsubs) - (assign12610_e14670 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_t0__blk329, locals.var_t0__blk329_dn0, locals.var_t0__blk329_dn2, locals.var_t0__blk329_dn6, locals.var_t0__blk329_dn7, locals.var_t0__blk329_dn10, locals.var_t0__blk329_dn11, locals.var_t0__blk329_dn12, locals.var_t0__blk329_dn17,)
    }
};
        locals.var_t0__blk329 = assign12610_e14674;
        locals.var_t0__blk329_dn0 = assign12610_e14674_d_n0;
        locals.var_t0__blk329_dn2 = assign12610_e14674_d_n2;
        locals.var_t0__blk329_dn6 = assign12610_e14674_d_n6;
        locals.var_t0__blk329_dn7 = assign12610_e14674_d_n7;
        locals.var_t0__blk329_dn10 = assign12610_e14674_d_n10;
        locals.var_t0__blk329_dn11 = assign12610_e14674_d_n11;
        locals.var_t0__blk329_dn12 = assign12610_e14674_d_n12;
        locals.var_t0__blk329_dn17 = assign12610_e14674_d_n17;

        let assign12620_e14677: f64 = if locals.var_t0__blk329 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard337 = assign12620_e14677;

        let (assign12630_e14701, assign12630_e14701_d_n0, assign12630_e14701_d_n2, assign12630_e14701_d_n6, assign12630_e14701_d_n7, assign12630_e14701_d_n10, assign12630_e14701_d_n11, assign12630_e14701_d_n12, assign12630_e14701_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign12630_e14692: f64 = (2.0 * 1.034943e-10);
        let assign12630_e14694: f64 = (assign12630_e14692 / 1.6021918e-19);
        let assign12630_e14696: f64 = (assign12630_e14694 * locals.var_phi_sl_soi);
        let assign12630_e14698: f64 = (assign12630_e14696 / locals.var_uc_nsubs);
        let assign12630_e14699: f64 = (assign12630_e14698).sqrt();
        (assign12630_e14699, (((((assign12630_e14694 * locals.var_phi_sl_soi_dn0) * locals.var_uc_nsubs) - (assign12630_e14696 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12630_e14699)), (((((assign12630_e14694 * locals.var_phi_sl_soi_dn2) * locals.var_uc_nsubs) - (assign12630_e14696 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12630_e14699)), (((((assign12630_e14694 * locals.var_phi_sl_soi_dn6) * locals.var_uc_nsubs) - (assign12630_e14696 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12630_e14699)), (((((assign12630_e14694 * locals.var_phi_sl_soi_dn7) * locals.var_uc_nsubs) - (assign12630_e14696 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12630_e14699)), (((((assign12630_e14694 * locals.var_phi_sl_soi_dn10) * locals.var_uc_nsubs) - (assign12630_e14696 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12630_e14699)), (((((assign12630_e14694 * locals.var_phi_sl_soi_dn11) * locals.var_uc_nsubs) - (assign12630_e14696 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12630_e14699)), (((((assign12630_e14694 * locals.var_phi_sl_soi_dn12) * locals.var_uc_nsubs) - (assign12630_e14696 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12630_e14699)), (((((assign12630_e14694 * locals.var_phi_sl_soi_dn17) * locals.var_uc_nsubs) - (assign12630_e14696 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12630_e14699)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
        locals.var_wdsoi = assign12630_e14701;
        locals.var_wdsoi_dn0 = assign12630_e14701_d_n0;
        locals.var_wdsoi_dn2 = assign12630_e14701_d_n2;
        locals.var_wdsoi_dn6 = assign12630_e14701_d_n6;
        locals.var_wdsoi_dn7 = assign12630_e14701_d_n7;
        locals.var_wdsoi_dn10 = assign12630_e14701_d_n10;
        locals.var_wdsoi_dn11 = assign12630_e14701_d_n11;
        locals.var_wdsoi_dn12 = assign12630_e14701_d_n12;
        locals.var_wdsoi_dn17 = assign12630_e14701_d_n17;

        let (assign12640_e14717, assign12640_e14717_d_n0, assign12640_e14717_d_n2, assign12640_e14717_d_n6, assign12640_e14717_d_n7, assign12640_e14717_d_n10, assign12640_e14717_d_n11, assign12640_e14717_d_n12, assign12640_e14717_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard337 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
        locals.var_wdsoi = assign12640_e14717;
        locals.var_wdsoi_dn0 = assign12640_e14717_d_n0;
        locals.var_wdsoi_dn2 = assign12640_e14717_d_n2;
        locals.var_wdsoi_dn6 = assign12640_e14717_d_n6;
        locals.var_wdsoi_dn7 = assign12640_e14717_d_n7;
        locals.var_wdsoi_dn10 = assign12640_e14717_d_n10;
        locals.var_wdsoi_dn11 = assign12640_e14717_d_n11;
        locals.var_wdsoi_dn12 = assign12640_e14717_d_n12;
        locals.var_wdsoi_dn17 = assign12640_e14717_d_n17;

        let assign12650_e14722: f64 = if ((locals.var_phi_sl_soi < locals.var_fd_end) && (0.0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard338 = assign12650_e14722;

        let (assign12670_e14752,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12670_e14752;

    }

    pub(super) fn stamp_transient_block_37(
        locals: &mut StampLocals,
    ) {
        let mut assign12680_loop_guard: usize = 0;
        while {
            let assign12680_cond_e14768: f64 = if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_lp_sl < locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12680_cond_e14768 != 0.0
        } {
            assign12680_loop_guard += 1;
            assert!(assign12680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12680_body0_e14783, assign12680_body0_e14783_d_n10,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    } else {
        (locals.var_t1__blk330, locals.var_t1__blk330_dn10,)
    }
};
            locals.var_t1__blk330 = assign12680_body0_e14783;
            locals.var_t1__blk330_dn10 = assign12680_body0_e14783_d_n10;
            let (assign12680_body1_e14800, assign12680_body1_e14800_d_n0, assign12680_body1_e14800_d_n2, assign12680_body1_e14800_d_n6, assign12680_body1_e14800_d_n7, assign12680_body1_e14800_d_n10, assign12680_body1_e14800_d_n11, assign12680_body1_e14800_d_n12, assign12680_body1_e14800_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body1_e14798: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        (assign12680_body1_e14798, (locals.var_beta * locals.var_phi_sl_bulk_dn0), (locals.var_beta * locals.var_phi_sl_bulk_dn2), (locals.var_beta * locals.var_phi_sl_bulk_dn6), (locals.var_beta * locals.var_phi_sl_bulk_dn7), ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10)), (locals.var_beta * locals.var_phi_sl_bulk_dn11), (locals.var_beta * locals.var_phi_sl_bulk_dn12), (locals.var_beta * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t2__blk331, locals.var_t2__blk331_dn0, locals.var_t2__blk331_dn2, locals.var_t2__blk331_dn6, locals.var_t2__blk331_dn7, locals.var_t2__blk331_dn10, locals.var_t2__blk331_dn11, locals.var_t2__blk331_dn12, locals.var_t2__blk331_dn17,)
    }
};
            locals.var_t2__blk331 = assign12680_body1_e14800;
            locals.var_t2__blk331_dn0 = assign12680_body1_e14800_d_n0;
            locals.var_t2__blk331_dn2 = assign12680_body1_e14800_d_n2;
            locals.var_t2__blk331_dn6 = assign12680_body1_e14800_d_n6;
            locals.var_t2__blk331_dn7 = assign12680_body1_e14800_d_n7;
            locals.var_t2__blk331_dn10 = assign12680_body1_e14800_d_n10;
            locals.var_t2__blk331_dn11 = assign12680_body1_e14800_d_n11;
            locals.var_t2__blk331_dn12 = assign12680_body1_e14800_d_n12;
            locals.var_t2__blk331_dn17 = assign12680_body1_e14800_d_n17;
            let (assign12680_body2_e14817, assign12680_body2_e14817_d_n0, assign12680_body2_e14817_d_n2, assign12680_body2_e14817_d_n6, assign12680_body2_e14817_d_n7, assign12680_body2_e14817_d_n10, assign12680_body2_e14817_d_n11, assign12680_body2_e14817_d_n12, assign12680_body2_e14817_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body2_e14814: f64 = (-locals.var_t2__blk331);
        let assign12680_body2_e14815: f64 = (assign12680_body2_e14814).exp();
        (assign12680_body2_e14815, (assign12680_body2_e14815 * (-locals.var_t2__blk331_dn0)), (assign12680_body2_e14815 * (-locals.var_t2__blk331_dn2)), (assign12680_body2_e14815 * (-locals.var_t2__blk331_dn6)), (assign12680_body2_e14815 * (-locals.var_t2__blk331_dn7)), (assign12680_body2_e14815 * (-locals.var_t2__blk331_dn10)), (assign12680_body2_e14815 * (-locals.var_t2__blk331_dn11)), (assign12680_body2_e14815 * (-locals.var_t2__blk331_dn12)), (assign12680_body2_e14815 * (-locals.var_t2__blk331_dn17)),)
    } else {
        (locals.var_t3__blk332, locals.var_t3__blk332_dn0, locals.var_t3__blk332_dn2, locals.var_t3__blk332_dn6, locals.var_t3__blk332_dn7, locals.var_t3__blk332_dn10, locals.var_t3__blk332_dn11, locals.var_t3__blk332_dn12, locals.var_t3__blk332_dn17,)
    }
};
            locals.var_t3__blk332 = assign12680_body2_e14817;
            locals.var_t3__blk332_dn0 = assign12680_body2_e14817_d_n0;
            locals.var_t3__blk332_dn2 = assign12680_body2_e14817_d_n2;
            locals.var_t3__blk332_dn6 = assign12680_body2_e14817_d_n6;
            locals.var_t3__blk332_dn7 = assign12680_body2_e14817_d_n7;
            locals.var_t3__blk332_dn10 = assign12680_body2_e14817_d_n10;
            locals.var_t3__blk332_dn11 = assign12680_body2_e14817_d_n11;
            locals.var_t3__blk332_dn12 = assign12680_body2_e14817_d_n12;
            locals.var_t3__blk332_dn17 = assign12680_body2_e14817_d_n17;
            let assign12680_body3_e14820: f64 = if locals.var_phi_sl_bulk > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard339 = assign12680_body3_e14820;
            let (assign12680_body4_e14840, assign12680_body4_e14840_d_n0, assign12680_body4_e14840_d_n2, assign12680_body4_e14840_d_n6, assign12680_body4_e14840_d_n7, assign12680_body4_e14840_d_n10, assign12680_body4_e14840_d_n11, assign12680_body4_e14840_d_n12, assign12680_body4_e14840_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard339 != 0.0)) {
        let assign12680_body4_e14837: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        let assign12680_body4_e14838: f64 = (assign12680_body4_e14837).exp();
        (assign12680_body4_e14838, (assign12680_body4_e14838 * (locals.var_beta * locals.var_phi_sl_bulk_dn0)), (assign12680_body4_e14838 * (locals.var_beta * locals.var_phi_sl_bulk_dn2)), (assign12680_body4_e14838 * (locals.var_beta * locals.var_phi_sl_bulk_dn6)), (assign12680_body4_e14838 * (locals.var_beta * locals.var_phi_sl_bulk_dn7)), (assign12680_body4_e14838 * ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10))), (assign12680_body4_e14838 * (locals.var_beta * locals.var_phi_sl_bulk_dn11)), (assign12680_body4_e14838 * (locals.var_beta * locals.var_phi_sl_bulk_dn12)), (assign12680_body4_e14838 * (locals.var_beta * locals.var_phi_sl_bulk_dn17)),)
    } else {
        (locals.var_t0__blk329, locals.var_t0__blk329_dn0, locals.var_t0__blk329_dn2, locals.var_t0__blk329_dn6, locals.var_t0__blk329_dn7, locals.var_t0__blk329_dn10, locals.var_t0__blk329_dn11, locals.var_t0__blk329_dn12, locals.var_t0__blk329_dn17,)
    }
};
            locals.var_t0__blk329 = assign12680_body4_e14840;
            locals.var_t0__blk329_dn0 = assign12680_body4_e14840_d_n0;
            locals.var_t0__blk329_dn2 = assign12680_body4_e14840_d_n2;
            locals.var_t0__blk329_dn6 = assign12680_body4_e14840_d_n6;
            locals.var_t0__blk329_dn7 = assign12680_body4_e14840_d_n7;
            locals.var_t0__blk329_dn10 = assign12680_body4_e14840_d_n10;
            locals.var_t0__blk329_dn11 = assign12680_body4_e14840_d_n11;
            locals.var_t0__blk329_dn12 = assign12680_body4_e14840_d_n12;
            locals.var_t0__blk329_dn17 = assign12680_body4_e14840_d_n17;
            let (assign12680_body5_e14871, assign12680_body5_e14871_d_n0, assign12680_body5_e14871_d_n2, assign12680_body5_e14871_d_n6, assign12680_body5_e14871_d_n7, assign12680_body5_e14871_d_n10, assign12680_body5_e14871_d_n11, assign12680_body5_e14871_d_n12, assign12680_body5_e14871_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard339 != 0.0)) {
        let assign12680_body5_e14856: f64 = (-locals.var_t1__blk330);
        let assign12680_body5_e14859: f64 = (locals.var_t3__blk332 + locals.var_t2__blk331);
        let assign12680_body5_e14861: f64 = (assign12680_body5_e14859 - 1.0);
        let assign12680_body5_e14865: f64 = (locals.var_t0__blk329 - 1.0);
        let assign12680_body5_e14866: f64 = (locals.var_cnst1bulk * assign12680_body5_e14865);
        let assign12680_body5_e14867: f64 = (assign12680_body5_e14861 + assign12680_body5_e14866);
        let assign12680_body5_e14868: f64 = (assign12680_body5_e14867).sqrt();
        let assign12680_body5_e14869: f64 = (assign12680_body5_e14856 * assign12680_body5_e14868);
        (assign12680_body5_e14869, (assign12680_body5_e14856 * (((locals.var_t3__blk332_dn0 + locals.var_t2__blk331_dn0) + ((locals.var_cnst1bulk_dn0 * assign12680_body5_e14865) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn0))) / (2.0 * assign12680_body5_e14868))), (assign12680_body5_e14856 * (((locals.var_t3__blk332_dn2 + locals.var_t2__blk331_dn2) + ((locals.var_cnst1bulk_dn2 * assign12680_body5_e14865) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn2))) / (2.0 * assign12680_body5_e14868))), (assign12680_body5_e14856 * (((locals.var_t3__blk332_dn6 + locals.var_t2__blk331_dn6) + ((locals.var_cnst1bulk_dn6 * assign12680_body5_e14865) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn6))) / (2.0 * assign12680_body5_e14868))), (assign12680_body5_e14856 * (((locals.var_t3__blk332_dn7 + locals.var_t2__blk331_dn7) + ((locals.var_cnst1bulk_dn7 * assign12680_body5_e14865) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn7))) / (2.0 * assign12680_body5_e14868))), (((-locals.var_t1__blk330_dn10) * assign12680_body5_e14868) + (assign12680_body5_e14856 * (((locals.var_t3__blk332_dn10 + locals.var_t2__blk331_dn10) + ((locals.var_cnst1bulk_dn10 * assign12680_body5_e14865) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn10))) / (2.0 * assign12680_body5_e14868)))), (assign12680_body5_e14856 * (((locals.var_t3__blk332_dn11 + locals.var_t2__blk331_dn11) + ((locals.var_cnst1bulk_dn11 * assign12680_body5_e14865) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn11))) / (2.0 * assign12680_body5_e14868))), (assign12680_body5_e14856 * (((locals.var_t3__blk332_dn12 + locals.var_t2__blk331_dn12) + ((locals.var_cnst1bulk_dn12 * assign12680_body5_e14865) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn12))) / (2.0 * assign12680_body5_e14868))), (assign12680_body5_e14856 * (((locals.var_t3__blk332_dn17 + locals.var_t2__blk331_dn17) + ((locals.var_cnst1bulk_dn17 * assign12680_body5_e14865) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn17))) / (2.0 * assign12680_body5_e14868))),)
    } else {
        (locals.var_t4__blk333, locals.var_t4__blk333_dn0, locals.var_t4__blk333_dn2, locals.var_t4__blk333_dn6, locals.var_t4__blk333_dn7, locals.var_t4__blk333_dn10, locals.var_t4__blk333_dn11, locals.var_t4__blk333_dn12, locals.var_t4__blk333_dn17,)
    }
};
            locals.var_t4__blk333 = assign12680_body5_e14871;
            locals.var_t4__blk333_dn0 = assign12680_body5_e14871_d_n0;
            locals.var_t4__blk333_dn2 = assign12680_body5_e14871_d_n2;
            locals.var_t4__blk333_dn6 = assign12680_body5_e14871_d_n6;
            locals.var_t4__blk333_dn7 = assign12680_body5_e14871_d_n7;
            locals.var_t4__blk333_dn10 = assign12680_body5_e14871_d_n10;
            locals.var_t4__blk333_dn11 = assign12680_body5_e14871_d_n11;
            locals.var_t4__blk333_dn12 = assign12680_body5_e14871_d_n12;
            locals.var_t4__blk333_dn17 = assign12680_body5_e14871_d_n17;
            let (assign12680_body6_e14899, assign12680_body6_e14899_d_n0, assign12680_body6_e14899_d_n2, assign12680_body6_e14899_d_n6, assign12680_body6_e14899_d_n7, assign12680_body6_e14899_d_n10, assign12680_body6_e14899_d_n11, assign12680_body6_e14899_d_n12, assign12680_body6_e14899_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard339 != 0.0)) {
        let assign12680_body6_e14888: f64 = (locals.var_c0bulk / locals.var_t4__blk333);
        let assign12680_body6_e14890: f64 = (-locals.var_t3__blk332);
        let assign12680_body6_e14892: f64 = (assign12680_body6_e14890 + 1.0);
        let assign12680_body6_e14895: f64 = (locals.var_cnst1bulk * locals.var_t0__blk329);
        let assign12680_body6_e14896: f64 = (assign12680_body6_e14892 + assign12680_body6_e14895);
        let assign12680_body6_e14897: f64 = (assign12680_body6_e14888 * assign12680_body6_e14896);
        (assign12680_body6_e14897, (((-((locals.var_c0bulk * locals.var_t4__blk333_dn0) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body6_e14896) + (assign12680_body6_e14888 * ((-locals.var_t3__blk332_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn0))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn2) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body6_e14896) + (assign12680_body6_e14888 * ((-locals.var_t3__blk332_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn2))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn6) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body6_e14896) + (assign12680_body6_e14888 * ((-locals.var_t3__blk332_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn6))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn7) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body6_e14896) + (assign12680_body6_e14888 * ((-locals.var_t3__blk332_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn7))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn10) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body6_e14896) + (assign12680_body6_e14888 * ((-locals.var_t3__blk332_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn10))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn11) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body6_e14896) + (assign12680_body6_e14888 * ((-locals.var_t3__blk332_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn11))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn12) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body6_e14896) + (assign12680_body6_e14888 * ((-locals.var_t3__blk332_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn12))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn17) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body6_e14896) + (assign12680_body6_e14888 * ((-locals.var_t3__blk332_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn17))))),)
    } else {
        (locals.var_t5__blk334, locals.var_t5__blk334_dn0, locals.var_t5__blk334_dn2, locals.var_t5__blk334_dn6, locals.var_t5__blk334_dn7, locals.var_t5__blk334_dn10, locals.var_t5__blk334_dn11, locals.var_t5__blk334_dn12, locals.var_t5__blk334_dn17,)
    }
};
            locals.var_t5__blk334 = assign12680_body6_e14899;
            locals.var_t5__blk334_dn0 = assign12680_body6_e14899_d_n0;
            locals.var_t5__blk334_dn2 = assign12680_body6_e14899_d_n2;
            locals.var_t5__blk334_dn6 = assign12680_body6_e14899_d_n6;
            locals.var_t5__blk334_dn7 = assign12680_body6_e14899_d_n7;
            locals.var_t5__blk334_dn10 = assign12680_body6_e14899_d_n10;
            locals.var_t5__blk334_dn11 = assign12680_body6_e14899_d_n11;
            locals.var_t5__blk334_dn12 = assign12680_body6_e14899_d_n12;
            locals.var_t5__blk334_dn17 = assign12680_body6_e14899_d_n17;
            let assign12680_body7_e14902: f64 = (-1e-9);
            let assign12680_body7_e14903: f64 = if locals.var_phi_sl_bulk < assign12680_body7_e14902 { 1.0 } else { 0.0 };
            locals.var_guard340 = assign12680_body7_e14903;
            let (assign12680_body8_e14930, assign12680_body8_e14930_d_n0, assign12680_body8_e14930_d_n2, assign12680_body8_e14930_d_n6, assign12680_body8_e14930_d_n7, assign12680_body8_e14930_d_n10, assign12680_body8_e14930_d_n11, assign12680_body8_e14930_d_n12, assign12680_body8_e14930_d_n17,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard339 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12680_body8_e14924: f64 = (locals.var_t3__blk332 + locals.var_t2__blk331);
        let assign12680_body8_e14926: f64 = (assign12680_body8_e14924 - 1.0);
        let assign12680_body8_e14927: f64 = (assign12680_body8_e14926).sqrt();
        let assign12680_body8_e14928: f64 = (locals.var_t1__blk330 * assign12680_body8_e14927);
        (assign12680_body8_e14928, (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn0 + locals.var_t2__blk331_dn0) / (2.0 * assign12680_body8_e14927))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn2 + locals.var_t2__blk331_dn2) / (2.0 * assign12680_body8_e14927))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn6 + locals.var_t2__blk331_dn6) / (2.0 * assign12680_body8_e14927))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn7 + locals.var_t2__blk331_dn7) / (2.0 * assign12680_body8_e14927))), ((locals.var_t1__blk330_dn10 * assign12680_body8_e14927) + (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn10 + locals.var_t2__blk331_dn10) / (2.0 * assign12680_body8_e14927)))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn11 + locals.var_t2__blk331_dn11) / (2.0 * assign12680_body8_e14927))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn12 + locals.var_t2__blk331_dn12) / (2.0 * assign12680_body8_e14927))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn17 + locals.var_t2__blk331_dn17) / (2.0 * assign12680_body8_e14927))),)
    } else {
        (locals.var_t4__blk333, locals.var_t4__blk333_dn0, locals.var_t4__blk333_dn2, locals.var_t4__blk333_dn6, locals.var_t4__blk333_dn7, locals.var_t4__blk333_dn10, locals.var_t4__blk333_dn11, locals.var_t4__blk333_dn12, locals.var_t4__blk333_dn17,)
    }
};
            locals.var_t4__blk333 = assign12680_body8_e14930;
            locals.var_t4__blk333_dn0 = assign12680_body8_e14930_d_n0;
            locals.var_t4__blk333_dn2 = assign12680_body8_e14930_d_n2;
            locals.var_t4__blk333_dn6 = assign12680_body8_e14930_d_n6;
            locals.var_t4__blk333_dn7 = assign12680_body8_e14930_d_n7;
            locals.var_t4__blk333_dn10 = assign12680_body8_e14930_d_n10;
            locals.var_t4__blk333_dn11 = assign12680_body8_e14930_d_n11;
            locals.var_t4__blk333_dn12 = assign12680_body8_e14930_d_n12;
            locals.var_t4__blk333_dn17 = assign12680_body8_e14930_d_n17;
            let (assign12680_body9_e14957, assign12680_body9_e14957_d_n0, assign12680_body9_e14957_d_n2, assign12680_body9_e14957_d_n6, assign12680_body9_e14957_d_n7, assign12680_body9_e14957_d_n10, assign12680_body9_e14957_d_n11, assign12680_body9_e14957_d_n12, assign12680_body9_e14957_d_n17,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard339 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12680_body9_e14950: f64 = (locals.var_c0bulk / locals.var_t4__blk333);
        let assign12680_body9_e14952: f64 = (-locals.var_t3__blk332);
        let assign12680_body9_e14954: f64 = (assign12680_body9_e14952 + 1.0);
        let assign12680_body9_e14955: f64 = (assign12680_body9_e14950 * assign12680_body9_e14954);
        (assign12680_body9_e14955, (((-((locals.var_c0bulk * locals.var_t4__blk333_dn0) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body9_e14954) + (assign12680_body9_e14950 * (-locals.var_t3__blk332_dn0))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn2) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body9_e14954) + (assign12680_body9_e14950 * (-locals.var_t3__blk332_dn2))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn6) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body9_e14954) + (assign12680_body9_e14950 * (-locals.var_t3__blk332_dn6))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn7) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body9_e14954) + (assign12680_body9_e14950 * (-locals.var_t3__blk332_dn7))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn10) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body9_e14954) + (assign12680_body9_e14950 * (-locals.var_t3__blk332_dn10))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn11) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body9_e14954) + (assign12680_body9_e14950 * (-locals.var_t3__blk332_dn11))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn12) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body9_e14954) + (assign12680_body9_e14950 * (-locals.var_t3__blk332_dn12))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn17) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12680_body9_e14954) + (assign12680_body9_e14950 * (-locals.var_t3__blk332_dn17))),)
    } else {
        (locals.var_t5__blk334, locals.var_t5__blk334_dn0, locals.var_t5__blk334_dn2, locals.var_t5__blk334_dn6, locals.var_t5__blk334_dn7, locals.var_t5__blk334_dn10, locals.var_t5__blk334_dn11, locals.var_t5__blk334_dn12, locals.var_t5__blk334_dn17,)
    }
};
            locals.var_t5__blk334 = assign12680_body9_e14957;
            locals.var_t5__blk334_dn0 = assign12680_body9_e14957_d_n0;
            locals.var_t5__blk334_dn2 = assign12680_body9_e14957_d_n2;
            locals.var_t5__blk334_dn6 = assign12680_body9_e14957_d_n6;
            locals.var_t5__blk334_dn7 = assign12680_body9_e14957_d_n7;
            locals.var_t5__blk334_dn10 = assign12680_body9_e14957_d_n10;
            locals.var_t5__blk334_dn11 = assign12680_body9_e14957_d_n11;
            locals.var_t5__blk334_dn12 = assign12680_body9_e14957_d_n12;
            locals.var_t5__blk334_dn17 = assign12680_body9_e14957_d_n17;
            let (assign12680_body10_e14986, assign12680_body10_e14986_d_n0, assign12680_body10_e14986_d_n2, assign12680_body10_e14986_d_n6, assign12680_body10_e14986_d_n7, assign12680_body10_e14986_d_n10, assign12680_body10_e14986_d_n11, assign12680_body10_e14986_d_n12, assign12680_body10_e14986_d_n17,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard339 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12680_body10_e14978: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign12680_body10_e14979: f64 = (assign12680_body10_e14978).sqrt();
        let assign12680_body10_e14980: f64 = (-assign12680_body10_e14979);
        let assign12680_body10_e14982: f64 = (assign12680_body10_e14980 * locals.var_beta);
        let assign12680_body10_e14984: f64 = (assign12680_body10_e14982 * locals.var_phi_sl_bulk);
        (assign12680_body10_e14984, (assign12680_body10_e14982 * locals.var_phi_sl_bulk_dn0), (assign12680_body10_e14982 * locals.var_phi_sl_bulk_dn2), (assign12680_body10_e14982 * locals.var_phi_sl_bulk_dn6), (assign12680_body10_e14982 * locals.var_phi_sl_bulk_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign12680_body10_e14979))) * locals.var_beta) + (assign12680_body10_e14980 * locals.var_beta_dn10)) * locals.var_phi_sl_bulk) + (assign12680_body10_e14982 * locals.var_phi_sl_bulk_dn10)), (assign12680_body10_e14982 * locals.var_phi_sl_bulk_dn11), (assign12680_body10_e14982 * locals.var_phi_sl_bulk_dn12), (assign12680_body10_e14982 * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t4__blk333, locals.var_t4__blk333_dn0, locals.var_t4__blk333_dn2, locals.var_t4__blk333_dn6, locals.var_t4__blk333_dn7, locals.var_t4__blk333_dn10, locals.var_t4__blk333_dn11, locals.var_t4__blk333_dn12, locals.var_t4__blk333_dn17,)
    }
};
            locals.var_t4__blk333 = assign12680_body10_e14986;
            locals.var_t4__blk333_dn0 = assign12680_body10_e14986_d_n0;
            locals.var_t4__blk333_dn2 = assign12680_body10_e14986_d_n2;
            locals.var_t4__blk333_dn6 = assign12680_body10_e14986_d_n6;
            locals.var_t4__blk333_dn7 = assign12680_body10_e14986_d_n7;
            locals.var_t4__blk333_dn10 = assign12680_body10_e14986_d_n10;
            locals.var_t4__blk333_dn11 = assign12680_body10_e14986_d_n11;
            locals.var_t4__blk333_dn12 = assign12680_body10_e14986_d_n12;
            locals.var_t4__blk333_dn17 = assign12680_body10_e14986_d_n17;
            let (assign12680_body11_e15011, assign12680_body11_e15011_d_n0, assign12680_body11_e15011_d_n2, assign12680_body11_e15011_d_n6, assign12680_body11_e15011_d_n7, assign12680_body11_e15011_d_n10, assign12680_body11_e15011_d_n11, assign12680_body11_e15011_d_n12, assign12680_body11_e15011_d_n17,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard339 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12680_body11_e15007: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign12680_body11_e15008: f64 = (assign12680_body11_e15007).sqrt();
        let assign12680_body11_e15009: f64 = (-assign12680_body11_e15008);
        (assign12680_body11_e15009, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign12680_body11_e15008))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk334, locals.var_t5__blk334_dn0, locals.var_t5__blk334_dn2, locals.var_t5__blk334_dn6, locals.var_t5__blk334_dn7, locals.var_t5__blk334_dn10, locals.var_t5__blk334_dn11, locals.var_t5__blk334_dn12, locals.var_t5__blk334_dn17,)
    }
};
            locals.var_t5__blk334 = assign12680_body11_e15011;
            locals.var_t5__blk334_dn0 = assign12680_body11_e15011_d_n0;
            locals.var_t5__blk334_dn2 = assign12680_body11_e15011_d_n2;
            locals.var_t5__blk334_dn6 = assign12680_body11_e15011_d_n6;
            locals.var_t5__blk334_dn7 = assign12680_body11_e15011_d_n7;
            locals.var_t5__blk334_dn10 = assign12680_body11_e15011_d_n10;
            locals.var_t5__blk334_dn11 = assign12680_body11_e15011_d_n11;
            locals.var_t5__blk334_dn12 = assign12680_body11_e15011_d_n12;
            locals.var_t5__blk334_dn17 = assign12680_body11_e15011_d_n17;
            let (assign12680_body12_e15035, assign12680_body12_e15035_d_n0, assign12680_body12_e15035_d_n2, assign12680_body12_e15035_d_n6, assign12680_body12_e15035_d_n7, assign12680_body12_e15035_d_n10, assign12680_body12_e15035_d_n11, assign12680_body12_e15035_d_n12, assign12680_body12_e15035_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body12_e15026: f64 = (locals.var_t4__blk333 * locals.var_t4__blk333);
        let assign12680_body12_e15029: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign12680_body12_e15031: f64 = (assign12680_body12_e15029 * locals.var_q_fd_dlt1);
        let assign12680_body12_e15032: f64 = (assign12680_body12_e15026 + assign12680_body12_e15031);
        let assign12680_body12_e15033: f64 = (assign12680_body12_e15032).sqrt();
        (assign12680_body12_e15033, ((((locals.var_t4__blk333_dn0 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign12680_body12_e15029 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign12680_body12_e15033)), ((((locals.var_t4__blk333_dn2 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign12680_body12_e15029 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign12680_body12_e15033)), ((((locals.var_t4__blk333_dn6 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign12680_body12_e15029 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign12680_body12_e15033)), ((((locals.var_t4__blk333_dn7 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign12680_body12_e15029 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign12680_body12_e15033)), ((((locals.var_t4__blk333_dn10 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign12680_body12_e15029 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign12680_body12_e15033)), ((((locals.var_t4__blk333_dn11 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign12680_body12_e15029 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign12680_body12_e15033)), ((((locals.var_t4__blk333_dn12 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign12680_body12_e15029 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign12680_body12_e15033)), ((((locals.var_t4__blk333_dn17 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign12680_body12_e15029 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign12680_body12_e15033)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12680_body12_e15035;
            locals.var_tmf2_dn0 = assign12680_body12_e15035_d_n0;
            locals.var_tmf2_dn2 = assign12680_body12_e15035_d_n2;
            locals.var_tmf2_dn6 = assign12680_body12_e15035_d_n6;
            locals.var_tmf2_dn7 = assign12680_body12_e15035_d_n7;
            locals.var_tmf2_dn10 = assign12680_body12_e15035_d_n10;
            locals.var_tmf2_dn11 = assign12680_body12_e15035_d_n11;
            locals.var_tmf2_dn12 = assign12680_body12_e15035_d_n12;
            locals.var_tmf2_dn17 = assign12680_body12_e15035_d_n17;
            let (assign12680_body13_e15056, assign12680_body13_e15056_d_n0, assign12680_body13_e15056_d_n2, assign12680_body13_e15056_d_n6, assign12680_body13_e15056_d_n7, assign12680_body13_e15056_d_n10, assign12680_body13_e15056_d_n11, assign12680_body13_e15056_d_n12, assign12680_body13_e15056_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body13_e15052: f64 = (locals.var_t4__blk333 / locals.var_tmf2);
        let assign12680_body13_e15053: f64 = (1.0 + assign12680_body13_e15052);
        let assign12680_body13_e15054: f64 = (0.5 * assign12680_body13_e15053);
        (assign12680_body13_e15054, (0.5 * (((locals.var_t4__blk333_dn0 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn2 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn6 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn7 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn10 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn11 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn12 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn17 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7__blk336, locals.var_t7__blk336_dn0, locals.var_t7__blk336_dn2, locals.var_t7__blk336_dn6, locals.var_t7__blk336_dn7, locals.var_t7__blk336_dn10, locals.var_t7__blk336_dn11, locals.var_t7__blk336_dn12, locals.var_t7__blk336_dn17,)
    }
};
            locals.var_t7__blk336 = assign12680_body13_e15056;
            locals.var_t7__blk336_dn0 = assign12680_body13_e15056_d_n0;
            locals.var_t7__blk336_dn2 = assign12680_body13_e15056_d_n2;
            locals.var_t7__blk336_dn6 = assign12680_body13_e15056_d_n6;
            locals.var_t7__blk336_dn7 = assign12680_body13_e15056_d_n7;
            locals.var_t7__blk336_dn10 = assign12680_body13_e15056_d_n10;
            locals.var_t7__blk336_dn11 = assign12680_body13_e15056_d_n11;
            locals.var_t7__blk336_dn12 = assign12680_body13_e15056_d_n12;
            locals.var_t7__blk336_dn17 = assign12680_body13_e15056_d_n17;
            let (assign12680_body14_e15079, assign12680_body14_e15079_d_n0, assign12680_body14_e15079_d_n2, assign12680_body14_e15079_d_n6, assign12680_body14_e15079_d_n7, assign12680_body14_e15079_d_n10, assign12680_body14_e15079_d_n11, assign12680_body14_e15079_d_n12, assign12680_body14_e15079_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body14_e15072: f64 = (locals.var_t4__blk333 + locals.var_tmf2);
        let assign12680_body14_e15073: f64 = (0.5 * assign12680_body14_e15072);
        let assign12680_body14_e15076: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign12680_body14_e15077: f64 = (assign12680_body14_e15073 + assign12680_body14_e15076);
        (assign12680_body14_e15077, ((0.5 * (locals.var_t4__blk333_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4__blk333_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4__blk333_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4__blk333_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4__blk333_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4__blk333_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4__blk333_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4__blk333_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    }
};
            locals.var_t6__blk335 = assign12680_body14_e15079;
            locals.var_t6__blk335_dn0 = assign12680_body14_e15079_d_n0;
            locals.var_t6__blk335_dn2 = assign12680_body14_e15079_d_n2;
            locals.var_t6__blk335_dn6 = assign12680_body14_e15079_d_n6;
            locals.var_t6__blk335_dn7 = assign12680_body14_e15079_d_n7;
            locals.var_t6__blk335_dn10 = assign12680_body14_e15079_d_n10;
            locals.var_t6__blk335_dn11 = assign12680_body14_e15079_d_n11;
            locals.var_t6__blk335_dn12 = assign12680_body14_e15079_d_n12;
            locals.var_t6__blk335_dn17 = assign12680_body14_e15079_d_n17;
            let assign12680_body15_e15082: f64 = if locals.var_t6__blk335 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard341 = assign12680_body15_e15082;
            let (assign12680_body16_e15099, assign12680_body16_e15099_d_n0, assign12680_body16_e15099_d_n2, assign12680_body16_e15099_d_n6, assign12680_body16_e15099_d_n7, assign12680_body16_e15099_d_n10, assign12680_body16_e15099_d_n11, assign12680_body16_e15099_d_n12, assign12680_body16_e15099_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard341 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    }
};
            locals.var_t6__blk335 = assign12680_body16_e15099;
            locals.var_t6__blk335_dn0 = assign12680_body16_e15099_d_n0;
            locals.var_t6__blk335_dn2 = assign12680_body16_e15099_d_n2;
            locals.var_t6__blk335_dn6 = assign12680_body16_e15099_d_n6;
            locals.var_t6__blk335_dn7 = assign12680_body16_e15099_d_n7;
            locals.var_t6__blk335_dn10 = assign12680_body16_e15099_d_n10;
            locals.var_t6__blk335_dn11 = assign12680_body16_e15099_d_n11;
            locals.var_t6__blk335_dn12 = assign12680_body16_e15099_d_n12;
            locals.var_t6__blk335_dn17 = assign12680_body16_e15099_d_n17;
            let (assign12680_body17_e15116, assign12680_body17_e15116_d_n0, assign12680_body17_e15116_d_n2, assign12680_body17_e15116_d_n6, assign12680_body17_e15116_d_n7, assign12680_body17_e15116_d_n10, assign12680_body17_e15116_d_n11, assign12680_body17_e15116_d_n12, assign12680_body17_e15116_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard341 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk336, locals.var_t7__blk336_dn0, locals.var_t7__blk336_dn2, locals.var_t7__blk336_dn6, locals.var_t7__blk336_dn7, locals.var_t7__blk336_dn10, locals.var_t7__blk336_dn11, locals.var_t7__blk336_dn12, locals.var_t7__blk336_dn17,)
    }
};
            locals.var_t7__blk336 = assign12680_body17_e15116;
            locals.var_t7__blk336_dn0 = assign12680_body17_e15116_d_n0;
            locals.var_t7__blk336_dn2 = assign12680_body17_e15116_d_n2;
            locals.var_t7__blk336_dn6 = assign12680_body17_e15116_d_n6;
            locals.var_t7__blk336_dn7 = assign12680_body17_e15116_d_n7;
            locals.var_t7__blk336_dn10 = assign12680_body17_e15116_d_n10;
            locals.var_t7__blk336_dn11 = assign12680_body17_e15116_d_n11;
            locals.var_t7__blk336_dn12 = assign12680_body17_e15116_d_n12;
            locals.var_t7__blk336_dn17 = assign12680_body17_e15116_d_n17;
            let (assign12680_body18_e15136, assign12680_body18_e15136_d_n0, assign12680_body18_e15136_d_n2, assign12680_body18_e15136_d_n6, assign12680_body18_e15136_d_n7, assign12680_body18_e15136_d_n10, assign12680_body18_e15136_d_n11, assign12680_body18_e15136_d_n12, assign12680_body18_e15136_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body18_e15130: f64 = (-locals.var_q_fd_soi);
        let assign12680_body18_e15132: f64 = (assign12680_body18_e15130 - locals.var_t6__blk335);
        let assign12680_body18_e15134: f64 = (assign12680_body18_e15132 - locals.var_q_fd_dlt2);
        (assign12680_body18_e15134, (((-locals.var_q_fd_soi_dn0) - locals.var_t6__blk335_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6__blk335_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6__blk335_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6__blk335_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6__blk335_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6__blk335_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6__blk335_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6__blk335_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12680_body18_e15136;
            locals.var_tmf1_dn0 = assign12680_body18_e15136_d_n0;
            locals.var_tmf1_dn2 = assign12680_body18_e15136_d_n2;
            locals.var_tmf1_dn6 = assign12680_body18_e15136_d_n6;
            locals.var_tmf1_dn7 = assign12680_body18_e15136_d_n7;
            locals.var_tmf1_dn10 = assign12680_body18_e15136_d_n10;
            locals.var_tmf1_dn11 = assign12680_body18_e15136_d_n11;
            locals.var_tmf1_dn12 = assign12680_body18_e15136_d_n12;
            locals.var_tmf1_dn17 = assign12680_body18_e15136_d_n17;
            let (assign12680_body19_e15156, assign12680_body19_e15156_d_n0, assign12680_body19_e15156_d_n2, assign12680_body19_e15156_d_n6, assign12680_body19_e15156_d_n7, assign12680_body19_e15156_d_n10, assign12680_body19_e15156_d_n11, assign12680_body19_e15156_d_n12, assign12680_body19_e15156_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body19_e15151: f64 = (-locals.var_q_fd_soi);
        let assign12680_body19_e15152: f64 = (4.0 * assign12680_body19_e15151);
        let assign12680_body19_e15154: f64 = (assign12680_body19_e15152 * locals.var_q_fd_dlt2);
        (assign12680_body19_e15154, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign12680_body19_e15152 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign12680_body19_e15152 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign12680_body19_e15152 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign12680_body19_e15152 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign12680_body19_e15152 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign12680_body19_e15152 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign12680_body19_e15152 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign12680_body19_e15152 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12680_body19_e15156;
            locals.var_tmf2_dn0 = assign12680_body19_e15156_d_n0;
            locals.var_tmf2_dn2 = assign12680_body19_e15156_d_n2;
            locals.var_tmf2_dn6 = assign12680_body19_e15156_d_n6;
            locals.var_tmf2_dn7 = assign12680_body19_e15156_d_n7;
            locals.var_tmf2_dn10 = assign12680_body19_e15156_d_n10;
            locals.var_tmf2_dn11 = assign12680_body19_e15156_d_n11;
            locals.var_tmf2_dn12 = assign12680_body19_e15156_d_n12;
            locals.var_tmf2_dn17 = assign12680_body19_e15156_d_n17;
            let (assign12680_body20_e15177, assign12680_body20_e15177_d_n0, assign12680_body20_e15177_d_n2, assign12680_body20_e15177_d_n6, assign12680_body20_e15177_d_n7, assign12680_body20_e15177_d_n10, assign12680_body20_e15177_d_n11, assign12680_body20_e15177_d_n12, assign12680_body20_e15177_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let (assign12680_body20_e15175, assign12680_body20_e15175_d_n0, assign12680_body20_e15175_d_n2, assign12680_body20_e15175_d_n6, assign12680_body20_e15175_d_n7, assign12680_body20_e15175_d_n10, assign12680_body20_e15175_d_n11, assign12680_body20_e15175_d_n12, assign12680_body20_e15175_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12680_body20_e15174: f64 = (-locals.var_tmf2);
                (assign12680_body20_e15174, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12680_body20_e15175, assign12680_body20_e15175_d_n0, assign12680_body20_e15175_d_n2, assign12680_body20_e15175_d_n6, assign12680_body20_e15175_d_n7, assign12680_body20_e15175_d_n10, assign12680_body20_e15175_d_n11, assign12680_body20_e15175_d_n12, assign12680_body20_e15175_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12680_body20_e15177;
            locals.var_tmf2_dn0 = assign12680_body20_e15177_d_n0;
            locals.var_tmf2_dn2 = assign12680_body20_e15177_d_n2;
            locals.var_tmf2_dn6 = assign12680_body20_e15177_d_n6;
            locals.var_tmf2_dn7 = assign12680_body20_e15177_d_n7;
            locals.var_tmf2_dn10 = assign12680_body20_e15177_d_n10;
            locals.var_tmf2_dn11 = assign12680_body20_e15177_d_n11;
            locals.var_tmf2_dn12 = assign12680_body20_e15177_d_n12;
            locals.var_tmf2_dn17 = assign12680_body20_e15177_d_n17;
            let (assign12680_body21_e15197, assign12680_body21_e15197_d_n0, assign12680_body21_e15197_d_n2, assign12680_body21_e15197_d_n6, assign12680_body21_e15197_d_n7, assign12680_body21_e15197_d_n10, assign12680_body21_e15197_d_n11, assign12680_body21_e15197_d_n12, assign12680_body21_e15197_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body21_e15192: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12680_body21_e15194: f64 = (assign12680_body21_e15192 + locals.var_tmf2);
        let assign12680_body21_e15195: f64 = (assign12680_body21_e15194).sqrt();
        (assign12680_body21_e15195, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12680_body21_e15195)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12680_body21_e15195)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12680_body21_e15195)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12680_body21_e15195)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12680_body21_e15195)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12680_body21_e15195)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12680_body21_e15195)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12680_body21_e15195)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12680_body21_e15197;
            locals.var_tmf2_dn0 = assign12680_body21_e15197_d_n0;
            locals.var_tmf2_dn2 = assign12680_body21_e15197_d_n2;
            locals.var_tmf2_dn6 = assign12680_body21_e15197_d_n6;
            locals.var_tmf2_dn7 = assign12680_body21_e15197_d_n7;
            locals.var_tmf2_dn10 = assign12680_body21_e15197_d_n10;
            locals.var_tmf2_dn11 = assign12680_body21_e15197_d_n11;
            locals.var_tmf2_dn12 = assign12680_body21_e15197_d_n12;
            locals.var_tmf2_dn17 = assign12680_body21_e15197_d_n17;
            let (assign12680_body22_e15218, assign12680_body22_e15218_d_n0, assign12680_body22_e15218_d_n2, assign12680_body22_e15218_d_n6, assign12680_body22_e15218_d_n7, assign12680_body22_e15218_d_n10, assign12680_body22_e15218_d_n11, assign12680_body22_e15218_d_n12, assign12680_body22_e15218_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body22_e15214: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign12680_body22_e15215: f64 = (1.0 + assign12680_body22_e15214);
        let assign12680_body22_e15216: f64 = (0.5 * assign12680_body22_e15215);
        (assign12680_body22_e15216, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign12680_body22_e15218;
            locals.var_t8_dn0 = assign12680_body22_e15218_d_n0;
            locals.var_t8_dn2 = assign12680_body22_e15218_d_n2;
            locals.var_t8_dn6 = assign12680_body22_e15218_d_n6;
            locals.var_t8_dn7 = assign12680_body22_e15218_d_n7;
            locals.var_t8_dn10 = assign12680_body22_e15218_d_n10;
            locals.var_t8_dn11 = assign12680_body22_e15218_d_n11;
            locals.var_t8_dn12 = assign12680_body22_e15218_d_n12;
            locals.var_t8_dn17 = assign12680_body22_e15218_d_n17;
            let (assign12680_body23_e15240, assign12680_body23_e15240_d_n0, assign12680_body23_e15240_d_n2, assign12680_body23_e15240_d_n6, assign12680_body23_e15240_d_n7, assign12680_body23_e15240_d_n10, assign12680_body23_e15240_d_n11, assign12680_body23_e15240_d_n12, assign12680_body23_e15240_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body23_e15232: f64 = (-locals.var_q_fd_soi);
        let assign12680_body23_e15236: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12680_body23_e15237: f64 = (0.5 * assign12680_body23_e15236);
        let assign12680_body23_e15238: f64 = (assign12680_body23_e15232 - assign12680_body23_e15237);
        (assign12680_body23_e15238, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    }
};
            locals.var_t6__blk335 = assign12680_body23_e15240;
            locals.var_t6__blk335_dn0 = assign12680_body23_e15240_d_n0;
            locals.var_t6__blk335_dn2 = assign12680_body23_e15240_d_n2;
            locals.var_t6__blk335_dn6 = assign12680_body23_e15240_d_n6;
            locals.var_t6__blk335_dn7 = assign12680_body23_e15240_d_n7;
            locals.var_t6__blk335_dn10 = assign12680_body23_e15240_d_n10;
            locals.var_t6__blk335_dn11 = assign12680_body23_e15240_d_n11;
            locals.var_t6__blk335_dn12 = assign12680_body23_e15240_d_n12;
            locals.var_t6__blk335_dn17 = assign12680_body23_e15240_d_n17;
            let (assign12680_body24_e15259, assign12680_body24_e15259_d_n0, assign12680_body24_e15259_d_n2, assign12680_body24_e15259_d_n6, assign12680_body24_e15259_d_n7, assign12680_body24_e15259_d_n10, assign12680_body24_e15259_d_n11, assign12680_body24_e15259_d_n12, assign12680_body24_e15259_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body24_e15256: f64 = (locals.var_t5__blk334 * locals.var_t8);
        let assign12680_body24_e15257: f64 = (locals.var_t7__blk336 * assign12680_body24_e15256);
        (assign12680_body24_e15257, ((locals.var_t7__blk336_dn0 * assign12680_body24_e15256) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn0 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn0)))), ((locals.var_t7__blk336_dn2 * assign12680_body24_e15256) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn2 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn2)))), ((locals.var_t7__blk336_dn6 * assign12680_body24_e15256) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn6 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn6)))), ((locals.var_t7__blk336_dn7 * assign12680_body24_e15256) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn7 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn7)))), ((locals.var_t7__blk336_dn10 * assign12680_body24_e15256) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn10 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn10)))), ((locals.var_t7__blk336_dn11 * assign12680_body24_e15256) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn11 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn11)))), ((locals.var_t7__blk336_dn12 * assign12680_body24_e15256) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn12 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn12)))), ((locals.var_t7__blk336_dn17 * assign12680_body24_e15256) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn17 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7__blk336, locals.var_t7__blk336_dn0, locals.var_t7__blk336_dn2, locals.var_t7__blk336_dn6, locals.var_t7__blk336_dn7, locals.var_t7__blk336_dn10, locals.var_t7__blk336_dn11, locals.var_t7__blk336_dn12, locals.var_t7__blk336_dn17,)
    }
};
            locals.var_t7__blk336 = assign12680_body24_e15259;
            locals.var_t7__blk336_dn0 = assign12680_body24_e15259_d_n0;
            locals.var_t7__blk336_dn2 = assign12680_body24_e15259_d_n2;
            locals.var_t7__blk336_dn6 = assign12680_body24_e15259_d_n6;
            locals.var_t7__blk336_dn7 = assign12680_body24_e15259_d_n7;
            locals.var_t7__blk336_dn10 = assign12680_body24_e15259_d_n10;
            locals.var_t7__blk336_dn11 = assign12680_body24_e15259_d_n11;
            locals.var_t7__blk336_dn12 = assign12680_body24_e15259_d_n12;
            locals.var_t7__blk336_dn17 = assign12680_body24_e15259_d_n17;
            let (assign12680_body25_e15284, assign12680_body25_e15284_d_n0, assign12680_body25_e15284_d_n2, assign12680_body25_e15284_d_n6, assign12680_body25_e15284_d_n7, assign12680_body25_e15284_d_n10, assign12680_body25_e15284_d_n11, assign12680_body25_e15284_d_n12, assign12680_body25_e15284_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body25_e15274: f64 = (locals.var_t6__blk335 * locals.var_t6__blk335);
        let assign12680_body25_e15276: f64 = (assign12680_body25_e15274 / 2.0);
        let assign12680_body25_e15278: f64 = (assign12680_body25_e15276 / 1.034943e-10);
        let assign12680_body25_e15280: f64 = (assign12680_body25_e15278 / 1.6021918e-19);
        let assign12680_body25_e15282: f64 = (assign12680_body25_e15280 / locals.var_uc_nsubs);
        (assign12680_body25_e15282, ((((((((locals.var_t6__blk335_dn0 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12680_body25_e15280 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn2 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12680_body25_e15280 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn6 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12680_body25_e15280 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn7 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12680_body25_e15280 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn10 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12680_body25_e15280 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn11 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12680_body25_e15280 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn12 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12680_body25_e15280 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn17 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12680_body25_e15280 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
            locals.var_phi_b_dep = assign12680_body25_e15284;
            locals.var_phi_b_dep_dn0 = assign12680_body25_e15284_d_n0;
            locals.var_phi_b_dep_dn2 = assign12680_body25_e15284_d_n2;
            locals.var_phi_b_dep_dn6 = assign12680_body25_e15284_d_n6;
            locals.var_phi_b_dep_dn7 = assign12680_body25_e15284_d_n7;
            locals.var_phi_b_dep_dn10 = assign12680_body25_e15284_d_n10;
            locals.var_phi_b_dep_dn11 = assign12680_body25_e15284_d_n11;
            locals.var_phi_b_dep_dn12 = assign12680_body25_e15284_d_n12;
            locals.var_phi_b_dep_dn17 = assign12680_body25_e15284_d_n17;
            let (assign12680_body26_e15305, assign12680_body26_e15305_d_n0, assign12680_body26_e15305_d_n2, assign12680_body26_e15305_d_n6, assign12680_body26_e15305_d_n7, assign12680_body26_e15305_d_n10, assign12680_body26_e15305_d_n11, assign12680_body26_e15305_d_n12, assign12680_body26_e15305_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body26_e15299: f64 = (2.0 * locals.var_phi_b_dep);
        let assign12680_body26_e15301: f64 = (assign12680_body26_e15299 * locals.var_t7__blk336);
        let assign12680_body26_e15303: f64 = (assign12680_body26_e15301 / locals.var_t6__blk335);
        (assign12680_body26_e15303, ((((((2.0 * locals.var_phi_b_dep_dn0) * locals.var_t7__blk336) + (assign12680_body26_e15299 * locals.var_t7__blk336_dn0)) * locals.var_t6__blk335) - (assign12680_body26_e15301 * locals.var_t6__blk335_dn0)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn2) * locals.var_t7__blk336) + (assign12680_body26_e15299 * locals.var_t7__blk336_dn2)) * locals.var_t6__blk335) - (assign12680_body26_e15301 * locals.var_t6__blk335_dn2)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn6) * locals.var_t7__blk336) + (assign12680_body26_e15299 * locals.var_t7__blk336_dn6)) * locals.var_t6__blk335) - (assign12680_body26_e15301 * locals.var_t6__blk335_dn6)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn7) * locals.var_t7__blk336) + (assign12680_body26_e15299 * locals.var_t7__blk336_dn7)) * locals.var_t6__blk335) - (assign12680_body26_e15301 * locals.var_t6__blk335_dn7)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn10) * locals.var_t7__blk336) + (assign12680_body26_e15299 * locals.var_t7__blk336_dn10)) * locals.var_t6__blk335) - (assign12680_body26_e15301 * locals.var_t6__blk335_dn10)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn11) * locals.var_t7__blk336) + (assign12680_body26_e15299 * locals.var_t7__blk336_dn11)) * locals.var_t6__blk335) - (assign12680_body26_e15301 * locals.var_t6__blk335_dn11)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn12) * locals.var_t7__blk336) + (assign12680_body26_e15299 * locals.var_t7__blk336_dn12)) * locals.var_t6__blk335) - (assign12680_body26_e15301 * locals.var_t6__blk335_dn12)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn17) * locals.var_t7__blk336) + (assign12680_body26_e15299 * locals.var_t7__blk336_dn17)) * locals.var_t6__blk335) - (assign12680_body26_e15301 * locals.var_t6__blk335_dn17)) / (locals.var_t6__blk335 * locals.var_t6__blk335)),)
    } else {
        (locals.var_phi_b_dep_dpsb, locals.var_phi_b_dep_dpsb_dn0, locals.var_phi_b_dep_dpsb_dn2, locals.var_phi_b_dep_dpsb_dn6, locals.var_phi_b_dep_dpsb_dn7, locals.var_phi_b_dep_dpsb_dn10, locals.var_phi_b_dep_dpsb_dn11, locals.var_phi_b_dep_dpsb_dn12, locals.var_phi_b_dep_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep_dpsb = assign12680_body26_e15305;
            locals.var_phi_b_dep_dpsb_dn0 = assign12680_body26_e15305_d_n0;
            locals.var_phi_b_dep_dpsb_dn2 = assign12680_body26_e15305_d_n2;
            locals.var_phi_b_dep_dpsb_dn6 = assign12680_body26_e15305_d_n6;
            locals.var_phi_b_dep_dpsb_dn7 = assign12680_body26_e15305_d_n7;
            locals.var_phi_b_dep_dpsb_dn10 = assign12680_body26_e15305_d_n10;
            locals.var_phi_b_dep_dpsb_dn11 = assign12680_body26_e15305_d_n11;
            locals.var_phi_b_dep_dpsb_dn12 = assign12680_body26_e15305_d_n12;
            locals.var_phi_b_dep_dpsb_dn17 = assign12680_body26_e15305_d_n17;
            let (assign12680_body27_e15340, assign12680_body27_e15340_d_n0, assign12680_body27_e15340_d_n2, assign12680_body27_e15340_d_n6, assign12680_body27_e15340_d_n7, assign12680_body27_e15340_d_n10, assign12680_body27_e15340_d_n11, assign12680_body27_e15340_d_n12, assign12680_body27_e15340_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body27_e15320: f64 = (-locals.var_phi_sl_bulk);
        let assign12680_body27_e15323: f64 = (locals.var_t4__blk333 / locals.var_c_box);
        let assign12680_body27_e15324: f64 = (assign12680_body27_e15320 + assign12680_body27_e15323);
        let assign12680_body27_e15326: f64 = (assign12680_body27_e15324 - locals.var_vbsbiz);
        let assign12680_body27_e15328: f64 = (assign12680_body27_e15326 + locals.var_phi_b_dep);
        let assign12680_body27_e15330: f64 = (-1.0);
        let assign12680_body27_e15333: f64 = (locals.var_t5__blk334 / locals.var_c_box);
        let assign12680_body27_e15334: f64 = (assign12680_body27_e15330 + assign12680_body27_e15333);
        let assign12680_body27_e15336: f64 = (assign12680_body27_e15334 + locals.var_phi_b_dep_dpsb);
        let assign12680_body27_e15337: f64 = (assign12680_body27_e15328 / assign12680_body27_e15336);
        let assign12680_body27_e15338: f64 = (locals.var_phi_sl_bulk - assign12680_body27_e15337);
        (assign12680_body27_e15338, (locals.var_phi_sl_bulk_dn0 - (((((((-locals.var_phi_sl_bulk_dn0) + (locals.var_t4__blk333_dn0 / locals.var_c_box)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep_dn0) * assign12680_body27_e15336) - (assign12680_body27_e15328 * ((locals.var_t5__blk334_dn0 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn0))) / (assign12680_body27_e15336 * assign12680_body27_e15336))), (locals.var_phi_sl_bulk_dn2 - (((((((-locals.var_phi_sl_bulk_dn2) + (locals.var_t4__blk333_dn2 / locals.var_c_box)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep_dn2) * assign12680_body27_e15336) - (assign12680_body27_e15328 * ((locals.var_t5__blk334_dn2 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn2))) / (assign12680_body27_e15336 * assign12680_body27_e15336))), (locals.var_phi_sl_bulk_dn6 - (((((((-locals.var_phi_sl_bulk_dn6) + (locals.var_t4__blk333_dn6 / locals.var_c_box)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep_dn6) * assign12680_body27_e15336) - (assign12680_body27_e15328 * ((locals.var_t5__blk334_dn6 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn6))) / (assign12680_body27_e15336 * assign12680_body27_e15336))), (locals.var_phi_sl_bulk_dn7 - (((((((-locals.var_phi_sl_bulk_dn7) + (locals.var_t4__blk333_dn7 / locals.var_c_box)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep_dn7) * assign12680_body27_e15336) - (assign12680_body27_e15328 * ((locals.var_t5__blk334_dn7 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn7))) / (assign12680_body27_e15336 * assign12680_body27_e15336))), (locals.var_phi_sl_bulk_dn10 - (((((((-locals.var_phi_sl_bulk_dn10) + (locals.var_t4__blk333_dn10 / locals.var_c_box)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep_dn10) * assign12680_body27_e15336) - (assign12680_body27_e15328 * ((locals.var_t5__blk334_dn10 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn10))) / (assign12680_body27_e15336 * assign12680_body27_e15336))), (locals.var_phi_sl_bulk_dn11 - (((((((-locals.var_phi_sl_bulk_dn11) + (locals.var_t4__blk333_dn11 / locals.var_c_box)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep_dn11) * assign12680_body27_e15336) - (assign12680_body27_e15328 * ((locals.var_t5__blk334_dn11 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn11))) / (assign12680_body27_e15336 * assign12680_body27_e15336))), (locals.var_phi_sl_bulk_dn12 - (((((((-locals.var_phi_sl_bulk_dn12) + (locals.var_t4__blk333_dn12 / locals.var_c_box)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep_dn12) * assign12680_body27_e15336) - (assign12680_body27_e15328 * ((locals.var_t5__blk334_dn12 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn12))) / (assign12680_body27_e15336 * assign12680_body27_e15336))), (locals.var_phi_sl_bulk_dn17 - (((((((-locals.var_phi_sl_bulk_dn17) + (locals.var_t4__blk333_dn17 / locals.var_c_box)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep_dn17) * assign12680_body27_e15336) - (assign12680_body27_e15328 * ((locals.var_t5__blk334_dn17 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn17))) / (assign12680_body27_e15336 * assign12680_body27_e15336))),)
    } else {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    }
};
            locals.var_t6__blk335 = assign12680_body27_e15340;
            locals.var_t6__blk335_dn0 = assign12680_body27_e15340_d_n0;
            locals.var_t6__blk335_dn2 = assign12680_body27_e15340_d_n2;
            locals.var_t6__blk335_dn6 = assign12680_body27_e15340_d_n6;
            locals.var_t6__blk335_dn7 = assign12680_body27_e15340_d_n7;
            locals.var_t6__blk335_dn10 = assign12680_body27_e15340_d_n10;
            locals.var_t6__blk335_dn11 = assign12680_body27_e15340_d_n11;
            locals.var_t6__blk335_dn12 = assign12680_body27_e15340_d_n12;
            locals.var_t6__blk335_dn17 = assign12680_body27_e15340_d_n17;
            let assign12680_body28_e15343: f64 = (locals.var_t6__blk335 - locals.var_phi_sl_bulk);
            let assign12680_body28_e15344: f64 = (assign12680_body28_e15343).abs();
            let assign12680_body28_e15346: f64 = if assign12680_body28_e15344 < 5e-12 { 1.0 } else { 0.0 };
            locals.var_guard342 = assign12680_body28_e15346;
            let (assign12680_body29_e15363,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) && (locals.var_guard342 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12680_body29_e15363;
            let (assign12680_body30_e15378, assign12680_body30_e15378_d_n0, assign12680_body30_e15378_d_n2, assign12680_body30_e15378_d_n6, assign12680_body30_e15378_d_n7, assign12680_body30_e15378_d_n10, assign12680_body30_e15378_d_n11, assign12680_body30_e15378_d_n12, assign12680_body30_e15378_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12680_body30_e15378;
            locals.var_phi_sl_bulk_dn0 = assign12680_body30_e15378_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12680_body30_e15378_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12680_body30_e15378_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12680_body30_e15378_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12680_body30_e15378_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12680_body30_e15378_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12680_body30_e15378_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12680_body30_e15378_d_n17;
            let (assign12680_body31_e15393, assign12680_body31_e15393_d_n0, assign12680_body31_e15393_d_n2, assign12680_body31_e15393_d_n6, assign12680_body31_e15393_d_n7, assign12680_body31_e15393_d_n10, assign12680_body31_e15393_d_n11, assign12680_body31_e15393_d_n12, assign12680_body31_e15393_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        (locals.var_t4__blk333, locals.var_t4__blk333_dn0, locals.var_t4__blk333_dn2, locals.var_t4__blk333_dn6, locals.var_t4__blk333_dn7, locals.var_t4__blk333_dn10, locals.var_t4__blk333_dn11, locals.var_t4__blk333_dn12, locals.var_t4__blk333_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12680_body31_e15393;
            locals.var_q_sl_bulk_dn0 = assign12680_body31_e15393_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12680_body31_e15393_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12680_body31_e15393_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12680_body31_e15393_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12680_body31_e15393_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12680_body31_e15393_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12680_body31_e15393_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12680_body31_e15393_d_n17;
            let (assign12680_body32_e15410,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12680_body32_e15408: f64 = (locals.var_lp_sl + 1.0);
        (assign12680_body32_e15408,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12680_body32_e15410;
        }

    }

    pub(super) fn stamp_transient_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign12690_e15427, assign12690_e15427_d_n0, assign12690_e15427_d_n2, assign12690_e15427_d_n6, assign12690_e15427_d_n7, assign12690_e15427_d_n10, assign12690_e15427_d_n11, assign12690_e15427_d_n12, assign12690_e15427_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12690_e15425: f64 = (locals.var_vbsbiz + locals.var_phi_sl_bulk);
        (assign12690_e15425, (locals.var_vbsbiz_dn0 + locals.var_phi_sl_bulk_dn0), (locals.var_vbsbiz_dn2 + locals.var_phi_sl_bulk_dn2), (locals.var_vbsbiz_dn6 + locals.var_phi_sl_bulk_dn6), (locals.var_vbsbiz_dn7 + locals.var_phi_sl_bulk_dn7), (locals.var_vbsbiz_dn10 + locals.var_phi_sl_bulk_dn10), (locals.var_vbsbiz_dn11 + locals.var_phi_sl_bulk_dn11), (locals.var_vbsbiz_dn12 + locals.var_phi_sl_bulk_dn12), (locals.var_vbsbiz_dn17 + locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12690_e15427;
        locals.var_phi_sl_bulk_dn0 = assign12690_e15427_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12690_e15427_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12690_e15427_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12690_e15427_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12690_e15427_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12690_e15427_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12690_e15427_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12690_e15427_d_n17;

        let (assign12700_e15446, assign12700_e15446_d_n0, assign12700_e15446_d_n2, assign12700_e15446_d_n6, assign12700_e15446_d_n7, assign12700_e15446_d_n10, assign12700_e15446_d_n11, assign12700_e15446_d_n12, assign12700_e15446_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 != 0.0)) {
        let assign12700_e15443: f64 = (locals.var_q_sl_bulk / locals.var_c_box);
        let assign12700_e15444: f64 = (locals.var_phi_sl_bulk - assign12700_e15443);
        (assign12700_e15444, (locals.var_phi_sl_bulk_dn0 - (locals.var_q_sl_bulk_dn0 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn2 - (locals.var_q_sl_bulk_dn2 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn6 - (locals.var_q_sl_bulk_dn6 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn7 - (locals.var_q_sl_bulk_dn7 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn10 - (locals.var_q_sl_bulk_dn10 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn11 - (locals.var_q_sl_bulk_dn11 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn12 - (locals.var_q_sl_bulk_dn12 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn17 - (locals.var_q_sl_bulk_dn17 / locals.var_c_box)),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12700_e15446;
        locals.var_phi_bl_soi_dn0 = assign12700_e15446_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12700_e15446_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12700_e15446_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12700_e15446_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12700_e15446_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12700_e15446_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12700_e15446_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12700_e15446_d_n17;

        let (assign12720_e15478,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12720_e15478;

    }

    pub(super) fn stamp_transient_block_39(
        locals: &mut StampLocals,
    ) {
        let mut assign12730_loop_guard: usize = 0;
        while {
            let assign12730_cond_e15495: f64 = if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_lp_sl < locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12730_cond_e15495 != 0.0
        } {
            assign12730_loop_guard += 1;
            assert!(assign12730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12730_body0_e15511, assign12730_body0_e15511_d_n10,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    } else {
        (locals.var_t1__blk330, locals.var_t1__blk330_dn10,)
    }
};
            locals.var_t1__blk330 = assign12730_body0_e15511;
            locals.var_t1__blk330_dn10 = assign12730_body0_e15511_d_n10;
            let (assign12730_body1_e15529, assign12730_body1_e15529_d_n0, assign12730_body1_e15529_d_n2, assign12730_body1_e15529_d_n6, assign12730_body1_e15529_d_n7, assign12730_body1_e15529_d_n10, assign12730_body1_e15529_d_n11, assign12730_body1_e15529_d_n12, assign12730_body1_e15529_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body1_e15527: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        (assign12730_body1_e15527, (locals.var_beta * locals.var_phi_sl_bulk_dn0), (locals.var_beta * locals.var_phi_sl_bulk_dn2), (locals.var_beta * locals.var_phi_sl_bulk_dn6), (locals.var_beta * locals.var_phi_sl_bulk_dn7), ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10)), (locals.var_beta * locals.var_phi_sl_bulk_dn11), (locals.var_beta * locals.var_phi_sl_bulk_dn12), (locals.var_beta * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t2__blk331, locals.var_t2__blk331_dn0, locals.var_t2__blk331_dn2, locals.var_t2__blk331_dn6, locals.var_t2__blk331_dn7, locals.var_t2__blk331_dn10, locals.var_t2__blk331_dn11, locals.var_t2__blk331_dn12, locals.var_t2__blk331_dn17,)
    }
};
            locals.var_t2__blk331 = assign12730_body1_e15529;
            locals.var_t2__blk331_dn0 = assign12730_body1_e15529_d_n0;
            locals.var_t2__blk331_dn2 = assign12730_body1_e15529_d_n2;
            locals.var_t2__blk331_dn6 = assign12730_body1_e15529_d_n6;
            locals.var_t2__blk331_dn7 = assign12730_body1_e15529_d_n7;
            locals.var_t2__blk331_dn10 = assign12730_body1_e15529_d_n10;
            locals.var_t2__blk331_dn11 = assign12730_body1_e15529_d_n11;
            locals.var_t2__blk331_dn12 = assign12730_body1_e15529_d_n12;
            locals.var_t2__blk331_dn17 = assign12730_body1_e15529_d_n17;
            let (assign12730_body2_e15547, assign12730_body2_e15547_d_n0, assign12730_body2_e15547_d_n2, assign12730_body2_e15547_d_n6, assign12730_body2_e15547_d_n7, assign12730_body2_e15547_d_n10, assign12730_body2_e15547_d_n11, assign12730_body2_e15547_d_n12, assign12730_body2_e15547_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body2_e15544: f64 = (-locals.var_t2__blk331);
        let assign12730_body2_e15545: f64 = (assign12730_body2_e15544).exp();
        (assign12730_body2_e15545, (assign12730_body2_e15545 * (-locals.var_t2__blk331_dn0)), (assign12730_body2_e15545 * (-locals.var_t2__blk331_dn2)), (assign12730_body2_e15545 * (-locals.var_t2__blk331_dn6)), (assign12730_body2_e15545 * (-locals.var_t2__blk331_dn7)), (assign12730_body2_e15545 * (-locals.var_t2__blk331_dn10)), (assign12730_body2_e15545 * (-locals.var_t2__blk331_dn11)), (assign12730_body2_e15545 * (-locals.var_t2__blk331_dn12)), (assign12730_body2_e15545 * (-locals.var_t2__blk331_dn17)),)
    } else {
        (locals.var_t3__blk332, locals.var_t3__blk332_dn0, locals.var_t3__blk332_dn2, locals.var_t3__blk332_dn6, locals.var_t3__blk332_dn7, locals.var_t3__blk332_dn10, locals.var_t3__blk332_dn11, locals.var_t3__blk332_dn12, locals.var_t3__blk332_dn17,)
    }
};
            locals.var_t3__blk332 = assign12730_body2_e15547;
            locals.var_t3__blk332_dn0 = assign12730_body2_e15547_d_n0;
            locals.var_t3__blk332_dn2 = assign12730_body2_e15547_d_n2;
            locals.var_t3__blk332_dn6 = assign12730_body2_e15547_d_n6;
            locals.var_t3__blk332_dn7 = assign12730_body2_e15547_d_n7;
            locals.var_t3__blk332_dn10 = assign12730_body2_e15547_d_n10;
            locals.var_t3__blk332_dn11 = assign12730_body2_e15547_d_n11;
            locals.var_t3__blk332_dn12 = assign12730_body2_e15547_d_n12;
            locals.var_t3__blk332_dn17 = assign12730_body2_e15547_d_n17;
            let assign12730_body3_e15550: f64 = if locals.var_phi_sl_bulk > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard343 = assign12730_body3_e15550;
            let (assign12730_body4_e15571, assign12730_body4_e15571_d_n0, assign12730_body4_e15571_d_n2, assign12730_body4_e15571_d_n6, assign12730_body4_e15571_d_n7, assign12730_body4_e15571_d_n10, assign12730_body4_e15571_d_n11, assign12730_body4_e15571_d_n12, assign12730_body4_e15571_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard343 != 0.0)) {
        let assign12730_body4_e15568: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        let assign12730_body4_e15569: f64 = (assign12730_body4_e15568).exp();
        (assign12730_body4_e15569, (assign12730_body4_e15569 * (locals.var_beta * locals.var_phi_sl_bulk_dn0)), (assign12730_body4_e15569 * (locals.var_beta * locals.var_phi_sl_bulk_dn2)), (assign12730_body4_e15569 * (locals.var_beta * locals.var_phi_sl_bulk_dn6)), (assign12730_body4_e15569 * (locals.var_beta * locals.var_phi_sl_bulk_dn7)), (assign12730_body4_e15569 * ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10))), (assign12730_body4_e15569 * (locals.var_beta * locals.var_phi_sl_bulk_dn11)), (assign12730_body4_e15569 * (locals.var_beta * locals.var_phi_sl_bulk_dn12)), (assign12730_body4_e15569 * (locals.var_beta * locals.var_phi_sl_bulk_dn17)),)
    } else {
        (locals.var_t0__blk329, locals.var_t0__blk329_dn0, locals.var_t0__blk329_dn2, locals.var_t0__blk329_dn6, locals.var_t0__blk329_dn7, locals.var_t0__blk329_dn10, locals.var_t0__blk329_dn11, locals.var_t0__blk329_dn12, locals.var_t0__blk329_dn17,)
    }
};
            locals.var_t0__blk329 = assign12730_body4_e15571;
            locals.var_t0__blk329_dn0 = assign12730_body4_e15571_d_n0;
            locals.var_t0__blk329_dn2 = assign12730_body4_e15571_d_n2;
            locals.var_t0__blk329_dn6 = assign12730_body4_e15571_d_n6;
            locals.var_t0__blk329_dn7 = assign12730_body4_e15571_d_n7;
            locals.var_t0__blk329_dn10 = assign12730_body4_e15571_d_n10;
            locals.var_t0__blk329_dn11 = assign12730_body4_e15571_d_n11;
            locals.var_t0__blk329_dn12 = assign12730_body4_e15571_d_n12;
            locals.var_t0__blk329_dn17 = assign12730_body4_e15571_d_n17;
            let (assign12730_body5_e15603, assign12730_body5_e15603_d_n0, assign12730_body5_e15603_d_n2, assign12730_body5_e15603_d_n6, assign12730_body5_e15603_d_n7, assign12730_body5_e15603_d_n10, assign12730_body5_e15603_d_n11, assign12730_body5_e15603_d_n12, assign12730_body5_e15603_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard343 != 0.0)) {
        let assign12730_body5_e15588: f64 = (-locals.var_t1__blk330);
        let assign12730_body5_e15591: f64 = (locals.var_t3__blk332 + locals.var_t2__blk331);
        let assign12730_body5_e15593: f64 = (assign12730_body5_e15591 - 1.0);
        let assign12730_body5_e15597: f64 = (locals.var_t0__blk329 - 1.0);
        let assign12730_body5_e15598: f64 = (locals.var_cnst1bulk * assign12730_body5_e15597);
        let assign12730_body5_e15599: f64 = (assign12730_body5_e15593 + assign12730_body5_e15598);
        let assign12730_body5_e15600: f64 = (assign12730_body5_e15599).sqrt();
        let assign12730_body5_e15601: f64 = (assign12730_body5_e15588 * assign12730_body5_e15600);
        (assign12730_body5_e15601, (assign12730_body5_e15588 * (((locals.var_t3__blk332_dn0 + locals.var_t2__blk331_dn0) + ((locals.var_cnst1bulk_dn0 * assign12730_body5_e15597) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn0))) / (2.0 * assign12730_body5_e15600))), (assign12730_body5_e15588 * (((locals.var_t3__blk332_dn2 + locals.var_t2__blk331_dn2) + ((locals.var_cnst1bulk_dn2 * assign12730_body5_e15597) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn2))) / (2.0 * assign12730_body5_e15600))), (assign12730_body5_e15588 * (((locals.var_t3__blk332_dn6 + locals.var_t2__blk331_dn6) + ((locals.var_cnst1bulk_dn6 * assign12730_body5_e15597) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn6))) / (2.0 * assign12730_body5_e15600))), (assign12730_body5_e15588 * (((locals.var_t3__blk332_dn7 + locals.var_t2__blk331_dn7) + ((locals.var_cnst1bulk_dn7 * assign12730_body5_e15597) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn7))) / (2.0 * assign12730_body5_e15600))), (((-locals.var_t1__blk330_dn10) * assign12730_body5_e15600) + (assign12730_body5_e15588 * (((locals.var_t3__blk332_dn10 + locals.var_t2__blk331_dn10) + ((locals.var_cnst1bulk_dn10 * assign12730_body5_e15597) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn10))) / (2.0 * assign12730_body5_e15600)))), (assign12730_body5_e15588 * (((locals.var_t3__blk332_dn11 + locals.var_t2__blk331_dn11) + ((locals.var_cnst1bulk_dn11 * assign12730_body5_e15597) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn11))) / (2.0 * assign12730_body5_e15600))), (assign12730_body5_e15588 * (((locals.var_t3__blk332_dn12 + locals.var_t2__blk331_dn12) + ((locals.var_cnst1bulk_dn12 * assign12730_body5_e15597) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn12))) / (2.0 * assign12730_body5_e15600))), (assign12730_body5_e15588 * (((locals.var_t3__blk332_dn17 + locals.var_t2__blk331_dn17) + ((locals.var_cnst1bulk_dn17 * assign12730_body5_e15597) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn17))) / (2.0 * assign12730_body5_e15600))),)
    } else {
        (locals.var_t4__blk333, locals.var_t4__blk333_dn0, locals.var_t4__blk333_dn2, locals.var_t4__blk333_dn6, locals.var_t4__blk333_dn7, locals.var_t4__blk333_dn10, locals.var_t4__blk333_dn11, locals.var_t4__blk333_dn12, locals.var_t4__blk333_dn17,)
    }
};
            locals.var_t4__blk333 = assign12730_body5_e15603;
            locals.var_t4__blk333_dn0 = assign12730_body5_e15603_d_n0;
            locals.var_t4__blk333_dn2 = assign12730_body5_e15603_d_n2;
            locals.var_t4__blk333_dn6 = assign12730_body5_e15603_d_n6;
            locals.var_t4__blk333_dn7 = assign12730_body5_e15603_d_n7;
            locals.var_t4__blk333_dn10 = assign12730_body5_e15603_d_n10;
            locals.var_t4__blk333_dn11 = assign12730_body5_e15603_d_n11;
            locals.var_t4__blk333_dn12 = assign12730_body5_e15603_d_n12;
            locals.var_t4__blk333_dn17 = assign12730_body5_e15603_d_n17;
            let (assign12730_body6_e15632, assign12730_body6_e15632_d_n0, assign12730_body6_e15632_d_n2, assign12730_body6_e15632_d_n6, assign12730_body6_e15632_d_n7, assign12730_body6_e15632_d_n10, assign12730_body6_e15632_d_n11, assign12730_body6_e15632_d_n12, assign12730_body6_e15632_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard343 != 0.0)) {
        let assign12730_body6_e15621: f64 = (locals.var_c0bulk / locals.var_t4__blk333);
        let assign12730_body6_e15623: f64 = (-locals.var_t3__blk332);
        let assign12730_body6_e15625: f64 = (assign12730_body6_e15623 + 1.0);
        let assign12730_body6_e15628: f64 = (locals.var_cnst1bulk * locals.var_t0__blk329);
        let assign12730_body6_e15629: f64 = (assign12730_body6_e15625 + assign12730_body6_e15628);
        let assign12730_body6_e15630: f64 = (assign12730_body6_e15621 * assign12730_body6_e15629);
        (assign12730_body6_e15630, (((-((locals.var_c0bulk * locals.var_t4__blk333_dn0) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body6_e15629) + (assign12730_body6_e15621 * ((-locals.var_t3__blk332_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn0))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn2) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body6_e15629) + (assign12730_body6_e15621 * ((-locals.var_t3__blk332_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn2))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn6) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body6_e15629) + (assign12730_body6_e15621 * ((-locals.var_t3__blk332_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn6))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn7) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body6_e15629) + (assign12730_body6_e15621 * ((-locals.var_t3__blk332_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn7))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn10) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body6_e15629) + (assign12730_body6_e15621 * ((-locals.var_t3__blk332_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn10))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn11) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body6_e15629) + (assign12730_body6_e15621 * ((-locals.var_t3__blk332_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn11))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn12) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body6_e15629) + (assign12730_body6_e15621 * ((-locals.var_t3__blk332_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn12))))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn17) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body6_e15629) + (assign12730_body6_e15621 * ((-locals.var_t3__blk332_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0__blk329) + (locals.var_cnst1bulk * locals.var_t0__blk329_dn17))))),)
    } else {
        (locals.var_t5__blk334, locals.var_t5__blk334_dn0, locals.var_t5__blk334_dn2, locals.var_t5__blk334_dn6, locals.var_t5__blk334_dn7, locals.var_t5__blk334_dn10, locals.var_t5__blk334_dn11, locals.var_t5__blk334_dn12, locals.var_t5__blk334_dn17,)
    }
};
            locals.var_t5__blk334 = assign12730_body6_e15632;
            locals.var_t5__blk334_dn0 = assign12730_body6_e15632_d_n0;
            locals.var_t5__blk334_dn2 = assign12730_body6_e15632_d_n2;
            locals.var_t5__blk334_dn6 = assign12730_body6_e15632_d_n6;
            locals.var_t5__blk334_dn7 = assign12730_body6_e15632_d_n7;
            locals.var_t5__blk334_dn10 = assign12730_body6_e15632_d_n10;
            locals.var_t5__blk334_dn11 = assign12730_body6_e15632_d_n11;
            locals.var_t5__blk334_dn12 = assign12730_body6_e15632_d_n12;
            locals.var_t5__blk334_dn17 = assign12730_body6_e15632_d_n17;
            let assign12730_body7_e15635: f64 = (-1e-9);
            let assign12730_body7_e15636: f64 = if locals.var_phi_sl_bulk < assign12730_body7_e15635 { 1.0 } else { 0.0 };
            locals.var_guard344 = assign12730_body7_e15636;
            let (assign12730_body8_e15664, assign12730_body8_e15664_d_n0, assign12730_body8_e15664_d_n2, assign12730_body8_e15664_d_n6, assign12730_body8_e15664_d_n7, assign12730_body8_e15664_d_n10, assign12730_body8_e15664_d_n11, assign12730_body8_e15664_d_n12, assign12730_body8_e15664_d_n17,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 != 0.0)) {
        let assign12730_body8_e15658: f64 = (locals.var_t3__blk332 + locals.var_t2__blk331);
        let assign12730_body8_e15660: f64 = (assign12730_body8_e15658 - 1.0);
        let assign12730_body8_e15661: f64 = (assign12730_body8_e15660).sqrt();
        let assign12730_body8_e15662: f64 = (locals.var_t1__blk330 * assign12730_body8_e15661);
        (assign12730_body8_e15662, (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn0 + locals.var_t2__blk331_dn0) / (2.0 * assign12730_body8_e15661))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn2 + locals.var_t2__blk331_dn2) / (2.0 * assign12730_body8_e15661))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn6 + locals.var_t2__blk331_dn6) / (2.0 * assign12730_body8_e15661))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn7 + locals.var_t2__blk331_dn7) / (2.0 * assign12730_body8_e15661))), ((locals.var_t1__blk330_dn10 * assign12730_body8_e15661) + (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn10 + locals.var_t2__blk331_dn10) / (2.0 * assign12730_body8_e15661)))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn11 + locals.var_t2__blk331_dn11) / (2.0 * assign12730_body8_e15661))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn12 + locals.var_t2__blk331_dn12) / (2.0 * assign12730_body8_e15661))), (locals.var_t1__blk330 * ((locals.var_t3__blk332_dn17 + locals.var_t2__blk331_dn17) / (2.0 * assign12730_body8_e15661))),)
    } else {
        (locals.var_t4__blk333, locals.var_t4__blk333_dn0, locals.var_t4__blk333_dn2, locals.var_t4__blk333_dn6, locals.var_t4__blk333_dn7, locals.var_t4__blk333_dn10, locals.var_t4__blk333_dn11, locals.var_t4__blk333_dn12, locals.var_t4__blk333_dn17,)
    }
};
            locals.var_t4__blk333 = assign12730_body8_e15664;
            locals.var_t4__blk333_dn0 = assign12730_body8_e15664_d_n0;
            locals.var_t4__blk333_dn2 = assign12730_body8_e15664_d_n2;
            locals.var_t4__blk333_dn6 = assign12730_body8_e15664_d_n6;
            locals.var_t4__blk333_dn7 = assign12730_body8_e15664_d_n7;
            locals.var_t4__blk333_dn10 = assign12730_body8_e15664_d_n10;
            locals.var_t4__blk333_dn11 = assign12730_body8_e15664_d_n11;
            locals.var_t4__blk333_dn12 = assign12730_body8_e15664_d_n12;
            locals.var_t4__blk333_dn17 = assign12730_body8_e15664_d_n17;
            let (assign12730_body9_e15692, assign12730_body9_e15692_d_n0, assign12730_body9_e15692_d_n2, assign12730_body9_e15692_d_n6, assign12730_body9_e15692_d_n7, assign12730_body9_e15692_d_n10, assign12730_body9_e15692_d_n11, assign12730_body9_e15692_d_n12, assign12730_body9_e15692_d_n17,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 != 0.0)) {
        let assign12730_body9_e15685: f64 = (locals.var_c0bulk / locals.var_t4__blk333);
        let assign12730_body9_e15687: f64 = (-locals.var_t3__blk332);
        let assign12730_body9_e15689: f64 = (assign12730_body9_e15687 + 1.0);
        let assign12730_body9_e15690: f64 = (assign12730_body9_e15685 * assign12730_body9_e15689);
        (assign12730_body9_e15690, (((-((locals.var_c0bulk * locals.var_t4__blk333_dn0) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body9_e15689) + (assign12730_body9_e15685 * (-locals.var_t3__blk332_dn0))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn2) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body9_e15689) + (assign12730_body9_e15685 * (-locals.var_t3__blk332_dn2))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn6) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body9_e15689) + (assign12730_body9_e15685 * (-locals.var_t3__blk332_dn6))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn7) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body9_e15689) + (assign12730_body9_e15685 * (-locals.var_t3__blk332_dn7))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn10) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body9_e15689) + (assign12730_body9_e15685 * (-locals.var_t3__blk332_dn10))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn11) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body9_e15689) + (assign12730_body9_e15685 * (-locals.var_t3__blk332_dn11))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn12) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body9_e15689) + (assign12730_body9_e15685 * (-locals.var_t3__blk332_dn12))), (((-((locals.var_c0bulk * locals.var_t4__blk333_dn17) / (locals.var_t4__blk333 * locals.var_t4__blk333))) * assign12730_body9_e15689) + (assign12730_body9_e15685 * (-locals.var_t3__blk332_dn17))),)
    } else {
        (locals.var_t5__blk334, locals.var_t5__blk334_dn0, locals.var_t5__blk334_dn2, locals.var_t5__blk334_dn6, locals.var_t5__blk334_dn7, locals.var_t5__blk334_dn10, locals.var_t5__blk334_dn11, locals.var_t5__blk334_dn12, locals.var_t5__blk334_dn17,)
    }
};
            locals.var_t5__blk334 = assign12730_body9_e15692;
            locals.var_t5__blk334_dn0 = assign12730_body9_e15692_d_n0;
            locals.var_t5__blk334_dn2 = assign12730_body9_e15692_d_n2;
            locals.var_t5__blk334_dn6 = assign12730_body9_e15692_d_n6;
            locals.var_t5__blk334_dn7 = assign12730_body9_e15692_d_n7;
            locals.var_t5__blk334_dn10 = assign12730_body9_e15692_d_n10;
            locals.var_t5__blk334_dn11 = assign12730_body9_e15692_d_n11;
            locals.var_t5__blk334_dn12 = assign12730_body9_e15692_d_n12;
            locals.var_t5__blk334_dn17 = assign12730_body9_e15692_d_n17;
            let (assign12730_body10_e15722, assign12730_body10_e15722_d_n0, assign12730_body10_e15722_d_n2, assign12730_body10_e15722_d_n6, assign12730_body10_e15722_d_n7, assign12730_body10_e15722_d_n10, assign12730_body10_e15722_d_n11, assign12730_body10_e15722_d_n12, assign12730_body10_e15722_d_n17,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
        let assign12730_body10_e15714: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign12730_body10_e15715: f64 = (assign12730_body10_e15714).sqrt();
        let assign12730_body10_e15716: f64 = (-assign12730_body10_e15715);
        let assign12730_body10_e15718: f64 = (assign12730_body10_e15716 * locals.var_beta);
        let assign12730_body10_e15720: f64 = (assign12730_body10_e15718 * locals.var_phi_sl_bulk);
        (assign12730_body10_e15720, (assign12730_body10_e15718 * locals.var_phi_sl_bulk_dn0), (assign12730_body10_e15718 * locals.var_phi_sl_bulk_dn2), (assign12730_body10_e15718 * locals.var_phi_sl_bulk_dn6), (assign12730_body10_e15718 * locals.var_phi_sl_bulk_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign12730_body10_e15715))) * locals.var_beta) + (assign12730_body10_e15716 * locals.var_beta_dn10)) * locals.var_phi_sl_bulk) + (assign12730_body10_e15718 * locals.var_phi_sl_bulk_dn10)), (assign12730_body10_e15718 * locals.var_phi_sl_bulk_dn11), (assign12730_body10_e15718 * locals.var_phi_sl_bulk_dn12), (assign12730_body10_e15718 * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t4__blk333, locals.var_t4__blk333_dn0, locals.var_t4__blk333_dn2, locals.var_t4__blk333_dn6, locals.var_t4__blk333_dn7, locals.var_t4__blk333_dn10, locals.var_t4__blk333_dn11, locals.var_t4__blk333_dn12, locals.var_t4__blk333_dn17,)
    }
};
            locals.var_t4__blk333 = assign12730_body10_e15722;
            locals.var_t4__blk333_dn0 = assign12730_body10_e15722_d_n0;
            locals.var_t4__blk333_dn2 = assign12730_body10_e15722_d_n2;
            locals.var_t4__blk333_dn6 = assign12730_body10_e15722_d_n6;
            locals.var_t4__blk333_dn7 = assign12730_body10_e15722_d_n7;
            locals.var_t4__blk333_dn10 = assign12730_body10_e15722_d_n10;
            locals.var_t4__blk333_dn11 = assign12730_body10_e15722_d_n11;
            locals.var_t4__blk333_dn12 = assign12730_body10_e15722_d_n12;
            locals.var_t4__blk333_dn17 = assign12730_body10_e15722_d_n17;
            let (assign12730_body11_e15748, assign12730_body11_e15748_d_n0, assign12730_body11_e15748_d_n2, assign12730_body11_e15748_d_n6, assign12730_body11_e15748_d_n7, assign12730_body11_e15748_d_n10, assign12730_body11_e15748_d_n11, assign12730_body11_e15748_d_n12, assign12730_body11_e15748_d_n17,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
        let assign12730_body11_e15744: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign12730_body11_e15745: f64 = (assign12730_body11_e15744).sqrt();
        let assign12730_body11_e15746: f64 = (-assign12730_body11_e15745);
        (assign12730_body11_e15746, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign12730_body11_e15745))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk334, locals.var_t5__blk334_dn0, locals.var_t5__blk334_dn2, locals.var_t5__blk334_dn6, locals.var_t5__blk334_dn7, locals.var_t5__blk334_dn10, locals.var_t5__blk334_dn11, locals.var_t5__blk334_dn12, locals.var_t5__blk334_dn17,)
    }
};
            locals.var_t5__blk334 = assign12730_body11_e15748;
            locals.var_t5__blk334_dn0 = assign12730_body11_e15748_d_n0;
            locals.var_t5__blk334_dn2 = assign12730_body11_e15748_d_n2;
            locals.var_t5__blk334_dn6 = assign12730_body11_e15748_d_n6;
            locals.var_t5__blk334_dn7 = assign12730_body11_e15748_d_n7;
            locals.var_t5__blk334_dn10 = assign12730_body11_e15748_d_n10;
            locals.var_t5__blk334_dn11 = assign12730_body11_e15748_d_n11;
            locals.var_t5__blk334_dn12 = assign12730_body11_e15748_d_n12;
            locals.var_t5__blk334_dn17 = assign12730_body11_e15748_d_n17;
            let (assign12730_body12_e15773, assign12730_body12_e15773_d_n0, assign12730_body12_e15773_d_n2, assign12730_body12_e15773_d_n6, assign12730_body12_e15773_d_n7, assign12730_body12_e15773_d_n10, assign12730_body12_e15773_d_n11, assign12730_body12_e15773_d_n12, assign12730_body12_e15773_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body12_e15764: f64 = (locals.var_t4__blk333 * locals.var_t4__blk333);
        let assign12730_body12_e15767: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign12730_body12_e15769: f64 = (assign12730_body12_e15767 * locals.var_q_fd_dlt1);
        let assign12730_body12_e15770: f64 = (assign12730_body12_e15764 + assign12730_body12_e15769);
        let assign12730_body12_e15771: f64 = (assign12730_body12_e15770).sqrt();
        (assign12730_body12_e15771, ((((locals.var_t4__blk333_dn0 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign12730_body12_e15767 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign12730_body12_e15771)), ((((locals.var_t4__blk333_dn2 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign12730_body12_e15767 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign12730_body12_e15771)), ((((locals.var_t4__blk333_dn6 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign12730_body12_e15767 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign12730_body12_e15771)), ((((locals.var_t4__blk333_dn7 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign12730_body12_e15767 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign12730_body12_e15771)), ((((locals.var_t4__blk333_dn10 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign12730_body12_e15767 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign12730_body12_e15771)), ((((locals.var_t4__blk333_dn11 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign12730_body12_e15767 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign12730_body12_e15771)), ((((locals.var_t4__blk333_dn12 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign12730_body12_e15767 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign12730_body12_e15771)), ((((locals.var_t4__blk333_dn17 * locals.var_t4__blk333) + (locals.var_t4__blk333 * locals.var_t4__blk333_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign12730_body12_e15767 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign12730_body12_e15771)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12730_body12_e15773;
            locals.var_tmf2_dn0 = assign12730_body12_e15773_d_n0;
            locals.var_tmf2_dn2 = assign12730_body12_e15773_d_n2;
            locals.var_tmf2_dn6 = assign12730_body12_e15773_d_n6;
            locals.var_tmf2_dn7 = assign12730_body12_e15773_d_n7;
            locals.var_tmf2_dn10 = assign12730_body12_e15773_d_n10;
            locals.var_tmf2_dn11 = assign12730_body12_e15773_d_n11;
            locals.var_tmf2_dn12 = assign12730_body12_e15773_d_n12;
            locals.var_tmf2_dn17 = assign12730_body12_e15773_d_n17;
            let (assign12730_body13_e15795, assign12730_body13_e15795_d_n0, assign12730_body13_e15795_d_n2, assign12730_body13_e15795_d_n6, assign12730_body13_e15795_d_n7, assign12730_body13_e15795_d_n10, assign12730_body13_e15795_d_n11, assign12730_body13_e15795_d_n12, assign12730_body13_e15795_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body13_e15791: f64 = (locals.var_t4__blk333 / locals.var_tmf2);
        let assign12730_body13_e15792: f64 = (1.0 + assign12730_body13_e15791);
        let assign12730_body13_e15793: f64 = (0.5 * assign12730_body13_e15792);
        (assign12730_body13_e15793, (0.5 * (((locals.var_t4__blk333_dn0 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn2 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn6 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn7 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn10 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn11 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn12 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk333_dn17 * locals.var_tmf2) - (locals.var_t4__blk333 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7__blk336, locals.var_t7__blk336_dn0, locals.var_t7__blk336_dn2, locals.var_t7__blk336_dn6, locals.var_t7__blk336_dn7, locals.var_t7__blk336_dn10, locals.var_t7__blk336_dn11, locals.var_t7__blk336_dn12, locals.var_t7__blk336_dn17,)
    }
};
            locals.var_t7__blk336 = assign12730_body13_e15795;
            locals.var_t7__blk336_dn0 = assign12730_body13_e15795_d_n0;
            locals.var_t7__blk336_dn2 = assign12730_body13_e15795_d_n2;
            locals.var_t7__blk336_dn6 = assign12730_body13_e15795_d_n6;
            locals.var_t7__blk336_dn7 = assign12730_body13_e15795_d_n7;
            locals.var_t7__blk336_dn10 = assign12730_body13_e15795_d_n10;
            locals.var_t7__blk336_dn11 = assign12730_body13_e15795_d_n11;
            locals.var_t7__blk336_dn12 = assign12730_body13_e15795_d_n12;
            locals.var_t7__blk336_dn17 = assign12730_body13_e15795_d_n17;
            let (assign12730_body14_e15819, assign12730_body14_e15819_d_n0, assign12730_body14_e15819_d_n2, assign12730_body14_e15819_d_n6, assign12730_body14_e15819_d_n7, assign12730_body14_e15819_d_n10, assign12730_body14_e15819_d_n11, assign12730_body14_e15819_d_n12, assign12730_body14_e15819_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body14_e15812: f64 = (locals.var_t4__blk333 + locals.var_tmf2);
        let assign12730_body14_e15813: f64 = (0.5 * assign12730_body14_e15812);
        let assign12730_body14_e15816: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign12730_body14_e15817: f64 = (assign12730_body14_e15813 + assign12730_body14_e15816);
        (assign12730_body14_e15817, ((0.5 * (locals.var_t4__blk333_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4__blk333_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4__blk333_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4__blk333_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4__blk333_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4__blk333_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4__blk333_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4__blk333_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    }
};
            locals.var_t6__blk335 = assign12730_body14_e15819;
            locals.var_t6__blk335_dn0 = assign12730_body14_e15819_d_n0;
            locals.var_t6__blk335_dn2 = assign12730_body14_e15819_d_n2;
            locals.var_t6__blk335_dn6 = assign12730_body14_e15819_d_n6;
            locals.var_t6__blk335_dn7 = assign12730_body14_e15819_d_n7;
            locals.var_t6__blk335_dn10 = assign12730_body14_e15819_d_n10;
            locals.var_t6__blk335_dn11 = assign12730_body14_e15819_d_n11;
            locals.var_t6__blk335_dn12 = assign12730_body14_e15819_d_n12;
            locals.var_t6__blk335_dn17 = assign12730_body14_e15819_d_n17;
            let assign12730_body15_e15822: f64 = if locals.var_t6__blk335 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard345 = assign12730_body15_e15822;
            let (assign12730_body16_e15840, assign12730_body16_e15840_d_n0, assign12730_body16_e15840_d_n2, assign12730_body16_e15840_d_n6, assign12730_body16_e15840_d_n7, assign12730_body16_e15840_d_n10, assign12730_body16_e15840_d_n11, assign12730_body16_e15840_d_n12, assign12730_body16_e15840_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard345 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    }
};
            locals.var_t6__blk335 = assign12730_body16_e15840;
            locals.var_t6__blk335_dn0 = assign12730_body16_e15840_d_n0;
            locals.var_t6__blk335_dn2 = assign12730_body16_e15840_d_n2;
            locals.var_t6__blk335_dn6 = assign12730_body16_e15840_d_n6;
            locals.var_t6__blk335_dn7 = assign12730_body16_e15840_d_n7;
            locals.var_t6__blk335_dn10 = assign12730_body16_e15840_d_n10;
            locals.var_t6__blk335_dn11 = assign12730_body16_e15840_d_n11;
            locals.var_t6__blk335_dn12 = assign12730_body16_e15840_d_n12;
            locals.var_t6__blk335_dn17 = assign12730_body16_e15840_d_n17;
            let (assign12730_body17_e15858, assign12730_body17_e15858_d_n0, assign12730_body17_e15858_d_n2, assign12730_body17_e15858_d_n6, assign12730_body17_e15858_d_n7, assign12730_body17_e15858_d_n10, assign12730_body17_e15858_d_n11, assign12730_body17_e15858_d_n12, assign12730_body17_e15858_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard345 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk336, locals.var_t7__blk336_dn0, locals.var_t7__blk336_dn2, locals.var_t7__blk336_dn6, locals.var_t7__blk336_dn7, locals.var_t7__blk336_dn10, locals.var_t7__blk336_dn11, locals.var_t7__blk336_dn12, locals.var_t7__blk336_dn17,)
    }
};
            locals.var_t7__blk336 = assign12730_body17_e15858;
            locals.var_t7__blk336_dn0 = assign12730_body17_e15858_d_n0;
            locals.var_t7__blk336_dn2 = assign12730_body17_e15858_d_n2;
            locals.var_t7__blk336_dn6 = assign12730_body17_e15858_d_n6;
            locals.var_t7__blk336_dn7 = assign12730_body17_e15858_d_n7;
            locals.var_t7__blk336_dn10 = assign12730_body17_e15858_d_n10;
            locals.var_t7__blk336_dn11 = assign12730_body17_e15858_d_n11;
            locals.var_t7__blk336_dn12 = assign12730_body17_e15858_d_n12;
            locals.var_t7__blk336_dn17 = assign12730_body17_e15858_d_n17;
            let (assign12730_body18_e15879, assign12730_body18_e15879_d_n0, assign12730_body18_e15879_d_n2, assign12730_body18_e15879_d_n6, assign12730_body18_e15879_d_n7, assign12730_body18_e15879_d_n10, assign12730_body18_e15879_d_n11, assign12730_body18_e15879_d_n12, assign12730_body18_e15879_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body18_e15873: f64 = (-locals.var_q_fd_soi);
        let assign12730_body18_e15875: f64 = (assign12730_body18_e15873 - locals.var_t6__blk335);
        let assign12730_body18_e15877: f64 = (assign12730_body18_e15875 - locals.var_q_fd_dlt2);
        (assign12730_body18_e15877, (((-locals.var_q_fd_soi_dn0) - locals.var_t6__blk335_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6__blk335_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6__blk335_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6__blk335_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6__blk335_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6__blk335_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6__blk335_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6__blk335_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12730_body18_e15879;
            locals.var_tmf1_dn0 = assign12730_body18_e15879_d_n0;
            locals.var_tmf1_dn2 = assign12730_body18_e15879_d_n2;
            locals.var_tmf1_dn6 = assign12730_body18_e15879_d_n6;
            locals.var_tmf1_dn7 = assign12730_body18_e15879_d_n7;
            locals.var_tmf1_dn10 = assign12730_body18_e15879_d_n10;
            locals.var_tmf1_dn11 = assign12730_body18_e15879_d_n11;
            locals.var_tmf1_dn12 = assign12730_body18_e15879_d_n12;
            locals.var_tmf1_dn17 = assign12730_body18_e15879_d_n17;
            let (assign12730_body19_e15900, assign12730_body19_e15900_d_n0, assign12730_body19_e15900_d_n2, assign12730_body19_e15900_d_n6, assign12730_body19_e15900_d_n7, assign12730_body19_e15900_d_n10, assign12730_body19_e15900_d_n11, assign12730_body19_e15900_d_n12, assign12730_body19_e15900_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body19_e15895: f64 = (-locals.var_q_fd_soi);
        let assign12730_body19_e15896: f64 = (4.0 * assign12730_body19_e15895);
        let assign12730_body19_e15898: f64 = (assign12730_body19_e15896 * locals.var_q_fd_dlt2);
        (assign12730_body19_e15898, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign12730_body19_e15896 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign12730_body19_e15896 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign12730_body19_e15896 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign12730_body19_e15896 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign12730_body19_e15896 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign12730_body19_e15896 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign12730_body19_e15896 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign12730_body19_e15896 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12730_body19_e15900;
            locals.var_tmf2_dn0 = assign12730_body19_e15900_d_n0;
            locals.var_tmf2_dn2 = assign12730_body19_e15900_d_n2;
            locals.var_tmf2_dn6 = assign12730_body19_e15900_d_n6;
            locals.var_tmf2_dn7 = assign12730_body19_e15900_d_n7;
            locals.var_tmf2_dn10 = assign12730_body19_e15900_d_n10;
            locals.var_tmf2_dn11 = assign12730_body19_e15900_d_n11;
            locals.var_tmf2_dn12 = assign12730_body19_e15900_d_n12;
            locals.var_tmf2_dn17 = assign12730_body19_e15900_d_n17;
            let (assign12730_body20_e15922, assign12730_body20_e15922_d_n0, assign12730_body20_e15922_d_n2, assign12730_body20_e15922_d_n6, assign12730_body20_e15922_d_n7, assign12730_body20_e15922_d_n10, assign12730_body20_e15922_d_n11, assign12730_body20_e15922_d_n12, assign12730_body20_e15922_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let (assign12730_body20_e15920, assign12730_body20_e15920_d_n0, assign12730_body20_e15920_d_n2, assign12730_body20_e15920_d_n6, assign12730_body20_e15920_d_n7, assign12730_body20_e15920_d_n10, assign12730_body20_e15920_d_n11, assign12730_body20_e15920_d_n12, assign12730_body20_e15920_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12730_body20_e15919: f64 = (-locals.var_tmf2);
                (assign12730_body20_e15919, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12730_body20_e15920, assign12730_body20_e15920_d_n0, assign12730_body20_e15920_d_n2, assign12730_body20_e15920_d_n6, assign12730_body20_e15920_d_n7, assign12730_body20_e15920_d_n10, assign12730_body20_e15920_d_n11, assign12730_body20_e15920_d_n12, assign12730_body20_e15920_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12730_body20_e15922;
            locals.var_tmf2_dn0 = assign12730_body20_e15922_d_n0;
            locals.var_tmf2_dn2 = assign12730_body20_e15922_d_n2;
            locals.var_tmf2_dn6 = assign12730_body20_e15922_d_n6;
            locals.var_tmf2_dn7 = assign12730_body20_e15922_d_n7;
            locals.var_tmf2_dn10 = assign12730_body20_e15922_d_n10;
            locals.var_tmf2_dn11 = assign12730_body20_e15922_d_n11;
            locals.var_tmf2_dn12 = assign12730_body20_e15922_d_n12;
            locals.var_tmf2_dn17 = assign12730_body20_e15922_d_n17;
            let (assign12730_body21_e15943, assign12730_body21_e15943_d_n0, assign12730_body21_e15943_d_n2, assign12730_body21_e15943_d_n6, assign12730_body21_e15943_d_n7, assign12730_body21_e15943_d_n10, assign12730_body21_e15943_d_n11, assign12730_body21_e15943_d_n12, assign12730_body21_e15943_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body21_e15938: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12730_body21_e15940: f64 = (assign12730_body21_e15938 + locals.var_tmf2);
        let assign12730_body21_e15941: f64 = (assign12730_body21_e15940).sqrt();
        (assign12730_body21_e15941, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12730_body21_e15941)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12730_body21_e15941)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12730_body21_e15941)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12730_body21_e15941)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12730_body21_e15941)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12730_body21_e15941)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12730_body21_e15941)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12730_body21_e15941)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12730_body21_e15943;
            locals.var_tmf2_dn0 = assign12730_body21_e15943_d_n0;
            locals.var_tmf2_dn2 = assign12730_body21_e15943_d_n2;
            locals.var_tmf2_dn6 = assign12730_body21_e15943_d_n6;
            locals.var_tmf2_dn7 = assign12730_body21_e15943_d_n7;
            locals.var_tmf2_dn10 = assign12730_body21_e15943_d_n10;
            locals.var_tmf2_dn11 = assign12730_body21_e15943_d_n11;
            locals.var_tmf2_dn12 = assign12730_body21_e15943_d_n12;
            locals.var_tmf2_dn17 = assign12730_body21_e15943_d_n17;
            let (assign12730_body22_e15965, assign12730_body22_e15965_d_n0, assign12730_body22_e15965_d_n2, assign12730_body22_e15965_d_n6, assign12730_body22_e15965_d_n7, assign12730_body22_e15965_d_n10, assign12730_body22_e15965_d_n11, assign12730_body22_e15965_d_n12, assign12730_body22_e15965_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body22_e15961: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign12730_body22_e15962: f64 = (1.0 + assign12730_body22_e15961);
        let assign12730_body22_e15963: f64 = (0.5 * assign12730_body22_e15962);
        (assign12730_body22_e15963, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign12730_body22_e15965;
            locals.var_t8_dn0 = assign12730_body22_e15965_d_n0;
            locals.var_t8_dn2 = assign12730_body22_e15965_d_n2;
            locals.var_t8_dn6 = assign12730_body22_e15965_d_n6;
            locals.var_t8_dn7 = assign12730_body22_e15965_d_n7;
            locals.var_t8_dn10 = assign12730_body22_e15965_d_n10;
            locals.var_t8_dn11 = assign12730_body22_e15965_d_n11;
            locals.var_t8_dn12 = assign12730_body22_e15965_d_n12;
            locals.var_t8_dn17 = assign12730_body22_e15965_d_n17;
            let (assign12730_body23_e15988, assign12730_body23_e15988_d_n0, assign12730_body23_e15988_d_n2, assign12730_body23_e15988_d_n6, assign12730_body23_e15988_d_n7, assign12730_body23_e15988_d_n10, assign12730_body23_e15988_d_n11, assign12730_body23_e15988_d_n12, assign12730_body23_e15988_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body23_e15980: f64 = (-locals.var_q_fd_soi);
        let assign12730_body23_e15984: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12730_body23_e15985: f64 = (0.5 * assign12730_body23_e15984);
        let assign12730_body23_e15986: f64 = (assign12730_body23_e15980 - assign12730_body23_e15985);
        (assign12730_body23_e15986, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    }
};
            locals.var_t6__blk335 = assign12730_body23_e15988;
            locals.var_t6__blk335_dn0 = assign12730_body23_e15988_d_n0;
            locals.var_t6__blk335_dn2 = assign12730_body23_e15988_d_n2;
            locals.var_t6__blk335_dn6 = assign12730_body23_e15988_d_n6;
            locals.var_t6__blk335_dn7 = assign12730_body23_e15988_d_n7;
            locals.var_t6__blk335_dn10 = assign12730_body23_e15988_d_n10;
            locals.var_t6__blk335_dn11 = assign12730_body23_e15988_d_n11;
            locals.var_t6__blk335_dn12 = assign12730_body23_e15988_d_n12;
            locals.var_t6__blk335_dn17 = assign12730_body23_e15988_d_n17;
            let (assign12730_body24_e16008, assign12730_body24_e16008_d_n0, assign12730_body24_e16008_d_n2, assign12730_body24_e16008_d_n6, assign12730_body24_e16008_d_n7, assign12730_body24_e16008_d_n10, assign12730_body24_e16008_d_n11, assign12730_body24_e16008_d_n12, assign12730_body24_e16008_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body24_e16005: f64 = (locals.var_t5__blk334 * locals.var_t8);
        let assign12730_body24_e16006: f64 = (locals.var_t7__blk336 * assign12730_body24_e16005);
        (assign12730_body24_e16006, ((locals.var_t7__blk336_dn0 * assign12730_body24_e16005) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn0 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn0)))), ((locals.var_t7__blk336_dn2 * assign12730_body24_e16005) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn2 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn2)))), ((locals.var_t7__blk336_dn6 * assign12730_body24_e16005) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn6 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn6)))), ((locals.var_t7__blk336_dn7 * assign12730_body24_e16005) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn7 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn7)))), ((locals.var_t7__blk336_dn10 * assign12730_body24_e16005) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn10 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn10)))), ((locals.var_t7__blk336_dn11 * assign12730_body24_e16005) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn11 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn11)))), ((locals.var_t7__blk336_dn12 * assign12730_body24_e16005) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn12 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn12)))), ((locals.var_t7__blk336_dn17 * assign12730_body24_e16005) + (locals.var_t7__blk336 * ((locals.var_t5__blk334_dn17 * locals.var_t8) + (locals.var_t5__blk334 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7__blk336, locals.var_t7__blk336_dn0, locals.var_t7__blk336_dn2, locals.var_t7__blk336_dn6, locals.var_t7__blk336_dn7, locals.var_t7__blk336_dn10, locals.var_t7__blk336_dn11, locals.var_t7__blk336_dn12, locals.var_t7__blk336_dn17,)
    }
};
            locals.var_t7__blk336 = assign12730_body24_e16008;
            locals.var_t7__blk336_dn0 = assign12730_body24_e16008_d_n0;
            locals.var_t7__blk336_dn2 = assign12730_body24_e16008_d_n2;
            locals.var_t7__blk336_dn6 = assign12730_body24_e16008_d_n6;
            locals.var_t7__blk336_dn7 = assign12730_body24_e16008_d_n7;
            locals.var_t7__blk336_dn10 = assign12730_body24_e16008_d_n10;
            locals.var_t7__blk336_dn11 = assign12730_body24_e16008_d_n11;
            locals.var_t7__blk336_dn12 = assign12730_body24_e16008_d_n12;
            locals.var_t7__blk336_dn17 = assign12730_body24_e16008_d_n17;
            let (assign12730_body25_e16034, assign12730_body25_e16034_d_n0, assign12730_body25_e16034_d_n2, assign12730_body25_e16034_d_n6, assign12730_body25_e16034_d_n7, assign12730_body25_e16034_d_n10, assign12730_body25_e16034_d_n11, assign12730_body25_e16034_d_n12, assign12730_body25_e16034_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body25_e16024: f64 = (locals.var_t6__blk335 * locals.var_t6__blk335);
        let assign12730_body25_e16026: f64 = (assign12730_body25_e16024 / 2.0);
        let assign12730_body25_e16028: f64 = (assign12730_body25_e16026 / 1.034943e-10);
        let assign12730_body25_e16030: f64 = (assign12730_body25_e16028 / 1.6021918e-19);
        let assign12730_body25_e16032: f64 = (assign12730_body25_e16030 / locals.var_uc_nsubs);
        (assign12730_body25_e16032, ((((((((locals.var_t6__blk335_dn0 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12730_body25_e16030 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn2 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12730_body25_e16030 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn6 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12730_body25_e16030 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn7 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12730_body25_e16030 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn10 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12730_body25_e16030 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn11 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12730_body25_e16030 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn12 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12730_body25_e16030 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk335_dn17 * locals.var_t6__blk335) + (locals.var_t6__blk335 * locals.var_t6__blk335_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12730_body25_e16030 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
            locals.var_phi_b_dep = assign12730_body25_e16034;
            locals.var_phi_b_dep_dn0 = assign12730_body25_e16034_d_n0;
            locals.var_phi_b_dep_dn2 = assign12730_body25_e16034_d_n2;
            locals.var_phi_b_dep_dn6 = assign12730_body25_e16034_d_n6;
            locals.var_phi_b_dep_dn7 = assign12730_body25_e16034_d_n7;
            locals.var_phi_b_dep_dn10 = assign12730_body25_e16034_d_n10;
            locals.var_phi_b_dep_dn11 = assign12730_body25_e16034_d_n11;
            locals.var_phi_b_dep_dn12 = assign12730_body25_e16034_d_n12;
            locals.var_phi_b_dep_dn17 = assign12730_body25_e16034_d_n17;
            let (assign12730_body26_e16056, assign12730_body26_e16056_d_n0, assign12730_body26_e16056_d_n2, assign12730_body26_e16056_d_n6, assign12730_body26_e16056_d_n7, assign12730_body26_e16056_d_n10, assign12730_body26_e16056_d_n11, assign12730_body26_e16056_d_n12, assign12730_body26_e16056_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body26_e16050: f64 = (2.0 * locals.var_phi_b_dep);
        let assign12730_body26_e16052: f64 = (assign12730_body26_e16050 * locals.var_t7__blk336);
        let assign12730_body26_e16054: f64 = (assign12730_body26_e16052 / locals.var_t6__blk335);
        (assign12730_body26_e16054, ((((((2.0 * locals.var_phi_b_dep_dn0) * locals.var_t7__blk336) + (assign12730_body26_e16050 * locals.var_t7__blk336_dn0)) * locals.var_t6__blk335) - (assign12730_body26_e16052 * locals.var_t6__blk335_dn0)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn2) * locals.var_t7__blk336) + (assign12730_body26_e16050 * locals.var_t7__blk336_dn2)) * locals.var_t6__blk335) - (assign12730_body26_e16052 * locals.var_t6__blk335_dn2)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn6) * locals.var_t7__blk336) + (assign12730_body26_e16050 * locals.var_t7__blk336_dn6)) * locals.var_t6__blk335) - (assign12730_body26_e16052 * locals.var_t6__blk335_dn6)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn7) * locals.var_t7__blk336) + (assign12730_body26_e16050 * locals.var_t7__blk336_dn7)) * locals.var_t6__blk335) - (assign12730_body26_e16052 * locals.var_t6__blk335_dn7)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn10) * locals.var_t7__blk336) + (assign12730_body26_e16050 * locals.var_t7__blk336_dn10)) * locals.var_t6__blk335) - (assign12730_body26_e16052 * locals.var_t6__blk335_dn10)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn11) * locals.var_t7__blk336) + (assign12730_body26_e16050 * locals.var_t7__blk336_dn11)) * locals.var_t6__blk335) - (assign12730_body26_e16052 * locals.var_t6__blk335_dn11)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn12) * locals.var_t7__blk336) + (assign12730_body26_e16050 * locals.var_t7__blk336_dn12)) * locals.var_t6__blk335) - (assign12730_body26_e16052 * locals.var_t6__blk335_dn12)) / (locals.var_t6__blk335 * locals.var_t6__blk335)), ((((((2.0 * locals.var_phi_b_dep_dn17) * locals.var_t7__blk336) + (assign12730_body26_e16050 * locals.var_t7__blk336_dn17)) * locals.var_t6__blk335) - (assign12730_body26_e16052 * locals.var_t6__blk335_dn17)) / (locals.var_t6__blk335 * locals.var_t6__blk335)),)
    } else {
        (locals.var_phi_b_dep_dpsb, locals.var_phi_b_dep_dpsb_dn0, locals.var_phi_b_dep_dpsb_dn2, locals.var_phi_b_dep_dpsb_dn6, locals.var_phi_b_dep_dpsb_dn7, locals.var_phi_b_dep_dpsb_dn10, locals.var_phi_b_dep_dpsb_dn11, locals.var_phi_b_dep_dpsb_dn12, locals.var_phi_b_dep_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep_dpsb = assign12730_body26_e16056;
            locals.var_phi_b_dep_dpsb_dn0 = assign12730_body26_e16056_d_n0;
            locals.var_phi_b_dep_dpsb_dn2 = assign12730_body26_e16056_d_n2;
            locals.var_phi_b_dep_dpsb_dn6 = assign12730_body26_e16056_d_n6;
            locals.var_phi_b_dep_dpsb_dn7 = assign12730_body26_e16056_d_n7;
            locals.var_phi_b_dep_dpsb_dn10 = assign12730_body26_e16056_d_n10;
            locals.var_phi_b_dep_dpsb_dn11 = assign12730_body26_e16056_d_n11;
            locals.var_phi_b_dep_dpsb_dn12 = assign12730_body26_e16056_d_n12;
            locals.var_phi_b_dep_dpsb_dn17 = assign12730_body26_e16056_d_n17;
            let (assign12730_body27_e16109, assign12730_body27_e16109_d_n0, assign12730_body27_e16109_d_n2, assign12730_body27_e16109_d_n6, assign12730_body27_e16109_d_n7, assign12730_body27_e16109_d_n10, assign12730_body27_e16109_d_n11, assign12730_body27_e16109_d_n12, assign12730_body27_e16109_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body27_e16073: f64 = (locals.var_phi_sl_soi - locals.var_phi_sl_bulk);
        let assign12730_body27_e16076: f64 = (locals.var_t4__blk333 / locals.var_c_box);
        let assign12730_body27_e16077: f64 = (assign12730_body27_e16073 + assign12730_body27_e16076);
        let assign12730_body27_e16081: f64 = (locals.var_q_fd_soi / 2.0);
        let assign12730_body27_e16082: f64 = (locals.var_t4__blk333 + assign12730_body27_e16081);
        let assign12730_body27_e16084: f64 = (assign12730_body27_e16082 * locals.var_t_soi);
        let assign12730_body27_e16086: f64 = (assign12730_body27_e16084 / 1.034943e-10);
        let assign12730_body27_e16087: f64 = (assign12730_body27_e16077 + assign12730_body27_e16086);
        let assign12730_body27_e16089: f64 = (assign12730_body27_e16087 - locals.var_vbsbiz);
        let assign12730_body27_e16091: f64 = (assign12730_body27_e16089 + locals.var_phi_b_dep);
        let assign12730_body27_e16093: f64 = (-1.0);
        let assign12730_body27_e16096: f64 = (locals.var_t5__blk334 / locals.var_c_box);
        let assign12730_body27_e16097: f64 = (assign12730_body27_e16093 + assign12730_body27_e16096);
        let assign12730_body27_e16100: f64 = (locals.var_t5__blk334 * locals.var_t_soi);
        let assign12730_body27_e16102: f64 = (assign12730_body27_e16100 / 1.034943e-10);
        let assign12730_body27_e16103: f64 = (assign12730_body27_e16097 + assign12730_body27_e16102);
        let assign12730_body27_e16105: f64 = (assign12730_body27_e16103 + locals.var_phi_b_dep_dpsb);
        let assign12730_body27_e16106: f64 = (assign12730_body27_e16091 / assign12730_body27_e16105);
        let assign12730_body27_e16107: f64 = (locals.var_phi_sl_bulk - assign12730_body27_e16106);
        (assign12730_body27_e16107, (locals.var_phi_sl_bulk_dn0 - ((((((((locals.var_phi_sl_soi_dn0 - locals.var_phi_sl_bulk_dn0) + (locals.var_t4__blk333_dn0 / locals.var_c_box)) + (((locals.var_t4__blk333_dn0 + (locals.var_q_fd_soi_dn0 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep_dn0) * assign12730_body27_e16105) - (assign12730_body27_e16091 * (((locals.var_t5__blk334_dn0 / locals.var_c_box) + ((locals.var_t5__blk334_dn0 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn0))) / (assign12730_body27_e16105 * assign12730_body27_e16105))), (locals.var_phi_sl_bulk_dn2 - ((((((((locals.var_phi_sl_soi_dn2 - locals.var_phi_sl_bulk_dn2) + (locals.var_t4__blk333_dn2 / locals.var_c_box)) + (((locals.var_t4__blk333_dn2 + (locals.var_q_fd_soi_dn2 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep_dn2) * assign12730_body27_e16105) - (assign12730_body27_e16091 * (((locals.var_t5__blk334_dn2 / locals.var_c_box) + ((locals.var_t5__blk334_dn2 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn2))) / (assign12730_body27_e16105 * assign12730_body27_e16105))), (locals.var_phi_sl_bulk_dn6 - ((((((((locals.var_phi_sl_soi_dn6 - locals.var_phi_sl_bulk_dn6) + (locals.var_t4__blk333_dn6 / locals.var_c_box)) + (((locals.var_t4__blk333_dn6 + (locals.var_q_fd_soi_dn6 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep_dn6) * assign12730_body27_e16105) - (assign12730_body27_e16091 * (((locals.var_t5__blk334_dn6 / locals.var_c_box) + ((locals.var_t5__blk334_dn6 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn6))) / (assign12730_body27_e16105 * assign12730_body27_e16105))), (locals.var_phi_sl_bulk_dn7 - ((((((((locals.var_phi_sl_soi_dn7 - locals.var_phi_sl_bulk_dn7) + (locals.var_t4__blk333_dn7 / locals.var_c_box)) + (((locals.var_t4__blk333_dn7 + (locals.var_q_fd_soi_dn7 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep_dn7) * assign12730_body27_e16105) - (assign12730_body27_e16091 * (((locals.var_t5__blk334_dn7 / locals.var_c_box) + ((locals.var_t5__blk334_dn7 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn7))) / (assign12730_body27_e16105 * assign12730_body27_e16105))), (locals.var_phi_sl_bulk_dn10 - ((((((((locals.var_phi_sl_soi_dn10 - locals.var_phi_sl_bulk_dn10) + (locals.var_t4__blk333_dn10 / locals.var_c_box)) + (((locals.var_t4__blk333_dn10 + (locals.var_q_fd_soi_dn10 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep_dn10) * assign12730_body27_e16105) - (assign12730_body27_e16091 * (((locals.var_t5__blk334_dn10 / locals.var_c_box) + ((locals.var_t5__blk334_dn10 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn10))) / (assign12730_body27_e16105 * assign12730_body27_e16105))), (locals.var_phi_sl_bulk_dn11 - ((((((((locals.var_phi_sl_soi_dn11 - locals.var_phi_sl_bulk_dn11) + (locals.var_t4__blk333_dn11 / locals.var_c_box)) + (((locals.var_t4__blk333_dn11 + (locals.var_q_fd_soi_dn11 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep_dn11) * assign12730_body27_e16105) - (assign12730_body27_e16091 * (((locals.var_t5__blk334_dn11 / locals.var_c_box) + ((locals.var_t5__blk334_dn11 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn11))) / (assign12730_body27_e16105 * assign12730_body27_e16105))), (locals.var_phi_sl_bulk_dn12 - ((((((((locals.var_phi_sl_soi_dn12 - locals.var_phi_sl_bulk_dn12) + (locals.var_t4__blk333_dn12 / locals.var_c_box)) + (((locals.var_t4__blk333_dn12 + (locals.var_q_fd_soi_dn12 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep_dn12) * assign12730_body27_e16105) - (assign12730_body27_e16091 * (((locals.var_t5__blk334_dn12 / locals.var_c_box) + ((locals.var_t5__blk334_dn12 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn12))) / (assign12730_body27_e16105 * assign12730_body27_e16105))), (locals.var_phi_sl_bulk_dn17 - ((((((((locals.var_phi_sl_soi_dn17 - locals.var_phi_sl_bulk_dn17) + (locals.var_t4__blk333_dn17 / locals.var_c_box)) + (((locals.var_t4__blk333_dn17 + (locals.var_q_fd_soi_dn17 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep_dn17) * assign12730_body27_e16105) - (assign12730_body27_e16091 * (((locals.var_t5__blk334_dn17 / locals.var_c_box) + ((locals.var_t5__blk334_dn17 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn17))) / (assign12730_body27_e16105 * assign12730_body27_e16105))),)
    } else {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    }
};
            locals.var_t6__blk335 = assign12730_body27_e16109;
            locals.var_t6__blk335_dn0 = assign12730_body27_e16109_d_n0;
            locals.var_t6__blk335_dn2 = assign12730_body27_e16109_d_n2;
            locals.var_t6__blk335_dn6 = assign12730_body27_e16109_d_n6;
            locals.var_t6__blk335_dn7 = assign12730_body27_e16109_d_n7;
            locals.var_t6__blk335_dn10 = assign12730_body27_e16109_d_n10;
            locals.var_t6__blk335_dn11 = assign12730_body27_e16109_d_n11;
            locals.var_t6__blk335_dn12 = assign12730_body27_e16109_d_n12;
            locals.var_t6__blk335_dn17 = assign12730_body27_e16109_d_n17;
            let assign12730_body28_e16112: f64 = (locals.var_t6__blk335 - locals.var_phi_sl_bulk);
            let assign12730_body28_e16113: f64 = (assign12730_body28_e16112).abs();
            let assign12730_body28_e16115: f64 = if assign12730_body28_e16113 < 5e-12 { 1.0 } else { 0.0 };
            locals.var_guard346 = assign12730_body28_e16115;
            let (assign12730_body29_e16133,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard346 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12730_body29_e16133;
            let (assign12730_body30_e16149, assign12730_body30_e16149_d_n0, assign12730_body30_e16149_d_n2, assign12730_body30_e16149_d_n6, assign12730_body30_e16149_d_n7, assign12730_body30_e16149_d_n10, assign12730_body30_e16149_d_n11, assign12730_body30_e16149_d_n12, assign12730_body30_e16149_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        (locals.var_t6__blk335, locals.var_t6__blk335_dn0, locals.var_t6__blk335_dn2, locals.var_t6__blk335_dn6, locals.var_t6__blk335_dn7, locals.var_t6__blk335_dn10, locals.var_t6__blk335_dn11, locals.var_t6__blk335_dn12, locals.var_t6__blk335_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12730_body30_e16149;
            locals.var_phi_sl_bulk_dn0 = assign12730_body30_e16149_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12730_body30_e16149_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12730_body30_e16149_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12730_body30_e16149_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12730_body30_e16149_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12730_body30_e16149_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12730_body30_e16149_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12730_body30_e16149_d_n17;
            let (assign12730_body31_e16165, assign12730_body31_e16165_d_n0, assign12730_body31_e16165_d_n2, assign12730_body31_e16165_d_n6, assign12730_body31_e16165_d_n7, assign12730_body31_e16165_d_n10, assign12730_body31_e16165_d_n11, assign12730_body31_e16165_d_n12, assign12730_body31_e16165_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        (locals.var_t4__blk333, locals.var_t4__blk333_dn0, locals.var_t4__blk333_dn2, locals.var_t4__blk333_dn6, locals.var_t4__blk333_dn7, locals.var_t4__blk333_dn10, locals.var_t4__blk333_dn11, locals.var_t4__blk333_dn12, locals.var_t4__blk333_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12730_body31_e16165;
            locals.var_q_sl_bulk_dn0 = assign12730_body31_e16165_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12730_body31_e16165_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12730_body31_e16165_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12730_body31_e16165_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12730_body31_e16165_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12730_body31_e16165_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12730_body31_e16165_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12730_body31_e16165_d_n17;
            let (assign12730_body32_e16183,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12730_body32_e16181: f64 = (locals.var_lp_sl + 1.0);
        (assign12730_body32_e16181,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12730_body32_e16183;
        }

    }

    pub(super) fn stamp_transient_block_40(
        locals: &mut StampLocals,
    ) {
        let (assign12740_e16201, assign12740_e16201_d_n0, assign12740_e16201_d_n2, assign12740_e16201_d_n6, assign12740_e16201_d_n7, assign12740_e16201_d_n10, assign12740_e16201_d_n11, assign12740_e16201_d_n12, assign12740_e16201_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12740_e16199: f64 = (locals.var_vbsbiz + locals.var_phi_sl_bulk);
        (assign12740_e16199, (locals.var_vbsbiz_dn0 + locals.var_phi_sl_bulk_dn0), (locals.var_vbsbiz_dn2 + locals.var_phi_sl_bulk_dn2), (locals.var_vbsbiz_dn6 + locals.var_phi_sl_bulk_dn6), (locals.var_vbsbiz_dn7 + locals.var_phi_sl_bulk_dn7), (locals.var_vbsbiz_dn10 + locals.var_phi_sl_bulk_dn10), (locals.var_vbsbiz_dn11 + locals.var_phi_sl_bulk_dn11), (locals.var_vbsbiz_dn12 + locals.var_phi_sl_bulk_dn12), (locals.var_vbsbiz_dn17 + locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12740_e16201;
        locals.var_phi_sl_bulk_dn0 = assign12740_e16201_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12740_e16201_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12740_e16201_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12740_e16201_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12740_e16201_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12740_e16201_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12740_e16201_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12740_e16201_d_n17;

        let (assign12750_e16221, assign12750_e16221_d_n0, assign12750_e16221_d_n2, assign12750_e16221_d_n6, assign12750_e16221_d_n7, assign12750_e16221_d_n10, assign12750_e16221_d_n11, assign12750_e16221_d_n12, assign12750_e16221_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign12750_e16218: f64 = (locals.var_q_sl_bulk / locals.var_c_box);
        let assign12750_e16219: f64 = (locals.var_phi_sl_bulk - assign12750_e16218);
        (assign12750_e16219, (locals.var_phi_sl_bulk_dn0 - (locals.var_q_sl_bulk_dn0 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn2 - (locals.var_q_sl_bulk_dn2 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn6 - (locals.var_q_sl_bulk_dn6 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn7 - (locals.var_q_sl_bulk_dn7 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn10 - (locals.var_q_sl_bulk_dn10 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn11 - (locals.var_q_sl_bulk_dn11 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn12 - (locals.var_q_sl_bulk_dn12 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn17 - (locals.var_q_sl_bulk_dn17 / locals.var_c_box)),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12750_e16221;
        locals.var_phi_bl_soi_dn0 = assign12750_e16221_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12750_e16221_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12750_e16221_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12750_e16221_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12750_e16221_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12750_e16221_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12750_e16221_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12750_e16221_d_n17;

        let assign12760_e16224: f64 = if locals.var_phi_bl_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard347 = assign12760_e16224;

        let (assign12770_e16239, assign12770_e16239_d_n0, assign12770_e16239_d_n2, assign12770_e16239_d_n6, assign12770_e16239_d_n7, assign12770_e16239_d_n10, assign12770_e16239_d_n11, assign12770_e16239_d_n12, assign12770_e16239_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard347 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12770_e16239;
        locals.var_phi_bl_soi_dn0 = assign12770_e16239_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12770_e16239_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12770_e16239_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12770_e16239_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12770_e16239_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12770_e16239_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12770_e16239_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12770_e16239_d_n17;

        let assign12780_e16242: f64 = if locals.var_phi_s0_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign12780_e16242;

        let (assign12790_e16251, assign12790_e16251_d_n0, assign12790_e16251_d_n2, assign12790_e16251_d_n6, assign12790_e16251_d_n7, assign12790_e16251_d_n10, assign12790_e16251_d_n11, assign12790_e16251_d_n12, assign12790_e16251_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard383 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12790_e16251;
        locals.var_phi_sl_soi_dn0 = assign12790_e16251_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12790_e16251_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12790_e16251_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12790_e16251_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12790_e16251_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12790_e16251_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12790_e16251_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12790_e16251_d_n17;

        let assign12800_e16254: f64 = if locals.var_phi_bl_soi < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign12800_e16254;

        let (assign12810_e16271, assign12810_e16271_d_n0, assign12810_e16271_d_n2, assign12810_e16271_d_n6, assign12810_e16271_d_n7, assign12810_e16271_d_n10, assign12810_e16271_d_n11, assign12810_e16271_d_n12, assign12810_e16271_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign12810_e16265: f64 = (0.5 * locals.var_q_fd_soi);
        let assign12810_e16267: f64 = (assign12810_e16265 + locals.var_q_s0_bulk);
        let assign12810_e16268: f64 = (locals.var_c_soi_inv__blk111 * assign12810_e16267);
        let assign12810_e16269: f64 = (locals.var_phi_sl_soi + assign12810_e16268);
        (assign12810_e16269, (locals.var_phi_sl_soi_dn0 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn0) + locals.var_q_s0_bulk_dn0))), (locals.var_phi_sl_soi_dn2 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn2) + locals.var_q_s0_bulk_dn2))), (locals.var_phi_sl_soi_dn6 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn6) + locals.var_q_s0_bulk_dn6))), (locals.var_phi_sl_soi_dn7 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn7) + locals.var_q_s0_bulk_dn7))), (locals.var_phi_sl_soi_dn10 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn10) + locals.var_q_s0_bulk_dn10))), (locals.var_phi_sl_soi_dn11 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn11) + locals.var_q_s0_bulk_dn11))), (locals.var_phi_sl_soi_dn12 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn12) + locals.var_q_s0_bulk_dn12))), (locals.var_phi_sl_soi_dn17 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn17) + locals.var_q_s0_bulk_dn17))),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12810_e16271;
        locals.var_phi_bl_soi_dn0 = assign12810_e16271_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12810_e16271_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12810_e16271_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12810_e16271_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12810_e16271_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12810_e16271_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12810_e16271_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12810_e16271_d_n17;

        let (assign12820_e16278, assign12820_e16278_d_n0, assign12820_e16278_d_n2, assign12820_e16278_d_n6, assign12820_e16278_d_n7, assign12820_e16278_d_n10, assign12820_e16278_d_n11, assign12820_e16278_d_n12, assign12820_e16278_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi_ini, locals.var_phi_sl_soi_ini_dn0, locals.var_phi_sl_soi_ini_dn2, locals.var_phi_sl_soi_ini_dn6, locals.var_phi_sl_soi_ini_dn7, locals.var_phi_sl_soi_ini_dn10, locals.var_phi_sl_soi_ini_dn11, locals.var_phi_sl_soi_ini_dn12, locals.var_phi_sl_soi_ini_dn17,)
    }
};
        locals.var_phi_sl_soi_ini = assign12820_e16278;
        locals.var_phi_sl_soi_ini_dn0 = assign12820_e16278_d_n0;
        locals.var_phi_sl_soi_ini_dn2 = assign12820_e16278_d_n2;
        locals.var_phi_sl_soi_ini_dn6 = assign12820_e16278_d_n6;
        locals.var_phi_sl_soi_ini_dn7 = assign12820_e16278_d_n7;
        locals.var_phi_sl_soi_ini_dn10 = assign12820_e16278_d_n10;
        locals.var_phi_sl_soi_ini_dn11 = assign12820_e16278_d_n11;
        locals.var_phi_sl_soi_ini_dn12 = assign12820_e16278_d_n12;
        locals.var_phi_sl_soi_ini_dn17 = assign12820_e16278_d_n17;

        let (assign12830_e16285, assign12830_e16285_d_n0, assign12830_e16285_d_n2, assign12830_e16285_d_n6, assign12830_e16285_d_n7, assign12830_e16285_d_n10, assign12830_e16285_d_n11, assign12830_e16285_d_n12, assign12830_e16285_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi_ini, locals.var_phi_bl_soi_ini_dn0, locals.var_phi_bl_soi_ini_dn2, locals.var_phi_bl_soi_ini_dn6, locals.var_phi_bl_soi_ini_dn7, locals.var_phi_bl_soi_ini_dn10, locals.var_phi_bl_soi_ini_dn11, locals.var_phi_bl_soi_ini_dn12, locals.var_phi_bl_soi_ini_dn17,)
    }
};
        locals.var_phi_bl_soi_ini = assign12830_e16285;
        locals.var_phi_bl_soi_ini_dn0 = assign12830_e16285_d_n0;
        locals.var_phi_bl_soi_ini_dn2 = assign12830_e16285_d_n2;
        locals.var_phi_bl_soi_ini_dn6 = assign12830_e16285_d_n6;
        locals.var_phi_bl_soi_ini_dn7 = assign12830_e16285_d_n7;
        locals.var_phi_bl_soi_ini_dn10 = assign12830_e16285_d_n10;
        locals.var_phi_bl_soi_ini_dn11 = assign12830_e16285_d_n11;
        locals.var_phi_bl_soi_ini_dn12 = assign12830_e16285_d_n12;
        locals.var_phi_bl_soi_ini_dn17 = assign12830_e16285_d_n17;

        let (assign12840_e16292, assign12840_e16292_d_n0, assign12840_e16292_d_n2, assign12840_e16292_d_n6, assign12840_e16292_d_n7, assign12840_e16292_d_n10, assign12840_e16292_d_n11, assign12840_e16292_d_n12, assign12840_e16292_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk_ini, locals.var_phi_sl_bulk_ini_dn0, locals.var_phi_sl_bulk_ini_dn2, locals.var_phi_sl_bulk_ini_dn6, locals.var_phi_sl_bulk_ini_dn7, locals.var_phi_sl_bulk_ini_dn10, locals.var_phi_sl_bulk_ini_dn11, locals.var_phi_sl_bulk_ini_dn12, locals.var_phi_sl_bulk_ini_dn17,)
    }
};
        locals.var_phi_sl_bulk_ini = assign12840_e16292;
        locals.var_phi_sl_bulk_ini_dn0 = assign12840_e16292_d_n0;
        locals.var_phi_sl_bulk_ini_dn2 = assign12840_e16292_d_n2;
        locals.var_phi_sl_bulk_ini_dn6 = assign12840_e16292_d_n6;
        locals.var_phi_sl_bulk_ini_dn7 = assign12840_e16292_d_n7;
        locals.var_phi_sl_bulk_ini_dn10 = assign12840_e16292_d_n10;
        locals.var_phi_sl_bulk_ini_dn11 = assign12840_e16292_d_n11;
        locals.var_phi_sl_bulk_ini_dn12 = assign12840_e16292_d_n12;
        locals.var_phi_sl_bulk_ini_dn17 = assign12840_e16292_d_n17;

        let (assign12850_e16299,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign12850_e16299;

        let (assign12860_e16306,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk8,)
    }
};
        locals.var_flg_brk8 = assign12860_e16306;

        let (assign12870_e16313,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12870_e16313;

    }

    pub(super) fn stamp_transient_block_41(
        locals: &mut StampLocals,
    ) {
        let mut assign12880_loop_guard: usize = 0;
        while {
            let assign12880_cond_e16321: f64 = if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_lp_sl <= locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12880_cond_e16321 != 0.0
        } {
            assign12880_loop_guard += 1;
            assert!(assign12880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12880_body0_e16330, assign12880_body0_e16330_d_n0, assign12880_body0_e16330_d_n2, assign12880_body0_e16330_d_n6, assign12880_body0_e16330_d_n7, assign12880_body0_e16330_d_n10, assign12880_body0_e16330_d_n11, assign12880_body0_e16330_d_n12, assign12880_body0_e16330_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body0_e16328: f64 = (locals.var_phi_sl_bulk - locals.var_vbsbiz);
        (assign12880_body0_e16328, (locals.var_phi_sl_bulk_dn0 - locals.var_vbsbiz_dn0), (locals.var_phi_sl_bulk_dn2 - locals.var_vbsbiz_dn2), (locals.var_phi_sl_bulk_dn6 - locals.var_vbsbiz_dn6), (locals.var_phi_sl_bulk_dn7 - locals.var_vbsbiz_dn7), (locals.var_phi_sl_bulk_dn10 - locals.var_vbsbiz_dn10), (locals.var_phi_sl_bulk_dn11 - locals.var_vbsbiz_dn11), (locals.var_phi_sl_bulk_dn12 - locals.var_vbsbiz_dn12), (locals.var_phi_sl_bulk_dn17 - locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk349, locals.var_t1__blk349_dn0, locals.var_t1__blk349_dn2, locals.var_t1__blk349_dn6, locals.var_t1__blk349_dn7, locals.var_t1__blk349_dn10, locals.var_t1__blk349_dn11, locals.var_t1__blk349_dn12, locals.var_t1__blk349_dn17,)
    }
};
            locals.var_t1__blk349 = assign12880_body0_e16330;
            locals.var_t1__blk349_dn0 = assign12880_body0_e16330_d_n0;
            locals.var_t1__blk349_dn2 = assign12880_body0_e16330_d_n2;
            locals.var_t1__blk349_dn6 = assign12880_body0_e16330_d_n6;
            locals.var_t1__blk349_dn7 = assign12880_body0_e16330_d_n7;
            locals.var_t1__blk349_dn10 = assign12880_body0_e16330_d_n10;
            locals.var_t1__blk349_dn11 = assign12880_body0_e16330_d_n11;
            locals.var_t1__blk349_dn12 = assign12880_body0_e16330_d_n12;
            locals.var_t1__blk349_dn17 = assign12880_body0_e16330_d_n17;
            let (assign12880_body1_e16339, assign12880_body1_e16339_d_n0, assign12880_body1_e16339_d_n2, assign12880_body1_e16339_d_n6, assign12880_body1_e16339_d_n7, assign12880_body1_e16339_d_n10, assign12880_body1_e16339_d_n11, assign12880_body1_e16339_d_n12, assign12880_body1_e16339_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body1_e16337: f64 = (locals.var_beta * locals.var_t1__blk349);
        (assign12880_body1_e16337, (locals.var_beta * locals.var_t1__blk349_dn0), (locals.var_beta * locals.var_t1__blk349_dn2), (locals.var_beta * locals.var_t1__blk349_dn6), (locals.var_beta * locals.var_t1__blk349_dn7), ((locals.var_beta_dn10 * locals.var_t1__blk349) + (locals.var_beta * locals.var_t1__blk349_dn10)), (locals.var_beta * locals.var_t1__blk349_dn11), (locals.var_beta * locals.var_t1__blk349_dn12), (locals.var_beta * locals.var_t1__blk349_dn17),)
    } else {
        (locals.var_el, locals.var_el_dn0, locals.var_el_dn2, locals.var_el_dn6, locals.var_el_dn7, locals.var_el_dn10, locals.var_el_dn11, locals.var_el_dn12, locals.var_el_dn17,)
    }
};
            locals.var_el = assign12880_body1_e16339;
            locals.var_el_dn0 = assign12880_body1_e16339_d_n0;
            locals.var_el_dn2 = assign12880_body1_e16339_d_n2;
            locals.var_el_dn6 = assign12880_body1_e16339_d_n6;
            locals.var_el_dn7 = assign12880_body1_e16339_d_n7;
            locals.var_el_dn10 = assign12880_body1_e16339_d_n10;
            locals.var_el_dn11 = assign12880_body1_e16339_d_n11;
            locals.var_el_dn12 = assign12880_body1_e16339_d_n12;
            locals.var_el_dn17 = assign12880_body1_e16339_d_n17;
            let (assign12880_body2_e16348, assign12880_body2_e16348_d_n0, assign12880_body2_e16348_d_n2, assign12880_body2_e16348_d_n6, assign12880_body2_e16348_d_n7, assign12880_body2_e16348_d_n10, assign12880_body2_e16348_d_n11, assign12880_body2_e16348_d_n12, assign12880_body2_e16348_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body2_e16345: f64 = (-locals.var_el);
        let assign12880_body2_e16346: f64 = (assign12880_body2_e16345).exp();
        (assign12880_body2_e16346, (assign12880_body2_e16346 * (-locals.var_el_dn0)), (assign12880_body2_e16346 * (-locals.var_el_dn2)), (assign12880_body2_e16346 * (-locals.var_el_dn6)), (assign12880_body2_e16346 * (-locals.var_el_dn7)), (assign12880_body2_e16346 * (-locals.var_el_dn10)), (assign12880_body2_e16346 * (-locals.var_el_dn11)), (assign12880_body2_e16346 * (-locals.var_el_dn12)), (assign12880_body2_e16346 * (-locals.var_el_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body2_e16348;
            locals.var_t0_dn0 = assign12880_body2_e16348_d_n0;
            locals.var_t0_dn2 = assign12880_body2_e16348_d_n2;
            locals.var_t0_dn6 = assign12880_body2_e16348_d_n6;
            locals.var_t0_dn7 = assign12880_body2_e16348_d_n7;
            locals.var_t0_dn10 = assign12880_body2_e16348_d_n10;
            locals.var_t0_dn11 = assign12880_body2_e16348_d_n11;
            locals.var_t0_dn12 = assign12880_body2_e16348_d_n12;
            locals.var_t0_dn17 = assign12880_body2_e16348_d_n17;
            let assign12880_body3_e16351: f64 = (-1e-9);
            let assign12880_body3_e16352: f64 = if locals.var_t1__blk349 < assign12880_body3_e16351 { 1.0 } else { 0.0 };
            locals.var_guard385 = assign12880_body3_e16352;
            let (assign12880_body4_e16368, assign12880_body4_e16368_d_n0, assign12880_body4_e16368_d_n2, assign12880_body4_e16368_d_n6, assign12880_body4_e16368_d_n7, assign12880_body4_e16368_d_n10, assign12880_body4_e16368_d_n11, assign12880_body4_e16368_d_n12, assign12880_body4_e16368_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign12880_body4_e16362: f64 = (locals.var_t0 + locals.var_el);
        let assign12880_body4_e16364: f64 = (assign12880_body4_e16362 - 1.0);
        let assign12880_body4_e16365: f64 = (assign12880_body4_e16364).sqrt();
        let assign12880_body4_e16366: f64 = (locals.var_cnst0bulk * assign12880_body4_e16365);
        (assign12880_body4_e16366, (locals.var_cnst0bulk * ((locals.var_t0_dn0 + locals.var_el_dn0) / (2.0 * assign12880_body4_e16365))), (locals.var_cnst0bulk * ((locals.var_t0_dn2 + locals.var_el_dn2) / (2.0 * assign12880_body4_e16365))), (locals.var_cnst0bulk * ((locals.var_t0_dn6 + locals.var_el_dn6) / (2.0 * assign12880_body4_e16365))), (locals.var_cnst0bulk * ((locals.var_t0_dn7 + locals.var_el_dn7) / (2.0 * assign12880_body4_e16365))), ((locals.var_cnst0bulk_dn10 * assign12880_body4_e16365) + (locals.var_cnst0bulk * ((locals.var_t0_dn10 + locals.var_el_dn10) / (2.0 * assign12880_body4_e16365)))), (locals.var_cnst0bulk * ((locals.var_t0_dn11 + locals.var_el_dn11) / (2.0 * assign12880_body4_e16365))), (locals.var_cnst0bulk * ((locals.var_t0_dn12 + locals.var_el_dn12) / (2.0 * assign12880_body4_e16365))), (locals.var_cnst0bulk * ((locals.var_t0_dn17 + locals.var_el_dn17) / (2.0 * assign12880_body4_e16365))),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12880_body4_e16368;
            locals.var_q_sl_bulk_dn0 = assign12880_body4_e16368_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12880_body4_e16368_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12880_body4_e16368_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12880_body4_e16368_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12880_body4_e16368_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12880_body4_e16368_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12880_body4_e16368_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12880_body4_e16368_d_n17;
            let (assign12880_body5_e16384, assign12880_body5_e16384_d_n0, assign12880_body5_e16384_d_n2, assign12880_body5_e16384_d_n6, assign12880_body5_e16384_d_n7, assign12880_body5_e16384_d_n10, assign12880_body5_e16384_d_n11, assign12880_body5_e16384_d_n12, assign12880_body5_e16384_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign12880_body5_e16377: f64 = (-locals.var_t0);
        let assign12880_body5_e16379: f64 = (assign12880_body5_e16377 + 1.0);
        let assign12880_body5_e16380: f64 = (locals.var_c0bulk * assign12880_body5_e16379);
        let assign12880_body5_e16382: f64 = (assign12880_body5_e16380 / locals.var_q_sl_bulk);
        (assign12880_body5_e16382, ((((locals.var_c0bulk * (-locals.var_t0_dn0)) * locals.var_q_sl_bulk) - (assign12880_body5_e16380 * locals.var_q_sl_bulk_dn0)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn2)) * locals.var_q_sl_bulk) - (assign12880_body5_e16380 * locals.var_q_sl_bulk_dn2)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn6)) * locals.var_q_sl_bulk) - (assign12880_body5_e16380 * locals.var_q_sl_bulk_dn6)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn7)) * locals.var_q_sl_bulk) - (assign12880_body5_e16380 * locals.var_q_sl_bulk_dn7)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn10)) * locals.var_q_sl_bulk) - (assign12880_body5_e16380 * locals.var_q_sl_bulk_dn10)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn11)) * locals.var_q_sl_bulk) - (assign12880_body5_e16380 * locals.var_q_sl_bulk_dn11)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn12)) * locals.var_q_sl_bulk) - (assign12880_body5_e16380 * locals.var_q_sl_bulk_dn12)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn17)) * locals.var_q_sl_bulk) - (assign12880_body5_e16380 * locals.var_q_sl_bulk_dn17)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)),)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12880_body5_e16384;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12880_body5_e16384_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12880_body5_e16384_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12880_body5_e16384_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12880_body5_e16384_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12880_body5_e16384_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12880_body5_e16384_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12880_body5_e16384_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12880_body5_e16384_d_n17;
            let assign12880_body6_e16387: f64 = if locals.var_t1__blk349 > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard386 = assign12880_body6_e16387;
            let (assign12880_body7_e16400, assign12880_body7_e16400_d_n0, assign12880_body7_e16400_d_n2, assign12880_body7_e16400_d_n6, assign12880_body7_e16400_d_n7, assign12880_body7_e16400_d_n10, assign12880_body7_e16400_d_n11, assign12880_body7_e16400_d_n12, assign12880_body7_e16400_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign12880_body7_e16398: f64 = (locals.var_el).exp();
        (assign12880_body7_e16398, (assign12880_body7_e16398 * locals.var_el_dn0), (assign12880_body7_e16398 * locals.var_el_dn2), (assign12880_body7_e16398 * locals.var_el_dn6), (assign12880_body7_e16398 * locals.var_el_dn7), (assign12880_body7_e16398 * locals.var_el_dn10), (assign12880_body7_e16398 * locals.var_el_dn11), (assign12880_body7_e16398 * locals.var_el_dn12), (assign12880_body7_e16398 * locals.var_el_dn17),)
    } else {
        (locals.var_t2__blk350, locals.var_t2__blk350_dn0, locals.var_t2__blk350_dn2, locals.var_t2__blk350_dn6, locals.var_t2__blk350_dn7, locals.var_t2__blk350_dn10, locals.var_t2__blk350_dn11, locals.var_t2__blk350_dn12, locals.var_t2__blk350_dn17,)
    }
};
            locals.var_t2__blk350 = assign12880_body7_e16400;
            locals.var_t2__blk350_dn0 = assign12880_body7_e16400_d_n0;
            locals.var_t2__blk350_dn2 = assign12880_body7_e16400_d_n2;
            locals.var_t2__blk350_dn6 = assign12880_body7_e16400_d_n6;
            locals.var_t2__blk350_dn7 = assign12880_body7_e16400_d_n7;
            locals.var_t2__blk350_dn10 = assign12880_body7_e16400_d_n10;
            locals.var_t2__blk350_dn11 = assign12880_body7_e16400_d_n11;
            locals.var_t2__blk350_dn12 = assign12880_body7_e16400_d_n12;
            locals.var_t2__blk350_dn17 = assign12880_body7_e16400_d_n17;
            let (assign12880_body8_e16428, assign12880_body8_e16428_d_n0, assign12880_body8_e16428_d_n2, assign12880_body8_e16428_d_n6, assign12880_body8_e16428_d_n7, assign12880_body8_e16428_d_n10, assign12880_body8_e16428_d_n11, assign12880_body8_e16428_d_n12, assign12880_body8_e16428_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign12880_body8_e16411: f64 = (-locals.var_cnst0bulk);
        let assign12880_body8_e16414: f64 = (locals.var_t0 + locals.var_el);
        let assign12880_body8_e16416: f64 = (assign12880_body8_e16414 - 1.0);
        let assign12880_body8_e16420: f64 = (locals.var_t2__blk350 + locals.var_el);
        let assign12880_body8_e16422: f64 = (assign12880_body8_e16420 - 1.0);
        let assign12880_body8_e16423: f64 = (locals.var_cnst1bulk * assign12880_body8_e16422);
        let assign12880_body8_e16424: f64 = (assign12880_body8_e16416 + assign12880_body8_e16423);
        let assign12880_body8_e16425: f64 = (assign12880_body8_e16424).sqrt();
        let assign12880_body8_e16426: f64 = (assign12880_body8_e16411 * assign12880_body8_e16425);
        (assign12880_body8_e16426, (assign12880_body8_e16411 * (((locals.var_t0_dn0 + locals.var_el_dn0) + ((locals.var_cnst1bulk_dn0 * assign12880_body8_e16422) + (locals.var_cnst1bulk * (locals.var_t2__blk350_dn0 + locals.var_el_dn0)))) / (2.0 * assign12880_body8_e16425))), (assign12880_body8_e16411 * (((locals.var_t0_dn2 + locals.var_el_dn2) + ((locals.var_cnst1bulk_dn2 * assign12880_body8_e16422) + (locals.var_cnst1bulk * (locals.var_t2__blk350_dn2 + locals.var_el_dn2)))) / (2.0 * assign12880_body8_e16425))), (assign12880_body8_e16411 * (((locals.var_t0_dn6 + locals.var_el_dn6) + ((locals.var_cnst1bulk_dn6 * assign12880_body8_e16422) + (locals.var_cnst1bulk * (locals.var_t2__blk350_dn6 + locals.var_el_dn6)))) / (2.0 * assign12880_body8_e16425))), (assign12880_body8_e16411 * (((locals.var_t0_dn7 + locals.var_el_dn7) + ((locals.var_cnst1bulk_dn7 * assign12880_body8_e16422) + (locals.var_cnst1bulk * (locals.var_t2__blk350_dn7 + locals.var_el_dn7)))) / (2.0 * assign12880_body8_e16425))), (((-locals.var_cnst0bulk_dn10) * assign12880_body8_e16425) + (assign12880_body8_e16411 * (((locals.var_t0_dn10 + locals.var_el_dn10) + ((locals.var_cnst1bulk_dn10 * assign12880_body8_e16422) + (locals.var_cnst1bulk * (locals.var_t2__blk350_dn10 + locals.var_el_dn10)))) / (2.0 * assign12880_body8_e16425)))), (assign12880_body8_e16411 * (((locals.var_t0_dn11 + locals.var_el_dn11) + ((locals.var_cnst1bulk_dn11 * assign12880_body8_e16422) + (locals.var_cnst1bulk * (locals.var_t2__blk350_dn11 + locals.var_el_dn11)))) / (2.0 * assign12880_body8_e16425))), (assign12880_body8_e16411 * (((locals.var_t0_dn12 + locals.var_el_dn12) + ((locals.var_cnst1bulk_dn12 * assign12880_body8_e16422) + (locals.var_cnst1bulk * (locals.var_t2__blk350_dn12 + locals.var_el_dn12)))) / (2.0 * assign12880_body8_e16425))), (assign12880_body8_e16411 * (((locals.var_t0_dn17 + locals.var_el_dn17) + ((locals.var_cnst1bulk_dn17 * assign12880_body8_e16422) + (locals.var_cnst1bulk * (locals.var_t2__blk350_dn17 + locals.var_el_dn17)))) / (2.0 * assign12880_body8_e16425))),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12880_body8_e16428;
            locals.var_q_sl_bulk_dn0 = assign12880_body8_e16428_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12880_body8_e16428_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12880_body8_e16428_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12880_body8_e16428_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12880_body8_e16428_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12880_body8_e16428_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12880_body8_e16428_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12880_body8_e16428_d_n17;
            let (assign12880_body9_e16453, assign12880_body9_e16453_d_n0, assign12880_body9_e16453_d_n2, assign12880_body9_e16453_d_n6, assign12880_body9_e16453_d_n7, assign12880_body9_e16453_d_n10, assign12880_body9_e16453_d_n11, assign12880_body9_e16453_d_n12, assign12880_body9_e16453_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign12880_body9_e16440: f64 = (-locals.var_t0);
        let assign12880_body9_e16442: f64 = (assign12880_body9_e16440 + 1.0);
        let assign12880_body9_e16446: f64 = (locals.var_t2__blk350 + 1.0);
        let assign12880_body9_e16447: f64 = (locals.var_cnst1bulk * assign12880_body9_e16446);
        let assign12880_body9_e16448: f64 = (assign12880_body9_e16442 + assign12880_body9_e16447);
        let assign12880_body9_e16449: f64 = (locals.var_c0bulk * assign12880_body9_e16448);
        let assign12880_body9_e16451: f64 = (assign12880_body9_e16449 / locals.var_q_sl_bulk);
        (assign12880_body9_e16451, ((((locals.var_c0bulk * ((-locals.var_t0_dn0) + ((locals.var_cnst1bulk_dn0 * assign12880_body9_e16446) + (locals.var_cnst1bulk * locals.var_t2__blk350_dn0)))) * locals.var_q_sl_bulk) - (assign12880_body9_e16449 * locals.var_q_sl_bulk_dn0)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn2) + ((locals.var_cnst1bulk_dn2 * assign12880_body9_e16446) + (locals.var_cnst1bulk * locals.var_t2__blk350_dn2)))) * locals.var_q_sl_bulk) - (assign12880_body9_e16449 * locals.var_q_sl_bulk_dn2)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn6) + ((locals.var_cnst1bulk_dn6 * assign12880_body9_e16446) + (locals.var_cnst1bulk * locals.var_t2__blk350_dn6)))) * locals.var_q_sl_bulk) - (assign12880_body9_e16449 * locals.var_q_sl_bulk_dn6)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn7) + ((locals.var_cnst1bulk_dn7 * assign12880_body9_e16446) + (locals.var_cnst1bulk * locals.var_t2__blk350_dn7)))) * locals.var_q_sl_bulk) - (assign12880_body9_e16449 * locals.var_q_sl_bulk_dn7)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn10) + ((locals.var_cnst1bulk_dn10 * assign12880_body9_e16446) + (locals.var_cnst1bulk * locals.var_t2__blk350_dn10)))) * locals.var_q_sl_bulk) - (assign12880_body9_e16449 * locals.var_q_sl_bulk_dn10)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn11) + ((locals.var_cnst1bulk_dn11 * assign12880_body9_e16446) + (locals.var_cnst1bulk * locals.var_t2__blk350_dn11)))) * locals.var_q_sl_bulk) - (assign12880_body9_e16449 * locals.var_q_sl_bulk_dn11)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn12) + ((locals.var_cnst1bulk_dn12 * assign12880_body9_e16446) + (locals.var_cnst1bulk * locals.var_t2__blk350_dn12)))) * locals.var_q_sl_bulk) - (assign12880_body9_e16449 * locals.var_q_sl_bulk_dn12)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn17) + ((locals.var_cnst1bulk_dn17 * assign12880_body9_e16446) + (locals.var_cnst1bulk * locals.var_t2__blk350_dn17)))) * locals.var_q_sl_bulk) - (assign12880_body9_e16449 * locals.var_q_sl_bulk_dn17)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)),)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12880_body9_e16453;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12880_body9_e16453_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12880_body9_e16453_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12880_body9_e16453_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12880_body9_e16453_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12880_body9_e16453_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12880_body9_e16453_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12880_body9_e16453_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12880_body9_e16453_d_n17;
            let (assign12880_body10_e16469, assign12880_body10_e16469_d_n0, assign12880_body10_e16469_d_n2, assign12880_body10_e16469_d_n6, assign12880_body10_e16469_d_n7, assign12880_body10_e16469_d_n10, assign12880_body10_e16469_d_n11, assign12880_body10_e16469_d_n12, assign12880_body10_e16469_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign12880_body10_e16465: f64 = (-locals.var_cnst0bulk);
        let assign12880_body10_e16467: f64 = (assign12880_body10_e16465 * locals.var_el);
        (assign12880_body10_e16467, (assign12880_body10_e16465 * locals.var_el_dn0), (assign12880_body10_e16465 * locals.var_el_dn2), (assign12880_body10_e16465 * locals.var_el_dn6), (assign12880_body10_e16465 * locals.var_el_dn7), (((-locals.var_cnst0bulk_dn10) * locals.var_el) + (assign12880_body10_e16465 * locals.var_el_dn10)), (assign12880_body10_e16465 * locals.var_el_dn11), (assign12880_body10_e16465 * locals.var_el_dn12), (assign12880_body10_e16465 * locals.var_el_dn17),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12880_body10_e16469;
            locals.var_q_sl_bulk_dn0 = assign12880_body10_e16469_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12880_body10_e16469_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12880_body10_e16469_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12880_body10_e16469_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12880_body10_e16469_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12880_body10_e16469_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12880_body10_e16469_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12880_body10_e16469_d_n17;
            let (assign12880_body11_e16485, assign12880_body11_e16485_d_n0, assign12880_body11_e16485_d_n2, assign12880_body11_e16485_d_n6, assign12880_body11_e16485_d_n7, assign12880_body11_e16485_d_n10, assign12880_body11_e16485_d_n11, assign12880_body11_e16485_d_n12, assign12880_body11_e16485_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign12880_body11_e16481: f64 = (-locals.var_cnst0bulk);
        let assign12880_body11_e16483: f64 = (assign12880_body11_e16481 * locals.var_beta);
        (assign12880_body11_e16483, 0.0, 0.0, 0.0, 0.0, (((-locals.var_cnst0bulk_dn10) * locals.var_beta) + (assign12880_body11_e16481 * locals.var_beta_dn10)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12880_body11_e16485;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12880_body11_e16485_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12880_body11_e16485_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12880_body11_e16485_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12880_body11_e16485_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12880_body11_e16485_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12880_body11_e16485_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12880_body11_e16485_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12880_body11_e16485_d_n17;
            let (assign12880_body12_e16492, assign12880_body12_e16492_d_n0, assign12880_body12_e16492_d_n2, assign12880_body12_e16492_d_n6, assign12880_body12_e16492_d_n7, assign12880_body12_e16492_d_n10, assign12880_body12_e16492_d_n11, assign12880_body12_e16492_d_n12, assign12880_body12_e16492_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_q_s0_dep_ini, locals.var_q_s0_dep_ini_dn0, locals.var_q_s0_dep_ini_dn2, locals.var_q_s0_dep_ini_dn6, locals.var_q_s0_dep_ini_dn7, locals.var_q_s0_dep_ini_dn10, locals.var_q_s0_dep_ini_dn11, locals.var_q_s0_dep_ini_dn12, locals.var_q_s0_dep_ini_dn17,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn12, locals.var_q_sl_dep_dn17,)
    }
};
            locals.var_q_sl_dep = assign12880_body12_e16492;
            locals.var_q_sl_dep_dn0 = assign12880_body12_e16492_d_n0;
            locals.var_q_sl_dep_dn2 = assign12880_body12_e16492_d_n2;
            locals.var_q_sl_dep_dn6 = assign12880_body12_e16492_d_n6;
            locals.var_q_sl_dep_dn7 = assign12880_body12_e16492_d_n7;
            locals.var_q_sl_dep_dn10 = assign12880_body12_e16492_d_n10;
            locals.var_q_sl_dep_dn11 = assign12880_body12_e16492_d_n11;
            locals.var_q_sl_dep_dn12 = assign12880_body12_e16492_d_n12;
            locals.var_q_sl_dep_dn17 = assign12880_body12_e16492_d_n17;
            let (assign12880_body13_e16504, assign12880_body13_e16504_d_n0, assign12880_body13_e16504_d_n2, assign12880_body13_e16504_d_n6, assign12880_body13_e16504_d_n7, assign12880_body13_e16504_d_n10, assign12880_body13_e16504_d_n11, assign12880_body13_e16504_d_n12, assign12880_body13_e16504_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body13_e16500: f64 = (locals.var_phi_sl_soi - locals.var_vds);
        let assign12880_body13_e16501: f64 = (locals.var_beta * assign12880_body13_e16500);
        let assign12880_body13_e16502: f64 = (assign12880_body13_e16501).exp();
        (assign12880_body13_e16502, (assign12880_body13_e16502 * (locals.var_beta * (locals.var_phi_sl_soi_dn0 - locals.var_vds_dn0))), (assign12880_body13_e16502 * (locals.var_beta * (locals.var_phi_sl_soi_dn2 - locals.var_vds_dn2))), (assign12880_body13_e16502 * (locals.var_beta * (locals.var_phi_sl_soi_dn6 - locals.var_vds_dn6))), (assign12880_body13_e16502 * (locals.var_beta * (locals.var_phi_sl_soi_dn7 - locals.var_vds_dn7))), (assign12880_body13_e16502 * ((locals.var_beta_dn10 * assign12880_body13_e16500) + (locals.var_beta * (locals.var_phi_sl_soi_dn10 - locals.var_vds_dn10)))), (assign12880_body13_e16502 * (locals.var_beta * (locals.var_phi_sl_soi_dn11 - locals.var_vds_dn11))), (assign12880_body13_e16502 * (locals.var_beta * (locals.var_phi_sl_soi_dn12 - locals.var_vds_dn12))), (assign12880_body13_e16502 * (locals.var_beta * (locals.var_phi_sl_soi_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_t5__blk353, locals.var_t5__blk353_dn0, locals.var_t5__blk353_dn2, locals.var_t5__blk353_dn6, locals.var_t5__blk353_dn7, locals.var_t5__blk353_dn10, locals.var_t5__blk353_dn11, locals.var_t5__blk353_dn12, locals.var_t5__blk353_dn17,)
    }
};
            locals.var_t5__blk353 = assign12880_body13_e16504;
            locals.var_t5__blk353_dn0 = assign12880_body13_e16504_d_n0;
            locals.var_t5__blk353_dn2 = assign12880_body13_e16504_d_n2;
            locals.var_t5__blk353_dn6 = assign12880_body13_e16504_d_n6;
            locals.var_t5__blk353_dn7 = assign12880_body13_e16504_d_n7;
            locals.var_t5__blk353_dn10 = assign12880_body13_e16504_d_n10;
            locals.var_t5__blk353_dn11 = assign12880_body13_e16504_d_n11;
            locals.var_t5__blk353_dn12 = assign12880_body13_e16504_d_n12;
            locals.var_t5__blk353_dn17 = assign12880_body13_e16504_d_n17;
            let (assign12880_body14_e16511, assign12880_body14_e16511_d_n0, assign12880_body14_e16511_d_n2, assign12880_body14_e16511_d_n6, assign12880_body14_e16511_d_n7, assign12880_body14_e16511_d_n10, assign12880_body14_e16511_d_n11, assign12880_body14_e16511_d_n12, assign12880_body14_e16511_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk351, locals.var_t3__blk351_dn0, locals.var_t3__blk351_dn2, locals.var_t3__blk351_dn6, locals.var_t3__blk351_dn7, locals.var_t3__blk351_dn10, locals.var_t3__blk351_dn11, locals.var_t3__blk351_dn12, locals.var_t3__blk351_dn17,)
    }
};
            locals.var_t3__blk351 = assign12880_body14_e16511;
            locals.var_t3__blk351_dn0 = assign12880_body14_e16511_d_n0;
            locals.var_t3__blk351_dn2 = assign12880_body14_e16511_d_n2;
            locals.var_t3__blk351_dn6 = assign12880_body14_e16511_d_n6;
            locals.var_t3__blk351_dn7 = assign12880_body14_e16511_d_n7;
            locals.var_t3__blk351_dn10 = assign12880_body14_e16511_d_n10;
            locals.var_t3__blk351_dn11 = assign12880_body14_e16511_d_n11;
            locals.var_t3__blk351_dn12 = assign12880_body14_e16511_d_n12;
            locals.var_t3__blk351_dn17 = assign12880_body14_e16511_d_n17;
            let (assign12880_body15_e16535, assign12880_body15_e16535_d_n0, assign12880_body15_e16535_d_n2, assign12880_body15_e16535_d_n6, assign12880_body15_e16535_d_n7, assign12880_body15_e16535_d_n10, assign12880_body15_e16535_d_n11, assign12880_body15_e16535_d_n12, assign12880_body15_e16535_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body15_e16518: f64 = (locals.var_q_sl_dep * locals.var_q_sl_dep);
        let assign12880_body15_e16521: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        let assign12880_body15_e16522: f64 = (assign12880_body15_e16518 / assign12880_body15_e16521);
        let assign12880_body15_e16525: f64 = (2.0 * locals.var_cnst1soi);
        let assign12880_body15_e16528: f64 = (locals.var_t5__blk353 + locals.var_el);
        let assign12880_body15_e16530: f64 = (assign12880_body15_e16528 - locals.var_t3__blk351);
        let assign12880_body15_e16531: f64 = (assign12880_body15_e16525 * assign12880_body15_e16530);
        let assign12880_body15_e16532: f64 = (assign12880_body15_e16522 + assign12880_body15_e16531);
        let assign12880_body15_e16533: f64 = (assign12880_body15_e16532).sqrt();
        (assign12880_body15_e16533, (((((((locals.var_q_sl_dep_dn0 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn0)) * assign12880_body15_e16521) - (assign12880_body15_e16518 * ((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)))) / (assign12880_body15_e16521 * assign12880_body15_e16521)) + (((2.0 * locals.var_cnst1soi_dn0) * assign12880_body15_e16530) + (assign12880_body15_e16525 * ((locals.var_t5__blk353_dn0 + locals.var_el_dn0) - locals.var_t3__blk351_dn0)))) / (2.0 * assign12880_body15_e16533)), (((((((locals.var_q_sl_dep_dn2 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn2)) * assign12880_body15_e16521) - (assign12880_body15_e16518 * ((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)))) / (assign12880_body15_e16521 * assign12880_body15_e16521)) + (((2.0 * locals.var_cnst1soi_dn2) * assign12880_body15_e16530) + (assign12880_body15_e16525 * ((locals.var_t5__blk353_dn2 + locals.var_el_dn2) - locals.var_t3__blk351_dn2)))) / (2.0 * assign12880_body15_e16533)), (((((((locals.var_q_sl_dep_dn6 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn6)) * assign12880_body15_e16521) - (assign12880_body15_e16518 * ((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)))) / (assign12880_body15_e16521 * assign12880_body15_e16521)) + (((2.0 * locals.var_cnst1soi_dn6) * assign12880_body15_e16530) + (assign12880_body15_e16525 * ((locals.var_t5__blk353_dn6 + locals.var_el_dn6) - locals.var_t3__blk351_dn6)))) / (2.0 * assign12880_body15_e16533)), (((((((locals.var_q_sl_dep_dn7 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn7)) * assign12880_body15_e16521) - (assign12880_body15_e16518 * ((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)))) / (assign12880_body15_e16521 * assign12880_body15_e16521)) + (((2.0 * locals.var_cnst1soi_dn7) * assign12880_body15_e16530) + (assign12880_body15_e16525 * ((locals.var_t5__blk353_dn7 + locals.var_el_dn7) - locals.var_t3__blk351_dn7)))) / (2.0 * assign12880_body15_e16533)), (((((((locals.var_q_sl_dep_dn10 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn10)) * assign12880_body15_e16521) - (assign12880_body15_e16518 * ((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)))) / (assign12880_body15_e16521 * assign12880_body15_e16521)) + (((2.0 * locals.var_cnst1soi_dn10) * assign12880_body15_e16530) + (assign12880_body15_e16525 * ((locals.var_t5__blk353_dn10 + locals.var_el_dn10) - locals.var_t3__blk351_dn10)))) / (2.0 * assign12880_body15_e16533)), (((((((locals.var_q_sl_dep_dn11 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn11)) * assign12880_body15_e16521) - (assign12880_body15_e16518 * ((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)))) / (assign12880_body15_e16521 * assign12880_body15_e16521)) + (((2.0 * locals.var_cnst1soi_dn11) * assign12880_body15_e16530) + (assign12880_body15_e16525 * ((locals.var_t5__blk353_dn11 + locals.var_el_dn11) - locals.var_t3__blk351_dn11)))) / (2.0 * assign12880_body15_e16533)), (((((((locals.var_q_sl_dep_dn12 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn12)) * assign12880_body15_e16521) - (assign12880_body15_e16518 * ((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)))) / (assign12880_body15_e16521 * assign12880_body15_e16521)) + (((2.0 * locals.var_cnst1soi_dn12) * assign12880_body15_e16530) + (assign12880_body15_e16525 * ((locals.var_t5__blk353_dn12 + locals.var_el_dn12) - locals.var_t3__blk351_dn12)))) / (2.0 * assign12880_body15_e16533)), (((((((locals.var_q_sl_dep_dn17 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn17)) * assign12880_body15_e16521) - (assign12880_body15_e16518 * ((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)))) / (assign12880_body15_e16521 * assign12880_body15_e16521)) + (((2.0 * locals.var_cnst1soi_dn17) * assign12880_body15_e16530) + (assign12880_body15_e16525 * ((locals.var_t5__blk353_dn17 + locals.var_el_dn17) - locals.var_t3__blk351_dn17)))) / (2.0 * assign12880_body15_e16533)),)
    } else {
        (locals.var_t4__blk352, locals.var_t4__blk352_dn0, locals.var_t4__blk352_dn2, locals.var_t4__blk352_dn6, locals.var_t4__blk352_dn7, locals.var_t4__blk352_dn10, locals.var_t4__blk352_dn11, locals.var_t4__blk352_dn12, locals.var_t4__blk352_dn17,)
    }
};
            locals.var_t4__blk352 = assign12880_body15_e16535;
            locals.var_t4__blk352_dn0 = assign12880_body15_e16535_d_n0;
            locals.var_t4__blk352_dn2 = assign12880_body15_e16535_d_n2;
            locals.var_t4__blk352_dn6 = assign12880_body15_e16535_d_n6;
            locals.var_t4__blk352_dn7 = assign12880_body15_e16535_d_n7;
            locals.var_t4__blk352_dn10 = assign12880_body15_e16535_d_n10;
            locals.var_t4__blk352_dn11 = assign12880_body15_e16535_d_n11;
            locals.var_t4__blk352_dn12 = assign12880_body15_e16535_d_n12;
            locals.var_t4__blk352_dn17 = assign12880_body15_e16535_d_n17;
            let (assign12880_body16_e16554, assign12880_body16_e16554_d_n0, assign12880_body16_e16554_d_n2, assign12880_body16_e16554_d_n6, assign12880_body16_e16554_d_n7, assign12880_body16_e16554_d_n10, assign12880_body16_e16554_d_n11, assign12880_body16_e16554_d_n12, assign12880_body16_e16554_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body16_e16542: f64 = (2.0 * locals.var_beta);
        let assign12880_body16_e16544: f64 = (assign12880_body16_e16542 * locals.var_cnst1soi);
        let assign12880_body16_e16547: f64 = (locals.var_t5__blk353 + 1.0);
        let assign12880_body16_e16548: f64 = (assign12880_body16_e16544 * assign12880_body16_e16547);
        let assign12880_body16_e16551: f64 = (2.0 * locals.var_t4__blk352);
        let assign12880_body16_e16552: f64 = (assign12880_body16_e16548 / assign12880_body16_e16551);
        (assign12880_body16_e16552, ((((((assign12880_body16_e16542 * locals.var_cnst1soi_dn0) * assign12880_body16_e16547) + (assign12880_body16_e16544 * locals.var_t5__blk353_dn0)) * assign12880_body16_e16551) - (assign12880_body16_e16548 * (2.0 * locals.var_t4__blk352_dn0))) / (assign12880_body16_e16551 * assign12880_body16_e16551)), ((((((assign12880_body16_e16542 * locals.var_cnst1soi_dn2) * assign12880_body16_e16547) + (assign12880_body16_e16544 * locals.var_t5__blk353_dn2)) * assign12880_body16_e16551) - (assign12880_body16_e16548 * (2.0 * locals.var_t4__blk352_dn2))) / (assign12880_body16_e16551 * assign12880_body16_e16551)), ((((((assign12880_body16_e16542 * locals.var_cnst1soi_dn6) * assign12880_body16_e16547) + (assign12880_body16_e16544 * locals.var_t5__blk353_dn6)) * assign12880_body16_e16551) - (assign12880_body16_e16548 * (2.0 * locals.var_t4__blk352_dn6))) / (assign12880_body16_e16551 * assign12880_body16_e16551)), ((((((assign12880_body16_e16542 * locals.var_cnst1soi_dn7) * assign12880_body16_e16547) + (assign12880_body16_e16544 * locals.var_t5__blk353_dn7)) * assign12880_body16_e16551) - (assign12880_body16_e16548 * (2.0 * locals.var_t4__blk352_dn7))) / (assign12880_body16_e16551 * assign12880_body16_e16551)), ((((((((2.0 * locals.var_beta_dn10) * locals.var_cnst1soi) + (assign12880_body16_e16542 * locals.var_cnst1soi_dn10)) * assign12880_body16_e16547) + (assign12880_body16_e16544 * locals.var_t5__blk353_dn10)) * assign12880_body16_e16551) - (assign12880_body16_e16548 * (2.0 * locals.var_t4__blk352_dn10))) / (assign12880_body16_e16551 * assign12880_body16_e16551)), ((((((assign12880_body16_e16542 * locals.var_cnst1soi_dn11) * assign12880_body16_e16547) + (assign12880_body16_e16544 * locals.var_t5__blk353_dn11)) * assign12880_body16_e16551) - (assign12880_body16_e16548 * (2.0 * locals.var_t4__blk352_dn11))) / (assign12880_body16_e16551 * assign12880_body16_e16551)), ((((((assign12880_body16_e16542 * locals.var_cnst1soi_dn12) * assign12880_body16_e16547) + (assign12880_body16_e16544 * locals.var_t5__blk353_dn12)) * assign12880_body16_e16551) - (assign12880_body16_e16548 * (2.0 * locals.var_t4__blk352_dn12))) / (assign12880_body16_e16551 * assign12880_body16_e16551)), ((((((assign12880_body16_e16542 * locals.var_cnst1soi_dn17) * assign12880_body16_e16547) + (assign12880_body16_e16544 * locals.var_t5__blk353_dn17)) * assign12880_body16_e16551) - (assign12880_body16_e16548 * (2.0 * locals.var_t4__blk352_dn17))) / (assign12880_body16_e16551 * assign12880_body16_e16551)),)
    } else {
        (locals.var_t4_dpss__blk382, locals.var_t4_dpss__blk382_dn0, locals.var_t4_dpss__blk382_dn2, locals.var_t4_dpss__blk382_dn6, locals.var_t4_dpss__blk382_dn7, locals.var_t4_dpss__blk382_dn10, locals.var_t4_dpss__blk382_dn11, locals.var_t4_dpss__blk382_dn12, locals.var_t4_dpss__blk382_dn17,)
    }
};
            locals.var_t4_dpss__blk382 = assign12880_body16_e16554;
            locals.var_t4_dpss__blk382_dn0 = assign12880_body16_e16554_d_n0;
            locals.var_t4_dpss__blk382_dn2 = assign12880_body16_e16554_d_n2;
            locals.var_t4_dpss__blk382_dn6 = assign12880_body16_e16554_d_n6;
            locals.var_t4_dpss__blk382_dn7 = assign12880_body16_e16554_d_n7;
            locals.var_t4_dpss__blk382_dn10 = assign12880_body16_e16554_d_n10;
            locals.var_t4_dpss__blk382_dn11 = assign12880_body16_e16554_d_n11;
            locals.var_t4_dpss__blk382_dn12 = assign12880_body16_e16554_d_n12;
            locals.var_t4_dpss__blk382_dn17 = assign12880_body16_e16554_d_n17;
            let (assign12880_body17_e16566, assign12880_body17_e16566_d_n0, assign12880_body17_e16566_d_n2, assign12880_body17_e16566_d_n6, assign12880_body17_e16566_d_n7, assign12880_body17_e16566_d_n10, assign12880_body17_e16566_d_n11, assign12880_body17_e16566_d_n12, assign12880_body17_e16566_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body17_e16560: f64 = (-locals.var_cnst0soi);
        let assign12880_body17_e16562: f64 = (assign12880_body17_e16560 * locals.var_t4__blk352);
        let assign12880_body17_e16564: f64 = (assign12880_body17_e16562 - locals.var_q_sl_dep);
        (assign12880_body17_e16564, ((((-locals.var_cnst0soi_dn0) * locals.var_t4__blk352) + (assign12880_body17_e16560 * locals.var_t4__blk352_dn0)) - locals.var_q_sl_dep_dn0), ((((-locals.var_cnst0soi_dn2) * locals.var_t4__blk352) + (assign12880_body17_e16560 * locals.var_t4__blk352_dn2)) - locals.var_q_sl_dep_dn2), ((((-locals.var_cnst0soi_dn6) * locals.var_t4__blk352) + (assign12880_body17_e16560 * locals.var_t4__blk352_dn6)) - locals.var_q_sl_dep_dn6), ((((-locals.var_cnst0soi_dn7) * locals.var_t4__blk352) + (assign12880_body17_e16560 * locals.var_t4__blk352_dn7)) - locals.var_q_sl_dep_dn7), ((((-locals.var_cnst0soi_dn10) * locals.var_t4__blk352) + (assign12880_body17_e16560 * locals.var_t4__blk352_dn10)) - locals.var_q_sl_dep_dn10), ((((-locals.var_cnst0soi_dn11) * locals.var_t4__blk352) + (assign12880_body17_e16560 * locals.var_t4__blk352_dn11)) - locals.var_q_sl_dep_dn11), ((((-locals.var_cnst0soi_dn12) * locals.var_t4__blk352) + (assign12880_body17_e16560 * locals.var_t4__blk352_dn12)) - locals.var_q_sl_dep_dn12), ((((-locals.var_cnst0soi_dn17) * locals.var_t4__blk352) + (assign12880_body17_e16560 * locals.var_t4__blk352_dn17)) - locals.var_q_sl_dep_dn17),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
            locals.var_q_nl = assign12880_body17_e16566;
            locals.var_q_nl_dn0 = assign12880_body17_e16566_d_n0;
            locals.var_q_nl_dn2 = assign12880_body17_e16566_d_n2;
            locals.var_q_nl_dn6 = assign12880_body17_e16566_d_n6;
            locals.var_q_nl_dn7 = assign12880_body17_e16566_d_n7;
            locals.var_q_nl_dn10 = assign12880_body17_e16566_d_n10;
            locals.var_q_nl_dn11 = assign12880_body17_e16566_d_n11;
            locals.var_q_nl_dn12 = assign12880_body17_e16566_d_n12;
            locals.var_q_nl_dn17 = assign12880_body17_e16566_d_n17;
            let (assign12880_body18_e16576, assign12880_body18_e16576_d_n0, assign12880_body18_e16576_d_n2, assign12880_body18_e16576_d_n6, assign12880_body18_e16576_d_n7, assign12880_body18_e16576_d_n10, assign12880_body18_e16576_d_n11, assign12880_body18_e16576_d_n12, assign12880_body18_e16576_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body18_e16572: f64 = (-locals.var_cnst0soi);
        let assign12880_body18_e16574: f64 = (assign12880_body18_e16572 * locals.var_t4_dpss__blk382);
        (assign12880_body18_e16574, (((-locals.var_cnst0soi_dn0) * locals.var_t4_dpss__blk382) + (assign12880_body18_e16572 * locals.var_t4_dpss__blk382_dn0)), (((-locals.var_cnst0soi_dn2) * locals.var_t4_dpss__blk382) + (assign12880_body18_e16572 * locals.var_t4_dpss__blk382_dn2)), (((-locals.var_cnst0soi_dn6) * locals.var_t4_dpss__blk382) + (assign12880_body18_e16572 * locals.var_t4_dpss__blk382_dn6)), (((-locals.var_cnst0soi_dn7) * locals.var_t4_dpss__blk382) + (assign12880_body18_e16572 * locals.var_t4_dpss__blk382_dn7)), (((-locals.var_cnst0soi_dn10) * locals.var_t4_dpss__blk382) + (assign12880_body18_e16572 * locals.var_t4_dpss__blk382_dn10)), (((-locals.var_cnst0soi_dn11) * locals.var_t4_dpss__blk382) + (assign12880_body18_e16572 * locals.var_t4_dpss__blk382_dn11)), (((-locals.var_cnst0soi_dn12) * locals.var_t4_dpss__blk382) + (assign12880_body18_e16572 * locals.var_t4_dpss__blk382_dn12)), (((-locals.var_cnst0soi_dn17) * locals.var_t4_dpss__blk382) + (assign12880_body18_e16572 * locals.var_t4_dpss__blk382_dn17)),)
    } else {
        (locals.var_q_nl_dpss, locals.var_q_nl_dpss_dn0, locals.var_q_nl_dpss_dn2, locals.var_q_nl_dpss_dn6, locals.var_q_nl_dpss_dn7, locals.var_q_nl_dpss_dn10, locals.var_q_nl_dpss_dn11, locals.var_q_nl_dpss_dn12, locals.var_q_nl_dpss_dn17,)
    }
};
            locals.var_q_nl_dpss = assign12880_body18_e16576;
            locals.var_q_nl_dpss_dn0 = assign12880_body18_e16576_d_n0;
            locals.var_q_nl_dpss_dn2 = assign12880_body18_e16576_d_n2;
            locals.var_q_nl_dpss_dn6 = assign12880_body18_e16576_d_n6;
            locals.var_q_nl_dpss_dn7 = assign12880_body18_e16576_d_n7;
            locals.var_q_nl_dpss_dn10 = assign12880_body18_e16576_d_n10;
            locals.var_q_nl_dpss_dn11 = assign12880_body18_e16576_d_n11;
            locals.var_q_nl_dpss_dn12 = assign12880_body18_e16576_d_n12;
            locals.var_q_nl_dpss_dn17 = assign12880_body18_e16576_d_n17;
            let (assign12880_body19_e16587, assign12880_body19_e16587_d_n0, assign12880_body19_e16587_d_n2, assign12880_body19_e16587_d_n6, assign12880_body19_e16587_d_n7, assign12880_body19_e16587_d_n10, assign12880_body19_e16587_d_n11, assign12880_body19_e16587_d_n12, assign12880_body19_e16587_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body19_e16583: f64 = (locals.var_phi_bl_soi - locals.var_phi_sl_soi);
        let assign12880_body19_e16585: f64 = (assign12880_body19_e16583 / locals.var_qdepb_dlt);
        (assign12880_body19_e16585, ((locals.var_phi_bl_soi_dn0 - locals.var_phi_sl_soi_dn0) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn2 - locals.var_phi_sl_soi_dn2) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn6 - locals.var_phi_sl_soi_dn6) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn7 - locals.var_phi_sl_soi_dn7) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn10 - locals.var_phi_sl_soi_dn10) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn11 - locals.var_phi_sl_soi_dn11) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn12 - locals.var_phi_sl_soi_dn12) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn17 - locals.var_phi_sl_soi_dn17) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_t1__blk349, locals.var_t1__blk349_dn0, locals.var_t1__blk349_dn2, locals.var_t1__blk349_dn6, locals.var_t1__blk349_dn7, locals.var_t1__blk349_dn10, locals.var_t1__blk349_dn11, locals.var_t1__blk349_dn12, locals.var_t1__blk349_dn17,)
    }
};
            locals.var_t1__blk349 = assign12880_body19_e16587;
            locals.var_t1__blk349_dn0 = assign12880_body19_e16587_d_n0;
            locals.var_t1__blk349_dn2 = assign12880_body19_e16587_d_n2;
            locals.var_t1__blk349_dn6 = assign12880_body19_e16587_d_n6;
            locals.var_t1__blk349_dn7 = assign12880_body19_e16587_d_n7;
            locals.var_t1__blk349_dn10 = assign12880_body19_e16587_d_n10;
            locals.var_t1__blk349_dn11 = assign12880_body19_e16587_d_n11;
            locals.var_t1__blk349_dn12 = assign12880_body19_e16587_d_n12;
            locals.var_t1__blk349_dn17 = assign12880_body19_e16587_d_n17;
            let (assign12880_body20_e16596, assign12880_body20_e16596_d_n0, assign12880_body20_e16596_d_n2, assign12880_body20_e16596_d_n6, assign12880_body20_e16596_d_n7, assign12880_body20_e16596_d_n10, assign12880_body20_e16596_d_n11, assign12880_body20_e16596_d_n12, assign12880_body20_e16596_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body20_e16594: f64 = (locals.var_beta * locals.var_t1__blk349);
        (assign12880_body20_e16594, (locals.var_beta * locals.var_t1__blk349_dn0), (locals.var_beta * locals.var_t1__blk349_dn2), (locals.var_beta * locals.var_t1__blk349_dn6), (locals.var_beta * locals.var_t1__blk349_dn7), ((locals.var_beta_dn10 * locals.var_t1__blk349) + (locals.var_beta * locals.var_t1__blk349_dn10)), (locals.var_beta * locals.var_t1__blk349_dn11), (locals.var_beta * locals.var_t1__blk349_dn12), (locals.var_beta * locals.var_t1__blk349_dn17),)
    } else {
        (locals.var_el, locals.var_el_dn0, locals.var_el_dn2, locals.var_el_dn6, locals.var_el_dn7, locals.var_el_dn10, locals.var_el_dn11, locals.var_el_dn12, locals.var_el_dn17,)
    }
};
            locals.var_el = assign12880_body20_e16596;
            locals.var_el_dn0 = assign12880_body20_e16596_d_n0;
            locals.var_el_dn2 = assign12880_body20_e16596_d_n2;
            locals.var_el_dn6 = assign12880_body20_e16596_d_n6;
            locals.var_el_dn7 = assign12880_body20_e16596_d_n7;
            locals.var_el_dn10 = assign12880_body20_e16596_d_n10;
            locals.var_el_dn11 = assign12880_body20_e16596_d_n11;
            locals.var_el_dn12 = assign12880_body20_e16596_d_n12;
            locals.var_el_dn17 = assign12880_body20_e16596_d_n17;
            let assign12880_body21_e16598: f64 = (-locals.var_el);
            let assign12880_body21_e16600: f64 = if assign12880_body21_e16598 >= 500.0 { 1.0 } else { 0.0 };
            locals.var_guard387 = assign12880_body21_e16600;
            let (assign12880_body22_e16616, assign12880_body22_e16616_d_n0, assign12880_body22_e16616_d_n2, assign12880_body22_e16616_d_n6, assign12880_body22_e16616_d_n7, assign12880_body22_e16616_d_n10, assign12880_body22_e16616_d_n11, assign12880_body22_e16616_d_n12, assign12880_body22_e16616_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign12880_body22_e16610: f64 = (-locals.var_el);
        let assign12880_body22_e16611: f64 = (1.0 + assign12880_body22_e16610);
        let assign12880_body22_e16613: f64 = (assign12880_body22_e16611 - 500.0);
        let assign12880_body22_e16614: f64 = (1.403592217853e217 * assign12880_body22_e16613);
        (assign12880_body22_e16614, (1.403592217853e217 * (-locals.var_el_dn0)), (1.403592217853e217 * (-locals.var_el_dn2)), (1.403592217853e217 * (-locals.var_el_dn6)), (1.403592217853e217 * (-locals.var_el_dn7)), (1.403592217853e217 * (-locals.var_el_dn10)), (1.403592217853e217 * (-locals.var_el_dn11)), (1.403592217853e217 * (-locals.var_el_dn12)), (1.403592217853e217 * (-locals.var_el_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body22_e16616;
            locals.var_t0_dn0 = assign12880_body22_e16616_d_n0;
            locals.var_t0_dn2 = assign12880_body22_e16616_d_n2;
            locals.var_t0_dn6 = assign12880_body22_e16616_d_n6;
            locals.var_t0_dn7 = assign12880_body22_e16616_d_n7;
            locals.var_t0_dn10 = assign12880_body22_e16616_d_n10;
            locals.var_t0_dn11 = assign12880_body22_e16616_d_n11;
            locals.var_t0_dn12 = assign12880_body22_e16616_d_n12;
            locals.var_t0_dn17 = assign12880_body22_e16616_d_n17;
            let (assign12880_body23_e16625, assign12880_body23_e16625_d_n0, assign12880_body23_e16625_d_n2, assign12880_body23_e16625_d_n6, assign12880_body23_e16625_d_n7, assign12880_body23_e16625_d_n10, assign12880_body23_e16625_d_n11, assign12880_body23_e16625_d_n12, assign12880_body23_e16625_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign12880_body23_e16625;
            locals.var_t6_dn0 = assign12880_body23_e16625_d_n0;
            locals.var_t6_dn2 = assign12880_body23_e16625_d_n2;
            locals.var_t6_dn6 = assign12880_body23_e16625_d_n6;
            locals.var_t6_dn7 = assign12880_body23_e16625_d_n7;
            locals.var_t6_dn10 = assign12880_body23_e16625_d_n10;
            locals.var_t6_dn11 = assign12880_body23_e16625_d_n11;
            locals.var_t6_dn12 = assign12880_body23_e16625_d_n12;
            locals.var_t6_dn17 = assign12880_body23_e16625_d_n17;
            let (assign12880_body24_e16636, assign12880_body24_e16636_d_n0, assign12880_body24_e16636_d_n2, assign12880_body24_e16636_d_n6, assign12880_body24_e16636_d_n7, assign12880_body24_e16636_d_n10, assign12880_body24_e16636_d_n11, assign12880_body24_e16636_d_n12, assign12880_body24_e16636_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign12880_body24_e16634: f64 = (-locals.var_el);
        (assign12880_body24_e16634, (-locals.var_el_dn0), (-locals.var_el_dn2), (-locals.var_el_dn6), (-locals.var_el_dn7), (-locals.var_el_dn10), (-locals.var_el_dn11), (-locals.var_el_dn12), (-locals.var_el_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12880_body24_e16636;
            locals.var_tmf1_dn0 = assign12880_body24_e16636_d_n0;
            locals.var_tmf1_dn2 = assign12880_body24_e16636_d_n2;
            locals.var_tmf1_dn6 = assign12880_body24_e16636_d_n6;
            locals.var_tmf1_dn7 = assign12880_body24_e16636_d_n7;
            locals.var_tmf1_dn10 = assign12880_body24_e16636_d_n10;
            locals.var_tmf1_dn11 = assign12880_body24_e16636_d_n11;
            locals.var_tmf1_dn12 = assign12880_body24_e16636_d_n12;
            locals.var_tmf1_dn17 = assign12880_body24_e16636_d_n17;
            let (assign12880_body25_e16646, assign12880_body25_e16646_d_n0, assign12880_body25_e16646_d_n2, assign12880_body25_e16646_d_n6, assign12880_body25_e16646_d_n7, assign12880_body25_e16646_d_n10, assign12880_body25_e16646_d_n11, assign12880_body25_e16646_d_n12, assign12880_body25_e16646_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body25_e16646;
            locals.var_t0_dn0 = assign12880_body25_e16646_d_n0;
            locals.var_t0_dn2 = assign12880_body25_e16646_d_n2;
            locals.var_t0_dn6 = assign12880_body25_e16646_d_n6;
            locals.var_t0_dn7 = assign12880_body25_e16646_d_n7;
            locals.var_t0_dn10 = assign12880_body25_e16646_d_n10;
            locals.var_t0_dn11 = assign12880_body25_e16646_d_n11;
            locals.var_t0_dn12 = assign12880_body25_e16646_d_n12;
            locals.var_t0_dn17 = assign12880_body25_e16646_d_n17;
            let mut assign12880_body26_loop_guard: usize = 0;
            while {
                let assign12880_body26_cond_e16657: f64 = if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
                assign12880_body26_cond_e16657 != 0.0
            } {
                assign12880_body26_loop_guard += 1;
                assert!(assign12880_body26_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12880_body26_body0_e16669, assign12880_body26_body0_e16669_d_n0, assign12880_body26_body0_e16669_d_n2, assign12880_body26_body0_e16669_d_n6, assign12880_body26_body0_e16669_d_n7, assign12880_body26_body0_e16669_d_n10, assign12880_body26_body0_e16669_d_n11, assign12880_body26_body0_e16669_d_n12, assign12880_body26_body0_e16669_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign12880_body26_body0_e16667: f64 = (locals.var_t0 * 1.14200738981568e26);
        (assign12880_body26_body0_e16667, (locals.var_t0_dn0 * 1.14200738981568e26), (locals.var_t0_dn2 * 1.14200738981568e26), (locals.var_t0_dn6 * 1.14200738981568e26), (locals.var_t0_dn7 * 1.14200738981568e26), (locals.var_t0_dn10 * 1.14200738981568e26), (locals.var_t0_dn11 * 1.14200738981568e26), (locals.var_t0_dn12 * 1.14200738981568e26), (locals.var_t0_dn17 * 1.14200738981568e26),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
                locals.var_t0 = assign12880_body26_body0_e16669;
                locals.var_t0_dn0 = assign12880_body26_body0_e16669_d_n0;
                locals.var_t0_dn2 = assign12880_body26_body0_e16669_d_n2;
                locals.var_t0_dn6 = assign12880_body26_body0_e16669_d_n6;
                locals.var_t0_dn7 = assign12880_body26_body0_e16669_d_n7;
                locals.var_t0_dn10 = assign12880_body26_body0_e16669_d_n10;
                locals.var_t0_dn11 = assign12880_body26_body0_e16669_d_n11;
                locals.var_t0_dn12 = assign12880_body26_body0_e16669_d_n12;
                locals.var_t0_dn17 = assign12880_body26_body0_e16669_d_n17;
                let (assign12880_body26_body1_e16681, assign12880_body26_body1_e16681_d_n0, assign12880_body26_body1_e16681_d_n2, assign12880_body26_body1_e16681_d_n6, assign12880_body26_body1_e16681_d_n7, assign12880_body26_body1_e16681_d_n10, assign12880_body26_body1_e16681_d_n11, assign12880_body26_body1_e16681_d_n12, assign12880_body26_body1_e16681_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign12880_body26_body1_e16679: f64 = (locals.var_tmf1 - 60.0);
        (assign12880_body26_body1_e16679, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
                locals.var_tmf1 = assign12880_body26_body1_e16681;
                locals.var_tmf1_dn0 = assign12880_body26_body1_e16681_d_n0;
                locals.var_tmf1_dn2 = assign12880_body26_body1_e16681_d_n2;
                locals.var_tmf1_dn6 = assign12880_body26_body1_e16681_d_n6;
                locals.var_tmf1_dn7 = assign12880_body26_body1_e16681_d_n7;
                locals.var_tmf1_dn10 = assign12880_body26_body1_e16681_d_n10;
                locals.var_tmf1_dn11 = assign12880_body26_body1_e16681_d_n11;
                locals.var_tmf1_dn12 = assign12880_body26_body1_e16681_d_n12;
                locals.var_tmf1_dn17 = assign12880_body26_body1_e16681_d_n17;
            }
            let (assign12880_body27_e16694, assign12880_body27_e16694_d_n0, assign12880_body27_e16694_d_n2, assign12880_body27_e16694_d_n6, assign12880_body27_e16694_d_n7, assign12880_body27_e16694_d_n10, assign12880_body27_e16694_d_n11, assign12880_body27_e16694_d_n12, assign12880_body27_e16694_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign12880_body27_e16691: f64 = (locals.var_tmf1).exp();
        let assign12880_body27_e16692: f64 = (locals.var_t0 * assign12880_body27_e16691);
        (assign12880_body27_e16692, ((locals.var_t0_dn0 * assign12880_body27_e16691) + (locals.var_t0 * (assign12880_body27_e16691 * locals.var_tmf1_dn0))), ((locals.var_t0_dn2 * assign12880_body27_e16691) + (locals.var_t0 * (assign12880_body27_e16691 * locals.var_tmf1_dn2))), ((locals.var_t0_dn6 * assign12880_body27_e16691) + (locals.var_t0 * (assign12880_body27_e16691 * locals.var_tmf1_dn6))), ((locals.var_t0_dn7 * assign12880_body27_e16691) + (locals.var_t0 * (assign12880_body27_e16691 * locals.var_tmf1_dn7))), ((locals.var_t0_dn10 * assign12880_body27_e16691) + (locals.var_t0 * (assign12880_body27_e16691 * locals.var_tmf1_dn10))), ((locals.var_t0_dn11 * assign12880_body27_e16691) + (locals.var_t0 * (assign12880_body27_e16691 * locals.var_tmf1_dn11))), ((locals.var_t0_dn12 * assign12880_body27_e16691) + (locals.var_t0 * (assign12880_body27_e16691 * locals.var_tmf1_dn12))), ((locals.var_t0_dn17 * assign12880_body27_e16691) + (locals.var_t0 * (assign12880_body27_e16691 * locals.var_tmf1_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body27_e16694;
            locals.var_t0_dn0 = assign12880_body27_e16694_d_n0;
            locals.var_t0_dn2 = assign12880_body27_e16694_d_n2;
            locals.var_t0_dn6 = assign12880_body27_e16694_d_n6;
            locals.var_t0_dn7 = assign12880_body27_e16694_d_n7;
            locals.var_t0_dn10 = assign12880_body27_e16694_d_n10;
            locals.var_t0_dn11 = assign12880_body27_e16694_d_n11;
            locals.var_t0_dn12 = assign12880_body27_e16694_d_n12;
            locals.var_t0_dn17 = assign12880_body27_e16694_d_n17;
            let (assign12880_body28_e16704, assign12880_body28_e16704_d_n0, assign12880_body28_e16704_d_n2, assign12880_body28_e16704_d_n6, assign12880_body28_e16704_d_n7, assign12880_body28_e16704_d_n10, assign12880_body28_e16704_d_n11, assign12880_body28_e16704_d_n12, assign12880_body28_e16704_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard387 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign12880_body28_e16704;
            locals.var_t6_dn0 = assign12880_body28_e16704_d_n0;
            locals.var_t6_dn2 = assign12880_body28_e16704_d_n2;
            locals.var_t6_dn6 = assign12880_body28_e16704_d_n6;
            locals.var_t6_dn7 = assign12880_body28_e16704_d_n7;
            locals.var_t6_dn10 = assign12880_body28_e16704_d_n10;
            locals.var_t6_dn11 = assign12880_body28_e16704_d_n11;
            locals.var_t6_dn12 = assign12880_body28_e16704_d_n12;
            locals.var_t6_dn17 = assign12880_body28_e16704_d_n17;
            let (assign12880_body29_e16716, assign12880_body29_e16716_d_n0, assign12880_body29_e16716_d_n2, assign12880_body29_e16716_d_n6, assign12880_body29_e16716_d_n7, assign12880_body29_e16716_d_n10, assign12880_body29_e16716_d_n11, assign12880_body29_e16716_d_n12, assign12880_body29_e16716_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body29_e16711: f64 = (locals.var_t0 + locals.var_el);
        let assign12880_body29_e16713: f64 = (assign12880_body29_e16711 - 1.0);
        let assign12880_body29_e16714: f64 = (assign12880_body29_e16713).sqrt();
        (assign12880_body29_e16714, ((locals.var_t0_dn0 + locals.var_el_dn0) / (2.0 * assign12880_body29_e16714)), ((locals.var_t0_dn2 + locals.var_el_dn2) / (2.0 * assign12880_body29_e16714)), ((locals.var_t0_dn6 + locals.var_el_dn6) / (2.0 * assign12880_body29_e16714)), ((locals.var_t0_dn7 + locals.var_el_dn7) / (2.0 * assign12880_body29_e16714)), ((locals.var_t0_dn10 + locals.var_el_dn10) / (2.0 * assign12880_body29_e16714)), ((locals.var_t0_dn11 + locals.var_el_dn11) / (2.0 * assign12880_body29_e16714)), ((locals.var_t0_dn12 + locals.var_el_dn12) / (2.0 * assign12880_body29_e16714)), ((locals.var_t0_dn17 + locals.var_el_dn17) / (2.0 * assign12880_body29_e16714)),)
    } else {
        (locals.var_t2__blk350, locals.var_t2__blk350_dn0, locals.var_t2__blk350_dn2, locals.var_t2__blk350_dn6, locals.var_t2__blk350_dn7, locals.var_t2__blk350_dn10, locals.var_t2__blk350_dn11, locals.var_t2__blk350_dn12, locals.var_t2__blk350_dn17,)
    }
};
            locals.var_t2__blk350 = assign12880_body29_e16716;
            locals.var_t2__blk350_dn0 = assign12880_body29_e16716_d_n0;
            locals.var_t2__blk350_dn2 = assign12880_body29_e16716_d_n2;
            locals.var_t2__blk350_dn6 = assign12880_body29_e16716_d_n6;
            locals.var_t2__blk350_dn7 = assign12880_body29_e16716_d_n7;
            locals.var_t2__blk350_dn10 = assign12880_body29_e16716_d_n10;
            locals.var_t2__blk350_dn11 = assign12880_body29_e16716_d_n11;
            locals.var_t2__blk350_dn12 = assign12880_body29_e16716_d_n12;
            locals.var_t2__blk350_dn17 = assign12880_body29_e16716_d_n17;
            let assign12880_body30_e16719: f64 = (-1e-9);
            let assign12880_body30_e16720: f64 = if locals.var_t1__blk349 < assign12880_body30_e16719 { 1.0 } else { 0.0 };
            locals.var_guard388 = assign12880_body30_e16720;
            let (assign12880_body31_e16731, assign12880_body31_e16731_d_n0, assign12880_body31_e16731_d_n2, assign12880_body31_e16731_d_n6, assign12880_body31_e16731_d_n7, assign12880_body31_e16731_d_n10, assign12880_body31_e16731_d_n11, assign12880_body31_e16731_d_n12, assign12880_body31_e16731_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign12880_body31_e16729: f64 = (locals.var_cnst0soi * locals.var_t2__blk350);
        (assign12880_body31_e16729, ((locals.var_cnst0soi_dn0 * locals.var_t2__blk350) + (locals.var_cnst0soi * locals.var_t2__blk350_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_t2__blk350) + (locals.var_cnst0soi * locals.var_t2__blk350_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_t2__blk350) + (locals.var_cnst0soi * locals.var_t2__blk350_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_t2__blk350) + (locals.var_cnst0soi * locals.var_t2__blk350_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_t2__blk350) + (locals.var_cnst0soi * locals.var_t2__blk350_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_t2__blk350) + (locals.var_cnst0soi * locals.var_t2__blk350_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_t2__blk350) + (locals.var_cnst0soi * locals.var_t2__blk350_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_t2__blk350) + (locals.var_cnst0soi * locals.var_t2__blk350_dn17)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12880_body31_e16731;
            locals.var_q_bl_dep_dn0 = assign12880_body31_e16731_d_n0;
            locals.var_q_bl_dep_dn2 = assign12880_body31_e16731_d_n2;
            locals.var_q_bl_dep_dn6 = assign12880_body31_e16731_d_n6;
            locals.var_q_bl_dep_dn7 = assign12880_body31_e16731_d_n7;
            locals.var_q_bl_dep_dn10 = assign12880_body31_e16731_d_n10;
            locals.var_q_bl_dep_dn11 = assign12880_body31_e16731_d_n11;
            locals.var_q_bl_dep_dn12 = assign12880_body31_e16731_d_n12;
            locals.var_q_bl_dep_dn17 = assign12880_body31_e16731_d_n17;
            let (assign12880_body32_e16753, assign12880_body32_e16753_d_n0, assign12880_body32_e16753_d_n2, assign12880_body32_e16753_d_n6, assign12880_body32_e16753_d_n7, assign12880_body32_e16753_d_n10, assign12880_body32_e16753_d_n11, assign12880_body32_e16753_d_n12, assign12880_body32_e16753_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign12880_body32_e16740: f64 = (locals.var_cnst0soi * locals.var_beta);
        let assign12880_body32_e16742: f64 = (-locals.var_t6);
        let assign12880_body32_e16744: f64 = (assign12880_body32_e16742 + 1.0);
        let assign12880_body32_e16745: f64 = (assign12880_body32_e16740 * assign12880_body32_e16744);
        let assign12880_body32_e16748: f64 = (2.0 * locals.var_t2__blk350);
        let assign12880_body32_e16749: f64 = (assign12880_body32_e16745 / assign12880_body32_e16748);
        let assign12880_body32_e16751: f64 = (assign12880_body32_e16749 / locals.var_qdepb_dlt);
        (assign12880_body32_e16751, (((((((locals.var_cnst0soi_dn0 * locals.var_beta) * assign12880_body32_e16744) + (assign12880_body32_e16740 * (-locals.var_t6_dn0))) * assign12880_body32_e16748) - (assign12880_body32_e16745 * (2.0 * locals.var_t2__blk350_dn0))) / (assign12880_body32_e16748 * assign12880_body32_e16748)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn2 * locals.var_beta) * assign12880_body32_e16744) + (assign12880_body32_e16740 * (-locals.var_t6_dn2))) * assign12880_body32_e16748) - (assign12880_body32_e16745 * (2.0 * locals.var_t2__blk350_dn2))) / (assign12880_body32_e16748 * assign12880_body32_e16748)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn6 * locals.var_beta) * assign12880_body32_e16744) + (assign12880_body32_e16740 * (-locals.var_t6_dn6))) * assign12880_body32_e16748) - (assign12880_body32_e16745 * (2.0 * locals.var_t2__blk350_dn6))) / (assign12880_body32_e16748 * assign12880_body32_e16748)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn7 * locals.var_beta) * assign12880_body32_e16744) + (assign12880_body32_e16740 * (-locals.var_t6_dn7))) * assign12880_body32_e16748) - (assign12880_body32_e16745 * (2.0 * locals.var_t2__blk350_dn7))) / (assign12880_body32_e16748 * assign12880_body32_e16748)) / locals.var_qdepb_dlt), ((((((((locals.var_cnst0soi_dn10 * locals.var_beta) + (locals.var_cnst0soi * locals.var_beta_dn10)) * assign12880_body32_e16744) + (assign12880_body32_e16740 * (-locals.var_t6_dn10))) * assign12880_body32_e16748) - (assign12880_body32_e16745 * (2.0 * locals.var_t2__blk350_dn10))) / (assign12880_body32_e16748 * assign12880_body32_e16748)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn11 * locals.var_beta) * assign12880_body32_e16744) + (assign12880_body32_e16740 * (-locals.var_t6_dn11))) * assign12880_body32_e16748) - (assign12880_body32_e16745 * (2.0 * locals.var_t2__blk350_dn11))) / (assign12880_body32_e16748 * assign12880_body32_e16748)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn12 * locals.var_beta) * assign12880_body32_e16744) + (assign12880_body32_e16740 * (-locals.var_t6_dn12))) * assign12880_body32_e16748) - (assign12880_body32_e16745 * (2.0 * locals.var_t2__blk350_dn12))) / (assign12880_body32_e16748 * assign12880_body32_e16748)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn17 * locals.var_beta) * assign12880_body32_e16744) + (assign12880_body32_e16740 * (-locals.var_t6_dn17))) * assign12880_body32_e16748) - (assign12880_body32_e16745 * (2.0 * locals.var_t2__blk350_dn17))) / (assign12880_body32_e16748 * assign12880_body32_e16748)) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12880_body32_e16753;
            locals.var_q_bl_dep_dpbs_dn0 = assign12880_body32_e16753_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12880_body32_e16753_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12880_body32_e16753_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12880_body32_e16753_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12880_body32_e16753_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12880_body32_e16753_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12880_body32_e16753_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12880_body32_e16753_d_n17;
            let (assign12880_body33_e16763, assign12880_body33_e16763_d_n0, assign12880_body33_e16763_d_n2, assign12880_body33_e16763_d_n6, assign12880_body33_e16763_d_n7, assign12880_body33_e16763_d_n10, assign12880_body33_e16763_d_n11, assign12880_body33_e16763_d_n12, assign12880_body33_e16763_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign12880_body33_e16761: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12880_body33_e16761, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12880_body33_e16763;
            locals.var_q_bl_dep_dpss_dn0 = assign12880_body33_e16763_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12880_body33_e16763_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12880_body33_e16763_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12880_body33_e16763_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12880_body33_e16763_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12880_body33_e16763_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12880_body33_e16763_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12880_body33_e16763_d_n17;
            let assign12880_body34_e16766: f64 = if locals.var_t1__blk349 > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard389 = assign12880_body34_e16766;
            let (assign12880_body35_e16781, assign12880_body35_e16781_d_n0, assign12880_body35_e16781_d_n2, assign12880_body35_e16781_d_n6, assign12880_body35_e16781_d_n7, assign12880_body35_e16781_d_n10, assign12880_body35_e16781_d_n11, assign12880_body35_e16781_d_n12, assign12880_body35_e16781_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 == 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign12880_body35_e16777: f64 = (-locals.var_cnst0soi);
        let assign12880_body35_e16779: f64 = (assign12880_body35_e16777 * locals.var_t2__blk350);
        (assign12880_body35_e16779, (((-locals.var_cnst0soi_dn0) * locals.var_t2__blk350) + (assign12880_body35_e16777 * locals.var_t2__blk350_dn0)), (((-locals.var_cnst0soi_dn2) * locals.var_t2__blk350) + (assign12880_body35_e16777 * locals.var_t2__blk350_dn2)), (((-locals.var_cnst0soi_dn6) * locals.var_t2__blk350) + (assign12880_body35_e16777 * locals.var_t2__blk350_dn6)), (((-locals.var_cnst0soi_dn7) * locals.var_t2__blk350) + (assign12880_body35_e16777 * locals.var_t2__blk350_dn7)), (((-locals.var_cnst0soi_dn10) * locals.var_t2__blk350) + (assign12880_body35_e16777 * locals.var_t2__blk350_dn10)), (((-locals.var_cnst0soi_dn11) * locals.var_t2__blk350) + (assign12880_body35_e16777 * locals.var_t2__blk350_dn11)), (((-locals.var_cnst0soi_dn12) * locals.var_t2__blk350) + (assign12880_body35_e16777 * locals.var_t2__blk350_dn12)), (((-locals.var_cnst0soi_dn17) * locals.var_t2__blk350) + (assign12880_body35_e16777 * locals.var_t2__blk350_dn17)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12880_body35_e16781;
            locals.var_q_bl_dep_dn0 = assign12880_body35_e16781_d_n0;
            locals.var_q_bl_dep_dn2 = assign12880_body35_e16781_d_n2;
            locals.var_q_bl_dep_dn6 = assign12880_body35_e16781_d_n6;
            locals.var_q_bl_dep_dn7 = assign12880_body35_e16781_d_n7;
            locals.var_q_bl_dep_dn10 = assign12880_body35_e16781_d_n10;
            locals.var_q_bl_dep_dn11 = assign12880_body35_e16781_d_n11;
            locals.var_q_bl_dep_dn12 = assign12880_body35_e16781_d_n12;
            locals.var_q_bl_dep_dn17 = assign12880_body35_e16781_d_n17;
            let (assign12880_body36_e16807, assign12880_body36_e16807_d_n0, assign12880_body36_e16807_d_n2, assign12880_body36_e16807_d_n6, assign12880_body36_e16807_d_n7, assign12880_body36_e16807_d_n10, assign12880_body36_e16807_d_n11, assign12880_body36_e16807_d_n12, assign12880_body36_e16807_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 == 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign12880_body36_e16792: f64 = (-locals.var_cnst0soi);
        let assign12880_body36_e16794: f64 = (assign12880_body36_e16792 * locals.var_beta);
        let assign12880_body36_e16796: f64 = (-locals.var_t6);
        let assign12880_body36_e16798: f64 = (assign12880_body36_e16796 + 1.0);
        let assign12880_body36_e16799: f64 = (assign12880_body36_e16794 * assign12880_body36_e16798);
        let assign12880_body36_e16802: f64 = (2.0 * locals.var_t2__blk350);
        let assign12880_body36_e16803: f64 = (assign12880_body36_e16799 / assign12880_body36_e16802);
        let assign12880_body36_e16805: f64 = (assign12880_body36_e16803 / locals.var_qdepb_dlt);
        (assign12880_body36_e16805, ((((((((-locals.var_cnst0soi_dn0) * locals.var_beta) * assign12880_body36_e16798) + (assign12880_body36_e16794 * (-locals.var_t6_dn0))) * assign12880_body36_e16802) - (assign12880_body36_e16799 * (2.0 * locals.var_t2__blk350_dn0))) / (assign12880_body36_e16802 * assign12880_body36_e16802)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn2) * locals.var_beta) * assign12880_body36_e16798) + (assign12880_body36_e16794 * (-locals.var_t6_dn2))) * assign12880_body36_e16802) - (assign12880_body36_e16799 * (2.0 * locals.var_t2__blk350_dn2))) / (assign12880_body36_e16802 * assign12880_body36_e16802)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn6) * locals.var_beta) * assign12880_body36_e16798) + (assign12880_body36_e16794 * (-locals.var_t6_dn6))) * assign12880_body36_e16802) - (assign12880_body36_e16799 * (2.0 * locals.var_t2__blk350_dn6))) / (assign12880_body36_e16802 * assign12880_body36_e16802)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn7) * locals.var_beta) * assign12880_body36_e16798) + (assign12880_body36_e16794 * (-locals.var_t6_dn7))) * assign12880_body36_e16802) - (assign12880_body36_e16799 * (2.0 * locals.var_t2__blk350_dn7))) / (assign12880_body36_e16802 * assign12880_body36_e16802)) / locals.var_qdepb_dlt), (((((((((-locals.var_cnst0soi_dn10) * locals.var_beta) + (assign12880_body36_e16792 * locals.var_beta_dn10)) * assign12880_body36_e16798) + (assign12880_body36_e16794 * (-locals.var_t6_dn10))) * assign12880_body36_e16802) - (assign12880_body36_e16799 * (2.0 * locals.var_t2__blk350_dn10))) / (assign12880_body36_e16802 * assign12880_body36_e16802)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn11) * locals.var_beta) * assign12880_body36_e16798) + (assign12880_body36_e16794 * (-locals.var_t6_dn11))) * assign12880_body36_e16802) - (assign12880_body36_e16799 * (2.0 * locals.var_t2__blk350_dn11))) / (assign12880_body36_e16802 * assign12880_body36_e16802)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn12) * locals.var_beta) * assign12880_body36_e16798) + (assign12880_body36_e16794 * (-locals.var_t6_dn12))) * assign12880_body36_e16802) - (assign12880_body36_e16799 * (2.0 * locals.var_t2__blk350_dn12))) / (assign12880_body36_e16802 * assign12880_body36_e16802)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn17) * locals.var_beta) * assign12880_body36_e16798) + (assign12880_body36_e16794 * (-locals.var_t6_dn17))) * assign12880_body36_e16802) - (assign12880_body36_e16799 * (2.0 * locals.var_t2__blk350_dn17))) / (assign12880_body36_e16802 * assign12880_body36_e16802)) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12880_body36_e16807;
            locals.var_q_bl_dep_dpbs_dn0 = assign12880_body36_e16807_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12880_body36_e16807_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12880_body36_e16807_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12880_body36_e16807_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12880_body36_e16807_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12880_body36_e16807_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12880_body36_e16807_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12880_body36_e16807_d_n17;
            let (assign12880_body37_e16820, assign12880_body37_e16820_d_n0, assign12880_body37_e16820_d_n2, assign12880_body37_e16820_d_n6, assign12880_body37_e16820_d_n7, assign12880_body37_e16820_d_n10, assign12880_body37_e16820_d_n11, assign12880_body37_e16820_d_n12, assign12880_body37_e16820_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 == 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign12880_body37_e16818: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12880_body37_e16818, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12880_body37_e16820;
            locals.var_q_bl_dep_dpss_dn0 = assign12880_body37_e16820_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12880_body37_e16820_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12880_body37_e16820_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12880_body37_e16820_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12880_body37_e16820_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12880_body37_e16820_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12880_body37_e16820_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12880_body37_e16820_d_n17;
            let (assign12880_body38_e16838, assign12880_body38_e16838_d_n0, assign12880_body38_e16838_d_n2, assign12880_body38_e16838_d_n6, assign12880_body38_e16838_d_n7, assign12880_body38_e16838_d_n10, assign12880_body38_e16838_d_n11, assign12880_body38_e16838_d_n12, assign12880_body38_e16838_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 == 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign12880_body38_e16832: f64 = (-locals.var_cnst0soi);
        let assign12880_body38_e16834: f64 = (assign12880_body38_e16832 * locals.var_el);
        let assign12880_body38_e16836: f64 = (assign12880_body38_e16834 / 1.414213562373095);
        (assign12880_body38_e16836, ((((-locals.var_cnst0soi_dn0) * locals.var_el) + (assign12880_body38_e16832 * locals.var_el_dn0)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn2) * locals.var_el) + (assign12880_body38_e16832 * locals.var_el_dn2)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn6) * locals.var_el) + (assign12880_body38_e16832 * locals.var_el_dn6)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn7) * locals.var_el) + (assign12880_body38_e16832 * locals.var_el_dn7)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn10) * locals.var_el) + (assign12880_body38_e16832 * locals.var_el_dn10)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn11) * locals.var_el) + (assign12880_body38_e16832 * locals.var_el_dn11)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn12) * locals.var_el) + (assign12880_body38_e16832 * locals.var_el_dn12)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn17) * locals.var_el) + (assign12880_body38_e16832 * locals.var_el_dn17)) / 1.414213562373095),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12880_body38_e16838;
            locals.var_q_bl_dep_dn0 = assign12880_body38_e16838_d_n0;
            locals.var_q_bl_dep_dn2 = assign12880_body38_e16838_d_n2;
            locals.var_q_bl_dep_dn6 = assign12880_body38_e16838_d_n6;
            locals.var_q_bl_dep_dn7 = assign12880_body38_e16838_d_n7;
            locals.var_q_bl_dep_dn10 = assign12880_body38_e16838_d_n10;
            locals.var_q_bl_dep_dn11 = assign12880_body38_e16838_d_n11;
            locals.var_q_bl_dep_dn12 = assign12880_body38_e16838_d_n12;
            locals.var_q_bl_dep_dn17 = assign12880_body38_e16838_d_n17;
            let (assign12880_body39_e16856, assign12880_body39_e16856_d_n0, assign12880_body39_e16856_d_n2, assign12880_body39_e16856_d_n6, assign12880_body39_e16856_d_n7, assign12880_body39_e16856_d_n10, assign12880_body39_e16856_d_n11, assign12880_body39_e16856_d_n12, assign12880_body39_e16856_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 == 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign12880_body39_e16850: f64 = (-locals.var_cnst0soi);
        let assign12880_body39_e16852: f64 = (assign12880_body39_e16850 * locals.var_beta);
        let assign12880_body39_e16854: f64 = (assign12880_body39_e16852 / 1.414213562373095);
        (assign12880_body39_e16854, (((-locals.var_cnst0soi_dn0) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn2) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn6) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn7) * locals.var_beta) / 1.414213562373095), ((((-locals.var_cnst0soi_dn10) * locals.var_beta) + (assign12880_body39_e16850 * locals.var_beta_dn10)) / 1.414213562373095), (((-locals.var_cnst0soi_dn11) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn12) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn17) * locals.var_beta) / 1.414213562373095),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12880_body39_e16856;
            locals.var_q_bl_dep_dpbs_dn0 = assign12880_body39_e16856_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12880_body39_e16856_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12880_body39_e16856_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12880_body39_e16856_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12880_body39_e16856_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12880_body39_e16856_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12880_body39_e16856_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12880_body39_e16856_d_n17;
            let (assign12880_body40_e16870, assign12880_body40_e16870_d_n0, assign12880_body40_e16870_d_n2, assign12880_body40_e16870_d_n6, assign12880_body40_e16870_d_n7, assign12880_body40_e16870_d_n10, assign12880_body40_e16870_d_n11, assign12880_body40_e16870_d_n12, assign12880_body40_e16870_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard388 == 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign12880_body40_e16868: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12880_body40_e16868, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12880_body40_e16870;
            locals.var_q_bl_dep_dpss_dn0 = assign12880_body40_e16870_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12880_body40_e16870_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12880_body40_e16870_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12880_body40_e16870_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12880_body40_e16870_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12880_body40_e16870_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12880_body40_e16870_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12880_body40_e16870_d_n17;
            let assign12880_body41_e16874: f64 = (-locals.var_q_wdsoi_max);
            let assign12880_body41_e16876: f64 = assign12880_body41_e16874;
            let assign12880_body41_e16877: f64 = (-assign12880_body41_e16876);
            let assign12880_body41_e16880: f64 = (-locals.var_q_wdsoi_max);
            let assign12880_body41_e16882: f64 = assign12880_body41_e16880;
            let assign12880_body41_e16885: f64 = if ((locals.var_q_bl_dep > assign12880_body41_e16877) && (assign12880_body41_e16882 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard390 = assign12880_body41_e16885;
            let (assign12880_body42_e16901, assign12880_body42_e16901_d_n0, assign12880_body42_e16901_d_n2, assign12880_body42_e16901_d_n6, assign12880_body42_e16901_d_n7, assign12880_body42_e16901_d_n10, assign12880_body42_e16901_d_n11, assign12880_body42_e16901_d_n12, assign12880_body42_e16901_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body42_e16894: f64 = locals.var_q_bl_dep;
        let assign12880_body42_e16896: f64 = (-locals.var_q_wdsoi_max);
        let assign12880_body42_e16898: f64 = assign12880_body42_e16896;
        let assign12880_body42_e16899: f64 = (assign12880_body42_e16894 + assign12880_body42_e16898);
        (assign12880_body42_e16899, (locals.var_q_bl_dep_dn0 + (-locals.var_q_wdsoi_max_dn0)), (locals.var_q_bl_dep_dn2 + (-locals.var_q_wdsoi_max_dn2)), (locals.var_q_bl_dep_dn6 + (-locals.var_q_wdsoi_max_dn6)), (locals.var_q_bl_dep_dn7 + (-locals.var_q_wdsoi_max_dn7)), (locals.var_q_bl_dep_dn10 + (-locals.var_q_wdsoi_max_dn10)), (locals.var_q_bl_dep_dn11 + (-locals.var_q_wdsoi_max_dn11)), (locals.var_q_bl_dep_dn12 + (-locals.var_q_wdsoi_max_dn12)), (locals.var_q_bl_dep_dn17 + (-locals.var_q_wdsoi_max_dn17)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12880_body42_e16901;
            locals.var_tmf1_dn0 = assign12880_body42_e16901_d_n0;
            locals.var_tmf1_dn2 = assign12880_body42_e16901_d_n2;
            locals.var_tmf1_dn6 = assign12880_body42_e16901_d_n6;
            locals.var_tmf1_dn7 = assign12880_body42_e16901_d_n7;
            locals.var_tmf1_dn10 = assign12880_body42_e16901_d_n10;
            locals.var_tmf1_dn11 = assign12880_body42_e16901_d_n11;
            locals.var_tmf1_dn12 = assign12880_body42_e16901_d_n12;
            locals.var_tmf1_dn17 = assign12880_body42_e16901_d_n17;
            let (assign12880_body43_e16912, assign12880_body43_e16912_d_n0, assign12880_body43_e16912_d_n2, assign12880_body43_e16912_d_n6, assign12880_body43_e16912_d_n7, assign12880_body43_e16912_d_n10, assign12880_body43_e16912_d_n11, assign12880_body43_e16912_d_n12, assign12880_body43_e16912_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body43_e16910: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign12880_body43_e16910, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
            locals.var_x2 = assign12880_body43_e16912;
            locals.var_x2_dn0 = assign12880_body43_e16912_d_n0;
            locals.var_x2_dn2 = assign12880_body43_e16912_d_n2;
            locals.var_x2_dn6 = assign12880_body43_e16912_d_n6;
            locals.var_x2_dn7 = assign12880_body43_e16912_d_n7;
            locals.var_x2_dn10 = assign12880_body43_e16912_d_n10;
            locals.var_x2_dn11 = assign12880_body43_e16912_d_n11;
            locals.var_x2_dn12 = assign12880_body43_e16912_d_n12;
            locals.var_x2_dn17 = assign12880_body43_e16912_d_n17;
            let (assign12880_body44_e16929, assign12880_body44_e16929_d_n0, assign12880_body44_e16929_d_n2, assign12880_body44_e16929_d_n6, assign12880_body44_e16929_d_n7, assign12880_body44_e16929_d_n10, assign12880_body44_e16929_d_n11, assign12880_body44_e16929_d_n12, assign12880_body44_e16929_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body44_e16920: f64 = (-locals.var_q_wdsoi_max);
        let assign12880_body44_e16922: f64 = assign12880_body44_e16920;
        let assign12880_body44_e16924: f64 = (-locals.var_q_wdsoi_max);
        let assign12880_body44_e16926: f64 = assign12880_body44_e16924;
        let assign12880_body44_e16927: f64 = (assign12880_body44_e16922 * assign12880_body44_e16926);
        (assign12880_body44_e16927, (((-locals.var_q_wdsoi_max_dn0) * assign12880_body44_e16926) + (assign12880_body44_e16922 * (-locals.var_q_wdsoi_max_dn0))), (((-locals.var_q_wdsoi_max_dn2) * assign12880_body44_e16926) + (assign12880_body44_e16922 * (-locals.var_q_wdsoi_max_dn2))), (((-locals.var_q_wdsoi_max_dn6) * assign12880_body44_e16926) + (assign12880_body44_e16922 * (-locals.var_q_wdsoi_max_dn6))), (((-locals.var_q_wdsoi_max_dn7) * assign12880_body44_e16926) + (assign12880_body44_e16922 * (-locals.var_q_wdsoi_max_dn7))), (((-locals.var_q_wdsoi_max_dn10) * assign12880_body44_e16926) + (assign12880_body44_e16922 * (-locals.var_q_wdsoi_max_dn10))), (((-locals.var_q_wdsoi_max_dn11) * assign12880_body44_e16926) + (assign12880_body44_e16922 * (-locals.var_q_wdsoi_max_dn11))), (((-locals.var_q_wdsoi_max_dn12) * assign12880_body44_e16926) + (assign12880_body44_e16922 * (-locals.var_q_wdsoi_max_dn12))), (((-locals.var_q_wdsoi_max_dn17) * assign12880_body44_e16926) + (assign12880_body44_e16922 * (-locals.var_q_wdsoi_max_dn17))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
            locals.var_xmax2 = assign12880_body44_e16929;
            locals.var_xmax2_dn0 = assign12880_body44_e16929_d_n0;
            locals.var_xmax2_dn2 = assign12880_body44_e16929_d_n2;
            locals.var_xmax2_dn6 = assign12880_body44_e16929_d_n6;
            locals.var_xmax2_dn7 = assign12880_body44_e16929_d_n7;
            locals.var_xmax2_dn10 = assign12880_body44_e16929_d_n10;
            locals.var_xmax2_dn11 = assign12880_body44_e16929_d_n11;
            locals.var_xmax2_dn12 = assign12880_body44_e16929_d_n12;
            locals.var_xmax2_dn17 = assign12880_body44_e16929_d_n17;
            let (assign12880_body45_e16938, assign12880_body45_e16938_d_n0, assign12880_body45_e16938_d_n2, assign12880_body45_e16938_d_n6, assign12880_body45_e16938_d_n7, assign12880_body45_e16938_d_n10, assign12880_body45_e16938_d_n11, assign12880_body45_e16938_d_n12, assign12880_body45_e16938_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12880_body45_e16938;
            locals.var_xp_dn0 = assign12880_body45_e16938_d_n0;
            locals.var_xp_dn2 = assign12880_body45_e16938_d_n2;
            locals.var_xp_dn6 = assign12880_body45_e16938_d_n6;
            locals.var_xp_dn7 = assign12880_body45_e16938_d_n7;
            locals.var_xp_dn10 = assign12880_body45_e16938_d_n10;
            locals.var_xp_dn11 = assign12880_body45_e16938_d_n11;
            locals.var_xp_dn12 = assign12880_body45_e16938_d_n12;
            locals.var_xp_dn17 = assign12880_body45_e16938_d_n17;
            let (assign12880_body46_e16947, assign12880_body46_e16947_d_n0, assign12880_body46_e16947_d_n2, assign12880_body46_e16947_d_n6, assign12880_body46_e16947_d_n7, assign12880_body46_e16947_d_n10, assign12880_body46_e16947_d_n11, assign12880_body46_e16947_d_n12, assign12880_body46_e16947_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12880_body46_e16947;
            locals.var_xmp_dn0 = assign12880_body46_e16947_d_n0;
            locals.var_xmp_dn2 = assign12880_body46_e16947_d_n2;
            locals.var_xmp_dn6 = assign12880_body46_e16947_d_n6;
            locals.var_xmp_dn7 = assign12880_body46_e16947_d_n7;
            locals.var_xmp_dn10 = assign12880_body46_e16947_d_n10;
            locals.var_xmp_dn11 = assign12880_body46_e16947_d_n11;
            locals.var_xmp_dn12 = assign12880_body46_e16947_d_n12;
            locals.var_xmp_dn17 = assign12880_body46_e16947_d_n17;
            let (assign12880_body47_e16956,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12880_body47_e16956;
            let (assign12880_body48_e16965,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body48_e16965;
            let (assign12880_body49_e16974, assign12880_body49_e16974_d_n0, assign12880_body49_e16974_d_n2, assign12880_body49_e16974_d_n6, assign12880_body49_e16974_d_n7, assign12880_body49_e16974_d_n10, assign12880_body49_e16974_d_n11, assign12880_body49_e16974_d_n12, assign12880_body49_e16974_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12880_body49_e16974;
            locals.var_arg_dn0 = assign12880_body49_e16974_d_n0;
            locals.var_arg_dn2 = assign12880_body49_e16974_d_n2;
            locals.var_arg_dn6 = assign12880_body49_e16974_d_n6;
            locals.var_arg_dn7 = assign12880_body49_e16974_d_n7;
            locals.var_arg_dn10 = assign12880_body49_e16974_d_n10;
            locals.var_arg_dn11 = assign12880_body49_e16974_d_n11;
            locals.var_arg_dn12 = assign12880_body49_e16974_d_n12;
            locals.var_arg_dn17 = assign12880_body49_e16974_d_n17;
            let (assign12880_body50_e16983, assign12880_body50_e16983_d_n0, assign12880_body50_e16983_d_n2, assign12880_body50_e16983_d_n6, assign12880_body50_e16983_d_n7, assign12880_body50_e16983_d_n10, assign12880_body50_e16983_d_n11, assign12880_body50_e16983_d_n12, assign12880_body50_e16983_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12880_body50_e16983;
            locals.var_dnm_dn0 = assign12880_body50_e16983_d_n0;
            locals.var_dnm_dn2 = assign12880_body50_e16983_d_n2;
            locals.var_dnm_dn6 = assign12880_body50_e16983_d_n6;
            locals.var_dnm_dn7 = assign12880_body50_e16983_d_n7;
            locals.var_dnm_dn10 = assign12880_body50_e16983_d_n10;
            locals.var_dnm_dn11 = assign12880_body50_e16983_d_n11;
            locals.var_dnm_dn12 = assign12880_body50_e16983_d_n12;
            locals.var_dnm_dn17 = assign12880_body50_e16983_d_n17;
            let (assign12880_body51_e16994, assign12880_body51_e16994_d_n0, assign12880_body51_e16994_d_n2, assign12880_body51_e16994_d_n6, assign12880_body51_e16994_d_n7, assign12880_body51_e16994_d_n10, assign12880_body51_e16994_d_n11, assign12880_body51_e16994_d_n12, assign12880_body51_e16994_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body51_e16992: f64 = (locals.var_xp * locals.var_x2);
        (assign12880_body51_e16992, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12880_body51_e16994;
            locals.var_xp_dn0 = assign12880_body51_e16994_d_n0;
            locals.var_xp_dn2 = assign12880_body51_e16994_d_n2;
            locals.var_xp_dn6 = assign12880_body51_e16994_d_n6;
            locals.var_xp_dn7 = assign12880_body51_e16994_d_n7;
            locals.var_xp_dn10 = assign12880_body51_e16994_d_n10;
            locals.var_xp_dn11 = assign12880_body51_e16994_d_n11;
            locals.var_xp_dn12 = assign12880_body51_e16994_d_n12;
            locals.var_xp_dn17 = assign12880_body51_e16994_d_n17;
            let (assign12880_body52_e17005, assign12880_body52_e17005_d_n0, assign12880_body52_e17005_d_n2, assign12880_body52_e17005_d_n6, assign12880_body52_e17005_d_n7, assign12880_body52_e17005_d_n10, assign12880_body52_e17005_d_n11, assign12880_body52_e17005_d_n12, assign12880_body52_e17005_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body52_e17003: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12880_body52_e17003, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12880_body52_e17005;
            locals.var_xmp_dn0 = assign12880_body52_e17005_d_n0;
            locals.var_xmp_dn2 = assign12880_body52_e17005_d_n2;
            locals.var_xmp_dn6 = assign12880_body52_e17005_d_n6;
            locals.var_xmp_dn7 = assign12880_body52_e17005_d_n7;
            locals.var_xmp_dn10 = assign12880_body52_e17005_d_n10;
            locals.var_xmp_dn11 = assign12880_body52_e17005_d_n11;
            locals.var_xmp_dn12 = assign12880_body52_e17005_d_n12;
            locals.var_xmp_dn17 = assign12880_body52_e17005_d_n17;
            let (assign12880_body53_e17016, assign12880_body53_e17016_d_n0, assign12880_body53_e17016_d_n2, assign12880_body53_e17016_d_n6, assign12880_body53_e17016_d_n7, assign12880_body53_e17016_d_n10, assign12880_body53_e17016_d_n11, assign12880_body53_e17016_d_n12, assign12880_body53_e17016_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body53_e17014: f64 = (locals.var_xp * locals.var_x2);
        (assign12880_body53_e17014, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12880_body53_e17016;
            locals.var_xp_dn0 = assign12880_body53_e17016_d_n0;
            locals.var_xp_dn2 = assign12880_body53_e17016_d_n2;
            locals.var_xp_dn6 = assign12880_body53_e17016_d_n6;
            locals.var_xp_dn7 = assign12880_body53_e17016_d_n7;
            locals.var_xp_dn10 = assign12880_body53_e17016_d_n10;
            locals.var_xp_dn11 = assign12880_body53_e17016_d_n11;
            locals.var_xp_dn12 = assign12880_body53_e17016_d_n12;
            locals.var_xp_dn17 = assign12880_body53_e17016_d_n17;
            let (assign12880_body54_e17027, assign12880_body54_e17027_d_n0, assign12880_body54_e17027_d_n2, assign12880_body54_e17027_d_n6, assign12880_body54_e17027_d_n7, assign12880_body54_e17027_d_n10, assign12880_body54_e17027_d_n11, assign12880_body54_e17027_d_n12, assign12880_body54_e17027_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body54_e17025: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12880_body54_e17025, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12880_body54_e17027;
            locals.var_xmp_dn0 = assign12880_body54_e17027_d_n0;
            locals.var_xmp_dn2 = assign12880_body54_e17027_d_n2;
            locals.var_xmp_dn6 = assign12880_body54_e17027_d_n6;
            locals.var_xmp_dn7 = assign12880_body54_e17027_d_n7;
            locals.var_xmp_dn10 = assign12880_body54_e17027_d_n10;
            locals.var_xmp_dn11 = assign12880_body54_e17027_d_n11;
            locals.var_xmp_dn12 = assign12880_body54_e17027_d_n12;
            locals.var_xmp_dn17 = assign12880_body54_e17027_d_n17;
            let (assign12880_body55_e17038, assign12880_body55_e17038_d_n0, assign12880_body55_e17038_d_n2, assign12880_body55_e17038_d_n6, assign12880_body55_e17038_d_n7, assign12880_body55_e17038_d_n10, assign12880_body55_e17038_d_n11, assign12880_body55_e17038_d_n12, assign12880_body55_e17038_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body55_e17036: f64 = (locals.var_xp + locals.var_xmp);
        (assign12880_body55_e17036, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12880_body55_e17038;
            locals.var_arg_dn0 = assign12880_body55_e17038_d_n0;
            locals.var_arg_dn2 = assign12880_body55_e17038_d_n2;
            locals.var_arg_dn6 = assign12880_body55_e17038_d_n6;
            locals.var_arg_dn7 = assign12880_body55_e17038_d_n7;
            locals.var_arg_dn10 = assign12880_body55_e17038_d_n10;
            locals.var_arg_dn11 = assign12880_body55_e17038_d_n11;
            locals.var_arg_dn12 = assign12880_body55_e17038_d_n12;
            locals.var_arg_dn17 = assign12880_body55_e17038_d_n17;
            let (assign12880_body56_e17047, assign12880_body56_e17047_d_n0, assign12880_body56_e17047_d_n2, assign12880_body56_e17047_d_n6, assign12880_body56_e17047_d_n7, assign12880_body56_e17047_d_n10, assign12880_body56_e17047_d_n11, assign12880_body56_e17047_d_n12, assign12880_body56_e17047_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12880_body56_e17047;
            locals.var_dnm_dn0 = assign12880_body56_e17047_d_n0;
            locals.var_dnm_dn2 = assign12880_body56_e17047_d_n2;
            locals.var_dnm_dn6 = assign12880_body56_e17047_d_n6;
            locals.var_dnm_dn7 = assign12880_body56_e17047_d_n7;
            locals.var_dnm_dn10 = assign12880_body56_e17047_d_n10;
            locals.var_dnm_dn11 = assign12880_body56_e17047_d_n11;
            locals.var_dnm_dn12 = assign12880_body56_e17047_d_n12;
            locals.var_dnm_dn17 = assign12880_body56_e17047_d_n17;
            let assign12880_body57_e17062: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard391 = assign12880_body57_e17062;
            let assign12880_body58_e17065: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard392 = assign12880_body58_e17065;
            let (assign12880_body59_e17078,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 != 0.0)) && (locals.var_guard392 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body59_e17078;
            let assign12880_body60_e17081: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard393 = assign12880_body60_e17081;
            let (assign12880_body61_e17097,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 != 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body61_e17097;
            let assign12880_body62_e17100: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard394 = assign12880_body62_e17100;
            let (assign12880_body63_e17119,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 != 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body63_e17119;
            let assign12880_body64_e17122: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard395 = assign12880_body64_e17122;
            let (assign12880_body65_e17144,) = {
    if ((((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 != 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 == 0.0)) && (locals.var_guard394 == 0.0)) && (locals.var_guard395 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body65_e17144;
            let (assign12880_body66_e17155,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12880_body66_e17155;
            let mut assign12880_body67_loop_guard: usize = 0;
            while {
                let assign12880_body67_cond_e17167: f64 = if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign12880_body67_cond_e17167 != 0.0
            } {
                assign12880_body67_loop_guard += 1;
                assert!(assign12880_body67_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12880_body67_body0_e17179, assign12880_body67_body0_e17179_d_n0, assign12880_body67_body0_e17179_d_n2, assign12880_body67_body0_e17179_d_n6, assign12880_body67_body0_e17179_d_n7, assign12880_body67_body0_e17179_d_n10, assign12880_body67_body0_e17179_d_n11, assign12880_body67_body0_e17179_d_n12, assign12880_body67_body0_e17179_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign12880_body67_body0_e17177: f64 = (locals.var_dnm).sqrt();
        (assign12880_body67_body0_e17177, (locals.var_dnm_dn0 / (2.0 * assign12880_body67_body0_e17177)), (locals.var_dnm_dn2 / (2.0 * assign12880_body67_body0_e17177)), (locals.var_dnm_dn6 / (2.0 * assign12880_body67_body0_e17177)), (locals.var_dnm_dn7 / (2.0 * assign12880_body67_body0_e17177)), (locals.var_dnm_dn10 / (2.0 * assign12880_body67_body0_e17177)), (locals.var_dnm_dn11 / (2.0 * assign12880_body67_body0_e17177)), (locals.var_dnm_dn12 / (2.0 * assign12880_body67_body0_e17177)), (locals.var_dnm_dn17 / (2.0 * assign12880_body67_body0_e17177)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
                locals.var_dnm = assign12880_body67_body0_e17179;
                locals.var_dnm_dn0 = assign12880_body67_body0_e17179_d_n0;
                locals.var_dnm_dn2 = assign12880_body67_body0_e17179_d_n2;
                locals.var_dnm_dn6 = assign12880_body67_body0_e17179_d_n6;
                locals.var_dnm_dn7 = assign12880_body67_body0_e17179_d_n7;
                locals.var_dnm_dn10 = assign12880_body67_body0_e17179_d_n10;
                locals.var_dnm_dn11 = assign12880_body67_body0_e17179_d_n11;
                locals.var_dnm_dn12 = assign12880_body67_body0_e17179_d_n12;
                locals.var_dnm_dn17 = assign12880_body67_body0_e17179_d_n17;
                let (assign12880_body67_body1_e17192,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign12880_body67_body1_e17190: f64 = (locals.var_m0 + 1.0);
        (assign12880_body67_body1_e17190,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign12880_body67_body1_e17192;
            }
            let (assign12880_body68_e17210, assign12880_body68_e17210_d_n0, assign12880_body68_e17210_d_n2, assign12880_body68_e17210_d_n6, assign12880_body68_e17210_d_n7, assign12880_body68_e17210_d_n10, assign12880_body68_e17210_d_n11, assign12880_body68_e17210_d_n12, assign12880_body68_e17210_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12880_body68_e17206: f64 = (2.0 * 2.0);
        let assign12880_body68_e17207: f64 = (1.0 / assign12880_body68_e17206);
        let assign12880_body68_e17208: f64 = (locals.var_dnm).powf(assign12880_body68_e17207);
        (assign12880_body68_e17208, if 0.0 == 0.0 && ((assign12880_body68_e17207) as f64).is_finite() && ((assign12880_body68_e17207) as f64).fract() == 0.0 { if assign12880_body68_e17207 == 0.0 { 0.0 } else { (assign12880_body68_e17207 * ((locals.var_dnm).powf(assign12880_body68_e17207 - 1.0) * locals.var_dnm_dn0)) } } else { (assign12880_body68_e17208 * (assign12880_body68_e17207 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body68_e17207) as f64).is_finite() && ((assign12880_body68_e17207) as f64).fract() == 0.0 { if assign12880_body68_e17207 == 0.0 { 0.0 } else { (assign12880_body68_e17207 * ((locals.var_dnm).powf(assign12880_body68_e17207 - 1.0) * locals.var_dnm_dn2)) } } else { (assign12880_body68_e17208 * (assign12880_body68_e17207 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body68_e17207) as f64).is_finite() && ((assign12880_body68_e17207) as f64).fract() == 0.0 { if assign12880_body68_e17207 == 0.0 { 0.0 } else { (assign12880_body68_e17207 * ((locals.var_dnm).powf(assign12880_body68_e17207 - 1.0) * locals.var_dnm_dn6)) } } else { (assign12880_body68_e17208 * (assign12880_body68_e17207 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body68_e17207) as f64).is_finite() && ((assign12880_body68_e17207) as f64).fract() == 0.0 { if assign12880_body68_e17207 == 0.0 { 0.0 } else { (assign12880_body68_e17207 * ((locals.var_dnm).powf(assign12880_body68_e17207 - 1.0) * locals.var_dnm_dn7)) } } else { (assign12880_body68_e17208 * (assign12880_body68_e17207 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body68_e17207) as f64).is_finite() && ((assign12880_body68_e17207) as f64).fract() == 0.0 { if assign12880_body68_e17207 == 0.0 { 0.0 } else { (assign12880_body68_e17207 * ((locals.var_dnm).powf(assign12880_body68_e17207 - 1.0) * locals.var_dnm_dn10)) } } else { (assign12880_body68_e17208 * (assign12880_body68_e17207 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body68_e17207) as f64).is_finite() && ((assign12880_body68_e17207) as f64).fract() == 0.0 { if assign12880_body68_e17207 == 0.0 { 0.0 } else { (assign12880_body68_e17207 * ((locals.var_dnm).powf(assign12880_body68_e17207 - 1.0) * locals.var_dnm_dn11)) } } else { (assign12880_body68_e17208 * (assign12880_body68_e17207 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body68_e17207) as f64).is_finite() && ((assign12880_body68_e17207) as f64).fract() == 0.0 { if assign12880_body68_e17207 == 0.0 { 0.0 } else { (assign12880_body68_e17207 * ((locals.var_dnm).powf(assign12880_body68_e17207 - 1.0) * locals.var_dnm_dn12)) } } else { (assign12880_body68_e17208 * (assign12880_body68_e17207 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body68_e17207) as f64).is_finite() && ((assign12880_body68_e17207) as f64).fract() == 0.0 { if assign12880_body68_e17207 == 0.0 { 0.0 } else { (assign12880_body68_e17207 * ((locals.var_dnm).powf(assign12880_body68_e17207 - 1.0) * locals.var_dnm_dn17)) } } else { (assign12880_body68_e17208 * (assign12880_body68_e17207 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12880_body68_e17210;
            locals.var_dnm_dn0 = assign12880_body68_e17210_d_n0;
            locals.var_dnm_dn2 = assign12880_body68_e17210_d_n2;
            locals.var_dnm_dn6 = assign12880_body68_e17210_d_n6;
            locals.var_dnm_dn7 = assign12880_body68_e17210_d_n7;
            locals.var_dnm_dn10 = assign12880_body68_e17210_d_n10;
            locals.var_dnm_dn11 = assign12880_body68_e17210_d_n11;
            locals.var_dnm_dn12 = assign12880_body68_e17210_d_n12;
            locals.var_dnm_dn17 = assign12880_body68_e17210_d_n17;
            let (assign12880_body69_e17221, assign12880_body69_e17221_d_n0, assign12880_body69_e17221_d_n2, assign12880_body69_e17221_d_n6, assign12880_body69_e17221_d_n7, assign12880_body69_e17221_d_n10, assign12880_body69_e17221_d_n11, assign12880_body69_e17221_d_n12, assign12880_body69_e17221_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body69_e17219: f64 = (1.0 / locals.var_dnm);
        (assign12880_body69_e17219, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12880_body69_e17221;
            locals.var_dnm_dn0 = assign12880_body69_e17221_d_n0;
            locals.var_dnm_dn2 = assign12880_body69_e17221_d_n2;
            locals.var_dnm_dn6 = assign12880_body69_e17221_d_n6;
            locals.var_dnm_dn7 = assign12880_body69_e17221_d_n7;
            locals.var_dnm_dn10 = assign12880_body69_e17221_d_n10;
            locals.var_dnm_dn11 = assign12880_body69_e17221_d_n11;
            locals.var_dnm_dn12 = assign12880_body69_e17221_d_n12;
            locals.var_dnm_dn17 = assign12880_body69_e17221_d_n17;
            let (assign12880_body70_e17237, assign12880_body70_e17237_d_n0, assign12880_body70_e17237_d_n2, assign12880_body70_e17237_d_n6, assign12880_body70_e17237_d_n7, assign12880_body70_e17237_d_n10, assign12880_body70_e17237_d_n11, assign12880_body70_e17237_d_n12, assign12880_body70_e17237_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body70_e17230: f64 = (-locals.var_q_wdsoi_max);
        let assign12880_body70_e17232: f64 = assign12880_body70_e17230;
        let assign12880_body70_e17233: f64 = (locals.var_tmf1 * assign12880_body70_e17232);
        let assign12880_body70_e17235: f64 = (assign12880_body70_e17233 * locals.var_dnm);
        (assign12880_body70_e17235, ((((locals.var_tmf1_dn0 * assign12880_body70_e17232) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn0))) * locals.var_dnm) + (assign12880_body70_e17233 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign12880_body70_e17232) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn2))) * locals.var_dnm) + (assign12880_body70_e17233 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign12880_body70_e17232) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn6))) * locals.var_dnm) + (assign12880_body70_e17233 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign12880_body70_e17232) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn7))) * locals.var_dnm) + (assign12880_body70_e17233 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign12880_body70_e17232) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn10))) * locals.var_dnm) + (assign12880_body70_e17233 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign12880_body70_e17232) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn11))) * locals.var_dnm) + (assign12880_body70_e17233 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign12880_body70_e17232) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn12))) * locals.var_dnm) + (assign12880_body70_e17233 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign12880_body70_e17232) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn17))) * locals.var_dnm) + (assign12880_body70_e17233 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0__blk381, locals.var_tmf0__blk381_dn0, locals.var_tmf0__blk381_dn2, locals.var_tmf0__blk381_dn6, locals.var_tmf0__blk381_dn7, locals.var_tmf0__blk381_dn10, locals.var_tmf0__blk381_dn11, locals.var_tmf0__blk381_dn12, locals.var_tmf0__blk381_dn17,)
    }
};
            locals.var_tmf0__blk381 = assign12880_body70_e17237;
            locals.var_tmf0__blk381_dn0 = assign12880_body70_e17237_d_n0;
            locals.var_tmf0__blk381_dn2 = assign12880_body70_e17237_d_n2;
            locals.var_tmf0__blk381_dn6 = assign12880_body70_e17237_d_n6;
            locals.var_tmf0__blk381_dn7 = assign12880_body70_e17237_d_n7;
            locals.var_tmf0__blk381_dn10 = assign12880_body70_e17237_d_n10;
            locals.var_tmf0__blk381_dn11 = assign12880_body70_e17237_d_n11;
            locals.var_tmf0__blk381_dn12 = assign12880_body70_e17237_d_n12;
            locals.var_tmf0__blk381_dn17 = assign12880_body70_e17237_d_n17;
            let (assign12880_body71_e17255, assign12880_body71_e17255_d_n0, assign12880_body71_e17255_d_n2, assign12880_body71_e17255_d_n6, assign12880_body71_e17255_d_n7, assign12880_body71_e17255_d_n10, assign12880_body71_e17255_d_n11, assign12880_body71_e17255_d_n12, assign12880_body71_e17255_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body71_e17245: f64 = (-locals.var_q_wdsoi_max);
        let assign12880_body71_e17247: f64 = assign12880_body71_e17245;
        let assign12880_body71_e17249: f64 = (assign12880_body71_e17247 * locals.var_xmp);
        let assign12880_body71_e17251: f64 = (assign12880_body71_e17249 * locals.var_dnm);
        let assign12880_body71_e17253: f64 = (assign12880_body71_e17251 / locals.var_arg);
        (assign12880_body71_e17253, ((((((((-locals.var_q_wdsoi_max_dn0) * locals.var_xmp) + (assign12880_body71_e17247 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign12880_body71_e17249 * locals.var_dnm_dn0)) * locals.var_arg) - (assign12880_body71_e17251 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn2) * locals.var_xmp) + (assign12880_body71_e17247 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign12880_body71_e17249 * locals.var_dnm_dn2)) * locals.var_arg) - (assign12880_body71_e17251 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn6) * locals.var_xmp) + (assign12880_body71_e17247 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign12880_body71_e17249 * locals.var_dnm_dn6)) * locals.var_arg) - (assign12880_body71_e17251 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn7) * locals.var_xmp) + (assign12880_body71_e17247 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign12880_body71_e17249 * locals.var_dnm_dn7)) * locals.var_arg) - (assign12880_body71_e17251 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn10) * locals.var_xmp) + (assign12880_body71_e17247 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign12880_body71_e17249 * locals.var_dnm_dn10)) * locals.var_arg) - (assign12880_body71_e17251 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn11) * locals.var_xmp) + (assign12880_body71_e17247 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign12880_body71_e17249 * locals.var_dnm_dn11)) * locals.var_arg) - (assign12880_body71_e17251 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn12) * locals.var_xmp) + (assign12880_body71_e17247 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign12880_body71_e17249 * locals.var_dnm_dn12)) * locals.var_arg) - (assign12880_body71_e17251 * locals.var_arg_dn12)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn17) * locals.var_xmp) + (assign12880_body71_e17247 * locals.var_xmp_dn17)) * locals.var_dnm) + (assign12880_body71_e17249 * locals.var_dnm_dn17)) * locals.var_arg) - (assign12880_body71_e17251 * locals.var_arg_dn17)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body71_e17255;
            locals.var_t0_dn0 = assign12880_body71_e17255_d_n0;
            locals.var_t0_dn2 = assign12880_body71_e17255_d_n2;
            locals.var_t0_dn6 = assign12880_body71_e17255_d_n6;
            locals.var_t0_dn7 = assign12880_body71_e17255_d_n7;
            locals.var_t0_dn10 = assign12880_body71_e17255_d_n10;
            locals.var_t0_dn11 = assign12880_body71_e17255_d_n11;
            locals.var_t0_dn12 = assign12880_body71_e17255_d_n12;
            locals.var_t0_dn17 = assign12880_body71_e17255_d_n17;
            let (assign12880_body72_e17271, assign12880_body72_e17271_d_n0, assign12880_body72_e17271_d_n2, assign12880_body72_e17271_d_n6, assign12880_body72_e17271_d_n7, assign12880_body72_e17271_d_n10, assign12880_body72_e17271_d_n11, assign12880_body72_e17271_d_n12, assign12880_body72_e17271_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12880_body72_e17264: f64 = (-locals.var_q_wdsoi_max);
        let assign12880_body72_e17266: f64 = assign12880_body72_e17264;
        let assign12880_body72_e17267: f64 = (-assign12880_body72_e17266);
        let assign12880_body72_e17269: f64 = (assign12880_body72_e17267 + locals.var_tmf0__blk381);
        (assign12880_body72_e17269, ((-(-locals.var_q_wdsoi_max_dn0)) + locals.var_tmf0__blk381_dn0), ((-(-locals.var_q_wdsoi_max_dn2)) + locals.var_tmf0__blk381_dn2), ((-(-locals.var_q_wdsoi_max_dn6)) + locals.var_tmf0__blk381_dn6), ((-(-locals.var_q_wdsoi_max_dn7)) + locals.var_tmf0__blk381_dn7), ((-(-locals.var_q_wdsoi_max_dn10)) + locals.var_tmf0__blk381_dn10), ((-(-locals.var_q_wdsoi_max_dn11)) + locals.var_tmf0__blk381_dn11), ((-(-locals.var_q_wdsoi_max_dn12)) + locals.var_tmf0__blk381_dn12), ((-(-locals.var_q_wdsoi_max_dn17)) + locals.var_tmf0__blk381_dn17),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12880_body72_e17271;
            locals.var_q_bl_dep_dn0 = assign12880_body72_e17271_d_n0;
            locals.var_q_bl_dep_dn2 = assign12880_body72_e17271_d_n2;
            locals.var_q_bl_dep_dn6 = assign12880_body72_e17271_d_n6;
            locals.var_q_bl_dep_dn7 = assign12880_body72_e17271_d_n7;
            locals.var_q_bl_dep_dn10 = assign12880_body72_e17271_d_n10;
            locals.var_q_bl_dep_dn11 = assign12880_body72_e17271_d_n11;
            locals.var_q_bl_dep_dn12 = assign12880_body72_e17271_d_n12;
            locals.var_q_bl_dep_dn17 = assign12880_body72_e17271_d_n17;
            let (assign12880_body73_e17280, assign12880_body73_e17280_d_n0, assign12880_body73_e17280_d_n2, assign12880_body73_e17280_d_n6, assign12880_body73_e17280_d_n7, assign12880_body73_e17280_d_n10, assign12880_body73_e17280_d_n11, assign12880_body73_e17280_d_n12, assign12880_body73_e17280_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body73_e17280;
            locals.var_t0_dn0 = assign12880_body73_e17280_d_n0;
            locals.var_t0_dn2 = assign12880_body73_e17280_d_n2;
            locals.var_t0_dn6 = assign12880_body73_e17280_d_n6;
            locals.var_t0_dn7 = assign12880_body73_e17280_d_n7;
            locals.var_t0_dn10 = assign12880_body73_e17280_d_n10;
            locals.var_t0_dn11 = assign12880_body73_e17280_d_n11;
            locals.var_t0_dn12 = assign12880_body73_e17280_d_n12;
            locals.var_t0_dn17 = assign12880_body73_e17280_d_n17;
            let (assign12880_body74_e17290, assign12880_body74_e17290_d_n0, assign12880_body74_e17290_d_n2, assign12880_body74_e17290_d_n6, assign12880_body74_e17290_d_n7, assign12880_body74_e17290_d_n10, assign12880_body74_e17290_d_n11, assign12880_body74_e17290_d_n12, assign12880_body74_e17290_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 == 0.0)) {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12880_body74_e17290;
            locals.var_q_bl_dep_dn0 = assign12880_body74_e17290_d_n0;
            locals.var_q_bl_dep_dn2 = assign12880_body74_e17290_d_n2;
            locals.var_q_bl_dep_dn6 = assign12880_body74_e17290_d_n6;
            locals.var_q_bl_dep_dn7 = assign12880_body74_e17290_d_n7;
            locals.var_q_bl_dep_dn10 = assign12880_body74_e17290_d_n10;
            locals.var_q_bl_dep_dn11 = assign12880_body74_e17290_d_n11;
            locals.var_q_bl_dep_dn12 = assign12880_body74_e17290_d_n12;
            locals.var_q_bl_dep_dn17 = assign12880_body74_e17290_d_n17;
            let (assign12880_body75_e17300, assign12880_body75_e17300_d_n0, assign12880_body75_e17300_d_n2, assign12880_body75_e17300_d_n6, assign12880_body75_e17300_d_n7, assign12880_body75_e17300_d_n10, assign12880_body75_e17300_d_n11, assign12880_body75_e17300_d_n12, assign12880_body75_e17300_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard390 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body75_e17300;
            locals.var_t0_dn0 = assign12880_body75_e17300_d_n0;
            locals.var_t0_dn2 = assign12880_body75_e17300_d_n2;
            locals.var_t0_dn6 = assign12880_body75_e17300_d_n6;
            locals.var_t0_dn7 = assign12880_body75_e17300_d_n7;
            locals.var_t0_dn10 = assign12880_body75_e17300_d_n10;
            locals.var_t0_dn11 = assign12880_body75_e17300_d_n11;
            locals.var_t0_dn12 = assign12880_body75_e17300_d_n12;
            locals.var_t0_dn17 = assign12880_body75_e17300_d_n17;
            let (assign12880_body76_e17309, assign12880_body76_e17309_d_n0, assign12880_body76_e17309_d_n2, assign12880_body76_e17309_d_n6, assign12880_body76_e17309_d_n7, assign12880_body76_e17309_d_n10, assign12880_body76_e17309_d_n11, assign12880_body76_e17309_d_n12, assign12880_body76_e17309_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body76_e17307: f64 = (locals.var_q_bl_dep_dpbs * locals.var_t0);
        (assign12880_body76_e17307, ((locals.var_q_bl_dep_dpbs_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpbs_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpbs_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpbs_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpbs_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpbs_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpbs_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpbs_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12880_body76_e17309;
            locals.var_q_bl_dep_dpbs_dn0 = assign12880_body76_e17309_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12880_body76_e17309_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12880_body76_e17309_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12880_body76_e17309_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12880_body76_e17309_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12880_body76_e17309_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12880_body76_e17309_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12880_body76_e17309_d_n17;
            let (assign12880_body77_e17318, assign12880_body77_e17318_d_n0, assign12880_body77_e17318_d_n2, assign12880_body77_e17318_d_n6, assign12880_body77_e17318_d_n7, assign12880_body77_e17318_d_n10, assign12880_body77_e17318_d_n11, assign12880_body77_e17318_d_n12, assign12880_body77_e17318_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body77_e17316: f64 = (locals.var_q_bl_dep_dpss * locals.var_t0);
        (assign12880_body77_e17316, ((locals.var_q_bl_dep_dpss_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpss_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpss_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpss_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpss_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpss_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpss_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpss_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12880_body77_e17318;
            locals.var_q_bl_dep_dpss_dn0 = assign12880_body77_e17318_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12880_body77_e17318_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12880_body77_e17318_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12880_body77_e17318_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12880_body77_e17318_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12880_body77_e17318_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12880_body77_e17318_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12880_body77_e17318_d_n17;
            let assign12880_body78_e17322: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12880_body78_e17325: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12880_body78_e17326: f64 = (-assign12880_body78_e17325);
            let assign12880_body78_e17328: f64 = assign12880_body78_e17326;
            let assign12880_body78_e17329: f64 = (assign12880_body78_e17322 + assign12880_body78_e17328);
            let assign12880_body78_e17333: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12880_body78_e17334: f64 = (-assign12880_body78_e17333);
            let assign12880_body78_e17336: f64 = assign12880_body78_e17334;
            let assign12880_body78_e17339: f64 = if ((locals.var_q_bl_dep < assign12880_body78_e17329) && (assign12880_body78_e17336 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard396 = assign12880_body78_e17339;
            let (assign12880_body79_e17359, assign12880_body79_e17359_d_n0, assign12880_body79_e17359_d_n2, assign12880_body79_e17359_d_n6, assign12880_body79_e17359_d_n7, assign12880_body79_e17359_d_n10, assign12880_body79_e17359_d_n11, assign12880_body79_e17359_d_n12, assign12880_body79_e17359_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body79_e17348: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12880_body79_e17351: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12880_body79_e17352: f64 = (-assign12880_body79_e17351);
        let assign12880_body79_e17354: f64 = assign12880_body79_e17352;
        let assign12880_body79_e17355: f64 = (assign12880_body79_e17348 + assign12880_body79_e17354);
        let assign12880_body79_e17357: f64 = (assign12880_body79_e17355 - locals.var_q_bl_dep);
        (assign12880_body79_e17357, (((locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0) + (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0))) - locals.var_q_bl_dep_dn0), (((locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2) + (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2))) - locals.var_q_bl_dep_dn2), (((locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6) + (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6))) - locals.var_q_bl_dep_dn6), (((locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7) + (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7))) - locals.var_q_bl_dep_dn7), (((locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10) + (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10))) - locals.var_q_bl_dep_dn10), (((locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11) + (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11))) - locals.var_q_bl_dep_dn11), (((locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12) + (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12))) - locals.var_q_bl_dep_dn12), (((locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17) + (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17))) - locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12880_body79_e17359;
            locals.var_tmf1_dn0 = assign12880_body79_e17359_d_n0;
            locals.var_tmf1_dn2 = assign12880_body79_e17359_d_n2;
            locals.var_tmf1_dn6 = assign12880_body79_e17359_d_n6;
            locals.var_tmf1_dn7 = assign12880_body79_e17359_d_n7;
            locals.var_tmf1_dn10 = assign12880_body79_e17359_d_n10;
            locals.var_tmf1_dn11 = assign12880_body79_e17359_d_n11;
            locals.var_tmf1_dn12 = assign12880_body79_e17359_d_n12;
            locals.var_tmf1_dn17 = assign12880_body79_e17359_d_n17;
            let (assign12880_body80_e17370, assign12880_body80_e17370_d_n0, assign12880_body80_e17370_d_n2, assign12880_body80_e17370_d_n6, assign12880_body80_e17370_d_n7, assign12880_body80_e17370_d_n10, assign12880_body80_e17370_d_n11, assign12880_body80_e17370_d_n12, assign12880_body80_e17370_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body80_e17368: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign12880_body80_e17368, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
            locals.var_x2 = assign12880_body80_e17370;
            locals.var_x2_dn0 = assign12880_body80_e17370_d_n0;
            locals.var_x2_dn2 = assign12880_body80_e17370_d_n2;
            locals.var_x2_dn6 = assign12880_body80_e17370_d_n6;
            locals.var_x2_dn7 = assign12880_body80_e17370_d_n7;
            locals.var_x2_dn10 = assign12880_body80_e17370_d_n10;
            locals.var_x2_dn11 = assign12880_body80_e17370_d_n11;
            locals.var_x2_dn12 = assign12880_body80_e17370_d_n12;
            locals.var_x2_dn17 = assign12880_body80_e17370_d_n17;
            let (assign12880_body81_e17391, assign12880_body81_e17391_d_n0, assign12880_body81_e17391_d_n2, assign12880_body81_e17391_d_n6, assign12880_body81_e17391_d_n7, assign12880_body81_e17391_d_n10, assign12880_body81_e17391_d_n11, assign12880_body81_e17391_d_n12, assign12880_body81_e17391_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body81_e17379: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12880_body81_e17380: f64 = (-assign12880_body81_e17379);
        let assign12880_body81_e17382: f64 = assign12880_body81_e17380;
        let assign12880_body81_e17385: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12880_body81_e17386: f64 = (-assign12880_body81_e17385);
        let assign12880_body81_e17388: f64 = assign12880_body81_e17386;
        let assign12880_body81_e17389: f64 = (assign12880_body81_e17382 * assign12880_body81_e17388);
        (assign12880_body81_e17389, (((-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)) * assign12880_body81_e17388) + (assign12880_body81_e17382 * (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)))), (((-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)) * assign12880_body81_e17388) + (assign12880_body81_e17382 * (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)))), (((-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)) * assign12880_body81_e17388) + (assign12880_body81_e17382 * (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)))), (((-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)) * assign12880_body81_e17388) + (assign12880_body81_e17382 * (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)))), (((-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)) * assign12880_body81_e17388) + (assign12880_body81_e17382 * (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)))), (((-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)) * assign12880_body81_e17388) + (assign12880_body81_e17382 * (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)))), (((-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)) * assign12880_body81_e17388) + (assign12880_body81_e17382 * (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)))), (((-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)) * assign12880_body81_e17388) + (assign12880_body81_e17382 * (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
            locals.var_xmax2 = assign12880_body81_e17391;
            locals.var_xmax2_dn0 = assign12880_body81_e17391_d_n0;
            locals.var_xmax2_dn2 = assign12880_body81_e17391_d_n2;
            locals.var_xmax2_dn6 = assign12880_body81_e17391_d_n6;
            locals.var_xmax2_dn7 = assign12880_body81_e17391_d_n7;
            locals.var_xmax2_dn10 = assign12880_body81_e17391_d_n10;
            locals.var_xmax2_dn11 = assign12880_body81_e17391_d_n11;
            locals.var_xmax2_dn12 = assign12880_body81_e17391_d_n12;
            locals.var_xmax2_dn17 = assign12880_body81_e17391_d_n17;
            let (assign12880_body82_e17400, assign12880_body82_e17400_d_n0, assign12880_body82_e17400_d_n2, assign12880_body82_e17400_d_n6, assign12880_body82_e17400_d_n7, assign12880_body82_e17400_d_n10, assign12880_body82_e17400_d_n11, assign12880_body82_e17400_d_n12, assign12880_body82_e17400_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12880_body82_e17400;
            locals.var_xp_dn0 = assign12880_body82_e17400_d_n0;
            locals.var_xp_dn2 = assign12880_body82_e17400_d_n2;
            locals.var_xp_dn6 = assign12880_body82_e17400_d_n6;
            locals.var_xp_dn7 = assign12880_body82_e17400_d_n7;
            locals.var_xp_dn10 = assign12880_body82_e17400_d_n10;
            locals.var_xp_dn11 = assign12880_body82_e17400_d_n11;
            locals.var_xp_dn12 = assign12880_body82_e17400_d_n12;
            locals.var_xp_dn17 = assign12880_body82_e17400_d_n17;
            let (assign12880_body83_e17409, assign12880_body83_e17409_d_n0, assign12880_body83_e17409_d_n2, assign12880_body83_e17409_d_n6, assign12880_body83_e17409_d_n7, assign12880_body83_e17409_d_n10, assign12880_body83_e17409_d_n11, assign12880_body83_e17409_d_n12, assign12880_body83_e17409_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12880_body83_e17409;
            locals.var_xmp_dn0 = assign12880_body83_e17409_d_n0;
            locals.var_xmp_dn2 = assign12880_body83_e17409_d_n2;
            locals.var_xmp_dn6 = assign12880_body83_e17409_d_n6;
            locals.var_xmp_dn7 = assign12880_body83_e17409_d_n7;
            locals.var_xmp_dn10 = assign12880_body83_e17409_d_n10;
            locals.var_xmp_dn11 = assign12880_body83_e17409_d_n11;
            locals.var_xmp_dn12 = assign12880_body83_e17409_d_n12;
            locals.var_xmp_dn17 = assign12880_body83_e17409_d_n17;
            let (assign12880_body84_e17418,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12880_body84_e17418;
            let (assign12880_body85_e17427,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body85_e17427;
            let (assign12880_body86_e17436, assign12880_body86_e17436_d_n0, assign12880_body86_e17436_d_n2, assign12880_body86_e17436_d_n6, assign12880_body86_e17436_d_n7, assign12880_body86_e17436_d_n10, assign12880_body86_e17436_d_n11, assign12880_body86_e17436_d_n12, assign12880_body86_e17436_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12880_body86_e17436;
            locals.var_arg_dn0 = assign12880_body86_e17436_d_n0;
            locals.var_arg_dn2 = assign12880_body86_e17436_d_n2;
            locals.var_arg_dn6 = assign12880_body86_e17436_d_n6;
            locals.var_arg_dn7 = assign12880_body86_e17436_d_n7;
            locals.var_arg_dn10 = assign12880_body86_e17436_d_n10;
            locals.var_arg_dn11 = assign12880_body86_e17436_d_n11;
            locals.var_arg_dn12 = assign12880_body86_e17436_d_n12;
            locals.var_arg_dn17 = assign12880_body86_e17436_d_n17;
            let (assign12880_body87_e17445, assign12880_body87_e17445_d_n0, assign12880_body87_e17445_d_n2, assign12880_body87_e17445_d_n6, assign12880_body87_e17445_d_n7, assign12880_body87_e17445_d_n10, assign12880_body87_e17445_d_n11, assign12880_body87_e17445_d_n12, assign12880_body87_e17445_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12880_body87_e17445;
            locals.var_dnm_dn0 = assign12880_body87_e17445_d_n0;
            locals.var_dnm_dn2 = assign12880_body87_e17445_d_n2;
            locals.var_dnm_dn6 = assign12880_body87_e17445_d_n6;
            locals.var_dnm_dn7 = assign12880_body87_e17445_d_n7;
            locals.var_dnm_dn10 = assign12880_body87_e17445_d_n10;
            locals.var_dnm_dn11 = assign12880_body87_e17445_d_n11;
            locals.var_dnm_dn12 = assign12880_body87_e17445_d_n12;
            locals.var_dnm_dn17 = assign12880_body87_e17445_d_n17;
            let (assign12880_body88_e17456, assign12880_body88_e17456_d_n0, assign12880_body88_e17456_d_n2, assign12880_body88_e17456_d_n6, assign12880_body88_e17456_d_n7, assign12880_body88_e17456_d_n10, assign12880_body88_e17456_d_n11, assign12880_body88_e17456_d_n12, assign12880_body88_e17456_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body88_e17454: f64 = (locals.var_xp * locals.var_x2);
        (assign12880_body88_e17454, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12880_body88_e17456;
            locals.var_xp_dn0 = assign12880_body88_e17456_d_n0;
            locals.var_xp_dn2 = assign12880_body88_e17456_d_n2;
            locals.var_xp_dn6 = assign12880_body88_e17456_d_n6;
            locals.var_xp_dn7 = assign12880_body88_e17456_d_n7;
            locals.var_xp_dn10 = assign12880_body88_e17456_d_n10;
            locals.var_xp_dn11 = assign12880_body88_e17456_d_n11;
            locals.var_xp_dn12 = assign12880_body88_e17456_d_n12;
            locals.var_xp_dn17 = assign12880_body88_e17456_d_n17;
            let (assign12880_body89_e17467, assign12880_body89_e17467_d_n0, assign12880_body89_e17467_d_n2, assign12880_body89_e17467_d_n6, assign12880_body89_e17467_d_n7, assign12880_body89_e17467_d_n10, assign12880_body89_e17467_d_n11, assign12880_body89_e17467_d_n12, assign12880_body89_e17467_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body89_e17465: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12880_body89_e17465, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12880_body89_e17467;
            locals.var_xmp_dn0 = assign12880_body89_e17467_d_n0;
            locals.var_xmp_dn2 = assign12880_body89_e17467_d_n2;
            locals.var_xmp_dn6 = assign12880_body89_e17467_d_n6;
            locals.var_xmp_dn7 = assign12880_body89_e17467_d_n7;
            locals.var_xmp_dn10 = assign12880_body89_e17467_d_n10;
            locals.var_xmp_dn11 = assign12880_body89_e17467_d_n11;
            locals.var_xmp_dn12 = assign12880_body89_e17467_d_n12;
            locals.var_xmp_dn17 = assign12880_body89_e17467_d_n17;
            let (assign12880_body90_e17478, assign12880_body90_e17478_d_n0, assign12880_body90_e17478_d_n2, assign12880_body90_e17478_d_n6, assign12880_body90_e17478_d_n7, assign12880_body90_e17478_d_n10, assign12880_body90_e17478_d_n11, assign12880_body90_e17478_d_n12, assign12880_body90_e17478_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body90_e17476: f64 = (locals.var_xp * locals.var_x2);
        (assign12880_body90_e17476, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12880_body90_e17478;
            locals.var_xp_dn0 = assign12880_body90_e17478_d_n0;
            locals.var_xp_dn2 = assign12880_body90_e17478_d_n2;
            locals.var_xp_dn6 = assign12880_body90_e17478_d_n6;
            locals.var_xp_dn7 = assign12880_body90_e17478_d_n7;
            locals.var_xp_dn10 = assign12880_body90_e17478_d_n10;
            locals.var_xp_dn11 = assign12880_body90_e17478_d_n11;
            locals.var_xp_dn12 = assign12880_body90_e17478_d_n12;
            locals.var_xp_dn17 = assign12880_body90_e17478_d_n17;
            let (assign12880_body91_e17489, assign12880_body91_e17489_d_n0, assign12880_body91_e17489_d_n2, assign12880_body91_e17489_d_n6, assign12880_body91_e17489_d_n7, assign12880_body91_e17489_d_n10, assign12880_body91_e17489_d_n11, assign12880_body91_e17489_d_n12, assign12880_body91_e17489_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body91_e17487: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12880_body91_e17487, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12880_body91_e17489;
            locals.var_xmp_dn0 = assign12880_body91_e17489_d_n0;
            locals.var_xmp_dn2 = assign12880_body91_e17489_d_n2;
            locals.var_xmp_dn6 = assign12880_body91_e17489_d_n6;
            locals.var_xmp_dn7 = assign12880_body91_e17489_d_n7;
            locals.var_xmp_dn10 = assign12880_body91_e17489_d_n10;
            locals.var_xmp_dn11 = assign12880_body91_e17489_d_n11;
            locals.var_xmp_dn12 = assign12880_body91_e17489_d_n12;
            locals.var_xmp_dn17 = assign12880_body91_e17489_d_n17;
            let (assign12880_body92_e17500, assign12880_body92_e17500_d_n0, assign12880_body92_e17500_d_n2, assign12880_body92_e17500_d_n6, assign12880_body92_e17500_d_n7, assign12880_body92_e17500_d_n10, assign12880_body92_e17500_d_n11, assign12880_body92_e17500_d_n12, assign12880_body92_e17500_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body92_e17498: f64 = (locals.var_xp + locals.var_xmp);
        (assign12880_body92_e17498, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12880_body92_e17500;
            locals.var_arg_dn0 = assign12880_body92_e17500_d_n0;
            locals.var_arg_dn2 = assign12880_body92_e17500_d_n2;
            locals.var_arg_dn6 = assign12880_body92_e17500_d_n6;
            locals.var_arg_dn7 = assign12880_body92_e17500_d_n7;
            locals.var_arg_dn10 = assign12880_body92_e17500_d_n10;
            locals.var_arg_dn11 = assign12880_body92_e17500_d_n11;
            locals.var_arg_dn12 = assign12880_body92_e17500_d_n12;
            locals.var_arg_dn17 = assign12880_body92_e17500_d_n17;
            let (assign12880_body93_e17509, assign12880_body93_e17509_d_n0, assign12880_body93_e17509_d_n2, assign12880_body93_e17509_d_n6, assign12880_body93_e17509_d_n7, assign12880_body93_e17509_d_n10, assign12880_body93_e17509_d_n11, assign12880_body93_e17509_d_n12, assign12880_body93_e17509_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12880_body93_e17509;
            locals.var_dnm_dn0 = assign12880_body93_e17509_d_n0;
            locals.var_dnm_dn2 = assign12880_body93_e17509_d_n2;
            locals.var_dnm_dn6 = assign12880_body93_e17509_d_n6;
            locals.var_dnm_dn7 = assign12880_body93_e17509_d_n7;
            locals.var_dnm_dn10 = assign12880_body93_e17509_d_n10;
            locals.var_dnm_dn11 = assign12880_body93_e17509_d_n11;
            locals.var_dnm_dn12 = assign12880_body93_e17509_d_n12;
            locals.var_dnm_dn17 = assign12880_body93_e17509_d_n17;
            let assign12880_body94_e17524: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard397 = assign12880_body94_e17524;
            let assign12880_body95_e17527: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard398 = assign12880_body95_e17527;
            let (assign12880_body96_e17540,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard398 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body96_e17540;
            let assign12880_body97_e17543: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard399 = assign12880_body97_e17543;
            let (assign12880_body98_e17559,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard398 == 0.0)) && (locals.var_guard399 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body98_e17559;
            let assign12880_body99_e17562: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard400 = assign12880_body99_e17562;
            let (assign12880_body100_e17581,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard398 == 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body100_e17581;
            let assign12880_body101_e17584: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard401 = assign12880_body101_e17584;
            let (assign12880_body102_e17606,) = {
    if ((((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard398 == 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) && (locals.var_guard401 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12880_body102_e17606;
            let (assign12880_body103_e17617,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12880_body103_e17617;
            let mut assign12880_body104_loop_guard: usize = 0;
            while {
                let assign12880_body104_cond_e17629: f64 = if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign12880_body104_cond_e17629 != 0.0
            } {
                assign12880_body104_loop_guard += 1;
                assert!(assign12880_body104_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12880_body104_body0_e17641, assign12880_body104_body0_e17641_d_n0, assign12880_body104_body0_e17641_d_n2, assign12880_body104_body0_e17641_d_n6, assign12880_body104_body0_e17641_d_n7, assign12880_body104_body0_e17641_d_n10, assign12880_body104_body0_e17641_d_n11, assign12880_body104_body0_e17641_d_n12, assign12880_body104_body0_e17641_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign12880_body104_body0_e17639: f64 = (locals.var_dnm).sqrt();
        (assign12880_body104_body0_e17639, (locals.var_dnm_dn0 / (2.0 * assign12880_body104_body0_e17639)), (locals.var_dnm_dn2 / (2.0 * assign12880_body104_body0_e17639)), (locals.var_dnm_dn6 / (2.0 * assign12880_body104_body0_e17639)), (locals.var_dnm_dn7 / (2.0 * assign12880_body104_body0_e17639)), (locals.var_dnm_dn10 / (2.0 * assign12880_body104_body0_e17639)), (locals.var_dnm_dn11 / (2.0 * assign12880_body104_body0_e17639)), (locals.var_dnm_dn12 / (2.0 * assign12880_body104_body0_e17639)), (locals.var_dnm_dn17 / (2.0 * assign12880_body104_body0_e17639)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
                locals.var_dnm = assign12880_body104_body0_e17641;
                locals.var_dnm_dn0 = assign12880_body104_body0_e17641_d_n0;
                locals.var_dnm_dn2 = assign12880_body104_body0_e17641_d_n2;
                locals.var_dnm_dn6 = assign12880_body104_body0_e17641_d_n6;
                locals.var_dnm_dn7 = assign12880_body104_body0_e17641_d_n7;
                locals.var_dnm_dn10 = assign12880_body104_body0_e17641_d_n10;
                locals.var_dnm_dn11 = assign12880_body104_body0_e17641_d_n11;
                locals.var_dnm_dn12 = assign12880_body104_body0_e17641_d_n12;
                locals.var_dnm_dn17 = assign12880_body104_body0_e17641_d_n17;
                let (assign12880_body104_body1_e17654,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign12880_body104_body1_e17652: f64 = (locals.var_m0 + 1.0);
        (assign12880_body104_body1_e17652,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign12880_body104_body1_e17654;
            }
            let (assign12880_body105_e17672, assign12880_body105_e17672_d_n0, assign12880_body105_e17672_d_n2, assign12880_body105_e17672_d_n6, assign12880_body105_e17672_d_n7, assign12880_body105_e17672_d_n10, assign12880_body105_e17672_d_n11, assign12880_body105_e17672_d_n12, assign12880_body105_e17672_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 == 0.0)) {
        let assign12880_body105_e17668: f64 = (2.0 * 2.0);
        let assign12880_body105_e17669: f64 = (1.0 / assign12880_body105_e17668);
        let assign12880_body105_e17670: f64 = (locals.var_dnm).powf(assign12880_body105_e17669);
        (assign12880_body105_e17670, if 0.0 == 0.0 && ((assign12880_body105_e17669) as f64).is_finite() && ((assign12880_body105_e17669) as f64).fract() == 0.0 { if assign12880_body105_e17669 == 0.0 { 0.0 } else { (assign12880_body105_e17669 * ((locals.var_dnm).powf(assign12880_body105_e17669 - 1.0) * locals.var_dnm_dn0)) } } else { (assign12880_body105_e17670 * (assign12880_body105_e17669 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body105_e17669) as f64).is_finite() && ((assign12880_body105_e17669) as f64).fract() == 0.0 { if assign12880_body105_e17669 == 0.0 { 0.0 } else { (assign12880_body105_e17669 * ((locals.var_dnm).powf(assign12880_body105_e17669 - 1.0) * locals.var_dnm_dn2)) } } else { (assign12880_body105_e17670 * (assign12880_body105_e17669 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body105_e17669) as f64).is_finite() && ((assign12880_body105_e17669) as f64).fract() == 0.0 { if assign12880_body105_e17669 == 0.0 { 0.0 } else { (assign12880_body105_e17669 * ((locals.var_dnm).powf(assign12880_body105_e17669 - 1.0) * locals.var_dnm_dn6)) } } else { (assign12880_body105_e17670 * (assign12880_body105_e17669 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body105_e17669) as f64).is_finite() && ((assign12880_body105_e17669) as f64).fract() == 0.0 { if assign12880_body105_e17669 == 0.0 { 0.0 } else { (assign12880_body105_e17669 * ((locals.var_dnm).powf(assign12880_body105_e17669 - 1.0) * locals.var_dnm_dn7)) } } else { (assign12880_body105_e17670 * (assign12880_body105_e17669 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body105_e17669) as f64).is_finite() && ((assign12880_body105_e17669) as f64).fract() == 0.0 { if assign12880_body105_e17669 == 0.0 { 0.0 } else { (assign12880_body105_e17669 * ((locals.var_dnm).powf(assign12880_body105_e17669 - 1.0) * locals.var_dnm_dn10)) } } else { (assign12880_body105_e17670 * (assign12880_body105_e17669 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body105_e17669) as f64).is_finite() && ((assign12880_body105_e17669) as f64).fract() == 0.0 { if assign12880_body105_e17669 == 0.0 { 0.0 } else { (assign12880_body105_e17669 * ((locals.var_dnm).powf(assign12880_body105_e17669 - 1.0) * locals.var_dnm_dn11)) } } else { (assign12880_body105_e17670 * (assign12880_body105_e17669 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body105_e17669) as f64).is_finite() && ((assign12880_body105_e17669) as f64).fract() == 0.0 { if assign12880_body105_e17669 == 0.0 { 0.0 } else { (assign12880_body105_e17669 * ((locals.var_dnm).powf(assign12880_body105_e17669 - 1.0) * locals.var_dnm_dn12)) } } else { (assign12880_body105_e17670 * (assign12880_body105_e17669 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12880_body105_e17669) as f64).is_finite() && ((assign12880_body105_e17669) as f64).fract() == 0.0 { if assign12880_body105_e17669 == 0.0 { 0.0 } else { (assign12880_body105_e17669 * ((locals.var_dnm).powf(assign12880_body105_e17669 - 1.0) * locals.var_dnm_dn17)) } } else { (assign12880_body105_e17670 * (assign12880_body105_e17669 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12880_body105_e17672;
            locals.var_dnm_dn0 = assign12880_body105_e17672_d_n0;
            locals.var_dnm_dn2 = assign12880_body105_e17672_d_n2;
            locals.var_dnm_dn6 = assign12880_body105_e17672_d_n6;
            locals.var_dnm_dn7 = assign12880_body105_e17672_d_n7;
            locals.var_dnm_dn10 = assign12880_body105_e17672_d_n10;
            locals.var_dnm_dn11 = assign12880_body105_e17672_d_n11;
            locals.var_dnm_dn12 = assign12880_body105_e17672_d_n12;
            locals.var_dnm_dn17 = assign12880_body105_e17672_d_n17;
            let (assign12880_body106_e17683, assign12880_body106_e17683_d_n0, assign12880_body106_e17683_d_n2, assign12880_body106_e17683_d_n6, assign12880_body106_e17683_d_n7, assign12880_body106_e17683_d_n10, assign12880_body106_e17683_d_n11, assign12880_body106_e17683_d_n12, assign12880_body106_e17683_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body106_e17681: f64 = (1.0 / locals.var_dnm);
        (assign12880_body106_e17681, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12880_body106_e17683;
            locals.var_dnm_dn0 = assign12880_body106_e17683_d_n0;
            locals.var_dnm_dn2 = assign12880_body106_e17683_d_n2;
            locals.var_dnm_dn6 = assign12880_body106_e17683_d_n6;
            locals.var_dnm_dn7 = assign12880_body106_e17683_d_n7;
            locals.var_dnm_dn10 = assign12880_body106_e17683_d_n10;
            locals.var_dnm_dn11 = assign12880_body106_e17683_d_n11;
            locals.var_dnm_dn12 = assign12880_body106_e17683_d_n12;
            locals.var_dnm_dn17 = assign12880_body106_e17683_d_n17;
            let (assign12880_body107_e17701, assign12880_body107_e17701_d_n0, assign12880_body107_e17701_d_n2, assign12880_body107_e17701_d_n6, assign12880_body107_e17701_d_n7, assign12880_body107_e17701_d_n10, assign12880_body107_e17701_d_n11, assign12880_body107_e17701_d_n12, assign12880_body107_e17701_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body107_e17693: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12880_body107_e17694: f64 = (-assign12880_body107_e17693);
        let assign12880_body107_e17696: f64 = assign12880_body107_e17694;
        let assign12880_body107_e17697: f64 = (locals.var_tmf1 * assign12880_body107_e17696);
        let assign12880_body107_e17699: f64 = (assign12880_body107_e17697 * locals.var_dnm);
        (assign12880_body107_e17699, ((((locals.var_tmf1_dn0 * assign12880_body107_e17696) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)))) * locals.var_dnm) + (assign12880_body107_e17697 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign12880_body107_e17696) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)))) * locals.var_dnm) + (assign12880_body107_e17697 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign12880_body107_e17696) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)))) * locals.var_dnm) + (assign12880_body107_e17697 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign12880_body107_e17696) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)))) * locals.var_dnm) + (assign12880_body107_e17697 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign12880_body107_e17696) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)))) * locals.var_dnm) + (assign12880_body107_e17697 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign12880_body107_e17696) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)))) * locals.var_dnm) + (assign12880_body107_e17697 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign12880_body107_e17696) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)))) * locals.var_dnm) + (assign12880_body107_e17697 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign12880_body107_e17696) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)))) * locals.var_dnm) + (assign12880_body107_e17697 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0__blk381, locals.var_tmf0__blk381_dn0, locals.var_tmf0__blk381_dn2, locals.var_tmf0__blk381_dn6, locals.var_tmf0__blk381_dn7, locals.var_tmf0__blk381_dn10, locals.var_tmf0__blk381_dn11, locals.var_tmf0__blk381_dn12, locals.var_tmf0__blk381_dn17,)
    }
};
            locals.var_tmf0__blk381 = assign12880_body107_e17701;
            locals.var_tmf0__blk381_dn0 = assign12880_body107_e17701_d_n0;
            locals.var_tmf0__blk381_dn2 = assign12880_body107_e17701_d_n2;
            locals.var_tmf0__blk381_dn6 = assign12880_body107_e17701_d_n6;
            locals.var_tmf0__blk381_dn7 = assign12880_body107_e17701_d_n7;
            locals.var_tmf0__blk381_dn10 = assign12880_body107_e17701_d_n10;
            locals.var_tmf0__blk381_dn11 = assign12880_body107_e17701_d_n11;
            locals.var_tmf0__blk381_dn12 = assign12880_body107_e17701_d_n12;
            locals.var_tmf0__blk381_dn17 = assign12880_body107_e17701_d_n17;
            let (assign12880_body108_e17721, assign12880_body108_e17721_d_n0, assign12880_body108_e17721_d_n2, assign12880_body108_e17721_d_n6, assign12880_body108_e17721_d_n7, assign12880_body108_e17721_d_n10, assign12880_body108_e17721_d_n11, assign12880_body108_e17721_d_n12, assign12880_body108_e17721_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body108_e17710: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12880_body108_e17711: f64 = (-assign12880_body108_e17710);
        let assign12880_body108_e17713: f64 = assign12880_body108_e17711;
        let assign12880_body108_e17715: f64 = (assign12880_body108_e17713 * locals.var_xmp);
        let assign12880_body108_e17717: f64 = (assign12880_body108_e17715 * locals.var_dnm);
        let assign12880_body108_e17719: f64 = (assign12880_body108_e17717 / locals.var_arg);
        (assign12880_body108_e17719, ((((((((-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)) * locals.var_xmp) + (assign12880_body108_e17713 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign12880_body108_e17715 * locals.var_dnm_dn0)) * locals.var_arg) - (assign12880_body108_e17717 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)) * locals.var_xmp) + (assign12880_body108_e17713 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign12880_body108_e17715 * locals.var_dnm_dn2)) * locals.var_arg) - (assign12880_body108_e17717 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)) * locals.var_xmp) + (assign12880_body108_e17713 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign12880_body108_e17715 * locals.var_dnm_dn6)) * locals.var_arg) - (assign12880_body108_e17717 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)) * locals.var_xmp) + (assign12880_body108_e17713 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign12880_body108_e17715 * locals.var_dnm_dn7)) * locals.var_arg) - (assign12880_body108_e17717 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)) * locals.var_xmp) + (assign12880_body108_e17713 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign12880_body108_e17715 * locals.var_dnm_dn10)) * locals.var_arg) - (assign12880_body108_e17717 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)) * locals.var_xmp) + (assign12880_body108_e17713 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign12880_body108_e17715 * locals.var_dnm_dn11)) * locals.var_arg) - (assign12880_body108_e17717 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)) * locals.var_xmp) + (assign12880_body108_e17713 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign12880_body108_e17715 * locals.var_dnm_dn12)) * locals.var_arg) - (assign12880_body108_e17717 * locals.var_arg_dn12)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)) * locals.var_xmp) + (assign12880_body108_e17713 * locals.var_xmp_dn17)) * locals.var_dnm) + (assign12880_body108_e17715 * locals.var_dnm_dn17)) * locals.var_arg) - (assign12880_body108_e17717 * locals.var_arg_dn17)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body108_e17721;
            locals.var_t0_dn0 = assign12880_body108_e17721_d_n0;
            locals.var_t0_dn2 = assign12880_body108_e17721_d_n2;
            locals.var_t0_dn6 = assign12880_body108_e17721_d_n6;
            locals.var_t0_dn7 = assign12880_body108_e17721_d_n7;
            locals.var_t0_dn10 = assign12880_body108_e17721_d_n10;
            locals.var_t0_dn11 = assign12880_body108_e17721_d_n11;
            locals.var_t0_dn12 = assign12880_body108_e17721_d_n12;
            locals.var_t0_dn17 = assign12880_body108_e17721_d_n17;
            let (assign12880_body109_e17741, assign12880_body109_e17741_d_n0, assign12880_body109_e17741_d_n2, assign12880_body109_e17741_d_n6, assign12880_body109_e17741_d_n7, assign12880_body109_e17741_d_n10, assign12880_body109_e17741_d_n11, assign12880_body109_e17741_d_n12, assign12880_body109_e17741_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        let assign12880_body109_e17730: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12880_body109_e17733: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12880_body109_e17734: f64 = (-assign12880_body109_e17733);
        let assign12880_body109_e17736: f64 = assign12880_body109_e17734;
        let assign12880_body109_e17737: f64 = (assign12880_body109_e17730 + assign12880_body109_e17736);
        let assign12880_body109_e17739: f64 = (assign12880_body109_e17737 - locals.var_tmf0__blk381);
        (assign12880_body109_e17739, (((locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0) + (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0))) - locals.var_tmf0__blk381_dn0), (((locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2) + (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2))) - locals.var_tmf0__blk381_dn2), (((locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6) + (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6))) - locals.var_tmf0__blk381_dn6), (((locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7) + (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7))) - locals.var_tmf0__blk381_dn7), (((locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10) + (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10))) - locals.var_tmf0__blk381_dn10), (((locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11) + (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11))) - locals.var_tmf0__blk381_dn11), (((locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12) + (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12))) - locals.var_tmf0__blk381_dn12), (((locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17) + (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17))) - locals.var_tmf0__blk381_dn17),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12880_body109_e17741;
            locals.var_q_bl_dep_dn0 = assign12880_body109_e17741_d_n0;
            locals.var_q_bl_dep_dn2 = assign12880_body109_e17741_d_n2;
            locals.var_q_bl_dep_dn6 = assign12880_body109_e17741_d_n6;
            locals.var_q_bl_dep_dn7 = assign12880_body109_e17741_d_n7;
            locals.var_q_bl_dep_dn10 = assign12880_body109_e17741_d_n10;
            locals.var_q_bl_dep_dn11 = assign12880_body109_e17741_d_n11;
            locals.var_q_bl_dep_dn12 = assign12880_body109_e17741_d_n12;
            locals.var_q_bl_dep_dn17 = assign12880_body109_e17741_d_n17;
            let (assign12880_body110_e17750, assign12880_body110_e17750_d_n0, assign12880_body110_e17750_d_n2, assign12880_body110_e17750_d_n6, assign12880_body110_e17750_d_n7, assign12880_body110_e17750_d_n10, assign12880_body110_e17750_d_n11, assign12880_body110_e17750_d_n12, assign12880_body110_e17750_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body110_e17750;
            locals.var_t0_dn0 = assign12880_body110_e17750_d_n0;
            locals.var_t0_dn2 = assign12880_body110_e17750_d_n2;
            locals.var_t0_dn6 = assign12880_body110_e17750_d_n6;
            locals.var_t0_dn7 = assign12880_body110_e17750_d_n7;
            locals.var_t0_dn10 = assign12880_body110_e17750_d_n10;
            locals.var_t0_dn11 = assign12880_body110_e17750_d_n11;
            locals.var_t0_dn12 = assign12880_body110_e17750_d_n12;
            locals.var_t0_dn17 = assign12880_body110_e17750_d_n17;
            let (assign12880_body111_e17760, assign12880_body111_e17760_d_n0, assign12880_body111_e17760_d_n2, assign12880_body111_e17760_d_n6, assign12880_body111_e17760_d_n7, assign12880_body111_e17760_d_n10, assign12880_body111_e17760_d_n11, assign12880_body111_e17760_d_n12, assign12880_body111_e17760_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 == 0.0)) {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12880_body111_e17760;
            locals.var_q_bl_dep_dn0 = assign12880_body111_e17760_d_n0;
            locals.var_q_bl_dep_dn2 = assign12880_body111_e17760_d_n2;
            locals.var_q_bl_dep_dn6 = assign12880_body111_e17760_d_n6;
            locals.var_q_bl_dep_dn7 = assign12880_body111_e17760_d_n7;
            locals.var_q_bl_dep_dn10 = assign12880_body111_e17760_d_n10;
            locals.var_q_bl_dep_dn11 = assign12880_body111_e17760_d_n11;
            locals.var_q_bl_dep_dn12 = assign12880_body111_e17760_d_n12;
            locals.var_q_bl_dep_dn17 = assign12880_body111_e17760_d_n17;
            let (assign12880_body112_e17770, assign12880_body112_e17770_d_n0, assign12880_body112_e17770_d_n2, assign12880_body112_e17770_d_n6, assign12880_body112_e17770_d_n7, assign12880_body112_e17770_d_n10, assign12880_body112_e17770_d_n11, assign12880_body112_e17770_d_n12, assign12880_body112_e17770_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard396 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12880_body112_e17770;
            locals.var_t0_dn0 = assign12880_body112_e17770_d_n0;
            locals.var_t0_dn2 = assign12880_body112_e17770_d_n2;
            locals.var_t0_dn6 = assign12880_body112_e17770_d_n6;
            locals.var_t0_dn7 = assign12880_body112_e17770_d_n7;
            locals.var_t0_dn10 = assign12880_body112_e17770_d_n10;
            locals.var_t0_dn11 = assign12880_body112_e17770_d_n11;
            locals.var_t0_dn12 = assign12880_body112_e17770_d_n12;
            locals.var_t0_dn17 = assign12880_body112_e17770_d_n17;
            let (assign12880_body113_e17779, assign12880_body113_e17779_d_n0, assign12880_body113_e17779_d_n2, assign12880_body113_e17779_d_n6, assign12880_body113_e17779_d_n7, assign12880_body113_e17779_d_n10, assign12880_body113_e17779_d_n11, assign12880_body113_e17779_d_n12, assign12880_body113_e17779_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body113_e17777: f64 = (locals.var_q_bl_dep_dpss * locals.var_t0);
        (assign12880_body113_e17777, ((locals.var_q_bl_dep_dpss_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpss_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpss_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpss_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpss_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpss_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpss_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpss_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12880_body113_e17779;
            locals.var_q_bl_dep_dpss_dn0 = assign12880_body113_e17779_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12880_body113_e17779_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12880_body113_e17779_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12880_body113_e17779_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12880_body113_e17779_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12880_body113_e17779_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12880_body113_e17779_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12880_body113_e17779_d_n17;
            let (assign12880_body114_e17788, assign12880_body114_e17788_d_n0, assign12880_body114_e17788_d_n2, assign12880_body114_e17788_d_n6, assign12880_body114_e17788_d_n7, assign12880_body114_e17788_d_n10, assign12880_body114_e17788_d_n11, assign12880_body114_e17788_d_n12, assign12880_body114_e17788_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body114_e17786: f64 = (locals.var_q_bl_dep_dpbs * locals.var_t0);
        (assign12880_body114_e17786, ((locals.var_q_bl_dep_dpbs_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpbs_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpbs_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpbs_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpbs_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpbs_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpbs_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpbs_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12880_body114_e17788;
            locals.var_q_bl_dep_dpbs_dn0 = assign12880_body114_e17788_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12880_body114_e17788_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12880_body114_e17788_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12880_body114_e17788_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12880_body114_e17788_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12880_body114_e17788_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12880_body114_e17788_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12880_body114_e17788_d_n17;
            let (assign12880_body115_e17797, assign12880_body115_e17797_d_n0, assign12880_body115_e17797_d_n2, assign12880_body115_e17797_d_n6, assign12880_body115_e17797_d_n7, assign12880_body115_e17797_d_n10, assign12880_body115_e17797_d_n11, assign12880_body115_e17797_d_n12, assign12880_body115_e17797_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body115_e17795: f64 = (locals.var_q_sl_dep + locals.var_q_bl_dep);
        (assign12880_body115_e17795, (locals.var_q_sl_dep_dn0 + locals.var_q_bl_dep_dn0), (locals.var_q_sl_dep_dn2 + locals.var_q_bl_dep_dn2), (locals.var_q_sl_dep_dn6 + locals.var_q_bl_dep_dn6), (locals.var_q_sl_dep_dn7 + locals.var_q_bl_dep_dn7), (locals.var_q_sl_dep_dn10 + locals.var_q_bl_dep_dn10), (locals.var_q_sl_dep_dn11 + locals.var_q_bl_dep_dn11), (locals.var_q_sl_dep_dn12 + locals.var_q_bl_dep_dn12), (locals.var_q_sl_dep_dn17 + locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
            locals.var_q_depl = assign12880_body115_e17797;
            locals.var_q_depl_dn0 = assign12880_body115_e17797_d_n0;
            locals.var_q_depl_dn2 = assign12880_body115_e17797_d_n2;
            locals.var_q_depl_dn6 = assign12880_body115_e17797_d_n6;
            locals.var_q_depl_dn7 = assign12880_body115_e17797_d_n7;
            locals.var_q_depl_dn10 = assign12880_body115_e17797_d_n10;
            locals.var_q_depl_dn11 = assign12880_body115_e17797_d_n11;
            locals.var_q_depl_dn12 = assign12880_body115_e17797_d_n12;
            locals.var_q_depl_dn17 = assign12880_body115_e17797_d_n17;
            let assign12880_body116_e17804: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_lp_sl > 3.0)) { 1.0 } else { 0.0 };
            locals.var_guard402 = assign12880_body116_e17804;
            let (assign12880_body117_e17813,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 != 0.0)) {
        (locals.var_lp_sl,)
    } else {
        (locals.var_flg_brk8,)
    }
};
            locals.var_flg_brk8 = assign12880_body117_e17813;
            let (assign12880_body118_e17822,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12880_body118_e17822;
            let (assign12880_body119_e17846, assign12880_body119_e17846_d_n0, assign12880_body119_e17846_d_n2, assign12880_body119_e17846_d_n6, assign12880_body119_e17846_d_n7, assign12880_body119_e17846_d_n10, assign12880_body119_e17846_d_n11, assign12880_body119_e17846_d_n12, assign12880_body119_e17846_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body119_e17832: f64 = (locals.var_phi_sl_soi - locals.var_vgpz);
        let assign12880_body119_e17836: f64 = (locals.var_q_sl_bulk + locals.var_q_sl_dep);
        let assign12880_body119_e17838: f64 = (assign12880_body119_e17836 + locals.var_q_nl);
        let assign12880_body119_e17840: f64 = (assign12880_body119_e17838 + locals.var_q_bl_dep);
        let assign12880_body119_e17842: f64 = (assign12880_body119_e17840 + locals.var_qhs);
        let assign12880_body119_e17843: f64 = (locals.var_c_fox_inv * assign12880_body119_e17842);
        let assign12880_body119_e17844: f64 = (assign12880_body119_e17832 - assign12880_body119_e17843);
        (assign12880_body119_e17844, ((locals.var_phi_sl_soi_dn0 - locals.var_vgpz_dn0) - ((locals.var_c_fox_inv_dn0 * assign12880_body119_e17842) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn0 + locals.var_q_sl_dep_dn0) + locals.var_q_nl_dn0) + locals.var_q_bl_dep_dn0) + locals.var_qhs_dn0)))), ((locals.var_phi_sl_soi_dn2 - locals.var_vgpz_dn2) - ((locals.var_c_fox_inv_dn2 * assign12880_body119_e17842) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn2 + locals.var_q_sl_dep_dn2) + locals.var_q_nl_dn2) + locals.var_q_bl_dep_dn2) + locals.var_qhs_dn2)))), ((locals.var_phi_sl_soi_dn6 - locals.var_vgpz_dn6) - ((locals.var_c_fox_inv_dn6 * assign12880_body119_e17842) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn6 + locals.var_q_sl_dep_dn6) + locals.var_q_nl_dn6) + locals.var_q_bl_dep_dn6) + locals.var_qhs_dn6)))), ((locals.var_phi_sl_soi_dn7 - locals.var_vgpz_dn7) - ((locals.var_c_fox_inv_dn7 * assign12880_body119_e17842) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn7 + locals.var_q_sl_dep_dn7) + locals.var_q_nl_dn7) + locals.var_q_bl_dep_dn7) + locals.var_qhs_dn7)))), ((locals.var_phi_sl_soi_dn10 - locals.var_vgpz_dn10) - ((locals.var_c_fox_inv_dn10 * assign12880_body119_e17842) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn10 + locals.var_q_sl_dep_dn10) + locals.var_q_nl_dn10) + locals.var_q_bl_dep_dn10) + locals.var_qhs_dn10)))), ((locals.var_phi_sl_soi_dn11 - locals.var_vgpz_dn11) - ((locals.var_c_fox_inv_dn11 * assign12880_body119_e17842) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn11 + locals.var_q_sl_dep_dn11) + locals.var_q_nl_dn11) + locals.var_q_bl_dep_dn11) + locals.var_qhs_dn11)))), ((locals.var_phi_sl_soi_dn12 - locals.var_vgpz_dn12) - ((locals.var_c_fox_inv_dn12 * assign12880_body119_e17842) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn12 + locals.var_q_sl_dep_dn12) + locals.var_q_nl_dn12) + locals.var_q_bl_dep_dn12) + locals.var_qhs_dn12)))), ((locals.var_phi_sl_soi_dn17 - locals.var_vgpz_dn17) - ((locals.var_c_fox_inv_dn17 * assign12880_body119_e17842) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn17 + locals.var_q_sl_dep_dn17) + locals.var_q_nl_dn17) + locals.var_q_bl_dep_dn17) + locals.var_qhs_dn17)))),)
    } else {
        (locals.var_pf1__blk359, locals.var_pf1__blk359_dn0, locals.var_pf1__blk359_dn2, locals.var_pf1__blk359_dn6, locals.var_pf1__blk359_dn7, locals.var_pf1__blk359_dn10, locals.var_pf1__blk359_dn11, locals.var_pf1__blk359_dn12, locals.var_pf1__blk359_dn17,)
    }
};
            locals.var_pf1__blk359 = assign12880_body119_e17846;
            locals.var_pf1__blk359_dn0 = assign12880_body119_e17846_d_n0;
            locals.var_pf1__blk359_dn2 = assign12880_body119_e17846_d_n2;
            locals.var_pf1__blk359_dn6 = assign12880_body119_e17846_d_n6;
            locals.var_pf1__blk359_dn7 = assign12880_body119_e17846_d_n7;
            locals.var_pf1__blk359_dn10 = assign12880_body119_e17846_d_n10;
            locals.var_pf1__blk359_dn11 = assign12880_body119_e17846_d_n11;
            locals.var_pf1__blk359_dn12 = assign12880_body119_e17846_d_n12;
            locals.var_pf1__blk359_dn17 = assign12880_body119_e17846_d_n17;
            let (assign12880_body120_e17862, assign12880_body120_e17862_d_n0, assign12880_body120_e17862_d_n2, assign12880_body120_e17862_d_n6, assign12880_body120_e17862_d_n7, assign12880_body120_e17862_d_n10, assign12880_body120_e17862_d_n11, assign12880_body120_e17862_d_n12, assign12880_body120_e17862_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body120_e17858: f64 = (locals.var_q_nl_dpss + locals.var_q_bl_dep_dpss);
        let assign12880_body120_e17859: f64 = (locals.var_c_fox_inv * assign12880_body120_e17858);
        let assign12880_body120_e17860: f64 = (1.0 - assign12880_body120_e17859);
        (assign12880_body120_e17860, (-((locals.var_c_fox_inv_dn0 * assign12880_body120_e17858) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn0 + locals.var_q_bl_dep_dpss_dn0)))), (-((locals.var_c_fox_inv_dn2 * assign12880_body120_e17858) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn2 + locals.var_q_bl_dep_dpss_dn2)))), (-((locals.var_c_fox_inv_dn6 * assign12880_body120_e17858) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn6 + locals.var_q_bl_dep_dpss_dn6)))), (-((locals.var_c_fox_inv_dn7 * assign12880_body120_e17858) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn7 + locals.var_q_bl_dep_dpss_dn7)))), (-((locals.var_c_fox_inv_dn10 * assign12880_body120_e17858) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn10 + locals.var_q_bl_dep_dpss_dn10)))), (-((locals.var_c_fox_inv_dn11 * assign12880_body120_e17858) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn11 + locals.var_q_bl_dep_dpss_dn11)))), (-((locals.var_c_fox_inv_dn12 * assign12880_body120_e17858) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn12 + locals.var_q_bl_dep_dpss_dn12)))), (-((locals.var_c_fox_inv_dn17 * assign12880_body120_e17858) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn17 + locals.var_q_bl_dep_dpss_dn17)))),)
    } else {
        (locals.var_pf11__blk360, locals.var_pf11__blk360_dn0, locals.var_pf11__blk360_dn2, locals.var_pf11__blk360_dn6, locals.var_pf11__blk360_dn7, locals.var_pf11__blk360_dn10, locals.var_pf11__blk360_dn11, locals.var_pf11__blk360_dn12, locals.var_pf11__blk360_dn17,)
    }
};
            locals.var_pf11__blk360 = assign12880_body120_e17862;
            locals.var_pf11__blk360_dn0 = assign12880_body120_e17862_d_n0;
            locals.var_pf11__blk360_dn2 = assign12880_body120_e17862_d_n2;
            locals.var_pf11__blk360_dn6 = assign12880_body120_e17862_d_n6;
            locals.var_pf11__blk360_dn7 = assign12880_body120_e17862_d_n7;
            locals.var_pf11__blk360_dn10 = assign12880_body120_e17862_d_n10;
            locals.var_pf11__blk360_dn11 = assign12880_body120_e17862_d_n11;
            locals.var_pf11__blk360_dn12 = assign12880_body120_e17862_d_n12;
            locals.var_pf11__blk360_dn17 = assign12880_body120_e17862_d_n17;
            let (assign12880_body121_e17875, assign12880_body121_e17875_d_n0, assign12880_body121_e17875_d_n2, assign12880_body121_e17875_d_n6, assign12880_body121_e17875_d_n7, assign12880_body121_e17875_d_n10, assign12880_body121_e17875_d_n11, assign12880_body121_e17875_d_n12, assign12880_body121_e17875_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body121_e17871: f64 = (-locals.var_c_fox_inv);
        let assign12880_body121_e17873: f64 = (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs);
        (assign12880_body121_e17873, (((-locals.var_c_fox_inv_dn0) * locals.var_q_bl_dep_dpbs) + (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs_dn0)), (((-locals.var_c_fox_inv_dn2) * locals.var_q_bl_dep_dpbs) + (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs_dn2)), (((-locals.var_c_fox_inv_dn6) * locals.var_q_bl_dep_dpbs) + (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs_dn6)), (((-locals.var_c_fox_inv_dn7) * locals.var_q_bl_dep_dpbs) + (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs_dn7)), (((-locals.var_c_fox_inv_dn10) * locals.var_q_bl_dep_dpbs) + (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs_dn10)), (((-locals.var_c_fox_inv_dn11) * locals.var_q_bl_dep_dpbs) + (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs_dn11)), (((-locals.var_c_fox_inv_dn12) * locals.var_q_bl_dep_dpbs) + (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs_dn12)), (((-locals.var_c_fox_inv_dn17) * locals.var_q_bl_dep_dpbs) + (assign12880_body121_e17871 * locals.var_q_bl_dep_dpbs_dn17)),)
    } else {
        (locals.var_pf12__blk361, locals.var_pf12__blk361_dn0, locals.var_pf12__blk361_dn2, locals.var_pf12__blk361_dn6, locals.var_pf12__blk361_dn7, locals.var_pf12__blk361_dn10, locals.var_pf12__blk361_dn11, locals.var_pf12__blk361_dn12, locals.var_pf12__blk361_dn17,)
    }
};
            locals.var_pf12__blk361 = assign12880_body121_e17875;
            locals.var_pf12__blk361_dn0 = assign12880_body121_e17875_d_n0;
            locals.var_pf12__blk361_dn2 = assign12880_body121_e17875_d_n2;
            locals.var_pf12__blk361_dn6 = assign12880_body121_e17875_d_n6;
            locals.var_pf12__blk361_dn7 = assign12880_body121_e17875_d_n7;
            locals.var_pf12__blk361_dn10 = assign12880_body121_e17875_d_n10;
            locals.var_pf12__blk361_dn11 = assign12880_body121_e17875_d_n11;
            locals.var_pf12__blk361_dn12 = assign12880_body121_e17875_d_n12;
            locals.var_pf12__blk361_dn17 = assign12880_body121_e17875_d_n17;
            let (assign12880_body122_e17888, assign12880_body122_e17888_d_n0, assign12880_body122_e17888_d_n2, assign12880_body122_e17888_d_n6, assign12880_body122_e17888_d_n7, assign12880_body122_e17888_d_n10, assign12880_body122_e17888_d_n11, assign12880_body122_e17888_d_n12, assign12880_body122_e17888_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body122_e17884: f64 = (-locals.var_c_fox_inv);
        let assign12880_body122_e17886: f64 = (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb);
        (assign12880_body122_e17886, (((-locals.var_c_fox_inv_dn0) * locals.var_q_sl_bulk_dpsb) + (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb_dn0)), (((-locals.var_c_fox_inv_dn2) * locals.var_q_sl_bulk_dpsb) + (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb_dn2)), (((-locals.var_c_fox_inv_dn6) * locals.var_q_sl_bulk_dpsb) + (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb_dn6)), (((-locals.var_c_fox_inv_dn7) * locals.var_q_sl_bulk_dpsb) + (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb_dn7)), (((-locals.var_c_fox_inv_dn10) * locals.var_q_sl_bulk_dpsb) + (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb_dn10)), (((-locals.var_c_fox_inv_dn11) * locals.var_q_sl_bulk_dpsb) + (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb_dn11)), (((-locals.var_c_fox_inv_dn12) * locals.var_q_sl_bulk_dpsb) + (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb_dn12)), (((-locals.var_c_fox_inv_dn17) * locals.var_q_sl_bulk_dpsb) + (assign12880_body122_e17884 * locals.var_q_sl_bulk_dpsb_dn17)),)
    } else {
        (locals.var_pf13__blk362, locals.var_pf13__blk362_dn0, locals.var_pf13__blk362_dn2, locals.var_pf13__blk362_dn6, locals.var_pf13__blk362_dn7, locals.var_pf13__blk362_dn10, locals.var_pf13__blk362_dn11, locals.var_pf13__blk362_dn12, locals.var_pf13__blk362_dn17,)
    }
};
            locals.var_pf13__blk362 = assign12880_body122_e17888;
            locals.var_pf13__blk362_dn0 = assign12880_body122_e17888_d_n0;
            locals.var_pf13__blk362_dn2 = assign12880_body122_e17888_d_n2;
            locals.var_pf13__blk362_dn6 = assign12880_body122_e17888_d_n6;
            locals.var_pf13__blk362_dn7 = assign12880_body122_e17888_d_n7;
            locals.var_pf13__blk362_dn10 = assign12880_body122_e17888_d_n10;
            locals.var_pf13__blk362_dn11 = assign12880_body122_e17888_d_n11;
            locals.var_pf13__blk362_dn12 = assign12880_body122_e17888_d_n12;
            locals.var_pf13__blk362_dn17 = assign12880_body122_e17888_d_n17;
            let (assign12880_body123_e17906, assign12880_body123_e17906_d_n0, assign12880_body123_e17906_d_n2, assign12880_body123_e17906_d_n6, assign12880_body123_e17906_d_n7, assign12880_body123_e17906_d_n10, assign12880_body123_e17906_d_n11, assign12880_body123_e17906_d_n12, assign12880_body123_e17906_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body123_e17900: f64 = (0.5 * locals.var_q_fd_soi);
        let assign12880_body123_e17902: f64 = (assign12880_body123_e17900 + locals.var_q_sl_bulk);
        let assign12880_body123_e17903: f64 = (locals.var_c_soi_inv__blk111 * assign12880_body123_e17902);
        let assign12880_body123_e17904: f64 = (locals.var_phi_sl_soi + assign12880_body123_e17903);
        (assign12880_body123_e17904, (locals.var_phi_sl_soi_dn0 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn0) + locals.var_q_sl_bulk_dn0))), (locals.var_phi_sl_soi_dn2 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn2) + locals.var_q_sl_bulk_dn2))), (locals.var_phi_sl_soi_dn6 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn6) + locals.var_q_sl_bulk_dn6))), (locals.var_phi_sl_soi_dn7 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn7) + locals.var_q_sl_bulk_dn7))), (locals.var_phi_sl_soi_dn10 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn10) + locals.var_q_sl_bulk_dn10))), (locals.var_phi_sl_soi_dn11 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn11) + locals.var_q_sl_bulk_dn11))), (locals.var_phi_sl_soi_dn12 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn12) + locals.var_q_sl_bulk_dn12))), (locals.var_phi_sl_soi_dn17 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn17) + locals.var_q_sl_bulk_dn17))),)
    } else {
        (locals.var_t1__blk349, locals.var_t1__blk349_dn0, locals.var_t1__blk349_dn2, locals.var_t1__blk349_dn6, locals.var_t1__blk349_dn7, locals.var_t1__blk349_dn10, locals.var_t1__blk349_dn11, locals.var_t1__blk349_dn12, locals.var_t1__blk349_dn17,)
    }
};
            locals.var_t1__blk349 = assign12880_body123_e17906;
            locals.var_t1__blk349_dn0 = assign12880_body123_e17906_d_n0;
            locals.var_t1__blk349_dn2 = assign12880_body123_e17906_d_n2;
            locals.var_t1__blk349_dn6 = assign12880_body123_e17906_d_n6;
            locals.var_t1__blk349_dn7 = assign12880_body123_e17906_d_n7;
            locals.var_t1__blk349_dn10 = assign12880_body123_e17906_d_n10;
            locals.var_t1__blk349_dn11 = assign12880_body123_e17906_d_n11;
            locals.var_t1__blk349_dn12 = assign12880_body123_e17906_d_n12;
            locals.var_t1__blk349_dn17 = assign12880_body123_e17906_d_n17;
            let (assign12880_body124_e17918, assign12880_body124_e17918_d_n0, assign12880_body124_e17918_d_n2, assign12880_body124_e17918_d_n6, assign12880_body124_e17918_d_n7, assign12880_body124_e17918_d_n10, assign12880_body124_e17918_d_n11, assign12880_body124_e17918_d_n12, assign12880_body124_e17918_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body124_e17916: f64 = (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb);
        (assign12880_body124_e17916, (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb_dn0), (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb_dn2), (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb_dn6), (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb_dn7), (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb_dn10), (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb_dn11), (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb_dn12), (locals.var_c_soi_inv__blk111 * locals.var_q_sl_bulk_dpsb_dn17),)
    } else {
        (locals.var_t3__blk351, locals.var_t3__blk351_dn0, locals.var_t3__blk351_dn2, locals.var_t3__blk351_dn6, locals.var_t3__blk351_dn7, locals.var_t3__blk351_dn10, locals.var_t3__blk351_dn11, locals.var_t3__blk351_dn12, locals.var_t3__blk351_dn17,)
    }
};
            locals.var_t3__blk351 = assign12880_body124_e17918;
            locals.var_t3__blk351_dn0 = assign12880_body124_e17918_d_n0;
            locals.var_t3__blk351_dn2 = assign12880_body124_e17918_d_n2;
            locals.var_t3__blk351_dn6 = assign12880_body124_e17918_d_n6;
            locals.var_t3__blk351_dn7 = assign12880_body124_e17918_d_n7;
            locals.var_t3__blk351_dn10 = assign12880_body124_e17918_d_n10;
            locals.var_t3__blk351_dn11 = assign12880_body124_e17918_d_n11;
            locals.var_t3__blk351_dn12 = assign12880_body124_e17918_d_n12;
            locals.var_t3__blk351_dn17 = assign12880_body124_e17918_d_n17;
            let (assign12880_body125_e17930, assign12880_body125_e17930_d_n0, assign12880_body125_e17930_d_n2, assign12880_body125_e17930_d_n6, assign12880_body125_e17930_d_n7, assign12880_body125_e17930_d_n10, assign12880_body125_e17930_d_n11, assign12880_body125_e17930_d_n12, assign12880_body125_e17930_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body125_e17928: f64 = (locals.var_phi_bl_soi - locals.var_t1__blk349);
        (assign12880_body125_e17928, (locals.var_phi_bl_soi_dn0 - locals.var_t1__blk349_dn0), (locals.var_phi_bl_soi_dn2 - locals.var_t1__blk349_dn2), (locals.var_phi_bl_soi_dn6 - locals.var_t1__blk349_dn6), (locals.var_phi_bl_soi_dn7 - locals.var_t1__blk349_dn7), (locals.var_phi_bl_soi_dn10 - locals.var_t1__blk349_dn10), (locals.var_phi_bl_soi_dn11 - locals.var_t1__blk349_dn11), (locals.var_phi_bl_soi_dn12 - locals.var_t1__blk349_dn12), (locals.var_phi_bl_soi_dn17 - locals.var_t1__blk349_dn17),)
    } else {
        (locals.var_pf2__blk363, locals.var_pf2__blk363_dn0, locals.var_pf2__blk363_dn2, locals.var_pf2__blk363_dn6, locals.var_pf2__blk363_dn7, locals.var_pf2__blk363_dn10, locals.var_pf2__blk363_dn11, locals.var_pf2__blk363_dn12, locals.var_pf2__blk363_dn17,)
    }
};
            locals.var_pf2__blk363 = assign12880_body125_e17930;
            locals.var_pf2__blk363_dn0 = assign12880_body125_e17930_d_n0;
            locals.var_pf2__blk363_dn2 = assign12880_body125_e17930_d_n2;
            locals.var_pf2__blk363_dn6 = assign12880_body125_e17930_d_n6;
            locals.var_pf2__blk363_dn7 = assign12880_body125_e17930_d_n7;
            locals.var_pf2__blk363_dn10 = assign12880_body125_e17930_d_n10;
            locals.var_pf2__blk363_dn11 = assign12880_body125_e17930_d_n11;
            locals.var_pf2__blk363_dn12 = assign12880_body125_e17930_d_n12;
            locals.var_pf2__blk363_dn17 = assign12880_body125_e17930_d_n17;
            let (assign12880_body126_e17941,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body126_e17939: f64 = (-1.0);
        (assign12880_body126_e17939,)
    } else {
        (locals.var_pf21__blk364,)
    }
};
            locals.var_pf21__blk364 = assign12880_body126_e17941;
            let (assign12880_body127_e17951,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_pf22__blk365,)
    }
};
            locals.var_pf22__blk365 = assign12880_body127_e17951;
            let (assign12880_body128_e17962, assign12880_body128_e17962_d_n0, assign12880_body128_e17962_d_n2, assign12880_body128_e17962_d_n6, assign12880_body128_e17962_d_n7, assign12880_body128_e17962_d_n10, assign12880_body128_e17962_d_n11, assign12880_body128_e17962_d_n12, assign12880_body128_e17962_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body128_e17960: f64 = (-locals.var_t3__blk351);
        (assign12880_body128_e17960, (-locals.var_t3__blk351_dn0), (-locals.var_t3__blk351_dn2), (-locals.var_t3__blk351_dn6), (-locals.var_t3__blk351_dn7), (-locals.var_t3__blk351_dn10), (-locals.var_t3__blk351_dn11), (-locals.var_t3__blk351_dn12), (-locals.var_t3__blk351_dn17),)
    } else {
        (locals.var_pf23__blk366, locals.var_pf23__blk366_dn0, locals.var_pf23__blk366_dn2, locals.var_pf23__blk366_dn6, locals.var_pf23__blk366_dn7, locals.var_pf23__blk366_dn10, locals.var_pf23__blk366_dn11, locals.var_pf23__blk366_dn12, locals.var_pf23__blk366_dn17,)
    }
};
            locals.var_pf23__blk366 = assign12880_body128_e17962;
            locals.var_pf23__blk366_dn0 = assign12880_body128_e17962_d_n0;
            locals.var_pf23__blk366_dn2 = assign12880_body128_e17962_d_n2;
            locals.var_pf23__blk366_dn6 = assign12880_body128_e17962_d_n6;
            locals.var_pf23__blk366_dn7 = assign12880_body128_e17962_d_n7;
            locals.var_pf23__blk366_dn10 = assign12880_body128_e17962_d_n10;
            locals.var_pf23__blk366_dn11 = assign12880_body128_e17962_d_n11;
            locals.var_pf23__blk366_dn12 = assign12880_body128_e17962_d_n12;
            locals.var_pf23__blk366_dn17 = assign12880_body128_e17962_d_n17;
            let (assign12880_body129_e17978, assign12880_body129_e17978_d_n0, assign12880_body129_e17978_d_n2, assign12880_body129_e17978_d_n6, assign12880_body129_e17978_d_n7, assign12880_body129_e17978_d_n10, assign12880_body129_e17978_d_n11, assign12880_body129_e17978_d_n12, assign12880_body129_e17978_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body129_e17972: f64 = (locals.var_phi_sl_bulk - locals.var_phi_bl_soi);
        let assign12880_body129_e17975: f64 = (locals.var_c_box_inv * locals.var_q_sl_bulk);
        let assign12880_body129_e17976: f64 = (assign12880_body129_e17972 - assign12880_body129_e17975);
        (assign12880_body129_e17976, ((locals.var_phi_sl_bulk_dn0 - locals.var_phi_bl_soi_dn0) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn0)), ((locals.var_phi_sl_bulk_dn2 - locals.var_phi_bl_soi_dn2) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn2)), ((locals.var_phi_sl_bulk_dn6 - locals.var_phi_bl_soi_dn6) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn6)), ((locals.var_phi_sl_bulk_dn7 - locals.var_phi_bl_soi_dn7) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn7)), ((locals.var_phi_sl_bulk_dn10 - locals.var_phi_bl_soi_dn10) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn10)), ((locals.var_phi_sl_bulk_dn11 - locals.var_phi_bl_soi_dn11) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn11)), ((locals.var_phi_sl_bulk_dn12 - locals.var_phi_bl_soi_dn12) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn12)), ((locals.var_phi_sl_bulk_dn17 - locals.var_phi_bl_soi_dn17) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn17)),)
    } else {
        (locals.var_pf3__blk367, locals.var_pf3__blk367_dn0, locals.var_pf3__blk367_dn2, locals.var_pf3__blk367_dn6, locals.var_pf3__blk367_dn7, locals.var_pf3__blk367_dn10, locals.var_pf3__blk367_dn11, locals.var_pf3__blk367_dn12, locals.var_pf3__blk367_dn17,)
    }
};
            locals.var_pf3__blk367 = assign12880_body129_e17978;
            locals.var_pf3__blk367_dn0 = assign12880_body129_e17978_d_n0;
            locals.var_pf3__blk367_dn2 = assign12880_body129_e17978_d_n2;
            locals.var_pf3__blk367_dn6 = assign12880_body129_e17978_d_n6;
            locals.var_pf3__blk367_dn7 = assign12880_body129_e17978_d_n7;
            locals.var_pf3__blk367_dn10 = assign12880_body129_e17978_d_n10;
            locals.var_pf3__blk367_dn11 = assign12880_body129_e17978_d_n11;
            locals.var_pf3__blk367_dn12 = assign12880_body129_e17978_d_n12;
            locals.var_pf3__blk367_dn17 = assign12880_body129_e17978_d_n17;
            let (assign12880_body130_e17989,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body130_e17987: f64 = (-1.0);
        (assign12880_body130_e17987,)
    } else {
        (locals.var_pf32__blk368,)
    }
};
            locals.var_pf32__blk368 = assign12880_body130_e17989;
            let (assign12880_body131_e18003, assign12880_body131_e18003_d_n0, assign12880_body131_e18003_d_n2, assign12880_body131_e18003_d_n6, assign12880_body131_e18003_d_n7, assign12880_body131_e18003_d_n10, assign12880_body131_e18003_d_n11, assign12880_body131_e18003_d_n12, assign12880_body131_e18003_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body131_e18000: f64 = (locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb);
        let assign12880_body131_e18001: f64 = (1.0 - assign12880_body131_e18000);
        (assign12880_body131_e18001, (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn0)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn2)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn6)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn7)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn10)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn11)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn12)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn17)),)
    } else {
        (locals.var_pf33__blk369, locals.var_pf33__blk369_dn0, locals.var_pf33__blk369_dn2, locals.var_pf33__blk369_dn6, locals.var_pf33__blk369_dn7, locals.var_pf33__blk369_dn10, locals.var_pf33__blk369_dn11, locals.var_pf33__blk369_dn12, locals.var_pf33__blk369_dn17,)
    }
};
            locals.var_pf33__blk369 = assign12880_body131_e18003;
            locals.var_pf33__blk369_dn0 = assign12880_body131_e18003_d_n0;
            locals.var_pf33__blk369_dn2 = assign12880_body131_e18003_d_n2;
            locals.var_pf33__blk369_dn6 = assign12880_body131_e18003_d_n6;
            locals.var_pf33__blk369_dn7 = assign12880_body131_e18003_d_n7;
            locals.var_pf33__blk369_dn10 = assign12880_body131_e18003_d_n10;
            locals.var_pf33__blk369_dn11 = assign12880_body131_e18003_d_n11;
            locals.var_pf33__blk369_dn12 = assign12880_body131_e18003_d_n12;
            locals.var_pf33__blk369_dn17 = assign12880_body131_e18003_d_n17;
            let (assign12880_body132_e18035, assign12880_body132_e18035_d_n0, assign12880_body132_e18035_d_n2, assign12880_body132_e18035_d_n6, assign12880_body132_e18035_d_n7, assign12880_body132_e18035_d_n10, assign12880_body132_e18035_d_n11, assign12880_body132_e18035_d_n12, assign12880_body132_e18035_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body132_e18013: f64 = (locals.var_pf11__blk360 * locals.var_pf22__blk365);
        let assign12880_body132_e18015: f64 = (assign12880_body132_e18013 * locals.var_pf33__blk369);
        let assign12880_body132_e18018: f64 = (locals.var_pf11__blk360 * locals.var_pf23__blk366);
        let assign12880_body132_e18020: f64 = (assign12880_body132_e18018 * locals.var_pf32__blk368);
        let assign12880_body132_e18021: f64 = (assign12880_body132_e18015 - assign12880_body132_e18020);
        let assign12880_body132_e18024: f64 = (locals.var_pf12__blk361 * locals.var_pf21__blk364);
        let assign12880_body132_e18026: f64 = (assign12880_body132_e18024 * locals.var_pf33__blk369);
        let assign12880_body132_e18027: f64 = (assign12880_body132_e18021 - assign12880_body132_e18026);
        let assign12880_body132_e18030: f64 = (locals.var_pf13__blk362 * locals.var_pf21__blk364);
        let assign12880_body132_e18032: f64 = (assign12880_body132_e18030 * locals.var_pf32__blk368);
        let assign12880_body132_e18033: f64 = (assign12880_body132_e18027 + assign12880_body132_e18032);
        (assign12880_body132_e18033, ((((((locals.var_pf11__blk360_dn0 * locals.var_pf22__blk365) * locals.var_pf33__blk369) + (assign12880_body132_e18013 * locals.var_pf33__blk369_dn0)) - (((locals.var_pf11__blk360_dn0 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn0)) * locals.var_pf32__blk368)) - (((locals.var_pf12__blk361_dn0 * locals.var_pf21__blk364) * locals.var_pf33__blk369) + (assign12880_body132_e18024 * locals.var_pf33__blk369_dn0))) + ((locals.var_pf13__blk362_dn0 * locals.var_pf21__blk364) * locals.var_pf32__blk368)), ((((((locals.var_pf11__blk360_dn2 * locals.var_pf22__blk365) * locals.var_pf33__blk369) + (assign12880_body132_e18013 * locals.var_pf33__blk369_dn2)) - (((locals.var_pf11__blk360_dn2 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn2)) * locals.var_pf32__blk368)) - (((locals.var_pf12__blk361_dn2 * locals.var_pf21__blk364) * locals.var_pf33__blk369) + (assign12880_body132_e18024 * locals.var_pf33__blk369_dn2))) + ((locals.var_pf13__blk362_dn2 * locals.var_pf21__blk364) * locals.var_pf32__blk368)), ((((((locals.var_pf11__blk360_dn6 * locals.var_pf22__blk365) * locals.var_pf33__blk369) + (assign12880_body132_e18013 * locals.var_pf33__blk369_dn6)) - (((locals.var_pf11__blk360_dn6 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn6)) * locals.var_pf32__blk368)) - (((locals.var_pf12__blk361_dn6 * locals.var_pf21__blk364) * locals.var_pf33__blk369) + (assign12880_body132_e18024 * locals.var_pf33__blk369_dn6))) + ((locals.var_pf13__blk362_dn6 * locals.var_pf21__blk364) * locals.var_pf32__blk368)), ((((((locals.var_pf11__blk360_dn7 * locals.var_pf22__blk365) * locals.var_pf33__blk369) + (assign12880_body132_e18013 * locals.var_pf33__blk369_dn7)) - (((locals.var_pf11__blk360_dn7 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn7)) * locals.var_pf32__blk368)) - (((locals.var_pf12__blk361_dn7 * locals.var_pf21__blk364) * locals.var_pf33__blk369) + (assign12880_body132_e18024 * locals.var_pf33__blk369_dn7))) + ((locals.var_pf13__blk362_dn7 * locals.var_pf21__blk364) * locals.var_pf32__blk368)), ((((((locals.var_pf11__blk360_dn10 * locals.var_pf22__blk365) * locals.var_pf33__blk369) + (assign12880_body132_e18013 * locals.var_pf33__blk369_dn10)) - (((locals.var_pf11__blk360_dn10 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn10)) * locals.var_pf32__blk368)) - (((locals.var_pf12__blk361_dn10 * locals.var_pf21__blk364) * locals.var_pf33__blk369) + (assign12880_body132_e18024 * locals.var_pf33__blk369_dn10))) + ((locals.var_pf13__blk362_dn10 * locals.var_pf21__blk364) * locals.var_pf32__blk368)), ((((((locals.var_pf11__blk360_dn11 * locals.var_pf22__blk365) * locals.var_pf33__blk369) + (assign12880_body132_e18013 * locals.var_pf33__blk369_dn11)) - (((locals.var_pf11__blk360_dn11 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn11)) * locals.var_pf32__blk368)) - (((locals.var_pf12__blk361_dn11 * locals.var_pf21__blk364) * locals.var_pf33__blk369) + (assign12880_body132_e18024 * locals.var_pf33__blk369_dn11))) + ((locals.var_pf13__blk362_dn11 * locals.var_pf21__blk364) * locals.var_pf32__blk368)), ((((((locals.var_pf11__blk360_dn12 * locals.var_pf22__blk365) * locals.var_pf33__blk369) + (assign12880_body132_e18013 * locals.var_pf33__blk369_dn12)) - (((locals.var_pf11__blk360_dn12 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn12)) * locals.var_pf32__blk368)) - (((locals.var_pf12__blk361_dn12 * locals.var_pf21__blk364) * locals.var_pf33__blk369) + (assign12880_body132_e18024 * locals.var_pf33__blk369_dn12))) + ((locals.var_pf13__blk362_dn12 * locals.var_pf21__blk364) * locals.var_pf32__blk368)), ((((((locals.var_pf11__blk360_dn17 * locals.var_pf22__blk365) * locals.var_pf33__blk369) + (assign12880_body132_e18013 * locals.var_pf33__blk369_dn17)) - (((locals.var_pf11__blk360_dn17 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn17)) * locals.var_pf32__blk368)) - (((locals.var_pf12__blk361_dn17 * locals.var_pf21__blk364) * locals.var_pf33__blk369) + (assign12880_body132_e18024 * locals.var_pf33__blk369_dn17))) + ((locals.var_pf13__blk362_dn17 * locals.var_pf21__blk364) * locals.var_pf32__blk368)),)
    } else {
        (locals.var_pdj__blk370, locals.var_pdj__blk370_dn0, locals.var_pdj__blk370_dn2, locals.var_pdj__blk370_dn6, locals.var_pdj__blk370_dn7, locals.var_pdj__blk370_dn10, locals.var_pdj__blk370_dn11, locals.var_pdj__blk370_dn12, locals.var_pdj__blk370_dn17,)
    }
};
            locals.var_pdj__blk370 = assign12880_body132_e18035;
            locals.var_pdj__blk370_dn0 = assign12880_body132_e18035_d_n0;
            locals.var_pdj__blk370_dn2 = assign12880_body132_e18035_d_n2;
            locals.var_pdj__blk370_dn6 = assign12880_body132_e18035_d_n6;
            locals.var_pdj__blk370_dn7 = assign12880_body132_e18035_d_n7;
            locals.var_pdj__blk370_dn10 = assign12880_body132_e18035_d_n10;
            locals.var_pdj__blk370_dn11 = assign12880_body132_e18035_d_n11;
            locals.var_pdj__blk370_dn12 = assign12880_body132_e18035_d_n12;
            locals.var_pdj__blk370_dn17 = assign12880_body132_e18035_d_n17;
            let (assign12880_body133_e18049, assign12880_body133_e18049_d_n0, assign12880_body133_e18049_d_n2, assign12880_body133_e18049_d_n6, assign12880_body133_e18049_d_n7, assign12880_body133_e18049_d_n10, assign12880_body133_e18049_d_n11, assign12880_body133_e18049_d_n12, assign12880_body133_e18049_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body133_e18046: f64 = (locals.var_pdj__blk370 + 1e-50);
        let assign12880_body133_e18047: f64 = (1.0 / assign12880_body133_e18046);
        (assign12880_body133_e18047, (-(locals.var_pdj__blk370_dn0 / (assign12880_body133_e18046 * assign12880_body133_e18046))), (-(locals.var_pdj__blk370_dn2 / (assign12880_body133_e18046 * assign12880_body133_e18046))), (-(locals.var_pdj__blk370_dn6 / (assign12880_body133_e18046 * assign12880_body133_e18046))), (-(locals.var_pdj__blk370_dn7 / (assign12880_body133_e18046 * assign12880_body133_e18046))), (-(locals.var_pdj__blk370_dn10 / (assign12880_body133_e18046 * assign12880_body133_e18046))), (-(locals.var_pdj__blk370_dn11 / (assign12880_body133_e18046 * assign12880_body133_e18046))), (-(locals.var_pdj__blk370_dn12 / (assign12880_body133_e18046 * assign12880_body133_e18046))), (-(locals.var_pdj__blk370_dn17 / (assign12880_body133_e18046 * assign12880_body133_e18046))),)
    } else {
        (locals.var_pdji__blk371, locals.var_pdji__blk371_dn0, locals.var_pdji__blk371_dn2, locals.var_pdji__blk371_dn6, locals.var_pdji__blk371_dn7, locals.var_pdji__blk371_dn10, locals.var_pdji__blk371_dn11, locals.var_pdji__blk371_dn12, locals.var_pdji__blk371_dn17,)
    }
};
            locals.var_pdji__blk371 = assign12880_body133_e18049;
            locals.var_pdji__blk371_dn0 = assign12880_body133_e18049_d_n0;
            locals.var_pdji__blk371_dn2 = assign12880_body133_e18049_d_n2;
            locals.var_pdji__blk371_dn6 = assign12880_body133_e18049_d_n6;
            locals.var_pdji__blk371_dn7 = assign12880_body133_e18049_d_n7;
            locals.var_pdji__blk371_dn10 = assign12880_body133_e18049_d_n10;
            locals.var_pdji__blk371_dn11 = assign12880_body133_e18049_d_n11;
            locals.var_pdji__blk371_dn12 = assign12880_body133_e18049_d_n12;
            locals.var_pdji__blk371_dn17 = assign12880_body133_e18049_d_n17;
            let (assign12880_body134_e18065, assign12880_body134_e18065_d_n0, assign12880_body134_e18065_d_n2, assign12880_body134_e18065_d_n6, assign12880_body134_e18065_d_n7, assign12880_body134_e18065_d_n10, assign12880_body134_e18065_d_n11, assign12880_body134_e18065_d_n12, assign12880_body134_e18065_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body134_e18059: f64 = (locals.var_pf22__blk365 * locals.var_pf33__blk369);
        let assign12880_body134_e18062: f64 = (locals.var_pf23__blk366 * locals.var_pf32__blk368);
        let assign12880_body134_e18063: f64 = (assign12880_body134_e18059 - assign12880_body134_e18062);
        (assign12880_body134_e18063, ((locals.var_pf22__blk365 * locals.var_pf33__blk369_dn0) - (locals.var_pf23__blk366_dn0 * locals.var_pf32__blk368)), ((locals.var_pf22__blk365 * locals.var_pf33__blk369_dn2) - (locals.var_pf23__blk366_dn2 * locals.var_pf32__blk368)), ((locals.var_pf22__blk365 * locals.var_pf33__blk369_dn6) - (locals.var_pf23__blk366_dn6 * locals.var_pf32__blk368)), ((locals.var_pf22__blk365 * locals.var_pf33__blk369_dn7) - (locals.var_pf23__blk366_dn7 * locals.var_pf32__blk368)), ((locals.var_pf22__blk365 * locals.var_pf33__blk369_dn10) - (locals.var_pf23__blk366_dn10 * locals.var_pf32__blk368)), ((locals.var_pf22__blk365 * locals.var_pf33__blk369_dn11) - (locals.var_pf23__blk366_dn11 * locals.var_pf32__blk368)), ((locals.var_pf22__blk365 * locals.var_pf33__blk369_dn12) - (locals.var_pf23__blk366_dn12 * locals.var_pf32__blk368)), ((locals.var_pf22__blk365 * locals.var_pf33__blk369_dn17) - (locals.var_pf23__blk366_dn17 * locals.var_pf32__blk368)),)
    } else {
        (locals.var_pji11__blk372, locals.var_pji11__blk372_dn0, locals.var_pji11__blk372_dn2, locals.var_pji11__blk372_dn6, locals.var_pji11__blk372_dn7, locals.var_pji11__blk372_dn10, locals.var_pji11__blk372_dn11, locals.var_pji11__blk372_dn12, locals.var_pji11__blk372_dn17,)
    }
};
            locals.var_pji11__blk372 = assign12880_body134_e18065;
            locals.var_pji11__blk372_dn0 = assign12880_body134_e18065_d_n0;
            locals.var_pji11__blk372_dn2 = assign12880_body134_e18065_d_n2;
            locals.var_pji11__blk372_dn6 = assign12880_body134_e18065_d_n6;
            locals.var_pji11__blk372_dn7 = assign12880_body134_e18065_d_n7;
            locals.var_pji11__blk372_dn10 = assign12880_body134_e18065_d_n10;
            locals.var_pji11__blk372_dn11 = assign12880_body134_e18065_d_n11;
            locals.var_pji11__blk372_dn12 = assign12880_body134_e18065_d_n12;
            locals.var_pji11__blk372_dn17 = assign12880_body134_e18065_d_n17;
            let (assign12880_body135_e18081, assign12880_body135_e18081_d_n0, assign12880_body135_e18081_d_n2, assign12880_body135_e18081_d_n6, assign12880_body135_e18081_d_n7, assign12880_body135_e18081_d_n10, assign12880_body135_e18081_d_n11, assign12880_body135_e18081_d_n12, assign12880_body135_e18081_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body135_e18075: f64 = (locals.var_pf13__blk362 * locals.var_pf32__blk368);
        let assign12880_body135_e18078: f64 = (locals.var_pf12__blk361 * locals.var_pf33__blk369);
        let assign12880_body135_e18079: f64 = (assign12880_body135_e18075 - assign12880_body135_e18078);
        (assign12880_body135_e18079, ((locals.var_pf13__blk362_dn0 * locals.var_pf32__blk368) - ((locals.var_pf12__blk361_dn0 * locals.var_pf33__blk369) + (locals.var_pf12__blk361 * locals.var_pf33__blk369_dn0))), ((locals.var_pf13__blk362_dn2 * locals.var_pf32__blk368) - ((locals.var_pf12__blk361_dn2 * locals.var_pf33__blk369) + (locals.var_pf12__blk361 * locals.var_pf33__blk369_dn2))), ((locals.var_pf13__blk362_dn6 * locals.var_pf32__blk368) - ((locals.var_pf12__blk361_dn6 * locals.var_pf33__blk369) + (locals.var_pf12__blk361 * locals.var_pf33__blk369_dn6))), ((locals.var_pf13__blk362_dn7 * locals.var_pf32__blk368) - ((locals.var_pf12__blk361_dn7 * locals.var_pf33__blk369) + (locals.var_pf12__blk361 * locals.var_pf33__blk369_dn7))), ((locals.var_pf13__blk362_dn10 * locals.var_pf32__blk368) - ((locals.var_pf12__blk361_dn10 * locals.var_pf33__blk369) + (locals.var_pf12__blk361 * locals.var_pf33__blk369_dn10))), ((locals.var_pf13__blk362_dn11 * locals.var_pf32__blk368) - ((locals.var_pf12__blk361_dn11 * locals.var_pf33__blk369) + (locals.var_pf12__blk361 * locals.var_pf33__blk369_dn11))), ((locals.var_pf13__blk362_dn12 * locals.var_pf32__blk368) - ((locals.var_pf12__blk361_dn12 * locals.var_pf33__blk369) + (locals.var_pf12__blk361 * locals.var_pf33__blk369_dn12))), ((locals.var_pf13__blk362_dn17 * locals.var_pf32__blk368) - ((locals.var_pf12__blk361_dn17 * locals.var_pf33__blk369) + (locals.var_pf12__blk361 * locals.var_pf33__blk369_dn17))),)
    } else {
        (locals.var_pji12__blk373, locals.var_pji12__blk373_dn0, locals.var_pji12__blk373_dn2, locals.var_pji12__blk373_dn6, locals.var_pji12__blk373_dn7, locals.var_pji12__blk373_dn10, locals.var_pji12__blk373_dn11, locals.var_pji12__blk373_dn12, locals.var_pji12__blk373_dn17,)
    }
};
            locals.var_pji12__blk373 = assign12880_body135_e18081;
            locals.var_pji12__blk373_dn0 = assign12880_body135_e18081_d_n0;
            locals.var_pji12__blk373_dn2 = assign12880_body135_e18081_d_n2;
            locals.var_pji12__blk373_dn6 = assign12880_body135_e18081_d_n6;
            locals.var_pji12__blk373_dn7 = assign12880_body135_e18081_d_n7;
            locals.var_pji12__blk373_dn10 = assign12880_body135_e18081_d_n10;
            locals.var_pji12__blk373_dn11 = assign12880_body135_e18081_d_n11;
            locals.var_pji12__blk373_dn12 = assign12880_body135_e18081_d_n12;
            locals.var_pji12__blk373_dn17 = assign12880_body135_e18081_d_n17;
            let (assign12880_body136_e18097, assign12880_body136_e18097_d_n0, assign12880_body136_e18097_d_n2, assign12880_body136_e18097_d_n6, assign12880_body136_e18097_d_n7, assign12880_body136_e18097_d_n10, assign12880_body136_e18097_d_n11, assign12880_body136_e18097_d_n12, assign12880_body136_e18097_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body136_e18091: f64 = (locals.var_pf12__blk361 * locals.var_pf23__blk366);
        let assign12880_body136_e18094: f64 = (locals.var_pf13__blk362 * locals.var_pf22__blk365);
        let assign12880_body136_e18095: f64 = (assign12880_body136_e18091 - assign12880_body136_e18094);
        (assign12880_body136_e18095, (((locals.var_pf12__blk361_dn0 * locals.var_pf23__blk366) + (locals.var_pf12__blk361 * locals.var_pf23__blk366_dn0)) - (locals.var_pf13__blk362_dn0 * locals.var_pf22__blk365)), (((locals.var_pf12__blk361_dn2 * locals.var_pf23__blk366) + (locals.var_pf12__blk361 * locals.var_pf23__blk366_dn2)) - (locals.var_pf13__blk362_dn2 * locals.var_pf22__blk365)), (((locals.var_pf12__blk361_dn6 * locals.var_pf23__blk366) + (locals.var_pf12__blk361 * locals.var_pf23__blk366_dn6)) - (locals.var_pf13__blk362_dn6 * locals.var_pf22__blk365)), (((locals.var_pf12__blk361_dn7 * locals.var_pf23__blk366) + (locals.var_pf12__blk361 * locals.var_pf23__blk366_dn7)) - (locals.var_pf13__blk362_dn7 * locals.var_pf22__blk365)), (((locals.var_pf12__blk361_dn10 * locals.var_pf23__blk366) + (locals.var_pf12__blk361 * locals.var_pf23__blk366_dn10)) - (locals.var_pf13__blk362_dn10 * locals.var_pf22__blk365)), (((locals.var_pf12__blk361_dn11 * locals.var_pf23__blk366) + (locals.var_pf12__blk361 * locals.var_pf23__blk366_dn11)) - (locals.var_pf13__blk362_dn11 * locals.var_pf22__blk365)), (((locals.var_pf12__blk361_dn12 * locals.var_pf23__blk366) + (locals.var_pf12__blk361 * locals.var_pf23__blk366_dn12)) - (locals.var_pf13__blk362_dn12 * locals.var_pf22__blk365)), (((locals.var_pf12__blk361_dn17 * locals.var_pf23__blk366) + (locals.var_pf12__blk361 * locals.var_pf23__blk366_dn17)) - (locals.var_pf13__blk362_dn17 * locals.var_pf22__blk365)),)
    } else {
        (locals.var_pji13__blk374, locals.var_pji13__blk374_dn0, locals.var_pji13__blk374_dn2, locals.var_pji13__blk374_dn6, locals.var_pji13__blk374_dn7, locals.var_pji13__blk374_dn10, locals.var_pji13__blk374_dn11, locals.var_pji13__blk374_dn12, locals.var_pji13__blk374_dn17,)
    }
};
            locals.var_pji13__blk374 = assign12880_body136_e18097;
            locals.var_pji13__blk374_dn0 = assign12880_body136_e18097_d_n0;
            locals.var_pji13__blk374_dn2 = assign12880_body136_e18097_d_n2;
            locals.var_pji13__blk374_dn6 = assign12880_body136_e18097_d_n6;
            locals.var_pji13__blk374_dn7 = assign12880_body136_e18097_d_n7;
            locals.var_pji13__blk374_dn10 = assign12880_body136_e18097_d_n10;
            locals.var_pji13__blk374_dn11 = assign12880_body136_e18097_d_n11;
            locals.var_pji13__blk374_dn12 = assign12880_body136_e18097_d_n12;
            locals.var_pji13__blk374_dn17 = assign12880_body136_e18097_d_n17;
            let (assign12880_body137_e18110, assign12880_body137_e18110_d_n0, assign12880_body137_e18110_d_n2, assign12880_body137_e18110_d_n6, assign12880_body137_e18110_d_n7, assign12880_body137_e18110_d_n10, assign12880_body137_e18110_d_n11, assign12880_body137_e18110_d_n12, assign12880_body137_e18110_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body137_e18106: f64 = (-locals.var_pf21__blk364);
        let assign12880_body137_e18108: f64 = (assign12880_body137_e18106 * locals.var_pf33__blk369);
        (assign12880_body137_e18108, (assign12880_body137_e18106 * locals.var_pf33__blk369_dn0), (assign12880_body137_e18106 * locals.var_pf33__blk369_dn2), (assign12880_body137_e18106 * locals.var_pf33__blk369_dn6), (assign12880_body137_e18106 * locals.var_pf33__blk369_dn7), (assign12880_body137_e18106 * locals.var_pf33__blk369_dn10), (assign12880_body137_e18106 * locals.var_pf33__blk369_dn11), (assign12880_body137_e18106 * locals.var_pf33__blk369_dn12), (assign12880_body137_e18106 * locals.var_pf33__blk369_dn17),)
    } else {
        (locals.var_pji21__blk375, locals.var_pji21__blk375_dn0, locals.var_pji21__blk375_dn2, locals.var_pji21__blk375_dn6, locals.var_pji21__blk375_dn7, locals.var_pji21__blk375_dn10, locals.var_pji21__blk375_dn11, locals.var_pji21__blk375_dn12, locals.var_pji21__blk375_dn17,)
    }
};
            locals.var_pji21__blk375 = assign12880_body137_e18110;
            locals.var_pji21__blk375_dn0 = assign12880_body137_e18110_d_n0;
            locals.var_pji21__blk375_dn2 = assign12880_body137_e18110_d_n2;
            locals.var_pji21__blk375_dn6 = assign12880_body137_e18110_d_n6;
            locals.var_pji21__blk375_dn7 = assign12880_body137_e18110_d_n7;
            locals.var_pji21__blk375_dn10 = assign12880_body137_e18110_d_n10;
            locals.var_pji21__blk375_dn11 = assign12880_body137_e18110_d_n11;
            locals.var_pji21__blk375_dn12 = assign12880_body137_e18110_d_n12;
            locals.var_pji21__blk375_dn17 = assign12880_body137_e18110_d_n17;
            let (assign12880_body138_e18122, assign12880_body138_e18122_d_n0, assign12880_body138_e18122_d_n2, assign12880_body138_e18122_d_n6, assign12880_body138_e18122_d_n7, assign12880_body138_e18122_d_n10, assign12880_body138_e18122_d_n11, assign12880_body138_e18122_d_n12, assign12880_body138_e18122_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body138_e18120: f64 = (locals.var_pf11__blk360 * locals.var_pf33__blk369);
        (assign12880_body138_e18120, ((locals.var_pf11__blk360_dn0 * locals.var_pf33__blk369) + (locals.var_pf11__blk360 * locals.var_pf33__blk369_dn0)), ((locals.var_pf11__blk360_dn2 * locals.var_pf33__blk369) + (locals.var_pf11__blk360 * locals.var_pf33__blk369_dn2)), ((locals.var_pf11__blk360_dn6 * locals.var_pf33__blk369) + (locals.var_pf11__blk360 * locals.var_pf33__blk369_dn6)), ((locals.var_pf11__blk360_dn7 * locals.var_pf33__blk369) + (locals.var_pf11__blk360 * locals.var_pf33__blk369_dn7)), ((locals.var_pf11__blk360_dn10 * locals.var_pf33__blk369) + (locals.var_pf11__blk360 * locals.var_pf33__blk369_dn10)), ((locals.var_pf11__blk360_dn11 * locals.var_pf33__blk369) + (locals.var_pf11__blk360 * locals.var_pf33__blk369_dn11)), ((locals.var_pf11__blk360_dn12 * locals.var_pf33__blk369) + (locals.var_pf11__blk360 * locals.var_pf33__blk369_dn12)), ((locals.var_pf11__blk360_dn17 * locals.var_pf33__blk369) + (locals.var_pf11__blk360 * locals.var_pf33__blk369_dn17)),)
    } else {
        (locals.var_pji22__blk376, locals.var_pji22__blk376_dn0, locals.var_pji22__blk376_dn2, locals.var_pji22__blk376_dn6, locals.var_pji22__blk376_dn7, locals.var_pji22__blk376_dn10, locals.var_pji22__blk376_dn11, locals.var_pji22__blk376_dn12, locals.var_pji22__blk376_dn17,)
    }
};
            locals.var_pji22__blk376 = assign12880_body138_e18122;
            locals.var_pji22__blk376_dn0 = assign12880_body138_e18122_d_n0;
            locals.var_pji22__blk376_dn2 = assign12880_body138_e18122_d_n2;
            locals.var_pji22__blk376_dn6 = assign12880_body138_e18122_d_n6;
            locals.var_pji22__blk376_dn7 = assign12880_body138_e18122_d_n7;
            locals.var_pji22__blk376_dn10 = assign12880_body138_e18122_d_n10;
            locals.var_pji22__blk376_dn11 = assign12880_body138_e18122_d_n11;
            locals.var_pji22__blk376_dn12 = assign12880_body138_e18122_d_n12;
            locals.var_pji22__blk376_dn17 = assign12880_body138_e18122_d_n17;
            let (assign12880_body139_e18138, assign12880_body139_e18138_d_n0, assign12880_body139_e18138_d_n2, assign12880_body139_e18138_d_n6, assign12880_body139_e18138_d_n7, assign12880_body139_e18138_d_n10, assign12880_body139_e18138_d_n11, assign12880_body139_e18138_d_n12, assign12880_body139_e18138_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body139_e18132: f64 = (locals.var_pf13__blk362 * locals.var_pf21__blk364);
        let assign12880_body139_e18135: f64 = (locals.var_pf11__blk360 * locals.var_pf23__blk366);
        let assign12880_body139_e18136: f64 = (assign12880_body139_e18132 - assign12880_body139_e18135);
        (assign12880_body139_e18136, ((locals.var_pf13__blk362_dn0 * locals.var_pf21__blk364) - ((locals.var_pf11__blk360_dn0 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn0))), ((locals.var_pf13__blk362_dn2 * locals.var_pf21__blk364) - ((locals.var_pf11__blk360_dn2 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn2))), ((locals.var_pf13__blk362_dn6 * locals.var_pf21__blk364) - ((locals.var_pf11__blk360_dn6 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn6))), ((locals.var_pf13__blk362_dn7 * locals.var_pf21__blk364) - ((locals.var_pf11__blk360_dn7 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn7))), ((locals.var_pf13__blk362_dn10 * locals.var_pf21__blk364) - ((locals.var_pf11__blk360_dn10 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn10))), ((locals.var_pf13__blk362_dn11 * locals.var_pf21__blk364) - ((locals.var_pf11__blk360_dn11 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn11))), ((locals.var_pf13__blk362_dn12 * locals.var_pf21__blk364) - ((locals.var_pf11__blk360_dn12 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn12))), ((locals.var_pf13__blk362_dn17 * locals.var_pf21__blk364) - ((locals.var_pf11__blk360_dn17 * locals.var_pf23__blk366) + (locals.var_pf11__blk360 * locals.var_pf23__blk366_dn17))),)
    } else {
        (locals.var_pji23__blk377, locals.var_pji23__blk377_dn0, locals.var_pji23__blk377_dn2, locals.var_pji23__blk377_dn6, locals.var_pji23__blk377_dn7, locals.var_pji23__blk377_dn10, locals.var_pji23__blk377_dn11, locals.var_pji23__blk377_dn12, locals.var_pji23__blk377_dn17,)
    }
};
            locals.var_pji23__blk377 = assign12880_body139_e18138;
            locals.var_pji23__blk377_dn0 = assign12880_body139_e18138_d_n0;
            locals.var_pji23__blk377_dn2 = assign12880_body139_e18138_d_n2;
            locals.var_pji23__blk377_dn6 = assign12880_body139_e18138_d_n6;
            locals.var_pji23__blk377_dn7 = assign12880_body139_e18138_d_n7;
            locals.var_pji23__blk377_dn10 = assign12880_body139_e18138_d_n10;
            locals.var_pji23__blk377_dn11 = assign12880_body139_e18138_d_n11;
            locals.var_pji23__blk377_dn12 = assign12880_body139_e18138_d_n12;
            locals.var_pji23__blk377_dn17 = assign12880_body139_e18138_d_n17;
            let (assign12880_body140_e18150,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body140_e18148: f64 = (locals.var_pf21__blk364 * locals.var_pf32__blk368);
        (assign12880_body140_e18148,)
    } else {
        (locals.var_pji31__blk378,)
    }
};
            locals.var_pji31__blk378 = assign12880_body140_e18150;
            let (assign12880_body141_e18163, assign12880_body141_e18163_d_n0, assign12880_body141_e18163_d_n2, assign12880_body141_e18163_d_n6, assign12880_body141_e18163_d_n7, assign12880_body141_e18163_d_n10, assign12880_body141_e18163_d_n11, assign12880_body141_e18163_d_n12, assign12880_body141_e18163_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body141_e18159: f64 = (-locals.var_pf11__blk360);
        let assign12880_body141_e18161: f64 = (assign12880_body141_e18159 * locals.var_pf32__blk368);
        (assign12880_body141_e18161, ((-locals.var_pf11__blk360_dn0) * locals.var_pf32__blk368), ((-locals.var_pf11__blk360_dn2) * locals.var_pf32__blk368), ((-locals.var_pf11__blk360_dn6) * locals.var_pf32__blk368), ((-locals.var_pf11__blk360_dn7) * locals.var_pf32__blk368), ((-locals.var_pf11__blk360_dn10) * locals.var_pf32__blk368), ((-locals.var_pf11__blk360_dn11) * locals.var_pf32__blk368), ((-locals.var_pf11__blk360_dn12) * locals.var_pf32__blk368), ((-locals.var_pf11__blk360_dn17) * locals.var_pf32__blk368),)
    } else {
        (locals.var_pji32__blk379, locals.var_pji32__blk379_dn0, locals.var_pji32__blk379_dn2, locals.var_pji32__blk379_dn6, locals.var_pji32__blk379_dn7, locals.var_pji32__blk379_dn10, locals.var_pji32__blk379_dn11, locals.var_pji32__blk379_dn12, locals.var_pji32__blk379_dn17,)
    }
};
            locals.var_pji32__blk379 = assign12880_body141_e18163;
            locals.var_pji32__blk379_dn0 = assign12880_body141_e18163_d_n0;
            locals.var_pji32__blk379_dn2 = assign12880_body141_e18163_d_n2;
            locals.var_pji32__blk379_dn6 = assign12880_body141_e18163_d_n6;
            locals.var_pji32__blk379_dn7 = assign12880_body141_e18163_d_n7;
            locals.var_pji32__blk379_dn10 = assign12880_body141_e18163_d_n10;
            locals.var_pji32__blk379_dn11 = assign12880_body141_e18163_d_n11;
            locals.var_pji32__blk379_dn12 = assign12880_body141_e18163_d_n12;
            locals.var_pji32__blk379_dn17 = assign12880_body141_e18163_d_n17;
            let (assign12880_body142_e18179, assign12880_body142_e18179_d_n0, assign12880_body142_e18179_d_n2, assign12880_body142_e18179_d_n6, assign12880_body142_e18179_d_n7, assign12880_body142_e18179_d_n10, assign12880_body142_e18179_d_n11, assign12880_body142_e18179_d_n12, assign12880_body142_e18179_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body142_e18173: f64 = (locals.var_pf11__blk360 * locals.var_pf22__blk365);
        let assign12880_body142_e18176: f64 = (locals.var_pf12__blk361 * locals.var_pf21__blk364);
        let assign12880_body142_e18177: f64 = (assign12880_body142_e18173 - assign12880_body142_e18176);
        (assign12880_body142_e18177, ((locals.var_pf11__blk360_dn0 * locals.var_pf22__blk365) - (locals.var_pf12__blk361_dn0 * locals.var_pf21__blk364)), ((locals.var_pf11__blk360_dn2 * locals.var_pf22__blk365) - (locals.var_pf12__blk361_dn2 * locals.var_pf21__blk364)), ((locals.var_pf11__blk360_dn6 * locals.var_pf22__blk365) - (locals.var_pf12__blk361_dn6 * locals.var_pf21__blk364)), ((locals.var_pf11__blk360_dn7 * locals.var_pf22__blk365) - (locals.var_pf12__blk361_dn7 * locals.var_pf21__blk364)), ((locals.var_pf11__blk360_dn10 * locals.var_pf22__blk365) - (locals.var_pf12__blk361_dn10 * locals.var_pf21__blk364)), ((locals.var_pf11__blk360_dn11 * locals.var_pf22__blk365) - (locals.var_pf12__blk361_dn11 * locals.var_pf21__blk364)), ((locals.var_pf11__blk360_dn12 * locals.var_pf22__blk365) - (locals.var_pf12__blk361_dn12 * locals.var_pf21__blk364)), ((locals.var_pf11__blk360_dn17 * locals.var_pf22__blk365) - (locals.var_pf12__blk361_dn17 * locals.var_pf21__blk364)),)
    } else {
        (locals.var_pji33__blk380, locals.var_pji33__blk380_dn0, locals.var_pji33__blk380_dn2, locals.var_pji33__blk380_dn6, locals.var_pji33__blk380_dn7, locals.var_pji33__blk380_dn10, locals.var_pji33__blk380_dn11, locals.var_pji33__blk380_dn12, locals.var_pji33__blk380_dn17,)
    }
};
            locals.var_pji33__blk380 = assign12880_body142_e18179;
            locals.var_pji33__blk380_dn0 = assign12880_body142_e18179_d_n0;
            locals.var_pji33__blk380_dn2 = assign12880_body142_e18179_d_n2;
            locals.var_pji33__blk380_dn6 = assign12880_body142_e18179_d_n6;
            locals.var_pji33__blk380_dn7 = assign12880_body142_e18179_d_n7;
            locals.var_pji33__blk380_dn10 = assign12880_body142_e18179_d_n10;
            locals.var_pji33__blk380_dn11 = assign12880_body142_e18179_d_n11;
            locals.var_pji33__blk380_dn12 = assign12880_body142_e18179_d_n12;
            locals.var_pji33__blk380_dn17 = assign12880_body142_e18179_d_n17;
            let (assign12880_body143_e18202, assign12880_body143_e18202_d_n0, assign12880_body143_e18202_d_n2, assign12880_body143_e18202_d_n6, assign12880_body143_e18202_d_n7, assign12880_body143_e18202_d_n10, assign12880_body143_e18202_d_n11, assign12880_body143_e18202_d_n12, assign12880_body143_e18202_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body143_e18188: f64 = (-locals.var_pdji__blk371);
        let assign12880_body143_e18191: f64 = (locals.var_pji11__blk372 * locals.var_pf1__blk359);
        let assign12880_body143_e18194: f64 = (locals.var_pji12__blk373 * locals.var_pf2__blk363);
        let assign12880_body143_e18195: f64 = (assign12880_body143_e18191 + assign12880_body143_e18194);
        let assign12880_body143_e18198: f64 = (locals.var_pji13__blk374 * locals.var_pf3__blk367);
        let assign12880_body143_e18199: f64 = (assign12880_body143_e18195 + assign12880_body143_e18198);
        let assign12880_body143_e18200: f64 = (assign12880_body143_e18188 * assign12880_body143_e18199);
        (assign12880_body143_e18200, (((-locals.var_pdji__blk371_dn0) * assign12880_body143_e18199) + (assign12880_body143_e18188 * ((((locals.var_pji11__blk372_dn0 * locals.var_pf1__blk359) + (locals.var_pji11__blk372 * locals.var_pf1__blk359_dn0)) + ((locals.var_pji12__blk373_dn0 * locals.var_pf2__blk363) + (locals.var_pji12__blk373 * locals.var_pf2__blk363_dn0))) + ((locals.var_pji13__blk374_dn0 * locals.var_pf3__blk367) + (locals.var_pji13__blk374 * locals.var_pf3__blk367_dn0))))), (((-locals.var_pdji__blk371_dn2) * assign12880_body143_e18199) + (assign12880_body143_e18188 * ((((locals.var_pji11__blk372_dn2 * locals.var_pf1__blk359) + (locals.var_pji11__blk372 * locals.var_pf1__blk359_dn2)) + ((locals.var_pji12__blk373_dn2 * locals.var_pf2__blk363) + (locals.var_pji12__blk373 * locals.var_pf2__blk363_dn2))) + ((locals.var_pji13__blk374_dn2 * locals.var_pf3__blk367) + (locals.var_pji13__blk374 * locals.var_pf3__blk367_dn2))))), (((-locals.var_pdji__blk371_dn6) * assign12880_body143_e18199) + (assign12880_body143_e18188 * ((((locals.var_pji11__blk372_dn6 * locals.var_pf1__blk359) + (locals.var_pji11__blk372 * locals.var_pf1__blk359_dn6)) + ((locals.var_pji12__blk373_dn6 * locals.var_pf2__blk363) + (locals.var_pji12__blk373 * locals.var_pf2__blk363_dn6))) + ((locals.var_pji13__blk374_dn6 * locals.var_pf3__blk367) + (locals.var_pji13__blk374 * locals.var_pf3__blk367_dn6))))), (((-locals.var_pdji__blk371_dn7) * assign12880_body143_e18199) + (assign12880_body143_e18188 * ((((locals.var_pji11__blk372_dn7 * locals.var_pf1__blk359) + (locals.var_pji11__blk372 * locals.var_pf1__blk359_dn7)) + ((locals.var_pji12__blk373_dn7 * locals.var_pf2__blk363) + (locals.var_pji12__blk373 * locals.var_pf2__blk363_dn7))) + ((locals.var_pji13__blk374_dn7 * locals.var_pf3__blk367) + (locals.var_pji13__blk374 * locals.var_pf3__blk367_dn7))))), (((-locals.var_pdji__blk371_dn10) * assign12880_body143_e18199) + (assign12880_body143_e18188 * ((((locals.var_pji11__blk372_dn10 * locals.var_pf1__blk359) + (locals.var_pji11__blk372 * locals.var_pf1__blk359_dn10)) + ((locals.var_pji12__blk373_dn10 * locals.var_pf2__blk363) + (locals.var_pji12__blk373 * locals.var_pf2__blk363_dn10))) + ((locals.var_pji13__blk374_dn10 * locals.var_pf3__blk367) + (locals.var_pji13__blk374 * locals.var_pf3__blk367_dn10))))), (((-locals.var_pdji__blk371_dn11) * assign12880_body143_e18199) + (assign12880_body143_e18188 * ((((locals.var_pji11__blk372_dn11 * locals.var_pf1__blk359) + (locals.var_pji11__blk372 * locals.var_pf1__blk359_dn11)) + ((locals.var_pji12__blk373_dn11 * locals.var_pf2__blk363) + (locals.var_pji12__blk373 * locals.var_pf2__blk363_dn11))) + ((locals.var_pji13__blk374_dn11 * locals.var_pf3__blk367) + (locals.var_pji13__blk374 * locals.var_pf3__blk367_dn11))))), (((-locals.var_pdji__blk371_dn12) * assign12880_body143_e18199) + (assign12880_body143_e18188 * ((((locals.var_pji11__blk372_dn12 * locals.var_pf1__blk359) + (locals.var_pji11__blk372 * locals.var_pf1__blk359_dn12)) + ((locals.var_pji12__blk373_dn12 * locals.var_pf2__blk363) + (locals.var_pji12__blk373 * locals.var_pf2__blk363_dn12))) + ((locals.var_pji13__blk374_dn12 * locals.var_pf3__blk367) + (locals.var_pji13__blk374 * locals.var_pf3__blk367_dn12))))), (((-locals.var_pdji__blk371_dn17) * assign12880_body143_e18199) + (assign12880_body143_e18188 * ((((locals.var_pji11__blk372_dn17 * locals.var_pf1__blk359) + (locals.var_pji11__blk372 * locals.var_pf1__blk359_dn17)) + ((locals.var_pji12__blk373_dn17 * locals.var_pf2__blk363) + (locals.var_pji12__blk373 * locals.var_pf2__blk363_dn17))) + ((locals.var_pji13__blk374_dn17 * locals.var_pf3__blk367) + (locals.var_pji13__blk374 * locals.var_pf3__blk367_dn17))))),)
    } else {
        (locals.var_dpss__blk356, locals.var_dpss__blk356_dn0, locals.var_dpss__blk356_dn2, locals.var_dpss__blk356_dn6, locals.var_dpss__blk356_dn7, locals.var_dpss__blk356_dn10, locals.var_dpss__blk356_dn11, locals.var_dpss__blk356_dn12, locals.var_dpss__blk356_dn17,)
    }
};
            locals.var_dpss__blk356 = assign12880_body143_e18202;
            locals.var_dpss__blk356_dn0 = assign12880_body143_e18202_d_n0;
            locals.var_dpss__blk356_dn2 = assign12880_body143_e18202_d_n2;
            locals.var_dpss__blk356_dn6 = assign12880_body143_e18202_d_n6;
            locals.var_dpss__blk356_dn7 = assign12880_body143_e18202_d_n7;
            locals.var_dpss__blk356_dn10 = assign12880_body143_e18202_d_n10;
            locals.var_dpss__blk356_dn11 = assign12880_body143_e18202_d_n11;
            locals.var_dpss__blk356_dn12 = assign12880_body143_e18202_d_n12;
            locals.var_dpss__blk356_dn17 = assign12880_body143_e18202_d_n17;
            let (assign12880_body144_e18225, assign12880_body144_e18225_d_n0, assign12880_body144_e18225_d_n2, assign12880_body144_e18225_d_n6, assign12880_body144_e18225_d_n7, assign12880_body144_e18225_d_n10, assign12880_body144_e18225_d_n11, assign12880_body144_e18225_d_n12, assign12880_body144_e18225_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body144_e18211: f64 = (-locals.var_pdji__blk371);
        let assign12880_body144_e18214: f64 = (locals.var_pji21__blk375 * locals.var_pf1__blk359);
        let assign12880_body144_e18217: f64 = (locals.var_pji22__blk376 * locals.var_pf2__blk363);
        let assign12880_body144_e18218: f64 = (assign12880_body144_e18214 + assign12880_body144_e18217);
        let assign12880_body144_e18221: f64 = (locals.var_pji23__blk377 * locals.var_pf3__blk367);
        let assign12880_body144_e18222: f64 = (assign12880_body144_e18218 + assign12880_body144_e18221);
        let assign12880_body144_e18223: f64 = (assign12880_body144_e18211 * assign12880_body144_e18222);
        (assign12880_body144_e18223, (((-locals.var_pdji__blk371_dn0) * assign12880_body144_e18222) + (assign12880_body144_e18211 * ((((locals.var_pji21__blk375_dn0 * locals.var_pf1__blk359) + (locals.var_pji21__blk375 * locals.var_pf1__blk359_dn0)) + ((locals.var_pji22__blk376_dn0 * locals.var_pf2__blk363) + (locals.var_pji22__blk376 * locals.var_pf2__blk363_dn0))) + ((locals.var_pji23__blk377_dn0 * locals.var_pf3__blk367) + (locals.var_pji23__blk377 * locals.var_pf3__blk367_dn0))))), (((-locals.var_pdji__blk371_dn2) * assign12880_body144_e18222) + (assign12880_body144_e18211 * ((((locals.var_pji21__blk375_dn2 * locals.var_pf1__blk359) + (locals.var_pji21__blk375 * locals.var_pf1__blk359_dn2)) + ((locals.var_pji22__blk376_dn2 * locals.var_pf2__blk363) + (locals.var_pji22__blk376 * locals.var_pf2__blk363_dn2))) + ((locals.var_pji23__blk377_dn2 * locals.var_pf3__blk367) + (locals.var_pji23__blk377 * locals.var_pf3__blk367_dn2))))), (((-locals.var_pdji__blk371_dn6) * assign12880_body144_e18222) + (assign12880_body144_e18211 * ((((locals.var_pji21__blk375_dn6 * locals.var_pf1__blk359) + (locals.var_pji21__blk375 * locals.var_pf1__blk359_dn6)) + ((locals.var_pji22__blk376_dn6 * locals.var_pf2__blk363) + (locals.var_pji22__blk376 * locals.var_pf2__blk363_dn6))) + ((locals.var_pji23__blk377_dn6 * locals.var_pf3__blk367) + (locals.var_pji23__blk377 * locals.var_pf3__blk367_dn6))))), (((-locals.var_pdji__blk371_dn7) * assign12880_body144_e18222) + (assign12880_body144_e18211 * ((((locals.var_pji21__blk375_dn7 * locals.var_pf1__blk359) + (locals.var_pji21__blk375 * locals.var_pf1__blk359_dn7)) + ((locals.var_pji22__blk376_dn7 * locals.var_pf2__blk363) + (locals.var_pji22__blk376 * locals.var_pf2__blk363_dn7))) + ((locals.var_pji23__blk377_dn7 * locals.var_pf3__blk367) + (locals.var_pji23__blk377 * locals.var_pf3__blk367_dn7))))), (((-locals.var_pdji__blk371_dn10) * assign12880_body144_e18222) + (assign12880_body144_e18211 * ((((locals.var_pji21__blk375_dn10 * locals.var_pf1__blk359) + (locals.var_pji21__blk375 * locals.var_pf1__blk359_dn10)) + ((locals.var_pji22__blk376_dn10 * locals.var_pf2__blk363) + (locals.var_pji22__blk376 * locals.var_pf2__blk363_dn10))) + ((locals.var_pji23__blk377_dn10 * locals.var_pf3__blk367) + (locals.var_pji23__blk377 * locals.var_pf3__blk367_dn10))))), (((-locals.var_pdji__blk371_dn11) * assign12880_body144_e18222) + (assign12880_body144_e18211 * ((((locals.var_pji21__blk375_dn11 * locals.var_pf1__blk359) + (locals.var_pji21__blk375 * locals.var_pf1__blk359_dn11)) + ((locals.var_pji22__blk376_dn11 * locals.var_pf2__blk363) + (locals.var_pji22__blk376 * locals.var_pf2__blk363_dn11))) + ((locals.var_pji23__blk377_dn11 * locals.var_pf3__blk367) + (locals.var_pji23__blk377 * locals.var_pf3__blk367_dn11))))), (((-locals.var_pdji__blk371_dn12) * assign12880_body144_e18222) + (assign12880_body144_e18211 * ((((locals.var_pji21__blk375_dn12 * locals.var_pf1__blk359) + (locals.var_pji21__blk375 * locals.var_pf1__blk359_dn12)) + ((locals.var_pji22__blk376_dn12 * locals.var_pf2__blk363) + (locals.var_pji22__blk376 * locals.var_pf2__blk363_dn12))) + ((locals.var_pji23__blk377_dn12 * locals.var_pf3__blk367) + (locals.var_pji23__blk377 * locals.var_pf3__blk367_dn12))))), (((-locals.var_pdji__blk371_dn17) * assign12880_body144_e18222) + (assign12880_body144_e18211 * ((((locals.var_pji21__blk375_dn17 * locals.var_pf1__blk359) + (locals.var_pji21__blk375 * locals.var_pf1__blk359_dn17)) + ((locals.var_pji22__blk376_dn17 * locals.var_pf2__blk363) + (locals.var_pji22__blk376 * locals.var_pf2__blk363_dn17))) + ((locals.var_pji23__blk377_dn17 * locals.var_pf3__blk367) + (locals.var_pji23__blk377 * locals.var_pf3__blk367_dn17))))),)
    } else {
        (locals.var_dpbs__blk357, locals.var_dpbs__blk357_dn0, locals.var_dpbs__blk357_dn2, locals.var_dpbs__blk357_dn6, locals.var_dpbs__blk357_dn7, locals.var_dpbs__blk357_dn10, locals.var_dpbs__blk357_dn11, locals.var_dpbs__blk357_dn12, locals.var_dpbs__blk357_dn17,)
    }
};
            locals.var_dpbs__blk357 = assign12880_body144_e18225;
            locals.var_dpbs__blk357_dn0 = assign12880_body144_e18225_d_n0;
            locals.var_dpbs__blk357_dn2 = assign12880_body144_e18225_d_n2;
            locals.var_dpbs__blk357_dn6 = assign12880_body144_e18225_d_n6;
            locals.var_dpbs__blk357_dn7 = assign12880_body144_e18225_d_n7;
            locals.var_dpbs__blk357_dn10 = assign12880_body144_e18225_d_n10;
            locals.var_dpbs__blk357_dn11 = assign12880_body144_e18225_d_n11;
            locals.var_dpbs__blk357_dn12 = assign12880_body144_e18225_d_n12;
            locals.var_dpbs__blk357_dn17 = assign12880_body144_e18225_d_n17;
            let (assign12880_body145_e18248, assign12880_body145_e18248_d_n0, assign12880_body145_e18248_d_n2, assign12880_body145_e18248_d_n6, assign12880_body145_e18248_d_n7, assign12880_body145_e18248_d_n10, assign12880_body145_e18248_d_n11, assign12880_body145_e18248_d_n12, assign12880_body145_e18248_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body145_e18234: f64 = (-locals.var_pdji__blk371);
        let assign12880_body145_e18237: f64 = (locals.var_pji31__blk378 * locals.var_pf1__blk359);
        let assign12880_body145_e18240: f64 = (locals.var_pji32__blk379 * locals.var_pf2__blk363);
        let assign12880_body145_e18241: f64 = (assign12880_body145_e18237 + assign12880_body145_e18240);
        let assign12880_body145_e18244: f64 = (locals.var_pji33__blk380 * locals.var_pf3__blk367);
        let assign12880_body145_e18245: f64 = (assign12880_body145_e18241 + assign12880_body145_e18244);
        let assign12880_body145_e18246: f64 = (assign12880_body145_e18234 * assign12880_body145_e18245);
        (assign12880_body145_e18246, (((-locals.var_pdji__blk371_dn0) * assign12880_body145_e18245) + (assign12880_body145_e18234 * (((locals.var_pji31__blk378 * locals.var_pf1__blk359_dn0) + ((locals.var_pji32__blk379_dn0 * locals.var_pf2__blk363) + (locals.var_pji32__blk379 * locals.var_pf2__blk363_dn0))) + ((locals.var_pji33__blk380_dn0 * locals.var_pf3__blk367) + (locals.var_pji33__blk380 * locals.var_pf3__blk367_dn0))))), (((-locals.var_pdji__blk371_dn2) * assign12880_body145_e18245) + (assign12880_body145_e18234 * (((locals.var_pji31__blk378 * locals.var_pf1__blk359_dn2) + ((locals.var_pji32__blk379_dn2 * locals.var_pf2__blk363) + (locals.var_pji32__blk379 * locals.var_pf2__blk363_dn2))) + ((locals.var_pji33__blk380_dn2 * locals.var_pf3__blk367) + (locals.var_pji33__blk380 * locals.var_pf3__blk367_dn2))))), (((-locals.var_pdji__blk371_dn6) * assign12880_body145_e18245) + (assign12880_body145_e18234 * (((locals.var_pji31__blk378 * locals.var_pf1__blk359_dn6) + ((locals.var_pji32__blk379_dn6 * locals.var_pf2__blk363) + (locals.var_pji32__blk379 * locals.var_pf2__blk363_dn6))) + ((locals.var_pji33__blk380_dn6 * locals.var_pf3__blk367) + (locals.var_pji33__blk380 * locals.var_pf3__blk367_dn6))))), (((-locals.var_pdji__blk371_dn7) * assign12880_body145_e18245) + (assign12880_body145_e18234 * (((locals.var_pji31__blk378 * locals.var_pf1__blk359_dn7) + ((locals.var_pji32__blk379_dn7 * locals.var_pf2__blk363) + (locals.var_pji32__blk379 * locals.var_pf2__blk363_dn7))) + ((locals.var_pji33__blk380_dn7 * locals.var_pf3__blk367) + (locals.var_pji33__blk380 * locals.var_pf3__blk367_dn7))))), (((-locals.var_pdji__blk371_dn10) * assign12880_body145_e18245) + (assign12880_body145_e18234 * (((locals.var_pji31__blk378 * locals.var_pf1__blk359_dn10) + ((locals.var_pji32__blk379_dn10 * locals.var_pf2__blk363) + (locals.var_pji32__blk379 * locals.var_pf2__blk363_dn10))) + ((locals.var_pji33__blk380_dn10 * locals.var_pf3__blk367) + (locals.var_pji33__blk380 * locals.var_pf3__blk367_dn10))))), (((-locals.var_pdji__blk371_dn11) * assign12880_body145_e18245) + (assign12880_body145_e18234 * (((locals.var_pji31__blk378 * locals.var_pf1__blk359_dn11) + ((locals.var_pji32__blk379_dn11 * locals.var_pf2__blk363) + (locals.var_pji32__blk379 * locals.var_pf2__blk363_dn11))) + ((locals.var_pji33__blk380_dn11 * locals.var_pf3__blk367) + (locals.var_pji33__blk380 * locals.var_pf3__blk367_dn11))))), (((-locals.var_pdji__blk371_dn12) * assign12880_body145_e18245) + (assign12880_body145_e18234 * (((locals.var_pji31__blk378 * locals.var_pf1__blk359_dn12) + ((locals.var_pji32__blk379_dn12 * locals.var_pf2__blk363) + (locals.var_pji32__blk379 * locals.var_pf2__blk363_dn12))) + ((locals.var_pji33__blk380_dn12 * locals.var_pf3__blk367) + (locals.var_pji33__blk380 * locals.var_pf3__blk367_dn12))))), (((-locals.var_pdji__blk371_dn17) * assign12880_body145_e18245) + (assign12880_body145_e18234 * (((locals.var_pji31__blk378 * locals.var_pf1__blk359_dn17) + ((locals.var_pji32__blk379_dn17 * locals.var_pf2__blk363) + (locals.var_pji32__blk379 * locals.var_pf2__blk363_dn17))) + ((locals.var_pji33__blk380_dn17 * locals.var_pf3__blk367) + (locals.var_pji33__blk380 * locals.var_pf3__blk367_dn17))))),)
    } else {
        (locals.var_dpsb__blk358, locals.var_dpsb__blk358_dn0, locals.var_dpsb__blk358_dn2, locals.var_dpsb__blk358_dn6, locals.var_dpsb__blk358_dn7, locals.var_dpsb__blk358_dn10, locals.var_dpsb__blk358_dn11, locals.var_dpsb__blk358_dn12, locals.var_dpsb__blk358_dn17,)
    }
};
            locals.var_dpsb__blk358 = assign12880_body145_e18248;
            locals.var_dpsb__blk358_dn0 = assign12880_body145_e18248_d_n0;
            locals.var_dpsb__blk358_dn2 = assign12880_body145_e18248_d_n2;
            locals.var_dpsb__blk358_dn6 = assign12880_body145_e18248_d_n6;
            locals.var_dpsb__blk358_dn7 = assign12880_body145_e18248_d_n7;
            locals.var_dpsb__blk358_dn10 = assign12880_body145_e18248_d_n10;
            locals.var_dpsb__blk358_dn11 = assign12880_body145_e18248_d_n11;
            locals.var_dpsb__blk358_dn12 = assign12880_body145_e18248_d_n12;
            locals.var_dpsb__blk358_dn17 = assign12880_body145_e18248_d_n17;
            let (assign12880_body146_e18259, assign12880_body146_e18259_d_n0, assign12880_body146_e18259_d_n2, assign12880_body146_e18259_d_n6, assign12880_body146_e18259_d_n7, assign12880_body146_e18259_d_n10, assign12880_body146_e18259_d_n11, assign12880_body146_e18259_d_n12, assign12880_body146_e18259_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body146_e18257: f64 = (locals.var_dpss__blk356).abs();
        (assign12880_body146_e18257, if locals.var_dpss__blk356 >= 0.0 { locals.var_dpss__blk356_dn0 } else { (-locals.var_dpss__blk356_dn0) }, if locals.var_dpss__blk356 >= 0.0 { locals.var_dpss__blk356_dn2 } else { (-locals.var_dpss__blk356_dn2) }, if locals.var_dpss__blk356 >= 0.0 { locals.var_dpss__blk356_dn6 } else { (-locals.var_dpss__blk356_dn6) }, if locals.var_dpss__blk356 >= 0.0 { locals.var_dpss__blk356_dn7 } else { (-locals.var_dpss__blk356_dn7) }, if locals.var_dpss__blk356 >= 0.0 { locals.var_dpss__blk356_dn10 } else { (-locals.var_dpss__blk356_dn10) }, if locals.var_dpss__blk356 >= 0.0 { locals.var_dpss__blk356_dn11 } else { (-locals.var_dpss__blk356_dn11) }, if locals.var_dpss__blk356 >= 0.0 { locals.var_dpss__blk356_dn12 } else { (-locals.var_dpss__blk356_dn12) }, if locals.var_dpss__blk356 >= 0.0 { locals.var_dpss__blk356_dn17 } else { (-locals.var_dpss__blk356_dn17) },)
    } else {
        (locals.var_t1__blk349, locals.var_t1__blk349_dn0, locals.var_t1__blk349_dn2, locals.var_t1__blk349_dn6, locals.var_t1__blk349_dn7, locals.var_t1__blk349_dn10, locals.var_t1__blk349_dn11, locals.var_t1__blk349_dn12, locals.var_t1__blk349_dn17,)
    }
};
            locals.var_t1__blk349 = assign12880_body146_e18259;
            locals.var_t1__blk349_dn0 = assign12880_body146_e18259_d_n0;
            locals.var_t1__blk349_dn2 = assign12880_body146_e18259_d_n2;
            locals.var_t1__blk349_dn6 = assign12880_body146_e18259_d_n6;
            locals.var_t1__blk349_dn7 = assign12880_body146_e18259_d_n7;
            locals.var_t1__blk349_dn10 = assign12880_body146_e18259_d_n10;
            locals.var_t1__blk349_dn11 = assign12880_body146_e18259_d_n11;
            locals.var_t1__blk349_dn12 = assign12880_body146_e18259_d_n12;
            locals.var_t1__blk349_dn17 = assign12880_body146_e18259_d_n17;
            let assign12880_body147_e18262: f64 = (locals.var_dpbs__blk357).abs();
            let assign12880_body147_e18263: f64 = if locals.var_t1__blk349 < assign12880_body147_e18262 { 1.0 } else { 0.0 };
            locals.var_guard403 = assign12880_body147_e18263;
            let (assign12880_body148_e18276, assign12880_body148_e18276_d_n0, assign12880_body148_e18276_d_n2, assign12880_body148_e18276_d_n6, assign12880_body148_e18276_d_n7, assign12880_body148_e18276_d_n10, assign12880_body148_e18276_d_n11, assign12880_body148_e18276_d_n12, assign12880_body148_e18276_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign12880_body148_e18274: f64 = (locals.var_dpbs__blk357).abs();
        (assign12880_body148_e18274, if locals.var_dpbs__blk357 >= 0.0 { locals.var_dpbs__blk357_dn0 } else { (-locals.var_dpbs__blk357_dn0) }, if locals.var_dpbs__blk357 >= 0.0 { locals.var_dpbs__blk357_dn2 } else { (-locals.var_dpbs__blk357_dn2) }, if locals.var_dpbs__blk357 >= 0.0 { locals.var_dpbs__blk357_dn6 } else { (-locals.var_dpbs__blk357_dn6) }, if locals.var_dpbs__blk357 >= 0.0 { locals.var_dpbs__blk357_dn7 } else { (-locals.var_dpbs__blk357_dn7) }, if locals.var_dpbs__blk357 >= 0.0 { locals.var_dpbs__blk357_dn10 } else { (-locals.var_dpbs__blk357_dn10) }, if locals.var_dpbs__blk357 >= 0.0 { locals.var_dpbs__blk357_dn11 } else { (-locals.var_dpbs__blk357_dn11) }, if locals.var_dpbs__blk357 >= 0.0 { locals.var_dpbs__blk357_dn12 } else { (-locals.var_dpbs__blk357_dn12) }, if locals.var_dpbs__blk357 >= 0.0 { locals.var_dpbs__blk357_dn17 } else { (-locals.var_dpbs__blk357_dn17) },)
    } else {
        (locals.var_t1__blk349, locals.var_t1__blk349_dn0, locals.var_t1__blk349_dn2, locals.var_t1__blk349_dn6, locals.var_t1__blk349_dn7, locals.var_t1__blk349_dn10, locals.var_t1__blk349_dn11, locals.var_t1__blk349_dn12, locals.var_t1__blk349_dn17,)
    }
};
            locals.var_t1__blk349 = assign12880_body148_e18276;
            locals.var_t1__blk349_dn0 = assign12880_body148_e18276_d_n0;
            locals.var_t1__blk349_dn2 = assign12880_body148_e18276_d_n2;
            locals.var_t1__blk349_dn6 = assign12880_body148_e18276_d_n6;
            locals.var_t1__blk349_dn7 = assign12880_body148_e18276_d_n7;
            locals.var_t1__blk349_dn10 = assign12880_body148_e18276_d_n10;
            locals.var_t1__blk349_dn11 = assign12880_body148_e18276_d_n11;
            locals.var_t1__blk349_dn12 = assign12880_body148_e18276_d_n12;
            locals.var_t1__blk349_dn17 = assign12880_body148_e18276_d_n17;
            let assign12880_body149_e18279: f64 = (locals.var_dpsb__blk358).abs();
            let assign12880_body149_e18280: f64 = if locals.var_t1__blk349 < assign12880_body149_e18279 { 1.0 } else { 0.0 };
            locals.var_guard404 = assign12880_body149_e18280;
            let (assign12880_body150_e18293, assign12880_body150_e18293_d_n0, assign12880_body150_e18293_d_n2, assign12880_body150_e18293_d_n6, assign12880_body150_e18293_d_n7, assign12880_body150_e18293_d_n10, assign12880_body150_e18293_d_n11, assign12880_body150_e18293_d_n12, assign12880_body150_e18293_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard404 != 0.0)) {
        let assign12880_body150_e18291: f64 = (locals.var_dpsb__blk358).abs();
        (assign12880_body150_e18291, if locals.var_dpsb__blk358 >= 0.0 { locals.var_dpsb__blk358_dn0 } else { (-locals.var_dpsb__blk358_dn0) }, if locals.var_dpsb__blk358 >= 0.0 { locals.var_dpsb__blk358_dn2 } else { (-locals.var_dpsb__blk358_dn2) }, if locals.var_dpsb__blk358 >= 0.0 { locals.var_dpsb__blk358_dn6 } else { (-locals.var_dpsb__blk358_dn6) }, if locals.var_dpsb__blk358 >= 0.0 { locals.var_dpsb__blk358_dn7 } else { (-locals.var_dpsb__blk358_dn7) }, if locals.var_dpsb__blk358 >= 0.0 { locals.var_dpsb__blk358_dn10 } else { (-locals.var_dpsb__blk358_dn10) }, if locals.var_dpsb__blk358 >= 0.0 { locals.var_dpsb__blk358_dn11 } else { (-locals.var_dpsb__blk358_dn11) }, if locals.var_dpsb__blk358 >= 0.0 { locals.var_dpsb__blk358_dn12 } else { (-locals.var_dpsb__blk358_dn12) }, if locals.var_dpsb__blk358 >= 0.0 { locals.var_dpsb__blk358_dn17 } else { (-locals.var_dpsb__blk358_dn17) },)
    } else {
        (locals.var_t1__blk349, locals.var_t1__blk349_dn0, locals.var_t1__blk349_dn2, locals.var_t1__blk349_dn6, locals.var_t1__blk349_dn7, locals.var_t1__blk349_dn10, locals.var_t1__blk349_dn11, locals.var_t1__blk349_dn12, locals.var_t1__blk349_dn17,)
    }
};
            locals.var_t1__blk349 = assign12880_body150_e18293;
            locals.var_t1__blk349_dn0 = assign12880_body150_e18293_d_n0;
            locals.var_t1__blk349_dn2 = assign12880_body150_e18293_d_n2;
            locals.var_t1__blk349_dn6 = assign12880_body150_e18293_d_n6;
            locals.var_t1__blk349_dn7 = assign12880_body150_e18293_d_n7;
            locals.var_t1__blk349_dn10 = assign12880_body150_e18293_d_n10;
            locals.var_t1__blk349_dn11 = assign12880_body150_e18293_d_n11;
            locals.var_t1__blk349_dn12 = assign12880_body150_e18293_d_n12;
            locals.var_t1__blk349_dn17 = assign12880_body150_e18293_d_n17;
            let (assign12880_body151_e18303,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12880_body151_e18303;
            let assign12880_body152_e18306: f64 = if locals.var_lp_sl > 80.0 { 1.0 } else { 0.0 };
            locals.var_guard405 = assign12880_body152_e18306;
            let (assign12880_body153_e18318,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard405 != 0.0)) {
        (125.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12880_body153_e18318;
            let assign12880_body154_e18321: f64 = if locals.var_lp_sl > 40.0 { 1.0 } else { 0.0 };
            locals.var_guard406 = assign12880_body154_e18321;
            let (assign12880_body155_e18336,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard405 == 0.0)) && (locals.var_guard406 != 0.0)) {
        (125.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12880_body155_e18336;
            let assign12880_body156_e18339: f64 = if locals.var_lp_sl > 20.0 { 1.0 } else { 0.0 };
            locals.var_guard407 = assign12880_body156_e18339;
            let (assign12880_body157_e18357,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard405 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard407 != 0.0)) {
        (25.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12880_body157_e18357;
            let assign12880_body158_e18360: f64 = if locals.var_lp_sl > 10.0 { 1.0 } else { 0.0 };
            locals.var_guard408 = assign12880_body158_e18360;
            let (assign12880_body159_e18381,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard405 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard407 == 0.0)) && (locals.var_guard408 != 0.0)) {
        (5.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12880_body159_e18381;
            let assign12880_body160_e18385: f64 = (0.1 / locals.var_scale_fac);
            let assign12880_body160_e18386: f64 = if locals.var_t1__blk349 > assign12880_body160_e18385 { 1.0 } else { 0.0 };
            locals.var_guard409 = assign12880_body160_e18386;
            let (assign12880_body161_e18404, assign12880_body161_e18404_d_n0, assign12880_body161_e18404_d_n2, assign12880_body161_e18404_d_n6, assign12880_body161_e18404_d_n7, assign12880_body161_e18404_d_n10, assign12880_body161_e18404_d_n11, assign12880_body161_e18404_d_n12, assign12880_body161_e18404_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard409 != 0.0)) {
        let assign12880_body161_e18399: f64 = (0.1 / locals.var_scale_fac);
        let assign12880_body161_e18401: f64 = (assign12880_body161_e18399 / locals.var_t1__blk349);
        let assign12880_body161_e18402: f64 = (locals.var_dpss__blk356 * assign12880_body161_e18401);
        (assign12880_body161_e18402, ((locals.var_dpss__blk356_dn0 * assign12880_body161_e18401) + (locals.var_dpss__blk356 * (-((assign12880_body161_e18399 * locals.var_t1__blk349_dn0) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpss__blk356_dn2 * assign12880_body161_e18401) + (locals.var_dpss__blk356 * (-((assign12880_body161_e18399 * locals.var_t1__blk349_dn2) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpss__blk356_dn6 * assign12880_body161_e18401) + (locals.var_dpss__blk356 * (-((assign12880_body161_e18399 * locals.var_t1__blk349_dn6) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpss__blk356_dn7 * assign12880_body161_e18401) + (locals.var_dpss__blk356 * (-((assign12880_body161_e18399 * locals.var_t1__blk349_dn7) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpss__blk356_dn10 * assign12880_body161_e18401) + (locals.var_dpss__blk356 * (-((assign12880_body161_e18399 * locals.var_t1__blk349_dn10) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpss__blk356_dn11 * assign12880_body161_e18401) + (locals.var_dpss__blk356 * (-((assign12880_body161_e18399 * locals.var_t1__blk349_dn11) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpss__blk356_dn12 * assign12880_body161_e18401) + (locals.var_dpss__blk356 * (-((assign12880_body161_e18399 * locals.var_t1__blk349_dn12) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpss__blk356_dn17 * assign12880_body161_e18401) + (locals.var_dpss__blk356 * (-((assign12880_body161_e18399 * locals.var_t1__blk349_dn17) / (locals.var_t1__blk349 * locals.var_t1__blk349))))),)
    } else {
        (locals.var_dpss__blk356, locals.var_dpss__blk356_dn0, locals.var_dpss__blk356_dn2, locals.var_dpss__blk356_dn6, locals.var_dpss__blk356_dn7, locals.var_dpss__blk356_dn10, locals.var_dpss__blk356_dn11, locals.var_dpss__blk356_dn12, locals.var_dpss__blk356_dn17,)
    }
};
            locals.var_dpss__blk356 = assign12880_body161_e18404;
            locals.var_dpss__blk356_dn0 = assign12880_body161_e18404_d_n0;
            locals.var_dpss__blk356_dn2 = assign12880_body161_e18404_d_n2;
            locals.var_dpss__blk356_dn6 = assign12880_body161_e18404_d_n6;
            locals.var_dpss__blk356_dn7 = assign12880_body161_e18404_d_n7;
            locals.var_dpss__blk356_dn10 = assign12880_body161_e18404_d_n10;
            locals.var_dpss__blk356_dn11 = assign12880_body161_e18404_d_n11;
            locals.var_dpss__blk356_dn12 = assign12880_body161_e18404_d_n12;
            locals.var_dpss__blk356_dn17 = assign12880_body161_e18404_d_n17;
            let (assign12880_body162_e18422, assign12880_body162_e18422_d_n0, assign12880_body162_e18422_d_n2, assign12880_body162_e18422_d_n6, assign12880_body162_e18422_d_n7, assign12880_body162_e18422_d_n10, assign12880_body162_e18422_d_n11, assign12880_body162_e18422_d_n12, assign12880_body162_e18422_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard409 != 0.0)) {
        let assign12880_body162_e18417: f64 = (0.1 / locals.var_scale_fac);
        let assign12880_body162_e18419: f64 = (assign12880_body162_e18417 / locals.var_t1__blk349);
        let assign12880_body162_e18420: f64 = (locals.var_dpbs__blk357 * assign12880_body162_e18419);
        (assign12880_body162_e18420, ((locals.var_dpbs__blk357_dn0 * assign12880_body162_e18419) + (locals.var_dpbs__blk357 * (-((assign12880_body162_e18417 * locals.var_t1__blk349_dn0) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpbs__blk357_dn2 * assign12880_body162_e18419) + (locals.var_dpbs__blk357 * (-((assign12880_body162_e18417 * locals.var_t1__blk349_dn2) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpbs__blk357_dn6 * assign12880_body162_e18419) + (locals.var_dpbs__blk357 * (-((assign12880_body162_e18417 * locals.var_t1__blk349_dn6) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpbs__blk357_dn7 * assign12880_body162_e18419) + (locals.var_dpbs__blk357 * (-((assign12880_body162_e18417 * locals.var_t1__blk349_dn7) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpbs__blk357_dn10 * assign12880_body162_e18419) + (locals.var_dpbs__blk357 * (-((assign12880_body162_e18417 * locals.var_t1__blk349_dn10) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpbs__blk357_dn11 * assign12880_body162_e18419) + (locals.var_dpbs__blk357 * (-((assign12880_body162_e18417 * locals.var_t1__blk349_dn11) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpbs__blk357_dn12 * assign12880_body162_e18419) + (locals.var_dpbs__blk357 * (-((assign12880_body162_e18417 * locals.var_t1__blk349_dn12) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpbs__blk357_dn17 * assign12880_body162_e18419) + (locals.var_dpbs__blk357 * (-((assign12880_body162_e18417 * locals.var_t1__blk349_dn17) / (locals.var_t1__blk349 * locals.var_t1__blk349))))),)
    } else {
        (locals.var_dpbs__blk357, locals.var_dpbs__blk357_dn0, locals.var_dpbs__blk357_dn2, locals.var_dpbs__blk357_dn6, locals.var_dpbs__blk357_dn7, locals.var_dpbs__blk357_dn10, locals.var_dpbs__blk357_dn11, locals.var_dpbs__blk357_dn12, locals.var_dpbs__blk357_dn17,)
    }
};
            locals.var_dpbs__blk357 = assign12880_body162_e18422;
            locals.var_dpbs__blk357_dn0 = assign12880_body162_e18422_d_n0;
            locals.var_dpbs__blk357_dn2 = assign12880_body162_e18422_d_n2;
            locals.var_dpbs__blk357_dn6 = assign12880_body162_e18422_d_n6;
            locals.var_dpbs__blk357_dn7 = assign12880_body162_e18422_d_n7;
            locals.var_dpbs__blk357_dn10 = assign12880_body162_e18422_d_n10;
            locals.var_dpbs__blk357_dn11 = assign12880_body162_e18422_d_n11;
            locals.var_dpbs__blk357_dn12 = assign12880_body162_e18422_d_n12;
            locals.var_dpbs__blk357_dn17 = assign12880_body162_e18422_d_n17;
            let (assign12880_body163_e18440, assign12880_body163_e18440_d_n0, assign12880_body163_e18440_d_n2, assign12880_body163_e18440_d_n6, assign12880_body163_e18440_d_n7, assign12880_body163_e18440_d_n10, assign12880_body163_e18440_d_n11, assign12880_body163_e18440_d_n12, assign12880_body163_e18440_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard409 != 0.0)) {
        let assign12880_body163_e18435: f64 = (0.1 / locals.var_scale_fac);
        let assign12880_body163_e18437: f64 = (assign12880_body163_e18435 / locals.var_t1__blk349);
        let assign12880_body163_e18438: f64 = (locals.var_dpsb__blk358 * assign12880_body163_e18437);
        (assign12880_body163_e18438, ((locals.var_dpsb__blk358_dn0 * assign12880_body163_e18437) + (locals.var_dpsb__blk358 * (-((assign12880_body163_e18435 * locals.var_t1__blk349_dn0) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpsb__blk358_dn2 * assign12880_body163_e18437) + (locals.var_dpsb__blk358 * (-((assign12880_body163_e18435 * locals.var_t1__blk349_dn2) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpsb__blk358_dn6 * assign12880_body163_e18437) + (locals.var_dpsb__blk358 * (-((assign12880_body163_e18435 * locals.var_t1__blk349_dn6) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpsb__blk358_dn7 * assign12880_body163_e18437) + (locals.var_dpsb__blk358 * (-((assign12880_body163_e18435 * locals.var_t1__blk349_dn7) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpsb__blk358_dn10 * assign12880_body163_e18437) + (locals.var_dpsb__blk358 * (-((assign12880_body163_e18435 * locals.var_t1__blk349_dn10) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpsb__blk358_dn11 * assign12880_body163_e18437) + (locals.var_dpsb__blk358 * (-((assign12880_body163_e18435 * locals.var_t1__blk349_dn11) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpsb__blk358_dn12 * assign12880_body163_e18437) + (locals.var_dpsb__blk358 * (-((assign12880_body163_e18435 * locals.var_t1__blk349_dn12) / (locals.var_t1__blk349 * locals.var_t1__blk349))))), ((locals.var_dpsb__blk358_dn17 * assign12880_body163_e18437) + (locals.var_dpsb__blk358 * (-((assign12880_body163_e18435 * locals.var_t1__blk349_dn17) / (locals.var_t1__blk349 * locals.var_t1__blk349))))),)
    } else {
        (locals.var_dpsb__blk358, locals.var_dpsb__blk358_dn0, locals.var_dpsb__blk358_dn2, locals.var_dpsb__blk358_dn6, locals.var_dpsb__blk358_dn7, locals.var_dpsb__blk358_dn10, locals.var_dpsb__blk358_dn11, locals.var_dpsb__blk358_dn12, locals.var_dpsb__blk358_dn17,)
    }
};
            locals.var_dpsb__blk358 = assign12880_body163_e18440;
            locals.var_dpsb__blk358_dn0 = assign12880_body163_e18440_d_n0;
            locals.var_dpsb__blk358_dn2 = assign12880_body163_e18440_d_n2;
            locals.var_dpsb__blk358_dn6 = assign12880_body163_e18440_d_n6;
            locals.var_dpsb__blk358_dn7 = assign12880_body163_e18440_d_n7;
            locals.var_dpsb__blk358_dn10 = assign12880_body163_e18440_d_n10;
            locals.var_dpsb__blk358_dn11 = assign12880_body163_e18440_d_n11;
            locals.var_dpsb__blk358_dn12 = assign12880_body163_e18440_d_n12;
            locals.var_dpsb__blk358_dn17 = assign12880_body163_e18440_d_n17;
            let (assign12880_body164_e18452, assign12880_body164_e18452_d_n0, assign12880_body164_e18452_d_n2, assign12880_body164_e18452_d_n6, assign12880_body164_e18452_d_n7, assign12880_body164_e18452_d_n10, assign12880_body164_e18452_d_n11, assign12880_body164_e18452_d_n12, assign12880_body164_e18452_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body164_e18450: f64 = (locals.var_phi_sl_soi + locals.var_dpss__blk356);
        (assign12880_body164_e18450, (locals.var_phi_sl_soi_dn0 + locals.var_dpss__blk356_dn0), (locals.var_phi_sl_soi_dn2 + locals.var_dpss__blk356_dn2), (locals.var_phi_sl_soi_dn6 + locals.var_dpss__blk356_dn6), (locals.var_phi_sl_soi_dn7 + locals.var_dpss__blk356_dn7), (locals.var_phi_sl_soi_dn10 + locals.var_dpss__blk356_dn10), (locals.var_phi_sl_soi_dn11 + locals.var_dpss__blk356_dn11), (locals.var_phi_sl_soi_dn12 + locals.var_dpss__blk356_dn12), (locals.var_phi_sl_soi_dn17 + locals.var_dpss__blk356_dn17),)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
            locals.var_phi_sl_soi = assign12880_body164_e18452;
            locals.var_phi_sl_soi_dn0 = assign12880_body164_e18452_d_n0;
            locals.var_phi_sl_soi_dn2 = assign12880_body164_e18452_d_n2;
            locals.var_phi_sl_soi_dn6 = assign12880_body164_e18452_d_n6;
            locals.var_phi_sl_soi_dn7 = assign12880_body164_e18452_d_n7;
            locals.var_phi_sl_soi_dn10 = assign12880_body164_e18452_d_n10;
            locals.var_phi_sl_soi_dn11 = assign12880_body164_e18452_d_n11;
            locals.var_phi_sl_soi_dn12 = assign12880_body164_e18452_d_n12;
            locals.var_phi_sl_soi_dn17 = assign12880_body164_e18452_d_n17;
            let (assign12880_body165_e18464, assign12880_body165_e18464_d_n0, assign12880_body165_e18464_d_n2, assign12880_body165_e18464_d_n6, assign12880_body165_e18464_d_n7, assign12880_body165_e18464_d_n10, assign12880_body165_e18464_d_n11, assign12880_body165_e18464_d_n12, assign12880_body165_e18464_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body165_e18462: f64 = (locals.var_phi_bl_soi + locals.var_dpbs__blk357);
        (assign12880_body165_e18462, (locals.var_phi_bl_soi_dn0 + locals.var_dpbs__blk357_dn0), (locals.var_phi_bl_soi_dn2 + locals.var_dpbs__blk357_dn2), (locals.var_phi_bl_soi_dn6 + locals.var_dpbs__blk357_dn6), (locals.var_phi_bl_soi_dn7 + locals.var_dpbs__blk357_dn7), (locals.var_phi_bl_soi_dn10 + locals.var_dpbs__blk357_dn10), (locals.var_phi_bl_soi_dn11 + locals.var_dpbs__blk357_dn11), (locals.var_phi_bl_soi_dn12 + locals.var_dpbs__blk357_dn12), (locals.var_phi_bl_soi_dn17 + locals.var_dpbs__blk357_dn17),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
            locals.var_phi_bl_soi = assign12880_body165_e18464;
            locals.var_phi_bl_soi_dn0 = assign12880_body165_e18464_d_n0;
            locals.var_phi_bl_soi_dn2 = assign12880_body165_e18464_d_n2;
            locals.var_phi_bl_soi_dn6 = assign12880_body165_e18464_d_n6;
            locals.var_phi_bl_soi_dn7 = assign12880_body165_e18464_d_n7;
            locals.var_phi_bl_soi_dn10 = assign12880_body165_e18464_d_n10;
            locals.var_phi_bl_soi_dn11 = assign12880_body165_e18464_d_n11;
            locals.var_phi_bl_soi_dn12 = assign12880_body165_e18464_d_n12;
            locals.var_phi_bl_soi_dn17 = assign12880_body165_e18464_d_n17;
            let (assign12880_body166_e18476, assign12880_body166_e18476_d_n0, assign12880_body166_e18476_d_n2, assign12880_body166_e18476_d_n6, assign12880_body166_e18476_d_n7, assign12880_body166_e18476_d_n10, assign12880_body166_e18476_d_n11, assign12880_body166_e18476_d_n12, assign12880_body166_e18476_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body166_e18474: f64 = (locals.var_phi_sl_bulk + locals.var_dpsb__blk358);
        (assign12880_body166_e18474, (locals.var_phi_sl_bulk_dn0 + locals.var_dpsb__blk358_dn0), (locals.var_phi_sl_bulk_dn2 + locals.var_dpsb__blk358_dn2), (locals.var_phi_sl_bulk_dn6 + locals.var_dpsb__blk358_dn6), (locals.var_phi_sl_bulk_dn7 + locals.var_dpsb__blk358_dn7), (locals.var_phi_sl_bulk_dn10 + locals.var_dpsb__blk358_dn10), (locals.var_phi_sl_bulk_dn11 + locals.var_dpsb__blk358_dn11), (locals.var_phi_sl_bulk_dn12 + locals.var_dpsb__blk358_dn12), (locals.var_phi_sl_bulk_dn17 + locals.var_dpsb__blk358_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12880_body166_e18476;
            locals.var_phi_sl_bulk_dn0 = assign12880_body166_e18476_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12880_body166_e18476_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12880_body166_e18476_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12880_body166_e18476_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12880_body166_e18476_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12880_body166_e18476_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12880_body166_e18476_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12880_body166_e18476_d_n17;
            let (assign12880_body167_e18490,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign12880_body167_e18486: f64 = (5e-12 * locals.var_scale_fac);
        let assign12880_body167_e18488: f64 = assign12880_body167_e18486;
        (assign12880_body167_e18488,)
    } else {
        (locals.var_psconv_3d,)
    }
};
            locals.var_psconv_3d = assign12880_body167_e18490;
            let assign12880_body168_e18493: f64 = if locals.var_t1__blk349 < locals.var_psconv_3d { 1.0 } else { 0.0 };
            locals.var_guard410 = assign12880_body168_e18493;
            let (assign12880_body169_e18505,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard410 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign12880_body169_e18505;
            let (assign12880_body170_e18514,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign12880_body170_e18512: f64 = (locals.var_lp_sl + 1.0);
        (assign12880_body170_e18512,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12880_body170_e18514;
        }

    }

    pub(super) fn stamp_transient_block_42(
        locals: &mut StampLocals,
    ) {
        let (assign12890_e18526,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let (assign12890_e18524,) = {
            if (locals.var_flg_brk8 > 0.0) {
                (locals.var_flg_brk8,)
            } else {
                (locals.var_lp_sl,)
            }
        };
        (assign12890_e18524,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12890_e18526;

        let assign12900_e18529: f64 = if locals.var_flg_conv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign12900_e18529;

        let (assign12910_e18538, assign12910_e18538_d_n0, assign12910_e18538_d_n2, assign12910_e18538_d_n6, assign12910_e18538_d_n7, assign12910_e18538_d_n10, assign12910_e18538_d_n11, assign12910_e18538_d_n12, assign12910_e18538_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard411 != 0.0)) {
        (locals.var_phi_sl_soi_ini, locals.var_phi_sl_soi_ini_dn0, locals.var_phi_sl_soi_ini_dn2, locals.var_phi_sl_soi_ini_dn6, locals.var_phi_sl_soi_ini_dn7, locals.var_phi_sl_soi_ini_dn10, locals.var_phi_sl_soi_ini_dn11, locals.var_phi_sl_soi_ini_dn12, locals.var_phi_sl_soi_ini_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12910_e18538;
        locals.var_phi_sl_soi_dn0 = assign12910_e18538_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12910_e18538_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12910_e18538_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12910_e18538_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12910_e18538_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12910_e18538_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12910_e18538_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12910_e18538_d_n17;

        let (assign12920_e18547, assign12920_e18547_d_n0, assign12920_e18547_d_n2, assign12920_e18547_d_n6, assign12920_e18547_d_n7, assign12920_e18547_d_n10, assign12920_e18547_d_n11, assign12920_e18547_d_n12, assign12920_e18547_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard411 != 0.0)) {
        (locals.var_phi_bl_soi_ini, locals.var_phi_bl_soi_ini_dn0, locals.var_phi_bl_soi_ini_dn2, locals.var_phi_bl_soi_ini_dn6, locals.var_phi_bl_soi_ini_dn7, locals.var_phi_bl_soi_ini_dn10, locals.var_phi_bl_soi_ini_dn11, locals.var_phi_bl_soi_ini_dn12, locals.var_phi_bl_soi_ini_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12920_e18547;
        locals.var_phi_bl_soi_dn0 = assign12920_e18547_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12920_e18547_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12920_e18547_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12920_e18547_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12920_e18547_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12920_e18547_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12920_e18547_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12920_e18547_d_n17;

        let (assign12930_e18556, assign12930_e18556_d_n0, assign12930_e18556_d_n2, assign12930_e18556_d_n6, assign12930_e18556_d_n7, assign12930_e18556_d_n10, assign12930_e18556_d_n11, assign12930_e18556_d_n12, assign12930_e18556_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard411 != 0.0)) {
        (locals.var_phi_sl_bulk_ini, locals.var_phi_sl_bulk_ini_dn0, locals.var_phi_sl_bulk_ini_dn2, locals.var_phi_sl_bulk_ini_dn6, locals.var_phi_sl_bulk_ini_dn7, locals.var_phi_sl_bulk_ini_dn10, locals.var_phi_sl_bulk_ini_dn11, locals.var_phi_sl_bulk_ini_dn12, locals.var_phi_sl_bulk_ini_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12930_e18556;
        locals.var_phi_sl_bulk_dn0 = assign12930_e18556_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12930_e18556_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12930_e18556_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12930_e18556_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12930_e18556_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12930_e18556_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12930_e18556_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12930_e18556_d_n17;

        let (assign12940_e18563, assign12940_e18563_d_n0, assign12940_e18563_d_n2, assign12940_e18563_d_n6, assign12940_e18563_d_n7, assign12940_e18563_d_n10, assign12940_e18563_d_n11, assign12940_e18563_d_n12, assign12940_e18563_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12940_e18563;
        locals.var_psl_dn0 = assign12940_e18563_d_n0;
        locals.var_psl_dn2 = assign12940_e18563_d_n2;
        locals.var_psl_dn6 = assign12940_e18563_d_n6;
        locals.var_psl_dn7 = assign12940_e18563_d_n7;
        locals.var_psl_dn10 = assign12940_e18563_d_n10;
        locals.var_psl_dn11 = assign12940_e18563_d_n11;
        locals.var_psl_dn12 = assign12940_e18563_d_n12;
        locals.var_psl_dn17 = assign12940_e18563_d_n17;

        let (assign12960_e18577, assign12960_e18577_d_n0, assign12960_e18577_d_n2, assign12960_e18577_d_n6, assign12960_e18577_d_n7, assign12960_e18577_d_n10, assign12960_e18577_d_n11, assign12960_e18577_d_n12, assign12960_e18577_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign12960_e18577;
        locals.var_vds_dn0 = assign12960_e18577_d_n0;
        locals.var_vds_dn2 = assign12960_e18577_d_n2;
        locals.var_vds_dn6 = assign12960_e18577_d_n6;
        locals.var_vds_dn7 = assign12960_e18577_d_n7;
        locals.var_vds_dn10 = assign12960_e18577_d_n10;
        locals.var_vds_dn11 = assign12960_e18577_d_n11;
        locals.var_vds_dn12 = assign12960_e18577_d_n12;
        locals.var_vds_dn17 = assign12960_e18577_d_n17;

        let assign12970_e18580: f64 = if locals.var_phi_s0_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign12970_e18580;

        let (assign12980_e18589,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard412 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign12980_e18589;

        let (assign12990_e18596, assign12990_e18596_d_n0, assign12990_e18596_d_n2, assign12990_e18596_d_n6, assign12990_e18596_d_n7, assign12990_e18596_d_n10, assign12990_e18596_d_n11, assign12990_e18596_d_n12, assign12990_e18596_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0s, locals.var_ps0s_dn0, locals.var_ps0s_dn2, locals.var_ps0s_dn6, locals.var_ps0s_dn7, locals.var_ps0s_dn10, locals.var_ps0s_dn11, locals.var_ps0s_dn12, locals.var_ps0s_dn17,)
    }
};
        locals.var_ps0s = assign12990_e18596;
        locals.var_ps0s_dn0 = assign12990_e18596_d_n0;
        locals.var_ps0s_dn2 = assign12990_e18596_d_n2;
        locals.var_ps0s_dn6 = assign12990_e18596_d_n6;
        locals.var_ps0s_dn7 = assign12990_e18596_d_n7;
        locals.var_ps0s_dn10 = assign12990_e18596_d_n10;
        locals.var_ps0s_dn11 = assign12990_e18596_d_n11;
        locals.var_ps0s_dn12 = assign12990_e18596_d_n12;
        locals.var_ps0s_dn17 = assign12990_e18596_d_n17;

        let (assign13000_e18603, assign13000_e18603_d_n0, assign13000_e18603_d_n2, assign13000_e18603_d_n6, assign13000_e18603_d_n7, assign13000_e18603_d_n10, assign13000_e18603_d_n11, assign13000_e18603_d_n12, assign13000_e18603_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psls, locals.var_psls_dn0, locals.var_psls_dn2, locals.var_psls_dn6, locals.var_psls_dn7, locals.var_psls_dn10, locals.var_psls_dn11, locals.var_psls_dn12, locals.var_psls_dn17,)
    }
};
        locals.var_psls = assign13000_e18603;
        locals.var_psls_dn0 = assign13000_e18603_d_n0;
        locals.var_psls_dn2 = assign13000_e18603_d_n2;
        locals.var_psls_dn6 = assign13000_e18603_d_n6;
        locals.var_psls_dn7 = assign13000_e18603_d_n7;
        locals.var_psls_dn10 = assign13000_e18603_d_n10;
        locals.var_psls_dn11 = assign13000_e18603_d_n11;
        locals.var_psls_dn12 = assign13000_e18603_d_n12;
        locals.var_psls_dn17 = assign13000_e18603_d_n17;

        let (assign13010_e18612, assign13010_e18612_d_n0, assign13010_e18612_d_n2, assign13010_e18612_d_n6, assign13010_e18612_d_n7, assign13010_e18612_d_n10, assign13010_e18612_d_n11, assign13010_e18612_d_n12, assign13010_e18612_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13010_e18610: f64 = (locals.var_psls - locals.var_ps0s);
        (assign13010_e18610, (locals.var_psls_dn0 - locals.var_ps0s_dn0), (locals.var_psls_dn2 - locals.var_ps0s_dn2), (locals.var_psls_dn6 - locals.var_ps0s_dn6), (locals.var_psls_dn7 - locals.var_ps0s_dn7), (locals.var_psls_dn10 - locals.var_ps0s_dn10), (locals.var_psls_dn11 - locals.var_ps0s_dn11), (locals.var_psls_dn12 - locals.var_ps0s_dn12), (locals.var_psls_dn17 - locals.var_ps0s_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign13010_e18612;
        locals.var_pds_dn0 = assign13010_e18612_d_n0;
        locals.var_pds_dn2 = assign13010_e18612_d_n2;
        locals.var_pds_dn6 = assign13010_e18612_d_n6;
        locals.var_pds_dn7 = assign13010_e18612_d_n7;
        locals.var_pds_dn10 = assign13010_e18612_d_n10;
        locals.var_pds_dn11 = assign13010_e18612_d_n11;
        locals.var_pds_dn12 = assign13010_e18612_d_n12;
        locals.var_pds_dn17 = assign13010_e18612_d_n17;

        let (assign13020_e18619, assign13020_e18619_d_n0, assign13020_e18619_d_n2, assign13020_e18619_d_n6, assign13020_e18619_d_n7, assign13020_e18619_d_n10, assign13020_e18619_d_n11, assign13020_e18619_d_n12, assign13020_e18619_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_ps0b, locals.var_ps0b_dn0, locals.var_ps0b_dn2, locals.var_ps0b_dn6, locals.var_ps0b_dn7, locals.var_ps0b_dn10, locals.var_ps0b_dn11, locals.var_ps0b_dn12, locals.var_ps0b_dn17,)
    }
};
        locals.var_ps0b = assign13020_e18619;
        locals.var_ps0b_dn0 = assign13020_e18619_d_n0;
        locals.var_ps0b_dn2 = assign13020_e18619_d_n2;
        locals.var_ps0b_dn6 = assign13020_e18619_d_n6;
        locals.var_ps0b_dn7 = assign13020_e18619_d_n7;
        locals.var_ps0b_dn10 = assign13020_e18619_d_n10;
        locals.var_ps0b_dn11 = assign13020_e18619_d_n11;
        locals.var_ps0b_dn12 = assign13020_e18619_d_n12;
        locals.var_ps0b_dn17 = assign13020_e18619_d_n17;

        let (assign13030_e18628, assign13030_e18628_d_n0, assign13030_e18628_d_n2, assign13030_e18628_d_n6, assign13030_e18628_d_n7, assign13030_e18628_d_n10, assign13030_e18628_d_n11, assign13030_e18628_d_n12, assign13030_e18628_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13030_e18626: f64 = (locals.var_wdsoi / 1.034943e-10);
        (assign13030_e18626, (locals.var_wdsoi_dn0 / 1.034943e-10), (locals.var_wdsoi_dn2 / 1.034943e-10), (locals.var_wdsoi_dn6 / 1.034943e-10), (locals.var_wdsoi_dn7 / 1.034943e-10), (locals.var_wdsoi_dn10 / 1.034943e-10), (locals.var_wdsoi_dn11 / 1.034943e-10), (locals.var_wdsoi_dn12 / 1.034943e-10), (locals.var_wdsoi_dn17 / 1.034943e-10),)
    } else {
        (locals.var_c_s_inv, locals.var_c_s_inv_dn0, locals.var_c_s_inv_dn2, locals.var_c_s_inv_dn6, locals.var_c_s_inv_dn7, locals.var_c_s_inv_dn10, locals.var_c_s_inv_dn11, locals.var_c_s_inv_dn12, locals.var_c_s_inv_dn17,)
    }
};
        locals.var_c_s_inv = assign13030_e18628;
        locals.var_c_s_inv_dn0 = assign13030_e18628_d_n0;
        locals.var_c_s_inv_dn2 = assign13030_e18628_d_n2;
        locals.var_c_s_inv_dn6 = assign13030_e18628_d_n6;
        locals.var_c_s_inv_dn7 = assign13030_e18628_d_n7;
        locals.var_c_s_inv_dn10 = assign13030_e18628_d_n10;
        locals.var_c_s_inv_dn11 = assign13030_e18628_d_n11;
        locals.var_c_s_inv_dn12 = assign13030_e18628_d_n12;
        locals.var_c_s_inv_dn17 = assign13030_e18628_d_n17;

        let (assign13040_e18649, assign13040_e18649_d_n0, assign13040_e18649_d_n2, assign13040_e18649_d_n6, assign13040_e18649_d_n7, assign13040_e18649_d_n10, assign13040_e18649_d_n11, assign13040_e18649_d_n12, assign13040_e18649_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13040_e18635: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign13040_e18639: f64 = (locals.var_q_nl + locals.var_q_n0);
        let assign13040_e18640: f64 = (locals.var_beta * assign13040_e18639);
        let assign13040_e18643: f64 = (locals.var_psls - locals.var_ps0s);
        let assign13040_e18644: f64 = (assign13040_e18640 * assign13040_e18643);
        let assign13040_e18646: f64 = (assign13040_e18644 * 0.5);
        let assign13040_e18647: f64 = (assign13040_e18635 - assign13040_e18646);
        (assign13040_e18647, ((locals.var_q_nl_dn0 - locals.var_q_n0_dn0) - ((((locals.var_beta * (locals.var_q_nl_dn0 + locals.var_q_n0_dn0)) * assign13040_e18643) + (assign13040_e18640 * (locals.var_psls_dn0 - locals.var_ps0s_dn0))) * 0.5)), ((locals.var_q_nl_dn2 - locals.var_q_n0_dn2) - ((((locals.var_beta * (locals.var_q_nl_dn2 + locals.var_q_n0_dn2)) * assign13040_e18643) + (assign13040_e18640 * (locals.var_psls_dn2 - locals.var_ps0s_dn2))) * 0.5)), ((locals.var_q_nl_dn6 - locals.var_q_n0_dn6) - ((((locals.var_beta * (locals.var_q_nl_dn6 + locals.var_q_n0_dn6)) * assign13040_e18643) + (assign13040_e18640 * (locals.var_psls_dn6 - locals.var_ps0s_dn6))) * 0.5)), ((locals.var_q_nl_dn7 - locals.var_q_n0_dn7) - ((((locals.var_beta * (locals.var_q_nl_dn7 + locals.var_q_n0_dn7)) * assign13040_e18643) + (assign13040_e18640 * (locals.var_psls_dn7 - locals.var_ps0s_dn7))) * 0.5)), ((locals.var_q_nl_dn10 - locals.var_q_n0_dn10) - (((((locals.var_beta_dn10 * assign13040_e18639) + (locals.var_beta * (locals.var_q_nl_dn10 + locals.var_q_n0_dn10))) * assign13040_e18643) + (assign13040_e18640 * (locals.var_psls_dn10 - locals.var_ps0s_dn10))) * 0.5)), ((locals.var_q_nl_dn11 - locals.var_q_n0_dn11) - ((((locals.var_beta * (locals.var_q_nl_dn11 + locals.var_q_n0_dn11)) * assign13040_e18643) + (assign13040_e18640 * (locals.var_psls_dn11 - locals.var_ps0s_dn11))) * 0.5)), ((locals.var_q_nl_dn12 - locals.var_q_n0_dn12) - ((((locals.var_beta * (locals.var_q_nl_dn12 + locals.var_q_n0_dn12)) * assign13040_e18643) + (assign13040_e18640 * (locals.var_psls_dn12 - locals.var_ps0s_dn12))) * 0.5)), ((locals.var_q_nl_dn17 - locals.var_q_n0_dn17) - ((((locals.var_beta * (locals.var_q_nl_dn17 + locals.var_q_n0_dn17)) * assign13040_e18643) + (assign13040_e18640 * (locals.var_psls_dn17 - locals.var_ps0s_dn17))) * 0.5)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign13040_e18649;
        locals.var_idd_dn0 = assign13040_e18649_d_n0;
        locals.var_idd_dn2 = assign13040_e18649_d_n2;
        locals.var_idd_dn6 = assign13040_e18649_d_n6;
        locals.var_idd_dn7 = assign13040_e18649_d_n7;
        locals.var_idd_dn10 = assign13040_e18649_d_n10;
        locals.var_idd_dn11 = assign13040_e18649_d_n11;
        locals.var_idd_dn12 = assign13040_e18649_d_n12;
        locals.var_idd_dn17 = assign13040_e18649_d_n17;

        let assign13050_e18656: f64 = if ((locals.var_idd < 0.0) || (locals.var_vds == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard413 = assign13050_e18656;

        let (assign13060_e18665, assign13060_e18665_d_n0, assign13060_e18665_d_n2, assign13060_e18665_d_n6, assign13060_e18665_d_n7, assign13060_e18665_d_n10, assign13060_e18665_d_n11, assign13060_e18665_d_n12, assign13060_e18665_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard413 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign13060_e18665;
        locals.var_idd_dn0 = assign13060_e18665_d_n0;
        locals.var_idd_dn2 = assign13060_e18665_d_n2;
        locals.var_idd_dn6 = assign13060_e18665_d_n6;
        locals.var_idd_dn7 = assign13060_e18665_d_n7;
        locals.var_idd_dn10 = assign13060_e18665_d_n10;
        locals.var_idd_dn11 = assign13060_e18665_d_n11;
        locals.var_idd_dn12 = assign13060_e18665_d_n12;
        locals.var_idd_dn17 = assign13060_e18665_d_n17;

        let (assign13070_e18677, assign13070_e18677_d_n0, assign13070_e18677_d_n2, assign13070_e18677_d_n6, assign13070_e18677_d_n7, assign13070_e18677_d_n10, assign13070_e18677_d_n11, assign13070_e18677_d_n12, assign13070_e18677_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13070_e18671: f64 = (-0.5);
        let assign13070_e18674: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign13070_e18675: f64 = (assign13070_e18671 * assign13070_e18674);
        (assign13070_e18675, (assign13070_e18671 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (assign13070_e18671 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (assign13070_e18671 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (assign13070_e18671 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (assign13070_e18671 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (assign13070_e18671 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (assign13070_e18671 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (assign13070_e18671 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign13070_e18677;
        locals.var_qbu_dn0 = assign13070_e18677_d_n0;
        locals.var_qbu_dn2 = assign13070_e18677_d_n2;
        locals.var_qbu_dn6 = assign13070_e18677_d_n6;
        locals.var_qbu_dn7 = assign13070_e18677_d_n7;
        locals.var_qbu_dn10 = assign13070_e18677_d_n10;
        locals.var_qbu_dn11 = assign13070_e18677_d_n11;
        locals.var_qbu_dn12 = assign13070_e18677_d_n12;
        locals.var_qbu_dn17 = assign13070_e18677_d_n17;

        let (assign13080_e18686, assign13080_e18686_d_n0, assign13080_e18686_d_n2, assign13080_e18686_d_n6, assign13080_e18686_d_n7, assign13080_e18686_d_n10, assign13080_e18686_d_n11, assign13080_e18686_d_n12, assign13080_e18686_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13080_e18684: f64 = (locals.var_phi_sl_soi - locals.var_phi_s0_soi);
        (assign13080_e18684, (locals.var_phi_sl_soi_dn0 - locals.var_phi_s0_soi_dn0), (locals.var_phi_sl_soi_dn2 - locals.var_phi_s0_soi_dn2), (locals.var_phi_sl_soi_dn6 - locals.var_phi_s0_soi_dn6), (locals.var_phi_sl_soi_dn7 - locals.var_phi_s0_soi_dn7), (locals.var_phi_sl_soi_dn10 - locals.var_phi_s0_soi_dn10), (locals.var_phi_sl_soi_dn11 - locals.var_phi_s0_soi_dn11), (locals.var_phi_sl_soi_dn12 - locals.var_phi_s0_soi_dn12), (locals.var_phi_sl_soi_dn17 - locals.var_phi_s0_soi_dn17),)
    } else {
        (locals.var_rrr_p0, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    }
};
        locals.var_rrr_p0 = assign13080_e18686;
        locals.var_rrr_p0_dn0 = assign13080_e18686_d_n0;
        locals.var_rrr_p0_dn2 = assign13080_e18686_d_n2;
        locals.var_rrr_p0_dn6 = assign13080_e18686_d_n6;
        locals.var_rrr_p0_dn7 = assign13080_e18686_d_n7;
        locals.var_rrr_p0_dn10 = assign13080_e18686_d_n10;
        locals.var_rrr_p0_dn11 = assign13080_e18686_d_n11;
        locals.var_rrr_p0_dn12 = assign13080_e18686_d_n12;
        locals.var_rrr_p0_dn17 = assign13080_e18686_d_n17;

        let (assign13090_e18695, assign13090_e18695_d_n0, assign13090_e18695_d_n2, assign13090_e18695_d_n6, assign13090_e18695_d_n7, assign13090_e18695_d_n10, assign13090_e18695_d_n11, assign13090_e18695_d_n12, assign13090_e18695_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13090_e18693: f64 = (locals.var_rrr_p0 + 5e-12);
        (assign13090_e18693, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    } else {
        (locals.var_rrr_p0, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    }
};
        locals.var_rrr_p0 = assign13090_e18695;
        locals.var_rrr_p0_dn0 = assign13090_e18695_d_n0;
        locals.var_rrr_p0_dn2 = assign13090_e18695_d_n2;
        locals.var_rrr_p0_dn6 = assign13090_e18695_d_n6;
        locals.var_rrr_p0_dn7 = assign13090_e18695_d_n7;
        locals.var_rrr_p0_dn10 = assign13090_e18695_d_n10;
        locals.var_rrr_p0_dn11 = assign13090_e18695_d_n11;
        locals.var_rrr_p0_dn12 = assign13090_e18695_d_n12;
        locals.var_rrr_p0_dn17 = assign13090_e18695_d_n17;

        let (assign13100_e18708, assign13100_e18708_d_n0, assign13100_e18708_d_n2, assign13100_e18708_d_n6, assign13100_e18708_d_n7, assign13100_e18708_d_n10, assign13100_e18708_d_n11, assign13100_e18708_d_n12, assign13100_e18708_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13100_e18703: f64 = (locals.var_c_box * locals.var_c_s_inv);
        let assign13100_e18705: f64 = (assign13100_e18703 + 1.0);
        let assign13100_e18706: f64 = (locals.var_c_box / assign13100_e18705);
        (assign13100_e18706, (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn0)) / (assign13100_e18705 * assign13100_e18705))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn2)) / (assign13100_e18705 * assign13100_e18705))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn6)) / (assign13100_e18705 * assign13100_e18705))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn7)) / (assign13100_e18705 * assign13100_e18705))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn10)) / (assign13100_e18705 * assign13100_e18705))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn11)) / (assign13100_e18705 * assign13100_e18705))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn12)) / (assign13100_e18705 * assign13100_e18705))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn17)) / (assign13100_e18705 * assign13100_e18705))),)
    } else {
        (locals.var_rrr_csoi_cbox, locals.var_rrr_csoi_cbox_dn0, locals.var_rrr_csoi_cbox_dn2, locals.var_rrr_csoi_cbox_dn6, locals.var_rrr_csoi_cbox_dn7, locals.var_rrr_csoi_cbox_dn10, locals.var_rrr_csoi_cbox_dn11, locals.var_rrr_csoi_cbox_dn12, locals.var_rrr_csoi_cbox_dn17,)
    }
};
        locals.var_rrr_csoi_cbox = assign13100_e18708;
        locals.var_rrr_csoi_cbox_dn0 = assign13100_e18708_d_n0;
        locals.var_rrr_csoi_cbox_dn2 = assign13100_e18708_d_n2;
        locals.var_rrr_csoi_cbox_dn6 = assign13100_e18708_d_n6;
        locals.var_rrr_csoi_cbox_dn7 = assign13100_e18708_d_n7;
        locals.var_rrr_csoi_cbox_dn10 = assign13100_e18708_d_n10;
        locals.var_rrr_csoi_cbox_dn11 = assign13100_e18708_d_n11;
        locals.var_rrr_csoi_cbox_dn12 = assign13100_e18708_d_n12;
        locals.var_rrr_csoi_cbox_dn17 = assign13100_e18708_d_n17;

        let (assign13110_e18723, assign13110_e18723_d_n0, assign13110_e18723_d_n2, assign13110_e18723_d_n6, assign13110_e18723_d_n7, assign13110_e18723_d_n10, assign13110_e18723_d_n11, assign13110_e18723_d_n12, assign13110_e18723_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13110_e18715: f64 = (locals.var_q_sl_bulk * locals.var_q_sl_bulk);
        let assign13110_e18718: f64 = (locals.var_q_s0_bulk * locals.var_q_s0_bulk);
        let assign13110_e18719: f64 = (assign13110_e18715 - assign13110_e18718);
        let assign13110_e18721: f64 = (assign13110_e18719 / locals.var_rrr_csoi_cbox);
        (assign13110_e18721, ((((((locals.var_q_sl_bulk_dn0 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn0)) - ((locals.var_q_s0_bulk_dn0 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn0))) * locals.var_rrr_csoi_cbox) - (assign13110_e18719 * locals.var_rrr_csoi_cbox_dn0)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn2 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn2)) - ((locals.var_q_s0_bulk_dn2 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn2))) * locals.var_rrr_csoi_cbox) - (assign13110_e18719 * locals.var_rrr_csoi_cbox_dn2)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn6 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn6)) - ((locals.var_q_s0_bulk_dn6 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn6))) * locals.var_rrr_csoi_cbox) - (assign13110_e18719 * locals.var_rrr_csoi_cbox_dn6)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn7 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn7)) - ((locals.var_q_s0_bulk_dn7 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn7))) * locals.var_rrr_csoi_cbox) - (assign13110_e18719 * locals.var_rrr_csoi_cbox_dn7)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn10 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn10)) - ((locals.var_q_s0_bulk_dn10 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn10))) * locals.var_rrr_csoi_cbox) - (assign13110_e18719 * locals.var_rrr_csoi_cbox_dn10)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn11 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn11)) - ((locals.var_q_s0_bulk_dn11 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn11))) * locals.var_rrr_csoi_cbox) - (assign13110_e18719 * locals.var_rrr_csoi_cbox_dn11)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn12 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn12)) - ((locals.var_q_s0_bulk_dn12 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn12))) * locals.var_rrr_csoi_cbox) - (assign13110_e18719 * locals.var_rrr_csoi_cbox_dn12)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn17 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn17)) - ((locals.var_q_s0_bulk_dn17 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn17))) * locals.var_rrr_csoi_cbox) - (assign13110_e18719 * locals.var_rrr_csoi_cbox_dn17)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)),)
    } else {
        (locals.var_rrr_b, locals.var_rrr_b_dn0, locals.var_rrr_b_dn2, locals.var_rrr_b_dn6, locals.var_rrr_b_dn7, locals.var_rrr_b_dn10, locals.var_rrr_b_dn11, locals.var_rrr_b_dn12, locals.var_rrr_b_dn17,)
    }
};
        locals.var_rrr_b = assign13110_e18723;
        locals.var_rrr_b_dn0 = assign13110_e18723_d_n0;
        locals.var_rrr_b_dn2 = assign13110_e18723_d_n2;
        locals.var_rrr_b_dn6 = assign13110_e18723_d_n6;
        locals.var_rrr_b_dn7 = assign13110_e18723_d_n7;
        locals.var_rrr_b_dn10 = assign13110_e18723_d_n10;
        locals.var_rrr_b_dn11 = assign13110_e18723_d_n11;
        locals.var_rrr_b_dn12 = assign13110_e18723_d_n12;
        locals.var_rrr_b_dn17 = assign13110_e18723_d_n17;

        let assign13120_e18725: f64 = (-locals.var_rrr_b);
        let assign13120_e18729: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13120_e18730: f64 = assign13120_e18729;
        let assign13120_e18734: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13120_e18737: f64 = if ((assign13120_e18725 < assign13120_e18730) && (assign13120_e18734 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard414 = assign13120_e18737;

        let (assign13130_e18753, assign13130_e18753_d_n0, assign13130_e18753_d_n2, assign13130_e18753_d_n6, assign13130_e18753_d_n7, assign13130_e18753_d_n10, assign13130_e18753_d_n11, assign13130_e18753_d_n12, assign13130_e18753_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13130_e18747: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13130_e18748: f64 = assign13130_e18747;
        let assign13130_e18750: f64 = (-locals.var_rrr_b);
        let assign13130_e18751: f64 = (assign13130_e18748 - assign13130_e18750);
        (assign13130_e18751, ((locals.var_q_fd_soi_dn0 * 1e-5) - (-locals.var_rrr_b_dn0)), ((locals.var_q_fd_soi_dn2 * 1e-5) - (-locals.var_rrr_b_dn2)), ((locals.var_q_fd_soi_dn6 * 1e-5) - (-locals.var_rrr_b_dn6)), ((locals.var_q_fd_soi_dn7 * 1e-5) - (-locals.var_rrr_b_dn7)), ((locals.var_q_fd_soi_dn10 * 1e-5) - (-locals.var_rrr_b_dn10)), ((locals.var_q_fd_soi_dn11 * 1e-5) - (-locals.var_rrr_b_dn11)), ((locals.var_q_fd_soi_dn12 * 1e-5) - (-locals.var_rrr_b_dn12)), ((locals.var_q_fd_soi_dn17 * 1e-5) - (-locals.var_rrr_b_dn17)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13130_e18753;
        locals.var_tmf1_dn0 = assign13130_e18753_d_n0;
        locals.var_tmf1_dn2 = assign13130_e18753_d_n2;
        locals.var_tmf1_dn6 = assign13130_e18753_d_n6;
        locals.var_tmf1_dn7 = assign13130_e18753_d_n7;
        locals.var_tmf1_dn10 = assign13130_e18753_d_n10;
        locals.var_tmf1_dn11 = assign13130_e18753_d_n11;
        locals.var_tmf1_dn12 = assign13130_e18753_d_n12;
        locals.var_tmf1_dn17 = assign13130_e18753_d_n17;

        let (assign13140_e18764, assign13140_e18764_d_n0, assign13140_e18764_d_n2, assign13140_e18764_d_n6, assign13140_e18764_d_n7, assign13140_e18764_d_n10, assign13140_e18764_d_n11, assign13140_e18764_d_n12, assign13140_e18764_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13140_e18762: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13140_e18762, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13140_e18764;
        locals.var_x2_dn0 = assign13140_e18764_d_n0;
        locals.var_x2_dn2 = assign13140_e18764_d_n2;
        locals.var_x2_dn6 = assign13140_e18764_d_n6;
        locals.var_x2_dn7 = assign13140_e18764_d_n7;
        locals.var_x2_dn10 = assign13140_e18764_d_n10;
        locals.var_x2_dn11 = assign13140_e18764_d_n11;
        locals.var_x2_dn12 = assign13140_e18764_d_n12;
        locals.var_x2_dn17 = assign13140_e18764_d_n17;

        let (assign13150_e18779, assign13150_e18779_d_n0, assign13150_e18779_d_n2, assign13150_e18779_d_n6, assign13150_e18779_d_n7, assign13150_e18779_d_n10, assign13150_e18779_d_n11, assign13150_e18779_d_n12, assign13150_e18779_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13150_e18773: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13150_e18776: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13150_e18777: f64 = (assign13150_e18773 * assign13150_e18776);
        (assign13150_e18777, (((locals.var_q_fd_soi_dn0 * 1e-5) * assign13150_e18776) + (assign13150_e18773 * (locals.var_q_fd_soi_dn0 * 1e-5))), (((locals.var_q_fd_soi_dn2 * 1e-5) * assign13150_e18776) + (assign13150_e18773 * (locals.var_q_fd_soi_dn2 * 1e-5))), (((locals.var_q_fd_soi_dn6 * 1e-5) * assign13150_e18776) + (assign13150_e18773 * (locals.var_q_fd_soi_dn6 * 1e-5))), (((locals.var_q_fd_soi_dn7 * 1e-5) * assign13150_e18776) + (assign13150_e18773 * (locals.var_q_fd_soi_dn7 * 1e-5))), (((locals.var_q_fd_soi_dn10 * 1e-5) * assign13150_e18776) + (assign13150_e18773 * (locals.var_q_fd_soi_dn10 * 1e-5))), (((locals.var_q_fd_soi_dn11 * 1e-5) * assign13150_e18776) + (assign13150_e18773 * (locals.var_q_fd_soi_dn11 * 1e-5))), (((locals.var_q_fd_soi_dn12 * 1e-5) * assign13150_e18776) + (assign13150_e18773 * (locals.var_q_fd_soi_dn12 * 1e-5))), (((locals.var_q_fd_soi_dn17 * 1e-5) * assign13150_e18776) + (assign13150_e18773 * (locals.var_q_fd_soi_dn17 * 1e-5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13150_e18779;
        locals.var_xmax2_dn0 = assign13150_e18779_d_n0;
        locals.var_xmax2_dn2 = assign13150_e18779_d_n2;
        locals.var_xmax2_dn6 = assign13150_e18779_d_n6;
        locals.var_xmax2_dn7 = assign13150_e18779_d_n7;
        locals.var_xmax2_dn10 = assign13150_e18779_d_n10;
        locals.var_xmax2_dn11 = assign13150_e18779_d_n11;
        locals.var_xmax2_dn12 = assign13150_e18779_d_n12;
        locals.var_xmax2_dn17 = assign13150_e18779_d_n17;

        let (assign13160_e18788, assign13160_e18788_d_n0, assign13160_e18788_d_n2, assign13160_e18788_d_n6, assign13160_e18788_d_n7, assign13160_e18788_d_n10, assign13160_e18788_d_n11, assign13160_e18788_d_n12, assign13160_e18788_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13160_e18788;
        locals.var_xp_dn0 = assign13160_e18788_d_n0;
        locals.var_xp_dn2 = assign13160_e18788_d_n2;
        locals.var_xp_dn6 = assign13160_e18788_d_n6;
        locals.var_xp_dn7 = assign13160_e18788_d_n7;
        locals.var_xp_dn10 = assign13160_e18788_d_n10;
        locals.var_xp_dn11 = assign13160_e18788_d_n11;
        locals.var_xp_dn12 = assign13160_e18788_d_n12;
        locals.var_xp_dn17 = assign13160_e18788_d_n17;

        let (assign13170_e18797, assign13170_e18797_d_n0, assign13170_e18797_d_n2, assign13170_e18797_d_n6, assign13170_e18797_d_n7, assign13170_e18797_d_n10, assign13170_e18797_d_n11, assign13170_e18797_d_n12, assign13170_e18797_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13170_e18797;
        locals.var_xmp_dn0 = assign13170_e18797_d_n0;
        locals.var_xmp_dn2 = assign13170_e18797_d_n2;
        locals.var_xmp_dn6 = assign13170_e18797_d_n6;
        locals.var_xmp_dn7 = assign13170_e18797_d_n7;
        locals.var_xmp_dn10 = assign13170_e18797_d_n10;
        locals.var_xmp_dn11 = assign13170_e18797_d_n11;
        locals.var_xmp_dn12 = assign13170_e18797_d_n12;
        locals.var_xmp_dn17 = assign13170_e18797_d_n17;

        let (assign13180_e18806,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13180_e18806;

        let (assign13190_e18815,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13190_e18815;

        let (assign13200_e18824, assign13200_e18824_d_n0, assign13200_e18824_d_n2, assign13200_e18824_d_n6, assign13200_e18824_d_n7, assign13200_e18824_d_n10, assign13200_e18824_d_n11, assign13200_e18824_d_n12, assign13200_e18824_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13200_e18824;
        locals.var_arg_dn0 = assign13200_e18824_d_n0;
        locals.var_arg_dn2 = assign13200_e18824_d_n2;
        locals.var_arg_dn6 = assign13200_e18824_d_n6;
        locals.var_arg_dn7 = assign13200_e18824_d_n7;
        locals.var_arg_dn10 = assign13200_e18824_d_n10;
        locals.var_arg_dn11 = assign13200_e18824_d_n11;
        locals.var_arg_dn12 = assign13200_e18824_d_n12;
        locals.var_arg_dn17 = assign13200_e18824_d_n17;

        let (assign13210_e18833, assign13210_e18833_d_n0, assign13210_e18833_d_n2, assign13210_e18833_d_n6, assign13210_e18833_d_n7, assign13210_e18833_d_n10, assign13210_e18833_d_n11, assign13210_e18833_d_n12, assign13210_e18833_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13210_e18833;
        locals.var_dnm_dn0 = assign13210_e18833_d_n0;
        locals.var_dnm_dn2 = assign13210_e18833_d_n2;
        locals.var_dnm_dn6 = assign13210_e18833_d_n6;
        locals.var_dnm_dn7 = assign13210_e18833_d_n7;
        locals.var_dnm_dn10 = assign13210_e18833_d_n10;
        locals.var_dnm_dn11 = assign13210_e18833_d_n11;
        locals.var_dnm_dn12 = assign13210_e18833_d_n12;
        locals.var_dnm_dn17 = assign13210_e18833_d_n17;

    }

    pub(super) fn stamp_transient_block_43(
        locals: &mut StampLocals,
    ) {
        let (assign13220_e18844, assign13220_e18844_d_n0, assign13220_e18844_d_n2, assign13220_e18844_d_n6, assign13220_e18844_d_n7, assign13220_e18844_d_n10, assign13220_e18844_d_n11, assign13220_e18844_d_n12, assign13220_e18844_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13220_e18842: f64 = (locals.var_xp * locals.var_x2);
        (assign13220_e18842, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13220_e18844;
        locals.var_xp_dn0 = assign13220_e18844_d_n0;
        locals.var_xp_dn2 = assign13220_e18844_d_n2;
        locals.var_xp_dn6 = assign13220_e18844_d_n6;
        locals.var_xp_dn7 = assign13220_e18844_d_n7;
        locals.var_xp_dn10 = assign13220_e18844_d_n10;
        locals.var_xp_dn11 = assign13220_e18844_d_n11;
        locals.var_xp_dn12 = assign13220_e18844_d_n12;
        locals.var_xp_dn17 = assign13220_e18844_d_n17;

        let (assign13230_e18855, assign13230_e18855_d_n0, assign13230_e18855_d_n2, assign13230_e18855_d_n6, assign13230_e18855_d_n7, assign13230_e18855_d_n10, assign13230_e18855_d_n11, assign13230_e18855_d_n12, assign13230_e18855_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13230_e18853: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13230_e18853, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13230_e18855;
        locals.var_xmp_dn0 = assign13230_e18855_d_n0;
        locals.var_xmp_dn2 = assign13230_e18855_d_n2;
        locals.var_xmp_dn6 = assign13230_e18855_d_n6;
        locals.var_xmp_dn7 = assign13230_e18855_d_n7;
        locals.var_xmp_dn10 = assign13230_e18855_d_n10;
        locals.var_xmp_dn11 = assign13230_e18855_d_n11;
        locals.var_xmp_dn12 = assign13230_e18855_d_n12;
        locals.var_xmp_dn17 = assign13230_e18855_d_n17;

        let (assign13240_e18866, assign13240_e18866_d_n0, assign13240_e18866_d_n2, assign13240_e18866_d_n6, assign13240_e18866_d_n7, assign13240_e18866_d_n10, assign13240_e18866_d_n11, assign13240_e18866_d_n12, assign13240_e18866_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13240_e18864: f64 = (locals.var_xp * locals.var_x2);
        (assign13240_e18864, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13240_e18866;
        locals.var_xp_dn0 = assign13240_e18866_d_n0;
        locals.var_xp_dn2 = assign13240_e18866_d_n2;
        locals.var_xp_dn6 = assign13240_e18866_d_n6;
        locals.var_xp_dn7 = assign13240_e18866_d_n7;
        locals.var_xp_dn10 = assign13240_e18866_d_n10;
        locals.var_xp_dn11 = assign13240_e18866_d_n11;
        locals.var_xp_dn12 = assign13240_e18866_d_n12;
        locals.var_xp_dn17 = assign13240_e18866_d_n17;

        let (assign13250_e18877, assign13250_e18877_d_n0, assign13250_e18877_d_n2, assign13250_e18877_d_n6, assign13250_e18877_d_n7, assign13250_e18877_d_n10, assign13250_e18877_d_n11, assign13250_e18877_d_n12, assign13250_e18877_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13250_e18875: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13250_e18875, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13250_e18877;
        locals.var_xmp_dn0 = assign13250_e18877_d_n0;
        locals.var_xmp_dn2 = assign13250_e18877_d_n2;
        locals.var_xmp_dn6 = assign13250_e18877_d_n6;
        locals.var_xmp_dn7 = assign13250_e18877_d_n7;
        locals.var_xmp_dn10 = assign13250_e18877_d_n10;
        locals.var_xmp_dn11 = assign13250_e18877_d_n11;
        locals.var_xmp_dn12 = assign13250_e18877_d_n12;
        locals.var_xmp_dn17 = assign13250_e18877_d_n17;

        let (assign13260_e18888, assign13260_e18888_d_n0, assign13260_e18888_d_n2, assign13260_e18888_d_n6, assign13260_e18888_d_n7, assign13260_e18888_d_n10, assign13260_e18888_d_n11, assign13260_e18888_d_n12, assign13260_e18888_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13260_e18886: f64 = (locals.var_xp + locals.var_xmp);
        (assign13260_e18886, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13260_e18888;
        locals.var_arg_dn0 = assign13260_e18888_d_n0;
        locals.var_arg_dn2 = assign13260_e18888_d_n2;
        locals.var_arg_dn6 = assign13260_e18888_d_n6;
        locals.var_arg_dn7 = assign13260_e18888_d_n7;
        locals.var_arg_dn10 = assign13260_e18888_d_n10;
        locals.var_arg_dn11 = assign13260_e18888_d_n11;
        locals.var_arg_dn12 = assign13260_e18888_d_n12;
        locals.var_arg_dn17 = assign13260_e18888_d_n17;

        let (assign13270_e18897, assign13270_e18897_d_n0, assign13270_e18897_d_n2, assign13270_e18897_d_n6, assign13270_e18897_d_n7, assign13270_e18897_d_n10, assign13270_e18897_d_n11, assign13270_e18897_d_n12, assign13270_e18897_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13270_e18897;
        locals.var_dnm_dn0 = assign13270_e18897_d_n0;
        locals.var_dnm_dn2 = assign13270_e18897_d_n2;
        locals.var_dnm_dn6 = assign13270_e18897_d_n6;
        locals.var_dnm_dn7 = assign13270_e18897_d_n7;
        locals.var_dnm_dn10 = assign13270_e18897_d_n10;
        locals.var_dnm_dn11 = assign13270_e18897_d_n11;
        locals.var_dnm_dn12 = assign13270_e18897_d_n12;
        locals.var_dnm_dn17 = assign13270_e18897_d_n17;

        let assign13280_e18912: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard415 = assign13280_e18912;

        let assign13290_e18915: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign13290_e18915;

        let (assign13300_e18928,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_guard416 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13300_e18928;

        let assign13310_e18931: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign13310_e18931;

        let (assign13320_e18947,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_guard416 == 0.0)) && (locals.var_guard417 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13320_e18947;

        let assign13330_e18950: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign13330_e18950;

        let (assign13340_e18969,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_guard416 == 0.0)) && (locals.var_guard417 == 0.0)) && (locals.var_guard418 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13340_e18969;

        let assign13350_e18972: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign13350_e18972;

        let (assign13360_e18994,) = {
    if ((((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_guard416 == 0.0)) && (locals.var_guard417 == 0.0)) && (locals.var_guard418 == 0.0)) && (locals.var_guard419 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13360_e18994;

        let (assign13370_e19005,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13370_e19005;

        let mut assign13380_loop_guard: usize = 0;
        while {
            let assign13380_cond_e19017: f64 = if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign13380_cond_e19017 != 0.0
        } {
            assign13380_loop_guard += 1;
            assert!(assign13380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign13380_body0_e19029, assign13380_body0_e19029_d_n0, assign13380_body0_e19029_d_n2, assign13380_body0_e19029_d_n6, assign13380_body0_e19029_d_n7, assign13380_body0_e19029_d_n10, assign13380_body0_e19029_d_n11, assign13380_body0_e19029_d_n12, assign13380_body0_e19029_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign13380_body0_e19027: f64 = (locals.var_dnm).sqrt();
        (assign13380_body0_e19027, (locals.var_dnm_dn0 / (2.0 * assign13380_body0_e19027)), (locals.var_dnm_dn2 / (2.0 * assign13380_body0_e19027)), (locals.var_dnm_dn6 / (2.0 * assign13380_body0_e19027)), (locals.var_dnm_dn7 / (2.0 * assign13380_body0_e19027)), (locals.var_dnm_dn10 / (2.0 * assign13380_body0_e19027)), (locals.var_dnm_dn11 / (2.0 * assign13380_body0_e19027)), (locals.var_dnm_dn12 / (2.0 * assign13380_body0_e19027)), (locals.var_dnm_dn17 / (2.0 * assign13380_body0_e19027)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign13380_body0_e19029;
            locals.var_dnm_dn0 = assign13380_body0_e19029_d_n0;
            locals.var_dnm_dn2 = assign13380_body0_e19029_d_n2;
            locals.var_dnm_dn6 = assign13380_body0_e19029_d_n6;
            locals.var_dnm_dn7 = assign13380_body0_e19029_d_n7;
            locals.var_dnm_dn10 = assign13380_body0_e19029_d_n10;
            locals.var_dnm_dn11 = assign13380_body0_e19029_d_n11;
            locals.var_dnm_dn12 = assign13380_body0_e19029_d_n12;
            locals.var_dnm_dn17 = assign13380_body0_e19029_d_n17;
            let (assign13380_body1_e19042,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign13380_body1_e19040: f64 = (locals.var_m0 + 1.0);
        (assign13380_body1_e19040,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign13380_body1_e19042;
        }

        let (assign13390_e19060, assign13390_e19060_d_n0, assign13390_e19060_d_n2, assign13390_e19060_d_n6, assign13390_e19060_d_n7, assign13390_e19060_d_n10, assign13390_e19060_d_n11, assign13390_e19060_d_n12, assign13390_e19060_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 == 0.0)) {
        let assign13390_e19056: f64 = (2.0 * 2.0);
        let assign13390_e19057: f64 = (1.0 / assign13390_e19056);
        let assign13390_e19058: f64 = (locals.var_dnm).powf(assign13390_e19057);
        (assign13390_e19058, if 0.0 == 0.0 && ((assign13390_e19057) as f64).is_finite() && ((assign13390_e19057) as f64).fract() == 0.0 { if assign13390_e19057 == 0.0 { 0.0 } else { (assign13390_e19057 * ((locals.var_dnm).powf(assign13390_e19057 - 1.0) * locals.var_dnm_dn0)) } } else { (assign13390_e19058 * (assign13390_e19057 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13390_e19057) as f64).is_finite() && ((assign13390_e19057) as f64).fract() == 0.0 { if assign13390_e19057 == 0.0 { 0.0 } else { (assign13390_e19057 * ((locals.var_dnm).powf(assign13390_e19057 - 1.0) * locals.var_dnm_dn2)) } } else { (assign13390_e19058 * (assign13390_e19057 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13390_e19057) as f64).is_finite() && ((assign13390_e19057) as f64).fract() == 0.0 { if assign13390_e19057 == 0.0 { 0.0 } else { (assign13390_e19057 * ((locals.var_dnm).powf(assign13390_e19057 - 1.0) * locals.var_dnm_dn6)) } } else { (assign13390_e19058 * (assign13390_e19057 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13390_e19057) as f64).is_finite() && ((assign13390_e19057) as f64).fract() == 0.0 { if assign13390_e19057 == 0.0 { 0.0 } else { (assign13390_e19057 * ((locals.var_dnm).powf(assign13390_e19057 - 1.0) * locals.var_dnm_dn7)) } } else { (assign13390_e19058 * (assign13390_e19057 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13390_e19057) as f64).is_finite() && ((assign13390_e19057) as f64).fract() == 0.0 { if assign13390_e19057 == 0.0 { 0.0 } else { (assign13390_e19057 * ((locals.var_dnm).powf(assign13390_e19057 - 1.0) * locals.var_dnm_dn10)) } } else { (assign13390_e19058 * (assign13390_e19057 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13390_e19057) as f64).is_finite() && ((assign13390_e19057) as f64).fract() == 0.0 { if assign13390_e19057 == 0.0 { 0.0 } else { (assign13390_e19057 * ((locals.var_dnm).powf(assign13390_e19057 - 1.0) * locals.var_dnm_dn11)) } } else { (assign13390_e19058 * (assign13390_e19057 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13390_e19057) as f64).is_finite() && ((assign13390_e19057) as f64).fract() == 0.0 { if assign13390_e19057 == 0.0 { 0.0 } else { (assign13390_e19057 * ((locals.var_dnm).powf(assign13390_e19057 - 1.0) * locals.var_dnm_dn12)) } } else { (assign13390_e19058 * (assign13390_e19057 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13390_e19057) as f64).is_finite() && ((assign13390_e19057) as f64).fract() == 0.0 { if assign13390_e19057 == 0.0 { 0.0 } else { (assign13390_e19057 * ((locals.var_dnm).powf(assign13390_e19057 - 1.0) * locals.var_dnm_dn17)) } } else { (assign13390_e19058 * (assign13390_e19057 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13390_e19060;
        locals.var_dnm_dn0 = assign13390_e19060_d_n0;
        locals.var_dnm_dn2 = assign13390_e19060_d_n2;
        locals.var_dnm_dn6 = assign13390_e19060_d_n6;
        locals.var_dnm_dn7 = assign13390_e19060_d_n7;
        locals.var_dnm_dn10 = assign13390_e19060_d_n10;
        locals.var_dnm_dn11 = assign13390_e19060_d_n11;
        locals.var_dnm_dn12 = assign13390_e19060_d_n12;
        locals.var_dnm_dn17 = assign13390_e19060_d_n17;

        let (assign13400_e19071, assign13400_e19071_d_n0, assign13400_e19071_d_n2, assign13400_e19071_d_n6, assign13400_e19071_d_n7, assign13400_e19071_d_n10, assign13400_e19071_d_n11, assign13400_e19071_d_n12, assign13400_e19071_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13400_e19069: f64 = (1.0 / locals.var_dnm);
        (assign13400_e19069, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13400_e19071;
        locals.var_dnm_dn0 = assign13400_e19071_d_n0;
        locals.var_dnm_dn2 = assign13400_e19071_d_n2;
        locals.var_dnm_dn6 = assign13400_e19071_d_n6;
        locals.var_dnm_dn7 = assign13400_e19071_d_n7;
        locals.var_dnm_dn10 = assign13400_e19071_d_n10;
        locals.var_dnm_dn11 = assign13400_e19071_d_n11;
        locals.var_dnm_dn12 = assign13400_e19071_d_n12;
        locals.var_dnm_dn17 = assign13400_e19071_d_n17;

        let (assign13410_e19086, assign13410_e19086_d_n0, assign13410_e19086_d_n2, assign13410_e19086_d_n6, assign13410_e19086_d_n7, assign13410_e19086_d_n10, assign13410_e19086_d_n11, assign13410_e19086_d_n12, assign13410_e19086_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13410_e19081: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13410_e19082: f64 = (locals.var_tmf1 * assign13410_e19081);
        let assign13410_e19084: f64 = (assign13410_e19082 * locals.var_dnm);
        (assign13410_e19084, ((((locals.var_tmf1_dn0 * assign13410_e19081) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn0 * 1e-5))) * locals.var_dnm) + (assign13410_e19082 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign13410_e19081) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn2 * 1e-5))) * locals.var_dnm) + (assign13410_e19082 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign13410_e19081) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn6 * 1e-5))) * locals.var_dnm) + (assign13410_e19082 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign13410_e19081) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn7 * 1e-5))) * locals.var_dnm) + (assign13410_e19082 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign13410_e19081) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn10 * 1e-5))) * locals.var_dnm) + (assign13410_e19082 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign13410_e19081) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn11 * 1e-5))) * locals.var_dnm) + (assign13410_e19082 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign13410_e19081) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn12 * 1e-5))) * locals.var_dnm) + (assign13410_e19082 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign13410_e19081) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn17 * 1e-5))) * locals.var_dnm) + (assign13410_e19082 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign13410_e19086;
        locals.var_tmf0_dn0 = assign13410_e19086_d_n0;
        locals.var_tmf0_dn2 = assign13410_e19086_d_n2;
        locals.var_tmf0_dn6 = assign13410_e19086_d_n6;
        locals.var_tmf0_dn7 = assign13410_e19086_d_n7;
        locals.var_tmf0_dn10 = assign13410_e19086_d_n10;
        locals.var_tmf0_dn11 = assign13410_e19086_d_n11;
        locals.var_tmf0_dn12 = assign13410_e19086_d_n12;
        locals.var_tmf0_dn17 = assign13410_e19086_d_n17;

        let (assign13420_e19101, assign13420_e19101_d_n0, assign13420_e19101_d_n2, assign13420_e19101_d_n6, assign13420_e19101_d_n7, assign13420_e19101_d_n10, assign13420_e19101_d_n11, assign13420_e19101_d_n12, assign13420_e19101_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 != 0.0)) {
        let assign13420_e19096: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13420_e19097: f64 = assign13420_e19096;
        let assign13420_e19099: f64 = (assign13420_e19097 - locals.var_tmf0);
        (assign13420_e19099, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_tmf0_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_tmf0_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_tmf0_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_tmf0_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_tmf0_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_tmf0_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_tmf0_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13420_e19101;
        locals.var_t1_dn0 = assign13420_e19101_d_n0;
        locals.var_t1_dn2 = assign13420_e19101_d_n2;
        locals.var_t1_dn6 = assign13420_e19101_d_n6;
        locals.var_t1_dn7 = assign13420_e19101_d_n7;
        locals.var_t1_dn10 = assign13420_e19101_d_n10;
        locals.var_t1_dn11 = assign13420_e19101_d_n11;
        locals.var_t1_dn12 = assign13420_e19101_d_n12;
        locals.var_t1_dn17 = assign13420_e19101_d_n17;

        let (assign13430_e19112, assign13430_e19112_d_n0, assign13430_e19112_d_n2, assign13430_e19112_d_n6, assign13430_e19112_d_n7, assign13430_e19112_d_n10, assign13430_e19112_d_n11, assign13430_e19112_d_n12, assign13430_e19112_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard414 == 0.0)) {
        let assign13430_e19110: f64 = (-locals.var_rrr_b);
        (assign13430_e19110, (-locals.var_rrr_b_dn0), (-locals.var_rrr_b_dn2), (-locals.var_rrr_b_dn6), (-locals.var_rrr_b_dn7), (-locals.var_rrr_b_dn10), (-locals.var_rrr_b_dn11), (-locals.var_rrr_b_dn12), (-locals.var_rrr_b_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13430_e19112;
        locals.var_t1_dn0 = assign13430_e19112_d_n0;
        locals.var_t1_dn2 = assign13430_e19112_d_n2;
        locals.var_t1_dn6 = assign13430_e19112_d_n6;
        locals.var_t1_dn7 = assign13430_e19112_d_n7;
        locals.var_t1_dn10 = assign13430_e19112_d_n10;
        locals.var_t1_dn11 = assign13430_e19112_d_n11;
        locals.var_t1_dn12 = assign13430_e19112_d_n12;
        locals.var_t1_dn17 = assign13430_e19112_d_n17;

        let (assign13440_e19120, assign13440_e19120_d_n0, assign13440_e19120_d_n2, assign13440_e19120_d_n6, assign13440_e19120_d_n7, assign13440_e19120_d_n10, assign13440_e19120_d_n11, assign13440_e19120_d_n12, assign13440_e19120_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13440_e19118: f64 = (-locals.var_t1);
        (assign13440_e19118, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn17),)
    } else {
        (locals.var_rrr_b, locals.var_rrr_b_dn0, locals.var_rrr_b_dn2, locals.var_rrr_b_dn6, locals.var_rrr_b_dn7, locals.var_rrr_b_dn10, locals.var_rrr_b_dn11, locals.var_rrr_b_dn12, locals.var_rrr_b_dn17,)
    }
};
        locals.var_rrr_b = assign13440_e19120;
        locals.var_rrr_b_dn0 = assign13440_e19120_d_n0;
        locals.var_rrr_b_dn2 = assign13440_e19120_d_n2;
        locals.var_rrr_b_dn6 = assign13440_e19120_d_n6;
        locals.var_rrr_b_dn7 = assign13440_e19120_d_n7;
        locals.var_rrr_b_dn10 = assign13440_e19120_d_n10;
        locals.var_rrr_b_dn11 = assign13440_e19120_d_n11;
        locals.var_rrr_b_dn12 = assign13440_e19120_d_n12;
        locals.var_rrr_b_dn17 = assign13440_e19120_d_n17;

        let assign13450_e19123: f64 = (locals.var_beta * locals.var_ps0b);
        let assign13450_e19125: f64 = (assign13450_e19123 - 1.0);
        let assign13450_e19127: f64 = if assign13450_e19125 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign13450_e19127;

        let (assign13460_e19141, assign13460_e19141_d_n0, assign13460_e19141_d_n2, assign13460_e19141_d_n6, assign13460_e19141_d_n7, assign13460_e19141_d_n10, assign13460_e19141_d_n11, assign13460_e19141_d_n12, assign13460_e19141_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign13460_e19136: f64 = (locals.var_beta * locals.var_ps0b);
        let assign13460_e19138: f64 = (assign13460_e19136 - 1.0);
        let assign13460_e19139: f64 = (assign13460_e19138).sqrt();
        (assign13460_e19139, ((locals.var_beta * locals.var_ps0b_dn0) / (2.0 * assign13460_e19139)), ((locals.var_beta * locals.var_ps0b_dn2) / (2.0 * assign13460_e19139)), ((locals.var_beta * locals.var_ps0b_dn6) / (2.0 * assign13460_e19139)), ((locals.var_beta * locals.var_ps0b_dn7) / (2.0 * assign13460_e19139)), (((locals.var_beta_dn10 * locals.var_ps0b) + (locals.var_beta * locals.var_ps0b_dn10)) / (2.0 * assign13460_e19139)), ((locals.var_beta * locals.var_ps0b_dn11) / (2.0 * assign13460_e19139)), ((locals.var_beta * locals.var_ps0b_dn12) / (2.0 * assign13460_e19139)), ((locals.var_beta * locals.var_ps0b_dn17) / (2.0 * assign13460_e19139)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13460_e19141;
        locals.var_t1_dn0 = assign13460_e19141_d_n0;
        locals.var_t1_dn2 = assign13460_e19141_d_n2;
        locals.var_t1_dn6 = assign13460_e19141_d_n6;
        locals.var_t1_dn7 = assign13460_e19141_d_n7;
        locals.var_t1_dn10 = assign13460_e19141_d_n10;
        locals.var_t1_dn11 = assign13460_e19141_d_n11;
        locals.var_t1_dn12 = assign13460_e19141_d_n12;
        locals.var_t1_dn17 = assign13460_e19141_d_n17;

        let (assign13470_e19151, assign13470_e19151_d_n0, assign13470_e19151_d_n2, assign13470_e19151_d_n6, assign13470_e19151_d_n7, assign13470_e19151_d_n10, assign13470_e19151_d_n11, assign13470_e19151_d_n12, assign13470_e19151_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13470_e19148: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign13470_e19149: f64 = (-assign13470_e19148);
        (assign13470_e19149, (-(locals.var_q_nl_dn0 - locals.var_q_n0_dn0)), (-(locals.var_q_nl_dn2 - locals.var_q_n0_dn2)), (-(locals.var_q_nl_dn6 - locals.var_q_n0_dn6)), (-(locals.var_q_nl_dn7 - locals.var_q_n0_dn7)), (-(locals.var_q_nl_dn10 - locals.var_q_n0_dn10)), (-(locals.var_q_nl_dn11 - locals.var_q_n0_dn11)), (-(locals.var_q_nl_dn12 - locals.var_q_n0_dn12)), (-(locals.var_q_nl_dn17 - locals.var_q_n0_dn17)),)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13470_e19151;
        locals.var_rrr_cc_dn0 = assign13470_e19151_d_n0;
        locals.var_rrr_cc_dn2 = assign13470_e19151_d_n2;
        locals.var_rrr_cc_dn6 = assign13470_e19151_d_n6;
        locals.var_rrr_cc_dn7 = assign13470_e19151_d_n7;
        locals.var_rrr_cc_dn10 = assign13470_e19151_d_n10;
        locals.var_rrr_cc_dn11 = assign13470_e19151_d_n11;
        locals.var_rrr_cc_dn12 = assign13470_e19151_d_n12;
        locals.var_rrr_cc_dn17 = assign13470_e19151_d_n17;

        let assign13480_e19156: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13480_e19157: f64 = assign13480_e19156;
        let assign13480_e19161: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13480_e19164: f64 = if ((locals.var_rrr_cc < assign13480_e19157) && (assign13480_e19161 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard421 = assign13480_e19164;

        let (assign13490_e19179, assign13490_e19179_d_n0, assign13490_e19179_d_n2, assign13490_e19179_d_n6, assign13490_e19179_d_n7, assign13490_e19179_d_n10, assign13490_e19179_d_n11, assign13490_e19179_d_n12, assign13490_e19179_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13490_e19174: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13490_e19175: f64 = assign13490_e19174;
        let assign13490_e19177: f64 = (assign13490_e19175 - locals.var_rrr_cc);
        (assign13490_e19177, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_rrr_cc_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_rrr_cc_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_rrr_cc_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_rrr_cc_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_rrr_cc_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_rrr_cc_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_rrr_cc_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_rrr_cc_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13490_e19179;
        locals.var_tmf1_dn0 = assign13490_e19179_d_n0;
        locals.var_tmf1_dn2 = assign13490_e19179_d_n2;
        locals.var_tmf1_dn6 = assign13490_e19179_d_n6;
        locals.var_tmf1_dn7 = assign13490_e19179_d_n7;
        locals.var_tmf1_dn10 = assign13490_e19179_d_n10;
        locals.var_tmf1_dn11 = assign13490_e19179_d_n11;
        locals.var_tmf1_dn12 = assign13490_e19179_d_n12;
        locals.var_tmf1_dn17 = assign13490_e19179_d_n17;

        let (assign13500_e19190, assign13500_e19190_d_n0, assign13500_e19190_d_n2, assign13500_e19190_d_n6, assign13500_e19190_d_n7, assign13500_e19190_d_n10, assign13500_e19190_d_n11, assign13500_e19190_d_n12, assign13500_e19190_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13500_e19188: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13500_e19188, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13500_e19190;
        locals.var_x2_dn0 = assign13500_e19190_d_n0;
        locals.var_x2_dn2 = assign13500_e19190_d_n2;
        locals.var_x2_dn6 = assign13500_e19190_d_n6;
        locals.var_x2_dn7 = assign13500_e19190_d_n7;
        locals.var_x2_dn10 = assign13500_e19190_d_n10;
        locals.var_x2_dn11 = assign13500_e19190_d_n11;
        locals.var_x2_dn12 = assign13500_e19190_d_n12;
        locals.var_x2_dn17 = assign13500_e19190_d_n17;

        let (assign13510_e19205, assign13510_e19205_d_n0, assign13510_e19205_d_n2, assign13510_e19205_d_n6, assign13510_e19205_d_n7, assign13510_e19205_d_n10, assign13510_e19205_d_n11, assign13510_e19205_d_n12, assign13510_e19205_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13510_e19199: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13510_e19202: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13510_e19203: f64 = (assign13510_e19199 * assign13510_e19202);
        (assign13510_e19203, (((locals.var_q_fd_soi_dn0 * 1e-5) * assign13510_e19202) + (assign13510_e19199 * (locals.var_q_fd_soi_dn0 * 1e-5))), (((locals.var_q_fd_soi_dn2 * 1e-5) * assign13510_e19202) + (assign13510_e19199 * (locals.var_q_fd_soi_dn2 * 1e-5))), (((locals.var_q_fd_soi_dn6 * 1e-5) * assign13510_e19202) + (assign13510_e19199 * (locals.var_q_fd_soi_dn6 * 1e-5))), (((locals.var_q_fd_soi_dn7 * 1e-5) * assign13510_e19202) + (assign13510_e19199 * (locals.var_q_fd_soi_dn7 * 1e-5))), (((locals.var_q_fd_soi_dn10 * 1e-5) * assign13510_e19202) + (assign13510_e19199 * (locals.var_q_fd_soi_dn10 * 1e-5))), (((locals.var_q_fd_soi_dn11 * 1e-5) * assign13510_e19202) + (assign13510_e19199 * (locals.var_q_fd_soi_dn11 * 1e-5))), (((locals.var_q_fd_soi_dn12 * 1e-5) * assign13510_e19202) + (assign13510_e19199 * (locals.var_q_fd_soi_dn12 * 1e-5))), (((locals.var_q_fd_soi_dn17 * 1e-5) * assign13510_e19202) + (assign13510_e19199 * (locals.var_q_fd_soi_dn17 * 1e-5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13510_e19205;
        locals.var_xmax2_dn0 = assign13510_e19205_d_n0;
        locals.var_xmax2_dn2 = assign13510_e19205_d_n2;
        locals.var_xmax2_dn6 = assign13510_e19205_d_n6;
        locals.var_xmax2_dn7 = assign13510_e19205_d_n7;
        locals.var_xmax2_dn10 = assign13510_e19205_d_n10;
        locals.var_xmax2_dn11 = assign13510_e19205_d_n11;
        locals.var_xmax2_dn12 = assign13510_e19205_d_n12;
        locals.var_xmax2_dn17 = assign13510_e19205_d_n17;

        let (assign13520_e19214, assign13520_e19214_d_n0, assign13520_e19214_d_n2, assign13520_e19214_d_n6, assign13520_e19214_d_n7, assign13520_e19214_d_n10, assign13520_e19214_d_n11, assign13520_e19214_d_n12, assign13520_e19214_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13520_e19214;
        locals.var_xp_dn0 = assign13520_e19214_d_n0;
        locals.var_xp_dn2 = assign13520_e19214_d_n2;
        locals.var_xp_dn6 = assign13520_e19214_d_n6;
        locals.var_xp_dn7 = assign13520_e19214_d_n7;
        locals.var_xp_dn10 = assign13520_e19214_d_n10;
        locals.var_xp_dn11 = assign13520_e19214_d_n11;
        locals.var_xp_dn12 = assign13520_e19214_d_n12;
        locals.var_xp_dn17 = assign13520_e19214_d_n17;

        let (assign13530_e19223, assign13530_e19223_d_n0, assign13530_e19223_d_n2, assign13530_e19223_d_n6, assign13530_e19223_d_n7, assign13530_e19223_d_n10, assign13530_e19223_d_n11, assign13530_e19223_d_n12, assign13530_e19223_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13530_e19223;
        locals.var_xmp_dn0 = assign13530_e19223_d_n0;
        locals.var_xmp_dn2 = assign13530_e19223_d_n2;
        locals.var_xmp_dn6 = assign13530_e19223_d_n6;
        locals.var_xmp_dn7 = assign13530_e19223_d_n7;
        locals.var_xmp_dn10 = assign13530_e19223_d_n10;
        locals.var_xmp_dn11 = assign13530_e19223_d_n11;
        locals.var_xmp_dn12 = assign13530_e19223_d_n12;
        locals.var_xmp_dn17 = assign13530_e19223_d_n17;

        let (assign13540_e19232,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13540_e19232;

        let (assign13550_e19241,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13550_e19241;

        let (assign13560_e19250, assign13560_e19250_d_n0, assign13560_e19250_d_n2, assign13560_e19250_d_n6, assign13560_e19250_d_n7, assign13560_e19250_d_n10, assign13560_e19250_d_n11, assign13560_e19250_d_n12, assign13560_e19250_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13560_e19250;
        locals.var_arg_dn0 = assign13560_e19250_d_n0;
        locals.var_arg_dn2 = assign13560_e19250_d_n2;
        locals.var_arg_dn6 = assign13560_e19250_d_n6;
        locals.var_arg_dn7 = assign13560_e19250_d_n7;
        locals.var_arg_dn10 = assign13560_e19250_d_n10;
        locals.var_arg_dn11 = assign13560_e19250_d_n11;
        locals.var_arg_dn12 = assign13560_e19250_d_n12;
        locals.var_arg_dn17 = assign13560_e19250_d_n17;

        let (assign13570_e19259, assign13570_e19259_d_n0, assign13570_e19259_d_n2, assign13570_e19259_d_n6, assign13570_e19259_d_n7, assign13570_e19259_d_n10, assign13570_e19259_d_n11, assign13570_e19259_d_n12, assign13570_e19259_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13570_e19259;
        locals.var_dnm_dn0 = assign13570_e19259_d_n0;
        locals.var_dnm_dn2 = assign13570_e19259_d_n2;
        locals.var_dnm_dn6 = assign13570_e19259_d_n6;
        locals.var_dnm_dn7 = assign13570_e19259_d_n7;
        locals.var_dnm_dn10 = assign13570_e19259_d_n10;
        locals.var_dnm_dn11 = assign13570_e19259_d_n11;
        locals.var_dnm_dn12 = assign13570_e19259_d_n12;
        locals.var_dnm_dn17 = assign13570_e19259_d_n17;

    }

    pub(super) fn stamp_transient_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign13580_e19270, assign13580_e19270_d_n0, assign13580_e19270_d_n2, assign13580_e19270_d_n6, assign13580_e19270_d_n7, assign13580_e19270_d_n10, assign13580_e19270_d_n11, assign13580_e19270_d_n12, assign13580_e19270_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13580_e19268: f64 = (locals.var_xp * locals.var_x2);
        (assign13580_e19268, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13580_e19270;
        locals.var_xp_dn0 = assign13580_e19270_d_n0;
        locals.var_xp_dn2 = assign13580_e19270_d_n2;
        locals.var_xp_dn6 = assign13580_e19270_d_n6;
        locals.var_xp_dn7 = assign13580_e19270_d_n7;
        locals.var_xp_dn10 = assign13580_e19270_d_n10;
        locals.var_xp_dn11 = assign13580_e19270_d_n11;
        locals.var_xp_dn12 = assign13580_e19270_d_n12;
        locals.var_xp_dn17 = assign13580_e19270_d_n17;

        let (assign13590_e19281, assign13590_e19281_d_n0, assign13590_e19281_d_n2, assign13590_e19281_d_n6, assign13590_e19281_d_n7, assign13590_e19281_d_n10, assign13590_e19281_d_n11, assign13590_e19281_d_n12, assign13590_e19281_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13590_e19279: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13590_e19279, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13590_e19281;
        locals.var_xmp_dn0 = assign13590_e19281_d_n0;
        locals.var_xmp_dn2 = assign13590_e19281_d_n2;
        locals.var_xmp_dn6 = assign13590_e19281_d_n6;
        locals.var_xmp_dn7 = assign13590_e19281_d_n7;
        locals.var_xmp_dn10 = assign13590_e19281_d_n10;
        locals.var_xmp_dn11 = assign13590_e19281_d_n11;
        locals.var_xmp_dn12 = assign13590_e19281_d_n12;
        locals.var_xmp_dn17 = assign13590_e19281_d_n17;

        let (assign13600_e19292, assign13600_e19292_d_n0, assign13600_e19292_d_n2, assign13600_e19292_d_n6, assign13600_e19292_d_n7, assign13600_e19292_d_n10, assign13600_e19292_d_n11, assign13600_e19292_d_n12, assign13600_e19292_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13600_e19290: f64 = (locals.var_xp * locals.var_x2);
        (assign13600_e19290, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13600_e19292;
        locals.var_xp_dn0 = assign13600_e19292_d_n0;
        locals.var_xp_dn2 = assign13600_e19292_d_n2;
        locals.var_xp_dn6 = assign13600_e19292_d_n6;
        locals.var_xp_dn7 = assign13600_e19292_d_n7;
        locals.var_xp_dn10 = assign13600_e19292_d_n10;
        locals.var_xp_dn11 = assign13600_e19292_d_n11;
        locals.var_xp_dn12 = assign13600_e19292_d_n12;
        locals.var_xp_dn17 = assign13600_e19292_d_n17;

        let (assign13610_e19303, assign13610_e19303_d_n0, assign13610_e19303_d_n2, assign13610_e19303_d_n6, assign13610_e19303_d_n7, assign13610_e19303_d_n10, assign13610_e19303_d_n11, assign13610_e19303_d_n12, assign13610_e19303_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13610_e19301: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13610_e19301, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13610_e19303;
        locals.var_xmp_dn0 = assign13610_e19303_d_n0;
        locals.var_xmp_dn2 = assign13610_e19303_d_n2;
        locals.var_xmp_dn6 = assign13610_e19303_d_n6;
        locals.var_xmp_dn7 = assign13610_e19303_d_n7;
        locals.var_xmp_dn10 = assign13610_e19303_d_n10;
        locals.var_xmp_dn11 = assign13610_e19303_d_n11;
        locals.var_xmp_dn12 = assign13610_e19303_d_n12;
        locals.var_xmp_dn17 = assign13610_e19303_d_n17;

        let (assign13620_e19314, assign13620_e19314_d_n0, assign13620_e19314_d_n2, assign13620_e19314_d_n6, assign13620_e19314_d_n7, assign13620_e19314_d_n10, assign13620_e19314_d_n11, assign13620_e19314_d_n12, assign13620_e19314_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13620_e19312: f64 = (locals.var_xp + locals.var_xmp);
        (assign13620_e19312, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13620_e19314;
        locals.var_arg_dn0 = assign13620_e19314_d_n0;
        locals.var_arg_dn2 = assign13620_e19314_d_n2;
        locals.var_arg_dn6 = assign13620_e19314_d_n6;
        locals.var_arg_dn7 = assign13620_e19314_d_n7;
        locals.var_arg_dn10 = assign13620_e19314_d_n10;
        locals.var_arg_dn11 = assign13620_e19314_d_n11;
        locals.var_arg_dn12 = assign13620_e19314_d_n12;
        locals.var_arg_dn17 = assign13620_e19314_d_n17;

        let (assign13630_e19323, assign13630_e19323_d_n0, assign13630_e19323_d_n2, assign13630_e19323_d_n6, assign13630_e19323_d_n7, assign13630_e19323_d_n10, assign13630_e19323_d_n11, assign13630_e19323_d_n12, assign13630_e19323_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13630_e19323;
        locals.var_dnm_dn0 = assign13630_e19323_d_n0;
        locals.var_dnm_dn2 = assign13630_e19323_d_n2;
        locals.var_dnm_dn6 = assign13630_e19323_d_n6;
        locals.var_dnm_dn7 = assign13630_e19323_d_n7;
        locals.var_dnm_dn10 = assign13630_e19323_d_n10;
        locals.var_dnm_dn11 = assign13630_e19323_d_n11;
        locals.var_dnm_dn12 = assign13630_e19323_d_n12;
        locals.var_dnm_dn17 = assign13630_e19323_d_n17;

        let assign13640_e19338: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard422 = assign13640_e19338;

        let assign13650_e19341: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign13650_e19341;

        let (assign13660_e19354,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_guard423 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13660_e19354;

        let assign13670_e19357: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign13670_e19357;

        let (assign13680_e19373,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_guard423 == 0.0)) && (locals.var_guard424 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13680_e19373;

        let assign13690_e19376: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign13690_e19376;

        let (assign13700_e19395,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_guard423 == 0.0)) && (locals.var_guard424 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13700_e19395;

        let assign13710_e19398: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign13710_e19398;

        let (assign13720_e19420,) = {
    if ((((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_guard423 == 0.0)) && (locals.var_guard424 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13720_e19420;

        let (assign13730_e19431,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13730_e19431;

        let mut assign13740_loop_guard: usize = 0;
        while {
            let assign13740_cond_e19443: f64 = if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign13740_cond_e19443 != 0.0
        } {
            assign13740_loop_guard += 1;
            assert!(assign13740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign13740_body0_e19455, assign13740_body0_e19455_d_n0, assign13740_body0_e19455_d_n2, assign13740_body0_e19455_d_n6, assign13740_body0_e19455_d_n7, assign13740_body0_e19455_d_n10, assign13740_body0_e19455_d_n11, assign13740_body0_e19455_d_n12, assign13740_body0_e19455_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign13740_body0_e19453: f64 = (locals.var_dnm).sqrt();
        (assign13740_body0_e19453, (locals.var_dnm_dn0 / (2.0 * assign13740_body0_e19453)), (locals.var_dnm_dn2 / (2.0 * assign13740_body0_e19453)), (locals.var_dnm_dn6 / (2.0 * assign13740_body0_e19453)), (locals.var_dnm_dn7 / (2.0 * assign13740_body0_e19453)), (locals.var_dnm_dn10 / (2.0 * assign13740_body0_e19453)), (locals.var_dnm_dn11 / (2.0 * assign13740_body0_e19453)), (locals.var_dnm_dn12 / (2.0 * assign13740_body0_e19453)), (locals.var_dnm_dn17 / (2.0 * assign13740_body0_e19453)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign13740_body0_e19455;
            locals.var_dnm_dn0 = assign13740_body0_e19455_d_n0;
            locals.var_dnm_dn2 = assign13740_body0_e19455_d_n2;
            locals.var_dnm_dn6 = assign13740_body0_e19455_d_n6;
            locals.var_dnm_dn7 = assign13740_body0_e19455_d_n7;
            locals.var_dnm_dn10 = assign13740_body0_e19455_d_n10;
            locals.var_dnm_dn11 = assign13740_body0_e19455_d_n11;
            locals.var_dnm_dn12 = assign13740_body0_e19455_d_n12;
            locals.var_dnm_dn17 = assign13740_body0_e19455_d_n17;
            let (assign13740_body1_e19468,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign13740_body1_e19466: f64 = (locals.var_m0 + 1.0);
        (assign13740_body1_e19466,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign13740_body1_e19468;
        }

        let (assign13750_e19486, assign13750_e19486_d_n0, assign13750_e19486_d_n2, assign13750_e19486_d_n6, assign13750_e19486_d_n7, assign13750_e19486_d_n10, assign13750_e19486_d_n11, assign13750_e19486_d_n12, assign13750_e19486_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) && (locals.var_guard422 == 0.0)) {
        let assign13750_e19482: f64 = (2.0 * 2.0);
        let assign13750_e19483: f64 = (1.0 / assign13750_e19482);
        let assign13750_e19484: f64 = (locals.var_dnm).powf(assign13750_e19483);
        (assign13750_e19484, if 0.0 == 0.0 && ((assign13750_e19483) as f64).is_finite() && ((assign13750_e19483) as f64).fract() == 0.0 { if assign13750_e19483 == 0.0 { 0.0 } else { (assign13750_e19483 * ((locals.var_dnm).powf(assign13750_e19483 - 1.0) * locals.var_dnm_dn0)) } } else { (assign13750_e19484 * (assign13750_e19483 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13750_e19483) as f64).is_finite() && ((assign13750_e19483) as f64).fract() == 0.0 { if assign13750_e19483 == 0.0 { 0.0 } else { (assign13750_e19483 * ((locals.var_dnm).powf(assign13750_e19483 - 1.0) * locals.var_dnm_dn2)) } } else { (assign13750_e19484 * (assign13750_e19483 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13750_e19483) as f64).is_finite() && ((assign13750_e19483) as f64).fract() == 0.0 { if assign13750_e19483 == 0.0 { 0.0 } else { (assign13750_e19483 * ((locals.var_dnm).powf(assign13750_e19483 - 1.0) * locals.var_dnm_dn6)) } } else { (assign13750_e19484 * (assign13750_e19483 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13750_e19483) as f64).is_finite() && ((assign13750_e19483) as f64).fract() == 0.0 { if assign13750_e19483 == 0.0 { 0.0 } else { (assign13750_e19483 * ((locals.var_dnm).powf(assign13750_e19483 - 1.0) * locals.var_dnm_dn7)) } } else { (assign13750_e19484 * (assign13750_e19483 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13750_e19483) as f64).is_finite() && ((assign13750_e19483) as f64).fract() == 0.0 { if assign13750_e19483 == 0.0 { 0.0 } else { (assign13750_e19483 * ((locals.var_dnm).powf(assign13750_e19483 - 1.0) * locals.var_dnm_dn10)) } } else { (assign13750_e19484 * (assign13750_e19483 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13750_e19483) as f64).is_finite() && ((assign13750_e19483) as f64).fract() == 0.0 { if assign13750_e19483 == 0.0 { 0.0 } else { (assign13750_e19483 * ((locals.var_dnm).powf(assign13750_e19483 - 1.0) * locals.var_dnm_dn11)) } } else { (assign13750_e19484 * (assign13750_e19483 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13750_e19483) as f64).is_finite() && ((assign13750_e19483) as f64).fract() == 0.0 { if assign13750_e19483 == 0.0 { 0.0 } else { (assign13750_e19483 * ((locals.var_dnm).powf(assign13750_e19483 - 1.0) * locals.var_dnm_dn12)) } } else { (assign13750_e19484 * (assign13750_e19483 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13750_e19483) as f64).is_finite() && ((assign13750_e19483) as f64).fract() == 0.0 { if assign13750_e19483 == 0.0 { 0.0 } else { (assign13750_e19483 * ((locals.var_dnm).powf(assign13750_e19483 - 1.0) * locals.var_dnm_dn17)) } } else { (assign13750_e19484 * (assign13750_e19483 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13750_e19486;
        locals.var_dnm_dn0 = assign13750_e19486_d_n0;
        locals.var_dnm_dn2 = assign13750_e19486_d_n2;
        locals.var_dnm_dn6 = assign13750_e19486_d_n6;
        locals.var_dnm_dn7 = assign13750_e19486_d_n7;
        locals.var_dnm_dn10 = assign13750_e19486_d_n10;
        locals.var_dnm_dn11 = assign13750_e19486_d_n11;
        locals.var_dnm_dn12 = assign13750_e19486_d_n12;
        locals.var_dnm_dn17 = assign13750_e19486_d_n17;

        let (assign13760_e19497, assign13760_e19497_d_n0, assign13760_e19497_d_n2, assign13760_e19497_d_n6, assign13760_e19497_d_n7, assign13760_e19497_d_n10, assign13760_e19497_d_n11, assign13760_e19497_d_n12, assign13760_e19497_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13760_e19495: f64 = (1.0 / locals.var_dnm);
        (assign13760_e19495, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13760_e19497;
        locals.var_dnm_dn0 = assign13760_e19497_d_n0;
        locals.var_dnm_dn2 = assign13760_e19497_d_n2;
        locals.var_dnm_dn6 = assign13760_e19497_d_n6;
        locals.var_dnm_dn7 = assign13760_e19497_d_n7;
        locals.var_dnm_dn10 = assign13760_e19497_d_n10;
        locals.var_dnm_dn11 = assign13760_e19497_d_n11;
        locals.var_dnm_dn12 = assign13760_e19497_d_n12;
        locals.var_dnm_dn17 = assign13760_e19497_d_n17;

        let (assign13770_e19512, assign13770_e19512_d_n0, assign13770_e19512_d_n2, assign13770_e19512_d_n6, assign13770_e19512_d_n7, assign13770_e19512_d_n10, assign13770_e19512_d_n11, assign13770_e19512_d_n12, assign13770_e19512_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13770_e19507: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13770_e19508: f64 = (locals.var_tmf1 * assign13770_e19507);
        let assign13770_e19510: f64 = (assign13770_e19508 * locals.var_dnm);
        (assign13770_e19510, ((((locals.var_tmf1_dn0 * assign13770_e19507) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn0 * 1e-5))) * locals.var_dnm) + (assign13770_e19508 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign13770_e19507) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn2 * 1e-5))) * locals.var_dnm) + (assign13770_e19508 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign13770_e19507) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn6 * 1e-5))) * locals.var_dnm) + (assign13770_e19508 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign13770_e19507) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn7 * 1e-5))) * locals.var_dnm) + (assign13770_e19508 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign13770_e19507) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn10 * 1e-5))) * locals.var_dnm) + (assign13770_e19508 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign13770_e19507) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn11 * 1e-5))) * locals.var_dnm) + (assign13770_e19508 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign13770_e19507) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn12 * 1e-5))) * locals.var_dnm) + (assign13770_e19508 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign13770_e19507) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn17 * 1e-5))) * locals.var_dnm) + (assign13770_e19508 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign13770_e19512;
        locals.var_tmf0_dn0 = assign13770_e19512_d_n0;
        locals.var_tmf0_dn2 = assign13770_e19512_d_n2;
        locals.var_tmf0_dn6 = assign13770_e19512_d_n6;
        locals.var_tmf0_dn7 = assign13770_e19512_d_n7;
        locals.var_tmf0_dn10 = assign13770_e19512_d_n10;
        locals.var_tmf0_dn11 = assign13770_e19512_d_n11;
        locals.var_tmf0_dn12 = assign13770_e19512_d_n12;
        locals.var_tmf0_dn17 = assign13770_e19512_d_n17;

        let (assign13780_e19527, assign13780_e19527_d_n0, assign13780_e19527_d_n2, assign13780_e19527_d_n6, assign13780_e19527_d_n7, assign13780_e19527_d_n10, assign13780_e19527_d_n11, assign13780_e19527_d_n12, assign13780_e19527_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 != 0.0)) {
        let assign13780_e19522: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13780_e19523: f64 = assign13780_e19522;
        let assign13780_e19525: f64 = (assign13780_e19523 - locals.var_tmf0);
        (assign13780_e19525, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_tmf0_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_tmf0_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_tmf0_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_tmf0_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_tmf0_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_tmf0_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_tmf0_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13780_e19527;
        locals.var_rrr_cc_dn0 = assign13780_e19527_d_n0;
        locals.var_rrr_cc_dn2 = assign13780_e19527_d_n2;
        locals.var_rrr_cc_dn6 = assign13780_e19527_d_n6;
        locals.var_rrr_cc_dn7 = assign13780_e19527_d_n7;
        locals.var_rrr_cc_dn10 = assign13780_e19527_d_n10;
        locals.var_rrr_cc_dn11 = assign13780_e19527_d_n11;
        locals.var_rrr_cc_dn12 = assign13780_e19527_d_n12;
        locals.var_rrr_cc_dn17 = assign13780_e19527_d_n17;

        let (assign13790_e19537, assign13790_e19537_d_n0, assign13790_e19537_d_n2, assign13790_e19537_d_n6, assign13790_e19537_d_n7, assign13790_e19537_d_n10, assign13790_e19537_d_n11, assign13790_e19537_d_n12, assign13790_e19537_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard421 == 0.0)) {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13790_e19537;
        locals.var_rrr_cc_dn0 = assign13790_e19537_d_n0;
        locals.var_rrr_cc_dn2 = assign13790_e19537_d_n2;
        locals.var_rrr_cc_dn6 = assign13790_e19537_d_n6;
        locals.var_rrr_cc_dn7 = assign13790_e19537_d_n7;
        locals.var_rrr_cc_dn10 = assign13790_e19537_d_n10;
        locals.var_rrr_cc_dn11 = assign13790_e19537_d_n11;
        locals.var_rrr_cc_dn12 = assign13790_e19537_d_n12;
        locals.var_rrr_cc_dn17 = assign13790_e19537_d_n17;

        let (assign13800_e19557, assign13800_e19557_d_n0, assign13800_e19557_d_n2, assign13800_e19557_d_n6, assign13800_e19557_d_n7, assign13800_e19557_d_n10, assign13800_e19557_d_n11, assign13800_e19557_d_n12, assign13800_e19557_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13800_e19545: f64 = (-locals.var_rrr_cc);
        let assign13800_e19546: f64 = (2.0 * assign13800_e19545);
        let assign13800_e19549: f64 = (locals.var_beta * locals.var_c_fox);
        let assign13800_e19551: f64 = (assign13800_e19549 * locals.var_rrr_p0);
        let assign13800_e19553: f64 = (assign13800_e19551 * locals.var_rrr_p0);
        let assign13800_e19554: f64 = (assign13800_e19546 / assign13800_e19553);
        let assign13800_e19555: f64 = (1.0 + assign13800_e19554);
        (assign13800_e19555, ((((2.0 * (-locals.var_rrr_cc_dn0)) * assign13800_e19553) - (assign13800_e19546 * (((((locals.var_beta * locals.var_c_fox_dn0) * locals.var_rrr_p0) + (assign13800_e19549 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13800_e19551 * locals.var_rrr_p0_dn0)))) / (assign13800_e19553 * assign13800_e19553)), ((((2.0 * (-locals.var_rrr_cc_dn2)) * assign13800_e19553) - (assign13800_e19546 * (((((locals.var_beta * locals.var_c_fox_dn2) * locals.var_rrr_p0) + (assign13800_e19549 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13800_e19551 * locals.var_rrr_p0_dn2)))) / (assign13800_e19553 * assign13800_e19553)), ((((2.0 * (-locals.var_rrr_cc_dn6)) * assign13800_e19553) - (assign13800_e19546 * (((((locals.var_beta * locals.var_c_fox_dn6) * locals.var_rrr_p0) + (assign13800_e19549 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13800_e19551 * locals.var_rrr_p0_dn6)))) / (assign13800_e19553 * assign13800_e19553)), ((((2.0 * (-locals.var_rrr_cc_dn7)) * assign13800_e19553) - (assign13800_e19546 * (((((locals.var_beta * locals.var_c_fox_dn7) * locals.var_rrr_p0) + (assign13800_e19549 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13800_e19551 * locals.var_rrr_p0_dn7)))) / (assign13800_e19553 * assign13800_e19553)), ((((2.0 * (-locals.var_rrr_cc_dn10)) * assign13800_e19553) - (assign13800_e19546 * ((((((locals.var_beta_dn10 * locals.var_c_fox) + (locals.var_beta * locals.var_c_fox_dn10)) * locals.var_rrr_p0) + (assign13800_e19549 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13800_e19551 * locals.var_rrr_p0_dn10)))) / (assign13800_e19553 * assign13800_e19553)), ((((2.0 * (-locals.var_rrr_cc_dn11)) * assign13800_e19553) - (assign13800_e19546 * (((((locals.var_beta * locals.var_c_fox_dn11) * locals.var_rrr_p0) + (assign13800_e19549 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13800_e19551 * locals.var_rrr_p0_dn11)))) / (assign13800_e19553 * assign13800_e19553)), ((((2.0 * (-locals.var_rrr_cc_dn12)) * assign13800_e19553) - (assign13800_e19546 * (((((locals.var_beta * locals.var_c_fox_dn12) * locals.var_rrr_p0) + (assign13800_e19549 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13800_e19551 * locals.var_rrr_p0_dn12)))) / (assign13800_e19553 * assign13800_e19553)), ((((2.0 * (-locals.var_rrr_cc_dn17)) * assign13800_e19553) - (assign13800_e19546 * (((((locals.var_beta * locals.var_c_fox_dn17) * locals.var_rrr_p0) + (assign13800_e19549 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13800_e19551 * locals.var_rrr_p0_dn17)))) / (assign13800_e19553 * assign13800_e19553)),)
    } else {
        (locals.var_rrr_alpha_soi, locals.var_rrr_alpha_soi_dn0, locals.var_rrr_alpha_soi_dn2, locals.var_rrr_alpha_soi_dn6, locals.var_rrr_alpha_soi_dn7, locals.var_rrr_alpha_soi_dn10, locals.var_rrr_alpha_soi_dn11, locals.var_rrr_alpha_soi_dn12, locals.var_rrr_alpha_soi_dn17,)
    }
};
        locals.var_rrr_alpha_soi = assign13800_e19557;
        locals.var_rrr_alpha_soi_dn0 = assign13800_e19557_d_n0;
        locals.var_rrr_alpha_soi_dn2 = assign13800_e19557_d_n2;
        locals.var_rrr_alpha_soi_dn6 = assign13800_e19557_d_n6;
        locals.var_rrr_alpha_soi_dn7 = assign13800_e19557_d_n7;
        locals.var_rrr_alpha_soi_dn10 = assign13800_e19557_d_n10;
        locals.var_rrr_alpha_soi_dn11 = assign13800_e19557_d_n11;
        locals.var_rrr_alpha_soi_dn12 = assign13800_e19557_d_n12;
        locals.var_rrr_alpha_soi_dn17 = assign13800_e19557_d_n17;

        let (assign13810_e19570, assign13810_e19570_d_n0, assign13810_e19570_d_n2, assign13810_e19570_d_n6, assign13810_e19570_d_n7, assign13810_e19570_d_n10, assign13810_e19570_d_n11, assign13810_e19570_d_n12, assign13810_e19570_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13810_e19564: f64 = (locals.var_rrr_p0 * locals.var_rrr_p0);
        let assign13810_e19566: f64 = (assign13810_e19564 * locals.var_rrr_p0);
        let assign13810_e19568: f64 = (assign13810_e19566 * locals.var_rrr_p0);
        (assign13810_e19568, ((((((locals.var_rrr_p0_dn0 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13810_e19564 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13810_e19566 * locals.var_rrr_p0_dn0)), ((((((locals.var_rrr_p0_dn2 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13810_e19564 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13810_e19566 * locals.var_rrr_p0_dn2)), ((((((locals.var_rrr_p0_dn6 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13810_e19564 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13810_e19566 * locals.var_rrr_p0_dn6)), ((((((locals.var_rrr_p0_dn7 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13810_e19564 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13810_e19566 * locals.var_rrr_p0_dn7)), ((((((locals.var_rrr_p0_dn10 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13810_e19564 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13810_e19566 * locals.var_rrr_p0_dn10)), ((((((locals.var_rrr_p0_dn11 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13810_e19564 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13810_e19566 * locals.var_rrr_p0_dn11)), ((((((locals.var_rrr_p0_dn12 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13810_e19564 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13810_e19566 * locals.var_rrr_p0_dn12)), ((((((locals.var_rrr_p0_dn17 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13810_e19564 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13810_e19566 * locals.var_rrr_p0_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13810_e19570;
        locals.var_t1_dn0 = assign13810_e19570_d_n0;
        locals.var_t1_dn2 = assign13810_e19570_d_n2;
        locals.var_t1_dn6 = assign13810_e19570_d_n6;
        locals.var_t1_dn7 = assign13810_e19570_d_n7;
        locals.var_t1_dn10 = assign13810_e19570_d_n10;
        locals.var_t1_dn11 = assign13810_e19570_d_n11;
        locals.var_t1_dn12 = assign13810_e19570_d_n12;
        locals.var_t1_dn17 = assign13810_e19570_d_n17;

        let (assign13820_e19579, assign13820_e19579_d_n0, assign13820_e19579_d_n2, assign13820_e19579_d_n6, assign13820_e19579_d_n7, assign13820_e19579_d_n10, assign13820_e19579_d_n11, assign13820_e19579_d_n12, assign13820_e19579_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13820_e19577: f64 = (locals.var_rrr_alpha_soi * locals.var_rrr_p0);
        (assign13820_e19577, ((locals.var_rrr_alpha_soi_dn0 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn0)), ((locals.var_rrr_alpha_soi_dn2 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn2)), ((locals.var_rrr_alpha_soi_dn6 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn6)), ((locals.var_rrr_alpha_soi_dn7 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn7)), ((locals.var_rrr_alpha_soi_dn10 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn10)), ((locals.var_rrr_alpha_soi_dn11 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn11)), ((locals.var_rrr_alpha_soi_dn12 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn12)), ((locals.var_rrr_alpha_soi_dn17 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn17)),)
    } else {
        (locals.var_rrr_dd, locals.var_rrr_dd_dn0, locals.var_rrr_dd_dn2, locals.var_rrr_dd_dn6, locals.var_rrr_dd_dn7, locals.var_rrr_dd_dn10, locals.var_rrr_dd_dn11, locals.var_rrr_dd_dn12, locals.var_rrr_dd_dn17,)
    }
};
        locals.var_rrr_dd = assign13820_e19579;
        locals.var_rrr_dd_dn0 = assign13820_e19579_d_n0;
        locals.var_rrr_dd_dn2 = assign13820_e19579_d_n2;
        locals.var_rrr_dd_dn6 = assign13820_e19579_d_n6;
        locals.var_rrr_dd_dn7 = assign13820_e19579_d_n7;
        locals.var_rrr_dd_dn10 = assign13820_e19579_d_n10;
        locals.var_rrr_dd_dn11 = assign13820_e19579_d_n11;
        locals.var_rrr_dd_dn12 = assign13820_e19579_d_n12;
        locals.var_rrr_dd_dn17 = assign13820_e19579_d_n17;

        let (assign13830_e19590, assign13830_e19590_d_n0, assign13830_e19590_d_n2, assign13830_e19590_d_n6, assign13830_e19590_d_n7, assign13830_e19590_d_n10, assign13830_e19590_d_n11, assign13830_e19590_d_n12, assign13830_e19590_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign13830_e19587: f64 = (locals.var_rrr_dd / locals.var_vgvt);
        let assign13830_e19588: f64 = (1.0 - assign13830_e19587);
        (assign13830_e19588, (-(((locals.var_rrr_dd_dn0 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn0)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn2 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn2)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn6 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn6)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn7 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn7)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn10 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn10)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn11 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn11)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn12 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn12)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn17 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn17)) / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign13830_e19590;
        locals.var_rrr_eta_dn0 = assign13830_e19590_d_n0;
        locals.var_rrr_eta_dn2 = assign13830_e19590_d_n2;
        locals.var_rrr_eta_dn6 = assign13830_e19590_d_n6;
        locals.var_rrr_eta_dn7 = assign13830_e19590_d_n7;
        locals.var_rrr_eta_dn10 = assign13830_e19590_d_n10;
        locals.var_rrr_eta_dn11 = assign13830_e19590_d_n11;
        locals.var_rrr_eta_dn12 = assign13830_e19590_d_n12;
        locals.var_rrr_eta_dn17 = assign13830_e19590_d_n17;

        let assign13840_e19594: f64 = 1e-5;
        let assign13840_e19599: f64 = if ((locals.var_rrr_eta < assign13840_e19594) && (1e-5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard427 = assign13840_e19599;

        let (assign13850_e19612, assign13850_e19612_d_n0, assign13850_e19612_d_n2, assign13850_e19612_d_n6, assign13850_e19612_d_n7, assign13850_e19612_d_n10, assign13850_e19612_d_n11, assign13850_e19612_d_n12, assign13850_e19612_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13850_e19608: f64 = 1e-5;
        let assign13850_e19610: f64 = (assign13850_e19608 - locals.var_rrr_eta);
        (assign13850_e19610, (-locals.var_rrr_eta_dn0), (-locals.var_rrr_eta_dn2), (-locals.var_rrr_eta_dn6), (-locals.var_rrr_eta_dn7), (-locals.var_rrr_eta_dn10), (-locals.var_rrr_eta_dn11), (-locals.var_rrr_eta_dn12), (-locals.var_rrr_eta_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13850_e19612;
        locals.var_tmf1_dn0 = assign13850_e19612_d_n0;
        locals.var_tmf1_dn2 = assign13850_e19612_d_n2;
        locals.var_tmf1_dn6 = assign13850_e19612_d_n6;
        locals.var_tmf1_dn7 = assign13850_e19612_d_n7;
        locals.var_tmf1_dn10 = assign13850_e19612_d_n10;
        locals.var_tmf1_dn11 = assign13850_e19612_d_n11;
        locals.var_tmf1_dn12 = assign13850_e19612_d_n12;
        locals.var_tmf1_dn17 = assign13850_e19612_d_n17;

        let (assign13860_e19623, assign13860_e19623_d_n0, assign13860_e19623_d_n2, assign13860_e19623_d_n6, assign13860_e19623_d_n7, assign13860_e19623_d_n10, assign13860_e19623_d_n11, assign13860_e19623_d_n12, assign13860_e19623_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13860_e19621: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13860_e19621, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13860_e19623;
        locals.var_x2_dn0 = assign13860_e19623_d_n0;
        locals.var_x2_dn2 = assign13860_e19623_d_n2;
        locals.var_x2_dn6 = assign13860_e19623_d_n6;
        locals.var_x2_dn7 = assign13860_e19623_d_n7;
        locals.var_x2_dn10 = assign13860_e19623_d_n10;
        locals.var_x2_dn11 = assign13860_e19623_d_n11;
        locals.var_x2_dn12 = assign13860_e19623_d_n12;
        locals.var_x2_dn17 = assign13860_e19623_d_n17;

        let (assign13870_e19634, assign13870_e19634_d_n0, assign13870_e19634_d_n2, assign13870_e19634_d_n6, assign13870_e19634_d_n7, assign13870_e19634_d_n10, assign13870_e19634_d_n11, assign13870_e19634_d_n12, assign13870_e19634_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13870_e19632: f64 = (1e-5 * 1e-5);
        (assign13870_e19632, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13870_e19634;
        locals.var_xmax2_dn0 = assign13870_e19634_d_n0;
        locals.var_xmax2_dn2 = assign13870_e19634_d_n2;
        locals.var_xmax2_dn6 = assign13870_e19634_d_n6;
        locals.var_xmax2_dn7 = assign13870_e19634_d_n7;
        locals.var_xmax2_dn10 = assign13870_e19634_d_n10;
        locals.var_xmax2_dn11 = assign13870_e19634_d_n11;
        locals.var_xmax2_dn12 = assign13870_e19634_d_n12;
        locals.var_xmax2_dn17 = assign13870_e19634_d_n17;

        let (assign13880_e19643, assign13880_e19643_d_n0, assign13880_e19643_d_n2, assign13880_e19643_d_n6, assign13880_e19643_d_n7, assign13880_e19643_d_n10, assign13880_e19643_d_n11, assign13880_e19643_d_n12, assign13880_e19643_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13880_e19643;
        locals.var_xp_dn0 = assign13880_e19643_d_n0;
        locals.var_xp_dn2 = assign13880_e19643_d_n2;
        locals.var_xp_dn6 = assign13880_e19643_d_n6;
        locals.var_xp_dn7 = assign13880_e19643_d_n7;
        locals.var_xp_dn10 = assign13880_e19643_d_n10;
        locals.var_xp_dn11 = assign13880_e19643_d_n11;
        locals.var_xp_dn12 = assign13880_e19643_d_n12;
        locals.var_xp_dn17 = assign13880_e19643_d_n17;

        let (assign13890_e19652, assign13890_e19652_d_n0, assign13890_e19652_d_n2, assign13890_e19652_d_n6, assign13890_e19652_d_n7, assign13890_e19652_d_n10, assign13890_e19652_d_n11, assign13890_e19652_d_n12, assign13890_e19652_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13890_e19652;
        locals.var_xmp_dn0 = assign13890_e19652_d_n0;
        locals.var_xmp_dn2 = assign13890_e19652_d_n2;
        locals.var_xmp_dn6 = assign13890_e19652_d_n6;
        locals.var_xmp_dn7 = assign13890_e19652_d_n7;
        locals.var_xmp_dn10 = assign13890_e19652_d_n10;
        locals.var_xmp_dn11 = assign13890_e19652_d_n11;
        locals.var_xmp_dn12 = assign13890_e19652_d_n12;
        locals.var_xmp_dn17 = assign13890_e19652_d_n17;

        let (assign13900_e19661,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13900_e19661;

        let (assign13910_e19670,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13910_e19670;

        let (assign13920_e19679, assign13920_e19679_d_n0, assign13920_e19679_d_n2, assign13920_e19679_d_n6, assign13920_e19679_d_n7, assign13920_e19679_d_n10, assign13920_e19679_d_n11, assign13920_e19679_d_n12, assign13920_e19679_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13920_e19679;
        locals.var_arg_dn0 = assign13920_e19679_d_n0;
        locals.var_arg_dn2 = assign13920_e19679_d_n2;
        locals.var_arg_dn6 = assign13920_e19679_d_n6;
        locals.var_arg_dn7 = assign13920_e19679_d_n7;
        locals.var_arg_dn10 = assign13920_e19679_d_n10;
        locals.var_arg_dn11 = assign13920_e19679_d_n11;
        locals.var_arg_dn12 = assign13920_e19679_d_n12;
        locals.var_arg_dn17 = assign13920_e19679_d_n17;

    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13930_e19688, assign13930_e19688_d_n0, assign13930_e19688_d_n2, assign13930_e19688_d_n6, assign13930_e19688_d_n7, assign13930_e19688_d_n10, assign13930_e19688_d_n11, assign13930_e19688_d_n12, assign13930_e19688_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13930_e19688;
        locals.var_dnm_dn0 = assign13930_e19688_d_n0;
        locals.var_dnm_dn2 = assign13930_e19688_d_n2;
        locals.var_dnm_dn6 = assign13930_e19688_d_n6;
        locals.var_dnm_dn7 = assign13930_e19688_d_n7;
        locals.var_dnm_dn10 = assign13930_e19688_d_n10;
        locals.var_dnm_dn11 = assign13930_e19688_d_n11;
        locals.var_dnm_dn12 = assign13930_e19688_d_n12;
        locals.var_dnm_dn17 = assign13930_e19688_d_n17;

        let (assign13940_e19699, assign13940_e19699_d_n0, assign13940_e19699_d_n2, assign13940_e19699_d_n6, assign13940_e19699_d_n7, assign13940_e19699_d_n10, assign13940_e19699_d_n11, assign13940_e19699_d_n12, assign13940_e19699_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13940_e19697: f64 = (locals.var_xp * locals.var_x2);
        (assign13940_e19697, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13940_e19699;
        locals.var_xp_dn0 = assign13940_e19699_d_n0;
        locals.var_xp_dn2 = assign13940_e19699_d_n2;
        locals.var_xp_dn6 = assign13940_e19699_d_n6;
        locals.var_xp_dn7 = assign13940_e19699_d_n7;
        locals.var_xp_dn10 = assign13940_e19699_d_n10;
        locals.var_xp_dn11 = assign13940_e19699_d_n11;
        locals.var_xp_dn12 = assign13940_e19699_d_n12;
        locals.var_xp_dn17 = assign13940_e19699_d_n17;

        let (assign13950_e19710, assign13950_e19710_d_n0, assign13950_e19710_d_n2, assign13950_e19710_d_n6, assign13950_e19710_d_n7, assign13950_e19710_d_n10, assign13950_e19710_d_n11, assign13950_e19710_d_n12, assign13950_e19710_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13950_e19708: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13950_e19708, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13950_e19710;
        locals.var_xmp_dn0 = assign13950_e19710_d_n0;
        locals.var_xmp_dn2 = assign13950_e19710_d_n2;
        locals.var_xmp_dn6 = assign13950_e19710_d_n6;
        locals.var_xmp_dn7 = assign13950_e19710_d_n7;
        locals.var_xmp_dn10 = assign13950_e19710_d_n10;
        locals.var_xmp_dn11 = assign13950_e19710_d_n11;
        locals.var_xmp_dn12 = assign13950_e19710_d_n12;
        locals.var_xmp_dn17 = assign13950_e19710_d_n17;

        let (assign13960_e19721, assign13960_e19721_d_n0, assign13960_e19721_d_n2, assign13960_e19721_d_n6, assign13960_e19721_d_n7, assign13960_e19721_d_n10, assign13960_e19721_d_n11, assign13960_e19721_d_n12, assign13960_e19721_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13960_e19719: f64 = (locals.var_xp * locals.var_x2);
        (assign13960_e19719, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13960_e19721;
        locals.var_xp_dn0 = assign13960_e19721_d_n0;
        locals.var_xp_dn2 = assign13960_e19721_d_n2;
        locals.var_xp_dn6 = assign13960_e19721_d_n6;
        locals.var_xp_dn7 = assign13960_e19721_d_n7;
        locals.var_xp_dn10 = assign13960_e19721_d_n10;
        locals.var_xp_dn11 = assign13960_e19721_d_n11;
        locals.var_xp_dn12 = assign13960_e19721_d_n12;
        locals.var_xp_dn17 = assign13960_e19721_d_n17;

        let (assign13970_e19732, assign13970_e19732_d_n0, assign13970_e19732_d_n2, assign13970_e19732_d_n6, assign13970_e19732_d_n7, assign13970_e19732_d_n10, assign13970_e19732_d_n11, assign13970_e19732_d_n12, assign13970_e19732_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13970_e19730: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13970_e19730, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13970_e19732;
        locals.var_xmp_dn0 = assign13970_e19732_d_n0;
        locals.var_xmp_dn2 = assign13970_e19732_d_n2;
        locals.var_xmp_dn6 = assign13970_e19732_d_n6;
        locals.var_xmp_dn7 = assign13970_e19732_d_n7;
        locals.var_xmp_dn10 = assign13970_e19732_d_n10;
        locals.var_xmp_dn11 = assign13970_e19732_d_n11;
        locals.var_xmp_dn12 = assign13970_e19732_d_n12;
        locals.var_xmp_dn17 = assign13970_e19732_d_n17;

        let (assign13980_e19743, assign13980_e19743_d_n0, assign13980_e19743_d_n2, assign13980_e19743_d_n6, assign13980_e19743_d_n7, assign13980_e19743_d_n10, assign13980_e19743_d_n11, assign13980_e19743_d_n12, assign13980_e19743_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13980_e19741: f64 = (locals.var_xp + locals.var_xmp);
        (assign13980_e19741, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13980_e19743;
        locals.var_arg_dn0 = assign13980_e19743_d_n0;
        locals.var_arg_dn2 = assign13980_e19743_d_n2;
        locals.var_arg_dn6 = assign13980_e19743_d_n6;
        locals.var_arg_dn7 = assign13980_e19743_d_n7;
        locals.var_arg_dn10 = assign13980_e19743_d_n10;
        locals.var_arg_dn11 = assign13980_e19743_d_n11;
        locals.var_arg_dn12 = assign13980_e19743_d_n12;
        locals.var_arg_dn17 = assign13980_e19743_d_n17;

        let (assign13990_e19752, assign13990_e19752_d_n0, assign13990_e19752_d_n2, assign13990_e19752_d_n6, assign13990_e19752_d_n7, assign13990_e19752_d_n10, assign13990_e19752_d_n11, assign13990_e19752_d_n12, assign13990_e19752_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13990_e19752;
        locals.var_dnm_dn0 = assign13990_e19752_d_n0;
        locals.var_dnm_dn2 = assign13990_e19752_d_n2;
        locals.var_dnm_dn6 = assign13990_e19752_d_n6;
        locals.var_dnm_dn7 = assign13990_e19752_d_n7;
        locals.var_dnm_dn10 = assign13990_e19752_d_n10;
        locals.var_dnm_dn11 = assign13990_e19752_d_n11;
        locals.var_dnm_dn12 = assign13990_e19752_d_n12;
        locals.var_dnm_dn17 = assign13990_e19752_d_n17;

        let assign14000_e19767: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard428 = assign14000_e19767;

        let assign14010_e19770: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign14010_e19770;

        let (assign14020_e19783,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_guard429 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14020_e19783;

        let assign14030_e19786: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign14030_e19786;

        let (assign14040_e19802,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14040_e19802;

        let assign14050_e19805: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign14050_e19805;

        let (assign14060_e19824,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14060_e19824;

        let assign14070_e19827: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign14070_e19827;

        let (assign14080_e19849,) = {
    if ((((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14080_e19849;

        let (assign14090_e19860,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign14090_e19860;

        let mut assign14100_loop_guard: usize = 0;
        while {
            let assign14100_cond_e19872: f64 = if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign14100_cond_e19872 != 0.0
        } {
            assign14100_loop_guard += 1;
            assert!(assign14100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign14100_body0_e19884, assign14100_body0_e19884_d_n0, assign14100_body0_e19884_d_n2, assign14100_body0_e19884_d_n6, assign14100_body0_e19884_d_n7, assign14100_body0_e19884_d_n10, assign14100_body0_e19884_d_n11, assign14100_body0_e19884_d_n12, assign14100_body0_e19884_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) {
        let assign14100_body0_e19882: f64 = (locals.var_dnm).sqrt();
        (assign14100_body0_e19882, (locals.var_dnm_dn0 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn2 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn6 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn7 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn10 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn11 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn12 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn17 / (2.0 * assign14100_body0_e19882)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign14100_body0_e19884;
            locals.var_dnm_dn0 = assign14100_body0_e19884_d_n0;
            locals.var_dnm_dn2 = assign14100_body0_e19884_d_n2;
            locals.var_dnm_dn6 = assign14100_body0_e19884_d_n6;
            locals.var_dnm_dn7 = assign14100_body0_e19884_d_n7;
            locals.var_dnm_dn10 = assign14100_body0_e19884_d_n10;
            locals.var_dnm_dn11 = assign14100_body0_e19884_d_n11;
            locals.var_dnm_dn12 = assign14100_body0_e19884_d_n12;
            locals.var_dnm_dn17 = assign14100_body0_e19884_d_n17;
            let (assign14100_body1_e19897,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) {
        let assign14100_body1_e19895: f64 = (locals.var_m0 + 1.0);
        (assign14100_body1_e19895,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign14100_body1_e19897;
        }

        let (assign14110_e19915, assign14110_e19915_d_n0, assign14110_e19915_d_n2, assign14110_e19915_d_n6, assign14110_e19915_d_n7, assign14110_e19915_d_n10, assign14110_e19915_d_n11, assign14110_e19915_d_n12, assign14110_e19915_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 == 0.0)) {
        let assign14110_e19911: f64 = (2.0 * 2.0);
        let assign14110_e19912: f64 = (1.0 / assign14110_e19911);
        let assign14110_e19913: f64 = (locals.var_dnm).powf(assign14110_e19912);
        (assign14110_e19913, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn0)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn2)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn6)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn7)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn10)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn11)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn12)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn17)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14110_e19915;
        locals.var_dnm_dn0 = assign14110_e19915_d_n0;
        locals.var_dnm_dn2 = assign14110_e19915_d_n2;
        locals.var_dnm_dn6 = assign14110_e19915_d_n6;
        locals.var_dnm_dn7 = assign14110_e19915_d_n7;
        locals.var_dnm_dn10 = assign14110_e19915_d_n10;
        locals.var_dnm_dn11 = assign14110_e19915_d_n11;
        locals.var_dnm_dn12 = assign14110_e19915_d_n12;
        locals.var_dnm_dn17 = assign14110_e19915_d_n17;

        let (assign14120_e19926, assign14120_e19926_d_n0, assign14120_e19926_d_n2, assign14120_e19926_d_n6, assign14120_e19926_d_n7, assign14120_e19926_d_n10, assign14120_e19926_d_n11, assign14120_e19926_d_n12, assign14120_e19926_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign14120_e19924: f64 = (1.0 / locals.var_dnm);
        (assign14120_e19924, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14120_e19926;
        locals.var_dnm_dn0 = assign14120_e19926_d_n0;
        locals.var_dnm_dn2 = assign14120_e19926_d_n2;
        locals.var_dnm_dn6 = assign14120_e19926_d_n6;
        locals.var_dnm_dn7 = assign14120_e19926_d_n7;
        locals.var_dnm_dn10 = assign14120_e19926_d_n10;
        locals.var_dnm_dn11 = assign14120_e19926_d_n11;
        locals.var_dnm_dn12 = assign14120_e19926_d_n12;
        locals.var_dnm_dn17 = assign14120_e19926_d_n17;

        let (assign14130_e19939, assign14130_e19939_d_n0, assign14130_e19939_d_n2, assign14130_e19939_d_n6, assign14130_e19939_d_n7, assign14130_e19939_d_n10, assign14130_e19939_d_n11, assign14130_e19939_d_n12, assign14130_e19939_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign14130_e19935: f64 = (locals.var_tmf1 * 1e-5);
        let assign14130_e19937: f64 = (assign14130_e19935 * locals.var_dnm);
        (assign14130_e19937, (((locals.var_tmf1_dn0 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign14130_e19939;
        locals.var_tmf0_dn0 = assign14130_e19939_d_n0;
        locals.var_tmf0_dn2 = assign14130_e19939_d_n2;
        locals.var_tmf0_dn6 = assign14130_e19939_d_n6;
        locals.var_tmf0_dn7 = assign14130_e19939_d_n7;
        locals.var_tmf0_dn10 = assign14130_e19939_d_n10;
        locals.var_tmf0_dn11 = assign14130_e19939_d_n11;
        locals.var_tmf0_dn12 = assign14130_e19939_d_n12;
        locals.var_tmf0_dn17 = assign14130_e19939_d_n17;

        let (assign14140_e19952, assign14140_e19952_d_n0, assign14140_e19952_d_n2, assign14140_e19952_d_n6, assign14140_e19952_d_n7, assign14140_e19952_d_n10, assign14140_e19952_d_n11, assign14140_e19952_d_n12, assign14140_e19952_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign14140_e19948: f64 = 1e-5;
        let assign14140_e19950: f64 = (assign14140_e19948 - locals.var_tmf0);
        (assign14140_e19950, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14140_e19952;
        locals.var_rrr_eta_dn0 = assign14140_e19952_d_n0;
        locals.var_rrr_eta_dn2 = assign14140_e19952_d_n2;
        locals.var_rrr_eta_dn6 = assign14140_e19952_d_n6;
        locals.var_rrr_eta_dn7 = assign14140_e19952_d_n7;
        locals.var_rrr_eta_dn10 = assign14140_e19952_d_n10;
        locals.var_rrr_eta_dn11 = assign14140_e19952_d_n11;
        locals.var_rrr_eta_dn12 = assign14140_e19952_d_n12;
        locals.var_rrr_eta_dn17 = assign14140_e19952_d_n17;

        let (assign14150_e19962, assign14150_e19962_d_n0, assign14150_e19962_d_n2, assign14150_e19962_d_n6, assign14150_e19962_d_n7, assign14150_e19962_d_n10, assign14150_e19962_d_n11, assign14150_e19962_d_n12, assign14150_e19962_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14150_e19962;
        locals.var_rrr_eta_dn0 = assign14150_e19962_d_n0;
        locals.var_rrr_eta_dn2 = assign14150_e19962_d_n2;
        locals.var_rrr_eta_dn6 = assign14150_e19962_d_n6;
        locals.var_rrr_eta_dn7 = assign14150_e19962_d_n7;
        locals.var_rrr_eta_dn10 = assign14150_e19962_d_n10;
        locals.var_rrr_eta_dn11 = assign14150_e19962_d_n11;
        locals.var_rrr_eta_dn12 = assign14150_e19962_d_n12;
        locals.var_rrr_eta_dn17 = assign14150_e19962_d_n17;

        let (assign14160_e19969, assign14160_e19969_d_n0, assign14160_e19969_d_n2, assign14160_e19969_d_n6, assign14160_e19969_d_n7, assign14160_e19969_d_n10, assign14160_e19969_d_n11, assign14160_e19969_d_n12, assign14160_e19969_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign14160_e19969;
        locals.var_alpha_dn0 = assign14160_e19969_d_n0;
        locals.var_alpha_dn2 = assign14160_e19969_d_n2;
        locals.var_alpha_dn6 = assign14160_e19969_d_n6;
        locals.var_alpha_dn7 = assign14160_e19969_d_n7;
        locals.var_alpha_dn10 = assign14160_e19969_d_n10;
        locals.var_alpha_dn11 = assign14160_e19969_d_n11;
        locals.var_alpha_dn12 = assign14160_e19969_d_n12;
        locals.var_alpha_dn17 = assign14160_e19969_d_n17;

        let (assign14170_e19982, assign14170_e19982_d_n0, assign14170_e19982_d_n2, assign14170_e19982_d_n6, assign14170_e19982_d_n7, assign14170_e19982_d_n10, assign14170_e19982_d_n11, assign14170_e19982_d_n12, assign14170_e19982_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign14170_e19978: f64 = (1.0 + locals.var_alpha);
        let assign14170_e19979: f64 = (locals.var_alpha * assign14170_e19978);
        let assign14170_e19980: f64 = (1.0 + assign14170_e19979);
        (assign14170_e19980, ((locals.var_alpha_dn0 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign14170_e19982;
        locals.var_qinm_dn0 = assign14170_e19982_d_n0;
        locals.var_qinm_dn2 = assign14170_e19982_d_n2;
        locals.var_qinm_dn6 = assign14170_e19982_d_n6;
        locals.var_qinm_dn7 = assign14170_e19982_d_n7;
        locals.var_qinm_dn10 = assign14170_e19982_d_n10;
        locals.var_qinm_dn11 = assign14170_e19982_d_n11;
        locals.var_qinm_dn12 = assign14170_e19982_d_n12;
        locals.var_qinm_dn17 = assign14170_e19982_d_n17;

        let (assign14180_e20002, assign14180_e20002_d_n0, assign14180_e20002_d_n2, assign14180_e20002_d_n6, assign14180_e20002_d_n7, assign14180_e20002_d_n10, assign14180_e20002_d_n11, assign14180_e20002_d_n12, assign14180_e20002_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign14180_e19989: f64 = (1.0 + locals.var_alpha);
        let assign14180_e19992: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14180_e20000, assign14180_e20000_d_n0, assign14180_e20000_d_n2, assign14180_e20000_d_n6, assign14180_e20000_d_n7, assign14180_e20000_d_n10, assign14180_e20000_d_n11, assign14180_e20000_d_n12, assign14180_e20000_d_n17,) = {
            if (assign14180_e19989 >= assign14180_e19992) {
                let assign14180_e19996: f64 = (1.0 + locals.var_alpha);
                (assign14180_e19996, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign14180_e19999: f64 = (10.0 * 2.220446049250313e-16);
                (assign14180_e19999, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14180_e20000, assign14180_e20000_d_n0, assign14180_e20000_d_n2, assign14180_e20000_d_n6, assign14180_e20000_d_n7, assign14180_e20000_d_n10, assign14180_e20000_d_n11, assign14180_e20000_d_n12, assign14180_e20000_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign14180_e20002;
        locals.var_qidn_dn0 = assign14180_e20002_d_n0;
        locals.var_qidn_dn2 = assign14180_e20002_d_n2;
        locals.var_qidn_dn6 = assign14180_e20002_d_n6;
        locals.var_qidn_dn7 = assign14180_e20002_d_n7;
        locals.var_qidn_dn10 = assign14180_e20002_d_n10;
        locals.var_qidn_dn11 = assign14180_e20002_d_n11;
        locals.var_qidn_dn12 = assign14180_e20002_d_n12;
        locals.var_qidn_dn17 = assign14180_e20002_d_n17;

        let (assign14190_e20014, assign14190_e20014_d_n0, assign14190_e20014_d_n2, assign14190_e20014_d_n6, assign14190_e20014_d_n7, assign14190_e20014_d_n10, assign14190_e20014_d_n11, assign14190_e20014_d_n12, assign14190_e20014_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign14190_e20008: f64 = (-0.5);
        let assign14190_e20011: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign14190_e20012: f64 = (assign14190_e20008 * assign14190_e20011);
        (assign14190_e20012, (assign14190_e20008 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign14190_e20008 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign14190_e20008 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign14190_e20008 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign14190_e20008 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign14190_e20008 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign14190_e20008 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign14190_e20008 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign14190_e20014;
        locals.var_qiu_dn0 = assign14190_e20014_d_n0;
        locals.var_qiu_dn2 = assign14190_e20014_d_n2;
        locals.var_qiu_dn6 = assign14190_e20014_d_n6;
        locals.var_qiu_dn7 = assign14190_e20014_d_n7;
        locals.var_qiu_dn10 = assign14190_e20014_d_n10;
        locals.var_qiu_dn11 = assign14190_e20014_d_n11;
        locals.var_qiu_dn12 = assign14190_e20014_d_n12;
        locals.var_qiu_dn17 = assign14190_e20014_d_n17;

        let (assign14260_e20047, assign14260_e20047_d_n0, assign14260_e20047_d_n2, assign14260_e20047_d_n6, assign14260_e20047_d_n7, assign14260_e20047_d_n10, assign14260_e20047_d_n11, assign14260_e20047_d_n12, assign14260_e20047_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    } else {
        (locals.var_vbcs_cl, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    }
};
        locals.var_vbcs_cl = assign14260_e20047;
        locals.var_vbcs_cl_dn0 = assign14260_e20047_d_n0;
        locals.var_vbcs_cl_dn2 = assign14260_e20047_d_n2;
        locals.var_vbcs_cl_dn6 = assign14260_e20047_d_n6;
        locals.var_vbcs_cl_dn7 = assign14260_e20047_d_n7;
        locals.var_vbcs_cl_dn10 = assign14260_e20047_d_n10;
        locals.var_vbcs_cl_dn11 = assign14260_e20047_d_n11;
        locals.var_vbcs_cl_dn12 = assign14260_e20047_d_n12;
        locals.var_vbcs_cl_dn17 = assign14260_e20047_d_n17;

        let assign14270_e20050: f64 = if locals.var_wdsoi_ini < p.p237 { 1.0 } else { 0.0 };
        locals.var_guard439 = assign14270_e20050;

        let (assign14280_e20057,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard439 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14280_e20057;

        let (assign14290_e20065,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard439 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14290_e20065;

        let (assign14300_e20076,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14300_e20070: f64 = (locals.var_vfb - locals.var_dvth);
        let assign14300_e20072: f64 = (assign14300_e20070 + locals.var_dppg);
        let assign14300_e20074: f64 = (assign14300_e20072 + locals.var_vbcs_cl);
        (assign14300_e20074,)
    } else {
        (locals.var_vgs_fb,)
    }
};
        locals.var_vgs_fb = assign14300_e20076;

        let assign14310_e20079: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard440 = assign14310_e20079;

        let (assign14320_e20087,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14320_e20085: f64 = (-1.0);
        (assign14320_e20085,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign14320_e20087;

        let (assign14330_e20102, assign14330_e20102_d_n0, assign14330_e20102_d_n2, assign14330_e20102_d_n6, assign14330_e20102_d_n7, assign14330_e20102_d_n10, assign14330_e20102_d_n11, assign14330_e20102_d_n12, assign14330_e20102_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14330_e20094: f64 = (2.0 * locals.var_beta_inv);
        let assign14330_e20096: f64 = (-locals.var_vgs_min);
        let assign14330_e20098: f64 = (assign14330_e20096 / locals.var_fac1);
        let assign14330_e20099: f64 = (assign14330_e20098).ln();
        let assign14330_e20100: f64 = (assign14330_e20094 * assign14330_e20099);
        (assign14330_e20100, (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (((2.0 * locals.var_beta_inv_dn10) * assign14330_e20099) + (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098))), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign14330_e20102;
        locals.var_ps0_min_dn0 = assign14330_e20102_d_n0;
        locals.var_ps0_min_dn2 = assign14330_e20102_d_n2;
        locals.var_ps0_min_dn6 = assign14330_e20102_d_n6;
        locals.var_ps0_min_dn7 = assign14330_e20102_d_n7;
        locals.var_ps0_min_dn10 = assign14330_e20102_d_n10;
        locals.var_ps0_min_dn11 = assign14330_e20102_d_n11;
        locals.var_ps0_min_dn12 = assign14330_e20102_d_n12;
        locals.var_ps0_min_dn17 = assign14330_e20102_d_n17;

        let (assign14340_e20113, assign14340_e20113_d_n0, assign14340_e20113_d_n2, assign14340_e20113_d_n6, assign14340_e20113_d_n7, assign14340_e20113_d_n10, assign14340_e20113_d_n11, assign14340_e20113_d_n12, assign14340_e20113_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14340_e20110: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14340_e20111: f64 = (locals.var_beta * assign14340_e20110);
        (assign14340_e20111, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14340_e20110) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14340_e20113;
        locals.var_tx_dn0 = assign14340_e20113_d_n0;
        locals.var_tx_dn2 = assign14340_e20113_d_n2;
        locals.var_tx_dn6 = assign14340_e20113_d_n6;
        locals.var_tx_dn7 = assign14340_e20113_d_n7;
        locals.var_tx_dn10 = assign14340_e20113_d_n10;
        locals.var_tx_dn11 = assign14340_e20113_d_n11;
        locals.var_tx_dn12 = assign14340_e20113_d_n12;
        locals.var_tx_dn17 = assign14340_e20113_d_n17;

    }

    pub(super) fn stamp_transient_block_46(
        locals: &mut StampLocals,
    ) {
        let (assign14350_e20124, assign14350_e20124_d_n0, assign14350_e20124_d_n2, assign14350_e20124_d_n6, assign14350_e20124_d_n7, assign14350_e20124_d_n10, assign14350_e20124_d_n11, assign14350_e20124_d_n12, assign14350_e20124_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14350_e20121: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign14350_e20122: f64 = (1.0 / assign14350_e20121);
        (assign14350_e20122, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign14350_e20121 * assign14350_e20121))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign14350_e20121 * assign14350_e20121))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14350_e20124;
        locals.var_t1_dn0 = assign14350_e20124_d_n0;
        locals.var_t1_dn2 = assign14350_e20124_d_n2;
        locals.var_t1_dn6 = assign14350_e20124_d_n6;
        locals.var_t1_dn7 = assign14350_e20124_d_n7;
        locals.var_t1_dn10 = assign14350_e20124_d_n10;
        locals.var_t1_dn11 = assign14350_e20124_d_n11;
        locals.var_t1_dn12 = assign14350_e20124_d_n12;
        locals.var_t1_dn17 = assign14350_e20124_d_n17;

        let (assign14360_e20133, assign14360_e20133_d_n0, assign14360_e20133_d_n2, assign14360_e20133_d_n6, assign14360_e20133_d_n7, assign14360_e20133_d_n10, assign14360_e20133_d_n11, assign14360_e20133_d_n12, assign14360_e20133_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14360_e20131: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign14360_e20131, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14360_e20133;
        locals.var_ty_dn0 = assign14360_e20133_d_n0;
        locals.var_ty_dn2 = assign14360_e20133_d_n2;
        locals.var_ty_dn6 = assign14360_e20133_d_n6;
        locals.var_ty_dn7 = assign14360_e20133_d_n7;
        locals.var_ty_dn10 = assign14360_e20133_d_n10;
        locals.var_ty_dn11 = assign14360_e20133_d_n11;
        locals.var_ty_dn12 = assign14360_e20133_d_n12;
        locals.var_ty_dn17 = assign14360_e20133_d_n17;

        let (assign14370_e20146, assign14370_e20146_d_n0, assign14370_e20146_d_n2, assign14370_e20146_d_n6, assign14370_e20146_d_n7, assign14370_e20146_d_n10, assign14370_e20146_d_n11, assign14370_e20146_d_n12, assign14370_e20146_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14370_e20141: f64 = (3.0 * 1.414213562373095);
        let assign14370_e20143: f64 = (assign14370_e20141 * locals.var_ty);
        let assign14370_e20144: f64 = (2.0 + assign14370_e20143);
        (assign14370_e20144, (assign14370_e20141 * locals.var_ty_dn0), (assign14370_e20141 * locals.var_ty_dn2), (assign14370_e20141 * locals.var_ty_dn6), (assign14370_e20141 * locals.var_ty_dn7), (assign14370_e20141 * locals.var_ty_dn10), (assign14370_e20141 * locals.var_ty_dn11), (assign14370_e20141 * locals.var_ty_dn12), (assign14370_e20141 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign14370_e20146;
        locals.var_ac41_dn0 = assign14370_e20146_d_n0;
        locals.var_ac41_dn2 = assign14370_e20146_d_n2;
        locals.var_ac41_dn6 = assign14370_e20146_d_n6;
        locals.var_ac41_dn7 = assign14370_e20146_d_n7;
        locals.var_ac41_dn10 = assign14370_e20146_d_n10;
        locals.var_ac41_dn11 = assign14370_e20146_d_n11;
        locals.var_ac41_dn12 = assign14370_e20146_d_n12;
        locals.var_ac41_dn17 = assign14370_e20146_d_n17;

        let (assign14380_e20159, assign14380_e20159_d_n0, assign14380_e20159_d_n2, assign14380_e20159_d_n6, assign14380_e20159_d_n7, assign14380_e20159_d_n10, assign14380_e20159_d_n11, assign14380_e20159_d_n12, assign14380_e20159_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14380_e20153: f64 = (8.0 * locals.var_ac41);
        let assign14380_e20155: f64 = (assign14380_e20153 * locals.var_ac41);
        let assign14380_e20157: f64 = (assign14380_e20155 * locals.var_ac41);
        (assign14380_e20157, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign14380_e20159;
        locals.var_ac4_dn0 = assign14380_e20159_d_n0;
        locals.var_ac4_dn2 = assign14380_e20159_d_n2;
        locals.var_ac4_dn6 = assign14380_e20159_d_n6;
        locals.var_ac4_dn7 = assign14380_e20159_d_n7;
        locals.var_ac4_dn10 = assign14380_e20159_d_n10;
        locals.var_ac4_dn11 = assign14380_e20159_d_n11;
        locals.var_ac4_dn12 = assign14380_e20159_d_n12;
        locals.var_ac4_dn17 = assign14380_e20159_d_n17;

        let (assign14390_e20168, assign14390_e20168_d_n0, assign14390_e20168_d_n2, assign14390_e20168_d_n6, assign14390_e20168_d_n7, assign14390_e20168_d_n10, assign14390_e20168_d_n11, assign14390_e20168_d_n12, assign14390_e20168_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14390_e20166: f64 = (locals.var_tx - 2.0);
        (assign14390_e20166, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14390_e20168;
        locals.var_t4_dn0 = assign14390_e20168_d_n0;
        locals.var_t4_dn2 = assign14390_e20168_d_n2;
        locals.var_t4_dn6 = assign14390_e20168_d_n6;
        locals.var_t4_dn7 = assign14390_e20168_d_n7;
        locals.var_t4_dn10 = assign14390_e20168_d_n10;
        locals.var_t4_dn11 = assign14390_e20168_d_n11;
        locals.var_t4_dn12 = assign14390_e20168_d_n12;
        locals.var_t4_dn17 = assign14390_e20168_d_n17;

        let (assign14400_e20179, assign14400_e20179_d_n0, assign14400_e20179_d_n2, assign14400_e20179_d_n6, assign14400_e20179_d_n7, assign14400_e20179_d_n10, assign14400_e20179_d_n11, assign14400_e20179_d_n12, assign14400_e20179_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14400_e20175: f64 = (9.0 * locals.var_ty);
        let assign14400_e20177: f64 = (assign14400_e20175 * locals.var_t4);
        (assign14400_e20177, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14400_e20179;
        locals.var_t5_dn0 = assign14400_e20179_d_n0;
        locals.var_t5_dn2 = assign14400_e20179_d_n2;
        locals.var_t5_dn6 = assign14400_e20179_d_n6;
        locals.var_t5_dn7 = assign14400_e20179_d_n7;
        locals.var_t5_dn10 = assign14400_e20179_d_n10;
        locals.var_t5_dn11 = assign14400_e20179_d_n11;
        locals.var_t5_dn12 = assign14400_e20179_d_n12;
        locals.var_t5_dn17 = assign14400_e20179_d_n17;

        let (assign14410_e20190, assign14410_e20190_d_n0, assign14410_e20190_d_n2, assign14410_e20190_d_n6, assign14410_e20190_d_n7, assign14410_e20190_d_n10, assign14410_e20190_d_n11, assign14410_e20190_d_n12, assign14410_e20190_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14410_e20186: f64 = (7.0 * 1.414213562373095);
        let assign14410_e20188: f64 = (assign14410_e20186 - locals.var_t5);
        (assign14410_e20188, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign14410_e20190;
        locals.var_ac31_dn0 = assign14410_e20190_d_n0;
        locals.var_ac31_dn2 = assign14410_e20190_d_n2;
        locals.var_ac31_dn6 = assign14410_e20190_d_n6;
        locals.var_ac31_dn7 = assign14410_e20190_d_n7;
        locals.var_ac31_dn10 = assign14410_e20190_d_n10;
        locals.var_ac31_dn11 = assign14410_e20190_d_n11;
        locals.var_ac31_dn12 = assign14410_e20190_d_n12;
        locals.var_ac31_dn17 = assign14410_e20190_d_n17;

        let (assign14420_e20199, assign14420_e20199_d_n0, assign14420_e20199_d_n2, assign14420_e20199_d_n6, assign14420_e20199_d_n7, assign14420_e20199_d_n10, assign14420_e20199_d_n11, assign14420_e20199_d_n12, assign14420_e20199_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14420_e20197: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign14420_e20197, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign14420_e20199;
        locals.var_ac3_dn0 = assign14420_e20199_d_n0;
        locals.var_ac3_dn2 = assign14420_e20199_d_n2;
        locals.var_ac3_dn6 = assign14420_e20199_d_n6;
        locals.var_ac3_dn7 = assign14420_e20199_d_n7;
        locals.var_ac3_dn10 = assign14420_e20199_d_n10;
        locals.var_ac3_dn11 = assign14420_e20199_d_n11;
        locals.var_ac3_dn12 = assign14420_e20199_d_n12;
        locals.var_ac3_dn17 = assign14420_e20199_d_n17;

        let assign14430_e20203: f64 = (locals.var_ac3 * 1e-8);
        let assign14430_e20204: f64 = if locals.var_ac4 < assign14430_e20203 { 1.0 } else { 0.0 };
        locals.var_guard441 = assign14430_e20204;

        let (assign14440_e20226, assign14440_e20226_d_n0, assign14440_e20226_d_n2, assign14440_e20226_d_n6, assign14440_e20226_d_n7, assign14440_e20226_d_n10, assign14440_e20226_d_n11, assign14440_e20226_d_n12, assign14440_e20226_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) && (locals.var_guard441 != 0.0)) {
        let assign14440_e20212: f64 = (-7.0);
        let assign14440_e20214: f64 = (assign14440_e20212 * 1.414213562373095);
        let assign14440_e20216: f64 = (assign14440_e20214 + locals.var_ac31);
        let assign14440_e20219: f64 = (0.5 * locals.var_ac4);
        let assign14440_e20221: f64 = (assign14440_e20219 / locals.var_ac31);
        let assign14440_e20222: f64 = (assign14440_e20216 + assign14440_e20221);
        let assign14440_e20224: f64 = (assign14440_e20222 + locals.var_t5);
        (assign14440_e20224, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14440_e20226;
        locals.var_ac1_dn0 = assign14440_e20226_d_n0;
        locals.var_ac1_dn2 = assign14440_e20226_d_n2;
        locals.var_ac1_dn6 = assign14440_e20226_d_n6;
        locals.var_ac1_dn7 = assign14440_e20226_d_n7;
        locals.var_ac1_dn10 = assign14440_e20226_d_n10;
        locals.var_ac1_dn11 = assign14440_e20226_d_n11;
        locals.var_ac1_dn12 = assign14440_e20226_d_n12;
        locals.var_ac1_dn17 = assign14440_e20226_d_n17;

        let (assign14450_e20239, assign14450_e20239_d_n0, assign14450_e20239_d_n2, assign14450_e20239_d_n6, assign14450_e20239_d_n7, assign14450_e20239_d_n10, assign14450_e20239_d_n11, assign14450_e20239_d_n12, assign14450_e20239_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) && (locals.var_guard441 == 0.0)) {
        let assign14450_e20236: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign14450_e20237: f64 = (assign14450_e20236).sqrt();
        (assign14450_e20237, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign14450_e20237)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign14450_e20239;
        locals.var_ac2_dn0 = assign14450_e20239_d_n0;
        locals.var_ac2_dn2 = assign14450_e20239_d_n2;
        locals.var_ac2_dn6 = assign14450_e20239_d_n6;
        locals.var_ac2_dn7 = assign14450_e20239_d_n7;
        locals.var_ac2_dn10 = assign14450_e20239_d_n10;
        locals.var_ac2_dn11 = assign14450_e20239_d_n11;
        locals.var_ac2_dn12 = assign14450_e20239_d_n12;
        locals.var_ac2_dn17 = assign14450_e20239_d_n17;

        let (assign14460_e20256, assign14460_e20256_d_n0, assign14460_e20256_d_n2, assign14460_e20256_d_n6, assign14460_e20256_d_n7, assign14460_e20256_d_n10, assign14460_e20256_d_n11, assign14460_e20256_d_n12, assign14460_e20256_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) && (locals.var_guard441 == 0.0)) {
        let assign14460_e20248: f64 = (-7.0);
        let assign14460_e20250: f64 = (assign14460_e20248 * 1.414213562373095);
        let assign14460_e20252: f64 = (assign14460_e20250 + locals.var_ac2);
        let assign14460_e20254: f64 = (assign14460_e20252 + locals.var_t5);
        (assign14460_e20254, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14460_e20256;
        locals.var_ac1_dn0 = assign14460_e20256_d_n0;
        locals.var_ac1_dn2 = assign14460_e20256_d_n2;
        locals.var_ac1_dn6 = assign14460_e20256_d_n6;
        locals.var_ac1_dn7 = assign14460_e20256_d_n7;
        locals.var_ac1_dn10 = assign14460_e20256_d_n10;
        locals.var_ac1_dn11 = assign14460_e20256_d_n11;
        locals.var_ac1_dn12 = assign14460_e20256_d_n12;
        locals.var_ac1_dn17 = assign14460_e20256_d_n17;

        let (assign14470_e20265, assign14470_e20265_d_n0, assign14470_e20265_d_n2, assign14470_e20265_d_n6, assign14470_e20265_d_n7, assign14470_e20265_d_n10, assign14470_e20265_d_n11, assign14470_e20265_d_n12, assign14470_e20265_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14470_e20263: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign14470_e20263, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign14470_e20265;
        locals.var_acd_dn0 = assign14470_e20265_d_n0;
        locals.var_acd_dn2 = assign14470_e20265_d_n2;
        locals.var_acd_dn6 = assign14470_e20265_d_n6;
        locals.var_acd_dn7 = assign14470_e20265_d_n7;
        locals.var_acd_dn10 = assign14470_e20265_d_n10;
        locals.var_acd_dn11 = assign14470_e20265_d_n11;
        locals.var_acd_dn12 = assign14470_e20265_d_n12;
        locals.var_acd_dn17 = assign14470_e20265_d_n17;

        let (assign14480_e20289, assign14480_e20289_d_n0, assign14480_e20289_d_n2, assign14480_e20289_d_n6, assign14480_e20289_d_n7, assign14480_e20289_d_n10, assign14480_e20289_d_n11, assign14480_e20289_d_n12, assign14480_e20289_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14480_e20271: f64 = (-4.0);
        let assign14480_e20273: f64 = (assign14480_e20271 * 1.414213562373095);
        let assign14480_e20276: f64 = (12.0 * locals.var_ty);
        let assign14480_e20277: f64 = (assign14480_e20273 - assign14480_e20276);
        let assign14480_e20280: f64 = (2.0 * locals.var_acd);
        let assign14480_e20281: f64 = (assign14480_e20277 + assign14480_e20280);
        let assign14480_e20284: f64 = (1.414213562373095 * locals.var_acd);
        let assign14480_e20286: f64 = (assign14480_e20284 * locals.var_acd);
        let assign14480_e20287: f64 = (assign14480_e20281 + assign14480_e20286);
        (assign14480_e20287, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign14480_e20289;
        locals.var_acn_dn0 = assign14480_e20289_d_n0;
        locals.var_acn_dn2 = assign14480_e20289_d_n2;
        locals.var_acn_dn6 = assign14480_e20289_d_n6;
        locals.var_acn_dn7 = assign14480_e20289_d_n7;
        locals.var_acn_dn10 = assign14480_e20289_d_n10;
        locals.var_acn_dn11 = assign14480_e20289_d_n11;
        locals.var_acn_dn12 = assign14480_e20289_d_n12;
        locals.var_acn_dn17 = assign14480_e20289_d_n17;

        let (assign14490_e20298, assign14490_e20298_d_n0, assign14490_e20298_d_n2, assign14490_e20298_d_n6, assign14490_e20298_d_n7, assign14490_e20298_d_n10, assign14490_e20298_d_n11, assign14490_e20298_d_n12, assign14490_e20298_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14490_e20296: f64 = (1.0 / locals.var_acd);
        (assign14490_e20296, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14490_e20298;
        locals.var_t1_dn0 = assign14490_e20298_d_n0;
        locals.var_t1_dn2 = assign14490_e20298_d_n2;
        locals.var_t1_dn6 = assign14490_e20298_d_n6;
        locals.var_t1_dn7 = assign14490_e20298_d_n7;
        locals.var_t1_dn10 = assign14490_e20298_d_n10;
        locals.var_t1_dn11 = assign14490_e20298_d_n11;
        locals.var_t1_dn12 = assign14490_e20298_d_n12;
        locals.var_t1_dn17 = assign14490_e20298_d_n17;

        let (assign14500_e20307, assign14500_e20307_d_n0, assign14500_e20307_d_n2, assign14500_e20307_d_n6, assign14500_e20307_d_n7, assign14500_e20307_d_n10, assign14500_e20307_d_n11, assign14500_e20307_d_n12, assign14500_e20307_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14500_e20305: f64 = (locals.var_acn * locals.var_t1);
        (assign14500_e20305, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14500_e20307;
        locals.var_chi_dn0 = assign14500_e20307_d_n0;
        locals.var_chi_dn2 = assign14500_e20307_d_n2;
        locals.var_chi_dn6 = assign14500_e20307_d_n6;
        locals.var_chi_dn7 = assign14500_e20307_d_n7;
        locals.var_chi_dn10 = assign14500_e20307_d_n10;
        locals.var_chi_dn11 = assign14500_e20307_d_n11;
        locals.var_chi_dn12 = assign14500_e20307_d_n12;
        locals.var_chi_dn17 = assign14500_e20307_d_n17;

        let (assign14510_e20318, assign14510_e20318_d_n0, assign14510_e20318_d_n2, assign14510_e20318_d_n6, assign14510_e20318_d_n7, assign14510_e20318_d_n10, assign14510_e20318_d_n11, assign14510_e20318_d_n12, assign14510_e20318_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14510_e20314: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign14510_e20316: f64 = (assign14510_e20314 + locals.var_vbcs_cl);
        (assign14510_e20316, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign14510_e20318;
        locals.var_psa_dn0 = assign14510_e20318_d_n0;
        locals.var_psa_dn2 = assign14510_e20318_d_n2;
        locals.var_psa_dn6 = assign14510_e20318_d_n6;
        locals.var_psa_dn7 = assign14510_e20318_d_n7;
        locals.var_psa_dn10 = assign14510_e20318_d_n10;
        locals.var_psa_dn11 = assign14510_e20318_d_n11;
        locals.var_psa_dn12 = assign14510_e20318_d_n12;
        locals.var_psa_dn17 = assign14510_e20318_d_n17;

        let (assign14520_e20327, assign14520_e20327_d_n0, assign14520_e20327_d_n2, assign14520_e20327_d_n6, assign14520_e20327_d_n7, assign14520_e20327_d_n10, assign14520_e20327_d_n11, assign14520_e20327_d_n12, assign14520_e20327_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14520_e20325: f64 = (locals.var_psa - locals.var_vbcs_cl);
        (assign14520_e20325, (locals.var_psa_dn0 - locals.var_vbcs_cl_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14520_e20327;
        locals.var_t1_dn0 = assign14520_e20327_d_n0;
        locals.var_t1_dn2 = assign14520_e20327_d_n2;
        locals.var_t1_dn6 = assign14520_e20327_d_n6;
        locals.var_t1_dn7 = assign14520_e20327_d_n7;
        locals.var_t1_dn10 = assign14520_e20327_d_n10;
        locals.var_t1_dn11 = assign14520_e20327_d_n11;
        locals.var_t1_dn12 = assign14520_e20327_d_n12;
        locals.var_t1_dn17 = assign14520_e20327_d_n17;

        let (assign14530_e20336, assign14530_e20336_d_n0, assign14530_e20336_d_n2, assign14530_e20336_d_n6, assign14530_e20336_d_n7, assign14530_e20336_d_n10, assign14530_e20336_d_n11, assign14530_e20336_d_n12, assign14530_e20336_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14530_e20334: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign14530_e20334, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14530_e20336;
        locals.var_t2_dn0 = assign14530_e20336_d_n0;
        locals.var_t2_dn2 = assign14530_e20336_d_n2;
        locals.var_t2_dn6 = assign14530_e20336_d_n6;
        locals.var_t2_dn7 = assign14530_e20336_d_n7;
        locals.var_t2_dn10 = assign14530_e20336_d_n10;
        locals.var_t2_dn11 = assign14530_e20336_d_n11;
        locals.var_t2_dn12 = assign14530_e20336_d_n12;
        locals.var_t2_dn17 = assign14530_e20336_d_n17;

        let (assign14540_e20348, assign14540_e20348_d_n0, assign14540_e20348_d_n2, assign14540_e20348_d_n6, assign14540_e20348_d_n7, assign14540_e20348_d_n10, assign14540_e20348_d_n11, assign14540_e20348_d_n12, assign14540_e20348_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14540_e20344: f64 = (locals.var_t2 * locals.var_t2);
        let assign14540_e20345: f64 = (1.0 + assign14540_e20344);
        let assign14540_e20346: f64 = (assign14540_e20345).sqrt();
        (assign14540_e20346, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign14540_e20346)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14540_e20348;
        locals.var_t3_dn0 = assign14540_e20348_d_n0;
        locals.var_t3_dn2 = assign14540_e20348_d_n2;
        locals.var_t3_dn6 = assign14540_e20348_d_n6;
        locals.var_t3_dn7 = assign14540_e20348_d_n7;
        locals.var_t3_dn10 = assign14540_e20348_d_n10;
        locals.var_t3_dn11 = assign14540_e20348_d_n11;
        locals.var_t3_dn12 = assign14540_e20348_d_n12;
        locals.var_t3_dn17 = assign14540_e20348_d_n17;

        let (assign14550_e20359, assign14550_e20359_d_n0, assign14550_e20359_d_n2, assign14550_e20359_d_n6, assign14550_e20359_d_n7, assign14550_e20359_d_n10, assign14550_e20359_d_n11, assign14550_e20359_d_n12, assign14550_e20359_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14550_e20355: f64 = (locals.var_t1 / locals.var_t3);
        let assign14550_e20357: f64 = (assign14550_e20355 + locals.var_vbcs_cl);
        (assign14550_e20357, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14550_e20359;
        locals.var_ps0_dn0 = assign14550_e20359_d_n0;
        locals.var_ps0_dn2 = assign14550_e20359_d_n2;
        locals.var_ps0_dn6 = assign14550_e20359_d_n6;
        locals.var_ps0_dn7 = assign14550_e20359_d_n7;
        locals.var_ps0_dn10 = assign14550_e20359_d_n10;
        locals.var_ps0_dn11 = assign14550_e20359_d_n11;
        locals.var_ps0_dn12 = assign14550_e20359_d_n12;
        locals.var_ps0_dn17 = assign14550_e20359_d_n17;

        let assign14560_e20362: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard442 = assign14560_e20362;

        let (assign14570_e20372, assign14570_e20372_d_n0, assign14570_e20372_d_n2, assign14570_e20372_d_n6, assign14570_e20372_d_n7, assign14570_e20372_d_n10, assign14570_e20372_d_n11, assign14570_e20372_d_n12, assign14570_e20372_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14570_e20372;
        locals.var_phi_s0_soi_dn0 = assign14570_e20372_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14570_e20372_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14570_e20372_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14570_e20372_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14570_e20372_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14570_e20372_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14570_e20372_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14570_e20372_d_n17;

        let (assign14580_e20382, assign14580_e20382_d_n0, assign14580_e20382_d_n2, assign14580_e20382_d_n6, assign14580_e20382_d_n7, assign14580_e20382_d_n10, assign14580_e20382_d_n11, assign14580_e20382_d_n12, assign14580_e20382_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14580_e20382;
        locals.var_ps0_ini_dn0 = assign14580_e20382_d_n0;
        locals.var_ps0_ini_dn2 = assign14580_e20382_d_n2;
        locals.var_ps0_ini_dn6 = assign14580_e20382_d_n6;
        locals.var_ps0_ini_dn7 = assign14580_e20382_d_n7;
        locals.var_ps0_ini_dn10 = assign14580_e20382_d_n10;
        locals.var_ps0_ini_dn11 = assign14580_e20382_d_n11;
        locals.var_ps0_ini_dn12 = assign14580_e20382_d_n12;
        locals.var_ps0_ini_dn17 = assign14580_e20382_d_n17;

        let (assign14590_e20407, assign14590_e20407_d_n0, assign14590_e20407_d_n2, assign14590_e20407_d_n6, assign14590_e20407_d_n7, assign14590_e20407_d_n10, assign14590_e20407_d_n11, assign14590_e20407_d_n12, assign14590_e20407_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14590_e20396: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14590_e20397: f64 = (locals.var_beta * assign14590_e20396);
        let assign14590_e20399: f64 = (assign14590_e20397 - 1.0);
        let assign14590_e20400: f64 = (4.0 * assign14590_e20399);
        let assign14590_e20403: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign14590_e20404: f64 = (assign14590_e20400 / assign14590_e20403);
        let assign14590_e20405: f64 = (1.0 + assign14590_e20404);
        (assign14590_e20405, ((((4.0 * (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * ((locals.var_beta_dn10 * assign14590_e20396) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10)))) * assign14590_e20403) - (assign14590_e20400 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14590_e20407;
        locals.var_tx_dn0 = assign14590_e20407_d_n0;
        locals.var_tx_dn2 = assign14590_e20407_d_n2;
        locals.var_tx_dn6 = assign14590_e20407_d_n6;
        locals.var_tx_dn7 = assign14590_e20407_d_n7;
        locals.var_tx_dn10 = assign14590_e20407_d_n10;
        locals.var_tx_dn11 = assign14590_e20407_d_n11;
        locals.var_tx_dn12 = assign14590_e20407_d_n12;
        locals.var_tx_dn17 = assign14590_e20407_d_n17;

        let (assign14600_e20427, assign14600_e20427_d_n0, assign14600_e20427_d_n2, assign14600_e20427_d_n6, assign14600_e20427_d_n7, assign14600_e20427_d_n10, assign14600_e20427_d_n11, assign14600_e20427_d_n12, assign14600_e20427_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14600_e20419: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14600_e20425, assign14600_e20425_d_n0, assign14600_e20425_d_n2, assign14600_e20425_d_n6, assign14600_e20425_d_n7, assign14600_e20425_d_n10, assign14600_e20425_d_n11, assign14600_e20425_d_n12, assign14600_e20425_d_n17,) = {
            if (locals.var_tx >= assign14600_e20419) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign14600_e20424: f64 = (10.0 * 2.220446049250313e-16);
                (assign14600_e20424, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14600_e20425, assign14600_e20425_d_n0, assign14600_e20425_d_n2, assign14600_e20425_d_n6, assign14600_e20425_d_n7, assign14600_e20425_d_n10, assign14600_e20425_d_n11, assign14600_e20425_d_n12, assign14600_e20425_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14600_e20427;
        locals.var_tx_dn0 = assign14600_e20427_d_n0;
        locals.var_tx_dn2 = assign14600_e20427_d_n2;
        locals.var_tx_dn6 = assign14600_e20427_d_n6;
        locals.var_tx_dn7 = assign14600_e20427_d_n7;
        locals.var_tx_dn10 = assign14600_e20427_d_n10;
        locals.var_tx_dn11 = assign14600_e20427_d_n11;
        locals.var_tx_dn12 = assign14600_e20427_d_n12;
        locals.var_tx_dn17 = assign14600_e20427_d_n17;

        let (assign14610_e20449, assign14610_e20449_d_n0, assign14610_e20449_d_n2, assign14610_e20449_d_n6, assign14610_e20449_d_n7, assign14610_e20449_d_n10, assign14610_e20449_d_n11, assign14610_e20449_d_n12, assign14610_e20449_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14610_e20439: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign14610_e20441: f64 = (assign14610_e20439 * 0.5);
        let assign14610_e20444: f64 = (locals.var_tx).sqrt();
        let assign14610_e20445: f64 = (1.0 - assign14610_e20444);
        let assign14610_e20446: f64 = (assign14610_e20441 * assign14610_e20445);
        let assign14610_e20447: f64 = (locals.var_vgp + assign14610_e20446);
        (assign14610_e20447, (locals.var_vgp_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn0 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn2 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn6 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn7 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn10 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn11 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn12 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn17 / (2.0 * assign14610_e20444)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14610_e20449;
        locals.var_ps0_inia_dn0 = assign14610_e20449_d_n0;
        locals.var_ps0_inia_dn2 = assign14610_e20449_d_n2;
        locals.var_ps0_inia_dn6 = assign14610_e20449_d_n6;
        locals.var_ps0_inia_dn7 = assign14610_e20449_d_n7;
        locals.var_ps0_inia_dn10 = assign14610_e20449_d_n10;
        locals.var_ps0_inia_dn11 = assign14610_e20449_d_n11;
        locals.var_ps0_inia_dn12 = assign14610_e20449_d_n12;
        locals.var_ps0_inia_dn17 = assign14610_e20449_d_n17;

    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14620_e20464, assign14620_e20464_d_n0, assign14620_e20464_d_n2, assign14620_e20464_d_n6, assign14620_e20464_d_n7, assign14620_e20464_d_n10, assign14620_e20464_d_n11, assign14620_e20464_d_n12, assign14620_e20464_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14620_e20461: f64 = (locals.var_ps0_inia - locals.var_vbcs_cl);
        let assign14620_e20462: f64 = (locals.var_beta * assign14620_e20461);
        (assign14620_e20462, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14620_e20461) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14620_e20464;
        locals.var_chi_dn0 = assign14620_e20464_d_n0;
        locals.var_chi_dn2 = assign14620_e20464_d_n2;
        locals.var_chi_dn6 = assign14620_e20464_d_n6;
        locals.var_chi_dn7 = assign14620_e20464_d_n7;
        locals.var_chi_dn10 = assign14620_e20464_d_n10;
        locals.var_chi_dn11 = assign14620_e20464_d_n11;
        locals.var_chi_dn12 = assign14620_e20464_d_n12;
        locals.var_chi_dn17 = assign14620_e20464_d_n17;

        let assign14630_e20467: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign14630_e20467;

        let (assign14640_e20484, assign14640_e20484_d_n0, assign14640_e20484_d_n2, assign14640_e20484_d_n6, assign14640_e20484_d_n7, assign14640_e20484_d_n10, assign14640_e20484_d_n11, assign14640_e20484_d_n12, assign14640_e20484_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14640_e20481: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14640_e20482: f64 = (locals.var_beta * assign14640_e20481);
        (assign14640_e20482, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14640_e20481) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14640_e20484;
        locals.var_ty_dn0 = assign14640_e20484_d_n0;
        locals.var_ty_dn2 = assign14640_e20484_d_n2;
        locals.var_ty_dn6 = assign14640_e20484_d_n6;
        locals.var_ty_dn7 = assign14640_e20484_d_n7;
        locals.var_ty_dn10 = assign14640_e20484_d_n10;
        locals.var_ty_dn11 = assign14640_e20484_d_n11;
        locals.var_ty_dn12 = assign14640_e20484_d_n12;
        locals.var_ty_dn17 = assign14640_e20484_d_n17;

        let (assign14650_e20505, assign14650_e20505_d_n0, assign14650_e20505_d_n2, assign14650_e20505_d_n6, assign14650_e20505_d_n7, assign14650_e20505_d_n10, assign14650_e20505_d_n11, assign14650_e20505_d_n12, assign14650_e20505_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14650_e20498: f64 = (1.414213562373095 / 108.0);
        let assign14650_e20500: f64 = (assign14650_e20498 * locals.var_beta);
        let assign14650_e20502: f64 = (assign14650_e20500 * locals.var_fac1);
        let assign14650_e20503: f64 = (1.0 / assign14650_e20502);
        (assign14650_e20503, (-((assign14650_e20500 * locals.var_fac1_dn0) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn2) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn6) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn7) / (assign14650_e20502 * assign14650_e20502))), (-((((assign14650_e20498 * locals.var_beta_dn10) * locals.var_fac1) + (assign14650_e20500 * locals.var_fac1_dn10)) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn11) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn12) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn17) / (assign14650_e20502 * assign14650_e20502))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14650_e20505;
        locals.var_t1_dn0 = assign14650_e20505_d_n0;
        locals.var_t1_dn2 = assign14650_e20505_d_n2;
        locals.var_t1_dn6 = assign14650_e20505_d_n6;
        locals.var_t1_dn7 = assign14650_e20505_d_n7;
        locals.var_t1_dn10 = assign14650_e20505_d_n10;
        locals.var_t1_dn11 = assign14650_e20505_d_n11;
        locals.var_t1_dn12 = assign14650_e20505_d_n12;
        locals.var_t1_dn17 = assign14650_e20505_d_n17;

        let (assign14660_e20522, assign14660_e20522_d_n0, assign14660_e20522_d_n2, assign14660_e20522_d_n6, assign14660_e20522_d_n7, assign14660_e20522_d_n10, assign14660_e20522_d_n11, assign14660_e20522_d_n12, assign14660_e20522_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14660_e20519: f64 = (3.0 * locals.var_t1);
        let assign14660_e20520: f64 = (81.0 + assign14660_e20519);
        (assign14660_e20520, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14660_e20522;
        locals.var_t2_dn0 = assign14660_e20522_d_n0;
        locals.var_t2_dn2 = assign14660_e20522_d_n2;
        locals.var_t2_dn6 = assign14660_e20522_d_n6;
        locals.var_t2_dn7 = assign14660_e20522_d_n7;
        locals.var_t2_dn10 = assign14660_e20522_d_n10;
        locals.var_t2_dn11 = assign14660_e20522_d_n11;
        locals.var_t2_dn12 = assign14660_e20522_d_n12;
        locals.var_t2_dn17 = assign14660_e20522_d_n17;

        let (assign14670_e20546, assign14670_e20546_d_n0, assign14670_e20546_d_n2, assign14670_e20546_d_n6, assign14670_e20546_d_n7, assign14670_e20546_d_n10, assign14670_e20546_d_n11, assign14670_e20546_d_n12, assign14670_e20546_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14670_e20534: f64 = (-2916.0);
        let assign14670_e20537: f64 = (81.0 * locals.var_t1);
        let assign14670_e20538: f64 = (assign14670_e20534 - assign14670_e20537);
        let assign14670_e20541: f64 = (27.0 * locals.var_t1);
        let assign14670_e20543: f64 = (assign14670_e20541 * locals.var_ty);
        let assign14670_e20544: f64 = (assign14670_e20538 + assign14670_e20543);
        (assign14670_e20544, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14670_e20546;
        locals.var_t3_dn0 = assign14670_e20546_d_n0;
        locals.var_t3_dn2 = assign14670_e20546_d_n2;
        locals.var_t3_dn6 = assign14670_e20546_d_n6;
        locals.var_t3_dn7 = assign14670_e20546_d_n7;
        locals.var_t3_dn10 = assign14670_e20546_d_n10;
        locals.var_t3_dn11 = assign14670_e20546_d_n11;
        locals.var_t3_dn12 = assign14670_e20546_d_n12;
        locals.var_t3_dn17 = assign14670_e20546_d_n17;

        let (assign14680_e20571, assign14680_e20571_d_n0, assign14680_e20571_d_n2, assign14680_e20571_d_n6, assign14680_e20571_d_n7, assign14680_e20571_d_n10, assign14680_e20571_d_n11, assign14680_e20571_d_n12, assign14680_e20571_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14680_e20561: f64 = (54.0 + locals.var_t1);
        let assign14680_e20562: f64 = (81.0 * assign14680_e20561);
        let assign14680_e20563: f64 = (1458.0 - assign14680_e20562);
        let assign14680_e20566: f64 = (27.0 * locals.var_t1);
        let assign14680_e20568: f64 = (assign14680_e20566 * locals.var_ty);
        let assign14680_e20569: f64 = (assign14680_e20563 + assign14680_e20568);
        (assign14680_e20569, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14680_e20571;
        locals.var_t4_dn0 = assign14680_e20571_d_n0;
        locals.var_t4_dn2 = assign14680_e20571_d_n2;
        locals.var_t4_dn6 = assign14680_e20571_d_n6;
        locals.var_t4_dn7 = assign14680_e20571_d_n7;
        locals.var_t4_dn10 = assign14680_e20571_d_n10;
        locals.var_t4_dn11 = assign14680_e20571_d_n11;
        locals.var_t4_dn12 = assign14680_e20571_d_n12;
        locals.var_t4_dn17 = assign14680_e20571_d_n17;

        let (assign14690_e20586, assign14690_e20586_d_n0, assign14690_e20586_d_n2, assign14690_e20586_d_n6, assign14690_e20586_d_n7, assign14690_e20586_d_n10, assign14690_e20586_d_n11, assign14690_e20586_d_n12, assign14690_e20586_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14690_e20584: f64 = (locals.var_t4 * locals.var_t4);
        (assign14690_e20584, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14690_e20586;
        locals.var_t4_dn0 = assign14690_e20586_d_n0;
        locals.var_t4_dn2 = assign14690_e20586_d_n2;
        locals.var_t4_dn6 = assign14690_e20586_d_n6;
        locals.var_t4_dn7 = assign14690_e20586_d_n7;
        locals.var_t4_dn10 = assign14690_e20586_d_n10;
        locals.var_t4_dn11 = assign14690_e20586_d_n11;
        locals.var_t4_dn12 = assign14690_e20586_d_n12;
        locals.var_t4_dn17 = assign14690_e20586_d_n17;

        let (assign14700_e20612, assign14700_e20612_d_n0, assign14700_e20612_d_n2, assign14700_e20612_d_n6, assign14700_e20612_d_n7, assign14700_e20612_d_n10, assign14700_e20612_d_n11, assign14700_e20612_d_n12, assign14700_e20612_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14700_e20600: f64 = (4.0 * locals.var_t2);
        let assign14700_e20602: f64 = (assign14700_e20600 * locals.var_t2);
        let assign14700_e20604: f64 = (assign14700_e20602 * locals.var_t2);
        let assign14700_e20606: f64 = (assign14700_e20604 + locals.var_t4);
        let assign14700_e20607: f64 = (assign14700_e20606).sqrt();
        let assign14700_e20608: f64 = (locals.var_t3 + assign14700_e20607);
        let assign14700_e20610: f64 = (assign14700_e20608).powf(0.3333333333333333);
        (assign14700_e20610, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn0)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn0)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn2)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn2)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn6)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn6)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn7)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn7)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn10)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn10)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn11)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn11)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn12)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn12)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn17)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn17)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14700_e20607))) / assign14700_e20608))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14700_e20612;
        locals.var_t5_dn0 = assign14700_e20612_d_n0;
        locals.var_t5_dn2 = assign14700_e20612_d_n2;
        locals.var_t5_dn6 = assign14700_e20612_d_n6;
        locals.var_t5_dn7 = assign14700_e20612_d_n7;
        locals.var_t5_dn10 = assign14700_e20612_d_n10;
        locals.var_t5_dn11 = assign14700_e20612_d_n11;
        locals.var_t5_dn12 = assign14700_e20612_d_n12;
        locals.var_t5_dn17 = assign14700_e20612_d_n17;

        let (assign14710_e20641, assign14710_e20641_d_n0, assign14710_e20641_d_n2, assign14710_e20641_d_n6, assign14710_e20641_d_n7, assign14710_e20641_d_n10, assign14710_e20641_d_n11, assign14710_e20641_d_n12, assign14710_e20641_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14710_e20626: f64 = (1.259921049894873 * locals.var_t2);
        let assign14710_e20629: f64 = (3.0 * locals.var_t5);
        let assign14710_e20630: f64 = (assign14710_e20626 / assign14710_e20629);
        let assign14710_e20631: f64 = (3.0 - assign14710_e20630);
        let assign14710_e20635: f64 = (3.0 * 1.259921049894873);
        let assign14710_e20636: f64 = (1.0 / assign14710_e20635);
        let assign14710_e20638: f64 = (assign14710_e20636 * locals.var_t5);
        let assign14710_e20639: f64 = (assign14710_e20631 + assign14710_e20638);
        (assign14710_e20639, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn0))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn2))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn6))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn7))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn10))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn11))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn12))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn17))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14710_e20641;
        locals.var_tx_dn0 = assign14710_e20641_d_n0;
        locals.var_tx_dn2 = assign14710_e20641_d_n2;
        locals.var_tx_dn6 = assign14710_e20641_d_n6;
        locals.var_tx_dn7 = assign14710_e20641_d_n7;
        locals.var_tx_dn10 = assign14710_e20641_d_n10;
        locals.var_tx_dn11 = assign14710_e20641_d_n11;
        locals.var_tx_dn12 = assign14710_e20641_d_n12;
        locals.var_tx_dn17 = assign14710_e20641_d_n17;

        let (assign14720_e20658, assign14720_e20658_d_n0, assign14720_e20658_d_n2, assign14720_e20658_d_n6, assign14720_e20658_d_n7, assign14720_e20658_d_n10, assign14720_e20658_d_n11, assign14720_e20658_d_n12, assign14720_e20658_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14720_e20654: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign14720_e20656: f64 = (assign14720_e20654 + locals.var_vbcs_cl);
        (assign14720_e20656, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14720_e20658;
        locals.var_ps0_inia_dn0 = assign14720_e20658_d_n0;
        locals.var_ps0_inia_dn2 = assign14720_e20658_d_n2;
        locals.var_ps0_inia_dn6 = assign14720_e20658_d_n6;
        locals.var_ps0_inia_dn7 = assign14720_e20658_d_n7;
        locals.var_ps0_inia_dn10 = assign14720_e20658_d_n10;
        locals.var_ps0_inia_dn11 = assign14720_e20658_d_n11;
        locals.var_ps0_inia_dn12 = assign14720_e20658_d_n12;
        locals.var_ps0_inia_dn17 = assign14720_e20658_d_n17;

        let (assign14730_e20671, assign14730_e20671_d_n0, assign14730_e20671_d_n2, assign14730_e20671_d_n6, assign14730_e20671_d_n7, assign14730_e20671_d_n10, assign14730_e20671_d_n11, assign14730_e20671_d_n12, assign14730_e20671_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14730_e20671;
        locals.var_ps0_ini_dn0 = assign14730_e20671_d_n0;
        locals.var_ps0_ini_dn2 = assign14730_e20671_d_n2;
        locals.var_ps0_ini_dn6 = assign14730_e20671_d_n6;
        locals.var_ps0_ini_dn7 = assign14730_e20671_d_n7;
        locals.var_ps0_ini_dn10 = assign14730_e20671_d_n10;
        locals.var_ps0_ini_dn11 = assign14730_e20671_d_n11;
        locals.var_ps0_ini_dn12 = assign14730_e20671_d_n12;
        locals.var_ps0_ini_dn17 = assign14730_e20671_d_n17;

        let assign14740_e20674: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard444 = assign14740_e20674;

        let (assign14750_e20690, assign14750_e20690_d_n0, assign14750_e20690_d_n2, assign14750_e20690_d_n6, assign14750_e20690_d_n7, assign14750_e20690_d_n10, assign14750_e20690_d_n11, assign14750_e20690_d_n12, assign14750_e20690_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14750_e20690;
        locals.var_ps0_ini_dn0 = assign14750_e20690_d_n0;
        locals.var_ps0_ini_dn2 = assign14750_e20690_d_n2;
        locals.var_ps0_ini_dn6 = assign14750_e20690_d_n6;
        locals.var_ps0_ini_dn7 = assign14750_e20690_d_n7;
        locals.var_ps0_ini_dn10 = assign14750_e20690_d_n10;
        locals.var_ps0_ini_dn11 = assign14750_e20690_d_n11;
        locals.var_ps0_ini_dn12 = assign14750_e20690_d_n12;
        locals.var_ps0_ini_dn17 = assign14750_e20690_d_n17;

        let (assign14760_e20711, assign14760_e20711_d_n0, assign14760_e20711_d_n2, assign14760_e20711_d_n6, assign14760_e20711_d_n7, assign14760_e20711_d_n10, assign14760_e20711_d_n11, assign14760_e20711_d_n12, assign14760_e20711_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14760_e20707: f64 = (1.0 / locals.var_cnst1soi);
        let assign14760_e20709: f64 = (assign14760_e20707 / locals.var_cnstc_foxi);
        (assign14760_e20709, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14760_e20711;
        locals.var_t1_dn0 = assign14760_e20711_d_n0;
        locals.var_t1_dn2 = assign14760_e20711_d_n2;
        locals.var_t1_dn6 = assign14760_e20711_d_n6;
        locals.var_t1_dn7 = assign14760_e20711_d_n7;
        locals.var_t1_dn10 = assign14760_e20711_d_n10;
        locals.var_t1_dn11 = assign14760_e20711_d_n11;
        locals.var_t1_dn12 = assign14760_e20711_d_n12;
        locals.var_t1_dn17 = assign14760_e20711_d_n17;

        let (assign14770_e20732, assign14770_e20732_d_n0, assign14770_e20732_d_n2, assign14770_e20732_d_n6, assign14770_e20732_d_n7, assign14770_e20732_d_n10, assign14770_e20732_d_n11, assign14770_e20732_d_n12, assign14770_e20732_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14770_e20728: f64 = (locals.var_t1 * locals.var_vgp);
        let assign14770_e20730: f64 = (assign14770_e20728 * locals.var_vgp);
        (assign14770_e20730, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn2)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn7)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn11)), ((((locals.var_t1_dn12 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn12)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn12)), ((((locals.var_t1_dn17 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn17)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14770_e20732;
        locals.var_t2_dn0 = assign14770_e20732_d_n0;
        locals.var_t2_dn2 = assign14770_e20732_d_n2;
        locals.var_t2_dn6 = assign14770_e20732_d_n6;
        locals.var_t2_dn7 = assign14770_e20732_d_n7;
        locals.var_t2_dn10 = assign14770_e20732_d_n10;
        locals.var_t2_dn11 = assign14770_e20732_d_n11;
        locals.var_t2_dn12 = assign14770_e20732_d_n12;
        locals.var_t2_dn17 = assign14770_e20732_d_n17;

        let (assign14780_e20753, assign14780_e20753_d_n0, assign14780_e20753_d_n2, assign14780_e20753_d_n6, assign14780_e20753_d_n7, assign14780_e20753_d_n10, assign14780_e20753_d_n11, assign14780_e20753_d_n12, assign14780_e20753_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14780_e20750: f64 = (2.0 / locals.var_vgp);
        let assign14780_e20751: f64 = (locals.var_beta + assign14780_e20750);
        (assign14780_e20751, (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn12) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn17) / (locals.var_vgp * locals.var_vgp))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14780_e20753;
        locals.var_t3_dn0 = assign14780_e20753_d_n0;
        locals.var_t3_dn2 = assign14780_e20753_d_n2;
        locals.var_t3_dn6 = assign14780_e20753_d_n6;
        locals.var_t3_dn7 = assign14780_e20753_d_n7;
        locals.var_t3_dn10 = assign14780_e20753_d_n10;
        locals.var_t3_dn11 = assign14780_e20753_d_n11;
        locals.var_t3_dn12 = assign14780_e20753_d_n12;
        locals.var_t3_dn17 = assign14780_e20753_d_n17;

        let (assign14790_e20773, assign14790_e20773_d_n0, assign14790_e20773_d_n2, assign14790_e20773_d_n6, assign14790_e20773_d_n7, assign14790_e20773_d_n10, assign14790_e20773_d_n11, assign14790_e20773_d_n12, assign14790_e20773_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14790_e20769: f64 = (locals.var_t2).ln();
        let assign14790_e20771: f64 = (assign14790_e20769 / locals.var_t3);
        (assign14790_e20771, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign14790_e20773;
        locals.var_ps0_inib_dn0 = assign14790_e20773_d_n0;
        locals.var_ps0_inib_dn2 = assign14790_e20773_d_n2;
        locals.var_ps0_inib_dn6 = assign14790_e20773_d_n6;
        locals.var_ps0_inib_dn7 = assign14790_e20773_d_n7;
        locals.var_ps0_inib_dn10 = assign14790_e20773_d_n10;
        locals.var_ps0_inib_dn11 = assign14790_e20773_d_n11;
        locals.var_ps0_inib_dn12 = assign14790_e20773_d_n12;
        locals.var_ps0_inib_dn17 = assign14790_e20773_d_n17;

        let (assign14800_e20794, assign14800_e20794_d_n0, assign14800_e20794_d_n2, assign14800_e20794_d_n6, assign14800_e20794_d_n7, assign14800_e20794_d_n10, assign14800_e20794_d_n11, assign14800_e20794_d_n12, assign14800_e20794_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14800_e20790: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign14800_e20792: f64 = (assign14800_e20790 - 0.0008);
        (assign14800_e20792, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign14800_e20794;
        locals.var_tmf1_dn0 = assign14800_e20794_d_n0;
        locals.var_tmf1_dn2 = assign14800_e20794_d_n2;
        locals.var_tmf1_dn6 = assign14800_e20794_d_n6;
        locals.var_tmf1_dn7 = assign14800_e20794_d_n7;
        locals.var_tmf1_dn10 = assign14800_e20794_d_n10;
        locals.var_tmf1_dn11 = assign14800_e20794_d_n11;
        locals.var_tmf1_dn12 = assign14800_e20794_d_n12;
        locals.var_tmf1_dn17 = assign14800_e20794_d_n17;

        let (assign14810_e20815, assign14810_e20815_d_n0, assign14810_e20815_d_n2, assign14810_e20815_d_n6, assign14810_e20815_d_n7, assign14810_e20815_d_n10, assign14810_e20815_d_n11, assign14810_e20815_d_n12, assign14810_e20815_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14810_e20811: f64 = (4.0 * locals.var_ps0_inib);
        let assign14810_e20813: f64 = (assign14810_e20811 * 0.0008);
        (assign14810_e20813, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14810_e20815;
        locals.var_tmf2_dn0 = assign14810_e20815_d_n0;
        locals.var_tmf2_dn2 = assign14810_e20815_d_n2;
        locals.var_tmf2_dn6 = assign14810_e20815_d_n6;
        locals.var_tmf2_dn7 = assign14810_e20815_d_n7;
        locals.var_tmf2_dn10 = assign14810_e20815_d_n10;
        locals.var_tmf2_dn11 = assign14810_e20815_d_n11;
        locals.var_tmf2_dn12 = assign14810_e20815_d_n12;
        locals.var_tmf2_dn17 = assign14810_e20815_d_n17;

        let (assign14820_e20838, assign14820_e20838_d_n0, assign14820_e20838_d_n2, assign14820_e20838_d_n6, assign14820_e20838_d_n7, assign14820_e20838_d_n10, assign14820_e20838_d_n11, assign14820_e20838_d_n12, assign14820_e20838_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let (assign14820_e20836, assign14820_e20836_d_n0, assign14820_e20836_d_n2, assign14820_e20836_d_n6, assign14820_e20836_d_n7, assign14820_e20836_d_n10, assign14820_e20836_d_n11, assign14820_e20836_d_n12, assign14820_e20836_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign14820_e20835: f64 = (-locals.var_tmf2);
                (assign14820_e20835, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign14820_e20836, assign14820_e20836_d_n0, assign14820_e20836_d_n2, assign14820_e20836_d_n6, assign14820_e20836_d_n7, assign14820_e20836_d_n10, assign14820_e20836_d_n11, assign14820_e20836_d_n12, assign14820_e20836_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14820_e20838;
        locals.var_tmf2_dn0 = assign14820_e20838_d_n0;
        locals.var_tmf2_dn2 = assign14820_e20838_d_n2;
        locals.var_tmf2_dn6 = assign14820_e20838_d_n6;
        locals.var_tmf2_dn7 = assign14820_e20838_d_n7;
        locals.var_tmf2_dn10 = assign14820_e20838_d_n10;
        locals.var_tmf2_dn11 = assign14820_e20838_d_n11;
        locals.var_tmf2_dn12 = assign14820_e20838_d_n12;
        locals.var_tmf2_dn17 = assign14820_e20838_d_n17;

        let (assign14830_e20860, assign14830_e20860_d_n0, assign14830_e20860_d_n2, assign14830_e20860_d_n6, assign14830_e20860_d_n7, assign14830_e20860_d_n10, assign14830_e20860_d_n11, assign14830_e20860_d_n12, assign14830_e20860_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14830_e20855: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14830_e20857: f64 = (assign14830_e20855 + locals.var_tmf2);
        let assign14830_e20858: f64 = (assign14830_e20857).sqrt();
        (assign14830_e20858, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign14830_e20858)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14830_e20860;
        locals.var_tmf2_dn0 = assign14830_e20860_d_n0;
        locals.var_tmf2_dn2 = assign14830_e20860_d_n2;
        locals.var_tmf2_dn6 = assign14830_e20860_d_n6;
        locals.var_tmf2_dn7 = assign14830_e20860_d_n7;
        locals.var_tmf2_dn10 = assign14830_e20860_d_n10;
        locals.var_tmf2_dn11 = assign14830_e20860_d_n11;
        locals.var_tmf2_dn12 = assign14830_e20860_d_n12;
        locals.var_tmf2_dn17 = assign14830_e20860_d_n17;

        let (assign14840_e20883, assign14840_e20883_d_n0, assign14840_e20883_d_n2, assign14840_e20883_d_n6, assign14840_e20883_d_n7, assign14840_e20883_d_n10, assign14840_e20883_d_n11, assign14840_e20883_d_n12, assign14840_e20883_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14840_e20879: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14840_e20880: f64 = (0.5 * assign14840_e20879);
        let assign14840_e20881: f64 = (locals.var_ps0_inib - assign14840_e20880);
        (assign14840_e20881, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14840_e20883;
        locals.var_ps0_ini_dn0 = assign14840_e20883_d_n0;
        locals.var_ps0_ini_dn2 = assign14840_e20883_d_n2;
        locals.var_ps0_ini_dn6 = assign14840_e20883_d_n6;
        locals.var_ps0_ini_dn7 = assign14840_e20883_d_n7;
        locals.var_ps0_ini_dn10 = assign14840_e20883_d_n10;
        locals.var_ps0_ini_dn11 = assign14840_e20883_d_n11;
        locals.var_ps0_ini_dn12 = assign14840_e20883_d_n12;
        locals.var_ps0_ini_dn17 = assign14840_e20883_d_n17;

        let (assign14850_e20898, assign14850_e20898_d_n0, assign14850_e20898_d_n2, assign14850_e20898_d_n6, assign14850_e20898_d_n7, assign14850_e20898_d_n10, assign14850_e20898_d_n11, assign14850_e20898_d_n12, assign14850_e20898_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14850_e20895: f64 = (5e-12 / 2.0);
        let assign14850_e20896: f64 = (locals.var_vbcs_cl + assign14850_e20895);
        (assign14850_e20896, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14850_e20898;
        locals.var_tx_dn0 = assign14850_e20898_d_n0;
        locals.var_tx_dn2 = assign14850_e20898_d_n2;
        locals.var_tx_dn6 = assign14850_e20898_d_n6;
        locals.var_tx_dn7 = assign14850_e20898_d_n7;
        locals.var_tx_dn10 = assign14850_e20898_d_n10;
        locals.var_tx_dn11 = assign14850_e20898_d_n11;
        locals.var_tx_dn12 = assign14850_e20898_d_n12;
        locals.var_tx_dn17 = assign14850_e20898_d_n17;

        let assign14860_e20901: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard445 = assign14860_e20901;

        let (assign14870_e20914, assign14870_e20914_d_n0, assign14870_e20914_d_n2, assign14870_e20914_d_n6, assign14870_e20914_d_n7, assign14870_e20914_d_n10, assign14870_e20914_d_n11, assign14870_e20914_d_n12, assign14870_e20914_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard445 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14870_e20914;
        locals.var_ps0_ini_dn0 = assign14870_e20914_d_n0;
        locals.var_ps0_ini_dn2 = assign14870_e20914_d_n2;
        locals.var_ps0_ini_dn6 = assign14870_e20914_d_n6;
        locals.var_ps0_ini_dn7 = assign14870_e20914_d_n7;
        locals.var_ps0_ini_dn10 = assign14870_e20914_d_n10;
        locals.var_ps0_ini_dn11 = assign14870_e20914_d_n11;
        locals.var_ps0_ini_dn12 = assign14870_e20914_d_n12;
        locals.var_ps0_ini_dn17 = assign14870_e20914_d_n17;

        let (assign14880_e20922, assign14880_e20922_d_n0, assign14880_e20922_d_n2, assign14880_e20922_d_n6, assign14880_e20922_d_n7, assign14880_e20922_d_n10, assign14880_e20922_d_n11, assign14880_e20922_d_n12, assign14880_e20922_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14880_e20922;
        locals.var_ps0_dn0 = assign14880_e20922_d_n0;
        locals.var_ps0_dn2 = assign14880_e20922_d_n2;
        locals.var_ps0_dn6 = assign14880_e20922_d_n6;
        locals.var_ps0_dn7 = assign14880_e20922_d_n7;
        locals.var_ps0_dn10 = assign14880_e20922_d_n10;
        locals.var_ps0_dn11 = assign14880_e20922_d_n11;
        locals.var_ps0_dn12 = assign14880_e20922_d_n12;
        locals.var_ps0_dn17 = assign14880_e20922_d_n17;

        let (assign14890_e20930, assign14890_e20930_d_n0, assign14890_e20930_d_n2, assign14890_e20930_d_n6, assign14890_e20930_d_n7, assign14890_e20930_d_n10, assign14890_e20930_d_n11, assign14890_e20930_d_n12, assign14890_e20930_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign14890_e20930;
        locals.var_psl_lim_dn0 = assign14890_e20930_d_n0;
        locals.var_psl_lim_dn2 = assign14890_e20930_d_n2;
        locals.var_psl_lim_dn6 = assign14890_e20930_d_n6;
        locals.var_psl_lim_dn7 = assign14890_e20930_d_n7;
        locals.var_psl_lim_dn10 = assign14890_e20930_d_n10;
        locals.var_psl_lim_dn11 = assign14890_e20930_d_n11;
        locals.var_psl_lim_dn12 = assign14890_e20930_d_n12;
        locals.var_psl_lim_dn17 = assign14890_e20930_d_n17;

        let assign14900_e20937: f64 = if ((p.p25 == 1.0) && (p.p26 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard446 = assign14900_e20937;

    }
}

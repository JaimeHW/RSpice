#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign14910_e20948, assign14910_e20948_d_n0, assign14910_e20948_d_n2, assign14910_e20948_d_n6, assign14910_e20948_d_n7, assign14910_e20948_d_n10, assign14910_e20948_d_n11, assign14910_e20948_d_n12, assign14910_e20948_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard446 != 0.0)) {
        let assign14910_e20944: f64 = (1e-9 / 0.0001);
        let assign14910_e20946: f64 = (assign14910_e20944 * (nv17 - 0.0));
        (assign14910_e20946, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign14910_e20944,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign14910_e20948;
        locals.var_qhs_dn0 = assign14910_e20948_d_n0;
        locals.var_qhs_dn2 = assign14910_e20948_d_n2;
        locals.var_qhs_dn6 = assign14910_e20948_d_n6;
        locals.var_qhs_dn7 = assign14910_e20948_d_n7;
        locals.var_qhs_dn10 = assign14910_e20948_d_n10;
        locals.var_qhs_dn11 = assign14910_e20948_d_n11;
        locals.var_qhs_dn12 = assign14910_e20948_d_n12;
        locals.var_qhs_dn17 = assign14910_e20948_d_n17;

        let (assign14920_e20956, assign14920_e20956_d_n0, assign14920_e20956_d_n2, assign14920_e20956_d_n6, assign14920_e20956_d_n7, assign14920_e20956_d_n10, assign14920_e20956_d_n11, assign14920_e20956_d_n12, assign14920_e20956_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard446 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign14920_e20956;
        locals.var_qhs_dn0 = assign14920_e20956_d_n0;
        locals.var_qhs_dn2 = assign14920_e20956_d_n2;
        locals.var_qhs_dn6 = assign14920_e20956_d_n6;
        locals.var_qhs_dn7 = assign14920_e20956_d_n7;
        locals.var_qhs_dn10 = assign14920_e20956_d_n10;
        locals.var_qhs_dn11 = assign14920_e20956_d_n11;
        locals.var_qhs_dn12 = assign14920_e20956_d_n12;
        locals.var_qhs_dn17 = assign14920_e20956_d_n17;

        let (assign14940_e20969, assign14940_e20969_d_n0, assign14940_e20969_d_n2, assign14940_e20969_d_n6, assign14940_e20969_d_n7, assign14940_e20969_d_n10, assign14940_e20969_d_n11, assign14940_e20969_d_n12, assign14940_e20969_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14940_e20966: f64 = (locals.var_beta * locals.var_vbcs_cl);
        let assign14940_e20967: f64 = (assign14940_e20966).exp();
        (assign14940_e20967, (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn0)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn2)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn6)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn7)), (assign14940_e20967 * ((locals.var_beta_dn10 * locals.var_vbcs_cl) + (locals.var_beta * locals.var_vbcs_cl_dn10))), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn11)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn12)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12, locals.var_exp_bvbs_dn17,)
    }
};
        locals.var_exp_bvbs = assign14940_e20969;
        locals.var_exp_bvbs_dn0 = assign14940_e20969_d_n0;
        locals.var_exp_bvbs_dn2 = assign14940_e20969_d_n2;
        locals.var_exp_bvbs_dn6 = assign14940_e20969_d_n6;
        locals.var_exp_bvbs_dn7 = assign14940_e20969_d_n7;
        locals.var_exp_bvbs_dn10 = assign14940_e20969_d_n10;
        locals.var_exp_bvbs_dn11 = assign14940_e20969_d_n11;
        locals.var_exp_bvbs_dn12 = assign14940_e20969_d_n12;
        locals.var_exp_bvbs_dn17 = assign14940_e20969_d_n17;

        let (assign14950_e20976, assign14950_e20976_d_n0, assign14950_e20976_d_n2, assign14950_e20976_d_n6, assign14950_e20976_d_n7, assign14950_e20976_d_n10, assign14950_e20976_d_n11, assign14950_e20976_d_n12, assign14950_e20976_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14950_e20974: f64 = (locals.var_cnst1soi * locals.var_exp_bvbs);
        (assign14950_e20974, ((locals.var_cnst1soi_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1soi_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1soi_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1soi_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1soi_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1soi_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1soi_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn12)), ((locals.var_cnst1soi_dn17 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn17)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn12, locals.var_cfs1_dn17,)
    }
};
        locals.var_cfs1 = assign14950_e20976;
        locals.var_cfs1_dn0 = assign14950_e20976_d_n0;
        locals.var_cfs1_dn2 = assign14950_e20976_d_n2;
        locals.var_cfs1_dn6 = assign14950_e20976_d_n6;
        locals.var_cfs1_dn7 = assign14950_e20976_d_n7;
        locals.var_cfs1_dn10 = assign14950_e20976_d_n10;
        locals.var_cfs1_dn11 = assign14950_e20976_d_n11;
        locals.var_cfs1_dn12 = assign14950_e20976_d_n12;
        locals.var_cfs1_dn17 = assign14950_e20976_d_n17;

        let (assign14960_e20981,) = {
    if (locals.var_guard109 == 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign14960_e20981;

        let (assign14970_e20986, assign14970_e20986_d_n0, assign14970_e20986_d_n2, assign14970_e20986_d_n6, assign14970_e20986_d_n7, assign14970_e20986_d_n10, assign14970_e20986_d_n11, assign14970_e20986_d_n12, assign14970_e20986_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14970_e20986;
        locals.var_phi_s0_soi_dn0 = assign14970_e20986_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14970_e20986_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14970_e20986_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14970_e20986_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14970_e20986_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14970_e20986_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14970_e20986_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14970_e20986_d_n17;

        let (assign14980_e20999, assign14980_e20999_d_n0, assign14980_e20999_d_n2, assign14980_e20999_d_n6, assign14980_e20999_d_n7, assign14980_e20999_d_n10, assign14980_e20999_d_n11, assign14980_e20999_d_n12, assign14980_e20999_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14980_e20991: f64 = (locals.var_q_nsub * p.p237);
        let assign14980_e20993: f64 = (assign14980_e20991 * p.p237);
        let assign14980_e20995: f64 = (assign14980_e20993 / 2.0);
        let assign14980_e20997: f64 = (assign14980_e20995 / 1.034943e-10);
        (assign14980_e20997, ((((locals.var_q_nsub_dn0 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn12 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn17 * p.p237) * p.p237) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn12, locals.var_dphi_sb_dn17,)
    }
};
        locals.var_dphi_sb = assign14980_e20999;
        locals.var_dphi_sb_dn0 = assign14980_e20999_d_n0;
        locals.var_dphi_sb_dn2 = assign14980_e20999_d_n2;
        locals.var_dphi_sb_dn6 = assign14980_e20999_d_n6;
        locals.var_dphi_sb_dn7 = assign14980_e20999_d_n7;
        locals.var_dphi_sb_dn10 = assign14980_e20999_d_n10;
        locals.var_dphi_sb_dn11 = assign14980_e20999_d_n11;
        locals.var_dphi_sb_dn12 = assign14980_e20999_d_n12;
        locals.var_dphi_sb_dn17 = assign14980_e20999_d_n17;

        let (assign14990_e21009, assign14990_e21009_d_n0, assign14990_e21009_d_n2, assign14990_e21009_d_n6, assign14990_e21009_d_n7, assign14990_e21009_d_n10, assign14990_e21009_d_n11, assign14990_e21009_d_n12, assign14990_e21009_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14990_e21004: f64 = (2.0 * locals.var_beta);
        let assign14990_e21006: f64 = (assign14990_e21004 * locals.var_dphi_sb);
        let assign14990_e21007: f64 = (assign14990_e21006).sqrt();
        (assign14990_e21007, ((assign14990_e21004 * locals.var_dphi_sb_dn0) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn2) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn6) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn7) / (2.0 * assign14990_e21007)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign14990_e21004 * locals.var_dphi_sb_dn10)) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn11) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn12) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn17) / (2.0 * assign14990_e21007)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign14990_e21009;
        locals.var_t0_dn0 = assign14990_e21009_d_n0;
        locals.var_t0_dn2 = assign14990_e21009_d_n2;
        locals.var_t0_dn6 = assign14990_e21009_d_n6;
        locals.var_t0_dn7 = assign14990_e21009_d_n7;
        locals.var_t0_dn10 = assign14990_e21009_d_n10;
        locals.var_t0_dn11 = assign14990_e21009_d_n11;
        locals.var_t0_dn12 = assign14990_e21009_d_n12;
        locals.var_t0_dn17 = assign14990_e21009_d_n17;

        let (assign15000_e21021, assign15000_e21021_d_n0, assign15000_e21021_d_n2, assign15000_e21021_d_n6, assign15000_e21021_d_n7, assign15000_e21021_d_n10, assign15000_e21021_d_n11, assign15000_e21021_d_n12, assign15000_e21021_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15000_e21013: f64 = (locals.var_t0).exp();
        let assign15000_e21015: f64 = (-locals.var_t0);
        let assign15000_e21016: f64 = (assign15000_e21015).exp();
        let assign15000_e21017: f64 = (assign15000_e21013 + assign15000_e21016);
        let assign15000_e21019: f64 = (assign15000_e21017 / 2.0);
        (assign15000_e21019, (((assign15000_e21013 * locals.var_t0_dn0) + (assign15000_e21016 * (-locals.var_t0_dn0))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn2) + (assign15000_e21016 * (-locals.var_t0_dn2))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn6) + (assign15000_e21016 * (-locals.var_t0_dn6))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn7) + (assign15000_e21016 * (-locals.var_t0_dn7))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn10) + (assign15000_e21016 * (-locals.var_t0_dn10))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn11) + (assign15000_e21016 * (-locals.var_t0_dn11))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn12) + (assign15000_e21016 * (-locals.var_t0_dn12))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn17) + (assign15000_e21016 * (-locals.var_t0_dn17))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15000_e21021;
        locals.var_t1_dn0 = assign15000_e21021_d_n0;
        locals.var_t1_dn2 = assign15000_e21021_d_n2;
        locals.var_t1_dn6 = assign15000_e21021_d_n6;
        locals.var_t1_dn7 = assign15000_e21021_d_n7;
        locals.var_t1_dn10 = assign15000_e21021_d_n10;
        locals.var_t1_dn11 = assign15000_e21021_d_n11;
        locals.var_t1_dn12 = assign15000_e21021_d_n12;
        locals.var_t1_dn17 = assign15000_e21021_d_n17;

        let (assign15010_e21029, assign15010_e21029_d_n0, assign15010_e21029_d_n2, assign15010_e21029_d_n6, assign15010_e21029_d_n7, assign15010_e21029_d_n10, assign15010_e21029_d_n11, assign15010_e21029_d_n12, assign15010_e21029_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15010_e21025: f64 = (locals.var_t1).ln();
        let assign15010_e21027: f64 = (assign15010_e21025 / locals.var_dphi_sb);
        (assign15010_e21027, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn12 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn12)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn17 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn17)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn12, locals.var_c_sb_dn17,)
    }
};
        locals.var_c_sb = assign15010_e21029;
        locals.var_c_sb_dn0 = assign15010_e21029_d_n0;
        locals.var_c_sb_dn2 = assign15010_e21029_d_n2;
        locals.var_c_sb_dn6 = assign15010_e21029_d_n6;
        locals.var_c_sb_dn7 = assign15010_e21029_d_n7;
        locals.var_c_sb_dn10 = assign15010_e21029_d_n10;
        locals.var_c_sb_dn11 = assign15010_e21029_d_n11;
        locals.var_c_sb_dn12 = assign15010_e21029_d_n12;
        locals.var_c_sb_dn17 = assign15010_e21029_d_n17;

        let (assign15020_e21034,) = {
    if (locals.var_guard109 == 0.0) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign15020_e21034;

    }

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign15030_loop_guard: usize = 0;
        while {
            let assign15030_cond_e21040: f64 = (locals.var_lp_s0_max + 1.0);
            let assign15030_cond_e21042: f64 = if ((locals.var_guard109 == 0.0) && (locals.var_lp_s0 <= assign15030_cond_e21040)) { 1.0 } else { 0.0 };
            assign15030_cond_e21042 != 0.0
        } {
            assign15030_loop_guard += 1;
            assert!(assign15030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15030_body0_e21049, assign15030_body0_e21049_d_n0, assign15030_body0_e21049_d_n2, assign15030_body0_e21049_d_n6, assign15030_body0_e21049_d_n7, assign15030_body0_e21049_d_n10, assign15030_body0_e21049_d_n11, assign15030_body0_e21049_d_n12, assign15030_body0_e21049_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body0_e21047: f64 = (locals.var_phi_s0_soi - locals.var_vbcs_cl);
        (assign15030_body0_e21047, (locals.var_phi_s0_soi_dn0 - locals.var_vbcs_cl_dn0), (locals.var_phi_s0_soi_dn2 - locals.var_vbcs_cl_dn2), (locals.var_phi_s0_soi_dn6 - locals.var_vbcs_cl_dn6), (locals.var_phi_s0_soi_dn7 - locals.var_vbcs_cl_dn7), (locals.var_phi_s0_soi_dn10 - locals.var_vbcs_cl_dn10), (locals.var_phi_s0_soi_dn11 - locals.var_vbcs_cl_dn11), (locals.var_phi_s0_soi_dn12 - locals.var_vbcs_cl_dn12), (locals.var_phi_s0_soi_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_phi_soi0, locals.var_phi_soi0_dn0, locals.var_phi_soi0_dn2, locals.var_phi_soi0_dn6, locals.var_phi_soi0_dn7, locals.var_phi_soi0_dn10, locals.var_phi_soi0_dn11, locals.var_phi_soi0_dn12, locals.var_phi_soi0_dn17,)
    }
};
            locals.var_phi_soi0 = assign15030_body0_e21049;
            locals.var_phi_soi0_dn0 = assign15030_body0_e21049_d_n0;
            locals.var_phi_soi0_dn2 = assign15030_body0_e21049_d_n2;
            locals.var_phi_soi0_dn6 = assign15030_body0_e21049_d_n6;
            locals.var_phi_soi0_dn7 = assign15030_body0_e21049_d_n7;
            locals.var_phi_soi0_dn10 = assign15030_body0_e21049_d_n10;
            locals.var_phi_soi0_dn11 = assign15030_body0_e21049_d_n11;
            locals.var_phi_soi0_dn12 = assign15030_body0_e21049_d_n12;
            locals.var_phi_soi0_dn17 = assign15030_body0_e21049_d_n17;
            let (assign15030_body1_e21056, assign15030_body1_e21056_d_n0, assign15030_body1_e21056_d_n2, assign15030_body1_e21056_d_n6, assign15030_body1_e21056_d_n7, assign15030_body1_e21056_d_n10, assign15030_body1_e21056_d_n11, assign15030_body1_e21056_d_n12, assign15030_body1_e21056_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body1_e21054: f64 = (locals.var_beta * locals.var_phi_soi0);
        (assign15030_body1_e21054, (locals.var_beta * locals.var_phi_soi0_dn0), (locals.var_beta * locals.var_phi_soi0_dn2), (locals.var_beta * locals.var_phi_soi0_dn6), (locals.var_beta * locals.var_phi_soi0_dn7), ((locals.var_beta_dn10 * locals.var_phi_soi0) + (locals.var_beta * locals.var_phi_soi0_dn10)), (locals.var_beta * locals.var_phi_soi0_dn11), (locals.var_beta * locals.var_phi_soi0_dn12), (locals.var_beta * locals.var_phi_soi0_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign15030_body1_e21056;
            locals.var_chi_dn0 = assign15030_body1_e21056_d_n0;
            locals.var_chi_dn2 = assign15030_body1_e21056_d_n2;
            locals.var_chi_dn6 = assign15030_body1_e21056_d_n6;
            locals.var_chi_dn7 = assign15030_body1_e21056_d_n7;
            locals.var_chi_dn10 = assign15030_body1_e21056_d_n10;
            locals.var_chi_dn11 = assign15030_body1_e21056_d_n11;
            locals.var_chi_dn12 = assign15030_body1_e21056_d_n12;
            locals.var_chi_dn17 = assign15030_body1_e21056_d_n17;
            let (assign15030_body2_e21065, assign15030_body2_e21065_d_n0, assign15030_body2_e21065_d_n2, assign15030_body2_e21065_d_n6, assign15030_body2_e21065_d_n7, assign15030_body2_e21065_d_n10, assign15030_body2_e21065_d_n11, assign15030_body2_e21065_d_n12, assign15030_body2_e21065_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body2_e21062: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        let assign15030_body2_e21063: f64 = (locals.var_c_sb * assign15030_body2_e21062);
        (assign15030_body2_e21063, ((locals.var_c_sb_dn0 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign15030_body2_e21065;
            locals.var_ty_dn0 = assign15030_body2_e21065_d_n0;
            locals.var_ty_dn2 = assign15030_body2_e21065_d_n2;
            locals.var_ty_dn6 = assign15030_body2_e21065_d_n6;
            locals.var_ty_dn7 = assign15030_body2_e21065_d_n7;
            locals.var_ty_dn10 = assign15030_body2_e21065_d_n10;
            locals.var_ty_dn11 = assign15030_body2_e21065_d_n11;
            locals.var_ty_dn12 = assign15030_body2_e21065_d_n12;
            locals.var_ty_dn17 = assign15030_body2_e21065_d_n17;
            let assign15030_body3_e21068: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard447 = assign15030_body3_e21068;
            let (assign15030_body4_e21076, assign15030_body4_e21076_d_n0, assign15030_body4_e21076_d_n2, assign15030_body4_e21076_d_n6, assign15030_body4_e21076_d_n7, assign15030_body4_e21076_d_n10, assign15030_body4_e21076_d_n11, assign15030_body4_e21076_d_n12, assign15030_body4_e21076_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body4_e21074: f64 = (locals.var_ty).exp();
        (assign15030_body4_e21074, (assign15030_body4_e21074 * locals.var_ty_dn0), (assign15030_body4_e21074 * locals.var_ty_dn2), (assign15030_body4_e21074 * locals.var_ty_dn6), (assign15030_body4_e21074 * locals.var_ty_dn7), (assign15030_body4_e21074 * locals.var_ty_dn10), (assign15030_body4_e21074 * locals.var_ty_dn11), (assign15030_body4_e21074 * locals.var_ty_dn12), (assign15030_body4_e21074 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15030_body4_e21076;
            locals.var_t1_dn0 = assign15030_body4_e21076_d_n0;
            locals.var_t1_dn2 = assign15030_body4_e21076_d_n2;
            locals.var_t1_dn6 = assign15030_body4_e21076_d_n6;
            locals.var_t1_dn7 = assign15030_body4_e21076_d_n7;
            locals.var_t1_dn10 = assign15030_body4_e21076_d_n10;
            locals.var_t1_dn11 = assign15030_body4_e21076_d_n11;
            locals.var_t1_dn12 = assign15030_body4_e21076_d_n12;
            locals.var_t1_dn17 = assign15030_body4_e21076_d_n17;
            let (assign15030_body5_e21087, assign15030_body5_e21087_d_n0, assign15030_body5_e21087_d_n2, assign15030_body5_e21087_d_n6, assign15030_body5_e21087_d_n7, assign15030_body5_e21087_d_n10, assign15030_body5_e21087_d_n11, assign15030_body5_e21087_d_n12, assign15030_body5_e21087_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body5_e21082: f64 = (-locals.var_c_sb);
        let assign15030_body5_e21084: f64 = (assign15030_body5_e21082 * locals.var_dphi_sb);
        let assign15030_body5_e21085: f64 = (assign15030_body5_e21084).exp();
        (assign15030_body5_e21085, (assign15030_body5_e21085 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn0))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn2))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn6))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn7))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn10))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn11))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn12))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15030_body5_e21087;
            locals.var_t0_dn0 = assign15030_body5_e21087_d_n0;
            locals.var_t0_dn2 = assign15030_body5_e21087_d_n2;
            locals.var_t0_dn6 = assign15030_body5_e21087_d_n6;
            locals.var_t0_dn7 = assign15030_body5_e21087_d_n7;
            locals.var_t0_dn10 = assign15030_body5_e21087_d_n10;
            locals.var_t0_dn11 = assign15030_body5_e21087_d_n11;
            locals.var_t0_dn12 = assign15030_body5_e21087_d_n12;
            locals.var_t0_dn17 = assign15030_body5_e21087_d_n17;
            let (assign15030_body6_e21096, assign15030_body6_e21096_d_n0, assign15030_body6_e21096_d_n2, assign15030_body6_e21096_d_n6, assign15030_body6_e21096_d_n7, assign15030_body6_e21096_d_n10, assign15030_body6_e21096_d_n11, assign15030_body6_e21096_d_n12, assign15030_body6_e21096_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body6_e21094: f64 = (locals.var_t1 - locals.var_t0);
        (assign15030_body6_e21094, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign15030_body6_e21096;
            locals.var_t2_dn0 = assign15030_body6_e21096_d_n0;
            locals.var_t2_dn2 = assign15030_body6_e21096_d_n2;
            locals.var_t2_dn6 = assign15030_body6_e21096_d_n6;
            locals.var_t2_dn7 = assign15030_body6_e21096_d_n7;
            locals.var_t2_dn10 = assign15030_body6_e21096_d_n10;
            locals.var_t2_dn11 = assign15030_body6_e21096_d_n11;
            locals.var_t2_dn12 = assign15030_body6_e21096_d_n12;
            locals.var_t2_dn17 = assign15030_body6_e21096_d_n17;
            let (assign15030_body7_e21108, assign15030_body7_e21108_d_n0, assign15030_body7_e21108_d_n2, assign15030_body7_e21108_d_n6, assign15030_body7_e21108_d_n7, assign15030_body7_e21108_d_n10, assign15030_body7_e21108_d_n11, assign15030_body7_e21108_d_n12, assign15030_body7_e21108_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body7_e21103: f64 = (1.0 + locals.var_t2);
        let assign15030_body7_e21104: f64 = (assign15030_body7_e21103).ln();
        let assign15030_body7_e21106: f64 = (assign15030_body7_e21104 / locals.var_c_sb);
        (assign15030_body7_e21106, ((((locals.var_t2_dn0 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign15030_body7_e21108;
            locals.var_phi_soib_dn0 = assign15030_body7_e21108_d_n0;
            locals.var_phi_soib_dn2 = assign15030_body7_e21108_d_n2;
            locals.var_phi_soib_dn6 = assign15030_body7_e21108_d_n6;
            locals.var_phi_soib_dn7 = assign15030_body7_e21108_d_n7;
            locals.var_phi_soib_dn10 = assign15030_body7_e21108_d_n10;
            locals.var_phi_soib_dn11 = assign15030_body7_e21108_d_n11;
            locals.var_phi_soib_dn12 = assign15030_body7_e21108_d_n12;
            locals.var_phi_soib_dn17 = assign15030_body7_e21108_d_n17;
            let (assign15030_body8_e21119, assign15030_body8_e21119_d_n0, assign15030_body8_e21119_d_n2, assign15030_body8_e21119_d_n6, assign15030_body8_e21119_d_n7, assign15030_body8_e21119_d_n10, assign15030_body8_e21119_d_n11, assign15030_body8_e21119_d_n12, assign15030_body8_e21119_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body8_e21116: f64 = (1.0 + locals.var_t2);
        let assign15030_body8_e21117: f64 = (locals.var_t1 / assign15030_body8_e21116);
        (assign15030_body8_e21117, (((locals.var_t1_dn0 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn0)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn2 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn2)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn6 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn6)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn7 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn7)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn10 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn10)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn11 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn11)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn12 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn12)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn17 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn17)) / (assign15030_body8_e21116 * assign15030_body8_e21116)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign15030_body8_e21119;
            locals.var_phi_soib_dpss_dn0 = assign15030_body8_e21119_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign15030_body8_e21119_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign15030_body8_e21119_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign15030_body8_e21119_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign15030_body8_e21119_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign15030_body8_e21119_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign15030_body8_e21119_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign15030_body8_e21119_d_n17;
            let (assign15030_body9_e21129, assign15030_body9_e21129_d_n0, assign15030_body9_e21129_d_n2, assign15030_body9_e21129_d_n6, assign15030_body9_e21129_d_n7, assign15030_body9_e21129_d_n10, assign15030_body9_e21129_d_n11, assign15030_body9_e21129_d_n12, assign15030_body9_e21129_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 == 0.0)) {
        let assign15030_body9_e21127: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        (assign15030_body9_e21127, (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign15030_body9_e21129;
            locals.var_phi_soib_dn0 = assign15030_body9_e21129_d_n0;
            locals.var_phi_soib_dn2 = assign15030_body9_e21129_d_n2;
            locals.var_phi_soib_dn6 = assign15030_body9_e21129_d_n6;
            locals.var_phi_soib_dn7 = assign15030_body9_e21129_d_n7;
            locals.var_phi_soib_dn10 = assign15030_body9_e21129_d_n10;
            locals.var_phi_soib_dn11 = assign15030_body9_e21129_d_n11;
            locals.var_phi_soib_dn12 = assign15030_body9_e21129_d_n12;
            locals.var_phi_soib_dn17 = assign15030_body9_e21129_d_n17;
            let (assign15030_body10_e21137, assign15030_body10_e21137_d_n0, assign15030_body10_e21137_d_n2, assign15030_body10_e21137_d_n6, assign15030_body10_e21137_d_n7, assign15030_body10_e21137_d_n10, assign15030_body10_e21137_d_n11, assign15030_body10_e21137_d_n12, assign15030_body10_e21137_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign15030_body10_e21137;
            locals.var_phi_soib_dpss_dn0 = assign15030_body10_e21137_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign15030_body10_e21137_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign15030_body10_e21137_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign15030_body10_e21137_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign15030_body10_e21137_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign15030_body10_e21137_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign15030_body10_e21137_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign15030_body10_e21137_d_n17;
            let (assign15030_body11_e21144, assign15030_body11_e21144_d_n0, assign15030_body11_e21144_d_n2, assign15030_body11_e21144_d_n6, assign15030_body11_e21144_d_n7, assign15030_body11_e21144_d_n10, assign15030_body11_e21144_d_n11, assign15030_body11_e21144_d_n12, assign15030_body11_e21144_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body11_e21142: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign15030_body11_e21142, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign15030_body11_e21144;
            locals.var_chib_dn0 = assign15030_body11_e21144_d_n0;
            locals.var_chib_dn2 = assign15030_body11_e21144_d_n2;
            locals.var_chib_dn6 = assign15030_body11_e21144_d_n6;
            locals.var_chib_dn7 = assign15030_body11_e21144_d_n7;
            locals.var_chib_dn10 = assign15030_body11_e21144_d_n10;
            locals.var_chib_dn11 = assign15030_body11_e21144_d_n11;
            locals.var_chib_dn12 = assign15030_body11_e21144_d_n12;
            locals.var_chib_dn17 = assign15030_body11_e21144_d_n17;
            let assign15030_body12_e21146: f64 = (locals.var_chi).abs();
            let assign15030_body12_e21148: f64 = if assign15030_body12_e21146 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard448 = assign15030_body12_e21148;
            let (assign15030_body13_e21162, assign15030_body13_e21162_d_n0, assign15030_body13_e21162_d_n2, assign15030_body13_e21162_d_n6, assign15030_body13_e21162_d_n7, assign15030_body13_e21162_d_n10, assign15030_body13_e21162_d_n11, assign15030_body13_e21162_d_n12, assign15030_body13_e21162_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) {
        let assign15030_body13_e21156: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign15030_body13_e21157: f64 = (1.0 - assign15030_body13_e21156);
        let assign15030_body13_e21159: f64 = (assign15030_body13_e21157 / 2.0);
        let assign15030_body13_e21160: f64 = (assign15030_body13_e21159).sqrt();
        (assign15030_body13_e21160, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign15030_body13_e21160)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15030_body13_e21162;
            locals.var_t0_dn0 = assign15030_body13_e21162_d_n0;
            locals.var_t0_dn2 = assign15030_body13_e21162_d_n2;
            locals.var_t0_dn6 = assign15030_body13_e21162_d_n6;
            locals.var_t0_dn7 = assign15030_body13_e21162_d_n7;
            locals.var_t0_dn10 = assign15030_body13_e21162_d_n10;
            locals.var_t0_dn11 = assign15030_body13_e21162_d_n11;
            locals.var_t0_dn12 = assign15030_body13_e21162_d_n12;
            locals.var_t0_dn17 = assign15030_body13_e21162_d_n17;
            let (assign15030_body14_e21171, assign15030_body14_e21171_d_n0, assign15030_body14_e21171_d_n2, assign15030_body14_e21171_d_n6, assign15030_body14_e21171_d_n7, assign15030_body14_e21171_d_n10, assign15030_body14_e21171_d_n11, assign15030_body14_e21171_d_n12, assign15030_body14_e21171_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) {
        let assign15030_body14_e21169: f64 = (locals.var_chi * locals.var_t0);
        (assign15030_body14_e21169, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15030_body14_e21171;
            locals.var_fb_dn0 = assign15030_body14_e21171_d_n0;
            locals.var_fb_dn2 = assign15030_body14_e21171_d_n2;
            locals.var_fb_dn6 = assign15030_body14_e21171_d_n6;
            locals.var_fb_dn7 = assign15030_body14_e21171_d_n7;
            locals.var_fb_dn10 = assign15030_body14_e21171_d_n10;
            locals.var_fb_dn11 = assign15030_body14_e21171_d_n11;
            locals.var_fb_dn12 = assign15030_body14_e21171_d_n12;
            locals.var_fb_dn17 = assign15030_body14_e21171_d_n17;
            let (assign15030_body15_e21180, assign15030_body15_e21180_d_n0, assign15030_body15_e21180_d_n2, assign15030_body15_e21180_d_n6, assign15030_body15_e21180_d_n7, assign15030_body15_e21180_d_n10, assign15030_body15_e21180_d_n11, assign15030_body15_e21180_d_n12, assign15030_body15_e21180_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) {
        let assign15030_body15_e21178: f64 = (locals.var_beta * locals.var_t0);
        (assign15030_body15_e21178, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15030_body15_e21180;
            locals.var_fb_dpss_dn0 = assign15030_body15_e21180_d_n0;
            locals.var_fb_dpss_dn2 = assign15030_body15_e21180_d_n2;
            locals.var_fb_dpss_dn6 = assign15030_body15_e21180_d_n6;
            locals.var_fb_dpss_dn7 = assign15030_body15_e21180_d_n7;
            locals.var_fb_dpss_dn10 = assign15030_body15_e21180_d_n10;
            locals.var_fb_dpss_dn11 = assign15030_body15_e21180_d_n11;
            locals.var_fb_dpss_dn12 = assign15030_body15_e21180_d_n12;
            locals.var_fb_dpss_dn17 = assign15030_body15_e21180_d_n17;
            let assign15030_body16_e21183: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard449 = assign15030_body16_e21183;
            let (assign15030_body17_e21193, assign15030_body17_e21193_d_n0, assign15030_body17_e21193_d_n2, assign15030_body17_e21193_d_n6, assign15030_body17_e21193_d_n7, assign15030_body17_e21193_d_n10, assign15030_body17_e21193_d_n11, assign15030_body17_e21193_d_n12, assign15030_body17_e21193_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard449 != 0.0)) {
        let assign15030_body17_e21191: f64 = (-locals.var_fb);
        (assign15030_body17_e21191, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15030_body17_e21193;
            locals.var_fb_dn0 = assign15030_body17_e21193_d_n0;
            locals.var_fb_dn2 = assign15030_body17_e21193_d_n2;
            locals.var_fb_dn6 = assign15030_body17_e21193_d_n6;
            locals.var_fb_dn7 = assign15030_body17_e21193_d_n7;
            locals.var_fb_dn10 = assign15030_body17_e21193_d_n10;
            locals.var_fb_dn11 = assign15030_body17_e21193_d_n11;
            locals.var_fb_dn12 = assign15030_body17_e21193_d_n12;
            locals.var_fb_dn17 = assign15030_body17_e21193_d_n17;
            let (assign15030_body18_e21203, assign15030_body18_e21203_d_n0, assign15030_body18_e21203_d_n2, assign15030_body18_e21203_d_n6, assign15030_body18_e21203_d_n7, assign15030_body18_e21203_d_n10, assign15030_body18_e21203_d_n11, assign15030_body18_e21203_d_n12, assign15030_body18_e21203_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard449 != 0.0)) {
        let assign15030_body18_e21201: f64 = (-locals.var_fb_dpss);
        (assign15030_body18_e21201, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15030_body18_e21203;
            locals.var_fb_dpss_dn0 = assign15030_body18_e21203_d_n0;
            locals.var_fb_dpss_dn2 = assign15030_body18_e21203_d_n2;
            locals.var_fb_dpss_dn6 = assign15030_body18_e21203_d_n6;
            locals.var_fb_dpss_dn7 = assign15030_body18_e21203_d_n7;
            locals.var_fb_dpss_dn10 = assign15030_body18_e21203_d_n10;
            locals.var_fb_dpss_dn11 = assign15030_body18_e21203_d_n11;
            locals.var_fb_dpss_dn12 = assign15030_body18_e21203_d_n12;
            locals.var_fb_dpss_dn17 = assign15030_body18_e21203_d_n17;
            let assign15030_body19_e21205: f64 = (locals.var_chi).abs();
            let assign15030_body19_e21207: f64 = if assign15030_body19_e21205 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard450 = assign15030_body19_e21207;
            let (assign15030_body20_e21239, assign15030_body20_e21239_d_n0, assign15030_body20_e21239_d_n2, assign15030_body20_e21239_d_n6, assign15030_body20_e21239_d_n7, assign15030_body20_e21239_d_n10, assign15030_body20_e21239_d_n11, assign15030_body20_e21239_d_n12, assign15030_body20_e21239_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body20_e21217: f64 = (locals.var_chi * locals.var_chi);
        let assign15030_body20_e21219: f64 = (assign15030_body20_e21217 / 2.0);
        let assign15030_body20_e21223: f64 = (locals.var_chi / 3.0);
        let assign15030_body20_e21227: f64 = (locals.var_chi / 4.0);
        let assign15030_body20_e21231: f64 = (locals.var_chi / 5.0);
        let assign15030_body20_e21232: f64 = (1.0 - assign15030_body20_e21231);
        let assign15030_body20_e21233: f64 = (assign15030_body20_e21227 * assign15030_body20_e21232);
        let assign15030_body20_e21234: f64 = (1.0 - assign15030_body20_e21233);
        let assign15030_body20_e21235: f64 = (assign15030_body20_e21223 * assign15030_body20_e21234);
        let assign15030_body20_e21236: f64 = (1.0 - assign15030_body20_e21235);
        let assign15030_body20_e21237: f64 = (assign15030_body20_e21219 * assign15030_body20_e21236);
        (assign15030_body20_e21237, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn0 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn0 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn2 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn2 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn6 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn6 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn7 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn7 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn10 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn10 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn11 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn11 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn12 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn12 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn17 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn17 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15030_body20_e21239;
            locals.var_t0_dn0 = assign15030_body20_e21239_d_n0;
            locals.var_t0_dn2 = assign15030_body20_e21239_d_n2;
            locals.var_t0_dn6 = assign15030_body20_e21239_d_n6;
            locals.var_t0_dn7 = assign15030_body20_e21239_d_n7;
            locals.var_t0_dn10 = assign15030_body20_e21239_d_n10;
            locals.var_t0_dn11 = assign15030_body20_e21239_d_n11;
            locals.var_t0_dn12 = assign15030_body20_e21239_d_n12;
            locals.var_t0_dn17 = assign15030_body20_e21239_d_n17;
            let (assign15030_body21_e21267, assign15030_body21_e21267_d_n0, assign15030_body21_e21267_d_n2, assign15030_body21_e21267_d_n6, assign15030_body21_e21267_d_n7, assign15030_body21_e21267_d_n10, assign15030_body21_e21267_d_n11, assign15030_body21_e21267_d_n12, assign15030_body21_e21267_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body21_e21251: f64 = (locals.var_chi / 2.0);
        let assign15030_body21_e21255: f64 = (locals.var_chi / 3.0);
        let assign15030_body21_e21259: f64 = (locals.var_chi / 4.0);
        let assign15030_body21_e21260: f64 = (1.0 - assign15030_body21_e21259);
        let assign15030_body21_e21261: f64 = (assign15030_body21_e21255 * assign15030_body21_e21260);
        let assign15030_body21_e21262: f64 = (1.0 - assign15030_body21_e21261);
        let assign15030_body21_e21263: f64 = (assign15030_body21_e21251 * assign15030_body21_e21262);
        let assign15030_body21_e21264: f64 = (1.0 - assign15030_body21_e21263);
        let assign15030_body21_e21265: f64 = (locals.var_chi * assign15030_body21_e21264);
        (assign15030_body21_e21265, ((locals.var_chi_dn0 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn0 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn2 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn6 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn7 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn10 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn11 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn12 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn17 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15030_body21_e21267;
            locals.var_t1_dn0 = assign15030_body21_e21267_d_n0;
            locals.var_t1_dn2 = assign15030_body21_e21267_d_n2;
            locals.var_t1_dn6 = assign15030_body21_e21267_d_n6;
            locals.var_t1_dn7 = assign15030_body21_e21267_d_n7;
            locals.var_t1_dn10 = assign15030_body21_e21267_d_n10;
            locals.var_t1_dn11 = assign15030_body21_e21267_d_n11;
            locals.var_t1_dn12 = assign15030_body21_e21267_d_n12;
            locals.var_t1_dn17 = assign15030_body21_e21267_d_n17;
            let (assign15030_body22_e21299, assign15030_body22_e21299_d_n0, assign15030_body22_e21299_d_n2, assign15030_body22_e21299_d_n6, assign15030_body22_e21299_d_n7, assign15030_body22_e21299_d_n10, assign15030_body22_e21299_d_n11, assign15030_body22_e21299_d_n12, assign15030_body22_e21299_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body22_e21277: f64 = (locals.var_chib * locals.var_chib);
        let assign15030_body22_e21279: f64 = (assign15030_body22_e21277 / 2.0);
        let assign15030_body22_e21283: f64 = (locals.var_chib / 3.0);
        let assign15030_body22_e21287: f64 = (locals.var_chib / 4.0);
        let assign15030_body22_e21291: f64 = (locals.var_chib / 5.0);
        let assign15030_body22_e21292: f64 = (1.0 - assign15030_body22_e21291);
        let assign15030_body22_e21293: f64 = (assign15030_body22_e21287 * assign15030_body22_e21292);
        let assign15030_body22_e21294: f64 = (1.0 - assign15030_body22_e21293);
        let assign15030_body22_e21295: f64 = (assign15030_body22_e21283 * assign15030_body22_e21294);
        let assign15030_body22_e21296: f64 = (1.0 - assign15030_body22_e21295);
        let assign15030_body22_e21297: f64 = (assign15030_body22_e21279 * assign15030_body22_e21296);
        (assign15030_body22_e21297, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn0 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn0 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn2 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn2 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn6 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn6 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn7 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn7 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn10 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn10 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn11 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn11 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn12 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn12 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn17 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn17 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign15030_body22_e21299;
            locals.var_t2_dn0 = assign15030_body22_e21299_d_n0;
            locals.var_t2_dn2 = assign15030_body22_e21299_d_n2;
            locals.var_t2_dn6 = assign15030_body22_e21299_d_n6;
            locals.var_t2_dn7 = assign15030_body22_e21299_d_n7;
            locals.var_t2_dn10 = assign15030_body22_e21299_d_n10;
            locals.var_t2_dn11 = assign15030_body22_e21299_d_n11;
            locals.var_t2_dn12 = assign15030_body22_e21299_d_n12;
            locals.var_t2_dn17 = assign15030_body22_e21299_d_n17;
            let (assign15030_body23_e21327, assign15030_body23_e21327_d_n0, assign15030_body23_e21327_d_n2, assign15030_body23_e21327_d_n6, assign15030_body23_e21327_d_n7, assign15030_body23_e21327_d_n10, assign15030_body23_e21327_d_n11, assign15030_body23_e21327_d_n12, assign15030_body23_e21327_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body23_e21311: f64 = (locals.var_chib / 2.0);
        let assign15030_body23_e21315: f64 = (locals.var_chib / 3.0);
        let assign15030_body23_e21319: f64 = (locals.var_chib / 4.0);
        let assign15030_body23_e21320: f64 = (1.0 - assign15030_body23_e21319);
        let assign15030_body23_e21321: f64 = (assign15030_body23_e21315 * assign15030_body23_e21320);
        let assign15030_body23_e21322: f64 = (1.0 - assign15030_body23_e21321);
        let assign15030_body23_e21323: f64 = (assign15030_body23_e21311 * assign15030_body23_e21322);
        let assign15030_body23_e21324: f64 = (1.0 - assign15030_body23_e21323);
        let assign15030_body23_e21325: f64 = (locals.var_chib * assign15030_body23_e21324);
        (assign15030_body23_e21325, ((locals.var_chib_dn0 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn0 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn2 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn6 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn7 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn10 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn11 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn12 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn17 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign15030_body23_e21327;
            locals.var_t3_dn0 = assign15030_body23_e21327_d_n0;
            locals.var_t3_dn2 = assign15030_body23_e21327_d_n2;
            locals.var_t3_dn6 = assign15030_body23_e21327_d_n6;
            locals.var_t3_dn7 = assign15030_body23_e21327_d_n7;
            locals.var_t3_dn10 = assign15030_body23_e21327_d_n10;
            locals.var_t3_dn11 = assign15030_body23_e21327_d_n11;
            locals.var_t3_dn12 = assign15030_body23_e21327_d_n12;
            locals.var_t3_dn17 = assign15030_body23_e21327_d_n17;
            let (assign15030_body24_e21340, assign15030_body24_e21340_d_n0, assign15030_body24_e21340_d_n2, assign15030_body24_e21340_d_n6, assign15030_body24_e21340_d_n7, assign15030_body24_e21340_d_n10, assign15030_body24_e21340_d_n11, assign15030_body24_e21340_d_n12, assign15030_body24_e21340_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body24_e21337: f64 = (locals.var_t0 - locals.var_t2);
        let assign15030_body24_e21338: f64 = (assign15030_body24_e21337).sqrt();
        (assign15030_body24_e21338, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign15030_body24_e21338)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15030_body24_e21340;
            locals.var_fb_dn0 = assign15030_body24_e21340_d_n0;
            locals.var_fb_dn2 = assign15030_body24_e21340_d_n2;
            locals.var_fb_dn6 = assign15030_body24_e21340_d_n6;
            locals.var_fb_dn7 = assign15030_body24_e21340_d_n7;
            locals.var_fb_dn10 = assign15030_body24_e21340_d_n10;
            locals.var_fb_dn11 = assign15030_body24_e21340_d_n11;
            locals.var_fb_dn12 = assign15030_body24_e21340_d_n12;
            locals.var_fb_dn17 = assign15030_body24_e21340_d_n17;
            let (assign15030_body25_e21360, assign15030_body25_e21360_d_n0, assign15030_body25_e21360_d_n2, assign15030_body25_e21360_d_n6, assign15030_body25_e21360_d_n7, assign15030_body25_e21360_d_n10, assign15030_body25_e21360_d_n11, assign15030_body25_e21360_d_n12, assign15030_body25_e21360_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body25_e21350: f64 = (locals.var_beta * 0.5);
        let assign15030_body25_e21354: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign15030_body25_e21355: f64 = (locals.var_t1 - assign15030_body25_e21354);
        let assign15030_body25_e21356: f64 = (assign15030_body25_e21350 * assign15030_body25_e21355);
        let assign15030_body25_e21358: f64 = (assign15030_body25_e21356 / locals.var_fb);
        (assign15030_body25_e21358, ((((assign15030_body25_e21350 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign15030_body25_e21355) + (assign15030_body25_e21350 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15030_body25_e21360;
            locals.var_fb_dpss_dn0 = assign15030_body25_e21360_d_n0;
            locals.var_fb_dpss_dn2 = assign15030_body25_e21360_d_n2;
            locals.var_fb_dpss_dn6 = assign15030_body25_e21360_d_n6;
            locals.var_fb_dpss_dn7 = assign15030_body25_e21360_d_n7;
            locals.var_fb_dpss_dn10 = assign15030_body25_e21360_d_n10;
            locals.var_fb_dpss_dn11 = assign15030_body25_e21360_d_n11;
            locals.var_fb_dpss_dn12 = assign15030_body25_e21360_d_n12;
            locals.var_fb_dpss_dn17 = assign15030_body25_e21360_d_n17;
            let (assign15030_body26_e21373, assign15030_body26_e21373_d_n0, assign15030_body26_e21373_d_n2, assign15030_body26_e21373_d_n6, assign15030_body26_e21373_d_n7, assign15030_body26_e21373_d_n10, assign15030_body26_e21373_d_n11, assign15030_body26_e21373_d_n12, assign15030_body26_e21373_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) {
        let assign15030_body26_e21370: f64 = (-locals.var_chi);
        let assign15030_body26_e21371: f64 = (assign15030_body26_e21370).exp();
        (assign15030_body26_e21371, (assign15030_body26_e21371 * (-locals.var_chi_dn0)), (assign15030_body26_e21371 * (-locals.var_chi_dn2)), (assign15030_body26_e21371 * (-locals.var_chi_dn6)), (assign15030_body26_e21371 * (-locals.var_chi_dn7)), (assign15030_body26_e21371 * (-locals.var_chi_dn10)), (assign15030_body26_e21371 * (-locals.var_chi_dn11)), (assign15030_body26_e21371 * (-locals.var_chi_dn12)), (assign15030_body26_e21371 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15030_body26_e21373;
            locals.var_t0_dn0 = assign15030_body26_e21373_d_n0;
            locals.var_t0_dn2 = assign15030_body26_e21373_d_n2;
            locals.var_t0_dn6 = assign15030_body26_e21373_d_n6;
            locals.var_t0_dn7 = assign15030_body26_e21373_d_n7;
            locals.var_t0_dn10 = assign15030_body26_e21373_d_n10;
            locals.var_t0_dn11 = assign15030_body26_e21373_d_n11;
            locals.var_t0_dn12 = assign15030_body26_e21373_d_n12;
            locals.var_t0_dn17 = assign15030_body26_e21373_d_n17;
            let (assign15030_body27_e21386, assign15030_body27_e21386_d_n0, assign15030_body27_e21386_d_n2, assign15030_body27_e21386_d_n6, assign15030_body27_e21386_d_n7, assign15030_body27_e21386_d_n10, assign15030_body27_e21386_d_n11, assign15030_body27_e21386_d_n12, assign15030_body27_e21386_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) {
        let assign15030_body27_e21383: f64 = (-locals.var_chib);
        let assign15030_body27_e21384: f64 = (assign15030_body27_e21383).exp();
        (assign15030_body27_e21384, (assign15030_body27_e21384 * (-locals.var_chib_dn0)), (assign15030_body27_e21384 * (-locals.var_chib_dn2)), (assign15030_body27_e21384 * (-locals.var_chib_dn6)), (assign15030_body27_e21384 * (-locals.var_chib_dn7)), (assign15030_body27_e21384 * (-locals.var_chib_dn10)), (assign15030_body27_e21384 * (-locals.var_chib_dn11)), (assign15030_body27_e21384 * (-locals.var_chib_dn12)), (assign15030_body27_e21384 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15030_body27_e21386;
            locals.var_t1_dn0 = assign15030_body27_e21386_d_n0;
            locals.var_t1_dn2 = assign15030_body27_e21386_d_n2;
            locals.var_t1_dn6 = assign15030_body27_e21386_d_n6;
            locals.var_t1_dn7 = assign15030_body27_e21386_d_n7;
            locals.var_t1_dn10 = assign15030_body27_e21386_d_n10;
            locals.var_t1_dn11 = assign15030_body27_e21386_d_n11;
            locals.var_t1_dn12 = assign15030_body27_e21386_d_n12;
            locals.var_t1_dn17 = assign15030_body27_e21386_d_n17;
            let (assign15030_body28_e21404, assign15030_body28_e21404_d_n0, assign15030_body28_e21404_d_n2, assign15030_body28_e21404_d_n6, assign15030_body28_e21404_d_n7, assign15030_body28_e21404_d_n10, assign15030_body28_e21404_d_n11, assign15030_body28_e21404_d_n12, assign15030_body28_e21404_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) {
        let assign15030_body28_e21397: f64 = (locals.var_chi - locals.var_chib);
        let assign15030_body28_e21400: f64 = (locals.var_t0 - locals.var_t1);
        let assign15030_body28_e21401: f64 = (assign15030_body28_e21397 + assign15030_body28_e21400);
        let assign15030_body28_e21402: f64 = (assign15030_body28_e21401).sqrt();
        (assign15030_body28_e21402, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign15030_body28_e21402)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15030_body28_e21404;
            locals.var_fb_dn0 = assign15030_body28_e21404_d_n0;
            locals.var_fb_dn2 = assign15030_body28_e21404_d_n2;
            locals.var_fb_dn6 = assign15030_body28_e21404_d_n6;
            locals.var_fb_dn7 = assign15030_body28_e21404_d_n7;
            locals.var_fb_dn10 = assign15030_body28_e21404_d_n10;
            locals.var_fb_dn11 = assign15030_body28_e21404_d_n11;
            locals.var_fb_dn12 = assign15030_body28_e21404_d_n12;
            locals.var_fb_dn17 = assign15030_body28_e21404_d_n17;
            let (assign15030_body29_e21429, assign15030_body29_e21429_d_n0, assign15030_body29_e21429_d_n2, assign15030_body29_e21429_d_n6, assign15030_body29_e21429_d_n7, assign15030_body29_e21429_d_n10, assign15030_body29_e21429_d_n11, assign15030_body29_e21429_d_n12, assign15030_body29_e21429_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) {
        let assign15030_body29_e21415: f64 = (locals.var_beta * 0.5);
        let assign15030_body29_e21418: f64 = (1.0 - locals.var_t0);
        let assign15030_body29_e21422: f64 = (1.0 - locals.var_t1);
        let assign15030_body29_e21423: f64 = (locals.var_phi_soib_dpss * assign15030_body29_e21422);
        let assign15030_body29_e21424: f64 = (assign15030_body29_e21418 - assign15030_body29_e21423);
        let assign15030_body29_e21425: f64 = (assign15030_body29_e21415 * assign15030_body29_e21424);
        let assign15030_body29_e21427: f64 = (assign15030_body29_e21425 / locals.var_fb);
        (assign15030_body29_e21427, ((((assign15030_body29_e21415 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign15030_body29_e21424) + (assign15030_body29_e21415 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15030_body29_e21429;
            locals.var_fb_dpss_dn0 = assign15030_body29_e21429_d_n0;
            locals.var_fb_dpss_dn2 = assign15030_body29_e21429_d_n2;
            locals.var_fb_dpss_dn6 = assign15030_body29_e21429_d_n6;
            locals.var_fb_dpss_dn7 = assign15030_body29_e21429_d_n7;
            locals.var_fb_dpss_dn10 = assign15030_body29_e21429_d_n10;
            locals.var_fb_dpss_dn11 = assign15030_body29_e21429_d_n11;
            locals.var_fb_dpss_dn12 = assign15030_body29_e21429_d_n12;
            locals.var_fb_dpss_dn17 = assign15030_body29_e21429_d_n17;
            let assign15030_body30_e21436: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard451 = assign15030_body30_e21436;
            let (assign15030_body31_e21444,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard451 != 0.0)) {
        let assign15030_body31_e21442: f64 = (-1.0);
        (assign15030_body31_e21442,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign15030_body31_e21444;
            let assign15030_body32_e21447: f64 = (-1.0);
            let assign15030_body32_e21448: f64 = if locals.var_flg_zone == assign15030_body32_e21447 { 1.0 } else { 0.0 };
            locals.var_guard452 = assign15030_body32_e21448;
            let (assign15030_body33_e21455, assign15030_body33_e21455_d_n0, assign15030_body33_e21455_d_n2, assign15030_body33_e21455_d_n6, assign15030_body33_e21455_d_n7, assign15030_body33_e21455_d_n10, assign15030_body33_e21455_d_n11, assign15030_body33_e21455_d_n12, assign15030_body33_e21455_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard452 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign15030_body33_e21455;
            locals.var_wdsoi_dn0 = assign15030_body33_e21455_d_n0;
            locals.var_wdsoi_dn2 = assign15030_body33_e21455_d_n2;
            locals.var_wdsoi_dn6 = assign15030_body33_e21455_d_n6;
            locals.var_wdsoi_dn7 = assign15030_body33_e21455_d_n7;
            locals.var_wdsoi_dn10 = assign15030_body33_e21455_d_n10;
            locals.var_wdsoi_dn11 = assign15030_body33_e21455_d_n11;
            locals.var_wdsoi_dn12 = assign15030_body33_e21455_d_n12;
            locals.var_wdsoi_dn17 = assign15030_body33_e21455_d_n17;
            let (assign15030_body34_e21465, assign15030_body34_e21465_d_n0, assign15030_body34_e21465_d_n2, assign15030_body34_e21465_d_n6, assign15030_body34_e21465_d_n7, assign15030_body34_e21465_d_n10, assign15030_body34_e21465_d_n11, assign15030_body34_e21465_d_n12, assign15030_body34_e21465_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard452 == 0.0)) {
        let assign15030_body34_e21463: f64 = (locals.var_c_w_soi * locals.var_fb);
        (assign15030_body34_e21463, ((locals.var_c_w_soi_dn0 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn0)), ((locals.var_c_w_soi_dn2 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn2)), ((locals.var_c_w_soi_dn6 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn6)), ((locals.var_c_w_soi_dn7 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn7)), ((locals.var_c_w_soi_dn10 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn10)), ((locals.var_c_w_soi_dn11 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn11)), ((locals.var_c_w_soi_dn12 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn12)), ((locals.var_c_w_soi_dn17 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn17)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign15030_body34_e21465;
            locals.var_wdsoi_dn0 = assign15030_body34_e21465_d_n0;
            locals.var_wdsoi_dn2 = assign15030_body34_e21465_d_n2;
            locals.var_wdsoi_dn6 = assign15030_body34_e21465_d_n6;
            locals.var_wdsoi_dn7 = assign15030_body34_e21465_d_n7;
            locals.var_wdsoi_dn10 = assign15030_body34_e21465_d_n10;
            locals.var_wdsoi_dn11 = assign15030_body34_e21465_d_n11;
            locals.var_wdsoi_dn12 = assign15030_body34_e21465_d_n12;
            locals.var_wdsoi_dn17 = assign15030_body34_e21465_d_n17;
            let assign15030_body35_e21469: f64 = (p.p237 * 1.01);
            let assign15030_body35_e21470: f64 = if locals.var_wdsoi < assign15030_body35_e21469 { 1.0 } else { 0.0 };
            locals.var_guard453 = assign15030_body35_e21470;
            let (assign15030_body36_e21477,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard453 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
            locals.var_flg_depmode = assign15030_body36_e21477;
            let (assign15030_body37_e21485,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard453 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
            locals.var_flg_depmode = assign15030_body37_e21485;
            let (assign15030_body38_e21492, assign15030_body38_e21492_d_n0, assign15030_body38_e21492_d_n2, assign15030_body38_e21492_d_n6, assign15030_body38_e21492_d_n7, assign15030_body38_e21492_d_n10, assign15030_body38_e21492_d_n11, assign15030_body38_e21492_d_n12, assign15030_body38_e21492_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body38_e21490: f64 = (locals.var_q_nsub * locals.var_wdsoi);
        (assign15030_body38_e21490, ((locals.var_q_nsub_dn0 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn0)), ((locals.var_q_nsub_dn2 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn2)), ((locals.var_q_nsub_dn6 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn6)), ((locals.var_q_nsub_dn7 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn7)), ((locals.var_q_nsub_dn10 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn10)), ((locals.var_q_nsub_dn11 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn11)), ((locals.var_q_nsub_dn12 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn12)), ((locals.var_q_nsub_dn17 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn17)),)
    } else {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    }
};
            locals.var_q_dep_soi = assign15030_body38_e21492;
            locals.var_q_dep_soi_dn0 = assign15030_body38_e21492_d_n0;
            locals.var_q_dep_soi_dn2 = assign15030_body38_e21492_d_n2;
            locals.var_q_dep_soi_dn6 = assign15030_body38_e21492_d_n6;
            locals.var_q_dep_soi_dn7 = assign15030_body38_e21492_d_n7;
            locals.var_q_dep_soi_dn10 = assign15030_body38_e21492_d_n10;
            locals.var_q_dep_soi_dn11 = assign15030_body38_e21492_d_n11;
            locals.var_q_dep_soi_dn12 = assign15030_body38_e21492_d_n12;
            locals.var_q_dep_soi_dn17 = assign15030_body38_e21492_d_n17;
            let assign15030_body39_e21495: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard454 = assign15030_body39_e21495;
            let (assign15030_body40_e21503, assign15030_body40_e21503_d_n0, assign15030_body40_e21503_d_n2, assign15030_body40_e21503_d_n6, assign15030_body40_e21503_d_n7, assign15030_body40_e21503_d_n10, assign15030_body40_e21503_d_n11, assign15030_body40_e21503_d_n12, assign15030_body40_e21503_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard454 != 0.0)) {
        let assign15030_body40_e21501: f64 = (-locals.var_fb);
        (assign15030_body40_e21501, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15030_body40_e21503;
            locals.var_fs02_dn0 = assign15030_body40_e21503_d_n0;
            locals.var_fs02_dn2 = assign15030_body40_e21503_d_n2;
            locals.var_fs02_dn6 = assign15030_body40_e21503_d_n6;
            locals.var_fs02_dn7 = assign15030_body40_e21503_d_n7;
            locals.var_fs02_dn10 = assign15030_body40_e21503_d_n10;
            locals.var_fs02_dn11 = assign15030_body40_e21503_d_n11;
            locals.var_fs02_dn12 = assign15030_body40_e21503_d_n12;
            locals.var_fs02_dn17 = assign15030_body40_e21503_d_n17;
            let (assign15030_body41_e21511, assign15030_body41_e21511_d_n0, assign15030_body41_e21511_d_n2, assign15030_body41_e21511_d_n6, assign15030_body41_e21511_d_n7, assign15030_body41_e21511_d_n10, assign15030_body41_e21511_d_n11, assign15030_body41_e21511_d_n12, assign15030_body41_e21511_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard454 != 0.0)) {
        let assign15030_body41_e21509: f64 = (-locals.var_fb_dpss);
        (assign15030_body41_e21509, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15030_body41_e21511;
            locals.var_fs02_dps0_dn0 = assign15030_body41_e21511_d_n0;
            locals.var_fs02_dps0_dn2 = assign15030_body41_e21511_d_n2;
            locals.var_fs02_dps0_dn6 = assign15030_body41_e21511_d_n6;
            locals.var_fs02_dps0_dn7 = assign15030_body41_e21511_d_n7;
            locals.var_fs02_dps0_dn10 = assign15030_body41_e21511_d_n10;
            locals.var_fs02_dps0_dn11 = assign15030_body41_e21511_d_n11;
            locals.var_fs02_dps0_dn12 = assign15030_body41_e21511_d_n12;
            locals.var_fs02_dps0_dn17 = assign15030_body41_e21511_d_n17;
            let assign15030_body42_e21514: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard455 = assign15030_body42_e21514;
            let (assign15030_body43_e21524, assign15030_body43_e21524_d_n0, assign15030_body43_e21524_d_n2, assign15030_body43_e21524_d_n6, assign15030_body43_e21524_d_n7, assign15030_body43_e21524_d_n10, assign15030_body43_e21524_d_n11, assign15030_body43_e21524_d_n12, assign15030_body43_e21524_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15030_body43_e21524;
            locals.var_fs02_dn0 = assign15030_body43_e21524_d_n0;
            locals.var_fs02_dn2 = assign15030_body43_e21524_d_n2;
            locals.var_fs02_dn6 = assign15030_body43_e21524_d_n6;
            locals.var_fs02_dn7 = assign15030_body43_e21524_d_n7;
            locals.var_fs02_dn10 = assign15030_body43_e21524_d_n10;
            locals.var_fs02_dn11 = assign15030_body43_e21524_d_n11;
            locals.var_fs02_dn12 = assign15030_body43_e21524_d_n12;
            locals.var_fs02_dn17 = assign15030_body43_e21524_d_n17;
            let (assign15030_body44_e21534, assign15030_body44_e21534_d_n0, assign15030_body44_e21534_d_n2, assign15030_body44_e21534_d_n6, assign15030_body44_e21534_d_n7, assign15030_body44_e21534_d_n10, assign15030_body44_e21534_d_n11, assign15030_body44_e21534_d_n12, assign15030_body44_e21534_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15030_body44_e21534;
            locals.var_fs02_dps0_dn0 = assign15030_body44_e21534_d_n0;
            locals.var_fs02_dps0_dn2 = assign15030_body44_e21534_d_n2;
            locals.var_fs02_dps0_dn6 = assign15030_body44_e21534_d_n6;
            locals.var_fs02_dps0_dn7 = assign15030_body44_e21534_d_n7;
            locals.var_fs02_dps0_dn10 = assign15030_body44_e21534_d_n10;
            locals.var_fs02_dps0_dn11 = assign15030_body44_e21534_d_n11;
            locals.var_fs02_dps0_dn12 = assign15030_body44_e21534_d_n12;
            locals.var_fs02_dps0_dn17 = assign15030_body44_e21534_d_n17;
            let assign15030_body45_e21537: f64 = if locals.var_chi < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard456 = assign15030_body45_e21537;
            let (assign15030_body46_e21551, assign15030_body46_e21551_d_n0, assign15030_body46_e21551_d_n2, assign15030_body46_e21551_d_n6, assign15030_body46_e21551_d_n7, assign15030_body46_e21551_d_n10, assign15030_body46_e21551_d_n11, assign15030_body46_e21551_d_n12, assign15030_body46_e21551_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign15030_body46_e21549: f64 = (locals.var_chi).exp();
        (assign15030_body46_e21549, (assign15030_body46_e21549 * locals.var_chi_dn0), (assign15030_body46_e21549 * locals.var_chi_dn2), (assign15030_body46_e21549 * locals.var_chi_dn6), (assign15030_body46_e21549 * locals.var_chi_dn7), (assign15030_body46_e21549 * locals.var_chi_dn10), (assign15030_body46_e21549 * locals.var_chi_dn11), (assign15030_body46_e21549 * locals.var_chi_dn12), (assign15030_body46_e21549 * locals.var_chi_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign15030_body46_e21551;
            locals.var_exp_chi_dn0 = assign15030_body46_e21551_d_n0;
            locals.var_exp_chi_dn2 = assign15030_body46_e21551_d_n2;
            locals.var_exp_chi_dn6 = assign15030_body46_e21551_d_n6;
            locals.var_exp_chi_dn7 = assign15030_body46_e21551_d_n7;
            locals.var_exp_chi_dn10 = assign15030_body46_e21551_d_n10;
            locals.var_exp_chi_dn11 = assign15030_body46_e21551_d_n11;
            locals.var_exp_chi_dn12 = assign15030_body46_e21551_d_n12;
            locals.var_exp_chi_dn17 = assign15030_body46_e21551_d_n17;
            let (assign15030_body47_e21570, assign15030_body47_e21570_d_n0, assign15030_body47_e21570_d_n2, assign15030_body47_e21570_d_n6, assign15030_body47_e21570_d_n7, assign15030_body47_e21570_d_n10, assign15030_body47_e21570_d_n11, assign15030_body47_e21570_d_n12, assign15030_body47_e21570_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign15030_body47_e21566: f64 = (locals.var_chi + 1.0);
        let assign15030_body47_e21567: f64 = (locals.var_exp_chi - assign15030_body47_e21566);
        let assign15030_body47_e21568: f64 = (locals.var_cfs1 * assign15030_body47_e21567);
        (assign15030_body47_e21568, ((locals.var_cfs1_dn0 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn6 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn10 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn12 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn12 - locals.var_chi_dn12))), ((locals.var_cfs1_dn17 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn17 - locals.var_chi_dn17))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign15030_body47_e21570;
            locals.var_fs01_dn0 = assign15030_body47_e21570_d_n0;
            locals.var_fs01_dn2 = assign15030_body47_e21570_d_n2;
            locals.var_fs01_dn6 = assign15030_body47_e21570_d_n6;
            locals.var_fs01_dn7 = assign15030_body47_e21570_d_n7;
            locals.var_fs01_dn10 = assign15030_body47_e21570_d_n10;
            locals.var_fs01_dn11 = assign15030_body47_e21570_d_n11;
            locals.var_fs01_dn12 = assign15030_body47_e21570_d_n12;
            locals.var_fs01_dn17 = assign15030_body47_e21570_d_n17;
            let (assign15030_body48_e21589, assign15030_body48_e21589_d_n0, assign15030_body48_e21589_d_n2, assign15030_body48_e21589_d_n6, assign15030_body48_e21589_d_n7, assign15030_body48_e21589_d_n10, assign15030_body48_e21589_d_n11, assign15030_body48_e21589_d_n12, assign15030_body48_e21589_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign15030_body48_e21583: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign15030_body48_e21586: f64 = (locals.var_exp_chi - 1.0);
        let assign15030_body48_e21587: f64 = (assign15030_body48_e21583 * assign15030_body48_e21586);
        (assign15030_body48_e21587, (((locals.var_cfs1_dn0 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn0)), (((locals.var_cfs1_dn2 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn2)), (((locals.var_cfs1_dn6 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn6)), (((locals.var_cfs1_dn7 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn10)), (((locals.var_cfs1_dn11 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn11)), (((locals.var_cfs1_dn12 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn12)), (((locals.var_cfs1_dn17 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign15030_body48_e21589;
            locals.var_fs01_dps0_dn0 = assign15030_body48_e21589_d_n0;
            locals.var_fs01_dps0_dn2 = assign15030_body48_e21589_d_n2;
            locals.var_fs01_dps0_dn6 = assign15030_body48_e21589_d_n6;
            locals.var_fs01_dps0_dn7 = assign15030_body48_e21589_d_n7;
            locals.var_fs01_dps0_dn10 = assign15030_body48_e21589_d_n10;
            locals.var_fs01_dps0_dn11 = assign15030_body48_e21589_d_n11;
            locals.var_fs01_dps0_dn12 = assign15030_body48_e21589_d_n12;
            locals.var_fs01_dps0_dn17 = assign15030_body48_e21589_d_n17;
            let (assign15030_body49_e21606, assign15030_body49_e21606_d_n0, assign15030_body49_e21606_d_n2, assign15030_body49_e21606_d_n6, assign15030_body49_e21606_d_n7, assign15030_body49_e21606_d_n10, assign15030_body49_e21606_d_n11, assign15030_body49_e21606_d_n12, assign15030_body49_e21606_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
        let assign15030_body49_e21603: f64 = (locals.var_beta * locals.var_phi_s0_soi);
        let assign15030_body49_e21604: f64 = (assign15030_body49_e21603).exp();
        (assign15030_body49_e21604, (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn0)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn2)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn6)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn7)), (assign15030_body49_e21604 * ((locals.var_beta_dn10 * locals.var_phi_s0_soi) + (locals.var_beta * locals.var_phi_s0_soi_dn10))), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn11)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn12)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn17)),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn12, locals.var_exp_bps0_dn17,)
    }
};
            locals.var_exp_bps0 = assign15030_body49_e21606;
            locals.var_exp_bps0_dn0 = assign15030_body49_e21606_d_n0;
            locals.var_exp_bps0_dn2 = assign15030_body49_e21606_d_n2;
            locals.var_exp_bps0_dn6 = assign15030_body49_e21606_d_n6;
            locals.var_exp_bps0_dn7 = assign15030_body49_e21606_d_n7;
            locals.var_exp_bps0_dn10 = assign15030_body49_e21606_d_n10;
            locals.var_exp_bps0_dn11 = assign15030_body49_e21606_d_n11;
            locals.var_exp_bps0_dn12 = assign15030_body49_e21606_d_n12;
            locals.var_exp_bps0_dn17 = assign15030_body49_e21606_d_n17;
            let (assign15030_body50_e21628, assign15030_body50_e21628_d_n0, assign15030_body50_e21628_d_n2, assign15030_body50_e21628_d_n6, assign15030_body50_e21628_d_n7, assign15030_body50_e21628_d_n10, assign15030_body50_e21628_d_n11, assign15030_body50_e21628_d_n12, assign15030_body50_e21628_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
        let assign15030_body50_e21623: f64 = (locals.var_chi + 1.0);
        let assign15030_body50_e21624: f64 = (locals.var_exp_bvbs * assign15030_body50_e21623);
        let assign15030_body50_e21625: f64 = (locals.var_exp_bps0 - assign15030_body50_e21624);
        let assign15030_body50_e21626: f64 = (locals.var_cnst1soi * assign15030_body50_e21625);
        (assign15030_body50_e21626, ((locals.var_cnst1soi_dn0 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn12 - ((locals.var_exp_bvbs_dn12 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn17 - ((locals.var_exp_bvbs_dn17 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign15030_body50_e21628;
            locals.var_fs01_dn0 = assign15030_body50_e21628_d_n0;
            locals.var_fs01_dn2 = assign15030_body50_e21628_d_n2;
            locals.var_fs01_dn6 = assign15030_body50_e21628_d_n6;
            locals.var_fs01_dn7 = assign15030_body50_e21628_d_n7;
            locals.var_fs01_dn10 = assign15030_body50_e21628_d_n10;
            locals.var_fs01_dn11 = assign15030_body50_e21628_d_n11;
            locals.var_fs01_dn12 = assign15030_body50_e21628_d_n12;
            locals.var_fs01_dn17 = assign15030_body50_e21628_d_n17;
            let (assign15030_body51_e21648, assign15030_body51_e21648_d_n0, assign15030_body51_e21648_d_n2, assign15030_body51_e21648_d_n6, assign15030_body51_e21648_d_n7, assign15030_body51_e21648_d_n10, assign15030_body51_e21648_d_n11, assign15030_body51_e21648_d_n12, assign15030_body51_e21648_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
        let assign15030_body51_e21642: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign15030_body51_e21645: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign15030_body51_e21646: f64 = (assign15030_body51_e21642 * assign15030_body51_e21645);
        (assign15030_body51_e21646, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn12 - locals.var_exp_bvbs_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn17 - locals.var_exp_bvbs_dn17))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign15030_body51_e21648;
            locals.var_fs01_dps0_dn0 = assign15030_body51_e21648_d_n0;
            locals.var_fs01_dps0_dn2 = assign15030_body51_e21648_d_n2;
            locals.var_fs01_dps0_dn6 = assign15030_body51_e21648_d_n6;
            locals.var_fs01_dps0_dn7 = assign15030_body51_e21648_d_n7;
            locals.var_fs01_dps0_dn10 = assign15030_body51_e21648_d_n10;
            locals.var_fs01_dps0_dn11 = assign15030_body51_e21648_d_n11;
            locals.var_fs01_dps0_dn12 = assign15030_body51_e21648_d_n12;
            locals.var_fs01_dps0_dn17 = assign15030_body51_e21648_d_n17;
            let (assign15030_body52_e21664, assign15030_body52_e21664_d_n0, assign15030_body52_e21664_d_n2, assign15030_body52_e21664_d_n6, assign15030_body52_e21664_d_n7, assign15030_body52_e21664_d_n10, assign15030_body52_e21664_d_n11, assign15030_body52_e21664_d_n12, assign15030_body52_e21664_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) {
        let assign15030_body52_e21659: f64 = (locals.var_fb * locals.var_fb);
        let assign15030_body52_e21661: f64 = (assign15030_body52_e21659 + locals.var_fs01);
        let assign15030_body52_e21662: f64 = (assign15030_body52_e21661).sqrt();
        (assign15030_body52_e21662, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fs01_dn17) / (2.0 * assign15030_body52_e21662)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15030_body52_e21664;
            locals.var_fs02_dn0 = assign15030_body52_e21664_d_n0;
            locals.var_fs02_dn2 = assign15030_body52_e21664_d_n2;
            locals.var_fs02_dn6 = assign15030_body52_e21664_d_n6;
            locals.var_fs02_dn7 = assign15030_body52_e21664_d_n7;
            locals.var_fs02_dn10 = assign15030_body52_e21664_d_n10;
            locals.var_fs02_dn11 = assign15030_body52_e21664_d_n11;
            locals.var_fs02_dn12 = assign15030_body52_e21664_d_n12;
            locals.var_fs02_dn17 = assign15030_body52_e21664_d_n17;
            let (assign15030_body53_e21685, assign15030_body53_e21685_d_n0, assign15030_body53_e21685_d_n2, assign15030_body53_e21685_d_n6, assign15030_body53_e21685_d_n7, assign15030_body53_e21685_d_n10, assign15030_body53_e21685_d_n11, assign15030_body53_e21685_d_n12, assign15030_body53_e21685_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) {
        let assign15030_body53_e21676: f64 = (2.0 * locals.var_fb_dpss);
        let assign15030_body53_e21678: f64 = (assign15030_body53_e21676 * locals.var_fb);
        let assign15030_body53_e21680: f64 = (assign15030_body53_e21678 + locals.var_fs01_dps0);
        let assign15030_body53_e21681: f64 = (0.5 * assign15030_body53_e21680);
        let assign15030_body53_e21683: f64 = (assign15030_body53_e21681 / locals.var_fs02);
        (assign15030_body53_e21683, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn17)) + locals.var_fs01_dps0_dn17)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn17)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15030_body53_e21685;
            locals.var_fs02_dps0_dn0 = assign15030_body53_e21685_d_n0;
            locals.var_fs02_dps0_dn2 = assign15030_body53_e21685_d_n2;
            locals.var_fs02_dps0_dn6 = assign15030_body53_e21685_d_n6;
            locals.var_fs02_dps0_dn7 = assign15030_body53_e21685_d_n7;
            locals.var_fs02_dps0_dn10 = assign15030_body53_e21685_d_n10;
            locals.var_fs02_dps0_dn11 = assign15030_body53_e21685_d_n11;
            locals.var_fs02_dps0_dn12 = assign15030_body53_e21685_d_n12;
            locals.var_fs02_dps0_dn17 = assign15030_body53_e21685_d_n17;
            let (assign15030_body54_e21701, assign15030_body54_e21701_d_n0, assign15030_body54_e21701_d_n2, assign15030_body54_e21701_d_n6, assign15030_body54_e21701_d_n7, assign15030_body54_e21701_d_n10, assign15030_body54_e21701_d_n11, assign15030_body54_e21701_d_n12, assign15030_body54_e21701_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body54_e21689: f64 = (-locals.var_vgp);
        let assign15030_body54_e21691: f64 = (assign15030_body54_e21689 + locals.var_phi_s0_soi);
        let assign15030_body54_e21694: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign15030_body54_e21695: f64 = (assign15030_body54_e21691 + assign15030_body54_e21694);
        let assign15030_body54_e21698: f64 = (locals.var_c_fox_inv * locals.var_qhs);
        let assign15030_body54_e21699: f64 = (assign15030_body54_e21695 - assign15030_body54_e21698);
        (assign15030_body54_e21699, ((((-locals.var_vgp_dn0) + locals.var_phi_s0_soi_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))) - ((locals.var_c_fox_inv_dn0 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn0))), ((((-locals.var_vgp_dn2) + locals.var_phi_s0_soi_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))) - ((locals.var_c_fox_inv_dn2 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn2))), ((((-locals.var_vgp_dn6) + locals.var_phi_s0_soi_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))) - ((locals.var_c_fox_inv_dn6 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn6))), ((((-locals.var_vgp_dn7) + locals.var_phi_s0_soi_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))) - ((locals.var_c_fox_inv_dn7 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn7))), ((((-locals.var_vgp_dn10) + locals.var_phi_s0_soi_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))) - ((locals.var_c_fox_inv_dn10 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn10))), ((((-locals.var_vgp_dn11) + locals.var_phi_s0_soi_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))) - ((locals.var_c_fox_inv_dn11 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn11))), ((((-locals.var_vgp_dn12) + locals.var_phi_s0_soi_dn12) + ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))) - ((locals.var_c_fox_inv_dn12 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn12))), ((((-locals.var_vgp_dn17) + locals.var_phi_s0_soi_dn17) + ((locals.var_fac1_dn17 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn17))) - ((locals.var_c_fox_inv_dn17 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn17))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12, locals.var_fs0_dn17,)
    }
};
            locals.var_fs0 = assign15030_body54_e21701;
            locals.var_fs0_dn0 = assign15030_body54_e21701_d_n0;
            locals.var_fs0_dn2 = assign15030_body54_e21701_d_n2;
            locals.var_fs0_dn6 = assign15030_body54_e21701_d_n6;
            locals.var_fs0_dn7 = assign15030_body54_e21701_d_n7;
            locals.var_fs0_dn10 = assign15030_body54_e21701_d_n10;
            locals.var_fs0_dn11 = assign15030_body54_e21701_d_n11;
            locals.var_fs0_dn12 = assign15030_body54_e21701_d_n12;
            locals.var_fs0_dn17 = assign15030_body54_e21701_d_n17;
            let (assign15030_body55_e21710, assign15030_body55_e21710_d_n0, assign15030_body55_e21710_d_n2, assign15030_body55_e21710_d_n6, assign15030_body55_e21710_d_n7, assign15030_body55_e21710_d_n10, assign15030_body55_e21710_d_n11, assign15030_body55_e21710_d_n12, assign15030_body55_e21710_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body55_e21707: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign15030_body55_e21708: f64 = (1.0 + assign15030_body55_e21707);
        (assign15030_body55_e21708, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12)), ((locals.var_fac1_dn17 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn17)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12, locals.var_fs0_dps0_dn17,)
    }
};
            locals.var_fs0_dps0 = assign15030_body55_e21710;
            locals.var_fs0_dps0_dn0 = assign15030_body55_e21710_d_n0;
            locals.var_fs0_dps0_dn2 = assign15030_body55_e21710_d_n2;
            locals.var_fs0_dps0_dn6 = assign15030_body55_e21710_d_n6;
            locals.var_fs0_dps0_dn7 = assign15030_body55_e21710_d_n7;
            locals.var_fs0_dps0_dn10 = assign15030_body55_e21710_d_n10;
            locals.var_fs0_dps0_dn11 = assign15030_body55_e21710_d_n11;
            locals.var_fs0_dps0_dn12 = assign15030_body55_e21710_d_n12;
            locals.var_fs0_dps0_dn17 = assign15030_body55_e21710_d_n17;
            let assign15030_body56_e21713: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard457 = assign15030_body56_e21713;
            let (assign15030_body57_e21722,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard457 != 0.0)) {
        let assign15030_body57_e21720: f64 = (locals.var_lp_s0_max + 1.0);
        (assign15030_body57_e21720,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign15030_body57_e21722;
            let (assign15030_body58_e21733, assign15030_body58_e21733_d_n0, assign15030_body58_e21733_d_n2, assign15030_body58_e21733_d_n6, assign15030_body58_e21733_d_n7, assign15030_body58_e21733_d_n10, assign15030_body58_e21733_d_n11, assign15030_body58_e21733_d_n12, assign15030_body58_e21733_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) {
        let assign15030_body58_e21729: f64 = (-locals.var_fs0);
        let assign15030_body58_e21731: f64 = (assign15030_body58_e21729 / locals.var_fs0_dps0);
        (assign15030_body58_e21731, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn17) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn17)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign15030_body58_e21733;
            locals.var_dps0_dn0 = assign15030_body58_e21733_d_n0;
            locals.var_dps0_dn2 = assign15030_body58_e21733_d_n2;
            locals.var_dps0_dn6 = assign15030_body58_e21733_d_n6;
            locals.var_dps0_dn7 = assign15030_body58_e21733_d_n7;
            locals.var_dps0_dn10 = assign15030_body58_e21733_d_n10;
            locals.var_dps0_dn11 = assign15030_body58_e21733_d_n11;
            locals.var_dps0_dn12 = assign15030_body58_e21733_d_n12;
            locals.var_dps0_dn17 = assign15030_body58_e21733_d_n17;
            let (assign15030_body59_e21754, assign15030_body59_e21754_d_n0, assign15030_body59_e21754_d_n2, assign15030_body59_e21754_d_n6, assign15030_body59_e21754_d_n7, assign15030_body59_e21754_d_n10, assign15030_body59_e21754_d_n11, assign15030_body59_e21754_d_n12, assign15030_body59_e21754_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) {
        let assign15030_body59_e21741: f64 = (0.5 * 0.1);
        let assign15030_body59_e21745: f64 = (locals.var_phi_s0_soi).abs();
        let (assign15030_body59_e21750, assign15030_body59_e21750_d_n0, assign15030_body59_e21750_d_n2, assign15030_body59_e21750_d_n6, assign15030_body59_e21750_d_n7, assign15030_body59_e21750_d_n10, assign15030_body59_e21750_d_n11, assign15030_body59_e21750_d_n12, assign15030_body59_e21750_d_n17,) = {
            if (1.0 >= assign15030_body59_e21745) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15030_body59_e21749: f64 = (locals.var_phi_s0_soi).abs();
                (assign15030_body59_e21749, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn0 } else { (-locals.var_phi_s0_soi_dn0) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn2 } else { (-locals.var_phi_s0_soi_dn2) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn6 } else { (-locals.var_phi_s0_soi_dn6) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn7 } else { (-locals.var_phi_s0_soi_dn7) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn10 } else { (-locals.var_phi_s0_soi_dn10) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn11 } else { (-locals.var_phi_s0_soi_dn11) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn12 } else { (-locals.var_phi_s0_soi_dn12) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn17 } else { (-locals.var_phi_s0_soi_dn17) },)
            }
        };
        let assign15030_body59_e21751: f64 = (1.0 + assign15030_body59_e21750);
        let assign15030_body59_e21752: f64 = (assign15030_body59_e21741 * assign15030_body59_e21751);
        (assign15030_body59_e21752, (assign15030_body59_e21741 * assign15030_body59_e21750_d_n0), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n2), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n6), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n7), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n10), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n11), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n12), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign15030_body59_e21754;
            locals.var_dplim_dn0 = assign15030_body59_e21754_d_n0;
            locals.var_dplim_dn2 = assign15030_body59_e21754_d_n2;
            locals.var_dplim_dn6 = assign15030_body59_e21754_d_n6;
            locals.var_dplim_dn7 = assign15030_body59_e21754_d_n7;
            locals.var_dplim_dn10 = assign15030_body59_e21754_d_n10;
            locals.var_dplim_dn11 = assign15030_body59_e21754_d_n11;
            locals.var_dplim_dn12 = assign15030_body59_e21754_d_n12;
            locals.var_dplim_dn17 = assign15030_body59_e21754_d_n17;
            let assign15030_body60_e21756: f64 = (locals.var_dps0).abs();
            let assign15030_body60_e21758: f64 = if assign15030_body60_e21756 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard458 = assign15030_body60_e21758;
            let (assign15030_body61_e21776, assign15030_body61_e21776_d_n0, assign15030_body61_e21776_d_n2, assign15030_body61_e21776_d_n6, assign15030_body61_e21776_d_n7, assign15030_body61_e21776_d_n10, assign15030_body61_e21776_d_n11, assign15030_body61_e21776_d_n12, assign15030_body61_e21776_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        let (assign15030_body61_e21773,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign15030_body61_e21772: f64 = (-1.0);
                (assign15030_body61_e21772,)
            }
        };
        let assign15030_body61_e21774: f64 = (locals.var_dplim * assign15030_body61_e21773);
        (assign15030_body61_e21774, (locals.var_dplim_dn0 * assign15030_body61_e21773), (locals.var_dplim_dn2 * assign15030_body61_e21773), (locals.var_dplim_dn6 * assign15030_body61_e21773), (locals.var_dplim_dn7 * assign15030_body61_e21773), (locals.var_dplim_dn10 * assign15030_body61_e21773), (locals.var_dplim_dn11 * assign15030_body61_e21773), (locals.var_dplim_dn12 * assign15030_body61_e21773), (locals.var_dplim_dn17 * assign15030_body61_e21773),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign15030_body61_e21776;
            locals.var_dps0_dn0 = assign15030_body61_e21776_d_n0;
            locals.var_dps0_dn2 = assign15030_body61_e21776_d_n2;
            locals.var_dps0_dn6 = assign15030_body61_e21776_d_n6;
            locals.var_dps0_dn7 = assign15030_body61_e21776_d_n7;
            locals.var_dps0_dn10 = assign15030_body61_e21776_d_n10;
            locals.var_dps0_dn11 = assign15030_body61_e21776_d_n11;
            locals.var_dps0_dn12 = assign15030_body61_e21776_d_n12;
            locals.var_dps0_dn17 = assign15030_body61_e21776_d_n17;
            let (assign15030_body62_e21786, assign15030_body62_e21786_d_n0, assign15030_body62_e21786_d_n2, assign15030_body62_e21786_d_n6, assign15030_body62_e21786_d_n7, assign15030_body62_e21786_d_n10, assign15030_body62_e21786_d_n11, assign15030_body62_e21786_d_n12, assign15030_body62_e21786_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) {
        let assign15030_body62_e21784: f64 = (locals.var_phi_s0_soi + locals.var_dps0);
        (assign15030_body62_e21784, (locals.var_phi_s0_soi_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_soi_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_soi_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_soi_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_soi_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_soi_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_soi_dn12 + locals.var_dps0_dn12), (locals.var_phi_s0_soi_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
            locals.var_phi_s0_soi = assign15030_body62_e21786;
            locals.var_phi_s0_soi_dn0 = assign15030_body62_e21786_d_n0;
            locals.var_phi_s0_soi_dn2 = assign15030_body62_e21786_d_n2;
            locals.var_phi_s0_soi_dn6 = assign15030_body62_e21786_d_n6;
            locals.var_phi_s0_soi_dn7 = assign15030_body62_e21786_d_n7;
            locals.var_phi_s0_soi_dn10 = assign15030_body62_e21786_d_n10;
            locals.var_phi_s0_soi_dn11 = assign15030_body62_e21786_d_n11;
            locals.var_phi_s0_soi_dn12 = assign15030_body62_e21786_d_n12;
            locals.var_phi_s0_soi_dn17 = assign15030_body62_e21786_d_n17;
            let assign15030_body63_e21788: f64 = (locals.var_dps0).abs();
            let assign15030_body63_e21792: f64 = (locals.var_fs0).abs();
            let assign15030_body63_e21795: f64 = if ((assign15030_body63_e21788 <= 5e-12) && (assign15030_body63_e21792 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard459 = assign15030_body63_e21795;
            let (assign15030_body64_e21805,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard459 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign15030_body64_e21805;
            let (assign15030_body65_e21812,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body65_e21810: f64 = (locals.var_lp_s0 + 1.0);
        (assign15030_body65_e21810,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign15030_body65_e21812;
        }

    }

    pub(super) fn stamp_transient_block_50(
        locals: &mut StampLocals,
    ) {
        let (assign15040_e21819,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15040_e21817: f64 = (locals.var_lp_s0 - 1.0);
        (assign15040_e21817,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign15040_e21819;

        let (assign15050_e21824, assign15050_e21824_d_n0, assign15050_e21824_d_n2, assign15050_e21824_d_n6, assign15050_e21824_d_n7, assign15050_e21824_d_n10, assign15050_e21824_d_n11, assign15050_e21824_d_n12, assign15050_e21824_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    } else {
        (locals.var_q_deps0, locals.var_q_deps0_dn0, locals.var_q_deps0_dn2, locals.var_q_deps0_dn6, locals.var_q_deps0_dn7, locals.var_q_deps0_dn10, locals.var_q_deps0_dn11, locals.var_q_deps0_dn12, locals.var_q_deps0_dn17,)
    }
};
        locals.var_q_deps0 = assign15050_e21824;
        locals.var_q_deps0_dn0 = assign15050_e21824_d_n0;
        locals.var_q_deps0_dn2 = assign15050_e21824_d_n2;
        locals.var_q_deps0_dn6 = assign15050_e21824_d_n6;
        locals.var_q_deps0_dn7 = assign15050_e21824_d_n7;
        locals.var_q_deps0_dn10 = assign15050_e21824_d_n10;
        locals.var_q_deps0_dn11 = assign15050_e21824_d_n11;
        locals.var_q_deps0_dn12 = assign15050_e21824_d_n12;
        locals.var_q_deps0_dn17 = assign15050_e21824_d_n17;

        let (assign15060_e21829, assign15060_e21829_d_n0, assign15060_e21829_d_n2, assign15060_e21829_d_n6, assign15060_e21829_d_n7, assign15060_e21829_d_n10, assign15060_e21829_d_n11, assign15060_e21829_d_n12, assign15060_e21829_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_q_deps0, locals.var_q_deps0_dn0, locals.var_q_deps0_dn2, locals.var_q_deps0_dn6, locals.var_q_deps0_dn7, locals.var_q_deps0_dn10, locals.var_q_deps0_dn11, locals.var_q_deps0_dn12, locals.var_q_deps0_dn17,)
    } else {
        (locals.var_q_dep0, locals.var_q_dep0_dn0, locals.var_q_dep0_dn2, locals.var_q_dep0_dn6, locals.var_q_dep0_dn7, locals.var_q_dep0_dn10, locals.var_q_dep0_dn11, locals.var_q_dep0_dn12, locals.var_q_dep0_dn17,)
    }
};
        locals.var_q_dep0 = assign15060_e21829;
        locals.var_q_dep0_dn0 = assign15060_e21829_d_n0;
        locals.var_q_dep0_dn2 = assign15060_e21829_d_n2;
        locals.var_q_dep0_dn6 = assign15060_e21829_d_n6;
        locals.var_q_dep0_dn7 = assign15060_e21829_d_n7;
        locals.var_q_dep0_dn10 = assign15060_e21829_d_n10;
        locals.var_q_dep0_dn11 = assign15060_e21829_d_n11;
        locals.var_q_dep0_dn12 = assign15060_e21829_d_n12;
        locals.var_q_dep0_dn17 = assign15060_e21829_d_n17;

        let (assign15070_e21834, assign15070_e21834_d_n0, assign15070_e21834_d_n2, assign15070_e21834_d_n6, assign15070_e21834_d_n7, assign15070_e21834_d_n10, assign15070_e21834_d_n11, assign15070_e21834_d_n12, assign15070_e21834_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign15070_e21834;
        locals.var_ps0_dn0 = assign15070_e21834_d_n0;
        locals.var_ps0_dn2 = assign15070_e21834_d_n2;
        locals.var_ps0_dn6 = assign15070_e21834_d_n6;
        locals.var_ps0_dn7 = assign15070_e21834_d_n7;
        locals.var_ps0_dn10 = assign15070_e21834_d_n10;
        locals.var_ps0_dn11 = assign15070_e21834_d_n11;
        locals.var_ps0_dn12 = assign15070_e21834_d_n12;
        locals.var_ps0_dn17 = assign15070_e21834_d_n17;

        let (assign15090_e21846, assign15090_e21846_d_n0, assign15090_e21846_d_n2, assign15090_e21846_d_n6, assign15090_e21846_d_n7, assign15090_e21846_d_n10, assign15090_e21846_d_n11, assign15090_e21846_d_n12, assign15090_e21846_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15090_e21844: f64 = (locals.var_q_deps0 / locals.var_cnst0soi);
        (assign15090_e21844, (((locals.var_q_deps0_dn0 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn0)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn2 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn2)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn6 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn6)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn7 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn7)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn10 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn10)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn11 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn11)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn12 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn12)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn17 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn17)) / (locals.var_cnst0soi * locals.var_cnst0soi)),)
    } else {
        (locals.var_q_deps0_soi_o_cnst0soi, locals.var_q_deps0_soi_o_cnst0soi_dn0, locals.var_q_deps0_soi_o_cnst0soi_dn2, locals.var_q_deps0_soi_o_cnst0soi_dn6, locals.var_q_deps0_soi_o_cnst0soi_dn7, locals.var_q_deps0_soi_o_cnst0soi_dn10, locals.var_q_deps0_soi_o_cnst0soi_dn11, locals.var_q_deps0_soi_o_cnst0soi_dn12, locals.var_q_deps0_soi_o_cnst0soi_dn17,)
    }
};
        locals.var_q_deps0_soi_o_cnst0soi = assign15090_e21846;
        locals.var_q_deps0_soi_o_cnst0soi_dn0 = assign15090_e21846_d_n0;
        locals.var_q_deps0_soi_o_cnst0soi_dn2 = assign15090_e21846_d_n2;
        locals.var_q_deps0_soi_o_cnst0soi_dn6 = assign15090_e21846_d_n6;
        locals.var_q_deps0_soi_o_cnst0soi_dn7 = assign15090_e21846_d_n7;
        locals.var_q_deps0_soi_o_cnst0soi_dn10 = assign15090_e21846_d_n10;
        locals.var_q_deps0_soi_o_cnst0soi_dn11 = assign15090_e21846_d_n11;
        locals.var_q_deps0_soi_o_cnst0soi_dn12 = assign15090_e21846_d_n12;
        locals.var_q_deps0_soi_o_cnst0soi_dn17 = assign15090_e21846_d_n17;

        let (assign15100_e21857, assign15100_e21857_d_n0, assign15100_e21857_d_n2, assign15100_e21857_d_n6, assign15100_e21857_d_n7, assign15100_e21857_d_n10, assign15100_e21857_d_n11, assign15100_e21857_d_n12, assign15100_e21857_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15100_e21851: f64 = (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi);
        let assign15100_e21854: f64 = (10.0 * 2.220446049250313e-16);
        let assign15100_e21855: f64 = (assign15100_e21851 + assign15100_e21854);
        (assign15100_e21855, ((locals.var_q_deps0_soi_o_cnst0soi_dn0 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn0)), ((locals.var_q_deps0_soi_o_cnst0soi_dn2 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn2)), ((locals.var_q_deps0_soi_o_cnst0soi_dn6 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn6)), ((locals.var_q_deps0_soi_o_cnst0soi_dn7 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn7)), ((locals.var_q_deps0_soi_o_cnst0soi_dn10 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn10)), ((locals.var_q_deps0_soi_o_cnst0soi_dn11 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn11)), ((locals.var_q_deps0_soi_o_cnst0soi_dn12 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn12)), ((locals.var_q_deps0_soi_o_cnst0soi_dn17 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn17)),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn6, locals.var_xi0_dn7, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12, locals.var_xi0_dn17,)
    }
};
        locals.var_xi0 = assign15100_e21857;
        locals.var_xi0_dn0 = assign15100_e21857_d_n0;
        locals.var_xi0_dn2 = assign15100_e21857_d_n2;
        locals.var_xi0_dn6 = assign15100_e21857_d_n6;
        locals.var_xi0_dn7 = assign15100_e21857_d_n7;
        locals.var_xi0_dn10 = assign15100_e21857_d_n10;
        locals.var_xi0_dn11 = assign15100_e21857_d_n11;
        locals.var_xi0_dn12 = assign15100_e21857_d_n12;
        locals.var_xi0_dn17 = assign15100_e21857_d_n17;

        let (assign15110_e21864, assign15110_e21864_d_n0, assign15110_e21864_d_n2, assign15110_e21864_d_n6, assign15110_e21864_d_n7, assign15110_e21864_d_n10, assign15110_e21864_d_n11, assign15110_e21864_d_n12, assign15110_e21864_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15110_e21862: f64 = (2.0 * locals.var_q_deps0_soi_o_cnst0soi);
        (assign15110_e21862, (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn0), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn2), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn6), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn7), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn10), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn11), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn12), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15110_e21864;
        locals.var_t1_dn0 = assign15110_e21864_d_n0;
        locals.var_t1_dn2 = assign15110_e21864_d_n2;
        locals.var_t1_dn6 = assign15110_e21864_d_n6;
        locals.var_t1_dn7 = assign15110_e21864_d_n7;
        locals.var_t1_dn10 = assign15110_e21864_d_n10;
        locals.var_t1_dn11 = assign15110_e21864_d_n11;
        locals.var_t1_dn12 = assign15110_e21864_d_n12;
        locals.var_t1_dn17 = assign15110_e21864_d_n17;

        let (assign15120_e21873, assign15120_e21873_d_n0, assign15120_e21873_d_n2, assign15120_e21873_d_n6, assign15120_e21873_d_n7, assign15120_e21873_d_n10, assign15120_e21873_d_n11, assign15120_e21873_d_n12, assign15120_e21873_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15120_e21870: f64 = (10.0 * 2.220446049250313e-16);
        let assign15120_e21871: f64 = (locals.var_q_deps0_soi_o_cnst0soi + assign15120_e21870);
        (assign15120_e21871, locals.var_q_deps0_soi_o_cnst0soi_dn0, locals.var_q_deps0_soi_o_cnst0soi_dn2, locals.var_q_deps0_soi_o_cnst0soi_dn6, locals.var_q_deps0_soi_o_cnst0soi_dn7, locals.var_q_deps0_soi_o_cnst0soi_dn10, locals.var_q_deps0_soi_o_cnst0soi_dn11, locals.var_q_deps0_soi_o_cnst0soi_dn12, locals.var_q_deps0_soi_o_cnst0soi_dn17,)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12, locals.var_xi0p12_dn17,)
    }
};
        locals.var_xi0p12 = assign15120_e21873;
        locals.var_xi0p12_dn0 = assign15120_e21873_d_n0;
        locals.var_xi0p12_dn2 = assign15120_e21873_d_n2;
        locals.var_xi0p12_dn6 = assign15120_e21873_d_n6;
        locals.var_xi0p12_dn7 = assign15120_e21873_d_n7;
        locals.var_xi0p12_dn10 = assign15120_e21873_d_n10;
        locals.var_xi0p12_dn11 = assign15120_e21873_d_n11;
        locals.var_xi0p12_dn12 = assign15120_e21873_d_n12;
        locals.var_xi0p12_dn17 = assign15120_e21873_d_n17;

        let (assign15130_e21880, assign15130_e21880_d_n0, assign15130_e21880_d_n2, assign15130_e21880_d_n6, assign15130_e21880_d_n7, assign15130_e21880_d_n10, assign15130_e21880_d_n11, assign15130_e21880_d_n12, assign15130_e21880_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15130_e21878: f64 = (locals.var_cnst0soi * locals.var_xi0p12);
        (assign15130_e21878, ((locals.var_cnst0soi_dn0 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn17)),)
    } else {
        (locals.var_qb0, locals.var_qb0_dn0, locals.var_qb0_dn2, locals.var_qb0_dn6, locals.var_qb0_dn7, locals.var_qb0_dn10, locals.var_qb0_dn11, locals.var_qb0_dn12, locals.var_qb0_dn17,)
    }
};
        locals.var_qb0 = assign15130_e21880;
        locals.var_qb0_dn0 = assign15130_e21880_d_n0;
        locals.var_qb0_dn2 = assign15130_e21880_d_n2;
        locals.var_qb0_dn6 = assign15130_e21880_d_n6;
        locals.var_qb0_dn7 = assign15130_e21880_d_n7;
        locals.var_qb0_dn10 = assign15130_e21880_d_n10;
        locals.var_qb0_dn11 = assign15130_e21880_d_n11;
        locals.var_qb0_dn12 = assign15130_e21880_d_n12;
        locals.var_qb0_dn17 = assign15130_e21880_d_n17;

        let (assign15140_e21889, assign15140_e21889_d_n0, assign15140_e21889_d_n2, assign15140_e21889_d_n6, assign15140_e21889_d_n7, assign15140_e21889_d_n10, assign15140_e21889_d_n11, assign15140_e21889_d_n12, assign15140_e21889_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15140_e21886: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign15140_e21887: f64 = (1.0 / assign15140_e21886);
        (assign15140_e21887, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn12 + locals.var_xi0p12_dn12) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn17 + locals.var_xi0p12_dn17) / (assign15140_e21886 * assign15140_e21886))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15140_e21889;
        locals.var_t1_dn0 = assign15140_e21889_d_n0;
        locals.var_t1_dn2 = assign15140_e21889_d_n2;
        locals.var_t1_dn6 = assign15140_e21889_d_n6;
        locals.var_t1_dn7 = assign15140_e21889_d_n7;
        locals.var_t1_dn10 = assign15140_e21889_d_n10;
        locals.var_t1_dn11 = assign15140_e21889_d_n11;
        locals.var_t1_dn12 = assign15140_e21889_d_n12;
        locals.var_t1_dn17 = assign15140_e21889_d_n17;

        let (assign15150_e21898, assign15150_e21898_d_n0, assign15150_e21898_d_n2, assign15150_e21898_d_n6, assign15150_e21898_d_n7, assign15150_e21898_d_n10, assign15150_e21898_d_n11, assign15150_e21898_d_n12, assign15150_e21898_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15150_e21894: f64 = (locals.var_cnst0soi * locals.var_fs01);
        let assign15150_e21896: f64 = (assign15150_e21894 * locals.var_t1);
        (assign15150_e21896, ((((locals.var_cnst0soi_dn0 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn0)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn2)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn6)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn7)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn10)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn11)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn12)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn17)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn17)),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign15150_e21898;
        locals.var_qn0_dn0 = assign15150_e21898_d_n0;
        locals.var_qn0_dn2 = assign15150_e21898_d_n2;
        locals.var_qn0_dn6 = assign15150_e21898_d_n6;
        locals.var_qn0_dn7 = assign15150_e21898_d_n7;
        locals.var_qn0_dn10 = assign15150_e21898_d_n10;
        locals.var_qn0_dn11 = assign15150_e21898_d_n11;
        locals.var_qn0_dn12 = assign15150_e21898_d_n12;
        locals.var_qn0_dn17 = assign15150_e21898_d_n17;

        let (assign15160_e21904, assign15160_e21904_d_n0, assign15160_e21904_d_n2, assign15160_e21904_d_n6, assign15160_e21904_d_n7, assign15160_e21904_d_n10, assign15160_e21904_d_n11, assign15160_e21904_d_n12, assign15160_e21904_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15160_e21902: f64 = (-locals.var_qn0);
        (assign15160_e21902, (-locals.var_qn0_dn0), (-locals.var_qn0_dn2), (-locals.var_qn0_dn6), (-locals.var_qn0_dn7), (-locals.var_qn0_dn10), (-locals.var_qn0_dn11), (-locals.var_qn0_dn12), (-locals.var_qn0_dn17),)
    } else {
        (locals.var_q_n0, locals.var_q_n0_dn0, locals.var_q_n0_dn2, locals.var_q_n0_dn6, locals.var_q_n0_dn7, locals.var_q_n0_dn10, locals.var_q_n0_dn11, locals.var_q_n0_dn12, locals.var_q_n0_dn17,)
    }
};
        locals.var_q_n0 = assign15160_e21904;
        locals.var_q_n0_dn0 = assign15160_e21904_d_n0;
        locals.var_q_n0_dn2 = assign15160_e21904_d_n2;
        locals.var_q_n0_dn6 = assign15160_e21904_d_n6;
        locals.var_q_n0_dn7 = assign15160_e21904_d_n7;
        locals.var_q_n0_dn10 = assign15160_e21904_d_n10;
        locals.var_q_n0_dn11 = assign15160_e21904_d_n11;
        locals.var_q_n0_dn12 = assign15160_e21904_d_n12;
        locals.var_q_n0_dn17 = assign15160_e21904_d_n17;

        let (assign15170_e21911, assign15170_e21911_d_n0, assign15170_e21911_d_n2, assign15170_e21911_d_n6, assign15170_e21911_d_n7, assign15170_e21911_d_n10, assign15170_e21911_d_n11, assign15170_e21911_d_n12, assign15170_e21911_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15170_e21909: f64 = (locals.var_qn0 * locals.var_c_fox_inv);
        (assign15170_e21909, ((locals.var_qn0_dn0 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn2)), ((locals.var_qn0_dn6 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn7)), ((locals.var_qn0_dn10 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn10)), ((locals.var_qn0_dn11 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn11)), ((locals.var_qn0_dn12 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn12)), ((locals.var_qn0_dn17 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign15170_e21911;
        locals.var_vgvt_dn0 = assign15170_e21911_d_n0;
        locals.var_vgvt_dn2 = assign15170_e21911_d_n2;
        locals.var_vgvt_dn6 = assign15170_e21911_d_n6;
        locals.var_vgvt_dn7 = assign15170_e21911_d_n7;
        locals.var_vgvt_dn10 = assign15170_e21911_d_n10;
        locals.var_vgvt_dn11 = assign15170_e21911_d_n11;
        locals.var_vgvt_dn12 = assign15170_e21911_d_n12;
        locals.var_vgvt_dn17 = assign15170_e21911_d_n17;

        let assign15180_e21914: f64 = (-1.0);
        let assign15180_e21919: f64 = if ((locals.var_flg_zone == assign15180_e21914) || (locals.var_vgvt <= 1e-12)) { 1.0 } else { 0.0 };
        locals.var_guard460 = assign15180_e21919;

        let (assign15190_e21926,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign15190_e21926;

        let (assign15200_e21933,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign15200_e21933;

        let (assign15210_e21942, assign15210_e21942_d_n0, assign15210_e21942_d_n2, assign15210_e21942_d_n6, assign15210_e21942_d_n7, assign15210_e21942_d_n10, assign15210_e21942_d_n11, assign15210_e21942_d_n12, assign15210_e21942_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15210_e21940: f64 = (locals.var_vgp - locals.var_ps0);
        (assign15210_e21940, (locals.var_vgp_dn0 - locals.var_ps0_dn0), (locals.var_vgp_dn2 - locals.var_ps0_dn2), (locals.var_vgp_dn6 - locals.var_ps0_dn6), (locals.var_vgp_dn7 - locals.var_ps0_dn7), (locals.var_vgp_dn10 - locals.var_ps0_dn10), (locals.var_vgp_dn11 - locals.var_ps0_dn11), (locals.var_vgp_dn12 - locals.var_ps0_dn12), (locals.var_vgp_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign15210_e21942;
        locals.var_t2_dn0 = assign15210_e21942_d_n0;
        locals.var_t2_dn2 = assign15210_e21942_d_n2;
        locals.var_t2_dn6 = assign15210_e21942_d_n6;
        locals.var_t2_dn7 = assign15210_e21942_d_n7;
        locals.var_t2_dn10 = assign15210_e21942_d_n10;
        locals.var_t2_dn11 = assign15210_e21942_d_n11;
        locals.var_t2_dn12 = assign15210_e21942_d_n12;
        locals.var_t2_dn17 = assign15210_e21942_d_n17;

        let (assign15220_e21951, assign15220_e21951_d_n0, assign15220_e21951_d_n2, assign15220_e21951_d_n6, assign15220_e21951_d_n7, assign15220_e21951_d_n10, assign15220_e21951_d_n11, assign15220_e21951_d_n12, assign15220_e21951_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15220_e21949: f64 = (locals.var_c_fox * locals.var_t2);
        (assign15220_e21949, ((locals.var_c_fox_dn0 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn0)), ((locals.var_c_fox_dn2 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn2)), ((locals.var_c_fox_dn6 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn6)), ((locals.var_c_fox_dn7 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn7)), ((locals.var_c_fox_dn10 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn10)), ((locals.var_c_fox_dn11 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn11)), ((locals.var_c_fox_dn12 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn12)), ((locals.var_c_fox_dn17 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign15220_e21951;
        locals.var_qbu_dn0 = assign15220_e21951_d_n0;
        locals.var_qbu_dn2 = assign15220_e21951_d_n2;
        locals.var_qbu_dn6 = assign15220_e21951_d_n6;
        locals.var_qbu_dn7 = assign15220_e21951_d_n7;
        locals.var_qbu_dn10 = assign15220_e21951_d_n10;
        locals.var_qbu_dn11 = assign15220_e21951_d_n11;
        locals.var_qbu_dn12 = assign15220_e21951_d_n12;
        locals.var_qbu_dn17 = assign15220_e21951_d_n17;

        let (assign15230_e21961, assign15230_e21961_d_n0, assign15230_e21961_d_n2, assign15230_e21961_d_n6, assign15230_e21961_d_n7, assign15230_e21961_d_n10, assign15230_e21961_d_n11, assign15230_e21961_d_n12, assign15230_e21961_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15230_e21957: f64 = (-locals.var_weffcv_nf);
        let assign15230_e21959: f64 = (assign15230_e21957 * locals.var_leff_cv);
        (assign15230_e21959, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign15230_e21961;
        locals.var_t0_dn0 = assign15230_e21961_d_n0;
        locals.var_t0_dn2 = assign15230_e21961_d_n2;
        locals.var_t0_dn6 = assign15230_e21961_d_n6;
        locals.var_t0_dn7 = assign15230_e21961_d_n7;
        locals.var_t0_dn10 = assign15230_e21961_d_n10;
        locals.var_t0_dn11 = assign15230_e21961_d_n11;
        locals.var_t0_dn12 = assign15230_e21961_d_n12;
        locals.var_t0_dn17 = assign15230_e21961_d_n17;

        let (assign15240_e21970, assign15240_e21970_d_n0, assign15240_e21970_d_n2, assign15240_e21970_d_n6, assign15240_e21970_d_n7, assign15240_e21970_d_n10, assign15240_e21970_d_n11, assign15240_e21970_d_n12, assign15240_e21970_d_n13, assign15240_e21970_d_n15, assign15240_e21970_d_n16, assign15240_e21970_d_n17, assign15240_e21970_d_n18,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15240_e21968: f64 = (locals.var_t0 * locals.var_qbu);
        (assign15240_e21968, ((locals.var_t0_dn0 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn0)), ((locals.var_t0_dn2 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn2)), ((locals.var_t0_dn6 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn6)), ((locals.var_t0_dn7 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn7)), ((locals.var_t0_dn10 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn10)), ((locals.var_t0_dn11 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn11)), ((locals.var_t0_dn12 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t0_dn17 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign15240_e21970;
        locals.var_qb_dn0 = assign15240_e21970_d_n0;
        locals.var_qb_dn2 = assign15240_e21970_d_n2;
        locals.var_qb_dn6 = assign15240_e21970_d_n6;
        locals.var_qb_dn7 = assign15240_e21970_d_n7;
        locals.var_qb_dn10 = assign15240_e21970_d_n10;
        locals.var_qb_dn11 = assign15240_e21970_d_n11;
        locals.var_qb_dn12 = assign15240_e21970_d_n12;
        locals.var_qb_dn13 = assign15240_e21970_d_n13;
        locals.var_qb_dn15 = assign15240_e21970_d_n15;
        locals.var_qb_dn16 = assign15240_e21970_d_n16;
        locals.var_qb_dn17 = assign15240_e21970_d_n17;
        locals.var_qb_dn18 = assign15240_e21970_d_n18;

        let (assign15250_e21977, assign15250_e21977_d_n0, assign15250_e21977_d_n2, assign15250_e21977_d_n6, assign15250_e21977_d_n7, assign15250_e21977_d_n10, assign15250_e21977_d_n11, assign15250_e21977_d_n12, assign15250_e21977_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign15250_e21977;
        locals.var_qi_dn0 = assign15250_e21977_d_n0;
        locals.var_qi_dn2 = assign15250_e21977_d_n2;
        locals.var_qi_dn6 = assign15250_e21977_d_n6;
        locals.var_qi_dn7 = assign15250_e21977_d_n7;
        locals.var_qi_dn10 = assign15250_e21977_d_n10;
        locals.var_qi_dn11 = assign15250_e21977_d_n11;
        locals.var_qi_dn12 = assign15250_e21977_d_n12;
        locals.var_qi_dn17 = assign15250_e21977_d_n17;

        let (assign15260_e21984, assign15260_e21984_d_n0, assign15260_e21984_d_n2, assign15260_e21984_d_n6, assign15260_e21984_d_n7, assign15260_e21984_d_n10, assign15260_e21984_d_n11, assign15260_e21984_d_n12, assign15260_e21984_d_n13, assign15260_e21984_d_n15, assign15260_e21984_d_n16, assign15260_e21984_d_n17, assign15260_e21984_d_n18,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign15260_e21984;
        locals.var_qd_dn0 = assign15260_e21984_d_n0;
        locals.var_qd_dn2 = assign15260_e21984_d_n2;
        locals.var_qd_dn6 = assign15260_e21984_d_n6;
        locals.var_qd_dn7 = assign15260_e21984_d_n7;
        locals.var_qd_dn10 = assign15260_e21984_d_n10;
        locals.var_qd_dn11 = assign15260_e21984_d_n11;
        locals.var_qd_dn12 = assign15260_e21984_d_n12;
        locals.var_qd_dn13 = assign15260_e21984_d_n13;
        locals.var_qd_dn15 = assign15260_e21984_d_n15;
        locals.var_qd_dn16 = assign15260_e21984_d_n16;
        locals.var_qd_dn17 = assign15260_e21984_d_n17;
        locals.var_qd_dn18 = assign15260_e21984_d_n18;

        let (assign15270_e21994, assign15270_e21994_d_n0, assign15270_e21994_d_n2, assign15270_e21994_d_n6, assign15270_e21994_d_n7, assign15270_e21994_d_n10, assign15270_e21994_d_n11, assign15270_e21994_d_n12, assign15270_e21994_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15270_e21990: f64 = (-locals.var_area_bt_n);
        let assign15270_e21992: f64 = (assign15270_e21990 * locals.var_qbu);
        (assign15270_e21992, (assign15270_e21990 * locals.var_qbu_dn0), (assign15270_e21990 * locals.var_qbu_dn2), (assign15270_e21990 * locals.var_qbu_dn6), (assign15270_e21990 * locals.var_qbu_dn7), (assign15270_e21990 * locals.var_qbu_dn10), (assign15270_e21990 * locals.var_qbu_dn11), (assign15270_e21990 * locals.var_qbu_dn12), (assign15270_e21990 * locals.var_qbu_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign15270_e21994;
        locals.var_t2_dn0 = assign15270_e21994_d_n0;
        locals.var_t2_dn2 = assign15270_e21994_d_n2;
        locals.var_t2_dn6 = assign15270_e21994_d_n6;
        locals.var_t2_dn7 = assign15270_e21994_d_n7;
        locals.var_t2_dn10 = assign15270_e21994_d_n10;
        locals.var_t2_dn11 = assign15270_e21994_d_n11;
        locals.var_t2_dn12 = assign15270_e21994_d_n12;
        locals.var_t2_dn17 = assign15270_e21994_d_n17;

        let (assign15280_e22003, assign15280_e22003_d_n0, assign15280_e22003_d_n2, assign15280_e22003_d_n6, assign15280_e22003_d_n7, assign15280_e22003_d_n10, assign15280_e22003_d_n11, assign15280_e22003_d_n12, assign15280_e22003_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15280_e22001: f64 = (locals.var_t2 * locals.var_qdrat);
        (assign15280_e22001, ((locals.var_t2_dn0 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn0)), ((locals.var_t2_dn2 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn2)), ((locals.var_t2_dn6 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn6)), ((locals.var_t2_dn7 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn7)), ((locals.var_t2_dn10 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn10)), ((locals.var_t2_dn11 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn11)), ((locals.var_t2_dn12 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn12)), ((locals.var_t2_dn17 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign15280_e22003;
        locals.var_qbody_bt_n_sud_dn0 = assign15280_e22003_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign15280_e22003_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign15280_e22003_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign15280_e22003_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign15280_e22003_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign15280_e22003_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign15280_e22003_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign15280_e22003_d_n17;

        let (assign15290_e22012, assign15290_e22012_d_n0, assign15290_e22012_d_n2, assign15290_e22012_d_n6, assign15290_e22012_d_n7, assign15290_e22012_d_n10, assign15290_e22012_d_n11, assign15290_e22012_d_n12, assign15290_e22012_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15290_e22010: f64 = (locals.var_t2 - locals.var_qbody_bt_n_sud);
        (assign15290_e22010, (locals.var_t2_dn0 - locals.var_qbody_bt_n_sud_dn0), (locals.var_t2_dn2 - locals.var_qbody_bt_n_sud_dn2), (locals.var_t2_dn6 - locals.var_qbody_bt_n_sud_dn6), (locals.var_t2_dn7 - locals.var_qbody_bt_n_sud_dn7), (locals.var_t2_dn10 - locals.var_qbody_bt_n_sud_dn10), (locals.var_t2_dn11 - locals.var_qbody_bt_n_sud_dn11), (locals.var_t2_dn12 - locals.var_qbody_bt_n_sud_dn12), (locals.var_t2_dn17 - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign15290_e22012;
        locals.var_qbody_bt_n_sus_dn0 = assign15290_e22012_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign15290_e22012_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign15290_e22012_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign15290_e22012_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign15290_e22012_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign15290_e22012_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign15290_e22012_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign15290_e22012_d_n17;

        let (assign15300_e22019, assign15300_e22019_d_n0, assign15300_e22019_d_n2, assign15300_e22019_d_n6, assign15300_e22019_d_n7, assign15300_e22019_d_n10, assign15300_e22019_d_n11, assign15300_e22019_d_n12, assign15300_e22019_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign15300_e22019;
        locals.var_qbody_bt_n_iud_dn0 = assign15300_e22019_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign15300_e22019_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign15300_e22019_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign15300_e22019_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign15300_e22019_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign15300_e22019_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign15300_e22019_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign15300_e22019_d_n17;

        let (assign15310_e22026, assign15310_e22026_d_n0, assign15310_e22026_d_n2, assign15310_e22026_d_n6, assign15310_e22026_d_n7, assign15310_e22026_d_n10, assign15310_e22026_d_n11, assign15310_e22026_d_n12, assign15310_e22026_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign15310_e22026;
        locals.var_qbody_bt_n_ius_dn0 = assign15310_e22026_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign15310_e22026_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign15310_e22026_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign15310_e22026_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign15310_e22026_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign15310_e22026_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign15310_e22026_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign15310_e22026_d_n17;

        let (assign15320_e22033, assign15320_e22033_d_n0, assign15320_e22033_d_n2, assign15320_e22033_d_n6, assign15320_e22033_d_n7, assign15320_e22033_d_n10, assign15320_e22033_d_n11, assign15320_e22033_d_n12, assign15320_e22033_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign15320_e22033;
        locals.var_ids_dn0 = assign15320_e22033_d_n0;
        locals.var_ids_dn2 = assign15320_e22033_d_n2;
        locals.var_ids_dn6 = assign15320_e22033_d_n6;
        locals.var_ids_dn7 = assign15320_e22033_d_n7;
        locals.var_ids_dn10 = assign15320_e22033_d_n10;
        locals.var_ids_dn11 = assign15320_e22033_d_n11;
        locals.var_ids_dn12 = assign15320_e22033_d_n12;
        locals.var_ids_dn17 = assign15320_e22033_d_n17;

        let (assign15330_e22040, assign15330_e22040_d_n0, assign15330_e22040_d_n2, assign15330_e22040_d_n6, assign15330_e22040_d_n7, assign15330_e22040_d_n10, assign15330_e22040_d_n11, assign15330_e22040_d_n12, assign15330_e22040_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign15330_e22040;
        locals.var_vgvt_dn0 = assign15330_e22040_d_n0;
        locals.var_vgvt_dn2 = assign15330_e22040_d_n2;
        locals.var_vgvt_dn6 = assign15330_e22040_d_n6;
        locals.var_vgvt_dn7 = assign15330_e22040_d_n7;
        locals.var_vgvt_dn10 = assign15330_e22040_d_n10;
        locals.var_vgvt_dn11 = assign15330_e22040_d_n11;
        locals.var_vgvt_dn12 = assign15330_e22040_d_n12;
        locals.var_vgvt_dn17 = assign15330_e22040_d_n17;

        let (assign15340_e22047,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign15340_e22047;

    }

    pub(super) fn stamp_transient_block_51(
        locals: &mut StampLocals,
    ) {
        let (assign15350_e22054, assign15350_e22054_d_n0, assign15350_e22054_d_n2, assign15350_e22054_d_n6, assign15350_e22054_d_n7, assign15350_e22054_d_n10, assign15350_e22054_d_n11, assign15350_e22054_d_n12, assign15350_e22054_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign15350_e22054;
        locals.var_phi_sl_soi_dn0 = assign15350_e22054_d_n0;
        locals.var_phi_sl_soi_dn2 = assign15350_e22054_d_n2;
        locals.var_phi_sl_soi_dn6 = assign15350_e22054_d_n6;
        locals.var_phi_sl_soi_dn7 = assign15350_e22054_d_n7;
        locals.var_phi_sl_soi_dn10 = assign15350_e22054_d_n10;
        locals.var_phi_sl_soi_dn11 = assign15350_e22054_d_n11;
        locals.var_phi_sl_soi_dn12 = assign15350_e22054_d_n12;
        locals.var_phi_sl_soi_dn17 = assign15350_e22054_d_n17;

        let (assign15360_e22061, assign15360_e22061_d_n0, assign15360_e22061_d_n2, assign15360_e22061_d_n6, assign15360_e22061_d_n7, assign15360_e22061_d_n10, assign15360_e22061_d_n11, assign15360_e22061_d_n12, assign15360_e22061_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign15360_e22061;
        locals.var_psl_dn0 = assign15360_e22061_d_n0;
        locals.var_psl_dn2 = assign15360_e22061_d_n2;
        locals.var_psl_dn6 = assign15360_e22061_d_n6;
        locals.var_psl_dn7 = assign15360_e22061_d_n7;
        locals.var_psl_dn10 = assign15360_e22061_d_n10;
        locals.var_psl_dn11 = assign15360_e22061_d_n11;
        locals.var_psl_dn12 = assign15360_e22061_d_n12;
        locals.var_psl_dn17 = assign15360_e22061_d_n17;

        let (assign15370_e22068, assign15370_e22068_d_n0, assign15370_e22068_d_n2, assign15370_e22068_d_n6, assign15370_e22068_d_n7, assign15370_e22068_d_n10, assign15370_e22068_d_n11, assign15370_e22068_d_n12, assign15370_e22068_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign15370_e22068;
        locals.var_psdl_dn0 = assign15370_e22068_d_n0;
        locals.var_psdl_dn2 = assign15370_e22068_d_n2;
        locals.var_psdl_dn6 = assign15370_e22068_d_n6;
        locals.var_psdl_dn7 = assign15370_e22068_d_n7;
        locals.var_psdl_dn10 = assign15370_e22068_d_n10;
        locals.var_psdl_dn11 = assign15370_e22068_d_n11;
        locals.var_psdl_dn12 = assign15370_e22068_d_n12;
        locals.var_psdl_dn17 = assign15370_e22068_d_n17;

        let (assign15380_e22075,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign15380_e22075;

        let assign15390_e22078: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign15390_e22078;

        let (assign15400_e22085, assign15400_e22085_d_n0, assign15400_e22085_d_n2, assign15400_e22085_d_n6, assign15400_e22085_d_n7, assign15400_e22085_d_n10, assign15400_e22085_d_n11, assign15400_e22085_d_n12, assign15400_e22085_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    }
};
        locals.var_vdsorg = assign15400_e22085;
        locals.var_vdsorg_dn0 = assign15400_e22085_d_n0;
        locals.var_vdsorg_dn2 = assign15400_e22085_d_n2;
        locals.var_vdsorg_dn6 = assign15400_e22085_d_n6;
        locals.var_vdsorg_dn7 = assign15400_e22085_d_n7;
        locals.var_vdsorg_dn10 = assign15400_e22085_d_n10;
        locals.var_vdsorg_dn11 = assign15400_e22085_d_n11;
        locals.var_vdsorg_dn12 = assign15400_e22085_d_n12;
        locals.var_vdsorg_dn17 = assign15400_e22085_d_n17;

        let (assign15410_e22092, assign15410_e22092_d_n0, assign15410_e22092_d_n2, assign15410_e22092_d_n6, assign15410_e22092_d_n7, assign15410_e22092_d_n10, assign15410_e22092_d_n11, assign15410_e22092_d_n12, assign15410_e22092_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10__blk468, locals.var_t10__blk468_dn0, locals.var_t10__blk468_dn2, locals.var_t10__blk468_dn6, locals.var_t10__blk468_dn7, locals.var_t10__blk468_dn10, locals.var_t10__blk468_dn11, locals.var_t10__blk468_dn12, locals.var_t10__blk468_dn17,)
    }
};
        locals.var_t10__blk468 = assign15410_e22092;
        locals.var_t10__blk468_dn0 = assign15410_e22092_d_n0;
        locals.var_t10__blk468_dn2 = assign15410_e22092_d_n2;
        locals.var_t10__blk468_dn6 = assign15410_e22092_d_n6;
        locals.var_t10__blk468_dn7 = assign15410_e22092_d_n7;
        locals.var_t10__blk468_dn10 = assign15410_e22092_d_n10;
        locals.var_t10__blk468_dn11 = assign15410_e22092_d_n11;
        locals.var_t10__blk468_dn12 = assign15410_e22092_d_n12;
        locals.var_t10__blk468_dn17 = assign15410_e22092_d_n17;

        let (assign15420_e22103, assign15420_e22103_d_n0, assign15420_e22103_d_n2, assign15420_e22103_d_n6, assign15420_e22103_d_n7, assign15420_e22103_d_n10, assign15420_e22103_d_n11, assign15420_e22103_d_n12, assign15420_e22103_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15420_e22100: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign15420_e22101: f64 = (locals.var_qnsub_esi / assign15420_e22100);
        (assign15420_e22101, (((locals.var_qnsub_esi_dn0 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn2 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn6 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn7 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn10 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn11 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn12 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn17 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign15420_e22100 * assign15420_e22100)),)
    } else {
        (locals.var_t2__blk463, locals.var_t2__blk463_dn0, locals.var_t2__blk463_dn2, locals.var_t2__blk463_dn6, locals.var_t2__blk463_dn7, locals.var_t2__blk463_dn10, locals.var_t2__blk463_dn11, locals.var_t2__blk463_dn12, locals.var_t2__blk463_dn17,)
    }
};
        locals.var_t2__blk463 = assign15420_e22103;
        locals.var_t2__blk463_dn0 = assign15420_e22103_d_n0;
        locals.var_t2__blk463_dn2 = assign15420_e22103_d_n2;
        locals.var_t2__blk463_dn6 = assign15420_e22103_d_n6;
        locals.var_t2__blk463_dn7 = assign15420_e22103_d_n7;
        locals.var_t2__blk463_dn10 = assign15420_e22103_d_n10;
        locals.var_t2__blk463_dn11 = assign15420_e22103_d_n11;
        locals.var_t2__blk463_dn12 = assign15420_e22103_d_n12;
        locals.var_t2__blk463_dn17 = assign15420_e22103_d_n17;

        let (assign15430_e22118, assign15430_e22118_d_n0, assign15430_e22118_d_n2, assign15430_e22118_d_n6, assign15430_e22118_d_n7, assign15430_e22118_d_n10, assign15430_e22118_d_n11, assign15430_e22118_d_n12, assign15430_e22118_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15430_e22111: f64 = (2.0 / locals.var_t2__blk463);
        let assign15430_e22114: f64 = (locals.var_vgp - locals.var_t10__blk468);
        let assign15430_e22115: f64 = (assign15430_e22111 * assign15430_e22114);
        let assign15430_e22116: f64 = (1.0 + assign15430_e22115);
        (assign15430_e22116, (((-((2.0 * locals.var_t2__blk463_dn0) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn0 - locals.var_t10__blk468_dn0))), (((-((2.0 * locals.var_t2__blk463_dn2) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn2 - locals.var_t10__blk468_dn2))), (((-((2.0 * locals.var_t2__blk463_dn6) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn6 - locals.var_t10__blk468_dn6))), (((-((2.0 * locals.var_t2__blk463_dn7) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn7 - locals.var_t10__blk468_dn7))), (((-((2.0 * locals.var_t2__blk463_dn10) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn10 - locals.var_t10__blk468_dn10))), (((-((2.0 * locals.var_t2__blk463_dn11) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn11 - locals.var_t10__blk468_dn11))), (((-((2.0 * locals.var_t2__blk463_dn12) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn12 - locals.var_t10__blk468_dn12))), (((-((2.0 * locals.var_t2__blk463_dn17) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn17 - locals.var_t10__blk468_dn17))),)
    } else {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    }
};
        locals.var_t4__blk465 = assign15430_e22118;
        locals.var_t4__blk465_dn0 = assign15430_e22118_d_n0;
        locals.var_t4__blk465_dn2 = assign15430_e22118_d_n2;
        locals.var_t4__blk465_dn6 = assign15430_e22118_d_n6;
        locals.var_t4__blk465_dn7 = assign15430_e22118_d_n7;
        locals.var_t4__blk465_dn10 = assign15430_e22118_d_n10;
        locals.var_t4__blk465_dn11 = assign15430_e22118_d_n11;
        locals.var_t4__blk465_dn12 = assign15430_e22118_d_n12;
        locals.var_t4__blk465_dn17 = assign15430_e22118_d_n17;

        let (assign15440_e22129, assign15440_e22129_d_n0, assign15440_e22129_d_n2, assign15440_e22129_d_n6, assign15440_e22129_d_n7, assign15440_e22129_d_n10, assign15440_e22129_d_n11, assign15440_e22129_d_n12, assign15440_e22129_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15440_e22126: f64 = (2.0 / locals.var_t2__blk463);
        let assign15440_e22127: f64 = (1.0 + assign15440_e22126);
        (assign15440_e22127, (-((2.0 * locals.var_t2__blk463_dn0) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn2) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn6) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn7) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn10) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn11) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn12) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn17) / (locals.var_t2__blk463 * locals.var_t2__blk463))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign15440_e22129;
        locals.var_t5_dn0 = assign15440_e22129_d_n0;
        locals.var_t5_dn2 = assign15440_e22129_d_n2;
        locals.var_t5_dn6 = assign15440_e22129_d_n6;
        locals.var_t5_dn7 = assign15440_e22129_d_n7;
        locals.var_t5_dn10 = assign15440_e22129_d_n10;
        locals.var_t5_dn11 = assign15440_e22129_d_n11;
        locals.var_t5_dn12 = assign15440_e22129_d_n12;
        locals.var_t5_dn17 = assign15440_e22129_d_n17;

        let assign15450_e22133: f64 = locals.var_t5;
        let assign15450_e22138: f64 = if ((locals.var_t4__blk465 < assign15450_e22133) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard469 = assign15450_e22138;

        let (assign15460_e22151, assign15460_e22151_d_n0, assign15460_e22151_d_n2, assign15460_e22151_d_n6, assign15460_e22151_d_n7, assign15460_e22151_d_n10, assign15460_e22151_d_n11, assign15460_e22151_d_n12, assign15460_e22151_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15460_e22147: f64 = locals.var_t5;
        let assign15460_e22149: f64 = (assign15460_e22147 - locals.var_t4__blk465);
        (assign15460_e22149, (locals.var_t5_dn0 - locals.var_t4__blk465_dn0), (locals.var_t5_dn2 - locals.var_t4__blk465_dn2), (locals.var_t5_dn6 - locals.var_t4__blk465_dn6), (locals.var_t5_dn7 - locals.var_t4__blk465_dn7), (locals.var_t5_dn10 - locals.var_t4__blk465_dn10), (locals.var_t5_dn11 - locals.var_t4__blk465_dn11), (locals.var_t5_dn12 - locals.var_t4__blk465_dn12), (locals.var_t5_dn17 - locals.var_t4__blk465_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign15460_e22151;
        locals.var_tmf1_dn0 = assign15460_e22151_d_n0;
        locals.var_tmf1_dn2 = assign15460_e22151_d_n2;
        locals.var_tmf1_dn6 = assign15460_e22151_d_n6;
        locals.var_tmf1_dn7 = assign15460_e22151_d_n7;
        locals.var_tmf1_dn10 = assign15460_e22151_d_n10;
        locals.var_tmf1_dn11 = assign15460_e22151_d_n11;
        locals.var_tmf1_dn12 = assign15460_e22151_d_n12;
        locals.var_tmf1_dn17 = assign15460_e22151_d_n17;

        let (assign15470_e22162, assign15470_e22162_d_n0, assign15470_e22162_d_n2, assign15470_e22162_d_n6, assign15470_e22162_d_n7, assign15470_e22162_d_n10, assign15470_e22162_d_n11, assign15470_e22162_d_n12, assign15470_e22162_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15470_e22160: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign15470_e22160, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign15470_e22162;
        locals.var_x2_dn0 = assign15470_e22162_d_n0;
        locals.var_x2_dn2 = assign15470_e22162_d_n2;
        locals.var_x2_dn6 = assign15470_e22162_d_n6;
        locals.var_x2_dn7 = assign15470_e22162_d_n7;
        locals.var_x2_dn10 = assign15470_e22162_d_n10;
        locals.var_x2_dn11 = assign15470_e22162_d_n11;
        locals.var_x2_dn12 = assign15470_e22162_d_n12;
        locals.var_x2_dn17 = assign15470_e22162_d_n17;

        let (assign15480_e22173, assign15480_e22173_d_n0, assign15480_e22173_d_n2, assign15480_e22173_d_n6, assign15480_e22173_d_n7, assign15480_e22173_d_n10, assign15480_e22173_d_n11, assign15480_e22173_d_n12, assign15480_e22173_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15480_e22171: f64 = (locals.var_t5 * locals.var_t5);
        (assign15480_e22171, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn12 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn12)), ((locals.var_t5_dn17 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign15480_e22173;
        locals.var_xmax2_dn0 = assign15480_e22173_d_n0;
        locals.var_xmax2_dn2 = assign15480_e22173_d_n2;
        locals.var_xmax2_dn6 = assign15480_e22173_d_n6;
        locals.var_xmax2_dn7 = assign15480_e22173_d_n7;
        locals.var_xmax2_dn10 = assign15480_e22173_d_n10;
        locals.var_xmax2_dn11 = assign15480_e22173_d_n11;
        locals.var_xmax2_dn12 = assign15480_e22173_d_n12;
        locals.var_xmax2_dn17 = assign15480_e22173_d_n17;

        let (assign15490_e22182, assign15490_e22182_d_n0, assign15490_e22182_d_n2, assign15490_e22182_d_n6, assign15490_e22182_d_n7, assign15490_e22182_d_n10, assign15490_e22182_d_n11, assign15490_e22182_d_n12, assign15490_e22182_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15490_e22182;
        locals.var_xp_dn0 = assign15490_e22182_d_n0;
        locals.var_xp_dn2 = assign15490_e22182_d_n2;
        locals.var_xp_dn6 = assign15490_e22182_d_n6;
        locals.var_xp_dn7 = assign15490_e22182_d_n7;
        locals.var_xp_dn10 = assign15490_e22182_d_n10;
        locals.var_xp_dn11 = assign15490_e22182_d_n11;
        locals.var_xp_dn12 = assign15490_e22182_d_n12;
        locals.var_xp_dn17 = assign15490_e22182_d_n17;

        let (assign15500_e22191, assign15500_e22191_d_n0, assign15500_e22191_d_n2, assign15500_e22191_d_n6, assign15500_e22191_d_n7, assign15500_e22191_d_n10, assign15500_e22191_d_n11, assign15500_e22191_d_n12, assign15500_e22191_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15500_e22191;
        locals.var_xmp_dn0 = assign15500_e22191_d_n0;
        locals.var_xmp_dn2 = assign15500_e22191_d_n2;
        locals.var_xmp_dn6 = assign15500_e22191_d_n6;
        locals.var_xmp_dn7 = assign15500_e22191_d_n7;
        locals.var_xmp_dn10 = assign15500_e22191_d_n10;
        locals.var_xmp_dn11 = assign15500_e22191_d_n11;
        locals.var_xmp_dn12 = assign15500_e22191_d_n12;
        locals.var_xmp_dn17 = assign15500_e22191_d_n17;

        let (assign15510_e22200,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15510_e22200;

        let (assign15520_e22209,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15520_e22209;

        let (assign15530_e22218, assign15530_e22218_d_n0, assign15530_e22218_d_n2, assign15530_e22218_d_n6, assign15530_e22218_d_n7, assign15530_e22218_d_n10, assign15530_e22218_d_n11, assign15530_e22218_d_n12, assign15530_e22218_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign15530_e22218;
        locals.var_arg_dn0 = assign15530_e22218_d_n0;
        locals.var_arg_dn2 = assign15530_e22218_d_n2;
        locals.var_arg_dn6 = assign15530_e22218_d_n6;
        locals.var_arg_dn7 = assign15530_e22218_d_n7;
        locals.var_arg_dn10 = assign15530_e22218_d_n10;
        locals.var_arg_dn11 = assign15530_e22218_d_n11;
        locals.var_arg_dn12 = assign15530_e22218_d_n12;
        locals.var_arg_dn17 = assign15530_e22218_d_n17;

        let (assign15540_e22227, assign15540_e22227_d_n0, assign15540_e22227_d_n2, assign15540_e22227_d_n6, assign15540_e22227_d_n7, assign15540_e22227_d_n10, assign15540_e22227_d_n11, assign15540_e22227_d_n12, assign15540_e22227_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15540_e22227;
        locals.var_dnm_dn0 = assign15540_e22227_d_n0;
        locals.var_dnm_dn2 = assign15540_e22227_d_n2;
        locals.var_dnm_dn6 = assign15540_e22227_d_n6;
        locals.var_dnm_dn7 = assign15540_e22227_d_n7;
        locals.var_dnm_dn10 = assign15540_e22227_d_n10;
        locals.var_dnm_dn11 = assign15540_e22227_d_n11;
        locals.var_dnm_dn12 = assign15540_e22227_d_n12;
        locals.var_dnm_dn17 = assign15540_e22227_d_n17;

        let (assign15550_e22238, assign15550_e22238_d_n0, assign15550_e22238_d_n2, assign15550_e22238_d_n6, assign15550_e22238_d_n7, assign15550_e22238_d_n10, assign15550_e22238_d_n11, assign15550_e22238_d_n12, assign15550_e22238_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15550_e22236: f64 = (locals.var_xp * locals.var_x2);
        (assign15550_e22236, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15550_e22238;
        locals.var_xp_dn0 = assign15550_e22238_d_n0;
        locals.var_xp_dn2 = assign15550_e22238_d_n2;
        locals.var_xp_dn6 = assign15550_e22238_d_n6;
        locals.var_xp_dn7 = assign15550_e22238_d_n7;
        locals.var_xp_dn10 = assign15550_e22238_d_n10;
        locals.var_xp_dn11 = assign15550_e22238_d_n11;
        locals.var_xp_dn12 = assign15550_e22238_d_n12;
        locals.var_xp_dn17 = assign15550_e22238_d_n17;

        let (assign15560_e22249, assign15560_e22249_d_n0, assign15560_e22249_d_n2, assign15560_e22249_d_n6, assign15560_e22249_d_n7, assign15560_e22249_d_n10, assign15560_e22249_d_n11, assign15560_e22249_d_n12, assign15560_e22249_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15560_e22247: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15560_e22247, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15560_e22249;
        locals.var_xmp_dn0 = assign15560_e22249_d_n0;
        locals.var_xmp_dn2 = assign15560_e22249_d_n2;
        locals.var_xmp_dn6 = assign15560_e22249_d_n6;
        locals.var_xmp_dn7 = assign15560_e22249_d_n7;
        locals.var_xmp_dn10 = assign15560_e22249_d_n10;
        locals.var_xmp_dn11 = assign15560_e22249_d_n11;
        locals.var_xmp_dn12 = assign15560_e22249_d_n12;
        locals.var_xmp_dn17 = assign15560_e22249_d_n17;

        let (assign15570_e22260, assign15570_e22260_d_n0, assign15570_e22260_d_n2, assign15570_e22260_d_n6, assign15570_e22260_d_n7, assign15570_e22260_d_n10, assign15570_e22260_d_n11, assign15570_e22260_d_n12, assign15570_e22260_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15570_e22258: f64 = (locals.var_xp * locals.var_x2);
        (assign15570_e22258, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15570_e22260;
        locals.var_xp_dn0 = assign15570_e22260_d_n0;
        locals.var_xp_dn2 = assign15570_e22260_d_n2;
        locals.var_xp_dn6 = assign15570_e22260_d_n6;
        locals.var_xp_dn7 = assign15570_e22260_d_n7;
        locals.var_xp_dn10 = assign15570_e22260_d_n10;
        locals.var_xp_dn11 = assign15570_e22260_d_n11;
        locals.var_xp_dn12 = assign15570_e22260_d_n12;
        locals.var_xp_dn17 = assign15570_e22260_d_n17;

        let (assign15580_e22271, assign15580_e22271_d_n0, assign15580_e22271_d_n2, assign15580_e22271_d_n6, assign15580_e22271_d_n7, assign15580_e22271_d_n10, assign15580_e22271_d_n11, assign15580_e22271_d_n12, assign15580_e22271_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15580_e22269: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15580_e22269, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15580_e22271;
        locals.var_xmp_dn0 = assign15580_e22271_d_n0;
        locals.var_xmp_dn2 = assign15580_e22271_d_n2;
        locals.var_xmp_dn6 = assign15580_e22271_d_n6;
        locals.var_xmp_dn7 = assign15580_e22271_d_n7;
        locals.var_xmp_dn10 = assign15580_e22271_d_n10;
        locals.var_xmp_dn11 = assign15580_e22271_d_n11;
        locals.var_xmp_dn12 = assign15580_e22271_d_n12;
        locals.var_xmp_dn17 = assign15580_e22271_d_n17;

        let (assign15590_e22282, assign15590_e22282_d_n0, assign15590_e22282_d_n2, assign15590_e22282_d_n6, assign15590_e22282_d_n7, assign15590_e22282_d_n10, assign15590_e22282_d_n11, assign15590_e22282_d_n12, assign15590_e22282_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15590_e22280: f64 = (locals.var_xp * locals.var_x2);
        (assign15590_e22280, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15590_e22282;
        locals.var_xp_dn0 = assign15590_e22282_d_n0;
        locals.var_xp_dn2 = assign15590_e22282_d_n2;
        locals.var_xp_dn6 = assign15590_e22282_d_n6;
        locals.var_xp_dn7 = assign15590_e22282_d_n7;
        locals.var_xp_dn10 = assign15590_e22282_d_n10;
        locals.var_xp_dn11 = assign15590_e22282_d_n11;
        locals.var_xp_dn12 = assign15590_e22282_d_n12;
        locals.var_xp_dn17 = assign15590_e22282_d_n17;

        let (assign15600_e22293, assign15600_e22293_d_n0, assign15600_e22293_d_n2, assign15600_e22293_d_n6, assign15600_e22293_d_n7, assign15600_e22293_d_n10, assign15600_e22293_d_n11, assign15600_e22293_d_n12, assign15600_e22293_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15600_e22291: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15600_e22291, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15600_e22293;
        locals.var_xmp_dn0 = assign15600_e22293_d_n0;
        locals.var_xmp_dn2 = assign15600_e22293_d_n2;
        locals.var_xmp_dn6 = assign15600_e22293_d_n6;
        locals.var_xmp_dn7 = assign15600_e22293_d_n7;
        locals.var_xmp_dn10 = assign15600_e22293_d_n10;
        locals.var_xmp_dn11 = assign15600_e22293_d_n11;
        locals.var_xmp_dn12 = assign15600_e22293_d_n12;
        locals.var_xmp_dn17 = assign15600_e22293_d_n17;

        let (assign15610_e22304, assign15610_e22304_d_n0, assign15610_e22304_d_n2, assign15610_e22304_d_n6, assign15610_e22304_d_n7, assign15610_e22304_d_n10, assign15610_e22304_d_n11, assign15610_e22304_d_n12, assign15610_e22304_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15610_e22302: f64 = (locals.var_xp * locals.var_x2);
        (assign15610_e22302, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15610_e22304;
        locals.var_xp_dn0 = assign15610_e22304_d_n0;
        locals.var_xp_dn2 = assign15610_e22304_d_n2;
        locals.var_xp_dn6 = assign15610_e22304_d_n6;
        locals.var_xp_dn7 = assign15610_e22304_d_n7;
        locals.var_xp_dn10 = assign15610_e22304_d_n10;
        locals.var_xp_dn11 = assign15610_e22304_d_n11;
        locals.var_xp_dn12 = assign15610_e22304_d_n12;
        locals.var_xp_dn17 = assign15610_e22304_d_n17;

        let (assign15620_e22315, assign15620_e22315_d_n0, assign15620_e22315_d_n2, assign15620_e22315_d_n6, assign15620_e22315_d_n7, assign15620_e22315_d_n10, assign15620_e22315_d_n11, assign15620_e22315_d_n12, assign15620_e22315_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15620_e22313: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15620_e22313, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15620_e22315;
        locals.var_xmp_dn0 = assign15620_e22315_d_n0;
        locals.var_xmp_dn2 = assign15620_e22315_d_n2;
        locals.var_xmp_dn6 = assign15620_e22315_d_n6;
        locals.var_xmp_dn7 = assign15620_e22315_d_n7;
        locals.var_xmp_dn10 = assign15620_e22315_d_n10;
        locals.var_xmp_dn11 = assign15620_e22315_d_n11;
        locals.var_xmp_dn12 = assign15620_e22315_d_n12;
        locals.var_xmp_dn17 = assign15620_e22315_d_n17;

        let (assign15630_e22326, assign15630_e22326_d_n0, assign15630_e22326_d_n2, assign15630_e22326_d_n6, assign15630_e22326_d_n7, assign15630_e22326_d_n10, assign15630_e22326_d_n11, assign15630_e22326_d_n12, assign15630_e22326_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15630_e22324: f64 = (locals.var_xp + locals.var_xmp);
        (assign15630_e22324, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign15630_e22326;
        locals.var_arg_dn0 = assign15630_e22326_d_n0;
        locals.var_arg_dn2 = assign15630_e22326_d_n2;
        locals.var_arg_dn6 = assign15630_e22326_d_n6;
        locals.var_arg_dn7 = assign15630_e22326_d_n7;
        locals.var_arg_dn10 = assign15630_e22326_d_n10;
        locals.var_arg_dn11 = assign15630_e22326_d_n11;
        locals.var_arg_dn12 = assign15630_e22326_d_n12;
        locals.var_arg_dn17 = assign15630_e22326_d_n17;

        let (assign15640_e22335, assign15640_e22335_d_n0, assign15640_e22335_d_n2, assign15640_e22335_d_n6, assign15640_e22335_d_n7, assign15640_e22335_d_n10, assign15640_e22335_d_n11, assign15640_e22335_d_n12, assign15640_e22335_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15640_e22335;
        locals.var_dnm_dn0 = assign15640_e22335_d_n0;
        locals.var_dnm_dn2 = assign15640_e22335_d_n2;
        locals.var_dnm_dn6 = assign15640_e22335_d_n6;
        locals.var_dnm_dn7 = assign15640_e22335_d_n7;
        locals.var_dnm_dn10 = assign15640_e22335_d_n10;
        locals.var_dnm_dn11 = assign15640_e22335_d_n11;
        locals.var_dnm_dn12 = assign15640_e22335_d_n12;
        locals.var_dnm_dn17 = assign15640_e22335_d_n17;

        let assign15650_e22350: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard470 = assign15650_e22350;

        let assign15660_e22353: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign15660_e22353;

        let (assign15670_e22366,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15670_e22366;

        let assign15680_e22369: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard472 = assign15680_e22369;

        let (assign15690_e22385,) = {
    if ((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15690_e22385;

        let assign15700_e22388: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign15700_e22388;

    }

    pub(super) fn stamp_transient_block_52(
        locals: &mut StampLocals,
    ) {
        let (assign15710_e22407,) = {
    if (((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 == 0.0)) && (locals.var_guard473 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15710_e22407;

        let assign15720_e22410: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign15720_e22410;

        let (assign15730_e22432,) = {
    if ((((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 == 0.0)) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15730_e22432;

        let (assign15740_e22443,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15740_e22443;

        let mut assign15750_loop_guard: usize = 0;
        while {
            let assign15750_cond_e22455: f64 = if (((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign15750_cond_e22455 != 0.0
        } {
            assign15750_loop_guard += 1;
            assert!(assign15750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15750_body0_e22467, assign15750_body0_e22467_d_n0, assign15750_body0_e22467_d_n2, assign15750_body0_e22467_d_n6, assign15750_body0_e22467_d_n7, assign15750_body0_e22467_d_n10, assign15750_body0_e22467_d_n11, assign15750_body0_e22467_d_n12, assign15750_body0_e22467_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) {
        let assign15750_body0_e22465: f64 = (locals.var_dnm).sqrt();
        (assign15750_body0_e22465, (locals.var_dnm_dn0 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn2 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn6 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn7 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn10 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn11 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn12 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn17 / (2.0 * assign15750_body0_e22465)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign15750_body0_e22467;
            locals.var_dnm_dn0 = assign15750_body0_e22467_d_n0;
            locals.var_dnm_dn2 = assign15750_body0_e22467_d_n2;
            locals.var_dnm_dn6 = assign15750_body0_e22467_d_n6;
            locals.var_dnm_dn7 = assign15750_body0_e22467_d_n7;
            locals.var_dnm_dn10 = assign15750_body0_e22467_d_n10;
            locals.var_dnm_dn11 = assign15750_body0_e22467_d_n11;
            locals.var_dnm_dn12 = assign15750_body0_e22467_d_n12;
            locals.var_dnm_dn17 = assign15750_body0_e22467_d_n17;
            let (assign15750_body1_e22480,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) {
        let assign15750_body1_e22478: f64 = (locals.var_m0 + 1.0);
        (assign15750_body1_e22478,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign15750_body1_e22480;
        }

        let (assign15760_e22498, assign15760_e22498_d_n0, assign15760_e22498_d_n2, assign15760_e22498_d_n6, assign15760_e22498_d_n7, assign15760_e22498_d_n10, assign15760_e22498_d_n11, assign15760_e22498_d_n12, assign15760_e22498_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 == 0.0)) {
        let assign15760_e22494: f64 = (2.0 * 4.0);
        let assign15760_e22495: f64 = (1.0 / assign15760_e22494);
        let assign15760_e22496: f64 = (locals.var_dnm).powf(assign15760_e22495);
        (assign15760_e22496, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn0)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn2)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn6)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn7)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn10)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn11)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn12)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn17)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15760_e22498;
        locals.var_dnm_dn0 = assign15760_e22498_d_n0;
        locals.var_dnm_dn2 = assign15760_e22498_d_n2;
        locals.var_dnm_dn6 = assign15760_e22498_d_n6;
        locals.var_dnm_dn7 = assign15760_e22498_d_n7;
        locals.var_dnm_dn10 = assign15760_e22498_d_n10;
        locals.var_dnm_dn11 = assign15760_e22498_d_n11;
        locals.var_dnm_dn12 = assign15760_e22498_d_n12;
        locals.var_dnm_dn17 = assign15760_e22498_d_n17;

        let (assign15770_e22509, assign15770_e22509_d_n0, assign15770_e22509_d_n2, assign15770_e22509_d_n6, assign15770_e22509_d_n7, assign15770_e22509_d_n10, assign15770_e22509_d_n11, assign15770_e22509_d_n12, assign15770_e22509_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15770_e22507: f64 = (1.0 / locals.var_dnm);
        (assign15770_e22507, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15770_e22509;
        locals.var_dnm_dn0 = assign15770_e22509_d_n0;
        locals.var_dnm_dn2 = assign15770_e22509_d_n2;
        locals.var_dnm_dn6 = assign15770_e22509_d_n6;
        locals.var_dnm_dn7 = assign15770_e22509_d_n7;
        locals.var_dnm_dn10 = assign15770_e22509_d_n10;
        locals.var_dnm_dn11 = assign15770_e22509_d_n11;
        locals.var_dnm_dn12 = assign15770_e22509_d_n12;
        locals.var_dnm_dn17 = assign15770_e22509_d_n17;

        let (assign15780_e22522, assign15780_e22522_d_n0, assign15780_e22522_d_n2, assign15780_e22522_d_n6, assign15780_e22522_d_n7, assign15780_e22522_d_n10, assign15780_e22522_d_n11, assign15780_e22522_d_n12, assign15780_e22522_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15780_e22518: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign15780_e22520: f64 = (assign15780_e22518 * locals.var_dnm);
        (assign15780_e22520, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn12)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn17)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign15780_e22522;
        locals.var_tmf0_dn0 = assign15780_e22522_d_n0;
        locals.var_tmf0_dn2 = assign15780_e22522_d_n2;
        locals.var_tmf0_dn6 = assign15780_e22522_d_n6;
        locals.var_tmf0_dn7 = assign15780_e22522_d_n7;
        locals.var_tmf0_dn10 = assign15780_e22522_d_n10;
        locals.var_tmf0_dn11 = assign15780_e22522_d_n11;
        locals.var_tmf0_dn12 = assign15780_e22522_d_n12;
        locals.var_tmf0_dn17 = assign15780_e22522_d_n17;

        let (assign15790_e22535, assign15790_e22535_d_n0, assign15790_e22535_d_n2, assign15790_e22535_d_n6, assign15790_e22535_d_n7, assign15790_e22535_d_n10, assign15790_e22535_d_n11, assign15790_e22535_d_n12, assign15790_e22535_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15790_e22531: f64 = locals.var_t5;
        let assign15790_e22533: f64 = (assign15790_e22531 - locals.var_tmf0);
        (assign15790_e22533, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn12 - locals.var_tmf0_dn12), (locals.var_t5_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    }
};
        locals.var_t4__blk465 = assign15790_e22535;
        locals.var_t4__blk465_dn0 = assign15790_e22535_d_n0;
        locals.var_t4__blk465_dn2 = assign15790_e22535_d_n2;
        locals.var_t4__blk465_dn6 = assign15790_e22535_d_n6;
        locals.var_t4__blk465_dn7 = assign15790_e22535_d_n7;
        locals.var_t4__blk465_dn10 = assign15790_e22535_d_n10;
        locals.var_t4__blk465_dn11 = assign15790_e22535_d_n11;
        locals.var_t4__blk465_dn12 = assign15790_e22535_d_n12;
        locals.var_t4__blk465_dn17 = assign15790_e22535_d_n17;

        let (assign15800_e22545, assign15800_e22545_d_n0, assign15800_e22545_d_n2, assign15800_e22545_d_n6, assign15800_e22545_d_n7, assign15800_e22545_d_n10, assign15800_e22545_d_n11, assign15800_e22545_d_n12, assign15800_e22545_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 == 0.0)) {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    } else {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    }
};
        locals.var_t4__blk465 = assign15800_e22545;
        locals.var_t4__blk465_dn0 = assign15800_e22545_d_n0;
        locals.var_t4__blk465_dn2 = assign15800_e22545_d_n2;
        locals.var_t4__blk465_dn6 = assign15800_e22545_d_n6;
        locals.var_t4__blk465_dn7 = assign15800_e22545_d_n7;
        locals.var_t4__blk465_dn10 = assign15800_e22545_d_n10;
        locals.var_t4__blk465_dn11 = assign15800_e22545_d_n11;
        locals.var_t4__blk465_dn12 = assign15800_e22545_d_n12;
        locals.var_t4__blk465_dn17 = assign15800_e22545_d_n17;

        let (assign15810_e22553, assign15810_e22553_d_n0, assign15810_e22553_d_n2, assign15810_e22553_d_n6, assign15810_e22553_d_n7, assign15810_e22553_d_n10, assign15810_e22553_d_n11, assign15810_e22553_d_n12, assign15810_e22553_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15810_e22551: f64 = (locals.var_t4__blk465).sqrt();
        (assign15810_e22551, (locals.var_t4__blk465_dn0 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn2 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn6 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn7 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn10 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn11 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn12 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn17 / (2.0 * assign15810_e22551)),)
    } else {
        (locals.var_t3__blk464, locals.var_t3__blk464_dn0, locals.var_t3__blk464_dn2, locals.var_t3__blk464_dn6, locals.var_t3__blk464_dn7, locals.var_t3__blk464_dn10, locals.var_t3__blk464_dn11, locals.var_t3__blk464_dn12, locals.var_t3__blk464_dn17,)
    }
};
        locals.var_t3__blk464 = assign15810_e22553;
        locals.var_t3__blk464_dn0 = assign15810_e22553_d_n0;
        locals.var_t3__blk464_dn2 = assign15810_e22553_d_n2;
        locals.var_t3__blk464_dn6 = assign15810_e22553_d_n6;
        locals.var_t3__blk464_dn7 = assign15810_e22553_d_n7;
        locals.var_t3__blk464_dn10 = assign15810_e22553_d_n10;
        locals.var_t3__blk464_dn11 = assign15810_e22553_d_n11;
        locals.var_t3__blk464_dn12 = assign15810_e22553_d_n12;
        locals.var_t3__blk464_dn17 = assign15810_e22553_d_n17;

        let (assign15820_e22566, assign15820_e22566_d_n0, assign15820_e22566_d_n2, assign15820_e22566_d_n6, assign15820_e22566_d_n7, assign15820_e22566_d_n10, assign15820_e22566_d_n11, assign15820_e22566_d_n12, assign15820_e22566_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15820_e22562: f64 = (1.0 - locals.var_t3__blk464);
        let assign15820_e22563: f64 = (locals.var_t2__blk463 * assign15820_e22562);
        let assign15820_e22564: f64 = (locals.var_vgp + assign15820_e22563);
        (assign15820_e22564, (locals.var_vgp_dn0 + ((locals.var_t2__blk463_dn0 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2__blk463_dn2 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn2)))), (locals.var_vgp_dn6 + ((locals.var_t2__blk463_dn6 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2__blk463_dn7 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn7)))), (locals.var_vgp_dn10 + ((locals.var_t2__blk463_dn10 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2__blk463_dn11 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn11)))), (locals.var_vgp_dn12 + ((locals.var_t2__blk463_dn12 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn12)))), (locals.var_vgp_dn17 + ((locals.var_t2__blk463_dn17 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn17)))),)
    } else {
        (locals.var_t10__blk468, locals.var_t10__blk468_dn0, locals.var_t10__blk468_dn2, locals.var_t10__blk468_dn6, locals.var_t10__blk468_dn7, locals.var_t10__blk468_dn10, locals.var_t10__blk468_dn11, locals.var_t10__blk468_dn12, locals.var_t10__blk468_dn17,)
    }
};
        locals.var_t10__blk468 = assign15820_e22566;
        locals.var_t10__blk468_dn0 = assign15820_e22566_d_n0;
        locals.var_t10__blk468_dn2 = assign15820_e22566_d_n2;
        locals.var_t10__blk468_dn6 = assign15820_e22566_d_n6;
        locals.var_t10__blk468_dn7 = assign15820_e22566_d_n7;
        locals.var_t10__blk468_dn10 = assign15820_e22566_d_n10;
        locals.var_t10__blk468_dn11 = assign15820_e22566_d_n11;
        locals.var_t10__blk468_dn12 = assign15820_e22566_d_n12;
        locals.var_t10__blk468_dn17 = assign15820_e22566_d_n17;

        let (assign15830_e22582, assign15830_e22582_d_n0, assign15830_e22582_d_n2, assign15830_e22582_d_n6, assign15830_e22582_d_n7, assign15830_e22582_d_n10, assign15830_e22582_d_n11, assign15830_e22582_d_n12, assign15830_e22582_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15830_e22573: f64 = (locals.var_t10__blk468 * locals.var_t10__blk468);
        let assign15830_e22576: f64 = (4.0 * 0.01);
        let assign15830_e22578: f64 = (assign15830_e22576 * 0.01);
        let assign15830_e22579: f64 = (assign15830_e22573 + assign15830_e22578);
        let assign15830_e22580: f64 = (assign15830_e22579).sqrt();
        (assign15830_e22580, (((locals.var_t10__blk468_dn0 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn0)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn2 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn2)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn6 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn6)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn7 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn7)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn10 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn10)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn11 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn11)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn12 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn12)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn17 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn17)) / (2.0 * assign15830_e22580)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign15830_e22582;
        locals.var_tmf1_dn0 = assign15830_e22582_d_n0;
        locals.var_tmf1_dn2 = assign15830_e22582_d_n2;
        locals.var_tmf1_dn6 = assign15830_e22582_d_n6;
        locals.var_tmf1_dn7 = assign15830_e22582_d_n7;
        locals.var_tmf1_dn10 = assign15830_e22582_d_n10;
        locals.var_tmf1_dn11 = assign15830_e22582_d_n11;
        locals.var_tmf1_dn12 = assign15830_e22582_d_n12;
        locals.var_tmf1_dn17 = assign15830_e22582_d_n17;

        let (assign15840_e22597, assign15840_e22597_d_n0, assign15840_e22597_d_n2, assign15840_e22597_d_n6, assign15840_e22597_d_n7, assign15840_e22597_d_n10, assign15840_e22597_d_n11, assign15840_e22597_d_n12, assign15840_e22597_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15840_e22590: f64 = (locals.var_t10__blk468 + locals.var_tmf1);
        let assign15840_e22591: f64 = (0.5 * assign15840_e22590);
        let assign15840_e22594: f64 = (1e-10 * 0.01);
        let assign15840_e22595: f64 = (assign15840_e22591 + assign15840_e22594);
        (assign15840_e22595, (0.5 * (locals.var_t10__blk468_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t10__blk468_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t10__blk468_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t10__blk468_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t10__blk468_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t10__blk468_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t10__blk468_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t10__blk468_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t10__blk468, locals.var_t10__blk468_dn0, locals.var_t10__blk468_dn2, locals.var_t10__blk468_dn6, locals.var_t10__blk468_dn7, locals.var_t10__blk468_dn10, locals.var_t10__blk468_dn11, locals.var_t10__blk468_dn12, locals.var_t10__blk468_dn17,)
    }
};
        locals.var_t10__blk468 = assign15840_e22597;
        locals.var_t10__blk468_dn0 = assign15840_e22597_d_n0;
        locals.var_t10__blk468_dn2 = assign15840_e22597_d_n2;
        locals.var_t10__blk468_dn6 = assign15840_e22597_d_n6;
        locals.var_t10__blk468_dn7 = assign15840_e22597_d_n7;
        locals.var_t10__blk468_dn10 = assign15840_e22597_d_n10;
        locals.var_t10__blk468_dn11 = assign15840_e22597_d_n11;
        locals.var_t10__blk468_dn12 = assign15840_e22597_d_n12;
        locals.var_t10__blk468_dn17 = assign15840_e22597_d_n17;

        let assign15850_e22600: f64 = if locals.var_t10__blk468 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign15850_e22600;

        let (assign15860_e22609, assign15860_e22609_d_n0, assign15860_e22609_d_n2, assign15860_e22609_d_n6, assign15860_e22609_d_n7, assign15860_e22609_d_n10, assign15860_e22609_d_n11, assign15860_e22609_d_n12, assign15860_e22609_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard475 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10__blk468, locals.var_t10__blk468_dn0, locals.var_t10__blk468_dn2, locals.var_t10__blk468_dn6, locals.var_t10__blk468_dn7, locals.var_t10__blk468_dn10, locals.var_t10__blk468_dn11, locals.var_t10__blk468_dn12, locals.var_t10__blk468_dn17,)
    }
};
        locals.var_t10__blk468 = assign15860_e22609;
        locals.var_t10__blk468_dn0 = assign15860_e22609_d_n0;
        locals.var_t10__blk468_dn2 = assign15860_e22609_d_n2;
        locals.var_t10__blk468_dn6 = assign15860_e22609_d_n6;
        locals.var_t10__blk468_dn7 = assign15860_e22609_d_n7;
        locals.var_t10__blk468_dn10 = assign15860_e22609_d_n10;
        locals.var_t10__blk468_dn11 = assign15860_e22609_d_n11;
        locals.var_t10__blk468_dn12 = assign15860_e22609_d_n12;
        locals.var_t10__blk468_dn17 = assign15860_e22609_d_n17;

        let (assign15880_e22625, assign15880_e22625_d_n0, assign15880_e22625_d_n2, assign15880_e22625_d_n6, assign15880_e22625_d_n7, assign15880_e22625_d_n10, assign15880_e22625_d_n11, assign15880_e22625_d_n12, assign15880_e22625_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15880_e22623: f64 = (locals.var_vds / locals.var_t10__blk468);
        (assign15880_e22623, (((locals.var_vds_dn0 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn0)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn2 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn2)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn6 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn6)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn7 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn7)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn10 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn10)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn11 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn11)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn12 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn12)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn17 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn17)) / (locals.var_t10__blk468 * locals.var_t10__blk468)),)
    } else {
        (locals.var_t1__blk462, locals.var_t1__blk462_dn0, locals.var_t1__blk462_dn2, locals.var_t1__blk462_dn6, locals.var_t1__blk462_dn7, locals.var_t1__blk462_dn10, locals.var_t1__blk462_dn11, locals.var_t1__blk462_dn12, locals.var_t1__blk462_dn17,)
    }
};
        locals.var_t1__blk462 = assign15880_e22625;
        locals.var_t1__blk462_dn0 = assign15880_e22625_d_n0;
        locals.var_t1__blk462_dn2 = assign15880_e22625_d_n2;
        locals.var_t1__blk462_dn6 = assign15880_e22625_d_n6;
        locals.var_t1__blk462_dn7 = assign15880_e22625_d_n7;
        locals.var_t1__blk462_dn10 = assign15880_e22625_d_n10;
        locals.var_t1__blk462_dn11 = assign15880_e22625_d_n11;
        locals.var_t1__blk462_dn12 = assign15880_e22625_d_n12;
        locals.var_t1__blk462_dn17 = assign15880_e22625_d_n17;

        let (assign15890_e22636, assign15890_e22636_d_n0, assign15890_e22636_d_n2, assign15890_e22636_d_n6, assign15890_e22636_d_n7, assign15890_e22636_d_n10, assign15890_e22636_d_n11, assign15890_e22636_d_n12, assign15890_e22636_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15890_e22633: f64 = (locals.var_ddlte - 1.0);
        let assign15890_e22634: f64 = (locals.var_t1__blk462).powf(assign15890_e22633);
        (assign15890_e22634, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn0)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn0 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn2)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn2 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn6)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn6 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn7)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn7 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn10)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn10 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn11)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn11 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn12)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn12 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn17)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn17 / locals.var_t1__blk462))) },)
    } else {
        (locals.var_t2__blk463, locals.var_t2__blk463_dn0, locals.var_t2__blk463_dn2, locals.var_t2__blk463_dn6, locals.var_t2__blk463_dn7, locals.var_t2__blk463_dn10, locals.var_t2__blk463_dn11, locals.var_t2__blk463_dn12, locals.var_t2__blk463_dn17,)
    }
};
        locals.var_t2__blk463 = assign15890_e22636;
        locals.var_t2__blk463_dn0 = assign15890_e22636_d_n0;
        locals.var_t2__blk463_dn2 = assign15890_e22636_d_n2;
        locals.var_t2__blk463_dn6 = assign15890_e22636_d_n6;
        locals.var_t2__blk463_dn7 = assign15890_e22636_d_n7;
        locals.var_t2__blk463_dn10 = assign15890_e22636_d_n10;
        locals.var_t2__blk463_dn11 = assign15890_e22636_d_n11;
        locals.var_t2__blk463_dn12 = assign15890_e22636_d_n12;
        locals.var_t2__blk463_dn17 = assign15890_e22636_d_n17;

        let (assign15900_e22645, assign15900_e22645_d_n0, assign15900_e22645_d_n2, assign15900_e22645_d_n6, assign15900_e22645_d_n7, assign15900_e22645_d_n10, assign15900_e22645_d_n11, assign15900_e22645_d_n12, assign15900_e22645_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15900_e22643: f64 = (locals.var_t2__blk463 * locals.var_t1__blk462);
        (assign15900_e22643, ((locals.var_t2__blk463_dn0 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn0)), ((locals.var_t2__blk463_dn2 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn2)), ((locals.var_t2__blk463_dn6 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn6)), ((locals.var_t2__blk463_dn7 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn7)), ((locals.var_t2__blk463_dn10 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn10)), ((locals.var_t2__blk463_dn11 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn11)), ((locals.var_t2__blk463_dn12 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn12)), ((locals.var_t2__blk463_dn17 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn17)),)
    } else {
        (locals.var_t7__blk467, locals.var_t7__blk467_dn0, locals.var_t7__blk467_dn2, locals.var_t7__blk467_dn6, locals.var_t7__blk467_dn7, locals.var_t7__blk467_dn10, locals.var_t7__blk467_dn11, locals.var_t7__blk467_dn12, locals.var_t7__blk467_dn17,)
    }
};
        locals.var_t7__blk467 = assign15900_e22645;
        locals.var_t7__blk467_dn0 = assign15900_e22645_d_n0;
        locals.var_t7__blk467_dn2 = assign15900_e22645_d_n2;
        locals.var_t7__blk467_dn6 = assign15900_e22645_d_n6;
        locals.var_t7__blk467_dn7 = assign15900_e22645_d_n7;
        locals.var_t7__blk467_dn10 = assign15900_e22645_d_n10;
        locals.var_t7__blk467_dn11 = assign15900_e22645_d_n11;
        locals.var_t7__blk467_dn12 = assign15900_e22645_d_n12;
        locals.var_t7__blk467_dn17 = assign15900_e22645_d_n17;

        let (assign15910_e22654, assign15910_e22654_d_n0, assign15910_e22654_d_n2, assign15910_e22654_d_n6, assign15910_e22654_d_n7, assign15910_e22654_d_n10, assign15910_e22654_d_n11, assign15910_e22654_d_n12, assign15910_e22654_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15910_e22652: f64 = (1.0 + locals.var_t7__blk467);
        (assign15910_e22652, locals.var_t7__blk467_dn0, locals.var_t7__blk467_dn2, locals.var_t7__blk467_dn6, locals.var_t7__blk467_dn7, locals.var_t7__blk467_dn10, locals.var_t7__blk467_dn11, locals.var_t7__blk467_dn12, locals.var_t7__blk467_dn17,)
    } else {
        (locals.var_t3__blk464, locals.var_t3__blk464_dn0, locals.var_t3__blk464_dn2, locals.var_t3__blk464_dn6, locals.var_t3__blk464_dn7, locals.var_t3__blk464_dn10, locals.var_t3__blk464_dn11, locals.var_t3__blk464_dn12, locals.var_t3__blk464_dn17,)
    }
};
        locals.var_t3__blk464 = assign15910_e22654;
        locals.var_t3__blk464_dn0 = assign15910_e22654_d_n0;
        locals.var_t3__blk464_dn2 = assign15910_e22654_d_n2;
        locals.var_t3__blk464_dn6 = assign15910_e22654_d_n6;
        locals.var_t3__blk464_dn7 = assign15910_e22654_d_n7;
        locals.var_t3__blk464_dn10 = assign15910_e22654_d_n10;
        locals.var_t3__blk464_dn11 = assign15910_e22654_d_n11;
        locals.var_t3__blk464_dn12 = assign15910_e22654_d_n12;
        locals.var_t3__blk464_dn17 = assign15910_e22654_d_n17;

        let (assign15920_e22667, assign15920_e22667_d_n0, assign15920_e22667_d_n2, assign15920_e22667_d_n6, assign15920_e22667_d_n7, assign15920_e22667_d_n10, assign15920_e22667_d_n11, assign15920_e22667_d_n12, assign15920_e22667_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15920_e22662: f64 = (1.0 / locals.var_ddlte);
        let assign15920_e22664: f64 = (assign15920_e22662 - 1.0);
        let assign15920_e22665: f64 = (locals.var_t3__blk464).powf(assign15920_e22664);
        (assign15920_e22665, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn0)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn0 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn2)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn2 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn6)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn6 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn7)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn7 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn10)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn10 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn11)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn11 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn12)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn12 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn17)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn17 / locals.var_t3__blk464))) },)
    } else {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    }
};
        locals.var_t4__blk465 = assign15920_e22667;
        locals.var_t4__blk465_dn0 = assign15920_e22667_d_n0;
        locals.var_t4__blk465_dn2 = assign15920_e22667_d_n2;
        locals.var_t4__blk465_dn6 = assign15920_e22667_d_n6;
        locals.var_t4__blk465_dn7 = assign15920_e22667_d_n7;
        locals.var_t4__blk465_dn10 = assign15920_e22667_d_n10;
        locals.var_t4__blk465_dn11 = assign15920_e22667_d_n11;
        locals.var_t4__blk465_dn12 = assign15920_e22667_d_n12;
        locals.var_t4__blk465_dn17 = assign15920_e22667_d_n17;

        let (assign15930_e22676, assign15930_e22676_d_n0, assign15930_e22676_d_n2, assign15930_e22676_d_n6, assign15930_e22676_d_n7, assign15930_e22676_d_n10, assign15930_e22676_d_n11, assign15930_e22676_d_n12, assign15930_e22676_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15930_e22674: f64 = (locals.var_t4__blk465 * locals.var_t3__blk464);
        (assign15930_e22674, ((locals.var_t4__blk465_dn0 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn0)), ((locals.var_t4__blk465_dn2 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn2)), ((locals.var_t4__blk465_dn6 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn6)), ((locals.var_t4__blk465_dn7 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn7)), ((locals.var_t4__blk465_dn10 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn10)), ((locals.var_t4__blk465_dn11 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn11)), ((locals.var_t4__blk465_dn12 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn12)), ((locals.var_t4__blk465_dn17 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn17)),)
    } else {
        (locals.var_t6__blk466, locals.var_t6__blk466_dn0, locals.var_t6__blk466_dn2, locals.var_t6__blk466_dn6, locals.var_t6__blk466_dn7, locals.var_t6__blk466_dn10, locals.var_t6__blk466_dn11, locals.var_t6__blk466_dn12, locals.var_t6__blk466_dn17,)
    }
};
        locals.var_t6__blk466 = assign15930_e22676;
        locals.var_t6__blk466_dn0 = assign15930_e22676_d_n0;
        locals.var_t6__blk466_dn2 = assign15930_e22676_d_n2;
        locals.var_t6__blk466_dn6 = assign15930_e22676_d_n6;
        locals.var_t6__blk466_dn7 = assign15930_e22676_d_n7;
        locals.var_t6__blk466_dn10 = assign15930_e22676_d_n10;
        locals.var_t6__blk466_dn11 = assign15930_e22676_d_n11;
        locals.var_t6__blk466_dn12 = assign15930_e22676_d_n12;
        locals.var_t6__blk466_dn17 = assign15930_e22676_d_n17;

        let (assign15940_e22685, assign15940_e22685_d_n0, assign15940_e22685_d_n2, assign15940_e22685_d_n6, assign15940_e22685_d_n7, assign15940_e22685_d_n10, assign15940_e22685_d_n11, assign15940_e22685_d_n12, assign15940_e22685_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15940_e22683: f64 = (locals.var_vds / locals.var_t6__blk466);
        (assign15940_e22683, (((locals.var_vds_dn0 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn0)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn2 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn2)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn6 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn6)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn7 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn7)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn10 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn10)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn11 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn11)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn12 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn12)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn17 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn17)) / (locals.var_t6__blk466 * locals.var_t6__blk466)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    }
};
        locals.var_vdseff = assign15940_e22685;
        locals.var_vdseff_dn0 = assign15940_e22685_d_n0;
        locals.var_vdseff_dn2 = assign15940_e22685_d_n2;
        locals.var_vdseff_dn6 = assign15940_e22685_d_n6;
        locals.var_vdseff_dn7 = assign15940_e22685_d_n7;
        locals.var_vdseff_dn10 = assign15940_e22685_d_n10;
        locals.var_vdseff_dn11 = assign15940_e22685_d_n11;
        locals.var_vdseff_dn12 = assign15940_e22685_d_n12;
        locals.var_vdseff_dn17 = assign15940_e22685_d_n17;

        let (assign15950_e22692, assign15950_e22692_d_n0, assign15950_e22692_d_n2, assign15950_e22692_d_n6, assign15950_e22692_d_n7, assign15950_e22692_d_n10, assign15950_e22692_d_n11, assign15950_e22692_d_n12, assign15950_e22692_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign15950_e22692;
        locals.var_vds_dn0 = assign15950_e22692_d_n0;
        locals.var_vds_dn2 = assign15950_e22692_d_n2;
        locals.var_vds_dn6 = assign15950_e22692_d_n6;
        locals.var_vds_dn7 = assign15950_e22692_d_n7;
        locals.var_vds_dn10 = assign15950_e22692_d_n10;
        locals.var_vds_dn11 = assign15950_e22692_d_n11;
        locals.var_vds_dn12 = assign15950_e22692_d_n12;
        locals.var_vds_dn17 = assign15950_e22692_d_n17;

        let (assign15960_e22704, assign15960_e22704_d_n0, assign15960_e22704_d_n2, assign15960_e22704_d_n6, assign15960_e22704_d_n7, assign15960_e22704_d_n10, assign15960_e22704_d_n11, assign15960_e22704_d_n12, assign15960_e22704_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15960_e22700: f64 = (locals.var_vbcs_cl - locals.var_vds);
        let assign15960_e22701: f64 = (locals.var_beta * assign15960_e22700);
        let assign15960_e22702: f64 = (assign15960_e22701).exp();
        (assign15960_e22702, (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn0 - locals.var_vds_dn0))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn2 - locals.var_vds_dn2))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn6 - locals.var_vds_dn6))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn7 - locals.var_vds_dn7))), (assign15960_e22702 * ((locals.var_beta_dn10 * assign15960_e22700) + (locals.var_beta * (locals.var_vbcs_cl_dn10 - locals.var_vds_dn10)))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn11 - locals.var_vds_dn11))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn12 - locals.var_vds_dn12))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn12, locals.var_exp_bvbsvds_dn17,)
    }
};
        locals.var_exp_bvbsvds = assign15960_e22704;
        locals.var_exp_bvbsvds_dn0 = assign15960_e22704_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign15960_e22704_d_n2;
        locals.var_exp_bvbsvds_dn6 = assign15960_e22704_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign15960_e22704_d_n7;
        locals.var_exp_bvbsvds_dn10 = assign15960_e22704_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign15960_e22704_d_n11;
        locals.var_exp_bvbsvds_dn12 = assign15960_e22704_d_n12;
        locals.var_exp_bvbsvds_dn17 = assign15960_e22704_d_n17;

        let assign15970_e22707: f64 = if locals.var_vds <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign15970_e22707;

        let (assign15980_e22716, assign15980_e22716_d_n0, assign15980_e22716_d_n2, assign15980_e22716_d_n6, assign15980_e22716_d_n7, assign15980_e22716_d_n10, assign15980_e22716_d_n11, assign15980_e22716_d_n12, assign15980_e22716_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign15980_e22716;
        locals.var_pds_dn0 = assign15980_e22716_d_n0;
        locals.var_pds_dn2 = assign15980_e22716_d_n2;
        locals.var_pds_dn6 = assign15980_e22716_d_n6;
        locals.var_pds_dn7 = assign15980_e22716_d_n7;
        locals.var_pds_dn10 = assign15980_e22716_d_n10;
        locals.var_pds_dn11 = assign15980_e22716_d_n11;
        locals.var_pds_dn12 = assign15980_e22716_d_n12;
        locals.var_pds_dn17 = assign15980_e22716_d_n17;

        let (assign15990_e22725, assign15990_e22725_d_n0, assign15990_e22725_d_n2, assign15990_e22725_d_n6, assign15990_e22725_d_n7, assign15990_e22725_d_n10, assign15990_e22725_d_n11, assign15990_e22725_d_n12, assign15990_e22725_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign15990_e22725;
        locals.var_psl_dn0 = assign15990_e22725_d_n0;
        locals.var_psl_dn2 = assign15990_e22725_d_n2;
        locals.var_psl_dn6 = assign15990_e22725_d_n6;
        locals.var_psl_dn7 = assign15990_e22725_d_n7;
        locals.var_psl_dn10 = assign15990_e22725_d_n10;
        locals.var_psl_dn11 = assign15990_e22725_d_n11;
        locals.var_psl_dn12 = assign15990_e22725_d_n12;
        locals.var_psl_dn17 = assign15990_e22725_d_n17;

        let (assign16000_e22734,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign16000_e22734;

        let assign16010_e22737: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign16010_e22737;

        let (assign16020_e22749, assign16020_e22749_d_n0, assign16020_e22749_d_n2, assign16020_e22749_d_n6, assign16020_e22749_d_n7, assign16020_e22749_d_n10, assign16020_e22749_d_n11, assign16020_e22749_d_n12, assign16020_e22749_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard477 != 0.0)) {
        (locals.var_pssl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign16020_e22749;
        locals.var_phi_sl_soi_dn0 = assign16020_e22749_d_n0;
        locals.var_phi_sl_soi_dn2 = assign16020_e22749_d_n2;
        locals.var_phi_sl_soi_dn6 = assign16020_e22749_d_n6;
        locals.var_phi_sl_soi_dn7 = assign16020_e22749_d_n7;
        locals.var_phi_sl_soi_dn10 = assign16020_e22749_d_n10;
        locals.var_phi_sl_soi_dn11 = assign16020_e22749_d_n11;
        locals.var_phi_sl_soi_dn12 = assign16020_e22749_d_n12;
        locals.var_phi_sl_soi_dn17 = assign16020_e22749_d_n17;

        let (assign16030_e22763, assign16030_e22763_d_n0, assign16030_e22763_d_n2, assign16030_e22763_d_n6, assign16030_e22763_d_n7, assign16030_e22763_d_n10, assign16030_e22763_d_n11, assign16030_e22763_d_n12, assign16030_e22763_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard477 != 0.0)) {
        let assign16030_e22761: f64 = (locals.var_pssl_ini - locals.var_ps0);
        (assign16030_e22761, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn12), (-locals.var_ps0_dn17),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16030_e22763;
        locals.var_pds_ini_dn0 = assign16030_e22763_d_n0;
        locals.var_pds_ini_dn2 = assign16030_e22763_d_n2;
        locals.var_pds_ini_dn6 = assign16030_e22763_d_n6;
        locals.var_pds_ini_dn7 = assign16030_e22763_d_n7;
        locals.var_pds_ini_dn10 = assign16030_e22763_d_n10;
        locals.var_pds_ini_dn11 = assign16030_e22763_d_n11;
        locals.var_pds_ini_dn12 = assign16030_e22763_d_n12;
        locals.var_pds_ini_dn17 = assign16030_e22763_d_n17;

        let assign16040_e22766: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign16040_e22766;

    }

    pub(super) fn stamp_transient_block_53(
        locals: &mut StampLocals,
    ) {
        let (assign16050_e22787, assign16050_e22787_d_n0, assign16050_e22787_d_n2, assign16050_e22787_d_n6, assign16050_e22787_d_n7, assign16050_e22787_d_n10, assign16050_e22787_d_n11, assign16050_e22787_d_n12, assign16050_e22787_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16050_e22778: f64 = (locals.var_psl_lim - locals.var_ps0);
        let (assign16050_e22785, assign16050_e22785_d_n0, assign16050_e22785_d_n2, assign16050_e22785_d_n6, assign16050_e22785_d_n7, assign16050_e22785_d_n10, assign16050_e22785_d_n11, assign16050_e22785_d_n12, assign16050_e22785_d_n17,) = {
            if (assign16050_e22778 >= 0.0) {
                let assign16050_e22783: f64 = (locals.var_psl_lim - locals.var_ps0);
                (assign16050_e22783, (locals.var_psl_lim_dn0 - locals.var_ps0_dn0), (locals.var_psl_lim_dn2 - locals.var_ps0_dn2), (locals.var_psl_lim_dn6 - locals.var_ps0_dn6), (locals.var_psl_lim_dn7 - locals.var_ps0_dn7), (locals.var_psl_lim_dn10 - locals.var_ps0_dn10), (locals.var_psl_lim_dn11 - locals.var_ps0_dn11), (locals.var_psl_lim_dn12 - locals.var_ps0_dn12), (locals.var_psl_lim_dn17 - locals.var_ps0_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign16050_e22785, assign16050_e22785_d_n0, assign16050_e22785_d_n2, assign16050_e22785_d_n6, assign16050_e22785_d_n7, assign16050_e22785_d_n10, assign16050_e22785_d_n11, assign16050_e22785_d_n12, assign16050_e22785_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign16050_e22787;
        locals.var_pds_max_dn0 = assign16050_e22787_d_n0;
        locals.var_pds_max_dn2 = assign16050_e22787_d_n2;
        locals.var_pds_max_dn6 = assign16050_e22787_d_n6;
        locals.var_pds_max_dn7 = assign16050_e22787_d_n7;
        locals.var_pds_max_dn10 = assign16050_e22787_d_n10;
        locals.var_pds_max_dn11 = assign16050_e22787_d_n11;
        locals.var_pds_max_dn12 = assign16050_e22787_d_n12;
        locals.var_pds_max_dn17 = assign16050_e22787_d_n17;

        let (assign16060_e22807, assign16060_e22807_d_n0, assign16060_e22807_d_n2, assign16060_e22807_d_n6, assign16060_e22807_d_n7, assign16060_e22807_d_n10, assign16060_e22807_d_n11, assign16060_e22807_d_n12, assign16060_e22807_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16060_e22799: f64 = (1.0 + 0.3);
        let assign16060_e22801: f64 = (assign16060_e22799 * locals.var_pds_max);
        let assign16060_e22803: f64 = (assign16060_e22801 - locals.var_vds);
        let assign16060_e22805: f64 = (assign16060_e22803 - 0.03);
        (assign16060_e22805, ((assign16060_e22799 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign16060_e22799 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign16060_e22799 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign16060_e22799 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign16060_e22799 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign16060_e22799 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign16060_e22799 * locals.var_pds_max_dn12) - locals.var_vds_dn12), ((assign16060_e22799 * locals.var_pds_max_dn17) - locals.var_vds_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign16060_e22807;
        locals.var_tmf1_dn0 = assign16060_e22807_d_n0;
        locals.var_tmf1_dn2 = assign16060_e22807_d_n2;
        locals.var_tmf1_dn6 = assign16060_e22807_d_n6;
        locals.var_tmf1_dn7 = assign16060_e22807_d_n7;
        locals.var_tmf1_dn10 = assign16060_e22807_d_n10;
        locals.var_tmf1_dn11 = assign16060_e22807_d_n11;
        locals.var_tmf1_dn12 = assign16060_e22807_d_n12;
        locals.var_tmf1_dn17 = assign16060_e22807_d_n17;

        let (assign16070_e22827, assign16070_e22827_d_n0, assign16070_e22827_d_n2, assign16070_e22827_d_n6, assign16070_e22827_d_n7, assign16070_e22827_d_n10, assign16070_e22827_d_n11, assign16070_e22827_d_n12, assign16070_e22827_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16070_e22820: f64 = (1.0 + 0.3);
        let assign16070_e22822: f64 = (assign16070_e22820 * locals.var_pds_max);
        let assign16070_e22823: f64 = (4.0 * assign16070_e22822);
        let assign16070_e22825: f64 = (assign16070_e22823 * 0.03);
        (assign16070_e22825, ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16070_e22827;
        locals.var_tmf2_dn0 = assign16070_e22827_d_n0;
        locals.var_tmf2_dn2 = assign16070_e22827_d_n2;
        locals.var_tmf2_dn6 = assign16070_e22827_d_n6;
        locals.var_tmf2_dn7 = assign16070_e22827_d_n7;
        locals.var_tmf2_dn10 = assign16070_e22827_d_n10;
        locals.var_tmf2_dn11 = assign16070_e22827_d_n11;
        locals.var_tmf2_dn12 = assign16070_e22827_d_n12;
        locals.var_tmf2_dn17 = assign16070_e22827_d_n17;

        let (assign16080_e22845, assign16080_e22845_d_n0, assign16080_e22845_d_n2, assign16080_e22845_d_n6, assign16080_e22845_d_n7, assign16080_e22845_d_n10, assign16080_e22845_d_n11, assign16080_e22845_d_n12, assign16080_e22845_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let (assign16080_e22843, assign16080_e22843_d_n0, assign16080_e22843_d_n2, assign16080_e22843_d_n6, assign16080_e22843_d_n7, assign16080_e22843_d_n10, assign16080_e22843_d_n11, assign16080_e22843_d_n12, assign16080_e22843_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign16080_e22842: f64 = (-locals.var_tmf2);
                (assign16080_e22842, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign16080_e22843, assign16080_e22843_d_n0, assign16080_e22843_d_n2, assign16080_e22843_d_n6, assign16080_e22843_d_n7, assign16080_e22843_d_n10, assign16080_e22843_d_n11, assign16080_e22843_d_n12, assign16080_e22843_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16080_e22845;
        locals.var_tmf2_dn0 = assign16080_e22845_d_n0;
        locals.var_tmf2_dn2 = assign16080_e22845_d_n2;
        locals.var_tmf2_dn6 = assign16080_e22845_d_n6;
        locals.var_tmf2_dn7 = assign16080_e22845_d_n7;
        locals.var_tmf2_dn10 = assign16080_e22845_d_n10;
        locals.var_tmf2_dn11 = assign16080_e22845_d_n11;
        locals.var_tmf2_dn12 = assign16080_e22845_d_n12;
        locals.var_tmf2_dn17 = assign16080_e22845_d_n17;

        let (assign16090_e22862, assign16090_e22862_d_n0, assign16090_e22862_d_n2, assign16090_e22862_d_n6, assign16090_e22862_d_n7, assign16090_e22862_d_n10, assign16090_e22862_d_n11, assign16090_e22862_d_n12, assign16090_e22862_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16090_e22857: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign16090_e22859: f64 = (assign16090_e22857 + locals.var_tmf2);
        let assign16090_e22860: f64 = (assign16090_e22859).sqrt();
        (assign16090_e22860, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign16090_e22860)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16090_e22862;
        locals.var_tmf2_dn0 = assign16090_e22862_d_n0;
        locals.var_tmf2_dn2 = assign16090_e22862_d_n2;
        locals.var_tmf2_dn6 = assign16090_e22862_d_n6;
        locals.var_tmf2_dn7 = assign16090_e22862_d_n7;
        locals.var_tmf2_dn10 = assign16090_e22862_d_n10;
        locals.var_tmf2_dn11 = assign16090_e22862_d_n11;
        locals.var_tmf2_dn12 = assign16090_e22862_d_n12;
        locals.var_tmf2_dn17 = assign16090_e22862_d_n17;

        let (assign16100_e22884, assign16100_e22884_d_n0, assign16100_e22884_d_n2, assign16100_e22884_d_n6, assign16100_e22884_d_n7, assign16100_e22884_d_n10, assign16100_e22884_d_n11, assign16100_e22884_d_n12, assign16100_e22884_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16100_e22874: f64 = (1.0 + 0.3);
        let assign16100_e22876: f64 = (assign16100_e22874 * locals.var_pds_max);
        let assign16100_e22880: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign16100_e22881: f64 = (0.5 * assign16100_e22880);
        let assign16100_e22882: f64 = (assign16100_e22876 - assign16100_e22881);
        (assign16100_e22882, ((assign16100_e22874 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign16100_e22874 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign16100_e22874 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign16100_e22874 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign16100_e22874 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign16100_e22874 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign16100_e22874 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign16100_e22874 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16100_e22884;
        locals.var_pds_ini_dn0 = assign16100_e22884_d_n0;
        locals.var_pds_ini_dn2 = assign16100_e22884_d_n2;
        locals.var_pds_ini_dn6 = assign16100_e22884_d_n6;
        locals.var_pds_ini_dn7 = assign16100_e22884_d_n7;
        locals.var_pds_ini_dn10 = assign16100_e22884_d_n10;
        locals.var_pds_ini_dn11 = assign16100_e22884_d_n11;
        locals.var_pds_ini_dn12 = assign16100_e22884_d_n12;
        locals.var_pds_ini_dn17 = assign16100_e22884_d_n17;

        let (assign16110_e22901, assign16110_e22901_d_n0, assign16110_e22901_d_n2, assign16110_e22901_d_n6, assign16110_e22901_d_n7, assign16110_e22901_d_n10, assign16110_e22901_d_n11, assign16110_e22901_d_n12, assign16110_e22901_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let (assign16110_e22899, assign16110_e22899_d_n0, assign16110_e22899_d_n2, assign16110_e22899_d_n6, assign16110_e22899_d_n7, assign16110_e22899_d_n10, assign16110_e22899_d_n11, assign16110_e22899_d_n12, assign16110_e22899_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign16110_e22899, assign16110_e22899_d_n0, assign16110_e22899_d_n2, assign16110_e22899_d_n6, assign16110_e22899_d_n7, assign16110_e22899_d_n10, assign16110_e22899_d_n11, assign16110_e22899_d_n12, assign16110_e22899_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16110_e22901;
        locals.var_pds_ini_dn0 = assign16110_e22901_d_n0;
        locals.var_pds_ini_dn2 = assign16110_e22901_d_n2;
        locals.var_pds_ini_dn6 = assign16110_e22901_d_n6;
        locals.var_pds_ini_dn7 = assign16110_e22901_d_n7;
        locals.var_pds_ini_dn10 = assign16110_e22901_d_n10;
        locals.var_pds_ini_dn11 = assign16110_e22901_d_n11;
        locals.var_pds_ini_dn12 = assign16110_e22901_d_n12;
        locals.var_pds_ini_dn17 = assign16110_e22901_d_n17;

        let assign16120_e22904: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign16120_e22904;

        let (assign16130_e22916, assign16130_e22916_d_n0, assign16130_e22916_d_n2, assign16130_e22916_d_n6, assign16130_e22916_d_n7, assign16130_e22916_d_n10, assign16130_e22916_d_n11, assign16130_e22916_d_n12, assign16130_e22916_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard479 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16130_e22916;
        locals.var_pds_ini_dn0 = assign16130_e22916_d_n0;
        locals.var_pds_ini_dn2 = assign16130_e22916_d_n2;
        locals.var_pds_ini_dn6 = assign16130_e22916_d_n6;
        locals.var_pds_ini_dn7 = assign16130_e22916_d_n7;
        locals.var_pds_ini_dn10 = assign16130_e22916_d_n10;
        locals.var_pds_ini_dn11 = assign16130_e22916_d_n11;
        locals.var_pds_ini_dn12 = assign16130_e22916_d_n12;
        locals.var_pds_ini_dn17 = assign16130_e22916_d_n17;

        let assign16140_e22919: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard480 = assign16140_e22919;

        let (assign16150_e22934, assign16150_e22934_d_n0, assign16150_e22934_d_n2, assign16150_e22934_d_n6, assign16150_e22934_d_n7, assign16150_e22934_d_n10, assign16150_e22934_d_n11, assign16150_e22934_d_n12, assign16150_e22934_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard479 == 0.0)) && (locals.var_guard480 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16150_e22934;
        locals.var_pds_ini_dn0 = assign16150_e22934_d_n0;
        locals.var_pds_ini_dn2 = assign16150_e22934_d_n2;
        locals.var_pds_ini_dn6 = assign16150_e22934_d_n6;
        locals.var_pds_ini_dn7 = assign16150_e22934_d_n7;
        locals.var_pds_ini_dn10 = assign16150_e22934_d_n10;
        locals.var_pds_ini_dn11 = assign16150_e22934_d_n11;
        locals.var_pds_ini_dn12 = assign16150_e22934_d_n12;
        locals.var_pds_ini_dn17 = assign16150_e22934_d_n17;

        let (assign16160_e22944, assign16160_e22944_d_n0, assign16160_e22944_d_n2, assign16160_e22944_d_n6, assign16160_e22944_d_n7, assign16160_e22944_d_n10, assign16160_e22944_d_n11, assign16160_e22944_d_n12, assign16160_e22944_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16160_e22944;
        locals.var_pds_dn0 = assign16160_e22944_d_n0;
        locals.var_pds_dn2 = assign16160_e22944_d_n2;
        locals.var_pds_dn6 = assign16160_e22944_d_n6;
        locals.var_pds_dn7 = assign16160_e22944_d_n7;
        locals.var_pds_dn10 = assign16160_e22944_d_n10;
        locals.var_pds_dn11 = assign16160_e22944_d_n11;
        locals.var_pds_dn12 = assign16160_e22944_d_n12;
        locals.var_pds_dn17 = assign16160_e22944_d_n17;

        let (assign16170_e22956, assign16170_e22956_d_n0, assign16170_e22956_d_n2, assign16170_e22956_d_n6, assign16170_e22956_d_n7, assign16170_e22956_d_n10, assign16170_e22956_d_n11, assign16170_e22956_d_n12, assign16170_e22956_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) {
        let assign16170_e22954: f64 = (locals.var_ps0 + locals.var_pds);
        (assign16170_e22954, (locals.var_ps0_dn0 + locals.var_pds_dn0), (locals.var_ps0_dn2 + locals.var_pds_dn2), (locals.var_ps0_dn6 + locals.var_pds_dn6), (locals.var_ps0_dn7 + locals.var_pds_dn7), (locals.var_ps0_dn10 + locals.var_pds_dn10), (locals.var_ps0_dn11 + locals.var_pds_dn11), (locals.var_ps0_dn12 + locals.var_pds_dn12), (locals.var_ps0_dn17 + locals.var_pds_dn17),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16170_e22956;
        locals.var_psl_dn0 = assign16170_e22956_d_n0;
        locals.var_psl_dn2 = assign16170_e22956_d_n2;
        locals.var_psl_dn6 = assign16170_e22956_d_n6;
        locals.var_psl_dn7 = assign16170_e22956_d_n7;
        locals.var_psl_dn10 = assign16170_e22956_d_n10;
        locals.var_psl_dn11 = assign16170_e22956_d_n11;
        locals.var_psl_dn12 = assign16170_e22956_d_n12;
        locals.var_psl_dn17 = assign16170_e22956_d_n17;

        let (assign16180_e22966,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign16180_e22966;

        let (assign16190_e22973, assign16190_e22973_d_n0, assign16190_e22973_d_n2, assign16190_e22973_d_n6, assign16190_e22973_d_n7, assign16190_e22973_d_n10, assign16190_e22973_d_n11, assign16190_e22973_d_n12, assign16190_e22973_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign16190_e22973;
        locals.var_phi_sl_soi_dn0 = assign16190_e22973_d_n0;
        locals.var_phi_sl_soi_dn2 = assign16190_e22973_d_n2;
        locals.var_phi_sl_soi_dn6 = assign16190_e22973_d_n6;
        locals.var_phi_sl_soi_dn7 = assign16190_e22973_d_n7;
        locals.var_phi_sl_soi_dn10 = assign16190_e22973_d_n10;
        locals.var_phi_sl_soi_dn11 = assign16190_e22973_d_n11;
        locals.var_phi_sl_soi_dn12 = assign16190_e22973_d_n12;
        locals.var_phi_sl_soi_dn17 = assign16190_e22973_d_n17;

        let (assign16200_e22980,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign16200_e22980;

    }

    pub(super) fn stamp_transient_block_54(
        locals: &mut StampLocals,
    ) {
        let mut assign16210_loop_guard: usize = 0;
        while {
            let assign16210_cond_e22988: f64 = (locals.var_lp_sl_max + 1.0);
            let assign16210_cond_e22990: f64 = if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_lp_sl <= assign16210_cond_e22988)) { 1.0 } else { 0.0 };
            assign16210_cond_e22990 != 0.0
        } {
            assign16210_loop_guard += 1;
            assert!(assign16210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign16210_body0_e22999, assign16210_body0_e22999_d_n0, assign16210_body0_e22999_d_n2, assign16210_body0_e22999_d_n6, assign16210_body0_e22999_d_n7, assign16210_body0_e22999_d_n10, assign16210_body0_e22999_d_n11, assign16210_body0_e22999_d_n12, assign16210_body0_e22999_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body0_e22997: f64 = (locals.var_phi_sl_soi - locals.var_vbcs_cl);
        (assign16210_body0_e22997, (locals.var_phi_sl_soi_dn0 - locals.var_vbcs_cl_dn0), (locals.var_phi_sl_soi_dn2 - locals.var_vbcs_cl_dn2), (locals.var_phi_sl_soi_dn6 - locals.var_vbcs_cl_dn6), (locals.var_phi_sl_soi_dn7 - locals.var_vbcs_cl_dn7), (locals.var_phi_sl_soi_dn10 - locals.var_vbcs_cl_dn10), (locals.var_phi_sl_soi_dn11 - locals.var_vbcs_cl_dn11), (locals.var_phi_sl_soi_dn12 - locals.var_vbcs_cl_dn12), (locals.var_phi_sl_soi_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_phi_soil, locals.var_phi_soil_dn0, locals.var_phi_soil_dn2, locals.var_phi_soil_dn6, locals.var_phi_soil_dn7, locals.var_phi_soil_dn10, locals.var_phi_soil_dn11, locals.var_phi_soil_dn12, locals.var_phi_soil_dn17,)
    }
};
            locals.var_phi_soil = assign16210_body0_e22999;
            locals.var_phi_soil_dn0 = assign16210_body0_e22999_d_n0;
            locals.var_phi_soil_dn2 = assign16210_body0_e22999_d_n2;
            locals.var_phi_soil_dn6 = assign16210_body0_e22999_d_n6;
            locals.var_phi_soil_dn7 = assign16210_body0_e22999_d_n7;
            locals.var_phi_soil_dn10 = assign16210_body0_e22999_d_n10;
            locals.var_phi_soil_dn11 = assign16210_body0_e22999_d_n11;
            locals.var_phi_soil_dn12 = assign16210_body0_e22999_d_n12;
            locals.var_phi_soil_dn17 = assign16210_body0_e22999_d_n17;
            let (assign16210_body1_e23008, assign16210_body1_e23008_d_n0, assign16210_body1_e23008_d_n2, assign16210_body1_e23008_d_n6, assign16210_body1_e23008_d_n7, assign16210_body1_e23008_d_n10, assign16210_body1_e23008_d_n11, assign16210_body1_e23008_d_n12, assign16210_body1_e23008_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body1_e23006: f64 = (locals.var_beta * locals.var_phi_soil);
        (assign16210_body1_e23006, (locals.var_beta * locals.var_phi_soil_dn0), (locals.var_beta * locals.var_phi_soil_dn2), (locals.var_beta * locals.var_phi_soil_dn6), (locals.var_beta * locals.var_phi_soil_dn7), ((locals.var_beta_dn10 * locals.var_phi_soil) + (locals.var_beta * locals.var_phi_soil_dn10)), (locals.var_beta * locals.var_phi_soil_dn11), (locals.var_beta * locals.var_phi_soil_dn12), (locals.var_beta * locals.var_phi_soil_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign16210_body1_e23008;
            locals.var_chi_dn0 = assign16210_body1_e23008_d_n0;
            locals.var_chi_dn2 = assign16210_body1_e23008_d_n2;
            locals.var_chi_dn6 = assign16210_body1_e23008_d_n6;
            locals.var_chi_dn7 = assign16210_body1_e23008_d_n7;
            locals.var_chi_dn10 = assign16210_body1_e23008_d_n10;
            locals.var_chi_dn11 = assign16210_body1_e23008_d_n11;
            locals.var_chi_dn12 = assign16210_body1_e23008_d_n12;
            locals.var_chi_dn17 = assign16210_body1_e23008_d_n17;
            let (assign16210_body2_e23019, assign16210_body2_e23019_d_n0, assign16210_body2_e23019_d_n2, assign16210_body2_e23019_d_n6, assign16210_body2_e23019_d_n7, assign16210_body2_e23019_d_n10, assign16210_body2_e23019_d_n11, assign16210_body2_e23019_d_n12, assign16210_body2_e23019_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body2_e23016: f64 = (locals.var_phi_soil - locals.var_dphi_sb);
        let assign16210_body2_e23017: f64 = (locals.var_c_sb * assign16210_body2_e23016);
        (assign16210_body2_e23017, ((locals.var_c_sb_dn0 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign16210_body2_e23019;
            locals.var_ty_dn0 = assign16210_body2_e23019_d_n0;
            locals.var_ty_dn2 = assign16210_body2_e23019_d_n2;
            locals.var_ty_dn6 = assign16210_body2_e23019_d_n6;
            locals.var_ty_dn7 = assign16210_body2_e23019_d_n7;
            locals.var_ty_dn10 = assign16210_body2_e23019_d_n10;
            locals.var_ty_dn11 = assign16210_body2_e23019_d_n11;
            locals.var_ty_dn12 = assign16210_body2_e23019_d_n12;
            locals.var_ty_dn17 = assign16210_body2_e23019_d_n17;
            let assign16210_body3_e23022: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard481 = assign16210_body3_e23022;
            let (assign16210_body4_e23032, assign16210_body4_e23032_d_n0, assign16210_body4_e23032_d_n2, assign16210_body4_e23032_d_n6, assign16210_body4_e23032_d_n7, assign16210_body4_e23032_d_n10, assign16210_body4_e23032_d_n11, assign16210_body4_e23032_d_n12, assign16210_body4_e23032_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body4_e23030: f64 = (locals.var_ty).exp();
        (assign16210_body4_e23030, (assign16210_body4_e23030 * locals.var_ty_dn0), (assign16210_body4_e23030 * locals.var_ty_dn2), (assign16210_body4_e23030 * locals.var_ty_dn6), (assign16210_body4_e23030 * locals.var_ty_dn7), (assign16210_body4_e23030 * locals.var_ty_dn10), (assign16210_body4_e23030 * locals.var_ty_dn11), (assign16210_body4_e23030 * locals.var_ty_dn12), (assign16210_body4_e23030 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16210_body4_e23032;
            locals.var_t1_dn0 = assign16210_body4_e23032_d_n0;
            locals.var_t1_dn2 = assign16210_body4_e23032_d_n2;
            locals.var_t1_dn6 = assign16210_body4_e23032_d_n6;
            locals.var_t1_dn7 = assign16210_body4_e23032_d_n7;
            locals.var_t1_dn10 = assign16210_body4_e23032_d_n10;
            locals.var_t1_dn11 = assign16210_body4_e23032_d_n11;
            locals.var_t1_dn12 = assign16210_body4_e23032_d_n12;
            locals.var_t1_dn17 = assign16210_body4_e23032_d_n17;
            let (assign16210_body5_e23045, assign16210_body5_e23045_d_n0, assign16210_body5_e23045_d_n2, assign16210_body5_e23045_d_n6, assign16210_body5_e23045_d_n7, assign16210_body5_e23045_d_n10, assign16210_body5_e23045_d_n11, assign16210_body5_e23045_d_n12, assign16210_body5_e23045_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body5_e23040: f64 = (-locals.var_c_sb);
        let assign16210_body5_e23042: f64 = (assign16210_body5_e23040 * locals.var_dphi_sb);
        let assign16210_body5_e23043: f64 = (assign16210_body5_e23042).exp();
        (assign16210_body5_e23043, (assign16210_body5_e23043 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn0))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn2))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn6))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn7))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn10))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn11))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn12))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16210_body5_e23045;
            locals.var_t0_dn0 = assign16210_body5_e23045_d_n0;
            locals.var_t0_dn2 = assign16210_body5_e23045_d_n2;
            locals.var_t0_dn6 = assign16210_body5_e23045_d_n6;
            locals.var_t0_dn7 = assign16210_body5_e23045_d_n7;
            locals.var_t0_dn10 = assign16210_body5_e23045_d_n10;
            locals.var_t0_dn11 = assign16210_body5_e23045_d_n11;
            locals.var_t0_dn12 = assign16210_body5_e23045_d_n12;
            locals.var_t0_dn17 = assign16210_body5_e23045_d_n17;
            let (assign16210_body6_e23056, assign16210_body6_e23056_d_n0, assign16210_body6_e23056_d_n2, assign16210_body6_e23056_d_n6, assign16210_body6_e23056_d_n7, assign16210_body6_e23056_d_n10, assign16210_body6_e23056_d_n11, assign16210_body6_e23056_d_n12, assign16210_body6_e23056_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body6_e23054: f64 = (locals.var_t1 - locals.var_t0);
        (assign16210_body6_e23054, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign16210_body6_e23056;
            locals.var_t2_dn0 = assign16210_body6_e23056_d_n0;
            locals.var_t2_dn2 = assign16210_body6_e23056_d_n2;
            locals.var_t2_dn6 = assign16210_body6_e23056_d_n6;
            locals.var_t2_dn7 = assign16210_body6_e23056_d_n7;
            locals.var_t2_dn10 = assign16210_body6_e23056_d_n10;
            locals.var_t2_dn11 = assign16210_body6_e23056_d_n11;
            locals.var_t2_dn12 = assign16210_body6_e23056_d_n12;
            locals.var_t2_dn17 = assign16210_body6_e23056_d_n17;
            let (assign16210_body7_e23070, assign16210_body7_e23070_d_n0, assign16210_body7_e23070_d_n2, assign16210_body7_e23070_d_n6, assign16210_body7_e23070_d_n7, assign16210_body7_e23070_d_n10, assign16210_body7_e23070_d_n11, assign16210_body7_e23070_d_n12, assign16210_body7_e23070_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body7_e23065: f64 = (1.0 + locals.var_t2);
        let assign16210_body7_e23066: f64 = (assign16210_body7_e23065).ln();
        let assign16210_body7_e23068: f64 = (assign16210_body7_e23066 / locals.var_c_sb);
        (assign16210_body7_e23068, ((((locals.var_t2_dn0 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign16210_body7_e23070;
            locals.var_phi_soib_dn0 = assign16210_body7_e23070_d_n0;
            locals.var_phi_soib_dn2 = assign16210_body7_e23070_d_n2;
            locals.var_phi_soib_dn6 = assign16210_body7_e23070_d_n6;
            locals.var_phi_soib_dn7 = assign16210_body7_e23070_d_n7;
            locals.var_phi_soib_dn10 = assign16210_body7_e23070_d_n10;
            locals.var_phi_soib_dn11 = assign16210_body7_e23070_d_n11;
            locals.var_phi_soib_dn12 = assign16210_body7_e23070_d_n12;
            locals.var_phi_soib_dn17 = assign16210_body7_e23070_d_n17;
            let (assign16210_body8_e23083, assign16210_body8_e23083_d_n0, assign16210_body8_e23083_d_n2, assign16210_body8_e23083_d_n6, assign16210_body8_e23083_d_n7, assign16210_body8_e23083_d_n10, assign16210_body8_e23083_d_n11, assign16210_body8_e23083_d_n12, assign16210_body8_e23083_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body8_e23080: f64 = (1.0 + locals.var_t2);
        let assign16210_body8_e23081: f64 = (locals.var_t1 / assign16210_body8_e23080);
        (assign16210_body8_e23081, (((locals.var_t1_dn0 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn0)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn2 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn2)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn6 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn6)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn7 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn7)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn10 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn10)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn11 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn11)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn12 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn12)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn17 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn17)) / (assign16210_body8_e23080 * assign16210_body8_e23080)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign16210_body8_e23083;
            locals.var_phi_soib_dpss_dn0 = assign16210_body8_e23083_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign16210_body8_e23083_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign16210_body8_e23083_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign16210_body8_e23083_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign16210_body8_e23083_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign16210_body8_e23083_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign16210_body8_e23083_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign16210_body8_e23083_d_n17;
            let (assign16210_body9_e23095, assign16210_body9_e23095_d_n0, assign16210_body9_e23095_d_n2, assign16210_body9_e23095_d_n6, assign16210_body9_e23095_d_n7, assign16210_body9_e23095_d_n10, assign16210_body9_e23095_d_n11, assign16210_body9_e23095_d_n12, assign16210_body9_e23095_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 == 0.0)) {
        let assign16210_body9_e23093: f64 = (locals.var_phi_soil - locals.var_dphi_sb);
        (assign16210_body9_e23093, (locals.var_phi_soil_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soil_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soil_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soil_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soil_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soil_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soil_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soil_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign16210_body9_e23095;
            locals.var_phi_soib_dn0 = assign16210_body9_e23095_d_n0;
            locals.var_phi_soib_dn2 = assign16210_body9_e23095_d_n2;
            locals.var_phi_soib_dn6 = assign16210_body9_e23095_d_n6;
            locals.var_phi_soib_dn7 = assign16210_body9_e23095_d_n7;
            locals.var_phi_soib_dn10 = assign16210_body9_e23095_d_n10;
            locals.var_phi_soib_dn11 = assign16210_body9_e23095_d_n11;
            locals.var_phi_soib_dn12 = assign16210_body9_e23095_d_n12;
            locals.var_phi_soib_dn17 = assign16210_body9_e23095_d_n17;
            let (assign16210_body10_e23105, assign16210_body10_e23105_d_n0, assign16210_body10_e23105_d_n2, assign16210_body10_e23105_d_n6, assign16210_body10_e23105_d_n7, assign16210_body10_e23105_d_n10, assign16210_body10_e23105_d_n11, assign16210_body10_e23105_d_n12, assign16210_body10_e23105_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign16210_body10_e23105;
            locals.var_phi_soib_dpss_dn0 = assign16210_body10_e23105_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign16210_body10_e23105_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign16210_body10_e23105_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign16210_body10_e23105_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign16210_body10_e23105_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign16210_body10_e23105_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign16210_body10_e23105_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign16210_body10_e23105_d_n17;
            let (assign16210_body11_e23114, assign16210_body11_e23114_d_n0, assign16210_body11_e23114_d_n2, assign16210_body11_e23114_d_n6, assign16210_body11_e23114_d_n7, assign16210_body11_e23114_d_n10, assign16210_body11_e23114_d_n11, assign16210_body11_e23114_d_n12, assign16210_body11_e23114_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body11_e23112: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign16210_body11_e23112, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign16210_body11_e23114;
            locals.var_chib_dn0 = assign16210_body11_e23114_d_n0;
            locals.var_chib_dn2 = assign16210_body11_e23114_d_n2;
            locals.var_chib_dn6 = assign16210_body11_e23114_d_n6;
            locals.var_chib_dn7 = assign16210_body11_e23114_d_n7;
            locals.var_chib_dn10 = assign16210_body11_e23114_d_n10;
            locals.var_chib_dn11 = assign16210_body11_e23114_d_n11;
            locals.var_chib_dn12 = assign16210_body11_e23114_d_n12;
            locals.var_chib_dn17 = assign16210_body11_e23114_d_n17;
            let assign16210_body12_e23116: f64 = (locals.var_chi).abs();
            let assign16210_body12_e23118: f64 = if assign16210_body12_e23116 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard482 = assign16210_body12_e23118;
            let (assign16210_body13_e23134, assign16210_body13_e23134_d_n0, assign16210_body13_e23134_d_n2, assign16210_body13_e23134_d_n6, assign16210_body13_e23134_d_n7, assign16210_body13_e23134_d_n10, assign16210_body13_e23134_d_n11, assign16210_body13_e23134_d_n12, assign16210_body13_e23134_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16210_body13_e23128: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign16210_body13_e23129: f64 = (1.0 - assign16210_body13_e23128);
        let assign16210_body13_e23131: f64 = (assign16210_body13_e23129 / 2.0);
        let assign16210_body13_e23132: f64 = (assign16210_body13_e23131).sqrt();
        (assign16210_body13_e23132, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign16210_body13_e23132)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16210_body13_e23134;
            locals.var_t0_dn0 = assign16210_body13_e23134_d_n0;
            locals.var_t0_dn2 = assign16210_body13_e23134_d_n2;
            locals.var_t0_dn6 = assign16210_body13_e23134_d_n6;
            locals.var_t0_dn7 = assign16210_body13_e23134_d_n7;
            locals.var_t0_dn10 = assign16210_body13_e23134_d_n10;
            locals.var_t0_dn11 = assign16210_body13_e23134_d_n11;
            locals.var_t0_dn12 = assign16210_body13_e23134_d_n12;
            locals.var_t0_dn17 = assign16210_body13_e23134_d_n17;
            let (assign16210_body14_e23145, assign16210_body14_e23145_d_n0, assign16210_body14_e23145_d_n2, assign16210_body14_e23145_d_n6, assign16210_body14_e23145_d_n7, assign16210_body14_e23145_d_n10, assign16210_body14_e23145_d_n11, assign16210_body14_e23145_d_n12, assign16210_body14_e23145_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16210_body14_e23143: f64 = (locals.var_chi * locals.var_t0);
        (assign16210_body14_e23143, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16210_body14_e23145;
            locals.var_fb_dn0 = assign16210_body14_e23145_d_n0;
            locals.var_fb_dn2 = assign16210_body14_e23145_d_n2;
            locals.var_fb_dn6 = assign16210_body14_e23145_d_n6;
            locals.var_fb_dn7 = assign16210_body14_e23145_d_n7;
            locals.var_fb_dn10 = assign16210_body14_e23145_d_n10;
            locals.var_fb_dn11 = assign16210_body14_e23145_d_n11;
            locals.var_fb_dn12 = assign16210_body14_e23145_d_n12;
            locals.var_fb_dn17 = assign16210_body14_e23145_d_n17;
            let (assign16210_body15_e23156, assign16210_body15_e23156_d_n0, assign16210_body15_e23156_d_n2, assign16210_body15_e23156_d_n6, assign16210_body15_e23156_d_n7, assign16210_body15_e23156_d_n10, assign16210_body15_e23156_d_n11, assign16210_body15_e23156_d_n12, assign16210_body15_e23156_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16210_body15_e23154: f64 = (locals.var_beta * locals.var_t0);
        (assign16210_body15_e23154, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16210_body15_e23156;
            locals.var_fb_dpss_dn0 = assign16210_body15_e23156_d_n0;
            locals.var_fb_dpss_dn2 = assign16210_body15_e23156_d_n2;
            locals.var_fb_dpss_dn6 = assign16210_body15_e23156_d_n6;
            locals.var_fb_dpss_dn7 = assign16210_body15_e23156_d_n7;
            locals.var_fb_dpss_dn10 = assign16210_body15_e23156_d_n10;
            locals.var_fb_dpss_dn11 = assign16210_body15_e23156_d_n11;
            locals.var_fb_dpss_dn12 = assign16210_body15_e23156_d_n12;
            locals.var_fb_dpss_dn17 = assign16210_body15_e23156_d_n17;
            let assign16210_body16_e23159: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard483 = assign16210_body16_e23159;
            let (assign16210_body17_e23171, assign16210_body17_e23171_d_n0, assign16210_body17_e23171_d_n2, assign16210_body17_e23171_d_n6, assign16210_body17_e23171_d_n7, assign16210_body17_e23171_d_n10, assign16210_body17_e23171_d_n11, assign16210_body17_e23171_d_n12, assign16210_body17_e23171_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16210_body17_e23169: f64 = (-locals.var_fb);
        (assign16210_body17_e23169, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16210_body17_e23171;
            locals.var_fb_dn0 = assign16210_body17_e23171_d_n0;
            locals.var_fb_dn2 = assign16210_body17_e23171_d_n2;
            locals.var_fb_dn6 = assign16210_body17_e23171_d_n6;
            locals.var_fb_dn7 = assign16210_body17_e23171_d_n7;
            locals.var_fb_dn10 = assign16210_body17_e23171_d_n10;
            locals.var_fb_dn11 = assign16210_body17_e23171_d_n11;
            locals.var_fb_dn12 = assign16210_body17_e23171_d_n12;
            locals.var_fb_dn17 = assign16210_body17_e23171_d_n17;
            let (assign16210_body18_e23183, assign16210_body18_e23183_d_n0, assign16210_body18_e23183_d_n2, assign16210_body18_e23183_d_n6, assign16210_body18_e23183_d_n7, assign16210_body18_e23183_d_n10, assign16210_body18_e23183_d_n11, assign16210_body18_e23183_d_n12, assign16210_body18_e23183_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16210_body18_e23181: f64 = (-locals.var_fb_dpss);
        (assign16210_body18_e23181, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16210_body18_e23183;
            locals.var_fb_dpss_dn0 = assign16210_body18_e23183_d_n0;
            locals.var_fb_dpss_dn2 = assign16210_body18_e23183_d_n2;
            locals.var_fb_dpss_dn6 = assign16210_body18_e23183_d_n6;
            locals.var_fb_dpss_dn7 = assign16210_body18_e23183_d_n7;
            locals.var_fb_dpss_dn10 = assign16210_body18_e23183_d_n10;
            locals.var_fb_dpss_dn11 = assign16210_body18_e23183_d_n11;
            locals.var_fb_dpss_dn12 = assign16210_body18_e23183_d_n12;
            locals.var_fb_dpss_dn17 = assign16210_body18_e23183_d_n17;
            let assign16210_body19_e23185: f64 = (locals.var_chi).abs();
            let assign16210_body19_e23187: f64 = if assign16210_body19_e23185 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard484 = assign16210_body19_e23187;
            let (assign16210_body20_e23221, assign16210_body20_e23221_d_n0, assign16210_body20_e23221_d_n2, assign16210_body20_e23221_d_n6, assign16210_body20_e23221_d_n7, assign16210_body20_e23221_d_n10, assign16210_body20_e23221_d_n11, assign16210_body20_e23221_d_n12, assign16210_body20_e23221_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body20_e23199: f64 = (locals.var_chi * locals.var_chi);
        let assign16210_body20_e23201: f64 = (assign16210_body20_e23199 / 2.0);
        let assign16210_body20_e23205: f64 = (locals.var_chi / 3.0);
        let assign16210_body20_e23209: f64 = (locals.var_chi / 4.0);
        let assign16210_body20_e23213: f64 = (locals.var_chi / 5.0);
        let assign16210_body20_e23214: f64 = (1.0 - assign16210_body20_e23213);
        let assign16210_body20_e23215: f64 = (assign16210_body20_e23209 * assign16210_body20_e23214);
        let assign16210_body20_e23216: f64 = (1.0 - assign16210_body20_e23215);
        let assign16210_body20_e23217: f64 = (assign16210_body20_e23205 * assign16210_body20_e23216);
        let assign16210_body20_e23218: f64 = (1.0 - assign16210_body20_e23217);
        let assign16210_body20_e23219: f64 = (assign16210_body20_e23201 * assign16210_body20_e23218);
        (assign16210_body20_e23219, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn0 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn0 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn2 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn2 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn6 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn6 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn7 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn7 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn10 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn10 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn11 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn11 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn12 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn12 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn17 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn17 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16210_body20_e23221;
            locals.var_t0_dn0 = assign16210_body20_e23221_d_n0;
            locals.var_t0_dn2 = assign16210_body20_e23221_d_n2;
            locals.var_t0_dn6 = assign16210_body20_e23221_d_n6;
            locals.var_t0_dn7 = assign16210_body20_e23221_d_n7;
            locals.var_t0_dn10 = assign16210_body20_e23221_d_n10;
            locals.var_t0_dn11 = assign16210_body20_e23221_d_n11;
            locals.var_t0_dn12 = assign16210_body20_e23221_d_n12;
            locals.var_t0_dn17 = assign16210_body20_e23221_d_n17;
            let (assign16210_body21_e23251, assign16210_body21_e23251_d_n0, assign16210_body21_e23251_d_n2, assign16210_body21_e23251_d_n6, assign16210_body21_e23251_d_n7, assign16210_body21_e23251_d_n10, assign16210_body21_e23251_d_n11, assign16210_body21_e23251_d_n12, assign16210_body21_e23251_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body21_e23235: f64 = (locals.var_chi / 2.0);
        let assign16210_body21_e23239: f64 = (locals.var_chi / 3.0);
        let assign16210_body21_e23243: f64 = (locals.var_chi / 4.0);
        let assign16210_body21_e23244: f64 = (1.0 - assign16210_body21_e23243);
        let assign16210_body21_e23245: f64 = (assign16210_body21_e23239 * assign16210_body21_e23244);
        let assign16210_body21_e23246: f64 = (1.0 - assign16210_body21_e23245);
        let assign16210_body21_e23247: f64 = (assign16210_body21_e23235 * assign16210_body21_e23246);
        let assign16210_body21_e23248: f64 = (1.0 - assign16210_body21_e23247);
        let assign16210_body21_e23249: f64 = (locals.var_chi * assign16210_body21_e23248);
        (assign16210_body21_e23249, ((locals.var_chi_dn0 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn0 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn2 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn6 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn7 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn10 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn11 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn12 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn17 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16210_body21_e23251;
            locals.var_t1_dn0 = assign16210_body21_e23251_d_n0;
            locals.var_t1_dn2 = assign16210_body21_e23251_d_n2;
            locals.var_t1_dn6 = assign16210_body21_e23251_d_n6;
            locals.var_t1_dn7 = assign16210_body21_e23251_d_n7;
            locals.var_t1_dn10 = assign16210_body21_e23251_d_n10;
            locals.var_t1_dn11 = assign16210_body21_e23251_d_n11;
            locals.var_t1_dn12 = assign16210_body21_e23251_d_n12;
            locals.var_t1_dn17 = assign16210_body21_e23251_d_n17;
            let (assign16210_body22_e23285, assign16210_body22_e23285_d_n0, assign16210_body22_e23285_d_n2, assign16210_body22_e23285_d_n6, assign16210_body22_e23285_d_n7, assign16210_body22_e23285_d_n10, assign16210_body22_e23285_d_n11, assign16210_body22_e23285_d_n12, assign16210_body22_e23285_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body22_e23263: f64 = (locals.var_chib * locals.var_chib);
        let assign16210_body22_e23265: f64 = (assign16210_body22_e23263 / 2.0);
        let assign16210_body22_e23269: f64 = (locals.var_chib / 3.0);
        let assign16210_body22_e23273: f64 = (locals.var_chib / 4.0);
        let assign16210_body22_e23277: f64 = (locals.var_chib / 5.0);
        let assign16210_body22_e23278: f64 = (1.0 - assign16210_body22_e23277);
        let assign16210_body22_e23279: f64 = (assign16210_body22_e23273 * assign16210_body22_e23278);
        let assign16210_body22_e23280: f64 = (1.0 - assign16210_body22_e23279);
        let assign16210_body22_e23281: f64 = (assign16210_body22_e23269 * assign16210_body22_e23280);
        let assign16210_body22_e23282: f64 = (1.0 - assign16210_body22_e23281);
        let assign16210_body22_e23283: f64 = (assign16210_body22_e23265 * assign16210_body22_e23282);
        (assign16210_body22_e23283, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn0 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn0 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn2 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn2 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn6 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn6 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn7 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn7 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn10 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn10 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn11 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn11 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn12 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn12 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn17 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn17 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign16210_body22_e23285;
            locals.var_t2_dn0 = assign16210_body22_e23285_d_n0;
            locals.var_t2_dn2 = assign16210_body22_e23285_d_n2;
            locals.var_t2_dn6 = assign16210_body22_e23285_d_n6;
            locals.var_t2_dn7 = assign16210_body22_e23285_d_n7;
            locals.var_t2_dn10 = assign16210_body22_e23285_d_n10;
            locals.var_t2_dn11 = assign16210_body22_e23285_d_n11;
            locals.var_t2_dn12 = assign16210_body22_e23285_d_n12;
            locals.var_t2_dn17 = assign16210_body22_e23285_d_n17;
            let (assign16210_body23_e23315, assign16210_body23_e23315_d_n0, assign16210_body23_e23315_d_n2, assign16210_body23_e23315_d_n6, assign16210_body23_e23315_d_n7, assign16210_body23_e23315_d_n10, assign16210_body23_e23315_d_n11, assign16210_body23_e23315_d_n12, assign16210_body23_e23315_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body23_e23299: f64 = (locals.var_chib / 2.0);
        let assign16210_body23_e23303: f64 = (locals.var_chib / 3.0);
        let assign16210_body23_e23307: f64 = (locals.var_chib / 4.0);
        let assign16210_body23_e23308: f64 = (1.0 - assign16210_body23_e23307);
        let assign16210_body23_e23309: f64 = (assign16210_body23_e23303 * assign16210_body23_e23308);
        let assign16210_body23_e23310: f64 = (1.0 - assign16210_body23_e23309);
        let assign16210_body23_e23311: f64 = (assign16210_body23_e23299 * assign16210_body23_e23310);
        let assign16210_body23_e23312: f64 = (1.0 - assign16210_body23_e23311);
        let assign16210_body23_e23313: f64 = (locals.var_chib * assign16210_body23_e23312);
        (assign16210_body23_e23313, ((locals.var_chib_dn0 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn0 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn2 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn6 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn7 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn10 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn11 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn12 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn17 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign16210_body23_e23315;
            locals.var_t3_dn0 = assign16210_body23_e23315_d_n0;
            locals.var_t3_dn2 = assign16210_body23_e23315_d_n2;
            locals.var_t3_dn6 = assign16210_body23_e23315_d_n6;
            locals.var_t3_dn7 = assign16210_body23_e23315_d_n7;
            locals.var_t3_dn10 = assign16210_body23_e23315_d_n10;
            locals.var_t3_dn11 = assign16210_body23_e23315_d_n11;
            locals.var_t3_dn12 = assign16210_body23_e23315_d_n12;
            locals.var_t3_dn17 = assign16210_body23_e23315_d_n17;
            let (assign16210_body24_e23330, assign16210_body24_e23330_d_n0, assign16210_body24_e23330_d_n2, assign16210_body24_e23330_d_n6, assign16210_body24_e23330_d_n7, assign16210_body24_e23330_d_n10, assign16210_body24_e23330_d_n11, assign16210_body24_e23330_d_n12, assign16210_body24_e23330_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body24_e23327: f64 = (locals.var_t0 - locals.var_t2);
        let assign16210_body24_e23328: f64 = (assign16210_body24_e23327).sqrt();
        (assign16210_body24_e23328, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign16210_body24_e23328)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16210_body24_e23330;
            locals.var_fb_dn0 = assign16210_body24_e23330_d_n0;
            locals.var_fb_dn2 = assign16210_body24_e23330_d_n2;
            locals.var_fb_dn6 = assign16210_body24_e23330_d_n6;
            locals.var_fb_dn7 = assign16210_body24_e23330_d_n7;
            locals.var_fb_dn10 = assign16210_body24_e23330_d_n10;
            locals.var_fb_dn11 = assign16210_body24_e23330_d_n11;
            locals.var_fb_dn12 = assign16210_body24_e23330_d_n12;
            locals.var_fb_dn17 = assign16210_body24_e23330_d_n17;
            let (assign16210_body25_e23352, assign16210_body25_e23352_d_n0, assign16210_body25_e23352_d_n2, assign16210_body25_e23352_d_n6, assign16210_body25_e23352_d_n7, assign16210_body25_e23352_d_n10, assign16210_body25_e23352_d_n11, assign16210_body25_e23352_d_n12, assign16210_body25_e23352_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body25_e23342: f64 = (locals.var_beta * 0.5);
        let assign16210_body25_e23346: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign16210_body25_e23347: f64 = (locals.var_t1 - assign16210_body25_e23346);
        let assign16210_body25_e23348: f64 = (assign16210_body25_e23342 * assign16210_body25_e23347);
        let assign16210_body25_e23350: f64 = (assign16210_body25_e23348 / locals.var_fb);
        (assign16210_body25_e23350, ((((assign16210_body25_e23342 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign16210_body25_e23347) + (assign16210_body25_e23342 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16210_body25_e23352;
            locals.var_fb_dpss_dn0 = assign16210_body25_e23352_d_n0;
            locals.var_fb_dpss_dn2 = assign16210_body25_e23352_d_n2;
            locals.var_fb_dpss_dn6 = assign16210_body25_e23352_d_n6;
            locals.var_fb_dpss_dn7 = assign16210_body25_e23352_d_n7;
            locals.var_fb_dpss_dn10 = assign16210_body25_e23352_d_n10;
            locals.var_fb_dpss_dn11 = assign16210_body25_e23352_d_n11;
            locals.var_fb_dpss_dn12 = assign16210_body25_e23352_d_n12;
            locals.var_fb_dpss_dn17 = assign16210_body25_e23352_d_n17;
            let (assign16210_body26_e23367, assign16210_body26_e23367_d_n0, assign16210_body26_e23367_d_n2, assign16210_body26_e23367_d_n6, assign16210_body26_e23367_d_n7, assign16210_body26_e23367_d_n10, assign16210_body26_e23367_d_n11, assign16210_body26_e23367_d_n12, assign16210_body26_e23367_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 == 0.0)) {
        let assign16210_body26_e23364: f64 = (-locals.var_chi);
        let assign16210_body26_e23365: f64 = (assign16210_body26_e23364).exp();
        (assign16210_body26_e23365, (assign16210_body26_e23365 * (-locals.var_chi_dn0)), (assign16210_body26_e23365 * (-locals.var_chi_dn2)), (assign16210_body26_e23365 * (-locals.var_chi_dn6)), (assign16210_body26_e23365 * (-locals.var_chi_dn7)), (assign16210_body26_e23365 * (-locals.var_chi_dn10)), (assign16210_body26_e23365 * (-locals.var_chi_dn11)), (assign16210_body26_e23365 * (-locals.var_chi_dn12)), (assign16210_body26_e23365 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16210_body26_e23367;
            locals.var_t0_dn0 = assign16210_body26_e23367_d_n0;
            locals.var_t0_dn2 = assign16210_body26_e23367_d_n2;
            locals.var_t0_dn6 = assign16210_body26_e23367_d_n6;
            locals.var_t0_dn7 = assign16210_body26_e23367_d_n7;
            locals.var_t0_dn10 = assign16210_body26_e23367_d_n10;
            locals.var_t0_dn11 = assign16210_body26_e23367_d_n11;
            locals.var_t0_dn12 = assign16210_body26_e23367_d_n12;
            locals.var_t0_dn17 = assign16210_body26_e23367_d_n17;
            let (assign16210_body27_e23382, assign16210_body27_e23382_d_n0, assign16210_body27_e23382_d_n2, assign16210_body27_e23382_d_n6, assign16210_body27_e23382_d_n7, assign16210_body27_e23382_d_n10, assign16210_body27_e23382_d_n11, assign16210_body27_e23382_d_n12, assign16210_body27_e23382_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 == 0.0)) {
        let assign16210_body27_e23379: f64 = (-locals.var_chib);
        let assign16210_body27_e23380: f64 = (assign16210_body27_e23379).exp();
        (assign16210_body27_e23380, (assign16210_body27_e23380 * (-locals.var_chib_dn0)), (assign16210_body27_e23380 * (-locals.var_chib_dn2)), (assign16210_body27_e23380 * (-locals.var_chib_dn6)), (assign16210_body27_e23380 * (-locals.var_chib_dn7)), (assign16210_body27_e23380 * (-locals.var_chib_dn10)), (assign16210_body27_e23380 * (-locals.var_chib_dn11)), (assign16210_body27_e23380 * (-locals.var_chib_dn12)), (assign16210_body27_e23380 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16210_body27_e23382;
            locals.var_t1_dn0 = assign16210_body27_e23382_d_n0;
            locals.var_t1_dn2 = assign16210_body27_e23382_d_n2;
            locals.var_t1_dn6 = assign16210_body27_e23382_d_n6;
            locals.var_t1_dn7 = assign16210_body27_e23382_d_n7;
            locals.var_t1_dn10 = assign16210_body27_e23382_d_n10;
            locals.var_t1_dn11 = assign16210_body27_e23382_d_n11;
            locals.var_t1_dn12 = assign16210_body27_e23382_d_n12;
            locals.var_t1_dn17 = assign16210_body27_e23382_d_n17;
            let (assign16210_body28_e23402, assign16210_body28_e23402_d_n0, assign16210_body28_e23402_d_n2, assign16210_body28_e23402_d_n6, assign16210_body28_e23402_d_n7, assign16210_body28_e23402_d_n10, assign16210_body28_e23402_d_n11, assign16210_body28_e23402_d_n12, assign16210_body28_e23402_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 == 0.0)) {
        let assign16210_body28_e23395: f64 = (locals.var_chi - locals.var_chib);
        let assign16210_body28_e23398: f64 = (locals.var_t0 - locals.var_t1);
        let assign16210_body28_e23399: f64 = (assign16210_body28_e23395 + assign16210_body28_e23398);
        let assign16210_body28_e23400: f64 = (assign16210_body28_e23399).sqrt();
        (assign16210_body28_e23400, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign16210_body28_e23400)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16210_body28_e23402;
            locals.var_fb_dn0 = assign16210_body28_e23402_d_n0;
            locals.var_fb_dn2 = assign16210_body28_e23402_d_n2;
            locals.var_fb_dn6 = assign16210_body28_e23402_d_n6;
            locals.var_fb_dn7 = assign16210_body28_e23402_d_n7;
            locals.var_fb_dn10 = assign16210_body28_e23402_d_n10;
            locals.var_fb_dn11 = assign16210_body28_e23402_d_n11;
            locals.var_fb_dn12 = assign16210_body28_e23402_d_n12;
            locals.var_fb_dn17 = assign16210_body28_e23402_d_n17;
            let (assign16210_body29_e23429, assign16210_body29_e23429_d_n0, assign16210_body29_e23429_d_n2, assign16210_body29_e23429_d_n6, assign16210_body29_e23429_d_n7, assign16210_body29_e23429_d_n10, assign16210_body29_e23429_d_n11, assign16210_body29_e23429_d_n12, assign16210_body29_e23429_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 == 0.0)) {
        let assign16210_body29_e23415: f64 = (locals.var_beta * 0.5);
        let assign16210_body29_e23418: f64 = (1.0 - locals.var_t0);
        let assign16210_body29_e23422: f64 = (1.0 - locals.var_t1);
        let assign16210_body29_e23423: f64 = (locals.var_phi_soib_dpss * assign16210_body29_e23422);
        let assign16210_body29_e23424: f64 = (assign16210_body29_e23418 - assign16210_body29_e23423);
        let assign16210_body29_e23425: f64 = (assign16210_body29_e23415 * assign16210_body29_e23424);
        let assign16210_body29_e23427: f64 = (assign16210_body29_e23425 / locals.var_fb);
        (assign16210_body29_e23427, ((((assign16210_body29_e23415 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign16210_body29_e23424) + (assign16210_body29_e23415 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16210_body29_e23429;
            locals.var_fb_dpss_dn0 = assign16210_body29_e23429_d_n0;
            locals.var_fb_dpss_dn2 = assign16210_body29_e23429_d_n2;
            locals.var_fb_dpss_dn6 = assign16210_body29_e23429_d_n6;
            locals.var_fb_dpss_dn7 = assign16210_body29_e23429_d_n7;
            locals.var_fb_dpss_dn10 = assign16210_body29_e23429_d_n10;
            locals.var_fb_dpss_dn11 = assign16210_body29_e23429_d_n11;
            locals.var_fb_dpss_dn12 = assign16210_body29_e23429_d_n12;
            locals.var_fb_dpss_dn17 = assign16210_body29_e23429_d_n17;
            let assign16210_body30_e23432: f64 = (-1.0);
            let assign16210_body30_e23433: f64 = if locals.var_flg_zone == assign16210_body30_e23432 { 1.0 } else { 0.0 };
            locals.var_guard485 = assign16210_body30_e23433;
            let (assign16210_body31_e23442, assign16210_body31_e23442_d_n0, assign16210_body31_e23442_d_n2, assign16210_body31_e23442_d_n6, assign16210_body31_e23442_d_n7, assign16210_body31_e23442_d_n10, assign16210_body31_e23442_d_n11, assign16210_body31_e23442_d_n12, assign16210_body31_e23442_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard485 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign16210_body31_e23442;
            locals.var_wdsoi_dn0 = assign16210_body31_e23442_d_n0;
            locals.var_wdsoi_dn2 = assign16210_body31_e23442_d_n2;
            locals.var_wdsoi_dn6 = assign16210_body31_e23442_d_n6;
            locals.var_wdsoi_dn7 = assign16210_body31_e23442_d_n7;
            locals.var_wdsoi_dn10 = assign16210_body31_e23442_d_n10;
            locals.var_wdsoi_dn11 = assign16210_body31_e23442_d_n11;
            locals.var_wdsoi_dn12 = assign16210_body31_e23442_d_n12;
            locals.var_wdsoi_dn17 = assign16210_body31_e23442_d_n17;
            let (assign16210_body32_e23454, assign16210_body32_e23454_d_n0, assign16210_body32_e23454_d_n2, assign16210_body32_e23454_d_n6, assign16210_body32_e23454_d_n7, assign16210_body32_e23454_d_n10, assign16210_body32_e23454_d_n11, assign16210_body32_e23454_d_n12, assign16210_body32_e23454_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard485 == 0.0)) {
        let assign16210_body32_e23452: f64 = (locals.var_c_w_soi * locals.var_fb);
        (assign16210_body32_e23452, ((locals.var_c_w_soi_dn0 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn0)), ((locals.var_c_w_soi_dn2 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn2)), ((locals.var_c_w_soi_dn6 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn6)), ((locals.var_c_w_soi_dn7 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn7)), ((locals.var_c_w_soi_dn10 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn10)), ((locals.var_c_w_soi_dn11 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn11)), ((locals.var_c_w_soi_dn12 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn12)), ((locals.var_c_w_soi_dn17 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn17)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign16210_body32_e23454;
            locals.var_wdsoi_dn0 = assign16210_body32_e23454_d_n0;
            locals.var_wdsoi_dn2 = assign16210_body32_e23454_d_n2;
            locals.var_wdsoi_dn6 = assign16210_body32_e23454_d_n6;
            locals.var_wdsoi_dn7 = assign16210_body32_e23454_d_n7;
            locals.var_wdsoi_dn10 = assign16210_body32_e23454_d_n10;
            locals.var_wdsoi_dn11 = assign16210_body32_e23454_d_n11;
            locals.var_wdsoi_dn12 = assign16210_body32_e23454_d_n12;
            locals.var_wdsoi_dn17 = assign16210_body32_e23454_d_n17;
            let (assign16210_body33_e23463, assign16210_body33_e23463_d_n0, assign16210_body33_e23463_d_n2, assign16210_body33_e23463_d_n6, assign16210_body33_e23463_d_n7, assign16210_body33_e23463_d_n10, assign16210_body33_e23463_d_n11, assign16210_body33_e23463_d_n12, assign16210_body33_e23463_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body33_e23461: f64 = (locals.var_q_nsub * locals.var_wdsoi);
        (assign16210_body33_e23461, ((locals.var_q_nsub_dn0 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn0)), ((locals.var_q_nsub_dn2 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn2)), ((locals.var_q_nsub_dn6 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn6)), ((locals.var_q_nsub_dn7 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn7)), ((locals.var_q_nsub_dn10 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn10)), ((locals.var_q_nsub_dn11 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn11)), ((locals.var_q_nsub_dn12 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn12)), ((locals.var_q_nsub_dn17 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn17)),)
    } else {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    }
};
            locals.var_q_dep_soi = assign16210_body33_e23463;
            locals.var_q_dep_soi_dn0 = assign16210_body33_e23463_d_n0;
            locals.var_q_dep_soi_dn2 = assign16210_body33_e23463_d_n2;
            locals.var_q_dep_soi_dn6 = assign16210_body33_e23463_d_n6;
            locals.var_q_dep_soi_dn7 = assign16210_body33_e23463_d_n7;
            locals.var_q_dep_soi_dn10 = assign16210_body33_e23463_d_n10;
            locals.var_q_dep_soi_dn11 = assign16210_body33_e23463_d_n11;
            locals.var_q_dep_soi_dn12 = assign16210_body33_e23463_d_n12;
            locals.var_q_dep_soi_dn17 = assign16210_body33_e23463_d_n17;
            let assign16210_body34_e23466: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard486 = assign16210_body34_e23466;
            let (assign16210_body35_e23476, assign16210_body35_e23476_d_n0, assign16210_body35_e23476_d_n2, assign16210_body35_e23476_d_n6, assign16210_body35_e23476_d_n7, assign16210_body35_e23476_d_n10, assign16210_body35_e23476_d_n11, assign16210_body35_e23476_d_n12, assign16210_body35_e23476_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16210_body35_e23474: f64 = (-locals.var_fb);
        (assign16210_body35_e23474, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16210_body35_e23476;
            locals.var_fsl2_dn0 = assign16210_body35_e23476_d_n0;
            locals.var_fsl2_dn2 = assign16210_body35_e23476_d_n2;
            locals.var_fsl2_dn6 = assign16210_body35_e23476_d_n6;
            locals.var_fsl2_dn7 = assign16210_body35_e23476_d_n7;
            locals.var_fsl2_dn10 = assign16210_body35_e23476_d_n10;
            locals.var_fsl2_dn11 = assign16210_body35_e23476_d_n11;
            locals.var_fsl2_dn12 = assign16210_body35_e23476_d_n12;
            locals.var_fsl2_dn17 = assign16210_body35_e23476_d_n17;
            let (assign16210_body36_e23486, assign16210_body36_e23486_d_n0, assign16210_body36_e23486_d_n2, assign16210_body36_e23486_d_n6, assign16210_body36_e23486_d_n7, assign16210_body36_e23486_d_n10, assign16210_body36_e23486_d_n11, assign16210_body36_e23486_d_n12, assign16210_body36_e23486_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16210_body36_e23484: f64 = (-locals.var_fb_dpss);
        (assign16210_body36_e23484, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16210_body36_e23486;
            locals.var_fsl2_dpsl_dn0 = assign16210_body36_e23486_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16210_body36_e23486_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16210_body36_e23486_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16210_body36_e23486_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16210_body36_e23486_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16210_body36_e23486_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16210_body36_e23486_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16210_body36_e23486_d_n17;
            let assign16210_body37_e23489: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard487 = assign16210_body37_e23489;
            let (assign16210_body38_e23501, assign16210_body38_e23501_d_n0, assign16210_body38_e23501_d_n2, assign16210_body38_e23501_d_n6, assign16210_body38_e23501_d_n7, assign16210_body38_e23501_d_n10, assign16210_body38_e23501_d_n11, assign16210_body38_e23501_d_n12, assign16210_body38_e23501_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16210_body38_e23501;
            locals.var_fsl2_dn0 = assign16210_body38_e23501_d_n0;
            locals.var_fsl2_dn2 = assign16210_body38_e23501_d_n2;
            locals.var_fsl2_dn6 = assign16210_body38_e23501_d_n6;
            locals.var_fsl2_dn7 = assign16210_body38_e23501_d_n7;
            locals.var_fsl2_dn10 = assign16210_body38_e23501_d_n10;
            locals.var_fsl2_dn11 = assign16210_body38_e23501_d_n11;
            locals.var_fsl2_dn12 = assign16210_body38_e23501_d_n12;
            locals.var_fsl2_dn17 = assign16210_body38_e23501_d_n17;
            let (assign16210_body39_e23513, assign16210_body39_e23513_d_n0, assign16210_body39_e23513_d_n2, assign16210_body39_e23513_d_n6, assign16210_body39_e23513_d_n7, assign16210_body39_e23513_d_n10, assign16210_body39_e23513_d_n11, assign16210_body39_e23513_d_n12, assign16210_body39_e23513_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16210_body39_e23513;
            locals.var_fsl2_dpsl_dn0 = assign16210_body39_e23513_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16210_body39_e23513_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16210_body39_e23513_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16210_body39_e23513_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16210_body39_e23513_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16210_body39_e23513_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16210_body39_e23513_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16210_body39_e23513_d_n17;
            let (assign16210_body40_e23530, assign16210_body40_e23530_d_n0, assign16210_body40_e23530_d_n2, assign16210_body40_e23530_d_n6, assign16210_body40_e23530_d_n7, assign16210_body40_e23530_d_n10, assign16210_body40_e23530_d_n11, assign16210_body40_e23530_d_n12, assign16210_body40_e23530_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body40_e23527: f64 = (locals.var_phi_sl_soi - locals.var_vds);
        let assign16210_body40_e23528: f64 = (locals.var_beta * assign16210_body40_e23527);
        (assign16210_body40_e23528, (locals.var_beta * (locals.var_phi_sl_soi_dn0 - locals.var_vds_dn0)), (locals.var_beta * (locals.var_phi_sl_soi_dn2 - locals.var_vds_dn2)), (locals.var_beta * (locals.var_phi_sl_soi_dn6 - locals.var_vds_dn6)), (locals.var_beta * (locals.var_phi_sl_soi_dn7 - locals.var_vds_dn7)), ((locals.var_beta_dn10 * assign16210_body40_e23527) + (locals.var_beta * (locals.var_phi_sl_soi_dn10 - locals.var_vds_dn10))), (locals.var_beta * (locals.var_phi_sl_soi_dn11 - locals.var_vds_dn11)), (locals.var_beta * (locals.var_phi_sl_soi_dn12 - locals.var_vds_dn12)), (locals.var_beta * (locals.var_phi_sl_soi_dn17 - locals.var_vds_dn17)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn17,)
    }
};
            locals.var_rho = assign16210_body40_e23530;
            locals.var_rho_dn0 = assign16210_body40_e23530_d_n0;
            locals.var_rho_dn2 = assign16210_body40_e23530_d_n2;
            locals.var_rho_dn6 = assign16210_body40_e23530_d_n6;
            locals.var_rho_dn7 = assign16210_body40_e23530_d_n7;
            locals.var_rho_dn10 = assign16210_body40_e23530_d_n10;
            locals.var_rho_dn11 = assign16210_body40_e23530_d_n11;
            locals.var_rho_dn12 = assign16210_body40_e23530_d_n12;
            locals.var_rho_dn17 = assign16210_body40_e23530_d_n17;
            let (assign16210_body41_e23544, assign16210_body41_e23544_d_n0, assign16210_body41_e23544_d_n2, assign16210_body41_e23544_d_n6, assign16210_body41_e23544_d_n7, assign16210_body41_e23544_d_n10, assign16210_body41_e23544_d_n11, assign16210_body41_e23544_d_n12, assign16210_body41_e23544_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body41_e23542: f64 = (locals.var_rho).exp();
        (assign16210_body41_e23542, (assign16210_body41_e23542 * locals.var_rho_dn0), (assign16210_body41_e23542 * locals.var_rho_dn2), (assign16210_body41_e23542 * locals.var_rho_dn6), (assign16210_body41_e23542 * locals.var_rho_dn7), (assign16210_body41_e23542 * locals.var_rho_dn10), (assign16210_body41_e23542 * locals.var_rho_dn11), (assign16210_body41_e23542 * locals.var_rho_dn12), (assign16210_body41_e23542 * locals.var_rho_dn17),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn12, locals.var_exp_rho_dn17,)
    }
};
            locals.var_exp_rho = assign16210_body41_e23544;
            locals.var_exp_rho_dn0 = assign16210_body41_e23544_d_n0;
            locals.var_exp_rho_dn2 = assign16210_body41_e23544_d_n2;
            locals.var_exp_rho_dn6 = assign16210_body41_e23544_d_n6;
            locals.var_exp_rho_dn7 = assign16210_body41_e23544_d_n7;
            locals.var_exp_rho_dn10 = assign16210_body41_e23544_d_n10;
            locals.var_exp_rho_dn11 = assign16210_body41_e23544_d_n11;
            locals.var_exp_rho_dn12 = assign16210_body41_e23544_d_n12;
            locals.var_exp_rho_dn17 = assign16210_body41_e23544_d_n17;
            let (assign16210_body42_e23565, assign16210_body42_e23565_d_n0, assign16210_body42_e23565_d_n2, assign16210_body42_e23565_d_n6, assign16210_body42_e23565_d_n7, assign16210_body42_e23565_d_n10, assign16210_body42_e23565_d_n11, assign16210_body42_e23565_d_n12, assign16210_body42_e23565_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body42_e23560: f64 = (locals.var_chi + 1.0);
        let assign16210_body42_e23561: f64 = (locals.var_exp_bvbsvds * assign16210_body42_e23560);
        let assign16210_body42_e23562: f64 = (locals.var_exp_rho - assign16210_body42_e23561);
        let assign16210_body42_e23563: f64 = (locals.var_cnst1soi * assign16210_body42_e23562);
        (assign16210_body42_e23563, ((locals.var_cnst1soi_dn0 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn12 - ((locals.var_exp_bvbsvds_dn12 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn17 - ((locals.var_exp_bvbsvds_dn17 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn10, locals.var_fsl1_dn11, locals.var_fsl1_dn12, locals.var_fsl1_dn17,)
    }
};
            locals.var_fsl1 = assign16210_body42_e23565;
            locals.var_fsl1_dn0 = assign16210_body42_e23565_d_n0;
            locals.var_fsl1_dn2 = assign16210_body42_e23565_d_n2;
            locals.var_fsl1_dn6 = assign16210_body42_e23565_d_n6;
            locals.var_fsl1_dn7 = assign16210_body42_e23565_d_n7;
            locals.var_fsl1_dn10 = assign16210_body42_e23565_d_n10;
            locals.var_fsl1_dn11 = assign16210_body42_e23565_d_n11;
            locals.var_fsl1_dn12 = assign16210_body42_e23565_d_n12;
            locals.var_fsl1_dn17 = assign16210_body42_e23565_d_n17;
            let (assign16210_body43_e23584, assign16210_body43_e23584_d_n0, assign16210_body43_e23584_d_n2, assign16210_body43_e23584_d_n6, assign16210_body43_e23584_d_n7, assign16210_body43_e23584_d_n10, assign16210_body43_e23584_d_n11, assign16210_body43_e23584_d_n12, assign16210_body43_e23584_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body43_e23578: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign16210_body43_e23581: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign16210_body43_e23582: f64 = (assign16210_body43_e23578 * assign16210_body43_e23581);
        (assign16210_body43_e23582, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn12 - locals.var_exp_bvbsvds_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn17 - locals.var_exp_bvbsvds_dn17))),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn11, locals.var_fsl1_dpsl_dn12, locals.var_fsl1_dpsl_dn17,)
    }
};
            locals.var_fsl1_dpsl = assign16210_body43_e23584;
            locals.var_fsl1_dpsl_dn0 = assign16210_body43_e23584_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign16210_body43_e23584_d_n2;
            locals.var_fsl1_dpsl_dn6 = assign16210_body43_e23584_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign16210_body43_e23584_d_n7;
            locals.var_fsl1_dpsl_dn10 = assign16210_body43_e23584_d_n10;
            locals.var_fsl1_dpsl_dn11 = assign16210_body43_e23584_d_n11;
            locals.var_fsl1_dpsl_dn12 = assign16210_body43_e23584_d_n12;
            locals.var_fsl1_dpsl_dn17 = assign16210_body43_e23584_d_n17;
            let (assign16210_body44_e23602, assign16210_body44_e23602_d_n0, assign16210_body44_e23602_d_n2, assign16210_body44_e23602_d_n6, assign16210_body44_e23602_d_n7, assign16210_body44_e23602_d_n10, assign16210_body44_e23602_d_n11, assign16210_body44_e23602_d_n12, assign16210_body44_e23602_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body44_e23597: f64 = (locals.var_fb * locals.var_fb);
        let assign16210_body44_e23599: f64 = (assign16210_body44_e23597 + locals.var_fsl1);
        let assign16210_body44_e23600: f64 = (assign16210_body44_e23599).sqrt();
        (assign16210_body44_e23600, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fsl1_dn0) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fsl1_dn2) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fsl1_dn6) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fsl1_dn7) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fsl1_dn10) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fsl1_dn11) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fsl1_dn12) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fsl1_dn17) / (2.0 * assign16210_body44_e23600)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16210_body44_e23602;
            locals.var_fsl2_dn0 = assign16210_body44_e23602_d_n0;
            locals.var_fsl2_dn2 = assign16210_body44_e23602_d_n2;
            locals.var_fsl2_dn6 = assign16210_body44_e23602_d_n6;
            locals.var_fsl2_dn7 = assign16210_body44_e23602_d_n7;
            locals.var_fsl2_dn10 = assign16210_body44_e23602_d_n10;
            locals.var_fsl2_dn11 = assign16210_body44_e23602_d_n11;
            locals.var_fsl2_dn12 = assign16210_body44_e23602_d_n12;
            locals.var_fsl2_dn17 = assign16210_body44_e23602_d_n17;
            let (assign16210_body45_e23625, assign16210_body45_e23625_d_n0, assign16210_body45_e23625_d_n2, assign16210_body45_e23625_d_n6, assign16210_body45_e23625_d_n7, assign16210_body45_e23625_d_n10, assign16210_body45_e23625_d_n11, assign16210_body45_e23625_d_n12, assign16210_body45_e23625_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body45_e23616: f64 = (2.0 * locals.var_fb_dpss);
        let assign16210_body45_e23618: f64 = (assign16210_body45_e23616 * locals.var_fb);
        let assign16210_body45_e23620: f64 = (assign16210_body45_e23618 + locals.var_fsl1_dpsl);
        let assign16210_body45_e23621: f64 = (0.5 * assign16210_body45_e23620);
        let assign16210_body45_e23623: f64 = (assign16210_body45_e23621 / locals.var_fsl2);
        (assign16210_body45_e23623, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn0)) + locals.var_fsl1_dpsl_dn0)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn0)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn2)) + locals.var_fsl1_dpsl_dn2)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn2)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn6)) + locals.var_fsl1_dpsl_dn6)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn6)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn7)) + locals.var_fsl1_dpsl_dn7)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn7)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn10)) + locals.var_fsl1_dpsl_dn10)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn10)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn11)) + locals.var_fsl1_dpsl_dn11)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn11)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn12)) + locals.var_fsl1_dpsl_dn12)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn12)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn17)) + locals.var_fsl1_dpsl_dn17)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn17)) / (locals.var_fsl2 * locals.var_fsl2)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16210_body45_e23625;
            locals.var_fsl2_dpsl_dn0 = assign16210_body45_e23625_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16210_body45_e23625_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16210_body45_e23625_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16210_body45_e23625_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16210_body45_e23625_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16210_body45_e23625_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16210_body45_e23625_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16210_body45_e23625_d_n17;
            let (assign16210_body46_e23643, assign16210_body46_e23643_d_n0, assign16210_body46_e23643_d_n2, assign16210_body46_e23643_d_n6, assign16210_body46_e23643_d_n7, assign16210_body46_e23643_d_n10, assign16210_body46_e23643_d_n11, assign16210_body46_e23643_d_n12, assign16210_body46_e23643_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body46_e23631: f64 = (-locals.var_vgp);
        let assign16210_body46_e23633: f64 = (assign16210_body46_e23631 + locals.var_phi_sl_soi);
        let assign16210_body46_e23636: f64 = (locals.var_fac1 * locals.var_fsl2);
        let assign16210_body46_e23637: f64 = (assign16210_body46_e23633 + assign16210_body46_e23636);
        let assign16210_body46_e23640: f64 = (locals.var_c_fox_inv * locals.var_qhs);
        let assign16210_body46_e23641: f64 = (assign16210_body46_e23637 - assign16210_body46_e23640);
        (assign16210_body46_e23641, ((((-locals.var_vgp_dn0) + locals.var_phi_sl_soi_dn0) + ((locals.var_fac1_dn0 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn0))) - ((locals.var_c_fox_inv_dn0 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn0))), ((((-locals.var_vgp_dn2) + locals.var_phi_sl_soi_dn2) + ((locals.var_fac1_dn2 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn2))) - ((locals.var_c_fox_inv_dn2 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn2))), ((((-locals.var_vgp_dn6) + locals.var_phi_sl_soi_dn6) + ((locals.var_fac1_dn6 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn6))) - ((locals.var_c_fox_inv_dn6 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn6))), ((((-locals.var_vgp_dn7) + locals.var_phi_sl_soi_dn7) + ((locals.var_fac1_dn7 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn7))) - ((locals.var_c_fox_inv_dn7 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn7))), ((((-locals.var_vgp_dn10) + locals.var_phi_sl_soi_dn10) + ((locals.var_fac1_dn10 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn10))) - ((locals.var_c_fox_inv_dn10 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn10))), ((((-locals.var_vgp_dn11) + locals.var_phi_sl_soi_dn11) + ((locals.var_fac1_dn11 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn11))) - ((locals.var_c_fox_inv_dn11 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn11))), ((((-locals.var_vgp_dn12) + locals.var_phi_sl_soi_dn12) + ((locals.var_fac1_dn12 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn12))) - ((locals.var_c_fox_inv_dn12 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn12))), ((((-locals.var_vgp_dn17) + locals.var_phi_sl_soi_dn17) + ((locals.var_fac1_dn17 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn17))) - ((locals.var_c_fox_inv_dn17 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn17))),)
    } else {
        (locals.var_fsl, locals.var_fsl_dn0, locals.var_fsl_dn2, locals.var_fsl_dn6, locals.var_fsl_dn7, locals.var_fsl_dn10, locals.var_fsl_dn11, locals.var_fsl_dn12, locals.var_fsl_dn17,)
    }
};
            locals.var_fsl = assign16210_body46_e23643;
            locals.var_fsl_dn0 = assign16210_body46_e23643_d_n0;
            locals.var_fsl_dn2 = assign16210_body46_e23643_d_n2;
            locals.var_fsl_dn6 = assign16210_body46_e23643_d_n6;
            locals.var_fsl_dn7 = assign16210_body46_e23643_d_n7;
            locals.var_fsl_dn10 = assign16210_body46_e23643_d_n10;
            locals.var_fsl_dn11 = assign16210_body46_e23643_d_n11;
            locals.var_fsl_dn12 = assign16210_body46_e23643_d_n12;
            locals.var_fsl_dn17 = assign16210_body46_e23643_d_n17;
            let (assign16210_body47_e23654, assign16210_body47_e23654_d_n0, assign16210_body47_e23654_d_n2, assign16210_body47_e23654_d_n6, assign16210_body47_e23654_d_n7, assign16210_body47_e23654_d_n10, assign16210_body47_e23654_d_n11, assign16210_body47_e23654_d_n12, assign16210_body47_e23654_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body47_e23651: f64 = (locals.var_fac1 * locals.var_fsl2_dpsl);
        let assign16210_body47_e23652: f64 = (1.0 + assign16210_body47_e23651);
        (assign16210_body47_e23652, ((locals.var_fac1_dn0 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn0)), ((locals.var_fac1_dn2 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn2)), ((locals.var_fac1_dn6 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn6)), ((locals.var_fac1_dn7 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn7)), ((locals.var_fac1_dn10 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn10)), ((locals.var_fac1_dn11 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn11)), ((locals.var_fac1_dn12 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn12)), ((locals.var_fac1_dn17 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn17)),)
    } else {
        (locals.var_fsl_dpsl, locals.var_fsl_dpsl_dn0, locals.var_fsl_dpsl_dn2, locals.var_fsl_dpsl_dn6, locals.var_fsl_dpsl_dn7, locals.var_fsl_dpsl_dn10, locals.var_fsl_dpsl_dn11, locals.var_fsl_dpsl_dn12, locals.var_fsl_dpsl_dn17,)
    }
};
            locals.var_fsl_dpsl = assign16210_body47_e23654;
            locals.var_fsl_dpsl_dn0 = assign16210_body47_e23654_d_n0;
            locals.var_fsl_dpsl_dn2 = assign16210_body47_e23654_d_n2;
            locals.var_fsl_dpsl_dn6 = assign16210_body47_e23654_d_n6;
            locals.var_fsl_dpsl_dn7 = assign16210_body47_e23654_d_n7;
            locals.var_fsl_dpsl_dn10 = assign16210_body47_e23654_d_n10;
            locals.var_fsl_dpsl_dn11 = assign16210_body47_e23654_d_n11;
            locals.var_fsl_dpsl_dn12 = assign16210_body47_e23654_d_n12;
            locals.var_fsl_dpsl_dn17 = assign16210_body47_e23654_d_n17;
            let assign16210_body48_e23661: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_lp_sl > 3.0)) { 1.0 } else { 0.0 };
            locals.var_guard488 = assign16210_body48_e23661;
            let (assign16210_body49_e23672,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16210_body49_e23670: f64 = (locals.var_lp_sl_max + 1.0);
        (assign16210_body49_e23670,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign16210_body49_e23672;
            let (assign16210_body50_e23685, assign16210_body50_e23685_d_n0, assign16210_body50_e23685_d_n2, assign16210_body50_e23685_d_n6, assign16210_body50_e23685_d_n7, assign16210_body50_e23685_d_n10, assign16210_body50_e23685_d_n11, assign16210_body50_e23685_d_n12, assign16210_body50_e23685_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16210_body50_e23681: f64 = (-locals.var_fsl);
        let assign16210_body50_e23683: f64 = (assign16210_body50_e23681 / locals.var_fsl_dpsl);
        (assign16210_body50_e23683, ((((-locals.var_fsl_dn0) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn0)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn2) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn2)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn6) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn6)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn7) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn7)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn10) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn10)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn11) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn11)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn12) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn12)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn17) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn17)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn12, locals.var_dpsl_dn17,)
    }
};
            locals.var_dpsl = assign16210_body50_e23685;
            locals.var_dpsl_dn0 = assign16210_body50_e23685_d_n0;
            locals.var_dpsl_dn2 = assign16210_body50_e23685_d_n2;
            locals.var_dpsl_dn6 = assign16210_body50_e23685_d_n6;
            locals.var_dpsl_dn7 = assign16210_body50_e23685_d_n7;
            locals.var_dpsl_dn10 = assign16210_body50_e23685_d_n10;
            locals.var_dpsl_dn11 = assign16210_body50_e23685_d_n11;
            locals.var_dpsl_dn12 = assign16210_body50_e23685_d_n12;
            locals.var_dpsl_dn17 = assign16210_body50_e23685_d_n17;
            let (assign16210_body51_e23708, assign16210_body51_e23708_d_n0, assign16210_body51_e23708_d_n2, assign16210_body51_e23708_d_n6, assign16210_body51_e23708_d_n7, assign16210_body51_e23708_d_n10, assign16210_body51_e23708_d_n11, assign16210_body51_e23708_d_n12, assign16210_body51_e23708_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16210_body51_e23695: f64 = (0.5 * 0.1);
        let assign16210_body51_e23699: f64 = (locals.var_phi_sl_soi).abs();
        let (assign16210_body51_e23704, assign16210_body51_e23704_d_n0, assign16210_body51_e23704_d_n2, assign16210_body51_e23704_d_n6, assign16210_body51_e23704_d_n7, assign16210_body51_e23704_d_n10, assign16210_body51_e23704_d_n11, assign16210_body51_e23704_d_n12, assign16210_body51_e23704_d_n17,) = {
            if (1.0 >= assign16210_body51_e23699) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign16210_body51_e23703: f64 = (locals.var_phi_sl_soi).abs();
                (assign16210_body51_e23703, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn0 } else { (-locals.var_phi_sl_soi_dn0) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn2 } else { (-locals.var_phi_sl_soi_dn2) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn6 } else { (-locals.var_phi_sl_soi_dn6) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn7 } else { (-locals.var_phi_sl_soi_dn7) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn10 } else { (-locals.var_phi_sl_soi_dn10) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn11 } else { (-locals.var_phi_sl_soi_dn11) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn12 } else { (-locals.var_phi_sl_soi_dn12) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn17 } else { (-locals.var_phi_sl_soi_dn17) },)
            }
        };
        let assign16210_body51_e23705: f64 = (1.0 + assign16210_body51_e23704);
        let assign16210_body51_e23706: f64 = (assign16210_body51_e23695 * assign16210_body51_e23705);
        (assign16210_body51_e23706, (assign16210_body51_e23695 * assign16210_body51_e23704_d_n0), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n2), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n6), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n7), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n10), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n11), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n12), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign16210_body51_e23708;
            locals.var_dplim_dn0 = assign16210_body51_e23708_d_n0;
            locals.var_dplim_dn2 = assign16210_body51_e23708_d_n2;
            locals.var_dplim_dn6 = assign16210_body51_e23708_d_n6;
            locals.var_dplim_dn7 = assign16210_body51_e23708_d_n7;
            locals.var_dplim_dn10 = assign16210_body51_e23708_d_n10;
            locals.var_dplim_dn11 = assign16210_body51_e23708_d_n11;
            locals.var_dplim_dn12 = assign16210_body51_e23708_d_n12;
            locals.var_dplim_dn17 = assign16210_body51_e23708_d_n17;
            let assign16210_body52_e23710: f64 = (locals.var_dpsl).abs();
            let assign16210_body52_e23712: f64 = if assign16210_body52_e23710 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard489 = assign16210_body52_e23712;
            let (assign16210_body53_e23732, assign16210_body53_e23732_d_n0, assign16210_body53_e23732_d_n2, assign16210_body53_e23732_d_n6, assign16210_body53_e23732_d_n7, assign16210_body53_e23732_d_n10, assign16210_body53_e23732_d_n11, assign16210_body53_e23732_d_n12, assign16210_body53_e23732_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 != 0.0)) {
        let (assign16210_body53_e23729,) = {
            if (locals.var_dpsl >= 0.0) {
                (1.0,)
            } else {
                let assign16210_body53_e23728: f64 = (-1.0);
                (assign16210_body53_e23728,)
            }
        };
        let assign16210_body53_e23730: f64 = (locals.var_dplim * assign16210_body53_e23729);
        (assign16210_body53_e23730, (locals.var_dplim_dn0 * assign16210_body53_e23729), (locals.var_dplim_dn2 * assign16210_body53_e23729), (locals.var_dplim_dn6 * assign16210_body53_e23729), (locals.var_dplim_dn7 * assign16210_body53_e23729), (locals.var_dplim_dn10 * assign16210_body53_e23729), (locals.var_dplim_dn11 * assign16210_body53_e23729), (locals.var_dplim_dn12 * assign16210_body53_e23729), (locals.var_dplim_dn17 * assign16210_body53_e23729),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn12, locals.var_dpsl_dn17,)
    }
};
            locals.var_dpsl = assign16210_body53_e23732;
            locals.var_dpsl_dn0 = assign16210_body53_e23732_d_n0;
            locals.var_dpsl_dn2 = assign16210_body53_e23732_d_n2;
            locals.var_dpsl_dn6 = assign16210_body53_e23732_d_n6;
            locals.var_dpsl_dn7 = assign16210_body53_e23732_d_n7;
            locals.var_dpsl_dn10 = assign16210_body53_e23732_d_n10;
            locals.var_dpsl_dn11 = assign16210_body53_e23732_d_n11;
            locals.var_dpsl_dn12 = assign16210_body53_e23732_d_n12;
            locals.var_dpsl_dn17 = assign16210_body53_e23732_d_n17;
            let (assign16210_body54_e23744, assign16210_body54_e23744_d_n0, assign16210_body54_e23744_d_n2, assign16210_body54_e23744_d_n6, assign16210_body54_e23744_d_n7, assign16210_body54_e23744_d_n10, assign16210_body54_e23744_d_n11, assign16210_body54_e23744_d_n12, assign16210_body54_e23744_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16210_body54_e23742: f64 = (locals.var_phi_sl_soi + locals.var_dpsl);
        (assign16210_body54_e23742, (locals.var_phi_sl_soi_dn0 + locals.var_dpsl_dn0), (locals.var_phi_sl_soi_dn2 + locals.var_dpsl_dn2), (locals.var_phi_sl_soi_dn6 + locals.var_dpsl_dn6), (locals.var_phi_sl_soi_dn7 + locals.var_dpsl_dn7), (locals.var_phi_sl_soi_dn10 + locals.var_dpsl_dn10), (locals.var_phi_sl_soi_dn11 + locals.var_dpsl_dn11), (locals.var_phi_sl_soi_dn12 + locals.var_dpsl_dn12), (locals.var_phi_sl_soi_dn17 + locals.var_dpsl_dn17),)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
            locals.var_phi_sl_soi = assign16210_body54_e23744;
            locals.var_phi_sl_soi_dn0 = assign16210_body54_e23744_d_n0;
            locals.var_phi_sl_soi_dn2 = assign16210_body54_e23744_d_n2;
            locals.var_phi_sl_soi_dn6 = assign16210_body54_e23744_d_n6;
            locals.var_phi_sl_soi_dn7 = assign16210_body54_e23744_d_n7;
            locals.var_phi_sl_soi_dn10 = assign16210_body54_e23744_d_n10;
            locals.var_phi_sl_soi_dn11 = assign16210_body54_e23744_d_n11;
            locals.var_phi_sl_soi_dn12 = assign16210_body54_e23744_d_n12;
            locals.var_phi_sl_soi_dn17 = assign16210_body54_e23744_d_n17;
            let assign16210_body55_e23746: f64 = (locals.var_dpsl).abs();
            let assign16210_body55_e23750: f64 = (locals.var_fsl).abs();
            let assign16210_body55_e23753: f64 = if ((assign16210_body55_e23746 <= 5e-12) && (assign16210_body55_e23750 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard490 = assign16210_body55_e23753;
            let (assign16210_body56_e23765,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard490 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign16210_body56_e23765;
            let (assign16210_body57_e23774,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body57_e23772: f64 = (locals.var_lp_sl + 1.0);
        (assign16210_body57_e23772,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign16210_body57_e23774;
        }

    }

    pub(super) fn stamp_transient_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign16220_e23783,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16220_e23781: f64 = (locals.var_lp_sl - 1.0);
        (assign16220_e23781,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign16220_e23783;

        let (assign16230_e23790, assign16230_e23790_d_n0, assign16230_e23790_d_n2, assign16230_e23790_d_n6, assign16230_e23790_d_n7, assign16230_e23790_d_n10, assign16230_e23790_d_n11, assign16230_e23790_d_n12, assign16230_e23790_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    } else {
        (locals.var_q_depsl, locals.var_q_depsl_dn0, locals.var_q_depsl_dn2, locals.var_q_depsl_dn6, locals.var_q_depsl_dn7, locals.var_q_depsl_dn10, locals.var_q_depsl_dn11, locals.var_q_depsl_dn12, locals.var_q_depsl_dn17,)
    }
};
        locals.var_q_depsl = assign16230_e23790;
        locals.var_q_depsl_dn0 = assign16230_e23790_d_n0;
        locals.var_q_depsl_dn2 = assign16230_e23790_d_n2;
        locals.var_q_depsl_dn6 = assign16230_e23790_d_n6;
        locals.var_q_depsl_dn7 = assign16230_e23790_d_n7;
        locals.var_q_depsl_dn10 = assign16230_e23790_d_n10;
        locals.var_q_depsl_dn11 = assign16230_e23790_d_n11;
        locals.var_q_depsl_dn12 = assign16230_e23790_d_n12;
        locals.var_q_depsl_dn17 = assign16230_e23790_d_n17;

        let (assign16240_e23797, assign16240_e23797_d_n0, assign16240_e23797_d_n2, assign16240_e23797_d_n6, assign16240_e23797_d_n7, assign16240_e23797_d_n10, assign16240_e23797_d_n11, assign16240_e23797_d_n12, assign16240_e23797_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_q_depsl, locals.var_q_depsl_dn0, locals.var_q_depsl_dn2, locals.var_q_depsl_dn6, locals.var_q_depsl_dn7, locals.var_q_depsl_dn10, locals.var_q_depsl_dn11, locals.var_q_depsl_dn12, locals.var_q_depsl_dn17,)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
        locals.var_q_depl = assign16240_e23797;
        locals.var_q_depl_dn0 = assign16240_e23797_d_n0;
        locals.var_q_depl_dn2 = assign16240_e23797_d_n2;
        locals.var_q_depl_dn6 = assign16240_e23797_d_n6;
        locals.var_q_depl_dn7 = assign16240_e23797_d_n7;
        locals.var_q_depl_dn10 = assign16240_e23797_d_n10;
        locals.var_q_depl_dn11 = assign16240_e23797_d_n11;
        locals.var_q_depl_dn12 = assign16240_e23797_d_n12;
        locals.var_q_depl_dn17 = assign16240_e23797_d_n17;

        let (assign16250_e23804, assign16250_e23804_d_n0, assign16250_e23804_d_n2, assign16250_e23804_d_n6, assign16250_e23804_d_n7, assign16250_e23804_d_n10, assign16250_e23804_d_n11, assign16250_e23804_d_n12, assign16250_e23804_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16250_e23804;
        locals.var_psl_dn0 = assign16250_e23804_d_n0;
        locals.var_psl_dn2 = assign16250_e23804_d_n2;
        locals.var_psl_dn6 = assign16250_e23804_d_n6;
        locals.var_psl_dn7 = assign16250_e23804_d_n7;
        locals.var_psl_dn10 = assign16250_e23804_d_n10;
        locals.var_psl_dn11 = assign16250_e23804_d_n11;
        locals.var_psl_dn12 = assign16250_e23804_d_n12;
        locals.var_psl_dn17 = assign16250_e23804_d_n17;

        let (assign16270_e23820, assign16270_e23820_d_n0, assign16270_e23820_d_n2, assign16270_e23820_d_n6, assign16270_e23820_d_n7, assign16270_e23820_d_n10, assign16270_e23820_d_n11, assign16270_e23820_d_n12, assign16270_e23820_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16270_e23818: f64 = (locals.var_q_depsl / locals.var_cnst0soi);
        (assign16270_e23818, (((locals.var_q_depsl_dn0 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn0)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn2 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn2)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn6 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn6)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn7 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn7)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn10 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn10)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn11 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn11)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn12 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn12)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn17 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn17)) / (locals.var_cnst0soi * locals.var_cnst0soi)),)
    } else {
        (locals.var_q_depsl_soi_o_cnst0soi, locals.var_q_depsl_soi_o_cnst0soi_dn0, locals.var_q_depsl_soi_o_cnst0soi_dn2, locals.var_q_depsl_soi_o_cnst0soi_dn6, locals.var_q_depsl_soi_o_cnst0soi_dn7, locals.var_q_depsl_soi_o_cnst0soi_dn10, locals.var_q_depsl_soi_o_cnst0soi_dn11, locals.var_q_depsl_soi_o_cnst0soi_dn12, locals.var_q_depsl_soi_o_cnst0soi_dn17,)
    }
};
        locals.var_q_depsl_soi_o_cnst0soi = assign16270_e23820;
        locals.var_q_depsl_soi_o_cnst0soi_dn0 = assign16270_e23820_d_n0;
        locals.var_q_depsl_soi_o_cnst0soi_dn2 = assign16270_e23820_d_n2;
        locals.var_q_depsl_soi_o_cnst0soi_dn6 = assign16270_e23820_d_n6;
        locals.var_q_depsl_soi_o_cnst0soi_dn7 = assign16270_e23820_d_n7;
        locals.var_q_depsl_soi_o_cnst0soi_dn10 = assign16270_e23820_d_n10;
        locals.var_q_depsl_soi_o_cnst0soi_dn11 = assign16270_e23820_d_n11;
        locals.var_q_depsl_soi_o_cnst0soi_dn12 = assign16270_e23820_d_n12;
        locals.var_q_depsl_soi_o_cnst0soi_dn17 = assign16270_e23820_d_n17;

        let (assign16280_e23831, assign16280_e23831_d_n0, assign16280_e23831_d_n2, assign16280_e23831_d_n6, assign16280_e23831_d_n7, assign16280_e23831_d_n10, assign16280_e23831_d_n11, assign16280_e23831_d_n12, assign16280_e23831_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16280_e23828: f64 = (10.0 * 2.220446049250313e-16);
        let assign16280_e23829: f64 = (locals.var_q_depsl_soi_o_cnst0soi + assign16280_e23828);
        (assign16280_e23829, locals.var_q_depsl_soi_o_cnst0soi_dn0, locals.var_q_depsl_soi_o_cnst0soi_dn2, locals.var_q_depsl_soi_o_cnst0soi_dn6, locals.var_q_depsl_soi_o_cnst0soi_dn7, locals.var_q_depsl_soi_o_cnst0soi_dn10, locals.var_q_depsl_soi_o_cnst0soi_dn11, locals.var_q_depsl_soi_o_cnst0soi_dn12, locals.var_q_depsl_soi_o_cnst0soi_dn17,)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn12, locals.var_xilp12_dn17,)
    }
};
        locals.var_xilp12 = assign16280_e23831;
        locals.var_xilp12_dn0 = assign16280_e23831_d_n0;
        locals.var_xilp12_dn2 = assign16280_e23831_d_n2;
        locals.var_xilp12_dn6 = assign16280_e23831_d_n6;
        locals.var_xilp12_dn7 = assign16280_e23831_d_n7;
        locals.var_xilp12_dn10 = assign16280_e23831_d_n10;
        locals.var_xilp12_dn11 = assign16280_e23831_d_n11;
        locals.var_xilp12_dn12 = assign16280_e23831_d_n12;
        locals.var_xilp12_dn17 = assign16280_e23831_d_n17;

        let (assign16290_e23842, assign16290_e23842_d_n0, assign16290_e23842_d_n2, assign16290_e23842_d_n6, assign16290_e23842_d_n7, assign16290_e23842_d_n10, assign16290_e23842_d_n11, assign16290_e23842_d_n12, assign16290_e23842_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16290_e23839: f64 = (locals.var_fsl2 + locals.var_xilp12);
        let assign16290_e23840: f64 = (1.0 / assign16290_e23839);
        (assign16290_e23840, (-((locals.var_fsl2_dn0 + locals.var_xilp12_dn0) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn2 + locals.var_xilp12_dn2) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn6 + locals.var_xilp12_dn6) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn7 + locals.var_xilp12_dn7) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn10 + locals.var_xilp12_dn10) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn11 + locals.var_xilp12_dn11) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn12 + locals.var_xilp12_dn12) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn17 + locals.var_xilp12_dn17) / (assign16290_e23839 * assign16290_e23839))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16290_e23842;
        locals.var_t1_dn0 = assign16290_e23842_d_n0;
        locals.var_t1_dn2 = assign16290_e23842_d_n2;
        locals.var_t1_dn6 = assign16290_e23842_d_n6;
        locals.var_t1_dn7 = assign16290_e23842_d_n7;
        locals.var_t1_dn10 = assign16290_e23842_d_n10;
        locals.var_t1_dn11 = assign16290_e23842_d_n11;
        locals.var_t1_dn12 = assign16290_e23842_d_n12;
        locals.var_t1_dn17 = assign16290_e23842_d_n17;

        let (assign16300_e23853, assign16300_e23853_d_n0, assign16300_e23853_d_n2, assign16300_e23853_d_n6, assign16300_e23853_d_n7, assign16300_e23853_d_n10, assign16300_e23853_d_n11, assign16300_e23853_d_n12, assign16300_e23853_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16300_e23849: f64 = (locals.var_cnst0soi * locals.var_fsl1);
        let assign16300_e23851: f64 = (assign16300_e23849 * locals.var_t1);
        (assign16300_e23851, ((((locals.var_cnst0soi_dn0 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn0)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn2)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn6)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn7)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn10)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn11)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn12)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn17)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn17)),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
        locals.var_q_nl = assign16300_e23853;
        locals.var_q_nl_dn0 = assign16300_e23853_d_n0;
        locals.var_q_nl_dn2 = assign16300_e23853_d_n2;
        locals.var_q_nl_dn6 = assign16300_e23853_d_n6;
        locals.var_q_nl_dn7 = assign16300_e23853_d_n7;
        locals.var_q_nl_dn10 = assign16300_e23853_d_n10;
        locals.var_q_nl_dn11 = assign16300_e23853_d_n11;
        locals.var_q_nl_dn12 = assign16300_e23853_d_n12;
        locals.var_q_nl_dn17 = assign16300_e23853_d_n17;

        let (assign16310_e23861, assign16310_e23861_d_n0, assign16310_e23861_d_n2, assign16310_e23861_d_n6, assign16310_e23861_d_n7, assign16310_e23861_d_n10, assign16310_e23861_d_n11, assign16310_e23861_d_n12, assign16310_e23861_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16310_e23859: f64 = (-locals.var_q_nl);
        (assign16310_e23859, (-locals.var_q_nl_dn0), (-locals.var_q_nl_dn2), (-locals.var_q_nl_dn6), (-locals.var_q_nl_dn7), (-locals.var_q_nl_dn10), (-locals.var_q_nl_dn11), (-locals.var_q_nl_dn12), (-locals.var_q_nl_dn17),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
        locals.var_q_nl = assign16310_e23861;
        locals.var_q_nl_dn0 = assign16310_e23861_d_n0;
        locals.var_q_nl_dn2 = assign16310_e23861_d_n2;
        locals.var_q_nl_dn6 = assign16310_e23861_d_n6;
        locals.var_q_nl_dn7 = assign16310_e23861_d_n7;
        locals.var_q_nl_dn10 = assign16310_e23861_d_n10;
        locals.var_q_nl_dn11 = assign16310_e23861_d_n11;
        locals.var_q_nl_dn12 = assign16310_e23861_d_n12;
        locals.var_q_nl_dn17 = assign16310_e23861_d_n17;

        let (assign16320_e23870, assign16320_e23870_d_n0, assign16320_e23870_d_n2, assign16320_e23870_d_n6, assign16320_e23870_d_n7, assign16320_e23870_d_n10, assign16320_e23870_d_n11, assign16320_e23870_d_n12, assign16320_e23870_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16320_e23868: f64 = (locals.var_psl - locals.var_ps0);
        (assign16320_e23868, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn12 - locals.var_ps0_dn12), (locals.var_psl_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16320_e23870;
        locals.var_pds_dn0 = assign16320_e23870_d_n0;
        locals.var_pds_dn2 = assign16320_e23870_d_n2;
        locals.var_pds_dn6 = assign16320_e23870_d_n6;
        locals.var_pds_dn7 = assign16320_e23870_d_n7;
        locals.var_pds_dn10 = assign16320_e23870_d_n10;
        locals.var_pds_dn11 = assign16320_e23870_d_n11;
        locals.var_pds_dn12 = assign16320_e23870_d_n12;
        locals.var_pds_dn17 = assign16320_e23870_d_n17;

        let (assign16330_e23877, assign16330_e23877_d_n0, assign16330_e23877_d_n2, assign16330_e23877_d_n6, assign16330_e23877_d_n7, assign16330_e23877_d_n10, assign16330_e23877_d_n11, assign16330_e23877_d_n12, assign16330_e23877_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign16330_e23877;
        locals.var_vds_dn0 = assign16330_e23877_d_n0;
        locals.var_vds_dn2 = assign16330_e23877_d_n2;
        locals.var_vds_dn6 = assign16330_e23877_d_n6;
        locals.var_vds_dn7 = assign16330_e23877_d_n7;
        locals.var_vds_dn10 = assign16330_e23877_d_n10;
        locals.var_vds_dn11 = assign16330_e23877_d_n11;
        locals.var_vds_dn12 = assign16330_e23877_d_n12;
        locals.var_vds_dn17 = assign16330_e23877_d_n17;

        let (assign16340_e23886, assign16340_e23886_d_n0, assign16340_e23886_d_n2, assign16340_e23886_d_n6, assign16340_e23886_d_n7, assign16340_e23886_d_n10, assign16340_e23886_d_n11, assign16340_e23886_d_n12, assign16340_e23886_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16340_e23884: f64 = (locals.var_beta / locals.var_xi0);
        (assign16340_e23884, (-((locals.var_beta * locals.var_xi0_dn0) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn2) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn6) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn7) / (locals.var_xi0 * locals.var_xi0))), (((locals.var_beta_dn10 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn10)) / (locals.var_xi0 * locals.var_xi0)), (-((locals.var_beta * locals.var_xi0_dn11) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn12) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn17) / (locals.var_xi0 * locals.var_xi0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16340_e23886;
        locals.var_t1_dn0 = assign16340_e23886_d_n0;
        locals.var_t1_dn2 = assign16340_e23886_d_n2;
        locals.var_t1_dn6 = assign16340_e23886_d_n6;
        locals.var_t1_dn7 = assign16340_e23886_d_n7;
        locals.var_t1_dn10 = assign16340_e23886_d_n10;
        locals.var_t1_dn11 = assign16340_e23886_d_n11;
        locals.var_t1_dn12 = assign16340_e23886_d_n12;
        locals.var_t1_dn17 = assign16340_e23886_d_n17;

        let (assign16350_e23895, assign16350_e23895_d_n0, assign16350_e23895_d_n2, assign16350_e23895_d_n6, assign16350_e23895_d_n7, assign16350_e23895_d_n10, assign16350_e23895_d_n11, assign16350_e23895_d_n12, assign16350_e23895_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16350_e23893: f64 = (locals.var_t1 * locals.var_pds);
        (assign16350_e23893, ((locals.var_t1_dn0 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn0)), ((locals.var_t1_dn2 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn2)), ((locals.var_t1_dn6 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn6)), ((locals.var_t1_dn7 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn7)), ((locals.var_t1_dn10 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn10)), ((locals.var_t1_dn11 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn11)), ((locals.var_t1_dn12 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn12)), ((locals.var_t1_dn17 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn17)),)
    } else {
        (locals.var_eta, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12, locals.var_eta_dn17,)
    }
};
        locals.var_eta = assign16350_e23895;
        locals.var_eta_dn0 = assign16350_e23895_d_n0;
        locals.var_eta_dn2 = assign16350_e23895_d_n2;
        locals.var_eta_dn6 = assign16350_e23895_d_n6;
        locals.var_eta_dn7 = assign16350_e23895_d_n7;
        locals.var_eta_dn10 = assign16350_e23895_d_n10;
        locals.var_eta_dn11 = assign16350_e23895_d_n11;
        locals.var_eta_dn12 = assign16350_e23895_d_n12;
        locals.var_eta_dn17 = assign16350_e23895_d_n17;

        let (assign16360_e23904, assign16360_e23904_d_n0, assign16360_e23904_d_n2, assign16360_e23904_d_n6, assign16360_e23904_d_n7, assign16360_e23904_d_n10, assign16360_e23904_d_n11, assign16360_e23904_d_n12, assign16360_e23904_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16360_e23902: f64 = (locals.var_eta + 1.0);
        (assign16360_e23902, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12, locals.var_eta_dn17,)
    } else {
        (locals.var_eta1, locals.var_eta1_dn0, locals.var_eta1_dn2, locals.var_eta1_dn6, locals.var_eta1_dn7, locals.var_eta1_dn10, locals.var_eta1_dn11, locals.var_eta1_dn12, locals.var_eta1_dn17,)
    }
};
        locals.var_eta1 = assign16360_e23904;
        locals.var_eta1_dn0 = assign16360_e23904_d_n0;
        locals.var_eta1_dn2 = assign16360_e23904_d_n2;
        locals.var_eta1_dn6 = assign16360_e23904_d_n6;
        locals.var_eta1_dn7 = assign16360_e23904_d_n7;
        locals.var_eta1_dn10 = assign16360_e23904_d_n10;
        locals.var_eta1_dn11 = assign16360_e23904_d_n11;
        locals.var_eta1_dn12 = assign16360_e23904_d_n12;
        locals.var_eta1_dn17 = assign16360_e23904_d_n17;

        let (assign16370_e23912, assign16370_e23912_d_n0, assign16370_e23912_d_n2, assign16370_e23912_d_n6, assign16370_e23912_d_n7, assign16370_e23912_d_n10, assign16370_e23912_d_n11, assign16370_e23912_d_n12, assign16370_e23912_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16370_e23910: f64 = (locals.var_eta1).sqrt();
        (assign16370_e23910, (locals.var_eta1_dn0 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn2 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn6 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn7 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn10 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn11 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn12 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn17 / (2.0 * assign16370_e23910)),)
    } else {
        (locals.var_eta1p12, locals.var_eta1p12_dn0, locals.var_eta1p12_dn2, locals.var_eta1p12_dn6, locals.var_eta1p12_dn7, locals.var_eta1p12_dn10, locals.var_eta1p12_dn11, locals.var_eta1p12_dn12, locals.var_eta1p12_dn17,)
    }
};
        locals.var_eta1p12 = assign16370_e23912;
        locals.var_eta1p12_dn0 = assign16370_e23912_d_n0;
        locals.var_eta1p12_dn2 = assign16370_e23912_d_n2;
        locals.var_eta1p12_dn6 = assign16370_e23912_d_n6;
        locals.var_eta1p12_dn7 = assign16370_e23912_d_n7;
        locals.var_eta1p12_dn10 = assign16370_e23912_d_n10;
        locals.var_eta1p12_dn11 = assign16370_e23912_d_n11;
        locals.var_eta1p12_dn12 = assign16370_e23912_d_n12;
        locals.var_eta1p12_dn17 = assign16370_e23912_d_n17;

        let (assign16380_e23923, assign16380_e23923_d_n0, assign16380_e23923_d_n2, assign16380_e23923_d_n6, assign16380_e23923_d_n7, assign16380_e23923_d_n10, assign16380_e23923_d_n11, assign16380_e23923_d_n12, assign16380_e23923_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16380_e23920: f64 = (locals.var_eta1p12 + 1.0);
        let assign16380_e23921: f64 = (1.0 / assign16380_e23920);
        (assign16380_e23921, (-(locals.var_eta1p12_dn0 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn2 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn6 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn7 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn10 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn11 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn12 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn17 / (assign16380_e23920 * assign16380_e23920))),)
    } else {
        (locals.var_zeta12, locals.var_zeta12_dn0, locals.var_zeta12_dn2, locals.var_zeta12_dn6, locals.var_zeta12_dn7, locals.var_zeta12_dn10, locals.var_zeta12_dn11, locals.var_zeta12_dn12, locals.var_zeta12_dn17,)
    }
};
        locals.var_zeta12 = assign16380_e23923;
        locals.var_zeta12_dn0 = assign16380_e23923_d_n0;
        locals.var_zeta12_dn2 = assign16380_e23923_d_n2;
        locals.var_zeta12_dn6 = assign16380_e23923_d_n6;
        locals.var_zeta12_dn7 = assign16380_e23923_d_n7;
        locals.var_zeta12_dn10 = assign16380_e23923_d_n10;
        locals.var_zeta12_dn11 = assign16380_e23923_d_n11;
        locals.var_zeta12_dn12 = assign16380_e23923_d_n12;
        locals.var_zeta12_dn17 = assign16380_e23923_d_n17;

        let (assign16390_e23932, assign16390_e23932_d_n0, assign16390_e23932_d_n2, assign16390_e23932_d_n6, assign16390_e23932_d_n7, assign16390_e23932_d_n10, assign16390_e23932_d_n11, assign16390_e23932_d_n12, assign16390_e23932_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16390_e23930: f64 = (locals.var_zeta12 / locals.var_xi0p12);
        (assign16390_e23930, (((locals.var_zeta12_dn0 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn0)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn2 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn2)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn6 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn6)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn7 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn7)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn10 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn10)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn11 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn11)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn12 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn12)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn17 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn17)) / (locals.var_xi0p12 * locals.var_xi0p12)),)
    } else {
        (locals.var_f00, locals.var_f00_dn0, locals.var_f00_dn2, locals.var_f00_dn6, locals.var_f00_dn7, locals.var_f00_dn10, locals.var_f00_dn11, locals.var_f00_dn12, locals.var_f00_dn17,)
    }
};
        locals.var_f00 = assign16390_e23932;
        locals.var_f00_dn0 = assign16390_e23932_d_n0;
        locals.var_f00_dn2 = assign16390_e23932_d_n2;
        locals.var_f00_dn6 = assign16390_e23932_d_n6;
        locals.var_f00_dn7 = assign16390_e23932_d_n7;
        locals.var_f00_dn10 = assign16390_e23932_d_n10;
        locals.var_f00_dn11 = assign16390_e23932_d_n11;
        locals.var_f00_dn12 = assign16390_e23932_d_n12;
        locals.var_f00_dn17 = assign16390_e23932_d_n17;

        let (assign16400_e23943, assign16400_e23943_d_n0, assign16400_e23943_d_n2, assign16400_e23943_d_n6, assign16400_e23943_d_n7, assign16400_e23943_d_n10, assign16400_e23943_d_n11, assign16400_e23943_d_n12, assign16400_e23943_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16400_e23940: f64 = (locals.var_q_deps0_soi_o_cnst0soi + locals.var_q_depsl_soi_o_cnst0soi);
        let assign16400_e23941: f64 = (0.5 * assign16400_e23940);
        (assign16400_e23941, (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn0 + locals.var_q_depsl_soi_o_cnst0soi_dn0)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn2 + locals.var_q_depsl_soi_o_cnst0soi_dn2)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn6 + locals.var_q_depsl_soi_o_cnst0soi_dn6)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn7 + locals.var_q_depsl_soi_o_cnst0soi_dn7)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn10 + locals.var_q_depsl_soi_o_cnst0soi_dn10)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn11 + locals.var_q_depsl_soi_o_cnst0soi_dn11)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn12 + locals.var_q_depsl_soi_o_cnst0soi_dn12)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn17 + locals.var_q_depsl_soi_o_cnst0soi_dn17)),)
    } else {
        (locals.var_f10, locals.var_f10_dn0, locals.var_f10_dn2, locals.var_f10_dn6, locals.var_f10_dn7, locals.var_f10_dn10, locals.var_f10_dn11, locals.var_f10_dn12, locals.var_f10_dn17,)
    }
};
        locals.var_f10 = assign16400_e23943;
        locals.var_f10_dn0 = assign16400_e23943_d_n0;
        locals.var_f10_dn2 = assign16400_e23943_d_n2;
        locals.var_f10_dn6 = assign16400_e23943_d_n6;
        locals.var_f10_dn7 = assign16400_e23943_d_n7;
        locals.var_f10_dn10 = assign16400_e23943_d_n10;
        locals.var_f10_dn11 = assign16400_e23943_d_n11;
        locals.var_f10_dn12 = assign16400_e23943_d_n12;
        locals.var_f10_dn17 = assign16400_e23943_d_n17;

        let (assign16410_e23960, assign16410_e23960_d_n0, assign16410_e23960_d_n2, assign16410_e23960_d_n6, assign16410_e23960_d_n7, assign16410_e23960_d_n10, assign16410_e23960_d_n11, assign16410_e23960_d_n12, assign16410_e23960_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16410_e23950: f64 = (locals.var_vgp + locals.var_beta_inv);
        let assign16410_e23954: f64 = (2.0 * locals.var_ps0);
        let assign16410_e23956: f64 = (assign16410_e23954 + locals.var_pds);
        let assign16410_e23957: f64 = (0.5 * assign16410_e23956);
        let assign16410_e23958: f64 = (assign16410_e23950 - assign16410_e23957);
        (assign16410_e23958, (locals.var_vgp_dn0 - (0.5 * ((2.0 * locals.var_ps0_dn0) + locals.var_pds_dn0))), (locals.var_vgp_dn2 - (0.5 * ((2.0 * locals.var_ps0_dn2) + locals.var_pds_dn2))), (locals.var_vgp_dn6 - (0.5 * ((2.0 * locals.var_ps0_dn6) + locals.var_pds_dn6))), (locals.var_vgp_dn7 - (0.5 * ((2.0 * locals.var_ps0_dn7) + locals.var_pds_dn7))), ((locals.var_vgp_dn10 + locals.var_beta_inv_dn10) - (0.5 * ((2.0 * locals.var_ps0_dn10) + locals.var_pds_dn10))), (locals.var_vgp_dn11 - (0.5 * ((2.0 * locals.var_ps0_dn11) + locals.var_pds_dn11))), (locals.var_vgp_dn12 - (0.5 * ((2.0 * locals.var_ps0_dn12) + locals.var_pds_dn12))), (locals.var_vgp_dn17 - (0.5 * ((2.0 * locals.var_ps0_dn17) + locals.var_pds_dn17))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16410_e23960;
        locals.var_t1_dn0 = assign16410_e23960_d_n0;
        locals.var_t1_dn2 = assign16410_e23960_d_n2;
        locals.var_t1_dn6 = assign16410_e23960_d_n6;
        locals.var_t1_dn7 = assign16410_e23960_d_n7;
        locals.var_t1_dn10 = assign16410_e23960_d_n10;
        locals.var_t1_dn11 = assign16410_e23960_d_n11;
        locals.var_t1_dn12 = assign16410_e23960_d_n12;
        locals.var_t1_dn17 = assign16410_e23960_d_n17;

        let (assign16420_e23970, assign16420_e23970_d_n0, assign16420_e23970_d_n2, assign16420_e23970_d_n6, assign16420_e23970_d_n7, assign16420_e23970_d_n10, assign16420_e23970_d_n11, assign16420_e23970_d_n12, assign16420_e23970_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16420_e23966: f64 = (-locals.var_f10);
        let assign16420_e23968: f64 = (assign16420_e23966 + locals.var_f00);
        (assign16420_e23968, ((-locals.var_f10_dn0) + locals.var_f00_dn0), ((-locals.var_f10_dn2) + locals.var_f00_dn2), ((-locals.var_f10_dn6) + locals.var_f00_dn6), ((-locals.var_f10_dn7) + locals.var_f00_dn7), ((-locals.var_f10_dn10) + locals.var_f00_dn10), ((-locals.var_f10_dn11) + locals.var_f00_dn11), ((-locals.var_f10_dn12) + locals.var_f00_dn12), ((-locals.var_f10_dn17) + locals.var_f00_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign16420_e23970;
        locals.var_t2_dn0 = assign16420_e23970_d_n0;
        locals.var_t2_dn2 = assign16420_e23970_d_n2;
        locals.var_t2_dn6 = assign16420_e23970_d_n6;
        locals.var_t2_dn7 = assign16420_e23970_d_n7;
        locals.var_t2_dn10 = assign16420_e23970_d_n10;
        locals.var_t2_dn11 = assign16420_e23970_d_n11;
        locals.var_t2_dn12 = assign16420_e23970_d_n12;
        locals.var_t2_dn17 = assign16420_e23970_d_n17;

        let (assign16430_e23979, assign16430_e23979_d_n0, assign16430_e23979_d_n2, assign16430_e23979_d_n6, assign16430_e23979_d_n7, assign16430_e23979_d_n10, assign16430_e23979_d_n11, assign16430_e23979_d_n12, assign16430_e23979_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16430_e23977: f64 = (locals.var_beta * locals.var_c_fox);
        (assign16430_e23977, (locals.var_beta * locals.var_c_fox_dn0), (locals.var_beta * locals.var_c_fox_dn2), (locals.var_beta * locals.var_c_fox_dn6), (locals.var_beta * locals.var_c_fox_dn7), ((locals.var_beta_dn10 * locals.var_c_fox) + (locals.var_beta * locals.var_c_fox_dn10)), (locals.var_beta * locals.var_c_fox_dn11), (locals.var_beta * locals.var_c_fox_dn12), (locals.var_beta * locals.var_c_fox_dn17),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign16430_e23979;
        locals.var_t3_dn0 = assign16430_e23979_d_n0;
        locals.var_t3_dn2 = assign16430_e23979_d_n2;
        locals.var_t3_dn6 = assign16430_e23979_d_n6;
        locals.var_t3_dn7 = assign16430_e23979_d_n7;
        locals.var_t3_dn10 = assign16430_e23979_d_n10;
        locals.var_t3_dn11 = assign16430_e23979_d_n11;
        locals.var_t3_dn12 = assign16430_e23979_d_n12;
        locals.var_t3_dn17 = assign16430_e23979_d_n17;

        let (assign16440_e23988, assign16440_e23988_d_n0, assign16440_e23988_d_n2, assign16440_e23988_d_n6, assign16440_e23988_d_n7, assign16440_e23988_d_n10, assign16440_e23988_d_n11, assign16440_e23988_d_n12, assign16440_e23988_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16440_e23986: f64 = (locals.var_beta * locals.var_cnst0soi);
        (assign16440_e23986, (locals.var_beta * locals.var_cnst0soi_dn0), (locals.var_beta * locals.var_cnst0soi_dn2), (locals.var_beta * locals.var_cnst0soi_dn6), (locals.var_beta * locals.var_cnst0soi_dn7), ((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)), (locals.var_beta * locals.var_cnst0soi_dn11), (locals.var_beta * locals.var_cnst0soi_dn12), (locals.var_beta * locals.var_cnst0soi_dn17),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign16440_e23988;
        locals.var_t4_dn0 = assign16440_e23988_d_n0;
        locals.var_t4_dn2 = assign16440_e23988_d_n2;
        locals.var_t4_dn6 = assign16440_e23988_d_n6;
        locals.var_t4_dn7 = assign16440_e23988_d_n7;
        locals.var_t4_dn10 = assign16440_e23988_d_n10;
        locals.var_t4_dn11 = assign16440_e23988_d_n11;
        locals.var_t4_dn12 = assign16440_e23988_d_n12;
        locals.var_t4_dn17 = assign16440_e23988_d_n17;

        let (assign16450_e24001, assign16450_e24001_d_n0, assign16450_e24001_d_n2, assign16450_e24001_d_n6, assign16450_e24001_d_n7, assign16450_e24001_d_n10, assign16450_e24001_d_n11, assign16450_e24001_d_n12, assign16450_e24001_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16450_e23995: f64 = (locals.var_t3 * locals.var_t1);
        let assign16450_e23998: f64 = (locals.var_t4 * locals.var_t2);
        let assign16450_e23999: f64 = (assign16450_e23995 + assign16450_e23998);
        (assign16450_e23999, (((locals.var_t3_dn0 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0))), (((locals.var_t3_dn2 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2))), (((locals.var_t3_dn6 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6))), (((locals.var_t3_dn7 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7))), (((locals.var_t3_dn10 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10))), (((locals.var_t3_dn11 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn11))), (((locals.var_t3_dn12 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn12)) + ((locals.var_t4_dn12 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn12))), (((locals.var_t3_dn17 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn17)) + ((locals.var_t4_dn17 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn17))),)
    } else {
        (locals.var_fdd, locals.var_fdd_dn0, locals.var_fdd_dn2, locals.var_fdd_dn6, locals.var_fdd_dn7, locals.var_fdd_dn10, locals.var_fdd_dn11, locals.var_fdd_dn12, locals.var_fdd_dn17,)
    }
};
        locals.var_fdd = assign16450_e24001;
        locals.var_fdd_dn0 = assign16450_e24001_d_n0;
        locals.var_fdd_dn2 = assign16450_e24001_d_n2;
        locals.var_fdd_dn6 = assign16450_e24001_d_n6;
        locals.var_fdd_dn7 = assign16450_e24001_d_n7;
        locals.var_fdd_dn10 = assign16450_e24001_d_n10;
        locals.var_fdd_dn11 = assign16450_e24001_d_n11;
        locals.var_fdd_dn12 = assign16450_e24001_d_n12;
        locals.var_fdd_dn17 = assign16450_e24001_d_n17;

        let (assign16460_e24012, assign16460_e24012_d_n0, assign16460_e24012_d_n2, assign16460_e24012_d_n6, assign16460_e24012_d_n7, assign16460_e24012_d_n10, assign16460_e24012_d_n11, assign16460_e24012_d_n12, assign16460_e24012_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16460_e24008: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign16460_e24010: f64 = (assign16460_e24008 / 2.0);
        (assign16460_e24010, ((locals.var_q_depl_dn0 + locals.var_q_dep0_dn0) / 2.0), ((locals.var_q_depl_dn2 + locals.var_q_dep0_dn2) / 2.0), ((locals.var_q_depl_dn6 + locals.var_q_dep0_dn6) / 2.0), ((locals.var_q_depl_dn7 + locals.var_q_dep0_dn7) / 2.0), ((locals.var_q_depl_dn10 + locals.var_q_dep0_dn10) / 2.0), ((locals.var_q_depl_dn11 + locals.var_q_dep0_dn11) / 2.0), ((locals.var_q_depl_dn12 + locals.var_q_dep0_dn12) / 2.0), ((locals.var_q_depl_dn17 + locals.var_q_dep0_dn17) / 2.0),)
    } else {
        (locals.var_ab, locals.var_ab_dn0, locals.var_ab_dn2, locals.var_ab_dn6, locals.var_ab_dn7, locals.var_ab_dn10, locals.var_ab_dn11, locals.var_ab_dn12, locals.var_ab_dn17,)
    }
};
        locals.var_ab = assign16460_e24012;
        locals.var_ab_dn0 = assign16460_e24012_d_n0;
        locals.var_ab_dn2 = assign16460_e24012_d_n2;
        locals.var_ab_dn6 = assign16460_e24012_d_n6;
        locals.var_ab_dn7 = assign16460_e24012_d_n7;
        locals.var_ab_dn10 = assign16460_e24012_d_n10;
        locals.var_ab_dn11 = assign16460_e24012_d_n11;
        locals.var_ab_dn12 = assign16460_e24012_d_n12;
        locals.var_ab_dn17 = assign16460_e24012_d_n17;

        let (assign16470_e24024, assign16470_e24024_d_n0, assign16470_e24024_d_n2, assign16470_e24024_d_n6, assign16470_e24024_d_n7, assign16470_e24024_d_n10, assign16470_e24024_d_n11, assign16470_e24024_d_n12, assign16470_e24024_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16470_e24019: f64 = (locals.var_q_nl + locals.var_q_n0);
        let assign16470_e24020: f64 = (-assign16470_e24019);
        let assign16470_e24022: f64 = (assign16470_e24020 / 2.0);
        (assign16470_e24022, ((-(locals.var_q_nl_dn0 + locals.var_q_n0_dn0)) / 2.0), ((-(locals.var_q_nl_dn2 + locals.var_q_n0_dn2)) / 2.0), ((-(locals.var_q_nl_dn6 + locals.var_q_n0_dn6)) / 2.0), ((-(locals.var_q_nl_dn7 + locals.var_q_n0_dn7)) / 2.0), ((-(locals.var_q_nl_dn10 + locals.var_q_n0_dn10)) / 2.0), ((-(locals.var_q_nl_dn11 + locals.var_q_n0_dn11)) / 2.0), ((-(locals.var_q_nl_dn12 + locals.var_q_n0_dn12)) / 2.0), ((-(locals.var_q_nl_dn17 + locals.var_q_n0_dn17)) / 2.0),)
    } else {
        (locals.var_ai, locals.var_ai_dn0, locals.var_ai_dn2, locals.var_ai_dn6, locals.var_ai_dn7, locals.var_ai_dn10, locals.var_ai_dn11, locals.var_ai_dn12, locals.var_ai_dn17,)
    }
};
        locals.var_ai = assign16470_e24024;
        locals.var_ai_dn0 = assign16470_e24024_d_n0;
        locals.var_ai_dn2 = assign16470_e24024_d_n2;
        locals.var_ai_dn6 = assign16470_e24024_d_n6;
        locals.var_ai_dn7 = assign16470_e24024_d_n7;
        locals.var_ai_dn10 = assign16470_e24024_d_n10;
        locals.var_ai_dn11 = assign16470_e24024_d_n11;
        locals.var_ai_dn12 = assign16470_e24024_d_n12;
        locals.var_ai_dn17 = assign16470_e24024_d_n17;

        let (assign16480_e24033, assign16480_e24033_d_n0, assign16480_e24033_d_n2, assign16480_e24033_d_n6, assign16480_e24033_d_n7, assign16480_e24033_d_n10, assign16480_e24033_d_n11, assign16480_e24033_d_n12, assign16480_e24033_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16480_e24031: f64 = (locals.var_q_depl - locals.var_q_dep0);
        (assign16480_e24031, (locals.var_q_depl_dn0 - locals.var_q_dep0_dn0), (locals.var_q_depl_dn2 - locals.var_q_dep0_dn2), (locals.var_q_depl_dn6 - locals.var_q_dep0_dn6), (locals.var_q_depl_dn7 - locals.var_q_dep0_dn7), (locals.var_q_depl_dn10 - locals.var_q_dep0_dn10), (locals.var_q_depl_dn11 - locals.var_q_dep0_dn11), (locals.var_q_depl_dn12 - locals.var_q_dep0_dn12), (locals.var_q_depl_dn17 - locals.var_q_dep0_dn17),)
    } else {
        (locals.var_db, locals.var_db_dn0, locals.var_db_dn2, locals.var_db_dn6, locals.var_db_dn7, locals.var_db_dn10, locals.var_db_dn11, locals.var_db_dn12, locals.var_db_dn17,)
    }
};
        locals.var_db = assign16480_e24033;
        locals.var_db_dn0 = assign16480_e24033_d_n0;
        locals.var_db_dn2 = assign16480_e24033_d_n2;
        locals.var_db_dn6 = assign16480_e24033_d_n6;
        locals.var_db_dn7 = assign16480_e24033_d_n7;
        locals.var_db_dn10 = assign16480_e24033_d_n10;
        locals.var_db_dn11 = assign16480_e24033_d_n11;
        locals.var_db_dn12 = assign16480_e24033_d_n12;
        locals.var_db_dn17 = assign16480_e24033_d_n17;

        let (assign16490_e24043, assign16490_e24043_d_n0, assign16490_e24043_d_n2, assign16490_e24043_d_n6, assign16490_e24043_d_n7, assign16490_e24043_d_n10, assign16490_e24043_d_n11, assign16490_e24043_d_n12, assign16490_e24043_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16490_e24040: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign16490_e24041: f64 = (-assign16490_e24040);
        (assign16490_e24041, (-(locals.var_q_nl_dn0 - locals.var_q_n0_dn0)), (-(locals.var_q_nl_dn2 - locals.var_q_n0_dn2)), (-(locals.var_q_nl_dn6 - locals.var_q_n0_dn6)), (-(locals.var_q_nl_dn7 - locals.var_q_n0_dn7)), (-(locals.var_q_nl_dn10 - locals.var_q_n0_dn10)), (-(locals.var_q_nl_dn11 - locals.var_q_n0_dn11)), (-(locals.var_q_nl_dn12 - locals.var_q_n0_dn12)), (-(locals.var_q_nl_dn17 - locals.var_q_n0_dn17)),)
    } else {
        (locals.var_di, locals.var_di_dn0, locals.var_di_dn2, locals.var_di_dn6, locals.var_di_dn7, locals.var_di_dn10, locals.var_di_dn11, locals.var_di_dn12, locals.var_di_dn17,)
    }
};
        locals.var_di = assign16490_e24043;
        locals.var_di_dn0 = assign16490_e24043_d_n0;
        locals.var_di_dn2 = assign16490_e24043_d_n2;
        locals.var_di_dn6 = assign16490_e24043_d_n6;
        locals.var_di_dn7 = assign16490_e24043_d_n7;
        locals.var_di_dn10 = assign16490_e24043_d_n10;
        locals.var_di_dn11 = assign16490_e24043_d_n11;
        locals.var_di_dn12 = assign16490_e24043_d_n12;
        locals.var_di_dn17 = assign16490_e24043_d_n17;

        let (assign16500_e24052, assign16500_e24052_d_n0, assign16500_e24052_d_n2, assign16500_e24052_d_n6, assign16500_e24052_d_n7, assign16500_e24052_d_n10, assign16500_e24052_d_n11, assign16500_e24052_d_n12, assign16500_e24052_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16500_e24050: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        (assign16500_e24050, ((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)),)
    } else {
        (locals.var_c2, locals.var_c2_dn0, locals.var_c2_dn2, locals.var_c2_dn6, locals.var_c2_dn7, locals.var_c2_dn10, locals.var_c2_dn11, locals.var_c2_dn12, locals.var_c2_dn17,)
    }
};
        locals.var_c2 = assign16500_e24052;
        locals.var_c2_dn0 = assign16500_e24052_d_n0;
        locals.var_c2_dn2 = assign16500_e24052_d_n2;
        locals.var_c2_dn6 = assign16500_e24052_d_n6;
        locals.var_c2_dn7 = assign16500_e24052_d_n7;
        locals.var_c2_dn10 = assign16500_e24052_d_n10;
        locals.var_c2_dn11 = assign16500_e24052_d_n11;
        locals.var_c2_dn12 = assign16500_e24052_d_n12;
        locals.var_c2_dn17 = assign16500_e24052_d_n17;

        let assign16510_e24055: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard491 = assign16510_e24055;

    }

    pub(super) fn stamp_transient_block_56(
        locals: &mut StampLocals,
    ) {
        let (assign16520_e24080, assign16520_e24080_d_n0, assign16520_e24080_d_n2, assign16520_e24080_d_n6, assign16520_e24080_d_n7, assign16520_e24080_d_n10, assign16520_e24080_d_n11, assign16520_e24080_d_n12, assign16520_e24080_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard491 != 0.0)) {
        let assign16520_e24064: f64 = (locals.var_ai * locals.var_beta);
        let assign16520_e24066: f64 = (assign16520_e24064 * locals.var_pds);
        let assign16520_e24068: f64 = (assign16520_e24066 - locals.var_di);
        let assign16520_e24071: f64 = (locals.var_db * locals.var_db);
        let assign16520_e24073: f64 = (assign16520_e24071 * locals.var_db);
        let assign16520_e24075: f64 = (assign16520_e24073 / locals.var_c2);
        let assign16520_e24077: f64 = (assign16520_e24075 / 6.0);
        let assign16520_e24078: f64 = (assign16520_e24068 - assign16520_e24077);
        (assign16520_e24078, (((((locals.var_ai_dn0 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn0)) - locals.var_di_dn0) - ((((((((locals.var_db_dn0 * locals.var_db) + (locals.var_db * locals.var_db_dn0)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn0)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn2 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn2)) - locals.var_di_dn2) - ((((((((locals.var_db_dn2 * locals.var_db) + (locals.var_db * locals.var_db_dn2)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn2)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn6 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn6)) - locals.var_di_dn6) - ((((((((locals.var_db_dn6 * locals.var_db) + (locals.var_db * locals.var_db_dn6)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn6)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn7 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn7)) - locals.var_di_dn7) - ((((((((locals.var_db_dn7 * locals.var_db) + (locals.var_db * locals.var_db_dn7)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn7)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((locals.var_ai_dn10 * locals.var_beta) + (locals.var_ai * locals.var_beta_dn10)) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn10)) - locals.var_di_dn10) - ((((((((locals.var_db_dn10 * locals.var_db) + (locals.var_db * locals.var_db_dn10)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn10)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn11 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn11)) - locals.var_di_dn11) - ((((((((locals.var_db_dn11 * locals.var_db) + (locals.var_db * locals.var_db_dn11)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn11)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn12 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn12)) - locals.var_di_dn12) - ((((((((locals.var_db_dn12 * locals.var_db) + (locals.var_db * locals.var_db_dn12)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn12)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn17 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn17)) - locals.var_di_dn17) - ((((((((locals.var_db_dn17 * locals.var_db) + (locals.var_db * locals.var_db_dn17)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn17)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16520_e24080;
        locals.var_idd_dn0 = assign16520_e24080_d_n0;
        locals.var_idd_dn2 = assign16520_e24080_d_n2;
        locals.var_idd_dn6 = assign16520_e24080_d_n6;
        locals.var_idd_dn7 = assign16520_e24080_d_n7;
        locals.var_idd_dn10 = assign16520_e24080_d_n10;
        locals.var_idd_dn11 = assign16520_e24080_d_n11;
        locals.var_idd_dn12 = assign16520_e24080_d_n12;
        locals.var_idd_dn17 = assign16520_e24080_d_n17;

        let (assign16530_e24092, assign16530_e24092_d_n0, assign16530_e24092_d_n2, assign16530_e24092_d_n6, assign16530_e24092_d_n7, assign16530_e24092_d_n10, assign16530_e24092_d_n11, assign16530_e24092_d_n12, assign16530_e24092_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard491 == 0.0)) {
        let assign16530_e24090: f64 = (locals.var_pds * locals.var_fdd);
        (assign16530_e24090, ((locals.var_pds_dn0 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn0)), ((locals.var_pds_dn2 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn2)), ((locals.var_pds_dn6 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn6)), ((locals.var_pds_dn7 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn7)), ((locals.var_pds_dn10 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn10)), ((locals.var_pds_dn11 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn11)), ((locals.var_pds_dn12 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn12)), ((locals.var_pds_dn17 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn17)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16530_e24092;
        locals.var_idd_dn0 = assign16530_e24092_d_n0;
        locals.var_idd_dn2 = assign16530_e24092_d_n2;
        locals.var_idd_dn6 = assign16530_e24092_d_n6;
        locals.var_idd_dn7 = assign16530_e24092_d_n7;
        locals.var_idd_dn10 = assign16530_e24092_d_n10;
        locals.var_idd_dn11 = assign16530_e24092_d_n11;
        locals.var_idd_dn12 = assign16530_e24092_d_n12;
        locals.var_idd_dn17 = assign16530_e24092_d_n17;

        let assign16540_e24099: f64 = if ((locals.var_flg_info >= 1.0) && (locals.var_idd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard492 = assign16540_e24099;

        let (assign16550_e24108, assign16550_e24108_d_n0, assign16550_e24108_d_n2, assign16550_e24108_d_n6, assign16550_e24108_d_n7, assign16550_e24108_d_n10, assign16550_e24108_d_n11, assign16550_e24108_d_n12, assign16550_e24108_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard492 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16550_e24108;
        locals.var_idd_dn0 = assign16550_e24108_d_n0;
        locals.var_idd_dn2 = assign16550_e24108_d_n2;
        locals.var_idd_dn6 = assign16550_e24108_d_n6;
        locals.var_idd_dn7 = assign16550_e24108_d_n7;
        locals.var_idd_dn10 = assign16550_e24108_d_n10;
        locals.var_idd_dn11 = assign16550_e24108_d_n11;
        locals.var_idd_dn12 = assign16550_e24108_d_n12;
        locals.var_idd_dn17 = assign16550_e24108_d_n17;

        let assign16560_e24111: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard493 = assign16560_e24111;

        let assign16570_e24113: f64 = (locals.var_pds).abs();
        let assign16570_e24115: f64 = if assign16570_e24113 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard494 = assign16570_e24115;

        let (assign16580_e24172, assign16580_e24172_d_n0, assign16580_e24172_d_n2, assign16580_e24172_d_n6, assign16580_e24172_d_n7, assign16580_e24172_d_n10, assign16580_e24172_d_n11, assign16580_e24172_d_n12, assign16580_e24172_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard493 != 0.0)) && (locals.var_guard494 != 0.0)) {
        let assign16580_e24127: f64 = (locals.var_ai * locals.var_beta);
        let assign16580_e24129: f64 = (assign16580_e24127 * locals.var_pds);
        let assign16580_e24131: f64 = (assign16580_e24129 - locals.var_di);
        let assign16580_e24132: f64 = (locals.var_ab * assign16580_e24131);
        let assign16580_e24136: f64 = (2.0 * locals.var_ab);
        let assign16580_e24137: f64 = (locals.var_ai - assign16580_e24136);
        let assign16580_e24140: f64 = (locals.var_c_fox / locals.var_beta);
        let assign16580_e24144: f64 = (2.0 * locals.var_ab);
        let assign16580_e24146: f64 = (assign16580_e24144 * locals.var_ab);
        let assign16580_e24148: f64 = (assign16580_e24146 / locals.var_c2);
        let assign16580_e24149: f64 = (1.0 - assign16580_e24148);
        let assign16580_e24152: f64 = (locals.var_db * locals.var_db);
        let assign16580_e24154: f64 = (assign16580_e24152 / locals.var_c2);
        let assign16580_e24156: f64 = (assign16580_e24154 / 10.0);
        let assign16580_e24157: f64 = (assign16580_e24149 + assign16580_e24156);
        let assign16580_e24158: f64 = (assign16580_e24140 * assign16580_e24157);
        let assign16580_e24159: f64 = (assign16580_e24137 + assign16580_e24158);
        let assign16580_e24161: f64 = (assign16580_e24159 * locals.var_db);
        let assign16580_e24163: f64 = (assign16580_e24161 * locals.var_db);
        let assign16580_e24165: f64 = (assign16580_e24163 * locals.var_db);
        let assign16580_e24167: f64 = (assign16580_e24165 / locals.var_c2);
        let assign16580_e24169: f64 = (assign16580_e24167 / 6.0);
        let assign16580_e24170: f64 = (assign16580_e24132 + assign16580_e24169);
        (assign16580_e24170, (((locals.var_ab_dn0 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn0 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn0)) - locals.var_di_dn0))) + ((((((((((((locals.var_ai_dn0 - (2.0 * locals.var_ab_dn0)) + (((locals.var_c_fox_dn0 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn0) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn0)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn0 * locals.var_db) + (locals.var_db * locals.var_db_dn0)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn0)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn0)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn0)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn2 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn2 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn2)) - locals.var_di_dn2))) + ((((((((((((locals.var_ai_dn2 - (2.0 * locals.var_ab_dn2)) + (((locals.var_c_fox_dn2 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn2) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn2)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn2 * locals.var_db) + (locals.var_db * locals.var_db_dn2)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn2)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn2)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn2)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn6 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn6 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn6)) - locals.var_di_dn6))) + ((((((((((((locals.var_ai_dn6 - (2.0 * locals.var_ab_dn6)) + (((locals.var_c_fox_dn6 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn6) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn6)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn6 * locals.var_db) + (locals.var_db * locals.var_db_dn6)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn6)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn6)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn6)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn7 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn7 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn7)) - locals.var_di_dn7))) + ((((((((((((locals.var_ai_dn7 - (2.0 * locals.var_ab_dn7)) + (((locals.var_c_fox_dn7 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn7) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn7)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn7 * locals.var_db) + (locals.var_db * locals.var_db_dn7)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn7)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn7)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn7)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn10 * assign16580_e24131) + (locals.var_ab * (((((locals.var_ai_dn10 * locals.var_beta) + (locals.var_ai * locals.var_beta_dn10)) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn10)) - locals.var_di_dn10))) + ((((((((((((locals.var_ai_dn10 - (2.0 * locals.var_ab_dn10)) + (((((locals.var_c_fox_dn10 * locals.var_beta) - (locals.var_c_fox * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn10) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn10)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn10 * locals.var_db) + (locals.var_db * locals.var_db_dn10)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn10)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn10)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn10)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn11 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn11 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn11)) - locals.var_di_dn11))) + ((((((((((((locals.var_ai_dn11 - (2.0 * locals.var_ab_dn11)) + (((locals.var_c_fox_dn11 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn11) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn11)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn11 * locals.var_db) + (locals.var_db * locals.var_db_dn11)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn11)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn11)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn11)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn12 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn12 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn12)) - locals.var_di_dn12))) + ((((((((((((locals.var_ai_dn12 - (2.0 * locals.var_ab_dn12)) + (((locals.var_c_fox_dn12 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn12) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn12)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn12 * locals.var_db) + (locals.var_db * locals.var_db_dn12)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn12)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn12)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn12)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn17 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn17 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn17)) - locals.var_di_dn17))) + ((((((((((((locals.var_ai_dn17 - (2.0 * locals.var_ab_dn17)) + (((locals.var_c_fox_dn17 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn17) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn17)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn17 * locals.var_db) + (locals.var_db * locals.var_db_dn17)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn17)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn17)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn17)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16580_e24172;
        locals.var_qbu_dn0 = assign16580_e24172_d_n0;
        locals.var_qbu_dn2 = assign16580_e24172_d_n2;
        locals.var_qbu_dn6 = assign16580_e24172_d_n6;
        locals.var_qbu_dn7 = assign16580_e24172_d_n7;
        locals.var_qbu_dn10 = assign16580_e24172_d_n10;
        locals.var_qbu_dn11 = assign16580_e24172_d_n11;
        locals.var_qbu_dn12 = assign16580_e24172_d_n12;
        locals.var_qbu_dn17 = assign16580_e24172_d_n17;

        let (assign16590_e24185, assign16590_e24185_d_n0, assign16590_e24185_d_n2, assign16590_e24185_d_n6, assign16590_e24185_d_n7, assign16590_e24185_d_n10, assign16590_e24185_d_n11, assign16590_e24185_d_n12, assign16590_e24185_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard493 != 0.0)) && (locals.var_guard494 != 0.0)) {
        let assign16590_e24183: f64 = (locals.var_qbu / locals.var_idd);
        (assign16590_e24183, (((locals.var_qbu_dn0 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn0)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn2 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn2)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn6 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn6)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn7 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn7)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn10 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn10)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn11 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn11)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn12 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn12)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn17 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn17)) / (locals.var_idd * locals.var_idd)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16590_e24185;
        locals.var_qbu_dn0 = assign16590_e24185_d_n0;
        locals.var_qbu_dn2 = assign16590_e24185_d_n2;
        locals.var_qbu_dn6 = assign16590_e24185_d_n6;
        locals.var_qbu_dn7 = assign16590_e24185_d_n7;
        locals.var_qbu_dn10 = assign16590_e24185_d_n10;
        locals.var_qbu_dn11 = assign16590_e24185_d_n11;
        locals.var_qbu_dn12 = assign16590_e24185_d_n12;
        locals.var_qbu_dn17 = assign16590_e24185_d_n17;

        let (assign16600_e24197, assign16600_e24197_d_n0, assign16600_e24197_d_n2, assign16600_e24197_d_n6, assign16600_e24197_d_n7, assign16600_e24197_d_n10, assign16600_e24197_d_n11, assign16600_e24197_d_n12, assign16600_e24197_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard493 != 0.0)) && (locals.var_guard494 == 0.0)) {
        (locals.var_ab, locals.var_ab_dn0, locals.var_ab_dn2, locals.var_ab_dn6, locals.var_ab_dn7, locals.var_ab_dn10, locals.var_ab_dn11, locals.var_ab_dn12, locals.var_ab_dn17,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16600_e24197;
        locals.var_qbu_dn0 = assign16600_e24197_d_n0;
        locals.var_qbu_dn2 = assign16600_e24197_d_n2;
        locals.var_qbu_dn6 = assign16600_e24197_d_n6;
        locals.var_qbu_dn7 = assign16600_e24197_d_n7;
        locals.var_qbu_dn10 = assign16600_e24197_d_n10;
        locals.var_qbu_dn11 = assign16600_e24197_d_n11;
        locals.var_qbu_dn12 = assign16600_e24197_d_n12;
        locals.var_qbu_dn17 = assign16600_e24197_d_n17;

        let (assign16610_e24211, assign16610_e24211_d_n0, assign16610_e24211_d_n2, assign16610_e24211_d_n6, assign16610_e24211_d_n7, assign16610_e24211_d_n10, assign16610_e24211_d_n11, assign16610_e24211_d_n12, assign16610_e24211_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard493 == 0.0)) {
        let assign16610_e24208: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign16610_e24209: f64 = (0.5 * assign16610_e24208);
        (assign16610_e24209, (0.5 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (0.5 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (0.5 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (0.5 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (0.5 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (0.5 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (0.5 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (0.5 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16610_e24211;
        locals.var_qbu_dn0 = assign16610_e24211_d_n0;
        locals.var_qbu_dn2 = assign16610_e24211_d_n2;
        locals.var_qbu_dn6 = assign16610_e24211_d_n6;
        locals.var_qbu_dn7 = assign16610_e24211_d_n7;
        locals.var_qbu_dn10 = assign16610_e24211_d_n10;
        locals.var_qbu_dn11 = assign16610_e24211_d_n11;
        locals.var_qbu_dn12 = assign16610_e24211_d_n12;
        locals.var_qbu_dn17 = assign16610_e24211_d_n17;

        let (assign16620_e24220, assign16620_e24220_d_n0, assign16620_e24220_d_n2, assign16620_e24220_d_n6, assign16620_e24220_d_n7, assign16620_e24220_d_n10, assign16620_e24220_d_n11, assign16620_e24220_d_n12, assign16620_e24220_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16620_e24218: f64 = (2.0 * locals.var_fac1);
        (assign16620_e24218, (2.0 * locals.var_fac1_dn0), (2.0 * locals.var_fac1_dn2), (2.0 * locals.var_fac1_dn6), (2.0 * locals.var_fac1_dn7), (2.0 * locals.var_fac1_dn10), (2.0 * locals.var_fac1_dn11), (2.0 * locals.var_fac1_dn12), (2.0 * locals.var_fac1_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16620_e24220;
        locals.var_t1_dn0 = assign16620_e24220_d_n0;
        locals.var_t1_dn2 = assign16620_e24220_d_n2;
        locals.var_t1_dn6 = assign16620_e24220_d_n6;
        locals.var_t1_dn7 = assign16620_e24220_d_n7;
        locals.var_t1_dn10 = assign16620_e24220_d_n10;
        locals.var_t1_dn11 = assign16620_e24220_d_n11;
        locals.var_t1_dn12 = assign16620_e24220_d_n12;
        locals.var_t1_dn17 = assign16620_e24220_d_n17;

        let (assign16630_e24231, assign16630_e24231_d_n0, assign16630_e24231_d_n2, assign16630_e24231_d_n6, assign16630_e24231_d_n7, assign16630_e24231_d_n10, assign16630_e24231_d_n11, assign16630_e24231_d_n12, assign16630_e24231_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16630_e24228: f64 = (locals.var_f10 - locals.var_xi0p12);
        let assign16630_e24229: f64 = (locals.var_t1 * assign16630_e24228);
        (assign16630_e24229, ((locals.var_t1_dn0 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn0 - locals.var_xi0p12_dn0))), ((locals.var_t1_dn2 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn2 - locals.var_xi0p12_dn2))), ((locals.var_t1_dn6 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn6 - locals.var_xi0p12_dn6))), ((locals.var_t1_dn7 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn7 - locals.var_xi0p12_dn7))), ((locals.var_t1_dn10 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn10 - locals.var_xi0p12_dn10))), ((locals.var_t1_dn11 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn11 - locals.var_xi0p12_dn11))), ((locals.var_t1_dn12 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn12 - locals.var_xi0p12_dn12))), ((locals.var_t1_dn17 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn17 - locals.var_xi0p12_dn17))),)
    } else {
        (locals.var_dtpds, locals.var_dtpds_dn0, locals.var_dtpds_dn2, locals.var_dtpds_dn6, locals.var_dtpds_dn7, locals.var_dtpds_dn10, locals.var_dtpds_dn11, locals.var_dtpds_dn12, locals.var_dtpds_dn17,)
    }
};
        locals.var_dtpds = assign16630_e24231;
        locals.var_dtpds_dn0 = assign16630_e24231_d_n0;
        locals.var_dtpds_dn2 = assign16630_e24231_d_n2;
        locals.var_dtpds_dn6 = assign16630_e24231_d_n6;
        locals.var_dtpds_dn7 = assign16630_e24231_d_n7;
        locals.var_dtpds_dn10 = assign16630_e24231_d_n10;
        locals.var_dtpds_dn11 = assign16630_e24231_d_n11;
        locals.var_dtpds_dn12 = assign16630_e24231_d_n12;
        locals.var_dtpds_dn17 = assign16630_e24231_d_n17;

        let (assign16640_e24240, assign16640_e24240_d_n0, assign16640_e24240_d_n2, assign16640_e24240_d_n6, assign16640_e24240_d_n7, assign16640_e24240_d_n10, assign16640_e24240_d_n11, assign16640_e24240_d_n12, assign16640_e24240_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16640_e24238: f64 = (locals.var_pds + locals.var_dtpds);
        (assign16640_e24238, (locals.var_pds_dn0 + locals.var_dtpds_dn0), (locals.var_pds_dn2 + locals.var_dtpds_dn2), (locals.var_pds_dn6 + locals.var_dtpds_dn6), (locals.var_pds_dn7 + locals.var_dtpds_dn7), (locals.var_pds_dn10 + locals.var_dtpds_dn10), (locals.var_pds_dn11 + locals.var_dtpds_dn11), (locals.var_pds_dn12 + locals.var_dtpds_dn12), (locals.var_pds_dn17 + locals.var_dtpds_dn17),)
    } else {
        (locals.var_achi, locals.var_achi_dn0, locals.var_achi_dn2, locals.var_achi_dn6, locals.var_achi_dn7, locals.var_achi_dn10, locals.var_achi_dn11, locals.var_achi_dn12, locals.var_achi_dn17,)
    }
};
        locals.var_achi = assign16640_e24240;
        locals.var_achi_dn0 = assign16640_e24240_d_n0;
        locals.var_achi_dn2 = assign16640_e24240_d_n2;
        locals.var_achi_dn6 = assign16640_e24240_d_n6;
        locals.var_achi_dn7 = assign16640_e24240_d_n7;
        locals.var_achi_dn10 = assign16640_e24240_d_n10;
        locals.var_achi_dn11 = assign16640_e24240_d_n11;
        locals.var_achi_dn12 = assign16640_e24240_d_n12;
        locals.var_achi_dn17 = assign16640_e24240_d_n17;

        let (assign16650_e24249, assign16650_e24249_d_n0, assign16650_e24249_d_n2, assign16650_e24249_d_n6, assign16650_e24249_d_n7, assign16650_e24249_d_n10, assign16650_e24249_d_n11, assign16650_e24249_d_n12, assign16650_e24249_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16650_e24247: f64 = (1.0 / locals.var_vgvt);
        (assign16650_e24247, (-(locals.var_vgvt_dn0 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn2 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn6 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn7 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn10 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn11 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn12 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn17 / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16650_e24249;
        locals.var_t1_dn0 = assign16650_e24249_d_n0;
        locals.var_t1_dn2 = assign16650_e24249_d_n2;
        locals.var_t1_dn6 = assign16650_e24249_d_n6;
        locals.var_t1_dn7 = assign16650_e24249_d_n7;
        locals.var_t1_dn10 = assign16650_e24249_d_n10;
        locals.var_t1_dn11 = assign16650_e24249_d_n11;
        locals.var_t1_dn12 = assign16650_e24249_d_n12;
        locals.var_t1_dn17 = assign16650_e24249_d_n17;

        let (assign16660_e24258, assign16660_e24258_d_n0, assign16660_e24258_d_n2, assign16660_e24258_d_n6, assign16660_e24258_d_n7, assign16660_e24258_d_n10, assign16660_e24258_d_n11, assign16660_e24258_d_n12, assign16660_e24258_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16660_e24256: f64 = (locals.var_achi * locals.var_t1);
        (assign16660_e24256, ((locals.var_achi_dn0 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn0)), ((locals.var_achi_dn2 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn2)), ((locals.var_achi_dn6 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn6)), ((locals.var_achi_dn7 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn7)), ((locals.var_achi_dn10 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn10)), ((locals.var_achi_dn11 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn11)), ((locals.var_achi_dn12 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn12)), ((locals.var_achi_dn17 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign16660_e24258;
        locals.var_t2_dn0 = assign16660_e24258_d_n0;
        locals.var_t2_dn2 = assign16660_e24258_d_n2;
        locals.var_t2_dn6 = assign16660_e24258_d_n6;
        locals.var_t2_dn7 = assign16660_e24258_d_n7;
        locals.var_t2_dn10 = assign16660_e24258_d_n10;
        locals.var_t2_dn11 = assign16660_e24258_d_n11;
        locals.var_t2_dn12 = assign16660_e24258_d_n12;
        locals.var_t2_dn17 = assign16660_e24258_d_n17;

        let (assign16670_e24267, assign16670_e24267_d_n0, assign16670_e24267_d_n2, assign16670_e24267_d_n6, assign16670_e24267_d_n7, assign16670_e24267_d_n10, assign16670_e24267_d_n11, assign16670_e24267_d_n12, assign16670_e24267_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16670_e24265: f64 = (1.0 - locals.var_t2);
        (assign16670_e24265, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn12), (-locals.var_t2_dn17),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign16670_e24267;
        locals.var_t3_dn0 = assign16670_e24267_d_n0;
        locals.var_t3_dn2 = assign16670_e24267_d_n2;
        locals.var_t3_dn6 = assign16670_e24267_d_n6;
        locals.var_t3_dn7 = assign16670_e24267_d_n7;
        locals.var_t3_dn10 = assign16670_e24267_d_n10;
        locals.var_t3_dn11 = assign16670_e24267_d_n11;
        locals.var_t3_dn12 = assign16670_e24267_d_n12;
        locals.var_t3_dn17 = assign16670_e24267_d_n17;

        let (assign16680_e24276, assign16680_e24276_d_n0, assign16680_e24276_d_n2, assign16680_e24276_d_n6, assign16680_e24276_d_n7, assign16680_e24276_d_n10, assign16680_e24276_d_n11, assign16680_e24276_d_n12, assign16680_e24276_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16680_e24274: f64 = (1.0 - locals.var_t3);
        (assign16680_e24274, (-locals.var_t3_dn0), (-locals.var_t3_dn2), (-locals.var_t3_dn6), (-locals.var_t3_dn7), (-locals.var_t3_dn10), (-locals.var_t3_dn11), (-locals.var_t3_dn12), (-locals.var_t3_dn17),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign16680_e24276;
        locals.var_tx_dn0 = assign16680_e24276_d_n0;
        locals.var_tx_dn2 = assign16680_e24276_d_n2;
        locals.var_tx_dn6 = assign16680_e24276_d_n6;
        locals.var_tx_dn7 = assign16680_e24276_d_n7;
        locals.var_tx_dn10 = assign16680_e24276_d_n10;
        locals.var_tx_dn11 = assign16680_e24276_d_n11;
        locals.var_tx_dn12 = assign16680_e24276_d_n12;
        locals.var_tx_dn17 = assign16680_e24276_d_n17;

        let (assign16690_e24285, assign16690_e24285_d_n0, assign16690_e24285_d_n2, assign16690_e24285_d_n6, assign16690_e24285_d_n7, assign16690_e24285_d_n10, assign16690_e24285_d_n11, assign16690_e24285_d_n12, assign16690_e24285_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16690_e24283: f64 = (locals.var_tx * locals.var_tx);
        (assign16690_e24283, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn12 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn12)), ((locals.var_tx_dn17 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign16690_e24285;
        locals.var_x2_dn0 = assign16690_e24285_d_n0;
        locals.var_x2_dn2 = assign16690_e24285_d_n2;
        locals.var_x2_dn6 = assign16690_e24285_d_n6;
        locals.var_x2_dn7 = assign16690_e24285_d_n7;
        locals.var_x2_dn10 = assign16690_e24285_d_n10;
        locals.var_x2_dn11 = assign16690_e24285_d_n11;
        locals.var_x2_dn12 = assign16690_e24285_d_n12;
        locals.var_x2_dn17 = assign16690_e24285_d_n17;

        let (assign16700_e24294, assign16700_e24294_d_n0, assign16700_e24294_d_n2, assign16700_e24294_d_n6, assign16700_e24294_d_n7, assign16700_e24294_d_n10, assign16700_e24294_d_n11, assign16700_e24294_d_n12, assign16700_e24294_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16700_e24292: f64 = 1.0;
        (assign16700_e24292, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign16700_e24294;
        locals.var_xmax2_dn0 = assign16700_e24294_d_n0;
        locals.var_xmax2_dn2 = assign16700_e24294_d_n2;
        locals.var_xmax2_dn6 = assign16700_e24294_d_n6;
        locals.var_xmax2_dn7 = assign16700_e24294_d_n7;
        locals.var_xmax2_dn10 = assign16700_e24294_d_n10;
        locals.var_xmax2_dn11 = assign16700_e24294_d_n11;
        locals.var_xmax2_dn12 = assign16700_e24294_d_n12;
        locals.var_xmax2_dn17 = assign16700_e24294_d_n17;

        let (assign16710_e24301, assign16710_e24301_d_n0, assign16710_e24301_d_n2, assign16710_e24301_d_n6, assign16710_e24301_d_n7, assign16710_e24301_d_n10, assign16710_e24301_d_n11, assign16710_e24301_d_n12, assign16710_e24301_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16710_e24301;
        locals.var_xp_dn0 = assign16710_e24301_d_n0;
        locals.var_xp_dn2 = assign16710_e24301_d_n2;
        locals.var_xp_dn6 = assign16710_e24301_d_n6;
        locals.var_xp_dn7 = assign16710_e24301_d_n7;
        locals.var_xp_dn10 = assign16710_e24301_d_n10;
        locals.var_xp_dn11 = assign16710_e24301_d_n11;
        locals.var_xp_dn12 = assign16710_e24301_d_n12;
        locals.var_xp_dn17 = assign16710_e24301_d_n17;

        let (assign16720_e24308, assign16720_e24308_d_n0, assign16720_e24308_d_n2, assign16720_e24308_d_n6, assign16720_e24308_d_n7, assign16720_e24308_d_n10, assign16720_e24308_d_n11, assign16720_e24308_d_n12, assign16720_e24308_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16720_e24308;
        locals.var_xmp_dn0 = assign16720_e24308_d_n0;
        locals.var_xmp_dn2 = assign16720_e24308_d_n2;
        locals.var_xmp_dn6 = assign16720_e24308_d_n6;
        locals.var_xmp_dn7 = assign16720_e24308_d_n7;
        locals.var_xmp_dn10 = assign16720_e24308_d_n10;
        locals.var_xmp_dn11 = assign16720_e24308_d_n11;
        locals.var_xmp_dn12 = assign16720_e24308_d_n12;
        locals.var_xmp_dn17 = assign16720_e24308_d_n17;

        let (assign16730_e24315,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign16730_e24315;

        let (assign16740_e24322,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16740_e24322;

        let (assign16750_e24329, assign16750_e24329_d_n0, assign16750_e24329_d_n2, assign16750_e24329_d_n6, assign16750_e24329_d_n7, assign16750_e24329_d_n10, assign16750_e24329_d_n11, assign16750_e24329_d_n12, assign16750_e24329_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign16750_e24329;
        locals.var_arg_dn0 = assign16750_e24329_d_n0;
        locals.var_arg_dn2 = assign16750_e24329_d_n2;
        locals.var_arg_dn6 = assign16750_e24329_d_n6;
        locals.var_arg_dn7 = assign16750_e24329_d_n7;
        locals.var_arg_dn10 = assign16750_e24329_d_n10;
        locals.var_arg_dn11 = assign16750_e24329_d_n11;
        locals.var_arg_dn12 = assign16750_e24329_d_n12;
        locals.var_arg_dn17 = assign16750_e24329_d_n17;

        let (assign16760_e24336, assign16760_e24336_d_n0, assign16760_e24336_d_n2, assign16760_e24336_d_n6, assign16760_e24336_d_n7, assign16760_e24336_d_n10, assign16760_e24336_d_n11, assign16760_e24336_d_n12, assign16760_e24336_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16760_e24336;
        locals.var_dnm_dn0 = assign16760_e24336_d_n0;
        locals.var_dnm_dn2 = assign16760_e24336_d_n2;
        locals.var_dnm_dn6 = assign16760_e24336_d_n6;
        locals.var_dnm_dn7 = assign16760_e24336_d_n7;
        locals.var_dnm_dn10 = assign16760_e24336_d_n10;
        locals.var_dnm_dn11 = assign16760_e24336_d_n11;
        locals.var_dnm_dn12 = assign16760_e24336_d_n12;
        locals.var_dnm_dn17 = assign16760_e24336_d_n17;

        let (assign16770_e24345, assign16770_e24345_d_n0, assign16770_e24345_d_n2, assign16770_e24345_d_n6, assign16770_e24345_d_n7, assign16770_e24345_d_n10, assign16770_e24345_d_n11, assign16770_e24345_d_n12, assign16770_e24345_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16770_e24343: f64 = (locals.var_xp * locals.var_x2);
        (assign16770_e24343, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16770_e24345;
        locals.var_xp_dn0 = assign16770_e24345_d_n0;
        locals.var_xp_dn2 = assign16770_e24345_d_n2;
        locals.var_xp_dn6 = assign16770_e24345_d_n6;
        locals.var_xp_dn7 = assign16770_e24345_d_n7;
        locals.var_xp_dn10 = assign16770_e24345_d_n10;
        locals.var_xp_dn11 = assign16770_e24345_d_n11;
        locals.var_xp_dn12 = assign16770_e24345_d_n12;
        locals.var_xp_dn17 = assign16770_e24345_d_n17;

        let (assign16780_e24354, assign16780_e24354_d_n0, assign16780_e24354_d_n2, assign16780_e24354_d_n6, assign16780_e24354_d_n7, assign16780_e24354_d_n10, assign16780_e24354_d_n11, assign16780_e24354_d_n12, assign16780_e24354_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16780_e24352: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16780_e24352, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16780_e24354;
        locals.var_xmp_dn0 = assign16780_e24354_d_n0;
        locals.var_xmp_dn2 = assign16780_e24354_d_n2;
        locals.var_xmp_dn6 = assign16780_e24354_d_n6;
        locals.var_xmp_dn7 = assign16780_e24354_d_n7;
        locals.var_xmp_dn10 = assign16780_e24354_d_n10;
        locals.var_xmp_dn11 = assign16780_e24354_d_n11;
        locals.var_xmp_dn12 = assign16780_e24354_d_n12;
        locals.var_xmp_dn17 = assign16780_e24354_d_n17;

        let (assign16790_e24363, assign16790_e24363_d_n0, assign16790_e24363_d_n2, assign16790_e24363_d_n6, assign16790_e24363_d_n7, assign16790_e24363_d_n10, assign16790_e24363_d_n11, assign16790_e24363_d_n12, assign16790_e24363_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16790_e24361: f64 = (locals.var_xp * locals.var_x2);
        (assign16790_e24361, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16790_e24363;
        locals.var_xp_dn0 = assign16790_e24363_d_n0;
        locals.var_xp_dn2 = assign16790_e24363_d_n2;
        locals.var_xp_dn6 = assign16790_e24363_d_n6;
        locals.var_xp_dn7 = assign16790_e24363_d_n7;
        locals.var_xp_dn10 = assign16790_e24363_d_n10;
        locals.var_xp_dn11 = assign16790_e24363_d_n11;
        locals.var_xp_dn12 = assign16790_e24363_d_n12;
        locals.var_xp_dn17 = assign16790_e24363_d_n17;

        let (assign16800_e24372, assign16800_e24372_d_n0, assign16800_e24372_d_n2, assign16800_e24372_d_n6, assign16800_e24372_d_n7, assign16800_e24372_d_n10, assign16800_e24372_d_n11, assign16800_e24372_d_n12, assign16800_e24372_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16800_e24370: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16800_e24370, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16800_e24372;
        locals.var_xmp_dn0 = assign16800_e24372_d_n0;
        locals.var_xmp_dn2 = assign16800_e24372_d_n2;
        locals.var_xmp_dn6 = assign16800_e24372_d_n6;
        locals.var_xmp_dn7 = assign16800_e24372_d_n7;
        locals.var_xmp_dn10 = assign16800_e24372_d_n10;
        locals.var_xmp_dn11 = assign16800_e24372_d_n11;
        locals.var_xmp_dn12 = assign16800_e24372_d_n12;
        locals.var_xmp_dn17 = assign16800_e24372_d_n17;

        let (assign16810_e24381, assign16810_e24381_d_n0, assign16810_e24381_d_n2, assign16810_e24381_d_n6, assign16810_e24381_d_n7, assign16810_e24381_d_n10, assign16810_e24381_d_n11, assign16810_e24381_d_n12, assign16810_e24381_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16810_e24379: f64 = (locals.var_xp * locals.var_x2);
        (assign16810_e24379, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16810_e24381;
        locals.var_xp_dn0 = assign16810_e24381_d_n0;
        locals.var_xp_dn2 = assign16810_e24381_d_n2;
        locals.var_xp_dn6 = assign16810_e24381_d_n6;
        locals.var_xp_dn7 = assign16810_e24381_d_n7;
        locals.var_xp_dn10 = assign16810_e24381_d_n10;
        locals.var_xp_dn11 = assign16810_e24381_d_n11;
        locals.var_xp_dn12 = assign16810_e24381_d_n12;
        locals.var_xp_dn17 = assign16810_e24381_d_n17;

    }

    pub(super) fn stamp_transient_block_57(
        locals: &mut StampLocals,
    ) {
        let (assign16820_e24390, assign16820_e24390_d_n0, assign16820_e24390_d_n2, assign16820_e24390_d_n6, assign16820_e24390_d_n7, assign16820_e24390_d_n10, assign16820_e24390_d_n11, assign16820_e24390_d_n12, assign16820_e24390_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16820_e24388: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16820_e24388, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16820_e24390;
        locals.var_xmp_dn0 = assign16820_e24390_d_n0;
        locals.var_xmp_dn2 = assign16820_e24390_d_n2;
        locals.var_xmp_dn6 = assign16820_e24390_d_n6;
        locals.var_xmp_dn7 = assign16820_e24390_d_n7;
        locals.var_xmp_dn10 = assign16820_e24390_d_n10;
        locals.var_xmp_dn11 = assign16820_e24390_d_n11;
        locals.var_xmp_dn12 = assign16820_e24390_d_n12;
        locals.var_xmp_dn17 = assign16820_e24390_d_n17;

        let (assign16830_e24399, assign16830_e24399_d_n0, assign16830_e24399_d_n2, assign16830_e24399_d_n6, assign16830_e24399_d_n7, assign16830_e24399_d_n10, assign16830_e24399_d_n11, assign16830_e24399_d_n12, assign16830_e24399_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16830_e24397: f64 = (locals.var_xp * locals.var_x2);
        (assign16830_e24397, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16830_e24399;
        locals.var_xp_dn0 = assign16830_e24399_d_n0;
        locals.var_xp_dn2 = assign16830_e24399_d_n2;
        locals.var_xp_dn6 = assign16830_e24399_d_n6;
        locals.var_xp_dn7 = assign16830_e24399_d_n7;
        locals.var_xp_dn10 = assign16830_e24399_d_n10;
        locals.var_xp_dn11 = assign16830_e24399_d_n11;
        locals.var_xp_dn12 = assign16830_e24399_d_n12;
        locals.var_xp_dn17 = assign16830_e24399_d_n17;

        let (assign16840_e24408, assign16840_e24408_d_n0, assign16840_e24408_d_n2, assign16840_e24408_d_n6, assign16840_e24408_d_n7, assign16840_e24408_d_n10, assign16840_e24408_d_n11, assign16840_e24408_d_n12, assign16840_e24408_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16840_e24406: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16840_e24406, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16840_e24408;
        locals.var_xmp_dn0 = assign16840_e24408_d_n0;
        locals.var_xmp_dn2 = assign16840_e24408_d_n2;
        locals.var_xmp_dn6 = assign16840_e24408_d_n6;
        locals.var_xmp_dn7 = assign16840_e24408_d_n7;
        locals.var_xmp_dn10 = assign16840_e24408_d_n10;
        locals.var_xmp_dn11 = assign16840_e24408_d_n11;
        locals.var_xmp_dn12 = assign16840_e24408_d_n12;
        locals.var_xmp_dn17 = assign16840_e24408_d_n17;

        let (assign16850_e24417, assign16850_e24417_d_n0, assign16850_e24417_d_n2, assign16850_e24417_d_n6, assign16850_e24417_d_n7, assign16850_e24417_d_n10, assign16850_e24417_d_n11, assign16850_e24417_d_n12, assign16850_e24417_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16850_e24415: f64 = (locals.var_xp + locals.var_xmp);
        (assign16850_e24415, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign16850_e24417;
        locals.var_arg_dn0 = assign16850_e24417_d_n0;
        locals.var_arg_dn2 = assign16850_e24417_d_n2;
        locals.var_arg_dn6 = assign16850_e24417_d_n6;
        locals.var_arg_dn7 = assign16850_e24417_d_n7;
        locals.var_arg_dn10 = assign16850_e24417_d_n10;
        locals.var_arg_dn11 = assign16850_e24417_d_n11;
        locals.var_arg_dn12 = assign16850_e24417_d_n12;
        locals.var_arg_dn17 = assign16850_e24417_d_n17;

        let (assign16860_e24424, assign16860_e24424_d_n0, assign16860_e24424_d_n2, assign16860_e24424_d_n6, assign16860_e24424_d_n7, assign16860_e24424_d_n10, assign16860_e24424_d_n11, assign16860_e24424_d_n12, assign16860_e24424_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16860_e24424;
        locals.var_dnm_dn0 = assign16860_e24424_d_n0;
        locals.var_dnm_dn2 = assign16860_e24424_d_n2;
        locals.var_dnm_dn6 = assign16860_e24424_d_n6;
        locals.var_dnm_dn7 = assign16860_e24424_d_n7;
        locals.var_dnm_dn10 = assign16860_e24424_d_n10;
        locals.var_dnm_dn11 = assign16860_e24424_d_n11;
        locals.var_dnm_dn12 = assign16860_e24424_d_n12;
        locals.var_dnm_dn17 = assign16860_e24424_d_n17;

        let assign16870_e24439: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard495 = assign16870_e24439;

        let assign16880_e24442: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard496 = assign16880_e24442;

        let (assign16890_e24453,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16890_e24453;

        let assign16900_e24456: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard497 = assign16900_e24456;

        let (assign16910_e24470,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 == 0.0)) && (locals.var_guard497 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16910_e24470;

        let assign16920_e24473: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign16920_e24473;

        let (assign16930_e24490,) = {
    if ((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 == 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard498 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16930_e24490;

        let assign16940_e24493: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard499 = assign16940_e24493;

        let (assign16950_e24513,) = {
    if (((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 == 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard499 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16950_e24513;

        let (assign16960_e24522,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign16960_e24522;

        let mut assign16970_loop_guard: usize = 0;
        while {
            let assign16970_cond_e24532: f64 = if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign16970_cond_e24532 != 0.0
        } {
            assign16970_loop_guard += 1;
            assert!(assign16970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign16970_body0_e24542, assign16970_body0_e24542_d_n0, assign16970_body0_e24542_d_n2, assign16970_body0_e24542_d_n6, assign16970_body0_e24542_d_n7, assign16970_body0_e24542_d_n10, assign16970_body0_e24542_d_n11, assign16970_body0_e24542_d_n12, assign16970_body0_e24542_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) {
        let assign16970_body0_e24540: f64 = (locals.var_dnm).sqrt();
        (assign16970_body0_e24540, (locals.var_dnm_dn0 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn2 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn6 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn7 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn10 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn11 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn12 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn17 / (2.0 * assign16970_body0_e24540)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign16970_body0_e24542;
            locals.var_dnm_dn0 = assign16970_body0_e24542_d_n0;
            locals.var_dnm_dn2 = assign16970_body0_e24542_d_n2;
            locals.var_dnm_dn6 = assign16970_body0_e24542_d_n6;
            locals.var_dnm_dn7 = assign16970_body0_e24542_d_n7;
            locals.var_dnm_dn10 = assign16970_body0_e24542_d_n10;
            locals.var_dnm_dn11 = assign16970_body0_e24542_d_n11;
            locals.var_dnm_dn12 = assign16970_body0_e24542_d_n12;
            locals.var_dnm_dn17 = assign16970_body0_e24542_d_n17;
            let (assign16970_body1_e24553,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) {
        let assign16970_body1_e24551: f64 = (locals.var_m0 + 1.0);
        (assign16970_body1_e24551,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign16970_body1_e24553;
        }

        let (assign16980_e24569, assign16980_e24569_d_n0, assign16980_e24569_d_n2, assign16980_e24569_d_n6, assign16980_e24569_d_n7, assign16980_e24569_d_n10, assign16980_e24569_d_n11, assign16980_e24569_d_n12, assign16980_e24569_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 == 0.0)) {
        let assign16980_e24565: f64 = (2.0 * 4.0);
        let assign16980_e24566: f64 = (1.0 / assign16980_e24565);
        let assign16980_e24567: f64 = (locals.var_dnm).powf(assign16980_e24566);
        (assign16980_e24567, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn0)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn2)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn6)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn7)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn10)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn11)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn12)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn17)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16980_e24569;
        locals.var_dnm_dn0 = assign16980_e24569_d_n0;
        locals.var_dnm_dn2 = assign16980_e24569_d_n2;
        locals.var_dnm_dn6 = assign16980_e24569_d_n6;
        locals.var_dnm_dn7 = assign16980_e24569_d_n7;
        locals.var_dnm_dn10 = assign16980_e24569_d_n10;
        locals.var_dnm_dn11 = assign16980_e24569_d_n11;
        locals.var_dnm_dn12 = assign16980_e24569_d_n12;
        locals.var_dnm_dn17 = assign16980_e24569_d_n17;

        let (assign16990_e24578, assign16990_e24578_d_n0, assign16990_e24578_d_n2, assign16990_e24578_d_n6, assign16990_e24578_d_n7, assign16990_e24578_d_n10, assign16990_e24578_d_n11, assign16990_e24578_d_n12, assign16990_e24578_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16990_e24576: f64 = (1.0 / locals.var_dnm);
        (assign16990_e24576, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16990_e24578;
        locals.var_dnm_dn0 = assign16990_e24578_d_n0;
        locals.var_dnm_dn2 = assign16990_e24578_d_n2;
        locals.var_dnm_dn6 = assign16990_e24578_d_n6;
        locals.var_dnm_dn7 = assign16990_e24578_d_n7;
        locals.var_dnm_dn10 = assign16990_e24578_d_n10;
        locals.var_dnm_dn11 = assign16990_e24578_d_n11;
        locals.var_dnm_dn12 = assign16990_e24578_d_n12;
        locals.var_dnm_dn17 = assign16990_e24578_d_n17;

        let (assign17000_e24589, assign17000_e24589_d_n0, assign17000_e24589_d_n2, assign17000_e24589_d_n6, assign17000_e24589_d_n7, assign17000_e24589_d_n10, assign17000_e24589_d_n11, assign17000_e24589_d_n12, assign17000_e24589_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17000_e24585: f64 = locals.var_tx;
        let assign17000_e24587: f64 = (assign17000_e24585 * locals.var_dnm);
        (assign17000_e24587, ((locals.var_tx_dn0 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn2)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn7)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn11)), ((locals.var_tx_dn12 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn12)), ((locals.var_tx_dn17 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign17000_e24589;
        locals.var_ty_dn0 = assign17000_e24589_d_n0;
        locals.var_ty_dn2 = assign17000_e24589_d_n2;
        locals.var_ty_dn6 = assign17000_e24589_d_n6;
        locals.var_ty_dn7 = assign17000_e24589_d_n7;
        locals.var_ty_dn10 = assign17000_e24589_d_n10;
        locals.var_ty_dn11 = assign17000_e24589_d_n11;
        locals.var_ty_dn12 = assign17000_e24589_d_n12;
        locals.var_ty_dn17 = assign17000_e24589_d_n17;

        let (assign17010_e24598, assign17010_e24598_d_n0, assign17010_e24598_d_n2, assign17010_e24598_d_n6, assign17010_e24598_d_n7, assign17010_e24598_d_n10, assign17010_e24598_d_n11, assign17010_e24598_d_n12, assign17010_e24598_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17010_e24596: f64 = (1.0 - locals.var_ty);
        (assign17010_e24596, (-locals.var_ty_dn0), (-locals.var_ty_dn2), (-locals.var_ty_dn6), (-locals.var_ty_dn7), (-locals.var_ty_dn10), (-locals.var_ty_dn11), (-locals.var_ty_dn12), (-locals.var_ty_dn17),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign17010_e24598;
        locals.var_alpha_dn0 = assign17010_e24598_d_n0;
        locals.var_alpha_dn2 = assign17010_e24598_d_n2;
        locals.var_alpha_dn6 = assign17010_e24598_d_n6;
        locals.var_alpha_dn7 = assign17010_e24598_d_n7;
        locals.var_alpha_dn10 = assign17010_e24598_d_n10;
        locals.var_alpha_dn11 = assign17010_e24598_d_n11;
        locals.var_alpha_dn12 = assign17010_e24598_d_n12;
        locals.var_alpha_dn17 = assign17010_e24598_d_n17;

        let (assign17020_e24611, assign17020_e24611_d_n0, assign17020_e24611_d_n2, assign17020_e24611_d_n6, assign17020_e24611_d_n7, assign17020_e24611_d_n10, assign17020_e24611_d_n11, assign17020_e24611_d_n12, assign17020_e24611_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17020_e24607: f64 = (1.0 + locals.var_alpha);
        let assign17020_e24608: f64 = (locals.var_alpha * assign17020_e24607);
        let assign17020_e24609: f64 = (1.0 + assign17020_e24608);
        (assign17020_e24609, ((locals.var_alpha_dn0 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign17020_e24611;
        locals.var_qinm_dn0 = assign17020_e24611_d_n0;
        locals.var_qinm_dn2 = assign17020_e24611_d_n2;
        locals.var_qinm_dn6 = assign17020_e24611_d_n6;
        locals.var_qinm_dn7 = assign17020_e24611_d_n7;
        locals.var_qinm_dn10 = assign17020_e24611_d_n10;
        locals.var_qinm_dn11 = assign17020_e24611_d_n11;
        locals.var_qinm_dn12 = assign17020_e24611_d_n12;
        locals.var_qinm_dn17 = assign17020_e24611_d_n17;

        let (assign17030_e24631, assign17030_e24631_d_n0, assign17030_e24631_d_n2, assign17030_e24631_d_n6, assign17030_e24631_d_n7, assign17030_e24631_d_n10, assign17030_e24631_d_n11, assign17030_e24631_d_n12, assign17030_e24631_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17030_e24618: f64 = (1.0 + locals.var_alpha);
        let assign17030_e24621: f64 = (10.0 * 2.220446049250313e-16);
        let (assign17030_e24629, assign17030_e24629_d_n0, assign17030_e24629_d_n2, assign17030_e24629_d_n6, assign17030_e24629_d_n7, assign17030_e24629_d_n10, assign17030_e24629_d_n11, assign17030_e24629_d_n12, assign17030_e24629_d_n17,) = {
            if (assign17030_e24618 >= assign17030_e24621) {
                let assign17030_e24625: f64 = (1.0 + locals.var_alpha);
                (assign17030_e24625, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign17030_e24628: f64 = (10.0 * 2.220446049250313e-16);
                (assign17030_e24628, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign17030_e24629, assign17030_e24629_d_n0, assign17030_e24629_d_n2, assign17030_e24629_d_n6, assign17030_e24629_d_n7, assign17030_e24629_d_n10, assign17030_e24629_d_n11, assign17030_e24629_d_n12, assign17030_e24629_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign17030_e24631;
        locals.var_qidn_dn0 = assign17030_e24631_d_n0;
        locals.var_qidn_dn2 = assign17030_e24631_d_n2;
        locals.var_qidn_dn6 = assign17030_e24631_d_n6;
        locals.var_qidn_dn7 = assign17030_e24631_d_n7;
        locals.var_qidn_dn10 = assign17030_e24631_d_n10;
        locals.var_qidn_dn11 = assign17030_e24631_d_n11;
        locals.var_qidn_dn12 = assign17030_e24631_d_n12;
        locals.var_qidn_dn17 = assign17030_e24631_d_n17;

        let (assign17040_e24644, assign17040_e24644_d_n0, assign17040_e24644_d_n2, assign17040_e24644_d_n6, assign17040_e24644_d_n7, assign17040_e24644_d_n10, assign17040_e24644_d_n11, assign17040_e24644_d_n12, assign17040_e24644_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17040_e24638: f64 = (0.6666666666666667 * locals.var_vgvt);
        let assign17040_e24640: f64 = (assign17040_e24638 * locals.var_qinm);
        let assign17040_e24642: f64 = (assign17040_e24640 / locals.var_qidn);
        (assign17040_e24642, ((((((0.6666666666666667 * locals.var_vgvt_dn0) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn0)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn0)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn2) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn2)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn2)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn6) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn6)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn6)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn7) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn7)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn7)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn10) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn10)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn10)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn11) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn11)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn11)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn12) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn12)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn12)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn17) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn17)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn17)) / (locals.var_qidn * locals.var_qidn)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign17040_e24644;
        locals.var_t1_dn0 = assign17040_e24644_d_n0;
        locals.var_t1_dn2 = assign17040_e24644_d_n2;
        locals.var_t1_dn6 = assign17040_e24644_d_n6;
        locals.var_t1_dn7 = assign17040_e24644_d_n7;
        locals.var_t1_dn10 = assign17040_e24644_d_n10;
        locals.var_t1_dn11 = assign17040_e24644_d_n11;
        locals.var_t1_dn12 = assign17040_e24644_d_n12;
        locals.var_t1_dn17 = assign17040_e24644_d_n17;

        let assign17050_e24647: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign17050_e24647;

        let assign17060_e24649: f64 = (locals.var_pds).abs();
        let assign17060_e24651: f64 = if assign17060_e24649 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign17060_e24651;

        let (assign17070_e24704, assign17070_e24704_d_n0, assign17070_e24704_d_n2, assign17070_e24704_d_n6, assign17070_e24704_d_n7, assign17070_e24704_d_n10, assign17070_e24704_d_n11, assign17070_e24704_d_n12, assign17070_e24704_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 != 0.0)) {
        let assign17070_e24662: f64 = (locals.var_ai * locals.var_ai);
        let assign17070_e24665: f64 = (locals.var_di * locals.var_di);
        let assign17070_e24667: f64 = (assign17070_e24665 / 12.0);
        let assign17070_e24668: f64 = (assign17070_e24662 + assign17070_e24667);
        let assign17070_e24670: f64 = (assign17070_e24668 * locals.var_beta);
        let assign17070_e24672: f64 = (assign17070_e24670 * locals.var_pds);
        let assign17070_e24675: f64 = (locals.var_ai * locals.var_di);
        let assign17070_e24676: f64 = (assign17070_e24672 - assign17070_e24675);
        let assign17070_e24679: f64 = (2.0 * locals.var_ai);
        let assign17070_e24682: f64 = (locals.var_c_fox / locals.var_beta);
        let assign17070_e24684: f64 = (assign17070_e24682 * locals.var_db);
        let assign17070_e24686: f64 = (assign17070_e24684 * locals.var_db);
        let assign17070_e24688: f64 = (assign17070_e24686 / locals.var_c2);
        let assign17070_e24690: f64 = (assign17070_e24688 / 5.0);
        let assign17070_e24691: f64 = (assign17070_e24679 + assign17070_e24690);
        let assign17070_e24693: f64 = (assign17070_e24691 * locals.var_db);
        let assign17070_e24695: f64 = (assign17070_e24693 * locals.var_db);
        let assign17070_e24697: f64 = (assign17070_e24695 * locals.var_db);
        let assign17070_e24699: f64 = (assign17070_e24697 / locals.var_c2);
        let assign17070_e24701: f64 = (assign17070_e24699 / 6.0);
        let assign17070_e24702: f64 = (assign17070_e24676 - assign17070_e24701);
        (assign17070_e24702, ((((((((locals.var_ai_dn0 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn0)) + (((locals.var_di_dn0 * locals.var_di) + (locals.var_di * locals.var_di_dn0)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn0)) - ((locals.var_ai_dn0 * locals.var_di) + (locals.var_ai * locals.var_di_dn0))) - ((((((((((((2.0 * locals.var_ai_dn0) + (((((((((locals.var_c_fox_dn0 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn0)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn0)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn0)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn0)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn0)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn2 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn2)) + (((locals.var_di_dn2 * locals.var_di) + (locals.var_di * locals.var_di_dn2)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn2)) - ((locals.var_ai_dn2 * locals.var_di) + (locals.var_ai * locals.var_di_dn2))) - ((((((((((((2.0 * locals.var_ai_dn2) + (((((((((locals.var_c_fox_dn2 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn2)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn2)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn2)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn2)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn2)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn6 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn6)) + (((locals.var_di_dn6 * locals.var_di) + (locals.var_di * locals.var_di_dn6)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn6)) - ((locals.var_ai_dn6 * locals.var_di) + (locals.var_ai * locals.var_di_dn6))) - ((((((((((((2.0 * locals.var_ai_dn6) + (((((((((locals.var_c_fox_dn6 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn6)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn6)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn6)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn6)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn6)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn7 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn7)) + (((locals.var_di_dn7 * locals.var_di) + (locals.var_di * locals.var_di_dn7)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn7)) - ((locals.var_ai_dn7 * locals.var_di) + (locals.var_ai * locals.var_di_dn7))) - ((((((((((((2.0 * locals.var_ai_dn7) + (((((((((locals.var_c_fox_dn7 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn7)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn7)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn7)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn7)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn7)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((((((locals.var_ai_dn10 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn10)) + (((locals.var_di_dn10 * locals.var_di) + (locals.var_di * locals.var_di_dn10)) / 12.0)) * locals.var_beta) + (assign17070_e24668 * locals.var_beta_dn10)) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn10)) - ((locals.var_ai_dn10 * locals.var_di) + (locals.var_ai * locals.var_di_dn10))) - ((((((((((((2.0 * locals.var_ai_dn10) + (((((((((((locals.var_c_fox_dn10 * locals.var_beta) - (locals.var_c_fox * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn10)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn10)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn10)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn10)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn10)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn11 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn11)) + (((locals.var_di_dn11 * locals.var_di) + (locals.var_di * locals.var_di_dn11)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn11)) - ((locals.var_ai_dn11 * locals.var_di) + (locals.var_ai * locals.var_di_dn11))) - ((((((((((((2.0 * locals.var_ai_dn11) + (((((((((locals.var_c_fox_dn11 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn11)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn11)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn11)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn11)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn11)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn12 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn12)) + (((locals.var_di_dn12 * locals.var_di) + (locals.var_di * locals.var_di_dn12)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn12)) - ((locals.var_ai_dn12 * locals.var_di) + (locals.var_ai * locals.var_di_dn12))) - ((((((((((((2.0 * locals.var_ai_dn12) + (((((((((locals.var_c_fox_dn12 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn12)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn12)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn12)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn12)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn12)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn17 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn17)) + (((locals.var_di_dn17 * locals.var_di) + (locals.var_di * locals.var_di_dn17)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn17)) - ((locals.var_ai_dn17 * locals.var_di) + (locals.var_ai * locals.var_di_dn17))) - ((((((((((((2.0 * locals.var_ai_dn17) + (((((((((locals.var_c_fox_dn17 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn17)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn17)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn17)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn17)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn17)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17070_e24704;
        locals.var_qiu_dn0 = assign17070_e24704_d_n0;
        locals.var_qiu_dn2 = assign17070_e24704_d_n2;
        locals.var_qiu_dn6 = assign17070_e24704_d_n6;
        locals.var_qiu_dn7 = assign17070_e24704_d_n7;
        locals.var_qiu_dn10 = assign17070_e24704_d_n10;
        locals.var_qiu_dn11 = assign17070_e24704_d_n11;
        locals.var_qiu_dn12 = assign17070_e24704_d_n12;
        locals.var_qiu_dn17 = assign17070_e24704_d_n17;

        let (assign17080_e24717, assign17080_e24717_d_n0, assign17080_e24717_d_n2, assign17080_e24717_d_n6, assign17080_e24717_d_n7, assign17080_e24717_d_n10, assign17080_e24717_d_n11, assign17080_e24717_d_n12, assign17080_e24717_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 != 0.0)) {
        let assign17080_e24715: f64 = (locals.var_qiu / locals.var_idd);
        (assign17080_e24715, (((locals.var_qiu_dn0 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn0)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn2 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn2)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn6 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn6)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn7 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn7)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn10 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn10)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn11 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn11)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn12 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn12)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn17 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn17)) / (locals.var_idd * locals.var_idd)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17080_e24717;
        locals.var_qiu_dn0 = assign17080_e24717_d_n0;
        locals.var_qiu_dn2 = assign17080_e24717_d_n2;
        locals.var_qiu_dn6 = assign17080_e24717_d_n6;
        locals.var_qiu_dn7 = assign17080_e24717_d_n7;
        locals.var_qiu_dn10 = assign17080_e24717_d_n10;
        locals.var_qiu_dn11 = assign17080_e24717_d_n11;
        locals.var_qiu_dn12 = assign17080_e24717_d_n12;
        locals.var_qiu_dn17 = assign17080_e24717_d_n17;

        let (assign17090_e24729, assign17090_e24729_d_n0, assign17090_e24729_d_n2, assign17090_e24729_d_n6, assign17090_e24729_d_n7, assign17090_e24729_d_n10, assign17090_e24729_d_n11, assign17090_e24729_d_n12, assign17090_e24729_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 == 0.0)) {
        (locals.var_ai, locals.var_ai_dn0, locals.var_ai_dn2, locals.var_ai_dn6, locals.var_ai_dn7, locals.var_ai_dn10, locals.var_ai_dn11, locals.var_ai_dn12, locals.var_ai_dn17,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17090_e24729;
        locals.var_qiu_dn0 = assign17090_e24729_d_n0;
        locals.var_qiu_dn2 = assign17090_e24729_d_n2;
        locals.var_qiu_dn6 = assign17090_e24729_d_n6;
        locals.var_qiu_dn7 = assign17090_e24729_d_n7;
        locals.var_qiu_dn10 = assign17090_e24729_d_n10;
        locals.var_qiu_dn11 = assign17090_e24729_d_n11;
        locals.var_qiu_dn12 = assign17090_e24729_d_n12;
        locals.var_qiu_dn17 = assign17090_e24729_d_n17;

        let (assign17100_e24744, assign17100_e24744_d_n0, assign17100_e24744_d_n2, assign17100_e24744_d_n6, assign17100_e24744_d_n7, assign17100_e24744_d_n10, assign17100_e24744_d_n11, assign17100_e24744_d_n12, assign17100_e24744_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard500 == 0.0)) {
        let assign17100_e24738: f64 = (-0.5);
        let assign17100_e24741: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign17100_e24742: f64 = (assign17100_e24738 * assign17100_e24741);
        (assign17100_e24742, (assign17100_e24738 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign17100_e24738 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign17100_e24738 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign17100_e24738 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign17100_e24738 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign17100_e24738 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign17100_e24738 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign17100_e24738 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17100_e24744;
        locals.var_qiu_dn0 = assign17100_e24744_d_n0;
        locals.var_qiu_dn2 = assign17100_e24744_d_n2;
        locals.var_qiu_dn6 = assign17100_e24744_d_n6;
        locals.var_qiu_dn7 = assign17100_e24744_d_n7;
        locals.var_qiu_dn10 = assign17100_e24744_d_n10;
        locals.var_qiu_dn11 = assign17100_e24744_d_n11;
        locals.var_qiu_dn12 = assign17100_e24744_d_n12;
        locals.var_qiu_dn17 = assign17100_e24744_d_n17;

        let assign17140_e24758: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard505 = assign17140_e24758;

        let (assign17150_e24764, assign17150_e24764_d_n0, assign17150_e24764_d_n2, assign17150_e24764_d_n6, assign17150_e24764_d_n7, assign17150_e24764_d_n10, assign17150_e24764_d_n11, assign17150_e24764_d_n12, assign17150_e24764_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17150_e24762: f64 = (0.5 + locals.var_alpha);
        (assign17150_e24762, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    } else {
        (locals.var_qdnm, locals.var_qdnm_dn0, locals.var_qdnm_dn2, locals.var_qdnm_dn6, locals.var_qdnm_dn7, locals.var_qdnm_dn10, locals.var_qdnm_dn11, locals.var_qdnm_dn12, locals.var_qdnm_dn17,)
    }
};
        locals.var_qdnm = assign17150_e24764;
        locals.var_qdnm_dn0 = assign17150_e24764_d_n0;
        locals.var_qdnm_dn2 = assign17150_e24764_d_n2;
        locals.var_qdnm_dn6 = assign17150_e24764_d_n6;
        locals.var_qdnm_dn7 = assign17150_e24764_d_n7;
        locals.var_qdnm_dn10 = assign17150_e24764_d_n10;
        locals.var_qdnm_dn11 = assign17150_e24764_d_n11;
        locals.var_qdnm_dn12 = assign17150_e24764_d_n12;
        locals.var_qdnm_dn17 = assign17150_e24764_d_n17;

        let (assign17160_e24770, assign17160_e24770_d_n0, assign17160_e24770_d_n2, assign17160_e24770_d_n6, assign17160_e24770_d_n7, assign17160_e24770_d_n10, assign17160_e24770_d_n11, assign17160_e24770_d_n12, assign17160_e24770_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17160_e24768: f64 = (locals.var_qidn * locals.var_qinm);
        (assign17160_e24768, ((locals.var_qidn_dn0 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn0)), ((locals.var_qidn_dn2 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn2)), ((locals.var_qidn_dn6 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn6)), ((locals.var_qidn_dn7 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn7)), ((locals.var_qidn_dn10 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn10)), ((locals.var_qidn_dn11 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn11)), ((locals.var_qidn_dn12 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn12)), ((locals.var_qidn_dn17 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn17)),)
    } else {
        (locals.var_qddn, locals.var_qddn_dn0, locals.var_qddn_dn2, locals.var_qddn_dn6, locals.var_qddn_dn7, locals.var_qddn_dn10, locals.var_qddn_dn11, locals.var_qddn_dn12, locals.var_qddn_dn17,)
    }
};
        locals.var_qddn = assign17160_e24770;
        locals.var_qddn_dn0 = assign17160_e24770_d_n0;
        locals.var_qddn_dn2 = assign17160_e24770_d_n2;
        locals.var_qddn_dn6 = assign17160_e24770_d_n6;
        locals.var_qddn_dn7 = assign17160_e24770_d_n7;
        locals.var_qddn_dn10 = assign17160_e24770_d_n10;
        locals.var_qddn_dn11 = assign17160_e24770_d_n11;
        locals.var_qddn_dn12 = assign17160_e24770_d_n12;
        locals.var_qddn_dn17 = assign17160_e24770_d_n17;

        let (assign17170_e24778, assign17170_e24778_d_n0, assign17170_e24778_d_n2, assign17170_e24778_d_n6, assign17170_e24778_d_n7, assign17170_e24778_d_n10, assign17170_e24778_d_n11, assign17170_e24778_d_n12, assign17170_e24778_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17170_e24774: f64 = (0.4 * locals.var_qdnm);
        let assign17170_e24776: f64 = (assign17170_e24774 / locals.var_qddn);
        (assign17170_e24776, ((((0.4 * locals.var_qdnm_dn0) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn0)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn2) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn2)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn6) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn6)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn7) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn7)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn10) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn10)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn11) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn11)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn12) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn12)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn17) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn17)) / (locals.var_qddn * locals.var_qddn)),)
    } else {
        (locals.var_quot, locals.var_quot_dn0, locals.var_quot_dn2, locals.var_quot_dn6, locals.var_quot_dn7, locals.var_quot_dn10, locals.var_quot_dn11, locals.var_quot_dn12, locals.var_quot_dn17,)
    }
};
        locals.var_quot = assign17170_e24778;
        locals.var_quot_dn0 = assign17170_e24778_d_n0;
        locals.var_quot_dn2 = assign17170_e24778_d_n2;
        locals.var_quot_dn6 = assign17170_e24778_d_n6;
        locals.var_quot_dn7 = assign17170_e24778_d_n7;
        locals.var_quot_dn10 = assign17170_e24778_d_n10;
        locals.var_quot_dn11 = assign17170_e24778_d_n11;
        locals.var_quot_dn12 = assign17170_e24778_d_n12;
        locals.var_quot_dn17 = assign17170_e24778_d_n17;

        let (assign17180_e24784, assign17180_e24784_d_n0, assign17180_e24784_d_n2, assign17180_e24784_d_n6, assign17180_e24784_d_n7, assign17180_e24784_d_n10, assign17180_e24784_d_n11, assign17180_e24784_d_n12, assign17180_e24784_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17180_e24782: f64 = (0.6 - locals.var_quot);
        (assign17180_e24782, (-locals.var_quot_dn0), (-locals.var_quot_dn2), (-locals.var_quot_dn6), (-locals.var_quot_dn7), (-locals.var_quot_dn10), (-locals.var_quot_dn11), (-locals.var_quot_dn12), (-locals.var_quot_dn17),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17180_e24784;
        locals.var_qdrat_dn0 = assign17180_e24784_d_n0;
        locals.var_qdrat_dn2 = assign17180_e24784_d_n2;
        locals.var_qdrat_dn6 = assign17180_e24784_d_n6;
        locals.var_qdrat_dn7 = assign17180_e24784_d_n7;
        locals.var_qdrat_dn10 = assign17180_e24784_d_n10;
        locals.var_qdrat_dn11 = assign17180_e24784_d_n11;
        locals.var_qdrat_dn12 = assign17180_e24784_d_n12;
        locals.var_qdrat_dn17 = assign17180_e24784_d_n17;

        let assign17190_e24788: f64 = (0.5 + 1e-8);
        let assign17190_e24789: f64 = if locals.var_qdrat > assign17190_e24788 { 1.0 } else { 0.0 };
        locals.var_guard506 = assign17190_e24789;

    }

    pub(super) fn stamp_transient_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17210_e24798, assign17210_e24798_d_n0, assign17210_e24798_d_n2, assign17210_e24798_d_n6, assign17210_e24798_d_n7, assign17210_e24798_d_n10, assign17210_e24798_d_n11, assign17210_e24798_d_n12, assign17210_e24798_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard506 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17210_e24798;
        locals.var_qdrat_dn0 = assign17210_e24798_d_n0;
        locals.var_qdrat_dn2 = assign17210_e24798_d_n2;
        locals.var_qdrat_dn6 = assign17210_e24798_d_n6;
        locals.var_qdrat_dn7 = assign17210_e24798_d_n7;
        locals.var_qdrat_dn10 = assign17210_e24798_d_n10;
        locals.var_qdrat_dn11 = assign17210_e24798_d_n11;
        locals.var_qdrat_dn12 = assign17210_e24798_d_n12;
        locals.var_qdrat_dn17 = assign17210_e24798_d_n17;

        let (assign17220_e24802, assign17220_e24802_d_n0, assign17220_e24802_d_n2, assign17220_e24802_d_n6, assign17220_e24802_d_n7, assign17220_e24802_d_n10, assign17220_e24802_d_n11, assign17220_e24802_d_n12, assign17220_e24802_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    } else {
        (locals.var_qdrat_noi, locals.var_qdrat_noi_dn0, locals.var_qdrat_noi_dn2, locals.var_qdrat_noi_dn6, locals.var_qdrat_noi_dn7, locals.var_qdrat_noi_dn10, locals.var_qdrat_noi_dn11, locals.var_qdrat_noi_dn12, locals.var_qdrat_noi_dn17,)
    }
};
        locals.var_qdrat_noi = assign17220_e24802;
        locals.var_qdrat_noi_dn0 = assign17220_e24802_d_n0;
        locals.var_qdrat_noi_dn2 = assign17220_e24802_d_n2;
        locals.var_qdrat_noi_dn6 = assign17220_e24802_d_n6;
        locals.var_qdrat_noi_dn7 = assign17220_e24802_d_n7;
        locals.var_qdrat_noi_dn10 = assign17220_e24802_d_n10;
        locals.var_qdrat_noi_dn11 = assign17220_e24802_d_n11;
        locals.var_qdrat_noi_dn12 = assign17220_e24802_d_n12;
        locals.var_qdrat_noi_dn17 = assign17220_e24802_d_n17;

        let (assign17230_e24806, assign17230_e24806_d_n0, assign17230_e24806_d_n2, assign17230_e24806_d_n6, assign17230_e24806_d_n7, assign17230_e24806_d_n10, assign17230_e24806_d_n11, assign17230_e24806_d_n12, assign17230_e24806_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17230_e24806;
        locals.var_qdrat_dn0 = assign17230_e24806_d_n0;
        locals.var_qdrat_dn2 = assign17230_e24806_d_n2;
        locals.var_qdrat_dn6 = assign17230_e24806_d_n6;
        locals.var_qdrat_dn7 = assign17230_e24806_d_n7;
        locals.var_qdrat_dn10 = assign17230_e24806_d_n10;
        locals.var_qdrat_dn11 = assign17230_e24806_d_n11;
        locals.var_qdrat_dn12 = assign17230_e24806_d_n12;
        locals.var_qdrat_dn17 = assign17230_e24806_d_n17;

        let assign17240_e24809: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard508 = assign17240_e24809;

        let assign17250_e24813: f64 = (10.0 * 2.220446049250313e-16);
        let assign17250_e24818: f64 = (10.0 * 2.220446049250313e-16);
        let assign17250_e24820: f64 = if ((p.p190 < assign17250_e24813) && (p.p191 < assign17250_e24818)) { 1.0 } else { 0.0 };
        locals.var_guard524 = assign17250_e24820;

        let (assign17260_e24828, assign17260_e24828_d_n0, assign17260_e24828_d_n2, assign17260_e24828_d_n6, assign17260_e24828_d_n7, assign17260_e24828_d_n10, assign17260_e24828_d_n11, assign17260_e24828_d_n12, assign17260_e24828_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17260_e24828;
        locals.var_lred_dn0 = assign17260_e24828_d_n0;
        locals.var_lred_dn2 = assign17260_e24828_d_n2;
        locals.var_lred_dn6 = assign17260_e24828_d_n6;
        locals.var_lred_dn7 = assign17260_e24828_d_n7;
        locals.var_lred_dn10 = assign17260_e24828_d_n10;
        locals.var_lred_dn11 = assign17260_e24828_d_n11;
        locals.var_lred_dn12 = assign17260_e24828_d_n12;
        locals.var_lred_dn17 = assign17260_e24828_d_n17;

        let (assign17270_e24836, assign17270_e24836_d_n0, assign17270_e24836_d_n2, assign17270_e24836_d_n6, assign17270_e24836_d_n7, assign17270_e24836_d_n10, assign17270_e24836_d_n11, assign17270_e24836_d_n12, assign17270_e24836_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17270_e24836;
        locals.var_psdl_dn0 = assign17270_e24836_d_n0;
        locals.var_psdl_dn2 = assign17270_e24836_d_n2;
        locals.var_psdl_dn6 = assign17270_e24836_d_n6;
        locals.var_psdl_dn7 = assign17270_e24836_d_n7;
        locals.var_psdl_dn10 = assign17270_e24836_d_n10;
        locals.var_psdl_dn11 = assign17270_e24836_d_n11;
        locals.var_psdl_dn12 = assign17270_e24836_d_n12;
        locals.var_psdl_dn17 = assign17270_e24836_d_n17;

        let assign17280_e24840: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17280_e24843: f64 = (10.0 * 2.220446049250313e-16);
        let assign17280_e24844: f64 = (assign17280_e24840 - assign17280_e24843);
        let assign17280_e24845: f64 = if locals.var_psdl > assign17280_e24844 { 1.0 } else { 0.0 };
        locals.var_guard525 = assign17280_e24845;

        let (assign17290_e24861, assign17290_e24861_d_n0, assign17290_e24861_d_n2, assign17290_e24861_d_n6, assign17290_e24861_d_n7, assign17290_e24861_d_n10, assign17290_e24861_d_n11, assign17290_e24861_d_n12, assign17290_e24861_d_n17,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 != 0.0)) && (locals.var_guard525 != 0.0)) {
        let assign17290_e24855: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17290_e24858: f64 = (10.0 * 2.220446049250313e-16);
        let assign17290_e24859: f64 = (assign17290_e24855 - assign17290_e24858);
        (assign17290_e24859, (locals.var_ps0_dn0 + locals.var_vdsz_dn0), (locals.var_ps0_dn2 + locals.var_vdsz_dn2), (locals.var_ps0_dn6 + locals.var_vdsz_dn6), (locals.var_ps0_dn7 + locals.var_vdsz_dn7), (locals.var_ps0_dn10 + locals.var_vdsz_dn10), (locals.var_ps0_dn11 + locals.var_vdsz_dn11), (locals.var_ps0_dn12 + locals.var_vdsz_dn12), (locals.var_ps0_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17290_e24861;
        locals.var_psdl_dn0 = assign17290_e24861_d_n0;
        locals.var_psdl_dn2 = assign17290_e24861_d_n2;
        locals.var_psdl_dn6 = assign17290_e24861_d_n6;
        locals.var_psdl_dn7 = assign17290_e24861_d_n7;
        locals.var_psdl_dn10 = assign17290_e24861_d_n10;
        locals.var_psdl_dn11 = assign17290_e24861_d_n11;
        locals.var_psdl_dn12 = assign17290_e24861_d_n12;
        locals.var_psdl_dn17 = assign17290_e24861_d_n17;

        let (assign17300_e24875,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let (assign17300_e24873,) = {
            if (p.p43 == 1.0) {
                (p.p237,)
            } else {
                (locals.var_wdsoi_0,)
            }
        };
        (assign17300_e24873,)
    } else {
        (locals.var_wd,)
    }
};
        locals.var_wd = assign17300_e24875;

        let (assign17310_e24886,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17310_e24884: f64 = (1.0 / locals.var_wd);
        (assign17310_e24884,)
    } else {
        (locals.var_t0__blk509,)
    }
};
        locals.var_t0__blk509 = assign17310_e24886;

        let (assign17320_e24897, assign17320_e24897_d_n0, assign17320_e24897_d_n2, assign17320_e24897_d_n6, assign17320_e24897_d_n7, assign17320_e24897_d_n10, assign17320_e24897_d_n11, assign17320_e24897_d_n12, assign17320_e24897_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17320_e24895: f64 = (locals.var_qn0 * locals.var_t0__blk509);
        (assign17320_e24895, (locals.var_qn0_dn0 * locals.var_t0__blk509), (locals.var_qn0_dn2 * locals.var_t0__blk509), (locals.var_qn0_dn6 * locals.var_t0__blk509), (locals.var_qn0_dn7 * locals.var_t0__blk509), (locals.var_qn0_dn10 * locals.var_t0__blk509), (locals.var_qn0_dn11 * locals.var_t0__blk509), (locals.var_qn0_dn12 * locals.var_t0__blk509), (locals.var_qn0_dn17 * locals.var_t0__blk509),)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17320_e24897;
        locals.var_t1__blk510_dn0 = assign17320_e24897_d_n0;
        locals.var_t1__blk510_dn2 = assign17320_e24897_d_n2;
        locals.var_t1__blk510_dn6 = assign17320_e24897_d_n6;
        locals.var_t1__blk510_dn7 = assign17320_e24897_d_n7;
        locals.var_t1__blk510_dn10 = assign17320_e24897_d_n10;
        locals.var_t1__blk510_dn11 = assign17320_e24897_d_n11;
        locals.var_t1__blk510_dn12 = assign17320_e24897_d_n12;
        locals.var_t1__blk510_dn17 = assign17320_e24897_d_n17;

        let (assign17330_e24908, assign17330_e24908_d_n0, assign17330_e24908_d_n2, assign17330_e24908_d_n6, assign17330_e24908_d_n7, assign17330_e24908_d_n10, assign17330_e24908_d_n11, assign17330_e24908_d_n12, assign17330_e24908_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17330_e24906: f64 = (p.p191 * locals.var_t1__blk510);
        (assign17330_e24906, (p.p191 * locals.var_t1__blk510_dn0), (p.p191 * locals.var_t1__blk510_dn2), (p.p191 * locals.var_t1__blk510_dn6), (p.p191 * locals.var_t1__blk510_dn7), (p.p191 * locals.var_t1__blk510_dn10), (p.p191 * locals.var_t1__blk510_dn11), (p.p191 * locals.var_t1__blk510_dn12), (p.p191 * locals.var_t1__blk510_dn17),)
    } else {
        (locals.var_t2__blk511, locals.var_t2__blk511_dn0, locals.var_t2__blk511_dn2, locals.var_t2__blk511_dn6, locals.var_t2__blk511_dn7, locals.var_t2__blk511_dn10, locals.var_t2__blk511_dn11, locals.var_t2__blk511_dn12, locals.var_t2__blk511_dn17,)
    }
};
        locals.var_t2__blk511 = assign17330_e24908;
        locals.var_t2__blk511_dn0 = assign17330_e24908_d_n0;
        locals.var_t2__blk511_dn2 = assign17330_e24908_d_n2;
        locals.var_t2__blk511_dn6 = assign17330_e24908_d_n6;
        locals.var_t2__blk511_dn7 = assign17330_e24908_d_n7;
        locals.var_t2__blk511_dn10 = assign17330_e24908_d_n10;
        locals.var_t2__blk511_dn11 = assign17330_e24908_d_n11;
        locals.var_t2__blk511_dn12 = assign17330_e24908_d_n12;
        locals.var_t2__blk511_dn17 = assign17330_e24908_d_n17;

        let (assign17340_e24921, assign17340_e24921_d_n0, assign17340_e24921_d_n2, assign17340_e24921_d_n6, assign17340_e24921_d_n7, assign17340_e24921_d_n10, assign17340_e24921_d_n11, assign17340_e24921_d_n12, assign17340_e24921_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17340_e24917: f64 = (locals.var_uc_clm2 * locals.var_q_nsub);
        let assign17340_e24919: f64 = (assign17340_e24917 + locals.var_t2__blk511);
        (assign17340_e24919, (((locals.var_uc_clm2_dn0 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn0)) + locals.var_t2__blk511_dn0), (((locals.var_uc_clm2_dn2 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn2)) + locals.var_t2__blk511_dn2), (((locals.var_uc_clm2_dn6 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn6)) + locals.var_t2__blk511_dn6), (((locals.var_uc_clm2_dn7 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn7)) + locals.var_t2__blk511_dn7), (((locals.var_uc_clm2_dn10 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn10)) + locals.var_t2__blk511_dn10), (((locals.var_uc_clm2_dn11 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn11)) + locals.var_t2__blk511_dn11), (((locals.var_uc_clm2_dn12 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn12)) + locals.var_t2__blk511_dn12), (((locals.var_uc_clm2_dn17 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn17)) + locals.var_t2__blk511_dn17),)
    } else {
        (locals.var_t5__blk514, locals.var_t5__blk514_dn0, locals.var_t5__blk514_dn2, locals.var_t5__blk514_dn6, locals.var_t5__blk514_dn7, locals.var_t5__blk514_dn10, locals.var_t5__blk514_dn11, locals.var_t5__blk514_dn12, locals.var_t5__blk514_dn17,)
    }
};
        locals.var_t5__blk514 = assign17340_e24921;
        locals.var_t5__blk514_dn0 = assign17340_e24921_d_n0;
        locals.var_t5__blk514_dn2 = assign17340_e24921_d_n2;
        locals.var_t5__blk514_dn6 = assign17340_e24921_d_n6;
        locals.var_t5__blk514_dn7 = assign17340_e24921_d_n7;
        locals.var_t5__blk514_dn10 = assign17340_e24921_d_n10;
        locals.var_t5__blk514_dn11 = assign17340_e24921_d_n11;
        locals.var_t5__blk514_dn12 = assign17340_e24921_d_n12;
        locals.var_t5__blk514_dn17 = assign17340_e24921_d_n17;

        let (assign17350_e24932, assign17350_e24932_d_n0, assign17350_e24932_d_n2, assign17350_e24932_d_n6, assign17350_e24932_d_n7, assign17350_e24932_d_n10, assign17350_e24932_d_n11, assign17350_e24932_d_n12, assign17350_e24932_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17350_e24930: f64 = (1.0 / locals.var_t5__blk514);
        (assign17350_e24930, (-(locals.var_t5__blk514_dn0 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn2 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn6 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn7 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn10 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn11 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn12 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn17 / (locals.var_t5__blk514 * locals.var_t5__blk514))),)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17350_e24932;
        locals.var_t1__blk510_dn0 = assign17350_e24932_d_n0;
        locals.var_t1__blk510_dn2 = assign17350_e24932_d_n2;
        locals.var_t1__blk510_dn6 = assign17350_e24932_d_n6;
        locals.var_t1__blk510_dn7 = assign17350_e24932_d_n7;
        locals.var_t1__blk510_dn10 = assign17350_e24932_d_n10;
        locals.var_t1__blk510_dn11 = assign17350_e24932_d_n11;
        locals.var_t1__blk510_dn12 = assign17350_e24932_d_n12;
        locals.var_t1__blk510_dn17 = assign17350_e24932_d_n17;

        let (assign17360_e24943, assign17360_e24943_d_n0, assign17360_e24943_d_n2, assign17360_e24943_d_n6, assign17360_e24943_d_n7, assign17360_e24943_d_n10, assign17360_e24943_d_n11, assign17360_e24943_d_n12, assign17360_e24943_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17360_e24941: f64 = (1.034943e-10 * locals.var_t1__blk510);
        (assign17360_e24941, (1.034943e-10 * locals.var_t1__blk510_dn0), (1.034943e-10 * locals.var_t1__blk510_dn2), (1.034943e-10 * locals.var_t1__blk510_dn6), (1.034943e-10 * locals.var_t1__blk510_dn7), (1.034943e-10 * locals.var_t1__blk510_dn10), (1.034943e-10 * locals.var_t1__blk510_dn11), (1.034943e-10 * locals.var_t1__blk510_dn12), (1.034943e-10 * locals.var_t1__blk510_dn17),)
    } else {
        (locals.var_t4__blk513, locals.var_t4__blk513_dn0, locals.var_t4__blk513_dn2, locals.var_t4__blk513_dn6, locals.var_t4__blk513_dn7, locals.var_t4__blk513_dn10, locals.var_t4__blk513_dn11, locals.var_t4__blk513_dn12, locals.var_t4__blk513_dn17,)
    }
};
        locals.var_t4__blk513 = assign17360_e24943;
        locals.var_t4__blk513_dn0 = assign17360_e24943_d_n0;
        locals.var_t4__blk513_dn2 = assign17360_e24943_d_n2;
        locals.var_t4__blk513_dn6 = assign17360_e24943_d_n6;
        locals.var_t4__blk513_dn7 = assign17360_e24943_d_n7;
        locals.var_t4__blk513_dn10 = assign17360_e24943_d_n10;
        locals.var_t4__blk513_dn11 = assign17360_e24943_d_n11;
        locals.var_t4__blk513_dn12 = assign17360_e24943_d_n12;
        locals.var_t4__blk513_dn17 = assign17360_e24943_d_n17;

        let (assign17370_e24954, assign17370_e24954_d_n0, assign17370_e24954_d_n2, assign17370_e24954_d_n6, assign17370_e24954_d_n7, assign17370_e24954_d_n10, assign17370_e24954_d_n11, assign17370_e24954_d_n12, assign17370_e24954_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17370_e24952: f64 = (1.0 - p.p189);
        (assign17370_e24952, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17370_e24954;
        locals.var_t1__blk510_dn0 = assign17370_e24954_d_n0;
        locals.var_t1__blk510_dn2 = assign17370_e24954_d_n2;
        locals.var_t1__blk510_dn6 = assign17370_e24954_d_n6;
        locals.var_t1__blk510_dn7 = assign17370_e24954_d_n7;
        locals.var_t1__blk510_dn10 = assign17370_e24954_d_n10;
        locals.var_t1__blk510_dn11 = assign17370_e24954_d_n11;
        locals.var_t1__blk510_dn12 = assign17370_e24954_d_n12;
        locals.var_t1__blk510_dn17 = assign17370_e24954_d_n17;

        let (assign17380_e24971, assign17380_e24971_d_n0, assign17380_e24971_d_n2, assign17380_e24971_d_n6, assign17380_e24971_d_n7, assign17380_e24971_d_n10, assign17380_e24971_d_n11, assign17380_e24971_d_n12, assign17380_e24971_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17380_e24964: f64 = (locals.var_vds + locals.var_ps0);
        let assign17380_e24965: f64 = (p.p189 * assign17380_e24964);
        let assign17380_e24968: f64 = (locals.var_t1__blk510 * locals.var_psl);
        let assign17380_e24969: f64 = (assign17380_e24965 + assign17380_e24968);
        (assign17380_e24969, ((p.p189 * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + ((locals.var_t1__blk510_dn0 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn0))), ((p.p189 * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + ((locals.var_t1__blk510_dn2 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn2))), ((p.p189 * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + ((locals.var_t1__blk510_dn6 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn6))), ((p.p189 * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + ((locals.var_t1__blk510_dn7 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn7))), ((p.p189 * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + ((locals.var_t1__blk510_dn10 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn10))), ((p.p189 * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + ((locals.var_t1__blk510_dn11 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn11))), ((p.p189 * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + ((locals.var_t1__blk510_dn12 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn12))), ((p.p189 * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + ((locals.var_t1__blk510_dn17 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn17))),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17380_e24971;
        locals.var_psdl_dn0 = assign17380_e24971_d_n0;
        locals.var_psdl_dn2 = assign17380_e24971_d_n2;
        locals.var_psdl_dn6 = assign17380_e24971_d_n6;
        locals.var_psdl_dn7 = assign17380_e24971_d_n7;
        locals.var_psdl_dn10 = assign17380_e24971_d_n10;
        locals.var_psdl_dn11 = assign17380_e24971_d_n11;
        locals.var_psdl_dn12 = assign17380_e24971_d_n12;
        locals.var_psdl_dn17 = assign17380_e24971_d_n17;

        let assign17390_e24975: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17390_e24978: f64 = (10.0 * 2.220446049250313e-16);
        let assign17390_e24979: f64 = (assign17390_e24975 - assign17390_e24978);
        let assign17390_e24980: f64 = if locals.var_psdl > assign17390_e24979 { 1.0 } else { 0.0 };
        locals.var_guard526 = assign17390_e24980;

        let (assign17400_e24997, assign17400_e24997_d_n0, assign17400_e24997_d_n2, assign17400_e24997_d_n6, assign17400_e24997_d_n7, assign17400_e24997_d_n10, assign17400_e24997_d_n11, assign17400_e24997_d_n12, assign17400_e24997_d_n17,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign17400_e24991: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17400_e24994: f64 = (10.0 * 2.220446049250313e-16);
        let assign17400_e24995: f64 = (assign17400_e24991 - assign17400_e24994);
        (assign17400_e24995, (locals.var_ps0_dn0 + locals.var_vdsz_dn0), (locals.var_ps0_dn2 + locals.var_vdsz_dn2), (locals.var_ps0_dn6 + locals.var_vdsz_dn6), (locals.var_ps0_dn7 + locals.var_vdsz_dn7), (locals.var_ps0_dn10 + locals.var_vdsz_dn10), (locals.var_ps0_dn11 + locals.var_vdsz_dn11), (locals.var_ps0_dn12 + locals.var_vdsz_dn12), (locals.var_ps0_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17400_e24997;
        locals.var_psdl_dn0 = assign17400_e24997_d_n0;
        locals.var_psdl_dn2 = assign17400_e24997_d_n2;
        locals.var_psdl_dn6 = assign17400_e24997_d_n6;
        locals.var_psdl_dn7 = assign17400_e24997_d_n7;
        locals.var_psdl_dn10 = assign17400_e24997_d_n10;
        locals.var_psdl_dn11 = assign17400_e24997_d_n11;
        locals.var_psdl_dn12 = assign17400_e24997_d_n12;
        locals.var_psdl_dn17 = assign17400_e24997_d_n17;

        let (assign17410_e25008, assign17410_e25008_d_n0, assign17410_e25008_d_n2, assign17410_e25008_d_n6, assign17410_e25008_d_n7, assign17410_e25008_d_n10, assign17410_e25008_d_n11, assign17410_e25008_d_n12, assign17410_e25008_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17410_e25006: f64 = (locals.var_psdl - locals.var_psl);
        (assign17410_e25006, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn11 - locals.var_psl_dn11), (locals.var_psdl_dn12 - locals.var_psl_dn12), (locals.var_psdl_dn17 - locals.var_psl_dn17),)
    } else {
        (locals.var_t6w__blk516, locals.var_t6w__blk516_dn0, locals.var_t6w__blk516_dn2, locals.var_t6w__blk516_dn6, locals.var_t6w__blk516_dn7, locals.var_t6w__blk516_dn10, locals.var_t6w__blk516_dn11, locals.var_t6w__blk516_dn12, locals.var_t6w__blk516_dn17,)
    }
};
        locals.var_t6w__blk516 = assign17410_e25008;
        locals.var_t6w__blk516_dn0 = assign17410_e25008_d_n0;
        locals.var_t6w__blk516_dn2 = assign17410_e25008_d_n2;
        locals.var_t6w__blk516_dn6 = assign17410_e25008_d_n6;
        locals.var_t6w__blk516_dn7 = assign17410_e25008_d_n7;
        locals.var_t6w__blk516_dn10 = assign17410_e25008_d_n10;
        locals.var_t6w__blk516_dn11 = assign17410_e25008_d_n11;
        locals.var_t6w__blk516_dn12 = assign17410_e25008_d_n12;
        locals.var_t6w__blk516_dn17 = assign17410_e25008_d_n17;

        let (assign17420_e25026, assign17420_e25026_d_n0, assign17420_e25026_d_n2, assign17420_e25026_d_n6, assign17420_e25026_d_n7, assign17420_e25026_d_n10, assign17420_e25026_d_n11, assign17420_e25026_d_n12, assign17420_e25026_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17420_e25017: f64 = (locals.var_t6w__blk516 * locals.var_t6w__blk516);
        let assign17420_e25020: f64 = (4.0 * 0.001);
        let assign17420_e25022: f64 = (assign17420_e25020 * 0.001);
        let assign17420_e25023: f64 = (assign17420_e25017 + assign17420_e25022);
        let assign17420_e25024: f64 = (assign17420_e25023).sqrt();
        (assign17420_e25024, (((locals.var_t6w__blk516_dn0 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn0)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn2 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn2)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn6 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn6)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn7 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn7)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn10 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn10)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn11 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn11)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn12 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn12)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn17 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn17)) / (2.0 * assign17420_e25024)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17420_e25026;
        locals.var_tmf1_dn0 = assign17420_e25026_d_n0;
        locals.var_tmf1_dn2 = assign17420_e25026_d_n2;
        locals.var_tmf1_dn6 = assign17420_e25026_d_n6;
        locals.var_tmf1_dn7 = assign17420_e25026_d_n7;
        locals.var_tmf1_dn10 = assign17420_e25026_d_n10;
        locals.var_tmf1_dn11 = assign17420_e25026_d_n11;
        locals.var_tmf1_dn12 = assign17420_e25026_d_n12;
        locals.var_tmf1_dn17 = assign17420_e25026_d_n17;

        let (assign17430_e25043, assign17430_e25043_d_n0, assign17430_e25043_d_n2, assign17430_e25043_d_n6, assign17430_e25043_d_n7, assign17430_e25043_d_n10, assign17430_e25043_d_n11, assign17430_e25043_d_n12, assign17430_e25043_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17430_e25036: f64 = (locals.var_t6w__blk516 + locals.var_tmf1);
        let assign17430_e25037: f64 = (0.5 * assign17430_e25036);
        let assign17430_e25040: f64 = (1e-10 * 0.001);
        let assign17430_e25041: f64 = (assign17430_e25037 + assign17430_e25040);
        (assign17430_e25041, (0.5 * (locals.var_t6w__blk516_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w__blk516_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w__blk516_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w__blk516_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w__blk516_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w__blk516_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w__blk516_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w__blk516_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk515, locals.var_t6__blk515_dn0, locals.var_t6__blk515_dn2, locals.var_t6__blk515_dn6, locals.var_t6__blk515_dn7, locals.var_t6__blk515_dn10, locals.var_t6__blk515_dn11, locals.var_t6__blk515_dn12, locals.var_t6__blk515_dn17,)
    }
};
        locals.var_t6__blk515 = assign17430_e25043;
        locals.var_t6__blk515_dn0 = assign17430_e25043_d_n0;
        locals.var_t6__blk515_dn2 = assign17430_e25043_d_n2;
        locals.var_t6__blk515_dn6 = assign17430_e25043_d_n6;
        locals.var_t6__blk515_dn7 = assign17430_e25043_d_n7;
        locals.var_t6__blk515_dn10 = assign17430_e25043_d_n10;
        locals.var_t6__blk515_dn11 = assign17430_e25043_d_n11;
        locals.var_t6__blk515_dn12 = assign17430_e25043_d_n12;
        locals.var_t6__blk515_dn17 = assign17430_e25043_d_n17;

        let assign17440_e25046: f64 = if locals.var_t6__blk515 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign17440_e25046;

        let (assign17450_e25057, assign17450_e25057_d_n0, assign17450_e25057_d_n2, assign17450_e25057_d_n6, assign17450_e25057_d_n7, assign17450_e25057_d_n10, assign17450_e25057_d_n11, assign17450_e25057_d_n12, assign17450_e25057_d_n17,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) && (locals.var_guard527 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk515, locals.var_t6__blk515_dn0, locals.var_t6__blk515_dn2, locals.var_t6__blk515_dn6, locals.var_t6__blk515_dn7, locals.var_t6__blk515_dn10, locals.var_t6__blk515_dn11, locals.var_t6__blk515_dn12, locals.var_t6__blk515_dn17,)
    }
};
        locals.var_t6__blk515 = assign17450_e25057;
        locals.var_t6__blk515_dn0 = assign17450_e25057_d_n0;
        locals.var_t6__blk515_dn2 = assign17450_e25057_d_n2;
        locals.var_t6__blk515_dn6 = assign17450_e25057_d_n6;
        locals.var_t6__blk515_dn7 = assign17450_e25057_d_n7;
        locals.var_t6__blk515_dn10 = assign17450_e25057_d_n10;
        locals.var_t6__blk515_dn11 = assign17450_e25057_d_n11;
        locals.var_t6__blk515_dn12 = assign17450_e25057_d_n12;
        locals.var_t6__blk515_dn17 = assign17450_e25057_d_n17;

        let (assign17460_e25068, assign17460_e25068_d_n0, assign17460_e25068_d_n2, assign17460_e25068_d_n6, assign17460_e25068_d_n7, assign17460_e25068_d_n10, assign17460_e25068_d_n11, assign17460_e25068_d_n12, assign17460_e25068_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17460_e25066: f64 = (locals.var_beta * locals.var_qn0);
        (assign17460_e25066, (locals.var_beta * locals.var_qn0_dn0), (locals.var_beta * locals.var_qn0_dn2), (locals.var_beta * locals.var_qn0_dn6), (locals.var_beta * locals.var_qn0_dn7), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), (locals.var_beta * locals.var_qn0_dn11), (locals.var_beta * locals.var_qn0_dn12), (locals.var_beta * locals.var_qn0_dn17),)
    } else {
        (locals.var_t3__blk512, locals.var_t3__blk512_dn0, locals.var_t3__blk512_dn2, locals.var_t3__blk512_dn6, locals.var_t3__blk512_dn7, locals.var_t3__blk512_dn10, locals.var_t3__blk512_dn11, locals.var_t3__blk512_dn12, locals.var_t3__blk512_dn17,)
    }
};
        locals.var_t3__blk512 = assign17460_e25068;
        locals.var_t3__blk512_dn0 = assign17460_e25068_d_n0;
        locals.var_t3__blk512_dn2 = assign17460_e25068_d_n2;
        locals.var_t3__blk512_dn6 = assign17460_e25068_d_n6;
        locals.var_t3__blk512_dn7 = assign17460_e25068_d_n7;
        locals.var_t3__blk512_dn10 = assign17460_e25068_d_n10;
        locals.var_t3__blk512_dn11 = assign17460_e25068_d_n11;
        locals.var_t3__blk512_dn12 = assign17460_e25068_d_n12;
        locals.var_t3__blk512_dn17 = assign17460_e25068_d_n17;

        let (assign17470_e25079, assign17470_e25079_d_n0, assign17470_e25079_d_n2, assign17470_e25079_d_n6, assign17470_e25079_d_n7, assign17470_e25079_d_n10, assign17470_e25079_d_n11, assign17470_e25079_d_n12, assign17470_e25079_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17470_e25077: f64 = (1.0 / locals.var_t3__blk512);
        (assign17470_e25077, (-(locals.var_t3__blk512_dn0 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn2 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn6 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn7 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn10 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn11 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn12 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn17 / (locals.var_t3__blk512 * locals.var_t3__blk512))),)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17470_e25079;
        locals.var_t1__blk510_dn0 = assign17470_e25079_d_n0;
        locals.var_t1__blk510_dn2 = assign17470_e25079_d_n2;
        locals.var_t1__blk510_dn6 = assign17470_e25079_d_n6;
        locals.var_t1__blk510_dn7 = assign17470_e25079_d_n7;
        locals.var_t1__blk510_dn10 = assign17470_e25079_d_n10;
        locals.var_t1__blk510_dn11 = assign17470_e25079_d_n11;
        locals.var_t1__blk510_dn12 = assign17470_e25079_d_n12;
        locals.var_t1__blk510_dn17 = assign17470_e25079_d_n17;

        let (assign17480_e25090, assign17480_e25090_d_n0, assign17480_e25090_d_n2, assign17480_e25090_d_n6, assign17480_e25090_d_n7, assign17480_e25090_d_n10, assign17480_e25090_d_n11, assign17480_e25090_d_n12, assign17480_e25090_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17480_e25088: f64 = (locals.var_idd * locals.var_t1__blk510);
        (assign17480_e25088, ((locals.var_idd_dn0 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn0)), ((locals.var_idd_dn2 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn2)), ((locals.var_idd_dn6 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn6)), ((locals.var_idd_dn7 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn7)), ((locals.var_idd_dn10 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn10)), ((locals.var_idd_dn11 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn11)), ((locals.var_idd_dn12 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn12)), ((locals.var_idd_dn17 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn17)),)
    } else {
        (locals.var_t5__blk514, locals.var_t5__blk514_dn0, locals.var_t5__blk514_dn2, locals.var_t5__blk514_dn6, locals.var_t5__blk514_dn7, locals.var_t5__blk514_dn10, locals.var_t5__blk514_dn11, locals.var_t5__blk514_dn12, locals.var_t5__blk514_dn17,)
    }
};
        locals.var_t5__blk514 = assign17480_e25090;
        locals.var_t5__blk514_dn0 = assign17480_e25090_d_n0;
        locals.var_t5__blk514_dn2 = assign17480_e25090_d_n2;
        locals.var_t5__blk514_dn6 = assign17480_e25090_d_n6;
        locals.var_t5__blk514_dn7 = assign17480_e25090_d_n7;
        locals.var_t5__blk514_dn10 = assign17480_e25090_d_n10;
        locals.var_t5__blk514_dn11 = assign17480_e25090_d_n11;
        locals.var_t5__blk514_dn12 = assign17480_e25090_d_n12;
        locals.var_t5__blk514_dn17 = assign17480_e25090_d_n17;

        let assign17490_e25093: f64 = if locals.var_t5__blk514 < locals.var_beta_inv { 1.0 } else { 0.0 };
        locals.var_guard528 = assign17490_e25093;

        let (assign17500_e25104, assign17500_e25104_d_n0, assign17500_e25104_d_n2, assign17500_e25104_d_n6, assign17500_e25104_d_n7, assign17500_e25104_d_n10, assign17500_e25104_d_n11, assign17500_e25104_d_n12, assign17500_e25104_d_n17,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) && (locals.var_guard528 != 0.0)) {
        (locals.var_beta_inv, 0.0, 0.0, 0.0, 0.0, locals.var_beta_inv_dn10, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk514, locals.var_t5__blk514_dn0, locals.var_t5__blk514_dn2, locals.var_t5__blk514_dn6, locals.var_t5__blk514_dn7, locals.var_t5__blk514_dn10, locals.var_t5__blk514_dn11, locals.var_t5__blk514_dn12, locals.var_t5__blk514_dn17,)
    }
};
        locals.var_t5__blk514 = assign17500_e25104;
        locals.var_t5__blk514_dn0 = assign17500_e25104_d_n0;
        locals.var_t5__blk514_dn2 = assign17500_e25104_d_n2;
        locals.var_t5__blk514_dn6 = assign17500_e25104_d_n6;
        locals.var_t5__blk514_dn7 = assign17500_e25104_d_n7;
        locals.var_t5__blk514_dn10 = assign17500_e25104_d_n10;
        locals.var_t5__blk514_dn11 = assign17500_e25104_d_n11;
        locals.var_t5__blk514_dn12 = assign17500_e25104_d_n12;
        locals.var_t5__blk514_dn17 = assign17500_e25104_d_n17;

        let (assign17510_e25115, assign17510_e25115_d_n0, assign17510_e25115_d_n2, assign17510_e25115_d_n6, assign17510_e25115_d_n7, assign17510_e25115_d_n10, assign17510_e25115_d_n11, assign17510_e25115_d_n12, assign17510_e25115_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17510_e25113: f64 = (locals.var_q_nsub / 1.034943e-10);
        (assign17510_e25113, (locals.var_q_nsub_dn0 / 1.034943e-10), (locals.var_q_nsub_dn2 / 1.034943e-10), (locals.var_q_nsub_dn6 / 1.034943e-10), (locals.var_q_nsub_dn7 / 1.034943e-10), (locals.var_q_nsub_dn10 / 1.034943e-10), (locals.var_q_nsub_dn11 / 1.034943e-10), (locals.var_q_nsub_dn12 / 1.034943e-10), (locals.var_q_nsub_dn17 / 1.034943e-10),)
    } else {
        (locals.var_t10__blk520, locals.var_t10__blk520_dn0, locals.var_t10__blk520_dn2, locals.var_t10__blk520_dn6, locals.var_t10__blk520_dn7, locals.var_t10__blk520_dn10, locals.var_t10__blk520_dn11, locals.var_t10__blk520_dn12, locals.var_t10__blk520_dn17,)
    }
};
        locals.var_t10__blk520 = assign17510_e25115;
        locals.var_t10__blk520_dn0 = assign17510_e25115_d_n0;
        locals.var_t10__blk520_dn2 = assign17510_e25115_d_n2;
        locals.var_t10__blk520_dn6 = assign17510_e25115_d_n6;
        locals.var_t10__blk520_dn7 = assign17510_e25115_d_n7;
        locals.var_t10__blk520_dn10 = assign17510_e25115_d_n10;
        locals.var_t10__blk520_dn11 = assign17510_e25115_d_n11;
        locals.var_t10__blk520_dn12 = assign17510_e25115_d_n12;
        locals.var_t10__blk520_dn17 = assign17510_e25115_d_n17;

        let (assign17520_e25126, assign17520_e25126_d_n0, assign17520_e25126_d_n2, assign17520_e25126_d_n6, assign17520_e25126_d_n7, assign17520_e25126_d_n10, assign17520_e25126_d_n11, assign17520_e25126_d_n12, assign17520_e25126_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17520_e25124: f64 = (100000.0 * 10000.0);
        (assign17520_e25124, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17520_e25126;
        locals.var_t1__blk510_dn0 = assign17520_e25126_d_n0;
        locals.var_t1__blk510_dn2 = assign17520_e25126_d_n2;
        locals.var_t1__blk510_dn6 = assign17520_e25126_d_n6;
        locals.var_t1__blk510_dn7 = assign17520_e25126_d_n7;
        locals.var_t1__blk510_dn10 = assign17520_e25126_d_n10;
        locals.var_t1__blk510_dn11 = assign17520_e25126_d_n11;
        locals.var_t1__blk510_dn12 = assign17520_e25126_d_n12;
        locals.var_t1__blk510_dn17 = assign17520_e25126_d_n17;

        let (assign17530_e25137, assign17530_e25137_d_n0, assign17530_e25137_d_n2, assign17530_e25137_d_n6, assign17530_e25137_d_n7, assign17530_e25137_d_n10, assign17530_e25137_d_n11, assign17530_e25137_d_n12, assign17530_e25137_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17530_e25135: f64 = (1.0 / locals.var_leff);
        (assign17530_e25135, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk511, locals.var_t2__blk511_dn0, locals.var_t2__blk511_dn2, locals.var_t2__blk511_dn6, locals.var_t2__blk511_dn7, locals.var_t2__blk511_dn10, locals.var_t2__blk511_dn11, locals.var_t2__blk511_dn12, locals.var_t2__blk511_dn17,)
    }
};
        locals.var_t2__blk511 = assign17530_e25137;
        locals.var_t2__blk511_dn0 = assign17530_e25137_d_n0;
        locals.var_t2__blk511_dn2 = assign17530_e25137_d_n2;
        locals.var_t2__blk511_dn6 = assign17530_e25137_d_n6;
        locals.var_t2__blk511_dn7 = assign17530_e25137_d_n7;
        locals.var_t2__blk511_dn10 = assign17530_e25137_d_n10;
        locals.var_t2__blk511_dn11 = assign17530_e25137_d_n11;
        locals.var_t2__blk511_dn12 = assign17530_e25137_d_n12;
        locals.var_t2__blk511_dn17 = assign17530_e25137_d_n17;

    }

    pub(super) fn stamp_transient_block_59(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17540_e25162, assign17540_e25162_d_n0, assign17540_e25162_d_n2, assign17540_e25162_d_n6, assign17540_e25162_d_n7, assign17540_e25162_d_n10, assign17540_e25162_d_n11, assign17540_e25162_d_n12, assign17540_e25162_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17540_e25146: f64 = (2.0 * locals.var_t5__blk514);
        let assign17540_e25149: f64 = (2.0 * locals.var_t10__blk520);
        let assign17540_e25151: f64 = (assign17540_e25149 * locals.var_t6__blk515);
        let assign17540_e25153: f64 = (assign17540_e25151 * locals.var_t4__blk513);
        let assign17540_e25154: f64 = (assign17540_e25146 + assign17540_e25153);
        let assign17540_e25157: f64 = (locals.var_t1__blk510 * locals.var_t4__blk513);
        let assign17540_e25158: f64 = (assign17540_e25154 + assign17540_e25157);
        let assign17540_e25160: f64 = (assign17540_e25158 * locals.var_t2__blk511);
        (assign17540_e25160, (((((2.0 * locals.var_t5__blk514_dn0) + (((((2.0 * locals.var_t10__blk520_dn0) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn0)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn0))) + ((locals.var_t1__blk510_dn0 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn0))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn0)), (((((2.0 * locals.var_t5__blk514_dn2) + (((((2.0 * locals.var_t10__blk520_dn2) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn2)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn2))) + ((locals.var_t1__blk510_dn2 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn2))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn2)), (((((2.0 * locals.var_t5__blk514_dn6) + (((((2.0 * locals.var_t10__blk520_dn6) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn6)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn6))) + ((locals.var_t1__blk510_dn6 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn6))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn6)), (((((2.0 * locals.var_t5__blk514_dn7) + (((((2.0 * locals.var_t10__blk520_dn7) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn7)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn7))) + ((locals.var_t1__blk510_dn7 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn7))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn7)), (((((2.0 * locals.var_t5__blk514_dn10) + (((((2.0 * locals.var_t10__blk520_dn10) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn10)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn10))) + ((locals.var_t1__blk510_dn10 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn10))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn10)), (((((2.0 * locals.var_t5__blk514_dn11) + (((((2.0 * locals.var_t10__blk520_dn11) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn11)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn11))) + ((locals.var_t1__blk510_dn11 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn11))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn11)), (((((2.0 * locals.var_t5__blk514_dn12) + (((((2.0 * locals.var_t10__blk520_dn12) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn12)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn12))) + ((locals.var_t1__blk510_dn12 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn12))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn12)), (((((2.0 * locals.var_t5__blk514_dn17) + (((((2.0 * locals.var_t10__blk520_dn17) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn17)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn17))) + ((locals.var_t1__blk510_dn17 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn17))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn17)),)
    } else {
        (locals.var_t11w, locals.var_t11w_dn0, locals.var_t11w_dn2, locals.var_t11w_dn6, locals.var_t11w_dn7, locals.var_t11w_dn10, locals.var_t11w_dn11, locals.var_t11w_dn12, locals.var_t11w_dn17,)
    }
};
        locals.var_t11w = assign17540_e25162;
        locals.var_t11w_dn0 = assign17540_e25162_d_n0;
        locals.var_t11w_dn2 = assign17540_e25162_d_n2;
        locals.var_t11w_dn6 = assign17540_e25162_d_n6;
        locals.var_t11w_dn7 = assign17540_e25162_d_n7;
        locals.var_t11w_dn10 = assign17540_e25162_d_n10;
        locals.var_t11w_dn11 = assign17540_e25162_d_n11;
        locals.var_t11w_dn12 = assign17540_e25162_d_n12;
        locals.var_t11w_dn17 = assign17540_e25162_d_n17;

        let (assign17550_e25173, assign17550_e25173_d_n0, assign17550_e25173_d_n2, assign17550_e25173_d_n6, assign17550_e25173_d_n7, assign17550_e25173_d_n10, assign17550_e25173_d_n11, assign17550_e25173_d_n12, assign17550_e25173_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17550_e25171: f64 = (locals.var_t11w * locals.var_t4__blk513);
        (assign17550_e25171, ((locals.var_t11w_dn0 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn0)), ((locals.var_t11w_dn2 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn2)), ((locals.var_t11w_dn6 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn6)), ((locals.var_t11w_dn7 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn7)), ((locals.var_t11w_dn10 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn10)), ((locals.var_t11w_dn11 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn11)), ((locals.var_t11w_dn12 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn12)), ((locals.var_t11w_dn17 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn17)),)
    } else {
        (locals.var_t7__blk517, locals.var_t7__blk517_dn0, locals.var_t7__blk517_dn2, locals.var_t7__blk517_dn6, locals.var_t7__blk517_dn7, locals.var_t7__blk517_dn10, locals.var_t7__blk517_dn11, locals.var_t7__blk517_dn12, locals.var_t7__blk517_dn17,)
    }
};
        locals.var_t7__blk517 = assign17550_e25173;
        locals.var_t7__blk517_dn0 = assign17550_e25173_d_n0;
        locals.var_t7__blk517_dn2 = assign17550_e25173_d_n2;
        locals.var_t7__blk517_dn6 = assign17550_e25173_d_n6;
        locals.var_t7__blk517_dn7 = assign17550_e25173_d_n7;
        locals.var_t7__blk517_dn10 = assign17550_e25173_d_n10;
        locals.var_t7__blk517_dn11 = assign17550_e25173_d_n11;
        locals.var_t7__blk517_dn12 = assign17550_e25173_d_n12;
        locals.var_t7__blk517_dn17 = assign17550_e25173_d_n17;

        let (assign17560_e25190, assign17560_e25190_d_n0, assign17560_e25190_d_n2, assign17560_e25190_d_n6, assign17560_e25190_d_n7, assign17560_e25190_d_n10, assign17560_e25190_d_n11, assign17560_e25190_d_n12, assign17560_e25190_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17560_e25183: f64 = (2.0 * locals.var_t10__blk520);
        let assign17560_e25185: f64 = (assign17560_e25183 * locals.var_t6__blk515);
        let assign17560_e25187: f64 = (assign17560_e25185 + locals.var_t1__blk510);
        let assign17560_e25188: f64 = (4.0 * assign17560_e25187);
        (assign17560_e25188, (4.0 * ((((2.0 * locals.var_t10__blk520_dn0) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn0)) + locals.var_t1__blk510_dn0)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn2) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn2)) + locals.var_t1__blk510_dn2)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn6) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn6)) + locals.var_t1__blk510_dn6)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn7) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn7)) + locals.var_t1__blk510_dn7)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn10) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn10)) + locals.var_t1__blk510_dn10)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn11) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn11)) + locals.var_t1__blk510_dn11)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn12) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn12)) + locals.var_t1__blk510_dn12)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn17) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn17)) + locals.var_t1__blk510_dn17)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12, locals.var_t11_dn17,)
    }
};
        locals.var_t11 = assign17560_e25190;
        locals.var_t11_dn0 = assign17560_e25190_d_n0;
        locals.var_t11_dn2 = assign17560_e25190_d_n2;
        locals.var_t11_dn6 = assign17560_e25190_d_n6;
        locals.var_t11_dn7 = assign17560_e25190_d_n7;
        locals.var_t11_dn10 = assign17560_e25190_d_n10;
        locals.var_t11_dn11 = assign17560_e25190_d_n11;
        locals.var_t11_dn12 = assign17560_e25190_d_n12;
        locals.var_t11_dn17 = assign17560_e25190_d_n17;

        let (assign17570_e25203, assign17570_e25203_d_n0, assign17570_e25203_d_n2, assign17570_e25203_d_n6, assign17570_e25203_d_n7, assign17570_e25203_d_n10, assign17570_e25203_d_n11, assign17570_e25203_d_n12, assign17570_e25203_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17570_e25199: f64 = (locals.var_t11 * locals.var_t4__blk513);
        let assign17570_e25201: f64 = (assign17570_e25199 * locals.var_t4__blk513);
        (assign17570_e25201, ((((locals.var_t11_dn0 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn0)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn0)), ((((locals.var_t11_dn2 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn2)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn2)), ((((locals.var_t11_dn6 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn6)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn6)), ((((locals.var_t11_dn7 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn7)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn7)), ((((locals.var_t11_dn10 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn10)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn10)), ((((locals.var_t11_dn11 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn11)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn11)), ((((locals.var_t11_dn12 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn12)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn12)), ((((locals.var_t11_dn17 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn17)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn17)),)
    } else {
        (locals.var_t8__blk518, locals.var_t8__blk518_dn0, locals.var_t8__blk518_dn2, locals.var_t8__blk518_dn6, locals.var_t8__blk518_dn7, locals.var_t8__blk518_dn10, locals.var_t8__blk518_dn11, locals.var_t8__blk518_dn12, locals.var_t8__blk518_dn17,)
    }
};
        locals.var_t8__blk518 = assign17570_e25203;
        locals.var_t8__blk518_dn0 = assign17570_e25203_d_n0;
        locals.var_t8__blk518_dn2 = assign17570_e25203_d_n2;
        locals.var_t8__blk518_dn6 = assign17570_e25203_d_n6;
        locals.var_t8__blk518_dn7 = assign17570_e25203_d_n7;
        locals.var_t8__blk518_dn10 = assign17570_e25203_d_n10;
        locals.var_t8__blk518_dn11 = assign17570_e25203_d_n11;
        locals.var_t8__blk518_dn12 = assign17570_e25203_d_n12;
        locals.var_t8__blk518_dn17 = assign17570_e25203_d_n17;

        let (assign17580_e25217, assign17580_e25217_d_n0, assign17580_e25217_d_n2, assign17580_e25217_d_n6, assign17580_e25217_d_n7, assign17580_e25217_d_n10, assign17580_e25217_d_n11, assign17580_e25217_d_n12, assign17580_e25217_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17580_e25212: f64 = (locals.var_t7__blk517 * locals.var_t7__blk517);
        let assign17580_e25214: f64 = (assign17580_e25212 + locals.var_t8__blk518);
        let assign17580_e25215: f64 = (assign17580_e25214).sqrt();
        (assign17580_e25215, ((((locals.var_t7__blk517_dn0 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn0)) + locals.var_t8__blk518_dn0) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn2 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn2)) + locals.var_t8__blk518_dn2) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn6 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn6)) + locals.var_t8__blk518_dn6) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn7 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn7)) + locals.var_t8__blk518_dn7) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn10 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn10)) + locals.var_t8__blk518_dn10) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn11 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn11)) + locals.var_t8__blk518_dn11) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn12 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn12)) + locals.var_t8__blk518_dn12) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn17 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn17)) + locals.var_t8__blk518_dn17) / (2.0 * assign17580_e25215)),)
    } else {
        (locals.var_t9__blk519, locals.var_t9__blk519_dn0, locals.var_t9__blk519_dn2, locals.var_t9__blk519_dn6, locals.var_t9__blk519_dn7, locals.var_t9__blk519_dn10, locals.var_t9__blk519_dn11, locals.var_t9__blk519_dn12, locals.var_t9__blk519_dn17,)
    }
};
        locals.var_t9__blk519 = assign17580_e25217;
        locals.var_t9__blk519_dn0 = assign17580_e25217_d_n0;
        locals.var_t9__blk519_dn2 = assign17580_e25217_d_n2;
        locals.var_t9__blk519_dn6 = assign17580_e25217_d_n6;
        locals.var_t9__blk519_dn7 = assign17580_e25217_d_n7;
        locals.var_t9__blk519_dn10 = assign17580_e25217_d_n10;
        locals.var_t9__blk519_dn11 = assign17580_e25217_d_n11;
        locals.var_t9__blk519_dn12 = assign17580_e25217_d_n12;
        locals.var_t9__blk519_dn17 = assign17580_e25217_d_n17;

        let (assign17590_e25233, assign17590_e25233_d_n0, assign17590_e25233_d_n2, assign17590_e25233_d_n6, assign17590_e25233_d_n7, assign17590_e25233_d_n10, assign17590_e25233_d_n11, assign17590_e25233_d_n12, assign17590_e25233_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17590_e25227: f64 = (-locals.var_t7__blk517);
        let assign17590_e25229: f64 = (assign17590_e25227 + locals.var_t9__blk519);
        let assign17590_e25230: f64 = (0.5 * assign17590_e25229);
        let assign17590_e25231: f64 = (locals.var_fmdvds * assign17590_e25230);
        (assign17590_e25231, ((locals.var_fmdvds_dn0 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn0) + locals.var_t9__blk519_dn0)))), ((locals.var_fmdvds_dn2 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn2) + locals.var_t9__blk519_dn2)))), ((locals.var_fmdvds_dn6 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn6) + locals.var_t9__blk519_dn6)))), ((locals.var_fmdvds_dn7 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn7) + locals.var_t9__blk519_dn7)))), ((locals.var_fmdvds_dn10 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn10) + locals.var_t9__blk519_dn10)))), ((locals.var_fmdvds_dn11 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn11) + locals.var_t9__blk519_dn11)))), ((locals.var_fmdvds_dn12 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn12) + locals.var_t9__blk519_dn12)))), ((locals.var_fmdvds_dn17 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn17) + locals.var_t9__blk519_dn17)))),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17590_e25233;
        locals.var_lred_dn0 = assign17590_e25233_d_n0;
        locals.var_lred_dn2 = assign17590_e25233_d_n2;
        locals.var_lred_dn6 = assign17590_e25233_d_n6;
        locals.var_lred_dn7 = assign17590_e25233_d_n7;
        locals.var_lred_dn10 = assign17590_e25233_d_n10;
        locals.var_lred_dn11 = assign17590_e25233_d_n11;
        locals.var_lred_dn12 = assign17590_e25233_d_n12;
        locals.var_lred_dn17 = assign17590_e25233_d_n17;

        let (assign17600_e25241, assign17600_e25241_d_n0, assign17600_e25241_d_n2, assign17600_e25241_d_n6, assign17600_e25241_d_n7, assign17600_e25241_d_n10, assign17600_e25241_d_n11, assign17600_e25241_d_n12, assign17600_e25241_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) {
        let assign17600_e25239: f64 = (locals.var_lred * locals.var_clmmod);
        (assign17600_e25239, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn12 * locals.var_clmmod), (locals.var_lred_dn17 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17600_e25241;
        locals.var_lred_dn0 = assign17600_e25241_d_n0;
        locals.var_lred_dn2 = assign17600_e25241_d_n2;
        locals.var_lred_dn6 = assign17600_e25241_d_n6;
        locals.var_lred_dn7 = assign17600_e25241_d_n7;
        locals.var_lred_dn10 = assign17600_e25241_d_n10;
        locals.var_lred_dn11 = assign17600_e25241_d_n11;
        locals.var_lred_dn12 = assign17600_e25241_d_n12;
        locals.var_lred_dn17 = assign17600_e25241_d_n17;

        let (assign17610_e25247, assign17610_e25247_d_n0, assign17610_e25247_d_n2, assign17610_e25247_d_n6, assign17610_e25247_d_n7, assign17610_e25247_d_n10, assign17610_e25247_d_n11, assign17610_e25247_d_n12, assign17610_e25247_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17610_e25245: f64 = (locals.var_leff - locals.var_lred);
        (assign17610_e25245, (-locals.var_lred_dn0), (-locals.var_lred_dn2), (-locals.var_lred_dn6), (-locals.var_lred_dn7), (-locals.var_lred_dn10), (-locals.var_lred_dn11), (-locals.var_lred_dn12), (-locals.var_lred_dn17),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    }
};
        locals.var_lch = assign17610_e25247;
        locals.var_lch_dn0 = assign17610_e25247_d_n0;
        locals.var_lch_dn2 = assign17610_e25247_d_n2;
        locals.var_lch_dn6 = assign17610_e25247_d_n6;
        locals.var_lch_dn7 = assign17610_e25247_d_n7;
        locals.var_lch_dn10 = assign17610_e25247_d_n10;
        locals.var_lch_dn11 = assign17610_e25247_d_n11;
        locals.var_lch_dn12 = assign17610_e25247_d_n12;
        locals.var_lch_dn17 = assign17610_e25247_d_n17;

        let assign17630_e25256: f64 = if locals.var_lch < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard529 = assign17630_e25256;

        let (assign17640_e25262, assign17640_e25262_d_n0, assign17640_e25262_d_n2, assign17640_e25262_d_n6, assign17640_e25262_d_n7, assign17640_e25262_d_n10, assign17640_e25262_d_n11, assign17640_e25262_d_n12, assign17640_e25262_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard529 != 0.0)) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    }
};
        locals.var_lch = assign17640_e25262;
        locals.var_lch_dn0 = assign17640_e25262_d_n0;
        locals.var_lch_dn2 = assign17640_e25262_d_n2;
        locals.var_lch_dn6 = assign17640_e25262_d_n6;
        locals.var_lch_dn7 = assign17640_e25262_d_n7;
        locals.var_lch_dn10 = assign17640_e25262_d_n10;
        locals.var_lch_dn11 = assign17640_e25262_d_n11;
        locals.var_lch_dn12 = assign17640_e25262_d_n12;
        locals.var_lch_dn17 = assign17640_e25262_d_n17;

        let (assign17650_e25269, assign17650_e25269_d_n0, assign17650_e25269_d_n2, assign17650_e25269_d_n6, assign17650_e25269_d_n7, assign17650_e25269_d_n10, assign17650_e25269_d_n11, assign17650_e25269_d_n12, assign17650_e25269_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17650_e25265: f64 = (-locals.var_weffcv_nf);
        let assign17650_e25267: f64 = (assign17650_e25265 * locals.var_leff_cv);
        (assign17650_e25267, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign17650_e25269;
        locals.var_t1_dn0 = assign17650_e25269_d_n0;
        locals.var_t1_dn2 = assign17650_e25269_d_n2;
        locals.var_t1_dn6 = assign17650_e25269_d_n6;
        locals.var_t1_dn7 = assign17650_e25269_d_n7;
        locals.var_t1_dn10 = assign17650_e25269_d_n10;
        locals.var_t1_dn11 = assign17650_e25269_d_n11;
        locals.var_t1_dn12 = assign17650_e25269_d_n12;
        locals.var_t1_dn17 = assign17650_e25269_d_n17;

        let (assign17660_e25275, assign17660_e25275_d_n0, assign17660_e25275_d_n2, assign17660_e25275_d_n6, assign17660_e25275_d_n7, assign17660_e25275_d_n10, assign17660_e25275_d_n11, assign17660_e25275_d_n12, assign17660_e25275_d_n13, assign17660_e25275_d_n15, assign17660_e25275_d_n16, assign17660_e25275_d_n17, assign17660_e25275_d_n18,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17660_e25273: f64 = (locals.var_t1 * locals.var_qbu);
        (assign17660_e25273, ((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0)), ((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2)), ((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6)), ((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7)), ((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10)), ((locals.var_t1_dn11 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn11)), ((locals.var_t1_dn12 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t1_dn17 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign17660_e25275;
        locals.var_qb_dn0 = assign17660_e25275_d_n0;
        locals.var_qb_dn2 = assign17660_e25275_d_n2;
        locals.var_qb_dn6 = assign17660_e25275_d_n6;
        locals.var_qb_dn7 = assign17660_e25275_d_n7;
        locals.var_qb_dn10 = assign17660_e25275_d_n10;
        locals.var_qb_dn11 = assign17660_e25275_d_n11;
        locals.var_qb_dn12 = assign17660_e25275_d_n12;
        locals.var_qb_dn13 = assign17660_e25275_d_n13;
        locals.var_qb_dn15 = assign17660_e25275_d_n15;
        locals.var_qb_dn16 = assign17660_e25275_d_n16;
        locals.var_qb_dn17 = assign17660_e25275_d_n17;
        locals.var_qb_dn18 = assign17660_e25275_d_n18;

        let (assign17670_e25281, assign17670_e25281_d_n0, assign17670_e25281_d_n2, assign17670_e25281_d_n6, assign17670_e25281_d_n7, assign17670_e25281_d_n10, assign17670_e25281_d_n11, assign17670_e25281_d_n12, assign17670_e25281_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17670_e25279: f64 = (locals.var_t1 * locals.var_qiu);
        (assign17670_e25279, ((locals.var_t1_dn0 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn0)), ((locals.var_t1_dn2 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn2)), ((locals.var_t1_dn6 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn6)), ((locals.var_t1_dn7 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn7)), ((locals.var_t1_dn10 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn10)), ((locals.var_t1_dn11 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn11)), ((locals.var_t1_dn12 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn12)), ((locals.var_t1_dn17 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn17)),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign17670_e25281;
        locals.var_qi_dn0 = assign17670_e25281_d_n0;
        locals.var_qi_dn2 = assign17670_e25281_d_n2;
        locals.var_qi_dn6 = assign17670_e25281_d_n6;
        locals.var_qi_dn7 = assign17670_e25281_d_n7;
        locals.var_qi_dn10 = assign17670_e25281_d_n10;
        locals.var_qi_dn11 = assign17670_e25281_d_n11;
        locals.var_qi_dn12 = assign17670_e25281_d_n12;
        locals.var_qi_dn17 = assign17670_e25281_d_n17;

        let (assign17680_e25287, assign17680_e25287_d_n0, assign17680_e25287_d_n2, assign17680_e25287_d_n6, assign17680_e25287_d_n7, assign17680_e25287_d_n10, assign17680_e25287_d_n11, assign17680_e25287_d_n12, assign17680_e25287_d_n13, assign17680_e25287_d_n15, assign17680_e25287_d_n16, assign17680_e25287_d_n17, assign17680_e25287_d_n18,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17680_e25285: f64 = (locals.var_qi * locals.var_qdrat);
        (assign17680_e25285, ((locals.var_qi_dn0 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn0)), ((locals.var_qi_dn2 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn2)), ((locals.var_qi_dn6 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn6)), ((locals.var_qi_dn7 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn7)), ((locals.var_qi_dn10 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn10)), ((locals.var_qi_dn11 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn11)), ((locals.var_qi_dn12 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn12)), 0.0, 0.0, 0.0, ((locals.var_qi_dn17 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn17)), 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign17680_e25287;
        locals.var_qd_dn0 = assign17680_e25287_d_n0;
        locals.var_qd_dn2 = assign17680_e25287_d_n2;
        locals.var_qd_dn6 = assign17680_e25287_d_n6;
        locals.var_qd_dn7 = assign17680_e25287_d_n7;
        locals.var_qd_dn10 = assign17680_e25287_d_n10;
        locals.var_qd_dn11 = assign17680_e25287_d_n11;
        locals.var_qd_dn12 = assign17680_e25287_d_n12;
        locals.var_qd_dn13 = assign17680_e25287_d_n13;
        locals.var_qd_dn15 = assign17680_e25287_d_n15;
        locals.var_qd_dn16 = assign17680_e25287_d_n16;
        locals.var_qd_dn17 = assign17680_e25287_d_n17;
        locals.var_qd_dn18 = assign17680_e25287_d_n18;

        let assign17690_e25290: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard530 = assign17690_e25290;

        let (assign17700_e25298, assign17700_e25298_d_n0, assign17700_e25298_d_n2, assign17700_e25298_d_n6, assign17700_e25298_d_n7, assign17700_e25298_d_n10, assign17700_e25298_d_n11, assign17700_e25298_d_n12, assign17700_e25298_d_n13, assign17700_e25298_d_n15, assign17700_e25298_d_n16, assign17700_e25298_d_n17, assign17700_e25298_d_n18,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign17700_e25296: f64 = (locals.var_qb * 0.5);
        (assign17700_e25296, (locals.var_qb_dn0 * 0.5), (locals.var_qb_dn2 * 0.5), (locals.var_qb_dn6 * 0.5), (locals.var_qb_dn7 * 0.5), (locals.var_qb_dn10 * 0.5), (locals.var_qb_dn11 * 0.5), (locals.var_qb_dn12 * 0.5), (locals.var_qb_dn13 * 0.5), (locals.var_qb_dn15 * 0.5), (locals.var_qb_dn16 * 0.5), (locals.var_qb_dn17 * 0.5), (locals.var_qb_dn18 * 0.5),)
    } else {
        (locals.var_qd_fb, locals.var_qd_fb_dn0, locals.var_qd_fb_dn2, locals.var_qd_fb_dn6, locals.var_qd_fb_dn7, locals.var_qd_fb_dn10, locals.var_qd_fb_dn11, locals.var_qd_fb_dn12, locals.var_qd_fb_dn13, locals.var_qd_fb_dn15, locals.var_qd_fb_dn16, locals.var_qd_fb_dn17, locals.var_qd_fb_dn18,)
    }
};
        locals.var_qd_fb = assign17700_e25298;
        locals.var_qd_fb_dn0 = assign17700_e25298_d_n0;
        locals.var_qd_fb_dn2 = assign17700_e25298_d_n2;
        locals.var_qd_fb_dn6 = assign17700_e25298_d_n6;
        locals.var_qd_fb_dn7 = assign17700_e25298_d_n7;
        locals.var_qd_fb_dn10 = assign17700_e25298_d_n10;
        locals.var_qd_fb_dn11 = assign17700_e25298_d_n11;
        locals.var_qd_fb_dn12 = assign17700_e25298_d_n12;
        locals.var_qd_fb_dn13 = assign17700_e25298_d_n13;
        locals.var_qd_fb_dn15 = assign17700_e25298_d_n15;
        locals.var_qd_fb_dn16 = assign17700_e25298_d_n16;
        locals.var_qd_fb_dn17 = assign17700_e25298_d_n17;
        locals.var_qd_fb_dn18 = assign17700_e25298_d_n18;

        let (assign17710_e25308, assign17710_e25308_d_n0, assign17710_e25308_d_n2, assign17710_e25308_d_n6, assign17710_e25308_d_n7, assign17710_e25308_d_n10, assign17710_e25308_d_n11, assign17710_e25308_d_n12, assign17710_e25308_d_n13, assign17710_e25308_d_n15, assign17710_e25308_d_n16, assign17710_e25308_d_n17, assign17710_e25308_d_n18,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign17710_e25305: f64 = (1.0 - 0.5);
        let assign17710_e25306: f64 = (locals.var_qb * assign17710_e25305);
        (assign17710_e25306, (locals.var_qb_dn0 * assign17710_e25305), (locals.var_qb_dn2 * assign17710_e25305), (locals.var_qb_dn6 * assign17710_e25305), (locals.var_qb_dn7 * assign17710_e25305), (locals.var_qb_dn10 * assign17710_e25305), (locals.var_qb_dn11 * assign17710_e25305), (locals.var_qb_dn12 * assign17710_e25305), (locals.var_qb_dn13 * assign17710_e25305), (locals.var_qb_dn15 * assign17710_e25305), (locals.var_qb_dn16 * assign17710_e25305), (locals.var_qb_dn17 * assign17710_e25305), (locals.var_qb_dn18 * assign17710_e25305),)
    } else {
        (locals.var_qs_fb, locals.var_qs_fb_dn0, locals.var_qs_fb_dn2, locals.var_qs_fb_dn6, locals.var_qs_fb_dn7, locals.var_qs_fb_dn10, locals.var_qs_fb_dn11, locals.var_qs_fb_dn12, locals.var_qs_fb_dn13, locals.var_qs_fb_dn15, locals.var_qs_fb_dn16, locals.var_qs_fb_dn17, locals.var_qs_fb_dn18,)
    }
};
        locals.var_qs_fb = assign17710_e25308;
        locals.var_qs_fb_dn0 = assign17710_e25308_d_n0;
        locals.var_qs_fb_dn2 = assign17710_e25308_d_n2;
        locals.var_qs_fb_dn6 = assign17710_e25308_d_n6;
        locals.var_qs_fb_dn7 = assign17710_e25308_d_n7;
        locals.var_qs_fb_dn10 = assign17710_e25308_d_n10;
        locals.var_qs_fb_dn11 = assign17710_e25308_d_n11;
        locals.var_qs_fb_dn12 = assign17710_e25308_d_n12;
        locals.var_qs_fb_dn13 = assign17710_e25308_d_n13;
        locals.var_qs_fb_dn15 = assign17710_e25308_d_n15;
        locals.var_qs_fb_dn16 = assign17710_e25308_d_n16;
        locals.var_qs_fb_dn17 = assign17710_e25308_d_n17;
        locals.var_qs_fb_dn18 = assign17710_e25308_d_n18;

        let (assign17720_e25322, assign17720_e25322_d_n0, assign17720_e25322_d_n2, assign17720_e25322_d_n6, assign17720_e25322_d_n7, assign17720_e25322_d_n10, assign17720_e25322_d_n11, assign17720_e25322_d_n12, assign17720_e25322_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign17720_e25315: f64 = (locals.var_q_s0_bulk + locals.var_q_sl_bulk);
        let assign17720_e25316: f64 = (0.5 * assign17720_e25315);
        let assign17720_e25318: f64 = (assign17720_e25316 * locals.var_leff_cv);
        let assign17720_e25320: f64 = (assign17720_e25318 * locals.var_weffcv_nf);
        (assign17720_e25320, (((0.5 * (locals.var_q_s0_bulk_dn0 + locals.var_q_sl_bulk_dn0)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn2 + locals.var_q_sl_bulk_dn2)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn6 + locals.var_q_sl_bulk_dn6)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn7 + locals.var_q_sl_bulk_dn7)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn10 + locals.var_q_sl_bulk_dn10)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn11 + locals.var_q_sl_bulk_dn11)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn12 + locals.var_q_sl_bulk_dn12)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn17 + locals.var_q_sl_bulk_dn17)) * locals.var_leff_cv) * locals.var_weffcv_nf),)
    } else {
        (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12, locals.var_qsub_dn17,)
    }
};
        locals.var_qsub = assign17720_e25322;
        locals.var_qsub_dn0 = assign17720_e25322_d_n0;
        locals.var_qsub_dn2 = assign17720_e25322_d_n2;
        locals.var_qsub_dn6 = assign17720_e25322_d_n6;
        locals.var_qsub_dn7 = assign17720_e25322_d_n7;
        locals.var_qsub_dn10 = assign17720_e25322_d_n10;
        locals.var_qsub_dn11 = assign17720_e25322_d_n11;
        locals.var_qsub_dn12 = assign17720_e25322_d_n12;
        locals.var_qsub_dn17 = assign17720_e25322_d_n17;

        let (assign17730_e25330, assign17730_e25330_d_n0, assign17730_e25330_d_n2, assign17730_e25330_d_n6, assign17730_e25330_d_n7, assign17730_e25330_d_n10, assign17730_e25330_d_n11, assign17730_e25330_d_n12, assign17730_e25330_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17730_e25326: f64 = (locals.var_vds - locals.var_pds);
        let assign17730_e25328: f64 = (assign17730_e25326 / 2.0);
        (assign17730_e25328, ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0), ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0), ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0), ((locals.var_vds_dn7 - locals.var_pds_dn7) / 2.0), ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0), ((locals.var_vds_dn11 - locals.var_pds_dn11) / 2.0), ((locals.var_vds_dn12 - locals.var_pds_dn12) / 2.0), ((locals.var_vds_dn17 - locals.var_pds_dn17) / 2.0),)
    } else {
        (locals.var_t1__blk531, locals.var_t1__blk531_dn0, locals.var_t1__blk531_dn2, locals.var_t1__blk531_dn6, locals.var_t1__blk531_dn7, locals.var_t1__blk531_dn10, locals.var_t1__blk531_dn11, locals.var_t1__blk531_dn12, locals.var_t1__blk531_dn17,)
    }
};
        locals.var_t1__blk531 = assign17730_e25330;
        locals.var_t1__blk531_dn0 = assign17730_e25330_d_n0;
        locals.var_t1__blk531_dn2 = assign17730_e25330_d_n2;
        locals.var_t1__blk531_dn6 = assign17730_e25330_d_n6;
        locals.var_t1__blk531_dn7 = assign17730_e25330_d_n7;
        locals.var_t1__blk531_dn10 = assign17730_e25330_d_n10;
        locals.var_t1__blk531_dn11 = assign17730_e25330_d_n11;
        locals.var_t1__blk531_dn12 = assign17730_e25330_d_n12;
        locals.var_t1__blk531_dn17 = assign17730_e25330_d_n17;

        let (assign17740_e25338, assign17740_e25338_d_n0, assign17740_e25338_d_n2, assign17740_e25338_d_n6, assign17740_e25338_d_n7, assign17740_e25338_d_n10, assign17740_e25338_d_n11, assign17740_e25338_d_n12, assign17740_e25338_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17740_e25334: f64 = (2.0 * locals.var_t1__blk531);
        let assign17740_e25336: f64 = (assign17740_e25334 / p.p227);
        (assign17740_e25336, ((2.0 * locals.var_t1__blk531_dn0) / p.p227), ((2.0 * locals.var_t1__blk531_dn2) / p.p227), ((2.0 * locals.var_t1__blk531_dn6) / p.p227), ((2.0 * locals.var_t1__blk531_dn7) / p.p227), ((2.0 * locals.var_t1__blk531_dn10) / p.p227), ((2.0 * locals.var_t1__blk531_dn11) / p.p227), ((2.0 * locals.var_t1__blk531_dn12) / p.p227), ((2.0 * locals.var_t1__blk531_dn17) / p.p227),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17740_e25338;
        locals.var_tmf1_dn0 = assign17740_e25338_d_n0;
        locals.var_tmf1_dn2 = assign17740_e25338_d_n2;
        locals.var_tmf1_dn6 = assign17740_e25338_d_n6;
        locals.var_tmf1_dn7 = assign17740_e25338_d_n7;
        locals.var_tmf1_dn10 = assign17740_e25338_d_n10;
        locals.var_tmf1_dn11 = assign17740_e25338_d_n11;
        locals.var_tmf1_dn12 = assign17740_e25338_d_n12;
        locals.var_tmf1_dn17 = assign17740_e25338_d_n17;

        let (assign17750_e25378, assign17750_e25378_d_n0, assign17750_e25378_d_n2, assign17750_e25378_d_n6, assign17750_e25378_d_n7, assign17750_e25378_d_n10, assign17750_e25378_d_n11, assign17750_e25378_d_n12, assign17750_e25378_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17750_e25344: f64 = (1.0 / 2.0);
        let assign17750_e25348: f64 = (1.0 / 6.0);
        let assign17750_e25352: f64 = (1.0 / 24.0);
        let assign17750_e25356: f64 = (1.0 / 120.0);
        let assign17750_e25360: f64 = (1.0 / 720.0);
        let assign17750_e25364: f64 = (1.0 / 5040.0);
        let assign17750_e25365: f64 = (locals.var_tmf1 * assign17750_e25364);
        let assign17750_e25366: f64 = (assign17750_e25360 + assign17750_e25365);
        let assign17750_e25367: f64 = (locals.var_tmf1 * assign17750_e25366);
        let assign17750_e25368: f64 = (assign17750_e25356 + assign17750_e25367);
        let assign17750_e25369: f64 = (locals.var_tmf1 * assign17750_e25368);
        let assign17750_e25370: f64 = (assign17750_e25352 + assign17750_e25369);
        let assign17750_e25371: f64 = (locals.var_tmf1 * assign17750_e25370);
        let assign17750_e25372: f64 = (assign17750_e25348 + assign17750_e25371);
        let assign17750_e25373: f64 = (locals.var_tmf1 * assign17750_e25372);
        let assign17750_e25374: f64 = (assign17750_e25344 + assign17750_e25373);
        let assign17750_e25375: f64 = (locals.var_tmf1 * assign17750_e25374);
        let assign17750_e25376: f64 = (1.0 + assign17750_e25375);
        (assign17750_e25376, ((locals.var_tmf1_dn0 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn2 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn6 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn7 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn10 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn11 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn12 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn17 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign17750_e25364))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign17750_e25378;
        locals.var_tmf2_dn0 = assign17750_e25378_d_n0;
        locals.var_tmf2_dn2 = assign17750_e25378_d_n2;
        locals.var_tmf2_dn6 = assign17750_e25378_d_n6;
        locals.var_tmf2_dn7 = assign17750_e25378_d_n7;
        locals.var_tmf2_dn10 = assign17750_e25378_d_n10;
        locals.var_tmf2_dn11 = assign17750_e25378_d_n11;
        locals.var_tmf2_dn12 = assign17750_e25378_d_n12;
        locals.var_tmf2_dn17 = assign17750_e25378_d_n17;

        let (assign17760_e25384, assign17760_e25384_d_n0, assign17760_e25384_d_n2, assign17760_e25384_d_n6, assign17760_e25384_d_n7, assign17760_e25384_d_n10, assign17760_e25384_d_n11, assign17760_e25384_d_n12, assign17760_e25384_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17760_e25382: f64 = (p.p227 / locals.var_tmf2);
        (assign17760_e25382, (-((p.p227 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn12, locals.var_pzadd_dn17,)
    }
};
        locals.var_pzadd = assign17760_e25384;
        locals.var_pzadd_dn0 = assign17760_e25384_d_n0;
        locals.var_pzadd_dn2 = assign17760_e25384_d_n2;
        locals.var_pzadd_dn6 = assign17760_e25384_d_n6;
        locals.var_pzadd_dn7 = assign17760_e25384_d_n7;
        locals.var_pzadd_dn10 = assign17760_e25384_d_n10;
        locals.var_pzadd_dn11 = assign17760_e25384_d_n11;
        locals.var_pzadd_dn12 = assign17760_e25384_d_n12;
        locals.var_pzadd_dn17 = assign17760_e25384_d_n17;

        let assign17770_e25388: f64 = (10.0 * 2.220446049250313e-16);
        let assign17770_e25389: f64 = if locals.var_pzadd < assign17770_e25388 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign17770_e25389;

        let (assign17780_e25397, assign17780_e25397_d_n0, assign17780_e25397_d_n2, assign17780_e25397_d_n6, assign17780_e25397_d_n7, assign17780_e25397_d_n10, assign17780_e25397_d_n11, assign17780_e25397_d_n12, assign17780_e25397_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard532 != 0.0)) {
        let assign17780_e25395: f64 = (10.0 * 2.220446049250313e-16);
        (assign17780_e25395, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn12, locals.var_pzadd_dn17,)
    }
};
        locals.var_pzadd = assign17780_e25397;
        locals.var_pzadd_dn0 = assign17780_e25397_d_n0;
        locals.var_pzadd_dn2 = assign17780_e25397_d_n2;
        locals.var_pzadd_dn6 = assign17780_e25397_d_n6;
        locals.var_pzadd_dn7 = assign17780_e25397_d_n7;
        locals.var_pzadd_dn10 = assign17780_e25397_d_n10;
        locals.var_pzadd_dn11 = assign17780_e25397_d_n11;
        locals.var_pzadd_dn12 = assign17780_e25397_d_n12;
        locals.var_pzadd_dn17 = assign17780_e25397_d_n17;

        let (assign17790_e25403, assign17790_e25403_d_n0, assign17790_e25403_d_n2, assign17790_e25403_d_n6, assign17790_e25403_d_n7, assign17790_e25403_d_n10, assign17790_e25403_d_n11, assign17790_e25403_d_n12, assign17790_e25403_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17790_e25401: f64 = (locals.var_ps0 + locals.var_pzadd);
        (assign17790_e25401, (locals.var_ps0_dn0 + locals.var_pzadd_dn0), (locals.var_ps0_dn2 + locals.var_pzadd_dn2), (locals.var_ps0_dn6 + locals.var_pzadd_dn6), (locals.var_ps0_dn7 + locals.var_pzadd_dn7), (locals.var_ps0_dn10 + locals.var_pzadd_dn10), (locals.var_ps0_dn11 + locals.var_pzadd_dn11), (locals.var_ps0_dn12 + locals.var_pzadd_dn12), (locals.var_ps0_dn17 + locals.var_pzadd_dn17),)
    } else {
        (locals.var_ps0z, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn10, locals.var_ps0z_dn11, locals.var_ps0z_dn12, locals.var_ps0z_dn17,)
    }
};
        locals.var_ps0z = assign17790_e25403;
        locals.var_ps0z_dn0 = assign17790_e25403_d_n0;
        locals.var_ps0z_dn2 = assign17790_e25403_d_n2;
        locals.var_ps0z_dn6 = assign17790_e25403_d_n6;
        locals.var_ps0z_dn7 = assign17790_e25403_d_n7;
        locals.var_ps0z_dn10 = assign17790_e25403_d_n10;
        locals.var_ps0z_dn11 = assign17790_e25403_d_n11;
        locals.var_ps0z_dn12 = assign17790_e25403_d_n12;
        locals.var_ps0z_dn17 = assign17790_e25403_d_n17;

        let (assign17800_e25409,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17800_e25407: f64 = (1.034943e-10 / 100.0);
        (assign17800_e25407,)
    } else {
        (locals.var_cgs_esi,)
    }
};
        locals.var_cgs_esi = assign17800_e25409;

        let (assign17810_e25415, assign17810_e25415_d_n0, assign17810_e25415_d_n2, assign17810_e25415_d_n6, assign17810_e25415_d_n7, assign17810_e25415_d_n10, assign17810_e25415_d_n11, assign17810_e25415_d_n12, assign17810_e25415_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17810_e25413: f64 = (locals.var_qbu / 10000.0);
        (assign17810_e25413, (locals.var_qbu_dn0 / 10000.0), (locals.var_qbu_dn2 / 10000.0), (locals.var_qbu_dn6 / 10000.0), (locals.var_qbu_dn7 / 10000.0), (locals.var_qbu_dn10 / 10000.0), (locals.var_qbu_dn11 / 10000.0), (locals.var_qbu_dn12 / 10000.0), (locals.var_qbu_dn17 / 10000.0),)
    } else {
        (locals.var_cgs_qbu, locals.var_cgs_qbu_dn0, locals.var_cgs_qbu_dn2, locals.var_cgs_qbu_dn6, locals.var_cgs_qbu_dn7, locals.var_cgs_qbu_dn10, locals.var_cgs_qbu_dn11, locals.var_cgs_qbu_dn12, locals.var_cgs_qbu_dn17,)
    }
};
        locals.var_cgs_qbu = assign17810_e25415;
        locals.var_cgs_qbu_dn0 = assign17810_e25415_d_n0;
        locals.var_cgs_qbu_dn2 = assign17810_e25415_d_n2;
        locals.var_cgs_qbu_dn6 = assign17810_e25415_d_n6;
        locals.var_cgs_qbu_dn7 = assign17810_e25415_d_n7;
        locals.var_cgs_qbu_dn10 = assign17810_e25415_d_n10;
        locals.var_cgs_qbu_dn11 = assign17810_e25415_d_n11;
        locals.var_cgs_qbu_dn12 = assign17810_e25415_d_n12;
        locals.var_cgs_qbu_dn17 = assign17810_e25415_d_n17;

        let (assign17820_e25421, assign17820_e25421_d_n0, assign17820_e25421_d_n2, assign17820_e25421_d_n6, assign17820_e25421_d_n7, assign17820_e25421_d_n10, assign17820_e25421_d_n11, assign17820_e25421_d_n12, assign17820_e25421_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17820_e25419: f64 = (locals.var_qiu / 10000.0);
        (assign17820_e25419, (locals.var_qiu_dn0 / 10000.0), (locals.var_qiu_dn2 / 10000.0), (locals.var_qiu_dn6 / 10000.0), (locals.var_qiu_dn7 / 10000.0), (locals.var_qiu_dn10 / 10000.0), (locals.var_qiu_dn11 / 10000.0), (locals.var_qiu_dn12 / 10000.0), (locals.var_qiu_dn17 / 10000.0),)
    } else {
        (locals.var_cgs_qiu, locals.var_cgs_qiu_dn0, locals.var_cgs_qiu_dn2, locals.var_cgs_qiu_dn6, locals.var_cgs_qiu_dn7, locals.var_cgs_qiu_dn10, locals.var_cgs_qiu_dn11, locals.var_cgs_qiu_dn12, locals.var_cgs_qiu_dn17,)
    }
};
        locals.var_cgs_qiu = assign17820_e25421;
        locals.var_cgs_qiu_dn0 = assign17820_e25421_d_n0;
        locals.var_cgs_qiu_dn2 = assign17820_e25421_d_n2;
        locals.var_cgs_qiu_dn6 = assign17820_e25421_d_n6;
        locals.var_cgs_qiu_dn7 = assign17820_e25421_d_n7;
        locals.var_cgs_qiu_dn10 = assign17820_e25421_d_n10;
        locals.var_cgs_qiu_dn11 = assign17820_e25421_d_n11;
        locals.var_cgs_qiu_dn12 = assign17820_e25421_d_n12;
        locals.var_cgs_qiu_dn17 = assign17820_e25421_d_n17;

    }

    pub(super) fn stamp_transient_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17830_e25427, assign17830_e25427_d_n0, assign17830_e25427_d_n2, assign17830_e25427_d_n6, assign17830_e25427_d_n7, assign17830_e25427_d_n10, assign17830_e25427_d_n11, assign17830_e25427_d_n12, assign17830_e25427_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17830_e25425: f64 = (p.p92 / locals.var_cgs_esi);
        (assign17830_e25425, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk533, locals.var_t1__blk533_dn0, locals.var_t1__blk533_dn2, locals.var_t1__blk533_dn6, locals.var_t1__blk533_dn7, locals.var_t1__blk533_dn10, locals.var_t1__blk533_dn11, locals.var_t1__blk533_dn12, locals.var_t1__blk533_dn17,)
    }
};
        locals.var_t1__blk533 = assign17830_e25427;
        locals.var_t1__blk533_dn0 = assign17830_e25427_d_n0;
        locals.var_t1__blk533_dn2 = assign17830_e25427_d_n2;
        locals.var_t1__blk533_dn6 = assign17830_e25427_d_n6;
        locals.var_t1__blk533_dn7 = assign17830_e25427_d_n7;
        locals.var_t1__blk533_dn10 = assign17830_e25427_d_n10;
        locals.var_t1__blk533_dn11 = assign17830_e25427_d_n11;
        locals.var_t1__blk533_dn12 = assign17830_e25427_d_n12;
        locals.var_t1__blk533_dn17 = assign17830_e25427_d_n17;

        let (assign17840_e25433,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17840_e25431: f64 = (p.p93 / locals.var_cgs_esi);
        (assign17840_e25431,)
    } else {
        (locals.var_t2__blk534,)
    }
};
        locals.var_t2__blk534 = assign17840_e25433;

        let (assign17850_e25437, assign17850_e25437_d_n0, assign17850_e25437_d_n2, assign17850_e25437_d_n6, assign17850_e25437_d_n7, assign17850_e25437_d_n10, assign17850_e25437_d_n11, assign17850_e25437_d_n12, assign17850_e25437_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (p.p94, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk535, locals.var_t0__blk535_dn0, locals.var_t0__blk535_dn2, locals.var_t0__blk535_dn6, locals.var_t0__blk535_dn7, locals.var_t0__blk535_dn10, locals.var_t0__blk535_dn11, locals.var_t0__blk535_dn12, locals.var_t0__blk535_dn17,)
    }
};
        locals.var_t0__blk535 = assign17850_e25437;
        locals.var_t0__blk535_dn0 = assign17850_e25437_d_n0;
        locals.var_t0__blk535_dn2 = assign17850_e25437_d_n2;
        locals.var_t0__blk535_dn6 = assign17850_e25437_d_n6;
        locals.var_t0__blk535_dn7 = assign17850_e25437_d_n7;
        locals.var_t0__blk535_dn10 = assign17850_e25437_d_n10;
        locals.var_t0__blk535_dn11 = assign17850_e25437_d_n11;
        locals.var_t0__blk535_dn12 = assign17850_e25437_d_n12;
        locals.var_t0__blk535_dn17 = assign17850_e25437_d_n17;

        let (assign17860_e25447, assign17860_e25447_d_n0, assign17860_e25447_d_n2, assign17860_e25447_d_n6, assign17860_e25447_d_n7, assign17860_e25447_d_n10, assign17860_e25447_d_n11, assign17860_e25447_d_n12, assign17860_e25447_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17860_e25442: f64 = (locals.var_psl - locals.var_ps0);
        let assign17860_e25444: f64 = (assign17860_e25442 * locals.var_t0__blk535);
        let assign17860_e25445: f64 = (1.0 + assign17860_e25444);
        (assign17860_e25445, (((locals.var_psl_dn0 - locals.var_ps0_dn0) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn0)), (((locals.var_psl_dn2 - locals.var_ps0_dn2) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn2)), (((locals.var_psl_dn6 - locals.var_ps0_dn6) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn6)), (((locals.var_psl_dn7 - locals.var_ps0_dn7) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn7)), (((locals.var_psl_dn10 - locals.var_ps0_dn10) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn10)), (((locals.var_psl_dn11 - locals.var_ps0_dn11) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn11)), (((locals.var_psl_dn12 - locals.var_ps0_dn12) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn12)), (((locals.var_psl_dn17 - locals.var_ps0_dn17) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn17)),)
    } else {
        (locals.var_t4__blk536, locals.var_t4__blk536_dn0, locals.var_t4__blk536_dn2, locals.var_t4__blk536_dn6, locals.var_t4__blk536_dn7, locals.var_t4__blk536_dn10, locals.var_t4__blk536_dn11, locals.var_t4__blk536_dn12, locals.var_t4__blk536_dn17,)
    }
};
        locals.var_t4__blk536 = assign17860_e25447;
        locals.var_t4__blk536_dn0 = assign17860_e25447_d_n0;
        locals.var_t4__blk536_dn2 = assign17860_e25447_d_n2;
        locals.var_t4__blk536_dn6 = assign17860_e25447_d_n6;
        locals.var_t4__blk536_dn7 = assign17860_e25447_d_n7;
        locals.var_t4__blk536_dn10 = assign17860_e25447_d_n10;
        locals.var_t4__blk536_dn11 = assign17860_e25447_d_n11;
        locals.var_t4__blk536_dn12 = assign17860_e25447_d_n12;
        locals.var_t4__blk536_dn17 = assign17860_e25447_d_n17;

        let (assign17870_e25457, assign17870_e25457_d_n0, assign17870_e25457_d_n2, assign17870_e25457_d_n6, assign17870_e25457_d_n7, assign17870_e25457_d_n10, assign17870_e25457_d_n11, assign17870_e25457_d_n12, assign17870_e25457_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17870_e25451: f64 = (locals.var_t1__blk533 * locals.var_cgs_qbu);
        let assign17870_e25454: f64 = (locals.var_t2__blk534 * locals.var_cgs_qiu);
        let assign17870_e25455: f64 = (assign17870_e25451 + assign17870_e25454);
        (assign17870_e25455, (((locals.var_t1__blk533_dn0 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn0)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn0)), (((locals.var_t1__blk533_dn2 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn2)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn2)), (((locals.var_t1__blk533_dn6 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn6)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn6)), (((locals.var_t1__blk533_dn7 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn7)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn7)), (((locals.var_t1__blk533_dn10 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn10)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn10)), (((locals.var_t1__blk533_dn11 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn11)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn11)), (((locals.var_t1__blk533_dn12 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn12)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn12)), (((locals.var_t1__blk533_dn17 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn17)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn17)),)
    } else {
        (locals.var_t5__blk537, locals.var_t5__blk537_dn0, locals.var_t5__blk537_dn2, locals.var_t5__blk537_dn6, locals.var_t5__blk537_dn7, locals.var_t5__blk537_dn10, locals.var_t5__blk537_dn11, locals.var_t5__blk537_dn12, locals.var_t5__blk537_dn17,)
    }
};
        locals.var_t5__blk537 = assign17870_e25457;
        locals.var_t5__blk537_dn0 = assign17870_e25457_d_n0;
        locals.var_t5__blk537_dn2 = assign17870_e25457_d_n2;
        locals.var_t5__blk537_dn6 = assign17870_e25457_d_n6;
        locals.var_t5__blk537_dn7 = assign17870_e25457_d_n7;
        locals.var_t5__blk537_dn10 = assign17870_e25457_d_n10;
        locals.var_t5__blk537_dn11 = assign17870_e25457_d_n11;
        locals.var_t5__blk537_dn12 = assign17870_e25457_d_n12;
        locals.var_t5__blk537_dn17 = assign17870_e25457_d_n17;

        let (assign17880_e25463, assign17880_e25463_d_n0, assign17880_e25463_d_n2, assign17880_e25463_d_n6, assign17880_e25463_d_n7, assign17880_e25463_d_n10, assign17880_e25463_d_n11, assign17880_e25463_d_n12, assign17880_e25463_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17880_e25461: f64 = (locals.var_t5__blk537 / locals.var_t4__blk536);
        (assign17880_e25461, (((locals.var_t5__blk537_dn0 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn0)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn2 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn2)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn6 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn6)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn7 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn7)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn10 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn10)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn11 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn11)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn12 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn12)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn17 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn17)) / (locals.var_t4__blk536 * locals.var_t4__blk536)),)
    } else {
        (locals.var_t3__blk538, locals.var_t3__blk538_dn0, locals.var_t3__blk538_dn2, locals.var_t3__blk538_dn6, locals.var_t3__blk538_dn7, locals.var_t3__blk538_dn10, locals.var_t3__blk538_dn11, locals.var_t3__blk538_dn12, locals.var_t3__blk538_dn17,)
    }
};
        locals.var_t3__blk538 = assign17880_e25463;
        locals.var_t3__blk538_dn0 = assign17880_e25463_d_n0;
        locals.var_t3__blk538_dn2 = assign17880_e25463_d_n2;
        locals.var_t3__blk538_dn6 = assign17880_e25463_d_n6;
        locals.var_t3__blk538_dn7 = assign17880_e25463_d_n7;
        locals.var_t3__blk538_dn10 = assign17880_e25463_d_n10;
        locals.var_t3__blk538_dn11 = assign17880_e25463_d_n11;
        locals.var_t3__blk538_dn12 = assign17880_e25463_d_n12;
        locals.var_t3__blk538_dn17 = assign17880_e25463_d_n17;

        let (assign17890_e25467, assign17890_e25467_d_n0, assign17890_e25467_d_n2, assign17890_e25467_d_n6, assign17890_e25467_d_n7, assign17890_e25467_d_n10, assign17890_e25467_d_n11, assign17890_e25467_d_n12, assign17890_e25467_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (locals.var_t3__blk538, locals.var_t3__blk538_dn0, locals.var_t3__blk538_dn2, locals.var_t3__blk538_dn6, locals.var_t3__blk538_dn7, locals.var_t3__blk538_dn10, locals.var_t3__blk538_dn11, locals.var_t3__blk538_dn12, locals.var_t3__blk538_dn17,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn12, locals.var_eeff_dn17,)
    }
};
        locals.var_eeff = assign17890_e25467;
        locals.var_eeff_dn0 = assign17890_e25467_d_n0;
        locals.var_eeff_dn2 = assign17890_e25467_d_n2;
        locals.var_eeff_dn6 = assign17890_e25467_d_n6;
        locals.var_eeff_dn7 = assign17890_e25467_d_n7;
        locals.var_eeff_dn10 = assign17890_e25467_d_n10;
        locals.var_eeff_dn11 = assign17890_e25467_d_n11;
        locals.var_eeff_dn12 = assign17890_e25467_d_n12;
        locals.var_eeff_dn17 = assign17890_e25467_d_n17;

        let (assign17900_e25480, assign17900_e25480_d_n0, assign17900_e25480_d_n2, assign17900_e25480_d_n6, assign17900_e25480_d_n7, assign17900_e25480_d_n10, assign17900_e25480_d_n11, assign17900_e25480_d_n12, assign17900_e25480_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17900_e25471: f64 = (locals.var_eeff * locals.var_eeff);
        let assign17900_e25474: f64 = (4.0 * 3000.0);
        let assign17900_e25476: f64 = (assign17900_e25474 * 3000.0);
        let assign17900_e25477: f64 = (assign17900_e25471 + assign17900_e25476);
        let assign17900_e25478: f64 = (assign17900_e25477).sqrt();
        (assign17900_e25478, (((locals.var_eeff_dn0 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn0)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn2 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn2)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn6 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn6)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn7 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn7)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn10 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn10)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn11 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn11)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn12 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn12)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn17 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn17)) / (2.0 * assign17900_e25478)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17900_e25480;
        locals.var_tmf1_dn0 = assign17900_e25480_d_n0;
        locals.var_tmf1_dn2 = assign17900_e25480_d_n2;
        locals.var_tmf1_dn6 = assign17900_e25480_d_n6;
        locals.var_tmf1_dn7 = assign17900_e25480_d_n7;
        locals.var_tmf1_dn10 = assign17900_e25480_d_n10;
        locals.var_tmf1_dn11 = assign17900_e25480_d_n11;
        locals.var_tmf1_dn12 = assign17900_e25480_d_n12;
        locals.var_tmf1_dn17 = assign17900_e25480_d_n17;

        let (assign17910_e25492, assign17910_e25492_d_n0, assign17910_e25492_d_n2, assign17910_e25492_d_n6, assign17910_e25492_d_n7, assign17910_e25492_d_n10, assign17910_e25492_d_n11, assign17910_e25492_d_n12, assign17910_e25492_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17910_e25485: f64 = (locals.var_eeff + locals.var_tmf1);
        let assign17910_e25486: f64 = (0.5 * assign17910_e25485);
        let assign17910_e25489: f64 = (1e-10 * 3000.0);
        let assign17910_e25490: f64 = (assign17910_e25486 + assign17910_e25489);
        (assign17910_e25490, (0.5 * (locals.var_eeff_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_eeff_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_eeff_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_eeff_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_eeff_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_eeff_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_eeff_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_eeff_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t0__blk535, locals.var_t0__blk535_dn0, locals.var_t0__blk535_dn2, locals.var_t0__blk535_dn6, locals.var_t0__blk535_dn7, locals.var_t0__blk535_dn10, locals.var_t0__blk535_dn11, locals.var_t0__blk535_dn12, locals.var_t0__blk535_dn17,)
    }
};
        locals.var_t0__blk535 = assign17910_e25492;
        locals.var_t0__blk535_dn0 = assign17910_e25492_d_n0;
        locals.var_t0__blk535_dn2 = assign17910_e25492_d_n2;
        locals.var_t0__blk535_dn6 = assign17910_e25492_d_n6;
        locals.var_t0__blk535_dn7 = assign17910_e25492_d_n7;
        locals.var_t0__blk535_dn10 = assign17910_e25492_d_n10;
        locals.var_t0__blk535_dn11 = assign17910_e25492_d_n11;
        locals.var_t0__blk535_dn12 = assign17910_e25492_d_n12;
        locals.var_t0__blk535_dn17 = assign17910_e25492_d_n17;

        let assign17920_e25495: f64 = if locals.var_t0__blk535 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign17920_e25495;

        let (assign17930_e25501, assign17930_e25501_d_n0, assign17930_e25501_d_n2, assign17930_e25501_d_n6, assign17930_e25501_d_n7, assign17930_e25501_d_n10, assign17930_e25501_d_n11, assign17930_e25501_d_n12, assign17930_e25501_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard545 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk535, locals.var_t0__blk535_dn0, locals.var_t0__blk535_dn2, locals.var_t0__blk535_dn6, locals.var_t0__blk535_dn7, locals.var_t0__blk535_dn10, locals.var_t0__blk535_dn11, locals.var_t0__blk535_dn12, locals.var_t0__blk535_dn17,)
    }
};
        locals.var_t0__blk535 = assign17930_e25501;
        locals.var_t0__blk535_dn0 = assign17930_e25501_d_n0;
        locals.var_t0__blk535_dn2 = assign17930_e25501_d_n2;
        locals.var_t0__blk535_dn6 = assign17930_e25501_d_n6;
        locals.var_t0__blk535_dn7 = assign17930_e25501_d_n7;
        locals.var_t0__blk535_dn10 = assign17930_e25501_d_n10;
        locals.var_t0__blk535_dn11 = assign17930_e25501_d_n11;
        locals.var_t0__blk535_dn12 = assign17930_e25501_d_n12;
        locals.var_t0__blk535_dn17 = assign17930_e25501_d_n17;

        let (assign17940_e25509, assign17940_e25509_d_n0, assign17940_e25509_d_n2, assign17940_e25509_d_n6, assign17940_e25509_d_n7, assign17940_e25509_d_n10, assign17940_e25509_d_n11, assign17940_e25509_d_n12, assign17940_e25509_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17940_e25506: f64 = (p.p97 - 1.0);
        let assign17940_e25507: f64 = (locals.var_t0__blk535).powf(assign17940_e25506);
        (assign17940_e25507, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn0)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn0 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn2)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn2 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn6)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn6 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn7)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn7 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn10)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn10 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn11)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn11 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn12)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn12 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn17)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn17 / locals.var_t0__blk535))) },)
    } else {
        (locals.var_t5__blk537, locals.var_t5__blk537_dn0, locals.var_t5__blk537_dn2, locals.var_t5__blk537_dn6, locals.var_t5__blk537_dn7, locals.var_t5__blk537_dn10, locals.var_t5__blk537_dn11, locals.var_t5__blk537_dn12, locals.var_t5__blk537_dn17,)
    }
};
        locals.var_t5__blk537 = assign17940_e25509;
        locals.var_t5__blk537_dn0 = assign17940_e25509_d_n0;
        locals.var_t5__blk537_dn2 = assign17940_e25509_d_n2;
        locals.var_t5__blk537_dn6 = assign17940_e25509_d_n6;
        locals.var_t5__blk537_dn7 = assign17940_e25509_d_n7;
        locals.var_t5__blk537_dn10 = assign17940_e25509_d_n10;
        locals.var_t5__blk537_dn11 = assign17940_e25509_d_n11;
        locals.var_t5__blk537_dn12 = assign17940_e25509_d_n12;
        locals.var_t5__blk537_dn17 = assign17940_e25509_d_n17;

        let (assign17950_e25515, assign17950_e25515_d_n0, assign17950_e25515_d_n2, assign17950_e25515_d_n6, assign17950_e25515_d_n7, assign17950_e25515_d_n10, assign17950_e25515_d_n11, assign17950_e25515_d_n12, assign17950_e25515_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17950_e25513: f64 = (locals.var_t5__blk537 * locals.var_t0__blk535);
        (assign17950_e25513, ((locals.var_t5__blk537_dn0 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn0)), ((locals.var_t5__blk537_dn2 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn2)), ((locals.var_t5__blk537_dn6 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn6)), ((locals.var_t5__blk537_dn7 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn7)), ((locals.var_t5__blk537_dn10 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn10)), ((locals.var_t5__blk537_dn11 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn11)), ((locals.var_t5__blk537_dn12 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn12)), ((locals.var_t5__blk537_dn17 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn17)),)
    } else {
        (locals.var_t8__blk539, locals.var_t8__blk539_dn0, locals.var_t8__blk539_dn2, locals.var_t8__blk539_dn6, locals.var_t8__blk539_dn7, locals.var_t8__blk539_dn10, locals.var_t8__blk539_dn11, locals.var_t8__blk539_dn12, locals.var_t8__blk539_dn17,)
    }
};
        locals.var_t8__blk539 = assign17950_e25515;
        locals.var_t8__blk539_dn0 = assign17950_e25515_d_n0;
        locals.var_t8__blk539_dn2 = assign17950_e25515_d_n2;
        locals.var_t8__blk539_dn6 = assign17950_e25515_d_n6;
        locals.var_t8__blk539_dn7 = assign17950_e25515_d_n7;
        locals.var_t8__blk539_dn10 = assign17950_e25515_d_n10;
        locals.var_t8__blk539_dn11 = assign17950_e25515_d_n11;
        locals.var_t8__blk539_dn12 = assign17950_e25515_d_n12;
        locals.var_t8__blk539_dn17 = assign17950_e25515_d_n17;

        let (assign17960_e25523, assign17960_e25523_d_n0, assign17960_e25523_d_n2, assign17960_e25523_d_n6, assign17960_e25523_d_n7, assign17960_e25523_d_n10, assign17960_e25523_d_n11, assign17960_e25523_d_n12, assign17960_e25523_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17960_e25520: f64 = (locals.var_muesr - 1.0);
        let assign17960_e25521: f64 = (locals.var_t0__blk535).powf(assign17960_e25520);
        (assign17960_e25521, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn0)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn0 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn2)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn2 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn6)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn6 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn7)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn7 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn10)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn10 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn11)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn11 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn12)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn12 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn17)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn17 / locals.var_t0__blk535))) },)
    } else {
        (locals.var_t7__blk540, locals.var_t7__blk540_dn0, locals.var_t7__blk540_dn2, locals.var_t7__blk540_dn6, locals.var_t7__blk540_dn7, locals.var_t7__blk540_dn10, locals.var_t7__blk540_dn11, locals.var_t7__blk540_dn12, locals.var_t7__blk540_dn17,)
    }
};
        locals.var_t7__blk540 = assign17960_e25523;
        locals.var_t7__blk540_dn0 = assign17960_e25523_d_n0;
        locals.var_t7__blk540_dn2 = assign17960_e25523_d_n2;
        locals.var_t7__blk540_dn6 = assign17960_e25523_d_n6;
        locals.var_t7__blk540_dn7 = assign17960_e25523_d_n7;
        locals.var_t7__blk540_dn10 = assign17960_e25523_d_n10;
        locals.var_t7__blk540_dn11 = assign17960_e25523_d_n11;
        locals.var_t7__blk540_dn12 = assign17960_e25523_d_n12;
        locals.var_t7__blk540_dn17 = assign17960_e25523_d_n17;

        let (assign17970_e25529, assign17970_e25529_d_n0, assign17970_e25529_d_n2, assign17970_e25529_d_n6, assign17970_e25529_d_n7, assign17970_e25529_d_n10, assign17970_e25529_d_n11, assign17970_e25529_d_n12, assign17970_e25529_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17970_e25527: f64 = (locals.var_t7__blk540 * locals.var_t0__blk535);
        (assign17970_e25527, ((locals.var_t7__blk540_dn0 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn0)), ((locals.var_t7__blk540_dn2 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn2)), ((locals.var_t7__blk540_dn6 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn6)), ((locals.var_t7__blk540_dn7 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn7)), ((locals.var_t7__blk540_dn10 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn10)), ((locals.var_t7__blk540_dn11 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn11)), ((locals.var_t7__blk540_dn12 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn12)), ((locals.var_t7__blk540_dn17 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn17)),)
    } else {
        (locals.var_t6__blk541, locals.var_t6__blk541_dn0, locals.var_t6__blk541_dn2, locals.var_t6__blk541_dn6, locals.var_t6__blk541_dn7, locals.var_t6__blk541_dn10, locals.var_t6__blk541_dn11, locals.var_t6__blk541_dn12, locals.var_t6__blk541_dn17,)
    }
};
        locals.var_t6__blk541 = assign17970_e25529;
        locals.var_t6__blk541_dn0 = assign17970_e25529_d_n0;
        locals.var_t6__blk541_dn2 = assign17970_e25529_d_n2;
        locals.var_t6__blk541_dn6 = assign17970_e25529_d_n6;
        locals.var_t6__blk541_dn7 = assign17970_e25529_d_n7;
        locals.var_t6__blk541_dn10 = assign17970_e25529_d_n10;
        locals.var_t6__blk541_dn11 = assign17970_e25529_d_n11;
        locals.var_t6__blk541_dn12 = assign17970_e25529_d_n12;
        locals.var_t6__blk541_dn17 = assign17970_e25529_d_n17;

        let (assign17980_e25535, assign17980_e25535_d_n0, assign17980_e25535_d_n2, assign17980_e25535_d_n6, assign17980_e25535_d_n7, assign17980_e25535_d_n10, assign17980_e25535_d_n11, assign17980_e25535_d_n12, assign17980_e25535_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17980_e25533: f64 = (locals.var_cgs_qiu / 1.6021918e-19);
        (assign17980_e25533, (locals.var_cgs_qiu_dn0 / 1.6021918e-19), (locals.var_cgs_qiu_dn2 / 1.6021918e-19), (locals.var_cgs_qiu_dn6 / 1.6021918e-19), (locals.var_cgs_qiu_dn7 / 1.6021918e-19), (locals.var_cgs_qiu_dn10 / 1.6021918e-19), (locals.var_cgs_qiu_dn11 / 1.6021918e-19), (locals.var_cgs_qiu_dn12 / 1.6021918e-19), (locals.var_cgs_qiu_dn17 / 1.6021918e-19),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn12, locals.var_rns_dn17,)
    }
};
        locals.var_rns = assign17980_e25535;
        locals.var_rns_dn0 = assign17980_e25535_d_n0;
        locals.var_rns_dn2 = assign17980_e25535_d_n2;
        locals.var_rns_dn6 = assign17980_e25535_d_n6;
        locals.var_rns_dn7 = assign17980_e25535_d_n7;
        locals.var_rns_dn10 = assign17980_e25535_d_n10;
        locals.var_rns_dn11 = assign17980_e25535_d_n11;
        locals.var_rns_dn12 = assign17980_e25535_d_n12;
        locals.var_rns_dn17 = assign17980_e25535_d_n17;

        let (assign17990_e25555, assign17990_e25555_d_n0, assign17990_e25555_d_n2, assign17990_e25555_d_n6, assign17990_e25555_d_n7, assign17990_e25555_d_n10, assign17990_e25555_d_n11, assign17990_e25555_d_n12, assign17990_e25555_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17990_e25541: f64 = (p.p96 * locals.var_rns);
        let assign17990_e25543: f64 = (assign17990_e25541 / 100000000000.0);
        let assign17990_e25544: f64 = (p.p95 + assign17990_e25543);
        let assign17990_e25545: f64 = (1.0 / assign17990_e25544);
        let assign17990_e25548: f64 = (locals.var_cgs_mphn0 * locals.var_t8__blk539);
        let assign17990_e25549: f64 = (assign17990_e25545 + assign17990_e25548);
        let assign17990_e25552: f64 = (locals.var_t6__blk541 / p.p106);
        let assign17990_e25553: f64 = (assign17990_e25549 + assign17990_e25552);
        (assign17990_e25553, (((-(((p.p96 * locals.var_rns_dn0) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn0)) + (locals.var_t6__blk541_dn0 / p.p106)), (((-(((p.p96 * locals.var_rns_dn2) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn2)) + (locals.var_t6__blk541_dn2 / p.p106)), (((-(((p.p96 * locals.var_rns_dn6) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn6)) + (locals.var_t6__blk541_dn6 / p.p106)), (((-(((p.p96 * locals.var_rns_dn7) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn7)) + (locals.var_t6__blk541_dn7 / p.p106)), (((-(((p.p96 * locals.var_rns_dn10) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + ((locals.var_cgs_mphn0_dn10 * locals.var_t8__blk539) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn10))) + (locals.var_t6__blk541_dn10 / p.p106)), (((-(((p.p96 * locals.var_rns_dn11) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn11)) + (locals.var_t6__blk541_dn11 / p.p106)), (((-(((p.p96 * locals.var_rns_dn12) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn12)) + (locals.var_t6__blk541_dn12 / p.p106)), (((-(((p.p96 * locals.var_rns_dn17) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn17)) + (locals.var_t6__blk541_dn17 / p.p106)),)
    } else {
        (locals.var_t1__blk533, locals.var_t1__blk533_dn0, locals.var_t1__blk533_dn2, locals.var_t1__blk533_dn6, locals.var_t1__blk533_dn7, locals.var_t1__blk533_dn10, locals.var_t1__blk533_dn11, locals.var_t1__blk533_dn12, locals.var_t1__blk533_dn17,)
    }
};
        locals.var_t1__blk533 = assign17990_e25555;
        locals.var_t1__blk533_dn0 = assign17990_e25555_d_n0;
        locals.var_t1__blk533_dn2 = assign17990_e25555_d_n2;
        locals.var_t1__blk533_dn6 = assign17990_e25555_d_n6;
        locals.var_t1__blk533_dn7 = assign17990_e25555_d_n7;
        locals.var_t1__blk533_dn10 = assign17990_e25555_d_n10;
        locals.var_t1__blk533_dn11 = assign17990_e25555_d_n11;
        locals.var_t1__blk533_dn12 = assign17990_e25555_d_n12;
        locals.var_t1__blk533_dn17 = assign17990_e25555_d_n17;

        let (assign18000_e25561, assign18000_e25561_d_n0, assign18000_e25561_d_n2, assign18000_e25561_d_n6, assign18000_e25561_d_n7, assign18000_e25561_d_n10, assign18000_e25561_d_n11, assign18000_e25561_d_n12, assign18000_e25561_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18000_e25559: f64 = (1.0 / locals.var_t1__blk533);
        (assign18000_e25559, (-(locals.var_t1__blk533_dn0 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn2 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn6 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn7 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn10 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn11 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn12 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn17 / (locals.var_t1__blk533 * locals.var_t1__blk533))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn12, locals.var_muun_dn17,)
    }
};
        locals.var_muun = assign18000_e25561;
        locals.var_muun_dn0 = assign18000_e25561_d_n0;
        locals.var_muun_dn2 = assign18000_e25561_d_n2;
        locals.var_muun_dn6 = assign18000_e25561_d_n6;
        locals.var_muun_dn7 = assign18000_e25561_d_n7;
        locals.var_muun_dn10 = assign18000_e25561_d_n10;
        locals.var_muun_dn11 = assign18000_e25561_d_n11;
        locals.var_muun_dn12 = assign18000_e25561_d_n12;
        locals.var_muun_dn17 = assign18000_e25561_d_n17;

        let (assign18010_e25567, assign18010_e25567_d_n0, assign18010_e25567_d_n2, assign18010_e25567_d_n6, assign18010_e25567_d_n7, assign18010_e25567_d_n10, assign18010_e25567_d_n11, assign18010_e25567_d_n12, assign18010_e25567_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18010_e25565: f64 = (locals.var_muun * 0.0001);
        (assign18010_e25565, (locals.var_muun_dn0 * 0.0001), (locals.var_muun_dn2 * 0.0001), (locals.var_muun_dn6 * 0.0001), (locals.var_muun_dn7 * 0.0001), (locals.var_muun_dn10 * 0.0001), (locals.var_muun_dn11 * 0.0001), (locals.var_muun_dn12 * 0.0001), (locals.var_muun_dn17 * 0.0001),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn12, locals.var_muun_dn17,)
    }
};
        locals.var_muun = assign18010_e25567;
        locals.var_muun_dn0 = assign18010_e25567_d_n0;
        locals.var_muun_dn2 = assign18010_e25567_d_n2;
        locals.var_muun_dn6 = assign18010_e25567_d_n6;
        locals.var_muun_dn7 = assign18010_e25567_d_n7;
        locals.var_muun_dn10 = assign18010_e25567_d_n10;
        locals.var_muun_dn11 = assign18010_e25567_d_n11;
        locals.var_muun_dn12 = assign18010_e25567_d_n12;
        locals.var_muun_dn17 = assign18010_e25567_d_n17;

        let (assign18020_e25575, assign18020_e25575_d_n0, assign18020_e25575_d_n2, assign18020_e25575_d_n6, assign18020_e25575_d_n7, assign18020_e25575_d_n10, assign18020_e25575_d_n11, assign18020_e25575_d_n12, assign18020_e25575_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18020_e25571: f64 = (locals.var_beta * locals.var_qn0);
        let assign18020_e25573: f64 = (assign18020_e25571 * locals.var_lch);
        (assign18020_e25573, (((locals.var_beta * locals.var_qn0_dn0) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn0)), (((locals.var_beta * locals.var_qn0_dn2) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn2)), (((locals.var_beta * locals.var_qn0_dn6) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn6)), (((locals.var_beta * locals.var_qn0_dn7) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn7)), ((((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn10)), (((locals.var_beta * locals.var_qn0_dn11) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn11)), (((locals.var_beta * locals.var_qn0_dn12) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn12)), (((locals.var_beta * locals.var_qn0_dn17) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn17)),)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18020_e25575;
        locals.var_t2__blk546_dn0 = assign18020_e25575_d_n0;
        locals.var_t2__blk546_dn2 = assign18020_e25575_d_n2;
        locals.var_t2__blk546_dn6 = assign18020_e25575_d_n6;
        locals.var_t2__blk546_dn7 = assign18020_e25575_d_n7;
        locals.var_t2__blk546_dn10 = assign18020_e25575_d_n10;
        locals.var_t2__blk546_dn11 = assign18020_e25575_d_n11;
        locals.var_t2__blk546_dn12 = assign18020_e25575_d_n12;
        locals.var_t2__blk546_dn17 = assign18020_e25575_d_n17;

        let (assign18030_e25588, assign18030_e25588_d_n0, assign18030_e25588_d_n2, assign18030_e25588_d_n6, assign18030_e25588_d_n7, assign18030_e25588_d_n10, assign18030_e25588_d_n11, assign18030_e25588_d_n12, assign18030_e25588_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18030_e25579: f64 = (locals.var_t2__blk546 * locals.var_t2__blk546);
        let assign18030_e25582: f64 = (4.0 * 1e-50);
        let assign18030_e25584: f64 = (assign18030_e25582 * 1e-50);
        let assign18030_e25585: f64 = (assign18030_e25579 + assign18030_e25584);
        let assign18030_e25586: f64 = (assign18030_e25585).sqrt();
        (assign18030_e25586, (((locals.var_t2__blk546_dn0 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn0)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn2 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn2)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn6 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn6)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn7 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn7)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn10 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn10)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn11 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn11)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn12 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn12)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn17 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn17)) / (2.0 * assign18030_e25586)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18030_e25588;
        locals.var_tmf1_dn0 = assign18030_e25588_d_n0;
        locals.var_tmf1_dn2 = assign18030_e25588_d_n2;
        locals.var_tmf1_dn6 = assign18030_e25588_d_n6;
        locals.var_tmf1_dn7 = assign18030_e25588_d_n7;
        locals.var_tmf1_dn10 = assign18030_e25588_d_n10;
        locals.var_tmf1_dn11 = assign18030_e25588_d_n11;
        locals.var_tmf1_dn12 = assign18030_e25588_d_n12;
        locals.var_tmf1_dn17 = assign18030_e25588_d_n17;

        let (assign18040_e25600, assign18040_e25600_d_n0, assign18040_e25600_d_n2, assign18040_e25600_d_n6, assign18040_e25600_d_n7, assign18040_e25600_d_n10, assign18040_e25600_d_n11, assign18040_e25600_d_n12, assign18040_e25600_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18040_e25593: f64 = (locals.var_t2__blk546 + locals.var_tmf1);
        let assign18040_e25594: f64 = (0.5 * assign18040_e25593);
        let assign18040_e25597: f64 = (1e-10 * 1e-50);
        let assign18040_e25598: f64 = (assign18040_e25594 + assign18040_e25597);
        (assign18040_e25598, (0.5 * (locals.var_t2__blk546_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t2__blk546_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t2__blk546_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t2__blk546_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t2__blk546_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t2__blk546_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t2__blk546_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t2__blk546_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18040_e25600;
        locals.var_t2__blk546_dn0 = assign18040_e25600_d_n0;
        locals.var_t2__blk546_dn2 = assign18040_e25600_d_n2;
        locals.var_t2__blk546_dn6 = assign18040_e25600_d_n6;
        locals.var_t2__blk546_dn7 = assign18040_e25600_d_n7;
        locals.var_t2__blk546_dn10 = assign18040_e25600_d_n10;
        locals.var_t2__blk546_dn11 = assign18040_e25600_d_n11;
        locals.var_t2__blk546_dn12 = assign18040_e25600_d_n12;
        locals.var_t2__blk546_dn17 = assign18040_e25600_d_n17;

        let assign18050_e25603: f64 = if locals.var_t2__blk546 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard554 = assign18050_e25603;

        let (assign18060_e25609, assign18060_e25609_d_n0, assign18060_e25609_d_n2, assign18060_e25609_d_n6, assign18060_e25609_d_n7, assign18060_e25609_d_n10, assign18060_e25609_d_n11, assign18060_e25609_d_n12, assign18060_e25609_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard554 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18060_e25609;
        locals.var_t2__blk546_dn0 = assign18060_e25609_d_n0;
        locals.var_t2__blk546_dn2 = assign18060_e25609_d_n2;
        locals.var_t2__blk546_dn6 = assign18060_e25609_d_n6;
        locals.var_t2__blk546_dn7 = assign18060_e25609_d_n7;
        locals.var_t2__blk546_dn10 = assign18060_e25609_d_n10;
        locals.var_t2__blk546_dn11 = assign18060_e25609_d_n11;
        locals.var_t2__blk546_dn12 = assign18060_e25609_d_n12;
        locals.var_t2__blk546_dn17 = assign18060_e25609_d_n17;

        let (assign18070_e25615, assign18070_e25615_d_n0, assign18070_e25615_d_n2, assign18070_e25615_d_n6, assign18070_e25615_d_n7, assign18070_e25615_d_n10, assign18070_e25615_d_n11, assign18070_e25615_d_n12, assign18070_e25615_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18070_e25613: f64 = (1.0 / locals.var_t2__blk546);
        (assign18070_e25613, (-(locals.var_t2__blk546_dn0 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn2 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn6 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn7 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn10 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn11 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn12 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn17 / (locals.var_t2__blk546 * locals.var_t2__blk546))),)
    } else {
        (locals.var_t1__blk547, locals.var_t1__blk547_dn0, locals.var_t1__blk547_dn2, locals.var_t1__blk547_dn6, locals.var_t1__blk547_dn7, locals.var_t1__blk547_dn10, locals.var_t1__blk547_dn11, locals.var_t1__blk547_dn12, locals.var_t1__blk547_dn17,)
    }
};
        locals.var_t1__blk547 = assign18070_e25615;
        locals.var_t1__blk547_dn0 = assign18070_e25615_d_n0;
        locals.var_t1__blk547_dn2 = assign18070_e25615_d_n2;
        locals.var_t1__blk547_dn6 = assign18070_e25615_d_n6;
        locals.var_t1__blk547_dn7 = assign18070_e25615_d_n7;
        locals.var_t1__blk547_dn10 = assign18070_e25615_d_n10;
        locals.var_t1__blk547_dn11 = assign18070_e25615_d_n11;
        locals.var_t1__blk547_dn12 = assign18070_e25615_d_n12;
        locals.var_t1__blk547_dn17 = assign18070_e25615_d_n17;

        let (assign18080_e25621, assign18080_e25621_d_n0, assign18080_e25621_d_n2, assign18080_e25621_d_n6, assign18080_e25621_d_n7, assign18080_e25621_d_n10, assign18080_e25621_d_n11, assign18080_e25621_d_n12, assign18080_e25621_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18080_e25619: f64 = (locals.var_idd * locals.var_t1__blk547);
        (assign18080_e25619, ((locals.var_idd_dn0 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn0)), ((locals.var_idd_dn2 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn2)), ((locals.var_idd_dn6 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn6)), ((locals.var_idd_dn7 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn7)), ((locals.var_idd_dn10 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn10)), ((locals.var_idd_dn11 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn11)), ((locals.var_idd_dn12 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn12)), ((locals.var_idd_dn17 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn17)),)
    } else {
        (locals.var_ty__blk548, locals.var_ty__blk548_dn0, locals.var_ty__blk548_dn2, locals.var_ty__blk548_dn6, locals.var_ty__blk548_dn7, locals.var_ty__blk548_dn10, locals.var_ty__blk548_dn11, locals.var_ty__blk548_dn12, locals.var_ty__blk548_dn17,)
    }
};
        locals.var_ty__blk548 = assign18080_e25621;
        locals.var_ty__blk548_dn0 = assign18080_e25621_d_n0;
        locals.var_ty__blk548_dn2 = assign18080_e25621_d_n2;
        locals.var_ty__blk548_dn6 = assign18080_e25621_d_n6;
        locals.var_ty__blk548_dn7 = assign18080_e25621_d_n7;
        locals.var_ty__blk548_dn10 = assign18080_e25621_d_n10;
        locals.var_ty__blk548_dn11 = assign18080_e25621_d_n11;
        locals.var_ty__blk548_dn12 = assign18080_e25621_d_n12;
        locals.var_ty__blk548_dn17 = assign18080_e25621_d_n17;

        let (assign18090_e25629, assign18090_e25629_d_n0, assign18090_e25629_d_n2, assign18090_e25629_d_n6, assign18090_e25629_d_n7, assign18090_e25629_d_n10, assign18090_e25629_d_n11, assign18090_e25629_d_n12, assign18090_e25629_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18090_e25625: f64 = (0.2 * locals.var_vmaxe);
        let assign18090_e25627: f64 = (assign18090_e25625 / locals.var_muun);
        (assign18090_e25627, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn12) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn12)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn17) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn17)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18090_e25629;
        locals.var_t2__blk546_dn0 = assign18090_e25629_d_n0;
        locals.var_t2__blk546_dn2 = assign18090_e25629_d_n2;
        locals.var_t2__blk546_dn6 = assign18090_e25629_d_n6;
        locals.var_t2__blk546_dn7 = assign18090_e25629_d_n7;
        locals.var_t2__blk546_dn10 = assign18090_e25629_d_n10;
        locals.var_t2__blk546_dn11 = assign18090_e25629_d_n11;
        locals.var_t2__blk546_dn12 = assign18090_e25629_d_n12;
        locals.var_t2__blk546_dn17 = assign18090_e25629_d_n17;

        let (assign18100_e25640, assign18100_e25640_d_n0, assign18100_e25640_d_n2, assign18100_e25640_d_n6, assign18100_e25640_d_n7, assign18100_e25640_d_n10, assign18100_e25640_d_n11, assign18100_e25640_d_n12, assign18100_e25640_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18100_e25633: f64 = (locals.var_ty__blk548 * locals.var_ty__blk548);
        let assign18100_e25636: f64 = (locals.var_t2__blk546 * locals.var_t2__blk546);
        let assign18100_e25637: f64 = (assign18100_e25633 + assign18100_e25636);
        let assign18100_e25638: f64 = (assign18100_e25637).sqrt();
        (assign18100_e25638, ((((locals.var_ty__blk548_dn0 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn0)) + ((locals.var_t2__blk546_dn0 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn0))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn2 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn2)) + ((locals.var_t2__blk546_dn2 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn2))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn6 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn6)) + ((locals.var_t2__blk546_dn6 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn6))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn7 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn7)) + ((locals.var_t2__blk546_dn7 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn7))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn10 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn10)) + ((locals.var_t2__blk546_dn10 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn10))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn11 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn11)) + ((locals.var_t2__blk546_dn11 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn11))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn12 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn12)) + ((locals.var_t2__blk546_dn12 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn12))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn17 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn17)) + ((locals.var_t2__blk546_dn17 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn17))) / (2.0 * assign18100_e25638)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn12, locals.var_ey_dn17,)
    }
};
        locals.var_ey = assign18100_e25640;
        locals.var_ey_dn0 = assign18100_e25640_d_n0;
        locals.var_ey_dn2 = assign18100_e25640_d_n2;
        locals.var_ey_dn6 = assign18100_e25640_d_n6;
        locals.var_ey_dn7 = assign18100_e25640_d_n7;
        locals.var_ey_dn10 = assign18100_e25640_d_n10;
        locals.var_ey_dn11 = assign18100_e25640_d_n11;
        locals.var_ey_dn12 = assign18100_e25640_d_n12;
        locals.var_ey_dn17 = assign18100_e25640_d_n17;

        let (assign18110_e25646, assign18110_e25646_d_n0, assign18110_e25646_d_n2, assign18110_e25646_d_n6, assign18110_e25646_d_n7, assign18110_e25646_d_n10, assign18110_e25646_d_n11, assign18110_e25646_d_n12, assign18110_e25646_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18110_e25644: f64 = (locals.var_muun * locals.var_ey);
        (assign18110_e25644, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn12 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn12)), ((locals.var_muun_dn17 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn17)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn12, locals.var_em_dn17,)
    }
};
        locals.var_em = assign18110_e25646;
        locals.var_em_dn0 = assign18110_e25646_d_n0;
        locals.var_em_dn2 = assign18110_e25646_d_n2;
        locals.var_em_dn6 = assign18110_e25646_d_n6;
        locals.var_em_dn7 = assign18110_e25646_d_n7;
        locals.var_em_dn10 = assign18110_e25646_d_n10;
        locals.var_em_dn11 = assign18110_e25646_d_n11;
        locals.var_em_dn12 = assign18110_e25646_d_n12;
        locals.var_em_dn17 = assign18110_e25646_d_n17;

    }

    pub(super) fn stamp_transient_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18120_e25652, assign18120_e25652_d_n0, assign18120_e25652_d_n2, assign18120_e25652_d_n6, assign18120_e25652_d_n7, assign18120_e25652_d_n10, assign18120_e25652_d_n11, assign18120_e25652_d_n12, assign18120_e25652_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18120_e25650: f64 = (locals.var_em / locals.var_vmaxe);
        (assign18120_e25650, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn12 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn12)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn17 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn17)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1__blk547, locals.var_t1__blk547_dn0, locals.var_t1__blk547_dn2, locals.var_t1__blk547_dn6, locals.var_t1__blk547_dn7, locals.var_t1__blk547_dn10, locals.var_t1__blk547_dn11, locals.var_t1__blk547_dn12, locals.var_t1__blk547_dn17,)
    }
};
        locals.var_t1__blk547 = assign18120_e25652;
        locals.var_t1__blk547_dn0 = assign18120_e25652_d_n0;
        locals.var_t1__blk547_dn2 = assign18120_e25652_d_n2;
        locals.var_t1__blk547_dn6 = assign18120_e25652_d_n6;
        locals.var_t1__blk547_dn7 = assign18120_e25652_d_n7;
        locals.var_t1__blk547_dn10 = assign18120_e25652_d_n10;
        locals.var_t1__blk547_dn11 = assign18120_e25652_d_n11;
        locals.var_t1__blk547_dn12 = assign18120_e25652_d_n12;
        locals.var_t1__blk547_dn17 = assign18120_e25652_d_n17;

        let assign18130_e25656: f64 = (10.0 * 2.220446049250313e-16);
        let assign18130_e25657: f64 = (1.0 - assign18130_e25656);
        let assign18130_e25664: f64 = (10.0 * 2.220446049250313e-16);
        let assign18130_e25665: f64 = (1.0 + assign18130_e25664);
        let assign18130_e25667: f64 = if ((assign18130_e25657 <= p.p113) && (p.p113 <= assign18130_e25665)) { 1.0 } else { 0.0 };
        locals.var_guard555 = assign18130_e25667;

        let (assign18140_e25673, assign18140_e25673_d_n0, assign18140_e25673_d_n2, assign18140_e25673_d_n6, assign18140_e25673_d_n7, assign18140_e25673_d_n10, assign18140_e25673_d_n11, assign18140_e25673_d_n12, assign18140_e25673_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard555 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk550, locals.var_t3__blk550_dn0, locals.var_t3__blk550_dn2, locals.var_t3__blk550_dn6, locals.var_t3__blk550_dn7, locals.var_t3__blk550_dn10, locals.var_t3__blk550_dn11, locals.var_t3__blk550_dn12, locals.var_t3__blk550_dn17,)
    }
};
        locals.var_t3__blk550 = assign18140_e25673;
        locals.var_t3__blk550_dn0 = assign18140_e25673_d_n0;
        locals.var_t3__blk550_dn2 = assign18140_e25673_d_n2;
        locals.var_t3__blk550_dn6 = assign18140_e25673_d_n6;
        locals.var_t3__blk550_dn7 = assign18140_e25673_d_n7;
        locals.var_t3__blk550_dn10 = assign18140_e25673_d_n10;
        locals.var_t3__blk550_dn11 = assign18140_e25673_d_n11;
        locals.var_t3__blk550_dn12 = assign18140_e25673_d_n12;
        locals.var_t3__blk550_dn17 = assign18140_e25673_d_n17;

        let assign18150_e25677: f64 = (10.0 * 2.220446049250313e-16);
        let assign18150_e25678: f64 = (2.0 - assign18150_e25677);
        let assign18150_e25685: f64 = (10.0 * 2.220446049250313e-16);
        let assign18150_e25686: f64 = (2.0 + assign18150_e25685);
        let assign18150_e25688: f64 = if ((assign18150_e25678 <= p.p113) && (p.p113 <= assign18150_e25686)) { 1.0 } else { 0.0 };
        locals.var_guard556 = assign18150_e25688;

        let (assign18160_e25697, assign18160_e25697_d_n0, assign18160_e25697_d_n2, assign18160_e25697_d_n6, assign18160_e25697_d_n7, assign18160_e25697_d_n10, assign18160_e25697_d_n11, assign18160_e25697_d_n12, assign18160_e25697_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard555 == 0.0)) && (locals.var_guard556 != 0.0)) {
        (locals.var_t1__blk547, locals.var_t1__blk547_dn0, locals.var_t1__blk547_dn2, locals.var_t1__blk547_dn6, locals.var_t1__blk547_dn7, locals.var_t1__blk547_dn10, locals.var_t1__blk547_dn11, locals.var_t1__blk547_dn12, locals.var_t1__blk547_dn17,)
    } else {
        (locals.var_t3__blk550, locals.var_t3__blk550_dn0, locals.var_t3__blk550_dn2, locals.var_t3__blk550_dn6, locals.var_t3__blk550_dn7, locals.var_t3__blk550_dn10, locals.var_t3__blk550_dn11, locals.var_t3__blk550_dn12, locals.var_t3__blk550_dn17,)
    }
};
        locals.var_t3__blk550 = assign18160_e25697;
        locals.var_t3__blk550_dn0 = assign18160_e25697_d_n0;
        locals.var_t3__blk550_dn2 = assign18160_e25697_d_n2;
        locals.var_t3__blk550_dn6 = assign18160_e25697_d_n6;
        locals.var_t3__blk550_dn7 = assign18160_e25697_d_n7;
        locals.var_t3__blk550_dn10 = assign18160_e25697_d_n10;
        locals.var_t3__blk550_dn11 = assign18160_e25697_d_n11;
        locals.var_t3__blk550_dn12 = assign18160_e25697_d_n12;
        locals.var_t3__blk550_dn17 = assign18160_e25697_d_n17;

        let (assign18170_e25711, assign18170_e25711_d_n0, assign18170_e25711_d_n2, assign18170_e25711_d_n6, assign18170_e25711_d_n7, assign18170_e25711_d_n10, assign18170_e25711_d_n11, assign18170_e25711_d_n12, assign18170_e25711_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard555 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign18170_e25708: f64 = (p.p113 - 1.0);
        let assign18170_e25709: f64 = (locals.var_t1__blk547).powf(assign18170_e25708);
        (assign18170_e25709, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn0)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn0 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn2)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn2 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn6)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn6 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn7)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn7 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn10)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn10 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn11)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn11 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn12)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn12 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn17)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn17 / locals.var_t1__blk547))) },)
    } else {
        (locals.var_t3__blk550, locals.var_t3__blk550_dn0, locals.var_t3__blk550_dn2, locals.var_t3__blk550_dn6, locals.var_t3__blk550_dn7, locals.var_t3__blk550_dn10, locals.var_t3__blk550_dn11, locals.var_t3__blk550_dn12, locals.var_t3__blk550_dn17,)
    }
};
        locals.var_t3__blk550 = assign18170_e25711;
        locals.var_t3__blk550_dn0 = assign18170_e25711_d_n0;
        locals.var_t3__blk550_dn2 = assign18170_e25711_d_n2;
        locals.var_t3__blk550_dn6 = assign18170_e25711_d_n6;
        locals.var_t3__blk550_dn7 = assign18170_e25711_d_n7;
        locals.var_t3__blk550_dn10 = assign18170_e25711_d_n10;
        locals.var_t3__blk550_dn11 = assign18170_e25711_d_n11;
        locals.var_t3__blk550_dn12 = assign18170_e25711_d_n12;
        locals.var_t3__blk550_dn17 = assign18170_e25711_d_n17;

        let (assign18180_e25717, assign18180_e25717_d_n0, assign18180_e25717_d_n2, assign18180_e25717_d_n6, assign18180_e25717_d_n7, assign18180_e25717_d_n10, assign18180_e25717_d_n11, assign18180_e25717_d_n12, assign18180_e25717_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18180_e25715: f64 = (locals.var_t1__blk547 * locals.var_t3__blk550);
        (assign18180_e25715, ((locals.var_t1__blk547_dn0 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn0)), ((locals.var_t1__blk547_dn2 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn2)), ((locals.var_t1__blk547_dn6 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn6)), ((locals.var_t1__blk547_dn7 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn7)), ((locals.var_t1__blk547_dn10 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn10)), ((locals.var_t1__blk547_dn11 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn11)), ((locals.var_t1__blk547_dn12 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn12)), ((locals.var_t1__blk547_dn17 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn17)),)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18180_e25717;
        locals.var_t2__blk546_dn0 = assign18180_e25717_d_n0;
        locals.var_t2__blk546_dn2 = assign18180_e25717_d_n2;
        locals.var_t2__blk546_dn6 = assign18180_e25717_d_n6;
        locals.var_t2__blk546_dn7 = assign18180_e25717_d_n7;
        locals.var_t2__blk546_dn10 = assign18180_e25717_d_n10;
        locals.var_t2__blk546_dn11 = assign18180_e25717_d_n11;
        locals.var_t2__blk546_dn12 = assign18180_e25717_d_n12;
        locals.var_t2__blk546_dn17 = assign18180_e25717_d_n17;

        let (assign18190_e25723, assign18190_e25723_d_n0, assign18190_e25723_d_n2, assign18190_e25723_d_n6, assign18190_e25723_d_n7, assign18190_e25723_d_n10, assign18190_e25723_d_n11, assign18190_e25723_d_n12, assign18190_e25723_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18190_e25721: f64 = (1.0 + locals.var_t2__blk546);
        (assign18190_e25721, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    } else {
        (locals.var_t4__blk551, locals.var_t4__blk551_dn0, locals.var_t4__blk551_dn2, locals.var_t4__blk551_dn6, locals.var_t4__blk551_dn7, locals.var_t4__blk551_dn10, locals.var_t4__blk551_dn11, locals.var_t4__blk551_dn12, locals.var_t4__blk551_dn17,)
    }
};
        locals.var_t4__blk551 = assign18190_e25723;
        locals.var_t4__blk551_dn0 = assign18190_e25723_d_n0;
        locals.var_t4__blk551_dn2 = assign18190_e25723_d_n2;
        locals.var_t4__blk551_dn6 = assign18190_e25723_d_n6;
        locals.var_t4__blk551_dn7 = assign18190_e25723_d_n7;
        locals.var_t4__blk551_dn10 = assign18190_e25723_d_n10;
        locals.var_t4__blk551_dn11 = assign18190_e25723_d_n11;
        locals.var_t4__blk551_dn12 = assign18190_e25723_d_n12;
        locals.var_t4__blk551_dn17 = assign18190_e25723_d_n17;

        let assign18200_e25727: f64 = (10.0 * 2.220446049250313e-16);
        let assign18200_e25728: f64 = (1.0 - assign18200_e25727);
        let assign18200_e25735: f64 = (10.0 * 2.220446049250313e-16);
        let assign18200_e25736: f64 = (1.0 + assign18200_e25735);
        let assign18200_e25738: f64 = if ((assign18200_e25728 <= p.p113) && (p.p113 <= assign18200_e25736)) { 1.0 } else { 0.0 };
        locals.var_guard557 = assign18200_e25738;

        let (assign18210_e25746, assign18210_e25746_d_n0, assign18210_e25746_d_n2, assign18210_e25746_d_n6, assign18210_e25746_d_n7, assign18210_e25746_d_n10, assign18210_e25746_d_n11, assign18210_e25746_d_n12, assign18210_e25746_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard557 != 0.0)) {
        let assign18210_e25744: f64 = (1.0 / locals.var_t4__blk551);
        (assign18210_e25744, (-(locals.var_t4__blk551_dn0 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn2 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn6 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn7 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn10 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn11 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn12 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn17 / (locals.var_t4__blk551 * locals.var_t4__blk551))),)
    } else {
        (locals.var_t5__blk552, locals.var_t5__blk552_dn0, locals.var_t5__blk552_dn2, locals.var_t5__blk552_dn6, locals.var_t5__blk552_dn7, locals.var_t5__blk552_dn10, locals.var_t5__blk552_dn11, locals.var_t5__blk552_dn12, locals.var_t5__blk552_dn17,)
    }
};
        locals.var_t5__blk552 = assign18210_e25746;
        locals.var_t5__blk552_dn0 = assign18210_e25746_d_n0;
        locals.var_t5__blk552_dn2 = assign18210_e25746_d_n2;
        locals.var_t5__blk552_dn6 = assign18210_e25746_d_n6;
        locals.var_t5__blk552_dn7 = assign18210_e25746_d_n7;
        locals.var_t5__blk552_dn10 = assign18210_e25746_d_n10;
        locals.var_t5__blk552_dn11 = assign18210_e25746_d_n11;
        locals.var_t5__blk552_dn12 = assign18210_e25746_d_n12;
        locals.var_t5__blk552_dn17 = assign18210_e25746_d_n17;

        let assign18220_e25750: f64 = (10.0 * 2.220446049250313e-16);
        let assign18220_e25751: f64 = (2.0 - assign18220_e25750);
        let assign18220_e25758: f64 = (10.0 * 2.220446049250313e-16);
        let assign18220_e25759: f64 = (2.0 + assign18220_e25758);
        let assign18220_e25761: f64 = if ((assign18220_e25751 <= p.p113) && (p.p113 <= assign18220_e25759)) { 1.0 } else { 0.0 };
        locals.var_guard558 = assign18220_e25761;

        let (assign18230_e25773, assign18230_e25773_d_n0, assign18230_e25773_d_n2, assign18230_e25773_d_n6, assign18230_e25773_d_n7, assign18230_e25773_d_n10, assign18230_e25773_d_n11, assign18230_e25773_d_n12, assign18230_e25773_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 != 0.0)) {
        let assign18230_e25770: f64 = (locals.var_t4__blk551).sqrt();
        let assign18230_e25771: f64 = (1.0 / assign18230_e25770);
        (assign18230_e25771, (-((locals.var_t4__blk551_dn0 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn2 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn6 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn7 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn10 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn11 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn12 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn17 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))),)
    } else {
        (locals.var_t5__blk552, locals.var_t5__blk552_dn0, locals.var_t5__blk552_dn2, locals.var_t5__blk552_dn6, locals.var_t5__blk552_dn7, locals.var_t5__blk552_dn10, locals.var_t5__blk552_dn11, locals.var_t5__blk552_dn12, locals.var_t5__blk552_dn17,)
    }
};
        locals.var_t5__blk552 = assign18230_e25773;
        locals.var_t5__blk552_dn0 = assign18230_e25773_d_n0;
        locals.var_t5__blk552_dn2 = assign18230_e25773_d_n2;
        locals.var_t5__blk552_dn6 = assign18230_e25773_d_n6;
        locals.var_t5__blk552_dn7 = assign18230_e25773_d_n7;
        locals.var_t5__blk552_dn10 = assign18230_e25773_d_n10;
        locals.var_t5__blk552_dn11 = assign18230_e25773_d_n11;
        locals.var_t5__blk552_dn12 = assign18230_e25773_d_n12;
        locals.var_t5__blk552_dn17 = assign18230_e25773_d_n17;

        let (assign18240_e25790, assign18240_e25790_d_n0, assign18240_e25790_d_n2, assign18240_e25790_d_n6, assign18240_e25790_d_n7, assign18240_e25790_d_n10, assign18240_e25790_d_n11, assign18240_e25790_d_n12, assign18240_e25790_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign18240_e25783: f64 = (-1.0);
        let assign18240_e25785: f64 = (assign18240_e25783 / p.p113);
        let assign18240_e25787: f64 = (assign18240_e25785 - 1.0);
        let assign18240_e25788: f64 = (locals.var_t4__blk551).powf(assign18240_e25787);
        (assign18240_e25788, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn0)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn0 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn2)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn2 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn6)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn6 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn7)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn7 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn10)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn10 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn11)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn11 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn12)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn12 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn17)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn17 / locals.var_t4__blk551))) },)
    } else {
        (locals.var_t6__blk553, locals.var_t6__blk553_dn0, locals.var_t6__blk553_dn2, locals.var_t6__blk553_dn6, locals.var_t6__blk553_dn7, locals.var_t6__blk553_dn10, locals.var_t6__blk553_dn11, locals.var_t6__blk553_dn12, locals.var_t6__blk553_dn17,)
    }
};
        locals.var_t6__blk553 = assign18240_e25790;
        locals.var_t6__blk553_dn0 = assign18240_e25790_d_n0;
        locals.var_t6__blk553_dn2 = assign18240_e25790_d_n2;
        locals.var_t6__blk553_dn6 = assign18240_e25790_d_n6;
        locals.var_t6__blk553_dn7 = assign18240_e25790_d_n7;
        locals.var_t6__blk553_dn10 = assign18240_e25790_d_n10;
        locals.var_t6__blk553_dn11 = assign18240_e25790_d_n11;
        locals.var_t6__blk553_dn12 = assign18240_e25790_d_n12;
        locals.var_t6__blk553_dn17 = assign18240_e25790_d_n17;

        let (assign18250_e25802, assign18250_e25802_d_n0, assign18250_e25802_d_n2, assign18250_e25802_d_n6, assign18250_e25802_d_n7, assign18250_e25802_d_n10, assign18250_e25802_d_n11, assign18250_e25802_d_n12, assign18250_e25802_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign18250_e25800: f64 = (locals.var_t4__blk551 * locals.var_t6__blk553);
        (assign18250_e25800, ((locals.var_t4__blk551_dn0 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn0)), ((locals.var_t4__blk551_dn2 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn2)), ((locals.var_t4__blk551_dn6 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn6)), ((locals.var_t4__blk551_dn7 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn7)), ((locals.var_t4__blk551_dn10 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn10)), ((locals.var_t4__blk551_dn11 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn11)), ((locals.var_t4__blk551_dn12 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn12)), ((locals.var_t4__blk551_dn17 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn17)),)
    } else {
        (locals.var_t5__blk552, locals.var_t5__blk552_dn0, locals.var_t5__blk552_dn2, locals.var_t5__blk552_dn6, locals.var_t5__blk552_dn7, locals.var_t5__blk552_dn10, locals.var_t5__blk552_dn11, locals.var_t5__blk552_dn12, locals.var_t5__blk552_dn17,)
    }
};
        locals.var_t5__blk552 = assign18250_e25802;
        locals.var_t5__blk552_dn0 = assign18250_e25802_d_n0;
        locals.var_t5__blk552_dn2 = assign18250_e25802_d_n2;
        locals.var_t5__blk552_dn6 = assign18250_e25802_d_n6;
        locals.var_t5__blk552_dn7 = assign18250_e25802_d_n7;
        locals.var_t5__blk552_dn10 = assign18250_e25802_d_n10;
        locals.var_t5__blk552_dn11 = assign18250_e25802_d_n11;
        locals.var_t5__blk552_dn12 = assign18250_e25802_d_n12;
        locals.var_t5__blk552_dn17 = assign18250_e25802_d_n17;

        let (assign18260_e25808, assign18260_e25808_d_n0, assign18260_e25808_d_n2, assign18260_e25808_d_n6, assign18260_e25808_d_n7, assign18260_e25808_d_n10, assign18260_e25808_d_n11, assign18260_e25808_d_n12, assign18260_e25808_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18260_e25806: f64 = (locals.var_muun * locals.var_t5__blk552);
        (assign18260_e25806, ((locals.var_muun_dn0 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn0)), ((locals.var_muun_dn2 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn2)), ((locals.var_muun_dn6 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn6)), ((locals.var_muun_dn7 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn7)), ((locals.var_muun_dn10 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn10)), ((locals.var_muun_dn11 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn11)), ((locals.var_muun_dn12 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn12)), ((locals.var_muun_dn17 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn17)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn12, locals.var_mu_dn17,)
    }
};
        locals.var_mu = assign18260_e25808;
        locals.var_mu_dn0 = assign18260_e25808_d_n0;
        locals.var_mu_dn2 = assign18260_e25808_d_n2;
        locals.var_mu_dn6 = assign18260_e25808_d_n6;
        locals.var_mu_dn7 = assign18260_e25808_d_n7;
        locals.var_mu_dn10 = assign18260_e25808_d_n10;
        locals.var_mu_dn11 = assign18260_e25808_d_n11;
        locals.var_mu_dn12 = assign18260_e25808_d_n12;
        locals.var_mu_dn17 = assign18260_e25808_d_n17;

        let (assign18270_e25818, assign18270_e25818_d_n0, assign18270_e25818_d_n2, assign18270_e25818_d_n6, assign18270_e25818_d_n7, assign18270_e25818_d_n10, assign18270_e25818_d_n11, assign18270_e25818_d_n12, assign18270_e25818_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18270_e25812: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign18270_e25815: f64 = (locals.var_leff - locals.var_lred);
        let assign18270_e25816: f64 = (assign18270_e25812 / assign18270_e25815);
        (assign18270_e25816, (-((assign18270_e25812 * (-locals.var_lred_dn0)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn2)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn6)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn7)) / (assign18270_e25815 * assign18270_e25815))), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * assign18270_e25815) - (assign18270_e25812 * (-locals.var_lred_dn10))) / (assign18270_e25815 * assign18270_e25815)), (-((assign18270_e25812 * (-locals.var_lred_dn11)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn12)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn17)) / (assign18270_e25815 * assign18270_e25815))),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn12, locals.var_betawl_dn17,)
    }
};
        locals.var_betawl = assign18270_e25818;
        locals.var_betawl_dn0 = assign18270_e25818_d_n0;
        locals.var_betawl_dn2 = assign18270_e25818_d_n2;
        locals.var_betawl_dn6 = assign18270_e25818_d_n6;
        locals.var_betawl_dn7 = assign18270_e25818_d_n7;
        locals.var_betawl_dn10 = assign18270_e25818_d_n10;
        locals.var_betawl_dn11 = assign18270_e25818_d_n11;
        locals.var_betawl_dn12 = assign18270_e25818_d_n12;
        locals.var_betawl_dn17 = assign18270_e25818_d_n17;

        let (assign18280_e25826, assign18280_e25826_d_n0, assign18280_e25826_d_n2, assign18280_e25826_d_n6, assign18280_e25826_d_n7, assign18280_e25826_d_n10, assign18280_e25826_d_n11, assign18280_e25826_d_n12, assign18280_e25826_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18280_e25822: f64 = (locals.var_betawl * locals.var_idd);
        let assign18280_e25824: f64 = (assign18280_e25822 * locals.var_mu);
        (assign18280_e25824, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn12)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn17)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn17)),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn12, locals.var_ids0_dn17,)
    }
};
        locals.var_ids0 = assign18280_e25826;
        locals.var_ids0_dn0 = assign18280_e25826_d_n0;
        locals.var_ids0_dn2 = assign18280_e25826_d_n2;
        locals.var_ids0_dn6 = assign18280_e25826_d_n6;
        locals.var_ids0_dn7 = assign18280_e25826_d_n7;
        locals.var_ids0_dn10 = assign18280_e25826_d_n10;
        locals.var_ids0_dn11 = assign18280_e25826_d_n11;
        locals.var_ids0_dn12 = assign18280_e25826_d_n12;
        locals.var_ids0_dn17 = assign18280_e25826_d_n17;

        let (assign18290_e25830, assign18290_e25830_d_n0, assign18290_e25830_d_n2, assign18290_e25830_d_n6, assign18290_e25830_d_n7, assign18290_e25830_d_n10, assign18290_e25830_d_n11, assign18290_e25830_d_n12, assign18290_e25830_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    }
};
        locals.var_idspt = assign18290_e25830;
        locals.var_idspt_dn0 = assign18290_e25830_d_n0;
        locals.var_idspt_dn2 = assign18290_e25830_d_n2;
        locals.var_idspt_dn6 = assign18290_e25830_d_n6;
        locals.var_idspt_dn7 = assign18290_e25830_d_n7;
        locals.var_idspt_dn10 = assign18290_e25830_d_n10;
        locals.var_idspt_dn11 = assign18290_e25830_d_n11;
        locals.var_idspt_dn12 = assign18290_e25830_d_n12;
        locals.var_idspt_dn17 = assign18290_e25830_d_n17;

        let assign18300_e25837: f64 = if ((p.p281 > 0.0) && (p.p244 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard568 = assign18300_e25837;

        let (assign18310_e25847, assign18310_e25847_d_n0, assign18310_e25847_d_n2, assign18310_e25847_d_n6, assign18310_e25847_d_n7, assign18310_e25847_d_n10, assign18310_e25847_d_n11, assign18310_e25847_d_n12, assign18310_e25847_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18310_e25844: f64 = (locals.var_vds - locals.var_pds);
        let assign18310_e25845: f64 = (0.5 * assign18310_e25844);
        (assign18310_e25845, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn12 - locals.var_pds_dn12)), (0.5 * (locals.var_vds_dn17 - locals.var_pds_dn17)),)
    } else {
        (locals.var_t1__blk559, locals.var_t1__blk559_dn0, locals.var_t1__blk559_dn2, locals.var_t1__blk559_dn6, locals.var_t1__blk559_dn7, locals.var_t1__blk559_dn10, locals.var_t1__blk559_dn11, locals.var_t1__blk559_dn12, locals.var_t1__blk559_dn17,)
    }
};
        locals.var_t1__blk559 = assign18310_e25847;
        locals.var_t1__blk559_dn0 = assign18310_e25847_d_n0;
        locals.var_t1__blk559_dn2 = assign18310_e25847_d_n2;
        locals.var_t1__blk559_dn6 = assign18310_e25847_d_n6;
        locals.var_t1__blk559_dn7 = assign18310_e25847_d_n7;
        locals.var_t1__blk559_dn10 = assign18310_e25847_d_n10;
        locals.var_t1__blk559_dn11 = assign18310_e25847_d_n11;
        locals.var_t1__blk559_dn12 = assign18310_e25847_d_n12;
        locals.var_t1__blk559_dn17 = assign18310_e25847_d_n17;

        let (assign18320_e25857, assign18320_e25857_d_n0, assign18320_e25857_d_n2, assign18320_e25857_d_n6, assign18320_e25857_d_n7, assign18320_e25857_d_n10, assign18320_e25857_d_n11, assign18320_e25857_d_n12, assign18320_e25857_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18320_e25853: f64 = (2.0 * locals.var_t1__blk559);
        let assign18320_e25855: f64 = (assign18320_e25853 / 0.01);
        (assign18320_e25855, ((2.0 * locals.var_t1__blk559_dn0) / 0.01), ((2.0 * locals.var_t1__blk559_dn2) / 0.01), ((2.0 * locals.var_t1__blk559_dn6) / 0.01), ((2.0 * locals.var_t1__blk559_dn7) / 0.01), ((2.0 * locals.var_t1__blk559_dn10) / 0.01), ((2.0 * locals.var_t1__blk559_dn11) / 0.01), ((2.0 * locals.var_t1__blk559_dn12) / 0.01), ((2.0 * locals.var_t1__blk559_dn17) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18320_e25857;
        locals.var_tmf1_dn0 = assign18320_e25857_d_n0;
        locals.var_tmf1_dn2 = assign18320_e25857_d_n2;
        locals.var_tmf1_dn6 = assign18320_e25857_d_n6;
        locals.var_tmf1_dn7 = assign18320_e25857_d_n7;
        locals.var_tmf1_dn10 = assign18320_e25857_d_n10;
        locals.var_tmf1_dn11 = assign18320_e25857_d_n11;
        locals.var_tmf1_dn12 = assign18320_e25857_d_n12;
        locals.var_tmf1_dn17 = assign18320_e25857_d_n17;

        let (assign18330_e25899, assign18330_e25899_d_n0, assign18330_e25899_d_n2, assign18330_e25899_d_n6, assign18330_e25899_d_n7, assign18330_e25899_d_n10, assign18330_e25899_d_n11, assign18330_e25899_d_n12, assign18330_e25899_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18330_e25865: f64 = (1.0 / 2.0);
        let assign18330_e25869: f64 = (1.0 / 6.0);
        let assign18330_e25873: f64 = (1.0 / 24.0);
        let assign18330_e25877: f64 = (1.0 / 120.0);
        let assign18330_e25881: f64 = (1.0 / 720.0);
        let assign18330_e25885: f64 = (1.0 / 5040.0);
        let assign18330_e25886: f64 = (locals.var_tmf1 * assign18330_e25885);
        let assign18330_e25887: f64 = (assign18330_e25881 + assign18330_e25886);
        let assign18330_e25888: f64 = (locals.var_tmf1 * assign18330_e25887);
        let assign18330_e25889: f64 = (assign18330_e25877 + assign18330_e25888);
        let assign18330_e25890: f64 = (locals.var_tmf1 * assign18330_e25889);
        let assign18330_e25891: f64 = (assign18330_e25873 + assign18330_e25890);
        let assign18330_e25892: f64 = (locals.var_tmf1 * assign18330_e25891);
        let assign18330_e25893: f64 = (assign18330_e25869 + assign18330_e25892);
        let assign18330_e25894: f64 = (locals.var_tmf1 * assign18330_e25893);
        let assign18330_e25895: f64 = (assign18330_e25865 + assign18330_e25894);
        let assign18330_e25896: f64 = (locals.var_tmf1 * assign18330_e25895);
        let assign18330_e25897: f64 = (1.0 + assign18330_e25896);
        (assign18330_e25897, ((locals.var_tmf1_dn0 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn2 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn6 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn7 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn10 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn11 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn12 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn17 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign18330_e25885))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign18330_e25899;
        locals.var_tmf2_dn0 = assign18330_e25899_d_n0;
        locals.var_tmf2_dn2 = assign18330_e25899_d_n2;
        locals.var_tmf2_dn6 = assign18330_e25899_d_n6;
        locals.var_tmf2_dn7 = assign18330_e25899_d_n7;
        locals.var_tmf2_dn10 = assign18330_e25899_d_n10;
        locals.var_tmf2_dn11 = assign18330_e25899_d_n11;
        locals.var_tmf2_dn12 = assign18330_e25899_d_n12;
        locals.var_tmf2_dn17 = assign18330_e25899_d_n17;

        let (assign18340_e25907, assign18340_e25907_d_n0, assign18340_e25907_d_n2, assign18340_e25907_d_n6, assign18340_e25907_d_n7, assign18340_e25907_d_n10, assign18340_e25907_d_n11, assign18340_e25907_d_n12, assign18340_e25907_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18340_e25905: f64 = (0.01 / locals.var_tmf2);
        (assign18340_e25905, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6__blk565, locals.var_t6__blk565_dn0, locals.var_t6__blk565_dn2, locals.var_t6__blk565_dn6, locals.var_t6__blk565_dn7, locals.var_t6__blk565_dn10, locals.var_t6__blk565_dn11, locals.var_t6__blk565_dn12, locals.var_t6__blk565_dn17,)
    }
};
        locals.var_t6__blk565 = assign18340_e25907;
        locals.var_t6__blk565_dn0 = assign18340_e25907_d_n0;
        locals.var_t6__blk565_dn2 = assign18340_e25907_d_n2;
        locals.var_t6__blk565_dn6 = assign18340_e25907_d_n6;
        locals.var_t6__blk565_dn7 = assign18340_e25907_d_n7;
        locals.var_t6__blk565_dn10 = assign18340_e25907_d_n10;
        locals.var_t6__blk565_dn11 = assign18340_e25907_d_n11;
        locals.var_t6__blk565_dn12 = assign18340_e25907_d_n12;
        locals.var_t6__blk565_dn17 = assign18340_e25907_d_n17;

        let (assign18350_e25917, assign18350_e25917_d_n0, assign18350_e25917_d_n2, assign18350_e25917_d_n6, assign18350_e25917_d_n7, assign18350_e25917_d_n10, assign18350_e25917_d_n11, assign18350_e25917_d_n12, assign18350_e25917_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18350_e25914: f64 = (locals.var_ps0 + locals.var_t6__blk565);
        let assign18350_e25915: f64 = (1.1 - assign18350_e25914);
        (assign18350_e25915, (-(locals.var_ps0_dn0 + locals.var_t6__blk565_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6__blk565_dn2)), (-(locals.var_ps0_dn6 + locals.var_t6__blk565_dn6)), (-(locals.var_ps0_dn7 + locals.var_t6__blk565_dn7)), (-(locals.var_ps0_dn10 + locals.var_t6__blk565_dn10)), (-(locals.var_ps0_dn11 + locals.var_t6__blk565_dn11)), (-(locals.var_ps0_dn12 + locals.var_t6__blk565_dn12)), (-(locals.var_ps0_dn17 + locals.var_t6__blk565_dn17)),)
    } else {
        (locals.var_t1__blk559, locals.var_t1__blk559_dn0, locals.var_t1__blk559_dn2, locals.var_t1__blk559_dn6, locals.var_t1__blk559_dn7, locals.var_t1__blk559_dn10, locals.var_t1__blk559_dn11, locals.var_t1__blk559_dn12, locals.var_t1__blk559_dn17,)
    }
};
        locals.var_t1__blk559 = assign18350_e25917;
        locals.var_t1__blk559_dn0 = assign18350_e25917_d_n0;
        locals.var_t1__blk559_dn2 = assign18350_e25917_d_n2;
        locals.var_t1__blk559_dn6 = assign18350_e25917_d_n6;
        locals.var_t1__blk559_dn7 = assign18350_e25917_d_n7;
        locals.var_t1__blk559_dn10 = assign18350_e25917_d_n10;
        locals.var_t1__blk559_dn11 = assign18350_e25917_d_n11;
        locals.var_t1__blk559_dn12 = assign18350_e25917_d_n12;
        locals.var_t1__blk559_dn17 = assign18350_e25917_d_n17;

        let (assign18360_e25932, assign18360_e25932_d_n0, assign18360_e25932_d_n2, assign18360_e25932_d_n6, assign18360_e25932_d_n7, assign18360_e25932_d_n10, assign18360_e25932_d_n11, assign18360_e25932_d_n12, assign18360_e25932_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18360_e25923: f64 = (locals.var_t1__blk559 * locals.var_t1__blk559);
        let assign18360_e25926: f64 = (4.0 * 0.05);
        let assign18360_e25928: f64 = (assign18360_e25926 * 0.05);
        let assign18360_e25929: f64 = (assign18360_e25923 + assign18360_e25928);
        let assign18360_e25930: f64 = (assign18360_e25929).sqrt();
        (assign18360_e25930, (((locals.var_t1__blk559_dn0 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn0)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn2 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn2)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn6 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn6)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn7 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn7)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn10 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn10)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn11 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn11)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn12 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn12)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn17 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn17)) / (2.0 * assign18360_e25930)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18360_e25932;
        locals.var_tmf1_dn0 = assign18360_e25932_d_n0;
        locals.var_tmf1_dn2 = assign18360_e25932_d_n2;
        locals.var_tmf1_dn6 = assign18360_e25932_d_n6;
        locals.var_tmf1_dn7 = assign18360_e25932_d_n7;
        locals.var_tmf1_dn10 = assign18360_e25932_d_n10;
        locals.var_tmf1_dn11 = assign18360_e25932_d_n11;
        locals.var_tmf1_dn12 = assign18360_e25932_d_n12;
        locals.var_tmf1_dn17 = assign18360_e25932_d_n17;

        let (assign18370_e25946, assign18370_e25946_d_n0, assign18370_e25946_d_n2, assign18370_e25946_d_n6, assign18370_e25946_d_n7, assign18370_e25946_d_n10, assign18370_e25946_d_n11, assign18370_e25946_d_n12, assign18370_e25946_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18370_e25939: f64 = (locals.var_t1__blk559 + locals.var_tmf1);
        let assign18370_e25940: f64 = (0.5 * assign18370_e25939);
        let assign18370_e25943: f64 = (1e-10 * 0.05);
        let assign18370_e25944: f64 = (assign18370_e25940 + assign18370_e25943);
        (assign18370_e25944, (0.5 * (locals.var_t1__blk559_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1__blk559_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1__blk559_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1__blk559_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1__blk559_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1__blk559_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1__blk559_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1__blk559_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk567, locals.var_t2__blk567_dn0, locals.var_t2__blk567_dn2, locals.var_t2__blk567_dn6, locals.var_t2__blk567_dn7, locals.var_t2__blk567_dn10, locals.var_t2__blk567_dn11, locals.var_t2__blk567_dn12, locals.var_t2__blk567_dn17,)
    }
};
        locals.var_t2__blk567 = assign18370_e25946;
        locals.var_t2__blk567_dn0 = assign18370_e25946_d_n0;
        locals.var_t2__blk567_dn2 = assign18370_e25946_d_n2;
        locals.var_t2__blk567_dn6 = assign18370_e25946_d_n6;
        locals.var_t2__blk567_dn7 = assign18370_e25946_d_n7;
        locals.var_t2__blk567_dn10 = assign18370_e25946_d_n10;
        locals.var_t2__blk567_dn11 = assign18370_e25946_d_n11;
        locals.var_t2__blk567_dn12 = assign18370_e25946_d_n12;
        locals.var_t2__blk567_dn17 = assign18370_e25946_d_n17;

        let assign18380_e25949: f64 = if locals.var_t2__blk567 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign18380_e25949;

        let (assign18390_e25957, assign18390_e25957_d_n0, assign18390_e25957_d_n2, assign18390_e25957_d_n6, assign18390_e25957_d_n7, assign18390_e25957_d_n10, assign18390_e25957_d_n11, assign18390_e25957_d_n12, assign18390_e25957_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk567, locals.var_t2__blk567_dn0, locals.var_t2__blk567_dn2, locals.var_t2__blk567_dn6, locals.var_t2__blk567_dn7, locals.var_t2__blk567_dn10, locals.var_t2__blk567_dn11, locals.var_t2__blk567_dn12, locals.var_t2__blk567_dn17,)
    }
};
        locals.var_t2__blk567 = assign18390_e25957;
        locals.var_t2__blk567_dn0 = assign18390_e25957_d_n0;
        locals.var_t2__blk567_dn2 = assign18390_e25957_d_n2;
        locals.var_t2__blk567_dn6 = assign18390_e25957_d_n6;
        locals.var_t2__blk567_dn7 = assign18390_e25957_d_n7;
        locals.var_t2__blk567_dn10 = assign18390_e25957_d_n10;
        locals.var_t2__blk567_dn11 = assign18390_e25957_d_n11;
        locals.var_t2__blk567_dn12 = assign18390_e25957_d_n12;
        locals.var_t2__blk567_dn17 = assign18390_e25957_d_n17;

        let (assign18400_e25965, assign18400_e25965_d_n0, assign18400_e25965_d_n2, assign18400_e25965_d_n6, assign18400_e25965_d_n7, assign18400_e25965_d_n10, assign18400_e25965_d_n11, assign18400_e25965_d_n12, assign18400_e25965_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18400_e25963: f64 = (locals.var_beta * locals.var_ptl0);
        (assign18400_e25963, 0.0, 0.0, 0.0, 0.0, (locals.var_beta_dn10 * locals.var_ptl0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk560, locals.var_t0__blk560_dn0, locals.var_t0__blk560_dn2, locals.var_t0__blk560_dn6, locals.var_t0__blk560_dn7, locals.var_t0__blk560_dn10, locals.var_t0__blk560_dn11, locals.var_t0__blk560_dn12, locals.var_t0__blk560_dn17,)
    }
};
        locals.var_t0__blk560 = assign18400_e25965;
        locals.var_t0__blk560_dn0 = assign18400_e25965_d_n0;
        locals.var_t0__blk560_dn2 = assign18400_e25965_d_n2;
        locals.var_t0__blk560_dn6 = assign18400_e25965_d_n6;
        locals.var_t0__blk560_dn7 = assign18400_e25965_d_n7;
        locals.var_t0__blk560_dn10 = assign18400_e25965_d_n10;
        locals.var_t0__blk560_dn11 = assign18400_e25965_d_n11;
        locals.var_t0__blk560_dn12 = assign18400_e25965_d_n12;
        locals.var_t0__blk560_dn17 = assign18400_e25965_d_n17;

        let (assign18410_e25973, assign18410_e25973_d_n0, assign18410_e25973_d_n2, assign18410_e25973_d_n6, assign18410_e25973_d_n7, assign18410_e25973_d_n10, assign18410_e25973_d_n11, assign18410_e25973_d_n12, assign18410_e25973_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18410_e25971: f64 = (locals.var_c_fox * locals.var_t0__blk560);
        (assign18410_e25971, ((locals.var_c_fox_dn0 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn0)), ((locals.var_c_fox_dn2 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn2)), ((locals.var_c_fox_dn6 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn6)), ((locals.var_c_fox_dn7 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn7)), ((locals.var_c_fox_dn10 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn10)), ((locals.var_c_fox_dn11 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn11)), ((locals.var_c_fox_dn12 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn12)), ((locals.var_c_fox_dn17 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn17)),)
    } else {
        (locals.var_t3__blk561, locals.var_t3__blk561_dn0, locals.var_t3__blk561_dn2, locals.var_t3__blk561_dn6, locals.var_t3__blk561_dn7, locals.var_t3__blk561_dn10, locals.var_t3__blk561_dn11, locals.var_t3__blk561_dn12, locals.var_t3__blk561_dn17,)
    }
};
        locals.var_t3__blk561 = assign18410_e25973;
        locals.var_t3__blk561_dn0 = assign18410_e25973_d_n0;
        locals.var_t3__blk561_dn2 = assign18410_e25973_d_n2;
        locals.var_t3__blk561_dn6 = assign18410_e25973_d_n6;
        locals.var_t3__blk561_dn7 = assign18410_e25973_d_n7;
        locals.var_t3__blk561_dn10 = assign18410_e25973_d_n10;
        locals.var_t3__blk561_dn11 = assign18410_e25973_d_n11;
        locals.var_t3__blk561_dn12 = assign18410_e25973_d_n12;
        locals.var_t3__blk561_dn17 = assign18410_e25973_d_n17;

    }

    pub(super) fn stamp_transient_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18420_e25981, assign18420_e25981_d_n0, assign18420_e25981_d_n2, assign18420_e25981_d_n6, assign18420_e25981_d_n7, assign18420_e25981_d_n10, assign18420_e25981_d_n11, assign18420_e25981_d_n12, assign18420_e25981_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18420_e25979: f64 = (locals.var_t2__blk567).powf(p.p245);
        (assign18420_e25979, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn0)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn0 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn2)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn2 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn6)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn6 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn7)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn7 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn10)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn10 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn11)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn11 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn12)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn12 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn17)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn17 / locals.var_t2__blk567))) },)
    } else {
        (locals.var_t0__blk560, locals.var_t0__blk560_dn0, locals.var_t0__blk560_dn2, locals.var_t0__blk560_dn6, locals.var_t0__blk560_dn7, locals.var_t0__blk560_dn10, locals.var_t0__blk560_dn11, locals.var_t0__blk560_dn12, locals.var_t0__blk560_dn17,)
    }
};
        locals.var_t0__blk560 = assign18420_e25981;
        locals.var_t0__blk560_dn0 = assign18420_e25981_d_n0;
        locals.var_t0__blk560_dn2 = assign18420_e25981_d_n2;
        locals.var_t0__blk560_dn6 = assign18420_e25981_d_n6;
        locals.var_t0__blk560_dn7 = assign18420_e25981_d_n7;
        locals.var_t0__blk560_dn10 = assign18420_e25981_d_n10;
        locals.var_t0__blk560_dn11 = assign18420_e25981_d_n11;
        locals.var_t0__blk560_dn12 = assign18420_e25981_d_n12;
        locals.var_t0__blk560_dn17 = assign18420_e25981_d_n17;

        let (assign18430_e25989, assign18430_e25989_d_n0, assign18430_e25989_d_n2, assign18430_e25989_d_n6, assign18430_e25989_d_n7, assign18430_e25989_d_n10, assign18430_e25989_d_n11, assign18430_e25989_d_n12, assign18430_e25989_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18430_e25987: f64 = (locals.var_t3__blk561 * locals.var_t0__blk560);
        (assign18430_e25987, ((locals.var_t3__blk561_dn0 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn0)), ((locals.var_t3__blk561_dn2 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn2)), ((locals.var_t3__blk561_dn6 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn6)), ((locals.var_t3__blk561_dn7 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn7)), ((locals.var_t3__blk561_dn10 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn10)), ((locals.var_t3__blk561_dn11 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn11)), ((locals.var_t3__blk561_dn12 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn12)), ((locals.var_t3__blk561_dn17 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn17)),)
    } else {
        (locals.var_t9__blk562, locals.var_t9__blk562_dn0, locals.var_t9__blk562_dn2, locals.var_t9__blk562_dn6, locals.var_t9__blk562_dn7, locals.var_t9__blk562_dn10, locals.var_t9__blk562_dn11, locals.var_t9__blk562_dn12, locals.var_t9__blk562_dn17,)
    }
};
        locals.var_t9__blk562 = assign18430_e25989;
        locals.var_t9__blk562_dn0 = assign18430_e25989_d_n0;
        locals.var_t9__blk562_dn2 = assign18430_e25989_d_n2;
        locals.var_t9__blk562_dn6 = assign18430_e25989_d_n6;
        locals.var_t9__blk562_dn7 = assign18430_e25989_d_n7;
        locals.var_t9__blk562_dn10 = assign18430_e25989_d_n10;
        locals.var_t9__blk562_dn11 = assign18430_e25989_d_n11;
        locals.var_t9__blk562_dn12 = assign18430_e25989_d_n12;
        locals.var_t9__blk562_dn17 = assign18430_e25989_d_n17;

        let (assign18440_e25999, assign18440_e25999_d_n0, assign18440_e25999_d_n2, assign18440_e25999_d_n6, assign18440_e25999_d_n7, assign18440_e25999_d_n10, assign18440_e25999_d_n11, assign18440_e25999_d_n12, assign18440_e25999_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18440_e25996: f64 = (locals.var_vdsz * p.p246);
        let assign18440_e25997: f64 = (1.0 + assign18440_e25996);
        (assign18440_e25997, (locals.var_vdsz_dn0 * p.p246), (locals.var_vdsz_dn2 * p.p246), (locals.var_vdsz_dn6 * p.p246), (locals.var_vdsz_dn7 * p.p246), (locals.var_vdsz_dn10 * p.p246), (locals.var_vdsz_dn11 * p.p246), (locals.var_vdsz_dn12 * p.p246), (locals.var_vdsz_dn17 * p.p246),)
    } else {
        (locals.var_t4__blk563, locals.var_t4__blk563_dn0, locals.var_t4__blk563_dn2, locals.var_t4__blk563_dn6, locals.var_t4__blk563_dn7, locals.var_t4__blk563_dn10, locals.var_t4__blk563_dn11, locals.var_t4__blk563_dn12, locals.var_t4__blk563_dn17,)
    }
};
        locals.var_t4__blk563 = assign18440_e25999;
        locals.var_t4__blk563_dn0 = assign18440_e25999_d_n0;
        locals.var_t4__blk563_dn2 = assign18440_e25999_d_n2;
        locals.var_t4__blk563_dn6 = assign18440_e25999_d_n6;
        locals.var_t4__blk563_dn7 = assign18440_e25999_d_n7;
        locals.var_t4__blk563_dn10 = assign18440_e25999_d_n10;
        locals.var_t4__blk563_dn11 = assign18440_e25999_d_n11;
        locals.var_t4__blk563_dn12 = assign18440_e25999_d_n12;
        locals.var_t4__blk563_dn17 = assign18440_e25999_d_n17;

        let (assign18450_e26005, assign18450_e26005_d_n0, assign18450_e26005_d_n2, assign18450_e26005_d_n6, assign18450_e26005_d_n7, assign18450_e26005_d_n10, assign18450_e26005_d_n11, assign18450_e26005_d_n12, assign18450_e26005_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk560, locals.var_t0__blk560_dn0, locals.var_t0__blk560_dn2, locals.var_t0__blk560_dn6, locals.var_t0__blk560_dn7, locals.var_t0__blk560_dn10, locals.var_t0__blk560_dn11, locals.var_t0__blk560_dn12, locals.var_t0__blk560_dn17,)
    }
};
        locals.var_t0__blk560 = assign18450_e26005;
        locals.var_t0__blk560_dn0 = assign18450_e26005_d_n0;
        locals.var_t0__blk560_dn2 = assign18450_e26005_d_n2;
        locals.var_t0__blk560_dn6 = assign18450_e26005_d_n6;
        locals.var_t0__blk560_dn7 = assign18450_e26005_d_n7;
        locals.var_t0__blk560_dn10 = assign18450_e26005_d_n10;
        locals.var_t0__blk560_dn11 = assign18450_e26005_d_n11;
        locals.var_t0__blk560_dn12 = assign18450_e26005_d_n12;
        locals.var_t0__blk560_dn17 = assign18450_e26005_d_n17;

        let assign18460_e26012: f64 = if ((locals.var_subversion < 3.0) || (p.p43 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard570 = assign18460_e26012;

        let (assign18470_e26024, assign18470_e26024_d_n0, assign18470_e26024_d_n2, assign18470_e26024_d_n6, assign18470_e26024_d_n7, assign18470_e26024_d_n10, assign18470_e26024_d_n11, assign18470_e26024_d_n12, assign18470_e26024_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign18470_e26020: f64 = (locals.var_ps0 + locals.var_t6__blk565);
        let assign18470_e26022: f64 = (assign18470_e26020 - locals.var_vbsz);
        (assign18470_e26022, ((locals.var_ps0_dn0 + locals.var_t6__blk565_dn0) - locals.var_vbsz_dn0), ((locals.var_ps0_dn2 + locals.var_t6__blk565_dn2) - locals.var_vbsz_dn2), ((locals.var_ps0_dn6 + locals.var_t6__blk565_dn6) - locals.var_vbsz_dn6), ((locals.var_ps0_dn7 + locals.var_t6__blk565_dn7) - locals.var_vbsz_dn7), ((locals.var_ps0_dn10 + locals.var_t6__blk565_dn10) - locals.var_vbsz_dn10), ((locals.var_ps0_dn11 + locals.var_t6__blk565_dn11) - locals.var_vbsz_dn11), ((locals.var_ps0_dn12 + locals.var_t6__blk565_dn12) - locals.var_vbsz_dn12), ((locals.var_ps0_dn17 + locals.var_t6__blk565_dn17) - locals.var_vbsz_dn17),)
    } else {
        (locals.var_t5__blk564, locals.var_t5__blk564_dn0, locals.var_t5__blk564_dn2, locals.var_t5__blk564_dn6, locals.var_t5__blk564_dn7, locals.var_t5__blk564_dn10, locals.var_t5__blk564_dn11, locals.var_t5__blk564_dn12, locals.var_t5__blk564_dn17,)
    }
};
        locals.var_t5__blk564 = assign18470_e26024;
        locals.var_t5__blk564_dn0 = assign18470_e26024_d_n0;
        locals.var_t5__blk564_dn2 = assign18470_e26024_d_n2;
        locals.var_t5__blk564_dn6 = assign18470_e26024_d_n6;
        locals.var_t5__blk564_dn7 = assign18470_e26024_d_n7;
        locals.var_t5__blk564_dn10 = assign18470_e26024_d_n10;
        locals.var_t5__blk564_dn11 = assign18470_e26024_d_n11;
        locals.var_t5__blk564_dn12 = assign18470_e26024_d_n12;
        locals.var_t5__blk564_dn17 = assign18470_e26024_d_n17;

        let (assign18480_e26037, assign18480_e26037_d_n0, assign18480_e26037_d_n2, assign18480_e26037_d_n6, assign18480_e26037_d_n7, assign18480_e26037_d_n10, assign18480_e26037_d_n11, assign18480_e26037_d_n12, assign18480_e26037_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign18480_e26033: f64 = (locals.var_ps0 + locals.var_t6__blk565);
        let assign18480_e26035: f64 = (assign18480_e26033 - locals.var_phi_b0_soi);
        (assign18480_e26035, ((locals.var_ps0_dn0 + locals.var_t6__blk565_dn0) - locals.var_phi_b0_soi_dn0), ((locals.var_ps0_dn2 + locals.var_t6__blk565_dn2) - locals.var_phi_b0_soi_dn2), ((locals.var_ps0_dn6 + locals.var_t6__blk565_dn6) - locals.var_phi_b0_soi_dn6), ((locals.var_ps0_dn7 + locals.var_t6__blk565_dn7) - locals.var_phi_b0_soi_dn7), ((locals.var_ps0_dn10 + locals.var_t6__blk565_dn10) - locals.var_phi_b0_soi_dn10), ((locals.var_ps0_dn11 + locals.var_t6__blk565_dn11) - locals.var_phi_b0_soi_dn11), ((locals.var_ps0_dn12 + locals.var_t6__blk565_dn12) - locals.var_phi_b0_soi_dn12), ((locals.var_ps0_dn17 + locals.var_t6__blk565_dn17) - locals.var_phi_b0_soi_dn17),)
    } else {
        (locals.var_t5__blk564, locals.var_t5__blk564_dn0, locals.var_t5__blk564_dn2, locals.var_t5__blk564_dn6, locals.var_t5__blk564_dn7, locals.var_t5__blk564_dn10, locals.var_t5__blk564_dn11, locals.var_t5__blk564_dn12, locals.var_t5__blk564_dn17,)
    }
};
        locals.var_t5__blk564 = assign18480_e26037;
        locals.var_t5__blk564_dn0 = assign18480_e26037_d_n0;
        locals.var_t5__blk564_dn2 = assign18480_e26037_d_n2;
        locals.var_t5__blk564_dn6 = assign18480_e26037_d_n6;
        locals.var_t5__blk564_dn7 = assign18480_e26037_d_n7;
        locals.var_t5__blk564_dn10 = assign18480_e26037_d_n10;
        locals.var_t5__blk564_dn11 = assign18480_e26037_d_n11;
        locals.var_t5__blk564_dn12 = assign18480_e26037_d_n12;
        locals.var_t5__blk564_dn17 = assign18480_e26037_d_n17;

        let (assign18490_e26049, assign18490_e26049_d_n0, assign18490_e26049_d_n2, assign18490_e26049_d_n6, assign18490_e26049_d_n7, assign18490_e26049_d_n10, assign18490_e26049_d_n11, assign18490_e26049_d_n12, assign18490_e26049_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18490_e26044: f64 = (locals.var_vdsz * locals.var_t0__blk560);
        let assign18490_e26046: f64 = (assign18490_e26044 * locals.var_t5__blk564);
        let assign18490_e26047: f64 = (locals.var_t4__blk563 + assign18490_e26046);
        (assign18490_e26047, (locals.var_t4__blk563_dn0 + ((((locals.var_vdsz_dn0 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn0)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn0))), (locals.var_t4__blk563_dn2 + ((((locals.var_vdsz_dn2 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn2)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn2))), (locals.var_t4__blk563_dn6 + ((((locals.var_vdsz_dn6 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn6)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn6))), (locals.var_t4__blk563_dn7 + ((((locals.var_vdsz_dn7 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn7)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn7))), (locals.var_t4__blk563_dn10 + ((((locals.var_vdsz_dn10 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn10)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn10))), (locals.var_t4__blk563_dn11 + ((((locals.var_vdsz_dn11 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn11)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn11))), (locals.var_t4__blk563_dn12 + ((((locals.var_vdsz_dn12 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn12)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn12))), (locals.var_t4__blk563_dn17 + ((((locals.var_vdsz_dn17 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn17)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn17))),)
    } else {
        (locals.var_t4__blk563, locals.var_t4__blk563_dn0, locals.var_t4__blk563_dn2, locals.var_t4__blk563_dn6, locals.var_t4__blk563_dn7, locals.var_t4__blk563_dn10, locals.var_t4__blk563_dn11, locals.var_t4__blk563_dn12, locals.var_t4__blk563_dn17,)
    }
};
        locals.var_t4__blk563 = assign18490_e26049;
        locals.var_t4__blk563_dn0 = assign18490_e26049_d_n0;
        locals.var_t4__blk563_dn2 = assign18490_e26049_d_n2;
        locals.var_t4__blk563_dn6 = assign18490_e26049_d_n6;
        locals.var_t4__blk563_dn7 = assign18490_e26049_d_n7;
        locals.var_t4__blk563_dn10 = assign18490_e26049_d_n10;
        locals.var_t4__blk563_dn11 = assign18490_e26049_d_n11;
        locals.var_t4__blk563_dn12 = assign18490_e26049_d_n12;
        locals.var_t4__blk563_dn17 = assign18490_e26049_d_n17;

        let (assign18500_e26057, assign18500_e26057_d_n0, assign18500_e26057_d_n2, assign18500_e26057_d_n6, assign18500_e26057_d_n7, assign18500_e26057_d_n10, assign18500_e26057_d_n11, assign18500_e26057_d_n12, assign18500_e26057_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18500_e26055: f64 = (locals.var_t9__blk562 * locals.var_t4__blk563);
        (assign18500_e26055, ((locals.var_t9__blk562_dn0 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn0)), ((locals.var_t9__blk562_dn2 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn2)), ((locals.var_t9__blk562_dn6 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn6)), ((locals.var_t9__blk562_dn7 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn7)), ((locals.var_t9__blk562_dn10 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn10)), ((locals.var_t9__blk562_dn11 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn11)), ((locals.var_t9__blk562_dn12 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn12)), ((locals.var_t9__blk562_dn17 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn17)),)
    } else {
        (locals.var_t6__blk565, locals.var_t6__blk565_dn0, locals.var_t6__blk565_dn2, locals.var_t6__blk565_dn6, locals.var_t6__blk565_dn7, locals.var_t6__blk565_dn10, locals.var_t6__blk565_dn11, locals.var_t6__blk565_dn12, locals.var_t6__blk565_dn17,)
    }
};
        locals.var_t6__blk565 = assign18500_e26057;
        locals.var_t6__blk565_dn0 = assign18500_e26057_d_n0;
        locals.var_t6__blk565_dn2 = assign18500_e26057_d_n2;
        locals.var_t6__blk565_dn6 = assign18500_e26057_d_n6;
        locals.var_t6__blk565_dn7 = assign18500_e26057_d_n7;
        locals.var_t6__blk565_dn10 = assign18500_e26057_d_n10;
        locals.var_t6__blk565_dn11 = assign18500_e26057_d_n11;
        locals.var_t6__blk565_dn12 = assign18500_e26057_d_n12;
        locals.var_t6__blk565_dn17 = assign18500_e26057_d_n17;

        let (assign18510_e26063, assign18510_e26063_d_n0, assign18510_e26063_d_n2, assign18510_e26063_d_n6, assign18510_e26063_d_n7, assign18510_e26063_d_n10, assign18510_e26063_d_n11, assign18510_e26063_d_n12, assign18510_e26063_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        (locals.var_t6__blk565, locals.var_t6__blk565_dn0, locals.var_t6__blk565_dn2, locals.var_t6__blk565_dn6, locals.var_t6__blk565_dn7, locals.var_t6__blk565_dn10, locals.var_t6__blk565_dn11, locals.var_t6__blk565_dn12, locals.var_t6__blk565_dn17,)
    } else {
        (locals.var_t9__blk562, locals.var_t9__blk562_dn0, locals.var_t9__blk562_dn2, locals.var_t9__blk562_dn6, locals.var_t9__blk562_dn7, locals.var_t9__blk562_dn10, locals.var_t9__blk562_dn11, locals.var_t9__blk562_dn12, locals.var_t9__blk562_dn17,)
    }
};
        locals.var_t9__blk562 = assign18510_e26063;
        locals.var_t9__blk562_dn0 = assign18510_e26063_d_n0;
        locals.var_t9__blk562_dn2 = assign18510_e26063_d_n2;
        locals.var_t9__blk562_dn6 = assign18510_e26063_d_n6;
        locals.var_t9__blk562_dn7 = assign18510_e26063_d_n7;
        locals.var_t9__blk562_dn10 = assign18510_e26063_d_n10;
        locals.var_t9__blk562_dn11 = assign18510_e26063_d_n11;
        locals.var_t9__blk562_dn12 = assign18510_e26063_d_n12;
        locals.var_t9__blk562_dn17 = assign18510_e26063_d_n17;

        let (assign18520_e26070, assign18520_e26070_d_n0, assign18520_e26070_d_n2, assign18520_e26070_d_n6, assign18520_e26070_d_n7, assign18520_e26070_d_n10, assign18520_e26070_d_n11, assign18520_e26070_d_n12, assign18520_e26070_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9__blk562, locals.var_t9__blk562_dn0, locals.var_t9__blk562_dn2, locals.var_t9__blk562_dn6, locals.var_t9__blk562_dn7, locals.var_t9__blk562_dn10, locals.var_t9__blk562_dn11, locals.var_t9__blk562_dn12, locals.var_t9__blk562_dn17,)
    }
};
        locals.var_t9__blk562 = assign18520_e26070;
        locals.var_t9__blk562_dn0 = assign18520_e26070_d_n0;
        locals.var_t9__blk562_dn2 = assign18520_e26070_d_n2;
        locals.var_t9__blk562_dn6 = assign18520_e26070_d_n6;
        locals.var_t9__blk562_dn7 = assign18520_e26070_d_n7;
        locals.var_t9__blk562_dn10 = assign18520_e26070_d_n10;
        locals.var_t9__blk562_dn11 = assign18520_e26070_d_n11;
        locals.var_t9__blk562_dn12 = assign18520_e26070_d_n12;
        locals.var_t9__blk562_dn17 = assign18520_e26070_d_n17;

        let assign18530_e26073: f64 = if p.p248 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign18530_e26073;

        let (assign18540_e26081, assign18540_e26081_d_n0, assign18540_e26081_d_n2, assign18540_e26081_d_n6, assign18540_e26081_d_n7, assign18540_e26081_d_n10, assign18540_e26081_d_n11, assign18540_e26081_d_n12, assign18540_e26081_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard571 != 0.0)) {
        let assign18540_e26079: f64 = (locals.var_beta * locals.var_gdl0);
        (assign18540_e26079, 0.0, 0.0, 0.0, 0.0, (locals.var_beta_dn10 * locals.var_gdl0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk559, locals.var_t1__blk559_dn0, locals.var_t1__blk559_dn2, locals.var_t1__blk559_dn6, locals.var_t1__blk559_dn7, locals.var_t1__blk559_dn10, locals.var_t1__blk559_dn11, locals.var_t1__blk559_dn12, locals.var_t1__blk559_dn17,)
    }
};
        locals.var_t1__blk559 = assign18540_e26081;
        locals.var_t1__blk559_dn0 = assign18540_e26081_d_n0;
        locals.var_t1__blk559_dn2 = assign18540_e26081_d_n2;
        locals.var_t1__blk559_dn6 = assign18540_e26081_d_n6;
        locals.var_t1__blk559_dn7 = assign18540_e26081_d_n7;
        locals.var_t1__blk559_dn10 = assign18540_e26081_d_n10;
        locals.var_t1__blk559_dn11 = assign18540_e26081_d_n11;
        locals.var_t1__blk559_dn12 = assign18540_e26081_d_n12;
        locals.var_t1__blk559_dn17 = assign18540_e26081_d_n17;

        let (assign18550_e26089, assign18550_e26089_d_n0, assign18550_e26089_d_n2, assign18550_e26089_d_n6, assign18550_e26089_d_n7, assign18550_e26089_d_n10, assign18550_e26089_d_n11, assign18550_e26089_d_n12, assign18550_e26089_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard571 != 0.0)) {
        let assign18550_e26087: f64 = (locals.var_c_fox * locals.var_t1__blk559);
        (assign18550_e26087, ((locals.var_c_fox_dn0 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn0)), ((locals.var_c_fox_dn2 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn2)), ((locals.var_c_fox_dn6 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn6)), ((locals.var_c_fox_dn7 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn7)), ((locals.var_c_fox_dn10 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn10)), ((locals.var_c_fox_dn11 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn11)), ((locals.var_c_fox_dn12 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn12)), ((locals.var_c_fox_dn17 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn17)),)
    } else {
        (locals.var_t2__blk567, locals.var_t2__blk567_dn0, locals.var_t2__blk567_dn2, locals.var_t2__blk567_dn6, locals.var_t2__blk567_dn7, locals.var_t2__blk567_dn10, locals.var_t2__blk567_dn11, locals.var_t2__blk567_dn12, locals.var_t2__blk567_dn17,)
    }
};
        locals.var_t2__blk567 = assign18550_e26089;
        locals.var_t2__blk567_dn0 = assign18550_e26089_d_n0;
        locals.var_t2__blk567_dn2 = assign18550_e26089_d_n2;
        locals.var_t2__blk567_dn6 = assign18550_e26089_d_n6;
        locals.var_t2__blk567_dn7 = assign18550_e26089_d_n7;
        locals.var_t2__blk567_dn10 = assign18550_e26089_d_n10;
        locals.var_t2__blk567_dn11 = assign18550_e26089_d_n11;
        locals.var_t2__blk567_dn12 = assign18550_e26089_d_n12;
        locals.var_t2__blk567_dn17 = assign18550_e26089_d_n17;

        let (assign18560_e26097, assign18560_e26097_d_n0, assign18560_e26097_d_n2, assign18560_e26097_d_n6, assign18560_e26097_d_n7, assign18560_e26097_d_n10, assign18560_e26097_d_n11, assign18560_e26097_d_n12, assign18560_e26097_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard571 != 0.0)) {
        let assign18560_e26095: f64 = (locals.var_t2__blk567 * locals.var_vdsz);
        (assign18560_e26095, ((locals.var_t2__blk567_dn0 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn0)), ((locals.var_t2__blk567_dn2 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn2)), ((locals.var_t2__blk567_dn6 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn6)), ((locals.var_t2__blk567_dn7 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn7)), ((locals.var_t2__blk567_dn10 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn10)), ((locals.var_t2__blk567_dn11 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn11)), ((locals.var_t2__blk567_dn12 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn12)), ((locals.var_t2__blk567_dn17 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn17)),)
    } else {
        (locals.var_t8__blk566, locals.var_t8__blk566_dn0, locals.var_t8__blk566_dn2, locals.var_t8__blk566_dn6, locals.var_t8__blk566_dn7, locals.var_t8__blk566_dn10, locals.var_t8__blk566_dn11, locals.var_t8__blk566_dn12, locals.var_t8__blk566_dn17,)
    }
};
        locals.var_t8__blk566 = assign18560_e26097;
        locals.var_t8__blk566_dn0 = assign18560_e26097_d_n0;
        locals.var_t8__blk566_dn2 = assign18560_e26097_d_n2;
        locals.var_t8__blk566_dn6 = assign18560_e26097_d_n6;
        locals.var_t8__blk566_dn7 = assign18560_e26097_d_n7;
        locals.var_t8__blk566_dn10 = assign18560_e26097_d_n10;
        locals.var_t8__blk566_dn11 = assign18560_e26097_d_n11;
        locals.var_t8__blk566_dn12 = assign18560_e26097_d_n12;
        locals.var_t8__blk566_dn17 = assign18560_e26097_d_n17;

        let (assign18570_e26104, assign18570_e26104_d_n0, assign18570_e26104_d_n2, assign18570_e26104_d_n6, assign18570_e26104_d_n7, assign18570_e26104_d_n10, assign18570_e26104_d_n11, assign18570_e26104_d_n12, assign18570_e26104_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard571 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8__blk566, locals.var_t8__blk566_dn0, locals.var_t8__blk566_dn2, locals.var_t8__blk566_dn6, locals.var_t8__blk566_dn7, locals.var_t8__blk566_dn10, locals.var_t8__blk566_dn11, locals.var_t8__blk566_dn12, locals.var_t8__blk566_dn17,)
    }
};
        locals.var_t8__blk566 = assign18570_e26104;
        locals.var_t8__blk566_dn0 = assign18570_e26104_d_n0;
        locals.var_t8__blk566_dn2 = assign18570_e26104_d_n2;
        locals.var_t8__blk566_dn6 = assign18570_e26104_d_n6;
        locals.var_t8__blk566_dn7 = assign18570_e26104_d_n7;
        locals.var_t8__blk566_dn10 = assign18570_e26104_d_n10;
        locals.var_t8__blk566_dn11 = assign18570_e26104_d_n11;
        locals.var_t8__blk566_dn12 = assign18570_e26104_d_n12;
        locals.var_t8__blk566_dn17 = assign18570_e26104_d_n17;

        let assign18580_e26107: f64 = (locals.var_t9__blk562 + locals.var_t8__blk566);
        let assign18580_e26109: f64 = if assign18580_e26107 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign18580_e26109;

        let (assign18590_e26119, assign18590_e26119_d_n0, assign18590_e26119_d_n2, assign18590_e26119_d_n6, assign18590_e26119_d_n7, assign18590_e26119_d_n10, assign18590_e26119_d_n11, assign18590_e26119_d_n12, assign18590_e26119_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18590_e26116: f64 = (locals.var_t9__blk562 + locals.var_t8__blk566);
        let assign18590_e26117: f64 = (locals.var_pds * assign18590_e26116);
        (assign18590_e26117, ((locals.var_pds_dn0 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn0 + locals.var_t8__blk566_dn0))), ((locals.var_pds_dn2 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn2 + locals.var_t8__blk566_dn2))), ((locals.var_pds_dn6 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn6 + locals.var_t8__blk566_dn6))), ((locals.var_pds_dn7 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn7 + locals.var_t8__blk566_dn7))), ((locals.var_pds_dn10 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn10 + locals.var_t8__blk566_dn10))), ((locals.var_pds_dn11 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn11 + locals.var_t8__blk566_dn11))), ((locals.var_pds_dn12 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn12 + locals.var_t8__blk566_dn12))), ((locals.var_pds_dn17 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn17 + locals.var_t8__blk566_dn17))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn12, locals.var_idd1_dn17,)
    }
};
        locals.var_idd1 = assign18590_e26119;
        locals.var_idd1_dn0 = assign18590_e26119_d_n0;
        locals.var_idd1_dn2 = assign18590_e26119_d_n2;
        locals.var_idd1_dn6 = assign18590_e26119_d_n6;
        locals.var_idd1_dn7 = assign18590_e26119_d_n7;
        locals.var_idd1_dn10 = assign18590_e26119_d_n10;
        locals.var_idd1_dn11 = assign18590_e26119_d_n11;
        locals.var_idd1_dn12 = assign18590_e26119_d_n12;
        locals.var_idd1_dn17 = assign18590_e26119_d_n17;

        let (assign18600_e26129, assign18600_e26129_d_n0, assign18600_e26129_d_n2, assign18600_e26129_d_n6, assign18600_e26129_d_n7, assign18600_e26129_d_n10, assign18600_e26129_d_n11, assign18600_e26129_d_n12, assign18600_e26129_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18600_e26125: f64 = (locals.var_betawl * locals.var_idd1);
        let assign18600_e26127: f64 = (assign18600_e26125 * locals.var_mu);
        (assign18600_e26127, ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn12)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn17)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn17)),)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    }
};
        locals.var_idspt = assign18600_e26129;
        locals.var_idspt_dn0 = assign18600_e26129_d_n0;
        locals.var_idspt_dn2 = assign18600_e26129_d_n2;
        locals.var_idspt_dn6 = assign18600_e26129_d_n6;
        locals.var_idspt_dn7 = assign18600_e26129_d_n7;
        locals.var_idspt_dn10 = assign18600_e26129_d_n10;
        locals.var_idspt_dn11 = assign18600_e26129_d_n11;
        locals.var_idspt_dn12 = assign18600_e26129_d_n12;
        locals.var_idspt_dn17 = assign18600_e26129_d_n17;

        let (assign18610_e26135, assign18610_e26135_d_n0, assign18610_e26135_d_n2, assign18610_e26135_d_n6, assign18610_e26135_d_n7, assign18610_e26135_d_n10, assign18610_e26135_d_n11, assign18610_e26135_d_n12, assign18610_e26135_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18610_e26133: f64 = (locals.var_ids0 + locals.var_idspt);
        (assign18610_e26133, (locals.var_ids0_dn0 + locals.var_idspt_dn0), (locals.var_ids0_dn2 + locals.var_idspt_dn2), (locals.var_ids0_dn6 + locals.var_idspt_dn6), (locals.var_ids0_dn7 + locals.var_idspt_dn7), (locals.var_ids0_dn10 + locals.var_idspt_dn10), (locals.var_ids0_dn11 + locals.var_idspt_dn11), (locals.var_ids0_dn12 + locals.var_idspt_dn12), (locals.var_ids0_dn17 + locals.var_idspt_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign18610_e26135;
        locals.var_ids_dn0 = assign18610_e26135_d_n0;
        locals.var_ids_dn2 = assign18610_e26135_d_n2;
        locals.var_ids_dn6 = assign18610_e26135_d_n6;
        locals.var_ids_dn7 = assign18610_e26135_d_n7;
        locals.var_ids_dn10 = assign18610_e26135_d_n10;
        locals.var_ids_dn11 = assign18610_e26135_d_n11;
        locals.var_ids_dn12 = assign18610_e26135_d_n12;
        locals.var_ids_dn17 = assign18610_e26135_d_n17;

        let (assign18620_e26139, assign18620_e26139_d_n0, assign18620_e26139_d_n2, assign18620_e26139_d_n6, assign18620_e26139_d_n7, assign18620_e26139_d_n10, assign18620_e26139_d_n11, assign18620_e26139_d_n12, assign18620_e26139_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn12, locals.var_idspt0_dn17,)
    }
};
        locals.var_idspt0 = assign18620_e26139;
        locals.var_idspt0_dn0 = assign18620_e26139_d_n0;
        locals.var_idspt0_dn2 = assign18620_e26139_d_n2;
        locals.var_idspt0_dn6 = assign18620_e26139_d_n6;
        locals.var_idspt0_dn7 = assign18620_e26139_d_n7;
        locals.var_idspt0_dn10 = assign18620_e26139_d_n10;
        locals.var_idspt0_dn11 = assign18620_e26139_d_n11;
        locals.var_idspt0_dn12 = assign18620_e26139_d_n12;
        locals.var_idspt0_dn17 = assign18620_e26139_d_n17;

        let assign18630_e26142: f64 = if p.p33 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign18630_e26142;

        let (assign18640_e26148, assign18640_e26148_d_n0, assign18640_e26148_d_n2, assign18640_e26148_d_n6, assign18640_e26148_d_n7, assign18640_e26148_d_n10, assign18640_e26148_d_n11, assign18640_e26148_d_n12, assign18640_e26148_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn12, locals.var_wdpl_dn17,)
    } else {
        (locals.var_t2__blk575, locals.var_t2__blk575_dn0, locals.var_t2__blk575_dn2, locals.var_t2__blk575_dn6, locals.var_t2__blk575_dn7, locals.var_t2__blk575_dn10, locals.var_t2__blk575_dn11, locals.var_t2__blk575_dn12, locals.var_t2__blk575_dn17,)
    }
};
        locals.var_t2__blk575 = assign18640_e26148;
        locals.var_t2__blk575_dn0 = assign18640_e26148_d_n0;
        locals.var_t2__blk575_dn2 = assign18640_e26148_d_n2;
        locals.var_t2__blk575_dn6 = assign18640_e26148_d_n6;
        locals.var_t2__blk575_dn7 = assign18640_e26148_d_n7;
        locals.var_t2__blk575_dn10 = assign18640_e26148_d_n10;
        locals.var_t2__blk575_dn11 = assign18640_e26148_d_n11;
        locals.var_t2__blk575_dn12 = assign18640_e26148_d_n12;
        locals.var_t2__blk575_dn17 = assign18640_e26148_d_n17;

        let (assign18650_e26156, assign18650_e26156_d_n0, assign18650_e26156_d_n2, assign18650_e26156_d_n6, assign18650_e26156_d_n7, assign18650_e26156_d_n10, assign18650_e26156_d_n11, assign18650_e26156_d_n12, assign18650_e26156_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18650_e26154: f64 = (locals.var_lgatesm - p.p71);
        (assign18650_e26154, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk576, locals.var_t3__blk576_dn0, locals.var_t3__blk576_dn2, locals.var_t3__blk576_dn6, locals.var_t3__blk576_dn7, locals.var_t3__blk576_dn10, locals.var_t3__blk576_dn11, locals.var_t3__blk576_dn12, locals.var_t3__blk576_dn17,)
    }
};
        locals.var_t3__blk576 = assign18650_e26156;
        locals.var_t3__blk576_dn0 = assign18650_e26156_d_n0;
        locals.var_t3__blk576_dn2 = assign18650_e26156_d_n2;
        locals.var_t3__blk576_dn6 = assign18650_e26156_d_n6;
        locals.var_t3__blk576_dn7 = assign18650_e26156_d_n7;
        locals.var_t3__blk576_dn10 = assign18650_e26156_d_n10;
        locals.var_t3__blk576_dn11 = assign18650_e26156_d_n11;
        locals.var_t3__blk576_dn12 = assign18650_e26156_d_n12;
        locals.var_t3__blk576_dn17 = assign18650_e26156_d_n17;

        let (assign18660_e26166, assign18660_e26166_d_n0, assign18660_e26166_d_n2, assign18660_e26166_d_n6, assign18660_e26166_d_n7, assign18660_e26166_d_n10, assign18660_e26166_d_n11, assign18660_e26166_d_n12, assign18660_e26166_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18660_e26163: f64 = (locals.var_t3__blk576 * locals.var_t3__blk576);
        let assign18660_e26164: f64 = (1.0 / assign18660_e26163);
        (assign18660_e26164, (-(((locals.var_t3__blk576_dn0 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn0)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn2 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn2)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn6 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn6)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn7 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn7)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn10 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn10)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn11 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn11)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn12 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn12)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn17 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn17)) / (assign18660_e26163 * assign18660_e26163))),)
    } else {
        (locals.var_t4__blk577, locals.var_t4__blk577_dn0, locals.var_t4__blk577_dn2, locals.var_t4__blk577_dn6, locals.var_t4__blk577_dn7, locals.var_t4__blk577_dn10, locals.var_t4__blk577_dn11, locals.var_t4__blk577_dn12, locals.var_t4__blk577_dn17,)
    }
};
        locals.var_t4__blk577 = assign18660_e26166;
        locals.var_t4__blk577_dn0 = assign18660_e26166_d_n0;
        locals.var_t4__blk577_dn2 = assign18660_e26166_d_n2;
        locals.var_t4__blk577_dn6 = assign18660_e26166_d_n6;
        locals.var_t4__blk577_dn7 = assign18660_e26166_d_n7;
        locals.var_t4__blk577_dn10 = assign18660_e26166_d_n10;
        locals.var_t4__blk577_dn11 = assign18660_e26166_d_n11;
        locals.var_t4__blk577_dn12 = assign18660_e26166_d_n12;
        locals.var_t4__blk577_dn17 = assign18660_e26166_d_n17;

        let (assign18670_e26184, assign18670_e26184_d_n0, assign18670_e26184_d_n2, assign18670_e26184_d_n6, assign18670_e26184_d_n7, assign18670_e26184_d_n10, assign18670_e26184_d_n11, assign18670_e26184_d_n12, assign18670_e26184_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18670_e26173: f64 = (p.p69 - locals.var_pb20b);
        let assign18670_e26174: f64 = (2.0 * assign18670_e26173);
        let assign18670_e26177: f64 = (1.034943e-10 * locals.var_c_fox_inv);
        let assign18670_e26178: f64 = (assign18670_e26174 * assign18670_e26177);
        let assign18670_e26180: f64 = (assign18670_e26178 * locals.var_t2__blk575);
        let assign18670_e26182: f64 = (assign18670_e26180 * locals.var_t4__blk577);
        (assign18670_e26182, (((((((2.0 * (-locals.var_pb20b_dn0)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn0))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn0)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn2))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn2)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn2)), (((((((2.0 * (-locals.var_pb20b_dn6)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn6))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn6)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn7))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn7)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn7)), (((((((2.0 * (-locals.var_pb20b_dn10)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn10))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn10)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn11))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn11)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn11)), (((((((2.0 * (-locals.var_pb20b_dn12)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn12))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn12)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn12)), (((((((2.0 * (-locals.var_pb20b_dn17)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn17))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn17)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn17)),)
    } else {
        (locals.var_t5__blk578, locals.var_t5__blk578_dn0, locals.var_t5__blk578_dn2, locals.var_t5__blk578_dn6, locals.var_t5__blk578_dn7, locals.var_t5__blk578_dn10, locals.var_t5__blk578_dn11, locals.var_t5__blk578_dn12, locals.var_t5__blk578_dn17,)
    }
};
        locals.var_t5__blk578 = assign18670_e26184;
        locals.var_t5__blk578_dn0 = assign18670_e26184_d_n0;
        locals.var_t5__blk578_dn2 = assign18670_e26184_d_n2;
        locals.var_t5__blk578_dn6 = assign18670_e26184_d_n6;
        locals.var_t5__blk578_dn7 = assign18670_e26184_d_n7;
        locals.var_t5__blk578_dn10 = assign18670_e26184_d_n10;
        locals.var_t5__blk578_dn11 = assign18670_e26184_d_n11;
        locals.var_t5__blk578_dn12 = assign18670_e26184_d_n12;
        locals.var_t5__blk578_dn17 = assign18670_e26184_d_n17;

        let (assign18680_e26192, assign18680_e26192_d_n0, assign18680_e26192_d_n2, assign18680_e26192_d_n6, assign18680_e26192_d_n7, assign18680_e26192_d_n10, assign18680_e26192_d_n11, assign18680_e26192_d_n12, assign18680_e26192_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18680_e26190: f64 = (locals.var_t5__blk578 * locals.var_sqrt_pbsum);
        (assign18680_e26190, ((locals.var_t5__blk578_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5__blk578_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5__blk578_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5__blk578_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5__blk578_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5__blk578_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5__blk578_dn12 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn12)), ((locals.var_t5__blk578_dn17 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn17)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn12, locals.var_dvth0_dn17,)
    }
};
        locals.var_dvth0 = assign18680_e26192;
        locals.var_dvth0_dn0 = assign18680_e26192_d_n0;
        locals.var_dvth0_dn2 = assign18680_e26192_d_n2;
        locals.var_dvth0_dn6 = assign18680_e26192_d_n6;
        locals.var_dvth0_dn7 = assign18680_e26192_d_n7;
        locals.var_dvth0_dn10 = assign18680_e26192_d_n10;
        locals.var_dvth0_dn11 = assign18680_e26192_d_n11;
        locals.var_dvth0_dn12 = assign18680_e26192_d_n12;
        locals.var_dvth0_dn17 = assign18680_e26192_d_n17;

        let (assign18690_e26202, assign18690_e26202_d_n0, assign18690_e26202_d_n2, assign18690_e26202_d_n6, assign18690_e26202_d_n7, assign18690_e26202_d_n10, assign18690_e26202_d_n11, assign18690_e26202_d_n12, assign18690_e26202_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18690_e26199: f64 = (p.p155 * locals.var_vdsz);
        let assign18690_e26200: f64 = (p.p154 + assign18690_e26199);
        (assign18690_e26200, (p.p155 * locals.var_vdsz_dn0), (p.p155 * locals.var_vdsz_dn2), (p.p155 * locals.var_vdsz_dn6), (p.p155 * locals.var_vdsz_dn7), (p.p155 * locals.var_vdsz_dn10), (p.p155 * locals.var_vdsz_dn11), (p.p155 * locals.var_vdsz_dn12), (p.p155 * locals.var_vdsz_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18690_e26202;
        locals.var_t1w_dn0 = assign18690_e26202_d_n0;
        locals.var_t1w_dn2 = assign18690_e26202_d_n2;
        locals.var_t1w_dn6 = assign18690_e26202_d_n6;
        locals.var_t1w_dn7 = assign18690_e26202_d_n7;
        locals.var_t1w_dn10 = assign18690_e26202_d_n10;
        locals.var_t1w_dn11 = assign18690_e26202_d_n11;
        locals.var_t1w_dn12 = assign18690_e26202_d_n12;
        locals.var_t1w_dn17 = assign18690_e26202_d_n17;

        let (assign18700_e26210, assign18700_e26210_d_n0, assign18700_e26210_d_n2, assign18700_e26210_d_n6, assign18700_e26210_d_n7, assign18700_e26210_d_n10, assign18700_e26210_d_n11, assign18700_e26210_d_n12, assign18700_e26210_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18700_e26208: f64 = (locals.var_dvth0 * locals.var_t1w);
        (assign18700_e26208, ((locals.var_dvth0_dn0 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn2)), ((locals.var_dvth0_dn6 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn6)), ((locals.var_dvth0_dn7 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn7)), ((locals.var_dvth0_dn10 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn10)), ((locals.var_dvth0_dn11 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn11)), ((locals.var_dvth0_dn12 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn12)), ((locals.var_dvth0_dn17 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn17)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn7, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn11, locals.var_dvthscsti_dn12, locals.var_dvthscsti_dn17,)
    }
};
        locals.var_dvthscsti = assign18700_e26210;
        locals.var_dvthscsti_dn0 = assign18700_e26210_d_n0;
        locals.var_dvthscsti_dn2 = assign18700_e26210_d_n2;
        locals.var_dvthscsti_dn6 = assign18700_e26210_d_n6;
        locals.var_dvthscsti_dn7 = assign18700_e26210_d_n7;
        locals.var_dvthscsti_dn10 = assign18700_e26210_d_n10;
        locals.var_dvthscsti_dn11 = assign18700_e26210_d_n11;
        locals.var_dvthscsti_dn12 = assign18700_e26210_d_n12;
        locals.var_dvthscsti_dn17 = assign18700_e26210_d_n17;

        let (assign18710_e26220, assign18710_e26220_d_n0, assign18710_e26220_d_n2, assign18710_e26220_d_n6, assign18710_e26220_d_n7, assign18710_e26220_d_n10, assign18710_e26220_d_n11, assign18710_e26220_d_n12, assign18710_e26220_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18710_e26217: f64 = (p.p157 * locals.var_vds);
        let assign18710_e26218: f64 = (p.p156 - assign18710_e26217);
        (assign18710_e26218, (-(p.p157 * locals.var_vds_dn0)), (-(p.p157 * locals.var_vds_dn2)), (-(p.p157 * locals.var_vds_dn6)), (-(p.p157 * locals.var_vds_dn7)), (-(p.p157 * locals.var_vds_dn10)), (-(p.p157 * locals.var_vds_dn11)), (-(p.p157 * locals.var_vds_dn12)), (-(p.p157 * locals.var_vds_dn17)),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18710_e26220;
        locals.var_t1__blk573_dn0 = assign18710_e26220_d_n0;
        locals.var_t1__blk573_dn2 = assign18710_e26220_d_n2;
        locals.var_t1__blk573_dn6 = assign18710_e26220_d_n6;
        locals.var_t1__blk573_dn7 = assign18710_e26220_d_n7;
        locals.var_t1__blk573_dn10 = assign18710_e26220_d_n10;
        locals.var_t1__blk573_dn11 = assign18710_e26220_d_n11;
        locals.var_t1__blk573_dn12 = assign18710_e26220_d_n12;
        locals.var_t1__blk573_dn17 = assign18710_e26220_d_n17;

        let (assign18720_e26232, assign18720_e26232_d_n0, assign18720_e26232_d_n2, assign18720_e26232_d_n6, assign18720_e26232_d_n7, assign18720_e26232_d_n10, assign18720_e26232_d_n11, assign18720_e26232_d_n12, assign18720_e26232_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18720_e26226: f64 = (locals.var_vgsz - locals.var_vfb);
        let assign18720_e26228: f64 = (assign18720_e26226 + locals.var_t1__blk573);
        let assign18720_e26230: f64 = (assign18720_e26228 + locals.var_dvthscsti);
        (assign18720_e26230, ((locals.var_vgsz_dn0 + locals.var_t1__blk573_dn0) + locals.var_dvthscsti_dn0), ((locals.var_vgsz_dn2 + locals.var_t1__blk573_dn2) + locals.var_dvthscsti_dn2), ((locals.var_vgsz_dn6 + locals.var_t1__blk573_dn6) + locals.var_dvthscsti_dn6), ((locals.var_vgsz_dn7 + locals.var_t1__blk573_dn7) + locals.var_dvthscsti_dn7), ((locals.var_vgsz_dn10 + locals.var_t1__blk573_dn10) + locals.var_dvthscsti_dn10), ((locals.var_vgsz_dn11 + locals.var_t1__blk573_dn11) + locals.var_dvthscsti_dn11), ((locals.var_vgsz_dn12 + locals.var_t1__blk573_dn12) + locals.var_dvthscsti_dn12), ((locals.var_vgsz_dn17 + locals.var_t1__blk573_dn17) + locals.var_dvthscsti_dn17),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn12, locals.var_vgssti_dn17,)
    }
};
        locals.var_vgssti = assign18720_e26232;
        locals.var_vgssti_dn0 = assign18720_e26232_d_n0;
        locals.var_vgssti_dn2 = assign18720_e26232_d_n2;
        locals.var_vgssti_dn6 = assign18720_e26232_d_n6;
        locals.var_vgssti_dn7 = assign18720_e26232_d_n7;
        locals.var_vgssti_dn10 = assign18720_e26232_d_n10;
        locals.var_vgssti_dn11 = assign18720_e26232_d_n11;
        locals.var_vgssti_dn12 = assign18720_e26232_d_n12;
        locals.var_vgssti_dn17 = assign18720_e26232_d_n17;

    }

    pub(super) fn stamp_transient_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18730_e26242, assign18730_e26242_d_n0, assign18730_e26242_d_n2, assign18730_e26242_d_n6, assign18730_e26242_d_n7, assign18730_e26242_d_n10, assign18730_e26242_d_n11, assign18730_e26242_d_n12, assign18730_e26242_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18730_e26238: f64 = (locals.var_costi0_p2 * locals.var_c_fox_inv);
        let assign18730_e26240: f64 = (assign18730_e26238 * locals.var_c_fox_inv);
        (assign18730_e26240, ((((locals.var_costi0_p2_dn0 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn0)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn2)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn2)), ((((locals.var_costi0_p2_dn6 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn6)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn6)), ((((locals.var_costi0_p2_dn7 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn7)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn7)), ((((locals.var_costi0_p2_dn10 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn10)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn10)), ((((locals.var_costi0_p2_dn11 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn11)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn11)), ((((locals.var_costi0_p2_dn12 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn12)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn12)), ((((locals.var_costi0_p2_dn17 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn17)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn12, locals.var_costi3_dn17,)
    }
};
        locals.var_costi3 = assign18730_e26242;
        locals.var_costi3_dn0 = assign18730_e26242_d_n0;
        locals.var_costi3_dn2 = assign18730_e26242_d_n2;
        locals.var_costi3_dn6 = assign18730_e26242_d_n6;
        locals.var_costi3_dn7 = assign18730_e26242_d_n7;
        locals.var_costi3_dn10 = assign18730_e26242_d_n10;
        locals.var_costi3_dn11 = assign18730_e26242_d_n11;
        locals.var_costi3_dn12 = assign18730_e26242_d_n12;
        locals.var_costi3_dn17 = assign18730_e26242_d_n17;

        let (assign18740_e26252, assign18740_e26252_d_n0, assign18740_e26252_d_n2, assign18740_e26252_d_n6, assign18740_e26252_d_n7, assign18740_e26252_d_n10, assign18740_e26252_d_n11, assign18740_e26252_d_n12, assign18740_e26252_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18740_e26248: f64 = (locals.var_costi3 * locals.var_beta);
        let assign18740_e26250: f64 = (assign18740_e26248 * 0.5);
        (assign18740_e26250, ((locals.var_costi3_dn0 * locals.var_beta) * 0.5), ((locals.var_costi3_dn2 * locals.var_beta) * 0.5), ((locals.var_costi3_dn6 * locals.var_beta) * 0.5), ((locals.var_costi3_dn7 * locals.var_beta) * 0.5), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) * 0.5), ((locals.var_costi3_dn11 * locals.var_beta) * 0.5), ((locals.var_costi3_dn12 * locals.var_beta) * 0.5), ((locals.var_costi3_dn17 * locals.var_beta) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn12, locals.var_costi4_dn17,)
    }
};
        locals.var_costi4 = assign18740_e26252;
        locals.var_costi4_dn0 = assign18740_e26252_d_n0;
        locals.var_costi4_dn2 = assign18740_e26252_d_n2;
        locals.var_costi4_dn6 = assign18740_e26252_d_n6;
        locals.var_costi4_dn7 = assign18740_e26252_d_n7;
        locals.var_costi4_dn10 = assign18740_e26252_d_n10;
        locals.var_costi4_dn11 = assign18740_e26252_d_n11;
        locals.var_costi4_dn12 = assign18740_e26252_d_n12;
        locals.var_costi4_dn17 = assign18740_e26252_d_n17;

        let (assign18750_e26262, assign18750_e26262_d_n0, assign18750_e26262_d_n2, assign18750_e26262_d_n6, assign18750_e26262_d_n7, assign18750_e26262_d_n10, assign18750_e26262_d_n11, assign18750_e26262_d_n12, assign18750_e26262_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18750_e26258: f64 = (locals.var_costi4 * locals.var_beta);
        let assign18750_e26260: f64 = (assign18750_e26258 * 2.0);
        (assign18750_e26260, ((locals.var_costi4_dn0 * locals.var_beta) * 2.0), ((locals.var_costi4_dn2 * locals.var_beta) * 2.0), ((locals.var_costi4_dn6 * locals.var_beta) * 2.0), ((locals.var_costi4_dn7 * locals.var_beta) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), ((locals.var_costi4_dn11 * locals.var_beta) * 2.0), ((locals.var_costi4_dn12 * locals.var_beta) * 2.0), ((locals.var_costi4_dn17 * locals.var_beta) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn12, locals.var_costi5_dn17,)
    }
};
        locals.var_costi5 = assign18750_e26262;
        locals.var_costi5_dn0 = assign18750_e26262_d_n0;
        locals.var_costi5_dn2 = assign18750_e26262_d_n2;
        locals.var_costi5_dn6 = assign18750_e26262_d_n6;
        locals.var_costi5_dn7 = assign18750_e26262_d_n7;
        locals.var_costi5_dn10 = assign18750_e26262_d_n10;
        locals.var_costi5_dn11 = assign18750_e26262_d_n11;
        locals.var_costi5_dn12 = assign18750_e26262_d_n12;
        locals.var_costi5_dn17 = assign18750_e26262_d_n17;

        let (assign18760_e26282, assign18760_e26282_d_n0, assign18760_e26282_d_n2, assign18760_e26282_d_n6, assign18760_e26282_d_n7, assign18760_e26282_d_n10, assign18760_e26282_d_n11, assign18760_e26282_d_n12, assign18760_e26282_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18760_e26270: f64 = (locals.var_beta * 0.25);
        let assign18760_e26271: f64 = (locals.var_costi3 * assign18760_e26270);
        let assign18760_e26272: f64 = (locals.var_beta_inv - assign18760_e26271);
        let assign18760_e26274: f64 = (assign18760_e26272 + locals.var_vfb);
        let assign18760_e26276: f64 = (assign18760_e26274 - p.p156);
        let assign18760_e26278: f64 = (assign18760_e26276 - locals.var_dvthscsti);
        let assign18760_e26280: f64 = (assign18760_e26278 + 1e-50);
        (assign18760_e26280, ((-(locals.var_costi3_dn0 * assign18760_e26270)) - locals.var_dvthscsti_dn0), ((-(locals.var_costi3_dn2 * assign18760_e26270)) - locals.var_dvthscsti_dn2), ((-(locals.var_costi3_dn6 * assign18760_e26270)) - locals.var_dvthscsti_dn6), ((-(locals.var_costi3_dn7 * assign18760_e26270)) - locals.var_dvthscsti_dn7), ((locals.var_beta_inv_dn10 - ((locals.var_costi3_dn10 * assign18760_e26270) + (locals.var_costi3 * (locals.var_beta_dn10 * 0.25)))) - locals.var_dvthscsti_dn10), ((-(locals.var_costi3_dn11 * assign18760_e26270)) - locals.var_dvthscsti_dn11), ((-(locals.var_costi3_dn12 * assign18760_e26270)) - locals.var_dvthscsti_dn12), ((-(locals.var_costi3_dn17 * assign18760_e26270)) - locals.var_dvthscsti_dn17),)
    } else {
        (locals.var_t10__blk579, locals.var_t10__blk579_dn0, locals.var_t10__blk579_dn2, locals.var_t10__blk579_dn6, locals.var_t10__blk579_dn7, locals.var_t10__blk579_dn10, locals.var_t10__blk579_dn11, locals.var_t10__blk579_dn12, locals.var_t10__blk579_dn17,)
    }
};
        locals.var_t10__blk579 = assign18760_e26282;
        locals.var_t10__blk579_dn0 = assign18760_e26282_d_n0;
        locals.var_t10__blk579_dn2 = assign18760_e26282_d_n2;
        locals.var_t10__blk579_dn6 = assign18760_e26282_d_n6;
        locals.var_t10__blk579_dn7 = assign18760_e26282_d_n7;
        locals.var_t10__blk579_dn10 = assign18760_e26282_d_n10;
        locals.var_t10__blk579_dn11 = assign18760_e26282_d_n11;
        locals.var_t10__blk579_dn12 = assign18760_e26282_d_n12;
        locals.var_t10__blk579_dn17 = assign18760_e26282_d_n17;

        let (assign18770_e26292, assign18770_e26292_d_n0, assign18770_e26292_d_n2, assign18770_e26292_d_n6, assign18770_e26292_d_n7, assign18770_e26292_d_n10, assign18770_e26292_d_n11, assign18770_e26292_d_n12, assign18770_e26292_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18770_e26288: f64 = (locals.var_vgsz - locals.var_t10__blk579);
        let assign18770_e26290: f64 = (assign18770_e26288 - 0.005);
        (assign18770_e26290, (locals.var_vgsz_dn0 - locals.var_t10__blk579_dn0), (locals.var_vgsz_dn2 - locals.var_t10__blk579_dn2), (locals.var_vgsz_dn6 - locals.var_t10__blk579_dn6), (locals.var_vgsz_dn7 - locals.var_t10__blk579_dn7), (locals.var_vgsz_dn10 - locals.var_t10__blk579_dn10), (locals.var_vgsz_dn11 - locals.var_t10__blk579_dn11), (locals.var_vgsz_dn12 - locals.var_t10__blk579_dn12), (locals.var_vgsz_dn17 - locals.var_t10__blk579_dn17),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18770_e26292;
        locals.var_t1__blk573_dn0 = assign18770_e26292_d_n0;
        locals.var_t1__blk573_dn2 = assign18770_e26292_d_n2;
        locals.var_t1__blk573_dn6 = assign18770_e26292_d_n6;
        locals.var_t1__blk573_dn7 = assign18770_e26292_d_n7;
        locals.var_t1__blk573_dn10 = assign18770_e26292_d_n10;
        locals.var_t1__blk573_dn11 = assign18770_e26292_d_n11;
        locals.var_t1__blk573_dn12 = assign18770_e26292_d_n12;
        locals.var_t1__blk573_dn17 = assign18770_e26292_d_n17;

        let (assign18780_e26304, assign18780_e26304_d_n0, assign18780_e26304_d_n2, assign18780_e26304_d_n6, assign18780_e26304_d_n7, assign18780_e26304_d_n10, assign18780_e26304_d_n11, assign18780_e26304_d_n12, assign18780_e26304_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let (assign18780_e26302,) = {
            if (locals.var_t10__blk579 >= 0.0) {
                (1.0,)
            } else {
                let assign18780_e26301: f64 = (-1.0);
                (assign18780_e26301,)
            }
        };
        (assign18780_e26302, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18780_e26304;
        locals.var_t0_dn0 = assign18780_e26304_d_n0;
        locals.var_t0_dn2 = assign18780_e26304_d_n2;
        locals.var_t0_dn6 = assign18780_e26304_d_n6;
        locals.var_t0_dn7 = assign18780_e26304_d_n7;
        locals.var_t0_dn10 = assign18780_e26304_d_n10;
        locals.var_t0_dn11 = assign18780_e26304_d_n11;
        locals.var_t0_dn12 = assign18780_e26304_d_n12;
        locals.var_t0_dn17 = assign18780_e26304_d_n17;

        let (assign18790_e26321, assign18790_e26321_d_n0, assign18790_e26321_d_n2, assign18790_e26321_d_n6, assign18790_e26321_d_n7, assign18790_e26321_d_n10, assign18790_e26321_d_n11, assign18790_e26321_d_n12, assign18790_e26321_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18790_e26310: f64 = (locals.var_t1__blk573 * locals.var_t1__blk573);
        let assign18790_e26313: f64 = (locals.var_t0 * 4.0);
        let assign18790_e26315: f64 = (assign18790_e26313 * locals.var_t10__blk579);
        let assign18790_e26317: f64 = (assign18790_e26315 * 0.005);
        let assign18790_e26318: f64 = (assign18790_e26310 + assign18790_e26317);
        let assign18790_e26319: f64 = (assign18790_e26318).sqrt();
        (assign18790_e26319, ((((locals.var_t1__blk573_dn0 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn0)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn2 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn2)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn6 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn6)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn7 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn7)) + ((((locals.var_t0_dn7 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn7)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn10 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn10)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn11 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn11)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn12 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn12)) + ((((locals.var_t0_dn12 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn12)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn17 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn17)) + ((((locals.var_t0_dn17 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn17)) * 0.005)) / (2.0 * assign18790_e26319)),)
    } else {
        (locals.var_t2__blk575, locals.var_t2__blk575_dn0, locals.var_t2__blk575_dn2, locals.var_t2__blk575_dn6, locals.var_t2__blk575_dn7, locals.var_t2__blk575_dn10, locals.var_t2__blk575_dn11, locals.var_t2__blk575_dn12, locals.var_t2__blk575_dn17,)
    }
};
        locals.var_t2__blk575 = assign18790_e26321;
        locals.var_t2__blk575_dn0 = assign18790_e26321_d_n0;
        locals.var_t2__blk575_dn2 = assign18790_e26321_d_n2;
        locals.var_t2__blk575_dn6 = assign18790_e26321_d_n6;
        locals.var_t2__blk575_dn7 = assign18790_e26321_d_n7;
        locals.var_t2__blk575_dn10 = assign18790_e26321_d_n10;
        locals.var_t2__blk575_dn11 = assign18790_e26321_d_n11;
        locals.var_t2__blk575_dn12 = assign18790_e26321_d_n12;
        locals.var_t2__blk575_dn17 = assign18790_e26321_d_n17;

        let (assign18800_e26341, assign18800_e26341_d_n0, assign18800_e26341_d_n2, assign18800_e26341_d_n6, assign18800_e26341_d_n7, assign18800_e26341_d_n10, assign18800_e26341_d_n11, assign18800_e26341_d_n12, assign18800_e26341_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18800_e26329: f64 = (locals.var_t1__blk573 + locals.var_t2__blk575);
        let assign18800_e26330: f64 = (0.5 * assign18800_e26329);
        let assign18800_e26331: f64 = (locals.var_t10__blk579 + assign18800_e26330);
        let assign18800_e26333: f64 = (assign18800_e26331 - locals.var_vfb);
        let assign18800_e26335: f64 = (assign18800_e26333 + p.p156);
        let assign18800_e26337: f64 = (assign18800_e26335 + locals.var_dvthscsti);
        let assign18800_e26339: f64 = (assign18800_e26337 - locals.var_vbspz);
        (assign18800_e26339, (((locals.var_t10__blk579_dn0 + (0.5 * (locals.var_t1__blk573_dn0 + locals.var_t2__blk575_dn0))) + locals.var_dvthscsti_dn0) - locals.var_vbspz_dn0), (((locals.var_t10__blk579_dn2 + (0.5 * (locals.var_t1__blk573_dn2 + locals.var_t2__blk575_dn2))) + locals.var_dvthscsti_dn2) - locals.var_vbspz_dn2), (((locals.var_t10__blk579_dn6 + (0.5 * (locals.var_t1__blk573_dn6 + locals.var_t2__blk575_dn6))) + locals.var_dvthscsti_dn6) - locals.var_vbspz_dn6), (((locals.var_t10__blk579_dn7 + (0.5 * (locals.var_t1__blk573_dn7 + locals.var_t2__blk575_dn7))) + locals.var_dvthscsti_dn7) - locals.var_vbspz_dn7), (((locals.var_t10__blk579_dn10 + (0.5 * (locals.var_t1__blk573_dn10 + locals.var_t2__blk575_dn10))) + locals.var_dvthscsti_dn10) - locals.var_vbspz_dn10), (((locals.var_t10__blk579_dn11 + (0.5 * (locals.var_t1__blk573_dn11 + locals.var_t2__blk575_dn11))) + locals.var_dvthscsti_dn11) - locals.var_vbspz_dn11), (((locals.var_t10__blk579_dn12 + (0.5 * (locals.var_t1__blk573_dn12 + locals.var_t2__blk575_dn12))) + locals.var_dvthscsti_dn12) - locals.var_vbspz_dn12), (((locals.var_t10__blk579_dn17 + (0.5 * (locals.var_t1__blk573_dn17 + locals.var_t2__blk575_dn17))) + locals.var_dvthscsti_dn17) - locals.var_vbspz_dn17),)
    } else {
        (locals.var_t3__blk576, locals.var_t3__blk576_dn0, locals.var_t3__blk576_dn2, locals.var_t3__blk576_dn6, locals.var_t3__blk576_dn7, locals.var_t3__blk576_dn10, locals.var_t3__blk576_dn11, locals.var_t3__blk576_dn12, locals.var_t3__blk576_dn17,)
    }
};
        locals.var_t3__blk576 = assign18800_e26341;
        locals.var_t3__blk576_dn0 = assign18800_e26341_d_n0;
        locals.var_t3__blk576_dn2 = assign18800_e26341_d_n2;
        locals.var_t3__blk576_dn6 = assign18800_e26341_d_n6;
        locals.var_t3__blk576_dn7 = assign18800_e26341_d_n7;
        locals.var_t3__blk576_dn10 = assign18800_e26341_d_n10;
        locals.var_t3__blk576_dn11 = assign18800_e26341_d_n11;
        locals.var_t3__blk576_dn12 = assign18800_e26341_d_n12;
        locals.var_t3__blk576_dn17 = assign18800_e26341_d_n17;

        let (assign18810_e26351, assign18810_e26351_d_n0, assign18810_e26351_d_n2, assign18810_e26351_d_n6, assign18810_e26351_d_n7, assign18810_e26351_d_n10, assign18810_e26351_d_n11, assign18810_e26351_d_n12, assign18810_e26351_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18810_e26347: f64 = (locals.var_beta * locals.var_t3__blk576);
        let assign18810_e26349: f64 = (assign18810_e26347 - 1.0);
        (assign18810_e26349, (locals.var_beta * locals.var_t3__blk576_dn0), (locals.var_beta * locals.var_t3__blk576_dn2), (locals.var_beta * locals.var_t3__blk576_dn6), (locals.var_beta * locals.var_t3__blk576_dn7), ((locals.var_beta_dn10 * locals.var_t3__blk576) + (locals.var_beta * locals.var_t3__blk576_dn10)), (locals.var_beta * locals.var_t3__blk576_dn11), (locals.var_beta * locals.var_t3__blk576_dn12), (locals.var_beta * locals.var_t3__blk576_dn17),)
    } else {
        (locals.var_t4__blk577, locals.var_t4__blk577_dn0, locals.var_t4__blk577_dn2, locals.var_t4__blk577_dn6, locals.var_t4__blk577_dn7, locals.var_t4__blk577_dn10, locals.var_t4__blk577_dn11, locals.var_t4__blk577_dn12, locals.var_t4__blk577_dn17,)
    }
};
        locals.var_t4__blk577 = assign18810_e26351;
        locals.var_t4__blk577_dn0 = assign18810_e26351_d_n0;
        locals.var_t4__blk577_dn2 = assign18810_e26351_d_n2;
        locals.var_t4__blk577_dn6 = assign18810_e26351_d_n6;
        locals.var_t4__blk577_dn7 = assign18810_e26351_d_n7;
        locals.var_t4__blk577_dn10 = assign18810_e26351_d_n10;
        locals.var_t4__blk577_dn11 = assign18810_e26351_d_n11;
        locals.var_t4__blk577_dn12 = assign18810_e26351_d_n12;
        locals.var_t4__blk577_dn17 = assign18810_e26351_d_n17;

        let (assign18820_e26359, assign18820_e26359_d_n0, assign18820_e26359_d_n2, assign18820_e26359_d_n6, assign18820_e26359_d_n7, assign18820_e26359_d_n10, assign18820_e26359_d_n11, assign18820_e26359_d_n12, assign18820_e26359_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18820_e26357: f64 = (4.0 / locals.var_costi5);
        (assign18820_e26357, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn7) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn11) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn12) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn17) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5__blk578, locals.var_t5__blk578_dn0, locals.var_t5__blk578_dn2, locals.var_t5__blk578_dn6, locals.var_t5__blk578_dn7, locals.var_t5__blk578_dn10, locals.var_t5__blk578_dn11, locals.var_t5__blk578_dn12, locals.var_t5__blk578_dn17,)
    }
};
        locals.var_t5__blk578 = assign18820_e26359;
        locals.var_t5__blk578_dn0 = assign18820_e26359_d_n0;
        locals.var_t5__blk578_dn2 = assign18820_e26359_d_n2;
        locals.var_t5__blk578_dn6 = assign18820_e26359_d_n6;
        locals.var_t5__blk578_dn7 = assign18820_e26359_d_n7;
        locals.var_t5__blk578_dn10 = assign18820_e26359_d_n10;
        locals.var_t5__blk578_dn11 = assign18820_e26359_d_n11;
        locals.var_t5__blk578_dn12 = assign18820_e26359_d_n12;
        locals.var_t5__blk578_dn17 = assign18820_e26359_d_n17;

        let (assign18830_e26369, assign18830_e26369_d_n0, assign18830_e26369_d_n2, assign18830_e26369_d_n6, assign18830_e26369_d_n7, assign18830_e26369_d_n10, assign18830_e26369_d_n11, assign18830_e26369_d_n12, assign18830_e26369_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18830_e26366: f64 = (locals.var_t4__blk577 * locals.var_t5__blk578);
        let assign18830_e26367: f64 = (1.0 + assign18830_e26366);
        (assign18830_e26367, ((locals.var_t4__blk577_dn0 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn0)), ((locals.var_t4__blk577_dn2 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn2)), ((locals.var_t4__blk577_dn6 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn6)), ((locals.var_t4__blk577_dn7 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn7)), ((locals.var_t4__blk577_dn10 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn10)), ((locals.var_t4__blk577_dn11 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn11)), ((locals.var_t4__blk577_dn12 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn12)), ((locals.var_t4__blk577_dn17 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn17)),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18830_e26369;
        locals.var_t1w_dn0 = assign18830_e26369_d_n0;
        locals.var_t1w_dn2 = assign18830_e26369_d_n2;
        locals.var_t1w_dn6 = assign18830_e26369_d_n6;
        locals.var_t1w_dn7 = assign18830_e26369_d_n7;
        locals.var_t1w_dn10 = assign18830_e26369_d_n10;
        locals.var_t1w_dn11 = assign18830_e26369_d_n11;
        locals.var_t1w_dn12 = assign18830_e26369_d_n12;
        locals.var_t1w_dn17 = assign18830_e26369_d_n17;

        let (assign18840_e26384, assign18840_e26384_d_n0, assign18840_e26384_d_n2, assign18840_e26384_d_n6, assign18840_e26384_d_n7, assign18840_e26384_d_n10, assign18840_e26384_d_n11, assign18840_e26384_d_n12, assign18840_e26384_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18840_e26375: f64 = (locals.var_t1w * locals.var_t1w);
        let assign18840_e26378: f64 = (4.0 * 0.01);
        let assign18840_e26380: f64 = (assign18840_e26378 * 0.01);
        let assign18840_e26381: f64 = (assign18840_e26375 + assign18840_e26380);
        let assign18840_e26382: f64 = (assign18840_e26381).sqrt();
        (assign18840_e26382, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign18840_e26382)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18840_e26384;
        locals.var_tmf1_dn0 = assign18840_e26384_d_n0;
        locals.var_tmf1_dn2 = assign18840_e26384_d_n2;
        locals.var_tmf1_dn6 = assign18840_e26384_d_n6;
        locals.var_tmf1_dn7 = assign18840_e26384_d_n7;
        locals.var_tmf1_dn10 = assign18840_e26384_d_n10;
        locals.var_tmf1_dn11 = assign18840_e26384_d_n11;
        locals.var_tmf1_dn12 = assign18840_e26384_d_n12;
        locals.var_tmf1_dn17 = assign18840_e26384_d_n17;

        let (assign18850_e26398, assign18850_e26398_d_n0, assign18850_e26398_d_n2, assign18850_e26398_d_n6, assign18850_e26398_d_n7, assign18850_e26398_d_n10, assign18850_e26398_d_n11, assign18850_e26398_d_n12, assign18850_e26398_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18850_e26391: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign18850_e26392: f64 = (0.5 * assign18850_e26391);
        let assign18850_e26395: f64 = (1e-10 * 0.01);
        let assign18850_e26396: f64 = (assign18850_e26392 + assign18850_e26395);
        (assign18850_e26396, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18850_e26398;
        locals.var_t1__blk573_dn0 = assign18850_e26398_d_n0;
        locals.var_t1__blk573_dn2 = assign18850_e26398_d_n2;
        locals.var_t1__blk573_dn6 = assign18850_e26398_d_n6;
        locals.var_t1__blk573_dn7 = assign18850_e26398_d_n7;
        locals.var_t1__blk573_dn10 = assign18850_e26398_d_n10;
        locals.var_t1__blk573_dn11 = assign18850_e26398_d_n11;
        locals.var_t1__blk573_dn12 = assign18850_e26398_d_n12;
        locals.var_t1__blk573_dn17 = assign18850_e26398_d_n17;

        let assign18860_e26401: f64 = if locals.var_t1__blk573 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign18860_e26401;

        let (assign18870_e26409, assign18870_e26409_d_n0, assign18870_e26409_d_n2, assign18870_e26409_d_n6, assign18870_e26409_d_n7, assign18870_e26409_d_n10, assign18870_e26409_d_n11, assign18870_e26409_d_n12, assign18870_e26409_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard583 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18870_e26409;
        locals.var_t1__blk573_dn0 = assign18870_e26409_d_n0;
        locals.var_t1__blk573_dn2 = assign18870_e26409_d_n2;
        locals.var_t1__blk573_dn6 = assign18870_e26409_d_n6;
        locals.var_t1__blk573_dn7 = assign18870_e26409_d_n7;
        locals.var_t1__blk573_dn10 = assign18870_e26409_d_n10;
        locals.var_t1__blk573_dn11 = assign18870_e26409_d_n11;
        locals.var_t1__blk573_dn12 = assign18870_e26409_d_n12;
        locals.var_t1__blk573_dn17 = assign18870_e26409_d_n17;

        let (assign18880_e26418, assign18880_e26418_d_n0, assign18880_e26418_d_n2, assign18880_e26418_d_n6, assign18880_e26418_d_n7, assign18880_e26418_d_n10, assign18880_e26418_d_n11, assign18880_e26418_d_n12, assign18880_e26418_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18880_e26415: f64 = (locals.var_t1__blk573 + 1e-50);
        let assign18880_e26416: f64 = (assign18880_e26415).sqrt();
        (assign18880_e26416, (locals.var_t1__blk573_dn0 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn2 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn6 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn7 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn10 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn11 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn12 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn17 / (2.0 * assign18880_e26416)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn12, locals.var_costi6_dn17,)
    }
};
        locals.var_costi6 = assign18880_e26418;
        locals.var_costi6_dn0 = assign18880_e26418_d_n0;
        locals.var_costi6_dn2 = assign18880_e26418_d_n2;
        locals.var_costi6_dn6 = assign18880_e26418_d_n6;
        locals.var_costi6_dn7 = assign18880_e26418_d_n7;
        locals.var_costi6_dn10 = assign18880_e26418_d_n10;
        locals.var_costi6_dn11 = assign18880_e26418_d_n11;
        locals.var_costi6_dn12 = assign18880_e26418_d_n12;
        locals.var_costi6_dn17 = assign18880_e26418_d_n17;

        let (assign18890_e26430, assign18890_e26430_d_n0, assign18890_e26430_d_n2, assign18890_e26430_d_n6, assign18890_e26430_d_n7, assign18890_e26430_d_n10, assign18890_e26430_d_n11, assign18890_e26430_d_n12, assign18890_e26430_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18890_e26426: f64 = (1.0 - locals.var_costi6);
        let assign18890_e26427: f64 = (locals.var_costi4 * assign18890_e26426);
        let assign18890_e26428: f64 = (locals.var_vgssti + assign18890_e26427);
        (assign18890_e26428, (locals.var_vgssti_dn0 + ((locals.var_costi4_dn0 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn0)))), (locals.var_vgssti_dn2 + ((locals.var_costi4_dn2 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn2)))), (locals.var_vgssti_dn6 + ((locals.var_costi4_dn6 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn6)))), (locals.var_vgssti_dn7 + ((locals.var_costi4_dn7 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn7)))), (locals.var_vgssti_dn10 + ((locals.var_costi4_dn10 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn10)))), (locals.var_vgssti_dn11 + ((locals.var_costi4_dn11 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn11)))), (locals.var_vgssti_dn12 + ((locals.var_costi4_dn12 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn12)))), (locals.var_vgssti_dn17 + ((locals.var_costi4_dn17 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn17)))),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn12, locals.var_psasti_dn17,)
    }
};
        locals.var_psasti = assign18890_e26430;
        locals.var_psasti_dn0 = assign18890_e26430_d_n0;
        locals.var_psasti_dn2 = assign18890_e26430_d_n2;
        locals.var_psasti_dn6 = assign18890_e26430_d_n6;
        locals.var_psasti_dn7 = assign18890_e26430_d_n7;
        locals.var_psasti_dn10 = assign18890_e26430_d_n10;
        locals.var_psasti_dn11 = assign18890_e26430_d_n11;
        locals.var_psasti_dn12 = assign18890_e26430_d_n12;
        locals.var_psasti_dn17 = assign18890_e26430_d_n17;

        let (assign18900_e26444, assign18900_e26444_d_n0, assign18900_e26444_d_n2, assign18900_e26444_d_n6, assign18900_e26444_d_n7, assign18900_e26444_d_n10, assign18900_e26444_d_n11, assign18900_e26444_d_n12, assign18900_e26444_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18900_e26439: f64 = (locals.var_vgssti + 1e-50);
        let assign18900_e26440: f64 = (2.0 / assign18900_e26439);
        let assign18900_e26441: f64 = (locals.var_beta + assign18900_e26440);
        let assign18900_e26442: f64 = (1.0 / assign18900_e26441);
        (assign18900_e26442, (-((-((2.0 * locals.var_vgssti_dn0) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn2) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn6) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn7) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (assign18900_e26439 * assign18900_e26439)))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn11) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn12) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn17) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18900_e26444;
        locals.var_t0_dn0 = assign18900_e26444_d_n0;
        locals.var_t0_dn2 = assign18900_e26444_d_n2;
        locals.var_t0_dn6 = assign18900_e26444_d_n6;
        locals.var_t0_dn7 = assign18900_e26444_d_n7;
        locals.var_t0_dn10 = assign18900_e26444_d_n10;
        locals.var_t0_dn11 = assign18900_e26444_d_n11;
        locals.var_t0_dn12 = assign18900_e26444_d_n12;
        locals.var_t0_dn17 = assign18900_e26444_d_n17;

        let (assign18910_e26461, assign18910_e26461_d_n0, assign18910_e26461_d_n2, assign18910_e26461_d_n6, assign18910_e26461_d_n7, assign18910_e26461_d_n10, assign18910_e26461_d_n11, assign18910_e26461_d_n12, assign18910_e26461_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18910_e26450: f64 = (1.0 / locals.var_costi1);
        let assign18910_e26452: f64 = (assign18910_e26450 / locals.var_costi3);
        let assign18910_e26455: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign18910_e26456: f64 = (assign18910_e26452 * assign18910_e26455);
        let assign18910_e26457: f64 = (assign18910_e26456).ln();
        let assign18910_e26459: f64 = (assign18910_e26457 * locals.var_t0);
        (assign18910_e26459, (((((((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn0)), (((((((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn2)), (((((((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn6)), (((((((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn7)), (((((((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn10)), (((((((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn11)), (((((((((-(locals.var_costi1_dn12 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn12)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn12 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn12)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn12)), (((((((((-(locals.var_costi1_dn17 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn17)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn17 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn17)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn17)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn12, locals.var_psbsti_dn17,)
    }
};
        locals.var_psbsti = assign18910_e26461;
        locals.var_psbsti_dn0 = assign18910_e26461_d_n0;
        locals.var_psbsti_dn2 = assign18910_e26461_d_n2;
        locals.var_psbsti_dn6 = assign18910_e26461_d_n6;
        locals.var_psbsti_dn7 = assign18910_e26461_d_n7;
        locals.var_psbsti_dn10 = assign18910_e26461_d_n10;
        locals.var_psbsti_dn11 = assign18910_e26461_d_n11;
        locals.var_psbsti_dn12 = assign18910_e26461_d_n12;
        locals.var_psbsti_dn17 = assign18910_e26461_d_n17;

        let (assign18920_e26471, assign18920_e26471_d_n0, assign18920_e26471_d_n2, assign18920_e26471_d_n6, assign18920_e26471_d_n7, assign18920_e26471_d_n10, assign18920_e26471_d_n11, assign18920_e26471_d_n12, assign18920_e26471_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18920_e26468: f64 = (locals.var_vgssti + 1e-50);
        let assign18920_e26469: f64 = (locals.var_psbsti / assign18920_e26468);
        (assign18920_e26469, (((locals.var_psbsti_dn0 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn0)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn2 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn2)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn6 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn6)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn7 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn7)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn10 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn10)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn11 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn11)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn12 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn12)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn17 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn17)) / (assign18920_e26468 * assign18920_e26468)),)
    } else {
        (locals.var_t3__blk576, locals.var_t3__blk576_dn0, locals.var_t3__blk576_dn2, locals.var_t3__blk576_dn6, locals.var_t3__blk576_dn7, locals.var_t3__blk576_dn10, locals.var_t3__blk576_dn11, locals.var_t3__blk576_dn12, locals.var_t3__blk576_dn17,)
    }
};
        locals.var_t3__blk576 = assign18920_e26471;
        locals.var_t3__blk576_dn0 = assign18920_e26471_d_n0;
        locals.var_t3__blk576_dn2 = assign18920_e26471_d_n2;
        locals.var_t3__blk576_dn6 = assign18920_e26471_d_n6;
        locals.var_t3__blk576_dn7 = assign18920_e26471_d_n7;
        locals.var_t3__blk576_dn10 = assign18920_e26471_d_n10;
        locals.var_t3__blk576_dn11 = assign18920_e26471_d_n11;
        locals.var_t3__blk576_dn12 = assign18920_e26471_d_n12;
        locals.var_t3__blk576_dn17 = assign18920_e26471_d_n17;

        let (assign18930_e26481, assign18930_e26481_d_n0, assign18930_e26481_d_n2, assign18930_e26481_d_n6, assign18930_e26481_d_n7, assign18930_e26481_d_n10, assign18930_e26481_d_n11, assign18930_e26481_d_n12, assign18930_e26481_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18930_e26477: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign18930_e26479: f64 = (assign18930_e26477 - 0.002);
        (assign18930_e26479, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn12 - locals.var_psasti_dn12), (locals.var_psbsti_dn17 - locals.var_psasti_dn17),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn12, locals.var_psab_dn17,)
    }
};
        locals.var_psab = assign18930_e26481;
        locals.var_psab_dn0 = assign18930_e26481_d_n0;
        locals.var_psab_dn2 = assign18930_e26481_d_n2;
        locals.var_psab_dn6 = assign18930_e26481_d_n6;
        locals.var_psab_dn7 = assign18930_e26481_d_n7;
        locals.var_psab_dn10 = assign18930_e26481_d_n10;
        locals.var_psab_dn11 = assign18930_e26481_d_n11;
        locals.var_psab_dn12 = assign18930_e26481_d_n12;
        locals.var_psab_dn17 = assign18930_e26481_d_n17;

        let (assign18940_e26496, assign18940_e26496_d_n0, assign18940_e26496_d_n2, assign18940_e26496_d_n6, assign18940_e26496_d_n7, assign18940_e26496_d_n10, assign18940_e26496_d_n11, assign18940_e26496_d_n12, assign18940_e26496_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18940_e26487: f64 = (locals.var_psab * locals.var_psab);
        let assign18940_e26490: f64 = (4.0 * 0.002);
        let assign18940_e26492: f64 = (assign18940_e26490 * locals.var_psbsti);
        let assign18940_e26493: f64 = (assign18940_e26487 + assign18940_e26492);
        let assign18940_e26494: f64 = (assign18940_e26493).sqrt();
        (assign18940_e26494, ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign18940_e26490 * locals.var_psbsti_dn0)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign18940_e26490 * locals.var_psbsti_dn2)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign18940_e26490 * locals.var_psbsti_dn6)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign18940_e26490 * locals.var_psbsti_dn7)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign18940_e26490 * locals.var_psbsti_dn10)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign18940_e26490 * locals.var_psbsti_dn11)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn12 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn12)) + (assign18940_e26490 * locals.var_psbsti_dn12)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn17 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn17)) + (assign18940_e26490 * locals.var_psbsti_dn17)) / (2.0 * assign18940_e26494)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18940_e26496;
        locals.var_t0_dn0 = assign18940_e26496_d_n0;
        locals.var_t0_dn2 = assign18940_e26496_d_n2;
        locals.var_t0_dn6 = assign18940_e26496_d_n6;
        locals.var_t0_dn7 = assign18940_e26496_d_n7;
        locals.var_t0_dn10 = assign18940_e26496_d_n10;
        locals.var_t0_dn11 = assign18940_e26496_d_n11;
        locals.var_t0_dn12 = assign18940_e26496_d_n12;
        locals.var_t0_dn17 = assign18940_e26496_d_n17;

        let (assign18950_e26508, assign18950_e26508_d_n0, assign18950_e26508_d_n2, assign18950_e26508_d_n6, assign18950_e26508_d_n7, assign18950_e26508_d_n10, assign18950_e26508_d_n11, assign18950_e26508_d_n12, assign18950_e26508_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18950_e26504: f64 = (locals.var_psab + locals.var_t0);
        let assign18950_e26505: f64 = (0.5 * assign18950_e26504);
        let assign18950_e26506: f64 = (locals.var_psbsti - assign18950_e26505);
        (assign18950_e26506, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + locals.var_t0_dn0))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + locals.var_t0_dn2))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + locals.var_t0_dn6))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + locals.var_t0_dn7))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + locals.var_t0_dn10))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + locals.var_t0_dn11))), (locals.var_psbsti_dn12 - (0.5 * (locals.var_psab_dn12 + locals.var_t0_dn12))), (locals.var_psbsti_dn17 - (0.5 * (locals.var_psab_dn17 + locals.var_t0_dn17))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12, locals.var_psti_dn17,)
    }
};
        locals.var_psti = assign18950_e26508;
        locals.var_psti_dn0 = assign18950_e26508_d_n0;
        locals.var_psti_dn2 = assign18950_e26508_d_n2;
        locals.var_psti_dn6 = assign18950_e26508_d_n6;
        locals.var_psti_dn7 = assign18950_e26508_d_n7;
        locals.var_psti_dn10 = assign18950_e26508_d_n10;
        locals.var_psti_dn11 = assign18950_e26508_d_n11;
        locals.var_psti_dn12 = assign18950_e26508_d_n12;
        locals.var_psti_dn17 = assign18950_e26508_d_n17;

        let (assign18960_e26516, assign18960_e26516_d_n0, assign18960_e26516_d_n2, assign18960_e26516_d_n6, assign18960_e26516_d_n7, assign18960_e26516_d_n10, assign18960_e26516_d_n11, assign18960_e26516_d_n12, assign18960_e26516_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18960_e26514: f64 = (1.0 / locals.var_t0);
        (assign18960_e26514, (-(locals.var_t0_dn0 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn2 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn12 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn17 / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18960_e26516;
        locals.var_t1__blk573_dn0 = assign18960_e26516_d_n0;
        locals.var_t1__blk573_dn2 = assign18960_e26516_d_n2;
        locals.var_t1__blk573_dn6 = assign18960_e26516_d_n6;
        locals.var_t1__blk573_dn7 = assign18960_e26516_d_n7;
        locals.var_t1__blk573_dn10 = assign18960_e26516_d_n10;
        locals.var_t1__blk573_dn11 = assign18960_e26516_d_n11;
        locals.var_t1__blk573_dn12 = assign18960_e26516_d_n12;
        locals.var_t1__blk573_dn17 = assign18960_e26516_d_n17;

        let (assign18970_e26527, assign18970_e26527_d_n0, assign18970_e26527_d_n2, assign18970_e26527_d_n6, assign18970_e26527_d_n7, assign18970_e26527_d_n10, assign18970_e26527_d_n11, assign18970_e26527_d_n12, assign18970_e26527_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18970_e26523: f64 = (locals.var_beta * locals.var_psti);
        let assign18970_e26524: f64 = (assign18970_e26523).exp();
        let assign18970_e26525: f64 = (locals.var_costi1 * assign18970_e26524);
        (assign18970_e26525, ((locals.var_costi1_dn0 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn0)))), ((locals.var_costi1_dn2 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn2)))), ((locals.var_costi1_dn6 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn6)))), ((locals.var_costi1_dn7 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn7)))), ((locals.var_costi1_dn10 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))))), ((locals.var_costi1_dn11 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn11)))), ((locals.var_costi1_dn12 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn12)))), ((locals.var_costi1_dn17 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn17)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18970_e26527;
        locals.var_t0_dn0 = assign18970_e26527_d_n0;
        locals.var_t0_dn2 = assign18970_e26527_d_n2;
        locals.var_t0_dn6 = assign18970_e26527_d_n6;
        locals.var_t0_dn7 = assign18970_e26527_d_n7;
        locals.var_t0_dn10 = assign18970_e26527_d_n10;
        locals.var_t0_dn11 = assign18970_e26527_d_n11;
        locals.var_t0_dn12 = assign18970_e26527_d_n12;
        locals.var_t0_dn17 = assign18970_e26527_d_n17;

    }
}

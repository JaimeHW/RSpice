#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_27(
        locals: &mut StampLocals,
    ) {
        let (assign44800_e57997, assign44800_e57997_d_n5, assign44800_e57997_d_n6, assign44800_e57997_d_n7, assign44800_e57997_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44800_e57995: f64 = (locals.var_ed * locals.var_es);
        (assign44800_e57995, ((locals.var_ed_dn5 * locals.var_es) + (locals.var_ed * locals.var_es_dn5)), ((locals.var_ed_dn6 * locals.var_es) + (locals.var_ed * locals.var_es_dn6)), ((locals.var_ed_dn7 * locals.var_es) + (locals.var_ed * locals.var_es_dn7)), ((locals.var_ed_dn8 * locals.var_es) + (locals.var_ed * locals.var_es_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44800_e57997;
        locals.var_temp__blk936_dn5 = assign44800_e57997_d_n5;
        locals.var_temp__blk936_dn6 = assign44800_e57997_d_n6;
        locals.var_temp__blk936_dn7 = assign44800_e57997_d_n7;
        locals.var_temp__blk936_dn8 = assign44800_e57997_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign44810_e58000: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign44810_e58000;
        locals.var_guard1212_rv = 0.0;

        let (assign44820_e58007, assign44820_e58007_d_n5, assign44820_e58007_d_n6, assign44820_e58007_d_n7, assign44820_e58007_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1212 != 0.0)) {
        let assign44820_e58005: f64 = (locals.var_temp__blk936).sqrt();
        (assign44820_e58005, (locals.var_temp__blk936_dn5 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn6 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn7 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn8 / (2.0 * assign44820_e58005)),)
    } else {
        (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8,)
    }
};
        locals.var_em = assign44820_e58007;
        locals.var_em_dn5 = assign44820_e58007_d_n5;
        locals.var_em_dn6 = assign44820_e58007_d_n6;
        locals.var_em_dn7 = assign44820_e58007_d_n7;
        locals.var_em_dn8 = assign44820_e58007_d_n8;
        locals.var_em_rv = 0.0;

        let (assign44830_e58015, assign44830_e58015_d_n5, assign44830_e58015_d_n6, assign44830_e58015_d_n7, assign44830_e58015_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44830_e58012: f64 = (locals.var_ds + locals.var_dd);
        let assign44830_e58013: f64 = (0.5 * assign44830_e58012);
        (assign44830_e58013, (0.5 * (locals.var_ds_dn5 + locals.var_dd_dn5)), (0.5 * (locals.var_ds_dn6 + locals.var_dd_dn6)), (0.5 * (locals.var_ds_dn7 + locals.var_dd_dn7)), (0.5 * (locals.var_ds_dn8 + locals.var_dd_dn8)),)
    } else {
        (locals.var_d_bar, locals.var_d_bar_dn5, locals.var_d_bar_dn6, locals.var_d_bar_dn7, locals.var_d_bar_dn8,)
    }
};
        locals.var_d_bar = assign44830_e58015;
        locals.var_d_bar_dn5 = assign44830_e58015_d_n5;
        locals.var_d_bar_dn6 = assign44830_e58015_d_n6;
        locals.var_d_bar_dn7 = assign44830_e58015_d_n7;
        locals.var_d_bar_dn8 = assign44830_e58015_d_n8;
        locals.var_d_bar_rv = 0.0;

        let (assign44840_e58031, assign44840_e58031_d_n5, assign44840_e58031_d_n6, assign44840_e58031_d_n7, assign44840_e58031_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44840_e58021: f64 = (locals.var_x_ds * locals.var_x_ds);
        let assign44840_e58025: f64 = (2.0 * locals.var_inv_gf2);
        let assign44840_e58026: f64 = (locals.var_em - assign44840_e58025);
        let assign44840_e58027: f64 = (assign44840_e58021 * assign44840_e58026);
        let assign44840_e58028: f64 = (0.125 * assign44840_e58027);
        let assign44840_e58029: f64 = (locals.var_d_bar + assign44840_e58028);
        (assign44840_e58029, (locals.var_d_bar_dn5 + (0.125 * ((((locals.var_x_ds_dn5 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn5)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn5 - (2.0 * locals.var_inv_gf2_dn5)))))), (locals.var_d_bar_dn6 + (0.125 * ((((locals.var_x_ds_dn6 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn6)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn6 - (2.0 * locals.var_inv_gf2_dn6)))))), (locals.var_d_bar_dn7 + (0.125 * ((((locals.var_x_ds_dn7 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn7)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn7 - (2.0 * locals.var_inv_gf2_dn7)))))), (locals.var_d_bar_dn8 + (0.125 * ((((locals.var_x_ds_dn8 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn8)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn8 - (2.0 * locals.var_inv_gf2_dn8)))))),)
    } else {
        (locals.var_dm, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8,)
    }
};
        locals.var_dm = assign44840_e58031;
        locals.var_dm_dn5 = assign44840_e58031_d_n5;
        locals.var_dm_dn6 = assign44840_e58031_d_n6;
        locals.var_dm_dn7 = assign44840_e58031_d_n7;
        locals.var_dm_dn8 = assign44840_e58031_d_n8;
        locals.var_dm_rv = 0.0;

        let assign44850_e58034: f64 = if locals.var_x_m < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign44850_e58034;
        locals.var_guard1213_rv = 0.0;

        let (assign44860_e58056, assign44860_e58056_d_n5, assign44860_e58056_d_n6, assign44860_e58056_d_n7, assign44860_e58056_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44860_e58041: f64 = (locals.var_x_m * locals.var_x_m);
        let assign44860_e58048: f64 = (0.25 * locals.var_x_m);
        let assign44860_e58049: f64 = (1.0 - assign44860_e58048);
        let assign44860_e58050: f64 = (locals.var_x_m * assign44860_e58049);
        let assign44860_e58051: f64 = (0.3333333333333333 * assign44860_e58050);
        let assign44860_e58052: f64 = (1.0 - assign44860_e58051);
        let assign44860_e58053: f64 = (assign44860_e58041 * assign44860_e58052);
        let assign44860_e58054: f64 = (0.5 * assign44860_e58053);
        (assign44860_e58054, (0.5 * ((((locals.var_x_m_dn5 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn5)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn5 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn5))))))))), (0.5 * ((((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6))))))))), (0.5 * ((((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7))))))))), (0.5 * ((((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8))))))))),)
    } else {
        (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8,)
    }
};
        locals.var_pm = assign44860_e58056;
        locals.var_pm_dn5 = assign44860_e58056_d_n5;
        locals.var_pm_dn6 = assign44860_e58056_d_n6;
        locals.var_pm_dn7 = assign44860_e58056_d_n7;
        locals.var_pm_dn8 = assign44860_e58056_d_n8;
        locals.var_pm_rv = 0.0;

        let (assign44870_e58067, assign44870_e58067_d_n5, assign44870_e58067_d_n6, assign44870_e58067_d_n7, assign44870_e58067_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44870_e58063: f64 = (locals.var_dm + locals.var_pm);
        let assign44870_e58064: f64 = (assign44870_e58063).sqrt();
        let assign44870_e58065: f64 = (locals.var_gf * assign44870_e58064);
        (assign44870_e58065, ((locals.var_gf_dn5 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn6 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn7 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn8 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign44870_e58064)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8,)
    }
};
        locals.var_xgm = assign44870_e58067;
        locals.var_xgm_dn5 = assign44870_e58067_d_n5;
        locals.var_xgm_dn6 = assign44870_e58067_d_n6;
        locals.var_xgm_dn7 = assign44870_e58067_d_n7;
        locals.var_xgm_dn8 = assign44870_e58067_d_n8;
        locals.var_xgm_rv = 0.0;

        let assign44880_e58070: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign44880_e58070;
        locals.var_guard1214_rv = 0.0;

        let (assign44890_e58085, assign44890_e58085_d_n5, assign44890_e58085_d_n6, assign44890_e58085_d_n7, assign44890_e58085_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 != 0.0)) {
        let assign44890_e58080: f64 = (locals.var_kp * locals.var_xgm);
        let assign44890_e58081: f64 = (1.0 + assign44890_e58080);
        let assign44890_e58082: f64 = (assign44890_e58081).sqrt();
        let assign44890_e58083: f64 = (1.0 / assign44890_e58082);
        (assign44890_e58083, (-(((locals.var_kp * locals.var_xgm_dn5) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8,)
    }
};
        locals.var_eta_p = assign44890_e58085;
        locals.var_eta_p_dn5 = assign44890_e58085_d_n5;
        locals.var_eta_p_dn6 = assign44890_e58085_d_n6;
        locals.var_eta_p_dn7 = assign44890_e58085_d_n7;
        locals.var_eta_p_dn8 = assign44890_e58085_d_n8;
        locals.var_eta_p_rv = 0.0;

        let (assign44900_e58102, assign44900_e58102_d_n5, assign44900_e58102_d_n6, assign44900_e58102_d_n7, assign44900_e58102_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44900_e58095: f64 = (0.25 * locals.var_x_m);
        let assign44900_e58096: f64 = (1.0 - assign44900_e58095);
        let assign44900_e58097: f64 = (locals.var_x_m * assign44900_e58096);
        let assign44900_e58098: f64 = (0.3333333333333333 * assign44900_e58097);
        let assign44900_e58099: f64 = (1.0 - assign44900_e58098);
        let assign44900_e58100: f64 = (assign44900_e58099).sqrt();
        (assign44900_e58100, ((-(0.3333333333333333 * ((locals.var_x_m_dn5 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn5)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8)))))) / (2.0 * assign44900_e58100)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44900_e58102;
        locals.var_temp__blk936_dn5 = assign44900_e58102_d_n5;
        locals.var_temp__blk936_dn6 = assign44900_e58102_d_n6;
        locals.var_temp__blk936_dn7 = assign44900_e58102_d_n7;
        locals.var_temp__blk936_dn8 = assign44900_e58102_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign44910_e58112, assign44910_e58112_d_n5, assign44910_e58112_d_n6, assign44910_e58112_d_n7, assign44910_e58112_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44910_e58109: f64 = (locals.var_x_m * locals.var_temp__blk936);
        let assign44910_e58110: f64 = (0.7071067811865475 * assign44910_e58109);
        (assign44910_e58110, (0.7071067811865475 * ((locals.var_x_m_dn5 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_m_dn6 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_m_dn7 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_m_dn8 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8,)
    }
};
        locals.var_sqm = assign44910_e58112;
        locals.var_sqm_dn5 = assign44910_e58112_d_n5;
        locals.var_sqm_dn6 = assign44910_e58112_d_n6;
        locals.var_sqm_dn7 = assign44910_e58112_d_n7;
        locals.var_sqm_dn8 = assign44910_e58112_d_n8;
        locals.var_sqm_rv = 0.0;

        let (assign44920_e58136, assign44920_e58136_d_n5, assign44920_e58136_d_n6, assign44920_e58136_d_n7, assign44920_e58136_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44920_e58122: f64 = (0.5 * locals.var_x_m);
        let assign44920_e58123: f64 = (1.0 - assign44920_e58122);
        let assign44920_e58127: f64 = (locals.var_x_m * locals.var_x_m);
        let assign44920_e58128: f64 = (0.16666666666666666 * assign44920_e58127);
        let assign44920_e58129: f64 = (assign44920_e58123 + assign44920_e58128);
        let assign44920_e58130: f64 = (locals.var_gf * assign44920_e58129);
        let assign44920_e58132: f64 = (assign44920_e58130 / locals.var_temp__blk936);
        let assign44920_e58133: f64 = (0.7071067811865475 * assign44920_e58132);
        let assign44920_e58134: f64 = (locals.var_eta_p + assign44920_e58133);
        (assign44920_e58134, (locals.var_eta_p_dn5 + (0.7071067811865475 * (((((locals.var_gf_dn5 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn5)) + (0.16666666666666666 * ((locals.var_x_m_dn5 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn5)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn6 + (0.7071067811865475 * (((((locals.var_gf_dn6 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn6)) + (0.16666666666666666 * ((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn7 + (0.7071067811865475 * (((((locals.var_gf_dn7 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn7)) + (0.16666666666666666 * ((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn8 + (0.7071067811865475 * (((((locals.var_gf_dn8 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn8)) + (0.16666666666666666 * ((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8,)
    }
};
        locals.var_alpha = assign44920_e58136;
        locals.var_alpha_dn5 = assign44920_e58136_d_n5;
        locals.var_alpha_dn6 = assign44920_e58136_d_n6;
        locals.var_alpha_dn7 = assign44920_e58136_d_n7;
        locals.var_alpha_dn8 = assign44920_e58136_d_n8;
        locals.var_alpha_rv = 0.0;

        let (assign44930_e58147, assign44930_e58147_d_n5, assign44930_e58147_d_n6, assign44930_e58147_d_n7, assign44930_e58147_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
        let assign44930_e58143: f64 = (locals.var_x_m - 1.0);
        let assign44930_e58145: f64 = (assign44930_e58143 + locals.var_em);
        (assign44930_e58145, (locals.var_x_m_dn5 + locals.var_em_dn5), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8),)
    } else {
        (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8,)
    }
};
        locals.var_pm = assign44930_e58147;
        locals.var_pm_dn5 = assign44930_e58147_d_n5;
        locals.var_pm_dn6 = assign44930_e58147_d_n6;
        locals.var_pm_dn7 = assign44930_e58147_d_n7;
        locals.var_pm_dn8 = assign44930_e58147_d_n8;
        locals.var_pm_rv = 0.0;

        let (assign44940_e58159, assign44940_e58159_d_n5, assign44940_e58159_d_n6, assign44940_e58159_d_n7, assign44940_e58159_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
        let assign44940_e58155: f64 = (locals.var_dm + locals.var_pm);
        let assign44940_e58156: f64 = (assign44940_e58155).sqrt();
        let assign44940_e58157: f64 = (locals.var_gf * assign44940_e58156);
        (assign44940_e58157, ((locals.var_gf_dn5 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn6 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn7 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn8 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign44940_e58156)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8,)
    }
};
        locals.var_xgm = assign44940_e58159;
        locals.var_xgm_dn5 = assign44940_e58159_d_n5;
        locals.var_xgm_dn6 = assign44940_e58159_d_n6;
        locals.var_xgm_dn7 = assign44940_e58159_d_n7;
        locals.var_xgm_dn8 = assign44940_e58159_d_n8;
        locals.var_xgm_rv = 0.0;

        let assign44950_e58162: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign44950_e58162;
        locals.var_guard1215_rv = 0.0;

        let (assign44960_e58179, assign44960_e58179_d_n5, assign44960_e58179_d_n6, assign44960_e58179_d_n7, assign44960_e58179_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign44960_e58171: f64 = (1.0 - locals.var_em);
        let assign44960_e58175: f64 = (locals.var_xgm * locals.var_inv_gf2);
        let assign44960_e58176: f64 = (2.0 * assign44960_e58175);
        let assign44960_e58177: f64 = (assign44960_e58171 + assign44960_e58176);
        (assign44960_e58177, ((-locals.var_em_dn5) + (2.0 * ((locals.var_xgm_dn5 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn5)))), ((-locals.var_em_dn6) + (2.0 * ((locals.var_xgm_dn6 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((locals.var_xgm_dn7 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((locals.var_xgm_dn8 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn8)))),)
    } else {
        (locals.var_d0, locals.var_d0_dn5, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn8,)
    }
};
        locals.var_d0 = assign44960_e58179;
        locals.var_d0_dn5 = assign44960_e58179_d_n5;
        locals.var_d0_dn6 = assign44960_e58179_d_n6;
        locals.var_d0_dn7 = assign44960_e58179_d_n7;
        locals.var_d0_dn8 = assign44960_e58179_d_n8;
        locals.var_d0_rv = 0.0;

        let (assign44970_e58195, assign44970_e58195_d_n5, assign44970_e58195_d_n6, assign44970_e58195_d_n7, assign44970_e58195_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign44970_e58190: f64 = (locals.var_kp * locals.var_xgm);
        let assign44970_e58191: f64 = (1.0 + assign44970_e58190);
        let assign44970_e58192: f64 = (assign44970_e58191).sqrt();
        let assign44970_e58193: f64 = (1.0 / assign44970_e58192);
        (assign44970_e58193, (-(((locals.var_kp * locals.var_xgm_dn5) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8,)
    }
};
        locals.var_eta_p = assign44970_e58195;
        locals.var_eta_p_dn5 = assign44970_e58195_d_n5;
        locals.var_eta_p_dn6 = assign44970_e58195_d_n6;
        locals.var_eta_p_dn7 = assign44970_e58195_d_n7;
        locals.var_eta_p_dn8 = assign44970_e58195_d_n8;
        locals.var_eta_p_rv = 0.0;

        let (assign44980_e58208, assign44980_e58208_d_n5, assign44980_e58208_d_n6, assign44980_e58208_d_n7, assign44980_e58208_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign44980_e58205: f64 = (locals.var_eta_p + 1.0);
        let assign44980_e58206: f64 = (locals.var_eta_p / assign44980_e58205);
        (assign44980_e58206, (((locals.var_eta_p_dn5 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn5)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn6 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn6)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn7 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn7)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn8 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn8)) / (assign44980_e58205 * assign44980_e58205)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44980_e58208;
        locals.var_temp__blk936_dn5 = assign44980_e58208_d_n5;
        locals.var_temp__blk936_dn6 = assign44980_e58208_d_n6;
        locals.var_temp__blk936_dn7 = assign44980_e58208_d_n7;
        locals.var_temp__blk936_dn8 = assign44980_e58208_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign44990_e58225, assign44990_e58225_d_n5, assign44990_e58225_d_n6, assign44990_e58225_d_n7, assign44990_e58225_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign44990_e58218: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign44990_e58220: f64 = (assign44990_e58218 * locals.var_gf2);
        let assign44990_e58222: f64 = (assign44990_e58220 * locals.var_dm);
        let assign44990_e58223: f64 = (locals.var_kp * assign44990_e58222);
        (assign44990_e58223, (locals.var_kp * ((((((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn5)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn5))), (locals.var_kp * ((((((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn6)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn6))), (locals.var_kp * ((((((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn7)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn7))), (locals.var_kp * ((((((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn8)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn8))),)
    } else {
        (locals.var_x_pm, locals.var_x_pm_dn5, locals.var_x_pm_dn6, locals.var_x_pm_dn7, locals.var_x_pm_dn8,)
    }
};
        locals.var_x_pm = assign44990_e58225;
        locals.var_x_pm_dn5 = assign44990_e58225_d_n5;
        locals.var_x_pm_dn6 = assign44990_e58225_d_n6;
        locals.var_x_pm_dn7 = assign44990_e58225_d_n7;
        locals.var_x_pm_dn8 = assign44990_e58225_d_n8;
        locals.var_x_pm_rv = 0.0;

        let (assign45000_e58246, assign45000_e58246_d_n5, assign45000_e58246_d_n6, assign45000_e58246_d_n7, assign45000_e58246_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45000_e58235: f64 = (locals.var_xgm - locals.var_x_pm);
        let assign45000_e58236: f64 = (2.0 * assign45000_e58235);
        let assign45000_e58240: f64 = (1.0 - locals.var_em);
        let assign45000_e58242: f64 = (assign45000_e58240 + locals.var_dm);
        let assign45000_e58243: f64 = (locals.var_gf2 * assign45000_e58242);
        let assign45000_e58244: f64 = (assign45000_e58236 + assign45000_e58243);
        (assign45000_e58244, ((2.0 * (locals.var_xgm_dn5 - locals.var_x_pm_dn5)) + ((locals.var_gf2_dn5 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn5) + locals.var_dm_dn5)))), ((2.0 * (locals.var_xgm_dn6 - locals.var_x_pm_dn6)) + ((locals.var_gf2_dn6 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn6) + locals.var_dm_dn6)))), ((2.0 * (locals.var_xgm_dn7 - locals.var_x_pm_dn7)) + ((locals.var_gf2_dn7 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn7) + locals.var_dm_dn7)))), ((2.0 * (locals.var_xgm_dn8 - locals.var_x_pm_dn8)) + ((locals.var_gf2_dn8 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn8) + locals.var_dm_dn8)))),)
    } else {
        (locals.var_p_pd, locals.var_p_pd_dn5, locals.var_p_pd_dn6, locals.var_p_pd_dn7, locals.var_p_pd_dn8,)
    }
};
        locals.var_p_pd = assign45000_e58246;
        locals.var_p_pd_dn5 = assign45000_e58246_d_n5;
        locals.var_p_pd_dn6 = assign45000_e58246_d_n6;
        locals.var_p_pd_dn7 = assign45000_e58246_d_n7;
        locals.var_p_pd_dn8 = assign45000_e58246_d_n8;
        locals.var_p_pd_rv = 0.0;

        let (assign45010_e58261, assign45010_e58261_d_n5, assign45010_e58261_d_n6, assign45010_e58261_d_n7, assign45010_e58261_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45010_e58257: f64 = (2.0 * locals.var_xgm);
        let assign45010_e58258: f64 = (locals.var_x_pm - assign45010_e58257);
        let assign45010_e58259: f64 = (locals.var_x_pm * assign45010_e58258);
        (assign45010_e58259, ((locals.var_x_pm_dn5 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn5 - (2.0 * locals.var_xgm_dn5)))), ((locals.var_x_pm_dn6 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn6 - (2.0 * locals.var_xgm_dn6)))), ((locals.var_x_pm_dn7 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn7 - (2.0 * locals.var_xgm_dn7)))), ((locals.var_x_pm_dn8 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn8 - (2.0 * locals.var_xgm_dn8)))),)
    } else {
        (locals.var_q_pd, locals.var_q_pd_dn5, locals.var_q_pd_dn6, locals.var_q_pd_dn7, locals.var_q_pd_dn8,)
    }
};
        locals.var_q_pd = assign45010_e58261;
        locals.var_q_pd_dn5 = assign45010_e58261_d_n5;
        locals.var_q_pd_dn6 = assign45010_e58261_d_n6;
        locals.var_q_pd_dn7 = assign45010_e58261_d_n7;
        locals.var_q_pd_dn8 = assign45010_e58261_d_n8;
        locals.var_q_pd_rv = 0.0;

        let (assign45020_e58278, assign45020_e58278_d_n5, assign45020_e58278_d_n6, assign45020_e58278_d_n7, assign45020_e58278_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45020_e58273: f64 = (locals.var_em + locals.var_dm);
        let assign45020_e58274: f64 = (locals.var_gf2 * assign45020_e58273);
        let assign45020_e58275: f64 = (0.5 * assign45020_e58274);
        let assign45020_e58276: f64 = (1.0 - assign45020_e58275);
        (assign45020_e58276, (-(0.5 * ((locals.var_gf2_dn5 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn5 + locals.var_dm_dn5))))), (-(0.5 * ((locals.var_gf2_dn6 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn6 + locals.var_dm_dn6))))), (-(0.5 * ((locals.var_gf2_dn7 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn7 + locals.var_dm_dn7))))), (-(0.5 * ((locals.var_gf2_dn8 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn8 + locals.var_dm_dn8))))),)
    } else {
        (locals.var_xi_pd, locals.var_xi_pd_dn5, locals.var_xi_pd_dn6, locals.var_xi_pd_dn7, locals.var_xi_pd_dn8,)
    }
};
        locals.var_xi_pd = assign45020_e58278;
        locals.var_xi_pd_dn5 = assign45020_e58278_d_n5;
        locals.var_xi_pd_dn6 = assign45020_e58278_d_n6;
        locals.var_xi_pd_dn7 = assign45020_e58278_d_n7;
        locals.var_xi_pd_dn8 = assign45020_e58278_d_n8;
        locals.var_xi_pd_rv = 0.0;

        let (assign45030_e58297, assign45030_e58297_d_n5, assign45030_e58297_d_n6, assign45030_e58297_d_n7, assign45030_e58297_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45030_e58287: f64 = (locals.var_q_pd * locals.var_p_pd);
        let assign45030_e58290: f64 = (locals.var_p_pd * locals.var_p_pd);
        let assign45030_e58293: f64 = (locals.var_xi_pd * locals.var_q_pd);
        let assign45030_e58294: f64 = (assign45030_e58290 - assign45030_e58293);
        let assign45030_e58295: f64 = (assign45030_e58287 / assign45030_e58294);
        (assign45030_e58295, (((((locals.var_q_pd_dn5 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn5)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn5 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn5)) - ((locals.var_xi_pd_dn5 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn5))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn6 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn6)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn6 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn6)) - ((locals.var_xi_pd_dn6 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn6))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn7 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn7)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn7 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn7)) - ((locals.var_xi_pd_dn7 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn7))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn8 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn8)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn8 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn8)) - ((locals.var_xi_pd_dn8 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn8))))) / (assign45030_e58294 * assign45030_e58294)),)
    } else {
        (locals.var_u_pd, locals.var_u_pd_dn5, locals.var_u_pd_dn6, locals.var_u_pd_dn7, locals.var_u_pd_dn8,)
    }
};
        locals.var_u_pd = assign45030_e58297;
        locals.var_u_pd_dn5 = assign45030_e58297_d_n5;
        locals.var_u_pd_dn6 = assign45030_e58297_d_n6;
        locals.var_u_pd_dn7 = assign45030_e58297_d_n7;
        locals.var_u_pd_dn8 = assign45030_e58297_d_n8;
        locals.var_u_pd_rv = 0.0;

        let (assign45040_e58308, assign45040_e58308_d_n5, assign45040_e58308_d_n6, assign45040_e58308_d_n7, assign45040_e58308_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45040_e58306: f64 = (locals.var_x_m + locals.var_u_pd);
        (assign45040_e58306, (locals.var_x_m_dn5 + locals.var_u_pd_dn5), (locals.var_x_m_dn6 + locals.var_u_pd_dn6), (locals.var_x_m_dn7 + locals.var_u_pd_dn7), (locals.var_x_m_dn8 + locals.var_u_pd_dn8),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8,)
    }
};
        locals.var_x_m = assign45040_e58308;
        locals.var_x_m_dn5 = assign45040_e58308_d_n5;
        locals.var_x_m_dn6 = assign45040_e58308_d_n6;
        locals.var_x_m_dn7 = assign45040_e58308_d_n7;
        locals.var_x_m_dn8 = assign45040_e58308_d_n8;
        locals.var_x_m_rv = 0.0;

        let (assign45050_e58318, assign45050_e58318_d_n5, assign45050_e58318_d_n6, assign45050_e58318_d_n7, assign45050_e58318_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45050_e58316: f64 = (locals.var_u_pd).exp();
        (assign45050_e58316, (assign45050_e58316 * locals.var_u_pd_dn5), (assign45050_e58316 * locals.var_u_pd_dn6), (assign45050_e58316 * locals.var_u_pd_dn7), (assign45050_e58316 * locals.var_u_pd_dn8),)
    } else {
        (locals.var_km, locals.var_km_dn5, locals.var_km_dn6, locals.var_km_dn7, locals.var_km_dn8,)
    }
};
        locals.var_km = assign45050_e58318;
        locals.var_km_dn5 = assign45050_e58318_d_n5;
        locals.var_km_dn6 = assign45050_e58318_d_n6;
        locals.var_km_dn7 = assign45050_e58318_d_n7;
        locals.var_km_dn8 = assign45050_e58318_d_n8;
        locals.var_km_rv = 0.0;

        let (assign45060_e58329, assign45060_e58329_d_n5, assign45060_e58329_d_n6, assign45060_e58329_d_n7, assign45060_e58329_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45060_e58327: f64 = (locals.var_em / locals.var_km);
        (assign45060_e58327, (((locals.var_em_dn5 * locals.var_km) - (locals.var_em * locals.var_km_dn5)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn6 * locals.var_km) - (locals.var_em * locals.var_km_dn6)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn7 * locals.var_km) - (locals.var_em * locals.var_km_dn7)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn8 * locals.var_km) - (locals.var_em * locals.var_km_dn8)) / (locals.var_km * locals.var_km)),)
    } else {
        (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8,)
    }
};
        locals.var_em = assign45060_e58329;
        locals.var_em_dn5 = assign45060_e58329_d_n5;
        locals.var_em_dn6 = assign45060_e58329_d_n6;
        locals.var_em_dn7 = assign45060_e58329_d_n7;
        locals.var_em_dn8 = assign45060_e58329_d_n8;
        locals.var_em_rv = 0.0;

        let (assign45070_e58340, assign45070_e58340_d_n5, assign45070_e58340_d_n6, assign45070_e58340_d_n7, assign45070_e58340_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45070_e58338: f64 = (locals.var_dm * locals.var_km);
        (assign45070_e58338, ((locals.var_dm_dn5 * locals.var_km) + (locals.var_dm * locals.var_km_dn5)), ((locals.var_dm_dn6 * locals.var_km) + (locals.var_dm * locals.var_km_dn6)), ((locals.var_dm_dn7 * locals.var_km) + (locals.var_dm * locals.var_km_dn7)), ((locals.var_dm_dn8 * locals.var_km) + (locals.var_dm * locals.var_km_dn8)),)
    } else {
        (locals.var_dm, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8,)
    }
};
        locals.var_dm = assign45070_e58340;
        locals.var_dm_dn5 = assign45070_e58340_d_n5;
        locals.var_dm_dn6 = assign45070_e58340_d_n6;
        locals.var_dm_dn7 = assign45070_e58340_d_n7;
        locals.var_dm_dn8 = assign45070_e58340_d_n8;
        locals.var_dm_rv = 0.0;

        let (assign45080_e58353, assign45080_e58353_d_n5, assign45080_e58353_d_n6, assign45080_e58353_d_n7, assign45080_e58353_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45080_e58349: f64 = (locals.var_x_m - 1.0);
        let assign45080_e58351: f64 = (assign45080_e58349 + locals.var_em);
        (assign45080_e58351, (locals.var_x_m_dn5 + locals.var_em_dn5), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8),)
    } else {
        (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8,)
    }
};
        locals.var_pm = assign45080_e58353;
        locals.var_pm_dn5 = assign45080_e58353_d_n5;
        locals.var_pm_dn6 = assign45080_e58353_d_n6;
        locals.var_pm_dn7 = assign45080_e58353_d_n7;
        locals.var_pm_dn8 = assign45080_e58353_d_n8;
        locals.var_pm_rv = 0.0;

        let (assign45090_e58367, assign45090_e58367_d_n5, assign45090_e58367_d_n6, assign45090_e58367_d_n7, assign45090_e58367_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45090_e58363: f64 = (locals.var_dm + locals.var_pm);
        let assign45090_e58364: f64 = (assign45090_e58363).sqrt();
        let assign45090_e58365: f64 = (locals.var_gf * assign45090_e58364);
        (assign45090_e58365, ((locals.var_gf_dn5 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn6 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn7 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn8 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45090_e58364)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8,)
    }
};
        locals.var_xgm = assign45090_e58367;
        locals.var_xgm_dn5 = assign45090_e58367_d_n5;
        locals.var_xgm_dn6 = assign45090_e58367_d_n6;
        locals.var_xgm_dn7 = assign45090_e58367_d_n7;
        locals.var_xgm_dn8 = assign45090_e58367_d_n8;
        locals.var_xgm_rv = 0.0;

        let (assign45100_e58386, assign45100_e58386_d_n5, assign45100_e58386_d_n6, assign45100_e58386_d_n7, assign45100_e58386_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45100_e58376: f64 = (1.0 - locals.var_em);
        let assign45100_e58380: f64 = (locals.var_xgm * locals.var_eta_p);
        let assign45100_e58382: f64 = (assign45100_e58380 * locals.var_inv_gf2);
        let assign45100_e58383: f64 = (2.0 * assign45100_e58382);
        let assign45100_e58384: f64 = (assign45100_e58376 + assign45100_e58383);
        (assign45100_e58384, ((-locals.var_em_dn5) + (2.0 * ((((locals.var_xgm_dn5 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn5)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn5)))), ((-locals.var_em_dn6) + (2.0 * ((((locals.var_xgm_dn6 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn6)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((((locals.var_xgm_dn7 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn7)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((((locals.var_xgm_dn8 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn8)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn8)))),)
    } else {
        (locals.var_km0, locals.var_km0_dn5, locals.var_km0_dn6, locals.var_km0_dn7, locals.var_km0_dn8,)
    }
};
        locals.var_km0 = assign45100_e58386;
        locals.var_km0_dn5 = assign45100_e58386_d_n5;
        locals.var_km0_dn6 = assign45100_e58386_d_n6;
        locals.var_km0_dn7 = assign45100_e58386_d_n7;
        locals.var_km0_dn8 = assign45100_e58386_d_n8;
        locals.var_km0_rv = 0.0;

        let (assign45110_e58407, assign45110_e58407_d_n5, assign45110_e58407_d_n6, assign45110_e58407_d_n7, assign45110_e58407_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45110_e58395: f64 = (locals.var_x_ds * locals.var_km);
        let assign45110_e58398: f64 = (locals.var_d0 + locals.var_d_bar);
        let assign45110_e58399: f64 = (assign45110_e58395 * assign45110_e58398);
        let assign45110_e58403: f64 = (locals.var_km * locals.var_d_bar);
        let assign45110_e58404: f64 = (locals.var_km0 + assign45110_e58403);
        let assign45110_e58405: f64 = (assign45110_e58399 / assign45110_e58404);
        (assign45110_e58405, (((((((locals.var_x_ds_dn5 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn5)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn5 + locals.var_d_bar_dn5))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn5 + ((locals.var_km_dn5 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn5))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn6 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn6)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn6 + locals.var_d_bar_dn6))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn6 + ((locals.var_km_dn6 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn6))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn7 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn7)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn7 + locals.var_d_bar_dn7))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn7 + ((locals.var_km_dn7 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn7))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn8 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn8)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn8 + locals.var_d_bar_dn8))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn8 + ((locals.var_km_dn8 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn8))))) / (assign45110_e58404 * assign45110_e58404)),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8,)
    }
};
        locals.var_x_ds = assign45110_e58407;
        locals.var_x_ds_dn5 = assign45110_e58407_d_n5;
        locals.var_x_ds_dn6 = assign45110_e58407_d_n6;
        locals.var_x_ds_dn7 = assign45110_e58407_d_n7;
        locals.var_x_ds_dn8 = assign45110_e58407_d_n8;
        locals.var_x_ds_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        locals: &mut StampLocals,
    ) {
        let (assign45120_e58418, assign45120_e58418_d_n5, assign45120_e58418_d_n6, assign45120_e58418_d_n7, assign45120_e58418_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45120_e58416: f64 = (locals.var_x_ds * locals.var_phit1);
        (assign45120_e58416, ((locals.var_x_ds_dn5 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn5)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)),)
    } else {
        (locals.var_dps, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8,)
    }
};
        locals.var_dps = assign45120_e58418;
        locals.var_dps_dn5 = assign45120_e58418_d_n5;
        locals.var_dps_dn6 = assign45120_e58418_d_n6;
        locals.var_dps_dn7 = assign45120_e58418_d_n7;
        locals.var_dps_dn8 = assign45120_e58418_d_n8;
        locals.var_dps_rv = 0.0;

        let (assign45130_e58426, assign45130_e58426_d_n5, assign45130_e58426_d_n6, assign45130_e58426_d_n7, assign45130_e58426_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
        let assign45130_e58424: f64 = (locals.var_pm).sqrt();
        (assign45130_e58424, (locals.var_pm_dn5 / (2.0 * assign45130_e58424)), (locals.var_pm_dn6 / (2.0 * assign45130_e58424)), (locals.var_pm_dn7 / (2.0 * assign45130_e58424)), (locals.var_pm_dn8 / (2.0 * assign45130_e58424)),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8,)
    }
};
        locals.var_sqm = assign45130_e58426;
        locals.var_sqm_dn5 = assign45130_e58426_d_n5;
        locals.var_sqm_dn6 = assign45130_e58426_d_n6;
        locals.var_sqm_dn7 = assign45130_e58426_d_n7;
        locals.var_sqm_dn8 = assign45130_e58426_d_n8;
        locals.var_sqm_rv = 0.0;

        let (assign45140_e58443, assign45140_e58443_d_n5, assign45140_e58443_d_n6, assign45140_e58443_d_n7, assign45140_e58443_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
        let assign45140_e58436: f64 = (1.0 - locals.var_em);
        let assign45140_e58437: f64 = (locals.var_gf * assign45140_e58436);
        let assign45140_e58439: f64 = (assign45140_e58437 / locals.var_sqm);
        let assign45140_e58440: f64 = (0.5 * assign45140_e58439);
        let assign45140_e58441: f64 = (locals.var_eta_p + assign45140_e58440);
        (assign45140_e58441, (locals.var_eta_p_dn5 + (0.5 * (((((locals.var_gf_dn5 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn5))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn5)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn6 + (0.5 * (((((locals.var_gf_dn6 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn6))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn6)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn7 + (0.5 * (((((locals.var_gf_dn7 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn7))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn7)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn8 + (0.5 * (((((locals.var_gf_dn8 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn8))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn8)) / (locals.var_sqm * locals.var_sqm)))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8,)
    }
};
        locals.var_alpha = assign45140_e58443;
        locals.var_alpha_dn5 = assign45140_e58443_d_n5;
        locals.var_alpha_dn6 = assign45140_e58443_d_n6;
        locals.var_alpha_dn7 = assign45140_e58443_d_n7;
        locals.var_alpha_dn8 = assign45140_e58443_d_n8;
        locals.var_alpha_rv = 0.0;

        let (assign45150_e58457, assign45150_e58457_d_n5, assign45150_e58457_d_n6, assign45150_e58457_d_n7, assign45150_e58457_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45150_e58448: f64 = (locals.var_gf2 * locals.var_dm);
        let assign45150_e58452: f64 = (locals.var_gf * locals.var_sqm);
        let assign45150_e58453: f64 = (locals.var_xgm + assign45150_e58452);
        let assign45150_e58454: f64 = (assign45150_e58448 / assign45150_e58453);
        let assign45150_e58455: f64 = (locals.var_phit1 * assign45150_e58454);
        (assign45150_e58455, ((locals.var_phit1_dn5 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn5 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn5)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn5 + ((locals.var_gf_dn5 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn5))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn6 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn6 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn6)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn6 + ((locals.var_gf_dn6 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn6))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn7 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn7 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn7)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn7 + ((locals.var_gf_dn7 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn7))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn8 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn8 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn8)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn8 + ((locals.var_gf_dn8 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn8))))) / (assign45150_e58453 * assign45150_e58453)))),)
    } else {
        (locals.var_qim, locals.var_qim_dn5, locals.var_qim_dn6, locals.var_qim_dn7, locals.var_qim_dn8,)
    }
};
        locals.var_qim = assign45150_e58457;
        locals.var_qim_dn5 = assign45150_e58457_d_n5;
        locals.var_qim_dn6 = assign45150_e58457_d_n6;
        locals.var_qim_dn7 = assign45150_e58457_d_n7;
        locals.var_qim_dn8 = assign45150_e58457_d_n8;
        locals.var_qim_rv = 0.0;

        let (assign45160_e58465, assign45160_e58465_d_n5, assign45160_e58465_d_n6, assign45160_e58465_d_n7, assign45160_e58465_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45160_e58462: f64 = (locals.var_phit1 * locals.var_alpha);
        let assign45160_e58463: f64 = (locals.var_qim + assign45160_e58462);
        (assign45160_e58463, (locals.var_qim_dn5 + ((locals.var_phit1_dn5 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn5))), (locals.var_qim_dn6 + ((locals.var_phit1_dn6 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn6))), (locals.var_qim_dn7 + ((locals.var_phit1_dn7 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn7))), (locals.var_qim_dn8 + ((locals.var_phit1_dn8 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn8))),)
    } else {
        (locals.var_qim1, locals.var_qim1_dn5, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8,)
    }
};
        locals.var_qim1 = assign45160_e58465;
        locals.var_qim1_dn5 = assign45160_e58465_d_n5;
        locals.var_qim1_dn6 = assign45160_e58465_d_n6;
        locals.var_qim1_dn7 = assign45160_e58465_d_n7;
        locals.var_qim1_dn8 = assign45160_e58465_d_n8;
        locals.var_qim1_rv = 0.0;

        let (assign45170_e58473, assign45170_e58473_d_n5, assign45170_e58473_d_n6, assign45170_e58473_d_n7, assign45170_e58473_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45170_e58469: f64 = (locals.var_sqm * locals.var_gf);
        let assign45170_e58471: f64 = (assign45170_e58469 * locals.var_phit1);
        (assign45170_e58471, ((((locals.var_sqm_dn5 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn5)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn5)), ((((locals.var_sqm_dn6 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn6)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn6)), ((((locals.var_sqm_dn7 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn7)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn7)), ((((locals.var_sqm_dn8 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn8)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn8)),)
    } else {
        (locals.var_qbm, locals.var_qbm_dn5, locals.var_qbm_dn6, locals.var_qbm_dn7, locals.var_qbm_dn8,)
    }
};
        locals.var_qbm = assign45170_e58473;
        locals.var_qbm_dn5 = assign45170_e58473_d_n5;
        locals.var_qbm_dn6 = assign45170_e58473_d_n6;
        locals.var_qbm_dn7 = assign45170_e58473_d_n7;
        locals.var_qbm_dn8 = assign45170_e58473_d_n8;
        locals.var_qbm_rv = 0.0;

        let assign45180_e58476: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign45180_e58476;
        locals.var_guard1216_rv = 0.0;

        let (assign45190_e58486, assign45190_e58486_d_n5, assign45190_e58486_d_n6, assign45190_e58486_d_n7, assign45190_e58486_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1216 != 0.0)) {
        let assign45190_e58483: f64 = (locals.var_rsg_i * locals.var_qim);
        let assign45190_e58484: f64 = (1.0 - assign45190_e58483);
        (assign45190_e58484, (-(locals.var_rsg_i * locals.var_qim_dn5)), (-(locals.var_rsg_i * locals.var_qim_dn6)), (-(locals.var_rsg_i * locals.var_qim_dn7)), (-(locals.var_rsg_i * locals.var_qim_dn8)),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8,)
    }
};
        locals.var_rhog = assign45190_e58486;
        locals.var_rhog_dn5 = assign45190_e58486_d_n5;
        locals.var_rhog_dn6 = assign45190_e58486_d_n6;
        locals.var_rhog_dn7 = assign45190_e58486_d_n7;
        locals.var_rhog_dn8 = assign45190_e58486_d_n8;
        locals.var_rhog_rv = 0.0;

        let (assign45200_e58499, assign45200_e58499_d_n5, assign45200_e58499_d_n6, assign45200_e58499_d_n7, assign45200_e58499_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1216 == 0.0)) {
        let assign45200_e58495: f64 = (locals.var_rsg_i * locals.var_qim);
        let assign45200_e58496: f64 = (1.0 + assign45200_e58495);
        let assign45200_e58497: f64 = (1.0 / assign45200_e58496);
        (assign45200_e58497, (-((locals.var_rsg_i * locals.var_qim_dn5) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn6) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn7) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn8) / (assign45200_e58496 * assign45200_e58496))),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8,)
    }
};
        locals.var_rhog = assign45200_e58499;
        locals.var_rhog_dn5 = assign45200_e58499_d_n5;
        locals.var_rhog_dn6 = assign45200_e58499_d_n6;
        locals.var_rhog_dn7 = assign45200_e58499_d_n7;
        locals.var_rhog_dn8 = assign45200_e58499_d_n8;
        locals.var_rhog_rv = 0.0;

        let (assign45210_e58509, assign45210_e58509_d_n5, assign45210_e58509_d_n6, assign45210_e58509_d_n7, assign45210_e58509_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45210_e58503: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign45210_e58505: f64 = (assign45210_e58503 * locals.var_rhog);
        let assign45210_e58507: f64 = (assign45210_e58505 * locals.var_qim);
        (assign45210_e58507, (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn5)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn5)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn6)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn7)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn8)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn8)),)
    } else {
        (locals.var_gr, locals.var_gr_dn5, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8,)
    }
};
        locals.var_gr = assign45210_e58509;
        locals.var_gr_dn5 = assign45210_e58509_d_n5;
        locals.var_gr_dn6 = assign45210_e58509_d_n6;
        locals.var_gr_dn7 = assign45210_e58509_d_n7;
        locals.var_gr_dn8 = assign45210_e58509_d_n8;
        locals.var_gr_rv = 0.0;

        let (assign45220_e58517, assign45220_e58517_d_n5, assign45220_e58517_d_n6, assign45220_e58517_d_n7, assign45220_e58517_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45220_e58514: f64 = (locals.var_eta_mu * locals.var_qim);
        let assign45220_e58515: f64 = (locals.var_qbm + assign45220_e58514);
        (assign45220_e58515, (locals.var_qbm_dn5 + (locals.var_eta_mu * locals.var_qim_dn5)), (locals.var_qbm_dn6 + (locals.var_eta_mu * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu * locals.var_qim_dn8)),)
    } else {
        (locals.var_qeff, locals.var_qeff_dn5, locals.var_qeff_dn6, locals.var_qeff_dn7, locals.var_qeff_dn8,)
    }
};
        locals.var_qeff = assign45220_e58517;
        locals.var_qeff_dn5 = assign45220_e58517_d_n5;
        locals.var_qeff_dn6 = assign45220_e58517_d_n6;
        locals.var_qeff_dn7 = assign45220_e58517_d_n7;
        locals.var_qeff_dn8 = assign45220_e58517_d_n8;
        locals.var_qeff_rv = 0.0;

        let (assign45230_e58525, assign45230_e58525_d_n5, assign45230_e58525_d_n6, assign45230_e58525_d_n7, assign45230_e58525_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45230_e58522: f64 = (locals.var_eta_mu1 * locals.var_qim);
        let assign45230_e58523: f64 = (locals.var_qbm + assign45230_e58522);
        (assign45230_e58523, (locals.var_qbm_dn5 + (locals.var_eta_mu1 * locals.var_qim_dn5)), (locals.var_qbm_dn6 + (locals.var_eta_mu1 * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu1 * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu1 * locals.var_qim_dn8)),)
    } else {
        (locals.var_qeff1, locals.var_qeff1_dn5, locals.var_qeff1_dn6, locals.var_qeff1_dn7, locals.var_qeff1_dn8,)
    }
};
        locals.var_qeff1 = assign45230_e58525;
        locals.var_qeff1_dn5 = assign45230_e58525_d_n5;
        locals.var_qeff1_dn6 = assign45230_e58525_d_n6;
        locals.var_qeff1_dn7 = assign45230_e58525_d_n7;
        locals.var_qeff1_dn8 = assign45230_e58525_d_n8;
        locals.var_qeff1_rv = 0.0;

        let (assign45240_e58531, assign45240_e58531_d_n5, assign45240_e58531_d_n6, assign45240_e58531_d_n7, assign45240_e58531_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45240_e58529: f64 = (locals.var_e_eff0 * locals.var_qeff);
        (assign45240_e58529, (locals.var_e_eff0 * locals.var_qeff_dn5), (locals.var_e_eff0 * locals.var_qeff_dn6), (locals.var_e_eff0 * locals.var_qeff_dn7), (locals.var_e_eff0 * locals.var_qeff_dn8),)
    } else {
        (locals.var_eeffm, locals.var_eeffm_dn5, locals.var_eeffm_dn6, locals.var_eeffm_dn7, locals.var_eeffm_dn8,)
    }
};
        locals.var_eeffm = assign45240_e58531;
        locals.var_eeffm_dn5 = assign45240_e58531_d_n5;
        locals.var_eeffm_dn6 = assign45240_e58531_d_n6;
        locals.var_eeffm_dn7 = assign45240_e58531_d_n7;
        locals.var_eeffm_dn8 = assign45240_e58531_d_n8;
        locals.var_eeffm_rv = 0.0;

        let (assign45250_e58542, assign45250_e58542_d_n5, assign45250_e58542_d_n6, assign45250_e58542_d_n7, assign45250_e58542_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45250_e58536: f64 = (locals.var_pm + locals.var_dm);
        let assign45250_e58538: f64 = (assign45250_e58536 + 1e-14);
        let assign45250_e58539: f64 = (locals.var_pm / assign45250_e58538);
        let assign45250_e58540: f64 = (assign45250_e58539).ln();
        (assign45250_e58540, ((((locals.var_pm_dn5 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn5 + locals.var_dm_dn5))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn6 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn6 + locals.var_dm_dn6))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn7 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn7 + locals.var_dm_dn7))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn8 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn8 + locals.var_dm_dn8))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign45250_e58542;
        locals.var_temp1_dn5 = assign45250_e58542_d_n5;
        locals.var_temp1_dn6 = assign45250_e58542_d_n6;
        locals.var_temp1_dn7 = assign45250_e58542_d_n7;
        locals.var_temp1_dn8 = assign45250_e58542_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign45260_e58559, assign45260_e58559_d_n5, assign45260_e58559_d_n6, assign45260_e58559_d_n7, assign45260_e58559_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45260_e58546: f64 = (locals.var_eeffm * locals.var_mue_t);
        let assign45260_e58548: f64 = (assign45260_e58546).powf(locals.var_themu_t);
        let assign45260_e58552: f64 = (0.5 * locals.var_thecs_t);
        let assign45260_e58554: f64 = (assign45260_e58552 * locals.var_temp1);
        let assign45260_e58555: f64 = (assign45260_e58554).exp();
        let assign45260_e58556: f64 = (locals.var_cs_t * assign45260_e58555);
        let assign45260_e58557: f64 = (assign45260_e58548 + assign45260_e58556);
        (assign45260_e58557, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn5 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn5 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn6 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn6 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn7 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn7 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn8 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn8 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn8)))),)
    } else {
        (locals.var_mutmp, locals.var_mutmp_dn5, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8,)
    }
};
        locals.var_mutmp = assign45260_e58559;
        locals.var_mutmp_dn5 = assign45260_e58559_d_n5;
        locals.var_mutmp_dn6 = assign45260_e58559_d_n6;
        locals.var_mutmp_dn7 = assign45260_e58559_d_n7;
        locals.var_mutmp_dn8 = assign45260_e58559_d_n8;
        locals.var_mutmp_rv = 0.0;

        let (assign45270_e58569, assign45270_e58569_d_n5, assign45270_e58569_d_n6, assign45270_e58569_d_n7, assign45270_e58569_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45270_e58563: f64 = (1.0 + locals.var_mutmp);
        let assign45270_e58565: f64 = (assign45270_e58563 + locals.var_gr);
        let assign45270_e58567: f64 = (assign45270_e58565 * locals.var_rxcor);
        (assign45270_e58567, (((locals.var_mutmp_dn5 + locals.var_gr_dn5) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn5)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn8)),)
    } else {
        (locals.var_gmob, locals.var_gmob_dn5, locals.var_gmob_dn6, locals.var_gmob_dn7, locals.var_gmob_dn8,)
    }
};
        locals.var_gmob = assign45270_e58569;
        locals.var_gmob_dn5 = assign45270_e58569_d_n5;
        locals.var_gmob_dn6 = assign45270_e58569_d_n6;
        locals.var_gmob_dn7 = assign45270_e58569_d_n7;
        locals.var_gmob_dn8 = assign45270_e58569_d_n8;
        locals.var_gmob_rv = 0.0;

        let (assign45280_e58588, assign45280_e58588_d_n5, assign45280_e58588_d_n6, assign45280_e58588_d_n7, assign45280_e58588_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45280_e58574: f64 = (locals.var_v_ds - locals.var_dps);
        let assign45280_e58576: f64 = (assign45280_e58574 * locals.var_inv_vp);
        let assign45280_e58577: f64 = (1.0 + assign45280_e58576);
        let assign45280_e58581: f64 = (locals.var_vdse - locals.var_dps);
        let assign45280_e58583: f64 = (assign45280_e58581 * locals.var_inv_vp);
        let assign45280_e58584: f64 = (1.0 + assign45280_e58583);
        let assign45280_e58585: f64 = (assign45280_e58577 / assign45280_e58584);
        let assign45280_e58586: f64 = (assign45280_e58585).ln();
        (assign45280_e58586, ((((((-locals.var_dps_dn5) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn5 - locals.var_dps_dn5) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((locals.var_v_ds_dn6 - locals.var_dps_dn6) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn6 - locals.var_dps_dn6) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((locals.var_v_ds_dn7 - locals.var_dps_dn7) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn7 - locals.var_dps_dn7) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((-locals.var_dps_dn8) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn8 - locals.var_dps_dn8) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585),)
    } else {
        (locals.var_s1, locals.var_s1_dn5, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8,)
    }
};
        locals.var_s1 = assign45280_e58588;
        locals.var_s1_dn5 = assign45280_e58588_d_n5;
        locals.var_s1_dn6 = assign45280_e58588_d_n6;
        locals.var_s1_dn7 = assign45280_e58588_d_n7;
        locals.var_s1_dn8 = assign45280_e58588_d_n8;
        locals.var_s1_rv = 0.0;

        let (assign45290_e58594, assign45290_e58594_d_n5, assign45290_e58594_d_n6, assign45290_e58594_d_n7, assign45290_e58594_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45290_e58592: f64 = (locals.var_qim * locals.var_xitsb);
        (assign45290_e58592, ((locals.var_qim_dn5 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn5)), ((locals.var_qim_dn6 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn6)), ((locals.var_qim_dn7 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn7)), ((locals.var_qim_dn8 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign45290_e58594;
        locals.var_temp2_dn5 = assign45290_e58594_d_n5;
        locals.var_temp2_dn6 = assign45290_e58594_d_n6;
        locals.var_temp2_dn7 = assign45290_e58594_d_n7;
        locals.var_temp2_dn8 = assign45290_e58594_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign45300_e58602, assign45300_e58602_d_n5, assign45300_e58602_d_n6, assign45300_e58602_d_n7, assign45300_e58602_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45300_e58599: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign45300_e58600: f64 = (locals.var_temp2 / assign45300_e58599);
        (assign45300_e58600, (((locals.var_temp2_dn5 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn6 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn7 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn8 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign45300_e58599 * assign45300_e58599)),)
    } else {
        (locals.var_wsat, locals.var_wsat_dn5, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8,)
    }
};
        locals.var_wsat = assign45300_e58602;
        locals.var_wsat_dn5 = assign45300_e58602_d_n5;
        locals.var_wsat_dn6 = assign45300_e58602_d_n6;
        locals.var_wsat_dn7 = assign45300_e58602_d_n7;
        locals.var_wsat_dn8 = assign45300_e58602_d_n8;
        locals.var_wsat_rv = 0.0;

        let assign45310_e58605: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign45310_e58605;
        locals.var_guard1217_rv = 0.0;

        let (assign45320_e58617, assign45320_e58617_d_n5, assign45320_e58617_d_n6, assign45320_e58617_d_n7, assign45320_e58617_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign45320_e58613: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign45320_e58614: f64 = (1.0 - assign45320_e58613);
        let assign45320_e58615: f64 = (1.0 / assign45320_e58614);
        (assign45320_e58615, (-((-(locals.var_thesatg_i * locals.var_wsat_dn5)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign45320_e58614 * assign45320_e58614))),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8,)
    }
};
        locals.var_factheta = assign45320_e58617;
        locals.var_factheta_dn5 = assign45320_e58617_d_n5;
        locals.var_factheta_dn6 = assign45320_e58617_d_n6;
        locals.var_factheta_dn7 = assign45320_e58617_d_n7;
        locals.var_factheta_dn8 = assign45320_e58617_d_n8;
        locals.var_factheta_rv = 0.0;

        let (assign45330_e58628, assign45330_e58628_d_n5, assign45330_e58628_d_n6, assign45330_e58628_d_n7, assign45330_e58628_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1217 == 0.0)) {
        let assign45330_e58625: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign45330_e58626: f64 = (1.0 + assign45330_e58625);
        (assign45330_e58626, (locals.var_thesatg_i * locals.var_wsat_dn5), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8,)
    }
};
        locals.var_factheta = assign45330_e58628;
        locals.var_factheta_dn5 = assign45330_e58628_d_n5;
        locals.var_factheta_dn6 = assign45330_e58628_d_n6;
        locals.var_factheta_dn7 = assign45330_e58628_d_n7;
        locals.var_factheta_dn8 = assign45330_e58628_d_n8;
        locals.var_factheta_rv = 0.0;

        let (assign45340_e58634, assign45340_e58634_d_n5, assign45340_e58634_d_n6, assign45340_e58634_d_n7, assign45340_e58634_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45340_e58632: f64 = (locals.var_thesatloc * locals.var_factheta);
        (assign45340_e58632, (locals.var_thesatloc * locals.var_factheta_dn5), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8),)
    } else {
        (locals.var_thesateff, locals.var_thesateff_dn5, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8,)
    }
};
        locals.var_thesateff = assign45340_e58634;
        locals.var_thesateff_dn5 = assign45340_e58634_d_n5;
        locals.var_thesateff_dn6 = assign45340_e58634_d_n6;
        locals.var_thesateff_dn7 = assign45340_e58634_d_n7;
        locals.var_thesateff_dn8 = assign45340_e58634_d_n8;
        locals.var_thesateff_rv = 0.0;

        let (assign45350_e58640, assign45350_e58640_d_n5, assign45350_e58640_d_n6, assign45350_e58640_d_n7, assign45350_e58640_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45350_e58638: f64 = (locals.var_xgm * locals.var_phit1);
        (assign45350_e58638, ((locals.var_xgm_dn5 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn5)), ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6)), ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7)), ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8)),)
    } else {
        (locals.var_voxm, locals.var_voxm_dn5, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8,)
    }
};
        locals.var_voxm = assign45350_e58640;
        locals.var_voxm_dn5 = assign45350_e58640_d_n5;
        locals.var_voxm_dn6 = assign45350_e58640_d_n6;
        locals.var_voxm_dn7 = assign45350_e58640_d_n7;
        locals.var_voxm_dn8 = assign45350_e58640_d_n8;
        locals.var_voxm_rv = 0.0;

        locals.var_vdsat_lim_dc = locals.var_vdsat_lim;
        locals.var_vdsat_lim_dc_dn5 = locals.var_vdsat_lim_dn5;
        locals.var_vdsat_lim_dc_dn6 = locals.var_vdsat_lim_dn6;
        locals.var_vdsat_lim_dc_dn7 = locals.var_vdsat_lim_dn7;
        locals.var_vdsat_lim_dc_dn8 = locals.var_vdsat_lim_dn8;
        locals.var_vdsat_lim_dc_rv = 0.0;

        locals.var_vdse_dc = locals.var_vdse;
        locals.var_vdse_dc_dn5 = locals.var_vdse_dn5;
        locals.var_vdse_dc_dn6 = locals.var_vdse_dn6;
        locals.var_vdse_dc_dn7 = locals.var_vdse_dn7;
        locals.var_vdse_dc_dn8 = locals.var_vdse_dn8;
        locals.var_vdse_dc_rv = 0.0;

        locals.var_udse_dc = locals.var_udse;
        locals.var_udse_dc_dn5 = locals.var_udse_dn5;
        locals.var_udse_dc_dn6 = locals.var_udse_dn6;
        locals.var_udse_dc_dn7 = locals.var_udse_dn7;
        locals.var_udse_dc_dn8 = locals.var_udse_dn8;
        locals.var_udse_dc_rv = 0.0;

        locals.var_x_ds_dc = locals.var_x_ds;
        locals.var_x_ds_dc_dn5 = locals.var_x_ds_dn5;
        locals.var_x_ds_dc_dn6 = locals.var_x_ds_dn6;
        locals.var_x_ds_dc_dn7 = locals.var_x_ds_dn7;
        locals.var_x_ds_dc_dn8 = locals.var_x_ds_dn8;
        locals.var_x_ds_dc_rv = 0.0;

        locals.var_dps_dc = locals.var_dps;
        locals.var_dps_dc_dn5 = locals.var_dps_dn5;
        locals.var_dps_dc_dn6 = locals.var_dps_dn6;
        locals.var_dps_dc_dn7 = locals.var_dps_dn7;
        locals.var_dps_dc_dn8 = locals.var_dps_dn8;
        locals.var_dps_dc_rv = 0.0;

        locals.var_x_m_dc = locals.var_x_m;
        locals.var_x_m_dc_dn5 = locals.var_x_m_dn5;
        locals.var_x_m_dc_dn6 = locals.var_x_m_dn6;
        locals.var_x_m_dc_dn7 = locals.var_x_m_dn7;
        locals.var_x_m_dc_dn8 = locals.var_x_m_dn8;
        locals.var_x_m_dc_rv = 0.0;

        locals.var_qbd_dc = locals.var_qbd;
        locals.var_qbd_dc_dn5 = locals.var_qbd_dn5;
        locals.var_qbd_dc_dn6 = locals.var_qbd_dn6;
        locals.var_qbd_dc_dn7 = locals.var_qbd_dn7;
        locals.var_qbd_dc_dn8 = locals.var_qbd_dn8;
        locals.var_qbd_dc_rv = 0.0;

        locals.var_eta_p_dc = locals.var_eta_p;
        locals.var_eta_p_dc_dn5 = locals.var_eta_p_dn5;
        locals.var_eta_p_dc_dn6 = locals.var_eta_p_dn6;
        locals.var_eta_p_dc_dn7 = locals.var_eta_p_dn7;
        locals.var_eta_p_dc_dn8 = locals.var_eta_p_dn8;
        locals.var_eta_p_dc_rv = 0.0;

        locals.var_alpha_dc = locals.var_alpha;
        locals.var_alpha_dc_dn5 = locals.var_alpha_dn5;
        locals.var_alpha_dc_dn6 = locals.var_alpha_dn6;
        locals.var_alpha_dc_dn7 = locals.var_alpha_dn7;
        locals.var_alpha_dc_dn8 = locals.var_alpha_dn8;
        locals.var_alpha_dc_rv = 0.0;

        locals.var_qim_dc = locals.var_qim;
        locals.var_qim_dc_dn5 = locals.var_qim_dn5;
        locals.var_qim_dc_dn6 = locals.var_qim_dn6;
        locals.var_qim_dc_dn7 = locals.var_qim_dn7;
        locals.var_qim_dc_dn8 = locals.var_qim_dn8;
        locals.var_qim_dc_rv = 0.0;

        locals.var_qim1_dc = locals.var_qim1;
        locals.var_qim1_dc_dn5 = locals.var_qim1_dn5;
        locals.var_qim1_dc_dn6 = locals.var_qim1_dn6;
        locals.var_qim1_dc_dn7 = locals.var_qim1_dn7;
        locals.var_qim1_dc_dn8 = locals.var_qim1_dn8;
        locals.var_qim1_dc_rv = 0.0;

        locals.var_qbm_dc = locals.var_qbm;
        locals.var_qbm_dc_dn5 = locals.var_qbm_dn5;
        locals.var_qbm_dc_dn6 = locals.var_qbm_dn6;
        locals.var_qbm_dc_dn7 = locals.var_qbm_dn7;
        locals.var_qbm_dc_dn8 = locals.var_qbm_dn8;
        locals.var_qbm_dc_rv = 0.0;

        locals.var_qeff1_dc = locals.var_qeff1;
        locals.var_qeff1_dc_dn5 = locals.var_qeff1_dn5;
        locals.var_qeff1_dc_dn6 = locals.var_qeff1_dn6;
        locals.var_qeff1_dc_dn7 = locals.var_qeff1_dn7;
        locals.var_qeff1_dc_dn8 = locals.var_qeff1_dn8;
        locals.var_qeff1_dc_rv = 0.0;

        locals.var_gmob_dc = locals.var_gmob;
        locals.var_gmob_dc_dn5 = locals.var_gmob_dn5;
        locals.var_gmob_dc_dn6 = locals.var_gmob_dn6;
        locals.var_gmob_dc_dn7 = locals.var_gmob_dn7;
        locals.var_gmob_dc_dn8 = locals.var_gmob_dn8;
        locals.var_gmob_dc_rv = 0.0;

        locals.var_s1_dc = locals.var_s1;
        locals.var_s1_dc_dn5 = locals.var_s1_dn5;
        locals.var_s1_dc_dn6 = locals.var_s1_dn6;
        locals.var_s1_dc_dn7 = locals.var_s1_dn7;
        locals.var_s1_dc_dn8 = locals.var_s1_dn8;
        locals.var_s1_dc_rv = 0.0;

        locals.var_thesateff_dc = locals.var_thesateff;
        locals.var_thesateff_dc_dn5 = locals.var_thesateff_dn5;
        locals.var_thesateff_dc_dn6 = locals.var_thesateff_dn6;
        locals.var_thesateff_dc_dn7 = locals.var_thesateff_dn7;
        locals.var_thesateff_dc_dn8 = locals.var_thesateff_dn8;
        locals.var_thesateff_dc_rv = 0.0;

        locals.var_voxm_dc = locals.var_voxm;
        locals.var_voxm_dc_dn5 = locals.var_voxm_dn5;
        locals.var_voxm_dc_dn6 = locals.var_voxm_dn6;
        locals.var_voxm_dc_dn7 = locals.var_voxm_dn7;
        locals.var_voxm_dc_dn8 = locals.var_voxm_dn8;
        locals.var_voxm_dc_rv = 0.0;

        locals.var_gdl_dc = 1.0;
        locals.var_gdl_dc_dn5 = 0.0;
        locals.var_gdl_dc_dn6 = 0.0;
        locals.var_gdl_dc_dn7 = 0.0;
        locals.var_gdl_dc_dn8 = 0.0;
        locals.var_gdl_dc_rv = 0.0;

        locals.var_gmob_dl_dc = 1.0;
        locals.var_gmob_dl_dc_dn5 = 0.0;
        locals.var_gmob_dl_dc_dn6 = 0.0;
        locals.var_gmob_dl_dc_dn7 = 0.0;
        locals.var_gmob_dl_dc_dn8 = 0.0;
        locals.var_gmob_dl_dc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_gvsatinv_dc = 1.0;
        locals.var_gvsatinv_dc_dn5 = 0.0;
        locals.var_gvsatinv_dc_dn6 = 0.0;
        locals.var_gvsatinv_dc_dn7 = 0.0;
        locals.var_gvsatinv_dc_dn8 = 0.0;
        locals.var_gvsatinv_dc_rv = 0.0;

        locals.var_h_dc = 1.0;
        locals.var_h_dc_dn5 = 0.0;
        locals.var_h_dc_dn6 = 0.0;
        locals.var_h_dc_dn7 = 0.0;
        locals.var_h_dc_dn8 = 0.0;
        locals.var_h_dc_rv = 0.0;

        locals.var_i_ds = 0.0;
        locals.var_i_ds_dn5 = 0.0;
        locals.var_i_ds_dn6 = 0.0;
        locals.var_i_ds_dn7 = 0.0;
        locals.var_i_ds_dn8 = 0.0;
        locals.var_i_ds_rv = 0.0;

        let assign45690_e58714: f64 = if locals.var_xg_dc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign45690_e58714;
        locals.var_guard1218_rv = 0.0;

        let (assign45700_e58723, assign45700_e58723_d_n6, assign45700_e58723_d_n7,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45700_e58719: f64 = (locals.var_vdsx * locals.var_inv_vp);
        let assign45700_e58720: f64 = (1.0 + assign45700_e58719);
        let assign45700_e58721: f64 = (assign45700_e58720).ln();
        (assign45700_e58721, ((locals.var_vdsx_dn6 * locals.var_inv_vp) / assign45700_e58720), ((locals.var_vdsx_dn7 * locals.var_inv_vp) / assign45700_e58720),)
    } else {
        (locals.var_s2, locals.var_s2_dn6, locals.var_s2_dn7,)
    }
};
        locals.var_s2 = assign45700_e58723;
        locals.var_s2_dn6 = assign45700_e58723_d_n6;
        locals.var_s2_dn7 = assign45700_e58723_d_n7;
        locals.var_s2_rv = 0.0;

        let (assign45710_e58731, assign45710_e58731_d_n5, assign45710_e58731_d_n6, assign45710_e58731_d_n7, assign45710_e58731_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45710_e58727: f64 = (locals.var_phit1_dc * locals.var_alpha_dc);
        let assign45710_e58729: f64 = (assign45710_e58727 / locals.var_qim1_dc);
        (assign45710_e58729, (((((locals.var_phit1_dc_dn5 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn5)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn6 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn6)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn7 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn7)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn8 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn8)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign45710_e58731;
        locals.var_temp__blk936_dn5 = assign45710_e58731_d_n5;
        locals.var_temp__blk936_dn6 = assign45710_e58731_d_n6;
        locals.var_temp__blk936_dn7 = assign45710_e58731_d_n7;
        locals.var_temp__blk936_dn8 = assign45710_e58731_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign45720_e58755, assign45720_e58755_d_n5, assign45720_e58755_d_n6, assign45720_e58755_d_n7, assign45720_e58755_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45720_e58736: f64 = (locals.var_alp1_i / locals.var_qim1_dc);
        let assign45720_e58737: f64 = (locals.var_alp_i + assign45720_e58736);
        let assign45720_e58739: f64 = (assign45720_e58737 * locals.var_qim_dc);
        let assign45720_e58741: f64 = (assign45720_e58739 / locals.var_qim1_dc);
        let assign45720_e58743: f64 = (assign45720_e58741 * locals.var_s1_dc);
        let assign45720_e58746: f64 = (locals.var_alp2_i * locals.var_qbm_dc);
        let assign45720_e58748: f64 = (assign45720_e58746 * locals.var_temp__blk936);
        let assign45720_e58750: f64 = (assign45720_e58748 * locals.var_temp__blk936);
        let assign45720_e58752: f64 = (assign45720_e58750 * locals.var_s2);
        let assign45720_e58753: f64 = (assign45720_e58743 + assign45720_e58752);
        (assign45720_e58753, (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn5) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn5)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn5)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn5) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn5)) * locals.var_s2)), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn6) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn6)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn6)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn6) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn6)) * locals.var_s2) + (assign45720_e58750 * locals.var_s2_dn6))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn7) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn7)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn7)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn7) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn7)) * locals.var_s2) + (assign45720_e58750 * locals.var_s2_dn7))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn8) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn8)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn8)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn8) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn8)) * locals.var_s2)),)
    } else {
        (locals.var_dl, locals.var_dl_dn5, locals.var_dl_dn6, locals.var_dl_dn7, locals.var_dl_dn8,)
    }
};
        locals.var_dl = assign45720_e58755;
        locals.var_dl_dn5 = assign45720_e58755_d_n5;
        locals.var_dl_dn6 = assign45720_e58755_d_n6;
        locals.var_dl_dn7 = assign45720_e58755_d_n7;
        locals.var_dl_dn8 = assign45720_e58755_d_n8;
        locals.var_dl_rv = 0.0;

        let (assign45730_e58767, assign45730_e58767_d_n5, assign45730_e58767_d_n6, assign45730_e58767_d_n7, assign45730_e58767_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45730_e58760: f64 = (1.0 + locals.var_dl);
        let assign45730_e58763: f64 = (locals.var_dl * locals.var_dl);
        let assign45730_e58764: f64 = (assign45730_e58760 + assign45730_e58763);
        let assign45730_e58765: f64 = (1.0 / assign45730_e58764);
        (assign45730_e58765, (-((locals.var_dl_dn5 + ((locals.var_dl_dn5 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn5))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn6 + ((locals.var_dl_dn6 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn6))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn7 + ((locals.var_dl_dn7 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn7))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn8 + ((locals.var_dl_dn8 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn8))) / (assign45730_e58764 * assign45730_e58764))),)
    } else {
        (locals.var_gdl_dc, locals.var_gdl_dc_dn5, locals.var_gdl_dc_dn6, locals.var_gdl_dc_dn7, locals.var_gdl_dc_dn8,)
    }
};
        locals.var_gdl_dc = assign45730_e58767;
        locals.var_gdl_dc_dn5 = assign45730_e58767_d_n5;
        locals.var_gdl_dc_dn6 = assign45730_e58767_d_n6;
        locals.var_gdl_dc_dn7 = assign45730_e58767_d_n7;
        locals.var_gdl_dc_dn8 = assign45730_e58767_d_n8;
        locals.var_gdl_dc_rv = 0.0;

        let (assign45740_e58773, assign45740_e58773_d_n5, assign45740_e58773_d_n6, assign45740_e58773_d_n7, assign45740_e58773_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45740_e58771: f64 = (locals.var_gmob_dc * locals.var_gdl_dc);
        (assign45740_e58771, ((locals.var_gmob_dc_dn5 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn5)), ((locals.var_gmob_dc_dn6 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn6)), ((locals.var_gmob_dc_dn7 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn7)), ((locals.var_gmob_dc_dn8 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn8)),)
    } else {
        (locals.var_gmob_dl_dc, locals.var_gmob_dl_dc_dn5, locals.var_gmob_dl_dc_dn6, locals.var_gmob_dl_dc_dn7, locals.var_gmob_dl_dc_dn8,)
    }
};
        locals.var_gmob_dl_dc = assign45740_e58773;
        locals.var_gmob_dl_dc_dn5 = assign45740_e58773_d_n5;
        locals.var_gmob_dl_dc_dn6 = assign45740_e58773_d_n6;
        locals.var_gmob_dl_dc_dn7 = assign45740_e58773_d_n7;
        locals.var_gmob_dl_dc_dn8 = assign45740_e58773_d_n8;
        locals.var_gmob_dl_dc_rv = 0.0;

        let (assign45750_e58779, assign45750_e58779_d_n5, assign45750_e58779_d_n6, assign45750_e58779_d_n7, assign45750_e58779_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45750_e58777: f64 = (locals.var_thesateff_dc / locals.var_gmob_dl_dc);
        (assign45750_e58777, (((locals.var_thesateff_dc_dn5 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn5)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn6)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn7)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn8)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)),)
    } else {
        (locals.var_thesat1_dc, locals.var_thesat1_dc_dn5, locals.var_thesat1_dc_dn6, locals.var_thesat1_dc_dn7, locals.var_thesat1_dc_dn8,)
    }
};
        locals.var_thesat1_dc = assign45750_e58779;
        locals.var_thesat1_dc_dn5 = assign45750_e58779_d_n5;
        locals.var_thesat1_dc_dn6 = assign45750_e58779_d_n6;
        locals.var_thesat1_dc_dn7 = assign45750_e58779_d_n7;
        locals.var_thesat1_dc_dn8 = assign45750_e58779_d_n8;
        locals.var_thesat1_dc_rv = 0.0;

        let (assign45760_e58789, assign45760_e58789_d_n5, assign45760_e58789_d_n6, assign45760_e58789_d_n7, assign45760_e58789_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45760_e58783: f64 = (locals.var_thesat1_dc * locals.var_thesat1_dc);
        let assign45760_e58785: f64 = (assign45760_e58783 * locals.var_dps_dc);
        let assign45760_e58787: f64 = (assign45760_e58785 * locals.var_dps_dc);
        (assign45760_e58787, ((((((locals.var_thesat1_dc_dn5 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn5)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn5)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn5)), ((((((locals.var_thesat1_dc_dn6 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn6)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_dc_dn7 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn7)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_dc_dn8 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn8)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn8)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8,)
    }
};
        locals.var_zsat = assign45760_e58789;
        locals.var_zsat_dn5 = assign45760_e58789_d_n5;
        locals.var_zsat_dn6 = assign45760_e58789_d_n6;
        locals.var_zsat_dn7 = assign45760_e58789_d_n7;
        locals.var_zsat_dn8 = assign45760_e58789_d_n8;
        locals.var_zsat_rv = 0.0;

        let assign45770_e58792: f64 = (-1.0);
        let assign45770_e58793: f64 = if locals.var_chnl_type == assign45770_e58792 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign45770_e58793;
        locals.var_guard1219_rv = 0.0;

        let (assign45780_e58805, assign45780_e58805_d_n5, assign45780_e58805_d_n6, assign45780_e58805_d_n7, assign45780_e58805_d_n8,) = {
    if ((locals.var_guard1218 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign45780_e58801: f64 = (locals.var_thesat1_dc * locals.var_dps_dc);
        let assign45780_e58802: f64 = (1.0 + assign45780_e58801);
        let assign45780_e58803: f64 = (locals.var_zsat / assign45780_e58802);
        (assign45780_e58803, (((locals.var_zsat_dn5 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn5 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn5)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn6 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn6)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn7 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn7)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn8 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn8)))) / (assign45780_e58802 * assign45780_e58802)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8,)
    }
};
        locals.var_zsat = assign45780_e58805;
        locals.var_zsat_dn5 = assign45780_e58805_d_n5;
        locals.var_zsat_dn6 = assign45780_e58805_d_n6;
        locals.var_zsat_dn7 = assign45780_e58805_d_n7;
        locals.var_zsat_dn8 = assign45780_e58805_d_n8;
        locals.var_zsat_rv = 0.0;

        let (assign45790_e58820, assign45790_e58820_d_n5, assign45790_e58820_d_n6, assign45790_e58820_d_n7, assign45790_e58820_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45790_e58813: f64 = (2.0 * locals.var_zsat);
        let assign45790_e58814: f64 = (1.0 + assign45790_e58813);
        let assign45790_e58815: f64 = (assign45790_e58814).sqrt();
        let assign45790_e58816: f64 = (1.0 + assign45790_e58815);
        let assign45790_e58817: f64 = (locals.var_gmob_dl_dc * assign45790_e58816);
        let assign45790_e58818: f64 = (0.5 * assign45790_e58817);
        (assign45790_e58818, (0.5 * ((locals.var_gmob_dl_dc_dn5 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn5) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn6 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn6) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn7 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn7) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn8 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn8) / (2.0 * assign45790_e58815))))),)
    } else {
        (locals.var_gvsat, locals.var_gvsat_dn5, locals.var_gvsat_dn6, locals.var_gvsat_dn7, locals.var_gvsat_dn8,)
    }
};
        locals.var_gvsat = assign45790_e58820;
        locals.var_gvsat_dn5 = assign45790_e58820_d_n5;
        locals.var_gvsat_dn6 = assign45790_e58820_d_n6;
        locals.var_gvsat_dn7 = assign45790_e58820_d_n7;
        locals.var_gvsat_dn8 = assign45790_e58820_d_n8;
        locals.var_gvsat_rv = 0.0;

        let (assign45800_e58826, assign45800_e58826_d_n5, assign45800_e58826_d_n6, assign45800_e58826_d_n7, assign45800_e58826_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45800_e58824: f64 = (1.0 / locals.var_gvsat);
        (assign45800_e58824, (-(locals.var_gvsat_dn5 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn6 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn7 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn8 / (locals.var_gvsat * locals.var_gvsat))),)
    } else {
        (locals.var_gvsatinv_dc, locals.var_gvsatinv_dc_dn5, locals.var_gvsatinv_dc_dn6, locals.var_gvsatinv_dc_dn7, locals.var_gvsatinv_dc_dn8,)
    }
};
        locals.var_gvsatinv_dc = assign45800_e58826;
        locals.var_gvsatinv_dc_dn5 = assign45800_e58826_d_n5;
        locals.var_gvsatinv_dc_dn6 = assign45800_e58826_d_n6;
        locals.var_gvsatinv_dc_dn7 = assign45800_e58826_d_n7;
        locals.var_gvsatinv_dc_dn8 = assign45800_e58826_d_n8;
        locals.var_gvsatinv_dc_rv = 0.0;

        let (assign45810_e58832, assign45810_e58832_d_n5, assign45810_e58832_d_n6, assign45810_e58832_d_n7, assign45810_e58832_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45810_e58830: f64 = (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc);
        (assign45810_e58830, ((locals.var_gmob_dl_dc_dn5 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn5)), ((locals.var_gmob_dl_dc_dn6 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn6)), ((locals.var_gmob_dl_dc_dn7 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn7)), ((locals.var_gmob_dl_dc_dn8 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign45810_e58832;
        locals.var_temp__blk936_dn5 = assign45810_e58832_d_n5;
        locals.var_temp__blk936_dn6 = assign45810_e58832_d_n6;
        locals.var_temp__blk936_dn7 = assign45810_e58832_d_n7;
        locals.var_temp__blk936_dn8 = assign45810_e58832_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign45820_e58846, assign45820_e58846_d_n5, assign45820_e58846_d_n6, assign45820_e58846_d_n7, assign45820_e58846_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45820_e58839: f64 = (locals.var_zsat * locals.var_temp__blk936);
        let assign45820_e58841: f64 = (assign45820_e58839 * locals.var_temp__blk936);
        let assign45820_e58842: f64 = (0.5 * assign45820_e58841);
        let assign45820_e58843: f64 = (1.0 + assign45820_e58842);
        let assign45820_e58844: f64 = (locals.var_alpha_dc * assign45820_e58843);
        (assign45820_e58844, ((locals.var_alpha_dc_dn5 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn5 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn5))))), ((locals.var_alpha_dc_dn6 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn6 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn6))))), ((locals.var_alpha_dc_dn7 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn7 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn7))))), ((locals.var_alpha_dc_dn8 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn8 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn8))))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8,)
    }
};
        locals.var_alpha1 = assign45820_e58846;
        locals.var_alpha1_dn5 = assign45820_e58846_d_n5;
        locals.var_alpha1_dn6 = assign45820_e58846_d_n6;
        locals.var_alpha1_dn7 = assign45820_e58846_d_n7;
        locals.var_alpha1_dn8 = assign45820_e58846_d_n8;
        locals.var_alpha1_rv = 0.0;

        let (assign45830_e58854, assign45830_e58854_d_n5, assign45830_e58854_d_n6, assign45830_e58854_d_n7, assign45830_e58854_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45830_e58850: f64 = (locals.var_temp__blk936 * locals.var_qim1_dc);
        let assign45830_e58852: f64 = (assign45830_e58850 / locals.var_alpha1);
        (assign45830_e58852, (((((locals.var_temp__blk936_dn5 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn5)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn5)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn6 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn6)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn6)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn7 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn7)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn7)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn8 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn8)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn8)) / (locals.var_alpha1 * locals.var_alpha1)),)
    } else {
        (locals.var_h_dc, locals.var_h_dc_dn5, locals.var_h_dc_dn6, locals.var_h_dc_dn7, locals.var_h_dc_dn8,)
    }
};
        locals.var_h_dc = assign45830_e58854;
        locals.var_h_dc_dn5 = assign45830_e58854_d_n5;
        locals.var_h_dc_dn6 = assign45830_e58854_d_n6;
        locals.var_h_dc_dn7 = assign45830_e58854_d_n7;
        locals.var_h_dc_dn8 = assign45830_e58854_d_n8;
        locals.var_h_dc_rv = 0.0;

        let (assign45840_e58864, assign45840_e58864_d_n5, assign45840_e58864_d_n6, assign45840_e58864_d_n7, assign45840_e58864_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45840_e58858: f64 = (locals.var_bet_i * locals.var_qim1_dc);
        let assign45840_e58860: f64 = (assign45840_e58858 * locals.var_dps_dc);
        let assign45840_e58862: f64 = (assign45840_e58860 * locals.var_gvsatinv_dc);
        (assign45840_e58862, (((((locals.var_bet_i * locals.var_qim1_dc_dn5) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn5)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn5)), (((((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn6)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn6)), (((((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn7)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn7)), (((((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn8)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn8)),)
    } else {
        (locals.var_i_ds, locals.var_i_ds_dn5, locals.var_i_ds_dn6, locals.var_i_ds_dn7, locals.var_i_ds_dn8,)
    }
};
        locals.var_i_ds = assign45840_e58864;
        locals.var_i_ds_dn5 = assign45840_e58864_d_n5;
        locals.var_i_ds_dn6 = assign45840_e58864_d_n6;
        locals.var_i_ds_dn7 = assign45840_e58864_d_n7;
        locals.var_i_ds_dn8 = assign45840_e58864_d_n8;
        locals.var_i_ds_rv = 0.0;

        locals.var_xs_ov = 0.0;
        locals.var_xs_ov_dn5 = 0.0;
        locals.var_xs_ov_dn6 = 0.0;
        locals.var_xs_ov_dn7 = 0.0;
        locals.var_xs_ov_rv = 0.0;

        locals.var_xd_ov = 0.0;
        locals.var_xd_ov_dn5 = 0.0;
        locals.var_xd_ov_dn6 = 0.0;
        locals.var_xd_ov_dn7 = 0.0;
        locals.var_xd_ov_rv = 0.0;

        locals.var_vovs = 0.0;
        locals.var_vovs_dn5 = 0.0;
        locals.var_vovs_dn6 = 0.0;
        locals.var_vovs_dn7 = 0.0;
        locals.var_vovs_rv = 0.0;

        locals.var_vovd = 0.0;
        locals.var_vovd_dn5 = 0.0;
        locals.var_vovd_dn6 = 0.0;
        locals.var_vovd_dn7 = 0.0;
        locals.var_vovd_rv = 0.0;

        let assign45890_e58899: f64 = if (((((p.p40 != 0.0) && ((locals.var_igov_i > 0.0) || (locals.var_igovd_i > 0.0))) || ((p.p42 != 0.0) && ((locals.var_agidl_i > 0.0) || (locals.var_agidld_i > 0.0)))) || (locals.var_cgov_i > 0.0)) || (locals.var_cgovd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign45890_e58899;
        locals.var_guard1220_rv = 0.0;

        let (assign45900_e58912, assign45900_e58912_d_n5, assign45900_e58912_d_n6, assign45900_e58912_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45900_e58905: f64 = (locals.var_xgs_ov * locals.var_xgs_ov);
        let assign45900_e58907: f64 = (assign45900_e58905 + locals.var_sp_ov_eps2_s);
        let assign45900_e58908: f64 = (assign45900_e58907).sqrt();
        let assign45900_e58909: f64 = (locals.var_xgs_ov + assign45900_e58908);
        let assign45900_e58910: f64 = (0.5 * assign45900_e58909);
        (assign45900_e58910, (0.5 * (locals.var_xgs_ov_dn5 + (((locals.var_xgs_ov_dn5 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn5)) / (2.0 * assign45900_e58908)))), (0.5 * (locals.var_xgs_ov_dn6 + (((locals.var_xgs_ov_dn6 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn6)) / (2.0 * assign45900_e58908)))), (0.5 * (locals.var_xgs_ov_dn7 + (((locals.var_xgs_ov_dn7 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn7)) / (2.0 * assign45900_e58908)))),)
    } else {
        (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn5, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7,)
    }
};
        locals.var_sp_ov_xg = assign45900_e58912;
        locals.var_sp_ov_xg_dn5 = assign45900_e58912_d_n5;
        locals.var_sp_ov_xg_dn6 = assign45900_e58912_d_n6;
        locals.var_sp_ov_xg_dn7 = assign45900_e58912_d_n7;
        locals.var_sp_ov_xg_rv = 0.0;

        let (assign45910_e58934, assign45910_e58934_d_n5, assign45910_e58934_d_n6, assign45910_e58934_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45910_e58915: f64 = (-locals.var_sp_ov_xg);
        let assign45910_e58918: f64 = (locals.var_gov2_s * 0.5);
        let assign45910_e58919: f64 = (assign45910_e58915 - assign45910_e58918);
        let assign45910_e58924: f64 = (locals.var_gov2_s * 0.25);
        let assign45910_e58925: f64 = (locals.var_sp_ov_xg + assign45910_e58924);
        let assign45910_e58927: f64 = (assign45910_e58925 + locals.var_sp_ov_a_s);
        let assign45910_e58928: f64 = (assign45910_e58927).sqrt();
        let assign45910_e58929: f64 = (locals.var_gov_s * assign45910_e58928);
        let assign45910_e58930: f64 = (assign45910_e58919 + assign45910_e58929);
        let assign45910_e58932: f64 = (assign45910_e58930 + locals.var_sp_ov_delta1_s);
        (assign45910_e58932, ((-locals.var_sp_ov_xg_dn5) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn5 / (2.0 * assign45910_e58928)))), ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn6 / (2.0 * assign45910_e58928)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn7 / (2.0 * assign45910_e58928)))),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7,)
    }
};
        locals.var_xs_ov = assign45910_e58934;
        locals.var_xs_ov_dn5 = assign45910_e58934_d_n5;
        locals.var_xs_ov_dn6 = assign45910_e58934_d_n6;
        locals.var_xs_ov_dn7 = assign45910_e58934_d_n7;
        locals.var_xs_ov_rv = 0.0;

        let (assign45920_e58947, assign45920_e58947_d_n5, assign45920_e58947_d_n6, assign45920_e58947_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45920_e58940: f64 = (locals.var_xgd_ov * locals.var_xgd_ov);
        let assign45920_e58942: f64 = (assign45920_e58940 + locals.var_sp_ov_eps2_d);
        let assign45920_e58943: f64 = (assign45920_e58942).sqrt();
        let assign45920_e58944: f64 = (locals.var_xgd_ov + assign45920_e58943);
        let assign45920_e58945: f64 = (0.5 * assign45920_e58944);
        (assign45920_e58945, (0.5 * (locals.var_xgd_ov_dn5 + (((locals.var_xgd_ov_dn5 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn5)) / (2.0 * assign45920_e58943)))), (0.5 * (locals.var_xgd_ov_dn6 + (((locals.var_xgd_ov_dn6 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn6)) / (2.0 * assign45920_e58943)))), (0.5 * (locals.var_xgd_ov_dn7 + (((locals.var_xgd_ov_dn7 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn7)) / (2.0 * assign45920_e58943)))),)
    } else {
        (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn5, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7,)
    }
};
        locals.var_sp_ov_xg = assign45920_e58947;
        locals.var_sp_ov_xg_dn5 = assign45920_e58947_d_n5;
        locals.var_sp_ov_xg_dn6 = assign45920_e58947_d_n6;
        locals.var_sp_ov_xg_dn7 = assign45920_e58947_d_n7;
        locals.var_sp_ov_xg_rv = 0.0;

        let (assign45930_e58969, assign45930_e58969_d_n5, assign45930_e58969_d_n6, assign45930_e58969_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45930_e58950: f64 = (-locals.var_sp_ov_xg);
        let assign45930_e58953: f64 = (locals.var_gov2_d * 0.5);
        let assign45930_e58954: f64 = (assign45930_e58950 - assign45930_e58953);
        let assign45930_e58959: f64 = (locals.var_gov2_d * 0.25);
        let assign45930_e58960: f64 = (locals.var_sp_ov_xg + assign45930_e58959);
        let assign45930_e58962: f64 = (assign45930_e58960 + locals.var_sp_ov_a_d);
        let assign45930_e58963: f64 = (assign45930_e58962).sqrt();
        let assign45930_e58964: f64 = (locals.var_gov_d * assign45930_e58963);
        let assign45930_e58965: f64 = (assign45930_e58954 + assign45930_e58964);
        let assign45930_e58967: f64 = (assign45930_e58965 + locals.var_sp_ov_delta1_d);
        (assign45930_e58967, ((-locals.var_sp_ov_xg_dn5) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn5 / (2.0 * assign45930_e58963)))), ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn6 / (2.0 * assign45930_e58963)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn7 / (2.0 * assign45930_e58963)))),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7,)
    }
};
        locals.var_xd_ov = assign45930_e58969;
        locals.var_xd_ov_dn5 = assign45930_e58969_d_n5;
        locals.var_xd_ov_dn6 = assign45930_e58969_d_n6;
        locals.var_xd_ov_dn7 = assign45930_e58969_d_n7;
        locals.var_xd_ov_rv = 0.0;

        let (assign45940_e58978, assign45940_e58978_d_n5, assign45940_e58978_d_n6, assign45940_e58978_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45940_e58972: f64 = (-locals.var_phita);
        let assign45940_e58975: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
        let assign45940_e58976: f64 = (assign45940_e58972 * assign45940_e58975);
        (assign45940_e58976, (assign45940_e58972 * (locals.var_xgs_ov_dn5 + locals.var_xs_ov_dn5)), (assign45940_e58972 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)), (assign45940_e58972 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)),)
    } else {
        (locals.var_vovs, locals.var_vovs_dn5, locals.var_vovs_dn6, locals.var_vovs_dn7,)
    }
};
        locals.var_vovs = assign45940_e58978;
        locals.var_vovs_dn5 = assign45940_e58978_d_n5;
        locals.var_vovs_dn6 = assign45940_e58978_d_n6;
        locals.var_vovs_dn7 = assign45940_e58978_d_n7;
        locals.var_vovs_rv = 0.0;

        let (assign45950_e58987, assign45950_e58987_d_n5, assign45950_e58987_d_n6, assign45950_e58987_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45950_e58981: f64 = (-locals.var_phita);
        let assign45950_e58984: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
        let assign45950_e58985: f64 = (assign45950_e58981 * assign45950_e58984);
        (assign45950_e58985, (assign45950_e58981 * (locals.var_xgd_ov_dn5 + locals.var_xd_ov_dn5)), (assign45950_e58981 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)), (assign45950_e58981 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)),)
    } else {
        (locals.var_vovd, locals.var_vovd_dn5, locals.var_vovd_dn6, locals.var_vovd_dn7,)
    }
};
        locals.var_vovd = assign45950_e58987;
        locals.var_vovd_dn5 = assign45950_e58987_d_n5;
        locals.var_vovd_dn6 = assign45950_e58987_d_n6;
        locals.var_vovd_dn7 = assign45950_e58987_d_n7;
        locals.var_vovd_rv = 0.0;

        let assign46020_e58996: f64 = if p.p40 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign46020_e58996;
        locals.var_guard1221_rv = 0.0;

        let assign46030_e58999: f64 = if locals.var_igov_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign46030_e58999;
        locals.var_guard1222_rv = 0.0;

        let (assign46040_e59012, assign46040_e59012_d_n5, assign46040_e59012_d_n6, assign46040_e59012_d_n7, assign46040_e59012_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46040_e59005: f64 = (locals.var_vovs * locals.var_vovs);
        let assign46040_e59007: f64 = (assign46040_e59005 + 1e-6);
        let assign46040_e59008: f64 = (assign46040_e59007).sqrt();
        let assign46040_e59010: f64 = (assign46040_e59008 * locals.var_inv_chib);
        (assign46040_e59010, ((((locals.var_vovs_dn5 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn5)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), 0.0,)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46040_e59012;
        locals.var_zg_dn5 = assign46040_e59012_d_n5;
        locals.var_zg_dn6 = assign46040_e59012_d_n6;
        locals.var_zg_dn7 = assign46040_e59012_d_n7;
        locals.var_zg_dn8 = assign46040_e59012_d_n8;
        locals.var_zg_rv = 0.0;

        let assign46050_e59015: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign46050_e59015;
        locals.var_guard1223_rv = 0.0;

        let (assign46060_e59038, assign46060_e59038_d_n5, assign46060_e59038_d_n6, assign46060_e59038_d_n7, assign46060_e59038_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign46060_e59024: f64 = (locals.var_zg + locals.var_gcqov);
        let assign46060_e59027: f64 = (locals.var_zg - locals.var_gcqov);
        let assign46060_e59030: f64 = (locals.var_zg - locals.var_gcqov);
        let assign46060_e59031: f64 = (assign46060_e59027 * assign46060_e59030);
        let assign46060_e59033: f64 = (assign46060_e59031 + 1e-6);
        let assign46060_e59034: f64 = (assign46060_e59033).sqrt();
        let assign46060_e59035: f64 = (assign46060_e59024 - assign46060_e59034);
        let assign46060_e59036: f64 = (0.5 * assign46060_e59035);
        (assign46060_e59036, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn5)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn6)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn7)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn8)) / (2.0 * assign46060_e59034)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46060_e59038;
        locals.var_zg_dn5 = assign46060_e59038_d_n5;
        locals.var_zg_dn6 = assign46060_e59038_d_n6;
        locals.var_zg_dn7 = assign46060_e59038_d_n7;
        locals.var_zg_dn8 = assign46060_e59038_d_n8;
        locals.var_zg_rv = 0.0;

        let (assign46070_e59055, assign46070_e59055_d_n5, assign46070_e59055_d_n6, assign46070_e59055_d_n7, assign46070_e59055_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46070_e59044: f64 = (-1.5);
        let assign46070_e59049: f64 = (locals.var_gc3ov_i * locals.var_zg);
        let assign46070_e59050: f64 = (locals.var_gc2ov_i + assign46070_e59049);
        let assign46070_e59051: f64 = (locals.var_zg * assign46070_e59050);
        let assign46070_e59052: f64 = (assign46070_e59044 + assign46070_e59051);
        let assign46070_e59053: f64 = (locals.var_bov * assign46070_e59052);
        (assign46070_e59053, (locals.var_bov * ((locals.var_zg_dn5 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn5)))), (locals.var_bov * ((locals.var_zg_dn6 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn6)))), (locals.var_bov * ((locals.var_zg_dn7 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn7)))), (locals.var_bov * ((locals.var_zg_dn8 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46070_e59055;
        locals.var_temp__blk936_dn5 = assign46070_e59055_d_n5;
        locals.var_temp__blk936_dn6 = assign46070_e59055_d_n6;
        locals.var_temp__blk936_dn7 = assign46070_e59055_d_n7;
        locals.var_temp__blk936_dn8 = assign46070_e59055_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46130_e59141, assign46130_e59141_d_n5, assign46130_e59141_d_n6, assign46130_e59141_d_n7,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46130_e59139: f64 = (3.0 + locals.var_xs_ov);
        (assign46130_e59139, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7,)
    } else {
        (locals.var_fs1, locals.var_fs1_dn5, locals.var_fs1_dn6, locals.var_fs1_dn7,)
    }
};
        locals.var_fs1 = assign46130_e59141;
        locals.var_fs1_dn5 = assign46130_e59141_d_n5;
        locals.var_fs1_dn6 = assign46130_e59141_d_n6;
        locals.var_fs1_dn7 = assign46130_e59141_d_n7;
        locals.var_fs1_rv = 0.0;

        let (assign46140_e59150,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46140_e59146: f64 = (-3.0);
        let assign46140_e59148: f64 = (assign46140_e59146 - locals.var_gco_i);
        (assign46140_e59148,)
    } else {
        (locals.var_fs2,)
    }
};
        locals.var_fs2 = assign46140_e59150;
        locals.var_fs2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        locals: &mut StampLocals,
    ) {
        let (assign46150_e59158, assign46150_e59158_d_n5, assign46150_e59158_d_n6, assign46150_e59158_d_n7,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46150_e59156: f64 = (30.0 * locals.var_vgsprime);
        (assign46150_e59156, (30.0 * locals.var_vgsprime_dn5), (30.0 * locals.var_vgsprime_dn6), (30.0 * locals.var_vgsprime_dn7),)
    } else {
        (locals.var_fs3, locals.var_fs3_dn5, locals.var_fs3_dn6, locals.var_fs3_dn7,)
    }
};
        locals.var_fs3 = assign46150_e59158;
        locals.var_fs3_dn5 = assign46150_e59158_d_n5;
        locals.var_fs3_dn6 = assign46150_e59158_d_n6;
        locals.var_fs3_dn7 = assign46150_e59158_d_n7;
        locals.var_fs3_rv = 0.0;

        let (assign46160_e59166,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46160_e59164: f64 = (4.0 - 0.9);
        (assign46160_e59164,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46160_e59166;
        locals.var_tme1_rv = 0.0;

        let (assign46170_e59174, assign46170_e59174_d_n5, assign46170_e59174_d_n6, assign46170_e59174_d_n7, assign46170_e59174_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46170_e59172: f64 = (locals.var_fs1 + locals.var_fs3);
        (assign46170_e59172, (locals.var_fs1_dn5 + locals.var_fs3_dn5), (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), 0.0,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8,)
    }
};
        locals.var_tme2 = assign46170_e59174;
        locals.var_tme2_dn5 = assign46170_e59174_d_n5;
        locals.var_tme2_dn6 = assign46170_e59174_d_n6;
        locals.var_tme2_dn7 = assign46170_e59174_d_n7;
        locals.var_tme2_dn8 = assign46170_e59174_d_n8;
        locals.var_tme2_rv = 0.0;

        let (assign46180_e59195, assign46180_e59195_d_n5, assign46180_e59195_d_n6, assign46180_e59195_d_n7, assign46180_e59195_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46180_e59180: f64 = (2.0 / locals.var_tme1);
        let assign46180_e59184: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46180_e59187: f64 = (locals.var_tme1 * locals.var_fs1);
        let assign46180_e59189: f64 = (assign46180_e59187 * locals.var_fs3);
        let assign46180_e59190: f64 = (assign46180_e59184 - assign46180_e59189);
        let assign46180_e59191: f64 = (assign46180_e59190).sqrt();
        let assign46180_e59192: f64 = (locals.var_tme2 - assign46180_e59191);
        let assign46180_e59193: f64 = (assign46180_e59180 * assign46180_e59192);
        (assign46180_e59193, (assign46180_e59180 * (locals.var_tme2_dn5 - ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (((locals.var_tme1 * locals.var_fs1_dn5) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn5))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn6))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn7))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn8 - (((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) / (2.0 * assign46180_e59191)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46180_e59195;
        locals.var_temp__blk936_dn5 = assign46180_e59195_d_n5;
        locals.var_temp__blk936_dn6 = assign46180_e59195_d_n6;
        locals.var_temp__blk936_dn7 = assign46180_e59195_d_n7;
        locals.var_temp__blk936_dn8 = assign46180_e59195_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46190_e59203,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46190_e59201: f64 = (4.0 - 0.3);
        (assign46190_e59201,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46190_e59203;
        locals.var_tme1_rv = 0.0;

        let (assign46200_e59211, assign46200_e59211_d_n5, assign46200_e59211_d_n6, assign46200_e59211_d_n7, assign46200_e59211_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46200_e59209: f64 = (locals.var_fs2 + locals.var_temp__blk936);
        (assign46200_e59209, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8,)
    }
};
        locals.var_tme2 = assign46200_e59211;
        locals.var_tme2_dn5 = assign46200_e59211_d_n5;
        locals.var_tme2_dn6 = assign46200_e59211_d_n6;
        locals.var_tme2_dn7 = assign46200_e59211_d_n7;
        locals.var_tme2_dn8 = assign46200_e59211_d_n8;
        locals.var_tme2_rv = 0.0;

        let assign46230_e59245: f64 = if locals.var_igovd_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign46230_e59245;
        locals.var_guard1226_rv = 0.0;

        let (assign46240_e59258, assign46240_e59258_d_n5, assign46240_e59258_d_n6, assign46240_e59258_d_n7, assign46240_e59258_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46240_e59251: f64 = (locals.var_vovd * locals.var_vovd);
        let assign46240_e59253: f64 = (assign46240_e59251 + 1e-6);
        let assign46240_e59254: f64 = (assign46240_e59253).sqrt();
        let assign46240_e59256: f64 = (assign46240_e59254 * locals.var_inv_chib);
        (assign46240_e59256, ((((locals.var_vovd_dn5 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn5)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), 0.0,)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46240_e59258;
        locals.var_zg_dn5 = assign46240_e59258_d_n5;
        locals.var_zg_dn6 = assign46240_e59258_d_n6;
        locals.var_zg_dn7 = assign46240_e59258_d_n7;
        locals.var_zg_dn8 = assign46240_e59258_d_n8;
        locals.var_zg_rv = 0.0;

        let assign46250_e59261: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign46250_e59261;
        locals.var_guard1227_rv = 0.0;

        let (assign46260_e59284, assign46260_e59284_d_n5, assign46260_e59284_d_n6, assign46260_e59284_d_n7, assign46260_e59284_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign46260_e59270: f64 = (locals.var_zg + locals.var_gcqovd);
        let assign46260_e59273: f64 = (locals.var_zg - locals.var_gcqovd);
        let assign46260_e59276: f64 = (locals.var_zg - locals.var_gcqovd);
        let assign46260_e59277: f64 = (assign46260_e59273 * assign46260_e59276);
        let assign46260_e59279: f64 = (assign46260_e59277 + 1e-6);
        let assign46260_e59280: f64 = (assign46260_e59279).sqrt();
        let assign46260_e59281: f64 = (assign46260_e59270 - assign46260_e59280);
        let assign46260_e59282: f64 = (0.5 * assign46260_e59281);
        (assign46260_e59282, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn5)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn6)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn7)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn8)) / (2.0 * assign46260_e59280)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46260_e59284;
        locals.var_zg_dn5 = assign46260_e59284_d_n5;
        locals.var_zg_dn6 = assign46260_e59284_d_n6;
        locals.var_zg_dn7 = assign46260_e59284_d_n7;
        locals.var_zg_dn8 = assign46260_e59284_d_n8;
        locals.var_zg_rv = 0.0;

        let (assign46270_e59301, assign46270_e59301_d_n5, assign46270_e59301_d_n6, assign46270_e59301_d_n7, assign46270_e59301_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46270_e59290: f64 = (-1.5);
        let assign46270_e59295: f64 = (locals.var_gc3ovd_i * locals.var_zg);
        let assign46270_e59296: f64 = (locals.var_gc2ovd_i + assign46270_e59295);
        let assign46270_e59297: f64 = (locals.var_zg * assign46270_e59296);
        let assign46270_e59298: f64 = (assign46270_e59290 + assign46270_e59297);
        let assign46270_e59299: f64 = (locals.var_bov_d * assign46270_e59298);
        (assign46270_e59299, (locals.var_bov_d * ((locals.var_zg_dn5 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn5)))), (locals.var_bov_d * ((locals.var_zg_dn6 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn6)))), (locals.var_bov_d * ((locals.var_zg_dn7 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn7)))), (locals.var_bov_d * ((locals.var_zg_dn8 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46270_e59301;
        locals.var_temp__blk936_dn5 = assign46270_e59301_d_n5;
        locals.var_temp__blk936_dn6 = assign46270_e59301_d_n6;
        locals.var_temp__blk936_dn7 = assign46270_e59301_d_n7;
        locals.var_temp__blk936_dn8 = assign46270_e59301_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46330_e59387, assign46330_e59387_d_n5, assign46330_e59387_d_n6, assign46330_e59387_d_n7,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46330_e59385: f64 = (3.0 + locals.var_xd_ov);
        (assign46330_e59385, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7,)
    } else {
        (locals.var_fs1, locals.var_fs1_dn5, locals.var_fs1_dn6, locals.var_fs1_dn7,)
    }
};
        locals.var_fs1 = assign46330_e59387;
        locals.var_fs1_dn5 = assign46330_e59387_d_n5;
        locals.var_fs1_dn6 = assign46330_e59387_d_n6;
        locals.var_fs1_dn7 = assign46330_e59387_d_n7;
        locals.var_fs1_rv = 0.0;

        let (assign46340_e59396,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46340_e59392: f64 = (-3.0);
        let assign46340_e59394: f64 = (assign46340_e59392 - locals.var_gco_i);
        (assign46340_e59394,)
    } else {
        (locals.var_fs2,)
    }
};
        locals.var_fs2 = assign46340_e59396;
        locals.var_fs2_rv = 0.0;

        let (assign46350_e59404, assign46350_e59404_d_n5, assign46350_e59404_d_n6, assign46350_e59404_d_n7,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46350_e59402: f64 = (30.0 * locals.var_vgdprime);
        (assign46350_e59402, (30.0 * locals.var_vgdprime_dn5), (30.0 * locals.var_vgdprime_dn6), (30.0 * locals.var_vgdprime_dn7),)
    } else {
        (locals.var_fs3, locals.var_fs3_dn5, locals.var_fs3_dn6, locals.var_fs3_dn7,)
    }
};
        locals.var_fs3 = assign46350_e59404;
        locals.var_fs3_dn5 = assign46350_e59404_d_n5;
        locals.var_fs3_dn6 = assign46350_e59404_d_n6;
        locals.var_fs3_dn7 = assign46350_e59404_d_n7;
        locals.var_fs3_rv = 0.0;

        let (assign46360_e59412,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46360_e59410: f64 = (4.0 - 0.9);
        (assign46360_e59410,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46360_e59412;
        locals.var_tme1_rv = 0.0;

        let (assign46370_e59420, assign46370_e59420_d_n5, assign46370_e59420_d_n6, assign46370_e59420_d_n7, assign46370_e59420_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46370_e59418: f64 = (locals.var_fs1 + locals.var_fs3);
        (assign46370_e59418, (locals.var_fs1_dn5 + locals.var_fs3_dn5), (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), 0.0,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8,)
    }
};
        locals.var_tme2 = assign46370_e59420;
        locals.var_tme2_dn5 = assign46370_e59420_d_n5;
        locals.var_tme2_dn6 = assign46370_e59420_d_n6;
        locals.var_tme2_dn7 = assign46370_e59420_d_n7;
        locals.var_tme2_dn8 = assign46370_e59420_d_n8;
        locals.var_tme2_rv = 0.0;

        let (assign46380_e59441, assign46380_e59441_d_n5, assign46380_e59441_d_n6, assign46380_e59441_d_n7, assign46380_e59441_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46380_e59426: f64 = (2.0 / locals.var_tme1);
        let assign46380_e59430: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46380_e59433: f64 = (locals.var_tme1 * locals.var_fs1);
        let assign46380_e59435: f64 = (assign46380_e59433 * locals.var_fs3);
        let assign46380_e59436: f64 = (assign46380_e59430 - assign46380_e59435);
        let assign46380_e59437: f64 = (assign46380_e59436).sqrt();
        let assign46380_e59438: f64 = (locals.var_tme2 - assign46380_e59437);
        let assign46380_e59439: f64 = (assign46380_e59426 * assign46380_e59438);
        (assign46380_e59439, (assign46380_e59426 * (locals.var_tme2_dn5 - ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (((locals.var_tme1 * locals.var_fs1_dn5) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn5))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn6))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn7))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn8 - (((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) / (2.0 * assign46380_e59437)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46380_e59441;
        locals.var_temp__blk936_dn5 = assign46380_e59441_d_n5;
        locals.var_temp__blk936_dn6 = assign46380_e59441_d_n6;
        locals.var_temp__blk936_dn7 = assign46380_e59441_d_n7;
        locals.var_temp__blk936_dn8 = assign46380_e59441_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46390_e59449,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46390_e59447: f64 = (4.0 - 0.3);
        (assign46390_e59447,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46390_e59449;
        locals.var_tme1_rv = 0.0;

        let (assign46400_e59457, assign46400_e59457_d_n5, assign46400_e59457_d_n6, assign46400_e59457_d_n7, assign46400_e59457_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46400_e59455: f64 = (locals.var_fs2 + locals.var_temp__blk936);
        (assign46400_e59455, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8,)
    }
};
        locals.var_tme2 = assign46400_e59457;
        locals.var_tme2_dn5 = assign46400_e59457_d_n5;
        locals.var_tme2_dn6 = assign46400_e59457_d_n6;
        locals.var_tme2_dn7 = assign46400_e59457_d_n7;
        locals.var_tme2_dn8 = assign46400_e59457_d_n8;
        locals.var_tme2_rv = 0.0;

        let assign46430_e59491: f64 = if locals.var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign46430_e59491;
        locals.var_guard1230_rv = 0.0;

        let assign46440_e59494: f64 = if locals.var_xg_dc <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign46440_e59494;
        locals.var_guard1231_rv = 0.0;

        let (assign46450_e59504, assign46450_e59504_d_n5, assign46450_e59504_d_n6, assign46450_e59504_d_n7, assign46450_e59504_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46450_e59502: f64 = (1.0 + locals.var_ar);
        (assign46450_e59502, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46450_e59504;
        locals.var_temp__blk936_dn5 = assign46450_e59504_d_n5;
        locals.var_temp__blk936_dn6 = assign46450_e59504_d_n6;
        locals.var_temp__blk936_dn7 = assign46450_e59504_d_n7;
        locals.var_temp__blk936_dn8 = assign46450_e59504_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46460_e59517, assign46460_e59517_d_n5, assign46460_e59517_d_n6, assign46460_e59517_d_n7, assign46460_e59517_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46460_e59511: f64 = (locals.var_temp__blk936).sqrt();
        let assign46460_e59513: f64 = (assign46460_e59511 * locals.var_v_ds);
        let assign46460_e59515: f64 = (assign46460_e59513 / locals.var_vdsat_lim_dc);
        (assign46460_e59515, (((((locals.var_temp__blk936_dn5 / (2.0 * assign46460_e59511)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn5)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk936_dn6 / (2.0 * assign46460_e59511)) * locals.var_v_ds) + (assign46460_e59511 * locals.var_v_ds_dn6)) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn6)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk936_dn7 / (2.0 * assign46460_e59511)) * locals.var_v_ds) + (assign46460_e59511 * locals.var_v_ds_dn7)) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn7)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk936_dn8 / (2.0 * assign46460_e59511)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn8)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign46460_e59517;
        locals.var_temp1_dn5 = assign46460_e59517_d_n5;
        locals.var_temp1_dn6 = assign46460_e59517_d_n6;
        locals.var_temp1_dn7 = assign46460_e59517_d_n7;
        locals.var_temp1_dn8 = assign46460_e59517_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign46470_e59529, assign46470_e59529_d_n5, assign46470_e59529_d_n6, assign46470_e59529_d_n7, assign46470_e59529_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46470_e59525: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign46470_e59527: f64 = (assign46470_e59525 + locals.var_temp__blk936);
        (assign46470_e59527, (((locals.var_temp1_dn5 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn5)) + locals.var_temp__blk936_dn5), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk936_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk936_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign46470_e59529;
        locals.var_temp2_dn5 = assign46470_e59529_d_n5;
        locals.var_temp2_dn6 = assign46470_e59529_d_n6;
        locals.var_temp2_dn7 = assign46470_e59529_d_n7;
        locals.var_temp2_dn8 = assign46470_e59529_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign46480_e59539, assign46480_e59539_d_n5, assign46480_e59539_d_n6, assign46480_e59539_d_n7, assign46480_e59539_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46480_e59537: f64 = (2.0 * locals.var_temp1);
        (assign46480_e59537, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46480_e59539;
        locals.var_temp__blk936_dn5 = assign46480_e59539_d_n5;
        locals.var_temp__blk936_dn6 = assign46480_e59539_d_n6;
        locals.var_temp__blk936_dn7 = assign46480_e59539_d_n7;
        locals.var_temp__blk936_dn8 = assign46480_e59539_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46490_e59561, assign46490_e59561_d_n5, assign46490_e59561_d_n6, assign46490_e59561_d_n7, assign46490_e59561_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46490_e59547: f64 = (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc);
        let assign46490_e59549: f64 = (assign46490_e59547 * locals.var_temp__blk936);
        let assign46490_e59552: f64 = (locals.var_temp2 - locals.var_temp__blk936);
        let assign46490_e59553: f64 = (assign46490_e59552).sqrt();
        let assign46490_e59556: f64 = (locals.var_temp2 + locals.var_temp__blk936);
        let assign46490_e59557: f64 = (assign46490_e59556).sqrt();
        let assign46490_e59558: f64 = (assign46490_e59553 + assign46490_e59557);
        let assign46490_e59559: f64 = (assign46490_e59549 / assign46490_e59558);
        (assign46490_e59559, (((((((locals.var_vdsat_lim_dc_dn5 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn5)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn5)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn5 - locals.var_temp__blk936_dn5) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn5 + locals.var_temp__blk936_dn5) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn6 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn6)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn6)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn6 - locals.var_temp__blk936_dn6) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn6 + locals.var_temp__blk936_dn6) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn7 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn7)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn7)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn7 - locals.var_temp__blk936_dn7) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn7 + locals.var_temp__blk936_dn7) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn8 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn8)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn8)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn8 - locals.var_temp__blk936_dn8) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn8 + locals.var_temp__blk936_dn8) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)),)
    } else {
        (locals.var_udse_dc, locals.var_udse_dc_dn5, locals.var_udse_dc_dn6, locals.var_udse_dc_dn7, locals.var_udse_dc_dn8,)
    }
};
        locals.var_udse_dc = assign46490_e59561;
        locals.var_udse_dc_dn5 = assign46490_e59561_d_n5;
        locals.var_udse_dc_dn6 = assign46490_e59561_d_n6;
        locals.var_udse_dc_dn7 = assign46490_e59561_d_n7;
        locals.var_udse_dc_dn8 = assign46490_e59561_d_n8;
        locals.var_udse_dc_rv = 0.0;

        let assign46500_e59564: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46500_e59566: f64 = (-230.25850929940458);
        let assign46500_e59567: f64 = if assign46500_e59564 > assign46500_e59566 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign46500_e59567;
        locals.var_guard1232_rv = 0.0;

        let (assign46510_e59578, assign46510_e59578_d_n5, assign46510_e59578_d_n6, assign46510_e59578_d_n7, assign46510_e59578_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign46510_e59575: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46510_e59576: f64 = (assign46510_e59575).exp();
        (assign46510_e59576, (assign46510_e59576 * (locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)), (assign46510_e59576 * (locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)), (assign46510_e59576 * (locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)), (assign46510_e59576 * (locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46510_e59578;
        locals.var_temp__blk936_dn5 = assign46510_e59578_d_n5;
        locals.var_temp__blk936_dn6 = assign46510_e59578_d_n6;
        locals.var_temp__blk936_dn7 = assign46510_e59578_d_n7;
        locals.var_temp__blk936_dn8 = assign46510_e59578_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46520_e59618, assign46520_e59618_d_n5, assign46520_e59618_d_n6, assign46520_e59618_d_n7, assign46520_e59618_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1232 == 0.0)) {
        let assign46520_e59588: f64 = (-230.25850929940458);
        let assign46520_e59591: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46520_e59592: f64 = (assign46520_e59588 - assign46520_e59591);
        let assign46520_e59596: f64 = (-230.25850929940458);
        let assign46520_e59599: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46520_e59600: f64 = (assign46520_e59596 - assign46520_e59599);
        let assign46520_e59603: f64 = (-230.25850929940458);
        let assign46520_e59606: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46520_e59607: f64 = (assign46520_e59603 - assign46520_e59606);
        let assign46520_e59609: f64 = (assign46520_e59607 * 0.3333333333333333);
        let assign46520_e59610: f64 = (1.0 + assign46520_e59609);
        let assign46520_e59611: f64 = (assign46520_e59600 * assign46520_e59610);
        let assign46520_e59612: f64 = (0.5 * assign46520_e59611);
        let assign46520_e59613: f64 = (1.0 + assign46520_e59612);
        let assign46520_e59614: f64 = (assign46520_e59592 * assign46520_e59613);
        let assign46520_e59615: f64 = (1.0 + assign46520_e59614);
        let assign46520_e59616: f64 = (1e-100 / assign46520_e59615);
        (assign46520_e59616, (-((1e-100 * (((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46520_e59618;
        locals.var_temp__blk936_dn5 = assign46520_e59618_d_n5;
        locals.var_temp__blk936_dn6 = assign46520_e59618_d_n6;
        locals.var_temp__blk936_dn7 = assign46520_e59618_d_n7;
        locals.var_temp__blk936_dn8 = assign46520_e59618_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46530_e59637, assign46530_e59637_d_n5, assign46530_e59637_d_n6, assign46530_e59637_d_n7, assign46530_e59637_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46530_e59626: f64 = (0.5 * locals.var_x_ds_dc);
        let assign46530_e59630: f64 = (1.0 + locals.var_temp__blk936);
        let assign46530_e59631: f64 = (0.5 * assign46530_e59630);
        let assign46530_e59632: f64 = (assign46530_e59631).ln();
        let assign46530_e59633: f64 = (assign46530_e59626 - assign46530_e59632);
        let assign46530_e59634: f64 = (locals.var_phit1_dc * assign46530_e59633);
        let assign46530_e59635: f64 = (locals.var_vsbstar_dc + assign46530_e59634);
        (assign46530_e59635, (locals.var_vsbstar_dc_dn5 + ((locals.var_phit1_dc_dn5 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn5) - ((0.5 * locals.var_temp__blk936_dn5) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn6 + ((locals.var_phit1_dc_dn6 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn6) - ((0.5 * locals.var_temp__blk936_dn6) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn7 + ((locals.var_phit1_dc_dn7 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn7) - ((0.5 * locals.var_temp__blk936_dn7) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn8 + ((locals.var_phit1_dc_dn8 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn8) - ((0.5 * locals.var_temp__blk936_dn8) / assign46530_e59631))))),)
    } else {
        (locals.var_vm, locals.var_vm_dn5, locals.var_vm_dn6, locals.var_vm_dn7, locals.var_vm_dn8,)
    }
};
        locals.var_vm = assign46530_e59637;
        locals.var_vm_dn5 = assign46530_e59637_d_n5;
        locals.var_vm_dn6 = assign46530_e59637_d_n6;
        locals.var_vm_dn7 = assign46530_e59637_d_n7;
        locals.var_vm_dn8 = assign46530_e59637_d_n8;
        locals.var_vm_rv = 0.0;

        let (assign46540_e59645, assign46540_e59645_d_n5, assign46540_e59645_d_n6, assign46540_e59645_d_n7, assign46540_e59645_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46540_e59643: f64 = (locals.var_gco_i * locals.var_phit1_dc);
        (assign46540_e59643, (locals.var_gco_i * locals.var_phit1_dc_dn5), (locals.var_gco_i * locals.var_phit1_dc_dn6), (locals.var_gco_i * locals.var_phit1_dc_dn7), (locals.var_gco_i * locals.var_phit1_dc_dn8),)
    } else {
        (locals.var_dch, locals.var_dch_dn5, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8,)
    }
};
        locals.var_dch = assign46540_e59645;
        locals.var_dch_dn5 = assign46540_e59645_d_n5;
        locals.var_dch_dn6 = assign46540_e59645_d_n6;
        locals.var_dch_dn7 = assign46540_e59645_d_n7;
        locals.var_dch_dn8 = assign46540_e59645_d_n8;
        locals.var_dch_rv = 0.0;

        let (assign46550_e59653, assign46550_e59653_d_n5, assign46550_e59653_d_n6, assign46550_e59653_d_n7, assign46550_e59653_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46550_e59651: f64 = (locals.var_voxm_dc + locals.var_dch);
        (assign46550_e59651, (locals.var_voxm_dc_dn5 + locals.var_dch_dn5), (locals.var_voxm_dc_dn6 + locals.var_dch_dn6), (locals.var_voxm_dc_dn7 + locals.var_dch_dn7), (locals.var_voxm_dc_dn8 + locals.var_dch_dn8),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn5, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8,)
    }
};
        locals.var_arg2mina = assign46550_e59653;
        locals.var_arg2mina_dn5 = assign46550_e59653_d_n5;
        locals.var_arg2mina_dn6 = assign46550_e59653_d_n6;
        locals.var_arg2mina_dn7 = assign46550_e59653_d_n7;
        locals.var_arg2mina_dn8 = assign46550_e59653_d_n8;
        locals.var_arg2mina_rv = 0.0;

        let (assign46560_e59674, assign46560_e59674_d_n5, assign46560_e59674_d_n6, assign46560_e59674_d_n7, assign46560_e59674_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46560_e59660: f64 = locals.var_arg2mina;
        let assign46560_e59663: f64 = (-locals.var_arg2mina);
        let assign46560_e59666: f64 = (-locals.var_arg2mina);
        let assign46560_e59667: f64 = (assign46560_e59663 * assign46560_e59666);
        let assign46560_e59669: f64 = (assign46560_e59667 + 0.01);
        let assign46560_e59670: f64 = (assign46560_e59669).sqrt();
        let assign46560_e59671: f64 = (assign46560_e59660 - assign46560_e59670);
        let assign46560_e59672: f64 = (0.5 * assign46560_e59671);
        (assign46560_e59672, (0.5 * (locals.var_arg2mina_dn5 - ((((-locals.var_arg2mina_dn5) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn5))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn6))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn7))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn8))) / (2.0 * assign46560_e59670)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn5, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8,)
    }
};
        locals.var_psi_t = assign46560_e59674;
        locals.var_psi_t_dn5 = assign46560_e59674_d_n5;
        locals.var_psi_t_dn6 = assign46560_e59674_d_n6;
        locals.var_psi_t_dn7 = assign46560_e59674_d_n7;
        locals.var_psi_t_dn8 = assign46560_e59674_d_n8;
        locals.var_psi_t_rv = 0.0;

        let (assign46570_e59687, assign46570_e59687_d_n5, assign46570_e59687_d_n6, assign46570_e59687_d_n7, assign46570_e59687_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46570_e59680: f64 = (locals.var_voxm_dc * locals.var_voxm_dc);
        let assign46570_e59682: f64 = (assign46570_e59680 + 1e-6);
        let assign46570_e59683: f64 = (assign46570_e59682).sqrt();
        let assign46570_e59685: f64 = (assign46570_e59683 * locals.var_inv_chib);
        (assign46570_e59685, ((((locals.var_voxm_dc_dn5 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn5)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn6 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn6)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn7 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn7)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn8 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn8)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46570_e59687;
        locals.var_zg_dn5 = assign46570_e59687_d_n5;
        locals.var_zg_dn6 = assign46570_e59687_d_n6;
        locals.var_zg_dn7 = assign46570_e59687_d_n7;
        locals.var_zg_dn8 = assign46570_e59687_d_n8;
        locals.var_zg_rv = 0.0;

        let assign46580_e59690: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign46580_e59690;
        locals.var_guard1233_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign46590_e59713, assign46590_e59713_d_n5, assign46590_e59713_d_n6, assign46590_e59713_d_n7, assign46590_e59713_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign46590_e59699: f64 = (locals.var_zg + locals.var_gcq);
        let assign46590_e59702: f64 = (locals.var_zg - locals.var_gcq);
        let assign46590_e59705: f64 = (locals.var_zg - locals.var_gcq);
        let assign46590_e59706: f64 = (assign46590_e59702 * assign46590_e59705);
        let assign46590_e59708: f64 = (assign46590_e59706 + 1e-6);
        let assign46590_e59709: f64 = (assign46590_e59708).sqrt();
        let assign46590_e59710: f64 = (assign46590_e59699 - assign46590_e59709);
        let assign46590_e59711: f64 = (0.5 * assign46590_e59710);
        (assign46590_e59711, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn5)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn6)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn7)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn8)) / (2.0 * assign46590_e59709)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46590_e59713;
        locals.var_zg_dn5 = assign46590_e59713_d_n5;
        locals.var_zg_dn6 = assign46590_e59713_d_n6;
        locals.var_zg_dn7 = assign46590_e59713_d_n7;
        locals.var_zg_dn8 = assign46590_e59713_d_n8;
        locals.var_zg_rv = 0.0;

        let (assign46600_e59727, assign46600_e59727_d_n5, assign46600_e59727_d_n6, assign46600_e59727_d_n7, assign46600_e59727_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46600_e59720: f64 = (locals.var_psi_t - locals.var_alpha_b);
        let assign46600_e59722: f64 = (assign46600_e59720 - locals.var_vm);
        let assign46600_e59724: f64 = (assign46600_e59722 * locals.var_inv_phit1_dc);
        let assign46600_e59725: f64 = (locals.var_x_m_dc + assign46600_e59724);
        (assign46600_e59725, (locals.var_x_m_dc_dn5 + (((locals.var_psi_t_dn5 - locals.var_vm_dn5) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn5))), (locals.var_x_m_dc_dn6 + (((locals.var_psi_t_dn6 - locals.var_vm_dn6) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn6))), (locals.var_x_m_dc_dn7 + (((locals.var_psi_t_dn7 - locals.var_vm_dn7) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn7))), (locals.var_x_m_dc_dn8 + (((locals.var_psi_t_dn8 - locals.var_vm_dn8) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn8))),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn5, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8,)
    }
};
        locals.var_arg1 = assign46600_e59727;
        locals.var_arg1_dn5 = assign46600_e59727_d_n5;
        locals.var_arg1_dn6 = assign46600_e59727_d_n6;
        locals.var_arg1_dn7 = assign46600_e59727_d_n7;
        locals.var_arg1_dn8 = assign46600_e59727_d_n8;
        locals.var_arg1_rv = 0.0;

        let (assign46660_e59826, assign46660_e59826_d_n5, assign46660_e59826_d_n6, assign46660_e59826_d_n7, assign46660_e59826_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46660_e59819: f64 = (locals.var_v_gs + locals.var_vsbstar_dc);
        let assign46660_e59821: f64 = (assign46660_e59819 - locals.var_vm);
        let assign46660_e59822: f64 = (-assign46660_e59821);
        let assign46660_e59824: f64 = (assign46660_e59822 * locals.var_inv_phit1_dc);
        (assign46660_e59824, (((-((locals.var_v_gs_dn5 + locals.var_vsbstar_dc_dn5) - locals.var_vm_dn5)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn5)), (((-((locals.var_v_gs_dn6 + locals.var_vsbstar_dc_dn6) - locals.var_vm_dn6)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn6)), (((-((locals.var_v_gs_dn7 + locals.var_vsbstar_dc_dn7) - locals.var_vm_dn7)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn7)), (((-(locals.var_vsbstar_dc_dn8 - locals.var_vm_dn8)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn8)),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn5, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8,)
    }
};
        locals.var_arg1 = assign46660_e59826;
        locals.var_arg1_dn5 = assign46660_e59826_d_n5;
        locals.var_arg1_dn6 = assign46660_e59826_d_n6;
        locals.var_arg1_dn7 = assign46660_e59826_d_n7;
        locals.var_arg1_dn8 = assign46660_e59826_d_n8;
        locals.var_arg1_rv = 0.0;

        let assign46670_e59828: f64 = (locals.var_arg1).abs();
        let assign46670_e59830: f64 = if assign46670_e59828 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1236 = assign46670_e59830;
        locals.var_guard1236_rv = 0.0;

        let (assign46680_e59839, assign46680_e59839_d_n5, assign46680_e59839_d_n6, assign46680_e59839_d_n7, assign46680_e59839_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 != 0.0)) {
        let assign46680_e59837: f64 = (locals.var_arg1).exp();
        (assign46680_e59837, (assign46680_e59837 * locals.var_arg1_dn5), (assign46680_e59837 * locals.var_arg1_dn6), (assign46680_e59837 * locals.var_arg1_dn7), (assign46680_e59837 * locals.var_arg1_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46680_e59839;
        locals.var_temp__blk936_dn5 = assign46680_e59839_d_n5;
        locals.var_temp__blk936_dn6 = assign46680_e59839_d_n6;
        locals.var_temp__blk936_dn7 = assign46680_e59839_d_n7;
        locals.var_temp__blk936_dn8 = assign46680_e59839_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign46690_e59842: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1237 = assign46690_e59842;
        locals.var_guard1237_rv = 0.0;

        let (assign46700_e59878, assign46700_e59878_d_n5, assign46700_e59878_d_n6, assign46700_e59878_d_n7, assign46700_e59878_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 == 0.0)) && (locals.var_guard1237 != 0.0)) {
        let assign46700_e59854: f64 = (-230.25850929940458);
        let assign46700_e59856: f64 = (assign46700_e59854 - locals.var_arg1);
        let assign46700_e59860: f64 = (-230.25850929940458);
        let assign46700_e59862: f64 = (assign46700_e59860 - locals.var_arg1);
        let assign46700_e59865: f64 = (-230.25850929940458);
        let assign46700_e59867: f64 = (assign46700_e59865 - locals.var_arg1);
        let assign46700_e59869: f64 = (assign46700_e59867 * 0.3333333333333333);
        let assign46700_e59870: f64 = (1.0 + assign46700_e59869);
        let assign46700_e59871: f64 = (assign46700_e59862 * assign46700_e59870);
        let assign46700_e59872: f64 = (0.5 * assign46700_e59871);
        let assign46700_e59873: f64 = (1.0 + assign46700_e59872);
        let assign46700_e59874: f64 = (assign46700_e59856 * assign46700_e59873);
        let assign46700_e59875: f64 = (1.0 + assign46700_e59874);
        let assign46700_e59876: f64 = (1e-100 / assign46700_e59875);
        (assign46700_e59876, (-((1e-100 * (((-locals.var_arg1_dn5) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn5) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn5) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn6) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn7) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn8) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46700_e59878;
        locals.var_temp__blk936_dn5 = assign46700_e59878_d_n5;
        locals.var_temp__blk936_dn6 = assign46700_e59878_d_n6;
        locals.var_temp__blk936_dn7 = assign46700_e59878_d_n7;
        locals.var_temp__blk936_dn8 = assign46700_e59878_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46710_e59912, assign46710_e59912_d_n5, assign46710_e59912_d_n6, assign46710_e59912_d_n7, assign46710_e59912_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 == 0.0)) && (locals.var_guard1237 == 0.0)) {
        let assign46710_e59892: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46710_e59897: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46710_e59901: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46710_e59903: f64 = (assign46710_e59901 * 0.3333333333333333);
        let assign46710_e59904: f64 = (1.0 + assign46710_e59903);
        let assign46710_e59905: f64 = (assign46710_e59897 * assign46710_e59904);
        let assign46710_e59906: f64 = (0.5 * assign46710_e59905);
        let assign46710_e59907: f64 = (1.0 + assign46710_e59906);
        let assign46710_e59908: f64 = (assign46710_e59892 * assign46710_e59907);
        let assign46710_e59909: f64 = (1.0 + assign46710_e59908);
        let assign46710_e59910: f64 = (1e100 * assign46710_e59909);
        (assign46710_e59910, (1e100 * ((locals.var_arg1_dn5 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn5 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn6 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn7 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn8 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn8 * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46710_e59912;
        locals.var_temp__blk936_dn5 = assign46710_e59912_d_n5;
        locals.var_temp__blk936_dn6 = assign46710_e59912_d_n6;
        locals.var_temp__blk936_dn7 = assign46710_e59912_d_n7;
        locals.var_temp__blk936_dn8 = assign46710_e59912_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46730_e59937, assign46730_e59937_d_n5, assign46730_e59937_d_n6, assign46730_e59937_d_n7, assign46730_e59937_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46730_e59926: f64 = (-1.5);
        let assign46730_e59931: f64 = (locals.var_gc3_i * locals.var_zg);
        let assign46730_e59932: f64 = (locals.var_gc2_i + assign46730_e59931);
        let assign46730_e59933: f64 = (locals.var_zg * assign46730_e59932);
        let assign46730_e59934: f64 = (assign46730_e59926 + assign46730_e59933);
        let assign46730_e59935: f64 = (locals.var_bch * assign46730_e59934);
        (assign46730_e59935, (locals.var_bch * ((locals.var_zg_dn5 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn5)))), (locals.var_bch * ((locals.var_zg_dn6 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn6)))), (locals.var_bch * ((locals.var_zg_dn7 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn7)))), (locals.var_bch * ((locals.var_zg_dn8 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46730_e59937;
        locals.var_temp__blk936_dn5 = assign46730_e59937_d_n5;
        locals.var_temp__blk936_dn6 = assign46730_e59937_d_n6;
        locals.var_temp__blk936_dn7 = assign46730_e59937_d_n7;
        locals.var_temp__blk936_dn8 = assign46730_e59937_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign46800_e60043: f64 = if ((locals.var_xg_dc <= 0.0) || ((locals.var_gc2_i == 0.0) && (locals.var_gc3_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1240 = assign46800_e60043;
        locals.var_guard1240_rv = 0.0;

        let (assign46830_e60074, assign46830_e60074_d_n5, assign46830_e60074_d_n6, assign46830_e60074_d_n7, assign46830_e60074_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46830_e60069: f64 = (2.0 * locals.var_gc3_i);
        let assign46830_e60071: f64 = (assign46830_e60069 * locals.var_zg);
        let assign46830_e60072: f64 = (locals.var_gc2_i + assign46830_e60071);
        (assign46830_e60072, (assign46830_e60069 * locals.var_zg_dn5), (assign46830_e60069 * locals.var_zg_dn6), (assign46830_e60069 * locals.var_zg_dn7), (assign46830_e60069 * locals.var_zg_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46830_e60074;
        locals.var_temp__blk936_dn5 = assign46830_e60074_d_n5;
        locals.var_temp__blk936_dn6 = assign46830_e60074_d_n6;
        locals.var_temp__blk936_dn7 = assign46830_e60074_d_n7;
        locals.var_temp__blk936_dn8 = assign46830_e60074_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign46840_e60087, assign46840_e60087_d_n5, assign46840_e60087_d_n6, assign46840_e60087_d_n7, assign46840_e60087_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46840_e60084: f64 = (locals.var_temp__blk936 * locals.var_bch);
        let assign46840_e60085: f64 = (locals.var_chib_i / assign46840_e60084);
        (assign46840_e60085, (-((locals.var_chib_i * (locals.var_temp__blk936_dn5 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn6 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn7 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn8 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))),)
    } else {
        (locals.var_u0, locals.var_u0_dn5, locals.var_u0_dn6, locals.var_u0_dn7, locals.var_u0_dn8,)
    }
};
        locals.var_u0 = assign46840_e60087;
        locals.var_u0_dn5 = assign46840_e60087_d_n5;
        locals.var_u0_dn6 = assign46840_e60087_d_n6;
        locals.var_u0_dn7 = assign46840_e60087_d_n7;
        locals.var_u0_dn8 = assign46840_e60087_d_n8;
        locals.var_u0_rv = 0.0;

        let (assign46850_e60100, assign46850_e60100_d_n5, assign46850_e60100_d_n6, assign46850_e60100_d_n7, assign46850_e60100_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46850_e60097: f64 = (locals.var_dps_dc / locals.var_u0);
        let assign46850_e60098: f64 = (0.5 * assign46850_e60097);
        (assign46850_e60098, (0.5 * (((locals.var_dps_dc_dn5 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn5)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn6 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn6)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn7 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn7)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn8 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn8)) / (locals.var_u0 * locals.var_u0))),)
    } else {
        (locals.var_x, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8,)
    }
};
        locals.var_x = assign46850_e60100;
        locals.var_x_dn5 = assign46850_e60100_d_n5;
        locals.var_x_dn6 = assign46850_e60100_d_n6;
        locals.var_x_dn7 = assign46850_e60100_d_n7;
        locals.var_x_dn8 = assign46850_e60100_d_n8;
        locals.var_x_rv = 0.0;

        let assign46890_e60142: f64 = if locals.var_x < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1241 = assign46890_e60142;
        locals.var_guard1241_rv = 0.0;

        let assign46940_e60235: f64 = (locals.var_x).abs();
        let assign46940_e60237: f64 = if assign46940_e60235 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1242 = assign46940_e60237;
        locals.var_guard1242_rv = 0.0;

        let (assign46950_e60252, assign46950_e60252_d_n5, assign46950_e60252_d_n6, assign46950_e60252_d_n7, assign46950_e60252_d_n8,) = {
    if (((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 != 0.0)) {
        let assign46950_e60250: f64 = (locals.var_x).exp();
        (assign46950_e60250, (assign46950_e60250 * locals.var_x_dn5), (assign46950_e60250 * locals.var_x_dn6), (assign46950_e60250 * locals.var_x_dn7), (assign46950_e60250 * locals.var_x_dn8),)
    } else {
        (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8,)
    }
};
        locals.var_ex = assign46950_e60252;
        locals.var_ex_dn5 = assign46950_e60252_d_n5;
        locals.var_ex_dn6 = assign46950_e60252_d_n6;
        locals.var_ex_dn7 = assign46950_e60252_d_n7;
        locals.var_ex_dn8 = assign46950_e60252_d_n8;
        locals.var_ex_rv = 0.0;

        let assign46960_e60255: f64 = if locals.var_x < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1243 = assign46960_e60255;
        locals.var_guard1243_rv = 0.0;

        let (assign46970_e60297, assign46970_e60297_d_n5, assign46970_e60297_d_n6, assign46970_e60297_d_n7, assign46970_e60297_d_n8,) = {
    if ((((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1243 != 0.0)) {
        let assign46970_e60273: f64 = (-230.25850929940458);
        let assign46970_e60275: f64 = (assign46970_e60273 - locals.var_x);
        let assign46970_e60279: f64 = (-230.25850929940458);
        let assign46970_e60281: f64 = (assign46970_e60279 - locals.var_x);
        let assign46970_e60284: f64 = (-230.25850929940458);
        let assign46970_e60286: f64 = (assign46970_e60284 - locals.var_x);
        let assign46970_e60288: f64 = (assign46970_e60286 * 0.3333333333333333);
        let assign46970_e60289: f64 = (1.0 + assign46970_e60288);
        let assign46970_e60290: f64 = (assign46970_e60281 * assign46970_e60289);
        let assign46970_e60291: f64 = (0.5 * assign46970_e60290);
        let assign46970_e60292: f64 = (1.0 + assign46970_e60291);
        let assign46970_e60293: f64 = (assign46970_e60275 * assign46970_e60292);
        let assign46970_e60294: f64 = (1.0 + assign46970_e60293);
        let assign46970_e60295: f64 = (1e-100 / assign46970_e60294);
        (assign46970_e60295, (-((1e-100 * (((-locals.var_x_dn5) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn5) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn5) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn6) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn6) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn6) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn7) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn7) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn7) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn8) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn8) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn8) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))),)
    } else {
        (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8,)
    }
};
        locals.var_ex = assign46970_e60297;
        locals.var_ex_dn5 = assign46970_e60297_d_n5;
        locals.var_ex_dn6 = assign46970_e60297_d_n6;
        locals.var_ex_dn7 = assign46970_e60297_d_n7;
        locals.var_ex_dn8 = assign46970_e60297_d_n8;
        locals.var_ex_rv = 0.0;

        let (assign46980_e60337, assign46980_e60337_d_n5, assign46980_e60337_d_n6, assign46980_e60337_d_n7, assign46980_e60337_d_n8,) = {
    if ((((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1243 == 0.0)) {
        let assign46980_e60317: f64 = (locals.var_x - 230.25850929940458);
        let assign46980_e60322: f64 = (locals.var_x - 230.25850929940458);
        let assign46980_e60326: f64 = (locals.var_x - 230.25850929940458);
        let assign46980_e60328: f64 = (assign46980_e60326 * 0.3333333333333333);
        let assign46980_e60329: f64 = (1.0 + assign46980_e60328);
        let assign46980_e60330: f64 = (assign46980_e60322 * assign46980_e60329);
        let assign46980_e60331: f64 = (0.5 * assign46980_e60330);
        let assign46980_e60332: f64 = (1.0 + assign46980_e60331);
        let assign46980_e60333: f64 = (assign46980_e60317 * assign46980_e60332);
        let assign46980_e60334: f64 = (1.0 + assign46980_e60333);
        let assign46980_e60335: f64 = (1e100 * assign46980_e60334);
        (assign46980_e60335, (1e100 * ((locals.var_x_dn5 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn5 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn6 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn6 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn7 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn7 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn8 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn8 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn8 * 0.3333333333333333))))))),)
    } else {
        (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8,)
    }
};
        locals.var_ex = assign46980_e60337;
        locals.var_ex_dn5 = assign46980_e60337_d_n5;
        locals.var_ex_dn6 = assign46980_e60337_d_n6;
        locals.var_ex_dn7 = assign46980_e60337_d_n7;
        locals.var_ex_dn8 = assign46980_e60337_d_n8;
        locals.var_ex_rv = 0.0;

        let (assign46990_e60351, assign46990_e60351_d_n5, assign46990_e60351_d_n6, assign46990_e60351_d_n7, assign46990_e60351_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign46990_e60349: f64 = (1.0 / locals.var_ex);
        (assign46990_e60349, (-(locals.var_ex_dn5 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn6 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn7 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn8 / (locals.var_ex * locals.var_ex))),)
    } else {
        (locals.var_inv_ex, locals.var_inv_ex_dn5, locals.var_inv_ex_dn6, locals.var_inv_ex_dn7, locals.var_inv_ex_dn8,)
    }
};
        locals.var_inv_ex = assign46990_e60351;
        locals.var_inv_ex_dn5 = assign46990_e60351_d_n5;
        locals.var_inv_ex_dn6 = assign46990_e60351_d_n6;
        locals.var_inv_ex_dn7 = assign46990_e60351_d_n7;
        locals.var_inv_ex_dn8 = assign46990_e60351_d_n8;
        locals.var_inv_ex_rv = 0.0;

        let (assign47000_e60365, assign47000_e60365_d_n5, assign47000_e60365_d_n6, assign47000_e60365_d_n7, assign47000_e60365_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign47000_e60363: f64 = (locals.var_ex - locals.var_inv_ex);
        (assign47000_e60363, (locals.var_ex_dn5 - locals.var_inv_ex_dn5), (locals.var_ex_dn6 - locals.var_inv_ex_dn6), (locals.var_ex_dn7 - locals.var_inv_ex_dn7), (locals.var_ex_dn8 - locals.var_inv_ex_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47000_e60365;
        locals.var_temp__blk936_dn5 = assign47000_e60365_d_n5;
        locals.var_temp__blk936_dn6 = assign47000_e60365_d_n6;
        locals.var_temp__blk936_dn7 = assign47000_e60365_d_n7;
        locals.var_temp__blk936_dn8 = assign47000_e60365_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign47010_e60379, assign47010_e60379_d_n5, assign47010_e60379_d_n6, assign47010_e60379_d_n7, assign47010_e60379_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign47010_e60377: f64 = (locals.var_ex + locals.var_inv_ex);
        (assign47010_e60377, (locals.var_ex_dn5 + locals.var_inv_ex_dn5), (locals.var_ex_dn6 + locals.var_inv_ex_dn6), (locals.var_ex_dn7 + locals.var_inv_ex_dn7), (locals.var_ex_dn8 + locals.var_inv_ex_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47010_e60379;
        locals.var_temp2_dn5 = assign47010_e60379_d_n5;
        locals.var_temp2_dn6 = assign47010_e60379_d_n6;
        locals.var_temp2_dn7 = assign47010_e60379_d_n7;
        locals.var_temp2_dn8 = assign47010_e60379_d_n8;
        locals.var_temp2_rv = 0.0;

        let assign47110_e60495: f64 = if p.p42 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1244 = assign47110_e60495;
        locals.var_guard1244_rv = 0.0;

        let assign47120_e60502: f64 = if ((locals.var_agidld_i > 0.0) && (locals.var_vovd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1245 = assign47120_e60502;
        locals.var_guard1245_rv = 0.0;

        let (assign47130_e60521, assign47130_e60521_d_n5, assign47130_e60521_d_n6, assign47130_e60521_d_n7, assign47130_e60521_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) {
        let assign47130_e60508: f64 = (locals.var_vovd * locals.var_vovd);
        let assign47130_e60511: f64 = (locals.var_cgidld_i * locals.var_cgidld_i);
        let assign47130_e60514: f64 = (locals.var_vdbprime * locals.var_vdbprime);
        let assign47130_e60515: f64 = (assign47130_e60511 * assign47130_e60514);
        let assign47130_e60516: f64 = (assign47130_e60508 + assign47130_e60515);
        let assign47130_e60518: f64 = (assign47130_e60516 + 1e-6);
        let assign47130_e60519: f64 = (assign47130_e60518).sqrt();
        (assign47130_e60519, (((locals.var_vovd_dn5 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn5)) / (2.0 * assign47130_e60519)), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) + (assign47130_e60511 * ((locals.var_vdbprime_dn6 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn6)))) / (2.0 * assign47130_e60519)), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) + (assign47130_e60511 * ((locals.var_vdbprime_dn7 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn7)))) / (2.0 * assign47130_e60519)), ((assign47130_e60511 * ((locals.var_vdbprime_dn8 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn8))) / (2.0 * assign47130_e60519)),)
    } else {
        (locals.var_vtovd, locals.var_vtovd_dn5, locals.var_vtovd_dn6, locals.var_vtovd_dn7, locals.var_vtovd_dn8,)
    }
};
        locals.var_vtovd = assign47130_e60521;
        locals.var_vtovd_dn5 = assign47130_e60521_d_n5;
        locals.var_vtovd_dn6 = assign47130_e60521_d_n6;
        locals.var_vtovd_dn7 = assign47130_e60521_d_n7;
        locals.var_vtovd_dn8 = assign47130_e60521_d_n8;
        locals.var_vtovd_rv = 0.0;

        let (assign47140_e60530, assign47140_e60530_d_n5, assign47140_e60530_d_n6, assign47140_e60530_d_n7, assign47140_e60530_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) {
        let assign47140_e60526: f64 = (-locals.var_bgidlds);
        let assign47140_e60528: f64 = (assign47140_e60526 / locals.var_vtovd);
        (assign47140_e60528, (-((assign47140_e60526 * locals.var_vtovd_dn5) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn6) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn7) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn8) / (locals.var_vtovd * locals.var_vtovd))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47140_e60530;
        locals.var_temp__blk936_dn5 = assign47140_e60530_d_n5;
        locals.var_temp__blk936_dn6 = assign47140_e60530_d_n6;
        locals.var_temp__blk936_dn7 = assign47140_e60530_d_n7;
        locals.var_temp__blk936_dn8 = assign47140_e60530_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign47150_e60533: f64 = (-230.25850929940458);
        let assign47150_e60534: f64 = if locals.var_temp__blk936 > assign47150_e60533 { 1.0 } else { 0.0 };
        locals.var_guard1246 = assign47150_e60534;
        locals.var_guard1246_rv = 0.0;

        let (assign47160_e60543, assign47160_e60543_d_n5, assign47160_e60543_d_n6, assign47160_e60543_d_n7, assign47160_e60543_d_n8,) = {
    if (((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) && (locals.var_guard1246 != 0.0)) {
        let assign47160_e60541: f64 = (locals.var_temp__blk936).exp();
        (assign47160_e60541, (assign47160_e60541 * locals.var_temp__blk936_dn5), (assign47160_e60541 * locals.var_temp__blk936_dn6), (assign47160_e60541 * locals.var_temp__blk936_dn7), (assign47160_e60541 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47160_e60543;
        locals.var_temp2_dn5 = assign47160_e60543_d_n5;
        locals.var_temp2_dn6 = assign47160_e60543_d_n6;
        locals.var_temp2_dn7 = assign47160_e60543_d_n7;
        locals.var_temp2_dn8 = assign47160_e60543_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign47170_e60577, assign47170_e60577_d_n5, assign47170_e60577_d_n6, assign47170_e60577_d_n7, assign47170_e60577_d_n8,) = {
    if (((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) && (locals.var_guard1246 == 0.0)) {
        let assign47170_e60553: f64 = (-230.25850929940458);
        let assign47170_e60555: f64 = (assign47170_e60553 - locals.var_temp__blk936);
        let assign47170_e60559: f64 = (-230.25850929940458);
        let assign47170_e60561: f64 = (assign47170_e60559 - locals.var_temp__blk936);
        let assign47170_e60564: f64 = (-230.25850929940458);
        let assign47170_e60566: f64 = (assign47170_e60564 - locals.var_temp__blk936);
        let assign47170_e60568: f64 = (assign47170_e60566 * 0.3333333333333333);
        let assign47170_e60569: f64 = (1.0 + assign47170_e60568);
        let assign47170_e60570: f64 = (assign47170_e60561 * assign47170_e60569);
        let assign47170_e60571: f64 = (0.5 * assign47170_e60570);
        let assign47170_e60572: f64 = (1.0 + assign47170_e60571);
        let assign47170_e60573: f64 = (assign47170_e60555 * assign47170_e60572);
        let assign47170_e60574: f64 = (1.0 + assign47170_e60573);
        let assign47170_e60575: f64 = (1e-100 / assign47170_e60574);
        (assign47170_e60575, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47170_e60577;
        locals.var_temp2_dn5 = assign47170_e60577_d_n5;
        locals.var_temp2_dn6 = assign47170_e60577_d_n6;
        locals.var_temp2_dn7 = assign47170_e60577_d_n7;
        locals.var_temp2_dn8 = assign47170_e60577_d_n8;
        locals.var_temp2_rv = 0.0;

        let assign47190_e60599: f64 = if ((locals.var_agidl_i > 0.0) && (locals.var_vovs < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1247 = assign47190_e60599;
        locals.var_guard1247_rv = 0.0;

        let (assign47200_e60618, assign47200_e60618_d_n5, assign47200_e60618_d_n6, assign47200_e60618_d_n7, assign47200_e60618_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47200_e60605: f64 = (locals.var_vovs * locals.var_vovs);
        let assign47200_e60608: f64 = (locals.var_cgidl_i * locals.var_cgidl_i);
        let assign47200_e60611: f64 = (locals.var_vsbprime * locals.var_vsbprime);
        let assign47200_e60612: f64 = (assign47200_e60608 * assign47200_e60611);
        let assign47200_e60613: f64 = (assign47200_e60605 + assign47200_e60612);
        let assign47200_e60615: f64 = (assign47200_e60613 + 1e-6);
        let assign47200_e60616: f64 = (assign47200_e60615).sqrt();
        (assign47200_e60616, (((locals.var_vovs_dn5 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn5)) / (2.0 * assign47200_e60616)), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) + (assign47200_e60608 * ((locals.var_vsbprime_dn6 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn6)))) / (2.0 * assign47200_e60616)), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) + (assign47200_e60608 * ((locals.var_vsbprime_dn7 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn7)))) / (2.0 * assign47200_e60616)), ((assign47200_e60608 * ((locals.var_vsbprime_dn8 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn8))) / (2.0 * assign47200_e60616)),)
    } else {
        (locals.var_vtovs, locals.var_vtovs_dn5, locals.var_vtovs_dn6, locals.var_vtovs_dn7, locals.var_vtovs_dn8,)
    }
};
        locals.var_vtovs = assign47200_e60618;
        locals.var_vtovs_dn5 = assign47200_e60618_d_n5;
        locals.var_vtovs_dn6 = assign47200_e60618_d_n6;
        locals.var_vtovs_dn7 = assign47200_e60618_d_n7;
        locals.var_vtovs_dn8 = assign47200_e60618_d_n8;
        locals.var_vtovs_rv = 0.0;

        let (assign47210_e60627, assign47210_e60627_d_n5, assign47210_e60627_d_n6, assign47210_e60627_d_n7, assign47210_e60627_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47210_e60623: f64 = (-locals.var_bgidls);
        let assign47210_e60625: f64 = (assign47210_e60623 / locals.var_vtovs);
        (assign47210_e60625, (-((assign47210_e60623 * locals.var_vtovs_dn5) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn6) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn7) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn8) / (locals.var_vtovs * locals.var_vtovs))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47210_e60627;
        locals.var_temp__blk936_dn5 = assign47210_e60627_d_n5;
        locals.var_temp__blk936_dn6 = assign47210_e60627_d_n6;
        locals.var_temp__blk936_dn7 = assign47210_e60627_d_n7;
        locals.var_temp__blk936_dn8 = assign47210_e60627_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign47220_e60630: f64 = (-230.25850929940458);
        let assign47220_e60631: f64 = if locals.var_temp__blk936 > assign47220_e60630 { 1.0 } else { 0.0 };
        locals.var_guard1248 = assign47220_e60631;
        locals.var_guard1248_rv = 0.0;

        let (assign47230_e60640, assign47230_e60640_d_n5, assign47230_e60640_d_n6, assign47230_e60640_d_n7, assign47230_e60640_d_n8,) = {
    if (((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign47230_e60638: f64 = (locals.var_temp__blk936).exp();
        (assign47230_e60638, (assign47230_e60638 * locals.var_temp__blk936_dn5), (assign47230_e60638 * locals.var_temp__blk936_dn6), (assign47230_e60638 * locals.var_temp__blk936_dn7), (assign47230_e60638 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47230_e60640;
        locals.var_temp2_dn5 = assign47230_e60640_d_n5;
        locals.var_temp2_dn6 = assign47230_e60640_d_n6;
        locals.var_temp2_dn7 = assign47230_e60640_d_n7;
        locals.var_temp2_dn8 = assign47230_e60640_d_n8;
        locals.var_temp2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47240_e60674, assign47240_e60674_d_n5, assign47240_e60674_d_n6, assign47240_e60674_d_n7, assign47240_e60674_d_n8,) = {
    if (((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 == 0.0)) {
        let assign47240_e60650: f64 = (-230.25850929940458);
        let assign47240_e60652: f64 = (assign47240_e60650 - locals.var_temp__blk936);
        let assign47240_e60656: f64 = (-230.25850929940458);
        let assign47240_e60658: f64 = (assign47240_e60656 - locals.var_temp__blk936);
        let assign47240_e60661: f64 = (-230.25850929940458);
        let assign47240_e60663: f64 = (assign47240_e60661 - locals.var_temp__blk936);
        let assign47240_e60665: f64 = (assign47240_e60663 * 0.3333333333333333);
        let assign47240_e60666: f64 = (1.0 + assign47240_e60665);
        let assign47240_e60667: f64 = (assign47240_e60658 * assign47240_e60666);
        let assign47240_e60668: f64 = (0.5 * assign47240_e60667);
        let assign47240_e60669: f64 = (1.0 + assign47240_e60668);
        let assign47240_e60670: f64 = (assign47240_e60652 * assign47240_e60669);
        let assign47240_e60671: f64 = (1.0 + assign47240_e60670);
        let assign47240_e60672: f64 = (1e-100 / assign47240_e60671);
        (assign47240_e60672, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47240_e60674;
        locals.var_temp2_dn5 = assign47240_e60674_d_n5;
        locals.var_temp2_dn6 = assign47240_e60674_d_n6;
        locals.var_temp2_dn7 = assign47240_e60674_d_n7;
        locals.var_temp2_dn8 = assign47240_e60674_d_n8;
        locals.var_temp2_rv = 0.0;

        locals.var_phit1edge = locals.var_phit;
        locals.var_phit1edge_dn5 = 0.0;
        locals.var_phit1edge_dn6 = 0.0;
        locals.var_phit1edge_dn7 = 0.0;
        locals.var_phit1edge_dn8 = 0.0;
        locals.var_phit1edge_rv = 0.0;

        locals.var_xgedge = 0.0;
        locals.var_xgedge_dn5 = 0.0;
        locals.var_xgedge_dn6 = 0.0;
        locals.var_xgedge_dn7 = 0.0;
        locals.var_xgedge_dn8 = 0.0;
        locals.var_xgedge_rv = 0.0;

        locals.var_qdseffedge = 0.0;
        locals.var_qdseffedge_dn5 = 0.0;
        locals.var_qdseffedge_dn6 = 0.0;
        locals.var_qdseffedge_dn7 = 0.0;
        locals.var_qdseffedge_dn8 = 0.0;
        locals.var_qdseffedge_rv = 0.0;

        locals.var_qmeffedge = 0.0;
        locals.var_qmeffedge_dn5 = 0.0;
        locals.var_qmeffedge_dn6 = 0.0;
        locals.var_qmeffedge_dn7 = 0.0;
        locals.var_qmeffedge_dn8 = 0.0;
        locals.var_qmeffedge_rv = 0.0;

        locals.var_dsqredge = 1e-40;
        locals.var_dsqredge_dn5 = 0.0;
        locals.var_dsqredge_dn6 = 0.0;
        locals.var_dsqredge_dn7 = 0.0;
        locals.var_dsqredge_dn8 = 0.0;
        locals.var_dsqredge_rv = 0.0;

        locals.var_alphabmedge = 1.0;
        locals.var_alphabmedge_dn5 = 0.0;
        locals.var_alphabmedge_dn6 = 0.0;
        locals.var_alphabmedge_dn7 = 0.0;
        locals.var_alphabmedge_dn8 = 0.0;
        locals.var_alphabmedge_rv = 0.0;

        locals.var_i_dsedge = 0.0;
        locals.var_i_dsedge_dn5 = 0.0;
        locals.var_i_dsedge_dn6 = 0.0;
        locals.var_i_dsedge_dn7 = 0.0;
        locals.var_i_dsedge_dn8 = 0.0;
        locals.var_i_dsedge_rv = 0.0;

        let assign47330_e60703: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign47330_e60703;
        locals.var_guard1249_rv = 0.0;

        let (assign47340_e60724, assign47340_e60724_d_n5, assign47340_e60724_d_n6, assign47340_e60724_d_n7, assign47340_e60724_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47340_e60708: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign47340_e60711: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign47340_e60714: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign47340_e60715: f64 = (assign47340_e60711 * assign47340_e60714);
        let assign47340_e60717: f64 = (assign47340_e60715 + locals.var_bphiedge);
        let assign47340_e60718: f64 = (assign47340_e60717).sqrt();
        let assign47340_e60719: f64 = (assign47340_e60708 - assign47340_e60718);
        let assign47340_e60720: f64 = (0.5 * assign47340_e60719);
        let assign47340_e60722: f64 = (assign47340_e60720 + locals.var_phixedge);
        (assign47340_e60722, 0.0, (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign47340_e60718)))), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign47340_e60718)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign47340_e60718)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47340_e60724;
        locals.var_temp__blk936_dn5 = assign47340_e60724_d_n5;
        locals.var_temp__blk936_dn6 = assign47340_e60724_d_n6;
        locals.var_temp__blk936_dn7 = assign47340_e60724_d_n7;
        locals.var_temp__blk936_dn8 = assign47340_e60724_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign47350_e60747, assign47350_e60747_d_n5, assign47350_e60747_d_n6, assign47350_e60747_d_n7, assign47350_e60747_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47350_e60730: f64 = locals.var_temp__blk936;
        let assign47350_e60733: f64 = locals.var_temp__blk936;
        let assign47350_e60736: f64 = locals.var_temp__blk936;
        let assign47350_e60737: f64 = (assign47350_e60733 * assign47350_e60736);
        let assign47350_e60739: f64 = (assign47350_e60737 + locals.var_aphiedge);
        let assign47350_e60740: f64 = (assign47350_e60739).sqrt();
        let assign47350_e60741: f64 = (assign47350_e60730 - assign47350_e60740);
        let assign47350_e60742: f64 = (0.5 * assign47350_e60741);
        let assign47350_e60743: f64 = (locals.var_v_sb - assign47350_e60742);
        let assign47350_e60745: f64 = (assign47350_e60743 + locals.var_phix1edge);
        (assign47350_e60745, (-(0.5 * (locals.var_temp__blk936_dn5 - (((locals.var_temp__blk936_dn5 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn5)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn6 - (0.5 * (locals.var_temp__blk936_dn6 - (((locals.var_temp__blk936_dn6 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn6)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_temp__blk936_dn7 - (((locals.var_temp__blk936_dn7 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn7)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_temp__blk936_dn8 - (((locals.var_temp__blk936_dn8 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn8)) / (2.0 * assign47350_e60740))))),)
    } else {
        (locals.var_vsbstaredge, locals.var_vsbstaredge_dn5, locals.var_vsbstaredge_dn6, locals.var_vsbstaredge_dn7, locals.var_vsbstaredge_dn8,)
    }
};
        locals.var_vsbstaredge = assign47350_e60747;
        locals.var_vsbstaredge_dn5 = assign47350_e60747_d_n5;
        locals.var_vsbstaredge_dn6 = assign47350_e60747_d_n6;
        locals.var_vsbstaredge_dn7 = assign47350_e60747_d_n7;
        locals.var_vsbstaredge_dn8 = assign47350_e60747_d_n8;
        locals.var_vsbstaredge_rv = 0.0;

        let (assign47360_e60757, assign47360_e60757_d_n5, assign47360_e60757_d_n6, assign47360_e60757_d_n7, assign47360_e60757_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47360_e60753: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign47360_e60754: f64 = (0.5 * assign47360_e60753);
        let assign47360_e60755: f64 = (locals.var_vsbstaredge + assign47360_e60754);
        (assign47360_e60755, locals.var_vsbstaredge_dn5, (locals.var_vsbstaredge_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstaredge_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstaredge_dn8,)
    } else {
        (locals.var_vsbxedge, locals.var_vsbxedge_dn5, locals.var_vsbxedge_dn6, locals.var_vsbxedge_dn7, locals.var_vsbxedge_dn8,)
    }
};
        locals.var_vsbxedge = assign47360_e60757;
        locals.var_vsbxedge_dn5 = assign47360_e60757_d_n5;
        locals.var_vsbxedge_dn6 = assign47360_e60757_d_n6;
        locals.var_vsbxedge_dn7 = assign47360_e60757_d_n7;
        locals.var_vsbxedge_dn8 = assign47360_e60757_d_n8;
        locals.var_vsbxedge_rv = 0.0;

        let (assign47370_e60773, assign47370_e60773_d_n5, assign47370_e60773_d_n6, assign47370_e60773_d_n7, assign47370_e60773_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47370_e60763: f64 = (locals.var_pscededge_i * locals.var_vdsx);
        let assign47370_e60764: f64 = (1.0 + assign47370_e60763);
        let assign47370_e60765: f64 = (locals.var_psceedge_i * assign47370_e60764);
        let assign47370_e60769: f64 = (locals.var_pscebedge_i * locals.var_vsbxedge);
        let assign47370_e60770: f64 = (1.0 + assign47370_e60769);
        let assign47370_e60771: f64 = (assign47370_e60765 * assign47370_e60770);
        (assign47370_e60771, (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn5)), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn6)) * assign47370_e60770) + (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn6))), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn7)) * assign47370_e60770) + (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn7))), (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn8)),)
    } else {
        (locals.var_dphit1edge, locals.var_dphit1edge_dn5, locals.var_dphit1edge_dn6, locals.var_dphit1edge_dn7, locals.var_dphit1edge_dn8,)
    }
};
        locals.var_dphit1edge = assign47370_e60773;
        locals.var_dphit1edge_dn5 = assign47370_e60773_d_n5;
        locals.var_dphit1edge_dn6 = assign47370_e60773_d_n6;
        locals.var_dphit1edge_dn7 = assign47370_e60773_d_n7;
        locals.var_dphit1edge_dn8 = assign47370_e60773_d_n8;
        locals.var_dphit1edge_rv = 0.0;

        let (assign47380_e60781, assign47380_e60781_d_n5, assign47380_e60781_d_n6, assign47380_e60781_d_n7, assign47380_e60781_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47380_e60778: f64 = (1.0 + locals.var_dphit1edge);
        let assign47380_e60779: f64 = (locals.var_phit0edge * assign47380_e60778);
        (assign47380_e60779, (locals.var_phit0edge * locals.var_dphit1edge_dn5), (locals.var_phit0edge * locals.var_dphit1edge_dn6), (locals.var_phit0edge * locals.var_dphit1edge_dn7), (locals.var_phit0edge * locals.var_dphit1edge_dn8),)
    } else {
        (locals.var_phit1edge, locals.var_phit1edge_dn5, locals.var_phit1edge_dn6, locals.var_phit1edge_dn7, locals.var_phit1edge_dn8,)
    }
};
        locals.var_phit1edge = assign47380_e60781;
        locals.var_phit1edge_dn5 = assign47380_e60781_d_n5;
        locals.var_phit1edge_dn6 = assign47380_e60781_d_n6;
        locals.var_phit1edge_dn7 = assign47380_e60781_d_n7;
        locals.var_phit1edge_dn8 = assign47380_e60781_d_n8;
        locals.var_phit1edge_rv = 0.0;

        let (assign47390_e60787, assign47390_e60787_d_n5, assign47390_e60787_d_n6, assign47390_e60787_d_n7, assign47390_e60787_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47390_e60785: f64 = (1.0 / locals.var_phit1edge);
        (assign47390_e60785, (-(locals.var_phit1edge_dn5 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn6 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn7 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn8 / (locals.var_phit1edge * locals.var_phit1edge))),)
    } else {
        (locals.var_inv_phit1edge, locals.var_inv_phit1edge_dn5, locals.var_inv_phit1edge_dn6, locals.var_inv_phit1edge_dn7, locals.var_inv_phit1edge_dn8,)
    }
};
        locals.var_inv_phit1edge = assign47390_e60787;
        locals.var_inv_phit1edge_dn5 = assign47390_e60787_d_n5;
        locals.var_inv_phit1edge_dn6 = assign47390_e60787_d_n6;
        locals.var_inv_phit1edge_dn7 = assign47390_e60787_d_n7;
        locals.var_inv_phit1edge_dn8 = assign47390_e60787_d_n8;
        locals.var_inv_phit1edge_rv = 0.0;

        let (assign47400_e60802, assign47400_e60802_d_n6, assign47400_e60802_d_n7,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47400_e60791: f64 = (2.0 * locals.var_vdsx);
        let assign47400_e60796: f64 = (locals.var_cfdedge_i * locals.var_vdsx);
        let assign47400_e60797: f64 = (1.0 + assign47400_e60796);
        let assign47400_e60798: f64 = (assign47400_e60797).sqrt();
        let assign47400_e60799: f64 = (1.0 + assign47400_e60798);
        let assign47400_e60800: f64 = (assign47400_e60791 / assign47400_e60799);
        (assign47400_e60800, ((((2.0 * locals.var_vdsx_dn6) * assign47400_e60799) - (assign47400_e60791 * ((locals.var_cfdedge_i * locals.var_vdsx_dn6) / (2.0 * assign47400_e60798)))) / (assign47400_e60799 * assign47400_e60799)), ((((2.0 * locals.var_vdsx_dn7) * assign47400_e60799) - (assign47400_e60791 * ((locals.var_cfdedge_i * locals.var_vdsx_dn7) / (2.0 * assign47400_e60798)))) / (assign47400_e60799 * assign47400_e60799)),)
    } else {
        (locals.var_vdspedge, locals.var_vdspedge_dn6, locals.var_vdspedge_dn7,)
    }
};
        locals.var_vdspedge = assign47400_e60802;
        locals.var_vdspedge_dn6 = assign47400_e60802_d_n6;
        locals.var_vdspedge_dn7 = assign47400_e60802_d_n7;
        locals.var_vdspedge_rv = 0.0;

        let (assign47410_e60814, assign47410_e60814_d_n5, assign47410_e60814_d_n6, assign47410_e60814_d_n7, assign47410_e60814_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47410_e60806: f64 = (locals.var_cfedge_i * locals.var_vdspedge);
        let assign47410_e60810: f64 = (locals.var_cfbedge_i * locals.var_vsbxedge);
        let assign47410_e60811: f64 = (1.0 + assign47410_e60810);
        let assign47410_e60812: f64 = (assign47410_e60806 * assign47410_e60811);
        (assign47410_e60812, (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn5)), (((locals.var_cfedge_i * locals.var_vdspedge_dn6) * assign47410_e60811) + (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn6))), (((locals.var_cfedge_i * locals.var_vdspedge_dn7) * assign47410_e60811) + (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn7))), (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn8)),)
    } else {
        (locals.var_delvgedge, locals.var_delvgedge_dn5, locals.var_delvgedge_dn6, locals.var_delvgedge_dn7, locals.var_delvgedge_dn8,)
    }
};
        locals.var_delvgedge = assign47410_e60814;
        locals.var_delvgedge_dn5 = assign47410_e60814_d_n5;
        locals.var_delvgedge_dn6 = assign47410_e60814_d_n6;
        locals.var_delvgedge_dn7 = assign47410_e60814_d_n7;
        locals.var_delvgedge_dn8 = assign47410_e60814_d_n8;
        locals.var_delvgedge_rv = 0.0;

        let (assign47420_e60824, assign47420_e60824_d_n5, assign47420_e60824_d_n6, assign47420_e60824_d_n7, assign47420_e60824_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47420_e60819: f64 = (locals.var_vgb + locals.var_delvgedge);
        let assign47420_e60821: f64 = (assign47420_e60819 - locals.var_vfbedge_t);
        let assign47420_e60822: f64 = (locals.var_inv_phit1edge * assign47420_e60821);
        (assign47420_e60822, ((locals.var_inv_phit1edge_dn5 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn5 + locals.var_delvgedge_dn5))), ((locals.var_inv_phit1edge_dn6 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn6 + locals.var_delvgedge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn7 + locals.var_delvgedge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn8 + locals.var_delvgedge_dn8))),)
    } else {
        (locals.var_xgedge, locals.var_xgedge_dn5, locals.var_xgedge_dn6, locals.var_xgedge_dn7, locals.var_xgedge_dn8,)
    }
};
        locals.var_xgedge = assign47420_e60824;
        locals.var_xgedge_dn5 = assign47420_e60824_d_n5;
        locals.var_xgedge_dn6 = assign47420_e60824_d_n6;
        locals.var_xgedge_dn7 = assign47420_e60824_d_n7;
        locals.var_xgedge_dn8 = assign47420_e60824_d_n8;
        locals.var_xgedge_rv = 0.0;

        let (assign47430_e60830, assign47430_e60830_d_n5, assign47430_e60830_d_n6, assign47430_e60830_d_n7, assign47430_e60830_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47430_e60828: f64 = (locals.var_inv_phit1edge * locals.var_phibedge);
        (assign47430_e60828, (locals.var_inv_phit1edge_dn5 * locals.var_phibedge), (locals.var_inv_phit1edge_dn6 * locals.var_phibedge), (locals.var_inv_phit1edge_dn7 * locals.var_phibedge), (locals.var_inv_phit1edge_dn8 * locals.var_phibedge),)
    } else {
        (locals.var_xbedge, locals.var_xbedge_dn5, locals.var_xbedge_dn6, locals.var_xbedge_dn7, locals.var_xbedge_dn8,)
    }
};
        locals.var_xbedge = assign47430_e60830;
        locals.var_xbedge_dn5 = assign47430_e60830_d_n5;
        locals.var_xbedge_dn6 = assign47430_e60830_d_n6;
        locals.var_xbedge_dn7 = assign47430_e60830_d_n7;
        locals.var_xbedge_dn8 = assign47430_e60830_d_n8;
        locals.var_xbedge_rv = 0.0;

        let (assign47440_e60842, assign47440_e60842_d_n5, assign47440_e60842_d_n6, assign47440_e60842_d_n7, assign47440_e60842_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47440_e60835: f64 = (locals.var_xbedge / locals.var_gfedge);
        let assign47440_e60837: f64 = (locals.var_xbedge).sqrt();
        let assign47440_e60838: f64 = (assign47440_e60835 + assign47440_e60837);
        let assign47440_e60839: f64 = (assign47440_e60838).ln();
        let assign47440_e60840: f64 = (2.0 * assign47440_e60839);
        (assign47440_e60840, (2.0 * (((locals.var_xbedge_dn5 / locals.var_gfedge) + (locals.var_xbedge_dn5 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn6 / locals.var_gfedge) + (locals.var_xbedge_dn6 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn7 / locals.var_gfedge) + (locals.var_xbedge_dn7 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn8 / locals.var_gfedge) + (locals.var_xbedge_dn8 / (2.0 * assign47440_e60837))) / assign47440_e60838)),)
    } else {
        (locals.var_dxthedge, locals.var_dxthedge_dn5, locals.var_dxthedge_dn6, locals.var_dxthedge_dn7, locals.var_dxthedge_dn8,)
    }
};
        locals.var_dxthedge = assign47440_e60842;
        locals.var_dxthedge_dn5 = assign47440_e60842_d_n5;
        locals.var_dxthedge_dn6 = assign47440_e60842_d_n6;
        locals.var_dxthedge_dn7 = assign47440_e60842_d_n7;
        locals.var_dxthedge_dn8 = assign47440_e60842_d_n8;
        locals.var_dxthedge_rv = 0.0;

        let (assign47450_e60848, assign47450_e60848_d_n5, assign47450_e60848_d_n6, assign47450_e60848_d_n7, assign47450_e60848_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47450_e60846: f64 = (locals.var_inv_phit1edge * locals.var_vsbstaredge);
        (assign47450_e60846, ((locals.var_inv_phit1edge_dn5 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn5)), ((locals.var_inv_phit1edge_dn6 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn6)), ((locals.var_inv_phit1edge_dn7 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn7)), ((locals.var_inv_phit1edge_dn8 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn8)),)
    } else {
        (locals.var_xnedge_s, locals.var_xnedge_s_dn5, locals.var_xnedge_s_dn6, locals.var_xnedge_s_dn7, locals.var_xnedge_s_dn8,)
    }
};
        locals.var_xnedge_s = assign47450_e60848;
        locals.var_xnedge_s_dn5 = assign47450_e60848_d_n5;
        locals.var_xnedge_s_dn6 = assign47450_e60848_d_n6;
        locals.var_xnedge_s_dn7 = assign47450_e60848_d_n7;
        locals.var_xnedge_s_dn8 = assign47450_e60848_d_n8;
        locals.var_xnedge_s_rv = 0.0;

        let (assign47460_e60854, assign47460_e60854_d_n5, assign47460_e60854_d_n6, assign47460_e60854_d_n7, assign47460_e60854_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47460_e60852: f64 = (locals.var_xbedge + locals.var_xnedge_s);
        (assign47460_e60852, (locals.var_xbedge_dn5 + locals.var_xnedge_s_dn5), (locals.var_xbedge_dn6 + locals.var_xnedge_s_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_s_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_s_dn8),)
    } else {
        (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn5, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8,)
    }
};
        locals.var_q_edge_xsth = assign47460_e60854;
        locals.var_q_edge_xsth_dn5 = assign47460_e60854_d_n5;
        locals.var_q_edge_xsth_dn6 = assign47460_e60854_d_n6;
        locals.var_q_edge_xsth_dn7 = assign47460_e60854_d_n7;
        locals.var_q_edge_xsth_dn8 = assign47460_e60854_d_n8;
        locals.var_q_edge_xsth_rv = 0.0;

        let (assign47470_e60863, assign47470_e60863_d_n5, assign47470_e60863_d_n6, assign47470_e60863_d_n7, assign47470_e60863_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47470_e60859: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47470_e60860: f64 = (locals.var_gfedge * assign47470_e60859);
        let assign47470_e60861: f64 = (locals.var_q_edge_xsth + assign47470_e60860);
        (assign47470_e60861, (locals.var_q_edge_xsth_dn5 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47470_e60859)))),)
    } else {
        (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn5, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8,)
    }
};
        locals.var_q_edge_xth0 = assign47470_e60863;
        locals.var_q_edge_xth0_dn5 = assign47470_e60863_d_n5;
        locals.var_q_edge_xth0_dn6 = assign47470_e60863_d_n6;
        locals.var_q_edge_xth0_dn7 = assign47470_e60863_d_n7;
        locals.var_q_edge_xth0_dn8 = assign47470_e60863_d_n8;
        locals.var_q_edge_xth0_rv = 0.0;

        let (assign47480_e60869, assign47480_e60869_d_n5, assign47480_e60869_d_n6, assign47480_e60869_d_n7, assign47480_e60869_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47480_e60867: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
        (assign47480_e60867, (locals.var_q_edge_xth0_dn5 + locals.var_dxthedge_dn5), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8),)
    } else {
        (locals.var_q_edge_xth, locals.var_q_edge_xth_dn5, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8,)
    }
};
        locals.var_q_edge_xth = assign47480_e60869;
        locals.var_q_edge_xth_dn5 = assign47480_e60869_d_n5;
        locals.var_q_edge_xth_dn6 = assign47480_e60869_d_n6;
        locals.var_q_edge_xth_dn7 = assign47480_e60869_d_n7;
        locals.var_q_edge_xth_dn8 = assign47480_e60869_d_n8;
        locals.var_q_edge_xth_rv = 0.0;

        let (assign47490_e60880, assign47490_e60880_d_n5, assign47490_e60880_d_n6, assign47490_e60880_d_n7, assign47490_e60880_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47490_e60875: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47490_e60876: f64 = (2.0 * assign47490_e60875);
        let assign47490_e60877: f64 = (locals.var_gfedge / assign47490_e60876);
        let assign47490_e60878: f64 = (1.0 + assign47490_e60877);
        (assign47490_e60878, (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))),)
    } else {
        (locals.var_q_edge_n, locals.var_q_edge_n_dn5, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8,)
    }
};
        locals.var_q_edge_n = assign47490_e60880;
        locals.var_q_edge_n_dn5 = assign47490_e60880_d_n5;
        locals.var_q_edge_n_dn6 = assign47490_e60880_d_n6;
        locals.var_q_edge_n_dn7 = assign47490_e60880_d_n7;
        locals.var_q_edge_n_dn8 = assign47490_e60880_d_n8;
        locals.var_q_edge_n_rv = 0.0;

        let (assign47500_e60886, assign47500_e60886_d_n5, assign47500_e60886_d_n6, assign47500_e60886_d_n7, assign47500_e60886_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47500_e60884: f64 = (1.0 / locals.var_q_edge_n);
        (assign47500_e60884, (-(locals.var_q_edge_n_dn5 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))),)
    } else {
        (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn5, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8,)
    }
};
        locals.var_q_edge_n_inv = assign47500_e60886;
        locals.var_q_edge_n_inv_dn5 = assign47500_e60886_d_n5;
        locals.var_q_edge_n_inv_dn6 = assign47500_e60886_d_n6;
        locals.var_q_edge_n_inv_dn7 = assign47500_e60886_d_n7;
        locals.var_q_edge_n_inv_dn8 = assign47500_e60886_d_n8;
        locals.var_q_edge_n_inv_rv = 0.0;

        let (assign47510_e60892, assign47510_e60892_d_n5, assign47510_e60892_d_n6, assign47510_e60892_d_n7, assign47510_e60892_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47510_e60890: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
        (assign47510_e60890, (locals.var_xgedge_dn5 - locals.var_q_edge_xth_dn5), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8),)
    } else {
        (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8,)
    }
};
        locals.var_q_edge_xgt = assign47510_e60892;
        locals.var_q_edge_xgt_dn5 = assign47510_e60892_d_n5;
        locals.var_q_edge_xgt_dn6 = assign47510_e60892_d_n6;
        locals.var_q_edge_xgt_dn7 = assign47510_e60892_d_n7;
        locals.var_q_edge_xgt_dn8 = assign47510_e60892_d_n8;
        locals.var_q_edge_xgt_rv = 0.0;

        let assign47520_e60895: f64 = (-12.0);
        let assign47520_e60896: f64 = if locals.var_q_edge_xgt > assign47520_e60895 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign47520_e60896;
        locals.var_guard1250_rv = 0.0;

        let (assign47530_e60906, assign47530_e60906_d_n5, assign47530_e60906_d_n6, assign47530_e60906_d_n7, assign47530_e60906_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47530_e60902: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47530_e60904: f64 = (assign47530_e60902 - 1.0);
        (assign47530_e60904, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8,)
    } else {
        (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn5, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8,)
    }
};
        locals.var_q_edge_xgt0 = assign47530_e60906;
        locals.var_q_edge_xgt0_dn5 = assign47530_e60906_d_n5;
        locals.var_q_edge_xgt0_dn6 = assign47530_e60906_d_n6;
        locals.var_q_edge_xgt0_dn7 = assign47530_e60906_d_n7;
        locals.var_q_edge_xgt0_dn8 = assign47530_e60906_d_n8;
        locals.var_q_edge_xgt0_rv = 0.0;

        let (assign47540_e60921, assign47540_e60921_d_n5, assign47540_e60921_d_n6, assign47540_e60921_d_n7, assign47540_e60921_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47540_e60914: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
        let assign47540_e60916: f64 = (assign47540_e60914 + 10.0);
        let assign47540_e60917: f64 = (assign47540_e60916).sqrt();
        let assign47540_e60918: f64 = (locals.var_q_edge_xgt0 + assign47540_e60917);
        let assign47540_e60919: f64 = (0.5 * assign47540_e60918);
        (assign47540_e60919, (0.5 * (locals.var_q_edge_xgt0_dn5 + (((locals.var_q_edge_xgt0_dn5 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn5)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47540_e60917)))),)
    } else {
        (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn5, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8,)
    }
};
        locals.var_q_edge_xgt0e = assign47540_e60921;
        locals.var_q_edge_xgt0e_dn5 = assign47540_e60921_d_n5;
        locals.var_q_edge_xgt0e_dn6 = assign47540_e60921_d_n6;
        locals.var_q_edge_xgt0e_dn7 = assign47540_e60921_d_n7;
        locals.var_q_edge_xgt0e_dn8 = assign47540_e60921_d_n8;
        locals.var_q_edge_xgt0e_rv = 0.0;

        let (assign47550_e60934, assign47550_e60934_d_n5, assign47550_e60934_d_n6, assign47550_e60934_d_n7, assign47550_e60934_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47550_e60928: f64 = (locals.var_q_edge_xgt0e).ln();
        let assign47550_e60929: f64 = (locals.var_q_edge_n * assign47550_e60928);
        let assign47550_e60930: f64 = (locals.var_q_edge_xgt - assign47550_e60929);
        let assign47550_e60932: f64 = (assign47550_e60930 + locals.var_lngfedge2);
        (assign47550_e60932, (locals.var_q_edge_xgt_dn5 - ((locals.var_q_edge_n_dn5 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn5 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))),)
    } else {
        (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn5, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8,)
    }
};
        locals.var_q_edge_qi0si = assign47550_e60934;
        locals.var_q_edge_qi0si_dn5 = assign47550_e60934_d_n5;
        locals.var_q_edge_qi0si_dn6 = assign47550_e60934_d_n6;
        locals.var_q_edge_qi0si_dn7 = assign47550_e60934_d_n7;
        locals.var_q_edge_qi0si_dn8 = assign47550_e60934_d_n8;
        locals.var_q_edge_qi0si_rv = 0.0;

        let (assign47560_e60949, assign47560_e60949_d_n5, assign47560_e60949_d_n6, assign47560_e60949_d_n7, assign47560_e60949_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47560_e60942: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
        let assign47560_e60944: f64 = (assign47560_e60942 + 2.0);
        let assign47560_e60945: f64 = (assign47560_e60944).sqrt();
        let assign47560_e60946: f64 = (locals.var_q_edge_qi0si + assign47560_e60945);
        let assign47560_e60947: f64 = (0.5 * assign47560_e60946);
        (assign47560_e60947, (0.5 * (locals.var_q_edge_qi0si_dn5 + (((locals.var_q_edge_qi0si_dn5 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn5)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47560_e60945)))),)
    } else {
        (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn5, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8,)
    }
};
        locals.var_q_edge_qi0 = assign47560_e60949;
        locals.var_q_edge_qi0_dn5 = assign47560_e60949_d_n5;
        locals.var_q_edge_qi0_dn6 = assign47560_e60949_d_n6;
        locals.var_q_edge_qi0_dn7 = assign47560_e60949_d_n7;
        locals.var_q_edge_qi0_dn8 = assign47560_e60949_d_n8;
        locals.var_q_edge_qi0_rv = 0.0;

        let assign47570_e60952: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47570_e60954: f64 = if assign47570_e60952 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1251 = assign47570_e60954;
        locals.var_guard1251_rv = 0.0;

        let (assign47580_e60965, assign47580_e60965_d_n5, assign47580_e60965_d_n6, assign47580_e60965_d_n7, assign47580_e60965_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) && (locals.var_guard1251 != 0.0)) {
        let assign47580_e60962: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47580_e60963: f64 = (assign47580_e60962).exp();
        (assign47580_e60963, (assign47580_e60963 * (locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8,)
    }
};
        locals.var_q_edge_exp_x = assign47580_e60965;
        locals.var_q_edge_exp_x_dn5 = assign47580_e60965_d_n5;
        locals.var_q_edge_exp_x_dn6 = assign47580_e60965_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47580_e60965_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47580_e60965_d_n8;
        locals.var_q_edge_exp_x_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_33(
        locals: &mut StampLocals,
    ) {
        let (assign47590_e61002, assign47590_e61002_d_n5, assign47590_e61002_d_n6, assign47590_e61002_d_n7, assign47590_e61002_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) && (locals.var_guard1251 == 0.0)) {
        let assign47590_e60976: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47590_e60978: f64 = (assign47590_e60976 - 230.25850929940458);
        let assign47590_e60983: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47590_e60985: f64 = (assign47590_e60983 - 230.25850929940458);
        let assign47590_e60989: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47590_e60991: f64 = (assign47590_e60989 - 230.25850929940458);
        let assign47590_e60993: f64 = (assign47590_e60991 * 0.3333333333333333);
        let assign47590_e60994: f64 = (1.0 + assign47590_e60993);
        let assign47590_e60995: f64 = (assign47590_e60985 * assign47590_e60994);
        let assign47590_e60996: f64 = (0.5 * assign47590_e60995);
        let assign47590_e60997: f64 = (1.0 + assign47590_e60996);
        let assign47590_e60998: f64 = (assign47590_e60978 * assign47590_e60997);
        let assign47590_e60999: f64 = (1.0 + assign47590_e60998);
        let assign47590_e61000: f64 = (1e100 * assign47590_e60999);
        (assign47590_e61000, (1e100 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8,)
    }
};
        locals.var_q_edge_exp_x = assign47590_e61002;
        locals.var_q_edge_exp_x_dn5 = assign47590_e61002_d_n5;
        locals.var_q_edge_exp_x_dn6 = assign47590_e61002_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47590_e61002_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47590_e61002_d_n8;
        locals.var_q_edge_exp_x_rv = 0.0;

        let (assign47600_e61010, assign47600_e61010_d_n5, assign47600_e61010_d_n6, assign47600_e61010_d_n7, assign47600_e61010_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47600_e61008: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
        (assign47600_e61008, (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn5), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8),)
    } else {
        (locals.var_q_edge_d0, locals.var_q_edge_d0_dn5, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8,)
    }
};
        locals.var_q_edge_d0 = assign47600_e61010;
        locals.var_q_edge_d0_dn5 = assign47600_e61010_d_n5;
        locals.var_q_edge_d0_dn6 = assign47600_e61010_d_n6;
        locals.var_q_edge_d0_dn7 = assign47600_e61010_d_n7;
        locals.var_q_edge_d0_dn8 = assign47600_e61010_d_n8;
        locals.var_q_edge_d0_rv = 0.0;

        let (assign47610_e61018, assign47610_e61018_d_n5, assign47610_e61018_d_n6, assign47610_e61018_d_n7, assign47610_e61018_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47610_e61016: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
        (assign47610_e61016, if locals.var_q_edge_n_inv_dn5 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn5)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn5 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn5 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) },)
    } else {
        (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn5, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8,)
    }
};
        locals.var_q_edge_d0p = assign47610_e61018;
        locals.var_q_edge_d0p_dn5 = assign47610_e61018_d_n5;
        locals.var_q_edge_d0p_dn6 = assign47610_e61018_d_n6;
        locals.var_q_edge_d0p_dn7 = assign47610_e61018_d_n7;
        locals.var_q_edge_d0p_dn8 = assign47610_e61018_d_n8;
        locals.var_q_edge_d0p_rv = 0.0;

        let (assign47620_e61036, assign47620_e61036_d_n5, assign47620_e61036_d_n6, assign47620_e61036_d_n7, assign47620_e61036_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47620_e61024: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
        let assign47620_e61028: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
        let assign47620_e61029: f64 = (2.0 * assign47620_e61028);
        let assign47620_e61031: f64 = (assign47620_e61029 - locals.var_q_edge_d0p);
        let assign47620_e61033: f64 = (assign47620_e61031 * locals.var_q_edge_d0p);
        let assign47620_e61034: f64 = (assign47620_e61024 + assign47620_e61033);
        (assign47620_e61034, (((locals.var_q_edge_n_dn5 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn5)) + ((((2.0 * (locals.var_q_edge_qi0_dn5 + locals.var_q_edge_n_dn5)) - locals.var_q_edge_d0p_dn5) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn5))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn8))),)
    } else {
        (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn5, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8,)
    }
};
        locals.var_q_edge_sqerr = assign47620_e61036;
        locals.var_q_edge_sqerr_dn5 = assign47620_e61036_d_n5;
        locals.var_q_edge_sqerr_dn6 = assign47620_e61036_d_n6;
        locals.var_q_edge_sqerr_dn7 = assign47620_e61036_d_n7;
        locals.var_q_edge_sqerr_dn8 = assign47620_e61036_d_n8;
        locals.var_q_edge_sqerr_rv = 0.0;

        let (assign47630_e61051, assign47630_e61051_d_n5, assign47630_e61051_d_n6, assign47630_e61051_d_n7, assign47630_e61051_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47630_e61042: f64 = (locals.var_q_edge_sqerr).sqrt();
        let assign47630_e61044: f64 = (assign47630_e61042 - locals.var_q_edge_n);
        let assign47630_e61046: f64 = (assign47630_e61044 / locals.var_q_edge_d0p);
        let assign47630_e61048: f64 = (assign47630_e61046 - 1.0);
        let assign47630_e61049: f64 = (locals.var_q_edge_n * assign47630_e61048);
        (assign47630_e61049, ((locals.var_q_edge_n_dn5 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn5 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn5) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn5)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))),)
    } else {
        (locals.var_q_edge_errq, locals.var_q_edge_errq_dn5, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8,)
    }
};
        locals.var_q_edge_errq = assign47630_e61051;
        locals.var_q_edge_errq_dn5 = assign47630_e61051_d_n5;
        locals.var_q_edge_errq_dn6 = assign47630_e61051_d_n6;
        locals.var_q_edge_errq_dn7 = assign47630_e61051_d_n7;
        locals.var_q_edge_errq_dn8 = assign47630_e61051_d_n8;
        locals.var_q_edge_errq_rv = 0.0;

        let (assign47640_e61059, assign47640_e61059_d_n5, assign47640_e61059_d_n6, assign47640_e61059_d_n7, assign47640_e61059_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47640_e61057: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
        (assign47640_e61057, (locals.var_q_edge_qi0_dn5 - locals.var_q_edge_errq_dn5), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8,)
    }
};
        locals.var_qseffedge = assign47640_e61059;
        locals.var_qseffedge_dn5 = assign47640_e61059_d_n5;
        locals.var_qseffedge_dn6 = assign47640_e61059_d_n6;
        locals.var_qseffedge_dn7 = assign47640_e61059_d_n7;
        locals.var_qseffedge_dn8 = assign47640_e61059_d_n8;
        locals.var_qseffedge_rv = 0.0;

        let assign47650_e61063: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47650_e61064: f64 = (locals.var_q_edge_n_inv * assign47650_e61063);
        let assign47650_e61066: f64 = (-230.25850929940458);
        let assign47650_e61067: f64 = if assign47650_e61064 > assign47650_e61066 { 1.0 } else { 0.0 };
        locals.var_guard1252 = assign47650_e61067;
        locals.var_guard1252_rv = 0.0;

        let (assign47660_e61081, assign47660_e61081_d_n5, assign47660_e61081_d_n6, assign47660_e61081_d_n7, assign47660_e61081_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 == 0.0)) && (locals.var_guard1252 != 0.0)) {
        let assign47660_e61077: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47660_e61078: f64 = (locals.var_q_edge_n_inv * assign47660_e61077);
        let assign47660_e61079: f64 = (assign47660_e61078).exp();
        (assign47660_e61079, (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn5 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn6 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn7 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn8 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8,)
    }
};
        locals.var_qseffedge = assign47660_e61081;
        locals.var_qseffedge_dn5 = assign47660_e61081_d_n5;
        locals.var_qseffedge_dn6 = assign47660_e61081_d_n6;
        locals.var_qseffedge_dn7 = assign47660_e61081_d_n7;
        locals.var_qseffedge_dn8 = assign47660_e61081_d_n8;
        locals.var_qseffedge_rv = 0.0;

        let (assign47670_e61128, assign47670_e61128_d_n5, assign47670_e61128_d_n6, assign47670_e61128_d_n7, assign47670_e61128_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 == 0.0)) && (locals.var_guard1252 == 0.0)) {
        let assign47670_e61092: f64 = (-230.25850929940458);
        let assign47670_e61096: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47670_e61097: f64 = (locals.var_q_edge_n_inv * assign47670_e61096);
        let assign47670_e61098: f64 = (assign47670_e61092 - assign47670_e61097);
        let assign47670_e61102: f64 = (-230.25850929940458);
        let assign47670_e61106: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47670_e61107: f64 = (locals.var_q_edge_n_inv * assign47670_e61106);
        let assign47670_e61108: f64 = (assign47670_e61102 - assign47670_e61107);
        let assign47670_e61111: f64 = (-230.25850929940458);
        let assign47670_e61115: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47670_e61116: f64 = (locals.var_q_edge_n_inv * assign47670_e61115);
        let assign47670_e61117: f64 = (assign47670_e61111 - assign47670_e61116);
        let assign47670_e61119: f64 = (assign47670_e61117 * 0.3333333333333333);
        let assign47670_e61120: f64 = (1.0 + assign47670_e61119);
        let assign47670_e61121: f64 = (assign47670_e61108 * assign47670_e61120);
        let assign47670_e61122: f64 = (0.5 * assign47670_e61121);
        let assign47670_e61123: f64 = (1.0 + assign47670_e61122);
        let assign47670_e61124: f64 = (assign47670_e61098 * assign47670_e61123);
        let assign47670_e61125: f64 = (1.0 + assign47670_e61124);
        let assign47670_e61126: f64 = (1e-100 / assign47670_e61125);
        (assign47670_e61126, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8,)
    }
};
        locals.var_qseffedge = assign47670_e61128;
        locals.var_qseffedge_dn5 = assign47670_e61128_d_n5;
        locals.var_qseffedge_dn6 = assign47670_e61128_d_n6;
        locals.var_qseffedge_dn7 = assign47670_e61128_d_n7;
        locals.var_qseffedge_dn8 = assign47670_e61128_d_n8;
        locals.var_qseffedge_rv = 0.0;

        let (assign47680_e61136, assign47680_e61136_d_n5, assign47680_e61136_d_n6, assign47680_e61136_d_n7, assign47680_e61136_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47680_e61133: f64 = (locals.var_vdse_dc + locals.var_vsbstaredge);
        let assign47680_e61134: f64 = (locals.var_inv_phit1edge * assign47680_e61133);
        (assign47680_e61134, ((locals.var_inv_phit1edge_dn5 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn5 + locals.var_vsbstaredge_dn5))), ((locals.var_inv_phit1edge_dn6 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn6 + locals.var_vsbstaredge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn7 + locals.var_vsbstaredge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn8 + locals.var_vsbstaredge_dn8))),)
    } else {
        (locals.var_xnedge_d, locals.var_xnedge_d_dn5, locals.var_xnedge_d_dn6, locals.var_xnedge_d_dn7, locals.var_xnedge_d_dn8,)
    }
};
        locals.var_xnedge_d = assign47680_e61136;
        locals.var_xnedge_d_dn5 = assign47680_e61136_d_n5;
        locals.var_xnedge_d_dn6 = assign47680_e61136_d_n6;
        locals.var_xnedge_d_dn7 = assign47680_e61136_d_n7;
        locals.var_xnedge_d_dn8 = assign47680_e61136_d_n8;
        locals.var_xnedge_d_rv = 0.0;

        let assign47690_e61143: f64 = if ((locals.var_qseffedge < 0.001) && (locals.var_vdse_dc < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard1253 = assign47690_e61143;
        locals.var_guard1253_rv = 0.0;

        let assign47700_e61145: f64 = (-locals.var_xnedge_d);
        let assign47700_e61147: f64 = (assign47700_e61145 + locals.var_xnedge_s);
        let assign47700_e61149: f64 = (-230.25850929940458);
        let assign47700_e61150: f64 = if assign47700_e61147 > assign47700_e61149 { 1.0 } else { 0.0 };
        locals.var_guard1254 = assign47700_e61150;
        locals.var_guard1254_rv = 0.0;

        let (assign47710_e61162, assign47710_e61162_d_n5, assign47710_e61162_d_n6, assign47710_e61162_d_n7, assign47710_e61162_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) && (locals.var_guard1254 != 0.0)) {
        let assign47710_e61157: f64 = (-locals.var_xnedge_d);
        let assign47710_e61159: f64 = (assign47710_e61157 + locals.var_xnedge_s);
        let assign47710_e61160: f64 = (assign47710_e61159).exp();
        (assign47710_e61160, (assign47710_e61160 * ((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47710_e61162;
        locals.var_temp__blk936_dn5 = assign47710_e61162_d_n5;
        locals.var_temp__blk936_dn6 = assign47710_e61162_d_n6;
        locals.var_temp__blk936_dn7 = assign47710_e61162_d_n7;
        locals.var_temp__blk936_dn8 = assign47710_e61162_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign47720_e61205, assign47720_e61205_d_n5, assign47720_e61205_d_n6, assign47720_e61205_d_n7, assign47720_e61205_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) && (locals.var_guard1254 == 0.0)) {
        let assign47720_e61172: f64 = (-230.25850929940458);
        let assign47720_e61174: f64 = (-locals.var_xnedge_d);
        let assign47720_e61176: f64 = (assign47720_e61174 + locals.var_xnedge_s);
        let assign47720_e61177: f64 = (assign47720_e61172 - assign47720_e61176);
        let assign47720_e61181: f64 = (-230.25850929940458);
        let assign47720_e61183: f64 = (-locals.var_xnedge_d);
        let assign47720_e61185: f64 = (assign47720_e61183 + locals.var_xnedge_s);
        let assign47720_e61186: f64 = (assign47720_e61181 - assign47720_e61185);
        let assign47720_e61189: f64 = (-230.25850929940458);
        let assign47720_e61191: f64 = (-locals.var_xnedge_d);
        let assign47720_e61193: f64 = (assign47720_e61191 + locals.var_xnedge_s);
        let assign47720_e61194: f64 = (assign47720_e61189 - assign47720_e61193);
        let assign47720_e61196: f64 = (assign47720_e61194 * 0.3333333333333333);
        let assign47720_e61197: f64 = (1.0 + assign47720_e61196);
        let assign47720_e61198: f64 = (assign47720_e61186 * assign47720_e61197);
        let assign47720_e61199: f64 = (0.5 * assign47720_e61198);
        let assign47720_e61200: f64 = (1.0 + assign47720_e61199);
        let assign47720_e61201: f64 = (assign47720_e61177 * assign47720_e61200);
        let assign47720_e61202: f64 = (1.0 + assign47720_e61201);
        let assign47720_e61203: f64 = (1e-100 / assign47720_e61202);
        (assign47720_e61203, (-((1e-100 * (((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47720_e61205;
        locals.var_temp__blk936_dn5 = assign47720_e61205_d_n5;
        locals.var_temp__blk936_dn6 = assign47720_e61205_d_n6;
        locals.var_temp__blk936_dn7 = assign47720_e61205_d_n7;
        locals.var_temp__blk936_dn8 = assign47720_e61205_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign47730_e61215, assign47730_e61215_d_n5, assign47730_e61215_d_n6, assign47730_e61215_d_n7, assign47730_e61215_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) {
        let assign47730_e61212: f64 = (locals.var_temp__blk936 - 1.0);
        let assign47730_e61213: f64 = (locals.var_qseffedge * assign47730_e61212);
        (assign47730_e61213, ((locals.var_qseffedge_dn5 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn5)), ((locals.var_qseffedge_dn6 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn6)), ((locals.var_qseffedge_dn7 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn7)), ((locals.var_qseffedge_dn8 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_qdseffedge, locals.var_qdseffedge_dn5, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8,)
    }
};
        locals.var_qdseffedge = assign47730_e61215;
        locals.var_qdseffedge_dn5 = assign47730_e61215_d_n5;
        locals.var_qdseffedge_dn6 = assign47730_e61215_d_n6;
        locals.var_qdseffedge_dn7 = assign47730_e61215_d_n7;
        locals.var_qdseffedge_dn8 = assign47730_e61215_d_n8;
        locals.var_qdseffedge_rv = 0.0;

        let (assign47740_e61223, assign47740_e61223_d_n5, assign47740_e61223_d_n6, assign47740_e61223_d_n7, assign47740_e61223_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) {
        let assign47740_e61221: f64 = (locals.var_qdseffedge + locals.var_qseffedge);
        (assign47740_e61221, (locals.var_qdseffedge_dn5 + locals.var_qseffedge_dn5), (locals.var_qdseffedge_dn6 + locals.var_qseffedge_dn6), (locals.var_qdseffedge_dn7 + locals.var_qseffedge_dn7), (locals.var_qdseffedge_dn8 + locals.var_qseffedge_dn8),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8,)
    }
};
        locals.var_qdeffedge = assign47740_e61223;
        locals.var_qdeffedge_dn5 = assign47740_e61223_d_n5;
        locals.var_qdeffedge_dn6 = assign47740_e61223_d_n6;
        locals.var_qdeffedge_dn7 = assign47740_e61223_d_n7;
        locals.var_qdeffedge_dn8 = assign47740_e61223_d_n8;
        locals.var_qdeffedge_rv = 0.0;

        let (assign47750_e61232, assign47750_e61232_d_n5, assign47750_e61232_d_n6, assign47750_e61232_d_n7, assign47750_e61232_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47750_e61230: f64 = (locals.var_xbedge + locals.var_xnedge_d);
        (assign47750_e61230, (locals.var_xbedge_dn5 + locals.var_xnedge_d_dn5), (locals.var_xbedge_dn6 + locals.var_xnedge_d_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_d_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_d_dn8),)
    } else {
        (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn5, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8,)
    }
};
        locals.var_q_edge_xsth = assign47750_e61232;
        locals.var_q_edge_xsth_dn5 = assign47750_e61232_d_n5;
        locals.var_q_edge_xsth_dn6 = assign47750_e61232_d_n6;
        locals.var_q_edge_xsth_dn7 = assign47750_e61232_d_n7;
        locals.var_q_edge_xsth_dn8 = assign47750_e61232_d_n8;
        locals.var_q_edge_xsth_rv = 0.0;

        let (assign47760_e61244, assign47760_e61244_d_n5, assign47760_e61244_d_n6, assign47760_e61244_d_n7, assign47760_e61244_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47760_e61240: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47760_e61241: f64 = (locals.var_gfedge * assign47760_e61240);
        let assign47760_e61242: f64 = (locals.var_q_edge_xsth + assign47760_e61241);
        (assign47760_e61242, (locals.var_q_edge_xsth_dn5 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47760_e61240)))),)
    } else {
        (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn5, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8,)
    }
};
        locals.var_q_edge_xth0 = assign47760_e61244;
        locals.var_q_edge_xth0_dn5 = assign47760_e61244_d_n5;
        locals.var_q_edge_xth0_dn6 = assign47760_e61244_d_n6;
        locals.var_q_edge_xth0_dn7 = assign47760_e61244_d_n7;
        locals.var_q_edge_xth0_dn8 = assign47760_e61244_d_n8;
        locals.var_q_edge_xth0_rv = 0.0;

        let (assign47770_e61253, assign47770_e61253_d_n5, assign47770_e61253_d_n6, assign47770_e61253_d_n7, assign47770_e61253_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47770_e61251: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
        (assign47770_e61251, (locals.var_q_edge_xth0_dn5 + locals.var_dxthedge_dn5), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8),)
    } else {
        (locals.var_q_edge_xth, locals.var_q_edge_xth_dn5, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8,)
    }
};
        locals.var_q_edge_xth = assign47770_e61253;
        locals.var_q_edge_xth_dn5 = assign47770_e61253_d_n5;
        locals.var_q_edge_xth_dn6 = assign47770_e61253_d_n6;
        locals.var_q_edge_xth_dn7 = assign47770_e61253_d_n7;
        locals.var_q_edge_xth_dn8 = assign47770_e61253_d_n8;
        locals.var_q_edge_xth_rv = 0.0;

        let (assign47780_e61267, assign47780_e61267_d_n5, assign47780_e61267_d_n6, assign47780_e61267_d_n7, assign47780_e61267_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47780_e61262: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47780_e61263: f64 = (2.0 * assign47780_e61262);
        let assign47780_e61264: f64 = (locals.var_gfedge / assign47780_e61263);
        let assign47780_e61265: f64 = (1.0 + assign47780_e61264);
        (assign47780_e61265, (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))),)
    } else {
        (locals.var_q_edge_n, locals.var_q_edge_n_dn5, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8,)
    }
};
        locals.var_q_edge_n = assign47780_e61267;
        locals.var_q_edge_n_dn5 = assign47780_e61267_d_n5;
        locals.var_q_edge_n_dn6 = assign47780_e61267_d_n6;
        locals.var_q_edge_n_dn7 = assign47780_e61267_d_n7;
        locals.var_q_edge_n_dn8 = assign47780_e61267_d_n8;
        locals.var_q_edge_n_rv = 0.0;

        let (assign47790_e61276, assign47790_e61276_d_n5, assign47790_e61276_d_n6, assign47790_e61276_d_n7, assign47790_e61276_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47790_e61274: f64 = (1.0 / locals.var_q_edge_n);
        (assign47790_e61274, (-(locals.var_q_edge_n_dn5 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))),)
    } else {
        (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn5, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8,)
    }
};
        locals.var_q_edge_n_inv = assign47790_e61276;
        locals.var_q_edge_n_inv_dn5 = assign47790_e61276_d_n5;
        locals.var_q_edge_n_inv_dn6 = assign47790_e61276_d_n6;
        locals.var_q_edge_n_inv_dn7 = assign47790_e61276_d_n7;
        locals.var_q_edge_n_inv_dn8 = assign47790_e61276_d_n8;
        locals.var_q_edge_n_inv_rv = 0.0;

        let (assign47800_e61285, assign47800_e61285_d_n5, assign47800_e61285_d_n6, assign47800_e61285_d_n7, assign47800_e61285_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47800_e61283: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
        (assign47800_e61283, (locals.var_xgedge_dn5 - locals.var_q_edge_xth_dn5), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8),)
    } else {
        (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8,)
    }
};
        locals.var_q_edge_xgt = assign47800_e61285;
        locals.var_q_edge_xgt_dn5 = assign47800_e61285_d_n5;
        locals.var_q_edge_xgt_dn6 = assign47800_e61285_d_n6;
        locals.var_q_edge_xgt_dn7 = assign47800_e61285_d_n7;
        locals.var_q_edge_xgt_dn8 = assign47800_e61285_d_n8;
        locals.var_q_edge_xgt_rv = 0.0;

        let assign47810_e61288: f64 = (-12.0);
        let assign47810_e61289: f64 = if locals.var_q_edge_xgt > assign47810_e61288 { 1.0 } else { 0.0 };
        locals.var_guard1255 = assign47810_e61289;
        locals.var_guard1255_rv = 0.0;

        let (assign47820_e61302, assign47820_e61302_d_n5, assign47820_e61302_d_n6, assign47820_e61302_d_n7, assign47820_e61302_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47820_e61298: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47820_e61300: f64 = (assign47820_e61298 - 1.0);
        (assign47820_e61300, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8,)
    } else {
        (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn5, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8,)
    }
};
        locals.var_q_edge_xgt0 = assign47820_e61302;
        locals.var_q_edge_xgt0_dn5 = assign47820_e61302_d_n5;
        locals.var_q_edge_xgt0_dn6 = assign47820_e61302_d_n6;
        locals.var_q_edge_xgt0_dn7 = assign47820_e61302_d_n7;
        locals.var_q_edge_xgt0_dn8 = assign47820_e61302_d_n8;
        locals.var_q_edge_xgt0_rv = 0.0;

        let (assign47830_e61320, assign47830_e61320_d_n5, assign47830_e61320_d_n6, assign47830_e61320_d_n7, assign47830_e61320_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47830_e61313: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
        let assign47830_e61315: f64 = (assign47830_e61313 + 10.0);
        let assign47830_e61316: f64 = (assign47830_e61315).sqrt();
        let assign47830_e61317: f64 = (locals.var_q_edge_xgt0 + assign47830_e61316);
        let assign47830_e61318: f64 = (0.5 * assign47830_e61317);
        (assign47830_e61318, (0.5 * (locals.var_q_edge_xgt0_dn5 + (((locals.var_q_edge_xgt0_dn5 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn5)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47830_e61316)))),)
    } else {
        (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn5, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8,)
    }
};
        locals.var_q_edge_xgt0e = assign47830_e61320;
        locals.var_q_edge_xgt0e_dn5 = assign47830_e61320_d_n5;
        locals.var_q_edge_xgt0e_dn6 = assign47830_e61320_d_n6;
        locals.var_q_edge_xgt0e_dn7 = assign47830_e61320_d_n7;
        locals.var_q_edge_xgt0e_dn8 = assign47830_e61320_d_n8;
        locals.var_q_edge_xgt0e_rv = 0.0;

        let (assign47840_e61336, assign47840_e61336_d_n5, assign47840_e61336_d_n6, assign47840_e61336_d_n7, assign47840_e61336_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47840_e61330: f64 = (locals.var_q_edge_xgt0e).ln();
        let assign47840_e61331: f64 = (locals.var_q_edge_n * assign47840_e61330);
        let assign47840_e61332: f64 = (locals.var_q_edge_xgt - assign47840_e61331);
        let assign47840_e61334: f64 = (assign47840_e61332 + locals.var_lngfedge2);
        (assign47840_e61334, (locals.var_q_edge_xgt_dn5 - ((locals.var_q_edge_n_dn5 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn5 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))),)
    } else {
        (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn5, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8,)
    }
};
        locals.var_q_edge_qi0si = assign47840_e61336;
        locals.var_q_edge_qi0si_dn5 = assign47840_e61336_d_n5;
        locals.var_q_edge_qi0si_dn6 = assign47840_e61336_d_n6;
        locals.var_q_edge_qi0si_dn7 = assign47840_e61336_d_n7;
        locals.var_q_edge_qi0si_dn8 = assign47840_e61336_d_n8;
        locals.var_q_edge_qi0si_rv = 0.0;

        let (assign47850_e61354, assign47850_e61354_d_n5, assign47850_e61354_d_n6, assign47850_e61354_d_n7, assign47850_e61354_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47850_e61347: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
        let assign47850_e61349: f64 = (assign47850_e61347 + 2.0);
        let assign47850_e61350: f64 = (assign47850_e61349).sqrt();
        let assign47850_e61351: f64 = (locals.var_q_edge_qi0si + assign47850_e61350);
        let assign47850_e61352: f64 = (0.5 * assign47850_e61351);
        (assign47850_e61352, (0.5 * (locals.var_q_edge_qi0si_dn5 + (((locals.var_q_edge_qi0si_dn5 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn5)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47850_e61350)))),)
    } else {
        (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn5, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8,)
    }
};
        locals.var_q_edge_qi0 = assign47850_e61354;
        locals.var_q_edge_qi0_dn5 = assign47850_e61354_d_n5;
        locals.var_q_edge_qi0_dn6 = assign47850_e61354_d_n6;
        locals.var_q_edge_qi0_dn7 = assign47850_e61354_d_n7;
        locals.var_q_edge_qi0_dn8 = assign47850_e61354_d_n8;
        locals.var_q_edge_qi0_rv = 0.0;

        let assign47860_e61357: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47860_e61359: f64 = if assign47860_e61357 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1256 = assign47860_e61359;
        locals.var_guard1256_rv = 0.0;

        let (assign47870_e61373, assign47870_e61373_d_n5, assign47870_e61373_d_n6, assign47870_e61373_d_n7, assign47870_e61373_d_n8,) = {
    if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) && (locals.var_guard1256 != 0.0)) {
        let assign47870_e61370: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47870_e61371: f64 = (assign47870_e61370).exp();
        (assign47870_e61371, (assign47870_e61371 * (locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8,)
    }
};
        locals.var_q_edge_exp_x = assign47870_e61373;
        locals.var_q_edge_exp_x_dn5 = assign47870_e61373_d_n5;
        locals.var_q_edge_exp_x_dn6 = assign47870_e61373_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47870_e61373_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47870_e61373_d_n8;
        locals.var_q_edge_exp_x_rv = 0.0;

        let (assign47880_e61413, assign47880_e61413_d_n5, assign47880_e61413_d_n6, assign47880_e61413_d_n7, assign47880_e61413_d_n8,) = {
    if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) && (locals.var_guard1256 == 0.0)) {
        let assign47880_e61387: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47880_e61389: f64 = (assign47880_e61387 - 230.25850929940458);
        let assign47880_e61394: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47880_e61396: f64 = (assign47880_e61394 - 230.25850929940458);
        let assign47880_e61400: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47880_e61402: f64 = (assign47880_e61400 - 230.25850929940458);
        let assign47880_e61404: f64 = (assign47880_e61402 * 0.3333333333333333);
        let assign47880_e61405: f64 = (1.0 + assign47880_e61404);
        let assign47880_e61406: f64 = (assign47880_e61396 * assign47880_e61405);
        let assign47880_e61407: f64 = (0.5 * assign47880_e61406);
        let assign47880_e61408: f64 = (1.0 + assign47880_e61407);
        let assign47880_e61409: f64 = (assign47880_e61389 * assign47880_e61408);
        let assign47880_e61410: f64 = (1.0 + assign47880_e61409);
        let assign47880_e61411: f64 = (1e100 * assign47880_e61410);
        (assign47880_e61411, (1e100 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8,)
    }
};
        locals.var_q_edge_exp_x = assign47880_e61413;
        locals.var_q_edge_exp_x_dn5 = assign47880_e61413_d_n5;
        locals.var_q_edge_exp_x_dn6 = assign47880_e61413_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47880_e61413_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47880_e61413_d_n8;
        locals.var_q_edge_exp_x_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47890_e61424, assign47890_e61424_d_n5, assign47890_e61424_d_n6, assign47890_e61424_d_n7, assign47890_e61424_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47890_e61422: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
        (assign47890_e61422, (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn5), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8),)
    } else {
        (locals.var_q_edge_d0, locals.var_q_edge_d0_dn5, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8,)
    }
};
        locals.var_q_edge_d0 = assign47890_e61424;
        locals.var_q_edge_d0_dn5 = assign47890_e61424_d_n5;
        locals.var_q_edge_d0_dn6 = assign47890_e61424_d_n6;
        locals.var_q_edge_d0_dn7 = assign47890_e61424_d_n7;
        locals.var_q_edge_d0_dn8 = assign47890_e61424_d_n8;
        locals.var_q_edge_d0_rv = 0.0;

        let (assign47900_e61435, assign47900_e61435_d_n5, assign47900_e61435_d_n6, assign47900_e61435_d_n7, assign47900_e61435_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47900_e61433: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
        (assign47900_e61433, if locals.var_q_edge_n_inv_dn5 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn5)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn5 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn5 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) },)
    } else {
        (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn5, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8,)
    }
};
        locals.var_q_edge_d0p = assign47900_e61435;
        locals.var_q_edge_d0p_dn5 = assign47900_e61435_d_n5;
        locals.var_q_edge_d0p_dn6 = assign47900_e61435_d_n6;
        locals.var_q_edge_d0p_dn7 = assign47900_e61435_d_n7;
        locals.var_q_edge_d0p_dn8 = assign47900_e61435_d_n8;
        locals.var_q_edge_d0p_rv = 0.0;

        let (assign47910_e61456, assign47910_e61456_d_n5, assign47910_e61456_d_n6, assign47910_e61456_d_n7, assign47910_e61456_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47910_e61444: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
        let assign47910_e61448: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
        let assign47910_e61449: f64 = (2.0 * assign47910_e61448);
        let assign47910_e61451: f64 = (assign47910_e61449 - locals.var_q_edge_d0p);
        let assign47910_e61453: f64 = (assign47910_e61451 * locals.var_q_edge_d0p);
        let assign47910_e61454: f64 = (assign47910_e61444 + assign47910_e61453);
        (assign47910_e61454, (((locals.var_q_edge_n_dn5 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn5)) + ((((2.0 * (locals.var_q_edge_qi0_dn5 + locals.var_q_edge_n_dn5)) - locals.var_q_edge_d0p_dn5) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn5))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn8))),)
    } else {
        (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn5, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8,)
    }
};
        locals.var_q_edge_sqerr = assign47910_e61456;
        locals.var_q_edge_sqerr_dn5 = assign47910_e61456_d_n5;
        locals.var_q_edge_sqerr_dn6 = assign47910_e61456_d_n6;
        locals.var_q_edge_sqerr_dn7 = assign47910_e61456_d_n7;
        locals.var_q_edge_sqerr_dn8 = assign47910_e61456_d_n8;
        locals.var_q_edge_sqerr_rv = 0.0;

        let (assign47920_e61474, assign47920_e61474_d_n5, assign47920_e61474_d_n6, assign47920_e61474_d_n7, assign47920_e61474_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47920_e61465: f64 = (locals.var_q_edge_sqerr).sqrt();
        let assign47920_e61467: f64 = (assign47920_e61465 - locals.var_q_edge_n);
        let assign47920_e61469: f64 = (assign47920_e61467 / locals.var_q_edge_d0p);
        let assign47920_e61471: f64 = (assign47920_e61469 - 1.0);
        let assign47920_e61472: f64 = (locals.var_q_edge_n * assign47920_e61471);
        (assign47920_e61472, ((locals.var_q_edge_n_dn5 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn5 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn5) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn5)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))),)
    } else {
        (locals.var_q_edge_errq, locals.var_q_edge_errq_dn5, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8,)
    }
};
        locals.var_q_edge_errq = assign47920_e61474;
        locals.var_q_edge_errq_dn5 = assign47920_e61474_d_n5;
        locals.var_q_edge_errq_dn6 = assign47920_e61474_d_n6;
        locals.var_q_edge_errq_dn7 = assign47920_e61474_d_n7;
        locals.var_q_edge_errq_dn8 = assign47920_e61474_d_n8;
        locals.var_q_edge_errq_rv = 0.0;

        let (assign47930_e61485, assign47930_e61485_d_n5, assign47930_e61485_d_n6, assign47930_e61485_d_n7, assign47930_e61485_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47930_e61483: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
        (assign47930_e61483, (locals.var_q_edge_qi0_dn5 - locals.var_q_edge_errq_dn5), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8,)
    }
};
        locals.var_qdeffedge = assign47930_e61485;
        locals.var_qdeffedge_dn5 = assign47930_e61485_d_n5;
        locals.var_qdeffedge_dn6 = assign47930_e61485_d_n6;
        locals.var_qdeffedge_dn7 = assign47930_e61485_d_n7;
        locals.var_qdeffedge_dn8 = assign47930_e61485_d_n8;
        locals.var_qdeffedge_rv = 0.0;

        let assign47940_e61489: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47940_e61490: f64 = (locals.var_q_edge_n_inv * assign47940_e61489);
        let assign47940_e61492: f64 = (-230.25850929940458);
        let assign47940_e61493: f64 = if assign47940_e61490 > assign47940_e61492 { 1.0 } else { 0.0 };
        locals.var_guard1257 = assign47940_e61493;
        locals.var_guard1257_rv = 0.0;

        let (assign47950_e61510, assign47950_e61510_d_n5, assign47950_e61510_d_n6, assign47950_e61510_d_n7, assign47950_e61510_d_n8,) = {
    if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 == 0.0)) && (locals.var_guard1257 != 0.0)) {
        let assign47950_e61506: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47950_e61507: f64 = (locals.var_q_edge_n_inv * assign47950_e61506);
        let assign47950_e61508: f64 = (assign47950_e61507).exp();
        (assign47950_e61508, (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn5 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn6 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn7 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn8 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8,)
    }
};
        locals.var_qdeffedge = assign47950_e61510;
        locals.var_qdeffedge_dn5 = assign47950_e61510_d_n5;
        locals.var_qdeffedge_dn6 = assign47950_e61510_d_n6;
        locals.var_qdeffedge_dn7 = assign47950_e61510_d_n7;
        locals.var_qdeffedge_dn8 = assign47950_e61510_d_n8;
        locals.var_qdeffedge_rv = 0.0;

        let (assign47960_e61560, assign47960_e61560_d_n5, assign47960_e61560_d_n6, assign47960_e61560_d_n7, assign47960_e61560_d_n8,) = {
    if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 == 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47960_e61524: f64 = (-230.25850929940458);
        let assign47960_e61528: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47960_e61529: f64 = (locals.var_q_edge_n_inv * assign47960_e61528);
        let assign47960_e61530: f64 = (assign47960_e61524 - assign47960_e61529);
        let assign47960_e61534: f64 = (-230.25850929940458);
        let assign47960_e61538: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47960_e61539: f64 = (locals.var_q_edge_n_inv * assign47960_e61538);
        let assign47960_e61540: f64 = (assign47960_e61534 - assign47960_e61539);
        let assign47960_e61543: f64 = (-230.25850929940458);
        let assign47960_e61547: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47960_e61548: f64 = (locals.var_q_edge_n_inv * assign47960_e61547);
        let assign47960_e61549: f64 = (assign47960_e61543 - assign47960_e61548);
        let assign47960_e61551: f64 = (assign47960_e61549 * 0.3333333333333333);
        let assign47960_e61552: f64 = (1.0 + assign47960_e61551);
        let assign47960_e61553: f64 = (assign47960_e61540 * assign47960_e61552);
        let assign47960_e61554: f64 = (0.5 * assign47960_e61553);
        let assign47960_e61555: f64 = (1.0 + assign47960_e61554);
        let assign47960_e61556: f64 = (assign47960_e61530 * assign47960_e61555);
        let assign47960_e61557: f64 = (1.0 + assign47960_e61556);
        let assign47960_e61558: f64 = (1e-100 / assign47960_e61557);
        (assign47960_e61558, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8,)
    }
};
        locals.var_qdeffedge = assign47960_e61560;
        locals.var_qdeffedge_dn5 = assign47960_e61560_d_n5;
        locals.var_qdeffedge_dn6 = assign47960_e61560_d_n6;
        locals.var_qdeffedge_dn7 = assign47960_e61560_d_n7;
        locals.var_qdeffedge_dn8 = assign47960_e61560_d_n8;
        locals.var_qdeffedge_rv = 0.0;

        let (assign47970_e61569, assign47970_e61569_d_n5, assign47970_e61569_d_n6, assign47970_e61569_d_n7, assign47970_e61569_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47970_e61567: f64 = (locals.var_qdeffedge - locals.var_qseffedge);
        (assign47970_e61567, (locals.var_qdeffedge_dn5 - locals.var_qseffedge_dn5), (locals.var_qdeffedge_dn6 - locals.var_qseffedge_dn6), (locals.var_qdeffedge_dn7 - locals.var_qseffedge_dn7), (locals.var_qdeffedge_dn8 - locals.var_qseffedge_dn8),)
    } else {
        (locals.var_qdseffedge, locals.var_qdseffedge_dn5, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8,)
    }
};
        locals.var_qdseffedge = assign47970_e61569;
        locals.var_qdseffedge_dn5 = assign47970_e61569_d_n5;
        locals.var_qdseffedge_dn6 = assign47970_e61569_d_n6;
        locals.var_qdseffedge_dn7 = assign47970_e61569_d_n7;
        locals.var_qdseffedge_dn8 = assign47970_e61569_d_n8;
        locals.var_qdseffedge_rv = 0.0;

        let (assign47980_e61577, assign47980_e61577_d_n5, assign47980_e61577_d_n6, assign47980_e61577_d_n7, assign47980_e61577_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47980_e61574: f64 = (locals.var_qdeffedge + locals.var_qseffedge);
        let assign47980_e61575: f64 = (0.5 * assign47980_e61574);
        (assign47980_e61575, (0.5 * (locals.var_qdeffedge_dn5 + locals.var_qseffedge_dn5)), (0.5 * (locals.var_qdeffedge_dn6 + locals.var_qseffedge_dn6)), (0.5 * (locals.var_qdeffedge_dn7 + locals.var_qseffedge_dn7)), (0.5 * (locals.var_qdeffedge_dn8 + locals.var_qseffedge_dn8)),)
    } else {
        (locals.var_qmeffedge, locals.var_qmeffedge_dn5, locals.var_qmeffedge_dn6, locals.var_qmeffedge_dn7, locals.var_qmeffedge_dn8,)
    }
};
        locals.var_qmeffedge = assign47980_e61577;
        locals.var_qmeffedge_dn5 = assign47980_e61577_d_n5;
        locals.var_qmeffedge_dn6 = assign47980_e61577_d_n6;
        locals.var_qmeffedge_dn7 = assign47980_e61577_d_n7;
        locals.var_qmeffedge_dn8 = assign47980_e61577_d_n8;
        locals.var_qmeffedge_rv = 0.0;

        let (assign47990_e61590, assign47990_e61590_d_n5, assign47990_e61590_d_n6, assign47990_e61590_d_n7, assign47990_e61590_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47990_e61581: f64 = (locals.var_xgedge - locals.var_qmeffedge);
        let (assign47990_e61588, assign47990_e61588_d_n5, assign47990_e61588_d_n6, assign47990_e61588_d_n7, assign47990_e61588_d_n8,) = {
            if (assign47990_e61581 > 1e-40) {
                let assign47990_e61586: f64 = (locals.var_xgedge - locals.var_qmeffedge);
                (assign47990_e61586, (locals.var_xgedge_dn5 - locals.var_qmeffedge_dn5), (locals.var_xgedge_dn6 - locals.var_qmeffedge_dn6), (locals.var_xgedge_dn7 - locals.var_qmeffedge_dn7), (locals.var_xgedge_dn8 - locals.var_qmeffedge_dn8),)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign47990_e61588, assign47990_e61588_d_n5, assign47990_e61588_d_n6, assign47990_e61588_d_n7, assign47990_e61588_d_n8,)
    } else {
        (locals.var_dsqredge, locals.var_dsqredge_dn5, locals.var_dsqredge_dn6, locals.var_dsqredge_dn7, locals.var_dsqredge_dn8,)
    }
};
        locals.var_dsqredge = assign47990_e61590;
        locals.var_dsqredge_dn5 = assign47990_e61590_d_n5;
        locals.var_dsqredge_dn6 = assign47990_e61590_d_n6;
        locals.var_dsqredge_dn7 = assign47990_e61590_d_n7;
        locals.var_dsqredge_dn8 = assign47990_e61590_d_n8;
        locals.var_dsqredge_rv = 0.0;

        let (assign48000_e61605, assign48000_e61605_d_n5, assign48000_e61605_d_n6, assign48000_e61605_d_n7, assign48000_e61605_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign48000_e61595: f64 = (0.5 * locals.var_gfedge);
        let assign48000_e61599: f64 = (0.25 * locals.var_gfedge2);
        let assign48000_e61600: f64 = (locals.var_dsqredge + assign48000_e61599);
        let assign48000_e61601: f64 = (assign48000_e61600).sqrt();
        let assign48000_e61602: f64 = (assign48000_e61595 / assign48000_e61601);
        let assign48000_e61603: f64 = (1.0 - assign48000_e61602);
        (assign48000_e61603, (-(-((assign48000_e61595 * (locals.var_dsqredge_dn5 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn6 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn7 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn8 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))),)
    } else {
        (locals.var_alphabmedge, locals.var_alphabmedge_dn5, locals.var_alphabmedge_dn6, locals.var_alphabmedge_dn7, locals.var_alphabmedge_dn8,)
    }
};
        locals.var_alphabmedge = assign48000_e61605;
        locals.var_alphabmedge_dn5 = assign48000_e61605_d_n5;
        locals.var_alphabmedge_dn6 = assign48000_e61605_d_n6;
        locals.var_alphabmedge_dn7 = assign48000_e61605_d_n7;
        locals.var_alphabmedge_dn8 = assign48000_e61605_d_n8;
        locals.var_alphabmedge_rv = 0.0;

        let (assign48010_e61624, assign48010_e61624_d_n5, assign48010_e61624_d_n6, assign48010_e61624_d_n7, assign48010_e61624_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign48010_e61608: f64 = (-locals.var_betedge_i);
        let assign48010_e61610: f64 = (assign48010_e61608 * locals.var_phit1edge);
        let assign48010_e61612: f64 = (assign48010_e61610 * locals.var_phit1edge);
        let assign48010_e61615: f64 = (locals.var_alphabmedge * locals.var_qmeffedge);
        let assign48010_e61617: f64 = (assign48010_e61615 + 1.0);
        let assign48010_e61618: f64 = (assign48010_e61612 * assign48010_e61617);
        let assign48010_e61620: f64 = (assign48010_e61618 * locals.var_qdseffedge);
        let assign48010_e61622: f64 = (assign48010_e61620 / locals.var_gmob_dc);
        (assign48010_e61622, ((((((((((assign48010_e61608 * locals.var_phit1edge_dn5) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn5)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn5 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn5)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn5)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn5)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn6) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn6)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn6 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn6)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn6)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn7) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn7)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn7 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn7)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn7)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn8) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn8)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn8 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn8)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn8)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_i_dsedge, locals.var_i_dsedge_dn5, locals.var_i_dsedge_dn6, locals.var_i_dsedge_dn7, locals.var_i_dsedge_dn8,)
    }
};
        locals.var_i_dsedge = assign48010_e61624;
        locals.var_i_dsedge_dn5 = assign48010_e61624_d_n5;
        locals.var_i_dsedge_dn6 = assign48010_e61624_d_n6;
        locals.var_i_dsedge_dn7 = assign48010_e61624_d_n7;
        locals.var_i_dsedge_dn8 = assign48010_e61624_d_n8;
        locals.var_i_dsedge_rv = 0.0;

        locals.var_mavl = 0.0;
        locals.var_mavl_dn5 = 0.0;
        locals.var_mavl_dn6 = 0.0;
        locals.var_mavl_dn7 = 0.0;
        locals.var_mavl_dn8 = 0.0;
        locals.var_mavl_rv = 0.0;

        locals.var_iimpact = 0.0;
        locals.var_iimpact_dn5 = 0.0;
        locals.var_iimpact_dn6 = 0.0;
        locals.var_iimpact_dn7 = 0.0;
        locals.var_iimpact_dn8 = 0.0;
        locals.var_iimpact_rv = 0.0;

        let assign48040_e61633: f64 = if ((locals.var_xg_dc > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1258 = assign48040_e61633;
        locals.var_guard1258_rv = 0.0;

        let (assign48050_e61641, assign48050_e61641_d_n5, assign48050_e61641_d_n6, assign48050_e61641_d_n7, assign48050_e61641_d_n8,) = {
    if (locals.var_guard1258 != 0.0) {
        let assign48050_e61638: f64 = (locals.var_a3_i * locals.var_dps_dc);
        let assign48050_e61639: f64 = (locals.var_v_ds - assign48050_e61638);
        (assign48050_e61639, (-(locals.var_a3_i * locals.var_dps_dc_dn5)), (locals.var_v_ds_dn6 - (locals.var_a3_i * locals.var_dps_dc_dn6)), (locals.var_v_ds_dn7 - (locals.var_a3_i * locals.var_dps_dc_dn7)), (-(locals.var_a3_i * locals.var_dps_dc_dn8)),)
    } else {
        (locals.var_delvsat, locals.var_delvsat_dn5, locals.var_delvsat_dn6, locals.var_delvsat_dn7, locals.var_delvsat_dn8,)
    }
};
        locals.var_delvsat = assign48050_e61641;
        locals.var_delvsat_dn5 = assign48050_e61641_d_n5;
        locals.var_delvsat_dn6 = assign48050_e61641_d_n6;
        locals.var_delvsat_dn7 = assign48050_e61641_d_n7;
        locals.var_delvsat_dn8 = assign48050_e61641_d_n8;
        locals.var_delvsat_rv = 0.0;

        let assign48060_e61644: f64 = if locals.var_delvsat > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1259 = assign48060_e61644;
        locals.var_guard1259_rv = 0.0;

        let (assign48070_e61665, assign48070_e61665_d_n5, assign48070_e61665_d_n6, assign48070_e61665_d_n7, assign48070_e61665_d_n8,) = {
    if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
        let assign48070_e61653: f64 = (locals.var_phib_dc + locals.var_vsbstar_dc);
        let assign48070_e61654: f64 = (assign48070_e61653).sqrt();
        let assign48070_e61656: f64 = (assign48070_e61654 - locals.var_sqrt_phib_dc);
        let assign48070_e61657: f64 = (locals.var_a4_i * assign48070_e61656);
        let assign48070_e61658: f64 = (1.0 + assign48070_e61657);
        let assign48070_e61661: f64 = (locals.var_delvsat + 1e-30);
        let assign48070_e61662: f64 = (assign48070_e61658 / assign48070_e61661);
        let assign48070_e61663: f64 = (locals.var_a2_t * assign48070_e61662);
        (assign48070_e61663, (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn5 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn5)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn6 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn6)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn7 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn7)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn8 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn8)) / (assign48070_e61661 * assign48070_e61661))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48070_e61665;
        locals.var_temp2_dn5 = assign48070_e61665_d_n5;
        locals.var_temp2_dn6 = assign48070_e61665_d_n6;
        locals.var_temp2_dn7 = assign48070_e61665_d_n7;
        locals.var_temp2_dn8 = assign48070_e61665_d_n8;
        locals.var_temp2_rv = 0.0;

        let assign48080_e61667: f64 = (-locals.var_temp2);
        let assign48080_e61668: f64 = (assign48080_e61667).abs();
        let assign48080_e61670: f64 = if assign48080_e61668 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1260 = assign48080_e61670;
        locals.var_guard1260_rv = 0.0;

        let (assign48090_e61680, assign48090_e61680_d_n5, assign48090_e61680_d_n6, assign48090_e61680_d_n7, assign48090_e61680_d_n8,) = {
    if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 != 0.0)) {
        let assign48090_e61677: f64 = (-locals.var_temp2);
        let assign48090_e61678: f64 = (assign48090_e61677).exp();
        (assign48090_e61678, (assign48090_e61678 * (-locals.var_temp2_dn5)), (assign48090_e61678 * (-locals.var_temp2_dn6)), (assign48090_e61678 * (-locals.var_temp2_dn7)), (assign48090_e61678 * (-locals.var_temp2_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48090_e61680;
        locals.var_temp__blk936_dn5 = assign48090_e61680_d_n5;
        locals.var_temp__blk936_dn6 = assign48090_e61680_d_n6;
        locals.var_temp__blk936_dn7 = assign48090_e61680_d_n7;
        locals.var_temp__blk936_dn8 = assign48090_e61680_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign48100_e61682: f64 = (-locals.var_temp2);
        let assign48100_e61684: f64 = if assign48100_e61682 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1261 = assign48100_e61684;
        locals.var_guard1261_rv = 0.0;

        let (assign48110_e61723, assign48110_e61723_d_n5, assign48110_e61723_d_n6, assign48110_e61723_d_n7, assign48110_e61723_d_n8,) = {
    if ((((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 == 0.0)) && (locals.var_guard1261 != 0.0)) {
        let assign48110_e61696: f64 = (-230.25850929940458);
        let assign48110_e61698: f64 = (-locals.var_temp2);
        let assign48110_e61699: f64 = (assign48110_e61696 - assign48110_e61698);
        let assign48110_e61703: f64 = (-230.25850929940458);
        let assign48110_e61705: f64 = (-locals.var_temp2);
        let assign48110_e61706: f64 = (assign48110_e61703 - assign48110_e61705);
        let assign48110_e61709: f64 = (-230.25850929940458);
        let assign48110_e61711: f64 = (-locals.var_temp2);
        let assign48110_e61712: f64 = (assign48110_e61709 - assign48110_e61711);
        let assign48110_e61714: f64 = (assign48110_e61712 * 0.3333333333333333);
        let assign48110_e61715: f64 = (1.0 + assign48110_e61714);
        let assign48110_e61716: f64 = (assign48110_e61706 * assign48110_e61715);
        let assign48110_e61717: f64 = (0.5 * assign48110_e61716);
        let assign48110_e61718: f64 = (1.0 + assign48110_e61717);
        let assign48110_e61719: f64 = (assign48110_e61699 * assign48110_e61718);
        let assign48110_e61720: f64 = (1.0 + assign48110_e61719);
        let assign48110_e61721: f64 = (1e-100 / assign48110_e61720);
        (assign48110_e61721, (-((1e-100 * (((-(-locals.var_temp2_dn5)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn5)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn5)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn6)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn6)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn6)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn7)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn7)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn7)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn8)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn8)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn8)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48110_e61723;
        locals.var_temp__blk936_dn5 = assign48110_e61723_d_n5;
        locals.var_temp__blk936_dn6 = assign48110_e61723_d_n6;
        locals.var_temp__blk936_dn7 = assign48110_e61723_d_n7;
        locals.var_temp__blk936_dn8 = assign48110_e61723_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign48120_e61760, assign48120_e61760_d_n5, assign48120_e61760_d_n6, assign48120_e61760_d_n7, assign48120_e61760_d_n8,) = {
    if ((((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 == 0.0)) && (locals.var_guard1261 == 0.0)) {
        let assign48120_e61736: f64 = (-locals.var_temp2);
        let assign48120_e61738: f64 = (assign48120_e61736 - 230.25850929940458);
        let assign48120_e61742: f64 = (-locals.var_temp2);
        let assign48120_e61744: f64 = (assign48120_e61742 - 230.25850929940458);
        let assign48120_e61747: f64 = (-locals.var_temp2);
        let assign48120_e61749: f64 = (assign48120_e61747 - 230.25850929940458);
        let assign48120_e61751: f64 = (assign48120_e61749 * 0.3333333333333333);
        let assign48120_e61752: f64 = (1.0 + assign48120_e61751);
        let assign48120_e61753: f64 = (assign48120_e61744 * assign48120_e61752);
        let assign48120_e61754: f64 = (0.5 * assign48120_e61753);
        let assign48120_e61755: f64 = (1.0 + assign48120_e61754);
        let assign48120_e61756: f64 = (assign48120_e61738 * assign48120_e61755);
        let assign48120_e61757: f64 = (1.0 + assign48120_e61756);
        let assign48120_e61758: f64 = (1e100 * assign48120_e61757);
        (assign48120_e61758, (1e100 * (((-locals.var_temp2_dn5) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn5) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn6) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn6) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn7) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn7) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn8) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn8) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48120_e61760;
        locals.var_temp__blk936_dn5 = assign48120_e61760_d_n5;
        locals.var_temp__blk936_dn6 = assign48120_e61760_d_n6;
        locals.var_temp__blk936_dn7 = assign48120_e61760_d_n7;
        locals.var_temp__blk936_dn8 = assign48120_e61760_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign48130_e61770, assign48130_e61770_d_n5, assign48130_e61770_d_n6, assign48130_e61770_d_n7, assign48130_e61770_d_n8,) = {
    if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
        let assign48130_e61767: f64 = (locals.var_delvsat * locals.var_temp__blk936);
        let assign48130_e61768: f64 = (locals.var_a1_i * assign48130_e61767);
        (assign48130_e61768, (locals.var_a1_i * ((locals.var_delvsat_dn5 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn5))), (locals.var_a1_i * ((locals.var_delvsat_dn6 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn6))), (locals.var_a1_i * ((locals.var_delvsat_dn7 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn7))), (locals.var_a1_i * ((locals.var_delvsat_dn8 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_mavl, locals.var_mavl_dn5, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8,)
    }
};
        locals.var_mavl = assign48130_e61770;
        locals.var_mavl_dn5 = assign48130_e61770_d_n5;
        locals.var_mavl_dn6 = assign48130_e61770_d_n6;
        locals.var_mavl_dn7 = assign48130_e61770_d_n7;
        locals.var_mavl_dn8 = assign48130_e61770_d_n8;
        locals.var_mavl_rv = 0.0;

        let (assign48140_e61780, assign48140_e61780_d_n5, assign48140_e61780_d_n6, assign48140_e61780_d_n7, assign48140_e61780_d_n8,) = {
    if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
        let assign48140_e61777: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let assign48140_e61778: f64 = (locals.var_mavl * assign48140_e61777);
        (assign48140_e61778, ((locals.var_mavl_dn5 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn5 + locals.var_i_dsedge_dn5))), ((locals.var_mavl_dn6 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6))), ((locals.var_mavl_dn7 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7))), ((locals.var_mavl_dn8 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn5, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8,)
    }
};
        locals.var_iimpact = assign48140_e61780;
        locals.var_iimpact_dn5 = assign48140_e61780_d_n5;
        locals.var_iimpact_dn6 = assign48140_e61780_d_n6;
        locals.var_iimpact_dn7 = assign48140_e61780_d_n7;
        locals.var_iimpact_dn8 = assign48140_e61780_d_n8;
        locals.var_iimpact_rv = 0.0;

        let assign48150_e61784: f64 = (0.5 * locals.var_imaxii_i);
        let assign48150_e61785: f64 = if locals.var_iimpact > assign48150_e61784 { 1.0 } else { 0.0 };
        locals.var_guard1262 = assign48150_e61785;
        locals.var_guard1262_rv = 0.0;

        let (assign48160_e61799, assign48160_e61799_d_n5, assign48160_e61799_d_n6, assign48160_e61799_d_n7, assign48160_e61799_d_n8,) = {
    if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1262 != 0.0)) {
        let assign48160_e61793: f64 = (2.0 * locals.var_iimpact);
        let assign48160_e61795: f64 = (assign48160_e61793 / locals.var_imaxii_i);
        let assign48160_e61797: f64 = (assign48160_e61795 - 1.0);
        (assign48160_e61797, ((2.0 * locals.var_iimpact_dn5) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn6) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn7) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn8) / locals.var_imaxii_i),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48160_e61799;
        locals.var_temp__blk936_dn5 = assign48160_e61799_d_n5;
        locals.var_temp__blk936_dn6 = assign48160_e61799_d_n6;
        locals.var_temp__blk936_dn7 = assign48160_e61799_d_n7;
        locals.var_temp__blk936_dn8 = assign48160_e61799_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign48170_e61820, assign48170_e61820_d_n5, assign48170_e61820_d_n6, assign48170_e61820_d_n7, assign48170_e61820_d_n8,) = {
    if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1262 != 0.0)) {
        let assign48170_e61807: f64 = (0.5 * locals.var_imaxii_i);
        let assign48170_e61813: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign48170_e61814: f64 = (1.0 + assign48170_e61813);
        let assign48170_e61815: f64 = (assign48170_e61814).sqrt();
        let assign48170_e61816: f64 = (locals.var_temp__blk936 / assign48170_e61815);
        let assign48170_e61817: f64 = (1.0 + assign48170_e61816);
        let assign48170_e61818: f64 = (assign48170_e61807 * assign48170_e61817);
        (assign48170_e61818, (assign48170_e61807 * (((locals.var_temp__blk936_dn5 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn6 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn7 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn8 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn5, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8,)
    }
};
        locals.var_iimpact = assign48170_e61820;
        locals.var_iimpact_dn5 = assign48170_e61820_d_n5;
        locals.var_iimpact_dn6 = assign48170_e61820_d_n6;
        locals.var_iimpact_dn7 = assign48170_e61820_d_n7;
        locals.var_iimpact_dn8 = assign48170_e61820_d_n8;
        locals.var_iimpact_rv = 0.0;

        let assign48180_e61831: f64 = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1456 = assign48180_e61831;
        locals.var_guard1456_rv = 0.0;

        let assign48190_e61838: f64 = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1457 = assign48190_e61838;
        locals.var_guard1457_rv = 0.0;

        let (assign48200_e61844,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_phib_dc,)
    } else {
        (locals.var_phib__blk1297,)
    }
};
        locals.var_phib__blk1297 = assign48200_e61844;
        locals.var_phib__blk1297_rv = 0.0;

        let (assign48210_e61850,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_aphi_dc,)
    } else {
        (locals.var_aphi__blk1298,)
    }
};
        locals.var_aphi__blk1298 = assign48210_e61850;
        locals.var_aphi__blk1298_rv = 0.0;

        let (assign48220_e61856,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_g_0_dc,)
    } else {
        (locals.var_g_0__blk1299,)
    }
};
        locals.var_g_0__blk1299 = assign48220_e61856;
        locals.var_g_0__blk1299_rv = 0.0;

        let (assign48230_e61862, assign48230_e61862_d_n6, assign48230_e61862_d_n7, assign48230_e61862_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_v_xb_dc_tmp, locals.var_v_xb_dc_tmp_dn6, locals.var_v_xb_dc_tmp_dn7, locals.var_v_xb_dc_tmp_dn8,)
    } else {
        (locals.var_v_xb__blk1300, locals.var_v_xb__blk1300_dn6, locals.var_v_xb__blk1300_dn7, locals.var_v_xb__blk1300_dn8,)
    }
};
        locals.var_v_xb__blk1300 = assign48230_e61862;
        locals.var_v_xb__blk1300_dn6 = assign48230_e61862_d_n6;
        locals.var_v_xb__blk1300_dn7 = assign48230_e61862_d_n7;
        locals.var_v_xb__blk1300_dn8 = assign48230_e61862_d_n8;
        locals.var_v_xb__blk1300_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48240_e61868, assign48240_e61868_d_n5, assign48240_e61868_d_n6, assign48240_e61868_d_n7, assign48240_e61868_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_vsbstar_dc_tmp, locals.var_vsbstar_dc_tmp_dn5, locals.var_vsbstar_dc_tmp_dn6, locals.var_vsbstar_dc_tmp_dn7, locals.var_vsbstar_dc_tmp_dn8,)
    } else {
        (locals.var_vsbstar__blk1301, locals.var_vsbstar__blk1301_dn5, locals.var_vsbstar__blk1301_dn6, locals.var_vsbstar__blk1301_dn7, locals.var_vsbstar__blk1301_dn8,)
    }
};
        locals.var_vsbstar__blk1301 = assign48240_e61868;
        locals.var_vsbstar__blk1301_dn5 = assign48240_e61868_d_n5;
        locals.var_vsbstar__blk1301_dn6 = assign48240_e61868_d_n6;
        locals.var_vsbstar__blk1301_dn7 = assign48240_e61868_d_n7;
        locals.var_vsbstar__blk1301_dn8 = assign48240_e61868_d_n8;
        locals.var_vsbstar__blk1301_rv = 0.0;

        let (assign48250_e61874,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_dvbstar__blk1305,)
    }
};
        locals.var_dvbstar__blk1305 = assign48250_e61874;
        locals.var_dvbstar__blk1305_rv = 0.0;

        let assign48260_e61877: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1458 = assign48260_e61877;
        locals.var_guard1458_rv = 0.0;

        let (assign48270_e61902, assign48270_e61902_d_n6, assign48270_e61902_d_n7, assign48270_e61902_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        let assign48270_e61886: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign48270_e61889: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign48270_e61892: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign48270_e61893: f64 = (assign48270_e61889 * assign48270_e61892);
        let assign48270_e61895: f64 = (assign48270_e61893 + locals.var_bphi_ac);
        let assign48270_e61896: f64 = (assign48270_e61895).sqrt();
        let assign48270_e61897: f64 = (assign48270_e61886 - assign48270_e61896);
        let assign48270_e61898: f64 = (0.5 * assign48270_e61897);
        let assign48270_e61900: f64 = (assign48270_e61898 + locals.var_phix_ac);
        (assign48270_e61900, (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign48270_e61896)))), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign48270_e61896)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign48270_e61896)))),)
    } else {
        (locals.var_v_xb__blk1300, locals.var_v_xb__blk1300_dn6, locals.var_v_xb__blk1300_dn7, locals.var_v_xb__blk1300_dn8,)
    }
};
        locals.var_v_xb__blk1300 = assign48270_e61902;
        locals.var_v_xb__blk1300_dn6 = assign48270_e61902_d_n6;
        locals.var_v_xb__blk1300_dn7 = assign48270_e61902_d_n7;
        locals.var_v_xb__blk1300_dn8 = assign48270_e61902_d_n8;
        locals.var_v_xb__blk1300_rv = 0.0;

        let (assign48280_e61929, assign48280_e61929_d_n6, assign48280_e61929_d_n7, assign48280_e61929_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        let assign48280_e61912: f64 = locals.var_v_xb__blk1300;
        let assign48280_e61915: f64 = locals.var_v_xb__blk1300;
        let assign48280_e61918: f64 = locals.var_v_xb__blk1300;
        let assign48280_e61919: f64 = (assign48280_e61915 * assign48280_e61918);
        let assign48280_e61921: f64 = (assign48280_e61919 + locals.var_aphi_ac);
        let assign48280_e61922: f64 = (assign48280_e61921).sqrt();
        let assign48280_e61923: f64 = (assign48280_e61912 - assign48280_e61922);
        let assign48280_e61924: f64 = (0.5 * assign48280_e61923);
        let assign48280_e61925: f64 = (locals.var_v_sb - assign48280_e61924);
        let assign48280_e61927: f64 = (assign48280_e61925 + locals.var_phix1_ac);
        (assign48280_e61927, (locals.var_v_sb_dn6 - (0.5 * (locals.var_v_xb__blk1300_dn6 - (((locals.var_v_xb__blk1300_dn6 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn6)) / (2.0 * assign48280_e61922))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb__blk1300_dn7 - (((locals.var_v_xb__blk1300_dn7 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn7)) / (2.0 * assign48280_e61922))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb__blk1300_dn8 - (((locals.var_v_xb__blk1300_dn8 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn8)) / (2.0 * assign48280_e61922))))),)
    } else {
        (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn6, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8,)
    }
};
        locals.var_vsbstar_ac = assign48280_e61929;
        locals.var_vsbstar_ac_dn6 = assign48280_e61929_d_n6;
        locals.var_vsbstar_ac_dn7 = assign48280_e61929_d_n7;
        locals.var_vsbstar_ac_dn8 = assign48280_e61929_d_n8;
        locals.var_vsbstar_ac_rv = 0.0;

        let (assign48290_e61937, assign48290_e61937_d_n5, assign48290_e61937_d_n6, assign48290_e61937_d_n7, assign48290_e61937_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (locals.var_vsbstar_ac, 0.0, locals.var_vsbstar_ac_dn6, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8,)
    } else {
        (locals.var_vsbstar__blk1301, locals.var_vsbstar__blk1301_dn5, locals.var_vsbstar__blk1301_dn6, locals.var_vsbstar__blk1301_dn7, locals.var_vsbstar__blk1301_dn8,)
    }
};
        locals.var_vsbstar__blk1301 = assign48290_e61937;
        locals.var_vsbstar__blk1301_dn5 = assign48290_e61937_d_n5;
        locals.var_vsbstar__blk1301_dn6 = assign48290_e61937_d_n6;
        locals.var_vsbstar__blk1301_dn7 = assign48290_e61937_d_n7;
        locals.var_vsbstar__blk1301_dn8 = assign48290_e61937_d_n8;
        locals.var_vsbstar__blk1301_rv = 0.0;

        let (assign48300_e61945,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (locals.var_phib_ac,)
    } else {
        (locals.var_phib__blk1297,)
    }
};
        locals.var_phib__blk1297 = assign48300_e61945;
        locals.var_phib__blk1297_rv = 0.0;

        let (assign48310_e61953,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (locals.var_aphi_ac,)
    } else {
        (locals.var_aphi__blk1298,)
    }
};
        locals.var_aphi__blk1298 = assign48310_e61953;
        locals.var_aphi__blk1298_rv = 0.0;

        let (assign48320_e61961,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (locals.var_g_0_ac,)
    } else {
        (locals.var_g_0__blk1299,)
    }
};
        locals.var_g_0__blk1299 = assign48320_e61961;
        locals.var_g_0__blk1299_rv = 0.0;

        let (assign48330_e61971, assign48330_e61971_d_n5, assign48330_e61971_d_n6, assign48330_e61971_d_n7, assign48330_e61971_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48330_e61967: f64 = (locals.var_vgb - locals.var_dvbstar__blk1305);
        let assign48330_e61969: f64 = (assign48330_e61967 - locals.var_vfb_t);
        (assign48330_e61969, locals.var_vgb_dn5, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8,)
    } else {
        (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8,)
    }
};
        locals.var_vgb1__blk1304 = assign48330_e61971;
        locals.var_vgb1__blk1304_dn5 = assign48330_e61971_d_n5;
        locals.var_vgb1__blk1304_dn6 = assign48330_e61971_d_n6;
        locals.var_vgb1__blk1304_dn7 = assign48330_e61971_d_n7;
        locals.var_vgb1__blk1304_dn8 = assign48330_e61971_d_n8;
        locals.var_vgb1__blk1304_rv = 0.0;

        let (assign48340_e61983, assign48340_e61983_d_n5, assign48340_e61983_d_n6, assign48340_e61983_d_n7, assign48340_e61983_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48340_e61979: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign48340_e61980: f64 = (0.5 * assign48340_e61979);
        let assign48340_e61981: f64 = (locals.var_vsbstar__blk1301 + assign48340_e61980);
        (assign48340_e61981, locals.var_vsbstar__blk1301_dn5, (locals.var_vsbstar__blk1301_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstar__blk1301_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstar__blk1301_dn8,)
    } else {
        (locals.var_vsbx__blk1306, locals.var_vsbx__blk1306_dn5, locals.var_vsbx__blk1306_dn6, locals.var_vsbx__blk1306_dn7, locals.var_vsbx__blk1306_dn8,)
    }
};
        locals.var_vsbx__blk1306 = assign48340_e61983;
        locals.var_vsbx__blk1306_dn5 = assign48340_e61983_d_n5;
        locals.var_vsbx__blk1306_dn6 = assign48340_e61983_d_n6;
        locals.var_vsbx__blk1306_dn7 = assign48340_e61983_d_n7;
        locals.var_vsbx__blk1306_dn8 = assign48340_e61983_d_n8;
        locals.var_vsbx__blk1306_rv = 0.0;

        let (assign48350_e61989, assign48350_e61989_d_n5, assign48350_e61989_d_n6, assign48350_e61989_d_n7, assign48350_e61989_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8,)
    }
};
        locals.var_dctg__blk1318 = assign48350_e61989;
        locals.var_dctg__blk1318_dn5 = assign48350_e61989_d_n5;
        locals.var_dctg__blk1318_dn6 = assign48350_e61989_d_n6;
        locals.var_dctg__blk1318_dn7 = assign48350_e61989_d_n7;
        locals.var_dctg__blk1318_dn8 = assign48350_e61989_d_n8;
        locals.var_dctg__blk1318_rv = 0.0;

        let assign48360_e61992: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1459 = assign48360_e61992;
        locals.var_guard1459_rv = 0.0;

        let (assign48370_e62002,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48370_e62000: f64 = (locals.var_phib__blk1297 * locals.var_inv_phit);
        (assign48370_e62000,)
    } else {
        (locals.var_xbct__blk1309,)
    }
};
        locals.var_xbct__blk1309 = assign48370_e62002;
        locals.var_xbct__blk1309_rv = 0.0;

        let (assign48380_e62012, assign48380_e62012_d_n5, assign48380_e62012_d_n6, assign48380_e62012_d_n7, assign48380_e62012_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48380_e62010: f64 = (locals.var_vsbx__blk1306 * locals.var_inv_phit);
        (assign48380_e62010, (locals.var_vsbx__blk1306_dn5 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn6 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn7 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn8 * locals.var_inv_phit),)
    } else {
        (locals.var_xsbstar__blk1310, locals.var_xsbstar__blk1310_dn5, locals.var_xsbstar__blk1310_dn6, locals.var_xsbstar__blk1310_dn7, locals.var_xsbstar__blk1310_dn8,)
    }
};
        locals.var_xsbstar__blk1310 = assign48380_e62012;
        locals.var_xsbstar__blk1310_dn5 = assign48380_e62012_d_n5;
        locals.var_xsbstar__blk1310_dn6 = assign48380_e62012_d_n6;
        locals.var_xsbstar__blk1310_dn7 = assign48380_e62012_d_n7;
        locals.var_xsbstar__blk1310_dn8 = assign48380_e62012_d_n8;
        locals.var_xsbstar__blk1310_rv = 0.0;

        let (assign48390_e62022, assign48390_e62022_d_n5, assign48390_e62022_d_n6, assign48390_e62022_d_n7, assign48390_e62022_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48390_e62020: f64 = (locals.var_vgb1__blk1304 * locals.var_inv_phit);
        (assign48390_e62020, (locals.var_vgb1__blk1304_dn5 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn6 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn7 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn8 * locals.var_inv_phit),)
    } else {
        (locals.var_xgct__blk1311, locals.var_xgct__blk1311_dn5, locals.var_xgct__blk1311_dn6, locals.var_xgct__blk1311_dn7, locals.var_xgct__blk1311_dn8,)
    }
};
        locals.var_xgct__blk1311 = assign48390_e62022;
        locals.var_xgct__blk1311_dn5 = assign48390_e62022_d_n5;
        locals.var_xgct__blk1311_dn6 = assign48390_e62022_d_n6;
        locals.var_xgct__blk1311_dn7 = assign48390_e62022_d_n7;
        locals.var_xgct__blk1311_dn8 = assign48390_e62022_d_n8;
        locals.var_xgct__blk1311_rv = 0.0;

        let (assign48400_e62037, assign48400_e62037_d_n5, assign48400_e62037_d_n6, assign48400_e62037_d_n7, assign48400_e62037_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48400_e62031: f64 = (0.5 * locals.var_g_0__blk1299);
        let assign48400_e62033: f64 = (locals.var_xbct__blk1309).sqrt();
        let assign48400_e62034: f64 = (assign48400_e62031 / assign48400_e62033);
        let assign48400_e62035: f64 = (1.0 + assign48400_e62034);
        (assign48400_e62035, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48400_e62037;
        locals.var_temp1_dn5 = assign48400_e62037_d_n5;
        locals.var_temp1_dn6 = assign48400_e62037_d_n6;
        locals.var_temp1_dn7 = assign48400_e62037_d_n7;
        locals.var_temp1_dn8 = assign48400_e62037_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign48410_e62050, assign48410_e62050_d_n5, assign48410_e62050_d_n6, assign48410_e62050_d_n7, assign48410_e62050_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48410_e62046: f64 = (locals.var_xbct__blk1309).sqrt();
        let assign48410_e62047: f64 = (locals.var_g_0__blk1299 * assign48410_e62046);
        let assign48410_e62048: f64 = (locals.var_xbct__blk1309 + assign48410_e62047);
        (assign48410_e62048, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48410_e62050;
        locals.var_temp2_dn5 = assign48410_e62050_d_n5;
        locals.var_temp2_dn6 = assign48410_e62050_d_n6;
        locals.var_temp2_dn7 = assign48410_e62050_d_n7;
        locals.var_temp2_dn8 = assign48410_e62050_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign48420_e62072, assign48420_e62072_d_n5, assign48420_e62072_d_n6, assign48420_e62072_d_n7, assign48420_e62072_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48420_e62058: f64 = (locals.var_xgct__blk1311 - locals.var_temp2);
        let assign48420_e62060: f64 = (assign48420_e62058 / locals.var_temp1);
        let assign48420_e62063: f64 = (0.5 * locals.var_xbct__blk1309);
        let assign48420_e62064: f64 = (assign48420_e62060 + assign48420_e62063);
        let assign48420_e62067: f64 = (1.0 + locals.var_ctb_i);
        let assign48420_e62069: f64 = (assign48420_e62067 * locals.var_xsbstar__blk1310);
        let assign48420_e62070: f64 = (assign48420_e62064 - assign48420_e62069);
        (assign48420_e62070, (((((locals.var_xgct__blk1311_dn5 - locals.var_temp2_dn5) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn5)), (((((locals.var_xgct__blk1311_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn6)), (((((locals.var_xgct__blk1311_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn7)), (((((locals.var_xgct__blk1311_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn8)),)
    } else {
        (locals.var_xwict__blk1312, locals.var_xwict__blk1312_dn5, locals.var_xwict__blk1312_dn6, locals.var_xwict__blk1312_dn7, locals.var_xwict__blk1312_dn8,)
    }
};
        locals.var_xwict__blk1312 = assign48420_e62072;
        locals.var_xwict__blk1312_dn5 = assign48420_e62072_d_n5;
        locals.var_xwict__blk1312_dn6 = assign48420_e62072_d_n6;
        locals.var_xwict__blk1312_dn7 = assign48420_e62072_d_n7;
        locals.var_xwict__blk1312_dn8 = assign48420_e62072_d_n8;
        locals.var_xwict__blk1312_rv = 0.0;

        let (assign48430_e62084,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48430_e62080: f64 = (0.5 * locals.var_xbct__blk1309);
        let assign48430_e62082: f64 = (assign48430_e62080 + 2.0);
        (assign48430_e62082,)
    } else {
        (locals.var_xctmax__blk1313,)
    }
};
        locals.var_xctmax__blk1313 = assign48430_e62084;
        locals.var_xctmax__blk1313_rv = 0.0;

        let (assign48440_e62094, assign48440_e62094_d_n5, assign48440_e62094_d_n6, assign48440_e62094_d_n7, assign48440_e62094_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48440_e62092: f64 = (locals.var_xbct__blk1309 + locals.var_xsbstar__blk1310);
        (assign48440_e62092, locals.var_xsbstar__blk1310_dn5, locals.var_xsbstar__blk1310_dn6, locals.var_xsbstar__blk1310_dn7, locals.var_xsbstar__blk1310_dn8,)
    } else {
        (locals.var_xnct__blk1314, locals.var_xnct__blk1314_dn5, locals.var_xnct__blk1314_dn6, locals.var_xnct__blk1314_dn7, locals.var_xnct__blk1314_dn8,)
    }
};
        locals.var_xnct__blk1314 = assign48440_e62094;
        locals.var_xnct__blk1314_dn5 = assign48440_e62094_d_n5;
        locals.var_xnct__blk1314_dn6 = assign48440_e62094_d_n6;
        locals.var_xnct__blk1314_dn7 = assign48440_e62094_d_n7;
        locals.var_xnct__blk1314_dn8 = assign48440_e62094_d_n8;
        locals.var_xnct__blk1314_rv = 0.0;

        let (assign48450_e62119, assign48450_e62119_d_n5, assign48450_e62119_d_n6, assign48450_e62119_d_n7, assign48450_e62119_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48450_e62102: f64 = (locals.var_xgct__blk1311 - locals.var_xnct__blk1314);
        let assign48450_e62105: f64 = (locals.var_xnct__blk1314).sqrt();
        let assign48450_e62106: f64 = (locals.var_g_0__blk1299 * assign48450_e62105);
        let assign48450_e62107: f64 = (assign48450_e62102 - assign48450_e62106);
        let assign48450_e62111: f64 = (locals.var_xbct__blk1309 / locals.var_g_0__blk1299);
        let assign48450_e62113: f64 = (locals.var_xbct__blk1309).sqrt();
        let assign48450_e62114: f64 = (assign48450_e62111 + assign48450_e62113);
        let assign48450_e62115: f64 = (assign48450_e62114).ln();
        let assign48450_e62116: f64 = (2.0 * assign48450_e62115);
        let assign48450_e62117: f64 = (assign48450_e62107 - assign48450_e62116);
        (assign48450_e62117, ((locals.var_xgct__blk1311_dn5 - locals.var_xnct__blk1314_dn5) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn5 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn6 - locals.var_xnct__blk1314_dn6) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn6 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn7 - locals.var_xnct__blk1314_dn7) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn7 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn8 - locals.var_xnct__blk1314_dn8) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn8 / (2.0 * assign48450_e62105)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48450_e62119;
        locals.var_temp1_dn5 = assign48450_e62119_d_n5;
        locals.var_temp1_dn6 = assign48450_e62119_d_n6;
        locals.var_temp1_dn7 = assign48450_e62119_d_n7;
        locals.var_temp1_dn8 = assign48450_e62119_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign48460_e62131, assign48460_e62131_d_n5, assign48460_e62131_d_n6, assign48460_e62131_d_n7, assign48460_e62131_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48460_e62127: f64 = (2.0 * locals.var_temp1);
        let assign48460_e62129: f64 = (assign48460_e62127 + locals.var_xctmax__blk1313);
        (assign48460_e62129, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8),)
    } else {
        (locals.var_xmict__blk1315, locals.var_xmict__blk1315_dn5, locals.var_xmict__blk1315_dn6, locals.var_xmict__blk1315_dn7, locals.var_xmict__blk1315_dn8,)
    }
};
        locals.var_xmict__blk1315 = assign48460_e62131;
        locals.var_xmict__blk1315_dn5 = assign48460_e62131_d_n5;
        locals.var_xmict__blk1315_dn6 = assign48460_e62131_d_n6;
        locals.var_xmict__blk1315_dn7 = assign48460_e62131_d_n7;
        locals.var_xmict__blk1315_dn8 = assign48460_e62131_d_n8;
        locals.var_xmict__blk1315_rv = 0.0;

        let (assign48470_e62154, assign48470_e62154_d_n5, assign48470_e62154_d_n6, assign48470_e62154_d_n7, assign48470_e62154_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48470_e62140: f64 = (locals.var_xwict__blk1312 + locals.var_xmict__blk1315);
        let assign48470_e62143: f64 = (locals.var_xwict__blk1312 - locals.var_xmict__blk1315);
        let assign48470_e62146: f64 = (locals.var_xwict__blk1312 - locals.var_xmict__blk1315);
        let assign48470_e62147: f64 = (assign48470_e62143 * assign48470_e62146);
        let assign48470_e62149: f64 = (assign48470_e62147 + 20.0);
        let assign48470_e62150: f64 = (assign48470_e62149).sqrt();
        let assign48470_e62151: f64 = (assign48470_e62140 + assign48470_e62150);
        let assign48470_e62152: f64 = (0.5 * assign48470_e62151);
        (assign48470_e62152, (0.5 * ((locals.var_xwict__blk1312_dn5 + locals.var_xmict__blk1315_dn5) + ((((locals.var_xwict__blk1312_dn5 - locals.var_xmict__blk1315_dn5) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn5 - locals.var_xmict__blk1315_dn5))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn6 + locals.var_xmict__blk1315_dn6) + ((((locals.var_xwict__blk1312_dn6 - locals.var_xmict__blk1315_dn6) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn6 - locals.var_xmict__blk1315_dn6))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn7 + locals.var_xmict__blk1315_dn7) + ((((locals.var_xwict__blk1312_dn7 - locals.var_xmict__blk1315_dn7) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn7 - locals.var_xmict__blk1315_dn7))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn8 + locals.var_xmict__blk1315_dn8) + ((((locals.var_xwict__blk1312_dn8 - locals.var_xmict__blk1315_dn8) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn8 - locals.var_xmict__blk1315_dn8))) / (2.0 * assign48470_e62150)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48470_e62154;
        locals.var_temp1_dn5 = assign48470_e62154_d_n5;
        locals.var_temp1_dn6 = assign48470_e62154_d_n6;
        locals.var_temp1_dn7 = assign48470_e62154_d_n7;
        locals.var_temp1_dn8 = assign48470_e62154_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign48480_e62168, assign48480_e62168_d_n5, assign48480_e62168_d_n6, assign48480_e62168_d_n7, assign48480_e62168_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48480_e62163: f64 = (locals.var_xgct__blk1311 - locals.var_xsbstar__blk1310);
        let assign48480_e62164: f64 = (2.0 * assign48480_e62163);
        let assign48480_e62166: f64 = (assign48480_e62164 - locals.var_xctmax__blk1313);
        (assign48480_e62166, (2.0 * (locals.var_xgct__blk1311_dn5 - locals.var_xsbstar__blk1310_dn5)), (2.0 * (locals.var_xgct__blk1311_dn6 - locals.var_xsbstar__blk1310_dn6)), (2.0 * (locals.var_xgct__blk1311_dn7 - locals.var_xsbstar__blk1310_dn7)), (2.0 * (locals.var_xgct__blk1311_dn8 - locals.var_xsbstar__blk1310_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48480_e62168;
        locals.var_temp2_dn5 = assign48480_e62168_d_n5;
        locals.var_temp2_dn6 = assign48480_e62168_d_n6;
        locals.var_temp2_dn7 = assign48480_e62168_d_n7;
        locals.var_temp2_dn8 = assign48480_e62168_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign48490_e62191, assign48490_e62191_d_n5, assign48490_e62191_d_n6, assign48490_e62191_d_n7, assign48490_e62191_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48490_e62177: f64 = (locals.var_temp1 + locals.var_temp2);
        let assign48490_e62180: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign48490_e62183: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign48490_e62184: f64 = (assign48490_e62180 * assign48490_e62183);
        let assign48490_e62186: f64 = (assign48490_e62184 + 20.0);
        let assign48490_e62187: f64 = (assign48490_e62186).sqrt();
        let assign48490_e62188: f64 = (assign48490_e62177 - assign48490_e62187);
        let assign48490_e62189: f64 = (0.5 * assign48490_e62188);
        (assign48490_e62189, (0.5 * ((locals.var_temp1_dn5 + locals.var_temp2_dn5) - ((((locals.var_temp1_dn5 - locals.var_temp2_dn5) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn5 - locals.var_temp2_dn5))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign48490_e62187)))),)
    } else {
        (locals.var_xsubct__blk1316, locals.var_xsubct__blk1316_dn5, locals.var_xsubct__blk1316_dn6, locals.var_xsubct__blk1316_dn7, locals.var_xsubct__blk1316_dn8,)
    }
};
        locals.var_xsubct__blk1316 = assign48490_e62191;
        locals.var_xsubct__blk1316_dn5 = assign48490_e62191_d_n5;
        locals.var_xsubct__blk1316_dn6 = assign48490_e62191_d_n6;
        locals.var_xsubct__blk1316_dn7 = assign48490_e62191_d_n7;
        locals.var_xsubct__blk1316_dn8 = assign48490_e62191_d_n8;
        locals.var_xsubct__blk1316_rv = 0.0;

        let (assign48500_e62214, assign48500_e62214_d_n5, assign48500_e62214_d_n6, assign48500_e62214_d_n7, assign48500_e62214_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48500_e62200: f64 = (locals.var_xsubct__blk1316 + locals.var_xctmax__blk1313);
        let assign48500_e62203: f64 = (locals.var_xsubct__blk1316 - locals.var_xctmax__blk1313);
        let assign48500_e62206: f64 = (locals.var_xsubct__blk1316 - locals.var_xctmax__blk1313);
        let assign48500_e62207: f64 = (assign48500_e62203 * assign48500_e62206);
        let assign48500_e62209: f64 = (assign48500_e62207 + 5.0);
        let assign48500_e62210: f64 = (assign48500_e62209).sqrt();
        let assign48500_e62211: f64 = (assign48500_e62200 - assign48500_e62210);
        let assign48500_e62212: f64 = (0.5 * assign48500_e62211);
        (assign48500_e62212, (0.5 * (locals.var_xsubct__blk1316_dn5 - (((locals.var_xsubct__blk1316_dn5 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn5)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn6 - (((locals.var_xsubct__blk1316_dn6 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn6)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn7 - (((locals.var_xsubct__blk1316_dn7 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn7)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn8 - (((locals.var_xsubct__blk1316_dn8 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn8)) / (2.0 * assign48500_e62210)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48500_e62214;
        locals.var_temp1_dn5 = assign48500_e62214_d_n5;
        locals.var_temp1_dn6 = assign48500_e62214_d_n6;
        locals.var_temp1_dn7 = assign48500_e62214_d_n7;
        locals.var_temp1_dn8 = assign48500_e62214_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign48510_e62240, assign48510_e62240_d_n5, assign48510_e62240_d_n6, assign48510_e62240_d_n7, assign48510_e62240_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48510_e62223: f64 = (-locals.var_xctmax__blk1313);
        let assign48510_e62224: f64 = (locals.var_temp1 + assign48510_e62223);
        let assign48510_e62227: f64 = (-locals.var_xctmax__blk1313);
        let assign48510_e62228: f64 = (locals.var_temp1 - assign48510_e62227);
        let assign48510_e62231: f64 = (-locals.var_xctmax__blk1313);
        let assign48510_e62232: f64 = (locals.var_temp1 - assign48510_e62231);
        let assign48510_e62233: f64 = (assign48510_e62228 * assign48510_e62232);
        let assign48510_e62235: f64 = (assign48510_e62233 + 20.0);
        let assign48510_e62236: f64 = (assign48510_e62235).sqrt();
        let assign48510_e62237: f64 = (assign48510_e62224 + assign48510_e62236);
        let assign48510_e62238: f64 = (0.5 * assign48510_e62237);
        (assign48510_e62238, (0.5 * (locals.var_temp1_dn5 + (((locals.var_temp1_dn5 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn5)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn6)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn7)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn8)) / (2.0 * assign48510_e62236)))),)
    } else {
        (locals.var_xct__blk1317, locals.var_xct__blk1317_dn5, locals.var_xct__blk1317_dn6, locals.var_xct__blk1317_dn7, locals.var_xct__blk1317_dn8,)
    }
};
        locals.var_xct__blk1317 = assign48510_e62240;
        locals.var_xct__blk1317_dn5 = assign48510_e62240_d_n5;
        locals.var_xct__blk1317_dn6 = assign48510_e62240_d_n6;
        locals.var_xct__blk1317_dn7 = assign48510_e62240_d_n7;
        locals.var_xct__blk1317_dn8 = assign48510_e62240_d_n8;
        locals.var_xct__blk1317_rv = 0.0;

        let (assign48520_e62254, assign48520_e62254_d_n5, assign48520_e62254_d_n6, assign48520_e62254_d_n7, assign48520_e62254_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48520_e62249: f64 = (locals.var_xct__blk1317 / locals.var_xctmax__blk1313);
        let assign48520_e62251: f64 = (assign48520_e62249 + 1.0);
        let assign48520_e62252: f64 = (locals.var_ctg_t * assign48520_e62251);
        (assign48520_e62252, (locals.var_ctg_t * (locals.var_xct__blk1317_dn5 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn6 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn7 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn8 / locals.var_xctmax__blk1313)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48520_e62254;
        locals.var_temp2_dn5 = assign48520_e62254_d_n5;
        locals.var_temp2_dn6 = assign48520_e62254_d_n6;
        locals.var_temp2_dn7 = assign48520_e62254_d_n7;
        locals.var_temp2_dn8 = assign48520_e62254_d_n8;
        locals.var_temp2_rv = 0.0;

        let assign48530_e62257: f64 = (-230.25850929940458);
        let assign48530_e62258: f64 = if locals.var_temp2 > assign48530_e62257 { 1.0 } else { 0.0 };
        locals.var_guard1460 = assign48530_e62258;
        locals.var_guard1460_rv = 0.0;

        let (assign48540_e62269, assign48540_e62269_d_n5, assign48540_e62269_d_n6, assign48540_e62269_d_n7, assign48540_e62269_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) && (locals.var_guard1460 != 0.0)) {
        let assign48540_e62267: f64 = (locals.var_temp2).exp();
        (assign48540_e62267, (assign48540_e62267 * locals.var_temp2_dn5), (assign48540_e62267 * locals.var_temp2_dn6), (assign48540_e62267 * locals.var_temp2_dn7), (assign48540_e62267 * locals.var_temp2_dn8),)
    } else {
        (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8,)
    }
};
        locals.var_dctg__blk1318 = assign48540_e62269;
        locals.var_dctg__blk1318_dn5 = assign48540_e62269_d_n5;
        locals.var_dctg__blk1318_dn6 = assign48540_e62269_d_n6;
        locals.var_dctg__blk1318_dn7 = assign48540_e62269_d_n7;
        locals.var_dctg__blk1318_dn8 = assign48540_e62269_d_n8;
        locals.var_dctg__blk1318_rv = 0.0;

        let (assign48550_e62305, assign48550_e62305_d_n5, assign48550_e62305_d_n6, assign48550_e62305_d_n7, assign48550_e62305_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) && (locals.var_guard1460 == 0.0)) {
        let assign48550_e62281: f64 = (-230.25850929940458);
        let assign48550_e62283: f64 = (assign48550_e62281 - locals.var_temp2);
        let assign48550_e62287: f64 = (-230.25850929940458);
        let assign48550_e62289: f64 = (assign48550_e62287 - locals.var_temp2);
        let assign48550_e62292: f64 = (-230.25850929940458);
        let assign48550_e62294: f64 = (assign48550_e62292 - locals.var_temp2);
        let assign48550_e62296: f64 = (assign48550_e62294 * 0.3333333333333333);
        let assign48550_e62297: f64 = (1.0 + assign48550_e62296);
        let assign48550_e62298: f64 = (assign48550_e62289 * assign48550_e62297);
        let assign48550_e62299: f64 = (0.5 * assign48550_e62298);
        let assign48550_e62300: f64 = (1.0 + assign48550_e62299);
        let assign48550_e62301: f64 = (assign48550_e62283 * assign48550_e62300);
        let assign48550_e62302: f64 = (1.0 + assign48550_e62301);
        let assign48550_e62303: f64 = (1e-100 / assign48550_e62302);
        (assign48550_e62303, (-((1e-100 * (((-locals.var_temp2_dn5) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn5) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn6) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn7) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn8) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))),)
    } else {
        (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8,)
    }
};
        locals.var_dctg__blk1318 = assign48550_e62305;
        locals.var_dctg__blk1318_dn5 = assign48550_e62305_d_n5;
        locals.var_dctg__blk1318_dn6 = assign48550_e62305_d_n6;
        locals.var_dctg__blk1318_dn7 = assign48550_e62305_d_n7;
        locals.var_dctg__blk1318_dn8 = assign48550_e62305_d_n8;
        locals.var_dctg__blk1318_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48560_e62315, assign48560_e62315_d_n5, assign48560_e62315_d_n6, assign48560_e62315_d_n7, assign48560_e62315_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48560_e62312: f64 = (locals.var_ct_t * locals.var_dctg__blk1318);
        let assign48560_e62313: f64 = (1.0 + assign48560_e62312);
        (assign48560_e62313, (locals.var_ct_t * locals.var_dctg__blk1318_dn5), (locals.var_ct_t * locals.var_dctg__blk1318_dn6), (locals.var_ct_t * locals.var_dctg__blk1318_dn7), (locals.var_ct_t * locals.var_dctg__blk1318_dn8),)
    } else {
        (locals.var_ct_fact__blk1319, locals.var_ct_fact__blk1319_dn5, locals.var_ct_fact__blk1319_dn6, locals.var_ct_fact__blk1319_dn7, locals.var_ct_fact__blk1319_dn8,)
    }
};
        locals.var_ct_fact__blk1319 = assign48560_e62315;
        locals.var_ct_fact__blk1319_dn5 = assign48560_e62315_d_n5;
        locals.var_ct_fact__blk1319_dn6 = assign48560_e62315_d_n6;
        locals.var_ct_fact__blk1319_dn7 = assign48560_e62315_d_n7;
        locals.var_ct_fact__blk1319_dn8 = assign48560_e62315_d_n8;
        locals.var_ct_fact__blk1319_rv = 0.0;

        let (assign48570_e62323, assign48570_e62323_d_n5, assign48570_e62323_d_n6, assign48570_e62323_d_n7, assign48570_e62323_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48570_e62321: f64 = (locals.var_phit * locals.var_ct_fact__blk1319);
        (assign48570_e62321, (locals.var_phit * locals.var_ct_fact__blk1319_dn5), (locals.var_phit * locals.var_ct_fact__blk1319_dn6), (locals.var_phit * locals.var_ct_fact__blk1319_dn7), (locals.var_phit * locals.var_ct_fact__blk1319_dn8),)
    } else {
        (locals.var_phitct__blk1320, locals.var_phitct__blk1320_dn5, locals.var_phitct__blk1320_dn6, locals.var_phitct__blk1320_dn7, locals.var_phitct__blk1320_dn8,)
    }
};
        locals.var_phitct__blk1320 = assign48570_e62323;
        locals.var_phitct__blk1320_dn5 = assign48570_e62323_d_n5;
        locals.var_phitct__blk1320_dn6 = assign48570_e62323_d_n6;
        locals.var_phitct__blk1320_dn7 = assign48570_e62323_d_n7;
        locals.var_phitct__blk1320_dn8 = assign48570_e62323_d_n8;
        locals.var_phitct__blk1320_rv = 0.0;

        let (assign48580_e62341, assign48580_e62341_d_n5, assign48580_e62341_d_n6, assign48580_e62341_d_n7, assign48580_e62341_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48580_e62331: f64 = (locals.var_psced_i * locals.var_vdsx);
        let assign48580_e62332: f64 = (1.0 + assign48580_e62331);
        let assign48580_e62333: f64 = (locals.var_psce_i * assign48580_e62332);
        let assign48580_e62337: f64 = (locals.var_psceb_i * locals.var_vsbx__blk1306);
        let assign48580_e62338: f64 = (1.0 + assign48580_e62337);
        let assign48580_e62339: f64 = (assign48580_e62333 * assign48580_e62338);
        (assign48580_e62339, (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn5)), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn6)) * assign48580_e62338) + (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn6))), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign48580_e62338) + (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn7))), (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn8)),)
    } else {
        (locals.var_dphit1__blk1321, locals.var_dphit1__blk1321_dn5, locals.var_dphit1__blk1321_dn6, locals.var_dphit1__blk1321_dn7, locals.var_dphit1__blk1321_dn8,)
    }
};
        locals.var_dphit1__blk1321 = assign48580_e62341;
        locals.var_dphit1__blk1321_dn5 = assign48580_e62341_d_n5;
        locals.var_dphit1__blk1321_dn6 = assign48580_e62341_d_n6;
        locals.var_dphit1__blk1321_dn7 = assign48580_e62341_d_n7;
        locals.var_dphit1__blk1321_dn8 = assign48580_e62341_d_n8;
        locals.var_dphit1__blk1321_rv = 0.0;

        let (assign48590_e62351, assign48590_e62351_d_n5, assign48590_e62351_d_n6, assign48590_e62351_d_n7, assign48590_e62351_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48590_e62348: f64 = (1.0 + locals.var_dphit1__blk1321);
        let assign48590_e62349: f64 = (locals.var_phitct__blk1320 * assign48590_e62348);
        (assign48590_e62349, ((locals.var_phitct__blk1320_dn5 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn5)), ((locals.var_phitct__blk1320_dn6 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn6)), ((locals.var_phitct__blk1320_dn7 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn7)), ((locals.var_phitct__blk1320_dn8 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn8)),)
    } else {
        (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8,)
    }
};
        locals.var_phit1__blk1322 = assign48590_e62351;
        locals.var_phit1__blk1322_dn5 = assign48590_e62351_d_n5;
        locals.var_phit1__blk1322_dn6 = assign48590_e62351_d_n6;
        locals.var_phit1__blk1322_dn7 = assign48590_e62351_d_n7;
        locals.var_phit1__blk1322_dn8 = assign48590_e62351_d_n8;
        locals.var_phit1__blk1322_rv = 0.0;

        let (assign48600_e62359, assign48600_e62359_d_n5, assign48600_e62359_d_n6, assign48600_e62359_d_n7, assign48600_e62359_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48600_e62357: f64 = (1.0 / locals.var_phit1__blk1322);
        (assign48600_e62357, (-(locals.var_phit1__blk1322_dn5 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn6 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn7 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn8 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))),)
    } else {
        (locals.var_inv_phit1__blk1323, locals.var_inv_phit1__blk1323_dn5, locals.var_inv_phit1__blk1323_dn6, locals.var_inv_phit1__blk1323_dn7, locals.var_inv_phit1__blk1323_dn8,)
    }
};
        locals.var_inv_phit1__blk1323 = assign48600_e62359;
        locals.var_inv_phit1__blk1323_dn5 = assign48600_e62359_d_n5;
        locals.var_inv_phit1__blk1323_dn6 = assign48600_e62359_d_n6;
        locals.var_inv_phit1__blk1323_dn7 = assign48600_e62359_d_n7;
        locals.var_inv_phit1__blk1323_dn8 = assign48600_e62359_d_n8;
        locals.var_inv_phit1__blk1323_rv = 0.0;

        let (assign48610_e62370, assign48610_e62370_d_n5, assign48610_e62370_d_n6, assign48610_e62370_d_n7, assign48610_e62370_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48610_e62366: f64 = (locals.var_phit * locals.var_inv_phit1__blk1323);
        let assign48610_e62367: f64 = (assign48610_e62366).sqrt();
        let assign48610_e62368: f64 = (locals.var_g_0__blk1299 * assign48610_e62367);
        (assign48610_e62368, (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn5) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn6) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn7) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn8) / (2.0 * assign48610_e62367))),)
    } else {
        (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8,)
    }
};
        locals.var_gf__blk1307 = assign48610_e62370;
        locals.var_gf__blk1307_dn5 = assign48610_e62370_d_n5;
        locals.var_gf__blk1307_dn6 = assign48610_e62370_d_n6;
        locals.var_gf__blk1307_dn7 = assign48610_e62370_d_n7;
        locals.var_gf__blk1307_dn8 = assign48610_e62370_d_n8;
        locals.var_gf__blk1307_rv = 0.0;

        let (assign48620_e62378, assign48620_e62378_d_n5, assign48620_e62378_d_n6, assign48620_e62378_d_n7, assign48620_e62378_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48620_e62376: f64 = (locals.var_gf__blk1307 * locals.var_gf__blk1307);
        (assign48620_e62376, ((locals.var_gf__blk1307_dn5 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn5)), ((locals.var_gf__blk1307_dn6 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn6)), ((locals.var_gf__blk1307_dn7 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn7)), ((locals.var_gf__blk1307_dn8 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn8)),)
    } else {
        (locals.var_gf2__blk1308, locals.var_gf2__blk1308_dn5, locals.var_gf2__blk1308_dn6, locals.var_gf2__blk1308_dn7, locals.var_gf2__blk1308_dn8,)
    }
};
        locals.var_gf2__blk1308 = assign48620_e62378;
        locals.var_gf2__blk1308_dn5 = assign48620_e62378_d_n5;
        locals.var_gf2__blk1308_dn6 = assign48620_e62378_d_n6;
        locals.var_gf2__blk1308_dn7 = assign48620_e62378_d_n7;
        locals.var_gf2__blk1308_dn8 = assign48620_e62378_d_n8;
        locals.var_gf2__blk1308_rv = 0.0;

        let (assign48630_e62386, assign48630_e62386_d_n5, assign48630_e62386_d_n6, assign48630_e62386_d_n7, assign48630_e62386_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48630_e62384: f64 = (1.0 / locals.var_gf2__blk1308);
        (assign48630_e62384, (-(locals.var_gf2__blk1308_dn5 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn6 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn7 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn8 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))),)
    } else {
        (locals.var_inv_gf2__blk1324, locals.var_inv_gf2__blk1324_dn5, locals.var_inv_gf2__blk1324_dn6, locals.var_inv_gf2__blk1324_dn7, locals.var_inv_gf2__blk1324_dn8,)
    }
};
        locals.var_inv_gf2__blk1324 = assign48630_e62386;
        locals.var_inv_gf2__blk1324_dn5 = assign48630_e62386_d_n5;
        locals.var_inv_gf2__blk1324_dn6 = assign48630_e62386_d_n6;
        locals.var_inv_gf2__blk1324_dn7 = assign48630_e62386_d_n7;
        locals.var_inv_gf2__blk1324_dn8 = assign48630_e62386_d_n8;
        locals.var_inv_gf2__blk1324_rv = 0.0;

        let (assign48640_e62394, assign48640_e62394_d_n5, assign48640_e62394_d_n6, assign48640_e62394_d_n7, assign48640_e62394_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48640_e62392: f64 = (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323);
        (assign48640_e62392, ((locals.var_vsbstar__blk1301_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vsbstar__blk1301_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vsbstar__blk1301_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vsbstar__blk1301_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn8)),)
    } else {
        (locals.var_ux__blk1325, locals.var_ux__blk1325_dn5, locals.var_ux__blk1325_dn6, locals.var_ux__blk1325_dn7, locals.var_ux__blk1325_dn8,)
    }
};
        locals.var_ux__blk1325 = assign48640_e62394;
        locals.var_ux__blk1325_dn5 = assign48640_e62394_d_n5;
        locals.var_ux__blk1325_dn6 = assign48640_e62394_d_n6;
        locals.var_ux__blk1325_dn7 = assign48640_e62394_d_n7;
        locals.var_ux__blk1325_dn8 = assign48640_e62394_d_n8;
        locals.var_ux__blk1325_rv = 0.0;

        let (assign48650_e62402, assign48650_e62402_d_n5, assign48650_e62402_d_n6, assign48650_e62402_d_n7, assign48650_e62402_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48650_e62400: f64 = (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323);
        (assign48650_e62400, ((locals.var_vgb1__blk1304_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vgb1__blk1304_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vgb1__blk1304_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vgb1__blk1304_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn8)),)
    } else {
        (locals.var_xg__blk1326, locals.var_xg__blk1326_dn5, locals.var_xg__blk1326_dn6, locals.var_xg__blk1326_dn7, locals.var_xg__blk1326_dn8,)
    }
};
        locals.var_xg__blk1326 = assign48650_e62402;
        locals.var_xg__blk1326_dn5 = assign48650_e62402_d_n5;
        locals.var_xg__blk1326_dn6 = assign48650_e62402_d_n6;
        locals.var_xg__blk1326_dn7 = assign48650_e62402_d_n7;
        locals.var_xg__blk1326_dn8 = assign48650_e62402_d_n8;
        locals.var_xg__blk1326_rv = 0.0;

        let (assign48660_e62419, assign48660_e62419_d_n6, assign48660_e62419_d_n7,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48660_e62408: f64 = (2.0 * locals.var_vdsx);
        let assign48660_e62413: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign48660_e62414: f64 = (1.0 + assign48660_e62413);
        let assign48660_e62415: f64 = (assign48660_e62414).sqrt();
        let assign48660_e62416: f64 = (1.0 + assign48660_e62415);
        let assign48660_e62417: f64 = (assign48660_e62408 / assign48660_e62416);
        (assign48660_e62417, ((((2.0 * locals.var_vdsx_dn6) * assign48660_e62416) - (assign48660_e62408 * ((locals.var_cfd_i * locals.var_vdsx_dn6) / (2.0 * assign48660_e62415)))) / (assign48660_e62416 * assign48660_e62416)), ((((2.0 * locals.var_vdsx_dn7) * assign48660_e62416) - (assign48660_e62408 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign48660_e62415)))) / (assign48660_e62416 * assign48660_e62416)),)
    } else {
        (locals.var_vdsp__blk1327, locals.var_vdsp__blk1327_dn6, locals.var_vdsp__blk1327_dn7,)
    }
};
        locals.var_vdsp__blk1327 = assign48660_e62419;
        locals.var_vdsp__blk1327_dn6 = assign48660_e62419_d_n6;
        locals.var_vdsp__blk1327_dn7 = assign48660_e62419_d_n7;
        locals.var_vdsp__blk1327_rv = 0.0;

        let (assign48670_e62433, assign48670_e62433_d_n5, assign48670_e62433_d_n6, assign48670_e62433_d_n7, assign48670_e62433_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48670_e62425: f64 = (locals.var_cf_i * locals.var_vdsp__blk1327);
        let assign48670_e62429: f64 = (locals.var_cfb_i * locals.var_vsbx__blk1306);
        let assign48670_e62430: f64 = (1.0 + assign48670_e62429);
        let assign48670_e62431: f64 = (assign48670_e62425 * assign48670_e62430);
        (assign48670_e62431, (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn5)), (((locals.var_cf_i * locals.var_vdsp__blk1327_dn6) * assign48670_e62430) + (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn6))), (((locals.var_cf_i * locals.var_vdsp__blk1327_dn7) * assign48670_e62430) + (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn7))), (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn8)),)
    } else {
        (locals.var_delphib__blk1328, locals.var_delphib__blk1328_dn5, locals.var_delphib__blk1328_dn6, locals.var_delphib__blk1328_dn7, locals.var_delphib__blk1328_dn8,)
    }
};
        locals.var_delphib__blk1328 = assign48670_e62433;
        locals.var_delphib__blk1328_dn5 = assign48670_e62433_d_n5;
        locals.var_delphib__blk1328_dn6 = assign48670_e62433_d_n6;
        locals.var_delphib__blk1328_dn7 = assign48670_e62433_d_n7;
        locals.var_delphib__blk1328_dn8 = assign48670_e62433_d_n8;
        locals.var_delphib__blk1328_rv = 0.0;

        let (assign48680_e62441, assign48680_e62441_d_n5, assign48680_e62441_d_n6, assign48680_e62441_d_n7, assign48680_e62441_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48680_e62439: f64 = (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323);
        (assign48680_e62439, (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn5), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn6), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn7), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn8),)
    } else {
        (locals.var_xb__blk1329, locals.var_xb__blk1329_dn5, locals.var_xb__blk1329_dn6, locals.var_xb__blk1329_dn7, locals.var_xb__blk1329_dn8,)
    }
};
        locals.var_xb__blk1329 = assign48680_e62441;
        locals.var_xb__blk1329_dn5 = assign48680_e62441_d_n5;
        locals.var_xb__blk1329_dn6 = assign48680_e62441_d_n6;
        locals.var_xb__blk1329_dn7 = assign48680_e62441_d_n7;
        locals.var_xb__blk1329_dn8 = assign48680_e62441_d_n8;
        locals.var_xb__blk1329_rv = 0.0;

        let (assign48690_e62452, assign48690_e62452_d_n5, assign48690_e62452_d_n6, assign48690_e62452_d_n7, assign48690_e62452_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48690_e62447: f64 = (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300);
        let assign48690_e62449: f64 = (assign48690_e62447 + locals.var_aphi__blk1298);
        let assign48690_e62450: f64 = (assign48690_e62449).sqrt();
        (assign48690_e62450, 0.0, (((locals.var_v_xb__blk1300_dn6 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn6)) / (2.0 * assign48690_e62450)), (((locals.var_v_xb__blk1300_dn7 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn7)) / (2.0 * assign48690_e62450)), (((locals.var_v_xb__blk1300_dn8 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn8)) / (2.0 * assign48690_e62450)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48690_e62452;
        locals.var_temp1_dn5 = assign48690_e62452_d_n5;
        locals.var_temp1_dn6 = assign48690_e62452_d_n6;
        locals.var_temp1_dn7 = assign48690_e62452_d_n7;
        locals.var_temp1_dn8 = assign48690_e62452_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign48700_e62467, assign48700_e62467_d_n5, assign48700_e62467_d_n6, assign48700_e62467_d_n7, assign48700_e62467_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48700_e62458: f64 = (locals.var_v_xb__blk1300 - locals.var_delphib__blk1328);
        let assign48700_e62461: f64 = (locals.var_v_xb__blk1300 - locals.var_delphib__blk1328);
        let assign48700_e62462: f64 = (assign48700_e62458 * assign48700_e62461);
        let assign48700_e62464: f64 = (assign48700_e62462 + locals.var_aphi__blk1298);
        let assign48700_e62465: f64 = (assign48700_e62464).sqrt();
        (assign48700_e62465, ((((-locals.var_delphib__blk1328_dn5) * assign48700_e62461) + (assign48700_e62458 * (-locals.var_delphib__blk1328_dn5))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn6 - locals.var_delphib__blk1328_dn6) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn6 - locals.var_delphib__blk1328_dn6))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn7 - locals.var_delphib__blk1328_dn7) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn7 - locals.var_delphib__blk1328_dn7))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn8 - locals.var_delphib__blk1328_dn8) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn8 - locals.var_delphib__blk1328_dn8))) / (2.0 * assign48700_e62465)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48700_e62467;
        locals.var_temp2_dn5 = assign48700_e62467_d_n5;
        locals.var_temp2_dn6 = assign48700_e62467_d_n6;
        locals.var_temp2_dn7 = assign48700_e62467_d_n7;
        locals.var_temp2_dn8 = assign48700_e62467_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign48710_e62481, assign48710_e62481_d_n5, assign48710_e62481_d_n6, assign48710_e62481_d_n7, assign48710_e62481_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48710_e62473: f64 = (0.5 * locals.var_inv_phit1__blk1323);
        let assign48710_e62476: f64 = (locals.var_delphib__blk1328 + locals.var_temp1);
        let assign48710_e62478: f64 = (assign48710_e62476 - locals.var_temp2);
        let assign48710_e62479: f64 = (assign48710_e62473 * assign48710_e62478);
        (assign48710_e62479, (((0.5 * locals.var_inv_phit1__blk1323_dn5) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn5 + locals.var_temp1_dn5) - locals.var_temp2_dn5))), (((0.5 * locals.var_inv_phit1__blk1323_dn6) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6))), (((0.5 * locals.var_inv_phit1__blk1323_dn7) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7))), (((0.5 * locals.var_inv_phit1__blk1323_dn8) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8))),)
    } else {
        (locals.var_delxb__blk1330, locals.var_delxb__blk1330_dn5, locals.var_delxb__blk1330_dn6, locals.var_delxb__blk1330_dn7, locals.var_delxb__blk1330_dn8,)
    }
};
        locals.var_delxb__blk1330 = assign48710_e62481;
        locals.var_delxb__blk1330_dn5 = assign48710_e62481_d_n5;
        locals.var_delxb__blk1330_dn6 = assign48710_e62481_d_n6;
        locals.var_delxb__blk1330_dn7 = assign48710_e62481_d_n7;
        locals.var_delxb__blk1330_dn8 = assign48710_e62481_d_n8;
        locals.var_delxb__blk1330_rv = 0.0;

        let (assign48720_e62489, assign48720_e62489_d_n5, assign48720_e62489_d_n6, assign48720_e62489_d_n7, assign48720_e62489_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48720_e62487: f64 = (locals.var_xb__blk1329 + locals.var_ux__blk1325);
        (assign48720_e62487, (locals.var_xb__blk1329_dn5 + locals.var_ux__blk1325_dn5), (locals.var_xb__blk1329_dn6 + locals.var_ux__blk1325_dn6), (locals.var_xb__blk1329_dn7 + locals.var_ux__blk1325_dn7), (locals.var_xb__blk1329_dn8 + locals.var_ux__blk1325_dn8),)
    } else {
        (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8,)
    }
};
        locals.var_xno_s__blk1331 = assign48720_e62489;
        locals.var_xno_s__blk1331_dn5 = assign48720_e62489_d_n5;
        locals.var_xno_s__blk1331_dn6 = assign48720_e62489_d_n6;
        locals.var_xno_s__blk1331_dn7 = assign48720_e62489_d_n7;
        locals.var_xno_s__blk1331_dn8 = assign48720_e62489_d_n8;
        locals.var_xno_s__blk1331_rv = 0.0;

        let (assign48730_e62497, assign48730_e62497_d_n5, assign48730_e62497_d_n6, assign48730_e62497_d_n7, assign48730_e62497_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48730_e62495: f64 = (locals.var_xno_s__blk1331 - locals.var_delxb__blk1330);
        (assign48730_e62495, (locals.var_xno_s__blk1331_dn5 - locals.var_delxb__blk1330_dn5), (locals.var_xno_s__blk1331_dn6 - locals.var_delxb__blk1330_dn6), (locals.var_xno_s__blk1331_dn7 - locals.var_delxb__blk1330_dn7), (locals.var_xno_s__blk1331_dn8 - locals.var_delxb__blk1330_dn8),)
    } else {
        (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8,)
    }
};
        locals.var_xn_s__blk1332 = assign48730_e62497;
        locals.var_xn_s__blk1332_dn5 = assign48730_e62497_d_n5;
        locals.var_xn_s__blk1332_dn6 = assign48730_e62497_d_n6;
        locals.var_xn_s__blk1332_dn7 = assign48730_e62497_d_n7;
        locals.var_xn_s__blk1332_dn8 = assign48730_e62497_d_n8;
        locals.var_xn_s__blk1332_rv = 0.0;

        let assign48740_e62500: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1461 = assign48740_e62500;
        locals.var_guard1461_rv = 0.0;

        let assign48750_e62502: f64 = (locals.var_xn_s__blk1332).abs();
        let assign48750_e62504: f64 = if assign48750_e62502 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1462 = assign48750_e62504;
        locals.var_guard1462_rv = 0.0;

        let (assign48760_e62528, assign48760_e62528_d_n5, assign48760_e62528_d_n6, assign48760_e62528_d_n7, assign48760_e62528_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) {
        let assign48760_e62517: f64 = (0.5 * locals.var_xn_s__blk1332);
        let assign48760_e62521: f64 = (0.3125 * locals.var_xn_s__blk1332);
        let assign48760_e62522: f64 = (1.0 - assign48760_e62521);
        let assign48760_e62523: f64 = (assign48760_e62517 * assign48760_e62522);
        let assign48760_e62524: f64 = (1.0 - assign48760_e62523);
        let assign48760_e62525: f64 = (locals.var_gf__blk1307 * assign48760_e62524);
        let assign48760_e62526: f64 = (1.0 + assign48760_e62525);
        (assign48760_e62526, ((locals.var_gf__blk1307_dn5 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn5) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn5))))))), ((locals.var_gf__blk1307_dn6 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn6) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn6))))))), ((locals.var_gf__blk1307_dn7 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn7) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn7))))))), ((locals.var_gf__blk1307_dn8 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn8) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn8))))))),)
    } else {
        (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8,)
    }
};
        locals.var_nscr__blk1333 = assign48760_e62528;
        locals.var_nscr__blk1333_dn5 = assign48760_e62528_d_n5;
        locals.var_nscr__blk1333_dn6 = assign48760_e62528_d_n6;
        locals.var_nscr__blk1333_dn7 = assign48760_e62528_d_n7;
        locals.var_nscr__blk1333_dn8 = assign48760_e62528_d_n8;
        locals.var_nscr__blk1333_rv = 0.0;

        let assign48770_e62531: f64 = if locals.var_xn_s__blk1332 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1463 = assign48770_e62531;
        locals.var_guard1463_rv = 0.0;

        let (assign48780_e62546, assign48780_e62546_d_n5, assign48780_e62546_d_n6, assign48780_e62546_d_n7, assign48780_e62546_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign48780_e62543: f64 = (-locals.var_xn_s__blk1332);
        let assign48780_e62544: f64 = (assign48780_e62543).exp();
        (assign48780_e62544, (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn5)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn6)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn7)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn8)),)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign48780_e62546;
        locals.var_delta_ns__blk1347_dn5 = assign48780_e62546_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign48780_e62546_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign48780_e62546_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign48780_e62546_d_n8;
        locals.var_delta_ns__blk1347_rv = 0.0;

        let (assign48790_e62582, assign48790_e62582_d_n5, assign48790_e62582_d_n6, assign48790_e62582_d_n7, assign48790_e62582_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1463 == 0.0)) {
        let assign48790_e62562: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62567: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62571: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62573: f64 = (assign48790_e62571 * 0.3333333333333333);
        let assign48790_e62574: f64 = (1.0 + assign48790_e62573);
        let assign48790_e62575: f64 = (assign48790_e62567 * assign48790_e62574);
        let assign48790_e62576: f64 = (0.5 * assign48790_e62575);
        let assign48790_e62577: f64 = (1.0 + assign48790_e62576);
        let assign48790_e62578: f64 = (assign48790_e62562 * assign48790_e62577);
        let assign48790_e62579: f64 = (1.0 + assign48790_e62578);
        let assign48790_e62580: f64 = (1e-200 / assign48790_e62579);
        (assign48790_e62580, (-((1e-200 * ((locals.var_xn_s__blk1332_dn5 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn5 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn5 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn6 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn6 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn6 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn7 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn7 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn7 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn8 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn8 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn8 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))),)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign48790_e62582;
        locals.var_delta_ns__blk1347_dn5 = assign48790_e62582_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign48790_e62582_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign48790_e62582_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign48790_e62582_d_n8;
        locals.var_delta_ns__blk1347_rv = 0.0;

        let (assign48800_e62599, assign48800_e62599_d_n5, assign48800_e62599_d_n6, assign48800_e62599_d_n7, assign48800_e62599_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let (assign48800_e62597,) = {
            if (locals.var_xn_s__blk1332 > 0.0) {
                (1.0,)
            } else {
                let assign48800_e62596: f64 = (-1.0);
                (assign48800_e62596,)
            }
        };
        (assign48800_e62597, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48800_e62599;
        locals.var_temp__blk936_dn5 = assign48800_e62599_d_n5;
        locals.var_temp__blk936_dn6 = assign48800_e62599_d_n6;
        locals.var_temp__blk936_dn7 = assign48800_e62599_d_n7;
        locals.var_temp__blk936_dn8 = assign48800_e62599_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign48810_e62631, assign48810_e62631_d_n5, assign48810_e62631_d_n6, assign48810_e62631_d_n7, assign48810_e62631_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign48810_e62611: f64 = (locals.var_temp__blk936 * locals.var_gf__blk1307);
        let assign48810_e62616: f64 = (1.0 - locals.var_xn_s__blk1332);
        let assign48810_e62617: f64 = (locals.var_delta_ns__blk1347 * assign48810_e62616);
        let assign48810_e62618: f64 = (1.0 - assign48810_e62617);
        let assign48810_e62619: f64 = (assign48810_e62611 * assign48810_e62618);
        let assign48810_e62624: f64 = (1.0 - locals.var_delta_ns__blk1347);
        let assign48810_e62625: f64 = (locals.var_xn_s__blk1332 * assign48810_e62624);
        let assign48810_e62626: f64 = (assign48810_e62625).sqrt();
        let assign48810_e62627: f64 = (2.0 * assign48810_e62626);
        let assign48810_e62628: f64 = (assign48810_e62619 / assign48810_e62627);
        let assign48810_e62629: f64 = (1.0 + assign48810_e62628);
        (assign48810_e62629, (((((((locals.var_temp__blk936_dn5 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn5)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn5 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn5)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn5 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn5))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn6 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn6)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn6 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn6)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn6 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn6))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn7 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn7)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn7 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn7)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn7 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn7))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn8 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn8)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn8 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn8)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn8 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn8))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)),)
    } else {
        (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8,)
    }
};
        locals.var_nscr__blk1333 = assign48810_e62631;
        locals.var_nscr__blk1333_dn5 = assign48810_e62631_d_n5;
        locals.var_nscr__blk1333_dn6 = assign48810_e62631_d_n6;
        locals.var_nscr__blk1333_dn7 = assign48810_e62631_d_n7;
        locals.var_nscr__blk1333_dn8 = assign48810_e62631_d_n8;
        locals.var_nscr__blk1333_rv = 0.0;

        let (assign48820_e62647, assign48820_e62647_d_n5, assign48820_e62647_d_n6, assign48820_e62647_d_n7, assign48820_e62647_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 == 0.0)) {
        let assign48820_e62641: f64 = (0.5 * locals.var_gf__blk1307);
        let assign48820_e62643: f64 = (locals.var_xn_s__blk1332).sqrt();
        let assign48820_e62644: f64 = (assign48820_e62641 / assign48820_e62643);
        let assign48820_e62645: f64 = (1.0 + assign48820_e62644);
        (assign48820_e62645, ((((0.5 * locals.var_gf__blk1307_dn5) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn5 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn6) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn6 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn7) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn7 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn8) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn8 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)),)
    } else {
        (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8,)
    }
};
        locals.var_nscr__blk1333 = assign48820_e62647;
        locals.var_nscr__blk1333_dn5 = assign48820_e62647_d_n5;
        locals.var_nscr__blk1333_dn6 = assign48820_e62647_d_n6;
        locals.var_nscr__blk1333_dn7 = assign48820_e62647_d_n7;
        locals.var_nscr__blk1333_dn8 = assign48820_e62647_d_n8;
        locals.var_nscr__blk1333_rv = 0.0;

        let (assign48830_e62665, assign48830_e62665_d_n5, assign48830_e62665_d_n6, assign48830_e62665_d_n7, assign48830_e62665_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48830_e62654: f64 = (locals.var_xn_s__blk1332).sqrt();
        let assign48830_e62655: f64 = (locals.var_gf__blk1307 * assign48830_e62654);
        let assign48830_e62656: f64 = (locals.var_xn_s__blk1332 + assign48830_e62655);
        let assign48830_e62660: f64 = (locals.var_nscr__blk1333 - 1.0);
        let assign48830_e62661: f64 = (assign48830_e62660).ln();
        let assign48830_e62662: f64 = (locals.var_nscr__blk1333 * assign48830_e62661);
        let assign48830_e62663: f64 = (assign48830_e62656 - assign48830_e62662);
        (assign48830_e62663, ((locals.var_xn_s__blk1332_dn5 + ((locals.var_gf__blk1307_dn5 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn5 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn5 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn5 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn6 + ((locals.var_gf__blk1307_dn6 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn6 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn6 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn6 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn7 + ((locals.var_gf__blk1307_dn7 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn7 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn7 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn7 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn8 + ((locals.var_gf__blk1307_dn8 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn8 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn8 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn8 / assign48830_e62660)))),)
    } else {
        (locals.var_xthscr__blk1334, locals.var_xthscr__blk1334_dn5, locals.var_xthscr__blk1334_dn6, locals.var_xthscr__blk1334_dn7, locals.var_xthscr__blk1334_dn8,)
    }
};
        locals.var_xthscr__blk1334 = assign48830_e62665;
        locals.var_xthscr__blk1334_dn5 = assign48830_e62665_d_n5;
        locals.var_xthscr__blk1334_dn6 = assign48830_e62665_d_n6;
        locals.var_xthscr__blk1334_dn7 = assign48830_e62665_d_n7;
        locals.var_xthscr__blk1334_dn8 = assign48830_e62665_d_n8;
        locals.var_xthscr__blk1334_rv = 0.0;

        let (assign48840_e62675, assign48840_e62675_d_n5, assign48840_e62675_d_n6, assign48840_e62675_d_n7, assign48840_e62675_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48840_e62671: f64 = (locals.var_xg__blk1326 - locals.var_xthscr__blk1334);
        let assign48840_e62673: f64 = (assign48840_e62671 / locals.var_nscr__blk1333);
        (assign48840_e62673, ((((locals.var_xg__blk1326_dn5 - locals.var_xthscr__blk1334_dn5) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn5)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn6 - locals.var_xthscr__blk1334_dn6) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn6)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn7 - locals.var_xthscr__blk1334_dn7) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn7)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn8 - locals.var_xthscr__blk1334_dn8) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn8)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)),)
    } else {
        (locals.var_xgtscr__blk1335, locals.var_xgtscr__blk1335_dn5, locals.var_xgtscr__blk1335_dn6, locals.var_xgtscr__blk1335_dn7, locals.var_xgtscr__blk1335_dn8,)
    }
};
        locals.var_xgtscr__blk1335 = assign48840_e62675;
        locals.var_xgtscr__blk1335_dn5 = assign48840_e62675_d_n5;
        locals.var_xgtscr__blk1335_dn6 = assign48840_e62675_d_n6;
        locals.var_xgtscr__blk1335_dn7 = assign48840_e62675_d_n7;
        locals.var_xgtscr__blk1335_dn8 = assign48840_e62675_d_n8;
        locals.var_xgtscr__blk1335_rv = 0.0;

        let (assign48850_e62692, assign48850_e62692_d_n5, assign48850_e62692_d_n6, assign48850_e62692_d_n7, assign48850_e62692_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48850_e62681: f64 = (0.5 * locals.var_gf2__blk1308);
        let assign48850_e62685: f64 = (8.0 / locals.var_gf2__blk1308);
        let assign48850_e62686: f64 = (1.0 + assign48850_e62685);
        let assign48850_e62687: f64 = (assign48850_e62686).sqrt();
        let assign48850_e62689: f64 = (assign48850_e62687 - 1.0);
        let assign48850_e62690: f64 = (assign48850_e62681 * assign48850_e62689);
        (assign48850_e62690, (((0.5 * locals.var_gf2__blk1308_dn5) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn5) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn6) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn6) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn7) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn7) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn8) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn8) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))),)
    } else {
        (locals.var_qbscr__blk1341, locals.var_qbscr__blk1341_dn5, locals.var_qbscr__blk1341_dn6, locals.var_qbscr__blk1341_dn7, locals.var_qbscr__blk1341_dn8,)
    }
};
        locals.var_qbscr__blk1341 = assign48850_e62692;
        locals.var_qbscr__blk1341_dn5 = assign48850_e62692_d_n5;
        locals.var_qbscr__blk1341_dn6 = assign48850_e62692_d_n6;
        locals.var_qbscr__blk1341_dn7 = assign48850_e62692_d_n7;
        locals.var_qbscr__blk1341_dn8 = assign48850_e62692_d_n8;
        locals.var_qbscr__blk1341_rv = 0.0;

        let (assign48860_e62698, assign48860_e62698_d_n5, assign48860_e62698_d_n6, assign48860_e62698_d_n7, assign48860_e62698_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8,)
    }
};
        locals.var_qiscr__blk1340 = assign48860_e62698;
        locals.var_qiscr__blk1340_dn5 = assign48860_e62698_d_n5;
        locals.var_qiscr__blk1340_dn6 = assign48860_e62698_d_n6;
        locals.var_qiscr__blk1340_dn7 = assign48860_e62698_d_n7;
        locals.var_qiscr__blk1340_dn8 = assign48860_e62698_d_n8;
        locals.var_qiscr__blk1340_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_37(
        locals: &mut StampLocals,
    ) {
        let (assign48870_e62704, assign48870_e62704_d_n5, assign48870_e62704_d_n6, assign48870_e62704_d_n7, assign48870_e62704_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fscr__blk1342, locals.var_fscr__blk1342_dn5, locals.var_fscr__blk1342_dn6, locals.var_fscr__blk1342_dn7, locals.var_fscr__blk1342_dn8,)
    }
};
        locals.var_fscr__blk1342 = assign48870_e62704;
        locals.var_fscr__blk1342_dn5 = assign48870_e62704_d_n5;
        locals.var_fscr__blk1342_dn6 = assign48870_e62704_d_n6;
        locals.var_fscr__blk1342_dn7 = assign48870_e62704_d_n7;
        locals.var_fscr__blk1342_dn8 = assign48870_e62704_d_n8;
        locals.var_fscr__blk1342_rv = 0.0;

        let assign48880_e62707: f64 = (-30.0);
        let assign48880_e62708: f64 = if locals.var_xgtscr__blk1335 > assign48880_e62707 { 1.0 } else { 0.0 };
        locals.var_guard1464 = assign48880_e62708;
        locals.var_guard1464_rv = 0.0;

        let (assign48890_e62720, assign48890_e62720_d_n5, assign48890_e62720_d_n6, assign48890_e62720_d_n7, assign48890_e62720_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48890_e62716: f64 = (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335);
        let assign48890_e62718: f64 = (assign48890_e62716 - 1.0);
        (assign48890_e62718, ((locals.var_nscr__blk1333_dn5 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn5)), ((locals.var_nscr__blk1333_dn6 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn6)), ((locals.var_nscr__blk1333_dn7 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn7)), ((locals.var_nscr__blk1333_dn8 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn8)),)
    } else {
        (locals.var_xgtscr0__blk1336, locals.var_xgtscr0__blk1336_dn5, locals.var_xgtscr0__blk1336_dn6, locals.var_xgtscr0__blk1336_dn7, locals.var_xgtscr0__blk1336_dn8,)
    }
};
        locals.var_xgtscr0__blk1336 = assign48890_e62720;
        locals.var_xgtscr0__blk1336_dn5 = assign48890_e62720_d_n5;
        locals.var_xgtscr0__blk1336_dn6 = assign48890_e62720_d_n6;
        locals.var_xgtscr0__blk1336_dn7 = assign48890_e62720_d_n7;
        locals.var_xgtscr0__blk1336_dn8 = assign48890_e62720_d_n8;
        locals.var_xgtscr0__blk1336_rv = 0.0;

        let (assign48900_e62737, assign48900_e62737_d_n5, assign48900_e62737_d_n6, assign48900_e62737_d_n7, assign48900_e62737_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48900_e62730: f64 = (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336);
        let assign48900_e62732: f64 = (assign48900_e62730 + 10.0);
        let assign48900_e62733: f64 = (assign48900_e62732).sqrt();
        let assign48900_e62734: f64 = (locals.var_xgtscr0__blk1336 + assign48900_e62733);
        let assign48900_e62735: f64 = (0.5 * assign48900_e62734);
        (assign48900_e62735, (0.5 * (locals.var_xgtscr0__blk1336_dn5 + (((locals.var_xgtscr0__blk1336_dn5 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn5)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn6 + (((locals.var_xgtscr0__blk1336_dn6 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn6)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn7 + (((locals.var_xgtscr0__blk1336_dn7 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn7)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn8 + (((locals.var_xgtscr0__blk1336_dn8 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn8)) / (2.0 * assign48900_e62733)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48900_e62737;
        locals.var_temp__blk936_dn5 = assign48900_e62737_d_n5;
        locals.var_temp__blk936_dn6 = assign48900_e62737_d_n6;
        locals.var_temp__blk936_dn7 = assign48900_e62737_d_n7;
        locals.var_temp__blk936_dn8 = assign48900_e62737_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign48910_e62748, assign48910_e62748_d_n5, assign48910_e62748_d_n6, assign48910_e62748_d_n7, assign48910_e62748_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48910_e62745: f64 = (locals.var_temp__blk936).ln();
        let assign48910_e62746: f64 = (locals.var_xgtscr__blk1335 - assign48910_e62745);
        (assign48910_e62746, (locals.var_xgtscr__blk1335_dn5 - (locals.var_temp__blk936_dn5 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn6 - (locals.var_temp__blk936_dn6 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn7 - (locals.var_temp__blk936_dn7 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn8 - (locals.var_temp__blk936_dn8 / locals.var_temp__blk936)),)
    } else {
        (locals.var_qiscr0si__blk1337, locals.var_qiscr0si__blk1337_dn5, locals.var_qiscr0si__blk1337_dn6, locals.var_qiscr0si__blk1337_dn7, locals.var_qiscr0si__blk1337_dn8,)
    }
};
        locals.var_qiscr0si__blk1337 = assign48910_e62748;
        locals.var_qiscr0si__blk1337_dn5 = assign48910_e62748_d_n5;
        locals.var_qiscr0si__blk1337_dn6 = assign48910_e62748_d_n6;
        locals.var_qiscr0si__blk1337_dn7 = assign48910_e62748_d_n7;
        locals.var_qiscr0si__blk1337_dn8 = assign48910_e62748_d_n8;
        locals.var_qiscr0si__blk1337_rv = 0.0;

        let (assign48920_e62765, assign48920_e62765_d_n5, assign48920_e62765_d_n6, assign48920_e62765_d_n7, assign48920_e62765_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48920_e62758: f64 = (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337);
        let assign48920_e62760: f64 = (assign48920_e62758 + 2.0);
        let assign48920_e62761: f64 = (assign48920_e62760).sqrt();
        let assign48920_e62762: f64 = (locals.var_qiscr0si__blk1337 + assign48920_e62761);
        let assign48920_e62763: f64 = (0.5 * assign48920_e62762);
        (assign48920_e62763, (0.5 * (locals.var_qiscr0si__blk1337_dn5 + (((locals.var_qiscr0si__blk1337_dn5 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn5)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn6 + (((locals.var_qiscr0si__blk1337_dn6 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn6)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn7 + (((locals.var_qiscr0si__blk1337_dn7 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn7)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn8 + (((locals.var_qiscr0si__blk1337_dn8 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn8)) / (2.0 * assign48920_e62761)))),)
    } else {
        (locals.var_qiscr0__blk1338, locals.var_qiscr0__blk1338_dn5, locals.var_qiscr0__blk1338_dn6, locals.var_qiscr0__blk1338_dn7, locals.var_qiscr0__blk1338_dn8,)
    }
};
        locals.var_qiscr0__blk1338 = assign48920_e62765;
        locals.var_qiscr0__blk1338_dn5 = assign48920_e62765_d_n5;
        locals.var_qiscr0__blk1338_dn6 = assign48920_e62765_d_n6;
        locals.var_qiscr0__blk1338_dn7 = assign48920_e62765_d_n7;
        locals.var_qiscr0__blk1338_dn8 = assign48920_e62765_d_n8;
        locals.var_qiscr0__blk1338_rv = 0.0;

        let assign48930_e62768: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48930_e62770: f64 = if assign48930_e62768 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1465 = assign48930_e62770;
        locals.var_guard1465_rv = 0.0;

        let (assign48940_e62783, assign48940_e62783_d_n5, assign48940_e62783_d_n6, assign48940_e62783_d_n7, assign48940_e62783_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign48940_e62780: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48940_e62781: f64 = (assign48940_e62780).exp();
        (assign48940_e62781, (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48940_e62783;
        locals.var_temp__blk936_dn5 = assign48940_e62783_d_n5;
        locals.var_temp__blk936_dn6 = assign48940_e62783_d_n6;
        locals.var_temp__blk936_dn7 = assign48940_e62783_d_n7;
        locals.var_temp__blk936_dn8 = assign48940_e62783_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign48950_e62822, assign48950_e62822_d_n5, assign48950_e62822_d_n6, assign48950_e62822_d_n7, assign48950_e62822_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 == 0.0)) {
        let assign48950_e62796: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48950_e62798: f64 = (assign48950_e62796 - 230.25850929940458);
        let assign48950_e62803: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48950_e62805: f64 = (assign48950_e62803 - 230.25850929940458);
        let assign48950_e62809: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48950_e62811: f64 = (assign48950_e62809 - 230.25850929940458);
        let assign48950_e62813: f64 = (assign48950_e62811 * 0.3333333333333333);
        let assign48950_e62814: f64 = (1.0 + assign48950_e62813);
        let assign48950_e62815: f64 = (assign48950_e62805 * assign48950_e62814);
        let assign48950_e62816: f64 = (0.5 * assign48950_e62815);
        let assign48950_e62817: f64 = (1.0 + assign48950_e62816);
        let assign48950_e62818: f64 = (assign48950_e62798 * assign48950_e62817);
        let assign48950_e62819: f64 = (1.0 + assign48950_e62818);
        let assign48950_e62820: f64 = (1e100 * assign48950_e62819);
        (assign48950_e62820, (1e100 * (((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48950_e62822;
        locals.var_temp__blk936_dn5 = assign48950_e62822_d_n5;
        locals.var_temp__blk936_dn6 = assign48950_e62822_d_n6;
        locals.var_temp__blk936_dn7 = assign48950_e62822_d_n7;
        locals.var_temp__blk936_dn8 = assign48950_e62822_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign48960_e62832, assign48960_e62832_d_n5, assign48960_e62832_d_n6, assign48960_e62832_d_n7, assign48960_e62832_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48960_e62830: f64 = (locals.var_temp__blk936 / locals.var_nscr__blk1333);
        (assign48960_e62830, (((locals.var_temp__blk936_dn5 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn5)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn6 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn6)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn7 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn7)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn8 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn8)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)),)
    } else {
        (locals.var_dscr0__blk1339, locals.var_dscr0__blk1339_dn5, locals.var_dscr0__blk1339_dn6, locals.var_dscr0__blk1339_dn7, locals.var_dscr0__blk1339_dn8,)
    }
};
        locals.var_dscr0__blk1339 = assign48960_e62832;
        locals.var_dscr0__blk1339_dn5 = assign48960_e62832_d_n5;
        locals.var_dscr0__blk1339_dn6 = assign48960_e62832_d_n6;
        locals.var_dscr0__blk1339_dn7 = assign48960_e62832_d_n7;
        locals.var_dscr0__blk1339_dn8 = assign48960_e62832_d_n8;
        locals.var_dscr0__blk1339_rv = 0.0;

        let (assign48970_e62846, assign48970_e62846_d_n5, assign48970_e62846_d_n6, assign48970_e62846_d_n7, assign48970_e62846_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48970_e62841: f64 = (locals.var_qiscr0__blk1338 + 1.0);
        let assign48970_e62842: f64 = (2.0 * assign48970_e62841);
        let assign48970_e62844: f64 = (assign48970_e62842 - locals.var_dscr0__blk1339);
        (assign48970_e62844, ((2.0 * locals.var_qiscr0__blk1338_dn5) - locals.var_dscr0__blk1339_dn5), ((2.0 * locals.var_qiscr0__blk1338_dn6) - locals.var_dscr0__blk1339_dn6), ((2.0 * locals.var_qiscr0__blk1338_dn7) - locals.var_dscr0__blk1339_dn7), ((2.0 * locals.var_qiscr0__blk1338_dn8) - locals.var_dscr0__blk1339_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48970_e62846;
        locals.var_temp__blk936_dn5 = assign48970_e62846_d_n5;
        locals.var_temp__blk936_dn6 = assign48970_e62846_d_n6;
        locals.var_temp__blk936_dn7 = assign48970_e62846_d_n7;
        locals.var_temp__blk936_dn8 = assign48970_e62846_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign48980_e62849: f64 = if locals.var_dscr0__blk1339 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1466 = assign48980_e62849;
        locals.var_guard1466_rv = 0.0;

        let (assign48990_e62874, assign48990_e62874_d_n5, assign48990_e62874_d_n6, assign48990_e62874_d_n7, assign48990_e62874_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1466 != 0.0)) {
        let assign48990_e62862: f64 = (locals.var_dscr0__blk1339 * locals.var_temp__blk936);
        let assign48990_e62863: f64 = (1.0 + assign48990_e62862);
        let assign48990_e62864: f64 = (assign48990_e62863).sqrt();
        let assign48990_e62866: f64 = (assign48990_e62864 - 1.0);
        let assign48990_e62868: f64 = (assign48990_e62866 / locals.var_dscr0__blk1339);
        let assign48990_e62869: f64 = (locals.var_qiscr0__blk1338 - assign48990_e62868);
        let assign48990_e62871: f64 = (assign48990_e62869 + 1.0);
        let assign48990_e62872: f64 = (locals.var_nscr__blk1333 * assign48990_e62871);
        (assign48990_e62872, ((locals.var_nscr__blk1333_dn5 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn5 - ((((((locals.var_dscr0__blk1339_dn5 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn5)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn5)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn6 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn6 - ((((((locals.var_dscr0__blk1339_dn6 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn6)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn6)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn7 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn7 - ((((((locals.var_dscr0__blk1339_dn7 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn7)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn7)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn8 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn8 - ((((((locals.var_dscr0__blk1339_dn8 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn8)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn8)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))),)
    } else {
        (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8,)
    }
};
        locals.var_qiscr__blk1340 = assign48990_e62874;
        locals.var_qiscr__blk1340_dn5 = assign48990_e62874_d_n5;
        locals.var_qiscr__blk1340_dn6 = assign48990_e62874_d_n6;
        locals.var_qiscr__blk1340_dn7 = assign48990_e62874_d_n7;
        locals.var_qiscr__blk1340_dn8 = assign48990_e62874_d_n8;
        locals.var_qiscr__blk1340_rv = 0.0;

        let (assign49000_e62897, assign49000_e62897_d_n5, assign49000_e62897_d_n6, assign49000_e62897_d_n7, assign49000_e62897_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign49000_e62885: f64 = (locals.var_nscr__blk1333 * 0.5);
        let assign49000_e62887: f64 = (assign49000_e62885 * locals.var_dscr0__blk1339);
        let assign49000_e62891: f64 = (0.25 * locals.var_temp__blk936);
        let assign49000_e62893: f64 = (assign49000_e62891 * locals.var_temp__blk936);
        let assign49000_e62894: f64 = (1.0 + assign49000_e62893);
        let assign49000_e62895: f64 = (assign49000_e62887 * assign49000_e62894);
        (assign49000_e62895, (((((locals.var_nscr__blk1333_dn5 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn5)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn5)))), (((((locals.var_nscr__blk1333_dn6 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn6)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn6)))), (((((locals.var_nscr__blk1333_dn7 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn7)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn7)))), (((((locals.var_nscr__blk1333_dn8 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn8)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn8)))),)
    } else {
        (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8,)
    }
};
        locals.var_qiscr__blk1340 = assign49000_e62897;
        locals.var_qiscr__blk1340_dn5 = assign49000_e62897_d_n5;
        locals.var_qiscr__blk1340_dn6 = assign49000_e62897_d_n6;
        locals.var_qiscr__blk1340_dn7 = assign49000_e62897_d_n7;
        locals.var_qiscr__blk1340_dn8 = assign49000_e62897_d_n8;
        locals.var_qiscr__blk1340_rv = 0.0;

        let (assign49010_e62926, assign49010_e62926_d_n5, assign49010_e62926_d_n6, assign49010_e62926_d_n7, assign49010_e62926_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign49010_e62906: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
        let assign49010_e62908: f64 = (assign49010_e62906 + 2.0);
        let assign49010_e62911: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
        let assign49010_e62913: f64 = (assign49010_e62911 - 2.0);
        let assign49010_e62916: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
        let assign49010_e62918: f64 = (assign49010_e62916 - 2.0);
        let assign49010_e62919: f64 = (assign49010_e62913 * assign49010_e62918);
        let assign49010_e62921: f64 = (assign49010_e62919 + 1.0);
        let assign49010_e62922: f64 = (assign49010_e62921).sqrt();
        let assign49010_e62923: f64 = (assign49010_e62908 + assign49010_e62922);
        let assign49010_e62924: f64 = (0.5 * assign49010_e62923);
        (assign49010_e62924, (0.5 * ((locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5) + ((((locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6) + ((((locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7) + ((((locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8) + ((((locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8))) / (2.0 * assign49010_e62922)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign49010_e62926;
        locals.var_temp__blk936_dn5 = assign49010_e62926_d_n5;
        locals.var_temp__blk936_dn6 = assign49010_e62926_d_n6;
        locals.var_temp__blk936_dn7 = assign49010_e62926_d_n7;
        locals.var_temp__blk936_dn8 = assign49010_e62926_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign49020_e62947, assign49020_e62947_d_n5, assign49020_e62947_d_n6, assign49020_e62947_d_n7, assign49020_e62947_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign49020_e62934: f64 = (0.5 * locals.var_gf2__blk1308);
        let assign49020_e62938: f64 = (4.0 / locals.var_gf2__blk1308);
        let assign49020_e62940: f64 = (assign49020_e62938 * locals.var_temp__blk936);
        let assign49020_e62941: f64 = (1.0 + assign49020_e62940);
        let assign49020_e62942: f64 = (assign49020_e62941).sqrt();
        let assign49020_e62944: f64 = (assign49020_e62942 - 1.0);
        let assign49020_e62945: f64 = (assign49020_e62934 * assign49020_e62944);
        (assign49020_e62945, (((0.5 * locals.var_gf2__blk1308_dn5) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn5) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn5)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn6) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn6) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn6)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn7) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn7) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn7)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn8) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn8) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn8)) / (2.0 * assign49020_e62942)))),)
    } else {
        (locals.var_qbscr__blk1341, locals.var_qbscr__blk1341_dn5, locals.var_qbscr__blk1341_dn6, locals.var_qbscr__blk1341_dn7, locals.var_qbscr__blk1341_dn8,)
    }
};
        locals.var_qbscr__blk1341 = assign49020_e62947;
        locals.var_qbscr__blk1341_dn5 = assign49020_e62947_d_n5;
        locals.var_qbscr__blk1341_dn6 = assign49020_e62947_d_n6;
        locals.var_qbscr__blk1341_dn7 = assign49020_e62947_d_n7;
        locals.var_qbscr__blk1341_dn8 = assign49020_e62947_d_n8;
        locals.var_qbscr__blk1341_rv = 0.0;

        let (assign49030_e62959, assign49030_e62959_d_n5, assign49030_e62959_d_n6, assign49030_e62959_d_n7, assign49030_e62959_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign49030_e62956: f64 = (locals.var_qbscr__blk1341 + locals.var_qiscr__blk1340);
        let assign49030_e62957: f64 = (locals.var_qbscr__blk1341 / assign49030_e62956);
        (assign49030_e62957, (((locals.var_qbscr__blk1341_dn5 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn5 + locals.var_qiscr__blk1340_dn5))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn6 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn6 + locals.var_qiscr__blk1340_dn6))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn7 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn7 + locals.var_qiscr__blk1340_dn7))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn8 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn8 + locals.var_qiscr__blk1340_dn8))) / (assign49030_e62956 * assign49030_e62956)),)
    } else {
        (locals.var_fscr__blk1342, locals.var_fscr__blk1342_dn5, locals.var_fscr__blk1342_dn6, locals.var_fscr__blk1342_dn7, locals.var_fscr__blk1342_dn8,)
    }
};
        locals.var_fscr__blk1342 = assign49030_e62959;
        locals.var_fscr__blk1342_dn5 = assign49030_e62959_d_n5;
        locals.var_fscr__blk1342_dn6 = assign49030_e62959_d_n6;
        locals.var_fscr__blk1342_dn7 = assign49030_e62959_d_n7;
        locals.var_fscr__blk1342_dn8 = assign49030_e62959_d_n8;
        locals.var_fscr__blk1342_rv = 0.0;

        let (assign49040_e62971, assign49040_e62971_d_n5, assign49040_e62971_d_n6, assign49040_e62971_d_n7, assign49040_e62971_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign49040_e62968: f64 = (locals.var_fscr__blk1342 * locals.var_delxb__blk1330);
        let assign49040_e62969: f64 = (locals.var_xno_s__blk1331 - assign49040_e62968);
        (assign49040_e62969, (locals.var_xno_s__blk1331_dn5 - ((locals.var_fscr__blk1342_dn5 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn5))), (locals.var_xno_s__blk1331_dn6 - ((locals.var_fscr__blk1342_dn6 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn6))), (locals.var_xno_s__blk1331_dn7 - ((locals.var_fscr__blk1342_dn7 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn7))), (locals.var_xno_s__blk1331_dn8 - ((locals.var_fscr__blk1342_dn8 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn8))),)
    } else {
        (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8,)
    }
};
        locals.var_xn_s__blk1332 = assign49040_e62971;
        locals.var_xn_s__blk1332_dn5 = assign49040_e62971_d_n5;
        locals.var_xn_s__blk1332_dn6 = assign49040_e62971_d_n6;
        locals.var_xn_s__blk1332_dn7 = assign49040_e62971_d_n7;
        locals.var_xn_s__blk1332_dn8 = assign49040_e62971_d_n8;
        locals.var_xn_s__blk1332_rv = 0.0;

        let (assign49050_e62981, assign49050_e62981_d_n5, assign49050_e62981_d_n6, assign49050_e62981_d_n7, assign49050_e62981_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49050_e62978: f64 = (locals.var_gf__blk1307 * 0.7071067811865475);
        let assign49050_e62979: f64 = (1.0 + assign49050_e62978);
        (assign49050_e62979, (locals.var_gf__blk1307_dn5 * 0.7071067811865475), (locals.var_gf__blk1307_dn6 * 0.7071067811865475), (locals.var_gf__blk1307_dn7 * 0.7071067811865475), (locals.var_gf__blk1307_dn8 * 0.7071067811865475),)
    } else {
        (locals.var_xi__blk1343, locals.var_xi__blk1343_dn5, locals.var_xi__blk1343_dn6, locals.var_xi__blk1343_dn7, locals.var_xi__blk1343_dn8,)
    }
};
        locals.var_xi__blk1343 = assign49050_e62981;
        locals.var_xi__blk1343_dn5 = assign49050_e62981_d_n5;
        locals.var_xi__blk1343_dn6 = assign49050_e62981_d_n6;
        locals.var_xi__blk1343_dn7 = assign49050_e62981_d_n7;
        locals.var_xi__blk1343_dn8 = assign49050_e62981_d_n8;
        locals.var_xi__blk1343_rv = 0.0;

        let (assign49060_e62989, assign49060_e62989_d_n5, assign49060_e62989_d_n6, assign49060_e62989_d_n7, assign49060_e62989_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49060_e62987: f64 = (1e-5 * locals.var_xi__blk1343);
        (assign49060_e62987, (1e-5 * locals.var_xi__blk1343_dn5), (1e-5 * locals.var_xi__blk1343_dn6), (1e-5 * locals.var_xi__blk1343_dn7), (1e-5 * locals.var_xi__blk1343_dn8),)
    } else {
        (locals.var_margin__blk1344, locals.var_margin__blk1344_dn5, locals.var_margin__blk1344_dn6, locals.var_margin__blk1344_dn7, locals.var_margin__blk1344_dn8,)
    }
};
        locals.var_margin__blk1344 = assign49060_e62989;
        locals.var_margin__blk1344_dn5 = assign49060_e62989_d_n5;
        locals.var_margin__blk1344_dn6 = assign49060_e62989_d_n6;
        locals.var_margin__blk1344_dn7 = assign49060_e62989_d_n7;
        locals.var_margin__blk1344_dn8 = assign49060_e62989_d_n8;
        locals.var_margin__blk1344_rv = 0.0;

        let (assign49070_e62997, assign49070_e62997_d_n5, assign49070_e62997_d_n6, assign49070_e62997_d_n7, assign49070_e62997_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49070_e62995: f64 = (1.0 / locals.var_xi__blk1343);
        (assign49070_e62995, (-(locals.var_xi__blk1343_dn5 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn6 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn7 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn8 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))),)
    } else {
        (locals.var_inv_xi__blk1345, locals.var_inv_xi__blk1345_dn5, locals.var_inv_xi__blk1345_dn6, locals.var_inv_xi__blk1345_dn7, locals.var_inv_xi__blk1345_dn8,)
    }
};
        locals.var_inv_xi__blk1345 = assign49070_e62997;
        locals.var_inv_xi__blk1345_dn5 = assign49070_e62997_d_n5;
        locals.var_inv_xi__blk1345_dn6 = assign49070_e62997_d_n6;
        locals.var_inv_xi__blk1345_dn7 = assign49070_e62997_d_n7;
        locals.var_inv_xi__blk1345_dn8 = assign49070_e62997_d_n8;
        locals.var_inv_xi__blk1345_rv = 0.0;

        let (assign49080_e63003, assign49080_e63003_d_n5, assign49080_e63003_d_n6, assign49080_e63003_d_n7, assign49080_e63003_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8,)
    }
};
        locals.var_sp_s_x1__blk1452 = assign49080_e63003;
        locals.var_sp_s_x1__blk1452_dn5 = assign49080_e63003_d_n5;
        locals.var_sp_s_x1__blk1452_dn6 = assign49080_e63003_d_n6;
        locals.var_sp_s_x1__blk1452_dn7 = assign49080_e63003_d_n7;
        locals.var_sp_s_x1__blk1452_dn8 = assign49080_e63003_d_n8;
        locals.var_sp_s_x1__blk1452_rv = 0.0;

        let (assign49090_e63009, assign49090_e63009_d_n5, assign49090_e63009_d_n6, assign49090_e63009_d_n7, assign49090_e63009_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign49090_e63009;
        locals.var_x_s__blk1346_dn5 = assign49090_e63009_d_n5;
        locals.var_x_s__blk1346_dn6 = assign49090_e63009_d_n6;
        locals.var_x_s__blk1346_dn7 = assign49090_e63009_d_n7;
        locals.var_x_s__blk1346_dn8 = assign49090_e63009_d_n8;
        locals.var_x_s__blk1346_rv = 0.0;

        let assign49100_e63012: f64 = if locals.var_xn_s__blk1332 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1467 = assign49100_e63012;
        locals.var_guard1467_rv = 0.0;

        let (assign49110_e63022, assign49110_e63022_d_n5, assign49110_e63022_d_n6, assign49110_e63022_d_n7, assign49110_e63022_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign49110_e63019: f64 = (-locals.var_xn_s__blk1332);
        let assign49110_e63020: f64 = (assign49110_e63019).exp();
        (assign49110_e63020, (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn5)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn6)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn7)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn8)),)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign49110_e63022;
        locals.var_delta_ns__blk1347_dn5 = assign49110_e63022_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign49110_e63022_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign49110_e63022_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign49110_e63022_d_n8;
        locals.var_delta_ns__blk1347_rv = 0.0;

        let (assign49120_e63053, assign49120_e63053_d_n5, assign49120_e63053_d_n6, assign49120_e63053_d_n7, assign49120_e63053_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1467 == 0.0)) {
        let assign49120_e63033: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign49120_e63038: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign49120_e63042: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign49120_e63044: f64 = (assign49120_e63042 * 0.3333333333333333);
        let assign49120_e63045: f64 = (1.0 + assign49120_e63044);
        let assign49120_e63046: f64 = (assign49120_e63038 * assign49120_e63045);
        let assign49120_e63047: f64 = (0.5 * assign49120_e63046);
        let assign49120_e63048: f64 = (1.0 + assign49120_e63047);
        let assign49120_e63049: f64 = (assign49120_e63033 * assign49120_e63048);
        let assign49120_e63050: f64 = (1.0 + assign49120_e63049);
        let assign49120_e63051: f64 = (1e-200 / assign49120_e63050);
        (assign49120_e63051, (-((1e-200 * ((locals.var_xn_s__blk1332_dn5 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn5 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn5 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn6 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn6 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn6 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn7 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn7 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn7 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn8 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn8 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn8 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))),)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign49120_e63053;
        locals.var_delta_ns__blk1347_dn5 = assign49120_e63053_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign49120_e63053_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign49120_e63053_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign49120_e63053_d_n8;
        locals.var_delta_ns__blk1347_rv = 0.0;

        let assign49130_e63055: f64 = (locals.var_xg__blk1326).abs();
        let assign49130_e63057: f64 = if assign49130_e63055 <= locals.var_margin__blk1344 { 1.0 } else { 0.0 };
        locals.var_guard1468 = assign49130_e63057;
        locals.var_guard1468_rv = 0.0;

        let (assign49140_e63071, assign49140_e63071_d_n5, assign49140_e63071_d_n6, assign49140_e63071_d_n7, assign49140_e63071_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 != 0.0)) {
        let assign49140_e63065: f64 = (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345);
        let assign49140_e63067: f64 = (assign49140_e63065 * 0.16666666666666666);
        let assign49140_e63069: f64 = (assign49140_e63067 * 0.7071067811865475);
        (assign49140_e63069, ((((locals.var_inv_xi__blk1345_dn5 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn6 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn7 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn8 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign49140_e63071;
        locals.var_sp_s_temp1__blk1432_dn5 = assign49140_e63071_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign49140_e63071_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign49140_e63071_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign49140_e63071_d_n8;
        locals.var_sp_s_temp1__blk1432_rv = 0.0;

        let (assign49150_e63093, assign49150_e63093_d_n5, assign49150_e63093_d_n6, assign49150_e63093_d_n7, assign49150_e63093_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 != 0.0)) {
        let assign49150_e63079: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
        let assign49150_e63084: f64 = (1.0 - locals.var_delta_ns__blk1347);
        let assign49150_e63085: f64 = (locals.var_xg__blk1326 * assign49150_e63084);
        let assign49150_e63087: f64 = (assign49150_e63085 * locals.var_gf__blk1307);
        let assign49150_e63089: f64 = (assign49150_e63087 * locals.var_sp_s_temp1__blk1432);
        let assign49150_e63090: f64 = (1.0 + assign49150_e63089);
        let assign49150_e63091: f64 = (assign49150_e63079 * assign49150_e63090);
        (assign49150_e63091, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn5 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn5))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn5)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn6 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn6))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn6)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn7 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn7))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn7)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn8 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn8))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn8)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn8)))),)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign49150_e63093;
        locals.var_x_s__blk1346_dn5 = assign49150_e63093_d_n5;
        locals.var_x_s__blk1346_dn6 = assign49150_e63093_d_n6;
        locals.var_x_s__blk1346_dn7 = assign49150_e63093_d_n7;
        locals.var_x_s__blk1346_dn8 = assign49150_e63093_d_n8;
        locals.var_x_s__blk1346_rv = 0.0;

        let assign49160_e63096: f64 = (-locals.var_margin__blk1344);
        let assign49160_e63097: f64 = if locals.var_xg__blk1326 < assign49160_e63096 { 1.0 } else { 0.0 };
        locals.var_guard1469 = assign49160_e63097;
        locals.var_guard1469_rv = 0.0;

        let (assign49170_e63109, assign49170_e63109_d_n5, assign49170_e63109_d_n6, assign49170_e63109_d_n7, assign49170_e63109_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49170_e63107: f64 = (-locals.var_xg__blk1326);
        (assign49170_e63107, (-locals.var_xg__blk1326_dn5), (-locals.var_xg__blk1326_dn6), (-locals.var_xg__blk1326_dn7), (-locals.var_xg__blk1326_dn8),)
    } else {
        (locals.var_sp_s_yg__blk1434, locals.var_sp_s_yg__blk1434_dn5, locals.var_sp_s_yg__blk1434_dn6, locals.var_sp_s_yg__blk1434_dn7, locals.var_sp_s_yg__blk1434_dn8,)
    }
};
        locals.var_sp_s_yg__blk1434 = assign49170_e63109;
        locals.var_sp_s_yg__blk1434_dn5 = assign49170_e63109_d_n5;
        locals.var_sp_s_yg__blk1434_dn6 = assign49170_e63109_d_n6;
        locals.var_sp_s_yg__blk1434_dn7 = assign49170_e63109_d_n7;
        locals.var_sp_s_yg__blk1434_dn8 = assign49170_e63109_d_n8;
        locals.var_sp_s_yg__blk1434_rv = 0.0;

        let (assign49180_e63124, assign49180_e63124_d_n5, assign49180_e63124_d_n6, assign49180_e63124_d_n7, assign49180_e63124_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49180_e63121: f64 = (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345);
        let assign49180_e63122: f64 = (1.25 * assign49180_e63121);
        (assign49180_e63122, (1.25 * ((locals.var_sp_s_yg__blk1434_dn5 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn5))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn6 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn6))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn7 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn7))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn8 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn8))),)
    } else {
        (locals.var_sp_s_ysub__blk1435, locals.var_sp_s_ysub__blk1435_dn5, locals.var_sp_s_ysub__blk1435_dn6, locals.var_sp_s_ysub__blk1435_dn7, locals.var_sp_s_ysub__blk1435_dn8,)
    }
};
        locals.var_sp_s_ysub__blk1435 = assign49180_e63124;
        locals.var_sp_s_ysub__blk1435_dn5 = assign49180_e63124_d_n5;
        locals.var_sp_s_ysub__blk1435_dn6 = assign49180_e63124_d_n6;
        locals.var_sp_s_ysub__blk1435_dn7 = assign49180_e63124_d_n7;
        locals.var_sp_s_ysub__blk1435_dn8 = assign49180_e63124_d_n8;
        locals.var_sp_s_ysub__blk1435_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign49190_e63150, assign49190_e63150_d_n5, assign49190_e63150_d_n6, assign49190_e63150_d_n7, assign49190_e63150_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49190_e63136: f64 = (locals.var_sp_s_ysub__blk1435 + 10.0);
        let assign49190_e63139: f64 = (locals.var_sp_s_ysub__blk1435 - 6.0);
        let assign49190_e63142: f64 = (locals.var_sp_s_ysub__blk1435 - 6.0);
        let assign49190_e63143: f64 = (assign49190_e63139 * assign49190_e63142);
        let assign49190_e63145: f64 = (assign49190_e63143 + 64.0);
        let assign49190_e63146: f64 = (assign49190_e63145).sqrt();
        let assign49190_e63147: f64 = (assign49190_e63136 - assign49190_e63146);
        let assign49190_e63148: f64 = (0.5 * assign49190_e63147);
        (assign49190_e63148, (0.5 * (locals.var_sp_s_ysub__blk1435_dn5 - (((locals.var_sp_s_ysub__blk1435_dn5 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn5)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn6 - (((locals.var_sp_s_ysub__blk1435_dn6 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn6)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn7 - (((locals.var_sp_s_ysub__blk1435_dn7 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn7)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn8 - (((locals.var_sp_s_ysub__blk1435_dn8 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn8)) / (2.0 * assign49190_e63146)))),)
    } else {
        (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8,)
    }
};
        locals.var_sp_s_eta__blk1436 = assign49190_e63150;
        locals.var_sp_s_eta__blk1436_dn5 = assign49190_e63150_d_n5;
        locals.var_sp_s_eta__blk1436_dn6 = assign49190_e63150_d_n6;
        locals.var_sp_s_eta__blk1436_dn7 = assign49190_e63150_d_n7;
        locals.var_sp_s_eta__blk1436_dn8 = assign49190_e63150_d_n8;
        locals.var_sp_s_eta__blk1436_rv = 0.0;

        let (assign49200_e63163, assign49200_e63163_d_n5, assign49200_e63163_d_n6, assign49200_e63163_d_n7, assign49200_e63163_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49200_e63161: f64 = (locals.var_sp_s_yg__blk1434 - locals.var_sp_s_eta__blk1436);
        (assign49200_e63161, (locals.var_sp_s_yg__blk1434_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_sp_s_yg__blk1434_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_sp_s_yg__blk1434_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_sp_s_yg__blk1434_dn8 - locals.var_sp_s_eta__blk1436_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49200_e63163;
        locals.var_sp_s_temp__blk1431_dn5 = assign49200_e63163_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49200_e63163_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49200_e63163_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49200_e63163_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49210_e63182, assign49210_e63182_d_n5, assign49210_e63182_d_n6, assign49210_e63182_d_n7, assign49210_e63182_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49210_e63174: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49210_e63178: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
        let assign49210_e63179: f64 = (locals.var_gf2__blk1308 * assign49210_e63178);
        let assign49210_e63180: f64 = (assign49210_e63174 + assign49210_e63179);
        (assign49210_e63180, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn5))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn6))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn7))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn8))),)
    } else {
        (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8,)
    }
};
        locals.var_sp_s_a__blk1437 = assign49210_e63182;
        locals.var_sp_s_a__blk1437_dn5 = assign49210_e63182_d_n5;
        locals.var_sp_s_a__blk1437_dn6 = assign49210_e63182_d_n6;
        locals.var_sp_s_a__blk1437_dn7 = assign49210_e63182_d_n7;
        locals.var_sp_s_a__blk1437_dn8 = assign49210_e63182_d_n8;
        locals.var_sp_s_a__blk1437_rv = 0.0;

        let (assign49220_e63197, assign49220_e63197_d_n5, assign49220_e63197_d_n6, assign49220_e63197_d_n7, assign49220_e63197_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49220_e63193: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign49220_e63195: f64 = (assign49220_e63193 - locals.var_gf2__blk1308);
        (assign49220_e63195, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) - locals.var_gf2__blk1308_dn5), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) - locals.var_gf2__blk1308_dn6), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) - locals.var_gf2__blk1308_dn7), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) - locals.var_gf2__blk1308_dn8),)
    } else {
        (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8,)
    }
};
        locals.var_sp_s_c__blk1438 = assign49220_e63197;
        locals.var_sp_s_c__blk1438_dn5 = assign49220_e63197_d_n5;
        locals.var_sp_s_c__blk1438_dn6 = assign49220_e63197_d_n6;
        locals.var_sp_s_c__blk1438_dn7 = assign49220_e63197_d_n7;
        locals.var_sp_s_c__blk1438_dn8 = assign49220_e63197_d_n8;
        locals.var_sp_s_c__blk1438_rv = 0.0;

        let (assign49230_e63214, assign49230_e63214_d_n5, assign49230_e63214_d_n6, assign49230_e63214_d_n7, assign49230_e63214_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49230_e63207: f64 = (-locals.var_sp_s_eta__blk1436);
        let assign49230_e63210: f64 = (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324);
        let assign49230_e63211: f64 = (assign49230_e63210).ln();
        let assign49230_e63212: f64 = (assign49230_e63207 + assign49230_e63211);
        (assign49230_e63212, ((-locals.var_sp_s_eta__blk1436_dn5) + (((locals.var_sp_s_a__blk1437_dn5 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn5)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn6) + (((locals.var_sp_s_a__blk1437_dn6 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn6)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn7) + (((locals.var_sp_s_a__blk1437_dn7 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn7)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn8) + (((locals.var_sp_s_a__blk1437_dn8 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn8)) / assign49230_e63210)),)
    } else {
        (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8,)
    }
};
        locals.var_sp_s_tau__blk1439 = assign49230_e63214;
        locals.var_sp_s_tau__blk1439_dn5 = assign49230_e63214_d_n5;
        locals.var_sp_s_tau__blk1439_dn6 = assign49230_e63214_d_n6;
        locals.var_sp_s_tau__blk1439_dn7 = assign49230_e63214_d_n7;
        locals.var_sp_s_tau__blk1439_dn8 = assign49230_e63214_d_n8;
        locals.var_sp_s_tau__blk1439_rv = 0.0;

        let (assign49240_e63227, assign49240_e63227_d_n5, assign49240_e63227_d_n6, assign49240_e63227_d_n7, assign49240_e63227_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49240_e63225: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
        (assign49240_e63225, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign49240_e63227;
        locals.var_nu_dn5 = assign49240_e63227_d_n5;
        locals.var_nu_dn6 = assign49240_e63227_d_n6;
        locals.var_nu_dn7 = assign49240_e63227_d_n7;
        locals.var_nu_dn8 = assign49240_e63227_d_n8;
        locals.var_nu_rv = 0.0;

        let (assign49250_e63250, assign49250_e63250_d_n5, assign49250_e63250_d_n6, assign49250_e63250_d_n7, assign49250_e63250_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49250_e63238: f64 = (locals.var_nu * locals.var_nu);
        let assign49250_e63243: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign49250_e63244: f64 = (0.5 * assign49250_e63243);
        let assign49250_e63246: f64 = (assign49250_e63244 - locals.var_sp_s_a__blk1437);
        let assign49250_e63247: f64 = (locals.var_sp_s_tau__blk1439 * assign49250_e63246);
        let assign49250_e63248: f64 = (assign49250_e63238 + assign49250_e63247);
        (assign49250_e63248, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - locals.var_sp_s_a__blk1437_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - locals.var_sp_s_a__blk1437_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - locals.var_sp_s_a__blk1437_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - locals.var_sp_s_a__blk1437_dn8)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign49250_e63250;
        locals.var_mutau_dn5 = assign49250_e63250_d_n5;
        locals.var_mutau_dn6 = assign49250_e63250_d_n6;
        locals.var_mutau_dn7 = assign49250_e63250_d_n7;
        locals.var_mutau_dn8 = assign49250_e63250_d_n8;
        locals.var_mutau_rv = 0.0;

        let (assign49260_e63287, assign49260_e63287_d_n5, assign49260_e63287_d_n6, assign49260_e63287_d_n7, assign49260_e63287_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49260_e63262: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
        let assign49260_e63264: f64 = (assign49260_e63262 * locals.var_sp_s_tau__blk1439);
        let assign49260_e63268: f64 = (locals.var_nu / locals.var_mutau);
        let assign49260_e63270: f64 = (assign49260_e63268 * locals.var_sp_s_tau__blk1439);
        let assign49260_e63272: f64 = (assign49260_e63270 * locals.var_sp_s_tau__blk1439);
        let assign49260_e63274: f64 = (assign49260_e63272 * locals.var_sp_s_c__blk1438);
        let assign49260_e63277: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign49260_e63279: f64 = (assign49260_e63277 * 0.3333333333333333);
        let assign49260_e63281: f64 = (assign49260_e63279 - locals.var_sp_s_a__blk1437);
        let assign49260_e63282: f64 = (assign49260_e63274 * assign49260_e63281);
        let assign49260_e63283: f64 = (locals.var_mutau + assign49260_e63282);
        let assign49260_e63284: f64 = (assign49260_e63264 / assign49260_e63283);
        let assign49260_e63285: f64 = (locals.var_sp_s_eta__blk1436 + assign49260_e63284);
        (assign49260_e63285, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn5)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn5)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn5)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn6)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn6)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn6)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn7)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn7)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn7)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn8)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn8)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn8)))))) / (assign49260_e63283 * assign49260_e63283))),)
    } else {
        (locals.var_sp_s_y0__blk1440, locals.var_sp_s_y0__blk1440_dn5, locals.var_sp_s_y0__blk1440_dn6, locals.var_sp_s_y0__blk1440_dn7, locals.var_sp_s_y0__blk1440_dn8,)
    }
};
        locals.var_sp_s_y0__blk1440 = assign49260_e63287;
        locals.var_sp_s_y0__blk1440_dn5 = assign49260_e63287_d_n5;
        locals.var_sp_s_y0__blk1440_dn6 = assign49260_e63287_d_n6;
        locals.var_sp_s_y0__blk1440_dn7 = assign49260_e63287_d_n7;
        locals.var_sp_s_y0__blk1440_dn8 = assign49260_e63287_d_n8;
        locals.var_sp_s_y0__blk1440_rv = 0.0;

        let assign49270_e63290: f64 = if locals.var_sp_s_y0__blk1440 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1470 = assign49270_e63290;
        locals.var_guard1470_rv = 0.0;

        let (assign49280_e63304, assign49280_e63304_d_n5, assign49280_e63304_d_n6, assign49280_e63304_d_n7, assign49280_e63304_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) {
        let assign49280_e63302: f64 = (locals.var_sp_s_y0__blk1440).exp();
        (assign49280_e63302, (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn5), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn6), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn7), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn8),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49280_e63304;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49280_e63304_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49280_e63304_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49280_e63304_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49280_e63304_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let (assign49290_e63340, assign49290_e63340_d_n5, assign49290_e63340_d_n6, assign49290_e63340_d_n7, assign49290_e63340_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 == 0.0)) {
        let assign49290_e63320: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
        let assign49290_e63325: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
        let assign49290_e63329: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
        let assign49290_e63331: f64 = (assign49290_e63329 * 0.3333333333333333);
        let assign49290_e63332: f64 = (1.0 + assign49290_e63331);
        let assign49290_e63333: f64 = (assign49290_e63325 * assign49290_e63332);
        let assign49290_e63334: f64 = (0.5 * assign49290_e63333);
        let assign49290_e63335: f64 = (1.0 + assign49290_e63334);
        let assign49290_e63336: f64 = (assign49290_e63320 * assign49290_e63335);
        let assign49290_e63337: f64 = (1.0 + assign49290_e63336);
        let assign49290_e63338: f64 = (1e100 * assign49290_e63337);
        (assign49290_e63338, (1e100 * ((locals.var_sp_s_y0__blk1440_dn5 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn5 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn6 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn6 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn7 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn7 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn8 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn8 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn8 * 0.3333333333333333))))))),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49290_e63340;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49290_e63340_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49290_e63340_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49290_e63340_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49290_e63340_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let (assign49300_e63353, assign49300_e63353_d_n5, assign49300_e63353_d_n6, assign49300_e63353_d_n7, assign49300_e63353_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49300_e63351: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
        (assign49300_e63351, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign49300_e63353;
        locals.var_sp_s_delta1__blk1442_dn5 = assign49300_e63353_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign49300_e63353_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign49300_e63353_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign49300_e63353_d_n8;
        locals.var_sp_s_delta1__blk1442_rv = 0.0;

        let (assign49310_e63370, assign49310_e63370_d_n5, assign49310_e63370_d_n6, assign49310_e63370_d_n7, assign49310_e63370_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49310_e63366: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440);
        let assign49310_e63367: f64 = (2.0 + assign49310_e63366);
        let assign49310_e63368: f64 = (1.0 / assign49310_e63367);
        (assign49310_e63368, (-(((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn5)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn6)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn7)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn8)) / (assign49310_e63367 * assign49310_e63367))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49310_e63370;
        locals.var_sp_s_temp__blk1431_dn5 = assign49310_e63370_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49310_e63370_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49310_e63370_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49310_e63370_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49320_e63385, assign49320_e63385_d_n5, assign49320_e63385_d_n6, assign49320_e63385_d_n7, assign49320_e63385_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49320_e63381: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440);
        let assign49320_e63383: f64 = (assign49320_e63381 * locals.var_sp_s_temp__blk1431);
        (assign49320_e63383, ((((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign49320_e63385;
        locals.var_sp_s_xi0__blk1443_dn5 = assign49320_e63385_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign49320_e63385_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign49320_e63385_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign49320_e63385_d_n8;
        locals.var_sp_s_xi0__blk1443_rv = 0.0;

        let (assign49330_e63402, assign49330_e63402_d_n5, assign49330_e63402_d_n6, assign49330_e63402_d_n7, assign49330_e63402_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49330_e63397: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431);
        let assign49330_e63399: f64 = (assign49330_e63397 * locals.var_sp_s_temp__blk1431);
        let assign49330_e63400: f64 = (4.0 * assign49330_e63399);
        (assign49330_e63400, (4.0 * ((((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign49330_e63402;
        locals.var_sp_s_xi1__blk1444_dn5 = assign49330_e63402_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign49330_e63402_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign49330_e63402_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign49330_e63402_d_n8;
        locals.var_sp_s_xi1__blk1444_rv = 0.0;

        let (assign49340_e63423, assign49340_e63423_d_n5, assign49340_e63423_d_n6, assign49340_e63423_d_n7, assign49340_e63423_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49340_e63413: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
        let assign49340_e63416: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign49340_e63417: f64 = (assign49340_e63413 - assign49340_e63416);
        let assign49340_e63419: f64 = (assign49340_e63417 * locals.var_sp_s_temp__blk1431);
        let assign49340_e63421: f64 = (assign49340_e63419 * locals.var_sp_s_temp__blk1431);
        (assign49340_e63421, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign49340_e63423;
        locals.var_sp_s_xi2__blk1445_dn5 = assign49340_e63423_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign49340_e63423_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign49340_e63423_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign49340_e63423_d_n8;
        locals.var_sp_s_xi2__blk1445_rv = 0.0;

        let (assign49350_e63436, assign49350_e63436_d_n5, assign49350_e63436_d_n6, assign49350_e63436_d_n7, assign49350_e63436_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49350_e63434: f64 = (locals.var_sp_s_yg__blk1434 - locals.var_sp_s_y0__blk1440);
        (assign49350_e63434, (locals.var_sp_s_yg__blk1434_dn5 - locals.var_sp_s_y0__blk1440_dn5), (locals.var_sp_s_yg__blk1434_dn6 - locals.var_sp_s_y0__blk1440_dn6), (locals.var_sp_s_yg__blk1434_dn7 - locals.var_sp_s_y0__blk1440_dn7), (locals.var_sp_s_yg__blk1434_dn8 - locals.var_sp_s_y0__blk1440_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49350_e63436;
        locals.var_sp_s_temp__blk1431_dn5 = assign49350_e63436_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49350_e63436_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49350_e63436_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49350_e63436_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49360_e63449, assign49360_e63449_d_n5, assign49360_e63449_d_n6, assign49360_e63449_d_n7, assign49360_e63449_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49360_e63447: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442);
        (assign49360_e63447, ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn8)),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign49360_e63449;
        locals.var_sp_s_temp1__blk1432_dn5 = assign49360_e63449_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign49360_e63449_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign49360_e63449_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign49360_e63449_d_n8;
        locals.var_sp_s_temp1__blk1432_rv = 0.0;

        let (assign49370_e63476, assign49370_e63476_d_n5, assign49370_e63476_d_n6, assign49370_e63476_d_n7, assign49370_e63476_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49370_e63460: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign49370_e63464: f64 = (locals.var_sp_s_delta0__blk1441 - 1.0);
        let assign49370_e63466: f64 = (assign49370_e63464 - locals.var_sp_s_temp1__blk1432);
        let assign49370_e63470: f64 = (1.0 - locals.var_sp_s_xi1__blk1444);
        let assign49370_e63471: f64 = (locals.var_delta_ns__blk1347 * assign49370_e63470);
        let assign49370_e63472: f64 = (assign49370_e63466 + assign49370_e63471);
        let assign49370_e63473: f64 = (locals.var_gf2__blk1308 * assign49370_e63472);
        let assign49370_e63474: f64 = (assign49370_e63460 + assign49370_e63473);
        (assign49370_e63474, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn5 - locals.var_sp_s_temp1__blk1432_dn5) + ((locals.var_delta_ns__blk1347_dn5 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn5))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn6 - locals.var_sp_s_temp1__blk1432_dn6) + ((locals.var_delta_ns__blk1347_dn6 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn6))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn7 - locals.var_sp_s_temp1__blk1432_dn7) + ((locals.var_delta_ns__blk1347_dn7 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn7))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn8 - locals.var_sp_s_temp1__blk1432_dn8) + ((locals.var_delta_ns__blk1347_dn8 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn8))))))),)
    } else {
        (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8,)
    }
};
        locals.var_sp_s_pc__blk1446 = assign49370_e63476;
        locals.var_sp_s_pc__blk1446_dn5 = assign49370_e63476_d_n5;
        locals.var_sp_s_pc__blk1446_dn6 = assign49370_e63476_d_n6;
        locals.var_sp_s_pc__blk1446_dn7 = assign49370_e63476_d_n7;
        locals.var_sp_s_pc__blk1446_dn8 = assign49370_e63476_d_n8;
        locals.var_sp_s_pc__blk1446_rv = 0.0;

        let (assign49380_e63507, assign49380_e63507_d_n5, assign49380_e63507_d_n6, assign49380_e63507_d_n7, assign49380_e63507_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49380_e63487: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49380_e63491: f64 = (locals.var_sp_s_delta0__blk1441 - locals.var_sp_s_y0__blk1440);
        let assign49380_e63493: f64 = (assign49380_e63491 - 1.0);
        let assign49380_e63495: f64 = (assign49380_e63493 + locals.var_sp_s_temp1__blk1432);
        let assign49380_e63499: f64 = (locals.var_sp_s_y0__blk1440 - 1.0);
        let assign49380_e63501: f64 = (assign49380_e63499 - locals.var_sp_s_xi0__blk1443);
        let assign49380_e63502: f64 = (locals.var_delta_ns__blk1347 * assign49380_e63501);
        let assign49380_e63503: f64 = (assign49380_e63495 + assign49380_e63502);
        let assign49380_e63504: f64 = (locals.var_gf2__blk1308 * assign49380_e63503);
        let assign49380_e63505: f64 = (assign49380_e63487 - assign49380_e63504);
        (assign49380_e63505, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn5 - locals.var_sp_s_y0__blk1440_dn5) + locals.var_sp_s_temp1__blk1432_dn5) + ((locals.var_delta_ns__blk1347_dn5 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn5 - locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn6 - locals.var_sp_s_y0__blk1440_dn6) + locals.var_sp_s_temp1__blk1432_dn6) + ((locals.var_delta_ns__blk1347_dn6 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn6 - locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn7 - locals.var_sp_s_y0__blk1440_dn7) + locals.var_sp_s_temp1__blk1432_dn7) + ((locals.var_delta_ns__blk1347_dn7 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn7 - locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn8 - locals.var_sp_s_y0__blk1440_dn8) + locals.var_sp_s_temp1__blk1432_dn8) + ((locals.var_delta_ns__blk1347_dn8 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn8 - locals.var_sp_s_xi0__blk1443_dn8))))))),)
    } else {
        (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8,)
    }
};
        locals.var_sp_s_qc__blk1447 = assign49380_e63507;
        locals.var_sp_s_qc__blk1447_dn5 = assign49380_e63507_d_n5;
        locals.var_sp_s_qc__blk1447_dn6 = assign49380_e63507_d_n6;
        locals.var_sp_s_qc__blk1447_dn7 = assign49380_e63507_d_n7;
        locals.var_sp_s_qc__blk1447_dn8 = assign49380_e63507_d_n8;
        locals.var_sp_s_qc__blk1447_rv = 0.0;

        let (assign49390_e63528, assign49390_e63528_d_n5, assign49390_e63528_d_n6, assign49390_e63528_d_n7, assign49390_e63528_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49390_e63520: f64 = (locals.var_sp_s_delta0__blk1441 + locals.var_sp_s_temp1__blk1432);
        let assign49390_e63523: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
        let assign49390_e63524: f64 = (assign49390_e63520 - assign49390_e63523);
        let assign49390_e63525: f64 = (locals.var_gf2__blk1308 * assign49390_e63524);
        let assign49390_e63526: f64 = (2.0 - assign49390_e63525);
        (assign49390_e63526, (-((locals.var_gf2__blk1308_dn5 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn5 + locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn6 + locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn7 + locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn8 + locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8)))))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49390_e63528;
        locals.var_sp_s_temp__blk1431_dn5 = assign49390_e63528_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49390_e63528_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49390_e63528_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49390_e63528_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49400_e63547, assign49400_e63547_d_n5, assign49400_e63547_d_n6, assign49400_e63547_d_n7, assign49400_e63547_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49400_e63539: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
        let assign49400_e63543: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
        let assign49400_e63544: f64 = (2.0 * assign49400_e63543);
        let assign49400_e63545: f64 = (assign49400_e63539 - assign49400_e63544);
        (assign49400_e63545, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49400_e63547;
        locals.var_sp_s_temp__blk1431_dn5 = assign49400_e63547_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49400_e63547_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49400_e63547_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49400_e63547_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49410_e63568, assign49410_e63568_d_n5, assign49410_e63568_d_n6, assign49410_e63568_d_n7, assign49410_e63568_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49410_e63557: f64 = (-locals.var_sp_s_y0__blk1440);
        let assign49410_e63562: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
        let assign49410_e63563: f64 = (locals.var_sp_s_pc__blk1446 + assign49410_e63562);
        let assign49410_e63564: f64 = (locals.var_sp_s_qc__blk1447 / assign49410_e63563);
        let assign49410_e63565: f64 = (2.0 * assign49410_e63564);
        let assign49410_e63566: f64 = (assign49410_e63557 - assign49410_e63565);
        (assign49410_e63566, ((-locals.var_sp_s_y0__blk1440_dn5) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn6) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn7) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn8) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))),)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign49410_e63568;
        locals.var_x_s__blk1346_dn5 = assign49410_e63568_d_n5;
        locals.var_x_s__blk1346_dn6 = assign49410_e63568_d_n6;
        locals.var_x_s__blk1346_dn7 = assign49410_e63568_d_n7;
        locals.var_x_s__blk1346_dn8 = assign49410_e63568_d_n8;
        locals.var_x_s__blk1346_rv = 0.0;

        let (assign49420_e63586, assign49420_e63586_d_n5, assign49420_e63586_d_n6, assign49420_e63586_d_n7, assign49420_e63586_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49420_e63582: f64 = (locals.var_gf__blk1307 * 0.7324648775608221);
        let assign49420_e63583: f64 = (1.25 + assign49420_e63582);
        let assign49420_e63584: f64 = (1.0 / assign49420_e63583);
        (assign49420_e63584, (-((locals.var_gf__blk1307_dn5 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn6 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn7 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn8 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))),)
    } else {
        (locals.var_sp_xg1__blk1448, locals.var_sp_xg1__blk1448_dn5, locals.var_sp_xg1__blk1448_dn6, locals.var_sp_xg1__blk1448_dn7, locals.var_sp_xg1__blk1448_dn8,)
    }
};
        locals.var_sp_xg1__blk1448 = assign49420_e63586;
        locals.var_sp_xg1__blk1448_dn5 = assign49420_e63586_d_n5;
        locals.var_sp_xg1__blk1448_dn6 = assign49420_e63586_d_n6;
        locals.var_sp_xg1__blk1448_dn7 = assign49420_e63586_d_n7;
        locals.var_sp_xg1__blk1448_dn8 = assign49420_e63586_d_n8;
        locals.var_sp_xg1__blk1448_rv = 0.0;

        let (assign49430_e63606, assign49430_e63606_d_n5, assign49430_e63606_d_n6, assign49430_e63606_d_n7, assign49430_e63606_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49430_e63598: f64 = (locals.var_xi__blk1343 * 1.25);
        let assign49430_e63600: f64 = (assign49430_e63598 * locals.var_sp_xg1__blk1448);
        let assign49430_e63602: f64 = (assign49430_e63600 - 1.0);
        let assign49430_e63604: f64 = (assign49430_e63602 * locals.var_sp_xg1__blk1448);
        (assign49430_e63604, (((((locals.var_xi__blk1343_dn5 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn5)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn5)), (((((locals.var_xi__blk1343_dn6 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn6)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn6)), (((((locals.var_xi__blk1343_dn7 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn7)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn7)), (((((locals.var_xi__blk1343_dn8 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn8)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn8)),)
    } else {
        (locals.var_sp_s_a_fac__blk1449, locals.var_sp_s_a_fac__blk1449_dn5, locals.var_sp_s_a_fac__blk1449_dn6, locals.var_sp_s_a_fac__blk1449_dn7, locals.var_sp_s_a_fac__blk1449_dn8,)
    }
};
        locals.var_sp_s_a_fac__blk1449 = assign49430_e63606;
        locals.var_sp_s_a_fac__blk1449_dn5 = assign49430_e63606_d_n5;
        locals.var_sp_s_a_fac__blk1449_dn6 = assign49430_e63606_d_n6;
        locals.var_sp_s_a_fac__blk1449_dn7 = assign49430_e63606_d_n7;
        locals.var_sp_s_a_fac__blk1449_dn8 = assign49430_e63606_d_n8;
        locals.var_sp_s_a_fac__blk1449_rv = 0.0;

        let (assign49440_e63626, assign49440_e63626_d_n5, assign49440_e63626_d_n6, assign49440_e63626_d_n7, assign49440_e63626_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49440_e63618: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
        let assign49440_e63622: f64 = (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326);
        let assign49440_e63623: f64 = (1.0 + assign49440_e63622);
        let assign49440_e63624: f64 = (assign49440_e63618 * assign49440_e63623);
        (assign49440_e63624, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn5 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn6 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn7 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn8 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn8)))),)
    } else {
        (locals.var_sp_s_xbar__blk1450, locals.var_sp_s_xbar__blk1450_dn5, locals.var_sp_s_xbar__blk1450_dn6, locals.var_sp_s_xbar__blk1450_dn7, locals.var_sp_s_xbar__blk1450_dn8,)
    }
};
        locals.var_sp_s_xbar__blk1450 = assign49440_e63626;
        locals.var_sp_s_xbar__blk1450_dn5 = assign49440_e63626_d_n5;
        locals.var_sp_s_xbar__blk1450_dn6 = assign49440_e63626_d_n6;
        locals.var_sp_s_xbar__blk1450_dn7 = assign49440_e63626_d_n7;
        locals.var_sp_s_xbar__blk1450_dn8 = assign49440_e63626_d_n8;
        locals.var_sp_s_xbar__blk1450_rv = 0.0;

        let assign49450_e63628: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49450_e63630: f64 = (-230.25850929940458);
        let assign49450_e63631: f64 = if assign49450_e63628 > assign49450_e63630 { 1.0 } else { 0.0 };
        locals.var_guard1471 = assign49450_e63631;
        locals.var_guard1471_rv = 0.0;

        let (assign49460_e63647, assign49460_e63647_d_n5, assign49460_e63647_d_n6, assign49460_e63647_d_n7, assign49460_e63647_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign49460_e63644: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49460_e63645: f64 = (assign49460_e63644).exp();
        (assign49460_e63645, (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn5)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn6)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn7)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn8)),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49460_e63647;
        locals.var_sp_s_temp__blk1431_dn5 = assign49460_e63647_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49460_e63647_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49460_e63647_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49460_e63647_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_39(
        locals: &mut StampLocals,
    ) {
        let (assign49470_e63690, assign49470_e63690_d_n5, assign49470_e63690_d_n6, assign49470_e63690_d_n7, assign49470_e63690_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1471 == 0.0)) {
        let assign49470_e63663: f64 = (-230.25850929940458);
        let assign49470_e63665: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49470_e63666: f64 = (assign49470_e63663 - assign49470_e63665);
        let assign49470_e63670: f64 = (-230.25850929940458);
        let assign49470_e63672: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49470_e63673: f64 = (assign49470_e63670 - assign49470_e63672);
        let assign49470_e63676: f64 = (-230.25850929940458);
        let assign49470_e63678: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49470_e63679: f64 = (assign49470_e63676 - assign49470_e63678);
        let assign49470_e63681: f64 = (assign49470_e63679 * 0.3333333333333333);
        let assign49470_e63682: f64 = (1.0 + assign49470_e63681);
        let assign49470_e63683: f64 = (assign49470_e63673 * assign49470_e63682);
        let assign49470_e63684: f64 = (0.5 * assign49470_e63683);
        let assign49470_e63685: f64 = (1.0 + assign49470_e63684);
        let assign49470_e63686: f64 = (assign49470_e63666 * assign49470_e63685);
        let assign49470_e63687: f64 = (1.0 + assign49470_e63686);
        let assign49470_e63688: f64 = (1e-100 / assign49470_e63687);
        (assign49470_e63688, (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn5)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn5)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn5)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn6)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn6)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn6)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn7)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn7)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn7)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn8)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn8)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn8)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49470_e63690;
        locals.var_sp_s_temp__blk1431_dn5 = assign49470_e63690_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49470_e63690_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49470_e63690_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49470_e63690_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49480_e63704, assign49480_e63704_d_n5, assign49480_e63704_d_n6, assign49480_e63704_d_n7, assign49480_e63704_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49480_e63702: f64 = (1.0 - locals.var_sp_s_temp__blk1431);
        (assign49480_e63702, (-locals.var_sp_s_temp__blk1431_dn5), (-locals.var_sp_s_temp__blk1431_dn6), (-locals.var_sp_s_temp__blk1431_dn7), (-locals.var_sp_s_temp__blk1431_dn8),)
    } else {
        (locals.var_sp_s_w__blk1451, locals.var_sp_s_w__blk1451_dn5, locals.var_sp_s_w__blk1451_dn6, locals.var_sp_s_w__blk1451_dn7, locals.var_sp_s_w__blk1451_dn8,)
    }
};
        locals.var_sp_s_w__blk1451 = assign49480_e63704;
        locals.var_sp_s_w__blk1451_dn5 = assign49480_e63704_d_n5;
        locals.var_sp_s_w__blk1451_dn6 = assign49480_e63704_d_n6;
        locals.var_sp_s_w__blk1451_dn7 = assign49480_e63704_d_n7;
        locals.var_sp_s_w__blk1451_dn8 = assign49480_e63704_d_n8;
        locals.var_sp_s_w__blk1451_rv = 0.0;

        let (assign49490_e63731, assign49490_e63731_d_n5, assign49490_e63731_d_n6, assign49490_e63731_d_n7, assign49490_e63731_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49490_e63717: f64 = (locals.var_gf2__blk1308 * 0.5);
        let assign49490_e63718: f64 = (locals.var_xg__blk1326 + assign49490_e63717);
        let assign49490_e63723: f64 = (locals.var_gf2__blk1308 * 0.25);
        let assign49490_e63724: f64 = (locals.var_xg__blk1326 + assign49490_e63723);
        let assign49490_e63726: f64 = (assign49490_e63724 - locals.var_sp_s_w__blk1451);
        let assign49490_e63727: f64 = (assign49490_e63726).sqrt();
        let assign49490_e63728: f64 = (locals.var_gf__blk1307 * assign49490_e63727);
        let assign49490_e63729: f64 = (assign49490_e63718 - assign49490_e63728);
        (assign49490_e63729, ((locals.var_xg__blk1326_dn5 + (locals.var_gf2__blk1308_dn5 * 0.5)) - ((locals.var_gf__blk1307_dn5 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn5 + (locals.var_gf2__blk1308_dn5 * 0.25)) - locals.var_sp_s_w__blk1451_dn5) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn6 + (locals.var_gf2__blk1308_dn6 * 0.5)) - ((locals.var_gf__blk1307_dn6 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn6 + (locals.var_gf2__blk1308_dn6 * 0.25)) - locals.var_sp_s_w__blk1451_dn6) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn7 + (locals.var_gf2__blk1308_dn7 * 0.5)) - ((locals.var_gf__blk1307_dn7 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn7 + (locals.var_gf2__blk1308_dn7 * 0.25)) - locals.var_sp_s_w__blk1451_dn7) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn8 + (locals.var_gf2__blk1308_dn8 * 0.5)) - ((locals.var_gf__blk1307_dn8 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn8 + (locals.var_gf2__blk1308_dn8 * 0.25)) - locals.var_sp_s_w__blk1451_dn8) / (2.0 * assign49490_e63727))))),)
    } else {
        (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8,)
    }
};
        locals.var_sp_s_x1__blk1452 = assign49490_e63731;
        locals.var_sp_s_x1__blk1452_dn5 = assign49490_e63731_d_n5;
        locals.var_sp_s_x1__blk1452_dn6 = assign49490_e63731_d_n6;
        locals.var_sp_s_x1__blk1452_dn7 = assign49490_e63731_d_n7;
        locals.var_sp_s_x1__blk1452_dn8 = assign49490_e63731_d_n8;
        locals.var_sp_s_x1__blk1452_rv = 0.0;

        let (assign49500_e63745, assign49500_e63745_d_n5, assign49500_e63745_d_n6, assign49500_e63745_d_n7, assign49500_e63745_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49500_e63743: f64 = (locals.var_xn_s__blk1332 + 3.0);
        (assign49500_e63743, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8,)
    } else {
        (locals.var_sp_s_bx__blk1453, locals.var_sp_s_bx__blk1453_dn5, locals.var_sp_s_bx__blk1453_dn6, locals.var_sp_s_bx__blk1453_dn7, locals.var_sp_s_bx__blk1453_dn8,)
    }
};
        locals.var_sp_s_bx__blk1453 = assign49500_e63745;
        locals.var_sp_s_bx__blk1453_dn5 = assign49500_e63745_d_n5;
        locals.var_sp_s_bx__blk1453_dn6 = assign49500_e63745_d_n6;
        locals.var_sp_s_bx__blk1453_dn7 = assign49500_e63745_d_n7;
        locals.var_sp_s_bx__blk1453_dn8 = assign49500_e63745_d_n8;
        locals.var_sp_s_bx__blk1453_rv = 0.0;

        let (assign49510_e63783, assign49510_e63783_d_n5, assign49510_e63783_d_n6, assign49510_e63783_d_n7, assign49510_e63783_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49510_e63758: f64 = (locals.var_sp_s_x1__blk1452 + locals.var_sp_s_bx__blk1453);
        let assign49510_e63761: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
        let assign49510_e63764: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
        let assign49510_e63765: f64 = (assign49510_e63761 * assign49510_e63764);
        let assign49510_e63767: f64 = (assign49510_e63765 + 5.0);
        let assign49510_e63768: f64 = (assign49510_e63767).sqrt();
        let assign49510_e63769: f64 = (assign49510_e63758 - assign49510_e63768);
        let assign49510_e63770: f64 = (0.5 * assign49510_e63769);
        let assign49510_e63775: f64 = (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453);
        let assign49510_e63777: f64 = (assign49510_e63775 + 5.0);
        let assign49510_e63778: f64 = (assign49510_e63777).sqrt();
        let assign49510_e63779: f64 = (locals.var_sp_s_bx__blk1453 - assign49510_e63778);
        let assign49510_e63780: f64 = (0.5 * assign49510_e63779);
        let assign49510_e63781: f64 = (assign49510_e63770 - assign49510_e63780);
        (assign49510_e63781, ((0.5 * ((locals.var_sp_s_x1__blk1452_dn5 + locals.var_sp_s_bx__blk1453_dn5) - ((((locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn5 - (((locals.var_sp_s_bx__blk1453_dn5 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn5)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn6 + locals.var_sp_s_bx__blk1453_dn6) - ((((locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn6 - (((locals.var_sp_s_bx__blk1453_dn6 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn6)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn7 + locals.var_sp_s_bx__blk1453_dn7) - ((((locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn7 - (((locals.var_sp_s_bx__blk1453_dn7 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn7)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn8 + locals.var_sp_s_bx__blk1453_dn8) - ((((locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn8 - (((locals.var_sp_s_bx__blk1453_dn8 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn8)) / (2.0 * assign49510_e63778))))),)
    } else {
        (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8,)
    }
};
        locals.var_sp_s_eta__blk1436 = assign49510_e63783;
        locals.var_sp_s_eta__blk1436_dn5 = assign49510_e63783_d_n5;
        locals.var_sp_s_eta__blk1436_dn6 = assign49510_e63783_d_n6;
        locals.var_sp_s_eta__blk1436_dn7 = assign49510_e63783_d_n7;
        locals.var_sp_s_eta__blk1436_dn8 = assign49510_e63783_d_n8;
        locals.var_sp_s_eta__blk1436_rv = 0.0;

        let (assign49520_e63797, assign49520_e63797_d_n5, assign49520_e63797_d_n6, assign49520_e63797_d_n7, assign49520_e63797_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49520_e63795: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_eta__blk1436);
        (assign49520_e63795, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_eta__blk1436_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49520_e63797;
        locals.var_sp_s_temp__blk1431_dn5 = assign49520_e63797_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49520_e63797_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49520_e63797_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49520_e63797_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49530_e63811, assign49530_e63811_d_n5, assign49530_e63811_d_n6, assign49530_e63811_d_n7, assign49530_e63811_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49530_e63808: f64 = (-locals.var_sp_s_eta__blk1436);
        let assign49530_e63809: f64 = (assign49530_e63808).exp();
        (assign49530_e63809, (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn5)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn6)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn7)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn8)),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign49530_e63811;
        locals.var_sp_s_temp1__blk1432_dn5 = assign49530_e63811_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign49530_e63811_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign49530_e63811_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign49530_e63811_d_n8;
        locals.var_sp_s_temp1__blk1432_rv = 0.0;

        let (assign49540_e63829, assign49540_e63829_d_n5, assign49540_e63829_d_n6, assign49540_e63829_d_n7, assign49540_e63829_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49540_e63825: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
        let assign49540_e63826: f64 = (2.0 + assign49540_e63825);
        let assign49540_e63827: f64 = (1.0 / assign49540_e63826);
        (assign49540_e63827, (-(((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) / (assign49540_e63826 * assign49540_e63826))),)
    } else {
        (locals.var_sp_s_temp2__blk1433, locals.var_sp_s_temp2__blk1433_dn5, locals.var_sp_s_temp2__blk1433_dn6, locals.var_sp_s_temp2__blk1433_dn7, locals.var_sp_s_temp2__blk1433_dn8,)
    }
};
        locals.var_sp_s_temp2__blk1433 = assign49540_e63829;
        locals.var_sp_s_temp2__blk1433_dn5 = assign49540_e63829_d_n5;
        locals.var_sp_s_temp2__blk1433_dn6 = assign49540_e63829_d_n6;
        locals.var_sp_s_temp2__blk1433_dn7 = assign49540_e63829_d_n7;
        locals.var_sp_s_temp2__blk1433_dn8 = assign49540_e63829_d_n8;
        locals.var_sp_s_temp2__blk1433_rv = 0.0;

        let (assign49550_e63845, assign49550_e63845_d_n5, assign49550_e63845_d_n6, assign49550_e63845_d_n7, assign49550_e63845_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49550_e63841: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
        let assign49550_e63843: f64 = (assign49550_e63841 * locals.var_sp_s_temp2__blk1433);
        (assign49550_e63843, ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn5)), ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn6)), ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn7)), ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign49550_e63845;
        locals.var_sp_s_xi0__blk1443_dn5 = assign49550_e63845_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign49550_e63845_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign49550_e63845_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign49550_e63845_d_n8;
        locals.var_sp_s_xi0__blk1443_rv = 0.0;

        let (assign49560_e63863, assign49560_e63863_d_n5, assign49560_e63863_d_n6, assign49560_e63863_d_n7, assign49560_e63863_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49560_e63858: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433);
        let assign49560_e63860: f64 = (assign49560_e63858 * locals.var_sp_s_temp2__blk1433);
        let assign49560_e63861: f64 = (4.0 * assign49560_e63860);
        (assign49560_e63861, (4.0 * ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn5))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign49560_e63863;
        locals.var_sp_s_xi1__blk1444_dn5 = assign49560_e63863_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign49560_e63863_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign49560_e63863_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign49560_e63863_d_n8;
        locals.var_sp_s_xi1__blk1444_rv = 0.0;

        let (assign49570_e63885, assign49570_e63885_d_n5, assign49570_e63885_d_n6, assign49570_e63885_d_n7, assign49570_e63885_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49570_e63875: f64 = (8.0 * locals.var_sp_s_temp2__blk1433);
        let assign49570_e63878: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign49570_e63879: f64 = (assign49570_e63875 - assign49570_e63878);
        let assign49570_e63881: f64 = (assign49570_e63879 * locals.var_sp_s_temp2__blk1433);
        let assign49570_e63883: f64 = (assign49570_e63881 * locals.var_sp_s_temp2__blk1433);
        (assign49570_e63883, ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn5)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign49570_e63885;
        locals.var_sp_s_xi2__blk1445_dn5 = assign49570_e63885_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign49570_e63885_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign49570_e63885_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign49570_e63885_d_n8;
        locals.var_sp_s_xi2__blk1445_rv = 0.0;

        let (assign49580_e63938, assign49580_e63938_d_n5, assign49580_e63938_d_n6, assign49580_e63938_d_n7, assign49580_e63938_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49580_e63898: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49580_e63902: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
        let assign49580_e63904: f64 = (assign49580_e63902 - 1.0);
        let assign49580_e63908: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
        let assign49580_e63910: f64 = (assign49580_e63908 + locals.var_sp_s_xi0__blk1443);
        let assign49580_e63911: f64 = (locals.var_delta_ns__blk1347 * assign49580_e63910);
        let assign49580_e63912: f64 = (assign49580_e63904 - assign49580_e63911);
        let assign49580_e63913: f64 = (locals.var_gf2__blk1308 * assign49580_e63912);
        let assign49580_e63914: f64 = (assign49580_e63898 - assign49580_e63913);
        let (assign49580_e63936, assign49580_e63936_d_n5, assign49580_e63936_d_n6, assign49580_e63936_d_n7, assign49580_e63936_d_n8,) = {
            if (1e-40 > assign49580_e63914) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign49580_e63919: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
                let assign49580_e63923: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
                let assign49580_e63925: f64 = (assign49580_e63923 - 1.0);
                let assign49580_e63929: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
                let assign49580_e63931: f64 = (assign49580_e63929 + locals.var_sp_s_xi0__blk1443);
                let assign49580_e63932: f64 = (locals.var_delta_ns__blk1347 * assign49580_e63931);
                let assign49580_e63933: f64 = (assign49580_e63925 - assign49580_e63932);
                let assign49580_e63934: f64 = (locals.var_gf2__blk1308 * assign49580_e63933);
                let assign49580_e63935: f64 = (assign49580_e63919 - assign49580_e63934);
                (assign49580_e63935, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn5 + locals.var_sp_s_eta__blk1436_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn6 + locals.var_sp_s_eta__blk1436_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn7 + locals.var_sp_s_eta__blk1436_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn8 + locals.var_sp_s_eta__blk1436_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
            }
        };
        (assign49580_e63936, assign49580_e63936_d_n5, assign49580_e63936_d_n6, assign49580_e63936_d_n7, assign49580_e63936_d_n8,)
    } else {
        (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8,)
    }
};
        locals.var_sp_s_a__blk1437 = assign49580_e63938;
        locals.var_sp_s_a__blk1437_dn5 = assign49580_e63938_d_n5;
        locals.var_sp_s_a__blk1437_dn6 = assign49580_e63938_d_n6;
        locals.var_sp_s_a__blk1437_dn7 = assign49580_e63938_d_n7;
        locals.var_sp_s_a__blk1437_dn8 = assign49580_e63938_d_n8;
        locals.var_sp_s_a__blk1437_rv = 0.0;

        let (assign49590_e63960, assign49590_e63960_d_n5, assign49590_e63960_d_n6, assign49590_e63960_d_n7, assign49590_e63960_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49590_e63954: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
        let assign49590_e63955: f64 = (locals.var_sp_s_temp1__blk1432 - assign49590_e63954);
        let assign49590_e63956: f64 = (locals.var_gf2__blk1308 * assign49590_e63955);
        let assign49590_e63957: f64 = (0.5 * assign49590_e63956);
        let assign49590_e63958: f64 = (1.0 - assign49590_e63957);
        (assign49590_e63958, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn5 - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn6 - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn7 - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn8 - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8))))))),)
    } else {
        (locals.var_sp_s_b__blk1454, locals.var_sp_s_b__blk1454_dn5, locals.var_sp_s_b__blk1454_dn6, locals.var_sp_s_b__blk1454_dn7, locals.var_sp_s_b__blk1454_dn8,)
    }
};
        locals.var_sp_s_b__blk1454 = assign49590_e63960;
        locals.var_sp_s_b__blk1454_dn5 = assign49590_e63960_d_n5;
        locals.var_sp_s_b__blk1454_dn6 = assign49590_e63960_d_n6;
        locals.var_sp_s_b__blk1454_dn7 = assign49590_e63960_d_n7;
        locals.var_sp_s_b__blk1454_dn8 = assign49590_e63960_d_n8;
        locals.var_sp_s_b__blk1454_rv = 0.0;

        let (assign49600_e63986, assign49600_e63986_d_n5, assign49600_e63986_d_n6, assign49600_e63986_d_n7, assign49600_e63986_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49600_e63972: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign49600_e63976: f64 = (1.0 - locals.var_sp_s_temp1__blk1432);
        let assign49600_e63980: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
        let assign49600_e63981: f64 = (locals.var_delta_ns__blk1347 * assign49600_e63980);
        let assign49600_e63982: f64 = (assign49600_e63976 - assign49600_e63981);
        let assign49600_e63983: f64 = (locals.var_gf2__blk1308 * assign49600_e63982);
        let assign49600_e63984: f64 = (assign49600_e63972 + assign49600_e63983);
        (assign49600_e63984, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn8)))))),)
    } else {
        (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8,)
    }
};
        locals.var_sp_s_c__blk1438 = assign49600_e63986;
        locals.var_sp_s_c__blk1438_dn5 = assign49600_e63986_d_n5;
        locals.var_sp_s_c__blk1438_dn6 = assign49600_e63986_d_n6;
        locals.var_sp_s_c__blk1438_dn7 = assign49600_e63986_d_n7;
        locals.var_sp_s_c__blk1438_dn8 = assign49600_e63986_d_n8;
        locals.var_sp_s_c__blk1438_rv = 0.0;

        let (assign49610_e64005, assign49610_e64005_d_n5, assign49610_e64005_d_n6, assign49610_e64005_d_n7, assign49610_e64005_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49610_e63998: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_eta__blk1436);
        let assign49610_e64001: f64 = (locals.var_sp_s_a__blk1437 / locals.var_gf2__blk1308);
        let assign49610_e64002: f64 = (assign49610_e64001).ln();
        let assign49610_e64003: f64 = (assign49610_e63998 + assign49610_e64002);
        (assign49610_e64003, ((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_eta__blk1436_dn5) + ((((locals.var_sp_s_a__blk1437_dn5 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn5)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_eta__blk1436_dn6) + ((((locals.var_sp_s_a__blk1437_dn6 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn6)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_eta__blk1436_dn7) + ((((locals.var_sp_s_a__blk1437_dn7 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn7)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_eta__blk1436_dn8) + ((((locals.var_sp_s_a__blk1437_dn8 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn8)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)),)
    } else {
        (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8,)
    }
};
        locals.var_sp_s_tau__blk1439 = assign49610_e64005;
        locals.var_sp_s_tau__blk1439_dn5 = assign49610_e64005_d_n5;
        locals.var_sp_s_tau__blk1439_dn6 = assign49610_e64005_d_n6;
        locals.var_sp_s_tau__blk1439_dn7 = assign49610_e64005_d_n7;
        locals.var_sp_s_tau__blk1439_dn8 = assign49610_e64005_d_n8;
        locals.var_sp_s_tau__blk1439_rv = 0.0;

        let (assign49620_e64019, assign49620_e64019_d_n5, assign49620_e64019_d_n6, assign49620_e64019_d_n7, assign49620_e64019_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49620_e64017: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
        (assign49620_e64017, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign49620_e64019;
        locals.var_nu_dn5 = assign49620_e64019_d_n5;
        locals.var_nu_dn6 = assign49620_e64019_d_n6;
        locals.var_nu_dn7 = assign49620_e64019_d_n7;
        locals.var_nu_dn8 = assign49620_e64019_d_n8;
        locals.var_nu_rv = 0.0;

        let (assign49630_e64045, assign49630_e64045_d_n5, assign49630_e64045_d_n6, assign49630_e64045_d_n7, assign49630_e64045_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49630_e64031: f64 = (locals.var_nu * locals.var_nu);
        let assign49630_e64036: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign49630_e64037: f64 = (0.5 * assign49630_e64036);
        let assign49630_e64040: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
        let assign49630_e64041: f64 = (assign49630_e64037 - assign49630_e64040);
        let assign49630_e64042: f64 = (locals.var_sp_s_tau__blk1439 * assign49630_e64041);
        let assign49630_e64043: f64 = (assign49630_e64031 + assign49630_e64042);
        (assign49630_e64043, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign49630_e64045;
        locals.var_mutau_dn5 = assign49630_e64045_d_n5;
        locals.var_mutau_dn6 = assign49630_e64045_d_n6;
        locals.var_mutau_dn7 = assign49630_e64045_d_n7;
        locals.var_mutau_dn8 = assign49630_e64045_d_n8;
        locals.var_mutau_rv = 0.0;

        let (assign49640_e64085, assign49640_e64085_d_n5, assign49640_e64085_d_n6, assign49640_e64085_d_n7, assign49640_e64085_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49640_e64058: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
        let assign49640_e64060: f64 = (assign49640_e64058 * locals.var_sp_s_tau__blk1439);
        let assign49640_e64064: f64 = (locals.var_nu / locals.var_mutau);
        let assign49640_e64066: f64 = (assign49640_e64064 * locals.var_sp_s_tau__blk1439);
        let assign49640_e64068: f64 = (assign49640_e64066 * locals.var_sp_s_tau__blk1439);
        let assign49640_e64070: f64 = (assign49640_e64068 * locals.var_sp_s_c__blk1438);
        let assign49640_e64073: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign49640_e64075: f64 = (assign49640_e64073 * 0.3333333333333333);
        let assign49640_e64078: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
        let assign49640_e64079: f64 = (assign49640_e64075 - assign49640_e64078);
        let assign49640_e64080: f64 = (assign49640_e64070 * assign49640_e64079);
        let assign49640_e64081: f64 = (locals.var_mutau + assign49640_e64080);
        let assign49640_e64082: f64 = (assign49640_e64060 / assign49640_e64081);
        let assign49640_e64083: f64 = (locals.var_sp_s_eta__blk1436 + assign49640_e64082);
        (assign49640_e64083, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn5)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn5)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn6)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn6)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn7)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn7)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn8)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn8)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))))) / (assign49640_e64081 * assign49640_e64081))),)
    } else {
        (locals.var_sp_s_x0__blk1455, locals.var_sp_s_x0__blk1455_dn5, locals.var_sp_s_x0__blk1455_dn6, locals.var_sp_s_x0__blk1455_dn7, locals.var_sp_s_x0__blk1455_dn8,)
    }
};
        locals.var_sp_s_x0__blk1455 = assign49640_e64085;
        locals.var_sp_s_x0__blk1455_dn5 = assign49640_e64085_d_n5;
        locals.var_sp_s_x0__blk1455_dn6 = assign49640_e64085_d_n6;
        locals.var_sp_s_x0__blk1455_dn7 = assign49640_e64085_d_n7;
        locals.var_sp_s_x0__blk1455_dn8 = assign49640_e64085_d_n8;
        locals.var_sp_s_x0__blk1455_rv = 0.0;

        let assign49650_e64088: f64 = if locals.var_sp_s_x0__blk1455 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1472 = assign49650_e64088;
        locals.var_guard1472_rv = 0.0;

        let (assign49660_e64103, assign49660_e64103_d_n5, assign49660_e64103_d_n6, assign49660_e64103_d_n7, assign49660_e64103_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
        let assign49660_e64101: f64 = (locals.var_sp_s_x0__blk1455).exp();
        (assign49660_e64101, (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn5), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn6), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn7), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn8),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49660_e64103;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49660_e64103_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49660_e64103_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49660_e64103_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49660_e64103_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let (assign49670_e64119, assign49670_e64119_d_n5, assign49670_e64119_d_n6, assign49670_e64119_d_n7, assign49670_e64119_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
        let assign49670_e64117: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
        (assign49670_e64117, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign49670_e64119;
        locals.var_sp_s_delta1__blk1442_dn5 = assign49670_e64119_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign49670_e64119_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign49670_e64119_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign49670_e64119_d_n8;
        locals.var_sp_s_delta1__blk1442_rv = 0.0;

        let (assign49680_e64135, assign49680_e64135_d_n5, assign49680_e64135_d_n6, assign49680_e64135_d_n7, assign49680_e64135_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
        let assign49680_e64133: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441);
        (assign49680_e64133, ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn8)),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49680_e64135;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49680_e64135_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49680_e64135_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49680_e64135_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49680_e64135_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let assign49690_e64139: f64 = (locals.var_xn_s__blk1332 - 230.25850929940458);
        let assign49690_e64140: f64 = if locals.var_sp_s_x0__blk1455 > assign49690_e64139 { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign49690_e64140;
        locals.var_guard1473_rv = 0.0;

        let (assign49700_e64160, assign49700_e64160_d_n5, assign49700_e64160_d_n6, assign49700_e64160_d_n7, assign49700_e64160_d_n8,) = {
    if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign49700_e64157: f64 = (locals.var_sp_s_x0__blk1455 - locals.var_xn_s__blk1332);
        let assign49700_e64158: f64 = (assign49700_e64157).exp();
        (assign49700_e64158, (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn5 - locals.var_xn_s__blk1332_dn5)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn6 - locals.var_xn_s__blk1332_dn6)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn7 - locals.var_xn_s__blk1332_dn7)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn8 - locals.var_xn_s__blk1332_dn8)),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49700_e64160;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49700_e64160_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49700_e64160_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49700_e64160_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49700_e64160_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let (assign49710_e64179, assign49710_e64179_d_n5, assign49710_e64179_d_n6, assign49710_e64179_d_n7, assign49710_e64179_d_n8,) = {
    if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign49710_e64177: f64 = (locals.var_delta_ns__blk1347 / locals.var_sp_s_delta0__blk1441);
        (assign49710_e64177, (((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn5)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn6)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn7)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn8)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign49710_e64179;
        locals.var_sp_s_delta1__blk1442_dn5 = assign49710_e64179_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign49710_e64179_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign49710_e64179_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign49710_e64179_d_n8;
        locals.var_sp_s_delta1__blk1442_rv = 0.0;

        let (assign49720_e64225, assign49720_e64225_d_n5, assign49720_e64225_d_n6, assign49720_e64225_d_n7, assign49720_e64225_d_n8,) = {
    if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) {
        let assign49720_e64199: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
        let assign49720_e64201: f64 = (assign49720_e64199 - 230.25850929940458);
        let assign49720_e64206: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
        let assign49720_e64208: f64 = (assign49720_e64206 - 230.25850929940458);
        let assign49720_e64212: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
        let assign49720_e64214: f64 = (assign49720_e64212 - 230.25850929940458);
        let assign49720_e64216: f64 = (assign49720_e64214 * 0.3333333333333333);
        let assign49720_e64217: f64 = (1.0 + assign49720_e64216);
        let assign49720_e64218: f64 = (assign49720_e64208 * assign49720_e64217);
        let assign49720_e64219: f64 = (0.5 * assign49720_e64218);
        let assign49720_e64220: f64 = (1.0 + assign49720_e64219);
        let assign49720_e64221: f64 = (assign49720_e64201 * assign49720_e64220);
        let assign49720_e64222: f64 = (1.0 + assign49720_e64221);
        let assign49720_e64223: f64 = (1e-100 / assign49720_e64222);
        (assign49720_e64223, (-((1e-100 * (((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49720_e64225;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49720_e64225_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49720_e64225_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49720_e64225_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49720_e64225_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let (assign49730_e64265, assign49730_e64265_d_n5, assign49730_e64265_d_n6, assign49730_e64265_d_n7, assign49730_e64265_d_n8,) = {
    if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) {
        let assign49730_e64245: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign49730_e64250: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign49730_e64254: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign49730_e64256: f64 = (assign49730_e64254 * 0.3333333333333333);
        let assign49730_e64257: f64 = (1.0 + assign49730_e64256);
        let assign49730_e64258: f64 = (assign49730_e64250 * assign49730_e64257);
        let assign49730_e64259: f64 = (0.5 * assign49730_e64258);
        let assign49730_e64260: f64 = (1.0 + assign49730_e64259);
        let assign49730_e64261: f64 = (assign49730_e64245 * assign49730_e64260);
        let assign49730_e64262: f64 = (1.0 + assign49730_e64261);
        let assign49730_e64263: f64 = (1e-100 / assign49730_e64262);
        (assign49730_e64263, (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn5 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn5 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn5 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn6 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn6 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn6 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn7 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn7 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn7 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn8 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn8 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn8 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign49730_e64265;
        locals.var_sp_s_delta1__blk1442_dn5 = assign49730_e64265_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign49730_e64265_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign49730_e64265_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign49730_e64265_d_n8;
        locals.var_sp_s_delta1__blk1442_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_40(
        locals: &mut StampLocals,
    ) {
        let (assign49740_e64283, assign49740_e64283_d_n5, assign49740_e64283_d_n6, assign49740_e64283_d_n7, assign49740_e64283_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49740_e64279: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
        let assign49740_e64280: f64 = (2.0 + assign49740_e64279);
        let assign49740_e64281: f64 = (1.0 / assign49740_e64280);
        (assign49740_e64281, (-(((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) / (assign49740_e64280 * assign49740_e64280))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49740_e64283;
        locals.var_sp_s_temp__blk1431_dn5 = assign49740_e64283_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49740_e64283_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49740_e64283_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49740_e64283_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49750_e64299, assign49750_e64299_d_n5, assign49750_e64299_d_n6, assign49750_e64299_d_n7, assign49750_e64299_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49750_e64295: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
        let assign49750_e64297: f64 = (assign49750_e64295 * locals.var_sp_s_temp__blk1431);
        (assign49750_e64297, ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign49750_e64299;
        locals.var_sp_s_xi0__blk1443_dn5 = assign49750_e64299_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign49750_e64299_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign49750_e64299_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign49750_e64299_d_n8;
        locals.var_sp_s_xi0__blk1443_rv = 0.0;

        let (assign49760_e64317, assign49760_e64317_d_n5, assign49760_e64317_d_n6, assign49760_e64317_d_n7, assign49760_e64317_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49760_e64312: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431);
        let assign49760_e64314: f64 = (assign49760_e64312 * locals.var_sp_s_temp__blk1431);
        let assign49760_e64315: f64 = (4.0 * assign49760_e64314);
        (assign49760_e64315, (4.0 * ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign49760_e64317;
        locals.var_sp_s_xi1__blk1444_dn5 = assign49760_e64317_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign49760_e64317_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign49760_e64317_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign49760_e64317_d_n8;
        locals.var_sp_s_xi1__blk1444_rv = 0.0;

        let (assign49770_e64339, assign49770_e64339_d_n5, assign49770_e64339_d_n6, assign49770_e64339_d_n7, assign49770_e64339_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49770_e64329: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
        let assign49770_e64332: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign49770_e64333: f64 = (assign49770_e64329 - assign49770_e64332);
        let assign49770_e64335: f64 = (assign49770_e64333 * locals.var_sp_s_temp__blk1431);
        let assign49770_e64337: f64 = (assign49770_e64335 * locals.var_sp_s_temp__blk1431);
        (assign49770_e64337, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign49770_e64339;
        locals.var_sp_s_xi2__blk1445_dn5 = assign49770_e64339_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign49770_e64339_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign49770_e64339_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign49770_e64339_d_n8;
        locals.var_sp_s_xi2__blk1445_rv = 0.0;

        let (assign49780_e64353, assign49780_e64353_d_n5, assign49780_e64353_d_n6, assign49780_e64353_d_n7, assign49780_e64353_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49780_e64351: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_x0__blk1455);
        (assign49780_e64351, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_x0__blk1455_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_x0__blk1455_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_x0__blk1455_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_x0__blk1455_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49780_e64353;
        locals.var_sp_s_temp__blk1431_dn5 = assign49780_e64353_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49780_e64353_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49780_e64353_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49780_e64353_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49790_e64381, assign49790_e64381_d_n5, assign49790_e64381_d_n6, assign49790_e64381_d_n7, assign49790_e64381_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49790_e64365: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign49790_e64369: f64 = (1.0 - locals.var_sp_s_delta1__blk1442);
        let assign49790_e64371: f64 = (assign49790_e64369 + locals.var_sp_s_delta0__blk1441);
        let assign49790_e64375: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
        let assign49790_e64376: f64 = (locals.var_delta_ns__blk1347 * assign49790_e64375);
        let assign49790_e64377: f64 = (assign49790_e64371 - assign49790_e64376);
        let assign49790_e64378: f64 = (locals.var_gf2__blk1308 * assign49790_e64377);
        let assign49790_e64379: f64 = (assign49790_e64365 + assign49790_e64378);
        (assign49790_e64379, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn8)))))),)
    } else {
        (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8,)
    }
};
        locals.var_sp_s_pc__blk1446 = assign49790_e64381;
        locals.var_sp_s_pc__blk1446_dn5 = assign49790_e64381_d_n5;
        locals.var_sp_s_pc__blk1446_dn6 = assign49790_e64381_d_n6;
        locals.var_sp_s_pc__blk1446_dn7 = assign49790_e64381_d_n7;
        locals.var_sp_s_pc__blk1446_dn8 = assign49790_e64381_d_n8;
        locals.var_sp_s_pc__blk1446_rv = 0.0;

        let (assign49800_e64413, assign49800_e64413_d_n5, assign49800_e64413_d_n6, assign49800_e64413_d_n7, assign49800_e64413_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49800_e64393: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49800_e64397: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_x0__blk1455);
        let assign49800_e64399: f64 = (assign49800_e64397 - 1.0);
        let assign49800_e64401: f64 = (assign49800_e64399 + locals.var_sp_s_delta0__blk1441);
        let assign49800_e64405: f64 = (locals.var_sp_s_x0__blk1455 + 1.0);
        let assign49800_e64407: f64 = (assign49800_e64405 + locals.var_sp_s_xi0__blk1443);
        let assign49800_e64408: f64 = (locals.var_delta_ns__blk1347 * assign49800_e64407);
        let assign49800_e64409: f64 = (assign49800_e64401 - assign49800_e64408);
        let assign49800_e64410: f64 = (locals.var_gf2__blk1308 * assign49800_e64409);
        let assign49800_e64411: f64 = (assign49800_e64393 - assign49800_e64410);
        (assign49800_e64411, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_x0__blk1455_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_x0__blk1455_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_x0__blk1455_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_x0__blk1455_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
    } else {
        (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8,)
    }
};
        locals.var_sp_s_qc__blk1447 = assign49800_e64413;
        locals.var_sp_s_qc__blk1447_dn5 = assign49800_e64413_d_n5;
        locals.var_sp_s_qc__blk1447_dn6 = assign49800_e64413_d_n6;
        locals.var_sp_s_qc__blk1447_dn7 = assign49800_e64413_d_n7;
        locals.var_sp_s_qc__blk1447_dn8 = assign49800_e64413_d_n8;
        locals.var_sp_s_qc__blk1447_rv = 0.0;

        let (assign49810_e64435, assign49810_e64435_d_n5, assign49810_e64435_d_n6, assign49810_e64435_d_n7, assign49810_e64435_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49810_e64427: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_delta0__blk1441);
        let assign49810_e64430: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
        let assign49810_e64431: f64 = (assign49810_e64427 - assign49810_e64430);
        let assign49810_e64432: f64 = (locals.var_gf2__blk1308 * assign49810_e64431);
        let assign49810_e64433: f64 = (2.0 - assign49810_e64432);
        (assign49810_e64433, (-((locals.var_gf2__blk1308_dn5 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8)))))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49810_e64435;
        locals.var_sp_s_temp__blk1431_dn5 = assign49810_e64435_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49810_e64435_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49810_e64435_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49810_e64435_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49820_e64455, assign49820_e64455_d_n5, assign49820_e64455_d_n6, assign49820_e64455_d_n7, assign49820_e64455_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49820_e64447: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
        let assign49820_e64451: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
        let assign49820_e64452: f64 = (2.0 * assign49820_e64451);
        let assign49820_e64453: f64 = (assign49820_e64447 - assign49820_e64452);
        (assign49820_e64453, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49820_e64455;
        locals.var_sp_s_temp__blk1431_dn5 = assign49820_e64455_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49820_e64455_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49820_e64455_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49820_e64455_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign49830_e64476, assign49830_e64476_d_n5, assign49830_e64476_d_n6, assign49830_e64476_d_n7, assign49830_e64476_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49830_e64470: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
        let assign49830_e64471: f64 = (locals.var_sp_s_pc__blk1446 + assign49830_e64470);
        let assign49830_e64472: f64 = (locals.var_sp_s_qc__blk1447 / assign49830_e64471);
        let assign49830_e64473: f64 = (2.0 * assign49830_e64472);
        let assign49830_e64474: f64 = (locals.var_sp_s_x0__blk1455 + assign49830_e64473);
        (assign49830_e64474, (locals.var_sp_s_x0__blk1455_dn5 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))),)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign49830_e64476;
        locals.var_x_s__blk1346_dn5 = assign49830_e64476_d_n5;
        locals.var_x_s__blk1346_dn6 = assign49830_e64476_d_n6;
        locals.var_x_s__blk1346_dn7 = assign49830_e64476_d_n7;
        locals.var_x_s__blk1346_dn8 = assign49830_e64476_d_n8;
        locals.var_x_s__blk1346_rv = 0.0;

        let (assign49840_e64482, assign49840_e64482_d_n5, assign49840_e64482_d_n6, assign49840_e64482_d_n7, assign49840_e64482_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8,)
    }
};
        locals.var_xi1s__blk1349 = assign49840_e64482;
        locals.var_xi1s__blk1349_dn5 = assign49840_e64482_d_n5;
        locals.var_xi1s__blk1349_dn6 = assign49840_e64482_d_n6;
        locals.var_xi1s__blk1349_dn7 = assign49840_e64482_d_n7;
        locals.var_xi1s__blk1349_dn8 = assign49840_e64482_d_n8;
        locals.var_xi1s__blk1349_rv = 0.0;

        let (assign49850_e64488, assign49850_e64488_d_n5, assign49850_e64488_d_n6, assign49850_e64488_d_n7, assign49850_e64488_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8,)
    }
};
        locals.var_xi2s__blk1350 = assign49850_e64488;
        locals.var_xi2s__blk1350_dn5 = assign49850_e64488_d_n5;
        locals.var_xi2s__blk1350_dn6 = assign49850_e64488_d_n6;
        locals.var_xi2s__blk1350_dn7 = assign49850_e64488_d_n7;
        locals.var_xi2s__blk1350_dn8 = assign49850_e64488_d_n8;
        locals.var_xi2s__blk1350_rv = 0.0;

        let (assign49860_e64494, assign49860_e64494_d_n5, assign49860_e64494_d_n6, assign49860_e64494_d_n7, assign49860_e64494_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign49860_e64494;
        locals.var_delta_1s__blk1351_dn5 = assign49860_e64494_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign49860_e64494_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign49860_e64494_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign49860_e64494_d_n8;
        locals.var_delta_1s__blk1351_rv = 0.0;

        let (assign49870_e64500, assign49870_e64500_d_n5, assign49870_e64500_d_n6, assign49870_e64500_d_n7, assign49870_e64500_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign49870_e64500;
        locals.var_es__blk1352_dn5 = assign49870_e64500_d_n5;
        locals.var_es__blk1352_dn6 = assign49870_e64500_d_n6;
        locals.var_es__blk1352_dn7 = assign49870_e64500_d_n7;
        locals.var_es__blk1352_dn8 = assign49870_e64500_d_n8;
        locals.var_es__blk1352_rv = 0.0;

        let (assign49880_e64506, assign49880_e64506_d_n5, assign49880_e64506_d_n6, assign49880_e64506_d_n7, assign49880_e64506_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    }
};
        locals.var_ds__blk1353 = assign49880_e64506;
        locals.var_ds__blk1353_dn5 = assign49880_e64506_d_n5;
        locals.var_ds__blk1353_dn6 = assign49880_e64506_d_n6;
        locals.var_ds__blk1353_dn7 = assign49880_e64506_d_n7;
        locals.var_ds__blk1353_dn8 = assign49880_e64506_d_n8;
        locals.var_ds__blk1353_rv = 0.0;

        let (assign49890_e64512, assign49890_e64512_d_n5, assign49890_e64512_d_n6, assign49890_e64512_d_n7, assign49890_e64512_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    }
};
        locals.var_ps__blk1354 = assign49890_e64512;
        locals.var_ps__blk1354_dn5 = assign49890_e64512_d_n5;
        locals.var_ps__blk1354_dn6 = assign49890_e64512_d_n6;
        locals.var_ps__blk1354_dn7 = assign49890_e64512_d_n7;
        locals.var_ps__blk1354_dn8 = assign49890_e64512_d_n8;
        locals.var_ps__blk1354_rv = 0.0;

        let (assign49900_e64518, assign49900_e64518_d_n5, assign49900_e64518_d_n6, assign49900_e64518_d_n7, assign49900_e64518_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8,)
    }
};
        locals.var_sqs__blk1355 = assign49900_e64518;
        locals.var_sqs__blk1355_dn5 = assign49900_e64518_d_n5;
        locals.var_sqs__blk1355_dn6 = assign49900_e64518_d_n6;
        locals.var_sqs__blk1355_dn7 = assign49900_e64518_d_n7;
        locals.var_sqs__blk1355_dn8 = assign49900_e64518_d_n8;
        locals.var_sqs__blk1355_rv = 0.0;

        let (assign49910_e64524, assign49910_e64524_d_n5, assign49910_e64524_d_n6, assign49910_e64524_d_n7, assign49910_e64524_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8,)
    }
};
        locals.var_alphas__blk1356 = assign49910_e64524;
        locals.var_alphas__blk1356_dn5 = assign49910_e64524_d_n5;
        locals.var_alphas__blk1356_dn6 = assign49910_e64524_d_n6;
        locals.var_alphas__blk1356_dn7 = assign49910_e64524_d_n7;
        locals.var_alphas__blk1356_dn8 = assign49910_e64524_d_n8;
        locals.var_alphas__blk1356_rv = 0.0;

        let (assign49920_e64530, assign49920_e64530_d_n5, assign49920_e64530_d_n6, assign49920_e64530_d_n7, assign49920_e64530_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8,)
    }
};
        locals.var_rxcor__blk1357 = assign49920_e64530;
        locals.var_rxcor__blk1357_dn5 = assign49920_e64530_d_n5;
        locals.var_rxcor__blk1357_dn6 = assign49920_e64530_d_n6;
        locals.var_rxcor__blk1357_dn7 = assign49920_e64530_d_n7;
        locals.var_rxcor__blk1357_dn8 = assign49920_e64530_d_n8;
        locals.var_rxcor__blk1357_rv = 0.0;

        let (assign49930_e64538, assign49930_e64538_d_n5, assign49930_e64538_d_n6, assign49930_e64538_d_n7, assign49930_e64538_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49930_e64536: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
        (assign49930_e64536, (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8),)
    } else {
        (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8,)
    }
};
        locals.var_xgs__blk1358 = assign49930_e64538;
        locals.var_xgs__blk1358_dn5 = assign49930_e64538_d_n5;
        locals.var_xgs__blk1358_dn6 = assign49930_e64538_d_n6;
        locals.var_xgs__blk1358_dn7 = assign49930_e64538_d_n7;
        locals.var_xgs__blk1358_dn8 = assign49930_e64538_d_n8;
        locals.var_xgs__blk1358_rv = 0.0;

        let (assign49940_e64544, assign49940_e64544_d_n5, assign49940_e64544_d_n6, assign49940_e64544_d_n7, assign49940_e64544_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8,)
    }
};
        locals.var_qis__blk1359 = assign49940_e64544;
        locals.var_qis__blk1359_dn5 = assign49940_e64544_d_n5;
        locals.var_qis__blk1359_dn6 = assign49940_e64544_d_n6;
        locals.var_qis__blk1359_dn7 = assign49940_e64544_d_n7;
        locals.var_qis__blk1359_dn8 = assign49940_e64544_d_n8;
        locals.var_qis__blk1359_rv = 0.0;

        let (assign49950_e64552, assign49950_e64552_d_n5, assign49950_e64552_d_n6, assign49950_e64552_d_n7, assign49950_e64552_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49950_e64550: f64 = (locals.var_phit1__blk1322 * locals.var_xgs__blk1358);
        (assign49950_e64550, ((locals.var_phit1__blk1322_dn5 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn5)), ((locals.var_phit1__blk1322_dn6 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn6)), ((locals.var_phit1__blk1322_dn7 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn7)), ((locals.var_phit1__blk1322_dn8 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn8)),)
    } else {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    }
};
        locals.var_qbs__blk1360 = assign49950_e64552;
        locals.var_qbs__blk1360_dn5 = assign49950_e64552_d_n5;
        locals.var_qbs__blk1360_dn6 = assign49950_e64552_d_n6;
        locals.var_qbs__blk1360_dn7 = assign49950_e64552_d_n7;
        locals.var_qbs__blk1360_dn8 = assign49950_e64552_d_n8;
        locals.var_qbs__blk1360_rv = 0.0;

        let (assign49960_e64558, assign49960_e64558_d_n5, assign49960_e64558_d_n6, assign49960_e64558_d_n7, assign49960_e64558_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8,)
    }
};
        locals.var_rhob__blk1361 = assign49960_e64558;
        locals.var_rhob__blk1361_dn5 = assign49960_e64558_d_n5;
        locals.var_rhob__blk1361_dn6 = assign49960_e64558_d_n6;
        locals.var_rhob__blk1361_dn7 = assign49960_e64558_d_n7;
        locals.var_rhob__blk1361_dn8 = assign49960_e64558_d_n8;
        locals.var_rhob__blk1361_rv = 0.0;

        let (assign49970_e64564, assign49970_e64564_d_n5, assign49970_e64564_d_n6, assign49970_e64564_d_n7, assign49970_e64564_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign49970_e64564;
        locals.var_rhog__blk1362_dn5 = assign49970_e64564_d_n5;
        locals.var_rhog__blk1362_dn6 = assign49970_e64564_d_n6;
        locals.var_rhog__blk1362_dn7 = assign49970_e64564_d_n7;
        locals.var_rhog__blk1362_dn8 = assign49970_e64564_d_n8;
        locals.var_rhog__blk1362_rv = 0.0;

        let (assign49980_e64570, assign49980_e64570_d_n5, assign49980_e64570_d_n6, assign49980_e64570_d_n7, assign49980_e64570_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8,)
    }
};
        locals.var_gmobs__blk1366 = assign49980_e64570;
        locals.var_gmobs__blk1366_dn5 = assign49980_e64570_d_n5;
        locals.var_gmobs__blk1366_dn6 = assign49980_e64570_d_n6;
        locals.var_gmobs__blk1366_dn7 = assign49980_e64570_d_n7;
        locals.var_gmobs__blk1366_dn8 = assign49980_e64570_d_n8;
        locals.var_gmobs__blk1366_rv = 0.0;

        let (assign49990_e64576, assign49990_e64576_d_n5, assign49990_e64576_d_n6, assign49990_e64576_d_n7, assign49990_e64576_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8,)
    }
};
        locals.var_xitsb__blk1367 = assign49990_e64576;
        locals.var_xitsb__blk1367_dn5 = assign49990_e64576_d_n5;
        locals.var_xitsb__blk1367_dn6 = assign49990_e64576_d_n6;
        locals.var_xitsb__blk1367_dn7 = assign49990_e64576_d_n7;
        locals.var_xitsb__blk1367_dn8 = assign49990_e64576_d_n8;
        locals.var_xitsb__blk1367_rv = 0.0;

        let (assign50000_e64582, assign50000_e64582_d_n5, assign50000_e64582_d_n6, assign50000_e64582_d_n7, assign50000_e64582_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign50000_e64582;
        locals.var_factheta__blk1369_dn5 = assign50000_e64582_d_n5;
        locals.var_factheta__blk1369_dn6 = assign50000_e64582_d_n6;
        locals.var_factheta__blk1369_dn7 = assign50000_e64582_d_n7;
        locals.var_factheta__blk1369_dn8 = assign50000_e64582_d_n8;
        locals.var_factheta__blk1369_rv = 0.0;

        let assign50010_e64585: f64 = if locals.var_xg__blk1326 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign50010_e64585;
        locals.var_guard1474_rv = 0.0;

        let (assign50020_e64599, assign50020_e64599_d_n5, assign50020_e64599_d_n6, assign50020_e64599_d_n7, assign50020_e64599_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50020_e64595: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
        let assign50020_e64596: f64 = (2.0 + assign50020_e64595);
        let assign50020_e64597: f64 = (1.0 / assign50020_e64596);
        (assign50020_e64597, (-(((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) / (assign50020_e64596 * assign50020_e64596))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign50020_e64599;
        locals.var_temp__blk936_dn5 = assign50020_e64599_d_n5;
        locals.var_temp__blk936_dn6 = assign50020_e64599_d_n6;
        locals.var_temp__blk936_dn7 = assign50020_e64599_d_n7;
        locals.var_temp__blk936_dn8 = assign50020_e64599_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign50030_e64611, assign50030_e64611_d_n5, assign50030_e64611_d_n6, assign50030_e64611_d_n7, assign50030_e64611_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50030_e64607: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
        let assign50030_e64609: f64 = (assign50030_e64607 * locals.var_temp__blk936);
        (assign50030_e64609, ((((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn5)), ((((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn6)), ((((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn7)), ((((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_xi0s__blk1348, locals.var_xi0s__blk1348_dn5, locals.var_xi0s__blk1348_dn6, locals.var_xi0s__blk1348_dn7, locals.var_xi0s__blk1348_dn8,)
    }
};
        locals.var_xi0s__blk1348 = assign50030_e64611;
        locals.var_xi0s__blk1348_dn5 = assign50030_e64611_d_n5;
        locals.var_xi0s__blk1348_dn6 = assign50030_e64611_d_n6;
        locals.var_xi0s__blk1348_dn7 = assign50030_e64611_d_n7;
        locals.var_xi0s__blk1348_dn8 = assign50030_e64611_d_n8;
        locals.var_xi0s__blk1348_rv = 0.0;

        let (assign50040_e64625, assign50040_e64625_d_n5, assign50040_e64625_d_n6, assign50040_e64625_d_n7, assign50040_e64625_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50040_e64620: f64 = (locals.var_x_s__blk1346 * locals.var_temp__blk936);
        let assign50040_e64622: f64 = (assign50040_e64620 * locals.var_temp__blk936);
        let assign50040_e64623: f64 = (4.0 * assign50040_e64622);
        (assign50040_e64623, (4.0 * ((((locals.var_x_s__blk1346_dn5 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn5))), (4.0 * ((((locals.var_x_s__blk1346_dn6 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn6))), (4.0 * ((((locals.var_x_s__blk1346_dn7 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn7))), (4.0 * ((((locals.var_x_s__blk1346_dn8 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8,)
    }
};
        locals.var_xi1s__blk1349 = assign50040_e64625;
        locals.var_xi1s__blk1349_dn5 = assign50040_e64625_d_n5;
        locals.var_xi1s__blk1349_dn6 = assign50040_e64625_d_n6;
        locals.var_xi1s__blk1349_dn7 = assign50040_e64625_d_n7;
        locals.var_xi1s__blk1349_dn8 = assign50040_e64625_d_n8;
        locals.var_xi1s__blk1349_rv = 0.0;

        let (assign50050_e64643, assign50050_e64643_d_n5, assign50050_e64643_d_n6, assign50050_e64643_d_n7, assign50050_e64643_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50050_e64633: f64 = (8.0 * locals.var_temp__blk936);
        let assign50050_e64636: f64 = (12.0 * locals.var_xi0s__blk1348);
        let assign50050_e64637: f64 = (assign50050_e64633 - assign50050_e64636);
        let assign50050_e64639: f64 = (assign50050_e64637 * locals.var_temp__blk936);
        let assign50050_e64641: f64 = (assign50050_e64639 * locals.var_temp__blk936);
        (assign50050_e64641, ((((((8.0 * locals.var_temp__blk936_dn5) - (12.0 * locals.var_xi0s__blk1348_dn5)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn5)), ((((((8.0 * locals.var_temp__blk936_dn6) - (12.0 * locals.var_xi0s__blk1348_dn6)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn6)), ((((((8.0 * locals.var_temp__blk936_dn7) - (12.0 * locals.var_xi0s__blk1348_dn7)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn7)), ((((((8.0 * locals.var_temp__blk936_dn8) - (12.0 * locals.var_xi0s__blk1348_dn8)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8,)
    }
};
        locals.var_xi2s__blk1350 = assign50050_e64643;
        locals.var_xi2s__blk1350_dn5 = assign50050_e64643_d_n5;
        locals.var_xi2s__blk1350_dn6 = assign50050_e64643_d_n6;
        locals.var_xi2s__blk1350_dn7 = assign50050_e64643_d_n7;
        locals.var_xi2s__blk1350_dn8 = assign50050_e64643_d_n8;
        locals.var_xi2s__blk1350_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_41(
        locals: &mut StampLocals,
    ) {
        let (assign50060_e64651, assign50060_e64651_d_n5, assign50060_e64651_d_n6, assign50060_e64651_d_n7, assign50060_e64651_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50060_e64651;
        locals.var_delta_1s__blk1351_dn5 = assign50060_e64651_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50060_e64651_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50060_e64651_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50060_e64651_d_n8;
        locals.var_delta_1s__blk1351_rv = 0.0;

        let assign50070_e64654: f64 = if locals.var_x_s__blk1346 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign50070_e64654;
        locals.var_guard1475_rv = 0.0;

        let (assign50080_e64665, assign50080_e64665_d_n5, assign50080_e64665_d_n6, assign50080_e64665_d_n7, assign50080_e64665_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign50080_e64663: f64 = (locals.var_x_s__blk1346).exp();
        (assign50080_e64663, (assign50080_e64663 * locals.var_x_s__blk1346_dn5), (assign50080_e64663 * locals.var_x_s__blk1346_dn6), (assign50080_e64663 * locals.var_x_s__blk1346_dn7), (assign50080_e64663 * locals.var_x_s__blk1346_dn8),)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50080_e64665;
        locals.var_delta_1s__blk1351_dn5 = assign50080_e64665_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50080_e64665_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50080_e64665_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50080_e64665_d_n8;
        locals.var_delta_1s__blk1351_rv = 0.0;

        let (assign50090_e64677, assign50090_e64677_d_n5, assign50090_e64677_d_n6, assign50090_e64677_d_n7, assign50090_e64677_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign50090_e64675: f64 = (1.0 / locals.var_delta_1s__blk1351);
        (assign50090_e64675, (-(locals.var_delta_1s__blk1351_dn5 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn6 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn7 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn8 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))),)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign50090_e64677;
        locals.var_es__blk1352_dn5 = assign50090_e64677_d_n5;
        locals.var_es__blk1352_dn6 = assign50090_e64677_d_n6;
        locals.var_es__blk1352_dn7 = assign50090_e64677_d_n7;
        locals.var_es__blk1352_dn8 = assign50090_e64677_d_n8;
        locals.var_es__blk1352_rv = 0.0;

        let (assign50100_e64689, assign50100_e64689_d_n5, assign50100_e64689_d_n6, assign50100_e64689_d_n7, assign50100_e64689_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign50100_e64687: f64 = (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351);
        (assign50100_e64687, ((locals.var_delta_ns__blk1347_dn5 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn8)),)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50100_e64689;
        locals.var_delta_1s__blk1351_dn5 = assign50100_e64689_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50100_e64689_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50100_e64689_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50100_e64689_d_n8;
        locals.var_delta_1s__blk1351_rv = 0.0;

        let assign50110_e64693: f64 = (locals.var_xn_s__blk1332 - 230.25850929940458);
        let assign50110_e64694: f64 = if locals.var_x_s__blk1346 > assign50110_e64693 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign50110_e64694;
        locals.var_guard1476_rv = 0.0;

        let (assign50120_e64710, assign50120_e64710_d_n5, assign50120_e64710_d_n6, assign50120_e64710_d_n7, assign50120_e64710_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign50120_e64707: f64 = (locals.var_x_s__blk1346 - locals.var_xn_s__blk1332);
        let assign50120_e64708: f64 = (assign50120_e64707).exp();
        (assign50120_e64708, (assign50120_e64708 * (locals.var_x_s__blk1346_dn5 - locals.var_xn_s__blk1332_dn5)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn6 - locals.var_xn_s__blk1332_dn6)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn7 - locals.var_xn_s__blk1332_dn7)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn8 - locals.var_xn_s__blk1332_dn8)),)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50120_e64710;
        locals.var_delta_1s__blk1351_dn5 = assign50120_e64710_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50120_e64710_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50120_e64710_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50120_e64710_d_n8;
        locals.var_delta_1s__blk1351_rv = 0.0;

        let (assign50130_e64725, assign50130_e64725_d_n5, assign50130_e64725_d_n6, assign50130_e64725_d_n7, assign50130_e64725_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign50130_e64723: f64 = (locals.var_delta_ns__blk1347 / locals.var_delta_1s__blk1351);
        (assign50130_e64723, (((locals.var_delta_ns__blk1347_dn5 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn5)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn6 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn6)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn7 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn7)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn8 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn8)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)),)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign50130_e64725;
        locals.var_es__blk1352_dn5 = assign50130_e64725_d_n5;
        locals.var_es__blk1352_dn6 = assign50130_e64725_d_n6;
        locals.var_es__blk1352_dn7 = assign50130_e64725_d_n7;
        locals.var_es__blk1352_dn8 = assign50130_e64725_d_n8;
        locals.var_es__blk1352_rv = 0.0;

        let (assign50140_e64767, assign50140_e64767_d_n5, assign50140_e64767_d_n6, assign50140_e64767_d_n7, assign50140_e64767_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) {
        let assign50140_e64741: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
        let assign50140_e64743: f64 = (assign50140_e64741 - 230.25850929940458);
        let assign50140_e64748: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
        let assign50140_e64750: f64 = (assign50140_e64748 - 230.25850929940458);
        let assign50140_e64754: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
        let assign50140_e64756: f64 = (assign50140_e64754 - 230.25850929940458);
        let assign50140_e64758: f64 = (assign50140_e64756 * 0.3333333333333333);
        let assign50140_e64759: f64 = (1.0 + assign50140_e64758);
        let assign50140_e64760: f64 = (assign50140_e64750 * assign50140_e64759);
        let assign50140_e64761: f64 = (0.5 * assign50140_e64760);
        let assign50140_e64762: f64 = (1.0 + assign50140_e64761);
        let assign50140_e64763: f64 = (assign50140_e64743 * assign50140_e64762);
        let assign50140_e64764: f64 = (1.0 + assign50140_e64763);
        let assign50140_e64765: f64 = (1e-100 / assign50140_e64764);
        (assign50140_e64765, (-((1e-100 * (((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))),)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50140_e64767;
        locals.var_delta_1s__blk1351_dn5 = assign50140_e64767_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50140_e64767_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50140_e64767_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50140_e64767_d_n8;
        locals.var_delta_1s__blk1351_rv = 0.0;

        let (assign50150_e64803, assign50150_e64803_d_n5, assign50150_e64803_d_n6, assign50150_e64803_d_n7, assign50150_e64803_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) {
        let assign50150_e64783: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
        let assign50150_e64788: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
        let assign50150_e64792: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
        let assign50150_e64794: f64 = (assign50150_e64792 * 0.3333333333333333);
        let assign50150_e64795: f64 = (1.0 + assign50150_e64794);
        let assign50150_e64796: f64 = (assign50150_e64788 * assign50150_e64795);
        let assign50150_e64797: f64 = (0.5 * assign50150_e64796);
        let assign50150_e64798: f64 = (1.0 + assign50150_e64797);
        let assign50150_e64799: f64 = (assign50150_e64783 * assign50150_e64798);
        let assign50150_e64800: f64 = (1.0 + assign50150_e64799);
        let assign50150_e64801: f64 = (1e-100 / assign50150_e64800);
        (assign50150_e64801, (-((1e-100 * ((locals.var_x_s__blk1346_dn5 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn5 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn5 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn6 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn6 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn6 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn7 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn7 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn7 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn8 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn8 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn8 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))),)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign50150_e64803;
        locals.var_es__blk1352_dn5 = assign50150_e64803_d_n5;
        locals.var_es__blk1352_dn6 = assign50150_e64803_d_n6;
        locals.var_es__blk1352_dn7 = assign50150_e64803_d_n7;
        locals.var_es__blk1352_dn8 = assign50150_e64803_d_n8;
        locals.var_es__blk1352_rv = 0.0;

        let (assign50160_e64819, assign50160_e64819_d_n5, assign50160_e64819_d_n6, assign50160_e64819_d_n7, assign50160_e64819_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50160_e64813: f64 = (locals.var_x_s__blk1346 + 1.0);
        let assign50160_e64815: f64 = (assign50160_e64813 + locals.var_xi0s__blk1348);
        let assign50160_e64816: f64 = (locals.var_delta_ns__blk1347 * assign50160_e64815);
        let assign50160_e64817: f64 = (locals.var_delta_1s__blk1351 - assign50160_e64816);
        (assign50160_e64817, (locals.var_delta_1s__blk1351_dn5 - ((locals.var_delta_ns__blk1347_dn5 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn5 + locals.var_xi0s__blk1348_dn5)))), (locals.var_delta_1s__blk1351_dn6 - ((locals.var_delta_ns__blk1347_dn6 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn6 + locals.var_xi0s__blk1348_dn6)))), (locals.var_delta_1s__blk1351_dn7 - ((locals.var_delta_ns__blk1347_dn7 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn7 + locals.var_xi0s__blk1348_dn7)))), (locals.var_delta_1s__blk1351_dn8 - ((locals.var_delta_ns__blk1347_dn8 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn8 + locals.var_xi0s__blk1348_dn8)))),)
    } else {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    }
};
        locals.var_ds__blk1353 = assign50160_e64819;
        locals.var_ds__blk1353_dn5 = assign50160_e64819_d_n5;
        locals.var_ds__blk1353_dn6 = assign50160_e64819_d_n6;
        locals.var_ds__blk1353_dn7 = assign50160_e64819_d_n7;
        locals.var_ds__blk1353_dn8 = assign50160_e64819_d_n8;
        locals.var_ds__blk1353_rv = 0.0;

        let assign50170_e64822: f64 = if locals.var_x_s__blk1346 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign50170_e64822;
        locals.var_guard1477_rv = 0.0;

        let (assign50180_e64848, assign50180_e64848_d_n5, assign50180_e64848_d_n6, assign50180_e64848_d_n7, assign50180_e64848_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50180_e64833: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
        let assign50180_e64840: f64 = (0.25 * locals.var_x_s__blk1346);
        let assign50180_e64841: f64 = (1.0 - assign50180_e64840);
        let assign50180_e64842: f64 = (locals.var_x_s__blk1346 * assign50180_e64841);
        let assign50180_e64843: f64 = (0.3333333333333333 * assign50180_e64842);
        let assign50180_e64844: f64 = (1.0 - assign50180_e64843);
        let assign50180_e64845: f64 = (assign50180_e64833 * assign50180_e64844);
        let assign50180_e64846: f64 = (0.5 * assign50180_e64845);
        (assign50180_e64846, (0.5 * ((((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn5 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn5))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn6 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn6))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn7 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn7))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn8 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn8))))))))),)
    } else {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    }
};
        locals.var_ps__blk1354 = assign50180_e64848;
        locals.var_ps__blk1354_dn5 = assign50180_e64848_d_n5;
        locals.var_ps__blk1354_dn6 = assign50180_e64848_d_n6;
        locals.var_ps__blk1354_dn7 = assign50180_e64848_d_n7;
        locals.var_ps__blk1354_dn8 = assign50180_e64848_d_n8;
        locals.var_ps__blk1354_rv = 0.0;

        let (assign50190_e64872, assign50190_e64872_d_n5, assign50190_e64872_d_n6, assign50190_e64872_d_n7, assign50190_e64872_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50190_e64859: f64 = (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346);
        let assign50190_e64861: f64 = (assign50190_e64859 * locals.var_x_s__blk1346);
        let assign50190_e64863: f64 = (assign50190_e64861 * locals.var_x_s__blk1346);
        let assign50190_e64867: f64 = (1.75 * locals.var_x_s__blk1346);
        let assign50190_e64868: f64 = (1.0 + assign50190_e64867);
        let assign50190_e64869: f64 = (assign50190_e64863 * assign50190_e64868);
        let assign50190_e64870: f64 = (0.16666666666666666 * assign50190_e64869);
        (assign50190_e64870, (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn5 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn5)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn5)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn5)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn5)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn6 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn6)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn6)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn6)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn7 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn7)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn7)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn7)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn8 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn8)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn8)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn8)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn8)))),)
    } else {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    }
};
        locals.var_ds__blk1353 = assign50190_e64872;
        locals.var_ds__blk1353_dn5 = assign50190_e64872_d_n5;
        locals.var_ds__blk1353_dn6 = assign50190_e64872_d_n6;
        locals.var_ds__blk1353_dn7 = assign50190_e64872_d_n7;
        locals.var_ds__blk1353_dn8 = assign50190_e64872_d_n8;
        locals.var_ds__blk1353_rv = 0.0;

        let (assign50200_e64893, assign50200_e64893_d_n5, assign50200_e64893_d_n6, assign50200_e64893_d_n7, assign50200_e64893_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50200_e64886: f64 = (0.25 * locals.var_x_s__blk1346);
        let assign50200_e64887: f64 = (1.0 - assign50200_e64886);
        let assign50200_e64888: f64 = (locals.var_x_s__blk1346 * assign50200_e64887);
        let assign50200_e64889: f64 = (0.3333333333333333 * assign50200_e64888);
        let assign50200_e64890: f64 = (1.0 - assign50200_e64889);
        let assign50200_e64891: f64 = (assign50200_e64890).sqrt();
        (assign50200_e64891, ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn5 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn5)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn6 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn6)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn7 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn7)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn8 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn8)))))) / (2.0 * assign50200_e64891)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign50200_e64893;
        locals.var_temp__blk936_dn5 = assign50200_e64893_d_n5;
        locals.var_temp__blk936_dn6 = assign50200_e64893_d_n6;
        locals.var_temp__blk936_dn7 = assign50200_e64893_d_n7;
        locals.var_temp__blk936_dn8 = assign50200_e64893_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign50210_e64907, assign50210_e64907_d_n5, assign50210_e64907_d_n6, assign50210_e64907_d_n7, assign50210_e64907_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50210_e64904: f64 = (locals.var_x_s__blk1346 * locals.var_temp__blk936);
        let assign50210_e64905: f64 = (0.7071067811865475 * assign50210_e64904);
        (assign50210_e64905, (0.7071067811865475 * ((locals.var_x_s__blk1346_dn5 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn6 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn7 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn8 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8,)
    }
};
        locals.var_sqs__blk1355 = assign50210_e64907;
        locals.var_sqs__blk1355_dn5 = assign50210_e64907_d_n5;
        locals.var_sqs__blk1355_dn6 = assign50210_e64907_d_n6;
        locals.var_sqs__blk1355_dn7 = assign50210_e64907_d_n7;
        locals.var_sqs__blk1355_dn8 = assign50210_e64907_d_n8;
        locals.var_sqs__blk1355_rv = 0.0;

        let (assign50220_e64935, assign50220_e64935_d_n5, assign50220_e64935_d_n6, assign50220_e64935_d_n7, assign50220_e64935_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50220_e64921: f64 = (0.5 * locals.var_x_s__blk1346);
        let assign50220_e64922: f64 = (1.0 - assign50220_e64921);
        let assign50220_e64926: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
        let assign50220_e64927: f64 = (0.16666666666666666 * assign50220_e64926);
        let assign50220_e64928: f64 = (assign50220_e64922 + assign50220_e64927);
        let assign50220_e64929: f64 = (locals.var_gf__blk1307 * assign50220_e64928);
        let assign50220_e64931: f64 = (assign50220_e64929 / locals.var_temp__blk936);
        let assign50220_e64932: f64 = (0.7071067811865475 * assign50220_e64931);
        let assign50220_e64933: f64 = (1.0 + assign50220_e64932);
        (assign50220_e64933, (0.7071067811865475 * (((((locals.var_gf__blk1307_dn5 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn5)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn6 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn6)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn7 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn7)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn8 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn8)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936))),)
    } else {
        (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8,)
    }
};
        locals.var_alphas__blk1356 = assign50220_e64935;
        locals.var_alphas__blk1356_dn5 = assign50220_e64935_d_n5;
        locals.var_alphas__blk1356_dn6 = assign50220_e64935_d_n6;
        locals.var_alphas__blk1356_dn7 = assign50220_e64935_d_n7;
        locals.var_alphas__blk1356_dn8 = assign50220_e64935_d_n8;
        locals.var_alphas__blk1356_rv = 0.0;

        let (assign50230_e64950, assign50230_e64950_d_n5, assign50230_e64950_d_n6, assign50230_e64950_d_n7, assign50230_e64950_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let assign50230_e64946: f64 = (locals.var_x_s__blk1346 - 1.0);
        let assign50230_e64948: f64 = (assign50230_e64946 + locals.var_es__blk1352);
        (assign50230_e64948, (locals.var_x_s__blk1346_dn5 + locals.var_es__blk1352_dn5), (locals.var_x_s__blk1346_dn6 + locals.var_es__blk1352_dn6), (locals.var_x_s__blk1346_dn7 + locals.var_es__blk1352_dn7), (locals.var_x_s__blk1346_dn8 + locals.var_es__blk1352_dn8),)
    } else {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    }
};
        locals.var_ps__blk1354 = assign50230_e64950;
        locals.var_ps__blk1354_dn5 = assign50230_e64950_d_n5;
        locals.var_ps__blk1354_dn6 = assign50230_e64950_d_n6;
        locals.var_ps__blk1354_dn7 = assign50230_e64950_d_n7;
        locals.var_ps__blk1354_dn8 = assign50230_e64950_d_n8;
        locals.var_ps__blk1354_rv = 0.0;

        let (assign50240_e64962, assign50240_e64962_d_n5, assign50240_e64962_d_n6, assign50240_e64962_d_n7, assign50240_e64962_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let assign50240_e64960: f64 = (locals.var_ps__blk1354).sqrt();
        (assign50240_e64960, (locals.var_ps__blk1354_dn5 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn6 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn7 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn8 / (2.0 * assign50240_e64960)),)
    } else {
        (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8,)
    }
};
        locals.var_sqs__blk1355 = assign50240_e64962;
        locals.var_sqs__blk1355_dn5 = assign50240_e64962_d_n5;
        locals.var_sqs__blk1355_dn6 = assign50240_e64962_d_n6;
        locals.var_sqs__blk1355_dn7 = assign50240_e64962_d_n7;
        locals.var_sqs__blk1355_dn8 = assign50240_e64962_d_n8;
        locals.var_sqs__blk1355_rv = 0.0;

        let (assign50250_e64983, assign50250_e64983_d_n5, assign50250_e64983_d_n6, assign50250_e64983_d_n7, assign50250_e64983_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let assign50250_e64976: f64 = (1.0 - locals.var_es__blk1352);
        let assign50250_e64977: f64 = (locals.var_gf__blk1307 * assign50250_e64976);
        let assign50250_e64979: f64 = (assign50250_e64977 / locals.var_sqs__blk1355);
        let assign50250_e64980: f64 = (0.5 * assign50250_e64979);
        let assign50250_e64981: f64 = (1.0 + assign50250_e64980);
        (assign50250_e64981, (0.5 * (((((locals.var_gf__blk1307_dn5 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn5))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn5)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn6 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn6))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn6)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn7 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn7))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn7)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn8 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn8))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn8)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))),)
    } else {
        (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8,)
    }
};
        locals.var_alphas__blk1356 = assign50250_e64983;
        locals.var_alphas__blk1356_dn5 = assign50250_e64983_d_n5;
        locals.var_alphas__blk1356_dn6 = assign50250_e64983_d_n6;
        locals.var_alphas__blk1356_dn7 = assign50250_e64983_d_n7;
        locals.var_alphas__blk1356_dn8 = assign50250_e64983_d_n8;
        locals.var_alphas__blk1356_rv = 0.0;

        let (assign50260_e65003, assign50260_e65003_d_n5, assign50260_e65003_d_n6, assign50260_e65003_d_n7, assign50260_e65003_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50260_e64992: f64 = (0.2 * locals.var_xcor_t);
        let assign50260_e64994: f64 = (assign50260_e64992 * locals.var_vsbx__blk1306);
        let assign50260_e64995: f64 = (1.0 + assign50260_e64994);
        let assign50260_e64999: f64 = (locals.var_xcor_t * locals.var_vsbx__blk1306);
        let assign50260_e65000: f64 = (1.0 + assign50260_e64999);
        let assign50260_e65001: f64 = (assign50260_e64995 / assign50260_e65000);
        (assign50260_e65001, ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn5) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn5))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn6) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn6))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn7) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn7))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn8) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn8))) / (assign50260_e65000 * assign50260_e65000)),)
    } else {
        (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8,)
    }
};
        locals.var_rxcor__blk1357 = assign50260_e65003;
        locals.var_rxcor__blk1357_dn5 = assign50260_e65003_d_n5;
        locals.var_rxcor__blk1357_dn6 = assign50260_e65003_d_n6;
        locals.var_rxcor__blk1357_dn7 = assign50260_e65003_d_n7;
        locals.var_rxcor__blk1357_dn8 = assign50260_e65003_d_n8;
        locals.var_rxcor__blk1357_rv = 0.0;

        let assign50270_e65006: f64 = if locals.var_ds__blk1353 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign50270_e65006;
        locals.var_guard1478_rv = 0.0;

        let (assign50280_e65021, assign50280_e65021_d_n5, assign50280_e65021_d_n6, assign50280_e65021_d_n7, assign50280_e65021_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50280_e65017: f64 = (locals.var_ps__blk1354 + locals.var_ds__blk1353);
        let assign50280_e65018: f64 = (assign50280_e65017).sqrt();
        let assign50280_e65019: f64 = (locals.var_gf__blk1307 * assign50280_e65018);
        (assign50280_e65019, ((locals.var_gf__blk1307_dn5 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn5 + locals.var_ds__blk1353_dn5) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn6 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn6 + locals.var_ds__blk1353_dn6) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn7 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn7 + locals.var_ds__blk1353_dn7) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn8 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn8 + locals.var_ds__blk1353_dn8) / (2.0 * assign50280_e65018)))),)
    } else {
        (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8,)
    }
};
        locals.var_xgs__blk1358 = assign50280_e65021;
        locals.var_xgs__blk1358_dn5 = assign50280_e65021_d_n5;
        locals.var_xgs__blk1358_dn6 = assign50280_e65021_d_n6;
        locals.var_xgs__blk1358_dn7 = assign50280_e65021_d_n7;
        locals.var_xgs__blk1358_dn8 = assign50280_e65021_d_n8;
        locals.var_xgs__blk1358_rv = 0.0;

        let (assign50290_e65041, assign50290_e65041_d_n5, assign50290_e65041_d_n6, assign50290_e65041_d_n7, assign50290_e65041_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50290_e65031: f64 = (locals.var_gf2__blk1308 * locals.var_ds__blk1353);
        let assign50290_e65033: f64 = (assign50290_e65031 * locals.var_phit1__blk1322);
        let assign50290_e65037: f64 = (locals.var_gf__blk1307 * locals.var_sqs__blk1355);
        let assign50290_e65038: f64 = (locals.var_xgs__blk1358 + assign50290_e65037);
        let assign50290_e65039: f64 = (assign50290_e65033 / assign50290_e65038);
        (assign50290_e65039, (((((((locals.var_gf2__blk1308_dn5 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn5)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn5)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn5 + ((locals.var_gf__blk1307_dn5 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn5))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn6 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn6)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn6)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn6 + ((locals.var_gf__blk1307_dn6 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn6))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn7 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn7)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn7)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn7 + ((locals.var_gf__blk1307_dn7 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn7))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn8 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn8)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn8)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn8 + ((locals.var_gf__blk1307_dn8 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn8))))) / (assign50290_e65038 * assign50290_e65038)),)
    } else {
        (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8,)
    }
};
        locals.var_qis__blk1359 = assign50290_e65041;
        locals.var_qis__blk1359_dn5 = assign50290_e65041_d_n5;
        locals.var_qis__blk1359_dn6 = assign50290_e65041_d_n6;
        locals.var_qis__blk1359_dn7 = assign50290_e65041_d_n7;
        locals.var_qis__blk1359_dn8 = assign50290_e65041_d_n8;
        locals.var_qis__blk1359_rv = 0.0;

        let (assign50300_e65055, assign50300_e65055_d_n5, assign50300_e65055_d_n6, assign50300_e65055_d_n7, assign50300_e65055_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50300_e65051: f64 = (locals.var_sqs__blk1355 * locals.var_gf__blk1307);
        let assign50300_e65053: f64 = (assign50300_e65051 * locals.var_phit1__blk1322);
        (assign50300_e65053, ((((locals.var_sqs__blk1355_dn5 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqs__blk1355_dn6 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqs__blk1355_dn7 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqs__blk1355_dn8 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    }
};
        locals.var_qbs__blk1360 = assign50300_e65055;
        locals.var_qbs__blk1360_dn5 = assign50300_e65055_d_n5;
        locals.var_qbs__blk1360_dn6 = assign50300_e65055_d_n6;
        locals.var_qbs__blk1360_dn7 = assign50300_e65055_d_n7;
        locals.var_qbs__blk1360_dn8 = assign50300_e65055_d_n8;
        locals.var_qbs__blk1360_rv = 0.0;

        let assign50310_e65058: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign50310_e65058;
        locals.var_guard1479_rv = 0.0;

        let (assign50320_e65076, assign50320_e65076_d_n5, assign50320_e65076_d_n6, assign50320_e65076_d_n7, assign50320_e65076_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 != 0.0)) {
        let assign50320_e65072: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1306);
        let assign50320_e65073: f64 = (1.0 - assign50320_e65072);
        let assign50320_e65074: f64 = (1.0 / assign50320_e65073);
        (assign50320_e65074, (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn5)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn6)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn7)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn8)) / (assign50320_e65073 * assign50320_e65073))),)
    } else {
        (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8,)
    }
};
        locals.var_rhob__blk1361 = assign50320_e65076;
        locals.var_rhob__blk1361_dn5 = assign50320_e65076_d_n5;
        locals.var_rhob__blk1361_dn6 = assign50320_e65076_d_n6;
        locals.var_rhob__blk1361_dn7 = assign50320_e65076_d_n7;
        locals.var_rhob__blk1361_dn8 = assign50320_e65076_d_n8;
        locals.var_rhob__blk1361_rv = 0.0;

        let (assign50330_e65093, assign50330_e65093_d_n5, assign50330_e65093_d_n6, assign50330_e65093_d_n7, assign50330_e65093_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
        let assign50330_e65090: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1306);
        let assign50330_e65091: f64 = (1.0 + assign50330_e65090);
        (assign50330_e65091, (locals.var_rsb_i * locals.var_vsbx__blk1306_dn5), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn6), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn7), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn8),)
    } else {
        (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8,)
    }
};
        locals.var_rhob__blk1361 = assign50330_e65093;
        locals.var_rhob__blk1361_dn5 = assign50330_e65093_d_n5;
        locals.var_rhob__blk1361_dn6 = assign50330_e65093_d_n6;
        locals.var_rhob__blk1361_dn7 = assign50330_e65093_d_n7;
        locals.var_rhob__blk1361_dn8 = assign50330_e65093_d_n8;
        locals.var_rhob__blk1361_rv = 0.0;

        let assign50340_e65096: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign50340_e65096;
        locals.var_guard1480_rv = 0.0;

        let (assign50350_e65112, assign50350_e65112_d_n5, assign50350_e65112_d_n6, assign50350_e65112_d_n7, assign50350_e65112_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1480 != 0.0)) {
        let assign50350_e65109: f64 = (locals.var_rsg_i * locals.var_qis__blk1359);
        let assign50350_e65110: f64 = (1.0 - assign50350_e65109);
        (assign50350_e65110, (-(locals.var_rsg_i * locals.var_qis__blk1359_dn5)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn6)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn7)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn8)),)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign50350_e65112;
        locals.var_rhog__blk1362_dn5 = assign50350_e65112_d_n5;
        locals.var_rhog__blk1362_dn6 = assign50350_e65112_d_n6;
        locals.var_rhog__blk1362_dn7 = assign50350_e65112_d_n7;
        locals.var_rhog__blk1362_dn8 = assign50350_e65112_d_n8;
        locals.var_rhog__blk1362_rv = 0.0;

        let (assign50360_e65131, assign50360_e65131_d_n5, assign50360_e65131_d_n6, assign50360_e65131_d_n7, assign50360_e65131_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1480 == 0.0)) {
        let assign50360_e65127: f64 = (locals.var_rsg_i * locals.var_qis__blk1359);
        let assign50360_e65128: f64 = (1.0 + assign50360_e65127);
        let assign50360_e65129: f64 = (1.0 / assign50360_e65128);
        (assign50360_e65129, (-((locals.var_rsg_i * locals.var_qis__blk1359_dn5) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn6) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn7) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn8) / (assign50360_e65128 * assign50360_e65128))),)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign50360_e65131;
        locals.var_rhog__blk1362_dn5 = assign50360_e65131_d_n5;
        locals.var_rhog__blk1362_dn6 = assign50360_e65131_d_n6;
        locals.var_rhog__blk1362_dn7 = assign50360_e65131_d_n7;
        locals.var_rhog__blk1362_dn8 = assign50360_e65131_d_n8;
        locals.var_rhog__blk1362_rv = 0.0;

        let (assign50370_e65147, assign50370_e65147_d_n5, assign50370_e65147_d_n6, assign50370_e65147_d_n7, assign50370_e65147_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50370_e65141: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
        let assign50370_e65143: f64 = (assign50370_e65141 * locals.var_rhog__blk1362);
        let assign50370_e65145: f64 = (assign50370_e65143 * locals.var_qis__blk1359);
        (assign50370_e65145, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn5)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn6)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn7)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn8)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn8)),)
    } else {
        (locals.var_gr__blk1363, locals.var_gr__blk1363_dn5, locals.var_gr__blk1363_dn6, locals.var_gr__blk1363_dn7, locals.var_gr__blk1363_dn8,)
    }
};
        locals.var_gr__blk1363 = assign50370_e65147;
        locals.var_gr__blk1363_dn5 = assign50370_e65147_d_n5;
        locals.var_gr__blk1363_dn6 = assign50370_e65147_d_n6;
        locals.var_gr__blk1363_dn7 = assign50370_e65147_d_n7;
        locals.var_gr__blk1363_dn8 = assign50370_e65147_d_n8;
        locals.var_gr__blk1363_rv = 0.0;

        let (assign50380_e65163, assign50380_e65163_d_n5, assign50380_e65163_d_n6, assign50380_e65163_d_n7, assign50380_e65163_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50380_e65159: f64 = (locals.var_eta_mu * locals.var_qis__blk1359);
        let assign50380_e65160: f64 = (locals.var_qbs__blk1360 + assign50380_e65159);
        let assign50380_e65161: f64 = (locals.var_e_eff0 * assign50380_e65160);
        (assign50380_e65161, (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn5 + (locals.var_eta_mu * locals.var_qis__blk1359_dn5))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn6 + (locals.var_eta_mu * locals.var_qis__blk1359_dn6))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn7 + (locals.var_eta_mu * locals.var_qis__blk1359_dn7))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn8 + (locals.var_eta_mu * locals.var_qis__blk1359_dn8))),)
    } else {
        (locals.var_eeffs__blk1364, locals.var_eeffs__blk1364_dn5, locals.var_eeffs__blk1364_dn6, locals.var_eeffs__blk1364_dn7, locals.var_eeffs__blk1364_dn8,)
    }
};
        locals.var_eeffs__blk1364 = assign50380_e65163;
        locals.var_eeffs__blk1364_dn5 = assign50380_e65163_d_n5;
        locals.var_eeffs__blk1364_dn6 = assign50380_e65163_d_n6;
        locals.var_eeffs__blk1364_dn7 = assign50380_e65163_d_n7;
        locals.var_eeffs__blk1364_dn8 = assign50380_e65163_d_n8;
        locals.var_eeffs__blk1364_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_42(
        locals: &mut StampLocals,
    ) {
        let (assign50390_e65180, assign50390_e65180_d_n5, assign50390_e65180_d_n6, assign50390_e65180_d_n7, assign50390_e65180_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50390_e65174: f64 = (locals.var_ps__blk1354 + locals.var_ds__blk1353);
        let assign50390_e65176: f64 = (assign50390_e65174 + 1e-14);
        let assign50390_e65177: f64 = (locals.var_ps__blk1354 / assign50390_e65176);
        let assign50390_e65178: f64 = (assign50390_e65177).ln();
        (assign50390_e65178, ((((locals.var_ps__blk1354_dn5 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn5 + locals.var_ds__blk1353_dn5))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn6 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn6 + locals.var_ds__blk1353_dn6))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn7 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn7 + locals.var_ds__blk1353_dn7))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn8 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn8 + locals.var_ds__blk1353_dn8))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign50390_e65180;
        locals.var_temp1_dn5 = assign50390_e65180_d_n5;
        locals.var_temp1_dn6 = assign50390_e65180_d_n6;
        locals.var_temp1_dn7 = assign50390_e65180_d_n7;
        locals.var_temp1_dn8 = assign50390_e65180_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign50400_e65203, assign50400_e65203_d_n5, assign50400_e65203_d_n6, assign50400_e65203_d_n7, assign50400_e65203_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50400_e65190: f64 = (locals.var_eeffs__blk1364 * locals.var_mue_t);
        let assign50400_e65192: f64 = (assign50400_e65190).powf(locals.var_themu_t);
        let assign50400_e65196: f64 = (0.5 * locals.var_thecs_t);
        let assign50400_e65198: f64 = (assign50400_e65196 * locals.var_temp1);
        let assign50400_e65199: f64 = (assign50400_e65198).exp();
        let assign50400_e65200: f64 = (locals.var_cs_t * assign50400_e65199);
        let assign50400_e65201: f64 = (assign50400_e65192 + assign50400_e65200);
        (assign50400_e65201, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn5 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn5 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn6 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn6 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn7 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn7 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn8 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn8 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn8)))),)
    } else {
        (locals.var_mutmp__blk1365, locals.var_mutmp__blk1365_dn5, locals.var_mutmp__blk1365_dn6, locals.var_mutmp__blk1365_dn7, locals.var_mutmp__blk1365_dn8,)
    }
};
        locals.var_mutmp__blk1365 = assign50400_e65203;
        locals.var_mutmp__blk1365_dn5 = assign50400_e65203_d_n5;
        locals.var_mutmp__blk1365_dn6 = assign50400_e65203_d_n6;
        locals.var_mutmp__blk1365_dn7 = assign50400_e65203_d_n7;
        locals.var_mutmp__blk1365_dn8 = assign50400_e65203_d_n8;
        locals.var_mutmp__blk1365_rv = 0.0;

        let (assign50410_e65219, assign50410_e65219_d_n5, assign50410_e65219_d_n6, assign50410_e65219_d_n7, assign50410_e65219_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50410_e65213: f64 = (1.0 + locals.var_mutmp__blk1365);
        let assign50410_e65215: f64 = (assign50410_e65213 + locals.var_gr__blk1363);
        let assign50410_e65217: f64 = (assign50410_e65215 * locals.var_rxcor__blk1357);
        (assign50410_e65217, (((locals.var_mutmp__blk1365_dn5 + locals.var_gr__blk1363_dn5) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn5)), (((locals.var_mutmp__blk1365_dn6 + locals.var_gr__blk1363_dn6) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn6)), (((locals.var_mutmp__blk1365_dn7 + locals.var_gr__blk1363_dn7) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn7)), (((locals.var_mutmp__blk1365_dn8 + locals.var_gr__blk1363_dn8) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn8)),)
    } else {
        (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8,)
    }
};
        locals.var_gmobs__blk1366 = assign50410_e65219;
        locals.var_gmobs__blk1366_dn5 = assign50410_e65219_d_n5;
        locals.var_gmobs__blk1366_dn6 = assign50410_e65219_d_n6;
        locals.var_gmobs__blk1366_dn7 = assign50410_e65219_d_n7;
        locals.var_gmobs__blk1366_dn8 = assign50410_e65219_d_n8;
        locals.var_gmobs__blk1366_rv = 0.0;

        let assign50420_e65222: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1481 = assign50420_e65222;
        locals.var_guard1481_rv = 0.0;

        let (assign50430_e65240, assign50430_e65240_d_n5, assign50430_e65240_d_n6, assign50430_e65240_d_n7, assign50430_e65240_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign50430_e65236: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1306);
        let assign50430_e65237: f64 = (1.0 - assign50430_e65236);
        let assign50430_e65238: f64 = (1.0 / assign50430_e65237);
        (assign50430_e65238, (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn5)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn6)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn7)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn8)) / (assign50430_e65237 * assign50430_e65237))),)
    } else {
        (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8,)
    }
};
        locals.var_xitsb__blk1367 = assign50430_e65240;
        locals.var_xitsb__blk1367_dn5 = assign50430_e65240_d_n5;
        locals.var_xitsb__blk1367_dn6 = assign50430_e65240_d_n6;
        locals.var_xitsb__blk1367_dn7 = assign50430_e65240_d_n7;
        locals.var_xitsb__blk1367_dn8 = assign50430_e65240_d_n8;
        locals.var_xitsb__blk1367_rv = 0.0;

        let (assign50440_e65257, assign50440_e65257_d_n5, assign50440_e65257_d_n6, assign50440_e65257_d_n7, assign50440_e65257_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1481 == 0.0)) {
        let assign50440_e65254: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1306);
        let assign50440_e65255: f64 = (1.0 + assign50440_e65254);
        (assign50440_e65255, (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn5), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn6), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn7), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn8),)
    } else {
        (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8,)
    }
};
        locals.var_xitsb__blk1367 = assign50440_e65257;
        locals.var_xitsb__blk1367_dn5 = assign50440_e65257_d_n5;
        locals.var_xitsb__blk1367_dn6 = assign50440_e65257_d_n6;
        locals.var_xitsb__blk1367_dn7 = assign50440_e65257_d_n7;
        locals.var_xitsb__blk1367_dn8 = assign50440_e65257_d_n8;
        locals.var_xitsb__blk1367_rv = 0.0;

        let (assign50450_e65269, assign50450_e65269_d_n5, assign50450_e65269_d_n6, assign50450_e65269_d_n7, assign50450_e65269_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50450_e65267: f64 = (locals.var_qis__blk1359 * locals.var_xitsb__blk1367);
        (assign50450_e65267, ((locals.var_qis__blk1359_dn5 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn5)), ((locals.var_qis__blk1359_dn6 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn6)), ((locals.var_qis__blk1359_dn7 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn7)), ((locals.var_qis__blk1359_dn8 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign50450_e65269;
        locals.var_temp2_dn5 = assign50450_e65269_d_n5;
        locals.var_temp2_dn6 = assign50450_e65269_d_n6;
        locals.var_temp2_dn7 = assign50450_e65269_d_n7;
        locals.var_temp2_dn8 = assign50450_e65269_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign50460_e65283, assign50460_e65283_d_n5, assign50460_e65283_d_n6, assign50460_e65283_d_n7, assign50460_e65283_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50460_e65280: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign50460_e65281: f64 = (locals.var_temp2 / assign50460_e65280);
        (assign50460_e65281, (((locals.var_temp2_dn5 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn6 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn7 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn8 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign50460_e65280 * assign50460_e65280)),)
    } else {
        (locals.var_wsat__blk1368, locals.var_wsat__blk1368_dn5, locals.var_wsat__blk1368_dn6, locals.var_wsat__blk1368_dn7, locals.var_wsat__blk1368_dn8,)
    }
};
        locals.var_wsat__blk1368 = assign50460_e65283;
        locals.var_wsat__blk1368_dn5 = assign50460_e65283_d_n5;
        locals.var_wsat__blk1368_dn6 = assign50460_e65283_d_n6;
        locals.var_wsat__blk1368_dn7 = assign50460_e65283_d_n7;
        locals.var_wsat__blk1368_dn8 = assign50460_e65283_d_n8;
        locals.var_wsat__blk1368_rv = 0.0;

        let assign50470_e65286: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1482 = assign50470_e65286;
        locals.var_guard1482_rv = 0.0;

        let (assign50480_e65304, assign50480_e65304_d_n5, assign50480_e65304_d_n6, assign50480_e65304_d_n7, assign50480_e65304_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign50480_e65300: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
        let assign50480_e65301: f64 = (1.0 - assign50480_e65300);
        let assign50480_e65302: f64 = (1.0 / assign50480_e65301);
        (assign50480_e65302, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn5)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn6)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn7)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn8)) / (assign50480_e65301 * assign50480_e65301))),)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign50480_e65304;
        locals.var_factheta__blk1369_dn5 = assign50480_e65304_d_n5;
        locals.var_factheta__blk1369_dn6 = assign50480_e65304_d_n6;
        locals.var_factheta__blk1369_dn7 = assign50480_e65304_d_n7;
        locals.var_factheta__blk1369_dn8 = assign50480_e65304_d_n8;
        locals.var_factheta__blk1369_rv = 0.0;

        let (assign50490_e65321, assign50490_e65321_d_n5, assign50490_e65321_d_n6, assign50490_e65321_d_n7, assign50490_e65321_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1482 == 0.0)) {
        let assign50490_e65318: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
        let assign50490_e65319: f64 = (1.0 + assign50490_e65318);
        (assign50490_e65319, (locals.var_thesatg_i * locals.var_wsat__blk1368_dn5), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn8),)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign50490_e65321;
        locals.var_factheta__blk1369_dn5 = assign50490_e65321_d_n5;
        locals.var_factheta__blk1369_dn6 = assign50490_e65321_d_n6;
        locals.var_factheta__blk1369_dn7 = assign50490_e65321_d_n7;
        locals.var_factheta__blk1369_dn8 = assign50490_e65321_d_n8;
        locals.var_factheta__blk1369_rv = 0.0;

        let (assign50590_e65420, assign50590_e65420_d_n5, assign50590_e65420_d_n6, assign50590_e65420_d_n7, assign50590_e65420_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_vgb1_dc, locals.var_vgb1_dc_dn5, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8,)
    } else {
        (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8,)
    }
};
        locals.var_vgb1__blk1304 = assign50590_e65420;
        locals.var_vgb1__blk1304_dn5 = assign50590_e65420_d_n5;
        locals.var_vgb1__blk1304_dn6 = assign50590_e65420_d_n6;
        locals.var_vgb1__blk1304_dn7 = assign50590_e65420_d_n7;
        locals.var_vgb1__blk1304_dn8 = assign50590_e65420_d_n8;
        locals.var_vgb1__blk1304_rv = 0.0;

        let (assign50600_e65427, assign50600_e65427_d_n5, assign50600_e65427_d_n6, assign50600_e65427_d_n7, assign50600_e65427_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_vsbx_dc, locals.var_vsbx_dc_dn5, locals.var_vsbx_dc_dn6, locals.var_vsbx_dc_dn7, locals.var_vsbx_dc_dn8,)
    } else {
        (locals.var_vsbx__blk1306, locals.var_vsbx__blk1306_dn5, locals.var_vsbx__blk1306_dn6, locals.var_vsbx__blk1306_dn7, locals.var_vsbx__blk1306_dn8,)
    }
};
        locals.var_vsbx__blk1306 = assign50600_e65427;
        locals.var_vsbx__blk1306_dn5 = assign50600_e65427_d_n5;
        locals.var_vsbx__blk1306_dn6 = assign50600_e65427_d_n6;
        locals.var_vsbx__blk1306_dn7 = assign50600_e65427_d_n7;
        locals.var_vsbx__blk1306_dn8 = assign50600_e65427_d_n8;
        locals.var_vsbx__blk1306_rv = 0.0;

        let (assign50610_e65434, assign50610_e65434_d_n5, assign50610_e65434_d_n6, assign50610_e65434_d_n7, assign50610_e65434_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_phit1_dc, locals.var_phit1_dc_dn5, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8,)
    } else {
        (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8,)
    }
};
        locals.var_phit1__blk1322 = assign50610_e65434;
        locals.var_phit1__blk1322_dn5 = assign50610_e65434_d_n5;
        locals.var_phit1__blk1322_dn6 = assign50610_e65434_d_n6;
        locals.var_phit1__blk1322_dn7 = assign50610_e65434_d_n7;
        locals.var_phit1__blk1322_dn8 = assign50610_e65434_d_n8;
        locals.var_phit1__blk1322_rv = 0.0;

        let (assign50620_e65441, assign50620_e65441_d_n5, assign50620_e65441_d_n6, assign50620_e65441_d_n7, assign50620_e65441_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_inv_phit1_dc, locals.var_inv_phit1_dc_dn5, locals.var_inv_phit1_dc_dn6, locals.var_inv_phit1_dc_dn7, locals.var_inv_phit1_dc_dn8,)
    } else {
        (locals.var_inv_phit1__blk1323, locals.var_inv_phit1__blk1323_dn5, locals.var_inv_phit1__blk1323_dn6, locals.var_inv_phit1__blk1323_dn7, locals.var_inv_phit1__blk1323_dn8,)
    }
};
        locals.var_inv_phit1__blk1323 = assign50620_e65441;
        locals.var_inv_phit1__blk1323_dn5 = assign50620_e65441_d_n5;
        locals.var_inv_phit1__blk1323_dn6 = assign50620_e65441_d_n6;
        locals.var_inv_phit1__blk1323_dn7 = assign50620_e65441_d_n7;
        locals.var_inv_phit1__blk1323_dn8 = assign50620_e65441_d_n8;
        locals.var_inv_phit1__blk1323_rv = 0.0;

        let (assign50630_e65448, assign50630_e65448_d_n5, assign50630_e65448_d_n6, assign50630_e65448_d_n7, assign50630_e65448_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_gf_dc, locals.var_gf_dc_dn5, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8,)
    } else {
        (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8,)
    }
};
        locals.var_gf__blk1307 = assign50630_e65448;
        locals.var_gf__blk1307_dn5 = assign50630_e65448_d_n5;
        locals.var_gf__blk1307_dn6 = assign50630_e65448_d_n6;
        locals.var_gf__blk1307_dn7 = assign50630_e65448_d_n7;
        locals.var_gf__blk1307_dn8 = assign50630_e65448_d_n8;
        locals.var_gf__blk1307_rv = 0.0;

        let (assign50640_e65455, assign50640_e65455_d_n5, assign50640_e65455_d_n6, assign50640_e65455_d_n7, assign50640_e65455_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_gf2_dc, locals.var_gf2_dc_dn5, locals.var_gf2_dc_dn6, locals.var_gf2_dc_dn7, locals.var_gf2_dc_dn8,)
    } else {
        (locals.var_gf2__blk1308, locals.var_gf2__blk1308_dn5, locals.var_gf2__blk1308_dn6, locals.var_gf2__blk1308_dn7, locals.var_gf2__blk1308_dn8,)
    }
};
        locals.var_gf2__blk1308 = assign50640_e65455;
        locals.var_gf2__blk1308_dn5 = assign50640_e65455_d_n5;
        locals.var_gf2__blk1308_dn6 = assign50640_e65455_d_n6;
        locals.var_gf2__blk1308_dn7 = assign50640_e65455_d_n7;
        locals.var_gf2__blk1308_dn8 = assign50640_e65455_d_n8;
        locals.var_gf2__blk1308_rv = 0.0;

        let (assign50650_e65462, assign50650_e65462_d_n5, assign50650_e65462_d_n6, assign50650_e65462_d_n7, assign50650_e65462_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_inv_gf2_dc, locals.var_inv_gf2_dc_dn5, locals.var_inv_gf2_dc_dn6, locals.var_inv_gf2_dc_dn7, locals.var_inv_gf2_dc_dn8,)
    } else {
        (locals.var_inv_gf2__blk1324, locals.var_inv_gf2__blk1324_dn5, locals.var_inv_gf2__blk1324_dn6, locals.var_inv_gf2__blk1324_dn7, locals.var_inv_gf2__blk1324_dn8,)
    }
};
        locals.var_inv_gf2__blk1324 = assign50650_e65462;
        locals.var_inv_gf2__blk1324_dn5 = assign50650_e65462_d_n5;
        locals.var_inv_gf2__blk1324_dn6 = assign50650_e65462_d_n6;
        locals.var_inv_gf2__blk1324_dn7 = assign50650_e65462_d_n7;
        locals.var_inv_gf2__blk1324_dn8 = assign50650_e65462_d_n8;
        locals.var_inv_gf2__blk1324_rv = 0.0;

        let (assign50660_e65469, assign50660_e65469_d_n5, assign50660_e65469_d_n6, assign50660_e65469_d_n7, assign50660_e65469_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xg_dc, locals.var_xg_dc_dn5, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8,)
    } else {
        (locals.var_xg__blk1326, locals.var_xg__blk1326_dn5, locals.var_xg__blk1326_dn6, locals.var_xg__blk1326_dn7, locals.var_xg__blk1326_dn8,)
    }
};
        locals.var_xg__blk1326 = assign50660_e65469;
        locals.var_xg__blk1326_dn5 = assign50660_e65469_d_n5;
        locals.var_xg__blk1326_dn6 = assign50660_e65469_d_n6;
        locals.var_xg__blk1326_dn7 = assign50660_e65469_d_n7;
        locals.var_xg__blk1326_dn8 = assign50660_e65469_d_n8;
        locals.var_xg__blk1326_rv = 0.0;

        let (assign50670_e65476, assign50670_e65476_d_n5, assign50670_e65476_d_n6, assign50670_e65476_d_n7, assign50670_e65476_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xno_s_dc, locals.var_xno_s_dc_dn5, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8,)
    } else {
        (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8,)
    }
};
        locals.var_xno_s__blk1331 = assign50670_e65476;
        locals.var_xno_s__blk1331_dn5 = assign50670_e65476_d_n5;
        locals.var_xno_s__blk1331_dn6 = assign50670_e65476_d_n6;
        locals.var_xno_s__blk1331_dn7 = assign50670_e65476_d_n7;
        locals.var_xno_s__blk1331_dn8 = assign50670_e65476_d_n8;
        locals.var_xno_s__blk1331_rv = 0.0;

        let (assign50680_e65483, assign50680_e65483_d_n5, assign50680_e65483_d_n6, assign50680_e65483_d_n7, assign50680_e65483_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xn_s_dc, locals.var_xn_s_dc_dn5, locals.var_xn_s_dc_dn6, locals.var_xn_s_dc_dn7, locals.var_xn_s_dc_dn8,)
    } else {
        (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8,)
    }
};
        locals.var_xn_s__blk1332 = assign50680_e65483;
        locals.var_xn_s__blk1332_dn5 = assign50680_e65483_d_n5;
        locals.var_xn_s__blk1332_dn6 = assign50680_e65483_d_n6;
        locals.var_xn_s__blk1332_dn7 = assign50680_e65483_d_n7;
        locals.var_xn_s__blk1332_dn8 = assign50680_e65483_d_n8;
        locals.var_xn_s__blk1332_rv = 0.0;

        let (assign50690_e65490, assign50690_e65490_d_n5, assign50690_e65490_d_n6, assign50690_e65490_d_n7, assign50690_e65490_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xi_dc, locals.var_xi_dc_dn5, locals.var_xi_dc_dn6, locals.var_xi_dc_dn7, locals.var_xi_dc_dn8,)
    } else {
        (locals.var_xi__blk1343, locals.var_xi__blk1343_dn5, locals.var_xi__blk1343_dn6, locals.var_xi__blk1343_dn7, locals.var_xi__blk1343_dn8,)
    }
};
        locals.var_xi__blk1343 = assign50690_e65490;
        locals.var_xi__blk1343_dn5 = assign50690_e65490_d_n5;
        locals.var_xi__blk1343_dn6 = assign50690_e65490_d_n6;
        locals.var_xi__blk1343_dn7 = assign50690_e65490_d_n7;
        locals.var_xi__blk1343_dn8 = assign50690_e65490_d_n8;
        locals.var_xi__blk1343_rv = 0.0;

        let (assign50700_e65497, assign50700_e65497_d_n5, assign50700_e65497_d_n6, assign50700_e65497_d_n7, assign50700_e65497_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_margin_dc, locals.var_margin_dc_dn5, locals.var_margin_dc_dn6, locals.var_margin_dc_dn7, locals.var_margin_dc_dn8,)
    } else {
        (locals.var_margin__blk1344, locals.var_margin__blk1344_dn5, locals.var_margin__blk1344_dn6, locals.var_margin__blk1344_dn7, locals.var_margin__blk1344_dn8,)
    }
};
        locals.var_margin__blk1344 = assign50700_e65497;
        locals.var_margin__blk1344_dn5 = assign50700_e65497_d_n5;
        locals.var_margin__blk1344_dn6 = assign50700_e65497_d_n6;
        locals.var_margin__blk1344_dn7 = assign50700_e65497_d_n7;
        locals.var_margin__blk1344_dn8 = assign50700_e65497_d_n8;
        locals.var_margin__blk1344_rv = 0.0;

        let (assign50710_e65504, assign50710_e65504_d_n5, assign50710_e65504_d_n6, assign50710_e65504_d_n7, assign50710_e65504_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_inv_xi_dc, locals.var_inv_xi_dc_dn5, locals.var_inv_xi_dc_dn6, locals.var_inv_xi_dc_dn7, locals.var_inv_xi_dc_dn8,)
    } else {
        (locals.var_inv_xi__blk1345, locals.var_inv_xi__blk1345_dn5, locals.var_inv_xi__blk1345_dn6, locals.var_inv_xi__blk1345_dn7, locals.var_inv_xi__blk1345_dn8,)
    }
};
        locals.var_inv_xi__blk1345 = assign50710_e65504;
        locals.var_inv_xi__blk1345_dn5 = assign50710_e65504_d_n5;
        locals.var_inv_xi__blk1345_dn6 = assign50710_e65504_d_n6;
        locals.var_inv_xi__blk1345_dn7 = assign50710_e65504_d_n7;
        locals.var_inv_xi__blk1345_dn8 = assign50710_e65504_d_n8;
        locals.var_inv_xi__blk1345_rv = 0.0;

        let (assign50720_e65511, assign50720_e65511_d_n5, assign50720_e65511_d_n6, assign50720_e65511_d_n7, assign50720_e65511_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_sp_s_x1_dc, locals.var_sp_s_x1_dc_dn5, locals.var_sp_s_x1_dc_dn6, locals.var_sp_s_x1_dc_dn7, locals.var_sp_s_x1_dc_dn8,)
    } else {
        (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8,)
    }
};
        locals.var_sp_s_x1__blk1452 = assign50720_e65511;
        locals.var_sp_s_x1__blk1452_dn5 = assign50720_e65511_d_n5;
        locals.var_sp_s_x1__blk1452_dn6 = assign50720_e65511_d_n6;
        locals.var_sp_s_x1__blk1452_dn7 = assign50720_e65511_d_n7;
        locals.var_sp_s_x1__blk1452_dn8 = assign50720_e65511_d_n8;
        locals.var_sp_s_x1__blk1452_rv = 0.0;

        let (assign50730_e65518, assign50730_e65518_d_n5, assign50730_e65518_d_n6, assign50730_e65518_d_n7, assign50730_e65518_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_delta_ns_dc, locals.var_delta_ns_dc_dn5, locals.var_delta_ns_dc_dn6, locals.var_delta_ns_dc_dn7, locals.var_delta_ns_dc_dn8,)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign50730_e65518;
        locals.var_delta_ns__blk1347_dn5 = assign50730_e65518_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign50730_e65518_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign50730_e65518_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign50730_e65518_d_n8;
        locals.var_delta_ns__blk1347_rv = 0.0;

        let (assign50740_e65525, assign50740_e65525_d_n5, assign50740_e65525_d_n6, assign50740_e65525_d_n7, assign50740_e65525_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_x_s_dc, locals.var_x_s_dc_dn5, locals.var_x_s_dc_dn6, locals.var_x_s_dc_dn7, locals.var_x_s_dc_dn8,)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign50740_e65525;
        locals.var_x_s__blk1346_dn5 = assign50740_e65525_d_n5;
        locals.var_x_s__blk1346_dn6 = assign50740_e65525_d_n6;
        locals.var_x_s__blk1346_dn7 = assign50740_e65525_d_n7;
        locals.var_x_s__blk1346_dn8 = assign50740_e65525_d_n8;
        locals.var_x_s__blk1346_rv = 0.0;

        let (assign50750_e65532, assign50750_e65532_d_n5, assign50750_e65532_d_n6, assign50750_e65532_d_n7, assign50750_e65532_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xi1s_dc, locals.var_xi1s_dc_dn5, locals.var_xi1s_dc_dn6, locals.var_xi1s_dc_dn7, locals.var_xi1s_dc_dn8,)
    } else {
        (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8,)
    }
};
        locals.var_xi1s__blk1349 = assign50750_e65532;
        locals.var_xi1s__blk1349_dn5 = assign50750_e65532_d_n5;
        locals.var_xi1s__blk1349_dn6 = assign50750_e65532_d_n6;
        locals.var_xi1s__blk1349_dn7 = assign50750_e65532_d_n7;
        locals.var_xi1s__blk1349_dn8 = assign50750_e65532_d_n8;
        locals.var_xi1s__blk1349_rv = 0.0;

        let (assign50760_e65539, assign50760_e65539_d_n5, assign50760_e65539_d_n6, assign50760_e65539_d_n7, assign50760_e65539_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xi2s_dc, locals.var_xi2s_dc_dn5, locals.var_xi2s_dc_dn6, locals.var_xi2s_dc_dn7, locals.var_xi2s_dc_dn8,)
    } else {
        (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8,)
    }
};
        locals.var_xi2s__blk1350 = assign50760_e65539;
        locals.var_xi2s__blk1350_dn5 = assign50760_e65539_d_n5;
        locals.var_xi2s__blk1350_dn6 = assign50760_e65539_d_n6;
        locals.var_xi2s__blk1350_dn7 = assign50760_e65539_d_n7;
        locals.var_xi2s__blk1350_dn8 = assign50760_e65539_d_n8;
        locals.var_xi2s__blk1350_rv = 0.0;

        let (assign50770_e65546, assign50770_e65546_d_n5, assign50770_e65546_d_n6, assign50770_e65546_d_n7, assign50770_e65546_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_delta_1s_dc, locals.var_delta_1s_dc_dn5, locals.var_delta_1s_dc_dn6, locals.var_delta_1s_dc_dn7, locals.var_delta_1s_dc_dn8,)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50770_e65546;
        locals.var_delta_1s__blk1351_dn5 = assign50770_e65546_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50770_e65546_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50770_e65546_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50770_e65546_d_n8;
        locals.var_delta_1s__blk1351_rv = 0.0;

        let (assign50780_e65553, assign50780_e65553_d_n5, assign50780_e65553_d_n6, assign50780_e65553_d_n7, assign50780_e65553_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_es_dc, locals.var_es_dc_dn5, locals.var_es_dc_dn6, locals.var_es_dc_dn7, locals.var_es_dc_dn8,)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign50780_e65553;
        locals.var_es__blk1352_dn5 = assign50780_e65553_d_n5;
        locals.var_es__blk1352_dn6 = assign50780_e65553_d_n6;
        locals.var_es__blk1352_dn7 = assign50780_e65553_d_n7;
        locals.var_es__blk1352_dn8 = assign50780_e65553_d_n8;
        locals.var_es__blk1352_rv = 0.0;

        let (assign50790_e65560, assign50790_e65560_d_n5, assign50790_e65560_d_n6, assign50790_e65560_d_n7, assign50790_e65560_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_ps_dc, locals.var_ps_dc_dn5, locals.var_ps_dc_dn6, locals.var_ps_dc_dn7, locals.var_ps_dc_dn8,)
    } else {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    }
};
        locals.var_ps__blk1354 = assign50790_e65560;
        locals.var_ps__blk1354_dn5 = assign50790_e65560_d_n5;
        locals.var_ps__blk1354_dn6 = assign50790_e65560_d_n6;
        locals.var_ps__blk1354_dn7 = assign50790_e65560_d_n7;
        locals.var_ps__blk1354_dn8 = assign50790_e65560_d_n8;
        locals.var_ps__blk1354_rv = 0.0;

        let (assign50800_e65567, assign50800_e65567_d_n5, assign50800_e65567_d_n6, assign50800_e65567_d_n7, assign50800_e65567_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_ds_dc, locals.var_ds_dc_dn5, locals.var_ds_dc_dn6, locals.var_ds_dc_dn7, locals.var_ds_dc_dn8,)
    } else {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    }
};
        locals.var_ds__blk1353 = assign50800_e65567;
        locals.var_ds__blk1353_dn5 = assign50800_e65567_d_n5;
        locals.var_ds__blk1353_dn6 = assign50800_e65567_d_n6;
        locals.var_ds__blk1353_dn7 = assign50800_e65567_d_n7;
        locals.var_ds__blk1353_dn8 = assign50800_e65567_d_n8;
        locals.var_ds__blk1353_rv = 0.0;

        let (assign50810_e65574, assign50810_e65574_d_n5, assign50810_e65574_d_n6, assign50810_e65574_d_n7, assign50810_e65574_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_sqs_dc, locals.var_sqs_dc_dn5, locals.var_sqs_dc_dn6, locals.var_sqs_dc_dn7, locals.var_sqs_dc_dn8,)
    } else {
        (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8,)
    }
};
        locals.var_sqs__blk1355 = assign50810_e65574;
        locals.var_sqs__blk1355_dn5 = assign50810_e65574_d_n5;
        locals.var_sqs__blk1355_dn6 = assign50810_e65574_d_n6;
        locals.var_sqs__blk1355_dn7 = assign50810_e65574_d_n7;
        locals.var_sqs__blk1355_dn8 = assign50810_e65574_d_n8;
        locals.var_sqs__blk1355_rv = 0.0;

        let (assign50820_e65581, assign50820_e65581_d_n5, assign50820_e65581_d_n6, assign50820_e65581_d_n7, assign50820_e65581_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_alphas_dc, locals.var_alphas_dc_dn5, locals.var_alphas_dc_dn6, locals.var_alphas_dc_dn7, locals.var_alphas_dc_dn8,)
    } else {
        (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8,)
    }
};
        locals.var_alphas__blk1356 = assign50820_e65581;
        locals.var_alphas__blk1356_dn5 = assign50820_e65581_d_n5;
        locals.var_alphas__blk1356_dn6 = assign50820_e65581_d_n6;
        locals.var_alphas__blk1356_dn7 = assign50820_e65581_d_n7;
        locals.var_alphas__blk1356_dn8 = assign50820_e65581_d_n8;
        locals.var_alphas__blk1356_rv = 0.0;

        let (assign50830_e65588, assign50830_e65588_d_n5, assign50830_e65588_d_n6, assign50830_e65588_d_n7, assign50830_e65588_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_rxcor_dc, locals.var_rxcor_dc_dn5, locals.var_rxcor_dc_dn6, locals.var_rxcor_dc_dn7, locals.var_rxcor_dc_dn8,)
    } else {
        (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8,)
    }
};
        locals.var_rxcor__blk1357 = assign50830_e65588;
        locals.var_rxcor__blk1357_dn5 = assign50830_e65588_d_n5;
        locals.var_rxcor__blk1357_dn6 = assign50830_e65588_d_n6;
        locals.var_rxcor__blk1357_dn7 = assign50830_e65588_d_n7;
        locals.var_rxcor__blk1357_dn8 = assign50830_e65588_d_n8;
        locals.var_rxcor__blk1357_rv = 0.0;

    }
}

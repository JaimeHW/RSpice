#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7430_e7003: f64 = (1e-8 * locals.var_phit);
        let assign7430_e7005: f64 = (assign7430_e7003 / locals.var_tsi_i);
        locals.var_temp = assign7430_e7005;
        locals.var_temp_dn4 = ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i);
        locals.var_temp_dn6 = ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i);
        locals.var_temp_dn7 = ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i);
        locals.var_temp_dn8 = ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i);
        locals.var_temp_dn9 = ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i);

        let assign7440_e7008: f64 = (locals.var_temp * locals.var_mue_i);
        locals.var_fmue = assign7440_e7008;
        locals.var_fmue_dn4 = ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4));
        locals.var_fmue_dn6 = ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6));
        locals.var_fmue_dn7 = ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7));
        locals.var_fmue_dn8 = ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8));
        locals.var_fmue_dn9 = ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9));

        let assign7450_e7012: f64 = (0.5 * locals.var_csthr_i);
        let assign7450_e7013: f64 = (1.0 / assign7450_e7012);
        locals.var_inv_qi1cs = assign7450_e7013;

        let assign7460_e7016: f64 = (locals.var_inv_qi1cs / locals.var_csthrb_i);
        locals.var_inv_qi2cs = assign7460_e7016;

        let assign7470_e7019: f64 = 1.0;
        let assign7470_e7020: f64 = if p.p14 == assign7470_e7019 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7470_e7020;

        let (assign7480_e7026,) = {
    if (locals.var_guard147 != 0.0) {
        let assign7480_e7024: f64 = (0.5 * locals.var_feta_i);
        (assign7480_e7024,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign7480_e7026;

        let (assign7490_e7033,) = {
    if (locals.var_guard147 == 0.0) {
        let assign7490_e7031: f64 = (0.3333333333333 * locals.var_feta_i);
        (assign7490_e7031,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign7490_e7033;

        let assign7500_e7036: f64 = (1.0 - locals.var_eta_mu);
        locals.var_one_m_eta = assign7500_e7036;

        let assign7510_e7039: f64 = (locals.var_strs_i * locals.var_lnrtn);
        let assign7510_e7040: f64 = (assign7510_e7039).exp();
        locals.var_tf_ther = assign7510_e7040;
        locals.var_tf_ther_dn4 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn4));
        locals.var_tf_ther_dn6 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn6));
        locals.var_tf_ther_dn7 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn7));
        locals.var_tf_ther_dn8 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn8));
        locals.var_tf_ther_dn9 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn9));

        let assign7520_e7043: f64 = (locals.var_rs_t * locals.var_tf_ther);
        locals.var_rs_i = assign7520_e7043;
        locals.var_rs_i_dn4 = (locals.var_rs_t * locals.var_tf_ther_dn4);
        locals.var_rs_i_dn6 = (locals.var_rs_t * locals.var_tf_ther_dn6);
        locals.var_rs_i_dn7 = (locals.var_rs_t * locals.var_tf_ther_dn7);
        locals.var_rs_i_dn8 = (locals.var_rs_t * locals.var_tf_ther_dn8);
        locals.var_rs_i_dn9 = (locals.var_rs_t * locals.var_tf_ther_dn9);

        let assign7530_e7046: f64 = (2.0 * locals.var_rs_i);
        let assign7530_e7048: f64 = (assign7530_e7046 * locals.var_phit);
        locals.var_frs = assign7530_e7048;
        locals.var_frs_dn4 = (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn4));
        locals.var_frs_dn6 = (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn6));
        locals.var_frs_dn7 = (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn7));
        locals.var_frs_dn8 = (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn8));
        locals.var_frs_dn9 = (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn9));

        let assign7540_e7052: f64 = (16.0 / locals.var_ax_i);
        let assign7540_e7054: f64 = (assign7540_e7052 * 0.6931471805599);
        let assign7540_e7055: f64 = (assign7540_e7054).exp();
        let assign7540_e7057: f64 = (assign7540_e7055 - 1.0);
        let assign7540_e7058: f64 = (assign7540_e7057).ln();
        let assign7540_e7059: f64 = (0.375 * assign7540_e7058);
        let assign7540_e7060: f64 = (assign7540_e7059).exp();
        let assign7540_e7062: f64 = (assign7540_e7060 - 1.0);
        locals.var_gamax = assign7540_e7062;

        let assign7550_e7066: f64 = (16.0 / locals.var_axac_i);
        let assign7550_e7068: f64 = (assign7550_e7066 * 0.6931471805599);
        let assign7550_e7069: f64 = (assign7550_e7068).exp();
        let assign7550_e7071: f64 = (assign7550_e7069 - 1.0);
        let assign7550_e7072: f64 = (assign7550_e7071).ln();
        let assign7550_e7073: f64 = (0.375 * assign7550_e7072);
        let assign7550_e7074: f64 = (assign7550_e7073).exp();
        let assign7550_e7076: f64 = (assign7550_e7074 - 1.0);
        locals.var_gamax_ac = assign7550_e7076;

        let assign7560_e7079: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
        let assign7560_e7080: f64 = (assign7560_e7079).exp();
        locals.var_tf_thesat = assign7560_e7080;
        locals.var_tf_thesat_dn4 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn4));
        locals.var_tf_thesat_dn6 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn6));
        locals.var_tf_thesat_dn7 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn7));
        locals.var_tf_thesat_dn8 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn8));
        locals.var_tf_thesat_dn9 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn9));

        let assign7570_e7083: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
        let assign7570_e7085: f64 = (assign7570_e7083 * locals.var_tf_bet);
        locals.var_thesat_i = assign7570_e7085;
        locals.var_thesat_i_dn4 = ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn4));
        locals.var_thesat_i_dn6 = ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn6));
        locals.var_thesat_i_dn7 = ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn7));
        locals.var_thesat_i_dn8 = ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn8));
        locals.var_thesat_i_dn9 = ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn9));

        let assign7580_e7088: f64 = (locals.var_thesat_i * locals.var_phit);
        locals.var_sat_phit = assign7580_e7088;
        locals.var_sat_phit_dn4 = ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4));
        locals.var_sat_phit_dn6 = ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6));
        locals.var_sat_phit_dn7 = ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7));
        locals.var_sat_phit_dn8 = ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8));
        locals.var_sat_phit_dn9 = ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9));

        let assign7590_e7091: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
        let assign7590_e7093: f64 = (assign7590_e7091 * locals.var_tf_bet);
        locals.var_thesatac_i = assign7590_e7093;
        locals.var_thesatac_i_dn4 = ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn4));
        locals.var_thesatac_i_dn6 = ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn6));
        locals.var_thesatac_i_dn7 = ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn7));
        locals.var_thesatac_i_dn8 = ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn8));
        locals.var_thesatac_i_dn9 = ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn9));

        let assign7600_e7096: f64 = (locals.var_thesatac_i * locals.var_phit);
        locals.var_sat_phit_ac = assign7600_e7096;
        locals.var_sat_phit_ac_dn4 = ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4));
        locals.var_sat_phit_ac_dn6 = ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6));
        locals.var_sat_phit_ac_dn7 = ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7));
        locals.var_sat_phit_ac_dn8 = ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8));
        locals.var_sat_phit_ac_dn9 = ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9));

        let assign7610_e7099: f64 = (locals.var_alp1_i * locals.var_inv_phit);
        locals.var_alp1_phit = assign7610_e7099;
        locals.var_alp1_phit_dn4 = (locals.var_alp1_i * locals.var_inv_phit_dn4);
        locals.var_alp1_phit_dn6 = (locals.var_alp1_i * locals.var_inv_phit_dn6);
        locals.var_alp1_phit_dn7 = (locals.var_alp1_i * locals.var_inv_phit_dn7);
        locals.var_alp1_phit_dn8 = (locals.var_alp1_i * locals.var_inv_phit_dn8);
        locals.var_alp1_phit_dn9 = (locals.var_alp1_i * locals.var_inv_phit_dn9);

        let assign7620_e7101: f64 = (-locals.var_stig_i);
        let assign7620_e7103: f64 = (assign7620_e7101 * locals.var_lnrtn);
        let assign7620_e7104: f64 = (assign7620_e7103).exp();
        locals.var_tf_ig = assign7620_e7104;

        let assign7630_e7107: f64 = (locals.var_iginv_t * locals.var_tf_ig);
        locals.var_iginv_i = assign7630_e7107;

        let assign7640_e7110: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
        locals.var_igovinv_i = assign7640_e7110;

        let assign7650_e7113: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
        locals.var_igovinvd_i = assign7650_e7113;

        let assign7660_e7116: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
        locals.var_igovacc_i = assign7660_e7116;

        let assign7670_e7119: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
        locals.var_igovaccd_i = assign7670_e7119;

        let assign7680_e7121: f64 = (-locals.var_stigfn_i);
        let assign7680_e7123: f64 = (assign7680_e7121 * locals.var_lnrtn);
        let assign7680_e7124: f64 = (assign7680_e7123).exp();
        locals.var_tf_ig = assign7680_e7124;

        let assign7710_e7133: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign7710_e7133;

        let assign7720_e7136: f64 = (4.0 * 0.3333333333333);
        let assign7720_e7139: f64 = (2.0 * 1.602176565e-19);
        let assign7720_e7141: f64 = (assign7720_e7139 * 9.10938291e-31);
        let assign7720_e7143: f64 = (assign7720_e7141 * locals.var_chib_i);
        let assign7720_e7144: f64 = (assign7720_e7143).sqrt();
        let assign7720_e7145: f64 = (assign7720_e7136 * assign7720_e7144);
        let assign7720_e7147: f64 = (assign7720_e7145 / 1.054571726e-34);
        locals.var_tempm = assign7720_e7147;
        locals.var_tempm_dn4 = 0.0;
        locals.var_tempm_dn6 = 0.0;
        locals.var_tempm_dn7 = 0.0;
        locals.var_tempm_dn8 = 0.0;
        locals.var_tempm_dn9 = 0.0;

        let assign7730_e7150: f64 = (locals.var_tempm * locals.var_toxp_i);
        locals.var_bch = assign7730_e7150;
        locals.var_bch_dn4 = (locals.var_tempm_dn4 * locals.var_toxp_i);
        locals.var_bch_dn6 = (locals.var_tempm_dn6 * locals.var_toxp_i);
        locals.var_bch_dn7 = (locals.var_tempm_dn7 * locals.var_toxp_i);
        locals.var_bch_dn8 = (locals.var_tempm_dn8 * locals.var_toxp_i);
        locals.var_bch_dn9 = (locals.var_tempm_dn9 * locals.var_toxp_i);

        let assign7740_e7153: f64 = (locals.var_tempm * locals.var_toxp_i);
        locals.var_bov = assign7740_e7153;
        locals.var_bov_dn4 = (locals.var_tempm_dn4 * locals.var_toxp_i);
        locals.var_bov_dn6 = (locals.var_tempm_dn6 * locals.var_toxp_i);
        locals.var_bov_dn7 = (locals.var_tempm_dn7 * locals.var_toxp_i);
        locals.var_bov_dn8 = (locals.var_tempm_dn8 * locals.var_toxp_i);
        locals.var_bov_dn9 = (locals.var_tempm_dn9 * locals.var_toxp_i);

        locals.var_gcqch = 0.0;

        let assign7760_e7157: f64 = if locals.var_gc3ch_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign7760_e7157;

        let (assign7770_e7166,) = {
    if (locals.var_guard148 != 0.0) {
        let assign7770_e7160: f64 = (-0.495);
        let assign7770_e7162: f64 = (assign7770_e7160 * locals.var_gc2ch_i);
        let assign7770_e7164: f64 = (assign7770_e7162 / locals.var_gc3ch_i);
        (assign7770_e7164,)
    } else {
        (locals.var_gcqch,)
    }
};
        locals.var_gcqch = assign7770_e7166;

        locals.var_gcqovinv = 0.0;

        let assign7790_e7170: f64 = if locals.var_gc3ovinv_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7790_e7170;

        let (assign7800_e7179,) = {
    if (locals.var_guard149 != 0.0) {
        let assign7800_e7173: f64 = (-0.495);
        let assign7800_e7175: f64 = (assign7800_e7173 * locals.var_gc2ovinv_i);
        let assign7800_e7177: f64 = (assign7800_e7175 / locals.var_gc3ovinv_i);
        (assign7800_e7177,)
    } else {
        (locals.var_gcqovinv,)
    }
};
        locals.var_gcqovinv = assign7800_e7179;

        locals.var_gcqovacc = 0.0;

        let assign7820_e7183: f64 = if locals.var_gc3ovacc_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7820_e7183;

        let (assign7830_e7192,) = {
    if (locals.var_guard150 != 0.0) {
        let assign7830_e7186: f64 = (-0.495);
        let assign7830_e7188: f64 = (assign7830_e7186 * locals.var_gc2ovacc_i);
        let assign7830_e7190: f64 = (assign7830_e7188 / locals.var_gc3ovacc_i);
        (assign7830_e7190,)
    } else {
        (locals.var_gcqovacc,)
    }
};
        locals.var_gcqovacc = assign7830_e7192;

        let assign7840_e7195: f64 = (0.5 * locals.var_eg);
        locals.var_alpha_b = assign7840_e7195;
        locals.var_alpha_b_dn4 = (0.5 * locals.var_eg_dn4);
        locals.var_alpha_b_dn6 = (0.5 * locals.var_eg_dn6);
        locals.var_alpha_b_dn7 = (0.5 * locals.var_eg_dn7);
        locals.var_alpha_b_dn8 = (0.5 * locals.var_eg_dn8);
        locals.var_alpha_b_dn9 = (0.5 * locals.var_eg_dn9);

        let assign7850_e7198: f64 = (locals.var_gco_i * locals.var_phit);
        locals.var_dch = assign7850_e7198;
        locals.var_dch_dn4 = (locals.var_gco_i * locals.var_phit_dn4);
        locals.var_dch_dn6 = (locals.var_gco_i * locals.var_phit_dn6);
        locals.var_dch_dn7 = (locals.var_gco_i * locals.var_phit_dn7);
        locals.var_dch_dn8 = (locals.var_gco_i * locals.var_phit_dn8);
        locals.var_dch_dn9 = (locals.var_gco_i * locals.var_phit_dn9);

        let assign7860_e7201: f64 = (locals.var_gco_i * locals.var_phit0);
        locals.var_dov = assign7860_e7201;
        locals.var_dov_dn4 = (locals.var_gco_i * locals.var_phit0_dn4);
        locals.var_dov_dn6 = (locals.var_gco_i * locals.var_phit0_dn6);
        locals.var_dov_dn7 = (locals.var_gco_i * locals.var_phit0_dn7);
        locals.var_dov_dn8 = (locals.var_gco_i * locals.var_phit0_dn8);
        locals.var_dov_dn9 = (locals.var_gco_i * locals.var_phit0_dn9);

        let assign7870_e7206: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
        let assign7870_e7207: f64 = (1.0 + assign7870_e7206);
        let assign7870_e7208: f64 = (1.0 / assign7870_e7207);
        locals.var_n_iginv = assign7870_e7208;
        locals.var_n_iginv_dn4 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_dn6 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_dn7 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_dn8 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_dn9 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign7870_e7207 * assign7870_e7207)));

        let assign7880_e7212: f64 = (locals.var_toxp_i * locals.var_toxp_i);
        let assign7880_e7213: f64 = (4e-18 / assign7880_e7212);
        locals.var_temp = assign7880_e7213;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;

        let assign7890_e7216: f64 = (locals.var_agidl_i * locals.var_temp);
        locals.var_agidl_i = assign7890_e7216;

        let assign7900_e7219: f64 = (locals.var_agidld_i * locals.var_temp);
        locals.var_agidld_i = assign7900_e7219;

        let assign7910_e7222: f64 = (locals.var_toxp_i * 500000000.0);
        locals.var_temp = assign7910_e7222;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;

        let assign7920_e7227: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7228: f64 = (1.0 + assign7920_e7227);
        let assign7920_e7230: f64 = assign7920_e7228;
        let assign7920_e7234: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7235: f64 = (1.0 + assign7920_e7234);
        let assign7920_e7237: f64 = assign7920_e7235;
        let assign7920_e7241: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7242: f64 = (1.0 + assign7920_e7241);
        let assign7920_e7244: f64 = assign7920_e7242;
        let assign7920_e7245: f64 = (assign7920_e7237 * assign7920_e7244);
        let assign7920_e7247: f64 = (assign7920_e7245 + 0.01);
        let assign7920_e7248: f64 = (assign7920_e7247).sqrt();
        let assign7920_e7249: f64 = (assign7920_e7230 + assign7920_e7248);
        let assign7920_e7250: f64 = (0.5 * assign7920_e7249);
        locals.var_tempm = assign7920_e7250;
        locals.var_tempm_dn4 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_dn6 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_dn7 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_dn8 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_dn9 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign7920_e7248))));

        let assign7930_e7253: f64 = (locals.var_bgidl_t * locals.var_tempm);
        let assign7930_e7255: f64 = (assign7930_e7253 * locals.var_temp);
        locals.var_bgidl_i = assign7930_e7255;
        locals.var_bgidl_i_dn4 = (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn4));
        locals.var_bgidl_i_dn6 = (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn6));
        locals.var_bgidl_i_dn7 = (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn7));
        locals.var_bgidl_i_dn8 = (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn8));
        locals.var_bgidl_i_dn9 = (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn9));

        let assign7940_e7260: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7261: f64 = (1.0 + assign7940_e7260);
        let assign7940_e7263: f64 = assign7940_e7261;
        let assign7940_e7267: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7268: f64 = (1.0 + assign7940_e7267);
        let assign7940_e7270: f64 = assign7940_e7268;
        let assign7940_e7274: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7275: f64 = (1.0 + assign7940_e7274);
        let assign7940_e7277: f64 = assign7940_e7275;
        let assign7940_e7278: f64 = (assign7940_e7270 * assign7940_e7277);
        let assign7940_e7280: f64 = (assign7940_e7278 + 0.01);
        let assign7940_e7281: f64 = (assign7940_e7280).sqrt();
        let assign7940_e7282: f64 = (assign7940_e7263 + assign7940_e7281);
        let assign7940_e7283: f64 = (0.5 * assign7940_e7282);
        locals.var_tempm = assign7940_e7283;
        locals.var_tempm_dn4 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_dn6 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_dn7 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_dn8 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_dn9 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign7940_e7281))));

        let assign7950_e7286: f64 = (locals.var_bgidld_t * locals.var_tempm);
        let assign7950_e7288: f64 = (assign7950_e7286 * locals.var_temp);
        locals.var_bgidld_i = assign7950_e7288;
        locals.var_bgidld_i_dn4 = (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn4));
        locals.var_bgidld_i_dn6 = (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn6));
        locals.var_bgidld_i_dn7 = (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn7));
        locals.var_bgidld_i_dn8 = (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn8));
        locals.var_bgidld_i_dn9 = (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn9));

        let assign7960_e7291: f64 = (-locals.var_sta2_i);
        let assign7960_e7293: f64 = (assign7960_e7291 * locals.var_lnrtn);
        let assign7960_e7294: f64 = (assign7960_e7293).exp();
        let assign7960_e7295: f64 = (locals.var_a2_t * assign7960_e7294);
        locals.var_a2_i = assign7960_e7295;
        locals.var_a2_i_dn4 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn4)));
        locals.var_a2_i_dn6 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn6)));
        locals.var_a2_i_dn7 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn7)));
        locals.var_a2_i_dn8 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn8)));
        locals.var_a2_i_dn9 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn9)));

        let assign7970_e7300: f64 = (locals.var_ctedge_i * locals.var_rtn);
        let assign7970_e7301: f64 = (1.0 + assign7970_e7300);
        let assign7970_e7302: f64 = (locals.var_phit0 * assign7970_e7301);
        locals.var_phit_edge = assign7970_e7302;
        locals.var_phit_edge_dn4 = ((locals.var_phit0_dn4 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn4)));
        locals.var_phit_edge_dn6 = ((locals.var_phit0_dn6 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn6)));
        locals.var_phit_edge_dn7 = ((locals.var_phit0_dn7 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn7)));
        locals.var_phit_edge_dn8 = ((locals.var_phit0_dn8 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn8)));
        locals.var_phit_edge_dn9 = ((locals.var_phit0_dn9 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn9)));

        let assign7980_e7305: f64 = (1.0 / locals.var_phit_edge);
        locals.var_inv_phit_edge = assign7980_e7305;
        locals.var_inv_phit_edge_dn4 = (-(locals.var_phit_edge_dn4 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn6 = (-(locals.var_phit_edge_dn6 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn7 = (-(locals.var_phit_edge_dn7 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn8 = (-(locals.var_phit_edge_dn8 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn9 = (-(locals.var_phit_edge_dn9 / (locals.var_phit_edge * locals.var_phit_edge)));

        let assign7990_e7308: f64 = (2.0 * 1.602176565e-19);
        let assign7990_e7310: f64 = (assign7990_e7308 * locals.var_neff);
        let assign7990_e7312: f64 = (assign7990_e7310 * locals.var_epsch);
        let assign7990_e7314: f64 = (assign7990_e7312 * locals.var_inv_phit_edge);
        locals.var_a0_csisq_edge = assign7990_e7314;
        locals.var_a0_csisq_edge_dn4 = ((((assign7990_e7308 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn4));
        locals.var_a0_csisq_edge_dn6 = ((((assign7990_e7308 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn6));
        locals.var_a0_csisq_edge_dn7 = ((((assign7990_e7308 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn7));
        locals.var_a0_csisq_edge_dn8 = ((((assign7990_e7308 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn8));
        locals.var_a0_csisq_edge_dn9 = ((((assign7990_e7308 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn9));

        let assign8000_e7317: f64 = (p.p14 * locals.var_stvfbedge_i);
        let assign8000_e7319: f64 = (assign8000_e7317 * locals.var_dt);
        let assign8000_e7321: f64 = (assign8000_e7319 + locals.var_dvfbqm);
        locals.var_temp = assign8000_e7321;
        locals.var_temp_dn4 = (assign8000_e7317 * locals.var_dt_dn4);
        locals.var_temp_dn6 = (assign8000_e7317 * locals.var_dt_dn6);
        locals.var_temp_dn7 = (assign8000_e7317 * locals.var_dt_dn7);
        locals.var_temp_dn8 = (assign8000_e7317 * locals.var_dt_dn8);
        locals.var_temp_dn9 = (assign8000_e7317 * locals.var_dt_dn9);

        let assign8010_e7325: f64 = (locals.var_vfb1edge_t + locals.var_dvfbch);
        let assign8010_e7327: f64 = (assign8010_e7325 + locals.var_dvfb1nch);
        let assign8010_e7328: f64 = (p.p14 * assign8010_e7327);
        let assign8010_e7330: f64 = (assign8010_e7328 + locals.var_temp);
        let assign8010_e7332: f64 = (assign8010_e7330 + p.p34);
        let assign8010_e7334: f64 = (assign8010_e7332 - locals.var_dvfbpdep);
        locals.var_vfb1edge_i = assign8010_e7334;
        locals.var_vfb1edge_i_dn4 = (((p.p14 * ((locals.var_vfb1edge_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_dn4);
        locals.var_vfb1edge_i_dn6 = (((p.p14 * ((locals.var_vfb1edge_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_dn6);
        locals.var_vfb1edge_i_dn7 = (((p.p14 * ((locals.var_vfb1edge_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_dn7);
        locals.var_vfb1edge_i_dn8 = (((p.p14 * ((locals.var_vfb1edge_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_dn8);
        locals.var_vfb1edge_i_dn9 = (((p.p14 * ((locals.var_vfb1edge_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_dn9);

        let assign8020_e7338: f64 = (locals.var_vfb2edge_t + locals.var_dvfbch);
        let assign8020_e7340: f64 = (assign8020_e7338 + locals.var_dvfb2nch);
        let assign8020_e7341: f64 = (p.p14 * assign8020_e7340);
        let assign8020_e7343: f64 = (assign8020_e7341 + locals.var_temp);
        locals.var_vfb2edge_i = assign8020_e7343;
        locals.var_vfb2edge_i_dn4 = ((p.p14 * (locals.var_dvfbch_dn4 + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfb2edge_i_dn6 = ((p.p14 * (locals.var_dvfbch_dn6 + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfb2edge_i_dn7 = ((p.p14 * (locals.var_dvfbch_dn7 + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfb2edge_i_dn8 = ((p.p14 * (locals.var_dvfbch_dn8 + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfb2edge_i_dn9 = ((p.p14 * (locals.var_dvfbch_dn9 + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);

        let assign8030_e7346: f64 = (locals.var_stbetedge_i * locals.var_lnrtn);
        let assign8030_e7347: f64 = (assign8030_e7346).exp();
        let assign8030_e7349: f64 = (assign8030_e7347 * p.p35);
        locals.var_temp = assign8030_e7349;
        locals.var_temp_dn4 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn4)) * p.p35);
        locals.var_temp_dn6 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn6)) * p.p35);
        locals.var_temp_dn7 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn7)) * p.p35);
        locals.var_temp_dn8 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn8)) * p.p35);
        locals.var_temp_dn9 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn9)) * p.p35);

        let assign8040_e7352: f64 = (locals.var_betnedge_t * locals.var_temp);
        locals.var_betnedge_i = assign8040_e7352;
        locals.var_betnedge_i_dn4 = ((locals.var_betnedge_t_dn4 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn4));
        locals.var_betnedge_i_dn6 = ((locals.var_betnedge_t_dn6 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn6));
        locals.var_betnedge_i_dn7 = ((locals.var_betnedge_t_dn7 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn7));
        locals.var_betnedge_i_dn8 = ((locals.var_betnedge_t_dn8 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn8));
        locals.var_betnedge_i_dn9 = ((locals.var_betnedge_t_dn9 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn9));

        let assign8050_e7355: f64 = (locals.var_areaq_i * locals.var_phit);
        locals.var_area_phit = assign8050_e7355;
        locals.var_area_phit_dn4 = (locals.var_areaq_i * locals.var_phit_dn4);
        locals.var_area_phit_dn6 = (locals.var_areaq_i * locals.var_phit_dn6);
        locals.var_area_phit_dn7 = (locals.var_areaq_i * locals.var_phit_dn7);
        locals.var_area_phit_dn8 = (locals.var_areaq_i * locals.var_phit_dn8);
        locals.var_area_phit_dn9 = (locals.var_areaq_i * locals.var_phit_dn9);

        let assign8060_e7358: f64 = (0.25 * 1.602176565e-19);
        let assign8060_e7360: f64 = (assign8060_e7358 * locals.var_nsdac_i);
        let assign8060_e7363: f64 = (locals.var_epsch * locals.var_phit);
        let assign8060_e7364: f64 = (assign8060_e7360 / assign8060_e7363);
        locals.var_inner_sd = assign8060_e7364;
        locals.var_inner_sd_dn4 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn4)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_dn6 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn6)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_dn7 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn7)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_dn8 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn8)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_dn9 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn9)) / (assign8060_e7363 * assign8060_e7363)));

        let assign8070_e7367: f64 = (locals.var_nsdac_i / locals.var_neff);
        let assign8070_e7368: f64 = (assign8070_e7367).ln();
        locals.var_xsd = assign8070_e7368;
        locals.var_xsd_dn4 = ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_dn6 = ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_dn7 = ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_dn8 = ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_dn9 = ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);

        let assign8080_e7371: f64 = (locals.var_fif_i * 1.25e-6);
        let assign8080_e7373: f64 = (assign8080_e7371 * locals.var_phit);
        locals.var_fif_phit = assign8080_e7373;
        locals.var_fif_phit_dn4 = (assign8080_e7371 * locals.var_phit_dn4);
        locals.var_fif_phit_dn6 = (assign8080_e7371 * locals.var_phit_dn6);
        locals.var_fif_phit_dn7 = (assign8080_e7371 * locals.var_phit_dn7);
        locals.var_fif_phit_dn8 = (assign8080_e7371 * locals.var_phit_dn8);
        locals.var_fif_phit_dn9 = (assign8080_e7371 * locals.var_phit_dn9);

        let assign8090_e7376: f64 = (locals.var_epsch / 3.45313e-11);
        let assign8090_e7378: f64 = (assign8090_e7376 * locals.var_tsi_i);
        let assign8090_e7381: f64 = (locals.var_tox1_i + 4e-10);
        let assign8090_e7382: f64 = (assign8090_e7378 * assign8090_e7381);
        let assign8090_e7383: f64 = (assign8090_e7382).sqrt();
        locals.var_lambda2d = assign8090_e7383;

    }

    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign8100_e7386: f64 = (locals.var_strth_i * locals.var_lnrtn);
        let assign8100_e7387: f64 = (assign8100_e7386).exp();
        locals.var_tf_rth = assign8100_e7387;
        locals.var_tf_rth_dn4 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn4));
        locals.var_tf_rth_dn6 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn6));
        locals.var_tf_rth_dn7 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn7));
        locals.var_tf_rth_dn8 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn8));
        locals.var_tf_rth_dn9 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn9));

        let assign8110_e7390: f64 = (locals.var_rth_t * locals.var_tf_rth);
        locals.var_rth_i = assign8110_e7390;
        locals.var_rth_i_dn4 = ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4));
        locals.var_rth_i_dn6 = ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6));
        locals.var_rth_i_dn7 = ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7));
        locals.var_rth_i_dn8 = ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8));
        locals.var_rth_i_dn9 = ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9));

        let assign8120_e7393: f64 = (4.0 * 1.3806488e-23);
        let assign8120_e7395: f64 = (assign8120_e7393 * locals.var_tkc);
        locals.var_nt0_4kt = assign8120_e7395;

        let assign8130_e7398: f64 = (locals.var_fnt_i * locals.var_nt0_4kt);
        locals.var_nt = assign8130_e7398;

        locals.var_nt0 = locals.var_nt;

        let assign8150_e7402: f64 = (9.10938291e-31 * 1000000000000.0);
        let assign8150_e7404: f64 = (assign8150_e7402 * locals.var_fntexc_i);
        locals.var_fac_exc = assign8150_e7404;

        let assign8280_e7463: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign8280_e7463;

        let (assign8290_e7467, assign8290_e7467_d_n4,) = {
    if (locals.var_guard257 != 0.0) {
        ((nv4 - 0.0), 1.0,)
    } else {
        (locals.var_dtc, locals.var_dtc_dn4,)
    }
};
        locals.var_dtc = assign8290_e7467;
        locals.var_dtc_dn4 = assign8290_e7467_d_n4;

        let (assign8300_e7473, assign8300_e7473_d_n4, assign8300_e7473_d_n6, assign8300_e7473_d_n7, assign8300_e7473_d_n8, assign8300_e7473_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8300_e7471: f64 = (locals.var_tkd + locals.var_dtc);
        (assign8300_e7471, (locals.var_tkd_dn4 + locals.var_dtc_dn4), locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    } else {
        (locals.var_tkc, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9,)
    }
};
        locals.var_tkc = assign8300_e7473;
        locals.var_tkc_dn4 = assign8300_e7473_d_n4;
        locals.var_tkc_dn6 = assign8300_e7473_d_n6;
        locals.var_tkc_dn7 = assign8300_e7473_d_n7;
        locals.var_tkc_dn8 = assign8300_e7473_d_n8;
        locals.var_tkc_dn9 = assign8300_e7473_d_n9;

        let (assign8310_e7479, assign8310_e7479_d_n4, assign8310_e7479_d_n6, assign8310_e7479_d_n7, assign8310_e7479_d_n8, assign8310_e7479_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8310_e7477: f64 = (locals.var_tkc * locals.var_tkc);
        (assign8310_e7477, ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4)), ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6)), ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7)), ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8)), ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9)),)
    } else {
        (locals.var_tkc_sq, locals.var_tkc_sq_dn4, locals.var_tkc_sq_dn6, locals.var_tkc_sq_dn7, locals.var_tkc_sq_dn8, locals.var_tkc_sq_dn9,)
    }
};
        locals.var_tkc_sq = assign8310_e7479;
        locals.var_tkc_sq_dn4 = assign8310_e7479_d_n4;
        locals.var_tkc_sq_dn6 = assign8310_e7479_d_n6;
        locals.var_tkc_sq_dn7 = assign8310_e7479_d_n7;
        locals.var_tkc_sq_dn8 = assign8310_e7479_d_n8;
        locals.var_tkc_sq_dn9 = assign8310_e7479_d_n9;

        let (assign8320_e7485, assign8320_e7485_d_n4, assign8320_e7485_d_n6, assign8320_e7485_d_n7, assign8320_e7485_d_n8, assign8320_e7485_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8320_e7483: f64 = (locals.var_tkc - locals.var_tkr);
        (assign8320_e7483, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9,)
    } else {
        (locals.var_dt, locals.var_dt_dn4, locals.var_dt_dn6, locals.var_dt_dn7, locals.var_dt_dn8, locals.var_dt_dn9,)
    }
};
        locals.var_dt = assign8320_e7485;
        locals.var_dt_dn4 = assign8320_e7485_d_n4;
        locals.var_dt_dn6 = assign8320_e7485_d_n6;
        locals.var_dt_dn7 = assign8320_e7485_d_n7;
        locals.var_dt_dn8 = assign8320_e7485_d_n8;
        locals.var_dt_dn9 = assign8320_e7485_d_n9;

        let (assign8330_e7491, assign8330_e7491_d_n4, assign8330_e7491_d_n6, assign8330_e7491_d_n7, assign8330_e7491_d_n8, assign8330_e7491_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8330_e7489: f64 = (locals.var_tkc / locals.var_tkr);
        (assign8330_e7489, (locals.var_tkc_dn4 / locals.var_tkr), (locals.var_tkc_dn6 / locals.var_tkr), (locals.var_tkc_dn7 / locals.var_tkr), (locals.var_tkc_dn8 / locals.var_tkr), (locals.var_tkc_dn9 / locals.var_tkr),)
    } else {
        (locals.var_rt, locals.var_rt_dn4, locals.var_rt_dn6, locals.var_rt_dn7, locals.var_rt_dn8, locals.var_rt_dn9,)
    }
};
        locals.var_rt = assign8330_e7491;
        locals.var_rt_dn4 = assign8330_e7491_d_n4;
        locals.var_rt_dn6 = assign8330_e7491_d_n6;
        locals.var_rt_dn7 = assign8330_e7491_d_n7;
        locals.var_rt_dn8 = assign8330_e7491_d_n8;
        locals.var_rt_dn9 = assign8330_e7491_d_n9;

        let (assign8340_e7497, assign8340_e7497_d_n4, assign8340_e7497_d_n6, assign8340_e7497_d_n7, assign8340_e7497_d_n8, assign8340_e7497_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8340_e7495: f64 = (locals.var_tkr / locals.var_tkc);
        (assign8340_e7495, (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc))),)
    } else {
        (locals.var_rtn, locals.var_rtn_dn4, locals.var_rtn_dn6, locals.var_rtn_dn7, locals.var_rtn_dn8, locals.var_rtn_dn9,)
    }
};
        locals.var_rtn = assign8340_e7497;
        locals.var_rtn_dn4 = assign8340_e7497_d_n4;
        locals.var_rtn_dn6 = assign8340_e7497_d_n6;
        locals.var_rtn_dn7 = assign8340_e7497_d_n7;
        locals.var_rtn_dn8 = assign8340_e7497_d_n8;
        locals.var_rtn_dn9 = assign8340_e7497_d_n9;

        let (assign8350_e7503, assign8350_e7503_d_n4, assign8350_e7503_d_n6, assign8350_e7503_d_n7, assign8350_e7503_d_n8, assign8350_e7503_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8350_e7501: f64 = (locals.var_tkc * 8.617332384961e-5);
        (assign8350_e7501, (locals.var_tkc_dn4 * 8.617332384961e-5), (locals.var_tkc_dn6 * 8.617332384961e-5), (locals.var_tkc_dn7 * 8.617332384961e-5), (locals.var_tkc_dn8 * 8.617332384961e-5), (locals.var_tkc_dn9 * 8.617332384961e-5),)
    } else {
        (locals.var_phit0, locals.var_phit0_dn4, locals.var_phit0_dn6, locals.var_phit0_dn7, locals.var_phit0_dn8, locals.var_phit0_dn9,)
    }
};
        locals.var_phit0 = assign8350_e7503;
        locals.var_phit0_dn4 = assign8350_e7503_d_n4;
        locals.var_phit0_dn6 = assign8350_e7503_d_n6;
        locals.var_phit0_dn7 = assign8350_e7503_d_n7;
        locals.var_phit0_dn8 = assign8350_e7503_d_n8;
        locals.var_phit0_dn9 = assign8350_e7503_d_n9;

        let (assign8360_e7509, assign8360_e7509_d_n4, assign8360_e7509_d_n6, assign8360_e7509_d_n7, assign8360_e7509_d_n8, assign8360_e7509_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8360_e7507: f64 = (1.0 / locals.var_phit0);
        (assign8360_e7507, (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0))),)
    } else {
        (locals.var_inv_phit0, locals.var_inv_phit0_dn4, locals.var_inv_phit0_dn6, locals.var_inv_phit0_dn7, locals.var_inv_phit0_dn8, locals.var_inv_phit0_dn9,)
    }
};
        locals.var_inv_phit0 = assign8360_e7509;
        locals.var_inv_phit0_dn4 = assign8360_e7509_d_n4;
        locals.var_inv_phit0_dn6 = assign8360_e7509_d_n6;
        locals.var_inv_phit0_dn7 = assign8360_e7509_d_n7;
        locals.var_inv_phit0_dn8 = assign8360_e7509_d_n8;
        locals.var_inv_phit0_dn9 = assign8360_e7509_d_n9;

        let assign8370_e7512: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign8370_e7512;

        let (assign8380_e7545, assign8380_e7545_d_n4, assign8380_e7545_d_n6, assign8380_e7545_d_n7, assign8380_e7545_d_n8, assign8380_e7545_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign8380_e7520: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8380_e7521: f64 = (10.0 / assign8380_e7520);
        let assign8380_e7523: f64 = (assign8380_e7521 + 600.0);
        let assign8380_e7527: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8380_e7528: f64 = (10.0 / assign8380_e7527);
        let assign8380_e7530: f64 = (assign8380_e7528 - 600.0);
        let assign8380_e7534: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8380_e7535: f64 = (10.0 / assign8380_e7534);
        let assign8380_e7537: f64 = (assign8380_e7535 - 600.0);
        let assign8380_e7538: f64 = (assign8380_e7530 * assign8380_e7537);
        let assign8380_e7540: f64 = (assign8380_e7538 + 0.01);
        let assign8380_e7541: f64 = (assign8380_e7540).sqrt();
        let assign8380_e7542: f64 = (assign8380_e7523 + assign8380_e7541);
        let assign8380_e7543: f64 = (0.5 * assign8380_e7542);
        (assign8380_e7543, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))),)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign8380_e7545;
        locals.var_xsatmax_dn4 = assign8380_e7545_d_n4;
        locals.var_xsatmax_dn6 = assign8380_e7545_d_n6;
        locals.var_xsatmax_dn7 = assign8380_e7545_d_n7;
        locals.var_xsatmax_dn8 = assign8380_e7545_d_n8;
        locals.var_xsatmax_dn9 = assign8380_e7545_d_n9;

        let (assign8390_e7552, assign8390_e7552_d_n4, assign8390_e7552_d_n6, assign8390_e7552_d_n7, assign8390_e7552_d_n8, assign8390_e7552_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 == 0.0)) {
        (600.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign8390_e7552;
        locals.var_xsatmax_dn4 = assign8390_e7552_d_n4;
        locals.var_xsatmax_dn6 = assign8390_e7552_d_n6;
        locals.var_xsatmax_dn7 = assign8390_e7552_d_n7;
        locals.var_xsatmax_dn8 = assign8390_e7552_d_n8;
        locals.var_xsatmax_dn9 = assign8390_e7552_d_n9;

        let (assign8400_e7564, assign8400_e7564_d_n4, assign8400_e7564_d_n6, assign8400_e7564_d_n7, assign8400_e7564_d_n8, assign8400_e7564_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8400_e7557: f64 = (0.000473 * locals.var_tkc_sq);
        let assign8400_e7560: f64 = (636.0 + locals.var_tkc);
        let assign8400_e7561: f64 = (assign8400_e7557 / assign8400_e7560);
        let assign8400_e7562: f64 = (1.17 - assign8400_e7561);
        (assign8400_e7562, (-((((0.000473 * locals.var_tkc_sq_dn4) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn4)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn6) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn6)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn7) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn7)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn8) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn8)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn9) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn9)) / (assign8400_e7560 * assign8400_e7560))),)
    } else {
        (locals.var_egsi, locals.var_egsi_dn4, locals.var_egsi_dn6, locals.var_egsi_dn7, locals.var_egsi_dn8, locals.var_egsi_dn9,)
    }
};
        locals.var_egsi = assign8400_e7564;
        locals.var_egsi_dn4 = assign8400_e7564_d_n4;
        locals.var_egsi_dn6 = assign8400_e7564_d_n6;
        locals.var_egsi_dn7 = assign8400_e7564_d_n7;
        locals.var_egsi_dn8 = assign8400_e7564_d_n8;
        locals.var_egsi_dn9 = assign8400_e7564_d_n9;

        let (assign8410_e7576, assign8410_e7576_d_n4, assign8410_e7576_d_n6, assign8410_e7576_d_n7, assign8410_e7576_d_n8, assign8410_e7576_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8410_e7569: f64 = (0.0004774 * locals.var_tkc_sq);
        let assign8410_e7572: f64 = (235.0 + locals.var_tkc);
        let assign8410_e7573: f64 = (assign8410_e7569 / assign8410_e7572);
        let assign8410_e7574: f64 = (0.744 - assign8410_e7573);
        (assign8410_e7574, (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn4)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn6)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn7)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn8)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn9)) / (assign8410_e7572 * assign8410_e7572))),)
    } else {
        (locals.var_egge, locals.var_egge_dn4, locals.var_egge_dn6, locals.var_egge_dn7, locals.var_egge_dn8, locals.var_egge_dn9,)
    }
};
        locals.var_egge = assign8410_e7576;
        locals.var_egge_dn4 = assign8410_e7576_d_n4;
        locals.var_egge_dn6 = assign8410_e7576_d_n6;
        locals.var_egge_dn7 = assign8410_e7576_d_n7;
        locals.var_egge_dn8 = assign8410_e7576_d_n8;
        locals.var_egge_dn9 = assign8410_e7576_d_n9;

        let (assign8420_e7589, assign8420_e7589_d_n4, assign8420_e7589_d_n6, assign8420_e7589_d_n7, assign8420_e7589_d_n8, assign8420_e7589_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8420_e7580: f64 = (locals.var_egge - locals.var_egsi);
        let assign8420_e7582: f64 = (-0.4);
        let assign8420_e7584: f64 = (assign8420_e7582 * locals.var_one_m_xge);
        let assign8420_e7585: f64 = (assign8420_e7580 + assign8420_e7584);
        let assign8420_e7587: f64 = (assign8420_e7585 * locals.var_xge_i);
        (assign8420_e7587, ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i), ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i), ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i), ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i), ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i),)
    } else {
        (locals.var_deg, locals.var_deg_dn4, locals.var_deg_dn6, locals.var_deg_dn7, locals.var_deg_dn8, locals.var_deg_dn9,)
    }
};
        locals.var_deg = assign8420_e7589;
        locals.var_deg_dn4 = assign8420_e7589_d_n4;
        locals.var_deg_dn6 = assign8420_e7589_d_n6;
        locals.var_deg_dn7 = assign8420_e7589_d_n7;
        locals.var_deg_dn8 = assign8420_e7589_d_n8;
        locals.var_deg_dn9 = assign8420_e7589_d_n9;

        let (assign8430_e7595, assign8430_e7595_d_n4, assign8430_e7595_d_n6, assign8430_e7595_d_n7, assign8430_e7595_d_n8, assign8430_e7595_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8430_e7593: f64 = (locals.var_egsi + locals.var_deg);
        (assign8430_e7593, (locals.var_egsi_dn4 + locals.var_deg_dn4), (locals.var_egsi_dn6 + locals.var_deg_dn6), (locals.var_egsi_dn7 + locals.var_deg_dn7), (locals.var_egsi_dn8 + locals.var_deg_dn8), (locals.var_egsi_dn9 + locals.var_deg_dn9),)
    } else {
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9,)
    }
};
        locals.var_eg = assign8430_e7595;
        locals.var_eg_dn4 = assign8430_e7595_d_n4;
        locals.var_eg_dn6 = assign8430_e7595_d_n6;
        locals.var_eg_dn7 = assign8430_e7595_d_n7;
        locals.var_eg_dn8 = assign8430_e7595_d_n8;
        locals.var_eg_dn9 = assign8430_e7595_d_n9;

        let (assign8440_e7603, assign8440_e7603_d_n4, assign8440_e7603_d_n6, assign8440_e7603_d_n7, assign8440_e7603_d_n8, assign8440_e7603_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8440_e7599: f64 = (0.5 * locals.var_eg);
        let assign8440_e7601: f64 = (assign8440_e7599 * locals.var_inv_phit0);
        (assign8440_e7601, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn9)),)
    } else {
        (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9,)
    }
};
        locals.var_eg_2phit0 = assign8440_e7603;
        locals.var_eg_2phit0_dn4 = assign8440_e7603_d_n4;
        locals.var_eg_2phit0_dn6 = assign8440_e7603_d_n6;
        locals.var_eg_2phit0_dn7 = assign8440_e7603_d_n7;
        locals.var_eg_2phit0_dn8 = assign8440_e7603_d_n8;
        locals.var_eg_2phit0_dn9 = assign8440_e7603_d_n9;

        let (assign8450_e7613, assign8450_e7613_d_n4, assign8450_e7613_d_n6, assign8450_e7613_d_n7, assign8450_e7613_d_n8, assign8450_e7613_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8450_e7607: f64 = (0.05 * locals.var_xge_i);
        let assign8450_e7610: f64 = (0.5 * locals.var_deg);
        let assign8450_e7611: f64 = (assign8450_e7607 - assign8450_e7610);
        (assign8450_e7611, (-(0.5 * locals.var_deg_dn4)), (-(0.5 * locals.var_deg_dn6)), (-(0.5 * locals.var_deg_dn7)), (-(0.5 * locals.var_deg_dn8)), (-(0.5 * locals.var_deg_dn9)),)
    } else {
        (locals.var_dvfbch, locals.var_dvfbch_dn4, locals.var_dvfbch_dn6, locals.var_dvfbch_dn7, locals.var_dvfbch_dn8, locals.var_dvfbch_dn9,)
    }
};
        locals.var_dvfbch = assign8450_e7613;
        locals.var_dvfbch_dn4 = assign8450_e7613_d_n4;
        locals.var_dvfbch_dn6 = assign8450_e7613_d_n6;
        locals.var_dvfbch_dn7 = assign8450_e7613_d_n7;
        locals.var_dvfbch_dn8 = assign8450_e7613_d_n8;
        locals.var_dvfbch_dn9 = assign8450_e7613_d_n9;

        let (assign8460_e7620, assign8460_e7620_d_n4, assign8460_e7620_d_n6, assign8460_e7620_d_n7, assign8460_e7620_d_n8, assign8460_e7620_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8460_e7617: f64 = (locals.var_tkc * 0.0033333333333);
        let assign8460_e7618: f64 = (assign8460_e7617).sqrt();
        (assign8460_e7618, ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign8460_e7618)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8460_e7620;
        locals.var_temp_dn4 = assign8460_e7620_d_n4;
        locals.var_temp_dn6 = assign8460_e7620_d_n6;
        locals.var_temp_dn7 = assign8460_e7620_d_n7;
        locals.var_temp_dn8 = assign8460_e7620_d_n8;
        locals.var_temp_dn9 = assign8460_e7620_d_n9;

        let (assign8470_e7630, assign8470_e7630_d_n4, assign8470_e7630_d_n6, assign8470_e7630_d_n7, assign8470_e7630_d_n8, assign8470_e7630_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8470_e7624: f64 = (4.05e25 * locals.var_temp);
        let assign8470_e7626: f64 = (assign8470_e7624 * locals.var_temp);
        let assign8470_e7628: f64 = (assign8470_e7626 * locals.var_temp);
        (assign8470_e7628, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn4)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn6)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn7)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn8)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn9)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign8470_e7630;
        locals.var_temp1_dn4 = assign8470_e7630_d_n4;
        locals.var_temp1_dn6 = assign8470_e7630_d_n6;
        locals.var_temp1_dn7 = assign8470_e7630_d_n7;
        locals.var_temp1_dn8 = assign8470_e7630_d_n8;
        locals.var_temp1_dn9 = assign8470_e7630_d_n9;

        let (assign8480_e7636, assign8480_e7636_d_n4, assign8480_e7636_d_n6, assign8480_e7636_d_n7, assign8480_e7636_d_n8, assign8480_e7636_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8480_e7634: f64 = (locals.var_temp1 * locals.var_niratio);
        (assign8480_e7634, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio),)
    } else {
        (locals.var_neff, locals.var_neff_dn4, locals.var_neff_dn6, locals.var_neff_dn7, locals.var_neff_dn8, locals.var_neff_dn9,)
    }
};
        locals.var_neff = assign8480_e7636;
        locals.var_neff_dn4 = assign8480_e7636_d_n4;
        locals.var_neff_dn6 = assign8480_e7636_d_n6;
        locals.var_neff_dn7 = assign8480_e7636_d_n7;
        locals.var_neff_dn8 = assign8480_e7636_d_n8;
        locals.var_neff_dn9 = assign8480_e7636_d_n9;

        let (assign8490_e7646, assign8490_e7646_d_n4, assign8490_e7646_d_n6, assign8490_e7646_d_n7, assign8490_e7646_d_n8, assign8490_e7646_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8490_e7642: f64 = (locals.var_ct_i * locals.var_rtn);
        let assign8490_e7643: f64 = (1.0 + assign8490_e7642);
        let assign8490_e7644: f64 = (locals.var_phit0 * assign8490_e7643);
        (assign8490_e7644, ((locals.var_phit0_dn4 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9))),)
    } else {
        (locals.var_phit, locals.var_phit_dn4, locals.var_phit_dn6, locals.var_phit_dn7, locals.var_phit_dn8, locals.var_phit_dn9,)
    }
};
        locals.var_phit = assign8490_e7646;
        locals.var_phit_dn4 = assign8490_e7646_d_n4;
        locals.var_phit_dn6 = assign8490_e7646_d_n6;
        locals.var_phit_dn7 = assign8490_e7646_d_n7;
        locals.var_phit_dn8 = assign8490_e7646_d_n8;
        locals.var_phit_dn9 = assign8490_e7646_d_n9;

        let (assign8500_e7652, assign8500_e7652_d_n4, assign8500_e7652_d_n6, assign8500_e7652_d_n7, assign8500_e7652_d_n8, assign8500_e7652_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8500_e7650: f64 = (1.0 / locals.var_phit);
        (assign8500_e7650, (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit))),)
    } else {
        (locals.var_inv_phit, locals.var_inv_phit_dn4, locals.var_inv_phit_dn6, locals.var_inv_phit_dn7, locals.var_inv_phit_dn8, locals.var_inv_phit_dn9,)
    }
};
        locals.var_inv_phit = assign8500_e7652;
        locals.var_inv_phit_dn4 = assign8500_e7652_d_n4;
        locals.var_inv_phit_dn6 = assign8500_e7652_d_n6;
        locals.var_inv_phit_dn7 = assign8500_e7652_d_n7;
        locals.var_inv_phit_dn8 = assign8500_e7652_d_n8;
        locals.var_inv_phit_dn9 = assign8500_e7652_d_n9;

        let (assign8510_e7660, assign8510_e7660_d_n4, assign8510_e7660_d_n6, assign8510_e7660_d_n7, assign8510_e7660_d_n8, assign8510_e7660_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8510_e7656: f64 = (0.5 * locals.var_eg);
        let assign8510_e7658: f64 = (assign8510_e7656 * locals.var_inv_phit);
        (assign8510_e7658, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_eg_2phit, locals.var_eg_2phit_dn4, locals.var_eg_2phit_dn6, locals.var_eg_2phit_dn7, locals.var_eg_2phit_dn8, locals.var_eg_2phit_dn9,)
    }
};
        locals.var_eg_2phit = assign8510_e7660;
        locals.var_eg_2phit_dn4 = assign8510_e7660_d_n4;
        locals.var_eg_2phit_dn6 = assign8510_e7660_d_n6;
        locals.var_eg_2phit_dn7 = assign8510_e7660_d_n7;
        locals.var_eg_2phit_dn8 = assign8510_e7660_d_n8;
        locals.var_eg_2phit_dn9 = assign8510_e7660_d_n9;

        let (assign8520_e7672, assign8520_e7672_d_n4, assign8520_e7672_d_n6, assign8520_e7672_d_n7, assign8520_e7672_d_n8, assign8520_e7672_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8520_e7664: f64 = (2.0 * 1.602176565e-19);
        let assign8520_e7666: f64 = (assign8520_e7664 * locals.var_neff);
        let assign8520_e7668: f64 = (assign8520_e7666 * locals.var_epsch);
        let assign8520_e7670: f64 = (assign8520_e7668 * locals.var_inv_phit);
        (assign8520_e7670, ((((assign8520_e7664 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn4)), ((((assign8520_e7664 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn6)), ((((assign8520_e7664 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn7)), ((((assign8520_e7664 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn8)), ((((assign8520_e7664 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_a0_csisq, locals.var_a0_csisq_dn4, locals.var_a0_csisq_dn6, locals.var_a0_csisq_dn7, locals.var_a0_csisq_dn8, locals.var_a0_csisq_dn9,)
    }
};
        locals.var_a0_csisq = assign8520_e7672;
        locals.var_a0_csisq_dn4 = assign8520_e7672_d_n4;
        locals.var_a0_csisq_dn6 = assign8520_e7672_d_n6;
        locals.var_a0_csisq_dn7 = assign8520_e7672_d_n7;
        locals.var_a0_csisq_dn8 = assign8520_e7672_d_n8;
        locals.var_a0_csisq_dn9 = assign8520_e7672_d_n9;

        let (assign8530_e7683, assign8530_e7683_d_n4, assign8530_e7683_d_n6, assign8530_e7683_d_n7, assign8530_e7683_d_n8, assign8530_e7683_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8530_e7676: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign8530_e7678: f64 = (assign8530_e7676 / locals.var_a0_csisq);
        let assign8530_e7679: f64 = (assign8530_e7678).ln();
        let assign8530_e7681: f64 = (assign8530_e7679 - 0.6931471805599);
        (assign8530_e7681, ((-((assign8530_e7676 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678),)
    } else {
        (locals.var_xth_1d, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9,)
    }
};
        locals.var_xth_1d = assign8530_e7683;
        locals.var_xth_1d_dn4 = assign8530_e7683_d_n4;
        locals.var_xth_1d_dn6 = assign8530_e7683_d_n6;
        locals.var_xth_1d_dn7 = assign8530_e7683_d_n7;
        locals.var_xth_1d_dn8 = assign8530_e7683_d_n8;
        locals.var_xth_1d_dn9 = assign8530_e7683_d_n9;

        let (assign8540_e7699, assign8540_e7699_d_n4, assign8540_e7699_d_n6, assign8540_e7699_d_n7, assign8540_e7699_d_n8, assign8540_e7699_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8540_e7687: f64 = (0.5 * 1.602176565e-19);
        let assign8540_e7689: f64 = (assign8540_e7687 * locals.var_nsddc_i);
        let assign8540_e7691: f64 = (assign8540_e7689 * locals.var_tsi_i);
        let assign8540_e7694: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign8540_e7695: f64 = (assign8540_e7691 / assign8540_e7694);
        let assign8540_e7697: f64 = (assign8540_e7695 * locals.var_inv_phit);
        (assign8540_e7697, (assign8540_e7695 * locals.var_inv_phit_dn4), (assign8540_e7695 * locals.var_inv_phit_dn6), (assign8540_e7695 * locals.var_inv_phit_dn7), (assign8540_e7695 * locals.var_inv_phit_dn8), (assign8540_e7695 * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_xsddep, locals.var_xsddep_dn4, locals.var_xsddep_dn6, locals.var_xsddep_dn7, locals.var_xsddep_dn8, locals.var_xsddep_dn9,)
    }
};
        locals.var_xsddep = assign8540_e7699;
        locals.var_xsddep_dn4 = assign8540_e7699_d_n4;
        locals.var_xsddep_dn6 = assign8540_e7699_d_n6;
        locals.var_xsddep_dn7 = assign8540_e7699_d_n7;
        locals.var_xsddep_dn8 = assign8540_e7699_d_n8;
        locals.var_xsddep_dn9 = assign8540_e7699_d_n9;

        let (assign8550_e7705, assign8550_e7705_d_n4, assign8550_e7705_d_n6, assign8550_e7705_d_n7, assign8550_e7705_d_n8, assign8550_e7705_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8550_e7703: f64 = (locals.var_stcf_i * locals.var_dt);
        (assign8550_e7703, ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4)), ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6)), ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7)), ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8)), ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8550_e7705;
        locals.var_temp_dn4 = assign8550_e7705_d_n4;
        locals.var_temp_dn6 = assign8550_e7705_d_n6;
        locals.var_temp_dn7 = assign8550_e7705_d_n7;
        locals.var_temp_dn8 = assign8550_e7705_d_n8;
        locals.var_temp_dn9 = assign8550_e7705_d_n9;

        let (assign8560_e7711, assign8560_e7711_d_n4, assign8560_e7711_d_n6, assign8560_e7711_d_n7, assign8560_e7711_d_n8, assign8560_e7711_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8560_e7709: f64 = (locals.var_cf1_t + locals.var_temp);
        (assign8560_e7709, (locals.var_cf1_t_dn4 + locals.var_temp_dn4), (locals.var_cf1_t_dn6 + locals.var_temp_dn6), (locals.var_cf1_t_dn7 + locals.var_temp_dn7), (locals.var_cf1_t_dn8 + locals.var_temp_dn8), (locals.var_cf1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9,)
    }
};
        locals.var_cf1_i = assign8560_e7711;
        locals.var_cf1_i_dn4 = assign8560_e7711_d_n4;
        locals.var_cf1_i_dn6 = assign8560_e7711_d_n6;
        locals.var_cf1_i_dn7 = assign8560_e7711_d_n7;
        locals.var_cf1_i_dn8 = assign8560_e7711_d_n8;
        locals.var_cf1_i_dn9 = assign8560_e7711_d_n9;

        let (assign8570_e7717, assign8570_e7717_d_n4, assign8570_e7717_d_n6, assign8570_e7717_d_n7, assign8570_e7717_d_n8, assign8570_e7717_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8570_e7715: f64 = (locals.var_cf2_t + locals.var_temp);
        (assign8570_e7715, (locals.var_cf2_t_dn4 + locals.var_temp_dn4), (locals.var_cf2_t_dn6 + locals.var_temp_dn6), (locals.var_cf2_t_dn7 + locals.var_temp_dn7), (locals.var_cf2_t_dn8 + locals.var_temp_dn8), (locals.var_cf2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9,)
    }
};
        locals.var_cf2_i = assign8570_e7717;
        locals.var_cf2_i_dn4 = assign8570_e7717_d_n4;
        locals.var_cf2_i_dn6 = assign8570_e7717_d_n6;
        locals.var_cf2_i_dn7 = assign8570_e7717_d_n7;
        locals.var_cf2_i_dn8 = assign8570_e7717_d_n8;
        locals.var_cf2_i_dn9 = assign8570_e7717_d_n9;

        let (assign8580_e7723, assign8580_e7723_d_n4, assign8580_e7723_d_n6, assign8580_e7723_d_n7, assign8580_e7723_d_n8, assign8580_e7723_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8580_e7721: f64 = (locals.var_cfd_i * locals.var_inv_phit);
        (assign8580_e7721, (locals.var_cfd_i * locals.var_inv_phit_dn4), (locals.var_cfd_i * locals.var_inv_phit_dn6), (locals.var_cfd_i * locals.var_inv_phit_dn7), (locals.var_cfd_i * locals.var_inv_phit_dn8), (locals.var_cfd_i * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_xd0, locals.var_xd0_dn4, locals.var_xd0_dn6, locals.var_xd0_dn7, locals.var_xd0_dn8, locals.var_xd0_dn9,)
    }
};
        locals.var_xd0 = assign8580_e7723;
        locals.var_xd0_dn4 = assign8580_e7723_d_n4;
        locals.var_xd0_dn6 = assign8580_e7723_d_n6;
        locals.var_xd0_dn7 = assign8580_e7723_d_n7;
        locals.var_xd0_dn8 = assign8580_e7723_d_n8;
        locals.var_xd0_dn9 = assign8580_e7723_d_n9;

    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8590_e7729, assign8590_e7729_d_n4, assign8590_e7729_d_n6, assign8590_e7729_d_n7, assign8590_e7729_d_n8, assign8590_e7729_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8590_e7727: f64 = (locals.var_cfac1_t + locals.var_temp);
        (assign8590_e7727, (locals.var_cfac1_t_dn4 + locals.var_temp_dn4), (locals.var_cfac1_t_dn6 + locals.var_temp_dn6), (locals.var_cfac1_t_dn7 + locals.var_temp_dn7), (locals.var_cfac1_t_dn8 + locals.var_temp_dn8), (locals.var_cfac1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9,)
    }
};
        locals.var_cfac1_i = assign8590_e7729;
        locals.var_cfac1_i_dn4 = assign8590_e7729_d_n4;
        locals.var_cfac1_i_dn6 = assign8590_e7729_d_n6;
        locals.var_cfac1_i_dn7 = assign8590_e7729_d_n7;
        locals.var_cfac1_i_dn8 = assign8590_e7729_d_n8;
        locals.var_cfac1_i_dn9 = assign8590_e7729_d_n9;

        let (assign8600_e7735, assign8600_e7735_d_n4, assign8600_e7735_d_n6, assign8600_e7735_d_n7, assign8600_e7735_d_n8, assign8600_e7735_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8600_e7733: f64 = (locals.var_cfac2_t + locals.var_temp);
        (assign8600_e7733, (locals.var_cfac2_t_dn4 + locals.var_temp_dn4), (locals.var_cfac2_t_dn6 + locals.var_temp_dn6), (locals.var_cfac2_t_dn7 + locals.var_temp_dn7), (locals.var_cfac2_t_dn8 + locals.var_temp_dn8), (locals.var_cfac2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9,)
    }
};
        locals.var_cfac2_i = assign8600_e7735;
        locals.var_cfac2_i_dn4 = assign8600_e7735_d_n4;
        locals.var_cfac2_i_dn6 = assign8600_e7735_d_n6;
        locals.var_cfac2_i_dn7 = assign8600_e7735_d_n7;
        locals.var_cfac2_i_dn8 = assign8600_e7735_d_n8;
        locals.var_cfac2_i_dn9 = assign8600_e7735_d_n9;

        let assign8610_e7738: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign8610_e7738;

        let (assign8620_e7751, assign8620_e7751_d_n4, assign8620_e7751_d_n6, assign8620_e7751_d_n7, assign8620_e7751_d_n8, assign8620_e7751_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard259 != 0.0)) {
        let assign8620_e7745: f64 = (locals.var_np_i / locals.var_neff_poly);
        let assign8620_e7746: f64 = (assign8620_e7745).ln();
        let assign8620_e7748: f64 = (assign8620_e7746 + locals.var_eg_2phit0_woshe);
        let assign8620_e7749: f64 = (locals.var_phit0 * assign8620_e7748);
        (assign8620_e7749, ((locals.var_phit0_dn4 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn4))), ((locals.var_phit0_dn6 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn6))), ((locals.var_phit0_dn7 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn7))), ((locals.var_phit0_dn8 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn8))), ((locals.var_phit0_dn9 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn9))),)
    } else {
        (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9,)
    }
};
        locals.var_dvfbpdep = assign8620_e7751;
        locals.var_dvfbpdep_dn4 = assign8620_e7751_d_n4;
        locals.var_dvfbpdep_dn6 = assign8620_e7751_d_n6;
        locals.var_dvfbpdep_dn7 = assign8620_e7751_d_n7;
        locals.var_dvfbpdep_dn8 = assign8620_e7751_d_n8;
        locals.var_dvfbpdep_dn9 = assign8620_e7751_d_n9;

        let assign8630_e7754: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign8630_e7754;

        let (assign8640_e7781, assign8640_e7781_d_n4, assign8640_e7781_d_n6, assign8640_e7781_d_n7, assign8640_e7781_d_n8, assign8640_e7781_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard260 != 0.0)) {
        let assign8640_e7762: f64 = (2970.0 / locals.var_tkd);
        let assign8640_e7763: f64 = (15.0 + assign8640_e7762);
        let assign8640_e7767: f64 = (2970.0 / locals.var_tkd);
        let assign8640_e7768: f64 = (15.0 - assign8640_e7767);
        let assign8640_e7772: f64 = (2970.0 / locals.var_tkd);
        let assign8640_e7773: f64 = (15.0 - assign8640_e7772);
        let assign8640_e7774: f64 = (assign8640_e7768 * assign8640_e7773);
        let assign8640_e7776: f64 = (assign8640_e7774 + 1e-6);
        let assign8640_e7777: f64 = (assign8640_e7776).sqrt();
        let assign8640_e7778: f64 = (assign8640_e7763 + assign8640_e7777);
        let assign8640_e7779: f64 = (0.5 * assign8640_e7778);
        (assign8640_e7779, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))),)
    } else {
        (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9,)
    }
};
        locals.var_emin = assign8640_e7781;
        locals.var_emin_dn4 = assign8640_e7781_d_n4;
        locals.var_emin_dn6 = assign8640_e7781_d_n6;
        locals.var_emin_dn7 = assign8640_e7781_d_n7;
        locals.var_emin_dn8 = assign8640_e7781_d_n8;
        locals.var_emin_dn9 = assign8640_e7781_d_n9;

        let (assign8650_e7785, assign8650_e7785_d_n4, assign8650_e7785_d_n6, assign8650_e7785_d_n7, assign8650_e7785_d_n8, assign8650_e7785_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8650_e7785;
        locals.var_qq_dn4 = assign8650_e7785_d_n4;
        locals.var_qq_dn6 = assign8650_e7785_d_n6;
        locals.var_qq_dn7 = assign8650_e7785_d_n7;
        locals.var_qq_dn8 = assign8650_e7785_d_n8;
        locals.var_qq_dn9 = assign8650_e7785_d_n9;

        let assign8660_e7788: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign8660_e7788;

        let assign8670_e7791: f64 = 1.0;
        let assign8670_e7792: f64 = if p.p14 == assign8670_e7791 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign8670_e7792;

        let (assign8680_e7813, assign8680_e7813_d_n4, assign8680_e7813_d_n6, assign8680_e7813_d_n7, assign8680_e7813_d_n8, assign8680_e7813_d_n9,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 != 0.0)) {
        let assign8680_e7800: f64 = (0.4 * p.p13);
        let assign8680_e7802: f64 = (assign8680_e7800 * 1.27520989);
        let assign8680_e7804: f64 = (-0.3333333333333);
        let assign8680_e7807: f64 = (locals.var_phit * locals.var_tsisq);
        let assign8680_e7808: f64 = (assign8680_e7807).ln();
        let assign8680_e7809: f64 = (assign8680_e7804 * assign8680_e7808);
        let assign8680_e7810: f64 = (assign8680_e7809).exp();
        let assign8680_e7811: f64 = (assign8680_e7802 * assign8680_e7810);
        (assign8680_e7811, (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8680_e7807)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8680_e7813;
        locals.var_qq_dn4 = assign8680_e7813_d_n4;
        locals.var_qq_dn6 = assign8680_e7813_d_n6;
        locals.var_qq_dn7 = assign8680_e7813_d_n7;
        locals.var_qq_dn8 = assign8680_e7813_d_n8;
        locals.var_qq_dn9 = assign8680_e7813_d_n9;

        let (assign8690_e7835, assign8690_e7835_d_n4, assign8690_e7835_d_n6, assign8690_e7835_d_n7, assign8690_e7835_d_n8, assign8690_e7835_d_n9,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 == 0.0)) {
        let assign8690_e7822: f64 = (0.4 * p.p13);
        let assign8690_e7824: f64 = (assign8690_e7822 * 1.5412087);
        let assign8690_e7826: f64 = (-0.3333333333333);
        let assign8690_e7829: f64 = (locals.var_phit * locals.var_tsisq);
        let assign8690_e7830: f64 = (assign8690_e7829).ln();
        let assign8690_e7831: f64 = (assign8690_e7826 * assign8690_e7830);
        let assign8690_e7832: f64 = (assign8690_e7831).exp();
        let assign8690_e7833: f64 = (assign8690_e7824 * assign8690_e7832);
        (assign8690_e7833, (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8690_e7829)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8690_e7835;
        locals.var_qq_dn4 = assign8690_e7835_d_n4;
        locals.var_qq_dn6 = assign8690_e7835_d_n6;
        locals.var_qq_dn7 = assign8690_e7835_d_n7;
        locals.var_qq_dn8 = assign8690_e7835_d_n8;
        locals.var_qq_dn9 = assign8690_e7835_d_n9;

        let (assign8700_e7845, assign8700_e7845_d_n4, assign8700_e7845_d_n6, assign8700_e7845_d_n7, assign8700_e7845_d_n8, assign8700_e7845_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8700_e7839: f64 = (p.p14 * locals.var_stvfb_i);
        let assign8700_e7841: f64 = (assign8700_e7839 * locals.var_dt);
        let assign8700_e7843: f64 = (assign8700_e7841 + locals.var_dvfbqm);
        (assign8700_e7843, (assign8700_e7839 * locals.var_dt_dn4), (assign8700_e7839 * locals.var_dt_dn6), (assign8700_e7839 * locals.var_dt_dn7), (assign8700_e7839 * locals.var_dt_dn8), (assign8700_e7839 * locals.var_dt_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8700_e7845;
        locals.var_temp_dn4 = assign8700_e7845_d_n4;
        locals.var_temp_dn6 = assign8700_e7845_d_n6;
        locals.var_temp_dn7 = assign8700_e7845_d_n7;
        locals.var_temp_dn8 = assign8700_e7845_d_n8;
        locals.var_temp_dn9 = assign8700_e7845_d_n9;

        let (assign8710_e7853, assign8710_e7853_d_n4, assign8710_e7853_d_n6, assign8710_e7853_d_n7, assign8710_e7853_d_n8, assign8710_e7853_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8710_e7849: f64 = (locals.var_temp + p.p34);
        let assign8710_e7851: f64 = (assign8710_e7849 - locals.var_dvfbpdep);
        (assign8710_e7851, (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4), (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6), (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7), (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8), (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign8710_e7853;
        locals.var_temp1_dn4 = assign8710_e7853_d_n4;
        locals.var_temp1_dn6 = assign8710_e7853_d_n6;
        locals.var_temp1_dn7 = assign8710_e7853_d_n7;
        locals.var_temp1_dn8 = assign8710_e7853_d_n8;
        locals.var_temp1_dn9 = assign8710_e7853_d_n9;

        let (assign8720_e7865, assign8720_e7865_d_n4, assign8720_e7865_d_n6, assign8720_e7865_d_n7, assign8720_e7865_d_n8, assign8720_e7865_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8720_e7858: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
        let assign8720_e7860: f64 = (assign8720_e7858 + locals.var_dvfb1nch);
        let assign8720_e7861: f64 = (p.p14 * assign8720_e7860);
        let assign8720_e7863: f64 = (assign8720_e7861 + locals.var_temp1);
        (assign8720_e7863, ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9),)
    } else {
        (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9,)
    }
};
        locals.var_vfb1_i = assign8720_e7865;
        locals.var_vfb1_i_dn4 = assign8720_e7865_d_n4;
        locals.var_vfb1_i_dn6 = assign8720_e7865_d_n6;
        locals.var_vfb1_i_dn7 = assign8720_e7865_d_n7;
        locals.var_vfb1_i_dn8 = assign8720_e7865_d_n8;
        locals.var_vfb1_i_dn9 = assign8720_e7865_d_n9;

        let (assign8730_e7877, assign8730_e7877_d_n4, assign8730_e7877_d_n6, assign8730_e7877_d_n7, assign8730_e7877_d_n8, assign8730_e7877_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8730_e7870: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
        let assign8730_e7872: f64 = (assign8730_e7870 + locals.var_dvfb2nch);
        let assign8730_e7873: f64 = (p.p14 * assign8730_e7872);
        let assign8730_e7875: f64 = (assign8730_e7873 + locals.var_temp);
        (assign8730_e7875, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9,)
    }
};
        locals.var_vfb2_i = assign8730_e7877;
        locals.var_vfb2_i_dn4 = assign8730_e7877_d_n4;
        locals.var_vfb2_i_dn6 = assign8730_e7877_d_n6;
        locals.var_vfb2_i_dn7 = assign8730_e7877_d_n7;
        locals.var_vfb2_i_dn8 = assign8730_e7877_d_n8;
        locals.var_vfb2_i_dn9 = assign8730_e7877_d_n9;

        let (assign8740_e7889, assign8740_e7889_d_n4, assign8740_e7889_d_n6, assign8740_e7889_d_n7, assign8740_e7889_d_n8, assign8740_e7889_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8740_e7882: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
        let assign8740_e7884: f64 = (assign8740_e7882 + locals.var_dvfb1nch);
        let assign8740_e7885: f64 = (p.p14 * assign8740_e7884);
        let assign8740_e7887: f64 = (assign8740_e7885 + locals.var_temp1);
        (assign8740_e7887, ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9),)
    } else {
        (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9,)
    }
};
        locals.var_vfbac1_i = assign8740_e7889;
        locals.var_vfbac1_i_dn4 = assign8740_e7889_d_n4;
        locals.var_vfbac1_i_dn6 = assign8740_e7889_d_n6;
        locals.var_vfbac1_i_dn7 = assign8740_e7889_d_n7;
        locals.var_vfbac1_i_dn8 = assign8740_e7889_d_n8;
        locals.var_vfbac1_i_dn9 = assign8740_e7889_d_n9;

        let (assign8750_e7901, assign8750_e7901_d_n4, assign8750_e7901_d_n6, assign8750_e7901_d_n7, assign8750_e7901_d_n8, assign8750_e7901_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8750_e7894: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
        let assign8750_e7896: f64 = (assign8750_e7894 + locals.var_dvfb2nch);
        let assign8750_e7897: f64 = (p.p14 * assign8750_e7896);
        let assign8750_e7899: f64 = (assign8750_e7897 + locals.var_temp);
        (assign8750_e7899, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9,)
    }
};
        locals.var_vfbac2_i = assign8750_e7901;
        locals.var_vfbac2_i_dn4 = assign8750_e7901_d_n4;
        locals.var_vfbac2_i_dn6 = assign8750_e7901_d_n6;
        locals.var_vfbac2_i_dn7 = assign8750_e7901_d_n7;
        locals.var_vfbac2_i_dn8 = assign8750_e7901_d_n8;
        locals.var_vfbac2_i_dn9 = assign8750_e7901_d_n9;

        let (assign8760_e7906, assign8760_e7906_d_n4, assign8760_e7906_d_n6, assign8760_e7906_d_n7, assign8760_e7906_d_n8, assign8760_e7906_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8760_e7904: f64 = (locals.var_rtn).ln();
        (assign8760_e7904, (locals.var_rtn_dn4 / locals.var_rtn), (locals.var_rtn_dn6 / locals.var_rtn), (locals.var_rtn_dn7 / locals.var_rtn), (locals.var_rtn_dn8 / locals.var_rtn), (locals.var_rtn_dn9 / locals.var_rtn),)
    } else {
        (locals.var_lnrtn, locals.var_lnrtn_dn4, locals.var_lnrtn_dn6, locals.var_lnrtn_dn7, locals.var_lnrtn_dn8, locals.var_lnrtn_dn9,)
    }
};
        locals.var_lnrtn = assign8760_e7906;
        locals.var_lnrtn_dn4 = assign8760_e7906_d_n4;
        locals.var_lnrtn_dn6 = assign8760_e7906_d_n6;
        locals.var_lnrtn_dn7 = assign8760_e7906_d_n7;
        locals.var_lnrtn_dn8 = assign8760_e7906_d_n8;
        locals.var_lnrtn_dn9 = assign8760_e7906_d_n9;

        let (assign8770_e7915, assign8770_e7915_d_n4, assign8770_e7915_d_n6, assign8770_e7915_d_n7, assign8770_e7915_d_n8, assign8770_e7915_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8770_e7910: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign8770_e7911: f64 = (assign8770_e7910).exp();
        let assign8770_e7913: f64 = (assign8770_e7911 * p.p35);
        (assign8770_e7913, ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35),)
    } else {
        (locals.var_tf_bet, locals.var_tf_bet_dn4, locals.var_tf_bet_dn6, locals.var_tf_bet_dn7, locals.var_tf_bet_dn8, locals.var_tf_bet_dn9,)
    }
};
        locals.var_tf_bet = assign8770_e7915;
        locals.var_tf_bet_dn4 = assign8770_e7915_d_n4;
        locals.var_tf_bet_dn6 = assign8770_e7915_d_n6;
        locals.var_tf_bet_dn7 = assign8770_e7915_d_n7;
        locals.var_tf_bet_dn8 = assign8770_e7915_d_n8;
        locals.var_tf_bet_dn9 = assign8770_e7915_d_n9;

        let (assign8780_e7921, assign8780_e7921_d_n4, assign8780_e7921_d_n6, assign8780_e7921_d_n7, assign8780_e7921_d_n8, assign8780_e7921_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8780_e7919: f64 = (locals.var_betn1_t * locals.var_tf_bet);
        (assign8780_e7919, ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4)), ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6)), ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7)), ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8)), ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_betn1_i, locals.var_betn1_i_dn4, locals.var_betn1_i_dn6, locals.var_betn1_i_dn7, locals.var_betn1_i_dn8, locals.var_betn1_i_dn9,)
    }
};
        locals.var_betn1_i = assign8780_e7921;
        locals.var_betn1_i_dn4 = assign8780_e7921_d_n4;
        locals.var_betn1_i_dn6 = assign8780_e7921_d_n6;
        locals.var_betn1_i_dn7 = assign8780_e7921_d_n7;
        locals.var_betn1_i_dn8 = assign8780_e7921_d_n8;
        locals.var_betn1_i_dn9 = assign8780_e7921_d_n9;

        let (assign8790_e7927, assign8790_e7927_d_n4, assign8790_e7927_d_n6, assign8790_e7927_d_n7, assign8790_e7927_d_n8, assign8790_e7927_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8790_e7925: f64 = (locals.var_betn2_t * locals.var_tf_bet);
        (assign8790_e7925, ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4)), ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6)), ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7)), ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8)), ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_betn2_i, locals.var_betn2_i_dn4, locals.var_betn2_i_dn6, locals.var_betn2_i_dn7, locals.var_betn2_i_dn8, locals.var_betn2_i_dn9,)
    }
};
        locals.var_betn2_i = assign8790_e7927;
        locals.var_betn2_i_dn4 = assign8790_e7927_d_n4;
        locals.var_betn2_i_dn6 = assign8790_e7927_d_n6;
        locals.var_betn2_i_dn7 = assign8790_e7927_d_n7;
        locals.var_betn2_i_dn8 = assign8790_e7927_d_n8;
        locals.var_betn2_i_dn9 = assign8790_e7927_d_n9;

        let (assign8800_e7934, assign8800_e7934_d_n4, assign8800_e7934_d_n6, assign8800_e7934_d_n7, assign8800_e7934_d_n8, assign8800_e7934_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8800_e7931: f64 = (locals.var_stmue_i * locals.var_lnrtn);
        let assign8800_e7932: f64 = (assign8800_e7931).exp();
        (assign8800_e7932, (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn4)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn6)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn7)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn8)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_mue, locals.var_tf_mue_dn4, locals.var_tf_mue_dn6, locals.var_tf_mue_dn7, locals.var_tf_mue_dn8, locals.var_tf_mue_dn9,)
    }
};
        locals.var_tf_mue = assign8800_e7934;
        locals.var_tf_mue_dn4 = assign8800_e7934_d_n4;
        locals.var_tf_mue_dn6 = assign8800_e7934_d_n6;
        locals.var_tf_mue_dn7 = assign8800_e7934_d_n7;
        locals.var_tf_mue_dn8 = assign8800_e7934_d_n8;
        locals.var_tf_mue_dn9 = assign8800_e7934_d_n9;

        let (assign8810_e7940, assign8810_e7940_d_n4, assign8810_e7940_d_n6, assign8810_e7940_d_n7, assign8810_e7940_d_n8, assign8810_e7940_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8810_e7938: f64 = (locals.var_mue_t * locals.var_tf_mue);
        (assign8810_e7938, (locals.var_mue_t * locals.var_tf_mue_dn4), (locals.var_mue_t * locals.var_tf_mue_dn6), (locals.var_mue_t * locals.var_tf_mue_dn7), (locals.var_mue_t * locals.var_tf_mue_dn8), (locals.var_mue_t * locals.var_tf_mue_dn9),)
    } else {
        (locals.var_mue_i, locals.var_mue_i_dn4, locals.var_mue_i_dn6, locals.var_mue_i_dn7, locals.var_mue_i_dn8, locals.var_mue_i_dn9,)
    }
};
        locals.var_mue_i = assign8810_e7940;
        locals.var_mue_i_dn4 = assign8810_e7940_d_n4;
        locals.var_mue_i_dn6 = assign8810_e7940_d_n6;
        locals.var_mue_i_dn7 = assign8810_e7940_d_n7;
        locals.var_mue_i_dn8 = assign8810_e7940_d_n8;
        locals.var_mue_i_dn9 = assign8810_e7940_d_n9;

        let (assign8820_e7947, assign8820_e7947_d_n4, assign8820_e7947_d_n6, assign8820_e7947_d_n7, assign8820_e7947_d_n8, assign8820_e7947_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8820_e7944: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
        let assign8820_e7945: f64 = (assign8820_e7944).exp();
        (assign8820_e7945, (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn4)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn6)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn7)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn8)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_themu, locals.var_tf_themu_dn4, locals.var_tf_themu_dn6, locals.var_tf_themu_dn7, locals.var_tf_themu_dn8, locals.var_tf_themu_dn9,)
    }
};
        locals.var_tf_themu = assign8820_e7947;
        locals.var_tf_themu_dn4 = assign8820_e7947_d_n4;
        locals.var_tf_themu_dn6 = assign8820_e7947_d_n6;
        locals.var_tf_themu_dn7 = assign8820_e7947_d_n7;
        locals.var_tf_themu_dn8 = assign8820_e7947_d_n8;
        locals.var_tf_themu_dn9 = assign8820_e7947_d_n9;

        let (assign8830_e7953, assign8830_e7953_d_n4, assign8830_e7953_d_n6, assign8830_e7953_d_n7, assign8830_e7953_d_n8, assign8830_e7953_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8830_e7951: f64 = (locals.var_themu_t * locals.var_tf_themu);
        (assign8830_e7951, (locals.var_themu_t * locals.var_tf_themu_dn4), (locals.var_themu_t * locals.var_tf_themu_dn6), (locals.var_themu_t * locals.var_tf_themu_dn7), (locals.var_themu_t * locals.var_tf_themu_dn8), (locals.var_themu_t * locals.var_tf_themu_dn9),)
    } else {
        (locals.var_themu_i, locals.var_themu_i_dn4, locals.var_themu_i_dn6, locals.var_themu_i_dn7, locals.var_themu_i_dn8, locals.var_themu_i_dn9,)
    }
};
        locals.var_themu_i = assign8830_e7953;
        locals.var_themu_i_dn4 = assign8830_e7953_d_n4;
        locals.var_themu_i_dn6 = assign8830_e7953_d_n6;
        locals.var_themu_i_dn7 = assign8830_e7953_d_n7;
        locals.var_themu_i_dn8 = assign8830_e7953_d_n8;
        locals.var_themu_i_dn9 = assign8830_e7953_d_n9;

        let (assign8840_e7960, assign8840_e7960_d_n4, assign8840_e7960_d_n6, assign8840_e7960_d_n7, assign8840_e7960_d_n8, assign8840_e7960_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8840_e7957: f64 = (locals.var_stcs_i * locals.var_lnrtn);
        let assign8840_e7958: f64 = (assign8840_e7957).exp();
        (assign8840_e7958, (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn4)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn6)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn7)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn8)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_cs, locals.var_tf_cs_dn4, locals.var_tf_cs_dn6, locals.var_tf_cs_dn7, locals.var_tf_cs_dn8, locals.var_tf_cs_dn9,)
    }
};
        locals.var_tf_cs = assign8840_e7960;
        locals.var_tf_cs_dn4 = assign8840_e7960_d_n4;
        locals.var_tf_cs_dn6 = assign8840_e7960_d_n6;
        locals.var_tf_cs_dn7 = assign8840_e7960_d_n7;
        locals.var_tf_cs_dn8 = assign8840_e7960_d_n8;
        locals.var_tf_cs_dn9 = assign8840_e7960_d_n9;

        let (assign8850_e7966, assign8850_e7966_d_n4, assign8850_e7966_d_n6, assign8850_e7966_d_n7, assign8850_e7966_d_n8, assign8850_e7966_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8850_e7964: f64 = (locals.var_cs_t * locals.var_tf_cs);
        (assign8850_e7964, (locals.var_cs_t * locals.var_tf_cs_dn4), (locals.var_cs_t * locals.var_tf_cs_dn6), (locals.var_cs_t * locals.var_tf_cs_dn7), (locals.var_cs_t * locals.var_tf_cs_dn8), (locals.var_cs_t * locals.var_tf_cs_dn9),)
    } else {
        (locals.var_cs_i, locals.var_cs_i_dn4, locals.var_cs_i_dn6, locals.var_cs_i_dn7, locals.var_cs_i_dn8, locals.var_cs_i_dn9,)
    }
};
        locals.var_cs_i = assign8850_e7966;
        locals.var_cs_i_dn4 = assign8850_e7966_d_n4;
        locals.var_cs_i_dn6 = assign8850_e7966_d_n6;
        locals.var_cs_i_dn7 = assign8850_e7966_d_n7;
        locals.var_cs_i_dn8 = assign8850_e7966_d_n8;
        locals.var_cs_i_dn9 = assign8850_e7966_d_n9;

        let (assign8860_e7973, assign8860_e7973_d_n4, assign8860_e7973_d_n6, assign8860_e7973_d_n7, assign8860_e7973_d_n8, assign8860_e7973_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8860_e7970: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
        let assign8860_e7971: f64 = (assign8860_e7970).exp();
        (assign8860_e7971, (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn4)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn6)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn7)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn8)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_thecs, locals.var_tf_thecs_dn4, locals.var_tf_thecs_dn6, locals.var_tf_thecs_dn7, locals.var_tf_thecs_dn8, locals.var_tf_thecs_dn9,)
    }
};
        locals.var_tf_thecs = assign8860_e7973;
        locals.var_tf_thecs_dn4 = assign8860_e7973_d_n4;
        locals.var_tf_thecs_dn6 = assign8860_e7973_d_n6;
        locals.var_tf_thecs_dn7 = assign8860_e7973_d_n7;
        locals.var_tf_thecs_dn8 = assign8860_e7973_d_n8;
        locals.var_tf_thecs_dn9 = assign8860_e7973_d_n9;

        let (assign8870_e7979, assign8870_e7979_d_n4, assign8870_e7979_d_n6, assign8870_e7979_d_n7, assign8870_e7979_d_n8, assign8870_e7979_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8870_e7977: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
        (assign8870_e7977, (locals.var_thecs_t * locals.var_tf_thecs_dn4), (locals.var_thecs_t * locals.var_tf_thecs_dn6), (locals.var_thecs_t * locals.var_tf_thecs_dn7), (locals.var_thecs_t * locals.var_tf_thecs_dn8), (locals.var_thecs_t * locals.var_tf_thecs_dn9),)
    } else {
        (locals.var_thecs_i, locals.var_thecs_i_dn4, locals.var_thecs_i_dn6, locals.var_thecs_i_dn7, locals.var_thecs_i_dn8, locals.var_thecs_i_dn9,)
    }
};
        locals.var_thecs_i = assign8870_e7979;
        locals.var_thecs_i_dn4 = assign8870_e7979_d_n4;
        locals.var_thecs_i_dn6 = assign8870_e7979_d_n6;
        locals.var_thecs_i_dn7 = assign8870_e7979_d_n7;
        locals.var_thecs_i_dn8 = assign8870_e7979_d_n8;
        locals.var_thecs_i_dn9 = assign8870_e7979_d_n9;

        let (assign8880_e7986, assign8880_e7986_d_n4, assign8880_e7986_d_n6, assign8880_e7986_d_n7, assign8880_e7986_d_n8, assign8880_e7986_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8880_e7983: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
        let assign8880_e7984: f64 = (assign8880_e7983).exp();
        (assign8880_e7984, (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn4)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn6)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn7)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn8)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_xcor, locals.var_tf_xcor_dn4, locals.var_tf_xcor_dn6, locals.var_tf_xcor_dn7, locals.var_tf_xcor_dn8, locals.var_tf_xcor_dn9,)
    }
};
        locals.var_tf_xcor = assign8880_e7986;
        locals.var_tf_xcor_dn4 = assign8880_e7986_d_n4;
        locals.var_tf_xcor_dn6 = assign8880_e7986_d_n6;
        locals.var_tf_xcor_dn7 = assign8880_e7986_d_n7;
        locals.var_tf_xcor_dn8 = assign8880_e7986_d_n8;
        locals.var_tf_xcor_dn9 = assign8880_e7986_d_n9;

        let (assign8890_e7992, assign8890_e7992_d_n4, assign8890_e7992_d_n6, assign8890_e7992_d_n7, assign8890_e7992_d_n8, assign8890_e7992_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8890_e7990: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
        (assign8890_e7990, (locals.var_xcor_t * locals.var_tf_xcor_dn4), (locals.var_xcor_t * locals.var_tf_xcor_dn6), (locals.var_xcor_t * locals.var_tf_xcor_dn7), (locals.var_xcor_t * locals.var_tf_xcor_dn8), (locals.var_xcor_t * locals.var_tf_xcor_dn9),)
    } else {
        (locals.var_xcor_i, locals.var_xcor_i_dn4, locals.var_xcor_i_dn6, locals.var_xcor_i_dn7, locals.var_xcor_i_dn8, locals.var_xcor_i_dn9,)
    }
};
        locals.var_xcor_i = assign8890_e7992;
        locals.var_xcor_i_dn4 = assign8890_e7992_d_n4;
        locals.var_xcor_i_dn6 = assign8890_e7992_d_n6;
        locals.var_xcor_i_dn7 = assign8890_e7992_d_n7;
        locals.var_xcor_i_dn8 = assign8890_e7992_d_n8;
        locals.var_xcor_i_dn9 = assign8890_e7992_d_n9;

        let (assign8900_e8000, assign8900_e8000_d_n4, assign8900_e8000_d_n6, assign8900_e8000_d_n7, assign8900_e8000_d_n8, assign8900_e8000_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8900_e7996: f64 = (1e-8 * locals.var_phit);
        let assign8900_e7998: f64 = (assign8900_e7996 / locals.var_tsi_i);
        (assign8900_e7998, ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8900_e8000;
        locals.var_temp_dn4 = assign8900_e8000_d_n4;
        locals.var_temp_dn6 = assign8900_e8000_d_n6;
        locals.var_temp_dn7 = assign8900_e8000_d_n7;
        locals.var_temp_dn8 = assign8900_e8000_d_n8;
        locals.var_temp_dn9 = assign8900_e8000_d_n9;

        let (assign8910_e8006, assign8910_e8006_d_n4, assign8910_e8006_d_n6, assign8910_e8006_d_n7, assign8910_e8006_d_n8, assign8910_e8006_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8910_e8004: f64 = (locals.var_temp * locals.var_mue_i);
        (assign8910_e8004, ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4)), ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6)), ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7)), ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8)), ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9)),)
    } else {
        (locals.var_fmue, locals.var_fmue_dn4, locals.var_fmue_dn6, locals.var_fmue_dn7, locals.var_fmue_dn8, locals.var_fmue_dn9,)
    }
};
        locals.var_fmue = assign8910_e8006;
        locals.var_fmue_dn4 = assign8910_e8006_d_n4;
        locals.var_fmue_dn6 = assign8910_e8006_d_n6;
        locals.var_fmue_dn7 = assign8910_e8006_d_n7;
        locals.var_fmue_dn8 = assign8910_e8006_d_n8;
        locals.var_fmue_dn9 = assign8910_e8006_d_n9;

    }

    pub(super) fn stamp_transient_block_19(
        locals: &mut StampLocals,
    ) {
        let (assign8920_e8013, assign8920_e8013_d_n4, assign8920_e8013_d_n6, assign8920_e8013_d_n7, assign8920_e8013_d_n8, assign8920_e8013_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8920_e8010: f64 = (locals.var_strs_i * locals.var_lnrtn);
        let assign8920_e8011: f64 = (assign8920_e8010).exp();
        (assign8920_e8011, (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn4)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn6)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn7)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn8)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_ther, locals.var_tf_ther_dn4, locals.var_tf_ther_dn6, locals.var_tf_ther_dn7, locals.var_tf_ther_dn8, locals.var_tf_ther_dn9,)
    }
};
        locals.var_tf_ther = assign8920_e8013;
        locals.var_tf_ther_dn4 = assign8920_e8013_d_n4;
        locals.var_tf_ther_dn6 = assign8920_e8013_d_n6;
        locals.var_tf_ther_dn7 = assign8920_e8013_d_n7;
        locals.var_tf_ther_dn8 = assign8920_e8013_d_n8;
        locals.var_tf_ther_dn9 = assign8920_e8013_d_n9;

        let (assign8930_e8019, assign8930_e8019_d_n4, assign8930_e8019_d_n6, assign8930_e8019_d_n7, assign8930_e8019_d_n8, assign8930_e8019_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8930_e8017: f64 = (locals.var_rs_t * locals.var_tf_ther);
        (assign8930_e8017, (locals.var_rs_t * locals.var_tf_ther_dn4), (locals.var_rs_t * locals.var_tf_ther_dn6), (locals.var_rs_t * locals.var_tf_ther_dn7), (locals.var_rs_t * locals.var_tf_ther_dn8), (locals.var_rs_t * locals.var_tf_ther_dn9),)
    } else {
        (locals.var_rs_i, locals.var_rs_i_dn4, locals.var_rs_i_dn6, locals.var_rs_i_dn7, locals.var_rs_i_dn8, locals.var_rs_i_dn9,)
    }
};
        locals.var_rs_i = assign8930_e8019;
        locals.var_rs_i_dn4 = assign8930_e8019_d_n4;
        locals.var_rs_i_dn6 = assign8930_e8019_d_n6;
        locals.var_rs_i_dn7 = assign8930_e8019_d_n7;
        locals.var_rs_i_dn8 = assign8930_e8019_d_n8;
        locals.var_rs_i_dn9 = assign8930_e8019_d_n9;

        let (assign8940_e8027, assign8940_e8027_d_n4, assign8940_e8027_d_n6, assign8940_e8027_d_n7, assign8940_e8027_d_n8, assign8940_e8027_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8940_e8023: f64 = (2.0 * locals.var_rs_i);
        let assign8940_e8025: f64 = (assign8940_e8023 * locals.var_phit);
        (assign8940_e8025, (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn4)), (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn6)), (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn7)), (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn8)), (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn9)),)
    } else {
        (locals.var_frs, locals.var_frs_dn4, locals.var_frs_dn6, locals.var_frs_dn7, locals.var_frs_dn8, locals.var_frs_dn9,)
    }
};
        locals.var_frs = assign8940_e8027;
        locals.var_frs_dn4 = assign8940_e8027_d_n4;
        locals.var_frs_dn6 = assign8940_e8027_d_n6;
        locals.var_frs_dn7 = assign8940_e8027_d_n7;
        locals.var_frs_dn8 = assign8940_e8027_d_n8;
        locals.var_frs_dn9 = assign8940_e8027_d_n9;

        let (assign8950_e8034, assign8950_e8034_d_n4, assign8950_e8034_d_n6, assign8950_e8034_d_n7, assign8950_e8034_d_n8, assign8950_e8034_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8950_e8031: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
        let assign8950_e8032: f64 = (assign8950_e8031).exp();
        (assign8950_e8032, (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn4)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn6)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn7)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn8)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_thesat, locals.var_tf_thesat_dn4, locals.var_tf_thesat_dn6, locals.var_tf_thesat_dn7, locals.var_tf_thesat_dn8, locals.var_tf_thesat_dn9,)
    }
};
        locals.var_tf_thesat = assign8950_e8034;
        locals.var_tf_thesat_dn4 = assign8950_e8034_d_n4;
        locals.var_tf_thesat_dn6 = assign8950_e8034_d_n6;
        locals.var_tf_thesat_dn7 = assign8950_e8034_d_n7;
        locals.var_tf_thesat_dn8 = assign8950_e8034_d_n8;
        locals.var_tf_thesat_dn9 = assign8950_e8034_d_n9;

        let (assign8960_e8042, assign8960_e8042_d_n4, assign8960_e8042_d_n6, assign8960_e8042_d_n7, assign8960_e8042_d_n8, assign8960_e8042_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8960_e8038: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
        let assign8960_e8040: f64 = (assign8960_e8038 * locals.var_tf_bet);
        (assign8960_e8040, ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn4)), ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn6)), ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn7)), ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn8)), ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_thesat_i, locals.var_thesat_i_dn4, locals.var_thesat_i_dn6, locals.var_thesat_i_dn7, locals.var_thesat_i_dn8, locals.var_thesat_i_dn9,)
    }
};
        locals.var_thesat_i = assign8960_e8042;
        locals.var_thesat_i_dn4 = assign8960_e8042_d_n4;
        locals.var_thesat_i_dn6 = assign8960_e8042_d_n6;
        locals.var_thesat_i_dn7 = assign8960_e8042_d_n7;
        locals.var_thesat_i_dn8 = assign8960_e8042_d_n8;
        locals.var_thesat_i_dn9 = assign8960_e8042_d_n9;

        let (assign8970_e8048, assign8970_e8048_d_n4, assign8970_e8048_d_n6, assign8970_e8048_d_n7, assign8970_e8048_d_n8, assign8970_e8048_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8970_e8046: f64 = (locals.var_thesat_i * locals.var_phit);
        (assign8970_e8046, ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4)), ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6)), ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7)), ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8)), ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9)),)
    } else {
        (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9,)
    }
};
        locals.var_sat_phit = assign8970_e8048;
        locals.var_sat_phit_dn4 = assign8970_e8048_d_n4;
        locals.var_sat_phit_dn6 = assign8970_e8048_d_n6;
        locals.var_sat_phit_dn7 = assign8970_e8048_d_n7;
        locals.var_sat_phit_dn8 = assign8970_e8048_d_n8;
        locals.var_sat_phit_dn9 = assign8970_e8048_d_n9;

        let (assign8980_e8056, assign8980_e8056_d_n4, assign8980_e8056_d_n6, assign8980_e8056_d_n7, assign8980_e8056_d_n8, assign8980_e8056_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8980_e8052: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
        let assign8980_e8054: f64 = (assign8980_e8052 * locals.var_tf_bet);
        (assign8980_e8054, ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn4)), ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn6)), ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn7)), ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn8)), ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_thesatac_i, locals.var_thesatac_i_dn4, locals.var_thesatac_i_dn6, locals.var_thesatac_i_dn7, locals.var_thesatac_i_dn8, locals.var_thesatac_i_dn9,)
    }
};
        locals.var_thesatac_i = assign8980_e8056;
        locals.var_thesatac_i_dn4 = assign8980_e8056_d_n4;
        locals.var_thesatac_i_dn6 = assign8980_e8056_d_n6;
        locals.var_thesatac_i_dn7 = assign8980_e8056_d_n7;
        locals.var_thesatac_i_dn8 = assign8980_e8056_d_n8;
        locals.var_thesatac_i_dn9 = assign8980_e8056_d_n9;

        let (assign8990_e8062, assign8990_e8062_d_n4, assign8990_e8062_d_n6, assign8990_e8062_d_n7, assign8990_e8062_d_n8, assign8990_e8062_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8990_e8060: f64 = (locals.var_thesatac_i * locals.var_phit);
        (assign8990_e8060, ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4)), ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6)), ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7)), ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8)), ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9)),)
    } else {
        (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9,)
    }
};
        locals.var_sat_phit_ac = assign8990_e8062;
        locals.var_sat_phit_ac_dn4 = assign8990_e8062_d_n4;
        locals.var_sat_phit_ac_dn6 = assign8990_e8062_d_n6;
        locals.var_sat_phit_ac_dn7 = assign8990_e8062_d_n7;
        locals.var_sat_phit_ac_dn8 = assign8990_e8062_d_n8;
        locals.var_sat_phit_ac_dn9 = assign8990_e8062_d_n9;

        let (assign9000_e8068, assign9000_e8068_d_n4, assign9000_e8068_d_n6, assign9000_e8068_d_n7, assign9000_e8068_d_n8, assign9000_e8068_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9000_e8066: f64 = (locals.var_alp1_i * locals.var_inv_phit);
        (assign9000_e8066, (locals.var_alp1_i * locals.var_inv_phit_dn4), (locals.var_alp1_i * locals.var_inv_phit_dn6), (locals.var_alp1_i * locals.var_inv_phit_dn7), (locals.var_alp1_i * locals.var_inv_phit_dn8), (locals.var_alp1_i * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_alp1_phit, locals.var_alp1_phit_dn4, locals.var_alp1_phit_dn6, locals.var_alp1_phit_dn7, locals.var_alp1_phit_dn8, locals.var_alp1_phit_dn9,)
    }
};
        locals.var_alp1_phit = assign9000_e8068;
        locals.var_alp1_phit_dn4 = assign9000_e8068_d_n4;
        locals.var_alp1_phit_dn6 = assign9000_e8068_d_n6;
        locals.var_alp1_phit_dn7 = assign9000_e8068_d_n7;
        locals.var_alp1_phit_dn8 = assign9000_e8068_d_n8;
        locals.var_alp1_phit_dn9 = assign9000_e8068_d_n9;

        let (assign9010_e8076,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9010_e8071: f64 = (-locals.var_stig_i);
        let assign9010_e8073: f64 = (assign9010_e8071 * locals.var_lnrtn);
        let assign9010_e8074: f64 = (assign9010_e8073).exp();
        (assign9010_e8074,)
    } else {
        (locals.var_tf_ig,)
    }
};
        locals.var_tf_ig = assign9010_e8076;

        let (assign9020_e8082,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9020_e8080: f64 = (locals.var_iginv_t * locals.var_tf_ig);
        (assign9020_e8080,)
    } else {
        (locals.var_iginv_i,)
    }
};
        locals.var_iginv_i = assign9020_e8082;

        let (assign9030_e8088,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9030_e8086: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
        (assign9030_e8086,)
    } else {
        (locals.var_igovinv_i,)
    }
};
        locals.var_igovinv_i = assign9030_e8088;

        let (assign9040_e8094,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9040_e8092: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
        (assign9040_e8092,)
    } else {
        (locals.var_igovinvd_i,)
    }
};
        locals.var_igovinvd_i = assign9040_e8094;

        let (assign9050_e8100,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9050_e8098: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
        (assign9050_e8098,)
    } else {
        (locals.var_igovacc_i,)
    }
};
        locals.var_igovacc_i = assign9050_e8100;

        let (assign9060_e8106,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9060_e8104: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
        (assign9060_e8104,)
    } else {
        (locals.var_igovaccd_i,)
    }
};
        locals.var_igovaccd_i = assign9060_e8106;

        let (assign9070_e8114,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9070_e8109: f64 = (-locals.var_stigfn_i);
        let assign9070_e8111: f64 = (assign9070_e8109 * locals.var_lnrtn);
        let assign9070_e8112: f64 = (assign9070_e8111).exp();
        (assign9070_e8112,)
    } else {
        (locals.var_tf_ig,)
    }
};
        locals.var_tf_ig = assign9070_e8114;

        let (assign9100_e8132, assign9100_e8132_d_n4, assign9100_e8132_d_n6, assign9100_e8132_d_n7, assign9100_e8132_d_n8, assign9100_e8132_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9100_e8130: f64 = (0.5 * locals.var_eg);
        (assign9100_e8130, (0.5 * locals.var_eg_dn4), (0.5 * locals.var_eg_dn6), (0.5 * locals.var_eg_dn7), (0.5 * locals.var_eg_dn8), (0.5 * locals.var_eg_dn9),)
    } else {
        (locals.var_alpha_b, locals.var_alpha_b_dn4, locals.var_alpha_b_dn6, locals.var_alpha_b_dn7, locals.var_alpha_b_dn8, locals.var_alpha_b_dn9,)
    }
};
        locals.var_alpha_b = assign9100_e8132;
        locals.var_alpha_b_dn4 = assign9100_e8132_d_n4;
        locals.var_alpha_b_dn6 = assign9100_e8132_d_n6;
        locals.var_alpha_b_dn7 = assign9100_e8132_d_n7;
        locals.var_alpha_b_dn8 = assign9100_e8132_d_n8;
        locals.var_alpha_b_dn9 = assign9100_e8132_d_n9;

        let (assign9110_e8138, assign9110_e8138_d_n4, assign9110_e8138_d_n6, assign9110_e8138_d_n7, assign9110_e8138_d_n8, assign9110_e8138_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9110_e8136: f64 = (locals.var_gco_i * locals.var_phit);
        (assign9110_e8136, (locals.var_gco_i * locals.var_phit_dn4), (locals.var_gco_i * locals.var_phit_dn6), (locals.var_gco_i * locals.var_phit_dn7), (locals.var_gco_i * locals.var_phit_dn8), (locals.var_gco_i * locals.var_phit_dn9),)
    } else {
        (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9,)
    }
};
        locals.var_dch = assign9110_e8138;
        locals.var_dch_dn4 = assign9110_e8138_d_n4;
        locals.var_dch_dn6 = assign9110_e8138_d_n6;
        locals.var_dch_dn7 = assign9110_e8138_d_n7;
        locals.var_dch_dn8 = assign9110_e8138_d_n8;
        locals.var_dch_dn9 = assign9110_e8138_d_n9;

        let (assign9120_e8144, assign9120_e8144_d_n4, assign9120_e8144_d_n6, assign9120_e8144_d_n7, assign9120_e8144_d_n8, assign9120_e8144_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9120_e8142: f64 = (locals.var_gco_i * locals.var_phit0);
        (assign9120_e8142, (locals.var_gco_i * locals.var_phit0_dn4), (locals.var_gco_i * locals.var_phit0_dn6), (locals.var_gco_i * locals.var_phit0_dn7), (locals.var_gco_i * locals.var_phit0_dn8), (locals.var_gco_i * locals.var_phit0_dn9),)
    } else {
        (locals.var_dov, locals.var_dov_dn4, locals.var_dov_dn6, locals.var_dov_dn7, locals.var_dov_dn8, locals.var_dov_dn9,)
    }
};
        locals.var_dov = assign9120_e8144;
        locals.var_dov_dn4 = assign9120_e8144_d_n4;
        locals.var_dov_dn6 = assign9120_e8144_d_n6;
        locals.var_dov_dn7 = assign9120_e8144_d_n7;
        locals.var_dov_dn8 = assign9120_e8144_d_n8;
        locals.var_dov_dn9 = assign9120_e8144_d_n9;

        let (assign9130_e8154, assign9130_e8154_d_n4, assign9130_e8154_d_n6, assign9130_e8154_d_n7, assign9130_e8154_d_n8, assign9130_e8154_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9130_e8150: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
        let assign9130_e8151: f64 = (1.0 + assign9130_e8150);
        let assign9130_e8152: f64 = (1.0 / assign9130_e8151);
        (assign9130_e8152, (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign9130_e8151 * assign9130_e8151))),)
    } else {
        (locals.var_n_iginv, locals.var_n_iginv_dn4, locals.var_n_iginv_dn6, locals.var_n_iginv_dn7, locals.var_n_iginv_dn8, locals.var_n_iginv_dn9,)
    }
};
        locals.var_n_iginv = assign9130_e8154;
        locals.var_n_iginv_dn4 = assign9130_e8154_d_n4;
        locals.var_n_iginv_dn6 = assign9130_e8154_d_n6;
        locals.var_n_iginv_dn7 = assign9130_e8154_d_n7;
        locals.var_n_iginv_dn8 = assign9130_e8154_d_n8;
        locals.var_n_iginv_dn9 = assign9130_e8154_d_n9;

        let (assign9140_e8160, assign9140_e8160_d_n4, assign9140_e8160_d_n6, assign9140_e8160_d_n7, assign9140_e8160_d_n8, assign9140_e8160_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9140_e8158: f64 = (locals.var_toxp_i * 500000000.0);
        (assign9140_e8158, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign9140_e8160;
        locals.var_temp_dn4 = assign9140_e8160_d_n4;
        locals.var_temp_dn6 = assign9140_e8160_d_n6;
        locals.var_temp_dn7 = assign9140_e8160_d_n7;
        locals.var_temp_dn8 = assign9140_e8160_d_n8;
        locals.var_temp_dn9 = assign9140_e8160_d_n9;

        let (assign9150_e8191, assign9150_e8191_d_n4, assign9150_e8191_d_n6, assign9150_e8191_d_n7, assign9150_e8191_d_n8, assign9150_e8191_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9150_e8166: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9150_e8167: f64 = (1.0 + assign9150_e8166);
        let assign9150_e8169: f64 = assign9150_e8167;
        let assign9150_e8173: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9150_e8174: f64 = (1.0 + assign9150_e8173);
        let assign9150_e8176: f64 = assign9150_e8174;
        let assign9150_e8180: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9150_e8181: f64 = (1.0 + assign9150_e8180);
        let assign9150_e8183: f64 = assign9150_e8181;
        let assign9150_e8184: f64 = (assign9150_e8176 * assign9150_e8183);
        let assign9150_e8186: f64 = (assign9150_e8184 + 0.01);
        let assign9150_e8187: f64 = (assign9150_e8186).sqrt();
        let assign9150_e8188: f64 = (assign9150_e8169 + assign9150_e8187);
        let assign9150_e8189: f64 = (0.5 * assign9150_e8188);
        (assign9150_e8189, (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign9150_e8187)))),)
    } else {
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9,)
    }
};
        locals.var_tempm = assign9150_e8191;
        locals.var_tempm_dn4 = assign9150_e8191_d_n4;
        locals.var_tempm_dn6 = assign9150_e8191_d_n6;
        locals.var_tempm_dn7 = assign9150_e8191_d_n7;
        locals.var_tempm_dn8 = assign9150_e8191_d_n8;
        locals.var_tempm_dn9 = assign9150_e8191_d_n9;

        let (assign9160_e8199, assign9160_e8199_d_n4, assign9160_e8199_d_n6, assign9160_e8199_d_n7, assign9160_e8199_d_n8, assign9160_e8199_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9160_e8195: f64 = (locals.var_bgidl_t * locals.var_tempm);
        let assign9160_e8197: f64 = (assign9160_e8195 * locals.var_temp);
        (assign9160_e8197, (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn4)), (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn6)), (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn7)), (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn8)), (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn9)),)
    } else {
        (locals.var_bgidl_i, locals.var_bgidl_i_dn4, locals.var_bgidl_i_dn6, locals.var_bgidl_i_dn7, locals.var_bgidl_i_dn8, locals.var_bgidl_i_dn9,)
    }
};
        locals.var_bgidl_i = assign9160_e8199;
        locals.var_bgidl_i_dn4 = assign9160_e8199_d_n4;
        locals.var_bgidl_i_dn6 = assign9160_e8199_d_n6;
        locals.var_bgidl_i_dn7 = assign9160_e8199_d_n7;
        locals.var_bgidl_i_dn8 = assign9160_e8199_d_n8;
        locals.var_bgidl_i_dn9 = assign9160_e8199_d_n9;

        let (assign9170_e8230, assign9170_e8230_d_n4, assign9170_e8230_d_n6, assign9170_e8230_d_n7, assign9170_e8230_d_n8, assign9170_e8230_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9170_e8205: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9170_e8206: f64 = (1.0 + assign9170_e8205);
        let assign9170_e8208: f64 = assign9170_e8206;
        let assign9170_e8212: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9170_e8213: f64 = (1.0 + assign9170_e8212);
        let assign9170_e8215: f64 = assign9170_e8213;
        let assign9170_e8219: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9170_e8220: f64 = (1.0 + assign9170_e8219);
        let assign9170_e8222: f64 = assign9170_e8220;
        let assign9170_e8223: f64 = (assign9170_e8215 * assign9170_e8222);
        let assign9170_e8225: f64 = (assign9170_e8223 + 0.01);
        let assign9170_e8226: f64 = (assign9170_e8225).sqrt();
        let assign9170_e8227: f64 = (assign9170_e8208 + assign9170_e8226);
        let assign9170_e8228: f64 = (0.5 * assign9170_e8227);
        (assign9170_e8228, (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign9170_e8226)))),)
    } else {
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9,)
    }
};
        locals.var_tempm = assign9170_e8230;
        locals.var_tempm_dn4 = assign9170_e8230_d_n4;
        locals.var_tempm_dn6 = assign9170_e8230_d_n6;
        locals.var_tempm_dn7 = assign9170_e8230_d_n7;
        locals.var_tempm_dn8 = assign9170_e8230_d_n8;
        locals.var_tempm_dn9 = assign9170_e8230_d_n9;

        let (assign9180_e8238, assign9180_e8238_d_n4, assign9180_e8238_d_n6, assign9180_e8238_d_n7, assign9180_e8238_d_n8, assign9180_e8238_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9180_e8234: f64 = (locals.var_bgidld_t * locals.var_tempm);
        let assign9180_e8236: f64 = (assign9180_e8234 * locals.var_temp);
        (assign9180_e8236, (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn4)), (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn6)), (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn7)), (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn8)), (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn9)),)
    } else {
        (locals.var_bgidld_i, locals.var_bgidld_i_dn4, locals.var_bgidld_i_dn6, locals.var_bgidld_i_dn7, locals.var_bgidld_i_dn8, locals.var_bgidld_i_dn9,)
    }
};
        locals.var_bgidld_i = assign9180_e8238;
        locals.var_bgidld_i_dn4 = assign9180_e8238_d_n4;
        locals.var_bgidld_i_dn6 = assign9180_e8238_d_n6;
        locals.var_bgidld_i_dn7 = assign9180_e8238_d_n7;
        locals.var_bgidld_i_dn8 = assign9180_e8238_d_n8;
        locals.var_bgidld_i_dn9 = assign9180_e8238_d_n9;

        let (assign9190_e8248, assign9190_e8248_d_n4, assign9190_e8248_d_n6, assign9190_e8248_d_n7, assign9190_e8248_d_n8, assign9190_e8248_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9190_e8242: f64 = (-locals.var_sta2_i);
        let assign9190_e8244: f64 = (assign9190_e8242 * locals.var_lnrtn);
        let assign9190_e8245: f64 = (assign9190_e8244).exp();
        let assign9190_e8246: f64 = (locals.var_a2_t * assign9190_e8245);
        (assign9190_e8246, (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn4))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn6))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn7))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn8))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn9))),)
    } else {
        (locals.var_a2_i, locals.var_a2_i_dn4, locals.var_a2_i_dn6, locals.var_a2_i_dn7, locals.var_a2_i_dn8, locals.var_a2_i_dn9,)
    }
};
        locals.var_a2_i = assign9190_e8248;
        locals.var_a2_i_dn4 = assign9190_e8248_d_n4;
        locals.var_a2_i_dn6 = assign9190_e8248_d_n6;
        locals.var_a2_i_dn7 = assign9190_e8248_d_n7;
        locals.var_a2_i_dn8 = assign9190_e8248_d_n8;
        locals.var_a2_i_dn9 = assign9190_e8248_d_n9;

        let (assign9200_e8254, assign9200_e8254_d_n4, assign9200_e8254_d_n6, assign9200_e8254_d_n7, assign9200_e8254_d_n8, assign9200_e8254_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9200_e8252: f64 = (locals.var_areaq_i * locals.var_phit);
        (assign9200_e8252, (locals.var_areaq_i * locals.var_phit_dn4), (locals.var_areaq_i * locals.var_phit_dn6), (locals.var_areaq_i * locals.var_phit_dn7), (locals.var_areaq_i * locals.var_phit_dn8), (locals.var_areaq_i * locals.var_phit_dn9),)
    } else {
        (locals.var_area_phit, locals.var_area_phit_dn4, locals.var_area_phit_dn6, locals.var_area_phit_dn7, locals.var_area_phit_dn8, locals.var_area_phit_dn9,)
    }
};
        locals.var_area_phit = assign9200_e8254;
        locals.var_area_phit_dn4 = assign9200_e8254_d_n4;
        locals.var_area_phit_dn6 = assign9200_e8254_d_n6;
        locals.var_area_phit_dn7 = assign9200_e8254_d_n7;
        locals.var_area_phit_dn8 = assign9200_e8254_d_n8;
        locals.var_area_phit_dn9 = assign9200_e8254_d_n9;

        let (assign9210_e8266, assign9210_e8266_d_n4, assign9210_e8266_d_n6, assign9210_e8266_d_n7, assign9210_e8266_d_n8, assign9210_e8266_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9210_e8258: f64 = (0.25 * 1.602176565e-19);
        let assign9210_e8260: f64 = (assign9210_e8258 * locals.var_nsdac_i);
        let assign9210_e8263: f64 = (locals.var_epsch * locals.var_phit);
        let assign9210_e8264: f64 = (assign9210_e8260 / assign9210_e8263);
        (assign9210_e8264, (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn4)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn6)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn7)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn8)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn9)) / (assign9210_e8263 * assign9210_e8263))),)
    } else {
        (locals.var_inner_sd, locals.var_inner_sd_dn4, locals.var_inner_sd_dn6, locals.var_inner_sd_dn7, locals.var_inner_sd_dn8, locals.var_inner_sd_dn9,)
    }
};
        locals.var_inner_sd = assign9210_e8266;
        locals.var_inner_sd_dn4 = assign9210_e8266_d_n4;
        locals.var_inner_sd_dn6 = assign9210_e8266_d_n6;
        locals.var_inner_sd_dn7 = assign9210_e8266_d_n7;
        locals.var_inner_sd_dn8 = assign9210_e8266_d_n8;
        locals.var_inner_sd_dn9 = assign9210_e8266_d_n9;

        let (assign9220_e8273, assign9220_e8273_d_n4, assign9220_e8273_d_n6, assign9220_e8273_d_n7, assign9220_e8273_d_n8, assign9220_e8273_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9220_e8270: f64 = (locals.var_nsdac_i / locals.var_neff);
        let assign9220_e8271: f64 = (assign9220_e8270).ln();
        (assign9220_e8271, ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign9220_e8270),)
    } else {
        (locals.var_xsd, locals.var_xsd_dn4, locals.var_xsd_dn6, locals.var_xsd_dn7, locals.var_xsd_dn8, locals.var_xsd_dn9,)
    }
};
        locals.var_xsd = assign9220_e8273;
        locals.var_xsd_dn4 = assign9220_e8273_d_n4;
        locals.var_xsd_dn6 = assign9220_e8273_d_n6;
        locals.var_xsd_dn7 = assign9220_e8273_d_n7;
        locals.var_xsd_dn8 = assign9220_e8273_d_n8;
        locals.var_xsd_dn9 = assign9220_e8273_d_n9;

        let (assign9230_e8281, assign9230_e8281_d_n4, assign9230_e8281_d_n6, assign9230_e8281_d_n7, assign9230_e8281_d_n8, assign9230_e8281_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9230_e8277: f64 = (locals.var_fif_i * 1.25e-6);
        let assign9230_e8279: f64 = (assign9230_e8277 * locals.var_phit);
        (assign9230_e8279, (assign9230_e8277 * locals.var_phit_dn4), (assign9230_e8277 * locals.var_phit_dn6), (assign9230_e8277 * locals.var_phit_dn7), (assign9230_e8277 * locals.var_phit_dn8), (assign9230_e8277 * locals.var_phit_dn9),)
    } else {
        (locals.var_fif_phit, locals.var_fif_phit_dn4, locals.var_fif_phit_dn6, locals.var_fif_phit_dn7, locals.var_fif_phit_dn8, locals.var_fif_phit_dn9,)
    }
};
        locals.var_fif_phit = assign9230_e8281;
        locals.var_fif_phit_dn4 = assign9230_e8281_d_n4;
        locals.var_fif_phit_dn6 = assign9230_e8281_d_n6;
        locals.var_fif_phit_dn7 = assign9230_e8281_d_n7;
        locals.var_fif_phit_dn8 = assign9230_e8281_d_n8;
        locals.var_fif_phit_dn9 = assign9230_e8281_d_n9;

        let (assign9240_e8288, assign9240_e8288_d_n4, assign9240_e8288_d_n6, assign9240_e8288_d_n7, assign9240_e8288_d_n8, assign9240_e8288_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9240_e8285: f64 = (locals.var_strth_i * locals.var_lnrtn);
        let assign9240_e8286: f64 = (assign9240_e8285).exp();
        (assign9240_e8286, (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn4)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn6)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn7)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn8)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_rth, locals.var_tf_rth_dn4, locals.var_tf_rth_dn6, locals.var_tf_rth_dn7, locals.var_tf_rth_dn8, locals.var_tf_rth_dn9,)
    }
};
        locals.var_tf_rth = assign9240_e8288;
        locals.var_tf_rth_dn4 = assign9240_e8288_d_n4;
        locals.var_tf_rth_dn6 = assign9240_e8288_d_n6;
        locals.var_tf_rth_dn7 = assign9240_e8288_d_n7;
        locals.var_tf_rth_dn8 = assign9240_e8288_d_n8;
        locals.var_tf_rth_dn9 = assign9240_e8288_d_n9;

        let (assign9250_e8294, assign9250_e8294_d_n4, assign9250_e8294_d_n6, assign9250_e8294_d_n7, assign9250_e8294_d_n8, assign9250_e8294_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9250_e8292: f64 = (locals.var_rth_t * locals.var_tf_rth);
        (assign9250_e8292, ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4)), ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6)), ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7)), ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8)), ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9)),)
    } else {
        (locals.var_rth_i, locals.var_rth_i_dn4, locals.var_rth_i_dn6, locals.var_rth_i_dn7, locals.var_rth_i_dn8, locals.var_rth_i_dn9,)
    }
};
        locals.var_rth_i = assign9250_e8294;
        locals.var_rth_i_dn4 = assign9250_e8294_d_n4;
        locals.var_rth_i_dn6 = assign9250_e8294_d_n6;
        locals.var_rth_i_dn7 = assign9250_e8294_d_n7;
        locals.var_rth_i_dn8 = assign9250_e8294_d_n8;
        locals.var_rth_i_dn9 = assign9250_e8294_d_n9;

        let (assign9260_e8302,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9260_e8298: f64 = (4.0 * 1.3806488e-23);
        let assign9260_e8300: f64 = (assign9260_e8298 * locals.var_tkc);
        (assign9260_e8300,)
    } else {
        (locals.var_nt0_4kt,)
    }
};
        locals.var_nt0_4kt = assign9260_e8302;

    }

    pub(super) fn stamp_transient_block_20(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (assign9270_e8308,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9270_e8306: f64 = (locals.var_fnt_i * locals.var_nt0_4kt);
        (assign9270_e8306,)
    } else {
        (locals.var_nt,)
    }
};
        locals.var_nt = assign9270_e8308;

        let assign9280_e8311: f64 = 1.0;
        let assign9280_e8312: f64 = if p.p14 == assign9280_e8311 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign9280_e8312;

        let (assign9290_e8316, assign9290_e8316_d_n6, assign9290_e8316_d_n9,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv9 - nv6), -1.0, 1.0,)
    } else {
        (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9,)
    }
};
        locals.var_vgsu = assign9290_e8316;
        locals.var_vgsu_dn6 = assign9290_e8316_d_n6;
        locals.var_vgsu_dn9 = assign9290_e8316_d_n9;

        let (assign9300_e8320, assign9300_e8320_d_n6, assign9300_e8320_d_n7,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv7 - nv6), -1.0, 1.0,)
    } else {
        (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7,)
    }
};
        locals.var_vdsu = assign9300_e8320;
        locals.var_vdsu_dn6 = assign9300_e8320_d_n6;
        locals.var_vdsu_dn7 = assign9300_e8320_d_n7;

        let (assign9310_e8324, assign9310_e8324_d_n6, assign9310_e8324_d_n8,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv6 - nv8), 1.0, -1.0,)
    } else {
        (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8,)
    }
};
        locals.var_vsbu = assign9310_e8324;
        locals.var_vsbu_dn6 = assign9310_e8324_d_n6;
        locals.var_vsbu_dn8 = assign9310_e8324_d_n8;

        let (assign9320_e8330, assign9320_e8330_d_n6, assign9320_e8330_d_n9,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9320_e8328: f64 = (-(nv9 - nv6));
        (assign9320_e8328, 1.0, (-1.0),)
    } else {
        (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9,)
    }
};
        locals.var_vgsu = assign9320_e8330;
        locals.var_vgsu_dn6 = assign9320_e8330_d_n6;
        locals.var_vgsu_dn9 = assign9320_e8330_d_n9;

        let (assign9330_e8336, assign9330_e8336_d_n6, assign9330_e8336_d_n7,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9330_e8334: f64 = (-(nv7 - nv6));
        (assign9330_e8334, 1.0, (-1.0),)
    } else {
        (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7,)
    }
};
        locals.var_vdsu = assign9330_e8336;
        locals.var_vdsu_dn6 = assign9330_e8336_d_n6;
        locals.var_vdsu_dn7 = assign9330_e8336_d_n7;

        let (assign9340_e8342, assign9340_e8342_d_n6, assign9340_e8342_d_n8,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9340_e8340: f64 = (-(nv6 - nv8));
        (assign9340_e8340, (-1.0), 1.0,)
    } else {
        (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8,)
    }
};
        locals.var_vsbu = assign9340_e8342;
        locals.var_vsbu_dn6 = assign9340_e8342_d_n6;
        locals.var_vsbu_dn8 = assign9340_e8342_d_n8;

        let assign9350_e8344: f64 = (-locals.var_vdsu);
        locals.var_vsdu = assign9350_e8344;
        locals.var_vsdu_dn6 = (-locals.var_vdsu_dn6);
        locals.var_vsdu_dn7 = (-locals.var_vdsu_dn7);

        let assign9360_e8347: f64 = (locals.var_vgsu + locals.var_vsdu);
        locals.var_vgdu = assign9360_e8347;
        locals.var_vgdu_dn6 = (locals.var_vgsu_dn6 + locals.var_vsdu_dn6);
        locals.var_vgdu_dn7 = locals.var_vsdu_dn7;
        locals.var_vgdu_dn9 = locals.var_vgsu_dn9;

        let assign9370_e8350: f64 = (locals.var_vdsu + locals.var_vsbu);
        locals.var_vdbu = assign9370_e8350;
        locals.var_vdbu_dn6 = (locals.var_vdsu_dn6 + locals.var_vsbu_dn6);
        locals.var_vdbu_dn7 = locals.var_vdsu_dn7;
        locals.var_vdbu_dn8 = locals.var_vsbu_dn8;

        let assign9380_e8353: f64 = if locals.var_vdsu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign9380_e8353;

        let (assign9390_e8358,) = {
    if (locals.var_guard264 != 0.0) {
        let assign9390_e8356: f64 = (-1.0);
        (assign9390_e8356,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign9390_e8358;

        let (assign9400_e8362, assign9400_e8362_d_n6, assign9400_e8362_d_n7,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_vsdu, locals.var_vsdu_dn6, locals.var_vsdu_dn7,)
    } else {
        (locals.var_vds, locals.var_vds_dn6, locals.var_vds_dn7,)
    }
};
        locals.var_vds = assign9400_e8362;
        locals.var_vds_dn6 = assign9400_e8362_d_n6;
        locals.var_vds_dn7 = assign9400_e8362_d_n7;

        let (assign9410_e8366, assign9410_e8366_d_n6, assign9410_e8366_d_n7, assign9410_e8366_d_n9,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_vgdu, locals.var_vgdu_dn6, locals.var_vgdu_dn7, locals.var_vgdu_dn9,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn9,)
    }
};
        locals.var_vgs = assign9410_e8366;
        locals.var_vgs_dn6 = assign9410_e8366_d_n6;
        locals.var_vgs_dn7 = assign9410_e8366_d_n7;
        locals.var_vgs_dn9 = assign9410_e8366_d_n9;

        let (assign9420_e8370, assign9420_e8370_d_n6, assign9420_e8370_d_n7, assign9420_e8370_d_n8,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_vdbu, locals.var_vdbu_dn6, locals.var_vdbu_dn7, locals.var_vdbu_dn8,)
    } else {
        (locals.var_vsb, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn8,)
    }
};
        locals.var_vsb = assign9420_e8370;
        locals.var_vsb_dn6 = assign9420_e8370_d_n6;
        locals.var_vsb_dn7 = assign9420_e8370_d_n7;
        locals.var_vsb_dn8 = assign9420_e8370_d_n8;

        let (assign9430_e8375,) = {
    if (locals.var_guard264 == 0.0) {
        (1.0,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign9430_e8375;

        let (assign9440_e8380, assign9440_e8380_d_n6, assign9440_e8380_d_n7,) = {
    if (locals.var_guard264 == 0.0) {
        (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7,)
    } else {
        (locals.var_vds, locals.var_vds_dn6, locals.var_vds_dn7,)
    }
};
        locals.var_vds = assign9440_e8380;
        locals.var_vds_dn6 = assign9440_e8380_d_n6;
        locals.var_vds_dn7 = assign9440_e8380_d_n7;

        let (assign9450_e8385, assign9450_e8385_d_n6, assign9450_e8385_d_n7, assign9450_e8385_d_n9,) = {
    if (locals.var_guard264 == 0.0) {
        (locals.var_vgsu, locals.var_vgsu_dn6, 0.0, locals.var_vgsu_dn9,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn9,)
    }
};
        locals.var_vgs = assign9450_e8385;
        locals.var_vgs_dn6 = assign9450_e8385_d_n6;
        locals.var_vgs_dn7 = assign9450_e8385_d_n7;
        locals.var_vgs_dn9 = assign9450_e8385_d_n9;

        let (assign9460_e8390, assign9460_e8390_d_n6, assign9460_e8390_d_n7, assign9460_e8390_d_n8,) = {
    if (locals.var_guard264 == 0.0) {
        (locals.var_vsbu, locals.var_vsbu_dn6, 0.0, locals.var_vsbu_dn8,)
    } else {
        (locals.var_vsb, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn8,)
    }
};
        locals.var_vsb = assign9460_e8390;
        locals.var_vsb_dn6 = assign9460_e8390_d_n6;
        locals.var_vsb_dn7 = assign9460_e8390_d_n7;
        locals.var_vsb_dn8 = assign9460_e8390_d_n8;

        let assign9470_e8393: f64 = (locals.var_vgs + locals.var_vsb);
        locals.var_vgb = assign9470_e8393;
        locals.var_vgb_dn6 = (locals.var_vgs_dn6 + locals.var_vsb_dn6);
        locals.var_vgb_dn7 = (locals.var_vgs_dn7 + locals.var_vsb_dn7);
        locals.var_vgb_dn8 = locals.var_vsb_dn8;
        locals.var_vgb_dn9 = locals.var_vgs_dn9;

        let assign9480_e8396: f64 = (locals.var_vds * locals.var_inv_phit);
        locals.var_xd = assign9480_e8396;
        locals.var_xd_dn4 = (locals.var_vds * locals.var_inv_phit_dn4);
        locals.var_xd_dn6 = ((locals.var_vds_dn6 * locals.var_inv_phit) + (locals.var_vds * locals.var_inv_phit_dn6));
        locals.var_xd_dn7 = ((locals.var_vds_dn7 * locals.var_inv_phit) + (locals.var_vds * locals.var_inv_phit_dn7));
        locals.var_xd_dn8 = (locals.var_vds * locals.var_inv_phit_dn8);
        locals.var_xd_dn9 = (locals.var_vds * locals.var_inv_phit_dn9);

        let assign9490_e8399: f64 = (locals.var_vds * locals.var_vds);
        let assign9490_e8401: f64 = (assign9490_e8399 + 0.01);
        let assign9490_e8402: f64 = (assign9490_e8401).sqrt();
        let assign9490_e8404: f64 = (assign9490_e8402 - 0.1);
        let assign9490_e8406: f64 = (assign9490_e8404 * locals.var_inv_phit);
        locals.var_xdsx = assign9490_e8406;
        locals.var_xdsx_dn4 = (assign9490_e8404 * locals.var_inv_phit_dn4);
        locals.var_xdsx_dn6 = (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign9490_e8402)) * locals.var_inv_phit) + (assign9490_e8404 * locals.var_inv_phit_dn6));
        locals.var_xdsx_dn7 = (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign9490_e8402)) * locals.var_inv_phit) + (assign9490_e8404 * locals.var_inv_phit_dn7));
        locals.var_xdsx_dn8 = (assign9490_e8404 * locals.var_inv_phit_dn8);
        locals.var_xdsx_dn9 = (assign9490_e8404 * locals.var_inv_phit_dn9);

        let assign9500_e8410: f64 = (locals.var_xd - locals.var_xdsx);
        let assign9500_e8411: f64 = (0.5 * assign9500_e8410);
        locals.var_dxdsx = assign9500_e8411;
        locals.var_dxdsx_dn4 = (0.5 * (locals.var_xd_dn4 - locals.var_xdsx_dn4));
        locals.var_dxdsx_dn6 = (0.5 * (locals.var_xd_dn6 - locals.var_xdsx_dn6));
        locals.var_dxdsx_dn7 = (0.5 * (locals.var_xd_dn7 - locals.var_xdsx_dn7));
        locals.var_dxdsx_dn8 = (0.5 * (locals.var_xd_dn8 - locals.var_xdsx_dn8));
        locals.var_dxdsx_dn9 = (0.5 * (locals.var_xd_dn9 - locals.var_xdsx_dn9));

        locals.var_vfb1_loc = locals.var_vfb1_i;
        locals.var_vfb1_loc_dn4 = locals.var_vfb1_i_dn4;
        locals.var_vfb1_loc_dn6 = locals.var_vfb1_i_dn6;
        locals.var_vfb1_loc_dn7 = locals.var_vfb1_i_dn7;
        locals.var_vfb1_loc_dn8 = locals.var_vfb1_i_dn8;
        locals.var_vfb1_loc_dn9 = locals.var_vfb1_i_dn9;

        locals.var_vfb2_loc = locals.var_vfb2_i;
        locals.var_vfb2_loc_dn4 = locals.var_vfb2_i_dn4;
        locals.var_vfb2_loc_dn6 = locals.var_vfb2_i_dn6;
        locals.var_vfb2_loc_dn7 = locals.var_vfb2_i_dn7;
        locals.var_vfb2_loc_dn8 = locals.var_vfb2_i_dn8;
        locals.var_vfb2_loc_dn9 = locals.var_vfb2_i_dn9;

        locals.var_psce1_loc = locals.var_psce1_i;

        locals.var_psce2_loc = locals.var_psce2_i;

        locals.var_cf1_loc = locals.var_cf1_i;
        locals.var_cf1_loc_dn4 = locals.var_cf1_i_dn4;
        locals.var_cf1_loc_dn6 = locals.var_cf1_i_dn6;
        locals.var_cf1_loc_dn7 = locals.var_cf1_i_dn7;
        locals.var_cf1_loc_dn8 = locals.var_cf1_i_dn8;
        locals.var_cf1_loc_dn9 = locals.var_cf1_i_dn9;

        locals.var_cf2_loc = locals.var_cf2_i;
        locals.var_cf2_loc_dn4 = locals.var_cf2_i_dn4;
        locals.var_cf2_loc_dn6 = locals.var_cf2_i_dn6;
        locals.var_cf2_loc_dn7 = locals.var_cf2_i_dn7;
        locals.var_cf2_loc_dn8 = locals.var_cf2_i_dn8;
        locals.var_cf2_loc_dn9 = locals.var_cf2_i_dn9;

        locals.var_sat_phit_loc = locals.var_sat_phit;
        locals.var_sat_phit_loc_dn4 = locals.var_sat_phit_dn4;
        locals.var_sat_phit_loc_dn6 = locals.var_sat_phit_dn6;
        locals.var_sat_phit_loc_dn7 = locals.var_sat_phit_dn7;
        locals.var_sat_phit_loc_dn8 = locals.var_sat_phit_dn8;
        locals.var_sat_phit_loc_dn9 = locals.var_sat_phit_dn9;

        locals.var_gamax_loc = locals.var_gamax;

        locals.var_alp_loc = locals.var_alp_i;

        let assign9600_e8423: f64 = (locals.var_vgs - locals.var_vfb1_loc);
        let assign9600_e8425: f64 = (assign9600_e8423 * locals.var_inv_phit);
        let assign9600_e8427: f64 = (assign9600_e8425 - locals.var_dxdsx);
        let assign9600_e8429: f64 = (assign9600_e8427 - locals.var_eg_2phit0);
        locals.var_xg10 = assign9600_e8429;
        locals.var_xg10_dn4 = (((((-locals.var_vfb1_loc_dn4) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4) - locals.var_eg_2phit0_dn4);
        locals.var_xg10_dn6 = (((((locals.var_vgs_dn6 - locals.var_vfb1_loc_dn6) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6) - locals.var_eg_2phit0_dn6);
        locals.var_xg10_dn7 = (((((locals.var_vgs_dn7 - locals.var_vfb1_loc_dn7) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7) - locals.var_eg_2phit0_dn7);
        locals.var_xg10_dn8 = (((((-locals.var_vfb1_loc_dn8) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8) - locals.var_eg_2phit0_dn8);
        locals.var_xg10_dn9 = (((((locals.var_vgs_dn9 - locals.var_vfb1_loc_dn9) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9) - locals.var_eg_2phit0_dn9);

        let assign9610_e8431: f64 = (-locals.var_vsb);
        let assign9610_e8433: f64 = (assign9610_e8431 - locals.var_vfb2_loc);
        let assign9610_e8435: f64 = (assign9610_e8433 * locals.var_inv_phit);
        let assign9610_e8437: f64 = (assign9610_e8435 - locals.var_dxdsx);
        locals.var_xg20shift = assign9610_e8437;
        locals.var_xg20shift_dn4 = ((((-locals.var_vfb2_loc_dn4) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4);
        locals.var_xg20shift_dn6 = (((((-locals.var_vsb_dn6) - locals.var_vfb2_loc_dn6) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6);
        locals.var_xg20shift_dn7 = (((((-locals.var_vsb_dn7) - locals.var_vfb2_loc_dn7) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7);
        locals.var_xg20shift_dn8 = (((((-locals.var_vsb_dn8) - locals.var_vfb2_loc_dn8) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8);
        locals.var_xg20shift_dn9 = ((((-locals.var_vfb2_loc_dn9) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9);

        let assign9620_e8440: f64 = (locals.var_xg20shift - locals.var_eg_2phit0);
        locals.var_xg20 = assign9620_e8440;
        locals.var_xg20_dn4 = (locals.var_xg20shift_dn4 - locals.var_eg_2phit0_dn4);
        locals.var_xg20_dn6 = (locals.var_xg20shift_dn6 - locals.var_eg_2phit0_dn6);
        locals.var_xg20_dn7 = (locals.var_xg20shift_dn7 - locals.var_eg_2phit0_dn7);
        locals.var_xg20_dn8 = (locals.var_xg20shift_dn8 - locals.var_eg_2phit0_dn8);
        locals.var_xg20_dn9 = (locals.var_xg20shift_dn9 - locals.var_eg_2phit0_dn9);

        let assign9630_e8443: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign9630_e8443;

        let (assign9640_e8449, assign9640_e8449_d_n4, assign9640_e8449_d_n6, assign9640_e8449_d_n7, assign9640_e8449_d_n8, assign9640_e8449_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9640_e8447: f64 = (p.p14 * locals.var_typesub_i);
        (assign9640_e8447, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign9640_e8449;
        locals.var_temp_dn4 = assign9640_e8449_d_n4;
        locals.var_temp_dn6 = assign9640_e8449_d_n6;
        locals.var_temp_dn7 = assign9640_e8449_d_n7;
        locals.var_temp_dn8 = assign9640_e8449_d_n8;
        locals.var_temp_dn9 = assign9640_e8449_d_n9;

        let (assign9650_e8459, assign9650_e8459_d_n4, assign9650_e8459_d_n6, assign9650_e8459_d_n7, assign9650_e8459_d_n8, assign9650_e8459_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9650_e8453: f64 = (1.0 + locals.var_k1_1d);
        let assign9650_e8456: f64 = (1.0 + locals.var_k2_1d);
        let assign9650_e8457: f64 = (assign9650_e8453 / assign9650_e8456);
        (assign9650_e8457, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_dxth, locals.var_exp_dxth_dn4, locals.var_exp_dxth_dn6, locals.var_exp_dxth_dn7, locals.var_exp_dxth_dn8, locals.var_exp_dxth_dn9,)
    }
};
        locals.var_exp_dxth = assign9650_e8459;
        locals.var_exp_dxth_dn4 = assign9650_e8459_d_n4;
        locals.var_exp_dxth_dn6 = assign9650_e8459_d_n6;
        locals.var_exp_dxth_dn7 = assign9650_e8459_d_n7;
        locals.var_exp_dxth_dn8 = assign9650_e8459_d_n8;
        locals.var_exp_dxth_dn9 = assign9650_e8459_d_n9;

        let (assign9660_e8464, assign9660_e8464_d_n4, assign9660_e8464_d_n6, assign9660_e8464_d_n7, assign9660_e8464_d_n8, assign9660_e8464_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9660_e8462: f64 = (locals.var_exp_dxth).ln();
        (assign9660_e8462, (locals.var_exp_dxth_dn4 / locals.var_exp_dxth), (locals.var_exp_dxth_dn6 / locals.var_exp_dxth), (locals.var_exp_dxth_dn7 / locals.var_exp_dxth), (locals.var_exp_dxth_dn8 / locals.var_exp_dxth), (locals.var_exp_dxth_dn9 / locals.var_exp_dxth),)
    } else {
        (locals.var_dxth, locals.var_dxth_dn4, locals.var_dxth_dn6, locals.var_dxth_dn7, locals.var_dxth_dn8, locals.var_dxth_dn9,)
    }
};
        locals.var_dxth = assign9660_e8464;
        locals.var_dxth_dn4 = assign9660_e8464_d_n4;
        locals.var_dxth_dn6 = assign9660_e8464_d_n6;
        locals.var_dxth_dn7 = assign9660_e8464_d_n7;
        locals.var_dxth_dn8 = assign9660_e8464_d_n8;
        locals.var_dxth_dn9 = assign9660_e8464_d_n9;

        let assign9670_e8467: f64 = if locals.var_dxth > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign9670_e8467;

        let (assign9680_e8483, assign9680_e8483_d_n4, assign9680_e8483_d_n6, assign9680_e8483_d_n7, assign9680_e8483_d_n8, assign9680_e8483_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard532 != 0.0)) {
        let assign9680_e8473: f64 = (2.0 * locals.var_dxth);
        let assign9680_e8476: f64 = (locals.var_exp_dxth + 1.0);
        let assign9680_e8477: f64 = (assign9680_e8473 * assign9680_e8476);
        let assign9680_e8480: f64 = (locals.var_exp_dxth - 1.0);
        let assign9680_e8481: f64 = (assign9680_e8477 / assign9680_e8480);
        (assign9680_e8481, ((((((2.0 * locals.var_dxth_dn4) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn4)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn4)) / (assign9680_e8480 * assign9680_e8480)), ((((((2.0 * locals.var_dxth_dn6) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn6)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn6)) / (assign9680_e8480 * assign9680_e8480)), ((((((2.0 * locals.var_dxth_dn7) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn7)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn7)) / (assign9680_e8480 * assign9680_e8480)), ((((((2.0 * locals.var_dxth_dn8) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn8)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn8)) / (assign9680_e8480 * assign9680_e8480)), ((((((2.0 * locals.var_dxth_dn9) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn9)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn9)) / (assign9680_e8480 * assign9680_e8480)),)
    } else {
        (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9,)
    }
};
        locals.var_diff_min = assign9680_e8483;
        locals.var_diff_min_dn4 = assign9680_e8483_d_n4;
        locals.var_diff_min_dn6 = assign9680_e8483_d_n6;
        locals.var_diff_min_dn7 = assign9680_e8483_d_n7;
        locals.var_diff_min_dn8 = assign9680_e8483_d_n8;
        locals.var_diff_min_dn9 = assign9680_e8483_d_n9;

        let (assign9690_e8494, assign9690_e8494_d_n4, assign9690_e8494_d_n6, assign9690_e8494_d_n7, assign9690_e8494_d_n8, assign9690_e8494_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard532 == 0.0)) {
        let assign9690_e8491: f64 = (2.0 + locals.var_dxth);
        let assign9690_e8492: f64 = (2.0 * assign9690_e8491);
        (assign9690_e8492, (2.0 * locals.var_dxth_dn4), (2.0 * locals.var_dxth_dn6), (2.0 * locals.var_dxth_dn7), (2.0 * locals.var_dxth_dn8), (2.0 * locals.var_dxth_dn9),)
    } else {
        (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9,)
    }
};
        locals.var_diff_min = assign9690_e8494;
        locals.var_diff_min_dn4 = assign9690_e8494_d_n4;
        locals.var_diff_min_dn6 = assign9690_e8494_d_n6;
        locals.var_diff_min_dn7 = assign9690_e8494_d_n7;
        locals.var_diff_min_dn8 = assign9690_e8494_d_n8;
        locals.var_diff_min_dn9 = assign9690_e8494_d_n9;

        let (assign9700_e8502, assign9700_e8502_d_n4, assign9700_e8502_d_n6, assign9700_e8502_d_n7, assign9700_e8502_d_n8, assign9700_e8502_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9700_e8499: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign9700_e8500: f64 = (locals.var_a0_csisq / assign9700_e8499);
        (assign9700_e8500, (locals.var_a0_csisq_dn4 / assign9700_e8499), (locals.var_a0_csisq_dn6 / assign9700_e8499), (locals.var_a0_csisq_dn7 / assign9700_e8499), (locals.var_a0_csisq_dn8 / assign9700_e8499), (locals.var_a0_csisq_dn9 / assign9700_e8499),)
    } else {
        (locals.var_a0, locals.var_a0_dn4, locals.var_a0_dn6, locals.var_a0_dn7, locals.var_a0_dn8, locals.var_a0_dn9,)
    }
};
        locals.var_a0 = assign9700_e8502;
        locals.var_a0_dn4 = assign9700_e8502_d_n4;
        locals.var_a0_dn6 = assign9700_e8502_d_n6;
        locals.var_a0_dn7 = assign9700_e8502_d_n7;
        locals.var_a0_dn8 = assign9700_e8502_d_n8;
        locals.var_a0_dn9 = assign9700_e8502_d_n9;

        let (assign9710_e8508, assign9710_e8508_d_n4, assign9710_e8508_d_n6, assign9710_e8508_d_n7, assign9710_e8508_d_n8, assign9710_e8508_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9710_e8506: f64 = (1.0 / locals.var_k1_1d);
        (assign9710_e8506, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k1, locals.var_inv_k1_dn4, locals.var_inv_k1_dn6, locals.var_inv_k1_dn7, locals.var_inv_k1_dn8, locals.var_inv_k1_dn9,)
    }
};
        locals.var_inv_k1 = assign9710_e8508;
        locals.var_inv_k1_dn4 = assign9710_e8508_d_n4;
        locals.var_inv_k1_dn6 = assign9710_e8508_d_n6;
        locals.var_inv_k1_dn7 = assign9710_e8508_d_n7;
        locals.var_inv_k1_dn8 = assign9710_e8508_d_n8;
        locals.var_inv_k1_dn9 = assign9710_e8508_d_n9;

        let (assign9720_e8514, assign9720_e8514_d_n4, assign9720_e8514_d_n6, assign9720_e8514_d_n7, assign9720_e8514_d_n8, assign9720_e8514_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9720_e8512: f64 = (1.0 / locals.var_k2_1d);
        (assign9720_e8512, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k2, locals.var_inv_k2_dn4, locals.var_inv_k2_dn6, locals.var_inv_k2_dn7, locals.var_inv_k2_dn8, locals.var_inv_k2_dn9,)
    }
};
        locals.var_inv_k2 = assign9720_e8514;
        locals.var_inv_k2_dn4 = assign9720_e8514_d_n4;
        locals.var_inv_k2_dn6 = assign9720_e8514_d_n6;
        locals.var_inv_k2_dn7 = assign9720_e8514_d_n7;
        locals.var_inv_k2_dn8 = assign9720_e8514_d_n8;
        locals.var_inv_k2_dn9 = assign9720_e8514_d_n9;

        let (assign9730_e8524, assign9730_e8524_d_n4, assign9730_e8524_d_n6, assign9730_e8524_d_n7, assign9730_e8524_d_n8, assign9730_e8524_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9730_e8519: f64 = (1.0 + locals.var_inv_k1);
        let assign9730_e8521: f64 = (assign9730_e8519 + locals.var_inv_k2);
        let assign9730_e8522: f64 = (1.0 / assign9730_e8521);
        (assign9730_e8522, (-((locals.var_inv_k1_dn4 + locals.var_inv_k2_dn4) / (assign9730_e8521 * assign9730_e8521))), (-((locals.var_inv_k1_dn6 + locals.var_inv_k2_dn6) / (assign9730_e8521 * assign9730_e8521))), (-((locals.var_inv_k1_dn7 + locals.var_inv_k2_dn7) / (assign9730_e8521 * assign9730_e8521))), (-((locals.var_inv_k1_dn8 + locals.var_inv_k2_dn8) / (assign9730_e8521 * assign9730_e8521))), (-((locals.var_inv_k1_dn9 + locals.var_inv_k2_dn9) / (assign9730_e8521 * assign9730_e8521))),)
    } else {
        (locals.var_keq, locals.var_keq_dn4, locals.var_keq_dn6, locals.var_keq_dn7, locals.var_keq_dn8, locals.var_keq_dn9,)
    }
};
        locals.var_keq = assign9730_e8524;
        locals.var_keq_dn4 = assign9730_e8524_d_n4;
        locals.var_keq_dn6 = assign9730_e8524_d_n6;
        locals.var_keq_dn7 = assign9730_e8524_d_n7;
        locals.var_keq_dn8 = assign9730_e8524_d_n8;
        locals.var_keq_dn9 = assign9730_e8524_d_n9;

        let (assign9740_e8532, assign9740_e8532_d_n4, assign9740_e8532_d_n6, assign9740_e8532_d_n7, assign9740_e8532_d_n8, assign9740_e8532_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9740_e8529: f64 = (locals.var_xg10 - locals.var_xg20);
        let assign9740_e8530: f64 = (locals.var_keq * assign9740_e8529);
        (assign9740_e8530, ((locals.var_keq_dn4 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn4 - locals.var_xg20_dn4))), ((locals.var_keq_dn6 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn6 - locals.var_xg20_dn6))), ((locals.var_keq_dn7 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn7 - locals.var_xg20_dn7))), ((locals.var_keq_dn8 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn8 - locals.var_xg20_dn8))), ((locals.var_keq_dn9 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn9 - locals.var_xg20_dn9))),)
    } else {
        (locals.var_dx_wi, locals.var_dx_wi_dn4, locals.var_dx_wi_dn6, locals.var_dx_wi_dn7, locals.var_dx_wi_dn8, locals.var_dx_wi_dn9,)
    }
};
        locals.var_dx_wi = assign9740_e8532;
        locals.var_dx_wi_dn4 = assign9740_e8532_d_n4;
        locals.var_dx_wi_dn6 = assign9740_e8532_d_n6;
        locals.var_dx_wi_dn7 = assign9740_e8532_d_n7;
        locals.var_dx_wi_dn8 = assign9740_e8532_d_n8;
        locals.var_dx_wi_dn9 = assign9740_e8532_d_n9;

        let (assign9750_e8540, assign9750_e8540_d_n4, assign9750_e8540_d_n6, assign9750_e8540_d_n7, assign9750_e8540_d_n8, assign9750_e8540_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9750_e8537: f64 = (locals.var_dx_wi * locals.var_inv_k1);
        let assign9750_e8538: f64 = (locals.var_xg10 - assign9750_e8537);
        (assign9750_e8538, (locals.var_xg10_dn4 - ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4))), (locals.var_xg10_dn6 - ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6))), (locals.var_xg10_dn7 - ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7))), (locals.var_xg10_dn8 - ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8))), (locals.var_xg10_dn9 - ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9))),)
    } else {
        (locals.var_x1_wi0, locals.var_x1_wi0_dn4, locals.var_x1_wi0_dn6, locals.var_x1_wi0_dn7, locals.var_x1_wi0_dn8, locals.var_x1_wi0_dn9,)
    }
};
        locals.var_x1_wi0 = assign9750_e8540;
        locals.var_x1_wi0_dn4 = assign9750_e8540_d_n4;
        locals.var_x1_wi0_dn6 = assign9750_e8540_d_n6;
        locals.var_x1_wi0_dn7 = assign9750_e8540_d_n7;
        locals.var_x1_wi0_dn8 = assign9750_e8540_d_n8;
        locals.var_x1_wi0_dn9 = assign9750_e8540_d_n9;

        let (assign9760_e8548, assign9760_e8548_d_n4, assign9760_e8548_d_n6, assign9760_e8548_d_n7, assign9760_e8548_d_n8, assign9760_e8548_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9760_e8545: f64 = (locals.var_dx_wi * locals.var_inv_k2);
        let assign9760_e8546: f64 = (locals.var_xg20 + assign9760_e8545);
        (assign9760_e8546, (locals.var_xg20_dn4 + ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4))), (locals.var_xg20_dn6 + ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6))), (locals.var_xg20_dn7 + ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7))), (locals.var_xg20_dn8 + ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8))), (locals.var_xg20_dn9 + ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9))),)
    } else {
        (locals.var_x2_wi0, locals.var_x2_wi0_dn4, locals.var_x2_wi0_dn6, locals.var_x2_wi0_dn7, locals.var_x2_wi0_dn8, locals.var_x2_wi0_dn9,)
    }
};
        locals.var_x2_wi0 = assign9760_e8548;
        locals.var_x2_wi0_dn4 = assign9760_e8548_d_n4;
        locals.var_x2_wi0_dn6 = assign9760_e8548_d_n6;
        locals.var_x2_wi0_dn7 = assign9760_e8548_d_n7;
        locals.var_x2_wi0_dn8 = assign9760_e8548_d_n8;
        locals.var_x2_wi0_dn9 = assign9760_e8548_d_n9;

    }

    pub(super) fn stamp_transient_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign9770_e8556, assign9770_e8556_d_n4, assign9770_e8556_d_n6, assign9770_e8556_d_n7, assign9770_e8556_d_n8, assign9770_e8556_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9770_e8553: f64 = (locals.var_k1_1d + 1.0);
        let assign9770_e8554: f64 = (1.0 / assign9770_e8553);
        (assign9770_e8554, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign9770_e8556;
        locals.var_q_temp1_dn4 = assign9770_e8556_d_n4;
        locals.var_q_temp1_dn6 = assign9770_e8556_d_n6;
        locals.var_q_temp1_dn7 = assign9770_e8556_d_n7;
        locals.var_q_temp1_dn8 = assign9770_e8556_d_n8;
        locals.var_q_temp1_dn9 = assign9770_e8556_d_n9;

        let (assign9780_e8564, assign9780_e8564_d_n4, assign9780_e8564_d_n6, assign9780_e8564_d_n7, assign9780_e8564_d_n8, assign9780_e8564_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9780_e8561: f64 = (locals.var_k2_1d + 1.0);
        let assign9780_e8562: f64 = (1.0 / assign9780_e8561);
        (assign9780_e8562, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign9780_e8564;
        locals.var_q_temp2_dn4 = assign9780_e8564_d_n4;
        locals.var_q_temp2_dn6 = assign9780_e8564_d_n6;
        locals.var_q_temp2_dn7 = assign9780_e8564_d_n7;
        locals.var_q_temp2_dn8 = assign9780_e8564_d_n8;
        locals.var_q_temp2_dn9 = assign9780_e8564_d_n9;

        let (assign9790_e8581, assign9790_e8581_d_n4, assign9790_e8581_d_n6, assign9790_e8581_d_n7, assign9790_e8581_d_n8, assign9790_e8581_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9790_e8569: f64 = (locals.var_k2_1d * locals.var_q_temp2);
        let assign9790_e8570: f64 = (locals.var_k1_1d + assign9790_e8569);
        let assign9790_e8572: f64 = (assign9790_e8570 * locals.var_diff_min);
        let assign9790_e8574: f64 = (assign9790_e8572 / locals.var_a0);
        let assign9790_e8575: f64 = (assign9790_e8574).ln();
        let assign9790_e8577: f64 = assign9790_e8575;
        let assign9790_e8579: f64 = (assign9790_e8577 + 1.5);
        (assign9790_e8579, (((((((locals.var_k2_1d * locals.var_q_temp2_dn4) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), (((((((locals.var_k2_1d * locals.var_q_temp2_dn6) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), (((((((locals.var_k2_1d * locals.var_q_temp2_dn7) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), (((((((locals.var_k2_1d * locals.var_q_temp2_dn8) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), (((((((locals.var_k2_1d * locals.var_q_temp2_dn9) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574),)
    } else {
        (locals.var_q_x1sat, locals.var_q_x1sat_dn4, locals.var_q_x1sat_dn6, locals.var_q_x1sat_dn7, locals.var_q_x1sat_dn8, locals.var_q_x1sat_dn9,)
    }
};
        locals.var_q_x1sat = assign9790_e8581;
        locals.var_q_x1sat_dn4 = assign9790_e8581_d_n4;
        locals.var_q_x1sat_dn6 = assign9790_e8581_d_n6;
        locals.var_q_x1sat_dn7 = assign9790_e8581_d_n7;
        locals.var_q_x1sat_dn8 = assign9790_e8581_d_n8;
        locals.var_q_x1sat_dn9 = assign9790_e8581_d_n9;

        let (assign9800_e8598, assign9800_e8598_d_n4, assign9800_e8598_d_n6, assign9800_e8598_d_n7, assign9800_e8598_d_n8, assign9800_e8598_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9800_e8586: f64 = (locals.var_k1_1d * locals.var_q_temp1);
        let assign9800_e8587: f64 = (locals.var_k2_1d + assign9800_e8586);
        let assign9800_e8589: f64 = (assign9800_e8587 * locals.var_diff_min);
        let assign9800_e8591: f64 = (assign9800_e8589 / locals.var_a0);
        let assign9800_e8592: f64 = (assign9800_e8591).ln();
        let assign9800_e8594: f64 = assign9800_e8592;
        let assign9800_e8596: f64 = (assign9800_e8594 + 1.5);
        (assign9800_e8596, (((((((locals.var_k1_1d * locals.var_q_temp1_dn4) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), (((((((locals.var_k1_1d * locals.var_q_temp1_dn6) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), (((((((locals.var_k1_1d * locals.var_q_temp1_dn7) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), (((((((locals.var_k1_1d * locals.var_q_temp1_dn8) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), (((((((locals.var_k1_1d * locals.var_q_temp1_dn9) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591),)
    } else {
        (locals.var_q_x2sat, locals.var_q_x2sat_dn4, locals.var_q_x2sat_dn6, locals.var_q_x2sat_dn7, locals.var_q_x2sat_dn8, locals.var_q_x2sat_dn9,)
    }
};
        locals.var_q_x2sat = assign9800_e8598;
        locals.var_q_x2sat_dn4 = assign9800_e8598_d_n4;
        locals.var_q_x2sat_dn6 = assign9800_e8598_d_n6;
        locals.var_q_x2sat_dn7 = assign9800_e8598_d_n7;
        locals.var_q_x2sat_dn8 = assign9800_e8598_d_n8;
        locals.var_q_x2sat_dn9 = assign9800_e8598_d_n9;

        let assign9810_e8601: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign9810_e8603: f64 = (assign9810_e8601 / 1.5);
        let assign9810_e8605: f64 = if assign9810_e8603 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard533 = assign9810_e8605;

        let (assign9820_e8619, assign9820_e8619_d_n4, assign9820_e8619_d_n6, assign9820_e8619_d_n7, assign9820_e8619_d_n8, assign9820_e8619_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard533 != 0.0)) {
        let assign9820_e8612: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign9820_e8614: f64 = (assign9820_e8612 / 1.5);
        let assign9820_e8615: f64 = (assign9820_e8614).exp();
        let assign9820_e8616: f64 = (1.0 + assign9820_e8615);
        let assign9820_e8617: f64 = (assign9820_e8616).ln();
        (assign9820_e8617, ((assign9820_e8615 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) / 1.5)) / assign9820_e8616), ((assign9820_e8615 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) / 1.5)) / assign9820_e8616), ((assign9820_e8615 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) / 1.5)) / assign9820_e8616), ((assign9820_e8615 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) / 1.5)) / assign9820_e8616), ((assign9820_e8615 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) / 1.5)) / assign9820_e8616),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign9820_e8619;
        locals.var_q_temp3_dn4 = assign9820_e8619_d_n4;
        locals.var_q_temp3_dn6 = assign9820_e8619_d_n6;
        locals.var_q_temp3_dn7 = assign9820_e8619_d_n7;
        locals.var_q_temp3_dn8 = assign9820_e8619_d_n8;
        locals.var_q_temp3_dn9 = assign9820_e8619_d_n9;

        let (assign9830_e8630, assign9830_e8630_d_n4, assign9830_e8630_d_n6, assign9830_e8630_d_n7, assign9830_e8630_d_n8, assign9830_e8630_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard533 == 0.0)) {
        let assign9830_e8626: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign9830_e8628: f64 = (assign9830_e8626 / 1.5);
        (assign9830_e8628, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) / 1.5), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) / 1.5), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) / 1.5), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) / 1.5), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) / 1.5),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign9830_e8630;
        locals.var_q_temp3_dn4 = assign9830_e8630_d_n4;
        locals.var_q_temp3_dn6 = assign9830_e8630_d_n6;
        locals.var_q_temp3_dn7 = assign9830_e8630_d_n7;
        locals.var_q_temp3_dn8 = assign9830_e8630_d_n8;
        locals.var_q_temp3_dn9 = assign9830_e8630_d_n9;

        let (assign9840_e8638, assign9840_e8638_d_n4, assign9840_e8638_d_n6, assign9840_e8638_d_n7, assign9840_e8638_d_n8, assign9840_e8638_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9840_e8635: f64 = (1.5 * locals.var_q_temp3);
        let assign9840_e8636: f64 = (locals.var_q_x1sat - assign9840_e8635);
        (assign9840_e8636, (locals.var_q_x1sat_dn4 - (1.5 * locals.var_q_temp3_dn4)), (locals.var_q_x1sat_dn6 - (1.5 * locals.var_q_temp3_dn6)), (locals.var_q_x1sat_dn7 - (1.5 * locals.var_q_temp3_dn7)), (locals.var_q_x1sat_dn8 - (1.5 * locals.var_q_temp3_dn8)), (locals.var_q_x1sat_dn9 - (1.5 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_x1, locals.var_q_x1_dn4, locals.var_q_x1_dn6, locals.var_q_x1_dn7, locals.var_q_x1_dn8, locals.var_q_x1_dn9,)
    }
};
        locals.var_q_x1 = assign9840_e8638;
        locals.var_q_x1_dn4 = assign9840_e8638_d_n4;
        locals.var_q_x1_dn6 = assign9840_e8638_d_n6;
        locals.var_q_x1_dn7 = assign9840_e8638_d_n7;
        locals.var_q_x1_dn8 = assign9840_e8638_d_n8;
        locals.var_q_x1_dn9 = assign9840_e8638_d_n9;

        let (assign9850_e8648, assign9850_e8648_d_n4, assign9850_e8648_d_n6, assign9850_e8648_d_n7, assign9850_e8648_d_n8, assign9850_e8648_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9850_e8642: f64 = (locals.var_k2_1d * locals.var_xg20);
        let assign9850_e8644: f64 = (assign9850_e8642 + locals.var_q_x1);
        let assign9850_e8646: f64 = (assign9850_e8644 * locals.var_q_temp2);
        (assign9850_e8646, ((((locals.var_k2_1d * locals.var_xg20_dn4) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn4)), ((((locals.var_k2_1d * locals.var_xg20_dn6) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn6)), ((((locals.var_k2_1d * locals.var_xg20_dn7) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn7)), ((((locals.var_k2_1d * locals.var_xg20_dn8) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn8)), ((((locals.var_k2_1d * locals.var_xg20_dn9) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_x2_wi, locals.var_q_x2_wi_dn4, locals.var_q_x2_wi_dn6, locals.var_q_x2_wi_dn7, locals.var_q_x2_wi_dn8, locals.var_q_x2_wi_dn9,)
    }
};
        locals.var_q_x2_wi = assign9850_e8648;
        locals.var_q_x2_wi_dn4 = assign9850_e8648_d_n4;
        locals.var_q_x2_wi_dn6 = assign9850_e8648_d_n6;
        locals.var_q_x2_wi_dn7 = assign9850_e8648_d_n7;
        locals.var_q_x2_wi_dn8 = assign9850_e8648_d_n8;
        locals.var_q_x2_wi_dn9 = assign9850_e8648_d_n9;

        let assign9860_e8651: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign9860_e8653: f64 = (assign9860_e8651 / 1.5);
        let assign9860_e8655: f64 = if assign9860_e8653 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard534 = assign9860_e8655;

        let (assign9870_e8669, assign9870_e8669_d_n4, assign9870_e8669_d_n6, assign9870_e8669_d_n7, assign9870_e8669_d_n8, assign9870_e8669_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard534 != 0.0)) {
        let assign9870_e8662: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign9870_e8664: f64 = (assign9870_e8662 / 1.5);
        let assign9870_e8665: f64 = (assign9870_e8664).exp();
        let assign9870_e8666: f64 = (1.0 + assign9870_e8665);
        let assign9870_e8667: f64 = (assign9870_e8666).ln();
        (assign9870_e8667, ((assign9870_e8665 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) / 1.5)) / assign9870_e8666), ((assign9870_e8665 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) / 1.5)) / assign9870_e8666), ((assign9870_e8665 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) / 1.5)) / assign9870_e8666), ((assign9870_e8665 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) / 1.5)) / assign9870_e8666), ((assign9870_e8665 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) / 1.5)) / assign9870_e8666),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign9870_e8669;
        locals.var_q_temp3_dn4 = assign9870_e8669_d_n4;
        locals.var_q_temp3_dn6 = assign9870_e8669_d_n6;
        locals.var_q_temp3_dn7 = assign9870_e8669_d_n7;
        locals.var_q_temp3_dn8 = assign9870_e8669_d_n8;
        locals.var_q_temp3_dn9 = assign9870_e8669_d_n9;

        let (assign9880_e8680, assign9880_e8680_d_n4, assign9880_e8680_d_n6, assign9880_e8680_d_n7, assign9880_e8680_d_n8, assign9880_e8680_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard534 == 0.0)) {
        let assign9880_e8676: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign9880_e8678: f64 = (assign9880_e8676 / 1.5);
        (assign9880_e8678, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) / 1.5), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) / 1.5), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) / 1.5), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) / 1.5), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) / 1.5),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign9880_e8680;
        locals.var_q_temp3_dn4 = assign9880_e8680_d_n4;
        locals.var_q_temp3_dn6 = assign9880_e8680_d_n6;
        locals.var_q_temp3_dn7 = assign9880_e8680_d_n7;
        locals.var_q_temp3_dn8 = assign9880_e8680_d_n8;
        locals.var_q_temp3_dn9 = assign9880_e8680_d_n9;

        let (assign9890_e8688, assign9890_e8688_d_n4, assign9890_e8688_d_n6, assign9890_e8688_d_n7, assign9890_e8688_d_n8, assign9890_e8688_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9890_e8685: f64 = (1.5 * locals.var_q_temp3);
        let assign9890_e8686: f64 = (locals.var_q_x2sat - assign9890_e8685);
        (assign9890_e8686, (locals.var_q_x2sat_dn4 - (1.5 * locals.var_q_temp3_dn4)), (locals.var_q_x2sat_dn6 - (1.5 * locals.var_q_temp3_dn6)), (locals.var_q_x2sat_dn7 - (1.5 * locals.var_q_temp3_dn7)), (locals.var_q_x2sat_dn8 - (1.5 * locals.var_q_temp3_dn8)), (locals.var_q_x2sat_dn9 - (1.5 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9,)
    }
};
        locals.var_temp0 = assign9890_e8688;
        locals.var_temp0_dn4 = assign9890_e8688_d_n4;
        locals.var_temp0_dn6 = assign9890_e8688_d_n6;
        locals.var_temp0_dn7 = assign9890_e8688_d_n7;
        locals.var_temp0_dn8 = assign9890_e8688_d_n8;
        locals.var_temp0_dn9 = assign9890_e8688_d_n9;

        let (assign9900_e8694, assign9900_e8694_d_n4, assign9900_e8694_d_n6, assign9900_e8694_d_n7, assign9900_e8694_d_n8, assign9900_e8694_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9900_e8692: f64 = (locals.var_temp * locals.var_temp0);
        (assign9900_e8692, ((locals.var_temp_dn4 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn4)), ((locals.var_temp_dn6 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn6)), ((locals.var_temp_dn7 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn7)), ((locals.var_temp_dn8 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn8)), ((locals.var_temp_dn9 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign9900_e8694;
        locals.var_temp1_dn4 = assign9900_e8694_d_n4;
        locals.var_temp1_dn6 = assign9900_e8694_d_n6;
        locals.var_temp1_dn7 = assign9900_e8694_d_n7;
        locals.var_temp1_dn8 = assign9900_e8694_d_n8;
        locals.var_temp1_dn9 = assign9900_e8694_d_n9;

        let (assign9910_e8700, assign9910_e8700_d_n4, assign9910_e8700_d_n6, assign9910_e8700_d_n7, assign9910_e8700_d_n8, assign9910_e8700_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9910_e8698: f64 = (locals.var_temp * locals.var_xg20);
        (assign9910_e8698, ((locals.var_temp_dn4 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn4)), ((locals.var_temp_dn6 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn6)), ((locals.var_temp_dn7 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn7)), ((locals.var_temp_dn8 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn8)), ((locals.var_temp_dn9 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign9910_e8700;
        locals.var_temp2_dn4 = assign9910_e8700_d_n4;
        locals.var_temp2_dn6 = assign9910_e8700_d_n6;
        locals.var_temp2_dn7 = assign9910_e8700_d_n7;
        locals.var_temp2_dn8 = assign9910_e8700_d_n8;
        locals.var_temp2_dn9 = assign9910_e8700_d_n9;

        let (assign9920_e8706, assign9920_e8706_d_n4, assign9920_e8706_d_n6, assign9920_e8706_d_n7, assign9920_e8706_d_n8, assign9920_e8706_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9920_e8704: f64 = (locals.var_temp1 - locals.var_temp2);
        (assign9920_e8704, (locals.var_temp1_dn4 - locals.var_temp2_dn4), (locals.var_temp1_dn6 - locals.var_temp2_dn6), (locals.var_temp1_dn7 - locals.var_temp2_dn7), (locals.var_temp1_dn8 - locals.var_temp2_dn8), (locals.var_temp1_dn9 - locals.var_temp2_dn9),)
    } else {
        (locals.var_spsub_xgb, locals.var_spsub_xgb_dn4, locals.var_spsub_xgb_dn6, locals.var_spsub_xgb_dn7, locals.var_spsub_xgb_dn8, locals.var_spsub_xgb_dn9,)
    }
};
        locals.var_spsub_xgb = assign9920_e8706;
        locals.var_spsub_xgb_dn4 = assign9920_e8706_d_n4;
        locals.var_spsub_xgb_dn6 = assign9920_e8706_d_n6;
        locals.var_spsub_xgb_dn7 = assign9920_e8706_d_n7;
        locals.var_spsub_xgb_dn8 = assign9920_e8706_d_n8;
        locals.var_spsub_xgb_dn9 = assign9920_e8706_d_n9;

        let assign9930_e8708: f64 = (-locals.var_xn_sub);
        let assign9930_e8709: f64 = (assign9930_e8708).abs();
        let assign9930_e8711: f64 = if assign9930_e8709 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard535 = assign9930_e8711;

        let (assign9940_e8719, assign9940_e8719_d_n4, assign9940_e8719_d_n6, assign9940_e8719_d_n7, assign9940_e8719_d_n8, assign9940_e8719_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard535 != 0.0)) {
        let assign9940_e8716: f64 = (-locals.var_xn_sub);
        let assign9940_e8717: f64 = (assign9940_e8716).exp();
        (assign9940_e8717, (assign9940_e8717 * (-locals.var_xn_sub_dn4)), (assign9940_e8717 * (-locals.var_xn_sub_dn6)), (assign9940_e8717 * (-locals.var_xn_sub_dn7)), (assign9940_e8717 * (-locals.var_xn_sub_dn8)), (assign9940_e8717 * (-locals.var_xn_sub_dn9)),)
    } else {
        (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9,)
    }
};
        locals.var_spsub_delta = assign9940_e8719;
        locals.var_spsub_delta_dn4 = assign9940_e8719_d_n4;
        locals.var_spsub_delta_dn6 = assign9940_e8719_d_n6;
        locals.var_spsub_delta_dn7 = assign9940_e8719_d_n7;
        locals.var_spsub_delta_dn8 = assign9940_e8719_d_n8;
        locals.var_spsub_delta_dn9 = assign9940_e8719_d_n9;

        let assign9950_e8721: f64 = (-locals.var_xn_sub);
        let assign9950_e8723: f64 = (-80.0);
        let assign9950_e8724: f64 = if assign9950_e8721 < assign9950_e8723 { 1.0 } else { 0.0 };
        locals.var_guard536 = assign9950_e8724;

        let (assign9960_e8761, assign9960_e8761_d_n4, assign9960_e8761_d_n6, assign9960_e8761_d_n7, assign9960_e8761_d_n8, assign9960_e8761_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard535 == 0.0)) && (locals.var_guard536 != 0.0)) {
        let assign9960_e8734: f64 = (-locals.var_xn_sub);
        let assign9960_e8735: f64 = (-assign9960_e8734);
        let assign9960_e8737: f64 = (assign9960_e8735 - 80.0);
        let assign9960_e8741: f64 = (-locals.var_xn_sub);
        let assign9960_e8742: f64 = (-assign9960_e8741);
        let assign9960_e8744: f64 = (assign9960_e8742 - 80.0);
        let assign9960_e8745: f64 = (0.5 * assign9960_e8744);
        let assign9960_e8748: f64 = (-locals.var_xn_sub);
        let assign9960_e8749: f64 = (-assign9960_e8748);
        let assign9960_e8751: f64 = (assign9960_e8749 - 80.0);
        let assign9960_e8753: f64 = (assign9960_e8751 * 0.3333333333333);
        let assign9960_e8754: f64 = (1.0 + assign9960_e8753);
        let assign9960_e8755: f64 = (assign9960_e8745 * assign9960_e8754);
        let assign9960_e8756: f64 = (1.0 + assign9960_e8755);
        let assign9960_e8757: f64 = (assign9960_e8737 * assign9960_e8756);
        let assign9960_e8758: f64 = (1.0 + assign9960_e8757);
        let assign9960_e8759: f64 = (1.80485e-35 / assign9960_e8758);
        (assign9960_e8759, (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn4)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn4))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn4)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn6)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn6))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn6)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn7)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn7))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn7)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn8)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn8))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn8)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn9)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn9))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn9)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))),)
    } else {
        (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9,)
    }
};
        locals.var_spsub_delta = assign9960_e8761;
        locals.var_spsub_delta_dn4 = assign9960_e8761_d_n4;
        locals.var_spsub_delta_dn6 = assign9960_e8761_d_n6;
        locals.var_spsub_delta_dn7 = assign9960_e8761_d_n7;
        locals.var_spsub_delta_dn8 = assign9960_e8761_d_n8;
        locals.var_spsub_delta_dn9 = assign9960_e8761_d_n9;

        let (assign9970_e8796, assign9970_e8796_d_n4, assign9970_e8796_d_n6, assign9970_e8796_d_n7, assign9970_e8796_d_n8, assign9970_e8796_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard535 == 0.0)) && (locals.var_guard536 == 0.0)) {
        let assign9970_e8772: f64 = (-locals.var_xn_sub);
        let assign9970_e8774: f64 = (assign9970_e8772 - 80.0);
        let assign9970_e8778: f64 = (-locals.var_xn_sub);
        let assign9970_e8780: f64 = (assign9970_e8778 - 80.0);
        let assign9970_e8781: f64 = (0.5 * assign9970_e8780);
        let assign9970_e8784: f64 = (-locals.var_xn_sub);
        let assign9970_e8786: f64 = (assign9970_e8784 - 80.0);
        let assign9970_e8788: f64 = (assign9970_e8786 * 0.3333333333333);
        let assign9970_e8789: f64 = (1.0 + assign9970_e8788);
        let assign9970_e8790: f64 = (assign9970_e8781 * assign9970_e8789);
        let assign9970_e8791: f64 = (1.0 + assign9970_e8790);
        let assign9970_e8792: f64 = (assign9970_e8774 * assign9970_e8791);
        let assign9970_e8793: f64 = (1.0 + assign9970_e8792);
        let assign9970_e8794: f64 = (5.54062e34 * assign9970_e8793);
        (assign9970_e8794, (5.54062e34 * (((-locals.var_xn_sub_dn4) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn4)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn6) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn6)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn7) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn7)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn8) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn8)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn9) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn9)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9,)
    }
};
        locals.var_spsub_delta = assign9970_e8796;
        locals.var_spsub_delta_dn4 = assign9970_e8796_d_n4;
        locals.var_spsub_delta_dn6 = assign9970_e8796_d_n6;
        locals.var_spsub_delta_dn7 = assign9970_e8796_d_n7;
        locals.var_spsub_delta_dn8 = assign9970_e8796_d_n8;
        locals.var_spsub_delta_dn9 = assign9970_e8796_d_n9;

        let assign9980_e8798: f64 = (locals.var_spsub_xgb).abs();
        let assign9980_e8800: f64 = if assign9980_e8798 <= locals.var_margin_sub { 1.0 } else { 0.0 };
        locals.var_guard537 = assign9980_e8800;

        let (assign9990_e8812, assign9990_e8812_d_n4, assign9990_e8812_d_n6, assign9990_e8812_d_n7, assign9990_e8812_d_n8, assign9990_e8812_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard537 != 0.0)) {
        let assign9990_e8806: f64 = (locals.var_inv_xisub * locals.var_inv_xisub);
        let assign9990_e8808: f64 = (assign9990_e8806 * 0.1666666666667);
        let assign9990_e8810: f64 = (assign9990_e8808 / 1.4142135623731);
        (assign9990_e8810, ((((locals.var_inv_xisub_dn4 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn4)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn6 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn6)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn7 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn7)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn8 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn8)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn9 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn9)) * 0.1666666666667) / 1.4142135623731),)
    } else {
        (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9,)
    }
};
        locals.var_spsub_temp1 = assign9990_e8812;
        locals.var_spsub_temp1_dn4 = assign9990_e8812_d_n4;
        locals.var_spsub_temp1_dn6 = assign9990_e8812_d_n6;
        locals.var_spsub_temp1_dn7 = assign9990_e8812_d_n7;
        locals.var_spsub_temp1_dn8 = assign9990_e8812_d_n8;
        locals.var_spsub_temp1_dn9 = assign9990_e8812_d_n9;

        let (assign10000_e8832, assign10000_e8832_d_n4, assign10000_e8832_d_n6, assign10000_e8832_d_n7, assign10000_e8832_d_n8, assign10000_e8832_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard537 != 0.0)) {
        let assign10000_e8818: f64 = (locals.var_spsub_xgb * locals.var_inv_xisub);
        let assign10000_e8823: f64 = (1.0 - locals.var_spsub_delta);
        let assign10000_e8824: f64 = (locals.var_spsub_xgb * assign10000_e8823);
        let assign10000_e8826: f64 = (assign10000_e8824 * locals.var_gfsub);
        let assign10000_e8828: f64 = (assign10000_e8826 * locals.var_spsub_temp1);
        let assign10000_e8829: f64 = (1.0 + assign10000_e8828);
        let assign10000_e8830: f64 = (assign10000_e8818 * assign10000_e8829);
        (assign10000_e8830, ((((locals.var_spsub_xgb_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn4)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn4 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn4))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn4)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn4)))), ((((locals.var_spsub_xgb_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn6)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn6 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn6))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn6)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn6)))), ((((locals.var_spsub_xgb_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn7)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn7 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn7))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn7)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn7)))), ((((locals.var_spsub_xgb_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn8)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn8 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn8))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn8)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn8)))), ((((locals.var_spsub_xgb_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn9)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn9 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn9))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn9)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn9)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign10000_e8832;
        locals.var_temp3_dn4 = assign10000_e8832_d_n4;
        locals.var_temp3_dn6 = assign10000_e8832_d_n6;
        locals.var_temp3_dn7 = assign10000_e8832_d_n7;
        locals.var_temp3_dn8 = assign10000_e8832_d_n8;
        locals.var_temp3_dn9 = assign10000_e8832_d_n9;

        let assign10010_e8835: f64 = (-locals.var_margin_sub);
        let assign10010_e8836: f64 = if locals.var_spsub_xgb < assign10010_e8835 { 1.0 } else { 0.0 };
        locals.var_guard538 = assign10010_e8836;

        let (assign10020_e8846, assign10020_e8846_d_n4, assign10020_e8846_d_n6, assign10020_e8846_d_n7, assign10020_e8846_d_n8, assign10020_e8846_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10020_e8844: f64 = (-locals.var_spsub_xgb);
        (assign10020_e8844, (-locals.var_spsub_xgb_dn4), (-locals.var_spsub_xgb_dn6), (-locals.var_spsub_xgb_dn7), (-locals.var_spsub_xgb_dn8), (-locals.var_spsub_xgb_dn9),)
    } else {
        (locals.var_spsub_yg, locals.var_spsub_yg_dn4, locals.var_spsub_yg_dn6, locals.var_spsub_yg_dn7, locals.var_spsub_yg_dn8, locals.var_spsub_yg_dn9,)
    }
};
        locals.var_spsub_yg = assign10020_e8846;
        locals.var_spsub_yg_dn4 = assign10020_e8846_d_n4;
        locals.var_spsub_yg_dn6 = assign10020_e8846_d_n6;
        locals.var_spsub_yg_dn7 = assign10020_e8846_d_n7;
        locals.var_spsub_yg_dn8 = assign10020_e8846_d_n8;
        locals.var_spsub_yg_dn9 = assign10020_e8846_d_n9;

        let (assign10030_e8859, assign10030_e8859_d_n4, assign10030_e8859_d_n6, assign10030_e8859_d_n7, assign10030_e8859_d_n8, assign10030_e8859_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10030_e8856: f64 = (locals.var_spsub_yg * locals.var_inv_xisub);
        let assign10030_e8857: f64 = (1.25 * assign10030_e8856);
        (assign10030_e8857, (1.25 * ((locals.var_spsub_yg_dn4 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn4))), (1.25 * ((locals.var_spsub_yg_dn6 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn6))), (1.25 * ((locals.var_spsub_yg_dn7 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn7))), (1.25 * ((locals.var_spsub_yg_dn8 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn8))), (1.25 * ((locals.var_spsub_yg_dn9 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn9))),)
    } else {
        (locals.var_spsub_ysub, locals.var_spsub_ysub_dn4, locals.var_spsub_ysub_dn6, locals.var_spsub_ysub_dn7, locals.var_spsub_ysub_dn8, locals.var_spsub_ysub_dn9,)
    }
};
        locals.var_spsub_ysub = assign10030_e8859;
        locals.var_spsub_ysub_dn4 = assign10030_e8859_d_n4;
        locals.var_spsub_ysub_dn6 = assign10030_e8859_d_n6;
        locals.var_spsub_ysub_dn7 = assign10030_e8859_d_n7;
        locals.var_spsub_ysub_dn8 = assign10030_e8859_d_n8;
        locals.var_spsub_ysub_dn9 = assign10030_e8859_d_n9;

        let (assign10040_e8883, assign10040_e8883_d_n4, assign10040_e8883_d_n6, assign10040_e8883_d_n7, assign10040_e8883_d_n8, assign10040_e8883_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10040_e8869: f64 = (locals.var_spsub_ysub + 10.0);
        let assign10040_e8872: f64 = (locals.var_spsub_ysub - 6.0);
        let assign10040_e8875: f64 = (locals.var_spsub_ysub - 6.0);
        let assign10040_e8876: f64 = (assign10040_e8872 * assign10040_e8875);
        let assign10040_e8878: f64 = (assign10040_e8876 + 64.0);
        let assign10040_e8879: f64 = (assign10040_e8878).sqrt();
        let assign10040_e8880: f64 = (assign10040_e8869 - assign10040_e8879);
        let assign10040_e8881: f64 = (0.5 * assign10040_e8880);
        (assign10040_e8881, (0.5 * (locals.var_spsub_ysub_dn4 - (((locals.var_spsub_ysub_dn4 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn4)) / (2.0 * assign10040_e8879)))), (0.5 * (locals.var_spsub_ysub_dn6 - (((locals.var_spsub_ysub_dn6 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn6)) / (2.0 * assign10040_e8879)))), (0.5 * (locals.var_spsub_ysub_dn7 - (((locals.var_spsub_ysub_dn7 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn7)) / (2.0 * assign10040_e8879)))), (0.5 * (locals.var_spsub_ysub_dn8 - (((locals.var_spsub_ysub_dn8 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn8)) / (2.0 * assign10040_e8879)))), (0.5 * (locals.var_spsub_ysub_dn9 - (((locals.var_spsub_ysub_dn9 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn9)) / (2.0 * assign10040_e8879)))),)
    } else {
        (locals.var_spsub_eta, locals.var_spsub_eta_dn4, locals.var_spsub_eta_dn6, locals.var_spsub_eta_dn7, locals.var_spsub_eta_dn8, locals.var_spsub_eta_dn9,)
    }
};
        locals.var_spsub_eta = assign10040_e8883;
        locals.var_spsub_eta_dn4 = assign10040_e8883_d_n4;
        locals.var_spsub_eta_dn6 = assign10040_e8883_d_n6;
        locals.var_spsub_eta_dn7 = assign10040_e8883_d_n7;
        locals.var_spsub_eta_dn8 = assign10040_e8883_d_n8;
        locals.var_spsub_eta_dn9 = assign10040_e8883_d_n9;

        let (assign10050_e8894, assign10050_e8894_d_n4, assign10050_e8894_d_n6, assign10050_e8894_d_n7, assign10050_e8894_d_n8, assign10050_e8894_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10050_e8892: f64 = (locals.var_spsub_yg - locals.var_spsub_eta);
        (assign10050_e8892, (locals.var_spsub_yg_dn4 - locals.var_spsub_eta_dn4), (locals.var_spsub_yg_dn6 - locals.var_spsub_eta_dn6), (locals.var_spsub_yg_dn7 - locals.var_spsub_eta_dn7), (locals.var_spsub_yg_dn8 - locals.var_spsub_eta_dn8), (locals.var_spsub_yg_dn9 - locals.var_spsub_eta_dn9),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10050_e8894;
        locals.var_spsub_temp_dn4 = assign10050_e8894_d_n4;
        locals.var_spsub_temp_dn6 = assign10050_e8894_d_n6;
        locals.var_spsub_temp_dn7 = assign10050_e8894_d_n7;
        locals.var_spsub_temp_dn8 = assign10050_e8894_d_n8;
        locals.var_spsub_temp_dn9 = assign10050_e8894_d_n9;

        let (assign10060_e8911, assign10060_e8911_d_n4, assign10060_e8911_d_n6, assign10060_e8911_d_n7, assign10060_e8911_d_n8, assign10060_e8911_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10060_e8903: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
        let assign10060_e8907: f64 = (locals.var_spsub_eta + 1.0);
        let assign10060_e8908: f64 = (locals.var_gfsub2 * assign10060_e8907);
        let assign10060_e8909: f64 = (assign10060_e8903 + assign10060_e8908);
        (assign10060_e8909, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) + ((locals.var_gfsub2_dn4 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn4))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) + ((locals.var_gfsub2_dn6 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn6))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) + ((locals.var_gfsub2_dn7 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn7))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) + ((locals.var_gfsub2_dn8 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn8))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) + ((locals.var_gfsub2_dn9 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn9))),)
    } else {
        (locals.var_spsub_a, locals.var_spsub_a_dn4, locals.var_spsub_a_dn6, locals.var_spsub_a_dn7, locals.var_spsub_a_dn8, locals.var_spsub_a_dn9,)
    }
};
        locals.var_spsub_a = assign10060_e8911;
        locals.var_spsub_a_dn4 = assign10060_e8911_d_n4;
        locals.var_spsub_a_dn6 = assign10060_e8911_d_n6;
        locals.var_spsub_a_dn7 = assign10060_e8911_d_n7;
        locals.var_spsub_a_dn8 = assign10060_e8911_d_n8;
        locals.var_spsub_a_dn9 = assign10060_e8911_d_n9;

        let (assign10070_e8924, assign10070_e8924_d_n4, assign10070_e8924_d_n6, assign10070_e8924_d_n7, assign10070_e8924_d_n8, assign10070_e8924_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10070_e8920: f64 = (2.0 * locals.var_spsub_temp);
        let assign10070_e8922: f64 = (assign10070_e8920 - locals.var_gfsub2);
        (assign10070_e8922, ((2.0 * locals.var_spsub_temp_dn4) - locals.var_gfsub2_dn4), ((2.0 * locals.var_spsub_temp_dn6) - locals.var_gfsub2_dn6), ((2.0 * locals.var_spsub_temp_dn7) - locals.var_gfsub2_dn7), ((2.0 * locals.var_spsub_temp_dn8) - locals.var_gfsub2_dn8), ((2.0 * locals.var_spsub_temp_dn9) - locals.var_gfsub2_dn9),)
    } else {
        (locals.var_spsub_c, locals.var_spsub_c_dn4, locals.var_spsub_c_dn6, locals.var_spsub_c_dn7, locals.var_spsub_c_dn8, locals.var_spsub_c_dn9,)
    }
};
        locals.var_spsub_c = assign10070_e8924;
        locals.var_spsub_c_dn4 = assign10070_e8924_d_n4;
        locals.var_spsub_c_dn6 = assign10070_e8924_d_n6;
        locals.var_spsub_c_dn7 = assign10070_e8924_d_n7;
        locals.var_spsub_c_dn8 = assign10070_e8924_d_n8;
        locals.var_spsub_c_dn9 = assign10070_e8924_d_n9;

        let (assign10080_e8939, assign10080_e8939_d_n4, assign10080_e8939_d_n6, assign10080_e8939_d_n7, assign10080_e8939_d_n8, assign10080_e8939_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10080_e8932: f64 = (-locals.var_spsub_eta);
        let assign10080_e8935: f64 = (locals.var_spsub_a * locals.var_inv_gfsub2);
        let assign10080_e8936: f64 = (assign10080_e8935).ln();
        let assign10080_e8937: f64 = (assign10080_e8932 + assign10080_e8936);
        (assign10080_e8937, ((-locals.var_spsub_eta_dn4) + (((locals.var_spsub_a_dn4 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn4)) / assign10080_e8935)), ((-locals.var_spsub_eta_dn6) + (((locals.var_spsub_a_dn6 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn6)) / assign10080_e8935)), ((-locals.var_spsub_eta_dn7) + (((locals.var_spsub_a_dn7 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn7)) / assign10080_e8935)), ((-locals.var_spsub_eta_dn8) + (((locals.var_spsub_a_dn8 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn8)) / assign10080_e8935)), ((-locals.var_spsub_eta_dn9) + (((locals.var_spsub_a_dn9 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn9)) / assign10080_e8935)),)
    } else {
        (locals.var_spsub_tau, locals.var_spsub_tau_dn4, locals.var_spsub_tau_dn6, locals.var_spsub_tau_dn7, locals.var_spsub_tau_dn8, locals.var_spsub_tau_dn9,)
    }
};
        locals.var_spsub_tau = assign10080_e8939;
        locals.var_spsub_tau_dn4 = assign10080_e8939_d_n4;
        locals.var_spsub_tau_dn6 = assign10080_e8939_d_n6;
        locals.var_spsub_tau_dn7 = assign10080_e8939_d_n7;
        locals.var_spsub_tau_dn8 = assign10080_e8939_d_n8;
        locals.var_spsub_tau_dn9 = assign10080_e8939_d_n9;

    }

    pub(super) fn stamp_transient_block_22(
        locals: &mut StampLocals,
    ) {
        let (assign10090_e8950, assign10090_e8950_d_n4, assign10090_e8950_d_n6, assign10090_e8950_d_n7, assign10090_e8950_d_n8, assign10090_e8950_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10090_e8948: f64 = (locals.var_spsub_a + locals.var_spsub_c);
        (assign10090_e8948, (locals.var_spsub_a_dn4 + locals.var_spsub_c_dn4), (locals.var_spsub_a_dn6 + locals.var_spsub_c_dn6), (locals.var_spsub_a_dn7 + locals.var_spsub_c_dn7), (locals.var_spsub_a_dn8 + locals.var_spsub_c_dn8), (locals.var_spsub_a_dn9 + locals.var_spsub_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign10090_e8950;
        locals.var_nu_dn4 = assign10090_e8950_d_n4;
        locals.var_nu_dn6 = assign10090_e8950_d_n6;
        locals.var_nu_dn7 = assign10090_e8950_d_n7;
        locals.var_nu_dn8 = assign10090_e8950_d_n8;
        locals.var_nu_dn9 = assign10090_e8950_d_n9;

        let (assign10100_e8971, assign10100_e8971_d_n4, assign10100_e8971_d_n6, assign10100_e8971_d_n7, assign10100_e8971_d_n8, assign10100_e8971_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10100_e8959: f64 = (locals.var_nu * locals.var_nu);
        let assign10100_e8963: f64 = (0.5 * locals.var_spsub_c);
        let assign10100_e8965: f64 = (assign10100_e8963 * locals.var_spsub_c);
        let assign10100_e8967: f64 = (assign10100_e8965 - locals.var_spsub_a);
        let assign10100_e8968: f64 = (locals.var_spsub_tau * assign10100_e8967);
        let assign10100_e8969: f64 = (assign10100_e8959 + assign10100_e8968);
        (assign10100_e8969, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_spsub_tau_dn4 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn4) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn4)) - locals.var_spsub_a_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_spsub_tau_dn6 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn6) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn6)) - locals.var_spsub_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_spsub_tau_dn7 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn7) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn7)) - locals.var_spsub_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_spsub_tau_dn8 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn8) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn8)) - locals.var_spsub_a_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_spsub_tau_dn9 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn9) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn9)) - locals.var_spsub_a_dn9)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign10100_e8971;
        locals.var_mutau_dn4 = assign10100_e8971_d_n4;
        locals.var_mutau_dn6 = assign10100_e8971_d_n6;
        locals.var_mutau_dn7 = assign10100_e8971_d_n7;
        locals.var_mutau_dn8 = assign10100_e8971_d_n8;
        locals.var_mutau_dn9 = assign10100_e8971_d_n9;

        let (assign10110_e9006, assign10110_e9006_d_n4, assign10110_e9006_d_n6, assign10110_e9006_d_n7, assign10110_e9006_d_n8, assign10110_e9006_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10110_e8981: f64 = (locals.var_spsub_a * locals.var_nu);
        let assign10110_e8983: f64 = (assign10110_e8981 * locals.var_spsub_tau);
        let assign10110_e8987: f64 = (locals.var_nu / locals.var_mutau);
        let assign10110_e8989: f64 = (assign10110_e8987 * locals.var_spsub_tau);
        let assign10110_e8991: f64 = (assign10110_e8989 * locals.var_spsub_tau);
        let assign10110_e8993: f64 = (assign10110_e8991 * locals.var_spsub_c);
        let assign10110_e8996: f64 = (locals.var_spsub_c * locals.var_spsub_c);
        let assign10110_e8998: f64 = (assign10110_e8996 * 0.3333333333333);
        let assign10110_e9000: f64 = (assign10110_e8998 - locals.var_spsub_a);
        let assign10110_e9001: f64 = (assign10110_e8993 * assign10110_e9000);
        let assign10110_e9002: f64 = (locals.var_mutau + assign10110_e9001);
        let assign10110_e9003: f64 = (assign10110_e8983 / assign10110_e9002);
        let assign10110_e9004: f64 = (locals.var_spsub_eta + assign10110_e9003);
        (assign10110_e9004, (locals.var_spsub_eta_dn4 + (((((((locals.var_spsub_a_dn4 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn4)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn4)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn4)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn4)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn4)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn4 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn4)) * 0.3333333333333) - locals.var_spsub_a_dn4)))))) / (assign10110_e9002 * assign10110_e9002))), (locals.var_spsub_eta_dn6 + (((((((locals.var_spsub_a_dn6 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn6)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn6)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn6)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn6)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn6)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn6 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn6)) * 0.3333333333333) - locals.var_spsub_a_dn6)))))) / (assign10110_e9002 * assign10110_e9002))), (locals.var_spsub_eta_dn7 + (((((((locals.var_spsub_a_dn7 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn7)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn7)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn7)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn7)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn7)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn7 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn7)) * 0.3333333333333) - locals.var_spsub_a_dn7)))))) / (assign10110_e9002 * assign10110_e9002))), (locals.var_spsub_eta_dn8 + (((((((locals.var_spsub_a_dn8 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn8)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn8)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn8)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn8)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn8)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn8 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn8)) * 0.3333333333333) - locals.var_spsub_a_dn8)))))) / (assign10110_e9002 * assign10110_e9002))), (locals.var_spsub_eta_dn9 + (((((((locals.var_spsub_a_dn9 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn9)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn9)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn9)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn9)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn9)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn9 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn9)) * 0.3333333333333) - locals.var_spsub_a_dn9)))))) / (assign10110_e9002 * assign10110_e9002))),)
    } else {
        (locals.var_spsub_y0, locals.var_spsub_y0_dn4, locals.var_spsub_y0_dn6, locals.var_spsub_y0_dn7, locals.var_spsub_y0_dn8, locals.var_spsub_y0_dn9,)
    }
};
        locals.var_spsub_y0 = assign10110_e9006;
        locals.var_spsub_y0_dn4 = assign10110_e9006_d_n4;
        locals.var_spsub_y0_dn6 = assign10110_e9006_d_n6;
        locals.var_spsub_y0_dn7 = assign10110_e9006_d_n7;
        locals.var_spsub_y0_dn8 = assign10110_e9006_d_n8;
        locals.var_spsub_y0_dn9 = assign10110_e9006_d_n9;

        let assign10120_e9009: f64 = if locals.var_spsub_y0 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard539 = assign10120_e9009;

        let (assign10130_e9021, assign10130_e9021_d_n4, assign10130_e9021_d_n6, assign10130_e9021_d_n7, assign10130_e9021_d_n8, assign10130_e9021_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) {
        let assign10130_e9019: f64 = (locals.var_spsub_y0).exp();
        (assign10130_e9019, (assign10130_e9019 * locals.var_spsub_y0_dn4), (assign10130_e9019 * locals.var_spsub_y0_dn6), (assign10130_e9019 * locals.var_spsub_y0_dn7), (assign10130_e9019 * locals.var_spsub_y0_dn8), (assign10130_e9019 * locals.var_spsub_y0_dn9),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10130_e9021;
        locals.var_spsub_delta0_dn4 = assign10130_e9021_d_n4;
        locals.var_spsub_delta0_dn6 = assign10130_e9021_d_n6;
        locals.var_spsub_delta0_dn7 = assign10130_e9021_d_n7;
        locals.var_spsub_delta0_dn8 = assign10130_e9021_d_n8;
        locals.var_spsub_delta0_dn9 = assign10130_e9021_d_n9;

        let (assign10140_e9055, assign10140_e9055_d_n4, assign10140_e9055_d_n6, assign10140_e9055_d_n7, assign10140_e9055_d_n8, assign10140_e9055_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 == 0.0)) {
        let assign10140_e9035: f64 = (locals.var_spsub_y0 - 80.0);
        let assign10140_e9040: f64 = (locals.var_spsub_y0 - 80.0);
        let assign10140_e9041: f64 = (0.5 * assign10140_e9040);
        let assign10140_e9045: f64 = (locals.var_spsub_y0 - 80.0);
        let assign10140_e9047: f64 = (assign10140_e9045 * 0.3333333333333);
        let assign10140_e9048: f64 = (1.0 + assign10140_e9047);
        let assign10140_e9049: f64 = (assign10140_e9041 * assign10140_e9048);
        let assign10140_e9050: f64 = (1.0 + assign10140_e9049);
        let assign10140_e9051: f64 = (assign10140_e9035 * assign10140_e9050);
        let assign10140_e9052: f64 = (1.0 + assign10140_e9051);
        let assign10140_e9053: f64 = (5.54062e34 * assign10140_e9052);
        (assign10140_e9053, (5.54062e34 * ((locals.var_spsub_y0_dn4 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn4) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn6 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn6) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn7 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn7) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn8 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn8) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn9 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn9) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10140_e9055;
        locals.var_spsub_delta0_dn4 = assign10140_e9055_d_n4;
        locals.var_spsub_delta0_dn6 = assign10140_e9055_d_n6;
        locals.var_spsub_delta0_dn7 = assign10140_e9055_d_n7;
        locals.var_spsub_delta0_dn8 = assign10140_e9055_d_n8;
        locals.var_spsub_delta0_dn9 = assign10140_e9055_d_n9;

        let (assign10150_e9066, assign10150_e9066_d_n4, assign10150_e9066_d_n6, assign10150_e9066_d_n7, assign10150_e9066_d_n8, assign10150_e9066_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10150_e9064: f64 = (1.0 / locals.var_spsub_delta0);
        (assign10150_e9064, (-(locals.var_spsub_delta0_dn4 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn6 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn7 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn8 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn9 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))),)
    } else {
        (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9,)
    }
};
        locals.var_spsub_delta1 = assign10150_e9066;
        locals.var_spsub_delta1_dn4 = assign10150_e9066_d_n4;
        locals.var_spsub_delta1_dn6 = assign10150_e9066_d_n6;
        locals.var_spsub_delta1_dn7 = assign10150_e9066_d_n7;
        locals.var_spsub_delta1_dn8 = assign10150_e9066_d_n8;
        locals.var_spsub_delta1_dn9 = assign10150_e9066_d_n9;

        let (assign10160_e9081, assign10160_e9081_d_n4, assign10160_e9081_d_n6, assign10160_e9081_d_n7, assign10160_e9081_d_n8, assign10160_e9081_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10160_e9077: f64 = (locals.var_spsub_y0 * locals.var_spsub_y0);
        let assign10160_e9078: f64 = (2.0 + assign10160_e9077);
        let assign10160_e9079: f64 = (1.0 / assign10160_e9078);
        (assign10160_e9079, (-(((locals.var_spsub_y0_dn4 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn4)) / (assign10160_e9078 * assign10160_e9078))), (-(((locals.var_spsub_y0_dn6 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn6)) / (assign10160_e9078 * assign10160_e9078))), (-(((locals.var_spsub_y0_dn7 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn7)) / (assign10160_e9078 * assign10160_e9078))), (-(((locals.var_spsub_y0_dn8 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn8)) / (assign10160_e9078 * assign10160_e9078))), (-(((locals.var_spsub_y0_dn9 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn9)) / (assign10160_e9078 * assign10160_e9078))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10160_e9081;
        locals.var_spsub_temp_dn4 = assign10160_e9081_d_n4;
        locals.var_spsub_temp_dn6 = assign10160_e9081_d_n6;
        locals.var_spsub_temp_dn7 = assign10160_e9081_d_n7;
        locals.var_spsub_temp_dn8 = assign10160_e9081_d_n8;
        locals.var_spsub_temp_dn9 = assign10160_e9081_d_n9;

        let (assign10170_e9094, assign10170_e9094_d_n4, assign10170_e9094_d_n6, assign10170_e9094_d_n7, assign10170_e9094_d_n8, assign10170_e9094_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10170_e9090: f64 = (locals.var_spsub_y0 * locals.var_spsub_y0);
        let assign10170_e9092: f64 = (assign10170_e9090 * locals.var_spsub_temp);
        (assign10170_e9092, ((((locals.var_spsub_y0_dn4 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn4)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn4)), ((((locals.var_spsub_y0_dn6 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn6)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn6)), ((((locals.var_spsub_y0_dn7 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn7)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn7)), ((((locals.var_spsub_y0_dn8 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn8)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn8)), ((((locals.var_spsub_y0_dn9 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn9)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn9)),)
    } else {
        (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9,)
    }
};
        locals.var_spsub_xi0 = assign10170_e9094;
        locals.var_spsub_xi0_dn4 = assign10170_e9094_d_n4;
        locals.var_spsub_xi0_dn6 = assign10170_e9094_d_n6;
        locals.var_spsub_xi0_dn7 = assign10170_e9094_d_n7;
        locals.var_spsub_xi0_dn8 = assign10170_e9094_d_n8;
        locals.var_spsub_xi0_dn9 = assign10170_e9094_d_n9;

        let (assign10180_e9109, assign10180_e9109_d_n4, assign10180_e9109_d_n6, assign10180_e9109_d_n7, assign10180_e9109_d_n8, assign10180_e9109_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10180_e9104: f64 = (locals.var_spsub_y0 * locals.var_spsub_temp);
        let assign10180_e9106: f64 = (assign10180_e9104 * locals.var_spsub_temp);
        let assign10180_e9107: f64 = (4.0 * assign10180_e9106);
        (assign10180_e9107, (4.0 * ((((locals.var_spsub_y0_dn4 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn4))), (4.0 * ((((locals.var_spsub_y0_dn6 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn6))), (4.0 * ((((locals.var_spsub_y0_dn7 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn7))), (4.0 * ((((locals.var_spsub_y0_dn8 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn8))), (4.0 * ((((locals.var_spsub_y0_dn9 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn9))),)
    } else {
        (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9,)
    }
};
        locals.var_spsub_xi1 = assign10180_e9109;
        locals.var_spsub_xi1_dn4 = assign10180_e9109_d_n4;
        locals.var_spsub_xi1_dn6 = assign10180_e9109_d_n6;
        locals.var_spsub_xi1_dn7 = assign10180_e9109_d_n7;
        locals.var_spsub_xi1_dn8 = assign10180_e9109_d_n8;
        locals.var_spsub_xi1_dn9 = assign10180_e9109_d_n9;

        let (assign10190_e9128, assign10190_e9128_d_n4, assign10190_e9128_d_n6, assign10190_e9128_d_n7, assign10190_e9128_d_n8, assign10190_e9128_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10190_e9118: f64 = (8.0 * locals.var_spsub_temp);
        let assign10190_e9121: f64 = (12.0 * locals.var_spsub_xi0);
        let assign10190_e9122: f64 = (assign10190_e9118 - assign10190_e9121);
        let assign10190_e9124: f64 = (assign10190_e9122 * locals.var_spsub_temp);
        let assign10190_e9126: f64 = (assign10190_e9124 * locals.var_spsub_temp);
        (assign10190_e9126, ((((((8.0 * locals.var_spsub_temp_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn4)), ((((((8.0 * locals.var_spsub_temp_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn6)), ((((((8.0 * locals.var_spsub_temp_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn7)), ((((((8.0 * locals.var_spsub_temp_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn8)), ((((((8.0 * locals.var_spsub_temp_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn9)),)
    } else {
        (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9,)
    }
};
        locals.var_spsub_xi2 = assign10190_e9128;
        locals.var_spsub_xi2_dn4 = assign10190_e9128_d_n4;
        locals.var_spsub_xi2_dn6 = assign10190_e9128_d_n6;
        locals.var_spsub_xi2_dn7 = assign10190_e9128_d_n7;
        locals.var_spsub_xi2_dn8 = assign10190_e9128_d_n8;
        locals.var_spsub_xi2_dn9 = assign10190_e9128_d_n9;

        let (assign10200_e9139, assign10200_e9139_d_n4, assign10200_e9139_d_n6, assign10200_e9139_d_n7, assign10200_e9139_d_n8, assign10200_e9139_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10200_e9137: f64 = (locals.var_spsub_yg - locals.var_spsub_y0);
        (assign10200_e9137, (locals.var_spsub_yg_dn4 - locals.var_spsub_y0_dn4), (locals.var_spsub_yg_dn6 - locals.var_spsub_y0_dn6), (locals.var_spsub_yg_dn7 - locals.var_spsub_y0_dn7), (locals.var_spsub_yg_dn8 - locals.var_spsub_y0_dn8), (locals.var_spsub_yg_dn9 - locals.var_spsub_y0_dn9),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10200_e9139;
        locals.var_spsub_temp_dn4 = assign10200_e9139_d_n4;
        locals.var_spsub_temp_dn6 = assign10200_e9139_d_n6;
        locals.var_spsub_temp_dn7 = assign10200_e9139_d_n7;
        locals.var_spsub_temp_dn8 = assign10200_e9139_d_n8;
        locals.var_spsub_temp_dn9 = assign10200_e9139_d_n9;

        let (assign10210_e9150, assign10210_e9150_d_n4, assign10210_e9150_d_n6, assign10210_e9150_d_n7, assign10210_e9150_d_n8, assign10210_e9150_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10210_e9148: f64 = (locals.var_spsub_delta * locals.var_spsub_delta1);
        (assign10210_e9148, ((locals.var_spsub_delta_dn4 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn4)), ((locals.var_spsub_delta_dn6 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn6)), ((locals.var_spsub_delta_dn7 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn7)), ((locals.var_spsub_delta_dn8 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn8)), ((locals.var_spsub_delta_dn9 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn9)),)
    } else {
        (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9,)
    }
};
        locals.var_spsub_temp1 = assign10210_e9150;
        locals.var_spsub_temp1_dn4 = assign10210_e9150_d_n4;
        locals.var_spsub_temp1_dn6 = assign10210_e9150_d_n6;
        locals.var_spsub_temp1_dn7 = assign10210_e9150_d_n7;
        locals.var_spsub_temp1_dn8 = assign10210_e9150_d_n8;
        locals.var_spsub_temp1_dn9 = assign10210_e9150_d_n9;

        let (assign10220_e9175, assign10220_e9175_d_n4, assign10220_e9175_d_n6, assign10220_e9175_d_n7, assign10220_e9175_d_n8, assign10220_e9175_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10220_e9159: f64 = (2.0 * locals.var_spsub_temp);
        let assign10220_e9163: f64 = (locals.var_spsub_delta0 - 1.0);
        let assign10220_e9165: f64 = (assign10220_e9163 - locals.var_spsub_temp1);
        let assign10220_e9169: f64 = (1.0 - locals.var_spsub_xi1);
        let assign10220_e9170: f64 = (locals.var_spsub_delta * assign10220_e9169);
        let assign10220_e9171: f64 = (assign10220_e9165 + assign10220_e9170);
        let assign10220_e9172: f64 = (locals.var_gfsub2 * assign10220_e9171);
        let assign10220_e9173: f64 = (assign10220_e9159 + assign10220_e9172);
        (assign10220_e9173, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn4 - locals.var_spsub_temp1_dn4) + ((locals.var_spsub_delta_dn4 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn4))))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn6 - locals.var_spsub_temp1_dn6) + ((locals.var_spsub_delta_dn6 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn6))))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn7 - locals.var_spsub_temp1_dn7) + ((locals.var_spsub_delta_dn7 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn7))))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn8 - locals.var_spsub_temp1_dn8) + ((locals.var_spsub_delta_dn8 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn8))))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn9 - locals.var_spsub_temp1_dn9) + ((locals.var_spsub_delta_dn9 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn9))))))),)
    } else {
        (locals.var_spsub_pc, locals.var_spsub_pc_dn4, locals.var_spsub_pc_dn6, locals.var_spsub_pc_dn7, locals.var_spsub_pc_dn8, locals.var_spsub_pc_dn9,)
    }
};
        locals.var_spsub_pc = assign10220_e9175;
        locals.var_spsub_pc_dn4 = assign10220_e9175_d_n4;
        locals.var_spsub_pc_dn6 = assign10220_e9175_d_n6;
        locals.var_spsub_pc_dn7 = assign10220_e9175_d_n7;
        locals.var_spsub_pc_dn8 = assign10220_e9175_d_n8;
        locals.var_spsub_pc_dn9 = assign10220_e9175_d_n9;

        let (assign10230_e9204, assign10230_e9204_d_n4, assign10230_e9204_d_n6, assign10230_e9204_d_n7, assign10230_e9204_d_n8, assign10230_e9204_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10230_e9184: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
        let assign10230_e9188: f64 = (locals.var_spsub_delta0 - locals.var_spsub_y0);
        let assign10230_e9190: f64 = (assign10230_e9188 - 1.0);
        let assign10230_e9192: f64 = (assign10230_e9190 + locals.var_spsub_temp1);
        let assign10230_e9196: f64 = (locals.var_spsub_y0 - 1.0);
        let assign10230_e9198: f64 = (assign10230_e9196 - locals.var_spsub_xi0);
        let assign10230_e9199: f64 = (locals.var_spsub_delta * assign10230_e9198);
        let assign10230_e9200: f64 = (assign10230_e9192 + assign10230_e9199);
        let assign10230_e9201: f64 = (locals.var_gfsub2 * assign10230_e9200);
        let assign10230_e9202: f64 = (assign10230_e9184 - assign10230_e9201);
        (assign10230_e9202, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn4 - locals.var_spsub_y0_dn4) + locals.var_spsub_temp1_dn4) + ((locals.var_spsub_delta_dn4 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn4 - locals.var_spsub_xi0_dn4))))))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn6 - locals.var_spsub_y0_dn6) + locals.var_spsub_temp1_dn6) + ((locals.var_spsub_delta_dn6 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn6 - locals.var_spsub_xi0_dn6))))))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn7 - locals.var_spsub_y0_dn7) + locals.var_spsub_temp1_dn7) + ((locals.var_spsub_delta_dn7 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn7 - locals.var_spsub_xi0_dn7))))))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn8 - locals.var_spsub_y0_dn8) + locals.var_spsub_temp1_dn8) + ((locals.var_spsub_delta_dn8 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn8 - locals.var_spsub_xi0_dn8))))))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn9 - locals.var_spsub_y0_dn9) + locals.var_spsub_temp1_dn9) + ((locals.var_spsub_delta_dn9 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn9 - locals.var_spsub_xi0_dn9))))))),)
    } else {
        (locals.var_spsub_qc, locals.var_spsub_qc_dn4, locals.var_spsub_qc_dn6, locals.var_spsub_qc_dn7, locals.var_spsub_qc_dn8, locals.var_spsub_qc_dn9,)
    }
};
        locals.var_spsub_qc = assign10230_e9204;
        locals.var_spsub_qc_dn4 = assign10230_e9204_d_n4;
        locals.var_spsub_qc_dn6 = assign10230_e9204_d_n6;
        locals.var_spsub_qc_dn7 = assign10230_e9204_d_n7;
        locals.var_spsub_qc_dn8 = assign10230_e9204_d_n8;
        locals.var_spsub_qc_dn9 = assign10230_e9204_d_n9;

        let (assign10240_e9223, assign10240_e9223_d_n4, assign10240_e9223_d_n6, assign10240_e9223_d_n7, assign10240_e9223_d_n8, assign10240_e9223_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10240_e9215: f64 = (locals.var_spsub_delta0 + locals.var_spsub_temp1);
        let assign10240_e9218: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
        let assign10240_e9219: f64 = (assign10240_e9215 - assign10240_e9218);
        let assign10240_e9220: f64 = (locals.var_gfsub2 * assign10240_e9219);
        let assign10240_e9221: f64 = (2.0 - assign10240_e9220);
        (assign10240_e9221, (-((locals.var_gfsub2_dn4 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn4 + locals.var_spsub_temp1_dn4) - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4)))))), (-((locals.var_gfsub2_dn6 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn6 + locals.var_spsub_temp1_dn6) - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6)))))), (-((locals.var_gfsub2_dn7 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn7 + locals.var_spsub_temp1_dn7) - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7)))))), (-((locals.var_gfsub2_dn8 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn8 + locals.var_spsub_temp1_dn8) - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8)))))), (-((locals.var_gfsub2_dn9 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn9 + locals.var_spsub_temp1_dn9) - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9)))))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10240_e9223;
        locals.var_spsub_temp_dn4 = assign10240_e9223_d_n4;
        locals.var_spsub_temp_dn6 = assign10240_e9223_d_n6;
        locals.var_spsub_temp_dn7 = assign10240_e9223_d_n7;
        locals.var_spsub_temp_dn8 = assign10240_e9223_d_n8;
        locals.var_spsub_temp_dn9 = assign10240_e9223_d_n9;

        let (assign10250_e9240, assign10250_e9240_d_n4, assign10250_e9240_d_n6, assign10250_e9240_d_n7, assign10250_e9240_d_n8, assign10250_e9240_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10250_e9232: f64 = (locals.var_spsub_pc * locals.var_spsub_pc);
        let assign10250_e9236: f64 = (locals.var_spsub_qc * locals.var_spsub_temp);
        let assign10250_e9237: f64 = (2.0 * assign10250_e9236);
        let assign10250_e9238: f64 = (assign10250_e9232 - assign10250_e9237);
        (assign10250_e9238, (((locals.var_spsub_pc_dn4 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn4)) - (2.0 * ((locals.var_spsub_qc_dn4 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn4)))), (((locals.var_spsub_pc_dn6 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn6)) - (2.0 * ((locals.var_spsub_qc_dn6 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn6)))), (((locals.var_spsub_pc_dn7 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn7)) - (2.0 * ((locals.var_spsub_qc_dn7 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn7)))), (((locals.var_spsub_pc_dn8 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn8)) - (2.0 * ((locals.var_spsub_qc_dn8 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn8)))), (((locals.var_spsub_pc_dn9 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn9)) - (2.0 * ((locals.var_spsub_qc_dn9 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn9)))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10250_e9240;
        locals.var_spsub_temp_dn4 = assign10250_e9240_d_n4;
        locals.var_spsub_temp_dn6 = assign10250_e9240_d_n6;
        locals.var_spsub_temp_dn7 = assign10250_e9240_d_n7;
        locals.var_spsub_temp_dn8 = assign10250_e9240_d_n8;
        locals.var_spsub_temp_dn9 = assign10250_e9240_d_n9;

        let (assign10260_e9259, assign10260_e9259_d_n4, assign10260_e9259_d_n6, assign10260_e9259_d_n7, assign10260_e9259_d_n8, assign10260_e9259_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10260_e9248: f64 = (-locals.var_spsub_y0);
        let assign10260_e9253: f64 = (locals.var_spsub_temp).sqrt();
        let assign10260_e9254: f64 = (locals.var_spsub_pc + assign10260_e9253);
        let assign10260_e9255: f64 = (locals.var_spsub_qc / assign10260_e9254);
        let assign10260_e9256: f64 = (2.0 * assign10260_e9255);
        let assign10260_e9257: f64 = (assign10260_e9248 - assign10260_e9256);
        (assign10260_e9257, ((-locals.var_spsub_y0_dn4) - (2.0 * (((locals.var_spsub_qc_dn4 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn4 + (locals.var_spsub_temp_dn4 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), ((-locals.var_spsub_y0_dn6) - (2.0 * (((locals.var_spsub_qc_dn6 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn6 + (locals.var_spsub_temp_dn6 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), ((-locals.var_spsub_y0_dn7) - (2.0 * (((locals.var_spsub_qc_dn7 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn7 + (locals.var_spsub_temp_dn7 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), ((-locals.var_spsub_y0_dn8) - (2.0 * (((locals.var_spsub_qc_dn8 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn8 + (locals.var_spsub_temp_dn8 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), ((-locals.var_spsub_y0_dn9) - (2.0 * (((locals.var_spsub_qc_dn9 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn9 + (locals.var_spsub_temp_dn9 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign10260_e9259;
        locals.var_temp3_dn4 = assign10260_e9259_d_n4;
        locals.var_temp3_dn6 = assign10260_e9259_d_n6;
        locals.var_temp3_dn7 = assign10260_e9259_d_n7;
        locals.var_temp3_dn8 = assign10260_e9259_d_n8;
        locals.var_temp3_dn9 = assign10260_e9259_d_n9;

        let (assign10270_e9275, assign10270_e9275_d_n4, assign10270_e9275_d_n6, assign10270_e9275_d_n7, assign10270_e9275_d_n8, assign10270_e9275_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10270_e9271: f64 = (locals.var_gfsub * 0.732464877560822);
        let assign10270_e9272: f64 = (1.25 + assign10270_e9271);
        let assign10270_e9273: f64 = (1.0 / assign10270_e9272);
        (assign10270_e9273, (-((locals.var_gfsub_dn4 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), (-((locals.var_gfsub_dn6 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), (-((locals.var_gfsub_dn7 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), (-((locals.var_gfsub_dn8 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), (-((locals.var_gfsub_dn9 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))),)
    } else {
        (locals.var_spsub_xg1, locals.var_spsub_xg1_dn4, locals.var_spsub_xg1_dn6, locals.var_spsub_xg1_dn7, locals.var_spsub_xg1_dn8, locals.var_spsub_xg1_dn9,)
    }
};
        locals.var_spsub_xg1 = assign10270_e9275;
        locals.var_spsub_xg1_dn4 = assign10270_e9275_d_n4;
        locals.var_spsub_xg1_dn6 = assign10270_e9275_d_n6;
        locals.var_spsub_xg1_dn7 = assign10270_e9275_d_n7;
        locals.var_spsub_xg1_dn8 = assign10270_e9275_d_n8;
        locals.var_spsub_xg1_dn9 = assign10270_e9275_d_n9;

        let (assign10280_e9293, assign10280_e9293_d_n4, assign10280_e9293_d_n6, assign10280_e9293_d_n7, assign10280_e9293_d_n8, assign10280_e9293_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10280_e9285: f64 = (1.25 * locals.var_xisub);
        let assign10280_e9287: f64 = (assign10280_e9285 * locals.var_spsub_xg1);
        let assign10280_e9289: f64 = (assign10280_e9287 - 1.0);
        let assign10280_e9291: f64 = (assign10280_e9289 * locals.var_spsub_xg1);
        (assign10280_e9291, (((((1.25 * locals.var_xisub_dn4) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn4)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn4)), (((((1.25 * locals.var_xisub_dn6) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn6)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn6)), (((((1.25 * locals.var_xisub_dn7) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn7)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn7)), (((((1.25 * locals.var_xisub_dn8) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn8)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn8)), (((((1.25 * locals.var_xisub_dn9) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn9)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn9)),)
    } else {
        (locals.var_spsub_a_fac, locals.var_spsub_a_fac_dn4, locals.var_spsub_a_fac_dn6, locals.var_spsub_a_fac_dn7, locals.var_spsub_a_fac_dn8, locals.var_spsub_a_fac_dn9,)
    }
};
        locals.var_spsub_a_fac = assign10280_e9293;
        locals.var_spsub_a_fac_dn4 = assign10280_e9293_d_n4;
        locals.var_spsub_a_fac_dn6 = assign10280_e9293_d_n6;
        locals.var_spsub_a_fac_dn7 = assign10280_e9293_d_n7;
        locals.var_spsub_a_fac_dn8 = assign10280_e9293_d_n8;
        locals.var_spsub_a_fac_dn9 = assign10280_e9293_d_n9;

        let (assign10290_e9311, assign10290_e9311_d_n4, assign10290_e9311_d_n6, assign10290_e9311_d_n7, assign10290_e9311_d_n8, assign10290_e9311_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10290_e9303: f64 = (locals.var_spsub_xgb * locals.var_inv_xisub);
        let assign10290_e9307: f64 = (locals.var_spsub_a_fac * locals.var_spsub_xgb);
        let assign10290_e9308: f64 = (1.0 + assign10290_e9307);
        let assign10290_e9309: f64 = (assign10290_e9303 * assign10290_e9308);
        (assign10290_e9309, ((((locals.var_spsub_xgb_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn4)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn4 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn4)))), ((((locals.var_spsub_xgb_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn6)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn6 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn6)))), ((((locals.var_spsub_xgb_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn7)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn7 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn7)))), ((((locals.var_spsub_xgb_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn8)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn8 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn8)))), ((((locals.var_spsub_xgb_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn9)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn9 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn9)))),)
    } else {
        (locals.var_spsub_xbar, locals.var_spsub_xbar_dn4, locals.var_spsub_xbar_dn6, locals.var_spsub_xbar_dn7, locals.var_spsub_xbar_dn8, locals.var_spsub_xbar_dn9,)
    }
};
        locals.var_spsub_xbar = assign10290_e9311;
        locals.var_spsub_xbar_dn4 = assign10290_e9311_d_n4;
        locals.var_spsub_xbar_dn6 = assign10290_e9311_d_n6;
        locals.var_spsub_xbar_dn7 = assign10290_e9311_d_n7;
        locals.var_spsub_xbar_dn8 = assign10290_e9311_d_n8;
        locals.var_spsub_xbar_dn9 = assign10290_e9311_d_n9;

        let assign10300_e9313: f64 = (-locals.var_spsub_xbar);
        let assign10300_e9315: f64 = (-80.0);
        let assign10300_e9316: f64 = if assign10300_e9313 > assign10300_e9315 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign10300_e9316;

        let (assign10310_e9330, assign10310_e9330_d_n4, assign10310_e9330_d_n6, assign10310_e9330_d_n7, assign10310_e9330_d_n8, assign10310_e9330_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard540 != 0.0)) {
        let assign10310_e9327: f64 = (-locals.var_spsub_xbar);
        let assign10310_e9328: f64 = (assign10310_e9327).exp();
        (assign10310_e9328, (assign10310_e9328 * (-locals.var_spsub_xbar_dn4)), (assign10310_e9328 * (-locals.var_spsub_xbar_dn6)), (assign10310_e9328 * (-locals.var_spsub_xbar_dn7)), (assign10310_e9328 * (-locals.var_spsub_xbar_dn8)), (assign10310_e9328 * (-locals.var_spsub_xbar_dn9)),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10310_e9330;
        locals.var_spsub_temp_dn4 = assign10310_e9330_d_n4;
        locals.var_spsub_temp_dn6 = assign10310_e9330_d_n6;
        locals.var_spsub_temp_dn7 = assign10310_e9330_d_n7;
        locals.var_spsub_temp_dn8 = assign10310_e9330_d_n8;
        locals.var_spsub_temp_dn9 = assign10310_e9330_d_n9;

        let (assign10320_e9371, assign10320_e9371_d_n4, assign10320_e9371_d_n6, assign10320_e9371_d_n7, assign10320_e9371_d_n8, assign10320_e9371_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard540 == 0.0)) {
        let assign10320_e9344: f64 = (-locals.var_spsub_xbar);
        let assign10320_e9345: f64 = (-assign10320_e9344);
        let assign10320_e9347: f64 = (assign10320_e9345 - 80.0);
        let assign10320_e9351: f64 = (-locals.var_spsub_xbar);
        let assign10320_e9352: f64 = (-assign10320_e9351);
        let assign10320_e9354: f64 = (assign10320_e9352 - 80.0);
        let assign10320_e9355: f64 = (0.5 * assign10320_e9354);
        let assign10320_e9358: f64 = (-locals.var_spsub_xbar);
        let assign10320_e9359: f64 = (-assign10320_e9358);
        let assign10320_e9361: f64 = (assign10320_e9359 - 80.0);
        let assign10320_e9363: f64 = (assign10320_e9361 * 0.3333333333333);
        let assign10320_e9364: f64 = (1.0 + assign10320_e9363);
        let assign10320_e9365: f64 = (assign10320_e9355 * assign10320_e9364);
        let assign10320_e9366: f64 = (1.0 + assign10320_e9365);
        let assign10320_e9367: f64 = (assign10320_e9347 * assign10320_e9366);
        let assign10320_e9368: f64 = (1.0 + assign10320_e9367);
        let assign10320_e9369: f64 = (1.80485e-35 / assign10320_e9368);
        (assign10320_e9369, (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn4)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn4))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn4)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn6)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn6))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn6)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn7)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn7))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn7)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn8)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn8))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn8)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn9)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn9))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn9)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10320_e9371;
        locals.var_spsub_temp_dn4 = assign10320_e9371_d_n4;
        locals.var_spsub_temp_dn6 = assign10320_e9371_d_n6;
        locals.var_spsub_temp_dn7 = assign10320_e9371_d_n7;
        locals.var_spsub_temp_dn8 = assign10320_e9371_d_n8;
        locals.var_spsub_temp_dn9 = assign10320_e9371_d_n9;

        let (assign10330_e9383, assign10330_e9383_d_n4, assign10330_e9383_d_n6, assign10330_e9383_d_n7, assign10330_e9383_d_n8, assign10330_e9383_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10330_e9381: f64 = (1.0 - locals.var_spsub_temp);
        (assign10330_e9381, (-locals.var_spsub_temp_dn4), (-locals.var_spsub_temp_dn6), (-locals.var_spsub_temp_dn7), (-locals.var_spsub_temp_dn8), (-locals.var_spsub_temp_dn9),)
    } else {
        (locals.var_spsub_w, locals.var_spsub_w_dn4, locals.var_spsub_w_dn6, locals.var_spsub_w_dn7, locals.var_spsub_w_dn8, locals.var_spsub_w_dn9,)
    }
};
        locals.var_spsub_w = assign10330_e9383;
        locals.var_spsub_w_dn4 = assign10330_e9383_d_n4;
        locals.var_spsub_w_dn6 = assign10330_e9383_d_n6;
        locals.var_spsub_w_dn7 = assign10330_e9383_d_n7;
        locals.var_spsub_w_dn8 = assign10330_e9383_d_n8;
        locals.var_spsub_w_dn9 = assign10330_e9383_d_n9;

        let (assign10340_e9408, assign10340_e9408_d_n4, assign10340_e9408_d_n6, assign10340_e9408_d_n7, assign10340_e9408_d_n8, assign10340_e9408_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10340_e9394: f64 = (locals.var_gfsub2 * 0.5);
        let assign10340_e9395: f64 = (locals.var_spsub_xgb + assign10340_e9394);
        let assign10340_e9400: f64 = (locals.var_gfsub2 * 0.25);
        let assign10340_e9401: f64 = (locals.var_spsub_xgb + assign10340_e9400);
        let assign10340_e9403: f64 = (assign10340_e9401 - locals.var_spsub_w);
        let assign10340_e9404: f64 = (assign10340_e9403).sqrt();
        let assign10340_e9405: f64 = (locals.var_gfsub * assign10340_e9404);
        let assign10340_e9406: f64 = (assign10340_e9395 - assign10340_e9405);
        (assign10340_e9406, ((locals.var_spsub_xgb_dn4 + (locals.var_gfsub2_dn4 * 0.5)) - ((locals.var_gfsub_dn4 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn4 + (locals.var_gfsub2_dn4 * 0.25)) - locals.var_spsub_w_dn4) / (2.0 * assign10340_e9404))))), ((locals.var_spsub_xgb_dn6 + (locals.var_gfsub2_dn6 * 0.5)) - ((locals.var_gfsub_dn6 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn6 + (locals.var_gfsub2_dn6 * 0.25)) - locals.var_spsub_w_dn6) / (2.0 * assign10340_e9404))))), ((locals.var_spsub_xgb_dn7 + (locals.var_gfsub2_dn7 * 0.5)) - ((locals.var_gfsub_dn7 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn7 + (locals.var_gfsub2_dn7 * 0.25)) - locals.var_spsub_w_dn7) / (2.0 * assign10340_e9404))))), ((locals.var_spsub_xgb_dn8 + (locals.var_gfsub2_dn8 * 0.5)) - ((locals.var_gfsub_dn8 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn8 + (locals.var_gfsub2_dn8 * 0.25)) - locals.var_spsub_w_dn8) / (2.0 * assign10340_e9404))))), ((locals.var_spsub_xgb_dn9 + (locals.var_gfsub2_dn9 * 0.5)) - ((locals.var_gfsub_dn9 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn9 + (locals.var_gfsub2_dn9 * 0.25)) - locals.var_spsub_w_dn9) / (2.0 * assign10340_e9404))))),)
    } else {
        (locals.var_spsub_x1, locals.var_spsub_x1_dn4, locals.var_spsub_x1_dn6, locals.var_spsub_x1_dn7, locals.var_spsub_x1_dn8, locals.var_spsub_x1_dn9,)
    }
};
        locals.var_spsub_x1 = assign10340_e9408;
        locals.var_spsub_x1_dn4 = assign10340_e9408_d_n4;
        locals.var_spsub_x1_dn6 = assign10340_e9408_d_n6;
        locals.var_spsub_x1_dn7 = assign10340_e9408_d_n7;
        locals.var_spsub_x1_dn8 = assign10340_e9408_d_n8;
        locals.var_spsub_x1_dn9 = assign10340_e9408_d_n9;

        let (assign10350_e9420, assign10350_e9420_d_n4, assign10350_e9420_d_n6, assign10350_e9420_d_n7, assign10350_e9420_d_n8, assign10350_e9420_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10350_e9418: f64 = (locals.var_xn_sub + 3.0);
        (assign10350_e9418, locals.var_xn_sub_dn4, locals.var_xn_sub_dn6, locals.var_xn_sub_dn7, locals.var_xn_sub_dn8, locals.var_xn_sub_dn9,)
    } else {
        (locals.var_spsub_bx, locals.var_spsub_bx_dn4, locals.var_spsub_bx_dn6, locals.var_spsub_bx_dn7, locals.var_spsub_bx_dn8, locals.var_spsub_bx_dn9,)
    }
};
        locals.var_spsub_bx = assign10350_e9420;
        locals.var_spsub_bx_dn4 = assign10350_e9420_d_n4;
        locals.var_spsub_bx_dn6 = assign10350_e9420_d_n6;
        locals.var_spsub_bx_dn7 = assign10350_e9420_d_n7;
        locals.var_spsub_bx_dn8 = assign10350_e9420_d_n8;
        locals.var_spsub_bx_dn9 = assign10350_e9420_d_n9;

        let (assign10360_e9456, assign10360_e9456_d_n4, assign10360_e9456_d_n6, assign10360_e9456_d_n7, assign10360_e9456_d_n8, assign10360_e9456_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10360_e9431: f64 = (locals.var_spsub_x1 + locals.var_spsub_bx);
        let assign10360_e9434: f64 = (locals.var_spsub_x1 - locals.var_spsub_bx);
        let assign10360_e9437: f64 = (locals.var_spsub_x1 - locals.var_spsub_bx);
        let assign10360_e9438: f64 = (assign10360_e9434 * assign10360_e9437);
        let assign10360_e9440: f64 = (assign10360_e9438 + 5.0);
        let assign10360_e9441: f64 = (assign10360_e9440).sqrt();
        let assign10360_e9442: f64 = (assign10360_e9431 - assign10360_e9441);
        let assign10360_e9443: f64 = (0.5 * assign10360_e9442);
        let assign10360_e9448: f64 = (locals.var_spsub_bx * locals.var_spsub_bx);
        let assign10360_e9450: f64 = (assign10360_e9448 + 5.0);
        let assign10360_e9451: f64 = (assign10360_e9450).sqrt();
        let assign10360_e9452: f64 = (locals.var_spsub_bx - assign10360_e9451);
        let assign10360_e9453: f64 = (0.5 * assign10360_e9452);
        let assign10360_e9454: f64 = (assign10360_e9443 - assign10360_e9453);
        (assign10360_e9454, ((0.5 * ((locals.var_spsub_x1_dn4 + locals.var_spsub_bx_dn4) - ((((locals.var_spsub_x1_dn4 - locals.var_spsub_bx_dn4) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn4 - locals.var_spsub_bx_dn4))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn4 - (((locals.var_spsub_bx_dn4 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn4)) / (2.0 * assign10360_e9451))))), ((0.5 * ((locals.var_spsub_x1_dn6 + locals.var_spsub_bx_dn6) - ((((locals.var_spsub_x1_dn6 - locals.var_spsub_bx_dn6) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn6 - locals.var_spsub_bx_dn6))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn6 - (((locals.var_spsub_bx_dn6 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn6)) / (2.0 * assign10360_e9451))))), ((0.5 * ((locals.var_spsub_x1_dn7 + locals.var_spsub_bx_dn7) - ((((locals.var_spsub_x1_dn7 - locals.var_spsub_bx_dn7) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn7 - locals.var_spsub_bx_dn7))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn7 - (((locals.var_spsub_bx_dn7 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn7)) / (2.0 * assign10360_e9451))))), ((0.5 * ((locals.var_spsub_x1_dn8 + locals.var_spsub_bx_dn8) - ((((locals.var_spsub_x1_dn8 - locals.var_spsub_bx_dn8) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn8 - locals.var_spsub_bx_dn8))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn8 - (((locals.var_spsub_bx_dn8 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn8)) / (2.0 * assign10360_e9451))))), ((0.5 * ((locals.var_spsub_x1_dn9 + locals.var_spsub_bx_dn9) - ((((locals.var_spsub_x1_dn9 - locals.var_spsub_bx_dn9) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn9 - locals.var_spsub_bx_dn9))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn9 - (((locals.var_spsub_bx_dn9 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn9)) / (2.0 * assign10360_e9451))))),)
    } else {
        (locals.var_spsub_eta, locals.var_spsub_eta_dn4, locals.var_spsub_eta_dn6, locals.var_spsub_eta_dn7, locals.var_spsub_eta_dn8, locals.var_spsub_eta_dn9,)
    }
};
        locals.var_spsub_eta = assign10360_e9456;
        locals.var_spsub_eta_dn4 = assign10360_e9456_d_n4;
        locals.var_spsub_eta_dn6 = assign10360_e9456_d_n6;
        locals.var_spsub_eta_dn7 = assign10360_e9456_d_n7;
        locals.var_spsub_eta_dn8 = assign10360_e9456_d_n8;
        locals.var_spsub_eta_dn9 = assign10360_e9456_d_n9;

    }

    pub(super) fn stamp_transient_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign10370_e9468, assign10370_e9468_d_n4, assign10370_e9468_d_n6, assign10370_e9468_d_n7, assign10370_e9468_d_n8, assign10370_e9468_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10370_e9466: f64 = (locals.var_spsub_xgb - locals.var_spsub_eta);
        (assign10370_e9466, (locals.var_spsub_xgb_dn4 - locals.var_spsub_eta_dn4), (locals.var_spsub_xgb_dn6 - locals.var_spsub_eta_dn6), (locals.var_spsub_xgb_dn7 - locals.var_spsub_eta_dn7), (locals.var_spsub_xgb_dn8 - locals.var_spsub_eta_dn8), (locals.var_spsub_xgb_dn9 - locals.var_spsub_eta_dn9),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10370_e9468;
        locals.var_spsub_temp_dn4 = assign10370_e9468_d_n4;
        locals.var_spsub_temp_dn6 = assign10370_e9468_d_n6;
        locals.var_spsub_temp_dn7 = assign10370_e9468_d_n7;
        locals.var_spsub_temp_dn8 = assign10370_e9468_d_n8;
        locals.var_spsub_temp_dn9 = assign10370_e9468_d_n9;

        let (assign10380_e9480, assign10380_e9480_d_n4, assign10380_e9480_d_n6, assign10380_e9480_d_n7, assign10380_e9480_d_n8, assign10380_e9480_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10380_e9477: f64 = (-locals.var_spsub_eta);
        let assign10380_e9478: f64 = (assign10380_e9477).exp();
        (assign10380_e9478, (assign10380_e9478 * (-locals.var_spsub_eta_dn4)), (assign10380_e9478 * (-locals.var_spsub_eta_dn6)), (assign10380_e9478 * (-locals.var_spsub_eta_dn7)), (assign10380_e9478 * (-locals.var_spsub_eta_dn8)), (assign10380_e9478 * (-locals.var_spsub_eta_dn9)),)
    } else {
        (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9,)
    }
};
        locals.var_spsub_temp1 = assign10380_e9480;
        locals.var_spsub_temp1_dn4 = assign10380_e9480_d_n4;
        locals.var_spsub_temp1_dn6 = assign10380_e9480_d_n6;
        locals.var_spsub_temp1_dn7 = assign10380_e9480_d_n7;
        locals.var_spsub_temp1_dn8 = assign10380_e9480_d_n8;
        locals.var_spsub_temp1_dn9 = assign10380_e9480_d_n9;

        let (assign10390_e9496, assign10390_e9496_d_n4, assign10390_e9496_d_n6, assign10390_e9496_d_n7, assign10390_e9496_d_n8, assign10390_e9496_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10390_e9492: f64 = (locals.var_spsub_eta * locals.var_spsub_eta);
        let assign10390_e9493: f64 = (2.0 + assign10390_e9492);
        let assign10390_e9494: f64 = (1.0 / assign10390_e9493);
        (assign10390_e9494, (-(((locals.var_spsub_eta_dn4 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn4)) / (assign10390_e9493 * assign10390_e9493))), (-(((locals.var_spsub_eta_dn6 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn6)) / (assign10390_e9493 * assign10390_e9493))), (-(((locals.var_spsub_eta_dn7 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn7)) / (assign10390_e9493 * assign10390_e9493))), (-(((locals.var_spsub_eta_dn8 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn8)) / (assign10390_e9493 * assign10390_e9493))), (-(((locals.var_spsub_eta_dn9 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn9)) / (assign10390_e9493 * assign10390_e9493))),)
    } else {
        (locals.var_spsub_temp2, locals.var_spsub_temp2_dn4, locals.var_spsub_temp2_dn6, locals.var_spsub_temp2_dn7, locals.var_spsub_temp2_dn8, locals.var_spsub_temp2_dn9,)
    }
};
        locals.var_spsub_temp2 = assign10390_e9496;
        locals.var_spsub_temp2_dn4 = assign10390_e9496_d_n4;
        locals.var_spsub_temp2_dn6 = assign10390_e9496_d_n6;
        locals.var_spsub_temp2_dn7 = assign10390_e9496_d_n7;
        locals.var_spsub_temp2_dn8 = assign10390_e9496_d_n8;
        locals.var_spsub_temp2_dn9 = assign10390_e9496_d_n9;

        let (assign10400_e9510, assign10400_e9510_d_n4, assign10400_e9510_d_n6, assign10400_e9510_d_n7, assign10400_e9510_d_n8, assign10400_e9510_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10400_e9506: f64 = (locals.var_spsub_eta * locals.var_spsub_eta);
        let assign10400_e9508: f64 = (assign10400_e9506 * locals.var_spsub_temp2);
        (assign10400_e9508, ((((locals.var_spsub_eta_dn4 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn4)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn4)), ((((locals.var_spsub_eta_dn6 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn6)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn6)), ((((locals.var_spsub_eta_dn7 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn7)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn7)), ((((locals.var_spsub_eta_dn8 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn8)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn8)), ((((locals.var_spsub_eta_dn9 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn9)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn9)),)
    } else {
        (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9,)
    }
};
        locals.var_spsub_xi0 = assign10400_e9510;
        locals.var_spsub_xi0_dn4 = assign10400_e9510_d_n4;
        locals.var_spsub_xi0_dn6 = assign10400_e9510_d_n6;
        locals.var_spsub_xi0_dn7 = assign10400_e9510_d_n7;
        locals.var_spsub_xi0_dn8 = assign10400_e9510_d_n8;
        locals.var_spsub_xi0_dn9 = assign10400_e9510_d_n9;

        let (assign10410_e9526, assign10410_e9526_d_n4, assign10410_e9526_d_n6, assign10410_e9526_d_n7, assign10410_e9526_d_n8, assign10410_e9526_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10410_e9521: f64 = (locals.var_spsub_eta * locals.var_spsub_temp2);
        let assign10410_e9523: f64 = (assign10410_e9521 * locals.var_spsub_temp2);
        let assign10410_e9524: f64 = (4.0 * assign10410_e9523);
        (assign10410_e9524, (4.0 * ((((locals.var_spsub_eta_dn4 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn4)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn4))), (4.0 * ((((locals.var_spsub_eta_dn6 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn6)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn6))), (4.0 * ((((locals.var_spsub_eta_dn7 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn7)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn7))), (4.0 * ((((locals.var_spsub_eta_dn8 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn8)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn8))), (4.0 * ((((locals.var_spsub_eta_dn9 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn9)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn9))),)
    } else {
        (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9,)
    }
};
        locals.var_spsub_xi1 = assign10410_e9526;
        locals.var_spsub_xi1_dn4 = assign10410_e9526_d_n4;
        locals.var_spsub_xi1_dn6 = assign10410_e9526_d_n6;
        locals.var_spsub_xi1_dn7 = assign10410_e9526_d_n7;
        locals.var_spsub_xi1_dn8 = assign10410_e9526_d_n8;
        locals.var_spsub_xi1_dn9 = assign10410_e9526_d_n9;

        let (assign10420_e9546, assign10420_e9546_d_n4, assign10420_e9546_d_n6, assign10420_e9546_d_n7, assign10420_e9546_d_n8, assign10420_e9546_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10420_e9536: f64 = (8.0 * locals.var_spsub_temp2);
        let assign10420_e9539: f64 = (12.0 * locals.var_spsub_xi0);
        let assign10420_e9540: f64 = (assign10420_e9536 - assign10420_e9539);
        let assign10420_e9542: f64 = (assign10420_e9540 * locals.var_spsub_temp2);
        let assign10420_e9544: f64 = (assign10420_e9542 * locals.var_spsub_temp2);
        (assign10420_e9544, ((((((8.0 * locals.var_spsub_temp2_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn4)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn4)), ((((((8.0 * locals.var_spsub_temp2_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn6)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn6)), ((((((8.0 * locals.var_spsub_temp2_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn7)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn7)), ((((((8.0 * locals.var_spsub_temp2_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn8)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn8)), ((((((8.0 * locals.var_spsub_temp2_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn9)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn9)),)
    } else {
        (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9,)
    }
};
        locals.var_spsub_xi2 = assign10420_e9546;
        locals.var_spsub_xi2_dn4 = assign10420_e9546_d_n4;
        locals.var_spsub_xi2_dn6 = assign10420_e9546_d_n6;
        locals.var_spsub_xi2_dn7 = assign10420_e9546_d_n7;
        locals.var_spsub_xi2_dn8 = assign10420_e9546_d_n8;
        locals.var_spsub_xi2_dn9 = assign10420_e9546_d_n9;

        let (assign10430_e9576, assign10430_e9576_d_n4, assign10430_e9576_d_n6, assign10430_e9576_d_n7, assign10430_e9576_d_n8, assign10430_e9576_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10430_e9557: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
        let assign10430_e9561: f64 = (locals.var_spsub_temp1 + locals.var_spsub_eta);
        let assign10430_e9563: f64 = (assign10430_e9561 - 1.0);
        let assign10430_e9567: f64 = (locals.var_spsub_eta + 1.0);
        let assign10430_e9569: f64 = (assign10430_e9567 + locals.var_spsub_xi0);
        let assign10430_e9570: f64 = (locals.var_spsub_delta * assign10430_e9569);
        let assign10430_e9571: f64 = (assign10430_e9563 - assign10430_e9570);
        let assign10430_e9572: f64 = (locals.var_gfsub2 * assign10430_e9571);
        let assign10430_e9573: f64 = (assign10430_e9557 - assign10430_e9572);
        let assign10430_e9574: f64 = (1e-40_f64).max(assign10430_e9573);
        (assign10430_e9574, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn4 + locals.var_spsub_eta_dn4) - ((locals.var_spsub_delta_dn4 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn4 + locals.var_spsub_xi0_dn4))))))) }, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn6 + locals.var_spsub_eta_dn6) - ((locals.var_spsub_delta_dn6 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn6 + locals.var_spsub_xi0_dn6))))))) }, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn7 + locals.var_spsub_eta_dn7) - ((locals.var_spsub_delta_dn7 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn7 + locals.var_spsub_xi0_dn7))))))) }, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn8 + locals.var_spsub_eta_dn8) - ((locals.var_spsub_delta_dn8 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn8 + locals.var_spsub_xi0_dn8))))))) }, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn9 + locals.var_spsub_eta_dn9) - ((locals.var_spsub_delta_dn9 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn9 + locals.var_spsub_xi0_dn9))))))) },)
    } else {
        (locals.var_spsub_a, locals.var_spsub_a_dn4, locals.var_spsub_a_dn6, locals.var_spsub_a_dn7, locals.var_spsub_a_dn8, locals.var_spsub_a_dn9,)
    }
};
        locals.var_spsub_a = assign10430_e9576;
        locals.var_spsub_a_dn4 = assign10430_e9576_d_n4;
        locals.var_spsub_a_dn6 = assign10430_e9576_d_n6;
        locals.var_spsub_a_dn7 = assign10430_e9576_d_n7;
        locals.var_spsub_a_dn8 = assign10430_e9576_d_n8;
        locals.var_spsub_a_dn9 = assign10430_e9576_d_n9;

        let (assign10440_e9596, assign10440_e9596_d_n4, assign10440_e9596_d_n6, assign10440_e9596_d_n7, assign10440_e9596_d_n8, assign10440_e9596_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10440_e9590: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
        let assign10440_e9591: f64 = (locals.var_spsub_temp1 - assign10440_e9590);
        let assign10440_e9592: f64 = (locals.var_gfsub2 * assign10440_e9591);
        let assign10440_e9593: f64 = (0.5 * assign10440_e9592);
        let assign10440_e9594: f64 = (1.0 - assign10440_e9593);
        (assign10440_e9594, (-(0.5 * ((locals.var_gfsub2_dn4 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn4 - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4))))))), (-(0.5 * ((locals.var_gfsub2_dn6 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn6 - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6))))))), (-(0.5 * ((locals.var_gfsub2_dn7 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn7 - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7))))))), (-(0.5 * ((locals.var_gfsub2_dn8 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn8 - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8))))))), (-(0.5 * ((locals.var_gfsub2_dn9 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn9 - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9))))))),)
    } else {
        (locals.var_spsub_b, locals.var_spsub_b_dn4, locals.var_spsub_b_dn6, locals.var_spsub_b_dn7, locals.var_spsub_b_dn8, locals.var_spsub_b_dn9,)
    }
};
        locals.var_spsub_b = assign10440_e9596;
        locals.var_spsub_b_dn4 = assign10440_e9596_d_n4;
        locals.var_spsub_b_dn6 = assign10440_e9596_d_n6;
        locals.var_spsub_b_dn7 = assign10440_e9596_d_n7;
        locals.var_spsub_b_dn8 = assign10440_e9596_d_n8;
        locals.var_spsub_b_dn9 = assign10440_e9596_d_n9;

        let (assign10450_e9620, assign10450_e9620_d_n4, assign10450_e9620_d_n6, assign10450_e9620_d_n7, assign10450_e9620_d_n8, assign10450_e9620_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10450_e9606: f64 = (2.0 * locals.var_spsub_temp);
        let assign10450_e9610: f64 = (1.0 - locals.var_spsub_temp1);
        let assign10450_e9614: f64 = (1.0 + locals.var_spsub_xi1);
        let assign10450_e9615: f64 = (locals.var_spsub_delta * assign10450_e9614);
        let assign10450_e9616: f64 = (assign10450_e9610 - assign10450_e9615);
        let assign10450_e9617: f64 = (locals.var_gfsub2 * assign10450_e9616);
        let assign10450_e9618: f64 = (assign10450_e9606 + assign10450_e9617);
        (assign10450_e9618, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn4) - ((locals.var_spsub_delta_dn4 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn4)))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn6) - ((locals.var_spsub_delta_dn6 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn6)))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn7) - ((locals.var_spsub_delta_dn7 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn7)))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn8) - ((locals.var_spsub_delta_dn8 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn8)))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn9) - ((locals.var_spsub_delta_dn9 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn9)))))),)
    } else {
        (locals.var_spsub_c, locals.var_spsub_c_dn4, locals.var_spsub_c_dn6, locals.var_spsub_c_dn7, locals.var_spsub_c_dn8, locals.var_spsub_c_dn9,)
    }
};
        locals.var_spsub_c = assign10450_e9620;
        locals.var_spsub_c_dn4 = assign10450_e9620_d_n4;
        locals.var_spsub_c_dn6 = assign10450_e9620_d_n6;
        locals.var_spsub_c_dn7 = assign10450_e9620_d_n7;
        locals.var_spsub_c_dn8 = assign10450_e9620_d_n8;
        locals.var_spsub_c_dn9 = assign10450_e9620_d_n9;

        let (assign10460_e9637, assign10460_e9637_d_n4, assign10460_e9637_d_n6, assign10460_e9637_d_n7, assign10460_e9637_d_n8, assign10460_e9637_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10460_e9630: f64 = (locals.var_xn_sub - locals.var_spsub_eta);
        let assign10460_e9633: f64 = (locals.var_spsub_a / locals.var_gfsub2);
        let assign10460_e9634: f64 = (assign10460_e9633).ln();
        let assign10460_e9635: f64 = (assign10460_e9630 + assign10460_e9634);
        (assign10460_e9635, ((locals.var_xn_sub_dn4 - locals.var_spsub_eta_dn4) + ((((locals.var_spsub_a_dn4 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn4)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), ((locals.var_xn_sub_dn6 - locals.var_spsub_eta_dn6) + ((((locals.var_spsub_a_dn6 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn6)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), ((locals.var_xn_sub_dn7 - locals.var_spsub_eta_dn7) + ((((locals.var_spsub_a_dn7 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn7)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), ((locals.var_xn_sub_dn8 - locals.var_spsub_eta_dn8) + ((((locals.var_spsub_a_dn8 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn8)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), ((locals.var_xn_sub_dn9 - locals.var_spsub_eta_dn9) + ((((locals.var_spsub_a_dn9 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn9)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)),)
    } else {
        (locals.var_spsub_tau, locals.var_spsub_tau_dn4, locals.var_spsub_tau_dn6, locals.var_spsub_tau_dn7, locals.var_spsub_tau_dn8, locals.var_spsub_tau_dn9,)
    }
};
        locals.var_spsub_tau = assign10460_e9637;
        locals.var_spsub_tau_dn4 = assign10460_e9637_d_n4;
        locals.var_spsub_tau_dn6 = assign10460_e9637_d_n6;
        locals.var_spsub_tau_dn7 = assign10460_e9637_d_n7;
        locals.var_spsub_tau_dn8 = assign10460_e9637_d_n8;
        locals.var_spsub_tau_dn9 = assign10460_e9637_d_n9;

        let (assign10470_e9649, assign10470_e9649_d_n4, assign10470_e9649_d_n6, assign10470_e9649_d_n7, assign10470_e9649_d_n8, assign10470_e9649_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10470_e9647: f64 = (locals.var_spsub_a + locals.var_spsub_c);
        (assign10470_e9647, (locals.var_spsub_a_dn4 + locals.var_spsub_c_dn4), (locals.var_spsub_a_dn6 + locals.var_spsub_c_dn6), (locals.var_spsub_a_dn7 + locals.var_spsub_c_dn7), (locals.var_spsub_a_dn8 + locals.var_spsub_c_dn8), (locals.var_spsub_a_dn9 + locals.var_spsub_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign10470_e9649;
        locals.var_nu_dn4 = assign10470_e9649_d_n4;
        locals.var_nu_dn6 = assign10470_e9649_d_n6;
        locals.var_nu_dn7 = assign10470_e9649_d_n7;
        locals.var_nu_dn8 = assign10470_e9649_d_n8;
        locals.var_nu_dn9 = assign10470_e9649_d_n9;

        let (assign10480_e9673, assign10480_e9673_d_n4, assign10480_e9673_d_n6, assign10480_e9673_d_n7, assign10480_e9673_d_n8, assign10480_e9673_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10480_e9659: f64 = (locals.var_nu * locals.var_nu);
        let assign10480_e9663: f64 = (0.5 * locals.var_spsub_c);
        let assign10480_e9665: f64 = (assign10480_e9663 * locals.var_spsub_c);
        let assign10480_e9668: f64 = (locals.var_spsub_a * locals.var_spsub_b);
        let assign10480_e9669: f64 = (assign10480_e9665 - assign10480_e9668);
        let assign10480_e9670: f64 = (locals.var_spsub_tau * assign10480_e9669);
        let assign10480_e9671: f64 = (assign10480_e9659 + assign10480_e9670);
        (assign10480_e9671, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_spsub_tau_dn4 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn4) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn4)) - ((locals.var_spsub_a_dn4 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_spsub_tau_dn6 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn6) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn6)) - ((locals.var_spsub_a_dn6 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_spsub_tau_dn7 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn7) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn7)) - ((locals.var_spsub_a_dn7 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_spsub_tau_dn8 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn8) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn8)) - ((locals.var_spsub_a_dn8 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_spsub_tau_dn9 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn9) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn9)) - ((locals.var_spsub_a_dn9 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign10480_e9673;
        locals.var_mutau_dn4 = assign10480_e9673_d_n4;
        locals.var_mutau_dn6 = assign10480_e9673_d_n6;
        locals.var_mutau_dn7 = assign10480_e9673_d_n7;
        locals.var_mutau_dn8 = assign10480_e9673_d_n8;
        locals.var_mutau_dn9 = assign10480_e9673_d_n9;

        let (assign10490_e9711, assign10490_e9711_d_n4, assign10490_e9711_d_n6, assign10490_e9711_d_n7, assign10490_e9711_d_n8, assign10490_e9711_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10490_e9684: f64 = (locals.var_spsub_a * locals.var_nu);
        let assign10490_e9686: f64 = (assign10490_e9684 * locals.var_spsub_tau);
        let assign10490_e9690: f64 = (locals.var_nu / locals.var_mutau);
        let assign10490_e9692: f64 = (assign10490_e9690 * locals.var_spsub_tau);
        let assign10490_e9694: f64 = (assign10490_e9692 * locals.var_spsub_tau);
        let assign10490_e9696: f64 = (assign10490_e9694 * locals.var_spsub_c);
        let assign10490_e9699: f64 = (locals.var_spsub_c * locals.var_spsub_c);
        let assign10490_e9701: f64 = (assign10490_e9699 * 0.3333333333333);
        let assign10490_e9704: f64 = (locals.var_spsub_a * locals.var_spsub_b);
        let assign10490_e9705: f64 = (assign10490_e9701 - assign10490_e9704);
        let assign10490_e9706: f64 = (assign10490_e9696 * assign10490_e9705);
        let assign10490_e9707: f64 = (locals.var_mutau + assign10490_e9706);
        let assign10490_e9708: f64 = (assign10490_e9686 / assign10490_e9707);
        let assign10490_e9709: f64 = (locals.var_spsub_eta + assign10490_e9708);
        (assign10490_e9709, (locals.var_spsub_eta_dn4 + (((((((locals.var_spsub_a_dn4 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn4)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn4)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn4)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn4)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn4)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn4 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn4)) * 0.3333333333333) - ((locals.var_spsub_a_dn4 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn4)))))))) / (assign10490_e9707 * assign10490_e9707))), (locals.var_spsub_eta_dn6 + (((((((locals.var_spsub_a_dn6 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn6)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn6)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn6)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn6)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn6)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn6 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn6)) * 0.3333333333333) - ((locals.var_spsub_a_dn6 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn6)))))))) / (assign10490_e9707 * assign10490_e9707))), (locals.var_spsub_eta_dn7 + (((((((locals.var_spsub_a_dn7 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn7)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn7)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn7)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn7)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn7)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn7 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn7)) * 0.3333333333333) - ((locals.var_spsub_a_dn7 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn7)))))))) / (assign10490_e9707 * assign10490_e9707))), (locals.var_spsub_eta_dn8 + (((((((locals.var_spsub_a_dn8 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn8)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn8)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn8)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn8)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn8)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn8 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn8)) * 0.3333333333333) - ((locals.var_spsub_a_dn8 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn8)))))))) / (assign10490_e9707 * assign10490_e9707))), (locals.var_spsub_eta_dn9 + (((((((locals.var_spsub_a_dn9 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn9)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn9)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn9)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn9)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn9)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn9 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn9)) * 0.3333333333333) - ((locals.var_spsub_a_dn9 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn9)))))))) / (assign10490_e9707 * assign10490_e9707))),)
    } else {
        (locals.var_spsub_x0, locals.var_spsub_x0_dn4, locals.var_spsub_x0_dn6, locals.var_spsub_x0_dn7, locals.var_spsub_x0_dn8, locals.var_spsub_x0_dn9,)
    }
};
        locals.var_spsub_x0 = assign10490_e9711;
        locals.var_spsub_x0_dn4 = assign10490_e9711_d_n4;
        locals.var_spsub_x0_dn6 = assign10490_e9711_d_n6;
        locals.var_spsub_x0_dn7 = assign10490_e9711_d_n7;
        locals.var_spsub_x0_dn8 = assign10490_e9711_d_n8;
        locals.var_spsub_x0_dn9 = assign10490_e9711_d_n9;

        let assign10500_e9714: f64 = if locals.var_spsub_x0 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign10500_e9714;

        let (assign10510_e9727, assign10510_e9727_d_n4, assign10510_e9727_d_n6, assign10510_e9727_d_n7, assign10510_e9727_d_n8, assign10510_e9727_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
        let assign10510_e9725: f64 = (locals.var_spsub_x0).exp();
        (assign10510_e9725, (assign10510_e9725 * locals.var_spsub_x0_dn4), (assign10510_e9725 * locals.var_spsub_x0_dn6), (assign10510_e9725 * locals.var_spsub_x0_dn7), (assign10510_e9725 * locals.var_spsub_x0_dn8), (assign10510_e9725 * locals.var_spsub_x0_dn9),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10510_e9727;
        locals.var_spsub_delta0_dn4 = assign10510_e9727_d_n4;
        locals.var_spsub_delta0_dn6 = assign10510_e9727_d_n6;
        locals.var_spsub_delta0_dn7 = assign10510_e9727_d_n7;
        locals.var_spsub_delta0_dn8 = assign10510_e9727_d_n8;
        locals.var_spsub_delta0_dn9 = assign10510_e9727_d_n9;

        let (assign10520_e9741, assign10520_e9741_d_n4, assign10520_e9741_d_n6, assign10520_e9741_d_n7, assign10520_e9741_d_n8, assign10520_e9741_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
        let assign10520_e9739: f64 = (1.0 / locals.var_spsub_delta0);
        (assign10520_e9739, (-(locals.var_spsub_delta0_dn4 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn6 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn7 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn8 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn9 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))),)
    } else {
        (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9,)
    }
};
        locals.var_spsub_delta1 = assign10520_e9741;
        locals.var_spsub_delta1_dn4 = assign10520_e9741_d_n4;
        locals.var_spsub_delta1_dn6 = assign10520_e9741_d_n6;
        locals.var_spsub_delta1_dn7 = assign10520_e9741_d_n7;
        locals.var_spsub_delta1_dn8 = assign10520_e9741_d_n8;
        locals.var_spsub_delta1_dn9 = assign10520_e9741_d_n9;

        let (assign10530_e9755, assign10530_e9755_d_n4, assign10530_e9755_d_n6, assign10530_e9755_d_n7, assign10530_e9755_d_n8, assign10530_e9755_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
        let assign10530_e9753: f64 = (locals.var_spsub_delta * locals.var_spsub_delta0);
        (assign10530_e9753, ((locals.var_spsub_delta_dn4 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn4)), ((locals.var_spsub_delta_dn6 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn6)), ((locals.var_spsub_delta_dn7 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn7)), ((locals.var_spsub_delta_dn8 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn8)), ((locals.var_spsub_delta_dn9 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn9)),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10530_e9755;
        locals.var_spsub_delta0_dn4 = assign10530_e9755_d_n4;
        locals.var_spsub_delta0_dn6 = assign10530_e9755_d_n6;
        locals.var_spsub_delta0_dn7 = assign10530_e9755_d_n7;
        locals.var_spsub_delta0_dn8 = assign10530_e9755_d_n8;
        locals.var_spsub_delta0_dn9 = assign10530_e9755_d_n9;

        let assign10540_e9759: f64 = (locals.var_xn_sub - 80.0);
        let assign10540_e9760: f64 = if locals.var_spsub_x0 > assign10540_e9759 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign10540_e9760;

        let (assign10550_e9778, assign10550_e9778_d_n4, assign10550_e9778_d_n6, assign10550_e9778_d_n7, assign10550_e9778_d_n8, assign10550_e9778_d_n9,) = {
    if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 != 0.0)) {
        let assign10550_e9775: f64 = (locals.var_spsub_x0 - locals.var_xn_sub);
        let assign10550_e9776: f64 = (assign10550_e9775).exp();
        (assign10550_e9776, (assign10550_e9776 * (locals.var_spsub_x0_dn4 - locals.var_xn_sub_dn4)), (assign10550_e9776 * (locals.var_spsub_x0_dn6 - locals.var_xn_sub_dn6)), (assign10550_e9776 * (locals.var_spsub_x0_dn7 - locals.var_xn_sub_dn7)), (assign10550_e9776 * (locals.var_spsub_x0_dn8 - locals.var_xn_sub_dn8)), (assign10550_e9776 * (locals.var_spsub_x0_dn9 - locals.var_xn_sub_dn9)),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10550_e9778;
        locals.var_spsub_delta0_dn4 = assign10550_e9778_d_n4;
        locals.var_spsub_delta0_dn6 = assign10550_e9778_d_n6;
        locals.var_spsub_delta0_dn7 = assign10550_e9778_d_n7;
        locals.var_spsub_delta0_dn8 = assign10550_e9778_d_n8;
        locals.var_spsub_delta0_dn9 = assign10550_e9778_d_n9;

        let (assign10560_e9795, assign10560_e9795_d_n4, assign10560_e9795_d_n6, assign10560_e9795_d_n7, assign10560_e9795_d_n8, assign10560_e9795_d_n9,) = {
    if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 != 0.0)) {
        let assign10560_e9793: f64 = (locals.var_spsub_delta / locals.var_spsub_delta0);
        (assign10560_e9793, (((locals.var_spsub_delta_dn4 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn4)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn6 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn6)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn7 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn7)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn8 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn8)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn9 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn9)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)),)
    } else {
        (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9,)
    }
};
        locals.var_spsub_delta1 = assign10560_e9795;
        locals.var_spsub_delta1_dn4 = assign10560_e9795_d_n4;
        locals.var_spsub_delta1_dn6 = assign10560_e9795_d_n6;
        locals.var_spsub_delta1_dn7 = assign10560_e9795_d_n7;
        locals.var_spsub_delta1_dn8 = assign10560_e9795_d_n8;
        locals.var_spsub_delta1_dn9 = assign10560_e9795_d_n9;

        let (assign10570_e9839, assign10570_e9839_d_n4, assign10570_e9839_d_n6, assign10570_e9839_d_n7, assign10570_e9839_d_n8, assign10570_e9839_d_n9,) = {
    if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 == 0.0)) {
        let assign10570_e9813: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
        let assign10570_e9815: f64 = (assign10570_e9813 - 80.0);
        let assign10570_e9820: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
        let assign10570_e9822: f64 = (assign10570_e9820 - 80.0);
        let assign10570_e9823: f64 = (0.5 * assign10570_e9822);
        let assign10570_e9827: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
        let assign10570_e9829: f64 = (assign10570_e9827 - 80.0);
        let assign10570_e9831: f64 = (assign10570_e9829 * 0.3333333333333);
        let assign10570_e9832: f64 = (1.0 + assign10570_e9831);
        let assign10570_e9833: f64 = (assign10570_e9823 * assign10570_e9832);
        let assign10570_e9834: f64 = (1.0 + assign10570_e9833);
        let assign10570_e9835: f64 = (assign10570_e9815 * assign10570_e9834);
        let assign10570_e9836: f64 = (1.0 + assign10570_e9835);
        let assign10570_e9837: f64 = (1.80485e-35 / assign10570_e9836);
        (assign10570_e9837, (-((1.80485e-35 * (((locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), (-((1.80485e-35 * (((locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), (-((1.80485e-35 * (((locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), (-((1.80485e-35 * (((locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), (-((1.80485e-35 * (((locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10570_e9839;
        locals.var_spsub_delta0_dn4 = assign10570_e9839_d_n4;
        locals.var_spsub_delta0_dn6 = assign10570_e9839_d_n6;
        locals.var_spsub_delta0_dn7 = assign10570_e9839_d_n7;
        locals.var_spsub_delta0_dn8 = assign10570_e9839_d_n8;
        locals.var_spsub_delta0_dn9 = assign10570_e9839_d_n9;

        let (assign10580_e9877, assign10580_e9877_d_n4, assign10580_e9877_d_n6, assign10580_e9877_d_n7, assign10580_e9877_d_n8, assign10580_e9877_d_n9,) = {
    if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 == 0.0)) {
        let assign10580_e9857: f64 = (locals.var_spsub_x0 - 80.0);
        let assign10580_e9862: f64 = (locals.var_spsub_x0 - 80.0);
        let assign10580_e9863: f64 = (0.5 * assign10580_e9862);
        let assign10580_e9867: f64 = (locals.var_spsub_x0 - 80.0);
        let assign10580_e9869: f64 = (assign10580_e9867 * 0.3333333333333);
        let assign10580_e9870: f64 = (1.0 + assign10580_e9869);
        let assign10580_e9871: f64 = (assign10580_e9863 * assign10580_e9870);
        let assign10580_e9872: f64 = (1.0 + assign10580_e9871);
        let assign10580_e9873: f64 = (assign10580_e9857 * assign10580_e9872);
        let assign10580_e9874: f64 = (1.0 + assign10580_e9873);
        let assign10580_e9875: f64 = (1.80485e-35 / assign10580_e9874);
        (assign10580_e9875, (-((1.80485e-35 * ((locals.var_spsub_x0_dn4 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn4) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn4 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn6 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn6) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn6 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn7 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn7) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn7 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn8 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn8) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn8 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn9 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn9) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn9 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))),)
    } else {
        (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9,)
    }
};
        locals.var_spsub_delta1 = assign10580_e9877;
        locals.var_spsub_delta1_dn4 = assign10580_e9877_d_n4;
        locals.var_spsub_delta1_dn6 = assign10580_e9877_d_n6;
        locals.var_spsub_delta1_dn7 = assign10580_e9877_d_n7;
        locals.var_spsub_delta1_dn8 = assign10580_e9877_d_n8;
        locals.var_spsub_delta1_dn9 = assign10580_e9877_d_n9;

        let (assign10590_e9893, assign10590_e9893_d_n4, assign10590_e9893_d_n6, assign10590_e9893_d_n7, assign10590_e9893_d_n8, assign10590_e9893_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10590_e9889: f64 = (locals.var_spsub_x0 * locals.var_spsub_x0);
        let assign10590_e9890: f64 = (2.0 + assign10590_e9889);
        let assign10590_e9891: f64 = (1.0 / assign10590_e9890);
        (assign10590_e9891, (-(((locals.var_spsub_x0_dn4 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn4)) / (assign10590_e9890 * assign10590_e9890))), (-(((locals.var_spsub_x0_dn6 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn6)) / (assign10590_e9890 * assign10590_e9890))), (-(((locals.var_spsub_x0_dn7 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn7)) / (assign10590_e9890 * assign10590_e9890))), (-(((locals.var_spsub_x0_dn8 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn8)) / (assign10590_e9890 * assign10590_e9890))), (-(((locals.var_spsub_x0_dn9 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn9)) / (assign10590_e9890 * assign10590_e9890))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10590_e9893;
        locals.var_spsub_temp_dn4 = assign10590_e9893_d_n4;
        locals.var_spsub_temp_dn6 = assign10590_e9893_d_n6;
        locals.var_spsub_temp_dn7 = assign10590_e9893_d_n7;
        locals.var_spsub_temp_dn8 = assign10590_e9893_d_n8;
        locals.var_spsub_temp_dn9 = assign10590_e9893_d_n9;

        let (assign10600_e9907, assign10600_e9907_d_n4, assign10600_e9907_d_n6, assign10600_e9907_d_n7, assign10600_e9907_d_n8, assign10600_e9907_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10600_e9903: f64 = (locals.var_spsub_x0 * locals.var_spsub_x0);
        let assign10600_e9905: f64 = (assign10600_e9903 * locals.var_spsub_temp);
        (assign10600_e9905, ((((locals.var_spsub_x0_dn4 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn4)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn4)), ((((locals.var_spsub_x0_dn6 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn6)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn6)), ((((locals.var_spsub_x0_dn7 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn7)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn7)), ((((locals.var_spsub_x0_dn8 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn8)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn8)), ((((locals.var_spsub_x0_dn9 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn9)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn9)),)
    } else {
        (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9,)
    }
};
        locals.var_spsub_xi0 = assign10600_e9907;
        locals.var_spsub_xi0_dn4 = assign10600_e9907_d_n4;
        locals.var_spsub_xi0_dn6 = assign10600_e9907_d_n6;
        locals.var_spsub_xi0_dn7 = assign10600_e9907_d_n7;
        locals.var_spsub_xi0_dn8 = assign10600_e9907_d_n8;
        locals.var_spsub_xi0_dn9 = assign10600_e9907_d_n9;

        let (assign10610_e9923, assign10610_e9923_d_n4, assign10610_e9923_d_n6, assign10610_e9923_d_n7, assign10610_e9923_d_n8, assign10610_e9923_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10610_e9918: f64 = (locals.var_spsub_x0 * locals.var_spsub_temp);
        let assign10610_e9920: f64 = (assign10610_e9918 * locals.var_spsub_temp);
        let assign10610_e9921: f64 = (4.0 * assign10610_e9920);
        (assign10610_e9921, (4.0 * ((((locals.var_spsub_x0_dn4 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn4))), (4.0 * ((((locals.var_spsub_x0_dn6 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn6))), (4.0 * ((((locals.var_spsub_x0_dn7 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn7))), (4.0 * ((((locals.var_spsub_x0_dn8 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn8))), (4.0 * ((((locals.var_spsub_x0_dn9 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn9))),)
    } else {
        (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9,)
    }
};
        locals.var_spsub_xi1 = assign10610_e9923;
        locals.var_spsub_xi1_dn4 = assign10610_e9923_d_n4;
        locals.var_spsub_xi1_dn6 = assign10610_e9923_d_n6;
        locals.var_spsub_xi1_dn7 = assign10610_e9923_d_n7;
        locals.var_spsub_xi1_dn8 = assign10610_e9923_d_n8;
        locals.var_spsub_xi1_dn9 = assign10610_e9923_d_n9;

        let (assign10620_e9943, assign10620_e9943_d_n4, assign10620_e9943_d_n6, assign10620_e9943_d_n7, assign10620_e9943_d_n8, assign10620_e9943_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10620_e9933: f64 = (8.0 * locals.var_spsub_temp);
        let assign10620_e9936: f64 = (12.0 * locals.var_spsub_xi0);
        let assign10620_e9937: f64 = (assign10620_e9933 - assign10620_e9936);
        let assign10620_e9939: f64 = (assign10620_e9937 * locals.var_spsub_temp);
        let assign10620_e9941: f64 = (assign10620_e9939 * locals.var_spsub_temp);
        (assign10620_e9941, ((((((8.0 * locals.var_spsub_temp_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn4)), ((((((8.0 * locals.var_spsub_temp_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn6)), ((((((8.0 * locals.var_spsub_temp_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn7)), ((((((8.0 * locals.var_spsub_temp_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn8)), ((((((8.0 * locals.var_spsub_temp_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn9)),)
    } else {
        (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9,)
    }
};
        locals.var_spsub_xi2 = assign10620_e9943;
        locals.var_spsub_xi2_dn4 = assign10620_e9943_d_n4;
        locals.var_spsub_xi2_dn6 = assign10620_e9943_d_n6;
        locals.var_spsub_xi2_dn7 = assign10620_e9943_d_n7;
        locals.var_spsub_xi2_dn8 = assign10620_e9943_d_n8;
        locals.var_spsub_xi2_dn9 = assign10620_e9943_d_n9;

        let (assign10630_e9955, assign10630_e9955_d_n4, assign10630_e9955_d_n6, assign10630_e9955_d_n7, assign10630_e9955_d_n8, assign10630_e9955_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10630_e9953: f64 = (locals.var_spsub_xgb - locals.var_spsub_x0);
        (assign10630_e9953, (locals.var_spsub_xgb_dn4 - locals.var_spsub_x0_dn4), (locals.var_spsub_xgb_dn6 - locals.var_spsub_x0_dn6), (locals.var_spsub_xgb_dn7 - locals.var_spsub_x0_dn7), (locals.var_spsub_xgb_dn8 - locals.var_spsub_x0_dn8), (locals.var_spsub_xgb_dn9 - locals.var_spsub_x0_dn9),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10630_e9955;
        locals.var_spsub_temp_dn4 = assign10630_e9955_d_n4;
        locals.var_spsub_temp_dn6 = assign10630_e9955_d_n6;
        locals.var_spsub_temp_dn7 = assign10630_e9955_d_n7;
        locals.var_spsub_temp_dn8 = assign10630_e9955_d_n8;
        locals.var_spsub_temp_dn9 = assign10630_e9955_d_n9;

        let (assign10640_e9981, assign10640_e9981_d_n4, assign10640_e9981_d_n6, assign10640_e9981_d_n7, assign10640_e9981_d_n8, assign10640_e9981_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10640_e9965: f64 = (2.0 * locals.var_spsub_temp);
        let assign10640_e9969: f64 = (1.0 - locals.var_spsub_delta1);
        let assign10640_e9971: f64 = (assign10640_e9969 + locals.var_spsub_delta0);
        let assign10640_e9975: f64 = (1.0 + locals.var_spsub_xi1);
        let assign10640_e9976: f64 = (locals.var_spsub_delta * assign10640_e9975);
        let assign10640_e9977: f64 = (assign10640_e9971 - assign10640_e9976);
        let assign10640_e9978: f64 = (locals.var_gfsub2 * assign10640_e9977);
        let assign10640_e9979: f64 = (assign10640_e9965 + assign10640_e9978);
        (assign10640_e9979, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn4) + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn4)))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn6) + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn6)))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn7) + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn7)))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn8) + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn8)))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn9) + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn9)))))),)
    } else {
        (locals.var_spsub_pc, locals.var_spsub_pc_dn4, locals.var_spsub_pc_dn6, locals.var_spsub_pc_dn7, locals.var_spsub_pc_dn8, locals.var_spsub_pc_dn9,)
    }
};
        locals.var_spsub_pc = assign10640_e9981;
        locals.var_spsub_pc_dn4 = assign10640_e9981_d_n4;
        locals.var_spsub_pc_dn6 = assign10640_e9981_d_n6;
        locals.var_spsub_pc_dn7 = assign10640_e9981_d_n7;
        locals.var_spsub_pc_dn8 = assign10640_e9981_d_n8;
        locals.var_spsub_pc_dn9 = assign10640_e9981_d_n9;

        let (assign10650_e10011, assign10650_e10011_d_n4, assign10650_e10011_d_n6, assign10650_e10011_d_n7, assign10650_e10011_d_n8, assign10650_e10011_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10650_e9991: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
        let assign10650_e9995: f64 = (locals.var_spsub_delta1 + locals.var_spsub_x0);
        let assign10650_e9997: f64 = (assign10650_e9995 - 1.0);
        let assign10650_e9999: f64 = (assign10650_e9997 + locals.var_spsub_delta0);
        let assign10650_e10003: f64 = (locals.var_spsub_x0 + 1.0);
        let assign10650_e10005: f64 = (assign10650_e10003 + locals.var_spsub_xi0);
        let assign10650_e10006: f64 = (locals.var_spsub_delta * assign10650_e10005);
        let assign10650_e10007: f64 = (assign10650_e9999 - assign10650_e10006);
        let assign10650_e10008: f64 = (locals.var_gfsub2 * assign10650_e10007);
        let assign10650_e10009: f64 = (assign10650_e9991 - assign10650_e10008);
        (assign10650_e10009, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn4 + locals.var_spsub_x0_dn4) + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn4 + locals.var_spsub_xi0_dn4))))))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn6 + locals.var_spsub_x0_dn6) + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn6 + locals.var_spsub_xi0_dn6))))))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn7 + locals.var_spsub_x0_dn7) + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn7 + locals.var_spsub_xi0_dn7))))))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn8 + locals.var_spsub_x0_dn8) + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn8 + locals.var_spsub_xi0_dn8))))))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn9 + locals.var_spsub_x0_dn9) + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn9 + locals.var_spsub_xi0_dn9))))))),)
    } else {
        (locals.var_spsub_qc, locals.var_spsub_qc_dn4, locals.var_spsub_qc_dn6, locals.var_spsub_qc_dn7, locals.var_spsub_qc_dn8, locals.var_spsub_qc_dn9,)
    }
};
        locals.var_spsub_qc = assign10650_e10011;
        locals.var_spsub_qc_dn4 = assign10650_e10011_d_n4;
        locals.var_spsub_qc_dn6 = assign10650_e10011_d_n6;
        locals.var_spsub_qc_dn7 = assign10650_e10011_d_n7;
        locals.var_spsub_qc_dn8 = assign10650_e10011_d_n8;
        locals.var_spsub_qc_dn9 = assign10650_e10011_d_n9;

    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10660_e10031, assign10660_e10031_d_n4, assign10660_e10031_d_n6, assign10660_e10031_d_n7, assign10660_e10031_d_n8, assign10660_e10031_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10660_e10023: f64 = (locals.var_spsub_delta1 + locals.var_spsub_delta0);
        let assign10660_e10026: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
        let assign10660_e10027: f64 = (assign10660_e10023 - assign10660_e10026);
        let assign10660_e10028: f64 = (locals.var_gfsub2 * assign10660_e10027);
        let assign10660_e10029: f64 = (2.0 - assign10660_e10028);
        (assign10660_e10029, (-((locals.var_gfsub2_dn4 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn4 + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4)))))), (-((locals.var_gfsub2_dn6 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn6 + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6)))))), (-((locals.var_gfsub2_dn7 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn7 + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7)))))), (-((locals.var_gfsub2_dn8 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn8 + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8)))))), (-((locals.var_gfsub2_dn9 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn9 + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9)))))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10660_e10031;
        locals.var_spsub_temp_dn4 = assign10660_e10031_d_n4;
        locals.var_spsub_temp_dn6 = assign10660_e10031_d_n6;
        locals.var_spsub_temp_dn7 = assign10660_e10031_d_n7;
        locals.var_spsub_temp_dn8 = assign10660_e10031_d_n8;
        locals.var_spsub_temp_dn9 = assign10660_e10031_d_n9;

        let (assign10670_e10049, assign10670_e10049_d_n4, assign10670_e10049_d_n6, assign10670_e10049_d_n7, assign10670_e10049_d_n8, assign10670_e10049_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10670_e10041: f64 = (locals.var_spsub_pc * locals.var_spsub_pc);
        let assign10670_e10045: f64 = (locals.var_spsub_qc * locals.var_spsub_temp);
        let assign10670_e10046: f64 = (2.0 * assign10670_e10045);
        let assign10670_e10047: f64 = (assign10670_e10041 - assign10670_e10046);
        (assign10670_e10047, (((locals.var_spsub_pc_dn4 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn4)) - (2.0 * ((locals.var_spsub_qc_dn4 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn4)))), (((locals.var_spsub_pc_dn6 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn6)) - (2.0 * ((locals.var_spsub_qc_dn6 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn6)))), (((locals.var_spsub_pc_dn7 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn7)) - (2.0 * ((locals.var_spsub_qc_dn7 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn7)))), (((locals.var_spsub_pc_dn8 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn8)) - (2.0 * ((locals.var_spsub_qc_dn8 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn8)))), (((locals.var_spsub_pc_dn9 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn9)) - (2.0 * ((locals.var_spsub_qc_dn9 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn9)))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10670_e10049;
        locals.var_spsub_temp_dn4 = assign10670_e10049_d_n4;
        locals.var_spsub_temp_dn6 = assign10670_e10049_d_n6;
        locals.var_spsub_temp_dn7 = assign10670_e10049_d_n7;
        locals.var_spsub_temp_dn8 = assign10670_e10049_d_n8;
        locals.var_spsub_temp_dn9 = assign10670_e10049_d_n9;

        let (assign10680_e10068, assign10680_e10068_d_n4, assign10680_e10068_d_n6, assign10680_e10068_d_n7, assign10680_e10068_d_n8, assign10680_e10068_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10680_e10062: f64 = (locals.var_spsub_temp).sqrt();
        let assign10680_e10063: f64 = (locals.var_spsub_pc + assign10680_e10062);
        let assign10680_e10064: f64 = (locals.var_spsub_qc / assign10680_e10063);
        let assign10680_e10065: f64 = (2.0 * assign10680_e10064);
        let assign10680_e10066: f64 = (locals.var_spsub_x0 + assign10680_e10065);
        (assign10680_e10066, (locals.var_spsub_x0_dn4 + (2.0 * (((locals.var_spsub_qc_dn4 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn4 + (locals.var_spsub_temp_dn4 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), (locals.var_spsub_x0_dn6 + (2.0 * (((locals.var_spsub_qc_dn6 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn6 + (locals.var_spsub_temp_dn6 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), (locals.var_spsub_x0_dn7 + (2.0 * (((locals.var_spsub_qc_dn7 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn7 + (locals.var_spsub_temp_dn7 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), (locals.var_spsub_x0_dn8 + (2.0 * (((locals.var_spsub_qc_dn8 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn8 + (locals.var_spsub_temp_dn8 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), (locals.var_spsub_x0_dn9 + (2.0 * (((locals.var_spsub_qc_dn9 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn9 + (locals.var_spsub_temp_dn9 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign10680_e10068;
        locals.var_temp3_dn4 = assign10680_e10068_d_n4;
        locals.var_temp3_dn6 = assign10680_e10068_d_n6;
        locals.var_temp3_dn7 = assign10680_e10068_d_n7;
        locals.var_temp3_dn8 = assign10680_e10068_d_n8;
        locals.var_temp3_dn9 = assign10680_e10068_d_n9;

        let (assign10690_e10076, assign10690_e10076_d_n4, assign10690_e10076_d_n6, assign10690_e10076_d_n7, assign10690_e10076_d_n8, assign10690_e10076_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign10690_e10073: f64 = (locals.var_temp3 + locals.var_temp2);
        let assign10690_e10074: f64 = (locals.var_temp * assign10690_e10073);
        (assign10690_e10074, ((locals.var_temp_dn4 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn4 + locals.var_temp2_dn4))), ((locals.var_temp_dn6 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn6 + locals.var_temp2_dn6))), ((locals.var_temp_dn7 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn7 + locals.var_temp2_dn7))), ((locals.var_temp_dn8 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn8 + locals.var_temp2_dn8))), ((locals.var_temp_dn9 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn9 + locals.var_temp2_dn9))),)
    } else {
        (locals.var_xg2eff, locals.var_xg2eff_dn4, locals.var_xg2eff_dn6, locals.var_xg2eff_dn7, locals.var_xg2eff_dn8, locals.var_xg2eff_dn9,)
    }
};
        locals.var_xg2eff = assign10690_e10076;
        locals.var_xg2eff_dn4 = assign10690_e10076_d_n4;
        locals.var_xg2eff_dn6 = assign10690_e10076_d_n6;
        locals.var_xg2eff_dn7 = assign10690_e10076_d_n7;
        locals.var_xg2eff_dn8 = assign10690_e10076_d_n8;
        locals.var_xg2eff_dn9 = assign10690_e10076_d_n9;

        let (assign10700_e10081, assign10700_e10081_d_n4, assign10700_e10081_d_n6, assign10700_e10081_d_n7, assign10700_e10081_d_n8, assign10700_e10081_d_n9,) = {
    if (locals.var_guard531 == 0.0) {
        (locals.var_xg20, locals.var_xg20_dn4, locals.var_xg20_dn6, locals.var_xg20_dn7, locals.var_xg20_dn8, locals.var_xg20_dn9,)
    } else {
        (locals.var_xg2eff, locals.var_xg2eff_dn4, locals.var_xg2eff_dn6, locals.var_xg2eff_dn7, locals.var_xg2eff_dn8, locals.var_xg2eff_dn9,)
    }
};
        locals.var_xg2eff = assign10700_e10081;
        locals.var_xg2eff_dn4 = assign10700_e10081_d_n4;
        locals.var_xg2eff_dn6 = assign10700_e10081_d_n6;
        locals.var_xg2eff_dn7 = assign10700_e10081_d_n7;
        locals.var_xg2eff_dn8 = assign10700_e10081_d_n8;
        locals.var_xg2eff_dn9 = assign10700_e10081_d_n9;

        let assign10710_e10085: f64 = (locals.var_xg10 - locals.var_xg2eff);
        let assign10710_e10086: f64 = (locals.var_keq_1d * assign10710_e10085);
        locals.var_temp = assign10710_e10086;
        locals.var_temp_dn4 = (locals.var_keq_1d * (locals.var_xg10_dn4 - locals.var_xg2eff_dn4));
        locals.var_temp_dn6 = (locals.var_keq_1d * (locals.var_xg10_dn6 - locals.var_xg2eff_dn6));
        locals.var_temp_dn7 = (locals.var_keq_1d * (locals.var_xg10_dn7 - locals.var_xg2eff_dn7));
        locals.var_temp_dn8 = (locals.var_keq_1d * (locals.var_xg10_dn8 - locals.var_xg2eff_dn8));
        locals.var_temp_dn9 = (locals.var_keq_1d * (locals.var_xg10_dn9 - locals.var_xg2eff_dn9));

        let assign10720_e10089: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign10720_e10089;

        let (assign10730_e10110, assign10730_e10110_d_n4, assign10730_e10110_d_n6, assign10730_e10110_d_n7, assign10730_e10110_d_n8, assign10730_e10110_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10730_e10094: f64 = (locals.var_temp + locals.var_emin);
        let assign10730_e10097: f64 = (locals.var_temp - locals.var_emin);
        let assign10730_e10100: f64 = (locals.var_temp - locals.var_emin);
        let assign10730_e10101: f64 = (assign10730_e10097 * assign10730_e10100);
        let assign10730_e10104: f64 = (locals.var_emin * locals.var_emin);
        let assign10730_e10105: f64 = (assign10730_e10101 + assign10730_e10104);
        let assign10730_e10106: f64 = (assign10730_e10105).sqrt();
        let assign10730_e10107: f64 = (assign10730_e10094 + assign10730_e10106);
        let assign10730_e10108: f64 = (0.5 * assign10730_e10107);
        (assign10730_e10108, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign10730_e10106)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign10730_e10106)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign10730_e10106)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign10730_e10106)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign10730_e10106)))),)
    } else {
        (locals.var_e1, locals.var_e1_dn4, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9,)
    }
};
        locals.var_e1 = assign10730_e10110;
        locals.var_e1_dn4 = assign10730_e10110_d_n4;
        locals.var_e1_dn6 = assign10730_e10110_d_n6;
        locals.var_e1_dn7 = assign10730_e10110_d_n7;
        locals.var_e1_dn8 = assign10730_e10110_d_n8;
        locals.var_e1_dn9 = assign10730_e10110_d_n9;

        let (assign10740_e10134, assign10740_e10134_d_n4, assign10740_e10134_d_n6, assign10740_e10134_d_n7, assign10740_e10134_d_n8, assign10740_e10134_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10740_e10114: f64 = (-locals.var_temp);
        let assign10740_e10116: f64 = (assign10740_e10114 + locals.var_emin);
        let assign10740_e10118: f64 = (-locals.var_temp);
        let assign10740_e10120: f64 = (assign10740_e10118 - locals.var_emin);
        let assign10740_e10122: f64 = (-locals.var_temp);
        let assign10740_e10124: f64 = (assign10740_e10122 - locals.var_emin);
        let assign10740_e10125: f64 = (assign10740_e10120 * assign10740_e10124);
        let assign10740_e10128: f64 = (locals.var_emin * locals.var_emin);
        let assign10740_e10129: f64 = (assign10740_e10125 + assign10740_e10128);
        let assign10740_e10130: f64 = (assign10740_e10129).sqrt();
        let assign10740_e10131: f64 = (assign10740_e10116 + assign10740_e10130);
        let assign10740_e10132: f64 = (0.5 * assign10740_e10131);
        (assign10740_e10132, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign10740_e10130)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign10740_e10130)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign10740_e10130)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign10740_e10130)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign10740_e10130)))),)
    } else {
        (locals.var_e2, locals.var_e2_dn4, locals.var_e2_dn6, locals.var_e2_dn7, locals.var_e2_dn8, locals.var_e2_dn9,)
    }
};
        locals.var_e2 = assign10740_e10134;
        locals.var_e2_dn4 = assign10740_e10134_d_n4;
        locals.var_e2_dn6 = assign10740_e10134_d_n6;
        locals.var_e2_dn7 = assign10740_e10134_d_n7;
        locals.var_e2_dn8 = assign10740_e10134_d_n8;
        locals.var_e2_dn9 = assign10740_e10134_d_n9;

        let (assign10750_e10145, assign10750_e10145_d_n4, assign10750_e10145_d_n6, assign10750_e10145_d_n7, assign10750_e10145_d_n8, assign10750_e10145_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10750_e10138: f64 = (-0.3333333333333);
        let assign10750_e10140: f64 = (locals.var_e1).ln();
        let assign10750_e10141: f64 = (assign10750_e10138 * assign10750_e10140);
        let assign10750_e10142: f64 = (assign10750_e10141).exp();
        let assign10750_e10143: f64 = (locals.var_qq * assign10750_e10142);
        (assign10750_e10143, ((locals.var_qq_dn4 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn4 / locals.var_e1))))), ((locals.var_qq_dn6 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn6 / locals.var_e1))))), ((locals.var_qq_dn7 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn7 / locals.var_e1))))), ((locals.var_qq_dn8 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn8 / locals.var_e1))))), ((locals.var_qq_dn9 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn9 / locals.var_e1))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign10750_e10145;
        locals.var_temp1_dn4 = assign10750_e10145_d_n4;
        locals.var_temp1_dn6 = assign10750_e10145_d_n6;
        locals.var_temp1_dn7 = assign10750_e10145_d_n7;
        locals.var_temp1_dn8 = assign10750_e10145_d_n8;
        locals.var_temp1_dn9 = assign10750_e10145_d_n9;

        let (assign10760_e10156, assign10760_e10156_d_n4, assign10760_e10156_d_n6, assign10760_e10156_d_n7, assign10760_e10156_d_n8, assign10760_e10156_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10760_e10149: f64 = (-0.3333333333333);
        let assign10760_e10151: f64 = (locals.var_e2).ln();
        let assign10760_e10152: f64 = (assign10760_e10149 * assign10760_e10151);
        let assign10760_e10153: f64 = (assign10760_e10152).exp();
        let assign10760_e10154: f64 = (locals.var_qq * assign10760_e10153);
        (assign10760_e10154, ((locals.var_qq_dn4 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn4 / locals.var_e2))))), ((locals.var_qq_dn6 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn6 / locals.var_e2))))), ((locals.var_qq_dn7 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn7 / locals.var_e2))))), ((locals.var_qq_dn8 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn8 / locals.var_e2))))), ((locals.var_qq_dn9 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn9 / locals.var_e2))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign10760_e10156;
        locals.var_temp2_dn4 = assign10760_e10156_d_n4;
        locals.var_temp2_dn6 = assign10760_e10156_d_n6;
        locals.var_temp2_dn7 = assign10760_e10156_d_n7;
        locals.var_temp2_dn8 = assign10760_e10156_d_n8;
        locals.var_temp2_dn9 = assign10760_e10156_d_n9;

        let (assign10770_e10164, assign10770_e10164_d_n4, assign10770_e10164_d_n6, assign10770_e10164_d_n7, assign10770_e10164_d_n8, assign10770_e10164_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10770_e10160: f64 = (1.0 - locals.var_temp1);
        let assign10770_e10162: f64 = (assign10770_e10160 - locals.var_temp2);
        (assign10770_e10162, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign10770_e10164;
        locals.var_temp3_dn4 = assign10770_e10164_d_n4;
        locals.var_temp3_dn6 = assign10770_e10164_d_n6;
        locals.var_temp3_dn7 = assign10770_e10164_d_n7;
        locals.var_temp3_dn8 = assign10770_e10164_d_n8;
        locals.var_temp3_dn9 = assign10770_e10164_d_n9;

        let (assign10780_e10170, assign10780_e10170_d_n4, assign10780_e10170_d_n6, assign10780_e10170_d_n7, assign10780_e10170_d_n8, assign10780_e10170_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10780_e10168: f64 = (locals.var_csiprime_0 / locals.var_temp3);
        (assign10780_e10168, (-((locals.var_csiprime_0 * locals.var_temp3_dn4) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn6) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn7) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn8) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn9) / (locals.var_temp3 * locals.var_temp3))),)
    } else {
        (locals.var_csiprime, locals.var_csiprime_dn4, locals.var_csiprime_dn6, locals.var_csiprime_dn7, locals.var_csiprime_dn8, locals.var_csiprime_dn9,)
    }
};
        locals.var_csiprime = assign10780_e10170;
        locals.var_csiprime_dn4 = assign10780_e10170_d_n4;
        locals.var_csiprime_dn6 = assign10780_e10170_d_n6;
        locals.var_csiprime_dn7 = assign10780_e10170_d_n7;
        locals.var_csiprime_dn8 = assign10780_e10170_d_n8;
        locals.var_csiprime_dn9 = assign10780_e10170_d_n9;

        let (assign10790_e10178, assign10790_e10178_d_n4, assign10790_e10178_d_n6, assign10790_e10178_d_n7, assign10790_e10178_d_n8, assign10790_e10178_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10790_e10175: f64 = (locals.var_k1_1d * locals.var_temp1);
        let assign10790_e10176: f64 = (1.0 + assign10790_e10175);
        (assign10790_e10176, (locals.var_k1_1d * locals.var_temp1_dn4), (locals.var_k1_1d * locals.var_temp1_dn6), (locals.var_k1_1d * locals.var_temp1_dn7), (locals.var_k1_1d * locals.var_temp1_dn8), (locals.var_k1_1d * locals.var_temp1_dn9),)
    } else {
        (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9,)
    }
};
        locals.var_tox1fact = assign10790_e10178;
        locals.var_tox1fact_dn4 = assign10790_e10178_d_n4;
        locals.var_tox1fact_dn6 = assign10790_e10178_d_n6;
        locals.var_tox1fact_dn7 = assign10790_e10178_d_n7;
        locals.var_tox1fact_dn8 = assign10790_e10178_d_n8;
        locals.var_tox1fact_dn9 = assign10790_e10178_d_n9;

        let (assign10800_e10186, assign10800_e10186_d_n4, assign10800_e10186_d_n6, assign10800_e10186_d_n7, assign10800_e10186_d_n8, assign10800_e10186_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10800_e10183: f64 = (locals.var_k2_1d * locals.var_temp2);
        let assign10800_e10184: f64 = (1.0 + assign10800_e10183);
        (assign10800_e10184, (locals.var_k2_1d * locals.var_temp2_dn4), (locals.var_k2_1d * locals.var_temp2_dn6), (locals.var_k2_1d * locals.var_temp2_dn7), (locals.var_k2_1d * locals.var_temp2_dn8), (locals.var_k2_1d * locals.var_temp2_dn9),)
    } else {
        (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9,)
    }
};
        locals.var_tox2fact = assign10800_e10186;
        locals.var_tox2fact_dn4 = assign10800_e10186_d_n4;
        locals.var_tox2fact_dn6 = assign10800_e10186_d_n6;
        locals.var_tox2fact_dn7 = assign10800_e10186_d_n7;
        locals.var_tox2fact_dn8 = assign10800_e10186_d_n8;
        locals.var_tox2fact_dn9 = assign10800_e10186_d_n9;

        let (assign10810_e10194, assign10810_e10194_d_n4, assign10810_e10194_d_n6, assign10810_e10194_d_n7, assign10810_e10194_d_n8, assign10810_e10194_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10810_e10190: f64 = (locals.var_k1_1d * locals.var_temp3);
        let assign10810_e10192: f64 = (assign10810_e10190 / locals.var_tox1fact);
        (assign10810_e10192, ((((locals.var_k1_1d * locals.var_temp3_dn4) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn4)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn6)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn7)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn8)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn9)) / (locals.var_tox1fact * locals.var_tox1fact)),)
    } else {
        (locals.var_k1_1d_qm, locals.var_k1_1d_qm_dn4, locals.var_k1_1d_qm_dn6, locals.var_k1_1d_qm_dn7, locals.var_k1_1d_qm_dn8, locals.var_k1_1d_qm_dn9,)
    }
};
        locals.var_k1_1d_qm = assign10810_e10194;
        locals.var_k1_1d_qm_dn4 = assign10810_e10194_d_n4;
        locals.var_k1_1d_qm_dn6 = assign10810_e10194_d_n6;
        locals.var_k1_1d_qm_dn7 = assign10810_e10194_d_n7;
        locals.var_k1_1d_qm_dn8 = assign10810_e10194_d_n8;
        locals.var_k1_1d_qm_dn9 = assign10810_e10194_d_n9;

        let (assign10820_e10202, assign10820_e10202_d_n4, assign10820_e10202_d_n6, assign10820_e10202_d_n7, assign10820_e10202_d_n8, assign10820_e10202_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10820_e10198: f64 = (locals.var_k2_1d * locals.var_temp3);
        let assign10820_e10200: f64 = (assign10820_e10198 / locals.var_tox2fact);
        (assign10820_e10200, ((((locals.var_k2_1d * locals.var_temp3_dn4) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn4)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn6)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn7)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn8)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn9)) / (locals.var_tox2fact * locals.var_tox2fact)),)
    } else {
        (locals.var_k2_1d_qm, locals.var_k2_1d_qm_dn4, locals.var_k2_1d_qm_dn6, locals.var_k2_1d_qm_dn7, locals.var_k2_1d_qm_dn8, locals.var_k2_1d_qm_dn9,)
    }
};
        locals.var_k2_1d_qm = assign10820_e10202;
        locals.var_k2_1d_qm_dn4 = assign10820_e10202_d_n4;
        locals.var_k2_1d_qm_dn6 = assign10820_e10202_d_n6;
        locals.var_k2_1d_qm_dn7 = assign10820_e10202_d_n7;
        locals.var_k2_1d_qm_dn8 = assign10820_e10202_d_n8;
        locals.var_k2_1d_qm_dn9 = assign10820_e10202_d_n9;

        let (assign10830_e10216, assign10830_e10216_d_n4, assign10830_e10216_d_n6, assign10830_e10216_d_n7, assign10830_e10216_d_n8, assign10830_e10216_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10830_e10208: f64 = (1.0 / locals.var_k1_1d_qm);
        let assign10830_e10209: f64 = (1.0 + assign10830_e10208);
        let assign10830_e10212: f64 = (1.0 / locals.var_k2_1d_qm);
        let assign10830_e10213: f64 = (assign10830_e10209 + assign10830_e10212);
        let assign10830_e10214: f64 = (1.0 / assign10830_e10213);
        (assign10830_e10214, (-(((-(locals.var_k1_1d_qm_dn4 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn4 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), (-(((-(locals.var_k1_1d_qm_dn6 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn6 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), (-(((-(locals.var_k1_1d_qm_dn7 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn7 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), (-(((-(locals.var_k1_1d_qm_dn8 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn8 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), (-(((-(locals.var_k1_1d_qm_dn9 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn9 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))),)
    } else {
        (locals.var_keq_1d_qm, locals.var_keq_1d_qm_dn4, locals.var_keq_1d_qm_dn6, locals.var_keq_1d_qm_dn7, locals.var_keq_1d_qm_dn8, locals.var_keq_1d_qm_dn9,)
    }
};
        locals.var_keq_1d_qm = assign10830_e10216;
        locals.var_keq_1d_qm_dn4 = assign10830_e10216_d_n4;
        locals.var_keq_1d_qm_dn6 = assign10830_e10216_d_n6;
        locals.var_keq_1d_qm_dn7 = assign10830_e10216_d_n7;
        locals.var_keq_1d_qm_dn8 = assign10830_e10216_d_n8;
        locals.var_keq_1d_qm_dn9 = assign10830_e10216_d_n9;

        let (assign10840_e10224, assign10840_e10224_d_n4, assign10840_e10224_d_n6, assign10840_e10224_d_n7, assign10840_e10224_d_n8, assign10840_e10224_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10840_e10221: f64 = (locals.var_k1_1d_qm * locals.var_temp1);
        let assign10840_e10222: f64 = (1.0 + assign10840_e10221);
        (assign10840_e10222, ((locals.var_k1_1d_qm_dn4 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn4)), ((locals.var_k1_1d_qm_dn6 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn6)), ((locals.var_k1_1d_qm_dn7 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn7)), ((locals.var_k1_1d_qm_dn8 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn8)), ((locals.var_k1_1d_qm_dn9 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn9)),)
    } else {
        (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9,)
    }
};
        locals.var_tox1fact = assign10840_e10224;
        locals.var_tox1fact_dn4 = assign10840_e10224_d_n4;
        locals.var_tox1fact_dn6 = assign10840_e10224_d_n6;
        locals.var_tox1fact_dn7 = assign10840_e10224_d_n7;
        locals.var_tox1fact_dn8 = assign10840_e10224_d_n8;
        locals.var_tox1fact_dn9 = assign10840_e10224_d_n9;

        let (assign10850_e10232, assign10850_e10232_d_n4, assign10850_e10232_d_n6, assign10850_e10232_d_n7, assign10850_e10232_d_n8, assign10850_e10232_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10850_e10229: f64 = (locals.var_k2_1d_qm * locals.var_temp2);
        let assign10850_e10230: f64 = (1.0 + assign10850_e10229);
        (assign10850_e10230, ((locals.var_k2_1d_qm_dn4 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn4)), ((locals.var_k2_1d_qm_dn6 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn6)), ((locals.var_k2_1d_qm_dn7 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn7)), ((locals.var_k2_1d_qm_dn8 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn8)), ((locals.var_k2_1d_qm_dn9 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn9)),)
    } else {
        (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9,)
    }
};
        locals.var_tox2fact = assign10850_e10232;
        locals.var_tox2fact_dn4 = assign10850_e10232_d_n4;
        locals.var_tox2fact_dn6 = assign10850_e10232_d_n6;
        locals.var_tox2fact_dn7 = assign10850_e10232_d_n7;
        locals.var_tox2fact_dn8 = assign10850_e10232_d_n8;
        locals.var_tox2fact_dn9 = assign10850_e10232_d_n9;

        let (assign10860_e10237, assign10860_e10237_d_n4, assign10860_e10237_d_n6, assign10860_e10237_d_n7, assign10860_e10237_d_n8, assign10860_e10237_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (locals.var_csiprime_0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_csiprime, locals.var_csiprime_dn4, locals.var_csiprime_dn6, locals.var_csiprime_dn7, locals.var_csiprime_dn8, locals.var_csiprime_dn9,)
    }
};
        locals.var_csiprime = assign10860_e10237;
        locals.var_csiprime_dn4 = assign10860_e10237_d_n4;
        locals.var_csiprime_dn6 = assign10860_e10237_d_n6;
        locals.var_csiprime_dn7 = assign10860_e10237_d_n7;
        locals.var_csiprime_dn8 = assign10860_e10237_d_n8;
        locals.var_csiprime_dn9 = assign10860_e10237_d_n9;

        let (assign10870_e10242, assign10870_e10242_d_n4, assign10870_e10242_d_n6, assign10870_e10242_d_n7, assign10870_e10242_d_n8, assign10870_e10242_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k1_1d_qm, locals.var_k1_1d_qm_dn4, locals.var_k1_1d_qm_dn6, locals.var_k1_1d_qm_dn7, locals.var_k1_1d_qm_dn8, locals.var_k1_1d_qm_dn9,)
    }
};
        locals.var_k1_1d_qm = assign10870_e10242;
        locals.var_k1_1d_qm_dn4 = assign10870_e10242_d_n4;
        locals.var_k1_1d_qm_dn6 = assign10870_e10242_d_n6;
        locals.var_k1_1d_qm_dn7 = assign10870_e10242_d_n7;
        locals.var_k1_1d_qm_dn8 = assign10870_e10242_d_n8;
        locals.var_k1_1d_qm_dn9 = assign10870_e10242_d_n9;

        let (assign10880_e10247, assign10880_e10247_d_n4, assign10880_e10247_d_n6, assign10880_e10247_d_n7, assign10880_e10247_d_n8, assign10880_e10247_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k2_1d_qm, locals.var_k2_1d_qm_dn4, locals.var_k2_1d_qm_dn6, locals.var_k2_1d_qm_dn7, locals.var_k2_1d_qm_dn8, locals.var_k2_1d_qm_dn9,)
    }
};
        locals.var_k2_1d_qm = assign10880_e10247;
        locals.var_k2_1d_qm_dn4 = assign10880_e10247_d_n4;
        locals.var_k2_1d_qm_dn6 = assign10880_e10247_d_n6;
        locals.var_k2_1d_qm_dn7 = assign10880_e10247_d_n7;
        locals.var_k2_1d_qm_dn8 = assign10880_e10247_d_n8;
        locals.var_k2_1d_qm_dn9 = assign10880_e10247_d_n9;

        let (assign10890_e10252, assign10890_e10252_d_n4, assign10890_e10252_d_n6, assign10890_e10252_d_n7, assign10890_e10252_d_n8, assign10890_e10252_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_keq_1d_qm, locals.var_keq_1d_qm_dn4, locals.var_keq_1d_qm_dn6, locals.var_keq_1d_qm_dn7, locals.var_keq_1d_qm_dn8, locals.var_keq_1d_qm_dn9,)
    }
};
        locals.var_keq_1d_qm = assign10890_e10252;
        locals.var_keq_1d_qm_dn4 = assign10890_e10252_d_n4;
        locals.var_keq_1d_qm_dn6 = assign10890_e10252_d_n6;
        locals.var_keq_1d_qm_dn7 = assign10890_e10252_d_n7;
        locals.var_keq_1d_qm_dn8 = assign10890_e10252_d_n8;
        locals.var_keq_1d_qm_dn9 = assign10890_e10252_d_n9;

        let (assign10900_e10257, assign10900_e10257_d_n4, assign10900_e10257_d_n6, assign10900_e10257_d_n7, assign10900_e10257_d_n8, assign10900_e10257_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9,)
    }
};
        locals.var_tox1fact = assign10900_e10257;
        locals.var_tox1fact_dn4 = assign10900_e10257_d_n4;
        locals.var_tox1fact_dn6 = assign10900_e10257_d_n6;
        locals.var_tox1fact_dn7 = assign10900_e10257_d_n7;
        locals.var_tox1fact_dn8 = assign10900_e10257_d_n8;
        locals.var_tox1fact_dn9 = assign10900_e10257_d_n9;

        let (assign10910_e10262, assign10910_e10262_d_n4, assign10910_e10262_d_n6, assign10910_e10262_d_n7, assign10910_e10262_d_n8, assign10910_e10262_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9,)
    }
};
        locals.var_tox2fact = assign10910_e10262;
        locals.var_tox2fact_dn4 = assign10910_e10262_d_n4;
        locals.var_tox2fact_dn6 = assign10910_e10262_d_n6;
        locals.var_tox2fact_dn7 = assign10910_e10262_d_n7;
        locals.var_tox2fact_dn8 = assign10910_e10262_d_n8;
        locals.var_tox2fact_dn9 = assign10910_e10262_d_n9;

        let assign10920_e10266: f64 = (locals.var_xg10 - locals.var_xg2eff);
        let assign10920_e10267: f64 = (locals.var_keq_1d_qm * assign10920_e10266);
        locals.var_dx_wi_1d = assign10920_e10267;
        locals.var_dx_wi_1d_dn4 = ((locals.var_keq_1d_qm_dn4 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn4 - locals.var_xg2eff_dn4)));
        locals.var_dx_wi_1d_dn6 = ((locals.var_keq_1d_qm_dn6 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn6 - locals.var_xg2eff_dn6)));
        locals.var_dx_wi_1d_dn7 = ((locals.var_keq_1d_qm_dn7 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn7 - locals.var_xg2eff_dn7)));
        locals.var_dx_wi_1d_dn8 = ((locals.var_keq_1d_qm_dn8 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn8 - locals.var_xg2eff_dn8)));
        locals.var_dx_wi_1d_dn9 = ((locals.var_keq_1d_qm_dn9 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn9 - locals.var_xg2eff_dn9)));

        let assign10930_e10270: f64 = if locals.var_dx_wi_1d > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign10930_e10270;

        let assign10940_e10272: f64 = (-locals.var_dx_wi_1d);
        let assign10940_e10274: f64 = if assign10940_e10272 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign10940_e10274;

        let (assign10950_e10285, assign10950_e10285_d_n4, assign10950_e10285_d_n6, assign10950_e10285_d_n7, assign10950_e10285_d_n8, assign10950_e10285_d_n9,) = {
    if ((locals.var_guard544 != 0.0) && (locals.var_guard545 != 0.0)) {
        let assign10950_e10280: f64 = (-locals.var_dx_wi_1d);
        let assign10950_e10281: f64 = (assign10950_e10280).exp();
        let assign10950_e10282: f64 = (1.0 + assign10950_e10281);
        let assign10950_e10283: f64 = (assign10950_e10282).ln();
        (assign10950_e10283, ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn4)) / assign10950_e10282), ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn6)) / assign10950_e10282), ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn7)) / assign10950_e10282), ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn8)) / assign10950_e10282), ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn9)) / assign10950_e10282),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign10950_e10285;
        locals.var_temp_dn4 = assign10950_e10285_d_n4;
        locals.var_temp_dn6 = assign10950_e10285_d_n6;
        locals.var_temp_dn7 = assign10950_e10285_d_n7;
        locals.var_temp_dn8 = assign10950_e10285_d_n8;
        locals.var_temp_dn9 = assign10950_e10285_d_n9;

        let (assign10960_e10293, assign10960_e10293_d_n4, assign10960_e10293_d_n6, assign10960_e10293_d_n7, assign10960_e10293_d_n8, assign10960_e10293_d_n9,) = {
    if ((locals.var_guard544 != 0.0) && (locals.var_guard545 == 0.0)) {
        let assign10960_e10291: f64 = (-locals.var_dx_wi_1d);
        (assign10960_e10291, (-locals.var_dx_wi_1d_dn4), (-locals.var_dx_wi_1d_dn6), (-locals.var_dx_wi_1d_dn7), (-locals.var_dx_wi_1d_dn8), (-locals.var_dx_wi_1d_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign10960_e10293;
        locals.var_temp_dn4 = assign10960_e10293_d_n4;
        locals.var_temp_dn6 = assign10960_e10293_d_n6;
        locals.var_temp_dn7 = assign10960_e10293_d_n7;
        locals.var_temp_dn8 = assign10960_e10293_d_n8;
        locals.var_temp_dn9 = assign10960_e10293_d_n9;

        let (assign10970_e10305, assign10970_e10305_d_n4, assign10970_e10305_d_n6, assign10970_e10305_d_n7, assign10970_e10305_d_n8, assign10970_e10305_d_n9,) = {
    if (locals.var_guard544 != 0.0) {
        let assign10970_e10298: f64 = (locals.var_dx_wi_1d / locals.var_k1_1d_qm);
        let assign10970_e10299: f64 = (locals.var_xg10 - assign10970_e10298);
        let assign10970_e10301: f64 = (assign10970_e10299 + locals.var_temp);
        let assign10970_e10303: f64 = (assign10970_e10301 - 0.6931471805599);
        (assign10970_e10303, ((locals.var_xg10_dn4 - (((locals.var_dx_wi_1d_dn4 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn4)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn4), ((locals.var_xg10_dn6 - (((locals.var_dx_wi_1d_dn6 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn6)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn6), ((locals.var_xg10_dn7 - (((locals.var_dx_wi_1d_dn7 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn7)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn7), ((locals.var_xg10_dn8 - (((locals.var_dx_wi_1d_dn8 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn8)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn8), ((locals.var_xg10_dn9 - (((locals.var_dx_wi_1d_dn9 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn9)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d, locals.var_x_wi_1d_dn4, locals.var_x_wi_1d_dn6, locals.var_x_wi_1d_dn7, locals.var_x_wi_1d_dn8, locals.var_x_wi_1d_dn9,)
    }
};
        locals.var_x_wi_1d = assign10970_e10305;
        locals.var_x_wi_1d_dn4 = assign10970_e10305_d_n4;
        locals.var_x_wi_1d_dn6 = assign10970_e10305_d_n6;
        locals.var_x_wi_1d_dn7 = assign10970_e10305_d_n7;
        locals.var_x_wi_1d_dn8 = assign10970_e10305_d_n8;
        locals.var_x_wi_1d_dn9 = assign10970_e10305_d_n9;

        let assign10980_e10308: f64 = if locals.var_dx_wi_1d < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign10980_e10308;

        let (assign10990_e10319, assign10990_e10319_d_n4, assign10990_e10319_d_n6, assign10990_e10319_d_n7, assign10990_e10319_d_n8, assign10990_e10319_d_n9,) = {
    if ((locals.var_guard544 == 0.0) && (locals.var_guard546 != 0.0)) {
        let assign10990_e10315: f64 = (locals.var_dx_wi_1d).exp();
        let assign10990_e10316: f64 = (1.0 + assign10990_e10315);
        let assign10990_e10317: f64 = (assign10990_e10316).ln();
        (assign10990_e10317, ((assign10990_e10315 * locals.var_dx_wi_1d_dn4) / assign10990_e10316), ((assign10990_e10315 * locals.var_dx_wi_1d_dn6) / assign10990_e10316), ((assign10990_e10315 * locals.var_dx_wi_1d_dn7) / assign10990_e10316), ((assign10990_e10315 * locals.var_dx_wi_1d_dn8) / assign10990_e10316), ((assign10990_e10315 * locals.var_dx_wi_1d_dn9) / assign10990_e10316),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign10990_e10319;
        locals.var_temp_dn4 = assign10990_e10319_d_n4;
        locals.var_temp_dn6 = assign10990_e10319_d_n6;
        locals.var_temp_dn7 = assign10990_e10319_d_n7;
        locals.var_temp_dn8 = assign10990_e10319_d_n8;
        locals.var_temp_dn9 = assign10990_e10319_d_n9;

    }

    pub(super) fn stamp_transient_block_25(
        locals: &mut StampLocals,
    ) {
        let (assign11000_e10327, assign11000_e10327_d_n4, assign11000_e10327_d_n6, assign11000_e10327_d_n7, assign11000_e10327_d_n8, assign11000_e10327_d_n9,) = {
    if ((locals.var_guard544 == 0.0) && (locals.var_guard546 == 0.0)) {
        (locals.var_dx_wi_1d, locals.var_dx_wi_1d_dn4, locals.var_dx_wi_1d_dn6, locals.var_dx_wi_1d_dn7, locals.var_dx_wi_1d_dn8, locals.var_dx_wi_1d_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign11000_e10327;
        locals.var_temp_dn4 = assign11000_e10327_d_n4;
        locals.var_temp_dn6 = assign11000_e10327_d_n6;
        locals.var_temp_dn7 = assign11000_e10327_d_n7;
        locals.var_temp_dn8 = assign11000_e10327_d_n8;
        locals.var_temp_dn9 = assign11000_e10327_d_n9;

        let (assign11010_e10340, assign11010_e10340_d_n4, assign11010_e10340_d_n6, assign11010_e10340_d_n7, assign11010_e10340_d_n8, assign11010_e10340_d_n9,) = {
    if (locals.var_guard544 == 0.0) {
        let assign11010_e10333: f64 = (locals.var_dx_wi_1d / locals.var_k2_1d_qm);
        let assign11010_e10334: f64 = (locals.var_xg2eff + assign11010_e10333);
        let assign11010_e10336: f64 = (assign11010_e10334 + locals.var_temp);
        let assign11010_e10338: f64 = (assign11010_e10336 - 0.6931471805599);
        (assign11010_e10338, ((locals.var_xg2eff_dn4 + (((locals.var_dx_wi_1d_dn4 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn4)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn4), ((locals.var_xg2eff_dn6 + (((locals.var_dx_wi_1d_dn6 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn6)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn6), ((locals.var_xg2eff_dn7 + (((locals.var_dx_wi_1d_dn7 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn7)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn7), ((locals.var_xg2eff_dn8 + (((locals.var_dx_wi_1d_dn8 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn8)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn8), ((locals.var_xg2eff_dn9 + (((locals.var_dx_wi_1d_dn9 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn9)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d, locals.var_x_wi_1d_dn4, locals.var_x_wi_1d_dn6, locals.var_x_wi_1d_dn7, locals.var_x_wi_1d_dn8, locals.var_x_wi_1d_dn9,)
    }
};
        locals.var_x_wi_1d = assign11010_e10340;
        locals.var_x_wi_1d_dn4 = assign11010_e10340_d_n4;
        locals.var_x_wi_1d_dn6 = assign11010_e10340_d_n6;
        locals.var_x_wi_1d_dn7 = assign11010_e10340_d_n7;
        locals.var_x_wi_1d_dn8 = assign11010_e10340_d_n8;
        locals.var_x_wi_1d_dn9 = assign11010_e10340_d_n9;

        let assign11020_e10344: f64 = (locals.var_x_wi_1d + locals.var_xth_1d);
        let assign11020_e10347: f64 = (locals.var_x_wi_1d - locals.var_xth_1d);
        let assign11020_e10350: f64 = (locals.var_x_wi_1d - locals.var_xth_1d);
        let assign11020_e10351: f64 = (assign11020_e10347 * assign11020_e10350);
        let assign11020_e10353: f64 = (assign11020_e10351 + 4.0);
        let assign11020_e10354: f64 = (assign11020_e10353).sqrt();
        let assign11020_e10355: f64 = (assign11020_e10344 - assign11020_e10354);
        let assign11020_e10356: f64 = (0.5 * assign11020_e10355);
        locals.var_x_1d = assign11020_e10356;
        locals.var_x_1d_dn4 = (0.5 * ((locals.var_x_wi_1d_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_x_wi_1d_dn4 - locals.var_xth_1d_dn4) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign11020_e10354))));
        locals.var_x_1d_dn6 = (0.5 * ((locals.var_x_wi_1d_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_x_wi_1d_dn6 - locals.var_xth_1d_dn6) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign11020_e10354))));
        locals.var_x_1d_dn7 = (0.5 * ((locals.var_x_wi_1d_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_x_wi_1d_dn7 - locals.var_xth_1d_dn7) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign11020_e10354))));
        locals.var_x_1d_dn8 = (0.5 * ((locals.var_x_wi_1d_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_x_wi_1d_dn8 - locals.var_xth_1d_dn8) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign11020_e10354))));
        locals.var_x_1d_dn9 = (0.5 * ((locals.var_x_wi_1d_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_x_wi_1d_dn9 - locals.var_xth_1d_dn9) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign11020_e10354))));

        let assign11030_e10361: f64 = (locals.var_xth_1d - locals.var_x_1d);
        let assign11030_e10362: f64 = (2.0 * assign11030_e10361);
        let assign11030_e10364: f64 = (assign11030_e10362 / locals.var_xsddep);
        let assign11030_e10365: f64 = (1.0 + assign11030_e10364);
        let assign11030_e10366: f64 = (assign11030_e10365).sqrt();
        let assign11030_e10368: f64 = (assign11030_e10366 - 1.0);
        locals.var_dleff = assign11030_e10368;
        locals.var_dleff_dn4 = (((((2.0 * (locals.var_xth_1d_dn4 - locals.var_x_1d_dn4)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn4)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366));
        locals.var_dleff_dn6 = (((((2.0 * (locals.var_xth_1d_dn6 - locals.var_x_1d_dn6)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn6)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366));
        locals.var_dleff_dn7 = (((((2.0 * (locals.var_xth_1d_dn7 - locals.var_x_1d_dn7)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn7)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366));
        locals.var_dleff_dn8 = (((((2.0 * (locals.var_xth_1d_dn8 - locals.var_x_1d_dn8)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn8)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366));
        locals.var_dleff_dn9 = (((((2.0 * (locals.var_xth_1d_dn9 - locals.var_x_1d_dn9)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn9)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366));

        let assign11040_e10372: f64 = (locals.var_xsddep * locals.var_dleff);
        let assign11040_e10373: f64 = (locals.var_x_1d + assign11040_e10372);
        locals.var_xedge = assign11040_e10373;
        locals.var_xedge_dn4 = (locals.var_x_1d_dn4 + ((locals.var_xsddep_dn4 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn4)));
        locals.var_xedge_dn6 = (locals.var_x_1d_dn6 + ((locals.var_xsddep_dn6 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn6)));
        locals.var_xedge_dn7 = (locals.var_x_1d_dn7 + ((locals.var_xsddep_dn7 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn7)));
        locals.var_xedge_dn8 = (locals.var_x_1d_dn8 + ((locals.var_xsddep_dn8 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn8)));
        locals.var_xedge_dn9 = (locals.var_x_1d_dn9 + ((locals.var_xsddep_dn9 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn9)));

        let assign11050_e10378: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11050_e10379: f64 = (1.0 + assign11050_e10378);
        let assign11050_e10381: f64 = (assign11050_e10379 + 0.5);
        let assign11050_e10385: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11050_e10386: f64 = (1.0 + assign11050_e10385);
        let assign11050_e10388: f64 = (assign11050_e10386 - 0.5);
        let assign11050_e10392: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11050_e10393: f64 = (1.0 + assign11050_e10392);
        let assign11050_e10395: f64 = (assign11050_e10393 - 0.5);
        let assign11050_e10396: f64 = (assign11050_e10388 * assign11050_e10395);
        let assign11050_e10398: f64 = (assign11050_e10396 + 0.01);
        let assign11050_e10399: f64 = (assign11050_e10398).sqrt();
        let assign11050_e10400: f64 = (assign11050_e10381 + assign11050_e10399);
        let assign11050_e10401: f64 = (0.5 * assign11050_e10400);
        locals.var_temp = assign11050_e10401;
        locals.var_temp_dn4 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn4) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn4))) / (2.0 * assign11050_e10399))));
        locals.var_temp_dn6 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn6) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn6))) / (2.0 * assign11050_e10399))));
        locals.var_temp_dn7 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn7) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn7))) / (2.0 * assign11050_e10399))));
        locals.var_temp_dn8 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn8) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn8))) / (2.0 * assign11050_e10399))));
        locals.var_temp_dn9 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn9) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn9))) / (2.0 * assign11050_e10399))));

        let assign11060_e10406: f64 = (locals.var_psce1_loc * locals.var_temp);
        let assign11060_e10407: f64 = (1.0 + assign11060_e10406);
        let assign11060_e10408: f64 = (1.0 / assign11060_e10407);
        locals.var_sce1 = assign11060_e10408;
        locals.var_sce1_dn4 = (-((locals.var_psce1_loc * locals.var_temp_dn4) / (assign11060_e10407 * assign11060_e10407)));
        locals.var_sce1_dn6 = (-((locals.var_psce1_loc * locals.var_temp_dn6) / (assign11060_e10407 * assign11060_e10407)));
        locals.var_sce1_dn7 = (-((locals.var_psce1_loc * locals.var_temp_dn7) / (assign11060_e10407 * assign11060_e10407)));
        locals.var_sce1_dn8 = (-((locals.var_psce1_loc * locals.var_temp_dn8) / (assign11060_e10407 * assign11060_e10407)));
        locals.var_sce1_dn9 = (-((locals.var_psce1_loc * locals.var_temp_dn9) / (assign11060_e10407 * assign11060_e10407)));

        let assign11070_e10413: f64 = (locals.var_psce2_loc * locals.var_temp);
        let assign11070_e10414: f64 = (1.0 + assign11070_e10413);
        let assign11070_e10415: f64 = (1.0 / assign11070_e10414);
        locals.var_sce2 = assign11070_e10415;
        locals.var_sce2_dn4 = (-((locals.var_psce2_loc * locals.var_temp_dn4) / (assign11070_e10414 * assign11070_e10414)));
        locals.var_sce2_dn6 = (-((locals.var_psce2_loc * locals.var_temp_dn6) / (assign11070_e10414 * assign11070_e10414)));
        locals.var_sce2_dn7 = (-((locals.var_psce2_loc * locals.var_temp_dn7) / (assign11070_e10414 * assign11070_e10414)));
        locals.var_sce2_dn8 = (-((locals.var_psce2_loc * locals.var_temp_dn8) / (assign11070_e10414 * assign11070_e10414)));
        locals.var_sce2_dn9 = (-((locals.var_psce2_loc * locals.var_temp_dn9) / (assign11070_e10414 * assign11070_e10414)));

        let assign11080_e10418: f64 = (2.0 * locals.var_xd0);
        let assign11080_e10422: f64 = (locals.var_xdsx / locals.var_xd0);
        let assign11080_e10423: f64 = (1.0 + assign11080_e10422);
        let assign11080_e10424: f64 = (assign11080_e10423).sqrt();
        let assign11080_e10426: f64 = (assign11080_e10424 - 1.0);
        let assign11080_e10427: f64 = (assign11080_e10418 * assign11080_e10426);
        let assign11080_e10431: f64 = (locals.var_cfdl_i * locals.var_dleff);
        let assign11080_e10432: f64 = (1.0 + assign11080_e10431);
        let assign11080_e10433: f64 = (assign11080_e10427 * assign11080_e10432);
        let assign11080_e10437: f64 = (locals.var_cfdlb_i * locals.var_xg20shift);
        let assign11080_e10438: f64 = (1.0 + assign11080_e10437);
        let assign11080_e10439: f64 = (assign11080_e10433 * assign11080_e10438);
        locals.var_temp = assign11080_e10439;
        locals.var_temp_dn4 = (((((((2.0 * locals.var_xd0_dn4) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn4 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn4)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn4))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn4)));
        locals.var_temp_dn6 = (((((((2.0 * locals.var_xd0_dn6) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn6 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn6)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn6))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn6)));
        locals.var_temp_dn7 = (((((((2.0 * locals.var_xd0_dn7) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn7 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn7)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn7))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn7)));
        locals.var_temp_dn8 = (((((((2.0 * locals.var_xd0_dn8) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn8 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn8)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn8))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn8)));
        locals.var_temp_dn9 = (((((((2.0 * locals.var_xd0_dn9) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn9 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn9)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn9))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn9)));

        let assign11090_e10442: f64 = (locals.var_cf1_loc * locals.var_temp);
        locals.var_dxg1_dibl = assign11090_e10442;
        locals.var_dxg1_dibl_dn4 = ((locals.var_cf1_loc_dn4 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn4));
        locals.var_dxg1_dibl_dn6 = ((locals.var_cf1_loc_dn6 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn6));
        locals.var_dxg1_dibl_dn7 = ((locals.var_cf1_loc_dn7 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn7));
        locals.var_dxg1_dibl_dn8 = ((locals.var_cf1_loc_dn8 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn8));
        locals.var_dxg1_dibl_dn9 = ((locals.var_cf1_loc_dn9 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn9));

        let assign11100_e10445: f64 = (locals.var_cf2_loc * locals.var_temp);
        locals.var_dxg2_dibl = assign11100_e10445;
        locals.var_dxg2_dibl_dn4 = ((locals.var_cf2_loc_dn4 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn4));
        locals.var_dxg2_dibl_dn6 = ((locals.var_cf2_loc_dn6 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn6));
        locals.var_dxg2_dibl_dn7 = ((locals.var_cf2_loc_dn7 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn7));
        locals.var_dxg2_dibl_dn8 = ((locals.var_cf2_loc_dn8 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn8));
        locals.var_dxg2_dibl_dn9 = ((locals.var_cf2_loc_dn9 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn9));

        let assign11110_e10448: f64 = (locals.var_xg10 - locals.var_xedge);
        let assign11110_e10450: f64 = (assign11110_e10448 + locals.var_dxg1_dibl);
        let assign11110_e10452: f64 = (assign11110_e10450 * locals.var_sce1);
        let assign11110_e10454: f64 = (assign11110_e10452 + locals.var_xedge);
        let assign11110_e10456: f64 = (assign11110_e10454 + locals.var_dxdsx);
        locals.var_xg1 = assign11110_e10456;
        locals.var_xg1_dn4 = ((((((locals.var_xg10_dn4 - locals.var_xedge_dn4) + locals.var_dxg1_dibl_dn4) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn4)) + locals.var_xedge_dn4) + locals.var_dxdsx_dn4);
        locals.var_xg1_dn6 = ((((((locals.var_xg10_dn6 - locals.var_xedge_dn6) + locals.var_dxg1_dibl_dn6) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn6)) + locals.var_xedge_dn6) + locals.var_dxdsx_dn6);
        locals.var_xg1_dn7 = ((((((locals.var_xg10_dn7 - locals.var_xedge_dn7) + locals.var_dxg1_dibl_dn7) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn7)) + locals.var_xedge_dn7) + locals.var_dxdsx_dn7);
        locals.var_xg1_dn8 = ((((((locals.var_xg10_dn8 - locals.var_xedge_dn8) + locals.var_dxg1_dibl_dn8) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn8)) + locals.var_xedge_dn8) + locals.var_dxdsx_dn8);
        locals.var_xg1_dn9 = ((((((locals.var_xg10_dn9 - locals.var_xedge_dn9) + locals.var_dxg1_dibl_dn9) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn9)) + locals.var_xedge_dn9) + locals.var_dxdsx_dn9);

        let assign11120_e10459: f64 = (locals.var_xg2eff - locals.var_xedge);
        let assign11120_e10461: f64 = (assign11120_e10459 + locals.var_dxg2_dibl);
        let assign11120_e10463: f64 = (assign11120_e10461 * locals.var_sce2);
        let assign11120_e10465: f64 = (assign11120_e10463 + locals.var_xedge);
        let assign11120_e10467: f64 = (assign11120_e10465 + locals.var_dxdsx);
        locals.var_xg2 = assign11120_e10467;
        locals.var_xg2_dn4 = ((((((locals.var_xg2eff_dn4 - locals.var_xedge_dn4) + locals.var_dxg2_dibl_dn4) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn4)) + locals.var_xedge_dn4) + locals.var_dxdsx_dn4);
        locals.var_xg2_dn6 = ((((((locals.var_xg2eff_dn6 - locals.var_xedge_dn6) + locals.var_dxg2_dibl_dn6) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn6)) + locals.var_xedge_dn6) + locals.var_dxdsx_dn6);
        locals.var_xg2_dn7 = ((((((locals.var_xg2eff_dn7 - locals.var_xedge_dn7) + locals.var_dxg2_dibl_dn7) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn7)) + locals.var_xedge_dn7) + locals.var_dxdsx_dn7);
        locals.var_xg2_dn8 = ((((((locals.var_xg2eff_dn8 - locals.var_xedge_dn8) + locals.var_dxg2_dibl_dn8) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn8)) + locals.var_xedge_dn8) + locals.var_dxdsx_dn8);
        locals.var_xg2_dn9 = ((((((locals.var_xg2eff_dn9 - locals.var_xedge_dn9) + locals.var_dxg2_dibl_dn9) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn9)) + locals.var_xedge_dn9) + locals.var_dxdsx_dn9);

        let assign11130_e10473: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11130_e10474: f64 = (locals.var_cic1_i * assign11130_e10473);
        let assign11130_e10475: f64 = (locals.var_xg2 + assign11130_e10474);
        let assign11130_e10477: f64 = (assign11130_e10475 + locals.var_xsatmax);
        let assign11130_e10482: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11130_e10483: f64 = (locals.var_cic1_i * assign11130_e10482);
        let assign11130_e10484: f64 = (locals.var_xg2 + assign11130_e10483);
        let assign11130_e10486: f64 = (assign11130_e10484 - locals.var_xsatmax);
        let assign11130_e10491: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11130_e10492: f64 = (locals.var_cic1_i * assign11130_e10491);
        let assign11130_e10493: f64 = (locals.var_xg2 + assign11130_e10492);
        let assign11130_e10495: f64 = (assign11130_e10493 - locals.var_xsatmax);
        let assign11130_e10496: f64 = (assign11130_e10486 * assign11130_e10495);
        let assign11130_e10498: f64 = (assign11130_e10496 + 0.01);
        let assign11130_e10499: f64 = (assign11130_e10498).sqrt();
        let assign11130_e10500: f64 = (assign11130_e10477 - assign11130_e10499);
        let assign11130_e10501: f64 = (0.5 * assign11130_e10500);
        locals.var_xg1x = assign11130_e10501;
        locals.var_xg1x_dn4 = (0.5 * (((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) - locals.var_xsatmax_dn4) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign11130_e10499))));
        locals.var_xg1x_dn6 = (0.5 * (((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) - locals.var_xsatmax_dn6) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign11130_e10499))));
        locals.var_xg1x_dn7 = (0.5 * (((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) - locals.var_xsatmax_dn7) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign11130_e10499))));
        locals.var_xg1x_dn8 = (0.5 * (((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) - locals.var_xsatmax_dn8) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign11130_e10499))));
        locals.var_xg1x_dn9 = (0.5 * (((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) - locals.var_xsatmax_dn9) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign11130_e10499))));

        let assign11140_e10507: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11140_e10508: f64 = (locals.var_cic2_i * assign11140_e10507);
        let assign11140_e10509: f64 = (locals.var_xg1 + assign11140_e10508);
        let assign11140_e10511: f64 = (assign11140_e10509 + locals.var_xsatmax);
        let assign11140_e10516: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11140_e10517: f64 = (locals.var_cic2_i * assign11140_e10516);
        let assign11140_e10518: f64 = (locals.var_xg1 + assign11140_e10517);
        let assign11140_e10520: f64 = (assign11140_e10518 - locals.var_xsatmax);
        let assign11140_e10525: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11140_e10526: f64 = (locals.var_cic2_i * assign11140_e10525);
        let assign11140_e10527: f64 = (locals.var_xg1 + assign11140_e10526);
        let assign11140_e10529: f64 = (assign11140_e10527 - locals.var_xsatmax);
        let assign11140_e10530: f64 = (assign11140_e10520 * assign11140_e10529);
        let assign11140_e10532: f64 = (assign11140_e10530 + 0.01);
        let assign11140_e10533: f64 = (assign11140_e10532).sqrt();
        let assign11140_e10534: f64 = (assign11140_e10511 - assign11140_e10533);
        let assign11140_e10535: f64 = (0.5 * assign11140_e10534);
        locals.var_xg2x = assign11140_e10535;
        locals.var_xg2x_dn4 = (0.5 * (((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) - locals.var_xsatmax_dn4) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign11140_e10533))));
        locals.var_xg2x_dn6 = (0.5 * (((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) - locals.var_xsatmax_dn6) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign11140_e10533))));
        locals.var_xg2x_dn7 = (0.5 * (((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) - locals.var_xsatmax_dn7) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign11140_e10533))));
        locals.var_xg2x_dn8 = (0.5 * (((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) - locals.var_xsatmax_dn8) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign11140_e10533))));
        locals.var_xg2x_dn9 = (0.5 * (((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) - locals.var_xsatmax_dn9) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign11140_e10533))));

        let assign11150_e10538: f64 = (locals.var_k1_1d_qm / locals.var_sce1);
        locals.var_k1 = assign11150_e10538;
        locals.var_k1_dn4 = (((locals.var_k1_1d_qm_dn4 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn4)) / (locals.var_sce1 * locals.var_sce1));
        locals.var_k1_dn6 = (((locals.var_k1_1d_qm_dn6 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn6)) / (locals.var_sce1 * locals.var_sce1));
        locals.var_k1_dn7 = (((locals.var_k1_1d_qm_dn7 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn7)) / (locals.var_sce1 * locals.var_sce1));
        locals.var_k1_dn8 = (((locals.var_k1_1d_qm_dn8 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn8)) / (locals.var_sce1 * locals.var_sce1));
        locals.var_k1_dn9 = (((locals.var_k1_1d_qm_dn9 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn9)) / (locals.var_sce1 * locals.var_sce1));

        let assign11160_e10541: f64 = (locals.var_k2_1d_qm / locals.var_sce2);
        locals.var_k2 = assign11160_e10541;
        locals.var_k2_dn4 = (((locals.var_k2_1d_qm_dn4 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn4)) / (locals.var_sce2 * locals.var_sce2));
        locals.var_k2_dn6 = (((locals.var_k2_1d_qm_dn6 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn6)) / (locals.var_sce2 * locals.var_sce2));
        locals.var_k2_dn7 = (((locals.var_k2_1d_qm_dn7 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn7)) / (locals.var_sce2 * locals.var_sce2));
        locals.var_k2_dn8 = (((locals.var_k2_1d_qm_dn8 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn8)) / (locals.var_sce2 * locals.var_sce2));
        locals.var_k2_dn9 = (((locals.var_k2_1d_qm_dn9 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn9)) / (locals.var_sce2 * locals.var_sce2));

        let assign11170_e10544: f64 = (1.0 / locals.var_k1);
        locals.var_inv_k1 = assign11170_e10544;
        locals.var_inv_k1_dn4 = (-(locals.var_k1_dn4 / (locals.var_k1 * locals.var_k1)));
        locals.var_inv_k1_dn6 = (-(locals.var_k1_dn6 / (locals.var_k1 * locals.var_k1)));
        locals.var_inv_k1_dn7 = (-(locals.var_k1_dn7 / (locals.var_k1 * locals.var_k1)));
        locals.var_inv_k1_dn8 = (-(locals.var_k1_dn8 / (locals.var_k1 * locals.var_k1)));
        locals.var_inv_k1_dn9 = (-(locals.var_k1_dn9 / (locals.var_k1 * locals.var_k1)));

        let assign11180_e10547: f64 = (1.0 / locals.var_k2);
        locals.var_inv_k2 = assign11180_e10547;
        locals.var_inv_k2_dn4 = (-(locals.var_k2_dn4 / (locals.var_k2 * locals.var_k2)));
        locals.var_inv_k2_dn6 = (-(locals.var_k2_dn6 / (locals.var_k2 * locals.var_k2)));
        locals.var_inv_k2_dn7 = (-(locals.var_k2_dn7 / (locals.var_k2 * locals.var_k2)));
        locals.var_inv_k2_dn8 = (-(locals.var_k2_dn8 / (locals.var_k2 * locals.var_k2)));
        locals.var_inv_k2_dn9 = (-(locals.var_k2_dn9 / (locals.var_k2 * locals.var_k2)));

        let assign11190_e10551: f64 = (1.0 + locals.var_inv_k1);
        let assign11190_e10553: f64 = (assign11190_e10551 + locals.var_inv_k2);
        let assign11190_e10554: f64 = (1.0 / assign11190_e10553);
        locals.var_keq = assign11190_e10554;
        locals.var_keq_dn4 = (-((locals.var_inv_k1_dn4 + locals.var_inv_k2_dn4) / (assign11190_e10553 * assign11190_e10553)));
        locals.var_keq_dn6 = (-((locals.var_inv_k1_dn6 + locals.var_inv_k2_dn6) / (assign11190_e10553 * assign11190_e10553)));
        locals.var_keq_dn7 = (-((locals.var_inv_k1_dn7 + locals.var_inv_k2_dn7) / (assign11190_e10553 * assign11190_e10553)));
        locals.var_keq_dn8 = (-((locals.var_inv_k1_dn8 + locals.var_inv_k2_dn8) / (assign11190_e10553 * assign11190_e10553)));
        locals.var_keq_dn9 = (-((locals.var_inv_k1_dn9 + locals.var_inv_k2_dn9) / (assign11190_e10553 * assign11190_e10553)));

        let assign11200_e10558: f64 = (locals.var_csiprime * locals.var_csiprime);
        let assign11200_e10559: f64 = (locals.var_a0_csisq / assign11200_e10558);
        locals.var_a0 = assign11200_e10559;
        locals.var_a0_dn4 = (((locals.var_a0_csisq_dn4 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn4 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn4)))) / (assign11200_e10558 * assign11200_e10558));
        locals.var_a0_dn6 = (((locals.var_a0_csisq_dn6 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn6 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn6)))) / (assign11200_e10558 * assign11200_e10558));
        locals.var_a0_dn7 = (((locals.var_a0_csisq_dn7 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn7 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn7)))) / (assign11200_e10558 * assign11200_e10558));
        locals.var_a0_dn8 = (((locals.var_a0_csisq_dn8 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn8 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn8)))) / (assign11200_e10558 * assign11200_e10558));
        locals.var_a0_dn9 = (((locals.var_a0_csisq_dn9 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn9 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn9)))) / (assign11200_e10558 * assign11200_e10558));

        let assign11210_e10562: f64 = (1.0 + locals.var_k1);
        let assign11210_e10565: f64 = (1.0 + locals.var_k2);
        let assign11210_e10566: f64 = (assign11210_e10562 / assign11210_e10565);
        locals.var_exp_dxth = assign11210_e10566;
        locals.var_exp_dxth_dn4 = (((locals.var_k1_dn4 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn4)) / (assign11210_e10565 * assign11210_e10565));
        locals.var_exp_dxth_dn6 = (((locals.var_k1_dn6 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn6)) / (assign11210_e10565 * assign11210_e10565));
        locals.var_exp_dxth_dn7 = (((locals.var_k1_dn7 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn7)) / (assign11210_e10565 * assign11210_e10565));
        locals.var_exp_dxth_dn8 = (((locals.var_k1_dn8 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn8)) / (assign11210_e10565 * assign11210_e10565));
        locals.var_exp_dxth_dn9 = (((locals.var_k1_dn9 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn9)) / (assign11210_e10565 * assign11210_e10565));

        let assign11220_e10568: f64 = (locals.var_exp_dxth).ln();
        locals.var_dxth = assign11220_e10568;
        locals.var_dxth_dn4 = (locals.var_exp_dxth_dn4 / locals.var_exp_dxth);
        locals.var_dxth_dn6 = (locals.var_exp_dxth_dn6 / locals.var_exp_dxth);
        locals.var_dxth_dn7 = (locals.var_exp_dxth_dn7 / locals.var_exp_dxth);
        locals.var_dxth_dn8 = (locals.var_exp_dxth_dn8 / locals.var_exp_dxth);
        locals.var_dxth_dn9 = (locals.var_exp_dxth_dn9 / locals.var_exp_dxth);

        let assign11230_e10571: f64 = if locals.var_dxth > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign11230_e10571;

        let (assign11240_e10585, assign11240_e10585_d_n4, assign11240_e10585_d_n6, assign11240_e10585_d_n7, assign11240_e10585_d_n8, assign11240_e10585_d_n9,) = {
    if (locals.var_guard547 != 0.0) {
        let assign11240_e10575: f64 = (2.0 * locals.var_dxth);
        let assign11240_e10578: f64 = (locals.var_exp_dxth + 1.0);
        let assign11240_e10579: f64 = (assign11240_e10575 * assign11240_e10578);
        let assign11240_e10582: f64 = (locals.var_exp_dxth - 1.0);
        let assign11240_e10583: f64 = (assign11240_e10579 / assign11240_e10582);
        (assign11240_e10583, ((((((2.0 * locals.var_dxth_dn4) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn4)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn4)) / (assign11240_e10582 * assign11240_e10582)), ((((((2.0 * locals.var_dxth_dn6) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn6)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn6)) / (assign11240_e10582 * assign11240_e10582)), ((((((2.0 * locals.var_dxth_dn7) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn7)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn7)) / (assign11240_e10582 * assign11240_e10582)), ((((((2.0 * locals.var_dxth_dn8) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn8)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn8)) / (assign11240_e10582 * assign11240_e10582)), ((((((2.0 * locals.var_dxth_dn9) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn9)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn9)) / (assign11240_e10582 * assign11240_e10582)),)
    } else {
        (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9,)
    }
};
        locals.var_diff_min = assign11240_e10585;
        locals.var_diff_min_dn4 = assign11240_e10585_d_n4;
        locals.var_diff_min_dn6 = assign11240_e10585_d_n6;
        locals.var_diff_min_dn7 = assign11240_e10585_d_n7;
        locals.var_diff_min_dn8 = assign11240_e10585_d_n8;
        locals.var_diff_min_dn9 = assign11240_e10585_d_n9;

        let (assign11250_e10594, assign11250_e10594_d_n4, assign11250_e10594_d_n6, assign11250_e10594_d_n7, assign11250_e10594_d_n8, assign11250_e10594_d_n9,) = {
    if (locals.var_guard547 == 0.0) {
        let assign11250_e10591: f64 = (2.0 + locals.var_dxth);
        let assign11250_e10592: f64 = (2.0 * assign11250_e10591);
        (assign11250_e10592, (2.0 * locals.var_dxth_dn4), (2.0 * locals.var_dxth_dn6), (2.0 * locals.var_dxth_dn7), (2.0 * locals.var_dxth_dn8), (2.0 * locals.var_dxth_dn9),)
    } else {
        (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9,)
    }
};
        locals.var_diff_min = assign11250_e10594;
        locals.var_diff_min_dn4 = assign11250_e10594_d_n4;
        locals.var_diff_min_dn6 = assign11250_e10594_d_n6;
        locals.var_diff_min_dn7 = assign11250_e10594_d_n7;
        locals.var_diff_min_dn8 = assign11250_e10594_d_n8;
        locals.var_diff_min_dn9 = assign11250_e10594_d_n9;

        let assign11260_e10598: f64 = (locals.var_xg1x - locals.var_xg2x);
        let assign11260_e10599: f64 = (locals.var_keq * assign11260_e10598);
        locals.var_dx_wi = assign11260_e10599;
        locals.var_dx_wi_dn4 = ((locals.var_keq_dn4 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn4 - locals.var_xg2x_dn4)));
        locals.var_dx_wi_dn6 = ((locals.var_keq_dn6 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn6 - locals.var_xg2x_dn6)));
        locals.var_dx_wi_dn7 = ((locals.var_keq_dn7 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn7 - locals.var_xg2x_dn7)));
        locals.var_dx_wi_dn8 = ((locals.var_keq_dn8 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn8 - locals.var_xg2x_dn8)));
        locals.var_dx_wi_dn9 = ((locals.var_keq_dn9 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn9 - locals.var_xg2x_dn9)));

        let assign11270_e10602: f64 = (locals.var_dx_wi * locals.var_dx_wi);
        locals.var_dx_wisq = assign11270_e10602;
        locals.var_dx_wisq_dn4 = ((locals.var_dx_wi_dn4 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn4));
        locals.var_dx_wisq_dn6 = ((locals.var_dx_wi_dn6 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn6));
        locals.var_dx_wisq_dn7 = ((locals.var_dx_wi_dn7 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn7));
        locals.var_dx_wisq_dn8 = ((locals.var_dx_wi_dn8 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn8));
        locals.var_dx_wisq_dn9 = ((locals.var_dx_wi_dn9 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn9));

        let assign11280_e10606: f64 = (locals.var_dx_wi * locals.var_inv_k1);
        let assign11280_e10607: f64 = (locals.var_xg1x - assign11280_e10606);
        locals.var_x1_wi0 = assign11280_e10607;
        locals.var_x1_wi0_dn4 = (locals.var_xg1x_dn4 - ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4)));
        locals.var_x1_wi0_dn6 = (locals.var_xg1x_dn6 - ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6)));
        locals.var_x1_wi0_dn7 = (locals.var_xg1x_dn7 - ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7)));
        locals.var_x1_wi0_dn8 = (locals.var_xg1x_dn8 - ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8)));
        locals.var_x1_wi0_dn9 = (locals.var_xg1x_dn9 - ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9)));

        let assign11290_e10611: f64 = (locals.var_dx_wi * locals.var_inv_k2);
        let assign11290_e10612: f64 = (locals.var_xg2x + assign11290_e10611);
        locals.var_x2_wi0 = assign11290_e10612;
        locals.var_x2_wi0_dn4 = (locals.var_xg2x_dn4 + ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4)));
        locals.var_x2_wi0_dn6 = (locals.var_xg2x_dn6 + ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6)));
        locals.var_x2_wi0_dn7 = (locals.var_xg2x_dn7 + ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7)));
        locals.var_x2_wi0_dn8 = (locals.var_xg2x_dn8 + ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8)));
        locals.var_x2_wi0_dn9 = (locals.var_xg2x_dn9 + ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9)));

        let assign11300_e10616: f64 = (locals.var_k1 + 1.0);
        let assign11300_e10617: f64 = (1.0 / assign11300_e10616);
        locals.var_q_temp1 = assign11300_e10617;
        locals.var_q_temp1_dn4 = (-(locals.var_k1_dn4 / (assign11300_e10616 * assign11300_e10616)));
        locals.var_q_temp1_dn6 = (-(locals.var_k1_dn6 / (assign11300_e10616 * assign11300_e10616)));
        locals.var_q_temp1_dn7 = (-(locals.var_k1_dn7 / (assign11300_e10616 * assign11300_e10616)));
        locals.var_q_temp1_dn8 = (-(locals.var_k1_dn8 / (assign11300_e10616 * assign11300_e10616)));
        locals.var_q_temp1_dn9 = (-(locals.var_k1_dn9 / (assign11300_e10616 * assign11300_e10616)));

        let assign11310_e10621: f64 = (locals.var_k2 + 1.0);
        let assign11310_e10622: f64 = (1.0 / assign11310_e10621);
        locals.var_q_temp2 = assign11310_e10622;
        locals.var_q_temp2_dn4 = (-(locals.var_k2_dn4 / (assign11310_e10621 * assign11310_e10621)));
        locals.var_q_temp2_dn6 = (-(locals.var_k2_dn6 / (assign11310_e10621 * assign11310_e10621)));
        locals.var_q_temp2_dn7 = (-(locals.var_k2_dn7 / (assign11310_e10621 * assign11310_e10621)));
        locals.var_q_temp2_dn8 = (-(locals.var_k2_dn8 / (assign11310_e10621 * assign11310_e10621)));
        locals.var_q_temp2_dn9 = (-(locals.var_k2_dn9 / (assign11310_e10621 * assign11310_e10621)));

        let assign11320_e10626: f64 = (locals.var_k2 * locals.var_q_temp2);
        let assign11320_e10627: f64 = (locals.var_k1 + assign11320_e10626);
        let assign11320_e10629: f64 = (assign11320_e10627 * locals.var_diff_min);
        let assign11320_e10631: f64 = (assign11320_e10629 / locals.var_a0);
        let assign11320_e10632: f64 = (assign11320_e10631).ln();
        let assign11320_e10634: f64 = assign11320_e10632;
        let assign11320_e10636: f64 = (assign11320_e10634 + 3.0);
        locals.var_q_x1sat = assign11320_e10636;
        locals.var_q_x1sat_dn4 = (((((((locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn4))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631);
        locals.var_q_x1sat_dn6 = (((((((locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn6))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631);
        locals.var_q_x1sat_dn7 = (((((((locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn7))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631);
        locals.var_q_x1sat_dn8 = (((((((locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn8))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631);
        locals.var_q_x1sat_dn9 = (((((((locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn9))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631);

        let assign11330_e10640: f64 = (locals.var_k1 * locals.var_q_temp1);
        let assign11330_e10641: f64 = (locals.var_k2 + assign11330_e10640);
        let assign11330_e10643: f64 = (assign11330_e10641 * locals.var_diff_min);
        let assign11330_e10645: f64 = (assign11330_e10643 / locals.var_a0);
        let assign11330_e10646: f64 = (assign11330_e10645).ln();
        let assign11330_e10648: f64 = assign11330_e10646;
        let assign11330_e10650: f64 = (assign11330_e10648 + 3.0);
        locals.var_q_x2sat = assign11330_e10650;
        locals.var_q_x2sat_dn4 = (((((((locals.var_k2_dn4 + ((locals.var_k1_dn4 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn4))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645);
        locals.var_q_x2sat_dn6 = (((((((locals.var_k2_dn6 + ((locals.var_k1_dn6 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn6))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645);
        locals.var_q_x2sat_dn7 = (((((((locals.var_k2_dn7 + ((locals.var_k1_dn7 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn7))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645);
        locals.var_q_x2sat_dn8 = (((((((locals.var_k2_dn8 + ((locals.var_k1_dn8 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn8))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645);
        locals.var_q_x2sat_dn9 = (((((((locals.var_k2_dn9 + ((locals.var_k1_dn9 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn9))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645);

        let assign11340_e10653: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign11340_e10655: f64 = (assign11340_e10653 * 0.3333333333333);
        let assign11340_e10657: f64 = if assign11340_e10655 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign11340_e10657;

        let (assign11350_e10669, assign11350_e10669_d_n4, assign11350_e10669_d_n6, assign11350_e10669_d_n7, assign11350_e10669_d_n8, assign11350_e10669_d_n9,) = {
    if (locals.var_guard548 != 0.0) {
        let assign11350_e10662: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign11350_e10664: f64 = (assign11350_e10662 * 0.3333333333333);
        let assign11350_e10665: f64 = (assign11350_e10664).exp();
        let assign11350_e10666: f64 = (1.0 + assign11350_e10665);
        let assign11350_e10667: f64 = (assign11350_e10666).ln();
        (assign11350_e10667, ((assign11350_e10665 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333)) / assign11350_e10666), ((assign11350_e10665 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333)) / assign11350_e10666), ((assign11350_e10665 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333)) / assign11350_e10666), ((assign11350_e10665 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333)) / assign11350_e10666), ((assign11350_e10665 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333)) / assign11350_e10666),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11350_e10669;
        locals.var_q_temp3_dn4 = assign11350_e10669_d_n4;
        locals.var_q_temp3_dn6 = assign11350_e10669_d_n6;
        locals.var_q_temp3_dn7 = assign11350_e10669_d_n7;
        locals.var_q_temp3_dn8 = assign11350_e10669_d_n8;
        locals.var_q_temp3_dn9 = assign11350_e10669_d_n9;

        let (assign11360_e10678, assign11360_e10678_d_n4, assign11360_e10678_d_n6, assign11360_e10678_d_n7, assign11360_e10678_d_n8, assign11360_e10678_d_n9,) = {
    if (locals.var_guard548 == 0.0) {
        let assign11360_e10674: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign11360_e10676: f64 = (assign11360_e10674 * 0.3333333333333);
        (assign11360_e10676, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11360_e10678;
        locals.var_q_temp3_dn4 = assign11360_e10678_d_n4;
        locals.var_q_temp3_dn6 = assign11360_e10678_d_n6;
        locals.var_q_temp3_dn7 = assign11360_e10678_d_n7;
        locals.var_q_temp3_dn8 = assign11360_e10678_d_n8;
        locals.var_q_temp3_dn9 = assign11360_e10678_d_n9;

        let assign11370_e10682: f64 = (3.0 * locals.var_q_temp3);
        let assign11370_e10683: f64 = (locals.var_q_x1sat - assign11370_e10682);
        locals.var_q_x1 = assign11370_e10683;
        locals.var_q_x1_dn4 = (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x1_dn6 = (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x1_dn7 = (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x1_dn8 = (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x1_dn9 = (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign11380_e10686: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign11380_e10688: f64 = (assign11380_e10686 * 0.3333333333333);
        let assign11380_e10690: f64 = if assign11380_e10688 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign11380_e10690;

        let (assign11390_e10702, assign11390_e10702_d_n4, assign11390_e10702_d_n6, assign11390_e10702_d_n7, assign11390_e10702_d_n8, assign11390_e10702_d_n9,) = {
    if (locals.var_guard549 != 0.0) {
        let assign11390_e10695: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign11390_e10697: f64 = (assign11390_e10695 * 0.3333333333333);
        let assign11390_e10698: f64 = (assign11390_e10697).exp();
        let assign11390_e10699: f64 = (1.0 + assign11390_e10698);
        let assign11390_e10700: f64 = (assign11390_e10699).ln();
        (assign11390_e10700, ((assign11390_e10698 * ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333)) / assign11390_e10699), ((assign11390_e10698 * ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333)) / assign11390_e10699), ((assign11390_e10698 * ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333)) / assign11390_e10699), ((assign11390_e10698 * ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333)) / assign11390_e10699), ((assign11390_e10698 * ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333)) / assign11390_e10699),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11390_e10702;
        locals.var_q_temp3_dn4 = assign11390_e10702_d_n4;
        locals.var_q_temp3_dn6 = assign11390_e10702_d_n6;
        locals.var_q_temp3_dn7 = assign11390_e10702_d_n7;
        locals.var_q_temp3_dn8 = assign11390_e10702_d_n8;
        locals.var_q_temp3_dn9 = assign11390_e10702_d_n9;

        let (assign11400_e10711, assign11400_e10711_d_n4, assign11400_e10711_d_n6, assign11400_e10711_d_n7, assign11400_e10711_d_n8, assign11400_e10711_d_n9,) = {
    if (locals.var_guard549 == 0.0) {
        let assign11400_e10707: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign11400_e10709: f64 = (assign11400_e10707 * 0.3333333333333);
        (assign11400_e10709, ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11400_e10711;
        locals.var_q_temp3_dn4 = assign11400_e10711_d_n4;
        locals.var_q_temp3_dn6 = assign11400_e10711_d_n6;
        locals.var_q_temp3_dn7 = assign11400_e10711_d_n7;
        locals.var_q_temp3_dn8 = assign11400_e10711_d_n8;
        locals.var_q_temp3_dn9 = assign11400_e10711_d_n9;

        let assign11410_e10715: f64 = (3.0 * locals.var_q_temp3);
        let assign11410_e10716: f64 = (locals.var_q_x2sat - assign11410_e10715);
        locals.var_q_x2 = assign11410_e10716;
        locals.var_q_x2_dn4 = (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x2_dn6 = (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x2_dn7 = (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x2_dn8 = (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x2_dn9 = (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

    }

    pub(super) fn stamp_transient_block_26(
        locals: &mut StampLocals,
    ) {
        let assign11420_e10719: f64 = (locals.var_k1 * locals.var_xg1x);
        let assign11420_e10721: f64 = (assign11420_e10719 + locals.var_q_x2);
        let assign11420_e10723: f64 = (assign11420_e10721 * locals.var_q_temp1);
        locals.var_q_x1_wi = assign11420_e10723;
        locals.var_q_x1_wi_dn4 = (((((locals.var_k1_dn4 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn4)) + locals.var_q_x2_dn4) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn4));
        locals.var_q_x1_wi_dn6 = (((((locals.var_k1_dn6 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn6)) + locals.var_q_x2_dn6) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn6));
        locals.var_q_x1_wi_dn7 = (((((locals.var_k1_dn7 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn7)) + locals.var_q_x2_dn7) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn7));
        locals.var_q_x1_wi_dn8 = (((((locals.var_k1_dn8 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn8)) + locals.var_q_x2_dn8) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn8));
        locals.var_q_x1_wi_dn9 = (((((locals.var_k1_dn9 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn9)) + locals.var_q_x2_dn9) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn9));

        let assign11430_e10726: f64 = (locals.var_k2 * locals.var_xg2x);
        let assign11430_e10728: f64 = (assign11430_e10726 + locals.var_q_x1);
        let assign11430_e10730: f64 = (assign11430_e10728 * locals.var_q_temp2);
        locals.var_q_x2_wi = assign11430_e10730;
        locals.var_q_x2_wi_dn4 = (((((locals.var_k2_dn4 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn4)) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn4));
        locals.var_q_x2_wi_dn6 = (((((locals.var_k2_dn6 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn6)) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn6));
        locals.var_q_x2_wi_dn7 = (((((locals.var_k2_dn7 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn7)) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn7));
        locals.var_q_x2_wi_dn8 = (((((locals.var_k2_dn8 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn8)) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn8));
        locals.var_q_x2_wi_dn9 = (((((locals.var_k2_dn9 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn9)) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn9));

        let assign11440_e10733: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign11440_e10735: f64 = (assign11440_e10733 * 0.3333333333333);
        let assign11440_e10737: f64 = if assign11440_e10735 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign11440_e10737;

        let (assign11450_e10749, assign11450_e10749_d_n4, assign11450_e10749_d_n6, assign11450_e10749_d_n7, assign11450_e10749_d_n8, assign11450_e10749_d_n9,) = {
    if (locals.var_guard550 != 0.0) {
        let assign11450_e10742: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign11450_e10744: f64 = (assign11450_e10742 * 0.3333333333333);
        let assign11450_e10745: f64 = (assign11450_e10744).exp();
        let assign11450_e10746: f64 = (1.0 + assign11450_e10745);
        let assign11450_e10747: f64 = (assign11450_e10746).ln();
        (assign11450_e10747, ((assign11450_e10745 * ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333)) / assign11450_e10746), ((assign11450_e10745 * ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333)) / assign11450_e10746), ((assign11450_e10745 * ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333)) / assign11450_e10746), ((assign11450_e10745 * ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333)) / assign11450_e10746), ((assign11450_e10745 * ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333)) / assign11450_e10746),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11450_e10749;
        locals.var_q_temp3_dn4 = assign11450_e10749_d_n4;
        locals.var_q_temp3_dn6 = assign11450_e10749_d_n6;
        locals.var_q_temp3_dn7 = assign11450_e10749_d_n7;
        locals.var_q_temp3_dn8 = assign11450_e10749_d_n8;
        locals.var_q_temp3_dn9 = assign11450_e10749_d_n9;

        let (assign11460_e10758, assign11460_e10758_d_n4, assign11460_e10758_d_n6, assign11460_e10758_d_n7, assign11460_e10758_d_n8, assign11460_e10758_d_n9,) = {
    if (locals.var_guard550 == 0.0) {
        let assign11460_e10754: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign11460_e10756: f64 = (assign11460_e10754 * 0.3333333333333);
        (assign11460_e10756, ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11460_e10758;
        locals.var_q_temp3_dn4 = assign11460_e10758_d_n4;
        locals.var_q_temp3_dn6 = assign11460_e10758_d_n6;
        locals.var_q_temp3_dn7 = assign11460_e10758_d_n7;
        locals.var_q_temp3_dn8 = assign11460_e10758_d_n8;
        locals.var_q_temp3_dn9 = assign11460_e10758_d_n9;

        let assign11470_e10762: f64 = (3.0 * locals.var_q_temp3);
        let assign11470_e10763: f64 = (locals.var_q_x1sat - assign11470_e10762);
        locals.var_q_x1 = assign11470_e10763;
        locals.var_q_x1_dn4 = (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x1_dn6 = (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x1_dn7 = (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x1_dn8 = (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x1_dn9 = (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign11480_e10766: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign11480_e10768: f64 = (assign11480_e10766 * 0.3333333333333);
        let assign11480_e10770: f64 = if assign11480_e10768 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign11480_e10770;

        let (assign11490_e10782, assign11490_e10782_d_n4, assign11490_e10782_d_n6, assign11490_e10782_d_n7, assign11490_e10782_d_n8, assign11490_e10782_d_n9,) = {
    if (locals.var_guard551 != 0.0) {
        let assign11490_e10775: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign11490_e10777: f64 = (assign11490_e10775 * 0.3333333333333);
        let assign11490_e10778: f64 = (assign11490_e10777).exp();
        let assign11490_e10779: f64 = (1.0 + assign11490_e10778);
        let assign11490_e10780: f64 = (assign11490_e10779).ln();
        (assign11490_e10780, ((assign11490_e10778 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333)) / assign11490_e10779), ((assign11490_e10778 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333)) / assign11490_e10779), ((assign11490_e10778 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333)) / assign11490_e10779), ((assign11490_e10778 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333)) / assign11490_e10779), ((assign11490_e10778 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333)) / assign11490_e10779),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11490_e10782;
        locals.var_q_temp3_dn4 = assign11490_e10782_d_n4;
        locals.var_q_temp3_dn6 = assign11490_e10782_d_n6;
        locals.var_q_temp3_dn7 = assign11490_e10782_d_n7;
        locals.var_q_temp3_dn8 = assign11490_e10782_d_n8;
        locals.var_q_temp3_dn9 = assign11490_e10782_d_n9;

        let (assign11500_e10791, assign11500_e10791_d_n4, assign11500_e10791_d_n6, assign11500_e10791_d_n7, assign11500_e10791_d_n8, assign11500_e10791_d_n9,) = {
    if (locals.var_guard551 == 0.0) {
        let assign11500_e10787: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign11500_e10789: f64 = (assign11500_e10787 * 0.3333333333333);
        (assign11500_e10789, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11500_e10791;
        locals.var_q_temp3_dn4 = assign11500_e10791_d_n4;
        locals.var_q_temp3_dn6 = assign11500_e10791_d_n6;
        locals.var_q_temp3_dn7 = assign11500_e10791_d_n7;
        locals.var_q_temp3_dn8 = assign11500_e10791_d_n8;
        locals.var_q_temp3_dn9 = assign11500_e10791_d_n9;

        let assign11510_e10795: f64 = (3.0 * locals.var_q_temp3);
        let assign11510_e10796: f64 = (locals.var_q_x2sat - assign11510_e10795);
        locals.var_q_x2 = assign11510_e10796;
        locals.var_q_x2_dn4 = (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x2_dn6 = (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x2_dn7 = (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x2_dn8 = (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x2_dn9 = (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign11520_e10799: f64 = (locals.var_xg1x - locals.var_q_x1);
        locals.var_q1s = assign11520_e10799;
        locals.var_q1s_dn4 = (locals.var_xg1x_dn4 - locals.var_q_x1_dn4);
        locals.var_q1s_dn6 = (locals.var_xg1x_dn6 - locals.var_q_x1_dn6);
        locals.var_q1s_dn7 = (locals.var_xg1x_dn7 - locals.var_q_x1_dn7);
        locals.var_q1s_dn8 = (locals.var_xg1x_dn8 - locals.var_q_x1_dn8);
        locals.var_q1s_dn9 = (locals.var_xg1x_dn9 - locals.var_q_x1_dn9);

        let assign11530_e10802: f64 = (locals.var_xg2x - locals.var_q_x2);
        locals.var_q2s = assign11530_e10802;
        locals.var_q2s_dn4 = (locals.var_xg2x_dn4 - locals.var_q_x2_dn4);
        locals.var_q2s_dn6 = (locals.var_xg2x_dn6 - locals.var_q_x2_dn6);
        locals.var_q2s_dn7 = (locals.var_xg2x_dn7 - locals.var_q_x2_dn7);
        locals.var_q2s_dn8 = (locals.var_xg2x_dn8 - locals.var_q_x2_dn8);
        locals.var_q2s_dn9 = (locals.var_xg2x_dn9 - locals.var_q_x2_dn9);

        locals.var_q_rac_qsq = 0.0;
        locals.var_q_rac_qsq_dn4 = 0.0;
        locals.var_q_rac_qsq_dn6 = 0.0;
        locals.var_q_rac_qsq_dn7 = 0.0;
        locals.var_q_rac_qsq_dn8 = 0.0;
        locals.var_q_rac_qsq_dn9 = 0.0;

        locals.var_q_invexpq = 0.0;
        locals.var_q_invexpq_dn4 = 0.0;
        locals.var_q_invexpq_dn6 = 0.0;
        locals.var_q_invexpq_dn7 = 0.0;
        locals.var_q_invexpq_dn8 = 0.0;
        locals.var_q_invexpq_dn9 = 0.0;

        let assign11560_e10807: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign11560_e10807;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign11570_e10810: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11570_e10812: f64 = assign11570_e10810;
        let assign11570_e10814: f64 = if assign11570_e10812 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign11570_e10814;

        let (assign11580_e10823, assign11580_e10823_d_n4, assign11580_e10823_d_n6, assign11580_e10823_d_n7, assign11580_e10823_d_n8, assign11580_e10823_d_n9,) = {
    if (locals.var_guard552 != 0.0) {
        let assign11580_e10818: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11580_e10820: f64 = assign11580_e10818;
        let assign11580_e10821: f64 = (assign11580_e10820).exp();
        (assign11580_e10821, (assign11580_e10821 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign11580_e10821 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign11580_e10821 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign11580_e10821 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign11580_e10821 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11580_e10823;
        locals.var_q_temp1_dn4 = assign11580_e10823_d_n4;
        locals.var_q_temp1_dn6 = assign11580_e10823_d_n6;
        locals.var_q_temp1_dn7 = assign11580_e10823_d_n7;
        locals.var_q_temp1_dn8 = assign11580_e10823_d_n8;
        locals.var_q_temp1_dn9 = assign11580_e10823_d_n9;

        let (assign11590_e10862, assign11590_e10862_d_n4, assign11590_e10862_d_n6, assign11590_e10862_d_n7, assign11590_e10862_d_n8, assign11590_e10862_d_n9,) = {
    if (locals.var_guard552 == 0.0) {
        let assign11590_e10830: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11590_e10832: f64 = assign11590_e10830;
        let assign11590_e10834: f64 = (assign11590_e10832 - 80.0);
        let assign11590_e10839: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11590_e10841: f64 = assign11590_e10839;
        let assign11590_e10843: f64 = (assign11590_e10841 - 80.0);
        let assign11590_e10844: f64 = (0.5 * assign11590_e10843);
        let assign11590_e10848: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11590_e10850: f64 = assign11590_e10848;
        let assign11590_e10852: f64 = (assign11590_e10850 - 80.0);
        let assign11590_e10854: f64 = (assign11590_e10852 * 0.3333333333333);
        let assign11590_e10855: f64 = (1.0 + assign11590_e10854);
        let assign11590_e10856: f64 = (assign11590_e10844 * assign11590_e10855);
        let assign11590_e10857: f64 = (1.0 + assign11590_e10856);
        let assign11590_e10858: f64 = (assign11590_e10834 * assign11590_e10857);
        let assign11590_e10859: f64 = (1.0 + assign11590_e10858);
        let assign11590_e10860: f64 = (5.54062e34 * assign11590_e10859);
        (assign11590_e10860, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11590_e10862;
        locals.var_q_temp1_dn4 = assign11590_e10862_d_n4;
        locals.var_q_temp1_dn6 = assign11590_e10862_d_n6;
        locals.var_q_temp1_dn7 = assign11590_e10862_d_n7;
        locals.var_q_temp1_dn8 = assign11590_e10862_d_n8;
        locals.var_q_temp1_dn9 = assign11590_e10862_d_n9;

        let assign11600_e10865: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign11600_e10865;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign11610_e10868: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign11610_e10870: f64 = (assign11610_e10868 - locals.var_q_aexp);
        locals.var_q_qsq = assign11610_e10870;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign11620_e10873: f64 = (2.0 * locals.var_k1);
        let assign11620_e10875: f64 = (assign11620_e10873 * locals.var_q_k1q1);
        let assign11620_e10877: f64 = (assign11620_e10875 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign11620_e10877;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign11630_e10880: f64 = (2.0 * locals.var_k1);
        let assign11630_e10882: f64 = (assign11630_e10880 * locals.var_k1);
        let assign11630_e10884: f64 = (assign11630_e10882 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign11630_e10884;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign11640_e10887: f64 = (-0.005);
        let assign11640_e10888: f64 = if locals.var_q_qsq < assign11640_e10887 { 1.0 } else { 0.0 };
        locals.var_guard553 = assign11640_e10888;

        let (assign11650_e10894, assign11650_e10894_d_n4, assign11650_e10894_d_n6, assign11650_e10894_d_n7, assign11650_e10894_d_n8, assign11650_e10894_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11650_e10891: f64 = (locals.var_q_qsq).abs();
        let assign11650_e10892: f64 = (assign11650_e10891).sqrt();
        (assign11650_e10892, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign11650_e10892)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign11650_e10892)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign11650_e10892)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign11650_e10892)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign11650_e10892)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign11650_e10894;
        locals.var_q_rac_qsq_dn4 = assign11650_e10894_d_n4;
        locals.var_q_rac_qsq_dn6 = assign11650_e10894_d_n6;
        locals.var_q_rac_qsq_dn7 = assign11650_e10894_d_n7;
        locals.var_q_rac_qsq_dn8 = assign11650_e10894_d_n8;
        locals.var_q_rac_qsq_dn9 = assign11650_e10894_d_n9;

        let (assign11660_e10903, assign11660_e10903_d_n4, assign11660_e10903_d_n6, assign11660_e10903_d_n7, assign11660_e10903_d_n8, assign11660_e10903_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11660_e10899: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign11660_e10900: f64 = (assign11660_e10899).tan();
        let assign11660_e10901: f64 = (locals.var_q_rac_qsq / assign11660_e10900);
        (assign11660_e10901, (((locals.var_q_rac_qsq_dn4 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), (((locals.var_q_rac_qsq_dn6 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), (((locals.var_q_rac_qsq_dn7 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), (((locals.var_q_rac_qsq_dn8 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), (((locals.var_q_rac_qsq_dn9 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign11660_e10903;
        locals.var_q_qcoth_dn4 = assign11660_e10903_d_n4;
        locals.var_q_qcoth_dn6 = assign11660_e10903_d_n6;
        locals.var_q_qcoth_dn7 = assign11660_e10903_d_n7;
        locals.var_q_qcoth_dn8 = assign11660_e10903_d_n8;
        locals.var_q_qcoth_dn9 = assign11660_e10903_d_n9;

        let (assign11670_e10911, assign11670_e10911_d_n4, assign11670_e10911_d_n6, assign11670_e10911_d_n7, assign11670_e10911_d_n8, assign11670_e10911_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11670_e10907: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign11670_e10909: f64 = (assign11670_e10907 / locals.var_q_qsq);
        (assign11670_e10909, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11670_e10911;
        locals.var_q_temp1_dn4 = assign11670_e10911_d_n4;
        locals.var_q_temp1_dn6 = assign11670_e10911_d_n6;
        locals.var_q_temp1_dn7 = assign11670_e10911_d_n7;
        locals.var_q_temp1_dn8 = assign11670_e10911_d_n8;
        locals.var_q_temp1_dn9 = assign11670_e10911_d_n9;

        let (assign11680_e10923, assign11680_e10923_d_n4, assign11680_e10923_d_n6, assign11680_e10923_d_n7, assign11680_e10923_d_n8, assign11680_e10923_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11680_e10917: f64 = (2.0 - locals.var_q_qcoth);
        let assign11680_e10918: f64 = (locals.var_q_qcoth * assign11680_e10917);
        let assign11680_e10919: f64 = (locals.var_q_qsq + assign11680_e10918);
        let assign11680_e10921: f64 = (assign11680_e10919 * locals.var_q_temp1);
        (assign11680_e10921, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign11680_e10923;
        locals.var_q_d1_qcoth_dn4 = assign11680_e10923_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign11680_e10923_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign11680_e10923_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign11680_e10923_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign11680_e10923_d_n9;

        let (assign11690_e10943, assign11690_e10943_d_n4, assign11690_e10943_d_n6, assign11690_e10943_d_n7, assign11690_e10943_d_n8, assign11690_e10943_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11690_e10928: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign11690_e10931: f64 = (1.0 + locals.var_q_qcoth);
        let assign11690_e10932: f64 = (assign11690_e10928 * assign11690_e10931);
        let assign11690_e10933: f64 = (locals.var_q_d1_qsq - assign11690_e10932);
        let assign11690_e10935: f64 = (assign11690_e10933 * locals.var_q_temp1);
        let assign11690_e10938: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign11690_e10940: f64 = (assign11690_e10938 / locals.var_q_d1_qsq);
        let assign11690_e10941: f64 = (assign11690_e10935 + assign11690_e10940);
        (assign11690_e10941, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign11690_e10943;
        locals.var_q_d2_qcoth_dn4 = assign11690_e10943_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign11690_e10943_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign11690_e10943_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign11690_e10943_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign11690_e10943_d_n9;

        let (assign11700_e10951, assign11700_e10951_d_n4, assign11700_e10951_d_n6, assign11700_e10951_d_n7, assign11700_e10951_d_n8, assign11700_e10951_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11700_e10948: f64 = (0.5 * locals.var_q_qcoth);
        let assign11700_e10949: f64 = (1.0 - assign11700_e10948);
        (assign11700_e10949, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign11700_e10951;
        locals.var_q_temp2_dn4 = assign11700_e10951_d_n4;
        locals.var_q_temp2_dn6 = assign11700_e10951_d_n6;
        locals.var_q_temp2_dn7 = assign11700_e10951_d_n7;
        locals.var_q_temp2_dn8 = assign11700_e10951_d_n8;
        locals.var_q_temp2_dn9 = assign11700_e10951_d_n9;

        let (assign11710_e10959, assign11710_e10959_d_n4, assign11710_e10959_d_n6, assign11710_e10959_d_n7, assign11710_e10959_d_n8, assign11710_e10959_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11710_e10955: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign11710_e10957: f64 = (assign11710_e10955 * locals.var_q_temp2);
        (assign11710_e10957, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign11710_e10959;
        locals.var_q_d1_ln_dn4 = assign11710_e10959_d_n4;
        locals.var_q_d1_ln_dn6 = assign11710_e10959_d_n6;
        locals.var_q_d1_ln_dn7 = assign11710_e10959_d_n7;
        locals.var_q_d1_ln_dn8 = assign11710_e10959_d_n8;
        locals.var_q_d1_ln_dn9 = assign11710_e10959_d_n9;

        let (assign11720_e10975, assign11720_e10975_d_n4, assign11720_e10975_d_n6, assign11720_e10975_d_n7, assign11720_e10975_d_n8, assign11720_e10975_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11720_e10963: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign11720_e10968: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign11720_e10969: f64 = (locals.var_q_d1_ln + assign11720_e10968);
        let assign11720_e10970: f64 = (locals.var_q_d1_qsq * assign11720_e10969);
        let assign11720_e10971: f64 = (assign11720_e10963 - assign11720_e10970);
        let assign11720_e10973: f64 = (assign11720_e10971 / locals.var_q_qsq);
        (assign11720_e10973, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign11720_e10975;
        locals.var_q_d2_ln_dn4 = assign11720_e10975_d_n4;
        locals.var_q_d2_ln_dn6 = assign11720_e10975_d_n6;
        locals.var_q_d2_ln_dn7 = assign11720_e10975_d_n7;
        locals.var_q_d2_ln_dn8 = assign11720_e10975_d_n8;
        locals.var_q_d2_ln_dn9 = assign11720_e10975_d_n9;

        let assign11730_e10978: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard554 = assign11730_e10978;

        let (assign11740_e10987, assign11740_e10987_d_n4, assign11740_e10987_d_n6, assign11740_e10987_d_n7, assign11740_e10987_d_n8, assign11740_e10987_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11740_e10984: f64 = (locals.var_q_qsq).abs();
        let assign11740_e10985: f64 = (assign11740_e10984).sqrt();
        (assign11740_e10985, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign11740_e10985)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign11740_e10985)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign11740_e10985)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign11740_e10985)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign11740_e10985)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign11740_e10987;
        locals.var_q_rac_qsq_dn4 = assign11740_e10987_d_n4;
        locals.var_q_rac_qsq_dn6 = assign11740_e10987_d_n6;
        locals.var_q_rac_qsq_dn7 = assign11740_e10987_d_n7;
        locals.var_q_rac_qsq_dn8 = assign11740_e10987_d_n8;
        locals.var_q_rac_qsq_dn9 = assign11740_e10987_d_n9;

        let (assign11750_e10996, assign11750_e10996_d_n4, assign11750_e10996_d_n6, assign11750_e10996_d_n7, assign11750_e10996_d_n8, assign11750_e10996_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11750_e10993: f64 = (-locals.var_q_rac_qsq);
        let assign11750_e10994: f64 = (assign11750_e10993).exp();
        (assign11750_e10994, (assign11750_e10994 * (-locals.var_q_rac_qsq_dn4)), (assign11750_e10994 * (-locals.var_q_rac_qsq_dn6)), (assign11750_e10994 * (-locals.var_q_rac_qsq_dn7)), (assign11750_e10994 * (-locals.var_q_rac_qsq_dn8)), (assign11750_e10994 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign11750_e10996;
        locals.var_q_invexpq_dn4 = assign11750_e10996_d_n4;
        locals.var_q_invexpq_dn6 = assign11750_e10996_d_n6;
        locals.var_q_invexpq_dn7 = assign11750_e10996_d_n7;
        locals.var_q_invexpq_dn8 = assign11750_e10996_d_n8;
        locals.var_q_invexpq_dn9 = assign11750_e10996_d_n9;

        let (assign11760_e11011, assign11760_e11011_d_n4, assign11760_e11011_d_n6, assign11760_e11011_d_n7, assign11760_e11011_d_n8, assign11760_e11011_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11760_e11004: f64 = (1.0 + locals.var_q_invexpq);
        let assign11760_e11005: f64 = (locals.var_q_rac_qsq * assign11760_e11004);
        let assign11760_e11008: f64 = (1.0 - locals.var_q_invexpq);
        let assign11760_e11009: f64 = (assign11760_e11005 / assign11760_e11008);
        (assign11760_e11009, (((((locals.var_q_rac_qsq_dn4 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn4))) / (assign11760_e11008 * assign11760_e11008)), (((((locals.var_q_rac_qsq_dn6 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn6))) / (assign11760_e11008 * assign11760_e11008)), (((((locals.var_q_rac_qsq_dn7 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn7))) / (assign11760_e11008 * assign11760_e11008)), (((((locals.var_q_rac_qsq_dn8 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn8))) / (assign11760_e11008 * assign11760_e11008)), (((((locals.var_q_rac_qsq_dn9 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn9))) / (assign11760_e11008 * assign11760_e11008)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign11760_e11011;
        locals.var_q_qcoth_dn4 = assign11760_e11011_d_n4;
        locals.var_q_qcoth_dn6 = assign11760_e11011_d_n6;
        locals.var_q_qcoth_dn7 = assign11760_e11011_d_n7;
        locals.var_q_qcoth_dn8 = assign11760_e11011_d_n8;
        locals.var_q_qcoth_dn9 = assign11760_e11011_d_n9;

        let (assign11770_e11022, assign11770_e11022_d_n4, assign11770_e11022_d_n6, assign11770_e11022_d_n7, assign11770_e11022_d_n8, assign11770_e11022_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11770_e11018: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign11770_e11020: f64 = (assign11770_e11018 / locals.var_q_qsq);
        (assign11770_e11020, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11770_e11022;
        locals.var_q_temp1_dn4 = assign11770_e11022_d_n4;
        locals.var_q_temp1_dn6 = assign11770_e11022_d_n6;
        locals.var_q_temp1_dn7 = assign11770_e11022_d_n7;
        locals.var_q_temp1_dn8 = assign11770_e11022_d_n8;
        locals.var_q_temp1_dn9 = assign11770_e11022_d_n9;

        let (assign11780_e11037, assign11780_e11037_d_n4, assign11780_e11037_d_n6, assign11780_e11037_d_n7, assign11780_e11037_d_n8, assign11780_e11037_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11780_e11031: f64 = (2.0 - locals.var_q_qcoth);
        let assign11780_e11032: f64 = (locals.var_q_qcoth * assign11780_e11031);
        let assign11780_e11033: f64 = (locals.var_q_qsq + assign11780_e11032);
        let assign11780_e11035: f64 = (assign11780_e11033 * locals.var_q_temp1);
        (assign11780_e11035, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign11780_e11037;
        locals.var_q_d1_qcoth_dn4 = assign11780_e11037_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign11780_e11037_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign11780_e11037_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign11780_e11037_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign11780_e11037_d_n9;

        let (assign11790_e11060, assign11790_e11060_d_n4, assign11790_e11060_d_n6, assign11790_e11060_d_n7, assign11790_e11060_d_n8, assign11790_e11060_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11790_e11045: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign11790_e11048: f64 = (1.0 + locals.var_q_qcoth);
        let assign11790_e11049: f64 = (assign11790_e11045 * assign11790_e11048);
        let assign11790_e11050: f64 = (locals.var_q_d1_qsq - assign11790_e11049);
        let assign11790_e11052: f64 = (assign11790_e11050 * locals.var_q_temp1);
        let assign11790_e11055: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign11790_e11057: f64 = (assign11790_e11055 / locals.var_q_d1_qsq);
        let assign11790_e11058: f64 = (assign11790_e11052 + assign11790_e11057);
        (assign11790_e11058, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign11790_e11060;
        locals.var_q_d2_qcoth_dn4 = assign11790_e11060_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign11790_e11060_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign11790_e11060_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign11790_e11060_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign11790_e11060_d_n9;

    }

    pub(super) fn stamp_transient_block_27(
        locals: &mut StampLocals,
    ) {
        let (assign11800_e11071, assign11800_e11071_d_n4, assign11800_e11071_d_n6, assign11800_e11071_d_n7, assign11800_e11071_d_n8, assign11800_e11071_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11800_e11068: f64 = (0.5 * locals.var_q_qcoth);
        let assign11800_e11069: f64 = (1.0 - assign11800_e11068);
        (assign11800_e11069, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign11800_e11071;
        locals.var_q_temp2_dn4 = assign11800_e11071_d_n4;
        locals.var_q_temp2_dn6 = assign11800_e11071_d_n6;
        locals.var_q_temp2_dn7 = assign11800_e11071_d_n7;
        locals.var_q_temp2_dn8 = assign11800_e11071_d_n8;
        locals.var_q_temp2_dn9 = assign11800_e11071_d_n9;

        let (assign11810_e11082, assign11810_e11082_d_n4, assign11810_e11082_d_n6, assign11810_e11082_d_n7, assign11810_e11082_d_n8, assign11810_e11082_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11810_e11078: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign11810_e11080: f64 = (assign11810_e11078 * locals.var_q_temp2);
        (assign11810_e11080, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign11810_e11082;
        locals.var_q_d1_ln_dn4 = assign11810_e11082_d_n4;
        locals.var_q_d1_ln_dn6 = assign11810_e11082_d_n6;
        locals.var_q_d1_ln_dn7 = assign11810_e11082_d_n7;
        locals.var_q_d1_ln_dn8 = assign11810_e11082_d_n8;
        locals.var_q_d1_ln_dn9 = assign11810_e11082_d_n9;

        let (assign11820_e11101, assign11820_e11101_d_n4, assign11820_e11101_d_n6, assign11820_e11101_d_n7, assign11820_e11101_d_n8, assign11820_e11101_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11820_e11089: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign11820_e11094: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign11820_e11095: f64 = (locals.var_q_d1_ln + assign11820_e11094);
        let assign11820_e11096: f64 = (locals.var_q_d1_qsq * assign11820_e11095);
        let assign11820_e11097: f64 = (assign11820_e11089 - assign11820_e11096);
        let assign11820_e11099: f64 = (assign11820_e11097 / locals.var_q_qsq);
        (assign11820_e11099, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign11820_e11101;
        locals.var_q_d2_ln_dn4 = assign11820_e11101_d_n4;
        locals.var_q_d2_ln_dn6 = assign11820_e11101_d_n6;
        locals.var_q_d2_ln_dn7 = assign11820_e11101_d_n7;
        locals.var_q_d2_ln_dn8 = assign11820_e11101_d_n8;
        locals.var_q_d2_ln_dn9 = assign11820_e11101_d_n9;

        let (assign11830_e11127, assign11830_e11127_d_n4, assign11830_e11127_d_n6, assign11830_e11127_d_n7, assign11830_e11127_d_n8, assign11830_e11127_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11830_e11111: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign11830_e11115: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign11830_e11119: f64 = (locals.var_q_qsq * 0.025);
        let assign11830_e11120: f64 = (1.0 - assign11830_e11119);
        let assign11830_e11121: f64 = (assign11830_e11115 * assign11830_e11120);
        let assign11830_e11122: f64 = (1.0 - assign11830_e11121);
        let assign11830_e11123: f64 = (assign11830_e11111 * assign11830_e11122);
        let assign11830_e11124: f64 = (1.0 - assign11830_e11123);
        let assign11830_e11125: f64 = (0.1666666666667 * assign11830_e11124);
        (assign11830_e11125, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11830_e11127;
        locals.var_q_temp3_dn4 = assign11830_e11127_d_n4;
        locals.var_q_temp3_dn6 = assign11830_e11127_d_n6;
        locals.var_q_temp3_dn7 = assign11830_e11127_d_n7;
        locals.var_q_temp3_dn8 = assign11830_e11127_d_n8;
        locals.var_q_temp3_dn9 = assign11830_e11127_d_n9;

        let (assign11840_e11139, assign11840_e11139_d_n4, assign11840_e11139_d_n6, assign11840_e11139_d_n7, assign11840_e11139_d_n8, assign11840_e11139_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11840_e11136: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign11840_e11137: f64 = (2.0 + assign11840_e11136);
        (assign11840_e11137, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign11840_e11139;
        locals.var_q_qcoth_dn4 = assign11840_e11139_d_n4;
        locals.var_q_qcoth_dn6 = assign11840_e11139_d_n6;
        locals.var_q_qcoth_dn7 = assign11840_e11139_d_n7;
        locals.var_q_qcoth_dn8 = assign11840_e11139_d_n8;
        locals.var_q_qcoth_dn9 = assign11840_e11139_d_n9;

        let (assign11850_e11165, assign11850_e11165_d_n4, assign11850_e11165_d_n6, assign11850_e11165_d_n7, assign11850_e11165_d_n8, assign11850_e11165_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11850_e11149: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign11850_e11153: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign11850_e11157: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign11850_e11158: f64 = (1.0 - assign11850_e11157);
        let assign11850_e11159: f64 = (assign11850_e11153 * assign11850_e11158);
        let assign11850_e11160: f64 = (1.0 - assign11850_e11159);
        let assign11850_e11161: f64 = (assign11850_e11149 * assign11850_e11160);
        let assign11850_e11162: f64 = (1.0 - assign11850_e11161);
        let assign11850_e11163: f64 = (0.1666666666667 * assign11850_e11162);
        (assign11850_e11163, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11850_e11165;
        locals.var_q_temp1_dn4 = assign11850_e11165_d_n4;
        locals.var_q_temp1_dn6 = assign11850_e11165_d_n6;
        locals.var_q_temp1_dn7 = assign11850_e11165_d_n7;
        locals.var_q_temp1_dn8 = assign11850_e11165_d_n8;
        locals.var_q_temp1_dn9 = assign11850_e11165_d_n9;

        let (assign11860_e11175, assign11860_e11175_d_n4, assign11860_e11175_d_n6, assign11860_e11175_d_n7, assign11860_e11175_d_n8, assign11860_e11175_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11860_e11173: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign11860_e11173, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign11860_e11175;
        locals.var_q_d1_qcoth_dn4 = assign11860_e11175_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign11860_e11175_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign11860_e11175_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign11860_e11175_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign11860_e11175_d_n9;

        let (assign11870_e11201, assign11870_e11201_d_n4, assign11870_e11201_d_n6, assign11870_e11201_d_n7, assign11870_e11201_d_n8, assign11870_e11201_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11870_e11185: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign11870_e11189: f64 = (0.05 * locals.var_q_qsq);
        let assign11870_e11193: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign11870_e11194: f64 = (1.0 - assign11870_e11193);
        let assign11870_e11195: f64 = (assign11870_e11189 * assign11870_e11194);
        let assign11870_e11196: f64 = (1.0 - assign11870_e11195);
        let assign11870_e11197: f64 = (assign11870_e11185 * assign11870_e11196);
        let assign11870_e11198: f64 = (1.0 - assign11870_e11197);
        let assign11870_e11199: f64 = (0.0055555555556 * assign11870_e11198);
        (assign11870_e11199, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn4) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn6) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn7) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn8) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn9) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign11870_e11201;
        locals.var_q_temp2_dn4 = assign11870_e11201_d_n4;
        locals.var_q_temp2_dn6 = assign11870_e11201_d_n6;
        locals.var_q_temp2_dn7 = assign11870_e11201_d_n7;
        locals.var_q_temp2_dn8 = assign11870_e11201_d_n8;
        locals.var_q_temp2_dn9 = assign11870_e11201_d_n9;

        let (assign11880_e11217, assign11880_e11217_d_n4, assign11880_e11217_d_n6, assign11880_e11217_d_n7, assign11880_e11217_d_n8, assign11880_e11217_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11880_e11209: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign11880_e11212: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign11880_e11214: f64 = (assign11880_e11212 * locals.var_q_temp2);
        let assign11880_e11215: f64 = (assign11880_e11209 - assign11880_e11214);
        (assign11880_e11215, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign11880_e11217;
        locals.var_q_d2_qcoth_dn4 = assign11880_e11217_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign11880_e11217_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign11880_e11217_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign11880_e11217_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign11880_e11217_d_n9;

        let (assign11890_e11230, assign11890_e11230_d_n4, assign11890_e11230_d_n6, assign11890_e11230_d_n7, assign11890_e11230_d_n8, assign11890_e11230_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11890_e11224: f64 = (-0.5);
        let assign11890_e11226: f64 = (assign11890_e11224 * locals.var_q_d1_qsq);
        let assign11890_e11228: f64 = (assign11890_e11226 * locals.var_q_temp3);
        (assign11890_e11228, (((assign11890_e11224 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn4)), (((assign11890_e11224 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn6)), (((assign11890_e11224 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn7)), (((assign11890_e11224 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn8)), (((assign11890_e11224 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign11890_e11230;
        locals.var_q_d1_ln_dn4 = assign11890_e11230_d_n4;
        locals.var_q_d1_ln_dn6 = assign11890_e11230_d_n6;
        locals.var_q_d1_ln_dn7 = assign11890_e11230_d_n7;
        locals.var_q_d1_ln_dn8 = assign11890_e11230_d_n8;
        locals.var_q_d1_ln_dn9 = assign11890_e11230_d_n9;

        let (assign11900_e11263, assign11900_e11263_d_n4, assign11900_e11263_d_n6, assign11900_e11263_d_n7, assign11900_e11263_d_n8, assign11900_e11263_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11900_e11237: f64 = (-0.5);
        let assign11900_e11239: f64 = (assign11900_e11237 * locals.var_q_d2_qsq);
        let assign11900_e11241: f64 = (assign11900_e11239 * locals.var_q_temp3);
        let assign11900_e11244: f64 = (0.25 * 0.0055555555556);
        let assign11900_e11246: f64 = (assign11900_e11244 * locals.var_q_d1_qsq);
        let assign11900_e11248: f64 = (assign11900_e11246 * locals.var_q_d1_qsq);
        let assign11900_e11252: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign11900_e11256: f64 = (0.075 * locals.var_q_qsq);
        let assign11900_e11257: f64 = (2.0 - assign11900_e11256);
        let assign11900_e11258: f64 = (assign11900_e11252 * assign11900_e11257);
        let assign11900_e11259: f64 = (1.0 - assign11900_e11258);
        let assign11900_e11260: f64 = (assign11900_e11248 * assign11900_e11259);
        let assign11900_e11261: f64 = (assign11900_e11241 + assign11900_e11260);
        (assign11900_e11261, ((((assign11900_e11237 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn4)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn4)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign11900_e11237 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn6)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn6)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign11900_e11237 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn7)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn7)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign11900_e11237 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn8)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn8)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign11900_e11237 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn9)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn9)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign11900_e11263;
        locals.var_q_d2_ln_dn4 = assign11900_e11263_d_n4;
        locals.var_q_d2_ln_dn6 = assign11900_e11263_d_n6;
        locals.var_q_d2_ln_dn7 = assign11900_e11263_d_n7;
        locals.var_q_d2_ln_dn8 = assign11900_e11263_d_n8;
        locals.var_q_d2_ln_dn9 = assign11900_e11263_d_n9;

        let assign11910_e11266: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard555 = assign11910_e11266;

        let (assign11920_e11280, assign11920_e11280_d_n4, assign11920_e11280_d_n6, assign11920_e11280_d_n7, assign11920_e11280_d_n8, assign11920_e11280_d_n9,) = {
    if (locals.var_guard555 != 0.0) {
        let assign11920_e11270: f64 = (4.0 * locals.var_q_qsq);
        let assign11920_e11275: f64 = (2.0 - locals.var_q_invexpq);
        let assign11920_e11276: f64 = (locals.var_q_invexpq * assign11920_e11275);
        let assign11920_e11277: f64 = (1.0 - assign11920_e11276);
        let assign11920_e11278: f64 = (assign11920_e11270 / assign11920_e11277);
        (assign11920_e11278, ((((4.0 * locals.var_q_qsq_dn4) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn4 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign11920_e11277 * assign11920_e11277)), ((((4.0 * locals.var_q_qsq_dn6) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn6 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign11920_e11277 * assign11920_e11277)), ((((4.0 * locals.var_q_qsq_dn7) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn7 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign11920_e11277 * assign11920_e11277)), ((((4.0 * locals.var_q_qsq_dn8) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn8 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign11920_e11277 * assign11920_e11277)), ((((4.0 * locals.var_q_qsq_dn9) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn9 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign11920_e11277 * assign11920_e11277)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign11920_e11280;
        locals.var_q_temp2_dn4 = assign11920_e11280_d_n4;
        locals.var_q_temp2_dn6 = assign11920_e11280_d_n6;
        locals.var_q_temp2_dn7 = assign11920_e11280_d_n7;
        locals.var_q_temp2_dn8 = assign11920_e11280_d_n8;
        locals.var_q_temp2_dn9 = assign11920_e11280_d_n9;

        let (assign11930_e11286, assign11930_e11286_d_n4, assign11930_e11286_d_n6, assign11930_e11286_d_n7, assign11930_e11286_d_n8, assign11930_e11286_d_n9,) = {
    if (locals.var_guard555 != 0.0) {
        let assign11930_e11284: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign11930_e11284, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign11930_e11286;
        locals.var_q_sh_term_dn4 = assign11930_e11286_d_n4;
        locals.var_q_sh_term_dn6 = assign11930_e11286_d_n6;
        locals.var_q_sh_term_dn7 = assign11930_e11286_d_n7;
        locals.var_q_sh_term_dn8 = assign11930_e11286_d_n8;
        locals.var_q_sh_term_dn9 = assign11930_e11286_d_n9;

        let (assign11940_e11293, assign11940_e11293_d_n4, assign11940_e11293_d_n6, assign11940_e11293_d_n7, assign11940_e11293_d_n8, assign11940_e11293_d_n9,) = {
    if (locals.var_guard555 != 0.0) {
        let assign11940_e11289: f64 = (locals.var_q_temp2).ln();
        let assign11940_e11291: f64 = (assign11940_e11289 - locals.var_q_rac_qsq);
        (assign11940_e11291, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign11940_e11293;
        locals.var_q_ln_term_dn4 = assign11940_e11293_d_n4;
        locals.var_q_ln_term_dn6 = assign11940_e11293_d_n6;
        locals.var_q_ln_term_dn7 = assign11940_e11293_d_n7;
        locals.var_q_ln_term_dn8 = assign11940_e11293_d_n8;
        locals.var_q_ln_term_dn9 = assign11940_e11293_d_n9;

        let assign11950_e11296: f64 = (-0.005);
        let assign11950_e11297: f64 = if locals.var_q_qsq < assign11950_e11296 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign11950_e11297;

        let (assign11960_e11307, assign11960_e11307_d_n4, assign11960_e11307_d_n6, assign11960_e11307_d_n7, assign11960_e11307_d_n8, assign11960_e11307_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
        let assign11960_e11304: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign11960_e11305: f64 = (assign11960_e11304).sin();
        (assign11960_e11305, ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign11960_e11307;
        locals.var_q_temp2_dn4 = assign11960_e11307_d_n4;
        locals.var_q_temp2_dn6 = assign11960_e11307_d_n6;
        locals.var_q_temp2_dn7 = assign11960_e11307_d_n7;
        locals.var_q_temp2_dn8 = assign11960_e11307_d_n8;
        locals.var_q_temp2_dn9 = assign11960_e11307_d_n9;

        let (assign11970_e11319, assign11970_e11319_d_n4, assign11970_e11319_d_n6, assign11970_e11319_d_n7, assign11970_e11319_d_n8, assign11970_e11319_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
        let assign11970_e11313: f64 = (-locals.var_q_qsq);
        let assign11970_e11316: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign11970_e11317: f64 = (assign11970_e11313 / assign11970_e11316);
        (assign11970_e11317, ((((-locals.var_q_qsq_dn4) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign11970_e11316 * assign11970_e11316)), ((((-locals.var_q_qsq_dn6) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign11970_e11316 * assign11970_e11316)), ((((-locals.var_q_qsq_dn7) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign11970_e11316 * assign11970_e11316)), ((((-locals.var_q_qsq_dn8) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign11970_e11316 * assign11970_e11316)), ((((-locals.var_q_qsq_dn9) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign11970_e11316 * assign11970_e11316)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign11970_e11319;
        locals.var_q_sh_term_dn4 = assign11970_e11319_d_n4;
        locals.var_q_sh_term_dn6 = assign11970_e11319_d_n6;
        locals.var_q_sh_term_dn7 = assign11970_e11319_d_n7;
        locals.var_q_sh_term_dn8 = assign11970_e11319_d_n8;
        locals.var_q_sh_term_dn9 = assign11970_e11319_d_n9;

        let (assign11980_e11327, assign11980_e11327_d_n4, assign11980_e11327_d_n6, assign11980_e11327_d_n7, assign11980_e11327_d_n8, assign11980_e11327_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
        let assign11980_e11325: f64 = (locals.var_q_sh_term).ln();
        (assign11980_e11325, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign11980_e11327;
        locals.var_q_ln_term_dn4 = assign11980_e11327_d_n4;
        locals.var_q_ln_term_dn6 = assign11980_e11327_d_n6;
        locals.var_q_ln_term_dn7 = assign11980_e11327_d_n7;
        locals.var_q_ln_term_dn8 = assign11980_e11327_d_n8;
        locals.var_q_ln_term_dn9 = assign11980_e11327_d_n9;

        let (assign11990_e11351, assign11990_e11351_d_n4, assign11990_e11351_d_n6, assign11990_e11351_d_n7, assign11990_e11351_d_n8, assign11990_e11351_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 == 0.0)) {
        let assign11990_e11336: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign11990_e11340: f64 = (0.05 * locals.var_q_qsq);
        let assign11990_e11344: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign11990_e11345: f64 = (1.0 - assign11990_e11344);
        let assign11990_e11346: f64 = (assign11990_e11340 * assign11990_e11345);
        let assign11990_e11347: f64 = (1.0 - assign11990_e11346);
        let assign11990_e11348: f64 = (assign11990_e11336 * assign11990_e11347);
        let assign11990_e11349: f64 = (4.0 - assign11990_e11348);
        (assign11990_e11349, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn4) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn6) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn7) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn8) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn9) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign11990_e11351;
        locals.var_q_sh_term_dn4 = assign11990_e11351_d_n4;
        locals.var_q_sh_term_dn6 = assign11990_e11351_d_n6;
        locals.var_q_sh_term_dn7 = assign11990_e11351_d_n7;
        locals.var_q_sh_term_dn8 = assign11990_e11351_d_n8;
        locals.var_q_sh_term_dn9 = assign11990_e11351_d_n9;

        let (assign12000_e11360, assign12000_e11360_d_n4, assign12000_e11360_d_n6, assign12000_e11360_d_n7, assign12000_e11360_d_n8, assign12000_e11360_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 == 0.0)) {
        let assign12000_e11358: f64 = (locals.var_q_sh_term).ln();
        (assign12000_e11358, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign12000_e11360;
        locals.var_q_ln_term_dn4 = assign12000_e11360_d_n4;
        locals.var_q_ln_term_dn6 = assign12000_e11360_d_n6;
        locals.var_q_ln_term_dn7 = assign12000_e11360_d_n7;
        locals.var_q_ln_term_dn8 = assign12000_e11360_d_n8;
        locals.var_q_ln_term_dn9 = assign12000_e11360_d_n9;

        let assign12010_e11363: f64 = (1.01 * locals.var_q_k1q1);
        let assign12010_e11365: f64 = (assign12010_e11363 + locals.var_q_qcoth);
        let assign12010_e11367: f64 = if assign12010_e11365 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign12010_e11367;

        let (assign12020_e11373, assign12020_e11373_d_n4, assign12020_e11373_d_n6, assign12020_e11373_d_n7, assign12020_e11373_d_n8, assign12020_e11373_d_n9,) = {
    if (locals.var_guard557 != 0.0) {
        let assign12020_e11371: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign12020_e11371, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign12020_e11373;
        locals.var_q_expnum_dn4 = assign12020_e11373_d_n4;
        locals.var_q_expnum_dn6 = assign12020_e11373_d_n6;
        locals.var_q_expnum_dn7 = assign12020_e11373_d_n7;
        locals.var_q_expnum_dn8 = assign12020_e11373_d_n8;
        locals.var_q_expnum_dn9 = assign12020_e11373_d_n9;

        let (assign12030_e11379, assign12030_e11379_d_n4, assign12030_e11379_d_n6, assign12030_e11379_d_n7, assign12030_e11379_d_n8, assign12030_e11379_d_n9,) = {
    if (locals.var_guard557 != 0.0) {
        let assign12030_e11377: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign12030_e11377, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign12030_e11379;
        locals.var_q_d1_expnum_dn4 = assign12030_e11379_d_n4;
        locals.var_q_d1_expnum_dn6 = assign12030_e11379_d_n6;
        locals.var_q_d1_expnum_dn7 = assign12030_e11379_d_n7;
        locals.var_q_d1_expnum_dn8 = assign12030_e11379_d_n8;
        locals.var_q_d1_expnum_dn9 = assign12030_e11379_d_n9;

        let (assign12040_e11383, assign12040_e11383_d_n4, assign12040_e11383_d_n6, assign12040_e11383_d_n7, assign12040_e11383_d_n8, assign12040_e11383_d_n9,) = {
    if (locals.var_guard557 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign12040_e11383;
        locals.var_q_d2_expnum_dn4 = assign12040_e11383_d_n4;
        locals.var_q_d2_expnum_dn6 = assign12040_e11383_d_n6;
        locals.var_q_d2_expnum_dn7 = assign12040_e11383_d_n7;
        locals.var_q_d2_expnum_dn8 = assign12040_e11383_d_n8;
        locals.var_q_d2_expnum_dn9 = assign12040_e11383_d_n9;

        let (assign12050_e11392, assign12050_e11392_d_n4, assign12050_e11392_d_n6, assign12050_e11392_d_n7, assign12050_e11392_d_n8, assign12050_e11392_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12050_e11389: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign12050_e11390: f64 = (1.0 / assign12050_e11389);
        (assign12050_e11390, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign12050_e11389 * assign12050_e11389))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign12050_e11389 * assign12050_e11389))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign12050_e11389 * assign12050_e11389))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign12050_e11389 * assign12050_e11389))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign12050_e11389 * assign12050_e11389))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign12050_e11392;
        locals.var_q_temp2_dn4 = assign12050_e11392_d_n4;
        locals.var_q_temp2_dn6 = assign12050_e11392_d_n6;
        locals.var_q_temp2_dn7 = assign12050_e11392_d_n7;
        locals.var_q_temp2_dn8 = assign12050_e11392_d_n8;
        locals.var_q_temp2_dn9 = assign12050_e11392_d_n9;

        let (assign12060_e11399, assign12060_e11399_d_n4, assign12060_e11399_d_n6, assign12060_e11399_d_n7, assign12060_e11399_d_n8, assign12060_e11399_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12060_e11397: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign12060_e11397, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign12060_e11399;
        locals.var_q_temp3_dn4 = assign12060_e11399_d_n4;
        locals.var_q_temp3_dn6 = assign12060_e11399_d_n6;
        locals.var_q_temp3_dn7 = assign12060_e11399_d_n7;
        locals.var_q_temp3_dn8 = assign12060_e11399_d_n8;
        locals.var_q_temp3_dn9 = assign12060_e11399_d_n9;

        let (assign12070_e11408, assign12070_e11408_d_n4, assign12070_e11408_d_n6, assign12070_e11408_d_n7, assign12070_e11408_d_n8, assign12070_e11408_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12070_e11404: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign12070_e11406: f64 = (assign12070_e11404 * locals.var_q_temp2);
        (assign12070_e11406, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign12070_e11408;
        locals.var_q_expnum_dn4 = assign12070_e11408_d_n4;
        locals.var_q_expnum_dn6 = assign12070_e11408_d_n6;
        locals.var_q_expnum_dn7 = assign12070_e11408_d_n7;
        locals.var_q_expnum_dn8 = assign12070_e11408_d_n8;
        locals.var_q_expnum_dn9 = assign12070_e11408_d_n9;

        let (assign12080_e11423, assign12080_e11423_d_n4, assign12080_e11423_d_n6, assign12080_e11423_d_n7, assign12080_e11423_d_n8, assign12080_e11423_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12080_e11413: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign12080_e11415: f64 = (assign12080_e11413 - locals.var_q_aexp);
        let assign12080_e11418: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign12080_e11419: f64 = (assign12080_e11415 - assign12080_e11418);
        let assign12080_e11421: f64 = (assign12080_e11419 * locals.var_q_temp2);
        (assign12080_e11421, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign12080_e11423;
        locals.var_q_d1_expnum_dn4 = assign12080_e11423_d_n4;
        locals.var_q_d1_expnum_dn6 = assign12080_e11423_d_n6;
        locals.var_q_d1_expnum_dn7 = assign12080_e11423_d_n7;
        locals.var_q_d1_expnum_dn8 = assign12080_e11423_d_n8;
        locals.var_q_d1_expnum_dn9 = assign12080_e11423_d_n9;

        let (assign12090_e11448, assign12090_e11448_d_n4, assign12090_e11448_d_n6, assign12090_e11448_d_n7, assign12090_e11448_d_n8, assign12090_e11448_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12090_e11428: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign12090_e11431: f64 = (2.0 * locals.var_q_temp3);
        let assign12090_e11433: f64 = (assign12090_e11431 * locals.var_q_d1_expnum);
        let assign12090_e11434: f64 = (assign12090_e11428 + assign12090_e11433);
        let assign12090_e11436: f64 = (assign12090_e11434 + locals.var_q_aexp);
        let assign12090_e11440: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign12090_e11441: f64 = (locals.var_q_d2_ln + assign12090_e11440);
        let assign12090_e11443: f64 = (assign12090_e11441 * locals.var_q_sh_term);
        let assign12090_e11444: f64 = (assign12090_e11436 - assign12090_e11443);
        let assign12090_e11446: f64 = (assign12090_e11444 * locals.var_q_temp2);
        (assign12090_e11446, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign12090_e11448;
        locals.var_q_d2_expnum_dn4 = assign12090_e11448_d_n4;
        locals.var_q_d2_expnum_dn6 = assign12090_e11448_d_n6;
        locals.var_q_d2_expnum_dn7 = assign12090_e11448_d_n7;
        locals.var_q_d2_expnum_dn8 = assign12090_e11448_d_n8;
        locals.var_q_d2_expnum_dn9 = assign12090_e11448_d_n9;

        let assign12100_e11451: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard558 = assign12100_e11451;

    }

    pub(super) fn stamp_transient_block_28(
        locals: &mut StampLocals,
    ) {
        let (assign12110_e11456, assign12110_e11456_d_n4, assign12110_e11456_d_n6, assign12110_e11456_d_n7, assign12110_e11456_d_n8, assign12110_e11456_d_n9,) = {
    if (locals.var_guard558 != 0.0) {
        let assign12110_e11454: f64 = (locals.var_q_expnum).ln();
        (assign12110_e11454, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign12110_e11456;
        locals.var_q_lnexpnum_dn4 = assign12110_e11456_d_n4;
        locals.var_q_lnexpnum_dn6 = assign12110_e11456_d_n6;
        locals.var_q_lnexpnum_dn7 = assign12110_e11456_d_n7;
        locals.var_q_lnexpnum_dn8 = assign12110_e11456_d_n8;
        locals.var_q_lnexpnum_dn9 = assign12110_e11456_d_n9;

        let (assign12120_e11462, assign12120_e11462_d_n4, assign12120_e11462_d_n6, assign12120_e11462_d_n7, assign12120_e11462_d_n8, assign12120_e11462_d_n9,) = {
    if (locals.var_guard558 != 0.0) {
        let assign12120_e11460: f64 = (1.0 / locals.var_q_expnum);
        (assign12120_e11460, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12120_e11462;
        locals.var_q_temp1_dn4 = assign12120_e11462_d_n4;
        locals.var_q_temp1_dn6 = assign12120_e11462_d_n6;
        locals.var_q_temp1_dn7 = assign12120_e11462_d_n7;
        locals.var_q_temp1_dn8 = assign12120_e11462_d_n8;
        locals.var_q_temp1_dn9 = assign12120_e11462_d_n9;

        let (assign12130_e11468, assign12130_e11468_d_n4, assign12130_e11468_d_n6, assign12130_e11468_d_n7, assign12130_e11468_d_n8, assign12130_e11468_d_n9,) = {
    if (locals.var_guard558 != 0.0) {
        let assign12130_e11466: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign12130_e11466, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign12130_e11468;
        locals.var_q_d1_lnexpnum_dn4 = assign12130_e11468_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign12130_e11468_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign12130_e11468_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign12130_e11468_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign12130_e11468_d_n9;

        let (assign12140_e11478, assign12140_e11478_d_n4, assign12140_e11478_d_n6, assign12140_e11478_d_n7, assign12140_e11478_d_n8, assign12140_e11478_d_n9,) = {
    if (locals.var_guard558 != 0.0) {
        let assign12140_e11472: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign12140_e11475: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign12140_e11476: f64 = (assign12140_e11472 - assign12140_e11475);
        (assign12140_e11476, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign12140_e11478;
        locals.var_q_d2_lnexpnum_dn4 = assign12140_e11478_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign12140_e11478_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign12140_e11478_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign12140_e11478_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign12140_e11478_d_n9;

        let (assign12150_e11489, assign12150_e11489_d_n4, assign12150_e11489_d_n6, assign12150_e11489_d_n7, assign12150_e11489_d_n8, assign12150_e11489_d_n9,) = {
    if (locals.var_guard558 == 0.0) {
        let assign12150_e11483: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign12150_e11485: f64 = (-locals.var_q_k1q1);
        let assign12150_e11486: f64 = (assign12150_e11485).ln();
        let assign12150_e11487: f64 = (assign12150_e11483 + assign12150_e11486);
        (assign12150_e11487, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign12150_e11485)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign12150_e11485)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign12150_e11485)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign12150_e11485)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign12150_e11485)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign12150_e11489;
        locals.var_q_lnexpnum_dn4 = assign12150_e11489_d_n4;
        locals.var_q_lnexpnum_dn6 = assign12150_e11489_d_n6;
        locals.var_q_lnexpnum_dn7 = assign12150_e11489_d_n7;
        locals.var_q_lnexpnum_dn8 = assign12150_e11489_d_n8;
        locals.var_q_lnexpnum_dn9 = assign12150_e11489_d_n9;

        let (assign12160_e11496, assign12160_e11496_d_n4, assign12160_e11496_d_n6, assign12160_e11496_d_n7, assign12160_e11496_d_n8, assign12160_e11496_d_n9,) = {
    if (locals.var_guard558 == 0.0) {
        let assign12160_e11494: f64 = (1.0 / locals.var_q1s);
        (assign12160_e11494, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12160_e11496;
        locals.var_q_temp1_dn4 = assign12160_e11496_d_n4;
        locals.var_q_temp1_dn6 = assign12160_e11496_d_n6;
        locals.var_q_temp1_dn7 = assign12160_e11496_d_n7;
        locals.var_q_temp1_dn8 = assign12160_e11496_d_n8;
        locals.var_q_temp1_dn9 = assign12160_e11496_d_n9;

        let (assign12170_e11503, assign12170_e11503_d_n4, assign12170_e11503_d_n6, assign12170_e11503_d_n7, assign12170_e11503_d_n8, assign12170_e11503_d_n9,) = {
    if (locals.var_guard558 == 0.0) {
        let assign12170_e11501: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign12170_e11501, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign12170_e11503;
        locals.var_q_d1_lnexpnum_dn4 = assign12170_e11503_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign12170_e11503_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign12170_e11503_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign12170_e11503_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign12170_e11503_d_n9;

        let (assign12180_e11511, assign12180_e11511_d_n4, assign12180_e11511_d_n6, assign12180_e11511_d_n7, assign12180_e11511_d_n8, assign12180_e11511_d_n9,) = {
    if (locals.var_guard558 == 0.0) {
        let assign12180_e11507: f64 = (-locals.var_q_temp1);
        let assign12180_e11509: f64 = (assign12180_e11507 * locals.var_q_temp1);
        (assign12180_e11509, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign12180_e11511;
        locals.var_q_d2_lnexpnum_dn4 = assign12180_e11511_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign12180_e11511_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign12180_e11511_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign12180_e11511_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign12180_e11511_d_n9;

        let assign12190_e11514: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign12190_e11516: f64 = (assign12190_e11514 + locals.var_q1s);
        let assign12190_e11519: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign12190_e11520: f64 = (assign12190_e11516 + assign12190_e11519);
        let assign12190_e11522: f64 = (assign12190_e11520 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign12190_e11522;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign12200_e11526: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign12200_e11527: f64 = (1.0 + assign12200_e11526);
        let assign12200_e11529: f64 = (assign12200_e11527 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign12200_e11529;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign12210_e11532: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign12210_e11534: f64 = (assign12210_e11532 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign12210_e11534;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign12220_e11538: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign12220_e11539: f64 = (locals.var_q_k1q1 + assign12220_e11538);
        locals.var_q_qi_int = assign12220_e11539;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign12230_e11543: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign12230_e11544: f64 = (locals.var_k1 + assign12230_e11543);
        locals.var_q_d1_qi = assign12230_e11544;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign12240_e11547: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign12240_e11547;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign12250_e11550: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign12250_e11552: f64 = (assign12250_e11550 - locals.var_q_aexp);
        locals.var_q_zero = assign12250_e11552;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign12260_e11555: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign12260_e11558: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign12260_e11559: f64 = (assign12260_e11555 + assign12260_e11558);
        let assign12260_e11561: f64 = (assign12260_e11559 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign12260_e11561;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign12270_e11564: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign12270_e11567: f64 = (2.0 * locals.var_q_d1_qi);
        let assign12270_e11569: f64 = (assign12270_e11567 * locals.var_q_d1_expnum);
        let assign12270_e11570: f64 = (assign12270_e11564 + assign12270_e11569);
        let assign12270_e11573: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign12270_e11574: f64 = (assign12270_e11570 + assign12270_e11573);
        let assign12270_e11576: f64 = (assign12270_e11574 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign12270_e11576;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign12280_e11579: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign12280_e11582: f64 = (0.5 * locals.var_q_zero);
        let assign12280_e11584: f64 = (assign12280_e11582 * locals.var_q_d2_zero);
        let assign12280_e11585: f64 = (assign12280_e11579 - assign12280_e11584);
        locals.var_q_temp = assign12280_e11585;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn9)));

        let assign12290_e11587: f64 = (-locals.var_q_zero);
        let assign12290_e11589: f64 = (assign12290_e11587 * locals.var_q_d1_zero);
        let assign12290_e11591: f64 = (assign12290_e11589 * locals.var_q_temp);
        let assign12290_e11594: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign12290_e11596: f64 = (assign12290_e11594 + 1e-200);
        let assign12290_e11597: f64 = (assign12290_e11591 / assign12290_e11596);
        locals.var_q_eps2 = assign12290_e11597;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn4)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign12290_e11596 * assign12290_e11596));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn6)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign12290_e11596 * assign12290_e11596));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn7)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign12290_e11596 * assign12290_e11596));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn8)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign12290_e11596 * assign12290_e11596));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn9)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign12290_e11596 * assign12290_e11596));

        let assign12300_e11600: f64 = (locals.var_q1s + locals.var_q_eps2);
        locals.var_q1s = assign12300_e11600;
        locals.var_q1s_dn4 = (locals.var_q1s_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1s_dn6 = (locals.var_q1s_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1s_dn7 = (locals.var_q1s_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1s_dn8 = (locals.var_q1s_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1s_dn9 = (locals.var_q1s_dn9 + locals.var_q_eps2_dn9);

        let assign12310_e11603: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign12310_e11603;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign12320_e11606: f64 = (locals.var_k2 * locals.var_q2s);
        locals.var_q_k2q2 = assign12320_e11606;
        locals.var_q_k2q2_dn4 = ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4));
        locals.var_q_k2q2_dn6 = ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6));
        locals.var_q_k2q2_dn7 = ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7));
        locals.var_q_k2q2_dn8 = ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8));
        locals.var_q_k2q2_dn9 = ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9));

        let assign12330_e11609: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        locals.var_q_qi_int = assign12330_e11609;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4);
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6);
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7);
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8);
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9);

        let assign12340_e11613: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign12340_e11614: f64 = (1.0 + assign12340_e11613);
        locals.var_q_a = assign12340_e11614;
        locals.var_q_a_dn4 = (0.065345483024 * locals.var_q_qi_int_dn4);
        locals.var_q_a_dn6 = (0.065345483024 * locals.var_q_qi_int_dn6);
        locals.var_q_a_dn7 = (0.065345483024 * locals.var_q_qi_int_dn7);
        locals.var_q_a_dn8 = (0.065345483024 * locals.var_q_qi_int_dn8);
        locals.var_q_a_dn9 = (0.065345483024 * locals.var_q_qi_int_dn9);

        let assign12350_e11618: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign12350_e11619: f64 = (39.478417604 + assign12350_e11618);
        let assign12350_e11622: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12350_e11623: f64 = (assign12350_e11619 + assign12350_e11622);
        locals.var_q_b = assign12350_e11623;
        locals.var_q_b_dn4 = ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)));
        locals.var_q_b_dn6 = ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)));
        locals.var_q_b_dn7 = ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)));
        locals.var_q_b_dn8 = ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)));
        locals.var_q_b_dn9 = ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)));

        let assign12360_e11627: f64 = (2.0 * locals.var_q_qi_int);
        let assign12360_e11630: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12360_e11631: f64 = (assign12360_e11627 + assign12360_e11630);
        let assign12360_e11632: f64 = (39.478417604 * assign12360_e11631);
        locals.var_q_c = assign12360_e11632;
        locals.var_q_c_dn4 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))));
        locals.var_q_c_dn6 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))));
        locals.var_q_c_dn7 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))));
        locals.var_q_c_dn8 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))));
        locals.var_q_c_dn9 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))));

        let assign12370_e11635: f64 = (locals.var_q_b * locals.var_q_b);
        let assign12370_e11638: f64 = (4.0 * locals.var_q_a);
        let assign12370_e11640: f64 = (assign12370_e11638 * locals.var_q_c);
        let assign12370_e11641: f64 = (assign12370_e11635 - assign12370_e11640);
        let assign12370_e11642: f64 = (assign12370_e11641).sqrt();
        locals.var_q_disc = assign12370_e11642;
        locals.var_q_disc_dn4 = ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn4))) / (2.0 * assign12370_e11642));
        locals.var_q_disc_dn6 = ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn6))) / (2.0 * assign12370_e11642));
        locals.var_q_disc_dn7 = ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn7))) / (2.0 * assign12370_e11642));
        locals.var_q_disc_dn8 = ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn8))) / (2.0 * assign12370_e11642));
        locals.var_q_disc_dn9 = ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn9))) / (2.0 * assign12370_e11642));

        let assign12380_e11645: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign12380_e11648: f64 = (2.0 * locals.var_q_a);
        let assign12380_e11649: f64 = (assign12380_e11645 / assign12380_e11648);
        locals.var_q_qsq = assign12380_e11649;
        locals.var_q_qsq_dn4 = ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn4))) / (assign12380_e11648 * assign12380_e11648));
        locals.var_q_qsq_dn6 = ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn6))) / (assign12380_e11648 * assign12380_e11648));
        locals.var_q_qsq_dn7 = ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn7))) / (assign12380_e11648 * assign12380_e11648));
        locals.var_q_qsq_dn8 = ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn8))) / (assign12380_e11648 * assign12380_e11648));
        locals.var_q_qsq_dn9 = ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn9))) / (assign12380_e11648 * assign12380_e11648));

        let assign12390_e11652: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12390_e11654: f64 = (assign12390_e11652 - locals.var_q_qsq);
        locals.var_q_delta = assign12390_e11654;
        locals.var_q_delta_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4);
        locals.var_q_delta_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6);
        locals.var_q_delta_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7);
        locals.var_q_delta_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8);
        locals.var_q_delta_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9);

        let assign12400_e11657: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign12400_e11657;

        let (assign12410_e11672, assign12410_e11672_d_n4, assign12410_e11672_d_n6, assign12410_e11672_d_n7, assign12410_e11672_d_n8, assign12410_e11672_d_n9,) = {
    if (locals.var_guard559 != 0.0) {
        let assign12410_e11662: f64 = (locals.var_q_delta / locals.var_a0);
        let assign12410_e11663: f64 = (assign12410_e11662).ln();
        let assign12410_e11665: f64 = assign12410_e11663;
        let assign12410_e11667: f64 = (assign12410_e11665 - locals.var_xg1x);
        let assign12410_e11669: f64 = (assign12410_e11667 + locals.var_q1s);
        let assign12410_e11670: f64 = (locals.var_q_delta * assign12410_e11669);
        (assign12410_e11670, ((locals.var_q_delta_dn4 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn4) + locals.var_q1s_dn4))), ((locals.var_q_delta_dn6 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn6) + locals.var_q1s_dn6))), ((locals.var_q_delta_dn7 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn7) + locals.var_q1s_dn7))), ((locals.var_q_delta_dn8 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn8) + locals.var_q1s_dn8))), ((locals.var_q_delta_dn9 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn9) + locals.var_q1s_dn9))),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign12410_e11672;
        locals.var_q_zero_dn4 = assign12410_e11672_d_n4;
        locals.var_q_zero_dn6 = assign12410_e11672_d_n6;
        locals.var_q_zero_dn7 = assign12410_e11672_d_n7;
        locals.var_q_zero_dn8 = assign12410_e11672_d_n8;
        locals.var_q_zero_dn9 = assign12410_e11672_d_n9;

        let (assign12420_e11682, assign12420_e11682_d_n4, assign12420_e11682_d_n6, assign12420_e11682_d_n7, assign12420_e11682_d_n8, assign12420_e11682_d_n9,) = {
    if (locals.var_guard559 != 0.0) {
        let assign12420_e11676: f64 = (2.0 * locals.var_k1);
        let assign12420_e11678: f64 = (assign12420_e11676 * locals.var_q_k1q1);
        let assign12420_e11680: f64 = (assign12420_e11678 + locals.var_q_delta);
        (assign12420_e11680, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign12420_e11682;
        locals.var_q_d1_zero_dn4 = assign12420_e11682_d_n4;
        locals.var_q_d1_zero_dn6 = assign12420_e11682_d_n6;
        locals.var_q_d1_zero_dn7 = assign12420_e11682_d_n7;
        locals.var_q_d1_zero_dn8 = assign12420_e11682_d_n8;
        locals.var_q_d1_zero_dn9 = assign12420_e11682_d_n9;

        let (assign12430_e11690,) = {
    if (locals.var_guard559 != 0.0) {
        let assign12430_e11686: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12430_e11688: f64 = (assign12430_e11686 - locals.var_q_x1sat);
        (assign12430_e11688,)
    } else {
        (locals.var_q_dx1,)
    }
};
        locals.var_q_dx1 = assign12430_e11690;

        let assign12440_e11700: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign12440_e11702: f64 = (locals.var_k1).ln();
        let assign12440_e11703: f64 = (assign12440_e11700 + assign12440_e11702);
        let assign12440_e11710: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign12440_e11703 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard560 = assign12440_e11710;

        let (assign12450_e11720, assign12450_e11720_d_n4, assign12450_e11720_d_n6, assign12450_e11720_d_n7, assign12450_e11720_d_n8, assign12450_e11720_d_n9,) = {
    if ((locals.var_guard559 != 0.0) && (locals.var_guard560 != 0.0)) {
        let assign12450_e11717: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
        let assign12450_e11718: f64 = (locals.var_q1s - assign12450_e11717);
        (assign12450_e11718, (locals.var_q1s_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))),)
    } else {
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9,)
    }
};
        locals.var_q1s = assign12450_e11720;
        locals.var_q1s_dn4 = assign12450_e11720_d_n4;
        locals.var_q1s_dn6 = assign12450_e11720_d_n6;
        locals.var_q1s_dn7 = assign12450_e11720_d_n7;
        locals.var_q1s_dn8 = assign12450_e11720_d_n8;
        locals.var_q1s_dn9 = assign12450_e11720_d_n9;

        let assign12460_e11723: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign12460_e11723;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign12470_e11726: f64 = (locals.var_k2 * locals.var_q2s);
        locals.var_q_k2q2 = assign12470_e11726;
        locals.var_q_k2q2_dn4 = ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4));
        locals.var_q_k2q2_dn6 = ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6));
        locals.var_q_k2q2_dn7 = ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7));
        locals.var_q_k2q2_dn8 = ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8));
        locals.var_q_k2q2_dn9 = ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9));

        let assign12480_e11729: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        locals.var_q_qi_int = assign12480_e11729;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4);
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6);
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7);
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8);
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9);

        let assign12490_e11733: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign12490_e11734: f64 = (1.0 + assign12490_e11733);
        locals.var_q_a = assign12490_e11734;
        locals.var_q_a_dn4 = (0.065345483024 * locals.var_q_qi_int_dn4);
        locals.var_q_a_dn6 = (0.065345483024 * locals.var_q_qi_int_dn6);
        locals.var_q_a_dn7 = (0.065345483024 * locals.var_q_qi_int_dn7);
        locals.var_q_a_dn8 = (0.065345483024 * locals.var_q_qi_int_dn8);
        locals.var_q_a_dn9 = (0.065345483024 * locals.var_q_qi_int_dn9);

        let assign12500_e11738: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign12500_e11739: f64 = (39.478417604 + assign12500_e11738);
        let assign12500_e11742: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12500_e11743: f64 = (assign12500_e11739 + assign12500_e11742);
        locals.var_q_b = assign12500_e11743;
        locals.var_q_b_dn4 = ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)));
        locals.var_q_b_dn6 = ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)));
        locals.var_q_b_dn7 = ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)));
        locals.var_q_b_dn8 = ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)));
        locals.var_q_b_dn9 = ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)));

        let assign12510_e11747: f64 = (2.0 * locals.var_q_qi_int);
        let assign12510_e11750: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12510_e11751: f64 = (assign12510_e11747 + assign12510_e11750);
        let assign12510_e11752: f64 = (39.478417604 * assign12510_e11751);
        locals.var_q_c = assign12510_e11752;
        locals.var_q_c_dn4 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))));
        locals.var_q_c_dn6 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))));
        locals.var_q_c_dn7 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))));
        locals.var_q_c_dn8 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))));
        locals.var_q_c_dn9 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))));

        let assign12520_e11755: f64 = (locals.var_q_b * locals.var_q_b);
        let assign12520_e11758: f64 = (4.0 * locals.var_q_a);
        let assign12520_e11760: f64 = (assign12520_e11758 * locals.var_q_c);
        let assign12520_e11761: f64 = (assign12520_e11755 - assign12520_e11760);
        let assign12520_e11762: f64 = (assign12520_e11761).sqrt();
        locals.var_q_disc = assign12520_e11762;
        locals.var_q_disc_dn4 = ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn4))) / (2.0 * assign12520_e11762));
        locals.var_q_disc_dn6 = ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn6))) / (2.0 * assign12520_e11762));
        locals.var_q_disc_dn7 = ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn7))) / (2.0 * assign12520_e11762));
        locals.var_q_disc_dn8 = ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn8))) / (2.0 * assign12520_e11762));
        locals.var_q_disc_dn9 = ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn9))) / (2.0 * assign12520_e11762));

        let assign12530_e11765: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign12530_e11768: f64 = (2.0 * locals.var_q_a);
        let assign12530_e11769: f64 = (assign12530_e11765 / assign12530_e11768);
        locals.var_q_qsq = assign12530_e11769;
        locals.var_q_qsq_dn4 = ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn4))) / (assign12530_e11768 * assign12530_e11768));
        locals.var_q_qsq_dn6 = ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn6))) / (assign12530_e11768 * assign12530_e11768));
        locals.var_q_qsq_dn7 = ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn7))) / (assign12530_e11768 * assign12530_e11768));
        locals.var_q_qsq_dn8 = ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn8))) / (assign12530_e11768 * assign12530_e11768));
        locals.var_q_qsq_dn9 = ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn9))) / (assign12530_e11768 * assign12530_e11768));

        let assign12540_e11772: f64 = (-0.005);
        let assign12540_e11773: f64 = if locals.var_q_qsq < assign12540_e11772 { 1.0 } else { 0.0 };
        locals.var_guard561 = assign12540_e11773;

        let (assign12550_e11779, assign12550_e11779_d_n4, assign12550_e11779_d_n6, assign12550_e11779_d_n7, assign12550_e11779_d_n8, assign12550_e11779_d_n9,) = {
    if (locals.var_guard561 != 0.0) {
        let assign12550_e11776: f64 = (locals.var_q_qsq).abs();
        let assign12550_e11777: f64 = (assign12550_e11776).sqrt();
        (assign12550_e11777, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12550_e11777)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12550_e11777)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12550_e11777)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12550_e11777)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12550_e11777)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign12550_e11779;
        locals.var_q_rac_qsq_dn4 = assign12550_e11779_d_n4;
        locals.var_q_rac_qsq_dn6 = assign12550_e11779_d_n6;
        locals.var_q_rac_qsq_dn7 = assign12550_e11779_d_n7;
        locals.var_q_rac_qsq_dn8 = assign12550_e11779_d_n8;
        locals.var_q_rac_qsq_dn9 = assign12550_e11779_d_n9;

    }

    pub(super) fn stamp_transient_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign12560_e11788, assign12560_e11788_d_n4, assign12560_e11788_d_n6, assign12560_e11788_d_n7, assign12560_e11788_d_n8, assign12560_e11788_d_n9,) = {
    if (locals.var_guard561 != 0.0) {
        let assign12560_e11784: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign12560_e11785: f64 = (assign12560_e11784).tan();
        let assign12560_e11786: f64 = (locals.var_q_rac_qsq / assign12560_e11785);
        (assign12560_e11786, (((locals.var_q_rac_qsq_dn4 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), (((locals.var_q_rac_qsq_dn6 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), (((locals.var_q_rac_qsq_dn7 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), (((locals.var_q_rac_qsq_dn8 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), (((locals.var_q_rac_qsq_dn9 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12560_e11788;
        locals.var_q_qcoth_dn4 = assign12560_e11788_d_n4;
        locals.var_q_qcoth_dn6 = assign12560_e11788_d_n6;
        locals.var_q_qcoth_dn7 = assign12560_e11788_d_n7;
        locals.var_q_qcoth_dn8 = assign12560_e11788_d_n8;
        locals.var_q_qcoth_dn9 = assign12560_e11788_d_n9;

        let (assign12570_e11802, assign12570_e11802_d_n4, assign12570_e11802_d_n6, assign12570_e11802_d_n7, assign12570_e11802_d_n8, assign12570_e11802_d_n9,) = {
    if (locals.var_guard561 != 0.0) {
        let assign12570_e11795: f64 = (2.0 - locals.var_q_qcoth);
        let assign12570_e11796: f64 = (locals.var_q_qcoth * assign12570_e11795);
        let assign12570_e11797: f64 = (locals.var_q_qsq + assign12570_e11796);
        let assign12570_e11798: f64 = (0.25 * assign12570_e11797);
        let assign12570_e11800: f64 = (assign12570_e11798 / locals.var_q_qsq);
        (assign12570_e11800, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12570_e11802;
        locals.var_q_d1_qcoth_dn4 = assign12570_e11802_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12570_e11802_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12570_e11802_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12570_e11802_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12570_e11802_d_n9;

        let assign12580_e11805: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard562 = assign12580_e11805;

        let (assign12590_e11814, assign12590_e11814_d_n4, assign12590_e11814_d_n6, assign12590_e11814_d_n7, assign12590_e11814_d_n8, assign12590_e11814_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
        let assign12590_e11811: f64 = (locals.var_q_qsq).abs();
        let assign12590_e11812: f64 = (assign12590_e11811).sqrt();
        (assign12590_e11812, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12590_e11812)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12590_e11812)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12590_e11812)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12590_e11812)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12590_e11812)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign12590_e11814;
        locals.var_q_rac_qsq_dn4 = assign12590_e11814_d_n4;
        locals.var_q_rac_qsq_dn6 = assign12590_e11814_d_n6;
        locals.var_q_rac_qsq_dn7 = assign12590_e11814_d_n7;
        locals.var_q_rac_qsq_dn8 = assign12590_e11814_d_n8;
        locals.var_q_rac_qsq_dn9 = assign12590_e11814_d_n9;

        let (assign12600_e11823, assign12600_e11823_d_n4, assign12600_e11823_d_n6, assign12600_e11823_d_n7, assign12600_e11823_d_n8, assign12600_e11823_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
        let assign12600_e11820: f64 = (-locals.var_q_rac_qsq);
        let assign12600_e11821: f64 = (assign12600_e11820).exp();
        (assign12600_e11821, (assign12600_e11821 * (-locals.var_q_rac_qsq_dn4)), (assign12600_e11821 * (-locals.var_q_rac_qsq_dn6)), (assign12600_e11821 * (-locals.var_q_rac_qsq_dn7)), (assign12600_e11821 * (-locals.var_q_rac_qsq_dn8)), (assign12600_e11821 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign12600_e11823;
        locals.var_q_invexpq_dn4 = assign12600_e11823_d_n4;
        locals.var_q_invexpq_dn6 = assign12600_e11823_d_n6;
        locals.var_q_invexpq_dn7 = assign12600_e11823_d_n7;
        locals.var_q_invexpq_dn8 = assign12600_e11823_d_n8;
        locals.var_q_invexpq_dn9 = assign12600_e11823_d_n9;

        let (assign12610_e11838, assign12610_e11838_d_n4, assign12610_e11838_d_n6, assign12610_e11838_d_n7, assign12610_e11838_d_n8, assign12610_e11838_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
        let assign12610_e11831: f64 = (1.0 + locals.var_q_invexpq);
        let assign12610_e11832: f64 = (locals.var_q_rac_qsq * assign12610_e11831);
        let assign12610_e11835: f64 = (1.0 - locals.var_q_invexpq);
        let assign12610_e11836: f64 = (assign12610_e11832 / assign12610_e11835);
        (assign12610_e11836, (((((locals.var_q_rac_qsq_dn4 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn4))) / (assign12610_e11835 * assign12610_e11835)), (((((locals.var_q_rac_qsq_dn6 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn6))) / (assign12610_e11835 * assign12610_e11835)), (((((locals.var_q_rac_qsq_dn7 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn7))) / (assign12610_e11835 * assign12610_e11835)), (((((locals.var_q_rac_qsq_dn8 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn8))) / (assign12610_e11835 * assign12610_e11835)), (((((locals.var_q_rac_qsq_dn9 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn9))) / (assign12610_e11835 * assign12610_e11835)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12610_e11838;
        locals.var_q_qcoth_dn4 = assign12610_e11838_d_n4;
        locals.var_q_qcoth_dn6 = assign12610_e11838_d_n6;
        locals.var_q_qcoth_dn7 = assign12610_e11838_d_n7;
        locals.var_q_qcoth_dn8 = assign12610_e11838_d_n8;
        locals.var_q_qcoth_dn9 = assign12610_e11838_d_n9;

        let (assign12620_e11855, assign12620_e11855_d_n4, assign12620_e11855_d_n6, assign12620_e11855_d_n7, assign12620_e11855_d_n8, assign12620_e11855_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
        let assign12620_e11848: f64 = (2.0 - locals.var_q_qcoth);
        let assign12620_e11849: f64 = (locals.var_q_qcoth * assign12620_e11848);
        let assign12620_e11850: f64 = (locals.var_q_qsq + assign12620_e11849);
        let assign12620_e11851: f64 = (0.25 * assign12620_e11850);
        let assign12620_e11853: f64 = (assign12620_e11851 / locals.var_q_qsq);
        (assign12620_e11853, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12620_e11855;
        locals.var_q_d1_qcoth_dn4 = assign12620_e11855_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12620_e11855_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12620_e11855_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12620_e11855_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12620_e11855_d_n9;

        let (assign12630_e11879, assign12630_e11879_d_n4, assign12630_e11879_d_n6, assign12630_e11879_d_n7, assign12630_e11879_d_n8, assign12630_e11879_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 == 0.0)) {
        let assign12630_e11864: f64 = (locals.var_q_qsq * 0.1666666666667);
        let assign12630_e11868: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign12630_e11872: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign12630_e11873: f64 = (1.0 - assign12630_e11872);
        let assign12630_e11874: f64 = (assign12630_e11868 * assign12630_e11873);
        let assign12630_e11875: f64 = (1.0 - assign12630_e11874);
        let assign12630_e11876: f64 = (assign12630_e11864 * assign12630_e11875);
        let assign12630_e11877: f64 = (2.0 + assign12630_e11876);
        (assign12630_e11877, (((locals.var_q_qsq_dn4 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq_dn6 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq_dn7 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq_dn8 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq_dn9 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12630_e11879;
        locals.var_q_qcoth_dn4 = assign12630_e11879_d_n4;
        locals.var_q_qcoth_dn6 = assign12630_e11879_d_n6;
        locals.var_q_qcoth_dn7 = assign12630_e11879_d_n7;
        locals.var_q_qcoth_dn8 = assign12630_e11879_d_n8;
        locals.var_q_qcoth_dn9 = assign12630_e11879_d_n9;

        let (assign12640_e11905, assign12640_e11905_d_n4, assign12640_e11905_d_n6, assign12640_e11905_d_n7, assign12640_e11905_d_n8, assign12640_e11905_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 == 0.0)) {
        let assign12640_e11889: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign12640_e11893: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign12640_e11897: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign12640_e11898: f64 = (1.0 - assign12640_e11897);
        let assign12640_e11899: f64 = (assign12640_e11893 * assign12640_e11898);
        let assign12640_e11900: f64 = (1.0 - assign12640_e11899);
        let assign12640_e11901: f64 = (assign12640_e11889 * assign12640_e11900);
        let assign12640_e11902: f64 = (1.0 - assign12640_e11901);
        let assign12640_e11903: f64 = (0.1666666666667 * assign12640_e11902);
        (assign12640_e11903, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12640_e11905;
        locals.var_q_d1_qcoth_dn4 = assign12640_e11905_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12640_e11905_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12640_e11905_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12640_e11905_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12640_e11905_d_n9;

        let assign12650_e11909: f64 = (locals.var_q_qi_int * locals.var_q_qcoth);
        let assign12650_e11912: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12650_e11913: f64 = (assign12650_e11909 + assign12650_e11912);
        let assign12650_e11915: f64 = (assign12650_e11913 + locals.var_q_qsq);
        let assign12650_e11918: f64 = (locals.var_q_qi_int * locals.var_q_d1_qcoth);
        let assign12650_e11920: f64 = (assign12650_e11918 + 1.0);
        let assign12650_e11921: f64 = (assign12650_e11915 / assign12650_e11920);
        let assign12650_e11922: f64 = (locals.var_q_qsq - assign12650_e11921);
        locals.var_q_qsq = assign12650_e11922;
        locals.var_q_qsq_dn4 = (locals.var_q_qsq_dn4 - (((((((locals.var_q_qi_int_dn4 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn4)) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))) + locals.var_q_qsq_dn4) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn4 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn4)))) / (assign12650_e11920 * assign12650_e11920)));
        locals.var_q_qsq_dn6 = (locals.var_q_qsq_dn6 - (((((((locals.var_q_qi_int_dn6 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn6)) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))) + locals.var_q_qsq_dn6) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn6 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn6)))) / (assign12650_e11920 * assign12650_e11920)));
        locals.var_q_qsq_dn7 = (locals.var_q_qsq_dn7 - (((((((locals.var_q_qi_int_dn7 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn7)) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))) + locals.var_q_qsq_dn7) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn7 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn7)))) / (assign12650_e11920 * assign12650_e11920)));
        locals.var_q_qsq_dn8 = (locals.var_q_qsq_dn8 - (((((((locals.var_q_qi_int_dn8 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn8)) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))) + locals.var_q_qsq_dn8) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn8 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn8)))) / (assign12650_e11920 * assign12650_e11920)));
        locals.var_q_qsq_dn9 = (locals.var_q_qsq_dn9 - (((((((locals.var_q_qi_int_dn9 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn9)) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))) + locals.var_q_qsq_dn9) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn9 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn9)))) / (assign12650_e11920 * assign12650_e11920)));

        let assign12660_e11925: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12660_e11927: f64 = (assign12660_e11925 - locals.var_q_qsq);
        locals.var_q_delta = assign12660_e11927;
        locals.var_q_delta_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4);
        locals.var_q_delta_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6);
        locals.var_q_delta_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7);
        locals.var_q_delta_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8);
        locals.var_q_delta_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9);

        let assign12670_e11930: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign12670_e11930;

        let (assign12680_e11945, assign12680_e11945_d_n4, assign12680_e11945_d_n6, assign12680_e11945_d_n7, assign12680_e11945_d_n8, assign12680_e11945_d_n9,) = {
    if (locals.var_guard563 != 0.0) {
        let assign12680_e11935: f64 = (locals.var_q_delta / locals.var_a0);
        let assign12680_e11936: f64 = (assign12680_e11935).ln();
        let assign12680_e11938: f64 = assign12680_e11936;
        let assign12680_e11940: f64 = (assign12680_e11938 - locals.var_xg1x);
        let assign12680_e11942: f64 = (assign12680_e11940 + locals.var_q1s);
        let assign12680_e11943: f64 = (locals.var_q_delta * assign12680_e11942);
        (assign12680_e11943, ((locals.var_q_delta_dn4 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn4) + locals.var_q1s_dn4))), ((locals.var_q_delta_dn6 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn6) + locals.var_q1s_dn6))), ((locals.var_q_delta_dn7 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn7) + locals.var_q1s_dn7))), ((locals.var_q_delta_dn8 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn8) + locals.var_q1s_dn8))), ((locals.var_q_delta_dn9 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn9) + locals.var_q1s_dn9))),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign12680_e11945;
        locals.var_q_zero_dn4 = assign12680_e11945_d_n4;
        locals.var_q_zero_dn6 = assign12680_e11945_d_n6;
        locals.var_q_zero_dn7 = assign12680_e11945_d_n7;
        locals.var_q_zero_dn8 = assign12680_e11945_d_n8;
        locals.var_q_zero_dn9 = assign12680_e11945_d_n9;

        let (assign12690_e11955, assign12690_e11955_d_n4, assign12690_e11955_d_n6, assign12690_e11955_d_n7, assign12690_e11955_d_n8, assign12690_e11955_d_n9,) = {
    if (locals.var_guard563 != 0.0) {
        let assign12690_e11949: f64 = (2.0 * locals.var_k1);
        let assign12690_e11951: f64 = (assign12690_e11949 * locals.var_q_k1q1);
        let assign12690_e11953: f64 = (assign12690_e11951 + locals.var_q_delta);
        (assign12690_e11953, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign12690_e11955;
        locals.var_q_d1_zero_dn4 = assign12690_e11955_d_n4;
        locals.var_q_d1_zero_dn6 = assign12690_e11955_d_n6;
        locals.var_q_d1_zero_dn7 = assign12690_e11955_d_n7;
        locals.var_q_d1_zero_dn8 = assign12690_e11955_d_n8;
        locals.var_q_d1_zero_dn9 = assign12690_e11955_d_n9;

        let (assign12700_e11963,) = {
    if (locals.var_guard563 != 0.0) {
        let assign12700_e11959: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12700_e11961: f64 = (assign12700_e11959 - locals.var_q_x1sat);
        (assign12700_e11961,)
    } else {
        (locals.var_q_dx1,)
    }
};
        locals.var_q_dx1 = assign12700_e11963;

        let assign12710_e11973: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign12710_e11975: f64 = (locals.var_k1).ln();
        let assign12710_e11976: f64 = (assign12710_e11973 + assign12710_e11975);
        let assign12710_e11983: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign12710_e11976 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard564 = assign12710_e11983;

        let (assign12720_e11993, assign12720_e11993_d_n4, assign12720_e11993_d_n6, assign12720_e11993_d_n7, assign12720_e11993_d_n8, assign12720_e11993_d_n9,) = {
    if ((locals.var_guard563 != 0.0) && (locals.var_guard564 != 0.0)) {
        let assign12720_e11990: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
        let assign12720_e11991: f64 = (locals.var_q1s - assign12720_e11990);
        (assign12720_e11991, (locals.var_q1s_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))),)
    } else {
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9,)
    }
};
        locals.var_q1s = assign12720_e11993;
        locals.var_q1s_dn4 = assign12720_e11993_d_n4;
        locals.var_q1s_dn6 = assign12720_e11993_d_n6;
        locals.var_q1s_dn7 = assign12720_e11993_d_n7;
        locals.var_q1s_dn8 = assign12720_e11993_d_n8;
        locals.var_q1s_dn9 = assign12720_e11993_d_n9;

        let assign12730_e11996: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign12730_e11996;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign12740_e11999: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12740_e12001: f64 = assign12740_e11999;
        let assign12740_e12003: f64 = if assign12740_e12001 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard565 = assign12740_e12003;

        let (assign12750_e12012, assign12750_e12012_d_n4, assign12750_e12012_d_n6, assign12750_e12012_d_n7, assign12750_e12012_d_n8, assign12750_e12012_d_n9,) = {
    if (locals.var_guard565 != 0.0) {
        let assign12750_e12007: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12750_e12009: f64 = assign12750_e12007;
        let assign12750_e12010: f64 = (assign12750_e12009).exp();
        (assign12750_e12010, (assign12750_e12010 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign12750_e12010 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign12750_e12010 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign12750_e12010 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign12750_e12010 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12750_e12012;
        locals.var_q_temp1_dn4 = assign12750_e12012_d_n4;
        locals.var_q_temp1_dn6 = assign12750_e12012_d_n6;
        locals.var_q_temp1_dn7 = assign12750_e12012_d_n7;
        locals.var_q_temp1_dn8 = assign12750_e12012_d_n8;
        locals.var_q_temp1_dn9 = assign12750_e12012_d_n9;

        let (assign12760_e12051, assign12760_e12051_d_n4, assign12760_e12051_d_n6, assign12760_e12051_d_n7, assign12760_e12051_d_n8, assign12760_e12051_d_n9,) = {
    if (locals.var_guard565 == 0.0) {
        let assign12760_e12019: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12760_e12021: f64 = assign12760_e12019;
        let assign12760_e12023: f64 = (assign12760_e12021 - 80.0);
        let assign12760_e12028: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12760_e12030: f64 = assign12760_e12028;
        let assign12760_e12032: f64 = (assign12760_e12030 - 80.0);
        let assign12760_e12033: f64 = (0.5 * assign12760_e12032);
        let assign12760_e12037: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12760_e12039: f64 = assign12760_e12037;
        let assign12760_e12041: f64 = (assign12760_e12039 - 80.0);
        let assign12760_e12043: f64 = (assign12760_e12041 * 0.3333333333333);
        let assign12760_e12044: f64 = (1.0 + assign12760_e12043);
        let assign12760_e12045: f64 = (assign12760_e12033 * assign12760_e12044);
        let assign12760_e12046: f64 = (1.0 + assign12760_e12045);
        let assign12760_e12047: f64 = (assign12760_e12023 * assign12760_e12046);
        let assign12760_e12048: f64 = (1.0 + assign12760_e12047);
        let assign12760_e12049: f64 = (5.54062e34 * assign12760_e12048);
        (assign12760_e12049, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12760_e12051;
        locals.var_q_temp1_dn4 = assign12760_e12051_d_n4;
        locals.var_q_temp1_dn6 = assign12760_e12051_d_n6;
        locals.var_q_temp1_dn7 = assign12760_e12051_d_n7;
        locals.var_q_temp1_dn8 = assign12760_e12051_d_n8;
        locals.var_q_temp1_dn9 = assign12760_e12051_d_n9;

        let assign12770_e12054: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign12770_e12054;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign12780_e12057: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12780_e12059: f64 = (assign12780_e12057 - locals.var_q_aexp);
        locals.var_q_qsq = assign12780_e12059;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign12790_e12062: f64 = (2.0 * locals.var_k1);
        let assign12790_e12064: f64 = (assign12790_e12062 * locals.var_q_k1q1);
        let assign12790_e12066: f64 = (assign12790_e12064 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign12790_e12066;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign12800_e12069: f64 = (2.0 * locals.var_k1);
        let assign12800_e12071: f64 = (assign12800_e12069 * locals.var_k1);
        let assign12800_e12073: f64 = (assign12800_e12071 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign12800_e12073;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign12810_e12076: f64 = (-0.005);
        let assign12810_e12077: f64 = if locals.var_q_qsq < assign12810_e12076 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign12810_e12077;

        let (assign12820_e12083, assign12820_e12083_d_n4, assign12820_e12083_d_n6, assign12820_e12083_d_n7, assign12820_e12083_d_n8, assign12820_e12083_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12820_e12080: f64 = (locals.var_q_qsq).abs();
        let assign12820_e12081: f64 = (assign12820_e12080).sqrt();
        (assign12820_e12081, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12820_e12081)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12820_e12081)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12820_e12081)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12820_e12081)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12820_e12081)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign12820_e12083;
        locals.var_q_rac_qsq_dn4 = assign12820_e12083_d_n4;
        locals.var_q_rac_qsq_dn6 = assign12820_e12083_d_n6;
        locals.var_q_rac_qsq_dn7 = assign12820_e12083_d_n7;
        locals.var_q_rac_qsq_dn8 = assign12820_e12083_d_n8;
        locals.var_q_rac_qsq_dn9 = assign12820_e12083_d_n9;

        let (assign12830_e12092, assign12830_e12092_d_n4, assign12830_e12092_d_n6, assign12830_e12092_d_n7, assign12830_e12092_d_n8, assign12830_e12092_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12830_e12088: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign12830_e12089: f64 = (assign12830_e12088).tan();
        let assign12830_e12090: f64 = (locals.var_q_rac_qsq / assign12830_e12089);
        (assign12830_e12090, (((locals.var_q_rac_qsq_dn4 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), (((locals.var_q_rac_qsq_dn6 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), (((locals.var_q_rac_qsq_dn7 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), (((locals.var_q_rac_qsq_dn8 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), (((locals.var_q_rac_qsq_dn9 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12830_e12092;
        locals.var_q_qcoth_dn4 = assign12830_e12092_d_n4;
        locals.var_q_qcoth_dn6 = assign12830_e12092_d_n6;
        locals.var_q_qcoth_dn7 = assign12830_e12092_d_n7;
        locals.var_q_qcoth_dn8 = assign12830_e12092_d_n8;
        locals.var_q_qcoth_dn9 = assign12830_e12092_d_n9;

        let (assign12840_e12100, assign12840_e12100_d_n4, assign12840_e12100_d_n6, assign12840_e12100_d_n7, assign12840_e12100_d_n8, assign12840_e12100_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12840_e12096: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign12840_e12098: f64 = (assign12840_e12096 / locals.var_q_qsq);
        (assign12840_e12098, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12840_e12100;
        locals.var_q_temp1_dn4 = assign12840_e12100_d_n4;
        locals.var_q_temp1_dn6 = assign12840_e12100_d_n6;
        locals.var_q_temp1_dn7 = assign12840_e12100_d_n7;
        locals.var_q_temp1_dn8 = assign12840_e12100_d_n8;
        locals.var_q_temp1_dn9 = assign12840_e12100_d_n9;

        let (assign12850_e12112, assign12850_e12112_d_n4, assign12850_e12112_d_n6, assign12850_e12112_d_n7, assign12850_e12112_d_n8, assign12850_e12112_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12850_e12106: f64 = (2.0 - locals.var_q_qcoth);
        let assign12850_e12107: f64 = (locals.var_q_qcoth * assign12850_e12106);
        let assign12850_e12108: f64 = (locals.var_q_qsq + assign12850_e12107);
        let assign12850_e12110: f64 = (assign12850_e12108 * locals.var_q_temp1);
        (assign12850_e12110, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12850_e12112;
        locals.var_q_d1_qcoth_dn4 = assign12850_e12112_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12850_e12112_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12850_e12112_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12850_e12112_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12850_e12112_d_n9;

        let (assign12860_e12132, assign12860_e12132_d_n4, assign12860_e12132_d_n6, assign12860_e12132_d_n7, assign12860_e12132_d_n8, assign12860_e12132_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12860_e12117: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign12860_e12120: f64 = (1.0 + locals.var_q_qcoth);
        let assign12860_e12121: f64 = (assign12860_e12117 * assign12860_e12120);
        let assign12860_e12122: f64 = (locals.var_q_d1_qsq - assign12860_e12121);
        let assign12860_e12124: f64 = (assign12860_e12122 * locals.var_q_temp1);
        let assign12860_e12127: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign12860_e12129: f64 = (assign12860_e12127 / locals.var_q_d1_qsq);
        let assign12860_e12130: f64 = (assign12860_e12124 + assign12860_e12129);
        (assign12860_e12130, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign12860_e12132;
        locals.var_q_d2_qcoth_dn4 = assign12860_e12132_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign12860_e12132_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign12860_e12132_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign12860_e12132_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign12860_e12132_d_n9;

        let (assign12870_e12140, assign12870_e12140_d_n4, assign12870_e12140_d_n6, assign12870_e12140_d_n7, assign12870_e12140_d_n8, assign12870_e12140_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12870_e12137: f64 = (0.5 * locals.var_q_qcoth);
        let assign12870_e12138: f64 = (1.0 - assign12870_e12137);
        (assign12870_e12138, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign12870_e12140;
        locals.var_q_temp2_dn4 = assign12870_e12140_d_n4;
        locals.var_q_temp2_dn6 = assign12870_e12140_d_n6;
        locals.var_q_temp2_dn7 = assign12870_e12140_d_n7;
        locals.var_q_temp2_dn8 = assign12870_e12140_d_n8;
        locals.var_q_temp2_dn9 = assign12870_e12140_d_n9;

        let (assign12880_e12148, assign12880_e12148_d_n4, assign12880_e12148_d_n6, assign12880_e12148_d_n7, assign12880_e12148_d_n8, assign12880_e12148_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12880_e12144: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign12880_e12146: f64 = (assign12880_e12144 * locals.var_q_temp2);
        (assign12880_e12146, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign12880_e12148;
        locals.var_q_d1_ln_dn4 = assign12880_e12148_d_n4;
        locals.var_q_d1_ln_dn6 = assign12880_e12148_d_n6;
        locals.var_q_d1_ln_dn7 = assign12880_e12148_d_n7;
        locals.var_q_d1_ln_dn8 = assign12880_e12148_d_n8;
        locals.var_q_d1_ln_dn9 = assign12880_e12148_d_n9;

        let (assign12890_e12164, assign12890_e12164_d_n4, assign12890_e12164_d_n6, assign12890_e12164_d_n7, assign12890_e12164_d_n8, assign12890_e12164_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12890_e12152: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign12890_e12157: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign12890_e12158: f64 = (locals.var_q_d1_ln + assign12890_e12157);
        let assign12890_e12159: f64 = (locals.var_q_d1_qsq * assign12890_e12158);
        let assign12890_e12160: f64 = (assign12890_e12152 - assign12890_e12159);
        let assign12890_e12162: f64 = (assign12890_e12160 / locals.var_q_qsq);
        (assign12890_e12162, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign12890_e12164;
        locals.var_q_d2_ln_dn4 = assign12890_e12164_d_n4;
        locals.var_q_d2_ln_dn6 = assign12890_e12164_d_n6;
        locals.var_q_d2_ln_dn7 = assign12890_e12164_d_n7;
        locals.var_q_d2_ln_dn8 = assign12890_e12164_d_n8;
        locals.var_q_d2_ln_dn9 = assign12890_e12164_d_n9;

        let assign12900_e12167: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard567 = assign12900_e12167;

        let (assign12910_e12176, assign12910_e12176_d_n4, assign12910_e12176_d_n6, assign12910_e12176_d_n7, assign12910_e12176_d_n8, assign12910_e12176_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12910_e12173: f64 = (locals.var_q_qsq).abs();
        let assign12910_e12174: f64 = (assign12910_e12173).sqrt();
        (assign12910_e12174, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12910_e12174)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12910_e12174)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12910_e12174)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12910_e12174)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12910_e12174)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign12910_e12176;
        locals.var_q_rac_qsq_dn4 = assign12910_e12176_d_n4;
        locals.var_q_rac_qsq_dn6 = assign12910_e12176_d_n6;
        locals.var_q_rac_qsq_dn7 = assign12910_e12176_d_n7;
        locals.var_q_rac_qsq_dn8 = assign12910_e12176_d_n8;
        locals.var_q_rac_qsq_dn9 = assign12910_e12176_d_n9;

    }

    pub(super) fn stamp_transient_block_30(
        locals: &mut StampLocals,
    ) {
        let (assign12920_e12185, assign12920_e12185_d_n4, assign12920_e12185_d_n6, assign12920_e12185_d_n7, assign12920_e12185_d_n8, assign12920_e12185_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12920_e12182: f64 = (-locals.var_q_rac_qsq);
        let assign12920_e12183: f64 = (assign12920_e12182).exp();
        (assign12920_e12183, (assign12920_e12183 * (-locals.var_q_rac_qsq_dn4)), (assign12920_e12183 * (-locals.var_q_rac_qsq_dn6)), (assign12920_e12183 * (-locals.var_q_rac_qsq_dn7)), (assign12920_e12183 * (-locals.var_q_rac_qsq_dn8)), (assign12920_e12183 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign12920_e12185;
        locals.var_q_invexpq_dn4 = assign12920_e12185_d_n4;
        locals.var_q_invexpq_dn6 = assign12920_e12185_d_n6;
        locals.var_q_invexpq_dn7 = assign12920_e12185_d_n7;
        locals.var_q_invexpq_dn8 = assign12920_e12185_d_n8;
        locals.var_q_invexpq_dn9 = assign12920_e12185_d_n9;

        let (assign12930_e12200, assign12930_e12200_d_n4, assign12930_e12200_d_n6, assign12930_e12200_d_n7, assign12930_e12200_d_n8, assign12930_e12200_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12930_e12193: f64 = (1.0 + locals.var_q_invexpq);
        let assign12930_e12194: f64 = (locals.var_q_rac_qsq * assign12930_e12193);
        let assign12930_e12197: f64 = (1.0 - locals.var_q_invexpq);
        let assign12930_e12198: f64 = (assign12930_e12194 / assign12930_e12197);
        (assign12930_e12198, (((((locals.var_q_rac_qsq_dn4 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn4))) / (assign12930_e12197 * assign12930_e12197)), (((((locals.var_q_rac_qsq_dn6 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn6))) / (assign12930_e12197 * assign12930_e12197)), (((((locals.var_q_rac_qsq_dn7 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn7))) / (assign12930_e12197 * assign12930_e12197)), (((((locals.var_q_rac_qsq_dn8 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn8))) / (assign12930_e12197 * assign12930_e12197)), (((((locals.var_q_rac_qsq_dn9 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn9))) / (assign12930_e12197 * assign12930_e12197)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12930_e12200;
        locals.var_q_qcoth_dn4 = assign12930_e12200_d_n4;
        locals.var_q_qcoth_dn6 = assign12930_e12200_d_n6;
        locals.var_q_qcoth_dn7 = assign12930_e12200_d_n7;
        locals.var_q_qcoth_dn8 = assign12930_e12200_d_n8;
        locals.var_q_qcoth_dn9 = assign12930_e12200_d_n9;

        let (assign12940_e12211, assign12940_e12211_d_n4, assign12940_e12211_d_n6, assign12940_e12211_d_n7, assign12940_e12211_d_n8, assign12940_e12211_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12940_e12207: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign12940_e12209: f64 = (assign12940_e12207 / locals.var_q_qsq);
        (assign12940_e12209, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12940_e12211;
        locals.var_q_temp1_dn4 = assign12940_e12211_d_n4;
        locals.var_q_temp1_dn6 = assign12940_e12211_d_n6;
        locals.var_q_temp1_dn7 = assign12940_e12211_d_n7;
        locals.var_q_temp1_dn8 = assign12940_e12211_d_n8;
        locals.var_q_temp1_dn9 = assign12940_e12211_d_n9;

        let (assign12950_e12226, assign12950_e12226_d_n4, assign12950_e12226_d_n6, assign12950_e12226_d_n7, assign12950_e12226_d_n8, assign12950_e12226_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12950_e12220: f64 = (2.0 - locals.var_q_qcoth);
        let assign12950_e12221: f64 = (locals.var_q_qcoth * assign12950_e12220);
        let assign12950_e12222: f64 = (locals.var_q_qsq + assign12950_e12221);
        let assign12950_e12224: f64 = (assign12950_e12222 * locals.var_q_temp1);
        (assign12950_e12224, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12950_e12226;
        locals.var_q_d1_qcoth_dn4 = assign12950_e12226_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12950_e12226_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12950_e12226_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12950_e12226_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12950_e12226_d_n9;

        let (assign12960_e12249, assign12960_e12249_d_n4, assign12960_e12249_d_n6, assign12960_e12249_d_n7, assign12960_e12249_d_n8, assign12960_e12249_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12960_e12234: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign12960_e12237: f64 = (1.0 + locals.var_q_qcoth);
        let assign12960_e12238: f64 = (assign12960_e12234 * assign12960_e12237);
        let assign12960_e12239: f64 = (locals.var_q_d1_qsq - assign12960_e12238);
        let assign12960_e12241: f64 = (assign12960_e12239 * locals.var_q_temp1);
        let assign12960_e12244: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign12960_e12246: f64 = (assign12960_e12244 / locals.var_q_d1_qsq);
        let assign12960_e12247: f64 = (assign12960_e12241 + assign12960_e12246);
        (assign12960_e12247, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign12960_e12249;
        locals.var_q_d2_qcoth_dn4 = assign12960_e12249_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign12960_e12249_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign12960_e12249_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign12960_e12249_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign12960_e12249_d_n9;

        let (assign12970_e12260, assign12970_e12260_d_n4, assign12970_e12260_d_n6, assign12970_e12260_d_n7, assign12970_e12260_d_n8, assign12970_e12260_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12970_e12257: f64 = (0.5 * locals.var_q_qcoth);
        let assign12970_e12258: f64 = (1.0 - assign12970_e12257);
        (assign12970_e12258, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign12970_e12260;
        locals.var_q_temp2_dn4 = assign12970_e12260_d_n4;
        locals.var_q_temp2_dn6 = assign12970_e12260_d_n6;
        locals.var_q_temp2_dn7 = assign12970_e12260_d_n7;
        locals.var_q_temp2_dn8 = assign12970_e12260_d_n8;
        locals.var_q_temp2_dn9 = assign12970_e12260_d_n9;

        let (assign12980_e12271, assign12980_e12271_d_n4, assign12980_e12271_d_n6, assign12980_e12271_d_n7, assign12980_e12271_d_n8, assign12980_e12271_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12980_e12267: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign12980_e12269: f64 = (assign12980_e12267 * locals.var_q_temp2);
        (assign12980_e12269, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign12980_e12271;
        locals.var_q_d1_ln_dn4 = assign12980_e12271_d_n4;
        locals.var_q_d1_ln_dn6 = assign12980_e12271_d_n6;
        locals.var_q_d1_ln_dn7 = assign12980_e12271_d_n7;
        locals.var_q_d1_ln_dn8 = assign12980_e12271_d_n8;
        locals.var_q_d1_ln_dn9 = assign12980_e12271_d_n9;

        let (assign12990_e12290, assign12990_e12290_d_n4, assign12990_e12290_d_n6, assign12990_e12290_d_n7, assign12990_e12290_d_n8, assign12990_e12290_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign12990_e12278: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign12990_e12283: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign12990_e12284: f64 = (locals.var_q_d1_ln + assign12990_e12283);
        let assign12990_e12285: f64 = (locals.var_q_d1_qsq * assign12990_e12284);
        let assign12990_e12286: f64 = (assign12990_e12278 - assign12990_e12285);
        let assign12990_e12288: f64 = (assign12990_e12286 / locals.var_q_qsq);
        (assign12990_e12288, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign12990_e12290;
        locals.var_q_d2_ln_dn4 = assign12990_e12290_d_n4;
        locals.var_q_d2_ln_dn6 = assign12990_e12290_d_n6;
        locals.var_q_d2_ln_dn7 = assign12990_e12290_d_n7;
        locals.var_q_d2_ln_dn8 = assign12990_e12290_d_n8;
        locals.var_q_d2_ln_dn9 = assign12990_e12290_d_n9;

        let (assign13000_e12316, assign13000_e12316_d_n4, assign13000_e12316_d_n6, assign13000_e12316_d_n7, assign13000_e12316_d_n8, assign13000_e12316_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13000_e12300: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign13000_e12304: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign13000_e12308: f64 = (locals.var_q_qsq * 0.025);
        let assign13000_e12309: f64 = (1.0 - assign13000_e12308);
        let assign13000_e12310: f64 = (assign13000_e12304 * assign13000_e12309);
        let assign13000_e12311: f64 = (1.0 - assign13000_e12310);
        let assign13000_e12312: f64 = (assign13000_e12300 * assign13000_e12311);
        let assign13000_e12313: f64 = (1.0 - assign13000_e12312);
        let assign13000_e12314: f64 = (0.1666666666667 * assign13000_e12313);
        (assign13000_e12314, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign13000_e12316;
        locals.var_q_temp3_dn4 = assign13000_e12316_d_n4;
        locals.var_q_temp3_dn6 = assign13000_e12316_d_n6;
        locals.var_q_temp3_dn7 = assign13000_e12316_d_n7;
        locals.var_q_temp3_dn8 = assign13000_e12316_d_n8;
        locals.var_q_temp3_dn9 = assign13000_e12316_d_n9;

        let (assign13010_e12328, assign13010_e12328_d_n4, assign13010_e12328_d_n6, assign13010_e12328_d_n7, assign13010_e12328_d_n8, assign13010_e12328_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13010_e12325: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign13010_e12326: f64 = (2.0 + assign13010_e12325);
        (assign13010_e12326, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13010_e12328;
        locals.var_q_qcoth_dn4 = assign13010_e12328_d_n4;
        locals.var_q_qcoth_dn6 = assign13010_e12328_d_n6;
        locals.var_q_qcoth_dn7 = assign13010_e12328_d_n7;
        locals.var_q_qcoth_dn8 = assign13010_e12328_d_n8;
        locals.var_q_qcoth_dn9 = assign13010_e12328_d_n9;

        let (assign13020_e12354, assign13020_e12354_d_n4, assign13020_e12354_d_n6, assign13020_e12354_d_n7, assign13020_e12354_d_n8, assign13020_e12354_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13020_e12338: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign13020_e12342: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign13020_e12346: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign13020_e12347: f64 = (1.0 - assign13020_e12346);
        let assign13020_e12348: f64 = (assign13020_e12342 * assign13020_e12347);
        let assign13020_e12349: f64 = (1.0 - assign13020_e12348);
        let assign13020_e12350: f64 = (assign13020_e12338 * assign13020_e12349);
        let assign13020_e12351: f64 = (1.0 - assign13020_e12350);
        let assign13020_e12352: f64 = (0.1666666666667 * assign13020_e12351);
        (assign13020_e12352, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13020_e12354;
        locals.var_q_temp1_dn4 = assign13020_e12354_d_n4;
        locals.var_q_temp1_dn6 = assign13020_e12354_d_n6;
        locals.var_q_temp1_dn7 = assign13020_e12354_d_n7;
        locals.var_q_temp1_dn8 = assign13020_e12354_d_n8;
        locals.var_q_temp1_dn9 = assign13020_e12354_d_n9;

        let (assign13030_e12364, assign13030_e12364_d_n4, assign13030_e12364_d_n6, assign13030_e12364_d_n7, assign13030_e12364_d_n8, assign13030_e12364_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13030_e12362: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign13030_e12362, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13030_e12364;
        locals.var_q_d1_qcoth_dn4 = assign13030_e12364_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13030_e12364_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13030_e12364_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13030_e12364_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13030_e12364_d_n9;

        let (assign13040_e12390, assign13040_e12390_d_n4, assign13040_e12390_d_n6, assign13040_e12390_d_n7, assign13040_e12390_d_n8, assign13040_e12390_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13040_e12374: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign13040_e12378: f64 = (0.05 * locals.var_q_qsq);
        let assign13040_e12382: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign13040_e12383: f64 = (1.0 - assign13040_e12382);
        let assign13040_e12384: f64 = (assign13040_e12378 * assign13040_e12383);
        let assign13040_e12385: f64 = (1.0 - assign13040_e12384);
        let assign13040_e12386: f64 = (assign13040_e12374 * assign13040_e12385);
        let assign13040_e12387: f64 = (1.0 - assign13040_e12386);
        let assign13040_e12388: f64 = (0.0055555555556 * assign13040_e12387);
        (assign13040_e12388, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13040_e12390;
        locals.var_q_temp2_dn4 = assign13040_e12390_d_n4;
        locals.var_q_temp2_dn6 = assign13040_e12390_d_n6;
        locals.var_q_temp2_dn7 = assign13040_e12390_d_n7;
        locals.var_q_temp2_dn8 = assign13040_e12390_d_n8;
        locals.var_q_temp2_dn9 = assign13040_e12390_d_n9;

        let (assign13050_e12406, assign13050_e12406_d_n4, assign13050_e12406_d_n6, assign13050_e12406_d_n7, assign13050_e12406_d_n8, assign13050_e12406_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13050_e12398: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign13050_e12401: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign13050_e12403: f64 = (assign13050_e12401 * locals.var_q_temp2);
        let assign13050_e12404: f64 = (assign13050_e12398 - assign13050_e12403);
        (assign13050_e12404, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13050_e12406;
        locals.var_q_d2_qcoth_dn4 = assign13050_e12406_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13050_e12406_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13050_e12406_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13050_e12406_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13050_e12406_d_n9;

        let (assign13060_e12419, assign13060_e12419_d_n4, assign13060_e12419_d_n6, assign13060_e12419_d_n7, assign13060_e12419_d_n8, assign13060_e12419_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13060_e12413: f64 = (-0.5);
        let assign13060_e12415: f64 = (assign13060_e12413 * locals.var_q_d1_qsq);
        let assign13060_e12417: f64 = (assign13060_e12415 * locals.var_q_temp3);
        (assign13060_e12417, (((assign13060_e12413 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn4)), (((assign13060_e12413 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn6)), (((assign13060_e12413 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn7)), (((assign13060_e12413 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn8)), (((assign13060_e12413 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13060_e12419;
        locals.var_q_d1_ln_dn4 = assign13060_e12419_d_n4;
        locals.var_q_d1_ln_dn6 = assign13060_e12419_d_n6;
        locals.var_q_d1_ln_dn7 = assign13060_e12419_d_n7;
        locals.var_q_d1_ln_dn8 = assign13060_e12419_d_n8;
        locals.var_q_d1_ln_dn9 = assign13060_e12419_d_n9;

        let (assign13070_e12452, assign13070_e12452_d_n4, assign13070_e12452_d_n6, assign13070_e12452_d_n7, assign13070_e12452_d_n8, assign13070_e12452_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13070_e12426: f64 = (-0.5);
        let assign13070_e12428: f64 = (assign13070_e12426 * locals.var_q_d2_qsq);
        let assign13070_e12430: f64 = (assign13070_e12428 * locals.var_q_temp3);
        let assign13070_e12433: f64 = (0.25 * 0.0055555555556);
        let assign13070_e12435: f64 = (assign13070_e12433 * locals.var_q_d1_qsq);
        let assign13070_e12437: f64 = (assign13070_e12435 * locals.var_q_d1_qsq);
        let assign13070_e12441: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign13070_e12445: f64 = (0.075 * locals.var_q_qsq);
        let assign13070_e12446: f64 = (2.0 - assign13070_e12445);
        let assign13070_e12447: f64 = (assign13070_e12441 * assign13070_e12446);
        let assign13070_e12448: f64 = (1.0 - assign13070_e12447);
        let assign13070_e12449: f64 = (assign13070_e12437 * assign13070_e12448);
        let assign13070_e12450: f64 = (assign13070_e12430 + assign13070_e12449);
        (assign13070_e12450, ((((assign13070_e12426 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn4)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn4)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign13070_e12426 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn6)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn6)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign13070_e12426 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn7)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn7)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign13070_e12426 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn8)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn8)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign13070_e12426 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn9)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn9)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13070_e12452;
        locals.var_q_d2_ln_dn4 = assign13070_e12452_d_n4;
        locals.var_q_d2_ln_dn6 = assign13070_e12452_d_n6;
        locals.var_q_d2_ln_dn7 = assign13070_e12452_d_n7;
        locals.var_q_d2_ln_dn8 = assign13070_e12452_d_n8;
        locals.var_q_d2_ln_dn9 = assign13070_e12452_d_n9;

        let assign13080_e12455: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign13080_e12455;

        let (assign13090_e12469, assign13090_e12469_d_n4, assign13090_e12469_d_n6, assign13090_e12469_d_n7, assign13090_e12469_d_n8, assign13090_e12469_d_n9,) = {
    if (locals.var_guard568 != 0.0) {
        let assign13090_e12459: f64 = (4.0 * locals.var_q_qsq);
        let assign13090_e12464: f64 = (2.0 - locals.var_q_invexpq);
        let assign13090_e12465: f64 = (locals.var_q_invexpq * assign13090_e12464);
        let assign13090_e12466: f64 = (1.0 - assign13090_e12465);
        let assign13090_e12467: f64 = (assign13090_e12459 / assign13090_e12466);
        (assign13090_e12467, ((((4.0 * locals.var_q_qsq_dn4) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn4 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign13090_e12466 * assign13090_e12466)), ((((4.0 * locals.var_q_qsq_dn6) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn6 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign13090_e12466 * assign13090_e12466)), ((((4.0 * locals.var_q_qsq_dn7) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn7 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign13090_e12466 * assign13090_e12466)), ((((4.0 * locals.var_q_qsq_dn8) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn8 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign13090_e12466 * assign13090_e12466)), ((((4.0 * locals.var_q_qsq_dn9) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn9 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign13090_e12466 * assign13090_e12466)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13090_e12469;
        locals.var_q_temp2_dn4 = assign13090_e12469_d_n4;
        locals.var_q_temp2_dn6 = assign13090_e12469_d_n6;
        locals.var_q_temp2_dn7 = assign13090_e12469_d_n7;
        locals.var_q_temp2_dn8 = assign13090_e12469_d_n8;
        locals.var_q_temp2_dn9 = assign13090_e12469_d_n9;

        let (assign13100_e12475, assign13100_e12475_d_n4, assign13100_e12475_d_n6, assign13100_e12475_d_n7, assign13100_e12475_d_n8, assign13100_e12475_d_n9,) = {
    if (locals.var_guard568 != 0.0) {
        let assign13100_e12473: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign13100_e12473, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13100_e12475;
        locals.var_q_sh_term_dn4 = assign13100_e12475_d_n4;
        locals.var_q_sh_term_dn6 = assign13100_e12475_d_n6;
        locals.var_q_sh_term_dn7 = assign13100_e12475_d_n7;
        locals.var_q_sh_term_dn8 = assign13100_e12475_d_n8;
        locals.var_q_sh_term_dn9 = assign13100_e12475_d_n9;

        let (assign13110_e12482, assign13110_e12482_d_n4, assign13110_e12482_d_n6, assign13110_e12482_d_n7, assign13110_e12482_d_n8, assign13110_e12482_d_n9,) = {
    if (locals.var_guard568 != 0.0) {
        let assign13110_e12478: f64 = (locals.var_q_temp2).ln();
        let assign13110_e12480: f64 = (assign13110_e12478 - locals.var_q_rac_qsq);
        (assign13110_e12480, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13110_e12482;
        locals.var_q_ln_term_dn4 = assign13110_e12482_d_n4;
        locals.var_q_ln_term_dn6 = assign13110_e12482_d_n6;
        locals.var_q_ln_term_dn7 = assign13110_e12482_d_n7;
        locals.var_q_ln_term_dn8 = assign13110_e12482_d_n8;
        locals.var_q_ln_term_dn9 = assign13110_e12482_d_n9;

        let assign13120_e12485: f64 = (-0.005);
        let assign13120_e12486: f64 = if locals.var_q_qsq < assign13120_e12485 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign13120_e12486;

        let (assign13130_e12496, assign13130_e12496_d_n4, assign13130_e12496_d_n6, assign13130_e12496_d_n7, assign13130_e12496_d_n8, assign13130_e12496_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
        let assign13130_e12493: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign13130_e12494: f64 = (assign13130_e12493).sin();
        (assign13130_e12494, ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13130_e12496;
        locals.var_q_temp2_dn4 = assign13130_e12496_d_n4;
        locals.var_q_temp2_dn6 = assign13130_e12496_d_n6;
        locals.var_q_temp2_dn7 = assign13130_e12496_d_n7;
        locals.var_q_temp2_dn8 = assign13130_e12496_d_n8;
        locals.var_q_temp2_dn9 = assign13130_e12496_d_n9;

        let (assign13140_e12508, assign13140_e12508_d_n4, assign13140_e12508_d_n6, assign13140_e12508_d_n7, assign13140_e12508_d_n8, assign13140_e12508_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
        let assign13140_e12502: f64 = (-locals.var_q_qsq);
        let assign13140_e12505: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign13140_e12506: f64 = (assign13140_e12502 / assign13140_e12505);
        (assign13140_e12506, ((((-locals.var_q_qsq_dn4) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign13140_e12505 * assign13140_e12505)), ((((-locals.var_q_qsq_dn6) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign13140_e12505 * assign13140_e12505)), ((((-locals.var_q_qsq_dn7) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign13140_e12505 * assign13140_e12505)), ((((-locals.var_q_qsq_dn8) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign13140_e12505 * assign13140_e12505)), ((((-locals.var_q_qsq_dn9) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign13140_e12505 * assign13140_e12505)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13140_e12508;
        locals.var_q_sh_term_dn4 = assign13140_e12508_d_n4;
        locals.var_q_sh_term_dn6 = assign13140_e12508_d_n6;
        locals.var_q_sh_term_dn7 = assign13140_e12508_d_n7;
        locals.var_q_sh_term_dn8 = assign13140_e12508_d_n8;
        locals.var_q_sh_term_dn9 = assign13140_e12508_d_n9;

        let (assign13150_e12516, assign13150_e12516_d_n4, assign13150_e12516_d_n6, assign13150_e12516_d_n7, assign13150_e12516_d_n8, assign13150_e12516_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
        let assign13150_e12514: f64 = (locals.var_q_sh_term).ln();
        (assign13150_e12514, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13150_e12516;
        locals.var_q_ln_term_dn4 = assign13150_e12516_d_n4;
        locals.var_q_ln_term_dn6 = assign13150_e12516_d_n6;
        locals.var_q_ln_term_dn7 = assign13150_e12516_d_n7;
        locals.var_q_ln_term_dn8 = assign13150_e12516_d_n8;
        locals.var_q_ln_term_dn9 = assign13150_e12516_d_n9;

        let (assign13160_e12540, assign13160_e12540_d_n4, assign13160_e12540_d_n6, assign13160_e12540_d_n7, assign13160_e12540_d_n8, assign13160_e12540_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 == 0.0)) {
        let assign13160_e12525: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign13160_e12529: f64 = (0.05 * locals.var_q_qsq);
        let assign13160_e12533: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign13160_e12534: f64 = (1.0 - assign13160_e12533);
        let assign13160_e12535: f64 = (assign13160_e12529 * assign13160_e12534);
        let assign13160_e12536: f64 = (1.0 - assign13160_e12535);
        let assign13160_e12537: f64 = (assign13160_e12525 * assign13160_e12536);
        let assign13160_e12538: f64 = (4.0 - assign13160_e12537);
        (assign13160_e12538, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13160_e12540;
        locals.var_q_sh_term_dn4 = assign13160_e12540_d_n4;
        locals.var_q_sh_term_dn6 = assign13160_e12540_d_n6;
        locals.var_q_sh_term_dn7 = assign13160_e12540_d_n7;
        locals.var_q_sh_term_dn8 = assign13160_e12540_d_n8;
        locals.var_q_sh_term_dn9 = assign13160_e12540_d_n9;

        let (assign13170_e12549, assign13170_e12549_d_n4, assign13170_e12549_d_n6, assign13170_e12549_d_n7, assign13170_e12549_d_n8, assign13170_e12549_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 == 0.0)) {
        let assign13170_e12547: f64 = (locals.var_q_sh_term).ln();
        (assign13170_e12547, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13170_e12549;
        locals.var_q_ln_term_dn4 = assign13170_e12549_d_n4;
        locals.var_q_ln_term_dn6 = assign13170_e12549_d_n6;
        locals.var_q_ln_term_dn7 = assign13170_e12549_d_n7;
        locals.var_q_ln_term_dn8 = assign13170_e12549_d_n8;
        locals.var_q_ln_term_dn9 = assign13170_e12549_d_n9;

        let assign13180_e12552: f64 = (1.01 * locals.var_q_k1q1);
        let assign13180_e12554: f64 = (assign13180_e12552 + locals.var_q_qcoth);
        let assign13180_e12556: f64 = if assign13180_e12554 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign13180_e12556;

        let (assign13190_e12562, assign13190_e12562_d_n4, assign13190_e12562_d_n6, assign13190_e12562_d_n7, assign13190_e12562_d_n8, assign13190_e12562_d_n9,) = {
    if (locals.var_guard570 != 0.0) {
        let assign13190_e12560: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign13190_e12560, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign13190_e12562;
        locals.var_q_expnum_dn4 = assign13190_e12562_d_n4;
        locals.var_q_expnum_dn6 = assign13190_e12562_d_n6;
        locals.var_q_expnum_dn7 = assign13190_e12562_d_n7;
        locals.var_q_expnum_dn8 = assign13190_e12562_d_n8;
        locals.var_q_expnum_dn9 = assign13190_e12562_d_n9;

        let (assign13200_e12568, assign13200_e12568_d_n4, assign13200_e12568_d_n6, assign13200_e12568_d_n7, assign13200_e12568_d_n8, assign13200_e12568_d_n9,) = {
    if (locals.var_guard570 != 0.0) {
        let assign13200_e12566: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign13200_e12566, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign13200_e12568;
        locals.var_q_d1_expnum_dn4 = assign13200_e12568_d_n4;
        locals.var_q_d1_expnum_dn6 = assign13200_e12568_d_n6;
        locals.var_q_d1_expnum_dn7 = assign13200_e12568_d_n7;
        locals.var_q_d1_expnum_dn8 = assign13200_e12568_d_n8;
        locals.var_q_d1_expnum_dn9 = assign13200_e12568_d_n9;

        let (assign13210_e12572, assign13210_e12572_d_n4, assign13210_e12572_d_n6, assign13210_e12572_d_n7, assign13210_e12572_d_n8, assign13210_e12572_d_n9,) = {
    if (locals.var_guard570 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign13210_e12572;
        locals.var_q_d2_expnum_dn4 = assign13210_e12572_d_n4;
        locals.var_q_d2_expnum_dn6 = assign13210_e12572_d_n6;
        locals.var_q_d2_expnum_dn7 = assign13210_e12572_d_n7;
        locals.var_q_d2_expnum_dn8 = assign13210_e12572_d_n8;
        locals.var_q_d2_expnum_dn9 = assign13210_e12572_d_n9;

        let (assign13220_e12581, assign13220_e12581_d_n4, assign13220_e12581_d_n6, assign13220_e12581_d_n7, assign13220_e12581_d_n8, assign13220_e12581_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13220_e12578: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign13220_e12579: f64 = (1.0 / assign13220_e12578);
        (assign13220_e12579, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign13220_e12578 * assign13220_e12578))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign13220_e12578 * assign13220_e12578))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign13220_e12578 * assign13220_e12578))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign13220_e12578 * assign13220_e12578))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign13220_e12578 * assign13220_e12578))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13220_e12581;
        locals.var_q_temp2_dn4 = assign13220_e12581_d_n4;
        locals.var_q_temp2_dn6 = assign13220_e12581_d_n6;
        locals.var_q_temp2_dn7 = assign13220_e12581_d_n7;
        locals.var_q_temp2_dn8 = assign13220_e12581_d_n8;
        locals.var_q_temp2_dn9 = assign13220_e12581_d_n9;

    }

    pub(super) fn stamp_transient_block_31(
        locals: &mut StampLocals,
    ) {
        let (assign13230_e12588, assign13230_e12588_d_n4, assign13230_e12588_d_n6, assign13230_e12588_d_n7, assign13230_e12588_d_n8, assign13230_e12588_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13230_e12586: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign13230_e12586, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign13230_e12588;
        locals.var_q_temp3_dn4 = assign13230_e12588_d_n4;
        locals.var_q_temp3_dn6 = assign13230_e12588_d_n6;
        locals.var_q_temp3_dn7 = assign13230_e12588_d_n7;
        locals.var_q_temp3_dn8 = assign13230_e12588_d_n8;
        locals.var_q_temp3_dn9 = assign13230_e12588_d_n9;

        let (assign13240_e12597, assign13240_e12597_d_n4, assign13240_e12597_d_n6, assign13240_e12597_d_n7, assign13240_e12597_d_n8, assign13240_e12597_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13240_e12593: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign13240_e12595: f64 = (assign13240_e12593 * locals.var_q_temp2);
        (assign13240_e12595, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign13240_e12597;
        locals.var_q_expnum_dn4 = assign13240_e12597_d_n4;
        locals.var_q_expnum_dn6 = assign13240_e12597_d_n6;
        locals.var_q_expnum_dn7 = assign13240_e12597_d_n7;
        locals.var_q_expnum_dn8 = assign13240_e12597_d_n8;
        locals.var_q_expnum_dn9 = assign13240_e12597_d_n9;

        let (assign13250_e12612, assign13250_e12612_d_n4, assign13250_e12612_d_n6, assign13250_e12612_d_n7, assign13250_e12612_d_n8, assign13250_e12612_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13250_e12602: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign13250_e12604: f64 = (assign13250_e12602 - locals.var_q_aexp);
        let assign13250_e12607: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign13250_e12608: f64 = (assign13250_e12604 - assign13250_e12607);
        let assign13250_e12610: f64 = (assign13250_e12608 * locals.var_q_temp2);
        (assign13250_e12610, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign13250_e12612;
        locals.var_q_d1_expnum_dn4 = assign13250_e12612_d_n4;
        locals.var_q_d1_expnum_dn6 = assign13250_e12612_d_n6;
        locals.var_q_d1_expnum_dn7 = assign13250_e12612_d_n7;
        locals.var_q_d1_expnum_dn8 = assign13250_e12612_d_n8;
        locals.var_q_d1_expnum_dn9 = assign13250_e12612_d_n9;

        let (assign13260_e12637, assign13260_e12637_d_n4, assign13260_e12637_d_n6, assign13260_e12637_d_n7, assign13260_e12637_d_n8, assign13260_e12637_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13260_e12617: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign13260_e12620: f64 = (2.0 * locals.var_q_temp3);
        let assign13260_e12622: f64 = (assign13260_e12620 * locals.var_q_d1_expnum);
        let assign13260_e12623: f64 = (assign13260_e12617 + assign13260_e12622);
        let assign13260_e12625: f64 = (assign13260_e12623 + locals.var_q_aexp);
        let assign13260_e12629: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign13260_e12630: f64 = (locals.var_q_d2_ln + assign13260_e12629);
        let assign13260_e12632: f64 = (assign13260_e12630 * locals.var_q_sh_term);
        let assign13260_e12633: f64 = (assign13260_e12625 - assign13260_e12632);
        let assign13260_e12635: f64 = (assign13260_e12633 * locals.var_q_temp2);
        (assign13260_e12635, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign13260_e12637;
        locals.var_q_d2_expnum_dn4 = assign13260_e12637_d_n4;
        locals.var_q_d2_expnum_dn6 = assign13260_e12637_d_n6;
        locals.var_q_d2_expnum_dn7 = assign13260_e12637_d_n7;
        locals.var_q_d2_expnum_dn8 = assign13260_e12637_d_n8;
        locals.var_q_d2_expnum_dn9 = assign13260_e12637_d_n9;

        let assign13270_e12640: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign13270_e12640;

        let (assign13280_e12645, assign13280_e12645_d_n4, assign13280_e12645_d_n6, assign13280_e12645_d_n7, assign13280_e12645_d_n8, assign13280_e12645_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13280_e12643: f64 = (locals.var_q_expnum).ln();
        (assign13280_e12643, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign13280_e12645;
        locals.var_q_lnexpnum_dn4 = assign13280_e12645_d_n4;
        locals.var_q_lnexpnum_dn6 = assign13280_e12645_d_n6;
        locals.var_q_lnexpnum_dn7 = assign13280_e12645_d_n7;
        locals.var_q_lnexpnum_dn8 = assign13280_e12645_d_n8;
        locals.var_q_lnexpnum_dn9 = assign13280_e12645_d_n9;

        let (assign13290_e12651, assign13290_e12651_d_n4, assign13290_e12651_d_n6, assign13290_e12651_d_n7, assign13290_e12651_d_n8, assign13290_e12651_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13290_e12649: f64 = (1.0 / locals.var_q_expnum);
        (assign13290_e12649, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13290_e12651;
        locals.var_q_temp1_dn4 = assign13290_e12651_d_n4;
        locals.var_q_temp1_dn6 = assign13290_e12651_d_n6;
        locals.var_q_temp1_dn7 = assign13290_e12651_d_n7;
        locals.var_q_temp1_dn8 = assign13290_e12651_d_n8;
        locals.var_q_temp1_dn9 = assign13290_e12651_d_n9;

        let (assign13300_e12657, assign13300_e12657_d_n4, assign13300_e12657_d_n6, assign13300_e12657_d_n7, assign13300_e12657_d_n8, assign13300_e12657_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13300_e12655: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign13300_e12655, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign13300_e12657;
        locals.var_q_d1_lnexpnum_dn4 = assign13300_e12657_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign13300_e12657_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign13300_e12657_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign13300_e12657_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign13300_e12657_d_n9;

        let (assign13310_e12667, assign13310_e12667_d_n4, assign13310_e12667_d_n6, assign13310_e12667_d_n7, assign13310_e12667_d_n8, assign13310_e12667_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13310_e12661: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign13310_e12664: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign13310_e12665: f64 = (assign13310_e12661 - assign13310_e12664);
        (assign13310_e12665, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign13310_e12667;
        locals.var_q_d2_lnexpnum_dn4 = assign13310_e12667_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign13310_e12667_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign13310_e12667_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign13310_e12667_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign13310_e12667_d_n9;

        let (assign13320_e12678, assign13320_e12678_d_n4, assign13320_e12678_d_n6, assign13320_e12678_d_n7, assign13320_e12678_d_n8, assign13320_e12678_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13320_e12672: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign13320_e12674: f64 = (-locals.var_q_k1q1);
        let assign13320_e12675: f64 = (assign13320_e12674).ln();
        let assign13320_e12676: f64 = (assign13320_e12672 + assign13320_e12675);
        (assign13320_e12676, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign13320_e12674)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign13320_e12674)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign13320_e12674)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign13320_e12674)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign13320_e12674)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign13320_e12678;
        locals.var_q_lnexpnum_dn4 = assign13320_e12678_d_n4;
        locals.var_q_lnexpnum_dn6 = assign13320_e12678_d_n6;
        locals.var_q_lnexpnum_dn7 = assign13320_e12678_d_n7;
        locals.var_q_lnexpnum_dn8 = assign13320_e12678_d_n8;
        locals.var_q_lnexpnum_dn9 = assign13320_e12678_d_n9;

        let (assign13330_e12685, assign13330_e12685_d_n4, assign13330_e12685_d_n6, assign13330_e12685_d_n7, assign13330_e12685_d_n8, assign13330_e12685_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13330_e12683: f64 = (1.0 / locals.var_q1s);
        (assign13330_e12683, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13330_e12685;
        locals.var_q_temp1_dn4 = assign13330_e12685_d_n4;
        locals.var_q_temp1_dn6 = assign13330_e12685_d_n6;
        locals.var_q_temp1_dn7 = assign13330_e12685_d_n7;
        locals.var_q_temp1_dn8 = assign13330_e12685_d_n8;
        locals.var_q_temp1_dn9 = assign13330_e12685_d_n9;

        let (assign13340_e12692, assign13340_e12692_d_n4, assign13340_e12692_d_n6, assign13340_e12692_d_n7, assign13340_e12692_d_n8, assign13340_e12692_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13340_e12690: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign13340_e12690, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign13340_e12692;
        locals.var_q_d1_lnexpnum_dn4 = assign13340_e12692_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign13340_e12692_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign13340_e12692_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign13340_e12692_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign13340_e12692_d_n9;

        let (assign13350_e12700, assign13350_e12700_d_n4, assign13350_e12700_d_n6, assign13350_e12700_d_n7, assign13350_e12700_d_n8, assign13350_e12700_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13350_e12696: f64 = (-locals.var_q_temp1);
        let assign13350_e12698: f64 = (assign13350_e12696 * locals.var_q_temp1);
        (assign13350_e12698, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign13350_e12700;
        locals.var_q_d2_lnexpnum_dn4 = assign13350_e12700_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign13350_e12700_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign13350_e12700_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign13350_e12700_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign13350_e12700_d_n9;

        let assign13360_e12703: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign13360_e12705: f64 = (assign13360_e12703 + locals.var_q1s);
        let assign13360_e12708: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign13360_e12709: f64 = (assign13360_e12705 + assign13360_e12708);
        let assign13360_e12711: f64 = (assign13360_e12709 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign13360_e12711;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign13370_e12715: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign13370_e12716: f64 = (1.0 + assign13370_e12715);
        let assign13370_e12718: f64 = (assign13370_e12716 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign13370_e12718;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign13380_e12721: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign13380_e12723: f64 = (assign13380_e12721 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign13380_e12723;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign13390_e12727: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign13390_e12728: f64 = (locals.var_q_k1q1 + assign13390_e12727);
        locals.var_q_qi_int = assign13390_e12728;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign13400_e12732: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign13400_e12733: f64 = (locals.var_k1 + assign13400_e12732);
        locals.var_q_d1_qi = assign13400_e12733;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign13410_e12736: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign13410_e12736;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign13420_e12739: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign13420_e12741: f64 = (assign13420_e12739 - locals.var_q_aexp);
        locals.var_q_zero = assign13420_e12741;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign13430_e12744: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign13430_e12747: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign13430_e12748: f64 = (assign13430_e12744 + assign13430_e12747);
        let assign13430_e12750: f64 = (assign13430_e12748 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign13430_e12750;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign13440_e12753: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign13440_e12756: f64 = (2.0 * locals.var_q_d1_qi);
        let assign13440_e12758: f64 = (assign13440_e12756 * locals.var_q_d1_expnum);
        let assign13440_e12759: f64 = (assign13440_e12753 + assign13440_e12758);
        let assign13440_e12762: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign13440_e12763: f64 = (assign13440_e12759 + assign13440_e12762);
        let assign13440_e12765: f64 = (assign13440_e12763 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign13440_e12765;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign13450_e12768: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign13450_e12771: f64 = (0.5 * locals.var_q_zero);
        let assign13450_e12773: f64 = (assign13450_e12771 * locals.var_q_d2_zero);
        let assign13450_e12774: f64 = (assign13450_e12768 - assign13450_e12773);
        locals.var_q_temp = assign13450_e12774;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn9)));

        let assign13460_e12776: f64 = (-locals.var_q_zero);
        let assign13460_e12778: f64 = (assign13460_e12776 * locals.var_q_d1_zero);
        let assign13460_e12780: f64 = (assign13460_e12778 * locals.var_q_temp);
        let assign13460_e12783: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign13460_e12785: f64 = (assign13460_e12783 + 1e-200);
        let assign13460_e12786: f64 = (assign13460_e12780 / assign13460_e12785);
        locals.var_q_eps2 = assign13460_e12786;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn4)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign13460_e12785 * assign13460_e12785));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn6)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign13460_e12785 * assign13460_e12785));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn7)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign13460_e12785 * assign13460_e12785));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn8)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign13460_e12785 * assign13460_e12785));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn9)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign13460_e12785 * assign13460_e12785));

        let assign13470_e12789: f64 = (locals.var_q1s + locals.var_q_eps2);
        locals.var_q1s = assign13470_e12789;
        locals.var_q1s_dn4 = (locals.var_q1s_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1s_dn6 = (locals.var_q1s_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1s_dn7 = (locals.var_q1s_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1s_dn8 = (locals.var_q1s_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1s_dn9 = (locals.var_q1s_dn9 + locals.var_q_eps2_dn9);

        let assign13480_e12792: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign13480_e12792;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign13490_e12795: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13490_e12797: f64 = assign13490_e12795;
        let assign13490_e12799: f64 = if assign13490_e12797 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign13490_e12799;

        let (assign13500_e12808, assign13500_e12808_d_n4, assign13500_e12808_d_n6, assign13500_e12808_d_n7, assign13500_e12808_d_n8, assign13500_e12808_d_n9,) = {
    if (locals.var_guard572 != 0.0) {
        let assign13500_e12803: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13500_e12805: f64 = assign13500_e12803;
        let assign13500_e12806: f64 = (assign13500_e12805).exp();
        (assign13500_e12806, (assign13500_e12806 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign13500_e12806 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign13500_e12806 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign13500_e12806 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign13500_e12806 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13500_e12808;
        locals.var_q_temp1_dn4 = assign13500_e12808_d_n4;
        locals.var_q_temp1_dn6 = assign13500_e12808_d_n6;
        locals.var_q_temp1_dn7 = assign13500_e12808_d_n7;
        locals.var_q_temp1_dn8 = assign13500_e12808_d_n8;
        locals.var_q_temp1_dn9 = assign13500_e12808_d_n9;

        let (assign13510_e12847, assign13510_e12847_d_n4, assign13510_e12847_d_n6, assign13510_e12847_d_n7, assign13510_e12847_d_n8, assign13510_e12847_d_n9,) = {
    if (locals.var_guard572 == 0.0) {
        let assign13510_e12815: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13510_e12817: f64 = assign13510_e12815;
        let assign13510_e12819: f64 = (assign13510_e12817 - 80.0);
        let assign13510_e12824: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13510_e12826: f64 = assign13510_e12824;
        let assign13510_e12828: f64 = (assign13510_e12826 - 80.0);
        let assign13510_e12829: f64 = (0.5 * assign13510_e12828);
        let assign13510_e12833: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13510_e12835: f64 = assign13510_e12833;
        let assign13510_e12837: f64 = (assign13510_e12835 - 80.0);
        let assign13510_e12839: f64 = (assign13510_e12837 * 0.3333333333333);
        let assign13510_e12840: f64 = (1.0 + assign13510_e12839);
        let assign13510_e12841: f64 = (assign13510_e12829 * assign13510_e12840);
        let assign13510_e12842: f64 = (1.0 + assign13510_e12841);
        let assign13510_e12843: f64 = (assign13510_e12819 * assign13510_e12842);
        let assign13510_e12844: f64 = (1.0 + assign13510_e12843);
        let assign13510_e12845: f64 = (5.54062e34 * assign13510_e12844);
        (assign13510_e12845, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13510_e12847;
        locals.var_q_temp1_dn4 = assign13510_e12847_d_n4;
        locals.var_q_temp1_dn6 = assign13510_e12847_d_n6;
        locals.var_q_temp1_dn7 = assign13510_e12847_d_n7;
        locals.var_q_temp1_dn8 = assign13510_e12847_d_n8;
        locals.var_q_temp1_dn9 = assign13510_e12847_d_n9;

        let assign13520_e12850: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign13520_e12850;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign13530_e12853: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign13530_e12855: f64 = (assign13530_e12853 - locals.var_q_aexp);
        locals.var_q_qsq = assign13530_e12855;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign13540_e12858: f64 = (2.0 * locals.var_k1);
        let assign13540_e12860: f64 = (assign13540_e12858 * locals.var_q_k1q1);
        let assign13540_e12862: f64 = (assign13540_e12860 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign13540_e12862;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign13550_e12865: f64 = (2.0 * locals.var_k1);
        let assign13550_e12867: f64 = (assign13550_e12865 * locals.var_k1);
        let assign13550_e12869: f64 = (assign13550_e12867 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign13550_e12869;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign13560_e12872: f64 = (-0.005);
        let assign13560_e12873: f64 = if locals.var_q_qsq < assign13560_e12872 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign13560_e12873;

        let (assign13570_e12879, assign13570_e12879_d_n4, assign13570_e12879_d_n6, assign13570_e12879_d_n7, assign13570_e12879_d_n8, assign13570_e12879_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13570_e12876: f64 = (locals.var_q_qsq).abs();
        let assign13570_e12877: f64 = (assign13570_e12876).sqrt();
        (assign13570_e12877, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13570_e12877)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign13570_e12879;
        locals.var_q_rac_qsq_dn4 = assign13570_e12879_d_n4;
        locals.var_q_rac_qsq_dn6 = assign13570_e12879_d_n6;
        locals.var_q_rac_qsq_dn7 = assign13570_e12879_d_n7;
        locals.var_q_rac_qsq_dn8 = assign13570_e12879_d_n8;
        locals.var_q_rac_qsq_dn9 = assign13570_e12879_d_n9;

        let (assign13580_e12888, assign13580_e12888_d_n4, assign13580_e12888_d_n6, assign13580_e12888_d_n7, assign13580_e12888_d_n8, assign13580_e12888_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13580_e12884: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign13580_e12885: f64 = (assign13580_e12884).tan();
        let assign13580_e12886: f64 = (locals.var_q_rac_qsq / assign13580_e12885);
        (assign13580_e12886, (((locals.var_q_rac_qsq_dn4 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn6 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn7 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn8 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn9 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13580_e12888;
        locals.var_q_qcoth_dn4 = assign13580_e12888_d_n4;
        locals.var_q_qcoth_dn6 = assign13580_e12888_d_n6;
        locals.var_q_qcoth_dn7 = assign13580_e12888_d_n7;
        locals.var_q_qcoth_dn8 = assign13580_e12888_d_n8;
        locals.var_q_qcoth_dn9 = assign13580_e12888_d_n9;

        let (assign13590_e12896, assign13590_e12896_d_n4, assign13590_e12896_d_n6, assign13590_e12896_d_n7, assign13590_e12896_d_n8, assign13590_e12896_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13590_e12892: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign13590_e12894: f64 = (assign13590_e12892 / locals.var_q_qsq);
        (assign13590_e12894, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13590_e12896;
        locals.var_q_temp1_dn4 = assign13590_e12896_d_n4;
        locals.var_q_temp1_dn6 = assign13590_e12896_d_n6;
        locals.var_q_temp1_dn7 = assign13590_e12896_d_n7;
        locals.var_q_temp1_dn8 = assign13590_e12896_d_n8;
        locals.var_q_temp1_dn9 = assign13590_e12896_d_n9;

        let (assign13600_e12908, assign13600_e12908_d_n4, assign13600_e12908_d_n6, assign13600_e12908_d_n7, assign13600_e12908_d_n8, assign13600_e12908_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13600_e12902: f64 = (2.0 - locals.var_q_qcoth);
        let assign13600_e12903: f64 = (locals.var_q_qcoth * assign13600_e12902);
        let assign13600_e12904: f64 = (locals.var_q_qsq + assign13600_e12903);
        let assign13600_e12906: f64 = (assign13600_e12904 * locals.var_q_temp1);
        (assign13600_e12906, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13600_e12908;
        locals.var_q_d1_qcoth_dn4 = assign13600_e12908_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13600_e12908_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13600_e12908_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13600_e12908_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13600_e12908_d_n9;

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7230_e6963, assign7230_e6963_d_n4, assign7230_e6963_d_n6, assign7230_e6963_d_n7, assign7230_e6963_d_n8, assign7230_e6963_d_n9,) = {
    if (locals.var_guard144 != 0.0) {
        let assign7230_e6944: f64 = (2970.0 / locals.var_tkd);
        let assign7230_e6945: f64 = (15.0 + assign7230_e6944);
        let assign7230_e6949: f64 = (2970.0 / locals.var_tkd);
        let assign7230_e6950: f64 = (15.0 - assign7230_e6949);
        let assign7230_e6954: f64 = (2970.0 / locals.var_tkd);
        let assign7230_e6955: f64 = (15.0 - assign7230_e6954);
        let assign7230_e6956: f64 = (assign7230_e6950 * assign7230_e6955);
        let assign7230_e6958: f64 = (assign7230_e6956 + 1e-6);
        let assign7230_e6959: f64 = (assign7230_e6958).sqrt();
        let assign7230_e6960: f64 = (assign7230_e6945 + assign7230_e6959);
        let assign7230_e6961: f64 = (0.5 * assign7230_e6960);
        (assign7230_e6961, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))),)
    } else {
        (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9,)
    }
};
        locals.var_emin = assign7230_e6963;
        locals.var_emin_dn4 = assign7230_e6963_d_n4;
        locals.var_emin_dn6 = assign7230_e6963_d_n6;
        locals.var_emin_dn7 = assign7230_e6963_d_n7;
        locals.var_emin_dn8 = assign7230_e6963_d_n8;
        locals.var_emin_dn9 = assign7230_e6963_d_n9;

        locals.var_dvfbqm = 0.0;

        locals.var_qq = 0.0;
        locals.var_qq_dn4 = 0.0;
        locals.var_qq_dn6 = 0.0;
        locals.var_qq_dn7 = 0.0;
        locals.var_qq_dn8 = 0.0;
        locals.var_qq_dn9 = 0.0;

        let assign7260_e6968: f64 = (1e18 * locals.var_tsi_i);
        let assign7260_e6970: f64 = (assign7260_e6968 * locals.var_tsi_i);
        locals.var_tsisq = assign7260_e6970;

        let assign7270_e6973: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7270_e6973;

        let assign7280_e6976: f64 = 1.0;
        let assign7280_e6977: f64 = if p.p14 == assign7280_e6976 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7280_e6977;

        let (assign7290_e6985,) = {
    if ((locals.var_guard145 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7290_e6983: f64 = (0.409618895 / locals.var_tsisq);
        (assign7290_e6983,)
    } else {
        (locals.var_dvfbqm,)
    }
};
        locals.var_dvfbqm = assign7290_e6985;

        let (assign7300_e7004, assign7300_e7004_d_n4, assign7300_e7004_d_n6, assign7300_e7004_d_n7, assign7300_e7004_d_n8, assign7300_e7004_d_n9,) = {
    if ((locals.var_guard145 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7300_e6991: f64 = (0.4 * p.p13);
        let assign7300_e6993: f64 = (assign7300_e6991 * 1.27520989);
        let assign7300_e6995: f64 = (-0.3333333333333);
        let assign7300_e6998: f64 = (locals.var_phit * locals.var_tsisq);
        let assign7300_e6999: f64 = (assign7300_e6998).ln();
        let assign7300_e7000: f64 = (assign7300_e6995 * assign7300_e6999);
        let assign7300_e7001: f64 = (assign7300_e7000).exp();
        let assign7300_e7002: f64 = (assign7300_e6993 * assign7300_e7001);
        (assign7300_e7002, (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign7300_e6998)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign7300_e7004;
        locals.var_qq_dn4 = assign7300_e7004_d_n4;
        locals.var_qq_dn6 = assign7300_e7004_d_n6;
        locals.var_qq_dn7 = assign7300_e7004_d_n7;
        locals.var_qq_dn8 = assign7300_e7004_d_n8;
        locals.var_qq_dn9 = assign7300_e7004_d_n9;

        let (assign7310_e7013,) = {
    if ((locals.var_guard145 != 0.0) && (locals.var_guard146 == 0.0)) {
        let assign7310_e7011: f64 = (0.723134895 / locals.var_tsisq);
        (assign7310_e7011,)
    } else {
        (locals.var_dvfbqm,)
    }
};
        locals.var_dvfbqm = assign7310_e7013;

        let (assign7320_e7033, assign7320_e7033_d_n4, assign7320_e7033_d_n6, assign7320_e7033_d_n7, assign7320_e7033_d_n8, assign7320_e7033_d_n9,) = {
    if ((locals.var_guard145 != 0.0) && (locals.var_guard146 == 0.0)) {
        let assign7320_e7020: f64 = (0.4 * p.p13);
        let assign7320_e7022: f64 = (assign7320_e7020 * 1.5412087);
        let assign7320_e7024: f64 = (-0.3333333333333);
        let assign7320_e7027: f64 = (locals.var_phit * locals.var_tsisq);
        let assign7320_e7028: f64 = (assign7320_e7027).ln();
        let assign7320_e7029: f64 = (assign7320_e7024 * assign7320_e7028);
        let assign7320_e7030: f64 = (assign7320_e7029).exp();
        let assign7320_e7031: f64 = (assign7320_e7022 * assign7320_e7030);
        (assign7320_e7031, (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign7320_e7027)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign7320_e7033;
        locals.var_qq_dn4 = assign7320_e7033_d_n4;
        locals.var_qq_dn6 = assign7320_e7033_d_n6;
        locals.var_qq_dn7 = assign7320_e7033_d_n7;
        locals.var_qq_dn8 = assign7320_e7033_d_n8;
        locals.var_qq_dn9 = assign7320_e7033_d_n9;

        let assign7330_e7036: f64 = (p.p14 * locals.var_stvfb_i);
        let assign7330_e7038: f64 = (assign7330_e7036 * locals.var_dt);
        let assign7330_e7040: f64 = (assign7330_e7038 + locals.var_dvfbqm);
        locals.var_temp = assign7330_e7040;
        locals.var_temp_dn4 = (assign7330_e7036 * locals.var_dt_dn4);
        locals.var_temp_dn6 = (assign7330_e7036 * locals.var_dt_dn6);
        locals.var_temp_dn7 = (assign7330_e7036 * locals.var_dt_dn7);
        locals.var_temp_dn8 = (assign7330_e7036 * locals.var_dt_dn8);
        locals.var_temp_dn9 = (assign7330_e7036 * locals.var_dt_dn9);

        let assign7340_e7043: f64 = (locals.var_temp + p.p34);
        let assign7340_e7045: f64 = (assign7340_e7043 - locals.var_dvfbpdep);
        locals.var_temp1 = assign7340_e7045;
        locals.var_temp1_dn4 = (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4);
        locals.var_temp1_dn6 = (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6);
        locals.var_temp1_dn7 = (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7);
        locals.var_temp1_dn8 = (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8);
        locals.var_temp1_dn9 = (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9);

        let assign7350_e7049: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
        let assign7350_e7051: f64 = (assign7350_e7049 + locals.var_dvfb1nch);
        let assign7350_e7052: f64 = (p.p14 * assign7350_e7051);
        let assign7350_e7054: f64 = (assign7350_e7052 + locals.var_temp1);
        locals.var_vfb1_i = assign7350_e7054;
        locals.var_vfb1_i_dn4 = ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4);
        locals.var_vfb1_i_dn6 = ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6);
        locals.var_vfb1_i_dn7 = ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7);
        locals.var_vfb1_i_dn8 = ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8);
        locals.var_vfb1_i_dn9 = ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9);

        let assign7360_e7058: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
        let assign7360_e7060: f64 = (assign7360_e7058 + locals.var_dvfb2nch);
        let assign7360_e7061: f64 = (p.p14 * assign7360_e7060);
        let assign7360_e7063: f64 = (assign7360_e7061 + locals.var_temp);
        locals.var_vfb2_i = assign7360_e7063;
        locals.var_vfb2_i_dn4 = ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfb2_i_dn6 = ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfb2_i_dn7 = ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfb2_i_dn8 = ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfb2_i_dn9 = ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);

        let assign7370_e7067: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
        let assign7370_e7069: f64 = (assign7370_e7067 + locals.var_dvfb1nch);
        let assign7370_e7070: f64 = (p.p14 * assign7370_e7069);
        let assign7370_e7072: f64 = (assign7370_e7070 + locals.var_temp1);
        locals.var_vfbac1_i = assign7370_e7072;
        locals.var_vfbac1_i_dn4 = ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4);
        locals.var_vfbac1_i_dn6 = ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6);
        locals.var_vfbac1_i_dn7 = ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7);
        locals.var_vfbac1_i_dn8 = ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8);
        locals.var_vfbac1_i_dn9 = ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9);

        let assign7380_e7076: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
        let assign7380_e7078: f64 = (assign7380_e7076 + locals.var_dvfb2nch);
        let assign7380_e7079: f64 = (p.p14 * assign7380_e7078);
        let assign7380_e7081: f64 = (assign7380_e7079 + locals.var_temp);
        locals.var_vfbac2_i = assign7380_e7081;
        locals.var_vfbac2_i_dn4 = ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfbac2_i_dn6 = ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfbac2_i_dn7 = ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfbac2_i_dn8 = ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfbac2_i_dn9 = ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);

        let assign7390_e7083: f64 = (locals.var_rtn).ln();
        locals.var_lnrtn = assign7390_e7083;
        locals.var_lnrtn_dn4 = (locals.var_rtn_dn4 / locals.var_rtn);
        locals.var_lnrtn_dn6 = (locals.var_rtn_dn6 / locals.var_rtn);
        locals.var_lnrtn_dn7 = (locals.var_rtn_dn7 / locals.var_rtn);
        locals.var_lnrtn_dn8 = (locals.var_rtn_dn8 / locals.var_rtn);
        locals.var_lnrtn_dn9 = (locals.var_rtn_dn9 / locals.var_rtn);

        let assign7400_e7086: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign7400_e7087: f64 = (assign7400_e7086).exp();
        let assign7400_e7089: f64 = (assign7400_e7087 * p.p35);
        locals.var_tf_bet = assign7400_e7089;
        locals.var_tf_bet_dn4 = ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35);
        locals.var_tf_bet_dn6 = ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35);
        locals.var_tf_bet_dn7 = ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35);
        locals.var_tf_bet_dn8 = ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35);
        locals.var_tf_bet_dn9 = ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35);

        let assign7410_e7092: f64 = (locals.var_betn1_t * locals.var_tf_bet);
        locals.var_betn1_i = assign7410_e7092;
        locals.var_betn1_i_dn4 = ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4));
        locals.var_betn1_i_dn6 = ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6));
        locals.var_betn1_i_dn7 = ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7));
        locals.var_betn1_i_dn8 = ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8));
        locals.var_betn1_i_dn9 = ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9));

        let assign7420_e7095: f64 = (locals.var_betn2_t * locals.var_tf_bet);
        locals.var_betn2_i = assign7420_e7095;
        locals.var_betn2_i_dn4 = ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4));
        locals.var_betn2_i_dn6 = ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6));
        locals.var_betn2_i_dn7 = ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7));
        locals.var_betn2_i_dn8 = ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8));
        locals.var_betn2_i_dn9 = ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9));

        let assign7430_e7098: f64 = (locals.var_stmue_i * locals.var_lnrtn);
        let assign7430_e7099: f64 = (assign7430_e7098).exp();
        locals.var_tf_mue = assign7430_e7099;
        locals.var_tf_mue_dn4 = (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn4));
        locals.var_tf_mue_dn6 = (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn6));
        locals.var_tf_mue_dn7 = (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn7));
        locals.var_tf_mue_dn8 = (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn8));
        locals.var_tf_mue_dn9 = (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn9));

        let assign7440_e7102: f64 = (locals.var_mue_t * locals.var_tf_mue);
        locals.var_mue_i = assign7440_e7102;
        locals.var_mue_i_dn4 = (locals.var_mue_t * locals.var_tf_mue_dn4);
        locals.var_mue_i_dn6 = (locals.var_mue_t * locals.var_tf_mue_dn6);
        locals.var_mue_i_dn7 = (locals.var_mue_t * locals.var_tf_mue_dn7);
        locals.var_mue_i_dn8 = (locals.var_mue_t * locals.var_tf_mue_dn8);
        locals.var_mue_i_dn9 = (locals.var_mue_t * locals.var_tf_mue_dn9);

        let assign7450_e7105: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
        let assign7450_e7106: f64 = (assign7450_e7105).exp();
        locals.var_tf_themu = assign7450_e7106;
        locals.var_tf_themu_dn4 = (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn4));
        locals.var_tf_themu_dn6 = (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn6));
        locals.var_tf_themu_dn7 = (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn7));
        locals.var_tf_themu_dn8 = (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn8));
        locals.var_tf_themu_dn9 = (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn9));

        let assign7460_e7109: f64 = (locals.var_themu_t * locals.var_tf_themu);
        locals.var_themu_i = assign7460_e7109;
        locals.var_themu_i_dn4 = (locals.var_themu_t * locals.var_tf_themu_dn4);
        locals.var_themu_i_dn6 = (locals.var_themu_t * locals.var_tf_themu_dn6);
        locals.var_themu_i_dn7 = (locals.var_themu_t * locals.var_tf_themu_dn7);
        locals.var_themu_i_dn8 = (locals.var_themu_t * locals.var_tf_themu_dn8);
        locals.var_themu_i_dn9 = (locals.var_themu_t * locals.var_tf_themu_dn9);

        let assign7470_e7112: f64 = (locals.var_stcs_i * locals.var_lnrtn);
        let assign7470_e7113: f64 = (assign7470_e7112).exp();
        locals.var_tf_cs = assign7470_e7113;
        locals.var_tf_cs_dn4 = (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn4));
        locals.var_tf_cs_dn6 = (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn6));
        locals.var_tf_cs_dn7 = (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn7));
        locals.var_tf_cs_dn8 = (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn8));
        locals.var_tf_cs_dn9 = (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn9));

        let assign7480_e7116: f64 = (locals.var_cs_t * locals.var_tf_cs);
        locals.var_cs_i = assign7480_e7116;
        locals.var_cs_i_dn4 = (locals.var_cs_t * locals.var_tf_cs_dn4);
        locals.var_cs_i_dn6 = (locals.var_cs_t * locals.var_tf_cs_dn6);
        locals.var_cs_i_dn7 = (locals.var_cs_t * locals.var_tf_cs_dn7);
        locals.var_cs_i_dn8 = (locals.var_cs_t * locals.var_tf_cs_dn8);
        locals.var_cs_i_dn9 = (locals.var_cs_t * locals.var_tf_cs_dn9);

        let assign7490_e7119: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
        let assign7490_e7120: f64 = (assign7490_e7119).exp();
        locals.var_tf_thecs = assign7490_e7120;
        locals.var_tf_thecs_dn4 = (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn4));
        locals.var_tf_thecs_dn6 = (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn6));
        locals.var_tf_thecs_dn7 = (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn7));
        locals.var_tf_thecs_dn8 = (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn8));
        locals.var_tf_thecs_dn9 = (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn9));

        let assign7500_e7123: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
        locals.var_thecs_i = assign7500_e7123;
        locals.var_thecs_i_dn4 = (locals.var_thecs_t * locals.var_tf_thecs_dn4);
        locals.var_thecs_i_dn6 = (locals.var_thecs_t * locals.var_tf_thecs_dn6);
        locals.var_thecs_i_dn7 = (locals.var_thecs_t * locals.var_tf_thecs_dn7);
        locals.var_thecs_i_dn8 = (locals.var_thecs_t * locals.var_tf_thecs_dn8);
        locals.var_thecs_i_dn9 = (locals.var_thecs_t * locals.var_tf_thecs_dn9);

        let assign7510_e7126: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
        let assign7510_e7127: f64 = (assign7510_e7126).exp();
        locals.var_tf_xcor = assign7510_e7127;
        locals.var_tf_xcor_dn4 = (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn4));
        locals.var_tf_xcor_dn6 = (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn6));
        locals.var_tf_xcor_dn7 = (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn7));
        locals.var_tf_xcor_dn8 = (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn8));
        locals.var_tf_xcor_dn9 = (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn9));

        let assign7520_e7130: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
        locals.var_xcor_i = assign7520_e7130;
        locals.var_xcor_i_dn4 = (locals.var_xcor_t * locals.var_tf_xcor_dn4);
        locals.var_xcor_i_dn6 = (locals.var_xcor_t * locals.var_tf_xcor_dn6);
        locals.var_xcor_i_dn7 = (locals.var_xcor_t * locals.var_tf_xcor_dn7);
        locals.var_xcor_i_dn8 = (locals.var_xcor_t * locals.var_tf_xcor_dn8);
        locals.var_xcor_i_dn9 = (locals.var_xcor_t * locals.var_tf_xcor_dn9);

        let assign7530_e7133: f64 = (1e-8 * locals.var_phit);
        let assign7530_e7135: f64 = (assign7530_e7133 / locals.var_tsi_i);
        locals.var_temp = assign7530_e7135;
        locals.var_temp_dn4 = ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i);
        locals.var_temp_dn6 = ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i);
        locals.var_temp_dn7 = ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i);
        locals.var_temp_dn8 = ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i);
        locals.var_temp_dn9 = ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i);

        let assign7540_e7138: f64 = (locals.var_temp * locals.var_mue_i);
        locals.var_fmue = assign7540_e7138;
        locals.var_fmue_dn4 = ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4));
        locals.var_fmue_dn6 = ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6));
        locals.var_fmue_dn7 = ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7));
        locals.var_fmue_dn8 = ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8));
        locals.var_fmue_dn9 = ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9));

        let assign7550_e7142: f64 = (0.5 * locals.var_csthr_i);
        let assign7550_e7143: f64 = (1.0 / assign7550_e7142);
        locals.var_inv_qi1cs = assign7550_e7143;

        let assign7560_e7146: f64 = (locals.var_inv_qi1cs / locals.var_csthrb_i);
        locals.var_inv_qi2cs = assign7560_e7146;

        let assign7570_e7149: f64 = 1.0;
        let assign7570_e7150: f64 = if p.p14 == assign7570_e7149 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7570_e7150;

        let (assign7580_e7156,) = {
    if (locals.var_guard147 != 0.0) {
        let assign7580_e7154: f64 = (0.5 * locals.var_feta_i);
        (assign7580_e7154,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign7580_e7156;

        let (assign7590_e7163,) = {
    if (locals.var_guard147 == 0.0) {
        let assign7590_e7161: f64 = (0.3333333333333 * locals.var_feta_i);
        (assign7590_e7161,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign7590_e7163;

        let assign7600_e7166: f64 = (1.0 - locals.var_eta_mu);
        locals.var_one_m_eta = assign7600_e7166;

        let assign7610_e7169: f64 = (locals.var_strs_i * locals.var_lnrtn);
        let assign7610_e7170: f64 = (assign7610_e7169).exp();
        locals.var_tf_ther = assign7610_e7170;
        locals.var_tf_ther_dn4 = (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn4));
        locals.var_tf_ther_dn6 = (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn6));
        locals.var_tf_ther_dn7 = (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn7));
        locals.var_tf_ther_dn8 = (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn8));
        locals.var_tf_ther_dn9 = (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn9));

        let assign7620_e7173: f64 = (locals.var_rs_t * locals.var_tf_ther);
        locals.var_rs_i = assign7620_e7173;
        locals.var_rs_i_dn4 = (locals.var_rs_t * locals.var_tf_ther_dn4);
        locals.var_rs_i_dn6 = (locals.var_rs_t * locals.var_tf_ther_dn6);
        locals.var_rs_i_dn7 = (locals.var_rs_t * locals.var_tf_ther_dn7);
        locals.var_rs_i_dn8 = (locals.var_rs_t * locals.var_tf_ther_dn8);
        locals.var_rs_i_dn9 = (locals.var_rs_t * locals.var_tf_ther_dn9);

        let assign7630_e7176: f64 = (2.0 * locals.var_rs_i);
        let assign7630_e7178: f64 = (assign7630_e7176 * locals.var_phit);
        locals.var_frs = assign7630_e7178;
        locals.var_frs_dn4 = (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn4));
        locals.var_frs_dn6 = (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn6));
        locals.var_frs_dn7 = (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn7));
        locals.var_frs_dn8 = (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn8));
        locals.var_frs_dn9 = (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn9));

        let assign7640_e7182: f64 = (16.0 / locals.var_ax_i);
        let assign7640_e7184: f64 = (assign7640_e7182 * 0.6931471805599);
        let assign7640_e7185: f64 = (assign7640_e7184).exp();
        let assign7640_e7187: f64 = (assign7640_e7185 - 1.0);
        let assign7640_e7188: f64 = (assign7640_e7187).ln();
        let assign7640_e7189: f64 = (0.375 * assign7640_e7188);
        let assign7640_e7190: f64 = (assign7640_e7189).exp();
        let assign7640_e7192: f64 = (assign7640_e7190 - 1.0);
        locals.var_gamax = assign7640_e7192;

        let assign7650_e7196: f64 = (16.0 / locals.var_axac_i);
        let assign7650_e7198: f64 = (assign7650_e7196 * 0.6931471805599);
        let assign7650_e7199: f64 = (assign7650_e7198).exp();
        let assign7650_e7201: f64 = (assign7650_e7199 - 1.0);
        let assign7650_e7202: f64 = (assign7650_e7201).ln();
        let assign7650_e7203: f64 = (0.375 * assign7650_e7202);
        let assign7650_e7204: f64 = (assign7650_e7203).exp();
        let assign7650_e7206: f64 = (assign7650_e7204 - 1.0);
        locals.var_gamax_ac = assign7650_e7206;

        let assign7660_e7209: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
        let assign7660_e7210: f64 = (assign7660_e7209).exp();
        locals.var_tf_thesat = assign7660_e7210;
        locals.var_tf_thesat_dn4 = (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn4));
        locals.var_tf_thesat_dn6 = (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn6));
        locals.var_tf_thesat_dn7 = (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn7));
        locals.var_tf_thesat_dn8 = (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn8));
        locals.var_tf_thesat_dn9 = (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn9));

        let assign7670_e7213: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
        let assign7670_e7215: f64 = (assign7670_e7213 * locals.var_tf_bet);
        locals.var_thesat_i = assign7670_e7215;
        locals.var_thesat_i_dn4 = ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn4));
        locals.var_thesat_i_dn6 = ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn6));
        locals.var_thesat_i_dn7 = ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn7));
        locals.var_thesat_i_dn8 = ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn8));
        locals.var_thesat_i_dn9 = ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn9));

        let assign7680_e7218: f64 = (locals.var_thesat_i * locals.var_phit);
        locals.var_sat_phit = assign7680_e7218;
        locals.var_sat_phit_dn4 = ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4));
        locals.var_sat_phit_dn6 = ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6));
        locals.var_sat_phit_dn7 = ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7));
        locals.var_sat_phit_dn8 = ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8));
        locals.var_sat_phit_dn9 = ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9));

        let assign7690_e7221: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
        let assign7690_e7223: f64 = (assign7690_e7221 * locals.var_tf_bet);
        locals.var_thesatac_i = assign7690_e7223;
        locals.var_thesatac_i_dn4 = ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn4));
        locals.var_thesatac_i_dn6 = ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn6));
        locals.var_thesatac_i_dn7 = ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn7));
        locals.var_thesatac_i_dn8 = ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn8));
        locals.var_thesatac_i_dn9 = ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn9));

        let assign7700_e7226: f64 = (locals.var_thesatac_i * locals.var_phit);
        locals.var_sat_phit_ac = assign7700_e7226;
        locals.var_sat_phit_ac_dn4 = ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4));
        locals.var_sat_phit_ac_dn6 = ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6));
        locals.var_sat_phit_ac_dn7 = ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7));
        locals.var_sat_phit_ac_dn8 = ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8));
        locals.var_sat_phit_ac_dn9 = ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9));

        let assign7710_e7229: f64 = (locals.var_alp1_i * locals.var_inv_phit);
        locals.var_alp1_phit = assign7710_e7229;
        locals.var_alp1_phit_dn4 = (locals.var_alp1_i * locals.var_inv_phit_dn4);
        locals.var_alp1_phit_dn6 = (locals.var_alp1_i * locals.var_inv_phit_dn6);
        locals.var_alp1_phit_dn7 = (locals.var_alp1_i * locals.var_inv_phit_dn7);
        locals.var_alp1_phit_dn8 = (locals.var_alp1_i * locals.var_inv_phit_dn8);
        locals.var_alp1_phit_dn9 = (locals.var_alp1_i * locals.var_inv_phit_dn9);

        let assign7720_e7231: f64 = (-locals.var_stig_i);
        let assign7720_e7233: f64 = (assign7720_e7231 * locals.var_lnrtn);
        let assign7720_e7234: f64 = (assign7720_e7233).exp();
        locals.var_tf_ig = assign7720_e7234;

        let assign7730_e7237: f64 = (locals.var_iginv_t * locals.var_tf_ig);
        locals.var_iginv_i = assign7730_e7237;

        let assign7740_e7240: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
        locals.var_igovinv_i = assign7740_e7240;

        let assign7750_e7243: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
        locals.var_igovinvd_i = assign7750_e7243;

        let assign7760_e7246: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
        locals.var_igovacc_i = assign7760_e7246;

        let assign7770_e7249: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
        locals.var_igovaccd_i = assign7770_e7249;

        let assign7780_e7251: f64 = (-locals.var_stigfn_i);
        let assign7780_e7253: f64 = (assign7780_e7251 * locals.var_lnrtn);
        let assign7780_e7254: f64 = (assign7780_e7253).exp();
        locals.var_tf_ig = assign7780_e7254;

        let assign7810_e7263: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign7810_e7263;

        let assign7820_e7266: f64 = (4.0 * 0.3333333333333);
        let assign7820_e7269: f64 = (2.0 * 1.602176565e-19);
        let assign7820_e7271: f64 = (assign7820_e7269 * 9.10938291e-31);
        let assign7820_e7273: f64 = (assign7820_e7271 * locals.var_chib_i);
        let assign7820_e7274: f64 = (assign7820_e7273).sqrt();
        let assign7820_e7275: f64 = (assign7820_e7266 * assign7820_e7274);
        let assign7820_e7277: f64 = (assign7820_e7275 / 1.054571726e-34);
        locals.var_tempm = assign7820_e7277;
        locals.var_tempm_dn4 = 0.0;
        locals.var_tempm_dn6 = 0.0;
        locals.var_tempm_dn7 = 0.0;
        locals.var_tempm_dn8 = 0.0;
        locals.var_tempm_dn9 = 0.0;

        let assign7830_e7280: f64 = (locals.var_tempm * locals.var_toxp_i);
        locals.var_bch = assign7830_e7280;
        locals.var_bch_dn4 = (locals.var_tempm_dn4 * locals.var_toxp_i);
        locals.var_bch_dn6 = (locals.var_tempm_dn6 * locals.var_toxp_i);
        locals.var_bch_dn7 = (locals.var_tempm_dn7 * locals.var_toxp_i);
        locals.var_bch_dn8 = (locals.var_tempm_dn8 * locals.var_toxp_i);
        locals.var_bch_dn9 = (locals.var_tempm_dn9 * locals.var_toxp_i);

        let assign7840_e7283: f64 = (locals.var_tempm * locals.var_toxp_i);
        locals.var_bov = assign7840_e7283;
        locals.var_bov_dn4 = (locals.var_tempm_dn4 * locals.var_toxp_i);
        locals.var_bov_dn6 = (locals.var_tempm_dn6 * locals.var_toxp_i);
        locals.var_bov_dn7 = (locals.var_tempm_dn7 * locals.var_toxp_i);
        locals.var_bov_dn8 = (locals.var_tempm_dn8 * locals.var_toxp_i);
        locals.var_bov_dn9 = (locals.var_tempm_dn9 * locals.var_toxp_i);

        locals.var_gcqch = 0.0;

        let assign7860_e7287: f64 = if locals.var_gc3ch_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign7860_e7287;

    }

    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign7870_e7296,) = {
    if (locals.var_guard148 != 0.0) {
        let assign7870_e7290: f64 = (-0.495);
        let assign7870_e7292: f64 = (assign7870_e7290 * locals.var_gc2ch_i);
        let assign7870_e7294: f64 = (assign7870_e7292 / locals.var_gc3ch_i);
        (assign7870_e7294,)
    } else {
        (locals.var_gcqch,)
    }
};
        locals.var_gcqch = assign7870_e7296;

        locals.var_gcqovinv = 0.0;

        let assign7890_e7300: f64 = if locals.var_gc3ovinv_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7890_e7300;

        let (assign7900_e7309,) = {
    if (locals.var_guard149 != 0.0) {
        let assign7900_e7303: f64 = (-0.495);
        let assign7900_e7305: f64 = (assign7900_e7303 * locals.var_gc2ovinv_i);
        let assign7900_e7307: f64 = (assign7900_e7305 / locals.var_gc3ovinv_i);
        (assign7900_e7307,)
    } else {
        (locals.var_gcqovinv,)
    }
};
        locals.var_gcqovinv = assign7900_e7309;

        locals.var_gcqovacc = 0.0;

        let assign7920_e7313: f64 = if locals.var_gc3ovacc_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7920_e7313;

        let (assign7930_e7322,) = {
    if (locals.var_guard150 != 0.0) {
        let assign7930_e7316: f64 = (-0.495);
        let assign7930_e7318: f64 = (assign7930_e7316 * locals.var_gc2ovacc_i);
        let assign7930_e7320: f64 = (assign7930_e7318 / locals.var_gc3ovacc_i);
        (assign7930_e7320,)
    } else {
        (locals.var_gcqovacc,)
    }
};
        locals.var_gcqovacc = assign7930_e7322;

        let assign7940_e7325: f64 = (0.5 * locals.var_eg);
        locals.var_alpha_b = assign7940_e7325;
        locals.var_alpha_b_dn4 = (0.5 * locals.var_eg_dn4);
        locals.var_alpha_b_dn6 = (0.5 * locals.var_eg_dn6);
        locals.var_alpha_b_dn7 = (0.5 * locals.var_eg_dn7);
        locals.var_alpha_b_dn8 = (0.5 * locals.var_eg_dn8);
        locals.var_alpha_b_dn9 = (0.5 * locals.var_eg_dn9);

        let assign7950_e7328: f64 = (locals.var_gco_i * locals.var_phit);
        locals.var_dch = assign7950_e7328;
        locals.var_dch_dn4 = (locals.var_gco_i * locals.var_phit_dn4);
        locals.var_dch_dn6 = (locals.var_gco_i * locals.var_phit_dn6);
        locals.var_dch_dn7 = (locals.var_gco_i * locals.var_phit_dn7);
        locals.var_dch_dn8 = (locals.var_gco_i * locals.var_phit_dn8);
        locals.var_dch_dn9 = (locals.var_gco_i * locals.var_phit_dn9);

        let assign7960_e7331: f64 = (locals.var_gco_i * locals.var_phit0);
        locals.var_dov = assign7960_e7331;
        locals.var_dov_dn4 = (locals.var_gco_i * locals.var_phit0_dn4);
        locals.var_dov_dn6 = (locals.var_gco_i * locals.var_phit0_dn6);
        locals.var_dov_dn7 = (locals.var_gco_i * locals.var_phit0_dn7);
        locals.var_dov_dn8 = (locals.var_gco_i * locals.var_phit0_dn8);
        locals.var_dov_dn9 = (locals.var_gco_i * locals.var_phit0_dn9);

        let assign7970_e7336: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
        let assign7970_e7337: f64 = (1.0 + assign7970_e7336);
        let assign7970_e7338: f64 = (1.0 / assign7970_e7337);
        locals.var_n_iginv = assign7970_e7338;
        locals.var_n_iginv_dn4 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign7970_e7337 * assign7970_e7337)));
        locals.var_n_iginv_dn6 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign7970_e7337 * assign7970_e7337)));
        locals.var_n_iginv_dn7 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign7970_e7337 * assign7970_e7337)));
        locals.var_n_iginv_dn8 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign7970_e7337 * assign7970_e7337)));
        locals.var_n_iginv_dn9 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign7970_e7337 * assign7970_e7337)));

        let assign7980_e7342: f64 = (locals.var_toxp_i * locals.var_toxp_i);
        let assign7980_e7343: f64 = (4e-18 / assign7980_e7342);
        locals.var_temp = assign7980_e7343;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;

        let assign7990_e7346: f64 = (locals.var_agidl_i * locals.var_temp);
        locals.var_agidl_i = assign7990_e7346;

        let assign8000_e7349: f64 = (locals.var_agidld_i * locals.var_temp);
        locals.var_agidld_i = assign8000_e7349;

        let assign8010_e7352: f64 = (locals.var_toxp_i * 500000000.0);
        locals.var_temp = assign8010_e7352;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;

        let assign8020_e7357: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign8020_e7358: f64 = (1.0 + assign8020_e7357);
        let assign8020_e7360: f64 = assign8020_e7358;
        let assign8020_e7364: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign8020_e7365: f64 = (1.0 + assign8020_e7364);
        let assign8020_e7367: f64 = assign8020_e7365;
        let assign8020_e7371: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign8020_e7372: f64 = (1.0 + assign8020_e7371);
        let assign8020_e7374: f64 = assign8020_e7372;
        let assign8020_e7375: f64 = (assign8020_e7367 * assign8020_e7374);
        let assign8020_e7377: f64 = (assign8020_e7375 + 0.01);
        let assign8020_e7378: f64 = (assign8020_e7377).sqrt();
        let assign8020_e7379: f64 = (assign8020_e7360 + assign8020_e7378);
        let assign8020_e7380: f64 = (0.5 * assign8020_e7379);
        locals.var_tempm = assign8020_e7380;
        locals.var_tempm_dn4 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign8020_e7378))));
        locals.var_tempm_dn6 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign8020_e7378))));
        locals.var_tempm_dn7 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign8020_e7378))));
        locals.var_tempm_dn8 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign8020_e7378))));
        locals.var_tempm_dn9 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign8020_e7378))));

        let assign8030_e7383: f64 = (locals.var_bgidl_t * locals.var_tempm);
        let assign8030_e7385: f64 = (assign8030_e7383 * locals.var_temp);
        locals.var_bgidl_i = assign8030_e7385;
        locals.var_bgidl_i_dn4 = (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn4));
        locals.var_bgidl_i_dn6 = (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn6));
        locals.var_bgidl_i_dn7 = (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn7));
        locals.var_bgidl_i_dn8 = (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn8));
        locals.var_bgidl_i_dn9 = (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn9));

        let assign8040_e7390: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign8040_e7391: f64 = (1.0 + assign8040_e7390);
        let assign8040_e7393: f64 = assign8040_e7391;
        let assign8040_e7397: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign8040_e7398: f64 = (1.0 + assign8040_e7397);
        let assign8040_e7400: f64 = assign8040_e7398;
        let assign8040_e7404: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign8040_e7405: f64 = (1.0 + assign8040_e7404);
        let assign8040_e7407: f64 = assign8040_e7405;
        let assign8040_e7408: f64 = (assign8040_e7400 * assign8040_e7407);
        let assign8040_e7410: f64 = (assign8040_e7408 + 0.01);
        let assign8040_e7411: f64 = (assign8040_e7410).sqrt();
        let assign8040_e7412: f64 = (assign8040_e7393 + assign8040_e7411);
        let assign8040_e7413: f64 = (0.5 * assign8040_e7412);
        locals.var_tempm = assign8040_e7413;
        locals.var_tempm_dn4 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign8040_e7411))));
        locals.var_tempm_dn6 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign8040_e7411))));
        locals.var_tempm_dn7 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign8040_e7411))));
        locals.var_tempm_dn8 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign8040_e7411))));
        locals.var_tempm_dn9 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign8040_e7411))));

        let assign8050_e7416: f64 = (locals.var_bgidld_t * locals.var_tempm);
        let assign8050_e7418: f64 = (assign8050_e7416 * locals.var_temp);
        locals.var_bgidld_i = assign8050_e7418;
        locals.var_bgidld_i_dn4 = (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn4));
        locals.var_bgidld_i_dn6 = (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn6));
        locals.var_bgidld_i_dn7 = (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn7));
        locals.var_bgidld_i_dn8 = (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn8));
        locals.var_bgidld_i_dn9 = (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn9));

        let assign8060_e7421: f64 = (-locals.var_sta2_i);
        let assign8060_e7423: f64 = (assign8060_e7421 * locals.var_lnrtn);
        let assign8060_e7424: f64 = (assign8060_e7423).exp();
        let assign8060_e7425: f64 = (locals.var_a2_t * assign8060_e7424);
        locals.var_a2_i = assign8060_e7425;
        locals.var_a2_i_dn4 = (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn4)));
        locals.var_a2_i_dn6 = (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn6)));
        locals.var_a2_i_dn7 = (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn7)));
        locals.var_a2_i_dn8 = (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn8)));
        locals.var_a2_i_dn9 = (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn9)));

        let assign8070_e7430: f64 = (locals.var_ctedge_i * locals.var_rtn);
        let assign8070_e7431: f64 = (1.0 + assign8070_e7430);
        let assign8070_e7432: f64 = (locals.var_phit0 * assign8070_e7431);
        locals.var_phit_edge = assign8070_e7432;
        locals.var_phit_edge_dn4 = ((locals.var_phit0_dn4 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn4)));
        locals.var_phit_edge_dn6 = ((locals.var_phit0_dn6 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn6)));
        locals.var_phit_edge_dn7 = ((locals.var_phit0_dn7 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn7)));
        locals.var_phit_edge_dn8 = ((locals.var_phit0_dn8 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn8)));
        locals.var_phit_edge_dn9 = ((locals.var_phit0_dn9 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn9)));

        let assign8080_e7435: f64 = (1.0 / locals.var_phit_edge);
        locals.var_inv_phit_edge = assign8080_e7435;
        locals.var_inv_phit_edge_dn4 = (-(locals.var_phit_edge_dn4 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn6 = (-(locals.var_phit_edge_dn6 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn7 = (-(locals.var_phit_edge_dn7 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn8 = (-(locals.var_phit_edge_dn8 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn9 = (-(locals.var_phit_edge_dn9 / (locals.var_phit_edge * locals.var_phit_edge)));

        let assign8090_e7438: f64 = (2.0 * 1.602176565e-19);
        let assign8090_e7440: f64 = (assign8090_e7438 * locals.var_neff);
        let assign8090_e7442: f64 = (assign8090_e7440 * locals.var_epsch);
        let assign8090_e7444: f64 = (assign8090_e7442 * locals.var_inv_phit_edge);
        locals.var_a0_csisq_edge = assign8090_e7444;
        locals.var_a0_csisq_edge_dn4 = ((((assign8090_e7438 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn4));
        locals.var_a0_csisq_edge_dn6 = ((((assign8090_e7438 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn6));
        locals.var_a0_csisq_edge_dn7 = ((((assign8090_e7438 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn7));
        locals.var_a0_csisq_edge_dn8 = ((((assign8090_e7438 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn8));
        locals.var_a0_csisq_edge_dn9 = ((((assign8090_e7438 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn9));

        let assign8100_e7447: f64 = (p.p14 * locals.var_stvfbedge_i);
        let assign8100_e7449: f64 = (assign8100_e7447 * locals.var_dt);
        let assign8100_e7451: f64 = (assign8100_e7449 + locals.var_dvfbqm);
        locals.var_temp = assign8100_e7451;
        locals.var_temp_dn4 = (assign8100_e7447 * locals.var_dt_dn4);
        locals.var_temp_dn6 = (assign8100_e7447 * locals.var_dt_dn6);
        locals.var_temp_dn7 = (assign8100_e7447 * locals.var_dt_dn7);
        locals.var_temp_dn8 = (assign8100_e7447 * locals.var_dt_dn8);
        locals.var_temp_dn9 = (assign8100_e7447 * locals.var_dt_dn9);

        let assign8110_e7455: f64 = (locals.var_vfb1edge_t + locals.var_dvfbch);
        let assign8110_e7457: f64 = (assign8110_e7455 + locals.var_dvfb1nch);
        let assign8110_e7458: f64 = (p.p14 * assign8110_e7457);
        let assign8110_e7460: f64 = (assign8110_e7458 + locals.var_temp);
        let assign8110_e7462: f64 = (assign8110_e7460 + p.p34);
        let assign8110_e7464: f64 = (assign8110_e7462 - locals.var_dvfbpdep);
        locals.var_vfb1edge_i = assign8110_e7464;
        locals.var_vfb1edge_i_dn4 = (((p.p14 * ((locals.var_vfb1edge_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_dn4);
        locals.var_vfb1edge_i_dn6 = (((p.p14 * ((locals.var_vfb1edge_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_dn6);
        locals.var_vfb1edge_i_dn7 = (((p.p14 * ((locals.var_vfb1edge_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_dn7);
        locals.var_vfb1edge_i_dn8 = (((p.p14 * ((locals.var_vfb1edge_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_dn8);
        locals.var_vfb1edge_i_dn9 = (((p.p14 * ((locals.var_vfb1edge_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_dn9);

        let assign8120_e7468: f64 = (locals.var_vfb2edge_t + locals.var_dvfbch);
        let assign8120_e7470: f64 = (assign8120_e7468 + locals.var_dvfb2nch);
        let assign8120_e7471: f64 = (p.p14 * assign8120_e7470);
        let assign8120_e7473: f64 = (assign8120_e7471 + locals.var_temp);
        locals.var_vfb2edge_i = assign8120_e7473;
        locals.var_vfb2edge_i_dn4 = ((p.p14 * (locals.var_dvfbch_dn4 + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfb2edge_i_dn6 = ((p.p14 * (locals.var_dvfbch_dn6 + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfb2edge_i_dn7 = ((p.p14 * (locals.var_dvfbch_dn7 + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfb2edge_i_dn8 = ((p.p14 * (locals.var_dvfbch_dn8 + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfb2edge_i_dn9 = ((p.p14 * (locals.var_dvfbch_dn9 + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);

        let assign8130_e7476: f64 = (locals.var_stbetedge_i * locals.var_lnrtn);
        let assign8130_e7477: f64 = (assign8130_e7476).exp();
        let assign8130_e7479: f64 = (assign8130_e7477 * p.p35);
        locals.var_temp = assign8130_e7479;
        locals.var_temp_dn4 = ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn4)) * p.p35);
        locals.var_temp_dn6 = ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn6)) * p.p35);
        locals.var_temp_dn7 = ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn7)) * p.p35);
        locals.var_temp_dn8 = ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn8)) * p.p35);
        locals.var_temp_dn9 = ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn9)) * p.p35);

        let assign8140_e7482: f64 = (locals.var_betnedge_t * locals.var_temp);
        locals.var_betnedge_i = assign8140_e7482;
        locals.var_betnedge_i_dn4 = ((locals.var_betnedge_t_dn4 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn4));
        locals.var_betnedge_i_dn6 = ((locals.var_betnedge_t_dn6 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn6));
        locals.var_betnedge_i_dn7 = ((locals.var_betnedge_t_dn7 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn7));
        locals.var_betnedge_i_dn8 = ((locals.var_betnedge_t_dn8 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn8));
        locals.var_betnedge_i_dn9 = ((locals.var_betnedge_t_dn9 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn9));

        let assign8150_e7485: f64 = (locals.var_areaq_i * locals.var_phit);
        locals.var_area_phit = assign8150_e7485;
        locals.var_area_phit_dn4 = (locals.var_areaq_i * locals.var_phit_dn4);
        locals.var_area_phit_dn6 = (locals.var_areaq_i * locals.var_phit_dn6);
        locals.var_area_phit_dn7 = (locals.var_areaq_i * locals.var_phit_dn7);
        locals.var_area_phit_dn8 = (locals.var_areaq_i * locals.var_phit_dn8);
        locals.var_area_phit_dn9 = (locals.var_areaq_i * locals.var_phit_dn9);

        let assign8160_e7488: f64 = (0.25 * 1.602176565e-19);
        let assign8160_e7490: f64 = (assign8160_e7488 * locals.var_nsdac_i);
        let assign8160_e7493: f64 = (locals.var_epsch * locals.var_phit);
        let assign8160_e7494: f64 = (assign8160_e7490 / assign8160_e7493);
        locals.var_inner_sd = assign8160_e7494;
        locals.var_inner_sd_dn4 = (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn4)) / (assign8160_e7493 * assign8160_e7493)));
        locals.var_inner_sd_dn6 = (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn6)) / (assign8160_e7493 * assign8160_e7493)));
        locals.var_inner_sd_dn7 = (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn7)) / (assign8160_e7493 * assign8160_e7493)));
        locals.var_inner_sd_dn8 = (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn8)) / (assign8160_e7493 * assign8160_e7493)));
        locals.var_inner_sd_dn9 = (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn9)) / (assign8160_e7493 * assign8160_e7493)));

        let assign8170_e7497: f64 = (locals.var_nsdac_i / locals.var_neff);
        let assign8170_e7498: f64 = (assign8170_e7497).ln();
        locals.var_xsd = assign8170_e7498;
        locals.var_xsd_dn4 = ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign8170_e7497);
        locals.var_xsd_dn6 = ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign8170_e7497);
        locals.var_xsd_dn7 = ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign8170_e7497);
        locals.var_xsd_dn8 = ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign8170_e7497);
        locals.var_xsd_dn9 = ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign8170_e7497);

        let assign8180_e7501: f64 = (locals.var_fif_i * 1.25e-6);
        let assign8180_e7503: f64 = (assign8180_e7501 * locals.var_phit);
        locals.var_fif_phit = assign8180_e7503;
        locals.var_fif_phit_dn4 = (assign8180_e7501 * locals.var_phit_dn4);
        locals.var_fif_phit_dn6 = (assign8180_e7501 * locals.var_phit_dn6);
        locals.var_fif_phit_dn7 = (assign8180_e7501 * locals.var_phit_dn7);
        locals.var_fif_phit_dn8 = (assign8180_e7501 * locals.var_phit_dn8);
        locals.var_fif_phit_dn9 = (assign8180_e7501 * locals.var_phit_dn9);

        let assign8190_e7506: f64 = (locals.var_epsch / 3.45313e-11);
        let assign8190_e7508: f64 = (assign8190_e7506 * locals.var_tsi_i);
        let assign8190_e7511: f64 = (locals.var_tox1_i + 4e-10);
        let assign8190_e7512: f64 = (assign8190_e7508 * assign8190_e7511);
        let assign8190_e7513: f64 = (assign8190_e7512).sqrt();
        locals.var_lambda2d = assign8190_e7513;

        let assign8200_e7516: f64 = (locals.var_strth_i * locals.var_lnrtn);
        let assign8200_e7517: f64 = (assign8200_e7516).exp();
        locals.var_tf_rth = assign8200_e7517;
        locals.var_tf_rth_dn4 = (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn4));
        locals.var_tf_rth_dn6 = (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn6));
        locals.var_tf_rth_dn7 = (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn7));
        locals.var_tf_rth_dn8 = (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn8));
        locals.var_tf_rth_dn9 = (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn9));

        let assign8210_e7520: f64 = (locals.var_rth_t * locals.var_tf_rth);
        locals.var_rth_i = assign8210_e7520;
        locals.var_rth_i_dn4 = ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4));
        locals.var_rth_i_dn6 = ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6));
        locals.var_rth_i_dn7 = ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7));
        locals.var_rth_i_dn8 = ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8));
        locals.var_rth_i_dn9 = ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9));

        let assign8220_e7523: f64 = (4.0 * 1.3806488e-23);
        let assign8220_e7525: f64 = (assign8220_e7523 * locals.var_tkc);
        locals.var_nt0_4kt = assign8220_e7525;

        let assign8230_e7528: f64 = (locals.var_fnt_i * locals.var_nt0_4kt);
        locals.var_nt = assign8230_e7528;

        locals.var_nt0 = locals.var_nt;

        let assign8250_e7532: f64 = (9.10938291e-31 * 1000000000000.0);
        let assign8250_e7534: f64 = (assign8250_e7532 * locals.var_fntexc_i);
        locals.var_fac_exc = assign8250_e7534;

        let assign8380_e7593: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign8380_e7593;

        let (assign8390_e7597, assign8390_e7597_d_n4,) = {
    if (locals.var_guard257 != 0.0) {
        ((nv4 - 0.0), 1.0,)
    } else {
        (locals.var_dtc, locals.var_dtc_dn4,)
    }
};
        locals.var_dtc = assign8390_e7597;
        locals.var_dtc_dn4 = assign8390_e7597_d_n4;

        let (assign8400_e7603, assign8400_e7603_d_n4, assign8400_e7603_d_n6, assign8400_e7603_d_n7, assign8400_e7603_d_n8, assign8400_e7603_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8400_e7601: f64 = (locals.var_tkd + locals.var_dtc);
        (assign8400_e7601, (locals.var_tkd_dn4 + locals.var_dtc_dn4), locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    } else {
        (locals.var_tkc, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9,)
    }
};
        locals.var_tkc = assign8400_e7603;
        locals.var_tkc_dn4 = assign8400_e7603_d_n4;
        locals.var_tkc_dn6 = assign8400_e7603_d_n6;
        locals.var_tkc_dn7 = assign8400_e7603_d_n7;
        locals.var_tkc_dn8 = assign8400_e7603_d_n8;
        locals.var_tkc_dn9 = assign8400_e7603_d_n9;

        let (assign8410_e7609, assign8410_e7609_d_n4, assign8410_e7609_d_n6, assign8410_e7609_d_n7, assign8410_e7609_d_n8, assign8410_e7609_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8410_e7607: f64 = (locals.var_tkc * locals.var_tkc);
        (assign8410_e7607, ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4)), ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6)), ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7)), ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8)), ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9)),)
    } else {
        (locals.var_tkc_sq, locals.var_tkc_sq_dn4, locals.var_tkc_sq_dn6, locals.var_tkc_sq_dn7, locals.var_tkc_sq_dn8, locals.var_tkc_sq_dn9,)
    }
};
        locals.var_tkc_sq = assign8410_e7609;
        locals.var_tkc_sq_dn4 = assign8410_e7609_d_n4;
        locals.var_tkc_sq_dn6 = assign8410_e7609_d_n6;
        locals.var_tkc_sq_dn7 = assign8410_e7609_d_n7;
        locals.var_tkc_sq_dn8 = assign8410_e7609_d_n8;
        locals.var_tkc_sq_dn9 = assign8410_e7609_d_n9;

        let (assign8420_e7615, assign8420_e7615_d_n4, assign8420_e7615_d_n6, assign8420_e7615_d_n7, assign8420_e7615_d_n8, assign8420_e7615_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8420_e7613: f64 = (locals.var_tkc - locals.var_tkr);
        (assign8420_e7613, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9,)
    } else {
        (locals.var_dt, locals.var_dt_dn4, locals.var_dt_dn6, locals.var_dt_dn7, locals.var_dt_dn8, locals.var_dt_dn9,)
    }
};
        locals.var_dt = assign8420_e7615;
        locals.var_dt_dn4 = assign8420_e7615_d_n4;
        locals.var_dt_dn6 = assign8420_e7615_d_n6;
        locals.var_dt_dn7 = assign8420_e7615_d_n7;
        locals.var_dt_dn8 = assign8420_e7615_d_n8;
        locals.var_dt_dn9 = assign8420_e7615_d_n9;

        let (assign8430_e7621, assign8430_e7621_d_n4, assign8430_e7621_d_n6, assign8430_e7621_d_n7, assign8430_e7621_d_n8, assign8430_e7621_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8430_e7619: f64 = (locals.var_tkc / locals.var_tkr);
        (assign8430_e7619, (locals.var_tkc_dn4 / locals.var_tkr), (locals.var_tkc_dn6 / locals.var_tkr), (locals.var_tkc_dn7 / locals.var_tkr), (locals.var_tkc_dn8 / locals.var_tkr), (locals.var_tkc_dn9 / locals.var_tkr),)
    } else {
        (locals.var_rt, locals.var_rt_dn4, locals.var_rt_dn6, locals.var_rt_dn7, locals.var_rt_dn8, locals.var_rt_dn9,)
    }
};
        locals.var_rt = assign8430_e7621;
        locals.var_rt_dn4 = assign8430_e7621_d_n4;
        locals.var_rt_dn6 = assign8430_e7621_d_n6;
        locals.var_rt_dn7 = assign8430_e7621_d_n7;
        locals.var_rt_dn8 = assign8430_e7621_d_n8;
        locals.var_rt_dn9 = assign8430_e7621_d_n9;

        let (assign8440_e7627, assign8440_e7627_d_n4, assign8440_e7627_d_n6, assign8440_e7627_d_n7, assign8440_e7627_d_n8, assign8440_e7627_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8440_e7625: f64 = (locals.var_tkr / locals.var_tkc);
        (assign8440_e7625, (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc))),)
    } else {
        (locals.var_rtn, locals.var_rtn_dn4, locals.var_rtn_dn6, locals.var_rtn_dn7, locals.var_rtn_dn8, locals.var_rtn_dn9,)
    }
};
        locals.var_rtn = assign8440_e7627;
        locals.var_rtn_dn4 = assign8440_e7627_d_n4;
        locals.var_rtn_dn6 = assign8440_e7627_d_n6;
        locals.var_rtn_dn7 = assign8440_e7627_d_n7;
        locals.var_rtn_dn8 = assign8440_e7627_d_n8;
        locals.var_rtn_dn9 = assign8440_e7627_d_n9;

        let (assign8450_e7633, assign8450_e7633_d_n4, assign8450_e7633_d_n6, assign8450_e7633_d_n7, assign8450_e7633_d_n8, assign8450_e7633_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8450_e7631: f64 = (locals.var_tkc * 8.617332384961e-5);
        (assign8450_e7631, (locals.var_tkc_dn4 * 8.617332384961e-5), (locals.var_tkc_dn6 * 8.617332384961e-5), (locals.var_tkc_dn7 * 8.617332384961e-5), (locals.var_tkc_dn8 * 8.617332384961e-5), (locals.var_tkc_dn9 * 8.617332384961e-5),)
    } else {
        (locals.var_phit0, locals.var_phit0_dn4, locals.var_phit0_dn6, locals.var_phit0_dn7, locals.var_phit0_dn8, locals.var_phit0_dn9,)
    }
};
        locals.var_phit0 = assign8450_e7633;
        locals.var_phit0_dn4 = assign8450_e7633_d_n4;
        locals.var_phit0_dn6 = assign8450_e7633_d_n6;
        locals.var_phit0_dn7 = assign8450_e7633_d_n7;
        locals.var_phit0_dn8 = assign8450_e7633_d_n8;
        locals.var_phit0_dn9 = assign8450_e7633_d_n9;

        let (assign8460_e7639, assign8460_e7639_d_n4, assign8460_e7639_d_n6, assign8460_e7639_d_n7, assign8460_e7639_d_n8, assign8460_e7639_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8460_e7637: f64 = (1.0 / locals.var_phit0);
        (assign8460_e7637, (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0))),)
    } else {
        (locals.var_inv_phit0, locals.var_inv_phit0_dn4, locals.var_inv_phit0_dn6, locals.var_inv_phit0_dn7, locals.var_inv_phit0_dn8, locals.var_inv_phit0_dn9,)
    }
};
        locals.var_inv_phit0 = assign8460_e7639;
        locals.var_inv_phit0_dn4 = assign8460_e7639_d_n4;
        locals.var_inv_phit0_dn6 = assign8460_e7639_d_n6;
        locals.var_inv_phit0_dn7 = assign8460_e7639_d_n7;
        locals.var_inv_phit0_dn8 = assign8460_e7639_d_n8;
        locals.var_inv_phit0_dn9 = assign8460_e7639_d_n9;

        let assign8470_e7642: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign8470_e7642;

        let (assign8480_e7675, assign8480_e7675_d_n4, assign8480_e7675_d_n6, assign8480_e7675_d_n7, assign8480_e7675_d_n8, assign8480_e7675_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign8480_e7650: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8480_e7651: f64 = (10.0 / assign8480_e7650);
        let assign8480_e7653: f64 = (assign8480_e7651 + 600.0);
        let assign8480_e7657: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8480_e7658: f64 = (10.0 / assign8480_e7657);
        let assign8480_e7660: f64 = (assign8480_e7658 - 600.0);
        let assign8480_e7664: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8480_e7665: f64 = (10.0 / assign8480_e7664);
        let assign8480_e7667: f64 = (assign8480_e7665 - 600.0);
        let assign8480_e7668: f64 = (assign8480_e7660 * assign8480_e7667);
        let assign8480_e7670: f64 = (assign8480_e7668 + 0.01);
        let assign8480_e7671: f64 = (assign8480_e7670).sqrt();
        let assign8480_e7672: f64 = (assign8480_e7653 + assign8480_e7671);
        let assign8480_e7673: f64 = (0.5 * assign8480_e7672);
        (assign8480_e7673, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))),)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign8480_e7675;
        locals.var_xsatmax_dn4 = assign8480_e7675_d_n4;
        locals.var_xsatmax_dn6 = assign8480_e7675_d_n6;
        locals.var_xsatmax_dn7 = assign8480_e7675_d_n7;
        locals.var_xsatmax_dn8 = assign8480_e7675_d_n8;
        locals.var_xsatmax_dn9 = assign8480_e7675_d_n9;

        let (assign8490_e7682, assign8490_e7682_d_n4, assign8490_e7682_d_n6, assign8490_e7682_d_n7, assign8490_e7682_d_n8, assign8490_e7682_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 == 0.0)) {
        (600.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign8490_e7682;
        locals.var_xsatmax_dn4 = assign8490_e7682_d_n4;
        locals.var_xsatmax_dn6 = assign8490_e7682_d_n6;
        locals.var_xsatmax_dn7 = assign8490_e7682_d_n7;
        locals.var_xsatmax_dn8 = assign8490_e7682_d_n8;
        locals.var_xsatmax_dn9 = assign8490_e7682_d_n9;

        let (assign8500_e7694, assign8500_e7694_d_n4, assign8500_e7694_d_n6, assign8500_e7694_d_n7, assign8500_e7694_d_n8, assign8500_e7694_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8500_e7687: f64 = (0.000473 * locals.var_tkc_sq);
        let assign8500_e7690: f64 = (636.0 + locals.var_tkc);
        let assign8500_e7691: f64 = (assign8500_e7687 / assign8500_e7690);
        let assign8500_e7692: f64 = (1.17 - assign8500_e7691);
        (assign8500_e7692, (-((((0.000473 * locals.var_tkc_sq_dn4) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn4)) / (assign8500_e7690 * assign8500_e7690))), (-((((0.000473 * locals.var_tkc_sq_dn6) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn6)) / (assign8500_e7690 * assign8500_e7690))), (-((((0.000473 * locals.var_tkc_sq_dn7) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn7)) / (assign8500_e7690 * assign8500_e7690))), (-((((0.000473 * locals.var_tkc_sq_dn8) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn8)) / (assign8500_e7690 * assign8500_e7690))), (-((((0.000473 * locals.var_tkc_sq_dn9) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn9)) / (assign8500_e7690 * assign8500_e7690))),)
    } else {
        (locals.var_egsi, locals.var_egsi_dn4, locals.var_egsi_dn6, locals.var_egsi_dn7, locals.var_egsi_dn8, locals.var_egsi_dn9,)
    }
};
        locals.var_egsi = assign8500_e7694;
        locals.var_egsi_dn4 = assign8500_e7694_d_n4;
        locals.var_egsi_dn6 = assign8500_e7694_d_n6;
        locals.var_egsi_dn7 = assign8500_e7694_d_n7;
        locals.var_egsi_dn8 = assign8500_e7694_d_n8;
        locals.var_egsi_dn9 = assign8500_e7694_d_n9;

    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8510_e7706, assign8510_e7706_d_n4, assign8510_e7706_d_n6, assign8510_e7706_d_n7, assign8510_e7706_d_n8, assign8510_e7706_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8510_e7699: f64 = (0.0004774 * locals.var_tkc_sq);
        let assign8510_e7702: f64 = (235.0 + locals.var_tkc);
        let assign8510_e7703: f64 = (assign8510_e7699 / assign8510_e7702);
        let assign8510_e7704: f64 = (0.744 - assign8510_e7703);
        (assign8510_e7704, (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn4)) / (assign8510_e7702 * assign8510_e7702))), (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn6)) / (assign8510_e7702 * assign8510_e7702))), (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn7)) / (assign8510_e7702 * assign8510_e7702))), (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn8)) / (assign8510_e7702 * assign8510_e7702))), (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn9)) / (assign8510_e7702 * assign8510_e7702))),)
    } else {
        (locals.var_egge, locals.var_egge_dn4, locals.var_egge_dn6, locals.var_egge_dn7, locals.var_egge_dn8, locals.var_egge_dn9,)
    }
};
        locals.var_egge = assign8510_e7706;
        locals.var_egge_dn4 = assign8510_e7706_d_n4;
        locals.var_egge_dn6 = assign8510_e7706_d_n6;
        locals.var_egge_dn7 = assign8510_e7706_d_n7;
        locals.var_egge_dn8 = assign8510_e7706_d_n8;
        locals.var_egge_dn9 = assign8510_e7706_d_n9;

        let (assign8520_e7719, assign8520_e7719_d_n4, assign8520_e7719_d_n6, assign8520_e7719_d_n7, assign8520_e7719_d_n8, assign8520_e7719_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8520_e7710: f64 = (locals.var_egge - locals.var_egsi);
        let assign8520_e7712: f64 = (-0.4);
        let assign8520_e7714: f64 = (assign8520_e7712 * locals.var_one_m_xge);
        let assign8520_e7715: f64 = (assign8520_e7710 + assign8520_e7714);
        let assign8520_e7717: f64 = (assign8520_e7715 * locals.var_xge_i);
        (assign8520_e7717, ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i), ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i), ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i), ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i), ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i),)
    } else {
        (locals.var_deg, locals.var_deg_dn4, locals.var_deg_dn6, locals.var_deg_dn7, locals.var_deg_dn8, locals.var_deg_dn9,)
    }
};
        locals.var_deg = assign8520_e7719;
        locals.var_deg_dn4 = assign8520_e7719_d_n4;
        locals.var_deg_dn6 = assign8520_e7719_d_n6;
        locals.var_deg_dn7 = assign8520_e7719_d_n7;
        locals.var_deg_dn8 = assign8520_e7719_d_n8;
        locals.var_deg_dn9 = assign8520_e7719_d_n9;

        let (assign8530_e7725, assign8530_e7725_d_n4, assign8530_e7725_d_n6, assign8530_e7725_d_n7, assign8530_e7725_d_n8, assign8530_e7725_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8530_e7723: f64 = (locals.var_egsi + locals.var_deg);
        (assign8530_e7723, (locals.var_egsi_dn4 + locals.var_deg_dn4), (locals.var_egsi_dn6 + locals.var_deg_dn6), (locals.var_egsi_dn7 + locals.var_deg_dn7), (locals.var_egsi_dn8 + locals.var_deg_dn8), (locals.var_egsi_dn9 + locals.var_deg_dn9),)
    } else {
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9,)
    }
};
        locals.var_eg = assign8530_e7725;
        locals.var_eg_dn4 = assign8530_e7725_d_n4;
        locals.var_eg_dn6 = assign8530_e7725_d_n6;
        locals.var_eg_dn7 = assign8530_e7725_d_n7;
        locals.var_eg_dn8 = assign8530_e7725_d_n8;
        locals.var_eg_dn9 = assign8530_e7725_d_n9;

        let (assign8540_e7733, assign8540_e7733_d_n4, assign8540_e7733_d_n6, assign8540_e7733_d_n7, assign8540_e7733_d_n8, assign8540_e7733_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8540_e7729: f64 = (0.5 * locals.var_eg);
        let assign8540_e7731: f64 = (assign8540_e7729 * locals.var_inv_phit0);
        (assign8540_e7731, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn9)),)
    } else {
        (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9,)
    }
};
        locals.var_eg_2phit0 = assign8540_e7733;
        locals.var_eg_2phit0_dn4 = assign8540_e7733_d_n4;
        locals.var_eg_2phit0_dn6 = assign8540_e7733_d_n6;
        locals.var_eg_2phit0_dn7 = assign8540_e7733_d_n7;
        locals.var_eg_2phit0_dn8 = assign8540_e7733_d_n8;
        locals.var_eg_2phit0_dn9 = assign8540_e7733_d_n9;

        let (assign8550_e7743, assign8550_e7743_d_n4, assign8550_e7743_d_n6, assign8550_e7743_d_n7, assign8550_e7743_d_n8, assign8550_e7743_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8550_e7737: f64 = (0.05 * locals.var_xge_i);
        let assign8550_e7740: f64 = (0.5 * locals.var_deg);
        let assign8550_e7741: f64 = (assign8550_e7737 - assign8550_e7740);
        (assign8550_e7741, (-(0.5 * locals.var_deg_dn4)), (-(0.5 * locals.var_deg_dn6)), (-(0.5 * locals.var_deg_dn7)), (-(0.5 * locals.var_deg_dn8)), (-(0.5 * locals.var_deg_dn9)),)
    } else {
        (locals.var_dvfbch, locals.var_dvfbch_dn4, locals.var_dvfbch_dn6, locals.var_dvfbch_dn7, locals.var_dvfbch_dn8, locals.var_dvfbch_dn9,)
    }
};
        locals.var_dvfbch = assign8550_e7743;
        locals.var_dvfbch_dn4 = assign8550_e7743_d_n4;
        locals.var_dvfbch_dn6 = assign8550_e7743_d_n6;
        locals.var_dvfbch_dn7 = assign8550_e7743_d_n7;
        locals.var_dvfbch_dn8 = assign8550_e7743_d_n8;
        locals.var_dvfbch_dn9 = assign8550_e7743_d_n9;

        let (assign8560_e7750, assign8560_e7750_d_n4, assign8560_e7750_d_n6, assign8560_e7750_d_n7, assign8560_e7750_d_n8, assign8560_e7750_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8560_e7747: f64 = (locals.var_tkc * 0.0033333333333);
        let assign8560_e7748: f64 = (assign8560_e7747).sqrt();
        (assign8560_e7748, ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign8560_e7748)), ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign8560_e7748)), ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign8560_e7748)), ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign8560_e7748)), ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign8560_e7748)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8560_e7750;
        locals.var_temp_dn4 = assign8560_e7750_d_n4;
        locals.var_temp_dn6 = assign8560_e7750_d_n6;
        locals.var_temp_dn7 = assign8560_e7750_d_n7;
        locals.var_temp_dn8 = assign8560_e7750_d_n8;
        locals.var_temp_dn9 = assign8560_e7750_d_n9;

        let (assign8570_e7760, assign8570_e7760_d_n4, assign8570_e7760_d_n6, assign8570_e7760_d_n7, assign8570_e7760_d_n8, assign8570_e7760_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8570_e7754: f64 = (4.05e25 * locals.var_temp);
        let assign8570_e7756: f64 = (assign8570_e7754 * locals.var_temp);
        let assign8570_e7758: f64 = (assign8570_e7756 * locals.var_temp);
        (assign8570_e7758, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn4)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn6)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn7)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn8)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn9)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign8570_e7760;
        locals.var_temp1_dn4 = assign8570_e7760_d_n4;
        locals.var_temp1_dn6 = assign8570_e7760_d_n6;
        locals.var_temp1_dn7 = assign8570_e7760_d_n7;
        locals.var_temp1_dn8 = assign8570_e7760_d_n8;
        locals.var_temp1_dn9 = assign8570_e7760_d_n9;

        let (assign8580_e7766, assign8580_e7766_d_n4, assign8580_e7766_d_n6, assign8580_e7766_d_n7, assign8580_e7766_d_n8, assign8580_e7766_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8580_e7764: f64 = (locals.var_temp1 * locals.var_niratio);
        (assign8580_e7764, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio),)
    } else {
        (locals.var_neff, locals.var_neff_dn4, locals.var_neff_dn6, locals.var_neff_dn7, locals.var_neff_dn8, locals.var_neff_dn9,)
    }
};
        locals.var_neff = assign8580_e7766;
        locals.var_neff_dn4 = assign8580_e7766_d_n4;
        locals.var_neff_dn6 = assign8580_e7766_d_n6;
        locals.var_neff_dn7 = assign8580_e7766_d_n7;
        locals.var_neff_dn8 = assign8580_e7766_d_n8;
        locals.var_neff_dn9 = assign8580_e7766_d_n9;

        let (assign8590_e7776, assign8590_e7776_d_n4, assign8590_e7776_d_n6, assign8590_e7776_d_n7, assign8590_e7776_d_n8, assign8590_e7776_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8590_e7772: f64 = (locals.var_ct_i * locals.var_rtn);
        let assign8590_e7773: f64 = (1.0 + assign8590_e7772);
        let assign8590_e7774: f64 = (locals.var_phit0 * assign8590_e7773);
        (assign8590_e7774, ((locals.var_phit0_dn4 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9))),)
    } else {
        (locals.var_phit, locals.var_phit_dn4, locals.var_phit_dn6, locals.var_phit_dn7, locals.var_phit_dn8, locals.var_phit_dn9,)
    }
};
        locals.var_phit = assign8590_e7776;
        locals.var_phit_dn4 = assign8590_e7776_d_n4;
        locals.var_phit_dn6 = assign8590_e7776_d_n6;
        locals.var_phit_dn7 = assign8590_e7776_d_n7;
        locals.var_phit_dn8 = assign8590_e7776_d_n8;
        locals.var_phit_dn9 = assign8590_e7776_d_n9;

        let (assign8600_e7782, assign8600_e7782_d_n4, assign8600_e7782_d_n6, assign8600_e7782_d_n7, assign8600_e7782_d_n8, assign8600_e7782_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8600_e7780: f64 = (1.0 / locals.var_phit);
        (assign8600_e7780, (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit))),)
    } else {
        (locals.var_inv_phit, locals.var_inv_phit_dn4, locals.var_inv_phit_dn6, locals.var_inv_phit_dn7, locals.var_inv_phit_dn8, locals.var_inv_phit_dn9,)
    }
};
        locals.var_inv_phit = assign8600_e7782;
        locals.var_inv_phit_dn4 = assign8600_e7782_d_n4;
        locals.var_inv_phit_dn6 = assign8600_e7782_d_n6;
        locals.var_inv_phit_dn7 = assign8600_e7782_d_n7;
        locals.var_inv_phit_dn8 = assign8600_e7782_d_n8;
        locals.var_inv_phit_dn9 = assign8600_e7782_d_n9;

        let (assign8610_e7790, assign8610_e7790_d_n4, assign8610_e7790_d_n6, assign8610_e7790_d_n7, assign8610_e7790_d_n8, assign8610_e7790_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8610_e7786: f64 = (0.5 * locals.var_eg);
        let assign8610_e7788: f64 = (assign8610_e7786 * locals.var_inv_phit);
        (assign8610_e7788, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_eg_2phit, locals.var_eg_2phit_dn4, locals.var_eg_2phit_dn6, locals.var_eg_2phit_dn7, locals.var_eg_2phit_dn8, locals.var_eg_2phit_dn9,)
    }
};
        locals.var_eg_2phit = assign8610_e7790;
        locals.var_eg_2phit_dn4 = assign8610_e7790_d_n4;
        locals.var_eg_2phit_dn6 = assign8610_e7790_d_n6;
        locals.var_eg_2phit_dn7 = assign8610_e7790_d_n7;
        locals.var_eg_2phit_dn8 = assign8610_e7790_d_n8;
        locals.var_eg_2phit_dn9 = assign8610_e7790_d_n9;

        let (assign8620_e7802, assign8620_e7802_d_n4, assign8620_e7802_d_n6, assign8620_e7802_d_n7, assign8620_e7802_d_n8, assign8620_e7802_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8620_e7794: f64 = (2.0 * 1.602176565e-19);
        let assign8620_e7796: f64 = (assign8620_e7794 * locals.var_neff);
        let assign8620_e7798: f64 = (assign8620_e7796 * locals.var_epsch);
        let assign8620_e7800: f64 = (assign8620_e7798 * locals.var_inv_phit);
        (assign8620_e7800, ((((assign8620_e7794 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn4)), ((((assign8620_e7794 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn6)), ((((assign8620_e7794 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn7)), ((((assign8620_e7794 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn8)), ((((assign8620_e7794 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_a0_csisq, locals.var_a0_csisq_dn4, locals.var_a0_csisq_dn6, locals.var_a0_csisq_dn7, locals.var_a0_csisq_dn8, locals.var_a0_csisq_dn9,)
    }
};
        locals.var_a0_csisq = assign8620_e7802;
        locals.var_a0_csisq_dn4 = assign8620_e7802_d_n4;
        locals.var_a0_csisq_dn6 = assign8620_e7802_d_n6;
        locals.var_a0_csisq_dn7 = assign8620_e7802_d_n7;
        locals.var_a0_csisq_dn8 = assign8620_e7802_d_n8;
        locals.var_a0_csisq_dn9 = assign8620_e7802_d_n9;

        let (assign8630_e7813, assign8630_e7813_d_n4, assign8630_e7813_d_n6, assign8630_e7813_d_n7, assign8630_e7813_d_n8, assign8630_e7813_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8630_e7806: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign8630_e7808: f64 = (assign8630_e7806 / locals.var_a0_csisq);
        let assign8630_e7809: f64 = (assign8630_e7808).ln();
        let assign8630_e7811: f64 = (assign8630_e7809 - 0.6931471805599);
        (assign8630_e7811, ((-((assign8630_e7806 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), ((-((assign8630_e7806 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), ((-((assign8630_e7806 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), ((-((assign8630_e7806 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), ((-((assign8630_e7806 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808),)
    } else {
        (locals.var_xth_1d, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9,)
    }
};
        locals.var_xth_1d = assign8630_e7813;
        locals.var_xth_1d_dn4 = assign8630_e7813_d_n4;
        locals.var_xth_1d_dn6 = assign8630_e7813_d_n6;
        locals.var_xth_1d_dn7 = assign8630_e7813_d_n7;
        locals.var_xth_1d_dn8 = assign8630_e7813_d_n8;
        locals.var_xth_1d_dn9 = assign8630_e7813_d_n9;

        let (assign8640_e7829, assign8640_e7829_d_n4, assign8640_e7829_d_n6, assign8640_e7829_d_n7, assign8640_e7829_d_n8, assign8640_e7829_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8640_e7817: f64 = (0.5 * 1.602176565e-19);
        let assign8640_e7819: f64 = (assign8640_e7817 * locals.var_nsddc_i);
        let assign8640_e7821: f64 = (assign8640_e7819 * locals.var_tsi_i);
        let assign8640_e7824: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign8640_e7825: f64 = (assign8640_e7821 / assign8640_e7824);
        let assign8640_e7827: f64 = (assign8640_e7825 * locals.var_inv_phit);
        (assign8640_e7827, (assign8640_e7825 * locals.var_inv_phit_dn4), (assign8640_e7825 * locals.var_inv_phit_dn6), (assign8640_e7825 * locals.var_inv_phit_dn7), (assign8640_e7825 * locals.var_inv_phit_dn8), (assign8640_e7825 * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_xsddep, locals.var_xsddep_dn4, locals.var_xsddep_dn6, locals.var_xsddep_dn7, locals.var_xsddep_dn8, locals.var_xsddep_dn9,)
    }
};
        locals.var_xsddep = assign8640_e7829;
        locals.var_xsddep_dn4 = assign8640_e7829_d_n4;
        locals.var_xsddep_dn6 = assign8640_e7829_d_n6;
        locals.var_xsddep_dn7 = assign8640_e7829_d_n7;
        locals.var_xsddep_dn8 = assign8640_e7829_d_n8;
        locals.var_xsddep_dn9 = assign8640_e7829_d_n9;

        let (assign8650_e7835, assign8650_e7835_d_n4, assign8650_e7835_d_n6, assign8650_e7835_d_n7, assign8650_e7835_d_n8, assign8650_e7835_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8650_e7833: f64 = (locals.var_stcf_i * locals.var_dt);
        (assign8650_e7833, ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4)), ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6)), ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7)), ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8)), ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8650_e7835;
        locals.var_temp_dn4 = assign8650_e7835_d_n4;
        locals.var_temp_dn6 = assign8650_e7835_d_n6;
        locals.var_temp_dn7 = assign8650_e7835_d_n7;
        locals.var_temp_dn8 = assign8650_e7835_d_n8;
        locals.var_temp_dn9 = assign8650_e7835_d_n9;

        let (assign8660_e7841, assign8660_e7841_d_n4, assign8660_e7841_d_n6, assign8660_e7841_d_n7, assign8660_e7841_d_n8, assign8660_e7841_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8660_e7839: f64 = (locals.var_cf1_t + locals.var_temp);
        (assign8660_e7839, (locals.var_cf1_t_dn4 + locals.var_temp_dn4), (locals.var_cf1_t_dn6 + locals.var_temp_dn6), (locals.var_cf1_t_dn7 + locals.var_temp_dn7), (locals.var_cf1_t_dn8 + locals.var_temp_dn8), (locals.var_cf1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9,)
    }
};
        locals.var_cf1_i = assign8660_e7841;
        locals.var_cf1_i_dn4 = assign8660_e7841_d_n4;
        locals.var_cf1_i_dn6 = assign8660_e7841_d_n6;
        locals.var_cf1_i_dn7 = assign8660_e7841_d_n7;
        locals.var_cf1_i_dn8 = assign8660_e7841_d_n8;
        locals.var_cf1_i_dn9 = assign8660_e7841_d_n9;

        let (assign8670_e7847, assign8670_e7847_d_n4, assign8670_e7847_d_n6, assign8670_e7847_d_n7, assign8670_e7847_d_n8, assign8670_e7847_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8670_e7845: f64 = (locals.var_cf2_t + locals.var_temp);
        (assign8670_e7845, (locals.var_cf2_t_dn4 + locals.var_temp_dn4), (locals.var_cf2_t_dn6 + locals.var_temp_dn6), (locals.var_cf2_t_dn7 + locals.var_temp_dn7), (locals.var_cf2_t_dn8 + locals.var_temp_dn8), (locals.var_cf2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9,)
    }
};
        locals.var_cf2_i = assign8670_e7847;
        locals.var_cf2_i_dn4 = assign8670_e7847_d_n4;
        locals.var_cf2_i_dn6 = assign8670_e7847_d_n6;
        locals.var_cf2_i_dn7 = assign8670_e7847_d_n7;
        locals.var_cf2_i_dn8 = assign8670_e7847_d_n8;
        locals.var_cf2_i_dn9 = assign8670_e7847_d_n9;

        let (assign8680_e7853, assign8680_e7853_d_n4, assign8680_e7853_d_n6, assign8680_e7853_d_n7, assign8680_e7853_d_n8, assign8680_e7853_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8680_e7851: f64 = (locals.var_cfd_i * locals.var_inv_phit);
        (assign8680_e7851, (locals.var_cfd_i * locals.var_inv_phit_dn4), (locals.var_cfd_i * locals.var_inv_phit_dn6), (locals.var_cfd_i * locals.var_inv_phit_dn7), (locals.var_cfd_i * locals.var_inv_phit_dn8), (locals.var_cfd_i * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_xd0, locals.var_xd0_dn4, locals.var_xd0_dn6, locals.var_xd0_dn7, locals.var_xd0_dn8, locals.var_xd0_dn9,)
    }
};
        locals.var_xd0 = assign8680_e7853;
        locals.var_xd0_dn4 = assign8680_e7853_d_n4;
        locals.var_xd0_dn6 = assign8680_e7853_d_n6;
        locals.var_xd0_dn7 = assign8680_e7853_d_n7;
        locals.var_xd0_dn8 = assign8680_e7853_d_n8;
        locals.var_xd0_dn9 = assign8680_e7853_d_n9;

        let (assign8690_e7859, assign8690_e7859_d_n4, assign8690_e7859_d_n6, assign8690_e7859_d_n7, assign8690_e7859_d_n8, assign8690_e7859_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8690_e7857: f64 = (locals.var_cfac1_t + locals.var_temp);
        (assign8690_e7857, (locals.var_cfac1_t_dn4 + locals.var_temp_dn4), (locals.var_cfac1_t_dn6 + locals.var_temp_dn6), (locals.var_cfac1_t_dn7 + locals.var_temp_dn7), (locals.var_cfac1_t_dn8 + locals.var_temp_dn8), (locals.var_cfac1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9,)
    }
};
        locals.var_cfac1_i = assign8690_e7859;
        locals.var_cfac1_i_dn4 = assign8690_e7859_d_n4;
        locals.var_cfac1_i_dn6 = assign8690_e7859_d_n6;
        locals.var_cfac1_i_dn7 = assign8690_e7859_d_n7;
        locals.var_cfac1_i_dn8 = assign8690_e7859_d_n8;
        locals.var_cfac1_i_dn9 = assign8690_e7859_d_n9;

        let (assign8700_e7865, assign8700_e7865_d_n4, assign8700_e7865_d_n6, assign8700_e7865_d_n7, assign8700_e7865_d_n8, assign8700_e7865_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8700_e7863: f64 = (locals.var_cfac2_t + locals.var_temp);
        (assign8700_e7863, (locals.var_cfac2_t_dn4 + locals.var_temp_dn4), (locals.var_cfac2_t_dn6 + locals.var_temp_dn6), (locals.var_cfac2_t_dn7 + locals.var_temp_dn7), (locals.var_cfac2_t_dn8 + locals.var_temp_dn8), (locals.var_cfac2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9,)
    }
};
        locals.var_cfac2_i = assign8700_e7865;
        locals.var_cfac2_i_dn4 = assign8700_e7865_d_n4;
        locals.var_cfac2_i_dn6 = assign8700_e7865_d_n6;
        locals.var_cfac2_i_dn7 = assign8700_e7865_d_n7;
        locals.var_cfac2_i_dn8 = assign8700_e7865_d_n8;
        locals.var_cfac2_i_dn9 = assign8700_e7865_d_n9;

        let assign8710_e7868: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign8710_e7868;

        let (assign8720_e7881, assign8720_e7881_d_n4, assign8720_e7881_d_n6, assign8720_e7881_d_n7, assign8720_e7881_d_n8, assign8720_e7881_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard259 != 0.0)) {
        let assign8720_e7875: f64 = (locals.var_np_i / locals.var_neff_poly);
        let assign8720_e7876: f64 = (assign8720_e7875).ln();
        let assign8720_e7878: f64 = (assign8720_e7876 + locals.var_eg_2phit0_woshe);
        let assign8720_e7879: f64 = (locals.var_phit0 * assign8720_e7878);
        (assign8720_e7879, ((locals.var_phit0_dn4 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn4))), ((locals.var_phit0_dn6 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn6))), ((locals.var_phit0_dn7 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn7))), ((locals.var_phit0_dn8 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn8))), ((locals.var_phit0_dn9 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn9))),)
    } else {
        (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9,)
    }
};
        locals.var_dvfbpdep = assign8720_e7881;
        locals.var_dvfbpdep_dn4 = assign8720_e7881_d_n4;
        locals.var_dvfbpdep_dn6 = assign8720_e7881_d_n6;
        locals.var_dvfbpdep_dn7 = assign8720_e7881_d_n7;
        locals.var_dvfbpdep_dn8 = assign8720_e7881_d_n8;
        locals.var_dvfbpdep_dn9 = assign8720_e7881_d_n9;

        let assign8730_e7884: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign8730_e7884;

        let (assign8740_e7911, assign8740_e7911_d_n4, assign8740_e7911_d_n6, assign8740_e7911_d_n7, assign8740_e7911_d_n8, assign8740_e7911_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard260 != 0.0)) {
        let assign8740_e7892: f64 = (2970.0 / locals.var_tkd);
        let assign8740_e7893: f64 = (15.0 + assign8740_e7892);
        let assign8740_e7897: f64 = (2970.0 / locals.var_tkd);
        let assign8740_e7898: f64 = (15.0 - assign8740_e7897);
        let assign8740_e7902: f64 = (2970.0 / locals.var_tkd);
        let assign8740_e7903: f64 = (15.0 - assign8740_e7902);
        let assign8740_e7904: f64 = (assign8740_e7898 * assign8740_e7903);
        let assign8740_e7906: f64 = (assign8740_e7904 + 1e-6);
        let assign8740_e7907: f64 = (assign8740_e7906).sqrt();
        let assign8740_e7908: f64 = (assign8740_e7893 + assign8740_e7907);
        let assign8740_e7909: f64 = (0.5 * assign8740_e7908);
        (assign8740_e7909, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))),)
    } else {
        (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9,)
    }
};
        locals.var_emin = assign8740_e7911;
        locals.var_emin_dn4 = assign8740_e7911_d_n4;
        locals.var_emin_dn6 = assign8740_e7911_d_n6;
        locals.var_emin_dn7 = assign8740_e7911_d_n7;
        locals.var_emin_dn8 = assign8740_e7911_d_n8;
        locals.var_emin_dn9 = assign8740_e7911_d_n9;

        let (assign8750_e7915, assign8750_e7915_d_n4, assign8750_e7915_d_n6, assign8750_e7915_d_n7, assign8750_e7915_d_n8, assign8750_e7915_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8750_e7915;
        locals.var_qq_dn4 = assign8750_e7915_d_n4;
        locals.var_qq_dn6 = assign8750_e7915_d_n6;
        locals.var_qq_dn7 = assign8750_e7915_d_n7;
        locals.var_qq_dn8 = assign8750_e7915_d_n8;
        locals.var_qq_dn9 = assign8750_e7915_d_n9;

        let assign8760_e7918: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign8760_e7918;

        let assign8770_e7921: f64 = 1.0;
        let assign8770_e7922: f64 = if p.p14 == assign8770_e7921 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign8770_e7922;

        let (assign8780_e7943, assign8780_e7943_d_n4, assign8780_e7943_d_n6, assign8780_e7943_d_n7, assign8780_e7943_d_n8, assign8780_e7943_d_n9,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 != 0.0)) {
        let assign8780_e7930: f64 = (0.4 * p.p13);
        let assign8780_e7932: f64 = (assign8780_e7930 * 1.27520989);
        let assign8780_e7934: f64 = (-0.3333333333333);
        let assign8780_e7937: f64 = (locals.var_phit * locals.var_tsisq);
        let assign8780_e7938: f64 = (assign8780_e7937).ln();
        let assign8780_e7939: f64 = (assign8780_e7934 * assign8780_e7938);
        let assign8780_e7940: f64 = (assign8780_e7939).exp();
        let assign8780_e7941: f64 = (assign8780_e7932 * assign8780_e7940);
        (assign8780_e7941, (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8780_e7937)))), (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8780_e7937)))), (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8780_e7937)))), (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8780_e7937)))), (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8780_e7937)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8780_e7943;
        locals.var_qq_dn4 = assign8780_e7943_d_n4;
        locals.var_qq_dn6 = assign8780_e7943_d_n6;
        locals.var_qq_dn7 = assign8780_e7943_d_n7;
        locals.var_qq_dn8 = assign8780_e7943_d_n8;
        locals.var_qq_dn9 = assign8780_e7943_d_n9;

        let (assign8790_e7965, assign8790_e7965_d_n4, assign8790_e7965_d_n6, assign8790_e7965_d_n7, assign8790_e7965_d_n8, assign8790_e7965_d_n9,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 == 0.0)) {
        let assign8790_e7952: f64 = (0.4 * p.p13);
        let assign8790_e7954: f64 = (assign8790_e7952 * 1.5412087);
        let assign8790_e7956: f64 = (-0.3333333333333);
        let assign8790_e7959: f64 = (locals.var_phit * locals.var_tsisq);
        let assign8790_e7960: f64 = (assign8790_e7959).ln();
        let assign8790_e7961: f64 = (assign8790_e7956 * assign8790_e7960);
        let assign8790_e7962: f64 = (assign8790_e7961).exp();
        let assign8790_e7963: f64 = (assign8790_e7954 * assign8790_e7962);
        (assign8790_e7963, (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8790_e7959)))), (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8790_e7959)))), (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8790_e7959)))), (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8790_e7959)))), (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8790_e7959)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8790_e7965;
        locals.var_qq_dn4 = assign8790_e7965_d_n4;
        locals.var_qq_dn6 = assign8790_e7965_d_n6;
        locals.var_qq_dn7 = assign8790_e7965_d_n7;
        locals.var_qq_dn8 = assign8790_e7965_d_n8;
        locals.var_qq_dn9 = assign8790_e7965_d_n9;

        let (assign8800_e7975, assign8800_e7975_d_n4, assign8800_e7975_d_n6, assign8800_e7975_d_n7, assign8800_e7975_d_n8, assign8800_e7975_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8800_e7969: f64 = (p.p14 * locals.var_stvfb_i);
        let assign8800_e7971: f64 = (assign8800_e7969 * locals.var_dt);
        let assign8800_e7973: f64 = (assign8800_e7971 + locals.var_dvfbqm);
        (assign8800_e7973, (assign8800_e7969 * locals.var_dt_dn4), (assign8800_e7969 * locals.var_dt_dn6), (assign8800_e7969 * locals.var_dt_dn7), (assign8800_e7969 * locals.var_dt_dn8), (assign8800_e7969 * locals.var_dt_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8800_e7975;
        locals.var_temp_dn4 = assign8800_e7975_d_n4;
        locals.var_temp_dn6 = assign8800_e7975_d_n6;
        locals.var_temp_dn7 = assign8800_e7975_d_n7;
        locals.var_temp_dn8 = assign8800_e7975_d_n8;
        locals.var_temp_dn9 = assign8800_e7975_d_n9;

        let (assign8810_e7983, assign8810_e7983_d_n4, assign8810_e7983_d_n6, assign8810_e7983_d_n7, assign8810_e7983_d_n8, assign8810_e7983_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8810_e7979: f64 = (locals.var_temp + p.p34);
        let assign8810_e7981: f64 = (assign8810_e7979 - locals.var_dvfbpdep);
        (assign8810_e7981, (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4), (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6), (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7), (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8), (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign8810_e7983;
        locals.var_temp1_dn4 = assign8810_e7983_d_n4;
        locals.var_temp1_dn6 = assign8810_e7983_d_n6;
        locals.var_temp1_dn7 = assign8810_e7983_d_n7;
        locals.var_temp1_dn8 = assign8810_e7983_d_n8;
        locals.var_temp1_dn9 = assign8810_e7983_d_n9;

        let (assign8820_e7995, assign8820_e7995_d_n4, assign8820_e7995_d_n6, assign8820_e7995_d_n7, assign8820_e7995_d_n8, assign8820_e7995_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8820_e7988: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
        let assign8820_e7990: f64 = (assign8820_e7988 + locals.var_dvfb1nch);
        let assign8820_e7991: f64 = (p.p14 * assign8820_e7990);
        let assign8820_e7993: f64 = (assign8820_e7991 + locals.var_temp1);
        (assign8820_e7993, ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9),)
    } else {
        (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9,)
    }
};
        locals.var_vfb1_i = assign8820_e7995;
        locals.var_vfb1_i_dn4 = assign8820_e7995_d_n4;
        locals.var_vfb1_i_dn6 = assign8820_e7995_d_n6;
        locals.var_vfb1_i_dn7 = assign8820_e7995_d_n7;
        locals.var_vfb1_i_dn8 = assign8820_e7995_d_n8;
        locals.var_vfb1_i_dn9 = assign8820_e7995_d_n9;

        let (assign8830_e8007, assign8830_e8007_d_n4, assign8830_e8007_d_n6, assign8830_e8007_d_n7, assign8830_e8007_d_n8, assign8830_e8007_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8830_e8000: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
        let assign8830_e8002: f64 = (assign8830_e8000 + locals.var_dvfb2nch);
        let assign8830_e8003: f64 = (p.p14 * assign8830_e8002);
        let assign8830_e8005: f64 = (assign8830_e8003 + locals.var_temp);
        (assign8830_e8005, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9,)
    }
};
        locals.var_vfb2_i = assign8830_e8007;
        locals.var_vfb2_i_dn4 = assign8830_e8007_d_n4;
        locals.var_vfb2_i_dn6 = assign8830_e8007_d_n6;
        locals.var_vfb2_i_dn7 = assign8830_e8007_d_n7;
        locals.var_vfb2_i_dn8 = assign8830_e8007_d_n8;
        locals.var_vfb2_i_dn9 = assign8830_e8007_d_n9;

    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8840_e8019, assign8840_e8019_d_n4, assign8840_e8019_d_n6, assign8840_e8019_d_n7, assign8840_e8019_d_n8, assign8840_e8019_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8840_e8012: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
        let assign8840_e8014: f64 = (assign8840_e8012 + locals.var_dvfb1nch);
        let assign8840_e8015: f64 = (p.p14 * assign8840_e8014);
        let assign8840_e8017: f64 = (assign8840_e8015 + locals.var_temp1);
        (assign8840_e8017, ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9),)
    } else {
        (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9,)
    }
};
        locals.var_vfbac1_i = assign8840_e8019;
        locals.var_vfbac1_i_dn4 = assign8840_e8019_d_n4;
        locals.var_vfbac1_i_dn6 = assign8840_e8019_d_n6;
        locals.var_vfbac1_i_dn7 = assign8840_e8019_d_n7;
        locals.var_vfbac1_i_dn8 = assign8840_e8019_d_n8;
        locals.var_vfbac1_i_dn9 = assign8840_e8019_d_n9;

        let (assign8850_e8031, assign8850_e8031_d_n4, assign8850_e8031_d_n6, assign8850_e8031_d_n7, assign8850_e8031_d_n8, assign8850_e8031_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8850_e8024: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
        let assign8850_e8026: f64 = (assign8850_e8024 + locals.var_dvfb2nch);
        let assign8850_e8027: f64 = (p.p14 * assign8850_e8026);
        let assign8850_e8029: f64 = (assign8850_e8027 + locals.var_temp);
        (assign8850_e8029, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9,)
    }
};
        locals.var_vfbac2_i = assign8850_e8031;
        locals.var_vfbac2_i_dn4 = assign8850_e8031_d_n4;
        locals.var_vfbac2_i_dn6 = assign8850_e8031_d_n6;
        locals.var_vfbac2_i_dn7 = assign8850_e8031_d_n7;
        locals.var_vfbac2_i_dn8 = assign8850_e8031_d_n8;
        locals.var_vfbac2_i_dn9 = assign8850_e8031_d_n9;

        let (assign8860_e8036, assign8860_e8036_d_n4, assign8860_e8036_d_n6, assign8860_e8036_d_n7, assign8860_e8036_d_n8, assign8860_e8036_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8860_e8034: f64 = (locals.var_rtn).ln();
        (assign8860_e8034, (locals.var_rtn_dn4 / locals.var_rtn), (locals.var_rtn_dn6 / locals.var_rtn), (locals.var_rtn_dn7 / locals.var_rtn), (locals.var_rtn_dn8 / locals.var_rtn), (locals.var_rtn_dn9 / locals.var_rtn),)
    } else {
        (locals.var_lnrtn, locals.var_lnrtn_dn4, locals.var_lnrtn_dn6, locals.var_lnrtn_dn7, locals.var_lnrtn_dn8, locals.var_lnrtn_dn9,)
    }
};
        locals.var_lnrtn = assign8860_e8036;
        locals.var_lnrtn_dn4 = assign8860_e8036_d_n4;
        locals.var_lnrtn_dn6 = assign8860_e8036_d_n6;
        locals.var_lnrtn_dn7 = assign8860_e8036_d_n7;
        locals.var_lnrtn_dn8 = assign8860_e8036_d_n8;
        locals.var_lnrtn_dn9 = assign8860_e8036_d_n9;

        let (assign8870_e8045, assign8870_e8045_d_n4, assign8870_e8045_d_n6, assign8870_e8045_d_n7, assign8870_e8045_d_n8, assign8870_e8045_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8870_e8040: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign8870_e8041: f64 = (assign8870_e8040).exp();
        let assign8870_e8043: f64 = (assign8870_e8041 * p.p35);
        (assign8870_e8043, ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35), ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35), ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35), ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35), ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35),)
    } else {
        (locals.var_tf_bet, locals.var_tf_bet_dn4, locals.var_tf_bet_dn6, locals.var_tf_bet_dn7, locals.var_tf_bet_dn8, locals.var_tf_bet_dn9,)
    }
};
        locals.var_tf_bet = assign8870_e8045;
        locals.var_tf_bet_dn4 = assign8870_e8045_d_n4;
        locals.var_tf_bet_dn6 = assign8870_e8045_d_n6;
        locals.var_tf_bet_dn7 = assign8870_e8045_d_n7;
        locals.var_tf_bet_dn8 = assign8870_e8045_d_n8;
        locals.var_tf_bet_dn9 = assign8870_e8045_d_n9;

        let (assign8880_e8051, assign8880_e8051_d_n4, assign8880_e8051_d_n6, assign8880_e8051_d_n7, assign8880_e8051_d_n8, assign8880_e8051_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8880_e8049: f64 = (locals.var_betn1_t * locals.var_tf_bet);
        (assign8880_e8049, ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4)), ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6)), ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7)), ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8)), ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_betn1_i, locals.var_betn1_i_dn4, locals.var_betn1_i_dn6, locals.var_betn1_i_dn7, locals.var_betn1_i_dn8, locals.var_betn1_i_dn9,)
    }
};
        locals.var_betn1_i = assign8880_e8051;
        locals.var_betn1_i_dn4 = assign8880_e8051_d_n4;
        locals.var_betn1_i_dn6 = assign8880_e8051_d_n6;
        locals.var_betn1_i_dn7 = assign8880_e8051_d_n7;
        locals.var_betn1_i_dn8 = assign8880_e8051_d_n8;
        locals.var_betn1_i_dn9 = assign8880_e8051_d_n9;

        let (assign8890_e8057, assign8890_e8057_d_n4, assign8890_e8057_d_n6, assign8890_e8057_d_n7, assign8890_e8057_d_n8, assign8890_e8057_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8890_e8055: f64 = (locals.var_betn2_t * locals.var_tf_bet);
        (assign8890_e8055, ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4)), ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6)), ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7)), ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8)), ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_betn2_i, locals.var_betn2_i_dn4, locals.var_betn2_i_dn6, locals.var_betn2_i_dn7, locals.var_betn2_i_dn8, locals.var_betn2_i_dn9,)
    }
};
        locals.var_betn2_i = assign8890_e8057;
        locals.var_betn2_i_dn4 = assign8890_e8057_d_n4;
        locals.var_betn2_i_dn6 = assign8890_e8057_d_n6;
        locals.var_betn2_i_dn7 = assign8890_e8057_d_n7;
        locals.var_betn2_i_dn8 = assign8890_e8057_d_n8;
        locals.var_betn2_i_dn9 = assign8890_e8057_d_n9;

        let (assign8900_e8064, assign8900_e8064_d_n4, assign8900_e8064_d_n6, assign8900_e8064_d_n7, assign8900_e8064_d_n8, assign8900_e8064_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8900_e8061: f64 = (locals.var_stmue_i * locals.var_lnrtn);
        let assign8900_e8062: f64 = (assign8900_e8061).exp();
        (assign8900_e8062, (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn4)), (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn6)), (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn7)), (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn8)), (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_mue, locals.var_tf_mue_dn4, locals.var_tf_mue_dn6, locals.var_tf_mue_dn7, locals.var_tf_mue_dn8, locals.var_tf_mue_dn9,)
    }
};
        locals.var_tf_mue = assign8900_e8064;
        locals.var_tf_mue_dn4 = assign8900_e8064_d_n4;
        locals.var_tf_mue_dn6 = assign8900_e8064_d_n6;
        locals.var_tf_mue_dn7 = assign8900_e8064_d_n7;
        locals.var_tf_mue_dn8 = assign8900_e8064_d_n8;
        locals.var_tf_mue_dn9 = assign8900_e8064_d_n9;

        let (assign8910_e8070, assign8910_e8070_d_n4, assign8910_e8070_d_n6, assign8910_e8070_d_n7, assign8910_e8070_d_n8, assign8910_e8070_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8910_e8068: f64 = (locals.var_mue_t * locals.var_tf_mue);
        (assign8910_e8068, (locals.var_mue_t * locals.var_tf_mue_dn4), (locals.var_mue_t * locals.var_tf_mue_dn6), (locals.var_mue_t * locals.var_tf_mue_dn7), (locals.var_mue_t * locals.var_tf_mue_dn8), (locals.var_mue_t * locals.var_tf_mue_dn9),)
    } else {
        (locals.var_mue_i, locals.var_mue_i_dn4, locals.var_mue_i_dn6, locals.var_mue_i_dn7, locals.var_mue_i_dn8, locals.var_mue_i_dn9,)
    }
};
        locals.var_mue_i = assign8910_e8070;
        locals.var_mue_i_dn4 = assign8910_e8070_d_n4;
        locals.var_mue_i_dn6 = assign8910_e8070_d_n6;
        locals.var_mue_i_dn7 = assign8910_e8070_d_n7;
        locals.var_mue_i_dn8 = assign8910_e8070_d_n8;
        locals.var_mue_i_dn9 = assign8910_e8070_d_n9;

        let (assign8920_e8077, assign8920_e8077_d_n4, assign8920_e8077_d_n6, assign8920_e8077_d_n7, assign8920_e8077_d_n8, assign8920_e8077_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8920_e8074: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
        let assign8920_e8075: f64 = (assign8920_e8074).exp();
        (assign8920_e8075, (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn4)), (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn6)), (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn7)), (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn8)), (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_themu, locals.var_tf_themu_dn4, locals.var_tf_themu_dn6, locals.var_tf_themu_dn7, locals.var_tf_themu_dn8, locals.var_tf_themu_dn9,)
    }
};
        locals.var_tf_themu = assign8920_e8077;
        locals.var_tf_themu_dn4 = assign8920_e8077_d_n4;
        locals.var_tf_themu_dn6 = assign8920_e8077_d_n6;
        locals.var_tf_themu_dn7 = assign8920_e8077_d_n7;
        locals.var_tf_themu_dn8 = assign8920_e8077_d_n8;
        locals.var_tf_themu_dn9 = assign8920_e8077_d_n9;

        let (assign8930_e8083, assign8930_e8083_d_n4, assign8930_e8083_d_n6, assign8930_e8083_d_n7, assign8930_e8083_d_n8, assign8930_e8083_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8930_e8081: f64 = (locals.var_themu_t * locals.var_tf_themu);
        (assign8930_e8081, (locals.var_themu_t * locals.var_tf_themu_dn4), (locals.var_themu_t * locals.var_tf_themu_dn6), (locals.var_themu_t * locals.var_tf_themu_dn7), (locals.var_themu_t * locals.var_tf_themu_dn8), (locals.var_themu_t * locals.var_tf_themu_dn9),)
    } else {
        (locals.var_themu_i, locals.var_themu_i_dn4, locals.var_themu_i_dn6, locals.var_themu_i_dn7, locals.var_themu_i_dn8, locals.var_themu_i_dn9,)
    }
};
        locals.var_themu_i = assign8930_e8083;
        locals.var_themu_i_dn4 = assign8930_e8083_d_n4;
        locals.var_themu_i_dn6 = assign8930_e8083_d_n6;
        locals.var_themu_i_dn7 = assign8930_e8083_d_n7;
        locals.var_themu_i_dn8 = assign8930_e8083_d_n8;
        locals.var_themu_i_dn9 = assign8930_e8083_d_n9;

        let (assign8940_e8090, assign8940_e8090_d_n4, assign8940_e8090_d_n6, assign8940_e8090_d_n7, assign8940_e8090_d_n8, assign8940_e8090_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8940_e8087: f64 = (locals.var_stcs_i * locals.var_lnrtn);
        let assign8940_e8088: f64 = (assign8940_e8087).exp();
        (assign8940_e8088, (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn4)), (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn6)), (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn7)), (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn8)), (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_cs, locals.var_tf_cs_dn4, locals.var_tf_cs_dn6, locals.var_tf_cs_dn7, locals.var_tf_cs_dn8, locals.var_tf_cs_dn9,)
    }
};
        locals.var_tf_cs = assign8940_e8090;
        locals.var_tf_cs_dn4 = assign8940_e8090_d_n4;
        locals.var_tf_cs_dn6 = assign8940_e8090_d_n6;
        locals.var_tf_cs_dn7 = assign8940_e8090_d_n7;
        locals.var_tf_cs_dn8 = assign8940_e8090_d_n8;
        locals.var_tf_cs_dn9 = assign8940_e8090_d_n9;

        let (assign8950_e8096, assign8950_e8096_d_n4, assign8950_e8096_d_n6, assign8950_e8096_d_n7, assign8950_e8096_d_n8, assign8950_e8096_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8950_e8094: f64 = (locals.var_cs_t * locals.var_tf_cs);
        (assign8950_e8094, (locals.var_cs_t * locals.var_tf_cs_dn4), (locals.var_cs_t * locals.var_tf_cs_dn6), (locals.var_cs_t * locals.var_tf_cs_dn7), (locals.var_cs_t * locals.var_tf_cs_dn8), (locals.var_cs_t * locals.var_tf_cs_dn9),)
    } else {
        (locals.var_cs_i, locals.var_cs_i_dn4, locals.var_cs_i_dn6, locals.var_cs_i_dn7, locals.var_cs_i_dn8, locals.var_cs_i_dn9,)
    }
};
        locals.var_cs_i = assign8950_e8096;
        locals.var_cs_i_dn4 = assign8950_e8096_d_n4;
        locals.var_cs_i_dn6 = assign8950_e8096_d_n6;
        locals.var_cs_i_dn7 = assign8950_e8096_d_n7;
        locals.var_cs_i_dn8 = assign8950_e8096_d_n8;
        locals.var_cs_i_dn9 = assign8950_e8096_d_n9;

        let (assign8960_e8103, assign8960_e8103_d_n4, assign8960_e8103_d_n6, assign8960_e8103_d_n7, assign8960_e8103_d_n8, assign8960_e8103_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8960_e8100: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
        let assign8960_e8101: f64 = (assign8960_e8100).exp();
        (assign8960_e8101, (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn4)), (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn6)), (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn7)), (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn8)), (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_thecs, locals.var_tf_thecs_dn4, locals.var_tf_thecs_dn6, locals.var_tf_thecs_dn7, locals.var_tf_thecs_dn8, locals.var_tf_thecs_dn9,)
    }
};
        locals.var_tf_thecs = assign8960_e8103;
        locals.var_tf_thecs_dn4 = assign8960_e8103_d_n4;
        locals.var_tf_thecs_dn6 = assign8960_e8103_d_n6;
        locals.var_tf_thecs_dn7 = assign8960_e8103_d_n7;
        locals.var_tf_thecs_dn8 = assign8960_e8103_d_n8;
        locals.var_tf_thecs_dn9 = assign8960_e8103_d_n9;

        let (assign8970_e8109, assign8970_e8109_d_n4, assign8970_e8109_d_n6, assign8970_e8109_d_n7, assign8970_e8109_d_n8, assign8970_e8109_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8970_e8107: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
        (assign8970_e8107, (locals.var_thecs_t * locals.var_tf_thecs_dn4), (locals.var_thecs_t * locals.var_tf_thecs_dn6), (locals.var_thecs_t * locals.var_tf_thecs_dn7), (locals.var_thecs_t * locals.var_tf_thecs_dn8), (locals.var_thecs_t * locals.var_tf_thecs_dn9),)
    } else {
        (locals.var_thecs_i, locals.var_thecs_i_dn4, locals.var_thecs_i_dn6, locals.var_thecs_i_dn7, locals.var_thecs_i_dn8, locals.var_thecs_i_dn9,)
    }
};
        locals.var_thecs_i = assign8970_e8109;
        locals.var_thecs_i_dn4 = assign8970_e8109_d_n4;
        locals.var_thecs_i_dn6 = assign8970_e8109_d_n6;
        locals.var_thecs_i_dn7 = assign8970_e8109_d_n7;
        locals.var_thecs_i_dn8 = assign8970_e8109_d_n8;
        locals.var_thecs_i_dn9 = assign8970_e8109_d_n9;

        let (assign8980_e8116, assign8980_e8116_d_n4, assign8980_e8116_d_n6, assign8980_e8116_d_n7, assign8980_e8116_d_n8, assign8980_e8116_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8980_e8113: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
        let assign8980_e8114: f64 = (assign8980_e8113).exp();
        (assign8980_e8114, (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn4)), (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn6)), (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn7)), (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn8)), (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_xcor, locals.var_tf_xcor_dn4, locals.var_tf_xcor_dn6, locals.var_tf_xcor_dn7, locals.var_tf_xcor_dn8, locals.var_tf_xcor_dn9,)
    }
};
        locals.var_tf_xcor = assign8980_e8116;
        locals.var_tf_xcor_dn4 = assign8980_e8116_d_n4;
        locals.var_tf_xcor_dn6 = assign8980_e8116_d_n6;
        locals.var_tf_xcor_dn7 = assign8980_e8116_d_n7;
        locals.var_tf_xcor_dn8 = assign8980_e8116_d_n8;
        locals.var_tf_xcor_dn9 = assign8980_e8116_d_n9;

        let (assign8990_e8122, assign8990_e8122_d_n4, assign8990_e8122_d_n6, assign8990_e8122_d_n7, assign8990_e8122_d_n8, assign8990_e8122_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8990_e8120: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
        (assign8990_e8120, (locals.var_xcor_t * locals.var_tf_xcor_dn4), (locals.var_xcor_t * locals.var_tf_xcor_dn6), (locals.var_xcor_t * locals.var_tf_xcor_dn7), (locals.var_xcor_t * locals.var_tf_xcor_dn8), (locals.var_xcor_t * locals.var_tf_xcor_dn9),)
    } else {
        (locals.var_xcor_i, locals.var_xcor_i_dn4, locals.var_xcor_i_dn6, locals.var_xcor_i_dn7, locals.var_xcor_i_dn8, locals.var_xcor_i_dn9,)
    }
};
        locals.var_xcor_i = assign8990_e8122;
        locals.var_xcor_i_dn4 = assign8990_e8122_d_n4;
        locals.var_xcor_i_dn6 = assign8990_e8122_d_n6;
        locals.var_xcor_i_dn7 = assign8990_e8122_d_n7;
        locals.var_xcor_i_dn8 = assign8990_e8122_d_n8;
        locals.var_xcor_i_dn9 = assign8990_e8122_d_n9;

        let (assign9000_e8130, assign9000_e8130_d_n4, assign9000_e8130_d_n6, assign9000_e8130_d_n7, assign9000_e8130_d_n8, assign9000_e8130_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9000_e8126: f64 = (1e-8 * locals.var_phit);
        let assign9000_e8128: f64 = (assign9000_e8126 / locals.var_tsi_i);
        (assign9000_e8128, ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign9000_e8130;
        locals.var_temp_dn4 = assign9000_e8130_d_n4;
        locals.var_temp_dn6 = assign9000_e8130_d_n6;
        locals.var_temp_dn7 = assign9000_e8130_d_n7;
        locals.var_temp_dn8 = assign9000_e8130_d_n8;
        locals.var_temp_dn9 = assign9000_e8130_d_n9;

        let (assign9010_e8136, assign9010_e8136_d_n4, assign9010_e8136_d_n6, assign9010_e8136_d_n7, assign9010_e8136_d_n8, assign9010_e8136_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9010_e8134: f64 = (locals.var_temp * locals.var_mue_i);
        (assign9010_e8134, ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4)), ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6)), ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7)), ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8)), ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9)),)
    } else {
        (locals.var_fmue, locals.var_fmue_dn4, locals.var_fmue_dn6, locals.var_fmue_dn7, locals.var_fmue_dn8, locals.var_fmue_dn9,)
    }
};
        locals.var_fmue = assign9010_e8136;
        locals.var_fmue_dn4 = assign9010_e8136_d_n4;
        locals.var_fmue_dn6 = assign9010_e8136_d_n6;
        locals.var_fmue_dn7 = assign9010_e8136_d_n7;
        locals.var_fmue_dn8 = assign9010_e8136_d_n8;
        locals.var_fmue_dn9 = assign9010_e8136_d_n9;

        let (assign9020_e8143, assign9020_e8143_d_n4, assign9020_e8143_d_n6, assign9020_e8143_d_n7, assign9020_e8143_d_n8, assign9020_e8143_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9020_e8140: f64 = (locals.var_strs_i * locals.var_lnrtn);
        let assign9020_e8141: f64 = (assign9020_e8140).exp();
        (assign9020_e8141, (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn4)), (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn6)), (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn7)), (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn8)), (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_ther, locals.var_tf_ther_dn4, locals.var_tf_ther_dn6, locals.var_tf_ther_dn7, locals.var_tf_ther_dn8, locals.var_tf_ther_dn9,)
    }
};
        locals.var_tf_ther = assign9020_e8143;
        locals.var_tf_ther_dn4 = assign9020_e8143_d_n4;
        locals.var_tf_ther_dn6 = assign9020_e8143_d_n6;
        locals.var_tf_ther_dn7 = assign9020_e8143_d_n7;
        locals.var_tf_ther_dn8 = assign9020_e8143_d_n8;
        locals.var_tf_ther_dn9 = assign9020_e8143_d_n9;

        let (assign9030_e8149, assign9030_e8149_d_n4, assign9030_e8149_d_n6, assign9030_e8149_d_n7, assign9030_e8149_d_n8, assign9030_e8149_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9030_e8147: f64 = (locals.var_rs_t * locals.var_tf_ther);
        (assign9030_e8147, (locals.var_rs_t * locals.var_tf_ther_dn4), (locals.var_rs_t * locals.var_tf_ther_dn6), (locals.var_rs_t * locals.var_tf_ther_dn7), (locals.var_rs_t * locals.var_tf_ther_dn8), (locals.var_rs_t * locals.var_tf_ther_dn9),)
    } else {
        (locals.var_rs_i, locals.var_rs_i_dn4, locals.var_rs_i_dn6, locals.var_rs_i_dn7, locals.var_rs_i_dn8, locals.var_rs_i_dn9,)
    }
};
        locals.var_rs_i = assign9030_e8149;
        locals.var_rs_i_dn4 = assign9030_e8149_d_n4;
        locals.var_rs_i_dn6 = assign9030_e8149_d_n6;
        locals.var_rs_i_dn7 = assign9030_e8149_d_n7;
        locals.var_rs_i_dn8 = assign9030_e8149_d_n8;
        locals.var_rs_i_dn9 = assign9030_e8149_d_n9;

        let (assign9040_e8157, assign9040_e8157_d_n4, assign9040_e8157_d_n6, assign9040_e8157_d_n7, assign9040_e8157_d_n8, assign9040_e8157_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9040_e8153: f64 = (2.0 * locals.var_rs_i);
        let assign9040_e8155: f64 = (assign9040_e8153 * locals.var_phit);
        (assign9040_e8155, (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn4)), (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn6)), (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn7)), (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn8)), (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn9)),)
    } else {
        (locals.var_frs, locals.var_frs_dn4, locals.var_frs_dn6, locals.var_frs_dn7, locals.var_frs_dn8, locals.var_frs_dn9,)
    }
};
        locals.var_frs = assign9040_e8157;
        locals.var_frs_dn4 = assign9040_e8157_d_n4;
        locals.var_frs_dn6 = assign9040_e8157_d_n6;
        locals.var_frs_dn7 = assign9040_e8157_d_n7;
        locals.var_frs_dn8 = assign9040_e8157_d_n8;
        locals.var_frs_dn9 = assign9040_e8157_d_n9;

        let (assign9050_e8164, assign9050_e8164_d_n4, assign9050_e8164_d_n6, assign9050_e8164_d_n7, assign9050_e8164_d_n8, assign9050_e8164_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9050_e8161: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
        let assign9050_e8162: f64 = (assign9050_e8161).exp();
        (assign9050_e8162, (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn4)), (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn6)), (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn7)), (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn8)), (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_thesat, locals.var_tf_thesat_dn4, locals.var_tf_thesat_dn6, locals.var_tf_thesat_dn7, locals.var_tf_thesat_dn8, locals.var_tf_thesat_dn9,)
    }
};
        locals.var_tf_thesat = assign9050_e8164;
        locals.var_tf_thesat_dn4 = assign9050_e8164_d_n4;
        locals.var_tf_thesat_dn6 = assign9050_e8164_d_n6;
        locals.var_tf_thesat_dn7 = assign9050_e8164_d_n7;
        locals.var_tf_thesat_dn8 = assign9050_e8164_d_n8;
        locals.var_tf_thesat_dn9 = assign9050_e8164_d_n9;

        let (assign9060_e8172, assign9060_e8172_d_n4, assign9060_e8172_d_n6, assign9060_e8172_d_n7, assign9060_e8172_d_n8, assign9060_e8172_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9060_e8168: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
        let assign9060_e8170: f64 = (assign9060_e8168 * locals.var_tf_bet);
        (assign9060_e8170, ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn4)), ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn6)), ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn7)), ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn8)), ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_thesat_i, locals.var_thesat_i_dn4, locals.var_thesat_i_dn6, locals.var_thesat_i_dn7, locals.var_thesat_i_dn8, locals.var_thesat_i_dn9,)
    }
};
        locals.var_thesat_i = assign9060_e8172;
        locals.var_thesat_i_dn4 = assign9060_e8172_d_n4;
        locals.var_thesat_i_dn6 = assign9060_e8172_d_n6;
        locals.var_thesat_i_dn7 = assign9060_e8172_d_n7;
        locals.var_thesat_i_dn8 = assign9060_e8172_d_n8;
        locals.var_thesat_i_dn9 = assign9060_e8172_d_n9;

        let (assign9070_e8178, assign9070_e8178_d_n4, assign9070_e8178_d_n6, assign9070_e8178_d_n7, assign9070_e8178_d_n8, assign9070_e8178_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9070_e8176: f64 = (locals.var_thesat_i * locals.var_phit);
        (assign9070_e8176, ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4)), ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6)), ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7)), ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8)), ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9)),)
    } else {
        (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9,)
    }
};
        locals.var_sat_phit = assign9070_e8178;
        locals.var_sat_phit_dn4 = assign9070_e8178_d_n4;
        locals.var_sat_phit_dn6 = assign9070_e8178_d_n6;
        locals.var_sat_phit_dn7 = assign9070_e8178_d_n7;
        locals.var_sat_phit_dn8 = assign9070_e8178_d_n8;
        locals.var_sat_phit_dn9 = assign9070_e8178_d_n9;

        let (assign9080_e8186, assign9080_e8186_d_n4, assign9080_e8186_d_n6, assign9080_e8186_d_n7, assign9080_e8186_d_n8, assign9080_e8186_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9080_e8182: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
        let assign9080_e8184: f64 = (assign9080_e8182 * locals.var_tf_bet);
        (assign9080_e8184, ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn4)), ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn6)), ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn7)), ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn8)), ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_thesatac_i, locals.var_thesatac_i_dn4, locals.var_thesatac_i_dn6, locals.var_thesatac_i_dn7, locals.var_thesatac_i_dn8, locals.var_thesatac_i_dn9,)
    }
};
        locals.var_thesatac_i = assign9080_e8186;
        locals.var_thesatac_i_dn4 = assign9080_e8186_d_n4;
        locals.var_thesatac_i_dn6 = assign9080_e8186_d_n6;
        locals.var_thesatac_i_dn7 = assign9080_e8186_d_n7;
        locals.var_thesatac_i_dn8 = assign9080_e8186_d_n8;
        locals.var_thesatac_i_dn9 = assign9080_e8186_d_n9;

        let (assign9090_e8192, assign9090_e8192_d_n4, assign9090_e8192_d_n6, assign9090_e8192_d_n7, assign9090_e8192_d_n8, assign9090_e8192_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9090_e8190: f64 = (locals.var_thesatac_i * locals.var_phit);
        (assign9090_e8190, ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4)), ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6)), ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7)), ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8)), ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9)),)
    } else {
        (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9,)
    }
};
        locals.var_sat_phit_ac = assign9090_e8192;
        locals.var_sat_phit_ac_dn4 = assign9090_e8192_d_n4;
        locals.var_sat_phit_ac_dn6 = assign9090_e8192_d_n6;
        locals.var_sat_phit_ac_dn7 = assign9090_e8192_d_n7;
        locals.var_sat_phit_ac_dn8 = assign9090_e8192_d_n8;
        locals.var_sat_phit_ac_dn9 = assign9090_e8192_d_n9;

        let (assign9100_e8198, assign9100_e8198_d_n4, assign9100_e8198_d_n6, assign9100_e8198_d_n7, assign9100_e8198_d_n8, assign9100_e8198_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9100_e8196: f64 = (locals.var_alp1_i * locals.var_inv_phit);
        (assign9100_e8196, (locals.var_alp1_i * locals.var_inv_phit_dn4), (locals.var_alp1_i * locals.var_inv_phit_dn6), (locals.var_alp1_i * locals.var_inv_phit_dn7), (locals.var_alp1_i * locals.var_inv_phit_dn8), (locals.var_alp1_i * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_alp1_phit, locals.var_alp1_phit_dn4, locals.var_alp1_phit_dn6, locals.var_alp1_phit_dn7, locals.var_alp1_phit_dn8, locals.var_alp1_phit_dn9,)
    }
};
        locals.var_alp1_phit = assign9100_e8198;
        locals.var_alp1_phit_dn4 = assign9100_e8198_d_n4;
        locals.var_alp1_phit_dn6 = assign9100_e8198_d_n6;
        locals.var_alp1_phit_dn7 = assign9100_e8198_d_n7;
        locals.var_alp1_phit_dn8 = assign9100_e8198_d_n8;
        locals.var_alp1_phit_dn9 = assign9100_e8198_d_n9;

        let (assign9110_e8206,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9110_e8201: f64 = (-locals.var_stig_i);
        let assign9110_e8203: f64 = (assign9110_e8201 * locals.var_lnrtn);
        let assign9110_e8204: f64 = (assign9110_e8203).exp();
        (assign9110_e8204,)
    } else {
        (locals.var_tf_ig,)
    }
};
        locals.var_tf_ig = assign9110_e8206;

        let (assign9120_e8212,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9120_e8210: f64 = (locals.var_iginv_t * locals.var_tf_ig);
        (assign9120_e8210,)
    } else {
        (locals.var_iginv_i,)
    }
};
        locals.var_iginv_i = assign9120_e8212;

        let (assign9130_e8218,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9130_e8216: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
        (assign9130_e8216,)
    } else {
        (locals.var_igovinv_i,)
    }
};
        locals.var_igovinv_i = assign9130_e8218;

        let (assign9140_e8224,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9140_e8222: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
        (assign9140_e8222,)
    } else {
        (locals.var_igovinvd_i,)
    }
};
        locals.var_igovinvd_i = assign9140_e8224;

        let (assign9150_e8230,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9150_e8228: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
        (assign9150_e8228,)
    } else {
        (locals.var_igovacc_i,)
    }
};
        locals.var_igovacc_i = assign9150_e8230;

        let (assign9160_e8236,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9160_e8234: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
        (assign9160_e8234,)
    } else {
        (locals.var_igovaccd_i,)
    }
};
        locals.var_igovaccd_i = assign9160_e8236;

        let (assign9170_e8244,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9170_e8239: f64 = (-locals.var_stigfn_i);
        let assign9170_e8241: f64 = (assign9170_e8239 * locals.var_lnrtn);
        let assign9170_e8242: f64 = (assign9170_e8241).exp();
        (assign9170_e8242,)
    } else {
        (locals.var_tf_ig,)
    }
};
        locals.var_tf_ig = assign9170_e8244;

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
        let (assign9200_e8262, assign9200_e8262_d_n4, assign9200_e8262_d_n6, assign9200_e8262_d_n7, assign9200_e8262_d_n8, assign9200_e8262_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9200_e8260: f64 = (0.5 * locals.var_eg);
        (assign9200_e8260, (0.5 * locals.var_eg_dn4), (0.5 * locals.var_eg_dn6), (0.5 * locals.var_eg_dn7), (0.5 * locals.var_eg_dn8), (0.5 * locals.var_eg_dn9),)
    } else {
        (locals.var_alpha_b, locals.var_alpha_b_dn4, locals.var_alpha_b_dn6, locals.var_alpha_b_dn7, locals.var_alpha_b_dn8, locals.var_alpha_b_dn9,)
    }
};
        locals.var_alpha_b = assign9200_e8262;
        locals.var_alpha_b_dn4 = assign9200_e8262_d_n4;
        locals.var_alpha_b_dn6 = assign9200_e8262_d_n6;
        locals.var_alpha_b_dn7 = assign9200_e8262_d_n7;
        locals.var_alpha_b_dn8 = assign9200_e8262_d_n8;
        locals.var_alpha_b_dn9 = assign9200_e8262_d_n9;

        let (assign9210_e8268, assign9210_e8268_d_n4, assign9210_e8268_d_n6, assign9210_e8268_d_n7, assign9210_e8268_d_n8, assign9210_e8268_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9210_e8266: f64 = (locals.var_gco_i * locals.var_phit);
        (assign9210_e8266, (locals.var_gco_i * locals.var_phit_dn4), (locals.var_gco_i * locals.var_phit_dn6), (locals.var_gco_i * locals.var_phit_dn7), (locals.var_gco_i * locals.var_phit_dn8), (locals.var_gco_i * locals.var_phit_dn9),)
    } else {
        (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9,)
    }
};
        locals.var_dch = assign9210_e8268;
        locals.var_dch_dn4 = assign9210_e8268_d_n4;
        locals.var_dch_dn6 = assign9210_e8268_d_n6;
        locals.var_dch_dn7 = assign9210_e8268_d_n7;
        locals.var_dch_dn8 = assign9210_e8268_d_n8;
        locals.var_dch_dn9 = assign9210_e8268_d_n9;

        let (assign9220_e8274, assign9220_e8274_d_n4, assign9220_e8274_d_n6, assign9220_e8274_d_n7, assign9220_e8274_d_n8, assign9220_e8274_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9220_e8272: f64 = (locals.var_gco_i * locals.var_phit0);
        (assign9220_e8272, (locals.var_gco_i * locals.var_phit0_dn4), (locals.var_gco_i * locals.var_phit0_dn6), (locals.var_gco_i * locals.var_phit0_dn7), (locals.var_gco_i * locals.var_phit0_dn8), (locals.var_gco_i * locals.var_phit0_dn9),)
    } else {
        (locals.var_dov, locals.var_dov_dn4, locals.var_dov_dn6, locals.var_dov_dn7, locals.var_dov_dn8, locals.var_dov_dn9,)
    }
};
        locals.var_dov = assign9220_e8274;
        locals.var_dov_dn4 = assign9220_e8274_d_n4;
        locals.var_dov_dn6 = assign9220_e8274_d_n6;
        locals.var_dov_dn7 = assign9220_e8274_d_n7;
        locals.var_dov_dn8 = assign9220_e8274_d_n8;
        locals.var_dov_dn9 = assign9220_e8274_d_n9;

        let (assign9230_e8284, assign9230_e8284_d_n4, assign9230_e8284_d_n6, assign9230_e8284_d_n7, assign9230_e8284_d_n8, assign9230_e8284_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9230_e8280: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
        let assign9230_e8281: f64 = (1.0 + assign9230_e8280);
        let assign9230_e8282: f64 = (1.0 / assign9230_e8281);
        (assign9230_e8282, (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign9230_e8281 * assign9230_e8281))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign9230_e8281 * assign9230_e8281))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign9230_e8281 * assign9230_e8281))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign9230_e8281 * assign9230_e8281))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign9230_e8281 * assign9230_e8281))),)
    } else {
        (locals.var_n_iginv, locals.var_n_iginv_dn4, locals.var_n_iginv_dn6, locals.var_n_iginv_dn7, locals.var_n_iginv_dn8, locals.var_n_iginv_dn9,)
    }
};
        locals.var_n_iginv = assign9230_e8284;
        locals.var_n_iginv_dn4 = assign9230_e8284_d_n4;
        locals.var_n_iginv_dn6 = assign9230_e8284_d_n6;
        locals.var_n_iginv_dn7 = assign9230_e8284_d_n7;
        locals.var_n_iginv_dn8 = assign9230_e8284_d_n8;
        locals.var_n_iginv_dn9 = assign9230_e8284_d_n9;

        let (assign9240_e8290, assign9240_e8290_d_n4, assign9240_e8290_d_n6, assign9240_e8290_d_n7, assign9240_e8290_d_n8, assign9240_e8290_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9240_e8288: f64 = (locals.var_toxp_i * 500000000.0);
        (assign9240_e8288, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign9240_e8290;
        locals.var_temp_dn4 = assign9240_e8290_d_n4;
        locals.var_temp_dn6 = assign9240_e8290_d_n6;
        locals.var_temp_dn7 = assign9240_e8290_d_n7;
        locals.var_temp_dn8 = assign9240_e8290_d_n8;
        locals.var_temp_dn9 = assign9240_e8290_d_n9;

        let (assign9250_e8321, assign9250_e8321_d_n4, assign9250_e8321_d_n6, assign9250_e8321_d_n7, assign9250_e8321_d_n8, assign9250_e8321_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9250_e8296: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9250_e8297: f64 = (1.0 + assign9250_e8296);
        let assign9250_e8299: f64 = assign9250_e8297;
        let assign9250_e8303: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9250_e8304: f64 = (1.0 + assign9250_e8303);
        let assign9250_e8306: f64 = assign9250_e8304;
        let assign9250_e8310: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9250_e8311: f64 = (1.0 + assign9250_e8310);
        let assign9250_e8313: f64 = assign9250_e8311;
        let assign9250_e8314: f64 = (assign9250_e8306 * assign9250_e8313);
        let assign9250_e8316: f64 = (assign9250_e8314 + 0.01);
        let assign9250_e8317: f64 = (assign9250_e8316).sqrt();
        let assign9250_e8318: f64 = (assign9250_e8299 + assign9250_e8317);
        let assign9250_e8319: f64 = (0.5 * assign9250_e8318);
        (assign9250_e8319, (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign9250_e8317)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign9250_e8317)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign9250_e8317)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign9250_e8317)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign9250_e8317)))),)
    } else {
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9,)
    }
};
        locals.var_tempm = assign9250_e8321;
        locals.var_tempm_dn4 = assign9250_e8321_d_n4;
        locals.var_tempm_dn6 = assign9250_e8321_d_n6;
        locals.var_tempm_dn7 = assign9250_e8321_d_n7;
        locals.var_tempm_dn8 = assign9250_e8321_d_n8;
        locals.var_tempm_dn9 = assign9250_e8321_d_n9;

        let (assign9260_e8329, assign9260_e8329_d_n4, assign9260_e8329_d_n6, assign9260_e8329_d_n7, assign9260_e8329_d_n8, assign9260_e8329_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9260_e8325: f64 = (locals.var_bgidl_t * locals.var_tempm);
        let assign9260_e8327: f64 = (assign9260_e8325 * locals.var_temp);
        (assign9260_e8327, (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn4)), (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn6)), (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn7)), (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn8)), (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn9)),)
    } else {
        (locals.var_bgidl_i, locals.var_bgidl_i_dn4, locals.var_bgidl_i_dn6, locals.var_bgidl_i_dn7, locals.var_bgidl_i_dn8, locals.var_bgidl_i_dn9,)
    }
};
        locals.var_bgidl_i = assign9260_e8329;
        locals.var_bgidl_i_dn4 = assign9260_e8329_d_n4;
        locals.var_bgidl_i_dn6 = assign9260_e8329_d_n6;
        locals.var_bgidl_i_dn7 = assign9260_e8329_d_n7;
        locals.var_bgidl_i_dn8 = assign9260_e8329_d_n8;
        locals.var_bgidl_i_dn9 = assign9260_e8329_d_n9;

        let (assign9270_e8360, assign9270_e8360_d_n4, assign9270_e8360_d_n6, assign9270_e8360_d_n7, assign9270_e8360_d_n8, assign9270_e8360_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9270_e8335: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9270_e8336: f64 = (1.0 + assign9270_e8335);
        let assign9270_e8338: f64 = assign9270_e8336;
        let assign9270_e8342: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9270_e8343: f64 = (1.0 + assign9270_e8342);
        let assign9270_e8345: f64 = assign9270_e8343;
        let assign9270_e8349: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9270_e8350: f64 = (1.0 + assign9270_e8349);
        let assign9270_e8352: f64 = assign9270_e8350;
        let assign9270_e8353: f64 = (assign9270_e8345 * assign9270_e8352);
        let assign9270_e8355: f64 = (assign9270_e8353 + 0.01);
        let assign9270_e8356: f64 = (assign9270_e8355).sqrt();
        let assign9270_e8357: f64 = (assign9270_e8338 + assign9270_e8356);
        let assign9270_e8358: f64 = (0.5 * assign9270_e8357);
        (assign9270_e8358, (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign9270_e8356)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign9270_e8356)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign9270_e8356)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign9270_e8356)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign9270_e8356)))),)
    } else {
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9,)
    }
};
        locals.var_tempm = assign9270_e8360;
        locals.var_tempm_dn4 = assign9270_e8360_d_n4;
        locals.var_tempm_dn6 = assign9270_e8360_d_n6;
        locals.var_tempm_dn7 = assign9270_e8360_d_n7;
        locals.var_tempm_dn8 = assign9270_e8360_d_n8;
        locals.var_tempm_dn9 = assign9270_e8360_d_n9;

        let (assign9280_e8368, assign9280_e8368_d_n4, assign9280_e8368_d_n6, assign9280_e8368_d_n7, assign9280_e8368_d_n8, assign9280_e8368_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9280_e8364: f64 = (locals.var_bgidld_t * locals.var_tempm);
        let assign9280_e8366: f64 = (assign9280_e8364 * locals.var_temp);
        (assign9280_e8366, (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn4)), (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn6)), (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn7)), (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn8)), (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn9)),)
    } else {
        (locals.var_bgidld_i, locals.var_bgidld_i_dn4, locals.var_bgidld_i_dn6, locals.var_bgidld_i_dn7, locals.var_bgidld_i_dn8, locals.var_bgidld_i_dn9,)
    }
};
        locals.var_bgidld_i = assign9280_e8368;
        locals.var_bgidld_i_dn4 = assign9280_e8368_d_n4;
        locals.var_bgidld_i_dn6 = assign9280_e8368_d_n6;
        locals.var_bgidld_i_dn7 = assign9280_e8368_d_n7;
        locals.var_bgidld_i_dn8 = assign9280_e8368_d_n8;
        locals.var_bgidld_i_dn9 = assign9280_e8368_d_n9;

        let (assign9290_e8378, assign9290_e8378_d_n4, assign9290_e8378_d_n6, assign9290_e8378_d_n7, assign9290_e8378_d_n8, assign9290_e8378_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9290_e8372: f64 = (-locals.var_sta2_i);
        let assign9290_e8374: f64 = (assign9290_e8372 * locals.var_lnrtn);
        let assign9290_e8375: f64 = (assign9290_e8374).exp();
        let assign9290_e8376: f64 = (locals.var_a2_t * assign9290_e8375);
        (assign9290_e8376, (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn4))), (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn6))), (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn7))), (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn8))), (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn9))),)
    } else {
        (locals.var_a2_i, locals.var_a2_i_dn4, locals.var_a2_i_dn6, locals.var_a2_i_dn7, locals.var_a2_i_dn8, locals.var_a2_i_dn9,)
    }
};
        locals.var_a2_i = assign9290_e8378;
        locals.var_a2_i_dn4 = assign9290_e8378_d_n4;
        locals.var_a2_i_dn6 = assign9290_e8378_d_n6;
        locals.var_a2_i_dn7 = assign9290_e8378_d_n7;
        locals.var_a2_i_dn8 = assign9290_e8378_d_n8;
        locals.var_a2_i_dn9 = assign9290_e8378_d_n9;

        let (assign9300_e8384, assign9300_e8384_d_n4, assign9300_e8384_d_n6, assign9300_e8384_d_n7, assign9300_e8384_d_n8, assign9300_e8384_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9300_e8382: f64 = (locals.var_areaq_i * locals.var_phit);
        (assign9300_e8382, (locals.var_areaq_i * locals.var_phit_dn4), (locals.var_areaq_i * locals.var_phit_dn6), (locals.var_areaq_i * locals.var_phit_dn7), (locals.var_areaq_i * locals.var_phit_dn8), (locals.var_areaq_i * locals.var_phit_dn9),)
    } else {
        (locals.var_area_phit, locals.var_area_phit_dn4, locals.var_area_phit_dn6, locals.var_area_phit_dn7, locals.var_area_phit_dn8, locals.var_area_phit_dn9,)
    }
};
        locals.var_area_phit = assign9300_e8384;
        locals.var_area_phit_dn4 = assign9300_e8384_d_n4;
        locals.var_area_phit_dn6 = assign9300_e8384_d_n6;
        locals.var_area_phit_dn7 = assign9300_e8384_d_n7;
        locals.var_area_phit_dn8 = assign9300_e8384_d_n8;
        locals.var_area_phit_dn9 = assign9300_e8384_d_n9;

        let (assign9310_e8396, assign9310_e8396_d_n4, assign9310_e8396_d_n6, assign9310_e8396_d_n7, assign9310_e8396_d_n8, assign9310_e8396_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9310_e8388: f64 = (0.25 * 1.602176565e-19);
        let assign9310_e8390: f64 = (assign9310_e8388 * locals.var_nsdac_i);
        let assign9310_e8393: f64 = (locals.var_epsch * locals.var_phit);
        let assign9310_e8394: f64 = (assign9310_e8390 / assign9310_e8393);
        (assign9310_e8394, (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn4)) / (assign9310_e8393 * assign9310_e8393))), (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn6)) / (assign9310_e8393 * assign9310_e8393))), (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn7)) / (assign9310_e8393 * assign9310_e8393))), (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn8)) / (assign9310_e8393 * assign9310_e8393))), (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn9)) / (assign9310_e8393 * assign9310_e8393))),)
    } else {
        (locals.var_inner_sd, locals.var_inner_sd_dn4, locals.var_inner_sd_dn6, locals.var_inner_sd_dn7, locals.var_inner_sd_dn8, locals.var_inner_sd_dn9,)
    }
};
        locals.var_inner_sd = assign9310_e8396;
        locals.var_inner_sd_dn4 = assign9310_e8396_d_n4;
        locals.var_inner_sd_dn6 = assign9310_e8396_d_n6;
        locals.var_inner_sd_dn7 = assign9310_e8396_d_n7;
        locals.var_inner_sd_dn8 = assign9310_e8396_d_n8;
        locals.var_inner_sd_dn9 = assign9310_e8396_d_n9;

        let (assign9320_e8403, assign9320_e8403_d_n4, assign9320_e8403_d_n6, assign9320_e8403_d_n7, assign9320_e8403_d_n8, assign9320_e8403_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9320_e8400: f64 = (locals.var_nsdac_i / locals.var_neff);
        let assign9320_e8401: f64 = (assign9320_e8400).ln();
        (assign9320_e8401, ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign9320_e8400),)
    } else {
        (locals.var_xsd, locals.var_xsd_dn4, locals.var_xsd_dn6, locals.var_xsd_dn7, locals.var_xsd_dn8, locals.var_xsd_dn9,)
    }
};
        locals.var_xsd = assign9320_e8403;
        locals.var_xsd_dn4 = assign9320_e8403_d_n4;
        locals.var_xsd_dn6 = assign9320_e8403_d_n6;
        locals.var_xsd_dn7 = assign9320_e8403_d_n7;
        locals.var_xsd_dn8 = assign9320_e8403_d_n8;
        locals.var_xsd_dn9 = assign9320_e8403_d_n9;

        let (assign9330_e8411, assign9330_e8411_d_n4, assign9330_e8411_d_n6, assign9330_e8411_d_n7, assign9330_e8411_d_n8, assign9330_e8411_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9330_e8407: f64 = (locals.var_fif_i * 1.25e-6);
        let assign9330_e8409: f64 = (assign9330_e8407 * locals.var_phit);
        (assign9330_e8409, (assign9330_e8407 * locals.var_phit_dn4), (assign9330_e8407 * locals.var_phit_dn6), (assign9330_e8407 * locals.var_phit_dn7), (assign9330_e8407 * locals.var_phit_dn8), (assign9330_e8407 * locals.var_phit_dn9),)
    } else {
        (locals.var_fif_phit, locals.var_fif_phit_dn4, locals.var_fif_phit_dn6, locals.var_fif_phit_dn7, locals.var_fif_phit_dn8, locals.var_fif_phit_dn9,)
    }
};
        locals.var_fif_phit = assign9330_e8411;
        locals.var_fif_phit_dn4 = assign9330_e8411_d_n4;
        locals.var_fif_phit_dn6 = assign9330_e8411_d_n6;
        locals.var_fif_phit_dn7 = assign9330_e8411_d_n7;
        locals.var_fif_phit_dn8 = assign9330_e8411_d_n8;
        locals.var_fif_phit_dn9 = assign9330_e8411_d_n9;

        let (assign9340_e8418, assign9340_e8418_d_n4, assign9340_e8418_d_n6, assign9340_e8418_d_n7, assign9340_e8418_d_n8, assign9340_e8418_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9340_e8415: f64 = (locals.var_strth_i * locals.var_lnrtn);
        let assign9340_e8416: f64 = (assign9340_e8415).exp();
        (assign9340_e8416, (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn4)), (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn6)), (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn7)), (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn8)), (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_rth, locals.var_tf_rth_dn4, locals.var_tf_rth_dn6, locals.var_tf_rth_dn7, locals.var_tf_rth_dn8, locals.var_tf_rth_dn9,)
    }
};
        locals.var_tf_rth = assign9340_e8418;
        locals.var_tf_rth_dn4 = assign9340_e8418_d_n4;
        locals.var_tf_rth_dn6 = assign9340_e8418_d_n6;
        locals.var_tf_rth_dn7 = assign9340_e8418_d_n7;
        locals.var_tf_rth_dn8 = assign9340_e8418_d_n8;
        locals.var_tf_rth_dn9 = assign9340_e8418_d_n9;

        let (assign9350_e8424, assign9350_e8424_d_n4, assign9350_e8424_d_n6, assign9350_e8424_d_n7, assign9350_e8424_d_n8, assign9350_e8424_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9350_e8422: f64 = (locals.var_rth_t * locals.var_tf_rth);
        (assign9350_e8422, ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4)), ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6)), ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7)), ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8)), ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9)),)
    } else {
        (locals.var_rth_i, locals.var_rth_i_dn4, locals.var_rth_i_dn6, locals.var_rth_i_dn7, locals.var_rth_i_dn8, locals.var_rth_i_dn9,)
    }
};
        locals.var_rth_i = assign9350_e8424;
        locals.var_rth_i_dn4 = assign9350_e8424_d_n4;
        locals.var_rth_i_dn6 = assign9350_e8424_d_n6;
        locals.var_rth_i_dn7 = assign9350_e8424_d_n7;
        locals.var_rth_i_dn8 = assign9350_e8424_d_n8;
        locals.var_rth_i_dn9 = assign9350_e8424_d_n9;

        let (assign9360_e8432,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9360_e8428: f64 = (4.0 * 1.3806488e-23);
        let assign9360_e8430: f64 = (assign9360_e8428 * locals.var_tkc);
        (assign9360_e8430,)
    } else {
        (locals.var_nt0_4kt,)
    }
};
        locals.var_nt0_4kt = assign9360_e8432;

        let (assign9370_e8438,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9370_e8436: f64 = (locals.var_fnt_i * locals.var_nt0_4kt);
        (assign9370_e8436,)
    } else {
        (locals.var_nt,)
    }
};
        locals.var_nt = assign9370_e8438;

        let assign9380_e8441: f64 = 1.0;
        let assign9380_e8442: f64 = if p.p14 == assign9380_e8441 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign9380_e8442;

        let (assign9390_e8446, assign9390_e8446_d_n6, assign9390_e8446_d_n9,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv9 - nv6), -1.0, 1.0,)
    } else {
        (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9,)
    }
};
        locals.var_vgsu = assign9390_e8446;
        locals.var_vgsu_dn6 = assign9390_e8446_d_n6;
        locals.var_vgsu_dn9 = assign9390_e8446_d_n9;

        let (assign9400_e8450, assign9400_e8450_d_n6, assign9400_e8450_d_n7,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv7 - nv6), -1.0, 1.0,)
    } else {
        (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7,)
    }
};
        locals.var_vdsu = assign9400_e8450;
        locals.var_vdsu_dn6 = assign9400_e8450_d_n6;
        locals.var_vdsu_dn7 = assign9400_e8450_d_n7;

        let (assign9410_e8454, assign9410_e8454_d_n6, assign9410_e8454_d_n8,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv6 - nv8), 1.0, -1.0,)
    } else {
        (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8,)
    }
};
        locals.var_vsbu = assign9410_e8454;
        locals.var_vsbu_dn6 = assign9410_e8454_d_n6;
        locals.var_vsbu_dn8 = assign9410_e8454_d_n8;

        let (assign9420_e8460, assign9420_e8460_d_n6, assign9420_e8460_d_n9,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9420_e8458: f64 = (-(nv9 - nv6));
        (assign9420_e8458, 1.0, (-1.0),)
    } else {
        (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9,)
    }
};
        locals.var_vgsu = assign9420_e8460;
        locals.var_vgsu_dn6 = assign9420_e8460_d_n6;
        locals.var_vgsu_dn9 = assign9420_e8460_d_n9;

        let (assign9430_e8466, assign9430_e8466_d_n6, assign9430_e8466_d_n7,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9430_e8464: f64 = (-(nv7 - nv6));
        (assign9430_e8464, 1.0, (-1.0),)
    } else {
        (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7,)
    }
};
        locals.var_vdsu = assign9430_e8466;
        locals.var_vdsu_dn6 = assign9430_e8466_d_n6;
        locals.var_vdsu_dn7 = assign9430_e8466_d_n7;

        let (assign9440_e8472, assign9440_e8472_d_n6, assign9440_e8472_d_n8,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9440_e8470: f64 = (-(nv6 - nv8));
        (assign9440_e8470, (-1.0), 1.0,)
    } else {
        (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8,)
    }
};
        locals.var_vsbu = assign9440_e8472;
        locals.var_vsbu_dn6 = assign9440_e8472_d_n6;
        locals.var_vsbu_dn8 = assign9440_e8472_d_n8;

        let assign9450_e8474: f64 = (-locals.var_vdsu);
        locals.var_vsdu = assign9450_e8474;
        locals.var_vsdu_dn6 = (-locals.var_vdsu_dn6);
        locals.var_vsdu_dn7 = (-locals.var_vdsu_dn7);

        let assign9460_e8477: f64 = (locals.var_vgsu + locals.var_vsdu);
        locals.var_vgdu = assign9460_e8477;
        locals.var_vgdu_dn6 = (locals.var_vgsu_dn6 + locals.var_vsdu_dn6);
        locals.var_vgdu_dn7 = locals.var_vsdu_dn7;
        locals.var_vgdu_dn9 = locals.var_vgsu_dn9;

        let assign9470_e8480: f64 = (locals.var_vdsu + locals.var_vsbu);
        locals.var_vdbu = assign9470_e8480;
        locals.var_vdbu_dn6 = (locals.var_vdsu_dn6 + locals.var_vsbu_dn6);
        locals.var_vdbu_dn7 = locals.var_vdsu_dn7;
        locals.var_vdbu_dn8 = locals.var_vsbu_dn8;

        let assign9480_e8483: f64 = if locals.var_vdsu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign9480_e8483;

        let (assign9490_e8488,) = {
    if (locals.var_guard264 != 0.0) {
        let assign9490_e8486: f64 = (-1.0);
        (assign9490_e8486,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign9490_e8488;

        let (assign9500_e8492, assign9500_e8492_d_n6, assign9500_e8492_d_n7,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_vsdu, locals.var_vsdu_dn6, locals.var_vsdu_dn7,)
    } else {
        (locals.var_vds, locals.var_vds_dn6, locals.var_vds_dn7,)
    }
};
        locals.var_vds = assign9500_e8492;
        locals.var_vds_dn6 = assign9500_e8492_d_n6;
        locals.var_vds_dn7 = assign9500_e8492_d_n7;

        let (assign9510_e8496, assign9510_e8496_d_n6, assign9510_e8496_d_n7, assign9510_e8496_d_n9,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_vgdu, locals.var_vgdu_dn6, locals.var_vgdu_dn7, locals.var_vgdu_dn9,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn9,)
    }
};
        locals.var_vgs = assign9510_e8496;
        locals.var_vgs_dn6 = assign9510_e8496_d_n6;
        locals.var_vgs_dn7 = assign9510_e8496_d_n7;
        locals.var_vgs_dn9 = assign9510_e8496_d_n9;

        let (assign9520_e8500, assign9520_e8500_d_n6, assign9520_e8500_d_n7, assign9520_e8500_d_n8,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_vdbu, locals.var_vdbu_dn6, locals.var_vdbu_dn7, locals.var_vdbu_dn8,)
    } else {
        (locals.var_vsb, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn8,)
    }
};
        locals.var_vsb = assign9520_e8500;
        locals.var_vsb_dn6 = assign9520_e8500_d_n6;
        locals.var_vsb_dn7 = assign9520_e8500_d_n7;
        locals.var_vsb_dn8 = assign9520_e8500_d_n8;

        let (assign9530_e8505,) = {
    if (locals.var_guard264 == 0.0) {
        (1.0,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign9530_e8505;

        let (assign9540_e8510, assign9540_e8510_d_n6, assign9540_e8510_d_n7,) = {
    if (locals.var_guard264 == 0.0) {
        (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7,)
    } else {
        (locals.var_vds, locals.var_vds_dn6, locals.var_vds_dn7,)
    }
};
        locals.var_vds = assign9540_e8510;
        locals.var_vds_dn6 = assign9540_e8510_d_n6;
        locals.var_vds_dn7 = assign9540_e8510_d_n7;

        let (assign9550_e8515, assign9550_e8515_d_n6, assign9550_e8515_d_n7, assign9550_e8515_d_n9,) = {
    if (locals.var_guard264 == 0.0) {
        (locals.var_vgsu, locals.var_vgsu_dn6, 0.0, locals.var_vgsu_dn9,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn9,)
    }
};
        locals.var_vgs = assign9550_e8515;
        locals.var_vgs_dn6 = assign9550_e8515_d_n6;
        locals.var_vgs_dn7 = assign9550_e8515_d_n7;
        locals.var_vgs_dn9 = assign9550_e8515_d_n9;

        let (assign9560_e8520, assign9560_e8520_d_n6, assign9560_e8520_d_n7, assign9560_e8520_d_n8,) = {
    if (locals.var_guard264 == 0.0) {
        (locals.var_vsbu, locals.var_vsbu_dn6, 0.0, locals.var_vsbu_dn8,)
    } else {
        (locals.var_vsb, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn8,)
    }
};
        locals.var_vsb = assign9560_e8520;
        locals.var_vsb_dn6 = assign9560_e8520_d_n6;
        locals.var_vsb_dn7 = assign9560_e8520_d_n7;
        locals.var_vsb_dn8 = assign9560_e8520_d_n8;

        let assign9570_e8523: f64 = (locals.var_vgs + locals.var_vsb);
        locals.var_vgb = assign9570_e8523;
        locals.var_vgb_dn6 = (locals.var_vgs_dn6 + locals.var_vsb_dn6);
        locals.var_vgb_dn7 = (locals.var_vgs_dn7 + locals.var_vsb_dn7);
        locals.var_vgb_dn8 = locals.var_vsb_dn8;
        locals.var_vgb_dn9 = locals.var_vgs_dn9;

        let assign9580_e8526: f64 = (locals.var_vds * locals.var_inv_phit);
        locals.var_xd = assign9580_e8526;
        locals.var_xd_dn4 = (locals.var_vds * locals.var_inv_phit_dn4);
        locals.var_xd_dn6 = ((locals.var_vds_dn6 * locals.var_inv_phit) + (locals.var_vds * locals.var_inv_phit_dn6));
        locals.var_xd_dn7 = ((locals.var_vds_dn7 * locals.var_inv_phit) + (locals.var_vds * locals.var_inv_phit_dn7));
        locals.var_xd_dn8 = (locals.var_vds * locals.var_inv_phit_dn8);
        locals.var_xd_dn9 = (locals.var_vds * locals.var_inv_phit_dn9);

        let assign9590_e8529: f64 = (locals.var_vds * locals.var_vds);
        let assign9590_e8531: f64 = (assign9590_e8529 + 0.01);
        let assign9590_e8532: f64 = (assign9590_e8531).sqrt();
        let assign9590_e8534: f64 = (assign9590_e8532 - 0.1);
        let assign9590_e8536: f64 = (assign9590_e8534 * locals.var_inv_phit);
        locals.var_xdsx = assign9590_e8536;
        locals.var_xdsx_dn4 = (assign9590_e8534 * locals.var_inv_phit_dn4);
        locals.var_xdsx_dn6 = (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign9590_e8532)) * locals.var_inv_phit) + (assign9590_e8534 * locals.var_inv_phit_dn6));
        locals.var_xdsx_dn7 = (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign9590_e8532)) * locals.var_inv_phit) + (assign9590_e8534 * locals.var_inv_phit_dn7));
        locals.var_xdsx_dn8 = (assign9590_e8534 * locals.var_inv_phit_dn8);
        locals.var_xdsx_dn9 = (assign9590_e8534 * locals.var_inv_phit_dn9);

    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign9600_e8540: f64 = (locals.var_xd - locals.var_xdsx);
        let assign9600_e8541: f64 = (0.5 * assign9600_e8540);
        locals.var_dxdsx = assign9600_e8541;
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

        let assign9700_e8553: f64 = (locals.var_vgs - locals.var_vfb1_loc);
        let assign9700_e8555: f64 = (assign9700_e8553 * locals.var_inv_phit);
        let assign9700_e8557: f64 = (assign9700_e8555 - locals.var_dxdsx);
        let assign9700_e8559: f64 = (assign9700_e8557 - locals.var_eg_2phit0);
        locals.var_xg10 = assign9700_e8559;
        locals.var_xg10_dn4 = (((((-locals.var_vfb1_loc_dn4) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4) - locals.var_eg_2phit0_dn4);
        locals.var_xg10_dn6 = (((((locals.var_vgs_dn6 - locals.var_vfb1_loc_dn6) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6) - locals.var_eg_2phit0_dn6);
        locals.var_xg10_dn7 = (((((locals.var_vgs_dn7 - locals.var_vfb1_loc_dn7) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7) - locals.var_eg_2phit0_dn7);
        locals.var_xg10_dn8 = (((((-locals.var_vfb1_loc_dn8) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8) - locals.var_eg_2phit0_dn8);
        locals.var_xg10_dn9 = (((((locals.var_vgs_dn9 - locals.var_vfb1_loc_dn9) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9) - locals.var_eg_2phit0_dn9);

        let assign9710_e8561: f64 = (-locals.var_vsb);
        let assign9710_e8563: f64 = (assign9710_e8561 - locals.var_vfb2_loc);
        let assign9710_e8565: f64 = (assign9710_e8563 * locals.var_inv_phit);
        let assign9710_e8567: f64 = (assign9710_e8565 - locals.var_dxdsx);
        locals.var_xg20shift = assign9710_e8567;
        locals.var_xg20shift_dn4 = ((((-locals.var_vfb2_loc_dn4) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4);
        locals.var_xg20shift_dn6 = (((((-locals.var_vsb_dn6) - locals.var_vfb2_loc_dn6) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6);
        locals.var_xg20shift_dn7 = (((((-locals.var_vsb_dn7) - locals.var_vfb2_loc_dn7) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7);
        locals.var_xg20shift_dn8 = (((((-locals.var_vsb_dn8) - locals.var_vfb2_loc_dn8) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8);
        locals.var_xg20shift_dn9 = ((((-locals.var_vfb2_loc_dn9) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9);

        let assign9720_e8570: f64 = (locals.var_xg20shift - locals.var_eg_2phit0);
        locals.var_xg20 = assign9720_e8570;
        locals.var_xg20_dn4 = (locals.var_xg20shift_dn4 - locals.var_eg_2phit0_dn4);
        locals.var_xg20_dn6 = (locals.var_xg20shift_dn6 - locals.var_eg_2phit0_dn6);
        locals.var_xg20_dn7 = (locals.var_xg20shift_dn7 - locals.var_eg_2phit0_dn7);
        locals.var_xg20_dn8 = (locals.var_xg20shift_dn8 - locals.var_eg_2phit0_dn8);
        locals.var_xg20_dn9 = (locals.var_xg20shift_dn9 - locals.var_eg_2phit0_dn9);

        let assign9730_e8573: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign9730_e8573;

        let (assign9740_e8579, assign9740_e8579_d_n4, assign9740_e8579_d_n6, assign9740_e8579_d_n7, assign9740_e8579_d_n8, assign9740_e8579_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9740_e8577: f64 = (p.p14 * locals.var_typesub_i);
        (assign9740_e8577, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign9740_e8579;
        locals.var_temp_dn4 = assign9740_e8579_d_n4;
        locals.var_temp_dn6 = assign9740_e8579_d_n6;
        locals.var_temp_dn7 = assign9740_e8579_d_n7;
        locals.var_temp_dn8 = assign9740_e8579_d_n8;
        locals.var_temp_dn9 = assign9740_e8579_d_n9;

        let (assign9750_e8589, assign9750_e8589_d_n4, assign9750_e8589_d_n6, assign9750_e8589_d_n7, assign9750_e8589_d_n8, assign9750_e8589_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9750_e8583: f64 = (1.0 + locals.var_k1_1d);
        let assign9750_e8586: f64 = (1.0 + locals.var_k2_1d);
        let assign9750_e8587: f64 = (assign9750_e8583 / assign9750_e8586);
        (assign9750_e8587, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_dxth, locals.var_exp_dxth_dn4, locals.var_exp_dxth_dn6, locals.var_exp_dxth_dn7, locals.var_exp_dxth_dn8, locals.var_exp_dxth_dn9,)
    }
};
        locals.var_exp_dxth = assign9750_e8589;
        locals.var_exp_dxth_dn4 = assign9750_e8589_d_n4;
        locals.var_exp_dxth_dn6 = assign9750_e8589_d_n6;
        locals.var_exp_dxth_dn7 = assign9750_e8589_d_n7;
        locals.var_exp_dxth_dn8 = assign9750_e8589_d_n8;
        locals.var_exp_dxth_dn9 = assign9750_e8589_d_n9;

        let (assign9760_e8594, assign9760_e8594_d_n4, assign9760_e8594_d_n6, assign9760_e8594_d_n7, assign9760_e8594_d_n8, assign9760_e8594_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9760_e8592: f64 = (locals.var_exp_dxth).ln();
        (assign9760_e8592, (locals.var_exp_dxth_dn4 / locals.var_exp_dxth), (locals.var_exp_dxth_dn6 / locals.var_exp_dxth), (locals.var_exp_dxth_dn7 / locals.var_exp_dxth), (locals.var_exp_dxth_dn8 / locals.var_exp_dxth), (locals.var_exp_dxth_dn9 / locals.var_exp_dxth),)
    } else {
        (locals.var_dxth, locals.var_dxth_dn4, locals.var_dxth_dn6, locals.var_dxth_dn7, locals.var_dxth_dn8, locals.var_dxth_dn9,)
    }
};
        locals.var_dxth = assign9760_e8594;
        locals.var_dxth_dn4 = assign9760_e8594_d_n4;
        locals.var_dxth_dn6 = assign9760_e8594_d_n6;
        locals.var_dxth_dn7 = assign9760_e8594_d_n7;
        locals.var_dxth_dn8 = assign9760_e8594_d_n8;
        locals.var_dxth_dn9 = assign9760_e8594_d_n9;

        let assign9770_e8597: f64 = if locals.var_dxth > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign9770_e8597;

        let (assign9780_e8613, assign9780_e8613_d_n4, assign9780_e8613_d_n6, assign9780_e8613_d_n7, assign9780_e8613_d_n8, assign9780_e8613_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard532 != 0.0)) {
        let assign9780_e8603: f64 = (2.0 * locals.var_dxth);
        let assign9780_e8606: f64 = (locals.var_exp_dxth + 1.0);
        let assign9780_e8607: f64 = (assign9780_e8603 * assign9780_e8606);
        let assign9780_e8610: f64 = (locals.var_exp_dxth - 1.0);
        let assign9780_e8611: f64 = (assign9780_e8607 / assign9780_e8610);
        (assign9780_e8611, ((((((2.0 * locals.var_dxth_dn4) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn4)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn4)) / (assign9780_e8610 * assign9780_e8610)), ((((((2.0 * locals.var_dxth_dn6) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn6)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn6)) / (assign9780_e8610 * assign9780_e8610)), ((((((2.0 * locals.var_dxth_dn7) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn7)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn7)) / (assign9780_e8610 * assign9780_e8610)), ((((((2.0 * locals.var_dxth_dn8) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn8)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn8)) / (assign9780_e8610 * assign9780_e8610)), ((((((2.0 * locals.var_dxth_dn9) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn9)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn9)) / (assign9780_e8610 * assign9780_e8610)),)
    } else {
        (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9,)
    }
};
        locals.var_diff_min = assign9780_e8613;
        locals.var_diff_min_dn4 = assign9780_e8613_d_n4;
        locals.var_diff_min_dn6 = assign9780_e8613_d_n6;
        locals.var_diff_min_dn7 = assign9780_e8613_d_n7;
        locals.var_diff_min_dn8 = assign9780_e8613_d_n8;
        locals.var_diff_min_dn9 = assign9780_e8613_d_n9;

        let (assign9790_e8624, assign9790_e8624_d_n4, assign9790_e8624_d_n6, assign9790_e8624_d_n7, assign9790_e8624_d_n8, assign9790_e8624_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard532 == 0.0)) {
        let assign9790_e8621: f64 = (2.0 + locals.var_dxth);
        let assign9790_e8622: f64 = (2.0 * assign9790_e8621);
        (assign9790_e8622, (2.0 * locals.var_dxth_dn4), (2.0 * locals.var_dxth_dn6), (2.0 * locals.var_dxth_dn7), (2.0 * locals.var_dxth_dn8), (2.0 * locals.var_dxth_dn9),)
    } else {
        (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9,)
    }
};
        locals.var_diff_min = assign9790_e8624;
        locals.var_diff_min_dn4 = assign9790_e8624_d_n4;
        locals.var_diff_min_dn6 = assign9790_e8624_d_n6;
        locals.var_diff_min_dn7 = assign9790_e8624_d_n7;
        locals.var_diff_min_dn8 = assign9790_e8624_d_n8;
        locals.var_diff_min_dn9 = assign9790_e8624_d_n9;

        let (assign9800_e8632, assign9800_e8632_d_n4, assign9800_e8632_d_n6, assign9800_e8632_d_n7, assign9800_e8632_d_n8, assign9800_e8632_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9800_e8629: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign9800_e8630: f64 = (locals.var_a0_csisq / assign9800_e8629);
        (assign9800_e8630, (locals.var_a0_csisq_dn4 / assign9800_e8629), (locals.var_a0_csisq_dn6 / assign9800_e8629), (locals.var_a0_csisq_dn7 / assign9800_e8629), (locals.var_a0_csisq_dn8 / assign9800_e8629), (locals.var_a0_csisq_dn9 / assign9800_e8629),)
    } else {
        (locals.var_a0, locals.var_a0_dn4, locals.var_a0_dn6, locals.var_a0_dn7, locals.var_a0_dn8, locals.var_a0_dn9,)
    }
};
        locals.var_a0 = assign9800_e8632;
        locals.var_a0_dn4 = assign9800_e8632_d_n4;
        locals.var_a0_dn6 = assign9800_e8632_d_n6;
        locals.var_a0_dn7 = assign9800_e8632_d_n7;
        locals.var_a0_dn8 = assign9800_e8632_d_n8;
        locals.var_a0_dn9 = assign9800_e8632_d_n9;

        let (assign9810_e8638, assign9810_e8638_d_n4, assign9810_e8638_d_n6, assign9810_e8638_d_n7, assign9810_e8638_d_n8, assign9810_e8638_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9810_e8636: f64 = (1.0 / locals.var_k1_1d);
        (assign9810_e8636, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k1, locals.var_inv_k1_dn4, locals.var_inv_k1_dn6, locals.var_inv_k1_dn7, locals.var_inv_k1_dn8, locals.var_inv_k1_dn9,)
    }
};
        locals.var_inv_k1 = assign9810_e8638;
        locals.var_inv_k1_dn4 = assign9810_e8638_d_n4;
        locals.var_inv_k1_dn6 = assign9810_e8638_d_n6;
        locals.var_inv_k1_dn7 = assign9810_e8638_d_n7;
        locals.var_inv_k1_dn8 = assign9810_e8638_d_n8;
        locals.var_inv_k1_dn9 = assign9810_e8638_d_n9;

        let (assign9820_e8644, assign9820_e8644_d_n4, assign9820_e8644_d_n6, assign9820_e8644_d_n7, assign9820_e8644_d_n8, assign9820_e8644_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9820_e8642: f64 = (1.0 / locals.var_k2_1d);
        (assign9820_e8642, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k2, locals.var_inv_k2_dn4, locals.var_inv_k2_dn6, locals.var_inv_k2_dn7, locals.var_inv_k2_dn8, locals.var_inv_k2_dn9,)
    }
};
        locals.var_inv_k2 = assign9820_e8644;
        locals.var_inv_k2_dn4 = assign9820_e8644_d_n4;
        locals.var_inv_k2_dn6 = assign9820_e8644_d_n6;
        locals.var_inv_k2_dn7 = assign9820_e8644_d_n7;
        locals.var_inv_k2_dn8 = assign9820_e8644_d_n8;
        locals.var_inv_k2_dn9 = assign9820_e8644_d_n9;

        let (assign9830_e8654, assign9830_e8654_d_n4, assign9830_e8654_d_n6, assign9830_e8654_d_n7, assign9830_e8654_d_n8, assign9830_e8654_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9830_e8649: f64 = (1.0 + locals.var_inv_k1);
        let assign9830_e8651: f64 = (assign9830_e8649 + locals.var_inv_k2);
        let assign9830_e8652: f64 = (1.0 / assign9830_e8651);
        (assign9830_e8652, (-((locals.var_inv_k1_dn4 + locals.var_inv_k2_dn4) / (assign9830_e8651 * assign9830_e8651))), (-((locals.var_inv_k1_dn6 + locals.var_inv_k2_dn6) / (assign9830_e8651 * assign9830_e8651))), (-((locals.var_inv_k1_dn7 + locals.var_inv_k2_dn7) / (assign9830_e8651 * assign9830_e8651))), (-((locals.var_inv_k1_dn8 + locals.var_inv_k2_dn8) / (assign9830_e8651 * assign9830_e8651))), (-((locals.var_inv_k1_dn9 + locals.var_inv_k2_dn9) / (assign9830_e8651 * assign9830_e8651))),)
    } else {
        (locals.var_keq, locals.var_keq_dn4, locals.var_keq_dn6, locals.var_keq_dn7, locals.var_keq_dn8, locals.var_keq_dn9,)
    }
};
        locals.var_keq = assign9830_e8654;
        locals.var_keq_dn4 = assign9830_e8654_d_n4;
        locals.var_keq_dn6 = assign9830_e8654_d_n6;
        locals.var_keq_dn7 = assign9830_e8654_d_n7;
        locals.var_keq_dn8 = assign9830_e8654_d_n8;
        locals.var_keq_dn9 = assign9830_e8654_d_n9;

        let (assign9840_e8662, assign9840_e8662_d_n4, assign9840_e8662_d_n6, assign9840_e8662_d_n7, assign9840_e8662_d_n8, assign9840_e8662_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9840_e8659: f64 = (locals.var_xg10 - locals.var_xg20);
        let assign9840_e8660: f64 = (locals.var_keq * assign9840_e8659);
        (assign9840_e8660, ((locals.var_keq_dn4 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn4 - locals.var_xg20_dn4))), ((locals.var_keq_dn6 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn6 - locals.var_xg20_dn6))), ((locals.var_keq_dn7 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn7 - locals.var_xg20_dn7))), ((locals.var_keq_dn8 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn8 - locals.var_xg20_dn8))), ((locals.var_keq_dn9 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn9 - locals.var_xg20_dn9))),)
    } else {
        (locals.var_dx_wi, locals.var_dx_wi_dn4, locals.var_dx_wi_dn6, locals.var_dx_wi_dn7, locals.var_dx_wi_dn8, locals.var_dx_wi_dn9,)
    }
};
        locals.var_dx_wi = assign9840_e8662;
        locals.var_dx_wi_dn4 = assign9840_e8662_d_n4;
        locals.var_dx_wi_dn6 = assign9840_e8662_d_n6;
        locals.var_dx_wi_dn7 = assign9840_e8662_d_n7;
        locals.var_dx_wi_dn8 = assign9840_e8662_d_n8;
        locals.var_dx_wi_dn9 = assign9840_e8662_d_n9;

        let (assign9850_e8670, assign9850_e8670_d_n4, assign9850_e8670_d_n6, assign9850_e8670_d_n7, assign9850_e8670_d_n8, assign9850_e8670_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9850_e8667: f64 = (locals.var_dx_wi * locals.var_inv_k1);
        let assign9850_e8668: f64 = (locals.var_xg10 - assign9850_e8667);
        (assign9850_e8668, (locals.var_xg10_dn4 - ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4))), (locals.var_xg10_dn6 - ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6))), (locals.var_xg10_dn7 - ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7))), (locals.var_xg10_dn8 - ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8))), (locals.var_xg10_dn9 - ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9))),)
    } else {
        (locals.var_x1_wi0, locals.var_x1_wi0_dn4, locals.var_x1_wi0_dn6, locals.var_x1_wi0_dn7, locals.var_x1_wi0_dn8, locals.var_x1_wi0_dn9,)
    }
};
        locals.var_x1_wi0 = assign9850_e8670;
        locals.var_x1_wi0_dn4 = assign9850_e8670_d_n4;
        locals.var_x1_wi0_dn6 = assign9850_e8670_d_n6;
        locals.var_x1_wi0_dn7 = assign9850_e8670_d_n7;
        locals.var_x1_wi0_dn8 = assign9850_e8670_d_n8;
        locals.var_x1_wi0_dn9 = assign9850_e8670_d_n9;

        let (assign9860_e8678, assign9860_e8678_d_n4, assign9860_e8678_d_n6, assign9860_e8678_d_n7, assign9860_e8678_d_n8, assign9860_e8678_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9860_e8675: f64 = (locals.var_dx_wi * locals.var_inv_k2);
        let assign9860_e8676: f64 = (locals.var_xg20 + assign9860_e8675);
        (assign9860_e8676, (locals.var_xg20_dn4 + ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4))), (locals.var_xg20_dn6 + ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6))), (locals.var_xg20_dn7 + ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7))), (locals.var_xg20_dn8 + ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8))), (locals.var_xg20_dn9 + ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9))),)
    } else {
        (locals.var_x2_wi0, locals.var_x2_wi0_dn4, locals.var_x2_wi0_dn6, locals.var_x2_wi0_dn7, locals.var_x2_wi0_dn8, locals.var_x2_wi0_dn9,)
    }
};
        locals.var_x2_wi0 = assign9860_e8678;
        locals.var_x2_wi0_dn4 = assign9860_e8678_d_n4;
        locals.var_x2_wi0_dn6 = assign9860_e8678_d_n6;
        locals.var_x2_wi0_dn7 = assign9860_e8678_d_n7;
        locals.var_x2_wi0_dn8 = assign9860_e8678_d_n8;
        locals.var_x2_wi0_dn9 = assign9860_e8678_d_n9;

        let (assign9870_e8686, assign9870_e8686_d_n4, assign9870_e8686_d_n6, assign9870_e8686_d_n7, assign9870_e8686_d_n8, assign9870_e8686_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9870_e8683: f64 = (locals.var_k1_1d + 1.0);
        let assign9870_e8684: f64 = (1.0 / assign9870_e8683);
        (assign9870_e8684, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign9870_e8686;
        locals.var_q_temp1_dn4 = assign9870_e8686_d_n4;
        locals.var_q_temp1_dn6 = assign9870_e8686_d_n6;
        locals.var_q_temp1_dn7 = assign9870_e8686_d_n7;
        locals.var_q_temp1_dn8 = assign9870_e8686_d_n8;
        locals.var_q_temp1_dn9 = assign9870_e8686_d_n9;

        let (assign9880_e8694, assign9880_e8694_d_n4, assign9880_e8694_d_n6, assign9880_e8694_d_n7, assign9880_e8694_d_n8, assign9880_e8694_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9880_e8691: f64 = (locals.var_k2_1d + 1.0);
        let assign9880_e8692: f64 = (1.0 / assign9880_e8691);
        (assign9880_e8692, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign9880_e8694;
        locals.var_q_temp2_dn4 = assign9880_e8694_d_n4;
        locals.var_q_temp2_dn6 = assign9880_e8694_d_n6;
        locals.var_q_temp2_dn7 = assign9880_e8694_d_n7;
        locals.var_q_temp2_dn8 = assign9880_e8694_d_n8;
        locals.var_q_temp2_dn9 = assign9880_e8694_d_n9;

        let (assign9890_e8711, assign9890_e8711_d_n4, assign9890_e8711_d_n6, assign9890_e8711_d_n7, assign9890_e8711_d_n8, assign9890_e8711_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9890_e8699: f64 = (locals.var_k2_1d * locals.var_q_temp2);
        let assign9890_e8700: f64 = (locals.var_k1_1d + assign9890_e8699);
        let assign9890_e8702: f64 = (assign9890_e8700 * locals.var_diff_min);
        let assign9890_e8704: f64 = (assign9890_e8702 / locals.var_a0);
        let assign9890_e8705: f64 = (assign9890_e8704).ln();
        let assign9890_e8707: f64 = assign9890_e8705;
        let assign9890_e8709: f64 = (assign9890_e8707 + 1.5);
        (assign9890_e8709, (((((((locals.var_k2_1d * locals.var_q_temp2_dn4) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), (((((((locals.var_k2_1d * locals.var_q_temp2_dn6) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), (((((((locals.var_k2_1d * locals.var_q_temp2_dn7) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), (((((((locals.var_k2_1d * locals.var_q_temp2_dn8) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), (((((((locals.var_k2_1d * locals.var_q_temp2_dn9) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704),)
    } else {
        (locals.var_q_x1sat, locals.var_q_x1sat_dn4, locals.var_q_x1sat_dn6, locals.var_q_x1sat_dn7, locals.var_q_x1sat_dn8, locals.var_q_x1sat_dn9,)
    }
};
        locals.var_q_x1sat = assign9890_e8711;
        locals.var_q_x1sat_dn4 = assign9890_e8711_d_n4;
        locals.var_q_x1sat_dn6 = assign9890_e8711_d_n6;
        locals.var_q_x1sat_dn7 = assign9890_e8711_d_n7;
        locals.var_q_x1sat_dn8 = assign9890_e8711_d_n8;
        locals.var_q_x1sat_dn9 = assign9890_e8711_d_n9;

        let (assign9900_e8728, assign9900_e8728_d_n4, assign9900_e8728_d_n6, assign9900_e8728_d_n7, assign9900_e8728_d_n8, assign9900_e8728_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9900_e8716: f64 = (locals.var_k1_1d * locals.var_q_temp1);
        let assign9900_e8717: f64 = (locals.var_k2_1d + assign9900_e8716);
        let assign9900_e8719: f64 = (assign9900_e8717 * locals.var_diff_min);
        let assign9900_e8721: f64 = (assign9900_e8719 / locals.var_a0);
        let assign9900_e8722: f64 = (assign9900_e8721).ln();
        let assign9900_e8724: f64 = assign9900_e8722;
        let assign9900_e8726: f64 = (assign9900_e8724 + 1.5);
        (assign9900_e8726, (((((((locals.var_k1_1d * locals.var_q_temp1_dn4) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), (((((((locals.var_k1_1d * locals.var_q_temp1_dn6) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), (((((((locals.var_k1_1d * locals.var_q_temp1_dn7) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), (((((((locals.var_k1_1d * locals.var_q_temp1_dn8) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), (((((((locals.var_k1_1d * locals.var_q_temp1_dn9) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721),)
    } else {
        (locals.var_q_x2sat, locals.var_q_x2sat_dn4, locals.var_q_x2sat_dn6, locals.var_q_x2sat_dn7, locals.var_q_x2sat_dn8, locals.var_q_x2sat_dn9,)
    }
};
        locals.var_q_x2sat = assign9900_e8728;
        locals.var_q_x2sat_dn4 = assign9900_e8728_d_n4;
        locals.var_q_x2sat_dn6 = assign9900_e8728_d_n6;
        locals.var_q_x2sat_dn7 = assign9900_e8728_d_n7;
        locals.var_q_x2sat_dn8 = assign9900_e8728_d_n8;
        locals.var_q_x2sat_dn9 = assign9900_e8728_d_n9;

        let assign9910_e8731: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign9910_e8733: f64 = (assign9910_e8731 / 1.5);
        let assign9910_e8735: f64 = if assign9910_e8733 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard533 = assign9910_e8735;

        let (assign9920_e8749, assign9920_e8749_d_n4, assign9920_e8749_d_n6, assign9920_e8749_d_n7, assign9920_e8749_d_n8, assign9920_e8749_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard533 != 0.0)) {
        let assign9920_e8742: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign9920_e8744: f64 = (assign9920_e8742 / 1.5);
        let assign9920_e8745: f64 = (assign9920_e8744).exp();
        let assign9920_e8746: f64 = (1.0 + assign9920_e8745);
        let assign9920_e8747: f64 = (assign9920_e8746).ln();
        (assign9920_e8747, ((assign9920_e8745 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) / 1.5)) / assign9920_e8746), ((assign9920_e8745 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) / 1.5)) / assign9920_e8746), ((assign9920_e8745 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) / 1.5)) / assign9920_e8746), ((assign9920_e8745 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) / 1.5)) / assign9920_e8746), ((assign9920_e8745 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) / 1.5)) / assign9920_e8746),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign9920_e8749;
        locals.var_q_temp3_dn4 = assign9920_e8749_d_n4;
        locals.var_q_temp3_dn6 = assign9920_e8749_d_n6;
        locals.var_q_temp3_dn7 = assign9920_e8749_d_n7;
        locals.var_q_temp3_dn8 = assign9920_e8749_d_n8;
        locals.var_q_temp3_dn9 = assign9920_e8749_d_n9;

        let (assign9930_e8760, assign9930_e8760_d_n4, assign9930_e8760_d_n6, assign9930_e8760_d_n7, assign9930_e8760_d_n8, assign9930_e8760_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard533 == 0.0)) {
        let assign9930_e8756: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign9930_e8758: f64 = (assign9930_e8756 / 1.5);
        (assign9930_e8758, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) / 1.5), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) / 1.5), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) / 1.5), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) / 1.5), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) / 1.5),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign9930_e8760;
        locals.var_q_temp3_dn4 = assign9930_e8760_d_n4;
        locals.var_q_temp3_dn6 = assign9930_e8760_d_n6;
        locals.var_q_temp3_dn7 = assign9930_e8760_d_n7;
        locals.var_q_temp3_dn8 = assign9930_e8760_d_n8;
        locals.var_q_temp3_dn9 = assign9930_e8760_d_n9;

        let (assign9940_e8768, assign9940_e8768_d_n4, assign9940_e8768_d_n6, assign9940_e8768_d_n7, assign9940_e8768_d_n8, assign9940_e8768_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9940_e8765: f64 = (1.5 * locals.var_q_temp3);
        let assign9940_e8766: f64 = (locals.var_q_x1sat - assign9940_e8765);
        (assign9940_e8766, (locals.var_q_x1sat_dn4 - (1.5 * locals.var_q_temp3_dn4)), (locals.var_q_x1sat_dn6 - (1.5 * locals.var_q_temp3_dn6)), (locals.var_q_x1sat_dn7 - (1.5 * locals.var_q_temp3_dn7)), (locals.var_q_x1sat_dn8 - (1.5 * locals.var_q_temp3_dn8)), (locals.var_q_x1sat_dn9 - (1.5 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_x1, locals.var_q_x1_dn4, locals.var_q_x1_dn6, locals.var_q_x1_dn7, locals.var_q_x1_dn8, locals.var_q_x1_dn9,)
    }
};
        locals.var_q_x1 = assign9940_e8768;
        locals.var_q_x1_dn4 = assign9940_e8768_d_n4;
        locals.var_q_x1_dn6 = assign9940_e8768_d_n6;
        locals.var_q_x1_dn7 = assign9940_e8768_d_n7;
        locals.var_q_x1_dn8 = assign9940_e8768_d_n8;
        locals.var_q_x1_dn9 = assign9940_e8768_d_n9;

        let (assign9950_e8778, assign9950_e8778_d_n4, assign9950_e8778_d_n6, assign9950_e8778_d_n7, assign9950_e8778_d_n8, assign9950_e8778_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9950_e8772: f64 = (locals.var_k2_1d * locals.var_xg20);
        let assign9950_e8774: f64 = (assign9950_e8772 + locals.var_q_x1);
        let assign9950_e8776: f64 = (assign9950_e8774 * locals.var_q_temp2);
        (assign9950_e8776, ((((locals.var_k2_1d * locals.var_xg20_dn4) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn4)), ((((locals.var_k2_1d * locals.var_xg20_dn6) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn6)), ((((locals.var_k2_1d * locals.var_xg20_dn7) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn7)), ((((locals.var_k2_1d * locals.var_xg20_dn8) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn8)), ((((locals.var_k2_1d * locals.var_xg20_dn9) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_x2_wi, locals.var_q_x2_wi_dn4, locals.var_q_x2_wi_dn6, locals.var_q_x2_wi_dn7, locals.var_q_x2_wi_dn8, locals.var_q_x2_wi_dn9,)
    }
};
        locals.var_q_x2_wi = assign9950_e8778;
        locals.var_q_x2_wi_dn4 = assign9950_e8778_d_n4;
        locals.var_q_x2_wi_dn6 = assign9950_e8778_d_n6;
        locals.var_q_x2_wi_dn7 = assign9950_e8778_d_n7;
        locals.var_q_x2_wi_dn8 = assign9950_e8778_d_n8;
        locals.var_q_x2_wi_dn9 = assign9950_e8778_d_n9;

        let assign9960_e8781: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign9960_e8783: f64 = (assign9960_e8781 / 1.5);
        let assign9960_e8785: f64 = if assign9960_e8783 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard534 = assign9960_e8785;

        let (assign9970_e8799, assign9970_e8799_d_n4, assign9970_e8799_d_n6, assign9970_e8799_d_n7, assign9970_e8799_d_n8, assign9970_e8799_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard534 != 0.0)) {
        let assign9970_e8792: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign9970_e8794: f64 = (assign9970_e8792 / 1.5);
        let assign9970_e8795: f64 = (assign9970_e8794).exp();
        let assign9970_e8796: f64 = (1.0 + assign9970_e8795);
        let assign9970_e8797: f64 = (assign9970_e8796).ln();
        (assign9970_e8797, ((assign9970_e8795 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) / 1.5)) / assign9970_e8796), ((assign9970_e8795 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) / 1.5)) / assign9970_e8796), ((assign9970_e8795 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) / 1.5)) / assign9970_e8796), ((assign9970_e8795 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) / 1.5)) / assign9970_e8796), ((assign9970_e8795 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) / 1.5)) / assign9970_e8796),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign9970_e8799;
        locals.var_q_temp3_dn4 = assign9970_e8799_d_n4;
        locals.var_q_temp3_dn6 = assign9970_e8799_d_n6;
        locals.var_q_temp3_dn7 = assign9970_e8799_d_n7;
        locals.var_q_temp3_dn8 = assign9970_e8799_d_n8;
        locals.var_q_temp3_dn9 = assign9970_e8799_d_n9;

        let (assign9980_e8810, assign9980_e8810_d_n4, assign9980_e8810_d_n6, assign9980_e8810_d_n7, assign9980_e8810_d_n8, assign9980_e8810_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard534 == 0.0)) {
        let assign9980_e8806: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign9980_e8808: f64 = (assign9980_e8806 / 1.5);
        (assign9980_e8808, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) / 1.5), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) / 1.5), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) / 1.5), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) / 1.5), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) / 1.5),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign9980_e8810;
        locals.var_q_temp3_dn4 = assign9980_e8810_d_n4;
        locals.var_q_temp3_dn6 = assign9980_e8810_d_n6;
        locals.var_q_temp3_dn7 = assign9980_e8810_d_n7;
        locals.var_q_temp3_dn8 = assign9980_e8810_d_n8;
        locals.var_q_temp3_dn9 = assign9980_e8810_d_n9;

        let (assign9990_e8818, assign9990_e8818_d_n4, assign9990_e8818_d_n6, assign9990_e8818_d_n7, assign9990_e8818_d_n8, assign9990_e8818_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign9990_e8815: f64 = (1.5 * locals.var_q_temp3);
        let assign9990_e8816: f64 = (locals.var_q_x2sat - assign9990_e8815);
        (assign9990_e8816, (locals.var_q_x2sat_dn4 - (1.5 * locals.var_q_temp3_dn4)), (locals.var_q_x2sat_dn6 - (1.5 * locals.var_q_temp3_dn6)), (locals.var_q_x2sat_dn7 - (1.5 * locals.var_q_temp3_dn7)), (locals.var_q_x2sat_dn8 - (1.5 * locals.var_q_temp3_dn8)), (locals.var_q_x2sat_dn9 - (1.5 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9,)
    }
};
        locals.var_temp0 = assign9990_e8818;
        locals.var_temp0_dn4 = assign9990_e8818_d_n4;
        locals.var_temp0_dn6 = assign9990_e8818_d_n6;
        locals.var_temp0_dn7 = assign9990_e8818_d_n7;
        locals.var_temp0_dn8 = assign9990_e8818_d_n8;
        locals.var_temp0_dn9 = assign9990_e8818_d_n9;

        let (assign10000_e8824, assign10000_e8824_d_n4, assign10000_e8824_d_n6, assign10000_e8824_d_n7, assign10000_e8824_d_n8, assign10000_e8824_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign10000_e8822: f64 = (locals.var_temp * locals.var_temp0);
        (assign10000_e8822, ((locals.var_temp_dn4 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn4)), ((locals.var_temp_dn6 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn6)), ((locals.var_temp_dn7 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn7)), ((locals.var_temp_dn8 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn8)), ((locals.var_temp_dn9 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign10000_e8824;
        locals.var_temp1_dn4 = assign10000_e8824_d_n4;
        locals.var_temp1_dn6 = assign10000_e8824_d_n6;
        locals.var_temp1_dn7 = assign10000_e8824_d_n7;
        locals.var_temp1_dn8 = assign10000_e8824_d_n8;
        locals.var_temp1_dn9 = assign10000_e8824_d_n9;

    }

    pub(super) fn stamp_transient_block_22(
        locals: &mut StampLocals,
    ) {
        let (assign10010_e8830, assign10010_e8830_d_n4, assign10010_e8830_d_n6, assign10010_e8830_d_n7, assign10010_e8830_d_n8, assign10010_e8830_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign10010_e8828: f64 = (locals.var_temp * locals.var_xg20);
        (assign10010_e8828, ((locals.var_temp_dn4 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn4)), ((locals.var_temp_dn6 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn6)), ((locals.var_temp_dn7 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn7)), ((locals.var_temp_dn8 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn8)), ((locals.var_temp_dn9 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign10010_e8830;
        locals.var_temp2_dn4 = assign10010_e8830_d_n4;
        locals.var_temp2_dn6 = assign10010_e8830_d_n6;
        locals.var_temp2_dn7 = assign10010_e8830_d_n7;
        locals.var_temp2_dn8 = assign10010_e8830_d_n8;
        locals.var_temp2_dn9 = assign10010_e8830_d_n9;

        let (assign10020_e8836, assign10020_e8836_d_n4, assign10020_e8836_d_n6, assign10020_e8836_d_n7, assign10020_e8836_d_n8, assign10020_e8836_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign10020_e8834: f64 = (locals.var_temp1 - locals.var_temp2);
        (assign10020_e8834, (locals.var_temp1_dn4 - locals.var_temp2_dn4), (locals.var_temp1_dn6 - locals.var_temp2_dn6), (locals.var_temp1_dn7 - locals.var_temp2_dn7), (locals.var_temp1_dn8 - locals.var_temp2_dn8), (locals.var_temp1_dn9 - locals.var_temp2_dn9),)
    } else {
        (locals.var_spsub_xgb, locals.var_spsub_xgb_dn4, locals.var_spsub_xgb_dn6, locals.var_spsub_xgb_dn7, locals.var_spsub_xgb_dn8, locals.var_spsub_xgb_dn9,)
    }
};
        locals.var_spsub_xgb = assign10020_e8836;
        locals.var_spsub_xgb_dn4 = assign10020_e8836_d_n4;
        locals.var_spsub_xgb_dn6 = assign10020_e8836_d_n6;
        locals.var_spsub_xgb_dn7 = assign10020_e8836_d_n7;
        locals.var_spsub_xgb_dn8 = assign10020_e8836_d_n8;
        locals.var_spsub_xgb_dn9 = assign10020_e8836_d_n9;

        let assign10030_e8838: f64 = (-locals.var_xn_sub);
        let assign10030_e8839: f64 = (assign10030_e8838).abs();
        let assign10030_e8841: f64 = if assign10030_e8839 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard535 = assign10030_e8841;

        let (assign10040_e8849, assign10040_e8849_d_n4, assign10040_e8849_d_n6, assign10040_e8849_d_n7, assign10040_e8849_d_n8, assign10040_e8849_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard535 != 0.0)) {
        let assign10040_e8846: f64 = (-locals.var_xn_sub);
        let assign10040_e8847: f64 = (assign10040_e8846).exp();
        (assign10040_e8847, (assign10040_e8847 * (-locals.var_xn_sub_dn4)), (assign10040_e8847 * (-locals.var_xn_sub_dn6)), (assign10040_e8847 * (-locals.var_xn_sub_dn7)), (assign10040_e8847 * (-locals.var_xn_sub_dn8)), (assign10040_e8847 * (-locals.var_xn_sub_dn9)),)
    } else {
        (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9,)
    }
};
        locals.var_spsub_delta = assign10040_e8849;
        locals.var_spsub_delta_dn4 = assign10040_e8849_d_n4;
        locals.var_spsub_delta_dn6 = assign10040_e8849_d_n6;
        locals.var_spsub_delta_dn7 = assign10040_e8849_d_n7;
        locals.var_spsub_delta_dn8 = assign10040_e8849_d_n8;
        locals.var_spsub_delta_dn9 = assign10040_e8849_d_n9;

        let assign10050_e8851: f64 = (-locals.var_xn_sub);
        let assign10050_e8853: f64 = (-80.0);
        let assign10050_e8854: f64 = if assign10050_e8851 < assign10050_e8853 { 1.0 } else { 0.0 };
        locals.var_guard536 = assign10050_e8854;

        let (assign10060_e8891, assign10060_e8891_d_n4, assign10060_e8891_d_n6, assign10060_e8891_d_n7, assign10060_e8891_d_n8, assign10060_e8891_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard535 == 0.0)) && (locals.var_guard536 != 0.0)) {
        let assign10060_e8864: f64 = (-locals.var_xn_sub);
        let assign10060_e8865: f64 = (-assign10060_e8864);
        let assign10060_e8867: f64 = (assign10060_e8865 - 80.0);
        let assign10060_e8871: f64 = (-locals.var_xn_sub);
        let assign10060_e8872: f64 = (-assign10060_e8871);
        let assign10060_e8874: f64 = (assign10060_e8872 - 80.0);
        let assign10060_e8875: f64 = (0.5 * assign10060_e8874);
        let assign10060_e8878: f64 = (-locals.var_xn_sub);
        let assign10060_e8879: f64 = (-assign10060_e8878);
        let assign10060_e8881: f64 = (assign10060_e8879 - 80.0);
        let assign10060_e8883: f64 = (assign10060_e8881 * 0.3333333333333);
        let assign10060_e8884: f64 = (1.0 + assign10060_e8883);
        let assign10060_e8885: f64 = (assign10060_e8875 * assign10060_e8884);
        let assign10060_e8886: f64 = (1.0 + assign10060_e8885);
        let assign10060_e8887: f64 = (assign10060_e8867 * assign10060_e8886);
        let assign10060_e8888: f64 = (1.0 + assign10060_e8887);
        let assign10060_e8889: f64 = (1.80485e-35 / assign10060_e8888);
        (assign10060_e8889, (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn4)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn4))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn4)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn6)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn6))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn6)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn7)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn7))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn7)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn8)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn8))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn8)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn9)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn9))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn9)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))),)
    } else {
        (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9,)
    }
};
        locals.var_spsub_delta = assign10060_e8891;
        locals.var_spsub_delta_dn4 = assign10060_e8891_d_n4;
        locals.var_spsub_delta_dn6 = assign10060_e8891_d_n6;
        locals.var_spsub_delta_dn7 = assign10060_e8891_d_n7;
        locals.var_spsub_delta_dn8 = assign10060_e8891_d_n8;
        locals.var_spsub_delta_dn9 = assign10060_e8891_d_n9;

        let (assign10070_e8926, assign10070_e8926_d_n4, assign10070_e8926_d_n6, assign10070_e8926_d_n7, assign10070_e8926_d_n8, assign10070_e8926_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard535 == 0.0)) && (locals.var_guard536 == 0.0)) {
        let assign10070_e8902: f64 = (-locals.var_xn_sub);
        let assign10070_e8904: f64 = (assign10070_e8902 - 80.0);
        let assign10070_e8908: f64 = (-locals.var_xn_sub);
        let assign10070_e8910: f64 = (assign10070_e8908 - 80.0);
        let assign10070_e8911: f64 = (0.5 * assign10070_e8910);
        let assign10070_e8914: f64 = (-locals.var_xn_sub);
        let assign10070_e8916: f64 = (assign10070_e8914 - 80.0);
        let assign10070_e8918: f64 = (assign10070_e8916 * 0.3333333333333);
        let assign10070_e8919: f64 = (1.0 + assign10070_e8918);
        let assign10070_e8920: f64 = (assign10070_e8911 * assign10070_e8919);
        let assign10070_e8921: f64 = (1.0 + assign10070_e8920);
        let assign10070_e8922: f64 = (assign10070_e8904 * assign10070_e8921);
        let assign10070_e8923: f64 = (1.0 + assign10070_e8922);
        let assign10070_e8924: f64 = (5.54062e34 * assign10070_e8923);
        (assign10070_e8924, (5.54062e34 * (((-locals.var_xn_sub_dn4) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn4)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn6) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn6)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn7) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn7)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn8) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn8)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn9) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn9)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9,)
    }
};
        locals.var_spsub_delta = assign10070_e8926;
        locals.var_spsub_delta_dn4 = assign10070_e8926_d_n4;
        locals.var_spsub_delta_dn6 = assign10070_e8926_d_n6;
        locals.var_spsub_delta_dn7 = assign10070_e8926_d_n7;
        locals.var_spsub_delta_dn8 = assign10070_e8926_d_n8;
        locals.var_spsub_delta_dn9 = assign10070_e8926_d_n9;

        let assign10080_e8928: f64 = (locals.var_spsub_xgb).abs();
        let assign10080_e8930: f64 = if assign10080_e8928 <= locals.var_margin_sub { 1.0 } else { 0.0 };
        locals.var_guard537 = assign10080_e8930;

        let (assign10090_e8942, assign10090_e8942_d_n4, assign10090_e8942_d_n6, assign10090_e8942_d_n7, assign10090_e8942_d_n8, assign10090_e8942_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard537 != 0.0)) {
        let assign10090_e8936: f64 = (locals.var_inv_xisub * locals.var_inv_xisub);
        let assign10090_e8938: f64 = (assign10090_e8936 * 0.1666666666667);
        let assign10090_e8940: f64 = (assign10090_e8938 / 1.4142135623731);
        (assign10090_e8940, ((((locals.var_inv_xisub_dn4 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn4)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn6 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn6)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn7 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn7)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn8 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn8)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn9 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn9)) * 0.1666666666667) / 1.4142135623731),)
    } else {
        (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9,)
    }
};
        locals.var_spsub_temp1 = assign10090_e8942;
        locals.var_spsub_temp1_dn4 = assign10090_e8942_d_n4;
        locals.var_spsub_temp1_dn6 = assign10090_e8942_d_n6;
        locals.var_spsub_temp1_dn7 = assign10090_e8942_d_n7;
        locals.var_spsub_temp1_dn8 = assign10090_e8942_d_n8;
        locals.var_spsub_temp1_dn9 = assign10090_e8942_d_n9;

        let (assign10100_e8962, assign10100_e8962_d_n4, assign10100_e8962_d_n6, assign10100_e8962_d_n7, assign10100_e8962_d_n8, assign10100_e8962_d_n9,) = {
    if ((locals.var_guard531 != 0.0) && (locals.var_guard537 != 0.0)) {
        let assign10100_e8948: f64 = (locals.var_spsub_xgb * locals.var_inv_xisub);
        let assign10100_e8953: f64 = (1.0 - locals.var_spsub_delta);
        let assign10100_e8954: f64 = (locals.var_spsub_xgb * assign10100_e8953);
        let assign10100_e8956: f64 = (assign10100_e8954 * locals.var_gfsub);
        let assign10100_e8958: f64 = (assign10100_e8956 * locals.var_spsub_temp1);
        let assign10100_e8959: f64 = (1.0 + assign10100_e8958);
        let assign10100_e8960: f64 = (assign10100_e8948 * assign10100_e8959);
        (assign10100_e8960, ((((locals.var_spsub_xgb_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn4)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn4 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn4))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn4)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn4)))), ((((locals.var_spsub_xgb_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn6)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn6 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn6))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn6)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn6)))), ((((locals.var_spsub_xgb_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn7)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn7 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn7))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn7)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn7)))), ((((locals.var_spsub_xgb_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn8)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn8 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn8))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn8)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn8)))), ((((locals.var_spsub_xgb_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn9)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn9 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn9))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn9)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn9)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign10100_e8962;
        locals.var_temp3_dn4 = assign10100_e8962_d_n4;
        locals.var_temp3_dn6 = assign10100_e8962_d_n6;
        locals.var_temp3_dn7 = assign10100_e8962_d_n7;
        locals.var_temp3_dn8 = assign10100_e8962_d_n8;
        locals.var_temp3_dn9 = assign10100_e8962_d_n9;

        let assign10110_e8965: f64 = (-locals.var_margin_sub);
        let assign10110_e8966: f64 = if locals.var_spsub_xgb < assign10110_e8965 { 1.0 } else { 0.0 };
        locals.var_guard538 = assign10110_e8966;

        let (assign10120_e8976, assign10120_e8976_d_n4, assign10120_e8976_d_n6, assign10120_e8976_d_n7, assign10120_e8976_d_n8, assign10120_e8976_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10120_e8974: f64 = (-locals.var_spsub_xgb);
        (assign10120_e8974, (-locals.var_spsub_xgb_dn4), (-locals.var_spsub_xgb_dn6), (-locals.var_spsub_xgb_dn7), (-locals.var_spsub_xgb_dn8), (-locals.var_spsub_xgb_dn9),)
    } else {
        (locals.var_spsub_yg, locals.var_spsub_yg_dn4, locals.var_spsub_yg_dn6, locals.var_spsub_yg_dn7, locals.var_spsub_yg_dn8, locals.var_spsub_yg_dn9,)
    }
};
        locals.var_spsub_yg = assign10120_e8976;
        locals.var_spsub_yg_dn4 = assign10120_e8976_d_n4;
        locals.var_spsub_yg_dn6 = assign10120_e8976_d_n6;
        locals.var_spsub_yg_dn7 = assign10120_e8976_d_n7;
        locals.var_spsub_yg_dn8 = assign10120_e8976_d_n8;
        locals.var_spsub_yg_dn9 = assign10120_e8976_d_n9;

        let (assign10130_e8989, assign10130_e8989_d_n4, assign10130_e8989_d_n6, assign10130_e8989_d_n7, assign10130_e8989_d_n8, assign10130_e8989_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10130_e8986: f64 = (locals.var_spsub_yg * locals.var_inv_xisub);
        let assign10130_e8987: f64 = (1.25 * assign10130_e8986);
        (assign10130_e8987, (1.25 * ((locals.var_spsub_yg_dn4 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn4))), (1.25 * ((locals.var_spsub_yg_dn6 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn6))), (1.25 * ((locals.var_spsub_yg_dn7 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn7))), (1.25 * ((locals.var_spsub_yg_dn8 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn8))), (1.25 * ((locals.var_spsub_yg_dn9 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn9))),)
    } else {
        (locals.var_spsub_ysub, locals.var_spsub_ysub_dn4, locals.var_spsub_ysub_dn6, locals.var_spsub_ysub_dn7, locals.var_spsub_ysub_dn8, locals.var_spsub_ysub_dn9,)
    }
};
        locals.var_spsub_ysub = assign10130_e8989;
        locals.var_spsub_ysub_dn4 = assign10130_e8989_d_n4;
        locals.var_spsub_ysub_dn6 = assign10130_e8989_d_n6;
        locals.var_spsub_ysub_dn7 = assign10130_e8989_d_n7;
        locals.var_spsub_ysub_dn8 = assign10130_e8989_d_n8;
        locals.var_spsub_ysub_dn9 = assign10130_e8989_d_n9;

        let (assign10140_e9013, assign10140_e9013_d_n4, assign10140_e9013_d_n6, assign10140_e9013_d_n7, assign10140_e9013_d_n8, assign10140_e9013_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10140_e8999: f64 = (locals.var_spsub_ysub + 10.0);
        let assign10140_e9002: f64 = (locals.var_spsub_ysub - 6.0);
        let assign10140_e9005: f64 = (locals.var_spsub_ysub - 6.0);
        let assign10140_e9006: f64 = (assign10140_e9002 * assign10140_e9005);
        let assign10140_e9008: f64 = (assign10140_e9006 + 64.0);
        let assign10140_e9009: f64 = (assign10140_e9008).sqrt();
        let assign10140_e9010: f64 = (assign10140_e8999 - assign10140_e9009);
        let assign10140_e9011: f64 = (0.5 * assign10140_e9010);
        (assign10140_e9011, (0.5 * (locals.var_spsub_ysub_dn4 - (((locals.var_spsub_ysub_dn4 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn4)) / (2.0 * assign10140_e9009)))), (0.5 * (locals.var_spsub_ysub_dn6 - (((locals.var_spsub_ysub_dn6 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn6)) / (2.0 * assign10140_e9009)))), (0.5 * (locals.var_spsub_ysub_dn7 - (((locals.var_spsub_ysub_dn7 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn7)) / (2.0 * assign10140_e9009)))), (0.5 * (locals.var_spsub_ysub_dn8 - (((locals.var_spsub_ysub_dn8 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn8)) / (2.0 * assign10140_e9009)))), (0.5 * (locals.var_spsub_ysub_dn9 - (((locals.var_spsub_ysub_dn9 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn9)) / (2.0 * assign10140_e9009)))),)
    } else {
        (locals.var_spsub_eta, locals.var_spsub_eta_dn4, locals.var_spsub_eta_dn6, locals.var_spsub_eta_dn7, locals.var_spsub_eta_dn8, locals.var_spsub_eta_dn9,)
    }
};
        locals.var_spsub_eta = assign10140_e9013;
        locals.var_spsub_eta_dn4 = assign10140_e9013_d_n4;
        locals.var_spsub_eta_dn6 = assign10140_e9013_d_n6;
        locals.var_spsub_eta_dn7 = assign10140_e9013_d_n7;
        locals.var_spsub_eta_dn8 = assign10140_e9013_d_n8;
        locals.var_spsub_eta_dn9 = assign10140_e9013_d_n9;

        let (assign10150_e9024, assign10150_e9024_d_n4, assign10150_e9024_d_n6, assign10150_e9024_d_n7, assign10150_e9024_d_n8, assign10150_e9024_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10150_e9022: f64 = (locals.var_spsub_yg - locals.var_spsub_eta);
        (assign10150_e9022, (locals.var_spsub_yg_dn4 - locals.var_spsub_eta_dn4), (locals.var_spsub_yg_dn6 - locals.var_spsub_eta_dn6), (locals.var_spsub_yg_dn7 - locals.var_spsub_eta_dn7), (locals.var_spsub_yg_dn8 - locals.var_spsub_eta_dn8), (locals.var_spsub_yg_dn9 - locals.var_spsub_eta_dn9),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10150_e9024;
        locals.var_spsub_temp_dn4 = assign10150_e9024_d_n4;
        locals.var_spsub_temp_dn6 = assign10150_e9024_d_n6;
        locals.var_spsub_temp_dn7 = assign10150_e9024_d_n7;
        locals.var_spsub_temp_dn8 = assign10150_e9024_d_n8;
        locals.var_spsub_temp_dn9 = assign10150_e9024_d_n9;

        let (assign10160_e9041, assign10160_e9041_d_n4, assign10160_e9041_d_n6, assign10160_e9041_d_n7, assign10160_e9041_d_n8, assign10160_e9041_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10160_e9033: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
        let assign10160_e9037: f64 = (locals.var_spsub_eta + 1.0);
        let assign10160_e9038: f64 = (locals.var_gfsub2 * assign10160_e9037);
        let assign10160_e9039: f64 = (assign10160_e9033 + assign10160_e9038);
        (assign10160_e9039, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) + ((locals.var_gfsub2_dn4 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn4))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) + ((locals.var_gfsub2_dn6 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn6))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) + ((locals.var_gfsub2_dn7 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn7))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) + ((locals.var_gfsub2_dn8 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn8))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) + ((locals.var_gfsub2_dn9 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn9))),)
    } else {
        (locals.var_spsub_a, locals.var_spsub_a_dn4, locals.var_spsub_a_dn6, locals.var_spsub_a_dn7, locals.var_spsub_a_dn8, locals.var_spsub_a_dn9,)
    }
};
        locals.var_spsub_a = assign10160_e9041;
        locals.var_spsub_a_dn4 = assign10160_e9041_d_n4;
        locals.var_spsub_a_dn6 = assign10160_e9041_d_n6;
        locals.var_spsub_a_dn7 = assign10160_e9041_d_n7;
        locals.var_spsub_a_dn8 = assign10160_e9041_d_n8;
        locals.var_spsub_a_dn9 = assign10160_e9041_d_n9;

        let (assign10170_e9054, assign10170_e9054_d_n4, assign10170_e9054_d_n6, assign10170_e9054_d_n7, assign10170_e9054_d_n8, assign10170_e9054_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10170_e9050: f64 = (2.0 * locals.var_spsub_temp);
        let assign10170_e9052: f64 = (assign10170_e9050 - locals.var_gfsub2);
        (assign10170_e9052, ((2.0 * locals.var_spsub_temp_dn4) - locals.var_gfsub2_dn4), ((2.0 * locals.var_spsub_temp_dn6) - locals.var_gfsub2_dn6), ((2.0 * locals.var_spsub_temp_dn7) - locals.var_gfsub2_dn7), ((2.0 * locals.var_spsub_temp_dn8) - locals.var_gfsub2_dn8), ((2.0 * locals.var_spsub_temp_dn9) - locals.var_gfsub2_dn9),)
    } else {
        (locals.var_spsub_c, locals.var_spsub_c_dn4, locals.var_spsub_c_dn6, locals.var_spsub_c_dn7, locals.var_spsub_c_dn8, locals.var_spsub_c_dn9,)
    }
};
        locals.var_spsub_c = assign10170_e9054;
        locals.var_spsub_c_dn4 = assign10170_e9054_d_n4;
        locals.var_spsub_c_dn6 = assign10170_e9054_d_n6;
        locals.var_spsub_c_dn7 = assign10170_e9054_d_n7;
        locals.var_spsub_c_dn8 = assign10170_e9054_d_n8;
        locals.var_spsub_c_dn9 = assign10170_e9054_d_n9;

        let (assign10180_e9069, assign10180_e9069_d_n4, assign10180_e9069_d_n6, assign10180_e9069_d_n7, assign10180_e9069_d_n8, assign10180_e9069_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10180_e9062: f64 = (-locals.var_spsub_eta);
        let assign10180_e9065: f64 = (locals.var_spsub_a * locals.var_inv_gfsub2);
        let assign10180_e9066: f64 = (assign10180_e9065).ln();
        let assign10180_e9067: f64 = (assign10180_e9062 + assign10180_e9066);
        (assign10180_e9067, ((-locals.var_spsub_eta_dn4) + (((locals.var_spsub_a_dn4 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn4)) / assign10180_e9065)), ((-locals.var_spsub_eta_dn6) + (((locals.var_spsub_a_dn6 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn6)) / assign10180_e9065)), ((-locals.var_spsub_eta_dn7) + (((locals.var_spsub_a_dn7 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn7)) / assign10180_e9065)), ((-locals.var_spsub_eta_dn8) + (((locals.var_spsub_a_dn8 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn8)) / assign10180_e9065)), ((-locals.var_spsub_eta_dn9) + (((locals.var_spsub_a_dn9 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn9)) / assign10180_e9065)),)
    } else {
        (locals.var_spsub_tau, locals.var_spsub_tau_dn4, locals.var_spsub_tau_dn6, locals.var_spsub_tau_dn7, locals.var_spsub_tau_dn8, locals.var_spsub_tau_dn9,)
    }
};
        locals.var_spsub_tau = assign10180_e9069;
        locals.var_spsub_tau_dn4 = assign10180_e9069_d_n4;
        locals.var_spsub_tau_dn6 = assign10180_e9069_d_n6;
        locals.var_spsub_tau_dn7 = assign10180_e9069_d_n7;
        locals.var_spsub_tau_dn8 = assign10180_e9069_d_n8;
        locals.var_spsub_tau_dn9 = assign10180_e9069_d_n9;

        let (assign10190_e9080, assign10190_e9080_d_n4, assign10190_e9080_d_n6, assign10190_e9080_d_n7, assign10190_e9080_d_n8, assign10190_e9080_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10190_e9078: f64 = (locals.var_spsub_a + locals.var_spsub_c);
        (assign10190_e9078, (locals.var_spsub_a_dn4 + locals.var_spsub_c_dn4), (locals.var_spsub_a_dn6 + locals.var_spsub_c_dn6), (locals.var_spsub_a_dn7 + locals.var_spsub_c_dn7), (locals.var_spsub_a_dn8 + locals.var_spsub_c_dn8), (locals.var_spsub_a_dn9 + locals.var_spsub_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign10190_e9080;
        locals.var_nu_dn4 = assign10190_e9080_d_n4;
        locals.var_nu_dn6 = assign10190_e9080_d_n6;
        locals.var_nu_dn7 = assign10190_e9080_d_n7;
        locals.var_nu_dn8 = assign10190_e9080_d_n8;
        locals.var_nu_dn9 = assign10190_e9080_d_n9;

        let (assign10200_e9101, assign10200_e9101_d_n4, assign10200_e9101_d_n6, assign10200_e9101_d_n7, assign10200_e9101_d_n8, assign10200_e9101_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10200_e9089: f64 = (locals.var_nu * locals.var_nu);
        let assign10200_e9093: f64 = (0.5 * locals.var_spsub_c);
        let assign10200_e9095: f64 = (assign10200_e9093 * locals.var_spsub_c);
        let assign10200_e9097: f64 = (assign10200_e9095 - locals.var_spsub_a);
        let assign10200_e9098: f64 = (locals.var_spsub_tau * assign10200_e9097);
        let assign10200_e9099: f64 = (assign10200_e9089 + assign10200_e9098);
        (assign10200_e9099, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_spsub_tau_dn4 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn4) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn4)) - locals.var_spsub_a_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_spsub_tau_dn6 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn6) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn6)) - locals.var_spsub_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_spsub_tau_dn7 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn7) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn7)) - locals.var_spsub_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_spsub_tau_dn8 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn8) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn8)) - locals.var_spsub_a_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_spsub_tau_dn9 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn9) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn9)) - locals.var_spsub_a_dn9)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign10200_e9101;
        locals.var_mutau_dn4 = assign10200_e9101_d_n4;
        locals.var_mutau_dn6 = assign10200_e9101_d_n6;
        locals.var_mutau_dn7 = assign10200_e9101_d_n7;
        locals.var_mutau_dn8 = assign10200_e9101_d_n8;
        locals.var_mutau_dn9 = assign10200_e9101_d_n9;

        let (assign10210_e9136, assign10210_e9136_d_n4, assign10210_e9136_d_n6, assign10210_e9136_d_n7, assign10210_e9136_d_n8, assign10210_e9136_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10210_e9111: f64 = (locals.var_spsub_a * locals.var_nu);
        let assign10210_e9113: f64 = (assign10210_e9111 * locals.var_spsub_tau);
        let assign10210_e9117: f64 = (locals.var_nu / locals.var_mutau);
        let assign10210_e9119: f64 = (assign10210_e9117 * locals.var_spsub_tau);
        let assign10210_e9121: f64 = (assign10210_e9119 * locals.var_spsub_tau);
        let assign10210_e9123: f64 = (assign10210_e9121 * locals.var_spsub_c);
        let assign10210_e9126: f64 = (locals.var_spsub_c * locals.var_spsub_c);
        let assign10210_e9128: f64 = (assign10210_e9126 * 0.3333333333333);
        let assign10210_e9130: f64 = (assign10210_e9128 - locals.var_spsub_a);
        let assign10210_e9131: f64 = (assign10210_e9123 * assign10210_e9130);
        let assign10210_e9132: f64 = (locals.var_mutau + assign10210_e9131);
        let assign10210_e9133: f64 = (assign10210_e9113 / assign10210_e9132);
        let assign10210_e9134: f64 = (locals.var_spsub_eta + assign10210_e9133);
        (assign10210_e9134, (locals.var_spsub_eta_dn4 + (((((((locals.var_spsub_a_dn4 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn4)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn4)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn4)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn4)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn4)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn4 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn4)) * 0.3333333333333) - locals.var_spsub_a_dn4)))))) / (assign10210_e9132 * assign10210_e9132))), (locals.var_spsub_eta_dn6 + (((((((locals.var_spsub_a_dn6 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn6)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn6)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn6)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn6)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn6)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn6 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn6)) * 0.3333333333333) - locals.var_spsub_a_dn6)))))) / (assign10210_e9132 * assign10210_e9132))), (locals.var_spsub_eta_dn7 + (((((((locals.var_spsub_a_dn7 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn7)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn7)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn7)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn7)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn7)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn7 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn7)) * 0.3333333333333) - locals.var_spsub_a_dn7)))))) / (assign10210_e9132 * assign10210_e9132))), (locals.var_spsub_eta_dn8 + (((((((locals.var_spsub_a_dn8 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn8)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn8)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn8)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn8)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn8)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn8 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn8)) * 0.3333333333333) - locals.var_spsub_a_dn8)))))) / (assign10210_e9132 * assign10210_e9132))), (locals.var_spsub_eta_dn9 + (((((((locals.var_spsub_a_dn9 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn9)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn9)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn9)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn9)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn9)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn9 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn9)) * 0.3333333333333) - locals.var_spsub_a_dn9)))))) / (assign10210_e9132 * assign10210_e9132))),)
    } else {
        (locals.var_spsub_y0, locals.var_spsub_y0_dn4, locals.var_spsub_y0_dn6, locals.var_spsub_y0_dn7, locals.var_spsub_y0_dn8, locals.var_spsub_y0_dn9,)
    }
};
        locals.var_spsub_y0 = assign10210_e9136;
        locals.var_spsub_y0_dn4 = assign10210_e9136_d_n4;
        locals.var_spsub_y0_dn6 = assign10210_e9136_d_n6;
        locals.var_spsub_y0_dn7 = assign10210_e9136_d_n7;
        locals.var_spsub_y0_dn8 = assign10210_e9136_d_n8;
        locals.var_spsub_y0_dn9 = assign10210_e9136_d_n9;

        let assign10220_e9139: f64 = if locals.var_spsub_y0 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard539 = assign10220_e9139;

        let (assign10230_e9151, assign10230_e9151_d_n4, assign10230_e9151_d_n6, assign10230_e9151_d_n7, assign10230_e9151_d_n8, assign10230_e9151_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) {
        let assign10230_e9149: f64 = (locals.var_spsub_y0).exp();
        (assign10230_e9149, (assign10230_e9149 * locals.var_spsub_y0_dn4), (assign10230_e9149 * locals.var_spsub_y0_dn6), (assign10230_e9149 * locals.var_spsub_y0_dn7), (assign10230_e9149 * locals.var_spsub_y0_dn8), (assign10230_e9149 * locals.var_spsub_y0_dn9),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10230_e9151;
        locals.var_spsub_delta0_dn4 = assign10230_e9151_d_n4;
        locals.var_spsub_delta0_dn6 = assign10230_e9151_d_n6;
        locals.var_spsub_delta0_dn7 = assign10230_e9151_d_n7;
        locals.var_spsub_delta0_dn8 = assign10230_e9151_d_n8;
        locals.var_spsub_delta0_dn9 = assign10230_e9151_d_n9;

        let (assign10240_e9185, assign10240_e9185_d_n4, assign10240_e9185_d_n6, assign10240_e9185_d_n7, assign10240_e9185_d_n8, assign10240_e9185_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 == 0.0)) {
        let assign10240_e9165: f64 = (locals.var_spsub_y0 - 80.0);
        let assign10240_e9170: f64 = (locals.var_spsub_y0 - 80.0);
        let assign10240_e9171: f64 = (0.5 * assign10240_e9170);
        let assign10240_e9175: f64 = (locals.var_spsub_y0 - 80.0);
        let assign10240_e9177: f64 = (assign10240_e9175 * 0.3333333333333);
        let assign10240_e9178: f64 = (1.0 + assign10240_e9177);
        let assign10240_e9179: f64 = (assign10240_e9171 * assign10240_e9178);
        let assign10240_e9180: f64 = (1.0 + assign10240_e9179);
        let assign10240_e9181: f64 = (assign10240_e9165 * assign10240_e9180);
        let assign10240_e9182: f64 = (1.0 + assign10240_e9181);
        let assign10240_e9183: f64 = (5.54062e34 * assign10240_e9182);
        (assign10240_e9183, (5.54062e34 * ((locals.var_spsub_y0_dn4 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn4) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn6 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn6) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn7 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn7) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn8 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn8) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn9 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn9) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10240_e9185;
        locals.var_spsub_delta0_dn4 = assign10240_e9185_d_n4;
        locals.var_spsub_delta0_dn6 = assign10240_e9185_d_n6;
        locals.var_spsub_delta0_dn7 = assign10240_e9185_d_n7;
        locals.var_spsub_delta0_dn8 = assign10240_e9185_d_n8;
        locals.var_spsub_delta0_dn9 = assign10240_e9185_d_n9;

        let (assign10250_e9196, assign10250_e9196_d_n4, assign10250_e9196_d_n6, assign10250_e9196_d_n7, assign10250_e9196_d_n8, assign10250_e9196_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10250_e9194: f64 = (1.0 / locals.var_spsub_delta0);
        (assign10250_e9194, (-(locals.var_spsub_delta0_dn4 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn6 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn7 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn8 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn9 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))),)
    } else {
        (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9,)
    }
};
        locals.var_spsub_delta1 = assign10250_e9196;
        locals.var_spsub_delta1_dn4 = assign10250_e9196_d_n4;
        locals.var_spsub_delta1_dn6 = assign10250_e9196_d_n6;
        locals.var_spsub_delta1_dn7 = assign10250_e9196_d_n7;
        locals.var_spsub_delta1_dn8 = assign10250_e9196_d_n8;
        locals.var_spsub_delta1_dn9 = assign10250_e9196_d_n9;

        let (assign10260_e9211, assign10260_e9211_d_n4, assign10260_e9211_d_n6, assign10260_e9211_d_n7, assign10260_e9211_d_n8, assign10260_e9211_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10260_e9207: f64 = (locals.var_spsub_y0 * locals.var_spsub_y0);
        let assign10260_e9208: f64 = (2.0 + assign10260_e9207);
        let assign10260_e9209: f64 = (1.0 / assign10260_e9208);
        (assign10260_e9209, (-(((locals.var_spsub_y0_dn4 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn4)) / (assign10260_e9208 * assign10260_e9208))), (-(((locals.var_spsub_y0_dn6 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn6)) / (assign10260_e9208 * assign10260_e9208))), (-(((locals.var_spsub_y0_dn7 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn7)) / (assign10260_e9208 * assign10260_e9208))), (-(((locals.var_spsub_y0_dn8 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn8)) / (assign10260_e9208 * assign10260_e9208))), (-(((locals.var_spsub_y0_dn9 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn9)) / (assign10260_e9208 * assign10260_e9208))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10260_e9211;
        locals.var_spsub_temp_dn4 = assign10260_e9211_d_n4;
        locals.var_spsub_temp_dn6 = assign10260_e9211_d_n6;
        locals.var_spsub_temp_dn7 = assign10260_e9211_d_n7;
        locals.var_spsub_temp_dn8 = assign10260_e9211_d_n8;
        locals.var_spsub_temp_dn9 = assign10260_e9211_d_n9;

        let (assign10270_e9224, assign10270_e9224_d_n4, assign10270_e9224_d_n6, assign10270_e9224_d_n7, assign10270_e9224_d_n8, assign10270_e9224_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10270_e9220: f64 = (locals.var_spsub_y0 * locals.var_spsub_y0);
        let assign10270_e9222: f64 = (assign10270_e9220 * locals.var_spsub_temp);
        (assign10270_e9222, ((((locals.var_spsub_y0_dn4 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn4)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn4)), ((((locals.var_spsub_y0_dn6 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn6)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn6)), ((((locals.var_spsub_y0_dn7 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn7)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn7)), ((((locals.var_spsub_y0_dn8 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn8)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn8)), ((((locals.var_spsub_y0_dn9 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn9)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn9)),)
    } else {
        (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9,)
    }
};
        locals.var_spsub_xi0 = assign10270_e9224;
        locals.var_spsub_xi0_dn4 = assign10270_e9224_d_n4;
        locals.var_spsub_xi0_dn6 = assign10270_e9224_d_n6;
        locals.var_spsub_xi0_dn7 = assign10270_e9224_d_n7;
        locals.var_spsub_xi0_dn8 = assign10270_e9224_d_n8;
        locals.var_spsub_xi0_dn9 = assign10270_e9224_d_n9;

        let (assign10280_e9239, assign10280_e9239_d_n4, assign10280_e9239_d_n6, assign10280_e9239_d_n7, assign10280_e9239_d_n8, assign10280_e9239_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10280_e9234: f64 = (locals.var_spsub_y0 * locals.var_spsub_temp);
        let assign10280_e9236: f64 = (assign10280_e9234 * locals.var_spsub_temp);
        let assign10280_e9237: f64 = (4.0 * assign10280_e9236);
        (assign10280_e9237, (4.0 * ((((locals.var_spsub_y0_dn4 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn4))), (4.0 * ((((locals.var_spsub_y0_dn6 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn6))), (4.0 * ((((locals.var_spsub_y0_dn7 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn7))), (4.0 * ((((locals.var_spsub_y0_dn8 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn8))), (4.0 * ((((locals.var_spsub_y0_dn9 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn9))),)
    } else {
        (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9,)
    }
};
        locals.var_spsub_xi1 = assign10280_e9239;
        locals.var_spsub_xi1_dn4 = assign10280_e9239_d_n4;
        locals.var_spsub_xi1_dn6 = assign10280_e9239_d_n6;
        locals.var_spsub_xi1_dn7 = assign10280_e9239_d_n7;
        locals.var_spsub_xi1_dn8 = assign10280_e9239_d_n8;
        locals.var_spsub_xi1_dn9 = assign10280_e9239_d_n9;

        let (assign10290_e9258, assign10290_e9258_d_n4, assign10290_e9258_d_n6, assign10290_e9258_d_n7, assign10290_e9258_d_n8, assign10290_e9258_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10290_e9248: f64 = (8.0 * locals.var_spsub_temp);
        let assign10290_e9251: f64 = (12.0 * locals.var_spsub_xi0);
        let assign10290_e9252: f64 = (assign10290_e9248 - assign10290_e9251);
        let assign10290_e9254: f64 = (assign10290_e9252 * locals.var_spsub_temp);
        let assign10290_e9256: f64 = (assign10290_e9254 * locals.var_spsub_temp);
        (assign10290_e9256, ((((((8.0 * locals.var_spsub_temp_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn4)), ((((((8.0 * locals.var_spsub_temp_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn6)), ((((((8.0 * locals.var_spsub_temp_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn7)), ((((((8.0 * locals.var_spsub_temp_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn8)), ((((((8.0 * locals.var_spsub_temp_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn9)),)
    } else {
        (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9,)
    }
};
        locals.var_spsub_xi2 = assign10290_e9258;
        locals.var_spsub_xi2_dn4 = assign10290_e9258_d_n4;
        locals.var_spsub_xi2_dn6 = assign10290_e9258_d_n6;
        locals.var_spsub_xi2_dn7 = assign10290_e9258_d_n7;
        locals.var_spsub_xi2_dn8 = assign10290_e9258_d_n8;
        locals.var_spsub_xi2_dn9 = assign10290_e9258_d_n9;

        let (assign10300_e9269, assign10300_e9269_d_n4, assign10300_e9269_d_n6, assign10300_e9269_d_n7, assign10300_e9269_d_n8, assign10300_e9269_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10300_e9267: f64 = (locals.var_spsub_yg - locals.var_spsub_y0);
        (assign10300_e9267, (locals.var_spsub_yg_dn4 - locals.var_spsub_y0_dn4), (locals.var_spsub_yg_dn6 - locals.var_spsub_y0_dn6), (locals.var_spsub_yg_dn7 - locals.var_spsub_y0_dn7), (locals.var_spsub_yg_dn8 - locals.var_spsub_y0_dn8), (locals.var_spsub_yg_dn9 - locals.var_spsub_y0_dn9),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10300_e9269;
        locals.var_spsub_temp_dn4 = assign10300_e9269_d_n4;
        locals.var_spsub_temp_dn6 = assign10300_e9269_d_n6;
        locals.var_spsub_temp_dn7 = assign10300_e9269_d_n7;
        locals.var_spsub_temp_dn8 = assign10300_e9269_d_n8;
        locals.var_spsub_temp_dn9 = assign10300_e9269_d_n9;

        let (assign10310_e9280, assign10310_e9280_d_n4, assign10310_e9280_d_n6, assign10310_e9280_d_n7, assign10310_e9280_d_n8, assign10310_e9280_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10310_e9278: f64 = (locals.var_spsub_delta * locals.var_spsub_delta1);
        (assign10310_e9278, ((locals.var_spsub_delta_dn4 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn4)), ((locals.var_spsub_delta_dn6 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn6)), ((locals.var_spsub_delta_dn7 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn7)), ((locals.var_spsub_delta_dn8 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn8)), ((locals.var_spsub_delta_dn9 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn9)),)
    } else {
        (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9,)
    }
};
        locals.var_spsub_temp1 = assign10310_e9280;
        locals.var_spsub_temp1_dn4 = assign10310_e9280_d_n4;
        locals.var_spsub_temp1_dn6 = assign10310_e9280_d_n6;
        locals.var_spsub_temp1_dn7 = assign10310_e9280_d_n7;
        locals.var_spsub_temp1_dn8 = assign10310_e9280_d_n8;
        locals.var_spsub_temp1_dn9 = assign10310_e9280_d_n9;

    }

    pub(super) fn stamp_transient_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign10320_e9305, assign10320_e9305_d_n4, assign10320_e9305_d_n6, assign10320_e9305_d_n7, assign10320_e9305_d_n8, assign10320_e9305_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10320_e9289: f64 = (2.0 * locals.var_spsub_temp);
        let assign10320_e9293: f64 = (locals.var_spsub_delta0 - 1.0);
        let assign10320_e9295: f64 = (assign10320_e9293 - locals.var_spsub_temp1);
        let assign10320_e9299: f64 = (1.0 - locals.var_spsub_xi1);
        let assign10320_e9300: f64 = (locals.var_spsub_delta * assign10320_e9299);
        let assign10320_e9301: f64 = (assign10320_e9295 + assign10320_e9300);
        let assign10320_e9302: f64 = (locals.var_gfsub2 * assign10320_e9301);
        let assign10320_e9303: f64 = (assign10320_e9289 + assign10320_e9302);
        (assign10320_e9303, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn4 - locals.var_spsub_temp1_dn4) + ((locals.var_spsub_delta_dn4 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn4))))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn6 - locals.var_spsub_temp1_dn6) + ((locals.var_spsub_delta_dn6 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn6))))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn7 - locals.var_spsub_temp1_dn7) + ((locals.var_spsub_delta_dn7 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn7))))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn8 - locals.var_spsub_temp1_dn8) + ((locals.var_spsub_delta_dn8 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn8))))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn9 - locals.var_spsub_temp1_dn9) + ((locals.var_spsub_delta_dn9 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn9))))))),)
    } else {
        (locals.var_spsub_pc, locals.var_spsub_pc_dn4, locals.var_spsub_pc_dn6, locals.var_spsub_pc_dn7, locals.var_spsub_pc_dn8, locals.var_spsub_pc_dn9,)
    }
};
        locals.var_spsub_pc = assign10320_e9305;
        locals.var_spsub_pc_dn4 = assign10320_e9305_d_n4;
        locals.var_spsub_pc_dn6 = assign10320_e9305_d_n6;
        locals.var_spsub_pc_dn7 = assign10320_e9305_d_n7;
        locals.var_spsub_pc_dn8 = assign10320_e9305_d_n8;
        locals.var_spsub_pc_dn9 = assign10320_e9305_d_n9;

        let (assign10330_e9334, assign10330_e9334_d_n4, assign10330_e9334_d_n6, assign10330_e9334_d_n7, assign10330_e9334_d_n8, assign10330_e9334_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10330_e9314: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
        let assign10330_e9318: f64 = (locals.var_spsub_delta0 - locals.var_spsub_y0);
        let assign10330_e9320: f64 = (assign10330_e9318 - 1.0);
        let assign10330_e9322: f64 = (assign10330_e9320 + locals.var_spsub_temp1);
        let assign10330_e9326: f64 = (locals.var_spsub_y0 - 1.0);
        let assign10330_e9328: f64 = (assign10330_e9326 - locals.var_spsub_xi0);
        let assign10330_e9329: f64 = (locals.var_spsub_delta * assign10330_e9328);
        let assign10330_e9330: f64 = (assign10330_e9322 + assign10330_e9329);
        let assign10330_e9331: f64 = (locals.var_gfsub2 * assign10330_e9330);
        let assign10330_e9332: f64 = (assign10330_e9314 - assign10330_e9331);
        (assign10330_e9332, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn4 - locals.var_spsub_y0_dn4) + locals.var_spsub_temp1_dn4) + ((locals.var_spsub_delta_dn4 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn4 - locals.var_spsub_xi0_dn4))))))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn6 - locals.var_spsub_y0_dn6) + locals.var_spsub_temp1_dn6) + ((locals.var_spsub_delta_dn6 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn6 - locals.var_spsub_xi0_dn6))))))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn7 - locals.var_spsub_y0_dn7) + locals.var_spsub_temp1_dn7) + ((locals.var_spsub_delta_dn7 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn7 - locals.var_spsub_xi0_dn7))))))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn8 - locals.var_spsub_y0_dn8) + locals.var_spsub_temp1_dn8) + ((locals.var_spsub_delta_dn8 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn8 - locals.var_spsub_xi0_dn8))))))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn9 - locals.var_spsub_y0_dn9) + locals.var_spsub_temp1_dn9) + ((locals.var_spsub_delta_dn9 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn9 - locals.var_spsub_xi0_dn9))))))),)
    } else {
        (locals.var_spsub_qc, locals.var_spsub_qc_dn4, locals.var_spsub_qc_dn6, locals.var_spsub_qc_dn7, locals.var_spsub_qc_dn8, locals.var_spsub_qc_dn9,)
    }
};
        locals.var_spsub_qc = assign10330_e9334;
        locals.var_spsub_qc_dn4 = assign10330_e9334_d_n4;
        locals.var_spsub_qc_dn6 = assign10330_e9334_d_n6;
        locals.var_spsub_qc_dn7 = assign10330_e9334_d_n7;
        locals.var_spsub_qc_dn8 = assign10330_e9334_d_n8;
        locals.var_spsub_qc_dn9 = assign10330_e9334_d_n9;

        let (assign10340_e9353, assign10340_e9353_d_n4, assign10340_e9353_d_n6, assign10340_e9353_d_n7, assign10340_e9353_d_n8, assign10340_e9353_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10340_e9345: f64 = (locals.var_spsub_delta0 + locals.var_spsub_temp1);
        let assign10340_e9348: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
        let assign10340_e9349: f64 = (assign10340_e9345 - assign10340_e9348);
        let assign10340_e9350: f64 = (locals.var_gfsub2 * assign10340_e9349);
        let assign10340_e9351: f64 = (2.0 - assign10340_e9350);
        (assign10340_e9351, (-((locals.var_gfsub2_dn4 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn4 + locals.var_spsub_temp1_dn4) - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4)))))), (-((locals.var_gfsub2_dn6 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn6 + locals.var_spsub_temp1_dn6) - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6)))))), (-((locals.var_gfsub2_dn7 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn7 + locals.var_spsub_temp1_dn7) - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7)))))), (-((locals.var_gfsub2_dn8 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn8 + locals.var_spsub_temp1_dn8) - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8)))))), (-((locals.var_gfsub2_dn9 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn9 + locals.var_spsub_temp1_dn9) - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9)))))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10340_e9353;
        locals.var_spsub_temp_dn4 = assign10340_e9353_d_n4;
        locals.var_spsub_temp_dn6 = assign10340_e9353_d_n6;
        locals.var_spsub_temp_dn7 = assign10340_e9353_d_n7;
        locals.var_spsub_temp_dn8 = assign10340_e9353_d_n8;
        locals.var_spsub_temp_dn9 = assign10340_e9353_d_n9;

        let (assign10350_e9370, assign10350_e9370_d_n4, assign10350_e9370_d_n6, assign10350_e9370_d_n7, assign10350_e9370_d_n8, assign10350_e9370_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10350_e9362: f64 = (locals.var_spsub_pc * locals.var_spsub_pc);
        let assign10350_e9366: f64 = (locals.var_spsub_qc * locals.var_spsub_temp);
        let assign10350_e9367: f64 = (2.0 * assign10350_e9366);
        let assign10350_e9368: f64 = (assign10350_e9362 - assign10350_e9367);
        (assign10350_e9368, (((locals.var_spsub_pc_dn4 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn4)) - (2.0 * ((locals.var_spsub_qc_dn4 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn4)))), (((locals.var_spsub_pc_dn6 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn6)) - (2.0 * ((locals.var_spsub_qc_dn6 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn6)))), (((locals.var_spsub_pc_dn7 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn7)) - (2.0 * ((locals.var_spsub_qc_dn7 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn7)))), (((locals.var_spsub_pc_dn8 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn8)) - (2.0 * ((locals.var_spsub_qc_dn8 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn8)))), (((locals.var_spsub_pc_dn9 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn9)) - (2.0 * ((locals.var_spsub_qc_dn9 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn9)))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10350_e9370;
        locals.var_spsub_temp_dn4 = assign10350_e9370_d_n4;
        locals.var_spsub_temp_dn6 = assign10350_e9370_d_n6;
        locals.var_spsub_temp_dn7 = assign10350_e9370_d_n7;
        locals.var_spsub_temp_dn8 = assign10350_e9370_d_n8;
        locals.var_spsub_temp_dn9 = assign10350_e9370_d_n9;

        let (assign10360_e9389, assign10360_e9389_d_n4, assign10360_e9389_d_n6, assign10360_e9389_d_n7, assign10360_e9389_d_n8, assign10360_e9389_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign10360_e9378: f64 = (-locals.var_spsub_y0);
        let assign10360_e9383: f64 = (locals.var_spsub_temp).sqrt();
        let assign10360_e9384: f64 = (locals.var_spsub_pc + assign10360_e9383);
        let assign10360_e9385: f64 = (locals.var_spsub_qc / assign10360_e9384);
        let assign10360_e9386: f64 = (2.0 * assign10360_e9385);
        let assign10360_e9387: f64 = (assign10360_e9378 - assign10360_e9386);
        (assign10360_e9387, ((-locals.var_spsub_y0_dn4) - (2.0 * (((locals.var_spsub_qc_dn4 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn4 + (locals.var_spsub_temp_dn4 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), ((-locals.var_spsub_y0_dn6) - (2.0 * (((locals.var_spsub_qc_dn6 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn6 + (locals.var_spsub_temp_dn6 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), ((-locals.var_spsub_y0_dn7) - (2.0 * (((locals.var_spsub_qc_dn7 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn7 + (locals.var_spsub_temp_dn7 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), ((-locals.var_spsub_y0_dn8) - (2.0 * (((locals.var_spsub_qc_dn8 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn8 + (locals.var_spsub_temp_dn8 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), ((-locals.var_spsub_y0_dn9) - (2.0 * (((locals.var_spsub_qc_dn9 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn9 + (locals.var_spsub_temp_dn9 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign10360_e9389;
        locals.var_temp3_dn4 = assign10360_e9389_d_n4;
        locals.var_temp3_dn6 = assign10360_e9389_d_n6;
        locals.var_temp3_dn7 = assign10360_e9389_d_n7;
        locals.var_temp3_dn8 = assign10360_e9389_d_n8;
        locals.var_temp3_dn9 = assign10360_e9389_d_n9;

        let (assign10370_e9405, assign10370_e9405_d_n4, assign10370_e9405_d_n6, assign10370_e9405_d_n7, assign10370_e9405_d_n8, assign10370_e9405_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10370_e9401: f64 = (locals.var_gfsub * 0.732464877560822);
        let assign10370_e9402: f64 = (1.25 + assign10370_e9401);
        let assign10370_e9403: f64 = (1.0 / assign10370_e9402);
        (assign10370_e9403, (-((locals.var_gfsub_dn4 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), (-((locals.var_gfsub_dn6 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), (-((locals.var_gfsub_dn7 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), (-((locals.var_gfsub_dn8 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), (-((locals.var_gfsub_dn9 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))),)
    } else {
        (locals.var_spsub_xg1, locals.var_spsub_xg1_dn4, locals.var_spsub_xg1_dn6, locals.var_spsub_xg1_dn7, locals.var_spsub_xg1_dn8, locals.var_spsub_xg1_dn9,)
    }
};
        locals.var_spsub_xg1 = assign10370_e9405;
        locals.var_spsub_xg1_dn4 = assign10370_e9405_d_n4;
        locals.var_spsub_xg1_dn6 = assign10370_e9405_d_n6;
        locals.var_spsub_xg1_dn7 = assign10370_e9405_d_n7;
        locals.var_spsub_xg1_dn8 = assign10370_e9405_d_n8;
        locals.var_spsub_xg1_dn9 = assign10370_e9405_d_n9;

        let (assign10380_e9423, assign10380_e9423_d_n4, assign10380_e9423_d_n6, assign10380_e9423_d_n7, assign10380_e9423_d_n8, assign10380_e9423_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10380_e9415: f64 = (1.25 * locals.var_xisub);
        let assign10380_e9417: f64 = (assign10380_e9415 * locals.var_spsub_xg1);
        let assign10380_e9419: f64 = (assign10380_e9417 - 1.0);
        let assign10380_e9421: f64 = (assign10380_e9419 * locals.var_spsub_xg1);
        (assign10380_e9421, (((((1.25 * locals.var_xisub_dn4) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn4)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn4)), (((((1.25 * locals.var_xisub_dn6) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn6)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn6)), (((((1.25 * locals.var_xisub_dn7) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn7)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn7)), (((((1.25 * locals.var_xisub_dn8) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn8)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn8)), (((((1.25 * locals.var_xisub_dn9) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn9)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn9)),)
    } else {
        (locals.var_spsub_a_fac, locals.var_spsub_a_fac_dn4, locals.var_spsub_a_fac_dn6, locals.var_spsub_a_fac_dn7, locals.var_spsub_a_fac_dn8, locals.var_spsub_a_fac_dn9,)
    }
};
        locals.var_spsub_a_fac = assign10380_e9423;
        locals.var_spsub_a_fac_dn4 = assign10380_e9423_d_n4;
        locals.var_spsub_a_fac_dn6 = assign10380_e9423_d_n6;
        locals.var_spsub_a_fac_dn7 = assign10380_e9423_d_n7;
        locals.var_spsub_a_fac_dn8 = assign10380_e9423_d_n8;
        locals.var_spsub_a_fac_dn9 = assign10380_e9423_d_n9;

        let (assign10390_e9441, assign10390_e9441_d_n4, assign10390_e9441_d_n6, assign10390_e9441_d_n7, assign10390_e9441_d_n8, assign10390_e9441_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10390_e9433: f64 = (locals.var_spsub_xgb * locals.var_inv_xisub);
        let assign10390_e9437: f64 = (locals.var_spsub_a_fac * locals.var_spsub_xgb);
        let assign10390_e9438: f64 = (1.0 + assign10390_e9437);
        let assign10390_e9439: f64 = (assign10390_e9433 * assign10390_e9438);
        (assign10390_e9439, ((((locals.var_spsub_xgb_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn4)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn4 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn4)))), ((((locals.var_spsub_xgb_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn6)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn6 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn6)))), ((((locals.var_spsub_xgb_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn7)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn7 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn7)))), ((((locals.var_spsub_xgb_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn8)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn8 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn8)))), ((((locals.var_spsub_xgb_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn9)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn9 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn9)))),)
    } else {
        (locals.var_spsub_xbar, locals.var_spsub_xbar_dn4, locals.var_spsub_xbar_dn6, locals.var_spsub_xbar_dn7, locals.var_spsub_xbar_dn8, locals.var_spsub_xbar_dn9,)
    }
};
        locals.var_spsub_xbar = assign10390_e9441;
        locals.var_spsub_xbar_dn4 = assign10390_e9441_d_n4;
        locals.var_spsub_xbar_dn6 = assign10390_e9441_d_n6;
        locals.var_spsub_xbar_dn7 = assign10390_e9441_d_n7;
        locals.var_spsub_xbar_dn8 = assign10390_e9441_d_n8;
        locals.var_spsub_xbar_dn9 = assign10390_e9441_d_n9;

        let assign10400_e9443: f64 = (-locals.var_spsub_xbar);
        let assign10400_e9445: f64 = (-80.0);
        let assign10400_e9446: f64 = if assign10400_e9443 > assign10400_e9445 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign10400_e9446;

        let (assign10410_e9460, assign10410_e9460_d_n4, assign10410_e9460_d_n6, assign10410_e9460_d_n7, assign10410_e9460_d_n8, assign10410_e9460_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard540 != 0.0)) {
        let assign10410_e9457: f64 = (-locals.var_spsub_xbar);
        let assign10410_e9458: f64 = (assign10410_e9457).exp();
        (assign10410_e9458, (assign10410_e9458 * (-locals.var_spsub_xbar_dn4)), (assign10410_e9458 * (-locals.var_spsub_xbar_dn6)), (assign10410_e9458 * (-locals.var_spsub_xbar_dn7)), (assign10410_e9458 * (-locals.var_spsub_xbar_dn8)), (assign10410_e9458 * (-locals.var_spsub_xbar_dn9)),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10410_e9460;
        locals.var_spsub_temp_dn4 = assign10410_e9460_d_n4;
        locals.var_spsub_temp_dn6 = assign10410_e9460_d_n6;
        locals.var_spsub_temp_dn7 = assign10410_e9460_d_n7;
        locals.var_spsub_temp_dn8 = assign10410_e9460_d_n8;
        locals.var_spsub_temp_dn9 = assign10410_e9460_d_n9;

        let (assign10420_e9501, assign10420_e9501_d_n4, assign10420_e9501_d_n6, assign10420_e9501_d_n7, assign10420_e9501_d_n8, assign10420_e9501_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard540 == 0.0)) {
        let assign10420_e9474: f64 = (-locals.var_spsub_xbar);
        let assign10420_e9475: f64 = (-assign10420_e9474);
        let assign10420_e9477: f64 = (assign10420_e9475 - 80.0);
        let assign10420_e9481: f64 = (-locals.var_spsub_xbar);
        let assign10420_e9482: f64 = (-assign10420_e9481);
        let assign10420_e9484: f64 = (assign10420_e9482 - 80.0);
        let assign10420_e9485: f64 = (0.5 * assign10420_e9484);
        let assign10420_e9488: f64 = (-locals.var_spsub_xbar);
        let assign10420_e9489: f64 = (-assign10420_e9488);
        let assign10420_e9491: f64 = (assign10420_e9489 - 80.0);
        let assign10420_e9493: f64 = (assign10420_e9491 * 0.3333333333333);
        let assign10420_e9494: f64 = (1.0 + assign10420_e9493);
        let assign10420_e9495: f64 = (assign10420_e9485 * assign10420_e9494);
        let assign10420_e9496: f64 = (1.0 + assign10420_e9495);
        let assign10420_e9497: f64 = (assign10420_e9477 * assign10420_e9496);
        let assign10420_e9498: f64 = (1.0 + assign10420_e9497);
        let assign10420_e9499: f64 = (1.80485e-35 / assign10420_e9498);
        (assign10420_e9499, (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn4)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn4))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn4)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn6)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn6))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn6)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn7)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn7))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn7)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn8)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn8))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn8)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn9)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn9))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn9)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10420_e9501;
        locals.var_spsub_temp_dn4 = assign10420_e9501_d_n4;
        locals.var_spsub_temp_dn6 = assign10420_e9501_d_n6;
        locals.var_spsub_temp_dn7 = assign10420_e9501_d_n7;
        locals.var_spsub_temp_dn8 = assign10420_e9501_d_n8;
        locals.var_spsub_temp_dn9 = assign10420_e9501_d_n9;

        let (assign10430_e9513, assign10430_e9513_d_n4, assign10430_e9513_d_n6, assign10430_e9513_d_n7, assign10430_e9513_d_n8, assign10430_e9513_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10430_e9511: f64 = (1.0 - locals.var_spsub_temp);
        (assign10430_e9511, (-locals.var_spsub_temp_dn4), (-locals.var_spsub_temp_dn6), (-locals.var_spsub_temp_dn7), (-locals.var_spsub_temp_dn8), (-locals.var_spsub_temp_dn9),)
    } else {
        (locals.var_spsub_w, locals.var_spsub_w_dn4, locals.var_spsub_w_dn6, locals.var_spsub_w_dn7, locals.var_spsub_w_dn8, locals.var_spsub_w_dn9,)
    }
};
        locals.var_spsub_w = assign10430_e9513;
        locals.var_spsub_w_dn4 = assign10430_e9513_d_n4;
        locals.var_spsub_w_dn6 = assign10430_e9513_d_n6;
        locals.var_spsub_w_dn7 = assign10430_e9513_d_n7;
        locals.var_spsub_w_dn8 = assign10430_e9513_d_n8;
        locals.var_spsub_w_dn9 = assign10430_e9513_d_n9;

        let (assign10440_e9538, assign10440_e9538_d_n4, assign10440_e9538_d_n6, assign10440_e9538_d_n7, assign10440_e9538_d_n8, assign10440_e9538_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10440_e9524: f64 = (locals.var_gfsub2 * 0.5);
        let assign10440_e9525: f64 = (locals.var_spsub_xgb + assign10440_e9524);
        let assign10440_e9530: f64 = (locals.var_gfsub2 * 0.25);
        let assign10440_e9531: f64 = (locals.var_spsub_xgb + assign10440_e9530);
        let assign10440_e9533: f64 = (assign10440_e9531 - locals.var_spsub_w);
        let assign10440_e9534: f64 = (assign10440_e9533).sqrt();
        let assign10440_e9535: f64 = (locals.var_gfsub * assign10440_e9534);
        let assign10440_e9536: f64 = (assign10440_e9525 - assign10440_e9535);
        (assign10440_e9536, ((locals.var_spsub_xgb_dn4 + (locals.var_gfsub2_dn4 * 0.5)) - ((locals.var_gfsub_dn4 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn4 + (locals.var_gfsub2_dn4 * 0.25)) - locals.var_spsub_w_dn4) / (2.0 * assign10440_e9534))))), ((locals.var_spsub_xgb_dn6 + (locals.var_gfsub2_dn6 * 0.5)) - ((locals.var_gfsub_dn6 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn6 + (locals.var_gfsub2_dn6 * 0.25)) - locals.var_spsub_w_dn6) / (2.0 * assign10440_e9534))))), ((locals.var_spsub_xgb_dn7 + (locals.var_gfsub2_dn7 * 0.5)) - ((locals.var_gfsub_dn7 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn7 + (locals.var_gfsub2_dn7 * 0.25)) - locals.var_spsub_w_dn7) / (2.0 * assign10440_e9534))))), ((locals.var_spsub_xgb_dn8 + (locals.var_gfsub2_dn8 * 0.5)) - ((locals.var_gfsub_dn8 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn8 + (locals.var_gfsub2_dn8 * 0.25)) - locals.var_spsub_w_dn8) / (2.0 * assign10440_e9534))))), ((locals.var_spsub_xgb_dn9 + (locals.var_gfsub2_dn9 * 0.5)) - ((locals.var_gfsub_dn9 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn9 + (locals.var_gfsub2_dn9 * 0.25)) - locals.var_spsub_w_dn9) / (2.0 * assign10440_e9534))))),)
    } else {
        (locals.var_spsub_x1, locals.var_spsub_x1_dn4, locals.var_spsub_x1_dn6, locals.var_spsub_x1_dn7, locals.var_spsub_x1_dn8, locals.var_spsub_x1_dn9,)
    }
};
        locals.var_spsub_x1 = assign10440_e9538;
        locals.var_spsub_x1_dn4 = assign10440_e9538_d_n4;
        locals.var_spsub_x1_dn6 = assign10440_e9538_d_n6;
        locals.var_spsub_x1_dn7 = assign10440_e9538_d_n7;
        locals.var_spsub_x1_dn8 = assign10440_e9538_d_n8;
        locals.var_spsub_x1_dn9 = assign10440_e9538_d_n9;

        let (assign10450_e9550, assign10450_e9550_d_n4, assign10450_e9550_d_n6, assign10450_e9550_d_n7, assign10450_e9550_d_n8, assign10450_e9550_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10450_e9548: f64 = (locals.var_xn_sub + 3.0);
        (assign10450_e9548, locals.var_xn_sub_dn4, locals.var_xn_sub_dn6, locals.var_xn_sub_dn7, locals.var_xn_sub_dn8, locals.var_xn_sub_dn9,)
    } else {
        (locals.var_spsub_bx, locals.var_spsub_bx_dn4, locals.var_spsub_bx_dn6, locals.var_spsub_bx_dn7, locals.var_spsub_bx_dn8, locals.var_spsub_bx_dn9,)
    }
};
        locals.var_spsub_bx = assign10450_e9550;
        locals.var_spsub_bx_dn4 = assign10450_e9550_d_n4;
        locals.var_spsub_bx_dn6 = assign10450_e9550_d_n6;
        locals.var_spsub_bx_dn7 = assign10450_e9550_d_n7;
        locals.var_spsub_bx_dn8 = assign10450_e9550_d_n8;
        locals.var_spsub_bx_dn9 = assign10450_e9550_d_n9;

        let (assign10460_e9586, assign10460_e9586_d_n4, assign10460_e9586_d_n6, assign10460_e9586_d_n7, assign10460_e9586_d_n8, assign10460_e9586_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10460_e9561: f64 = (locals.var_spsub_x1 + locals.var_spsub_bx);
        let assign10460_e9564: f64 = (locals.var_spsub_x1 - locals.var_spsub_bx);
        let assign10460_e9567: f64 = (locals.var_spsub_x1 - locals.var_spsub_bx);
        let assign10460_e9568: f64 = (assign10460_e9564 * assign10460_e9567);
        let assign10460_e9570: f64 = (assign10460_e9568 + 5.0);
        let assign10460_e9571: f64 = (assign10460_e9570).sqrt();
        let assign10460_e9572: f64 = (assign10460_e9561 - assign10460_e9571);
        let assign10460_e9573: f64 = (0.5 * assign10460_e9572);
        let assign10460_e9578: f64 = (locals.var_spsub_bx * locals.var_spsub_bx);
        let assign10460_e9580: f64 = (assign10460_e9578 + 5.0);
        let assign10460_e9581: f64 = (assign10460_e9580).sqrt();
        let assign10460_e9582: f64 = (locals.var_spsub_bx - assign10460_e9581);
        let assign10460_e9583: f64 = (0.5 * assign10460_e9582);
        let assign10460_e9584: f64 = (assign10460_e9573 - assign10460_e9583);
        (assign10460_e9584, ((0.5 * ((locals.var_spsub_x1_dn4 + locals.var_spsub_bx_dn4) - ((((locals.var_spsub_x1_dn4 - locals.var_spsub_bx_dn4) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn4 - locals.var_spsub_bx_dn4))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn4 - (((locals.var_spsub_bx_dn4 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn4)) / (2.0 * assign10460_e9581))))), ((0.5 * ((locals.var_spsub_x1_dn6 + locals.var_spsub_bx_dn6) - ((((locals.var_spsub_x1_dn6 - locals.var_spsub_bx_dn6) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn6 - locals.var_spsub_bx_dn6))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn6 - (((locals.var_spsub_bx_dn6 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn6)) / (2.0 * assign10460_e9581))))), ((0.5 * ((locals.var_spsub_x1_dn7 + locals.var_spsub_bx_dn7) - ((((locals.var_spsub_x1_dn7 - locals.var_spsub_bx_dn7) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn7 - locals.var_spsub_bx_dn7))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn7 - (((locals.var_spsub_bx_dn7 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn7)) / (2.0 * assign10460_e9581))))), ((0.5 * ((locals.var_spsub_x1_dn8 + locals.var_spsub_bx_dn8) - ((((locals.var_spsub_x1_dn8 - locals.var_spsub_bx_dn8) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn8 - locals.var_spsub_bx_dn8))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn8 - (((locals.var_spsub_bx_dn8 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn8)) / (2.0 * assign10460_e9581))))), ((0.5 * ((locals.var_spsub_x1_dn9 + locals.var_spsub_bx_dn9) - ((((locals.var_spsub_x1_dn9 - locals.var_spsub_bx_dn9) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn9 - locals.var_spsub_bx_dn9))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn9 - (((locals.var_spsub_bx_dn9 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn9)) / (2.0 * assign10460_e9581))))),)
    } else {
        (locals.var_spsub_eta, locals.var_spsub_eta_dn4, locals.var_spsub_eta_dn6, locals.var_spsub_eta_dn7, locals.var_spsub_eta_dn8, locals.var_spsub_eta_dn9,)
    }
};
        locals.var_spsub_eta = assign10460_e9586;
        locals.var_spsub_eta_dn4 = assign10460_e9586_d_n4;
        locals.var_spsub_eta_dn6 = assign10460_e9586_d_n6;
        locals.var_spsub_eta_dn7 = assign10460_e9586_d_n7;
        locals.var_spsub_eta_dn8 = assign10460_e9586_d_n8;
        locals.var_spsub_eta_dn9 = assign10460_e9586_d_n9;

        let (assign10470_e9598, assign10470_e9598_d_n4, assign10470_e9598_d_n6, assign10470_e9598_d_n7, assign10470_e9598_d_n8, assign10470_e9598_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10470_e9596: f64 = (locals.var_spsub_xgb - locals.var_spsub_eta);
        (assign10470_e9596, (locals.var_spsub_xgb_dn4 - locals.var_spsub_eta_dn4), (locals.var_spsub_xgb_dn6 - locals.var_spsub_eta_dn6), (locals.var_spsub_xgb_dn7 - locals.var_spsub_eta_dn7), (locals.var_spsub_xgb_dn8 - locals.var_spsub_eta_dn8), (locals.var_spsub_xgb_dn9 - locals.var_spsub_eta_dn9),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10470_e9598;
        locals.var_spsub_temp_dn4 = assign10470_e9598_d_n4;
        locals.var_spsub_temp_dn6 = assign10470_e9598_d_n6;
        locals.var_spsub_temp_dn7 = assign10470_e9598_d_n7;
        locals.var_spsub_temp_dn8 = assign10470_e9598_d_n8;
        locals.var_spsub_temp_dn9 = assign10470_e9598_d_n9;

        let (assign10480_e9610, assign10480_e9610_d_n4, assign10480_e9610_d_n6, assign10480_e9610_d_n7, assign10480_e9610_d_n8, assign10480_e9610_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10480_e9607: f64 = (-locals.var_spsub_eta);
        let assign10480_e9608: f64 = (assign10480_e9607).exp();
        (assign10480_e9608, (assign10480_e9608 * (-locals.var_spsub_eta_dn4)), (assign10480_e9608 * (-locals.var_spsub_eta_dn6)), (assign10480_e9608 * (-locals.var_spsub_eta_dn7)), (assign10480_e9608 * (-locals.var_spsub_eta_dn8)), (assign10480_e9608 * (-locals.var_spsub_eta_dn9)),)
    } else {
        (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9,)
    }
};
        locals.var_spsub_temp1 = assign10480_e9610;
        locals.var_spsub_temp1_dn4 = assign10480_e9610_d_n4;
        locals.var_spsub_temp1_dn6 = assign10480_e9610_d_n6;
        locals.var_spsub_temp1_dn7 = assign10480_e9610_d_n7;
        locals.var_spsub_temp1_dn8 = assign10480_e9610_d_n8;
        locals.var_spsub_temp1_dn9 = assign10480_e9610_d_n9;

        let (assign10490_e9626, assign10490_e9626_d_n4, assign10490_e9626_d_n6, assign10490_e9626_d_n7, assign10490_e9626_d_n8, assign10490_e9626_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10490_e9622: f64 = (locals.var_spsub_eta * locals.var_spsub_eta);
        let assign10490_e9623: f64 = (2.0 + assign10490_e9622);
        let assign10490_e9624: f64 = (1.0 / assign10490_e9623);
        (assign10490_e9624, (-(((locals.var_spsub_eta_dn4 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn4)) / (assign10490_e9623 * assign10490_e9623))), (-(((locals.var_spsub_eta_dn6 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn6)) / (assign10490_e9623 * assign10490_e9623))), (-(((locals.var_spsub_eta_dn7 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn7)) / (assign10490_e9623 * assign10490_e9623))), (-(((locals.var_spsub_eta_dn8 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn8)) / (assign10490_e9623 * assign10490_e9623))), (-(((locals.var_spsub_eta_dn9 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn9)) / (assign10490_e9623 * assign10490_e9623))),)
    } else {
        (locals.var_spsub_temp2, locals.var_spsub_temp2_dn4, locals.var_spsub_temp2_dn6, locals.var_spsub_temp2_dn7, locals.var_spsub_temp2_dn8, locals.var_spsub_temp2_dn9,)
    }
};
        locals.var_spsub_temp2 = assign10490_e9626;
        locals.var_spsub_temp2_dn4 = assign10490_e9626_d_n4;
        locals.var_spsub_temp2_dn6 = assign10490_e9626_d_n6;
        locals.var_spsub_temp2_dn7 = assign10490_e9626_d_n7;
        locals.var_spsub_temp2_dn8 = assign10490_e9626_d_n8;
        locals.var_spsub_temp2_dn9 = assign10490_e9626_d_n9;

        let (assign10500_e9640, assign10500_e9640_d_n4, assign10500_e9640_d_n6, assign10500_e9640_d_n7, assign10500_e9640_d_n8, assign10500_e9640_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10500_e9636: f64 = (locals.var_spsub_eta * locals.var_spsub_eta);
        let assign10500_e9638: f64 = (assign10500_e9636 * locals.var_spsub_temp2);
        (assign10500_e9638, ((((locals.var_spsub_eta_dn4 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn4)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn4)), ((((locals.var_spsub_eta_dn6 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn6)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn6)), ((((locals.var_spsub_eta_dn7 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn7)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn7)), ((((locals.var_spsub_eta_dn8 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn8)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn8)), ((((locals.var_spsub_eta_dn9 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn9)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn9)),)
    } else {
        (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9,)
    }
};
        locals.var_spsub_xi0 = assign10500_e9640;
        locals.var_spsub_xi0_dn4 = assign10500_e9640_d_n4;
        locals.var_spsub_xi0_dn6 = assign10500_e9640_d_n6;
        locals.var_spsub_xi0_dn7 = assign10500_e9640_d_n7;
        locals.var_spsub_xi0_dn8 = assign10500_e9640_d_n8;
        locals.var_spsub_xi0_dn9 = assign10500_e9640_d_n9;

        let (assign10510_e9656, assign10510_e9656_d_n4, assign10510_e9656_d_n6, assign10510_e9656_d_n7, assign10510_e9656_d_n8, assign10510_e9656_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10510_e9651: f64 = (locals.var_spsub_eta * locals.var_spsub_temp2);
        let assign10510_e9653: f64 = (assign10510_e9651 * locals.var_spsub_temp2);
        let assign10510_e9654: f64 = (4.0 * assign10510_e9653);
        (assign10510_e9654, (4.0 * ((((locals.var_spsub_eta_dn4 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn4)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn4))), (4.0 * ((((locals.var_spsub_eta_dn6 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn6)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn6))), (4.0 * ((((locals.var_spsub_eta_dn7 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn7)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn7))), (4.0 * ((((locals.var_spsub_eta_dn8 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn8)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn8))), (4.0 * ((((locals.var_spsub_eta_dn9 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn9)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn9))),)
    } else {
        (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9,)
    }
};
        locals.var_spsub_xi1 = assign10510_e9656;
        locals.var_spsub_xi1_dn4 = assign10510_e9656_d_n4;
        locals.var_spsub_xi1_dn6 = assign10510_e9656_d_n6;
        locals.var_spsub_xi1_dn7 = assign10510_e9656_d_n7;
        locals.var_spsub_xi1_dn8 = assign10510_e9656_d_n8;
        locals.var_spsub_xi1_dn9 = assign10510_e9656_d_n9;

        let (assign10520_e9676, assign10520_e9676_d_n4, assign10520_e9676_d_n6, assign10520_e9676_d_n7, assign10520_e9676_d_n8, assign10520_e9676_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10520_e9666: f64 = (8.0 * locals.var_spsub_temp2);
        let assign10520_e9669: f64 = (12.0 * locals.var_spsub_xi0);
        let assign10520_e9670: f64 = (assign10520_e9666 - assign10520_e9669);
        let assign10520_e9672: f64 = (assign10520_e9670 * locals.var_spsub_temp2);
        let assign10520_e9674: f64 = (assign10520_e9672 * locals.var_spsub_temp2);
        (assign10520_e9674, ((((((8.0 * locals.var_spsub_temp2_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn4)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn4)), ((((((8.0 * locals.var_spsub_temp2_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn6)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn6)), ((((((8.0 * locals.var_spsub_temp2_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn7)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn7)), ((((((8.0 * locals.var_spsub_temp2_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn8)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn8)), ((((((8.0 * locals.var_spsub_temp2_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn9)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn9)),)
    } else {
        (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9,)
    }
};
        locals.var_spsub_xi2 = assign10520_e9676;
        locals.var_spsub_xi2_dn4 = assign10520_e9676_d_n4;
        locals.var_spsub_xi2_dn6 = assign10520_e9676_d_n6;
        locals.var_spsub_xi2_dn7 = assign10520_e9676_d_n7;
        locals.var_spsub_xi2_dn8 = assign10520_e9676_d_n8;
        locals.var_spsub_xi2_dn9 = assign10520_e9676_d_n9;

        let (assign10530_e9706, assign10530_e9706_d_n4, assign10530_e9706_d_n6, assign10530_e9706_d_n7, assign10530_e9706_d_n8, assign10530_e9706_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10530_e9687: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
        let assign10530_e9691: f64 = (locals.var_spsub_temp1 + locals.var_spsub_eta);
        let assign10530_e9693: f64 = (assign10530_e9691 - 1.0);
        let assign10530_e9697: f64 = (locals.var_spsub_eta + 1.0);
        let assign10530_e9699: f64 = (assign10530_e9697 + locals.var_spsub_xi0);
        let assign10530_e9700: f64 = (locals.var_spsub_delta * assign10530_e9699);
        let assign10530_e9701: f64 = (assign10530_e9693 - assign10530_e9700);
        let assign10530_e9702: f64 = (locals.var_gfsub2 * assign10530_e9701);
        let assign10530_e9703: f64 = (assign10530_e9687 - assign10530_e9702);
        let assign10530_e9704: f64 = (1e-40_f64).max(assign10530_e9703);
        (assign10530_e9704, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn4 + locals.var_spsub_eta_dn4) - ((locals.var_spsub_delta_dn4 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn4 + locals.var_spsub_xi0_dn4))))))) }, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn6 + locals.var_spsub_eta_dn6) - ((locals.var_spsub_delta_dn6 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn6 + locals.var_spsub_xi0_dn6))))))) }, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn7 + locals.var_spsub_eta_dn7) - ((locals.var_spsub_delta_dn7 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn7 + locals.var_spsub_xi0_dn7))))))) }, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn8 + locals.var_spsub_eta_dn8) - ((locals.var_spsub_delta_dn8 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn8 + locals.var_spsub_xi0_dn8))))))) }, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn9 + locals.var_spsub_eta_dn9) - ((locals.var_spsub_delta_dn9 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn9 + locals.var_spsub_xi0_dn9))))))) },)
    } else {
        (locals.var_spsub_a, locals.var_spsub_a_dn4, locals.var_spsub_a_dn6, locals.var_spsub_a_dn7, locals.var_spsub_a_dn8, locals.var_spsub_a_dn9,)
    }
};
        locals.var_spsub_a = assign10530_e9706;
        locals.var_spsub_a_dn4 = assign10530_e9706_d_n4;
        locals.var_spsub_a_dn6 = assign10530_e9706_d_n6;
        locals.var_spsub_a_dn7 = assign10530_e9706_d_n7;
        locals.var_spsub_a_dn8 = assign10530_e9706_d_n8;
        locals.var_spsub_a_dn9 = assign10530_e9706_d_n9;

        let (assign10540_e9726, assign10540_e9726_d_n4, assign10540_e9726_d_n6, assign10540_e9726_d_n7, assign10540_e9726_d_n8, assign10540_e9726_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10540_e9720: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
        let assign10540_e9721: f64 = (locals.var_spsub_temp1 - assign10540_e9720);
        let assign10540_e9722: f64 = (locals.var_gfsub2 * assign10540_e9721);
        let assign10540_e9723: f64 = (0.5 * assign10540_e9722);
        let assign10540_e9724: f64 = (1.0 - assign10540_e9723);
        (assign10540_e9724, (-(0.5 * ((locals.var_gfsub2_dn4 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn4 - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4))))))), (-(0.5 * ((locals.var_gfsub2_dn6 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn6 - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6))))))), (-(0.5 * ((locals.var_gfsub2_dn7 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn7 - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7))))))), (-(0.5 * ((locals.var_gfsub2_dn8 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn8 - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8))))))), (-(0.5 * ((locals.var_gfsub2_dn9 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn9 - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9))))))),)
    } else {
        (locals.var_spsub_b, locals.var_spsub_b_dn4, locals.var_spsub_b_dn6, locals.var_spsub_b_dn7, locals.var_spsub_b_dn8, locals.var_spsub_b_dn9,)
    }
};
        locals.var_spsub_b = assign10540_e9726;
        locals.var_spsub_b_dn4 = assign10540_e9726_d_n4;
        locals.var_spsub_b_dn6 = assign10540_e9726_d_n6;
        locals.var_spsub_b_dn7 = assign10540_e9726_d_n7;
        locals.var_spsub_b_dn8 = assign10540_e9726_d_n8;
        locals.var_spsub_b_dn9 = assign10540_e9726_d_n9;

        let (assign10550_e9750, assign10550_e9750_d_n4, assign10550_e9750_d_n6, assign10550_e9750_d_n7, assign10550_e9750_d_n8, assign10550_e9750_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10550_e9736: f64 = (2.0 * locals.var_spsub_temp);
        let assign10550_e9740: f64 = (1.0 - locals.var_spsub_temp1);
        let assign10550_e9744: f64 = (1.0 + locals.var_spsub_xi1);
        let assign10550_e9745: f64 = (locals.var_spsub_delta * assign10550_e9744);
        let assign10550_e9746: f64 = (assign10550_e9740 - assign10550_e9745);
        let assign10550_e9747: f64 = (locals.var_gfsub2 * assign10550_e9746);
        let assign10550_e9748: f64 = (assign10550_e9736 + assign10550_e9747);
        (assign10550_e9748, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn4) - ((locals.var_spsub_delta_dn4 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn4)))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn6) - ((locals.var_spsub_delta_dn6 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn6)))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn7) - ((locals.var_spsub_delta_dn7 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn7)))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn8) - ((locals.var_spsub_delta_dn8 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn8)))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn9) - ((locals.var_spsub_delta_dn9 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn9)))))),)
    } else {
        (locals.var_spsub_c, locals.var_spsub_c_dn4, locals.var_spsub_c_dn6, locals.var_spsub_c_dn7, locals.var_spsub_c_dn8, locals.var_spsub_c_dn9,)
    }
};
        locals.var_spsub_c = assign10550_e9750;
        locals.var_spsub_c_dn4 = assign10550_e9750_d_n4;
        locals.var_spsub_c_dn6 = assign10550_e9750_d_n6;
        locals.var_spsub_c_dn7 = assign10550_e9750_d_n7;
        locals.var_spsub_c_dn8 = assign10550_e9750_d_n8;
        locals.var_spsub_c_dn9 = assign10550_e9750_d_n9;

        let (assign10560_e9767, assign10560_e9767_d_n4, assign10560_e9767_d_n6, assign10560_e9767_d_n7, assign10560_e9767_d_n8, assign10560_e9767_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10560_e9760: f64 = (locals.var_xn_sub - locals.var_spsub_eta);
        let assign10560_e9763: f64 = (locals.var_spsub_a / locals.var_gfsub2);
        let assign10560_e9764: f64 = (assign10560_e9763).ln();
        let assign10560_e9765: f64 = (assign10560_e9760 + assign10560_e9764);
        (assign10560_e9765, ((locals.var_xn_sub_dn4 - locals.var_spsub_eta_dn4) + ((((locals.var_spsub_a_dn4 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn4)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), ((locals.var_xn_sub_dn6 - locals.var_spsub_eta_dn6) + ((((locals.var_spsub_a_dn6 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn6)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), ((locals.var_xn_sub_dn7 - locals.var_spsub_eta_dn7) + ((((locals.var_spsub_a_dn7 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn7)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), ((locals.var_xn_sub_dn8 - locals.var_spsub_eta_dn8) + ((((locals.var_spsub_a_dn8 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn8)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), ((locals.var_xn_sub_dn9 - locals.var_spsub_eta_dn9) + ((((locals.var_spsub_a_dn9 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn9)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)),)
    } else {
        (locals.var_spsub_tau, locals.var_spsub_tau_dn4, locals.var_spsub_tau_dn6, locals.var_spsub_tau_dn7, locals.var_spsub_tau_dn8, locals.var_spsub_tau_dn9,)
    }
};
        locals.var_spsub_tau = assign10560_e9767;
        locals.var_spsub_tau_dn4 = assign10560_e9767_d_n4;
        locals.var_spsub_tau_dn6 = assign10560_e9767_d_n6;
        locals.var_spsub_tau_dn7 = assign10560_e9767_d_n7;
        locals.var_spsub_tau_dn8 = assign10560_e9767_d_n8;
        locals.var_spsub_tau_dn9 = assign10560_e9767_d_n9;

        let (assign10570_e9779, assign10570_e9779_d_n4, assign10570_e9779_d_n6, assign10570_e9779_d_n7, assign10570_e9779_d_n8, assign10570_e9779_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10570_e9777: f64 = (locals.var_spsub_a + locals.var_spsub_c);
        (assign10570_e9777, (locals.var_spsub_a_dn4 + locals.var_spsub_c_dn4), (locals.var_spsub_a_dn6 + locals.var_spsub_c_dn6), (locals.var_spsub_a_dn7 + locals.var_spsub_c_dn7), (locals.var_spsub_a_dn8 + locals.var_spsub_c_dn8), (locals.var_spsub_a_dn9 + locals.var_spsub_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign10570_e9779;
        locals.var_nu_dn4 = assign10570_e9779_d_n4;
        locals.var_nu_dn6 = assign10570_e9779_d_n6;
        locals.var_nu_dn7 = assign10570_e9779_d_n7;
        locals.var_nu_dn8 = assign10570_e9779_d_n8;
        locals.var_nu_dn9 = assign10570_e9779_d_n9;

        let (assign10580_e9803, assign10580_e9803_d_n4, assign10580_e9803_d_n6, assign10580_e9803_d_n7, assign10580_e9803_d_n8, assign10580_e9803_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10580_e9789: f64 = (locals.var_nu * locals.var_nu);
        let assign10580_e9793: f64 = (0.5 * locals.var_spsub_c);
        let assign10580_e9795: f64 = (assign10580_e9793 * locals.var_spsub_c);
        let assign10580_e9798: f64 = (locals.var_spsub_a * locals.var_spsub_b);
        let assign10580_e9799: f64 = (assign10580_e9795 - assign10580_e9798);
        let assign10580_e9800: f64 = (locals.var_spsub_tau * assign10580_e9799);
        let assign10580_e9801: f64 = (assign10580_e9789 + assign10580_e9800);
        (assign10580_e9801, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_spsub_tau_dn4 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn4) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn4)) - ((locals.var_spsub_a_dn4 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_spsub_tau_dn6 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn6) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn6)) - ((locals.var_spsub_a_dn6 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_spsub_tau_dn7 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn7) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn7)) - ((locals.var_spsub_a_dn7 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_spsub_tau_dn8 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn8) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn8)) - ((locals.var_spsub_a_dn8 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_spsub_tau_dn9 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn9) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn9)) - ((locals.var_spsub_a_dn9 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign10580_e9803;
        locals.var_mutau_dn4 = assign10580_e9803_d_n4;
        locals.var_mutau_dn6 = assign10580_e9803_d_n6;
        locals.var_mutau_dn7 = assign10580_e9803_d_n7;
        locals.var_mutau_dn8 = assign10580_e9803_d_n8;
        locals.var_mutau_dn9 = assign10580_e9803_d_n9;

    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10590_e9841, assign10590_e9841_d_n4, assign10590_e9841_d_n6, assign10590_e9841_d_n7, assign10590_e9841_d_n8, assign10590_e9841_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10590_e9814: f64 = (locals.var_spsub_a * locals.var_nu);
        let assign10590_e9816: f64 = (assign10590_e9814 * locals.var_spsub_tau);
        let assign10590_e9820: f64 = (locals.var_nu / locals.var_mutau);
        let assign10590_e9822: f64 = (assign10590_e9820 * locals.var_spsub_tau);
        let assign10590_e9824: f64 = (assign10590_e9822 * locals.var_spsub_tau);
        let assign10590_e9826: f64 = (assign10590_e9824 * locals.var_spsub_c);
        let assign10590_e9829: f64 = (locals.var_spsub_c * locals.var_spsub_c);
        let assign10590_e9831: f64 = (assign10590_e9829 * 0.3333333333333);
        let assign10590_e9834: f64 = (locals.var_spsub_a * locals.var_spsub_b);
        let assign10590_e9835: f64 = (assign10590_e9831 - assign10590_e9834);
        let assign10590_e9836: f64 = (assign10590_e9826 * assign10590_e9835);
        let assign10590_e9837: f64 = (locals.var_mutau + assign10590_e9836);
        let assign10590_e9838: f64 = (assign10590_e9816 / assign10590_e9837);
        let assign10590_e9839: f64 = (locals.var_spsub_eta + assign10590_e9838);
        (assign10590_e9839, (locals.var_spsub_eta_dn4 + (((((((locals.var_spsub_a_dn4 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn4)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn4)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn4)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn4)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn4)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn4 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn4)) * 0.3333333333333) - ((locals.var_spsub_a_dn4 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn4)))))))) / (assign10590_e9837 * assign10590_e9837))), (locals.var_spsub_eta_dn6 + (((((((locals.var_spsub_a_dn6 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn6)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn6)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn6)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn6)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn6)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn6 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn6)) * 0.3333333333333) - ((locals.var_spsub_a_dn6 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn6)))))))) / (assign10590_e9837 * assign10590_e9837))), (locals.var_spsub_eta_dn7 + (((((((locals.var_spsub_a_dn7 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn7)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn7)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn7)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn7)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn7)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn7 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn7)) * 0.3333333333333) - ((locals.var_spsub_a_dn7 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn7)))))))) / (assign10590_e9837 * assign10590_e9837))), (locals.var_spsub_eta_dn8 + (((((((locals.var_spsub_a_dn8 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn8)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn8)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn8)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn8)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn8)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn8 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn8)) * 0.3333333333333) - ((locals.var_spsub_a_dn8 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn8)))))))) / (assign10590_e9837 * assign10590_e9837))), (locals.var_spsub_eta_dn9 + (((((((locals.var_spsub_a_dn9 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn9)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn9)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn9)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn9)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn9)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn9 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn9)) * 0.3333333333333) - ((locals.var_spsub_a_dn9 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn9)))))))) / (assign10590_e9837 * assign10590_e9837))),)
    } else {
        (locals.var_spsub_x0, locals.var_spsub_x0_dn4, locals.var_spsub_x0_dn6, locals.var_spsub_x0_dn7, locals.var_spsub_x0_dn8, locals.var_spsub_x0_dn9,)
    }
};
        locals.var_spsub_x0 = assign10590_e9841;
        locals.var_spsub_x0_dn4 = assign10590_e9841_d_n4;
        locals.var_spsub_x0_dn6 = assign10590_e9841_d_n6;
        locals.var_spsub_x0_dn7 = assign10590_e9841_d_n7;
        locals.var_spsub_x0_dn8 = assign10590_e9841_d_n8;
        locals.var_spsub_x0_dn9 = assign10590_e9841_d_n9;

        let assign10600_e9844: f64 = if locals.var_spsub_x0 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign10600_e9844;

        let (assign10610_e9857, assign10610_e9857_d_n4, assign10610_e9857_d_n6, assign10610_e9857_d_n7, assign10610_e9857_d_n8, assign10610_e9857_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
        let assign10610_e9855: f64 = (locals.var_spsub_x0).exp();
        (assign10610_e9855, (assign10610_e9855 * locals.var_spsub_x0_dn4), (assign10610_e9855 * locals.var_spsub_x0_dn6), (assign10610_e9855 * locals.var_spsub_x0_dn7), (assign10610_e9855 * locals.var_spsub_x0_dn8), (assign10610_e9855 * locals.var_spsub_x0_dn9),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10610_e9857;
        locals.var_spsub_delta0_dn4 = assign10610_e9857_d_n4;
        locals.var_spsub_delta0_dn6 = assign10610_e9857_d_n6;
        locals.var_spsub_delta0_dn7 = assign10610_e9857_d_n7;
        locals.var_spsub_delta0_dn8 = assign10610_e9857_d_n8;
        locals.var_spsub_delta0_dn9 = assign10610_e9857_d_n9;

        let (assign10620_e9871, assign10620_e9871_d_n4, assign10620_e9871_d_n6, assign10620_e9871_d_n7, assign10620_e9871_d_n8, assign10620_e9871_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
        let assign10620_e9869: f64 = (1.0 / locals.var_spsub_delta0);
        (assign10620_e9869, (-(locals.var_spsub_delta0_dn4 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn6 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn7 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn8 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn9 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))),)
    } else {
        (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9,)
    }
};
        locals.var_spsub_delta1 = assign10620_e9871;
        locals.var_spsub_delta1_dn4 = assign10620_e9871_d_n4;
        locals.var_spsub_delta1_dn6 = assign10620_e9871_d_n6;
        locals.var_spsub_delta1_dn7 = assign10620_e9871_d_n7;
        locals.var_spsub_delta1_dn8 = assign10620_e9871_d_n8;
        locals.var_spsub_delta1_dn9 = assign10620_e9871_d_n9;

        let (assign10630_e9885, assign10630_e9885_d_n4, assign10630_e9885_d_n6, assign10630_e9885_d_n7, assign10630_e9885_d_n8, assign10630_e9885_d_n9,) = {
    if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
        let assign10630_e9883: f64 = (locals.var_spsub_delta * locals.var_spsub_delta0);
        (assign10630_e9883, ((locals.var_spsub_delta_dn4 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn4)), ((locals.var_spsub_delta_dn6 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn6)), ((locals.var_spsub_delta_dn7 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn7)), ((locals.var_spsub_delta_dn8 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn8)), ((locals.var_spsub_delta_dn9 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn9)),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10630_e9885;
        locals.var_spsub_delta0_dn4 = assign10630_e9885_d_n4;
        locals.var_spsub_delta0_dn6 = assign10630_e9885_d_n6;
        locals.var_spsub_delta0_dn7 = assign10630_e9885_d_n7;
        locals.var_spsub_delta0_dn8 = assign10630_e9885_d_n8;
        locals.var_spsub_delta0_dn9 = assign10630_e9885_d_n9;

        let assign10640_e9889: f64 = (locals.var_xn_sub - 80.0);
        let assign10640_e9890: f64 = if locals.var_spsub_x0 > assign10640_e9889 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign10640_e9890;

        let (assign10650_e9908, assign10650_e9908_d_n4, assign10650_e9908_d_n6, assign10650_e9908_d_n7, assign10650_e9908_d_n8, assign10650_e9908_d_n9,) = {
    if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 != 0.0)) {
        let assign10650_e9905: f64 = (locals.var_spsub_x0 - locals.var_xn_sub);
        let assign10650_e9906: f64 = (assign10650_e9905).exp();
        (assign10650_e9906, (assign10650_e9906 * (locals.var_spsub_x0_dn4 - locals.var_xn_sub_dn4)), (assign10650_e9906 * (locals.var_spsub_x0_dn6 - locals.var_xn_sub_dn6)), (assign10650_e9906 * (locals.var_spsub_x0_dn7 - locals.var_xn_sub_dn7)), (assign10650_e9906 * (locals.var_spsub_x0_dn8 - locals.var_xn_sub_dn8)), (assign10650_e9906 * (locals.var_spsub_x0_dn9 - locals.var_xn_sub_dn9)),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10650_e9908;
        locals.var_spsub_delta0_dn4 = assign10650_e9908_d_n4;
        locals.var_spsub_delta0_dn6 = assign10650_e9908_d_n6;
        locals.var_spsub_delta0_dn7 = assign10650_e9908_d_n7;
        locals.var_spsub_delta0_dn8 = assign10650_e9908_d_n8;
        locals.var_spsub_delta0_dn9 = assign10650_e9908_d_n9;

        let (assign10660_e9925, assign10660_e9925_d_n4, assign10660_e9925_d_n6, assign10660_e9925_d_n7, assign10660_e9925_d_n8, assign10660_e9925_d_n9,) = {
    if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 != 0.0)) {
        let assign10660_e9923: f64 = (locals.var_spsub_delta / locals.var_spsub_delta0);
        (assign10660_e9923, (((locals.var_spsub_delta_dn4 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn4)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn6 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn6)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn7 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn7)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn8 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn8)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn9 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn9)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)),)
    } else {
        (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9,)
    }
};
        locals.var_spsub_delta1 = assign10660_e9925;
        locals.var_spsub_delta1_dn4 = assign10660_e9925_d_n4;
        locals.var_spsub_delta1_dn6 = assign10660_e9925_d_n6;
        locals.var_spsub_delta1_dn7 = assign10660_e9925_d_n7;
        locals.var_spsub_delta1_dn8 = assign10660_e9925_d_n8;
        locals.var_spsub_delta1_dn9 = assign10660_e9925_d_n9;

        let (assign10670_e9969, assign10670_e9969_d_n4, assign10670_e9969_d_n6, assign10670_e9969_d_n7, assign10670_e9969_d_n8, assign10670_e9969_d_n9,) = {
    if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 == 0.0)) {
        let assign10670_e9943: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
        let assign10670_e9945: f64 = (assign10670_e9943 - 80.0);
        let assign10670_e9950: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
        let assign10670_e9952: f64 = (assign10670_e9950 - 80.0);
        let assign10670_e9953: f64 = (0.5 * assign10670_e9952);
        let assign10670_e9957: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
        let assign10670_e9959: f64 = (assign10670_e9957 - 80.0);
        let assign10670_e9961: f64 = (assign10670_e9959 * 0.3333333333333);
        let assign10670_e9962: f64 = (1.0 + assign10670_e9961);
        let assign10670_e9963: f64 = (assign10670_e9953 * assign10670_e9962);
        let assign10670_e9964: f64 = (1.0 + assign10670_e9963);
        let assign10670_e9965: f64 = (assign10670_e9945 * assign10670_e9964);
        let assign10670_e9966: f64 = (1.0 + assign10670_e9965);
        let assign10670_e9967: f64 = (1.80485e-35 / assign10670_e9966);
        (assign10670_e9967, (-((1.80485e-35 * (((locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), (-((1.80485e-35 * (((locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), (-((1.80485e-35 * (((locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), (-((1.80485e-35 * (((locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), (-((1.80485e-35 * (((locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))),)
    } else {
        (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9,)
    }
};
        locals.var_spsub_delta0 = assign10670_e9969;
        locals.var_spsub_delta0_dn4 = assign10670_e9969_d_n4;
        locals.var_spsub_delta0_dn6 = assign10670_e9969_d_n6;
        locals.var_spsub_delta0_dn7 = assign10670_e9969_d_n7;
        locals.var_spsub_delta0_dn8 = assign10670_e9969_d_n8;
        locals.var_spsub_delta0_dn9 = assign10670_e9969_d_n9;

        let (assign10680_e10007, assign10680_e10007_d_n4, assign10680_e10007_d_n6, assign10680_e10007_d_n7, assign10680_e10007_d_n8, assign10680_e10007_d_n9,) = {
    if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 == 0.0)) {
        let assign10680_e9987: f64 = (locals.var_spsub_x0 - 80.0);
        let assign10680_e9992: f64 = (locals.var_spsub_x0 - 80.0);
        let assign10680_e9993: f64 = (0.5 * assign10680_e9992);
        let assign10680_e9997: f64 = (locals.var_spsub_x0 - 80.0);
        let assign10680_e9999: f64 = (assign10680_e9997 * 0.3333333333333);
        let assign10680_e10000: f64 = (1.0 + assign10680_e9999);
        let assign10680_e10001: f64 = (assign10680_e9993 * assign10680_e10000);
        let assign10680_e10002: f64 = (1.0 + assign10680_e10001);
        let assign10680_e10003: f64 = (assign10680_e9987 * assign10680_e10002);
        let assign10680_e10004: f64 = (1.0 + assign10680_e10003);
        let assign10680_e10005: f64 = (1.80485e-35 / assign10680_e10004);
        (assign10680_e10005, (-((1.80485e-35 * ((locals.var_spsub_x0_dn4 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn4) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn4 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn6 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn6) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn6 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn7 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn7) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn7 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn8 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn8) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn8 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn9 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn9) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn9 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))),)
    } else {
        (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9,)
    }
};
        locals.var_spsub_delta1 = assign10680_e10007;
        locals.var_spsub_delta1_dn4 = assign10680_e10007_d_n4;
        locals.var_spsub_delta1_dn6 = assign10680_e10007_d_n6;
        locals.var_spsub_delta1_dn7 = assign10680_e10007_d_n7;
        locals.var_spsub_delta1_dn8 = assign10680_e10007_d_n8;
        locals.var_spsub_delta1_dn9 = assign10680_e10007_d_n9;

        let (assign10690_e10023, assign10690_e10023_d_n4, assign10690_e10023_d_n6, assign10690_e10023_d_n7, assign10690_e10023_d_n8, assign10690_e10023_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10690_e10019: f64 = (locals.var_spsub_x0 * locals.var_spsub_x0);
        let assign10690_e10020: f64 = (2.0 + assign10690_e10019);
        let assign10690_e10021: f64 = (1.0 / assign10690_e10020);
        (assign10690_e10021, (-(((locals.var_spsub_x0_dn4 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn4)) / (assign10690_e10020 * assign10690_e10020))), (-(((locals.var_spsub_x0_dn6 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn6)) / (assign10690_e10020 * assign10690_e10020))), (-(((locals.var_spsub_x0_dn7 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn7)) / (assign10690_e10020 * assign10690_e10020))), (-(((locals.var_spsub_x0_dn8 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn8)) / (assign10690_e10020 * assign10690_e10020))), (-(((locals.var_spsub_x0_dn9 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn9)) / (assign10690_e10020 * assign10690_e10020))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10690_e10023;
        locals.var_spsub_temp_dn4 = assign10690_e10023_d_n4;
        locals.var_spsub_temp_dn6 = assign10690_e10023_d_n6;
        locals.var_spsub_temp_dn7 = assign10690_e10023_d_n7;
        locals.var_spsub_temp_dn8 = assign10690_e10023_d_n8;
        locals.var_spsub_temp_dn9 = assign10690_e10023_d_n9;

        let (assign10700_e10037, assign10700_e10037_d_n4, assign10700_e10037_d_n6, assign10700_e10037_d_n7, assign10700_e10037_d_n8, assign10700_e10037_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10700_e10033: f64 = (locals.var_spsub_x0 * locals.var_spsub_x0);
        let assign10700_e10035: f64 = (assign10700_e10033 * locals.var_spsub_temp);
        (assign10700_e10035, ((((locals.var_spsub_x0_dn4 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn4)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn4)), ((((locals.var_spsub_x0_dn6 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn6)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn6)), ((((locals.var_spsub_x0_dn7 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn7)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn7)), ((((locals.var_spsub_x0_dn8 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn8)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn8)), ((((locals.var_spsub_x0_dn9 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn9)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn9)),)
    } else {
        (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9,)
    }
};
        locals.var_spsub_xi0 = assign10700_e10037;
        locals.var_spsub_xi0_dn4 = assign10700_e10037_d_n4;
        locals.var_spsub_xi0_dn6 = assign10700_e10037_d_n6;
        locals.var_spsub_xi0_dn7 = assign10700_e10037_d_n7;
        locals.var_spsub_xi0_dn8 = assign10700_e10037_d_n8;
        locals.var_spsub_xi0_dn9 = assign10700_e10037_d_n9;

        let (assign10710_e10053, assign10710_e10053_d_n4, assign10710_e10053_d_n6, assign10710_e10053_d_n7, assign10710_e10053_d_n8, assign10710_e10053_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10710_e10048: f64 = (locals.var_spsub_x0 * locals.var_spsub_temp);
        let assign10710_e10050: f64 = (assign10710_e10048 * locals.var_spsub_temp);
        let assign10710_e10051: f64 = (4.0 * assign10710_e10050);
        (assign10710_e10051, (4.0 * ((((locals.var_spsub_x0_dn4 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn4))), (4.0 * ((((locals.var_spsub_x0_dn6 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn6))), (4.0 * ((((locals.var_spsub_x0_dn7 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn7))), (4.0 * ((((locals.var_spsub_x0_dn8 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn8))), (4.0 * ((((locals.var_spsub_x0_dn9 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn9))),)
    } else {
        (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9,)
    }
};
        locals.var_spsub_xi1 = assign10710_e10053;
        locals.var_spsub_xi1_dn4 = assign10710_e10053_d_n4;
        locals.var_spsub_xi1_dn6 = assign10710_e10053_d_n6;
        locals.var_spsub_xi1_dn7 = assign10710_e10053_d_n7;
        locals.var_spsub_xi1_dn8 = assign10710_e10053_d_n8;
        locals.var_spsub_xi1_dn9 = assign10710_e10053_d_n9;

        let (assign10720_e10073, assign10720_e10073_d_n4, assign10720_e10073_d_n6, assign10720_e10073_d_n7, assign10720_e10073_d_n8, assign10720_e10073_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10720_e10063: f64 = (8.0 * locals.var_spsub_temp);
        let assign10720_e10066: f64 = (12.0 * locals.var_spsub_xi0);
        let assign10720_e10067: f64 = (assign10720_e10063 - assign10720_e10066);
        let assign10720_e10069: f64 = (assign10720_e10067 * locals.var_spsub_temp);
        let assign10720_e10071: f64 = (assign10720_e10069 * locals.var_spsub_temp);
        (assign10720_e10071, ((((((8.0 * locals.var_spsub_temp_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn4)), ((((((8.0 * locals.var_spsub_temp_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn6)), ((((((8.0 * locals.var_spsub_temp_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn7)), ((((((8.0 * locals.var_spsub_temp_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn8)), ((((((8.0 * locals.var_spsub_temp_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn9)),)
    } else {
        (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9,)
    }
};
        locals.var_spsub_xi2 = assign10720_e10073;
        locals.var_spsub_xi2_dn4 = assign10720_e10073_d_n4;
        locals.var_spsub_xi2_dn6 = assign10720_e10073_d_n6;
        locals.var_spsub_xi2_dn7 = assign10720_e10073_d_n7;
        locals.var_spsub_xi2_dn8 = assign10720_e10073_d_n8;
        locals.var_spsub_xi2_dn9 = assign10720_e10073_d_n9;

        let (assign10730_e10085, assign10730_e10085_d_n4, assign10730_e10085_d_n6, assign10730_e10085_d_n7, assign10730_e10085_d_n8, assign10730_e10085_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10730_e10083: f64 = (locals.var_spsub_xgb - locals.var_spsub_x0);
        (assign10730_e10083, (locals.var_spsub_xgb_dn4 - locals.var_spsub_x0_dn4), (locals.var_spsub_xgb_dn6 - locals.var_spsub_x0_dn6), (locals.var_spsub_xgb_dn7 - locals.var_spsub_x0_dn7), (locals.var_spsub_xgb_dn8 - locals.var_spsub_x0_dn8), (locals.var_spsub_xgb_dn9 - locals.var_spsub_x0_dn9),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10730_e10085;
        locals.var_spsub_temp_dn4 = assign10730_e10085_d_n4;
        locals.var_spsub_temp_dn6 = assign10730_e10085_d_n6;
        locals.var_spsub_temp_dn7 = assign10730_e10085_d_n7;
        locals.var_spsub_temp_dn8 = assign10730_e10085_d_n8;
        locals.var_spsub_temp_dn9 = assign10730_e10085_d_n9;

        let (assign10740_e10111, assign10740_e10111_d_n4, assign10740_e10111_d_n6, assign10740_e10111_d_n7, assign10740_e10111_d_n8, assign10740_e10111_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10740_e10095: f64 = (2.0 * locals.var_spsub_temp);
        let assign10740_e10099: f64 = (1.0 - locals.var_spsub_delta1);
        let assign10740_e10101: f64 = (assign10740_e10099 + locals.var_spsub_delta0);
        let assign10740_e10105: f64 = (1.0 + locals.var_spsub_xi1);
        let assign10740_e10106: f64 = (locals.var_spsub_delta * assign10740_e10105);
        let assign10740_e10107: f64 = (assign10740_e10101 - assign10740_e10106);
        let assign10740_e10108: f64 = (locals.var_gfsub2 * assign10740_e10107);
        let assign10740_e10109: f64 = (assign10740_e10095 + assign10740_e10108);
        (assign10740_e10109, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn4) + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn4)))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn6) + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn6)))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn7) + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn7)))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn8) + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn8)))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn9) + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn9)))))),)
    } else {
        (locals.var_spsub_pc, locals.var_spsub_pc_dn4, locals.var_spsub_pc_dn6, locals.var_spsub_pc_dn7, locals.var_spsub_pc_dn8, locals.var_spsub_pc_dn9,)
    }
};
        locals.var_spsub_pc = assign10740_e10111;
        locals.var_spsub_pc_dn4 = assign10740_e10111_d_n4;
        locals.var_spsub_pc_dn6 = assign10740_e10111_d_n6;
        locals.var_spsub_pc_dn7 = assign10740_e10111_d_n7;
        locals.var_spsub_pc_dn8 = assign10740_e10111_d_n8;
        locals.var_spsub_pc_dn9 = assign10740_e10111_d_n9;

        let (assign10750_e10141, assign10750_e10141_d_n4, assign10750_e10141_d_n6, assign10750_e10141_d_n7, assign10750_e10141_d_n8, assign10750_e10141_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10750_e10121: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
        let assign10750_e10125: f64 = (locals.var_spsub_delta1 + locals.var_spsub_x0);
        let assign10750_e10127: f64 = (assign10750_e10125 - 1.0);
        let assign10750_e10129: f64 = (assign10750_e10127 + locals.var_spsub_delta0);
        let assign10750_e10133: f64 = (locals.var_spsub_x0 + 1.0);
        let assign10750_e10135: f64 = (assign10750_e10133 + locals.var_spsub_xi0);
        let assign10750_e10136: f64 = (locals.var_spsub_delta * assign10750_e10135);
        let assign10750_e10137: f64 = (assign10750_e10129 - assign10750_e10136);
        let assign10750_e10138: f64 = (locals.var_gfsub2 * assign10750_e10137);
        let assign10750_e10139: f64 = (assign10750_e10121 - assign10750_e10138);
        (assign10750_e10139, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn4 + locals.var_spsub_x0_dn4) + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn4 + locals.var_spsub_xi0_dn4))))))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn6 + locals.var_spsub_x0_dn6) + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn6 + locals.var_spsub_xi0_dn6))))))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn7 + locals.var_spsub_x0_dn7) + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn7 + locals.var_spsub_xi0_dn7))))))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn8 + locals.var_spsub_x0_dn8) + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn8 + locals.var_spsub_xi0_dn8))))))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn9 + locals.var_spsub_x0_dn9) + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn9 + locals.var_spsub_xi0_dn9))))))),)
    } else {
        (locals.var_spsub_qc, locals.var_spsub_qc_dn4, locals.var_spsub_qc_dn6, locals.var_spsub_qc_dn7, locals.var_spsub_qc_dn8, locals.var_spsub_qc_dn9,)
    }
};
        locals.var_spsub_qc = assign10750_e10141;
        locals.var_spsub_qc_dn4 = assign10750_e10141_d_n4;
        locals.var_spsub_qc_dn6 = assign10750_e10141_d_n6;
        locals.var_spsub_qc_dn7 = assign10750_e10141_d_n7;
        locals.var_spsub_qc_dn8 = assign10750_e10141_d_n8;
        locals.var_spsub_qc_dn9 = assign10750_e10141_d_n9;

        let (assign10760_e10161, assign10760_e10161_d_n4, assign10760_e10161_d_n6, assign10760_e10161_d_n7, assign10760_e10161_d_n8, assign10760_e10161_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10760_e10153: f64 = (locals.var_spsub_delta1 + locals.var_spsub_delta0);
        let assign10760_e10156: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
        let assign10760_e10157: f64 = (assign10760_e10153 - assign10760_e10156);
        let assign10760_e10158: f64 = (locals.var_gfsub2 * assign10760_e10157);
        let assign10760_e10159: f64 = (2.0 - assign10760_e10158);
        (assign10760_e10159, (-((locals.var_gfsub2_dn4 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn4 + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4)))))), (-((locals.var_gfsub2_dn6 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn6 + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6)))))), (-((locals.var_gfsub2_dn7 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn7 + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7)))))), (-((locals.var_gfsub2_dn8 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn8 + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8)))))), (-((locals.var_gfsub2_dn9 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn9 + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9)))))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10760_e10161;
        locals.var_spsub_temp_dn4 = assign10760_e10161_d_n4;
        locals.var_spsub_temp_dn6 = assign10760_e10161_d_n6;
        locals.var_spsub_temp_dn7 = assign10760_e10161_d_n7;
        locals.var_spsub_temp_dn8 = assign10760_e10161_d_n8;
        locals.var_spsub_temp_dn9 = assign10760_e10161_d_n9;

        let (assign10770_e10179, assign10770_e10179_d_n4, assign10770_e10179_d_n6, assign10770_e10179_d_n7, assign10770_e10179_d_n8, assign10770_e10179_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10770_e10171: f64 = (locals.var_spsub_pc * locals.var_spsub_pc);
        let assign10770_e10175: f64 = (locals.var_spsub_qc * locals.var_spsub_temp);
        let assign10770_e10176: f64 = (2.0 * assign10770_e10175);
        let assign10770_e10177: f64 = (assign10770_e10171 - assign10770_e10176);
        (assign10770_e10177, (((locals.var_spsub_pc_dn4 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn4)) - (2.0 * ((locals.var_spsub_qc_dn4 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn4)))), (((locals.var_spsub_pc_dn6 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn6)) - (2.0 * ((locals.var_spsub_qc_dn6 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn6)))), (((locals.var_spsub_pc_dn7 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn7)) - (2.0 * ((locals.var_spsub_qc_dn7 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn7)))), (((locals.var_spsub_pc_dn8 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn8)) - (2.0 * ((locals.var_spsub_qc_dn8 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn8)))), (((locals.var_spsub_pc_dn9 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn9)) - (2.0 * ((locals.var_spsub_qc_dn9 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn9)))),)
    } else {
        (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9,)
    }
};
        locals.var_spsub_temp = assign10770_e10179;
        locals.var_spsub_temp_dn4 = assign10770_e10179_d_n4;
        locals.var_spsub_temp_dn6 = assign10770_e10179_d_n6;
        locals.var_spsub_temp_dn7 = assign10770_e10179_d_n7;
        locals.var_spsub_temp_dn8 = assign10770_e10179_d_n8;
        locals.var_spsub_temp_dn9 = assign10770_e10179_d_n9;

        let (assign10780_e10198, assign10780_e10198_d_n4, assign10780_e10198_d_n6, assign10780_e10198_d_n7, assign10780_e10198_d_n8, assign10780_e10198_d_n9,) = {
    if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
        let assign10780_e10192: f64 = (locals.var_spsub_temp).sqrt();
        let assign10780_e10193: f64 = (locals.var_spsub_pc + assign10780_e10192);
        let assign10780_e10194: f64 = (locals.var_spsub_qc / assign10780_e10193);
        let assign10780_e10195: f64 = (2.0 * assign10780_e10194);
        let assign10780_e10196: f64 = (locals.var_spsub_x0 + assign10780_e10195);
        (assign10780_e10196, (locals.var_spsub_x0_dn4 + (2.0 * (((locals.var_spsub_qc_dn4 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn4 + (locals.var_spsub_temp_dn4 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), (locals.var_spsub_x0_dn6 + (2.0 * (((locals.var_spsub_qc_dn6 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn6 + (locals.var_spsub_temp_dn6 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), (locals.var_spsub_x0_dn7 + (2.0 * (((locals.var_spsub_qc_dn7 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn7 + (locals.var_spsub_temp_dn7 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), (locals.var_spsub_x0_dn8 + (2.0 * (((locals.var_spsub_qc_dn8 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn8 + (locals.var_spsub_temp_dn8 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), (locals.var_spsub_x0_dn9 + (2.0 * (((locals.var_spsub_qc_dn9 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn9 + (locals.var_spsub_temp_dn9 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign10780_e10198;
        locals.var_temp3_dn4 = assign10780_e10198_d_n4;
        locals.var_temp3_dn6 = assign10780_e10198_d_n6;
        locals.var_temp3_dn7 = assign10780_e10198_d_n7;
        locals.var_temp3_dn8 = assign10780_e10198_d_n8;
        locals.var_temp3_dn9 = assign10780_e10198_d_n9;

        let (assign10790_e10206, assign10790_e10206_d_n4, assign10790_e10206_d_n6, assign10790_e10206_d_n7, assign10790_e10206_d_n8, assign10790_e10206_d_n9,) = {
    if (locals.var_guard531 != 0.0) {
        let assign10790_e10203: f64 = (locals.var_temp3 + locals.var_temp2);
        let assign10790_e10204: f64 = (locals.var_temp * assign10790_e10203);
        (assign10790_e10204, ((locals.var_temp_dn4 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn4 + locals.var_temp2_dn4))), ((locals.var_temp_dn6 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn6 + locals.var_temp2_dn6))), ((locals.var_temp_dn7 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn7 + locals.var_temp2_dn7))), ((locals.var_temp_dn8 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn8 + locals.var_temp2_dn8))), ((locals.var_temp_dn9 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn9 + locals.var_temp2_dn9))),)
    } else {
        (locals.var_xg2eff, locals.var_xg2eff_dn4, locals.var_xg2eff_dn6, locals.var_xg2eff_dn7, locals.var_xg2eff_dn8, locals.var_xg2eff_dn9,)
    }
};
        locals.var_xg2eff = assign10790_e10206;
        locals.var_xg2eff_dn4 = assign10790_e10206_d_n4;
        locals.var_xg2eff_dn6 = assign10790_e10206_d_n6;
        locals.var_xg2eff_dn7 = assign10790_e10206_d_n7;
        locals.var_xg2eff_dn8 = assign10790_e10206_d_n8;
        locals.var_xg2eff_dn9 = assign10790_e10206_d_n9;

        let (assign10800_e10211, assign10800_e10211_d_n4, assign10800_e10211_d_n6, assign10800_e10211_d_n7, assign10800_e10211_d_n8, assign10800_e10211_d_n9,) = {
    if (locals.var_guard531 == 0.0) {
        (locals.var_xg20, locals.var_xg20_dn4, locals.var_xg20_dn6, locals.var_xg20_dn7, locals.var_xg20_dn8, locals.var_xg20_dn9,)
    } else {
        (locals.var_xg2eff, locals.var_xg2eff_dn4, locals.var_xg2eff_dn6, locals.var_xg2eff_dn7, locals.var_xg2eff_dn8, locals.var_xg2eff_dn9,)
    }
};
        locals.var_xg2eff = assign10800_e10211;
        locals.var_xg2eff_dn4 = assign10800_e10211_d_n4;
        locals.var_xg2eff_dn6 = assign10800_e10211_d_n6;
        locals.var_xg2eff_dn7 = assign10800_e10211_d_n7;
        locals.var_xg2eff_dn8 = assign10800_e10211_d_n8;
        locals.var_xg2eff_dn9 = assign10800_e10211_d_n9;

        let assign10810_e10215: f64 = (locals.var_xg10 - locals.var_xg2eff);
        let assign10810_e10216: f64 = (locals.var_keq_1d * assign10810_e10215);
        locals.var_temp = assign10810_e10216;
        locals.var_temp_dn4 = (locals.var_keq_1d * (locals.var_xg10_dn4 - locals.var_xg2eff_dn4));
        locals.var_temp_dn6 = (locals.var_keq_1d * (locals.var_xg10_dn6 - locals.var_xg2eff_dn6));
        locals.var_temp_dn7 = (locals.var_keq_1d * (locals.var_xg10_dn7 - locals.var_xg2eff_dn7));
        locals.var_temp_dn8 = (locals.var_keq_1d * (locals.var_xg10_dn8 - locals.var_xg2eff_dn8));
        locals.var_temp_dn9 = (locals.var_keq_1d * (locals.var_xg10_dn9 - locals.var_xg2eff_dn9));

        let assign10820_e10219: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign10820_e10219;

        let (assign10830_e10240, assign10830_e10240_d_n4, assign10830_e10240_d_n6, assign10830_e10240_d_n7, assign10830_e10240_d_n8, assign10830_e10240_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10830_e10224: f64 = (locals.var_temp + locals.var_emin);
        let assign10830_e10227: f64 = (locals.var_temp - locals.var_emin);
        let assign10830_e10230: f64 = (locals.var_temp - locals.var_emin);
        let assign10830_e10231: f64 = (assign10830_e10227 * assign10830_e10230);
        let assign10830_e10234: f64 = (locals.var_emin * locals.var_emin);
        let assign10830_e10235: f64 = (assign10830_e10231 + assign10830_e10234);
        let assign10830_e10236: f64 = (assign10830_e10235).sqrt();
        let assign10830_e10237: f64 = (assign10830_e10224 + assign10830_e10236);
        let assign10830_e10238: f64 = (0.5 * assign10830_e10237);
        (assign10830_e10238, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign10830_e10236)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign10830_e10236)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign10830_e10236)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign10830_e10236)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign10830_e10236)))),)
    } else {
        (locals.var_e1, locals.var_e1_dn4, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9,)
    }
};
        locals.var_e1 = assign10830_e10240;
        locals.var_e1_dn4 = assign10830_e10240_d_n4;
        locals.var_e1_dn6 = assign10830_e10240_d_n6;
        locals.var_e1_dn7 = assign10830_e10240_d_n7;
        locals.var_e1_dn8 = assign10830_e10240_d_n8;
        locals.var_e1_dn9 = assign10830_e10240_d_n9;

        let (assign10840_e10264, assign10840_e10264_d_n4, assign10840_e10264_d_n6, assign10840_e10264_d_n7, assign10840_e10264_d_n8, assign10840_e10264_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10840_e10244: f64 = (-locals.var_temp);
        let assign10840_e10246: f64 = (assign10840_e10244 + locals.var_emin);
        let assign10840_e10248: f64 = (-locals.var_temp);
        let assign10840_e10250: f64 = (assign10840_e10248 - locals.var_emin);
        let assign10840_e10252: f64 = (-locals.var_temp);
        let assign10840_e10254: f64 = (assign10840_e10252 - locals.var_emin);
        let assign10840_e10255: f64 = (assign10840_e10250 * assign10840_e10254);
        let assign10840_e10258: f64 = (locals.var_emin * locals.var_emin);
        let assign10840_e10259: f64 = (assign10840_e10255 + assign10840_e10258);
        let assign10840_e10260: f64 = (assign10840_e10259).sqrt();
        let assign10840_e10261: f64 = (assign10840_e10246 + assign10840_e10260);
        let assign10840_e10262: f64 = (0.5 * assign10840_e10261);
        (assign10840_e10262, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign10840_e10260)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign10840_e10260)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign10840_e10260)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign10840_e10260)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign10840_e10260)))),)
    } else {
        (locals.var_e2, locals.var_e2_dn4, locals.var_e2_dn6, locals.var_e2_dn7, locals.var_e2_dn8, locals.var_e2_dn9,)
    }
};
        locals.var_e2 = assign10840_e10264;
        locals.var_e2_dn4 = assign10840_e10264_d_n4;
        locals.var_e2_dn6 = assign10840_e10264_d_n6;
        locals.var_e2_dn7 = assign10840_e10264_d_n7;
        locals.var_e2_dn8 = assign10840_e10264_d_n8;
        locals.var_e2_dn9 = assign10840_e10264_d_n9;

        let (assign10850_e10275, assign10850_e10275_d_n4, assign10850_e10275_d_n6, assign10850_e10275_d_n7, assign10850_e10275_d_n8, assign10850_e10275_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10850_e10268: f64 = (-0.3333333333333);
        let assign10850_e10270: f64 = (locals.var_e1).ln();
        let assign10850_e10271: f64 = (assign10850_e10268 * assign10850_e10270);
        let assign10850_e10272: f64 = (assign10850_e10271).exp();
        let assign10850_e10273: f64 = (locals.var_qq * assign10850_e10272);
        (assign10850_e10273, ((locals.var_qq_dn4 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn4 / locals.var_e1))))), ((locals.var_qq_dn6 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn6 / locals.var_e1))))), ((locals.var_qq_dn7 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn7 / locals.var_e1))))), ((locals.var_qq_dn8 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn8 / locals.var_e1))))), ((locals.var_qq_dn9 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn9 / locals.var_e1))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign10850_e10275;
        locals.var_temp1_dn4 = assign10850_e10275_d_n4;
        locals.var_temp1_dn6 = assign10850_e10275_d_n6;
        locals.var_temp1_dn7 = assign10850_e10275_d_n7;
        locals.var_temp1_dn8 = assign10850_e10275_d_n8;
        locals.var_temp1_dn9 = assign10850_e10275_d_n9;

        let (assign10860_e10286, assign10860_e10286_d_n4, assign10860_e10286_d_n6, assign10860_e10286_d_n7, assign10860_e10286_d_n8, assign10860_e10286_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10860_e10279: f64 = (-0.3333333333333);
        let assign10860_e10281: f64 = (locals.var_e2).ln();
        let assign10860_e10282: f64 = (assign10860_e10279 * assign10860_e10281);
        let assign10860_e10283: f64 = (assign10860_e10282).exp();
        let assign10860_e10284: f64 = (locals.var_qq * assign10860_e10283);
        (assign10860_e10284, ((locals.var_qq_dn4 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn4 / locals.var_e2))))), ((locals.var_qq_dn6 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn6 / locals.var_e2))))), ((locals.var_qq_dn7 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn7 / locals.var_e2))))), ((locals.var_qq_dn8 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn8 / locals.var_e2))))), ((locals.var_qq_dn9 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn9 / locals.var_e2))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign10860_e10286;
        locals.var_temp2_dn4 = assign10860_e10286_d_n4;
        locals.var_temp2_dn6 = assign10860_e10286_d_n6;
        locals.var_temp2_dn7 = assign10860_e10286_d_n7;
        locals.var_temp2_dn8 = assign10860_e10286_d_n8;
        locals.var_temp2_dn9 = assign10860_e10286_d_n9;

        let (assign10870_e10294, assign10870_e10294_d_n4, assign10870_e10294_d_n6, assign10870_e10294_d_n7, assign10870_e10294_d_n8, assign10870_e10294_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10870_e10290: f64 = (1.0 - locals.var_temp1);
        let assign10870_e10292: f64 = (assign10870_e10290 - locals.var_temp2);
        (assign10870_e10292, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign10870_e10294;
        locals.var_temp3_dn4 = assign10870_e10294_d_n4;
        locals.var_temp3_dn6 = assign10870_e10294_d_n6;
        locals.var_temp3_dn7 = assign10870_e10294_d_n7;
        locals.var_temp3_dn8 = assign10870_e10294_d_n8;
        locals.var_temp3_dn9 = assign10870_e10294_d_n9;

        let (assign10880_e10300, assign10880_e10300_d_n4, assign10880_e10300_d_n6, assign10880_e10300_d_n7, assign10880_e10300_d_n8, assign10880_e10300_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10880_e10298: f64 = (locals.var_csiprime_0 / locals.var_temp3);
        (assign10880_e10298, (-((locals.var_csiprime_0 * locals.var_temp3_dn4) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn6) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn7) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn8) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn9) / (locals.var_temp3 * locals.var_temp3))),)
    } else {
        (locals.var_csiprime, locals.var_csiprime_dn4, locals.var_csiprime_dn6, locals.var_csiprime_dn7, locals.var_csiprime_dn8, locals.var_csiprime_dn9,)
    }
};
        locals.var_csiprime = assign10880_e10300;
        locals.var_csiprime_dn4 = assign10880_e10300_d_n4;
        locals.var_csiprime_dn6 = assign10880_e10300_d_n6;
        locals.var_csiprime_dn7 = assign10880_e10300_d_n7;
        locals.var_csiprime_dn8 = assign10880_e10300_d_n8;
        locals.var_csiprime_dn9 = assign10880_e10300_d_n9;

    }

    pub(super) fn stamp_transient_block_25(
        locals: &mut StampLocals,
    ) {
        let (assign10890_e10308, assign10890_e10308_d_n4, assign10890_e10308_d_n6, assign10890_e10308_d_n7, assign10890_e10308_d_n8, assign10890_e10308_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10890_e10305: f64 = (locals.var_k1_1d * locals.var_temp1);
        let assign10890_e10306: f64 = (1.0 + assign10890_e10305);
        (assign10890_e10306, (locals.var_k1_1d * locals.var_temp1_dn4), (locals.var_k1_1d * locals.var_temp1_dn6), (locals.var_k1_1d * locals.var_temp1_dn7), (locals.var_k1_1d * locals.var_temp1_dn8), (locals.var_k1_1d * locals.var_temp1_dn9),)
    } else {
        (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9,)
    }
};
        locals.var_tox1fact = assign10890_e10308;
        locals.var_tox1fact_dn4 = assign10890_e10308_d_n4;
        locals.var_tox1fact_dn6 = assign10890_e10308_d_n6;
        locals.var_tox1fact_dn7 = assign10890_e10308_d_n7;
        locals.var_tox1fact_dn8 = assign10890_e10308_d_n8;
        locals.var_tox1fact_dn9 = assign10890_e10308_d_n9;

        let (assign10900_e10316, assign10900_e10316_d_n4, assign10900_e10316_d_n6, assign10900_e10316_d_n7, assign10900_e10316_d_n8, assign10900_e10316_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10900_e10313: f64 = (locals.var_k2_1d * locals.var_temp2);
        let assign10900_e10314: f64 = (1.0 + assign10900_e10313);
        (assign10900_e10314, (locals.var_k2_1d * locals.var_temp2_dn4), (locals.var_k2_1d * locals.var_temp2_dn6), (locals.var_k2_1d * locals.var_temp2_dn7), (locals.var_k2_1d * locals.var_temp2_dn8), (locals.var_k2_1d * locals.var_temp2_dn9),)
    } else {
        (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9,)
    }
};
        locals.var_tox2fact = assign10900_e10316;
        locals.var_tox2fact_dn4 = assign10900_e10316_d_n4;
        locals.var_tox2fact_dn6 = assign10900_e10316_d_n6;
        locals.var_tox2fact_dn7 = assign10900_e10316_d_n7;
        locals.var_tox2fact_dn8 = assign10900_e10316_d_n8;
        locals.var_tox2fact_dn9 = assign10900_e10316_d_n9;

        let (assign10910_e10324, assign10910_e10324_d_n4, assign10910_e10324_d_n6, assign10910_e10324_d_n7, assign10910_e10324_d_n8, assign10910_e10324_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10910_e10320: f64 = (locals.var_k1_1d * locals.var_temp3);
        let assign10910_e10322: f64 = (assign10910_e10320 / locals.var_tox1fact);
        (assign10910_e10322, ((((locals.var_k1_1d * locals.var_temp3_dn4) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn4)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn6)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn7)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn8)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn9)) / (locals.var_tox1fact * locals.var_tox1fact)),)
    } else {
        (locals.var_k1_1d_qm, locals.var_k1_1d_qm_dn4, locals.var_k1_1d_qm_dn6, locals.var_k1_1d_qm_dn7, locals.var_k1_1d_qm_dn8, locals.var_k1_1d_qm_dn9,)
    }
};
        locals.var_k1_1d_qm = assign10910_e10324;
        locals.var_k1_1d_qm_dn4 = assign10910_e10324_d_n4;
        locals.var_k1_1d_qm_dn6 = assign10910_e10324_d_n6;
        locals.var_k1_1d_qm_dn7 = assign10910_e10324_d_n7;
        locals.var_k1_1d_qm_dn8 = assign10910_e10324_d_n8;
        locals.var_k1_1d_qm_dn9 = assign10910_e10324_d_n9;

        let (assign10920_e10332, assign10920_e10332_d_n4, assign10920_e10332_d_n6, assign10920_e10332_d_n7, assign10920_e10332_d_n8, assign10920_e10332_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10920_e10328: f64 = (locals.var_k2_1d * locals.var_temp3);
        let assign10920_e10330: f64 = (assign10920_e10328 / locals.var_tox2fact);
        (assign10920_e10330, ((((locals.var_k2_1d * locals.var_temp3_dn4) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn4)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn6)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn7)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn8)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn9)) / (locals.var_tox2fact * locals.var_tox2fact)),)
    } else {
        (locals.var_k2_1d_qm, locals.var_k2_1d_qm_dn4, locals.var_k2_1d_qm_dn6, locals.var_k2_1d_qm_dn7, locals.var_k2_1d_qm_dn8, locals.var_k2_1d_qm_dn9,)
    }
};
        locals.var_k2_1d_qm = assign10920_e10332;
        locals.var_k2_1d_qm_dn4 = assign10920_e10332_d_n4;
        locals.var_k2_1d_qm_dn6 = assign10920_e10332_d_n6;
        locals.var_k2_1d_qm_dn7 = assign10920_e10332_d_n7;
        locals.var_k2_1d_qm_dn8 = assign10920_e10332_d_n8;
        locals.var_k2_1d_qm_dn9 = assign10920_e10332_d_n9;

        let (assign10930_e10346, assign10930_e10346_d_n4, assign10930_e10346_d_n6, assign10930_e10346_d_n7, assign10930_e10346_d_n8, assign10930_e10346_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10930_e10338: f64 = (1.0 / locals.var_k1_1d_qm);
        let assign10930_e10339: f64 = (1.0 + assign10930_e10338);
        let assign10930_e10342: f64 = (1.0 / locals.var_k2_1d_qm);
        let assign10930_e10343: f64 = (assign10930_e10339 + assign10930_e10342);
        let assign10930_e10344: f64 = (1.0 / assign10930_e10343);
        (assign10930_e10344, (-(((-(locals.var_k1_1d_qm_dn4 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn4 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), (-(((-(locals.var_k1_1d_qm_dn6 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn6 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), (-(((-(locals.var_k1_1d_qm_dn7 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn7 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), (-(((-(locals.var_k1_1d_qm_dn8 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn8 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), (-(((-(locals.var_k1_1d_qm_dn9 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn9 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))),)
    } else {
        (locals.var_keq_1d_qm, locals.var_keq_1d_qm_dn4, locals.var_keq_1d_qm_dn6, locals.var_keq_1d_qm_dn7, locals.var_keq_1d_qm_dn8, locals.var_keq_1d_qm_dn9,)
    }
};
        locals.var_keq_1d_qm = assign10930_e10346;
        locals.var_keq_1d_qm_dn4 = assign10930_e10346_d_n4;
        locals.var_keq_1d_qm_dn6 = assign10930_e10346_d_n6;
        locals.var_keq_1d_qm_dn7 = assign10930_e10346_d_n7;
        locals.var_keq_1d_qm_dn8 = assign10930_e10346_d_n8;
        locals.var_keq_1d_qm_dn9 = assign10930_e10346_d_n9;

        let (assign10940_e10354, assign10940_e10354_d_n4, assign10940_e10354_d_n6, assign10940_e10354_d_n7, assign10940_e10354_d_n8, assign10940_e10354_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10940_e10351: f64 = (locals.var_k1_1d_qm * locals.var_temp1);
        let assign10940_e10352: f64 = (1.0 + assign10940_e10351);
        (assign10940_e10352, ((locals.var_k1_1d_qm_dn4 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn4)), ((locals.var_k1_1d_qm_dn6 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn6)), ((locals.var_k1_1d_qm_dn7 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn7)), ((locals.var_k1_1d_qm_dn8 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn8)), ((locals.var_k1_1d_qm_dn9 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn9)),)
    } else {
        (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9,)
    }
};
        locals.var_tox1fact = assign10940_e10354;
        locals.var_tox1fact_dn4 = assign10940_e10354_d_n4;
        locals.var_tox1fact_dn6 = assign10940_e10354_d_n6;
        locals.var_tox1fact_dn7 = assign10940_e10354_d_n7;
        locals.var_tox1fact_dn8 = assign10940_e10354_d_n8;
        locals.var_tox1fact_dn9 = assign10940_e10354_d_n9;

        let (assign10950_e10362, assign10950_e10362_d_n4, assign10950_e10362_d_n6, assign10950_e10362_d_n7, assign10950_e10362_d_n8, assign10950_e10362_d_n9,) = {
    if (locals.var_guard543 != 0.0) {
        let assign10950_e10359: f64 = (locals.var_k2_1d_qm * locals.var_temp2);
        let assign10950_e10360: f64 = (1.0 + assign10950_e10359);
        (assign10950_e10360, ((locals.var_k2_1d_qm_dn4 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn4)), ((locals.var_k2_1d_qm_dn6 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn6)), ((locals.var_k2_1d_qm_dn7 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn7)), ((locals.var_k2_1d_qm_dn8 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn8)), ((locals.var_k2_1d_qm_dn9 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn9)),)
    } else {
        (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9,)
    }
};
        locals.var_tox2fact = assign10950_e10362;
        locals.var_tox2fact_dn4 = assign10950_e10362_d_n4;
        locals.var_tox2fact_dn6 = assign10950_e10362_d_n6;
        locals.var_tox2fact_dn7 = assign10950_e10362_d_n7;
        locals.var_tox2fact_dn8 = assign10950_e10362_d_n8;
        locals.var_tox2fact_dn9 = assign10950_e10362_d_n9;

        let (assign10960_e10367, assign10960_e10367_d_n4, assign10960_e10367_d_n6, assign10960_e10367_d_n7, assign10960_e10367_d_n8, assign10960_e10367_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (locals.var_csiprime_0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_csiprime, locals.var_csiprime_dn4, locals.var_csiprime_dn6, locals.var_csiprime_dn7, locals.var_csiprime_dn8, locals.var_csiprime_dn9,)
    }
};
        locals.var_csiprime = assign10960_e10367;
        locals.var_csiprime_dn4 = assign10960_e10367_d_n4;
        locals.var_csiprime_dn6 = assign10960_e10367_d_n6;
        locals.var_csiprime_dn7 = assign10960_e10367_d_n7;
        locals.var_csiprime_dn8 = assign10960_e10367_d_n8;
        locals.var_csiprime_dn9 = assign10960_e10367_d_n9;

        let (assign10970_e10372, assign10970_e10372_d_n4, assign10970_e10372_d_n6, assign10970_e10372_d_n7, assign10970_e10372_d_n8, assign10970_e10372_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k1_1d_qm, locals.var_k1_1d_qm_dn4, locals.var_k1_1d_qm_dn6, locals.var_k1_1d_qm_dn7, locals.var_k1_1d_qm_dn8, locals.var_k1_1d_qm_dn9,)
    }
};
        locals.var_k1_1d_qm = assign10970_e10372;
        locals.var_k1_1d_qm_dn4 = assign10970_e10372_d_n4;
        locals.var_k1_1d_qm_dn6 = assign10970_e10372_d_n6;
        locals.var_k1_1d_qm_dn7 = assign10970_e10372_d_n7;
        locals.var_k1_1d_qm_dn8 = assign10970_e10372_d_n8;
        locals.var_k1_1d_qm_dn9 = assign10970_e10372_d_n9;

        let (assign10980_e10377, assign10980_e10377_d_n4, assign10980_e10377_d_n6, assign10980_e10377_d_n7, assign10980_e10377_d_n8, assign10980_e10377_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k2_1d_qm, locals.var_k2_1d_qm_dn4, locals.var_k2_1d_qm_dn6, locals.var_k2_1d_qm_dn7, locals.var_k2_1d_qm_dn8, locals.var_k2_1d_qm_dn9,)
    }
};
        locals.var_k2_1d_qm = assign10980_e10377;
        locals.var_k2_1d_qm_dn4 = assign10980_e10377_d_n4;
        locals.var_k2_1d_qm_dn6 = assign10980_e10377_d_n6;
        locals.var_k2_1d_qm_dn7 = assign10980_e10377_d_n7;
        locals.var_k2_1d_qm_dn8 = assign10980_e10377_d_n8;
        locals.var_k2_1d_qm_dn9 = assign10980_e10377_d_n9;

        let (assign10990_e10382, assign10990_e10382_d_n4, assign10990_e10382_d_n6, assign10990_e10382_d_n7, assign10990_e10382_d_n8, assign10990_e10382_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_keq_1d_qm, locals.var_keq_1d_qm_dn4, locals.var_keq_1d_qm_dn6, locals.var_keq_1d_qm_dn7, locals.var_keq_1d_qm_dn8, locals.var_keq_1d_qm_dn9,)
    }
};
        locals.var_keq_1d_qm = assign10990_e10382;
        locals.var_keq_1d_qm_dn4 = assign10990_e10382_d_n4;
        locals.var_keq_1d_qm_dn6 = assign10990_e10382_d_n6;
        locals.var_keq_1d_qm_dn7 = assign10990_e10382_d_n7;
        locals.var_keq_1d_qm_dn8 = assign10990_e10382_d_n8;
        locals.var_keq_1d_qm_dn9 = assign10990_e10382_d_n9;

        let (assign11000_e10387, assign11000_e10387_d_n4, assign11000_e10387_d_n6, assign11000_e10387_d_n7, assign11000_e10387_d_n8, assign11000_e10387_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9,)
    }
};
        locals.var_tox1fact = assign11000_e10387;
        locals.var_tox1fact_dn4 = assign11000_e10387_d_n4;
        locals.var_tox1fact_dn6 = assign11000_e10387_d_n6;
        locals.var_tox1fact_dn7 = assign11000_e10387_d_n7;
        locals.var_tox1fact_dn8 = assign11000_e10387_d_n8;
        locals.var_tox1fact_dn9 = assign11000_e10387_d_n9;

        let (assign11010_e10392, assign11010_e10392_d_n4, assign11010_e10392_d_n6, assign11010_e10392_d_n7, assign11010_e10392_d_n8, assign11010_e10392_d_n9,) = {
    if (locals.var_guard543 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9,)
    }
};
        locals.var_tox2fact = assign11010_e10392;
        locals.var_tox2fact_dn4 = assign11010_e10392_d_n4;
        locals.var_tox2fact_dn6 = assign11010_e10392_d_n6;
        locals.var_tox2fact_dn7 = assign11010_e10392_d_n7;
        locals.var_tox2fact_dn8 = assign11010_e10392_d_n8;
        locals.var_tox2fact_dn9 = assign11010_e10392_d_n9;

        let assign11020_e10396: f64 = (locals.var_xg10 - locals.var_xg2eff);
        let assign11020_e10397: f64 = (locals.var_keq_1d_qm * assign11020_e10396);
        locals.var_dx_wi_1d = assign11020_e10397;
        locals.var_dx_wi_1d_dn4 = ((locals.var_keq_1d_qm_dn4 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn4 - locals.var_xg2eff_dn4)));
        locals.var_dx_wi_1d_dn6 = ((locals.var_keq_1d_qm_dn6 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn6 - locals.var_xg2eff_dn6)));
        locals.var_dx_wi_1d_dn7 = ((locals.var_keq_1d_qm_dn7 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn7 - locals.var_xg2eff_dn7)));
        locals.var_dx_wi_1d_dn8 = ((locals.var_keq_1d_qm_dn8 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn8 - locals.var_xg2eff_dn8)));
        locals.var_dx_wi_1d_dn9 = ((locals.var_keq_1d_qm_dn9 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn9 - locals.var_xg2eff_dn9)));

        let assign11030_e10400: f64 = if locals.var_dx_wi_1d > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign11030_e10400;

        let assign11040_e10402: f64 = (-locals.var_dx_wi_1d);
        let assign11040_e10404: f64 = if assign11040_e10402 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign11040_e10404;

        let (assign11050_e10415, assign11050_e10415_d_n4, assign11050_e10415_d_n6, assign11050_e10415_d_n7, assign11050_e10415_d_n8, assign11050_e10415_d_n9,) = {
    if ((locals.var_guard544 != 0.0) && (locals.var_guard545 != 0.0)) {
        let assign11050_e10410: f64 = (-locals.var_dx_wi_1d);
        let assign11050_e10411: f64 = (assign11050_e10410).exp();
        let assign11050_e10412: f64 = (1.0 + assign11050_e10411);
        let assign11050_e10413: f64 = (assign11050_e10412).ln();
        (assign11050_e10413, ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn4)) / assign11050_e10412), ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn6)) / assign11050_e10412), ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn7)) / assign11050_e10412), ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn8)) / assign11050_e10412), ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn9)) / assign11050_e10412),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign11050_e10415;
        locals.var_temp_dn4 = assign11050_e10415_d_n4;
        locals.var_temp_dn6 = assign11050_e10415_d_n6;
        locals.var_temp_dn7 = assign11050_e10415_d_n7;
        locals.var_temp_dn8 = assign11050_e10415_d_n8;
        locals.var_temp_dn9 = assign11050_e10415_d_n9;

        let (assign11060_e10423, assign11060_e10423_d_n4, assign11060_e10423_d_n6, assign11060_e10423_d_n7, assign11060_e10423_d_n8, assign11060_e10423_d_n9,) = {
    if ((locals.var_guard544 != 0.0) && (locals.var_guard545 == 0.0)) {
        let assign11060_e10421: f64 = (-locals.var_dx_wi_1d);
        (assign11060_e10421, (-locals.var_dx_wi_1d_dn4), (-locals.var_dx_wi_1d_dn6), (-locals.var_dx_wi_1d_dn7), (-locals.var_dx_wi_1d_dn8), (-locals.var_dx_wi_1d_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign11060_e10423;
        locals.var_temp_dn4 = assign11060_e10423_d_n4;
        locals.var_temp_dn6 = assign11060_e10423_d_n6;
        locals.var_temp_dn7 = assign11060_e10423_d_n7;
        locals.var_temp_dn8 = assign11060_e10423_d_n8;
        locals.var_temp_dn9 = assign11060_e10423_d_n9;

        let (assign11070_e10435, assign11070_e10435_d_n4, assign11070_e10435_d_n6, assign11070_e10435_d_n7, assign11070_e10435_d_n8, assign11070_e10435_d_n9,) = {
    if (locals.var_guard544 != 0.0) {
        let assign11070_e10428: f64 = (locals.var_dx_wi_1d / locals.var_k1_1d_qm);
        let assign11070_e10429: f64 = (locals.var_xg10 - assign11070_e10428);
        let assign11070_e10431: f64 = (assign11070_e10429 + locals.var_temp);
        let assign11070_e10433: f64 = (assign11070_e10431 - 0.6931471805599);
        (assign11070_e10433, ((locals.var_xg10_dn4 - (((locals.var_dx_wi_1d_dn4 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn4)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn4), ((locals.var_xg10_dn6 - (((locals.var_dx_wi_1d_dn6 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn6)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn6), ((locals.var_xg10_dn7 - (((locals.var_dx_wi_1d_dn7 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn7)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn7), ((locals.var_xg10_dn8 - (((locals.var_dx_wi_1d_dn8 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn8)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn8), ((locals.var_xg10_dn9 - (((locals.var_dx_wi_1d_dn9 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn9)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d, locals.var_x_wi_1d_dn4, locals.var_x_wi_1d_dn6, locals.var_x_wi_1d_dn7, locals.var_x_wi_1d_dn8, locals.var_x_wi_1d_dn9,)
    }
};
        locals.var_x_wi_1d = assign11070_e10435;
        locals.var_x_wi_1d_dn4 = assign11070_e10435_d_n4;
        locals.var_x_wi_1d_dn6 = assign11070_e10435_d_n6;
        locals.var_x_wi_1d_dn7 = assign11070_e10435_d_n7;
        locals.var_x_wi_1d_dn8 = assign11070_e10435_d_n8;
        locals.var_x_wi_1d_dn9 = assign11070_e10435_d_n9;

        let assign11080_e10438: f64 = if locals.var_dx_wi_1d < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign11080_e10438;

        let (assign11090_e10449, assign11090_e10449_d_n4, assign11090_e10449_d_n6, assign11090_e10449_d_n7, assign11090_e10449_d_n8, assign11090_e10449_d_n9,) = {
    if ((locals.var_guard544 == 0.0) && (locals.var_guard546 != 0.0)) {
        let assign11090_e10445: f64 = (locals.var_dx_wi_1d).exp();
        let assign11090_e10446: f64 = (1.0 + assign11090_e10445);
        let assign11090_e10447: f64 = (assign11090_e10446).ln();
        (assign11090_e10447, ((assign11090_e10445 * locals.var_dx_wi_1d_dn4) / assign11090_e10446), ((assign11090_e10445 * locals.var_dx_wi_1d_dn6) / assign11090_e10446), ((assign11090_e10445 * locals.var_dx_wi_1d_dn7) / assign11090_e10446), ((assign11090_e10445 * locals.var_dx_wi_1d_dn8) / assign11090_e10446), ((assign11090_e10445 * locals.var_dx_wi_1d_dn9) / assign11090_e10446),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign11090_e10449;
        locals.var_temp_dn4 = assign11090_e10449_d_n4;
        locals.var_temp_dn6 = assign11090_e10449_d_n6;
        locals.var_temp_dn7 = assign11090_e10449_d_n7;
        locals.var_temp_dn8 = assign11090_e10449_d_n8;
        locals.var_temp_dn9 = assign11090_e10449_d_n9;

        let (assign11100_e10457, assign11100_e10457_d_n4, assign11100_e10457_d_n6, assign11100_e10457_d_n7, assign11100_e10457_d_n8, assign11100_e10457_d_n9,) = {
    if ((locals.var_guard544 == 0.0) && (locals.var_guard546 == 0.0)) {
        (locals.var_dx_wi_1d, locals.var_dx_wi_1d_dn4, locals.var_dx_wi_1d_dn6, locals.var_dx_wi_1d_dn7, locals.var_dx_wi_1d_dn8, locals.var_dx_wi_1d_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign11100_e10457;
        locals.var_temp_dn4 = assign11100_e10457_d_n4;
        locals.var_temp_dn6 = assign11100_e10457_d_n6;
        locals.var_temp_dn7 = assign11100_e10457_d_n7;
        locals.var_temp_dn8 = assign11100_e10457_d_n8;
        locals.var_temp_dn9 = assign11100_e10457_d_n9;

        let (assign11110_e10470, assign11110_e10470_d_n4, assign11110_e10470_d_n6, assign11110_e10470_d_n7, assign11110_e10470_d_n8, assign11110_e10470_d_n9,) = {
    if (locals.var_guard544 == 0.0) {
        let assign11110_e10463: f64 = (locals.var_dx_wi_1d / locals.var_k2_1d_qm);
        let assign11110_e10464: f64 = (locals.var_xg2eff + assign11110_e10463);
        let assign11110_e10466: f64 = (assign11110_e10464 + locals.var_temp);
        let assign11110_e10468: f64 = (assign11110_e10466 - 0.6931471805599);
        (assign11110_e10468, ((locals.var_xg2eff_dn4 + (((locals.var_dx_wi_1d_dn4 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn4)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn4), ((locals.var_xg2eff_dn6 + (((locals.var_dx_wi_1d_dn6 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn6)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn6), ((locals.var_xg2eff_dn7 + (((locals.var_dx_wi_1d_dn7 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn7)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn7), ((locals.var_xg2eff_dn8 + (((locals.var_dx_wi_1d_dn8 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn8)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn8), ((locals.var_xg2eff_dn9 + (((locals.var_dx_wi_1d_dn9 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn9)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d, locals.var_x_wi_1d_dn4, locals.var_x_wi_1d_dn6, locals.var_x_wi_1d_dn7, locals.var_x_wi_1d_dn8, locals.var_x_wi_1d_dn9,)
    }
};
        locals.var_x_wi_1d = assign11110_e10470;
        locals.var_x_wi_1d_dn4 = assign11110_e10470_d_n4;
        locals.var_x_wi_1d_dn6 = assign11110_e10470_d_n6;
        locals.var_x_wi_1d_dn7 = assign11110_e10470_d_n7;
        locals.var_x_wi_1d_dn8 = assign11110_e10470_d_n8;
        locals.var_x_wi_1d_dn9 = assign11110_e10470_d_n9;

        let assign11120_e10474: f64 = (locals.var_x_wi_1d + locals.var_xth_1d);
        let assign11120_e10477: f64 = (locals.var_x_wi_1d - locals.var_xth_1d);
        let assign11120_e10480: f64 = (locals.var_x_wi_1d - locals.var_xth_1d);
        let assign11120_e10481: f64 = (assign11120_e10477 * assign11120_e10480);
        let assign11120_e10483: f64 = (assign11120_e10481 + 4.0);
        let assign11120_e10484: f64 = (assign11120_e10483).sqrt();
        let assign11120_e10485: f64 = (assign11120_e10474 - assign11120_e10484);
        let assign11120_e10486: f64 = (0.5 * assign11120_e10485);
        locals.var_x_1d = assign11120_e10486;
        locals.var_x_1d_dn4 = (0.5 * ((locals.var_x_wi_1d_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_x_wi_1d_dn4 - locals.var_xth_1d_dn4) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign11120_e10484))));
        locals.var_x_1d_dn6 = (0.5 * ((locals.var_x_wi_1d_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_x_wi_1d_dn6 - locals.var_xth_1d_dn6) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign11120_e10484))));
        locals.var_x_1d_dn7 = (0.5 * ((locals.var_x_wi_1d_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_x_wi_1d_dn7 - locals.var_xth_1d_dn7) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign11120_e10484))));
        locals.var_x_1d_dn8 = (0.5 * ((locals.var_x_wi_1d_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_x_wi_1d_dn8 - locals.var_xth_1d_dn8) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign11120_e10484))));
        locals.var_x_1d_dn9 = (0.5 * ((locals.var_x_wi_1d_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_x_wi_1d_dn9 - locals.var_xth_1d_dn9) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign11120_e10484))));

        let assign11130_e10491: f64 = (locals.var_xth_1d - locals.var_x_1d);
        let assign11130_e10492: f64 = (2.0 * assign11130_e10491);
        let assign11130_e10494: f64 = (assign11130_e10492 / locals.var_xsddep);
        let assign11130_e10495: f64 = (1.0 + assign11130_e10494);
        let assign11130_e10496: f64 = (assign11130_e10495).sqrt();
        let assign11130_e10498: f64 = (assign11130_e10496 - 1.0);
        locals.var_dleff = assign11130_e10498;
        locals.var_dleff_dn4 = (((((2.0 * (locals.var_xth_1d_dn4 - locals.var_x_1d_dn4)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn4)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496));
        locals.var_dleff_dn6 = (((((2.0 * (locals.var_xth_1d_dn6 - locals.var_x_1d_dn6)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn6)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496));
        locals.var_dleff_dn7 = (((((2.0 * (locals.var_xth_1d_dn7 - locals.var_x_1d_dn7)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn7)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496));
        locals.var_dleff_dn8 = (((((2.0 * (locals.var_xth_1d_dn8 - locals.var_x_1d_dn8)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn8)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496));
        locals.var_dleff_dn9 = (((((2.0 * (locals.var_xth_1d_dn9 - locals.var_x_1d_dn9)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn9)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496));

        let assign11140_e10502: f64 = (locals.var_xsddep * locals.var_dleff);
        let assign11140_e10503: f64 = (locals.var_x_1d + assign11140_e10502);
        locals.var_xedge = assign11140_e10503;
        locals.var_xedge_dn4 = (locals.var_x_1d_dn4 + ((locals.var_xsddep_dn4 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn4)));
        locals.var_xedge_dn6 = (locals.var_x_1d_dn6 + ((locals.var_xsddep_dn6 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn6)));
        locals.var_xedge_dn7 = (locals.var_x_1d_dn7 + ((locals.var_xsddep_dn7 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn7)));
        locals.var_xedge_dn8 = (locals.var_x_1d_dn8 + ((locals.var_xsddep_dn8 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn8)));
        locals.var_xedge_dn9 = (locals.var_x_1d_dn9 + ((locals.var_xsddep_dn9 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn9)));

        let assign11150_e10508: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11150_e10509: f64 = (1.0 + assign11150_e10508);
        let assign11150_e10511: f64 = (assign11150_e10509 + 0.5);
        let assign11150_e10515: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11150_e10516: f64 = (1.0 + assign11150_e10515);
        let assign11150_e10518: f64 = (assign11150_e10516 - 0.5);
        let assign11150_e10522: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11150_e10523: f64 = (1.0 + assign11150_e10522);
        let assign11150_e10525: f64 = (assign11150_e10523 - 0.5);
        let assign11150_e10526: f64 = (assign11150_e10518 * assign11150_e10525);
        let assign11150_e10528: f64 = (assign11150_e10526 + 0.01);
        let assign11150_e10529: f64 = (assign11150_e10528).sqrt();
        let assign11150_e10530: f64 = (assign11150_e10511 + assign11150_e10529);
        let assign11150_e10531: f64 = (0.5 * assign11150_e10530);
        locals.var_temp = assign11150_e10531;
        locals.var_temp_dn4 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn4) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn4))) / (2.0 * assign11150_e10529))));
        locals.var_temp_dn6 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn6) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn6))) / (2.0 * assign11150_e10529))));
        locals.var_temp_dn7 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn7) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn7))) / (2.0 * assign11150_e10529))));
        locals.var_temp_dn8 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn8) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn8))) / (2.0 * assign11150_e10529))));
        locals.var_temp_dn9 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn9) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn9))) / (2.0 * assign11150_e10529))));

        let assign11160_e10536: f64 = (locals.var_psce1_loc * locals.var_temp);
        let assign11160_e10537: f64 = (1.0 + assign11160_e10536);
        let assign11160_e10538: f64 = (1.0 / assign11160_e10537);
        locals.var_sce1 = assign11160_e10538;
        locals.var_sce1_dn4 = (-((locals.var_psce1_loc * locals.var_temp_dn4) / (assign11160_e10537 * assign11160_e10537)));
        locals.var_sce1_dn6 = (-((locals.var_psce1_loc * locals.var_temp_dn6) / (assign11160_e10537 * assign11160_e10537)));
        locals.var_sce1_dn7 = (-((locals.var_psce1_loc * locals.var_temp_dn7) / (assign11160_e10537 * assign11160_e10537)));
        locals.var_sce1_dn8 = (-((locals.var_psce1_loc * locals.var_temp_dn8) / (assign11160_e10537 * assign11160_e10537)));
        locals.var_sce1_dn9 = (-((locals.var_psce1_loc * locals.var_temp_dn9) / (assign11160_e10537 * assign11160_e10537)));

        let assign11170_e10543: f64 = (locals.var_psce2_loc * locals.var_temp);
        let assign11170_e10544: f64 = (1.0 + assign11170_e10543);
        let assign11170_e10545: f64 = (1.0 / assign11170_e10544);
        locals.var_sce2 = assign11170_e10545;
        locals.var_sce2_dn4 = (-((locals.var_psce2_loc * locals.var_temp_dn4) / (assign11170_e10544 * assign11170_e10544)));
        locals.var_sce2_dn6 = (-((locals.var_psce2_loc * locals.var_temp_dn6) / (assign11170_e10544 * assign11170_e10544)));
        locals.var_sce2_dn7 = (-((locals.var_psce2_loc * locals.var_temp_dn7) / (assign11170_e10544 * assign11170_e10544)));
        locals.var_sce2_dn8 = (-((locals.var_psce2_loc * locals.var_temp_dn8) / (assign11170_e10544 * assign11170_e10544)));
        locals.var_sce2_dn9 = (-((locals.var_psce2_loc * locals.var_temp_dn9) / (assign11170_e10544 * assign11170_e10544)));

        let assign11180_e10548: f64 = (2.0 * locals.var_xd0);
        let assign11180_e10552: f64 = (locals.var_xdsx / locals.var_xd0);
        let assign11180_e10553: f64 = (1.0 + assign11180_e10552);
        let assign11180_e10554: f64 = (assign11180_e10553).sqrt();
        let assign11180_e10556: f64 = (assign11180_e10554 - 1.0);
        let assign11180_e10557: f64 = (assign11180_e10548 * assign11180_e10556);
        let assign11180_e10561: f64 = (locals.var_cfdl_i * locals.var_dleff);
        let assign11180_e10562: f64 = (1.0 + assign11180_e10561);
        let assign11180_e10563: f64 = (assign11180_e10557 * assign11180_e10562);
        let assign11180_e10567: f64 = (locals.var_cfdlb_i * locals.var_xg20shift);
        let assign11180_e10568: f64 = (1.0 + assign11180_e10567);
        let assign11180_e10569: f64 = (assign11180_e10563 * assign11180_e10568);
        locals.var_temp = assign11180_e10569;
        locals.var_temp_dn4 = (((((((2.0 * locals.var_xd0_dn4) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn4 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn4)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn4))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn4)));
        locals.var_temp_dn6 = (((((((2.0 * locals.var_xd0_dn6) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn6 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn6)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn6))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn6)));
        locals.var_temp_dn7 = (((((((2.0 * locals.var_xd0_dn7) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn7 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn7)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn7))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn7)));
        locals.var_temp_dn8 = (((((((2.0 * locals.var_xd0_dn8) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn8 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn8)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn8))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn8)));
        locals.var_temp_dn9 = (((((((2.0 * locals.var_xd0_dn9) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn9 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn9)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn9))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn9)));

        let assign11190_e10572: f64 = (locals.var_cf1_loc * locals.var_temp);
        locals.var_dxg1_dibl = assign11190_e10572;
        locals.var_dxg1_dibl_dn4 = ((locals.var_cf1_loc_dn4 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn4));
        locals.var_dxg1_dibl_dn6 = ((locals.var_cf1_loc_dn6 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn6));
        locals.var_dxg1_dibl_dn7 = ((locals.var_cf1_loc_dn7 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn7));
        locals.var_dxg1_dibl_dn8 = ((locals.var_cf1_loc_dn8 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn8));
        locals.var_dxg1_dibl_dn9 = ((locals.var_cf1_loc_dn9 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn9));

        let assign11200_e10575: f64 = (locals.var_cf2_loc * locals.var_temp);
        locals.var_dxg2_dibl = assign11200_e10575;
        locals.var_dxg2_dibl_dn4 = ((locals.var_cf2_loc_dn4 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn4));
        locals.var_dxg2_dibl_dn6 = ((locals.var_cf2_loc_dn6 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn6));
        locals.var_dxg2_dibl_dn7 = ((locals.var_cf2_loc_dn7 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn7));
        locals.var_dxg2_dibl_dn8 = ((locals.var_cf2_loc_dn8 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn8));
        locals.var_dxg2_dibl_dn9 = ((locals.var_cf2_loc_dn9 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn9));

        let assign11210_e10578: f64 = (locals.var_xg10 - locals.var_xedge);
        let assign11210_e10580: f64 = (assign11210_e10578 + locals.var_dxg1_dibl);
        let assign11210_e10582: f64 = (assign11210_e10580 * locals.var_sce1);
        let assign11210_e10584: f64 = (assign11210_e10582 + locals.var_xedge);
        let assign11210_e10586: f64 = (assign11210_e10584 + locals.var_dxdsx);
        locals.var_xg1 = assign11210_e10586;
        locals.var_xg1_dn4 = ((((((locals.var_xg10_dn4 - locals.var_xedge_dn4) + locals.var_dxg1_dibl_dn4) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn4)) + locals.var_xedge_dn4) + locals.var_dxdsx_dn4);
        locals.var_xg1_dn6 = ((((((locals.var_xg10_dn6 - locals.var_xedge_dn6) + locals.var_dxg1_dibl_dn6) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn6)) + locals.var_xedge_dn6) + locals.var_dxdsx_dn6);
        locals.var_xg1_dn7 = ((((((locals.var_xg10_dn7 - locals.var_xedge_dn7) + locals.var_dxg1_dibl_dn7) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn7)) + locals.var_xedge_dn7) + locals.var_dxdsx_dn7);
        locals.var_xg1_dn8 = ((((((locals.var_xg10_dn8 - locals.var_xedge_dn8) + locals.var_dxg1_dibl_dn8) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn8)) + locals.var_xedge_dn8) + locals.var_dxdsx_dn8);
        locals.var_xg1_dn9 = ((((((locals.var_xg10_dn9 - locals.var_xedge_dn9) + locals.var_dxg1_dibl_dn9) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn9)) + locals.var_xedge_dn9) + locals.var_dxdsx_dn9);

        let assign11220_e10589: f64 = (locals.var_xg2eff - locals.var_xedge);
        let assign11220_e10591: f64 = (assign11220_e10589 + locals.var_dxg2_dibl);
        let assign11220_e10593: f64 = (assign11220_e10591 * locals.var_sce2);
        let assign11220_e10595: f64 = (assign11220_e10593 + locals.var_xedge);
        let assign11220_e10597: f64 = (assign11220_e10595 + locals.var_dxdsx);
        locals.var_xg2 = assign11220_e10597;
        locals.var_xg2_dn4 = ((((((locals.var_xg2eff_dn4 - locals.var_xedge_dn4) + locals.var_dxg2_dibl_dn4) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn4)) + locals.var_xedge_dn4) + locals.var_dxdsx_dn4);
        locals.var_xg2_dn6 = ((((((locals.var_xg2eff_dn6 - locals.var_xedge_dn6) + locals.var_dxg2_dibl_dn6) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn6)) + locals.var_xedge_dn6) + locals.var_dxdsx_dn6);
        locals.var_xg2_dn7 = ((((((locals.var_xg2eff_dn7 - locals.var_xedge_dn7) + locals.var_dxg2_dibl_dn7) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn7)) + locals.var_xedge_dn7) + locals.var_dxdsx_dn7);
        locals.var_xg2_dn8 = ((((((locals.var_xg2eff_dn8 - locals.var_xedge_dn8) + locals.var_dxg2_dibl_dn8) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn8)) + locals.var_xedge_dn8) + locals.var_dxdsx_dn8);
        locals.var_xg2_dn9 = ((((((locals.var_xg2eff_dn9 - locals.var_xedge_dn9) + locals.var_dxg2_dibl_dn9) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn9)) + locals.var_xedge_dn9) + locals.var_dxdsx_dn9);

        let assign11230_e10603: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11230_e10604: f64 = (locals.var_cic1_i * assign11230_e10603);
        let assign11230_e10605: f64 = (locals.var_xg2 + assign11230_e10604);
        let assign11230_e10607: f64 = (assign11230_e10605 + locals.var_xsatmax);
        let assign11230_e10612: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11230_e10613: f64 = (locals.var_cic1_i * assign11230_e10612);
        let assign11230_e10614: f64 = (locals.var_xg2 + assign11230_e10613);
        let assign11230_e10616: f64 = (assign11230_e10614 - locals.var_xsatmax);
        let assign11230_e10621: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11230_e10622: f64 = (locals.var_cic1_i * assign11230_e10621);
        let assign11230_e10623: f64 = (locals.var_xg2 + assign11230_e10622);
        let assign11230_e10625: f64 = (assign11230_e10623 - locals.var_xsatmax);
        let assign11230_e10626: f64 = (assign11230_e10616 * assign11230_e10625);
        let assign11230_e10628: f64 = (assign11230_e10626 + 0.01);
        let assign11230_e10629: f64 = (assign11230_e10628).sqrt();
        let assign11230_e10630: f64 = (assign11230_e10607 - assign11230_e10629);
        let assign11230_e10631: f64 = (0.5 * assign11230_e10630);
        locals.var_xg1x = assign11230_e10631;
        locals.var_xg1x_dn4 = (0.5 * (((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) - locals.var_xsatmax_dn4) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign11230_e10629))));
        locals.var_xg1x_dn6 = (0.5 * (((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) - locals.var_xsatmax_dn6) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign11230_e10629))));
        locals.var_xg1x_dn7 = (0.5 * (((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) - locals.var_xsatmax_dn7) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign11230_e10629))));
        locals.var_xg1x_dn8 = (0.5 * (((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) - locals.var_xsatmax_dn8) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign11230_e10629))));
        locals.var_xg1x_dn9 = (0.5 * (((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) - locals.var_xsatmax_dn9) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign11230_e10629))));

        let assign11240_e10637: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11240_e10638: f64 = (locals.var_cic2_i * assign11240_e10637);
        let assign11240_e10639: f64 = (locals.var_xg1 + assign11240_e10638);
        let assign11240_e10641: f64 = (assign11240_e10639 + locals.var_xsatmax);
        let assign11240_e10646: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11240_e10647: f64 = (locals.var_cic2_i * assign11240_e10646);
        let assign11240_e10648: f64 = (locals.var_xg1 + assign11240_e10647);
        let assign11240_e10650: f64 = (assign11240_e10648 - locals.var_xsatmax);
        let assign11240_e10655: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11240_e10656: f64 = (locals.var_cic2_i * assign11240_e10655);
        let assign11240_e10657: f64 = (locals.var_xg1 + assign11240_e10656);
        let assign11240_e10659: f64 = (assign11240_e10657 - locals.var_xsatmax);
        let assign11240_e10660: f64 = (assign11240_e10650 * assign11240_e10659);
        let assign11240_e10662: f64 = (assign11240_e10660 + 0.01);
        let assign11240_e10663: f64 = (assign11240_e10662).sqrt();
        let assign11240_e10664: f64 = (assign11240_e10641 - assign11240_e10663);
        let assign11240_e10665: f64 = (0.5 * assign11240_e10664);
        locals.var_xg2x = assign11240_e10665;
        locals.var_xg2x_dn4 = (0.5 * (((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) - locals.var_xsatmax_dn4) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign11240_e10663))));
        locals.var_xg2x_dn6 = (0.5 * (((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) - locals.var_xsatmax_dn6) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign11240_e10663))));
        locals.var_xg2x_dn7 = (0.5 * (((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) - locals.var_xsatmax_dn7) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign11240_e10663))));
        locals.var_xg2x_dn8 = (0.5 * (((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) - locals.var_xsatmax_dn8) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign11240_e10663))));
        locals.var_xg2x_dn9 = (0.5 * (((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) - locals.var_xsatmax_dn9) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign11240_e10663))));

        let assign11250_e10668: f64 = (locals.var_k1_1d_qm / locals.var_sce1);
        locals.var_k1 = assign11250_e10668;
        locals.var_k1_dn4 = (((locals.var_k1_1d_qm_dn4 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn4)) / (locals.var_sce1 * locals.var_sce1));
        locals.var_k1_dn6 = (((locals.var_k1_1d_qm_dn6 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn6)) / (locals.var_sce1 * locals.var_sce1));
        locals.var_k1_dn7 = (((locals.var_k1_1d_qm_dn7 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn7)) / (locals.var_sce1 * locals.var_sce1));
        locals.var_k1_dn8 = (((locals.var_k1_1d_qm_dn8 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn8)) / (locals.var_sce1 * locals.var_sce1));
        locals.var_k1_dn9 = (((locals.var_k1_1d_qm_dn9 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn9)) / (locals.var_sce1 * locals.var_sce1));

    }

    pub(super) fn stamp_transient_block_26(
        locals: &mut StampLocals,
    ) {
        let assign11260_e10671: f64 = (locals.var_k2_1d_qm / locals.var_sce2);
        locals.var_k2 = assign11260_e10671;
        locals.var_k2_dn4 = (((locals.var_k2_1d_qm_dn4 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn4)) / (locals.var_sce2 * locals.var_sce2));
        locals.var_k2_dn6 = (((locals.var_k2_1d_qm_dn6 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn6)) / (locals.var_sce2 * locals.var_sce2));
        locals.var_k2_dn7 = (((locals.var_k2_1d_qm_dn7 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn7)) / (locals.var_sce2 * locals.var_sce2));
        locals.var_k2_dn8 = (((locals.var_k2_1d_qm_dn8 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn8)) / (locals.var_sce2 * locals.var_sce2));
        locals.var_k2_dn9 = (((locals.var_k2_1d_qm_dn9 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn9)) / (locals.var_sce2 * locals.var_sce2));

        let assign11270_e10674: f64 = (1.0 / locals.var_k1);
        locals.var_inv_k1 = assign11270_e10674;
        locals.var_inv_k1_dn4 = (-(locals.var_k1_dn4 / (locals.var_k1 * locals.var_k1)));
        locals.var_inv_k1_dn6 = (-(locals.var_k1_dn6 / (locals.var_k1 * locals.var_k1)));
        locals.var_inv_k1_dn7 = (-(locals.var_k1_dn7 / (locals.var_k1 * locals.var_k1)));
        locals.var_inv_k1_dn8 = (-(locals.var_k1_dn8 / (locals.var_k1 * locals.var_k1)));
        locals.var_inv_k1_dn9 = (-(locals.var_k1_dn9 / (locals.var_k1 * locals.var_k1)));

        let assign11280_e10677: f64 = (1.0 / locals.var_k2);
        locals.var_inv_k2 = assign11280_e10677;
        locals.var_inv_k2_dn4 = (-(locals.var_k2_dn4 / (locals.var_k2 * locals.var_k2)));
        locals.var_inv_k2_dn6 = (-(locals.var_k2_dn6 / (locals.var_k2 * locals.var_k2)));
        locals.var_inv_k2_dn7 = (-(locals.var_k2_dn7 / (locals.var_k2 * locals.var_k2)));
        locals.var_inv_k2_dn8 = (-(locals.var_k2_dn8 / (locals.var_k2 * locals.var_k2)));
        locals.var_inv_k2_dn9 = (-(locals.var_k2_dn9 / (locals.var_k2 * locals.var_k2)));

        let assign11290_e10681: f64 = (1.0 + locals.var_inv_k1);
        let assign11290_e10683: f64 = (assign11290_e10681 + locals.var_inv_k2);
        let assign11290_e10684: f64 = (1.0 / assign11290_e10683);
        locals.var_keq = assign11290_e10684;
        locals.var_keq_dn4 = (-((locals.var_inv_k1_dn4 + locals.var_inv_k2_dn4) / (assign11290_e10683 * assign11290_e10683)));
        locals.var_keq_dn6 = (-((locals.var_inv_k1_dn6 + locals.var_inv_k2_dn6) / (assign11290_e10683 * assign11290_e10683)));
        locals.var_keq_dn7 = (-((locals.var_inv_k1_dn7 + locals.var_inv_k2_dn7) / (assign11290_e10683 * assign11290_e10683)));
        locals.var_keq_dn8 = (-((locals.var_inv_k1_dn8 + locals.var_inv_k2_dn8) / (assign11290_e10683 * assign11290_e10683)));
        locals.var_keq_dn9 = (-((locals.var_inv_k1_dn9 + locals.var_inv_k2_dn9) / (assign11290_e10683 * assign11290_e10683)));

        let assign11300_e10688: f64 = (locals.var_csiprime * locals.var_csiprime);
        let assign11300_e10689: f64 = (locals.var_a0_csisq / assign11300_e10688);
        locals.var_a0 = assign11300_e10689;
        locals.var_a0_dn4 = (((locals.var_a0_csisq_dn4 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn4 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn4)))) / (assign11300_e10688 * assign11300_e10688));
        locals.var_a0_dn6 = (((locals.var_a0_csisq_dn6 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn6 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn6)))) / (assign11300_e10688 * assign11300_e10688));
        locals.var_a0_dn7 = (((locals.var_a0_csisq_dn7 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn7 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn7)))) / (assign11300_e10688 * assign11300_e10688));
        locals.var_a0_dn8 = (((locals.var_a0_csisq_dn8 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn8 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn8)))) / (assign11300_e10688 * assign11300_e10688));
        locals.var_a0_dn9 = (((locals.var_a0_csisq_dn9 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn9 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn9)))) / (assign11300_e10688 * assign11300_e10688));

        let assign11310_e10692: f64 = (1.0 + locals.var_k1);
        let assign11310_e10695: f64 = (1.0 + locals.var_k2);
        let assign11310_e10696: f64 = (assign11310_e10692 / assign11310_e10695);
        locals.var_exp_dxth = assign11310_e10696;
        locals.var_exp_dxth_dn4 = (((locals.var_k1_dn4 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn4)) / (assign11310_e10695 * assign11310_e10695));
        locals.var_exp_dxth_dn6 = (((locals.var_k1_dn6 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn6)) / (assign11310_e10695 * assign11310_e10695));
        locals.var_exp_dxth_dn7 = (((locals.var_k1_dn7 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn7)) / (assign11310_e10695 * assign11310_e10695));
        locals.var_exp_dxth_dn8 = (((locals.var_k1_dn8 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn8)) / (assign11310_e10695 * assign11310_e10695));
        locals.var_exp_dxth_dn9 = (((locals.var_k1_dn9 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn9)) / (assign11310_e10695 * assign11310_e10695));

        let assign11320_e10698: f64 = (locals.var_exp_dxth).ln();
        locals.var_dxth = assign11320_e10698;
        locals.var_dxth_dn4 = (locals.var_exp_dxth_dn4 / locals.var_exp_dxth);
        locals.var_dxth_dn6 = (locals.var_exp_dxth_dn6 / locals.var_exp_dxth);
        locals.var_dxth_dn7 = (locals.var_exp_dxth_dn7 / locals.var_exp_dxth);
        locals.var_dxth_dn8 = (locals.var_exp_dxth_dn8 / locals.var_exp_dxth);
        locals.var_dxth_dn9 = (locals.var_exp_dxth_dn9 / locals.var_exp_dxth);

        let assign11330_e10701: f64 = if locals.var_dxth > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign11330_e10701;

        let (assign11340_e10715, assign11340_e10715_d_n4, assign11340_e10715_d_n6, assign11340_e10715_d_n7, assign11340_e10715_d_n8, assign11340_e10715_d_n9,) = {
    if (locals.var_guard547 != 0.0) {
        let assign11340_e10705: f64 = (2.0 * locals.var_dxth);
        let assign11340_e10708: f64 = (locals.var_exp_dxth + 1.0);
        let assign11340_e10709: f64 = (assign11340_e10705 * assign11340_e10708);
        let assign11340_e10712: f64 = (locals.var_exp_dxth - 1.0);
        let assign11340_e10713: f64 = (assign11340_e10709 / assign11340_e10712);
        (assign11340_e10713, ((((((2.0 * locals.var_dxth_dn4) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn4)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn4)) / (assign11340_e10712 * assign11340_e10712)), ((((((2.0 * locals.var_dxth_dn6) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn6)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn6)) / (assign11340_e10712 * assign11340_e10712)), ((((((2.0 * locals.var_dxth_dn7) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn7)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn7)) / (assign11340_e10712 * assign11340_e10712)), ((((((2.0 * locals.var_dxth_dn8) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn8)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn8)) / (assign11340_e10712 * assign11340_e10712)), ((((((2.0 * locals.var_dxth_dn9) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn9)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn9)) / (assign11340_e10712 * assign11340_e10712)),)
    } else {
        (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9,)
    }
};
        locals.var_diff_min = assign11340_e10715;
        locals.var_diff_min_dn4 = assign11340_e10715_d_n4;
        locals.var_diff_min_dn6 = assign11340_e10715_d_n6;
        locals.var_diff_min_dn7 = assign11340_e10715_d_n7;
        locals.var_diff_min_dn8 = assign11340_e10715_d_n8;
        locals.var_diff_min_dn9 = assign11340_e10715_d_n9;

        let (assign11350_e10724, assign11350_e10724_d_n4, assign11350_e10724_d_n6, assign11350_e10724_d_n7, assign11350_e10724_d_n8, assign11350_e10724_d_n9,) = {
    if (locals.var_guard547 == 0.0) {
        let assign11350_e10721: f64 = (2.0 + locals.var_dxth);
        let assign11350_e10722: f64 = (2.0 * assign11350_e10721);
        (assign11350_e10722, (2.0 * locals.var_dxth_dn4), (2.0 * locals.var_dxth_dn6), (2.0 * locals.var_dxth_dn7), (2.0 * locals.var_dxth_dn8), (2.0 * locals.var_dxth_dn9),)
    } else {
        (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9,)
    }
};
        locals.var_diff_min = assign11350_e10724;
        locals.var_diff_min_dn4 = assign11350_e10724_d_n4;
        locals.var_diff_min_dn6 = assign11350_e10724_d_n6;
        locals.var_diff_min_dn7 = assign11350_e10724_d_n7;
        locals.var_diff_min_dn8 = assign11350_e10724_d_n8;
        locals.var_diff_min_dn9 = assign11350_e10724_d_n9;

        let assign11360_e10728: f64 = (locals.var_xg1x - locals.var_xg2x);
        let assign11360_e10729: f64 = (locals.var_keq * assign11360_e10728);
        locals.var_dx_wi = assign11360_e10729;
        locals.var_dx_wi_dn4 = ((locals.var_keq_dn4 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn4 - locals.var_xg2x_dn4)));
        locals.var_dx_wi_dn6 = ((locals.var_keq_dn6 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn6 - locals.var_xg2x_dn6)));
        locals.var_dx_wi_dn7 = ((locals.var_keq_dn7 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn7 - locals.var_xg2x_dn7)));
        locals.var_dx_wi_dn8 = ((locals.var_keq_dn8 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn8 - locals.var_xg2x_dn8)));
        locals.var_dx_wi_dn9 = ((locals.var_keq_dn9 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn9 - locals.var_xg2x_dn9)));

        let assign11370_e10732: f64 = (locals.var_dx_wi * locals.var_dx_wi);
        locals.var_dx_wisq = assign11370_e10732;
        locals.var_dx_wisq_dn4 = ((locals.var_dx_wi_dn4 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn4));
        locals.var_dx_wisq_dn6 = ((locals.var_dx_wi_dn6 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn6));
        locals.var_dx_wisq_dn7 = ((locals.var_dx_wi_dn7 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn7));
        locals.var_dx_wisq_dn8 = ((locals.var_dx_wi_dn8 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn8));
        locals.var_dx_wisq_dn9 = ((locals.var_dx_wi_dn9 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn9));

        let assign11380_e10736: f64 = (locals.var_dx_wi * locals.var_inv_k1);
        let assign11380_e10737: f64 = (locals.var_xg1x - assign11380_e10736);
        locals.var_x1_wi0 = assign11380_e10737;
        locals.var_x1_wi0_dn4 = (locals.var_xg1x_dn4 - ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4)));
        locals.var_x1_wi0_dn6 = (locals.var_xg1x_dn6 - ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6)));
        locals.var_x1_wi0_dn7 = (locals.var_xg1x_dn7 - ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7)));
        locals.var_x1_wi0_dn8 = (locals.var_xg1x_dn8 - ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8)));
        locals.var_x1_wi0_dn9 = (locals.var_xg1x_dn9 - ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9)));

        let assign11390_e10741: f64 = (locals.var_dx_wi * locals.var_inv_k2);
        let assign11390_e10742: f64 = (locals.var_xg2x + assign11390_e10741);
        locals.var_x2_wi0 = assign11390_e10742;
        locals.var_x2_wi0_dn4 = (locals.var_xg2x_dn4 + ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4)));
        locals.var_x2_wi0_dn6 = (locals.var_xg2x_dn6 + ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6)));
        locals.var_x2_wi0_dn7 = (locals.var_xg2x_dn7 + ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7)));
        locals.var_x2_wi0_dn8 = (locals.var_xg2x_dn8 + ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8)));
        locals.var_x2_wi0_dn9 = (locals.var_xg2x_dn9 + ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9)));

        let assign11400_e10746: f64 = (locals.var_k1 + 1.0);
        let assign11400_e10747: f64 = (1.0 / assign11400_e10746);
        locals.var_q_temp1 = assign11400_e10747;
        locals.var_q_temp1_dn4 = (-(locals.var_k1_dn4 / (assign11400_e10746 * assign11400_e10746)));
        locals.var_q_temp1_dn6 = (-(locals.var_k1_dn6 / (assign11400_e10746 * assign11400_e10746)));
        locals.var_q_temp1_dn7 = (-(locals.var_k1_dn7 / (assign11400_e10746 * assign11400_e10746)));
        locals.var_q_temp1_dn8 = (-(locals.var_k1_dn8 / (assign11400_e10746 * assign11400_e10746)));
        locals.var_q_temp1_dn9 = (-(locals.var_k1_dn9 / (assign11400_e10746 * assign11400_e10746)));

        let assign11410_e10751: f64 = (locals.var_k2 + 1.0);
        let assign11410_e10752: f64 = (1.0 / assign11410_e10751);
        locals.var_q_temp2 = assign11410_e10752;
        locals.var_q_temp2_dn4 = (-(locals.var_k2_dn4 / (assign11410_e10751 * assign11410_e10751)));
        locals.var_q_temp2_dn6 = (-(locals.var_k2_dn6 / (assign11410_e10751 * assign11410_e10751)));
        locals.var_q_temp2_dn7 = (-(locals.var_k2_dn7 / (assign11410_e10751 * assign11410_e10751)));
        locals.var_q_temp2_dn8 = (-(locals.var_k2_dn8 / (assign11410_e10751 * assign11410_e10751)));
        locals.var_q_temp2_dn9 = (-(locals.var_k2_dn9 / (assign11410_e10751 * assign11410_e10751)));

        let assign11420_e10756: f64 = (locals.var_k2 * locals.var_q_temp2);
        let assign11420_e10757: f64 = (locals.var_k1 + assign11420_e10756);
        let assign11420_e10759: f64 = (assign11420_e10757 * locals.var_diff_min);
        let assign11420_e10761: f64 = (assign11420_e10759 / locals.var_a0);
        let assign11420_e10762: f64 = (assign11420_e10761).ln();
        let assign11420_e10764: f64 = assign11420_e10762;
        let assign11420_e10766: f64 = (assign11420_e10764 + 3.0);
        locals.var_q_x1sat = assign11420_e10766;
        locals.var_q_x1sat_dn4 = (((((((locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn4))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761);
        locals.var_q_x1sat_dn6 = (((((((locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn6))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761);
        locals.var_q_x1sat_dn7 = (((((((locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn7))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761);
        locals.var_q_x1sat_dn8 = (((((((locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn8))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761);
        locals.var_q_x1sat_dn9 = (((((((locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn9))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761);

        let assign11430_e10770: f64 = (locals.var_k1 * locals.var_q_temp1);
        let assign11430_e10771: f64 = (locals.var_k2 + assign11430_e10770);
        let assign11430_e10773: f64 = (assign11430_e10771 * locals.var_diff_min);
        let assign11430_e10775: f64 = (assign11430_e10773 / locals.var_a0);
        let assign11430_e10776: f64 = (assign11430_e10775).ln();
        let assign11430_e10778: f64 = assign11430_e10776;
        let assign11430_e10780: f64 = (assign11430_e10778 + 3.0);
        locals.var_q_x2sat = assign11430_e10780;
        locals.var_q_x2sat_dn4 = (((((((locals.var_k2_dn4 + ((locals.var_k1_dn4 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn4))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775);
        locals.var_q_x2sat_dn6 = (((((((locals.var_k2_dn6 + ((locals.var_k1_dn6 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn6))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775);
        locals.var_q_x2sat_dn7 = (((((((locals.var_k2_dn7 + ((locals.var_k1_dn7 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn7))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775);
        locals.var_q_x2sat_dn8 = (((((((locals.var_k2_dn8 + ((locals.var_k1_dn8 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn8))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775);
        locals.var_q_x2sat_dn9 = (((((((locals.var_k2_dn9 + ((locals.var_k1_dn9 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn9))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775);

        let assign11440_e10783: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign11440_e10785: f64 = (assign11440_e10783 * 0.3333333333333);
        let assign11440_e10787: f64 = if assign11440_e10785 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign11440_e10787;

        let (assign11450_e10799, assign11450_e10799_d_n4, assign11450_e10799_d_n6, assign11450_e10799_d_n7, assign11450_e10799_d_n8, assign11450_e10799_d_n9,) = {
    if (locals.var_guard548 != 0.0) {
        let assign11450_e10792: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign11450_e10794: f64 = (assign11450_e10792 * 0.3333333333333);
        let assign11450_e10795: f64 = (assign11450_e10794).exp();
        let assign11450_e10796: f64 = (1.0 + assign11450_e10795);
        let assign11450_e10797: f64 = (assign11450_e10796).ln();
        (assign11450_e10797, ((assign11450_e10795 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333)) / assign11450_e10796), ((assign11450_e10795 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333)) / assign11450_e10796), ((assign11450_e10795 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333)) / assign11450_e10796), ((assign11450_e10795 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333)) / assign11450_e10796), ((assign11450_e10795 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333)) / assign11450_e10796),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11450_e10799;
        locals.var_q_temp3_dn4 = assign11450_e10799_d_n4;
        locals.var_q_temp3_dn6 = assign11450_e10799_d_n6;
        locals.var_q_temp3_dn7 = assign11450_e10799_d_n7;
        locals.var_q_temp3_dn8 = assign11450_e10799_d_n8;
        locals.var_q_temp3_dn9 = assign11450_e10799_d_n9;

        let (assign11460_e10808, assign11460_e10808_d_n4, assign11460_e10808_d_n6, assign11460_e10808_d_n7, assign11460_e10808_d_n8, assign11460_e10808_d_n9,) = {
    if (locals.var_guard548 == 0.0) {
        let assign11460_e10804: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign11460_e10806: f64 = (assign11460_e10804 * 0.3333333333333);
        (assign11460_e10806, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11460_e10808;
        locals.var_q_temp3_dn4 = assign11460_e10808_d_n4;
        locals.var_q_temp3_dn6 = assign11460_e10808_d_n6;
        locals.var_q_temp3_dn7 = assign11460_e10808_d_n7;
        locals.var_q_temp3_dn8 = assign11460_e10808_d_n8;
        locals.var_q_temp3_dn9 = assign11460_e10808_d_n9;

        let assign11470_e10812: f64 = (3.0 * locals.var_q_temp3);
        let assign11470_e10813: f64 = (locals.var_q_x1sat - assign11470_e10812);
        locals.var_q_x1 = assign11470_e10813;
        locals.var_q_x1_dn4 = (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x1_dn6 = (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x1_dn7 = (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x1_dn8 = (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x1_dn9 = (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign11480_e10816: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign11480_e10818: f64 = (assign11480_e10816 * 0.3333333333333);
        let assign11480_e10820: f64 = if assign11480_e10818 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign11480_e10820;

        let (assign11490_e10832, assign11490_e10832_d_n4, assign11490_e10832_d_n6, assign11490_e10832_d_n7, assign11490_e10832_d_n8, assign11490_e10832_d_n9,) = {
    if (locals.var_guard549 != 0.0) {
        let assign11490_e10825: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign11490_e10827: f64 = (assign11490_e10825 * 0.3333333333333);
        let assign11490_e10828: f64 = (assign11490_e10827).exp();
        let assign11490_e10829: f64 = (1.0 + assign11490_e10828);
        let assign11490_e10830: f64 = (assign11490_e10829).ln();
        (assign11490_e10830, ((assign11490_e10828 * ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333)) / assign11490_e10829), ((assign11490_e10828 * ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333)) / assign11490_e10829), ((assign11490_e10828 * ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333)) / assign11490_e10829), ((assign11490_e10828 * ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333)) / assign11490_e10829), ((assign11490_e10828 * ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333)) / assign11490_e10829),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11490_e10832;
        locals.var_q_temp3_dn4 = assign11490_e10832_d_n4;
        locals.var_q_temp3_dn6 = assign11490_e10832_d_n6;
        locals.var_q_temp3_dn7 = assign11490_e10832_d_n7;
        locals.var_q_temp3_dn8 = assign11490_e10832_d_n8;
        locals.var_q_temp3_dn9 = assign11490_e10832_d_n9;

        let (assign11500_e10841, assign11500_e10841_d_n4, assign11500_e10841_d_n6, assign11500_e10841_d_n7, assign11500_e10841_d_n8, assign11500_e10841_d_n9,) = {
    if (locals.var_guard549 == 0.0) {
        let assign11500_e10837: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign11500_e10839: f64 = (assign11500_e10837 * 0.3333333333333);
        (assign11500_e10839, ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11500_e10841;
        locals.var_q_temp3_dn4 = assign11500_e10841_d_n4;
        locals.var_q_temp3_dn6 = assign11500_e10841_d_n6;
        locals.var_q_temp3_dn7 = assign11500_e10841_d_n7;
        locals.var_q_temp3_dn8 = assign11500_e10841_d_n8;
        locals.var_q_temp3_dn9 = assign11500_e10841_d_n9;

        let assign11510_e10845: f64 = (3.0 * locals.var_q_temp3);
        let assign11510_e10846: f64 = (locals.var_q_x2sat - assign11510_e10845);
        locals.var_q_x2 = assign11510_e10846;
        locals.var_q_x2_dn4 = (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x2_dn6 = (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x2_dn7 = (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x2_dn8 = (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x2_dn9 = (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign11520_e10849: f64 = (locals.var_k1 * locals.var_xg1x);
        let assign11520_e10851: f64 = (assign11520_e10849 + locals.var_q_x2);
        let assign11520_e10853: f64 = (assign11520_e10851 * locals.var_q_temp1);
        locals.var_q_x1_wi = assign11520_e10853;
        locals.var_q_x1_wi_dn4 = (((((locals.var_k1_dn4 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn4)) + locals.var_q_x2_dn4) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn4));
        locals.var_q_x1_wi_dn6 = (((((locals.var_k1_dn6 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn6)) + locals.var_q_x2_dn6) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn6));
        locals.var_q_x1_wi_dn7 = (((((locals.var_k1_dn7 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn7)) + locals.var_q_x2_dn7) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn7));
        locals.var_q_x1_wi_dn8 = (((((locals.var_k1_dn8 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn8)) + locals.var_q_x2_dn8) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn8));
        locals.var_q_x1_wi_dn9 = (((((locals.var_k1_dn9 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn9)) + locals.var_q_x2_dn9) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn9));

        let assign11530_e10856: f64 = (locals.var_k2 * locals.var_xg2x);
        let assign11530_e10858: f64 = (assign11530_e10856 + locals.var_q_x1);
        let assign11530_e10860: f64 = (assign11530_e10858 * locals.var_q_temp2);
        locals.var_q_x2_wi = assign11530_e10860;
        locals.var_q_x2_wi_dn4 = (((((locals.var_k2_dn4 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn4)) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn4));
        locals.var_q_x2_wi_dn6 = (((((locals.var_k2_dn6 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn6)) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn6));
        locals.var_q_x2_wi_dn7 = (((((locals.var_k2_dn7 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn7)) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn7));
        locals.var_q_x2_wi_dn8 = (((((locals.var_k2_dn8 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn8)) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn8));
        locals.var_q_x2_wi_dn9 = (((((locals.var_k2_dn9 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn9)) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn9));

        let assign11540_e10863: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign11540_e10865: f64 = (assign11540_e10863 * 0.3333333333333);
        let assign11540_e10867: f64 = if assign11540_e10865 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign11540_e10867;

        let (assign11550_e10879, assign11550_e10879_d_n4, assign11550_e10879_d_n6, assign11550_e10879_d_n7, assign11550_e10879_d_n8, assign11550_e10879_d_n9,) = {
    if (locals.var_guard550 != 0.0) {
        let assign11550_e10872: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign11550_e10874: f64 = (assign11550_e10872 * 0.3333333333333);
        let assign11550_e10875: f64 = (assign11550_e10874).exp();
        let assign11550_e10876: f64 = (1.0 + assign11550_e10875);
        let assign11550_e10877: f64 = (assign11550_e10876).ln();
        (assign11550_e10877, ((assign11550_e10875 * ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333)) / assign11550_e10876), ((assign11550_e10875 * ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333)) / assign11550_e10876), ((assign11550_e10875 * ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333)) / assign11550_e10876), ((assign11550_e10875 * ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333)) / assign11550_e10876), ((assign11550_e10875 * ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333)) / assign11550_e10876),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11550_e10879;
        locals.var_q_temp3_dn4 = assign11550_e10879_d_n4;
        locals.var_q_temp3_dn6 = assign11550_e10879_d_n6;
        locals.var_q_temp3_dn7 = assign11550_e10879_d_n7;
        locals.var_q_temp3_dn8 = assign11550_e10879_d_n8;
        locals.var_q_temp3_dn9 = assign11550_e10879_d_n9;

        let (assign11560_e10888, assign11560_e10888_d_n4, assign11560_e10888_d_n6, assign11560_e10888_d_n7, assign11560_e10888_d_n8, assign11560_e10888_d_n9,) = {
    if (locals.var_guard550 == 0.0) {
        let assign11560_e10884: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign11560_e10886: f64 = (assign11560_e10884 * 0.3333333333333);
        (assign11560_e10886, ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11560_e10888;
        locals.var_q_temp3_dn4 = assign11560_e10888_d_n4;
        locals.var_q_temp3_dn6 = assign11560_e10888_d_n6;
        locals.var_q_temp3_dn7 = assign11560_e10888_d_n7;
        locals.var_q_temp3_dn8 = assign11560_e10888_d_n8;
        locals.var_q_temp3_dn9 = assign11560_e10888_d_n9;

        let assign11570_e10892: f64 = (3.0 * locals.var_q_temp3);
        let assign11570_e10893: f64 = (locals.var_q_x1sat - assign11570_e10892);
        locals.var_q_x1 = assign11570_e10893;
        locals.var_q_x1_dn4 = (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x1_dn6 = (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x1_dn7 = (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x1_dn8 = (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x1_dn9 = (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign11580_e10896: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign11580_e10898: f64 = (assign11580_e10896 * 0.3333333333333);
        let assign11580_e10900: f64 = if assign11580_e10898 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign11580_e10900;

        let (assign11590_e10912, assign11590_e10912_d_n4, assign11590_e10912_d_n6, assign11590_e10912_d_n7, assign11590_e10912_d_n8, assign11590_e10912_d_n9,) = {
    if (locals.var_guard551 != 0.0) {
        let assign11590_e10905: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign11590_e10907: f64 = (assign11590_e10905 * 0.3333333333333);
        let assign11590_e10908: f64 = (assign11590_e10907).exp();
        let assign11590_e10909: f64 = (1.0 + assign11590_e10908);
        let assign11590_e10910: f64 = (assign11590_e10909).ln();
        (assign11590_e10910, ((assign11590_e10908 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333)) / assign11590_e10909), ((assign11590_e10908 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333)) / assign11590_e10909), ((assign11590_e10908 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333)) / assign11590_e10909), ((assign11590_e10908 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333)) / assign11590_e10909), ((assign11590_e10908 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333)) / assign11590_e10909),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11590_e10912;
        locals.var_q_temp3_dn4 = assign11590_e10912_d_n4;
        locals.var_q_temp3_dn6 = assign11590_e10912_d_n6;
        locals.var_q_temp3_dn7 = assign11590_e10912_d_n7;
        locals.var_q_temp3_dn8 = assign11590_e10912_d_n8;
        locals.var_q_temp3_dn9 = assign11590_e10912_d_n9;

        let (assign11600_e10921, assign11600_e10921_d_n4, assign11600_e10921_d_n6, assign11600_e10921_d_n7, assign11600_e10921_d_n8, assign11600_e10921_d_n9,) = {
    if (locals.var_guard551 == 0.0) {
        let assign11600_e10917: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign11600_e10919: f64 = (assign11600_e10917 * 0.3333333333333);
        (assign11600_e10919, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11600_e10921;
        locals.var_q_temp3_dn4 = assign11600_e10921_d_n4;
        locals.var_q_temp3_dn6 = assign11600_e10921_d_n6;
        locals.var_q_temp3_dn7 = assign11600_e10921_d_n7;
        locals.var_q_temp3_dn8 = assign11600_e10921_d_n8;
        locals.var_q_temp3_dn9 = assign11600_e10921_d_n9;

        let assign11610_e10925: f64 = (3.0 * locals.var_q_temp3);
        let assign11610_e10926: f64 = (locals.var_q_x2sat - assign11610_e10925);
        locals.var_q_x2 = assign11610_e10926;
        locals.var_q_x2_dn4 = (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x2_dn6 = (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x2_dn7 = (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x2_dn8 = (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x2_dn9 = (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign11620_e10929: f64 = (locals.var_xg1x - locals.var_q_x1);
        locals.var_q1s = assign11620_e10929;
        locals.var_q1s_dn4 = (locals.var_xg1x_dn4 - locals.var_q_x1_dn4);
        locals.var_q1s_dn6 = (locals.var_xg1x_dn6 - locals.var_q_x1_dn6);
        locals.var_q1s_dn7 = (locals.var_xg1x_dn7 - locals.var_q_x1_dn7);
        locals.var_q1s_dn8 = (locals.var_xg1x_dn8 - locals.var_q_x1_dn8);
        locals.var_q1s_dn9 = (locals.var_xg1x_dn9 - locals.var_q_x1_dn9);

        let assign11630_e10932: f64 = (locals.var_xg2x - locals.var_q_x2);
        locals.var_q2s = assign11630_e10932;
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

        let assign11660_e10937: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign11660_e10937;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign11670_e10940: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11670_e10942: f64 = assign11670_e10940;
        let assign11670_e10944: f64 = if assign11670_e10942 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign11670_e10944;

        let (assign11680_e10953, assign11680_e10953_d_n4, assign11680_e10953_d_n6, assign11680_e10953_d_n7, assign11680_e10953_d_n8, assign11680_e10953_d_n9,) = {
    if (locals.var_guard552 != 0.0) {
        let assign11680_e10948: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11680_e10950: f64 = assign11680_e10948;
        let assign11680_e10951: f64 = (assign11680_e10950).exp();
        (assign11680_e10951, (assign11680_e10951 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign11680_e10951 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign11680_e10951 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign11680_e10951 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign11680_e10951 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11680_e10953;
        locals.var_q_temp1_dn4 = assign11680_e10953_d_n4;
        locals.var_q_temp1_dn6 = assign11680_e10953_d_n6;
        locals.var_q_temp1_dn7 = assign11680_e10953_d_n7;
        locals.var_q_temp1_dn8 = assign11680_e10953_d_n8;
        locals.var_q_temp1_dn9 = assign11680_e10953_d_n9;

        let (assign11690_e10992, assign11690_e10992_d_n4, assign11690_e10992_d_n6, assign11690_e10992_d_n7, assign11690_e10992_d_n8, assign11690_e10992_d_n9,) = {
    if (locals.var_guard552 == 0.0) {
        let assign11690_e10960: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11690_e10962: f64 = assign11690_e10960;
        let assign11690_e10964: f64 = (assign11690_e10962 - 80.0);
        let assign11690_e10969: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11690_e10971: f64 = assign11690_e10969;
        let assign11690_e10973: f64 = (assign11690_e10971 - 80.0);
        let assign11690_e10974: f64 = (0.5 * assign11690_e10973);
        let assign11690_e10978: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11690_e10980: f64 = assign11690_e10978;
        let assign11690_e10982: f64 = (assign11690_e10980 - 80.0);
        let assign11690_e10984: f64 = (assign11690_e10982 * 0.3333333333333);
        let assign11690_e10985: f64 = (1.0 + assign11690_e10984);
        let assign11690_e10986: f64 = (assign11690_e10974 * assign11690_e10985);
        let assign11690_e10987: f64 = (1.0 + assign11690_e10986);
        let assign11690_e10988: f64 = (assign11690_e10964 * assign11690_e10987);
        let assign11690_e10989: f64 = (1.0 + assign11690_e10988);
        let assign11690_e10990: f64 = (5.54062e34 * assign11690_e10989);
        (assign11690_e10990, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11690_e10992;
        locals.var_q_temp1_dn4 = assign11690_e10992_d_n4;
        locals.var_q_temp1_dn6 = assign11690_e10992_d_n6;
        locals.var_q_temp1_dn7 = assign11690_e10992_d_n7;
        locals.var_q_temp1_dn8 = assign11690_e10992_d_n8;
        locals.var_q_temp1_dn9 = assign11690_e10992_d_n9;

        let assign11700_e10995: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign11700_e10995;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign11710_e10998: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign11710_e11000: f64 = (assign11710_e10998 - locals.var_q_aexp);
        locals.var_q_qsq = assign11710_e11000;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

    }

    pub(super) fn stamp_transient_block_27(
        locals: &mut StampLocals,
    ) {
        let assign11720_e11003: f64 = (2.0 * locals.var_k1);
        let assign11720_e11005: f64 = (assign11720_e11003 * locals.var_q_k1q1);
        let assign11720_e11007: f64 = (assign11720_e11005 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign11720_e11007;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign11730_e11010: f64 = (2.0 * locals.var_k1);
        let assign11730_e11012: f64 = (assign11730_e11010 * locals.var_k1);
        let assign11730_e11014: f64 = (assign11730_e11012 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign11730_e11014;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign11740_e11017: f64 = (-0.005);
        let assign11740_e11018: f64 = if locals.var_q_qsq < assign11740_e11017 { 1.0 } else { 0.0 };
        locals.var_guard553 = assign11740_e11018;

        let (assign11750_e11024, assign11750_e11024_d_n4, assign11750_e11024_d_n6, assign11750_e11024_d_n7, assign11750_e11024_d_n8, assign11750_e11024_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11750_e11021: f64 = (locals.var_q_qsq).abs();
        let assign11750_e11022: f64 = (assign11750_e11021).sqrt();
        (assign11750_e11022, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign11750_e11022)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign11750_e11022)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign11750_e11022)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign11750_e11022)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign11750_e11022)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign11750_e11024;
        locals.var_q_rac_qsq_dn4 = assign11750_e11024_d_n4;
        locals.var_q_rac_qsq_dn6 = assign11750_e11024_d_n6;
        locals.var_q_rac_qsq_dn7 = assign11750_e11024_d_n7;
        locals.var_q_rac_qsq_dn8 = assign11750_e11024_d_n8;
        locals.var_q_rac_qsq_dn9 = assign11750_e11024_d_n9;

        let (assign11760_e11033, assign11760_e11033_d_n4, assign11760_e11033_d_n6, assign11760_e11033_d_n7, assign11760_e11033_d_n8, assign11760_e11033_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11760_e11029: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign11760_e11030: f64 = (assign11760_e11029).tan();
        let assign11760_e11031: f64 = (locals.var_q_rac_qsq / assign11760_e11030);
        (assign11760_e11031, (((locals.var_q_rac_qsq_dn4 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), (((locals.var_q_rac_qsq_dn6 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), (((locals.var_q_rac_qsq_dn7 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), (((locals.var_q_rac_qsq_dn8 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), (((locals.var_q_rac_qsq_dn9 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign11760_e11033;
        locals.var_q_qcoth_dn4 = assign11760_e11033_d_n4;
        locals.var_q_qcoth_dn6 = assign11760_e11033_d_n6;
        locals.var_q_qcoth_dn7 = assign11760_e11033_d_n7;
        locals.var_q_qcoth_dn8 = assign11760_e11033_d_n8;
        locals.var_q_qcoth_dn9 = assign11760_e11033_d_n9;

        let (assign11770_e11041, assign11770_e11041_d_n4, assign11770_e11041_d_n6, assign11770_e11041_d_n7, assign11770_e11041_d_n8, assign11770_e11041_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11770_e11037: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign11770_e11039: f64 = (assign11770_e11037 / locals.var_q_qsq);
        (assign11770_e11039, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11770_e11041;
        locals.var_q_temp1_dn4 = assign11770_e11041_d_n4;
        locals.var_q_temp1_dn6 = assign11770_e11041_d_n6;
        locals.var_q_temp1_dn7 = assign11770_e11041_d_n7;
        locals.var_q_temp1_dn8 = assign11770_e11041_d_n8;
        locals.var_q_temp1_dn9 = assign11770_e11041_d_n9;

        let (assign11780_e11053, assign11780_e11053_d_n4, assign11780_e11053_d_n6, assign11780_e11053_d_n7, assign11780_e11053_d_n8, assign11780_e11053_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11780_e11047: f64 = (2.0 - locals.var_q_qcoth);
        let assign11780_e11048: f64 = (locals.var_q_qcoth * assign11780_e11047);
        let assign11780_e11049: f64 = (locals.var_q_qsq + assign11780_e11048);
        let assign11780_e11051: f64 = (assign11780_e11049 * locals.var_q_temp1);
        (assign11780_e11051, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign11780_e11053;
        locals.var_q_d1_qcoth_dn4 = assign11780_e11053_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign11780_e11053_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign11780_e11053_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign11780_e11053_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign11780_e11053_d_n9;

        let (assign11790_e11073, assign11790_e11073_d_n4, assign11790_e11073_d_n6, assign11790_e11073_d_n7, assign11790_e11073_d_n8, assign11790_e11073_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11790_e11058: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign11790_e11061: f64 = (1.0 + locals.var_q_qcoth);
        let assign11790_e11062: f64 = (assign11790_e11058 * assign11790_e11061);
        let assign11790_e11063: f64 = (locals.var_q_d1_qsq - assign11790_e11062);
        let assign11790_e11065: f64 = (assign11790_e11063 * locals.var_q_temp1);
        let assign11790_e11068: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign11790_e11070: f64 = (assign11790_e11068 / locals.var_q_d1_qsq);
        let assign11790_e11071: f64 = (assign11790_e11065 + assign11790_e11070);
        (assign11790_e11071, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign11790_e11073;
        locals.var_q_d2_qcoth_dn4 = assign11790_e11073_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign11790_e11073_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign11790_e11073_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign11790_e11073_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign11790_e11073_d_n9;

        let (assign11800_e11081, assign11800_e11081_d_n4, assign11800_e11081_d_n6, assign11800_e11081_d_n7, assign11800_e11081_d_n8, assign11800_e11081_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11800_e11078: f64 = (0.5 * locals.var_q_qcoth);
        let assign11800_e11079: f64 = (1.0 - assign11800_e11078);
        (assign11800_e11079, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign11800_e11081;
        locals.var_q_temp2_dn4 = assign11800_e11081_d_n4;
        locals.var_q_temp2_dn6 = assign11800_e11081_d_n6;
        locals.var_q_temp2_dn7 = assign11800_e11081_d_n7;
        locals.var_q_temp2_dn8 = assign11800_e11081_d_n8;
        locals.var_q_temp2_dn9 = assign11800_e11081_d_n9;

        let (assign11810_e11089, assign11810_e11089_d_n4, assign11810_e11089_d_n6, assign11810_e11089_d_n7, assign11810_e11089_d_n8, assign11810_e11089_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11810_e11085: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign11810_e11087: f64 = (assign11810_e11085 * locals.var_q_temp2);
        (assign11810_e11087, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign11810_e11089;
        locals.var_q_d1_ln_dn4 = assign11810_e11089_d_n4;
        locals.var_q_d1_ln_dn6 = assign11810_e11089_d_n6;
        locals.var_q_d1_ln_dn7 = assign11810_e11089_d_n7;
        locals.var_q_d1_ln_dn8 = assign11810_e11089_d_n8;
        locals.var_q_d1_ln_dn9 = assign11810_e11089_d_n9;

        let (assign11820_e11105, assign11820_e11105_d_n4, assign11820_e11105_d_n6, assign11820_e11105_d_n7, assign11820_e11105_d_n8, assign11820_e11105_d_n9,) = {
    if (locals.var_guard553 != 0.0) {
        let assign11820_e11093: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign11820_e11098: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign11820_e11099: f64 = (locals.var_q_d1_ln + assign11820_e11098);
        let assign11820_e11100: f64 = (locals.var_q_d1_qsq * assign11820_e11099);
        let assign11820_e11101: f64 = (assign11820_e11093 - assign11820_e11100);
        let assign11820_e11103: f64 = (assign11820_e11101 / locals.var_q_qsq);
        (assign11820_e11103, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign11820_e11105;
        locals.var_q_d2_ln_dn4 = assign11820_e11105_d_n4;
        locals.var_q_d2_ln_dn6 = assign11820_e11105_d_n6;
        locals.var_q_d2_ln_dn7 = assign11820_e11105_d_n7;
        locals.var_q_d2_ln_dn8 = assign11820_e11105_d_n8;
        locals.var_q_d2_ln_dn9 = assign11820_e11105_d_n9;

        let assign11830_e11108: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard554 = assign11830_e11108;

        let (assign11840_e11117, assign11840_e11117_d_n4, assign11840_e11117_d_n6, assign11840_e11117_d_n7, assign11840_e11117_d_n8, assign11840_e11117_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11840_e11114: f64 = (locals.var_q_qsq).abs();
        let assign11840_e11115: f64 = (assign11840_e11114).sqrt();
        (assign11840_e11115, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign11840_e11115)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign11840_e11115)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign11840_e11115)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign11840_e11115)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign11840_e11115)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign11840_e11117;
        locals.var_q_rac_qsq_dn4 = assign11840_e11117_d_n4;
        locals.var_q_rac_qsq_dn6 = assign11840_e11117_d_n6;
        locals.var_q_rac_qsq_dn7 = assign11840_e11117_d_n7;
        locals.var_q_rac_qsq_dn8 = assign11840_e11117_d_n8;
        locals.var_q_rac_qsq_dn9 = assign11840_e11117_d_n9;

        let (assign11850_e11126, assign11850_e11126_d_n4, assign11850_e11126_d_n6, assign11850_e11126_d_n7, assign11850_e11126_d_n8, assign11850_e11126_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11850_e11123: f64 = (-locals.var_q_rac_qsq);
        let assign11850_e11124: f64 = (assign11850_e11123).exp();
        (assign11850_e11124, (assign11850_e11124 * (-locals.var_q_rac_qsq_dn4)), (assign11850_e11124 * (-locals.var_q_rac_qsq_dn6)), (assign11850_e11124 * (-locals.var_q_rac_qsq_dn7)), (assign11850_e11124 * (-locals.var_q_rac_qsq_dn8)), (assign11850_e11124 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign11850_e11126;
        locals.var_q_invexpq_dn4 = assign11850_e11126_d_n4;
        locals.var_q_invexpq_dn6 = assign11850_e11126_d_n6;
        locals.var_q_invexpq_dn7 = assign11850_e11126_d_n7;
        locals.var_q_invexpq_dn8 = assign11850_e11126_d_n8;
        locals.var_q_invexpq_dn9 = assign11850_e11126_d_n9;

        let (assign11860_e11141, assign11860_e11141_d_n4, assign11860_e11141_d_n6, assign11860_e11141_d_n7, assign11860_e11141_d_n8, assign11860_e11141_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11860_e11134: f64 = (1.0 + locals.var_q_invexpq);
        let assign11860_e11135: f64 = (locals.var_q_rac_qsq * assign11860_e11134);
        let assign11860_e11138: f64 = (1.0 - locals.var_q_invexpq);
        let assign11860_e11139: f64 = (assign11860_e11135 / assign11860_e11138);
        (assign11860_e11139, (((((locals.var_q_rac_qsq_dn4 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn4))) / (assign11860_e11138 * assign11860_e11138)), (((((locals.var_q_rac_qsq_dn6 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn6))) / (assign11860_e11138 * assign11860_e11138)), (((((locals.var_q_rac_qsq_dn7 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn7))) / (assign11860_e11138 * assign11860_e11138)), (((((locals.var_q_rac_qsq_dn8 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn8))) / (assign11860_e11138 * assign11860_e11138)), (((((locals.var_q_rac_qsq_dn9 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn9))) / (assign11860_e11138 * assign11860_e11138)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign11860_e11141;
        locals.var_q_qcoth_dn4 = assign11860_e11141_d_n4;
        locals.var_q_qcoth_dn6 = assign11860_e11141_d_n6;
        locals.var_q_qcoth_dn7 = assign11860_e11141_d_n7;
        locals.var_q_qcoth_dn8 = assign11860_e11141_d_n8;
        locals.var_q_qcoth_dn9 = assign11860_e11141_d_n9;

        let (assign11870_e11152, assign11870_e11152_d_n4, assign11870_e11152_d_n6, assign11870_e11152_d_n7, assign11870_e11152_d_n8, assign11870_e11152_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11870_e11148: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign11870_e11150: f64 = (assign11870_e11148 / locals.var_q_qsq);
        (assign11870_e11150, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11870_e11152;
        locals.var_q_temp1_dn4 = assign11870_e11152_d_n4;
        locals.var_q_temp1_dn6 = assign11870_e11152_d_n6;
        locals.var_q_temp1_dn7 = assign11870_e11152_d_n7;
        locals.var_q_temp1_dn8 = assign11870_e11152_d_n8;
        locals.var_q_temp1_dn9 = assign11870_e11152_d_n9;

        let (assign11880_e11167, assign11880_e11167_d_n4, assign11880_e11167_d_n6, assign11880_e11167_d_n7, assign11880_e11167_d_n8, assign11880_e11167_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11880_e11161: f64 = (2.0 - locals.var_q_qcoth);
        let assign11880_e11162: f64 = (locals.var_q_qcoth * assign11880_e11161);
        let assign11880_e11163: f64 = (locals.var_q_qsq + assign11880_e11162);
        let assign11880_e11165: f64 = (assign11880_e11163 * locals.var_q_temp1);
        (assign11880_e11165, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign11880_e11167;
        locals.var_q_d1_qcoth_dn4 = assign11880_e11167_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign11880_e11167_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign11880_e11167_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign11880_e11167_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign11880_e11167_d_n9;

        let (assign11890_e11190, assign11890_e11190_d_n4, assign11890_e11190_d_n6, assign11890_e11190_d_n7, assign11890_e11190_d_n8, assign11890_e11190_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11890_e11175: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign11890_e11178: f64 = (1.0 + locals.var_q_qcoth);
        let assign11890_e11179: f64 = (assign11890_e11175 * assign11890_e11178);
        let assign11890_e11180: f64 = (locals.var_q_d1_qsq - assign11890_e11179);
        let assign11890_e11182: f64 = (assign11890_e11180 * locals.var_q_temp1);
        let assign11890_e11185: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign11890_e11187: f64 = (assign11890_e11185 / locals.var_q_d1_qsq);
        let assign11890_e11188: f64 = (assign11890_e11182 + assign11890_e11187);
        (assign11890_e11188, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign11890_e11190;
        locals.var_q_d2_qcoth_dn4 = assign11890_e11190_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign11890_e11190_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign11890_e11190_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign11890_e11190_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign11890_e11190_d_n9;

        let (assign11900_e11201, assign11900_e11201_d_n4, assign11900_e11201_d_n6, assign11900_e11201_d_n7, assign11900_e11201_d_n8, assign11900_e11201_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11900_e11198: f64 = (0.5 * locals.var_q_qcoth);
        let assign11900_e11199: f64 = (1.0 - assign11900_e11198);
        (assign11900_e11199, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign11900_e11201;
        locals.var_q_temp2_dn4 = assign11900_e11201_d_n4;
        locals.var_q_temp2_dn6 = assign11900_e11201_d_n6;
        locals.var_q_temp2_dn7 = assign11900_e11201_d_n7;
        locals.var_q_temp2_dn8 = assign11900_e11201_d_n8;
        locals.var_q_temp2_dn9 = assign11900_e11201_d_n9;

        let (assign11910_e11212, assign11910_e11212_d_n4, assign11910_e11212_d_n6, assign11910_e11212_d_n7, assign11910_e11212_d_n8, assign11910_e11212_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11910_e11208: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign11910_e11210: f64 = (assign11910_e11208 * locals.var_q_temp2);
        (assign11910_e11210, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign11910_e11212;
        locals.var_q_d1_ln_dn4 = assign11910_e11212_d_n4;
        locals.var_q_d1_ln_dn6 = assign11910_e11212_d_n6;
        locals.var_q_d1_ln_dn7 = assign11910_e11212_d_n7;
        locals.var_q_d1_ln_dn8 = assign11910_e11212_d_n8;
        locals.var_q_d1_ln_dn9 = assign11910_e11212_d_n9;

        let (assign11920_e11231, assign11920_e11231_d_n4, assign11920_e11231_d_n6, assign11920_e11231_d_n7, assign11920_e11231_d_n8, assign11920_e11231_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
        let assign11920_e11219: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign11920_e11224: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign11920_e11225: f64 = (locals.var_q_d1_ln + assign11920_e11224);
        let assign11920_e11226: f64 = (locals.var_q_d1_qsq * assign11920_e11225);
        let assign11920_e11227: f64 = (assign11920_e11219 - assign11920_e11226);
        let assign11920_e11229: f64 = (assign11920_e11227 / locals.var_q_qsq);
        (assign11920_e11229, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign11920_e11231;
        locals.var_q_d2_ln_dn4 = assign11920_e11231_d_n4;
        locals.var_q_d2_ln_dn6 = assign11920_e11231_d_n6;
        locals.var_q_d2_ln_dn7 = assign11920_e11231_d_n7;
        locals.var_q_d2_ln_dn8 = assign11920_e11231_d_n8;
        locals.var_q_d2_ln_dn9 = assign11920_e11231_d_n9;

        let (assign11930_e11257, assign11930_e11257_d_n4, assign11930_e11257_d_n6, assign11930_e11257_d_n7, assign11930_e11257_d_n8, assign11930_e11257_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11930_e11241: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign11930_e11245: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign11930_e11249: f64 = (locals.var_q_qsq * 0.025);
        let assign11930_e11250: f64 = (1.0 - assign11930_e11249);
        let assign11930_e11251: f64 = (assign11930_e11245 * assign11930_e11250);
        let assign11930_e11252: f64 = (1.0 - assign11930_e11251);
        let assign11930_e11253: f64 = (assign11930_e11241 * assign11930_e11252);
        let assign11930_e11254: f64 = (1.0 - assign11930_e11253);
        let assign11930_e11255: f64 = (0.1666666666667 * assign11930_e11254);
        (assign11930_e11255, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign11930_e11257;
        locals.var_q_temp3_dn4 = assign11930_e11257_d_n4;
        locals.var_q_temp3_dn6 = assign11930_e11257_d_n6;
        locals.var_q_temp3_dn7 = assign11930_e11257_d_n7;
        locals.var_q_temp3_dn8 = assign11930_e11257_d_n8;
        locals.var_q_temp3_dn9 = assign11930_e11257_d_n9;

        let (assign11940_e11269, assign11940_e11269_d_n4, assign11940_e11269_d_n6, assign11940_e11269_d_n7, assign11940_e11269_d_n8, assign11940_e11269_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11940_e11266: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign11940_e11267: f64 = (2.0 + assign11940_e11266);
        (assign11940_e11267, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign11940_e11269;
        locals.var_q_qcoth_dn4 = assign11940_e11269_d_n4;
        locals.var_q_qcoth_dn6 = assign11940_e11269_d_n6;
        locals.var_q_qcoth_dn7 = assign11940_e11269_d_n7;
        locals.var_q_qcoth_dn8 = assign11940_e11269_d_n8;
        locals.var_q_qcoth_dn9 = assign11940_e11269_d_n9;

        let (assign11950_e11295, assign11950_e11295_d_n4, assign11950_e11295_d_n6, assign11950_e11295_d_n7, assign11950_e11295_d_n8, assign11950_e11295_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11950_e11279: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign11950_e11283: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign11950_e11287: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign11950_e11288: f64 = (1.0 - assign11950_e11287);
        let assign11950_e11289: f64 = (assign11950_e11283 * assign11950_e11288);
        let assign11950_e11290: f64 = (1.0 - assign11950_e11289);
        let assign11950_e11291: f64 = (assign11950_e11279 * assign11950_e11290);
        let assign11950_e11292: f64 = (1.0 - assign11950_e11291);
        let assign11950_e11293: f64 = (0.1666666666667 * assign11950_e11292);
        (assign11950_e11293, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign11950_e11295;
        locals.var_q_temp1_dn4 = assign11950_e11295_d_n4;
        locals.var_q_temp1_dn6 = assign11950_e11295_d_n6;
        locals.var_q_temp1_dn7 = assign11950_e11295_d_n7;
        locals.var_q_temp1_dn8 = assign11950_e11295_d_n8;
        locals.var_q_temp1_dn9 = assign11950_e11295_d_n9;

        let (assign11960_e11305, assign11960_e11305_d_n4, assign11960_e11305_d_n6, assign11960_e11305_d_n7, assign11960_e11305_d_n8, assign11960_e11305_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11960_e11303: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign11960_e11303, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign11960_e11305;
        locals.var_q_d1_qcoth_dn4 = assign11960_e11305_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign11960_e11305_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign11960_e11305_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign11960_e11305_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign11960_e11305_d_n9;

        let (assign11970_e11331, assign11970_e11331_d_n4, assign11970_e11331_d_n6, assign11970_e11331_d_n7, assign11970_e11331_d_n8, assign11970_e11331_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11970_e11315: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign11970_e11319: f64 = (0.05 * locals.var_q_qsq);
        let assign11970_e11323: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign11970_e11324: f64 = (1.0 - assign11970_e11323);
        let assign11970_e11325: f64 = (assign11970_e11319 * assign11970_e11324);
        let assign11970_e11326: f64 = (1.0 - assign11970_e11325);
        let assign11970_e11327: f64 = (assign11970_e11315 * assign11970_e11326);
        let assign11970_e11328: f64 = (1.0 - assign11970_e11327);
        let assign11970_e11329: f64 = (0.0055555555556 * assign11970_e11328);
        (assign11970_e11329, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn4) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn6) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn7) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn8) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn9) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign11970_e11331;
        locals.var_q_temp2_dn4 = assign11970_e11331_d_n4;
        locals.var_q_temp2_dn6 = assign11970_e11331_d_n6;
        locals.var_q_temp2_dn7 = assign11970_e11331_d_n7;
        locals.var_q_temp2_dn8 = assign11970_e11331_d_n8;
        locals.var_q_temp2_dn9 = assign11970_e11331_d_n9;

        let (assign11980_e11347, assign11980_e11347_d_n4, assign11980_e11347_d_n6, assign11980_e11347_d_n7, assign11980_e11347_d_n8, assign11980_e11347_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11980_e11339: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign11980_e11342: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign11980_e11344: f64 = (assign11980_e11342 * locals.var_q_temp2);
        let assign11980_e11345: f64 = (assign11980_e11339 - assign11980_e11344);
        (assign11980_e11345, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign11980_e11347;
        locals.var_q_d2_qcoth_dn4 = assign11980_e11347_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign11980_e11347_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign11980_e11347_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign11980_e11347_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign11980_e11347_d_n9;

        let (assign11990_e11360, assign11990_e11360_d_n4, assign11990_e11360_d_n6, assign11990_e11360_d_n7, assign11990_e11360_d_n8, assign11990_e11360_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign11990_e11354: f64 = (-0.5);
        let assign11990_e11356: f64 = (assign11990_e11354 * locals.var_q_d1_qsq);
        let assign11990_e11358: f64 = (assign11990_e11356 * locals.var_q_temp3);
        (assign11990_e11358, (((assign11990_e11354 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn4)), (((assign11990_e11354 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn6)), (((assign11990_e11354 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn7)), (((assign11990_e11354 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn8)), (((assign11990_e11354 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign11990_e11360;
        locals.var_q_d1_ln_dn4 = assign11990_e11360_d_n4;
        locals.var_q_d1_ln_dn6 = assign11990_e11360_d_n6;
        locals.var_q_d1_ln_dn7 = assign11990_e11360_d_n7;
        locals.var_q_d1_ln_dn8 = assign11990_e11360_d_n8;
        locals.var_q_d1_ln_dn9 = assign11990_e11360_d_n9;

        let (assign12000_e11393, assign12000_e11393_d_n4, assign12000_e11393_d_n6, assign12000_e11393_d_n7, assign12000_e11393_d_n8, assign12000_e11393_d_n9,) = {
    if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
        let assign12000_e11367: f64 = (-0.5);
        let assign12000_e11369: f64 = (assign12000_e11367 * locals.var_q_d2_qsq);
        let assign12000_e11371: f64 = (assign12000_e11369 * locals.var_q_temp3);
        let assign12000_e11374: f64 = (0.25 * 0.0055555555556);
        let assign12000_e11376: f64 = (assign12000_e11374 * locals.var_q_d1_qsq);
        let assign12000_e11378: f64 = (assign12000_e11376 * locals.var_q_d1_qsq);
        let assign12000_e11382: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign12000_e11386: f64 = (0.075 * locals.var_q_qsq);
        let assign12000_e11387: f64 = (2.0 - assign12000_e11386);
        let assign12000_e11388: f64 = (assign12000_e11382 * assign12000_e11387);
        let assign12000_e11389: f64 = (1.0 - assign12000_e11388);
        let assign12000_e11390: f64 = (assign12000_e11378 * assign12000_e11389);
        let assign12000_e11391: f64 = (assign12000_e11371 + assign12000_e11390);
        (assign12000_e11391, ((((assign12000_e11367 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn4)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn4)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign12000_e11367 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn6)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn6)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign12000_e11367 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn7)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn7)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign12000_e11367 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn8)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn8)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign12000_e11367 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn9)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn9)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign12000_e11393;
        locals.var_q_d2_ln_dn4 = assign12000_e11393_d_n4;
        locals.var_q_d2_ln_dn6 = assign12000_e11393_d_n6;
        locals.var_q_d2_ln_dn7 = assign12000_e11393_d_n7;
        locals.var_q_d2_ln_dn8 = assign12000_e11393_d_n8;
        locals.var_q_d2_ln_dn9 = assign12000_e11393_d_n9;

        let assign12010_e11396: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard555 = assign12010_e11396;

        let (assign12020_e11410, assign12020_e11410_d_n4, assign12020_e11410_d_n6, assign12020_e11410_d_n7, assign12020_e11410_d_n8, assign12020_e11410_d_n9,) = {
    if (locals.var_guard555 != 0.0) {
        let assign12020_e11400: f64 = (4.0 * locals.var_q_qsq);
        let assign12020_e11405: f64 = (2.0 - locals.var_q_invexpq);
        let assign12020_e11406: f64 = (locals.var_q_invexpq * assign12020_e11405);
        let assign12020_e11407: f64 = (1.0 - assign12020_e11406);
        let assign12020_e11408: f64 = (assign12020_e11400 / assign12020_e11407);
        (assign12020_e11408, ((((4.0 * locals.var_q_qsq_dn4) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn4 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign12020_e11407 * assign12020_e11407)), ((((4.0 * locals.var_q_qsq_dn6) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn6 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign12020_e11407 * assign12020_e11407)), ((((4.0 * locals.var_q_qsq_dn7) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn7 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign12020_e11407 * assign12020_e11407)), ((((4.0 * locals.var_q_qsq_dn8) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn8 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign12020_e11407 * assign12020_e11407)), ((((4.0 * locals.var_q_qsq_dn9) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn9 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign12020_e11407 * assign12020_e11407)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign12020_e11410;
        locals.var_q_temp2_dn4 = assign12020_e11410_d_n4;
        locals.var_q_temp2_dn6 = assign12020_e11410_d_n6;
        locals.var_q_temp2_dn7 = assign12020_e11410_d_n7;
        locals.var_q_temp2_dn8 = assign12020_e11410_d_n8;
        locals.var_q_temp2_dn9 = assign12020_e11410_d_n9;

    }

    pub(super) fn stamp_transient_block_28(
        locals: &mut StampLocals,
    ) {
        let (assign12030_e11416, assign12030_e11416_d_n4, assign12030_e11416_d_n6, assign12030_e11416_d_n7, assign12030_e11416_d_n8, assign12030_e11416_d_n9,) = {
    if (locals.var_guard555 != 0.0) {
        let assign12030_e11414: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign12030_e11414, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign12030_e11416;
        locals.var_q_sh_term_dn4 = assign12030_e11416_d_n4;
        locals.var_q_sh_term_dn6 = assign12030_e11416_d_n6;
        locals.var_q_sh_term_dn7 = assign12030_e11416_d_n7;
        locals.var_q_sh_term_dn8 = assign12030_e11416_d_n8;
        locals.var_q_sh_term_dn9 = assign12030_e11416_d_n9;

        let (assign12040_e11423, assign12040_e11423_d_n4, assign12040_e11423_d_n6, assign12040_e11423_d_n7, assign12040_e11423_d_n8, assign12040_e11423_d_n9,) = {
    if (locals.var_guard555 != 0.0) {
        let assign12040_e11419: f64 = (locals.var_q_temp2).ln();
        let assign12040_e11421: f64 = (assign12040_e11419 - locals.var_q_rac_qsq);
        (assign12040_e11421, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign12040_e11423;
        locals.var_q_ln_term_dn4 = assign12040_e11423_d_n4;
        locals.var_q_ln_term_dn6 = assign12040_e11423_d_n6;
        locals.var_q_ln_term_dn7 = assign12040_e11423_d_n7;
        locals.var_q_ln_term_dn8 = assign12040_e11423_d_n8;
        locals.var_q_ln_term_dn9 = assign12040_e11423_d_n9;

        let assign12050_e11426: f64 = (-0.005);
        let assign12050_e11427: f64 = if locals.var_q_qsq < assign12050_e11426 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign12050_e11427;

        let (assign12060_e11437, assign12060_e11437_d_n4, assign12060_e11437_d_n6, assign12060_e11437_d_n7, assign12060_e11437_d_n8, assign12060_e11437_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
        let assign12060_e11434: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign12060_e11435: f64 = (assign12060_e11434).sin();
        (assign12060_e11435, ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign12060_e11437;
        locals.var_q_temp2_dn4 = assign12060_e11437_d_n4;
        locals.var_q_temp2_dn6 = assign12060_e11437_d_n6;
        locals.var_q_temp2_dn7 = assign12060_e11437_d_n7;
        locals.var_q_temp2_dn8 = assign12060_e11437_d_n8;
        locals.var_q_temp2_dn9 = assign12060_e11437_d_n9;

        let (assign12070_e11449, assign12070_e11449_d_n4, assign12070_e11449_d_n6, assign12070_e11449_d_n7, assign12070_e11449_d_n8, assign12070_e11449_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
        let assign12070_e11443: f64 = (-locals.var_q_qsq);
        let assign12070_e11446: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign12070_e11447: f64 = (assign12070_e11443 / assign12070_e11446);
        (assign12070_e11447, ((((-locals.var_q_qsq_dn4) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign12070_e11446 * assign12070_e11446)), ((((-locals.var_q_qsq_dn6) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign12070_e11446 * assign12070_e11446)), ((((-locals.var_q_qsq_dn7) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign12070_e11446 * assign12070_e11446)), ((((-locals.var_q_qsq_dn8) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign12070_e11446 * assign12070_e11446)), ((((-locals.var_q_qsq_dn9) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign12070_e11446 * assign12070_e11446)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign12070_e11449;
        locals.var_q_sh_term_dn4 = assign12070_e11449_d_n4;
        locals.var_q_sh_term_dn6 = assign12070_e11449_d_n6;
        locals.var_q_sh_term_dn7 = assign12070_e11449_d_n7;
        locals.var_q_sh_term_dn8 = assign12070_e11449_d_n8;
        locals.var_q_sh_term_dn9 = assign12070_e11449_d_n9;

        let (assign12080_e11457, assign12080_e11457_d_n4, assign12080_e11457_d_n6, assign12080_e11457_d_n7, assign12080_e11457_d_n8, assign12080_e11457_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
        let assign12080_e11455: f64 = (locals.var_q_sh_term).ln();
        (assign12080_e11455, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign12080_e11457;
        locals.var_q_ln_term_dn4 = assign12080_e11457_d_n4;
        locals.var_q_ln_term_dn6 = assign12080_e11457_d_n6;
        locals.var_q_ln_term_dn7 = assign12080_e11457_d_n7;
        locals.var_q_ln_term_dn8 = assign12080_e11457_d_n8;
        locals.var_q_ln_term_dn9 = assign12080_e11457_d_n9;

        let (assign12090_e11481, assign12090_e11481_d_n4, assign12090_e11481_d_n6, assign12090_e11481_d_n7, assign12090_e11481_d_n8, assign12090_e11481_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 == 0.0)) {
        let assign12090_e11466: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign12090_e11470: f64 = (0.05 * locals.var_q_qsq);
        let assign12090_e11474: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign12090_e11475: f64 = (1.0 - assign12090_e11474);
        let assign12090_e11476: f64 = (assign12090_e11470 * assign12090_e11475);
        let assign12090_e11477: f64 = (1.0 - assign12090_e11476);
        let assign12090_e11478: f64 = (assign12090_e11466 * assign12090_e11477);
        let assign12090_e11479: f64 = (4.0 - assign12090_e11478);
        (assign12090_e11479, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn4) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn6) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn7) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn8) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn9) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign12090_e11481;
        locals.var_q_sh_term_dn4 = assign12090_e11481_d_n4;
        locals.var_q_sh_term_dn6 = assign12090_e11481_d_n6;
        locals.var_q_sh_term_dn7 = assign12090_e11481_d_n7;
        locals.var_q_sh_term_dn8 = assign12090_e11481_d_n8;
        locals.var_q_sh_term_dn9 = assign12090_e11481_d_n9;

        let (assign12100_e11490, assign12100_e11490_d_n4, assign12100_e11490_d_n6, assign12100_e11490_d_n7, assign12100_e11490_d_n8, assign12100_e11490_d_n9,) = {
    if ((locals.var_guard555 == 0.0) && (locals.var_guard556 == 0.0)) {
        let assign12100_e11488: f64 = (locals.var_q_sh_term).ln();
        (assign12100_e11488, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign12100_e11490;
        locals.var_q_ln_term_dn4 = assign12100_e11490_d_n4;
        locals.var_q_ln_term_dn6 = assign12100_e11490_d_n6;
        locals.var_q_ln_term_dn7 = assign12100_e11490_d_n7;
        locals.var_q_ln_term_dn8 = assign12100_e11490_d_n8;
        locals.var_q_ln_term_dn9 = assign12100_e11490_d_n9;

        let assign12110_e11493: f64 = (1.01 * locals.var_q_k1q1);
        let assign12110_e11495: f64 = (assign12110_e11493 + locals.var_q_qcoth);
        let assign12110_e11497: f64 = if assign12110_e11495 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign12110_e11497;

        let (assign12120_e11503, assign12120_e11503_d_n4, assign12120_e11503_d_n6, assign12120_e11503_d_n7, assign12120_e11503_d_n8, assign12120_e11503_d_n9,) = {
    if (locals.var_guard557 != 0.0) {
        let assign12120_e11501: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign12120_e11501, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign12120_e11503;
        locals.var_q_expnum_dn4 = assign12120_e11503_d_n4;
        locals.var_q_expnum_dn6 = assign12120_e11503_d_n6;
        locals.var_q_expnum_dn7 = assign12120_e11503_d_n7;
        locals.var_q_expnum_dn8 = assign12120_e11503_d_n8;
        locals.var_q_expnum_dn9 = assign12120_e11503_d_n9;

        let (assign12130_e11509, assign12130_e11509_d_n4, assign12130_e11509_d_n6, assign12130_e11509_d_n7, assign12130_e11509_d_n8, assign12130_e11509_d_n9,) = {
    if (locals.var_guard557 != 0.0) {
        let assign12130_e11507: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign12130_e11507, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign12130_e11509;
        locals.var_q_d1_expnum_dn4 = assign12130_e11509_d_n4;
        locals.var_q_d1_expnum_dn6 = assign12130_e11509_d_n6;
        locals.var_q_d1_expnum_dn7 = assign12130_e11509_d_n7;
        locals.var_q_d1_expnum_dn8 = assign12130_e11509_d_n8;
        locals.var_q_d1_expnum_dn9 = assign12130_e11509_d_n9;

        let (assign12140_e11513, assign12140_e11513_d_n4, assign12140_e11513_d_n6, assign12140_e11513_d_n7, assign12140_e11513_d_n8, assign12140_e11513_d_n9,) = {
    if (locals.var_guard557 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign12140_e11513;
        locals.var_q_d2_expnum_dn4 = assign12140_e11513_d_n4;
        locals.var_q_d2_expnum_dn6 = assign12140_e11513_d_n6;
        locals.var_q_d2_expnum_dn7 = assign12140_e11513_d_n7;
        locals.var_q_d2_expnum_dn8 = assign12140_e11513_d_n8;
        locals.var_q_d2_expnum_dn9 = assign12140_e11513_d_n9;

        let (assign12150_e11522, assign12150_e11522_d_n4, assign12150_e11522_d_n6, assign12150_e11522_d_n7, assign12150_e11522_d_n8, assign12150_e11522_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12150_e11519: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign12150_e11520: f64 = (1.0 / assign12150_e11519);
        (assign12150_e11520, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign12150_e11519 * assign12150_e11519))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign12150_e11519 * assign12150_e11519))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign12150_e11519 * assign12150_e11519))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign12150_e11519 * assign12150_e11519))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign12150_e11519 * assign12150_e11519))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign12150_e11522;
        locals.var_q_temp2_dn4 = assign12150_e11522_d_n4;
        locals.var_q_temp2_dn6 = assign12150_e11522_d_n6;
        locals.var_q_temp2_dn7 = assign12150_e11522_d_n7;
        locals.var_q_temp2_dn8 = assign12150_e11522_d_n8;
        locals.var_q_temp2_dn9 = assign12150_e11522_d_n9;

        let (assign12160_e11529, assign12160_e11529_d_n4, assign12160_e11529_d_n6, assign12160_e11529_d_n7, assign12160_e11529_d_n8, assign12160_e11529_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12160_e11527: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign12160_e11527, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign12160_e11529;
        locals.var_q_temp3_dn4 = assign12160_e11529_d_n4;
        locals.var_q_temp3_dn6 = assign12160_e11529_d_n6;
        locals.var_q_temp3_dn7 = assign12160_e11529_d_n7;
        locals.var_q_temp3_dn8 = assign12160_e11529_d_n8;
        locals.var_q_temp3_dn9 = assign12160_e11529_d_n9;

        let (assign12170_e11538, assign12170_e11538_d_n4, assign12170_e11538_d_n6, assign12170_e11538_d_n7, assign12170_e11538_d_n8, assign12170_e11538_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12170_e11534: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign12170_e11536: f64 = (assign12170_e11534 * locals.var_q_temp2);
        (assign12170_e11536, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign12170_e11538;
        locals.var_q_expnum_dn4 = assign12170_e11538_d_n4;
        locals.var_q_expnum_dn6 = assign12170_e11538_d_n6;
        locals.var_q_expnum_dn7 = assign12170_e11538_d_n7;
        locals.var_q_expnum_dn8 = assign12170_e11538_d_n8;
        locals.var_q_expnum_dn9 = assign12170_e11538_d_n9;

        let (assign12180_e11553, assign12180_e11553_d_n4, assign12180_e11553_d_n6, assign12180_e11553_d_n7, assign12180_e11553_d_n8, assign12180_e11553_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12180_e11543: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign12180_e11545: f64 = (assign12180_e11543 - locals.var_q_aexp);
        let assign12180_e11548: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign12180_e11549: f64 = (assign12180_e11545 - assign12180_e11548);
        let assign12180_e11551: f64 = (assign12180_e11549 * locals.var_q_temp2);
        (assign12180_e11551, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign12180_e11553;
        locals.var_q_d1_expnum_dn4 = assign12180_e11553_d_n4;
        locals.var_q_d1_expnum_dn6 = assign12180_e11553_d_n6;
        locals.var_q_d1_expnum_dn7 = assign12180_e11553_d_n7;
        locals.var_q_d1_expnum_dn8 = assign12180_e11553_d_n8;
        locals.var_q_d1_expnum_dn9 = assign12180_e11553_d_n9;

        let (assign12190_e11578, assign12190_e11578_d_n4, assign12190_e11578_d_n6, assign12190_e11578_d_n7, assign12190_e11578_d_n8, assign12190_e11578_d_n9,) = {
    if (locals.var_guard557 == 0.0) {
        let assign12190_e11558: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign12190_e11561: f64 = (2.0 * locals.var_q_temp3);
        let assign12190_e11563: f64 = (assign12190_e11561 * locals.var_q_d1_expnum);
        let assign12190_e11564: f64 = (assign12190_e11558 + assign12190_e11563);
        let assign12190_e11566: f64 = (assign12190_e11564 + locals.var_q_aexp);
        let assign12190_e11570: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign12190_e11571: f64 = (locals.var_q_d2_ln + assign12190_e11570);
        let assign12190_e11573: f64 = (assign12190_e11571 * locals.var_q_sh_term);
        let assign12190_e11574: f64 = (assign12190_e11566 - assign12190_e11573);
        let assign12190_e11576: f64 = (assign12190_e11574 * locals.var_q_temp2);
        (assign12190_e11576, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign12190_e11578;
        locals.var_q_d2_expnum_dn4 = assign12190_e11578_d_n4;
        locals.var_q_d2_expnum_dn6 = assign12190_e11578_d_n6;
        locals.var_q_d2_expnum_dn7 = assign12190_e11578_d_n7;
        locals.var_q_d2_expnum_dn8 = assign12190_e11578_d_n8;
        locals.var_q_d2_expnum_dn9 = assign12190_e11578_d_n9;

        let assign12200_e11581: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard558 = assign12200_e11581;

        let (assign12210_e11586, assign12210_e11586_d_n4, assign12210_e11586_d_n6, assign12210_e11586_d_n7, assign12210_e11586_d_n8, assign12210_e11586_d_n9,) = {
    if (locals.var_guard558 != 0.0) {
        let assign12210_e11584: f64 = (locals.var_q_expnum).ln();
        (assign12210_e11584, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign12210_e11586;
        locals.var_q_lnexpnum_dn4 = assign12210_e11586_d_n4;
        locals.var_q_lnexpnum_dn6 = assign12210_e11586_d_n6;
        locals.var_q_lnexpnum_dn7 = assign12210_e11586_d_n7;
        locals.var_q_lnexpnum_dn8 = assign12210_e11586_d_n8;
        locals.var_q_lnexpnum_dn9 = assign12210_e11586_d_n9;

        let (assign12220_e11592, assign12220_e11592_d_n4, assign12220_e11592_d_n6, assign12220_e11592_d_n7, assign12220_e11592_d_n8, assign12220_e11592_d_n9,) = {
    if (locals.var_guard558 != 0.0) {
        let assign12220_e11590: f64 = (1.0 / locals.var_q_expnum);
        (assign12220_e11590, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12220_e11592;
        locals.var_q_temp1_dn4 = assign12220_e11592_d_n4;
        locals.var_q_temp1_dn6 = assign12220_e11592_d_n6;
        locals.var_q_temp1_dn7 = assign12220_e11592_d_n7;
        locals.var_q_temp1_dn8 = assign12220_e11592_d_n8;
        locals.var_q_temp1_dn9 = assign12220_e11592_d_n9;

        let (assign12230_e11598, assign12230_e11598_d_n4, assign12230_e11598_d_n6, assign12230_e11598_d_n7, assign12230_e11598_d_n8, assign12230_e11598_d_n9,) = {
    if (locals.var_guard558 != 0.0) {
        let assign12230_e11596: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign12230_e11596, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign12230_e11598;
        locals.var_q_d1_lnexpnum_dn4 = assign12230_e11598_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign12230_e11598_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign12230_e11598_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign12230_e11598_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign12230_e11598_d_n9;

        let (assign12240_e11608, assign12240_e11608_d_n4, assign12240_e11608_d_n6, assign12240_e11608_d_n7, assign12240_e11608_d_n8, assign12240_e11608_d_n9,) = {
    if (locals.var_guard558 != 0.0) {
        let assign12240_e11602: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign12240_e11605: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign12240_e11606: f64 = (assign12240_e11602 - assign12240_e11605);
        (assign12240_e11606, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign12240_e11608;
        locals.var_q_d2_lnexpnum_dn4 = assign12240_e11608_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign12240_e11608_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign12240_e11608_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign12240_e11608_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign12240_e11608_d_n9;

        let (assign12250_e11619, assign12250_e11619_d_n4, assign12250_e11619_d_n6, assign12250_e11619_d_n7, assign12250_e11619_d_n8, assign12250_e11619_d_n9,) = {
    if (locals.var_guard558 == 0.0) {
        let assign12250_e11613: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign12250_e11615: f64 = (-locals.var_q_k1q1);
        let assign12250_e11616: f64 = (assign12250_e11615).ln();
        let assign12250_e11617: f64 = (assign12250_e11613 + assign12250_e11616);
        (assign12250_e11617, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign12250_e11615)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign12250_e11615)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign12250_e11615)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign12250_e11615)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign12250_e11615)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign12250_e11619;
        locals.var_q_lnexpnum_dn4 = assign12250_e11619_d_n4;
        locals.var_q_lnexpnum_dn6 = assign12250_e11619_d_n6;
        locals.var_q_lnexpnum_dn7 = assign12250_e11619_d_n7;
        locals.var_q_lnexpnum_dn8 = assign12250_e11619_d_n8;
        locals.var_q_lnexpnum_dn9 = assign12250_e11619_d_n9;

        let (assign12260_e11626, assign12260_e11626_d_n4, assign12260_e11626_d_n6, assign12260_e11626_d_n7, assign12260_e11626_d_n8, assign12260_e11626_d_n9,) = {
    if (locals.var_guard558 == 0.0) {
        let assign12260_e11624: f64 = (1.0 / locals.var_q1s);
        (assign12260_e11624, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12260_e11626;
        locals.var_q_temp1_dn4 = assign12260_e11626_d_n4;
        locals.var_q_temp1_dn6 = assign12260_e11626_d_n6;
        locals.var_q_temp1_dn7 = assign12260_e11626_d_n7;
        locals.var_q_temp1_dn8 = assign12260_e11626_d_n8;
        locals.var_q_temp1_dn9 = assign12260_e11626_d_n9;

        let (assign12270_e11633, assign12270_e11633_d_n4, assign12270_e11633_d_n6, assign12270_e11633_d_n7, assign12270_e11633_d_n8, assign12270_e11633_d_n9,) = {
    if (locals.var_guard558 == 0.0) {
        let assign12270_e11631: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign12270_e11631, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign12270_e11633;
        locals.var_q_d1_lnexpnum_dn4 = assign12270_e11633_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign12270_e11633_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign12270_e11633_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign12270_e11633_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign12270_e11633_d_n9;

        let (assign12280_e11641, assign12280_e11641_d_n4, assign12280_e11641_d_n6, assign12280_e11641_d_n7, assign12280_e11641_d_n8, assign12280_e11641_d_n9,) = {
    if (locals.var_guard558 == 0.0) {
        let assign12280_e11637: f64 = (-locals.var_q_temp1);
        let assign12280_e11639: f64 = (assign12280_e11637 * locals.var_q_temp1);
        (assign12280_e11639, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign12280_e11641;
        locals.var_q_d2_lnexpnum_dn4 = assign12280_e11641_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign12280_e11641_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign12280_e11641_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign12280_e11641_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign12280_e11641_d_n9;

        let assign12290_e11644: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign12290_e11646: f64 = (assign12290_e11644 + locals.var_q1s);
        let assign12290_e11649: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign12290_e11650: f64 = (assign12290_e11646 + assign12290_e11649);
        let assign12290_e11652: f64 = (assign12290_e11650 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign12290_e11652;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign12300_e11656: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign12300_e11657: f64 = (1.0 + assign12300_e11656);
        let assign12300_e11659: f64 = (assign12300_e11657 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign12300_e11659;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign12310_e11662: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign12310_e11664: f64 = (assign12310_e11662 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign12310_e11664;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign12320_e11668: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign12320_e11669: f64 = (locals.var_q_k1q1 + assign12320_e11668);
        locals.var_q_qi_int = assign12320_e11669;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign12330_e11673: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign12330_e11674: f64 = (locals.var_k1 + assign12330_e11673);
        locals.var_q_d1_qi = assign12330_e11674;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign12340_e11677: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign12340_e11677;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign12350_e11680: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign12350_e11682: f64 = (assign12350_e11680 - locals.var_q_aexp);
        locals.var_q_zero = assign12350_e11682;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign12360_e11685: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign12360_e11688: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign12360_e11689: f64 = (assign12360_e11685 + assign12360_e11688);
        let assign12360_e11691: f64 = (assign12360_e11689 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign12360_e11691;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign12370_e11694: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign12370_e11697: f64 = (2.0 * locals.var_q_d1_qi);
        let assign12370_e11699: f64 = (assign12370_e11697 * locals.var_q_d1_expnum);
        let assign12370_e11700: f64 = (assign12370_e11694 + assign12370_e11699);
        let assign12370_e11703: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign12370_e11704: f64 = (assign12370_e11700 + assign12370_e11703);
        let assign12370_e11706: f64 = (assign12370_e11704 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign12370_e11706;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign12380_e11709: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign12380_e11712: f64 = (0.5 * locals.var_q_zero);
        let assign12380_e11714: f64 = (assign12380_e11712 * locals.var_q_d2_zero);
        let assign12380_e11715: f64 = (assign12380_e11709 - assign12380_e11714);
        locals.var_q_temp = assign12380_e11715;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn9)));

        let assign12390_e11717: f64 = (-locals.var_q_zero);
        let assign12390_e11719: f64 = (assign12390_e11717 * locals.var_q_d1_zero);
        let assign12390_e11721: f64 = (assign12390_e11719 * locals.var_q_temp);
        let assign12390_e11724: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign12390_e11726: f64 = (assign12390_e11724 + 1e-200);
        let assign12390_e11727: f64 = (assign12390_e11721 / assign12390_e11726);
        locals.var_q_eps2 = assign12390_e11727;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn4)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign12390_e11726 * assign12390_e11726));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn6)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign12390_e11726 * assign12390_e11726));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn7)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign12390_e11726 * assign12390_e11726));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn8)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign12390_e11726 * assign12390_e11726));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn9)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign12390_e11726 * assign12390_e11726));

        let assign12400_e11730: f64 = (locals.var_q1s + locals.var_q_eps2);
        locals.var_q1s = assign12400_e11730;
        locals.var_q1s_dn4 = (locals.var_q1s_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1s_dn6 = (locals.var_q1s_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1s_dn7 = (locals.var_q1s_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1s_dn8 = (locals.var_q1s_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1s_dn9 = (locals.var_q1s_dn9 + locals.var_q_eps2_dn9);

    }

    pub(super) fn stamp_transient_block_29(
        locals: &mut StampLocals,
    ) {
        let assign12410_e11733: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign12410_e11733;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign12420_e11736: f64 = (locals.var_k2 * locals.var_q2s);
        locals.var_q_k2q2 = assign12420_e11736;
        locals.var_q_k2q2_dn4 = ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4));
        locals.var_q_k2q2_dn6 = ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6));
        locals.var_q_k2q2_dn7 = ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7));
        locals.var_q_k2q2_dn8 = ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8));
        locals.var_q_k2q2_dn9 = ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9));

        let assign12430_e11739: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        locals.var_q_qi_int = assign12430_e11739;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4);
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6);
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7);
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8);
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9);

        let assign12440_e11743: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign12440_e11744: f64 = (1.0 + assign12440_e11743);
        locals.var_q_a = assign12440_e11744;
        locals.var_q_a_dn4 = (0.065345483024 * locals.var_q_qi_int_dn4);
        locals.var_q_a_dn6 = (0.065345483024 * locals.var_q_qi_int_dn6);
        locals.var_q_a_dn7 = (0.065345483024 * locals.var_q_qi_int_dn7);
        locals.var_q_a_dn8 = (0.065345483024 * locals.var_q_qi_int_dn8);
        locals.var_q_a_dn9 = (0.065345483024 * locals.var_q_qi_int_dn9);

        let assign12450_e11748: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign12450_e11749: f64 = (39.478417604 + assign12450_e11748);
        let assign12450_e11752: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12450_e11753: f64 = (assign12450_e11749 + assign12450_e11752);
        locals.var_q_b = assign12450_e11753;
        locals.var_q_b_dn4 = ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)));
        locals.var_q_b_dn6 = ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)));
        locals.var_q_b_dn7 = ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)));
        locals.var_q_b_dn8 = ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)));
        locals.var_q_b_dn9 = ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)));

        let assign12460_e11757: f64 = (2.0 * locals.var_q_qi_int);
        let assign12460_e11760: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12460_e11761: f64 = (assign12460_e11757 + assign12460_e11760);
        let assign12460_e11762: f64 = (39.478417604 * assign12460_e11761);
        locals.var_q_c = assign12460_e11762;
        locals.var_q_c_dn4 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))));
        locals.var_q_c_dn6 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))));
        locals.var_q_c_dn7 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))));
        locals.var_q_c_dn8 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))));
        locals.var_q_c_dn9 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))));

        let assign12470_e11765: f64 = (locals.var_q_b * locals.var_q_b);
        let assign12470_e11768: f64 = (4.0 * locals.var_q_a);
        let assign12470_e11770: f64 = (assign12470_e11768 * locals.var_q_c);
        let assign12470_e11771: f64 = (assign12470_e11765 - assign12470_e11770);
        let assign12470_e11772: f64 = (assign12470_e11771).sqrt();
        locals.var_q_disc = assign12470_e11772;
        locals.var_q_disc_dn4 = ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn4))) / (2.0 * assign12470_e11772));
        locals.var_q_disc_dn6 = ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn6))) / (2.0 * assign12470_e11772));
        locals.var_q_disc_dn7 = ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn7))) / (2.0 * assign12470_e11772));
        locals.var_q_disc_dn8 = ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn8))) / (2.0 * assign12470_e11772));
        locals.var_q_disc_dn9 = ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn9))) / (2.0 * assign12470_e11772));

        let assign12480_e11775: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign12480_e11778: f64 = (2.0 * locals.var_q_a);
        let assign12480_e11779: f64 = (assign12480_e11775 / assign12480_e11778);
        locals.var_q_qsq = assign12480_e11779;
        locals.var_q_qsq_dn4 = ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn4))) / (assign12480_e11778 * assign12480_e11778));
        locals.var_q_qsq_dn6 = ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn6))) / (assign12480_e11778 * assign12480_e11778));
        locals.var_q_qsq_dn7 = ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn7))) / (assign12480_e11778 * assign12480_e11778));
        locals.var_q_qsq_dn8 = ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn8))) / (assign12480_e11778 * assign12480_e11778));
        locals.var_q_qsq_dn9 = ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn9))) / (assign12480_e11778 * assign12480_e11778));

        let assign12490_e11782: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12490_e11784: f64 = (assign12490_e11782 - locals.var_q_qsq);
        locals.var_q_delta = assign12490_e11784;
        locals.var_q_delta_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4);
        locals.var_q_delta_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6);
        locals.var_q_delta_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7);
        locals.var_q_delta_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8);
        locals.var_q_delta_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9);

        let assign12500_e11787: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign12500_e11787;

        let (assign12510_e11802, assign12510_e11802_d_n4, assign12510_e11802_d_n6, assign12510_e11802_d_n7, assign12510_e11802_d_n8, assign12510_e11802_d_n9,) = {
    if (locals.var_guard559 != 0.0) {
        let assign12510_e11792: f64 = (locals.var_q_delta / locals.var_a0);
        let assign12510_e11793: f64 = (assign12510_e11792).ln();
        let assign12510_e11795: f64 = assign12510_e11793;
        let assign12510_e11797: f64 = (assign12510_e11795 - locals.var_xg1x);
        let assign12510_e11799: f64 = (assign12510_e11797 + locals.var_q1s);
        let assign12510_e11800: f64 = (locals.var_q_delta * assign12510_e11799);
        (assign12510_e11800, ((locals.var_q_delta_dn4 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn4) + locals.var_q1s_dn4))), ((locals.var_q_delta_dn6 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn6) + locals.var_q1s_dn6))), ((locals.var_q_delta_dn7 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn7) + locals.var_q1s_dn7))), ((locals.var_q_delta_dn8 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn8) + locals.var_q1s_dn8))), ((locals.var_q_delta_dn9 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn9) + locals.var_q1s_dn9))),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign12510_e11802;
        locals.var_q_zero_dn4 = assign12510_e11802_d_n4;
        locals.var_q_zero_dn6 = assign12510_e11802_d_n6;
        locals.var_q_zero_dn7 = assign12510_e11802_d_n7;
        locals.var_q_zero_dn8 = assign12510_e11802_d_n8;
        locals.var_q_zero_dn9 = assign12510_e11802_d_n9;

        let (assign12520_e11812, assign12520_e11812_d_n4, assign12520_e11812_d_n6, assign12520_e11812_d_n7, assign12520_e11812_d_n8, assign12520_e11812_d_n9,) = {
    if (locals.var_guard559 != 0.0) {
        let assign12520_e11806: f64 = (2.0 * locals.var_k1);
        let assign12520_e11808: f64 = (assign12520_e11806 * locals.var_q_k1q1);
        let assign12520_e11810: f64 = (assign12520_e11808 + locals.var_q_delta);
        (assign12520_e11810, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign12520_e11812;
        locals.var_q_d1_zero_dn4 = assign12520_e11812_d_n4;
        locals.var_q_d1_zero_dn6 = assign12520_e11812_d_n6;
        locals.var_q_d1_zero_dn7 = assign12520_e11812_d_n7;
        locals.var_q_d1_zero_dn8 = assign12520_e11812_d_n8;
        locals.var_q_d1_zero_dn9 = assign12520_e11812_d_n9;

        let (assign12530_e11820,) = {
    if (locals.var_guard559 != 0.0) {
        let assign12530_e11816: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12530_e11818: f64 = (assign12530_e11816 - locals.var_q_x1sat);
        (assign12530_e11818,)
    } else {
        (locals.var_q_dx1,)
    }
};
        locals.var_q_dx1 = assign12530_e11820;

        let assign12540_e11830: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign12540_e11832: f64 = (locals.var_k1).ln();
        let assign12540_e11833: f64 = (assign12540_e11830 + assign12540_e11832);
        let assign12540_e11840: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign12540_e11833 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard560 = assign12540_e11840;

        let (assign12550_e11850, assign12550_e11850_d_n4, assign12550_e11850_d_n6, assign12550_e11850_d_n7, assign12550_e11850_d_n8, assign12550_e11850_d_n9,) = {
    if ((locals.var_guard559 != 0.0) && (locals.var_guard560 != 0.0)) {
        let assign12550_e11847: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
        let assign12550_e11848: f64 = (locals.var_q1s - assign12550_e11847);
        (assign12550_e11848, (locals.var_q1s_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))),)
    } else {
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9,)
    }
};
        locals.var_q1s = assign12550_e11850;
        locals.var_q1s_dn4 = assign12550_e11850_d_n4;
        locals.var_q1s_dn6 = assign12550_e11850_d_n6;
        locals.var_q1s_dn7 = assign12550_e11850_d_n7;
        locals.var_q1s_dn8 = assign12550_e11850_d_n8;
        locals.var_q1s_dn9 = assign12550_e11850_d_n9;

        let assign12560_e11853: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign12560_e11853;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign12570_e11856: f64 = (locals.var_k2 * locals.var_q2s);
        locals.var_q_k2q2 = assign12570_e11856;
        locals.var_q_k2q2_dn4 = ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4));
        locals.var_q_k2q2_dn6 = ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6));
        locals.var_q_k2q2_dn7 = ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7));
        locals.var_q_k2q2_dn8 = ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8));
        locals.var_q_k2q2_dn9 = ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9));

        let assign12580_e11859: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        locals.var_q_qi_int = assign12580_e11859;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4);
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6);
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7);
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8);
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9);

        let assign12590_e11863: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign12590_e11864: f64 = (1.0 + assign12590_e11863);
        locals.var_q_a = assign12590_e11864;
        locals.var_q_a_dn4 = (0.065345483024 * locals.var_q_qi_int_dn4);
        locals.var_q_a_dn6 = (0.065345483024 * locals.var_q_qi_int_dn6);
        locals.var_q_a_dn7 = (0.065345483024 * locals.var_q_qi_int_dn7);
        locals.var_q_a_dn8 = (0.065345483024 * locals.var_q_qi_int_dn8);
        locals.var_q_a_dn9 = (0.065345483024 * locals.var_q_qi_int_dn9);

        let assign12600_e11868: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign12600_e11869: f64 = (39.478417604 + assign12600_e11868);
        let assign12600_e11872: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12600_e11873: f64 = (assign12600_e11869 + assign12600_e11872);
        locals.var_q_b = assign12600_e11873;
        locals.var_q_b_dn4 = ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)));
        locals.var_q_b_dn6 = ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)));
        locals.var_q_b_dn7 = ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)));
        locals.var_q_b_dn8 = ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)));
        locals.var_q_b_dn9 = ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)));

        let assign12610_e11877: f64 = (2.0 * locals.var_q_qi_int);
        let assign12610_e11880: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12610_e11881: f64 = (assign12610_e11877 + assign12610_e11880);
        let assign12610_e11882: f64 = (39.478417604 * assign12610_e11881);
        locals.var_q_c = assign12610_e11882;
        locals.var_q_c_dn4 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))));
        locals.var_q_c_dn6 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))));
        locals.var_q_c_dn7 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))));
        locals.var_q_c_dn8 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))));
        locals.var_q_c_dn9 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))));

        let assign12620_e11885: f64 = (locals.var_q_b * locals.var_q_b);
        let assign12620_e11888: f64 = (4.0 * locals.var_q_a);
        let assign12620_e11890: f64 = (assign12620_e11888 * locals.var_q_c);
        let assign12620_e11891: f64 = (assign12620_e11885 - assign12620_e11890);
        let assign12620_e11892: f64 = (assign12620_e11891).sqrt();
        locals.var_q_disc = assign12620_e11892;
        locals.var_q_disc_dn4 = ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn4))) / (2.0 * assign12620_e11892));
        locals.var_q_disc_dn6 = ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn6))) / (2.0 * assign12620_e11892));
        locals.var_q_disc_dn7 = ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn7))) / (2.0 * assign12620_e11892));
        locals.var_q_disc_dn8 = ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn8))) / (2.0 * assign12620_e11892));
        locals.var_q_disc_dn9 = ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn9))) / (2.0 * assign12620_e11892));

        let assign12630_e11895: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign12630_e11898: f64 = (2.0 * locals.var_q_a);
        let assign12630_e11899: f64 = (assign12630_e11895 / assign12630_e11898);
        locals.var_q_qsq = assign12630_e11899;
        locals.var_q_qsq_dn4 = ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn4))) / (assign12630_e11898 * assign12630_e11898));
        locals.var_q_qsq_dn6 = ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn6))) / (assign12630_e11898 * assign12630_e11898));
        locals.var_q_qsq_dn7 = ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn7))) / (assign12630_e11898 * assign12630_e11898));
        locals.var_q_qsq_dn8 = ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn8))) / (assign12630_e11898 * assign12630_e11898));
        locals.var_q_qsq_dn9 = ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn9))) / (assign12630_e11898 * assign12630_e11898));

        let assign12640_e11902: f64 = (-0.005);
        let assign12640_e11903: f64 = if locals.var_q_qsq < assign12640_e11902 { 1.0 } else { 0.0 };
        locals.var_guard561 = assign12640_e11903;

        let (assign12650_e11909, assign12650_e11909_d_n4, assign12650_e11909_d_n6, assign12650_e11909_d_n7, assign12650_e11909_d_n8, assign12650_e11909_d_n9,) = {
    if (locals.var_guard561 != 0.0) {
        let assign12650_e11906: f64 = (locals.var_q_qsq).abs();
        let assign12650_e11907: f64 = (assign12650_e11906).sqrt();
        (assign12650_e11907, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12650_e11907)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12650_e11907)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12650_e11907)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12650_e11907)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12650_e11907)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign12650_e11909;
        locals.var_q_rac_qsq_dn4 = assign12650_e11909_d_n4;
        locals.var_q_rac_qsq_dn6 = assign12650_e11909_d_n6;
        locals.var_q_rac_qsq_dn7 = assign12650_e11909_d_n7;
        locals.var_q_rac_qsq_dn8 = assign12650_e11909_d_n8;
        locals.var_q_rac_qsq_dn9 = assign12650_e11909_d_n9;

        let (assign12660_e11918, assign12660_e11918_d_n4, assign12660_e11918_d_n6, assign12660_e11918_d_n7, assign12660_e11918_d_n8, assign12660_e11918_d_n9,) = {
    if (locals.var_guard561 != 0.0) {
        let assign12660_e11914: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign12660_e11915: f64 = (assign12660_e11914).tan();
        let assign12660_e11916: f64 = (locals.var_q_rac_qsq / assign12660_e11915);
        (assign12660_e11916, (((locals.var_q_rac_qsq_dn4 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), (((locals.var_q_rac_qsq_dn6 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), (((locals.var_q_rac_qsq_dn7 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), (((locals.var_q_rac_qsq_dn8 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), (((locals.var_q_rac_qsq_dn9 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12660_e11918;
        locals.var_q_qcoth_dn4 = assign12660_e11918_d_n4;
        locals.var_q_qcoth_dn6 = assign12660_e11918_d_n6;
        locals.var_q_qcoth_dn7 = assign12660_e11918_d_n7;
        locals.var_q_qcoth_dn8 = assign12660_e11918_d_n8;
        locals.var_q_qcoth_dn9 = assign12660_e11918_d_n9;

        let (assign12670_e11932, assign12670_e11932_d_n4, assign12670_e11932_d_n6, assign12670_e11932_d_n7, assign12670_e11932_d_n8, assign12670_e11932_d_n9,) = {
    if (locals.var_guard561 != 0.0) {
        let assign12670_e11925: f64 = (2.0 - locals.var_q_qcoth);
        let assign12670_e11926: f64 = (locals.var_q_qcoth * assign12670_e11925);
        let assign12670_e11927: f64 = (locals.var_q_qsq + assign12670_e11926);
        let assign12670_e11928: f64 = (0.25 * assign12670_e11927);
        let assign12670_e11930: f64 = (assign12670_e11928 / locals.var_q_qsq);
        (assign12670_e11930, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12670_e11932;
        locals.var_q_d1_qcoth_dn4 = assign12670_e11932_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12670_e11932_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12670_e11932_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12670_e11932_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12670_e11932_d_n9;

        let assign12680_e11935: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard562 = assign12680_e11935;

        let (assign12690_e11944, assign12690_e11944_d_n4, assign12690_e11944_d_n6, assign12690_e11944_d_n7, assign12690_e11944_d_n8, assign12690_e11944_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
        let assign12690_e11941: f64 = (locals.var_q_qsq).abs();
        let assign12690_e11942: f64 = (assign12690_e11941).sqrt();
        (assign12690_e11942, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12690_e11942)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12690_e11942)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12690_e11942)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12690_e11942)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12690_e11942)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign12690_e11944;
        locals.var_q_rac_qsq_dn4 = assign12690_e11944_d_n4;
        locals.var_q_rac_qsq_dn6 = assign12690_e11944_d_n6;
        locals.var_q_rac_qsq_dn7 = assign12690_e11944_d_n7;
        locals.var_q_rac_qsq_dn8 = assign12690_e11944_d_n8;
        locals.var_q_rac_qsq_dn9 = assign12690_e11944_d_n9;

        let (assign12700_e11953, assign12700_e11953_d_n4, assign12700_e11953_d_n6, assign12700_e11953_d_n7, assign12700_e11953_d_n8, assign12700_e11953_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
        let assign12700_e11950: f64 = (-locals.var_q_rac_qsq);
        let assign12700_e11951: f64 = (assign12700_e11950).exp();
        (assign12700_e11951, (assign12700_e11951 * (-locals.var_q_rac_qsq_dn4)), (assign12700_e11951 * (-locals.var_q_rac_qsq_dn6)), (assign12700_e11951 * (-locals.var_q_rac_qsq_dn7)), (assign12700_e11951 * (-locals.var_q_rac_qsq_dn8)), (assign12700_e11951 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign12700_e11953;
        locals.var_q_invexpq_dn4 = assign12700_e11953_d_n4;
        locals.var_q_invexpq_dn6 = assign12700_e11953_d_n6;
        locals.var_q_invexpq_dn7 = assign12700_e11953_d_n7;
        locals.var_q_invexpq_dn8 = assign12700_e11953_d_n8;
        locals.var_q_invexpq_dn9 = assign12700_e11953_d_n9;

        let (assign12710_e11968, assign12710_e11968_d_n4, assign12710_e11968_d_n6, assign12710_e11968_d_n7, assign12710_e11968_d_n8, assign12710_e11968_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
        let assign12710_e11961: f64 = (1.0 + locals.var_q_invexpq);
        let assign12710_e11962: f64 = (locals.var_q_rac_qsq * assign12710_e11961);
        let assign12710_e11965: f64 = (1.0 - locals.var_q_invexpq);
        let assign12710_e11966: f64 = (assign12710_e11962 / assign12710_e11965);
        (assign12710_e11966, (((((locals.var_q_rac_qsq_dn4 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn4))) / (assign12710_e11965 * assign12710_e11965)), (((((locals.var_q_rac_qsq_dn6 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn6))) / (assign12710_e11965 * assign12710_e11965)), (((((locals.var_q_rac_qsq_dn7 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn7))) / (assign12710_e11965 * assign12710_e11965)), (((((locals.var_q_rac_qsq_dn8 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn8))) / (assign12710_e11965 * assign12710_e11965)), (((((locals.var_q_rac_qsq_dn9 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn9))) / (assign12710_e11965 * assign12710_e11965)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12710_e11968;
        locals.var_q_qcoth_dn4 = assign12710_e11968_d_n4;
        locals.var_q_qcoth_dn6 = assign12710_e11968_d_n6;
        locals.var_q_qcoth_dn7 = assign12710_e11968_d_n7;
        locals.var_q_qcoth_dn8 = assign12710_e11968_d_n8;
        locals.var_q_qcoth_dn9 = assign12710_e11968_d_n9;

        let (assign12720_e11985, assign12720_e11985_d_n4, assign12720_e11985_d_n6, assign12720_e11985_d_n7, assign12720_e11985_d_n8, assign12720_e11985_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
        let assign12720_e11978: f64 = (2.0 - locals.var_q_qcoth);
        let assign12720_e11979: f64 = (locals.var_q_qcoth * assign12720_e11978);
        let assign12720_e11980: f64 = (locals.var_q_qsq + assign12720_e11979);
        let assign12720_e11981: f64 = (0.25 * assign12720_e11980);
        let assign12720_e11983: f64 = (assign12720_e11981 / locals.var_q_qsq);
        (assign12720_e11983, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12720_e11985;
        locals.var_q_d1_qcoth_dn4 = assign12720_e11985_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12720_e11985_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12720_e11985_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12720_e11985_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12720_e11985_d_n9;

        let (assign12730_e12009, assign12730_e12009_d_n4, assign12730_e12009_d_n6, assign12730_e12009_d_n7, assign12730_e12009_d_n8, assign12730_e12009_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 == 0.0)) {
        let assign12730_e11994: f64 = (locals.var_q_qsq * 0.1666666666667);
        let assign12730_e11998: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign12730_e12002: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign12730_e12003: f64 = (1.0 - assign12730_e12002);
        let assign12730_e12004: f64 = (assign12730_e11998 * assign12730_e12003);
        let assign12730_e12005: f64 = (1.0 - assign12730_e12004);
        let assign12730_e12006: f64 = (assign12730_e11994 * assign12730_e12005);
        let assign12730_e12007: f64 = (2.0 + assign12730_e12006);
        (assign12730_e12007, (((locals.var_q_qsq_dn4 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq_dn6 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq_dn7 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq_dn8 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq_dn9 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12730_e12009;
        locals.var_q_qcoth_dn4 = assign12730_e12009_d_n4;
        locals.var_q_qcoth_dn6 = assign12730_e12009_d_n6;
        locals.var_q_qcoth_dn7 = assign12730_e12009_d_n7;
        locals.var_q_qcoth_dn8 = assign12730_e12009_d_n8;
        locals.var_q_qcoth_dn9 = assign12730_e12009_d_n9;

        let (assign12740_e12035, assign12740_e12035_d_n4, assign12740_e12035_d_n6, assign12740_e12035_d_n7, assign12740_e12035_d_n8, assign12740_e12035_d_n9,) = {
    if ((locals.var_guard561 == 0.0) && (locals.var_guard562 == 0.0)) {
        let assign12740_e12019: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign12740_e12023: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign12740_e12027: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign12740_e12028: f64 = (1.0 - assign12740_e12027);
        let assign12740_e12029: f64 = (assign12740_e12023 * assign12740_e12028);
        let assign12740_e12030: f64 = (1.0 - assign12740_e12029);
        let assign12740_e12031: f64 = (assign12740_e12019 * assign12740_e12030);
        let assign12740_e12032: f64 = (1.0 - assign12740_e12031);
        let assign12740_e12033: f64 = (0.1666666666667 * assign12740_e12032);
        (assign12740_e12033, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12740_e12035;
        locals.var_q_d1_qcoth_dn4 = assign12740_e12035_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12740_e12035_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12740_e12035_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12740_e12035_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12740_e12035_d_n9;

        let assign12750_e12039: f64 = (locals.var_q_qi_int * locals.var_q_qcoth);
        let assign12750_e12042: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12750_e12043: f64 = (assign12750_e12039 + assign12750_e12042);
        let assign12750_e12045: f64 = (assign12750_e12043 + locals.var_q_qsq);
        let assign12750_e12048: f64 = (locals.var_q_qi_int * locals.var_q_d1_qcoth);
        let assign12750_e12050: f64 = (assign12750_e12048 + 1.0);
        let assign12750_e12051: f64 = (assign12750_e12045 / assign12750_e12050);
        let assign12750_e12052: f64 = (locals.var_q_qsq - assign12750_e12051);
        locals.var_q_qsq = assign12750_e12052;
        locals.var_q_qsq_dn4 = (locals.var_q_qsq_dn4 - (((((((locals.var_q_qi_int_dn4 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn4)) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))) + locals.var_q_qsq_dn4) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn4 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn4)))) / (assign12750_e12050 * assign12750_e12050)));
        locals.var_q_qsq_dn6 = (locals.var_q_qsq_dn6 - (((((((locals.var_q_qi_int_dn6 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn6)) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))) + locals.var_q_qsq_dn6) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn6 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn6)))) / (assign12750_e12050 * assign12750_e12050)));
        locals.var_q_qsq_dn7 = (locals.var_q_qsq_dn7 - (((((((locals.var_q_qi_int_dn7 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn7)) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))) + locals.var_q_qsq_dn7) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn7 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn7)))) / (assign12750_e12050 * assign12750_e12050)));
        locals.var_q_qsq_dn8 = (locals.var_q_qsq_dn8 - (((((((locals.var_q_qi_int_dn8 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn8)) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))) + locals.var_q_qsq_dn8) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn8 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn8)))) / (assign12750_e12050 * assign12750_e12050)));
        locals.var_q_qsq_dn9 = (locals.var_q_qsq_dn9 - (((((((locals.var_q_qi_int_dn9 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn9)) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))) + locals.var_q_qsq_dn9) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn9 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn9)))) / (assign12750_e12050 * assign12750_e12050)));

        let assign12760_e12055: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12760_e12057: f64 = (assign12760_e12055 - locals.var_q_qsq);
        locals.var_q_delta = assign12760_e12057;
        locals.var_q_delta_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4);
        locals.var_q_delta_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6);
        locals.var_q_delta_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7);
        locals.var_q_delta_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8);
        locals.var_q_delta_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9);

        let assign12770_e12060: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign12770_e12060;

        let (assign12780_e12075, assign12780_e12075_d_n4, assign12780_e12075_d_n6, assign12780_e12075_d_n7, assign12780_e12075_d_n8, assign12780_e12075_d_n9,) = {
    if (locals.var_guard563 != 0.0) {
        let assign12780_e12065: f64 = (locals.var_q_delta / locals.var_a0);
        let assign12780_e12066: f64 = (assign12780_e12065).ln();
        let assign12780_e12068: f64 = assign12780_e12066;
        let assign12780_e12070: f64 = (assign12780_e12068 - locals.var_xg1x);
        let assign12780_e12072: f64 = (assign12780_e12070 + locals.var_q1s);
        let assign12780_e12073: f64 = (locals.var_q_delta * assign12780_e12072);
        (assign12780_e12073, ((locals.var_q_delta_dn4 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn4) + locals.var_q1s_dn4))), ((locals.var_q_delta_dn6 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn6) + locals.var_q1s_dn6))), ((locals.var_q_delta_dn7 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn7) + locals.var_q1s_dn7))), ((locals.var_q_delta_dn8 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn8) + locals.var_q1s_dn8))), ((locals.var_q_delta_dn9 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn9) + locals.var_q1s_dn9))),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign12780_e12075;
        locals.var_q_zero_dn4 = assign12780_e12075_d_n4;
        locals.var_q_zero_dn6 = assign12780_e12075_d_n6;
        locals.var_q_zero_dn7 = assign12780_e12075_d_n7;
        locals.var_q_zero_dn8 = assign12780_e12075_d_n8;
        locals.var_q_zero_dn9 = assign12780_e12075_d_n9;

        let (assign12790_e12085, assign12790_e12085_d_n4, assign12790_e12085_d_n6, assign12790_e12085_d_n7, assign12790_e12085_d_n8, assign12790_e12085_d_n9,) = {
    if (locals.var_guard563 != 0.0) {
        let assign12790_e12079: f64 = (2.0 * locals.var_k1);
        let assign12790_e12081: f64 = (assign12790_e12079 * locals.var_q_k1q1);
        let assign12790_e12083: f64 = (assign12790_e12081 + locals.var_q_delta);
        (assign12790_e12083, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign12790_e12085;
        locals.var_q_d1_zero_dn4 = assign12790_e12085_d_n4;
        locals.var_q_d1_zero_dn6 = assign12790_e12085_d_n6;
        locals.var_q_d1_zero_dn7 = assign12790_e12085_d_n7;
        locals.var_q_d1_zero_dn8 = assign12790_e12085_d_n8;
        locals.var_q_d1_zero_dn9 = assign12790_e12085_d_n9;

        let (assign12800_e12093,) = {
    if (locals.var_guard563 != 0.0) {
        let assign12800_e12089: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12800_e12091: f64 = (assign12800_e12089 - locals.var_q_x1sat);
        (assign12800_e12091,)
    } else {
        (locals.var_q_dx1,)
    }
};
        locals.var_q_dx1 = assign12800_e12093;

        let assign12810_e12103: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign12810_e12105: f64 = (locals.var_k1).ln();
        let assign12810_e12106: f64 = (assign12810_e12103 + assign12810_e12105);
        let assign12810_e12113: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign12810_e12106 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard564 = assign12810_e12113;

        let (assign12820_e12123, assign12820_e12123_d_n4, assign12820_e12123_d_n6, assign12820_e12123_d_n7, assign12820_e12123_d_n8, assign12820_e12123_d_n9,) = {
    if ((locals.var_guard563 != 0.0) && (locals.var_guard564 != 0.0)) {
        let assign12820_e12120: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
        let assign12820_e12121: f64 = (locals.var_q1s - assign12820_e12120);
        (assign12820_e12121, (locals.var_q1s_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))),)
    } else {
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9,)
    }
};
        locals.var_q1s = assign12820_e12123;
        locals.var_q1s_dn4 = assign12820_e12123_d_n4;
        locals.var_q1s_dn6 = assign12820_e12123_d_n6;
        locals.var_q1s_dn7 = assign12820_e12123_d_n7;
        locals.var_q1s_dn8 = assign12820_e12123_d_n8;
        locals.var_q1s_dn9 = assign12820_e12123_d_n9;

    }

    pub(super) fn stamp_transient_block_30(
        locals: &mut StampLocals,
    ) {
        let assign12830_e12126: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign12830_e12126;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign12840_e12129: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12840_e12131: f64 = assign12840_e12129;
        let assign12840_e12133: f64 = if assign12840_e12131 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard565 = assign12840_e12133;

        let (assign12850_e12142, assign12850_e12142_d_n4, assign12850_e12142_d_n6, assign12850_e12142_d_n7, assign12850_e12142_d_n8, assign12850_e12142_d_n9,) = {
    if (locals.var_guard565 != 0.0) {
        let assign12850_e12137: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12850_e12139: f64 = assign12850_e12137;
        let assign12850_e12140: f64 = (assign12850_e12139).exp();
        (assign12850_e12140, (assign12850_e12140 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign12850_e12140 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign12850_e12140 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign12850_e12140 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign12850_e12140 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12850_e12142;
        locals.var_q_temp1_dn4 = assign12850_e12142_d_n4;
        locals.var_q_temp1_dn6 = assign12850_e12142_d_n6;
        locals.var_q_temp1_dn7 = assign12850_e12142_d_n7;
        locals.var_q_temp1_dn8 = assign12850_e12142_d_n8;
        locals.var_q_temp1_dn9 = assign12850_e12142_d_n9;

        let (assign12860_e12181, assign12860_e12181_d_n4, assign12860_e12181_d_n6, assign12860_e12181_d_n7, assign12860_e12181_d_n8, assign12860_e12181_d_n9,) = {
    if (locals.var_guard565 == 0.0) {
        let assign12860_e12149: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12860_e12151: f64 = assign12860_e12149;
        let assign12860_e12153: f64 = (assign12860_e12151 - 80.0);
        let assign12860_e12158: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12860_e12160: f64 = assign12860_e12158;
        let assign12860_e12162: f64 = (assign12860_e12160 - 80.0);
        let assign12860_e12163: f64 = (0.5 * assign12860_e12162);
        let assign12860_e12167: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12860_e12169: f64 = assign12860_e12167;
        let assign12860_e12171: f64 = (assign12860_e12169 - 80.0);
        let assign12860_e12173: f64 = (assign12860_e12171 * 0.3333333333333);
        let assign12860_e12174: f64 = (1.0 + assign12860_e12173);
        let assign12860_e12175: f64 = (assign12860_e12163 * assign12860_e12174);
        let assign12860_e12176: f64 = (1.0 + assign12860_e12175);
        let assign12860_e12177: f64 = (assign12860_e12153 * assign12860_e12176);
        let assign12860_e12178: f64 = (1.0 + assign12860_e12177);
        let assign12860_e12179: f64 = (5.54062e34 * assign12860_e12178);
        (assign12860_e12179, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12860_e12181;
        locals.var_q_temp1_dn4 = assign12860_e12181_d_n4;
        locals.var_q_temp1_dn6 = assign12860_e12181_d_n6;
        locals.var_q_temp1_dn7 = assign12860_e12181_d_n7;
        locals.var_q_temp1_dn8 = assign12860_e12181_d_n8;
        locals.var_q_temp1_dn9 = assign12860_e12181_d_n9;

        let assign12870_e12184: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign12870_e12184;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign12880_e12187: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12880_e12189: f64 = (assign12880_e12187 - locals.var_q_aexp);
        locals.var_q_qsq = assign12880_e12189;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign12890_e12192: f64 = (2.0 * locals.var_k1);
        let assign12890_e12194: f64 = (assign12890_e12192 * locals.var_q_k1q1);
        let assign12890_e12196: f64 = (assign12890_e12194 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign12890_e12196;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign12900_e12199: f64 = (2.0 * locals.var_k1);
        let assign12900_e12201: f64 = (assign12900_e12199 * locals.var_k1);
        let assign12900_e12203: f64 = (assign12900_e12201 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign12900_e12203;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign12910_e12206: f64 = (-0.005);
        let assign12910_e12207: f64 = if locals.var_q_qsq < assign12910_e12206 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign12910_e12207;

        let (assign12920_e12213, assign12920_e12213_d_n4, assign12920_e12213_d_n6, assign12920_e12213_d_n7, assign12920_e12213_d_n8, assign12920_e12213_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12920_e12210: f64 = (locals.var_q_qsq).abs();
        let assign12920_e12211: f64 = (assign12920_e12210).sqrt();
        (assign12920_e12211, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12920_e12211)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12920_e12211)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12920_e12211)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12920_e12211)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12920_e12211)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign12920_e12213;
        locals.var_q_rac_qsq_dn4 = assign12920_e12213_d_n4;
        locals.var_q_rac_qsq_dn6 = assign12920_e12213_d_n6;
        locals.var_q_rac_qsq_dn7 = assign12920_e12213_d_n7;
        locals.var_q_rac_qsq_dn8 = assign12920_e12213_d_n8;
        locals.var_q_rac_qsq_dn9 = assign12920_e12213_d_n9;

        let (assign12930_e12222, assign12930_e12222_d_n4, assign12930_e12222_d_n6, assign12930_e12222_d_n7, assign12930_e12222_d_n8, assign12930_e12222_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12930_e12218: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign12930_e12219: f64 = (assign12930_e12218).tan();
        let assign12930_e12220: f64 = (locals.var_q_rac_qsq / assign12930_e12219);
        (assign12930_e12220, (((locals.var_q_rac_qsq_dn4 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), (((locals.var_q_rac_qsq_dn6 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), (((locals.var_q_rac_qsq_dn7 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), (((locals.var_q_rac_qsq_dn8 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), (((locals.var_q_rac_qsq_dn9 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign12930_e12222;
        locals.var_q_qcoth_dn4 = assign12930_e12222_d_n4;
        locals.var_q_qcoth_dn6 = assign12930_e12222_d_n6;
        locals.var_q_qcoth_dn7 = assign12930_e12222_d_n7;
        locals.var_q_qcoth_dn8 = assign12930_e12222_d_n8;
        locals.var_q_qcoth_dn9 = assign12930_e12222_d_n9;

        let (assign12940_e12230, assign12940_e12230_d_n4, assign12940_e12230_d_n6, assign12940_e12230_d_n7, assign12940_e12230_d_n8, assign12940_e12230_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12940_e12226: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign12940_e12228: f64 = (assign12940_e12226 / locals.var_q_qsq);
        (assign12940_e12228, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign12940_e12230;
        locals.var_q_temp1_dn4 = assign12940_e12230_d_n4;
        locals.var_q_temp1_dn6 = assign12940_e12230_d_n6;
        locals.var_q_temp1_dn7 = assign12940_e12230_d_n7;
        locals.var_q_temp1_dn8 = assign12940_e12230_d_n8;
        locals.var_q_temp1_dn9 = assign12940_e12230_d_n9;

        let (assign12950_e12242, assign12950_e12242_d_n4, assign12950_e12242_d_n6, assign12950_e12242_d_n7, assign12950_e12242_d_n8, assign12950_e12242_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12950_e12236: f64 = (2.0 - locals.var_q_qcoth);
        let assign12950_e12237: f64 = (locals.var_q_qcoth * assign12950_e12236);
        let assign12950_e12238: f64 = (locals.var_q_qsq + assign12950_e12237);
        let assign12950_e12240: f64 = (assign12950_e12238 * locals.var_q_temp1);
        (assign12950_e12240, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign12950_e12242;
        locals.var_q_d1_qcoth_dn4 = assign12950_e12242_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign12950_e12242_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign12950_e12242_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign12950_e12242_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign12950_e12242_d_n9;

        let (assign12960_e12262, assign12960_e12262_d_n4, assign12960_e12262_d_n6, assign12960_e12262_d_n7, assign12960_e12262_d_n8, assign12960_e12262_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12960_e12247: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign12960_e12250: f64 = (1.0 + locals.var_q_qcoth);
        let assign12960_e12251: f64 = (assign12960_e12247 * assign12960_e12250);
        let assign12960_e12252: f64 = (locals.var_q_d1_qsq - assign12960_e12251);
        let assign12960_e12254: f64 = (assign12960_e12252 * locals.var_q_temp1);
        let assign12960_e12257: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign12960_e12259: f64 = (assign12960_e12257 / locals.var_q_d1_qsq);
        let assign12960_e12260: f64 = (assign12960_e12254 + assign12960_e12259);
        (assign12960_e12260, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign12960_e12262;
        locals.var_q_d2_qcoth_dn4 = assign12960_e12262_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign12960_e12262_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign12960_e12262_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign12960_e12262_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign12960_e12262_d_n9;

        let (assign12970_e12270, assign12970_e12270_d_n4, assign12970_e12270_d_n6, assign12970_e12270_d_n7, assign12970_e12270_d_n8, assign12970_e12270_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12970_e12267: f64 = (0.5 * locals.var_q_qcoth);
        let assign12970_e12268: f64 = (1.0 - assign12970_e12267);
        (assign12970_e12268, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign12970_e12270;
        locals.var_q_temp2_dn4 = assign12970_e12270_d_n4;
        locals.var_q_temp2_dn6 = assign12970_e12270_d_n6;
        locals.var_q_temp2_dn7 = assign12970_e12270_d_n7;
        locals.var_q_temp2_dn8 = assign12970_e12270_d_n8;
        locals.var_q_temp2_dn9 = assign12970_e12270_d_n9;

        let (assign12980_e12278, assign12980_e12278_d_n4, assign12980_e12278_d_n6, assign12980_e12278_d_n7, assign12980_e12278_d_n8, assign12980_e12278_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12980_e12274: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign12980_e12276: f64 = (assign12980_e12274 * locals.var_q_temp2);
        (assign12980_e12276, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign12980_e12278;
        locals.var_q_d1_ln_dn4 = assign12980_e12278_d_n4;
        locals.var_q_d1_ln_dn6 = assign12980_e12278_d_n6;
        locals.var_q_d1_ln_dn7 = assign12980_e12278_d_n7;
        locals.var_q_d1_ln_dn8 = assign12980_e12278_d_n8;
        locals.var_q_d1_ln_dn9 = assign12980_e12278_d_n9;

        let (assign12990_e12294, assign12990_e12294_d_n4, assign12990_e12294_d_n6, assign12990_e12294_d_n7, assign12990_e12294_d_n8, assign12990_e12294_d_n9,) = {
    if (locals.var_guard566 != 0.0) {
        let assign12990_e12282: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign12990_e12287: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign12990_e12288: f64 = (locals.var_q_d1_ln + assign12990_e12287);
        let assign12990_e12289: f64 = (locals.var_q_d1_qsq * assign12990_e12288);
        let assign12990_e12290: f64 = (assign12990_e12282 - assign12990_e12289);
        let assign12990_e12292: f64 = (assign12990_e12290 / locals.var_q_qsq);
        (assign12990_e12292, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign12990_e12294;
        locals.var_q_d2_ln_dn4 = assign12990_e12294_d_n4;
        locals.var_q_d2_ln_dn6 = assign12990_e12294_d_n6;
        locals.var_q_d2_ln_dn7 = assign12990_e12294_d_n7;
        locals.var_q_d2_ln_dn8 = assign12990_e12294_d_n8;
        locals.var_q_d2_ln_dn9 = assign12990_e12294_d_n9;

        let assign13000_e12297: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard567 = assign13000_e12297;

        let (assign13010_e12306, assign13010_e12306_d_n4, assign13010_e12306_d_n6, assign13010_e12306_d_n7, assign13010_e12306_d_n8, assign13010_e12306_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13010_e12303: f64 = (locals.var_q_qsq).abs();
        let assign13010_e12304: f64 = (assign13010_e12303).sqrt();
        (assign13010_e12304, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13010_e12304)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13010_e12304)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13010_e12304)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13010_e12304)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13010_e12304)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign13010_e12306;
        locals.var_q_rac_qsq_dn4 = assign13010_e12306_d_n4;
        locals.var_q_rac_qsq_dn6 = assign13010_e12306_d_n6;
        locals.var_q_rac_qsq_dn7 = assign13010_e12306_d_n7;
        locals.var_q_rac_qsq_dn8 = assign13010_e12306_d_n8;
        locals.var_q_rac_qsq_dn9 = assign13010_e12306_d_n9;

        let (assign13020_e12315, assign13020_e12315_d_n4, assign13020_e12315_d_n6, assign13020_e12315_d_n7, assign13020_e12315_d_n8, assign13020_e12315_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13020_e12312: f64 = (-locals.var_q_rac_qsq);
        let assign13020_e12313: f64 = (assign13020_e12312).exp();
        (assign13020_e12313, (assign13020_e12313 * (-locals.var_q_rac_qsq_dn4)), (assign13020_e12313 * (-locals.var_q_rac_qsq_dn6)), (assign13020_e12313 * (-locals.var_q_rac_qsq_dn7)), (assign13020_e12313 * (-locals.var_q_rac_qsq_dn8)), (assign13020_e12313 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign13020_e12315;
        locals.var_q_invexpq_dn4 = assign13020_e12315_d_n4;
        locals.var_q_invexpq_dn6 = assign13020_e12315_d_n6;
        locals.var_q_invexpq_dn7 = assign13020_e12315_d_n7;
        locals.var_q_invexpq_dn8 = assign13020_e12315_d_n8;
        locals.var_q_invexpq_dn9 = assign13020_e12315_d_n9;

        let (assign13030_e12330, assign13030_e12330_d_n4, assign13030_e12330_d_n6, assign13030_e12330_d_n7, assign13030_e12330_d_n8, assign13030_e12330_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13030_e12323: f64 = (1.0 + locals.var_q_invexpq);
        let assign13030_e12324: f64 = (locals.var_q_rac_qsq * assign13030_e12323);
        let assign13030_e12327: f64 = (1.0 - locals.var_q_invexpq);
        let assign13030_e12328: f64 = (assign13030_e12324 / assign13030_e12327);
        (assign13030_e12328, (((((locals.var_q_rac_qsq_dn4 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn4))) / (assign13030_e12327 * assign13030_e12327)), (((((locals.var_q_rac_qsq_dn6 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn6))) / (assign13030_e12327 * assign13030_e12327)), (((((locals.var_q_rac_qsq_dn7 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn7))) / (assign13030_e12327 * assign13030_e12327)), (((((locals.var_q_rac_qsq_dn8 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn8))) / (assign13030_e12327 * assign13030_e12327)), (((((locals.var_q_rac_qsq_dn9 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn9))) / (assign13030_e12327 * assign13030_e12327)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13030_e12330;
        locals.var_q_qcoth_dn4 = assign13030_e12330_d_n4;
        locals.var_q_qcoth_dn6 = assign13030_e12330_d_n6;
        locals.var_q_qcoth_dn7 = assign13030_e12330_d_n7;
        locals.var_q_qcoth_dn8 = assign13030_e12330_d_n8;
        locals.var_q_qcoth_dn9 = assign13030_e12330_d_n9;

        let (assign13040_e12341, assign13040_e12341_d_n4, assign13040_e12341_d_n6, assign13040_e12341_d_n7, assign13040_e12341_d_n8, assign13040_e12341_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13040_e12337: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign13040_e12339: f64 = (assign13040_e12337 / locals.var_q_qsq);
        (assign13040_e12339, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13040_e12341;
        locals.var_q_temp1_dn4 = assign13040_e12341_d_n4;
        locals.var_q_temp1_dn6 = assign13040_e12341_d_n6;
        locals.var_q_temp1_dn7 = assign13040_e12341_d_n7;
        locals.var_q_temp1_dn8 = assign13040_e12341_d_n8;
        locals.var_q_temp1_dn9 = assign13040_e12341_d_n9;

        let (assign13050_e12356, assign13050_e12356_d_n4, assign13050_e12356_d_n6, assign13050_e12356_d_n7, assign13050_e12356_d_n8, assign13050_e12356_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13050_e12350: f64 = (2.0 - locals.var_q_qcoth);
        let assign13050_e12351: f64 = (locals.var_q_qcoth * assign13050_e12350);
        let assign13050_e12352: f64 = (locals.var_q_qsq + assign13050_e12351);
        let assign13050_e12354: f64 = (assign13050_e12352 * locals.var_q_temp1);
        (assign13050_e12354, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13050_e12356;
        locals.var_q_d1_qcoth_dn4 = assign13050_e12356_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13050_e12356_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13050_e12356_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13050_e12356_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13050_e12356_d_n9;

        let (assign13060_e12379, assign13060_e12379_d_n4, assign13060_e12379_d_n6, assign13060_e12379_d_n7, assign13060_e12379_d_n8, assign13060_e12379_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13060_e12364: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign13060_e12367: f64 = (1.0 + locals.var_q_qcoth);
        let assign13060_e12368: f64 = (assign13060_e12364 * assign13060_e12367);
        let assign13060_e12369: f64 = (locals.var_q_d1_qsq - assign13060_e12368);
        let assign13060_e12371: f64 = (assign13060_e12369 * locals.var_q_temp1);
        let assign13060_e12374: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign13060_e12376: f64 = (assign13060_e12374 / locals.var_q_d1_qsq);
        let assign13060_e12377: f64 = (assign13060_e12371 + assign13060_e12376);
        (assign13060_e12377, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13060_e12379;
        locals.var_q_d2_qcoth_dn4 = assign13060_e12379_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13060_e12379_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13060_e12379_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13060_e12379_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13060_e12379_d_n9;

        let (assign13070_e12390, assign13070_e12390_d_n4, assign13070_e12390_d_n6, assign13070_e12390_d_n7, assign13070_e12390_d_n8, assign13070_e12390_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13070_e12387: f64 = (0.5 * locals.var_q_qcoth);
        let assign13070_e12388: f64 = (1.0 - assign13070_e12387);
        (assign13070_e12388, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13070_e12390;
        locals.var_q_temp2_dn4 = assign13070_e12390_d_n4;
        locals.var_q_temp2_dn6 = assign13070_e12390_d_n6;
        locals.var_q_temp2_dn7 = assign13070_e12390_d_n7;
        locals.var_q_temp2_dn8 = assign13070_e12390_d_n8;
        locals.var_q_temp2_dn9 = assign13070_e12390_d_n9;

        let (assign13080_e12401, assign13080_e12401_d_n4, assign13080_e12401_d_n6, assign13080_e12401_d_n7, assign13080_e12401_d_n8, assign13080_e12401_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13080_e12397: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign13080_e12399: f64 = (assign13080_e12397 * locals.var_q_temp2);
        (assign13080_e12399, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13080_e12401;
        locals.var_q_d1_ln_dn4 = assign13080_e12401_d_n4;
        locals.var_q_d1_ln_dn6 = assign13080_e12401_d_n6;
        locals.var_q_d1_ln_dn7 = assign13080_e12401_d_n7;
        locals.var_q_d1_ln_dn8 = assign13080_e12401_d_n8;
        locals.var_q_d1_ln_dn9 = assign13080_e12401_d_n9;

        let (assign13090_e12420, assign13090_e12420_d_n4, assign13090_e12420_d_n6, assign13090_e12420_d_n7, assign13090_e12420_d_n8, assign13090_e12420_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
        let assign13090_e12408: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign13090_e12413: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign13090_e12414: f64 = (locals.var_q_d1_ln + assign13090_e12413);
        let assign13090_e12415: f64 = (locals.var_q_d1_qsq * assign13090_e12414);
        let assign13090_e12416: f64 = (assign13090_e12408 - assign13090_e12415);
        let assign13090_e12418: f64 = (assign13090_e12416 / locals.var_q_qsq);
        (assign13090_e12418, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13090_e12420;
        locals.var_q_d2_ln_dn4 = assign13090_e12420_d_n4;
        locals.var_q_d2_ln_dn6 = assign13090_e12420_d_n6;
        locals.var_q_d2_ln_dn7 = assign13090_e12420_d_n7;
        locals.var_q_d2_ln_dn8 = assign13090_e12420_d_n8;
        locals.var_q_d2_ln_dn9 = assign13090_e12420_d_n9;

        let (assign13100_e12446, assign13100_e12446_d_n4, assign13100_e12446_d_n6, assign13100_e12446_d_n7, assign13100_e12446_d_n8, assign13100_e12446_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13100_e12430: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign13100_e12434: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign13100_e12438: f64 = (locals.var_q_qsq * 0.025);
        let assign13100_e12439: f64 = (1.0 - assign13100_e12438);
        let assign13100_e12440: f64 = (assign13100_e12434 * assign13100_e12439);
        let assign13100_e12441: f64 = (1.0 - assign13100_e12440);
        let assign13100_e12442: f64 = (assign13100_e12430 * assign13100_e12441);
        let assign13100_e12443: f64 = (1.0 - assign13100_e12442);
        let assign13100_e12444: f64 = (0.1666666666667 * assign13100_e12443);
        (assign13100_e12444, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign13100_e12446;
        locals.var_q_temp3_dn4 = assign13100_e12446_d_n4;
        locals.var_q_temp3_dn6 = assign13100_e12446_d_n6;
        locals.var_q_temp3_dn7 = assign13100_e12446_d_n7;
        locals.var_q_temp3_dn8 = assign13100_e12446_d_n8;
        locals.var_q_temp3_dn9 = assign13100_e12446_d_n9;

        let (assign13110_e12458, assign13110_e12458_d_n4, assign13110_e12458_d_n6, assign13110_e12458_d_n7, assign13110_e12458_d_n8, assign13110_e12458_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13110_e12455: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign13110_e12456: f64 = (2.0 + assign13110_e12455);
        (assign13110_e12456, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13110_e12458;
        locals.var_q_qcoth_dn4 = assign13110_e12458_d_n4;
        locals.var_q_qcoth_dn6 = assign13110_e12458_d_n6;
        locals.var_q_qcoth_dn7 = assign13110_e12458_d_n7;
        locals.var_q_qcoth_dn8 = assign13110_e12458_d_n8;
        locals.var_q_qcoth_dn9 = assign13110_e12458_d_n9;

        let (assign13120_e12484, assign13120_e12484_d_n4, assign13120_e12484_d_n6, assign13120_e12484_d_n7, assign13120_e12484_d_n8, assign13120_e12484_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13120_e12468: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign13120_e12472: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign13120_e12476: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign13120_e12477: f64 = (1.0 - assign13120_e12476);
        let assign13120_e12478: f64 = (assign13120_e12472 * assign13120_e12477);
        let assign13120_e12479: f64 = (1.0 - assign13120_e12478);
        let assign13120_e12480: f64 = (assign13120_e12468 * assign13120_e12479);
        let assign13120_e12481: f64 = (1.0 - assign13120_e12480);
        let assign13120_e12482: f64 = (0.1666666666667 * assign13120_e12481);
        (assign13120_e12482, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13120_e12484;
        locals.var_q_temp1_dn4 = assign13120_e12484_d_n4;
        locals.var_q_temp1_dn6 = assign13120_e12484_d_n6;
        locals.var_q_temp1_dn7 = assign13120_e12484_d_n7;
        locals.var_q_temp1_dn8 = assign13120_e12484_d_n8;
        locals.var_q_temp1_dn9 = assign13120_e12484_d_n9;

        let (assign13130_e12494, assign13130_e12494_d_n4, assign13130_e12494_d_n6, assign13130_e12494_d_n7, assign13130_e12494_d_n8, assign13130_e12494_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13130_e12492: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign13130_e12492, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13130_e12494;
        locals.var_q_d1_qcoth_dn4 = assign13130_e12494_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13130_e12494_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13130_e12494_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13130_e12494_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13130_e12494_d_n9;

        let (assign13140_e12520, assign13140_e12520_d_n4, assign13140_e12520_d_n6, assign13140_e12520_d_n7, assign13140_e12520_d_n8, assign13140_e12520_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13140_e12504: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign13140_e12508: f64 = (0.05 * locals.var_q_qsq);
        let assign13140_e12512: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign13140_e12513: f64 = (1.0 - assign13140_e12512);
        let assign13140_e12514: f64 = (assign13140_e12508 * assign13140_e12513);
        let assign13140_e12515: f64 = (1.0 - assign13140_e12514);
        let assign13140_e12516: f64 = (assign13140_e12504 * assign13140_e12515);
        let assign13140_e12517: f64 = (1.0 - assign13140_e12516);
        let assign13140_e12518: f64 = (0.0055555555556 * assign13140_e12517);
        (assign13140_e12518, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13140_e12520;
        locals.var_q_temp2_dn4 = assign13140_e12520_d_n4;
        locals.var_q_temp2_dn6 = assign13140_e12520_d_n6;
        locals.var_q_temp2_dn7 = assign13140_e12520_d_n7;
        locals.var_q_temp2_dn8 = assign13140_e12520_d_n8;
        locals.var_q_temp2_dn9 = assign13140_e12520_d_n9;

    }

    pub(super) fn stamp_transient_block_31(
        locals: &mut StampLocals,
    ) {
        let (assign13150_e12536, assign13150_e12536_d_n4, assign13150_e12536_d_n6, assign13150_e12536_d_n7, assign13150_e12536_d_n8, assign13150_e12536_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13150_e12528: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign13150_e12531: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign13150_e12533: f64 = (assign13150_e12531 * locals.var_q_temp2);
        let assign13150_e12534: f64 = (assign13150_e12528 - assign13150_e12533);
        (assign13150_e12534, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13150_e12536;
        locals.var_q_d2_qcoth_dn4 = assign13150_e12536_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13150_e12536_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13150_e12536_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13150_e12536_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13150_e12536_d_n9;

        let (assign13160_e12549, assign13160_e12549_d_n4, assign13160_e12549_d_n6, assign13160_e12549_d_n7, assign13160_e12549_d_n8, assign13160_e12549_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13160_e12543: f64 = (-0.5);
        let assign13160_e12545: f64 = (assign13160_e12543 * locals.var_q_d1_qsq);
        let assign13160_e12547: f64 = (assign13160_e12545 * locals.var_q_temp3);
        (assign13160_e12547, (((assign13160_e12543 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn4)), (((assign13160_e12543 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn6)), (((assign13160_e12543 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn7)), (((assign13160_e12543 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn8)), (((assign13160_e12543 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13160_e12549;
        locals.var_q_d1_ln_dn4 = assign13160_e12549_d_n4;
        locals.var_q_d1_ln_dn6 = assign13160_e12549_d_n6;
        locals.var_q_d1_ln_dn7 = assign13160_e12549_d_n7;
        locals.var_q_d1_ln_dn8 = assign13160_e12549_d_n8;
        locals.var_q_d1_ln_dn9 = assign13160_e12549_d_n9;

        let (assign13170_e12582, assign13170_e12582_d_n4, assign13170_e12582_d_n6, assign13170_e12582_d_n7, assign13170_e12582_d_n8, assign13170_e12582_d_n9,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
        let assign13170_e12556: f64 = (-0.5);
        let assign13170_e12558: f64 = (assign13170_e12556 * locals.var_q_d2_qsq);
        let assign13170_e12560: f64 = (assign13170_e12558 * locals.var_q_temp3);
        let assign13170_e12563: f64 = (0.25 * 0.0055555555556);
        let assign13170_e12565: f64 = (assign13170_e12563 * locals.var_q_d1_qsq);
        let assign13170_e12567: f64 = (assign13170_e12565 * locals.var_q_d1_qsq);
        let assign13170_e12571: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign13170_e12575: f64 = (0.075 * locals.var_q_qsq);
        let assign13170_e12576: f64 = (2.0 - assign13170_e12575);
        let assign13170_e12577: f64 = (assign13170_e12571 * assign13170_e12576);
        let assign13170_e12578: f64 = (1.0 - assign13170_e12577);
        let assign13170_e12579: f64 = (assign13170_e12567 * assign13170_e12578);
        let assign13170_e12580: f64 = (assign13170_e12560 + assign13170_e12579);
        (assign13170_e12580, ((((assign13170_e12556 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn4)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn4)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign13170_e12556 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn6)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn6)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign13170_e12556 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn7)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn7)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign13170_e12556 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn8)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn8)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign13170_e12556 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn9)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn9)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13170_e12582;
        locals.var_q_d2_ln_dn4 = assign13170_e12582_d_n4;
        locals.var_q_d2_ln_dn6 = assign13170_e12582_d_n6;
        locals.var_q_d2_ln_dn7 = assign13170_e12582_d_n7;
        locals.var_q_d2_ln_dn8 = assign13170_e12582_d_n8;
        locals.var_q_d2_ln_dn9 = assign13170_e12582_d_n9;

        let assign13180_e12585: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign13180_e12585;

        let (assign13190_e12599, assign13190_e12599_d_n4, assign13190_e12599_d_n6, assign13190_e12599_d_n7, assign13190_e12599_d_n8, assign13190_e12599_d_n9,) = {
    if (locals.var_guard568 != 0.0) {
        let assign13190_e12589: f64 = (4.0 * locals.var_q_qsq);
        let assign13190_e12594: f64 = (2.0 - locals.var_q_invexpq);
        let assign13190_e12595: f64 = (locals.var_q_invexpq * assign13190_e12594);
        let assign13190_e12596: f64 = (1.0 - assign13190_e12595);
        let assign13190_e12597: f64 = (assign13190_e12589 / assign13190_e12596);
        (assign13190_e12597, ((((4.0 * locals.var_q_qsq_dn4) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn4 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign13190_e12596 * assign13190_e12596)), ((((4.0 * locals.var_q_qsq_dn6) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn6 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign13190_e12596 * assign13190_e12596)), ((((4.0 * locals.var_q_qsq_dn7) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn7 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign13190_e12596 * assign13190_e12596)), ((((4.0 * locals.var_q_qsq_dn8) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn8 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign13190_e12596 * assign13190_e12596)), ((((4.0 * locals.var_q_qsq_dn9) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn9 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign13190_e12596 * assign13190_e12596)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13190_e12599;
        locals.var_q_temp2_dn4 = assign13190_e12599_d_n4;
        locals.var_q_temp2_dn6 = assign13190_e12599_d_n6;
        locals.var_q_temp2_dn7 = assign13190_e12599_d_n7;
        locals.var_q_temp2_dn8 = assign13190_e12599_d_n8;
        locals.var_q_temp2_dn9 = assign13190_e12599_d_n9;

        let (assign13200_e12605, assign13200_e12605_d_n4, assign13200_e12605_d_n6, assign13200_e12605_d_n7, assign13200_e12605_d_n8, assign13200_e12605_d_n9,) = {
    if (locals.var_guard568 != 0.0) {
        let assign13200_e12603: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign13200_e12603, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13200_e12605;
        locals.var_q_sh_term_dn4 = assign13200_e12605_d_n4;
        locals.var_q_sh_term_dn6 = assign13200_e12605_d_n6;
        locals.var_q_sh_term_dn7 = assign13200_e12605_d_n7;
        locals.var_q_sh_term_dn8 = assign13200_e12605_d_n8;
        locals.var_q_sh_term_dn9 = assign13200_e12605_d_n9;

        let (assign13210_e12612, assign13210_e12612_d_n4, assign13210_e12612_d_n6, assign13210_e12612_d_n7, assign13210_e12612_d_n8, assign13210_e12612_d_n9,) = {
    if (locals.var_guard568 != 0.0) {
        let assign13210_e12608: f64 = (locals.var_q_temp2).ln();
        let assign13210_e12610: f64 = (assign13210_e12608 - locals.var_q_rac_qsq);
        (assign13210_e12610, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13210_e12612;
        locals.var_q_ln_term_dn4 = assign13210_e12612_d_n4;
        locals.var_q_ln_term_dn6 = assign13210_e12612_d_n6;
        locals.var_q_ln_term_dn7 = assign13210_e12612_d_n7;
        locals.var_q_ln_term_dn8 = assign13210_e12612_d_n8;
        locals.var_q_ln_term_dn9 = assign13210_e12612_d_n9;

        let assign13220_e12615: f64 = (-0.005);
        let assign13220_e12616: f64 = if locals.var_q_qsq < assign13220_e12615 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign13220_e12616;

        let (assign13230_e12626, assign13230_e12626_d_n4, assign13230_e12626_d_n6, assign13230_e12626_d_n7, assign13230_e12626_d_n8, assign13230_e12626_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
        let assign13230_e12623: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign13230_e12624: f64 = (assign13230_e12623).sin();
        (assign13230_e12624, ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13230_e12626;
        locals.var_q_temp2_dn4 = assign13230_e12626_d_n4;
        locals.var_q_temp2_dn6 = assign13230_e12626_d_n6;
        locals.var_q_temp2_dn7 = assign13230_e12626_d_n7;
        locals.var_q_temp2_dn8 = assign13230_e12626_d_n8;
        locals.var_q_temp2_dn9 = assign13230_e12626_d_n9;

        let (assign13240_e12638, assign13240_e12638_d_n4, assign13240_e12638_d_n6, assign13240_e12638_d_n7, assign13240_e12638_d_n8, assign13240_e12638_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
        let assign13240_e12632: f64 = (-locals.var_q_qsq);
        let assign13240_e12635: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign13240_e12636: f64 = (assign13240_e12632 / assign13240_e12635);
        (assign13240_e12636, ((((-locals.var_q_qsq_dn4) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign13240_e12635 * assign13240_e12635)), ((((-locals.var_q_qsq_dn6) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign13240_e12635 * assign13240_e12635)), ((((-locals.var_q_qsq_dn7) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign13240_e12635 * assign13240_e12635)), ((((-locals.var_q_qsq_dn8) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign13240_e12635 * assign13240_e12635)), ((((-locals.var_q_qsq_dn9) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign13240_e12635 * assign13240_e12635)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13240_e12638;
        locals.var_q_sh_term_dn4 = assign13240_e12638_d_n4;
        locals.var_q_sh_term_dn6 = assign13240_e12638_d_n6;
        locals.var_q_sh_term_dn7 = assign13240_e12638_d_n7;
        locals.var_q_sh_term_dn8 = assign13240_e12638_d_n8;
        locals.var_q_sh_term_dn9 = assign13240_e12638_d_n9;

        let (assign13250_e12646, assign13250_e12646_d_n4, assign13250_e12646_d_n6, assign13250_e12646_d_n7, assign13250_e12646_d_n8, assign13250_e12646_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
        let assign13250_e12644: f64 = (locals.var_q_sh_term).ln();
        (assign13250_e12644, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13250_e12646;
        locals.var_q_ln_term_dn4 = assign13250_e12646_d_n4;
        locals.var_q_ln_term_dn6 = assign13250_e12646_d_n6;
        locals.var_q_ln_term_dn7 = assign13250_e12646_d_n7;
        locals.var_q_ln_term_dn8 = assign13250_e12646_d_n8;
        locals.var_q_ln_term_dn9 = assign13250_e12646_d_n9;

        let (assign13260_e12670, assign13260_e12670_d_n4, assign13260_e12670_d_n6, assign13260_e12670_d_n7, assign13260_e12670_d_n8, assign13260_e12670_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 == 0.0)) {
        let assign13260_e12655: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign13260_e12659: f64 = (0.05 * locals.var_q_qsq);
        let assign13260_e12663: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign13260_e12664: f64 = (1.0 - assign13260_e12663);
        let assign13260_e12665: f64 = (assign13260_e12659 * assign13260_e12664);
        let assign13260_e12666: f64 = (1.0 - assign13260_e12665);
        let assign13260_e12667: f64 = (assign13260_e12655 * assign13260_e12666);
        let assign13260_e12668: f64 = (4.0 - assign13260_e12667);
        (assign13260_e12668, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13260_e12670;
        locals.var_q_sh_term_dn4 = assign13260_e12670_d_n4;
        locals.var_q_sh_term_dn6 = assign13260_e12670_d_n6;
        locals.var_q_sh_term_dn7 = assign13260_e12670_d_n7;
        locals.var_q_sh_term_dn8 = assign13260_e12670_d_n8;
        locals.var_q_sh_term_dn9 = assign13260_e12670_d_n9;

        let (assign13270_e12679, assign13270_e12679_d_n4, assign13270_e12679_d_n6, assign13270_e12679_d_n7, assign13270_e12679_d_n8, assign13270_e12679_d_n9,) = {
    if ((locals.var_guard568 == 0.0) && (locals.var_guard569 == 0.0)) {
        let assign13270_e12677: f64 = (locals.var_q_sh_term).ln();
        (assign13270_e12677, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13270_e12679;
        locals.var_q_ln_term_dn4 = assign13270_e12679_d_n4;
        locals.var_q_ln_term_dn6 = assign13270_e12679_d_n6;
        locals.var_q_ln_term_dn7 = assign13270_e12679_d_n7;
        locals.var_q_ln_term_dn8 = assign13270_e12679_d_n8;
        locals.var_q_ln_term_dn9 = assign13270_e12679_d_n9;

        let assign13280_e12682: f64 = (1.01 * locals.var_q_k1q1);
        let assign13280_e12684: f64 = (assign13280_e12682 + locals.var_q_qcoth);
        let assign13280_e12686: f64 = if assign13280_e12684 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign13280_e12686;

        let (assign13290_e12692, assign13290_e12692_d_n4, assign13290_e12692_d_n6, assign13290_e12692_d_n7, assign13290_e12692_d_n8, assign13290_e12692_d_n9,) = {
    if (locals.var_guard570 != 0.0) {
        let assign13290_e12690: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign13290_e12690, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign13290_e12692;
        locals.var_q_expnum_dn4 = assign13290_e12692_d_n4;
        locals.var_q_expnum_dn6 = assign13290_e12692_d_n6;
        locals.var_q_expnum_dn7 = assign13290_e12692_d_n7;
        locals.var_q_expnum_dn8 = assign13290_e12692_d_n8;
        locals.var_q_expnum_dn9 = assign13290_e12692_d_n9;

        let (assign13300_e12698, assign13300_e12698_d_n4, assign13300_e12698_d_n6, assign13300_e12698_d_n7, assign13300_e12698_d_n8, assign13300_e12698_d_n9,) = {
    if (locals.var_guard570 != 0.0) {
        let assign13300_e12696: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign13300_e12696, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign13300_e12698;
        locals.var_q_d1_expnum_dn4 = assign13300_e12698_d_n4;
        locals.var_q_d1_expnum_dn6 = assign13300_e12698_d_n6;
        locals.var_q_d1_expnum_dn7 = assign13300_e12698_d_n7;
        locals.var_q_d1_expnum_dn8 = assign13300_e12698_d_n8;
        locals.var_q_d1_expnum_dn9 = assign13300_e12698_d_n9;

        let (assign13310_e12702, assign13310_e12702_d_n4, assign13310_e12702_d_n6, assign13310_e12702_d_n7, assign13310_e12702_d_n8, assign13310_e12702_d_n9,) = {
    if (locals.var_guard570 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign13310_e12702;
        locals.var_q_d2_expnum_dn4 = assign13310_e12702_d_n4;
        locals.var_q_d2_expnum_dn6 = assign13310_e12702_d_n6;
        locals.var_q_d2_expnum_dn7 = assign13310_e12702_d_n7;
        locals.var_q_d2_expnum_dn8 = assign13310_e12702_d_n8;
        locals.var_q_d2_expnum_dn9 = assign13310_e12702_d_n9;

        let (assign13320_e12711, assign13320_e12711_d_n4, assign13320_e12711_d_n6, assign13320_e12711_d_n7, assign13320_e12711_d_n8, assign13320_e12711_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13320_e12708: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign13320_e12709: f64 = (1.0 / assign13320_e12708);
        (assign13320_e12709, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign13320_e12708 * assign13320_e12708))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign13320_e12708 * assign13320_e12708))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign13320_e12708 * assign13320_e12708))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign13320_e12708 * assign13320_e12708))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign13320_e12708 * assign13320_e12708))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13320_e12711;
        locals.var_q_temp2_dn4 = assign13320_e12711_d_n4;
        locals.var_q_temp2_dn6 = assign13320_e12711_d_n6;
        locals.var_q_temp2_dn7 = assign13320_e12711_d_n7;
        locals.var_q_temp2_dn8 = assign13320_e12711_d_n8;
        locals.var_q_temp2_dn9 = assign13320_e12711_d_n9;

        let (assign13330_e12718, assign13330_e12718_d_n4, assign13330_e12718_d_n6, assign13330_e12718_d_n7, assign13330_e12718_d_n8, assign13330_e12718_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13330_e12716: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign13330_e12716, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign13330_e12718;
        locals.var_q_temp3_dn4 = assign13330_e12718_d_n4;
        locals.var_q_temp3_dn6 = assign13330_e12718_d_n6;
        locals.var_q_temp3_dn7 = assign13330_e12718_d_n7;
        locals.var_q_temp3_dn8 = assign13330_e12718_d_n8;
        locals.var_q_temp3_dn9 = assign13330_e12718_d_n9;

        let (assign13340_e12727, assign13340_e12727_d_n4, assign13340_e12727_d_n6, assign13340_e12727_d_n7, assign13340_e12727_d_n8, assign13340_e12727_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13340_e12723: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign13340_e12725: f64 = (assign13340_e12723 * locals.var_q_temp2);
        (assign13340_e12725, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign13340_e12727;
        locals.var_q_expnum_dn4 = assign13340_e12727_d_n4;
        locals.var_q_expnum_dn6 = assign13340_e12727_d_n6;
        locals.var_q_expnum_dn7 = assign13340_e12727_d_n7;
        locals.var_q_expnum_dn8 = assign13340_e12727_d_n8;
        locals.var_q_expnum_dn9 = assign13340_e12727_d_n9;

        let (assign13350_e12742, assign13350_e12742_d_n4, assign13350_e12742_d_n6, assign13350_e12742_d_n7, assign13350_e12742_d_n8, assign13350_e12742_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13350_e12732: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign13350_e12734: f64 = (assign13350_e12732 - locals.var_q_aexp);
        let assign13350_e12737: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign13350_e12738: f64 = (assign13350_e12734 - assign13350_e12737);
        let assign13350_e12740: f64 = (assign13350_e12738 * locals.var_q_temp2);
        (assign13350_e12740, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign13350_e12742;
        locals.var_q_d1_expnum_dn4 = assign13350_e12742_d_n4;
        locals.var_q_d1_expnum_dn6 = assign13350_e12742_d_n6;
        locals.var_q_d1_expnum_dn7 = assign13350_e12742_d_n7;
        locals.var_q_d1_expnum_dn8 = assign13350_e12742_d_n8;
        locals.var_q_d1_expnum_dn9 = assign13350_e12742_d_n9;

        let (assign13360_e12767, assign13360_e12767_d_n4, assign13360_e12767_d_n6, assign13360_e12767_d_n7, assign13360_e12767_d_n8, assign13360_e12767_d_n9,) = {
    if (locals.var_guard570 == 0.0) {
        let assign13360_e12747: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign13360_e12750: f64 = (2.0 * locals.var_q_temp3);
        let assign13360_e12752: f64 = (assign13360_e12750 * locals.var_q_d1_expnum);
        let assign13360_e12753: f64 = (assign13360_e12747 + assign13360_e12752);
        let assign13360_e12755: f64 = (assign13360_e12753 + locals.var_q_aexp);
        let assign13360_e12759: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign13360_e12760: f64 = (locals.var_q_d2_ln + assign13360_e12759);
        let assign13360_e12762: f64 = (assign13360_e12760 * locals.var_q_sh_term);
        let assign13360_e12763: f64 = (assign13360_e12755 - assign13360_e12762);
        let assign13360_e12765: f64 = (assign13360_e12763 * locals.var_q_temp2);
        (assign13360_e12765, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign13360_e12767;
        locals.var_q_d2_expnum_dn4 = assign13360_e12767_d_n4;
        locals.var_q_d2_expnum_dn6 = assign13360_e12767_d_n6;
        locals.var_q_d2_expnum_dn7 = assign13360_e12767_d_n7;
        locals.var_q_d2_expnum_dn8 = assign13360_e12767_d_n8;
        locals.var_q_d2_expnum_dn9 = assign13360_e12767_d_n9;

        let assign13370_e12770: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign13370_e12770;

        let (assign13380_e12775, assign13380_e12775_d_n4, assign13380_e12775_d_n6, assign13380_e12775_d_n7, assign13380_e12775_d_n8, assign13380_e12775_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13380_e12773: f64 = (locals.var_q_expnum).ln();
        (assign13380_e12773, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign13380_e12775;
        locals.var_q_lnexpnum_dn4 = assign13380_e12775_d_n4;
        locals.var_q_lnexpnum_dn6 = assign13380_e12775_d_n6;
        locals.var_q_lnexpnum_dn7 = assign13380_e12775_d_n7;
        locals.var_q_lnexpnum_dn8 = assign13380_e12775_d_n8;
        locals.var_q_lnexpnum_dn9 = assign13380_e12775_d_n9;

        let (assign13390_e12781, assign13390_e12781_d_n4, assign13390_e12781_d_n6, assign13390_e12781_d_n7, assign13390_e12781_d_n8, assign13390_e12781_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13390_e12779: f64 = (1.0 / locals.var_q_expnum);
        (assign13390_e12779, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13390_e12781;
        locals.var_q_temp1_dn4 = assign13390_e12781_d_n4;
        locals.var_q_temp1_dn6 = assign13390_e12781_d_n6;
        locals.var_q_temp1_dn7 = assign13390_e12781_d_n7;
        locals.var_q_temp1_dn8 = assign13390_e12781_d_n8;
        locals.var_q_temp1_dn9 = assign13390_e12781_d_n9;

        let (assign13400_e12787, assign13400_e12787_d_n4, assign13400_e12787_d_n6, assign13400_e12787_d_n7, assign13400_e12787_d_n8, assign13400_e12787_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13400_e12785: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign13400_e12785, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign13400_e12787;
        locals.var_q_d1_lnexpnum_dn4 = assign13400_e12787_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign13400_e12787_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign13400_e12787_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign13400_e12787_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign13400_e12787_d_n9;

        let (assign13410_e12797, assign13410_e12797_d_n4, assign13410_e12797_d_n6, assign13410_e12797_d_n7, assign13410_e12797_d_n8, assign13410_e12797_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13410_e12791: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign13410_e12794: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign13410_e12795: f64 = (assign13410_e12791 - assign13410_e12794);
        (assign13410_e12795, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign13410_e12797;
        locals.var_q_d2_lnexpnum_dn4 = assign13410_e12797_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign13410_e12797_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign13410_e12797_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign13410_e12797_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign13410_e12797_d_n9;

        let (assign13420_e12808, assign13420_e12808_d_n4, assign13420_e12808_d_n6, assign13420_e12808_d_n7, assign13420_e12808_d_n8, assign13420_e12808_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13420_e12802: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign13420_e12804: f64 = (-locals.var_q_k1q1);
        let assign13420_e12805: f64 = (assign13420_e12804).ln();
        let assign13420_e12806: f64 = (assign13420_e12802 + assign13420_e12805);
        (assign13420_e12806, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign13420_e12804)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign13420_e12804)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign13420_e12804)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign13420_e12804)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign13420_e12804)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign13420_e12808;
        locals.var_q_lnexpnum_dn4 = assign13420_e12808_d_n4;
        locals.var_q_lnexpnum_dn6 = assign13420_e12808_d_n6;
        locals.var_q_lnexpnum_dn7 = assign13420_e12808_d_n7;
        locals.var_q_lnexpnum_dn8 = assign13420_e12808_d_n8;
        locals.var_q_lnexpnum_dn9 = assign13420_e12808_d_n9;

        let (assign13430_e12815, assign13430_e12815_d_n4, assign13430_e12815_d_n6, assign13430_e12815_d_n7, assign13430_e12815_d_n8, assign13430_e12815_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13430_e12813: f64 = (1.0 / locals.var_q1s);
        (assign13430_e12813, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13430_e12815;
        locals.var_q_temp1_dn4 = assign13430_e12815_d_n4;
        locals.var_q_temp1_dn6 = assign13430_e12815_d_n6;
        locals.var_q_temp1_dn7 = assign13430_e12815_d_n7;
        locals.var_q_temp1_dn8 = assign13430_e12815_d_n8;
        locals.var_q_temp1_dn9 = assign13430_e12815_d_n9;

        let (assign13440_e12822, assign13440_e12822_d_n4, assign13440_e12822_d_n6, assign13440_e12822_d_n7, assign13440_e12822_d_n8, assign13440_e12822_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13440_e12820: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign13440_e12820, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign13440_e12822;
        locals.var_q_d1_lnexpnum_dn4 = assign13440_e12822_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign13440_e12822_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign13440_e12822_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign13440_e12822_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign13440_e12822_d_n9;

        let (assign13450_e12830, assign13450_e12830_d_n4, assign13450_e12830_d_n6, assign13450_e12830_d_n7, assign13450_e12830_d_n8, assign13450_e12830_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13450_e12826: f64 = (-locals.var_q_temp1);
        let assign13450_e12828: f64 = (assign13450_e12826 * locals.var_q_temp1);
        (assign13450_e12828, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign13450_e12830;
        locals.var_q_d2_lnexpnum_dn4 = assign13450_e12830_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign13450_e12830_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign13450_e12830_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign13450_e12830_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign13450_e12830_d_n9;

        let assign13460_e12833: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign13460_e12835: f64 = (assign13460_e12833 + locals.var_q1s);
        let assign13460_e12838: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign13460_e12839: f64 = (assign13460_e12835 + assign13460_e12838);
        let assign13460_e12841: f64 = (assign13460_e12839 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign13460_e12841;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign13470_e12845: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign13470_e12846: f64 = (1.0 + assign13470_e12845);
        let assign13470_e12848: f64 = (assign13470_e12846 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign13470_e12848;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign13480_e12851: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign13480_e12853: f64 = (assign13480_e12851 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign13480_e12853;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign13490_e12857: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign13490_e12858: f64 = (locals.var_q_k1q1 + assign13490_e12857);
        locals.var_q_qi_int = assign13490_e12858;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

    }
}

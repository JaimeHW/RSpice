#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10960_e5880, assign10960_e5880_d_n0, assign10960_e5880_d_n2, assign10960_e5880_d_n4, assign10960_e5880_d_n5, assign10960_e5880_d_n6, assign10960_e5880_d_n7, assign10960_e5880_d_n8, assign10960_e5880_d_n9, assign10960_e5880_d_n10, assign10960_e5880_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10960_e5870: f64 = (locals.var_t1 * locals.var_t2);
        let assign10960_e5871: f64 = (1.0 + assign10960_e5870);
        let assign10960_e5872: f64 = (locals.var_mueph * assign10960_e5871);
        let assign10960_e5876: f64 = (locals.var_t1 * locals.var_t3);
        let assign10960_e5877: f64 = (1.0 + assign10960_e5876);
        let assign10960_e5878: f64 = (assign10960_e5872 / assign10960_e5877);
        (assign10960_e5878, (((((locals.var_mueph_dn0 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn2 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn4 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn5 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn6 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn7 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn8 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn9 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn10 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn13 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)))) / (assign10960_e5877 * assign10960_e5877)),)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn13,)
    }
};
        locals.var_mueph = assign10960_e5880;
        locals.var_mueph_dn0 = assign10960_e5880_d_n0;
        locals.var_mueph_dn2 = assign10960_e5880_d_n2;
        locals.var_mueph_dn4 = assign10960_e5880_d_n4;
        locals.var_mueph_dn5 = assign10960_e5880_d_n5;
        locals.var_mueph_dn6 = assign10960_e5880_d_n6;
        locals.var_mueph_dn7 = assign10960_e5880_d_n7;
        locals.var_mueph_dn8 = assign10960_e5880_d_n8;
        locals.var_mueph_dn9 = assign10960_e5880_d_n9;
        locals.var_mueph_dn10 = assign10960_e5880_d_n10;
        locals.var_mueph_dn13 = assign10960_e5880_d_n13;

        let assign10970_e5886: f64 = (locals.var_lg).powf(p.p176);
        let assign10970_e5887: f64 = (p.p173 / assign10970_e5886);
        let assign10970_e5888: f64 = (1.0 + assign10970_e5887);
        let assign10970_e5889: f64 = (p.p171 * assign10970_e5888);
        let assign10970_e5894: f64 = (locals.var_wg).powf(p.p175);
        let assign10970_e5895: f64 = (p.p174 / assign10970_e5894);
        let assign10970_e5896: f64 = (1.0 + assign10970_e5895);
        let assign10970_e5897: f64 = (assign10970_e5889 * assign10970_e5896);
        locals.var_muesr = assign10970_e5897;

        let (assign11000_e5921, assign11000_e5921_d_n0, assign11000_e5921_d_n2, assign11000_e5921_d_n4, assign11000_e5921_d_n5, assign11000_e5921_d_n6, assign11000_e5921_d_n7, assign11000_e5921_d_n8, assign11000_e5921_d_n9, assign11000_e5921_d_n10, assign11000_e5921_d_n13,) = {
    if (locals.var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn13,)
    }
};
        locals.var_mueph = assign11000_e5921;
        locals.var_mueph_dn0 = assign11000_e5921_d_n0;
        locals.var_mueph_dn2 = assign11000_e5921_d_n2;
        locals.var_mueph_dn4 = assign11000_e5921_d_n4;
        locals.var_mueph_dn5 = assign11000_e5921_d_n5;
        locals.var_mueph_dn6 = assign11000_e5921_d_n6;
        locals.var_mueph_dn7 = assign11000_e5921_d_n7;
        locals.var_mueph_dn8 = assign11000_e5921_d_n8;
        locals.var_mueph_dn9 = assign11000_e5921_d_n9;
        locals.var_mueph_dn10 = assign11000_e5921_d_n10;
        locals.var_mueph_dn13 = assign11000_e5921_d_n13;

        let (assign11010_e5927,) = {
    if (locals.var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (locals.var_muesr,)
    }
};
        locals.var_muesr = assign11010_e5927;

        let assign11020_e5930: f64 = (locals.var_lg).powf(p.p156);
        locals.var_t1 = assign11020_e5930;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;

        let assign11030_e5933: f64 = (locals.var_uc_ndep * locals.var_t1);
        let assign11030_e5936: f64 = (locals.var_t1 + p.p155);
        let assign11030_e5937: f64 = (assign11030_e5933 / assign11030_e5936);
        let assign11030_e5939: f64 = (assign11030_e5937 / 1.034943e-10);
        locals.var_ndep_o_esi = assign11030_e5939;
        locals.var_ndep_o_esi_dn0 = (((((locals.var_uc_ndep * locals.var_t1_dn0) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn0)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn2 = (((((locals.var_uc_ndep * locals.var_t1_dn2) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn2)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn4 = (((((locals.var_uc_ndep * locals.var_t1_dn4) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn4)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn5 = (((((locals.var_uc_ndep * locals.var_t1_dn5) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn5)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn6 = (((((locals.var_uc_ndep * locals.var_t1_dn6) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn6)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn7 = (((((locals.var_uc_ndep * locals.var_t1_dn7) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn7)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn8 = (((((locals.var_uc_ndep * locals.var_t1_dn8) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn8)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn9 = (((((locals.var_uc_ndep * locals.var_t1_dn9) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn9)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn10 = (((((locals.var_uc_ndep * locals.var_t1_dn10) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn10)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn13 = (((((locals.var_uc_ndep * locals.var_t1_dn13) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn13)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);

        let assign11040_e5942: f64 = (locals.var_uc_ninv / 1.034943e-10);
        locals.var_ninv_o_esi = assign11040_e5942;

        let assign11050_e5948: f64 = (locals.var_lg).powf(p.p321);
        let assign11050_e5949: f64 = (p.p320 / assign11050_e5948);
        let assign11050_e5950: f64 = (1.0 + assign11050_e5949);
        let assign11050_e5951: f64 = (p.p319 * assign11050_e5950);
        let assign11050_e5956: f64 = (locals.var_wg).powf(p.p323);
        let assign11050_e5957: f64 = (p.p322 / assign11050_e5956);
        let assign11050_e5958: f64 = (1.0 + assign11050_e5957);
        let assign11050_e5959: f64 = (assign11050_e5951 * assign11050_e5958);
        locals.var_ninvd0 = assign11050_e5959;

        let assign11060_e5964: f64 = (locals.var_lg).powf(p.p387);
        let assign11060_e5965: f64 = (p.p386 / assign11060_e5964);
        let assign11060_e5966: f64 = (1.0 + assign11060_e5965);
        let assign11060_e5971: f64 = (locals.var_wg).powf(p.p389);
        let assign11060_e5972: f64 = (p.p388 / assign11060_e5971);
        let assign11060_e5973: f64 = (1.0 + assign11060_e5972);
        let assign11060_e5974: f64 = (assign11060_e5966 * assign11060_e5973);
        locals.var_t1 = assign11060_e5974;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;

        let assign11070_e5977: f64 = (p.p384 * locals.var_t1);
        locals.var_ninvd0cres = assign11070_e5977;
        locals.var_ninvd0cres_dn0 = (p.p384 * locals.var_t1_dn0);
        locals.var_ninvd0cres_dn2 = (p.p384 * locals.var_t1_dn2);
        locals.var_ninvd0cres_dn4 = (p.p384 * locals.var_t1_dn4);
        locals.var_ninvd0cres_dn5 = (p.p384 * locals.var_t1_dn5);
        locals.var_ninvd0cres_dn6 = (p.p384 * locals.var_t1_dn6);
        locals.var_ninvd0cres_dn7 = (p.p384 * locals.var_t1_dn7);
        locals.var_ninvd0cres_dn8 = (p.p384 * locals.var_t1_dn8);
        locals.var_ninvd0cres_dn9 = (p.p384 * locals.var_t1_dn9);
        locals.var_ninvd0cres_dn10 = (p.p384 * locals.var_t1_dn10);
        locals.var_ninvd0cres_dn13 = (p.p384 * locals.var_t1_dn13);

        let assign11080_e5980: f64 = (p.p385 * locals.var_t1);
        locals.var_ninvd0hres = assign11080_e5980;
        locals.var_ninvd0hres_dn0 = (p.p385 * locals.var_t1_dn0);
        locals.var_ninvd0hres_dn2 = (p.p385 * locals.var_t1_dn2);
        locals.var_ninvd0hres_dn4 = (p.p385 * locals.var_t1_dn4);
        locals.var_ninvd0hres_dn5 = (p.p385 * locals.var_t1_dn5);
        locals.var_ninvd0hres_dn6 = (p.p385 * locals.var_t1_dn6);
        locals.var_ninvd0hres_dn7 = (p.p385 * locals.var_t1_dn7);
        locals.var_ninvd0hres_dn8 = (p.p385 * locals.var_t1_dn8);
        locals.var_ninvd0hres_dn9 = (p.p385 * locals.var_t1_dn9);
        locals.var_ninvd0hres_dn10 = (p.p385 * locals.var_t1_dn10);
        locals.var_ninvd0hres_dn13 = (p.p385 * locals.var_t1_dn13);

        let assign11090_e5985: f64 = (locals.var_lgate + p.p121);
        let assign11090_e5987: f64 = (assign11090_e5985).powf(p.p122);
        let assign11090_e5988: f64 = (locals.var_mks_ll / assign11090_e5987);
        let assign11090_e5989: f64 = (p.p97 + assign11090_e5988);
        locals.var_dl = assign11090_e5989;

        let assign11100_e5994: f64 = (locals.var_lgate + p.p121);
        let assign11100_e5996: f64 = (assign11100_e5994).powf(p.p122);
        let assign11100_e5997: f64 = (locals.var_mks_ll / assign11100_e5996);
        let assign11100_e5998: f64 = (locals.var_uc_xldld + assign11100_e5997);
        locals.var_dlld = assign11100_e5998;

        let assign11110_e6003: f64 = (locals.var_wgate + p.p128);
        let assign11110_e6005: f64 = (assign11110_e6003).powf(p.p129);
        let assign11110_e6006: f64 = (locals.var_mks_wl / assign11110_e6005);
        let assign11110_e6007: f64 = (p.p114 + assign11110_e6006);
        locals.var_dw = assign11110_e6007;

        let assign11120_e6012: f64 = (locals.var_wgate + p.p128);
        let assign11120_e6014: f64 = (assign11120_e6012).powf(p.p129);
        let assign11120_e6015: f64 = (locals.var_mks_wl / assign11120_e6014);
        let assign11120_e6016: f64 = (p.p295 + assign11120_e6015);
        locals.var_dwld = assign11120_e6016;

        let assign11130_e6021: f64 = (locals.var_wgate + p.p128);
        let assign11130_e6023: f64 = (assign11130_e6021).powf(p.p129);
        let assign11130_e6024: f64 = (locals.var_mks_wl / assign11130_e6023);
        let assign11130_e6025: f64 = (p.p115 + assign11130_e6024);
        locals.var_dwcv = assign11130_e6025;

        let assign11140_e6029: f64 = (locals.var_dl + locals.var_dlld);
        let assign11140_e6030: f64 = (locals.var_lgate - assign11140_e6029);
        locals.var_leff = assign11140_e6030;

        let assign11170_e6042: f64 = (locals.var_wlg).powf(p.p125);
        let assign11170_e6043: f64 = (p.p124 / assign11170_e6042);
        let assign11170_e6044: f64 = (locals.var_lgate + assign11170_e6043);
        locals.var_lgatesm = assign11170_e6044;

        let assign11180_e6048: f64 = (locals.var_wlg).powf(p.p127);
        let assign11180_e6049: f64 = (locals.var_uc_wl2 / assign11180_e6048);
        locals.var_dvthsm = assign11180_e6049;

        let assign11190_e6054: f64 = (locals.var_lgatesm * 1000000.0);
        let assign11190_e6056: f64 = (assign11190_e6054).powf(p.p207);
        let assign11190_e6057: f64 = (p.p206 / assign11190_e6056);
        let assign11190_e6058: f64 = (1.0 + assign11190_e6057);
        locals.var_t1 = assign11190_e6058;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;

        let assign11200_e6063: f64 = (locals.var_wg).powf(p.p209);
        let assign11200_e6064: f64 = (p.p208 / assign11200_e6063);
        let assign11200_e6065: f64 = (1.0 + assign11200_e6064);
        locals.var_t2 = assign11200_e6065;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn13 = 0.0;

        let assign11210_e6068: f64 = (locals.var_uc_wsti * locals.var_t1);
        let assign11210_e6070: f64 = (assign11210_e6068 * locals.var_t2);
        locals.var_uc_wsti = assign11210_e6070;
        locals.var_uc_wsti_dn0 = ((((locals.var_uc_wsti_dn0 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn0)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn0));
        locals.var_uc_wsti_dn2 = ((((locals.var_uc_wsti_dn2 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn2)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn2));
        locals.var_uc_wsti_dn4 = ((((locals.var_uc_wsti_dn4 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn4)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn4));
        locals.var_uc_wsti_dn5 = ((((locals.var_uc_wsti_dn5 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn5)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn5));
        locals.var_uc_wsti_dn6 = ((((locals.var_uc_wsti_dn6 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn6)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn6));
        locals.var_uc_wsti_dn7 = ((((locals.var_uc_wsti_dn7 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn7)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn7));
        locals.var_uc_wsti_dn8 = ((((locals.var_uc_wsti_dn8 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn8)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn8));
        locals.var_uc_wsti_dn9 = ((((locals.var_uc_wsti_dn9 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn9)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn9));
        locals.var_uc_wsti_dn10 = ((((locals.var_uc_wsti_dn10 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn10)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn10));
        locals.var_uc_wsti_dn13 = ((((locals.var_uc_wsti_dn13 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn13)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn13));

        let assign11220_e6074: f64 = (2.0 * locals.var_dw);
        let assign11220_e6075: f64 = (locals.var_wgate - assign11220_e6074);
        locals.var_weff = assign11220_e6075;

        let assign11230_e6079: f64 = (2.0 * locals.var_dwld);
        let assign11230_e6080: f64 = (locals.var_wgate - assign11230_e6079);
        locals.var_weff_ld = assign11230_e6080;

        let assign11240_e6084: f64 = (2.0 * locals.var_dwcv);
        let assign11240_e6085: f64 = (locals.var_wgate - assign11240_e6084);
        locals.var_weff_cv = assign11240_e6085;

        let assign11310_e6109: f64 = (locals.var_weff * p.p7);
        locals.var_weff_nf = assign11310_e6109;

        let assign11320_e6112: f64 = (locals.var_weff_cv * p.p7);
        locals.var_weffcv_nf = assign11320_e6112;

        let assign11330_e6118: f64 = (locals.var_wg).powf(p.p143);
        let assign11330_e6119: f64 = (p.p142 / assign11330_e6118);
        let assign11330_e6120: f64 = (1.0 + assign11330_e6119);
        let assign11330_e6121: f64 = (locals.var_ef_nsubp * assign11330_e6120);
        locals.var_nsubpp = assign11330_e6121;
        locals.var_nsubpp_dn0 = (locals.var_ef_nsubp_dn0 * assign11330_e6120);
        locals.var_nsubpp_dn2 = (locals.var_ef_nsubp_dn2 * assign11330_e6120);
        locals.var_nsubpp_dn4 = (locals.var_ef_nsubp_dn4 * assign11330_e6120);
        locals.var_nsubpp_dn5 = (locals.var_ef_nsubp_dn5 * assign11330_e6120);
        locals.var_nsubpp_dn6 = (locals.var_ef_nsubp_dn6 * assign11330_e6120);
        locals.var_nsubpp_dn7 = (locals.var_ef_nsubp_dn7 * assign11330_e6120);
        locals.var_nsubpp_dn8 = (locals.var_ef_nsubp_dn8 * assign11330_e6120);
        locals.var_nsubpp_dn9 = (locals.var_ef_nsubp_dn9 * assign11330_e6120);
        locals.var_nsubpp_dn10 = (locals.var_ef_nsubp_dn10 * assign11330_e6120);
        locals.var_nsubpp_dn13 = (locals.var_ef_nsubp_dn13 * assign11330_e6120);

        let assign11340_e6127: f64 = (locals.var_wg).powf(p.p234);
        let assign11340_e6128: f64 = (p.p233 / assign11340_e6127);
        let assign11340_e6129: f64 = (1.0 + assign11340_e6128);
        let assign11340_e6130: f64 = (locals.var_ef_nsubc * assign11340_e6129);
        locals.var_ef_nsubc = assign11340_e6130;
        locals.var_ef_nsubc_dn0 = (locals.var_ef_nsubc_dn0 * assign11340_e6129);
        locals.var_ef_nsubc_dn2 = (locals.var_ef_nsubc_dn2 * assign11340_e6129);
        locals.var_ef_nsubc_dn4 = (locals.var_ef_nsubc_dn4 * assign11340_e6129);
        locals.var_ef_nsubc_dn5 = (locals.var_ef_nsubc_dn5 * assign11340_e6129);
        locals.var_ef_nsubc_dn6 = (locals.var_ef_nsubc_dn6 * assign11340_e6129);
        locals.var_ef_nsubc_dn7 = (locals.var_ef_nsubc_dn7 * assign11340_e6129);
        locals.var_ef_nsubc_dn8 = (locals.var_ef_nsubc_dn8 * assign11340_e6129);
        locals.var_ef_nsubc_dn9 = (locals.var_ef_nsubc_dn9 * assign11340_e6129);
        locals.var_ef_nsubc_dn10 = (locals.var_ef_nsubc_dn10 * assign11340_e6129);
        locals.var_ef_nsubc_dn13 = (locals.var_ef_nsubc_dn13 * assign11340_e6129);

        let assign11350_e6133: f64 = (locals.var_ef_nsubc * 1e-6);
        locals.var_t1 = assign11350_e6133;
        locals.var_t1_dn0 = (locals.var_ef_nsubc_dn0 * 1e-6);
        locals.var_t1_dn2 = (locals.var_ef_nsubc_dn2 * 1e-6);
        locals.var_t1_dn4 = (locals.var_ef_nsubc_dn4 * 1e-6);
        locals.var_t1_dn5 = (locals.var_ef_nsubc_dn5 * 1e-6);
        locals.var_t1_dn6 = (locals.var_ef_nsubc_dn6 * 1e-6);
        locals.var_t1_dn7 = (locals.var_ef_nsubc_dn7 * 1e-6);
        locals.var_t1_dn8 = (locals.var_ef_nsubc_dn8 * 1e-6);
        locals.var_t1_dn9 = (locals.var_ef_nsubc_dn9 * 1e-6);
        locals.var_t1_dn10 = (locals.var_ef_nsubc_dn10 * 1e-6);
        locals.var_t1_dn13 = (locals.var_ef_nsubc_dn13 * 1e-6);

        let assign11360_e6136: f64 = (locals.var_nsubpp * 1e-6);
        locals.var_t2 = assign11360_e6136;
        locals.var_t2_dn0 = (locals.var_nsubpp_dn0 * 1e-6);
        locals.var_t2_dn2 = (locals.var_nsubpp_dn2 * 1e-6);
        locals.var_t2_dn4 = (locals.var_nsubpp_dn4 * 1e-6);
        locals.var_t2_dn5 = (locals.var_nsubpp_dn5 * 1e-6);
        locals.var_t2_dn6 = (locals.var_nsubpp_dn6 * 1e-6);
        locals.var_t2_dn7 = (locals.var_nsubpp_dn7 * 1e-6);
        locals.var_t2_dn8 = (locals.var_nsubpp_dn8 * 1e-6);
        locals.var_t2_dn9 = (locals.var_nsubpp_dn9 * 1e-6);
        locals.var_t2_dn10 = (locals.var_nsubpp_dn10 * 1e-6);
        locals.var_t2_dn13 = (locals.var_nsubpp_dn13 * 1e-6);

        let assign11380_e6144: f64 = if locals.var_t1 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign11380_e6144;

        let (assign11390_e6148, assign11390_e6148_d_n0, assign11390_e6148_d_n2, assign11390_e6148_d_n4, assign11390_e6148_d_n5, assign11390_e6148_d_n6, assign11390_e6148_d_n7, assign11390_e6148_d_n8, assign11390_e6148_d_n9, assign11390_e6148_d_n10, assign11390_e6148_d_n13,) = {
    if (locals.var_guard263 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign11390_e6148;
        locals.var_t1_dn0 = assign11390_e6148_d_n0;
        locals.var_t1_dn2 = assign11390_e6148_d_n2;
        locals.var_t1_dn4 = assign11390_e6148_d_n4;
        locals.var_t1_dn5 = assign11390_e6148_d_n5;
        locals.var_t1_dn6 = assign11390_e6148_d_n6;
        locals.var_t1_dn7 = assign11390_e6148_d_n7;
        locals.var_t1_dn8 = assign11390_e6148_d_n8;
        locals.var_t1_dn9 = assign11390_e6148_d_n9;
        locals.var_t1_dn10 = assign11390_e6148_d_n10;
        locals.var_t1_dn13 = assign11390_e6148_d_n13;

        let assign11400_e6151: f64 = (locals.var_t1 / 1e-6);
        locals.var_ef_nsubc = assign11400_e6151;
        locals.var_ef_nsubc_dn0 = (locals.var_t1_dn0 / 1e-6);
        locals.var_ef_nsubc_dn2 = (locals.var_t1_dn2 / 1e-6);
        locals.var_ef_nsubc_dn4 = (locals.var_t1_dn4 / 1e-6);
        locals.var_ef_nsubc_dn5 = (locals.var_t1_dn5 / 1e-6);
        locals.var_ef_nsubc_dn6 = (locals.var_t1_dn6 / 1e-6);
        locals.var_ef_nsubc_dn7 = (locals.var_t1_dn7 / 1e-6);
        locals.var_ef_nsubc_dn8 = (locals.var_t1_dn8 / 1e-6);
        locals.var_ef_nsubc_dn9 = (locals.var_t1_dn9 / 1e-6);
        locals.var_ef_nsubc_dn10 = (locals.var_t1_dn10 / 1e-6);
        locals.var_ef_nsubc_dn13 = (locals.var_t1_dn13 / 1e-6);

        let assign11420_e6159: f64 = if locals.var_t2 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign11420_e6159;

        let (assign11430_e6163, assign11430_e6163_d_n0, assign11430_e6163_d_n2, assign11430_e6163_d_n4, assign11430_e6163_d_n5, assign11430_e6163_d_n6, assign11430_e6163_d_n7, assign11430_e6163_d_n8, assign11430_e6163_d_n9, assign11430_e6163_d_n10, assign11430_e6163_d_n13,) = {
    if (locals.var_guard265 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign11430_e6163;
        locals.var_t2_dn0 = assign11430_e6163_d_n0;
        locals.var_t2_dn2 = assign11430_e6163_d_n2;
        locals.var_t2_dn4 = assign11430_e6163_d_n4;
        locals.var_t2_dn5 = assign11430_e6163_d_n5;
        locals.var_t2_dn6 = assign11430_e6163_d_n6;
        locals.var_t2_dn7 = assign11430_e6163_d_n7;
        locals.var_t2_dn8 = assign11430_e6163_d_n8;
        locals.var_t2_dn9 = assign11430_e6163_d_n9;
        locals.var_t2_dn10 = assign11430_e6163_d_n10;
        locals.var_t2_dn13 = assign11430_e6163_d_n13;

        let assign11440_e6166: f64 = (locals.var_t2 / 1e-6);
        locals.var_nsubpp = assign11440_e6166;
        locals.var_nsubpp_dn0 = (locals.var_t2_dn0 / 1e-6);
        locals.var_nsubpp_dn2 = (locals.var_t2_dn2 / 1e-6);
        locals.var_nsubpp_dn4 = (locals.var_t2_dn4 / 1e-6);
        locals.var_nsubpp_dn5 = (locals.var_t2_dn5 / 1e-6);
        locals.var_nsubpp_dn6 = (locals.var_t2_dn6 / 1e-6);
        locals.var_nsubpp_dn7 = (locals.var_t2_dn7 / 1e-6);
        locals.var_nsubpp_dn8 = (locals.var_t2_dn8 / 1e-6);
        locals.var_nsubpp_dn9 = (locals.var_t2_dn9 / 1e-6);
        locals.var_nsubpp_dn10 = (locals.var_t2_dn10 / 1e-6);
        locals.var_nsubpp_dn13 = (locals.var_t2_dn13 / 1e-6);

        let assign11450_e6169: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign11450_e6169;

        let (assign11460_e6177, assign11460_e6177_d_n0, assign11460_e6177_d_n2, assign11460_e6177_d_n4, assign11460_e6177_d_n5, assign11460_e6177_d_n6, assign11460_e6177_d_n7, assign11460_e6177_d_n8, assign11460_e6177_d_n9, assign11460_e6177_d_n10, assign11460_e6177_d_n13,) = {
    if (locals.var_guard266 != 0.0) {
        let assign11460_e6174: f64 = (1.0 + locals.var_uc_nsubpsti2);
        let assign11460_e6175: f64 = (1.0 / assign11460_e6174);
        (assign11460_e6175, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign11460_e6177;
        locals.var_t1_dn0 = assign11460_e6177_d_n0;
        locals.var_t1_dn2 = assign11460_e6177_d_n2;
        locals.var_t1_dn4 = assign11460_e6177_d_n4;
        locals.var_t1_dn5 = assign11460_e6177_d_n5;
        locals.var_t1_dn6 = assign11460_e6177_d_n6;
        locals.var_t1_dn7 = assign11460_e6177_d_n7;
        locals.var_t1_dn8 = assign11460_e6177_d_n8;
        locals.var_t1_dn9 = assign11460_e6177_d_n9;
        locals.var_t1_dn10 = assign11460_e6177_d_n10;
        locals.var_t1_dn13 = assign11460_e6177_d_n13;

        let (assign11470_e6185, assign11470_e6185_d_n0, assign11470_e6185_d_n2, assign11470_e6185_d_n4, assign11470_e6185_d_n5, assign11470_e6185_d_n6, assign11470_e6185_d_n7, assign11470_e6185_d_n8, assign11470_e6185_d_n9, assign11470_e6185_d_n10, assign11470_e6185_d_n13,) = {
    if (locals.var_guard266 != 0.0) {
        let assign11470_e6181: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half);
        let assign11470_e6183: f64 = (assign11470_e6181).powf(locals.var_uc_nsubpsti3);
        (assign11470_e6183, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn13) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn13) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign11470_e6185;
        locals.var_t2_dn0 = assign11470_e6185_d_n0;
        locals.var_t2_dn2 = assign11470_e6185_d_n2;
        locals.var_t2_dn4 = assign11470_e6185_d_n4;
        locals.var_t2_dn5 = assign11470_e6185_d_n5;
        locals.var_t2_dn6 = assign11470_e6185_d_n6;
        locals.var_t2_dn7 = assign11470_e6185_d_n7;
        locals.var_t2_dn8 = assign11470_e6185_d_n8;
        locals.var_t2_dn9 = assign11470_e6185_d_n9;
        locals.var_t2_dn10 = assign11470_e6185_d_n10;
        locals.var_t2_dn13 = assign11470_e6185_d_n13;

        let (assign11480_e6193, assign11480_e6193_d_n0, assign11480_e6193_d_n2, assign11480_e6193_d_n4, assign11480_e6193_d_n5, assign11480_e6193_d_n6, assign11480_e6193_d_n7, assign11480_e6193_d_n8, assign11480_e6193_d_n9, assign11480_e6193_d_n10, assign11480_e6193_d_n13,) = {
    if (locals.var_guard266 != 0.0) {
        let assign11480_e6189: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half_ref);
        let assign11480_e6191: f64 = (assign11480_e6189).powf(locals.var_uc_nsubpsti3);
        (assign11480_e6191, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn13) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn13) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign11480_e6193;
        locals.var_t3_dn0 = assign11480_e6193_d_n0;
        locals.var_t3_dn2 = assign11480_e6193_d_n2;
        locals.var_t3_dn4 = assign11480_e6193_d_n4;
        locals.var_t3_dn5 = assign11480_e6193_d_n5;
        locals.var_t3_dn6 = assign11480_e6193_d_n6;
        locals.var_t3_dn7 = assign11480_e6193_d_n7;
        locals.var_t3_dn8 = assign11480_e6193_d_n8;
        locals.var_t3_dn9 = assign11480_e6193_d_n9;
        locals.var_t3_dn10 = assign11480_e6193_d_n10;
        locals.var_t3_dn13 = assign11480_e6193_d_n13;

        let (assign11490_e6209, assign11490_e6209_d_n0, assign11490_e6209_d_n2, assign11490_e6209_d_n4, assign11490_e6209_d_n5, assign11490_e6209_d_n6, assign11490_e6209_d_n7, assign11490_e6209_d_n8, assign11490_e6209_d_n9, assign11490_e6209_d_n10, assign11490_e6209_d_n13,) = {
    if (locals.var_guard266 != 0.0) {
        let assign11490_e6199: f64 = (locals.var_t1 * locals.var_t2);
        let assign11490_e6200: f64 = (1.0 + assign11490_e6199);
        let assign11490_e6201: f64 = (locals.var_nsubpp * assign11490_e6200);
        let assign11490_e6205: f64 = (locals.var_t1 * locals.var_t3);
        let assign11490_e6206: f64 = (1.0 + assign11490_e6205);
        let assign11490_e6207: f64 = (assign11490_e6201 / assign11490_e6206);
        (assign11490_e6207, (((((locals.var_nsubpp_dn0 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn2 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn4 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn5 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn6 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn7 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn8 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn9 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn10 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn13 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)))) / (assign11490_e6206 * assign11490_e6206)),)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn13,)
    }
};
        locals.var_nsubps = assign11490_e6209;
        locals.var_nsubps_dn0 = assign11490_e6209_d_n0;
        locals.var_nsubps_dn2 = assign11490_e6209_d_n2;
        locals.var_nsubps_dn4 = assign11490_e6209_d_n4;
        locals.var_nsubps_dn5 = assign11490_e6209_d_n5;
        locals.var_nsubps_dn6 = assign11490_e6209_d_n6;
        locals.var_nsubps_dn7 = assign11490_e6209_d_n7;
        locals.var_nsubps_dn8 = assign11490_e6209_d_n8;
        locals.var_nsubps_dn9 = assign11490_e6209_d_n9;
        locals.var_nsubps_dn10 = assign11490_e6209_d_n10;
        locals.var_nsubps_dn13 = assign11490_e6209_d_n13;

        let (assign11500_e6214, assign11500_e6214_d_n0, assign11500_e6214_d_n2, assign11500_e6214_d_n4, assign11500_e6214_d_n5, assign11500_e6214_d_n6, assign11500_e6214_d_n7, assign11500_e6214_d_n8, assign11500_e6214_d_n9, assign11500_e6214_d_n10, assign11500_e6214_d_n13,) = {
    if (locals.var_guard266 == 0.0) {
        (locals.var_nsubpp, locals.var_nsubpp_dn0, locals.var_nsubpp_dn2, locals.var_nsubpp_dn4, locals.var_nsubpp_dn5, locals.var_nsubpp_dn6, locals.var_nsubpp_dn7, locals.var_nsubpp_dn8, locals.var_nsubpp_dn9, locals.var_nsubpp_dn10, locals.var_nsubpp_dn13,)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn13,)
    }
};
        locals.var_nsubps = assign11500_e6214;
        locals.var_nsubps_dn0 = assign11500_e6214_d_n0;
        locals.var_nsubps_dn2 = assign11500_e6214_d_n2;
        locals.var_nsubps_dn4 = assign11500_e6214_d_n4;
        locals.var_nsubps_dn5 = assign11500_e6214_d_n5;
        locals.var_nsubps_dn6 = assign11500_e6214_d_n6;
        locals.var_nsubps_dn7 = assign11500_e6214_d_n7;
        locals.var_nsubps_dn8 = assign11500_e6214_d_n8;
        locals.var_nsubps_dn9 = assign11500_e6214_d_n9;
        locals.var_nsubps_dn10 = assign11500_e6214_d_n10;
        locals.var_nsubps_dn13 = assign11500_e6214_d_n13;

        let assign11510_e6221: f64 = if ((locals.var_lgate > p.p140) || (p.p140 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard267 = assign11510_e6221;

    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11520_e6235, assign11520_e6235_d_n0, assign11520_e6235_d_n2, assign11520_e6235_d_n4, assign11520_e6235_d_n5, assign11520_e6235_d_n6, assign11520_e6235_d_n7, assign11520_e6235_d_n8, assign11520_e6235_d_n9, assign11520_e6235_d_n10, assign11520_e6235_d_n13,) = {
    if (locals.var_guard267 != 0.0) {
        let assign11520_e6226: f64 = (locals.var_lgate - p.p140);
        let assign11520_e6227: f64 = (locals.var_ef_nsubc * assign11520_e6226);
        let assign11520_e6230: f64 = (locals.var_nsubps * p.p140);
        let assign11520_e6231: f64 = (assign11520_e6227 + assign11520_e6230);
        let assign11520_e6233: f64 = (assign11520_e6231 / locals.var_lgate);
        (assign11520_e6233, (((locals.var_ef_nsubc_dn0 * assign11520_e6226) + (locals.var_nsubps_dn0 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn2 * assign11520_e6226) + (locals.var_nsubps_dn2 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn4 * assign11520_e6226) + (locals.var_nsubps_dn4 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn5 * assign11520_e6226) + (locals.var_nsubps_dn5 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn6 * assign11520_e6226) + (locals.var_nsubps_dn6 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn7 * assign11520_e6226) + (locals.var_nsubps_dn7 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn8 * assign11520_e6226) + (locals.var_nsubps_dn8 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn9 * assign11520_e6226) + (locals.var_nsubps_dn9 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn10 * assign11520_e6226) + (locals.var_nsubps_dn10 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn13 * assign11520_e6226) + (locals.var_nsubps_dn13 * p.p140)) / locals.var_lgate),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn13,)
    }
};
        locals.var_nsub = assign11520_e6235;
        locals.var_nsub_dn0 = assign11520_e6235_d_n0;
        locals.var_nsub_dn2 = assign11520_e6235_d_n2;
        locals.var_nsub_dn4 = assign11520_e6235_d_n4;
        locals.var_nsub_dn5 = assign11520_e6235_d_n5;
        locals.var_nsub_dn6 = assign11520_e6235_d_n6;
        locals.var_nsub_dn7 = assign11520_e6235_d_n7;
        locals.var_nsub_dn8 = assign11520_e6235_d_n8;
        locals.var_nsub_dn9 = assign11520_e6235_d_n9;
        locals.var_nsub_dn10 = assign11520_e6235_d_n10;
        locals.var_nsub_dn13 = assign11520_e6235_d_n13;

        let (assign11530_e6250, assign11530_e6250_d_n0, assign11530_e6250_d_n2, assign11530_e6250_d_n4, assign11530_e6250_d_n5, assign11530_e6250_d_n6, assign11530_e6250_d_n7, assign11530_e6250_d_n8, assign11530_e6250_d_n9, assign11530_e6250_d_n10, assign11530_e6250_d_n13,) = {
    if (locals.var_guard267 == 0.0) {
        let assign11530_e6241: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11530_e6244: f64 = (p.p140 - locals.var_lgate);
        let assign11530_e6245: f64 = (assign11530_e6241 * assign11530_e6244);
        let assign11530_e6247: f64 = (assign11530_e6245 / p.p140);
        let assign11530_e6248: f64 = (locals.var_nsubps + assign11530_e6247);
        (assign11530_e6248, (locals.var_nsubps_dn0 + (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn2 + (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn4 + (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn5 + (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn6 + (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn7 + (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn8 + (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn9 + (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn10 + (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn13 + (((locals.var_nsubps_dn13 - locals.var_ef_nsubc_dn13) * assign11530_e6244) / p.p140)),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn13,)
    }
};
        locals.var_nsub = assign11530_e6250;
        locals.var_nsub_dn0 = assign11530_e6250_d_n0;
        locals.var_nsub_dn2 = assign11530_e6250_d_n2;
        locals.var_nsub_dn4 = assign11530_e6250_d_n4;
        locals.var_nsub_dn5 = assign11530_e6250_d_n5;
        locals.var_nsub_dn6 = assign11530_e6250_d_n6;
        locals.var_nsub_dn7 = assign11530_e6250_d_n7;
        locals.var_nsub_dn8 = assign11530_e6250_d_n8;
        locals.var_nsub_dn9 = assign11530_e6250_d_n9;
        locals.var_nsub_dn10 = assign11530_e6250_d_n10;
        locals.var_nsub_dn13 = assign11530_e6250_d_n13;

        let assign11540_e6253: f64 = (0.5 * locals.var_lgate);
        let assign11540_e6255: f64 = (assign11540_e6253 - p.p140);
        locals.var_t3 = assign11540_e6255;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn13 = 0.0;

        let assign11550_e6258: f64 = (locals.var_t3 - 1e-9);
        let assign11550_e6260: f64 = (assign11550_e6258 - 1e-10);
        locals.var_tmf1 = assign11550_e6260;
        locals.var_tmf1_dn0 = locals.var_t3_dn0;
        locals.var_tmf1_dn2 = locals.var_t3_dn2;
        locals.var_tmf1_dn4 = locals.var_t3_dn4;
        locals.var_tmf1_dn5 = locals.var_t3_dn5;
        locals.var_tmf1_dn6 = locals.var_t3_dn6;
        locals.var_tmf1_dn7 = locals.var_t3_dn7;
        locals.var_tmf1_dn8 = locals.var_t3_dn8;
        locals.var_tmf1_dn9 = locals.var_t3_dn9;
        locals.var_tmf1_dn10 = locals.var_t3_dn10;
        locals.var_tmf1_dn13 = locals.var_t3_dn13;

        let assign11560_e6263: f64 = (4.0 * 1e-9);
        let assign11560_e6265: f64 = (assign11560_e6263 * 1e-10);
        locals.var_tmf2 = assign11560_e6265;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn13 = 0.0;

        let (assign11570_e6272, assign11570_e6272_d_n0, assign11570_e6272_d_n2, assign11570_e6272_d_n4, assign11570_e6272_d_n5, assign11570_e6272_d_n6, assign11570_e6272_d_n7, assign11570_e6272_d_n8, assign11570_e6272_d_n9, assign11570_e6272_d_n10, assign11570_e6272_d_n13,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    } else {
        let assign11570_e6271: f64 = (-locals.var_tmf2);
        (assign11570_e6271, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
    }
};
        locals.var_tmf2 = assign11570_e6272;
        locals.var_tmf2_dn0 = assign11570_e6272_d_n0;
        locals.var_tmf2_dn2 = assign11570_e6272_d_n2;
        locals.var_tmf2_dn4 = assign11570_e6272_d_n4;
        locals.var_tmf2_dn5 = assign11570_e6272_d_n5;
        locals.var_tmf2_dn6 = assign11570_e6272_d_n6;
        locals.var_tmf2_dn7 = assign11570_e6272_d_n7;
        locals.var_tmf2_dn8 = assign11570_e6272_d_n8;
        locals.var_tmf2_dn9 = assign11570_e6272_d_n9;
        locals.var_tmf2_dn10 = assign11570_e6272_d_n10;
        locals.var_tmf2_dn13 = assign11570_e6272_d_n13;

        let assign11580_e6275: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign11580_e6277: f64 = (assign11580_e6275 + locals.var_tmf2);
        let assign11580_e6278: f64 = (assign11580_e6277).sqrt();
        locals.var_tmf2 = assign11580_e6278;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn13 = ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign11580_e6278));

        let assign11590_e6283: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign11590_e6284: f64 = (1.0 + assign11590_e6283);
        let assign11590_e6285: f64 = (0.5 * assign11590_e6284);
        locals.var_t0 = assign11590_e6285;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn13 = (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)));

        let assign11600_e6290: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign11600_e6291: f64 = (0.5 * assign11600_e6290);
        let assign11600_e6292: f64 = (1e-9 + assign11600_e6291);
        locals.var_t3 = assign11600_e6292;
        locals.var_t3_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_t3_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_t3_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_t3_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_t3_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_t3_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_t3_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_t3_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_t3_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_t3_dn13 = (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13));

        let assign11610_e6296: f64 = (1.0 / locals.var_t3);
        let assign11610_e6299: f64 = (1.0 / p.p220);
        let assign11610_e6300: f64 = (assign11610_e6296 + assign11610_e6299);
        let assign11610_e6301: f64 = (1.0 / assign11610_e6300);
        locals.var_t1 = assign11610_e6301;
        locals.var_t1_dn0 = (-((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn2 = (-((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn4 = (-((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn5 = (-((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn6 = (-((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn7 = (-((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn8 = (-((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn9 = (-((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn10 = (-((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn13 = (-((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));

        let (assign11620_e6307, assign11620_e6307_d_n0, assign11620_e6307_d_n2, assign11620_e6307_d_n4, assign11620_e6307_d_n5, assign11620_e6307_d_n6, assign11620_e6307_d_n7, assign11620_e6307_d_n8, assign11620_e6307_d_n9, assign11620_e6307_d_n10, assign11620_e6307_d_n13,) = {
    if (0.0 >= locals.var_t1) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t2 = assign11620_e6307;
        locals.var_t2_dn0 = assign11620_e6307_d_n0;
        locals.var_t2_dn2 = assign11620_e6307_d_n2;
        locals.var_t2_dn4 = assign11620_e6307_d_n4;
        locals.var_t2_dn5 = assign11620_e6307_d_n5;
        locals.var_t2_dn6 = assign11620_e6307_d_n6;
        locals.var_t2_dn7 = assign11620_e6307_d_n7;
        locals.var_t2_dn8 = assign11620_e6307_d_n8;
        locals.var_t2_dn9 = assign11620_e6307_d_n9;
        locals.var_t2_dn10 = assign11620_e6307_d_n10;
        locals.var_t2_dn13 = assign11620_e6307_d_n13;

        let assign11630_e6312: f64 = (locals.var_npexte - locals.var_ef_nsubc);
        let assign11630_e6313: f64 = (locals.var_t2 * assign11630_e6312);
        let assign11630_e6315: f64 = (assign11630_e6313 / locals.var_lgate);
        let assign11630_e6316: f64 = (locals.var_nsub + assign11630_e6315);
        locals.var_nsub = assign11630_e6316;
        locals.var_nsub_dn0 = (locals.var_nsub_dn0 + (((locals.var_t2_dn0 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0))) / locals.var_lgate));
        locals.var_nsub_dn2 = (locals.var_nsub_dn2 + (((locals.var_t2_dn2 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2))) / locals.var_lgate));
        locals.var_nsub_dn4 = (locals.var_nsub_dn4 + (((locals.var_t2_dn4 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4))) / locals.var_lgate));
        locals.var_nsub_dn5 = (locals.var_nsub_dn5 + (((locals.var_t2_dn5 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5))) / locals.var_lgate));
        locals.var_nsub_dn6 = (locals.var_nsub_dn6 + (((locals.var_t2_dn6 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6))) / locals.var_lgate));
        locals.var_nsub_dn7 = (locals.var_nsub_dn7 + (((locals.var_t2_dn7 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7))) / locals.var_lgate));
        locals.var_nsub_dn8 = (locals.var_nsub_dn8 + (((locals.var_t2_dn8 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8))) / locals.var_lgate));
        locals.var_nsub_dn9 = (locals.var_nsub_dn9 + (((locals.var_t2_dn9 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9))) / locals.var_lgate));
        locals.var_nsub_dn10 = (locals.var_nsub_dn10 + (((locals.var_t2_dn10 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10))) / locals.var_lgate));
        locals.var_nsub_dn13 = (locals.var_nsub_dn13 + (((locals.var_t2_dn13 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn13 - locals.var_ef_nsubc_dn13))) / locals.var_lgate));

        let assign11640_e6319: f64 = (1.6021918e-19 * locals.var_nsub);
        locals.var_q_nsub = assign11640_e6319;
        locals.var_q_nsub_dn0 = (1.6021918e-19 * locals.var_nsub_dn0);
        locals.var_q_nsub_dn2 = (1.6021918e-19 * locals.var_nsub_dn2);
        locals.var_q_nsub_dn4 = (1.6021918e-19 * locals.var_nsub_dn4);
        locals.var_q_nsub_dn5 = (1.6021918e-19 * locals.var_nsub_dn5);
        locals.var_q_nsub_dn6 = (1.6021918e-19 * locals.var_nsub_dn6);
        locals.var_q_nsub_dn7 = (1.6021918e-19 * locals.var_nsub_dn7);
        locals.var_q_nsub_dn8 = (1.6021918e-19 * locals.var_nsub_dn8);
        locals.var_q_nsub_dn9 = (1.6021918e-19 * locals.var_nsub_dn9);
        locals.var_q_nsub_dn10 = (1.6021918e-19 * locals.var_nsub_dn10);
        locals.var_q_nsub_dn13 = (1.6021918e-19 * locals.var_nsub_dn13);

        let assign11650_e6322: f64 = (locals.var_q_nsub * 1.034943e-10);
        locals.var_qnsub_esi = assign11650_e6322;
        locals.var_qnsub_esi_dn0 = (locals.var_q_nsub_dn0 * 1.034943e-10);
        locals.var_qnsub_esi_dn2 = (locals.var_q_nsub_dn2 * 1.034943e-10);
        locals.var_qnsub_esi_dn4 = (locals.var_q_nsub_dn4 * 1.034943e-10);
        locals.var_qnsub_esi_dn5 = (locals.var_q_nsub_dn5 * 1.034943e-10);
        locals.var_qnsub_esi_dn6 = (locals.var_q_nsub_dn6 * 1.034943e-10);
        locals.var_qnsub_esi_dn7 = (locals.var_q_nsub_dn7 * 1.034943e-10);
        locals.var_qnsub_esi_dn8 = (locals.var_q_nsub_dn8 * 1.034943e-10);
        locals.var_qnsub_esi_dn9 = (locals.var_q_nsub_dn9 * 1.034943e-10);
        locals.var_qnsub_esi_dn10 = (locals.var_q_nsub_dn10 * 1.034943e-10);
        locals.var_qnsub_esi_dn13 = (locals.var_q_nsub_dn13 * 1.034943e-10);

        let assign11660_e6325: f64 = (2.0 * locals.var_qnsub_esi);
        locals.var_qnsub_esi2 = assign11660_e6325;
        locals.var_qnsub_esi2_dn0 = (2.0 * locals.var_qnsub_esi_dn0);
        locals.var_qnsub_esi2_dn2 = (2.0 * locals.var_qnsub_esi_dn2);
        locals.var_qnsub_esi2_dn4 = (2.0 * locals.var_qnsub_esi_dn4);
        locals.var_qnsub_esi2_dn5 = (2.0 * locals.var_qnsub_esi_dn5);
        locals.var_qnsub_esi2_dn6 = (2.0 * locals.var_qnsub_esi_dn6);
        locals.var_qnsub_esi2_dn7 = (2.0 * locals.var_qnsub_esi_dn7);
        locals.var_qnsub_esi2_dn8 = (2.0 * locals.var_qnsub_esi_dn8);
        locals.var_qnsub_esi2_dn9 = (2.0 * locals.var_qnsub_esi_dn9);
        locals.var_qnsub_esi2_dn10 = (2.0 * locals.var_qnsub_esi_dn10);
        locals.var_qnsub_esi2_dn13 = (2.0 * locals.var_qnsub_esi_dn13);

        let assign11670_e6329: f64 = (2.0 * p.p140);
        let assign11670_e6334: f64 = if ((locals.var_lgate <= assign11670_e6329) && (p.p140 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard268 = assign11670_e6334;

        let (assign11680_e6350, assign11680_e6350_d_n0, assign11680_e6350_d_n2, assign11680_e6350_d_n4, assign11680_e6350_d_n5, assign11680_e6350_d_n6, assign11680_e6350_d_n7, assign11680_e6350_d_n8, assign11680_e6350_d_n9, assign11680_e6350_d_n10, assign11680_e6350_d_n13,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11680_e6338: f64 = (2.0 * locals.var_nsubps);
        let assign11680_e6341: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11680_e6343: f64 = (assign11680_e6341 * locals.var_lgate);
        let assign11680_e6345: f64 = (assign11680_e6343 / p.p140);
        let assign11680_e6346: f64 = (assign11680_e6338 - assign11680_e6345);
        let assign11680_e6348: f64 = (assign11680_e6346 - locals.var_ef_nsubc);
        (assign11680_e6348, (((2.0 * locals.var_nsubps_dn0) - (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn0), (((2.0 * locals.var_nsubps_dn2) - (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn2), (((2.0 * locals.var_nsubps_dn4) - (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn4), (((2.0 * locals.var_nsubps_dn5) - (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn5), (((2.0 * locals.var_nsubps_dn6) - (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn6), (((2.0 * locals.var_nsubps_dn7) - (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn7), (((2.0 * locals.var_nsubps_dn8) - (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn8), (((2.0 * locals.var_nsubps_dn9) - (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn9), (((2.0 * locals.var_nsubps_dn10) - (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn10), (((2.0 * locals.var_nsubps_dn13) - (((locals.var_nsubps_dn13 - locals.var_ef_nsubc_dn13) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn13),)
    } else {
        (locals.var_nsubb, locals.var_nsubb_dn0, locals.var_nsubb_dn2, locals.var_nsubb_dn4, locals.var_nsubb_dn5, locals.var_nsubb_dn6, locals.var_nsubb_dn7, locals.var_nsubb_dn8, locals.var_nsubb_dn9, locals.var_nsubb_dn10, locals.var_nsubb_dn13,)
    }
};
        locals.var_nsubb = assign11680_e6350;
        locals.var_nsubb_dn0 = assign11680_e6350_d_n0;
        locals.var_nsubb_dn2 = assign11680_e6350_d_n2;
        locals.var_nsubb_dn4 = assign11680_e6350_d_n4;
        locals.var_nsubb_dn5 = assign11680_e6350_d_n5;
        locals.var_nsubb_dn6 = assign11680_e6350_d_n6;
        locals.var_nsubb_dn7 = assign11680_e6350_d_n7;
        locals.var_nsubb_dn8 = assign11680_e6350_d_n8;
        locals.var_nsubb_dn9 = assign11680_e6350_d_n9;
        locals.var_nsubb_dn10 = assign11680_e6350_d_n10;
        locals.var_nsubb_dn13 = assign11680_e6350_d_n13;

        let (assign11690_e6357, assign11690_e6357_d_n0, assign11690_e6357_d_n2, assign11690_e6357_d_n4, assign11690_e6357_d_n5, assign11690_e6357_d_n6, assign11690_e6357_d_n7, assign11690_e6357_d_n8, assign11690_e6357_d_n9, assign11690_e6357_d_n10, assign11690_e6357_d_n13,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11690_e6354: f64 = (locals.var_nsubb / locals.var_ef_nsubc);
        let assign11690_e6355: f64 = (assign11690_e6354).ln();
        (assign11690_e6355, ((((locals.var_nsubb_dn0 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn2 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn4 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn5 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn6 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn7 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn8 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn9 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn10 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn13 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn13)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354),)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn13,)
    }
};
        locals.var_ptovr0 = assign11690_e6357;
        locals.var_ptovr0_dn0 = assign11690_e6357_d_n0;
        locals.var_ptovr0_dn2 = assign11690_e6357_d_n2;
        locals.var_ptovr0_dn4 = assign11690_e6357_d_n4;
        locals.var_ptovr0_dn5 = assign11690_e6357_d_n5;
        locals.var_ptovr0_dn6 = assign11690_e6357_d_n6;
        locals.var_ptovr0_dn7 = assign11690_e6357_d_n7;
        locals.var_ptovr0_dn8 = assign11690_e6357_d_n8;
        locals.var_ptovr0_dn9 = assign11690_e6357_d_n9;
        locals.var_ptovr0_dn10 = assign11690_e6357_d_n10;
        locals.var_ptovr0_dn13 = assign11690_e6357_d_n13;

        let (assign11700_e6362, assign11700_e6362_d_n0, assign11700_e6362_d_n2, assign11700_e6362_d_n4, assign11700_e6362_d_n5, assign11700_e6362_d_n6, assign11700_e6362_d_n7, assign11700_e6362_d_n8, assign11700_e6362_d_n9, assign11700_e6362_d_n10, assign11700_e6362_d_n13,) = {
    if (locals.var_guard268 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn13,)
    }
};
        locals.var_ptovr0 = assign11700_e6362;
        locals.var_ptovr0_dn0 = assign11700_e6362_d_n0;
        locals.var_ptovr0_dn2 = assign11700_e6362_d_n2;
        locals.var_ptovr0_dn4 = assign11700_e6362_d_n4;
        locals.var_ptovr0_dn5 = assign11700_e6362_d_n5;
        locals.var_ptovr0_dn6 = assign11700_e6362_d_n6;
        locals.var_ptovr0_dn7 = assign11700_e6362_d_n7;
        locals.var_ptovr0_dn8 = assign11700_e6362_d_n8;
        locals.var_ptovr0_dn9 = assign11700_e6362_d_n9;
        locals.var_ptovr0_dn10 = assign11700_e6362_d_n10;
        locals.var_ptovr0_dn13 = assign11700_e6362_d_n13;

        let assign11710_e6365: f64 = (2.0 * 1.6021918e-19);
        let assign11710_e6367: f64 = (assign11710_e6365 * locals.var_uc_nsti);
        let assign11710_e6369: f64 = (assign11710_e6367 * 1.034943e-10);
        let assign11710_e6370: f64 = (assign11710_e6369).sqrt();
        locals.var_costi00 = assign11710_e6370;

        let assign11720_e6374: f64 = (locals.var_uc_nsti * locals.var_uc_nsti);
        let assign11720_e6375: f64 = (1.0 / assign11720_e6374);
        locals.var_nsti_p2 = assign11720_e6375;

        let assign11730_e6380: f64 = (locals.var_lg).powf(p.p231);
        let assign11730_e6381: f64 = (locals.var_uc_vover / assign11730_e6380);
        let assign11730_e6382: f64 = (1.0 + assign11730_e6381);
        let assign11730_e6387: f64 = (locals.var_wlg).powf(p.p239);
        let assign11730_e6388: f64 = (p.p238 / assign11730_e6387);
        let assign11730_e6389: f64 = (1.0 + assign11730_e6388);
        let assign11730_e6390: f64 = (assign11730_e6382 * assign11730_e6389);
        locals.var_vmax0 = assign11730_e6390;

        let assign11740_e6393: f64 = (2.0 / 38.68283);
        let assign11740_e6396: f64 = (locals.var_nsub / 1.04e16);
        let assign11740_e6397: f64 = (assign11740_e6396).ln();
        let assign11740_e6398: f64 = (assign11740_e6393 * assign11740_e6397);
        locals.var_pb20 = assign11740_e6398;
        locals.var_pb20_dn0 = (assign11740_e6393 * ((locals.var_nsub_dn0 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn2 = (assign11740_e6393 * ((locals.var_nsub_dn2 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn4 = (assign11740_e6393 * ((locals.var_nsub_dn4 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn5 = (assign11740_e6393 * ((locals.var_nsub_dn5 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn6 = (assign11740_e6393 * ((locals.var_nsub_dn6 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn7 = (assign11740_e6393 * ((locals.var_nsub_dn7 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn8 = (assign11740_e6393 * ((locals.var_nsub_dn8 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn9 = (assign11740_e6393 * ((locals.var_nsub_dn9 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn10 = (assign11740_e6393 * ((locals.var_nsub_dn10 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn13 = (assign11740_e6393 * ((locals.var_nsub_dn13 / 1.04e16) / assign11740_e6396));

        let assign11750_e6401: f64 = (2.0 / 38.68283);
        let assign11750_e6404: f64 = (locals.var_ef_nsubc / 1.04e16);
        let assign11750_e6405: f64 = (assign11750_e6404).ln();
        let assign11750_e6406: f64 = (assign11750_e6401 * assign11750_e6405);
        locals.var_pb2c = assign11750_e6406;
        locals.var_pb2c_dn0 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn0 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn2 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn2 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn4 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn4 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn5 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn5 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn6 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn6 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn7 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn7 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn8 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn8 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn9 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn9 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn10 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn10 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn13 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn13 / 1.04e16) / assign11750_e6404));

        let assign11760_e6409: f64 = if p.p51 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign11760_e6409;

        let (assign11770_e6419, assign11770_e6419_d_n0, assign11770_e6419_d_n2, assign11770_e6419_d_n4, assign11770_e6419_d_n5, assign11770_e6419_d_n6, assign11770_e6419_d_n7, assign11770_e6419_d_n8, assign11770_e6419_d_n9, assign11770_e6419_d_n10, assign11770_e6419_d_n13,) = {
    if (locals.var_guard269 != 0.0) {
        let assign11770_e6415: f64 = (3.0 * p.p4);
        let assign11770_e6416: f64 = (locals.var_weff / assign11770_e6415);
        let assign11770_e6417: f64 = (p.p5 + assign11770_e6416);
        (assign11770_e6417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign11770_e6419;
        locals.var_t1_dn0 = assign11770_e6419_d_n0;
        locals.var_t1_dn2 = assign11770_e6419_d_n2;
        locals.var_t1_dn4 = assign11770_e6419_d_n4;
        locals.var_t1_dn5 = assign11770_e6419_d_n5;
        locals.var_t1_dn6 = assign11770_e6419_d_n6;
        locals.var_t1_dn7 = assign11770_e6419_d_n7;
        locals.var_t1_dn8 = assign11770_e6419_d_n8;
        locals.var_t1_dn9 = assign11770_e6419_d_n9;
        locals.var_t1_dn10 = assign11770_e6419_d_n10;
        locals.var_t1_dn13 = assign11770_e6419_d_n13;

        let (assign11780_e6425, assign11780_e6425_d_n0, assign11780_e6425_d_n2, assign11780_e6425_d_n4, assign11780_e6425_d_n5, assign11780_e6425_d_n6, assign11780_e6425_d_n7, assign11780_e6425_d_n8, assign11780_e6425_d_n9, assign11780_e6425_d_n10, assign11780_e6425_d_n13,) = {
    if (locals.var_guard269 != 0.0) {
        let assign11780_e6423: f64 = (locals.var_lgate - p.p6);
        (assign11780_e6423, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign11780_e6425;
        locals.var_t2_dn0 = assign11780_e6425_d_n0;
        locals.var_t2_dn2 = assign11780_e6425_d_n2;
        locals.var_t2_dn4 = assign11780_e6425_d_n4;
        locals.var_t2_dn5 = assign11780_e6425_d_n5;
        locals.var_t2_dn6 = assign11780_e6425_d_n6;
        locals.var_t2_dn7 = assign11780_e6425_d_n7;
        locals.var_t2_dn8 = assign11780_e6425_d_n8;
        locals.var_t2_dn9 = assign11780_e6425_d_n9;
        locals.var_t2_dn10 = assign11780_e6425_d_n10;
        locals.var_t2_dn13 = assign11780_e6425_d_n13;

        let assign11840_e6467: f64 = if p.p130 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign11840_e6467;

        let (assign11850_e6473,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11850_e6471: f64 = (p.p130 * p.p2);
        (assign11850_e6471,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11850_e6473;

        let (assign11860_e6479,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11860_e6477: f64 = (p.p130 * p.p3);
        (assign11860_e6477,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11860_e6479;

        let (assign11870_e6484,) = {
    if (locals.var_guard271 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11870_e6484;

        let (assign11880_e6489,) = {
    if (locals.var_guard271 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11880_e6489;

        let assign11890_e6492: f64 = if p.p131 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign11890_e6492;

        let (assign11900_e6498,) = {
    if (locals.var_guard272 != 0.0) {
        let assign11900_e6496: f64 = (p.p131 * p.p3);
        (assign11900_e6496,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11900_e6498;

        let (assign11910_e6503,) = {
    if (locals.var_guard272 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11910_e6503;

        let assign11920_e6506: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign11920_e6506;

        let assign11930_e6513: f64 = if ((locals.var_uc_rd > 0.0) || (locals.var_uc_rs > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard274 = assign11930_e6513;

        let (assign11940_e6525,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) {
        let assign11940_e6521: f64 = (locals.var_wlg).powf(p.p310);
        let assign11940_e6522: f64 = (p.p309 / assign11940_e6521);
        let assign11940_e6523: f64 = (1.0 + assign11940_e6522);
        (assign11940_e6523,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign11940_e6525;

        let assign11950_e6528: f64 = if locals.var_uc_rdvd != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign11950_e6528;

    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11960_e6542, assign11960_e6542_d_n0, assign11960_e6542_d_n2, assign11960_e6542_d_n4, assign11960_e6542_d_n5, assign11960_e6542_d_n6, assign11960_e6542_d_n7, assign11960_e6542_d_n8, assign11960_e6542_d_n9, assign11960_e6542_d_n10, assign11960_e6542_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign11960_e6538: f64 = (locals.var_wlg).powf(p.p304);
        let assign11960_e6539: f64 = (p.p303 / assign11960_e6538);
        let assign11960_e6540: f64 = (1.0 + assign11960_e6539);
        (assign11960_e6540, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign11960_e6542;
        locals.var_t7_dn0 = assign11960_e6542_d_n0;
        locals.var_t7_dn2 = assign11960_e6542_d_n2;
        locals.var_t7_dn4 = assign11960_e6542_d_n4;
        locals.var_t7_dn5 = assign11960_e6542_d_n5;
        locals.var_t7_dn6 = assign11960_e6542_d_n6;
        locals.var_t7_dn7 = assign11960_e6542_d_n7;
        locals.var_t7_dn8 = assign11960_e6542_d_n8;
        locals.var_t7_dn9 = assign11960_e6542_d_n9;
        locals.var_t7_dn10 = assign11960_e6542_d_n10;
        locals.var_t7_dn13 = assign11960_e6542_d_n13;

        let (assign11970_e6555, assign11970_e6555_d_n0, assign11970_e6555_d_n2, assign11970_e6555_d_n4, assign11970_e6555_d_n5, assign11970_e6555_d_n6, assign11970_e6555_d_n7, assign11970_e6555_d_n8, assign11970_e6555_d_n9, assign11970_e6555_d_n10, assign11970_e6555_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign11970_e6549: f64 = (-p.p301);
        let assign11970_e6552: f64 = (locals.var_lg).powf(p.p302);
        let assign11970_e6553: f64 = (assign11970_e6549 * assign11970_e6552);
        (assign11970_e6553, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign11970_e6555;
        locals.var_t6_dn0 = assign11970_e6555_d_n0;
        locals.var_t6_dn2 = assign11970_e6555_d_n2;
        locals.var_t6_dn4 = assign11970_e6555_d_n4;
        locals.var_t6_dn5 = assign11970_e6555_d_n5;
        locals.var_t6_dn6 = assign11970_e6555_d_n6;
        locals.var_t6_dn7 = assign11970_e6555_d_n7;
        locals.var_t6_dn8 = assign11970_e6555_d_n8;
        locals.var_t6_dn9 = assign11970_e6555_d_n9;
        locals.var_t6_dn10 = assign11970_e6555_d_n10;
        locals.var_t6_dn13 = assign11970_e6555_d_n13;

        let assign11980_e6558: f64 = if locals.var_t6 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign11980_e6558;

        let (assign11990_e6568, assign11990_e6568_d_n0, assign11990_e6568_d_n2, assign11990_e6568_d_n4, assign11990_e6568_d_n5, assign11990_e6568_d_n6, assign11990_e6568_d_n7, assign11990_e6568_d_n8, assign11990_e6568_d_n9, assign11990_e6568_d_n10, assign11990_e6568_d_n13,) = {
    if ((((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) && (locals.var_guard276 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign11990_e6568;
        locals.var_t6_dn0 = assign11990_e6568_d_n0;
        locals.var_t6_dn2 = assign11990_e6568_d_n2;
        locals.var_t6_dn4 = assign11990_e6568_d_n4;
        locals.var_t6_dn5 = assign11990_e6568_d_n5;
        locals.var_t6_dn6 = assign11990_e6568_d_n6;
        locals.var_t6_dn7 = assign11990_e6568_d_n7;
        locals.var_t6_dn8 = assign11990_e6568_d_n8;
        locals.var_t6_dn9 = assign11990_e6568_d_n9;
        locals.var_t6_dn10 = assign11990_e6568_d_n10;
        locals.var_t6_dn13 = assign11990_e6568_d_n13;

        let (assign12000_e6577, assign12000_e6577_d_n0, assign12000_e6577_d_n2, assign12000_e6577_d_n4, assign12000_e6577_d_n5, assign12000_e6577_d_n6, assign12000_e6577_d_n7, assign12000_e6577_d_n8, assign12000_e6577_d_n9, assign12000_e6577_d_n10, assign12000_e6577_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign12000_e6575: f64 = (locals.var_t6).exp();
        (assign12000_e6575, (assign12000_e6575 * locals.var_t6_dn0), (assign12000_e6575 * locals.var_t6_dn2), (assign12000_e6575 * locals.var_t6_dn4), (assign12000_e6575 * locals.var_t6_dn5), (assign12000_e6575 * locals.var_t6_dn6), (assign12000_e6575 * locals.var_t6_dn7), (assign12000_e6575 * locals.var_t6_dn8), (assign12000_e6575 * locals.var_t6_dn9), (assign12000_e6575 * locals.var_t6_dn10), (assign12000_e6575 * locals.var_t6_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign12000_e6577;
        locals.var_t6_dn0 = assign12000_e6577_d_n0;
        locals.var_t6_dn2 = assign12000_e6577_d_n2;
        locals.var_t6_dn4 = assign12000_e6577_d_n4;
        locals.var_t6_dn5 = assign12000_e6577_d_n5;
        locals.var_t6_dn6 = assign12000_e6577_d_n6;
        locals.var_t6_dn7 = assign12000_e6577_d_n7;
        locals.var_t6_dn8 = assign12000_e6577_d_n8;
        locals.var_t6_dn9 = assign12000_e6577_d_n9;
        locals.var_t6_dn10 = assign12000_e6577_d_n10;
        locals.var_t6_dn13 = assign12000_e6577_d_n13;

        let (assign12010_e6587, assign12010_e6587_d_n0, assign12010_e6587_d_n2, assign12010_e6587_d_n4, assign12010_e6587_d_n5, assign12010_e6587_d_n6, assign12010_e6587_d_n7, assign12010_e6587_d_n8, assign12010_e6587_d_n9, assign12010_e6587_d_n10, assign12010_e6587_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign12010_e6585: f64 = (locals.var_t6 * locals.var_t7);
        (assign12010_e6585, ((locals.var_t6_dn0 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn0)), ((locals.var_t6_dn2 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn2)), ((locals.var_t6_dn4 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn4)), ((locals.var_t6_dn5 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn5)), ((locals.var_t6_dn6 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn6)), ((locals.var_t6_dn7 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn7)), ((locals.var_t6_dn8 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn8)), ((locals.var_t6_dn9 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn9)), ((locals.var_t6_dn10 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn10)), ((locals.var_t6_dn13 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn13)),)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn13,)
    }
};
        locals.var_rdvdtemp0 = assign12010_e6587;
        locals.var_rdvdtemp0_dn0 = assign12010_e6587_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12010_e6587_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12010_e6587_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12010_e6587_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12010_e6587_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12010_e6587_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12010_e6587_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12010_e6587_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12010_e6587_d_n10;
        locals.var_rdvdtemp0_dn13 = assign12010_e6587_d_n13;

        let (assign12020_e6596, assign12020_e6596_d_n0, assign12020_e6596_d_n2, assign12020_e6596_d_n4, assign12020_e6596_d_n5, assign12020_e6596_d_n6, assign12020_e6596_d_n7, assign12020_e6596_d_n8, assign12020_e6596_d_n9, assign12020_e6596_d_n10, assign12020_e6596_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn13,)
    }
};
        locals.var_rdvdtemp0 = assign12020_e6596;
        locals.var_rdvdtemp0_dn0 = assign12020_e6596_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12020_e6596_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12020_e6596_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12020_e6596_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12020_e6596_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12020_e6596_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12020_e6596_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12020_e6596_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12020_e6596_d_n10;
        locals.var_rdvdtemp0_dn13 = assign12020_e6596_d_n13;

        let (assign12030_e6603,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard274 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12030_e6603;

        let (assign12040_e6610, assign12040_e6610_d_n0, assign12040_e6610_d_n2, assign12040_e6610_d_n4, assign12040_e6610_d_n5, assign12040_e6610_d_n6, assign12040_e6610_d_n7, assign12040_e6610_d_n8, assign12040_e6610_d_n9, assign12040_e6610_d_n10, assign12040_e6610_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard274 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn13,)
    }
};
        locals.var_rdvdtemp0 = assign12040_e6610;
        locals.var_rdvdtemp0_dn0 = assign12040_e6610_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12040_e6610_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12040_e6610_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12040_e6610_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12040_e6610_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12040_e6610_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12040_e6610_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12040_e6610_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12040_e6610_d_n10;
        locals.var_rdvdtemp0_dn13 = assign12040_e6610_d_n13;

        let assign12050_e6613: f64 = if locals.var_uc_rd23 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign12050_e6613;

        let (assign12060_e6625, assign12060_e6625_d_n0, assign12060_e6625_d_n2, assign12060_e6625_d_n4, assign12060_e6625_d_n5, assign12060_e6625_d_n6, assign12060_e6625_d_n7, assign12060_e6625_d_n8, assign12060_e6625_d_n9, assign12060_e6625_d_n10, assign12060_e6625_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12060_e6621: f64 = (locals.var_wlg).powf(p.p308);
        let assign12060_e6622: f64 = (p.p307 / assign12060_e6621);
        let assign12060_e6623: f64 = (1.0 + assign12060_e6622);
        (assign12060_e6623, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign12060_e6625;
        locals.var_t2_dn0 = assign12060_e6625_d_n0;
        locals.var_t2_dn2 = assign12060_e6625_d_n2;
        locals.var_t2_dn4 = assign12060_e6625_d_n4;
        locals.var_t2_dn5 = assign12060_e6625_d_n5;
        locals.var_t2_dn6 = assign12060_e6625_d_n6;
        locals.var_t2_dn7 = assign12060_e6625_d_n7;
        locals.var_t2_dn8 = assign12060_e6625_d_n8;
        locals.var_t2_dn9 = assign12060_e6625_d_n9;
        locals.var_t2_dn10 = assign12060_e6625_d_n10;
        locals.var_t2_dn13 = assign12060_e6625_d_n13;

        let (assign12070_e6636, assign12070_e6636_d_n0, assign12070_e6636_d_n2, assign12070_e6636_d_n4, assign12070_e6636_d_n5, assign12070_e6636_d_n6, assign12070_e6636_d_n7, assign12070_e6636_d_n8, assign12070_e6636_d_n9, assign12070_e6636_d_n10, assign12070_e6636_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12070_e6630: f64 = (-p.p305);
        let assign12070_e6633: f64 = (locals.var_lg).powf(p.p306);
        let assign12070_e6634: f64 = (assign12070_e6630 * assign12070_e6633);
        (assign12070_e6634, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12070_e6636;
        locals.var_t1_dn0 = assign12070_e6636_d_n0;
        locals.var_t1_dn2 = assign12070_e6636_d_n2;
        locals.var_t1_dn4 = assign12070_e6636_d_n4;
        locals.var_t1_dn5 = assign12070_e6636_d_n5;
        locals.var_t1_dn6 = assign12070_e6636_d_n6;
        locals.var_t1_dn7 = assign12070_e6636_d_n7;
        locals.var_t1_dn8 = assign12070_e6636_d_n8;
        locals.var_t1_dn9 = assign12070_e6636_d_n9;
        locals.var_t1_dn10 = assign12070_e6636_d_n10;
        locals.var_t1_dn13 = assign12070_e6636_d_n13;

        let assign12080_e6639: f64 = if locals.var_t1 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign12080_e6639;

        let (assign12090_e6647, assign12090_e6647_d_n0, assign12090_e6647_d_n2, assign12090_e6647_d_n4, assign12090_e6647_d_n5, assign12090_e6647_d_n6, assign12090_e6647_d_n7, assign12090_e6647_d_n8, assign12090_e6647_d_n9, assign12090_e6647_d_n10, assign12090_e6647_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) && (locals.var_guard278 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12090_e6647;
        locals.var_t1_dn0 = assign12090_e6647_d_n0;
        locals.var_t1_dn2 = assign12090_e6647_d_n2;
        locals.var_t1_dn4 = assign12090_e6647_d_n4;
        locals.var_t1_dn5 = assign12090_e6647_d_n5;
        locals.var_t1_dn6 = assign12090_e6647_d_n6;
        locals.var_t1_dn7 = assign12090_e6647_d_n7;
        locals.var_t1_dn8 = assign12090_e6647_d_n8;
        locals.var_t1_dn9 = assign12090_e6647_d_n9;
        locals.var_t1_dn10 = assign12090_e6647_d_n10;
        locals.var_t1_dn13 = assign12090_e6647_d_n13;

        let (assign12100_e6654, assign12100_e6654_d_n0, assign12100_e6654_d_n2, assign12100_e6654_d_n4, assign12100_e6654_d_n5, assign12100_e6654_d_n6, assign12100_e6654_d_n7, assign12100_e6654_d_n8, assign12100_e6654_d_n9, assign12100_e6654_d_n10, assign12100_e6654_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12100_e6652: f64 = (locals.var_t1).exp();
        (assign12100_e6652, (assign12100_e6652 * locals.var_t1_dn0), (assign12100_e6652 * locals.var_t1_dn2), (assign12100_e6652 * locals.var_t1_dn4), (assign12100_e6652 * locals.var_t1_dn5), (assign12100_e6652 * locals.var_t1_dn6), (assign12100_e6652 * locals.var_t1_dn7), (assign12100_e6652 * locals.var_t1_dn8), (assign12100_e6652 * locals.var_t1_dn9), (assign12100_e6652 * locals.var_t1_dn10), (assign12100_e6652 * locals.var_t1_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12100_e6654;
        locals.var_t1_dn0 = assign12100_e6654_d_n0;
        locals.var_t1_dn2 = assign12100_e6654_d_n2;
        locals.var_t1_dn4 = assign12100_e6654_d_n4;
        locals.var_t1_dn5 = assign12100_e6654_d_n5;
        locals.var_t1_dn6 = assign12100_e6654_d_n6;
        locals.var_t1_dn7 = assign12100_e6654_d_n7;
        locals.var_t1_dn8 = assign12100_e6654_d_n8;
        locals.var_t1_dn9 = assign12100_e6654_d_n9;
        locals.var_t1_dn10 = assign12100_e6654_d_n10;
        locals.var_t1_dn13 = assign12100_e6654_d_n13;

        let (assign12110_e6664, assign12110_e6664_d_n0, assign12110_e6664_d_n2, assign12110_e6664_d_n4, assign12110_e6664_d_n5, assign12110_e6664_d_n6, assign12110_e6664_d_n7, assign12110_e6664_d_n8, assign12110_e6664_d_n9, assign12110_e6664_d_n10, assign12110_e6664_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12110_e6660: f64 = (locals.var_uc_rd23 * locals.var_t2);
        let assign12110_e6662: f64 = (assign12110_e6660 * locals.var_t1);
        (assign12110_e6662, (((locals.var_uc_rd23 * locals.var_t2_dn0) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn0)), (((locals.var_uc_rd23 * locals.var_t2_dn2) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn2)), (((locals.var_uc_rd23 * locals.var_t2_dn4) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn4)), (((locals.var_uc_rd23 * locals.var_t2_dn5) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn5)), (((locals.var_uc_rd23 * locals.var_t2_dn6) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn6)), (((locals.var_uc_rd23 * locals.var_t2_dn7) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn7)), (((locals.var_uc_rd23 * locals.var_t2_dn8) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn8)), (((locals.var_uc_rd23 * locals.var_t2_dn9) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn9)), (((locals.var_uc_rd23 * locals.var_t2_dn10) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn10)), (((locals.var_uc_rd23 * locals.var_t2_dn13) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign12110_e6664;
        locals.var_t3_dn0 = assign12110_e6664_d_n0;
        locals.var_t3_dn2 = assign12110_e6664_d_n2;
        locals.var_t3_dn4 = assign12110_e6664_d_n4;
        locals.var_t3_dn5 = assign12110_e6664_d_n5;
        locals.var_t3_dn6 = assign12110_e6664_d_n6;
        locals.var_t3_dn7 = assign12110_e6664_d_n7;
        locals.var_t3_dn8 = assign12110_e6664_d_n8;
        locals.var_t3_dn9 = assign12110_e6664_d_n9;
        locals.var_t3_dn10 = assign12110_e6664_d_n10;
        locals.var_t3_dn13 = assign12110_e6664_d_n13;

        let (assign12120_e6687, assign12120_e6687_d_n0, assign12120_e6687_d_n2, assign12120_e6687_d_n4, assign12120_e6687_d_n5, assign12120_e6687_d_n6, assign12120_e6687_d_n7, assign12120_e6687_d_n8, assign12120_e6687_d_n9, assign12120_e6687_d_n10, assign12120_e6687_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12120_e6672: f64 = (locals.var_t3 * locals.var_t3);
        let assign12120_e6675: f64 = (4.0 * 1e-6);
        let assign12120_e6677: f64 = (assign12120_e6675 / 100.0);
        let assign12120_e6679: f64 = (assign12120_e6677 * 1e-6);
        let assign12120_e6681: f64 = (assign12120_e6679 / 100.0);
        let assign12120_e6682: f64 = (assign12120_e6672 + assign12120_e6681);
        let assign12120_e6683: f64 = (assign12120_e6682).sqrt();
        let assign12120_e6684: f64 = (locals.var_t3 + assign12120_e6683);
        let assign12120_e6685: f64 = (0.5 * assign12120_e6684);
        (assign12120_e6685, (0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13)) / (2.0 * assign12120_e6683)))),)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    }
};
        locals.var_rd23e = assign12120_e6687;
        locals.var_rd23e_dn0 = assign12120_e6687_d_n0;
        locals.var_rd23e_dn2 = assign12120_e6687_d_n2;
        locals.var_rd23e_dn4 = assign12120_e6687_d_n4;
        locals.var_rd23e_dn5 = assign12120_e6687_d_n5;
        locals.var_rd23e_dn6 = assign12120_e6687_d_n6;
        locals.var_rd23e_dn7 = assign12120_e6687_d_n7;
        locals.var_rd23e_dn8 = assign12120_e6687_d_n8;
        locals.var_rd23e_dn9 = assign12120_e6687_d_n9;
        locals.var_rd23e_dn10 = assign12120_e6687_d_n10;
        locals.var_rd23e_dn13 = assign12120_e6687_d_n13;

        let (assign12130_e6694, assign12130_e6694_d_n0, assign12130_e6694_d_n2, assign12130_e6694_d_n4, assign12130_e6694_d_n5, assign12130_e6694_d_n6, assign12130_e6694_d_n7, assign12130_e6694_d_n8, assign12130_e6694_d_n9, assign12130_e6694_d_n10, assign12130_e6694_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    }
};
        locals.var_rd23e = assign12130_e6694;
        locals.var_rd23e_dn0 = assign12130_e6694_d_n0;
        locals.var_rd23e_dn2 = assign12130_e6694_d_n2;
        locals.var_rd23e_dn4 = assign12130_e6694_d_n4;
        locals.var_rd23e_dn5 = assign12130_e6694_d_n5;
        locals.var_rd23e_dn6 = assign12130_e6694_d_n6;
        locals.var_rd23e_dn7 = assign12130_e6694_d_n7;
        locals.var_rd23e_dn8 = assign12130_e6694_d_n8;
        locals.var_rd23e_dn9 = assign12130_e6694_d_n9;
        locals.var_rd23e_dn10 = assign12130_e6694_d_n10;
        locals.var_rd23e_dn13 = assign12130_e6694_d_n13;

        let (assign12140_e6698,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12140_e6698;

        let (assign12150_e6702,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12150_e6702;

        let (assign12160_e6706,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12160_e6706;

        let (assign12170_e6710,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12170_e6710;

        let (assign12180_e6714,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12180_e6714;

        let (assign12190_e6718, assign12190_e6718_d_n0, assign12190_e6718_d_n2, assign12190_e6718_d_n4, assign12190_e6718_d_n5, assign12190_e6718_d_n6, assign12190_e6718_d_n7, assign12190_e6718_d_n8, assign12190_e6718_d_n9, assign12190_e6718_d_n10, assign12190_e6718_d_n13,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn13,)
    }
};
        locals.var_rdrmuevbs = assign12190_e6718;
        locals.var_rdrmuevbs_dn0 = assign12190_e6718_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12190_e6718_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12190_e6718_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12190_e6718_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12190_e6718_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12190_e6718_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12190_e6718_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12190_e6718_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12190_e6718_d_n10;
        locals.var_rdrmuevbs_dn13 = assign12190_e6718_d_n13;

        let (assign12200_e6730,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12200_e6723: f64 = (p.p419 * p.p419);
        let assign12200_e6726: f64 = (locals.var_uc_xldld * locals.var_uc_xldld);
        let assign12200_e6727: f64 = (assign12200_e6723 + assign12200_e6726);
        let assign12200_e6728: f64 = (assign12200_e6727).sqrt();
        (assign12200_e6728,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12200_e6730;

        let (assign12210_e6742,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12210_e6735: f64 = (p.p419 * p.p419);
        let assign12210_e6738: f64 = (p.p97 * p.p97);
        let assign12210_e6739: f64 = (assign12210_e6735 + assign12210_e6738);
        let assign12210_e6740: f64 = (assign12210_e6739).sqrt();
        (assign12210_e6740,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12210_e6742;

        let (assign12220_e6753,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12220_e6749: f64 = (locals.var_wg).powf(p.p425);
        let assign12220_e6750: f64 = (p.p424 / assign12220_e6749);
        let assign12220_e6751: f64 = (1.0 + assign12220_e6750);
        (assign12220_e6751,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12220_e6753;

        let (assign12230_e6764,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12230_e6760: f64 = (locals.var_lg).powf(p.p427);
        let assign12230_e6761: f64 = (p.p426 / assign12230_e6760);
        let assign12230_e6762: f64 = (1.0 + assign12230_e6761);
        (assign12230_e6762,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12230_e6764;

        let (assign12240_e6775,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12240_e6771: f64 = (locals.var_lg).powf(p.p429);
        let assign12240_e6772: f64 = (p.p428 / assign12240_e6771);
        let assign12240_e6773: f64 = (1.0 + assign12240_e6772);
        (assign12240_e6773,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12240_e6775;

        let (assign12250_e6780, assign12250_e6780_d_n0, assign12250_e6780_d_n2, assign12250_e6780_d_n4, assign12250_e6780_d_n5, assign12250_e6780_d_n6, assign12250_e6780_d_n7, assign12250_e6780_d_n8, assign12250_e6780_d_n9, assign12250_e6780_d_n10, assign12250_e6780_d_n13,) = {
    if (locals.var_guard273 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn13,)
    }
};
        locals.var_rdrmuevbs = assign12250_e6780;
        locals.var_rdrmuevbs_dn0 = assign12250_e6780_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12250_e6780_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12250_e6780_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12250_e6780_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12250_e6780_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12250_e6780_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12250_e6780_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12250_e6780_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12250_e6780_d_n10;
        locals.var_rdrmuevbs_dn13 = assign12250_e6780_d_n13;

        let (assign12260_e6785,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12260_e6785;

        let (assign12270_e6790, assign12270_e6790_d_n0, assign12270_e6790_d_n2, assign12270_e6790_d_n4, assign12270_e6790_d_n5, assign12270_e6790_d_n6, assign12270_e6790_d_n7, assign12270_e6790_d_n8, assign12270_e6790_d_n9, assign12270_e6790_d_n10, assign12270_e6790_d_n13,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn13,)
    }
};
        locals.var_rdvdtemp0 = assign12270_e6790;
        locals.var_rdvdtemp0_dn0 = assign12270_e6790_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12270_e6790_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12270_e6790_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12270_e6790_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12270_e6790_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12270_e6790_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12270_e6790_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12270_e6790_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12270_e6790_d_n10;
        locals.var_rdvdtemp0_dn13 = assign12270_e6790_d_n13;

        let (assign12280_e6795, assign12280_e6795_d_n0, assign12280_e6795_d_n2, assign12280_e6795_d_n4, assign12280_e6795_d_n5, assign12280_e6795_d_n6, assign12280_e6795_d_n7, assign12280_e6795_d_n8, assign12280_e6795_d_n9, assign12280_e6795_d_n10, assign12280_e6795_d_n13,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    }
};
        locals.var_rd23e = assign12280_e6795;
        locals.var_rd23e_dn0 = assign12280_e6795_d_n0;
        locals.var_rd23e_dn2 = assign12280_e6795_d_n2;
        locals.var_rd23e_dn4 = assign12280_e6795_d_n4;
        locals.var_rd23e_dn5 = assign12280_e6795_d_n5;
        locals.var_rd23e_dn6 = assign12280_e6795_d_n6;
        locals.var_rd23e_dn7 = assign12280_e6795_d_n7;
        locals.var_rd23e_dn8 = assign12280_e6795_d_n8;
        locals.var_rd23e_dn9 = assign12280_e6795_d_n9;
        locals.var_rd23e_dn10 = assign12280_e6795_d_n10;
        locals.var_rd23e_dn13 = assign12280_e6795_d_n13;

        let assign12290_e6798: f64 = if locals.var_uc_nover > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign12290_e6798;

    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12300_e6808,) = {
    if (locals.var_guard279 != 0.0) {
        let assign12300_e6802: f64 = (2.0 * 1.034943e-10);
        let assign12300_e6805: f64 = (1.6021918e-19 * locals.var_uc_nover);
        let assign12300_e6806: f64 = (assign12300_e6802 / assign12300_e6805);
        (assign12300_e6806,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12300_e6808;

        let (assign12310_e6824, assign12310_e6824_d_n0, assign12310_e6824_d_n2, assign12310_e6824_d_n4, assign12310_e6824_d_n5, assign12310_e6824_d_n6, assign12310_e6824_d_n7, assign12310_e6824_d_n8, assign12310_e6824_d_n9, assign12310_e6824_d_n10, assign12310_e6824_d_n13,) = {
    if (locals.var_guard279 != 0.0) {
        let assign12310_e6812: f64 = (2.0 * 1.034943e-10);
        let assign12310_e6814: f64 = (assign12310_e6812 / 1.6021918e-19);
        let assign12310_e6816: f64 = (assign12310_e6814 * locals.var_ef_nsubc);
        let assign12310_e6819: f64 = (locals.var_uc_nover + locals.var_ef_nsubc);
        let assign12310_e6820: f64 = (assign12310_e6816 / assign12310_e6819);
        let assign12310_e6822: f64 = (assign12310_e6820 / locals.var_uc_nover);
        (assign12310_e6822, (((((assign12310_e6814 * locals.var_ef_nsubc_dn0) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn0)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn2) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn2)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn4) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn4)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn5) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn5)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn6) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn6)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn7) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn7)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn8) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn8)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn9) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn9)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn10) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn10)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn13) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn13)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover),)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn13,)
    }
};
        locals.var_kjunc = assign12310_e6824;
        locals.var_kjunc_dn0 = assign12310_e6824_d_n0;
        locals.var_kjunc_dn2 = assign12310_e6824_d_n2;
        locals.var_kjunc_dn4 = assign12310_e6824_d_n4;
        locals.var_kjunc_dn5 = assign12310_e6824_d_n5;
        locals.var_kjunc_dn6 = assign12310_e6824_d_n6;
        locals.var_kjunc_dn7 = assign12310_e6824_d_n7;
        locals.var_kjunc_dn8 = assign12310_e6824_d_n8;
        locals.var_kjunc_dn9 = assign12310_e6824_d_n9;
        locals.var_kjunc_dn10 = assign12310_e6824_d_n10;
        locals.var_kjunc_dn13 = assign12310_e6824_d_n13;

        let (assign12320_e6829,) = {
    if (locals.var_guard279 == 0.0) {
        (0.0,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12320_e6829;

        let (assign12330_e6834, assign12330_e6834_d_n0, assign12330_e6834_d_n2, assign12330_e6834_d_n4, assign12330_e6834_d_n5, assign12330_e6834_d_n6, assign12330_e6834_d_n7, assign12330_e6834_d_n8, assign12330_e6834_d_n9, assign12330_e6834_d_n10, assign12330_e6834_d_n13,) = {
    if (locals.var_guard279 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn13,)
    }
};
        locals.var_kjunc = assign12330_e6834;
        locals.var_kjunc_dn0 = assign12330_e6834_d_n0;
        locals.var_kjunc_dn2 = assign12330_e6834_d_n2;
        locals.var_kjunc_dn4 = assign12330_e6834_d_n4;
        locals.var_kjunc_dn5 = assign12330_e6834_d_n5;
        locals.var_kjunc_dn6 = assign12330_e6834_d_n6;
        locals.var_kjunc_dn7 = assign12330_e6834_d_n7;
        locals.var_kjunc_dn8 = assign12330_e6834_d_n8;
        locals.var_kjunc_dn9 = assign12330_e6834_d_n9;
        locals.var_kjunc_dn10 = assign12330_e6834_d_n10;
        locals.var_kjunc_dn13 = assign12330_e6834_d_n13;

        let assign12470_e6929: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign12470_e6929;

        let (assign12480_e6937, assign12480_e6937_d_n0, assign12480_e6937_d_n2, assign12480_e6937_d_n4, assign12480_e6937_d_n5, assign12480_e6937_d_n6, assign12480_e6937_d_n7, assign12480_e6937_d_n8, assign12480_e6937_d_n9, assign12480_e6937_d_n10, assign12480_e6937_d_n13,) = {
    if (locals.var_guard284 != 0.0) {
        let assign12480_e6933: f64 = (p.p108 * locals.var_lg);
        let assign12480_e6935: f64 = (assign12480_e6933 + p.p109);
        (assign12480_e6935, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12480_e6937;
        locals.var_t1_dn0 = assign12480_e6937_d_n0;
        locals.var_t1_dn2 = assign12480_e6937_d_n2;
        locals.var_t1_dn4 = assign12480_e6937_d_n4;
        locals.var_t1_dn5 = assign12480_e6937_d_n5;
        locals.var_t1_dn6 = assign12480_e6937_d_n6;
        locals.var_t1_dn7 = assign12480_e6937_d_n7;
        locals.var_t1_dn8 = assign12480_e6937_d_n8;
        locals.var_t1_dn9 = assign12480_e6937_d_n9;
        locals.var_t1_dn10 = assign12480_e6937_d_n10;
        locals.var_t1_dn13 = assign12480_e6937_d_n13;

        let assign12490_e6940: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign12490_e6940;

        let (assign12500_e6946, assign12500_e6946_d_n0, assign12500_e6946_d_n2, assign12500_e6946_d_n4, assign12500_e6946_d_n5, assign12500_e6946_d_n6, assign12500_e6946_d_n7, assign12500_e6946_d_n8, assign12500_e6946_d_n9, assign12500_e6946_d_n10, assign12500_e6946_d_n13,) = {
    if ((locals.var_guard284 != 0.0) && (locals.var_guard285 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12500_e6946;
        locals.var_t1_dn0 = assign12500_e6946_d_n0;
        locals.var_t1_dn2 = assign12500_e6946_d_n2;
        locals.var_t1_dn4 = assign12500_e6946_d_n4;
        locals.var_t1_dn5 = assign12500_e6946_d_n5;
        locals.var_t1_dn6 = assign12500_e6946_d_n6;
        locals.var_t1_dn7 = assign12500_e6946_d_n7;
        locals.var_t1_dn8 = assign12500_e6946_d_n8;
        locals.var_t1_dn9 = assign12500_e6946_d_n9;
        locals.var_t1_dn10 = assign12500_e6946_d_n10;
        locals.var_t1_dn13 = assign12500_e6946_d_n13;

        let (assign12510_e6958, assign12510_e6958_d_n0, assign12510_e6958_d_n2, assign12510_e6958_d_n4, assign12510_e6958_d_n5, assign12510_e6958_d_n6, assign12510_e6958_d_n7, assign12510_e6958_d_n8, assign12510_e6958_d_n9, assign12510_e6958_d_n10, assign12510_e6958_d_n13,) = {
    if (locals.var_guard284 != 0.0) {
        let assign12510_e6950: f64 = (locals.var_t1 * p.p107);
        let assign12510_e6953: f64 = (locals.var_t1 + p.p107);
        let assign12510_e6954: f64 = (assign12510_e6950 / assign12510_e6953);
        let assign12510_e6956: f64 = (assign12510_e6954 + 1.0);
        (assign12510_e6956, ((((locals.var_t1_dn0 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn0)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn2 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn2)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn4 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn4)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn5 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn5)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn6 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn6)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn7 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn7)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn8 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn8)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn9 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn9)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn10 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn10)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn13 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn13)) / (assign12510_e6953 * assign12510_e6953)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn13,)
    }
};
        locals.var_ddlte = assign12510_e6958;
        locals.var_ddlte_dn0 = assign12510_e6958_d_n0;
        locals.var_ddlte_dn2 = assign12510_e6958_d_n2;
        locals.var_ddlte_dn4 = assign12510_e6958_d_n4;
        locals.var_ddlte_dn5 = assign12510_e6958_d_n5;
        locals.var_ddlte_dn6 = assign12510_e6958_d_n6;
        locals.var_ddlte_dn7 = assign12510_e6958_d_n7;
        locals.var_ddlte_dn8 = assign12510_e6958_d_n8;
        locals.var_ddlte_dn9 = assign12510_e6958_d_n9;
        locals.var_ddlte_dn10 = assign12510_e6958_d_n10;
        locals.var_ddlte_dn13 = assign12510_e6958_d_n13;

        let (assign12520_e6965, assign12520_e6965_d_n0, assign12520_e6965_d_n2, assign12520_e6965_d_n4, assign12520_e6965_d_n5, assign12520_e6965_d_n6, assign12520_e6965_d_n7, assign12520_e6965_d_n8, assign12520_e6965_d_n9, assign12520_e6965_d_n10, assign12520_e6965_d_n13,) = {
    if (locals.var_guard284 == 0.0) {
        let assign12520_e6963: f64 = (p.p108 * locals.var_lg);
        (assign12520_e6963, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12520_e6965;
        locals.var_t1_dn0 = assign12520_e6965_d_n0;
        locals.var_t1_dn2 = assign12520_e6965_d_n2;
        locals.var_t1_dn4 = assign12520_e6965_d_n4;
        locals.var_t1_dn5 = assign12520_e6965_d_n5;
        locals.var_t1_dn6 = assign12520_e6965_d_n6;
        locals.var_t1_dn7 = assign12520_e6965_d_n7;
        locals.var_t1_dn8 = assign12520_e6965_d_n8;
        locals.var_t1_dn9 = assign12520_e6965_d_n9;
        locals.var_t1_dn10 = assign12520_e6965_d_n10;
        locals.var_t1_dn13 = assign12520_e6965_d_n13;

        let assign12530_e6968: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign12530_e6968;

        let (assign12540_e6975, assign12540_e6975_d_n0, assign12540_e6975_d_n2, assign12540_e6975_d_n4, assign12540_e6975_d_n5, assign12540_e6975_d_n6, assign12540_e6975_d_n7, assign12540_e6975_d_n8, assign12540_e6975_d_n9, assign12540_e6975_d_n10, assign12540_e6975_d_n13,) = {
    if ((locals.var_guard284 == 0.0) && (locals.var_guard286 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12540_e6975;
        locals.var_t1_dn0 = assign12540_e6975_d_n0;
        locals.var_t1_dn2 = assign12540_e6975_d_n2;
        locals.var_t1_dn4 = assign12540_e6975_d_n4;
        locals.var_t1_dn5 = assign12540_e6975_d_n5;
        locals.var_t1_dn6 = assign12540_e6975_d_n6;
        locals.var_t1_dn7 = assign12540_e6975_d_n7;
        locals.var_t1_dn8 = assign12540_e6975_d_n8;
        locals.var_t1_dn9 = assign12540_e6975_d_n9;
        locals.var_t1_dn10 = assign12540_e6975_d_n10;
        locals.var_t1_dn13 = assign12540_e6975_d_n13;

        let (assign12550_e6990, assign12550_e6990_d_n0, assign12550_e6990_d_n2, assign12550_e6990_d_n4, assign12550_e6990_d_n5, assign12550_e6990_d_n6, assign12550_e6990_d_n7, assign12550_e6990_d_n8, assign12550_e6990_d_n9, assign12550_e6990_d_n10, assign12550_e6990_d_n13,) = {
    if (locals.var_guard284 == 0.0) {
        let assign12550_e6980: f64 = (locals.var_t1 * p.p107);
        let assign12550_e6983: f64 = (locals.var_t1 + p.p107);
        let assign12550_e6984: f64 = (assign12550_e6980 / assign12550_e6983);
        let assign12550_e6986: f64 = (assign12550_e6984 + p.p109);
        let assign12550_e6988: f64 = (assign12550_e6986 + 1e-25);
        (assign12550_e6988, ((((locals.var_t1_dn0 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn0)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn2 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn2)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn4 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn4)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn5 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn5)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn6 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn6)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn7 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn7)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn8 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn8)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn9 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn9)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn10 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn10)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn13 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn13)) / (assign12550_e6983 * assign12550_e6983)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn13,)
    }
};
        locals.var_ddlte = assign12550_e6990;
        locals.var_ddlte_dn0 = assign12550_e6990_d_n0;
        locals.var_ddlte_dn2 = assign12550_e6990_d_n2;
        locals.var_ddlte_dn4 = assign12550_e6990_d_n4;
        locals.var_ddlte_dn5 = assign12550_e6990_d_n5;
        locals.var_ddlte_dn6 = assign12550_e6990_d_n6;
        locals.var_ddlte_dn7 = assign12550_e6990_d_n7;
        locals.var_ddlte_dn8 = assign12550_e6990_d_n8;
        locals.var_ddlte_dn9 = assign12550_e6990_d_n9;
        locals.var_ddlte_dn10 = assign12550_e6990_d_n10;
        locals.var_ddlte_dn13 = assign12550_e6990_d_n13;

        let assign12570_e6998: f64 = if locals.var_ddlte < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign12570_e6998;

        let (assign12580_e7002, assign12580_e7002_d_n0, assign12580_e7002_d_n2, assign12580_e7002_d_n4, assign12580_e7002_d_n5, assign12580_e7002_d_n6, assign12580_e7002_d_n7, assign12580_e7002_d_n8, assign12580_e7002_d_n9, assign12580_e7002_d_n10, assign12580_e7002_d_n13,) = {
    if (locals.var_guard288 != 0.0) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn13,)
    }
};
        locals.var_ddlte = assign12580_e7002;
        locals.var_ddlte_dn0 = assign12580_e7002_d_n0;
        locals.var_ddlte_dn2 = assign12580_e7002_d_n2;
        locals.var_ddlte_dn4 = assign12580_e7002_d_n4;
        locals.var_ddlte_dn5 = assign12580_e7002_d_n5;
        locals.var_ddlte_dn6 = assign12580_e7002_d_n6;
        locals.var_ddlte_dn7 = assign12580_e7002_d_n7;
        locals.var_ddlte_dn8 = assign12580_e7002_d_n8;
        locals.var_ddlte_dn9 = assign12580_e7002_d_n9;
        locals.var_ddlte_dn10 = assign12580_e7002_d_n10;
        locals.var_ddlte_dn13 = assign12580_e7002_d_n13;

        let (assign12590_e7008, assign12590_e7008_d_n0, assign12590_e7008_d_n2, assign12590_e7008_d_n4, assign12590_e7008_d_n5, assign12590_e7008_d_n6, assign12590_e7008_d_n7, assign12590_e7008_d_n8, assign12590_e7008_d_n9, assign12590_e7008_d_n10, assign12590_e7008_d_n13,) = {
    if (p.p23 != 0.0) {
        let assign12590_e7006: f64 = (locals.var_weff).powf(p.p201);
        (assign12590_e7006, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign12590_e7008;
        locals.var_t2_dn0 = assign12590_e7008_d_n0;
        locals.var_t2_dn2 = assign12590_e7008_d_n2;
        locals.var_t2_dn4 = assign12590_e7008_d_n4;
        locals.var_t2_dn5 = assign12590_e7008_d_n5;
        locals.var_t2_dn6 = assign12590_e7008_d_n6;
        locals.var_t2_dn7 = assign12590_e7008_d_n7;
        locals.var_t2_dn8 = assign12590_e7008_d_n8;
        locals.var_t2_dn9 = assign12590_e7008_d_n9;
        locals.var_t2_dn10 = assign12590_e7008_d_n10;
        locals.var_t2_dn13 = assign12590_e7008_d_n13;

        let (assign12600_e7026, assign12600_e7026_d_n0, assign12600_e7026_d_n2, assign12600_e7026_d_n4, assign12600_e7026_d_n5, assign12600_e7026_d_n6, assign12600_e7026_d_n7, assign12600_e7026_d_n8, assign12600_e7026_d_n9, assign12600_e7026_d_n10, assign12600_e7026_d_n13,) = {
    if (p.p23 != 0.0) {
        let assign12600_e7015: f64 = (locals.var_lgate).powf(p.p199);
        let assign12600_e7016: f64 = (locals.var_mks_svgsl / assign12600_e7015);
        let assign12600_e7017: f64 = (1.0 + assign12600_e7016);
        let assign12600_e7018: f64 = (locals.var_uc_svgs * assign12600_e7017);
        let assign12600_e7022: f64 = (locals.var_t2 + locals.var_mks_svgsw);
        let assign12600_e7023: f64 = (locals.var_t2 / assign12600_e7022);
        let assign12600_e7024: f64 = (assign12600_e7018 * assign12600_e7023);
        (assign12600_e7024, (assign12600_e7018 * (((locals.var_t2_dn0 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn0)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn2 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn2)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn4 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn4)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn5 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn5)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn6 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn6)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn7 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn7)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn8 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn8)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn9 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn9)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn10 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn10)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn13 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn13)) / (assign12600_e7022 * assign12600_e7022))),)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn13,)
    }
};
        locals.var_vg2const = assign12600_e7026;
        locals.var_vg2const_dn0 = assign12600_e7026_d_n0;
        locals.var_vg2const_dn2 = assign12600_e7026_d_n2;
        locals.var_vg2const_dn4 = assign12600_e7026_d_n4;
        locals.var_vg2const_dn5 = assign12600_e7026_d_n5;
        locals.var_vg2const_dn6 = assign12600_e7026_d_n6;
        locals.var_vg2const_dn7 = assign12600_e7026_d_n7;
        locals.var_vg2const_dn8 = assign12600_e7026_d_n8;
        locals.var_vg2const_dn9 = assign12600_e7026_d_n9;
        locals.var_vg2const_dn10 = assign12600_e7026_d_n10;
        locals.var_vg2const_dn13 = assign12600_e7026_d_n13;

        let (assign12610_e7038,) = {
    if (p.p23 != 0.0) {
        let assign12610_e7033: f64 = (locals.var_lgate).powf(p.p184);
        let assign12610_e7034: f64 = (locals.var_mks_svbsl / assign12610_e7033);
        let assign12610_e7035: f64 = (1.0 + assign12610_e7034);
        let assign12610_e7036: f64 = (locals.var_uc_svbs * assign12610_e7035);
        (assign12610_e7036,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12610_e7038;

        let (assign12620_e7050,) = {
    if (p.p23 != 0.0) {
        let assign12620_e7045: f64 = (locals.var_lgate).powf(p.p203);
        let assign12620_e7046: f64 = (locals.var_mks_slgl / assign12620_e7045);
        let assign12620_e7047: f64 = (1.0 + assign12620_e7046);
        let assign12620_e7048: f64 = (locals.var_mks_slg * assign12620_e7047);
        (assign12620_e7048,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12620_e7050;

        let (assign12630_e7062,) = {
    if (p.p23 != 0.0) {
        let assign12630_e7057: f64 = (locals.var_lgate).powf(p.p191);
        let assign12630_e7058: f64 = (locals.var_mks_sub1l / assign12630_e7057);
        let assign12630_e7059: f64 = (1.0 + assign12630_e7058);
        let assign12630_e7060: f64 = (locals.var_uc_sub1 * assign12630_e7059);
        (assign12630_e7060,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12630_e7062;

        let (assign12640_e7072,) = {
    if (p.p23 != 0.0) {
        let assign12640_e7068: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12640_e7069: f64 = (1.0 + assign12640_e7068);
        let assign12640_e7070: f64 = (locals.var_uc_sub2 * assign12640_e7069);
        (assign12640_e7070,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12640_e7072;

        let (assign12650_e7076,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub1,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12650_e7076;

        let (assign12660_e7080,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub2,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12660_e7080;

        let (assign12670_e7084, assign12670_e7084_d_n0, assign12670_e7084_d_n2, assign12670_e7084_d_n4, assign12670_e7084_d_n5, assign12670_e7084_d_n6, assign12670_e7084_d_n7, assign12670_e7084_d_n8, assign12670_e7084_d_n9, assign12670_e7084_d_n10, assign12670_e7084_d_n13,) = {
    if (p.p23 != 0.0) {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn13,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn13,)
    }
};
        locals.var_vg2const_1 = assign12670_e7084;
        locals.var_vg2const_1_dn0 = assign12670_e7084_d_n0;
        locals.var_vg2const_1_dn2 = assign12670_e7084_d_n2;
        locals.var_vg2const_1_dn4 = assign12670_e7084_d_n4;
        locals.var_vg2const_1_dn5 = assign12670_e7084_d_n5;
        locals.var_vg2const_1_dn6 = assign12670_e7084_d_n6;
        locals.var_vg2const_1_dn7 = assign12670_e7084_d_n7;
        locals.var_vg2const_1_dn8 = assign12670_e7084_d_n8;
        locals.var_vg2const_1_dn9 = assign12670_e7084_d_n9;
        locals.var_vg2const_1_dn10 = assign12670_e7084_d_n10;
        locals.var_vg2const_1_dn13 = assign12670_e7084_d_n13;

        let (assign12680_e7088,) = {
    if (p.p23 != 0.0) {
        (locals.var_xvbs,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12680_e7088;

        let (assign12690_e7092,) = {
    if (p.p23 != 0.0) {
        (locals.var_xgate,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12690_e7092;

        let (assign12700_e7106,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12700_e7101: f64 = (locals.var_lgate).powf(p.p191);
        let assign12700_e7102: f64 = (locals.var_mks_sub1l / assign12700_e7101);
        let assign12700_e7103: f64 = (1.0 + assign12700_e7102);
        let assign12700_e7104: f64 = (locals.var_uc_sub1snp * assign12700_e7103);
        (assign12700_e7104,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12700_e7106;

        let (assign12710_e7118,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12710_e7114: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12710_e7115: f64 = (1.0 + assign12710_e7114);
        let assign12710_e7116: f64 = (locals.var_uc_sub2snp * assign12710_e7115);
        (assign12710_e7116,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12710_e7118;

        let (assign12720_e7130,) = {
    if (p.p23 != 0.0) {
        let assign12720_e7125: f64 = (locals.var_lg).powf(p.p103);
        let assign12720_e7126: f64 = (p.p102 / assign12720_e7125);
        let assign12720_e7127: f64 = (1.0 + assign12720_e7126);
        let assign12720_e7128: f64 = (p.p72 * assign12720_e7127);
        (assign12720_e7128,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12720_e7130;

        let (assign12730_e7135, assign12730_e7135_d_n0, assign12730_e7135_d_n2, assign12730_e7135_d_n4, assign12730_e7135_d_n5, assign12730_e7135_d_n6, assign12730_e7135_d_n7, assign12730_e7135_d_n8, assign12730_e7135_d_n9, assign12730_e7135_d_n10, assign12730_e7135_d_n13,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn13,)
    }
};
        locals.var_vg2const = assign12730_e7135;
        locals.var_vg2const_dn0 = assign12730_e7135_d_n0;
        locals.var_vg2const_dn2 = assign12730_e7135_d_n2;
        locals.var_vg2const_dn4 = assign12730_e7135_d_n4;
        locals.var_vg2const_dn5 = assign12730_e7135_d_n5;
        locals.var_vg2const_dn6 = assign12730_e7135_d_n6;
        locals.var_vg2const_dn7 = assign12730_e7135_d_n7;
        locals.var_vg2const_dn8 = assign12730_e7135_d_n8;
        locals.var_vg2const_dn9 = assign12730_e7135_d_n9;
        locals.var_vg2const_dn10 = assign12730_e7135_d_n10;
        locals.var_vg2const_dn13 = assign12730_e7135_d_n13;

        let (assign12740_e7140,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12740_e7140;

        let (assign12750_e7145,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12750_e7145;

        let (assign12760_e7150,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12760_e7150;

        let (assign12770_e7155,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12770_e7155;

        let (assign12780_e7160,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12780_e7160;

        let (assign12790_e7165, assign12790_e7165_d_n0, assign12790_e7165_d_n2, assign12790_e7165_d_n4, assign12790_e7165_d_n5, assign12790_e7165_d_n6, assign12790_e7165_d_n7, assign12790_e7165_d_n8, assign12790_e7165_d_n9, assign12790_e7165_d_n10, assign12790_e7165_d_n13,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn13,)
    }
};
        locals.var_vg2const_1 = assign12790_e7165;
        locals.var_vg2const_1_dn0 = assign12790_e7165_d_n0;
        locals.var_vg2const_1_dn2 = assign12790_e7165_d_n2;
        locals.var_vg2const_1_dn4 = assign12790_e7165_d_n4;
        locals.var_vg2const_1_dn5 = assign12790_e7165_d_n5;
        locals.var_vg2const_1_dn6 = assign12790_e7165_d_n6;
        locals.var_vg2const_1_dn7 = assign12790_e7165_d_n7;
        locals.var_vg2const_1_dn8 = assign12790_e7165_d_n8;
        locals.var_vg2const_1_dn9 = assign12790_e7165_d_n9;
        locals.var_vg2const_1_dn10 = assign12790_e7165_d_n10;
        locals.var_vg2const_1_dn13 = assign12790_e7165_d_n13;

        let (assign12800_e7170,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12800_e7170;

    }

    pub(super) fn stamp_transient_block_20(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign12810_e7175,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12810_e7175;

        let (assign12820_e7180,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12820_e7180;

        let (assign12830_e7185,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12830_e7185;

        let (assign12840_e7199,) = {
    if (locals.var_uc_ibpc1 != 0.0) {
        let assign12840_e7194: f64 = (locals.var_lg).powf(p.p280);
        let assign12840_e7195: f64 = (p.p279 / assign12840_e7194);
        let assign12840_e7196: f64 = (1.0 + assign12840_e7195);
        let assign12840_e7197: f64 = (locals.var_uc_ibpc1 * assign12840_e7196);
        (assign12840_e7197,)
    } else {
        (0.0,)
    }
};
        locals.var_uc_ibpc1 = assign12840_e7199;

        let assign12850_e7203: f64 = (3.141592653589793 / 2.0);
        let assign12850_e7204: f64 = (3.453133e-11 / assign12850_e7203);
        let assign12850_e7206: f64 = (assign12850_e7204 * locals.var_weffcv_nf);
        let assign12850_e7210: f64 = (p.p225 / p.p95);
        let assign12850_e7211: f64 = (1.0 + assign12850_e7210);
        let assign12850_e7212: f64 = (assign12850_e7211).ln();
        let assign12850_e7213: f64 = (assign12850_e7206 * assign12850_e7212);
        locals.var_cfrng = assign12850_e7213;

        let (assign12860_e7227,) = {
    if (p.p134 != 0.0) {
        let assign12860_e7219: f64 = (1000000.0 * locals.var_weffcv_nf);
        let assign12860_e7221: f64 = (assign12860_e7219 * p.p134);
        let assign12860_e7224: f64 = (locals.var_lg).powf(p.p135);
        let assign12860_e7225: f64 = (assign12860_e7221 / assign12860_e7224);
        (assign12860_e7225,)
    } else {
        (0.0,)
    }
};
        locals.var_cqyb0 = assign12860_e7227;

        let assign12870_e7231: f64 = (-p.p286);
        let assign12870_e7232: f64 = (locals.var_lg).powf(assign12870_e7231);
        let assign12870_e7233: f64 = (p.p283 * assign12870_e7232);
        locals.var_ptl0 = assign12870_e7233;

        let assign12880_e7237: f64 = (-p.p291);
        let assign12880_e7238: f64 = (locals.var_lg).powf(assign12880_e7237);
        let assign12880_e7239: f64 = (p.p290 * assign12880_e7238);
        locals.var_pt40 = assign12880_e7239;

        let assign12890_e7243: f64 = (locals.var_lg + locals.var_uc_gdld);
        let assign12890_e7245: f64 = (-p.p288);
        let assign12890_e7246: f64 = (assign12890_e7243).powf(assign12890_e7245);
        let assign12890_e7247: f64 = (p.p287 * assign12890_e7246);
        locals.var_gdl0 = assign12890_e7247;

        let assign12900_e7251: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12900_e7252: f64 = (locals.var_uc_rth0 / assign12900_e7251);
        let assign12900_e7257: f64 = (locals.var_lg).powf(p.p318);
        let assign12900_e7258: f64 = (p.p317 / assign12900_e7257);
        let assign12900_e7259: f64 = (1.0 + assign12900_e7258);
        let assign12900_e7260: f64 = (assign12900_e7252 * assign12900_e7259);
        let assign12900_e7265: f64 = (locals.var_wg).powf(p.p316);
        let assign12900_e7266: f64 = (p.p315 / assign12900_e7265);
        let assign12900_e7267: f64 = (1.0 + assign12900_e7266);
        let assign12900_e7268: f64 = (assign12900_e7260 * assign12900_e7267);
        locals.var_rth = assign12900_e7268;
        locals.var_rth_dn0 = 0.0;
        locals.var_rth_dn2 = 0.0;
        locals.var_rth_dn4 = 0.0;
        locals.var_rth_dn5 = 0.0;
        locals.var_rth_dn6 = 0.0;
        locals.var_rth_dn7 = 0.0;
        locals.var_rth_dn8 = 0.0;
        locals.var_rth_dn9 = 0.0;
        locals.var_rth_dn10 = 0.0;
        locals.var_rth_dn13 = 0.0;

        let assign12920_e7278: f64 = (p.p7).powf(p.p327);
        let assign12920_e7279: f64 = (1.0 / assign12920_e7278);
        let assign12920_e7280: f64 = (locals.var_rth * assign12920_e7279);
        locals.var_rth = assign12920_e7280;
        locals.var_rth_dn0 = (locals.var_rth_dn0 * assign12920_e7279);
        locals.var_rth_dn2 = (locals.var_rth_dn2 * assign12920_e7279);
        locals.var_rth_dn4 = (locals.var_rth_dn4 * assign12920_e7279);
        locals.var_rth_dn5 = (locals.var_rth_dn5 * assign12920_e7279);
        locals.var_rth_dn6 = (locals.var_rth_dn6 * assign12920_e7279);
        locals.var_rth_dn7 = (locals.var_rth_dn7 * assign12920_e7279);
        locals.var_rth_dn8 = (locals.var_rth_dn8 * assign12920_e7279);
        locals.var_rth_dn9 = (locals.var_rth_dn9 * assign12920_e7279);
        locals.var_rth_dn10 = (locals.var_rth_dn10 * assign12920_e7279);
        locals.var_rth_dn13 = (locals.var_rth_dn13 * assign12920_e7279);

        let assign12930_e7284: f64 = (p.p7).powf(p.p327);
        let assign12930_e7285: f64 = (1.0 / assign12930_e7284);
        let assign12930_e7288: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12930_e7289: f64 = (assign12930_e7285 / assign12930_e7288);
        let assign12930_e7294: f64 = (locals.var_lg).powf(p.p318);
        let assign12930_e7295: f64 = (p.p317 / assign12930_e7294);
        let assign12930_e7296: f64 = (1.0 + assign12930_e7295);
        let assign12930_e7297: f64 = (assign12930_e7289 * assign12930_e7296);
        let assign12930_e7302: f64 = (locals.var_wg).powf(p.p316);
        let assign12930_e7303: f64 = (p.p315 / assign12930_e7302);
        let assign12930_e7304: f64 = (1.0 + assign12930_e7303);
        let assign12930_e7305: f64 = (assign12930_e7297 * assign12930_e7304);
        locals.var_rthtemp0 = assign12930_e7305;

        let assign12940_e7312: f64 = if ((p.p53 == 0.0) || (locals.var_uc_rth0 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard289 = assign12940_e7312;

        let (assign12950_e7316, assign12950_e7316_d_n0, assign12950_e7316_d_n2, assign12950_e7316_d_n4, assign12950_e7316_d_n5, assign12950_e7316_d_n6, assign12950_e7316_d_n7, assign12950_e7316_d_n8, assign12950_e7316_d_n9, assign12950_e7316_d_n10, assign12950_e7316_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign12950_e7316;
        locals.var_cnst0over_dn0 = assign12950_e7316_d_n0;
        locals.var_cnst0over_dn2 = assign12950_e7316_d_n2;
        locals.var_cnst0over_dn4 = assign12950_e7316_d_n4;
        locals.var_cnst0over_dn5 = assign12950_e7316_d_n5;
        locals.var_cnst0over_dn6 = assign12950_e7316_d_n6;
        locals.var_cnst0over_dn7 = assign12950_e7316_d_n7;
        locals.var_cnst0over_dn8 = assign12950_e7316_d_n8;
        locals.var_cnst0over_dn9 = assign12950_e7316_d_n9;
        locals.var_cnst0over_dn10 = assign12950_e7316_d_n10;
        locals.var_cnst0over_dn13 = assign12950_e7316_d_n13;

        let (assign12960_e7320, assign12960_e7320_d_n0, assign12960_e7320_d_n2, assign12960_e7320_d_n4, assign12960_e7320_d_n5, assign12960_e7320_d_n6, assign12960_e7320_d_n7, assign12960_e7320_d_n8, assign12960_e7320_d_n9, assign12960_e7320_d_n10, assign12960_e7320_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign12960_e7320;
        locals.var_cnst0overs_dn0 = assign12960_e7320_d_n0;
        locals.var_cnst0overs_dn2 = assign12960_e7320_d_n2;
        locals.var_cnst0overs_dn4 = assign12960_e7320_d_n4;
        locals.var_cnst0overs_dn5 = assign12960_e7320_d_n5;
        locals.var_cnst0overs_dn6 = assign12960_e7320_d_n6;
        locals.var_cnst0overs_dn7 = assign12960_e7320_d_n7;
        locals.var_cnst0overs_dn8 = assign12960_e7320_d_n8;
        locals.var_cnst0overs_dn9 = assign12960_e7320_d_n9;
        locals.var_cnst0overs_dn10 = assign12960_e7320_d_n10;
        locals.var_cnst0overs_dn13 = assign12960_e7320_d_n13;

        let (assign12970_e7326, assign12970_e7326_d_n0, assign12970_e7326_d_n2, assign12970_e7326_d_n4, assign12970_e7326_d_n5, assign12970_e7326_d_n6, assign12970_e7326_d_n7, assign12970_e7326_d_n8, assign12970_e7326_d_n9, assign12970_e7326_d_n10, assign12970_e7326_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign12970_e7322: f64 = ctx_temp;
        let assign12970_e7324: f64 = (assign12970_e7322 + p.p11);
        (assign12970_e7324, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign12970_e7326;
        locals.var_ttemp_dn0 = assign12970_e7326_d_n0;
        locals.var_ttemp_dn2 = assign12970_e7326_d_n2;
        locals.var_ttemp_dn4 = assign12970_e7326_d_n4;
        locals.var_ttemp_dn5 = assign12970_e7326_d_n5;
        locals.var_ttemp_dn6 = assign12970_e7326_d_n6;
        locals.var_ttemp_dn7 = assign12970_e7326_d_n7;
        locals.var_ttemp_dn8 = assign12970_e7326_d_n8;
        locals.var_ttemp_dn9 = assign12970_e7326_d_n9;
        locals.var_ttemp_dn10 = assign12970_e7326_d_n10;
        locals.var_ttemp_dn13 = assign12970_e7326_d_n13;

        let (assign12980_e7330, assign12980_e7330_d_n0, assign12980_e7330_d_n2, assign12980_e7330_d_n4, assign12980_e7330_d_n5, assign12980_e7330_d_n6, assign12980_e7330_d_n7, assign12980_e7330_d_n8, assign12980_e7330_d_n9, assign12980_e7330_d_n10, assign12980_e7330_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn13,)
    }
};
        locals.var_ttemp0 = assign12980_e7330;
        locals.var_ttemp0_dn0 = assign12980_e7330_d_n0;
        locals.var_ttemp0_dn2 = assign12980_e7330_d_n2;
        locals.var_ttemp0_dn4 = assign12980_e7330_d_n4;
        locals.var_ttemp0_dn5 = assign12980_e7330_d_n5;
        locals.var_ttemp0_dn6 = assign12980_e7330_d_n6;
        locals.var_ttemp0_dn7 = assign12980_e7330_d_n7;
        locals.var_ttemp0_dn8 = assign12980_e7330_d_n8;
        locals.var_ttemp0_dn9 = assign12980_e7330_d_n9;
        locals.var_ttemp0_dn10 = assign12980_e7330_d_n10;
        locals.var_ttemp0_dn13 = assign12980_e7330_d_n13;

        let (assign12990_e7336, assign12990_e7336_d_n0, assign12990_e7336_d_n2, assign12990_e7336_d_n4, assign12990_e7336_d_n5, assign12990_e7336_d_n6, assign12990_e7336_d_n7, assign12990_e7336_d_n8, assign12990_e7336_d_n9, assign12990_e7336_d_n10, assign12990_e7336_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign12990_e7334: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign12990_e7334, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn13 + locals.var_deltemp_dn13),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign12990_e7336;
        locals.var_ttemp_dn0 = assign12990_e7336_d_n0;
        locals.var_ttemp_dn2 = assign12990_e7336_d_n2;
        locals.var_ttemp_dn4 = assign12990_e7336_d_n4;
        locals.var_ttemp_dn5 = assign12990_e7336_d_n5;
        locals.var_ttemp_dn6 = assign12990_e7336_d_n6;
        locals.var_ttemp_dn7 = assign12990_e7336_d_n7;
        locals.var_ttemp_dn8 = assign12990_e7336_d_n8;
        locals.var_ttemp_dn9 = assign12990_e7336_d_n9;
        locals.var_ttemp_dn10 = assign12990_e7336_d_n10;
        locals.var_ttemp_dn13 = assign12990_e7336_d_n13;

        let (assign13000_e7342, assign13000_e7342_d_n0, assign13000_e7342_d_n2, assign13000_e7342_d_n4, assign13000_e7342_d_n5, assign13000_e7342_d_n6, assign13000_e7342_d_n7, assign13000_e7342_d_n8, assign13000_e7342_d_n9, assign13000_e7342_d_n10, assign13000_e7342_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13000_e7340: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign13000_e7340, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn13,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn13,)
    }
};
        locals.var_tdiff0 = assign13000_e7342;
        locals.var_tdiff0_dn0 = assign13000_e7342_d_n0;
        locals.var_tdiff0_dn2 = assign13000_e7342_d_n2;
        locals.var_tdiff0_dn4 = assign13000_e7342_d_n4;
        locals.var_tdiff0_dn5 = assign13000_e7342_d_n5;
        locals.var_tdiff0_dn6 = assign13000_e7342_d_n6;
        locals.var_tdiff0_dn7 = assign13000_e7342_d_n7;
        locals.var_tdiff0_dn8 = assign13000_e7342_d_n8;
        locals.var_tdiff0_dn9 = assign13000_e7342_d_n9;
        locals.var_tdiff0_dn10 = assign13000_e7342_d_n10;
        locals.var_tdiff0_dn13 = assign13000_e7342_d_n13;

        let (assign13010_e7352, assign13010_e7352_d_n0, assign13010_e7352_d_n2, assign13010_e7352_d_n4, assign13010_e7352_d_n5, assign13010_e7352_d_n6, assign13010_e7352_d_n7, assign13010_e7352_d_n8, assign13010_e7352_d_n9, assign13010_e7352_d_n10, assign13010_e7352_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13010_e7346: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign13010_e7349: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13010_e7350: f64 = (assign13010_e7346 - assign13010_e7349);
        (assign13010_e7350, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn13 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn13)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn13,)
    }
};
        locals.var_tdiff0_2 = assign13010_e7352;
        locals.var_tdiff0_2_dn0 = assign13010_e7352_d_n0;
        locals.var_tdiff0_2_dn2 = assign13010_e7352_d_n2;
        locals.var_tdiff0_2_dn4 = assign13010_e7352_d_n4;
        locals.var_tdiff0_2_dn5 = assign13010_e7352_d_n5;
        locals.var_tdiff0_2_dn6 = assign13010_e7352_d_n6;
        locals.var_tdiff0_2_dn7 = assign13010_e7352_d_n7;
        locals.var_tdiff0_2_dn8 = assign13010_e7352_d_n8;
        locals.var_tdiff0_2_dn9 = assign13010_e7352_d_n9;
        locals.var_tdiff0_2_dn10 = assign13010_e7352_d_n10;
        locals.var_tdiff0_2_dn13 = assign13010_e7352_d_n13;

        let (assign13020_e7358, assign13020_e7358_d_n0, assign13020_e7358_d_n2, assign13020_e7358_d_n4, assign13020_e7358_d_n5, assign13020_e7358_d_n6, assign13020_e7358_d_n7, assign13020_e7358_d_n8, assign13020_e7358_d_n9, assign13020_e7358_d_n10, assign13020_e7358_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13020_e7356: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign13020_e7356, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn13,)
    }
};
        locals.var_tdiff = assign13020_e7358;
        locals.var_tdiff_dn0 = assign13020_e7358_d_n0;
        locals.var_tdiff_dn2 = assign13020_e7358_d_n2;
        locals.var_tdiff_dn4 = assign13020_e7358_d_n4;
        locals.var_tdiff_dn5 = assign13020_e7358_d_n5;
        locals.var_tdiff_dn6 = assign13020_e7358_d_n6;
        locals.var_tdiff_dn7 = assign13020_e7358_d_n7;
        locals.var_tdiff_dn8 = assign13020_e7358_d_n8;
        locals.var_tdiff_dn9 = assign13020_e7358_d_n9;
        locals.var_tdiff_dn10 = assign13020_e7358_d_n10;
        locals.var_tdiff_dn13 = assign13020_e7358_d_n13;

        let (assign13030_e7368, assign13030_e7368_d_n0, assign13030_e7368_d_n2, assign13030_e7368_d_n4, assign13030_e7368_d_n5, assign13030_e7368_d_n6, assign13030_e7368_d_n7, assign13030_e7368_d_n8, assign13030_e7368_d_n9, assign13030_e7368_d_n10, assign13030_e7368_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13030_e7362: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign13030_e7365: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13030_e7366: f64 = (assign13030_e7362 - assign13030_e7365);
        (assign13030_e7366, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn13 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn13,)
    }
};
        locals.var_tdiff_2 = assign13030_e7368;
        locals.var_tdiff_2_dn0 = assign13030_e7368_d_n0;
        locals.var_tdiff_2_dn2 = assign13030_e7368_d_n2;
        locals.var_tdiff_2_dn4 = assign13030_e7368_d_n4;
        locals.var_tdiff_2_dn5 = assign13030_e7368_d_n5;
        locals.var_tdiff_2_dn6 = assign13030_e7368_d_n6;
        locals.var_tdiff_2_dn7 = assign13030_e7368_d_n7;
        locals.var_tdiff_2_dn8 = assign13030_e7368_d_n8;
        locals.var_tdiff_2_dn9 = assign13030_e7368_d_n9;
        locals.var_tdiff_2_dn10 = assign13030_e7368_d_n10;
        locals.var_tdiff_2_dn13 = assign13030_e7368_d_n13;

        let (assign13040_e7374, assign13040_e7374_d_n0, assign13040_e7374_d_n2, assign13040_e7374_d_n4, assign13040_e7374_d_n5, assign13040_e7374_d_n6, assign13040_e7374_d_n7, assign13040_e7374_d_n8, assign13040_e7374_d_n9, assign13040_e7374_d_n10, assign13040_e7374_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13040_e7372: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13040_e7372, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn13,)
    }
};
        locals.var_tratio = assign13040_e7374;
        locals.var_tratio_dn0 = assign13040_e7374_d_n0;
        locals.var_tratio_dn2 = assign13040_e7374_d_n2;
        locals.var_tratio_dn4 = assign13040_e7374_d_n4;
        locals.var_tratio_dn5 = assign13040_e7374_d_n5;
        locals.var_tratio_dn6 = assign13040_e7374_d_n6;
        locals.var_tratio_dn7 = assign13040_e7374_d_n7;
        locals.var_tratio_dn8 = assign13040_e7374_d_n8;
        locals.var_tratio_dn9 = assign13040_e7374_d_n9;
        locals.var_tratio_dn10 = assign13040_e7374_d_n10;
        locals.var_tratio_dn13 = assign13040_e7374_d_n13;

        let (assign13050_e7379, assign13050_e7379_d_n0, assign13050_e7379_d_n2, assign13050_e7379_d_n4, assign13050_e7379_d_n5, assign13050_e7379_d_n6, assign13050_e7379_d_n7, assign13050_e7379_d_n8, assign13050_e7379_d_n9, assign13050_e7379_d_n10, assign13050_e7379_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13050_e7377: f64 = (locals.var_tratio).ln();
        (assign13050_e7377, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn13 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn13,)
    }
};
        locals.var_log_tratio = assign13050_e7379;
        locals.var_log_tratio_dn0 = assign13050_e7379_d_n0;
        locals.var_log_tratio_dn2 = assign13050_e7379_d_n2;
        locals.var_log_tratio_dn4 = assign13050_e7379_d_n4;
        locals.var_log_tratio_dn5 = assign13050_e7379_d_n5;
        locals.var_log_tratio_dn6 = assign13050_e7379_d_n6;
        locals.var_log_tratio_dn7 = assign13050_e7379_d_n7;
        locals.var_log_tratio_dn8 = assign13050_e7379_d_n8;
        locals.var_log_tratio_dn9 = assign13050_e7379_d_n9;
        locals.var_log_tratio_dn10 = assign13050_e7379_d_n10;
        locals.var_log_tratio_dn13 = assign13050_e7379_d_n13;

        let (assign13060_e7391, assign13060_e7391_d_n0, assign13060_e7391_d_n2, assign13060_e7391_d_n4, assign13060_e7391_d_n5, assign13060_e7391_d_n6, assign13060_e7391_d_n7, assign13060_e7391_d_n8, assign13060_e7391_d_n9, assign13060_e7391_d_n10, assign13060_e7391_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13060_e7384: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign13060_e7385: f64 = (locals.var_egtnom - assign13060_e7384);
        let assign13060_e7388: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign13060_e7389: f64 = (assign13060_e7385 - assign13060_e7388);
        (assign13060_e7389, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn13)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn13,)
    }
};
        locals.var_eg = assign13060_e7391;
        locals.var_eg_dn0 = assign13060_e7391_d_n0;
        locals.var_eg_dn2 = assign13060_e7391_d_n2;
        locals.var_eg_dn4 = assign13060_e7391_d_n4;
        locals.var_eg_dn5 = assign13060_e7391_d_n5;
        locals.var_eg_dn6 = assign13060_e7391_d_n6;
        locals.var_eg_dn7 = assign13060_e7391_d_n7;
        locals.var_eg_dn8 = assign13060_e7391_d_n8;
        locals.var_eg_dn9 = assign13060_e7391_d_n9;
        locals.var_eg_dn10 = assign13060_e7391_d_n10;
        locals.var_eg_dn13 = assign13060_e7391_d_n13;

        let (assign13070_e7396, assign13070_e7396_d_n0, assign13070_e7396_d_n2, assign13070_e7396_d_n4, assign13070_e7396_d_n5, assign13070_e7396_d_n6, assign13070_e7396_d_n7, assign13070_e7396_d_n8, assign13070_e7396_d_n9, assign13070_e7396_d_n10, assign13070_e7396_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13070_e7394: f64 = (locals.var_eg).sqrt();
        (assign13070_e7394, (locals.var_eg_dn0 / (2.0 * assign13070_e7394)), (locals.var_eg_dn2 / (2.0 * assign13070_e7394)), (locals.var_eg_dn4 / (2.0 * assign13070_e7394)), (locals.var_eg_dn5 / (2.0 * assign13070_e7394)), (locals.var_eg_dn6 / (2.0 * assign13070_e7394)), (locals.var_eg_dn7 / (2.0 * assign13070_e7394)), (locals.var_eg_dn8 / (2.0 * assign13070_e7394)), (locals.var_eg_dn9 / (2.0 * assign13070_e7394)), (locals.var_eg_dn10 / (2.0 * assign13070_e7394)), (locals.var_eg_dn13 / (2.0 * assign13070_e7394)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn13,)
    }
};
        locals.var_sqrt_eg = assign13070_e7396;
        locals.var_sqrt_eg_dn0 = assign13070_e7396_d_n0;
        locals.var_sqrt_eg_dn2 = assign13070_e7396_d_n2;
        locals.var_sqrt_eg_dn4 = assign13070_e7396_d_n4;
        locals.var_sqrt_eg_dn5 = assign13070_e7396_d_n5;
        locals.var_sqrt_eg_dn6 = assign13070_e7396_d_n6;
        locals.var_sqrt_eg_dn7 = assign13070_e7396_d_n7;
        locals.var_sqrt_eg_dn8 = assign13070_e7396_d_n8;
        locals.var_sqrt_eg_dn9 = assign13070_e7396_d_n9;
        locals.var_sqrt_eg_dn10 = assign13070_e7396_d_n10;
        locals.var_sqrt_eg_dn13 = assign13070_e7396_d_n13;

        let (assign13080_e7402, assign13080_e7402_d_n0, assign13080_e7402_d_n2, assign13080_e7402_d_n4, assign13080_e7402_d_n5, assign13080_e7402_d_n6, assign13080_e7402_d_n7, assign13080_e7402_d_n8, assign13080_e7402_d_n9, assign13080_e7402_d_n10, assign13080_e7402_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13080_e7400: f64 = (1.0 / locals.var_ttemp);
        (assign13080_e7400, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn13 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13080_e7402;
        locals.var_t1_dn0 = assign13080_e7402_d_n0;
        locals.var_t1_dn2 = assign13080_e7402_d_n2;
        locals.var_t1_dn4 = assign13080_e7402_d_n4;
        locals.var_t1_dn5 = assign13080_e7402_d_n5;
        locals.var_t1_dn6 = assign13080_e7402_d_n6;
        locals.var_t1_dn7 = assign13080_e7402_d_n7;
        locals.var_t1_dn8 = assign13080_e7402_d_n8;
        locals.var_t1_dn9 = assign13080_e7402_d_n9;
        locals.var_t1_dn10 = assign13080_e7402_d_n10;
        locals.var_t1_dn13 = assign13080_e7402_d_n13;

        let (assign13090_e7408, assign13090_e7408_d_n0, assign13090_e7408_d_n2, assign13090_e7408_d_n4, assign13090_e7408_d_n5, assign13090_e7408_d_n6, assign13090_e7408_d_n7, assign13090_e7408_d_n8, assign13090_e7408_d_n9, assign13090_e7408_d_n10, assign13090_e7408_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13090_e7406: f64 = (1.0 / locals.var_ktnom);
        (assign13090_e7406, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign13090_e7408;
        locals.var_t2_dn0 = assign13090_e7408_d_n0;
        locals.var_t2_dn2 = assign13090_e7408_d_n2;
        locals.var_t2_dn4 = assign13090_e7408_d_n4;
        locals.var_t2_dn5 = assign13090_e7408_d_n5;
        locals.var_t2_dn6 = assign13090_e7408_d_n6;
        locals.var_t2_dn7 = assign13090_e7408_d_n7;
        locals.var_t2_dn8 = assign13090_e7408_d_n8;
        locals.var_t2_dn9 = assign13090_e7408_d_n9;
        locals.var_t2_dn10 = assign13090_e7408_d_n10;
        locals.var_t2_dn13 = assign13090_e7408_d_n13;

        let (assign13100_e7430, assign13100_e7430_d_n0, assign13100_e7430_d_n2, assign13100_e7430_d_n4, assign13100_e7430_d_n5, assign13100_e7430_d_n6, assign13100_e7430_d_n7, assign13100_e7430_d_n8, assign13100_e7430_d_n9, assign13100_e7430_d_n10, assign13100_e7430_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13100_e7412: f64 = (locals.var_egtnom + p.p259);
        let assign13100_e7416: f64 = (locals.var_t1 - locals.var_t2);
        let assign13100_e7417: f64 = (p.p260 * assign13100_e7416);
        let assign13100_e7418: f64 = (assign13100_e7412 + assign13100_e7417);
        let assign13100_e7422: f64 = (locals.var_t1 * locals.var_t1);
        let assign13100_e7425: f64 = (locals.var_t2 * locals.var_t2);
        let assign13100_e7426: f64 = (assign13100_e7422 - assign13100_e7425);
        let assign13100_e7427: f64 = (p.p261 * assign13100_e7426);
        let assign13100_e7428: f64 = (assign13100_e7418 + assign13100_e7427);
        (assign13100_e7428, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn13 - locals.var_t2_dn13)) + (p.p261 * (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) - ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign13100_e7430;
        locals.var_t3_dn0 = assign13100_e7430_d_n0;
        locals.var_t3_dn2 = assign13100_e7430_d_n2;
        locals.var_t3_dn4 = assign13100_e7430_d_n4;
        locals.var_t3_dn5 = assign13100_e7430_d_n5;
        locals.var_t3_dn6 = assign13100_e7430_d_n6;
        locals.var_t3_dn7 = assign13100_e7430_d_n7;
        locals.var_t3_dn8 = assign13100_e7430_d_n8;
        locals.var_t3_dn9 = assign13100_e7430_d_n9;
        locals.var_t3_dn10 = assign13100_e7430_d_n10;
        locals.var_t3_dn13 = assign13100_e7430_d_n13;

        let (assign13110_e7435, assign13110_e7435_d_n0, assign13110_e7435_d_n2, assign13110_e7435_d_n4, assign13110_e7435_d_n5, assign13110_e7435_d_n6, assign13110_e7435_d_n7, assign13110_e7435_d_n8, assign13110_e7435_d_n9, assign13110_e7435_d_n10, assign13110_e7435_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13110_e7433: f64 = (locals.var_t3).sqrt();
        (assign13110_e7433, (locals.var_t3_dn0 / (2.0 * assign13110_e7433)), (locals.var_t3_dn2 / (2.0 * assign13110_e7433)), (locals.var_t3_dn4 / (2.0 * assign13110_e7433)), (locals.var_t3_dn5 / (2.0 * assign13110_e7433)), (locals.var_t3_dn6 / (2.0 * assign13110_e7433)), (locals.var_t3_dn7 / (2.0 * assign13110_e7433)), (locals.var_t3_dn8 / (2.0 * assign13110_e7433)), (locals.var_t3_dn9 / (2.0 * assign13110_e7433)), (locals.var_t3_dn10 / (2.0 * assign13110_e7433)), (locals.var_t3_dn13 / (2.0 * assign13110_e7433)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn13,)
    }
};
        locals.var_egp12 = assign13110_e7435;
        locals.var_egp12_dn0 = assign13110_e7435_d_n0;
        locals.var_egp12_dn2 = assign13110_e7435_d_n2;
        locals.var_egp12_dn4 = assign13110_e7435_d_n4;
        locals.var_egp12_dn5 = assign13110_e7435_d_n5;
        locals.var_egp12_dn6 = assign13110_e7435_d_n6;
        locals.var_egp12_dn7 = assign13110_e7435_d_n7;
        locals.var_egp12_dn8 = assign13110_e7435_d_n8;
        locals.var_egp12_dn9 = assign13110_e7435_d_n9;
        locals.var_egp12_dn10 = assign13110_e7435_d_n10;
        locals.var_egp12_dn13 = assign13110_e7435_d_n13;

        let (assign13120_e7441, assign13120_e7441_d_n0, assign13120_e7441_d_n2, assign13120_e7441_d_n4, assign13120_e7441_d_n5, assign13120_e7441_d_n6, assign13120_e7441_d_n7, assign13120_e7441_d_n8, assign13120_e7441_d_n9, assign13120_e7441_d_n10, assign13120_e7441_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13120_e7439: f64 = (locals.var_t3 * locals.var_egp12);
        (assign13120_e7439, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn13 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn13)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn13,)
    }
};
        locals.var_egp32 = assign13120_e7441;
        locals.var_egp32_dn0 = assign13120_e7441_d_n0;
        locals.var_egp32_dn2 = assign13120_e7441_d_n2;
        locals.var_egp32_dn4 = assign13120_e7441_d_n4;
        locals.var_egp32_dn5 = assign13120_e7441_d_n5;
        locals.var_egp32_dn6 = assign13120_e7441_d_n6;
        locals.var_egp32_dn7 = assign13120_e7441_d_n7;
        locals.var_egp32_dn8 = assign13120_e7441_d_n8;
        locals.var_egp32_dn9 = assign13120_e7441_d_n9;
        locals.var_egp32_dn10 = assign13120_e7441_d_n10;
        locals.var_egp32_dn13 = assign13120_e7441_d_n13;

    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13130_e7449, assign13130_e7449_d_n0, assign13130_e7449_d_n2, assign13130_e7449_d_n4, assign13130_e7449_d_n5, assign13130_e7449_d_n6, assign13130_e7449_d_n7, assign13130_e7449_d_n8, assign13130_e7449_d_n9, assign13130_e7449_d_n10, assign13130_e7449_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13130_e7446: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign13130_e7447: f64 = (1.6021918e-19 / assign13130_e7446);
        (assign13130_e7447, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn13)) / (assign13130_e7446 * assign13130_e7446))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn13,)
    }
};
        locals.var_beta = assign13130_e7449;
        locals.var_beta_dn0 = assign13130_e7449_d_n0;
        locals.var_beta_dn2 = assign13130_e7449_d_n2;
        locals.var_beta_dn4 = assign13130_e7449_d_n4;
        locals.var_beta_dn5 = assign13130_e7449_d_n5;
        locals.var_beta_dn6 = assign13130_e7449_d_n6;
        locals.var_beta_dn7 = assign13130_e7449_d_n7;
        locals.var_beta_dn8 = assign13130_e7449_d_n8;
        locals.var_beta_dn9 = assign13130_e7449_d_n9;
        locals.var_beta_dn10 = assign13130_e7449_d_n10;
        locals.var_beta_dn13 = assign13130_e7449_d_n13;

        let (assign13140_e7455, assign13140_e7455_d_n0, assign13140_e7455_d_n2, assign13140_e7455_d_n4, assign13140_e7455_d_n5, assign13140_e7455_d_n6, assign13140_e7455_d_n7, assign13140_e7455_d_n8, assign13140_e7455_d_n9, assign13140_e7455_d_n10, assign13140_e7455_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13140_e7453: f64 = (1.0 / locals.var_beta);
        (assign13140_e7453, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn13 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn13,)
    }
};
        locals.var_beta_inv = assign13140_e7455;
        locals.var_beta_inv_dn0 = assign13140_e7455_d_n0;
        locals.var_beta_inv_dn2 = assign13140_e7455_d_n2;
        locals.var_beta_inv_dn4 = assign13140_e7455_d_n4;
        locals.var_beta_inv_dn5 = assign13140_e7455_d_n5;
        locals.var_beta_inv_dn6 = assign13140_e7455_d_n6;
        locals.var_beta_inv_dn7 = assign13140_e7455_d_n7;
        locals.var_beta_inv_dn8 = assign13140_e7455_d_n8;
        locals.var_beta_inv_dn9 = assign13140_e7455_d_n9;
        locals.var_beta_inv_dn10 = assign13140_e7455_d_n10;
        locals.var_beta_inv_dn13 = assign13140_e7455_d_n13;

        let (assign13150_e7461, assign13150_e7461_d_n0, assign13150_e7461_d_n2, assign13150_e7461_d_n4, assign13150_e7461_d_n5, assign13150_e7461_d_n6, assign13150_e7461_d_n7, assign13150_e7461_d_n8, assign13150_e7461_d_n9, assign13150_e7461_d_n10, assign13150_e7461_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13150_e7459: f64 = (locals.var_beta * locals.var_beta);
        (assign13150_e7459, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn13 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn13)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn13,)
    }
};
        locals.var_beta2 = assign13150_e7461;
        locals.var_beta2_dn0 = assign13150_e7461_d_n0;
        locals.var_beta2_dn2 = assign13150_e7461_d_n2;
        locals.var_beta2_dn4 = assign13150_e7461_d_n4;
        locals.var_beta2_dn5 = assign13150_e7461_d_n5;
        locals.var_beta2_dn6 = assign13150_e7461_d_n6;
        locals.var_beta2_dn7 = assign13150_e7461_d_n7;
        locals.var_beta2_dn8 = assign13150_e7461_d_n8;
        locals.var_beta2_dn9 = assign13150_e7461_d_n9;
        locals.var_beta2_dn10 = assign13150_e7461_d_n10;
        locals.var_beta2_dn13 = assign13150_e7461_d_n13;

        let (assign13160_e7469,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13160_e7466: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign13160_e7467: f64 = (1.6021918e-19 / assign13160_e7466);
        (assign13160_e7467,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign13160_e7469;

        let (assign13170_e7492, assign13170_e7492_d_n0, assign13170_e7492_d_n2, assign13170_e7492_d_n4, assign13170_e7492_d_n5, assign13170_e7492_d_n6, assign13170_e7492_d_n7, assign13170_e7492_d_n8, assign13170_e7492_d_n9, assign13170_e7492_d_n10, assign13170_e7492_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13170_e7474: f64 = (locals.var_log_tratio * 1.5);
        let assign13170_e7475: f64 = (assign13170_e7474).exp();
        let assign13170_e7476: f64 = (1.04e16 * assign13170_e7475);
        let assign13170_e7478: f64 = (-locals.var_eg);
        let assign13170_e7480: f64 = (assign13170_e7478 / 2.0);
        let assign13170_e7482: f64 = (assign13170_e7480 * locals.var_beta);
        let assign13170_e7485: f64 = (locals.var_egtnom / 2.0);
        let assign13170_e7487: f64 = (assign13170_e7485 * locals.var_betatnom);
        let assign13170_e7488: f64 = (assign13170_e7482 + assign13170_e7487);
        let assign13170_e7489: f64 = (assign13170_e7488).exp();
        let assign13170_e7490: f64 = (assign13170_e7476 * assign13170_e7489);
        (assign13170_e7490, (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn0 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn0))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn2 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn2))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn4 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn4))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn5 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn5))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn6 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn6))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn7 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn7))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn8 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn8))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn9 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn9))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn10 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn10))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn13 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn13) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn13))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn13,)
    }
};
        locals.var_nin = assign13170_e7492;
        locals.var_nin_dn0 = assign13170_e7492_d_n0;
        locals.var_nin_dn2 = assign13170_e7492_d_n2;
        locals.var_nin_dn4 = assign13170_e7492_d_n4;
        locals.var_nin_dn5 = assign13170_e7492_d_n5;
        locals.var_nin_dn6 = assign13170_e7492_d_n6;
        locals.var_nin_dn7 = assign13170_e7492_d_n7;
        locals.var_nin_dn8 = assign13170_e7492_d_n8;
        locals.var_nin_dn9 = assign13170_e7492_d_n9;
        locals.var_nin_dn10 = assign13170_e7492_d_n10;
        locals.var_nin_dn13 = assign13170_e7492_d_n13;

        let (assign13180_e7499, assign13180_e7499_d_n0, assign13180_e7499_d_n2, assign13180_e7499_d_n4, assign13180_e7499_d_n5, assign13180_e7499_d_n6, assign13180_e7499_d_n7, assign13180_e7499_d_n8, assign13180_e7499_d_n9, assign13180_e7499_d_n10, assign13180_e7499_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13180_e7496: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign13180_e7497: f64 = (assign13180_e7496).exp();
        (assign13180_e7497, (assign13180_e7497 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn13 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13180_e7499;
        locals.var_t1_dn0 = assign13180_e7499_d_n0;
        locals.var_t1_dn2 = assign13180_e7499_d_n2;
        locals.var_t1_dn4 = assign13180_e7499_d_n4;
        locals.var_t1_dn5 = assign13180_e7499_d_n5;
        locals.var_t1_dn6 = assign13180_e7499_d_n6;
        locals.var_t1_dn7 = assign13180_e7499_d_n7;
        locals.var_t1_dn8 = assign13180_e7499_d_n8;
        locals.var_t1_dn9 = assign13180_e7499_d_n9;
        locals.var_t1_dn10 = assign13180_e7499_d_n10;
        locals.var_t1_dn13 = assign13180_e7499_d_n13;

        let (assign13190_e7505, assign13190_e7505_d_n0, assign13190_e7505_d_n2, assign13190_e7505_d_n4, assign13190_e7505_d_n5, assign13190_e7505_d_n6, assign13190_e7505_d_n7, assign13190_e7505_d_n8, assign13190_e7505_d_n9, assign13190_e7505_d_n10, assign13190_e7505_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13190_e7503: f64 = (locals.var_t1 / locals.var_mueph);
        (assign13190_e7503, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn13 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn13)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn13,)
    }
};
        locals.var_mphn0 = assign13190_e7505;
        locals.var_mphn0_dn0 = assign13190_e7505_d_n0;
        locals.var_mphn0_dn2 = assign13190_e7505_d_n2;
        locals.var_mphn0_dn4 = assign13190_e7505_d_n4;
        locals.var_mphn0_dn5 = assign13190_e7505_d_n5;
        locals.var_mphn0_dn6 = assign13190_e7505_d_n6;
        locals.var_mphn0_dn7 = assign13190_e7505_d_n7;
        locals.var_mphn0_dn8 = assign13190_e7505_d_n8;
        locals.var_mphn0_dn9 = assign13190_e7505_d_n9;
        locals.var_mphn0_dn10 = assign13190_e7505_d_n10;
        locals.var_mphn0_dn13 = assign13190_e7505_d_n13;

        let assign13200_e7512: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard290 = assign13200_e7512;

        let (assign13210_e7527, assign13210_e7527_d_n0, assign13210_e7527_d_n2, assign13210_e7527_d_n4, assign13210_e7527_d_n5, assign13210_e7527_d_n6, assign13210_e7527_d_n7, assign13210_e7527_d_n8, assign13210_e7527_d_n9, assign13210_e7527_d_n10, assign13210_e7527_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13210_e7518: f64 = (2.0 * 1.034943e-10);
        let assign13210_e7520: f64 = (assign13210_e7518 * 1.6021918e-19);
        let assign13210_e7522: f64 = (assign13210_e7520 * locals.var_uc_ndepm);
        let assign13210_e7524: f64 = (assign13210_e7522 * locals.var_beta_inv);
        let assign13210_e7525: f64 = (assign13210_e7524).sqrt();
        (assign13210_e7525, ((((assign13210_e7520 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn0)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn2)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn4)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn5)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn6)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn7)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn8)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn9)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn10)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn13)) / (2.0 * assign13210_e7525)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign13210_e7527;
        locals.var_cnst0_dn0 = assign13210_e7527_d_n0;
        locals.var_cnst0_dn2 = assign13210_e7527_d_n2;
        locals.var_cnst0_dn4 = assign13210_e7527_d_n4;
        locals.var_cnst0_dn5 = assign13210_e7527_d_n5;
        locals.var_cnst0_dn6 = assign13210_e7527_d_n6;
        locals.var_cnst0_dn7 = assign13210_e7527_d_n7;
        locals.var_cnst0_dn8 = assign13210_e7527_d_n8;
        locals.var_cnst0_dn9 = assign13210_e7527_d_n9;
        locals.var_cnst0_dn10 = assign13210_e7527_d_n10;
        locals.var_cnst0_dn13 = assign13210_e7527_d_n13;

        let (assign13220_e7539, assign13220_e7539_d_n0, assign13220_e7539_d_n2, assign13220_e7539_d_n4, assign13220_e7539_d_n5, assign13220_e7539_d_n6, assign13220_e7539_d_n7, assign13220_e7539_d_n8, assign13220_e7539_d_n9, assign13220_e7539_d_n10, assign13220_e7539_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13220_e7533: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign13220_e7535: f64 = (assign13220_e7533 * __rspice_inv_cse_0);
        let assign13220_e7537: f64 = (assign13220_e7535 * __rspice_inv_cse_0);
        (assign13220_e7537, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign13220_e7539;
        locals.var_cnst1_dn0 = assign13220_e7539_d_n0;
        locals.var_cnst1_dn2 = assign13220_e7539_d_n2;
        locals.var_cnst1_dn4 = assign13220_e7539_d_n4;
        locals.var_cnst1_dn5 = assign13220_e7539_d_n5;
        locals.var_cnst1_dn6 = assign13220_e7539_d_n6;
        locals.var_cnst1_dn7 = assign13220_e7539_d_n7;
        locals.var_cnst1_dn8 = assign13220_e7539_d_n8;
        locals.var_cnst1_dn9 = assign13220_e7539_d_n9;
        locals.var_cnst1_dn10 = assign13220_e7539_d_n10;
        locals.var_cnst1_dn13 = assign13220_e7539_d_n13;

        let (assign13230_e7552, assign13230_e7552_d_n0, assign13230_e7552_d_n2, assign13230_e7552_d_n4, assign13230_e7552_d_n5, assign13230_e7552_d_n6, assign13230_e7552_d_n7, assign13230_e7552_d_n8, assign13230_e7552_d_n9, assign13230_e7552_d_n10, assign13230_e7552_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13230_e7545: f64 = (2.0 * locals.var_beta_inv);
        let assign13230_e7548: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign13230_e7549: f64 = (assign13230_e7548).ln();
        let assign13230_e7550: f64 = (assign13230_e7545 * assign13230_e7549);
        (assign13230_e7550, (((2.0 * locals.var_beta_inv_dn0) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn2) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn4) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn5) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn6) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn7) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn8) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn9) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn10) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn13) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign13230_e7552;
        locals.var_pb2n_dn0 = assign13230_e7552_d_n0;
        locals.var_pb2n_dn2 = assign13230_e7552_d_n2;
        locals.var_pb2n_dn4 = assign13230_e7552_d_n4;
        locals.var_pb2n_dn5 = assign13230_e7552_d_n5;
        locals.var_pb2n_dn6 = assign13230_e7552_d_n6;
        locals.var_pb2n_dn7 = assign13230_e7552_d_n7;
        locals.var_pb2n_dn8 = assign13230_e7552_d_n8;
        locals.var_pb2n_dn9 = assign13230_e7552_d_n9;
        locals.var_pb2n_dn10 = assign13230_e7552_d_n10;
        locals.var_pb2n_dn13 = assign13230_e7552_d_n13;

        let (assign13240_e7567, assign13240_e7567_d_n0, assign13240_e7567_d_n2, assign13240_e7567_d_n4, assign13240_e7567_d_n5, assign13240_e7567_d_n6, assign13240_e7567_d_n7, assign13240_e7567_d_n8, assign13240_e7567_d_n9, assign13240_e7567_d_n10, assign13240_e7567_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13240_e7559: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign13240_e7561: f64 = (assign13240_e7559 * __rspice_inv_cse_1);
        let assign13240_e7563: f64 = (assign13240_e7561 * __rspice_inv_cse_1);
        let assign13240_e7564: f64 = (assign13240_e7563).ln();
        let assign13240_e7565: f64 = (locals.var_beta_inv * assign13240_e7564);
        (assign13240_e7565, ((locals.var_beta_inv_dn0 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn2 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn4 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn5 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn6 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn7 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn8 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn9 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn10 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn13 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign13240_e7567;
        locals.var_vbipn_dn0 = assign13240_e7567_d_n0;
        locals.var_vbipn_dn2 = assign13240_e7567_d_n2;
        locals.var_vbipn_dn4 = assign13240_e7567_d_n4;
        locals.var_vbipn_dn5 = assign13240_e7567_d_n5;
        locals.var_vbipn_dn6 = assign13240_e7567_d_n6;
        locals.var_vbipn_dn7 = assign13240_e7567_d_n7;
        locals.var_vbipn_dn8 = assign13240_e7567_d_n8;
        locals.var_vbipn_dn9 = assign13240_e7567_d_n9;
        locals.var_vbipn_dn10 = assign13240_e7567_d_n10;
        locals.var_vbipn_dn13 = assign13240_e7567_d_n13;

        let (assign13250_e7576, assign13250_e7576_d_n0, assign13250_e7576_d_n2, assign13250_e7576_d_n4, assign13250_e7576_d_n5, assign13250_e7576_d_n6, assign13250_e7576_d_n7, assign13250_e7576_d_n8, assign13250_e7576_d_n9, assign13250_e7576_d_n10, assign13250_e7576_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13250_e7573: f64 = (locals.var_log_tratio * p.p380);
        let assign13250_e7574: f64 = (assign13250_e7573).exp();
        (assign13250_e7574, (assign13250_e7574 * (locals.var_log_tratio_dn0 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn2 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn4 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn5 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn6 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn7 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn8 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn9 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn10 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13250_e7576;
        locals.var_t1_dn0 = assign13250_e7576_d_n0;
        locals.var_t1_dn2 = assign13250_e7576_d_n2;
        locals.var_t1_dn4 = assign13250_e7576_d_n4;
        locals.var_t1_dn5 = assign13250_e7576_d_n5;
        locals.var_t1_dn6 = assign13250_e7576_d_n6;
        locals.var_t1_dn7 = assign13250_e7576_d_n7;
        locals.var_t1_dn8 = assign13250_e7576_d_n8;
        locals.var_t1_dn9 = assign13250_e7576_d_n9;
        locals.var_t1_dn10 = assign13250_e7576_d_n10;
        locals.var_t1_dn13 = assign13250_e7576_d_n13;

        let (assign13260_e7584, assign13260_e7584_d_n0, assign13260_e7584_d_n2, assign13260_e7584_d_n4, assign13260_e7584_d_n5, assign13260_e7584_d_n6, assign13260_e7584_d_n7, assign13260_e7584_d_n8, assign13260_e7584_d_n9, assign13260_e7584_d_n10, assign13260_e7584_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13260_e7582: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign13260_e7582, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign13260_e7584;
        locals.var_depmphn0_dn0 = assign13260_e7584_d_n0;
        locals.var_depmphn0_dn2 = assign13260_e7584_d_n2;
        locals.var_depmphn0_dn4 = assign13260_e7584_d_n4;
        locals.var_depmphn0_dn5 = assign13260_e7584_d_n5;
        locals.var_depmphn0_dn6 = assign13260_e7584_d_n6;
        locals.var_depmphn0_dn7 = assign13260_e7584_d_n7;
        locals.var_depmphn0_dn8 = assign13260_e7584_d_n8;
        locals.var_depmphn0_dn9 = assign13260_e7584_d_n9;
        locals.var_depmphn0_dn10 = assign13260_e7584_d_n10;
        locals.var_depmphn0_dn13 = assign13260_e7584_d_n13;

        let (assign13270_e7606, assign13270_e7606_d_n0, assign13270_e7606_d_n2, assign13270_e7606_d_n4, assign13270_e7606_d_n5, assign13270_e7606_d_n6, assign13270_e7606_d_n7, assign13270_e7606_d_n8, assign13270_e7606_d_n9, assign13270_e7606_d_n10, assign13270_e7606_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13270_e7591: f64 = (0.4 * locals.var_tratio);
        let assign13270_e7592: f64 = (1.8 + assign13270_e7591);
        let assign13270_e7595: f64 = (0.1 * locals.var_tratio);
        let assign13270_e7597: f64 = (assign13270_e7595 * locals.var_tratio);
        let assign13270_e7598: f64 = (assign13270_e7592 + assign13270_e7597);
        let assign13270_e7602: f64 = (1.0 - locals.var_tratio);
        let assign13270_e7603: f64 = (p.p379 * assign13270_e7602);
        let assign13270_e7604: f64 = (assign13270_e7598 - assign13270_e7603);
        (assign13270_e7604, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13270_e7606;
        locals.var_t0_dn0 = assign13270_e7606_d_n0;
        locals.var_t0_dn2 = assign13270_e7606_d_n2;
        locals.var_t0_dn4 = assign13270_e7606_d_n4;
        locals.var_t0_dn5 = assign13270_e7606_d_n5;
        locals.var_t0_dn6 = assign13270_e7606_d_n6;
        locals.var_t0_dn7 = assign13270_e7606_d_n7;
        locals.var_t0_dn8 = assign13270_e7606_d_n8;
        locals.var_t0_dn9 = assign13270_e7606_d_n9;
        locals.var_t0_dn10 = assign13270_e7606_d_n10;
        locals.var_t0_dn13 = assign13270_e7606_d_n13;

        let (assign13280_e7614, assign13280_e7614_d_n0, assign13280_e7614_d_n2, assign13280_e7614_d_n4, assign13280_e7614_d_n5, assign13280_e7614_d_n6, assign13280_e7614_d_n7, assign13280_e7614_d_n8, assign13280_e7614_d_n9, assign13280_e7614_d_n10, assign13280_e7614_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13280_e7612: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13280_e7612, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign13280_e7614;
        locals.var_uc_depvmax_dn0 = assign13280_e7614_d_n0;
        locals.var_uc_depvmax_dn2 = assign13280_e7614_d_n2;
        locals.var_uc_depvmax_dn4 = assign13280_e7614_d_n4;
        locals.var_uc_depvmax_dn5 = assign13280_e7614_d_n5;
        locals.var_uc_depvmax_dn6 = assign13280_e7614_d_n6;
        locals.var_uc_depvmax_dn7 = assign13280_e7614_d_n7;
        locals.var_uc_depvmax_dn8 = assign13280_e7614_d_n8;
        locals.var_uc_depvmax_dn9 = assign13280_e7614_d_n9;
        locals.var_uc_depvmax_dn10 = assign13280_e7614_d_n10;
        locals.var_uc_depvmax_dn13 = assign13280_e7614_d_n13;

        let assign13300_e7622: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign13300_e7622;

        let (assign13310_e7630, assign13310_e7630_d_n0, assign13310_e7630_d_n2, assign13310_e7630_d_n4, assign13310_e7630_d_n5, assign13310_e7630_d_n6, assign13310_e7630_d_n7, assign13310_e7630_d_n8, assign13310_e7630_d_n9, assign13310_e7630_d_n10, assign13310_e7630_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) && (locals.var_guard292 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign13310_e7630;
        locals.var_uc_depvmax_dn0 = assign13310_e7630_d_n0;
        locals.var_uc_depvmax_dn2 = assign13310_e7630_d_n2;
        locals.var_uc_depvmax_dn4 = assign13310_e7630_d_n4;
        locals.var_uc_depvmax_dn5 = assign13310_e7630_d_n5;
        locals.var_uc_depvmax_dn6 = assign13310_e7630_d_n6;
        locals.var_uc_depvmax_dn7 = assign13310_e7630_d_n7;
        locals.var_uc_depvmax_dn8 = assign13310_e7630_d_n8;
        locals.var_uc_depvmax_dn9 = assign13310_e7630_d_n9;
        locals.var_uc_depvmax_dn10 = assign13310_e7630_d_n10;
        locals.var_uc_depvmax_dn13 = assign13310_e7630_d_n13;

        let (assign13320_e7640, assign13320_e7640_d_n0, assign13320_e7640_d_n2, assign13320_e7640_d_n4, assign13320_e7640_d_n5, assign13320_e7640_d_n6, assign13320_e7640_d_n7, assign13320_e7640_d_n8, assign13320_e7640_d_n9, assign13320_e7640_d_n10, assign13320_e7640_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13320_e7637: f64 = (locals.var_tratio).powf(p.p381);
        let assign13320_e7638: f64 = (locals.var_uc_depmue0 / assign13320_e7637);
        (assign13320_e7638, (((locals.var_uc_depmue0_dn0 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn2 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn4 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn5 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn6 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn7 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn8 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn9 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn10 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn13 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign13320_e7640;
        locals.var_uc_depmue0_dn0 = assign13320_e7640_d_n0;
        locals.var_uc_depmue0_dn2 = assign13320_e7640_d_n2;
        locals.var_uc_depmue0_dn4 = assign13320_e7640_d_n4;
        locals.var_uc_depmue0_dn5 = assign13320_e7640_d_n5;
        locals.var_uc_depmue0_dn6 = assign13320_e7640_d_n6;
        locals.var_uc_depmue0_dn7 = assign13320_e7640_d_n7;
        locals.var_uc_depmue0_dn8 = assign13320_e7640_d_n8;
        locals.var_uc_depmue0_dn9 = assign13320_e7640_d_n9;
        locals.var_uc_depmue0_dn10 = assign13320_e7640_d_n10;
        locals.var_uc_depmue0_dn13 = assign13320_e7640_d_n13;

        let (assign13330_e7650, assign13330_e7650_d_n0, assign13330_e7650_d_n2, assign13330_e7650_d_n4, assign13330_e7650_d_n5, assign13330_e7650_d_n6, assign13330_e7650_d_n7, assign13330_e7650_d_n8, assign13330_e7650_d_n9, assign13330_e7650_d_n10, assign13330_e7650_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13330_e7647: f64 = (locals.var_tratio).powf(p.p382);
        let assign13330_e7648: f64 = (locals.var_uc_depmue2 / assign13330_e7647);
        (assign13330_e7648, (((locals.var_uc_depmue2_dn0 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn2 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn4 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn5 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn6 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn7 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn8 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn9 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn10 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn13 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn13)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn13,)
    }
};
        locals.var_uc_depmue2 = assign13330_e7650;
        locals.var_uc_depmue2_dn0 = assign13330_e7650_d_n0;
        locals.var_uc_depmue2_dn2 = assign13330_e7650_d_n2;
        locals.var_uc_depmue2_dn4 = assign13330_e7650_d_n4;
        locals.var_uc_depmue2_dn5 = assign13330_e7650_d_n5;
        locals.var_uc_depmue2_dn6 = assign13330_e7650_d_n6;
        locals.var_uc_depmue2_dn7 = assign13330_e7650_d_n7;
        locals.var_uc_depmue2_dn8 = assign13330_e7650_d_n8;
        locals.var_uc_depmue2_dn9 = assign13330_e7650_d_n9;
        locals.var_uc_depmue2_dn10 = assign13330_e7650_d_n10;
        locals.var_uc_depmue2_dn13 = assign13330_e7650_d_n13;

        let assign13340_e7653: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard293 = assign13340_e7653;

        let (assign13350_e7671, assign13350_e7671_d_n0, assign13350_e7671_d_n2, assign13350_e7671_d_n4, assign13350_e7671_d_n5, assign13350_e7671_d_n6, assign13350_e7671_d_n7, assign13350_e7671_d_n8, assign13350_e7671_d_n9, assign13350_e7671_d_n10, assign13350_e7671_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13350_e7662: f64 = (2.0 * 1.034943e-10);
        let assign13350_e7664: f64 = (assign13350_e7662 * 1.6021918e-19);
        let assign13350_e7666: f64 = (assign13350_e7664 * locals.var_uc_ndepm);
        let assign13350_e7668: f64 = (assign13350_e7666 * locals.var_beta_inv);
        let assign13350_e7669: f64 = (assign13350_e7668).sqrt();
        (assign13350_e7669, ((((assign13350_e7664 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn0)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn2)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn4)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn5)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn6)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn7)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn8)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn9)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn10)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn13)) / (2.0 * assign13350_e7669)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign13350_e7671;
        locals.var_cnst0_dn0 = assign13350_e7671_d_n0;
        locals.var_cnst0_dn2 = assign13350_e7671_d_n2;
        locals.var_cnst0_dn4 = assign13350_e7671_d_n4;
        locals.var_cnst0_dn5 = assign13350_e7671_d_n5;
        locals.var_cnst0_dn6 = assign13350_e7671_d_n6;
        locals.var_cnst0_dn7 = assign13350_e7671_d_n7;
        locals.var_cnst0_dn8 = assign13350_e7671_d_n8;
        locals.var_cnst0_dn9 = assign13350_e7671_d_n9;
        locals.var_cnst0_dn10 = assign13350_e7671_d_n10;
        locals.var_cnst0_dn13 = assign13350_e7671_d_n13;

        let (assign13360_e7686, assign13360_e7686_d_n0, assign13360_e7686_d_n2, assign13360_e7686_d_n4, assign13360_e7686_d_n5, assign13360_e7686_d_n6, assign13360_e7686_d_n7, assign13360_e7686_d_n8, assign13360_e7686_d_n9, assign13360_e7686_d_n10, assign13360_e7686_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13360_e7680: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_uc_ndepm;
        let assign13360_e7682: f64 = (assign13360_e7680 * __rspice_inv_cse_2);
        let assign13360_e7684: f64 = (assign13360_e7682 * __rspice_inv_cse_2);
        (assign13360_e7684, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign13360_e7686;
        locals.var_cnst1_dn0 = assign13360_e7686_d_n0;
        locals.var_cnst1_dn2 = assign13360_e7686_d_n2;
        locals.var_cnst1_dn4 = assign13360_e7686_d_n4;
        locals.var_cnst1_dn5 = assign13360_e7686_d_n5;
        locals.var_cnst1_dn6 = assign13360_e7686_d_n6;
        locals.var_cnst1_dn7 = assign13360_e7686_d_n7;
        locals.var_cnst1_dn8 = assign13360_e7686_d_n8;
        locals.var_cnst1_dn9 = assign13360_e7686_d_n9;
        locals.var_cnst1_dn10 = assign13360_e7686_d_n10;
        locals.var_cnst1_dn13 = assign13360_e7686_d_n13;

        let (assign13370_e7702, assign13370_e7702_d_n0, assign13370_e7702_d_n2, assign13370_e7702_d_n4, assign13370_e7702_d_n5, assign13370_e7702_d_n6, assign13370_e7702_d_n7, assign13370_e7702_d_n8, assign13370_e7702_d_n9, assign13370_e7702_d_n10, assign13370_e7702_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13370_e7695: f64 = (2.0 * locals.var_beta_inv);
        let assign13370_e7698: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign13370_e7699: f64 = (assign13370_e7698).ln();
        let assign13370_e7700: f64 = (assign13370_e7695 * assign13370_e7699);
        (assign13370_e7700, (((2.0 * locals.var_beta_inv_dn0) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn2) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn4) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn5) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn6) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn7) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn8) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn9) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn10) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn13) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign13370_e7702;
        locals.var_pb2n_dn0 = assign13370_e7702_d_n0;
        locals.var_pb2n_dn2 = assign13370_e7702_d_n2;
        locals.var_pb2n_dn4 = assign13370_e7702_d_n4;
        locals.var_pb2n_dn5 = assign13370_e7702_d_n5;
        locals.var_pb2n_dn6 = assign13370_e7702_d_n6;
        locals.var_pb2n_dn7 = assign13370_e7702_d_n7;
        locals.var_pb2n_dn8 = assign13370_e7702_d_n8;
        locals.var_pb2n_dn9 = assign13370_e7702_d_n9;
        locals.var_pb2n_dn10 = assign13370_e7702_d_n10;
        locals.var_pb2n_dn13 = assign13370_e7702_d_n13;

        let (assign13380_e7720, assign13380_e7720_d_n0, assign13380_e7720_d_n2, assign13380_e7720_d_n4, assign13380_e7720_d_n5, assign13380_e7720_d_n6, assign13380_e7720_d_n7, assign13380_e7720_d_n8, assign13380_e7720_d_n9, assign13380_e7720_d_n10, assign13380_e7720_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13380_e7712: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_3: f64 = 1.0 / locals.var_nin;
        let assign13380_e7714: f64 = (assign13380_e7712 * __rspice_inv_cse_3);
        let assign13380_e7716: f64 = (assign13380_e7714 * __rspice_inv_cse_3);
        let assign13380_e7717: f64 = (assign13380_e7716).ln();
        let assign13380_e7718: f64 = (locals.var_beta_inv * assign13380_e7717);
        (assign13380_e7718, ((locals.var_beta_inv_dn0 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn2 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn4 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn5 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn6 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn7 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn8 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn9 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn10 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn13 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign13380_e7720;
        locals.var_vbipn_dn0 = assign13380_e7720_d_n0;
        locals.var_vbipn_dn2 = assign13380_e7720_d_n2;
        locals.var_vbipn_dn4 = assign13380_e7720_d_n4;
        locals.var_vbipn_dn5 = assign13380_e7720_d_n5;
        locals.var_vbipn_dn6 = assign13380_e7720_d_n6;
        locals.var_vbipn_dn7 = assign13380_e7720_d_n7;
        locals.var_vbipn_dn8 = assign13380_e7720_d_n8;
        locals.var_vbipn_dn9 = assign13380_e7720_d_n9;
        locals.var_vbipn_dn10 = assign13380_e7720_d_n10;
        locals.var_vbipn_dn13 = assign13380_e7720_d_n13;

        let (assign13390_e7732, assign13390_e7732_d_n0, assign13390_e7732_d_n2, assign13390_e7732_d_n4, assign13390_e7732_d_n5, assign13390_e7732_d_n6, assign13390_e7732_d_n7, assign13390_e7732_d_n8, assign13390_e7732_d_n9, assign13390_e7732_d_n10, assign13390_e7732_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13390_e7729: f64 = (locals.var_log_tratio * p.p380);
        let assign13390_e7730: f64 = (assign13390_e7729).exp();
        (assign13390_e7730, (assign13390_e7730 * (locals.var_log_tratio_dn0 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn2 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn4 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn5 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn6 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn7 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn8 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn9 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn10 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13390_e7732;
        locals.var_t1_dn0 = assign13390_e7732_d_n0;
        locals.var_t1_dn2 = assign13390_e7732_d_n2;
        locals.var_t1_dn4 = assign13390_e7732_d_n4;
        locals.var_t1_dn5 = assign13390_e7732_d_n5;
        locals.var_t1_dn6 = assign13390_e7732_d_n6;
        locals.var_t1_dn7 = assign13390_e7732_d_n7;
        locals.var_t1_dn8 = assign13390_e7732_d_n8;
        locals.var_t1_dn9 = assign13390_e7732_d_n9;
        locals.var_t1_dn10 = assign13390_e7732_d_n10;
        locals.var_t1_dn13 = assign13390_e7732_d_n13;

    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13400_e7743, assign13400_e7743_d_n0, assign13400_e7743_d_n2, assign13400_e7743_d_n4, assign13400_e7743_d_n5, assign13400_e7743_d_n6, assign13400_e7743_d_n7, assign13400_e7743_d_n8, assign13400_e7743_d_n9, assign13400_e7743_d_n10, assign13400_e7743_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13400_e7741: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign13400_e7741, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign13400_e7743;
        locals.var_depmphn0_dn0 = assign13400_e7743_d_n0;
        locals.var_depmphn0_dn2 = assign13400_e7743_d_n2;
        locals.var_depmphn0_dn4 = assign13400_e7743_d_n4;
        locals.var_depmphn0_dn5 = assign13400_e7743_d_n5;
        locals.var_depmphn0_dn6 = assign13400_e7743_d_n6;
        locals.var_depmphn0_dn7 = assign13400_e7743_d_n7;
        locals.var_depmphn0_dn8 = assign13400_e7743_d_n8;
        locals.var_depmphn0_dn9 = assign13400_e7743_d_n9;
        locals.var_depmphn0_dn10 = assign13400_e7743_d_n10;
        locals.var_depmphn0_dn13 = assign13400_e7743_d_n13;

        let (assign13410_e7768, assign13410_e7768_d_n0, assign13410_e7768_d_n2, assign13410_e7768_d_n4, assign13410_e7768_d_n5, assign13410_e7768_d_n6, assign13410_e7768_d_n7, assign13410_e7768_d_n8, assign13410_e7768_d_n9, assign13410_e7768_d_n10, assign13410_e7768_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13410_e7753: f64 = (0.4 * locals.var_tratio);
        let assign13410_e7754: f64 = (1.8 + assign13410_e7753);
        let assign13410_e7757: f64 = (0.1 * locals.var_tratio);
        let assign13410_e7759: f64 = (assign13410_e7757 * locals.var_tratio);
        let assign13410_e7760: f64 = (assign13410_e7754 + assign13410_e7759);
        let assign13410_e7764: f64 = (1.0 - locals.var_tratio);
        let assign13410_e7765: f64 = (p.p379 * assign13410_e7764);
        let assign13410_e7766: f64 = (assign13410_e7760 - assign13410_e7765);
        (assign13410_e7766, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13410_e7768;
        locals.var_t0_dn0 = assign13410_e7768_d_n0;
        locals.var_t0_dn2 = assign13410_e7768_d_n2;
        locals.var_t0_dn4 = assign13410_e7768_d_n4;
        locals.var_t0_dn5 = assign13410_e7768_d_n5;
        locals.var_t0_dn6 = assign13410_e7768_d_n6;
        locals.var_t0_dn7 = assign13410_e7768_d_n7;
        locals.var_t0_dn8 = assign13410_e7768_d_n8;
        locals.var_t0_dn9 = assign13410_e7768_d_n9;
        locals.var_t0_dn10 = assign13410_e7768_d_n10;
        locals.var_t0_dn13 = assign13410_e7768_d_n13;

        let (assign13420_e7779, assign13420_e7779_d_n0, assign13420_e7779_d_n2, assign13420_e7779_d_n4, assign13420_e7779_d_n5, assign13420_e7779_d_n6, assign13420_e7779_d_n7, assign13420_e7779_d_n8, assign13420_e7779_d_n9, assign13420_e7779_d_n10, assign13420_e7779_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13420_e7777: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13420_e7777, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign13420_e7779;
        locals.var_uc_depvmax_dn0 = assign13420_e7779_d_n0;
        locals.var_uc_depvmax_dn2 = assign13420_e7779_d_n2;
        locals.var_uc_depvmax_dn4 = assign13420_e7779_d_n4;
        locals.var_uc_depvmax_dn5 = assign13420_e7779_d_n5;
        locals.var_uc_depvmax_dn6 = assign13420_e7779_d_n6;
        locals.var_uc_depvmax_dn7 = assign13420_e7779_d_n7;
        locals.var_uc_depvmax_dn8 = assign13420_e7779_d_n8;
        locals.var_uc_depvmax_dn9 = assign13420_e7779_d_n9;
        locals.var_uc_depvmax_dn10 = assign13420_e7779_d_n10;
        locals.var_uc_depvmax_dn13 = assign13420_e7779_d_n13;

        let assign13440_e7787: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign13440_e7787;

        let (assign13450_e7798, assign13450_e7798_d_n0, assign13450_e7798_d_n2, assign13450_e7798_d_n4, assign13450_e7798_d_n5, assign13450_e7798_d_n6, assign13450_e7798_d_n7, assign13450_e7798_d_n8, assign13450_e7798_d_n9, assign13450_e7798_d_n10, assign13450_e7798_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) && (locals.var_guard295 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign13450_e7798;
        locals.var_uc_depvmax_dn0 = assign13450_e7798_d_n0;
        locals.var_uc_depvmax_dn2 = assign13450_e7798_d_n2;
        locals.var_uc_depvmax_dn4 = assign13450_e7798_d_n4;
        locals.var_uc_depvmax_dn5 = assign13450_e7798_d_n5;
        locals.var_uc_depvmax_dn6 = assign13450_e7798_d_n6;
        locals.var_uc_depvmax_dn7 = assign13450_e7798_d_n7;
        locals.var_uc_depvmax_dn8 = assign13450_e7798_d_n8;
        locals.var_uc_depvmax_dn9 = assign13450_e7798_d_n9;
        locals.var_uc_depvmax_dn10 = assign13450_e7798_d_n10;
        locals.var_uc_depvmax_dn13 = assign13450_e7798_d_n13;

        let (assign13460_e7811, assign13460_e7811_d_n0, assign13460_e7811_d_n2, assign13460_e7811_d_n4, assign13460_e7811_d_n5, assign13460_e7811_d_n6, assign13460_e7811_d_n7, assign13460_e7811_d_n8, assign13460_e7811_d_n9, assign13460_e7811_d_n10, assign13460_e7811_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13460_e7808: f64 = (locals.var_tratio).powf(p.p381);
        let assign13460_e7809: f64 = (locals.var_uc_depmue0 / assign13460_e7808);
        (assign13460_e7809, (((locals.var_uc_depmue0_dn0 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn2 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn4 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn5 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn6 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn7 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn8 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn9 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn10 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn13 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign13460_e7811;
        locals.var_uc_depmue0_dn0 = assign13460_e7811_d_n0;
        locals.var_uc_depmue0_dn2 = assign13460_e7811_d_n2;
        locals.var_uc_depmue0_dn4 = assign13460_e7811_d_n4;
        locals.var_uc_depmue0_dn5 = assign13460_e7811_d_n5;
        locals.var_uc_depmue0_dn6 = assign13460_e7811_d_n6;
        locals.var_uc_depmue0_dn7 = assign13460_e7811_d_n7;
        locals.var_uc_depmue0_dn8 = assign13460_e7811_d_n8;
        locals.var_uc_depmue0_dn9 = assign13460_e7811_d_n9;
        locals.var_uc_depmue0_dn10 = assign13460_e7811_d_n10;
        locals.var_uc_depmue0_dn13 = assign13460_e7811_d_n13;

        let (assign13470_e7826, assign13470_e7826_d_n0, assign13470_e7826_d_n2, assign13470_e7826_d_n4, assign13470_e7826_d_n5, assign13470_e7826_d_n6, assign13470_e7826_d_n7, assign13470_e7826_d_n8, assign13470_e7826_d_n9, assign13470_e7826_d_n10, assign13470_e7826_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13470_e7822: f64 = (locals.var_tratio - 1.0);
        let assign13470_e7823: f64 = (p.p365 * assign13470_e7822);
        let assign13470_e7824: f64 = (p.p364 + assign13470_e7823);
        (assign13470_e7824, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn13),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn13,)
    }
};
        locals.var_uc_depwlp = assign13470_e7826;
        locals.var_uc_depwlp_dn0 = assign13470_e7826_d_n0;
        locals.var_uc_depwlp_dn2 = assign13470_e7826_d_n2;
        locals.var_uc_depwlp_dn4 = assign13470_e7826_d_n4;
        locals.var_uc_depwlp_dn5 = assign13470_e7826_d_n5;
        locals.var_uc_depwlp_dn6 = assign13470_e7826_d_n6;
        locals.var_uc_depwlp_dn7 = assign13470_e7826_d_n7;
        locals.var_uc_depwlp_dn8 = assign13470_e7826_d_n8;
        locals.var_uc_depwlp_dn9 = assign13470_e7826_d_n9;
        locals.var_uc_depwlp_dn10 = assign13470_e7826_d_n10;
        locals.var_uc_depwlp_dn13 = assign13470_e7826_d_n13;

        let (assign13480_e7836, assign13480_e7836_d_n0, assign13480_e7836_d_n2, assign13480_e7836_d_n4, assign13480_e7836_d_n5, assign13480_e7836_d_n6, assign13480_e7836_d_n7, assign13480_e7836_d_n8, assign13480_e7836_d_n9, assign13480_e7836_d_n10, assign13480_e7836_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign13480_e7836;
        locals.var_pb2n_dn0 = assign13480_e7836_d_n0;
        locals.var_pb2n_dn2 = assign13480_e7836_d_n2;
        locals.var_pb2n_dn4 = assign13480_e7836_d_n4;
        locals.var_pb2n_dn5 = assign13480_e7836_d_n5;
        locals.var_pb2n_dn6 = assign13480_e7836_d_n6;
        locals.var_pb2n_dn7 = assign13480_e7836_d_n7;
        locals.var_pb2n_dn8 = assign13480_e7836_d_n8;
        locals.var_pb2n_dn9 = assign13480_e7836_d_n9;
        locals.var_pb2n_dn10 = assign13480_e7836_d_n10;
        locals.var_pb2n_dn13 = assign13480_e7836_d_n13;

        let (assign13490_e7855, assign13490_e7855_d_n0, assign13490_e7855_d_n2, assign13490_e7855_d_n4, assign13490_e7855_d_n5, assign13490_e7855_d_n6, assign13490_e7855_d_n7, assign13490_e7855_d_n8, assign13490_e7855_d_n9, assign13490_e7855_d_n10, assign13490_e7855_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 == 0.0)) {
        let assign13490_e7847: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign13490_e7849: f64 = (assign13490_e7847 * locals.var_nsub);
        let assign13490_e7851: f64 = (assign13490_e7849 / locals.var_nin);
        let assign13490_e7852: f64 = (assign13490_e7851).ln();
        let assign13490_e7853: f64 = (locals.var_beta_inv * assign13490_e7852);
        (assign13490_e7853, ((locals.var_beta_inv_dn0 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn0)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn2 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn2)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn4 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn4)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn5 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn5)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn6 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn6)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn7 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn7)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn8 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn8)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn9 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn9)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn10 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn10)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn13 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn13)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign13490_e7855;
        locals.var_vbipn_dn0 = assign13490_e7855_d_n0;
        locals.var_vbipn_dn2 = assign13490_e7855_d_n2;
        locals.var_vbipn_dn4 = assign13490_e7855_d_n4;
        locals.var_vbipn_dn5 = assign13490_e7855_d_n5;
        locals.var_vbipn_dn6 = assign13490_e7855_d_n6;
        locals.var_vbipn_dn7 = assign13490_e7855_d_n7;
        locals.var_vbipn_dn8 = assign13490_e7855_d_n8;
        locals.var_vbipn_dn9 = assign13490_e7855_d_n9;
        locals.var_vbipn_dn10 = assign13490_e7855_d_n10;
        locals.var_vbipn_dn13 = assign13490_e7855_d_n13;

        let (assign13500_e7865, assign13500_e7865_d_n0, assign13500_e7865_d_n2, assign13500_e7865_d_n4, assign13500_e7865_d_n5, assign13500_e7865_d_n6, assign13500_e7865_d_n7, assign13500_e7865_d_n8, assign13500_e7865_d_n9, assign13500_e7865_d_n10, assign13500_e7865_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign13500_e7865;
        locals.var_depmphn0_dn0 = assign13500_e7865_d_n0;
        locals.var_depmphn0_dn2 = assign13500_e7865_d_n2;
        locals.var_depmphn0_dn4 = assign13500_e7865_d_n4;
        locals.var_depmphn0_dn5 = assign13500_e7865_d_n5;
        locals.var_depmphn0_dn6 = assign13500_e7865_d_n6;
        locals.var_depmphn0_dn7 = assign13500_e7865_d_n7;
        locals.var_depmphn0_dn8 = assign13500_e7865_d_n8;
        locals.var_depmphn0_dn9 = assign13500_e7865_d_n9;
        locals.var_depmphn0_dn10 = assign13500_e7865_d_n10;
        locals.var_depmphn0_dn13 = assign13500_e7865_d_n13;

        let (assign13510_e7871, assign13510_e7871_d_n0, assign13510_e7871_d_n2, assign13510_e7871_d_n4, assign13510_e7871_d_n5, assign13510_e7871_d_n6, assign13510_e7871_d_n7, assign13510_e7871_d_n8, assign13510_e7871_d_n9, assign13510_e7871_d_n10, assign13510_e7871_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13510_e7869: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign13510_e7869, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn13 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn13,)
    }
};
        locals.var_ptovr = assign13510_e7871;
        locals.var_ptovr_dn0 = assign13510_e7871_d_n0;
        locals.var_ptovr_dn2 = assign13510_e7871_d_n2;
        locals.var_ptovr_dn4 = assign13510_e7871_d_n4;
        locals.var_ptovr_dn5 = assign13510_e7871_d_n5;
        locals.var_ptovr_dn6 = assign13510_e7871_d_n6;
        locals.var_ptovr_dn7 = assign13510_e7871_d_n7;
        locals.var_ptovr_dn8 = assign13510_e7871_d_n8;
        locals.var_ptovr_dn9 = assign13510_e7871_d_n9;
        locals.var_ptovr_dn10 = assign13510_e7871_d_n10;
        locals.var_ptovr_dn13 = assign13510_e7871_d_n13;

        let (assign13520_e7877, assign13520_e7877_d_n0, assign13520_e7877_d_n2, assign13520_e7877_d_n4, assign13520_e7877_d_n5, assign13520_e7877_d_n6, assign13520_e7877_d_n7, assign13520_e7877_d_n8, assign13520_e7877_d_n9, assign13520_e7877_d_n10, assign13520_e7877_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13520_e7875: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13520_e7875, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13520_e7877;
        locals.var_t1_dn0 = assign13520_e7877_d_n0;
        locals.var_t1_dn2 = assign13520_e7877_d_n2;
        locals.var_t1_dn4 = assign13520_e7877_d_n4;
        locals.var_t1_dn5 = assign13520_e7877_d_n5;
        locals.var_t1_dn6 = assign13520_e7877_d_n6;
        locals.var_t1_dn7 = assign13520_e7877_d_n7;
        locals.var_t1_dn8 = assign13520_e7877_d_n8;
        locals.var_t1_dn9 = assign13520_e7877_d_n9;
        locals.var_t1_dn10 = assign13520_e7877_d_n10;
        locals.var_t1_dn13 = assign13520_e7877_d_n13;

        let (assign13530_e7897, assign13530_e7897_d_n0, assign13530_e7897_d_n2, assign13530_e7897_d_n4, assign13530_e7897_d_n5, assign13530_e7897_d_n6, assign13530_e7897_d_n7, assign13530_e7897_d_n8, assign13530_e7897_d_n9, assign13530_e7897_d_n10, assign13530_e7897_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13530_e7882: f64 = (0.4 * locals.var_t1);
        let assign13530_e7883: f64 = (1.8 + assign13530_e7882);
        let assign13530_e7886: f64 = (0.1 * locals.var_t1);
        let assign13530_e7888: f64 = (assign13530_e7886 * locals.var_t1);
        let assign13530_e7889: f64 = (assign13530_e7883 + assign13530_e7888);
        let assign13530_e7893: f64 = (1.0 - locals.var_t1);
        let assign13530_e7894: f64 = (locals.var_uc_vtmp * assign13530_e7893);
        let assign13530_e7895: f64 = (assign13530_e7889 - assign13530_e7894);
        (assign13530_e7895, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn13) + (((0.1 * locals.var_t1_dn13) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn13))) - (locals.var_uc_vtmp * (-locals.var_t1_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13530_e7897;
        locals.var_t0_dn0 = assign13530_e7897_d_n0;
        locals.var_t0_dn2 = assign13530_e7897_d_n2;
        locals.var_t0_dn4 = assign13530_e7897_d_n4;
        locals.var_t0_dn5 = assign13530_e7897_d_n5;
        locals.var_t0_dn6 = assign13530_e7897_d_n6;
        locals.var_t0_dn7 = assign13530_e7897_d_n7;
        locals.var_t0_dn8 = assign13530_e7897_d_n8;
        locals.var_t0_dn9 = assign13530_e7897_d_n9;
        locals.var_t0_dn10 = assign13530_e7897_d_n10;
        locals.var_t0_dn13 = assign13530_e7897_d_n13;

        let assign13540_e7900: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign13540_e7900;

        let (assign13550_e7920, assign13550_e7920_d_n0, assign13550_e7920_d_n2, assign13550_e7920_d_n4, assign13550_e7920_d_n5, assign13550_e7920_d_n6, assign13550_e7920_d_n7, assign13550_e7920_d_n8, assign13550_e7920_d_n9, assign13550_e7920_d_n10, assign13550_e7920_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard296 != 0.0)) {
        let assign13550_e7906: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13550_e7908: f64 = (assign13550_e7906 / locals.var_t0);
        let assign13550_e7912: f64 = (p.p90 * locals.var_tdiff0);
        let assign13550_e7913: f64 = (1.0 + assign13550_e7912);
        let assign13550_e7916: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign13550_e7917: f64 = (assign13550_e7913 + assign13550_e7916);
        let assign13550_e7918: f64 = (assign13550_e7908 * assign13550_e7917);
        (assign13550_e7918, (((-((assign13550_e7906 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign13550_e7906 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign13550_e7906 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign13550_e7906 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign13550_e7906 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign13550_e7906 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign13550_e7906 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign13550_e7906 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign13550_e7906 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign13550_e7906 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn13) + (p.p91 * locals.var_tdiff0_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign13550_e7920;
        locals.var_vmaxeff_dn0 = assign13550_e7920_d_n0;
        locals.var_vmaxeff_dn2 = assign13550_e7920_d_n2;
        locals.var_vmaxeff_dn4 = assign13550_e7920_d_n4;
        locals.var_vmaxeff_dn5 = assign13550_e7920_d_n5;
        locals.var_vmaxeff_dn6 = assign13550_e7920_d_n6;
        locals.var_vmaxeff_dn7 = assign13550_e7920_d_n7;
        locals.var_vmaxeff_dn8 = assign13550_e7920_d_n8;
        locals.var_vmaxeff_dn9 = assign13550_e7920_d_n9;
        locals.var_vmaxeff_dn10 = assign13550_e7920_d_n10;
        locals.var_vmaxeff_dn13 = assign13550_e7920_d_n13;

        let (assign13560_e7941, assign13560_e7941_d_n0, assign13560_e7941_d_n2, assign13560_e7941_d_n4, assign13560_e7941_d_n5, assign13560_e7941_d_n6, assign13560_e7941_d_n7, assign13560_e7941_d_n8, assign13560_e7941_d_n9, assign13560_e7941_d_n10, assign13560_e7941_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard296 == 0.0)) {
        let assign13560_e7927: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13560_e7929: f64 = (assign13560_e7927 / locals.var_t0);
        let assign13560_e7933: f64 = (p.p90 * locals.var_tdiff);
        let assign13560_e7934: f64 = (1.0 + assign13560_e7933);
        let assign13560_e7937: f64 = (p.p91 * locals.var_tdiff_2);
        let assign13560_e7938: f64 = (assign13560_e7934 + assign13560_e7937);
        let assign13560_e7939: f64 = (assign13560_e7929 * assign13560_e7938);
        (assign13560_e7939, (((-((assign13560_e7927 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign13560_e7927 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign13560_e7927 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign13560_e7927 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign13560_e7927 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign13560_e7927 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign13560_e7927 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign13560_e7927 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign13560_e7927 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign13560_e7927 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn13) + (p.p91 * locals.var_tdiff_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign13560_e7941;
        locals.var_vmaxeff_dn0 = assign13560_e7941_d_n0;
        locals.var_vmaxeff_dn2 = assign13560_e7941_d_n2;
        locals.var_vmaxeff_dn4 = assign13560_e7941_d_n4;
        locals.var_vmaxeff_dn5 = assign13560_e7941_d_n5;
        locals.var_vmaxeff_dn6 = assign13560_e7941_d_n6;
        locals.var_vmaxeff_dn7 = assign13560_e7941_d_n7;
        locals.var_vmaxeff_dn8 = assign13560_e7941_d_n8;
        locals.var_vmaxeff_dn9 = assign13560_e7941_d_n9;
        locals.var_vmaxeff_dn10 = assign13560_e7941_d_n10;
        locals.var_vmaxeff_dn13 = assign13560_e7941_d_n13;

        let assign13580_e7949: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign13580_e7949;

        let (assign13590_e7965, assign13590_e7965_d_n0, assign13590_e7965_d_n2, assign13590_e7965_d_n4, assign13590_e7965_d_n5, assign13590_e7965_d_n6, assign13590_e7965_d_n7, assign13590_e7965_d_n8, assign13590_e7965_d_n9, assign13590_e7965_d_n10, assign13590_e7965_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13590_e7957: f64 = (p.p324 * locals.var_tdiff0);
        let assign13590_e7958: f64 = (1.0 + assign13590_e7957);
        let assign13590_e7961: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign13590_e7962: f64 = (assign13590_e7958 + assign13590_e7961);
        let assign13590_e7963: f64 = (locals.var_ninvd0 * assign13590_e7962);
        (assign13590_e7963, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn13) + (p.p325 * locals.var_tdiff0_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign13590_e7965;
        locals.var_ninvde_dn0 = assign13590_e7965_d_n0;
        locals.var_ninvde_dn2 = assign13590_e7965_d_n2;
        locals.var_ninvde_dn4 = assign13590_e7965_d_n4;
        locals.var_ninvde_dn5 = assign13590_e7965_d_n5;
        locals.var_ninvde_dn6 = assign13590_e7965_d_n6;
        locals.var_ninvde_dn7 = assign13590_e7965_d_n7;
        locals.var_ninvde_dn8 = assign13590_e7965_d_n8;
        locals.var_ninvde_dn9 = assign13590_e7965_d_n9;
        locals.var_ninvde_dn10 = assign13590_e7965_d_n10;
        locals.var_ninvde_dn13 = assign13590_e7965_d_n13;

        let (assign13600_e7979, assign13600_e7979_d_n0, assign13600_e7979_d_n2, assign13600_e7979_d_n4, assign13600_e7979_d_n5, assign13600_e7979_d_n6, assign13600_e7979_d_n7, assign13600_e7979_d_n8, assign13600_e7979_d_n9, assign13600_e7979_d_n10, assign13600_e7979_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13600_e7972: f64 = (p.p390 * locals.var_tdiff0);
        let assign13600_e7973: f64 = (1.0 + assign13600_e7972);
        let assign13600_e7976: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign13600_e7977: f64 = (assign13600_e7973 + assign13600_e7976);
        (assign13600_e7977, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn13) + (p.p391 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13600_e7979;
        locals.var_t1_dn0 = assign13600_e7979_d_n0;
        locals.var_t1_dn2 = assign13600_e7979_d_n2;
        locals.var_t1_dn4 = assign13600_e7979_d_n4;
        locals.var_t1_dn5 = assign13600_e7979_d_n5;
        locals.var_t1_dn6 = assign13600_e7979_d_n6;
        locals.var_t1_dn7 = assign13600_e7979_d_n7;
        locals.var_t1_dn8 = assign13600_e7979_d_n8;
        locals.var_t1_dn9 = assign13600_e7979_d_n9;
        locals.var_t1_dn10 = assign13600_e7979_d_n10;
        locals.var_t1_dn13 = assign13600_e7979_d_n13;

        let (assign13610_e7987, assign13610_e7987_d_n0, assign13610_e7987_d_n2, assign13610_e7987_d_n4, assign13610_e7987_d_n5, assign13610_e7987_d_n6, assign13610_e7987_d_n7, assign13610_e7987_d_n8, assign13610_e7987_d_n9, assign13610_e7987_d_n10, assign13610_e7987_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13610_e7985: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13610_e7985, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign13610_e7987;
        locals.var_ninvdecres_dn0 = assign13610_e7987_d_n0;
        locals.var_ninvdecres_dn2 = assign13610_e7987_d_n2;
        locals.var_ninvdecres_dn4 = assign13610_e7987_d_n4;
        locals.var_ninvdecres_dn5 = assign13610_e7987_d_n5;
        locals.var_ninvdecres_dn6 = assign13610_e7987_d_n6;
        locals.var_ninvdecres_dn7 = assign13610_e7987_d_n7;
        locals.var_ninvdecres_dn8 = assign13610_e7987_d_n8;
        locals.var_ninvdecres_dn9 = assign13610_e7987_d_n9;
        locals.var_ninvdecres_dn10 = assign13610_e7987_d_n10;
        locals.var_ninvdecres_dn13 = assign13610_e7987_d_n13;

        let (assign13620_e7995, assign13620_e7995_d_n0, assign13620_e7995_d_n2, assign13620_e7995_d_n4, assign13620_e7995_d_n5, assign13620_e7995_d_n6, assign13620_e7995_d_n7, assign13620_e7995_d_n8, assign13620_e7995_d_n9, assign13620_e7995_d_n10, assign13620_e7995_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13620_e7993: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13620_e7993, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign13620_e7995;
        locals.var_ninvdehres_dn0 = assign13620_e7995_d_n0;
        locals.var_ninvdehres_dn2 = assign13620_e7995_d_n2;
        locals.var_ninvdehres_dn4 = assign13620_e7995_d_n4;
        locals.var_ninvdehres_dn5 = assign13620_e7995_d_n5;
        locals.var_ninvdehres_dn6 = assign13620_e7995_d_n6;
        locals.var_ninvdehres_dn7 = assign13620_e7995_d_n7;
        locals.var_ninvdehres_dn8 = assign13620_e7995_d_n8;
        locals.var_ninvdehres_dn9 = assign13620_e7995_d_n9;
        locals.var_ninvdehres_dn10 = assign13620_e7995_d_n10;
        locals.var_ninvdehres_dn13 = assign13620_e7995_d_n13;

        let (assign13630_e8012, assign13630_e8012_d_n0, assign13630_e8012_d_n2, assign13630_e8012_d_n4, assign13630_e8012_d_n5, assign13630_e8012_d_n6, assign13630_e8012_d_n7, assign13630_e8012_d_n8, assign13630_e8012_d_n9, assign13630_e8012_d_n10, assign13630_e8012_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13630_e8004: f64 = (p.p324 * locals.var_tdiff);
        let assign13630_e8005: f64 = (1.0 + assign13630_e8004);
        let assign13630_e8008: f64 = (p.p325 * locals.var_tdiff_2);
        let assign13630_e8009: f64 = (assign13630_e8005 + assign13630_e8008);
        let assign13630_e8010: f64 = (locals.var_ninvd0 * assign13630_e8009);
        (assign13630_e8010, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn13) + (p.p325 * locals.var_tdiff_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign13630_e8012;
        locals.var_ninvde_dn0 = assign13630_e8012_d_n0;
        locals.var_ninvde_dn2 = assign13630_e8012_d_n2;
        locals.var_ninvde_dn4 = assign13630_e8012_d_n4;
        locals.var_ninvde_dn5 = assign13630_e8012_d_n5;
        locals.var_ninvde_dn6 = assign13630_e8012_d_n6;
        locals.var_ninvde_dn7 = assign13630_e8012_d_n7;
        locals.var_ninvde_dn8 = assign13630_e8012_d_n8;
        locals.var_ninvde_dn9 = assign13630_e8012_d_n9;
        locals.var_ninvde_dn10 = assign13630_e8012_d_n10;
        locals.var_ninvde_dn13 = assign13630_e8012_d_n13;

        let (assign13640_e8027, assign13640_e8027_d_n0, assign13640_e8027_d_n2, assign13640_e8027_d_n4, assign13640_e8027_d_n5, assign13640_e8027_d_n6, assign13640_e8027_d_n7, assign13640_e8027_d_n8, assign13640_e8027_d_n9, assign13640_e8027_d_n10, assign13640_e8027_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13640_e8020: f64 = (p.p390 * locals.var_tdiff);
        let assign13640_e8021: f64 = (1.0 + assign13640_e8020);
        let assign13640_e8024: f64 = (p.p391 * locals.var_tdiff_2);
        let assign13640_e8025: f64 = (assign13640_e8021 + assign13640_e8024);
        (assign13640_e8025, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn13) + (p.p391 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13640_e8027;
        locals.var_t1_dn0 = assign13640_e8027_d_n0;
        locals.var_t1_dn2 = assign13640_e8027_d_n2;
        locals.var_t1_dn4 = assign13640_e8027_d_n4;
        locals.var_t1_dn5 = assign13640_e8027_d_n5;
        locals.var_t1_dn6 = assign13640_e8027_d_n6;
        locals.var_t1_dn7 = assign13640_e8027_d_n7;
        locals.var_t1_dn8 = assign13640_e8027_d_n8;
        locals.var_t1_dn9 = assign13640_e8027_d_n9;
        locals.var_t1_dn10 = assign13640_e8027_d_n10;
        locals.var_t1_dn13 = assign13640_e8027_d_n13;

        let (assign13650_e8036, assign13650_e8036_d_n0, assign13650_e8036_d_n2, assign13650_e8036_d_n4, assign13650_e8036_d_n5, assign13650_e8036_d_n6, assign13650_e8036_d_n7, assign13650_e8036_d_n8, assign13650_e8036_d_n9, assign13650_e8036_d_n10, assign13650_e8036_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13650_e8034: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13650_e8034, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign13650_e8036;
        locals.var_ninvdecres_dn0 = assign13650_e8036_d_n0;
        locals.var_ninvdecres_dn2 = assign13650_e8036_d_n2;
        locals.var_ninvdecres_dn4 = assign13650_e8036_d_n4;
        locals.var_ninvdecres_dn5 = assign13650_e8036_d_n5;
        locals.var_ninvdecres_dn6 = assign13650_e8036_d_n6;
        locals.var_ninvdecres_dn7 = assign13650_e8036_d_n7;
        locals.var_ninvdecres_dn8 = assign13650_e8036_d_n8;
        locals.var_ninvdecres_dn9 = assign13650_e8036_d_n9;
        locals.var_ninvdecres_dn10 = assign13650_e8036_d_n10;
        locals.var_ninvdecres_dn13 = assign13650_e8036_d_n13;

        let (assign13660_e8045, assign13660_e8045_d_n0, assign13660_e8045_d_n2, assign13660_e8045_d_n4, assign13660_e8045_d_n5, assign13660_e8045_d_n6, assign13660_e8045_d_n7, assign13660_e8045_d_n8, assign13660_e8045_d_n9, assign13660_e8045_d_n10, assign13660_e8045_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13660_e8043: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13660_e8043, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign13660_e8045;
        locals.var_ninvdehres_dn0 = assign13660_e8045_d_n0;
        locals.var_ninvdehres_dn2 = assign13660_e8045_d_n2;
        locals.var_ninvdehres_dn4 = assign13660_e8045_d_n4;
        locals.var_ninvdehres_dn5 = assign13660_e8045_d_n5;
        locals.var_ninvdehres_dn6 = assign13660_e8045_d_n6;
        locals.var_ninvdehres_dn7 = assign13660_e8045_d_n7;
        locals.var_ninvdehres_dn8 = assign13660_e8045_d_n8;
        locals.var_ninvdehres_dn9 = assign13660_e8045_d_n9;
        locals.var_ninvdehres_dn10 = assign13660_e8045_d_n10;
        locals.var_ninvdehres_dn13 = assign13660_e8045_d_n13;

        let assign13680_e8053: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign13680_e8053;

    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13690_e8059, assign13690_e8059_d_n0, assign13690_e8059_d_n2, assign13690_e8059_d_n4, assign13690_e8059_d_n5, assign13690_e8059_d_n6, assign13690_e8059_d_n7, assign13690_e8059_d_n8, assign13690_e8059_d_n9, assign13690_e8059_d_n10, assign13690_e8059_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard300 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign13690_e8059;
        locals.var_ninvde_dn0 = assign13690_e8059_d_n0;
        locals.var_ninvde_dn2 = assign13690_e8059_d_n2;
        locals.var_ninvde_dn4 = assign13690_e8059_d_n4;
        locals.var_ninvde_dn5 = assign13690_e8059_d_n5;
        locals.var_ninvde_dn6 = assign13690_e8059_d_n6;
        locals.var_ninvde_dn7 = assign13690_e8059_d_n7;
        locals.var_ninvde_dn8 = assign13690_e8059_d_n8;
        locals.var_ninvde_dn9 = assign13690_e8059_d_n9;
        locals.var_ninvde_dn10 = assign13690_e8059_d_n10;
        locals.var_ninvde_dn13 = assign13690_e8059_d_n13;

        let assign13710_e8067: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign13710_e8067;

        let (assign13720_e8073, assign13720_e8073_d_n0, assign13720_e8073_d_n2, assign13720_e8073_d_n4, assign13720_e8073_d_n5, assign13720_e8073_d_n6, assign13720_e8073_d_n7, assign13720_e8073_d_n8, assign13720_e8073_d_n9, assign13720_e8073_d_n10, assign13720_e8073_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard302 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign13720_e8073;
        locals.var_ninvdecres_dn0 = assign13720_e8073_d_n0;
        locals.var_ninvdecres_dn2 = assign13720_e8073_d_n2;
        locals.var_ninvdecres_dn4 = assign13720_e8073_d_n4;
        locals.var_ninvdecres_dn5 = assign13720_e8073_d_n5;
        locals.var_ninvdecres_dn6 = assign13720_e8073_d_n6;
        locals.var_ninvdecres_dn7 = assign13720_e8073_d_n7;
        locals.var_ninvdecres_dn8 = assign13720_e8073_d_n8;
        locals.var_ninvdecres_dn9 = assign13720_e8073_d_n9;
        locals.var_ninvdecres_dn10 = assign13720_e8073_d_n10;
        locals.var_ninvdecres_dn13 = assign13720_e8073_d_n13;

        let assign13740_e8081: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard304 = assign13740_e8081;

        let (assign13750_e8087, assign13750_e8087_d_n0, assign13750_e8087_d_n2, assign13750_e8087_d_n4, assign13750_e8087_d_n5, assign13750_e8087_d_n6, assign13750_e8087_d_n7, assign13750_e8087_d_n8, assign13750_e8087_d_n9, assign13750_e8087_d_n10, assign13750_e8087_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard304 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign13750_e8087;
        locals.var_ninvdehres_dn0 = assign13750_e8087_d_n0;
        locals.var_ninvdehres_dn2 = assign13750_e8087_d_n2;
        locals.var_ninvdehres_dn4 = assign13750_e8087_d_n4;
        locals.var_ninvdehres_dn5 = assign13750_e8087_d_n5;
        locals.var_ninvdehres_dn6 = assign13750_e8087_d_n6;
        locals.var_ninvdehres_dn7 = assign13750_e8087_d_n7;
        locals.var_ninvdehres_dn8 = assign13750_e8087_d_n8;
        locals.var_ninvdehres_dn9 = assign13750_e8087_d_n9;
        locals.var_ninvdehres_dn10 = assign13750_e8087_d_n10;
        locals.var_ninvdehres_dn13 = assign13750_e8087_d_n13;

        let (assign13760_e8103, assign13760_e8103_d_n0, assign13760_e8103_d_n2, assign13760_e8103_d_n4, assign13760_e8103_d_n5, assign13760_e8103_d_n6, assign13760_e8103_d_n7, assign13760_e8103_d_n8, assign13760_e8103_d_n9, assign13760_e8103_d_n10, assign13760_e8103_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (p.p53 != 0.0)) {
        let assign13760_e8094: f64 = (p.p328 * locals.var_tdiff0);
        let assign13760_e8095: f64 = (locals.var_uc_rth0 + assign13760_e8094);
        let assign13760_e8098: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign13760_e8099: f64 = (assign13760_e8095 + assign13760_e8098);
        let assign13760_e8101: f64 = (assign13760_e8099 * locals.var_rthtemp0);
        (assign13760_e8101, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn13) + (p.p329 * locals.var_tdiff0_2_dn13)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign13760_e8103;
        locals.var_rth_dn0 = assign13760_e8103_d_n0;
        locals.var_rth_dn2 = assign13760_e8103_d_n2;
        locals.var_rth_dn4 = assign13760_e8103_d_n4;
        locals.var_rth_dn5 = assign13760_e8103_d_n5;
        locals.var_rth_dn6 = assign13760_e8103_d_n6;
        locals.var_rth_dn7 = assign13760_e8103_d_n7;
        locals.var_rth_dn8 = assign13760_e8103_d_n8;
        locals.var_rth_dn9 = assign13760_e8103_d_n9;
        locals.var_rth_dn10 = assign13760_e8103_d_n10;
        locals.var_rth_dn13 = assign13760_e8103_d_n13;

        let assign13780_e8111: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign13780_e8111;

        let (assign13790_e8119, assign13790_e8119_d_n0, assign13790_e8119_d_n2, assign13790_e8119_d_n4, assign13790_e8119_d_n5, assign13790_e8119_d_n6, assign13790_e8119_d_n7, assign13790_e8119_d_n8, assign13790_e8119_d_n9, assign13790_e8119_d_n10, assign13790_e8119_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard306 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign13790_e8119;
        locals.var_rth_dn0 = assign13790_e8119_d_n0;
        locals.var_rth_dn2 = assign13790_e8119_d_n2;
        locals.var_rth_dn4 = assign13790_e8119_d_n4;
        locals.var_rth_dn5 = assign13790_e8119_d_n5;
        locals.var_rth_dn6 = assign13790_e8119_d_n6;
        locals.var_rth_dn7 = assign13790_e8119_d_n7;
        locals.var_rth_dn8 = assign13790_e8119_d_n8;
        locals.var_rth_dn9 = assign13790_e8119_d_n9;
        locals.var_rth_dn10 = assign13790_e8119_d_n10;
        locals.var_rth_dn13 = assign13790_e8119_d_n13;

        let (assign13800_e8131, assign13800_e8131_d_n0, assign13800_e8131_d_n2, assign13800_e8131_d_n4, assign13800_e8131_d_n5, assign13800_e8131_d_n6, assign13800_e8131_d_n7, assign13800_e8131_d_n8, assign13800_e8131_d_n9, assign13800_e8131_d_n10, assign13800_e8131_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13800_e8124: f64 = (p.p330 * locals.var_tdiff0);
        let assign13800_e8125: f64 = (locals.var_uc_powrat + assign13800_e8124);
        let assign13800_e8128: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign13800_e8129: f64 = (assign13800_e8125 + assign13800_e8128);
        (assign13800_e8129, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn13) + (p.p331 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign13800_e8131;
        locals.var_t2_dn0 = assign13800_e8131_d_n0;
        locals.var_t2_dn2 = assign13800_e8131_d_n2;
        locals.var_t2_dn4 = assign13800_e8131_d_n4;
        locals.var_t2_dn5 = assign13800_e8131_d_n5;
        locals.var_t2_dn6 = assign13800_e8131_d_n6;
        locals.var_t2_dn7 = assign13800_e8131_d_n7;
        locals.var_t2_dn8 = assign13800_e8131_d_n8;
        locals.var_t2_dn9 = assign13800_e8131_d_n9;
        locals.var_t2_dn10 = assign13800_e8131_d_n10;
        locals.var_t2_dn13 = assign13800_e8131_d_n13;

        let (assign13810_e8139, assign13810_e8139_d_n0, assign13810_e8139_d_n2, assign13810_e8139_d_n4, assign13810_e8139_d_n5, assign13810_e8139_d_n6, assign13810_e8139_d_n7, assign13810_e8139_d_n8, assign13810_e8139_d_n9, assign13810_e8139_d_n10, assign13810_e8139_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13810_e8135: f64 = locals.var_t2;
        let assign13810_e8137: f64 = (assign13810_e8135 - 0.05);
        (assign13810_e8137, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign13810_e8139;
        locals.var_tmf1_dn0 = assign13810_e8139_d_n0;
        locals.var_tmf1_dn2 = assign13810_e8139_d_n2;
        locals.var_tmf1_dn4 = assign13810_e8139_d_n4;
        locals.var_tmf1_dn5 = assign13810_e8139_d_n5;
        locals.var_tmf1_dn6 = assign13810_e8139_d_n6;
        locals.var_tmf1_dn7 = assign13810_e8139_d_n7;
        locals.var_tmf1_dn8 = assign13810_e8139_d_n8;
        locals.var_tmf1_dn9 = assign13810_e8139_d_n9;
        locals.var_tmf1_dn10 = assign13810_e8139_d_n10;
        locals.var_tmf1_dn13 = assign13810_e8139_d_n13;

        let (assign13820_e8147, assign13820_e8147_d_n0, assign13820_e8147_d_n2, assign13820_e8147_d_n4, assign13820_e8147_d_n5, assign13820_e8147_d_n6, assign13820_e8147_d_n7, assign13820_e8147_d_n8, assign13820_e8147_d_n9, assign13820_e8147_d_n10, assign13820_e8147_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13820_e8147;
        locals.var_tmf2_dn0 = assign13820_e8147_d_n0;
        locals.var_tmf2_dn2 = assign13820_e8147_d_n2;
        locals.var_tmf2_dn4 = assign13820_e8147_d_n4;
        locals.var_tmf2_dn5 = assign13820_e8147_d_n5;
        locals.var_tmf2_dn6 = assign13820_e8147_d_n6;
        locals.var_tmf2_dn7 = assign13820_e8147_d_n7;
        locals.var_tmf2_dn8 = assign13820_e8147_d_n8;
        locals.var_tmf2_dn9 = assign13820_e8147_d_n9;
        locals.var_tmf2_dn10 = assign13820_e8147_d_n10;
        locals.var_tmf2_dn13 = assign13820_e8147_d_n13;

        let (assign13830_e8157, assign13830_e8157_d_n0, assign13830_e8157_d_n2, assign13830_e8157_d_n4, assign13830_e8157_d_n5, assign13830_e8157_d_n6, assign13830_e8157_d_n7, assign13830_e8157_d_n8, assign13830_e8157_d_n9, assign13830_e8157_d_n10, assign13830_e8157_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let (assign13830_e8155, assign13830_e8155_d_n0, assign13830_e8155_d_n2, assign13830_e8155_d_n4, assign13830_e8155_d_n5, assign13830_e8155_d_n6, assign13830_e8155_d_n7, assign13830_e8155_d_n8, assign13830_e8155_d_n9, assign13830_e8155_d_n10, assign13830_e8155_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign13830_e8154: f64 = (-locals.var_tmf2);
                (assign13830_e8154, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign13830_e8155, assign13830_e8155_d_n0, assign13830_e8155_d_n2, assign13830_e8155_d_n4, assign13830_e8155_d_n5, assign13830_e8155_d_n6, assign13830_e8155_d_n7, assign13830_e8155_d_n8, assign13830_e8155_d_n9, assign13830_e8155_d_n10, assign13830_e8155_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13830_e8157;
        locals.var_tmf2_dn0 = assign13830_e8157_d_n0;
        locals.var_tmf2_dn2 = assign13830_e8157_d_n2;
        locals.var_tmf2_dn4 = assign13830_e8157_d_n4;
        locals.var_tmf2_dn5 = assign13830_e8157_d_n5;
        locals.var_tmf2_dn6 = assign13830_e8157_d_n6;
        locals.var_tmf2_dn7 = assign13830_e8157_d_n7;
        locals.var_tmf2_dn8 = assign13830_e8157_d_n8;
        locals.var_tmf2_dn9 = assign13830_e8157_d_n9;
        locals.var_tmf2_dn10 = assign13830_e8157_d_n10;
        locals.var_tmf2_dn13 = assign13830_e8157_d_n13;

        let (assign13840_e8166, assign13840_e8166_d_n0, assign13840_e8166_d_n2, assign13840_e8166_d_n4, assign13840_e8166_d_n5, assign13840_e8166_d_n6, assign13840_e8166_d_n7, assign13840_e8166_d_n8, assign13840_e8166_d_n9, assign13840_e8166_d_n10, assign13840_e8166_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13840_e8161: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13840_e8163: f64 = (assign13840_e8161 + locals.var_tmf2);
        let assign13840_e8164: f64 = (assign13840_e8163).sqrt();
        (assign13840_e8164, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13840_e8164)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign13840_e8164)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13840_e8166;
        locals.var_tmf2_dn0 = assign13840_e8166_d_n0;
        locals.var_tmf2_dn2 = assign13840_e8166_d_n2;
        locals.var_tmf2_dn4 = assign13840_e8166_d_n4;
        locals.var_tmf2_dn5 = assign13840_e8166_d_n5;
        locals.var_tmf2_dn6 = assign13840_e8166_d_n6;
        locals.var_tmf2_dn7 = assign13840_e8166_d_n7;
        locals.var_tmf2_dn8 = assign13840_e8166_d_n8;
        locals.var_tmf2_dn9 = assign13840_e8166_d_n9;
        locals.var_tmf2_dn10 = assign13840_e8166_d_n10;
        locals.var_tmf2_dn13 = assign13840_e8166_d_n13;

        let (assign13850_e8176, assign13850_e8176_d_n0, assign13850_e8176_d_n2, assign13850_e8176_d_n4, assign13850_e8176_d_n5, assign13850_e8176_d_n6, assign13850_e8176_d_n7, assign13850_e8176_d_n8, assign13850_e8176_d_n9, assign13850_e8176_d_n10, assign13850_e8176_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13850_e8172: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13850_e8173: f64 = (1.0 + assign13850_e8172);
        let assign13850_e8174: f64 = (0.5 * assign13850_e8173);
        (assign13850_e8174, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13850_e8176;
        locals.var_t0_dn0 = assign13850_e8176_d_n0;
        locals.var_t0_dn2 = assign13850_e8176_d_n2;
        locals.var_t0_dn4 = assign13850_e8176_d_n4;
        locals.var_t0_dn5 = assign13850_e8176_d_n5;
        locals.var_t0_dn6 = assign13850_e8176_d_n6;
        locals.var_t0_dn7 = assign13850_e8176_d_n7;
        locals.var_t0_dn8 = assign13850_e8176_d_n8;
        locals.var_t0_dn9 = assign13850_e8176_d_n9;
        locals.var_t0_dn10 = assign13850_e8176_d_n10;
        locals.var_t0_dn13 = assign13850_e8176_d_n13;

        let (assign13860_e8186, assign13860_e8186_d_n0, assign13860_e8186_d_n2, assign13860_e8186_d_n4, assign13860_e8186_d_n5, assign13860_e8186_d_n6, assign13860_e8186_d_n7, assign13860_e8186_d_n8, assign13860_e8186_d_n9, assign13860_e8186_d_n10, assign13860_e8186_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13860_e8182: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13860_e8183: f64 = (0.5 * assign13860_e8182);
        let assign13860_e8184: f64 = assign13860_e8183;
        (assign13860_e8184, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign13860_e8186;
        locals.var_t2_dn0 = assign13860_e8186_d_n0;
        locals.var_t2_dn2 = assign13860_e8186_d_n2;
        locals.var_t2_dn4 = assign13860_e8186_d_n4;
        locals.var_t2_dn5 = assign13860_e8186_d_n5;
        locals.var_t2_dn6 = assign13860_e8186_d_n6;
        locals.var_t2_dn7 = assign13860_e8186_d_n7;
        locals.var_t2_dn8 = assign13860_e8186_d_n8;
        locals.var_t2_dn9 = assign13860_e8186_d_n9;
        locals.var_t2_dn10 = assign13860_e8186_d_n10;
        locals.var_t2_dn13 = assign13860_e8186_d_n13;

        let (assign13870_e8194, assign13870_e8194_d_n0, assign13870_e8194_d_n2, assign13870_e8194_d_n4, assign13870_e8194_d_n5, assign13870_e8194_d_n6, assign13870_e8194_d_n7, assign13870_e8194_d_n8, assign13870_e8194_d_n9, assign13870_e8194_d_n10, assign13870_e8194_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13870_e8190: f64 = (1.0 - locals.var_t2);
        let assign13870_e8192: f64 = (assign13870_e8190 - 0.05);
        (assign13870_e8192, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign13870_e8194;
        locals.var_tmf1_dn0 = assign13870_e8194_d_n0;
        locals.var_tmf1_dn2 = assign13870_e8194_d_n2;
        locals.var_tmf1_dn4 = assign13870_e8194_d_n4;
        locals.var_tmf1_dn5 = assign13870_e8194_d_n5;
        locals.var_tmf1_dn6 = assign13870_e8194_d_n6;
        locals.var_tmf1_dn7 = assign13870_e8194_d_n7;
        locals.var_tmf1_dn8 = assign13870_e8194_d_n8;
        locals.var_tmf1_dn9 = assign13870_e8194_d_n9;
        locals.var_tmf1_dn10 = assign13870_e8194_d_n10;
        locals.var_tmf1_dn13 = assign13870_e8194_d_n13;

        let (assign13880_e8202, assign13880_e8202_d_n0, assign13880_e8202_d_n2, assign13880_e8202_d_n4, assign13880_e8202_d_n5, assign13880_e8202_d_n6, assign13880_e8202_d_n7, assign13880_e8202_d_n8, assign13880_e8202_d_n9, assign13880_e8202_d_n10, assign13880_e8202_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13880_e8198: f64 = 4.0;
        let assign13880_e8200: f64 = (assign13880_e8198 * 0.05);
        (assign13880_e8200, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13880_e8202;
        locals.var_tmf2_dn0 = assign13880_e8202_d_n0;
        locals.var_tmf2_dn2 = assign13880_e8202_d_n2;
        locals.var_tmf2_dn4 = assign13880_e8202_d_n4;
        locals.var_tmf2_dn5 = assign13880_e8202_d_n5;
        locals.var_tmf2_dn6 = assign13880_e8202_d_n6;
        locals.var_tmf2_dn7 = assign13880_e8202_d_n7;
        locals.var_tmf2_dn8 = assign13880_e8202_d_n8;
        locals.var_tmf2_dn9 = assign13880_e8202_d_n9;
        locals.var_tmf2_dn10 = assign13880_e8202_d_n10;
        locals.var_tmf2_dn13 = assign13880_e8202_d_n13;

        let (assign13890_e8212, assign13890_e8212_d_n0, assign13890_e8212_d_n2, assign13890_e8212_d_n4, assign13890_e8212_d_n5, assign13890_e8212_d_n6, assign13890_e8212_d_n7, assign13890_e8212_d_n8, assign13890_e8212_d_n9, assign13890_e8212_d_n10, assign13890_e8212_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let (assign13890_e8210, assign13890_e8210_d_n0, assign13890_e8210_d_n2, assign13890_e8210_d_n4, assign13890_e8210_d_n5, assign13890_e8210_d_n6, assign13890_e8210_d_n7, assign13890_e8210_d_n8, assign13890_e8210_d_n9, assign13890_e8210_d_n10, assign13890_e8210_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign13890_e8209: f64 = (-locals.var_tmf2);
                (assign13890_e8209, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign13890_e8210, assign13890_e8210_d_n0, assign13890_e8210_d_n2, assign13890_e8210_d_n4, assign13890_e8210_d_n5, assign13890_e8210_d_n6, assign13890_e8210_d_n7, assign13890_e8210_d_n8, assign13890_e8210_d_n9, assign13890_e8210_d_n10, assign13890_e8210_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13890_e8212;
        locals.var_tmf2_dn0 = assign13890_e8212_d_n0;
        locals.var_tmf2_dn2 = assign13890_e8212_d_n2;
        locals.var_tmf2_dn4 = assign13890_e8212_d_n4;
        locals.var_tmf2_dn5 = assign13890_e8212_d_n5;
        locals.var_tmf2_dn6 = assign13890_e8212_d_n6;
        locals.var_tmf2_dn7 = assign13890_e8212_d_n7;
        locals.var_tmf2_dn8 = assign13890_e8212_d_n8;
        locals.var_tmf2_dn9 = assign13890_e8212_d_n9;
        locals.var_tmf2_dn10 = assign13890_e8212_d_n10;
        locals.var_tmf2_dn13 = assign13890_e8212_d_n13;

        let (assign13900_e8221, assign13900_e8221_d_n0, assign13900_e8221_d_n2, assign13900_e8221_d_n4, assign13900_e8221_d_n5, assign13900_e8221_d_n6, assign13900_e8221_d_n7, assign13900_e8221_d_n8, assign13900_e8221_d_n9, assign13900_e8221_d_n10, assign13900_e8221_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13900_e8216: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13900_e8218: f64 = (assign13900_e8216 + locals.var_tmf2);
        let assign13900_e8219: f64 = (assign13900_e8218).sqrt();
        (assign13900_e8219, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13900_e8219)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign13900_e8219)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign13900_e8221;
        locals.var_tmf2_dn0 = assign13900_e8221_d_n0;
        locals.var_tmf2_dn2 = assign13900_e8221_d_n2;
        locals.var_tmf2_dn4 = assign13900_e8221_d_n4;
        locals.var_tmf2_dn5 = assign13900_e8221_d_n5;
        locals.var_tmf2_dn6 = assign13900_e8221_d_n6;
        locals.var_tmf2_dn7 = assign13900_e8221_d_n7;
        locals.var_tmf2_dn8 = assign13900_e8221_d_n8;
        locals.var_tmf2_dn9 = assign13900_e8221_d_n9;
        locals.var_tmf2_dn10 = assign13900_e8221_d_n10;
        locals.var_tmf2_dn13 = assign13900_e8221_d_n13;

        let (assign13910_e8231, assign13910_e8231_d_n0, assign13910_e8231_d_n2, assign13910_e8231_d_n4, assign13910_e8231_d_n5, assign13910_e8231_d_n6, assign13910_e8231_d_n7, assign13910_e8231_d_n8, assign13910_e8231_d_n9, assign13910_e8231_d_n10, assign13910_e8231_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13910_e8227: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13910_e8228: f64 = (1.0 + assign13910_e8227);
        let assign13910_e8229: f64 = (0.5 * assign13910_e8228);
        (assign13910_e8229, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13910_e8231;
        locals.var_t0_dn0 = assign13910_e8231_d_n0;
        locals.var_t0_dn2 = assign13910_e8231_d_n2;
        locals.var_t0_dn4 = assign13910_e8231_d_n4;
        locals.var_t0_dn5 = assign13910_e8231_d_n5;
        locals.var_t0_dn6 = assign13910_e8231_d_n6;
        locals.var_t0_dn7 = assign13910_e8231_d_n7;
        locals.var_t0_dn8 = assign13910_e8231_d_n8;
        locals.var_t0_dn9 = assign13910_e8231_d_n9;
        locals.var_t0_dn10 = assign13910_e8231_d_n10;
        locals.var_t0_dn13 = assign13910_e8231_d_n13;

        let (assign13920_e8241, assign13920_e8241_d_n0, assign13920_e8241_d_n2, assign13920_e8241_d_n4, assign13920_e8241_d_n5, assign13920_e8241_d_n6, assign13920_e8241_d_n7, assign13920_e8241_d_n8, assign13920_e8241_d_n9, assign13920_e8241_d_n10, assign13920_e8241_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13920_e8237: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13920_e8238: f64 = (0.5 * assign13920_e8237);
        let assign13920_e8239: f64 = (1.0 - assign13920_e8238);
        (assign13920_e8239, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn13,)
    }
};
        locals.var_powratio = assign13920_e8241;
        locals.var_powratio_dn0 = assign13920_e8241_d_n0;
        locals.var_powratio_dn2 = assign13920_e8241_d_n2;
        locals.var_powratio_dn4 = assign13920_e8241_d_n4;
        locals.var_powratio_dn5 = assign13920_e8241_d_n5;
        locals.var_powratio_dn6 = assign13920_e8241_d_n6;
        locals.var_powratio_dn7 = assign13920_e8241_d_n7;
        locals.var_powratio_dn8 = assign13920_e8241_d_n8;
        locals.var_powratio_dn9 = assign13920_e8241_d_n9;
        locals.var_powratio_dn10 = assign13920_e8241_d_n10;
        locals.var_powratio_dn13 = assign13920_e8241_d_n13;

        let (assign13930_e8252, assign13930_e8252_d_n0, assign13930_e8252_d_n2, assign13930_e8252_d_n4, assign13930_e8252_d_n5, assign13930_e8252_d_n6, assign13930_e8252_d_n7, assign13930_e8252_d_n8, assign13930_e8252_d_n9, assign13930_e8252_d_n10, assign13930_e8252_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13930_e8245: f64 = (2.0 * locals.var_beta_inv);
        let assign13930_e8248: f64 = (locals.var_nsub / locals.var_nin);
        let assign13930_e8249: f64 = (assign13930_e8248).ln();
        let assign13930_e8250: f64 = (assign13930_e8245 * assign13930_e8249);
        (assign13930_e8250, (((2.0 * locals.var_beta_inv_dn0) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn2) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn4) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn5) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn6) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn7) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn8) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn9) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn10) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))), (((2.0 * locals.var_beta_inv_dn13) * assign13930_e8249) + (assign13930_e8245 * ((((locals.var_nsub_dn13 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13930_e8248))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn13,)
    }
};
        locals.var_pb2 = assign13930_e8252;
        locals.var_pb2_dn0 = assign13930_e8252_d_n0;
        locals.var_pb2_dn2 = assign13930_e8252_d_n2;
        locals.var_pb2_dn4 = assign13930_e8252_d_n4;
        locals.var_pb2_dn5 = assign13930_e8252_d_n5;
        locals.var_pb2_dn6 = assign13930_e8252_d_n6;
        locals.var_pb2_dn7 = assign13930_e8252_d_n7;
        locals.var_pb2_dn8 = assign13930_e8252_d_n8;
        locals.var_pb2_dn9 = assign13930_e8252_d_n9;
        locals.var_pb2_dn10 = assign13930_e8252_d_n10;
        locals.var_pb2_dn13 = assign13930_e8252_d_n13;

        let (assign13940_e8260, assign13940_e8260_d_n0, assign13940_e8260_d_n2, assign13940_e8260_d_n4, assign13940_e8260_d_n5, assign13940_e8260_d_n6, assign13940_e8260_d_n7, assign13940_e8260_d_n8, assign13940_e8260_d_n9, assign13940_e8260_d_n10, assign13940_e8260_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13940_e8256: f64 = (2.0 * 1.034943e-10);
        let assign13940_e8258: f64 = (assign13940_e8256 / 1.6021918e-19);
        (assign13940_e8258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13940_e8260;
        locals.var_t1_dn0 = assign13940_e8260_d_n0;
        locals.var_t1_dn2 = assign13940_e8260_d_n2;
        locals.var_t1_dn4 = assign13940_e8260_d_n4;
        locals.var_t1_dn5 = assign13940_e8260_d_n5;
        locals.var_t1_dn6 = assign13940_e8260_d_n6;
        locals.var_t1_dn7 = assign13940_e8260_d_n7;
        locals.var_t1_dn8 = assign13940_e8260_d_n8;
        locals.var_t1_dn9 = assign13940_e8260_d_n9;
        locals.var_t1_dn10 = assign13940_e8260_d_n10;
        locals.var_t1_dn13 = assign13940_e8260_d_n13;

        let (assign13950_e8267, assign13950_e8267_d_n0, assign13950_e8267_d_n2, assign13950_e8267_d_n4, assign13950_e8267_d_n5, assign13950_e8267_d_n6, assign13950_e8267_d_n7, assign13950_e8267_d_n8, assign13950_e8267_d_n9, assign13950_e8267_d_n10, assign13950_e8267_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13950_e8264: f64 = (locals.var_t1 / locals.var_nsub);
        let assign13950_e8265: f64 = (assign13950_e8264).sqrt();
        (assign13950_e8265, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)), ((((locals.var_t1_dn13 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn13)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13950_e8265)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn13,)
    }
};
        locals.var_wdpl = assign13950_e8267;
        locals.var_wdpl_dn0 = assign13950_e8267_d_n0;
        locals.var_wdpl_dn2 = assign13950_e8267_d_n2;
        locals.var_wdpl_dn4 = assign13950_e8267_d_n4;
        locals.var_wdpl_dn5 = assign13950_e8267_d_n5;
        locals.var_wdpl_dn6 = assign13950_e8267_d_n6;
        locals.var_wdpl_dn7 = assign13950_e8267_d_n7;
        locals.var_wdpl_dn8 = assign13950_e8267_d_n8;
        locals.var_wdpl_dn9 = assign13950_e8267_d_n9;
        locals.var_wdpl_dn10 = assign13950_e8267_d_n10;
        locals.var_wdpl_dn13 = assign13950_e8267_d_n13;

        let (assign13960_e8274, assign13960_e8274_d_n0, assign13960_e8274_d_n2, assign13960_e8274_d_n4, assign13960_e8274_d_n5, assign13960_e8274_d_n6, assign13960_e8274_d_n7, assign13960_e8274_d_n8, assign13960_e8274_d_n9, assign13960_e8274_d_n10, assign13960_e8274_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13960_e8271: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign13960_e8272: f64 = (assign13960_e8271).sqrt();
        (assign13960_e8272, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)), ((((locals.var_t1_dn13 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn13)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13960_e8272)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn13,)
    }
};
        locals.var_wdplp = assign13960_e8274;
        locals.var_wdplp_dn0 = assign13960_e8274_d_n0;
        locals.var_wdplp_dn2 = assign13960_e8274_d_n2;
        locals.var_wdplp_dn4 = assign13960_e8274_d_n4;
        locals.var_wdplp_dn5 = assign13960_e8274_d_n5;
        locals.var_wdplp_dn6 = assign13960_e8274_d_n6;
        locals.var_wdplp_dn7 = assign13960_e8274_d_n7;
        locals.var_wdplp_dn8 = assign13960_e8274_d_n8;
        locals.var_wdplp_dn9 = assign13960_e8274_d_n9;
        locals.var_wdplp_dn10 = assign13960_e8274_d_n10;
        locals.var_wdplp_dn13 = assign13960_e8274_d_n13;

        let assign13970_e8277: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign13970_e8277;

    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13980_e8292, assign13980_e8292_d_n0, assign13980_e8292_d_n2, assign13980_e8292_d_n4, assign13980_e8292_d_n5, assign13980_e8292_d_n6, assign13980_e8292_d_n7, assign13980_e8292_d_n8, assign13980_e8292_d_n9, assign13980_e8292_d_n10, assign13980_e8292_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard307 != 0.0)) {
        let assign13980_e8283: f64 = (2.0 * 1.034943e-10);
        let assign13980_e8285: f64 = (assign13980_e8283 * 1.6021918e-19);
        let assign13980_e8287: f64 = (assign13980_e8285 * locals.var_nsub);
        let assign13980_e8289: f64 = (assign13980_e8287 * locals.var_beta_inv);
        let assign13980_e8290: f64 = (assign13980_e8289).sqrt();
        (assign13980_e8290, ((((assign13980_e8285 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn0)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn2)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn4)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn5)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn6)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn7)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn8)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn9)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn10)) / (2.0 * assign13980_e8290)), ((((assign13980_e8285 * locals.var_nsub_dn13) * locals.var_beta_inv) + (assign13980_e8287 * locals.var_beta_inv_dn13)) / (2.0 * assign13980_e8290)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign13980_e8292;
        locals.var_cnst0_dn0 = assign13980_e8292_d_n0;
        locals.var_cnst0_dn2 = assign13980_e8292_d_n2;
        locals.var_cnst0_dn4 = assign13980_e8292_d_n4;
        locals.var_cnst0_dn5 = assign13980_e8292_d_n5;
        locals.var_cnst0_dn6 = assign13980_e8292_d_n6;
        locals.var_cnst0_dn7 = assign13980_e8292_d_n7;
        locals.var_cnst0_dn8 = assign13980_e8292_d_n8;
        locals.var_cnst0_dn9 = assign13980_e8292_d_n9;
        locals.var_cnst0_dn10 = assign13980_e8292_d_n10;
        locals.var_cnst0_dn13 = assign13980_e8292_d_n13;

        let (assign13990_e8300, assign13990_e8300_d_n0, assign13990_e8300_d_n2, assign13990_e8300_d_n4, assign13990_e8300_d_n5, assign13990_e8300_d_n6, assign13990_e8300_d_n7, assign13990_e8300_d_n8, assign13990_e8300_d_n9, assign13990_e8300_d_n10, assign13990_e8300_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard307 != 0.0)) {
        let assign13990_e8298: f64 = (locals.var_nin / locals.var_nsub);
        (assign13990_e8298, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn13 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn13)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13990_e8300;
        locals.var_t1_dn0 = assign13990_e8300_d_n0;
        locals.var_t1_dn2 = assign13990_e8300_d_n2;
        locals.var_t1_dn4 = assign13990_e8300_d_n4;
        locals.var_t1_dn5 = assign13990_e8300_d_n5;
        locals.var_t1_dn6 = assign13990_e8300_d_n6;
        locals.var_t1_dn7 = assign13990_e8300_d_n7;
        locals.var_t1_dn8 = assign13990_e8300_d_n8;
        locals.var_t1_dn9 = assign13990_e8300_d_n9;
        locals.var_t1_dn10 = assign13990_e8300_d_n10;
        locals.var_t1_dn13 = assign13990_e8300_d_n13;

        let (assign14000_e8308, assign14000_e8308_d_n0, assign14000_e8308_d_n2, assign14000_e8308_d_n4, assign14000_e8308_d_n5, assign14000_e8308_d_n6, assign14000_e8308_d_n7, assign14000_e8308_d_n8, assign14000_e8308_d_n9, assign14000_e8308_d_n10, assign14000_e8308_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard307 != 0.0)) {
        let assign14000_e8306: f64 = (locals.var_t1 * locals.var_t1);
        (assign14000_e8306, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign14000_e8308;
        locals.var_cnst1_dn0 = assign14000_e8308_d_n0;
        locals.var_cnst1_dn2 = assign14000_e8308_d_n2;
        locals.var_cnst1_dn4 = assign14000_e8308_d_n4;
        locals.var_cnst1_dn5 = assign14000_e8308_d_n5;
        locals.var_cnst1_dn6 = assign14000_e8308_d_n6;
        locals.var_cnst1_dn7 = assign14000_e8308_d_n7;
        locals.var_cnst1_dn8 = assign14000_e8308_d_n8;
        locals.var_cnst1_dn9 = assign14000_e8308_d_n9;
        locals.var_cnst1_dn10 = assign14000_e8308_d_n10;
        locals.var_cnst1_dn13 = assign14000_e8308_d_n13;

        let assign14010_e8311: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign14010_e8311;

        let assign14020_e8314: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign14020_e8314;

        let (assign14030_e8327, assign14030_e8327_d_n0, assign14030_e8327_d_n2, assign14030_e8327_d_n4, assign14030_e8327_d_n5, assign14030_e8327_d_n6, assign14030_e8327_d_n7, assign14030_e8327_d_n8, assign14030_e8327_d_n9, assign14030_e8327_d_n10, assign14030_e8327_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign14030_e8323: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign14030_e8324: f64 = (assign14030_e8323).sqrt();
        let assign14030_e8325: f64 = (locals.var_cnst0 * assign14030_e8324);
        (assign14030_e8325, ((locals.var_cnst0_dn0 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn2 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn4 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn5 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn6 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn7 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn8 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn9 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn10 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))), ((locals.var_cnst0_dn13 * assign14030_e8324) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn13) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14030_e8324)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign14030_e8327;
        locals.var_cnst0over_dn0 = assign14030_e8327_d_n0;
        locals.var_cnst0over_dn2 = assign14030_e8327_d_n2;
        locals.var_cnst0over_dn4 = assign14030_e8327_d_n4;
        locals.var_cnst0over_dn5 = assign14030_e8327_d_n5;
        locals.var_cnst0over_dn6 = assign14030_e8327_d_n6;
        locals.var_cnst0over_dn7 = assign14030_e8327_d_n7;
        locals.var_cnst0over_dn8 = assign14030_e8327_d_n8;
        locals.var_cnst0over_dn9 = assign14030_e8327_d_n9;
        locals.var_cnst0over_dn10 = assign14030_e8327_d_n10;
        locals.var_cnst0over_dn13 = assign14030_e8327_d_n13;

        let assign14040_e8330: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign14040_e8330;

        let (assign14050_e8343, assign14050_e8343_d_n0, assign14050_e8343_d_n2, assign14050_e8343_d_n4, assign14050_e8343_d_n5, assign14050_e8343_d_n6, assign14050_e8343_d_n7, assign14050_e8343_d_n8, assign14050_e8343_d_n9, assign14050_e8343_d_n10, assign14050_e8343_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign14050_e8339: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign14050_e8340: f64 = (assign14050_e8339).sqrt();
        let assign14050_e8341: f64 = (locals.var_cnst0 * assign14050_e8340);
        (assign14050_e8341, ((locals.var_cnst0_dn0 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn2 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn4 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn5 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn6 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn7 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn8 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn9 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn10 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))), ((locals.var_cnst0_dn13 * assign14050_e8340) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn13) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8340)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign14050_e8343;
        locals.var_cnst0overs_dn0 = assign14050_e8343_d_n0;
        locals.var_cnst0overs_dn2 = assign14050_e8343_d_n2;
        locals.var_cnst0overs_dn4 = assign14050_e8343_d_n4;
        locals.var_cnst0overs_dn5 = assign14050_e8343_d_n5;
        locals.var_cnst0overs_dn6 = assign14050_e8343_d_n6;
        locals.var_cnst0overs_dn7 = assign14050_e8343_d_n7;
        locals.var_cnst0overs_dn8 = assign14050_e8343_d_n8;
        locals.var_cnst0overs_dn9 = assign14050_e8343_d_n9;
        locals.var_cnst0overs_dn10 = assign14050_e8343_d_n10;
        locals.var_cnst0overs_dn13 = assign14050_e8343_d_n13;

        let assign14060_e8346: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign14060_e8346;

        let (assign14070_e8360, assign14070_e8360_d_n0, assign14070_e8360_d_n2, assign14070_e8360_d_n4, assign14070_e8360_d_n5, assign14070_e8360_d_n6, assign14070_e8360_d_n7, assign14070_e8360_d_n8, assign14070_e8360_d_n9, assign14070_e8360_d_n10, assign14070_e8360_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard308 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign14070_e8356: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign14070_e8357: f64 = (assign14070_e8356).sqrt();
        let assign14070_e8358: f64 = (locals.var_cnst0 * assign14070_e8357);
        (assign14070_e8358, ((locals.var_cnst0_dn0 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn2 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn4 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn5 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn6 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn7 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn8 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn9 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn10 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))), ((locals.var_cnst0_dn13 * assign14070_e8357) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn13) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14070_e8357)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign14070_e8360;
        locals.var_cnst0over_dn0 = assign14070_e8360_d_n0;
        locals.var_cnst0over_dn2 = assign14070_e8360_d_n2;
        locals.var_cnst0over_dn4 = assign14070_e8360_d_n4;
        locals.var_cnst0over_dn5 = assign14070_e8360_d_n5;
        locals.var_cnst0over_dn6 = assign14070_e8360_d_n6;
        locals.var_cnst0over_dn7 = assign14070_e8360_d_n7;
        locals.var_cnst0over_dn8 = assign14070_e8360_d_n8;
        locals.var_cnst0over_dn9 = assign14070_e8360_d_n9;
        locals.var_cnst0over_dn10 = assign14070_e8360_d_n10;
        locals.var_cnst0over_dn13 = assign14070_e8360_d_n13;

        let assign14080_e8363: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign14080_e8363;

        let (assign14090_e8377, assign14090_e8377_d_n0, assign14090_e8377_d_n2, assign14090_e8377_d_n4, assign14090_e8377_d_n5, assign14090_e8377_d_n6, assign14090_e8377_d_n7, assign14090_e8377_d_n8, assign14090_e8377_d_n9, assign14090_e8377_d_n10, assign14090_e8377_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard308 == 0.0)) && (locals.var_guard312 != 0.0)) {
        let assign14090_e8373: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign14090_e8374: f64 = (assign14090_e8373).sqrt();
        let assign14090_e8375: f64 = (locals.var_cnst0 * assign14090_e8374);
        (assign14090_e8375, ((locals.var_cnst0_dn0 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn2 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn4 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn5 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn6 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn7 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn8 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn9 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn10 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))), ((locals.var_cnst0_dn13 * assign14090_e8374) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn13) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8374)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign14090_e8377;
        locals.var_cnst0overs_dn0 = assign14090_e8377_d_n0;
        locals.var_cnst0overs_dn2 = assign14090_e8377_d_n2;
        locals.var_cnst0overs_dn4 = assign14090_e8377_d_n4;
        locals.var_cnst0overs_dn5 = assign14090_e8377_d_n5;
        locals.var_cnst0overs_dn6 = assign14090_e8377_d_n6;
        locals.var_cnst0overs_dn7 = assign14090_e8377_d_n7;
        locals.var_cnst0overs_dn8 = assign14090_e8377_d_n8;
        locals.var_cnst0overs_dn9 = assign14090_e8377_d_n9;
        locals.var_cnst0overs_dn10 = assign14090_e8377_d_n10;
        locals.var_cnst0overs_dn13 = assign14090_e8377_d_n13;

        let assign14100_e8380: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign14100_e8380;

        let assign14110_e8383: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign14110_e8383;

        let (assign14120_e8407, assign14120_e8407_d_n0, assign14120_e8407_d_n2, assign14120_e8407_d_n4, assign14120_e8407_d_n5, assign14120_e8407_d_n6, assign14120_e8407_d_n7, assign14120_e8407_d_n8, assign14120_e8407_d_n9, assign14120_e8407_d_n10, assign14120_e8407_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign14120_e8392: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign14120_e8394: f64 = (assign14120_e8392 * 1000000.0);
        let assign14120_e8396: f64 = (assign14120_e8394 + locals.var_uc_rdict1);
        let assign14120_e8397: f64 = (locals.var_rdtemp0 * assign14120_e8396);
        let assign14120_e8400: f64 = (p.p68 * p.p100);
        let assign14120_e8402: f64 = (assign14120_e8400 * 1000000.0);
        let assign14120_e8404: f64 = (assign14120_e8402 + p.p101);
        let assign14120_e8405: f64 = (assign14120_e8397 * assign14120_e8404);
        (assign14120_e8405, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14120_e8407;
        locals.var_t2_dn0 = assign14120_e8407_d_n0;
        locals.var_t2_dn2 = assign14120_e8407_d_n2;
        locals.var_t2_dn4 = assign14120_e8407_d_n4;
        locals.var_t2_dn5 = assign14120_e8407_d_n5;
        locals.var_t2_dn6 = assign14120_e8407_d_n6;
        locals.var_t2_dn7 = assign14120_e8407_d_n7;
        locals.var_t2_dn8 = assign14120_e8407_d_n8;
        locals.var_t2_dn9 = assign14120_e8407_d_n9;
        locals.var_t2_dn10 = assign14120_e8407_d_n10;
        locals.var_t2_dn13 = assign14120_e8407_d_n13;

        let assign14130_e8410: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign14130_e8410;

        let (assign14140_e8430, assign14140_e8430_d_n0, assign14140_e8430_d_n2, assign14140_e8430_d_n4, assign14140_e8430_d_n5, assign14140_e8430_d_n6, assign14140_e8430_d_n7, assign14140_e8430_d_n8, assign14140_e8430_d_n9, assign14140_e8430_d_n10, assign14140_e8430_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14140_e8421: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign14140_e8422: f64 = (locals.var_uc_rd + assign14140_e8421);
        let assign14140_e8425: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign14140_e8426: f64 = (assign14140_e8422 + assign14140_e8425);
        let assign14140_e8428: f64 = (assign14140_e8426 * locals.var_t2);
        (assign14140_e8428, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign14140_e8426 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14140_e8430;
        locals.var_rde_dn0 = assign14140_e8430_d_n0;
        locals.var_rde_dn2 = assign14140_e8430_d_n2;
        locals.var_rde_dn4 = assign14140_e8430_d_n4;
        locals.var_rde_dn5 = assign14140_e8430_d_n5;
        locals.var_rde_dn6 = assign14140_e8430_d_n6;
        locals.var_rde_dn7 = assign14140_e8430_d_n7;
        locals.var_rde_dn8 = assign14140_e8430_d_n8;
        locals.var_rde_dn9 = assign14140_e8430_d_n9;
        locals.var_rde_dn10 = assign14140_e8430_d_n10;
        locals.var_rde_dn13 = assign14140_e8430_d_n13;

        let (assign14150_e8448, assign14150_e8448_d_n0, assign14150_e8448_d_n2, assign14150_e8448_d_n4, assign14150_e8448_d_n5, assign14150_e8448_d_n6, assign14150_e8448_d_n7, assign14150_e8448_d_n8, assign14150_e8448_d_n9, assign14150_e8448_d_n10, assign14150_e8448_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14150_e8441: f64 = (0.005 * locals.var_uc_rd);
        let assign14150_e8442: f64 = (locals.var_rde - assign14150_e8441);
        let assign14150_e8445: f64 = (0.01 * locals.var_uc_rd);
        let assign14150_e8446: f64 = (assign14150_e8442 - assign14150_e8445);
        (assign14150_e8446, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14150_e8448;
        locals.var_tmf1_dn0 = assign14150_e8448_d_n0;
        locals.var_tmf1_dn2 = assign14150_e8448_d_n2;
        locals.var_tmf1_dn4 = assign14150_e8448_d_n4;
        locals.var_tmf1_dn5 = assign14150_e8448_d_n5;
        locals.var_tmf1_dn6 = assign14150_e8448_d_n6;
        locals.var_tmf1_dn7 = assign14150_e8448_d_n7;
        locals.var_tmf1_dn8 = assign14150_e8448_d_n8;
        locals.var_tmf1_dn9 = assign14150_e8448_d_n9;
        locals.var_tmf1_dn10 = assign14150_e8448_d_n10;
        locals.var_tmf1_dn13 = assign14150_e8448_d_n13;

        let (assign14160_e8466, assign14160_e8466_d_n0, assign14160_e8466_d_n2, assign14160_e8466_d_n4, assign14160_e8466_d_n5, assign14160_e8466_d_n6, assign14160_e8466_d_n7, assign14160_e8466_d_n8, assign14160_e8466_d_n9, assign14160_e8466_d_n10, assign14160_e8466_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14160_e8459: f64 = (0.005 * locals.var_uc_rd);
        let assign14160_e8460: f64 = (4.0 * assign14160_e8459);
        let assign14160_e8463: f64 = (0.01 * locals.var_uc_rd);
        let assign14160_e8464: f64 = (assign14160_e8460 * assign14160_e8463);
        (assign14160_e8464, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14160_e8466;
        locals.var_tmf2_dn0 = assign14160_e8466_d_n0;
        locals.var_tmf2_dn2 = assign14160_e8466_d_n2;
        locals.var_tmf2_dn4 = assign14160_e8466_d_n4;
        locals.var_tmf2_dn5 = assign14160_e8466_d_n5;
        locals.var_tmf2_dn6 = assign14160_e8466_d_n6;
        locals.var_tmf2_dn7 = assign14160_e8466_d_n7;
        locals.var_tmf2_dn8 = assign14160_e8466_d_n8;
        locals.var_tmf2_dn9 = assign14160_e8466_d_n9;
        locals.var_tmf2_dn10 = assign14160_e8466_d_n10;
        locals.var_tmf2_dn13 = assign14160_e8466_d_n13;

        let (assign14170_e8482, assign14170_e8482_d_n0, assign14170_e8482_d_n2, assign14170_e8482_d_n4, assign14170_e8482_d_n5, assign14170_e8482_d_n6, assign14170_e8482_d_n7, assign14170_e8482_d_n8, assign14170_e8482_d_n9, assign14170_e8482_d_n10, assign14170_e8482_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let (assign14170_e8480, assign14170_e8480_d_n0, assign14170_e8480_d_n2, assign14170_e8480_d_n4, assign14170_e8480_d_n5, assign14170_e8480_d_n6, assign14170_e8480_d_n7, assign14170_e8480_d_n8, assign14170_e8480_d_n9, assign14170_e8480_d_n10, assign14170_e8480_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14170_e8479: f64 = (-locals.var_tmf2);
                (assign14170_e8479, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14170_e8480, assign14170_e8480_d_n0, assign14170_e8480_d_n2, assign14170_e8480_d_n4, assign14170_e8480_d_n5, assign14170_e8480_d_n6, assign14170_e8480_d_n7, assign14170_e8480_d_n8, assign14170_e8480_d_n9, assign14170_e8480_d_n10, assign14170_e8480_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14170_e8482;
        locals.var_tmf2_dn0 = assign14170_e8482_d_n0;
        locals.var_tmf2_dn2 = assign14170_e8482_d_n2;
        locals.var_tmf2_dn4 = assign14170_e8482_d_n4;
        locals.var_tmf2_dn5 = assign14170_e8482_d_n5;
        locals.var_tmf2_dn6 = assign14170_e8482_d_n6;
        locals.var_tmf2_dn7 = assign14170_e8482_d_n7;
        locals.var_tmf2_dn8 = assign14170_e8482_d_n8;
        locals.var_tmf2_dn9 = assign14170_e8482_d_n9;
        locals.var_tmf2_dn10 = assign14170_e8482_d_n10;
        locals.var_tmf2_dn13 = assign14170_e8482_d_n13;

        let (assign14180_e8497, assign14180_e8497_d_n0, assign14180_e8497_d_n2, assign14180_e8497_d_n4, assign14180_e8497_d_n5, assign14180_e8497_d_n6, assign14180_e8497_d_n7, assign14180_e8497_d_n8, assign14180_e8497_d_n9, assign14180_e8497_d_n10, assign14180_e8497_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14180_e8492: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14180_e8494: f64 = (assign14180_e8492 + locals.var_tmf2);
        let assign14180_e8495: f64 = (assign14180_e8494).sqrt();
        (assign14180_e8495, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14180_e8495)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14180_e8495)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14180_e8497;
        locals.var_tmf2_dn0 = assign14180_e8497_d_n0;
        locals.var_tmf2_dn2 = assign14180_e8497_d_n2;
        locals.var_tmf2_dn4 = assign14180_e8497_d_n4;
        locals.var_tmf2_dn5 = assign14180_e8497_d_n5;
        locals.var_tmf2_dn6 = assign14180_e8497_d_n6;
        locals.var_tmf2_dn7 = assign14180_e8497_d_n7;
        locals.var_tmf2_dn8 = assign14180_e8497_d_n8;
        locals.var_tmf2_dn9 = assign14180_e8497_d_n9;
        locals.var_tmf2_dn10 = assign14180_e8497_d_n10;
        locals.var_tmf2_dn13 = assign14180_e8497_d_n13;

        let (assign14190_e8513, assign14190_e8513_d_n0, assign14190_e8513_d_n2, assign14190_e8513_d_n4, assign14190_e8513_d_n5, assign14190_e8513_d_n6, assign14190_e8513_d_n7, assign14190_e8513_d_n8, assign14190_e8513_d_n9, assign14190_e8513_d_n10, assign14190_e8513_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14190_e8509: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14190_e8510: f64 = (1.0 + assign14190_e8509);
        let assign14190_e8511: f64 = (0.5 * assign14190_e8510);
        (assign14190_e8511, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14190_e8513;
        locals.var_t0_dn0 = assign14190_e8513_d_n0;
        locals.var_t0_dn2 = assign14190_e8513_d_n2;
        locals.var_t0_dn4 = assign14190_e8513_d_n4;
        locals.var_t0_dn5 = assign14190_e8513_d_n5;
        locals.var_t0_dn6 = assign14190_e8513_d_n6;
        locals.var_t0_dn7 = assign14190_e8513_d_n7;
        locals.var_t0_dn8 = assign14190_e8513_d_n8;
        locals.var_t0_dn9 = assign14190_e8513_d_n9;
        locals.var_t0_dn10 = assign14190_e8513_d_n10;
        locals.var_t0_dn13 = assign14190_e8513_d_n13;

        let (assign14200_e8531, assign14200_e8531_d_n0, assign14200_e8531_d_n2, assign14200_e8531_d_n4, assign14200_e8531_d_n5, assign14200_e8531_d_n6, assign14200_e8531_d_n7, assign14200_e8531_d_n8, assign14200_e8531_d_n9, assign14200_e8531_d_n10, assign14200_e8531_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign14200_e8523: f64 = (0.005 * locals.var_uc_rd);
        let assign14200_e8527: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14200_e8528: f64 = (0.5 * assign14200_e8527);
        let assign14200_e8529: f64 = (assign14200_e8523 + assign14200_e8528);
        (assign14200_e8529, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14200_e8531;
        locals.var_rde_dn0 = assign14200_e8531_d_n0;
        locals.var_rde_dn2 = assign14200_e8531_d_n2;
        locals.var_rde_dn4 = assign14200_e8531_d_n4;
        locals.var_rde_dn5 = assign14200_e8531_d_n5;
        locals.var_rde_dn6 = assign14200_e8531_d_n6;
        locals.var_rde_dn7 = assign14200_e8531_d_n7;
        locals.var_rde_dn8 = assign14200_e8531_d_n8;
        locals.var_rde_dn9 = assign14200_e8531_d_n9;
        locals.var_rde_dn10 = assign14200_e8531_d_n10;
        locals.var_rde_dn13 = assign14200_e8531_d_n13;

        let (assign14210_e8552, assign14210_e8552_d_n0, assign14210_e8552_d_n2, assign14210_e8552_d_n4, assign14210_e8552_d_n5, assign14210_e8552_d_n6, assign14210_e8552_d_n7, assign14210_e8552_d_n8, assign14210_e8552_d_n9, assign14210_e8552_d_n10, assign14210_e8552_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14210_e8543: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign14210_e8544: f64 = (locals.var_uc_rd + assign14210_e8543);
        let assign14210_e8547: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign14210_e8548: f64 = (assign14210_e8544 + assign14210_e8547);
        let assign14210_e8550: f64 = (assign14210_e8548 * locals.var_t2);
        (assign14210_e8550, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign14210_e8548 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14210_e8552;
        locals.var_rde_dn0 = assign14210_e8552_d_n0;
        locals.var_rde_dn2 = assign14210_e8552_d_n2;
        locals.var_rde_dn4 = assign14210_e8552_d_n4;
        locals.var_rde_dn5 = assign14210_e8552_d_n5;
        locals.var_rde_dn6 = assign14210_e8552_d_n6;
        locals.var_rde_dn7 = assign14210_e8552_d_n7;
        locals.var_rde_dn8 = assign14210_e8552_d_n8;
        locals.var_rde_dn9 = assign14210_e8552_d_n9;
        locals.var_rde_dn10 = assign14210_e8552_d_n10;
        locals.var_rde_dn13 = assign14210_e8552_d_n13;

        let (assign14220_e8571, assign14220_e8571_d_n0, assign14220_e8571_d_n2, assign14220_e8571_d_n4, assign14220_e8571_d_n5, assign14220_e8571_d_n6, assign14220_e8571_d_n7, assign14220_e8571_d_n8, assign14220_e8571_d_n9, assign14220_e8571_d_n10, assign14220_e8571_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14220_e8564: f64 = (0.005 * locals.var_uc_rd);
        let assign14220_e8565: f64 = (locals.var_rde - assign14220_e8564);
        let assign14220_e8568: f64 = (0.01 * locals.var_uc_rd);
        let assign14220_e8569: f64 = (assign14220_e8565 - assign14220_e8568);
        (assign14220_e8569, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14220_e8571;
        locals.var_tmf1_dn0 = assign14220_e8571_d_n0;
        locals.var_tmf1_dn2 = assign14220_e8571_d_n2;
        locals.var_tmf1_dn4 = assign14220_e8571_d_n4;
        locals.var_tmf1_dn5 = assign14220_e8571_d_n5;
        locals.var_tmf1_dn6 = assign14220_e8571_d_n6;
        locals.var_tmf1_dn7 = assign14220_e8571_d_n7;
        locals.var_tmf1_dn8 = assign14220_e8571_d_n8;
        locals.var_tmf1_dn9 = assign14220_e8571_d_n9;
        locals.var_tmf1_dn10 = assign14220_e8571_d_n10;
        locals.var_tmf1_dn13 = assign14220_e8571_d_n13;

        let (assign14230_e8590, assign14230_e8590_d_n0, assign14230_e8590_d_n2, assign14230_e8590_d_n4, assign14230_e8590_d_n5, assign14230_e8590_d_n6, assign14230_e8590_d_n7, assign14230_e8590_d_n8, assign14230_e8590_d_n9, assign14230_e8590_d_n10, assign14230_e8590_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14230_e8583: f64 = (0.005 * locals.var_uc_rd);
        let assign14230_e8584: f64 = (4.0 * assign14230_e8583);
        let assign14230_e8587: f64 = (0.01 * locals.var_uc_rd);
        let assign14230_e8588: f64 = (assign14230_e8584 * assign14230_e8587);
        (assign14230_e8588, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14230_e8590;
        locals.var_tmf2_dn0 = assign14230_e8590_d_n0;
        locals.var_tmf2_dn2 = assign14230_e8590_d_n2;
        locals.var_tmf2_dn4 = assign14230_e8590_d_n4;
        locals.var_tmf2_dn5 = assign14230_e8590_d_n5;
        locals.var_tmf2_dn6 = assign14230_e8590_d_n6;
        locals.var_tmf2_dn7 = assign14230_e8590_d_n7;
        locals.var_tmf2_dn8 = assign14230_e8590_d_n8;
        locals.var_tmf2_dn9 = assign14230_e8590_d_n9;
        locals.var_tmf2_dn10 = assign14230_e8590_d_n10;
        locals.var_tmf2_dn13 = assign14230_e8590_d_n13;

        let (assign14240_e8607, assign14240_e8607_d_n0, assign14240_e8607_d_n2, assign14240_e8607_d_n4, assign14240_e8607_d_n5, assign14240_e8607_d_n6, assign14240_e8607_d_n7, assign14240_e8607_d_n8, assign14240_e8607_d_n9, assign14240_e8607_d_n10, assign14240_e8607_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let (assign14240_e8605, assign14240_e8605_d_n0, assign14240_e8605_d_n2, assign14240_e8605_d_n4, assign14240_e8605_d_n5, assign14240_e8605_d_n6, assign14240_e8605_d_n7, assign14240_e8605_d_n8, assign14240_e8605_d_n9, assign14240_e8605_d_n10, assign14240_e8605_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14240_e8604: f64 = (-locals.var_tmf2);
                (assign14240_e8604, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14240_e8605, assign14240_e8605_d_n0, assign14240_e8605_d_n2, assign14240_e8605_d_n4, assign14240_e8605_d_n5, assign14240_e8605_d_n6, assign14240_e8605_d_n7, assign14240_e8605_d_n8, assign14240_e8605_d_n9, assign14240_e8605_d_n10, assign14240_e8605_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14240_e8607;
        locals.var_tmf2_dn0 = assign14240_e8607_d_n0;
        locals.var_tmf2_dn2 = assign14240_e8607_d_n2;
        locals.var_tmf2_dn4 = assign14240_e8607_d_n4;
        locals.var_tmf2_dn5 = assign14240_e8607_d_n5;
        locals.var_tmf2_dn6 = assign14240_e8607_d_n6;
        locals.var_tmf2_dn7 = assign14240_e8607_d_n7;
        locals.var_tmf2_dn8 = assign14240_e8607_d_n8;
        locals.var_tmf2_dn9 = assign14240_e8607_d_n9;
        locals.var_tmf2_dn10 = assign14240_e8607_d_n10;
        locals.var_tmf2_dn13 = assign14240_e8607_d_n13;

        let (assign14250_e8623, assign14250_e8623_d_n0, assign14250_e8623_d_n2, assign14250_e8623_d_n4, assign14250_e8623_d_n5, assign14250_e8623_d_n6, assign14250_e8623_d_n7, assign14250_e8623_d_n8, assign14250_e8623_d_n9, assign14250_e8623_d_n10, assign14250_e8623_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14250_e8618: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14250_e8620: f64 = (assign14250_e8618 + locals.var_tmf2);
        let assign14250_e8621: f64 = (assign14250_e8620).sqrt();
        (assign14250_e8621, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14250_e8621)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14250_e8621)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14250_e8623;
        locals.var_tmf2_dn0 = assign14250_e8623_d_n0;
        locals.var_tmf2_dn2 = assign14250_e8623_d_n2;
        locals.var_tmf2_dn4 = assign14250_e8623_d_n4;
        locals.var_tmf2_dn5 = assign14250_e8623_d_n5;
        locals.var_tmf2_dn6 = assign14250_e8623_d_n6;
        locals.var_tmf2_dn7 = assign14250_e8623_d_n7;
        locals.var_tmf2_dn8 = assign14250_e8623_d_n8;
        locals.var_tmf2_dn9 = assign14250_e8623_d_n9;
        locals.var_tmf2_dn10 = assign14250_e8623_d_n10;
        locals.var_tmf2_dn13 = assign14250_e8623_d_n13;

        let (assign14260_e8640, assign14260_e8640_d_n0, assign14260_e8640_d_n2, assign14260_e8640_d_n4, assign14260_e8640_d_n5, assign14260_e8640_d_n6, assign14260_e8640_d_n7, assign14260_e8640_d_n8, assign14260_e8640_d_n9, assign14260_e8640_d_n10, assign14260_e8640_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14260_e8636: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14260_e8637: f64 = (1.0 + assign14260_e8636);
        let assign14260_e8638: f64 = (0.5 * assign14260_e8637);
        (assign14260_e8638, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14260_e8640;
        locals.var_t0_dn0 = assign14260_e8640_d_n0;
        locals.var_t0_dn2 = assign14260_e8640_d_n2;
        locals.var_t0_dn4 = assign14260_e8640_d_n4;
        locals.var_t0_dn5 = assign14260_e8640_d_n5;
        locals.var_t0_dn6 = assign14260_e8640_d_n6;
        locals.var_t0_dn7 = assign14260_e8640_d_n7;
        locals.var_t0_dn8 = assign14260_e8640_d_n8;
        locals.var_t0_dn9 = assign14260_e8640_d_n9;
        locals.var_t0_dn10 = assign14260_e8640_d_n10;
        locals.var_t0_dn13 = assign14260_e8640_d_n13;

    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14270_e8659, assign14270_e8659_d_n0, assign14270_e8659_d_n2, assign14270_e8659_d_n4, assign14270_e8659_d_n5, assign14270_e8659_d_n6, assign14270_e8659_d_n7, assign14270_e8659_d_n8, assign14270_e8659_d_n9, assign14270_e8659_d_n10, assign14270_e8659_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign14270_e8651: f64 = (0.005 * locals.var_uc_rd);
        let assign14270_e8655: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14270_e8656: f64 = (0.5 * assign14270_e8655);
        let assign14270_e8657: f64 = (assign14270_e8651 + assign14270_e8656);
        (assign14270_e8657, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14270_e8659;
        locals.var_rde_dn0 = assign14270_e8659_d_n0;
        locals.var_rde_dn2 = assign14270_e8659_d_n2;
        locals.var_rde_dn4 = assign14270_e8659_d_n4;
        locals.var_rde_dn5 = assign14270_e8659_d_n5;
        locals.var_rde_dn6 = assign14270_e8659_d_n6;
        locals.var_rde_dn7 = assign14270_e8659_d_n7;
        locals.var_rde_dn8 = assign14270_e8659_d_n8;
        locals.var_rde_dn9 = assign14270_e8659_d_n9;
        locals.var_rde_dn10 = assign14270_e8659_d_n10;
        locals.var_rde_dn13 = assign14270_e8659_d_n13;

        let (assign14280_e8668, assign14280_e8668_d_n0, assign14280_e8668_d_n2, assign14280_e8668_d_n4, assign14280_e8668_d_n5, assign14280_e8668_d_n6, assign14280_e8668_d_n7, assign14280_e8668_d_n8, assign14280_e8668_d_n9, assign14280_e8668_d_n10, assign14280_e8668_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign14280_e8668;
        locals.var_rde_dn0 = assign14280_e8668_d_n0;
        locals.var_rde_dn2 = assign14280_e8668_d_n2;
        locals.var_rde_dn4 = assign14280_e8668_d_n4;
        locals.var_rde_dn5 = assign14280_e8668_d_n5;
        locals.var_rde_dn6 = assign14280_e8668_d_n6;
        locals.var_rde_dn7 = assign14280_e8668_d_n7;
        locals.var_rde_dn8 = assign14280_e8668_d_n8;
        locals.var_rde_dn9 = assign14280_e8668_d_n9;
        locals.var_rde_dn10 = assign14280_e8668_d_n10;
        locals.var_rde_dn13 = assign14280_e8668_d_n13;

        let assign14290_e8671: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign14290_e8671;

        let (assign14300_e8695, assign14300_e8695_d_n0, assign14300_e8695_d_n2, assign14300_e8695_d_n4, assign14300_e8695_d_n5, assign14300_e8695_d_n6, assign14300_e8695_d_n7, assign14300_e8695_d_n8, assign14300_e8695_d_n9, assign14300_e8695_d_n10, assign14300_e8695_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) {
        let assign14300_e8680: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign14300_e8682: f64 = (assign14300_e8680 * 1000000.0);
        let assign14300_e8684: f64 = (assign14300_e8682 + locals.var_uc_rdict1);
        let assign14300_e8685: f64 = (locals.var_rdtemp0 * assign14300_e8684);
        let assign14300_e8688: f64 = (p.p70 * p.p100);
        let assign14300_e8690: f64 = (assign14300_e8688 * 1000000.0);
        let assign14300_e8692: f64 = (assign14300_e8690 + p.p101);
        let assign14300_e8693: f64 = (assign14300_e8685 * assign14300_e8692);
        (assign14300_e8693, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14300_e8695;
        locals.var_t2_dn0 = assign14300_e8695_d_n0;
        locals.var_t2_dn2 = assign14300_e8695_d_n2;
        locals.var_t2_dn4 = assign14300_e8695_d_n4;
        locals.var_t2_dn5 = assign14300_e8695_d_n5;
        locals.var_t2_dn6 = assign14300_e8695_d_n6;
        locals.var_t2_dn7 = assign14300_e8695_d_n7;
        locals.var_t2_dn8 = assign14300_e8695_d_n8;
        locals.var_t2_dn9 = assign14300_e8695_d_n9;
        locals.var_t2_dn10 = assign14300_e8695_d_n10;
        locals.var_t2_dn13 = assign14300_e8695_d_n13;

        let assign14310_e8698: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign14310_e8698;

        let (assign14320_e8718, assign14320_e8718_d_n0, assign14320_e8718_d_n2, assign14320_e8718_d_n4, assign14320_e8718_d_n5, assign14320_e8718_d_n6, assign14320_e8718_d_n7, assign14320_e8718_d_n8, assign14320_e8718_d_n9, assign14320_e8718_d_n10, assign14320_e8718_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14320_e8709: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign14320_e8710: f64 = (locals.var_uc_rs + assign14320_e8709);
        let assign14320_e8713: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign14320_e8714: f64 = (assign14320_e8710 + assign14320_e8713);
        let assign14320_e8716: f64 = (assign14320_e8714 * locals.var_t2);
        (assign14320_e8716, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign14320_e8714 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14320_e8718;
        locals.var_rse_dn0 = assign14320_e8718_d_n0;
        locals.var_rse_dn2 = assign14320_e8718_d_n2;
        locals.var_rse_dn4 = assign14320_e8718_d_n4;
        locals.var_rse_dn5 = assign14320_e8718_d_n5;
        locals.var_rse_dn6 = assign14320_e8718_d_n6;
        locals.var_rse_dn7 = assign14320_e8718_d_n7;
        locals.var_rse_dn8 = assign14320_e8718_d_n8;
        locals.var_rse_dn9 = assign14320_e8718_d_n9;
        locals.var_rse_dn10 = assign14320_e8718_d_n10;
        locals.var_rse_dn13 = assign14320_e8718_d_n13;

        let (assign14330_e8736, assign14330_e8736_d_n0, assign14330_e8736_d_n2, assign14330_e8736_d_n4, assign14330_e8736_d_n5, assign14330_e8736_d_n6, assign14330_e8736_d_n7, assign14330_e8736_d_n8, assign14330_e8736_d_n9, assign14330_e8736_d_n10, assign14330_e8736_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14330_e8729: f64 = (0.005 * locals.var_uc_rs);
        let assign14330_e8730: f64 = (locals.var_rse - assign14330_e8729);
        let assign14330_e8733: f64 = (0.01 * locals.var_uc_rs);
        let assign14330_e8734: f64 = (assign14330_e8730 - assign14330_e8733);
        (assign14330_e8734, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14330_e8736;
        locals.var_tmf1_dn0 = assign14330_e8736_d_n0;
        locals.var_tmf1_dn2 = assign14330_e8736_d_n2;
        locals.var_tmf1_dn4 = assign14330_e8736_d_n4;
        locals.var_tmf1_dn5 = assign14330_e8736_d_n5;
        locals.var_tmf1_dn6 = assign14330_e8736_d_n6;
        locals.var_tmf1_dn7 = assign14330_e8736_d_n7;
        locals.var_tmf1_dn8 = assign14330_e8736_d_n8;
        locals.var_tmf1_dn9 = assign14330_e8736_d_n9;
        locals.var_tmf1_dn10 = assign14330_e8736_d_n10;
        locals.var_tmf1_dn13 = assign14330_e8736_d_n13;

        let (assign14340_e8754, assign14340_e8754_d_n0, assign14340_e8754_d_n2, assign14340_e8754_d_n4, assign14340_e8754_d_n5, assign14340_e8754_d_n6, assign14340_e8754_d_n7, assign14340_e8754_d_n8, assign14340_e8754_d_n9, assign14340_e8754_d_n10, assign14340_e8754_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14340_e8747: f64 = (0.005 * locals.var_uc_rs);
        let assign14340_e8748: f64 = (4.0 * assign14340_e8747);
        let assign14340_e8751: f64 = (0.01 * locals.var_uc_rs);
        let assign14340_e8752: f64 = (assign14340_e8748 * assign14340_e8751);
        (assign14340_e8752, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14340_e8754;
        locals.var_tmf2_dn0 = assign14340_e8754_d_n0;
        locals.var_tmf2_dn2 = assign14340_e8754_d_n2;
        locals.var_tmf2_dn4 = assign14340_e8754_d_n4;
        locals.var_tmf2_dn5 = assign14340_e8754_d_n5;
        locals.var_tmf2_dn6 = assign14340_e8754_d_n6;
        locals.var_tmf2_dn7 = assign14340_e8754_d_n7;
        locals.var_tmf2_dn8 = assign14340_e8754_d_n8;
        locals.var_tmf2_dn9 = assign14340_e8754_d_n9;
        locals.var_tmf2_dn10 = assign14340_e8754_d_n10;
        locals.var_tmf2_dn13 = assign14340_e8754_d_n13;

        let (assign14350_e8770, assign14350_e8770_d_n0, assign14350_e8770_d_n2, assign14350_e8770_d_n4, assign14350_e8770_d_n5, assign14350_e8770_d_n6, assign14350_e8770_d_n7, assign14350_e8770_d_n8, assign14350_e8770_d_n9, assign14350_e8770_d_n10, assign14350_e8770_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let (assign14350_e8768, assign14350_e8768_d_n0, assign14350_e8768_d_n2, assign14350_e8768_d_n4, assign14350_e8768_d_n5, assign14350_e8768_d_n6, assign14350_e8768_d_n7, assign14350_e8768_d_n8, assign14350_e8768_d_n9, assign14350_e8768_d_n10, assign14350_e8768_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14350_e8767: f64 = (-locals.var_tmf2);
                (assign14350_e8767, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14350_e8768, assign14350_e8768_d_n0, assign14350_e8768_d_n2, assign14350_e8768_d_n4, assign14350_e8768_d_n5, assign14350_e8768_d_n6, assign14350_e8768_d_n7, assign14350_e8768_d_n8, assign14350_e8768_d_n9, assign14350_e8768_d_n10, assign14350_e8768_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14350_e8770;
        locals.var_tmf2_dn0 = assign14350_e8770_d_n0;
        locals.var_tmf2_dn2 = assign14350_e8770_d_n2;
        locals.var_tmf2_dn4 = assign14350_e8770_d_n4;
        locals.var_tmf2_dn5 = assign14350_e8770_d_n5;
        locals.var_tmf2_dn6 = assign14350_e8770_d_n6;
        locals.var_tmf2_dn7 = assign14350_e8770_d_n7;
        locals.var_tmf2_dn8 = assign14350_e8770_d_n8;
        locals.var_tmf2_dn9 = assign14350_e8770_d_n9;
        locals.var_tmf2_dn10 = assign14350_e8770_d_n10;
        locals.var_tmf2_dn13 = assign14350_e8770_d_n13;

        let (assign14360_e8785, assign14360_e8785_d_n0, assign14360_e8785_d_n2, assign14360_e8785_d_n4, assign14360_e8785_d_n5, assign14360_e8785_d_n6, assign14360_e8785_d_n7, assign14360_e8785_d_n8, assign14360_e8785_d_n9, assign14360_e8785_d_n10, assign14360_e8785_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14360_e8780: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14360_e8782: f64 = (assign14360_e8780 + locals.var_tmf2);
        let assign14360_e8783: f64 = (assign14360_e8782).sqrt();
        (assign14360_e8783, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14360_e8783)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14360_e8783)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14360_e8785;
        locals.var_tmf2_dn0 = assign14360_e8785_d_n0;
        locals.var_tmf2_dn2 = assign14360_e8785_d_n2;
        locals.var_tmf2_dn4 = assign14360_e8785_d_n4;
        locals.var_tmf2_dn5 = assign14360_e8785_d_n5;
        locals.var_tmf2_dn6 = assign14360_e8785_d_n6;
        locals.var_tmf2_dn7 = assign14360_e8785_d_n7;
        locals.var_tmf2_dn8 = assign14360_e8785_d_n8;
        locals.var_tmf2_dn9 = assign14360_e8785_d_n9;
        locals.var_tmf2_dn10 = assign14360_e8785_d_n10;
        locals.var_tmf2_dn13 = assign14360_e8785_d_n13;

        let (assign14370_e8801, assign14370_e8801_d_n0, assign14370_e8801_d_n2, assign14370_e8801_d_n4, assign14370_e8801_d_n5, assign14370_e8801_d_n6, assign14370_e8801_d_n7, assign14370_e8801_d_n8, assign14370_e8801_d_n9, assign14370_e8801_d_n10, assign14370_e8801_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14370_e8797: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14370_e8798: f64 = (1.0 + assign14370_e8797);
        let assign14370_e8799: f64 = (0.5 * assign14370_e8798);
        (assign14370_e8799, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14370_e8801;
        locals.var_t0_dn0 = assign14370_e8801_d_n0;
        locals.var_t0_dn2 = assign14370_e8801_d_n2;
        locals.var_t0_dn4 = assign14370_e8801_d_n4;
        locals.var_t0_dn5 = assign14370_e8801_d_n5;
        locals.var_t0_dn6 = assign14370_e8801_d_n6;
        locals.var_t0_dn7 = assign14370_e8801_d_n7;
        locals.var_t0_dn8 = assign14370_e8801_d_n8;
        locals.var_t0_dn9 = assign14370_e8801_d_n9;
        locals.var_t0_dn10 = assign14370_e8801_d_n10;
        locals.var_t0_dn13 = assign14370_e8801_d_n13;

        let (assign14380_e8819, assign14380_e8819_d_n0, assign14380_e8819_d_n2, assign14380_e8819_d_n4, assign14380_e8819_d_n5, assign14380_e8819_d_n6, assign14380_e8819_d_n7, assign14380_e8819_d_n8, assign14380_e8819_d_n9, assign14380_e8819_d_n10, assign14380_e8819_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14380_e8811: f64 = (0.005 * locals.var_uc_rs);
        let assign14380_e8815: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14380_e8816: f64 = (0.5 * assign14380_e8815);
        let assign14380_e8817: f64 = (assign14380_e8811 + assign14380_e8816);
        (assign14380_e8817, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14380_e8819;
        locals.var_rse_dn0 = assign14380_e8819_d_n0;
        locals.var_rse_dn2 = assign14380_e8819_d_n2;
        locals.var_rse_dn4 = assign14380_e8819_d_n4;
        locals.var_rse_dn5 = assign14380_e8819_d_n5;
        locals.var_rse_dn6 = assign14380_e8819_d_n6;
        locals.var_rse_dn7 = assign14380_e8819_d_n7;
        locals.var_rse_dn8 = assign14380_e8819_d_n8;
        locals.var_rse_dn9 = assign14380_e8819_d_n9;
        locals.var_rse_dn10 = assign14380_e8819_d_n10;
        locals.var_rse_dn13 = assign14380_e8819_d_n13;

        let (assign14390_e8840, assign14390_e8840_d_n0, assign14390_e8840_d_n2, assign14390_e8840_d_n4, assign14390_e8840_d_n5, assign14390_e8840_d_n6, assign14390_e8840_d_n7, assign14390_e8840_d_n8, assign14390_e8840_d_n9, assign14390_e8840_d_n10, assign14390_e8840_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14390_e8831: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign14390_e8832: f64 = (locals.var_uc_rs + assign14390_e8831);
        let assign14390_e8835: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign14390_e8836: f64 = (assign14390_e8832 + assign14390_e8835);
        let assign14390_e8838: f64 = (assign14390_e8836 * locals.var_t2);
        (assign14390_e8838, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign14390_e8836 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14390_e8840;
        locals.var_rse_dn0 = assign14390_e8840_d_n0;
        locals.var_rse_dn2 = assign14390_e8840_d_n2;
        locals.var_rse_dn4 = assign14390_e8840_d_n4;
        locals.var_rse_dn5 = assign14390_e8840_d_n5;
        locals.var_rse_dn6 = assign14390_e8840_d_n6;
        locals.var_rse_dn7 = assign14390_e8840_d_n7;
        locals.var_rse_dn8 = assign14390_e8840_d_n8;
        locals.var_rse_dn9 = assign14390_e8840_d_n9;
        locals.var_rse_dn10 = assign14390_e8840_d_n10;
        locals.var_rse_dn13 = assign14390_e8840_d_n13;

        let (assign14400_e8859, assign14400_e8859_d_n0, assign14400_e8859_d_n2, assign14400_e8859_d_n4, assign14400_e8859_d_n5, assign14400_e8859_d_n6, assign14400_e8859_d_n7, assign14400_e8859_d_n8, assign14400_e8859_d_n9, assign14400_e8859_d_n10, assign14400_e8859_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14400_e8852: f64 = (0.005 * locals.var_uc_rs);
        let assign14400_e8853: f64 = (locals.var_rse - assign14400_e8852);
        let assign14400_e8856: f64 = (0.01 * locals.var_uc_rs);
        let assign14400_e8857: f64 = (assign14400_e8853 - assign14400_e8856);
        (assign14400_e8857, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14400_e8859;
        locals.var_tmf1_dn0 = assign14400_e8859_d_n0;
        locals.var_tmf1_dn2 = assign14400_e8859_d_n2;
        locals.var_tmf1_dn4 = assign14400_e8859_d_n4;
        locals.var_tmf1_dn5 = assign14400_e8859_d_n5;
        locals.var_tmf1_dn6 = assign14400_e8859_d_n6;
        locals.var_tmf1_dn7 = assign14400_e8859_d_n7;
        locals.var_tmf1_dn8 = assign14400_e8859_d_n8;
        locals.var_tmf1_dn9 = assign14400_e8859_d_n9;
        locals.var_tmf1_dn10 = assign14400_e8859_d_n10;
        locals.var_tmf1_dn13 = assign14400_e8859_d_n13;

        let (assign14410_e8878, assign14410_e8878_d_n0, assign14410_e8878_d_n2, assign14410_e8878_d_n4, assign14410_e8878_d_n5, assign14410_e8878_d_n6, assign14410_e8878_d_n7, assign14410_e8878_d_n8, assign14410_e8878_d_n9, assign14410_e8878_d_n10, assign14410_e8878_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14410_e8871: f64 = (0.005 * locals.var_uc_rs);
        let assign14410_e8872: f64 = (4.0 * assign14410_e8871);
        let assign14410_e8875: f64 = (0.01 * locals.var_uc_rs);
        let assign14410_e8876: f64 = (assign14410_e8872 * assign14410_e8875);
        (assign14410_e8876, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14410_e8878;
        locals.var_tmf2_dn0 = assign14410_e8878_d_n0;
        locals.var_tmf2_dn2 = assign14410_e8878_d_n2;
        locals.var_tmf2_dn4 = assign14410_e8878_d_n4;
        locals.var_tmf2_dn5 = assign14410_e8878_d_n5;
        locals.var_tmf2_dn6 = assign14410_e8878_d_n6;
        locals.var_tmf2_dn7 = assign14410_e8878_d_n7;
        locals.var_tmf2_dn8 = assign14410_e8878_d_n8;
        locals.var_tmf2_dn9 = assign14410_e8878_d_n9;
        locals.var_tmf2_dn10 = assign14410_e8878_d_n10;
        locals.var_tmf2_dn13 = assign14410_e8878_d_n13;

        let (assign14420_e8895, assign14420_e8895_d_n0, assign14420_e8895_d_n2, assign14420_e8895_d_n4, assign14420_e8895_d_n5, assign14420_e8895_d_n6, assign14420_e8895_d_n7, assign14420_e8895_d_n8, assign14420_e8895_d_n9, assign14420_e8895_d_n10, assign14420_e8895_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let (assign14420_e8893, assign14420_e8893_d_n0, assign14420_e8893_d_n2, assign14420_e8893_d_n4, assign14420_e8893_d_n5, assign14420_e8893_d_n6, assign14420_e8893_d_n7, assign14420_e8893_d_n8, assign14420_e8893_d_n9, assign14420_e8893_d_n10, assign14420_e8893_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14420_e8892: f64 = (-locals.var_tmf2);
                (assign14420_e8892, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14420_e8893, assign14420_e8893_d_n0, assign14420_e8893_d_n2, assign14420_e8893_d_n4, assign14420_e8893_d_n5, assign14420_e8893_d_n6, assign14420_e8893_d_n7, assign14420_e8893_d_n8, assign14420_e8893_d_n9, assign14420_e8893_d_n10, assign14420_e8893_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14420_e8895;
        locals.var_tmf2_dn0 = assign14420_e8895_d_n0;
        locals.var_tmf2_dn2 = assign14420_e8895_d_n2;
        locals.var_tmf2_dn4 = assign14420_e8895_d_n4;
        locals.var_tmf2_dn5 = assign14420_e8895_d_n5;
        locals.var_tmf2_dn6 = assign14420_e8895_d_n6;
        locals.var_tmf2_dn7 = assign14420_e8895_d_n7;
        locals.var_tmf2_dn8 = assign14420_e8895_d_n8;
        locals.var_tmf2_dn9 = assign14420_e8895_d_n9;
        locals.var_tmf2_dn10 = assign14420_e8895_d_n10;
        locals.var_tmf2_dn13 = assign14420_e8895_d_n13;

        let (assign14430_e8911, assign14430_e8911_d_n0, assign14430_e8911_d_n2, assign14430_e8911_d_n4, assign14430_e8911_d_n5, assign14430_e8911_d_n6, assign14430_e8911_d_n7, assign14430_e8911_d_n8, assign14430_e8911_d_n9, assign14430_e8911_d_n10, assign14430_e8911_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14430_e8906: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14430_e8908: f64 = (assign14430_e8906 + locals.var_tmf2);
        let assign14430_e8909: f64 = (assign14430_e8908).sqrt();
        (assign14430_e8909, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14430_e8909)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14430_e8909)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14430_e8911;
        locals.var_tmf2_dn0 = assign14430_e8911_d_n0;
        locals.var_tmf2_dn2 = assign14430_e8911_d_n2;
        locals.var_tmf2_dn4 = assign14430_e8911_d_n4;
        locals.var_tmf2_dn5 = assign14430_e8911_d_n5;
        locals.var_tmf2_dn6 = assign14430_e8911_d_n6;
        locals.var_tmf2_dn7 = assign14430_e8911_d_n7;
        locals.var_tmf2_dn8 = assign14430_e8911_d_n8;
        locals.var_tmf2_dn9 = assign14430_e8911_d_n9;
        locals.var_tmf2_dn10 = assign14430_e8911_d_n10;
        locals.var_tmf2_dn13 = assign14430_e8911_d_n13;

        let (assign14440_e8928, assign14440_e8928_d_n0, assign14440_e8928_d_n2, assign14440_e8928_d_n4, assign14440_e8928_d_n5, assign14440_e8928_d_n6, assign14440_e8928_d_n7, assign14440_e8928_d_n8, assign14440_e8928_d_n9, assign14440_e8928_d_n10, assign14440_e8928_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14440_e8924: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14440_e8925: f64 = (1.0 + assign14440_e8924);
        let assign14440_e8926: f64 = (0.5 * assign14440_e8925);
        (assign14440_e8926, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14440_e8928;
        locals.var_t0_dn0 = assign14440_e8928_d_n0;
        locals.var_t0_dn2 = assign14440_e8928_d_n2;
        locals.var_t0_dn4 = assign14440_e8928_d_n4;
        locals.var_t0_dn5 = assign14440_e8928_d_n5;
        locals.var_t0_dn6 = assign14440_e8928_d_n6;
        locals.var_t0_dn7 = assign14440_e8928_d_n7;
        locals.var_t0_dn8 = assign14440_e8928_d_n8;
        locals.var_t0_dn9 = assign14440_e8928_d_n9;
        locals.var_t0_dn10 = assign14440_e8928_d_n10;
        locals.var_t0_dn13 = assign14440_e8928_d_n13;

        let (assign14450_e8947, assign14450_e8947_d_n0, assign14450_e8947_d_n2, assign14450_e8947_d_n4, assign14450_e8947_d_n5, assign14450_e8947_d_n6, assign14450_e8947_d_n7, assign14450_e8947_d_n8, assign14450_e8947_d_n9, assign14450_e8947_d_n10, assign14450_e8947_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14450_e8939: f64 = (0.005 * locals.var_uc_rs);
        let assign14450_e8943: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14450_e8944: f64 = (0.5 * assign14450_e8943);
        let assign14450_e8945: f64 = (assign14450_e8939 + assign14450_e8944);
        (assign14450_e8945, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14450_e8947;
        locals.var_rse_dn0 = assign14450_e8947_d_n0;
        locals.var_rse_dn2 = assign14450_e8947_d_n2;
        locals.var_rse_dn4 = assign14450_e8947_d_n4;
        locals.var_rse_dn5 = assign14450_e8947_d_n5;
        locals.var_rse_dn6 = assign14450_e8947_d_n6;
        locals.var_rse_dn7 = assign14450_e8947_d_n7;
        locals.var_rse_dn8 = assign14450_e8947_d_n8;
        locals.var_rse_dn9 = assign14450_e8947_d_n9;
        locals.var_rse_dn10 = assign14450_e8947_d_n10;
        locals.var_rse_dn13 = assign14450_e8947_d_n13;

        let (assign14460_e8956, assign14460_e8956_d_n0, assign14460_e8956_d_n2, assign14460_e8956_d_n4, assign14460_e8956_d_n5, assign14460_e8956_d_n6, assign14460_e8956_d_n7, assign14460_e8956_d_n8, assign14460_e8956_d_n9, assign14460_e8956_d_n10, assign14460_e8956_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard316 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign14460_e8956;
        locals.var_rse_dn0 = assign14460_e8956_d_n0;
        locals.var_rse_dn2 = assign14460_e8956_d_n2;
        locals.var_rse_dn4 = assign14460_e8956_d_n4;
        locals.var_rse_dn5 = assign14460_e8956_d_n5;
        locals.var_rse_dn6 = assign14460_e8956_d_n6;
        locals.var_rse_dn7 = assign14460_e8956_d_n7;
        locals.var_rse_dn8 = assign14460_e8956_d_n8;
        locals.var_rse_dn9 = assign14460_e8956_d_n9;
        locals.var_rse_dn10 = assign14460_e8956_d_n10;
        locals.var_rse_dn13 = assign14460_e8956_d_n13;

        let assign14470_e8959: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign14470_e8959;

        let (assign14480_e8983, assign14480_e8983_d_n0, assign14480_e8983_d_n2, assign14480_e8983_d_n4, assign14480_e8983_d_n5, assign14480_e8983_d_n6, assign14480_e8983_d_n7, assign14480_e8983_d_n8, assign14480_e8983_d_n9, assign14480_e8983_d_n10, assign14480_e8983_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14480_e8968: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign14480_e8970: f64 = (assign14480_e8968 * 1000000.0);
        let assign14480_e8972: f64 = (assign14480_e8970 + locals.var_uc_rdict1);
        let assign14480_e8973: f64 = (locals.var_rdvdtemp0 * assign14480_e8972);
        let assign14480_e8976: f64 = (p.p68 * p.p100);
        let assign14480_e8978: f64 = (assign14480_e8976 * 1000000.0);
        let assign14480_e8980: f64 = (assign14480_e8978 + p.p101);
        let assign14480_e8981: f64 = (assign14480_e8973 * assign14480_e8980);
        (assign14480_e8981, ((locals.var_rdvdtemp0_dn0 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn2 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn4 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn5 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn6 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn7 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn8 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn9 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn10 * assign14480_e8972) * assign14480_e8980), ((locals.var_rdvdtemp0_dn13 * assign14480_e8972) * assign14480_e8980),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign14480_e8983;
        locals.var_t4_dn0 = assign14480_e8983_d_n0;
        locals.var_t4_dn2 = assign14480_e8983_d_n2;
        locals.var_t4_dn4 = assign14480_e8983_d_n4;
        locals.var_t4_dn5 = assign14480_e8983_d_n5;
        locals.var_t4_dn6 = assign14480_e8983_d_n6;
        locals.var_t4_dn7 = assign14480_e8983_d_n7;
        locals.var_t4_dn8 = assign14480_e8983_d_n8;
        locals.var_t4_dn9 = assign14480_e8983_d_n9;
        locals.var_t4_dn10 = assign14480_e8983_d_n10;
        locals.var_t4_dn13 = assign14480_e8983_d_n13;

        let (assign14490_e8997, assign14490_e8997_d_n0, assign14490_e8997_d_n2, assign14490_e8997_d_n4, assign14490_e8997_d_n5, assign14490_e8997_d_n6, assign14490_e8997_d_n7, assign14490_e8997_d_n8, assign14490_e8997_d_n9, assign14490_e8997_d_n10, assign14490_e8997_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14490_e8991: f64 = (1.0 - locals.var_uc_rdov13);
        let assign14490_e8993: f64 = (assign14490_e8991 * p.p63);
        let assign14490_e8995: f64 = (assign14490_e8993 * 1000000.0);
        (assign14490_e8995, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign14490_e8997;
        locals.var_t1_dn0 = assign14490_e8997_d_n0;
        locals.var_t1_dn2 = assign14490_e8997_d_n2;
        locals.var_t1_dn4 = assign14490_e8997_d_n4;
        locals.var_t1_dn5 = assign14490_e8997_d_n5;
        locals.var_t1_dn6 = assign14490_e8997_d_n6;
        locals.var_t1_dn7 = assign14490_e8997_d_n7;
        locals.var_t1_dn8 = assign14490_e8997_d_n8;
        locals.var_t1_dn9 = assign14490_e8997_d_n9;
        locals.var_t1_dn10 = assign14490_e8997_d_n10;
        locals.var_t1_dn13 = assign14490_e8997_d_n13;

        let (assign14500_e9018, assign14500_e9018_d_n0, assign14500_e9018_d_n2, assign14500_e9018_d_n4, assign14500_e9018_d_n5, assign14500_e9018_d_n6, assign14500_e9018_d_n7, assign14500_e9018_d_n8, assign14500_e9018_d_n9, assign14500_e9018_d_n10, assign14500_e9018_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14500_e9005: f64 = (p.p99 * p.p99);
        let assign14500_e9009: f64 = (0.0001 * 0.01);
        let assign14500_e9010: f64 = (4.0 * assign14500_e9009);
        let assign14500_e9013: f64 = (0.0001 * 0.01);
        let assign14500_e9014: f64 = (assign14500_e9010 * assign14500_e9013);
        let assign14500_e9015: f64 = (assign14500_e9005 + assign14500_e9014);
        let assign14500_e9016: f64 = (assign14500_e9015).sqrt();
        (assign14500_e9016, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14500_e9018;
        locals.var_tmf2_dn0 = assign14500_e9018_d_n0;
        locals.var_tmf2_dn2 = assign14500_e9018_d_n2;
        locals.var_tmf2_dn4 = assign14500_e9018_d_n4;
        locals.var_tmf2_dn5 = assign14500_e9018_d_n5;
        locals.var_tmf2_dn6 = assign14500_e9018_d_n6;
        locals.var_tmf2_dn7 = assign14500_e9018_d_n7;
        locals.var_tmf2_dn8 = assign14500_e9018_d_n8;
        locals.var_tmf2_dn9 = assign14500_e9018_d_n9;
        locals.var_tmf2_dn10 = assign14500_e9018_d_n10;
        locals.var_tmf2_dn13 = assign14500_e9018_d_n13;

    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14510_e9032, assign14510_e9032_d_n0, assign14510_e9032_d_n2, assign14510_e9032_d_n4, assign14510_e9032_d_n5, assign14510_e9032_d_n6, assign14510_e9032_d_n7, assign14510_e9032_d_n8, assign14510_e9032_d_n9, assign14510_e9032_d_n10, assign14510_e9032_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14510_e9028: f64 = (p.p99 / locals.var_tmf2);
        let assign14510_e9029: f64 = (1.0 + assign14510_e9028);
        let assign14510_e9030: f64 = (0.5 * assign14510_e9029);
        (assign14510_e9030, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14510_e9032;
        locals.var_t0_dn0 = assign14510_e9032_d_n0;
        locals.var_t0_dn2 = assign14510_e9032_d_n2;
        locals.var_t0_dn4 = assign14510_e9032_d_n4;
        locals.var_t0_dn5 = assign14510_e9032_d_n5;
        locals.var_t0_dn6 = assign14510_e9032_d_n6;
        locals.var_t0_dn7 = assign14510_e9032_d_n7;
        locals.var_t0_dn8 = assign14510_e9032_d_n8;
        locals.var_t0_dn9 = assign14510_e9032_d_n9;
        locals.var_t0_dn10 = assign14510_e9032_d_n10;
        locals.var_t0_dn13 = assign14510_e9032_d_n13;

        let (assign14520_e9044, assign14520_e9044_d_n0, assign14520_e9044_d_n2, assign14520_e9044_d_n4, assign14520_e9044_d_n5, assign14520_e9044_d_n6, assign14520_e9044_d_n7, assign14520_e9044_d_n8, assign14520_e9044_d_n9, assign14520_e9044_d_n10, assign14520_e9044_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14520_e9041: f64 = (p.p99 + locals.var_tmf2);
        let assign14520_e9042: f64 = (0.5 * assign14520_e9041);
        (assign14520_e9042, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14520_e9044;
        locals.var_t2_dn0 = assign14520_e9044_d_n0;
        locals.var_t2_dn2 = assign14520_e9044_d_n2;
        locals.var_t2_dn4 = assign14520_e9044_d_n4;
        locals.var_t2_dn5 = assign14520_e9044_d_n5;
        locals.var_t2_dn6 = assign14520_e9044_d_n6;
        locals.var_t2_dn7 = assign14520_e9044_d_n7;
        locals.var_t2_dn8 = assign14520_e9044_d_n8;
        locals.var_t2_dn9 = assign14520_e9044_d_n9;
        locals.var_t2_dn10 = assign14520_e9044_d_n10;
        locals.var_t2_dn13 = assign14520_e9044_d_n13;

        let assign14530_e9047: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign14530_e9047;

        let (assign14540_e9057, assign14540_e9057_d_n0, assign14540_e9057_d_n2, assign14540_e9057_d_n4, assign14540_e9057_d_n5, assign14540_e9057_d_n6, assign14540_e9057_d_n7, assign14540_e9057_d_n8, assign14540_e9057_d_n9, assign14540_e9057_d_n10, assign14540_e9057_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14540_e9057;
        locals.var_t2_dn0 = assign14540_e9057_d_n0;
        locals.var_t2_dn2 = assign14540_e9057_d_n2;
        locals.var_t2_dn4 = assign14540_e9057_d_n4;
        locals.var_t2_dn5 = assign14540_e9057_d_n5;
        locals.var_t2_dn6 = assign14540_e9057_d_n6;
        locals.var_t2_dn7 = assign14540_e9057_d_n7;
        locals.var_t2_dn8 = assign14540_e9057_d_n8;
        locals.var_t2_dn9 = assign14540_e9057_d_n9;
        locals.var_t2_dn10 = assign14540_e9057_d_n10;
        locals.var_t2_dn13 = assign14540_e9057_d_n13;

        let (assign14550_e9067, assign14550_e9067_d_n0, assign14550_e9067_d_n2, assign14550_e9067_d_n4, assign14550_e9067_d_n5, assign14550_e9067_d_n6, assign14550_e9067_d_n7, assign14550_e9067_d_n8, assign14550_e9067_d_n9, assign14550_e9067_d_n10, assign14550_e9067_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14550_e9067;
        locals.var_t0_dn0 = assign14550_e9067_d_n0;
        locals.var_t0_dn2 = assign14550_e9067_d_n2;
        locals.var_t0_dn4 = assign14550_e9067_d_n4;
        locals.var_t0_dn5 = assign14550_e9067_d_n5;
        locals.var_t0_dn6 = assign14550_e9067_d_n6;
        locals.var_t0_dn7 = assign14550_e9067_d_n7;
        locals.var_t0_dn8 = assign14550_e9067_d_n8;
        locals.var_t0_dn9 = assign14550_e9067_d_n9;
        locals.var_t0_dn10 = assign14550_e9067_d_n10;
        locals.var_t0_dn13 = assign14550_e9067_d_n13;

        let (assign14560_e9078, assign14560_e9078_d_n0, assign14560_e9078_d_n2, assign14560_e9078_d_n4, assign14560_e9078_d_n5, assign14560_e9078_d_n6, assign14560_e9078_d_n7, assign14560_e9078_d_n8, assign14560_e9078_d_n9, assign14560_e9078_d_n10, assign14560_e9078_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14560_e9074: f64 = (-p.p98);
        let assign14560_e9076: f64 = (assign14560_e9074 / locals.var_t2);
        (assign14560_e9076, (-((assign14560_e9074 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign14560_e9074 * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign14560_e9078;
        locals.var_t8_dn0 = assign14560_e9078_d_n0;
        locals.var_t8_dn2 = assign14560_e9078_d_n2;
        locals.var_t8_dn4 = assign14560_e9078_d_n4;
        locals.var_t8_dn5 = assign14560_e9078_d_n5;
        locals.var_t8_dn6 = assign14560_e9078_d_n6;
        locals.var_t8_dn7 = assign14560_e9078_d_n7;
        locals.var_t8_dn8 = assign14560_e9078_d_n8;
        locals.var_t8_dn9 = assign14560_e9078_d_n9;
        locals.var_t8_dn10 = assign14560_e9078_d_n10;
        locals.var_t8_dn13 = assign14560_e9078_d_n13;

        let (assign14570_e9094, assign14570_e9094_d_n0, assign14570_e9094_d_n2, assign14570_e9094_d_n4, assign14570_e9094_d_n5, assign14570_e9094_d_n6, assign14570_e9094_d_n7, assign14570_e9094_d_n8, assign14570_e9094_d_n9, assign14570_e9094_d_n10, assign14570_e9094_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14570_e9086: f64 = (locals.var_t8 * p.p63);
        let assign14570_e9088: f64 = (assign14570_e9086 * 1000000.0);
        let assign14570_e9090: f64 = (assign14570_e9088 + 1.0);
        let assign14570_e9092: f64 = (assign14570_e9090 + p.p98);
        (assign14570_e9092, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn13 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign14570_e9094;
        locals.var_t3_dn0 = assign14570_e9094_d_n0;
        locals.var_t3_dn2 = assign14570_e9094_d_n2;
        locals.var_t3_dn4 = assign14570_e9094_d_n4;
        locals.var_t3_dn5 = assign14570_e9094_d_n5;
        locals.var_t3_dn6 = assign14570_e9094_d_n6;
        locals.var_t3_dn7 = assign14570_e9094_d_n7;
        locals.var_t3_dn8 = assign14570_e9094_d_n8;
        locals.var_t3_dn9 = assign14570_e9094_d_n9;
        locals.var_t3_dn10 = assign14570_e9094_d_n10;
        locals.var_t3_dn13 = assign14570_e9094_d_n13;

        let (assign14580_e9108, assign14580_e9108_d_n0, assign14580_e9108_d_n2, assign14580_e9108_d_n4, assign14580_e9108_d_n5, assign14580_e9108_d_n6, assign14580_e9108_d_n7, assign14580_e9108_d_n8, assign14580_e9108_d_n9, assign14580_e9108_d_n10, assign14580_e9108_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14580_e9102: f64 = (locals.var_t3 * locals.var_t4);
        let assign14580_e9104: f64 = (assign14580_e9102 - locals.var_t4);
        let assign14580_e9106: f64 = (assign14580_e9104 - 0.01);
        (assign14580_e9106, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn13 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn13)) - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14580_e9108;
        locals.var_tmf1_dn0 = assign14580_e9108_d_n0;
        locals.var_tmf1_dn2 = assign14580_e9108_d_n2;
        locals.var_tmf1_dn4 = assign14580_e9108_d_n4;
        locals.var_tmf1_dn5 = assign14580_e9108_d_n5;
        locals.var_tmf1_dn6 = assign14580_e9108_d_n6;
        locals.var_tmf1_dn7 = assign14580_e9108_d_n7;
        locals.var_tmf1_dn8 = assign14580_e9108_d_n8;
        locals.var_tmf1_dn9 = assign14580_e9108_d_n9;
        locals.var_tmf1_dn10 = assign14580_e9108_d_n10;
        locals.var_tmf1_dn13 = assign14580_e9108_d_n13;

        let (assign14590_e9120, assign14590_e9120_d_n0, assign14590_e9120_d_n2, assign14590_e9120_d_n4, assign14590_e9120_d_n5, assign14590_e9120_d_n6, assign14590_e9120_d_n7, assign14590_e9120_d_n8, assign14590_e9120_d_n9, assign14590_e9120_d_n10, assign14590_e9120_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14590_e9116: f64 = (4.0 * locals.var_t4);
        let assign14590_e9118: f64 = (assign14590_e9116 * 0.01);
        (assign14590_e9118, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn13) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14590_e9120;
        locals.var_tmf2_dn0 = assign14590_e9120_d_n0;
        locals.var_tmf2_dn2 = assign14590_e9120_d_n2;
        locals.var_tmf2_dn4 = assign14590_e9120_d_n4;
        locals.var_tmf2_dn5 = assign14590_e9120_d_n5;
        locals.var_tmf2_dn6 = assign14590_e9120_d_n6;
        locals.var_tmf2_dn7 = assign14590_e9120_d_n7;
        locals.var_tmf2_dn8 = assign14590_e9120_d_n8;
        locals.var_tmf2_dn9 = assign14590_e9120_d_n9;
        locals.var_tmf2_dn10 = assign14590_e9120_d_n10;
        locals.var_tmf2_dn13 = assign14590_e9120_d_n13;

        let (assign14600_e9134, assign14600_e9134_d_n0, assign14600_e9134_d_n2, assign14600_e9134_d_n4, assign14600_e9134_d_n5, assign14600_e9134_d_n6, assign14600_e9134_d_n7, assign14600_e9134_d_n8, assign14600_e9134_d_n9, assign14600_e9134_d_n10, assign14600_e9134_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign14600_e9132, assign14600_e9132_d_n0, assign14600_e9132_d_n2, assign14600_e9132_d_n4, assign14600_e9132_d_n5, assign14600_e9132_d_n6, assign14600_e9132_d_n7, assign14600_e9132_d_n8, assign14600_e9132_d_n9, assign14600_e9132_d_n10, assign14600_e9132_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14600_e9131: f64 = (-locals.var_tmf2);
                (assign14600_e9131, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14600_e9132, assign14600_e9132_d_n0, assign14600_e9132_d_n2, assign14600_e9132_d_n4, assign14600_e9132_d_n5, assign14600_e9132_d_n6, assign14600_e9132_d_n7, assign14600_e9132_d_n8, assign14600_e9132_d_n9, assign14600_e9132_d_n10, assign14600_e9132_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14600_e9134;
        locals.var_tmf2_dn0 = assign14600_e9134_d_n0;
        locals.var_tmf2_dn2 = assign14600_e9134_d_n2;
        locals.var_tmf2_dn4 = assign14600_e9134_d_n4;
        locals.var_tmf2_dn5 = assign14600_e9134_d_n5;
        locals.var_tmf2_dn6 = assign14600_e9134_d_n6;
        locals.var_tmf2_dn7 = assign14600_e9134_d_n7;
        locals.var_tmf2_dn8 = assign14600_e9134_d_n8;
        locals.var_tmf2_dn9 = assign14600_e9134_d_n9;
        locals.var_tmf2_dn10 = assign14600_e9134_d_n10;
        locals.var_tmf2_dn13 = assign14600_e9134_d_n13;

        let (assign14610_e9147, assign14610_e9147_d_n0, assign14610_e9147_d_n2, assign14610_e9147_d_n4, assign14610_e9147_d_n5, assign14610_e9147_d_n6, assign14610_e9147_d_n7, assign14610_e9147_d_n8, assign14610_e9147_d_n9, assign14610_e9147_d_n10, assign14610_e9147_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14610_e9142: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14610_e9144: f64 = (assign14610_e9142 + locals.var_tmf2);
        let assign14610_e9145: f64 = (assign14610_e9144).sqrt();
        (assign14610_e9145, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14610_e9145)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14610_e9145)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14610_e9147;
        locals.var_tmf2_dn0 = assign14610_e9147_d_n0;
        locals.var_tmf2_dn2 = assign14610_e9147_d_n2;
        locals.var_tmf2_dn4 = assign14610_e9147_d_n4;
        locals.var_tmf2_dn5 = assign14610_e9147_d_n5;
        locals.var_tmf2_dn6 = assign14610_e9147_d_n6;
        locals.var_tmf2_dn7 = assign14610_e9147_d_n7;
        locals.var_tmf2_dn8 = assign14610_e9147_d_n8;
        locals.var_tmf2_dn9 = assign14610_e9147_d_n9;
        locals.var_tmf2_dn10 = assign14610_e9147_d_n10;
        locals.var_tmf2_dn13 = assign14610_e9147_d_n13;

        let (assign14620_e9161, assign14620_e9161_d_n0, assign14620_e9161_d_n2, assign14620_e9161_d_n4, assign14620_e9161_d_n5, assign14620_e9161_d_n6, assign14620_e9161_d_n7, assign14620_e9161_d_n8, assign14620_e9161_d_n9, assign14620_e9161_d_n10, assign14620_e9161_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14620_e9157: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14620_e9158: f64 = (1.0 + assign14620_e9157);
        let assign14620_e9159: f64 = (0.5 * assign14620_e9158);
        (assign14620_e9159, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign14620_e9161;
        locals.var_t6_dn0 = assign14620_e9161_d_n0;
        locals.var_t6_dn2 = assign14620_e9161_d_n2;
        locals.var_t6_dn4 = assign14620_e9161_d_n4;
        locals.var_t6_dn5 = assign14620_e9161_d_n5;
        locals.var_t6_dn6 = assign14620_e9161_d_n6;
        locals.var_t6_dn7 = assign14620_e9161_d_n7;
        locals.var_t6_dn8 = assign14620_e9161_d_n8;
        locals.var_t6_dn9 = assign14620_e9161_d_n9;
        locals.var_t6_dn10 = assign14620_e9161_d_n10;
        locals.var_t6_dn13 = assign14620_e9161_d_n13;

        let (assign14630_e9175, assign14630_e9175_d_n0, assign14630_e9175_d_n2, assign14630_e9175_d_n4, assign14630_e9175_d_n5, assign14630_e9175_d_n6, assign14630_e9175_d_n7, assign14630_e9175_d_n8, assign14630_e9175_d_n9, assign14630_e9175_d_n10, assign14630_e9175_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14630_e9171: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14630_e9172: f64 = (0.5 * assign14630_e9171);
        let assign14630_e9173: f64 = (locals.var_t4 + assign14630_e9172);
        (assign14630_e9173, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign14630_e9175;
        locals.var_t5_dn0 = assign14630_e9175_d_n0;
        locals.var_t5_dn2 = assign14630_e9175_d_n2;
        locals.var_t5_dn4 = assign14630_e9175_d_n4;
        locals.var_t5_dn5 = assign14630_e9175_d_n5;
        locals.var_t5_dn6 = assign14630_e9175_d_n6;
        locals.var_t5_dn7 = assign14630_e9175_d_n7;
        locals.var_t5_dn8 = assign14630_e9175_d_n8;
        locals.var_t5_dn9 = assign14630_e9175_d_n9;
        locals.var_t5_dn10 = assign14630_e9175_d_n10;
        locals.var_t5_dn13 = assign14630_e9175_d_n13;

        let (assign14640_e9191, assign14640_e9191_d_n0, assign14640_e9191_d_n2, assign14640_e9191_d_n4, assign14640_e9191_d_n5, assign14640_e9191_d_n6, assign14640_e9191_d_n7, assign14640_e9191_d_n8, assign14640_e9191_d_n9, assign14640_e9191_d_n10, assign14640_e9191_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14640_e9184: f64 = (p.p98 + 1.0);
        let assign14640_e9185: f64 = (locals.var_t4 * assign14640_e9184);
        let assign14640_e9187: f64 = (assign14640_e9185 - locals.var_t5);
        let assign14640_e9189: f64 = (assign14640_e9187 - 5e-5);
        (assign14640_e9189, ((locals.var_t4_dn0 * assign14640_e9184) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign14640_e9184) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign14640_e9184) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign14640_e9184) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign14640_e9184) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign14640_e9184) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign14640_e9184) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign14640_e9184) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign14640_e9184) - locals.var_t5_dn10), ((locals.var_t4_dn13 * assign14640_e9184) - locals.var_t5_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14640_e9191;
        locals.var_tmf1_dn0 = assign14640_e9191_d_n0;
        locals.var_tmf1_dn2 = assign14640_e9191_d_n2;
        locals.var_tmf1_dn4 = assign14640_e9191_d_n4;
        locals.var_tmf1_dn5 = assign14640_e9191_d_n5;
        locals.var_tmf1_dn6 = assign14640_e9191_d_n6;
        locals.var_tmf1_dn7 = assign14640_e9191_d_n7;
        locals.var_tmf1_dn8 = assign14640_e9191_d_n8;
        locals.var_tmf1_dn9 = assign14640_e9191_d_n9;
        locals.var_tmf1_dn10 = assign14640_e9191_d_n10;
        locals.var_tmf1_dn13 = assign14640_e9191_d_n13;

        let (assign14650_e9207, assign14650_e9207_d_n0, assign14650_e9207_d_n2, assign14650_e9207_d_n4, assign14650_e9207_d_n5, assign14650_e9207_d_n6, assign14650_e9207_d_n7, assign14650_e9207_d_n8, assign14650_e9207_d_n9, assign14650_e9207_d_n10, assign14650_e9207_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14650_e9201: f64 = (p.p98 + 1.0);
        let assign14650_e9202: f64 = (locals.var_t4 * assign14650_e9201);
        let assign14650_e9203: f64 = (4.0 * assign14650_e9202);
        let assign14650_e9205: f64 = (assign14650_e9203 * 5e-5);
        (assign14650_e9205, ((4.0 * (locals.var_t4_dn0 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign14650_e9201)) * 5e-5), ((4.0 * (locals.var_t4_dn13 * assign14650_e9201)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14650_e9207;
        locals.var_tmf2_dn0 = assign14650_e9207_d_n0;
        locals.var_tmf2_dn2 = assign14650_e9207_d_n2;
        locals.var_tmf2_dn4 = assign14650_e9207_d_n4;
        locals.var_tmf2_dn5 = assign14650_e9207_d_n5;
        locals.var_tmf2_dn6 = assign14650_e9207_d_n6;
        locals.var_tmf2_dn7 = assign14650_e9207_d_n7;
        locals.var_tmf2_dn8 = assign14650_e9207_d_n8;
        locals.var_tmf2_dn9 = assign14650_e9207_d_n9;
        locals.var_tmf2_dn10 = assign14650_e9207_d_n10;
        locals.var_tmf2_dn13 = assign14650_e9207_d_n13;

        let (assign14660_e9221, assign14660_e9221_d_n0, assign14660_e9221_d_n2, assign14660_e9221_d_n4, assign14660_e9221_d_n5, assign14660_e9221_d_n6, assign14660_e9221_d_n7, assign14660_e9221_d_n8, assign14660_e9221_d_n9, assign14660_e9221_d_n10, assign14660_e9221_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign14660_e9219, assign14660_e9219_d_n0, assign14660_e9219_d_n2, assign14660_e9219_d_n4, assign14660_e9219_d_n5, assign14660_e9219_d_n6, assign14660_e9219_d_n7, assign14660_e9219_d_n8, assign14660_e9219_d_n9, assign14660_e9219_d_n10, assign14660_e9219_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14660_e9218: f64 = (-locals.var_tmf2);
                (assign14660_e9218, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14660_e9219, assign14660_e9219_d_n0, assign14660_e9219_d_n2, assign14660_e9219_d_n4, assign14660_e9219_d_n5, assign14660_e9219_d_n6, assign14660_e9219_d_n7, assign14660_e9219_d_n8, assign14660_e9219_d_n9, assign14660_e9219_d_n10, assign14660_e9219_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14660_e9221;
        locals.var_tmf2_dn0 = assign14660_e9221_d_n0;
        locals.var_tmf2_dn2 = assign14660_e9221_d_n2;
        locals.var_tmf2_dn4 = assign14660_e9221_d_n4;
        locals.var_tmf2_dn5 = assign14660_e9221_d_n5;
        locals.var_tmf2_dn6 = assign14660_e9221_d_n6;
        locals.var_tmf2_dn7 = assign14660_e9221_d_n7;
        locals.var_tmf2_dn8 = assign14660_e9221_d_n8;
        locals.var_tmf2_dn9 = assign14660_e9221_d_n9;
        locals.var_tmf2_dn10 = assign14660_e9221_d_n10;
        locals.var_tmf2_dn13 = assign14660_e9221_d_n13;

        let (assign14670_e9234, assign14670_e9234_d_n0, assign14670_e9234_d_n2, assign14670_e9234_d_n4, assign14670_e9234_d_n5, assign14670_e9234_d_n6, assign14670_e9234_d_n7, assign14670_e9234_d_n8, assign14670_e9234_d_n9, assign14670_e9234_d_n10, assign14670_e9234_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14670_e9229: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14670_e9231: f64 = (assign14670_e9229 + locals.var_tmf2);
        let assign14670_e9232: f64 = (assign14670_e9231).sqrt();
        (assign14670_e9232, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14670_e9232)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14670_e9232)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14670_e9234;
        locals.var_tmf2_dn0 = assign14670_e9234_d_n0;
        locals.var_tmf2_dn2 = assign14670_e9234_d_n2;
        locals.var_tmf2_dn4 = assign14670_e9234_d_n4;
        locals.var_tmf2_dn5 = assign14670_e9234_d_n5;
        locals.var_tmf2_dn6 = assign14670_e9234_d_n6;
        locals.var_tmf2_dn7 = assign14670_e9234_d_n7;
        locals.var_tmf2_dn8 = assign14670_e9234_d_n8;
        locals.var_tmf2_dn9 = assign14670_e9234_d_n9;
        locals.var_tmf2_dn10 = assign14670_e9234_d_n10;
        locals.var_tmf2_dn13 = assign14670_e9234_d_n13;

        let (assign14680_e9248, assign14680_e9248_d_n0, assign14680_e9248_d_n2, assign14680_e9248_d_n4, assign14680_e9248_d_n5, assign14680_e9248_d_n6, assign14680_e9248_d_n7, assign14680_e9248_d_n8, assign14680_e9248_d_n9, assign14680_e9248_d_n10, assign14680_e9248_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14680_e9244: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14680_e9245: f64 = (1.0 + assign14680_e9244);
        let assign14680_e9246: f64 = (0.5 * assign14680_e9245);
        (assign14680_e9246, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign14680_e9248;
        locals.var_t6_dn0 = assign14680_e9248_d_n0;
        locals.var_t6_dn2 = assign14680_e9248_d_n2;
        locals.var_t6_dn4 = assign14680_e9248_d_n4;
        locals.var_t6_dn5 = assign14680_e9248_d_n5;
        locals.var_t6_dn6 = assign14680_e9248_d_n6;
        locals.var_t6_dn7 = assign14680_e9248_d_n7;
        locals.var_t6_dn8 = assign14680_e9248_d_n8;
        locals.var_t6_dn9 = assign14680_e9248_d_n9;
        locals.var_t6_dn10 = assign14680_e9248_d_n10;
        locals.var_t6_dn13 = assign14680_e9248_d_n13;

        let (assign14690_e9266, assign14690_e9266_d_n0, assign14690_e9266_d_n2, assign14690_e9266_d_n4, assign14690_e9266_d_n5, assign14690_e9266_d_n6, assign14690_e9266_d_n7, assign14690_e9266_d_n8, assign14690_e9266_d_n9, assign14690_e9266_d_n10, assign14690_e9266_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14690_e9257: f64 = (p.p98 + 1.0);
        let assign14690_e9258: f64 = (locals.var_t4 * assign14690_e9257);
        let assign14690_e9262: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14690_e9263: f64 = (0.5 * assign14690_e9262);
        let assign14690_e9264: f64 = (assign14690_e9258 - assign14690_e9263);
        (assign14690_e9264, ((locals.var_t4_dn0 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn13 * assign14690_e9257) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign14690_e9266;
        locals.var_t7_dn0 = assign14690_e9266_d_n0;
        locals.var_t7_dn2 = assign14690_e9266_d_n2;
        locals.var_t7_dn4 = assign14690_e9266_d_n4;
        locals.var_t7_dn5 = assign14690_e9266_d_n5;
        locals.var_t7_dn6 = assign14690_e9266_d_n6;
        locals.var_t7_dn7 = assign14690_e9266_d_n7;
        locals.var_t7_dn8 = assign14690_e9266_d_n8;
        locals.var_t7_dn9 = assign14690_e9266_d_n9;
        locals.var_t7_dn10 = assign14690_e9266_d_n10;
        locals.var_t7_dn13 = assign14690_e9266_d_n13;

        let (assign14700_e9282, assign14700_e9282_d_n0, assign14700_e9282_d_n2, assign14700_e9282_d_n4, assign14700_e9282_d_n5, assign14700_e9282_d_n6, assign14700_e9282_d_n7, assign14700_e9282_d_n8, assign14700_e9282_d_n9, assign14700_e9282_d_n10, assign14700_e9282_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14700_e9275: f64 = (locals.var_t1 * locals.var_t4);
        let assign14700_e9276: f64 = (locals.var_t7 + assign14700_e9275);
        let assign14700_e9278: f64 = assign14700_e9276;
        let assign14700_e9280: f64 = (assign14700_e9278 - 5e-5);
        (assign14700_e9280, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn13 + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14700_e9282;
        locals.var_tmf1_dn0 = assign14700_e9282_d_n0;
        locals.var_tmf1_dn2 = assign14700_e9282_d_n2;
        locals.var_tmf1_dn4 = assign14700_e9282_d_n4;
        locals.var_tmf1_dn5 = assign14700_e9282_d_n5;
        locals.var_tmf1_dn6 = assign14700_e9282_d_n6;
        locals.var_tmf1_dn7 = assign14700_e9282_d_n7;
        locals.var_tmf1_dn8 = assign14700_e9282_d_n8;
        locals.var_tmf1_dn9 = assign14700_e9282_d_n9;
        locals.var_tmf1_dn10 = assign14700_e9282_d_n10;
        locals.var_tmf1_dn13 = assign14700_e9282_d_n13;

        let (assign14710_e9294, assign14710_e9294_d_n0, assign14710_e9294_d_n2, assign14710_e9294_d_n4, assign14710_e9294_d_n5, assign14710_e9294_d_n6, assign14710_e9294_d_n7, assign14710_e9294_d_n8, assign14710_e9294_d_n9, assign14710_e9294_d_n10, assign14710_e9294_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14710_e9294;
        locals.var_tmf2_dn0 = assign14710_e9294_d_n0;
        locals.var_tmf2_dn2 = assign14710_e9294_d_n2;
        locals.var_tmf2_dn4 = assign14710_e9294_d_n4;
        locals.var_tmf2_dn5 = assign14710_e9294_d_n5;
        locals.var_tmf2_dn6 = assign14710_e9294_d_n6;
        locals.var_tmf2_dn7 = assign14710_e9294_d_n7;
        locals.var_tmf2_dn8 = assign14710_e9294_d_n8;
        locals.var_tmf2_dn9 = assign14710_e9294_d_n9;
        locals.var_tmf2_dn10 = assign14710_e9294_d_n10;
        locals.var_tmf2_dn13 = assign14710_e9294_d_n13;

        let (assign14720_e9308, assign14720_e9308_d_n0, assign14720_e9308_d_n2, assign14720_e9308_d_n4, assign14720_e9308_d_n5, assign14720_e9308_d_n6, assign14720_e9308_d_n7, assign14720_e9308_d_n8, assign14720_e9308_d_n9, assign14720_e9308_d_n10, assign14720_e9308_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign14720_e9306, assign14720_e9306_d_n0, assign14720_e9306_d_n2, assign14720_e9306_d_n4, assign14720_e9306_d_n5, assign14720_e9306_d_n6, assign14720_e9306_d_n7, assign14720_e9306_d_n8, assign14720_e9306_d_n9, assign14720_e9306_d_n10, assign14720_e9306_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14720_e9305: f64 = (-locals.var_tmf2);
                (assign14720_e9305, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14720_e9306, assign14720_e9306_d_n0, assign14720_e9306_d_n2, assign14720_e9306_d_n4, assign14720_e9306_d_n5, assign14720_e9306_d_n6, assign14720_e9306_d_n7, assign14720_e9306_d_n8, assign14720_e9306_d_n9, assign14720_e9306_d_n10, assign14720_e9306_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14720_e9308;
        locals.var_tmf2_dn0 = assign14720_e9308_d_n0;
        locals.var_tmf2_dn2 = assign14720_e9308_d_n2;
        locals.var_tmf2_dn4 = assign14720_e9308_d_n4;
        locals.var_tmf2_dn5 = assign14720_e9308_d_n5;
        locals.var_tmf2_dn6 = assign14720_e9308_d_n6;
        locals.var_tmf2_dn7 = assign14720_e9308_d_n7;
        locals.var_tmf2_dn8 = assign14720_e9308_d_n8;
        locals.var_tmf2_dn9 = assign14720_e9308_d_n9;
        locals.var_tmf2_dn10 = assign14720_e9308_d_n10;
        locals.var_tmf2_dn13 = assign14720_e9308_d_n13;

        let (assign14730_e9321, assign14730_e9321_d_n0, assign14730_e9321_d_n2, assign14730_e9321_d_n4, assign14730_e9321_d_n5, assign14730_e9321_d_n6, assign14730_e9321_d_n7, assign14730_e9321_d_n8, assign14730_e9321_d_n9, assign14730_e9321_d_n10, assign14730_e9321_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14730_e9316: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14730_e9318: f64 = (assign14730_e9316 + locals.var_tmf2);
        let assign14730_e9319: f64 = (assign14730_e9318).sqrt();
        (assign14730_e9319, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14730_e9319)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14730_e9319)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14730_e9321;
        locals.var_tmf2_dn0 = assign14730_e9321_d_n0;
        locals.var_tmf2_dn2 = assign14730_e9321_d_n2;
        locals.var_tmf2_dn4 = assign14730_e9321_d_n4;
        locals.var_tmf2_dn5 = assign14730_e9321_d_n5;
        locals.var_tmf2_dn6 = assign14730_e9321_d_n6;
        locals.var_tmf2_dn7 = assign14730_e9321_d_n7;
        locals.var_tmf2_dn8 = assign14730_e9321_d_n8;
        locals.var_tmf2_dn9 = assign14730_e9321_d_n9;
        locals.var_tmf2_dn10 = assign14730_e9321_d_n10;
        locals.var_tmf2_dn13 = assign14730_e9321_d_n13;

    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14740_e9335, assign14740_e9335_d_n0, assign14740_e9335_d_n2, assign14740_e9335_d_n4, assign14740_e9335_d_n5, assign14740_e9335_d_n6, assign14740_e9335_d_n7, assign14740_e9335_d_n8, assign14740_e9335_d_n9, assign14740_e9335_d_n10, assign14740_e9335_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14740_e9331: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14740_e9332: f64 = (1.0 + assign14740_e9331);
        let assign14740_e9333: f64 = (0.5 * assign14740_e9332);
        (assign14740_e9333, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign14740_e9335;
        locals.var_t6_dn0 = assign14740_e9335_d_n0;
        locals.var_t6_dn2 = assign14740_e9335_d_n2;
        locals.var_t6_dn4 = assign14740_e9335_d_n4;
        locals.var_t6_dn5 = assign14740_e9335_d_n5;
        locals.var_t6_dn6 = assign14740_e9335_d_n6;
        locals.var_t6_dn7 = assign14740_e9335_d_n7;
        locals.var_t6_dn8 = assign14740_e9335_d_n8;
        locals.var_t6_dn9 = assign14740_e9335_d_n9;
        locals.var_t6_dn10 = assign14740_e9335_d_n10;
        locals.var_t6_dn13 = assign14740_e9335_d_n13;

        let (assign14750_e9349, assign14750_e9349_d_n0, assign14750_e9349_d_n2, assign14750_e9349_d_n4, assign14750_e9349_d_n5, assign14750_e9349_d_n6, assign14750_e9349_d_n7, assign14750_e9349_d_n8, assign14750_e9349_d_n9, assign14750_e9349_d_n10, assign14750_e9349_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14750_e9345: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14750_e9346: f64 = (0.5 * assign14750_e9345);
        let assign14750_e9347: f64 = assign14750_e9346;
        (assign14750_e9347, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign14750_e9349;
        locals.var_t2_dn0 = assign14750_e9349_d_n0;
        locals.var_t2_dn2 = assign14750_e9349_d_n2;
        locals.var_t2_dn4 = assign14750_e9349_d_n4;
        locals.var_t2_dn5 = assign14750_e9349_d_n5;
        locals.var_t2_dn6 = assign14750_e9349_d_n6;
        locals.var_t2_dn7 = assign14750_e9349_d_n7;
        locals.var_t2_dn8 = assign14750_e9349_d_n8;
        locals.var_t2_dn9 = assign14750_e9349_d_n9;
        locals.var_t2_dn10 = assign14750_e9349_d_n10;
        locals.var_t2_dn13 = assign14750_e9349_d_n13;

        let assign14760_e9356: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard320 = assign14760_e9356;

        let (assign14770_e9376, assign14770_e9376_d_n0, assign14770_e9376_d_n2, assign14770_e9376_d_n4, assign14770_e9376_d_n5, assign14770_e9376_d_n6, assign14770_e9376_d_n7, assign14770_e9376_d_n8, assign14770_e9376_d_n9, assign14770_e9376_d_n10, assign14770_e9376_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14770_e9367: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign14770_e9368: f64 = (locals.var_uc_rdvd + assign14770_e9367);
        let assign14770_e9371: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign14770_e9372: f64 = (assign14770_e9368 + assign14770_e9371);
        let assign14770_e9374: f64 = (assign14770_e9372 * locals.var_t2);
        (assign14770_e9374, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign14770_e9372 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign14770_e9376;
        locals.var_rdvde_dn0 = assign14770_e9376_d_n0;
        locals.var_rdvde_dn2 = assign14770_e9376_d_n2;
        locals.var_rdvde_dn4 = assign14770_e9376_d_n4;
        locals.var_rdvde_dn5 = assign14770_e9376_d_n5;
        locals.var_rdvde_dn6 = assign14770_e9376_d_n6;
        locals.var_rdvde_dn7 = assign14770_e9376_d_n7;
        locals.var_rdvde_dn8 = assign14770_e9376_d_n8;
        locals.var_rdvde_dn9 = assign14770_e9376_d_n9;
        locals.var_rdvde_dn10 = assign14770_e9376_d_n10;
        locals.var_rdvde_dn13 = assign14770_e9376_d_n13;

        let (assign14780_e9394, assign14780_e9394_d_n0, assign14780_e9394_d_n2, assign14780_e9394_d_n4, assign14780_e9394_d_n5, assign14780_e9394_d_n6, assign14780_e9394_d_n7, assign14780_e9394_d_n8, assign14780_e9394_d_n9, assign14780_e9394_d_n10, assign14780_e9394_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14780_e9387: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14780_e9388: f64 = (locals.var_rdvde - assign14780_e9387);
        let assign14780_e9391: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14780_e9392: f64 = (assign14780_e9388 - assign14780_e9391);
        (assign14780_e9392, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14780_e9394;
        locals.var_tmf1_dn0 = assign14780_e9394_d_n0;
        locals.var_tmf1_dn2 = assign14780_e9394_d_n2;
        locals.var_tmf1_dn4 = assign14780_e9394_d_n4;
        locals.var_tmf1_dn5 = assign14780_e9394_d_n5;
        locals.var_tmf1_dn6 = assign14780_e9394_d_n6;
        locals.var_tmf1_dn7 = assign14780_e9394_d_n7;
        locals.var_tmf1_dn8 = assign14780_e9394_d_n8;
        locals.var_tmf1_dn9 = assign14780_e9394_d_n9;
        locals.var_tmf1_dn10 = assign14780_e9394_d_n10;
        locals.var_tmf1_dn13 = assign14780_e9394_d_n13;

        let (assign14790_e9412, assign14790_e9412_d_n0, assign14790_e9412_d_n2, assign14790_e9412_d_n4, assign14790_e9412_d_n5, assign14790_e9412_d_n6, assign14790_e9412_d_n7, assign14790_e9412_d_n8, assign14790_e9412_d_n9, assign14790_e9412_d_n10, assign14790_e9412_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14790_e9405: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14790_e9406: f64 = (4.0 * assign14790_e9405);
        let assign14790_e9409: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14790_e9410: f64 = (assign14790_e9406 * assign14790_e9409);
        (assign14790_e9410, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14790_e9412;
        locals.var_tmf2_dn0 = assign14790_e9412_d_n0;
        locals.var_tmf2_dn2 = assign14790_e9412_d_n2;
        locals.var_tmf2_dn4 = assign14790_e9412_d_n4;
        locals.var_tmf2_dn5 = assign14790_e9412_d_n5;
        locals.var_tmf2_dn6 = assign14790_e9412_d_n6;
        locals.var_tmf2_dn7 = assign14790_e9412_d_n7;
        locals.var_tmf2_dn8 = assign14790_e9412_d_n8;
        locals.var_tmf2_dn9 = assign14790_e9412_d_n9;
        locals.var_tmf2_dn10 = assign14790_e9412_d_n10;
        locals.var_tmf2_dn13 = assign14790_e9412_d_n13;

        let (assign14800_e9428, assign14800_e9428_d_n0, assign14800_e9428_d_n2, assign14800_e9428_d_n4, assign14800_e9428_d_n5, assign14800_e9428_d_n6, assign14800_e9428_d_n7, assign14800_e9428_d_n8, assign14800_e9428_d_n9, assign14800_e9428_d_n10, assign14800_e9428_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14800_e9426, assign14800_e9426_d_n0, assign14800_e9426_d_n2, assign14800_e9426_d_n4, assign14800_e9426_d_n5, assign14800_e9426_d_n6, assign14800_e9426_d_n7, assign14800_e9426_d_n8, assign14800_e9426_d_n9, assign14800_e9426_d_n10, assign14800_e9426_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14800_e9425: f64 = (-locals.var_tmf2);
                (assign14800_e9425, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14800_e9426, assign14800_e9426_d_n0, assign14800_e9426_d_n2, assign14800_e9426_d_n4, assign14800_e9426_d_n5, assign14800_e9426_d_n6, assign14800_e9426_d_n7, assign14800_e9426_d_n8, assign14800_e9426_d_n9, assign14800_e9426_d_n10, assign14800_e9426_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14800_e9428;
        locals.var_tmf2_dn0 = assign14800_e9428_d_n0;
        locals.var_tmf2_dn2 = assign14800_e9428_d_n2;
        locals.var_tmf2_dn4 = assign14800_e9428_d_n4;
        locals.var_tmf2_dn5 = assign14800_e9428_d_n5;
        locals.var_tmf2_dn6 = assign14800_e9428_d_n6;
        locals.var_tmf2_dn7 = assign14800_e9428_d_n7;
        locals.var_tmf2_dn8 = assign14800_e9428_d_n8;
        locals.var_tmf2_dn9 = assign14800_e9428_d_n9;
        locals.var_tmf2_dn10 = assign14800_e9428_d_n10;
        locals.var_tmf2_dn13 = assign14800_e9428_d_n13;

        let (assign14810_e9443, assign14810_e9443_d_n0, assign14810_e9443_d_n2, assign14810_e9443_d_n4, assign14810_e9443_d_n5, assign14810_e9443_d_n6, assign14810_e9443_d_n7, assign14810_e9443_d_n8, assign14810_e9443_d_n9, assign14810_e9443_d_n10, assign14810_e9443_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14810_e9438: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14810_e9440: f64 = (assign14810_e9438 + locals.var_tmf2);
        let assign14810_e9441: f64 = (assign14810_e9440).sqrt();
        (assign14810_e9441, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14810_e9441)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14810_e9441)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14810_e9443;
        locals.var_tmf2_dn0 = assign14810_e9443_d_n0;
        locals.var_tmf2_dn2 = assign14810_e9443_d_n2;
        locals.var_tmf2_dn4 = assign14810_e9443_d_n4;
        locals.var_tmf2_dn5 = assign14810_e9443_d_n5;
        locals.var_tmf2_dn6 = assign14810_e9443_d_n6;
        locals.var_tmf2_dn7 = assign14810_e9443_d_n7;
        locals.var_tmf2_dn8 = assign14810_e9443_d_n8;
        locals.var_tmf2_dn9 = assign14810_e9443_d_n9;
        locals.var_tmf2_dn10 = assign14810_e9443_d_n10;
        locals.var_tmf2_dn13 = assign14810_e9443_d_n13;

        let (assign14820_e9459, assign14820_e9459_d_n0, assign14820_e9459_d_n2, assign14820_e9459_d_n4, assign14820_e9459_d_n5, assign14820_e9459_d_n6, assign14820_e9459_d_n7, assign14820_e9459_d_n8, assign14820_e9459_d_n9, assign14820_e9459_d_n10, assign14820_e9459_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14820_e9455: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14820_e9456: f64 = (1.0 + assign14820_e9455);
        let assign14820_e9457: f64 = (0.5 * assign14820_e9456);
        (assign14820_e9457, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14820_e9459;
        locals.var_t0_dn0 = assign14820_e9459_d_n0;
        locals.var_t0_dn2 = assign14820_e9459_d_n2;
        locals.var_t0_dn4 = assign14820_e9459_d_n4;
        locals.var_t0_dn5 = assign14820_e9459_d_n5;
        locals.var_t0_dn6 = assign14820_e9459_d_n6;
        locals.var_t0_dn7 = assign14820_e9459_d_n7;
        locals.var_t0_dn8 = assign14820_e9459_d_n8;
        locals.var_t0_dn9 = assign14820_e9459_d_n9;
        locals.var_t0_dn10 = assign14820_e9459_d_n10;
        locals.var_t0_dn13 = assign14820_e9459_d_n13;

        let (assign14830_e9477, assign14830_e9477_d_n0, assign14830_e9477_d_n2, assign14830_e9477_d_n4, assign14830_e9477_d_n5, assign14830_e9477_d_n6, assign14830_e9477_d_n7, assign14830_e9477_d_n8, assign14830_e9477_d_n9, assign14830_e9477_d_n10, assign14830_e9477_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14830_e9469: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14830_e9473: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14830_e9474: f64 = (0.5 * assign14830_e9473);
        let assign14830_e9475: f64 = (assign14830_e9469 + assign14830_e9474);
        (assign14830_e9475, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign14830_e9477;
        locals.var_rdvde_dn0 = assign14830_e9477_d_n0;
        locals.var_rdvde_dn2 = assign14830_e9477_d_n2;
        locals.var_rdvde_dn4 = assign14830_e9477_d_n4;
        locals.var_rdvde_dn5 = assign14830_e9477_d_n5;
        locals.var_rdvde_dn6 = assign14830_e9477_d_n6;
        locals.var_rdvde_dn7 = assign14830_e9477_d_n7;
        locals.var_rdvde_dn8 = assign14830_e9477_d_n8;
        locals.var_rdvde_dn9 = assign14830_e9477_d_n9;
        locals.var_rdvde_dn10 = assign14830_e9477_d_n10;
        locals.var_rdvde_dn13 = assign14830_e9477_d_n13;

        let (assign14840_e9498, assign14840_e9498_d_n0, assign14840_e9498_d_n2, assign14840_e9498_d_n4, assign14840_e9498_d_n5, assign14840_e9498_d_n6, assign14840_e9498_d_n7, assign14840_e9498_d_n8, assign14840_e9498_d_n9, assign14840_e9498_d_n10, assign14840_e9498_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14840_e9489: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign14840_e9490: f64 = (locals.var_uc_rdvd + assign14840_e9489);
        let assign14840_e9493: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign14840_e9494: f64 = (assign14840_e9490 + assign14840_e9493);
        let assign14840_e9496: f64 = (assign14840_e9494 * locals.var_t2);
        (assign14840_e9496, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign14840_e9494 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign14840_e9498;
        locals.var_rdvde_dn0 = assign14840_e9498_d_n0;
        locals.var_rdvde_dn2 = assign14840_e9498_d_n2;
        locals.var_rdvde_dn4 = assign14840_e9498_d_n4;
        locals.var_rdvde_dn5 = assign14840_e9498_d_n5;
        locals.var_rdvde_dn6 = assign14840_e9498_d_n6;
        locals.var_rdvde_dn7 = assign14840_e9498_d_n7;
        locals.var_rdvde_dn8 = assign14840_e9498_d_n8;
        locals.var_rdvde_dn9 = assign14840_e9498_d_n9;
        locals.var_rdvde_dn10 = assign14840_e9498_d_n10;
        locals.var_rdvde_dn13 = assign14840_e9498_d_n13;

        let (assign14850_e9517, assign14850_e9517_d_n0, assign14850_e9517_d_n2, assign14850_e9517_d_n4, assign14850_e9517_d_n5, assign14850_e9517_d_n6, assign14850_e9517_d_n7, assign14850_e9517_d_n8, assign14850_e9517_d_n9, assign14850_e9517_d_n10, assign14850_e9517_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14850_e9510: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14850_e9511: f64 = (locals.var_rdvde - assign14850_e9510);
        let assign14850_e9514: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14850_e9515: f64 = (assign14850_e9511 - assign14850_e9514);
        (assign14850_e9515, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14850_e9517;
        locals.var_tmf1_dn0 = assign14850_e9517_d_n0;
        locals.var_tmf1_dn2 = assign14850_e9517_d_n2;
        locals.var_tmf1_dn4 = assign14850_e9517_d_n4;
        locals.var_tmf1_dn5 = assign14850_e9517_d_n5;
        locals.var_tmf1_dn6 = assign14850_e9517_d_n6;
        locals.var_tmf1_dn7 = assign14850_e9517_d_n7;
        locals.var_tmf1_dn8 = assign14850_e9517_d_n8;
        locals.var_tmf1_dn9 = assign14850_e9517_d_n9;
        locals.var_tmf1_dn10 = assign14850_e9517_d_n10;
        locals.var_tmf1_dn13 = assign14850_e9517_d_n13;

        let (assign14860_e9536, assign14860_e9536_d_n0, assign14860_e9536_d_n2, assign14860_e9536_d_n4, assign14860_e9536_d_n5, assign14860_e9536_d_n6, assign14860_e9536_d_n7, assign14860_e9536_d_n8, assign14860_e9536_d_n9, assign14860_e9536_d_n10, assign14860_e9536_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14860_e9529: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14860_e9530: f64 = (4.0 * assign14860_e9529);
        let assign14860_e9533: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14860_e9534: f64 = (assign14860_e9530 * assign14860_e9533);
        (assign14860_e9534, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14860_e9536;
        locals.var_tmf2_dn0 = assign14860_e9536_d_n0;
        locals.var_tmf2_dn2 = assign14860_e9536_d_n2;
        locals.var_tmf2_dn4 = assign14860_e9536_d_n4;
        locals.var_tmf2_dn5 = assign14860_e9536_d_n5;
        locals.var_tmf2_dn6 = assign14860_e9536_d_n6;
        locals.var_tmf2_dn7 = assign14860_e9536_d_n7;
        locals.var_tmf2_dn8 = assign14860_e9536_d_n8;
        locals.var_tmf2_dn9 = assign14860_e9536_d_n9;
        locals.var_tmf2_dn10 = assign14860_e9536_d_n10;
        locals.var_tmf2_dn13 = assign14860_e9536_d_n13;

        let (assign14870_e9553, assign14870_e9553_d_n0, assign14870_e9553_d_n2, assign14870_e9553_d_n4, assign14870_e9553_d_n5, assign14870_e9553_d_n6, assign14870_e9553_d_n7, assign14870_e9553_d_n8, assign14870_e9553_d_n9, assign14870_e9553_d_n10, assign14870_e9553_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let (assign14870_e9551, assign14870_e9551_d_n0, assign14870_e9551_d_n2, assign14870_e9551_d_n4, assign14870_e9551_d_n5, assign14870_e9551_d_n6, assign14870_e9551_d_n7, assign14870_e9551_d_n8, assign14870_e9551_d_n9, assign14870_e9551_d_n10, assign14870_e9551_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14870_e9550: f64 = (-locals.var_tmf2);
                (assign14870_e9550, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14870_e9551, assign14870_e9551_d_n0, assign14870_e9551_d_n2, assign14870_e9551_d_n4, assign14870_e9551_d_n5, assign14870_e9551_d_n6, assign14870_e9551_d_n7, assign14870_e9551_d_n8, assign14870_e9551_d_n9, assign14870_e9551_d_n10, assign14870_e9551_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14870_e9553;
        locals.var_tmf2_dn0 = assign14870_e9553_d_n0;
        locals.var_tmf2_dn2 = assign14870_e9553_d_n2;
        locals.var_tmf2_dn4 = assign14870_e9553_d_n4;
        locals.var_tmf2_dn5 = assign14870_e9553_d_n5;
        locals.var_tmf2_dn6 = assign14870_e9553_d_n6;
        locals.var_tmf2_dn7 = assign14870_e9553_d_n7;
        locals.var_tmf2_dn8 = assign14870_e9553_d_n8;
        locals.var_tmf2_dn9 = assign14870_e9553_d_n9;
        locals.var_tmf2_dn10 = assign14870_e9553_d_n10;
        locals.var_tmf2_dn13 = assign14870_e9553_d_n13;

        let (assign14880_e9569, assign14880_e9569_d_n0, assign14880_e9569_d_n2, assign14880_e9569_d_n4, assign14880_e9569_d_n5, assign14880_e9569_d_n6, assign14880_e9569_d_n7, assign14880_e9569_d_n8, assign14880_e9569_d_n9, assign14880_e9569_d_n10, assign14880_e9569_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14880_e9564: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14880_e9566: f64 = (assign14880_e9564 + locals.var_tmf2);
        let assign14880_e9567: f64 = (assign14880_e9566).sqrt();
        (assign14880_e9567, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14880_e9567)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14880_e9567)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14880_e9569;
        locals.var_tmf2_dn0 = assign14880_e9569_d_n0;
        locals.var_tmf2_dn2 = assign14880_e9569_d_n2;
        locals.var_tmf2_dn4 = assign14880_e9569_d_n4;
        locals.var_tmf2_dn5 = assign14880_e9569_d_n5;
        locals.var_tmf2_dn6 = assign14880_e9569_d_n6;
        locals.var_tmf2_dn7 = assign14880_e9569_d_n7;
        locals.var_tmf2_dn8 = assign14880_e9569_d_n8;
        locals.var_tmf2_dn9 = assign14880_e9569_d_n9;
        locals.var_tmf2_dn10 = assign14880_e9569_d_n10;
        locals.var_tmf2_dn13 = assign14880_e9569_d_n13;

        let (assign14890_e9586, assign14890_e9586_d_n0, assign14890_e9586_d_n2, assign14890_e9586_d_n4, assign14890_e9586_d_n5, assign14890_e9586_d_n6, assign14890_e9586_d_n7, assign14890_e9586_d_n8, assign14890_e9586_d_n9, assign14890_e9586_d_n10, assign14890_e9586_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14890_e9582: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14890_e9583: f64 = (1.0 + assign14890_e9582);
        let assign14890_e9584: f64 = (0.5 * assign14890_e9583);
        (assign14890_e9584, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign14890_e9586;
        locals.var_t0_dn0 = assign14890_e9586_d_n0;
        locals.var_t0_dn2 = assign14890_e9586_d_n2;
        locals.var_t0_dn4 = assign14890_e9586_d_n4;
        locals.var_t0_dn5 = assign14890_e9586_d_n5;
        locals.var_t0_dn6 = assign14890_e9586_d_n6;
        locals.var_t0_dn7 = assign14890_e9586_d_n7;
        locals.var_t0_dn8 = assign14890_e9586_d_n8;
        locals.var_t0_dn9 = assign14890_e9586_d_n9;
        locals.var_t0_dn10 = assign14890_e9586_d_n10;
        locals.var_t0_dn13 = assign14890_e9586_d_n13;

        let (assign14900_e9605, assign14900_e9605_d_n0, assign14900_e9605_d_n2, assign14900_e9605_d_n4, assign14900_e9605_d_n5, assign14900_e9605_d_n6, assign14900_e9605_d_n7, assign14900_e9605_d_n8, assign14900_e9605_d_n9, assign14900_e9605_d_n10, assign14900_e9605_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign14900_e9597: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14900_e9601: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14900_e9602: f64 = (0.5 * assign14900_e9601);
        let assign14900_e9603: f64 = (assign14900_e9597 + assign14900_e9602);
        (assign14900_e9603, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign14900_e9605;
        locals.var_rdvde_dn0 = assign14900_e9605_d_n0;
        locals.var_rdvde_dn2 = assign14900_e9605_d_n2;
        locals.var_rdvde_dn4 = assign14900_e9605_d_n4;
        locals.var_rdvde_dn5 = assign14900_e9605_d_n5;
        locals.var_rdvde_dn6 = assign14900_e9605_d_n6;
        locals.var_rdvde_dn7 = assign14900_e9605_d_n7;
        locals.var_rdvde_dn8 = assign14900_e9605_d_n8;
        locals.var_rdvde_dn9 = assign14900_e9605_d_n9;
        locals.var_rdvde_dn10 = assign14900_e9605_d_n10;
        locals.var_rdvde_dn13 = assign14900_e9605_d_n13;

        let (assign14910_e9629, assign14910_e9629_d_n0, assign14910_e9629_d_n2, assign14910_e9629_d_n4, assign14910_e9629_d_n5, assign14910_e9629_d_n6, assign14910_e9629_d_n7, assign14910_e9629_d_n8, assign14910_e9629_d_n9, assign14910_e9629_d_n10, assign14910_e9629_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14910_e9614: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign14910_e9616: f64 = (assign14910_e9614 * 1000000.0);
        let assign14910_e9618: f64 = (assign14910_e9616 + locals.var_uc_rdict1);
        let assign14910_e9619: f64 = (locals.var_rdvdtemp0 * assign14910_e9618);
        let assign14910_e9622: f64 = (p.p70 * p.p100);
        let assign14910_e9624: f64 = (assign14910_e9622 * 1000000.0);
        let assign14910_e9626: f64 = (assign14910_e9624 + p.p101);
        let assign14910_e9627: f64 = (assign14910_e9619 * assign14910_e9626);
        (assign14910_e9627, ((locals.var_rdvdtemp0_dn0 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn2 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn4 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn5 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn6 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn7 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn8 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn9 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn10 * assign14910_e9618) * assign14910_e9626), ((locals.var_rdvdtemp0_dn13 * assign14910_e9618) * assign14910_e9626),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign14910_e9629;
        locals.var_t4_dn0 = assign14910_e9629_d_n0;
        locals.var_t4_dn2 = assign14910_e9629_d_n2;
        locals.var_t4_dn4 = assign14910_e9629_d_n4;
        locals.var_t4_dn5 = assign14910_e9629_d_n5;
        locals.var_t4_dn6 = assign14910_e9629_d_n6;
        locals.var_t4_dn7 = assign14910_e9629_d_n7;
        locals.var_t4_dn8 = assign14910_e9629_d_n8;
        locals.var_t4_dn9 = assign14910_e9629_d_n9;
        locals.var_t4_dn10 = assign14910_e9629_d_n10;
        locals.var_t4_dn13 = assign14910_e9629_d_n13;

        let (assign14920_e9643, assign14920_e9643_d_n0, assign14920_e9643_d_n2, assign14920_e9643_d_n4, assign14920_e9643_d_n5, assign14920_e9643_d_n6, assign14920_e9643_d_n7, assign14920_e9643_d_n8, assign14920_e9643_d_n9, assign14920_e9643_d_n10, assign14920_e9643_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14920_e9637: f64 = (1.0 - locals.var_uc_rdov13);
        let assign14920_e9639: f64 = (assign14920_e9637 * p.p66);
        let assign14920_e9641: f64 = (assign14920_e9639 * 1000000.0);
        (assign14920_e9641, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign14920_e9643;
        locals.var_t1_dn0 = assign14920_e9643_d_n0;
        locals.var_t1_dn2 = assign14920_e9643_d_n2;
        locals.var_t1_dn4 = assign14920_e9643_d_n4;
        locals.var_t1_dn5 = assign14920_e9643_d_n5;
        locals.var_t1_dn6 = assign14920_e9643_d_n6;
        locals.var_t1_dn7 = assign14920_e9643_d_n7;
        locals.var_t1_dn8 = assign14920_e9643_d_n8;
        locals.var_t1_dn9 = assign14920_e9643_d_n9;
        locals.var_t1_dn10 = assign14920_e9643_d_n10;
        locals.var_t1_dn13 = assign14920_e9643_d_n13;

        let (assign14930_e9659, assign14930_e9659_d_n0, assign14930_e9659_d_n2, assign14930_e9659_d_n4, assign14930_e9659_d_n5, assign14930_e9659_d_n6, assign14930_e9659_d_n7, assign14930_e9659_d_n8, assign14930_e9659_d_n9, assign14930_e9659_d_n10, assign14930_e9659_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14930_e9651: f64 = (locals.var_t8 * p.p66);
        let assign14930_e9653: f64 = (assign14930_e9651 * 1000000.0);
        let assign14930_e9655: f64 = (assign14930_e9653 + 1.0);
        let assign14930_e9657: f64 = (assign14930_e9655 + p.p98);
        (assign14930_e9657, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn13 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign14930_e9659;
        locals.var_t3_dn0 = assign14930_e9659_d_n0;
        locals.var_t3_dn2 = assign14930_e9659_d_n2;
        locals.var_t3_dn4 = assign14930_e9659_d_n4;
        locals.var_t3_dn5 = assign14930_e9659_d_n5;
        locals.var_t3_dn6 = assign14930_e9659_d_n6;
        locals.var_t3_dn7 = assign14930_e9659_d_n7;
        locals.var_t3_dn8 = assign14930_e9659_d_n8;
        locals.var_t3_dn9 = assign14930_e9659_d_n9;
        locals.var_t3_dn10 = assign14930_e9659_d_n10;
        locals.var_t3_dn13 = assign14930_e9659_d_n13;

        let (assign14940_e9673, assign14940_e9673_d_n0, assign14940_e9673_d_n2, assign14940_e9673_d_n4, assign14940_e9673_d_n5, assign14940_e9673_d_n6, assign14940_e9673_d_n7, assign14940_e9673_d_n8, assign14940_e9673_d_n9, assign14940_e9673_d_n10, assign14940_e9673_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14940_e9667: f64 = (locals.var_t3 * locals.var_t4);
        let assign14940_e9669: f64 = (assign14940_e9667 - locals.var_t4);
        let assign14940_e9671: f64 = (assign14940_e9669 - 0.01);
        (assign14940_e9671, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn13 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn13)) - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign14940_e9673;
        locals.var_tmf1_dn0 = assign14940_e9673_d_n0;
        locals.var_tmf1_dn2 = assign14940_e9673_d_n2;
        locals.var_tmf1_dn4 = assign14940_e9673_d_n4;
        locals.var_tmf1_dn5 = assign14940_e9673_d_n5;
        locals.var_tmf1_dn6 = assign14940_e9673_d_n6;
        locals.var_tmf1_dn7 = assign14940_e9673_d_n7;
        locals.var_tmf1_dn8 = assign14940_e9673_d_n8;
        locals.var_tmf1_dn9 = assign14940_e9673_d_n9;
        locals.var_tmf1_dn10 = assign14940_e9673_d_n10;
        locals.var_tmf1_dn13 = assign14940_e9673_d_n13;

        let (assign14950_e9685, assign14950_e9685_d_n0, assign14950_e9685_d_n2, assign14950_e9685_d_n4, assign14950_e9685_d_n5, assign14950_e9685_d_n6, assign14950_e9685_d_n7, assign14950_e9685_d_n8, assign14950_e9685_d_n9, assign14950_e9685_d_n10, assign14950_e9685_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14950_e9681: f64 = (4.0 * locals.var_t4);
        let assign14950_e9683: f64 = (assign14950_e9681 * 0.01);
        (assign14950_e9683, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn13) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14950_e9685;
        locals.var_tmf2_dn0 = assign14950_e9685_d_n0;
        locals.var_tmf2_dn2 = assign14950_e9685_d_n2;
        locals.var_tmf2_dn4 = assign14950_e9685_d_n4;
        locals.var_tmf2_dn5 = assign14950_e9685_d_n5;
        locals.var_tmf2_dn6 = assign14950_e9685_d_n6;
        locals.var_tmf2_dn7 = assign14950_e9685_d_n7;
        locals.var_tmf2_dn8 = assign14950_e9685_d_n8;
        locals.var_tmf2_dn9 = assign14950_e9685_d_n9;
        locals.var_tmf2_dn10 = assign14950_e9685_d_n10;
        locals.var_tmf2_dn13 = assign14950_e9685_d_n13;

    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14960_e9699, assign14960_e9699_d_n0, assign14960_e9699_d_n2, assign14960_e9699_d_n4, assign14960_e9699_d_n5, assign14960_e9699_d_n6, assign14960_e9699_d_n7, assign14960_e9699_d_n8, assign14960_e9699_d_n9, assign14960_e9699_d_n10, assign14960_e9699_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign14960_e9697, assign14960_e9697_d_n0, assign14960_e9697_d_n2, assign14960_e9697_d_n4, assign14960_e9697_d_n5, assign14960_e9697_d_n6, assign14960_e9697_d_n7, assign14960_e9697_d_n8, assign14960_e9697_d_n9, assign14960_e9697_d_n10, assign14960_e9697_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign14960_e9696: f64 = (-locals.var_tmf2);
                (assign14960_e9696, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign14960_e9697, assign14960_e9697_d_n0, assign14960_e9697_d_n2, assign14960_e9697_d_n4, assign14960_e9697_d_n5, assign14960_e9697_d_n6, assign14960_e9697_d_n7, assign14960_e9697_d_n8, assign14960_e9697_d_n9, assign14960_e9697_d_n10, assign14960_e9697_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14960_e9699;
        locals.var_tmf2_dn0 = assign14960_e9699_d_n0;
        locals.var_tmf2_dn2 = assign14960_e9699_d_n2;
        locals.var_tmf2_dn4 = assign14960_e9699_d_n4;
        locals.var_tmf2_dn5 = assign14960_e9699_d_n5;
        locals.var_tmf2_dn6 = assign14960_e9699_d_n6;
        locals.var_tmf2_dn7 = assign14960_e9699_d_n7;
        locals.var_tmf2_dn8 = assign14960_e9699_d_n8;
        locals.var_tmf2_dn9 = assign14960_e9699_d_n9;
        locals.var_tmf2_dn10 = assign14960_e9699_d_n10;
        locals.var_tmf2_dn13 = assign14960_e9699_d_n13;

        let (assign14970_e9712, assign14970_e9712_d_n0, assign14970_e9712_d_n2, assign14970_e9712_d_n4, assign14970_e9712_d_n5, assign14970_e9712_d_n6, assign14970_e9712_d_n7, assign14970_e9712_d_n8, assign14970_e9712_d_n9, assign14970_e9712_d_n10, assign14970_e9712_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14970_e9707: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14970_e9709: f64 = (assign14970_e9707 + locals.var_tmf2);
        let assign14970_e9710: f64 = (assign14970_e9709).sqrt();
        (assign14970_e9710, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14970_e9710)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign14970_e9710)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign14970_e9712;
        locals.var_tmf2_dn0 = assign14970_e9712_d_n0;
        locals.var_tmf2_dn2 = assign14970_e9712_d_n2;
        locals.var_tmf2_dn4 = assign14970_e9712_d_n4;
        locals.var_tmf2_dn5 = assign14970_e9712_d_n5;
        locals.var_tmf2_dn6 = assign14970_e9712_d_n6;
        locals.var_tmf2_dn7 = assign14970_e9712_d_n7;
        locals.var_tmf2_dn8 = assign14970_e9712_d_n8;
        locals.var_tmf2_dn9 = assign14970_e9712_d_n9;
        locals.var_tmf2_dn10 = assign14970_e9712_d_n10;
        locals.var_tmf2_dn13 = assign14970_e9712_d_n13;

        let (assign14980_e9726, assign14980_e9726_d_n0, assign14980_e9726_d_n2, assign14980_e9726_d_n4, assign14980_e9726_d_n5, assign14980_e9726_d_n6, assign14980_e9726_d_n7, assign14980_e9726_d_n8, assign14980_e9726_d_n9, assign14980_e9726_d_n10, assign14980_e9726_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14980_e9722: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14980_e9723: f64 = (1.0 + assign14980_e9722);
        let assign14980_e9724: f64 = (0.5 * assign14980_e9723);
        (assign14980_e9724, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign14980_e9726;
        locals.var_t6_dn0 = assign14980_e9726_d_n0;
        locals.var_t6_dn2 = assign14980_e9726_d_n2;
        locals.var_t6_dn4 = assign14980_e9726_d_n4;
        locals.var_t6_dn5 = assign14980_e9726_d_n5;
        locals.var_t6_dn6 = assign14980_e9726_d_n6;
        locals.var_t6_dn7 = assign14980_e9726_d_n7;
        locals.var_t6_dn8 = assign14980_e9726_d_n8;
        locals.var_t6_dn9 = assign14980_e9726_d_n9;
        locals.var_t6_dn10 = assign14980_e9726_d_n10;
        locals.var_t6_dn13 = assign14980_e9726_d_n13;

        let (assign14990_e9740, assign14990_e9740_d_n0, assign14990_e9740_d_n2, assign14990_e9740_d_n4, assign14990_e9740_d_n5, assign14990_e9740_d_n6, assign14990_e9740_d_n7, assign14990_e9740_d_n8, assign14990_e9740_d_n9, assign14990_e9740_d_n10, assign14990_e9740_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14990_e9736: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14990_e9737: f64 = (0.5 * assign14990_e9736);
        let assign14990_e9738: f64 = (locals.var_t4 + assign14990_e9737);
        (assign14990_e9738, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign14990_e9740;
        locals.var_t5_dn0 = assign14990_e9740_d_n0;
        locals.var_t5_dn2 = assign14990_e9740_d_n2;
        locals.var_t5_dn4 = assign14990_e9740_d_n4;
        locals.var_t5_dn5 = assign14990_e9740_d_n5;
        locals.var_t5_dn6 = assign14990_e9740_d_n6;
        locals.var_t5_dn7 = assign14990_e9740_d_n7;
        locals.var_t5_dn8 = assign14990_e9740_d_n8;
        locals.var_t5_dn9 = assign14990_e9740_d_n9;
        locals.var_t5_dn10 = assign14990_e9740_d_n10;
        locals.var_t5_dn13 = assign14990_e9740_d_n13;

        let (assign15000_e9756, assign15000_e9756_d_n0, assign15000_e9756_d_n2, assign15000_e9756_d_n4, assign15000_e9756_d_n5, assign15000_e9756_d_n6, assign15000_e9756_d_n7, assign15000_e9756_d_n8, assign15000_e9756_d_n9, assign15000_e9756_d_n10, assign15000_e9756_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15000_e9749: f64 = (p.p98 + 1.0);
        let assign15000_e9750: f64 = (locals.var_t4 * assign15000_e9749);
        let assign15000_e9752: f64 = (assign15000_e9750 - locals.var_t5);
        let assign15000_e9754: f64 = (assign15000_e9752 - 5e-5);
        (assign15000_e9754, ((locals.var_t4_dn0 * assign15000_e9749) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign15000_e9749) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign15000_e9749) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign15000_e9749) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign15000_e9749) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign15000_e9749) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign15000_e9749) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign15000_e9749) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign15000_e9749) - locals.var_t5_dn10), ((locals.var_t4_dn13 * assign15000_e9749) - locals.var_t5_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign15000_e9756;
        locals.var_tmf1_dn0 = assign15000_e9756_d_n0;
        locals.var_tmf1_dn2 = assign15000_e9756_d_n2;
        locals.var_tmf1_dn4 = assign15000_e9756_d_n4;
        locals.var_tmf1_dn5 = assign15000_e9756_d_n5;
        locals.var_tmf1_dn6 = assign15000_e9756_d_n6;
        locals.var_tmf1_dn7 = assign15000_e9756_d_n7;
        locals.var_tmf1_dn8 = assign15000_e9756_d_n8;
        locals.var_tmf1_dn9 = assign15000_e9756_d_n9;
        locals.var_tmf1_dn10 = assign15000_e9756_d_n10;
        locals.var_tmf1_dn13 = assign15000_e9756_d_n13;

        let (assign15010_e9772, assign15010_e9772_d_n0, assign15010_e9772_d_n2, assign15010_e9772_d_n4, assign15010_e9772_d_n5, assign15010_e9772_d_n6, assign15010_e9772_d_n7, assign15010_e9772_d_n8, assign15010_e9772_d_n9, assign15010_e9772_d_n10, assign15010_e9772_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15010_e9766: f64 = (p.p98 + 1.0);
        let assign15010_e9767: f64 = (locals.var_t4 * assign15010_e9766);
        let assign15010_e9768: f64 = (4.0 * assign15010_e9767);
        let assign15010_e9770: f64 = (assign15010_e9768 * 5e-5);
        (assign15010_e9770, ((4.0 * (locals.var_t4_dn0 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign15010_e9766)) * 5e-5), ((4.0 * (locals.var_t4_dn13 * assign15010_e9766)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15010_e9772;
        locals.var_tmf2_dn0 = assign15010_e9772_d_n0;
        locals.var_tmf2_dn2 = assign15010_e9772_d_n2;
        locals.var_tmf2_dn4 = assign15010_e9772_d_n4;
        locals.var_tmf2_dn5 = assign15010_e9772_d_n5;
        locals.var_tmf2_dn6 = assign15010_e9772_d_n6;
        locals.var_tmf2_dn7 = assign15010_e9772_d_n7;
        locals.var_tmf2_dn8 = assign15010_e9772_d_n8;
        locals.var_tmf2_dn9 = assign15010_e9772_d_n9;
        locals.var_tmf2_dn10 = assign15010_e9772_d_n10;
        locals.var_tmf2_dn13 = assign15010_e9772_d_n13;

        let (assign15020_e9786, assign15020_e9786_d_n0, assign15020_e9786_d_n2, assign15020_e9786_d_n4, assign15020_e9786_d_n5, assign15020_e9786_d_n6, assign15020_e9786_d_n7, assign15020_e9786_d_n8, assign15020_e9786_d_n9, assign15020_e9786_d_n10, assign15020_e9786_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign15020_e9784, assign15020_e9784_d_n0, assign15020_e9784_d_n2, assign15020_e9784_d_n4, assign15020_e9784_d_n5, assign15020_e9784_d_n6, assign15020_e9784_d_n7, assign15020_e9784_d_n8, assign15020_e9784_d_n9, assign15020_e9784_d_n10, assign15020_e9784_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign15020_e9783: f64 = (-locals.var_tmf2);
                (assign15020_e9783, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign15020_e9784, assign15020_e9784_d_n0, assign15020_e9784_d_n2, assign15020_e9784_d_n4, assign15020_e9784_d_n5, assign15020_e9784_d_n6, assign15020_e9784_d_n7, assign15020_e9784_d_n8, assign15020_e9784_d_n9, assign15020_e9784_d_n10, assign15020_e9784_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15020_e9786;
        locals.var_tmf2_dn0 = assign15020_e9786_d_n0;
        locals.var_tmf2_dn2 = assign15020_e9786_d_n2;
        locals.var_tmf2_dn4 = assign15020_e9786_d_n4;
        locals.var_tmf2_dn5 = assign15020_e9786_d_n5;
        locals.var_tmf2_dn6 = assign15020_e9786_d_n6;
        locals.var_tmf2_dn7 = assign15020_e9786_d_n7;
        locals.var_tmf2_dn8 = assign15020_e9786_d_n8;
        locals.var_tmf2_dn9 = assign15020_e9786_d_n9;
        locals.var_tmf2_dn10 = assign15020_e9786_d_n10;
        locals.var_tmf2_dn13 = assign15020_e9786_d_n13;

        let (assign15030_e9799, assign15030_e9799_d_n0, assign15030_e9799_d_n2, assign15030_e9799_d_n4, assign15030_e9799_d_n5, assign15030_e9799_d_n6, assign15030_e9799_d_n7, assign15030_e9799_d_n8, assign15030_e9799_d_n9, assign15030_e9799_d_n10, assign15030_e9799_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15030_e9794: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15030_e9796: f64 = (assign15030_e9794 + locals.var_tmf2);
        let assign15030_e9797: f64 = (assign15030_e9796).sqrt();
        (assign15030_e9797, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15030_e9797)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign15030_e9797)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15030_e9799;
        locals.var_tmf2_dn0 = assign15030_e9799_d_n0;
        locals.var_tmf2_dn2 = assign15030_e9799_d_n2;
        locals.var_tmf2_dn4 = assign15030_e9799_d_n4;
        locals.var_tmf2_dn5 = assign15030_e9799_d_n5;
        locals.var_tmf2_dn6 = assign15030_e9799_d_n6;
        locals.var_tmf2_dn7 = assign15030_e9799_d_n7;
        locals.var_tmf2_dn8 = assign15030_e9799_d_n8;
        locals.var_tmf2_dn9 = assign15030_e9799_d_n9;
        locals.var_tmf2_dn10 = assign15030_e9799_d_n10;
        locals.var_tmf2_dn13 = assign15030_e9799_d_n13;

        let (assign15040_e9813, assign15040_e9813_d_n0, assign15040_e9813_d_n2, assign15040_e9813_d_n4, assign15040_e9813_d_n5, assign15040_e9813_d_n6, assign15040_e9813_d_n7, assign15040_e9813_d_n8, assign15040_e9813_d_n9, assign15040_e9813_d_n10, assign15040_e9813_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15040_e9809: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15040_e9810: f64 = (1.0 + assign15040_e9809);
        let assign15040_e9811: f64 = (0.5 * assign15040_e9810);
        (assign15040_e9811, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign15040_e9813;
        locals.var_t6_dn0 = assign15040_e9813_d_n0;
        locals.var_t6_dn2 = assign15040_e9813_d_n2;
        locals.var_t6_dn4 = assign15040_e9813_d_n4;
        locals.var_t6_dn5 = assign15040_e9813_d_n5;
        locals.var_t6_dn6 = assign15040_e9813_d_n6;
        locals.var_t6_dn7 = assign15040_e9813_d_n7;
        locals.var_t6_dn8 = assign15040_e9813_d_n8;
        locals.var_t6_dn9 = assign15040_e9813_d_n9;
        locals.var_t6_dn10 = assign15040_e9813_d_n10;
        locals.var_t6_dn13 = assign15040_e9813_d_n13;

        let (assign15050_e9831, assign15050_e9831_d_n0, assign15050_e9831_d_n2, assign15050_e9831_d_n4, assign15050_e9831_d_n5, assign15050_e9831_d_n6, assign15050_e9831_d_n7, assign15050_e9831_d_n8, assign15050_e9831_d_n9, assign15050_e9831_d_n10, assign15050_e9831_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15050_e9822: f64 = (p.p98 + 1.0);
        let assign15050_e9823: f64 = (locals.var_t4 * assign15050_e9822);
        let assign15050_e9827: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15050_e9828: f64 = (0.5 * assign15050_e9827);
        let assign15050_e9829: f64 = (assign15050_e9823 - assign15050_e9828);
        (assign15050_e9829, ((locals.var_t4_dn0 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn13 * assign15050_e9822) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign15050_e9831;
        locals.var_t7_dn0 = assign15050_e9831_d_n0;
        locals.var_t7_dn2 = assign15050_e9831_d_n2;
        locals.var_t7_dn4 = assign15050_e9831_d_n4;
        locals.var_t7_dn5 = assign15050_e9831_d_n5;
        locals.var_t7_dn6 = assign15050_e9831_d_n6;
        locals.var_t7_dn7 = assign15050_e9831_d_n7;
        locals.var_t7_dn8 = assign15050_e9831_d_n8;
        locals.var_t7_dn9 = assign15050_e9831_d_n9;
        locals.var_t7_dn10 = assign15050_e9831_d_n10;
        locals.var_t7_dn13 = assign15050_e9831_d_n13;

        let (assign15060_e9847, assign15060_e9847_d_n0, assign15060_e9847_d_n2, assign15060_e9847_d_n4, assign15060_e9847_d_n5, assign15060_e9847_d_n6, assign15060_e9847_d_n7, assign15060_e9847_d_n8, assign15060_e9847_d_n9, assign15060_e9847_d_n10, assign15060_e9847_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15060_e9840: f64 = (locals.var_t1 * locals.var_t4);
        let assign15060_e9841: f64 = (locals.var_t7 + assign15060_e9840);
        let assign15060_e9843: f64 = assign15060_e9841;
        let assign15060_e9845: f64 = (assign15060_e9843 - 5e-5);
        (assign15060_e9845, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn13 + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign15060_e9847;
        locals.var_tmf1_dn0 = assign15060_e9847_d_n0;
        locals.var_tmf1_dn2 = assign15060_e9847_d_n2;
        locals.var_tmf1_dn4 = assign15060_e9847_d_n4;
        locals.var_tmf1_dn5 = assign15060_e9847_d_n5;
        locals.var_tmf1_dn6 = assign15060_e9847_d_n6;
        locals.var_tmf1_dn7 = assign15060_e9847_d_n7;
        locals.var_tmf1_dn8 = assign15060_e9847_d_n8;
        locals.var_tmf1_dn9 = assign15060_e9847_d_n9;
        locals.var_tmf1_dn10 = assign15060_e9847_d_n10;
        locals.var_tmf1_dn13 = assign15060_e9847_d_n13;

        let (assign15070_e9859, assign15070_e9859_d_n0, assign15070_e9859_d_n2, assign15070_e9859_d_n4, assign15070_e9859_d_n5, assign15070_e9859_d_n6, assign15070_e9859_d_n7, assign15070_e9859_d_n8, assign15070_e9859_d_n9, assign15070_e9859_d_n10, assign15070_e9859_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15070_e9859;
        locals.var_tmf2_dn0 = assign15070_e9859_d_n0;
        locals.var_tmf2_dn2 = assign15070_e9859_d_n2;
        locals.var_tmf2_dn4 = assign15070_e9859_d_n4;
        locals.var_tmf2_dn5 = assign15070_e9859_d_n5;
        locals.var_tmf2_dn6 = assign15070_e9859_d_n6;
        locals.var_tmf2_dn7 = assign15070_e9859_d_n7;
        locals.var_tmf2_dn8 = assign15070_e9859_d_n8;
        locals.var_tmf2_dn9 = assign15070_e9859_d_n9;
        locals.var_tmf2_dn10 = assign15070_e9859_d_n10;
        locals.var_tmf2_dn13 = assign15070_e9859_d_n13;

        let (assign15080_e9873, assign15080_e9873_d_n0, assign15080_e9873_d_n2, assign15080_e9873_d_n4, assign15080_e9873_d_n5, assign15080_e9873_d_n6, assign15080_e9873_d_n7, assign15080_e9873_d_n8, assign15080_e9873_d_n9, assign15080_e9873_d_n10, assign15080_e9873_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let (assign15080_e9871, assign15080_e9871_d_n0, assign15080_e9871_d_n2, assign15080_e9871_d_n4, assign15080_e9871_d_n5, assign15080_e9871_d_n6, assign15080_e9871_d_n7, assign15080_e9871_d_n8, assign15080_e9871_d_n9, assign15080_e9871_d_n10, assign15080_e9871_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign15080_e9870: f64 = (-locals.var_tmf2);
                (assign15080_e9870, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign15080_e9871, assign15080_e9871_d_n0, assign15080_e9871_d_n2, assign15080_e9871_d_n4, assign15080_e9871_d_n5, assign15080_e9871_d_n6, assign15080_e9871_d_n7, assign15080_e9871_d_n8, assign15080_e9871_d_n9, assign15080_e9871_d_n10, assign15080_e9871_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15080_e9873;
        locals.var_tmf2_dn0 = assign15080_e9873_d_n0;
        locals.var_tmf2_dn2 = assign15080_e9873_d_n2;
        locals.var_tmf2_dn4 = assign15080_e9873_d_n4;
        locals.var_tmf2_dn5 = assign15080_e9873_d_n5;
        locals.var_tmf2_dn6 = assign15080_e9873_d_n6;
        locals.var_tmf2_dn7 = assign15080_e9873_d_n7;
        locals.var_tmf2_dn8 = assign15080_e9873_d_n8;
        locals.var_tmf2_dn9 = assign15080_e9873_d_n9;
        locals.var_tmf2_dn10 = assign15080_e9873_d_n10;
        locals.var_tmf2_dn13 = assign15080_e9873_d_n13;

        let (assign15090_e9886, assign15090_e9886_d_n0, assign15090_e9886_d_n2, assign15090_e9886_d_n4, assign15090_e9886_d_n5, assign15090_e9886_d_n6, assign15090_e9886_d_n7, assign15090_e9886_d_n8, assign15090_e9886_d_n9, assign15090_e9886_d_n10, assign15090_e9886_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15090_e9881: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15090_e9883: f64 = (assign15090_e9881 + locals.var_tmf2);
        let assign15090_e9884: f64 = (assign15090_e9883).sqrt();
        (assign15090_e9884, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15090_e9884)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign15090_e9884)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15090_e9886;
        locals.var_tmf2_dn0 = assign15090_e9886_d_n0;
        locals.var_tmf2_dn2 = assign15090_e9886_d_n2;
        locals.var_tmf2_dn4 = assign15090_e9886_d_n4;
        locals.var_tmf2_dn5 = assign15090_e9886_d_n5;
        locals.var_tmf2_dn6 = assign15090_e9886_d_n6;
        locals.var_tmf2_dn7 = assign15090_e9886_d_n7;
        locals.var_tmf2_dn8 = assign15090_e9886_d_n8;
        locals.var_tmf2_dn9 = assign15090_e9886_d_n9;
        locals.var_tmf2_dn10 = assign15090_e9886_d_n10;
        locals.var_tmf2_dn13 = assign15090_e9886_d_n13;

        let (assign15100_e9900, assign15100_e9900_d_n0, assign15100_e9900_d_n2, assign15100_e9900_d_n4, assign15100_e9900_d_n5, assign15100_e9900_d_n6, assign15100_e9900_d_n7, assign15100_e9900_d_n8, assign15100_e9900_d_n9, assign15100_e9900_d_n10, assign15100_e9900_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15100_e9896: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15100_e9897: f64 = (1.0 + assign15100_e9896);
        let assign15100_e9898: f64 = (0.5 * assign15100_e9897);
        (assign15100_e9898, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign15100_e9900;
        locals.var_t6_dn0 = assign15100_e9900_d_n0;
        locals.var_t6_dn2 = assign15100_e9900_d_n2;
        locals.var_t6_dn4 = assign15100_e9900_d_n4;
        locals.var_t6_dn5 = assign15100_e9900_d_n5;
        locals.var_t6_dn6 = assign15100_e9900_d_n6;
        locals.var_t6_dn7 = assign15100_e9900_d_n7;
        locals.var_t6_dn8 = assign15100_e9900_d_n8;
        locals.var_t6_dn9 = assign15100_e9900_d_n9;
        locals.var_t6_dn10 = assign15100_e9900_d_n10;
        locals.var_t6_dn13 = assign15100_e9900_d_n13;

        let (assign15110_e9914, assign15110_e9914_d_n0, assign15110_e9914_d_n2, assign15110_e9914_d_n4, assign15110_e9914_d_n5, assign15110_e9914_d_n6, assign15110_e9914_d_n7, assign15110_e9914_d_n8, assign15110_e9914_d_n9, assign15110_e9914_d_n10, assign15110_e9914_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign15110_e9910: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15110_e9911: f64 = (0.5 * assign15110_e9910);
        let assign15110_e9912: f64 = assign15110_e9911;
        (assign15110_e9912, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign15110_e9914;
        locals.var_t2_dn0 = assign15110_e9914_d_n0;
        locals.var_t2_dn2 = assign15110_e9914_d_n2;
        locals.var_t2_dn4 = assign15110_e9914_d_n4;
        locals.var_t2_dn5 = assign15110_e9914_d_n5;
        locals.var_t2_dn6 = assign15110_e9914_d_n6;
        locals.var_t2_dn7 = assign15110_e9914_d_n7;
        locals.var_t2_dn8 = assign15110_e9914_d_n8;
        locals.var_t2_dn9 = assign15110_e9914_d_n9;
        locals.var_t2_dn10 = assign15110_e9914_d_n10;
        locals.var_t2_dn13 = assign15110_e9914_d_n13;

        let assign15120_e9921: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard321 = assign15120_e9921;

        let (assign15130_e9941, assign15130_e9941_d_n0, assign15130_e9941_d_n2, assign15130_e9941_d_n4, assign15130_e9941_d_n5, assign15130_e9941_d_n6, assign15130_e9941_d_n7, assign15130_e9941_d_n8, assign15130_e9941_d_n9, assign15130_e9941_d_n10, assign15130_e9941_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15130_e9932: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign15130_e9933: f64 = (locals.var_uc_rdvd + assign15130_e9932);
        let assign15130_e9936: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign15130_e9937: f64 = (assign15130_e9933 + assign15130_e9936);
        let assign15130_e9939: f64 = (assign15130_e9937 * locals.var_t2);
        (assign15130_e9939, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign15130_e9937 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15130_e9941;
        locals.var_rsvde_dn0 = assign15130_e9941_d_n0;
        locals.var_rsvde_dn2 = assign15130_e9941_d_n2;
        locals.var_rsvde_dn4 = assign15130_e9941_d_n4;
        locals.var_rsvde_dn5 = assign15130_e9941_d_n5;
        locals.var_rsvde_dn6 = assign15130_e9941_d_n6;
        locals.var_rsvde_dn7 = assign15130_e9941_d_n7;
        locals.var_rsvde_dn8 = assign15130_e9941_d_n8;
        locals.var_rsvde_dn9 = assign15130_e9941_d_n9;
        locals.var_rsvde_dn10 = assign15130_e9941_d_n10;
        locals.var_rsvde_dn13 = assign15130_e9941_d_n13;

        let (assign15140_e9959, assign15140_e9959_d_n0, assign15140_e9959_d_n2, assign15140_e9959_d_n4, assign15140_e9959_d_n5, assign15140_e9959_d_n6, assign15140_e9959_d_n7, assign15140_e9959_d_n8, assign15140_e9959_d_n9, assign15140_e9959_d_n10, assign15140_e9959_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15140_e9952: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15140_e9953: f64 = (locals.var_rsvde - assign15140_e9952);
        let assign15140_e9956: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15140_e9957: f64 = (assign15140_e9953 - assign15140_e9956);
        (assign15140_e9957, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign15140_e9959;
        locals.var_tmf1_dn0 = assign15140_e9959_d_n0;
        locals.var_tmf1_dn2 = assign15140_e9959_d_n2;
        locals.var_tmf1_dn4 = assign15140_e9959_d_n4;
        locals.var_tmf1_dn5 = assign15140_e9959_d_n5;
        locals.var_tmf1_dn6 = assign15140_e9959_d_n6;
        locals.var_tmf1_dn7 = assign15140_e9959_d_n7;
        locals.var_tmf1_dn8 = assign15140_e9959_d_n8;
        locals.var_tmf1_dn9 = assign15140_e9959_d_n9;
        locals.var_tmf1_dn10 = assign15140_e9959_d_n10;
        locals.var_tmf1_dn13 = assign15140_e9959_d_n13;

        let (assign15150_e9977, assign15150_e9977_d_n0, assign15150_e9977_d_n2, assign15150_e9977_d_n4, assign15150_e9977_d_n5, assign15150_e9977_d_n6, assign15150_e9977_d_n7, assign15150_e9977_d_n8, assign15150_e9977_d_n9, assign15150_e9977_d_n10, assign15150_e9977_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15150_e9970: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15150_e9971: f64 = (4.0 * assign15150_e9970);
        let assign15150_e9974: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15150_e9975: f64 = (assign15150_e9971 * assign15150_e9974);
        (assign15150_e9975, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15150_e9977;
        locals.var_tmf2_dn0 = assign15150_e9977_d_n0;
        locals.var_tmf2_dn2 = assign15150_e9977_d_n2;
        locals.var_tmf2_dn4 = assign15150_e9977_d_n4;
        locals.var_tmf2_dn5 = assign15150_e9977_d_n5;
        locals.var_tmf2_dn6 = assign15150_e9977_d_n6;
        locals.var_tmf2_dn7 = assign15150_e9977_d_n7;
        locals.var_tmf2_dn8 = assign15150_e9977_d_n8;
        locals.var_tmf2_dn9 = assign15150_e9977_d_n9;
        locals.var_tmf2_dn10 = assign15150_e9977_d_n10;
        locals.var_tmf2_dn13 = assign15150_e9977_d_n13;

        let (assign15160_e9993, assign15160_e9993_d_n0, assign15160_e9993_d_n2, assign15160_e9993_d_n4, assign15160_e9993_d_n5, assign15160_e9993_d_n6, assign15160_e9993_d_n7, assign15160_e9993_d_n8, assign15160_e9993_d_n9, assign15160_e9993_d_n10, assign15160_e9993_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let (assign15160_e9991, assign15160_e9991_d_n0, assign15160_e9991_d_n2, assign15160_e9991_d_n4, assign15160_e9991_d_n5, assign15160_e9991_d_n6, assign15160_e9991_d_n7, assign15160_e9991_d_n8, assign15160_e9991_d_n9, assign15160_e9991_d_n10, assign15160_e9991_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign15160_e9990: f64 = (-locals.var_tmf2);
                (assign15160_e9990, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign15160_e9991, assign15160_e9991_d_n0, assign15160_e9991_d_n2, assign15160_e9991_d_n4, assign15160_e9991_d_n5, assign15160_e9991_d_n6, assign15160_e9991_d_n7, assign15160_e9991_d_n8, assign15160_e9991_d_n9, assign15160_e9991_d_n10, assign15160_e9991_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15160_e9993;
        locals.var_tmf2_dn0 = assign15160_e9993_d_n0;
        locals.var_tmf2_dn2 = assign15160_e9993_d_n2;
        locals.var_tmf2_dn4 = assign15160_e9993_d_n4;
        locals.var_tmf2_dn5 = assign15160_e9993_d_n5;
        locals.var_tmf2_dn6 = assign15160_e9993_d_n6;
        locals.var_tmf2_dn7 = assign15160_e9993_d_n7;
        locals.var_tmf2_dn8 = assign15160_e9993_d_n8;
        locals.var_tmf2_dn9 = assign15160_e9993_d_n9;
        locals.var_tmf2_dn10 = assign15160_e9993_d_n10;
        locals.var_tmf2_dn13 = assign15160_e9993_d_n13;

        let (assign15170_e10008, assign15170_e10008_d_n0, assign15170_e10008_d_n2, assign15170_e10008_d_n4, assign15170_e10008_d_n5, assign15170_e10008_d_n6, assign15170_e10008_d_n7, assign15170_e10008_d_n8, assign15170_e10008_d_n9, assign15170_e10008_d_n10, assign15170_e10008_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15170_e10003: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15170_e10005: f64 = (assign15170_e10003 + locals.var_tmf2);
        let assign15170_e10006: f64 = (assign15170_e10005).sqrt();
        (assign15170_e10006, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15170_e10006)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign15170_e10006)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15170_e10008;
        locals.var_tmf2_dn0 = assign15170_e10008_d_n0;
        locals.var_tmf2_dn2 = assign15170_e10008_d_n2;
        locals.var_tmf2_dn4 = assign15170_e10008_d_n4;
        locals.var_tmf2_dn5 = assign15170_e10008_d_n5;
        locals.var_tmf2_dn6 = assign15170_e10008_d_n6;
        locals.var_tmf2_dn7 = assign15170_e10008_d_n7;
        locals.var_tmf2_dn8 = assign15170_e10008_d_n8;
        locals.var_tmf2_dn9 = assign15170_e10008_d_n9;
        locals.var_tmf2_dn10 = assign15170_e10008_d_n10;
        locals.var_tmf2_dn13 = assign15170_e10008_d_n13;

    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15180_e10024, assign15180_e10024_d_n0, assign15180_e10024_d_n2, assign15180_e10024_d_n4, assign15180_e10024_d_n5, assign15180_e10024_d_n6, assign15180_e10024_d_n7, assign15180_e10024_d_n8, assign15180_e10024_d_n9, assign15180_e10024_d_n10, assign15180_e10024_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15180_e10020: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15180_e10021: f64 = (1.0 + assign15180_e10020);
        let assign15180_e10022: f64 = (0.5 * assign15180_e10021);
        (assign15180_e10022, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15180_e10024;
        locals.var_t0_dn0 = assign15180_e10024_d_n0;
        locals.var_t0_dn2 = assign15180_e10024_d_n2;
        locals.var_t0_dn4 = assign15180_e10024_d_n4;
        locals.var_t0_dn5 = assign15180_e10024_d_n5;
        locals.var_t0_dn6 = assign15180_e10024_d_n6;
        locals.var_t0_dn7 = assign15180_e10024_d_n7;
        locals.var_t0_dn8 = assign15180_e10024_d_n8;
        locals.var_t0_dn9 = assign15180_e10024_d_n9;
        locals.var_t0_dn10 = assign15180_e10024_d_n10;
        locals.var_t0_dn13 = assign15180_e10024_d_n13;

        let (assign15190_e10042, assign15190_e10042_d_n0, assign15190_e10042_d_n2, assign15190_e10042_d_n4, assign15190_e10042_d_n5, assign15190_e10042_d_n6, assign15190_e10042_d_n7, assign15190_e10042_d_n8, assign15190_e10042_d_n9, assign15190_e10042_d_n10, assign15190_e10042_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign15190_e10034: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15190_e10038: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15190_e10039: f64 = (0.5 * assign15190_e10038);
        let assign15190_e10040: f64 = (assign15190_e10034 + assign15190_e10039);
        (assign15190_e10040, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15190_e10042;
        locals.var_rsvde_dn0 = assign15190_e10042_d_n0;
        locals.var_rsvde_dn2 = assign15190_e10042_d_n2;
        locals.var_rsvde_dn4 = assign15190_e10042_d_n4;
        locals.var_rsvde_dn5 = assign15190_e10042_d_n5;
        locals.var_rsvde_dn6 = assign15190_e10042_d_n6;
        locals.var_rsvde_dn7 = assign15190_e10042_d_n7;
        locals.var_rsvde_dn8 = assign15190_e10042_d_n8;
        locals.var_rsvde_dn9 = assign15190_e10042_d_n9;
        locals.var_rsvde_dn10 = assign15190_e10042_d_n10;
        locals.var_rsvde_dn13 = assign15190_e10042_d_n13;

        let (assign15200_e10063, assign15200_e10063_d_n0, assign15200_e10063_d_n2, assign15200_e10063_d_n4, assign15200_e10063_d_n5, assign15200_e10063_d_n6, assign15200_e10063_d_n7, assign15200_e10063_d_n8, assign15200_e10063_d_n9, assign15200_e10063_d_n10, assign15200_e10063_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15200_e10054: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign15200_e10055: f64 = (locals.var_uc_rdvd + assign15200_e10054);
        let assign15200_e10058: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign15200_e10059: f64 = (assign15200_e10055 + assign15200_e10058);
        let assign15200_e10061: f64 = (assign15200_e10059 * locals.var_t2);
        (assign15200_e10061, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign15200_e10059 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15200_e10063;
        locals.var_rsvde_dn0 = assign15200_e10063_d_n0;
        locals.var_rsvde_dn2 = assign15200_e10063_d_n2;
        locals.var_rsvde_dn4 = assign15200_e10063_d_n4;
        locals.var_rsvde_dn5 = assign15200_e10063_d_n5;
        locals.var_rsvde_dn6 = assign15200_e10063_d_n6;
        locals.var_rsvde_dn7 = assign15200_e10063_d_n7;
        locals.var_rsvde_dn8 = assign15200_e10063_d_n8;
        locals.var_rsvde_dn9 = assign15200_e10063_d_n9;
        locals.var_rsvde_dn10 = assign15200_e10063_d_n10;
        locals.var_rsvde_dn13 = assign15200_e10063_d_n13;

        let (assign15210_e10082, assign15210_e10082_d_n0, assign15210_e10082_d_n2, assign15210_e10082_d_n4, assign15210_e10082_d_n5, assign15210_e10082_d_n6, assign15210_e10082_d_n7, assign15210_e10082_d_n8, assign15210_e10082_d_n9, assign15210_e10082_d_n10, assign15210_e10082_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15210_e10075: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15210_e10076: f64 = (locals.var_rsvde - assign15210_e10075);
        let assign15210_e10079: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15210_e10080: f64 = (assign15210_e10076 - assign15210_e10079);
        (assign15210_e10080, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign15210_e10082;
        locals.var_tmf1_dn0 = assign15210_e10082_d_n0;
        locals.var_tmf1_dn2 = assign15210_e10082_d_n2;
        locals.var_tmf1_dn4 = assign15210_e10082_d_n4;
        locals.var_tmf1_dn5 = assign15210_e10082_d_n5;
        locals.var_tmf1_dn6 = assign15210_e10082_d_n6;
        locals.var_tmf1_dn7 = assign15210_e10082_d_n7;
        locals.var_tmf1_dn8 = assign15210_e10082_d_n8;
        locals.var_tmf1_dn9 = assign15210_e10082_d_n9;
        locals.var_tmf1_dn10 = assign15210_e10082_d_n10;
        locals.var_tmf1_dn13 = assign15210_e10082_d_n13;

        let (assign15220_e10101, assign15220_e10101_d_n0, assign15220_e10101_d_n2, assign15220_e10101_d_n4, assign15220_e10101_d_n5, assign15220_e10101_d_n6, assign15220_e10101_d_n7, assign15220_e10101_d_n8, assign15220_e10101_d_n9, assign15220_e10101_d_n10, assign15220_e10101_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15220_e10094: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15220_e10095: f64 = (4.0 * assign15220_e10094);
        let assign15220_e10098: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15220_e10099: f64 = (assign15220_e10095 * assign15220_e10098);
        (assign15220_e10099, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15220_e10101;
        locals.var_tmf2_dn0 = assign15220_e10101_d_n0;
        locals.var_tmf2_dn2 = assign15220_e10101_d_n2;
        locals.var_tmf2_dn4 = assign15220_e10101_d_n4;
        locals.var_tmf2_dn5 = assign15220_e10101_d_n5;
        locals.var_tmf2_dn6 = assign15220_e10101_d_n6;
        locals.var_tmf2_dn7 = assign15220_e10101_d_n7;
        locals.var_tmf2_dn8 = assign15220_e10101_d_n8;
        locals.var_tmf2_dn9 = assign15220_e10101_d_n9;
        locals.var_tmf2_dn10 = assign15220_e10101_d_n10;
        locals.var_tmf2_dn13 = assign15220_e10101_d_n13;

        let (assign15230_e10118, assign15230_e10118_d_n0, assign15230_e10118_d_n2, assign15230_e10118_d_n4, assign15230_e10118_d_n5, assign15230_e10118_d_n6, assign15230_e10118_d_n7, assign15230_e10118_d_n8, assign15230_e10118_d_n9, assign15230_e10118_d_n10, assign15230_e10118_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let (assign15230_e10116, assign15230_e10116_d_n0, assign15230_e10116_d_n2, assign15230_e10116_d_n4, assign15230_e10116_d_n5, assign15230_e10116_d_n6, assign15230_e10116_d_n7, assign15230_e10116_d_n8, assign15230_e10116_d_n9, assign15230_e10116_d_n10, assign15230_e10116_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign15230_e10115: f64 = (-locals.var_tmf2);
                (assign15230_e10115, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign15230_e10116, assign15230_e10116_d_n0, assign15230_e10116_d_n2, assign15230_e10116_d_n4, assign15230_e10116_d_n5, assign15230_e10116_d_n6, assign15230_e10116_d_n7, assign15230_e10116_d_n8, assign15230_e10116_d_n9, assign15230_e10116_d_n10, assign15230_e10116_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15230_e10118;
        locals.var_tmf2_dn0 = assign15230_e10118_d_n0;
        locals.var_tmf2_dn2 = assign15230_e10118_d_n2;
        locals.var_tmf2_dn4 = assign15230_e10118_d_n4;
        locals.var_tmf2_dn5 = assign15230_e10118_d_n5;
        locals.var_tmf2_dn6 = assign15230_e10118_d_n6;
        locals.var_tmf2_dn7 = assign15230_e10118_d_n7;
        locals.var_tmf2_dn8 = assign15230_e10118_d_n8;
        locals.var_tmf2_dn9 = assign15230_e10118_d_n9;
        locals.var_tmf2_dn10 = assign15230_e10118_d_n10;
        locals.var_tmf2_dn13 = assign15230_e10118_d_n13;

        let (assign15240_e10134, assign15240_e10134_d_n0, assign15240_e10134_d_n2, assign15240_e10134_d_n4, assign15240_e10134_d_n5, assign15240_e10134_d_n6, assign15240_e10134_d_n7, assign15240_e10134_d_n8, assign15240_e10134_d_n9, assign15240_e10134_d_n10, assign15240_e10134_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15240_e10129: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15240_e10131: f64 = (assign15240_e10129 + locals.var_tmf2);
        let assign15240_e10132: f64 = (assign15240_e10131).sqrt();
        (assign15240_e10132, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15240_e10132)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign15240_e10132)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign15240_e10134;
        locals.var_tmf2_dn0 = assign15240_e10134_d_n0;
        locals.var_tmf2_dn2 = assign15240_e10134_d_n2;
        locals.var_tmf2_dn4 = assign15240_e10134_d_n4;
        locals.var_tmf2_dn5 = assign15240_e10134_d_n5;
        locals.var_tmf2_dn6 = assign15240_e10134_d_n6;
        locals.var_tmf2_dn7 = assign15240_e10134_d_n7;
        locals.var_tmf2_dn8 = assign15240_e10134_d_n8;
        locals.var_tmf2_dn9 = assign15240_e10134_d_n9;
        locals.var_tmf2_dn10 = assign15240_e10134_d_n10;
        locals.var_tmf2_dn13 = assign15240_e10134_d_n13;

        let (assign15250_e10151, assign15250_e10151_d_n0, assign15250_e10151_d_n2, assign15250_e10151_d_n4, assign15250_e10151_d_n5, assign15250_e10151_d_n6, assign15250_e10151_d_n7, assign15250_e10151_d_n8, assign15250_e10151_d_n9, assign15250_e10151_d_n10, assign15250_e10151_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15250_e10147: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15250_e10148: f64 = (1.0 + assign15250_e10147);
        let assign15250_e10149: f64 = (0.5 * assign15250_e10148);
        (assign15250_e10149, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15250_e10151;
        locals.var_t0_dn0 = assign15250_e10151_d_n0;
        locals.var_t0_dn2 = assign15250_e10151_d_n2;
        locals.var_t0_dn4 = assign15250_e10151_d_n4;
        locals.var_t0_dn5 = assign15250_e10151_d_n5;
        locals.var_t0_dn6 = assign15250_e10151_d_n6;
        locals.var_t0_dn7 = assign15250_e10151_d_n7;
        locals.var_t0_dn8 = assign15250_e10151_d_n8;
        locals.var_t0_dn9 = assign15250_e10151_d_n9;
        locals.var_t0_dn10 = assign15250_e10151_d_n10;
        locals.var_t0_dn13 = assign15250_e10151_d_n13;

        let (assign15260_e10170, assign15260_e10170_d_n0, assign15260_e10170_d_n2, assign15260_e10170_d_n4, assign15260_e10170_d_n5, assign15260_e10170_d_n6, assign15260_e10170_d_n7, assign15260_e10170_d_n8, assign15260_e10170_d_n9, assign15260_e10170_d_n10, assign15260_e10170_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign15260_e10162: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15260_e10166: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15260_e10167: f64 = (0.5 * assign15260_e10166);
        let assign15260_e10168: f64 = (assign15260_e10162 + assign15260_e10167);
        (assign15260_e10168, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15260_e10170;
        locals.var_rsvde_dn0 = assign15260_e10170_d_n0;
        locals.var_rsvde_dn2 = assign15260_e10170_d_n2;
        locals.var_rsvde_dn4 = assign15260_e10170_d_n4;
        locals.var_rsvde_dn5 = assign15260_e10170_d_n5;
        locals.var_rsvde_dn6 = assign15260_e10170_d_n6;
        locals.var_rsvde_dn7 = assign15260_e10170_d_n7;
        locals.var_rsvde_dn8 = assign15260_e10170_d_n8;
        locals.var_rsvde_dn9 = assign15260_e10170_d_n9;
        locals.var_rsvde_dn10 = assign15260_e10170_d_n10;
        locals.var_rsvde_dn13 = assign15260_e10170_d_n13;

        let (assign15270_e10179, assign15270_e10179_d_n0, assign15270_e10179_d_n2, assign15270_e10179_d_n4, assign15270_e10179_d_n5, assign15270_e10179_d_n6, assign15270_e10179_d_n7, assign15270_e10179_d_n8, assign15270_e10179_d_n9, assign15270_e10179_d_n10, assign15270_e10179_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign15270_e10179;
        locals.var_rdvde_dn0 = assign15270_e10179_d_n0;
        locals.var_rdvde_dn2 = assign15270_e10179_d_n2;
        locals.var_rdvde_dn4 = assign15270_e10179_d_n4;
        locals.var_rdvde_dn5 = assign15270_e10179_d_n5;
        locals.var_rdvde_dn6 = assign15270_e10179_d_n6;
        locals.var_rdvde_dn7 = assign15270_e10179_d_n7;
        locals.var_rdvde_dn8 = assign15270_e10179_d_n8;
        locals.var_rdvde_dn9 = assign15270_e10179_d_n9;
        locals.var_rdvde_dn10 = assign15270_e10179_d_n10;
        locals.var_rdvde_dn13 = assign15270_e10179_d_n13;

        let (assign15280_e10188, assign15280_e10188_d_n0, assign15280_e10188_d_n2, assign15280_e10188_d_n4, assign15280_e10188_d_n5, assign15280_e10188_d_n6, assign15280_e10188_d_n7, assign15280_e10188_d_n8, assign15280_e10188_d_n9, assign15280_e10188_d_n10, assign15280_e10188_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard313 != 0.0)) && (locals.var_guard318 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign15280_e10188;
        locals.var_rsvde_dn0 = assign15280_e10188_d_n0;
        locals.var_rsvde_dn2 = assign15280_e10188_d_n2;
        locals.var_rsvde_dn4 = assign15280_e10188_d_n4;
        locals.var_rsvde_dn5 = assign15280_e10188_d_n5;
        locals.var_rsvde_dn6 = assign15280_e10188_d_n6;
        locals.var_rsvde_dn7 = assign15280_e10188_d_n7;
        locals.var_rsvde_dn8 = assign15280_e10188_d_n8;
        locals.var_rsvde_dn9 = assign15280_e10188_d_n9;
        locals.var_rsvde_dn10 = assign15280_e10188_d_n10;
        locals.var_rsvde_dn13 = assign15280_e10188_d_n13;

        let (assign15290_e10195, assign15290_e10195_d_n0, assign15290_e10195_d_n2, assign15290_e10195_d_n4, assign15290_e10195_d_n5, assign15290_e10195_d_n6, assign15290_e10195_d_n7, assign15290_e10195_d_n8, assign15290_e10195_d_n9, assign15290_e10195_d_n10, assign15290_e10195_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15290_e10192: f64 = (locals.var_beta_inv).sqrt();
        let assign15290_e10193: f64 = (locals.var_costi00 * assign15290_e10192);
        (assign15290_e10193, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign15290_e10192))), (locals.var_costi00 * (locals.var_beta_inv_dn13 / (2.0 * assign15290_e10192))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn13,)
    }
};
        locals.var_costi0 = assign15290_e10195;
        locals.var_costi0_dn0 = assign15290_e10195_d_n0;
        locals.var_costi0_dn2 = assign15290_e10195_d_n2;
        locals.var_costi0_dn4 = assign15290_e10195_d_n4;
        locals.var_costi0_dn5 = assign15290_e10195_d_n5;
        locals.var_costi0_dn6 = assign15290_e10195_d_n6;
        locals.var_costi0_dn7 = assign15290_e10195_d_n7;
        locals.var_costi0_dn8 = assign15290_e10195_d_n8;
        locals.var_costi0_dn9 = assign15290_e10195_d_n9;
        locals.var_costi0_dn10 = assign15290_e10195_d_n10;
        locals.var_costi0_dn13 = assign15290_e10195_d_n13;

        let (assign15300_e10201, assign15300_e10201_d_n0, assign15300_e10201_d_n2, assign15300_e10201_d_n4, assign15300_e10201_d_n5, assign15300_e10201_d_n6, assign15300_e10201_d_n7, assign15300_e10201_d_n8, assign15300_e10201_d_n9, assign15300_e10201_d_n10, assign15300_e10201_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15300_e10199: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign15300_e10199, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn13 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn13)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn13,)
    }
};
        locals.var_costi0_p2 = assign15300_e10201;
        locals.var_costi0_p2_dn0 = assign15300_e10201_d_n0;
        locals.var_costi0_p2_dn2 = assign15300_e10201_d_n2;
        locals.var_costi0_p2_dn4 = assign15300_e10201_d_n4;
        locals.var_costi0_p2_dn5 = assign15300_e10201_d_n5;
        locals.var_costi0_p2_dn6 = assign15300_e10201_d_n6;
        locals.var_costi0_p2_dn7 = assign15300_e10201_d_n7;
        locals.var_costi0_p2_dn8 = assign15300_e10201_d_n8;
        locals.var_costi0_p2_dn9 = assign15300_e10201_d_n9;
        locals.var_costi0_p2_dn10 = assign15300_e10201_d_n10;
        locals.var_costi0_p2_dn13 = assign15300_e10201_d_n13;

        let (assign15310_e10209, assign15310_e10209_d_n0, assign15310_e10209_d_n2, assign15310_e10209_d_n4, assign15310_e10209_d_n5, assign15310_e10209_d_n6, assign15310_e10209_d_n7, assign15310_e10209_d_n8, assign15310_e10209_d_n9, assign15310_e10209_d_n10, assign15310_e10209_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15310_e10205: f64 = (locals.var_nin * locals.var_nin);
        let assign15310_e10207: f64 = (assign15310_e10205 * locals.var_nsti_p2);
        (assign15310_e10207, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn13,)
    }
};
        locals.var_costi1 = assign15310_e10209;
        locals.var_costi1_dn0 = assign15310_e10209_d_n0;
        locals.var_costi1_dn2 = assign15310_e10209_d_n2;
        locals.var_costi1_dn4 = assign15310_e10209_d_n4;
        locals.var_costi1_dn5 = assign15310_e10209_d_n5;
        locals.var_costi1_dn6 = assign15310_e10209_d_n6;
        locals.var_costi1_dn7 = assign15310_e10209_d_n7;
        locals.var_costi1_dn8 = assign15310_e10209_d_n8;
        locals.var_costi1_dn9 = assign15310_e10209_d_n9;
        locals.var_costi1_dn10 = assign15310_e10209_d_n10;
        locals.var_costi1_dn13 = assign15310_e10209_d_n13;

        let (assign15320_e10217, assign15320_e10217_d_n0, assign15320_e10217_d_n2, assign15320_e10217_d_n4, assign15320_e10217_d_n5, assign15320_e10217_d_n6, assign15320_e10217_d_n7, assign15320_e10217_d_n8, assign15320_e10217_d_n9, assign15320_e10217_d_n10, assign15320_e10217_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15320_e10214: f64 = (p.p448 * locals.var_tdiff);
        let assign15320_e10215: f64 = (p.p447 + assign15320_e10214);
        (assign15320_e10215, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn13),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn13,)
    }
};
        locals.var_hbdceff = assign15320_e10217;
        locals.var_hbdceff_dn0 = assign15320_e10217_d_n0;
        locals.var_hbdceff_dn2 = assign15320_e10217_d_n2;
        locals.var_hbdceff_dn4 = assign15320_e10217_d_n4;
        locals.var_hbdceff_dn5 = assign15320_e10217_d_n5;
        locals.var_hbdceff_dn6 = assign15320_e10217_d_n6;
        locals.var_hbdceff_dn7 = assign15320_e10217_d_n7;
        locals.var_hbdceff_dn8 = assign15320_e10217_d_n8;
        locals.var_hbdceff_dn9 = assign15320_e10217_d_n9;
        locals.var_hbdceff_dn10 = assign15320_e10217_d_n10;
        locals.var_hbdceff_dn13 = assign15320_e10217_d_n13;

        let (assign15330_e10221,) = {
    if (locals.var_guard289 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15330_e10221;

        let assign15360_e10234: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard324 = assign15360_e10234;

        let (assign15370_e10240,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard324 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15370_e10240;

        let assign15380_e10243: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard325 = assign15380_e10243;

        let (assign15390_e10249,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard325 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15390_e10249;

        let assign15400_e10252: f64 = if locals.var_uc_cordrift > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign15400_e10252;

        let (assign15410_e10265, assign15410_e10265_d_n0, assign15410_e10265_d_n2, assign15410_e10265_d_n4, assign15410_e10265_d_n5, assign15410_e10265_d_n6, assign15410_e10265_d_n7, assign15410_e10265_d_n8, assign15410_e10265_d_n9, assign15410_e10265_d_n10, assign15410_e10265_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let (assign15410_e10263, assign15410_e10263_d_n0, assign15410_e10263_d_n2, assign15410_e10263_d_n4, assign15410_e10263_d_n5, assign15410_e10263_d_n6, assign15410_e10263_d_n7, assign15410_e10263_d_n8, assign15410_e10263_d_n9, assign15410_e10263_d_n10, assign15410_e10263_d_n13,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15410_e10262: f64 = (locals.var_tratio).powf(p.p416);
                (assign15410_e10262, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn13)) } } else { (assign15410_e10262 * (p.p416 * (locals.var_tratio_dn13 / locals.var_tratio))) },)
            }
        };
        (assign15410_e10263, assign15410_e10263_d_n0, assign15410_e10263_d_n2, assign15410_e10263_d_n4, assign15410_e10263_d_n5, assign15410_e10263_d_n6, assign15410_e10263_d_n7, assign15410_e10263_d_n8, assign15410_e10263_d_n9, assign15410_e10263_d_n10, assign15410_e10263_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign15410_e10265;
        locals.var_t1_dn0 = assign15410_e10265_d_n0;
        locals.var_t1_dn2 = assign15410_e10265_d_n2;
        locals.var_t1_dn4 = assign15410_e10265_d_n4;
        locals.var_t1_dn5 = assign15410_e10265_d_n5;
        locals.var_t1_dn6 = assign15410_e10265_d_n6;
        locals.var_t1_dn7 = assign15410_e10265_d_n7;
        locals.var_t1_dn8 = assign15410_e10265_d_n8;
        locals.var_t1_dn9 = assign15410_e10265_d_n9;
        locals.var_t1_dn10 = assign15410_e10265_d_n10;
        locals.var_t1_dn13 = assign15410_e10265_d_n13;

        let (assign15420_e10273, assign15420_e10273_d_n0, assign15420_e10273_d_n2, assign15420_e10273_d_n4, assign15420_e10273_d_n5, assign15420_e10273_d_n6, assign15420_e10273_d_n7, assign15420_e10273_d_n8, assign15420_e10273_d_n9, assign15420_e10273_d_n10, assign15420_e10273_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15420_e10271: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign15420_e10271, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn13,)
    }
};
        locals.var_rrdrmues = assign15420_e10273;
        locals.var_rrdrmues_dn0 = assign15420_e10273_d_n0;
        locals.var_rrdrmues_dn2 = assign15420_e10273_d_n2;
        locals.var_rrdrmues_dn4 = assign15420_e10273_d_n4;
        locals.var_rrdrmues_dn5 = assign15420_e10273_d_n5;
        locals.var_rrdrmues_dn6 = assign15420_e10273_d_n6;
        locals.var_rrdrmues_dn7 = assign15420_e10273_d_n7;
        locals.var_rrdrmues_dn8 = assign15420_e10273_d_n8;
        locals.var_rrdrmues_dn9 = assign15420_e10273_d_n9;
        locals.var_rrdrmues_dn10 = assign15420_e10273_d_n10;
        locals.var_rrdrmues_dn13 = assign15420_e10273_d_n13;

        let (assign15430_e10295, assign15430_e10295_d_n0, assign15430_e10295_d_n2, assign15430_e10295_d_n4, assign15430_e10295_d_n5, assign15430_e10295_d_n6, assign15430_e10295_d_n7, assign15430_e10295_d_n8, assign15430_e10295_d_n9, assign15430_e10295_d_n10, assign15430_e10295_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15430_e10280: f64 = (0.4 * locals.var_tratio);
        let assign15430_e10281: f64 = (1.8 + assign15430_e10280);
        let assign15430_e10284: f64 = (0.1 * locals.var_tratio);
        let assign15430_e10286: f64 = (assign15430_e10284 * locals.var_tratio);
        let assign15430_e10287: f64 = (assign15430_e10281 + assign15430_e10286);
        let assign15430_e10291: f64 = (1.0 - locals.var_tratio);
        let assign15430_e10292: f64 = (p.p418 * assign15430_e10291);
        let assign15430_e10293: f64 = (assign15430_e10287 - assign15430_e10292);
        (assign15430_e10293, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign15430_e10284 * locals.var_tratio_dn13))) - (p.p418 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15430_e10295;
        locals.var_t0_dn0 = assign15430_e10295_d_n0;
        locals.var_t0_dn2 = assign15430_e10295_d_n2;
        locals.var_t0_dn4 = assign15430_e10295_d_n4;
        locals.var_t0_dn5 = assign15430_e10295_d_n5;
        locals.var_t0_dn6 = assign15430_e10295_d_n6;
        locals.var_t0_dn7 = assign15430_e10295_d_n7;
        locals.var_t0_dn8 = assign15430_e10295_d_n8;
        locals.var_t0_dn9 = assign15430_e10295_d_n9;
        locals.var_t0_dn10 = assign15430_e10295_d_n10;
        locals.var_t0_dn13 = assign15430_e10295_d_n13;

        let (assign15440_e10303, assign15440_e10303_d_n0, assign15440_e10303_d_n2, assign15440_e10303_d_n4, assign15440_e10303_d_n5, assign15440_e10303_d_n6, assign15440_e10303_d_n7, assign15440_e10303_d_n8, assign15440_e10303_d_n9, assign15440_e10303_d_n10, assign15440_e10303_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15440_e10301: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign15440_e10301, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn13,)
    }
};
        locals.var_rrdrvmaxs = assign15440_e10303;
        locals.var_rrdrvmaxs_dn0 = assign15440_e10303_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign15440_e10303_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign15440_e10303_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign15440_e10303_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign15440_e10303_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign15440_e10303_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign15440_e10303_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign15440_e10303_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign15440_e10303_d_n10;
        locals.var_rrdrvmaxs_dn13 = assign15440_e10303_d_n13;

        let (assign15450_e10315, assign15450_e10315_d_n0, assign15450_e10315_d_n2, assign15450_e10315_d_n4, assign15450_e10315_d_n5, assign15450_e10315_d_n6, assign15450_e10315_d_n7, assign15450_e10315_d_n8, assign15450_e10315_d_n9, assign15450_e10315_d_n10, assign15450_e10315_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15450_e10311: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign15450_e10312: f64 = (p.p439 * assign15450_e10311);
        let assign15450_e10313: f64 = (locals.var_uc_rdrbb_s + assign15450_e10312);
        (assign15450_e10313, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn13 + (p.p439 * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn13,)
    }
};
        locals.var_uc_rdrbb_s = assign15450_e10315;
        locals.var_uc_rdrbb_s_dn0 = assign15450_e10315_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign15450_e10315_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign15450_e10315_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign15450_e10315_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign15450_e10315_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign15450_e10315_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign15450_e10315_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign15450_e10315_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign15450_e10315_d_n10;
        locals.var_uc_rdrbb_s_dn13 = assign15450_e10315_d_n13;

        let (assign15460_e10328, assign15460_e10328_d_n0, assign15460_e10328_d_n2, assign15460_e10328_d_n4, assign15460_e10328_d_n5, assign15460_e10328_d_n6, assign15460_e10328_d_n7, assign15460_e10328_d_n8, assign15460_e10328_d_n9, assign15460_e10328_d_n10, assign15460_e10328_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let (assign15460_e10326, assign15460_e10326_d_n0, assign15460_e10326_d_n2, assign15460_e10326_d_n4, assign15460_e10326_d_n5, assign15460_e10326_d_n6, assign15460_e10326_d_n7, assign15460_e10326_d_n8, assign15460_e10326_d_n9, assign15460_e10326_d_n10, assign15460_e10326_d_n13,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15460_e10325: f64 = (locals.var_tratio).powf(p.p415);
                (assign15460_e10325, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn13)) } } else { (assign15460_e10325 * (p.p415 * (locals.var_tratio_dn13 / locals.var_tratio))) },)
            }
        };
        (assign15460_e10326, assign15460_e10326_d_n0, assign15460_e10326_d_n2, assign15460_e10326_d_n4, assign15460_e10326_d_n5, assign15460_e10326_d_n6, assign15460_e10326_d_n7, assign15460_e10326_d_n8, assign15460_e10326_d_n9, assign15460_e10326_d_n10, assign15460_e10326_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign15460_e10328;
        locals.var_t1_dn0 = assign15460_e10328_d_n0;
        locals.var_t1_dn2 = assign15460_e10328_d_n2;
        locals.var_t1_dn4 = assign15460_e10328_d_n4;
        locals.var_t1_dn5 = assign15460_e10328_d_n5;
        locals.var_t1_dn6 = assign15460_e10328_d_n6;
        locals.var_t1_dn7 = assign15460_e10328_d_n7;
        locals.var_t1_dn8 = assign15460_e10328_d_n8;
        locals.var_t1_dn9 = assign15460_e10328_d_n9;
        locals.var_t1_dn10 = assign15460_e10328_d_n10;
        locals.var_t1_dn13 = assign15460_e10328_d_n13;

    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15470_e10336, assign15470_e10336_d_n0, assign15470_e10336_d_n2, assign15470_e10336_d_n4, assign15470_e10336_d_n5, assign15470_e10336_d_n6, assign15470_e10336_d_n7, assign15470_e10336_d_n8, assign15470_e10336_d_n9, assign15470_e10336_d_n10, assign15470_e10336_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15470_e10334: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign15470_e10334, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn13,)
    }
};
        locals.var_rrdrmue = assign15470_e10336;
        locals.var_rrdrmue_dn0 = assign15470_e10336_d_n0;
        locals.var_rrdrmue_dn2 = assign15470_e10336_d_n2;
        locals.var_rrdrmue_dn4 = assign15470_e10336_d_n4;
        locals.var_rrdrmue_dn5 = assign15470_e10336_d_n5;
        locals.var_rrdrmue_dn6 = assign15470_e10336_d_n6;
        locals.var_rrdrmue_dn7 = assign15470_e10336_d_n7;
        locals.var_rrdrmue_dn8 = assign15470_e10336_d_n8;
        locals.var_rrdrmue_dn9 = assign15470_e10336_d_n9;
        locals.var_rrdrmue_dn10 = assign15470_e10336_d_n10;
        locals.var_rrdrmue_dn13 = assign15470_e10336_d_n13;

        let (assign15480_e10358, assign15480_e10358_d_n0, assign15480_e10358_d_n2, assign15480_e10358_d_n4, assign15480_e10358_d_n5, assign15480_e10358_d_n6, assign15480_e10358_d_n7, assign15480_e10358_d_n8, assign15480_e10358_d_n9, assign15480_e10358_d_n10, assign15480_e10358_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15480_e10343: f64 = (0.4 * locals.var_tratio);
        let assign15480_e10344: f64 = (1.8 + assign15480_e10343);
        let assign15480_e10347: f64 = (0.1 * locals.var_tratio);
        let assign15480_e10349: f64 = (assign15480_e10347 * locals.var_tratio);
        let assign15480_e10350: f64 = (assign15480_e10344 + assign15480_e10349);
        let assign15480_e10354: f64 = (1.0 - locals.var_tratio);
        let assign15480_e10355: f64 = (p.p417 * assign15480_e10354);
        let assign15480_e10356: f64 = (assign15480_e10350 - assign15480_e10355);
        (assign15480_e10356, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign15480_e10347 * locals.var_tratio_dn13))) - (p.p417 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15480_e10358;
        locals.var_t0_dn0 = assign15480_e10358_d_n0;
        locals.var_t0_dn2 = assign15480_e10358_d_n2;
        locals.var_t0_dn4 = assign15480_e10358_d_n4;
        locals.var_t0_dn5 = assign15480_e10358_d_n5;
        locals.var_t0_dn6 = assign15480_e10358_d_n6;
        locals.var_t0_dn7 = assign15480_e10358_d_n7;
        locals.var_t0_dn8 = assign15480_e10358_d_n8;
        locals.var_t0_dn9 = assign15480_e10358_d_n9;
        locals.var_t0_dn10 = assign15480_e10358_d_n10;
        locals.var_t0_dn13 = assign15480_e10358_d_n13;

        let (assign15490_e10366, assign15490_e10366_d_n0, assign15490_e10366_d_n2, assign15490_e10366_d_n4, assign15490_e10366_d_n5, assign15490_e10366_d_n6, assign15490_e10366_d_n7, assign15490_e10366_d_n8, assign15490_e10366_d_n9, assign15490_e10366_d_n10, assign15490_e10366_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15490_e10364: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign15490_e10364, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn13,)
    }
};
        locals.var_rrdrvmax = assign15490_e10366;
        locals.var_rrdrvmax_dn0 = assign15490_e10366_d_n0;
        locals.var_rrdrvmax_dn2 = assign15490_e10366_d_n2;
        locals.var_rrdrvmax_dn4 = assign15490_e10366_d_n4;
        locals.var_rrdrvmax_dn5 = assign15490_e10366_d_n5;
        locals.var_rrdrvmax_dn6 = assign15490_e10366_d_n6;
        locals.var_rrdrvmax_dn7 = assign15490_e10366_d_n7;
        locals.var_rrdrvmax_dn8 = assign15490_e10366_d_n8;
        locals.var_rrdrvmax_dn9 = assign15490_e10366_d_n9;
        locals.var_rrdrvmax_dn10 = assign15490_e10366_d_n10;
        locals.var_rrdrvmax_dn13 = assign15490_e10366_d_n13;

        let (assign15500_e10378, assign15500_e10378_d_n0, assign15500_e10378_d_n2, assign15500_e10378_d_n4, assign15500_e10378_d_n5, assign15500_e10378_d_n6, assign15500_e10378_d_n7, assign15500_e10378_d_n8, assign15500_e10378_d_n9, assign15500_e10378_d_n10, assign15500_e10378_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) {
        let assign15500_e10374: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign15500_e10375: f64 = (p.p438 * assign15500_e10374);
        let assign15500_e10376: f64 = (locals.var_uc_rdrbb + assign15500_e10375);
        (assign15500_e10376, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn13 + (p.p438 * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn13,)
    }
};
        locals.var_uc_rdrbb = assign15500_e10378;
        locals.var_uc_rdrbb_dn0 = assign15500_e10378_d_n0;
        locals.var_uc_rdrbb_dn2 = assign15500_e10378_d_n2;
        locals.var_uc_rdrbb_dn4 = assign15500_e10378_d_n4;
        locals.var_uc_rdrbb_dn5 = assign15500_e10378_d_n5;
        locals.var_uc_rdrbb_dn6 = assign15500_e10378_d_n6;
        locals.var_uc_rdrbb_dn7 = assign15500_e10378_d_n7;
        locals.var_uc_rdrbb_dn8 = assign15500_e10378_d_n8;
        locals.var_uc_rdrbb_dn9 = assign15500_e10378_d_n9;
        locals.var_uc_rdrbb_dn10 = assign15500_e10378_d_n10;
        locals.var_uc_rdrbb_dn13 = assign15500_e10378_d_n13;

        let assign15520_e10386: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard328 = assign15520_e10386;

        let (assign15530_e10394, assign15530_e10394_d_n0, assign15530_e10394_d_n2, assign15530_e10394_d_n4, assign15530_e10394_d_n5, assign15530_e10394_d_n6, assign15530_e10394_d_n7, assign15530_e10394_d_n8, assign15530_e10394_d_n9, assign15530_e10394_d_n10, assign15530_e10394_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard326 != 0.0)) && (locals.var_guard328 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn13,)
    }
};
        locals.var_uc_rdrbb = assign15530_e10394;
        locals.var_uc_rdrbb_dn0 = assign15530_e10394_d_n0;
        locals.var_uc_rdrbb_dn2 = assign15530_e10394_d_n2;
        locals.var_uc_rdrbb_dn4 = assign15530_e10394_d_n4;
        locals.var_uc_rdrbb_dn5 = assign15530_e10394_d_n5;
        locals.var_uc_rdrbb_dn6 = assign15530_e10394_d_n6;
        locals.var_uc_rdrbb_dn7 = assign15530_e10394_d_n7;
        locals.var_uc_rdrbb_dn8 = assign15530_e10394_d_n8;
        locals.var_uc_rdrbb_dn9 = assign15530_e10394_d_n9;
        locals.var_uc_rdrbb_dn10 = assign15530_e10394_d_n10;
        locals.var_uc_rdrbb_dn13 = assign15530_e10394_d_n13;

        let (assign15540_e10400, assign15540_e10400_d_n0, assign15540_e10400_d_n2, assign15540_e10400_d_n4, assign15540_e10400_d_n5, assign15540_e10400_d_n6, assign15540_e10400_d_n7, assign15540_e10400_d_n8, assign15540_e10400_d_n9, assign15540_e10400_d_n10, assign15540_e10400_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15540_e10398: f64 = (locals.var_tratio * locals.var_tratio);
        (assign15540_e10398, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn13 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign15540_e10400;
        locals.var_t0_dn0 = assign15540_e10400_d_n0;
        locals.var_t0_dn2 = assign15540_e10400_d_n2;
        locals.var_t0_dn4 = assign15540_e10400_d_n4;
        locals.var_t0_dn5 = assign15540_e10400_d_n5;
        locals.var_t0_dn6 = assign15540_e10400_d_n6;
        locals.var_t0_dn7 = assign15540_e10400_d_n7;
        locals.var_t0_dn8 = assign15540_e10400_d_n8;
        locals.var_t0_dn9 = assign15540_e10400_d_n9;
        locals.var_t0_dn10 = assign15540_e10400_d_n10;
        locals.var_t0_dn13 = assign15540_e10400_d_n13;

        let (assign15550_e10419, assign15550_e10419_d_n0, assign15550_e10419_d_n2, assign15550_e10419_d_n4, assign15550_e10419_d_n5, assign15550_e10419_d_n6, assign15550_e10419_d_n7, assign15550_e10419_d_n8, assign15550_e10419_d_n9, assign15550_e10419_d_n10, assign15550_e10419_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15550_e10405: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15550_e10408: f64 = (locals.var_eg * locals.var_beta);
        let assign15550_e10409: f64 = (assign15550_e10405 - assign15550_e10408);
        let assign15550_e10412: f64 = (p.p499 * locals.var_log_tratio);
        let assign15550_e10413: f64 = (assign15550_e10409 + assign15550_e10412);
        let assign15550_e10415: f64 = (assign15550_e10413 / locals.var_uc_njd);
        let assign15550_e10416: f64 = (assign15550_e10415).exp();
        let assign15550_e10417: f64 = (locals.var_uc_js0d * assign15550_e10416);
        (assign15550_e10417, (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15550_e10416 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign15550_e10419;
        locals.var_js_dn0 = assign15550_e10419_d_n0;
        locals.var_js_dn2 = assign15550_e10419_d_n2;
        locals.var_js_dn4 = assign15550_e10419_d_n4;
        locals.var_js_dn5 = assign15550_e10419_d_n5;
        locals.var_js_dn6 = assign15550_e10419_d_n6;
        locals.var_js_dn7 = assign15550_e10419_d_n7;
        locals.var_js_dn8 = assign15550_e10419_d_n8;
        locals.var_js_dn9 = assign15550_e10419_d_n9;
        locals.var_js_dn10 = assign15550_e10419_d_n10;
        locals.var_js_dn13 = assign15550_e10419_d_n13;

        let (assign15560_e10438, assign15560_e10438_d_n0, assign15560_e10438_d_n2, assign15560_e10438_d_n4, assign15560_e10438_d_n5, assign15560_e10438_d_n6, assign15560_e10438_d_n7, assign15560_e10438_d_n8, assign15560_e10438_d_n9, assign15560_e10438_d_n10, assign15560_e10438_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15560_e10424: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15560_e10427: f64 = (locals.var_eg * locals.var_beta);
        let assign15560_e10428: f64 = (assign15560_e10424 - assign15560_e10427);
        let assign15560_e10431: f64 = (p.p499 * locals.var_log_tratio);
        let assign15560_e10432: f64 = (assign15560_e10428 + assign15560_e10431);
        let assign15560_e10434: f64 = (assign15560_e10432 / p.p497);
        let assign15560_e10435: f64 = (assign15560_e10434).exp();
        let assign15560_e10436: f64 = (locals.var_uc_js0swd * assign15560_e10435);
        (assign15560_e10436, (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign15560_e10435 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign15560_e10438;
        locals.var_jssw_dn0 = assign15560_e10438_d_n0;
        locals.var_jssw_dn2 = assign15560_e10438_d_n2;
        locals.var_jssw_dn4 = assign15560_e10438_d_n4;
        locals.var_jssw_dn5 = assign15560_e10438_d_n5;
        locals.var_jssw_dn6 = assign15560_e10438_d_n6;
        locals.var_jssw_dn7 = assign15560_e10438_d_n7;
        locals.var_jssw_dn8 = assign15560_e10438_d_n8;
        locals.var_jssw_dn9 = assign15560_e10438_d_n9;
        locals.var_jssw_dn10 = assign15560_e10438_d_n10;
        locals.var_jssw_dn13 = assign15560_e10438_d_n13;

        let (assign15570_e10457, assign15570_e10457_d_n0, assign15570_e10457_d_n2, assign15570_e10457_d_n4, assign15570_e10457_d_n5, assign15570_e10457_d_n6, assign15570_e10457_d_n7, assign15570_e10457_d_n8, assign15570_e10457_d_n9, assign15570_e10457_d_n10, assign15570_e10457_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15570_e10443: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15570_e10446: f64 = (locals.var_eg * locals.var_beta);
        let assign15570_e10447: f64 = (assign15570_e10443 - assign15570_e10446);
        let assign15570_e10450: f64 = (p.p499 * locals.var_log_tratio);
        let assign15570_e10451: f64 = (assign15570_e10447 + assign15570_e10450);
        let assign15570_e10453: f64 = (assign15570_e10451 / p.p498);
        let assign15570_e10454: f64 = (assign15570_e10453).exp();
        let assign15570_e10455: f64 = (p.p495 * assign15570_e10454);
        (assign15570_e10455, (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign15570_e10454 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign15570_e10457;
        locals.var_jsswg_dn0 = assign15570_e10457_d_n0;
        locals.var_jsswg_dn2 = assign15570_e10457_d_n2;
        locals.var_jsswg_dn4 = assign15570_e10457_d_n4;
        locals.var_jsswg_dn5 = assign15570_e10457_d_n5;
        locals.var_jsswg_dn6 = assign15570_e10457_d_n6;
        locals.var_jsswg_dn7 = assign15570_e10457_d_n7;
        locals.var_jsswg_dn8 = assign15570_e10457_d_n8;
        locals.var_jsswg_dn9 = assign15570_e10457_d_n9;
        locals.var_jsswg_dn10 = assign15570_e10457_d_n10;
        locals.var_jsswg_dn13 = assign15570_e10457_d_n13;

        let (assign15580_e10476, assign15580_e10476_d_n0, assign15580_e10476_d_n2, assign15580_e10476_d_n4, assign15580_e10476_d_n5, assign15580_e10476_d_n6, assign15580_e10476_d_n7, assign15580_e10476_d_n8, assign15580_e10476_d_n9, assign15580_e10476_d_n10, assign15580_e10476_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15580_e10462: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15580_e10465: f64 = (locals.var_eg * locals.var_beta);
        let assign15580_e10466: f64 = (assign15580_e10462 - assign15580_e10465);
        let assign15580_e10469: f64 = (p.p509 * locals.var_log_tratio);
        let assign15580_e10470: f64 = (assign15580_e10466 + assign15580_e10469);
        let assign15580_e10472: f64 = (assign15580_e10470 / locals.var_uc_njd);
        let assign15580_e10473: f64 = (assign15580_e10472).exp();
        let assign15580_e10474: f64 = (locals.var_uc_js0d * assign15580_e10473);
        (assign15580_e10474, (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15580_e10473 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign15580_e10476;
        locals.var_js2_dn0 = assign15580_e10476_d_n0;
        locals.var_js2_dn2 = assign15580_e10476_d_n2;
        locals.var_js2_dn4 = assign15580_e10476_d_n4;
        locals.var_js2_dn5 = assign15580_e10476_d_n5;
        locals.var_js2_dn6 = assign15580_e10476_d_n6;
        locals.var_js2_dn7 = assign15580_e10476_d_n7;
        locals.var_js2_dn8 = assign15580_e10476_d_n8;
        locals.var_js2_dn9 = assign15580_e10476_d_n9;
        locals.var_js2_dn10 = assign15580_e10476_d_n10;
        locals.var_js2_dn13 = assign15580_e10476_d_n13;

        let (assign15590_e10495, assign15590_e10495_d_n0, assign15590_e10495_d_n2, assign15590_e10495_d_n4, assign15590_e10495_d_n5, assign15590_e10495_d_n6, assign15590_e10495_d_n7, assign15590_e10495_d_n8, assign15590_e10495_d_n9, assign15590_e10495_d_n10, assign15590_e10495_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15590_e10481: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15590_e10484: f64 = (locals.var_eg * locals.var_beta);
        let assign15590_e10485: f64 = (assign15590_e10481 - assign15590_e10484);
        let assign15590_e10488: f64 = (p.p509 * locals.var_log_tratio);
        let assign15590_e10489: f64 = (assign15590_e10485 + assign15590_e10488);
        let assign15590_e10491: f64 = (assign15590_e10489 / p.p497);
        let assign15590_e10492: f64 = (assign15590_e10491).exp();
        let assign15590_e10493: f64 = (locals.var_uc_js0swd * assign15590_e10492);
        (assign15590_e10493, (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign15590_e10492 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign15590_e10495;
        locals.var_jssw2_dn0 = assign15590_e10495_d_n0;
        locals.var_jssw2_dn2 = assign15590_e10495_d_n2;
        locals.var_jssw2_dn4 = assign15590_e10495_d_n4;
        locals.var_jssw2_dn5 = assign15590_e10495_d_n5;
        locals.var_jssw2_dn6 = assign15590_e10495_d_n6;
        locals.var_jssw2_dn7 = assign15590_e10495_d_n7;
        locals.var_jssw2_dn8 = assign15590_e10495_d_n8;
        locals.var_jssw2_dn9 = assign15590_e10495_d_n9;
        locals.var_jssw2_dn10 = assign15590_e10495_d_n10;
        locals.var_jssw2_dn13 = assign15590_e10495_d_n13;

        let (assign15600_e10514, assign15600_e10514_d_n0, assign15600_e10514_d_n2, assign15600_e10514_d_n4, assign15600_e10514_d_n5, assign15600_e10514_d_n6, assign15600_e10514_d_n7, assign15600_e10514_d_n8, assign15600_e10514_d_n9, assign15600_e10514_d_n10, assign15600_e10514_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15600_e10500: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15600_e10503: f64 = (locals.var_eg * locals.var_beta);
        let assign15600_e10504: f64 = (assign15600_e10500 - assign15600_e10503);
        let assign15600_e10507: f64 = (p.p509 * locals.var_log_tratio);
        let assign15600_e10508: f64 = (assign15600_e10504 + assign15600_e10507);
        let assign15600_e10510: f64 = (assign15600_e10508 / p.p498);
        let assign15600_e10511: f64 = (assign15600_e10510).exp();
        let assign15600_e10512: f64 = (p.p495 * assign15600_e10511);
        (assign15600_e10512, (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign15600_e10511 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign15600_e10514;
        locals.var_jsswg2_dn0 = assign15600_e10514_d_n0;
        locals.var_jsswg2_dn2 = assign15600_e10514_d_n2;
        locals.var_jsswg2_dn4 = assign15600_e10514_d_n4;
        locals.var_jsswg2_dn5 = assign15600_e10514_d_n5;
        locals.var_jsswg2_dn6 = assign15600_e10514_d_n6;
        locals.var_jsswg2_dn7 = assign15600_e10514_d_n7;
        locals.var_jsswg2_dn8 = assign15600_e10514_d_n8;
        locals.var_jsswg2_dn9 = assign15600_e10514_d_n9;
        locals.var_jsswg2_dn10 = assign15600_e10514_d_n10;
        locals.var_jsswg2_dn13 = assign15600_e10514_d_n13;

        let assign15610_e10517: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard329 = assign15610_e10517;

        let assign15620_e10520: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard330 = assign15620_e10520;

        let (assign15630_e10530, assign15630_e10530_d_n0, assign15630_e10530_d_n2, assign15630_e10530_d_n4, assign15630_e10530_d_n5, assign15630_e10530_d_n6, assign15630_e10530_d_n7, assign15630_e10530_d_n8, assign15630_e10530_d_n9, assign15630_e10530_d_n10, assign15630_e10530_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15630_e10528: f64 = (p.p13 * locals.var_js);
        (assign15630_e10528, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign15630_e10530;
        locals.var_isbd_btm_dn0 = assign15630_e10530_d_n0;
        locals.var_isbd_btm_dn2 = assign15630_e10530_d_n2;
        locals.var_isbd_btm_dn4 = assign15630_e10530_d_n4;
        locals.var_isbd_btm_dn5 = assign15630_e10530_d_n5;
        locals.var_isbd_btm_dn6 = assign15630_e10530_d_n6;
        locals.var_isbd_btm_dn7 = assign15630_e10530_d_n7;
        locals.var_isbd_btm_dn8 = assign15630_e10530_d_n8;
        locals.var_isbd_btm_dn9 = assign15630_e10530_d_n9;
        locals.var_isbd_btm_dn10 = assign15630_e10530_d_n10;
        locals.var_isbd_btm_dn13 = assign15630_e10530_d_n13;

        let (assign15640_e10540, assign15640_e10540_d_n0, assign15640_e10540_d_n2, assign15640_e10540_d_n4, assign15640_e10540_d_n5, assign15640_e10540_d_n6, assign15640_e10540_d_n7, assign15640_e10540_d_n8, assign15640_e10540_d_n9, assign15640_e10540_d_n10, assign15640_e10540_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15640_e10538: f64 = (p.p13 * locals.var_js2);
        (assign15640_e10538, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign15640_e10540;
        locals.var_isbd2_btm_dn0 = assign15640_e10540_d_n0;
        locals.var_isbd2_btm_dn2 = assign15640_e10540_d_n2;
        locals.var_isbd2_btm_dn4 = assign15640_e10540_d_n4;
        locals.var_isbd2_btm_dn5 = assign15640_e10540_d_n5;
        locals.var_isbd2_btm_dn6 = assign15640_e10540_d_n6;
        locals.var_isbd2_btm_dn7 = assign15640_e10540_d_n7;
        locals.var_isbd2_btm_dn8 = assign15640_e10540_d_n8;
        locals.var_isbd2_btm_dn9 = assign15640_e10540_d_n9;
        locals.var_isbd2_btm_dn10 = assign15640_e10540_d_n10;
        locals.var_isbd2_btm_dn13 = assign15640_e10540_d_n13;

        let (assign15650_e10552, assign15650_e10552_d_n0, assign15650_e10552_d_n2, assign15650_e10552_d_n4, assign15650_e10552_d_n5, assign15650_e10552_d_n6, assign15650_e10552_d_n7, assign15650_e10552_d_n8, assign15650_e10552_d_n9, assign15650_e10552_d_n10, assign15650_e10552_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15650_e10548: f64 = (p.p15 - locals.var_weff_nf);
        let assign15650_e10550: f64 = (assign15650_e10548 * locals.var_jssw);
        (assign15650_e10550, (assign15650_e10548 * locals.var_jssw_dn0), (assign15650_e10548 * locals.var_jssw_dn2), (assign15650_e10548 * locals.var_jssw_dn4), (assign15650_e10548 * locals.var_jssw_dn5), (assign15650_e10548 * locals.var_jssw_dn6), (assign15650_e10548 * locals.var_jssw_dn7), (assign15650_e10548 * locals.var_jssw_dn8), (assign15650_e10548 * locals.var_jssw_dn9), (assign15650_e10548 * locals.var_jssw_dn10), (assign15650_e10548 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign15650_e10552;
        locals.var_isbd_sws_dn0 = assign15650_e10552_d_n0;
        locals.var_isbd_sws_dn2 = assign15650_e10552_d_n2;
        locals.var_isbd_sws_dn4 = assign15650_e10552_d_n4;
        locals.var_isbd_sws_dn5 = assign15650_e10552_d_n5;
        locals.var_isbd_sws_dn6 = assign15650_e10552_d_n6;
        locals.var_isbd_sws_dn7 = assign15650_e10552_d_n7;
        locals.var_isbd_sws_dn8 = assign15650_e10552_d_n8;
        locals.var_isbd_sws_dn9 = assign15650_e10552_d_n9;
        locals.var_isbd_sws_dn10 = assign15650_e10552_d_n10;
        locals.var_isbd_sws_dn13 = assign15650_e10552_d_n13;

        let (assign15660_e10564, assign15660_e10564_d_n0, assign15660_e10564_d_n2, assign15660_e10564_d_n4, assign15660_e10564_d_n5, assign15660_e10564_d_n6, assign15660_e10564_d_n7, assign15660_e10564_d_n8, assign15660_e10564_d_n9, assign15660_e10564_d_n10, assign15660_e10564_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15660_e10560: f64 = (p.p15 - locals.var_weff_nf);
        let assign15660_e10562: f64 = (assign15660_e10560 * locals.var_jssw2);
        (assign15660_e10562, (assign15660_e10560 * locals.var_jssw2_dn0), (assign15660_e10560 * locals.var_jssw2_dn2), (assign15660_e10560 * locals.var_jssw2_dn4), (assign15660_e10560 * locals.var_jssw2_dn5), (assign15660_e10560 * locals.var_jssw2_dn6), (assign15660_e10560 * locals.var_jssw2_dn7), (assign15660_e10560 * locals.var_jssw2_dn8), (assign15660_e10560 * locals.var_jssw2_dn9), (assign15660_e10560 * locals.var_jssw2_dn10), (assign15660_e10560 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign15660_e10564;
        locals.var_isbd2_sws_dn0 = assign15660_e10564_d_n0;
        locals.var_isbd2_sws_dn2 = assign15660_e10564_d_n2;
        locals.var_isbd2_sws_dn4 = assign15660_e10564_d_n4;
        locals.var_isbd2_sws_dn5 = assign15660_e10564_d_n5;
        locals.var_isbd2_sws_dn6 = assign15660_e10564_d_n6;
        locals.var_isbd2_sws_dn7 = assign15660_e10564_d_n7;
        locals.var_isbd2_sws_dn8 = assign15660_e10564_d_n8;
        locals.var_isbd2_sws_dn9 = assign15660_e10564_d_n9;
        locals.var_isbd2_sws_dn10 = assign15660_e10564_d_n10;
        locals.var_isbd2_sws_dn13 = assign15660_e10564_d_n13;

        let (assign15670_e10574, assign15670_e10574_d_n0, assign15670_e10574_d_n2, assign15670_e10574_d_n4, assign15670_e10574_d_n5, assign15670_e10574_d_n6, assign15670_e10574_d_n7, assign15670_e10574_d_n8, assign15670_e10574_d_n9, assign15670_e10574_d_n10, assign15670_e10574_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15670_e10572: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign15670_e10572, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign15670_e10574;
        locals.var_isbd_swg_dn0 = assign15670_e10574_d_n0;
        locals.var_isbd_swg_dn2 = assign15670_e10574_d_n2;
        locals.var_isbd_swg_dn4 = assign15670_e10574_d_n4;
        locals.var_isbd_swg_dn5 = assign15670_e10574_d_n5;
        locals.var_isbd_swg_dn6 = assign15670_e10574_d_n6;
        locals.var_isbd_swg_dn7 = assign15670_e10574_d_n7;
        locals.var_isbd_swg_dn8 = assign15670_e10574_d_n8;
        locals.var_isbd_swg_dn9 = assign15670_e10574_d_n9;
        locals.var_isbd_swg_dn10 = assign15670_e10574_d_n10;
        locals.var_isbd_swg_dn13 = assign15670_e10574_d_n13;

        let (assign15680_e10584, assign15680_e10584_d_n0, assign15680_e10584_d_n2, assign15680_e10584_d_n4, assign15680_e10584_d_n5, assign15680_e10584_d_n6, assign15680_e10584_d_n7, assign15680_e10584_d_n8, assign15680_e10584_d_n9, assign15680_e10584_d_n10, assign15680_e10584_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign15680_e10582: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign15680_e10582, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign15680_e10584;
        locals.var_isbd2_swg_dn0 = assign15680_e10584_d_n0;
        locals.var_isbd2_swg_dn2 = assign15680_e10584_d_n2;
        locals.var_isbd2_swg_dn4 = assign15680_e10584_d_n4;
        locals.var_isbd2_swg_dn5 = assign15680_e10584_d_n5;
        locals.var_isbd2_swg_dn6 = assign15680_e10584_d_n6;
        locals.var_isbd2_swg_dn7 = assign15680_e10584_d_n7;
        locals.var_isbd2_swg_dn8 = assign15680_e10584_d_n8;
        locals.var_isbd2_swg_dn9 = assign15680_e10584_d_n9;
        locals.var_isbd2_swg_dn10 = assign15680_e10584_d_n10;
        locals.var_isbd2_swg_dn13 = assign15680_e10584_d_n13;

        let (assign15690_e10595, assign15690_e10595_d_n0, assign15690_e10595_d_n2, assign15690_e10595_d_n4, assign15690_e10595_d_n5, assign15690_e10595_d_n6, assign15690_e10595_d_n7, assign15690_e10595_d_n8, assign15690_e10595_d_n9, assign15690_e10595_d_n10, assign15690_e10595_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign15690_e10593: f64 = (p.p13 * locals.var_js);
        (assign15690_e10593, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign15690_e10595;
        locals.var_isbd_btm_dn0 = assign15690_e10595_d_n0;
        locals.var_isbd_btm_dn2 = assign15690_e10595_d_n2;
        locals.var_isbd_btm_dn4 = assign15690_e10595_d_n4;
        locals.var_isbd_btm_dn5 = assign15690_e10595_d_n5;
        locals.var_isbd_btm_dn6 = assign15690_e10595_d_n6;
        locals.var_isbd_btm_dn7 = assign15690_e10595_d_n7;
        locals.var_isbd_btm_dn8 = assign15690_e10595_d_n8;
        locals.var_isbd_btm_dn9 = assign15690_e10595_d_n9;
        locals.var_isbd_btm_dn10 = assign15690_e10595_d_n10;
        locals.var_isbd_btm_dn13 = assign15690_e10595_d_n13;

        let (assign15700_e10606, assign15700_e10606_d_n0, assign15700_e10606_d_n2, assign15700_e10606_d_n4, assign15700_e10606_d_n5, assign15700_e10606_d_n6, assign15700_e10606_d_n7, assign15700_e10606_d_n8, assign15700_e10606_d_n9, assign15700_e10606_d_n10, assign15700_e10606_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign15700_e10604: f64 = (p.p13 * locals.var_js2);
        (assign15700_e10604, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign15700_e10606;
        locals.var_isbd2_btm_dn0 = assign15700_e10606_d_n0;
        locals.var_isbd2_btm_dn2 = assign15700_e10606_d_n2;
        locals.var_isbd2_btm_dn4 = assign15700_e10606_d_n4;
        locals.var_isbd2_btm_dn5 = assign15700_e10606_d_n5;
        locals.var_isbd2_btm_dn6 = assign15700_e10606_d_n6;
        locals.var_isbd2_btm_dn7 = assign15700_e10606_d_n7;
        locals.var_isbd2_btm_dn8 = assign15700_e10606_d_n8;
        locals.var_isbd2_btm_dn9 = assign15700_e10606_d_n9;
        locals.var_isbd2_btm_dn10 = assign15700_e10606_d_n10;
        locals.var_isbd2_btm_dn13 = assign15700_e10606_d_n13;

        let (assign15710_e10615, assign15710_e10615_d_n0, assign15710_e10615_d_n2, assign15710_e10615_d_n4, assign15710_e10615_d_n5, assign15710_e10615_d_n6, assign15710_e10615_d_n7, assign15710_e10615_d_n8, assign15710_e10615_d_n9, assign15710_e10615_d_n10, assign15710_e10615_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign15710_e10615;
        locals.var_isbd_sws_dn0 = assign15710_e10615_d_n0;
        locals.var_isbd_sws_dn2 = assign15710_e10615_d_n2;
        locals.var_isbd_sws_dn4 = assign15710_e10615_d_n4;
        locals.var_isbd_sws_dn5 = assign15710_e10615_d_n5;
        locals.var_isbd_sws_dn6 = assign15710_e10615_d_n6;
        locals.var_isbd_sws_dn7 = assign15710_e10615_d_n7;
        locals.var_isbd_sws_dn8 = assign15710_e10615_d_n8;
        locals.var_isbd_sws_dn9 = assign15710_e10615_d_n9;
        locals.var_isbd_sws_dn10 = assign15710_e10615_d_n10;
        locals.var_isbd_sws_dn13 = assign15710_e10615_d_n13;

        let (assign15720_e10624, assign15720_e10624_d_n0, assign15720_e10624_d_n2, assign15720_e10624_d_n4, assign15720_e10624_d_n5, assign15720_e10624_d_n6, assign15720_e10624_d_n7, assign15720_e10624_d_n8, assign15720_e10624_d_n9, assign15720_e10624_d_n10, assign15720_e10624_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign15720_e10624;
        locals.var_isbd2_sws_dn0 = assign15720_e10624_d_n0;
        locals.var_isbd2_sws_dn2 = assign15720_e10624_d_n2;
        locals.var_isbd2_sws_dn4 = assign15720_e10624_d_n4;
        locals.var_isbd2_sws_dn5 = assign15720_e10624_d_n5;
        locals.var_isbd2_sws_dn6 = assign15720_e10624_d_n6;
        locals.var_isbd2_sws_dn7 = assign15720_e10624_d_n7;
        locals.var_isbd2_sws_dn8 = assign15720_e10624_d_n8;
        locals.var_isbd2_sws_dn9 = assign15720_e10624_d_n9;
        locals.var_isbd2_sws_dn10 = assign15720_e10624_d_n10;
        locals.var_isbd2_sws_dn13 = assign15720_e10624_d_n13;

    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15730_e10635, assign15730_e10635_d_n0, assign15730_e10635_d_n2, assign15730_e10635_d_n4, assign15730_e10635_d_n5, assign15730_e10635_d_n6, assign15730_e10635_d_n7, assign15730_e10635_d_n8, assign15730_e10635_d_n9, assign15730_e10635_d_n10, assign15730_e10635_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign15730_e10633: f64 = (p.p15 * locals.var_jsswg);
        (assign15730_e10633, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign15730_e10635;
        locals.var_isbd_swg_dn0 = assign15730_e10635_d_n0;
        locals.var_isbd_swg_dn2 = assign15730_e10635_d_n2;
        locals.var_isbd_swg_dn4 = assign15730_e10635_d_n4;
        locals.var_isbd_swg_dn5 = assign15730_e10635_d_n5;
        locals.var_isbd_swg_dn6 = assign15730_e10635_d_n6;
        locals.var_isbd_swg_dn7 = assign15730_e10635_d_n7;
        locals.var_isbd_swg_dn8 = assign15730_e10635_d_n8;
        locals.var_isbd_swg_dn9 = assign15730_e10635_d_n9;
        locals.var_isbd_swg_dn10 = assign15730_e10635_d_n10;
        locals.var_isbd_swg_dn13 = assign15730_e10635_d_n13;

        let (assign15740_e10646, assign15740_e10646_d_n0, assign15740_e10646_d_n2, assign15740_e10646_d_n4, assign15740_e10646_d_n5, assign15740_e10646_d_n6, assign15740_e10646_d_n7, assign15740_e10646_d_n8, assign15740_e10646_d_n9, assign15740_e10646_d_n10, assign15740_e10646_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard329 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign15740_e10644: f64 = (p.p15 * locals.var_jsswg2);
        (assign15740_e10644, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign15740_e10646;
        locals.var_isbd2_swg_dn0 = assign15740_e10646_d_n0;
        locals.var_isbd2_swg_dn2 = assign15740_e10646_d_n2;
        locals.var_isbd2_swg_dn4 = assign15740_e10646_d_n4;
        locals.var_isbd2_swg_dn5 = assign15740_e10646_d_n5;
        locals.var_isbd2_swg_dn6 = assign15740_e10646_d_n6;
        locals.var_isbd2_swg_dn7 = assign15740_e10646_d_n7;
        locals.var_isbd2_swg_dn8 = assign15740_e10646_d_n8;
        locals.var_isbd2_swg_dn9 = assign15740_e10646_d_n9;
        locals.var_isbd2_swg_dn10 = assign15740_e10646_d_n10;
        locals.var_isbd2_swg_dn13 = assign15740_e10646_d_n13;

        let (assign15750_e10655, assign15750_e10655_d_n0, assign15750_e10655_d_n2, assign15750_e10655_d_n4, assign15750_e10655_d_n5, assign15750_e10655_d_n6, assign15750_e10655_d_n7, assign15750_e10655_d_n8, assign15750_e10655_d_n9, assign15750_e10655_d_n10, assign15750_e10655_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign15750_e10653: f64 = (p.p13 * locals.var_js);
        (assign15750_e10653, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign15750_e10655;
        locals.var_isbd_btm_dn0 = assign15750_e10655_d_n0;
        locals.var_isbd_btm_dn2 = assign15750_e10655_d_n2;
        locals.var_isbd_btm_dn4 = assign15750_e10655_d_n4;
        locals.var_isbd_btm_dn5 = assign15750_e10655_d_n5;
        locals.var_isbd_btm_dn6 = assign15750_e10655_d_n6;
        locals.var_isbd_btm_dn7 = assign15750_e10655_d_n7;
        locals.var_isbd_btm_dn8 = assign15750_e10655_d_n8;
        locals.var_isbd_btm_dn9 = assign15750_e10655_d_n9;
        locals.var_isbd_btm_dn10 = assign15750_e10655_d_n10;
        locals.var_isbd_btm_dn13 = assign15750_e10655_d_n13;

        let (assign15760_e10664, assign15760_e10664_d_n0, assign15760_e10664_d_n2, assign15760_e10664_d_n4, assign15760_e10664_d_n5, assign15760_e10664_d_n6, assign15760_e10664_d_n7, assign15760_e10664_d_n8, assign15760_e10664_d_n9, assign15760_e10664_d_n10, assign15760_e10664_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign15760_e10662: f64 = (p.p13 * locals.var_js2);
        (assign15760_e10662, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign15760_e10664;
        locals.var_isbd2_btm_dn0 = assign15760_e10664_d_n0;
        locals.var_isbd2_btm_dn2 = assign15760_e10664_d_n2;
        locals.var_isbd2_btm_dn4 = assign15760_e10664_d_n4;
        locals.var_isbd2_btm_dn5 = assign15760_e10664_d_n5;
        locals.var_isbd2_btm_dn6 = assign15760_e10664_d_n6;
        locals.var_isbd2_btm_dn7 = assign15760_e10664_d_n7;
        locals.var_isbd2_btm_dn8 = assign15760_e10664_d_n8;
        locals.var_isbd2_btm_dn9 = assign15760_e10664_d_n9;
        locals.var_isbd2_btm_dn10 = assign15760_e10664_d_n10;
        locals.var_isbd2_btm_dn13 = assign15760_e10664_d_n13;

        let (assign15770_e10673, assign15770_e10673_d_n0, assign15770_e10673_d_n2, assign15770_e10673_d_n4, assign15770_e10673_d_n5, assign15770_e10673_d_n6, assign15770_e10673_d_n7, assign15770_e10673_d_n8, assign15770_e10673_d_n9, assign15770_e10673_d_n10, assign15770_e10673_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign15770_e10671: f64 = (p.p15 * locals.var_jssw);
        (assign15770_e10671, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign15770_e10673;
        locals.var_isbd_sws_dn0 = assign15770_e10673_d_n0;
        locals.var_isbd_sws_dn2 = assign15770_e10673_d_n2;
        locals.var_isbd_sws_dn4 = assign15770_e10673_d_n4;
        locals.var_isbd_sws_dn5 = assign15770_e10673_d_n5;
        locals.var_isbd_sws_dn6 = assign15770_e10673_d_n6;
        locals.var_isbd_sws_dn7 = assign15770_e10673_d_n7;
        locals.var_isbd_sws_dn8 = assign15770_e10673_d_n8;
        locals.var_isbd_sws_dn9 = assign15770_e10673_d_n9;
        locals.var_isbd_sws_dn10 = assign15770_e10673_d_n10;
        locals.var_isbd_sws_dn13 = assign15770_e10673_d_n13;

        let (assign15780_e10682, assign15780_e10682_d_n0, assign15780_e10682_d_n2, assign15780_e10682_d_n4, assign15780_e10682_d_n5, assign15780_e10682_d_n6, assign15780_e10682_d_n7, assign15780_e10682_d_n8, assign15780_e10682_d_n9, assign15780_e10682_d_n10, assign15780_e10682_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign15780_e10680: f64 = (p.p15 * locals.var_jssw2);
        (assign15780_e10680, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign15780_e10682;
        locals.var_isbd2_sws_dn0 = assign15780_e10682_d_n0;
        locals.var_isbd2_sws_dn2 = assign15780_e10682_d_n2;
        locals.var_isbd2_sws_dn4 = assign15780_e10682_d_n4;
        locals.var_isbd2_sws_dn5 = assign15780_e10682_d_n5;
        locals.var_isbd2_sws_dn6 = assign15780_e10682_d_n6;
        locals.var_isbd2_sws_dn7 = assign15780_e10682_d_n7;
        locals.var_isbd2_sws_dn8 = assign15780_e10682_d_n8;
        locals.var_isbd2_sws_dn9 = assign15780_e10682_d_n9;
        locals.var_isbd2_sws_dn10 = assign15780_e10682_d_n10;
        locals.var_isbd2_sws_dn13 = assign15780_e10682_d_n13;

        let (assign15790_e10689, assign15790_e10689_d_n0, assign15790_e10689_d_n2, assign15790_e10689_d_n4, assign15790_e10689_d_n5, assign15790_e10689_d_n6, assign15790_e10689_d_n7, assign15790_e10689_d_n8, assign15790_e10689_d_n9, assign15790_e10689_d_n10, assign15790_e10689_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign15790_e10689;
        locals.var_isbd_swg_dn0 = assign15790_e10689_d_n0;
        locals.var_isbd_swg_dn2 = assign15790_e10689_d_n2;
        locals.var_isbd_swg_dn4 = assign15790_e10689_d_n4;
        locals.var_isbd_swg_dn5 = assign15790_e10689_d_n5;
        locals.var_isbd_swg_dn6 = assign15790_e10689_d_n6;
        locals.var_isbd_swg_dn7 = assign15790_e10689_d_n7;
        locals.var_isbd_swg_dn8 = assign15790_e10689_d_n8;
        locals.var_isbd_swg_dn9 = assign15790_e10689_d_n9;
        locals.var_isbd_swg_dn10 = assign15790_e10689_d_n10;
        locals.var_isbd_swg_dn13 = assign15790_e10689_d_n13;

        let (assign15800_e10696, assign15800_e10696_d_n0, assign15800_e10696_d_n2, assign15800_e10696_d_n4, assign15800_e10696_d_n5, assign15800_e10696_d_n6, assign15800_e10696_d_n7, assign15800_e10696_d_n8, assign15800_e10696_d_n9, assign15800_e10696_d_n10, assign15800_e10696_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard329 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign15800_e10696;
        locals.var_isbd2_swg_dn0 = assign15800_e10696_d_n0;
        locals.var_isbd2_swg_dn2 = assign15800_e10696_d_n2;
        locals.var_isbd2_swg_dn4 = assign15800_e10696_d_n4;
        locals.var_isbd2_swg_dn5 = assign15800_e10696_d_n5;
        locals.var_isbd2_swg_dn6 = assign15800_e10696_d_n6;
        locals.var_isbd2_swg_dn7 = assign15800_e10696_d_n7;
        locals.var_isbd2_swg_dn8 = assign15800_e10696_d_n8;
        locals.var_isbd2_swg_dn9 = assign15800_e10696_d_n9;
        locals.var_isbd2_swg_dn10 = assign15800_e10696_d_n10;
        locals.var_isbd2_swg_dn13 = assign15800_e10696_d_n13;

        let (assign15810_e10704, assign15810_e10704_d_n0, assign15810_e10704_d_n2, assign15810_e10704_d_n4, assign15810_e10704_d_n5, assign15810_e10704_d_n6, assign15810_e10704_d_n7, assign15810_e10704_d_n8, assign15810_e10704_d_n9, assign15810_e10704_d_n10, assign15810_e10704_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15810_e10700: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign15810_e10702: f64 = (assign15810_e10700 + locals.var_isbd_swg);
        (assign15810_e10702, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn13 + locals.var_isbd_sws_dn13) + locals.var_isbd_swg_dn13),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn13,)
    }
};
        locals.var_isbd = assign15810_e10704;
        locals.var_isbd_dn0 = assign15810_e10704_d_n0;
        locals.var_isbd_dn2 = assign15810_e10704_d_n2;
        locals.var_isbd_dn4 = assign15810_e10704_d_n4;
        locals.var_isbd_dn5 = assign15810_e10704_d_n5;
        locals.var_isbd_dn6 = assign15810_e10704_d_n6;
        locals.var_isbd_dn7 = assign15810_e10704_d_n7;
        locals.var_isbd_dn8 = assign15810_e10704_d_n8;
        locals.var_isbd_dn9 = assign15810_e10704_d_n9;
        locals.var_isbd_dn10 = assign15810_e10704_d_n10;
        locals.var_isbd_dn13 = assign15810_e10704_d_n13;

        let assign15820_e10707: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign15820_e10707;

        let (assign15830_e10715, assign15830_e10715_d_n0, assign15830_e10715_d_n2, assign15830_e10715_d_n4, assign15830_e10715_d_n5, assign15830_e10715_d_n6, assign15830_e10715_d_n7, assign15830_e10715_d_n8, assign15830_e10715_d_n9, assign15830_e10715_d_n10, assign15830_e10715_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15830_e10713: f64 = (locals.var_isbd + 1e-25);
        (assign15830_e10713, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign15830_e10715;
        locals.var_t2_dn0 = assign15830_e10715_d_n0;
        locals.var_t2_dn2 = assign15830_e10715_d_n2;
        locals.var_t2_dn4 = assign15830_e10715_d_n4;
        locals.var_t2_dn5 = assign15830_e10715_d_n5;
        locals.var_t2_dn6 = assign15830_e10715_d_n6;
        locals.var_t2_dn7 = assign15830_e10715_d_n7;
        locals.var_t2_dn8 = assign15830_e10715_d_n8;
        locals.var_t2_dn9 = assign15830_e10715_d_n9;
        locals.var_t2_dn10 = assign15830_e10715_d_n10;
        locals.var_t2_dn13 = assign15830_e10715_d_n13;

        let (assign15840_e10732, assign15840_e10732_d_n0, assign15840_e10732_d_n2, assign15840_e10732_d_n4, assign15840_e10732_d_n5, assign15840_e10732_d_n6, assign15840_e10732_d_n7, assign15840_e10732_d_n8, assign15840_e10732_d_n9, assign15840_e10732_d_n10, assign15840_e10732_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15840_e10721: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign15840_e10724: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign15840_e10726: f64 = (assign15840_e10724 / locals.var_t2);
        let assign15840_e10728: f64 = (assign15840_e10726 + 1.0);
        let assign15840_e10729: f64 = (assign15840_e10728).ln();
        let assign15840_e10730: f64 = (assign15840_e10721 * assign15840_e10729);
        (assign15840_e10730, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))), (((-((locals.var_uc_njd * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) * assign15840_e10729) + (assign15840_e10721 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn13) * locals.var_t2) - (assign15840_e10724 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)) / assign15840_e10728))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn13,)
    }
};
        locals.var_vbdt = assign15840_e10732;
        locals.var_vbdt_dn0 = assign15840_e10732_d_n0;
        locals.var_vbdt_dn2 = assign15840_e10732_d_n2;
        locals.var_vbdt_dn4 = assign15840_e10732_d_n4;
        locals.var_vbdt_dn5 = assign15840_e10732_d_n5;
        locals.var_vbdt_dn6 = assign15840_e10732_d_n6;
        locals.var_vbdt_dn7 = assign15840_e10732_d_n7;
        locals.var_vbdt_dn8 = assign15840_e10732_d_n8;
        locals.var_vbdt_dn9 = assign15840_e10732_d_n9;
        locals.var_vbdt_dn10 = assign15840_e10732_d_n10;
        locals.var_vbdt_dn13 = assign15840_e10732_d_n13;

        let (assign15850_e10743, assign15850_e10743_d_n0, assign15850_e10743_d_n2, assign15850_e10743_d_n4, assign15850_e10743_d_n5, assign15850_e10743_d_n6, assign15850_e10743_d_n7, assign15850_e10743_d_n8, assign15850_e10743_d_n9, assign15850_e10743_d_n10, assign15850_e10743_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15850_e10738: f64 = (locals.var_tratio - 1.0);
        let assign15850_e10740: f64 = (assign15850_e10738 * p.p512);
        let assign15850_e10741: f64 = (assign15850_e10740).exp();
        (assign15850_e10741, (assign15850_e10741 * (locals.var_tratio_dn0 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn2 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn4 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn5 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn6 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn7 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn8 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn9 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn10 * p.p512)), (assign15850_e10741 * (locals.var_tratio_dn13 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn13,)
    }
};
        locals.var_exptempd = assign15850_e10743;
        locals.var_exptempd_dn0 = assign15850_e10743_d_n0;
        locals.var_exptempd_dn2 = assign15850_e10743_d_n2;
        locals.var_exptempd_dn4 = assign15850_e10743_d_n4;
        locals.var_exptempd_dn5 = assign15850_e10743_d_n5;
        locals.var_exptempd_dn6 = assign15850_e10743_d_n6;
        locals.var_exptempd_dn7 = assign15850_e10743_d_n7;
        locals.var_exptempd_dn8 = assign15850_e10743_d_n8;
        locals.var_exptempd_dn9 = assign15850_e10743_d_n9;
        locals.var_exptempd_dn10 = assign15850_e10743_d_n10;
        locals.var_exptempd_dn13 = assign15850_e10743_d_n13;

        let (assign15860_e10753, assign15860_e10753_d_n0, assign15860_e10753_d_n2, assign15860_e10753_d_n4, assign15860_e10753_d_n5, assign15860_e10753_d_n6, assign15860_e10753_d_n7, assign15860_e10753_d_n8, assign15860_e10753_d_n9, assign15860_e10753_d_n10, assign15860_e10753_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15860_e10750: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign15860_e10751: f64 = (1.0 / assign15860_e10750);
        (assign15860_e10751, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))), (-((-((locals.var_uc_njd * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) / (assign15860_e10750 * assign15860_e10750))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn13,)
    }
};
        locals.var_jd_nvtm_invd = assign15860_e10753;
        locals.var_jd_nvtm_invd_dn0 = assign15860_e10753_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign15860_e10753_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign15860_e10753_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign15860_e10753_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign15860_e10753_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign15860_e10753_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign15860_e10753_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign15860_e10753_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign15860_e10753_d_n10;
        locals.var_jd_nvtm_invd_dn13 = assign15860_e10753_d_n13;

        let (assign15870_e10762, assign15870_e10762_d_n0, assign15870_e10762_d_n2, assign15870_e10762_d_n4, assign15870_e10762_d_n5, assign15870_e10762_d_n6, assign15870_e10762_d_n7, assign15870_e10762_d_n8, assign15870_e10762_d_n9, assign15870_e10762_d_n10, assign15870_e10762_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard331 != 0.0)) {
        let assign15870_e10759: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign15870_e10760: f64 = (assign15870_e10759).exp();
        (assign15870_e10760, (assign15870_e10760 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign15870_e10760 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign15870_e10760 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign15870_e10760 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign15870_e10760 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign15870_e10760 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign15870_e10760 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign15870_e10760 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign15870_e10760 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign15870_e10760 * ((locals.var_vbdt_dn13 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn13))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    }
};
        locals.var_jd_expcd = assign15870_e10762;
        locals.var_jd_expcd_dn0 = assign15870_e10762_d_n0;
        locals.var_jd_expcd_dn2 = assign15870_e10762_d_n2;
        locals.var_jd_expcd_dn4 = assign15870_e10762_d_n4;
        locals.var_jd_expcd_dn5 = assign15870_e10762_d_n5;
        locals.var_jd_expcd_dn6 = assign15870_e10762_d_n6;
        locals.var_jd_expcd_dn7 = assign15870_e10762_d_n7;
        locals.var_jd_expcd_dn8 = assign15870_e10762_d_n8;
        locals.var_jd_expcd_dn9 = assign15870_e10762_d_n9;
        locals.var_jd_expcd_dn10 = assign15870_e10762_d_n10;
        locals.var_jd_expcd_dn13 = assign15870_e10762_d_n13;

        let (assign15880_e10781, assign15880_e10781_d_n0, assign15880_e10781_d_n2, assign15880_e10781_d_n4, assign15880_e10781_d_n5, assign15880_e10781_d_n6, assign15880_e10781_d_n7, assign15880_e10781_d_n8, assign15880_e10781_d_n9, assign15880_e10781_d_n10, assign15880_e10781_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15880_e10767: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15880_e10770: f64 = (locals.var_eg * locals.var_beta);
        let assign15880_e10771: f64 = (assign15880_e10767 - assign15880_e10770);
        let assign15880_e10774: f64 = (p.p522 * locals.var_log_tratio);
        let assign15880_e10775: f64 = (assign15880_e10771 + assign15880_e10774);
        let assign15880_e10777: f64 = (assign15880_e10775 / locals.var_uc_njs);
        let assign15880_e10778: f64 = (assign15880_e10777).exp();
        let assign15880_e10779: f64 = (locals.var_uc_js0s * assign15880_e10778);
        (assign15880_e10779, (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15880_e10778 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign15880_e10781;
        locals.var_js_dn0 = assign15880_e10781_d_n0;
        locals.var_js_dn2 = assign15880_e10781_d_n2;
        locals.var_js_dn4 = assign15880_e10781_d_n4;
        locals.var_js_dn5 = assign15880_e10781_d_n5;
        locals.var_js_dn6 = assign15880_e10781_d_n6;
        locals.var_js_dn7 = assign15880_e10781_d_n7;
        locals.var_js_dn8 = assign15880_e10781_d_n8;
        locals.var_js_dn9 = assign15880_e10781_d_n9;
        locals.var_js_dn10 = assign15880_e10781_d_n10;
        locals.var_js_dn13 = assign15880_e10781_d_n13;

        let (assign15890_e10800, assign15890_e10800_d_n0, assign15890_e10800_d_n2, assign15890_e10800_d_n4, assign15890_e10800_d_n5, assign15890_e10800_d_n6, assign15890_e10800_d_n7, assign15890_e10800_d_n8, assign15890_e10800_d_n9, assign15890_e10800_d_n10, assign15890_e10800_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15890_e10786: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15890_e10789: f64 = (locals.var_eg * locals.var_beta);
        let assign15890_e10790: f64 = (assign15890_e10786 - assign15890_e10789);
        let assign15890_e10793: f64 = (p.p522 * locals.var_log_tratio);
        let assign15890_e10794: f64 = (assign15890_e10790 + assign15890_e10793);
        let assign15890_e10796: f64 = (assign15890_e10794 / p.p520);
        let assign15890_e10797: f64 = (assign15890_e10796).exp();
        let assign15890_e10798: f64 = (locals.var_uc_js0sws * assign15890_e10797);
        (assign15890_e10798, (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15890_e10797 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign15890_e10800;
        locals.var_jssw_dn0 = assign15890_e10800_d_n0;
        locals.var_jssw_dn2 = assign15890_e10800_d_n2;
        locals.var_jssw_dn4 = assign15890_e10800_d_n4;
        locals.var_jssw_dn5 = assign15890_e10800_d_n5;
        locals.var_jssw_dn6 = assign15890_e10800_d_n6;
        locals.var_jssw_dn7 = assign15890_e10800_d_n7;
        locals.var_jssw_dn8 = assign15890_e10800_d_n8;
        locals.var_jssw_dn9 = assign15890_e10800_d_n9;
        locals.var_jssw_dn10 = assign15890_e10800_d_n10;
        locals.var_jssw_dn13 = assign15890_e10800_d_n13;

        let (assign15900_e10819, assign15900_e10819_d_n0, assign15900_e10819_d_n2, assign15900_e10819_d_n4, assign15900_e10819_d_n5, assign15900_e10819_d_n6, assign15900_e10819_d_n7, assign15900_e10819_d_n8, assign15900_e10819_d_n9, assign15900_e10819_d_n10, assign15900_e10819_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15900_e10805: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15900_e10808: f64 = (locals.var_eg * locals.var_beta);
        let assign15900_e10809: f64 = (assign15900_e10805 - assign15900_e10808);
        let assign15900_e10812: f64 = (p.p522 * locals.var_log_tratio);
        let assign15900_e10813: f64 = (assign15900_e10809 + assign15900_e10812);
        let assign15900_e10815: f64 = (assign15900_e10813 / p.p521);
        let assign15900_e10816: f64 = (assign15900_e10815).exp();
        let assign15900_e10817: f64 = (p.p518 * assign15900_e10816);
        (assign15900_e10817, (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15900_e10816 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign15900_e10819;
        locals.var_jsswg_dn0 = assign15900_e10819_d_n0;
        locals.var_jsswg_dn2 = assign15900_e10819_d_n2;
        locals.var_jsswg_dn4 = assign15900_e10819_d_n4;
        locals.var_jsswg_dn5 = assign15900_e10819_d_n5;
        locals.var_jsswg_dn6 = assign15900_e10819_d_n6;
        locals.var_jsswg_dn7 = assign15900_e10819_d_n7;
        locals.var_jsswg_dn8 = assign15900_e10819_d_n8;
        locals.var_jsswg_dn9 = assign15900_e10819_d_n9;
        locals.var_jsswg_dn10 = assign15900_e10819_d_n10;
        locals.var_jsswg_dn13 = assign15900_e10819_d_n13;

        let (assign15910_e10838, assign15910_e10838_d_n0, assign15910_e10838_d_n2, assign15910_e10838_d_n4, assign15910_e10838_d_n5, assign15910_e10838_d_n6, assign15910_e10838_d_n7, assign15910_e10838_d_n8, assign15910_e10838_d_n9, assign15910_e10838_d_n10, assign15910_e10838_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15910_e10824: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15910_e10827: f64 = (locals.var_eg * locals.var_beta);
        let assign15910_e10828: f64 = (assign15910_e10824 - assign15910_e10827);
        let assign15910_e10831: f64 = (p.p532 * locals.var_log_tratio);
        let assign15910_e10832: f64 = (assign15910_e10828 + assign15910_e10831);
        let assign15910_e10834: f64 = (assign15910_e10832 / locals.var_uc_njs);
        let assign15910_e10835: f64 = (assign15910_e10834).exp();
        let assign15910_e10836: f64 = (locals.var_uc_js0s * assign15910_e10835);
        (assign15910_e10836, (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15910_e10835 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign15910_e10838;
        locals.var_js2_dn0 = assign15910_e10838_d_n0;
        locals.var_js2_dn2 = assign15910_e10838_d_n2;
        locals.var_js2_dn4 = assign15910_e10838_d_n4;
        locals.var_js2_dn5 = assign15910_e10838_d_n5;
        locals.var_js2_dn6 = assign15910_e10838_d_n6;
        locals.var_js2_dn7 = assign15910_e10838_d_n7;
        locals.var_js2_dn8 = assign15910_e10838_d_n8;
        locals.var_js2_dn9 = assign15910_e10838_d_n9;
        locals.var_js2_dn10 = assign15910_e10838_d_n10;
        locals.var_js2_dn13 = assign15910_e10838_d_n13;

        let (assign15920_e10857, assign15920_e10857_d_n0, assign15920_e10857_d_n2, assign15920_e10857_d_n4, assign15920_e10857_d_n5, assign15920_e10857_d_n6, assign15920_e10857_d_n7, assign15920_e10857_d_n8, assign15920_e10857_d_n9, assign15920_e10857_d_n10, assign15920_e10857_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15920_e10843: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15920_e10846: f64 = (locals.var_eg * locals.var_beta);
        let assign15920_e10847: f64 = (assign15920_e10843 - assign15920_e10846);
        let assign15920_e10850: f64 = (p.p532 * locals.var_log_tratio);
        let assign15920_e10851: f64 = (assign15920_e10847 + assign15920_e10850);
        let assign15920_e10853: f64 = (assign15920_e10851 / p.p520);
        let assign15920_e10854: f64 = (assign15920_e10853).exp();
        let assign15920_e10855: f64 = (locals.var_uc_js0sws * assign15920_e10854);
        (assign15920_e10855, (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign15920_e10857;
        locals.var_jssw2_dn0 = assign15920_e10857_d_n0;
        locals.var_jssw2_dn2 = assign15920_e10857_d_n2;
        locals.var_jssw2_dn4 = assign15920_e10857_d_n4;
        locals.var_jssw2_dn5 = assign15920_e10857_d_n5;
        locals.var_jssw2_dn6 = assign15920_e10857_d_n6;
        locals.var_jssw2_dn7 = assign15920_e10857_d_n7;
        locals.var_jssw2_dn8 = assign15920_e10857_d_n8;
        locals.var_jssw2_dn9 = assign15920_e10857_d_n9;
        locals.var_jssw2_dn10 = assign15920_e10857_d_n10;
        locals.var_jssw2_dn13 = assign15920_e10857_d_n13;

        let (assign15930_e10876, assign15930_e10876_d_n0, assign15930_e10876_d_n2, assign15930_e10876_d_n4, assign15930_e10876_d_n5, assign15930_e10876_d_n6, assign15930_e10876_d_n7, assign15930_e10876_d_n8, assign15930_e10876_d_n9, assign15930_e10876_d_n10, assign15930_e10876_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15930_e10862: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15930_e10865: f64 = (locals.var_eg * locals.var_beta);
        let assign15930_e10866: f64 = (assign15930_e10862 - assign15930_e10865);
        let assign15930_e10869: f64 = (p.p532 * locals.var_log_tratio);
        let assign15930_e10870: f64 = (assign15930_e10866 + assign15930_e10869);
        let assign15930_e10872: f64 = (assign15930_e10870 / p.p521);
        let assign15930_e10873: f64 = (assign15930_e10872).exp();
        let assign15930_e10874: f64 = (p.p518 * assign15930_e10873);
        (assign15930_e10874, (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign15930_e10876;
        locals.var_jsswg2_dn0 = assign15930_e10876_d_n0;
        locals.var_jsswg2_dn2 = assign15930_e10876_d_n2;
        locals.var_jsswg2_dn4 = assign15930_e10876_d_n4;
        locals.var_jsswg2_dn5 = assign15930_e10876_d_n5;
        locals.var_jsswg2_dn6 = assign15930_e10876_d_n6;
        locals.var_jsswg2_dn7 = assign15930_e10876_d_n7;
        locals.var_jsswg2_dn8 = assign15930_e10876_d_n8;
        locals.var_jsswg2_dn9 = assign15930_e10876_d_n9;
        locals.var_jsswg2_dn10 = assign15930_e10876_d_n10;
        locals.var_jsswg2_dn13 = assign15930_e10876_d_n13;

        let assign15940_e10879: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard332 = assign15940_e10879;

        let assign15950_e10882: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard333 = assign15950_e10882;

        let (assign15960_e10892, assign15960_e10892_d_n0, assign15960_e10892_d_n2, assign15960_e10892_d_n4, assign15960_e10892_d_n5, assign15960_e10892_d_n6, assign15960_e10892_d_n7, assign15960_e10892_d_n8, assign15960_e10892_d_n9, assign15960_e10892_d_n10, assign15960_e10892_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15960_e10890: f64 = (p.p14 * locals.var_js);
        (assign15960_e10890, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign15960_e10892;
        locals.var_isbs_btm_dn0 = assign15960_e10892_d_n0;
        locals.var_isbs_btm_dn2 = assign15960_e10892_d_n2;
        locals.var_isbs_btm_dn4 = assign15960_e10892_d_n4;
        locals.var_isbs_btm_dn5 = assign15960_e10892_d_n5;
        locals.var_isbs_btm_dn6 = assign15960_e10892_d_n6;
        locals.var_isbs_btm_dn7 = assign15960_e10892_d_n7;
        locals.var_isbs_btm_dn8 = assign15960_e10892_d_n8;
        locals.var_isbs_btm_dn9 = assign15960_e10892_d_n9;
        locals.var_isbs_btm_dn10 = assign15960_e10892_d_n10;
        locals.var_isbs_btm_dn13 = assign15960_e10892_d_n13;

        let (assign15970_e10902, assign15970_e10902_d_n0, assign15970_e10902_d_n2, assign15970_e10902_d_n4, assign15970_e10902_d_n5, assign15970_e10902_d_n6, assign15970_e10902_d_n7, assign15970_e10902_d_n8, assign15970_e10902_d_n9, assign15970_e10902_d_n10, assign15970_e10902_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15970_e10900: f64 = (p.p14 * locals.var_js2);
        (assign15970_e10900, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign15970_e10902;
        locals.var_isbs2_btm_dn0 = assign15970_e10902_d_n0;
        locals.var_isbs2_btm_dn2 = assign15970_e10902_d_n2;
        locals.var_isbs2_btm_dn4 = assign15970_e10902_d_n4;
        locals.var_isbs2_btm_dn5 = assign15970_e10902_d_n5;
        locals.var_isbs2_btm_dn6 = assign15970_e10902_d_n6;
        locals.var_isbs2_btm_dn7 = assign15970_e10902_d_n7;
        locals.var_isbs2_btm_dn8 = assign15970_e10902_d_n8;
        locals.var_isbs2_btm_dn9 = assign15970_e10902_d_n9;
        locals.var_isbs2_btm_dn10 = assign15970_e10902_d_n10;
        locals.var_isbs2_btm_dn13 = assign15970_e10902_d_n13;

    }
}

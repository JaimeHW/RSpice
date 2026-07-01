#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        var_bpfactornw_i_slot: &mut f64,
        var_bpfactorpw_i_slot: &mut f64,
        var_cbgcbg_i_slot: &mut f64,
        var_cdsc_i_slot: &mut f64,
        var_cdscd_i_slot: &mut f64,
        var_cit_i_slot: &mut f64,
        var_dbgnw_i_slot: &mut f64,
        var_dbgpw_i_slot: &mut f64,
        var_devsign_slot: &mut f64,
        var_dlcv_slot: &mut f64,
        var_dliv_slot: &mut f64,
        var_dvt0_i_slot: &mut f64,
        var_dvt1_i_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_dwiv_slot: &mut f64,
        var_epssi_slot: &mut f64,
        var_gdpr_slot: &mut f64,
        var_gdpr_dn3_slot: &mut f64,
        var_gdpr_dn4_slot: &mut f64,
        var_gdpr_dn5_slot: &mut f64,
        var_gdpr_dn6_slot: &mut f64,
        var_gdpr_dn7_slot: &mut f64,
        var_gdpr_dn8_slot: &mut f64,
        var_gspr_slot: &mut f64,
        var_gspr_dn3_slot: &mut f64,
        var_gspr_dn4_slot: &mut f64,
        var_gspr_dn5_slot: &mut f64,
        var_gspr_dn6_slot: &mut f64,
        var_gspr_dn7_slot: &mut f64,
        var_gspr_dn8_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_inv_l_slot: &mut f64,
        var_inv_w_slot: &mut f64,
        var_inv_wl_slot: &mut f64,
        var_kbg0nw_i_slot: &mut f64,
        var_kbg0pw_i_slot: &mut f64,
        var_kbg1nw_i_slot: &mut f64,
        var_kbg1pw_i_slot: &mut f64,
        var_kbg2nw_i_slot: &mut f64,
        var_kbg2pw_i_slot: &mut f64,
        var_l_lln_slot: &mut f64,
        var_l_wln_slot: &mut f64,
        var_leff_slot: &mut f64,
        var_leffcv_slot: &mut f64,
        var_lnew_slot: &mut f64,
        var_lw_lln_lwn_slot: &mut f64,
        var_lw_wln_wwn_slot: &mut f64,
        var_mpower_i_slot: &mut f64,
        var_nbody_i_slot: &mut f64,
        var_noia2_i_slot: &mut f64,
        var_nsd_i_slot: &mut f64,
        var_phig1_i_slot: &mut f64,
        var_phig2_i_slot: &mut f64,
        var_phig2_i_dn3_slot: &mut f64,
        var_phig2_i_dn4_slot: &mut f64,
        var_phig2_i_dn5_slot: &mut f64,
        var_phig2_i_dn6_slot: &mut f64,
        var_phig2_i_dn7_slot: &mut f64,
        var_phig2_i_dn8_slot: &mut f64,
        var_phin_i_slot: &mut f64,
        var_prwb_i_slot: &mut f64,
        var_prwg_i_slot: &mut f64,
        var_qsref_i_slot: &mut f64,
        var_rdrain_slot: &mut f64,
        var_rdrain_dn3_slot: &mut f64,
        var_rdrain_dn4_slot: &mut f64,
        var_rdrain_dn5_slot: &mut f64,
        var_rdrain_dn6_slot: &mut f64,
        var_rdrain_dn7_slot: &mut f64,
        var_rdrain_dn8_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rsource_slot: &mut f64,
        var_rsource_dn3_slot: &mut f64,
        var_rsource_dn4_slot: &mut f64,
        var_rsource_dn5_slot: &mut f64,
        var_rsource_dn6_slot: &mut f64,
        var_rsource_dn7_slot: &mut f64,
        var_rsource_dn8_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_vknee1nw_i_slot: &mut f64,
        var_vknee1pw_i_slot: &mut f64,
        var_vknee2nw_i_slot: &mut f64,
        var_vknee2pw_i_slot: &mut f64,
        var_w_lwn_slot: &mut f64,
        var_w_wwn_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weffcv_slot: &mut f64,
        var_welsign_slot: &mut f64,
        var_wnew_slot: &mut f64,
        var_wr_i_slot: &mut f64,
    ) {
        let mut var_bpfactornw_i: f64 = *var_bpfactornw_i_slot;
        let mut var_bpfactorpw_i: f64 = *var_bpfactorpw_i_slot;
        let mut var_cbgcbg_i: f64 = *var_cbgcbg_i_slot;
        let mut var_cdsc_i: f64 = *var_cdsc_i_slot;
        let mut var_cdscd_i: f64 = *var_cdscd_i_slot;
        let mut var_cit_i: f64 = *var_cit_i_slot;
        let mut var_dbgnw_i: f64 = *var_dbgnw_i_slot;
        let mut var_dbgpw_i: f64 = *var_dbgpw_i_slot;
        let mut var_devsign: f64 = *var_devsign_slot;
        let mut var_dlcv: f64 = *var_dlcv_slot;
        let mut var_dliv: f64 = *var_dliv_slot;
        let mut var_dvt0_i: f64 = *var_dvt0_i_slot;
        let mut var_dvt1_i: f64 = *var_dvt1_i_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_dwiv: f64 = *var_dwiv_slot;
        let mut var_epssi: f64 = *var_epssi_slot;
        let mut var_gdpr: f64 = *var_gdpr_slot;
        let mut var_gdpr_dn3: f64 = *var_gdpr_dn3_slot;
        let mut var_gdpr_dn4: f64 = *var_gdpr_dn4_slot;
        let mut var_gdpr_dn5: f64 = *var_gdpr_dn5_slot;
        let mut var_gdpr_dn6: f64 = *var_gdpr_dn6_slot;
        let mut var_gdpr_dn7: f64 = *var_gdpr_dn7_slot;
        let mut var_gdpr_dn8: f64 = *var_gdpr_dn8_slot;
        let mut var_gspr: f64 = *var_gspr_slot;
        let mut var_gspr_dn3: f64 = *var_gspr_dn3_slot;
        let mut var_gspr_dn4: f64 = *var_gspr_dn4_slot;
        let mut var_gspr_dn5: f64 = *var_gspr_dn5_slot;
        let mut var_gspr_dn6: f64 = *var_gspr_dn6_slot;
        let mut var_gspr_dn7: f64 = *var_gspr_dn7_slot;
        let mut var_gspr_dn8: f64 = *var_gspr_dn8_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_inv_l: f64 = *var_inv_l_slot;
        let mut var_inv_w: f64 = *var_inv_w_slot;
        let mut var_inv_wl: f64 = *var_inv_wl_slot;
        let mut var_kbg0nw_i: f64 = *var_kbg0nw_i_slot;
        let mut var_kbg0pw_i: f64 = *var_kbg0pw_i_slot;
        let mut var_kbg1nw_i: f64 = *var_kbg1nw_i_slot;
        let mut var_kbg1pw_i: f64 = *var_kbg1pw_i_slot;
        let mut var_kbg2nw_i: f64 = *var_kbg2nw_i_slot;
        let mut var_kbg2pw_i: f64 = *var_kbg2pw_i_slot;
        let mut var_l_lln: f64 = *var_l_lln_slot;
        let mut var_l_wln: f64 = *var_l_wln_slot;
        let mut var_leff: f64 = *var_leff_slot;
        let mut var_leffcv: f64 = *var_leffcv_slot;
        let mut var_lnew: f64 = *var_lnew_slot;
        let mut var_lw_lln_lwn: f64 = *var_lw_lln_lwn_slot;
        let mut var_lw_wln_wwn: f64 = *var_lw_wln_wwn_slot;
        let mut var_mpower_i: f64 = *var_mpower_i_slot;
        let mut var_nbody_i: f64 = *var_nbody_i_slot;
        let mut var_noia2_i: f64 = *var_noia2_i_slot;
        let mut var_nsd_i: f64 = *var_nsd_i_slot;
        let mut var_phig1_i: f64 = *var_phig1_i_slot;
        let mut var_phig2_i: f64 = *var_phig2_i_slot;
        let mut var_phig2_i_dn3: f64 = *var_phig2_i_dn3_slot;
        let mut var_phig2_i_dn4: f64 = *var_phig2_i_dn4_slot;
        let mut var_phig2_i_dn5: f64 = *var_phig2_i_dn5_slot;
        let mut var_phig2_i_dn6: f64 = *var_phig2_i_dn6_slot;
        let mut var_phig2_i_dn7: f64 = *var_phig2_i_dn7_slot;
        let mut var_phig2_i_dn8: f64 = *var_phig2_i_dn8_slot;
        let mut var_phin_i: f64 = *var_phin_i_slot;
        let mut var_prwb_i: f64 = *var_prwb_i_slot;
        let mut var_prwg_i: f64 = *var_prwg_i_slot;
        let mut var_qsref_i: f64 = *var_qsref_i_slot;
        let mut var_rdrain: f64 = *var_rdrain_slot;
        let mut var_rdrain_dn3: f64 = *var_rdrain_dn3_slot;
        let mut var_rdrain_dn4: f64 = *var_rdrain_dn4_slot;
        let mut var_rdrain_dn5: f64 = *var_rdrain_dn5_slot;
        let mut var_rdrain_dn6: f64 = *var_rdrain_dn6_slot;
        let mut var_rdrain_dn7: f64 = *var_rdrain_dn7_slot;
        let mut var_rdrain_dn8: f64 = *var_rdrain_dn8_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rsource: f64 = *var_rsource_slot;
        let mut var_rsource_dn3: f64 = *var_rsource_dn3_slot;
        let mut var_rsource_dn4: f64 = *var_rsource_dn4_slot;
        let mut var_rsource_dn5: f64 = *var_rsource_dn5_slot;
        let mut var_rsource_dn6: f64 = *var_rsource_dn6_slot;
        let mut var_rsource_dn7: f64 = *var_rsource_dn7_slot;
        let mut var_rsource_dn8: f64 = *var_rsource_dn8_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_vknee1nw_i: f64 = *var_vknee1nw_i_slot;
        let mut var_vknee1pw_i: f64 = *var_vknee1pw_i_slot;
        let mut var_vknee2nw_i: f64 = *var_vknee2nw_i_slot;
        let mut var_vknee2pw_i: f64 = *var_vknee2pw_i_slot;
        let mut var_w_lwn: f64 = *var_w_lwn_slot;
        let mut var_w_wwn: f64 = *var_w_wwn_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weffcv: f64 = *var_weffcv_slot;
        let mut var_welsign: f64 = *var_welsign_slot;
        let mut var_wnew: f64 = *var_wnew_slot;
        let mut var_wr_i: f64 = *var_wr_i_slot;

        var_rdrain = 0.0;
        var_rdrain_dn3 = 0.0;
        var_rdrain_dn4 = 0.0;
        var_rdrain_dn5 = 0.0;
        var_rdrain_dn6 = 0.0;
        var_rdrain_dn7 = 0.0;
        var_rdrain_dn8 = 0.0;

        var_rsource = 0.0;
        var_rsource_dn3 = 0.0;
        var_rsource_dn4 = 0.0;
        var_rsource_dn5 = 0.0;
        var_rsource_dn6 = 0.0;
        var_rsource_dn7 = 0.0;
        var_rsource_dn8 = 0.0;

        var_gspr = 0.0;
        var_gspr_dn3 = 0.0;
        var_gspr_dn4 = 0.0;
        var_gspr_dn5 = 0.0;
        var_gspr_dn6 = 0.0;
        var_gspr_dn7 = 0.0;
        var_gspr_dn8 = 0.0;

        var_gdpr = 0.0;
        var_gdpr_dn3 = 0.0;
        var_gdpr_dn4 = 0.0;
        var_gdpr_dn5 = 0.0;
        var_gdpr_dn6 = 0.0;
        var_gdpr_dn7 = 0.0;
        var_gdpr_dn8 = 0.0;

        let assign70_e1130: f64 = if p.p12 == 1.0 { 1.0 } else { 0.0 };
        var_guard3 = assign70_e1130;

        let (assign80_e1134,) = {
    if (var_guard3 != 0.0) {
        (1.0,)
    } else {
        (var_devsign,)
    }
};
        var_devsign = assign80_e1134;

        let (assign90_e1140,) = {
    if (var_guard3 == 0.0) {
        let assign90_e1138: f64 = (-1.0);
        (assign90_e1138,)
    } else {
        (var_devsign,)
    }
};
        var_devsign = assign90_e1140;

        let assign100_e1143: f64 = if p.p13 == 1.0 { 1.0 } else { 0.0 };
        var_guard4 = assign100_e1143;

        let (assign110_e1147,) = {
    if (var_guard4 != 0.0) {
        (1.0,)
    } else {
        (var_welsign,)
    }
};
        var_welsign = assign110_e1147;

        let (assign120_e1153,) = {
    if (var_guard4 == 0.0) {
        let assign120_e1151: f64 = (-1.0);
        (assign120_e1151,)
    } else {
        (var_welsign,)
    }
};
        var_welsign = assign120_e1153;

        let assign130_e1156: f64 = (p.p59 * 8.85418e-12);
        var_epssi = assign130_e1156;

        let assign140_e1159: f64 = if p.p21 == 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign140_e1159;

        let (assign150_e1165,) = {
    if (var_guard5 != 0.0) {
        let assign150_e1163: f64 = (p.p1 / p.p2);
        (assign150_e1163,)
    } else {
        (var_wnew,)
    }
};
        var_wnew = assign150_e1165;

        let (assign160_e1170,) = {
    if (var_guard5 == 0.0) {
        (p.p1,)
    } else {
        (var_wnew,)
    }
};
        var_wnew = assign160_e1170;

        let assign170_e1173: f64 = (p.p0 + p.p23);
        var_lnew = assign170_e1173;

        let assign180_e1176: f64 = (var_wnew + p.p24);
        var_wnew = assign180_e1176;

        let assign190_e1179: f64 = (-p.p29);
        let assign190_e1180: f64 = (var_lnew).powf(assign190_e1179);
        var_l_lln = assign190_e1180;

        let assign200_e1183: f64 = (-p.p30);
        let assign200_e1184: f64 = (var_wnew).powf(assign200_e1183);
        var_w_lwn = assign200_e1184;

        let assign210_e1187: f64 = (var_l_lln * var_w_lwn);
        var_lw_lln_lwn = assign210_e1187;

        let assign220_e1191: f64 = (p.p26 * var_l_lln);
        let assign220_e1192: f64 = (p.p25 + assign220_e1191);
        let assign220_e1195: f64 = (p.p27 * var_w_lwn);
        let assign220_e1196: f64 = (assign220_e1192 + assign220_e1195);
        let assign220_e1199: f64 = (p.p28 * var_lw_lln_lwn);
        let assign220_e1200: f64 = (assign220_e1196 + assign220_e1199);
        var_dliv = assign220_e1200;

        let assign230_e1203: f64 = (-p.p35);
        let assign230_e1204: f64 = (var_lnew).powf(assign230_e1203);
        var_l_wln = assign230_e1204;

        let assign240_e1207: f64 = (-p.p36);
        let assign240_e1208: f64 = (var_wnew).powf(assign240_e1207);
        var_w_wwn = assign240_e1208;

        let assign250_e1211: f64 = (var_l_wln * var_w_wwn);
        var_lw_wln_wwn = assign250_e1211;

        let assign260_e1215: f64 = (p.p32 * var_l_wln);
        let assign260_e1216: f64 = (p.p31 + assign260_e1215);
        let assign260_e1219: f64 = (p.p33 * var_w_wwn);
        let assign260_e1220: f64 = (assign260_e1216 + assign260_e1219);
        let assign260_e1223: f64 = (p.p34 * var_lw_wln_wwn);
        let assign260_e1224: f64 = (assign260_e1220 + assign260_e1223);
        var_dwiv = assign260_e1224;

        let assign270_e1228: f64 = (2.0 * var_dliv);
        let assign270_e1229: f64 = (var_lnew - assign270_e1228);
        var_leff = assign270_e1229;

        let assign300_e1239: f64 = (2.0 * var_dwiv);
        let assign300_e1240: f64 = (var_wnew - assign300_e1239);
        var_weff = assign300_e1240;

        let assign330_e1250: f64 = (p.p38 * var_l_lln);
        let assign330_e1251: f64 = (p.p37 + assign330_e1250);
        let assign330_e1254: f64 = (p.p39 * var_w_lwn);
        let assign330_e1255: f64 = (assign330_e1251 + assign330_e1254);
        let assign330_e1258: f64 = (p.p40 * var_lw_lln_lwn);
        let assign330_e1259: f64 = (assign330_e1255 + assign330_e1258);
        var_dlcv = assign330_e1259;

        let assign340_e1263: f64 = (p.p42 * var_l_wln);
        let assign340_e1264: f64 = (p.p41 + assign340_e1263);
        let assign340_e1267: f64 = (p.p43 * var_w_wwn);
        let assign340_e1268: f64 = (assign340_e1264 + assign340_e1267);
        let assign340_e1271: f64 = (p.p44 * var_lw_wln_wwn);
        let assign340_e1272: f64 = (assign340_e1268 + assign340_e1271);
        var_dwcv = assign340_e1272;

        let assign350_e1276: f64 = (2.0 * var_dlcv);
        let assign350_e1277: f64 = (var_lnew - assign350_e1276);
        var_leffcv = assign350_e1277;

        let assign380_e1287: f64 = (2.0 * var_dwcv);
        let assign380_e1288: f64 = (var_wnew - assign380_e1287);
        var_weffcv = assign380_e1288;

        let assign410_e1297: f64 = (1e-6 / var_leff);
        var_inv_l = assign410_e1297;

        let assign420_e1300: f64 = (1e-6 / var_weff);
        var_inv_w = assign420_e1300;

        let assign430_e1303: f64 = (var_inv_l * var_inv_w);
        var_inv_wl = assign430_e1303;

        let assign440_e1307: f64 = (p.p319 * var_inv_l);
        let assign440_e1308: f64 = (p.p191 + assign440_e1307);
        let assign440_e1311: f64 = (p.p320 * var_inv_w);
        let assign440_e1312: f64 = (assign440_e1308 + assign440_e1311);
        let assign440_e1315: f64 = (p.p321 * var_inv_wl);
        let assign440_e1316: f64 = (assign440_e1312 + assign440_e1315);
        var_rdsw_i = assign440_e1316;

        let assign450_e1320: f64 = (p.p325 * var_inv_l);
        let assign450_e1321: f64 = (p.p199 + assign450_e1320);
        let assign450_e1324: f64 = (p.p326 * var_inv_w);
        let assign450_e1325: f64 = (assign450_e1321 + assign450_e1324);
        let assign450_e1328: f64 = (p.p327 * var_inv_wl);
        let assign450_e1329: f64 = (assign450_e1325 + assign450_e1328);
        var_rdw_i = assign450_e1329;

        let assign460_e1333: f64 = (p.p322 * var_inv_l);
        let assign460_e1334: f64 = (p.p195 + assign460_e1333);
        let assign460_e1337: f64 = (p.p323 * var_inv_w);
        let assign460_e1338: f64 = (assign460_e1334 + assign460_e1337);
        let assign460_e1341: f64 = (p.p324 * var_inv_wl);
        let assign460_e1342: f64 = (assign460_e1338 + assign460_e1341);
        var_rsw_i = assign460_e1342;

        let assign470_e1346: f64 = (p.p328 * var_inv_l);
        let assign470_e1347: f64 = (p.p202 + assign470_e1346);
        let assign470_e1350: f64 = (p.p329 * var_inv_w);
        let assign470_e1351: f64 = (assign470_e1347 + assign470_e1350);
        let assign470_e1354: f64 = (p.p330 * var_inv_wl);
        let assign470_e1355: f64 = (assign470_e1351 + assign470_e1354);
        var_prwg_i = assign470_e1355;

        let assign480_e1359: f64 = (p.p331 * var_inv_l);
        let assign480_e1360: f64 = (p.p203 + assign480_e1359);
        let assign480_e1363: f64 = (p.p332 * var_inv_w);
        let assign480_e1364: f64 = (assign480_e1360 + assign480_e1363);
        let assign480_e1367: f64 = (p.p333 * var_inv_wl);
        let assign480_e1368: f64 = (assign480_e1364 + assign480_e1367);
        var_prwb_i = assign480_e1368;

        let assign490_e1372: f64 = (p.p334 * var_inv_l);
        let assign490_e1373: f64 = (p.p204 + assign490_e1372);
        let assign490_e1376: f64 = (p.p335 * var_inv_w);
        let assign490_e1377: f64 = (assign490_e1373 + assign490_e1376);
        let assign490_e1380: f64 = (p.p336 * var_inv_wl);
        let assign490_e1381: f64 = (assign490_e1377 + assign490_e1380);
        var_wr_i = assign490_e1381;

        let assign500_e1385: f64 = (p.p337 * var_inv_l);
        let assign500_e1386: f64 = (p.p57 + assign500_e1385);
        let assign500_e1389: f64 = (p.p338 * var_inv_w);
        let assign500_e1390: f64 = (assign500_e1386 + assign500_e1389);
        let assign500_e1393: f64 = (p.p339 * var_inv_wl);
        let assign500_e1394: f64 = (assign500_e1390 + assign500_e1393);
        var_phig1_i = assign500_e1394;

        let assign510_e1398: f64 = (p.p340 * var_inv_l);
        let assign510_e1399: f64 = (p.p58 + assign510_e1398);
        let assign510_e1402: f64 = (p.p341 * var_inv_w);
        let assign510_e1403: f64 = (assign510_e1399 + assign510_e1402);
        let assign510_e1406: f64 = (p.p342 * var_inv_wl);
        let assign510_e1407: f64 = (assign510_e1403 + assign510_e1406);
        var_phig2_i = assign510_e1407;
        var_phig2_i_dn3 = 0.0;
        var_phig2_i_dn4 = 0.0;
        var_phig2_i_dn5 = 0.0;
        var_phig2_i_dn6 = 0.0;
        var_phig2_i_dn7 = 0.0;
        var_phig2_i_dn8 = 0.0;

        let assign520_e1411: f64 = (p.p343 * var_inv_l);
        let assign520_e1412: f64 = (p.p51 + assign520_e1411);
        let assign520_e1415: f64 = (p.p344 * var_inv_w);
        let assign520_e1416: f64 = (assign520_e1412 + assign520_e1415);
        let assign520_e1419: f64 = (p.p345 * var_inv_wl);
        let assign520_e1420: f64 = (assign520_e1416 + assign520_e1419);
        var_nsd_i = assign520_e1420;

        let assign530_e1424: f64 = (p.p346 * var_inv_l);
        let assign530_e1425: f64 = (p.p50 + assign530_e1424);
        let assign530_e1428: f64 = (p.p347 * var_inv_w);
        let assign530_e1429: f64 = (assign530_e1425 + assign530_e1428);
        let assign530_e1432: f64 = (p.p348 * var_inv_wl);
        let assign530_e1433: f64 = (assign530_e1429 + assign530_e1432);
        var_nbody_i = assign530_e1433;

        let assign540_e1437: f64 = (p.p349 * var_inv_l);
        let assign540_e1438: f64 = (p.p63 + assign540_e1437);
        let assign540_e1441: f64 = (p.p350 * var_inv_w);
        let assign540_e1442: f64 = (assign540_e1438 + assign540_e1441);
        let assign540_e1445: f64 = (p.p351 * var_inv_wl);
        let assign540_e1446: f64 = (assign540_e1442 + assign540_e1445);
        var_cit_i = assign540_e1446;

        let assign550_e1450: f64 = (p.p352 * var_inv_l);
        let assign550_e1451: f64 = (p.p64 + assign550_e1450);
        let assign550_e1454: f64 = (p.p353 * var_inv_w);
        let assign550_e1455: f64 = (assign550_e1451 + assign550_e1454);
        let assign550_e1458: f64 = (p.p354 * var_inv_wl);
        let assign550_e1459: f64 = (assign550_e1455 + assign550_e1458);
        var_cdsc_i = assign550_e1459;

        let assign560_e1463: f64 = (p.p355 * var_inv_l);
        let assign560_e1464: f64 = (p.p65 + assign560_e1463);
        let assign560_e1467: f64 = (p.p356 * var_inv_w);
        let assign560_e1468: f64 = (assign560_e1464 + assign560_e1467);
        let assign560_e1471: f64 = (p.p357 * var_inv_wl);
        let assign560_e1472: f64 = (assign560_e1468 + assign560_e1471);
        var_cdscd_i = assign560_e1472;

        let assign570_e1476: f64 = (p.p358 * var_inv_l);
        let assign570_e1477: f64 = (p.p68 + assign570_e1476);
        let assign570_e1480: f64 = (p.p359 * var_inv_w);
        let assign570_e1481: f64 = (assign570_e1477 + assign570_e1480);
        let assign570_e1484: f64 = (p.p360 * var_inv_wl);
        let assign570_e1485: f64 = (assign570_e1481 + assign570_e1484);
        var_cbgcbg_i = assign570_e1485;

        let assign580_e1489: f64 = (p.p361 * var_inv_l);
        let assign580_e1490: f64 = (p.p276 + assign580_e1489);
        let assign580_e1493: f64 = (p.p362 * var_inv_w);
        let assign580_e1494: f64 = (assign580_e1490 + assign580_e1493);
        let assign580_e1497: f64 = (p.p363 * var_inv_wl);
        let assign580_e1498: f64 = (assign580_e1494 + assign580_e1497);
        var_bpfactorpw_i = assign580_e1498;

        let assign590_e1502: f64 = (p.p751 * var_inv_l);
        let assign590_e1503: f64 = (p.p291 + assign590_e1502);
        let assign590_e1506: f64 = (p.p752 * var_inv_w);
        let assign590_e1507: f64 = (assign590_e1503 + assign590_e1506);
        let assign590_e1510: f64 = (p.p753 * var_inv_wl);
        let assign590_e1511: f64 = (assign590_e1507 + assign590_e1510);
        var_noia2_i = assign590_e1511;

        let assign600_e1515: f64 = (p.p757 * var_inv_l);
        let assign600_e1516: f64 = (p.p294 + assign600_e1515);
        let assign600_e1519: f64 = (p.p758 * var_inv_w);
        let assign600_e1520: f64 = (assign600_e1516 + assign600_e1519);
        let assign600_e1523: f64 = (p.p759 * var_inv_wl);
        let assign600_e1524: f64 = (assign600_e1520 + assign600_e1523);
        var_qsref_i = assign600_e1524;

        let assign610_e1528: f64 = (p.p754 * var_inv_l);
        let assign610_e1529: f64 = (p.p293 + assign610_e1528);
        let assign610_e1532: f64 = (p.p755 * var_inv_w);
        let assign610_e1533: f64 = (assign610_e1529 + assign610_e1532);
        let assign610_e1536: f64 = (p.p756 * var_inv_wl);
        let assign610_e1537: f64 = (assign610_e1533 + assign610_e1536);
        var_mpower_i = assign610_e1537;

        let assign620_e1540: f64 = if var_bpfactorpw_i < 0.0 { 1.0 } else { 0.0 };
        var_guard14 = assign620_e1540;

        let (assign630_e1544,) = {
    if (var_guard14 != 0.0) {
        (0.0,)
    } else {
        (var_bpfactorpw_i,)
    }
};
        var_bpfactorpw_i = assign630_e1544;

        let assign640_e1547: f64 = if var_bpfactorpw_i > 1.0 { 1.0 } else { 0.0 };
        var_guard15 = assign640_e1547;

        let (assign650_e1554,) = {
    if ((var_guard14 == 0.0) && (var_guard15 != 0.0)) {
        (1.0,)
    } else {
        (var_bpfactorpw_i,)
    }
};
        var_bpfactorpw_i = assign650_e1554;

        let assign660_e1558: f64 = (p.p364 * var_inv_l);
        let assign660_e1559: f64 = (p.p277 + assign660_e1558);
        let assign660_e1562: f64 = (p.p365 * var_inv_w);
        let assign660_e1563: f64 = (assign660_e1559 + assign660_e1562);
        let assign660_e1566: f64 = (p.p366 * var_inv_wl);
        let assign660_e1567: f64 = (assign660_e1563 + assign660_e1566);
        var_vknee1pw_i = assign660_e1567;

        let assign670_e1571: f64 = (p.p367 * var_inv_l);
        let assign670_e1572: f64 = (p.p278 + assign670_e1571);
        let assign670_e1575: f64 = (p.p368 * var_inv_w);
        let assign670_e1576: f64 = (assign670_e1572 + assign670_e1575);
        let assign670_e1579: f64 = (p.p369 * var_inv_wl);
        let assign670_e1580: f64 = (assign670_e1576 + assign670_e1579);
        var_vknee2pw_i = assign670_e1580;

        let assign680_e1584: f64 = (p.p370 * var_inv_l);
        let assign680_e1585: f64 = (p.p275 + assign680_e1584);
        let assign680_e1588: f64 = (p.p371 * var_inv_w);
        let assign680_e1589: f64 = (assign680_e1585 + assign680_e1588);
        let assign680_e1592: f64 = (p.p372 * var_inv_wl);
        let assign680_e1593: f64 = (assign680_e1589 + assign680_e1592);
        var_dbgpw_i = assign680_e1593;

        let assign690_e1597: f64 = (p.p373 * var_inv_l);
        let assign690_e1598: f64 = (p.p272 + assign690_e1597);
        let assign690_e1601: f64 = (p.p374 * var_inv_w);
        let assign690_e1602: f64 = (assign690_e1598 + assign690_e1601);
        let assign690_e1605: f64 = (p.p375 * var_inv_wl);
        let assign690_e1606: f64 = (assign690_e1602 + assign690_e1605);
        var_kbg0pw_i = assign690_e1606;

        let assign700_e1610: f64 = (p.p376 * var_inv_l);
        let assign700_e1611: f64 = (p.p273 + assign700_e1610);
        let assign700_e1614: f64 = (p.p377 * var_inv_w);
        let assign700_e1615: f64 = (assign700_e1611 + assign700_e1614);
        let assign700_e1618: f64 = (p.p378 * var_inv_wl);
        let assign700_e1619: f64 = (assign700_e1615 + assign700_e1618);
        var_kbg1pw_i = assign700_e1619;

        let assign710_e1623: f64 = (p.p379 * var_inv_l);
        let assign710_e1624: f64 = (p.p274 + assign710_e1623);
        let assign710_e1627: f64 = (p.p380 * var_inv_w);
        let assign710_e1628: f64 = (assign710_e1624 + assign710_e1627);
        let assign710_e1631: f64 = (p.p381 * var_inv_wl);
        let assign710_e1632: f64 = (assign710_e1628 + assign710_e1631);
        var_kbg2pw_i = assign710_e1632;

        let assign720_e1636: f64 = (p.p382 * var_inv_l);
        let assign720_e1637: f64 = (p.p283 + assign720_e1636);
        let assign720_e1640: f64 = (p.p383 * var_inv_w);
        let assign720_e1641: f64 = (assign720_e1637 + assign720_e1640);
        let assign720_e1644: f64 = (p.p384 * var_inv_wl);
        let assign720_e1645: f64 = (assign720_e1641 + assign720_e1644);
        var_bpfactornw_i = assign720_e1645;

        let assign730_e1648: f64 = if var_bpfactornw_i < 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign730_e1648;

        let (assign740_e1652,) = {
    if (var_guard16 != 0.0) {
        (0.0,)
    } else {
        (var_bpfactornw_i,)
    }
};
        var_bpfactornw_i = assign740_e1652;

        let assign750_e1655: f64 = if var_bpfactornw_i > 1.0 { 1.0 } else { 0.0 };
        var_guard17 = assign750_e1655;

        let (assign760_e1662,) = {
    if ((var_guard16 == 0.0) && (var_guard17 != 0.0)) {
        (1.0,)
    } else {
        (var_bpfactornw_i,)
    }
};
        var_bpfactornw_i = assign760_e1662;

        let assign770_e1666: f64 = (p.p385 * var_inv_l);
        let assign770_e1667: f64 = (p.p284 + assign770_e1666);
        let assign770_e1670: f64 = (p.p386 * var_inv_w);
        let assign770_e1671: f64 = (assign770_e1667 + assign770_e1670);
        let assign770_e1674: f64 = (p.p387 * var_inv_wl);
        let assign770_e1675: f64 = (assign770_e1671 + assign770_e1674);
        var_vknee1nw_i = assign770_e1675;

        let assign780_e1679: f64 = (p.p388 * var_inv_l);
        let assign780_e1680: f64 = (p.p285 + assign780_e1679);
        let assign780_e1683: f64 = (p.p389 * var_inv_w);
        let assign780_e1684: f64 = (assign780_e1680 + assign780_e1683);
        let assign780_e1687: f64 = (p.p390 * var_inv_wl);
        let assign780_e1688: f64 = (assign780_e1684 + assign780_e1687);
        var_vknee2nw_i = assign780_e1688;

        let assign790_e1692: f64 = (p.p391 * var_inv_l);
        let assign790_e1693: f64 = (p.p282 + assign790_e1692);
        let assign790_e1696: f64 = (p.p392 * var_inv_w);
        let assign790_e1697: f64 = (assign790_e1693 + assign790_e1696);
        let assign790_e1700: f64 = (p.p393 * var_inv_wl);
        let assign790_e1701: f64 = (assign790_e1697 + assign790_e1700);
        var_dbgnw_i = assign790_e1701;

        let assign800_e1705: f64 = (p.p394 * var_inv_l);
        let assign800_e1706: f64 = (p.p279 + assign800_e1705);
        let assign800_e1709: f64 = (p.p395 * var_inv_w);
        let assign800_e1710: f64 = (assign800_e1706 + assign800_e1709);
        let assign800_e1713: f64 = (p.p396 * var_inv_wl);
        let assign800_e1714: f64 = (assign800_e1710 + assign800_e1713);
        var_kbg0nw_i = assign800_e1714;

        let assign810_e1718: f64 = (p.p397 * var_inv_l);
        let assign810_e1719: f64 = (p.p280 + assign810_e1718);
        let assign810_e1722: f64 = (p.p398 * var_inv_w);
        let assign810_e1723: f64 = (assign810_e1719 + assign810_e1722);
        let assign810_e1726: f64 = (p.p399 * var_inv_wl);
        let assign810_e1727: f64 = (assign810_e1723 + assign810_e1726);
        var_kbg1nw_i = assign810_e1727;

        let assign820_e1731: f64 = (p.p400 * var_inv_l);
        let assign820_e1732: f64 = (p.p281 + assign820_e1731);
        let assign820_e1735: f64 = (p.p401 * var_inv_w);
        let assign820_e1736: f64 = (assign820_e1732 + assign820_e1735);
        let assign820_e1739: f64 = (p.p402 * var_inv_wl);
        let assign820_e1740: f64 = (assign820_e1736 + assign820_e1739);
        var_kbg2nw_i = assign820_e1740;

        let assign830_e1744: f64 = (p.p403 * var_inv_l);
        let assign830_e1745: f64 = (p.p71 + assign830_e1744);
        let assign830_e1748: f64 = (p.p404 * var_inv_w);
        let assign830_e1749: f64 = (assign830_e1745 + assign830_e1748);
        let assign830_e1752: f64 = (p.p405 * var_inv_wl);
        let assign830_e1753: f64 = (assign830_e1749 + assign830_e1752);
        var_dvt0_i = assign830_e1753;

        let assign840_e1757: f64 = (p.p406 * var_inv_l);
        let assign840_e1758: f64 = (p.p72 + assign840_e1757);
        let assign840_e1761: f64 = (p.p407 * var_inv_w);
        let assign840_e1762: f64 = (assign840_e1758 + assign840_e1761);
        let assign840_e1765: f64 = (p.p408 * var_inv_wl);
        let assign840_e1766: f64 = (assign840_e1762 + assign840_e1765);
        var_dvt1_i = assign840_e1766;

        let assign850_e1770: f64 = (p.p409 * var_inv_l);
        let assign850_e1771: f64 = (p.p73 + assign850_e1770);
        let assign850_e1774: f64 = (p.p410 * var_inv_w);
        let assign850_e1775: f64 = (assign850_e1771 + assign850_e1774);
        let assign850_e1778: f64 = (p.p411 * var_inv_wl);
        let assign850_e1779: f64 = (assign850_e1775 + assign850_e1778);
        var_phin_i = assign850_e1779;

        *var_bpfactornw_i_slot = var_bpfactornw_i;
        *var_bpfactorpw_i_slot = var_bpfactorpw_i;
        *var_cbgcbg_i_slot = var_cbgcbg_i;
        *var_cdsc_i_slot = var_cdsc_i;
        *var_cdscd_i_slot = var_cdscd_i;
        *var_cit_i_slot = var_cit_i;
        *var_dbgnw_i_slot = var_dbgnw_i;
        *var_dbgpw_i_slot = var_dbgpw_i;
        *var_devsign_slot = var_devsign;
        *var_dlcv_slot = var_dlcv;
        *var_dliv_slot = var_dliv;
        *var_dvt0_i_slot = var_dvt0_i;
        *var_dvt1_i_slot = var_dvt1_i;
        *var_dwcv_slot = var_dwcv;
        *var_dwiv_slot = var_dwiv;
        *var_epssi_slot = var_epssi;
        *var_gdpr_slot = var_gdpr;
        *var_gdpr_dn3_slot = var_gdpr_dn3;
        *var_gdpr_dn4_slot = var_gdpr_dn4;
        *var_gdpr_dn5_slot = var_gdpr_dn5;
        *var_gdpr_dn6_slot = var_gdpr_dn6;
        *var_gdpr_dn7_slot = var_gdpr_dn7;
        *var_gdpr_dn8_slot = var_gdpr_dn8;
        *var_gspr_slot = var_gspr;
        *var_gspr_dn3_slot = var_gspr_dn3;
        *var_gspr_dn4_slot = var_gspr_dn4;
        *var_gspr_dn5_slot = var_gspr_dn5;
        *var_gspr_dn6_slot = var_gspr_dn6;
        *var_gspr_dn7_slot = var_gspr_dn7;
        *var_gspr_dn8_slot = var_gspr_dn8;
        *var_guard14_slot = var_guard14;
        *var_guard15_slot = var_guard15;
        *var_guard16_slot = var_guard16;
        *var_guard17_slot = var_guard17;
        *var_guard3_slot = var_guard3;
        *var_guard4_slot = var_guard4;
        *var_guard5_slot = var_guard5;
        *var_inv_l_slot = var_inv_l;
        *var_inv_w_slot = var_inv_w;
        *var_inv_wl_slot = var_inv_wl;
        *var_kbg0nw_i_slot = var_kbg0nw_i;
        *var_kbg0pw_i_slot = var_kbg0pw_i;
        *var_kbg1nw_i_slot = var_kbg1nw_i;
        *var_kbg1pw_i_slot = var_kbg1pw_i;
        *var_kbg2nw_i_slot = var_kbg2nw_i;
        *var_kbg2pw_i_slot = var_kbg2pw_i;
        *var_l_lln_slot = var_l_lln;
        *var_l_wln_slot = var_l_wln;
        *var_leff_slot = var_leff;
        *var_leffcv_slot = var_leffcv;
        *var_lnew_slot = var_lnew;
        *var_lw_lln_lwn_slot = var_lw_lln_lwn;
        *var_lw_wln_wwn_slot = var_lw_wln_wwn;
        *var_mpower_i_slot = var_mpower_i;
        *var_nbody_i_slot = var_nbody_i;
        *var_noia2_i_slot = var_noia2_i;
        *var_nsd_i_slot = var_nsd_i;
        *var_phig1_i_slot = var_phig1_i;
        *var_phig2_i_slot = var_phig2_i;
        *var_phig2_i_dn3_slot = var_phig2_i_dn3;
        *var_phig2_i_dn4_slot = var_phig2_i_dn4;
        *var_phig2_i_dn5_slot = var_phig2_i_dn5;
        *var_phig2_i_dn6_slot = var_phig2_i_dn6;
        *var_phig2_i_dn7_slot = var_phig2_i_dn7;
        *var_phig2_i_dn8_slot = var_phig2_i_dn8;
        *var_phin_i_slot = var_phin_i;
        *var_prwb_i_slot = var_prwb_i;
        *var_prwg_i_slot = var_prwg_i;
        *var_qsref_i_slot = var_qsref_i;
        *var_rdrain_slot = var_rdrain;
        *var_rdrain_dn3_slot = var_rdrain_dn3;
        *var_rdrain_dn4_slot = var_rdrain_dn4;
        *var_rdrain_dn5_slot = var_rdrain_dn5;
        *var_rdrain_dn6_slot = var_rdrain_dn6;
        *var_rdrain_dn7_slot = var_rdrain_dn7;
        *var_rdrain_dn8_slot = var_rdrain_dn8;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdw_i_slot = var_rdw_i;
        *var_rsource_slot = var_rsource;
        *var_rsource_dn3_slot = var_rsource_dn3;
        *var_rsource_dn4_slot = var_rsource_dn4;
        *var_rsource_dn5_slot = var_rsource_dn5;
        *var_rsource_dn6_slot = var_rsource_dn6;
        *var_rsource_dn7_slot = var_rsource_dn7;
        *var_rsource_dn8_slot = var_rsource_dn8;
        *var_rsw_i_slot = var_rsw_i;
        *var_vknee1nw_i_slot = var_vknee1nw_i;
        *var_vknee1pw_i_slot = var_vknee1pw_i;
        *var_vknee2nw_i_slot = var_vknee2nw_i;
        *var_vknee2pw_i_slot = var_vknee2pw_i;
        *var_w_lwn_slot = var_w_lwn;
        *var_w_wwn_slot = var_w_wwn;
        *var_weff_slot = var_weff;
        *var_weffcv_slot = var_weffcv;
        *var_welsign_slot = var_welsign;
        *var_wnew_slot = var_wnew;
        *var_wr_i_slot = var_wr_i;
    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_aigbacc_i_slot: &mut f64,
        var_aigbinv_i_slot: &mut f64,
        var_alpha0_i_slot: &mut f64,
        var_alpha1_i_slot: &mut f64,
        var_ascl_i_slot: &mut f64,
        var_at_i_slot: &mut f64,
        var_atb_i_slot: &mut f64,
        var_beta0_i_slot: &mut f64,
        var_bigbinv_i_slot: &mut f64,
        var_bscl_i_slot: &mut f64,
        var_cigbinv_i_slot: &mut f64,
        var_drout_i_slot: &mut f64,
        var_dsc0_i_slot: &mut f64,
        var_dsc1_i_slot: &mut f64,
        var_dsub_i_slot: &mut f64,
        var_eigbinv_i_slot: &mut f64,
        var_eta0_i_slot: &mut f64,
        var_eta1_i_slot: &mut f64,
        var_etab_i_slot: &mut f64,
        var_etamob2_i_slot: &mut f64,
        var_etamob_i_slot: &mut f64,
        var_eu2_i_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eub2_i_slot: &mut f64,
        var_eub_i_slot: &mut f64,
        var_igt_i_slot: &mut f64,
        var_iit_i_slot: &mut f64,
        var_k01_i_slot: &mut f64,
        var_k0_i_slot: &mut f64,
        var_k0si1_i_slot: &mut f64,
        var_k0si_i_slot: &mut f64,
        var_k0sisat1_i_slot: &mut f64,
        var_k0sisat_i_slot: &mut f64,
        var_k1rsce_i_slot: &mut f64,
        var_lpe0_i_slot: &mut f64,
        var_mexp_i_slot: &mut f64,
        var_nigbinv_i_slot: &mut f64,
        var_pclm_i_slot: &mut f64,
        var_pclmcv_i_slot: &mut f64,
        var_pdibl1_i_slot: &mut f64,
        var_pdibl2_i_slot: &mut f64,
        var_prt_i_slot: &mut f64,
        var_ptwg_i_slot: &mut f64,
        var_ptwgb2_i_slot: &mut f64,
        var_ptwgb_i_slot: &mut f64,
        var_ptwgt_i_slot: &mut f64,
        var_pvag_i_slot: &mut f64,
        var_tgidl_i_slot: &mut f64,
        var_tgisl_i_slot: &mut f64,
        var_u02_i_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_ua1_i_slot: &mut f64,
        var_ua2_i_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_uc2_i_slot: &mut f64,
        var_uc_i_slot: &mut f64,
        var_ucs2_i_slot: &mut f64,
        var_ucs_i_slot: &mut f64,
        var_ucste_i_slot: &mut f64,
        var_ud1_i_slot: &mut f64,
        var_ud2_i_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_ute_i_slot: &mut f64,
        var_utl_i_slot: &mut f64,
    ) {
        let mut var_aigbacc_i: f64 = *var_aigbacc_i_slot;
        let mut var_aigbinv_i: f64 = *var_aigbinv_i_slot;
        let mut var_alpha0_i: f64 = *var_alpha0_i_slot;
        let mut var_alpha1_i: f64 = *var_alpha1_i_slot;
        let mut var_ascl_i: f64 = *var_ascl_i_slot;
        let mut var_at_i: f64 = *var_at_i_slot;
        let mut var_atb_i: f64 = *var_atb_i_slot;
        let mut var_beta0_i: f64 = *var_beta0_i_slot;
        let mut var_bigbinv_i: f64 = *var_bigbinv_i_slot;
        let mut var_bscl_i: f64 = *var_bscl_i_slot;
        let mut var_cigbinv_i: f64 = *var_cigbinv_i_slot;
        let mut var_drout_i: f64 = *var_drout_i_slot;
        let mut var_dsc0_i: f64 = *var_dsc0_i_slot;
        let mut var_dsc1_i: f64 = *var_dsc1_i_slot;
        let mut var_dsub_i: f64 = *var_dsub_i_slot;
        let mut var_eigbinv_i: f64 = *var_eigbinv_i_slot;
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta1_i: f64 = *var_eta1_i_slot;
        let mut var_etab_i: f64 = *var_etab_i_slot;
        let mut var_etamob2_i: f64 = *var_etamob2_i_slot;
        let mut var_etamob_i: f64 = *var_etamob_i_slot;
        let mut var_eu2_i: f64 = *var_eu2_i_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eub2_i: f64 = *var_eub2_i_slot;
        let mut var_eub_i: f64 = *var_eub_i_slot;
        let mut var_igt_i: f64 = *var_igt_i_slot;
        let mut var_iit_i: f64 = *var_iit_i_slot;
        let mut var_k01_i: f64 = *var_k01_i_slot;
        let mut var_k0_i: f64 = *var_k0_i_slot;
        let mut var_k0si1_i: f64 = *var_k0si1_i_slot;
        let mut var_k0si_i: f64 = *var_k0si_i_slot;
        let mut var_k0sisat1_i: f64 = *var_k0sisat1_i_slot;
        let mut var_k0sisat_i: f64 = *var_k0sisat_i_slot;
        let mut var_k1rsce_i: f64 = *var_k1rsce_i_slot;
        let mut var_lpe0_i: f64 = *var_lpe0_i_slot;
        let mut var_mexp_i: f64 = *var_mexp_i_slot;
        let mut var_nigbinv_i: f64 = *var_nigbinv_i_slot;
        let mut var_pclm_i: f64 = *var_pclm_i_slot;
        let mut var_pclmcv_i: f64 = *var_pclmcv_i_slot;
        let mut var_pdibl1_i: f64 = *var_pdibl1_i_slot;
        let mut var_pdibl2_i: f64 = *var_pdibl2_i_slot;
        let mut var_prt_i: f64 = *var_prt_i_slot;
        let mut var_ptwg_i: f64 = *var_ptwg_i_slot;
        let mut var_ptwgb2_i: f64 = *var_ptwgb2_i_slot;
        let mut var_ptwgb_i: f64 = *var_ptwgb_i_slot;
        let mut var_ptwgt_i: f64 = *var_ptwgt_i_slot;
        let mut var_pvag_i: f64 = *var_pvag_i_slot;
        let mut var_tgidl_i: f64 = *var_tgidl_i_slot;
        let mut var_tgisl_i: f64 = *var_tgisl_i_slot;
        let mut var_u02_i: f64 = *var_u02_i_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_ua1_i: f64 = *var_ua1_i_slot;
        let mut var_ua2_i: f64 = *var_ua2_i_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_uc2_i: f64 = *var_uc2_i_slot;
        let mut var_uc_i: f64 = *var_uc_i_slot;
        let mut var_ucs2_i: f64 = *var_ucs2_i_slot;
        let mut var_ucs_i: f64 = *var_ucs_i_slot;
        let mut var_ucste_i: f64 = *var_ucste_i_slot;
        let mut var_ud1_i: f64 = *var_ud1_i_slot;
        let mut var_ud2_i: f64 = *var_ud2_i_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_ute_i: f64 = *var_ute_i_slot;
        let mut var_utl_i: f64 = *var_utl_i_slot;

        let assign860_e1783: f64 = (p.p412 * var_inv_l);
        let assign860_e1784: f64 = (p.p74 + assign860_e1783);
        let assign860_e1787: f64 = (p.p413 * var_inv_w);
        let assign860_e1788: f64 = (assign860_e1784 + assign860_e1787);
        let assign860_e1791: f64 = (p.p414 * var_inv_wl);
        let assign860_e1792: f64 = (assign860_e1788 + assign860_e1791);
        var_eta0_i = assign860_e1792;

        let assign870_e1796: f64 = (p.p415 * var_inv_l);
        let assign870_e1797: f64 = (p.p75 + assign870_e1796);
        let assign870_e1800: f64 = (p.p416 * var_inv_w);
        let assign870_e1801: f64 = (assign870_e1797 + assign870_e1800);
        let assign870_e1804: f64 = (p.p417 * var_inv_wl);
        let assign870_e1805: f64 = (assign870_e1801 + assign870_e1804);
        var_eta1_i = assign870_e1805;

        let assign880_e1809: f64 = (p.p418 * var_inv_l);
        let assign880_e1810: f64 = (p.p84 + assign880_e1809);
        let assign880_e1813: f64 = (p.p419 * var_inv_w);
        let assign880_e1814: f64 = (assign880_e1810 + assign880_e1813);
        let assign880_e1817: f64 = (p.p420 * var_inv_wl);
        let assign880_e1818: f64 = (assign880_e1814 + assign880_e1817);
        var_etab_i = assign880_e1818;

        let assign890_e1822: f64 = (p.p421 * var_inv_l);
        let assign890_e1823: f64 = (p.p76 + assign890_e1822);
        let assign890_e1826: f64 = (p.p422 * var_inv_w);
        let assign890_e1827: f64 = (assign890_e1823 + assign890_e1826);
        let assign890_e1830: f64 = (p.p423 * var_inv_wl);
        let assign890_e1831: f64 = (assign890_e1827 + assign890_e1830);
        var_dsub_i = assign890_e1831;

        let assign900_e1835: f64 = (p.p430 * var_inv_l);
        let assign900_e1836: f64 = (p.p87 + assign900_e1835);
        let assign900_e1839: f64 = (p.p431 * var_inv_w);
        let assign900_e1840: f64 = (assign900_e1836 + assign900_e1839);
        let assign900_e1843: f64 = (p.p432 * var_inv_wl);
        let assign900_e1844: f64 = (assign900_e1840 + assign900_e1843);
        var_dsc0_i = assign900_e1844;

        let assign910_e1848: f64 = (p.p433 * var_inv_l);
        let assign910_e1849: f64 = (p.p88 + assign910_e1848);
        let assign910_e1852: f64 = (p.p434 * var_inv_w);
        let assign910_e1853: f64 = (assign910_e1849 + assign910_e1852);
        let assign910_e1856: f64 = (p.p435 * var_inv_wl);
        let assign910_e1857: f64 = (assign910_e1853 + assign910_e1856);
        var_dsc1_i = assign910_e1857;

        let assign920_e1861: f64 = (p.p436 * var_inv_l);
        let assign920_e1862: f64 = (p.p61 + assign920_e1861);
        let assign920_e1865: f64 = (p.p437 * var_inv_w);
        let assign920_e1866: f64 = (assign920_e1862 + assign920_e1865);
        let assign920_e1869: f64 = (p.p438 * var_inv_wl);
        let assign920_e1870: f64 = (assign920_e1866 + assign920_e1869);
        var_ascl_i = assign920_e1870;

        let assign930_e1874: f64 = (p.p439 * var_inv_l);
        let assign930_e1875: f64 = (p.p62 + assign930_e1874);
        let assign930_e1878: f64 = (p.p440 * var_inv_w);
        let assign930_e1879: f64 = (assign930_e1875 + assign930_e1878);
        let assign930_e1882: f64 = (p.p441 * var_inv_wl);
        let assign930_e1883: f64 = (assign930_e1879 + assign930_e1882);
        var_bscl_i = assign930_e1883;

        let assign940_e1887: f64 = (p.p424 * var_inv_l);
        let assign940_e1888: f64 = (p.p85 + assign940_e1887);
        let assign940_e1891: f64 = (p.p425 * var_inv_w);
        let assign940_e1892: f64 = (assign940_e1888 + assign940_e1891);
        let assign940_e1895: f64 = (p.p426 * var_inv_wl);
        let assign940_e1896: f64 = (assign940_e1892 + assign940_e1895);
        var_k1rsce_i = assign940_e1896;

        let assign950_e1900: f64 = (p.p427 * var_inv_l);
        let assign950_e1901: f64 = (p.p86 + assign950_e1900);
        let assign950_e1904: f64 = (p.p428 * var_inv_w);
        let assign950_e1905: f64 = (assign950_e1901 + assign950_e1904);
        let assign950_e1908: f64 = (p.p429 * var_inv_wl);
        let assign950_e1909: f64 = (assign950_e1905 + assign950_e1908);
        var_lpe0_i = assign950_e1909;

        let assign960_e1913: f64 = (p.p460 * var_inv_l);
        let assign960_e1914: f64 = (p.p113 + assign960_e1913);
        let assign960_e1917: f64 = (p.p461 * var_inv_w);
        let assign960_e1918: f64 = (assign960_e1914 + assign960_e1917);
        let assign960_e1921: f64 = (p.p462 * var_inv_wl);
        let assign960_e1922: f64 = (assign960_e1918 + assign960_e1921);
        var_mexp_i = assign960_e1922;

        let assign970_e1926: f64 = (p.p442 * var_inv_l);
        let assign970_e1927: f64 = (p.p89 + assign970_e1926);
        let assign970_e1930: f64 = (p.p443 * var_inv_w);
        let assign970_e1931: f64 = (assign970_e1927 + assign970_e1930);
        let assign970_e1934: f64 = (p.p444 * var_inv_wl);
        let assign970_e1935: f64 = (assign970_e1931 + assign970_e1934);
        var_k0_i = assign970_e1935;

        let assign980_e1939: f64 = (p.p445 * var_inv_l);
        let assign980_e1940: f64 = (p.p90 + assign980_e1939);
        let assign980_e1943: f64 = (p.p446 * var_inv_w);
        let assign980_e1944: f64 = (assign980_e1940 + assign980_e1943);
        let assign980_e1947: f64 = (p.p447 * var_inv_wl);
        let assign980_e1948: f64 = (assign980_e1944 + assign980_e1947);
        var_k01_i = assign980_e1948;

        let assign990_e1952: f64 = (p.p448 * var_inv_l);
        let assign990_e1953: f64 = (p.p91 + assign990_e1952);
        let assign990_e1956: f64 = (p.p449 * var_inv_w);
        let assign990_e1957: f64 = (assign990_e1953 + assign990_e1956);
        let assign990_e1960: f64 = (p.p450 * var_inv_wl);
        let assign990_e1961: f64 = (assign990_e1957 + assign990_e1960);
        var_k0si_i = assign990_e1961;

        let assign1000_e1965: f64 = (p.p451 * var_inv_l);
        let assign1000_e1966: f64 = (p.p92 + assign1000_e1965);
        let assign1000_e1969: f64 = (p.p452 * var_inv_w);
        let assign1000_e1970: f64 = (assign1000_e1966 + assign1000_e1969);
        let assign1000_e1973: f64 = (p.p453 * var_inv_wl);
        let assign1000_e1974: f64 = (assign1000_e1970 + assign1000_e1973);
        var_k0si1_i = assign1000_e1974;

        let assign1010_e1978: f64 = (p.p454 * var_inv_l);
        let assign1010_e1979: f64 = (p.p93 + assign1010_e1978);
        let assign1010_e1982: f64 = (p.p455 * var_inv_w);
        let assign1010_e1983: f64 = (assign1010_e1979 + assign1010_e1982);
        let assign1010_e1986: f64 = (p.p456 * var_inv_wl);
        let assign1010_e1987: f64 = (assign1010_e1983 + assign1010_e1986);
        var_k0sisat_i = assign1010_e1987;

        let assign1020_e1991: f64 = (p.p457 * var_inv_l);
        let assign1020_e1992: f64 = (p.p94 + assign1020_e1991);
        let assign1020_e1995: f64 = (p.p458 * var_inv_w);
        let assign1020_e1996: f64 = (assign1020_e1992 + assign1020_e1995);
        let assign1020_e1999: f64 = (p.p459 * var_inv_wl);
        let assign1020_e2000: f64 = (assign1020_e1996 + assign1020_e1999);
        var_k0sisat1_i = assign1020_e2000;

        let assign1030_e2004: f64 = (p.p463 * var_inv_l);
        let assign1030_e2005: f64 = (p.p116 + assign1030_e2004);
        let assign1030_e2008: f64 = (p.p464 * var_inv_w);
        let assign1030_e2009: f64 = (assign1030_e2005 + assign1030_e2008);
        let assign1030_e2012: f64 = (p.p465 * var_inv_wl);
        let assign1030_e2013: f64 = (assign1030_e2009 + assign1030_e2012);
        var_ptwg_i = assign1030_e2013;

        let assign1040_e2017: f64 = (p.p466 * var_inv_l);
        let assign1040_e2018: f64 = (p.p123 + assign1040_e2017);
        let assign1040_e2021: f64 = (p.p467 * var_inv_w);
        let assign1040_e2022: f64 = (assign1040_e2018 + assign1040_e2021);
        let assign1040_e2025: f64 = (p.p468 * var_inv_wl);
        let assign1040_e2026: f64 = (assign1040_e2022 + assign1040_e2025);
        var_ptwgb_i = assign1040_e2026;

        let assign1050_e2030: f64 = (p.p469 * var_inv_l);
        let assign1050_e2031: f64 = (p.p124 + assign1050_e2030);
        let assign1050_e2034: f64 = (p.p470 * var_inv_w);
        let assign1050_e2035: f64 = (assign1050_e2031 + assign1050_e2034);
        let assign1050_e2038: f64 = (p.p471 * var_inv_wl);
        let assign1050_e2039: f64 = (assign1050_e2035 + assign1050_e2038);
        var_ptwgb2_i = assign1050_e2039;

        let assign1060_e2043: f64 = (p.p472 * var_inv_l);
        let assign1060_e2044: f64 = (p.p122 + assign1060_e2043);
        let assign1060_e2047: f64 = (p.p473 * var_inv_w);
        let assign1060_e2048: f64 = (assign1060_e2044 + assign1060_e2047);
        let assign1060_e2051: f64 = (p.p474 * var_inv_wl);
        let assign1060_e2052: f64 = (assign1060_e2048 + assign1060_e2051);
        var_ptwgt_i = assign1060_e2052;

        let assign1070_e2056: f64 = (p.p475 * var_inv_l);
        let assign1070_e2057: f64 = (p.p135 + assign1070_e2056);
        let assign1070_e2060: f64 = (p.p476 * var_inv_w);
        let assign1070_e2061: f64 = (assign1070_e2057 + assign1070_e2060);
        let assign1070_e2064: f64 = (p.p477 * var_inv_wl);
        let assign1070_e2065: f64 = (assign1070_e2061 + assign1070_e2064);
        var_u0_i = assign1070_e2065;

        let assign1080_e2069: f64 = (p.p478 * var_inv_l);
        let assign1080_e2070: f64 = (p.p139 + assign1080_e2069);
        let assign1080_e2073: f64 = (p.p479 * var_inv_w);
        let assign1080_e2074: f64 = (assign1080_e2070 + assign1080_e2073);
        let assign1080_e2077: f64 = (p.p480 * var_inv_wl);
        let assign1080_e2078: f64 = (assign1080_e2074 + assign1080_e2077);
        var_ua_i = assign1080_e2078;

        let assign1090_e2082: f64 = (p.p481 * var_inv_l);
        let assign1090_e2083: f64 = (p.p145 + assign1090_e2082);
        let assign1090_e2086: f64 = (p.p482 * var_inv_w);
        let assign1090_e2087: f64 = (assign1090_e2083 + assign1090_e2086);
        let assign1090_e2090: f64 = (p.p483 * var_inv_wl);
        let assign1090_e2091: f64 = (assign1090_e2087 + assign1090_e2090);
        var_uc_i = assign1090_e2091;

        let assign1100_e2095: f64 = (p.p484 * var_inv_l);
        let assign1100_e2096: f64 = (p.p148 + assign1100_e2095);
        let assign1100_e2099: f64 = (p.p485 * var_inv_w);
        let assign1100_e2100: f64 = (assign1100_e2096 + assign1100_e2099);
        let assign1100_e2103: f64 = (p.p486 * var_inv_wl);
        let assign1100_e2104: f64 = (assign1100_e2100 + assign1100_e2103);
        var_ud_i = assign1100_e2104;

        let assign1110_e2108: f64 = (p.p487 * var_inv_l);
        let assign1110_e2109: f64 = (p.p155 + assign1110_e2108);
        let assign1110_e2112: f64 = (p.p488 * var_inv_w);
        let assign1110_e2113: f64 = (assign1110_e2109 + assign1110_e2112);
        let assign1110_e2116: f64 = (p.p489 * var_inv_wl);
        let assign1110_e2117: f64 = (assign1110_e2113 + assign1110_e2116);
        var_ucs_i = assign1110_e2117;

        let assign1120_e2121: f64 = (p.p490 * var_inv_l);
        let assign1120_e2122: f64 = (p.p142 + assign1120_e2121);
        let assign1120_e2125: f64 = (p.p491 * var_inv_w);
        let assign1120_e2126: f64 = (assign1120_e2122 + assign1120_e2125);
        let assign1120_e2129: f64 = (p.p492 * var_inv_wl);
        let assign1120_e2130: f64 = (assign1120_e2126 + assign1120_e2129);
        var_eu_i = assign1120_e2130;

        let assign1130_e2134: f64 = (p.p493 * var_inv_l);
        let assign1130_e2135: f64 = (p.p163 + assign1130_e2134);
        let assign1130_e2138: f64 = (p.p494 * var_inv_w);
        let assign1130_e2139: f64 = (assign1130_e2135 + assign1130_e2138);
        let assign1130_e2142: f64 = (p.p495 * var_inv_wl);
        let assign1130_e2143: f64 = (assign1130_e2139 + assign1130_e2142);
        var_eub_i = assign1130_e2143;

        let assign1140_e2147: f64 = (p.p496 * var_inv_l);
        let assign1140_e2148: f64 = (p.p157 + assign1140_e2147);
        let assign1140_e2151: f64 = (p.p497 * var_inv_w);
        let assign1140_e2152: f64 = (assign1140_e2148 + assign1140_e2151);
        let assign1140_e2155: f64 = (p.p498 * var_inv_wl);
        let assign1140_e2156: f64 = (assign1140_e2152 + assign1140_e2155);
        var_utl_i = assign1140_e2156;

        let assign1150_e2160: f64 = (p.p499 * var_inv_l);
        let assign1150_e2161: f64 = (p.p156 + assign1150_e2160);
        let assign1150_e2164: f64 = (p.p500 * var_inv_w);
        let assign1150_e2165: f64 = (assign1150_e2161 + assign1150_e2164);
        let assign1150_e2168: f64 = (p.p501 * var_inv_wl);
        let assign1150_e2169: f64 = (assign1150_e2165 + assign1150_e2168);
        var_ute_i = assign1150_e2169;

        let assign1160_e2173: f64 = (p.p502 * var_inv_l);
        let assign1160_e2174: f64 = (p.p158 + assign1160_e2173);
        let assign1160_e2177: f64 = (p.p503 * var_inv_w);
        let assign1160_e2178: f64 = (assign1160_e2174 + assign1160_e2177);
        let assign1160_e2181: f64 = (p.p504 * var_inv_wl);
        let assign1160_e2182: f64 = (assign1160_e2178 + assign1160_e2181);
        var_ua1_i = assign1160_e2182;

        let assign1170_e2186: f64 = (p.p505 * var_inv_l);
        let assign1170_e2187: f64 = (p.p160 + assign1170_e2186);
        let assign1170_e2190: f64 = (p.p506 * var_inv_w);
        let assign1170_e2191: f64 = (assign1170_e2187 + assign1170_e2190);
        let assign1170_e2194: f64 = (p.p507 * var_inv_wl);
        let assign1170_e2195: f64 = (assign1170_e2191 + assign1170_e2194);
        var_ud1_i = assign1170_e2195;

        let assign1180_e2199: f64 = (p.p508 * var_inv_l);
        let assign1180_e2200: f64 = (p.p161 + assign1180_e2199);
        let assign1180_e2203: f64 = (p.p509 * var_inv_w);
        let assign1180_e2204: f64 = (assign1180_e2200 + assign1180_e2203);
        let assign1180_e2207: f64 = (p.p510 * var_inv_wl);
        let assign1180_e2208: f64 = (assign1180_e2204 + assign1180_e2207);
        var_ucste_i = assign1180_e2208;

        let assign1190_e2212: f64 = (p.p511 * var_inv_l);
        let assign1190_e2213: f64 = (p.p136 + assign1190_e2212);
        let assign1190_e2216: f64 = (p.p512 * var_inv_w);
        let assign1190_e2217: f64 = (assign1190_e2213 + assign1190_e2216);
        let assign1190_e2220: f64 = (p.p513 * var_inv_wl);
        let assign1190_e2221: f64 = (assign1190_e2217 + assign1190_e2220);
        var_etamob_i = assign1190_e2221;

        let assign1200_e2225: f64 = (p.p514 * var_inv_l);
        let assign1200_e2226: f64 = (p.p166 + assign1200_e2225);
        let assign1200_e2229: f64 = (p.p515 * var_inv_w);
        let assign1200_e2230: f64 = (assign1200_e2226 + assign1200_e2229);
        let assign1200_e2233: f64 = (p.p516 * var_inv_wl);
        let assign1200_e2234: f64 = (assign1200_e2230 + assign1200_e2233);
        var_u02_i = assign1200_e2234;

        let assign1210_e2238: f64 = (p.p517 * var_inv_l);
        let assign1210_e2239: f64 = (p.p167 + assign1210_e2238);
        let assign1210_e2242: f64 = (p.p518 * var_inv_w);
        let assign1210_e2243: f64 = (assign1210_e2239 + assign1210_e2242);
        let assign1210_e2246: f64 = (p.p519 * var_inv_wl);
        let assign1210_e2247: f64 = (assign1210_e2243 + assign1210_e2246);
        var_ua2_i = assign1210_e2247;

        let assign1220_e2251: f64 = (p.p520 * var_inv_l);
        let assign1220_e2252: f64 = (p.p173 + assign1220_e2251);
        let assign1220_e2255: f64 = (p.p521 * var_inv_w);
        let assign1220_e2256: f64 = (assign1220_e2252 + assign1220_e2255);
        let assign1220_e2259: f64 = (p.p522 * var_inv_wl);
        let assign1220_e2260: f64 = (assign1220_e2256 + assign1220_e2259);
        var_uc2_i = assign1220_e2260;

        let assign1230_e2264: f64 = (p.p523 * var_inv_l);
        let assign1230_e2265: f64 = (p.p176 + assign1230_e2264);
        let assign1230_e2268: f64 = (p.p524 * var_inv_w);
        let assign1230_e2269: f64 = (assign1230_e2265 + assign1230_e2268);
        let assign1230_e2272: f64 = (p.p525 * var_inv_wl);
        let assign1230_e2273: f64 = (assign1230_e2269 + assign1230_e2272);
        var_ud2_i = assign1230_e2273;

        let assign1240_e2277: f64 = (p.p526 * var_inv_l);
        let assign1240_e2278: f64 = (p.p182 + assign1240_e2277);
        let assign1240_e2281: f64 = (p.p527 * var_inv_w);
        let assign1240_e2282: f64 = (assign1240_e2278 + assign1240_e2281);
        let assign1240_e2285: f64 = (p.p528 * var_inv_wl);
        let assign1240_e2286: f64 = (assign1240_e2282 + assign1240_e2285);
        var_ucs2_i = assign1240_e2286;

        let assign1250_e2290: f64 = (p.p529 * var_inv_l);
        let assign1250_e2291: f64 = (p.p170 + assign1250_e2290);
        let assign1250_e2294: f64 = (p.p530 * var_inv_w);
        let assign1250_e2295: f64 = (assign1250_e2291 + assign1250_e2294);
        let assign1250_e2298: f64 = (p.p531 * var_inv_wl);
        let assign1250_e2299: f64 = (assign1250_e2295 + assign1250_e2298);
        var_eu2_i = assign1250_e2299;

        let assign1260_e2303: f64 = (p.p532 * var_inv_l);
        let assign1260_e2304: f64 = (p.p183 + assign1260_e2303);
        let assign1260_e2307: f64 = (p.p533 * var_inv_w);
        let assign1260_e2308: f64 = (assign1260_e2304 + assign1260_e2307);
        let assign1260_e2311: f64 = (p.p534 * var_inv_wl);
        let assign1260_e2312: f64 = (assign1260_e2308 + assign1260_e2311);
        var_eub2_i = assign1260_e2312;

        let assign1270_e2316: f64 = (p.p535 * var_inv_l);
        let assign1270_e2317: f64 = (p.p186 + assign1270_e2316);
        let assign1270_e2320: f64 = (p.p536 * var_inv_w);
        let assign1270_e2321: f64 = (assign1270_e2317 + assign1270_e2320);
        let assign1270_e2324: f64 = (p.p537 * var_inv_wl);
        let assign1270_e2325: f64 = (assign1270_e2321 + assign1270_e2324);
        var_etamob2_i = assign1270_e2325;

        let assign1280_e2329: f64 = (p.p538 * var_inv_l);
        let assign1280_e2330: f64 = (p.p119 + assign1280_e2329);
        let assign1280_e2333: f64 = (p.p539 * var_inv_w);
        let assign1280_e2334: f64 = (assign1280_e2330 + assign1280_e2333);
        let assign1280_e2337: f64 = (p.p540 * var_inv_wl);
        let assign1280_e2338: f64 = (assign1280_e2334 + assign1280_e2337);
        var_at_i = assign1280_e2338;

        let assign1290_e2342: f64 = (p.p541 * var_inv_l);
        let assign1290_e2343: f64 = (p.p130 + assign1290_e2342);
        let assign1290_e2346: f64 = (p.p542 * var_inv_w);
        let assign1290_e2347: f64 = (assign1290_e2343 + assign1290_e2346);
        let assign1290_e2350: f64 = (p.p543 * var_inv_wl);
        let assign1290_e2351: f64 = (assign1290_e2347 + assign1290_e2350);
        var_atb_i = assign1290_e2351;

        let assign1300_e2355: f64 = (p.p544 * var_inv_l);
        let assign1300_e2356: f64 = (p.p205 + assign1300_e2355);
        let assign1300_e2359: f64 = (p.p545 * var_inv_w);
        let assign1300_e2360: f64 = (assign1300_e2356 + assign1300_e2359);
        let assign1300_e2363: f64 = (p.p546 * var_inv_wl);
        let assign1300_e2364: f64 = (assign1300_e2360 + assign1300_e2363);
        var_prt_i = assign1300_e2364;

        let assign1310_e2368: f64 = (p.p547 * var_inv_l);
        let assign1310_e2369: f64 = (p.p305 + assign1310_e2368);
        let assign1310_e2372: f64 = (p.p548 * var_inv_w);
        let assign1310_e2373: f64 = (assign1310_e2369 + assign1310_e2372);
        let assign1310_e2376: f64 = (p.p549 * var_inv_wl);
        let assign1310_e2377: f64 = (assign1310_e2373 + assign1310_e2376);
        var_iit_i = assign1310_e2377;

        let assign1320_e2381: f64 = (p.p550 * var_inv_l);
        let assign1320_e2382: f64 = (p.p306 + assign1320_e2381);
        let assign1320_e2385: f64 = (p.p551 * var_inv_w);
        let assign1320_e2386: f64 = (assign1320_e2382 + assign1320_e2385);
        let assign1320_e2389: f64 = (p.p552 * var_inv_wl);
        let assign1320_e2390: f64 = (assign1320_e2386 + assign1320_e2389);
        var_tgidl_i = assign1320_e2390;

        let assign1330_e2394: f64 = (p.p553 * var_inv_l);
        let assign1330_e2395: f64 = (p.p307 + assign1330_e2394);
        let assign1330_e2398: f64 = (p.p554 * var_inv_w);
        let assign1330_e2399: f64 = (assign1330_e2395 + assign1330_e2398);
        let assign1330_e2402: f64 = (p.p555 * var_inv_wl);
        let assign1330_e2403: f64 = (assign1330_e2399 + assign1330_e2402);
        var_tgisl_i = assign1330_e2403;

        let assign1340_e2407: f64 = (p.p556 * var_inv_l);
        let assign1340_e2408: f64 = (p.p308 + assign1340_e2407);
        let assign1340_e2411: f64 = (p.p557 * var_inv_w);
        let assign1340_e2412: f64 = (assign1340_e2408 + assign1340_e2411);
        let assign1340_e2415: f64 = (p.p558 * var_inv_wl);
        let assign1340_e2416: f64 = (assign1340_e2412 + assign1340_e2415);
        var_igt_i = assign1340_e2416;

        let assign1350_e2420: f64 = (p.p559 * var_inv_l);
        let assign1350_e2421: f64 = (p.p210 + assign1350_e2420);
        let assign1350_e2424: f64 = (p.p560 * var_inv_w);
        let assign1350_e2425: f64 = (assign1350_e2421 + assign1350_e2424);
        let assign1350_e2428: f64 = (p.p561 * var_inv_wl);
        let assign1350_e2429: f64 = (assign1350_e2425 + assign1350_e2428);
        var_pclm_i = assign1350_e2429;

        let assign1360_e2433: f64 = (p.p562 * var_inv_l);
        let assign1360_e2434: f64 = (p.p214 + assign1360_e2433);
        let assign1360_e2437: f64 = (p.p563 * var_inv_w);
        let assign1360_e2438: f64 = (assign1360_e2434 + assign1360_e2437);
        let assign1360_e2441: f64 = (p.p564 * var_inv_wl);
        let assign1360_e2442: f64 = (assign1360_e2438 + assign1360_e2441);
        var_pclmcv_i = assign1360_e2442;

        let assign1370_e2446: f64 = (p.p565 * var_inv_l);
        let assign1370_e2447: f64 = (p.p208 + assign1370_e2446);
        let assign1370_e2450: f64 = (p.p566 * var_inv_w);
        let assign1370_e2451: f64 = (assign1370_e2447 + assign1370_e2450);
        let assign1370_e2454: f64 = (p.p567 * var_inv_wl);
        let assign1370_e2455: f64 = (assign1370_e2451 + assign1370_e2454);
        var_drout_i = assign1370_e2455;

        let assign1380_e2459: f64 = (p.p568 * var_inv_l);
        let assign1380_e2460: f64 = (p.p206 + assign1380_e2459);
        let assign1380_e2463: f64 = (p.p569 * var_inv_w);
        let assign1380_e2464: f64 = (assign1380_e2460 + assign1380_e2463);
        let assign1380_e2467: f64 = (p.p570 * var_inv_wl);
        let assign1380_e2468: f64 = (assign1380_e2464 + assign1380_e2467);
        var_pdibl1_i = assign1380_e2468;

        let assign1390_e2472: f64 = (p.p571 * var_inv_l);
        let assign1390_e2473: f64 = (p.p207 + assign1390_e2472);
        let assign1390_e2476: f64 = (p.p572 * var_inv_w);
        let assign1390_e2477: f64 = (assign1390_e2473 + assign1390_e2476);
        let assign1390_e2480: f64 = (p.p573 * var_inv_wl);
        let assign1390_e2481: f64 = (assign1390_e2477 + assign1390_e2480);
        var_pdibl2_i = assign1390_e2481;

        let assign1400_e2485: f64 = (p.p574 * var_inv_l);
        let assign1400_e2486: f64 = (p.p209 + assign1400_e2485);
        let assign1400_e2489: f64 = (p.p575 * var_inv_w);
        let assign1400_e2490: f64 = (assign1400_e2486 + assign1400_e2489);
        let assign1400_e2493: f64 = (p.p576 * var_inv_wl);
        let assign1400_e2494: f64 = (assign1400_e2490 + assign1400_e2493);
        var_pvag_i = assign1400_e2494;

        let assign1410_e2498: f64 = (p.p577 * var_inv_l);
        let assign1410_e2499: f64 = (p.p256 + assign1410_e2498);
        let assign1410_e2502: f64 = (p.p578 * var_inv_w);
        let assign1410_e2503: f64 = (assign1410_e2499 + assign1410_e2502);
        let assign1410_e2506: f64 = (p.p579 * var_inv_wl);
        let assign1410_e2507: f64 = (assign1410_e2503 + assign1410_e2506);
        var_alpha0_i = assign1410_e2507;

        let assign1420_e2511: f64 = (p.p580 * var_inv_l);
        let assign1420_e2512: f64 = (p.p257 + assign1420_e2511);
        let assign1420_e2515: f64 = (p.p581 * var_inv_w);
        let assign1420_e2516: f64 = (assign1420_e2512 + assign1420_e2515);
        let assign1420_e2519: f64 = (p.p582 * var_inv_wl);
        let assign1420_e2520: f64 = (assign1420_e2516 + assign1420_e2519);
        var_alpha1_i = assign1420_e2520;

        let assign1430_e2524: f64 = (p.p583 * var_inv_l);
        let assign1430_e2525: f64 = (p.p258 + assign1430_e2524);
        let assign1430_e2528: f64 = (p.p584 * var_inv_w);
        let assign1430_e2529: f64 = (assign1430_e2525 + assign1430_e2528);
        let assign1430_e2532: f64 = (p.p585 * var_inv_wl);
        let assign1430_e2533: f64 = (assign1430_e2529 + assign1430_e2532);
        var_beta0_i = assign1430_e2533;

        let assign1440_e2537: f64 = (var_inv_l * p.p706);
        let assign1440_e2538: f64 = (p.p217 + assign1440_e2537);
        let assign1440_e2541: f64 = (var_inv_w * p.p707);
        let assign1440_e2542: f64 = (assign1440_e2538 + assign1440_e2541);
        let assign1440_e2545: f64 = (var_inv_wl * p.p708);
        let assign1440_e2546: f64 = (assign1440_e2542 + assign1440_e2545);
        var_aigbinv_i = assign1440_e2546;

        let assign1450_e2550: f64 = (var_inv_l * p.p709);
        let assign1450_e2551: f64 = (p.p218 + assign1450_e2550);
        let assign1450_e2554: f64 = (var_inv_w * p.p710);
        let assign1450_e2555: f64 = (assign1450_e2551 + assign1450_e2554);
        let assign1450_e2558: f64 = (var_inv_wl * p.p711);
        let assign1450_e2559: f64 = (assign1450_e2555 + assign1450_e2558);
        var_bigbinv_i = assign1450_e2559;

        let assign1460_e2563: f64 = (var_inv_l * p.p712);
        let assign1460_e2564: f64 = (p.p219 + assign1460_e2563);
        let assign1460_e2567: f64 = (var_inv_w * p.p713);
        let assign1460_e2568: f64 = (assign1460_e2564 + assign1460_e2567);
        let assign1460_e2571: f64 = (var_inv_wl * p.p714);
        let assign1460_e2572: f64 = (assign1460_e2568 + assign1460_e2571);
        var_cigbinv_i = assign1460_e2572;

        let assign1470_e2576: f64 = (var_inv_l * p.p715);
        let assign1470_e2577: f64 = (p.p220 + assign1470_e2576);
        let assign1470_e2580: f64 = (var_inv_w * p.p716);
        let assign1470_e2581: f64 = (assign1470_e2577 + assign1470_e2580);
        let assign1470_e2584: f64 = (var_inv_wl * p.p717);
        let assign1470_e2585: f64 = (assign1470_e2581 + assign1470_e2584);
        var_eigbinv_i = assign1470_e2585;

        let assign1480_e2589: f64 = (var_inv_l * p.p718);
        let assign1480_e2590: f64 = (p.p221 + assign1480_e2589);
        let assign1480_e2593: f64 = (var_inv_w * p.p719);
        let assign1480_e2594: f64 = (assign1480_e2590 + assign1480_e2593);
        let assign1480_e2597: f64 = (var_inv_wl * p.p720);
        let assign1480_e2598: f64 = (assign1480_e2594 + assign1480_e2597);
        var_nigbinv_i = assign1480_e2598;

        let assign1490_e2602: f64 = (var_inv_l * p.p721);
        let assign1490_e2603: f64 = (p.p222 + assign1490_e2602);
        let assign1490_e2606: f64 = (var_inv_w * p.p722);
        let assign1490_e2607: f64 = (assign1490_e2603 + assign1490_e2606);
        let assign1490_e2610: f64 = (var_inv_wl * p.p723);
        let assign1490_e2611: f64 = (assign1490_e2607 + assign1490_e2610);
        var_aigbacc_i = assign1490_e2611;

        *var_aigbacc_i_slot = var_aigbacc_i;
        *var_aigbinv_i_slot = var_aigbinv_i;
        *var_alpha0_i_slot = var_alpha0_i;
        *var_alpha1_i_slot = var_alpha1_i;
        *var_ascl_i_slot = var_ascl_i;
        *var_at_i_slot = var_at_i;
        *var_atb_i_slot = var_atb_i;
        *var_beta0_i_slot = var_beta0_i;
        *var_bigbinv_i_slot = var_bigbinv_i;
        *var_bscl_i_slot = var_bscl_i;
        *var_cigbinv_i_slot = var_cigbinv_i;
        *var_drout_i_slot = var_drout_i;
        *var_dsc0_i_slot = var_dsc0_i;
        *var_dsc1_i_slot = var_dsc1_i;
        *var_dsub_i_slot = var_dsub_i;
        *var_eigbinv_i_slot = var_eigbinv_i;
        *var_eta0_i_slot = var_eta0_i;
        *var_eta1_i_slot = var_eta1_i;
        *var_etab_i_slot = var_etab_i;
        *var_etamob2_i_slot = var_etamob2_i;
        *var_etamob_i_slot = var_etamob_i;
        *var_eu2_i_slot = var_eu2_i;
        *var_eu_i_slot = var_eu_i;
        *var_eub2_i_slot = var_eub2_i;
        *var_eub_i_slot = var_eub_i;
        *var_igt_i_slot = var_igt_i;
        *var_iit_i_slot = var_iit_i;
        *var_k01_i_slot = var_k01_i;
        *var_k0_i_slot = var_k0_i;
        *var_k0si1_i_slot = var_k0si1_i;
        *var_k0si_i_slot = var_k0si_i;
        *var_k0sisat1_i_slot = var_k0sisat1_i;
        *var_k0sisat_i_slot = var_k0sisat_i;
        *var_k1rsce_i_slot = var_k1rsce_i;
        *var_lpe0_i_slot = var_lpe0_i;
        *var_mexp_i_slot = var_mexp_i;
        *var_nigbinv_i_slot = var_nigbinv_i;
        *var_pclm_i_slot = var_pclm_i;
        *var_pclmcv_i_slot = var_pclmcv_i;
        *var_pdibl1_i_slot = var_pdibl1_i;
        *var_pdibl2_i_slot = var_pdibl2_i;
        *var_prt_i_slot = var_prt_i;
        *var_ptwg_i_slot = var_ptwg_i;
        *var_ptwgb2_i_slot = var_ptwgb2_i;
        *var_ptwgb_i_slot = var_ptwgb_i;
        *var_ptwgt_i_slot = var_ptwgt_i;
        *var_pvag_i_slot = var_pvag_i;
        *var_tgidl_i_slot = var_tgidl_i;
        *var_tgisl_i_slot = var_tgisl_i;
        *var_u02_i_slot = var_u02_i;
        *var_u0_i_slot = var_u0_i;
        *var_ua1_i_slot = var_ua1_i;
        *var_ua2_i_slot = var_ua2_i;
        *var_ua_i_slot = var_ua_i;
        *var_uc2_i_slot = var_uc2_i;
        *var_uc_i_slot = var_uc_i;
        *var_ucs2_i_slot = var_ucs2_i;
        *var_ucs_i_slot = var_ucs_i;
        *var_ucste_i_slot = var_ucste_i;
        *var_ud1_i_slot = var_ud1_i;
        *var_ud2_i_slot = var_ud2_i;
        *var_ud_i_slot = var_ud_i;
        *var_ute_i_slot = var_ute_i;
        *var_utl_i_slot = var_utl_i;
    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_epssi: f64,
        var_inv_l: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_leff: f64,
        var_agidl_i_slot: &mut f64,
        var_agisl_i_slot: &mut f64,
        var_aigc_i_slot: &mut f64,
        var_aigd_i_slot: &mut f64,
        var_aigs_i_slot: &mut f64,
        var_bgidl_i_slot: &mut f64,
        var_bgisl_i_slot: &mut f64,
        var_bigbacc_i_slot: &mut f64,
        var_bigc_i_slot: &mut f64,
        var_bigd_i_slot: &mut f64,
        var_bigs_i_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfs_i_slot: &mut f64,
        var_cigbacc_i_slot: &mut f64,
        var_cigc_i_slot: &mut f64,
        var_cigd_i_slot: &mut f64,
        var_cigs_i_slot: &mut f64,
        var_cox1_slot: &mut f64,
        var_cox1p_slot: &mut f64,
        var_cox2_slot: &mut f64,
        var_csi_slot: &mut f64,
        var_digc_i_slot: &mut f64,
        var_digd_i_slot: &mut f64,
        var_digs_i_slot: &mut f64,
        var_egidl_i_slot: &mut f64,
        var_egisl_i_slot: &mut f64,
        var_epsratio_slot: &mut f64,
        var_etaqm_i_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard19_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_imgtoxp_slot: &mut f64,
        var_ksativ_i_slot: &mut f64,
        var_ksativb_i_slot: &mut f64,
        var_ksubiv_i_slot: &mut f64,
        var_lovd_i_slot: &mut f64,
        var_lovs_i_slot: &mut f64,
        var_nigbacc_i_slot: &mut f64,
        var_ntox_i_slot: &mut f64,
        var_pgidl_i_slot: &mut f64,
        var_pgisl_i_slot: &mut f64,
        var_pigcd_i_slot: &mut f64,
        var_poxedge_i_slot: &mut f64,
        var_pqm_i_slot: &mut f64,
        var_qm0_i_slot: &mut f64,
        var_qmtcencv_i_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_uc_i_slot: &mut f64,
        var_up2_i_slot: &mut f64,
        var_up_i_slot: &mut f64,
        var_vbegidl_i_slot: &mut f64,
        var_vbegisl_i_slot: &mut f64,
        var_vbgidl_i_slot: &mut f64,
        var_vbgisl_i_slot: &mut f64,
        var_vsat1_i_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsatb_i_slot: &mut f64,
        var_vsatcv_i_slot: &mut f64,
        var_xrcrg1_i_slot: &mut f64,
        var_xrcrg2_i_slot: &mut f64,
    ) {
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agisl_i: f64 = *var_agisl_i_slot;
        let mut var_aigc_i: f64 = *var_aigc_i_slot;
        let mut var_aigd_i: f64 = *var_aigd_i_slot;
        let mut var_aigs_i: f64 = *var_aigs_i_slot;
        let mut var_bgidl_i: f64 = *var_bgidl_i_slot;
        let mut var_bgisl_i: f64 = *var_bgisl_i_slot;
        let mut var_bigbacc_i: f64 = *var_bigbacc_i_slot;
        let mut var_bigc_i: f64 = *var_bigc_i_slot;
        let mut var_bigd_i: f64 = *var_bigd_i_slot;
        let mut var_bigs_i: f64 = *var_bigs_i_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfs_i: f64 = *var_cfs_i_slot;
        let mut var_cigbacc_i: f64 = *var_cigbacc_i_slot;
        let mut var_cigc_i: f64 = *var_cigc_i_slot;
        let mut var_cigd_i: f64 = *var_cigd_i_slot;
        let mut var_cigs_i: f64 = *var_cigs_i_slot;
        let mut var_cox1: f64 = *var_cox1_slot;
        let mut var_cox1p: f64 = *var_cox1p_slot;
        let mut var_cox2: f64 = *var_cox2_slot;
        let mut var_csi: f64 = *var_csi_slot;
        let mut var_digc_i: f64 = *var_digc_i_slot;
        let mut var_digd_i: f64 = *var_digd_i_slot;
        let mut var_digs_i: f64 = *var_digs_i_slot;
        let mut var_egidl_i: f64 = *var_egidl_i_slot;
        let mut var_egisl_i: f64 = *var_egisl_i_slot;
        let mut var_epsratio: f64 = *var_epsratio_slot;
        let mut var_etaqm_i: f64 = *var_etaqm_i_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_imgtoxp: f64 = *var_imgtoxp_slot;
        let mut var_ksativ_i: f64 = *var_ksativ_i_slot;
        let mut var_ksativb_i: f64 = *var_ksativb_i_slot;
        let mut var_ksubiv_i: f64 = *var_ksubiv_i_slot;
        let mut var_lovd_i: f64 = *var_lovd_i_slot;
        let mut var_lovs_i: f64 = *var_lovs_i_slot;
        let mut var_nigbacc_i: f64 = *var_nigbacc_i_slot;
        let mut var_ntox_i: f64 = *var_ntox_i_slot;
        let mut var_pgidl_i: f64 = *var_pgidl_i_slot;
        let mut var_pgisl_i: f64 = *var_pgisl_i_slot;
        let mut var_pigcd_i: f64 = *var_pigcd_i_slot;
        let mut var_poxedge_i: f64 = *var_poxedge_i_slot;
        let mut var_pqm_i: f64 = *var_pqm_i_slot;
        let mut var_qm0_i: f64 = *var_qm0_i_slot;
        let mut var_qmtcencv_i: f64 = *var_qmtcencv_i_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_uc_i: f64 = *var_uc_i_slot;
        let mut var_up2_i: f64 = *var_up2_i_slot;
        let mut var_up_i: f64 = *var_up_i_slot;
        let mut var_vbegidl_i: f64 = *var_vbegidl_i_slot;
        let mut var_vbegisl_i: f64 = *var_vbegisl_i_slot;
        let mut var_vbgidl_i: f64 = *var_vbgidl_i_slot;
        let mut var_vbgisl_i: f64 = *var_vbgisl_i_slot;
        let mut var_vsat1_i: f64 = *var_vsat1_i_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsatb_i: f64 = *var_vsatb_i_slot;
        let mut var_vsatcv_i: f64 = *var_vsatcv_i_slot;
        let mut var_xrcrg1_i: f64 = *var_xrcrg1_i_slot;
        let mut var_xrcrg2_i: f64 = *var_xrcrg2_i_slot;

        let assign1500_e2615: f64 = (var_inv_l * p.p724);
        let assign1500_e2616: f64 = (p.p223 + assign1500_e2615);
        let assign1500_e2619: f64 = (var_inv_w * p.p725);
        let assign1500_e2620: f64 = (assign1500_e2616 + assign1500_e2619);
        let assign1500_e2623: f64 = (var_inv_wl * p.p726);
        let assign1500_e2624: f64 = (assign1500_e2620 + assign1500_e2623);
        var_bigbacc_i = assign1500_e2624;

        let assign1510_e2628: f64 = (var_inv_l * p.p727);
        let assign1510_e2629: f64 = (p.p224 + assign1510_e2628);
        let assign1510_e2632: f64 = (var_inv_w * p.p728);
        let assign1510_e2633: f64 = (assign1510_e2629 + assign1510_e2632);
        let assign1510_e2636: f64 = (var_inv_wl * p.p729);
        let assign1510_e2637: f64 = (assign1510_e2633 + assign1510_e2636);
        var_cigbacc_i = assign1510_e2637;

        let assign1520_e2641: f64 = (var_inv_l * p.p730);
        let assign1520_e2642: f64 = (p.p225 + assign1520_e2641);
        let assign1520_e2645: f64 = (var_inv_w * p.p731);
        let assign1520_e2646: f64 = (assign1520_e2642 + assign1520_e2645);
        let assign1520_e2649: f64 = (var_inv_wl * p.p732);
        let assign1520_e2650: f64 = (assign1520_e2646 + assign1520_e2649);
        var_nigbacc_i = assign1520_e2650;

        let assign1530_e2654: f64 = (p.p586 * var_inv_l);
        let assign1530_e2655: f64 = (p.p226 + assign1530_e2654);
        let assign1530_e2658: f64 = (p.p587 * var_inv_w);
        let assign1530_e2659: f64 = (assign1530_e2655 + assign1530_e2658);
        let assign1530_e2662: f64 = (p.p588 * var_inv_wl);
        let assign1530_e2663: f64 = (assign1530_e2659 + assign1530_e2662);
        var_aigc_i = assign1530_e2663;

        let assign1540_e2667: f64 = (p.p589 * var_inv_l);
        let assign1540_e2668: f64 = (p.p227 + assign1540_e2667);
        let assign1540_e2671: f64 = (p.p590 * var_inv_w);
        let assign1540_e2672: f64 = (assign1540_e2668 + assign1540_e2671);
        let assign1540_e2675: f64 = (p.p591 * var_inv_wl);
        let assign1540_e2676: f64 = (assign1540_e2672 + assign1540_e2675);
        var_bigc_i = assign1540_e2676;

        let assign1550_e2680: f64 = (p.p592 * var_inv_l);
        let assign1550_e2681: f64 = (p.p228 + assign1550_e2680);
        let assign1550_e2684: f64 = (p.p593 * var_inv_w);
        let assign1550_e2685: f64 = (assign1550_e2681 + assign1550_e2684);
        let assign1550_e2688: f64 = (p.p594 * var_inv_wl);
        let assign1550_e2689: f64 = (assign1550_e2685 + assign1550_e2688);
        var_cigc_i = assign1550_e2689;

        let assign1560_e2693: f64 = (p.p595 * var_inv_l);
        let assign1560_e2694: f64 = (p.p230 + assign1560_e2693);
        let assign1560_e2697: f64 = (p.p596 * var_inv_w);
        let assign1560_e2698: f64 = (assign1560_e2694 + assign1560_e2697);
        let assign1560_e2701: f64 = (p.p597 * var_inv_wl);
        let assign1560_e2702: f64 = (assign1560_e2698 + assign1560_e2701);
        var_digc_i = assign1560_e2702;

        let assign1570_e2706: f64 = (p.p598 * var_inv_l);
        let assign1570_e2707: f64 = (p.p229 + assign1570_e2706);
        let assign1570_e2710: f64 = (p.p599 * var_inv_w);
        let assign1570_e2711: f64 = (assign1570_e2707 + assign1570_e2710);
        let assign1570_e2714: f64 = (p.p600 * var_inv_wl);
        let assign1570_e2715: f64 = (assign1570_e2711 + assign1570_e2714);
        var_pigcd_i = assign1570_e2715;

        let assign1580_e2719: f64 = (p.p610 * var_inv_l);
        let assign1580_e2720: f64 = (p.p247 + assign1580_e2719);
        let assign1580_e2723: f64 = (p.p611 * var_inv_w);
        let assign1580_e2724: f64 = (assign1580_e2720 + assign1580_e2723);
        let assign1580_e2727: f64 = (p.p612 * var_inv_wl);
        let assign1580_e2728: f64 = (assign1580_e2724 + assign1580_e2727);
        var_pgidl_i = assign1580_e2728;

        let assign1590_e2732: f64 = (p.p619 * var_inv_l);
        let assign1590_e2733: f64 = (p.p250 + assign1590_e2732);
        let assign1590_e2736: f64 = (p.p620 * var_inv_w);
        let assign1590_e2737: f64 = (assign1590_e2733 + assign1590_e2736);
        let assign1590_e2740: f64 = (p.p621 * var_inv_wl);
        let assign1590_e2741: f64 = (assign1590_e2737 + assign1590_e2740);
        var_agisl_i = assign1590_e2741;

        let assign1600_e2745: f64 = (p.p622 * var_inv_l);
        let assign1600_e2746: f64 = (p.p251 + assign1600_e2745);
        let assign1600_e2749: f64 = (p.p623 * var_inv_w);
        let assign1600_e2750: f64 = (assign1600_e2746 + assign1600_e2749);
        let assign1600_e2753: f64 = (p.p624 * var_inv_wl);
        let assign1600_e2754: f64 = (assign1600_e2750 + assign1600_e2753);
        var_bgisl_i = assign1600_e2754;

        let assign1610_e2758: f64 = (p.p625 * var_inv_l);
        let assign1610_e2759: f64 = (p.p252 + assign1610_e2758);
        let assign1610_e2762: f64 = (p.p626 * var_inv_w);
        let assign1610_e2763: f64 = (assign1610_e2759 + assign1610_e2762);
        let assign1610_e2766: f64 = (p.p627 * var_inv_wl);
        let assign1610_e2767: f64 = (assign1610_e2763 + assign1610_e2766);
        var_egisl_i = assign1610_e2767;

        let assign1620_e2771: f64 = (p.p628 * var_inv_l);
        let assign1620_e2772: f64 = (p.p253 + assign1620_e2771);
        let assign1620_e2775: f64 = (p.p629 * var_inv_w);
        let assign1620_e2776: f64 = (assign1620_e2772 + assign1620_e2775);
        let assign1620_e2779: f64 = (p.p630 * var_inv_wl);
        let assign1620_e2780: f64 = (assign1620_e2776 + assign1620_e2779);
        var_pgisl_i = assign1620_e2780;

        let assign1630_e2784: f64 = (p.p601 * var_inv_l);
        let assign1630_e2785: f64 = (p.p244 + assign1630_e2784);
        let assign1630_e2788: f64 = (p.p602 * var_inv_w);
        let assign1630_e2789: f64 = (assign1630_e2785 + assign1630_e2788);
        let assign1630_e2792: f64 = (p.p603 * var_inv_wl);
        let assign1630_e2793: f64 = (assign1630_e2789 + assign1630_e2792);
        var_agidl_i = assign1630_e2793;

        let assign1640_e2797: f64 = (p.p604 * var_inv_l);
        let assign1640_e2798: f64 = (p.p245 + assign1640_e2797);
        let assign1640_e2801: f64 = (p.p605 * var_inv_w);
        let assign1640_e2802: f64 = (assign1640_e2798 + assign1640_e2801);
        let assign1640_e2805: f64 = (p.p606 * var_inv_wl);
        let assign1640_e2806: f64 = (assign1640_e2802 + assign1640_e2805);
        var_bgidl_i = assign1640_e2806;

        let assign1650_e2810: f64 = (p.p607 * var_inv_l);
        let assign1650_e2811: f64 = (p.p246 + assign1650_e2810);
        let assign1650_e2814: f64 = (p.p608 * var_inv_w);
        let assign1650_e2815: f64 = (assign1650_e2811 + assign1650_e2814);
        let assign1650_e2818: f64 = (p.p609 * var_inv_wl);
        let assign1650_e2819: f64 = (assign1650_e2815 + assign1650_e2818);
        var_egidl_i = assign1650_e2819;

        let assign1660_e2823: f64 = (p.p613 * var_inv_l);
        let assign1660_e2824: f64 = (p.p248 + assign1660_e2823);
        let assign1660_e2827: f64 = (p.p614 * var_inv_w);
        let assign1660_e2828: f64 = (assign1660_e2824 + assign1660_e2827);
        let assign1660_e2831: f64 = (p.p615 * var_inv_wl);
        let assign1660_e2832: f64 = (assign1660_e2828 + assign1660_e2831);
        var_vbgidl_i = assign1660_e2832;

        let assign1670_e2836: f64 = (p.p631 * var_inv_l);
        let assign1670_e2837: f64 = (p.p254 + assign1670_e2836);
        let assign1670_e2840: f64 = (p.p632 * var_inv_w);
        let assign1670_e2841: f64 = (assign1670_e2837 + assign1670_e2840);
        let assign1670_e2844: f64 = (p.p633 * var_inv_wl);
        let assign1670_e2845: f64 = (assign1670_e2841 + assign1670_e2844);
        var_vbgisl_i = assign1670_e2845;

        let assign1680_e2849: f64 = (p.p616 * var_inv_l);
        let assign1680_e2850: f64 = (p.p249 + assign1680_e2849);
        let assign1680_e2853: f64 = (p.p617 * var_inv_w);
        let assign1680_e2854: f64 = (assign1680_e2850 + assign1680_e2853);
        let assign1680_e2857: f64 = (p.p618 * var_inv_wl);
        let assign1680_e2858: f64 = (assign1680_e2854 + assign1680_e2857);
        var_vbegidl_i = assign1680_e2858;

        let assign1690_e2862: f64 = (p.p634 * var_inv_l);
        let assign1690_e2863: f64 = (p.p255 + assign1690_e2862);
        let assign1690_e2866: f64 = (p.p635 * var_inv_w);
        let assign1690_e2867: f64 = (assign1690_e2863 + assign1690_e2866);
        let assign1690_e2870: f64 = (p.p636 * var_inv_wl);
        let assign1690_e2871: f64 = (assign1690_e2867 + assign1690_e2870);
        var_vbegisl_i = assign1690_e2871;

        let assign1700_e2875: f64 = (p.p637 * var_inv_l);
        let assign1700_e2876: f64 = (p.p231 + assign1700_e2875);
        let assign1700_e2879: f64 = (p.p638 * var_inv_w);
        let assign1700_e2880: f64 = (assign1700_e2876 + assign1700_e2879);
        let assign1700_e2883: f64 = (p.p639 * var_inv_wl);
        let assign1700_e2884: f64 = (assign1700_e2880 + assign1700_e2883);
        var_aigs_i = assign1700_e2884;

        let assign1710_e2888: f64 = (p.p643 * var_inv_l);
        let assign1710_e2889: f64 = (p.p232 + assign1710_e2888);
        let assign1710_e2892: f64 = (p.p644 * var_inv_w);
        let assign1710_e2893: f64 = (assign1710_e2889 + assign1710_e2892);
        let assign1710_e2896: f64 = (p.p645 * var_inv_wl);
        let assign1710_e2897: f64 = (assign1710_e2893 + assign1710_e2896);
        var_bigs_i = assign1710_e2897;

        let assign1720_e2901: f64 = (p.p649 * var_inv_l);
        let assign1720_e2902: f64 = (p.p233 + assign1720_e2901);
        let assign1720_e2905: f64 = (p.p650 * var_inv_w);
        let assign1720_e2906: f64 = (assign1720_e2902 + assign1720_e2905);
        let assign1720_e2909: f64 = (p.p651 * var_inv_wl);
        let assign1720_e2910: f64 = (assign1720_e2906 + assign1720_e2909);
        var_cigs_i = assign1720_e2910;

        let assign1730_e2914: f64 = (p.p655 * var_inv_l);
        let assign1730_e2915: f64 = (p.p242 + assign1730_e2914);
        let assign1730_e2918: f64 = (p.p656 * var_inv_w);
        let assign1730_e2919: f64 = (assign1730_e2915 + assign1730_e2918);
        let assign1730_e2922: f64 = (p.p657 * var_inv_wl);
        let assign1730_e2923: f64 = (assign1730_e2919 + assign1730_e2922);
        var_digs_i = assign1730_e2923;

        let assign1740_e2927: f64 = (p.p640 * var_inv_l);
        let assign1740_e2928: f64 = (p.p236 + assign1740_e2927);
        let assign1740_e2931: f64 = (p.p641 * var_inv_w);
        let assign1740_e2932: f64 = (assign1740_e2928 + assign1740_e2931);
        let assign1740_e2935: f64 = (p.p642 * var_inv_wl);
        let assign1740_e2936: f64 = (assign1740_e2932 + assign1740_e2935);
        var_aigd_i = assign1740_e2936;

        let assign1750_e2940: f64 = (p.p646 * var_inv_l);
        let assign1750_e2941: f64 = (p.p237 + assign1750_e2940);
        let assign1750_e2944: f64 = (p.p647 * var_inv_w);
        let assign1750_e2945: f64 = (assign1750_e2941 + assign1750_e2944);
        let assign1750_e2948: f64 = (p.p648 * var_inv_wl);
        let assign1750_e2949: f64 = (assign1750_e2945 + assign1750_e2948);
        var_bigd_i = assign1750_e2949;

        let assign1760_e2953: f64 = (p.p652 * var_inv_l);
        let assign1760_e2954: f64 = (p.p238 + assign1760_e2953);
        let assign1760_e2957: f64 = (p.p653 * var_inv_w);
        let assign1760_e2958: f64 = (assign1760_e2954 + assign1760_e2957);
        let assign1760_e2961: f64 = (p.p654 * var_inv_wl);
        let assign1760_e2962: f64 = (assign1760_e2958 + assign1760_e2961);
        var_cigd_i = assign1760_e2962;

        let assign1770_e2966: f64 = (p.p658 * var_inv_l);
        let assign1770_e2967: f64 = (p.p243 + assign1770_e2966);
        let assign1770_e2970: f64 = (p.p659 * var_inv_w);
        let assign1770_e2971: f64 = (assign1770_e2967 + assign1770_e2970);
        let assign1770_e2974: f64 = (p.p660 * var_inv_wl);
        let assign1770_e2975: f64 = (assign1770_e2971 + assign1770_e2974);
        var_digd_i = assign1770_e2975;

        let assign1780_e2979: f64 = (p.p661 * var_inv_l);
        let assign1780_e2980: f64 = (p.p240 + assign1780_e2979);
        let assign1780_e2983: f64 = (p.p662 * var_inv_w);
        let assign1780_e2984: f64 = (assign1780_e2980 + assign1780_e2983);
        let assign1780_e2987: f64 = (p.p663 * var_inv_wl);
        let assign1780_e2988: f64 = (assign1780_e2984 + assign1780_e2987);
        var_ntox_i = assign1780_e2988;

        let assign1790_e2992: f64 = (p.p664 * var_inv_l);
        let assign1790_e2993: f64 = (p.p241 + assign1790_e2992);
        let assign1790_e2996: f64 = (p.p665 * var_inv_w);
        let assign1790_e2997: f64 = (assign1790_e2993 + assign1790_e2996);
        let assign1790_e3000: f64 = (p.p666 * var_inv_wl);
        let assign1790_e3001: f64 = (assign1790_e2997 + assign1790_e3000);
        var_poxedge_i = assign1790_e3001;

        let assign1800_e3005: f64 = (p.p667 * var_inv_l);
        let assign1800_e3006: f64 = (p.p259 + assign1800_e3005);
        let assign1800_e3009: f64 = (p.p668 * var_inv_w);
        let assign1800_e3010: f64 = (assign1800_e3006 + assign1800_e3009);
        let assign1800_e3013: f64 = (p.p669 * var_inv_wl);
        let assign1800_e3014: f64 = (assign1800_e3010 + assign1800_e3013);
        var_lovs_i = assign1800_e3014;

        let assign1810_e3018: f64 = (p.p670 * var_inv_l);
        let assign1810_e3019: f64 = (p.p260 + assign1810_e3018);
        let assign1810_e3022: f64 = (p.p671 * var_inv_w);
        let assign1810_e3023: f64 = (assign1810_e3019 + assign1810_e3022);
        let assign1810_e3026: f64 = (p.p672 * var_inv_wl);
        let assign1810_e3027: f64 = (assign1810_e3023 + assign1810_e3026);
        var_lovd_i = assign1810_e3027;

        let assign1820_e3031: f64 = (p.p673 * var_inv_l);
        let assign1820_e3032: f64 = (p.p261 + assign1820_e3031);
        let assign1820_e3035: f64 = (p.p674 * var_inv_w);
        let assign1820_e3036: f64 = (assign1820_e3032 + assign1820_e3035);
        let assign1820_e3039: f64 = (p.p675 * var_inv_wl);
        let assign1820_e3040: f64 = (assign1820_e3036 + assign1820_e3039);
        var_cfs_i = assign1820_e3040;

        let assign1830_e3044: f64 = (p.p676 * var_inv_l);
        let assign1830_e3045: f64 = (p.p262 + assign1830_e3044);
        let assign1830_e3048: f64 = (p.p677 * var_inv_w);
        let assign1830_e3049: f64 = (assign1830_e3045 + assign1830_e3048);
        let assign1830_e3052: f64 = (p.p678 * var_inv_wl);
        let assign1830_e3053: f64 = (assign1830_e3049 + assign1830_e3052);
        var_cfd_i = assign1830_e3053;

        let assign1840_e3057: f64 = (p.p679 * var_inv_l);
        let assign1840_e3058: f64 = (p.p100 + assign1840_e3057);
        let assign1840_e3061: f64 = (p.p680 * var_inv_w);
        let assign1840_e3062: f64 = (assign1840_e3058 + assign1840_e3061);
        let assign1840_e3065: f64 = (p.p681 * var_inv_wl);
        let assign1840_e3066: f64 = (assign1840_e3062 + assign1840_e3065);
        var_vsat_i = assign1840_e3066;

        let assign1850_e3070: f64 = (p.p682 * var_inv_l);
        let assign1850_e3071: f64 = (p.p129 + assign1850_e3070);
        let assign1850_e3074: f64 = (p.p683 * var_inv_w);
        let assign1850_e3075: f64 = (assign1850_e3071 + assign1850_e3074);
        let assign1850_e3078: f64 = (p.p684 * var_inv_wl);
        let assign1850_e3079: f64 = (assign1850_e3075 + assign1850_e3078);
        var_vsatb_i = assign1850_e3079;

        let assign1860_e3083: f64 = (p.p685 * var_inv_l);
        let assign1860_e3084: f64 = (p.p103 + assign1860_e3083);
        let assign1860_e3087: f64 = (p.p686 * var_inv_w);
        let assign1860_e3088: f64 = (assign1860_e3084 + assign1860_e3087);
        let assign1860_e3091: f64 = (p.p687 * var_inv_wl);
        let assign1860_e3092: f64 = (assign1860_e3088 + assign1860_e3091);
        var_vsat1_i = assign1860_e3092;

        let assign1870_e3096: f64 = (p.p688 * var_inv_l);
        let assign1870_e3097: f64 = (p.p106 + assign1870_e3096);
        let assign1870_e3100: f64 = (p.p689 * var_inv_w);
        let assign1870_e3101: f64 = (assign1870_e3097 + assign1870_e3100);
        let assign1870_e3104: f64 = (p.p690 * var_inv_wl);
        let assign1870_e3105: f64 = (assign1870_e3101 + assign1870_e3104);
        var_vsatcv_i = assign1870_e3105;

        let assign1880_e3109: f64 = (p.p691 * var_inv_l);
        let assign1880_e3110: f64 = (p.p110 + assign1880_e3109);
        let assign1880_e3113: f64 = (p.p692 * var_inv_w);
        let assign1880_e3114: f64 = (assign1880_e3110 + assign1880_e3113);
        let assign1880_e3117: f64 = (p.p693 * var_inv_wl);
        let assign1880_e3118: f64 = (assign1880_e3114 + assign1880_e3117);
        var_ksativ_i = assign1880_e3118;

        let assign1890_e3122: f64 = (p.p694 * var_inv_l);
        let assign1890_e3123: f64 = (p.p111 + assign1890_e3122);
        let assign1890_e3126: f64 = (p.p695 * var_inv_w);
        let assign1890_e3127: f64 = (assign1890_e3123 + assign1890_e3126);
        let assign1890_e3130: f64 = (p.p696 * var_inv_wl);
        let assign1890_e3131: f64 = (assign1890_e3127 + assign1890_e3130);
        var_ksubiv_i = assign1890_e3131;

        let assign1900_e3135: f64 = (p.p697 * var_inv_l);
        let assign1900_e3136: f64 = (p.p112 + assign1900_e3135);
        let assign1900_e3139: f64 = (p.p698 * var_inv_w);
        let assign1900_e3140: f64 = (assign1900_e3136 + assign1900_e3139);
        let assign1900_e3143: f64 = (p.p699 * var_inv_wl);
        let assign1900_e3144: f64 = (assign1900_e3140 + assign1900_e3143);
        var_ksativb_i = assign1900_e3144;

        let assign1910_e3148: f64 = (p.p700 * var_inv_l);
        let assign1910_e3149: f64 = (p.p137 + assign1910_e3148);
        let assign1910_e3152: f64 = (p.p701 * var_inv_w);
        let assign1910_e3153: f64 = (assign1910_e3149 + assign1910_e3152);
        let assign1910_e3156: f64 = (p.p702 * var_inv_wl);
        let assign1910_e3157: f64 = (assign1910_e3153 + assign1910_e3156);
        var_up_i = assign1910_e3157;

        let assign1920_e3161: f64 = (p.p703 * var_inv_l);
        let assign1920_e3162: f64 = (p.p187 + assign1920_e3161);
        let assign1920_e3165: f64 = (p.p704 * var_inv_w);
        let assign1920_e3166: f64 = (assign1920_e3162 + assign1920_e3165);
        let assign1920_e3169: f64 = (p.p705 * var_inv_wl);
        let assign1920_e3170: f64 = (assign1920_e3166 + assign1920_e3169);
        var_up2_i = assign1920_e3170;

        let assign1930_e3174: f64 = (p.p739 * var_inv_l);
        let assign1930_e3175: f64 = (p.p95 + assign1930_e3174);
        let assign1930_e3178: f64 = (p.p740 * var_inv_w);
        let assign1930_e3179: f64 = (assign1930_e3175 + assign1930_e3178);
        let assign1930_e3182: f64 = (p.p741 * var_inv_wl);
        let assign1930_e3183: f64 = (assign1930_e3179 + assign1930_e3182);
        var_qmtcencv_i = assign1930_e3183;

        let assign1940_e3187: f64 = (p.p742 * var_inv_l);
        let assign1940_e3188: f64 = (p.p96 + assign1940_e3187);
        let assign1940_e3191: f64 = (p.p743 * var_inv_w);
        let assign1940_e3192: f64 = (assign1940_e3188 + assign1940_e3191);
        let assign1940_e3195: f64 = (p.p744 * var_inv_wl);
        let assign1940_e3196: f64 = (assign1940_e3192 + assign1940_e3195);
        var_etaqm_i = assign1940_e3196;

        let assign1950_e3200: f64 = (p.p745 * var_inv_l);
        let assign1950_e3201: f64 = (p.p97 + assign1950_e3200);
        let assign1950_e3204: f64 = (p.p746 * var_inv_w);
        let assign1950_e3205: f64 = (assign1950_e3201 + assign1950_e3204);
        let assign1950_e3208: f64 = (p.p747 * var_inv_wl);
        let assign1950_e3209: f64 = (assign1950_e3205 + assign1950_e3208);
        var_qm0_i = assign1950_e3209;

        let assign1960_e3213: f64 = (p.p748 * var_inv_l);
        let assign1960_e3214: f64 = (p.p98 + assign1960_e3213);
        let assign1960_e3217: f64 = (p.p749 * var_inv_w);
        let assign1960_e3218: f64 = (assign1960_e3214 + assign1960_e3217);
        let assign1960_e3221: f64 = (p.p750 * var_inv_wl);
        let assign1960_e3222: f64 = (assign1960_e3218 + assign1960_e3221);
        var_pqm_i = assign1960_e3222;

        let assign1970_e3229: f64 = if ((p.p20 == 1.0) && (p.p317 != 0.0)) { 1.0 } else { 0.0 };
        var_guard18 = assign1970_e3229;

        let (assign1980_e3245,) = {
    if (var_guard18 != 0.0) {
        let assign1980_e3234: f64 = (p.p733 * var_inv_l);
        let assign1980_e3235: f64 = (p.p317 + assign1980_e3234);
        let assign1980_e3238: f64 = (p.p734 * var_inv_w);
        let assign1980_e3239: f64 = (assign1980_e3235 + assign1980_e3238);
        let assign1980_e3242: f64 = (p.p735 * var_inv_wl);
        let assign1980_e3243: f64 = (assign1980_e3239 + assign1980_e3242);
        (assign1980_e3243,)
    } else {
        (var_xrcrg1_i,)
    }
};
        var_xrcrg1_i = assign1980_e3245;

        let (assign1990_e3261,) = {
    if (var_guard18 != 0.0) {
        let assign1990_e3250: f64 = (p.p736 * var_inv_l);
        let assign1990_e3251: f64 = (p.p318 + assign1990_e3250);
        let assign1990_e3254: f64 = (p.p737 * var_inv_w);
        let assign1990_e3255: f64 = (assign1990_e3251 + assign1990_e3254);
        let assign1990_e3258: f64 = (p.p738 * var_inv_wl);
        let assign1990_e3259: f64 = (assign1990_e3255 + assign1990_e3258);
        (assign1990_e3259,)
    } else {
        (var_xrcrg2_i,)
    }
};
        var_xrcrg2_i = assign1990_e3261;

        let (assign2000_e3266,) = {
    if (var_guard18 == 0.0) {
        (0.0,)
    } else {
        (var_xrcrg1_i,)
    }
};
        var_xrcrg1_i = assign2000_e3266;

        let (assign2010_e3271,) = {
    if (var_guard18 == 0.0) {
        (0.0,)
    } else {
        (var_xrcrg2_i,)
    }
};
        var_xrcrg2_i = assign2010_e3271;

        let assign2020_e3274: f64 = (3.9 * 8.85418e-12);
        let assign2020_e3276: f64 = (assign2020_e3274 / p.p45);
        var_cox1 = assign2020_e3276;

        let assign2030_e3279: f64 = (3.9 * 8.85418e-12);
        let assign2030_e3281: f64 = (assign2030_e3279 / p.p47);
        var_cox1p = assign2030_e3281;

        let assign2040_e3284: f64 = (3.9 * 8.85418e-12);
        let assign2040_e3286: f64 = (assign2040_e3284 / p.p46);
        var_cox2 = assign2040_e3286;

        let assign2050_e3289: f64 = (var_epssi / p.p49);
        var_csi = assign2050_e3289;

        let assign2060_e3292: f64 = (p.p59 / 3.9);
        var_epsratio = assign2060_e3292;

        let assign2070_e3295: f64 = if (!param_given[47]) { 1.0 } else { 0.0 };
        var_guard19 = assign2070_e3295;

        let (assign2080_e3305,) = {
    if (var_guard19 != 0.0) {
        let assign2080_e3299: f64 = (p.p45 * p.p60);
        let assign2080_e3301: f64 = (assign2080_e3299 / 3.9);
        let assign2080_e3303: f64 = (assign2080_e3301 - p.p48);
        (assign2080_e3303,)
    } else {
        (var_imgtoxp,)
    }
};
        var_imgtoxp = assign2080_e3305;

        let (assign2090_e3310,) = {
    if (var_guard19 == 0.0) {
        (p.p47,)
    } else {
        (var_imgtoxp,)
    }
};
        var_imgtoxp = assign2090_e3310;

        let assign2100_e3313: f64 = if p.p138 > 0.0 { 1.0 } else { 0.0 };
        var_guard20 = assign2100_e3313;

        let (assign2110_e3326,) = {
    if (var_guard20 != 0.0) {
        let assign2110_e3320: f64 = (-p.p138);
        let assign2110_e3321: f64 = (var_leff).powf(assign2110_e3320);
        let assign2110_e3322: f64 = (var_up_i * assign2110_e3321);
        let assign2110_e3323: f64 = (1.0 - assign2110_e3322);
        let assign2110_e3324: f64 = (var_u0_i * assign2110_e3323);
        (assign2110_e3324,)
    } else {
        (var_u0_i,)
    }
};
        var_u0_i = assign2110_e3326;

        let (assign2120_e3335,) = {
    if (var_guard20 == 0.0) {
        let assign2120_e3332: f64 = (1.0 - var_up_i);
        let assign2120_e3333: f64 = (var_u0_i * assign2120_e3332);
        (assign2120_e3333,)
    } else {
        (var_u0_i,)
    }
};
        var_u0_i = assign2120_e3335;

        let assign2130_e3339: f64 = (-var_leff);
        let assign2130_e3341: f64 = (assign2130_e3339 / p.p141);
        let assign2130_e3342: f64 = { let limited_exp_arg = assign2130_e3341; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2130_e3343: f64 = (p.p140 * assign2130_e3342);
        let assign2130_e3344: f64 = (var_ua_i + assign2130_e3343);
        var_ua_i = assign2130_e3344;

        let assign2140_e3348: f64 = (-var_leff);
        let assign2140_e3350: f64 = (assign2140_e3348 / p.p147);
        let assign2140_e3351: f64 = { let limited_exp_arg = assign2140_e3350; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2140_e3352: f64 = (p.p146 * assign2140_e3351);
        let assign2140_e3353: f64 = (var_uc_i + assign2140_e3352);
        var_uc_i = assign2140_e3353;

        *var_agidl_i_slot = var_agidl_i;
        *var_agisl_i_slot = var_agisl_i;
        *var_aigc_i_slot = var_aigc_i;
        *var_aigd_i_slot = var_aigd_i;
        *var_aigs_i_slot = var_aigs_i;
        *var_bgidl_i_slot = var_bgidl_i;
        *var_bgisl_i_slot = var_bgisl_i;
        *var_bigbacc_i_slot = var_bigbacc_i;
        *var_bigc_i_slot = var_bigc_i;
        *var_bigd_i_slot = var_bigd_i;
        *var_bigs_i_slot = var_bigs_i;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfs_i_slot = var_cfs_i;
        *var_cigbacc_i_slot = var_cigbacc_i;
        *var_cigc_i_slot = var_cigc_i;
        *var_cigd_i_slot = var_cigd_i;
        *var_cigs_i_slot = var_cigs_i;
        *var_cox1_slot = var_cox1;
        *var_cox1p_slot = var_cox1p;
        *var_cox2_slot = var_cox2;
        *var_csi_slot = var_csi;
        *var_digc_i_slot = var_digc_i;
        *var_digd_i_slot = var_digd_i;
        *var_digs_i_slot = var_digs_i;
        *var_egidl_i_slot = var_egidl_i;
        *var_egisl_i_slot = var_egisl_i;
        *var_epsratio_slot = var_epsratio;
        *var_etaqm_i_slot = var_etaqm_i;
        *var_guard18_slot = var_guard18;
        *var_guard19_slot = var_guard19;
        *var_guard20_slot = var_guard20;
        *var_imgtoxp_slot = var_imgtoxp;
        *var_ksativ_i_slot = var_ksativ_i;
        *var_ksativb_i_slot = var_ksativb_i;
        *var_ksubiv_i_slot = var_ksubiv_i;
        *var_lovd_i_slot = var_lovd_i;
        *var_lovs_i_slot = var_lovs_i;
        *var_nigbacc_i_slot = var_nigbacc_i;
        *var_ntox_i_slot = var_ntox_i;
        *var_pgidl_i_slot = var_pgidl_i;
        *var_pgisl_i_slot = var_pgisl_i;
        *var_pigcd_i_slot = var_pigcd_i;
        *var_poxedge_i_slot = var_poxedge_i;
        *var_pqm_i_slot = var_pqm_i;
        *var_qm0_i_slot = var_qm0_i;
        *var_qmtcencv_i_slot = var_qmtcencv_i;
        *var_u0_i_slot = var_u0_i;
        *var_ua_i_slot = var_ua_i;
        *var_uc_i_slot = var_uc_i;
        *var_up2_i_slot = var_up2_i;
        *var_up_i_slot = var_up_i;
        *var_vbegidl_i_slot = var_vbegidl_i;
        *var_vbegisl_i_slot = var_vbegisl_i;
        *var_vbgidl_i_slot = var_vbgidl_i;
        *var_vbgisl_i_slot = var_vbgisl_i;
        *var_vsat1_i_slot = var_vsat1_i;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsatb_i_slot = var_vsatb_i;
        *var_vsatcv_i_slot = var_vsatcv_i;
        *var_xrcrg1_i_slot = var_xrcrg1_i;
        *var_xrcrg2_i_slot = var_xrcrg2_i;
    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        var_cox2: f64,
        var_epsratio: f64,
        var_etamob2_i: f64,
        var_etamob_i: f64,
        var_leff: f64,
        var_lpe0_i: f64,
        var_up2_i: f64,
        var_cdbox_slot: &mut f64,
        var_cdbox_dn3_slot: &mut f64,
        var_cdbox_dn4_slot: &mut f64,
        var_cdbox_dn5_slot: &mut f64,
        var_cdbox_dn6_slot: &mut f64,
        var_cdbox_dn7_slot: &mut f64,
        var_cdbox_dn8_slot: &mut f64,
        var_csbox_slot: &mut f64,
        var_csbox_dn3_slot: &mut f64,
        var_csbox_dn4_slot: &mut f64,
        var_csbox_dn5_slot: &mut f64,
        var_csbox_dn6_slot: &mut f64,
        var_csbox_dn7_slot: &mut f64,
        var_csbox_dn8_slot: &mut f64,
        var_dvtp0_i_slot: &mut f64,
        var_dvtp1_i_slot: &mut f64,
        var_eta_mu_slot: &mut f64,
        var_eta_mu2_slot: &mut f64,
        var_eta_mu_cv_slot: &mut f64,
        var_eu2_i_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eub2_i_slot: &mut f64,
        var_eub_i_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard23_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_inv_mexp_slot: &mut f64,
        var_mexp_i_slot: &mut f64,
        var_pclm_i_slot: &mut f64,
        var_prwg_i_slot: &mut f64,
        var_ptwg_i_slot: &mut f64,
        var_ptwgb2_i_slot: &mut f64,
        var_ptwgb_i_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdswmin_i_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdwmin_i_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rswmin_i_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_teff_slot: &mut f64,
        var_theta_rsce_slot: &mut f64,
        var_u02_i_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_ua2_i_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_uc2_i_slot: &mut f64,
        var_ucs_i_slot: &mut f64,
        var_ud2_i_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_udb2_i_slot: &mut f64,
        var_udb_i_slot: &mut f64,
        var_vsat1_i_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsatb_i_slot: &mut f64,
        var_vsatcv_i_slot: &mut f64,
    ) {
        let mut var_cdbox: f64 = *var_cdbox_slot;
        let mut var_cdbox_dn3: f64 = *var_cdbox_dn3_slot;
        let mut var_cdbox_dn4: f64 = *var_cdbox_dn4_slot;
        let mut var_cdbox_dn5: f64 = *var_cdbox_dn5_slot;
        let mut var_cdbox_dn6: f64 = *var_cdbox_dn6_slot;
        let mut var_cdbox_dn7: f64 = *var_cdbox_dn7_slot;
        let mut var_cdbox_dn8: f64 = *var_cdbox_dn8_slot;
        let mut var_csbox: f64 = *var_csbox_slot;
        let mut var_csbox_dn3: f64 = *var_csbox_dn3_slot;
        let mut var_csbox_dn4: f64 = *var_csbox_dn4_slot;
        let mut var_csbox_dn5: f64 = *var_csbox_dn5_slot;
        let mut var_csbox_dn6: f64 = *var_csbox_dn6_slot;
        let mut var_csbox_dn7: f64 = *var_csbox_dn7_slot;
        let mut var_csbox_dn8: f64 = *var_csbox_dn8_slot;
        let mut var_dvtp0_i: f64 = *var_dvtp0_i_slot;
        let mut var_dvtp1_i: f64 = *var_dvtp1_i_slot;
        let mut var_eta_mu: f64 = *var_eta_mu_slot;
        let mut var_eta_mu2: f64 = *var_eta_mu2_slot;
        let mut var_eta_mu_cv: f64 = *var_eta_mu_cv_slot;
        let mut var_eu2_i: f64 = *var_eu2_i_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eub2_i: f64 = *var_eub2_i_slot;
        let mut var_eub_i: f64 = *var_eub_i_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard23: f64 = *var_guard23_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_inv_mexp: f64 = *var_inv_mexp_slot;
        let mut var_mexp_i: f64 = *var_mexp_i_slot;
        let mut var_pclm_i: f64 = *var_pclm_i_slot;
        let mut var_prwg_i: f64 = *var_prwg_i_slot;
        let mut var_ptwg_i: f64 = *var_ptwg_i_slot;
        let mut var_ptwgb2_i: f64 = *var_ptwgb2_i_slot;
        let mut var_ptwgb_i: f64 = *var_ptwgb_i_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdswmin_i: f64 = *var_rdswmin_i_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdwmin_i: f64 = *var_rdwmin_i_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rswmin_i: f64 = *var_rswmin_i_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_teff: f64 = *var_teff_slot;
        let mut var_theta_rsce: f64 = *var_theta_rsce_slot;
        let mut var_u02_i: f64 = *var_u02_i_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_ua2_i: f64 = *var_ua2_i_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_uc2_i: f64 = *var_uc2_i_slot;
        let mut var_ucs_i: f64 = *var_ucs_i_slot;
        let mut var_ud2_i: f64 = *var_ud2_i_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_udb2_i: f64 = *var_udb2_i_slot;
        let mut var_udb_i: f64 = *var_udb_i_slot;
        let mut var_vsat1_i: f64 = *var_vsat1_i_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsatb_i: f64 = *var_vsatb_i_slot;
        let mut var_vsatcv_i: f64 = *var_vsatcv_i_slot;

        let assign2150_e3357: f64 = (-var_leff);
        let assign2150_e3359: f64 = (assign2150_e3357 / p.p153);
        let assign2150_e3360: f64 = { let limited_exp_arg = assign2150_e3359; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2150_e3361: f64 = (p.p152 * assign2150_e3360);
        let assign2150_e3362: f64 = (p.p151 + assign2150_e3361);
        var_udb_i = assign2150_e3362;

        let assign2160_e3366: f64 = (-var_leff);
        let assign2160_e3368: f64 = (assign2160_e3366 / p.p150);
        let assign2160_e3369: f64 = { let limited_exp_arg = assign2160_e3368; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2160_e3370: f64 = (p.p149 * assign2160_e3369);
        let assign2160_e3371: f64 = (var_ud_i + assign2160_e3370);
        var_ud_i = assign2160_e3371;

        let assign2170_e3375: f64 = (-var_leff);
        let assign2170_e3377: f64 = (assign2170_e3375 / p.p144);
        let assign2170_e3378: f64 = { let limited_exp_arg = assign2170_e3377; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2170_e3379: f64 = (p.p143 * assign2170_e3378);
        let assign2170_e3380: f64 = (var_eu_i + assign2170_e3379);
        var_eu_i = assign2170_e3380;

        let assign2180_e3384: f64 = (-var_leff);
        let assign2180_e3386: f64 = (assign2180_e3384 / p.p165);
        let assign2180_e3387: f64 = { let limited_exp_arg = assign2180_e3386; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2180_e3388: f64 = (p.p164 * assign2180_e3387);
        let assign2180_e3389: f64 = (var_eub_i + assign2180_e3388);
        var_eub_i = assign2180_e3389;

        let assign2190_e3392: f64 = if p.p188 > 0.0 { 1.0 } else { 0.0 };
        var_guard21 = assign2190_e3392;

        let (assign2200_e3405,) = {
    if (var_guard21 != 0.0) {
        let assign2200_e3399: f64 = (-p.p188);
        let assign2200_e3400: f64 = (var_leff).powf(assign2200_e3399);
        let assign2200_e3401: f64 = (var_up2_i * assign2200_e3400);
        let assign2200_e3402: f64 = (1.0 - assign2200_e3401);
        let assign2200_e3403: f64 = (var_u02_i * assign2200_e3402);
        (assign2200_e3403,)
    } else {
        (var_u02_i,)
    }
};
        var_u02_i = assign2200_e3405;

        let (assign2210_e3414,) = {
    if (var_guard21 == 0.0) {
        let assign2210_e3411: f64 = (1.0 - var_up2_i);
        let assign2210_e3412: f64 = (var_u02_i * assign2210_e3411);
        (assign2210_e3412,)
    } else {
        (var_u02_i,)
    }
};
        var_u02_i = assign2210_e3414;

        let assign2220_e3418: f64 = (-var_leff);
        let assign2220_e3420: f64 = (assign2220_e3418 / p.p169);
        let assign2220_e3421: f64 = { let limited_exp_arg = assign2220_e3420; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2220_e3422: f64 = (p.p168 * assign2220_e3421);
        let assign2220_e3423: f64 = (var_ua2_i + assign2220_e3422);
        var_ua2_i = assign2220_e3423;

        let assign2230_e3427: f64 = (-var_leff);
        let assign2230_e3429: f64 = (assign2230_e3427 / p.p175);
        let assign2230_e3430: f64 = { let limited_exp_arg = assign2230_e3429; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2230_e3431: f64 = (p.p174 * assign2230_e3430);
        let assign2230_e3432: f64 = (var_uc2_i + assign2230_e3431);
        var_uc2_i = assign2230_e3432;

        let assign2240_e3436: f64 = (-var_leff);
        let assign2240_e3438: f64 = (assign2240_e3436 / p.p181);
        let assign2240_e3439: f64 = { let limited_exp_arg = assign2240_e3438; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2240_e3440: f64 = (p.p180 * assign2240_e3439);
        let assign2240_e3441: f64 = (p.p179 + assign2240_e3440);
        var_udb2_i = assign2240_e3441;

        let assign2250_e3445: f64 = (-var_leff);
        let assign2250_e3447: f64 = (assign2250_e3445 / p.p178);
        let assign2250_e3448: f64 = { let limited_exp_arg = assign2250_e3447; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2250_e3449: f64 = (p.p177 * assign2250_e3448);
        let assign2250_e3450: f64 = (var_ud2_i + assign2250_e3449);
        var_ud2_i = assign2250_e3450;

        let assign2260_e3454: f64 = (-var_leff);
        let assign2260_e3456: f64 = (assign2260_e3454 / p.p172);
        let assign2260_e3457: f64 = { let limited_exp_arg = assign2260_e3456; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2260_e3458: f64 = (p.p171 * assign2260_e3457);
        let assign2260_e3459: f64 = (var_eu2_i + assign2260_e3458);
        var_eu2_i = assign2260_e3459;

        let assign2270_e3463: f64 = (-var_leff);
        let assign2270_e3465: f64 = (assign2270_e3463 / p.p185);
        let assign2270_e3466: f64 = { let limited_exp_arg = assign2270_e3465; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2270_e3467: f64 = (p.p184 * assign2270_e3466);
        let assign2270_e3468: f64 = (var_eub2_i + assign2270_e3467);
        var_eub2_i = assign2270_e3468;

        let assign2280_e3471: f64 = if p.p14 == 1.0 { 1.0 } else { 0.0 };
        var_guard22 = assign2280_e3471;

        let (assign2290_e3483,) = {
    if (var_guard22 != 0.0) {
        let assign2290_e3476: f64 = (-var_leff);
        let assign2290_e3478: f64 = (assign2290_e3476 / p.p197);
        let assign2290_e3479: f64 = { let limited_exp_arg = assign2290_e3478; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2290_e3480: f64 = (p.p196 * assign2290_e3479);
        let assign2290_e3481: f64 = (var_rsw_i + assign2290_e3480);
        (assign2290_e3481,)
    } else {
        (var_rsw_i,)
    }
};
        var_rsw_i = assign2290_e3483;

        let (assign2300_e3495,) = {
    if (var_guard22 != 0.0) {
        let assign2300_e3488: f64 = (-var_leff);
        let assign2300_e3490: f64 = (assign2300_e3488 / p.p201);
        let assign2300_e3491: f64 = { let limited_exp_arg = assign2300_e3490; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2300_e3492: f64 = (p.p200 * assign2300_e3491);
        let assign2300_e3493: f64 = (var_rdw_i + assign2300_e3492);
        (assign2300_e3493,)
    } else {
        (var_rdw_i,)
    }
};
        var_rdw_i = assign2300_e3495;

        let (assign2310_e3508,) = {
    if (var_guard22 == 0.0) {
        let assign2310_e3501: f64 = (-var_leff);
        let assign2310_e3503: f64 = (assign2310_e3501 / p.p193);
        let assign2310_e3504: f64 = { let limited_exp_arg = assign2310_e3503; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2310_e3505: f64 = (p.p192 * assign2310_e3504);
        let assign2310_e3506: f64 = (var_rdsw_i + assign2310_e3505);
        (assign2310_e3506,)
    } else {
        (var_rdsw_i,)
    }
};
        var_rdsw_i = assign2310_e3508;

        let assign2320_e3512: f64 = (-var_leff);
        let assign2320_e3514: f64 = (assign2320_e3512 / p.p212);
        let assign2320_e3515: f64 = { let limited_exp_arg = assign2320_e3514; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2320_e3516: f64 = (p.p211 * assign2320_e3515);
        let assign2320_e3517: f64 = (var_pclm_i + assign2320_e3516);
        var_pclm_i = assign2320_e3517;

        let assign2330_e3522: f64 = (var_leff * 1000000.0);
        let assign2330_e3524: f64 = (-p.p115);
        let assign2330_e3525: f64 = (assign2330_e3522).powf(assign2330_e3524);
        let assign2330_e3526: f64 = (p.p114 * assign2330_e3525);
        let assign2330_e3527: f64 = (var_mexp_i + assign2330_e3526);
        var_mexp_i = assign2330_e3527;

        let assign2340_e3531: f64 = (-var_leff);
        let assign2340_e3533: f64 = (assign2340_e3531 / p.p118);
        let assign2340_e3534: f64 = { let limited_exp_arg = assign2340_e3533; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2340_e3535: f64 = (p.p117 * assign2340_e3534);
        let assign2340_e3536: f64 = (var_ptwg_i + assign2340_e3535);
        var_ptwg_i = assign2340_e3536;

        let assign2350_e3540: f64 = (-var_leff);
        let assign2350_e3542: f64 = (assign2350_e3540 / p.p126);
        let assign2350_e3543: f64 = { let limited_exp_arg = assign2350_e3542; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2350_e3544: f64 = (p.p125 * assign2350_e3543);
        let assign2350_e3545: f64 = (var_ptwgb_i + assign2350_e3544);
        var_ptwgb_i = assign2350_e3545;

        let assign2360_e3549: f64 = (-var_leff);
        let assign2360_e3551: f64 = (assign2360_e3549 / p.p128);
        let assign2360_e3552: f64 = { let limited_exp_arg = assign2360_e3551; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2360_e3553: f64 = (p.p127 * assign2360_e3552);
        let assign2360_e3554: f64 = (var_ptwgb2_i + assign2360_e3553);
        var_ptwgb2_i = assign2360_e3554;

        let assign2370_e3558: f64 = (-var_leff);
        let assign2370_e3560: f64 = (assign2370_e3558 / p.p102);
        let assign2370_e3561: f64 = { let limited_exp_arg = assign2370_e3560; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2370_e3562: f64 = (p.p101 * assign2370_e3561);
        let assign2370_e3563: f64 = (var_vsat_i + assign2370_e3562);
        var_vsat_i = assign2370_e3563;

        let assign2380_e3567: f64 = (-var_leff);
        let assign2380_e3569: f64 = (assign2380_e3567 / p.p133);
        let assign2380_e3570: f64 = { let limited_exp_arg = assign2380_e3569; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2380_e3571: f64 = (p.p132 * assign2380_e3570);
        let assign2380_e3572: f64 = (var_vsatb_i + assign2380_e3571);
        var_vsatb_i = assign2380_e3572;

        let assign2390_e3576: f64 = (-var_leff);
        let assign2390_e3578: f64 = (assign2390_e3576 / p.p105);
        let assign2390_e3579: f64 = { let limited_exp_arg = assign2390_e3578; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2390_e3580: f64 = (p.p104 * assign2390_e3579);
        let assign2390_e3581: f64 = (var_vsat1_i + assign2390_e3580);
        var_vsat1_i = assign2390_e3581;

        let assign2400_e3585: f64 = (-var_leff);
        let assign2400_e3587: f64 = (assign2400_e3585 / p.p108);
        let assign2400_e3588: f64 = { let limited_exp_arg = assign2400_e3587; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2400_e3589: f64 = (p.p107 * assign2400_e3588);
        let assign2400_e3590: f64 = (var_vsatcv_i + assign2400_e3589);
        var_vsatcv_i = assign2400_e3590;

        let assign2410_e3594: f64 = (-var_leff);
        let assign2410_e3596: f64 = (assign2410_e3594 / p.p80);
        let assign2410_e3597: f64 = { let limited_exp_arg = assign2410_e3596; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2410_e3598: f64 = (p.p79 * assign2410_e3597);
        let assign2410_e3599: f64 = (p.p77 + assign2410_e3598);
        var_dvtp0_i = assign2410_e3599;

        let assign2420_e3603: f64 = (-var_leff);
        let assign2420_e3605: f64 = (assign2420_e3603 / p.p82);
        let assign2420_e3606: f64 = { let limited_exp_arg = assign2420_e3605; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign2420_e3607: f64 = (p.p81 * assign2420_e3606);
        let assign2420_e3608: f64 = (p.p78 + assign2420_e3607);
        var_dvtp1_i = assign2420_e3608;

        let assign2430_e3611: f64 = if var_u0_i < 0.0 { 1.0 } else { 0.0 };
        var_guard23 = assign2430_e3611;

        let (assign2440_e3615,) = {
    if (var_guard23 != 0.0) {
        (0.03,)
    } else {
        (var_u0_i,)
    }
};
        var_u0_i = assign2440_e3615;

        let assign2450_e3618: f64 = if var_ua_i < 0.0 { 1.0 } else { 0.0 };
        var_guard24 = assign2450_e3618;

        let (assign2460_e3622,) = {
    if (var_guard24 != 0.0) {
        (0.0,)
    } else {
        (var_ua_i,)
    }
};
        var_ua_i = assign2460_e3622;

        let assign2470_e3625: f64 = if var_eu_i < 0.0 { 1.0 } else { 0.0 };
        var_guard25 = assign2470_e3625;

        let (assign2480_e3629,) = {
    if (var_guard25 != 0.0) {
        (0.0,)
    } else {
        (var_eu_i,)
    }
};
        var_eu_i = assign2480_e3629;

        let assign2490_e3632: f64 = if var_ud_i < 0.0 { 1.0 } else { 0.0 };
        var_guard26 = assign2490_e3632;

        let (assign2500_e3636,) = {
    if (var_guard26 != 0.0) {
        (0.0,)
    } else {
        (var_ud_i,)
    }
};
        var_ud_i = assign2500_e3636;

        let assign2510_e3639: f64 = if var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        var_guard27 = assign2510_e3639;

        let (assign2520_e3643,) = {
    if (var_guard27 != 0.0) {
        (0.0,)
    } else {
        (var_ucs_i,)
    }
};
        var_ucs_i = assign2520_e3643;

        let assign2530_e3646: f64 = if var_vsatb_i < 0.0 { 1.0 } else { 0.0 };
        var_guard28 = assign2530_e3646;

        let (assign2540_e3650,) = {
    if (var_guard28 != 0.0) {
        (0.0,)
    } else {
        (var_vsatb_i,)
    }
};
        var_vsatb_i = assign2540_e3650;

        var_rdswmin_i = p.p190;

        let assign2580_e3660: f64 = if var_rdswmin_i < 0.0 { 1.0 } else { 0.0 };
        var_guard31 = assign2580_e3660;

        let (assign2590_e3664,) = {
    if (var_guard31 != 0.0) {
        (0.0,)
    } else {
        (var_rdswmin_i,)
    }
};
        var_rdswmin_i = assign2590_e3664;

        let assign2600_e3667: f64 = if var_rdsw_i < 0.0 { 1.0 } else { 0.0 };
        var_guard32 = assign2600_e3667;

        let (assign2610_e3671,) = {
    if (var_guard32 != 0.0) {
        (0.0,)
    } else {
        (var_rdsw_i,)
    }
};
        var_rdsw_i = assign2610_e3671;

        var_rswmin_i = p.p194;

        let assign2630_e3675: f64 = if var_rswmin_i < 0.0 { 1.0 } else { 0.0 };
        var_guard33 = assign2630_e3675;

        let (assign2640_e3679,) = {
    if (var_guard33 != 0.0) {
        (0.0,)
    } else {
        (var_rswmin_i,)
    }
};
        var_rswmin_i = assign2640_e3679;

        let assign2650_e3682: f64 = if var_rsw_i < 0.0 { 1.0 } else { 0.0 };
        var_guard34 = assign2650_e3682;

        let (assign2660_e3686,) = {
    if (var_guard34 != 0.0) {
        (0.0,)
    } else {
        (var_rsw_i,)
    }
};
        var_rsw_i = assign2660_e3686;

        var_rdwmin_i = p.p198;

        let assign2680_e3690: f64 = if var_rdwmin_i < 0.0 { 1.0 } else { 0.0 };
        var_guard35 = assign2680_e3690;

        let (assign2690_e3694,) = {
    if (var_guard35 != 0.0) {
        (0.0,)
    } else {
        (var_rdwmin_i,)
    }
};
        var_rdwmin_i = assign2690_e3694;

        let assign2700_e3697: f64 = if var_rdw_i < 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign2700_e3697;

        let (assign2710_e3701,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_rdw_i,)
    }
};
        var_rdw_i = assign2710_e3701;

        let assign2720_e3704: f64 = if var_prwg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard37 = assign2720_e3704;

        let (assign2730_e3708,) = {
    if (var_guard37 != 0.0) {
        (0.0,)
    } else {
        (var_prwg_i,)
    }
};
        var_prwg_i = assign2730_e3708;

        let assign2770_e3720: f64 = if var_mexp_i < 2.0 { 1.0 } else { 0.0 };
        var_guard41 = assign2770_e3720;

        let (assign2780_e3724,) = {
    if (var_guard41 != 0.0) {
        (2.0,)
    } else {
        (var_mexp_i,)
    }
};
        var_mexp_i = assign2780_e3724;

        let assign2790_e3728: f64 = (var_lpe0_i / var_leff);
        let assign2790_e3729: f64 = (1.0 + assign2790_e3728);
        let assign2790_e3730: f64 = (assign2790_e3729).sqrt();
        let assign2790_e3732: f64 = (assign2790_e3730 - 1.0);
        var_theta_rsce = assign2790_e3732;

        let assign2800_e3737: f64 = (p.p45 + p.p46);
        let assign2800_e3738: f64 = (var_epsratio * assign2800_e3737);
        let assign2800_e3739: f64 = (p.p49 + assign2800_e3738);
        var_teff = assign2800_e3739;

        let assign2810_e3742: f64 = (1.0 / var_mexp_i);
        var_inv_mexp = assign2810_e3742;

        let assign2820_e3745: f64 = (var_cox2 * p.p3);
        var_csbox = assign2820_e3745;
        var_csbox_dn3 = 0.0;
        var_csbox_dn4 = 0.0;
        var_csbox_dn5 = 0.0;
        var_csbox_dn6 = 0.0;
        var_csbox_dn7 = 0.0;
        var_csbox_dn8 = 0.0;

        let assign2830_e3748: f64 = (var_cox2 * p.p4);
        var_cdbox = assign2830_e3748;
        var_cdbox_dn3 = 0.0;
        var_cdbox_dn4 = 0.0;
        var_cdbox_dn5 = 0.0;
        var_cdbox_dn6 = 0.0;
        var_cdbox_dn7 = 0.0;
        var_cdbox_dn8 = 0.0;

        let assign2840_e3753: f64 = (p.p49 / p.p46);
        let assign2840_e3754: f64 = (1.0 + assign2840_e3753);
        let assign2840_e3756: f64 = (assign2840_e3754).max(1e-38);
        let assign2840_e3757: f64 = (assign2840_e3756).ln();
        let assign2840_e3758: f64 = (p.p267 * assign2840_e3757);
        var_t0 = assign2840_e3758;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;

        let assign2850_e3763: f64 = (p.p5 - p.p1);
        let assign2850_e3765: f64 = (assign2850_e3763).max(0.0);
        let assign2850_e3766: f64 = (var_t0 * assign2850_e3765);
        let assign2850_e3767: f64 = (var_csbox + assign2850_e3766);
        var_csbox = assign2850_e3767;
        var_csbox_dn3 = (var_csbox_dn3 + (var_t0_dn3 * assign2850_e3765));
        var_csbox_dn4 = (var_csbox_dn4 + (var_t0_dn4 * assign2850_e3765));
        var_csbox_dn5 = (var_csbox_dn5 + (var_t0_dn5 * assign2850_e3765));
        var_csbox_dn6 = (var_csbox_dn6 + (var_t0_dn6 * assign2850_e3765));
        var_csbox_dn7 = (var_csbox_dn7 + (var_t0_dn7 * assign2850_e3765));
        var_csbox_dn8 = (var_csbox_dn8 + (var_t0_dn8 * assign2850_e3765));

        let assign2860_e3772: f64 = (p.p6 - p.p1);
        let assign2860_e3774: f64 = (assign2860_e3772).max(0.0);
        let assign2860_e3775: f64 = (var_t0 * assign2860_e3774);
        let assign2860_e3776: f64 = (var_cdbox + assign2860_e3775);
        var_cdbox = assign2860_e3776;
        var_cdbox_dn3 = (var_cdbox_dn3 + (var_t0_dn3 * assign2860_e3774));
        var_cdbox_dn4 = (var_cdbox_dn4 + (var_t0_dn4 * assign2860_e3774));
        var_cdbox_dn5 = (var_cdbox_dn5 + (var_t0_dn5 * assign2860_e3774));
        var_cdbox_dn6 = (var_cdbox_dn6 + (var_t0_dn6 * assign2860_e3774));
        var_cdbox_dn7 = (var_cdbox_dn7 + (var_t0_dn7 * assign2860_e3774));
        var_cdbox_dn8 = (var_cdbox_dn8 + (var_t0_dn8 * assign2860_e3774));

        let assign2870_e3779: f64 = (var_csbox).max(1e-20);
        var_csbox = assign2870_e3779;
        var_csbox_dn3 = if var_csbox >= 1e-20 { var_csbox_dn3 } else { 0.0 };
        var_csbox_dn4 = if var_csbox >= 1e-20 { var_csbox_dn4 } else { 0.0 };
        var_csbox_dn5 = if var_csbox >= 1e-20 { var_csbox_dn5 } else { 0.0 };
        var_csbox_dn6 = if var_csbox >= 1e-20 { var_csbox_dn6 } else { 0.0 };
        var_csbox_dn7 = if var_csbox >= 1e-20 { var_csbox_dn7 } else { 0.0 };
        var_csbox_dn8 = if var_csbox >= 1e-20 { var_csbox_dn8 } else { 0.0 };

        let assign2880_e3782: f64 = (var_cdbox).max(1e-20);
        var_cdbox = assign2880_e3782;
        var_cdbox_dn3 = if var_cdbox >= 1e-20 { var_cdbox_dn3 } else { 0.0 };
        var_cdbox_dn4 = if var_cdbox >= 1e-20 { var_cdbox_dn4 } else { 0.0 };
        var_cdbox_dn5 = if var_cdbox >= 1e-20 { var_cdbox_dn5 } else { 0.0 };
        var_cdbox_dn6 = if var_cdbox >= 1e-20 { var_cdbox_dn6 } else { 0.0 };
        var_cdbox_dn7 = if var_cdbox >= 1e-20 { var_cdbox_dn7 } else { 0.0 };
        var_cdbox_dn8 = if var_cdbox >= 1e-20 { var_cdbox_dn8 } else { 0.0 };

        let assign2890_e3785: f64 = (0.5 * var_etamob_i);
        var_eta_mu = assign2890_e3785;

        var_eta_mu_cv = 0.5;

        let assign2910_e3789: f64 = (0.5 * var_etamob2_i);
        var_eta_mu2 = assign2910_e3789;

        let assign2920_e3792: f64 = if p.p12 != 1.0 { 1.0 } else { 0.0 };
        var_guard42 = assign2920_e3792;

        let (assign2930_e3800,) = {
    if (var_guard42 != 0.0) {
        let assign2930_e3796: f64 = (1.0 / 3.0);
        let assign2930_e3798: f64 = (assign2930_e3796 * var_etamob_i);
        (assign2930_e3798,)
    } else {
        (var_eta_mu,)
    }
};
        var_eta_mu = assign2930_e3800;

        *var_cdbox_slot = var_cdbox;
        *var_cdbox_dn3_slot = var_cdbox_dn3;
        *var_cdbox_dn4_slot = var_cdbox_dn4;
        *var_cdbox_dn5_slot = var_cdbox_dn5;
        *var_cdbox_dn6_slot = var_cdbox_dn6;
        *var_cdbox_dn7_slot = var_cdbox_dn7;
        *var_cdbox_dn8_slot = var_cdbox_dn8;
        *var_csbox_slot = var_csbox;
        *var_csbox_dn3_slot = var_csbox_dn3;
        *var_csbox_dn4_slot = var_csbox_dn4;
        *var_csbox_dn5_slot = var_csbox_dn5;
        *var_csbox_dn6_slot = var_csbox_dn6;
        *var_csbox_dn7_slot = var_csbox_dn7;
        *var_csbox_dn8_slot = var_csbox_dn8;
        *var_dvtp0_i_slot = var_dvtp0_i;
        *var_dvtp1_i_slot = var_dvtp1_i;
        *var_eta_mu_slot = var_eta_mu;
        *var_eta_mu2_slot = var_eta_mu2;
        *var_eta_mu_cv_slot = var_eta_mu_cv;
        *var_eu2_i_slot = var_eu2_i;
        *var_eu_i_slot = var_eu_i;
        *var_eub2_i_slot = var_eub2_i;
        *var_eub_i_slot = var_eub_i;
        *var_guard21_slot = var_guard21;
        *var_guard22_slot = var_guard22;
        *var_guard23_slot = var_guard23;
        *var_guard24_slot = var_guard24;
        *var_guard25_slot = var_guard25;
        *var_guard26_slot = var_guard26;
        *var_guard27_slot = var_guard27;
        *var_guard28_slot = var_guard28;
        *var_guard31_slot = var_guard31;
        *var_guard32_slot = var_guard32;
        *var_guard33_slot = var_guard33;
        *var_guard34_slot = var_guard34;
        *var_guard35_slot = var_guard35;
        *var_guard36_slot = var_guard36;
        *var_guard37_slot = var_guard37;
        *var_guard41_slot = var_guard41;
        *var_guard42_slot = var_guard42;
        *var_inv_mexp_slot = var_inv_mexp;
        *var_mexp_i_slot = var_mexp_i;
        *var_pclm_i_slot = var_pclm_i;
        *var_prwg_i_slot = var_prwg_i;
        *var_ptwg_i_slot = var_ptwg_i;
        *var_ptwgb2_i_slot = var_ptwgb2_i;
        *var_ptwgb_i_slot = var_ptwgb_i;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdswmin_i_slot = var_rdswmin_i;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdwmin_i_slot = var_rdwmin_i;
        *var_rsw_i_slot = var_rsw_i;
        *var_rswmin_i_slot = var_rswmin_i;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_teff_slot = var_teff;
        *var_theta_rsce_slot = var_theta_rsce;
        *var_u02_i_slot = var_u02_i;
        *var_u0_i_slot = var_u0_i;
        *var_ua2_i_slot = var_ua2_i;
        *var_ua_i_slot = var_ua_i;
        *var_uc2_i_slot = var_uc2_i;
        *var_ucs_i_slot = var_ucs_i;
        *var_ud2_i_slot = var_ud2_i;
        *var_ud_i_slot = var_ud_i;
        *var_udb2_i_slot = var_udb2_i;
        *var_udb_i_slot = var_udb_i;
        *var_vsat1_i_slot = var_vsat1_i;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsatb_i_slot = var_vsatb_i;
        *var_vsatcv_i_slot = var_vsatcv_i;
    }

    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_epsratio: f64,
        var_etamob2_i: f64,
        var_guard42: f64,
        var_leff: f64,
        var_nbody_i: f64,
        var_nsd_i: f64,
        var_ntox_i: f64,
        var_poxedge_i: f64,
        var_weff: f64,
        var_wr_i: f64,
        var_aechvb_slot: &mut f64,
        var_bechvb_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_deltemp_slot: &mut f64,
        var_deltemp_dn4_slot: &mut f64,
        var_devtemp_slot: &mut f64,
        var_devtemp_dn4_slot: &mut f64,
        var_eefffactor_slot: &mut f64,
        var_eefffactor2_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_eta_mu2_slot: &mut f64,
        var_eta_mu_cv_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard56_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard61_slot: &mut f64,
        var_guard62_slot: &mut f64,
        var_igsd_mult0_slot: &mut f64,
        var_igsd_mult0_dn3_slot: &mut f64,
        var_igsd_mult0_dn4_slot: &mut f64,
        var_igsd_mult0_dn5_slot: &mut f64,
        var_igsd_mult0_dn6_slot: &mut f64,
        var_igsd_mult0_dn7_slot: &mut f64,
        var_igsd_mult0_dn8_slot: &mut f64,
        var_lintnoi_i_slot: &mut f64,
        var_litl_slot: &mut f64,
        var_ni_slot: &mut f64,
        var_ni_dn3_slot: &mut f64,
        var_ni_dn4_slot: &mut f64,
        var_ni_dn5_slot: &mut f64,
        var_ni_dn6_slot: &mut f64,
        var_ni_dn7_slot: &mut f64,
        var_ni_dn8_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_dn3_slot: &mut f64,
        var_phib_dn4_slot: &mut f64,
        var_phib_dn5_slot: &mut f64,
        var_phib_dn6_slot: &mut f64,
        var_phib_dn7_slot: &mut f64,
        var_phib_dn8_slot: &mut f64,
        var_phisub_slot: &mut f64,
        var_phisub_dn3_slot: &mut f64,
        var_phisub_dn4_slot: &mut f64,
        var_phisub_dn5_slot: &mut f64,
        var_phisub_dn6_slot: &mut f64,
        var_phisub_dn7_slot: &mut f64,
        var_phisub_dn8_slot: &mut f64,
        var_rdraingeo_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdswmin_i_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdwmin_i_slot: &mut f64,
        var_rsourcegeo_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rswmin_i_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_tmaxk_slot: &mut f64,
        var_tnom_slot: &mut f64,
        var_toxratio_slot: &mut f64,
        var_toxratio_dn3_slot: &mut f64,
        var_toxratio_dn4_slot: &mut f64,
        var_toxratio_dn5_slot: &mut f64,
        var_toxratio_dn6_slot: &mut f64,
        var_toxratio_dn7_slot: &mut f64,
        var_toxratio_dn8_slot: &mut f64,
        var_toxratioedge_slot: &mut f64,
        var_toxratioedge_dn3_slot: &mut f64,
        var_toxratioedge_dn4_slot: &mut f64,
        var_toxratioedge_dn5_slot: &mut f64,
        var_toxratioedge_dn6_slot: &mut f64,
        var_toxratioedge_dn7_slot: &mut f64,
        var_toxratioedge_dn8_slot: &mut f64,
        var_tratio_slot: &mut f64,
        var_tratio_dn4_slot: &mut f64,
        var_vbi_slot: &mut f64,
        var_vbi_dn3_slot: &mut f64,
        var_vbi_dn4_slot: &mut f64,
        var_vbi_dn5_slot: &mut f64,
        var_vbi_dn6_slot: &mut f64,
        var_vbi_dn7_slot: &mut f64,
        var_vbi_dn8_slot: &mut f64,
        var_vtm_slot: &mut f64,
        var_vtm_dn4_slot: &mut f64,
        var_weffwrfactor_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_aechvb: f64 = *var_aechvb_slot;
        let mut var_bechvb: f64 = *var_bechvb_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_deltemp: f64 = *var_deltemp_slot;
        let mut var_deltemp_dn4: f64 = *var_deltemp_dn4_slot;
        let mut var_devtemp: f64 = *var_devtemp_slot;
        let mut var_devtemp_dn4: f64 = *var_devtemp_dn4_slot;
        let mut var_eefffactor: f64 = *var_eefffactor_slot;
        let mut var_eefffactor2: f64 = *var_eefffactor2_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_eta_mu2: f64 = *var_eta_mu2_slot;
        let mut var_eta_mu_cv: f64 = *var_eta_mu_cv_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard61: f64 = *var_guard61_slot;
        let mut var_guard62: f64 = *var_guard62_slot;
        let mut var_igsd_mult0: f64 = *var_igsd_mult0_slot;
        let mut var_igsd_mult0_dn3: f64 = *var_igsd_mult0_dn3_slot;
        let mut var_igsd_mult0_dn4: f64 = *var_igsd_mult0_dn4_slot;
        let mut var_igsd_mult0_dn5: f64 = *var_igsd_mult0_dn5_slot;
        let mut var_igsd_mult0_dn6: f64 = *var_igsd_mult0_dn6_slot;
        let mut var_igsd_mult0_dn7: f64 = *var_igsd_mult0_dn7_slot;
        let mut var_igsd_mult0_dn8: f64 = *var_igsd_mult0_dn8_slot;
        let mut var_lintnoi_i: f64 = *var_lintnoi_i_slot;
        let mut var_litl: f64 = *var_litl_slot;
        let mut var_ni: f64 = *var_ni_slot;
        let mut var_ni_dn3: f64 = *var_ni_dn3_slot;
        let mut var_ni_dn4: f64 = *var_ni_dn4_slot;
        let mut var_ni_dn5: f64 = *var_ni_dn5_slot;
        let mut var_ni_dn6: f64 = *var_ni_dn6_slot;
        let mut var_ni_dn7: f64 = *var_ni_dn7_slot;
        let mut var_ni_dn8: f64 = *var_ni_dn8_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_dn3: f64 = *var_phib_dn3_slot;
        let mut var_phib_dn4: f64 = *var_phib_dn4_slot;
        let mut var_phib_dn5: f64 = *var_phib_dn5_slot;
        let mut var_phib_dn6: f64 = *var_phib_dn6_slot;
        let mut var_phib_dn7: f64 = *var_phib_dn7_slot;
        let mut var_phib_dn8: f64 = *var_phib_dn8_slot;
        let mut var_phisub: f64 = *var_phisub_slot;
        let mut var_phisub_dn3: f64 = *var_phisub_dn3_slot;
        let mut var_phisub_dn4: f64 = *var_phisub_dn4_slot;
        let mut var_phisub_dn5: f64 = *var_phisub_dn5_slot;
        let mut var_phisub_dn6: f64 = *var_phisub_dn6_slot;
        let mut var_phisub_dn7: f64 = *var_phisub_dn7_slot;
        let mut var_phisub_dn8: f64 = *var_phisub_dn8_slot;
        let mut var_rdraingeo: f64 = *var_rdraingeo_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdswmin_i: f64 = *var_rdswmin_i_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdwmin_i: f64 = *var_rdwmin_i_slot;
        let mut var_rsourcegeo: f64 = *var_rsourcegeo_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rswmin_i: f64 = *var_rswmin_i_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_tmaxk: f64 = *var_tmaxk_slot;
        let mut var_tnom: f64 = *var_tnom_slot;
        let mut var_toxratio: f64 = *var_toxratio_slot;
        let mut var_toxratio_dn3: f64 = *var_toxratio_dn3_slot;
        let mut var_toxratio_dn4: f64 = *var_toxratio_dn4_slot;
        let mut var_toxratio_dn5: f64 = *var_toxratio_dn5_slot;
        let mut var_toxratio_dn6: f64 = *var_toxratio_dn6_slot;
        let mut var_toxratio_dn7: f64 = *var_toxratio_dn7_slot;
        let mut var_toxratio_dn8: f64 = *var_toxratio_dn8_slot;
        let mut var_toxratioedge: f64 = *var_toxratioedge_slot;
        let mut var_toxratioedge_dn3: f64 = *var_toxratioedge_dn3_slot;
        let mut var_toxratioedge_dn4: f64 = *var_toxratioedge_dn4_slot;
        let mut var_toxratioedge_dn5: f64 = *var_toxratioedge_dn5_slot;
        let mut var_toxratioedge_dn6: f64 = *var_toxratioedge_dn6_slot;
        let mut var_toxratioedge_dn7: f64 = *var_toxratioedge_dn7_slot;
        let mut var_toxratioedge_dn8: f64 = *var_toxratioedge_dn8_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn4: f64 = *var_tratio_dn4_slot;
        let mut var_vbi: f64 = *var_vbi_slot;
        let mut var_vbi_dn3: f64 = *var_vbi_dn3_slot;
        let mut var_vbi_dn4: f64 = *var_vbi_dn4_slot;
        let mut var_vbi_dn5: f64 = *var_vbi_dn5_slot;
        let mut var_vbi_dn6: f64 = *var_vbi_dn6_slot;
        let mut var_vbi_dn7: f64 = *var_vbi_dn7_slot;
        let mut var_vbi_dn8: f64 = *var_vbi_dn8_slot;
        let mut var_vtm: f64 = *var_vtm_slot;
        let mut var_vtm_dn4: f64 = *var_vtm_dn4_slot;
        let mut var_weffwrfactor: f64 = *var_weffwrfactor_slot;

        let (assign2940_e3806,) = {
    if (var_guard42 != 0.0) {
        let assign2940_e3804: f64 = (1.0 / 3.0);
        (assign2940_e3804,)
    } else {
        (var_eta_mu_cv,)
    }
};
        var_eta_mu_cv = assign2940_e3806;

        let (assign2950_e3814,) = {
    if (var_guard42 != 0.0) {
        let assign2950_e3810: f64 = (1.0 / 3.0);
        let assign2950_e3812: f64 = (assign2950_e3810 * var_etamob2_i);
        (assign2950_e3812,)
    } else {
        (var_eta_mu2,)
    }
};
        var_eta_mu2 = assign2950_e3814;

        let assign2960_e3818: f64 = (var_epsratio * p.p45);
        let assign2960_e3819: f64 = (1e-8 / assign2960_e3818);
        var_eefffactor = assign2960_e3819;

        let assign2970_e3823: f64 = (var_weff * 1000000.0);
        let assign2970_e3825: f64 = (assign2970_e3823).powf(var_wr_i);
        let assign2970_e3827: f64 = (assign2970_e3825 * p.p2);
        let assign2970_e3828: f64 = (1.0 / assign2970_e3827);
        var_weffwrfactor = assign2970_e3828;

        let assign2980_e3831: f64 = (var_epsratio * p.p45);
        let assign2980_e3833: f64 = (assign2980_e3831 * p.p49);
        let assign2980_e3834: f64 = (assign2980_e3833).sqrt();
        var_litl = assign2980_e3834;

        let assign2990_e3838: f64 = (var_epsratio * p.p46);
        let assign2990_e3839: f64 = (1e-8 / assign2990_e3838);
        var_eefffactor2 = assign2990_e3839;

        let assign3000_e3843: f64 = (var_leff / 2.0);
        let assign3000_e3844: f64 = if p.p296 >= assign3000_e3843 { 1.0 } else { 0.0 };
        var_guard43 = assign3000_e3844;

        let (assign3010_e3848,) = {
    if (var_guard43 != 0.0) {
        (0.0,)
    } else {
        (var_lintnoi_i,)
    }
};
        var_lintnoi_i = assign3010_e3848;

        let (assign3020_e3853,) = {
    if (var_guard43 == 0.0) {
        (p.p296,)
    } else {
        (var_lintnoi_i,)
    }
};
        var_lintnoi_i = assign3020_e3853;

        let assign3030_e3860: f64 = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };
        var_guard44 = assign3030_e3860;

        let (assign3040_e3870,) = {
    if (var_guard44 != 0.0) {
        let assign3040_e3865: f64 = (var_weff * p.p2);
        let assign3040_e3866: f64 = (p.p312 + assign3040_e3865);
        let assign3040_e3868: f64 = (assign3040_e3866 / p.p310);
        (assign3040_e3868,)
    } else {
        (var_gth,)
    }
};
        var_gth = assign3040_e3870;

        let (assign3050_e3880,) = {
    if (var_guard44 != 0.0) {
        let assign3050_e3876: f64 = (var_weff * p.p2);
        let assign3050_e3877: f64 = (p.p312 + assign3050_e3876);
        let assign3050_e3878: f64 = (p.p311 * assign3050_e3877);
        (assign3050_e3878,)
    } else {
        (var_cth,)
    }
};
        var_cth = assign3050_e3880;

        let (assign3060_e3885,) = {
    if (var_guard44 == 0.0) {
        (1.0,)
    } else {
        (var_gth,)
    }
};
        var_gth = assign3060_e3885;

        let (assign3070_e3890,) = {
    if (var_guard44 == 0.0) {
        (0.0,)
    } else {
        (var_cth,)
    }
};
        var_cth = assign3070_e3890;

        let assign3080_e3893: f64 = (p.p215 * p.p7);
        var_rsourcegeo = assign3080_e3893;

        let assign3090_e3896: f64 = (p.p216 * p.p8);
        var_rdraingeo = assign3090_e3896;

        let assign3100_e3899: f64 = if var_rsourcegeo <= 0.001 { 1.0 } else { 0.0 };
        var_guard45 = assign3100_e3899;

        let (assign3110_e3903,) = {
    if (var_guard45 != 0.0) {
        (0.001,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign3110_e3903;

        let assign3120_e3906: f64 = if var_rdraingeo <= 0.001 { 1.0 } else { 0.0 };
        var_guard46 = assign3120_e3906;

        let (assign3130_e3910,) = {
    if (var_guard46 != 0.0) {
        (0.001,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign3130_e3910;

        let assign3140_e3913: f64 = if p.p14 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign3140_e3913;

        let assign3150_e3916: f64 = if var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard48 = assign3150_e3916;

        let (assign3160_e3922,) = {
    if ((var_guard47 != 0.0) && (var_guard48 != 0.0)) {
        (0.0,)
    } else {
        (var_rswmin_i,)
    }
};
        var_rswmin_i = assign3160_e3922;

        let assign3170_e3925: f64 = if var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard49 = assign3170_e3925;

        let (assign3180_e3931,) = {
    if ((var_guard47 != 0.0) && (var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (var_rdwmin_i,)
    }
};
        var_rdwmin_i = assign3180_e3931;

        let assign3190_e3934: f64 = if var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard50 = assign3190_e3934;

        let (assign3200_e3940,) = {
    if ((var_guard47 != 0.0) && (var_guard50 != 0.0)) {
        (0.0,)
    } else {
        (var_rsw_i,)
    }
};
        var_rsw_i = assign3200_e3940;

        let assign3210_e3943: f64 = if var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard51 = assign3210_e3943;

        let (assign3220_e3949,) = {
    if ((var_guard47 != 0.0) && (var_guard51 != 0.0)) {
        (0.0,)
    } else {
        (var_rdw_i,)
    }
};
        var_rdw_i = assign3220_e3949;

        let assign3230_e3952: f64 = if var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard52 = assign3230_e3952;

        let (assign3240_e3959,) = {
    if ((var_guard47 == 0.0) && (var_guard52 != 0.0)) {
        (0.0,)
    } else {
        (var_rdswmin_i,)
    }
};
        var_rdswmin_i = assign3240_e3959;

        let assign3250_e3962: f64 = if var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard53 = assign3250_e3962;

        let (assign3260_e3969,) = {
    if ((var_guard47 == 0.0) && (var_guard53 != 0.0)) {
        (0.0,)
    } else {
        (var_rdsw_i,)
    }
};
        var_rdsw_i = assign3260_e3969;

        let assign3270_e3972: f64 = if p.p297 <= 0.0 { 1.0 } else { 0.0 };
        var_guard54 = assign3270_e3972;

        let (assign3280_e3976,) = {
    if (var_guard54 != 0.0) {
        (300.15,)
    } else {
        (var_tnom,)
    }
};
        var_tnom = assign3280_e3976;

        let (assign3290_e3983,) = {
    if (var_guard54 == 0.0) {
        let assign3290_e3981: f64 = (p.p297 + 273.15);
        (assign3290_e3981,)
    } else {
        (var_tnom,)
    }
};
        var_tnom = assign3290_e3983;

        let assign3300_e3986: f64 = if p.p12 == 1.0 { 1.0 } else { 0.0 };
        var_guard55 = assign3300_e3986;

        let (assign3310_e3990,) = {
    if (var_guard55 != 0.0) {
        (4.97232e-7,)
    } else {
        (var_aechvb,)
    }
};
        var_aechvb = assign3310_e3990;

        let (assign3320_e3995,) = {
    if (var_guard55 == 0.0) {
        (3.42537e-7,)
    } else {
        (var_aechvb,)
    }
};
        var_aechvb = assign3320_e3995;

        let assign3330_e3998: f64 = if p.p12 == 1.0 { 1.0 } else { 0.0 };
        var_guard56 = assign3330_e3998;

        let (assign3340_e4002,) = {
    if (var_guard56 != 0.0) {
        (745669000000.0,)
    } else {
        (var_bechvb,)
    }
};
        var_bechvb = assign3340_e4002;

        let (assign3350_e4007,) = {
    if (var_guard56 == 0.0) {
        (1166450000000.0,)
    } else {
        (var_bechvb,)
    }
};
        var_bechvb = assign3350_e4007;

        let assign3360_e4010: f64 = (p.p99 * p.p99);
        var_t0 = assign3360_e4010;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;

        let assign3370_e4013: f64 = (p.p99 * var_poxedge_i);
        var_t1 = assign3370_e4013;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;

        let assign3380_e4016: f64 = (var_t1 * var_t1);
        var_t2 = assign3380_e4016;
        var_t2_dn3 = ((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3));
        var_t2_dn4 = ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4));
        var_t2_dn5 = ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5));
        var_t2_dn6 = ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6));
        var_t2_dn7 = ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7));
        var_t2_dn8 = ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8));

        let assign3390_e4020: f64 = (p.p239 / p.p99);
        let assign3390_e4022: f64 = (assign3390_e4020).max(1e-38);
        let assign3390_e4023: f64 = (assign3390_e4022).ln();
        let assign3390_e4024: f64 = (var_ntox_i * assign3390_e4023);
        let assign3390_e4025: f64 = { let limited_exp_arg = assign3390_e4024; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3390_e4027: f64 = (assign3390_e4025 / var_t0);
        var_toxratio = assign3390_e4027;
        var_toxratio_dn3 = (-((assign3390_e4025 * var_t0_dn3) / (var_t0 * var_t0)));
        var_toxratio_dn4 = (-((assign3390_e4025 * var_t0_dn4) / (var_t0 * var_t0)));
        var_toxratio_dn5 = (-((assign3390_e4025 * var_t0_dn5) / (var_t0 * var_t0)));
        var_toxratio_dn6 = (-((assign3390_e4025 * var_t0_dn6) / (var_t0 * var_t0)));
        var_toxratio_dn7 = (-((assign3390_e4025 * var_t0_dn7) / (var_t0 * var_t0)));
        var_toxratio_dn8 = (-((assign3390_e4025 * var_t0_dn8) / (var_t0 * var_t0)));

        let assign3400_e4031: f64 = (p.p239 / var_t1);
        let assign3400_e4033: f64 = (assign3400_e4031).max(1e-38);
        let assign3400_e4034: f64 = (assign3400_e4033).ln();
        let assign3400_e4035: f64 = (var_ntox_i * assign3400_e4034);
        let assign3400_e4036: f64 = { let limited_exp_arg = assign3400_e4035; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3400_e4038: f64 = (assign3400_e4036 / var_t2);
        var_toxratioedge = assign3400_e4038;
        var_toxratioedge_dn3 = (((({ let limited_exp_arg = assign3400_e4035; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_ntox_i * (if assign3400_e4031 >= 1e-38 { (-((p.p239 * var_t1_dn3) / (var_t1 * var_t1))) } else { 0.0 } / assign3400_e4033))) * var_t2) - (assign3400_e4036 * var_t2_dn3)) / (var_t2 * var_t2));
        var_toxratioedge_dn4 = (((({ let limited_exp_arg = assign3400_e4035; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_ntox_i * (if assign3400_e4031 >= 1e-38 { (-((p.p239 * var_t1_dn4) / (var_t1 * var_t1))) } else { 0.0 } / assign3400_e4033))) * var_t2) - (assign3400_e4036 * var_t2_dn4)) / (var_t2 * var_t2));
        var_toxratioedge_dn5 = (((({ let limited_exp_arg = assign3400_e4035; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_ntox_i * (if assign3400_e4031 >= 1e-38 { (-((p.p239 * var_t1_dn5) / (var_t1 * var_t1))) } else { 0.0 } / assign3400_e4033))) * var_t2) - (assign3400_e4036 * var_t2_dn5)) / (var_t2 * var_t2));
        var_toxratioedge_dn6 = (((({ let limited_exp_arg = assign3400_e4035; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_ntox_i * (if assign3400_e4031 >= 1e-38 { (-((p.p239 * var_t1_dn6) / (var_t1 * var_t1))) } else { 0.0 } / assign3400_e4033))) * var_t2) - (assign3400_e4036 * var_t2_dn6)) / (var_t2 * var_t2));
        var_toxratioedge_dn7 = (((({ let limited_exp_arg = assign3400_e4035; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_ntox_i * (if assign3400_e4031 >= 1e-38 { (-((p.p239 * var_t1_dn7) / (var_t1 * var_t1))) } else { 0.0 } / assign3400_e4033))) * var_t2) - (assign3400_e4036 * var_t2_dn7)) / (var_t2 * var_t2));
        var_toxratioedge_dn8 = (((({ let limited_exp_arg = assign3400_e4035; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_ntox_i * (if assign3400_e4031 >= 1e-38 { (-((p.p239 * var_t1_dn8) / (var_t1 * var_t1))) } else { 0.0 } / assign3400_e4033))) * var_t2) - (assign3400_e4036 * var_t2_dn8)) / (var_t2 * var_t2));

        let assign3410_e4041: f64 = (var_weff * var_aechvb);
        let assign3410_e4043: f64 = (assign3410_e4041 * var_toxratioedge);
        var_igsd_mult0 = assign3410_e4043;
        var_igsd_mult0_dn3 = (assign3410_e4041 * var_toxratioedge_dn3);
        var_igsd_mult0_dn4 = (assign3410_e4041 * var_toxratioedge_dn4);
        var_igsd_mult0_dn5 = (assign3410_e4041 * var_toxratioedge_dn5);
        var_igsd_mult0_dn6 = (assign3410_e4041 * var_toxratioedge_dn6);
        var_igsd_mult0_dn7 = (assign3410_e4041 * var_toxratioedge_dn7);
        var_igsd_mult0_dn8 = (assign3410_e4041 * var_toxratioedge_dn8);

        let assign3470_e4084: f64 = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };
        var_guard59 = assign3470_e4084;

        let (assign3480_e4092, assign3480_e4092_d_n4,) = {
    if (var_guard59 != 0.0) {
        let assign3480_e4086: f64 = ctx_temp;
        let assign3480_e4088: f64 = (assign3480_e4086 + (nv4 - 0.0));
        let assign3480_e4090: f64 = (assign3480_e4088 + p.p9);
        (assign3480_e4090, 1.0,)
    } else {
        (var_devtemp, var_devtemp_dn4,)
    }
};
        var_devtemp = assign3480_e4092;
        var_devtemp_dn4 = assign3480_e4092_d_n4;

        let (assign3490_e4099, assign3490_e4099_d_n4,) = {
    if (var_guard59 == 0.0) {
        let assign3490_e4095: f64 = ctx_temp;
        let assign3490_e4097: f64 = (assign3490_e4095 + p.p9);
        (assign3490_e4097, 0.0,)
    } else {
        (var_devtemp, var_devtemp_dn4,)
    }
};
        var_devtemp = assign3490_e4099;
        var_devtemp_dn4 = assign3490_e4099_d_n4;

        let assign3500_e4102: f64 = (p.p298 + 273.15);
        var_tmaxk = assign3500_e4102;

        let assign3520_e4109: f64 = (var_devtemp + var_tmaxk);
        let assign3520_e4112: f64 = (var_devtemp - var_tmaxk);
        let assign3520_e4115: f64 = (var_devtemp - var_tmaxk);
        let assign3520_e4116: f64 = (assign3520_e4112 * assign3520_e4115);
        let assign3520_e4119: f64 = (0.25 * 0.01);
        let assign3520_e4121: f64 = (assign3520_e4119 * 0.01);
        let assign3520_e4122: f64 = (assign3520_e4116 + assign3520_e4121);
        let assign3520_e4123: f64 = (assign3520_e4122).sqrt();
        let assign3520_e4124: f64 = (assign3520_e4109 - assign3520_e4123);
        let assign3520_e4125: f64 = (0.5 * assign3520_e4124);
        var_devtemp = assign3520_e4125;
        var_devtemp_dn4 = (0.5 * (var_devtemp_dn4 - (((var_devtemp_dn4 * assign3520_e4115) + (assign3520_e4112 * var_devtemp_dn4)) / (2.0 * assign3520_e4123))));

        let assign3530_e4128: f64 = (var_devtemp / var_tnom);
        var_tratio = assign3530_e4128;
        var_tratio_dn4 = (var_devtemp_dn4 / var_tnom);

        let assign3540_e4131: f64 = (var_devtemp - var_tnom);
        var_deltemp = assign3540_e4131;
        var_deltemp_dn4 = var_devtemp_dn4;

        let assign3550_e4134: f64 = (8.61708e-5 * var_devtemp);
        var_vtm = assign3550_e4134;
        var_vtm_dn4 = (8.61708e-5 * var_devtemp_dn4);

        let assign3560_e4138: f64 = (p.p299 * var_devtemp);
        let assign3560_e4140: f64 = (assign3560_e4138 * var_devtemp);
        let assign3560_e4143: f64 = (var_devtemp + p.p300);
        let assign3560_e4144: f64 = (assign3560_e4140 / assign3560_e4143);
        let assign3560_e4145: f64 = (p.p55 - assign3560_e4144);
        var_eg = assign3560_e4145;
        var_eg_dn4 = (-((((((p.p299 * var_devtemp_dn4) * var_devtemp) + (assign3560_e4138 * var_devtemp_dn4)) * assign3560_e4143) - (assign3560_e4140 * var_devtemp_dn4)) / (assign3560_e4143 * assign3560_e4143)));

        let __rspice_inv_cse_0: f64 = 1.0 / 300.15;
        let assign3570_e4148: f64 = (var_devtemp * __rspice_inv_cse_0);
        let assign3570_e4151: f64 = (var_devtemp * __rspice_inv_cse_0);
        let assign3570_e4152: f64 = (assign3570_e4151).sqrt();
        let assign3570_e4153: f64 = (assign3570_e4148 * assign3570_e4152);
        var_t1 = assign3570_e4153;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = (((var_devtemp_dn4 / 300.15) * assign3570_e4152) + (assign3570_e4148 * ((var_devtemp_dn4 / 300.15) / (2.0 * assign3570_e4152))));
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;

        let assign3580_e4156: f64 = (p.p54 * var_t1);
        let assign3580_e4160: f64 = (2.0 * 8.61708e-5);
        let assign3580_e4162: f64 = (assign3580_e4160 * 300.15);
        let assign3580_e4163: f64 = (p.p55 / assign3580_e4162);
        let assign3580_e4167: f64 = (2.0 * var_vtm);
        let assign3580_e4168: f64 = (var_eg / assign3580_e4167);
        let assign3580_e4169: f64 = (assign3580_e4163 - assign3580_e4168);
        let assign3580_e4170: f64 = { let limited_exp_arg = assign3580_e4169; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3580_e4171: f64 = (assign3580_e4156 * assign3580_e4170);
        var_ni = assign3580_e4171;
        var_ni_dn3 = ((p.p54 * var_t1_dn3) * assign3580_e4170);
        var_ni_dn4 = (((p.p54 * var_t1_dn4) * assign3580_e4170) + (assign3580_e4156 * ({ let limited_exp_arg = assign3580_e4169; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-(((var_eg_dn4 * assign3580_e4167) - (var_eg * (2.0 * var_vtm_dn4))) / (assign3580_e4167 * assign3580_e4167))))));
        var_ni_dn5 = ((p.p54 * var_t1_dn5) * assign3580_e4170);
        var_ni_dn6 = ((p.p54 * var_t1_dn6) * assign3580_e4170);
        var_ni_dn7 = ((p.p54 * var_t1_dn7) * assign3580_e4170);
        var_ni_dn8 = ((p.p54 * var_t1_dn8) * assign3580_e4170);

        let assign3590_e4175: f64 = (var_nsd_i * var_nbody_i);
        let assign3590_e4178: f64 = (var_ni * var_ni);
        let assign3590_e4179: f64 = (assign3590_e4175 / assign3590_e4178);
        let assign3590_e4181: f64 = (assign3590_e4179).max(1e-38);
        let assign3590_e4182: f64 = (assign3590_e4181).ln();
        let assign3590_e4183: f64 = (var_vtm * assign3590_e4182);
        var_vbi = assign3590_e4183;
        var_vbi_dn3 = (var_vtm * (if assign3590_e4179 >= 1e-38 { (-((assign3590_e4175 * ((var_ni_dn3 * var_ni) + (var_ni * var_ni_dn3))) / (assign3590_e4178 * assign3590_e4178))) } else { 0.0 } / assign3590_e4181));
        var_vbi_dn4 = ((var_vtm_dn4 * assign3590_e4182) + (var_vtm * (if assign3590_e4179 >= 1e-38 { (-((assign3590_e4175 * ((var_ni_dn4 * var_ni) + (var_ni * var_ni_dn4))) / (assign3590_e4178 * assign3590_e4178))) } else { 0.0 } / assign3590_e4181)));
        var_vbi_dn5 = (var_vtm * (if assign3590_e4179 >= 1e-38 { (-((assign3590_e4175 * ((var_ni_dn5 * var_ni) + (var_ni * var_ni_dn5))) / (assign3590_e4178 * assign3590_e4178))) } else { 0.0 } / assign3590_e4181));
        var_vbi_dn6 = (var_vtm * (if assign3590_e4179 >= 1e-38 { (-((assign3590_e4175 * ((var_ni_dn6 * var_ni) + (var_ni * var_ni_dn6))) / (assign3590_e4178 * assign3590_e4178))) } else { 0.0 } / assign3590_e4181));
        var_vbi_dn7 = (var_vtm * (if assign3590_e4179 >= 1e-38 { (-((assign3590_e4175 * ((var_ni_dn7 * var_ni) + (var_ni * var_ni_dn7))) / (assign3590_e4178 * assign3590_e4178))) } else { 0.0 } / assign3590_e4181));
        var_vbi_dn8 = (var_vtm * (if assign3590_e4179 >= 1e-38 { (-((assign3590_e4175 * ((var_ni_dn8 * var_ni) + (var_ni * var_ni_dn8))) / (assign3590_e4178 * assign3590_e4178))) } else { 0.0 } / assign3590_e4181));

        let assign3600_e4187: f64 = (var_nbody_i / var_ni);
        let assign3600_e4189: f64 = (assign3600_e4187).max(1e-38);
        let assign3600_e4190: f64 = (assign3600_e4189).ln();
        let assign3600_e4191: f64 = (var_vtm * assign3600_e4190);
        var_phib = assign3600_e4191;
        var_phib_dn3 = (var_vtm * (if assign3600_e4187 >= 1e-38 { (-((var_nbody_i * var_ni_dn3) / (var_ni * var_ni))) } else { 0.0 } / assign3600_e4189));
        var_phib_dn4 = ((var_vtm_dn4 * assign3600_e4190) + (var_vtm * (if assign3600_e4187 >= 1e-38 { (-((var_nbody_i * var_ni_dn4) / (var_ni * var_ni))) } else { 0.0 } / assign3600_e4189)));
        var_phib_dn5 = (var_vtm * (if assign3600_e4187 >= 1e-38 { (-((var_nbody_i * var_ni_dn5) / (var_ni * var_ni))) } else { 0.0 } / assign3600_e4189));
        var_phib_dn6 = (var_vtm * (if assign3600_e4187 >= 1e-38 { (-((var_nbody_i * var_ni_dn6) / (var_ni * var_ni))) } else { 0.0 } / assign3600_e4189));
        var_phib_dn7 = (var_vtm * (if assign3600_e4187 >= 1e-38 { (-((var_nbody_i * var_ni_dn7) / (var_ni * var_ni))) } else { 0.0 } / assign3600_e4189));
        var_phib_dn8 = (var_vtm * (if assign3600_e4187 >= 1e-38 { (-((var_nbody_i * var_ni_dn8) / (var_ni * var_ni))) } else { 0.0 } / assign3600_e4189));

        let assign3610_e4194: f64 = (0.5 * var_eg);
        let assign3610_e4198: f64 = (0.5 * var_eg);
        let assign3610_e4202: f64 = (p.p52 / var_ni);
        let assign3610_e4204: f64 = (assign3610_e4202).max(1e-38);
        let assign3610_e4205: f64 = (assign3610_e4204).ln();
        let assign3610_e4206: f64 = (var_vtm * assign3610_e4205);
        let assign3610_e4207: f64 = (assign3610_e4198 - assign3610_e4206);
        let assign3610_e4210: f64 = (0.5 * var_eg);
        let assign3610_e4214: f64 = (p.p52 / var_ni);
        let assign3610_e4216: f64 = (assign3610_e4214).max(1e-38);
        let assign3610_e4217: f64 = (assign3610_e4216).ln();
        let assign3610_e4218: f64 = (var_vtm * assign3610_e4217);
        let assign3610_e4219: f64 = (assign3610_e4210 - assign3610_e4218);
        let assign3610_e4222: f64 = (0.5 * var_eg);
        let assign3610_e4226: f64 = (p.p52 / var_ni);
        let assign3610_e4228: f64 = (assign3610_e4226).max(1e-38);
        let assign3610_e4229: f64 = (assign3610_e4228).ln();
        let assign3610_e4230: f64 = (var_vtm * assign3610_e4229);
        let assign3610_e4231: f64 = (assign3610_e4222 - assign3610_e4230);
        let assign3610_e4232: f64 = (assign3610_e4219 * assign3610_e4231);
        let assign3610_e4235: f64 = (4.0 * 0.0001);
        let assign3610_e4237: f64 = (assign3610_e4235 * 0.0001);
        let assign3610_e4238: f64 = (assign3610_e4232 + assign3610_e4237);
        let assign3610_e4239: f64 = (assign3610_e4238).sqrt();
        let assign3610_e4240: f64 = (assign3610_e4207 + assign3610_e4239);
        let assign3610_e4241: f64 = (0.5 * assign3610_e4240);
        let assign3610_e4242: f64 = (assign3610_e4194 - assign3610_e4241);
        var_phisub = assign3610_e4242;
        var_phisub_dn3 = (-(0.5 * ((-(var_vtm * (if assign3610_e4202 >= 1e-38 { (-((p.p52 * var_ni_dn3) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4204))) + ((((-(var_vtm * (if assign3610_e4214 >= 1e-38 { (-((p.p52 * var_ni_dn3) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4216))) * assign3610_e4231) + (assign3610_e4219 * (-(var_vtm * (if assign3610_e4226 >= 1e-38 { (-((p.p52 * var_ni_dn3) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4228))))) / (2.0 * assign3610_e4239)))));
        var_phisub_dn4 = ((0.5 * var_eg_dn4) - (0.5 * (((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign3610_e4205) + (var_vtm * (if assign3610_e4202 >= 1e-38 { (-((p.p52 * var_ni_dn4) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4204)))) + (((((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign3610_e4217) + (var_vtm * (if assign3610_e4214 >= 1e-38 { (-((p.p52 * var_ni_dn4) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4216)))) * assign3610_e4231) + (assign3610_e4219 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign3610_e4229) + (var_vtm * (if assign3610_e4226 >= 1e-38 { (-((p.p52 * var_ni_dn4) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4228)))))) / (2.0 * assign3610_e4239)))));
        var_phisub_dn5 = (-(0.5 * ((-(var_vtm * (if assign3610_e4202 >= 1e-38 { (-((p.p52 * var_ni_dn5) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4204))) + ((((-(var_vtm * (if assign3610_e4214 >= 1e-38 { (-((p.p52 * var_ni_dn5) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4216))) * assign3610_e4231) + (assign3610_e4219 * (-(var_vtm * (if assign3610_e4226 >= 1e-38 { (-((p.p52 * var_ni_dn5) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4228))))) / (2.0 * assign3610_e4239)))));
        var_phisub_dn6 = (-(0.5 * ((-(var_vtm * (if assign3610_e4202 >= 1e-38 { (-((p.p52 * var_ni_dn6) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4204))) + ((((-(var_vtm * (if assign3610_e4214 >= 1e-38 { (-((p.p52 * var_ni_dn6) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4216))) * assign3610_e4231) + (assign3610_e4219 * (-(var_vtm * (if assign3610_e4226 >= 1e-38 { (-((p.p52 * var_ni_dn6) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4228))))) / (2.0 * assign3610_e4239)))));
        var_phisub_dn7 = (-(0.5 * ((-(var_vtm * (if assign3610_e4202 >= 1e-38 { (-((p.p52 * var_ni_dn7) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4204))) + ((((-(var_vtm * (if assign3610_e4214 >= 1e-38 { (-((p.p52 * var_ni_dn7) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4216))) * assign3610_e4231) + (assign3610_e4219 * (-(var_vtm * (if assign3610_e4226 >= 1e-38 { (-((p.p52 * var_ni_dn7) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4228))))) / (2.0 * assign3610_e4239)))));
        var_phisub_dn8 = (-(0.5 * ((-(var_vtm * (if assign3610_e4202 >= 1e-38 { (-((p.p52 * var_ni_dn8) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4204))) + ((((-(var_vtm * (if assign3610_e4214 >= 1e-38 { (-((p.p52 * var_ni_dn8) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4216))) * assign3610_e4231) + (assign3610_e4219 * (-(var_vtm * (if assign3610_e4226 >= 1e-38 { (-((p.p52 * var_ni_dn8) / (var_ni * var_ni))) } else { 0.0 } / assign3610_e4228))))) / (2.0 * assign3610_e4239)))));

        let assign3620_e4249: f64 = if ((p.p52 != 0.0) && (!param_given[58])) { 1.0 } else { 0.0 };
        var_guard61 = assign3620_e4249;

        let assign3630_e4252: f64 = (-1.0);
        let assign3630_e4253: f64 = if p.p13 == assign3630_e4252 { 1.0 } else { 0.0 };
        var_guard62 = assign3630_e4253;

        *var_aechvb_slot = var_aechvb;
        *var_bechvb_slot = var_bechvb;
        *var_cth_slot = var_cth;
        *var_deltemp_slot = var_deltemp;
        *var_deltemp_dn4_slot = var_deltemp_dn4;
        *var_devtemp_slot = var_devtemp;
        *var_devtemp_dn4_slot = var_devtemp_dn4;
        *var_eefffactor_slot = var_eefffactor;
        *var_eefffactor2_slot = var_eefffactor2;
        *var_eg_slot = var_eg;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_eta_mu2_slot = var_eta_mu2;
        *var_eta_mu_cv_slot = var_eta_mu_cv;
        *var_gth_slot = var_gth;
        *var_guard43_slot = var_guard43;
        *var_guard44_slot = var_guard44;
        *var_guard45_slot = var_guard45;
        *var_guard46_slot = var_guard46;
        *var_guard47_slot = var_guard47;
        *var_guard48_slot = var_guard48;
        *var_guard49_slot = var_guard49;
        *var_guard50_slot = var_guard50;
        *var_guard51_slot = var_guard51;
        *var_guard52_slot = var_guard52;
        *var_guard53_slot = var_guard53;
        *var_guard54_slot = var_guard54;
        *var_guard55_slot = var_guard55;
        *var_guard56_slot = var_guard56;
        *var_guard59_slot = var_guard59;
        *var_guard61_slot = var_guard61;
        *var_guard62_slot = var_guard62;
        *var_igsd_mult0_slot = var_igsd_mult0;
        *var_igsd_mult0_dn3_slot = var_igsd_mult0_dn3;
        *var_igsd_mult0_dn4_slot = var_igsd_mult0_dn4;
        *var_igsd_mult0_dn5_slot = var_igsd_mult0_dn5;
        *var_igsd_mult0_dn6_slot = var_igsd_mult0_dn6;
        *var_igsd_mult0_dn7_slot = var_igsd_mult0_dn7;
        *var_igsd_mult0_dn8_slot = var_igsd_mult0_dn8;
        *var_lintnoi_i_slot = var_lintnoi_i;
        *var_litl_slot = var_litl;
        *var_ni_slot = var_ni;
        *var_ni_dn3_slot = var_ni_dn3;
        *var_ni_dn4_slot = var_ni_dn4;
        *var_ni_dn5_slot = var_ni_dn5;
        *var_ni_dn6_slot = var_ni_dn6;
        *var_ni_dn7_slot = var_ni_dn7;
        *var_ni_dn8_slot = var_ni_dn8;
        *var_phib_slot = var_phib;
        *var_phib_dn3_slot = var_phib_dn3;
        *var_phib_dn4_slot = var_phib_dn4;
        *var_phib_dn5_slot = var_phib_dn5;
        *var_phib_dn6_slot = var_phib_dn6;
        *var_phib_dn7_slot = var_phib_dn7;
        *var_phib_dn8_slot = var_phib_dn8;
        *var_phisub_slot = var_phisub;
        *var_phisub_dn3_slot = var_phisub_dn3;
        *var_phisub_dn4_slot = var_phisub_dn4;
        *var_phisub_dn5_slot = var_phisub_dn5;
        *var_phisub_dn6_slot = var_phisub_dn6;
        *var_phisub_dn7_slot = var_phisub_dn7;
        *var_phisub_dn8_slot = var_phisub_dn8;
        *var_rdraingeo_slot = var_rdraingeo;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdswmin_i_slot = var_rdswmin_i;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdwmin_i_slot = var_rdwmin_i;
        *var_rsourcegeo_slot = var_rsourcegeo;
        *var_rsw_i_slot = var_rsw_i;
        *var_rswmin_i_slot = var_rswmin_i;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_tmaxk_slot = var_tmaxk;
        *var_tnom_slot = var_tnom;
        *var_toxratio_slot = var_toxratio;
        *var_toxratio_dn3_slot = var_toxratio_dn3;
        *var_toxratio_dn4_slot = var_toxratio_dn4;
        *var_toxratio_dn5_slot = var_toxratio_dn5;
        *var_toxratio_dn6_slot = var_toxratio_dn6;
        *var_toxratio_dn7_slot = var_toxratio_dn7;
        *var_toxratio_dn8_slot = var_toxratio_dn8;
        *var_toxratioedge_slot = var_toxratioedge;
        *var_toxratioedge_dn3_slot = var_toxratioedge_dn3;
        *var_toxratioedge_dn4_slot = var_toxratioedge_dn4;
        *var_toxratioedge_dn5_slot = var_toxratioedge_dn5;
        *var_toxratioedge_dn6_slot = var_toxratioedge_dn6;
        *var_toxratioedge_dn7_slot = var_toxratioedge_dn7;
        *var_toxratioedge_dn8_slot = var_toxratioedge_dn8;
        *var_tratio_slot = var_tratio;
        *var_tratio_dn4_slot = var_tratio_dn4;
        *var_vbi_slot = var_vbi;
        *var_vbi_dn3_slot = var_vbi_dn3;
        *var_vbi_dn4_slot = var_vbi_dn4;
        *var_vbi_dn5_slot = var_vbi_dn5;
        *var_vbi_dn6_slot = var_vbi_dn6;
        *var_vbi_dn7_slot = var_vbi_dn7;
        *var_vbi_dn8_slot = var_vbi_dn8;
        *var_vtm_slot = var_vtm;
        *var_vtm_dn4_slot = var_vtm_dn4;
        *var_weffwrfactor_slot = var_weffwrfactor;
    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        var_beta0_i: f64,
        var_bgidl_i: f64,
        var_bgisl_i: f64,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_devsign: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_eta0_i: f64,
        var_guard61: f64,
        var_guard62: f64,
        var_igt_i: f64,
        var_iit_i: f64,
        var_inv_l: f64,
        var_k01_i: f64,
        var_k0_i: f64,
        var_k0si1_i: f64,
        var_k0si_i: f64,
        var_k0sisat1_i: f64,
        var_k0sisat_i: f64,
        var_leff: f64,
        var_mexp_i: f64,
        var_ni: f64,
        var_ni_dn3: f64,
        var_ni_dn4: f64,
        var_ni_dn5: f64,
        var_ni_dn6: f64,
        var_ni_dn7: f64,
        var_ni_dn8: f64,
        var_nsd_i: f64,
        var_phig1_i: f64,
        var_phisub: f64,
        var_phisub_dn3: f64,
        var_phisub_dn4: f64,
        var_phisub_dn5: f64,
        var_phisub_dn6: f64,
        var_phisub_dn7: f64,
        var_phisub_dn8: f64,
        var_prt_i: f64,
        var_ptwg_i: f64,
        var_ptwgt_i: f64,
        var_tgidl_i: f64,
        var_tgisl_i: f64,
        var_tratio: f64,
        var_tratio_dn4: f64,
        var_u0_i: f64,
        var_ua1_i: f64,
        var_ua_i: f64,
        var_uc_i: f64,
        var_ucs_i: f64,
        var_ucste_i: f64,
        var_ud1_i: f64,
        var_ud_i: f64,
        var_ute_i: f64,
        var_utl_i: f64,
        var_vsat1_i: f64,
        var_vsat_i: f64,
        var_vsatb_i: f64,
        var_vsatcv_i: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_at_i_slot: &mut f64,
        var_atb_i_slot: &mut f64,
        var_beta0_t_slot: &mut f64,
        var_beta0_t_dn4_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidl_t_dn4_slot: &mut f64,
        var_bgisl_t_slot: &mut f64,
        var_bgisl_t_dn4_slot: &mut f64,
        var_deltaphi1_slot: &mut f64,
        var_deltaphi1_dn4_slot: &mut f64,
        var_deltaphi2_slot: &mut f64,
        var_deltaphi2_dn3_slot: &mut f64,
        var_deltaphi2_dn4_slot: &mut f64,
        var_deltaphi2_dn5_slot: &mut f64,
        var_deltaphi2_dn6_slot: &mut f64,
        var_deltaphi2_dn7_slot: &mut f64,
        var_deltaphi2_dn8_slot: &mut f64,
        var_dvth_temp0_slot: &mut f64,
        var_dvth_temp0_dn4_slot: &mut f64,
        var_eta0_t_slot: &mut f64,
        var_eta0_t_dn4_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_guard65_slot: &mut f64,
        var_igtemp_slot: &mut f64,
        var_igtemp_dn4_slot: &mut f64,
        var_k0_t_slot: &mut f64,
        var_k0_t_dn4_slot: &mut f64,
        var_k0si_t_slot: &mut f64,
        var_k0si_t_dn4_slot: &mut f64,
        var_k0sisat_t_slot: &mut f64,
        var_k0sisat_t_dn4_slot: &mut f64,
        var_mexp_t_slot: &mut f64,
        var_mexp_t_dn4_slot: &mut f64,
        var_phig2_i_slot: &mut f64,
        var_phig2_i_dn3_slot: &mut f64,
        var_phig2_i_dn4_slot: &mut f64,
        var_phig2_i_dn5_slot: &mut f64,
        var_phig2_i_dn6_slot: &mut f64,
        var_phig2_i_dn7_slot: &mut f64,
        var_phig2_i_dn8_slot: &mut f64,
        var_phiref_slot: &mut f64,
        var_phiref_dn4_slot: &mut f64,
        var_phisd_slot: &mut f64,
        var_phisd_dn3_slot: &mut f64,
        var_phisd_dn4_slot: &mut f64,
        var_phisd_dn5_slot: &mut f64,
        var_phisd_dn6_slot: &mut f64,
        var_phisd_dn7_slot: &mut f64,
        var_phisd_dn8_slot: &mut f64,
        var_ptwg_t_slot: &mut f64,
        var_ptwg_t_dn4_slot: &mut f64,
        var_rdstemp_slot: &mut f64,
        var_rdstemp_dn4_slot: &mut f64,
        var_u0_t_slot: &mut f64,
        var_u0_t_dn4_slot: &mut f64,
        var_ua_t_slot: &mut f64,
        var_ua_t_dn4_slot: &mut f64,
        var_uc_t_slot: &mut f64,
        var_uc_t_dn4_slot: &mut f64,
        var_ucs_t_slot: &mut f64,
        var_ucs_t_dn4_slot: &mut f64,
        var_ud_t_slot: &mut f64,
        var_ud_t_dn4_slot: &mut f64,
        var_vfbsd_slot: &mut f64,
        var_vfbsd_bg_slot: &mut f64,
        var_vfbsd_bg_dn3_slot: &mut f64,
        var_vfbsd_bg_dn4_slot: &mut f64,
        var_vfbsd_bg_dn5_slot: &mut f64,
        var_vfbsd_bg_dn6_slot: &mut f64,
        var_vfbsd_bg_dn7_slot: &mut f64,
        var_vfbsd_bg_dn8_slot: &mut f64,
        var_vfbsd_dn3_slot: &mut f64,
        var_vfbsd_dn4_slot: &mut f64,
        var_vfbsd_dn5_slot: &mut f64,
        var_vfbsd_dn6_slot: &mut f64,
        var_vfbsd_dn7_slot: &mut f64,
        var_vfbsd_dn8_slot: &mut f64,
        var_vsat1_t_slot: &mut f64,
        var_vsat1_t_dn4_slot: &mut f64,
        var_vsat_t_slot: &mut f64,
        var_vsat_t_dn4_slot: &mut f64,
        var_vsatb_t_slot: &mut f64,
        var_vsatb_t_dn4_slot: &mut f64,
        var_vsatcv_t_slot: &mut f64,
        var_vsatcv_t_dn4_slot: &mut f64,
    ) {
        let mut var_at_i: f64 = *var_at_i_slot;
        let mut var_atb_i: f64 = *var_atb_i_slot;
        let mut var_beta0_t: f64 = *var_beta0_t_slot;
        let mut var_beta0_t_dn4: f64 = *var_beta0_t_dn4_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidl_t_dn4: f64 = *var_bgidl_t_dn4_slot;
        let mut var_bgisl_t: f64 = *var_bgisl_t_slot;
        let mut var_bgisl_t_dn4: f64 = *var_bgisl_t_dn4_slot;
        let mut var_deltaphi1: f64 = *var_deltaphi1_slot;
        let mut var_deltaphi1_dn4: f64 = *var_deltaphi1_dn4_slot;
        let mut var_deltaphi2: f64 = *var_deltaphi2_slot;
        let mut var_deltaphi2_dn3: f64 = *var_deltaphi2_dn3_slot;
        let mut var_deltaphi2_dn4: f64 = *var_deltaphi2_dn4_slot;
        let mut var_deltaphi2_dn5: f64 = *var_deltaphi2_dn5_slot;
        let mut var_deltaphi2_dn6: f64 = *var_deltaphi2_dn6_slot;
        let mut var_deltaphi2_dn7: f64 = *var_deltaphi2_dn7_slot;
        let mut var_deltaphi2_dn8: f64 = *var_deltaphi2_dn8_slot;
        let mut var_dvth_temp0: f64 = *var_dvth_temp0_slot;
        let mut var_dvth_temp0_dn4: f64 = *var_dvth_temp0_dn4_slot;
        let mut var_eta0_t: f64 = *var_eta0_t_slot;
        let mut var_eta0_t_dn4: f64 = *var_eta0_t_dn4_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_guard65: f64 = *var_guard65_slot;
        let mut var_igtemp: f64 = *var_igtemp_slot;
        let mut var_igtemp_dn4: f64 = *var_igtemp_dn4_slot;
        let mut var_k0_t: f64 = *var_k0_t_slot;
        let mut var_k0_t_dn4: f64 = *var_k0_t_dn4_slot;
        let mut var_k0si_t: f64 = *var_k0si_t_slot;
        let mut var_k0si_t_dn4: f64 = *var_k0si_t_dn4_slot;
        let mut var_k0sisat_t: f64 = *var_k0sisat_t_slot;
        let mut var_k0sisat_t_dn4: f64 = *var_k0sisat_t_dn4_slot;
        let mut var_mexp_t: f64 = *var_mexp_t_slot;
        let mut var_mexp_t_dn4: f64 = *var_mexp_t_dn4_slot;
        let mut var_phig2_i: f64 = *var_phig2_i_slot;
        let mut var_phig2_i_dn3: f64 = *var_phig2_i_dn3_slot;
        let mut var_phig2_i_dn4: f64 = *var_phig2_i_dn4_slot;
        let mut var_phig2_i_dn5: f64 = *var_phig2_i_dn5_slot;
        let mut var_phig2_i_dn6: f64 = *var_phig2_i_dn6_slot;
        let mut var_phig2_i_dn7: f64 = *var_phig2_i_dn7_slot;
        let mut var_phig2_i_dn8: f64 = *var_phig2_i_dn8_slot;
        let mut var_phiref: f64 = *var_phiref_slot;
        let mut var_phiref_dn4: f64 = *var_phiref_dn4_slot;
        let mut var_phisd: f64 = *var_phisd_slot;
        let mut var_phisd_dn3: f64 = *var_phisd_dn3_slot;
        let mut var_phisd_dn4: f64 = *var_phisd_dn4_slot;
        let mut var_phisd_dn5: f64 = *var_phisd_dn5_slot;
        let mut var_phisd_dn6: f64 = *var_phisd_dn6_slot;
        let mut var_phisd_dn7: f64 = *var_phisd_dn7_slot;
        let mut var_phisd_dn8: f64 = *var_phisd_dn8_slot;
        let mut var_ptwg_t: f64 = *var_ptwg_t_slot;
        let mut var_ptwg_t_dn4: f64 = *var_ptwg_t_dn4_slot;
        let mut var_rdstemp: f64 = *var_rdstemp_slot;
        let mut var_rdstemp_dn4: f64 = *var_rdstemp_dn4_slot;
        let mut var_u0_t: f64 = *var_u0_t_slot;
        let mut var_u0_t_dn4: f64 = *var_u0_t_dn4_slot;
        let mut var_ua_t: f64 = *var_ua_t_slot;
        let mut var_ua_t_dn4: f64 = *var_ua_t_dn4_slot;
        let mut var_uc_t: f64 = *var_uc_t_slot;
        let mut var_uc_t_dn4: f64 = *var_uc_t_dn4_slot;
        let mut var_ucs_t: f64 = *var_ucs_t_slot;
        let mut var_ucs_t_dn4: f64 = *var_ucs_t_dn4_slot;
        let mut var_ud_t: f64 = *var_ud_t_slot;
        let mut var_ud_t_dn4: f64 = *var_ud_t_dn4_slot;
        let mut var_vfbsd: f64 = *var_vfbsd_slot;
        let mut var_vfbsd_bg: f64 = *var_vfbsd_bg_slot;
        let mut var_vfbsd_bg_dn3: f64 = *var_vfbsd_bg_dn3_slot;
        let mut var_vfbsd_bg_dn4: f64 = *var_vfbsd_bg_dn4_slot;
        let mut var_vfbsd_bg_dn5: f64 = *var_vfbsd_bg_dn5_slot;
        let mut var_vfbsd_bg_dn6: f64 = *var_vfbsd_bg_dn6_slot;
        let mut var_vfbsd_bg_dn7: f64 = *var_vfbsd_bg_dn7_slot;
        let mut var_vfbsd_bg_dn8: f64 = *var_vfbsd_bg_dn8_slot;
        let mut var_vfbsd_dn3: f64 = *var_vfbsd_dn3_slot;
        let mut var_vfbsd_dn4: f64 = *var_vfbsd_dn4_slot;
        let mut var_vfbsd_dn5: f64 = *var_vfbsd_dn5_slot;
        let mut var_vfbsd_dn6: f64 = *var_vfbsd_dn6_slot;
        let mut var_vfbsd_dn7: f64 = *var_vfbsd_dn7_slot;
        let mut var_vfbsd_dn8: f64 = *var_vfbsd_dn8_slot;
        let mut var_vsat1_t: f64 = *var_vsat1_t_slot;
        let mut var_vsat1_t_dn4: f64 = *var_vsat1_t_dn4_slot;
        let mut var_vsat_t: f64 = *var_vsat_t_slot;
        let mut var_vsat_t_dn4: f64 = *var_vsat_t_dn4_slot;
        let mut var_vsatb_t: f64 = *var_vsatb_t_slot;
        let mut var_vsatb_t_dn4: f64 = *var_vsatb_t_dn4_slot;
        let mut var_vsatcv_t: f64 = *var_vsatcv_t_slot;
        let mut var_vsatcv_t_dn4: f64 = *var_vsatcv_t_dn4_slot;

        let (assign3640_e4265, assign3640_e4265_d_n3, assign3640_e4265_d_n4, assign3640_e4265_d_n5, assign3640_e4265_d_n6, assign3640_e4265_d_n7, assign3640_e4265_d_n8,) = {
    if ((var_guard61 != 0.0) && (var_guard62 != 0.0)) {
        let assign3640_e4260: f64 = (0.5 * p.p55);
        let assign3640_e4261: f64 = (var_phig2_i - assign3640_e4260);
        let assign3640_e4263: f64 = (assign3640_e4261 + var_phisub);
        (assign3640_e4263, (var_phig2_i_dn3 + var_phisub_dn3), (var_phig2_i_dn4 + var_phisub_dn4), (var_phig2_i_dn5 + var_phisub_dn5), (var_phig2_i_dn6 + var_phisub_dn6), (var_phig2_i_dn7 + var_phisub_dn7), (var_phig2_i_dn8 + var_phisub_dn8),)
    } else {
        (var_phig2_i, var_phig2_i_dn3, var_phig2_i_dn4, var_phig2_i_dn5, var_phig2_i_dn6, var_phig2_i_dn7, var_phig2_i_dn8,)
    }
};
        var_phig2_i = assign3640_e4265;
        var_phig2_i_dn3 = assign3640_e4265_d_n3;
        var_phig2_i_dn4 = assign3640_e4265_d_n4;
        var_phig2_i_dn5 = assign3640_e4265_d_n5;
        var_phig2_i_dn6 = assign3640_e4265_d_n6;
        var_phig2_i_dn7 = assign3640_e4265_d_n7;
        var_phig2_i_dn8 = assign3640_e4265_d_n8;

        let (assign3650_e4278, assign3650_e4278_d_n3, assign3650_e4278_d_n4, assign3650_e4278_d_n5, assign3650_e4278_d_n6, assign3650_e4278_d_n7, assign3650_e4278_d_n8,) = {
    if ((var_guard61 != 0.0) && (var_guard62 == 0.0)) {
        let assign3650_e4273: f64 = (0.5 * p.p55);
        let assign3650_e4274: f64 = (var_phig2_i + assign3650_e4273);
        let assign3650_e4276: f64 = (assign3650_e4274 - var_phisub);
        (assign3650_e4276, (var_phig2_i_dn3 - var_phisub_dn3), (var_phig2_i_dn4 - var_phisub_dn4), (var_phig2_i_dn5 - var_phisub_dn5), (var_phig2_i_dn6 - var_phisub_dn6), (var_phig2_i_dn7 - var_phisub_dn7), (var_phig2_i_dn8 - var_phisub_dn8),)
    } else {
        (var_phig2_i, var_phig2_i_dn3, var_phig2_i_dn4, var_phig2_i_dn5, var_phig2_i_dn6, var_phig2_i_dn7, var_phig2_i_dn8,)
    }
};
        var_phig2_i = assign3650_e4278;
        var_phig2_i_dn3 = assign3650_e4278_d_n3;
        var_phig2_i_dn4 = assign3650_e4278_d_n4;
        var_phig2_i_dn5 = assign3650_e4278_d_n5;
        var_phig2_i_dn6 = assign3650_e4278_d_n6;
        var_phig2_i_dn7 = assign3650_e4278_d_n7;
        var_phig2_i_dn8 = assign3650_e4278_d_n8;

        let assign3660_e4282: f64 = (var_eg / 2.0);
        let assign3660_e4283: f64 = (p.p53 + assign3660_e4282);
        var_phiref = assign3660_e4283;
        var_phiref_dn4 = (var_eg_dn4 / 2.0);

        let assign3670_e4287: f64 = (var_phig1_i - var_phiref);
        let assign3670_e4288: f64 = (var_devsign * assign3670_e4287);
        var_deltaphi1 = assign3670_e4288;
        var_deltaphi1_dn4 = (var_devsign * (-var_phiref_dn4));

        let assign3680_e4292: f64 = (var_phig2_i - var_phiref);
        let assign3680_e4293: f64 = (var_devsign * assign3680_e4292);
        var_deltaphi2 = assign3680_e4293;
        var_deltaphi2_dn3 = (var_devsign * var_phig2_i_dn3);
        var_deltaphi2_dn4 = (var_devsign * (var_phig2_i_dn4 - var_phiref_dn4));
        var_deltaphi2_dn5 = (var_devsign * var_phig2_i_dn5);
        var_deltaphi2_dn6 = (var_devsign * var_phig2_i_dn6);
        var_deltaphi2_dn7 = (var_devsign * var_phig2_i_dn7);
        var_deltaphi2_dn8 = (var_devsign * var_phig2_i_dn8);

        let assign3690_e4297: f64 = (var_eg / 2.0);
        let assign3690_e4298: f64 = (p.p53 + assign3690_e4297);
        let assign3690_e4302: f64 = (var_eg / 2.0);
        let assign3690_e4306: f64 = (var_nsd_i / var_ni);
        let assign3690_e4308: f64 = (assign3690_e4306).max(1e-38);
        let assign3690_e4309: f64 = (assign3690_e4308).ln();
        let assign3690_e4310: f64 = (var_vtm * assign3690_e4309);
        let assign3690_e4311: f64 = (assign3690_e4302).min(assign3690_e4310);
        let assign3690_e4312: f64 = (var_devsign * assign3690_e4311);
        let assign3690_e4313: f64 = (assign3690_e4298 - assign3690_e4312);
        var_phisd = assign3690_e4313;
        var_phisd_dn3 = (-(var_devsign * if assign3690_e4302 <= assign3690_e4310 { 0.0 } else { (var_vtm * (if assign3690_e4306 >= 1e-38 { (-((var_nsd_i * var_ni_dn3) / (var_ni * var_ni))) } else { 0.0 } / assign3690_e4308)) }));
        var_phisd_dn4 = ((var_eg_dn4 / 2.0) - (var_devsign * if assign3690_e4302 <= assign3690_e4310 { (var_eg_dn4 / 2.0) } else { ((var_vtm_dn4 * assign3690_e4309) + (var_vtm * (if assign3690_e4306 >= 1e-38 { (-((var_nsd_i * var_ni_dn4) / (var_ni * var_ni))) } else { 0.0 } / assign3690_e4308))) }));
        var_phisd_dn5 = (-(var_devsign * if assign3690_e4302 <= assign3690_e4310 { 0.0 } else { (var_vtm * (if assign3690_e4306 >= 1e-38 { (-((var_nsd_i * var_ni_dn5) / (var_ni * var_ni))) } else { 0.0 } / assign3690_e4308)) }));
        var_phisd_dn6 = (-(var_devsign * if assign3690_e4302 <= assign3690_e4310 { 0.0 } else { (var_vtm * (if assign3690_e4306 >= 1e-38 { (-((var_nsd_i * var_ni_dn6) / (var_ni * var_ni))) } else { 0.0 } / assign3690_e4308)) }));
        var_phisd_dn7 = (-(var_devsign * if assign3690_e4302 <= assign3690_e4310 { 0.0 } else { (var_vtm * (if assign3690_e4306 >= 1e-38 { (-((var_nsd_i * var_ni_dn7) / (var_ni * var_ni))) } else { 0.0 } / assign3690_e4308)) }));
        var_phisd_dn8 = (-(var_devsign * if assign3690_e4302 <= assign3690_e4310 { 0.0 } else { (var_vtm * (if assign3690_e4306 >= 1e-38 { (-((var_nsd_i * var_ni_dn8) / (var_ni * var_ni))) } else { 0.0 } / assign3690_e4308)) }));

        let assign3700_e4317: f64 = (var_phig1_i - var_phisd);
        let assign3700_e4318: f64 = (var_devsign * assign3700_e4317);
        var_vfbsd = assign3700_e4318;
        var_vfbsd_dn3 = (var_devsign * (-var_phisd_dn3));
        var_vfbsd_dn4 = (var_devsign * (-var_phisd_dn4));
        var_vfbsd_dn5 = (var_devsign * (-var_phisd_dn5));
        var_vfbsd_dn6 = (var_devsign * (-var_phisd_dn6));
        var_vfbsd_dn7 = (var_devsign * (-var_phisd_dn7));
        var_vfbsd_dn8 = (var_devsign * (-var_phisd_dn8));

        let assign3710_e4322: f64 = (var_phig2_i - var_phisd);
        let assign3710_e4323: f64 = (var_devsign * assign3710_e4322);
        var_vfbsd_bg = assign3710_e4323;
        var_vfbsd_bg_dn3 = (var_devsign * (var_phig2_i_dn3 - var_phisd_dn3));
        var_vfbsd_bg_dn4 = (var_devsign * (var_phig2_i_dn4 - var_phisd_dn4));
        var_vfbsd_bg_dn5 = (var_devsign * (var_phig2_i_dn5 - var_phisd_dn5));
        var_vfbsd_bg_dn6 = (var_devsign * (var_phig2_i_dn6 - var_phisd_dn6));
        var_vfbsd_bg_dn7 = (var_devsign * (var_phig2_i_dn7 - var_phisd_dn7));
        var_vfbsd_bg_dn8 = (var_devsign * (var_phig2_i_dn8 - var_phisd_dn8));

        let assign3720_e4327: f64 = (var_tratio).powf(var_ute_i);
        let assign3720_e4328: f64 = (var_u0_i * assign3720_e4327);
        let assign3720_e4334: f64 = (var_utl_i * var_deltemp);
        let assign3720_e4335: f64 = (0.9 + assign3720_e4334);
        let assign3720_e4339: f64 = (var_utl_i * var_deltemp);
        let assign3720_e4340: f64 = (0.9 + assign3720_e4339);
        let assign3720_e4344: f64 = (var_utl_i * var_deltemp);
        let assign3720_e4345: f64 = (0.9 + assign3720_e4344);
        let assign3720_e4346: f64 = (assign3720_e4340 * assign3720_e4345);
        let assign3720_e4349: f64 = (4.0 * 0.001);
        let assign3720_e4351: f64 = (assign3720_e4349 * 0.001);
        let assign3720_e4352: f64 = (assign3720_e4346 + assign3720_e4351);
        let assign3720_e4353: f64 = (assign3720_e4352).sqrt();
        let assign3720_e4354: f64 = (assign3720_e4335 + assign3720_e4353);
        let assign3720_e4355: f64 = (0.5 * assign3720_e4354);
        let assign3720_e4356: f64 = (1.0 + assign3720_e4355);
        let assign3720_e4361: f64 = (0.9 * 0.9);
        let assign3720_e4364: f64 = (4.0 * 0.001);
        let assign3720_e4366: f64 = (assign3720_e4364 * 0.001);
        let assign3720_e4367: f64 = (assign3720_e4361 + assign3720_e4366);
        let assign3720_e4368: f64 = (assign3720_e4367).sqrt();
        let assign3720_e4369: f64 = (0.9 + assign3720_e4368);
        let assign3720_e4370: f64 = (0.5 * assign3720_e4369);
        let assign3720_e4371: f64 = (assign3720_e4356 - assign3720_e4370);
        let assign3720_e4372: f64 = (assign3720_e4328 * assign3720_e4371);
        var_u0_t = assign3720_e4372;
        var_u0_t_dn4 = (((var_u0_i * if 0.0 == 0.0 && ((var_ute_i) as f64).is_finite() && ((var_ute_i) as f64).fract() == 0.0 { if var_ute_i == 0.0 { 0.0 } else { (var_ute_i * ((var_tratio).powf(var_ute_i - 1.0) * var_tratio_dn4)) } } else { (assign3720_e4327 * (var_ute_i * (var_tratio_dn4 / var_tratio))) }) * assign3720_e4371) + (assign3720_e4328 * (0.5 * ((var_utl_i * var_deltemp_dn4) + ((((var_utl_i * var_deltemp_dn4) * assign3720_e4345) + (assign3720_e4340 * (var_utl_i * var_deltemp_dn4))) / (2.0 * assign3720_e4353))))));

        let assign3730_e4378: f64 = (p.p159 * var_deltemp);
        let assign3730_e4379: f64 = (1.0 + assign3730_e4378);
        let assign3730_e4381: f64 = (assign3730_e4379 - 1e-6);
        let assign3730_e4385: f64 = (p.p159 * var_deltemp);
        let assign3730_e4386: f64 = (1.0 + assign3730_e4385);
        let assign3730_e4388: f64 = (assign3730_e4386 - 1e-6);
        let assign3730_e4392: f64 = (p.p159 * var_deltemp);
        let assign3730_e4393: f64 = (1.0 + assign3730_e4392);
        let assign3730_e4395: f64 = (assign3730_e4393 - 1e-6);
        let assign3730_e4396: f64 = (assign3730_e4388 * assign3730_e4395);
        let assign3730_e4399: f64 = (4.0 * 0.001);
        let assign3730_e4401: f64 = (assign3730_e4399 * 0.001);
        let assign3730_e4402: f64 = (assign3730_e4396 + assign3730_e4401);
        let assign3730_e4403: f64 = (assign3730_e4402).sqrt();
        let assign3730_e4404: f64 = (assign3730_e4381 + assign3730_e4403);
        let assign3730_e4405: f64 = (0.5 * assign3730_e4404);
        let assign3730_e4406: f64 = (var_uc_i * assign3730_e4405);
        var_uc_t = assign3730_e4406;
        var_uc_t_dn4 = (var_uc_i * (0.5 * ((p.p159 * var_deltemp_dn4) + ((((p.p159 * var_deltemp_dn4) * assign3730_e4395) + (assign3730_e4388 * (p.p159 * var_deltemp_dn4))) / (2.0 * assign3730_e4403)))));

        let assign3740_e4412: f64 = (var_ua1_i * var_deltemp);
        let assign3740_e4413: f64 = (1.0 + assign3740_e4412);
        let assign3740_e4415: f64 = (assign3740_e4413 - 1e-6);
        let assign3740_e4419: f64 = (var_ua1_i * var_deltemp);
        let assign3740_e4420: f64 = (1.0 + assign3740_e4419);
        let assign3740_e4422: f64 = (assign3740_e4420 - 1e-6);
        let assign3740_e4426: f64 = (var_ua1_i * var_deltemp);
        let assign3740_e4427: f64 = (1.0 + assign3740_e4426);
        let assign3740_e4429: f64 = (assign3740_e4427 - 1e-6);
        let assign3740_e4430: f64 = (assign3740_e4422 * assign3740_e4429);
        let assign3740_e4433: f64 = (4.0 * 0.001);
        let assign3740_e4435: f64 = (assign3740_e4433 * 0.001);
        let assign3740_e4436: f64 = (assign3740_e4430 + assign3740_e4435);
        let assign3740_e4437: f64 = (assign3740_e4436).sqrt();
        let assign3740_e4438: f64 = (assign3740_e4415 + assign3740_e4437);
        let assign3740_e4439: f64 = (0.5 * assign3740_e4438);
        let assign3740_e4440: f64 = (var_ua_i * assign3740_e4439);
        var_ua_t = assign3740_e4440;
        var_ua_t_dn4 = (var_ua_i * (0.5 * ((var_ua1_i * var_deltemp_dn4) + ((((var_ua1_i * var_deltemp_dn4) * assign3740_e4429) + (assign3740_e4422 * (var_ua1_i * var_deltemp_dn4))) / (2.0 * assign3740_e4437)))));

        let assign3750_e4444: f64 = (var_tratio).powf(var_ud1_i);
        let assign3750_e4445: f64 = (var_ud_i * assign3750_e4444);
        var_ud_t = assign3750_e4445;
        var_ud_t_dn4 = (var_ud_i * if 0.0 == 0.0 && ((var_ud1_i) as f64).is_finite() && ((var_ud1_i) as f64).fract() == 0.0 { if var_ud1_i == 0.0 { 0.0 } else { (var_ud1_i * ((var_tratio).powf(var_ud1_i - 1.0) * var_tratio_dn4)) } } else { (assign3750_e4444 * (var_ud1_i * (var_tratio_dn4 / var_tratio))) });

        let assign3760_e4449: f64 = (var_tratio).powf(var_ucste_i);
        let assign3760_e4450: f64 = (var_ucs_i * assign3760_e4449);
        var_ucs_t = assign3760_e4450;
        var_ucs_t_dn4 = (var_ucs_i * if 0.0 == 0.0 && ((var_ucste_i) as f64).is_finite() && ((var_ucste_i) as f64).fract() == 0.0 { if var_ucste_i == 0.0 { 0.0 } else { (var_ucste_i * ((var_tratio).powf(var_ucste_i - 1.0) * var_tratio_dn4)) } } else { (assign3760_e4449 * (var_ucste_i * (var_tratio_dn4 / var_tratio))) });

        let assign3770_e4455: f64 = (var_prt_i * var_deltemp);
        let assign3770_e4456: f64 = (1.0 + assign3770_e4455);
        let assign3770_e4458: f64 = (assign3770_e4456 - 1e-6);
        let assign3770_e4462: f64 = (var_prt_i * var_deltemp);
        let assign3770_e4463: f64 = (1.0 + assign3770_e4462);
        let assign3770_e4465: f64 = (assign3770_e4463 - 1e-6);
        let assign3770_e4469: f64 = (var_prt_i * var_deltemp);
        let assign3770_e4470: f64 = (1.0 + assign3770_e4469);
        let assign3770_e4472: f64 = (assign3770_e4470 - 1e-6);
        let assign3770_e4473: f64 = (assign3770_e4465 * assign3770_e4472);
        let assign3770_e4476: f64 = (4.0 * 0.001);
        let assign3770_e4478: f64 = (assign3770_e4476 * 0.001);
        let assign3770_e4479: f64 = (assign3770_e4473 + assign3770_e4478);
        let assign3770_e4480: f64 = (assign3770_e4479).sqrt();
        let assign3770_e4481: f64 = (assign3770_e4458 + assign3770_e4480);
        let assign3770_e4482: f64 = (0.5 * assign3770_e4481);
        var_rdstemp = assign3770_e4482;
        var_rdstemp_dn4 = (0.5 * ((var_prt_i * var_deltemp_dn4) + ((((var_prt_i * var_deltemp_dn4) * assign3770_e4472) + (assign3770_e4465 * (var_prt_i * var_deltemp_dn4))) / (2.0 * assign3770_e4480))));

        let assign3780_e4487: f64 = (var_inv_l * p.p120);
        let assign3780_e4488: f64 = (1.0 + assign3780_e4487);
        let assign3780_e4489: f64 = (var_at_i * assign3780_e4488);
        var_at_i = assign3780_e4489;

        let assign3790_e4496: f64 = (var_at_i * var_deltemp);
        let assign3790_e4497: f64 = (0.9 - assign3790_e4496);
        let assign3790_e4501: f64 = (var_at_i * var_deltemp);
        let assign3790_e4502: f64 = (0.9 - assign3790_e4501);
        let assign3790_e4506: f64 = (var_at_i * var_deltemp);
        let assign3790_e4507: f64 = (0.9 - assign3790_e4506);
        let assign3790_e4508: f64 = (assign3790_e4502 * assign3790_e4507);
        let assign3790_e4511: f64 = (4.0 * 0.001);
        let assign3790_e4513: f64 = (assign3790_e4511 * 0.001);
        let assign3790_e4514: f64 = (assign3790_e4508 + assign3790_e4513);
        let assign3790_e4515: f64 = (assign3790_e4514).sqrt();
        let assign3790_e4516: f64 = (assign3790_e4497 + assign3790_e4515);
        let assign3790_e4517: f64 = (0.5 * assign3790_e4516);
        let assign3790_e4518: f64 = (1.0 + assign3790_e4517);
        let assign3790_e4523: f64 = (0.9 * 0.9);
        let assign3790_e4526: f64 = (4.0 * 0.001);
        let assign3790_e4528: f64 = (assign3790_e4526 * 0.001);
        let assign3790_e4529: f64 = (assign3790_e4523 + assign3790_e4528);
        let assign3790_e4530: f64 = (assign3790_e4529).sqrt();
        let assign3790_e4531: f64 = (0.9 + assign3790_e4530);
        let assign3790_e4532: f64 = (0.5 * assign3790_e4531);
        let assign3790_e4533: f64 = (assign3790_e4518 - assign3790_e4532);
        let assign3790_e4534: f64 = (var_vsat_i * assign3790_e4533);
        var_vsat_t = assign3790_e4534;
        var_vsat_t_dn4 = (var_vsat_i * (0.5 * ((-(var_at_i * var_deltemp_dn4)) + ((((-(var_at_i * var_deltemp_dn4)) * assign3790_e4507) + (assign3790_e4502 * (-(var_at_i * var_deltemp_dn4)))) / (2.0 * assign3790_e4515)))));

        let assign3800_e4537: f64 = if var_vsat_t < 1000.0 { 1.0 } else { 0.0 };
        var_guard63 = assign3800_e4537;

        let (assign3810_e4541, assign3810_e4541_d_n4,) = {
    if (var_guard63 != 0.0) {
        (1000.0, 0.0,)
    } else {
        (var_vsat_t, var_vsat_t_dn4,)
    }
};
        var_vsat_t = assign3810_e4541;
        var_vsat_t_dn4 = assign3810_e4541_d_n4;

        let assign3820_e4548: f64 = (var_at_i * var_deltemp);
        let assign3820_e4549: f64 = (0.9 - assign3820_e4548);
        let assign3820_e4553: f64 = (var_at_i * var_deltemp);
        let assign3820_e4554: f64 = (0.9 - assign3820_e4553);
        let assign3820_e4558: f64 = (var_at_i * var_deltemp);
        let assign3820_e4559: f64 = (0.9 - assign3820_e4558);
        let assign3820_e4560: f64 = (assign3820_e4554 * assign3820_e4559);
        let assign3820_e4563: f64 = (4.0 * 0.001);
        let assign3820_e4565: f64 = (assign3820_e4563 * 0.001);
        let assign3820_e4566: f64 = (assign3820_e4560 + assign3820_e4565);
        let assign3820_e4567: f64 = (assign3820_e4566).sqrt();
        let assign3820_e4568: f64 = (assign3820_e4549 + assign3820_e4567);
        let assign3820_e4569: f64 = (0.5 * assign3820_e4568);
        let assign3820_e4570: f64 = (1.0 + assign3820_e4569);
        let assign3820_e4575: f64 = (0.9 * 0.9);
        let assign3820_e4578: f64 = (4.0 * 0.001);
        let assign3820_e4580: f64 = (assign3820_e4578 * 0.001);
        let assign3820_e4581: f64 = (assign3820_e4575 + assign3820_e4580);
        let assign3820_e4582: f64 = (assign3820_e4581).sqrt();
        let assign3820_e4583: f64 = (0.9 + assign3820_e4582);
        let assign3820_e4584: f64 = (0.5 * assign3820_e4583);
        let assign3820_e4585: f64 = (assign3820_e4570 - assign3820_e4584);
        let assign3820_e4586: f64 = (var_vsat1_i * assign3820_e4585);
        var_vsat1_t = assign3820_e4586;
        var_vsat1_t_dn4 = (var_vsat1_i * (0.5 * ((-(var_at_i * var_deltemp_dn4)) + ((((-(var_at_i * var_deltemp_dn4)) * assign3820_e4559) + (assign3820_e4554 * (-(var_at_i * var_deltemp_dn4)))) / (2.0 * assign3820_e4567)))));

        let assign3830_e4589: f64 = if var_vsat1_t < 1000.0 { 1.0 } else { 0.0 };
        var_guard64 = assign3830_e4589;

        let (assign3840_e4593, assign3840_e4593_d_n4,) = {
    if (var_guard64 != 0.0) {
        (1000.0, 0.0,)
    } else {
        (var_vsat1_t, var_vsat1_t_dn4,)
    }
};
        var_vsat1_t = assign3840_e4593;
        var_vsat1_t_dn4 = assign3840_e4593_d_n4;

        let assign3850_e4600: f64 = (var_at_i * var_deltemp);
        let assign3850_e4601: f64 = (0.9 - assign3850_e4600);
        let assign3850_e4605: f64 = (var_at_i * var_deltemp);
        let assign3850_e4606: f64 = (0.9 - assign3850_e4605);
        let assign3850_e4610: f64 = (var_at_i * var_deltemp);
        let assign3850_e4611: f64 = (0.9 - assign3850_e4610);
        let assign3850_e4612: f64 = (assign3850_e4606 * assign3850_e4611);
        let assign3850_e4615: f64 = (4.0 * 0.001);
        let assign3850_e4617: f64 = (assign3850_e4615 * 0.001);
        let assign3850_e4618: f64 = (assign3850_e4612 + assign3850_e4617);
        let assign3850_e4619: f64 = (assign3850_e4618).sqrt();
        let assign3850_e4620: f64 = (assign3850_e4601 + assign3850_e4619);
        let assign3850_e4621: f64 = (0.5 * assign3850_e4620);
        let assign3850_e4622: f64 = (1.0 + assign3850_e4621);
        let assign3850_e4627: f64 = (0.9 * 0.9);
        let assign3850_e4630: f64 = (4.0 * 0.001);
        let assign3850_e4632: f64 = (assign3850_e4630 * 0.001);
        let assign3850_e4633: f64 = (assign3850_e4627 + assign3850_e4632);
        let assign3850_e4634: f64 = (assign3850_e4633).sqrt();
        let assign3850_e4635: f64 = (0.9 + assign3850_e4634);
        let assign3850_e4636: f64 = (0.5 * assign3850_e4635);
        let assign3850_e4637: f64 = (assign3850_e4622 - assign3850_e4636);
        let assign3850_e4638: f64 = (var_vsatcv_i * assign3850_e4637);
        var_vsatcv_t = assign3850_e4638;
        var_vsatcv_t_dn4 = (var_vsatcv_i * (0.5 * ((-(var_at_i * var_deltemp_dn4)) + ((((-(var_at_i * var_deltemp_dn4)) * assign3850_e4611) + (assign3850_e4606 * (-(var_at_i * var_deltemp_dn4)))) / (2.0 * assign3850_e4619)))));

        let assign3860_e4641: f64 = if var_vsatcv_t < 1000.0 { 1.0 } else { 0.0 };
        var_guard65 = assign3860_e4641;

        let (assign3870_e4645, assign3870_e4645_d_n4,) = {
    if (var_guard65 != 0.0) {
        (1000.0, 0.0,)
    } else {
        (var_vsatcv_t, var_vsatcv_t_dn4,)
    }
};
        var_vsatcv_t = assign3870_e4645;
        var_vsatcv_t_dn4 = assign3870_e4645_d_n4;

        let assign3880_e4649: f64 = (-0.9);
        let assign3880_e4653: f64 = (p.p309 * var_deltemp);
        let assign3880_e4655: f64 = (-0.9);
        let assign3880_e4656: f64 = (assign3880_e4653 - assign3880_e4655);
        let assign3880_e4658: f64 = (assign3880_e4656 - 0.0001);
        let assign3880_e4661: f64 = (p.p309 * var_deltemp);
        let assign3880_e4663: f64 = (-0.9);
        let assign3880_e4664: f64 = (assign3880_e4661 - assign3880_e4663);
        let assign3880_e4666: f64 = (assign3880_e4664 - 0.0001);
        let assign3880_e4669: f64 = (p.p309 * var_deltemp);
        let assign3880_e4671: f64 = (-0.9);
        let assign3880_e4672: f64 = (assign3880_e4669 - assign3880_e4671);
        let assign3880_e4674: f64 = (assign3880_e4672 - 0.0001);
        let assign3880_e4675: f64 = (assign3880_e4666 * assign3880_e4674);
        let assign3880_e4678: f64 = (-0.9);
        let assign3880_e4679: f64 = (4.0 * assign3880_e4678);
        let assign3880_e4681: f64 = (assign3880_e4679 * 0.0001);
        let assign3880_e4682: f64 = (assign3880_e4675 - assign3880_e4681);
        let assign3880_e4683: f64 = (assign3880_e4682).sqrt();
        let assign3880_e4684: f64 = (assign3880_e4658 + assign3880_e4683);
        let assign3880_e4685: f64 = (0.5 * assign3880_e4684);
        let assign3880_e4686: f64 = (assign3880_e4649 + assign3880_e4685);
        let assign3880_e4687: f64 = (1.0 + assign3880_e4686);
        let assign3880_e4688: f64 = (var_eta0_i * assign3880_e4687);
        var_eta0_t = assign3880_e4688;
        var_eta0_t_dn4 = (var_eta0_i * (0.5 * ((p.p309 * var_deltemp_dn4) + ((((p.p309 * var_deltemp_dn4) * assign3880_e4674) + (assign3880_e4666 * (p.p309 * var_deltemp_dn4))) / (2.0 * assign3880_e4683)))));

        let assign3890_e4693: f64 = (var_inv_l * p.p131);
        let assign3890_e4694: f64 = (1.0 + assign3890_e4693);
        let assign3890_e4695: f64 = (var_atb_i * assign3890_e4694);
        var_atb_i = assign3890_e4695;

        let assign3900_e4702: f64 = (var_atb_i * var_deltemp);
        let assign3900_e4703: f64 = (0.9 - assign3900_e4702);
        let assign3900_e4707: f64 = (var_atb_i * var_deltemp);
        let assign3900_e4708: f64 = (0.9 - assign3900_e4707);
        let assign3900_e4712: f64 = (var_atb_i * var_deltemp);
        let assign3900_e4713: f64 = (0.9 - assign3900_e4712);
        let assign3900_e4714: f64 = (assign3900_e4708 * assign3900_e4713);
        let assign3900_e4717: f64 = (4.0 * 0.001);
        let assign3900_e4719: f64 = (assign3900_e4717 * 0.001);
        let assign3900_e4720: f64 = (assign3900_e4714 + assign3900_e4719);
        let assign3900_e4721: f64 = (assign3900_e4720).sqrt();
        let assign3900_e4722: f64 = (assign3900_e4703 + assign3900_e4721);
        let assign3900_e4723: f64 = (0.5 * assign3900_e4722);
        let assign3900_e4724: f64 = (1.0 + assign3900_e4723);
        let assign3900_e4729: f64 = (0.9 * 0.9);
        let assign3900_e4732: f64 = (4.0 * 0.001);
        let assign3900_e4734: f64 = (assign3900_e4732 * 0.001);
        let assign3900_e4735: f64 = (assign3900_e4729 + assign3900_e4734);
        let assign3900_e4736: f64 = (assign3900_e4735).sqrt();
        let assign3900_e4737: f64 = (0.9 + assign3900_e4736);
        let assign3900_e4738: f64 = (0.5 * assign3900_e4737);
        let assign3900_e4739: f64 = (assign3900_e4724 - assign3900_e4738);
        let assign3900_e4740: f64 = (var_vsatb_i * assign3900_e4739);
        var_vsatb_t = assign3900_e4740;
        var_vsatb_t_dn4 = (var_vsatb_i * (0.5 * ((-(var_atb_i * var_deltemp_dn4)) + ((((-(var_atb_i * var_deltemp_dn4)) * assign3900_e4713) + (assign3900_e4708 * (-(var_atb_i * var_deltemp_dn4)))) / (2.0 * assign3900_e4721)))));

        let assign3910_e4746: f64 = (p.p121 * var_deltemp);
        let assign3910_e4747: f64 = (1.0 + assign3910_e4746);
        let assign3910_e4748: f64 = (var_mexp_i * assign3910_e4747);
        let assign3910_e4750: f64 = (assign3910_e4748 - 2.0);
        let assign3910_e4755: f64 = (p.p121 * var_deltemp);
        let assign3910_e4756: f64 = (1.0 + assign3910_e4755);
        let assign3910_e4757: f64 = (var_mexp_i * assign3910_e4756);
        let assign3910_e4759: f64 = (assign3910_e4757 - 2.0);
        let assign3910_e4764: f64 = (p.p121 * var_deltemp);
        let assign3910_e4765: f64 = (1.0 + assign3910_e4764);
        let assign3910_e4766: f64 = (var_mexp_i * assign3910_e4765);
        let assign3910_e4768: f64 = (assign3910_e4766 - 2.0);
        let assign3910_e4769: f64 = (assign3910_e4759 * assign3910_e4768);
        let assign3910_e4772: f64 = (4.0 * 0.001);
        let assign3910_e4774: f64 = (assign3910_e4772 * 0.001);
        let assign3910_e4775: f64 = (assign3910_e4769 + assign3910_e4774);
        let assign3910_e4776: f64 = (assign3910_e4775).sqrt();
        let assign3910_e4777: f64 = (assign3910_e4750 + assign3910_e4776);
        let assign3910_e4778: f64 = (0.5 * assign3910_e4777);
        let assign3910_e4780: f64 = (assign3910_e4778 + 2.0);
        var_mexp_t = assign3910_e4780;
        var_mexp_t_dn4 = (0.5 * ((var_mexp_i * (p.p121 * var_deltemp_dn4)) + ((((var_mexp_i * (p.p121 * var_deltemp_dn4)) * assign3910_e4768) + (assign3910_e4759 * (var_mexp_i * (p.p121 * var_deltemp_dn4)))) / (2.0 * assign3910_e4776))));

        let assign3920_e4784: f64 = (var_k01_i * var_deltemp);
        let assign3920_e4785: f64 = (var_k0_i + assign3920_e4784);
        var_k0_t = assign3920_e4785;
        var_k0_t_dn4 = (var_k01_i * var_deltemp_dn4);

        let assign3930_e4788: f64 = (-var_k0si_i);
        let assign3930_e4792: f64 = (var_k0si1_i * var_deltemp);
        let assign3930_e4794: f64 = (-var_k0si_i);
        let assign3930_e4795: f64 = (assign3930_e4792 - assign3930_e4794);
        let assign3930_e4797: f64 = (assign3930_e4795 - 1e-6);
        let assign3930_e4800: f64 = (var_k0si1_i * var_deltemp);
        let assign3930_e4802: f64 = (-var_k0si_i);
        let assign3930_e4803: f64 = (assign3930_e4800 - assign3930_e4802);
        let assign3930_e4805: f64 = (assign3930_e4803 - 1e-6);
        let assign3930_e4808: f64 = (var_k0si1_i * var_deltemp);
        let assign3930_e4810: f64 = (-var_k0si_i);
        let assign3930_e4811: f64 = (assign3930_e4808 - assign3930_e4810);
        let assign3930_e4813: f64 = (assign3930_e4811 - 1e-6);
        let assign3930_e4814: f64 = (assign3930_e4805 * assign3930_e4813);
        let assign3930_e4817: f64 = (-var_k0si_i);
        let assign3930_e4818: f64 = (4.0 * assign3930_e4817);
        let assign3930_e4820: f64 = (assign3930_e4818 * 1e-6);
        let assign3930_e4821: f64 = (assign3930_e4814 - assign3930_e4820);
        let assign3930_e4822: f64 = (assign3930_e4821).sqrt();
        let assign3930_e4823: f64 = (assign3930_e4797 + assign3930_e4822);
        let assign3930_e4824: f64 = (0.5 * assign3930_e4823);
        let assign3930_e4825: f64 = (assign3930_e4788 + assign3930_e4824);
        let assign3930_e4826: f64 = (var_k0si_i + assign3930_e4825);
        var_k0si_t = assign3930_e4826;
        var_k0si_t_dn4 = (0.5 * ((var_k0si1_i * var_deltemp_dn4) + ((((var_k0si1_i * var_deltemp_dn4) * assign3930_e4813) + (assign3930_e4805 * (var_k0si1_i * var_deltemp_dn4))) / (2.0 * assign3930_e4822))));

        let assign3940_e4830: f64 = (var_k0sisat1_i * var_deltemp);
        let assign3940_e4831: f64 = (var_k0sisat_i + assign3940_e4830);
        var_k0sisat_t = assign3940_e4831;
        var_k0sisat_t_dn4 = (var_k0sisat1_i * var_deltemp_dn4);

        let assign3950_e4837: f64 = (var_ptwgt_i * var_deltemp);
        let assign3950_e4838: f64 = (1.0 - assign3950_e4837);
        let assign3950_e4840: f64 = (assign3950_e4838 - 1e-6);
        let assign3950_e4844: f64 = (var_ptwgt_i * var_deltemp);
        let assign3950_e4845: f64 = (1.0 - assign3950_e4844);
        let assign3950_e4847: f64 = (assign3950_e4845 - 1e-6);
        let assign3950_e4851: f64 = (var_ptwgt_i * var_deltemp);
        let assign3950_e4852: f64 = (1.0 - assign3950_e4851);
        let assign3950_e4854: f64 = (assign3950_e4852 - 1e-6);
        let assign3950_e4855: f64 = (assign3950_e4847 * assign3950_e4854);
        let assign3950_e4858: f64 = (4.0 * 0.001);
        let assign3950_e4860: f64 = (assign3950_e4858 * 0.001);
        let assign3950_e4861: f64 = (assign3950_e4855 + assign3950_e4860);
        let assign3950_e4862: f64 = (assign3950_e4861).sqrt();
        let assign3950_e4863: f64 = (assign3950_e4840 + assign3950_e4862);
        let assign3950_e4864: f64 = (0.5 * assign3950_e4863);
        let assign3950_e4865: f64 = (var_ptwg_i * assign3950_e4864);
        var_ptwg_t = assign3950_e4865;
        var_ptwg_t_dn4 = (var_ptwg_i * (0.5 * ((-(var_ptwgt_i * var_deltemp_dn4)) + ((((-(var_ptwgt_i * var_deltemp_dn4)) * assign3950_e4854) + (assign3950_e4847 * (-(var_ptwgt_i * var_deltemp_dn4)))) / (2.0 * assign3950_e4862)))));

        let assign3960_e4869: f64 = (p.p302 / var_leff);
        let assign3960_e4870: f64 = (p.p301 + assign3960_e4869);
        let assign3960_e4873: f64 = (var_tratio - 1.0);
        let assign3960_e4874: f64 = (assign3960_e4870 * assign3960_e4873);
        var_dvth_temp0 = assign3960_e4874;
        var_dvth_temp0_dn4 = (assign3960_e4870 * var_tratio_dn4);

        let assign3970_e4878: f64 = (var_tratio).powf(var_iit_i);
        let assign3970_e4879: f64 = (var_beta0_i * assign3970_e4878);
        var_beta0_t = assign3970_e4879;
        var_beta0_t_dn4 = (var_beta0_i * if 0.0 == 0.0 && ((var_iit_i) as f64).is_finite() && ((var_iit_i) as f64).fract() == 0.0 { if var_iit_i == 0.0 { 0.0 } else { (var_iit_i * ((var_tratio).powf(var_iit_i - 1.0) * var_tratio_dn4)) } } else { (assign3970_e4878 * (var_iit_i * (var_tratio_dn4 / var_tratio))) });

        let assign3980_e4885: f64 = (var_tgidl_i * var_deltemp);
        let assign3980_e4886: f64 = (1.0 + assign3980_e4885);
        let assign3980_e4888: f64 = (assign3980_e4886 - 1e-6);
        let assign3980_e4892: f64 = (var_tgidl_i * var_deltemp);
        let assign3980_e4893: f64 = (1.0 + assign3980_e4892);
        let assign3980_e4895: f64 = (assign3980_e4893 - 1e-6);
        let assign3980_e4899: f64 = (var_tgidl_i * var_deltemp);
        let assign3980_e4900: f64 = (1.0 + assign3980_e4899);
        let assign3980_e4902: f64 = (assign3980_e4900 - 1e-6);
        let assign3980_e4903: f64 = (assign3980_e4895 * assign3980_e4902);
        let assign3980_e4906: f64 = (4.0 * 0.001);
        let assign3980_e4908: f64 = (assign3980_e4906 * 0.001);
        let assign3980_e4909: f64 = (assign3980_e4903 + assign3980_e4908);
        let assign3980_e4910: f64 = (assign3980_e4909).sqrt();
        let assign3980_e4911: f64 = (assign3980_e4888 + assign3980_e4910);
        let assign3980_e4912: f64 = (0.5 * assign3980_e4911);
        let assign3980_e4913: f64 = (var_bgidl_i * assign3980_e4912);
        var_bgidl_t = assign3980_e4913;
        var_bgidl_t_dn4 = (var_bgidl_i * (0.5 * ((var_tgidl_i * var_deltemp_dn4) + ((((var_tgidl_i * var_deltemp_dn4) * assign3980_e4902) + (assign3980_e4895 * (var_tgidl_i * var_deltemp_dn4))) / (2.0 * assign3980_e4910)))));

        let assign3990_e4919: f64 = (var_tgisl_i * var_deltemp);
        let assign3990_e4920: f64 = (1.0 + assign3990_e4919);
        let assign3990_e4922: f64 = (assign3990_e4920 - 1e-6);
        let assign3990_e4926: f64 = (var_tgisl_i * var_deltemp);
        let assign3990_e4927: f64 = (1.0 + assign3990_e4926);
        let assign3990_e4929: f64 = (assign3990_e4927 - 1e-6);
        let assign3990_e4933: f64 = (var_tgisl_i * var_deltemp);
        let assign3990_e4934: f64 = (1.0 + assign3990_e4933);
        let assign3990_e4936: f64 = (assign3990_e4934 - 1e-6);
        let assign3990_e4937: f64 = (assign3990_e4929 * assign3990_e4936);
        let assign3990_e4940: f64 = (4.0 * 0.001);
        let assign3990_e4942: f64 = (assign3990_e4940 * 0.001);
        let assign3990_e4943: f64 = (assign3990_e4937 + assign3990_e4942);
        let assign3990_e4944: f64 = (assign3990_e4943).sqrt();
        let assign3990_e4945: f64 = (assign3990_e4922 + assign3990_e4944);
        let assign3990_e4946: f64 = (0.5 * assign3990_e4945);
        let assign3990_e4947: f64 = (var_bgisl_i * assign3990_e4946);
        var_bgisl_t = assign3990_e4947;
        var_bgisl_t_dn4 = (var_bgisl_i * (0.5 * ((var_tgisl_i * var_deltemp_dn4) + ((((var_tgisl_i * var_deltemp_dn4) * assign3990_e4936) + (assign3990_e4929 * (var_tgisl_i * var_deltemp_dn4))) / (2.0 * assign3990_e4944)))));

        let assign4000_e4951: f64 = (var_tratio).max(1e-38);
        let assign4000_e4952: f64 = (assign4000_e4951).ln();
        let assign4000_e4953: f64 = (var_igt_i * assign4000_e4952);
        let assign4000_e4954: f64 = { let limited_exp_arg = assign4000_e4953; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        var_igtemp = assign4000_e4954;
        var_igtemp_dn4 = ({ let limited_exp_arg = assign4000_e4953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_igt_i * (if var_tratio >= 1e-38 { var_tratio_dn4 } else { 0.0 } / assign4000_e4951)));

        *var_at_i_slot = var_at_i;
        *var_atb_i_slot = var_atb_i;
        *var_beta0_t_slot = var_beta0_t;
        *var_beta0_t_dn4_slot = var_beta0_t_dn4;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidl_t_dn4_slot = var_bgidl_t_dn4;
        *var_bgisl_t_slot = var_bgisl_t;
        *var_bgisl_t_dn4_slot = var_bgisl_t_dn4;
        *var_deltaphi1_slot = var_deltaphi1;
        *var_deltaphi1_dn4_slot = var_deltaphi1_dn4;
        *var_deltaphi2_slot = var_deltaphi2;
        *var_deltaphi2_dn3_slot = var_deltaphi2_dn3;
        *var_deltaphi2_dn4_slot = var_deltaphi2_dn4;
        *var_deltaphi2_dn5_slot = var_deltaphi2_dn5;
        *var_deltaphi2_dn6_slot = var_deltaphi2_dn6;
        *var_deltaphi2_dn7_slot = var_deltaphi2_dn7;
        *var_deltaphi2_dn8_slot = var_deltaphi2_dn8;
        *var_dvth_temp0_slot = var_dvth_temp0;
        *var_dvth_temp0_dn4_slot = var_dvth_temp0_dn4;
        *var_eta0_t_slot = var_eta0_t;
        *var_eta0_t_dn4_slot = var_eta0_t_dn4;
        *var_guard63_slot = var_guard63;
        *var_guard64_slot = var_guard64;
        *var_guard65_slot = var_guard65;
        *var_igtemp_slot = var_igtemp;
        *var_igtemp_dn4_slot = var_igtemp_dn4;
        *var_k0_t_slot = var_k0_t;
        *var_k0_t_dn4_slot = var_k0_t_dn4;
        *var_k0si_t_slot = var_k0si_t;
        *var_k0si_t_dn4_slot = var_k0si_t_dn4;
        *var_k0sisat_t_slot = var_k0sisat_t;
        *var_k0sisat_t_dn4_slot = var_k0sisat_t_dn4;
        *var_mexp_t_slot = var_mexp_t;
        *var_mexp_t_dn4_slot = var_mexp_t_dn4;
        *var_phig2_i_slot = var_phig2_i;
        *var_phig2_i_dn3_slot = var_phig2_i_dn3;
        *var_phig2_i_dn4_slot = var_phig2_i_dn4;
        *var_phig2_i_dn5_slot = var_phig2_i_dn5;
        *var_phig2_i_dn6_slot = var_phig2_i_dn6;
        *var_phig2_i_dn7_slot = var_phig2_i_dn7;
        *var_phig2_i_dn8_slot = var_phig2_i_dn8;
        *var_phiref_slot = var_phiref;
        *var_phiref_dn4_slot = var_phiref_dn4;
        *var_phisd_slot = var_phisd;
        *var_phisd_dn3_slot = var_phisd_dn3;
        *var_phisd_dn4_slot = var_phisd_dn4;
        *var_phisd_dn5_slot = var_phisd_dn5;
        *var_phisd_dn6_slot = var_phisd_dn6;
        *var_phisd_dn7_slot = var_phisd_dn7;
        *var_phisd_dn8_slot = var_phisd_dn8;
        *var_ptwg_t_slot = var_ptwg_t;
        *var_ptwg_t_dn4_slot = var_ptwg_t_dn4;
        *var_rdstemp_slot = var_rdstemp;
        *var_rdstemp_dn4_slot = var_rdstemp_dn4;
        *var_u0_t_slot = var_u0_t;
        *var_u0_t_dn4_slot = var_u0_t_dn4;
        *var_ua_t_slot = var_ua_t;
        *var_ua_t_dn4_slot = var_ua_t_dn4;
        *var_uc_t_slot = var_uc_t;
        *var_uc_t_dn4_slot = var_uc_t_dn4;
        *var_ucs_t_slot = var_ucs_t;
        *var_ucs_t_dn4_slot = var_ucs_t_dn4;
        *var_ud_t_slot = var_ud_t;
        *var_ud_t_dn4_slot = var_ud_t_dn4;
        *var_vfbsd_slot = var_vfbsd;
        *var_vfbsd_bg_slot = var_vfbsd_bg;
        *var_vfbsd_bg_dn3_slot = var_vfbsd_bg_dn3;
        *var_vfbsd_bg_dn4_slot = var_vfbsd_bg_dn4;
        *var_vfbsd_bg_dn5_slot = var_vfbsd_bg_dn5;
        *var_vfbsd_bg_dn6_slot = var_vfbsd_bg_dn6;
        *var_vfbsd_bg_dn7_slot = var_vfbsd_bg_dn7;
        *var_vfbsd_bg_dn8_slot = var_vfbsd_bg_dn8;
        *var_vfbsd_dn3_slot = var_vfbsd_dn3;
        *var_vfbsd_dn4_slot = var_vfbsd_dn4;
        *var_vfbsd_dn5_slot = var_vfbsd_dn5;
        *var_vfbsd_dn6_slot = var_vfbsd_dn6;
        *var_vfbsd_dn7_slot = var_vfbsd_dn7;
        *var_vfbsd_dn8_slot = var_vfbsd_dn8;
        *var_vsat1_t_slot = var_vsat1_t;
        *var_vsat1_t_dn4_slot = var_vsat1_t_dn4;
        *var_vsat_t_slot = var_vsat_t;
        *var_vsat_t_dn4_slot = var_vsat_t_dn4;
        *var_vsatb_t_slot = var_vsatb_t;
        *var_vsatb_t_dn4_slot = var_vsatb_t_dn4;
        *var_vsatcv_t_slot = var_vsatcv_t;
        *var_vsatcv_t_dn4_slot = var_vsatcv_t_dn4;
    }

    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ascl_i: f64,
        var_bscl_i: f64,
        var_dbgpw_i: f64,
        var_deltaphi1: f64,
        var_deltaphi1_dn4: f64,
        var_deltaphi2: f64,
        var_deltaphi2_dn3: f64,
        var_deltaphi2_dn4: f64,
        var_deltaphi2_dn5: f64,
        var_deltaphi2_dn6: f64,
        var_deltaphi2_dn7: f64,
        var_deltaphi2_dn8: f64,
        var_devsign: f64,
        var_drout_i: f64,
        var_dsub_i: f64,
        var_dvt1_i: f64,
        var_epsratio: f64,
        var_igsd_mult0: f64,
        var_igsd_mult0_dn3: f64,
        var_igsd_mult0_dn4: f64,
        var_igsd_mult0_dn5: f64,
        var_igsd_mult0_dn6: f64,
        var_igsd_mult0_dn7: f64,
        var_igsd_mult0_dn8: f64,
        var_igtemp: f64,
        var_igtemp_dn4: f64,
        var_leff: f64,
        var_pdibl1_i: f64,
        var_pdibl2_i: f64,
        var_teff: f64,
        var_diblfactor_slot: &mut f64,
        var_diblfactor_dn3_slot: &mut f64,
        var_diblfactor_dn4_slot: &mut f64,
        var_diblfactor_dn5_slot: &mut f64,
        var_diblfactor_dn6_slot: &mut f64,
        var_diblfactor_dn7_slot: &mut f64,
        var_diblfactor_dn8_slot: &mut f64,
        var_guard66_slot: &mut f64,
        var_guard67_slot: &mut f64,
        var_guard68_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_igsd_mult_slot: &mut f64,
        var_igsd_mult_dn3_slot: &mut f64,
        var_igsd_mult_dn4_slot: &mut f64,
        var_igsd_mult_dn5_slot: &mut f64,
        var_igsd_mult_dn6_slot: &mut f64,
        var_igsd_mult_dn7_slot: &mut f64,
        var_igsd_mult_dn8_slot: &mut f64,
        var_scl_slot: &mut f64,
        var_scl_dn3_slot: &mut f64,
        var_scl_dn4_slot: &mut f64,
        var_scl_dn5_slot: &mut f64,
        var_scl_dn6_slot: &mut f64,
        var_scl_dn7_slot: &mut f64,
        var_scl_dn8_slot: &mut f64,
        var_sclf_slot: &mut f64,
        var_sclm_slot: &mut f64,
        var_sigvds_slot: &mut f64,
        var_symmetry_factor_slot: &mut f64,
        var_symmetry_factor_dn5_slot: &mut f64,
        var_symmetry_factor_dn6_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_theta_dibl_slot: &mut f64,
        var_theta_dibl_dn3_slot: &mut f64,
        var_theta_dibl_dn4_slot: &mut f64,
        var_theta_dibl_dn5_slot: &mut f64,
        var_theta_dibl_dn6_slot: &mut f64,
        var_theta_dibl_dn7_slot: &mut f64,
        var_theta_dibl_dn8_slot: &mut f64,
        var_theta_dits_slot: &mut f64,
        var_theta_dits_dn3_slot: &mut f64,
        var_theta_dits_dn4_slot: &mut f64,
        var_theta_dits_dn5_slot: &mut f64,
        var_theta_dits_dn6_slot: &mut f64,
        var_theta_dits_dn7_slot: &mut f64,
        var_theta_dits_dn8_slot: &mut f64,
        var_theta_sce_slot: &mut f64,
        var_theta_sce_dn3_slot: &mut f64,
        var_theta_sce_dn4_slot: &mut f64,
        var_theta_sce_dn5_slot: &mut f64,
        var_theta_sce_dn6_slot: &mut f64,
        var_theta_sce_dn7_slot: &mut f64,
        var_theta_sce_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn3_slot: &mut f64,
        var_tmp_dn4_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbgd_slot: &mut f64,
        var_vbgd_dn3_slot: &mut f64,
        var_vbgd_dn5_slot: &mut f64,
        var_vbgd_dn6_slot: &mut f64,
        var_vbgd_noswap_slot: &mut f64,
        var_vbgd_noswap_dn3_slot: &mut f64,
        var_vbgd_noswap_dn5_slot: &mut f64,
        var_vbgs_slot: &mut f64,
        var_vbgs_dn3_slot: &mut f64,
        var_vbgs_dn5_slot: &mut f64,
        var_vbgs_dn6_slot: &mut f64,
        var_vbgs_noswap_slot: &mut f64,
        var_vbgs_noswap_dn3_slot: &mut f64,
        var_vbgs_noswap_dn6_slot: &mut f64,
        var_vbgx_slot: &mut f64,
        var_vbgx_dn3_slot: &mut f64,
        var_vbgx_dn5_slot: &mut f64,
        var_vbgx_dn6_slot: &mut f64,
        var_vds_slot: &mut f64,
        var_vds_dn5_slot: &mut f64,
        var_vds_dn6_slot: &mut f64,
        var_vds_noswap_slot: &mut f64,
        var_vds_noswap_dn5_slot: &mut f64,
        var_vds_noswap_dn6_slot: &mut f64,
        var_vdsx_slot: &mut f64,
        var_vdsx_dn5_slot: &mut f64,
        var_vdsx_dn6_slot: &mut f64,
        var_vfgs_slot: &mut f64,
        var_vfgs_dn5_slot: &mut f64,
        var_vfgs_dn6_slot: &mut f64,
        var_vfgs_dn8_slot: &mut f64,
        var_vgbg_slot: &mut f64,
        var_vgbg_dn3_slot: &mut f64,
        var_vgbg_dn8_slot: &mut f64,
        var_vgd_noswap_slot: &mut f64,
        var_vgd_noswap_dn5_slot: &mut f64,
        var_vgd_noswap_dn8_slot: &mut f64,
        var_vgd_ov_noswap_slot: &mut f64,
        var_vgd_ov_noswap_dn5_slot: &mut f64,
        var_vgd_ov_noswap_dn7_slot: &mut f64,
        var_vgfb1_slot: &mut f64,
        var_vgfb1_dn4_slot: &mut f64,
        var_vgfb1_dn5_slot: &mut f64,
        var_vgfb1_dn6_slot: &mut f64,
        var_vgfb1_dn8_slot: &mut f64,
        var_vgfb2_slot: &mut f64,
        var_vgfb2_dn3_slot: &mut f64,
        var_vgfb2_dn4_slot: &mut f64,
        var_vgfb2_dn5_slot: &mut f64,
        var_vgfb2_dn6_slot: &mut f64,
        var_vgfb2_dn7_slot: &mut f64,
        var_vgfb2_dn8_slot: &mut f64,
        var_vgs_noswap_slot: &mut f64,
        var_vgs_noswap_dn6_slot: &mut f64,
        var_vgs_noswap_dn8_slot: &mut f64,
        var_vgs_ov_noswap_slot: &mut f64,
        var_vgs_ov_noswap_dn6_slot: &mut f64,
        var_vgs_ov_noswap_dn7_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_diblfactor: f64 = *var_diblfactor_slot;
        let mut var_diblfactor_dn3: f64 = *var_diblfactor_dn3_slot;
        let mut var_diblfactor_dn4: f64 = *var_diblfactor_dn4_slot;
        let mut var_diblfactor_dn5: f64 = *var_diblfactor_dn5_slot;
        let mut var_diblfactor_dn6: f64 = *var_diblfactor_dn6_slot;
        let mut var_diblfactor_dn7: f64 = *var_diblfactor_dn7_slot;
        let mut var_diblfactor_dn8: f64 = *var_diblfactor_dn8_slot;
        let mut var_guard66: f64 = *var_guard66_slot;
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_igsd_mult: f64 = *var_igsd_mult_slot;
        let mut var_igsd_mult_dn3: f64 = *var_igsd_mult_dn3_slot;
        let mut var_igsd_mult_dn4: f64 = *var_igsd_mult_dn4_slot;
        let mut var_igsd_mult_dn5: f64 = *var_igsd_mult_dn5_slot;
        let mut var_igsd_mult_dn6: f64 = *var_igsd_mult_dn6_slot;
        let mut var_igsd_mult_dn7: f64 = *var_igsd_mult_dn7_slot;
        let mut var_igsd_mult_dn8: f64 = *var_igsd_mult_dn8_slot;
        let mut var_scl: f64 = *var_scl_slot;
        let mut var_scl_dn3: f64 = *var_scl_dn3_slot;
        let mut var_scl_dn4: f64 = *var_scl_dn4_slot;
        let mut var_scl_dn5: f64 = *var_scl_dn5_slot;
        let mut var_scl_dn6: f64 = *var_scl_dn6_slot;
        let mut var_scl_dn7: f64 = *var_scl_dn7_slot;
        let mut var_scl_dn8: f64 = *var_scl_dn8_slot;
        let mut var_sclf: f64 = *var_sclf_slot;
        let mut var_sclm: f64 = *var_sclm_slot;
        let mut var_sigvds: f64 = *var_sigvds_slot;
        let mut var_symmetry_factor: f64 = *var_symmetry_factor_slot;
        let mut var_symmetry_factor_dn5: f64 = *var_symmetry_factor_dn5_slot;
        let mut var_symmetry_factor_dn6: f64 = *var_symmetry_factor_dn6_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_theta_dibl: f64 = *var_theta_dibl_slot;
        let mut var_theta_dibl_dn3: f64 = *var_theta_dibl_dn3_slot;
        let mut var_theta_dibl_dn4: f64 = *var_theta_dibl_dn4_slot;
        let mut var_theta_dibl_dn5: f64 = *var_theta_dibl_dn5_slot;
        let mut var_theta_dibl_dn6: f64 = *var_theta_dibl_dn6_slot;
        let mut var_theta_dibl_dn7: f64 = *var_theta_dibl_dn7_slot;
        let mut var_theta_dibl_dn8: f64 = *var_theta_dibl_dn8_slot;
        let mut var_theta_dits: f64 = *var_theta_dits_slot;
        let mut var_theta_dits_dn3: f64 = *var_theta_dits_dn3_slot;
        let mut var_theta_dits_dn4: f64 = *var_theta_dits_dn4_slot;
        let mut var_theta_dits_dn5: f64 = *var_theta_dits_dn5_slot;
        let mut var_theta_dits_dn6: f64 = *var_theta_dits_dn6_slot;
        let mut var_theta_dits_dn7: f64 = *var_theta_dits_dn7_slot;
        let mut var_theta_dits_dn8: f64 = *var_theta_dits_dn8_slot;
        let mut var_theta_sce: f64 = *var_theta_sce_slot;
        let mut var_theta_sce_dn3: f64 = *var_theta_sce_dn3_slot;
        let mut var_theta_sce_dn4: f64 = *var_theta_sce_dn4_slot;
        let mut var_theta_sce_dn5: f64 = *var_theta_sce_dn5_slot;
        let mut var_theta_sce_dn6: f64 = *var_theta_sce_dn6_slot;
        let mut var_theta_sce_dn7: f64 = *var_theta_sce_dn7_slot;
        let mut var_theta_sce_dn8: f64 = *var_theta_sce_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn3: f64 = *var_tmp_dn3_slot;
        let mut var_tmp_dn4: f64 = *var_tmp_dn4_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbgd: f64 = *var_vbgd_slot;
        let mut var_vbgd_dn3: f64 = *var_vbgd_dn3_slot;
        let mut var_vbgd_dn5: f64 = *var_vbgd_dn5_slot;
        let mut var_vbgd_dn6: f64 = *var_vbgd_dn6_slot;
        let mut var_vbgd_noswap: f64 = *var_vbgd_noswap_slot;
        let mut var_vbgd_noswap_dn3: f64 = *var_vbgd_noswap_dn3_slot;
        let mut var_vbgd_noswap_dn5: f64 = *var_vbgd_noswap_dn5_slot;
        let mut var_vbgs: f64 = *var_vbgs_slot;
        let mut var_vbgs_dn3: f64 = *var_vbgs_dn3_slot;
        let mut var_vbgs_dn5: f64 = *var_vbgs_dn5_slot;
        let mut var_vbgs_dn6: f64 = *var_vbgs_dn6_slot;
        let mut var_vbgs_noswap: f64 = *var_vbgs_noswap_slot;
        let mut var_vbgs_noswap_dn3: f64 = *var_vbgs_noswap_dn3_slot;
        let mut var_vbgs_noswap_dn6: f64 = *var_vbgs_noswap_dn6_slot;
        let mut var_vbgx: f64 = *var_vbgx_slot;
        let mut var_vbgx_dn3: f64 = *var_vbgx_dn3_slot;
        let mut var_vbgx_dn5: f64 = *var_vbgx_dn5_slot;
        let mut var_vbgx_dn6: f64 = *var_vbgx_dn6_slot;
        let mut var_vds: f64 = *var_vds_slot;
        let mut var_vds_dn5: f64 = *var_vds_dn5_slot;
        let mut var_vds_dn6: f64 = *var_vds_dn6_slot;
        let mut var_vds_noswap: f64 = *var_vds_noswap_slot;
        let mut var_vds_noswap_dn5: f64 = *var_vds_noswap_dn5_slot;
        let mut var_vds_noswap_dn6: f64 = *var_vds_noswap_dn6_slot;
        let mut var_vdsx: f64 = *var_vdsx_slot;
        let mut var_vdsx_dn5: f64 = *var_vdsx_dn5_slot;
        let mut var_vdsx_dn6: f64 = *var_vdsx_dn6_slot;
        let mut var_vfgs: f64 = *var_vfgs_slot;
        let mut var_vfgs_dn5: f64 = *var_vfgs_dn5_slot;
        let mut var_vfgs_dn6: f64 = *var_vfgs_dn6_slot;
        let mut var_vfgs_dn8: f64 = *var_vfgs_dn8_slot;
        let mut var_vgbg: f64 = *var_vgbg_slot;
        let mut var_vgbg_dn3: f64 = *var_vgbg_dn3_slot;
        let mut var_vgbg_dn8: f64 = *var_vgbg_dn8_slot;
        let mut var_vgd_noswap: f64 = *var_vgd_noswap_slot;
        let mut var_vgd_noswap_dn5: f64 = *var_vgd_noswap_dn5_slot;
        let mut var_vgd_noswap_dn8: f64 = *var_vgd_noswap_dn8_slot;
        let mut var_vgd_ov_noswap: f64 = *var_vgd_ov_noswap_slot;
        let mut var_vgd_ov_noswap_dn5: f64 = *var_vgd_ov_noswap_dn5_slot;
        let mut var_vgd_ov_noswap_dn7: f64 = *var_vgd_ov_noswap_dn7_slot;
        let mut var_vgfb1: f64 = *var_vgfb1_slot;
        let mut var_vgfb1_dn4: f64 = *var_vgfb1_dn4_slot;
        let mut var_vgfb1_dn5: f64 = *var_vgfb1_dn5_slot;
        let mut var_vgfb1_dn6: f64 = *var_vgfb1_dn6_slot;
        let mut var_vgfb1_dn8: f64 = *var_vgfb1_dn8_slot;
        let mut var_vgfb2: f64 = *var_vgfb2_slot;
        let mut var_vgfb2_dn3: f64 = *var_vgfb2_dn3_slot;
        let mut var_vgfb2_dn4: f64 = *var_vgfb2_dn4_slot;
        let mut var_vgfb2_dn5: f64 = *var_vgfb2_dn5_slot;
        let mut var_vgfb2_dn6: f64 = *var_vgfb2_dn6_slot;
        let mut var_vgfb2_dn7: f64 = *var_vgfb2_dn7_slot;
        let mut var_vgfb2_dn8: f64 = *var_vgfb2_dn8_slot;
        let mut var_vgs_noswap: f64 = *var_vgs_noswap_slot;
        let mut var_vgs_noswap_dn6: f64 = *var_vgs_noswap_dn6_slot;
        let mut var_vgs_noswap_dn8: f64 = *var_vgs_noswap_dn8_slot;
        let mut var_vgs_ov_noswap: f64 = *var_vgs_ov_noswap_slot;
        let mut var_vgs_ov_noswap_dn6: f64 = *var_vgs_ov_noswap_dn6_slot;
        let mut var_vgs_ov_noswap_dn7: f64 = *var_vgs_ov_noswap_dn7_slot;

        let assign4010_e4957: f64 = (var_igsd_mult0 * var_igtemp);
        var_igsd_mult = assign4010_e4957;
        var_igsd_mult_dn3 = (var_igsd_mult0_dn3 * var_igtemp);
        var_igsd_mult_dn4 = ((var_igsd_mult0_dn4 * var_igtemp) + (var_igsd_mult0 * var_igtemp_dn4));
        var_igsd_mult_dn5 = (var_igsd_mult0_dn5 * var_igtemp);
        var_igsd_mult_dn6 = (var_igsd_mult0_dn6 * var_igtemp);
        var_igsd_mult_dn7 = (var_igsd_mult0_dn7 * var_igtemp);
        var_igsd_mult_dn8 = (var_igsd_mult0_dn8 * var_igtemp);

        let assign4020_e4960: f64 = (var_devsign * (nv8 - nv6));
        var_vgs_noswap = assign4020_e4960;
        var_vgs_noswap_dn6 = (-var_devsign);
        var_vgs_noswap_dn8 = var_devsign;

        let assign4030_e4963: f64 = (var_devsign * (nv5 - nv6));
        var_vds_noswap = assign4030_e4963;
        var_vds_noswap_dn5 = var_devsign;
        var_vds_noswap_dn6 = (-var_devsign);

        let assign4040_e4966: f64 = (var_devsign * (nv8 - nv5));
        var_vgd_noswap = assign4040_e4966;
        var_vgd_noswap_dn5 = (-var_devsign);
        var_vgd_noswap_dn8 = var_devsign;

        let assign4050_e4969: f64 = (var_devsign * (nv3 - nv6));
        var_vbgs_noswap = assign4050_e4969;
        var_vbgs_noswap_dn3 = var_devsign;
        var_vbgs_noswap_dn6 = (-var_devsign);

        let assign4060_e4972: f64 = (var_devsign * (nv3 - nv5));
        var_vbgd_noswap = assign4060_e4972;
        var_vbgd_noswap_dn3 = var_devsign;
        var_vbgd_noswap_dn5 = (-var_devsign);

        let assign4070_e4975: f64 = (var_devsign * (nv8 - nv3));
        var_vgbg = assign4070_e4975;
        var_vgbg_dn3 = (-var_devsign);
        var_vgbg_dn8 = var_devsign;

        var_sigvds = 1.0;

        let assign4090_e4979: f64 = if var_vds_noswap < 0.0 { 1.0 } else { 0.0 };
        var_guard66 = assign4090_e4979;

        let (assign4100_e4984,) = {
    if (var_guard66 != 0.0) {
        let assign4100_e4982: f64 = (-1.0);
        (assign4100_e4982,)
    } else {
        (var_sigvds,)
    }
};
        var_sigvds = assign4100_e4984;

        let (assign4110_e4988, assign4110_e4988_d_n5, assign4110_e4988_d_n6, assign4110_e4988_d_n8,) = {
    if (var_guard66 != 0.0) {
        (var_vgd_noswap, var_vgd_noswap_dn5, 0.0, var_vgd_noswap_dn8,)
    } else {
        (var_vfgs, var_vfgs_dn5, var_vfgs_dn6, var_vfgs_dn8,)
    }
};
        var_vfgs = assign4110_e4988;
        var_vfgs_dn5 = assign4110_e4988_d_n5;
        var_vfgs_dn6 = assign4110_e4988_d_n6;
        var_vfgs_dn8 = assign4110_e4988_d_n8;

        let (assign4120_e4993, assign4120_e4993_d_n5, assign4120_e4993_d_n6,) = {
    if (var_guard66 != 0.0) {
        let assign4120_e4991: f64 = (-var_vds_noswap);
        (assign4120_e4991, (-var_vds_noswap_dn5), (-var_vds_noswap_dn6),)
    } else {
        (var_vds, var_vds_dn5, var_vds_dn6,)
    }
};
        var_vds = assign4120_e4993;
        var_vds_dn5 = assign4120_e4993_d_n5;
        var_vds_dn6 = assign4120_e4993_d_n6;

        let (assign4130_e4997, assign4130_e4997_d_n3, assign4130_e4997_d_n5, assign4130_e4997_d_n6,) = {
    if (var_guard66 != 0.0) {
        (var_vbgd_noswap, var_vbgd_noswap_dn3, var_vbgd_noswap_dn5, 0.0,)
    } else {
        (var_vbgs, var_vbgs_dn3, var_vbgs_dn5, var_vbgs_dn6,)
    }
};
        var_vbgs = assign4130_e4997;
        var_vbgs_dn3 = assign4130_e4997_d_n3;
        var_vbgs_dn5 = assign4130_e4997_d_n5;
        var_vbgs_dn6 = assign4130_e4997_d_n6;

        let (assign4140_e5001, assign4140_e5001_d_n3, assign4140_e5001_d_n5, assign4140_e5001_d_n6,) = {
    if (var_guard66 != 0.0) {
        (var_vbgs_noswap, var_vbgs_noswap_dn3, 0.0, var_vbgs_noswap_dn6,)
    } else {
        (var_vbgd, var_vbgd_dn3, var_vbgd_dn5, var_vbgd_dn6,)
    }
};
        var_vbgd = assign4140_e5001;
        var_vbgd_dn3 = assign4140_e5001_d_n3;
        var_vbgd_dn5 = assign4140_e5001_d_n5;
        var_vbgd_dn6 = assign4140_e5001_d_n6;

        let (assign4150_e5006, assign4150_e5006_d_n5, assign4150_e5006_d_n6, assign4150_e5006_d_n8,) = {
    if (var_guard66 == 0.0) {
        (var_vgs_noswap, 0.0, var_vgs_noswap_dn6, var_vgs_noswap_dn8,)
    } else {
        (var_vfgs, var_vfgs_dn5, var_vfgs_dn6, var_vfgs_dn8,)
    }
};
        var_vfgs = assign4150_e5006;
        var_vfgs_dn5 = assign4150_e5006_d_n5;
        var_vfgs_dn6 = assign4150_e5006_d_n6;
        var_vfgs_dn8 = assign4150_e5006_d_n8;

        let (assign4160_e5011, assign4160_e5011_d_n5, assign4160_e5011_d_n6,) = {
    if (var_guard66 == 0.0) {
        (var_vds_noswap, var_vds_noswap_dn5, var_vds_noswap_dn6,)
    } else {
        (var_vds, var_vds_dn5, var_vds_dn6,)
    }
};
        var_vds = assign4160_e5011;
        var_vds_dn5 = assign4160_e5011_d_n5;
        var_vds_dn6 = assign4160_e5011_d_n6;

        let (assign4170_e5016, assign4170_e5016_d_n3, assign4170_e5016_d_n5, assign4170_e5016_d_n6,) = {
    if (var_guard66 == 0.0) {
        (var_vbgs_noswap, var_vbgs_noswap_dn3, 0.0, var_vbgs_noswap_dn6,)
    } else {
        (var_vbgs, var_vbgs_dn3, var_vbgs_dn5, var_vbgs_dn6,)
    }
};
        var_vbgs = assign4170_e5016;
        var_vbgs_dn3 = assign4170_e5016_d_n3;
        var_vbgs_dn5 = assign4170_e5016_d_n5;
        var_vbgs_dn6 = assign4170_e5016_d_n6;

        let (assign4180_e5021, assign4180_e5021_d_n3, assign4180_e5021_d_n5, assign4180_e5021_d_n6,) = {
    if (var_guard66 == 0.0) {
        (var_vbgd_noswap, var_vbgd_noswap_dn3, var_vbgd_noswap_dn5, 0.0,)
    } else {
        (var_vbgd, var_vbgd_dn3, var_vbgd_dn5, var_vbgd_dn6,)
    }
};
        var_vbgd = assign4180_e5021;
        var_vbgd_dn3 = assign4180_e5021_d_n3;
        var_vbgd_dn5 = assign4180_e5021_d_n5;
        var_vbgd_dn6 = assign4180_e5021_d_n6;

        let assign4190_e5024: f64 = (var_devsign * (nv7 - nv5));
        var_vgd_ov_noswap = assign4190_e5024;
        var_vgd_ov_noswap_dn5 = (-var_devsign);
        var_vgd_ov_noswap_dn7 = var_devsign;

        let assign4200_e5027: f64 = (var_devsign * (nv7 - nv6));
        var_vgs_ov_noswap = assign4200_e5027;
        var_vgs_ov_noswap_dn6 = (-var_devsign);
        var_vgs_ov_noswap_dn7 = var_devsign;

        let assign4210_e5030: f64 = (var_vds * var_vds);
        let assign4210_e5032: f64 = (assign4210_e5030 + 0.0004);
        let assign4210_e5033: f64 = (assign4210_e5032).sqrt();
        let assign4210_e5035: f64 = (assign4210_e5033 - 0.02);
        var_vdsx = assign4210_e5035;
        var_vdsx_dn5 = (((var_vds_dn5 * var_vds) + (var_vds * var_vds_dn5)) / (2.0 * assign4210_e5033));
        var_vdsx_dn6 = (((var_vds_dn6 * var_vds) + (var_vds * var_vds_dn6)) / (2.0 * assign4210_e5033));

        let assign4220_e5039: f64 = (var_vdsx - var_vds);
        let assign4220_e5040: f64 = (0.5 * assign4220_e5039);
        var_symmetry_factor = assign4220_e5040;
        var_symmetry_factor_dn5 = (0.5 * (var_vdsx_dn5 - var_vds_dn5));
        var_symmetry_factor_dn6 = (0.5 * (var_vdsx_dn6 - var_vds_dn6));

        let assign4230_e5043: f64 = (var_vbgs + var_symmetry_factor);
        var_vbgx = assign4230_e5043;
        var_vbgx_dn3 = var_vbgs_dn3;
        var_vbgx_dn5 = (var_vbgs_dn5 + var_symmetry_factor_dn5);
        var_vbgx_dn6 = (var_vbgs_dn6 + var_symmetry_factor_dn6);

        let assign4240_e5046: f64 = (var_vfgs - var_deltaphi1);
        var_vgfb1 = assign4240_e5046;
        var_vgfb1_dn4 = (-var_deltaphi1_dn4);
        var_vgfb1_dn5 = var_vfgs_dn5;
        var_vgfb1_dn6 = var_vfgs_dn6;
        var_vgfb1_dn8 = var_vfgs_dn8;

        let assign4250_e5049: f64 = (var_vbgs - var_deltaphi2);
        var_vgfb2 = assign4250_e5049;
        var_vgfb2_dn3 = (var_vbgs_dn3 - var_deltaphi2_dn3);
        var_vgfb2_dn4 = (-var_deltaphi2_dn4);
        var_vgfb2_dn5 = (var_vbgs_dn5 - var_deltaphi2_dn5);
        var_vgfb2_dn6 = (var_vbgs_dn6 - var_deltaphi2_dn6);
        var_vgfb2_dn7 = (-var_deltaphi2_dn7);
        var_vgfb2_dn8 = (-var_deltaphi2_dn8);

        let assign4260_e5052: f64 = (var_epsratio * p.p49);
        let assign4260_e5054: f64 = (assign4260_e5052 * p.p45);
        let assign4260_e5055: f64 = (assign4260_e5054).sqrt();
        var_sclf = assign4260_e5055;

        let assign4270_e5059: f64 = (var_epsratio * p.p45);
        let assign4270_e5062: f64 = (0.375 * p.p49);
        let assign4270_e5063: f64 = (assign4270_e5059 + assign4270_e5062);
        let assign4270_e5064: f64 = (p.p49 * assign4270_e5063);
        let assign4270_e5065: f64 = (assign4270_e5064).sqrt();
        var_sclm = assign4270_e5065;

        let assign4280_e5069: f64 = (p.p46 * var_epsratio);
        let assign4280_e5070: f64 = (var_vgfb1 * assign4280_e5069);
        let assign4280_e5074: f64 = (p.p45 * var_epsratio);
        let assign4280_e5076: f64 = (assign4280_e5074 + p.p49);
        let assign4280_e5077: f64 = (var_vgfb2 * assign4280_e5076);
        let assign4280_e5078: f64 = (assign4280_e5070 + assign4280_e5077);
        let assign4280_e5080: f64 = (assign4280_e5078 / var_teff);
        let assign4280_e5082: f64 = (assign4280_e5080 + var_symmetry_factor);
        var_t0 = assign4280_e5082;
        var_t0_dn3 = ((var_vgfb2_dn3 * assign4280_e5076) / var_teff);
        var_t0_dn4 = (((var_vgfb1_dn4 * assign4280_e5069) + (var_vgfb2_dn4 * assign4280_e5076)) / var_teff);
        var_t0_dn5 = ((((var_vgfb1_dn5 * assign4280_e5069) + (var_vgfb2_dn5 * assign4280_e5076)) / var_teff) + var_symmetry_factor_dn5);
        var_t0_dn6 = ((((var_vgfb1_dn6 * assign4280_e5069) + (var_vgfb2_dn6 * assign4280_e5076)) / var_teff) + var_symmetry_factor_dn6);
        var_t0_dn7 = ((var_vgfb2_dn7 * assign4280_e5076) / var_teff);
        var_t0_dn8 = (((var_vgfb1_dn8 * assign4280_e5069) + (var_vgfb2_dn8 * assign4280_e5076)) / var_teff);

        let assign4290_e5086: f64 = (var_bscl_i * var_t0);
        let assign4290_e5087: f64 = (var_ascl_i + assign4290_e5086);
        let assign4290_e5088: f64 = (assign4290_e5087).atan();
        let assign4290_e5090: f64 = (assign4290_e5088 / 3.141592653589793);
        let assign4290_e5092: f64 = (assign4290_e5090 + 0.5);
        var_t1 = assign4290_e5092;
        var_t1_dn3 = (((var_bscl_i * var_t0_dn3) / (1.0 + (assign4290_e5087 * assign4290_e5087))) / 3.141592653589793);
        var_t1_dn4 = (((var_bscl_i * var_t0_dn4) / (1.0 + (assign4290_e5087 * assign4290_e5087))) / 3.141592653589793);
        var_t1_dn5 = (((var_bscl_i * var_t0_dn5) / (1.0 + (assign4290_e5087 * assign4290_e5087))) / 3.141592653589793);
        var_t1_dn6 = (((var_bscl_i * var_t0_dn6) / (1.0 + (assign4290_e5087 * assign4290_e5087))) / 3.141592653589793);
        var_t1_dn7 = (((var_bscl_i * var_t0_dn7) / (1.0 + (assign4290_e5087 * assign4290_e5087))) / 3.141592653589793);
        var_t1_dn8 = (((var_bscl_i * var_t0_dn8) / (1.0 + (assign4290_e5087 * assign4290_e5087))) / 3.141592653589793);

        let assign4300_e5097: f64 = (var_sclf - var_sclm);
        let assign4300_e5098: f64 = (var_t1 * assign4300_e5097);
        let assign4300_e5099: f64 = (var_sclm + assign4300_e5098);
        var_scl = assign4300_e5099;
        var_scl_dn3 = (var_t1_dn3 * assign4300_e5097);
        var_scl_dn4 = (var_t1_dn4 * assign4300_e5097);
        var_scl_dn5 = (var_t1_dn5 * assign4300_e5097);
        var_scl_dn6 = (var_t1_dn6 * assign4300_e5097);
        var_scl_dn7 = (var_t1_dn7 * assign4300_e5097);
        var_scl_dn8 = (var_t1_dn8 * assign4300_e5097);

        let assign4310_e5102: f64 = (var_dvt1_i * var_leff);
        let assign4310_e5104: f64 = (assign4310_e5102 / var_scl);
        let assign4310_e5106: f64 = (assign4310_e5104 + 1e-6);
        var_tmp = assign4310_e5106;
        var_tmp_dn3 = (-((assign4310_e5102 * var_scl_dn3) / (var_scl * var_scl)));
        var_tmp_dn4 = (-((assign4310_e5102 * var_scl_dn4) / (var_scl * var_scl)));
        var_tmp_dn5 = (-((assign4310_e5102 * var_scl_dn5) / (var_scl * var_scl)));
        var_tmp_dn6 = (-((assign4310_e5102 * var_scl_dn6) / (var_scl * var_scl)));
        var_tmp_dn7 = (-((assign4310_e5102 * var_scl_dn7) / (var_scl * var_scl)));
        var_tmp_dn8 = (-((assign4310_e5102 * var_scl_dn8) / (var_scl * var_scl)));

        let assign4320_e5109: f64 = if var_tmp < 40.0 { 1.0 } else { 0.0 };
        var_guard67 = assign4320_e5109;

        let (assign4330_e5118, assign4330_e5118_d_n3, assign4330_e5118_d_n4, assign4330_e5118_d_n5, assign4330_e5118_d_n6, assign4330_e5118_d_n7, assign4330_e5118_d_n8,) = {
    if (var_guard67 != 0.0) {
        let assign4330_e5113: f64 = (var_tmp).cosh();
        let assign4330_e5115: f64 = (assign4330_e5113 - 1.0);
        let assign4330_e5116: f64 = (0.5 / assign4330_e5115);
        (assign4330_e5116, (-((0.5 * ((var_tmp).sinh() * var_tmp_dn3)) / (assign4330_e5115 * assign4330_e5115))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn4)) / (assign4330_e5115 * assign4330_e5115))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn5)) / (assign4330_e5115 * assign4330_e5115))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn6)) / (assign4330_e5115 * assign4330_e5115))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn7)) / (assign4330_e5115 * assign4330_e5115))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn8)) / (assign4330_e5115 * assign4330_e5115))),)
    } else {
        (var_theta_sce, var_theta_sce_dn3, var_theta_sce_dn4, var_theta_sce_dn5, var_theta_sce_dn6, var_theta_sce_dn7, var_theta_sce_dn8,)
    }
};
        var_theta_sce = assign4330_e5118;
        var_theta_sce_dn3 = assign4330_e5118_d_n3;
        var_theta_sce_dn4 = assign4330_e5118_d_n4;
        var_theta_sce_dn5 = assign4330_e5118_d_n5;
        var_theta_sce_dn6 = assign4330_e5118_d_n6;
        var_theta_sce_dn7 = assign4330_e5118_d_n7;
        var_theta_sce_dn8 = assign4330_e5118_d_n8;

        let (assign4340_e5125, assign4340_e5125_d_n3, assign4340_e5125_d_n4, assign4340_e5125_d_n5, assign4340_e5125_d_n6, assign4340_e5125_d_n7, assign4340_e5125_d_n8,) = {
    if (var_guard67 == 0.0) {
        let assign4340_e5122: f64 = (-var_tmp);
        let assign4340_e5123: f64 = { let limited_exp_arg = assign4340_e5122; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign4340_e5123, ({ let limited_exp_arg = assign4340_e5122; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn3)), ({ let limited_exp_arg = assign4340_e5122; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn4)), ({ let limited_exp_arg = assign4340_e5122; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn5)), ({ let limited_exp_arg = assign4340_e5122; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn6)), ({ let limited_exp_arg = assign4340_e5122; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn7)), ({ let limited_exp_arg = assign4340_e5122; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn8)),)
    } else {
        (var_theta_sce, var_theta_sce_dn3, var_theta_sce_dn4, var_theta_sce_dn5, var_theta_sce_dn6, var_theta_sce_dn7, var_theta_sce_dn8,)
    }
};
        var_theta_sce = assign4340_e5125;
        var_theta_sce_dn3 = assign4340_e5125_d_n3;
        var_theta_sce_dn4 = assign4340_e5125_d_n4;
        var_theta_sce_dn5 = assign4340_e5125_d_n5;
        var_theta_sce_dn6 = assign4340_e5125_d_n6;
        var_theta_sce_dn7 = assign4340_e5125_d_n7;
        var_theta_sce_dn8 = assign4340_e5125_d_n8;

        let assign4350_e5128: f64 = (var_dsub_i * var_leff);
        let assign4350_e5130: f64 = (assign4350_e5128 / var_scl);
        let assign4350_e5132: f64 = (assign4350_e5130 + 1e-6);
        var_tmp = assign4350_e5132;
        var_tmp_dn3 = (-((assign4350_e5128 * var_scl_dn3) / (var_scl * var_scl)));
        var_tmp_dn4 = (-((assign4350_e5128 * var_scl_dn4) / (var_scl * var_scl)));
        var_tmp_dn5 = (-((assign4350_e5128 * var_scl_dn5) / (var_scl * var_scl)));
        var_tmp_dn6 = (-((assign4350_e5128 * var_scl_dn6) / (var_scl * var_scl)));
        var_tmp_dn7 = (-((assign4350_e5128 * var_scl_dn7) / (var_scl * var_scl)));
        var_tmp_dn8 = (-((assign4350_e5128 * var_scl_dn8) / (var_scl * var_scl)));

        let assign4360_e5135: f64 = if var_tmp < 40.0 { 1.0 } else { 0.0 };
        var_guard68 = assign4360_e5135;

        let (assign4370_e5144, assign4370_e5144_d_n3, assign4370_e5144_d_n4, assign4370_e5144_d_n5, assign4370_e5144_d_n6, assign4370_e5144_d_n7, assign4370_e5144_d_n8,) = {
    if (var_guard68 != 0.0) {
        let assign4370_e5139: f64 = (var_tmp).cosh();
        let assign4370_e5141: f64 = (assign4370_e5139 - 1.0);
        let assign4370_e5142: f64 = (0.5 / assign4370_e5141);
        (assign4370_e5142, (-((0.5 * ((var_tmp).sinh() * var_tmp_dn3)) / (assign4370_e5141 * assign4370_e5141))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn4)) / (assign4370_e5141 * assign4370_e5141))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn5)) / (assign4370_e5141 * assign4370_e5141))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn6)) / (assign4370_e5141 * assign4370_e5141))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn7)) / (assign4370_e5141 * assign4370_e5141))), (-((0.5 * ((var_tmp).sinh() * var_tmp_dn8)) / (assign4370_e5141 * assign4370_e5141))),)
    } else {
        (var_theta_dibl, var_theta_dibl_dn3, var_theta_dibl_dn4, var_theta_dibl_dn5, var_theta_dibl_dn6, var_theta_dibl_dn7, var_theta_dibl_dn8,)
    }
};
        var_theta_dibl = assign4370_e5144;
        var_theta_dibl_dn3 = assign4370_e5144_d_n3;
        var_theta_dibl_dn4 = assign4370_e5144_d_n4;
        var_theta_dibl_dn5 = assign4370_e5144_d_n5;
        var_theta_dibl_dn6 = assign4370_e5144_d_n6;
        var_theta_dibl_dn7 = assign4370_e5144_d_n7;
        var_theta_dibl_dn8 = assign4370_e5144_d_n8;

        let (assign4380_e5151, assign4380_e5151_d_n3, assign4380_e5151_d_n4, assign4380_e5151_d_n5, assign4380_e5151_d_n6, assign4380_e5151_d_n7, assign4380_e5151_d_n8,) = {
    if (var_guard68 == 0.0) {
        let assign4380_e5148: f64 = (-var_tmp);
        let assign4380_e5149: f64 = { let limited_exp_arg = assign4380_e5148; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign4380_e5149, ({ let limited_exp_arg = assign4380_e5148; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn3)), ({ let limited_exp_arg = assign4380_e5148; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn4)), ({ let limited_exp_arg = assign4380_e5148; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn5)), ({ let limited_exp_arg = assign4380_e5148; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn6)), ({ let limited_exp_arg = assign4380_e5148; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn7)), ({ let limited_exp_arg = assign4380_e5148; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn8)),)
    } else {
        (var_theta_dibl, var_theta_dibl_dn3, var_theta_dibl_dn4, var_theta_dibl_dn5, var_theta_dibl_dn6, var_theta_dibl_dn7, var_theta_dibl_dn8,)
    }
};
        var_theta_dibl = assign4380_e5151;
        var_theta_dibl_dn3 = assign4380_e5151_d_n3;
        var_theta_dibl_dn4 = assign4380_e5151_d_n4;
        var_theta_dibl_dn5 = assign4380_e5151_d_n5;
        var_theta_dibl_dn6 = assign4380_e5151_d_n6;
        var_theta_dibl_dn7 = assign4380_e5151_d_n7;
        var_theta_dibl_dn8 = assign4380_e5151_d_n8;

        let assign4390_e5154: f64 = if var_tmp < 40.0 { 1.0 } else { 0.0 };
        var_guard69 = assign4390_e5154;

        let (assign4400_e5169, assign4400_e5169_d_n3, assign4400_e5169_d_n4, assign4400_e5169_d_n5, assign4400_e5169_d_n6, assign4400_e5169_d_n7, assign4400_e5169_d_n8,) = {
    if (var_guard69 != 0.0) {
        let assign4400_e5160: f64 = (var_tmp).cosh();
        let assign4400_e5162: f64 = (assign4400_e5160 - 2.0);
        let assign4400_e5163: f64 = (p.p83 * assign4400_e5162);
        let assign4400_e5164: f64 = (1.0 + assign4400_e5163);
        let assign4400_e5166: f64 = (assign4400_e5164).max(1e-6);
        let assign4400_e5167: f64 = (1.0 / assign4400_e5166);
        (assign4400_e5167, (-(if assign4400_e5164 >= 1e-6 { (p.p83 * ((var_tmp).sinh() * var_tmp_dn3)) } else { 0.0 } / (assign4400_e5166 * assign4400_e5166))), (-(if assign4400_e5164 >= 1e-6 { (p.p83 * ((var_tmp).sinh() * var_tmp_dn4)) } else { 0.0 } / (assign4400_e5166 * assign4400_e5166))), (-(if assign4400_e5164 >= 1e-6 { (p.p83 * ((var_tmp).sinh() * var_tmp_dn5)) } else { 0.0 } / (assign4400_e5166 * assign4400_e5166))), (-(if assign4400_e5164 >= 1e-6 { (p.p83 * ((var_tmp).sinh() * var_tmp_dn6)) } else { 0.0 } / (assign4400_e5166 * assign4400_e5166))), (-(if assign4400_e5164 >= 1e-6 { (p.p83 * ((var_tmp).sinh() * var_tmp_dn7)) } else { 0.0 } / (assign4400_e5166 * assign4400_e5166))), (-(if assign4400_e5164 >= 1e-6 { (p.p83 * ((var_tmp).sinh() * var_tmp_dn8)) } else { 0.0 } / (assign4400_e5166 * assign4400_e5166))),)
    } else {
        (var_theta_dits, var_theta_dits_dn3, var_theta_dits_dn4, var_theta_dits_dn5, var_theta_dits_dn6, var_theta_dits_dn7, var_theta_dits_dn8,)
    }
};
        var_theta_dits = assign4400_e5169;
        var_theta_dits_dn3 = assign4400_e5169_d_n3;
        var_theta_dits_dn4 = assign4400_e5169_d_n4;
        var_theta_dits_dn5 = assign4400_e5169_d_n5;
        var_theta_dits_dn6 = assign4400_e5169_d_n6;
        var_theta_dits_dn7 = assign4400_e5169_d_n7;
        var_theta_dits_dn8 = assign4400_e5169_d_n8;

        let (assign4410_e5184, assign4410_e5184_d_n3, assign4410_e5184_d_n4, assign4410_e5184_d_n5, assign4410_e5184_d_n6, assign4410_e5184_d_n7, assign4410_e5184_d_n8,) = {
    if (var_guard69 == 0.0) {
        let assign4410_e5173: f64 = (-var_tmp);
        let assign4410_e5174: f64 = { let limited_exp_arg = assign4410_e5173; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4410_e5176: f64 = (-var_tmp);
        let assign4410_e5177: f64 = { let limited_exp_arg = assign4410_e5176; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4410_e5179: f64 = (assign4410_e5177 + p.p83);
        let assign4410_e5181: f64 = (assign4410_e5179).max(1e-6);
        let assign4410_e5182: f64 = (assign4410_e5174 / assign4410_e5181);
        (assign4410_e5182, (((({ let limited_exp_arg = assign4410_e5173; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn3)) * assign4410_e5181) - (assign4410_e5174 * if assign4410_e5179 >= 1e-6 { ({ let limited_exp_arg = assign4410_e5176; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn3)) } else { 0.0 })) / (assign4410_e5181 * assign4410_e5181)), (((({ let limited_exp_arg = assign4410_e5173; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn4)) * assign4410_e5181) - (assign4410_e5174 * if assign4410_e5179 >= 1e-6 { ({ let limited_exp_arg = assign4410_e5176; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn4)) } else { 0.0 })) / (assign4410_e5181 * assign4410_e5181)), (((({ let limited_exp_arg = assign4410_e5173; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn5)) * assign4410_e5181) - (assign4410_e5174 * if assign4410_e5179 >= 1e-6 { ({ let limited_exp_arg = assign4410_e5176; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn5)) } else { 0.0 })) / (assign4410_e5181 * assign4410_e5181)), (((({ let limited_exp_arg = assign4410_e5173; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn6)) * assign4410_e5181) - (assign4410_e5174 * if assign4410_e5179 >= 1e-6 { ({ let limited_exp_arg = assign4410_e5176; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn6)) } else { 0.0 })) / (assign4410_e5181 * assign4410_e5181)), (((({ let limited_exp_arg = assign4410_e5173; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn7)) * assign4410_e5181) - (assign4410_e5174 * if assign4410_e5179 >= 1e-6 { ({ let limited_exp_arg = assign4410_e5176; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn7)) } else { 0.0 })) / (assign4410_e5181 * assign4410_e5181)), (((({ let limited_exp_arg = assign4410_e5173; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn8)) * assign4410_e5181) - (assign4410_e5174 * if assign4410_e5179 >= 1e-6 { ({ let limited_exp_arg = assign4410_e5176; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn8)) } else { 0.0 })) / (assign4410_e5181 * assign4410_e5181)),)
    } else {
        (var_theta_dits, var_theta_dits_dn3, var_theta_dits_dn4, var_theta_dits_dn5, var_theta_dits_dn6, var_theta_dits_dn7, var_theta_dits_dn8,)
    }
};
        var_theta_dits = assign4410_e5184;
        var_theta_dits_dn3 = assign4410_e5184_d_n3;
        var_theta_dits_dn4 = assign4410_e5184_d_n4;
        var_theta_dits_dn5 = assign4410_e5184_d_n5;
        var_theta_dits_dn6 = assign4410_e5184_d_n6;
        var_theta_dits_dn7 = assign4410_e5184_d_n7;
        var_theta_dits_dn8 = assign4410_e5184_d_n8;

        let assign4420_e5187: f64 = (var_drout_i * var_leff);
        let assign4420_e5189: f64 = (assign4420_e5187 / var_scl);
        let assign4420_e5191: f64 = (assign4420_e5189 + 1e-6);
        var_tmp = assign4420_e5191;
        var_tmp_dn3 = (-((assign4420_e5187 * var_scl_dn3) / (var_scl * var_scl)));
        var_tmp_dn4 = (-((assign4420_e5187 * var_scl_dn4) / (var_scl * var_scl)));
        var_tmp_dn5 = (-((assign4420_e5187 * var_scl_dn5) / (var_scl * var_scl)));
        var_tmp_dn6 = (-((assign4420_e5187 * var_scl_dn6) / (var_scl * var_scl)));
        var_tmp_dn7 = (-((assign4420_e5187 * var_scl_dn7) / (var_scl * var_scl)));
        var_tmp_dn8 = (-((assign4420_e5187 * var_scl_dn8) / (var_scl * var_scl)));

        let assign4430_e5194: f64 = if var_tmp < 40.0 { 1.0 } else { 0.0 };
        var_guard70 = assign4430_e5194;

        let (assign4440_e5207, assign4440_e5207_d_n3, assign4440_e5207_d_n4, assign4440_e5207_d_n5, assign4440_e5207_d_n6, assign4440_e5207_d_n7, assign4440_e5207_d_n8,) = {
    if (var_guard70 != 0.0) {
        let assign4440_e5198: f64 = (0.5 * var_pdibl1_i);
        let assign4440_e5200: f64 = (var_tmp).cosh();
        let assign4440_e5202: f64 = (assign4440_e5200 - 1.0);
        let assign4440_e5203: f64 = (assign4440_e5198 / assign4440_e5202);
        let assign4440_e5205: f64 = (assign4440_e5203 + var_pdibl2_i);
        (assign4440_e5205, (-((assign4440_e5198 * ((var_tmp).sinh() * var_tmp_dn3)) / (assign4440_e5202 * assign4440_e5202))), (-((assign4440_e5198 * ((var_tmp).sinh() * var_tmp_dn4)) / (assign4440_e5202 * assign4440_e5202))), (-((assign4440_e5198 * ((var_tmp).sinh() * var_tmp_dn5)) / (assign4440_e5202 * assign4440_e5202))), (-((assign4440_e5198 * ((var_tmp).sinh() * var_tmp_dn6)) / (assign4440_e5202 * assign4440_e5202))), (-((assign4440_e5198 * ((var_tmp).sinh() * var_tmp_dn7)) / (assign4440_e5202 * assign4440_e5202))), (-((assign4440_e5198 * ((var_tmp).sinh() * var_tmp_dn8)) / (assign4440_e5202 * assign4440_e5202))),)
    } else {
        (var_diblfactor, var_diblfactor_dn3, var_diblfactor_dn4, var_diblfactor_dn5, var_diblfactor_dn6, var_diblfactor_dn7, var_diblfactor_dn8,)
    }
};
        var_diblfactor = assign4440_e5207;
        var_diblfactor_dn3 = assign4440_e5207_d_n3;
        var_diblfactor_dn4 = assign4440_e5207_d_n4;
        var_diblfactor_dn5 = assign4440_e5207_d_n5;
        var_diblfactor_dn6 = assign4440_e5207_d_n6;
        var_diblfactor_dn7 = assign4440_e5207_d_n7;
        var_diblfactor_dn8 = assign4440_e5207_d_n8;

        let (assign4450_e5218, assign4450_e5218_d_n3, assign4450_e5218_d_n4, assign4450_e5218_d_n5, assign4450_e5218_d_n6, assign4450_e5218_d_n7, assign4450_e5218_d_n8,) = {
    if (var_guard70 == 0.0) {
        let assign4450_e5212: f64 = (-var_tmp);
        let assign4450_e5213: f64 = { let limited_exp_arg = assign4450_e5212; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4450_e5214: f64 = (var_pdibl1_i * assign4450_e5213);
        let assign4450_e5216: f64 = (assign4450_e5214 + var_pdibl2_i);
        (assign4450_e5216, (var_pdibl1_i * ({ let limited_exp_arg = assign4450_e5212; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn3))), (var_pdibl1_i * ({ let limited_exp_arg = assign4450_e5212; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn4))), (var_pdibl1_i * ({ let limited_exp_arg = assign4450_e5212; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn5))), (var_pdibl1_i * ({ let limited_exp_arg = assign4450_e5212; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn6))), (var_pdibl1_i * ({ let limited_exp_arg = assign4450_e5212; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn7))), (var_pdibl1_i * ({ let limited_exp_arg = assign4450_e5212; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_tmp_dn8))),)
    } else {
        (var_diblfactor, var_diblfactor_dn3, var_diblfactor_dn4, var_diblfactor_dn5, var_diblfactor_dn6, var_diblfactor_dn7, var_diblfactor_dn8,)
    }
};
        var_diblfactor = assign4450_e5218;
        var_diblfactor_dn3 = assign4450_e5218_d_n3;
        var_diblfactor_dn4 = assign4450_e5218_d_n4;
        var_diblfactor_dn5 = assign4450_e5218_d_n5;
        var_diblfactor_dn6 = assign4450_e5218_d_n6;
        var_diblfactor_dn7 = assign4450_e5218_d_n7;
        var_diblfactor_dn8 = assign4450_e5218_d_n8;

        let assign4460_e5221: f64 = (-1.0);
        let assign4460_e5222: f64 = if p.p13 == assign4460_e5221 { 1.0 } else { 0.0 };
        var_guard71 = assign4460_e5222;

        let (assign4470_e5230, assign4470_e5230_d_n3, assign4470_e5230_d_n4, assign4470_e5230_d_n5, assign4470_e5230_d_n6, assign4470_e5230_d_n7, assign4470_e5230_d_n8,) = {
    if (var_guard71 != 0.0) {
        let assign4470_e5226: f64 = (var_dbgpw_i * var_leff);
        let assign4470_e5228: f64 = (assign4470_e5226 / var_scl);
        (assign4470_e5228, (-((assign4470_e5226 * var_scl_dn3) / (var_scl * var_scl))), (-((assign4470_e5226 * var_scl_dn4) / (var_scl * var_scl))), (-((assign4470_e5226 * var_scl_dn5) / (var_scl * var_scl))), (-((assign4470_e5226 * var_scl_dn6) / (var_scl * var_scl))), (-((assign4470_e5226 * var_scl_dn7) / (var_scl * var_scl))), (-((assign4470_e5226 * var_scl_dn8) / (var_scl * var_scl))),)
    } else {
        (var_temp, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8,)
    }
};
        var_temp = assign4470_e5230;
        var_temp_dn3 = assign4470_e5230_d_n3;
        var_temp_dn4 = assign4470_e5230_d_n4;
        var_temp_dn5 = assign4470_e5230_d_n5;
        var_temp_dn6 = assign4470_e5230_d_n6;
        var_temp_dn7 = assign4470_e5230_d_n7;
        var_temp_dn8 = assign4470_e5230_d_n8;

        let assign4480_e5233: f64 = if var_temp > 40.0 { 1.0 } else { 0.0 };
        var_guard72 = assign4480_e5233;

        let (assign4490_e5242, assign4490_e5242_d_n3, assign4490_e5242_d_n4, assign4490_e5242_d_n5, assign4490_e5242_d_n6, assign4490_e5242_d_n7, assign4490_e5242_d_n8,) = {
    if ((var_guard71 != 0.0) && (var_guard72 != 0.0)) {
        let assign4490_e5238: f64 = { let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4490_e5240: f64 = (assign4490_e5238 / 2.0);
        (assign4490_e5240, (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn3) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn4) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn5) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn6) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn7) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn8) / 2.0),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign4490_e5242;
        var_t0_dn3 = assign4490_e5242_d_n3;
        var_t0_dn4 = assign4490_e5242_d_n4;
        var_t0_dn5 = assign4490_e5242_d_n5;
        var_t0_dn6 = assign4490_e5242_d_n6;
        var_t0_dn7 = assign4490_e5242_d_n7;
        var_t0_dn8 = assign4490_e5242_d_n8;

        let (assign4500_e5252, assign4500_e5252_d_n3, assign4500_e5252_d_n4, assign4500_e5252_d_n5, assign4500_e5252_d_n6, assign4500_e5252_d_n7, assign4500_e5252_d_n8,) = {
    if ((var_guard71 != 0.0) && (var_guard72 == 0.0)) {
        let assign4500_e5248: f64 = (var_temp).cosh();
        let assign4500_e5250: f64 = (assign4500_e5248 - 1.0);
        (assign4500_e5250, ((var_temp).sinh() * var_temp_dn3), ((var_temp).sinh() * var_temp_dn4), ((var_temp).sinh() * var_temp_dn5), ((var_temp).sinh() * var_temp_dn6), ((var_temp).sinh() * var_temp_dn7), ((var_temp).sinh() * var_temp_dn8),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign4500_e5252;
        var_t0_dn3 = assign4500_e5252_d_n3;
        var_t0_dn4 = assign4500_e5252_d_n4;
        var_t0_dn5 = assign4500_e5252_d_n5;
        var_t0_dn6 = assign4500_e5252_d_n6;
        var_t0_dn7 = assign4500_e5252_d_n7;
        var_t0_dn8 = assign4500_e5252_d_n8;

        *var_diblfactor_slot = var_diblfactor;
        *var_diblfactor_dn3_slot = var_diblfactor_dn3;
        *var_diblfactor_dn4_slot = var_diblfactor_dn4;
        *var_diblfactor_dn5_slot = var_diblfactor_dn5;
        *var_diblfactor_dn6_slot = var_diblfactor_dn6;
        *var_diblfactor_dn7_slot = var_diblfactor_dn7;
        *var_diblfactor_dn8_slot = var_diblfactor_dn8;
        *var_guard66_slot = var_guard66;
        *var_guard67_slot = var_guard67;
        *var_guard68_slot = var_guard68;
        *var_guard69_slot = var_guard69;
        *var_guard70_slot = var_guard70;
        *var_guard71_slot = var_guard71;
        *var_guard72_slot = var_guard72;
        *var_igsd_mult_slot = var_igsd_mult;
        *var_igsd_mult_dn3_slot = var_igsd_mult_dn3;
        *var_igsd_mult_dn4_slot = var_igsd_mult_dn4;
        *var_igsd_mult_dn5_slot = var_igsd_mult_dn5;
        *var_igsd_mult_dn6_slot = var_igsd_mult_dn6;
        *var_igsd_mult_dn7_slot = var_igsd_mult_dn7;
        *var_igsd_mult_dn8_slot = var_igsd_mult_dn8;
        *var_scl_slot = var_scl;
        *var_scl_dn3_slot = var_scl_dn3;
        *var_scl_dn4_slot = var_scl_dn4;
        *var_scl_dn5_slot = var_scl_dn5;
        *var_scl_dn6_slot = var_scl_dn6;
        *var_scl_dn7_slot = var_scl_dn7;
        *var_scl_dn8_slot = var_scl_dn8;
        *var_sclf_slot = var_sclf;
        *var_sclm_slot = var_sclm;
        *var_sigvds_slot = var_sigvds;
        *var_symmetry_factor_slot = var_symmetry_factor;
        *var_symmetry_factor_dn5_slot = var_symmetry_factor_dn5;
        *var_symmetry_factor_dn6_slot = var_symmetry_factor_dn6;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_temp_slot = var_temp;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_theta_dibl_slot = var_theta_dibl;
        *var_theta_dibl_dn3_slot = var_theta_dibl_dn3;
        *var_theta_dibl_dn4_slot = var_theta_dibl_dn4;
        *var_theta_dibl_dn5_slot = var_theta_dibl_dn5;
        *var_theta_dibl_dn6_slot = var_theta_dibl_dn6;
        *var_theta_dibl_dn7_slot = var_theta_dibl_dn7;
        *var_theta_dibl_dn8_slot = var_theta_dibl_dn8;
        *var_theta_dits_slot = var_theta_dits;
        *var_theta_dits_dn3_slot = var_theta_dits_dn3;
        *var_theta_dits_dn4_slot = var_theta_dits_dn4;
        *var_theta_dits_dn5_slot = var_theta_dits_dn5;
        *var_theta_dits_dn6_slot = var_theta_dits_dn6;
        *var_theta_dits_dn7_slot = var_theta_dits_dn7;
        *var_theta_dits_dn8_slot = var_theta_dits_dn8;
        *var_theta_sce_slot = var_theta_sce;
        *var_theta_sce_dn3_slot = var_theta_sce_dn3;
        *var_theta_sce_dn4_slot = var_theta_sce_dn4;
        *var_theta_sce_dn5_slot = var_theta_sce_dn5;
        *var_theta_sce_dn6_slot = var_theta_sce_dn6;
        *var_theta_sce_dn7_slot = var_theta_sce_dn7;
        *var_theta_sce_dn8_slot = var_theta_sce_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn3_slot = var_tmp_dn3;
        *var_tmp_dn4_slot = var_tmp_dn4;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbgd_slot = var_vbgd;
        *var_vbgd_dn3_slot = var_vbgd_dn3;
        *var_vbgd_dn5_slot = var_vbgd_dn5;
        *var_vbgd_dn6_slot = var_vbgd_dn6;
        *var_vbgd_noswap_slot = var_vbgd_noswap;
        *var_vbgd_noswap_dn3_slot = var_vbgd_noswap_dn3;
        *var_vbgd_noswap_dn5_slot = var_vbgd_noswap_dn5;
        *var_vbgs_slot = var_vbgs;
        *var_vbgs_dn3_slot = var_vbgs_dn3;
        *var_vbgs_dn5_slot = var_vbgs_dn5;
        *var_vbgs_dn6_slot = var_vbgs_dn6;
        *var_vbgs_noswap_slot = var_vbgs_noswap;
        *var_vbgs_noswap_dn3_slot = var_vbgs_noswap_dn3;
        *var_vbgs_noswap_dn6_slot = var_vbgs_noswap_dn6;
        *var_vbgx_slot = var_vbgx;
        *var_vbgx_dn3_slot = var_vbgx_dn3;
        *var_vbgx_dn5_slot = var_vbgx_dn5;
        *var_vbgx_dn6_slot = var_vbgx_dn6;
        *var_vds_slot = var_vds;
        *var_vds_dn5_slot = var_vds_dn5;
        *var_vds_dn6_slot = var_vds_dn6;
        *var_vds_noswap_slot = var_vds_noswap;
        *var_vds_noswap_dn5_slot = var_vds_noswap_dn5;
        *var_vds_noswap_dn6_slot = var_vds_noswap_dn6;
        *var_vdsx_slot = var_vdsx;
        *var_vdsx_dn5_slot = var_vdsx_dn5;
        *var_vdsx_dn6_slot = var_vdsx_dn6;
        *var_vfgs_slot = var_vfgs;
        *var_vfgs_dn5_slot = var_vfgs_dn5;
        *var_vfgs_dn6_slot = var_vfgs_dn6;
        *var_vfgs_dn8_slot = var_vfgs_dn8;
        *var_vgbg_slot = var_vgbg;
        *var_vgbg_dn3_slot = var_vgbg_dn3;
        *var_vgbg_dn8_slot = var_vgbg_dn8;
        *var_vgd_noswap_slot = var_vgd_noswap;
        *var_vgd_noswap_dn5_slot = var_vgd_noswap_dn5;
        *var_vgd_noswap_dn8_slot = var_vgd_noswap_dn8;
        *var_vgd_ov_noswap_slot = var_vgd_ov_noswap;
        *var_vgd_ov_noswap_dn5_slot = var_vgd_ov_noswap_dn5;
        *var_vgd_ov_noswap_dn7_slot = var_vgd_ov_noswap_dn7;
        *var_vgfb1_slot = var_vgfb1;
        *var_vgfb1_dn4_slot = var_vgfb1_dn4;
        *var_vgfb1_dn5_slot = var_vgfb1_dn5;
        *var_vgfb1_dn6_slot = var_vgfb1_dn6;
        *var_vgfb1_dn8_slot = var_vgfb1_dn8;
        *var_vgfb2_slot = var_vgfb2;
        *var_vgfb2_dn3_slot = var_vgfb2_dn3;
        *var_vgfb2_dn4_slot = var_vgfb2_dn4;
        *var_vgfb2_dn5_slot = var_vgfb2_dn5;
        *var_vgfb2_dn6_slot = var_vgfb2_dn6;
        *var_vgfb2_dn7_slot = var_vgfb2_dn7;
        *var_vgfb2_dn8_slot = var_vgfb2_dn8;
        *var_vgs_noswap_slot = var_vgs_noswap;
        *var_vgs_noswap_dn6_slot = var_vgs_noswap_dn6;
        *var_vgs_noswap_dn8_slot = var_vgs_noswap_dn8;
        *var_vgs_ov_noswap_slot = var_vgs_ov_noswap;
        *var_vgs_ov_noswap_dn6_slot = var_vgs_ov_noswap_dn6;
        *var_vgs_ov_noswap_dn7_slot = var_vgs_ov_noswap_dn7;
    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        var_bpfactornw_i: f64,
        var_bpfactorpw_i: f64,
        var_cbgcbg_i: f64,
        var_cdsc_i: f64,
        var_cdscd_i: f64,
        var_cit_i: f64,
        var_cox1: f64,
        var_cox2: f64,
        var_csi: f64,
        var_dbgnw_i: f64,
        var_devsign: f64,
        var_dsc0_i: f64,
        var_dsc1_i: f64,
        var_dvt0_i: f64,
        var_dvtp0_i: f64,
        var_dvtp1_i: f64,
        var_epsratio: f64,
        var_epssi: f64,
        var_eta0_t: f64,
        var_eta0_t_dn4: f64,
        var_eta1_i: f64,
        var_etab_i: f64,
        var_guard71: f64,
        var_k1rsce_i: f64,
        var_kbg0nw_i: f64,
        var_kbg0pw_i: f64,
        var_kbg1nw_i: f64,
        var_kbg1pw_i: f64,
        var_kbg2nw_i: f64,
        var_kbg2pw_i: f64,
        var_leff: f64,
        var_nbody_i: f64,
        var_phib: f64,
        var_phib_dn3: f64,
        var_phib_dn4: f64,
        var_phib_dn5: f64,
        var_phib_dn6: f64,
        var_phib_dn7: f64,
        var_phib_dn8: f64,
        var_phin_i: f64,
        var_scl: f64,
        var_scl_dn3: f64,
        var_scl_dn4: f64,
        var_scl_dn5: f64,
        var_scl_dn6: f64,
        var_scl_dn7: f64,
        var_scl_dn8: f64,
        var_symmetry_factor: f64,
        var_symmetry_factor_dn5: f64,
        var_symmetry_factor_dn6: f64,
        var_theta_dibl: f64,
        var_theta_dibl_dn3: f64,
        var_theta_dibl_dn4: f64,
        var_theta_dibl_dn5: f64,
        var_theta_dibl_dn6: f64,
        var_theta_dibl_dn7: f64,
        var_theta_dibl_dn8: f64,
        var_theta_dits: f64,
        var_theta_dits_dn3: f64,
        var_theta_dits_dn4: f64,
        var_theta_dits_dn5: f64,
        var_theta_dits_dn6: f64,
        var_theta_dits_dn7: f64,
        var_theta_dits_dn8: f64,
        var_theta_rsce: f64,
        var_theta_sce: f64,
        var_theta_sce_dn3: f64,
        var_theta_sce_dn4: f64,
        var_theta_sce_dn5: f64,
        var_theta_sce_dn6: f64,
        var_theta_sce_dn7: f64,
        var_theta_sce_dn8: f64,
        var_vbgx: f64,
        var_vbgx_dn3: f64,
        var_vbgx_dn5: f64,
        var_vbgx_dn6: f64,
        var_vbi: f64,
        var_vbi_dn3: f64,
        var_vbi_dn4: f64,
        var_vbi_dn5: f64,
        var_vbi_dn6: f64,
        var_vbi_dn7: f64,
        var_vbi_dn8: f64,
        var_vdsx: f64,
        var_vdsx_dn5: f64,
        var_vdsx_dn6: f64,
        var_vgfb2: f64,
        var_vgfb2_dn3: f64,
        var_vgfb2_dn4: f64,
        var_vgfb2_dn5: f64,
        var_vgfb2_dn6: f64,
        var_vgfb2_dn7: f64,
        var_vgfb2_dn8: f64,
        var_vknee1nw_i: f64,
        var_vknee1pw_i: f64,
        var_vknee2nw_i: f64,
        var_vknee2pw_i: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_welsign: f64,
        var_bpfactor_slot: &mut f64,
        var_dvth_dibl_slot: &mut f64,
        var_dvth_dibl_dn3_slot: &mut f64,
        var_dvth_dibl_dn4_slot: &mut f64,
        var_dvth_dibl_dn5_slot: &mut f64,
        var_dvth_dibl_dn6_slot: &mut f64,
        var_dvth_dibl_dn7_slot: &mut f64,
        var_dvth_dibl_dn8_slot: &mut f64,
        var_dvth_dsc_slot: &mut f64,
        var_dvth_dsc_dn5_slot: &mut f64,
        var_dvth_dsc_dn6_slot: &mut f64,
        var_dvth_nbody_slot: &mut f64,
        var_dvth_rsce_slot: &mut f64,
        var_dvth_rsce_dn3_slot: &mut f64,
        var_dvth_rsce_dn4_slot: &mut f64,
        var_dvth_rsce_dn5_slot: &mut f64,
        var_dvth_rsce_dn6_slot: &mut f64,
        var_dvth_rsce_dn7_slot: &mut f64,
        var_dvth_rsce_dn8_slot: &mut f64,
        var_dvth_vbg_slot: &mut f64,
        var_dvth_vbg_dn3_slot: &mut f64,
        var_dvth_vbg_dn4_slot: &mut f64,
        var_dvth_vbg_dn5_slot: &mut f64,
        var_dvth_vbg_dn6_slot: &mut f64,
        var_dvth_vbg_dn7_slot: &mut f64,
        var_dvth_vbg_dn8_slot: &mut f64,
        var_dvth_vtroll_slot: &mut f64,
        var_dvth_vtroll_dn3_slot: &mut f64,
        var_dvth_vtroll_dn4_slot: &mut f64,
        var_dvth_vtroll_dn5_slot: &mut f64,
        var_dvth_vtroll_dn6_slot: &mut f64,
        var_dvth_vtroll_dn7_slot: &mut f64,
        var_dvth_vtroll_dn8_slot: &mut f64,
        var_gamma0_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_kvbg_slot: &mut f64,
        var_kvbg_dn3_slot: &mut f64,
        var_kvbg_dn4_slot: &mut f64,
        var_kvbg_dn5_slot: &mut f64,
        var_kvbg_dn6_slot: &mut f64,
        var_kvbg_dn7_slot: &mut f64,
        var_kvbg_dn8_slot: &mut f64,
        var_nvtm_slot: &mut f64,
        var_nvtm_dn3_slot: &mut f64,
        var_nvtm_dn4_slot: &mut f64,
        var_nvtm_dn5_slot: &mut f64,
        var_nvtm_dn6_slot: &mut f64,
        var_nvtm_dn7_slot: &mut f64,
        var_nvtm_dn8_slot: &mut f64,
        var_phist_slot: &mut f64,
        var_phist_dn3_slot: &mut f64,
        var_phist_dn4_slot: &mut f64,
        var_phist_dn5_slot: &mut f64,
        var_phist_dn6_slot: &mut f64,
        var_phist_dn7_slot: &mut f64,
        var_phist_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_vbgxpos_slot: &mut f64,
        var_vbgxpos_dn3_slot: &mut f64,
        var_vbgxpos_dn5_slot: &mut f64,
        var_vbgxpos_dn6_slot: &mut f64,
        var_vgfb2eff_slot: &mut f64,
        var_vgfb2eff_dn5_slot: &mut f64,
        var_vgfb2eff_dn6_slot: &mut f64,
        var_vknee1_slot: &mut f64,
        var_vknee2_slot: &mut f64,
        var_vsubdep_slot: &mut f64,
        var_vsubdep0_slot: &mut f64,
        var_vsubdep_dn3_slot: &mut f64,
        var_vsubdep_dn4_slot: &mut f64,
        var_vsubdep_dn5_slot: &mut f64,
        var_vsubdep_dn6_slot: &mut f64,
        var_vsubdep_dn7_slot: &mut f64,
        var_vsubdep_dn8_slot: &mut f64,
    ) {
        let mut var_bpfactor: f64 = *var_bpfactor_slot;
        let mut var_dvth_dibl: f64 = *var_dvth_dibl_slot;
        let mut var_dvth_dibl_dn3: f64 = *var_dvth_dibl_dn3_slot;
        let mut var_dvth_dibl_dn4: f64 = *var_dvth_dibl_dn4_slot;
        let mut var_dvth_dibl_dn5: f64 = *var_dvth_dibl_dn5_slot;
        let mut var_dvth_dibl_dn6: f64 = *var_dvth_dibl_dn6_slot;
        let mut var_dvth_dibl_dn7: f64 = *var_dvth_dibl_dn7_slot;
        let mut var_dvth_dibl_dn8: f64 = *var_dvth_dibl_dn8_slot;
        let mut var_dvth_dsc: f64 = *var_dvth_dsc_slot;
        let mut var_dvth_dsc_dn5: f64 = *var_dvth_dsc_dn5_slot;
        let mut var_dvth_dsc_dn6: f64 = *var_dvth_dsc_dn6_slot;
        let mut var_dvth_nbody: f64 = *var_dvth_nbody_slot;
        let mut var_dvth_rsce: f64 = *var_dvth_rsce_slot;
        let mut var_dvth_rsce_dn3: f64 = *var_dvth_rsce_dn3_slot;
        let mut var_dvth_rsce_dn4: f64 = *var_dvth_rsce_dn4_slot;
        let mut var_dvth_rsce_dn5: f64 = *var_dvth_rsce_dn5_slot;
        let mut var_dvth_rsce_dn6: f64 = *var_dvth_rsce_dn6_slot;
        let mut var_dvth_rsce_dn7: f64 = *var_dvth_rsce_dn7_slot;
        let mut var_dvth_rsce_dn8: f64 = *var_dvth_rsce_dn8_slot;
        let mut var_dvth_vbg: f64 = *var_dvth_vbg_slot;
        let mut var_dvth_vbg_dn3: f64 = *var_dvth_vbg_dn3_slot;
        let mut var_dvth_vbg_dn4: f64 = *var_dvth_vbg_dn4_slot;
        let mut var_dvth_vbg_dn5: f64 = *var_dvth_vbg_dn5_slot;
        let mut var_dvth_vbg_dn6: f64 = *var_dvth_vbg_dn6_slot;
        let mut var_dvth_vbg_dn7: f64 = *var_dvth_vbg_dn7_slot;
        let mut var_dvth_vbg_dn8: f64 = *var_dvth_vbg_dn8_slot;
        let mut var_dvth_vtroll: f64 = *var_dvth_vtroll_slot;
        let mut var_dvth_vtroll_dn3: f64 = *var_dvth_vtroll_dn3_slot;
        let mut var_dvth_vtroll_dn4: f64 = *var_dvth_vtroll_dn4_slot;
        let mut var_dvth_vtroll_dn5: f64 = *var_dvth_vtroll_dn5_slot;
        let mut var_dvth_vtroll_dn6: f64 = *var_dvth_vtroll_dn6_slot;
        let mut var_dvth_vtroll_dn7: f64 = *var_dvth_vtroll_dn7_slot;
        let mut var_dvth_vtroll_dn8: f64 = *var_dvth_vtroll_dn8_slot;
        let mut var_gamma0: f64 = *var_gamma0_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_kvbg: f64 = *var_kvbg_slot;
        let mut var_kvbg_dn3: f64 = *var_kvbg_dn3_slot;
        let mut var_kvbg_dn4: f64 = *var_kvbg_dn4_slot;
        let mut var_kvbg_dn5: f64 = *var_kvbg_dn5_slot;
        let mut var_kvbg_dn6: f64 = *var_kvbg_dn6_slot;
        let mut var_kvbg_dn7: f64 = *var_kvbg_dn7_slot;
        let mut var_kvbg_dn8: f64 = *var_kvbg_dn8_slot;
        let mut var_nvtm: f64 = *var_nvtm_slot;
        let mut var_nvtm_dn3: f64 = *var_nvtm_dn3_slot;
        let mut var_nvtm_dn4: f64 = *var_nvtm_dn4_slot;
        let mut var_nvtm_dn5: f64 = *var_nvtm_dn5_slot;
        let mut var_nvtm_dn6: f64 = *var_nvtm_dn6_slot;
        let mut var_nvtm_dn7: f64 = *var_nvtm_dn7_slot;
        let mut var_nvtm_dn8: f64 = *var_nvtm_dn8_slot;
        let mut var_phist: f64 = *var_phist_slot;
        let mut var_phist_dn3: f64 = *var_phist_dn3_slot;
        let mut var_phist_dn4: f64 = *var_phist_dn4_slot;
        let mut var_phist_dn5: f64 = *var_phist_dn5_slot;
        let mut var_phist_dn6: f64 = *var_phist_dn6_slot;
        let mut var_phist_dn7: f64 = *var_phist_dn7_slot;
        let mut var_phist_dn8: f64 = *var_phist_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_vbgxpos: f64 = *var_vbgxpos_slot;
        let mut var_vbgxpos_dn3: f64 = *var_vbgxpos_dn3_slot;
        let mut var_vbgxpos_dn5: f64 = *var_vbgxpos_dn5_slot;
        let mut var_vbgxpos_dn6: f64 = *var_vbgxpos_dn6_slot;
        let mut var_vgfb2eff: f64 = *var_vgfb2eff_slot;
        let mut var_vgfb2eff_dn5: f64 = *var_vgfb2eff_dn5_slot;
        let mut var_vgfb2eff_dn6: f64 = *var_vgfb2eff_dn6_slot;
        let mut var_vknee1: f64 = *var_vknee1_slot;
        let mut var_vknee2: f64 = *var_vknee2_slot;
        let mut var_vsubdep: f64 = *var_vsubdep_slot;
        let mut var_vsubdep0: f64 = *var_vsubdep0_slot;
        let mut var_vsubdep_dn3: f64 = *var_vsubdep_dn3_slot;
        let mut var_vsubdep_dn4: f64 = *var_vsubdep_dn4_slot;
        let mut var_vsubdep_dn5: f64 = *var_vsubdep_dn5_slot;
        let mut var_vsubdep_dn6: f64 = *var_vsubdep_dn6_slot;
        let mut var_vsubdep_dn7: f64 = *var_vsubdep_dn7_slot;
        let mut var_vsubdep_dn8: f64 = *var_vsubdep_dn8_slot;

        let (assign4510_e5262, assign4510_e5262_d_n3, assign4510_e5262_d_n4, assign4510_e5262_d_n5, assign4510_e5262_d_n6, assign4510_e5262_d_n7, assign4510_e5262_d_n8,) = {
    if (var_guard71 != 0.0) {
        let assign4510_e5257: f64 = (0.5 * var_kbg1pw_i);
        let assign4510_e5259: f64 = (assign4510_e5257 / var_t0);
        let assign4510_e5260: f64 = (var_kbg0pw_i - assign4510_e5259);
        (assign4510_e5260, (-(-((assign4510_e5257 * var_t0_dn3) / (var_t0 * var_t0)))), (-(-((assign4510_e5257 * var_t0_dn4) / (var_t0 * var_t0)))), (-(-((assign4510_e5257 * var_t0_dn5) / (var_t0 * var_t0)))), (-(-((assign4510_e5257 * var_t0_dn6) / (var_t0 * var_t0)))), (-(-((assign4510_e5257 * var_t0_dn7) / (var_t0 * var_t0)))), (-(-((assign4510_e5257 * var_t0_dn8) / (var_t0 * var_t0)))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign4510_e5262;
        var_t1_dn3 = assign4510_e5262_d_n3;
        var_t1_dn4 = assign4510_e5262_d_n4;
        var_t1_dn5 = assign4510_e5262_d_n5;
        var_t1_dn6 = assign4510_e5262_d_n6;
        var_t1_dn7 = assign4510_e5262_d_n7;
        var_t1_dn8 = assign4510_e5262_d_n8;

        let (assign4520_e5266, assign4520_e5266_d_n3, assign4520_e5266_d_n4, assign4520_e5266_d_n5, assign4520_e5266_d_n6, assign4520_e5266_d_n7, assign4520_e5266_d_n8,) = {
    if (var_guard71 != 0.0) {
        (var_kbg2pw_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign4520_e5266;
        var_t2_dn3 = assign4520_e5266_d_n3;
        var_t2_dn4 = assign4520_e5266_d_n4;
        var_t2_dn5 = assign4520_e5266_d_n5;
        var_t2_dn6 = assign4520_e5266_d_n6;
        var_t2_dn7 = assign4520_e5266_d_n7;
        var_t2_dn8 = assign4520_e5266_d_n8;

        let (assign4530_e5270,) = {
    if (var_guard71 != 0.0) {
        (var_vknee1pw_i,)
    } else {
        (var_vknee1,)
    }
};
        var_vknee1 = assign4530_e5270;

        let (assign4540_e5274,) = {
    if (var_guard71 != 0.0) {
        (var_vknee2pw_i,)
    } else {
        (var_vknee2,)
    }
};
        var_vknee2 = assign4540_e5274;

        let (assign4550_e5278,) = {
    if (var_guard71 != 0.0) {
        (var_bpfactorpw_i,)
    } else {
        (var_bpfactor,)
    }
};
        var_bpfactor = assign4550_e5278;

        let (assign4560_e5287, assign4560_e5287_d_n3, assign4560_e5287_d_n4, assign4560_e5287_d_n5, assign4560_e5287_d_n6, assign4560_e5287_d_n7, assign4560_e5287_d_n8,) = {
    if (var_guard71 == 0.0) {
        let assign4560_e5283: f64 = (var_dbgnw_i * var_leff);
        let assign4560_e5285: f64 = (assign4560_e5283 / var_scl);
        (assign4560_e5285, (-((assign4560_e5283 * var_scl_dn3) / (var_scl * var_scl))), (-((assign4560_e5283 * var_scl_dn4) / (var_scl * var_scl))), (-((assign4560_e5283 * var_scl_dn5) / (var_scl * var_scl))), (-((assign4560_e5283 * var_scl_dn6) / (var_scl * var_scl))), (-((assign4560_e5283 * var_scl_dn7) / (var_scl * var_scl))), (-((assign4560_e5283 * var_scl_dn8) / (var_scl * var_scl))),)
    } else {
        (var_temp, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8,)
    }
};
        var_temp = assign4560_e5287;
        var_temp_dn3 = assign4560_e5287_d_n3;
        var_temp_dn4 = assign4560_e5287_d_n4;
        var_temp_dn5 = assign4560_e5287_d_n5;
        var_temp_dn6 = assign4560_e5287_d_n6;
        var_temp_dn7 = assign4560_e5287_d_n7;
        var_temp_dn8 = assign4560_e5287_d_n8;

        let assign4570_e5290: f64 = if var_temp > 40.0 { 1.0 } else { 0.0 };
        var_guard73 = assign4570_e5290;

        let (assign4580_e5300, assign4580_e5300_d_n3, assign4580_e5300_d_n4, assign4580_e5300_d_n5, assign4580_e5300_d_n6, assign4580_e5300_d_n7, assign4580_e5300_d_n8,) = {
    if ((var_guard71 == 0.0) && (var_guard73 != 0.0)) {
        let assign4580_e5296: f64 = { let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4580_e5298: f64 = (assign4580_e5296 / 2.0);
        (assign4580_e5298, (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn3) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn4) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn5) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn6) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn7) / 2.0), (({ let limited_exp_arg = var_temp; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_temp_dn8) / 2.0),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign4580_e5300;
        var_t0_dn3 = assign4580_e5300_d_n3;
        var_t0_dn4 = assign4580_e5300_d_n4;
        var_t0_dn5 = assign4580_e5300_d_n5;
        var_t0_dn6 = assign4580_e5300_d_n6;
        var_t0_dn7 = assign4580_e5300_d_n7;
        var_t0_dn8 = assign4580_e5300_d_n8;

        let (assign4590_e5311, assign4590_e5311_d_n3, assign4590_e5311_d_n4, assign4590_e5311_d_n5, assign4590_e5311_d_n6, assign4590_e5311_d_n7, assign4590_e5311_d_n8,) = {
    if ((var_guard71 == 0.0) && (var_guard73 == 0.0)) {
        let assign4590_e5307: f64 = (var_temp).cosh();
        let assign4590_e5309: f64 = (assign4590_e5307 - 1.0);
        (assign4590_e5309, ((var_temp).sinh() * var_temp_dn3), ((var_temp).sinh() * var_temp_dn4), ((var_temp).sinh() * var_temp_dn5), ((var_temp).sinh() * var_temp_dn6), ((var_temp).sinh() * var_temp_dn7), ((var_temp).sinh() * var_temp_dn8),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign4590_e5311;
        var_t0_dn3 = assign4590_e5311_d_n3;
        var_t0_dn4 = assign4590_e5311_d_n4;
        var_t0_dn5 = assign4590_e5311_d_n5;
        var_t0_dn6 = assign4590_e5311_d_n6;
        var_t0_dn7 = assign4590_e5311_d_n7;
        var_t0_dn8 = assign4590_e5311_d_n8;

        let (assign4600_e5322, assign4600_e5322_d_n3, assign4600_e5322_d_n4, assign4600_e5322_d_n5, assign4600_e5322_d_n6, assign4600_e5322_d_n7, assign4600_e5322_d_n8,) = {
    if (var_guard71 == 0.0) {
        let assign4600_e5317: f64 = (0.5 * var_kbg1nw_i);
        let assign4600_e5319: f64 = (assign4600_e5317 / var_t0);
        let assign4600_e5320: f64 = (var_kbg0nw_i - assign4600_e5319);
        (assign4600_e5320, (-(-((assign4600_e5317 * var_t0_dn3) / (var_t0 * var_t0)))), (-(-((assign4600_e5317 * var_t0_dn4) / (var_t0 * var_t0)))), (-(-((assign4600_e5317 * var_t0_dn5) / (var_t0 * var_t0)))), (-(-((assign4600_e5317 * var_t0_dn6) / (var_t0 * var_t0)))), (-(-((assign4600_e5317 * var_t0_dn7) / (var_t0 * var_t0)))), (-(-((assign4600_e5317 * var_t0_dn8) / (var_t0 * var_t0)))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign4600_e5322;
        var_t1_dn3 = assign4600_e5322_d_n3;
        var_t1_dn4 = assign4600_e5322_d_n4;
        var_t1_dn5 = assign4600_e5322_d_n5;
        var_t1_dn6 = assign4600_e5322_d_n6;
        var_t1_dn7 = assign4600_e5322_d_n7;
        var_t1_dn8 = assign4600_e5322_d_n8;

        let (assign4610_e5327, assign4610_e5327_d_n3, assign4610_e5327_d_n4, assign4610_e5327_d_n5, assign4610_e5327_d_n6, assign4610_e5327_d_n7, assign4610_e5327_d_n8,) = {
    if (var_guard71 == 0.0) {
        (var_kbg2nw_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign4610_e5327;
        var_t2_dn3 = assign4610_e5327_d_n3;
        var_t2_dn4 = assign4610_e5327_d_n4;
        var_t2_dn5 = assign4610_e5327_d_n5;
        var_t2_dn6 = assign4610_e5327_d_n6;
        var_t2_dn7 = assign4610_e5327_d_n7;
        var_t2_dn8 = assign4610_e5327_d_n8;

        let (assign4620_e5332,) = {
    if (var_guard71 == 0.0) {
        (var_vknee1nw_i,)
    } else {
        (var_vknee1,)
    }
};
        var_vknee1 = assign4620_e5332;

        let (assign4630_e5337,) = {
    if (var_guard71 == 0.0) {
        (var_vknee2nw_i,)
    } else {
        (var_vknee2,)
    }
};
        var_vknee2 = assign4630_e5337;

        let (assign4640_e5342,) = {
    if (var_guard71 == 0.0) {
        (var_bpfactornw_i,)
    } else {
        (var_bpfactor,)
    }
};
        var_bpfactor = assign4640_e5342;

        let assign4650_e5345: f64 = (var_t1 - var_t2);
        var_t0 = assign4650_e5345;
        var_t0_dn3 = (var_t1_dn3 - var_t2_dn3);
        var_t0_dn4 = (var_t1_dn4 - var_t2_dn4);
        var_t0_dn5 = (var_t1_dn5 - var_t2_dn5);
        var_t0_dn6 = (var_t1_dn6 - var_t2_dn6);
        var_t0_dn7 = (var_t1_dn7 - var_t2_dn7);
        var_t0_dn8 = (var_t1_dn8 - var_t2_dn8);

        let assign4660_e5351: f64 = (var_t0 * var_t0);
        let assign4660_e5353: f64 = (assign4660_e5351 + 0.0001);
        let assign4660_e5354: f64 = (assign4660_e5353).sqrt();
        let assign4660_e5355: f64 = (var_t0 + assign4660_e5354);
        let assign4660_e5356: f64 = (0.5 * assign4660_e5355);
        let assign4660_e5357: f64 = (var_t2 + assign4660_e5356);
        var_kvbg = assign4660_e5357;
        var_kvbg_dn3 = (var_t2_dn3 + (0.5 * (var_t0_dn3 + (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign4660_e5354)))));
        var_kvbg_dn4 = (var_t2_dn4 + (0.5 * (var_t0_dn4 + (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign4660_e5354)))));
        var_kvbg_dn5 = (var_t2_dn5 + (0.5 * (var_t0_dn5 + (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign4660_e5354)))));
        var_kvbg_dn6 = (var_t2_dn6 + (0.5 * (var_t0_dn6 + (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign4660_e5354)))));
        var_kvbg_dn7 = (var_t2_dn7 + (0.5 * (var_t0_dn7 + (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign4660_e5354)))));
        var_kvbg_dn8 = (var_t2_dn8 + (0.5 * (var_t0_dn8 + (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign4660_e5354)))));

        let assign4670_e5360: f64 = (1.60219e-19 * p.p52);
        let assign4670_e5362: f64 = (assign4670_e5360 * var_epssi);
        let assign4670_e5365: f64 = (2.0 * var_cox2);
        let assign4670_e5367: f64 = (assign4670_e5365 * var_cox2);
        let assign4670_e5368: f64 = (assign4670_e5362 / assign4670_e5367);
        var_vsubdep0 = assign4670_e5368;

        let assign4680_e5371: f64 = if p.p52 != 0.0 { 1.0 } else { 0.0 };
        var_guard74 = assign4680_e5371;

        let (assign4690_e5413, assign4690_e5413_d_n3, assign4690_e5413_d_n4, assign4690_e5413_d_n5, assign4690_e5413_d_n6, assign4690_e5413_d_n7, assign4690_e5413_d_n8,) = {
    if (var_guard74 != 0.0) {
        let assign4690_e5378: f64 = (var_devsign * var_vbgx);
        let assign4690_e5380: f64 = (assign4690_e5378 - var_vknee1);
        let assign4690_e5381: f64 = (var_welsign * assign4690_e5380);
        let assign4690_e5385: f64 = (var_devsign * var_vbgx);
        let assign4690_e5387: f64 = (assign4690_e5385 - var_vknee1);
        let assign4690_e5388: f64 = (var_welsign * assign4690_e5387);
        let assign4690_e5392: f64 = (var_devsign * var_vbgx);
        let assign4690_e5394: f64 = (assign4690_e5392 - var_vknee1);
        let assign4690_e5395: f64 = (var_welsign * assign4690_e5394);
        let assign4690_e5396: f64 = (assign4690_e5388 * assign4690_e5395);
        let assign4690_e5399: f64 = (4.0 * 0.01);
        let assign4690_e5401: f64 = (assign4690_e5399 * 0.01);
        let assign4690_e5402: f64 = (assign4690_e5396 + assign4690_e5401);
        let assign4690_e5403: f64 = (assign4690_e5402).sqrt();
        let assign4690_e5404: f64 = (assign4690_e5381 + assign4690_e5403);
        let assign4690_e5405: f64 = (0.5 * assign4690_e5404);
        let assign4690_e5407: f64 = (assign4690_e5405 / var_vsubdep0);
        let assign4690_e5408: f64 = (1.0 + assign4690_e5407);
        let assign4690_e5409: f64 = (assign4690_e5408).sqrt();
        let assign4690_e5411: f64 = (assign4690_e5409 - 1.0);
        (assign4690_e5411, (((0.5 * ((var_welsign * (var_devsign * var_vbgx_dn3)) + ((((var_welsign * (var_devsign * var_vbgx_dn3)) * assign4690_e5395) + (assign4690_e5388 * (var_welsign * (var_devsign * var_vbgx_dn3)))) / (2.0 * assign4690_e5403)))) / var_vsubdep0) / (2.0 * assign4690_e5409)), 0.0, (((0.5 * ((var_welsign * (var_devsign * var_vbgx_dn5)) + ((((var_welsign * (var_devsign * var_vbgx_dn5)) * assign4690_e5395) + (assign4690_e5388 * (var_welsign * (var_devsign * var_vbgx_dn5)))) / (2.0 * assign4690_e5403)))) / var_vsubdep0) / (2.0 * assign4690_e5409)), (((0.5 * ((var_welsign * (var_devsign * var_vbgx_dn6)) + ((((var_welsign * (var_devsign * var_vbgx_dn6)) * assign4690_e5395) + (assign4690_e5388 * (var_welsign * (var_devsign * var_vbgx_dn6)))) / (2.0 * assign4690_e5403)))) / var_vsubdep0) / (2.0 * assign4690_e5409)), 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign4690_e5413;
        var_t0_dn3 = assign4690_e5413_d_n3;
        var_t0_dn4 = assign4690_e5413_d_n4;
        var_t0_dn5 = assign4690_e5413_d_n5;
        var_t0_dn6 = assign4690_e5413_d_n6;
        var_t0_dn7 = assign4690_e5413_d_n7;
        var_t0_dn8 = assign4690_e5413_d_n8;

        let (assign4700_e5418, assign4700_e5418_d_n3, assign4700_e5418_d_n4, assign4700_e5418_d_n5, assign4700_e5418_d_n6, assign4700_e5418_d_n7, assign4700_e5418_d_n8,) = {
    if (var_guard74 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign4700_e5418;
        var_t0_dn3 = assign4700_e5418_d_n3;
        var_t0_dn4 = assign4700_e5418_d_n4;
        var_t0_dn5 = assign4700_e5418_d_n5;
        var_t0_dn6 = assign4700_e5418_d_n6;
        var_t0_dn7 = assign4700_e5418_d_n7;
        var_t0_dn8 = assign4700_e5418_d_n8;

        let assign4710_e5421: f64 = (var_vsubdep0 * var_t0);
        let assign4710_e5423: f64 = (assign4710_e5421 * var_t0);
        var_vsubdep = assign4710_e5423;
        var_vsubdep_dn3 = (((var_vsubdep0 * var_t0_dn3) * var_t0) + (assign4710_e5421 * var_t0_dn3));
        var_vsubdep_dn4 = (((var_vsubdep0 * var_t0_dn4) * var_t0) + (assign4710_e5421 * var_t0_dn4));
        var_vsubdep_dn5 = (((var_vsubdep0 * var_t0_dn5) * var_t0) + (assign4710_e5421 * var_t0_dn5));
        var_vsubdep_dn6 = (((var_vsubdep0 * var_t0_dn6) * var_t0) + (assign4710_e5421 * var_t0_dn6));
        var_vsubdep_dn7 = (((var_vsubdep0 * var_t0_dn7) * var_t0) + (assign4710_e5421 * var_t0_dn7));
        var_vsubdep_dn8 = (((var_vsubdep0 * var_t0_dn8) * var_t0) + (assign4710_e5421 * var_t0_dn8));

        let assign4720_e5425: f64 = (-var_vknee2);
        let assign4720_e5428: f64 = (-var_vsubdep);
        let assign4720_e5430: f64 = (-var_vknee2);
        let assign4720_e5431: f64 = (assign4720_e5428 - assign4720_e5430);
        let assign4720_e5433: f64 = (assign4720_e5431 - 0.01);
        let assign4720_e5435: f64 = (-var_vsubdep);
        let assign4720_e5437: f64 = (-var_vknee2);
        let assign4720_e5438: f64 = (assign4720_e5435 - assign4720_e5437);
        let assign4720_e5440: f64 = (assign4720_e5438 - 0.01);
        let assign4720_e5442: f64 = (-var_vsubdep);
        let assign4720_e5444: f64 = (-var_vknee2);
        let assign4720_e5445: f64 = (assign4720_e5442 - assign4720_e5444);
        let assign4720_e5447: f64 = (assign4720_e5445 - 0.01);
        let assign4720_e5448: f64 = (assign4720_e5440 * assign4720_e5447);
        let assign4720_e5451: f64 = (-var_vknee2);
        let assign4720_e5452: f64 = (4.0 * assign4720_e5451);
        let assign4720_e5454: f64 = (assign4720_e5452 * 0.01);
        let assign4720_e5455: f64 = (assign4720_e5448 - assign4720_e5454);
        let assign4720_e5456: f64 = (assign4720_e5455).sqrt();
        let assign4720_e5457: f64 = (assign4720_e5433 + assign4720_e5456);
        let assign4720_e5458: f64 = (0.5 * assign4720_e5457);
        let assign4720_e5459: f64 = (assign4720_e5425 + assign4720_e5458);
        let assign4720_e5460: f64 = (-assign4720_e5459);
        var_vsubdep = assign4720_e5460;
        var_vsubdep_dn3 = (-(0.5 * ((-var_vsubdep_dn3) + ((((-var_vsubdep_dn3) * assign4720_e5447) + (assign4720_e5440 * (-var_vsubdep_dn3))) / (2.0 * assign4720_e5456)))));
        var_vsubdep_dn4 = (-(0.5 * ((-var_vsubdep_dn4) + ((((-var_vsubdep_dn4) * assign4720_e5447) + (assign4720_e5440 * (-var_vsubdep_dn4))) / (2.0 * assign4720_e5456)))));
        var_vsubdep_dn5 = (-(0.5 * ((-var_vsubdep_dn5) + ((((-var_vsubdep_dn5) * assign4720_e5447) + (assign4720_e5440 * (-var_vsubdep_dn5))) / (2.0 * assign4720_e5456)))));
        var_vsubdep_dn6 = (-(0.5 * ((-var_vsubdep_dn6) + ((((-var_vsubdep_dn6) * assign4720_e5447) + (assign4720_e5440 * (-var_vsubdep_dn6))) / (2.0 * assign4720_e5456)))));
        var_vsubdep_dn7 = (-(0.5 * ((-var_vsubdep_dn7) + ((((-var_vsubdep_dn7) * assign4720_e5447) + (assign4720_e5440 * (-var_vsubdep_dn7))) / (2.0 * assign4720_e5456)))));
        var_vsubdep_dn8 = (-(0.5 * ((-var_vsubdep_dn8) + ((((-var_vsubdep_dn8) * assign4720_e5447) + (assign4720_e5440 * (-var_vsubdep_dn8))) / (2.0 * assign4720_e5456)))));

        let assign4730_e5462: f64 = (-1.2);
        let assign4730_e5464: f64 = (assign4730_e5462 - var_symmetry_factor);
        var_vgfb2eff = assign4730_e5464;
        var_vgfb2eff_dn5 = (-var_symmetry_factor_dn5);
        var_vgfb2eff_dn6 = (-var_symmetry_factor_dn6);

        let assign4740_e5466: f64 = (-var_cox2);
        let assign4740_e5468: f64 = (assign4740_e5466 * var_csi);
        let assign4740_e5471: f64 = (var_cox2 + var_csi);
        let assign4740_e5473: f64 = (assign4740_e5471 * var_cox1);
        let assign4740_e5474: f64 = (assign4740_e5468 / assign4740_e5473);
        var_gamma0 = assign4740_e5474;

        let assign4750_e5477: f64 = (var_gamma0 * var_kvbg);
        let assign4750_e5481: f64 = (var_devsign * var_welsign);
        let assign4750_e5483: f64 = (assign4750_e5481 * var_bpfactor);
        let assign4750_e5485: f64 = (assign4750_e5483 * var_vsubdep);
        let assign4750_e5486: f64 = (var_vgfb2 - assign4750_e5485);
        let assign4750_e5488: f64 = (assign4750_e5486 - var_vgfb2eff);
        let assign4750_e5489: f64 = (assign4750_e5477 * assign4750_e5488);
        var_dvth_vbg = assign4750_e5489;
        var_dvth_vbg_dn3 = (((var_gamma0 * var_kvbg_dn3) * assign4750_e5488) + (assign4750_e5477 * (var_vgfb2_dn3 - (assign4750_e5483 * var_vsubdep_dn3))));
        var_dvth_vbg_dn4 = (((var_gamma0 * var_kvbg_dn4) * assign4750_e5488) + (assign4750_e5477 * (var_vgfb2_dn4 - (assign4750_e5483 * var_vsubdep_dn4))));
        var_dvth_vbg_dn5 = (((var_gamma0 * var_kvbg_dn5) * assign4750_e5488) + (assign4750_e5477 * ((var_vgfb2_dn5 - (assign4750_e5483 * var_vsubdep_dn5)) - var_vgfb2eff_dn5)));
        var_dvth_vbg_dn6 = (((var_gamma0 * var_kvbg_dn6) * assign4750_e5488) + (assign4750_e5477 * ((var_vgfb2_dn6 - (assign4750_e5483 * var_vsubdep_dn6)) - var_vgfb2eff_dn6)));
        var_dvth_vbg_dn7 = (((var_gamma0 * var_kvbg_dn7) * assign4750_e5488) + (assign4750_e5477 * (var_vgfb2_dn7 - (assign4750_e5483 * var_vsubdep_dn7))));
        var_dvth_vbg_dn8 = (((var_gamma0 * var_kvbg_dn8) * assign4750_e5488) + (assign4750_e5477 * (var_vgfb2_dn8 - (assign4750_e5483 * var_vsubdep_dn8))));

        let assign4760_e5494: f64 = (var_vbgx * var_vbgx);
        let assign4760_e5497: f64 = (4.0 * 0.001);
        let assign4760_e5499: f64 = (assign4760_e5497 * 0.001);
        let assign4760_e5500: f64 = (assign4760_e5494 + assign4760_e5499);
        let assign4760_e5501: f64 = (assign4760_e5500).sqrt();
        let assign4760_e5502: f64 = (var_vbgx + assign4760_e5501);
        let assign4760_e5503: f64 = (0.5 * assign4760_e5502);
        var_vbgxpos = assign4760_e5503;
        var_vbgxpos_dn3 = (0.5 * (var_vbgx_dn3 + (((var_vbgx_dn3 * var_vbgx) + (var_vbgx * var_vbgx_dn3)) / (2.0 * assign4760_e5501))));
        var_vbgxpos_dn5 = (0.5 * (var_vbgx_dn5 + (((var_vbgx_dn5 * var_vbgx) + (var_vbgx * var_vbgx_dn5)) / (2.0 * assign4760_e5501))));
        var_vbgxpos_dn6 = (0.5 * (var_vbgx_dn6 + (((var_vbgx_dn6 * var_vbgx) + (var_vbgx * var_vbgx_dn6)) / (2.0 * assign4760_e5501))));

        let assign4770_e5506: f64 = (0.4 + var_phib);
        let assign4770_e5508: f64 = (assign4770_e5506 + var_phin_i);
        var_phist = assign4770_e5508;
        var_phist_dn3 = var_phib_dn3;
        var_phist_dn4 = var_phib_dn4;
        var_phist_dn5 = var_phib_dn5;
        var_phist_dn6 = var_phib_dn6;
        var_phist_dn7 = var_phib_dn7;
        var_phist_dn8 = var_phib_dn8;

        let assign4780_e5511: f64 = if var_phist < 0.0 { 1.0 } else { 0.0 };
        var_guard75 = assign4780_e5511;

        let (assign4790_e5515, assign4790_e5515_d_n3, assign4790_e5515_d_n4, assign4790_e5515_d_n5, assign4790_e5515_d_n6, assign4790_e5515_d_n7, assign4790_e5515_d_n8,) = {
    if (var_guard75 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dvth_rsce, var_dvth_rsce_dn3, var_dvth_rsce_dn4, var_dvth_rsce_dn5, var_dvth_rsce_dn6, var_dvth_rsce_dn7, var_dvth_rsce_dn8,)
    }
};
        var_dvth_rsce = assign4790_e5515;
        var_dvth_rsce_dn3 = assign4790_e5515_d_n3;
        var_dvth_rsce_dn4 = assign4790_e5515_d_n4;
        var_dvth_rsce_dn5 = assign4790_e5515_d_n5;
        var_dvth_rsce_dn6 = assign4790_e5515_d_n6;
        var_dvth_rsce_dn7 = assign4790_e5515_d_n7;
        var_dvth_rsce_dn8 = assign4790_e5515_d_n8;

        let (assign4800_e5525, assign4800_e5525_d_n3, assign4800_e5525_d_n4, assign4800_e5525_d_n5, assign4800_e5525_d_n6, assign4800_e5525_d_n7, assign4800_e5525_d_n8,) = {
    if (var_guard75 == 0.0) {
        let assign4800_e5520: f64 = (var_k1rsce_i * var_theta_rsce);
        let assign4800_e5522: f64 = (var_phist).sqrt();
        let assign4800_e5523: f64 = (assign4800_e5520 * assign4800_e5522);
        (assign4800_e5523, (assign4800_e5520 * (var_phist_dn3 / (2.0 * assign4800_e5522))), (assign4800_e5520 * (var_phist_dn4 / (2.0 * assign4800_e5522))), (assign4800_e5520 * (var_phist_dn5 / (2.0 * assign4800_e5522))), (assign4800_e5520 * (var_phist_dn6 / (2.0 * assign4800_e5522))), (assign4800_e5520 * (var_phist_dn7 / (2.0 * assign4800_e5522))), (assign4800_e5520 * (var_phist_dn8 / (2.0 * assign4800_e5522))),)
    } else {
        (var_dvth_rsce, var_dvth_rsce_dn3, var_dvth_rsce_dn4, var_dvth_rsce_dn5, var_dvth_rsce_dn6, var_dvth_rsce_dn7, var_dvth_rsce_dn8,)
    }
};
        var_dvth_rsce = assign4800_e5525;
        var_dvth_rsce_dn3 = assign4800_e5525_d_n3;
        var_dvth_rsce_dn4 = assign4800_e5525_d_n4;
        var_dvth_rsce_dn5 = assign4800_e5525_d_n5;
        var_dvth_rsce_dn6 = assign4800_e5525_d_n6;
        var_dvth_rsce_dn7 = assign4800_e5525_d_n7;
        var_dvth_rsce_dn8 = assign4800_e5525_d_n8;

        let assign4810_e5527: f64 = (-var_dvt0_i);
        let assign4810_e5529: f64 = (assign4810_e5527 * var_theta_sce);
        let assign4810_e5532: f64 = (var_vbi - var_phist);
        let assign4810_e5533: f64 = (assign4810_e5529 * assign4810_e5532);
        var_dvth_vtroll = assign4810_e5533;
        var_dvth_vtroll_dn3 = (((assign4810_e5527 * var_theta_sce_dn3) * assign4810_e5532) + (assign4810_e5529 * (var_vbi_dn3 - var_phist_dn3)));
        var_dvth_vtroll_dn4 = (((assign4810_e5527 * var_theta_sce_dn4) * assign4810_e5532) + (assign4810_e5529 * (var_vbi_dn4 - var_phist_dn4)));
        var_dvth_vtroll_dn5 = (((assign4810_e5527 * var_theta_sce_dn5) * assign4810_e5532) + (assign4810_e5529 * (var_vbi_dn5 - var_phist_dn5)));
        var_dvth_vtroll_dn6 = (((assign4810_e5527 * var_theta_sce_dn6) * assign4810_e5532) + (assign4810_e5529 * (var_vbi_dn6 - var_phist_dn6)));
        var_dvth_vtroll_dn7 = (((assign4810_e5527 * var_theta_sce_dn7) * assign4810_e5532) + (assign4810_e5529 * (var_vbi_dn7 - var_phist_dn7)));
        var_dvth_vtroll_dn8 = (((assign4810_e5527 * var_theta_sce_dn8) * assign4810_e5532) + (assign4810_e5529 * (var_vbi_dn8 - var_phist_dn8)));

        let assign4820_e5537: f64 = (var_etab_i * var_vbgx);
        let assign4820_e5538: f64 = (var_eta0_t + assign4820_e5537);
        let assign4820_e5539: f64 = (-assign4820_e5538);
        let assign4820_e5541: f64 = (assign4820_e5539 * var_theta_dibl);
        let assign4820_e5546: f64 = (var_vdsx + 0.01);
        let assign4820_e5547: f64 = (assign4820_e5546).sqrt();
        let assign4820_e5548: f64 = (var_eta1_i * assign4820_e5547);
        let assign4820_e5549: f64 = (var_vdsx + assign4820_e5548);
        let assign4820_e5550: f64 = (assign4820_e5541 * assign4820_e5549);
        let assign4820_e5553: f64 = (var_dvtp0_i * var_theta_dits);
        let assign4820_e5556: f64 = (var_vdsx + 0.01);
        let assign4820_e5558: f64 = (assign4820_e5556).powf(var_dvtp1_i);
        let assign4820_e5559: f64 = (assign4820_e5553 * assign4820_e5558);
        let assign4820_e5560: f64 = (assign4820_e5550 + assign4820_e5559);
        var_dvth_dibl = assign4820_e5560;
        var_dvth_dibl_dn3 = (((((-(var_etab_i * var_vbgx_dn3)) * var_theta_dibl) + (assign4820_e5539 * var_theta_dibl_dn3)) * assign4820_e5549) + ((var_dvtp0_i * var_theta_dits_dn3) * assign4820_e5558));
        var_dvth_dibl_dn4 = (((((-var_eta0_t_dn4) * var_theta_dibl) + (assign4820_e5539 * var_theta_dibl_dn4)) * assign4820_e5549) + ((var_dvtp0_i * var_theta_dits_dn4) * assign4820_e5558));
        var_dvth_dibl_dn5 = ((((((-(var_etab_i * var_vbgx_dn5)) * var_theta_dibl) + (assign4820_e5539 * var_theta_dibl_dn5)) * assign4820_e5549) + (assign4820_e5541 * (var_vdsx_dn5 + (var_eta1_i * (var_vdsx_dn5 / (2.0 * assign4820_e5547)))))) + (((var_dvtp0_i * var_theta_dits_dn5) * assign4820_e5558) + (assign4820_e5553 * if 0.0 == 0.0 && ((var_dvtp1_i) as f64).is_finite() && ((var_dvtp1_i) as f64).fract() == 0.0 { if var_dvtp1_i == 0.0 { 0.0 } else { (var_dvtp1_i * ((assign4820_e5556).powf(var_dvtp1_i - 1.0) * var_vdsx_dn5)) } } else { (assign4820_e5558 * (var_dvtp1_i * (var_vdsx_dn5 / assign4820_e5556))) })));
        var_dvth_dibl_dn6 = ((((((-(var_etab_i * var_vbgx_dn6)) * var_theta_dibl) + (assign4820_e5539 * var_theta_dibl_dn6)) * assign4820_e5549) + (assign4820_e5541 * (var_vdsx_dn6 + (var_eta1_i * (var_vdsx_dn6 / (2.0 * assign4820_e5547)))))) + (((var_dvtp0_i * var_theta_dits_dn6) * assign4820_e5558) + (assign4820_e5553 * if 0.0 == 0.0 && ((var_dvtp1_i) as f64).is_finite() && ((var_dvtp1_i) as f64).fract() == 0.0 { if var_dvtp1_i == 0.0 { 0.0 } else { (var_dvtp1_i * ((assign4820_e5556).powf(var_dvtp1_i - 1.0) * var_vdsx_dn6)) } } else { (assign4820_e5558 * (var_dvtp1_i * (var_vdsx_dn6 / assign4820_e5556))) })));
        var_dvth_dibl_dn7 = (((assign4820_e5539 * var_theta_dibl_dn7) * assign4820_e5549) + ((var_dvtp0_i * var_theta_dits_dn7) * assign4820_e5558));
        var_dvth_dibl_dn8 = (((assign4820_e5539 * var_theta_dibl_dn8) * assign4820_e5549) + ((var_dvtp0_i * var_theta_dits_dn8) * assign4820_e5558));

        let assign4830_e5562: f64 = (-var_dsc0_i);
        let assign4830_e5565: f64 = (var_leff + var_dsc1_i);
        let assign4830_e5566: f64 = (assign4830_e5562 / assign4830_e5565);
        let assign4830_e5568: f64 = (assign4830_e5566 * var_vdsx);
        var_dvth_dsc = assign4830_e5568;
        var_dvth_dsc_dn5 = (assign4830_e5566 * var_vdsx_dn5);
        var_dvth_dsc_dn6 = (assign4830_e5566 * var_vdsx_dn6);

        let assign4840_e5571: f64 = (var_csi * var_cox2);
        let assign4840_e5574: f64 = (var_csi + var_cox2);
        let assign4840_e5575: f64 = (assign4840_e5571 / assign4840_e5574);
        var_t1 = assign4840_e5575;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;

        let assign4850_e5579: f64 = (p.p70 * var_vbgxpos);
        let assign4850_e5580: f64 = (var_cdscd_i + assign4850_e5579);
        let assign4850_e5582: f64 = (assign4850_e5580 * var_vdsx);
        var_t2 = assign4850_e5582;
        var_t2_dn3 = ((p.p70 * var_vbgxpos_dn3) * var_vdsx);
        var_t2_dn4 = 0.0;
        var_t2_dn5 = (((p.p70 * var_vbgxpos_dn5) * var_vdsx) + (assign4850_e5580 * var_vdsx_dn5));
        var_t2_dn6 = (((p.p70 * var_vbgxpos_dn6) * var_vdsx) + (assign4850_e5580 * var_vdsx_dn6));
        var_t2_dn7 = 0.0;
        var_t2_dn8 = 0.0;

        let assign4860_e5585: f64 = (p.p66 * var_vbgx);
        let assign4860_e5588: f64 = (p.p67 * var_vbgx);
        let assign4860_e5590: f64 = (assign4860_e5588 * var_vbgx);
        let assign4860_e5591: f64 = (assign4860_e5585 + assign4860_e5590);
        let assign4860_e5596: f64 = (var_cbgcbg_i * var_vbgx);
        let assign4860_e5597: f64 = (var_cdsc_i + assign4860_e5596);
        let assign4860_e5600: f64 = (p.p69 * var_vbgx);
        let assign4860_e5602: f64 = (assign4860_e5600 * var_vbgx);
        let assign4860_e5603: f64 = (assign4860_e5597 + assign4860_e5602);
        let assign4860_e5605: f64 = (assign4860_e5603 + var_t2);
        let assign4860_e5606: f64 = (var_theta_sce * assign4860_e5605);
        let assign4860_e5607: f64 = (assign4860_e5591 + assign4860_e5606);
        var_t3 = assign4860_e5607;
        var_t3_dn3 = (((p.p66 * var_vbgx_dn3) + (((p.p67 * var_vbgx_dn3) * var_vbgx) + (assign4860_e5588 * var_vbgx_dn3))) + ((var_theta_sce_dn3 * assign4860_e5605) + (var_theta_sce * (((var_cbgcbg_i * var_vbgx_dn3) + (((p.p69 * var_vbgx_dn3) * var_vbgx) + (assign4860_e5600 * var_vbgx_dn3))) + var_t2_dn3))));
        var_t3_dn4 = ((var_theta_sce_dn4 * assign4860_e5605) + (var_theta_sce * var_t2_dn4));
        var_t3_dn5 = (((p.p66 * var_vbgx_dn5) + (((p.p67 * var_vbgx_dn5) * var_vbgx) + (assign4860_e5588 * var_vbgx_dn5))) + ((var_theta_sce_dn5 * assign4860_e5605) + (var_theta_sce * (((var_cbgcbg_i * var_vbgx_dn5) + (((p.p69 * var_vbgx_dn5) * var_vbgx) + (assign4860_e5600 * var_vbgx_dn5))) + var_t2_dn5))));
        var_t3_dn6 = (((p.p66 * var_vbgx_dn6) + (((p.p67 * var_vbgx_dn6) * var_vbgx) + (assign4860_e5588 * var_vbgx_dn6))) + ((var_theta_sce_dn6 * assign4860_e5605) + (var_theta_sce * (((var_cbgcbg_i * var_vbgx_dn6) + (((p.p69 * var_vbgx_dn6) * var_vbgx) + (assign4860_e5600 * var_vbgx_dn6))) + var_t2_dn6))));
        var_t3_dn7 = ((var_theta_sce_dn7 * assign4860_e5605) + (var_theta_sce * var_t2_dn7));
        var_t3_dn8 = ((var_theta_sce_dn8 * assign4860_e5605) + (var_theta_sce * var_t2_dn8));

        let assign4870_e5611: f64 = (var_cox1 + var_t1);
        let assign4870_e5613: f64 = (assign4870_e5611 + var_cit_i);
        let assign4870_e5615: f64 = (assign4870_e5613 + var_t3);
        let assign4870_e5616: f64 = (var_vtm * assign4870_e5615);
        let assign4870_e5619: f64 = (var_cox1 + var_t1);
        let assign4870_e5620: f64 = (assign4870_e5616 / assign4870_e5619);
        var_nvtm = assign4870_e5620;
        var_nvtm_dn3 = ((((var_vtm * (var_t1_dn3 + var_t3_dn3)) * assign4870_e5619) - (assign4870_e5616 * var_t1_dn3)) / (assign4870_e5619 * assign4870_e5619));
        var_nvtm_dn4 = (((((var_vtm_dn4 * assign4870_e5615) + (var_vtm * (var_t1_dn4 + var_t3_dn4))) * assign4870_e5619) - (assign4870_e5616 * var_t1_dn4)) / (assign4870_e5619 * assign4870_e5619));
        var_nvtm_dn5 = ((((var_vtm * (var_t1_dn5 + var_t3_dn5)) * assign4870_e5619) - (assign4870_e5616 * var_t1_dn5)) / (assign4870_e5619 * assign4870_e5619));
        var_nvtm_dn6 = ((((var_vtm * (var_t1_dn6 + var_t3_dn6)) * assign4870_e5619) - (assign4870_e5616 * var_t1_dn6)) / (assign4870_e5619 * assign4870_e5619));
        var_nvtm_dn7 = ((((var_vtm * (var_t1_dn7 + var_t3_dn7)) * assign4870_e5619) - (assign4870_e5616 * var_t1_dn7)) / (assign4870_e5619 * assign4870_e5619));
        var_nvtm_dn8 = ((((var_vtm * (var_t1_dn8 + var_t3_dn8)) * assign4870_e5619) - (assign4870_e5616 * var_t1_dn8)) / (assign4870_e5619 * assign4870_e5619));

        let assign4880_e5623: f64 = (1.60219e-19 * var_nbody_i);
        let assign4880_e5625: f64 = (assign4880_e5623 * p.p49);
        let assign4880_e5627: f64 = (assign4880_e5625 / var_cox1);
        let assign4880_e5631: f64 = (0.5 * p.p49);
        let assign4880_e5635: f64 = (var_epsratio * p.p46);
        let assign4880_e5636: f64 = (p.p49 + assign4880_e5635);
        let assign4880_e5637: f64 = (assign4880_e5631 / assign4880_e5636);
        let assign4880_e5638: f64 = (1.0 - assign4880_e5637);
        let assign4880_e5639: f64 = (assign4880_e5627 * assign4880_e5638);
        var_dvth_nbody = assign4880_e5639;

        let assign4890_e5643: f64 = (p.p304 / var_leff);
        let assign4890_e5644: f64 = (p.p303 + assign4890_e5643);
        let assign4890_e5646: f64 = (assign4890_e5644 * var_vbgx);
        var_t0 = assign4890_e5646;
        var_t0_dn3 = (assign4890_e5644 * var_vbgx_dn3);
        var_t0_dn4 = 0.0;
        var_t0_dn5 = (assign4890_e5644 * var_vbgx_dn5);
        var_t0_dn6 = (assign4890_e5644 * var_vbgx_dn6);
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;

        *var_bpfactor_slot = var_bpfactor;
        *var_dvth_dibl_slot = var_dvth_dibl;
        *var_dvth_dibl_dn3_slot = var_dvth_dibl_dn3;
        *var_dvth_dibl_dn4_slot = var_dvth_dibl_dn4;
        *var_dvth_dibl_dn5_slot = var_dvth_dibl_dn5;
        *var_dvth_dibl_dn6_slot = var_dvth_dibl_dn6;
        *var_dvth_dibl_dn7_slot = var_dvth_dibl_dn7;
        *var_dvth_dibl_dn8_slot = var_dvth_dibl_dn8;
        *var_dvth_dsc_slot = var_dvth_dsc;
        *var_dvth_dsc_dn5_slot = var_dvth_dsc_dn5;
        *var_dvth_dsc_dn6_slot = var_dvth_dsc_dn6;
        *var_dvth_nbody_slot = var_dvth_nbody;
        *var_dvth_rsce_slot = var_dvth_rsce;
        *var_dvth_rsce_dn3_slot = var_dvth_rsce_dn3;
        *var_dvth_rsce_dn4_slot = var_dvth_rsce_dn4;
        *var_dvth_rsce_dn5_slot = var_dvth_rsce_dn5;
        *var_dvth_rsce_dn6_slot = var_dvth_rsce_dn6;
        *var_dvth_rsce_dn7_slot = var_dvth_rsce_dn7;
        *var_dvth_rsce_dn8_slot = var_dvth_rsce_dn8;
        *var_dvth_vbg_slot = var_dvth_vbg;
        *var_dvth_vbg_dn3_slot = var_dvth_vbg_dn3;
        *var_dvth_vbg_dn4_slot = var_dvth_vbg_dn4;
        *var_dvth_vbg_dn5_slot = var_dvth_vbg_dn5;
        *var_dvth_vbg_dn6_slot = var_dvth_vbg_dn6;
        *var_dvth_vbg_dn7_slot = var_dvth_vbg_dn7;
        *var_dvth_vbg_dn8_slot = var_dvth_vbg_dn8;
        *var_dvth_vtroll_slot = var_dvth_vtroll;
        *var_dvth_vtroll_dn3_slot = var_dvth_vtroll_dn3;
        *var_dvth_vtroll_dn4_slot = var_dvth_vtroll_dn4;
        *var_dvth_vtroll_dn5_slot = var_dvth_vtroll_dn5;
        *var_dvth_vtroll_dn6_slot = var_dvth_vtroll_dn6;
        *var_dvth_vtroll_dn7_slot = var_dvth_vtroll_dn7;
        *var_dvth_vtroll_dn8_slot = var_dvth_vtroll_dn8;
        *var_gamma0_slot = var_gamma0;
        *var_guard73_slot = var_guard73;
        *var_guard74_slot = var_guard74;
        *var_guard75_slot = var_guard75;
        *var_kvbg_slot = var_kvbg;
        *var_kvbg_dn3_slot = var_kvbg_dn3;
        *var_kvbg_dn4_slot = var_kvbg_dn4;
        *var_kvbg_dn5_slot = var_kvbg_dn5;
        *var_kvbg_dn6_slot = var_kvbg_dn6;
        *var_kvbg_dn7_slot = var_kvbg_dn7;
        *var_kvbg_dn8_slot = var_kvbg_dn8;
        *var_nvtm_slot = var_nvtm;
        *var_nvtm_dn3_slot = var_nvtm_dn3;
        *var_nvtm_dn4_slot = var_nvtm_dn4;
        *var_nvtm_dn5_slot = var_nvtm_dn5;
        *var_nvtm_dn6_slot = var_nvtm_dn6;
        *var_nvtm_dn7_slot = var_nvtm_dn7;
        *var_nvtm_dn8_slot = var_nvtm_dn8;
        *var_phist_slot = var_phist;
        *var_phist_dn3_slot = var_phist_dn3;
        *var_phist_dn4_slot = var_phist_dn4;
        *var_phist_dn5_slot = var_phist_dn5;
        *var_phist_dn6_slot = var_phist_dn6;
        *var_phist_dn7_slot = var_phist_dn7;
        *var_phist_dn8_slot = var_phist_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_temp_slot = var_temp;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_vbgxpos_slot = var_vbgxpos;
        *var_vbgxpos_dn3_slot = var_vbgxpos_dn3;
        *var_vbgxpos_dn5_slot = var_vbgxpos_dn5;
        *var_vbgxpos_dn6_slot = var_vbgxpos_dn6;
        *var_vgfb2eff_slot = var_vgfb2eff;
        *var_vgfb2eff_dn5_slot = var_vgfb2eff_dn5;
        *var_vgfb2eff_dn6_slot = var_vgfb2eff_dn6;
        *var_vknee1_slot = var_vknee1;
        *var_vknee2_slot = var_vknee2;
        *var_vsubdep_slot = var_vsubdep;
        *var_vsubdep0_slot = var_vsubdep0;
        *var_vsubdep_dn3_slot = var_vsubdep_dn3;
        *var_vsubdep_dn4_slot = var_vsubdep_dn4;
        *var_vsubdep_dn5_slot = var_vsubdep_dn5;
        *var_vsubdep_dn6_slot = var_vsubdep_dn6;
        *var_vsubdep_dn7_slot = var_vsubdep_dn7;
        *var_vsubdep_dn8_slot = var_vsubdep_dn8;
    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        var_cox1: f64,
        var_cox2: f64,
        var_csi: f64,
        var_dvth_dibl: f64,
        var_dvth_dibl_dn3: f64,
        var_dvth_dibl_dn4: f64,
        var_dvth_dibl_dn5: f64,
        var_dvth_dibl_dn6: f64,
        var_dvth_dibl_dn7: f64,
        var_dvth_dibl_dn8: f64,
        var_dvth_dsc: f64,
        var_dvth_dsc_dn5: f64,
        var_dvth_dsc_dn6: f64,
        var_dvth_nbody: f64,
        var_dvth_rsce: f64,
        var_dvth_rsce_dn3: f64,
        var_dvth_rsce_dn4: f64,
        var_dvth_rsce_dn5: f64,
        var_dvth_rsce_dn6: f64,
        var_dvth_rsce_dn7: f64,
        var_dvth_rsce_dn8: f64,
        var_dvth_temp0: f64,
        var_dvth_temp0_dn4: f64,
        var_dvth_vbg: f64,
        var_dvth_vbg_dn3: f64,
        var_dvth_vbg_dn4: f64,
        var_dvth_vbg_dn5: f64,
        var_dvth_vbg_dn6: f64,
        var_dvth_vbg_dn7: f64,
        var_dvth_vbg_dn8: f64,
        var_dvth_vtroll: f64,
        var_dvth_vtroll_dn3: f64,
        var_dvth_vtroll_dn4: f64,
        var_dvth_vtroll_dn5: f64,
        var_dvth_vtroll_dn6: f64,
        var_dvth_vtroll_dn7: f64,
        var_dvth_vtroll_dn8: f64,
        var_epssi: f64,
        var_ni: f64,
        var_ni_dn3: f64,
        var_ni_dn4: f64,
        var_ni_dn5: f64,
        var_ni_dn6: f64,
        var_ni_dn7: f64,
        var_ni_dn8: f64,
        var_nvtm: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_phib: f64,
        var_phib_dn3: f64,
        var_phib_dn4: f64,
        var_phib_dn5: f64,
        var_phib_dn6: f64,
        var_phib_dn7: f64,
        var_phib_dn8: f64,
        var_tratio: f64,
        var_tratio_dn4: f64,
        var_vgfb1: f64,
        var_vgfb1_dn4: f64,
        var_vgfb1_dn5: f64,
        var_vgfb1_dn6: f64,
        var_vgfb1_dn8: f64,
        var_vgfb2: f64,
        var_vgfb2_dn3: f64,
        var_vgfb2_dn4: f64,
        var_vgfb2_dn5: f64,
        var_vgfb2_dn6: f64,
        var_vgfb2_dn7: f64,
        var_vgfb2_dn8: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_a0_slot: &mut f64,
        var_a0_dn3_slot: &mut f64,
        var_a0_dn4_slot: &mut f64,
        var_a0_dn5_slot: &mut f64,
        var_a0_dn6_slot: &mut f64,
        var_a0_dn7_slot: &mut f64,
        var_a0_dn8_slot: &mut f64,
        var_dvth_all_slot: &mut f64,
        var_dvth_all_dn3_slot: &mut f64,
        var_dvth_all_dn4_slot: &mut f64,
        var_dvth_all_dn5_slot: &mut f64,
        var_dvth_all_dn6_slot: &mut f64,
        var_dvth_all_dn7_slot: &mut f64,
        var_dvth_all_dn8_slot: &mut f64,
        var_dvth_temp_slot: &mut f64,
        var_dvth_temp_dn3_slot: &mut f64,
        var_dvth_temp_dn4_slot: &mut f64,
        var_dvth_temp_dn5_slot: &mut f64,
        var_dvth_temp_dn6_slot: &mut f64,
        var_dvth_temp_dn7_slot: &mut f64,
        var_dvth_temp_dn8_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_k1_slot: &mut f64,
        var_k1_2_slot: &mut f64,
        var_k2_slot: &mut f64,
        var_keq_k2_slot: &mut f64,
        var_lna0_slot: &mut f64,
        var_lna0_dn3_slot: &mut f64,
        var_lna0_dn4_slot: &mut f64,
        var_lna0_dn5_slot: &mut f64,
        var_lna0_dn6_slot: &mut f64,
        var_lna0_dn7_slot: &mut f64,
        var_lna0_dn8_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_0_slot: &mut f64,
        var_phi1_0_dn3_slot: &mut f64,
        var_phi1_0_dn4_slot: &mut f64,
        var_phi1_0_dn5_slot: &mut f64,
        var_phi1_0_dn6_slot: &mut f64,
        var_phi1_0_dn7_slot: &mut f64,
        var_phi1_0_dn8_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_phi2_slot: &mut f64,
        var_phi2_dn3_slot: &mut f64,
        var_phi2_dn4_slot: &mut f64,
        var_phi2_dn5_slot: &mut f64,
        var_phi2_dn6_slot: &mut f64,
        var_phi2_dn7_slot: &mut f64,
        var_phi2_dn8_slot: &mut f64,
        var_phi2sub_slot: &mut f64,
        var_phi2sub_dn3_slot: &mut f64,
        var_phi2sub_dn4_slot: &mut f64,
        var_phi2sub_dn5_slot: &mut f64,
        var_phi2sub_dn6_slot: &mut f64,
        var_phi2sub_dn7_slot: &mut f64,
        var_phi2sub_dn8_slot: &mut f64,
        var_phissat_slot: &mut f64,
        var_phissat_dn3_slot: &mut f64,
        var_phissat_dn4_slot: &mut f64,
        var_phissat_dn5_slot: &mut f64,
        var_phissat_dn6_slot: &mut f64,
        var_phissat_dn7_slot: &mut f64,
        var_phissat_dn8_slot: &mut f64,
        var_phissatback_slot: &mut f64,
        var_phissatback2_slot: &mut f64,
        var_phissatback2_dn3_slot: &mut f64,
        var_phissatback2_dn4_slot: &mut f64,
        var_phissatback2_dn5_slot: &mut f64,
        var_phissatback2_dn6_slot: &mut f64,
        var_phissatback2_dn7_slot: &mut f64,
        var_phissatback2_dn8_slot: &mut f64,
        var_phissatback_dn3_slot: &mut f64,
        var_phissatback_dn4_slot: &mut f64,
        var_phissatback_dn5_slot: &mut f64,
        var_phissatback_dn6_slot: &mut f64,
        var_phissatback_dn7_slot: &mut f64,
        var_phissatback_dn8_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_qcoth1_slot: &mut f64,
        var_qcoth1_dn3_slot: &mut f64,
        var_qcoth1_dn4_slot: &mut f64,
        var_qcoth1_dn5_slot: &mut f64,
        var_qcoth1_dn6_slot: &mut f64,
        var_qcoth1_dn7_slot: &mut f64,
        var_qcoth1_dn8_slot: &mut f64,
        var_qsq1_slot: &mut f64,
        var_qsq1_dn3_slot: &mut f64,
        var_qsq1_dn4_slot: &mut f64,
        var_qsq1_dn5_slot: &mut f64,
        var_qsq1_dn6_slot: &mut f64,
        var_qsq1_dn7_slot: &mut f64,
        var_qsq1_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt1_slot: &mut f64,
        var_qsqrt1_dn3_slot: &mut f64,
        var_qsqrt1_dn4_slot: &mut f64,
        var_qsqrt1_dn5_slot: &mut f64,
        var_qsqrt1_dn6_slot: &mut f64,
        var_qsqrt1_dn7_slot: &mut f64,
        var_qsqrt1_dn8_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_qt_slot: &mut f64,
        var_qt_dn3_slot: &mut f64,
        var_qt_dn4_slot: &mut f64,
        var_qt_dn5_slot: &mut f64,
        var_qt_dn6_slot: &mut f64,
        var_qt_dn7_slot: &mut f64,
        var_qt_dn8_slot: &mut f64,
        var_qth_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_vgfb1eff_slot: &mut f64,
        var_vgfb1eff_dn3_slot: &mut f64,
        var_vgfb1eff_dn4_slot: &mut f64,
        var_vgfb1eff_dn5_slot: &mut f64,
        var_vgfb1eff_dn6_slot: &mut f64,
        var_vgfb1eff_dn7_slot: &mut f64,
        var_vgfb1eff_dn8_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_dn3_slot: &mut f64,
        var_xg1_dn4_slot: &mut f64,
        var_xg1_dn5_slot: &mut f64,
        var_xg1_dn6_slot: &mut f64,
        var_xg1_dn7_slot: &mut f64,
        var_xg1_dn8_slot: &mut f64,
        var_xg2_slot: &mut f64,
        var_xg2_dn3_slot: &mut f64,
        var_xg2_dn4_slot: &mut f64,
        var_xg2_dn5_slot: &mut f64,
        var_xg2_dn6_slot: &mut f64,
        var_xg2_dn7_slot: &mut f64,
        var_xg2_dn8_slot: &mut f64,
    ) {
        let mut var_a0: f64 = *var_a0_slot;
        let mut var_a0_dn3: f64 = *var_a0_dn3_slot;
        let mut var_a0_dn4: f64 = *var_a0_dn4_slot;
        let mut var_a0_dn5: f64 = *var_a0_dn5_slot;
        let mut var_a0_dn6: f64 = *var_a0_dn6_slot;
        let mut var_a0_dn7: f64 = *var_a0_dn7_slot;
        let mut var_a0_dn8: f64 = *var_a0_dn8_slot;
        let mut var_dvth_all: f64 = *var_dvth_all_slot;
        let mut var_dvth_all_dn3: f64 = *var_dvth_all_dn3_slot;
        let mut var_dvth_all_dn4: f64 = *var_dvth_all_dn4_slot;
        let mut var_dvth_all_dn5: f64 = *var_dvth_all_dn5_slot;
        let mut var_dvth_all_dn6: f64 = *var_dvth_all_dn6_slot;
        let mut var_dvth_all_dn7: f64 = *var_dvth_all_dn7_slot;
        let mut var_dvth_all_dn8: f64 = *var_dvth_all_dn8_slot;
        let mut var_dvth_temp: f64 = *var_dvth_temp_slot;
        let mut var_dvth_temp_dn3: f64 = *var_dvth_temp_dn3_slot;
        let mut var_dvth_temp_dn4: f64 = *var_dvth_temp_dn4_slot;
        let mut var_dvth_temp_dn5: f64 = *var_dvth_temp_dn5_slot;
        let mut var_dvth_temp_dn6: f64 = *var_dvth_temp_dn6_slot;
        let mut var_dvth_temp_dn7: f64 = *var_dvth_temp_dn7_slot;
        let mut var_dvth_temp_dn8: f64 = *var_dvth_temp_dn8_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_k1: f64 = *var_k1_slot;
        let mut var_k1_2: f64 = *var_k1_2_slot;
        let mut var_k2: f64 = *var_k2_slot;
        let mut var_keq_k2: f64 = *var_keq_k2_slot;
        let mut var_lna0: f64 = *var_lna0_slot;
        let mut var_lna0_dn3: f64 = *var_lna0_dn3_slot;
        let mut var_lna0_dn4: f64 = *var_lna0_dn4_slot;
        let mut var_lna0_dn5: f64 = *var_lna0_dn5_slot;
        let mut var_lna0_dn6: f64 = *var_lna0_dn6_slot;
        let mut var_lna0_dn7: f64 = *var_lna0_dn7_slot;
        let mut var_lna0_dn8: f64 = *var_lna0_dn8_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_0: f64 = *var_phi1_0_slot;
        let mut var_phi1_0_dn3: f64 = *var_phi1_0_dn3_slot;
        let mut var_phi1_0_dn4: f64 = *var_phi1_0_dn4_slot;
        let mut var_phi1_0_dn5: f64 = *var_phi1_0_dn5_slot;
        let mut var_phi1_0_dn6: f64 = *var_phi1_0_dn6_slot;
        let mut var_phi1_0_dn7: f64 = *var_phi1_0_dn7_slot;
        let mut var_phi1_0_dn8: f64 = *var_phi1_0_dn8_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_phi2: f64 = *var_phi2_slot;
        let mut var_phi2_dn3: f64 = *var_phi2_dn3_slot;
        let mut var_phi2_dn4: f64 = *var_phi2_dn4_slot;
        let mut var_phi2_dn5: f64 = *var_phi2_dn5_slot;
        let mut var_phi2_dn6: f64 = *var_phi2_dn6_slot;
        let mut var_phi2_dn7: f64 = *var_phi2_dn7_slot;
        let mut var_phi2_dn8: f64 = *var_phi2_dn8_slot;
        let mut var_phi2sub: f64 = *var_phi2sub_slot;
        let mut var_phi2sub_dn3: f64 = *var_phi2sub_dn3_slot;
        let mut var_phi2sub_dn4: f64 = *var_phi2sub_dn4_slot;
        let mut var_phi2sub_dn5: f64 = *var_phi2sub_dn5_slot;
        let mut var_phi2sub_dn6: f64 = *var_phi2sub_dn6_slot;
        let mut var_phi2sub_dn7: f64 = *var_phi2sub_dn7_slot;
        let mut var_phi2sub_dn8: f64 = *var_phi2sub_dn8_slot;
        let mut var_phissat: f64 = *var_phissat_slot;
        let mut var_phissat_dn3: f64 = *var_phissat_dn3_slot;
        let mut var_phissat_dn4: f64 = *var_phissat_dn4_slot;
        let mut var_phissat_dn5: f64 = *var_phissat_dn5_slot;
        let mut var_phissat_dn6: f64 = *var_phissat_dn6_slot;
        let mut var_phissat_dn7: f64 = *var_phissat_dn7_slot;
        let mut var_phissat_dn8: f64 = *var_phissat_dn8_slot;
        let mut var_phissatback: f64 = *var_phissatback_slot;
        let mut var_phissatback2: f64 = *var_phissatback2_slot;
        let mut var_phissatback2_dn3: f64 = *var_phissatback2_dn3_slot;
        let mut var_phissatback2_dn4: f64 = *var_phissatback2_dn4_slot;
        let mut var_phissatback2_dn5: f64 = *var_phissatback2_dn5_slot;
        let mut var_phissatback2_dn6: f64 = *var_phissatback2_dn6_slot;
        let mut var_phissatback2_dn7: f64 = *var_phissatback2_dn7_slot;
        let mut var_phissatback2_dn8: f64 = *var_phissatback2_dn8_slot;
        let mut var_phissatback_dn3: f64 = *var_phissatback_dn3_slot;
        let mut var_phissatback_dn4: f64 = *var_phissatback_dn4_slot;
        let mut var_phissatback_dn5: f64 = *var_phissatback_dn5_slot;
        let mut var_phissatback_dn6: f64 = *var_phissatback_dn6_slot;
        let mut var_phissatback_dn7: f64 = *var_phissatback_dn7_slot;
        let mut var_phissatback_dn8: f64 = *var_phissatback_dn8_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_qcoth1: f64 = *var_qcoth1_slot;
        let mut var_qcoth1_dn3: f64 = *var_qcoth1_dn3_slot;
        let mut var_qcoth1_dn4: f64 = *var_qcoth1_dn4_slot;
        let mut var_qcoth1_dn5: f64 = *var_qcoth1_dn5_slot;
        let mut var_qcoth1_dn6: f64 = *var_qcoth1_dn6_slot;
        let mut var_qcoth1_dn7: f64 = *var_qcoth1_dn7_slot;
        let mut var_qcoth1_dn8: f64 = *var_qcoth1_dn8_slot;
        let mut var_qsq1: f64 = *var_qsq1_slot;
        let mut var_qsq1_dn3: f64 = *var_qsq1_dn3_slot;
        let mut var_qsq1_dn4: f64 = *var_qsq1_dn4_slot;
        let mut var_qsq1_dn5: f64 = *var_qsq1_dn5_slot;
        let mut var_qsq1_dn6: f64 = *var_qsq1_dn6_slot;
        let mut var_qsq1_dn7: f64 = *var_qsq1_dn7_slot;
        let mut var_qsq1_dn8: f64 = *var_qsq1_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt1: f64 = *var_qsqrt1_slot;
        let mut var_qsqrt1_dn3: f64 = *var_qsqrt1_dn3_slot;
        let mut var_qsqrt1_dn4: f64 = *var_qsqrt1_dn4_slot;
        let mut var_qsqrt1_dn5: f64 = *var_qsqrt1_dn5_slot;
        let mut var_qsqrt1_dn6: f64 = *var_qsqrt1_dn6_slot;
        let mut var_qsqrt1_dn7: f64 = *var_qsqrt1_dn7_slot;
        let mut var_qsqrt1_dn8: f64 = *var_qsqrt1_dn8_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_qt: f64 = *var_qt_slot;
        let mut var_qt_dn3: f64 = *var_qt_dn3_slot;
        let mut var_qt_dn4: f64 = *var_qt_dn4_slot;
        let mut var_qt_dn5: f64 = *var_qt_dn5_slot;
        let mut var_qt_dn6: f64 = *var_qt_dn6_slot;
        let mut var_qt_dn7: f64 = *var_qt_dn7_slot;
        let mut var_qt_dn8: f64 = *var_qt_dn8_slot;
        let mut var_qth: f64 = *var_qth_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_vgfb1eff: f64 = *var_vgfb1eff_slot;
        let mut var_vgfb1eff_dn3: f64 = *var_vgfb1eff_dn3_slot;
        let mut var_vgfb1eff_dn4: f64 = *var_vgfb1eff_dn4_slot;
        let mut var_vgfb1eff_dn5: f64 = *var_vgfb1eff_dn5_slot;
        let mut var_vgfb1eff_dn6: f64 = *var_vgfb1eff_dn6_slot;
        let mut var_vgfb1eff_dn7: f64 = *var_vgfb1eff_dn7_slot;
        let mut var_vgfb1eff_dn8: f64 = *var_vgfb1eff_dn8_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_dn3: f64 = *var_xg1_dn3_slot;
        let mut var_xg1_dn4: f64 = *var_xg1_dn4_slot;
        let mut var_xg1_dn5: f64 = *var_xg1_dn5_slot;
        let mut var_xg1_dn6: f64 = *var_xg1_dn6_slot;
        let mut var_xg1_dn7: f64 = *var_xg1_dn7_slot;
        let mut var_xg1_dn8: f64 = *var_xg1_dn8_slot;
        let mut var_xg2: f64 = *var_xg2_slot;
        let mut var_xg2_dn3: f64 = *var_xg2_dn3_slot;
        let mut var_xg2_dn4: f64 = *var_xg2_dn4_slot;
        let mut var_xg2_dn5: f64 = *var_xg2_dn5_slot;
        let mut var_xg2_dn6: f64 = *var_xg2_dn6_slot;
        let mut var_xg2_dn7: f64 = *var_xg2_dn7_slot;
        let mut var_xg2_dn8: f64 = *var_xg2_dn8_slot;

        let assign4900_e5651: f64 = (var_tratio - 1.0);
        let assign4900_e5652: f64 = (var_t0 * assign4900_e5651);
        let assign4900_e5653: f64 = (var_dvth_temp0 + assign4900_e5652);
        var_dvth_temp = assign4900_e5653;
        var_dvth_temp_dn3 = (var_t0_dn3 * assign4900_e5651);
        var_dvth_temp_dn4 = (var_dvth_temp0_dn4 + ((var_t0_dn4 * assign4900_e5651) + (var_t0 * var_tratio_dn4)));
        var_dvth_temp_dn5 = (var_t0_dn5 * assign4900_e5651);
        var_dvth_temp_dn6 = (var_t0_dn6 * assign4900_e5651);
        var_dvth_temp_dn7 = (var_t0_dn7 * assign4900_e5651);
        var_dvth_temp_dn8 = (var_t0_dn8 * assign4900_e5651);

        let assign4910_e5656: f64 = (var_dvth_vtroll + var_dvth_dibl);
        let assign4910_e5658: f64 = (assign4910_e5656 + var_dvth_rsce);
        let assign4910_e5660: f64 = (assign4910_e5658 + var_dvth_dsc);
        let assign4910_e5662: f64 = (assign4910_e5660 + var_dvth_nbody);
        let assign4910_e5664: f64 = (assign4910_e5662 + var_dvth_temp);
        let assign4910_e5666: f64 = (assign4910_e5664 + var_dvth_vbg);
        var_dvth_all = assign4910_e5666;
        var_dvth_all_dn3 = ((((var_dvth_vtroll_dn3 + var_dvth_dibl_dn3) + var_dvth_rsce_dn3) + var_dvth_temp_dn3) + var_dvth_vbg_dn3);
        var_dvth_all_dn4 = ((((var_dvth_vtroll_dn4 + var_dvth_dibl_dn4) + var_dvth_rsce_dn4) + var_dvth_temp_dn4) + var_dvth_vbg_dn4);
        var_dvth_all_dn5 = (((((var_dvth_vtroll_dn5 + var_dvth_dibl_dn5) + var_dvth_rsce_dn5) + var_dvth_dsc_dn5) + var_dvth_temp_dn5) + var_dvth_vbg_dn5);
        var_dvth_all_dn6 = (((((var_dvth_vtroll_dn6 + var_dvth_dibl_dn6) + var_dvth_rsce_dn6) + var_dvth_dsc_dn6) + var_dvth_temp_dn6) + var_dvth_vbg_dn6);
        var_dvth_all_dn7 = ((((var_dvth_vtroll_dn7 + var_dvth_dibl_dn7) + var_dvth_rsce_dn7) + var_dvth_temp_dn7) + var_dvth_vbg_dn7);
        var_dvth_all_dn8 = ((((var_dvth_vtroll_dn8 + var_dvth_dibl_dn8) + var_dvth_rsce_dn8) + var_dvth_temp_dn8) + var_dvth_vbg_dn8);

        let assign4920_e5669: f64 = (var_vgfb1 - var_dvth_all);
        let assign4920_e5671: f64 = (assign4920_e5669 + p.p10);
        var_vgfb1eff = assign4920_e5671;
        var_vgfb1eff_dn3 = (-var_dvth_all_dn3);
        var_vgfb1eff_dn4 = (var_vgfb1_dn4 - var_dvth_all_dn4);
        var_vgfb1eff_dn5 = (var_vgfb1_dn5 - var_dvth_all_dn5);
        var_vgfb1eff_dn6 = (var_vgfb1_dn6 - var_dvth_all_dn6);
        var_vgfb1eff_dn7 = (-var_dvth_all_dn7);
        var_vgfb1eff_dn8 = (var_vgfb1_dn8 - var_dvth_all_dn8);

        let assign4930_e5674: f64 = (2.0 * 1.60219e-19);
        let assign4930_e5676: f64 = (assign4930_e5674 * var_ni);
        let assign4930_e5678: f64 = (assign4930_e5676 * p.p49);
        let assign4930_e5680: f64 = (assign4930_e5678 * p.p49);
        let assign4930_e5683: f64 = (var_epssi * var_vtm);
        let assign4930_e5684: f64 = (assign4930_e5680 / assign4930_e5683);
        var_a0 = assign4930_e5684;
        var_a0_dn3 = ((((assign4930_e5674 * var_ni_dn3) * p.p49) * p.p49) / assign4930_e5683);
        var_a0_dn4 = ((((((assign4930_e5674 * var_ni_dn4) * p.p49) * p.p49) * assign4930_e5683) - (assign4930_e5680 * (var_epssi * var_vtm_dn4))) / (assign4930_e5683 * assign4930_e5683));
        var_a0_dn5 = ((((assign4930_e5674 * var_ni_dn5) * p.p49) * p.p49) / assign4930_e5683);
        var_a0_dn6 = ((((assign4930_e5674 * var_ni_dn6) * p.p49) * p.p49) / assign4930_e5683);
        var_a0_dn7 = ((((assign4930_e5674 * var_ni_dn7) * p.p49) * p.p49) / assign4930_e5683);
        var_a0_dn8 = ((((assign4930_e5674 * var_ni_dn8) * p.p49) * p.p49) / assign4930_e5683);

        let assign4940_e5687: f64 = (var_cox1 / var_csi);
        var_k1 = assign4940_e5687;

        let assign4950_e5690: f64 = (var_cox2 / var_csi);
        var_k2 = assign4950_e5690;

        let assign4960_e5692: f64 = (var_a0).ln();
        var_lna0 = assign4960_e5692;
        var_lna0_dn3 = (var_a0_dn3 / var_a0);
        var_lna0_dn4 = (var_a0_dn4 / var_a0);
        var_lna0_dn5 = (var_a0_dn5 / var_a0);
        var_lna0_dn6 = (var_a0_dn6 / var_a0);
        var_lna0_dn7 = (var_a0_dn7 / var_a0);
        var_lna0_dn8 = (var_a0_dn8 / var_a0);

        let assign4970_e5694: f64 = (39.47841_f64).ln();
        let assign4970_e5696: f64 = (assign4970_e5694 - var_lna0);
        var_phi1_0 = assign4970_e5696;
        var_phi1_0_dn3 = (-var_lna0_dn3);
        var_phi1_0_dn4 = (-var_lna0_dn4);
        var_phi1_0_dn5 = (-var_lna0_dn5);
        var_phi1_0_dn6 = (-var_lna0_dn6);
        var_phi1_0_dn7 = (-var_lna0_dn7);
        var_phi1_0_dn8 = (-var_lna0_dn8);

        let assign4980_e5699: f64 = (var_k1 * var_k1);
        var_k1_2 = assign4980_e5699;

        let assign4990_e5703: f64 = (var_k2 * var_k1);
        let assign4990_e5705: f64 = (assign4990_e5703 + var_k2);
        let assign4990_e5707: f64 = (assign4990_e5705 + var_k1);
        let assign4990_e5708: f64 = (var_k1 / assign4990_e5707);
        var_keq_k2 = assign4990_e5708;

        var_qth = 1.0;

        let assign5010_e5712: f64 = (var_k1_2 * var_qth);
        let assign5010_e5714: f64 = (assign5010_e5712 * var_qth);
        let assign5010_e5718: f64 = (var_phib * 2.0);
        let assign5010_e5719: f64 = { let limited_exp_arg = assign5010_e5718; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign5010_e5720: f64 = (var_a0 * assign5010_e5719);
        let assign5010_e5721: f64 = (assign5010_e5714 - assign5010_e5720);
        var_qsq1 = assign5010_e5721;
        var_qsq1_dn3 = (-((var_a0_dn3 * assign5010_e5719) + (var_a0 * ({ let limited_exp_arg = assign5010_e5718; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_phib_dn3 * 2.0)))));
        var_qsq1_dn4 = (-((var_a0_dn4 * assign5010_e5719) + (var_a0 * ({ let limited_exp_arg = assign5010_e5718; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_phib_dn4 * 2.0)))));
        var_qsq1_dn5 = (-((var_a0_dn5 * assign5010_e5719) + (var_a0 * ({ let limited_exp_arg = assign5010_e5718; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_phib_dn5 * 2.0)))));
        var_qsq1_dn6 = (-((var_a0_dn6 * assign5010_e5719) + (var_a0 * ({ let limited_exp_arg = assign5010_e5718; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_phib_dn6 * 2.0)))));
        var_qsq1_dn7 = (-((var_a0_dn7 * assign5010_e5719) + (var_a0 * ({ let limited_exp_arg = assign5010_e5718; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_phib_dn7 * 2.0)))));
        var_qsq1_dn8 = (-((var_a0_dn8 * assign5010_e5719) + (var_a0 * ({ let limited_exp_arg = assign5010_e5718; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_phib_dn8 * 2.0)))));

        let assign5020_e5723: f64 = (var_qsq1).sqrt();
        var_qsqrt1 = assign5020_e5723;
        var_qsqrt1_dn3 = (var_qsq1_dn3 / (2.0 * assign5020_e5723));
        var_qsqrt1_dn4 = (var_qsq1_dn4 / (2.0 * assign5020_e5723));
        var_qsqrt1_dn5 = (var_qsq1_dn5 / (2.0 * assign5020_e5723));
        var_qsqrt1_dn6 = (var_qsq1_dn6 / (2.0 * assign5020_e5723));
        var_qsqrt1_dn7 = (var_qsq1_dn7 / (2.0 * assign5020_e5723));
        var_qsqrt1_dn8 = (var_qsq1_dn8 / (2.0 * assign5020_e5723));

        let assign5030_e5727: f64 = (var_qsqrt1 / 8.0);
        let assign5030_e5728: f64 = (1.0 - assign5030_e5727);
        let assign5030_e5732: f64 = (var_qsqrt1 / 24.0);
        let assign5030_e5733: f64 = (0.5 - assign5030_e5732);
        let assign5030_e5734: f64 = (assign5030_e5728 / assign5030_e5733);
        var_qcoth1 = assign5030_e5734;
        var_qcoth1_dn3 = ((((-(var_qsqrt1_dn3 / 8.0)) * assign5030_e5733) - (assign5030_e5728 * (-(var_qsqrt1_dn3 / 24.0)))) / (assign5030_e5733 * assign5030_e5733));
        var_qcoth1_dn4 = ((((-(var_qsqrt1_dn4 / 8.0)) * assign5030_e5733) - (assign5030_e5728 * (-(var_qsqrt1_dn4 / 24.0)))) / (assign5030_e5733 * assign5030_e5733));
        var_qcoth1_dn5 = ((((-(var_qsqrt1_dn5 / 8.0)) * assign5030_e5733) - (assign5030_e5728 * (-(var_qsqrt1_dn5 / 24.0)))) / (assign5030_e5733 * assign5030_e5733));
        var_qcoth1_dn6 = ((((-(var_qsqrt1_dn6 / 8.0)) * assign5030_e5733) - (assign5030_e5728 * (-(var_qsqrt1_dn6 / 24.0)))) / (assign5030_e5733 * assign5030_e5733));
        var_qcoth1_dn7 = ((((-(var_qsqrt1_dn7 / 8.0)) * assign5030_e5733) - (assign5030_e5728 * (-(var_qsqrt1_dn7 / 24.0)))) / (assign5030_e5733 * assign5030_e5733));
        var_qcoth1_dn8 = ((((-(var_qsqrt1_dn8 / 8.0)) * assign5030_e5733) - (assign5030_e5728 * (-(var_qsqrt1_dn8 / 24.0)))) / (assign5030_e5733 * assign5030_e5733));

        let assign5040_e5738: f64 = (var_k1 * var_k1);
        let assign5040_e5740: f64 = (assign5040_e5738 * var_qth);
        let assign5040_e5742: f64 = (assign5040_e5740 * var_qth);
        let assign5040_e5745: f64 = (var_k1 * var_qth);
        let assign5040_e5747: f64 = (assign5040_e5745 * var_qcoth1);
        let assign5040_e5748: f64 = (assign5040_e5742 + assign5040_e5747);
        let assign5040_e5750: f64 = (assign5040_e5748).max(1e-38);
        let assign5040_e5751: f64 = (assign5040_e5750).ln();
        let assign5040_e5752: f64 = (1.0 + assign5040_e5751);
        let assign5040_e5755: f64 = (var_a0).max(1e-38);
        let assign5040_e5756: f64 = (assign5040_e5755).ln();
        let assign5040_e5757: f64 = (assign5040_e5752 - assign5040_e5756);
        let assign5040_e5759: f64 = (assign5040_e5757 * var_vtm);
        var_t1 = assign5040_e5759;
        var_t1_dn3 = (((if assign5040_e5748 >= 1e-38 { (assign5040_e5745 * var_qcoth1_dn3) } else { 0.0 } / assign5040_e5750) - (if var_a0 >= 1e-38 { var_a0_dn3 } else { 0.0 } / assign5040_e5755)) * var_vtm);
        var_t1_dn4 = ((((if assign5040_e5748 >= 1e-38 { (assign5040_e5745 * var_qcoth1_dn4) } else { 0.0 } / assign5040_e5750) - (if var_a0 >= 1e-38 { var_a0_dn4 } else { 0.0 } / assign5040_e5755)) * var_vtm) + (assign5040_e5757 * var_vtm_dn4));
        var_t1_dn5 = (((if assign5040_e5748 >= 1e-38 { (assign5040_e5745 * var_qcoth1_dn5) } else { 0.0 } / assign5040_e5750) - (if var_a0 >= 1e-38 { var_a0_dn5 } else { 0.0 } / assign5040_e5755)) * var_vtm);
        var_t1_dn6 = (((if assign5040_e5748 >= 1e-38 { (assign5040_e5745 * var_qcoth1_dn6) } else { 0.0 } / assign5040_e5750) - (if var_a0 >= 1e-38 { var_a0_dn6 } else { 0.0 } / assign5040_e5755)) * var_vtm);
        var_t1_dn7 = (((if assign5040_e5748 >= 1e-38 { (assign5040_e5745 * var_qcoth1_dn7) } else { 0.0 } / assign5040_e5750) - (if var_a0 >= 1e-38 { var_a0_dn7 } else { 0.0 } / assign5040_e5755)) * var_vtm);
        var_t1_dn8 = (((if assign5040_e5748 >= 1e-38 { (assign5040_e5745 * var_qcoth1_dn8) } else { 0.0 } / assign5040_e5750) - (if var_a0 >= 1e-38 { var_a0_dn8 } else { 0.0 } / assign5040_e5755)) * var_vtm);

        let assign5060_e5774: f64 = (var_vgfb1eff / var_nvtm);
        var_xg1 = assign5060_e5774;
        var_xg1_dn3 = (((var_vgfb1eff_dn3 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn3)) / (var_nvtm * var_nvtm));
        var_xg1_dn4 = (((var_vgfb1eff_dn4 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn4)) / (var_nvtm * var_nvtm));
        var_xg1_dn5 = (((var_vgfb1eff_dn5 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn5)) / (var_nvtm * var_nvtm));
        var_xg1_dn6 = (((var_vgfb1eff_dn6 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn6)) / (var_nvtm * var_nvtm));
        var_xg1_dn7 = (((var_vgfb1eff_dn7 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn7)) / (var_nvtm * var_nvtm));
        var_xg1_dn8 = (((var_vgfb1eff_dn8 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn8)) / (var_nvtm * var_nvtm));

        let assign5070_e5777: f64 = (var_vgfb2 - var_dvth_all);
        let assign5070_e5779: f64 = (assign5070_e5777 + p.p10);
        let assign5070_e5781: f64 = (assign5070_e5779 / var_nvtm);
        var_xg2 = assign5070_e5781;
        var_xg2_dn3 = ((((var_vgfb2_dn3 - var_dvth_all_dn3) * var_nvtm) - (assign5070_e5779 * var_nvtm_dn3)) / (var_nvtm * var_nvtm));
        var_xg2_dn4 = ((((var_vgfb2_dn4 - var_dvth_all_dn4) * var_nvtm) - (assign5070_e5779 * var_nvtm_dn4)) / (var_nvtm * var_nvtm));
        var_xg2_dn5 = ((((var_vgfb2_dn5 - var_dvth_all_dn5) * var_nvtm) - (assign5070_e5779 * var_nvtm_dn5)) / (var_nvtm * var_nvtm));
        var_xg2_dn6 = ((((var_vgfb2_dn6 - var_dvth_all_dn6) * var_nvtm) - (assign5070_e5779 * var_nvtm_dn6)) / (var_nvtm * var_nvtm));
        var_xg2_dn7 = ((((var_vgfb2_dn7 - var_dvth_all_dn7) * var_nvtm) - (assign5070_e5779 * var_nvtm_dn7)) / (var_nvtm * var_nvtm));
        var_xg2_dn8 = ((((var_vgfb2_dn8 - var_dvth_all_dn8) * var_nvtm) - (assign5070_e5779 * var_nvtm_dn8)) / (var_nvtm * var_nvtm));

        let assign5080_e5785: f64 = (var_xg1 - var_phi1_0);
        let assign5080_e5786: f64 = (var_k1_2 * assign5080_e5785);
        let assign5080_e5789: f64 = (var_xg1 - var_phi1_0);
        let assign5080_e5790: f64 = (assign5080_e5786 * assign5080_e5789);
        let assign5080_e5792: f64 = (assign5080_e5790 + 39.47841);
        let assign5080_e5793: f64 = (assign5080_e5792).ln();
        let assign5080_e5795: f64 = (assign5080_e5793 - var_lna0);
        var_phissatback = assign5080_e5795;
        var_phissatback_dn3 = (((((var_k1_2 * (var_xg1_dn3 - var_phi1_0_dn3)) * assign5080_e5789) + (assign5080_e5786 * (var_xg1_dn3 - var_phi1_0_dn3))) / assign5080_e5792) - var_lna0_dn3);
        var_phissatback_dn4 = (((((var_k1_2 * (var_xg1_dn4 - var_phi1_0_dn4)) * assign5080_e5789) + (assign5080_e5786 * (var_xg1_dn4 - var_phi1_0_dn4))) / assign5080_e5792) - var_lna0_dn4);
        var_phissatback_dn5 = (((((var_k1_2 * (var_xg1_dn5 - var_phi1_0_dn5)) * assign5080_e5789) + (assign5080_e5786 * (var_xg1_dn5 - var_phi1_0_dn5))) / assign5080_e5792) - var_lna0_dn5);
        var_phissatback_dn6 = (((((var_k1_2 * (var_xg1_dn6 - var_phi1_0_dn6)) * assign5080_e5789) + (assign5080_e5786 * (var_xg1_dn6 - var_phi1_0_dn6))) / assign5080_e5792) - var_lna0_dn6);
        var_phissatback_dn7 = (((((var_k1_2 * (var_xg1_dn7 - var_phi1_0_dn7)) * assign5080_e5789) + (assign5080_e5786 * (var_xg1_dn7 - var_phi1_0_dn7))) / assign5080_e5792) - var_lna0_dn7);
        var_phissatback_dn8 = (((((var_k1_2 * (var_xg1_dn8 - var_phi1_0_dn8)) * assign5080_e5789) + (assign5080_e5786 * (var_xg1_dn8 - var_phi1_0_dn8))) / assign5080_e5792) - var_lna0_dn8);

        let assign5090_e5799: f64 = (var_xg1 - var_phi1_0);
        let assign5090_e5800: f64 = (var_k1_2 * assign5090_e5799);
        let assign5090_e5803: f64 = (var_xg1 - var_phi1_0);
        let assign5090_e5804: f64 = (assign5090_e5800 * assign5090_e5803);
        let assign5090_e5806: f64 = (assign5090_e5804 + 39.47841);
        let assign5090_e5807: f64 = (assign5090_e5806).ln();
        let assign5090_e5809: f64 = (assign5090_e5807 - var_lna0);
        var_phissat = assign5090_e5809;
        var_phissat_dn3 = (((((var_k1_2 * (var_xg1_dn3 - var_phi1_0_dn3)) * assign5090_e5803) + (assign5090_e5800 * (var_xg1_dn3 - var_phi1_0_dn3))) / assign5090_e5806) - var_lna0_dn3);
        var_phissat_dn4 = (((((var_k1_2 * (var_xg1_dn4 - var_phi1_0_dn4)) * assign5090_e5803) + (assign5090_e5800 * (var_xg1_dn4 - var_phi1_0_dn4))) / assign5090_e5806) - var_lna0_dn4);
        var_phissat_dn5 = (((((var_k1_2 * (var_xg1_dn5 - var_phi1_0_dn5)) * assign5090_e5803) + (assign5090_e5800 * (var_xg1_dn5 - var_phi1_0_dn5))) / assign5090_e5806) - var_lna0_dn5);
        var_phissat_dn6 = (((((var_k1_2 * (var_xg1_dn6 - var_phi1_0_dn6)) * assign5090_e5803) + (assign5090_e5800 * (var_xg1_dn6 - var_phi1_0_dn6))) / assign5090_e5806) - var_lna0_dn6);
        var_phissat_dn7 = (((((var_k1_2 * (var_xg1_dn7 - var_phi1_0_dn7)) * assign5090_e5803) + (assign5090_e5800 * (var_xg1_dn7 - var_phi1_0_dn7))) / assign5090_e5806) - var_lna0_dn7);
        var_phissat_dn8 = (((((var_k1_2 * (var_xg1_dn8 - var_phi1_0_dn8)) * assign5090_e5803) + (assign5090_e5800 * (var_xg1_dn8 - var_phi1_0_dn8))) / assign5090_e5806) - var_lna0_dn8);

        let assign5100_e5813: f64 = (var_k2 * var_xg2);
        let assign5100_e5814: f64 = (var_phissat + assign5100_e5813);
        let assign5100_e5817: f64 = (1.0 + var_k2);
        let assign5100_e5818: f64 = (assign5100_e5814 / assign5100_e5817);
        var_phissatback2 = assign5100_e5818;
        var_phissatback2_dn3 = ((var_phissat_dn3 + (var_k2 * var_xg2_dn3)) / assign5100_e5817);
        var_phissatback2_dn4 = ((var_phissat_dn4 + (var_k2 * var_xg2_dn4)) / assign5100_e5817);
        var_phissatback2_dn5 = ((var_phissat_dn5 + (var_k2 * var_xg2_dn5)) / assign5100_e5817);
        var_phissatback2_dn6 = ((var_phissat_dn6 + (var_k2 * var_xg2_dn6)) / assign5100_e5817);
        var_phissatback2_dn7 = ((var_phissat_dn7 + (var_k2 * var_xg2_dn7)) / assign5100_e5817);
        var_phissatback2_dn8 = ((var_phissat_dn8 + (var_k2 * var_xg2_dn8)) / assign5100_e5817);

        let assign5110_e5823: f64 = (var_xg1 - var_xg2);
        let assign5110_e5824: f64 = (var_keq_k2 * assign5110_e5823);
        let assign5110_e5825: f64 = (var_xg2 + assign5110_e5824);
        var_phi2sub = assign5110_e5825;
        var_phi2sub_dn3 = (var_xg2_dn3 + (var_keq_k2 * (var_xg1_dn3 - var_xg2_dn3)));
        var_phi2sub_dn4 = (var_xg2_dn4 + (var_keq_k2 * (var_xg1_dn4 - var_xg2_dn4)));
        var_phi2sub_dn5 = (var_xg2_dn5 + (var_keq_k2 * (var_xg1_dn5 - var_xg2_dn5)));
        var_phi2sub_dn6 = (var_xg2_dn6 + (var_keq_k2 * (var_xg1_dn6 - var_xg2_dn6)));
        var_phi2sub_dn7 = (var_xg2_dn7 + (var_keq_k2 * (var_xg1_dn7 - var_xg2_dn7)));
        var_phi2sub_dn8 = (var_xg2_dn8 + (var_keq_k2 * (var_xg1_dn8 - var_xg2_dn8)));

        let assign5120_e5828: f64 = (var_phi2sub).min(var_phissatback);
        var_phi2 = assign5120_e5828;
        var_phi2_dn3 = if var_phi2sub <= var_phissatback { var_phi2sub_dn3 } else { var_phissatback_dn3 };
        var_phi2_dn4 = if var_phi2sub <= var_phissatback { var_phi2sub_dn4 } else { var_phissatback_dn4 };
        var_phi2_dn5 = if var_phi2sub <= var_phissatback { var_phi2sub_dn5 } else { var_phissatback_dn5 };
        var_phi2_dn6 = if var_phi2sub <= var_phissatback { var_phi2sub_dn6 } else { var_phissatback_dn6 };
        var_phi2_dn7 = if var_phi2sub <= var_phissatback { var_phi2sub_dn7 } else { var_phissatback_dn7 };
        var_phi2_dn8 = if var_phi2sub <= var_phissatback { var_phi2sub_dn8 } else { var_phissatback_dn8 };

        let assign5130_e5831: f64 = (var_phi2).min(var_phi1_0);
        var_phi2 = assign5130_e5831;
        var_phi2_dn3 = if var_phi2 <= var_phi1_0 { var_phi2_dn3 } else { var_phi1_0_dn3 };
        var_phi2_dn4 = if var_phi2 <= var_phi1_0 { var_phi2_dn4 } else { var_phi1_0_dn4 };
        var_phi2_dn5 = if var_phi2 <= var_phi1_0 { var_phi2_dn5 } else { var_phi1_0_dn5 };
        var_phi2_dn6 = if var_phi2 <= var_phi1_0 { var_phi2_dn6 } else { var_phi1_0_dn6 };
        var_phi2_dn7 = if var_phi2 <= var_phi1_0 { var_phi2_dn7 } else { var_phi1_0_dn7 };
        var_phi2_dn8 = if var_phi2 <= var_phi1_0 { var_phi2_dn8 } else { var_phi1_0_dn8 };

        let assign5140_e5835: f64 = (var_k1 * var_xg1);
        let assign5140_e5836: f64 = (var_phi2 + assign5140_e5835);
        let assign5140_e5839: f64 = (1.0 + var_k1);
        let assign5140_e5840: f64 = (assign5140_e5836 / assign5140_e5839);
        var_phi1 = assign5140_e5840;
        var_phi1_dn3 = ((var_phi2_dn3 + (var_k1 * var_xg1_dn3)) / assign5140_e5839);
        var_phi1_dn4 = ((var_phi2_dn4 + (var_k1 * var_xg1_dn4)) / assign5140_e5839);
        var_phi1_dn5 = ((var_phi2_dn5 + (var_k1 * var_xg1_dn5)) / assign5140_e5839);
        var_phi1_dn6 = ((var_phi2_dn6 + (var_k1 * var_xg1_dn6)) / assign5140_e5839);
        var_phi1_dn7 = ((var_phi2_dn7 + (var_k1 * var_xg1_dn7)) / assign5140_e5839);
        var_phi1_dn8 = ((var_phi2_dn8 + (var_k1 * var_xg1_dn8)) / assign5140_e5839);

        let assign5150_e5843: f64 = (var_phi1 - var_phi2);
        var_t0 = assign5150_e5843;
        var_t0_dn3 = (var_phi1_dn3 - var_phi2_dn3);
        var_t0_dn4 = (var_phi1_dn4 - var_phi2_dn4);
        var_t0_dn5 = (var_phi1_dn5 - var_phi2_dn5);
        var_t0_dn6 = (var_phi1_dn6 - var_phi2_dn6);
        var_t0_dn7 = (var_phi1_dn7 - var_phi2_dn7);
        var_t0_dn8 = (var_phi1_dn8 - var_phi2_dn8);

        let assign5160_e5845: f64 = { let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign5160_e5847: f64 = { let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign5160_e5849: f64 = (assign5160_e5847 - 1.0);
        let assign5160_e5850: f64 = (assign5160_e5845 * assign5160_e5849);
        let assign5160_e5852: f64 = (assign5160_e5850 / var_t0);
        var_t3 = assign5160_e5852;
        var_t3_dn3 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn3) * assign5160_e5849) + (assign5160_e5845 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn3))) * var_t0) - (assign5160_e5850 * var_t0_dn3)) / (var_t0 * var_t0));
        var_t3_dn4 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn4) * assign5160_e5849) + (assign5160_e5845 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn4))) * var_t0) - (assign5160_e5850 * var_t0_dn4)) / (var_t0 * var_t0));
        var_t3_dn5 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn5) * assign5160_e5849) + (assign5160_e5845 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn5))) * var_t0) - (assign5160_e5850 * var_t0_dn5)) / (var_t0 * var_t0));
        var_t3_dn6 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn6) * assign5160_e5849) + (assign5160_e5845 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn6))) * var_t0) - (assign5160_e5850 * var_t0_dn6)) / (var_t0 * var_t0));
        var_t3_dn7 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn7) * assign5160_e5849) + (assign5160_e5845 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn7))) * var_t0) - (assign5160_e5850 * var_t0_dn7)) / (var_t0 * var_t0));
        var_t3_dn8 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn8) * assign5160_e5849) + (assign5160_e5845 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn8))) * var_t0) - (assign5160_e5850 * var_t0_dn8)) / (var_t0 * var_t0));

        let assign5170_e5855: f64 = (var_xg2 - var_phissatback2);
        var_q2 = assign5170_e5855;
        var_q2_dn3 = (var_xg2_dn3 - var_phissatback2_dn3);
        var_q2_dn4 = (var_xg2_dn4 - var_phissatback2_dn4);
        var_q2_dn5 = (var_xg2_dn5 - var_phissatback2_dn5);
        var_q2_dn6 = (var_xg2_dn6 - var_phissatback2_dn6);
        var_q2_dn7 = (var_xg2_dn7 - var_phissatback2_dn7);
        var_q2_dn8 = (var_xg2_dn8 - var_phissatback2_dn8);

        let assign5180_e5858: f64 = (var_k2 * var_k2);
        let assign5180_e5860: f64 = (assign5180_e5858 * var_q2);
        let assign5180_e5862: f64 = (assign5180_e5860 * var_q2);
        let assign5180_e5865: f64 = (var_phissatback2).exp();
        let assign5180_e5866: f64 = (var_a0 * assign5180_e5865);
        let assign5180_e5867: f64 = (assign5180_e5862 - assign5180_e5866);
        var_qsqrt = assign5180_e5867;
        var_qsqrt_dn3 = ((((assign5180_e5858 * var_q2_dn3) * var_q2) + (assign5180_e5860 * var_q2_dn3)) - ((var_a0_dn3 * assign5180_e5865) + (var_a0 * (assign5180_e5865 * var_phissatback2_dn3))));
        var_qsqrt_dn4 = ((((assign5180_e5858 * var_q2_dn4) * var_q2) + (assign5180_e5860 * var_q2_dn4)) - ((var_a0_dn4 * assign5180_e5865) + (var_a0 * (assign5180_e5865 * var_phissatback2_dn4))));
        var_qsqrt_dn5 = ((((assign5180_e5858 * var_q2_dn5) * var_q2) + (assign5180_e5860 * var_q2_dn5)) - ((var_a0_dn5 * assign5180_e5865) + (var_a0 * (assign5180_e5865 * var_phissatback2_dn5))));
        var_qsqrt_dn6 = ((((assign5180_e5858 * var_q2_dn6) * var_q2) + (assign5180_e5860 * var_q2_dn6)) - ((var_a0_dn6 * assign5180_e5865) + (var_a0 * (assign5180_e5865 * var_phissatback2_dn6))));
        var_qsqrt_dn7 = ((((assign5180_e5858 * var_q2_dn7) * var_q2) + (assign5180_e5860 * var_q2_dn7)) - ((var_a0_dn7 * assign5180_e5865) + (var_a0 * (assign5180_e5865 * var_phissatback2_dn7))));
        var_qsqrt_dn8 = ((((assign5180_e5858 * var_q2_dn8) * var_q2) + (assign5180_e5860 * var_q2_dn8)) - ((var_a0_dn8 * assign5180_e5865) + (var_a0 * (assign5180_e5865 * var_phissatback2_dn8))));

        let assign5190_e5870: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard76 = assign5190_e5870;

        let (assign5200_e5878, assign5200_e5878_d_n3, assign5200_e5878_d_n4, assign5200_e5878_d_n5, assign5200_e5878_d_n6, assign5200_e5878_d_n7, assign5200_e5878_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5200_e5874: f64 = (var_xg2 - var_phi2);
        let assign5200_e5876: f64 = (assign5200_e5874 * var_k2);
        (assign5200_e5876, ((var_xg2_dn3 - var_phi2_dn3) * var_k2), ((var_xg2_dn4 - var_phi2_dn4) * var_k2), ((var_xg2_dn5 - var_phi2_dn5) * var_k2), ((var_xg2_dn6 - var_phi2_dn6) * var_k2), ((var_xg2_dn7 - var_phi2_dn7) * var_k2), ((var_xg2_dn8 - var_phi2_dn8) * var_k2),)
    } else {
        (var_q2, var_q2_dn3, var_q2_dn4, var_q2_dn5, var_q2_dn6, var_q2_dn7, var_q2_dn8,)
    }
};
        var_q2 = assign5200_e5878;
        var_q2_dn3 = assign5200_e5878_d_n3;
        var_q2_dn4 = assign5200_e5878_d_n4;
        var_q2_dn5 = assign5200_e5878_d_n5;
        var_q2_dn6 = assign5200_e5878_d_n6;
        var_q2_dn7 = assign5200_e5878_d_n7;
        var_q2_dn8 = assign5200_e5878_d_n8;

        let (assign5210_e5884, assign5210_e5884_d_n3, assign5210_e5884_d_n4, assign5210_e5884_d_n5, assign5210_e5884_d_n6, assign5210_e5884_d_n7, assign5210_e5884_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5210_e5882: f64 = (40.0 * var_k1);
        (assign5210_e5882, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_q1, var_q1_dn3, var_q1_dn4, var_q1_dn5, var_q1_dn6, var_q1_dn7, var_q1_dn8,)
    }
};
        var_q1 = assign5210_e5884;
        var_q1_dn3 = assign5210_e5884_d_n3;
        var_q1_dn4 = assign5210_e5884_d_n4;
        var_q1_dn5 = assign5210_e5884_d_n5;
        var_q1_dn6 = assign5210_e5884_d_n6;
        var_q1_dn7 = assign5210_e5884_d_n7;
        var_q1_dn8 = assign5210_e5884_d_n8;

        let (assign5220_e5890, assign5220_e5890_d_n3, assign5220_e5890_d_n4, assign5220_e5890_d_n5, assign5220_e5890_d_n6, assign5220_e5890_d_n7, assign5220_e5890_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5220_e5888: f64 = (var_q1 + var_q2);
        (assign5220_e5888, (var_q1_dn3 + var_q2_dn3), (var_q1_dn4 + var_q2_dn4), (var_q1_dn5 + var_q2_dn5), (var_q1_dn6 + var_q2_dn6), (var_q1_dn7 + var_q2_dn7), (var_q1_dn8 + var_q2_dn8),)
    } else {
        (var_qt, var_qt_dn3, var_qt_dn4, var_qt_dn5, var_qt_dn6, var_qt_dn7, var_qt_dn8,)
    }
};
        var_qt = assign5220_e5890;
        var_qt_dn3 = assign5220_e5890_d_n3;
        var_qt_dn4 = assign5220_e5890_d_n4;
        var_qt_dn5 = assign5220_e5890_d_n5;
        var_qt_dn6 = assign5220_e5890_d_n6;
        var_qt_dn7 = assign5220_e5890_d_n7;
        var_qt_dn8 = assign5220_e5890_d_n8;

        let (assign5230_e5896, assign5230_e5896_d_n3, assign5230_e5896_d_n4, assign5230_e5896_d_n5, assign5230_e5896_d_n6, assign5230_e5896_d_n7, assign5230_e5896_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5230_e5894: f64 = (var_q1 * var_q2);
        (assign5230_e5894, ((var_q1_dn3 * var_q2) + (var_q1 * var_q2_dn3)), ((var_q1_dn4 * var_q2) + (var_q1 * var_q2_dn4)), ((var_q1_dn5 * var_q2) + (var_q1 * var_q2_dn5)), ((var_q1_dn6 * var_q2) + (var_q1 * var_q2_dn6)), ((var_q1_dn7 * var_q2) + (var_q1 * var_q2_dn7)), ((var_q1_dn8 * var_q2) + (var_q1 * var_q2_dn8)),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign5230_e5896;
        var_t3_dn3 = assign5230_e5896_d_n3;
        var_t3_dn4 = assign5230_e5896_d_n4;
        var_t3_dn5 = assign5230_e5896_d_n5;
        var_t3_dn6 = assign5230_e5896_d_n6;
        var_t3_dn7 = assign5230_e5896_d_n7;
        var_t3_dn8 = assign5230_e5896_d_n8;

        let (assign5240_e5904, assign5240_e5904_d_n3, assign5240_e5904_d_n4, assign5240_e5904_d_n5, assign5240_e5904_d_n6, assign5240_e5904_d_n7, assign5240_e5904_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5240_e5900: f64 = (0.06534 * var_qt);
        let assign5240_e5902: f64 = (assign5240_e5900 + 1.0);
        (assign5240_e5902, (0.06534 * var_qt_dn3), (0.06534 * var_qt_dn4), (0.06534 * var_qt_dn5), (0.06534 * var_qt_dn6), (0.06534 * var_qt_dn7), (0.06534 * var_qt_dn8),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign5240_e5904;
        var_t4_dn3 = assign5240_e5904_d_n3;
        var_t4_dn4 = assign5240_e5904_d_n4;
        var_t4_dn5 = assign5240_e5904_d_n5;
        var_t4_dn6 = assign5240_e5904_d_n6;
        var_t4_dn7 = assign5240_e5904_d_n7;
        var_t4_dn8 = assign5240_e5904_d_n8;

        let (assign5250_e5914, assign5250_e5914_d_n3, assign5250_e5914_d_n4, assign5250_e5914_d_n5, assign5250_e5914_d_n6, assign5250_e5914_d_n7, assign5250_e5914_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5250_e5908: f64 = (var_qt * 8.57973);
        let assign5250_e5910: f64 = (assign5250_e5908 + var_t3);
        let assign5250_e5912: f64 = (assign5250_e5910 + 39.47841);
        (assign5250_e5912, ((var_qt_dn3 * 8.57973) + var_t3_dn3), ((var_qt_dn4 * 8.57973) + var_t3_dn4), ((var_qt_dn5 * 8.57973) + var_t3_dn5), ((var_qt_dn6 * 8.57973) + var_t3_dn6), ((var_qt_dn7 * 8.57973) + var_t3_dn7), ((var_qt_dn8 * 8.57973) + var_t3_dn8),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign5250_e5914;
        var_t5_dn3 = assign5250_e5914_d_n3;
        var_t5_dn4 = assign5250_e5914_d_n4;
        var_t5_dn5 = assign5250_e5914_d_n5;
        var_t5_dn6 = assign5250_e5914_d_n6;
        var_t5_dn7 = assign5250_e5914_d_n7;
        var_t5_dn8 = assign5250_e5914_d_n8;

        let (assign5260_e5924, assign5260_e5924_d_n3, assign5260_e5924_d_n4, assign5260_e5924_d_n5, assign5260_e5924_d_n6, assign5260_e5924_d_n7, assign5260_e5924_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5260_e5918: f64 = (78.95683 * var_qt);
        let assign5260_e5921: f64 = (39.47841 * var_t3);
        let assign5260_e5922: f64 = (assign5260_e5918 + assign5260_e5921);
        (assign5260_e5922, ((78.95683 * var_qt_dn3) + (39.47841 * var_t3_dn3)), ((78.95683 * var_qt_dn4) + (39.47841 * var_t3_dn4)), ((78.95683 * var_qt_dn5) + (39.47841 * var_t3_dn5)), ((78.95683 * var_qt_dn6) + (39.47841 * var_t3_dn6)), ((78.95683 * var_qt_dn7) + (39.47841 * var_t3_dn7)), ((78.95683 * var_qt_dn8) + (39.47841 * var_t3_dn8)),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign5260_e5924;
        var_t6_dn3 = assign5260_e5924_d_n3;
        var_t6_dn4 = assign5260_e5924_d_n4;
        var_t6_dn5 = assign5260_e5924_d_n5;
        var_t6_dn6 = assign5260_e5924_d_n6;
        var_t6_dn7 = assign5260_e5924_d_n7;
        var_t6_dn8 = assign5260_e5924_d_n8;

        let (assign5270_e5945, assign5270_e5945_d_n3, assign5270_e5945_d_n4, assign5270_e5945_d_n5, assign5270_e5945_d_n6, assign5270_e5945_d_n7, assign5270_e5945_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5270_e5927: f64 = (-var_t5);
        let assign5270_e5929: f64 = (-4.0);
        let assign5270_e5931: f64 = (assign5270_e5929 * var_t4);
        let assign5270_e5933: f64 = (assign5270_e5931 * var_t6);
        let assign5270_e5936: f64 = (var_t5 * var_t5);
        let assign5270_e5937: f64 = (assign5270_e5933 + assign5270_e5936);
        let assign5270_e5938: f64 = (assign5270_e5937).sqrt();
        let assign5270_e5939: f64 = (assign5270_e5927 + assign5270_e5938);
        let assign5270_e5942: f64 = (2.0 * var_t4);
        let assign5270_e5943: f64 = (assign5270_e5939 / assign5270_e5942);
        (assign5270_e5943, (((((-var_t5_dn3) + (((((assign5270_e5929 * var_t4_dn3) * var_t6) + (assign5270_e5931 * var_t6_dn3)) + ((var_t5_dn3 * var_t5) + (var_t5 * var_t5_dn3))) / (2.0 * assign5270_e5938))) * assign5270_e5942) - (assign5270_e5939 * (2.0 * var_t4_dn3))) / (assign5270_e5942 * assign5270_e5942)), (((((-var_t5_dn4) + (((((assign5270_e5929 * var_t4_dn4) * var_t6) + (assign5270_e5931 * var_t6_dn4)) + ((var_t5_dn4 * var_t5) + (var_t5 * var_t5_dn4))) / (2.0 * assign5270_e5938))) * assign5270_e5942) - (assign5270_e5939 * (2.0 * var_t4_dn4))) / (assign5270_e5942 * assign5270_e5942)), (((((-var_t5_dn5) + (((((assign5270_e5929 * var_t4_dn5) * var_t6) + (assign5270_e5931 * var_t6_dn5)) + ((var_t5_dn5 * var_t5) + (var_t5 * var_t5_dn5))) / (2.0 * assign5270_e5938))) * assign5270_e5942) - (assign5270_e5939 * (2.0 * var_t4_dn5))) / (assign5270_e5942 * assign5270_e5942)), (((((-var_t5_dn6) + (((((assign5270_e5929 * var_t4_dn6) * var_t6) + (assign5270_e5931 * var_t6_dn6)) + ((var_t5_dn6 * var_t5) + (var_t5 * var_t5_dn6))) / (2.0 * assign5270_e5938))) * assign5270_e5942) - (assign5270_e5939 * (2.0 * var_t4_dn6))) / (assign5270_e5942 * assign5270_e5942)), (((((-var_t5_dn7) + (((((assign5270_e5929 * var_t4_dn7) * var_t6) + (assign5270_e5931 * var_t6_dn7)) + ((var_t5_dn7 * var_t5) + (var_t5 * var_t5_dn7))) / (2.0 * assign5270_e5938))) * assign5270_e5942) - (assign5270_e5939 * (2.0 * var_t4_dn7))) / (assign5270_e5942 * assign5270_e5942)), (((((-var_t5_dn8) + (((((assign5270_e5929 * var_t4_dn8) * var_t6) + (assign5270_e5931 * var_t6_dn8)) + ((var_t5_dn8 * var_t5) + (var_t5 * var_t5_dn8))) / (2.0 * assign5270_e5938))) * assign5270_e5942) - (assign5270_e5939 * (2.0 * var_t4_dn8))) / (assign5270_e5942 * assign5270_e5942)),)
    } else {
        (var_qsqrt, var_qsqrt_dn3, var_qsqrt_dn4, var_qsqrt_dn5, var_qsqrt_dn6, var_qsqrt_dn7, var_qsqrt_dn8,)
    }
};
        var_qsqrt = assign5270_e5945;
        var_qsqrt_dn3 = assign5270_e5945_d_n3;
        var_qsqrt_dn4 = assign5270_e5945_d_n4;
        var_qsqrt_dn5 = assign5270_e5945_d_n5;
        var_qsqrt_dn6 = assign5270_e5945_d_n6;
        var_qsqrt_dn7 = assign5270_e5945_d_n7;
        var_qsqrt_dn8 = assign5270_e5945_d_n8;

        let (assign5280_e5957, assign5280_e5957_d_n3, assign5280_e5957_d_n4, assign5280_e5957_d_n5, assign5280_e5957_d_n6, assign5280_e5957_d_n7, assign5280_e5957_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5280_e5950: f64 = (1.0 + var_k1);
        let assign5280_e5951: f64 = (var_phi1_0 * assign5280_e5950);
        let assign5280_e5953: f64 = (assign5280_e5951 - var_phi2);
        let assign5280_e5955: f64 = (assign5280_e5953 / var_k1);
        (assign5280_e5955, (((var_phi1_0_dn3 * assign5280_e5950) - var_phi2_dn3) / var_k1), (((var_phi1_0_dn4 * assign5280_e5950) - var_phi2_dn4) / var_k1), (((var_phi1_0_dn5 * assign5280_e5950) - var_phi2_dn5) / var_k1), (((var_phi1_0_dn6 * assign5280_e5950) - var_phi2_dn6) / var_k1), (((var_phi1_0_dn7 * assign5280_e5950) - var_phi2_dn7) / var_k1), (((var_phi1_0_dn8 * assign5280_e5950) - var_phi2_dn8) / var_k1),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign5280_e5957;
        var_t3_dn3 = assign5280_e5957_d_n3;
        var_t3_dn4 = assign5280_e5957_d_n4;
        var_t3_dn5 = assign5280_e5957_d_n5;
        var_t3_dn6 = assign5280_e5957_d_n6;
        var_t3_dn7 = assign5280_e5957_d_n7;
        var_t3_dn8 = assign5280_e5957_d_n8;

        let (assign5290_e5969, assign5290_e5969_d_n3, assign5290_e5969_d_n4, assign5290_e5969_d_n5, assign5290_e5969_d_n6, assign5290_e5969_d_n7, assign5290_e5969_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5290_e5962: f64 = (var_xg1 - var_t3);
        let assign5290_e5964: f64 = (assign5290_e5962 + 2.0);
        let assign5290_e5965: f64 = (40.0 * assign5290_e5964);
        let assign5290_e5967: f64 = (assign5290_e5965 / 5.0);
        (assign5290_e5967, ((40.0 * (var_xg1_dn3 - var_t3_dn3)) / 5.0), ((40.0 * (var_xg1_dn4 - var_t3_dn4)) / 5.0), ((40.0 * (var_xg1_dn5 - var_t3_dn5)) / 5.0), ((40.0 * (var_xg1_dn6 - var_t3_dn6)) / 5.0), ((40.0 * (var_xg1_dn7 - var_t3_dn7)) / 5.0), ((40.0 * (var_xg1_dn8 - var_t3_dn8)) / 5.0),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign5290_e5969;
        var_t0_dn3 = assign5290_e5969_d_n3;
        var_t0_dn4 = assign5290_e5969_d_n4;
        var_t0_dn5 = assign5290_e5969_d_n5;
        var_t0_dn6 = assign5290_e5969_d_n6;
        var_t0_dn7 = assign5290_e5969_d_n7;
        var_t0_dn8 = assign5290_e5969_d_n8;

        let (assign5300_e5987, assign5300_e5987_d_n3, assign5300_e5987_d_n4, assign5300_e5987_d_n5, assign5300_e5987_d_n6, assign5300_e5987_d_n7, assign5300_e5987_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5300_e5975: f64 = (var_xg1 - var_t3);
        let assign5300_e5977: f64 = (assign5300_e5975 + 2.0);
        let assign5300_e5978: f64 = (-assign5300_e5977);
        let assign5300_e5981: f64 = (2.0 / 0.69);
        let assign5300_e5982: f64 = (assign5300_e5978 / assign5300_e5981);
        let assign5300_e5983: f64 = (assign5300_e5982).exp();
        let assign5300_e5984: f64 = (1.0 - assign5300_e5983);
        let assign5300_e5985: f64 = (var_qsqrt * assign5300_e5984);
        (assign5300_e5985, ((var_qsqrt_dn3 * assign5300_e5984) + (var_qsqrt * (-(assign5300_e5983 * ((-(var_xg1_dn3 - var_t3_dn3)) / assign5300_e5981))))), ((var_qsqrt_dn4 * assign5300_e5984) + (var_qsqrt * (-(assign5300_e5983 * ((-(var_xg1_dn4 - var_t3_dn4)) / assign5300_e5981))))), ((var_qsqrt_dn5 * assign5300_e5984) + (var_qsqrt * (-(assign5300_e5983 * ((-(var_xg1_dn5 - var_t3_dn5)) / assign5300_e5981))))), ((var_qsqrt_dn6 * assign5300_e5984) + (var_qsqrt * (-(assign5300_e5983 * ((-(var_xg1_dn6 - var_t3_dn6)) / assign5300_e5981))))), ((var_qsqrt_dn7 * assign5300_e5984) + (var_qsqrt * (-(assign5300_e5983 * ((-(var_xg1_dn7 - var_t3_dn7)) / assign5300_e5981))))), ((var_qsqrt_dn8 * assign5300_e5984) + (var_qsqrt * (-(assign5300_e5983 * ((-(var_xg1_dn8 - var_t3_dn8)) / assign5300_e5981))))),)
    } else {
        (var_qsqrt, var_qsqrt_dn3, var_qsqrt_dn4, var_qsqrt_dn5, var_qsqrt_dn6, var_qsqrt_dn7, var_qsqrt_dn8,)
    }
};
        var_qsqrt = assign5300_e5987;
        var_qsqrt_dn3 = assign5300_e5987_d_n3;
        var_qsqrt_dn4 = assign5300_e5987_d_n4;
        var_qsqrt_dn5 = assign5300_e5987_d_n5;
        var_qsqrt_dn6 = assign5300_e5987_d_n6;
        var_qsqrt_dn7 = assign5300_e5987_d_n7;
        var_qsqrt_dn8 = assign5300_e5987_d_n8;

        *var_a0_slot = var_a0;
        *var_a0_dn3_slot = var_a0_dn3;
        *var_a0_dn4_slot = var_a0_dn4;
        *var_a0_dn5_slot = var_a0_dn5;
        *var_a0_dn6_slot = var_a0_dn6;
        *var_a0_dn7_slot = var_a0_dn7;
        *var_a0_dn8_slot = var_a0_dn8;
        *var_dvth_all_slot = var_dvth_all;
        *var_dvth_all_dn3_slot = var_dvth_all_dn3;
        *var_dvth_all_dn4_slot = var_dvth_all_dn4;
        *var_dvth_all_dn5_slot = var_dvth_all_dn5;
        *var_dvth_all_dn6_slot = var_dvth_all_dn6;
        *var_dvth_all_dn7_slot = var_dvth_all_dn7;
        *var_dvth_all_dn8_slot = var_dvth_all_dn8;
        *var_dvth_temp_slot = var_dvth_temp;
        *var_dvth_temp_dn3_slot = var_dvth_temp_dn3;
        *var_dvth_temp_dn4_slot = var_dvth_temp_dn4;
        *var_dvth_temp_dn5_slot = var_dvth_temp_dn5;
        *var_dvth_temp_dn6_slot = var_dvth_temp_dn6;
        *var_dvth_temp_dn7_slot = var_dvth_temp_dn7;
        *var_dvth_temp_dn8_slot = var_dvth_temp_dn8;
        *var_guard76_slot = var_guard76;
        *var_k1_slot = var_k1;
        *var_k1_2_slot = var_k1_2;
        *var_k2_slot = var_k2;
        *var_keq_k2_slot = var_keq_k2;
        *var_lna0_slot = var_lna0;
        *var_lna0_dn3_slot = var_lna0_dn3;
        *var_lna0_dn4_slot = var_lna0_dn4;
        *var_lna0_dn5_slot = var_lna0_dn5;
        *var_lna0_dn6_slot = var_lna0_dn6;
        *var_lna0_dn7_slot = var_lna0_dn7;
        *var_lna0_dn8_slot = var_lna0_dn8;
        *var_phi1_slot = var_phi1;
        *var_phi1_0_slot = var_phi1_0;
        *var_phi1_0_dn3_slot = var_phi1_0_dn3;
        *var_phi1_0_dn4_slot = var_phi1_0_dn4;
        *var_phi1_0_dn5_slot = var_phi1_0_dn5;
        *var_phi1_0_dn6_slot = var_phi1_0_dn6;
        *var_phi1_0_dn7_slot = var_phi1_0_dn7;
        *var_phi1_0_dn8_slot = var_phi1_0_dn8;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_phi2_slot = var_phi2;
        *var_phi2_dn3_slot = var_phi2_dn3;
        *var_phi2_dn4_slot = var_phi2_dn4;
        *var_phi2_dn5_slot = var_phi2_dn5;
        *var_phi2_dn6_slot = var_phi2_dn6;
        *var_phi2_dn7_slot = var_phi2_dn7;
        *var_phi2_dn8_slot = var_phi2_dn8;
        *var_phi2sub_slot = var_phi2sub;
        *var_phi2sub_dn3_slot = var_phi2sub_dn3;
        *var_phi2sub_dn4_slot = var_phi2sub_dn4;
        *var_phi2sub_dn5_slot = var_phi2sub_dn5;
        *var_phi2sub_dn6_slot = var_phi2sub_dn6;
        *var_phi2sub_dn7_slot = var_phi2sub_dn7;
        *var_phi2sub_dn8_slot = var_phi2sub_dn8;
        *var_phissat_slot = var_phissat;
        *var_phissat_dn3_slot = var_phissat_dn3;
        *var_phissat_dn4_slot = var_phissat_dn4;
        *var_phissat_dn5_slot = var_phissat_dn5;
        *var_phissat_dn6_slot = var_phissat_dn6;
        *var_phissat_dn7_slot = var_phissat_dn7;
        *var_phissat_dn8_slot = var_phissat_dn8;
        *var_phissatback_slot = var_phissatback;
        *var_phissatback2_slot = var_phissatback2;
        *var_phissatback2_dn3_slot = var_phissatback2_dn3;
        *var_phissatback2_dn4_slot = var_phissatback2_dn4;
        *var_phissatback2_dn5_slot = var_phissatback2_dn5;
        *var_phissatback2_dn6_slot = var_phissatback2_dn6;
        *var_phissatback2_dn7_slot = var_phissatback2_dn7;
        *var_phissatback2_dn8_slot = var_phissatback2_dn8;
        *var_phissatback_dn3_slot = var_phissatback_dn3;
        *var_phissatback_dn4_slot = var_phissatback_dn4;
        *var_phissatback_dn5_slot = var_phissatback_dn5;
        *var_phissatback_dn6_slot = var_phissatback_dn6;
        *var_phissatback_dn7_slot = var_phissatback_dn7;
        *var_phissatback_dn8_slot = var_phissatback_dn8;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_qcoth1_slot = var_qcoth1;
        *var_qcoth1_dn3_slot = var_qcoth1_dn3;
        *var_qcoth1_dn4_slot = var_qcoth1_dn4;
        *var_qcoth1_dn5_slot = var_qcoth1_dn5;
        *var_qcoth1_dn6_slot = var_qcoth1_dn6;
        *var_qcoth1_dn7_slot = var_qcoth1_dn7;
        *var_qcoth1_dn8_slot = var_qcoth1_dn8;
        *var_qsq1_slot = var_qsq1;
        *var_qsq1_dn3_slot = var_qsq1_dn3;
        *var_qsq1_dn4_slot = var_qsq1_dn4;
        *var_qsq1_dn5_slot = var_qsq1_dn5;
        *var_qsq1_dn6_slot = var_qsq1_dn6;
        *var_qsq1_dn7_slot = var_qsq1_dn7;
        *var_qsq1_dn8_slot = var_qsq1_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt1_slot = var_qsqrt1;
        *var_qsqrt1_dn3_slot = var_qsqrt1_dn3;
        *var_qsqrt1_dn4_slot = var_qsqrt1_dn4;
        *var_qsqrt1_dn5_slot = var_qsqrt1_dn5;
        *var_qsqrt1_dn6_slot = var_qsqrt1_dn6;
        *var_qsqrt1_dn7_slot = var_qsqrt1_dn7;
        *var_qsqrt1_dn8_slot = var_qsqrt1_dn8;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_qt_slot = var_qt;
        *var_qt_dn3_slot = var_qt_dn3;
        *var_qt_dn4_slot = var_qt_dn4;
        *var_qt_dn5_slot = var_qt_dn5;
        *var_qt_dn6_slot = var_qt_dn6;
        *var_qt_dn7_slot = var_qt_dn7;
        *var_qt_dn8_slot = var_qt_dn8;
        *var_qth_slot = var_qth;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_vgfb1eff_slot = var_vgfb1eff;
        *var_vgfb1eff_dn3_slot = var_vgfb1eff_dn3;
        *var_vgfb1eff_dn4_slot = var_vgfb1eff_dn4;
        *var_vgfb1eff_dn5_slot = var_vgfb1eff_dn5;
        *var_vgfb1eff_dn6_slot = var_vgfb1eff_dn6;
        *var_vgfb1eff_dn7_slot = var_vgfb1eff_dn7;
        *var_vgfb1eff_dn8_slot = var_vgfb1eff_dn8;
        *var_xg1_slot = var_xg1;
        *var_xg1_dn3_slot = var_xg1_dn3;
        *var_xg1_dn4_slot = var_xg1_dn4;
        *var_xg1_dn5_slot = var_xg1_dn5;
        *var_xg1_dn6_slot = var_xg1_dn6;
        *var_xg1_dn7_slot = var_xg1_dn7;
        *var_xg1_dn8_slot = var_xg1_dn8;
        *var_xg2_slot = var_xg2;
        *var_xg2_dn3_slot = var_xg2_dn3;
        *var_xg2_dn4_slot = var_xg2_dn4;
        *var_xg2_dn5_slot = var_xg2_dn5;
        *var_xg2_dn6_slot = var_xg2_dn6;
        *var_xg2_dn7_slot = var_xg2_dn7;
        *var_xg2_dn8_slot = var_xg2_dn8;
    }

    pub(super) fn stamp_transient_block_9(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_guard76: f64,
        var_k1: f64,
        var_k1_2: f64,
        var_lna0: f64,
        var_lna0_dn3: f64,
        var_lna0_dn4: f64,
        var_lna0_dn5: f64,
        var_lna0_dn6: f64,
        var_lna0_dn7: f64,
        var_lna0_dn8: f64,
        var_nvtm: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_phi1_0: f64,
        var_phi1_0_dn3: f64,
        var_phi1_0_dn4: f64,
        var_phi1_0_dn5: f64,
        var_phi1_0_dn6: f64,
        var_phi1_0_dn7: f64,
        var_phi1_0_dn8: f64,
        var_phi2: f64,
        var_phi2_dn3: f64,
        var_phi2_dn4: f64,
        var_phi2_dn5: f64,
        var_phi2_dn6: f64,
        var_phi2_dn7: f64,
        var_phi2_dn8: f64,
        var_vgfb1eff: f64,
        var_vgfb1eff_dn3: f64,
        var_vgfb1eff_dn4: f64,
        var_vgfb1eff_dn5: f64,
        var_vgfb1eff_dn6: f64,
        var_vgfb1eff_dn7: f64,
        var_vgfb1eff_dn8: f64,
        var_aaux_slot: &mut f64,
        var_aaux_dn3_slot: &mut f64,
        var_aaux_dn4_slot: &mut f64,
        var_aaux_dn5_slot: &mut f64,
        var_aaux_dn6_slot: &mut f64,
        var_aaux_dn7_slot: &mut f64,
        var_aaux_dn8_slot: &mut f64,
        var_auxb1_slot: &mut f64,
        var_auxb1_dn3_slot: &mut f64,
        var_auxb1_dn4_slot: &mut f64,
        var_auxb1_dn5_slot: &mut f64,
        var_auxb1_dn6_slot: &mut f64,
        var_auxb1_dn7_slot: &mut f64,
        var_auxb1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_dg1_slot: &mut f64,
        var_dg1_dn3_slot: &mut f64,
        var_dg1_dn4_slot: &mut f64,
        var_dg1_dn5_slot: &mut f64,
        var_dg1_dn6_slot: &mut f64,
        var_dg1_dn7_slot: &mut f64,
        var_dg1_dn8_slot: &mut f64,
        var_dg2_slot: &mut f64,
        var_dg2_dn3_slot: &mut f64,
        var_dg2_dn4_slot: &mut f64,
        var_dg2_dn5_slot: &mut f64,
        var_dg2_dn6_slot: &mut f64,
        var_dg2_dn7_slot: &mut f64,
        var_dg2_dn8_slot: &mut f64,
        var_g_slot: &mut f64,
        var_g_dn3_slot: &mut f64,
        var_g_dn4_slot: &mut f64,
        var_g_dn5_slot: &mut f64,
        var_g_dn6_slot: &mut f64,
        var_g_dn7_slot: &mut f64,
        var_g_dn8_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_phissat_slot: &mut f64,
        var_phissat_dn3_slot: &mut f64,
        var_phissat_dn4_slot: &mut f64,
        var_phissat_dn5_slot: &mut f64,
        var_phissat_dn6_slot: &mut f64,
        var_phissat_dn7_slot: &mut f64,
        var_phissat_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_dn3_slot: &mut f64,
        var_xg1_dn4_slot: &mut f64,
        var_xg1_dn5_slot: &mut f64,
        var_xg1_dn6_slot: &mut f64,
        var_xg1_dn7_slot: &mut f64,
        var_xg1_dn8_slot: &mut f64,
    ) {
        let mut var_aaux: f64 = *var_aaux_slot;
        let mut var_aaux_dn3: f64 = *var_aaux_dn3_slot;
        let mut var_aaux_dn4: f64 = *var_aaux_dn4_slot;
        let mut var_aaux_dn5: f64 = *var_aaux_dn5_slot;
        let mut var_aaux_dn6: f64 = *var_aaux_dn6_slot;
        let mut var_aaux_dn7: f64 = *var_aaux_dn7_slot;
        let mut var_aaux_dn8: f64 = *var_aaux_dn8_slot;
        let mut var_auxb1: f64 = *var_auxb1_slot;
        let mut var_auxb1_dn3: f64 = *var_auxb1_dn3_slot;
        let mut var_auxb1_dn4: f64 = *var_auxb1_dn4_slot;
        let mut var_auxb1_dn5: f64 = *var_auxb1_dn5_slot;
        let mut var_auxb1_dn6: f64 = *var_auxb1_dn6_slot;
        let mut var_auxb1_dn7: f64 = *var_auxb1_dn7_slot;
        let mut var_auxb1_dn8: f64 = *var_auxb1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_dg1: f64 = *var_dg1_slot;
        let mut var_dg1_dn3: f64 = *var_dg1_dn3_slot;
        let mut var_dg1_dn4: f64 = *var_dg1_dn4_slot;
        let mut var_dg1_dn5: f64 = *var_dg1_dn5_slot;
        let mut var_dg1_dn6: f64 = *var_dg1_dn6_slot;
        let mut var_dg1_dn7: f64 = *var_dg1_dn7_slot;
        let mut var_dg1_dn8: f64 = *var_dg1_dn8_slot;
        let mut var_dg2: f64 = *var_dg2_slot;
        let mut var_dg2_dn3: f64 = *var_dg2_dn3_slot;
        let mut var_dg2_dn4: f64 = *var_dg2_dn4_slot;
        let mut var_dg2_dn5: f64 = *var_dg2_dn5_slot;
        let mut var_dg2_dn6: f64 = *var_dg2_dn6_slot;
        let mut var_dg2_dn7: f64 = *var_dg2_dn7_slot;
        let mut var_dg2_dn8: f64 = *var_dg2_dn8_slot;
        let mut var_g: f64 = *var_g_slot;
        let mut var_g_dn3: f64 = *var_g_dn3_slot;
        let mut var_g_dn4: f64 = *var_g_dn4_slot;
        let mut var_g_dn5: f64 = *var_g_dn5_slot;
        let mut var_g_dn6: f64 = *var_g_dn6_slot;
        let mut var_g_dn7: f64 = *var_g_dn7_slot;
        let mut var_g_dn8: f64 = *var_g_dn8_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_phissat: f64 = *var_phissat_slot;
        let mut var_phissat_dn3: f64 = *var_phissat_dn3_slot;
        let mut var_phissat_dn4: f64 = *var_phissat_dn4_slot;
        let mut var_phissat_dn5: f64 = *var_phissat_dn5_slot;
        let mut var_phissat_dn6: f64 = *var_phissat_dn6_slot;
        let mut var_phissat_dn7: f64 = *var_phissat_dn7_slot;
        let mut var_phissat_dn8: f64 = *var_phissat_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_dn3: f64 = *var_xg1_dn3_slot;
        let mut var_xg1_dn4: f64 = *var_xg1_dn4_slot;
        let mut var_xg1_dn5: f64 = *var_xg1_dn5_slot;
        let mut var_xg1_dn6: f64 = *var_xg1_dn6_slot;
        let mut var_xg1_dn7: f64 = *var_xg1_dn7_slot;
        let mut var_xg1_dn8: f64 = *var_xg1_dn8_slot;

        let (assign5310_e5993, assign5310_e5993_d_n3, assign5310_e5993_d_n4, assign5310_e5993_d_n5, assign5310_e5993_d_n6, assign5310_e5993_d_n7, assign5310_e5993_d_n8,) = {
    if (var_guard76 != 0.0) {
        let assign5310_e5991: f64 = (var_qsqrt).min(50.0);
        (assign5310_e5991, if var_qsqrt <= 50.0 { var_qsqrt_dn3 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn4 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn5 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn6 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn7 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn8 } else { 0.0 },)
    } else {
        (var_qsqrt, var_qsqrt_dn3, var_qsqrt_dn4, var_qsqrt_dn5, var_qsqrt_dn6, var_qsqrt_dn7, var_qsqrt_dn8,)
    }
};
        var_qsqrt = assign5310_e5993;
        var_qsqrt_dn3 = assign5310_e5993_d_n3;
        var_qsqrt_dn4 = assign5310_e5993_d_n4;
        var_qsqrt_dn5 = assign5310_e5993_d_n5;
        var_qsqrt_dn6 = assign5310_e5993_d_n6;
        var_qsqrt_dn7 = assign5310_e5993_d_n7;
        var_qsqrt_dn8 = assign5310_e5993_d_n8;

        let assign5320_e5996: f64 = (var_xg1).max(var_phi1_0);
        var_xg1 = assign5320_e5996;
        var_xg1_dn3 = if var_xg1 >= var_phi1_0 { var_xg1_dn3 } else { var_phi1_0_dn3 };
        var_xg1_dn4 = if var_xg1 >= var_phi1_0 { var_xg1_dn4 } else { var_phi1_0_dn4 };
        var_xg1_dn5 = if var_xg1 >= var_phi1_0 { var_xg1_dn5 } else { var_phi1_0_dn5 };
        var_xg1_dn6 = if var_xg1 >= var_phi1_0 { var_xg1_dn6 } else { var_phi1_0_dn6 };
        var_xg1_dn7 = if var_xg1 >= var_phi1_0 { var_xg1_dn7 } else { var_phi1_0_dn7 };
        var_xg1_dn8 = if var_xg1 >= var_phi1_0 { var_xg1_dn8 } else { var_phi1_0_dn8 };

        let assign5330_e6000: f64 = (var_xg1 - var_phi1_0);
        let assign5330_e6001: f64 = (var_k1_2 * assign5330_e6000);
        let assign5330_e6004: f64 = (var_xg1 - var_phi1_0);
        let assign5330_e6005: f64 = (assign5330_e6001 * assign5330_e6004);
        let assign5330_e6007: f64 = (assign5330_e6005 + 39.47841);
        let assign5330_e6008: f64 = (assign5330_e6007).ln();
        let assign5330_e6010: f64 = (assign5330_e6008 - var_lna0);
        var_phissat = assign5330_e6010;
        var_phissat_dn3 = (((((var_k1_2 * (var_xg1_dn3 - var_phi1_0_dn3)) * assign5330_e6004) + (assign5330_e6001 * (var_xg1_dn3 - var_phi1_0_dn3))) / assign5330_e6007) - var_lna0_dn3);
        var_phissat_dn4 = (((((var_k1_2 * (var_xg1_dn4 - var_phi1_0_dn4)) * assign5330_e6004) + (assign5330_e6001 * (var_xg1_dn4 - var_phi1_0_dn4))) / assign5330_e6007) - var_lna0_dn4);
        var_phissat_dn5 = (((((var_k1_2 * (var_xg1_dn5 - var_phi1_0_dn5)) * assign5330_e6004) + (assign5330_e6001 * (var_xg1_dn5 - var_phi1_0_dn5))) / assign5330_e6007) - var_lna0_dn5);
        var_phissat_dn6 = (((((var_k1_2 * (var_xg1_dn6 - var_phi1_0_dn6)) * assign5330_e6004) + (assign5330_e6001 * (var_xg1_dn6 - var_phi1_0_dn6))) / assign5330_e6007) - var_lna0_dn6);
        var_phissat_dn7 = (((((var_k1_2 * (var_xg1_dn7 - var_phi1_0_dn7)) * assign5330_e6004) + (assign5330_e6001 * (var_xg1_dn7 - var_phi1_0_dn7))) / assign5330_e6007) - var_lna0_dn7);
        var_phissat_dn8 = (((((var_k1_2 * (var_xg1_dn8 - var_phi1_0_dn8)) * assign5330_e6004) + (assign5330_e6001 * (var_xg1_dn8 - var_phi1_0_dn8))) / assign5330_e6007) - var_lna0_dn8);

        let assign5340_e6014: f64 = (1.0 + var_k1);
        let assign5340_e6015: f64 = (var_phi1_0 * assign5340_e6014);
        let assign5340_e6017: f64 = (assign5340_e6015 - var_phi2);
        let assign5340_e6019: f64 = (assign5340_e6017 / var_k1);
        var_t3 = assign5340_e6019;
        var_t3_dn3 = (((var_phi1_0_dn3 * assign5340_e6014) - var_phi2_dn3) / var_k1);
        var_t3_dn4 = (((var_phi1_0_dn4 * assign5340_e6014) - var_phi2_dn4) / var_k1);
        var_t3_dn5 = (((var_phi1_0_dn5 * assign5340_e6014) - var_phi2_dn5) / var_k1);
        var_t3_dn6 = (((var_phi1_0_dn6 * assign5340_e6014) - var_phi2_dn6) / var_k1);
        var_t3_dn7 = (((var_phi1_0_dn7 * assign5340_e6014) - var_phi2_dn7) / var_k1);
        var_t3_dn8 = (((var_phi1_0_dn8 * assign5340_e6014) - var_phi2_dn8) / var_k1);

        let assign5350_e6023: f64 = (var_t3 - var_phi1_0);
        let assign5350_e6024: f64 = (var_k1_2 * assign5350_e6023);
        let assign5350_e6027: f64 = (var_t3 - var_phi1_0);
        let assign5350_e6028: f64 = (assign5350_e6024 * assign5350_e6027);
        let assign5350_e6030: f64 = (assign5350_e6028 + 39.47841);
        let assign5350_e6031: f64 = (assign5350_e6030).ln();
        let assign5350_e6033: f64 = (assign5350_e6031 - var_lna0);
        var_t4 = assign5350_e6033;
        var_t4_dn3 = (((((var_k1_2 * (var_t3_dn3 - var_phi1_0_dn3)) * assign5350_e6027) + (assign5350_e6024 * (var_t3_dn3 - var_phi1_0_dn3))) / assign5350_e6030) - var_lna0_dn3);
        var_t4_dn4 = (((((var_k1_2 * (var_t3_dn4 - var_phi1_0_dn4)) * assign5350_e6027) + (assign5350_e6024 * (var_t3_dn4 - var_phi1_0_dn4))) / assign5350_e6030) - var_lna0_dn4);
        var_t4_dn5 = (((((var_k1_2 * (var_t3_dn5 - var_phi1_0_dn5)) * assign5350_e6027) + (assign5350_e6024 * (var_t3_dn5 - var_phi1_0_dn5))) / assign5350_e6030) - var_lna0_dn5);
        var_t4_dn6 = (((((var_k1_2 * (var_t3_dn6 - var_phi1_0_dn6)) * assign5350_e6027) + (assign5350_e6024 * (var_t3_dn6 - var_phi1_0_dn6))) / assign5350_e6030) - var_lna0_dn6);
        var_t4_dn7 = (((((var_k1_2 * (var_t3_dn7 - var_phi1_0_dn7)) * assign5350_e6027) + (assign5350_e6024 * (var_t3_dn7 - var_phi1_0_dn7))) / assign5350_e6030) - var_lna0_dn7);
        var_t4_dn8 = (((((var_k1_2 * (var_t3_dn8 - var_phi1_0_dn8)) * assign5350_e6027) + (assign5350_e6024 * (var_t3_dn8 - var_phi1_0_dn8))) / assign5350_e6030) - var_lna0_dn8);

        let assign5360_e6036: f64 = (var_t4 - var_phi1_0);
        var_t5 = assign5360_e6036;
        var_t5_dn3 = (var_t4_dn3 - var_phi1_0_dn3);
        var_t5_dn4 = (var_t4_dn4 - var_phi1_0_dn4);
        var_t5_dn5 = (var_t4_dn5 - var_phi1_0_dn5);
        var_t5_dn6 = (var_t4_dn6 - var_phi1_0_dn6);
        var_t5_dn7 = (var_t4_dn7 - var_phi1_0_dn7);
        var_t5_dn8 = (var_t4_dn8 - var_phi1_0_dn8);

        let assign5370_e6039: f64 = (var_phissat - var_t5);
        var_phissat = assign5370_e6039;
        var_phissat_dn3 = (var_phissat_dn3 - var_t5_dn3);
        var_phissat_dn4 = (var_phissat_dn4 - var_t5_dn4);
        var_phissat_dn5 = (var_phissat_dn5 - var_t5_dn5);
        var_phissat_dn6 = (var_phissat_dn6 - var_t5_dn6);
        var_phissat_dn7 = (var_phissat_dn7 - var_t5_dn7);
        var_phissat_dn8 = (var_phissat_dn8 - var_t5_dn8);

        let assign5380_e6042: f64 = (var_xg1 - var_phissat);
        var_q1 = assign5380_e6042;
        var_q1_dn3 = (var_xg1_dn3 - var_phissat_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phissat_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phissat_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phissat_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phissat_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phissat_dn8);

        let assign5390_e6044: f64 = (-var_a0);
        let assign5390_e6046: f64 = (var_phissat).exp();
        let assign5390_e6047: f64 = (assign5390_e6044 * assign5390_e6046);
        var_t0 = assign5390_e6047;
        var_t0_dn3 = (((-var_a0_dn3) * assign5390_e6046) + (assign5390_e6044 * (assign5390_e6046 * var_phissat_dn3)));
        var_t0_dn4 = (((-var_a0_dn4) * assign5390_e6046) + (assign5390_e6044 * (assign5390_e6046 * var_phissat_dn4)));
        var_t0_dn5 = (((-var_a0_dn5) * assign5390_e6046) + (assign5390_e6044 * (assign5390_e6046 * var_phissat_dn5)));
        var_t0_dn6 = (((-var_a0_dn6) * assign5390_e6046) + (assign5390_e6044 * (assign5390_e6046 * var_phissat_dn6)));
        var_t0_dn7 = (((-var_a0_dn7) * assign5390_e6046) + (assign5390_e6044 * (assign5390_e6046 * var_phissat_dn7)));
        var_t0_dn8 = (((-var_a0_dn8) * assign5390_e6046) + (assign5390_e6044 * (assign5390_e6046 * var_phissat_dn8)));

        let assign5400_e6050: f64 = (var_k1_2 * var_q1);
        var_t1 = assign5400_e6050;
        var_t1_dn3 = (var_k1_2 * var_q1_dn3);
        var_t1_dn4 = (var_k1_2 * var_q1_dn4);
        var_t1_dn5 = (var_k1_2 * var_q1_dn5);
        var_t1_dn6 = (var_k1_2 * var_q1_dn6);
        var_t1_dn7 = (var_k1_2 * var_q1_dn7);
        var_t1_dn8 = (var_k1_2 * var_q1_dn8);

        let assign5410_e6053: f64 = (var_t1 * var_q1);
        let assign5410_e6055: f64 = (assign5410_e6053 + var_t0);
        let assign5410_e6057: f64 = (assign5410_e6055 - var_qsqrt);
        let assign5410_e6058: f64 = (-assign5410_e6057);
        let assign5410_e6060: f64 = (-2.0);
        let assign5410_e6062: f64 = (assign5410_e6060 * var_t1);
        let assign5410_e6064: f64 = (assign5410_e6062 + var_t0);
        let assign5410_e6065: f64 = (assign5410_e6058 / assign5410_e6064);
        var_delta = assign5410_e6065;
        var_delta_dn3 = ((((-((((var_t1_dn3 * var_q1) + (var_t1 * var_q1_dn3)) + var_t0_dn3) - var_qsqrt_dn3)) * assign5410_e6064) - (assign5410_e6058 * ((assign5410_e6060 * var_t1_dn3) + var_t0_dn3))) / (assign5410_e6064 * assign5410_e6064));
        var_delta_dn4 = ((((-((((var_t1_dn4 * var_q1) + (var_t1 * var_q1_dn4)) + var_t0_dn4) - var_qsqrt_dn4)) * assign5410_e6064) - (assign5410_e6058 * ((assign5410_e6060 * var_t1_dn4) + var_t0_dn4))) / (assign5410_e6064 * assign5410_e6064));
        var_delta_dn5 = ((((-((((var_t1_dn5 * var_q1) + (var_t1 * var_q1_dn5)) + var_t0_dn5) - var_qsqrt_dn5)) * assign5410_e6064) - (assign5410_e6058 * ((assign5410_e6060 * var_t1_dn5) + var_t0_dn5))) / (assign5410_e6064 * assign5410_e6064));
        var_delta_dn6 = ((((-((((var_t1_dn6 * var_q1) + (var_t1 * var_q1_dn6)) + var_t0_dn6) - var_qsqrt_dn6)) * assign5410_e6064) - (assign5410_e6058 * ((assign5410_e6060 * var_t1_dn6) + var_t0_dn6))) / (assign5410_e6064 * assign5410_e6064));
        var_delta_dn7 = ((((-((((var_t1_dn7 * var_q1) + (var_t1 * var_q1_dn7)) + var_t0_dn7) - var_qsqrt_dn7)) * assign5410_e6064) - (assign5410_e6058 * ((assign5410_e6060 * var_t1_dn7) + var_t0_dn7))) / (assign5410_e6064 * assign5410_e6064));
        var_delta_dn8 = ((((-((((var_t1_dn8 * var_q1) + (var_t1 * var_q1_dn8)) + var_t0_dn8) - var_qsqrt_dn8)) * assign5410_e6064) - (assign5410_e6058 * ((assign5410_e6060 * var_t1_dn8) + var_t0_dn8))) / (assign5410_e6064 * assign5410_e6064));

        let assign5420_e6068: f64 = (var_phissat + var_delta);
        var_phissat = assign5420_e6068;
        var_phissat_dn3 = (var_phissat_dn3 + var_delta_dn3);
        var_phissat_dn4 = (var_phissat_dn4 + var_delta_dn4);
        var_phissat_dn5 = (var_phissat_dn5 + var_delta_dn5);
        var_phissat_dn6 = (var_phissat_dn6 + var_delta_dn6);
        var_phissat_dn7 = (var_phissat_dn7 + var_delta_dn7);
        var_phissat_dn8 = (var_phissat_dn8 + var_delta_dn8);

        let assign5430_e6071: f64 = (var_xg1 - var_phissat);
        var_q1 = assign5430_e6071;
        var_q1_dn3 = (var_xg1_dn3 - var_phissat_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phissat_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phissat_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phissat_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phissat_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phissat_dn8);

        let assign5440_e6074: f64 = (var_k1_2 * var_q1);
        var_t2 = assign5440_e6074;
        var_t2_dn3 = (var_k1_2 * var_q1_dn3);
        var_t2_dn4 = (var_k1_2 * var_q1_dn4);
        var_t2_dn5 = (var_k1_2 * var_q1_dn5);
        var_t2_dn6 = (var_k1_2 * var_q1_dn6);
        var_t2_dn7 = (var_k1_2 * var_q1_dn7);
        var_t2_dn8 = (var_k1_2 * var_q1_dn8);

        let assign5450_e6078: f64 = (var_t2 * var_q1);
        let assign5450_e6080: f64 = (assign5450_e6078 - var_qsqrt);
        let assign5450_e6081: f64 = (1.0 / assign5450_e6080);
        var_t0 = assign5450_e6081;
        var_t0_dn3 = (-((((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3) / (assign5450_e6080 * assign5450_e6080)));
        var_t0_dn4 = (-((((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4) / (assign5450_e6080 * assign5450_e6080)));
        var_t0_dn5 = (-((((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5) / (assign5450_e6080 * assign5450_e6080)));
        var_t0_dn6 = (-((((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6) / (assign5450_e6080 * assign5450_e6080)));
        var_t0_dn7 = (-((((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7) / (assign5450_e6080 * assign5450_e6080)));
        var_t0_dn8 = (-((((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8) / (assign5450_e6080 * assign5450_e6080)));

        let assign5460_e6084: f64 = (var_t2 * var_q1);
        let assign5460_e6086: f64 = (assign5460_e6084 - var_qsqrt);
        let assign5460_e6087: f64 = (assign5460_e6086).abs();
        let assign5460_e6088: f64 = (assign5460_e6087).ln();
        let assign5460_e6090: f64 = (assign5460_e6088 - var_lna0);
        let assign5460_e6092: f64 = (assign5460_e6090 - var_phissat);
        var_g = assign5460_e6092;
        var_g_dn3 = (((if assign5460_e6086 >= 0.0 { (((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3) } else { (-(((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3)) } / assign5460_e6087) - var_lna0_dn3) - var_phissat_dn3);
        var_g_dn4 = (((if assign5460_e6086 >= 0.0 { (((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4) } else { (-(((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4)) } / assign5460_e6087) - var_lna0_dn4) - var_phissat_dn4);
        var_g_dn5 = (((if assign5460_e6086 >= 0.0 { (((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5) } else { (-(((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5)) } / assign5460_e6087) - var_lna0_dn5) - var_phissat_dn5);
        var_g_dn6 = (((if assign5460_e6086 >= 0.0 { (((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6) } else { (-(((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6)) } / assign5460_e6087) - var_lna0_dn6) - var_phissat_dn6);
        var_g_dn7 = (((if assign5460_e6086 >= 0.0 { (((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7) } else { (-(((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7)) } / assign5460_e6087) - var_lna0_dn7) - var_phissat_dn7);
        var_g_dn8 = (((if assign5460_e6086 >= 0.0 { (((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8) } else { (-(((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8)) } / assign5460_e6087) - var_lna0_dn8) - var_phissat_dn8);

        let assign5470_e6095: f64 = (-2.0);
        let assign5470_e6097: f64 = (assign5470_e6095 * var_t2);
        let assign5470_e6099: f64 = (assign5470_e6097 * var_t0);
        let assign5470_e6101: f64 = (assign5470_e6099 - 1.0);
        let assign5470_e6102: f64 = (1.0 / assign5470_e6101);
        var_dg1 = assign5470_e6102;
        var_dg1_dn3 = (-((((assign5470_e6095 * var_t2_dn3) * var_t0) + (assign5470_e6097 * var_t0_dn3)) / (assign5470_e6101 * assign5470_e6101)));
        var_dg1_dn4 = (-((((assign5470_e6095 * var_t2_dn4) * var_t0) + (assign5470_e6097 * var_t0_dn4)) / (assign5470_e6101 * assign5470_e6101)));
        var_dg1_dn5 = (-((((assign5470_e6095 * var_t2_dn5) * var_t0) + (assign5470_e6097 * var_t0_dn5)) / (assign5470_e6101 * assign5470_e6101)));
        var_dg1_dn6 = (-((((assign5470_e6095 * var_t2_dn6) * var_t0) + (assign5470_e6097 * var_t0_dn6)) / (assign5470_e6101 * assign5470_e6101)));
        var_dg1_dn7 = (-((((assign5470_e6095 * var_t2_dn7) * var_t0) + (assign5470_e6097 * var_t0_dn7)) / (assign5470_e6101 * assign5470_e6101)));
        var_dg1_dn8 = (-((((assign5470_e6095 * var_t2_dn8) * var_t0) + (assign5470_e6097 * var_t0_dn8)) / (assign5470_e6101 * assign5470_e6101)));

        let assign5480_e6104: f64 = (-4.0);
        let assign5480_e6106: f64 = (assign5480_e6104 * var_t2);
        let assign5480_e6108: f64 = (assign5480_e6106 * var_t2);
        let assign5480_e6110: f64 = (assign5480_e6108 * var_t0);
        let assign5480_e6112: f64 = (assign5480_e6110 * var_t0);
        let assign5480_e6115: f64 = (2.0 * var_k1_2);
        let assign5480_e6117: f64 = (assign5480_e6115 * var_t0);
        let assign5480_e6118: f64 = (assign5480_e6112 + assign5480_e6117);
        var_dg2 = assign5480_e6118;
        var_dg2_dn3 = ((((((((assign5480_e6104 * var_t2_dn3) * var_t2) + (assign5480_e6106 * var_t2_dn3)) * var_t0) + (assign5480_e6108 * var_t0_dn3)) * var_t0) + (assign5480_e6110 * var_t0_dn3)) + (assign5480_e6115 * var_t0_dn3));
        var_dg2_dn4 = ((((((((assign5480_e6104 * var_t2_dn4) * var_t2) + (assign5480_e6106 * var_t2_dn4)) * var_t0) + (assign5480_e6108 * var_t0_dn4)) * var_t0) + (assign5480_e6110 * var_t0_dn4)) + (assign5480_e6115 * var_t0_dn4));
        var_dg2_dn5 = ((((((((assign5480_e6104 * var_t2_dn5) * var_t2) + (assign5480_e6106 * var_t2_dn5)) * var_t0) + (assign5480_e6108 * var_t0_dn5)) * var_t0) + (assign5480_e6110 * var_t0_dn5)) + (assign5480_e6115 * var_t0_dn5));
        var_dg2_dn6 = ((((((((assign5480_e6104 * var_t2_dn6) * var_t2) + (assign5480_e6106 * var_t2_dn6)) * var_t0) + (assign5480_e6108 * var_t0_dn6)) * var_t0) + (assign5480_e6110 * var_t0_dn6)) + (assign5480_e6115 * var_t0_dn6));
        var_dg2_dn7 = ((((((((assign5480_e6104 * var_t2_dn7) * var_t2) + (assign5480_e6106 * var_t2_dn7)) * var_t0) + (assign5480_e6108 * var_t0_dn7)) * var_t0) + (assign5480_e6110 * var_t0_dn7)) + (assign5480_e6115 * var_t0_dn7));
        var_dg2_dn8 = ((((((((assign5480_e6104 * var_t2_dn8) * var_t2) + (assign5480_e6106 * var_t2_dn8)) * var_t0) + (assign5480_e6108 * var_t0_dn8)) * var_t0) + (assign5480_e6110 * var_t0_dn8)) + (assign5480_e6115 * var_t0_dn8));

        let assign5490_e6121: f64 = (var_g * var_dg1);
        var_t1 = assign5490_e6121;
        var_t1_dn3 = ((var_g_dn3 * var_dg1) + (var_g * var_dg1_dn3));
        var_t1_dn4 = ((var_g_dn4 * var_dg1) + (var_g * var_dg1_dn4));
        var_t1_dn5 = ((var_g_dn5 * var_dg1) + (var_g * var_dg1_dn5));
        var_t1_dn6 = ((var_g_dn6 * var_dg1) + (var_g * var_dg1_dn6));
        var_t1_dn7 = ((var_g_dn7 * var_dg1) + (var_g * var_dg1_dn7));
        var_t1_dn8 = ((var_g_dn8 * var_dg1) + (var_g * var_dg1_dn8));

        let assign5500_e6123: f64 = (-var_t1);
        let assign5500_e6126: f64 = (0.5 * var_t1);
        let assign5500_e6128: f64 = (assign5500_e6126 * var_t1);
        let assign5500_e6130: f64 = (assign5500_e6128 * var_dg2);
        let assign5500_e6132: f64 = (assign5500_e6130 * var_dg1);
        let assign5500_e6133: f64 = (assign5500_e6123 - assign5500_e6132);
        var_delta = assign5500_e6133;
        var_delta_dn3 = ((-var_t1_dn3) - (((((((0.5 * var_t1_dn3) * var_t1) + (assign5500_e6126 * var_t1_dn3)) * var_dg2) + (assign5500_e6128 * var_dg2_dn3)) * var_dg1) + (assign5500_e6130 * var_dg1_dn3)));
        var_delta_dn4 = ((-var_t1_dn4) - (((((((0.5 * var_t1_dn4) * var_t1) + (assign5500_e6126 * var_t1_dn4)) * var_dg2) + (assign5500_e6128 * var_dg2_dn4)) * var_dg1) + (assign5500_e6130 * var_dg1_dn4)));
        var_delta_dn5 = ((-var_t1_dn5) - (((((((0.5 * var_t1_dn5) * var_t1) + (assign5500_e6126 * var_t1_dn5)) * var_dg2) + (assign5500_e6128 * var_dg2_dn5)) * var_dg1) + (assign5500_e6130 * var_dg1_dn5)));
        var_delta_dn6 = ((-var_t1_dn6) - (((((((0.5 * var_t1_dn6) * var_t1) + (assign5500_e6126 * var_t1_dn6)) * var_dg2) + (assign5500_e6128 * var_dg2_dn6)) * var_dg1) + (assign5500_e6130 * var_dg1_dn6)));
        var_delta_dn7 = ((-var_t1_dn7) - (((((((0.5 * var_t1_dn7) * var_t1) + (assign5500_e6126 * var_t1_dn7)) * var_dg2) + (assign5500_e6128 * var_dg2_dn7)) * var_dg1) + (assign5500_e6130 * var_dg1_dn7)));
        var_delta_dn8 = ((-var_t1_dn8) - (((((((0.5 * var_t1_dn8) * var_t1) + (assign5500_e6126 * var_t1_dn8)) * var_dg2) + (assign5500_e6128 * var_dg2_dn8)) * var_dg1) + (assign5500_e6130 * var_dg1_dn8)));

        let assign5510_e6136: f64 = (-10.0);
        let assign5510_e6137: f64 = (var_delta).max(assign5510_e6136);
        var_delta = assign5510_e6137;
        var_delta_dn3 = if var_delta >= assign5510_e6136 { var_delta_dn3 } else { 0.0 };
        var_delta_dn4 = if var_delta >= assign5510_e6136 { var_delta_dn4 } else { 0.0 };
        var_delta_dn5 = if var_delta >= assign5510_e6136 { var_delta_dn5 } else { 0.0 };
        var_delta_dn6 = if var_delta >= assign5510_e6136 { var_delta_dn6 } else { 0.0 };
        var_delta_dn7 = if var_delta >= assign5510_e6136 { var_delta_dn7 } else { 0.0 };
        var_delta_dn8 = if var_delta >= assign5510_e6136 { var_delta_dn8 } else { 0.0 };

        let assign5520_e6140: f64 = (var_delta).min(10.0);
        var_delta = assign5520_e6140;
        var_delta_dn3 = if var_delta <= 10.0 { var_delta_dn3 } else { 0.0 };
        var_delta_dn4 = if var_delta <= 10.0 { var_delta_dn4 } else { 0.0 };
        var_delta_dn5 = if var_delta <= 10.0 { var_delta_dn5 } else { 0.0 };
        var_delta_dn6 = if var_delta <= 10.0 { var_delta_dn6 } else { 0.0 };
        var_delta_dn7 = if var_delta <= 10.0 { var_delta_dn7 } else { 0.0 };
        var_delta_dn8 = if var_delta <= 10.0 { var_delta_dn8 } else { 0.0 };

        let assign5530_e6143: f64 = (var_phissat + var_delta);
        var_phissat = assign5530_e6143;
        var_phissat_dn3 = (var_phissat_dn3 + var_delta_dn3);
        var_phissat_dn4 = (var_phissat_dn4 + var_delta_dn4);
        var_phissat_dn5 = (var_phissat_dn5 + var_delta_dn5);
        var_phissat_dn6 = (var_phissat_dn6 + var_delta_dn6);
        var_phissat_dn7 = (var_phissat_dn7 + var_delta_dn7);
        var_phissat_dn8 = (var_phissat_dn8 + var_delta_dn8);

        let assign5540_e6146: f64 = (var_xg1 - var_phissat);
        var_q1 = assign5540_e6146;
        var_q1_dn3 = (var_xg1_dn3 - var_phissat_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phissat_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phissat_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phissat_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phissat_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phissat_dn8);

        let assign5550_e6149: f64 = (var_k1_2 * var_q1);
        var_t2 = assign5550_e6149;
        var_t2_dn3 = (var_k1_2 * var_q1_dn3);
        var_t2_dn4 = (var_k1_2 * var_q1_dn4);
        var_t2_dn5 = (var_k1_2 * var_q1_dn5);
        var_t2_dn6 = (var_k1_2 * var_q1_dn6);
        var_t2_dn7 = (var_k1_2 * var_q1_dn7);
        var_t2_dn8 = (var_k1_2 * var_q1_dn8);

        let assign5560_e6153: f64 = (var_t2 * var_q1);
        let assign5560_e6155: f64 = (assign5560_e6153 - var_qsqrt);
        let assign5560_e6156: f64 = (1.0 / assign5560_e6155);
        var_t0 = assign5560_e6156;
        var_t0_dn3 = (-((((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3) / (assign5560_e6155 * assign5560_e6155)));
        var_t0_dn4 = (-((((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4) / (assign5560_e6155 * assign5560_e6155)));
        var_t0_dn5 = (-((((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5) / (assign5560_e6155 * assign5560_e6155)));
        var_t0_dn6 = (-((((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6) / (assign5560_e6155 * assign5560_e6155)));
        var_t0_dn7 = (-((((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7) / (assign5560_e6155 * assign5560_e6155)));
        var_t0_dn8 = (-((((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8) / (assign5560_e6155 * assign5560_e6155)));

        let assign5570_e6159: f64 = (var_t2 * var_q1);
        let assign5570_e6161: f64 = (assign5570_e6159 - var_qsqrt);
        let assign5570_e6162: f64 = (assign5570_e6161).abs();
        let assign5570_e6163: f64 = (assign5570_e6162).ln();
        let assign5570_e6165: f64 = (assign5570_e6163 - var_lna0);
        let assign5570_e6167: f64 = (assign5570_e6165 - var_phissat);
        var_g = assign5570_e6167;
        var_g_dn3 = (((if assign5570_e6161 >= 0.0 { (((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3) } else { (-(((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3)) } / assign5570_e6162) - var_lna0_dn3) - var_phissat_dn3);
        var_g_dn4 = (((if assign5570_e6161 >= 0.0 { (((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4) } else { (-(((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4)) } / assign5570_e6162) - var_lna0_dn4) - var_phissat_dn4);
        var_g_dn5 = (((if assign5570_e6161 >= 0.0 { (((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5) } else { (-(((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5)) } / assign5570_e6162) - var_lna0_dn5) - var_phissat_dn5);
        var_g_dn6 = (((if assign5570_e6161 >= 0.0 { (((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6) } else { (-(((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6)) } / assign5570_e6162) - var_lna0_dn6) - var_phissat_dn6);
        var_g_dn7 = (((if assign5570_e6161 >= 0.0 { (((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7) } else { (-(((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7)) } / assign5570_e6162) - var_lna0_dn7) - var_phissat_dn7);
        var_g_dn8 = (((if assign5570_e6161 >= 0.0 { (((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8) } else { (-(((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8)) } / assign5570_e6162) - var_lna0_dn8) - var_phissat_dn8);

        let assign5580_e6170: f64 = (-2.0);
        let assign5580_e6172: f64 = (assign5580_e6170 * var_t2);
        let assign5580_e6174: f64 = (assign5580_e6172 * var_t0);
        let assign5580_e6176: f64 = (assign5580_e6174 - 1.0);
        let assign5580_e6177: f64 = (1.0 / assign5580_e6176);
        var_dg1 = assign5580_e6177;
        var_dg1_dn3 = (-((((assign5580_e6170 * var_t2_dn3) * var_t0) + (assign5580_e6172 * var_t0_dn3)) / (assign5580_e6176 * assign5580_e6176)));
        var_dg1_dn4 = (-((((assign5580_e6170 * var_t2_dn4) * var_t0) + (assign5580_e6172 * var_t0_dn4)) / (assign5580_e6176 * assign5580_e6176)));
        var_dg1_dn5 = (-((((assign5580_e6170 * var_t2_dn5) * var_t0) + (assign5580_e6172 * var_t0_dn5)) / (assign5580_e6176 * assign5580_e6176)));
        var_dg1_dn6 = (-((((assign5580_e6170 * var_t2_dn6) * var_t0) + (assign5580_e6172 * var_t0_dn6)) / (assign5580_e6176 * assign5580_e6176)));
        var_dg1_dn7 = (-((((assign5580_e6170 * var_t2_dn7) * var_t0) + (assign5580_e6172 * var_t0_dn7)) / (assign5580_e6176 * assign5580_e6176)));
        var_dg1_dn8 = (-((((assign5580_e6170 * var_t2_dn8) * var_t0) + (assign5580_e6172 * var_t0_dn8)) / (assign5580_e6176 * assign5580_e6176)));

        let assign5590_e6179: f64 = (-4.0);
        let assign5590_e6181: f64 = (assign5590_e6179 * var_t2);
        let assign5590_e6183: f64 = (assign5590_e6181 * var_t2);
        let assign5590_e6185: f64 = (assign5590_e6183 * var_t0);
        let assign5590_e6187: f64 = (assign5590_e6185 * var_t0);
        let assign5590_e6190: f64 = (2.0 * var_k1_2);
        let assign5590_e6192: f64 = (assign5590_e6190 * var_t0);
        let assign5590_e6193: f64 = (assign5590_e6187 + assign5590_e6192);
        var_dg2 = assign5590_e6193;
        var_dg2_dn3 = ((((((((assign5590_e6179 * var_t2_dn3) * var_t2) + (assign5590_e6181 * var_t2_dn3)) * var_t0) + (assign5590_e6183 * var_t0_dn3)) * var_t0) + (assign5590_e6185 * var_t0_dn3)) + (assign5590_e6190 * var_t0_dn3));
        var_dg2_dn4 = ((((((((assign5590_e6179 * var_t2_dn4) * var_t2) + (assign5590_e6181 * var_t2_dn4)) * var_t0) + (assign5590_e6183 * var_t0_dn4)) * var_t0) + (assign5590_e6185 * var_t0_dn4)) + (assign5590_e6190 * var_t0_dn4));
        var_dg2_dn5 = ((((((((assign5590_e6179 * var_t2_dn5) * var_t2) + (assign5590_e6181 * var_t2_dn5)) * var_t0) + (assign5590_e6183 * var_t0_dn5)) * var_t0) + (assign5590_e6185 * var_t0_dn5)) + (assign5590_e6190 * var_t0_dn5));
        var_dg2_dn6 = ((((((((assign5590_e6179 * var_t2_dn6) * var_t2) + (assign5590_e6181 * var_t2_dn6)) * var_t0) + (assign5590_e6183 * var_t0_dn6)) * var_t0) + (assign5590_e6185 * var_t0_dn6)) + (assign5590_e6190 * var_t0_dn6));
        var_dg2_dn7 = ((((((((assign5590_e6179 * var_t2_dn7) * var_t2) + (assign5590_e6181 * var_t2_dn7)) * var_t0) + (assign5590_e6183 * var_t0_dn7)) * var_t0) + (assign5590_e6185 * var_t0_dn7)) + (assign5590_e6190 * var_t0_dn7));
        var_dg2_dn8 = ((((((((assign5590_e6179 * var_t2_dn8) * var_t2) + (assign5590_e6181 * var_t2_dn8)) * var_t0) + (assign5590_e6183 * var_t0_dn8)) * var_t0) + (assign5590_e6185 * var_t0_dn8)) + (assign5590_e6190 * var_t0_dn8));

        let assign5600_e6196: f64 = (var_g * var_dg1);
        var_t1 = assign5600_e6196;
        var_t1_dn3 = ((var_g_dn3 * var_dg1) + (var_g * var_dg1_dn3));
        var_t1_dn4 = ((var_g_dn4 * var_dg1) + (var_g * var_dg1_dn4));
        var_t1_dn5 = ((var_g_dn5 * var_dg1) + (var_g * var_dg1_dn5));
        var_t1_dn6 = ((var_g_dn6 * var_dg1) + (var_g * var_dg1_dn6));
        var_t1_dn7 = ((var_g_dn7 * var_dg1) + (var_g * var_dg1_dn7));
        var_t1_dn8 = ((var_g_dn8 * var_dg1) + (var_g * var_dg1_dn8));

        let assign5610_e6198: f64 = (-var_t1);
        let assign5610_e6201: f64 = (0.5 * var_t1);
        let assign5610_e6203: f64 = (assign5610_e6201 * var_t1);
        let assign5610_e6205: f64 = (assign5610_e6203 * var_dg2);
        let assign5610_e6207: f64 = (assign5610_e6205 * var_dg1);
        let assign5610_e6208: f64 = (assign5610_e6198 - assign5610_e6207);
        var_delta = assign5610_e6208;
        var_delta_dn3 = ((-var_t1_dn3) - (((((((0.5 * var_t1_dn3) * var_t1) + (assign5610_e6201 * var_t1_dn3)) * var_dg2) + (assign5610_e6203 * var_dg2_dn3)) * var_dg1) + (assign5610_e6205 * var_dg1_dn3)));
        var_delta_dn4 = ((-var_t1_dn4) - (((((((0.5 * var_t1_dn4) * var_t1) + (assign5610_e6201 * var_t1_dn4)) * var_dg2) + (assign5610_e6203 * var_dg2_dn4)) * var_dg1) + (assign5610_e6205 * var_dg1_dn4)));
        var_delta_dn5 = ((-var_t1_dn5) - (((((((0.5 * var_t1_dn5) * var_t1) + (assign5610_e6201 * var_t1_dn5)) * var_dg2) + (assign5610_e6203 * var_dg2_dn5)) * var_dg1) + (assign5610_e6205 * var_dg1_dn5)));
        var_delta_dn6 = ((-var_t1_dn6) - (((((((0.5 * var_t1_dn6) * var_t1) + (assign5610_e6201 * var_t1_dn6)) * var_dg2) + (assign5610_e6203 * var_dg2_dn6)) * var_dg1) + (assign5610_e6205 * var_dg1_dn6)));
        var_delta_dn7 = ((-var_t1_dn7) - (((((((0.5 * var_t1_dn7) * var_t1) + (assign5610_e6201 * var_t1_dn7)) * var_dg2) + (assign5610_e6203 * var_dg2_dn7)) * var_dg1) + (assign5610_e6205 * var_dg1_dn7)));
        var_delta_dn8 = ((-var_t1_dn8) - (((((((0.5 * var_t1_dn8) * var_t1) + (assign5610_e6201 * var_t1_dn8)) * var_dg2) + (assign5610_e6203 * var_dg2_dn8)) * var_dg1) + (assign5610_e6205 * var_dg1_dn8)));

        let assign5620_e6211: f64 = (-10.0);
        let assign5620_e6212: f64 = (var_delta).max(assign5620_e6211);
        var_delta = assign5620_e6212;
        var_delta_dn3 = if var_delta >= assign5620_e6211 { var_delta_dn3 } else { 0.0 };
        var_delta_dn4 = if var_delta >= assign5620_e6211 { var_delta_dn4 } else { 0.0 };
        var_delta_dn5 = if var_delta >= assign5620_e6211 { var_delta_dn5 } else { 0.0 };
        var_delta_dn6 = if var_delta >= assign5620_e6211 { var_delta_dn6 } else { 0.0 };
        var_delta_dn7 = if var_delta >= assign5620_e6211 { var_delta_dn7 } else { 0.0 };
        var_delta_dn8 = if var_delta >= assign5620_e6211 { var_delta_dn8 } else { 0.0 };

        let assign5630_e6215: f64 = (var_delta).min(10.0);
        var_delta = assign5630_e6215;
        var_delta_dn3 = if var_delta <= 10.0 { var_delta_dn3 } else { 0.0 };
        var_delta_dn4 = if var_delta <= 10.0 { var_delta_dn4 } else { 0.0 };
        var_delta_dn5 = if var_delta <= 10.0 { var_delta_dn5 } else { 0.0 };
        var_delta_dn6 = if var_delta <= 10.0 { var_delta_dn6 } else { 0.0 };
        var_delta_dn7 = if var_delta <= 10.0 { var_delta_dn7 } else { 0.0 };
        var_delta_dn8 = if var_delta <= 10.0 { var_delta_dn8 } else { 0.0 };

        let assign5640_e6218: f64 = (var_phissat + var_delta);
        var_phissat = assign5640_e6218;
        var_phissat_dn3 = (var_phissat_dn3 + var_delta_dn3);
        var_phissat_dn4 = (var_phissat_dn4 + var_delta_dn4);
        var_phissat_dn5 = (var_phissat_dn5 + var_delta_dn5);
        var_phissat_dn6 = (var_phissat_dn6 + var_delta_dn6);
        var_phissat_dn7 = (var_phissat_dn7 + var_delta_dn7);
        var_phissat_dn8 = (var_phissat_dn8 + var_delta_dn8);

        let assign5650_e6222: f64 = (var_phi1_0 - 4.0);
        let assign5650_e6223: f64 = (var_phissat).max(assign5650_e6222);
        var_phissat = assign5650_e6223;
        var_phissat_dn3 = if var_phissat >= assign5650_e6222 { var_phissat_dn3 } else { var_phi1_0_dn3 };
        var_phissat_dn4 = if var_phissat >= assign5650_e6222 { var_phissat_dn4 } else { var_phi1_0_dn4 };
        var_phissat_dn5 = if var_phissat >= assign5650_e6222 { var_phissat_dn5 } else { var_phi1_0_dn5 };
        var_phissat_dn6 = if var_phissat >= assign5650_e6222 { var_phissat_dn6 } else { var_phi1_0_dn6 };
        var_phissat_dn7 = if var_phissat >= assign5650_e6222 { var_phissat_dn7 } else { var_phi1_0_dn7 };
        var_phissat_dn8 = if var_phissat >= assign5650_e6222 { var_phissat_dn8 } else { var_phi1_0_dn8 };

        let assign5660_e6226: f64 = (var_vgfb1eff / var_nvtm);
        var_xg1 = assign5660_e6226;
        var_xg1_dn3 = (((var_vgfb1eff_dn3 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn3)) / (var_nvtm * var_nvtm));
        var_xg1_dn4 = (((var_vgfb1eff_dn4 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn4)) / (var_nvtm * var_nvtm));
        var_xg1_dn5 = (((var_vgfb1eff_dn5 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn5)) / (var_nvtm * var_nvtm));
        var_xg1_dn6 = (((var_vgfb1eff_dn6 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn6)) / (var_nvtm * var_nvtm));
        var_xg1_dn7 = (((var_vgfb1eff_dn7 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn7)) / (var_nvtm * var_nvtm));
        var_xg1_dn8 = (((var_vgfb1eff_dn8 * var_nvtm) - (var_vgfb1eff * var_nvtm_dn8)) / (var_nvtm * var_nvtm));

        let assign5670_e6233: f64 = (1.05 * var_phissat);
        let assign5670_e6234: f64 = (var_phi1 - assign5670_e6233);
        let assign5670_e6236: f64 = assign5670_e6234;
        let assign5670_e6237: f64 = (assign5670_e6236).exp();
        let assign5670_e6238: f64 = (1.0 + assign5670_e6237);
        let assign5670_e6239: f64 = (assign5670_e6238).ln();
        let assign5670_e6240: f64 = assign5670_e6239;
        let assign5670_e6241: f64 = (var_phi1 - assign5670_e6240);
        var_phi1 = assign5670_e6241;
        var_phi1_dn3 = (var_phi1_dn3 - ((assign5670_e6237 * (var_phi1_dn3 - (1.05 * var_phissat_dn3))) / assign5670_e6238));
        var_phi1_dn4 = (var_phi1_dn4 - ((assign5670_e6237 * (var_phi1_dn4 - (1.05 * var_phissat_dn4))) / assign5670_e6238));
        var_phi1_dn5 = (var_phi1_dn5 - ((assign5670_e6237 * (var_phi1_dn5 - (1.05 * var_phissat_dn5))) / assign5670_e6238));
        var_phi1_dn6 = (var_phi1_dn6 - ((assign5670_e6237 * (var_phi1_dn6 - (1.05 * var_phissat_dn6))) / assign5670_e6238));
        var_phi1_dn7 = (var_phi1_dn7 - ((assign5670_e6237 * (var_phi1_dn7 - (1.05 * var_phissat_dn7))) / assign5670_e6238));
        var_phi1_dn8 = (var_phi1_dn8 - ((assign5670_e6237 * (var_phi1_dn8 - (1.05 * var_phissat_dn8))) / assign5670_e6238));

        let assign5680_e6244: f64 = (var_phi1).min(var_phissat);
        var_phi1 = assign5680_e6244;
        var_phi1_dn3 = if var_phi1 <= var_phissat { var_phi1_dn3 } else { var_phissat_dn3 };
        var_phi1_dn4 = if var_phi1 <= var_phissat { var_phi1_dn4 } else { var_phissat_dn4 };
        var_phi1_dn5 = if var_phi1 <= var_phissat { var_phi1_dn5 } else { var_phissat_dn5 };
        var_phi1_dn6 = if var_phi1 <= var_phissat { var_phi1_dn6 } else { var_phissat_dn6 };
        var_phi1_dn7 = if var_phi1 <= var_phissat { var_phi1_dn7 } else { var_phissat_dn7 };
        var_phi1_dn8 = if var_phi1 <= var_phissat { var_phi1_dn8 } else { var_phissat_dn8 };

        let assign5690_e6247: f64 = (var_xg1 - var_phi1);
        var_q1 = assign5690_e6247;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign5700_e6250: f64 = (var_k1 * var_q1);
        var_auxb1 = assign5700_e6250;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign5710_e6252: f64 = (-var_a0);
        let assign5710_e6254: f64 = (var_phi1).exp();
        let assign5710_e6255: f64 = (assign5710_e6252 * assign5710_e6254);
        var_aaux = assign5710_e6255;
        var_aaux_dn3 = (((-var_a0_dn3) * assign5710_e6254) + (assign5710_e6252 * (assign5710_e6254 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign5710_e6254) + (assign5710_e6252 * (assign5710_e6254 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign5710_e6254) + (assign5710_e6252 * (assign5710_e6254 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign5710_e6254) + (assign5710_e6252 * (assign5710_e6254 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign5710_e6254) + (assign5710_e6252 * (assign5710_e6254 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign5710_e6254) + (assign5710_e6252 * (assign5710_e6254 * var_phi1_dn8)));

        let assign5720_e6258: f64 = (var_auxb1 * var_auxb1);
        let assign5720_e6260: f64 = (assign5720_e6258 + var_aaux);
        var_qsqrt = assign5720_e6260;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign5730_e6263: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard77 = assign5730_e6263;

        let (assign5740_e6269, assign5740_e6269_d_n3, assign5740_e6269_d_n4, assign5740_e6269_d_n5, assign5740_e6269_d_n6, assign5740_e6269_d_n7, assign5740_e6269_d_n8,) = {
    if (var_guard77 != 0.0) {
        let assign5740_e6266: f64 = (-var_qsqrt);
        let assign5740_e6267: f64 = (assign5740_e6266).sqrt();
        (assign5740_e6267, ((-var_qsqrt_dn3) / (2.0 * assign5740_e6267)), ((-var_qsqrt_dn4) / (2.0 * assign5740_e6267)), ((-var_qsqrt_dn5) / (2.0 * assign5740_e6267)), ((-var_qsqrt_dn6) / (2.0 * assign5740_e6267)), ((-var_qsqrt_dn7) / (2.0 * assign5740_e6267)), ((-var_qsqrt_dn8) / (2.0 * assign5740_e6267)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign5740_e6269;
        var_q_dn3 = assign5740_e6269_d_n3;
        var_q_dn4 = assign5740_e6269_d_n4;
        var_q_dn5 = assign5740_e6269_d_n5;
        var_q_dn6 = assign5740_e6269_d_n6;
        var_q_dn7 = assign5740_e6269_d_n7;
        var_q_dn8 = assign5740_e6269_d_n8;

        let (assign5750_e6278, assign5750_e6278_d_n3, assign5750_e6278_d_n4, assign5750_e6278_d_n5, assign5750_e6278_d_n6, assign5750_e6278_d_n7, assign5750_e6278_d_n8,) = {
    if (var_guard77 != 0.0) {
        let assign5750_e6274: f64 = (0.5 * var_q);
        let assign5750_e6275: f64 = (assign5750_e6274).sin();
        let assign5750_e6276: f64 = (1.0 / assign5750_e6275);
        (assign5750_e6276, (-(((assign5750_e6274).cos() * (0.5 * var_q_dn3)) / (assign5750_e6275 * assign5750_e6275))), (-(((assign5750_e6274).cos() * (0.5 * var_q_dn4)) / (assign5750_e6275 * assign5750_e6275))), (-(((assign5750_e6274).cos() * (0.5 * var_q_dn5)) / (assign5750_e6275 * assign5750_e6275))), (-(((assign5750_e6274).cos() * (0.5 * var_q_dn6)) / (assign5750_e6275 * assign5750_e6275))), (-(((assign5750_e6274).cos() * (0.5 * var_q_dn7)) / (assign5750_e6275 * assign5750_e6275))), (-(((assign5750_e6274).cos() * (0.5 * var_q_dn8)) / (assign5750_e6275 * assign5750_e6275))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign5750_e6278;
        var_csc1_dn3 = assign5750_e6278_d_n3;
        var_csc1_dn4 = assign5750_e6278_d_n4;
        var_csc1_dn5 = assign5750_e6278_d_n5;
        var_csc1_dn6 = assign5750_e6278_d_n6;
        var_csc1_dn7 = assign5750_e6278_d_n7;
        var_csc1_dn8 = assign5750_e6278_d_n8;

        *var_aaux_slot = var_aaux;
        *var_aaux_dn3_slot = var_aaux_dn3;
        *var_aaux_dn4_slot = var_aaux_dn4;
        *var_aaux_dn5_slot = var_aaux_dn5;
        *var_aaux_dn6_slot = var_aaux_dn6;
        *var_aaux_dn7_slot = var_aaux_dn7;
        *var_aaux_dn8_slot = var_aaux_dn8;
        *var_auxb1_slot = var_auxb1;
        *var_auxb1_dn3_slot = var_auxb1_dn3;
        *var_auxb1_dn4_slot = var_auxb1_dn4;
        *var_auxb1_dn5_slot = var_auxb1_dn5;
        *var_auxb1_dn6_slot = var_auxb1_dn6;
        *var_auxb1_dn7_slot = var_auxb1_dn7;
        *var_auxb1_dn8_slot = var_auxb1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_dg1_slot = var_dg1;
        *var_dg1_dn3_slot = var_dg1_dn3;
        *var_dg1_dn4_slot = var_dg1_dn4;
        *var_dg1_dn5_slot = var_dg1_dn5;
        *var_dg1_dn6_slot = var_dg1_dn6;
        *var_dg1_dn7_slot = var_dg1_dn7;
        *var_dg1_dn8_slot = var_dg1_dn8;
        *var_dg2_slot = var_dg2;
        *var_dg2_dn3_slot = var_dg2_dn3;
        *var_dg2_dn4_slot = var_dg2_dn4;
        *var_dg2_dn5_slot = var_dg2_dn5;
        *var_dg2_dn6_slot = var_dg2_dn6;
        *var_dg2_dn7_slot = var_dg2_dn7;
        *var_dg2_dn8_slot = var_dg2_dn8;
        *var_g_slot = var_g;
        *var_g_dn3_slot = var_g_dn3;
        *var_g_dn4_slot = var_g_dn4;
        *var_g_dn5_slot = var_g_dn5;
        *var_g_dn6_slot = var_g_dn6;
        *var_g_dn7_slot = var_g_dn7;
        *var_g_dn8_slot = var_g_dn8;
        *var_guard77_slot = var_guard77;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_phissat_slot = var_phissat;
        *var_phissat_dn3_slot = var_phissat_dn3;
        *var_phissat_dn4_slot = var_phissat_dn4;
        *var_phissat_dn5_slot = var_phissat_dn5;
        *var_phissat_dn6_slot = var_phissat_dn6;
        *var_phissat_dn7_slot = var_phissat_dn7;
        *var_phissat_dn8_slot = var_phissat_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_xg1_slot = var_xg1;
        *var_xg1_dn3_slot = var_xg1_dn3;
        *var_xg1_dn4_slot = var_xg1_dn4;
        *var_xg1_dn5_slot = var_xg1_dn5;
        *var_xg1_dn6_slot = var_xg1_dn6;
        *var_xg1_dn7_slot = var_xg1_dn7;
        *var_xg1_dn8_slot = var_xg1_dn8;
    }

    pub(super) fn stamp_transient_block_10(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_guard77: f64,
        var_k1: f64,
        var_k2: f64,
        var_xg1: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_aaux_slot: &mut f64,
        var_aaux_dn3_slot: &mut f64,
        var_aaux_dn4_slot: &mut f64,
        var_aaux_dn5_slot: &mut f64,
        var_aaux_dn6_slot: &mut f64,
        var_aaux_dn7_slot: &mut f64,
        var_aaux_dn8_slot: &mut f64,
        var_auxb1_slot: &mut f64,
        var_auxb1_dn3_slot: &mut f64,
        var_auxb1_dn4_slot: &mut f64,
        var_auxb1_dn5_slot: &mut f64,
        var_auxb1_dn6_slot: &mut f64,
        var_auxb1_dn7_slot: &mut f64,
        var_auxb1_dn8_slot: &mut f64,
        var_coth1_slot: &mut f64,
        var_coth1_dn3_slot: &mut f64,
        var_coth1_dn4_slot: &mut f64,
        var_coth1_dn5_slot: &mut f64,
        var_coth1_dn6_slot: &mut f64,
        var_coth1_dn7_slot: &mut f64,
        var_coth1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_df_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_dn6_slot: &mut f64,
        var_df_dn7_slot: &mut f64,
        var_df_dn8_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn3_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn4_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn5_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn6_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn7_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn8_slot: &mut f64,
        var_dq2_slot: &mut f64,
        var_dq2_dn3_slot: &mut f64,
        var_dq2_dn4_slot: &mut f64,
        var_dq2_dn5_slot: &mut f64,
        var_dq2_dn6_slot: &mut f64,
        var_dq2_dn7_slot: &mut f64,
        var_dq2_dn8_slot: &mut f64,
        var_dqcoth_slot: &mut f64,
        var_dqcoth_dn3_slot: &mut f64,
        var_dqcoth_dn4_slot: &mut f64,
        var_dqcoth_dn5_slot: &mut f64,
        var_dqcoth_dn6_slot: &mut f64,
        var_dqcoth_dn7_slot: &mut f64,
        var_dqcoth_dn8_slot: &mut f64,
        var_dqcothqdqsqrt_slot: &mut f64,
        var_dqcothqdqsqrt_dn3_slot: &mut f64,
        var_dqcothqdqsqrt_dn4_slot: &mut f64,
        var_dqcothqdqsqrt_dn5_slot: &mut f64,
        var_dqcothqdqsqrt_dn6_slot: &mut f64,
        var_dqcothqdqsqrt_dn7_slot: &mut f64,
        var_dqcothqdqsqrt_dn8_slot: &mut f64,
        var_dqsqrt_slot: &mut f64,
        var_dqsqrt_dn3_slot: &mut f64,
        var_dqsqrt_dn4_slot: &mut f64,
        var_dqsqrt_dn5_slot: &mut f64,
        var_dqsqrt_dn6_slot: &mut f64,
        var_dqsqrt_dn7_slot: &mut f64,
        var_dqsqrt_dn8_slot: &mut f64,
        var_f_slot: &mut f64,
        var_f_dn3_slot: &mut f64,
        var_f_dn4_slot: &mut f64,
        var_f_dn5_slot: &mut f64,
        var_f_dn6_slot: &mut f64,
        var_f_dn7_slot: &mut f64,
        var_f_dn8_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
    ) {
        let mut var_aaux: f64 = *var_aaux_slot;
        let mut var_aaux_dn3: f64 = *var_aaux_dn3_slot;
        let mut var_aaux_dn4: f64 = *var_aaux_dn4_slot;
        let mut var_aaux_dn5: f64 = *var_aaux_dn5_slot;
        let mut var_aaux_dn6: f64 = *var_aaux_dn6_slot;
        let mut var_aaux_dn7: f64 = *var_aaux_dn7_slot;
        let mut var_aaux_dn8: f64 = *var_aaux_dn8_slot;
        let mut var_auxb1: f64 = *var_auxb1_slot;
        let mut var_auxb1_dn3: f64 = *var_auxb1_dn3_slot;
        let mut var_auxb1_dn4: f64 = *var_auxb1_dn4_slot;
        let mut var_auxb1_dn5: f64 = *var_auxb1_dn5_slot;
        let mut var_auxb1_dn6: f64 = *var_auxb1_dn6_slot;
        let mut var_auxb1_dn7: f64 = *var_auxb1_dn7_slot;
        let mut var_auxb1_dn8: f64 = *var_auxb1_dn8_slot;
        let mut var_coth1: f64 = *var_coth1_slot;
        let mut var_coth1_dn3: f64 = *var_coth1_dn3_slot;
        let mut var_coth1_dn4: f64 = *var_coth1_dn4_slot;
        let mut var_coth1_dn5: f64 = *var_coth1_dn5_slot;
        let mut var_coth1_dn6: f64 = *var_coth1_dn6_slot;
        let mut var_coth1_dn7: f64 = *var_coth1_dn7_slot;
        let mut var_coth1_dn8: f64 = *var_coth1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_dn6: f64 = *var_df_dn6_slot;
        let mut var_df_dn7: f64 = *var_df_dn7_slot;
        let mut var_df_dn8: f64 = *var_df_dn8_slot;
        let mut var_dlogsinhqsqdqsqrt: f64 = *var_dlogsinhqsqdqsqrt_slot;
        let mut var_dlogsinhqsqdqsqrt_dn3: f64 = *var_dlogsinhqsqdqsqrt_dn3_slot;
        let mut var_dlogsinhqsqdqsqrt_dn4: f64 = *var_dlogsinhqsqdqsqrt_dn4_slot;
        let mut var_dlogsinhqsqdqsqrt_dn5: f64 = *var_dlogsinhqsqdqsqrt_dn5_slot;
        let mut var_dlogsinhqsqdqsqrt_dn6: f64 = *var_dlogsinhqsqdqsqrt_dn6_slot;
        let mut var_dlogsinhqsqdqsqrt_dn7: f64 = *var_dlogsinhqsqdqsqrt_dn7_slot;
        let mut var_dlogsinhqsqdqsqrt_dn8: f64 = *var_dlogsinhqsqdqsqrt_dn8_slot;
        let mut var_dq2: f64 = *var_dq2_slot;
        let mut var_dq2_dn3: f64 = *var_dq2_dn3_slot;
        let mut var_dq2_dn4: f64 = *var_dq2_dn4_slot;
        let mut var_dq2_dn5: f64 = *var_dq2_dn5_slot;
        let mut var_dq2_dn6: f64 = *var_dq2_dn6_slot;
        let mut var_dq2_dn7: f64 = *var_dq2_dn7_slot;
        let mut var_dq2_dn8: f64 = *var_dq2_dn8_slot;
        let mut var_dqcoth: f64 = *var_dqcoth_slot;
        let mut var_dqcoth_dn3: f64 = *var_dqcoth_dn3_slot;
        let mut var_dqcoth_dn4: f64 = *var_dqcoth_dn4_slot;
        let mut var_dqcoth_dn5: f64 = *var_dqcoth_dn5_slot;
        let mut var_dqcoth_dn6: f64 = *var_dqcoth_dn6_slot;
        let mut var_dqcoth_dn7: f64 = *var_dqcoth_dn7_slot;
        let mut var_dqcoth_dn8: f64 = *var_dqcoth_dn8_slot;
        let mut var_dqcothqdqsqrt: f64 = *var_dqcothqdqsqrt_slot;
        let mut var_dqcothqdqsqrt_dn3: f64 = *var_dqcothqdqsqrt_dn3_slot;
        let mut var_dqcothqdqsqrt_dn4: f64 = *var_dqcothqdqsqrt_dn4_slot;
        let mut var_dqcothqdqsqrt_dn5: f64 = *var_dqcothqdqsqrt_dn5_slot;
        let mut var_dqcothqdqsqrt_dn6: f64 = *var_dqcothqdqsqrt_dn6_slot;
        let mut var_dqcothqdqsqrt_dn7: f64 = *var_dqcothqdqsqrt_dn7_slot;
        let mut var_dqcothqdqsqrt_dn8: f64 = *var_dqcothqdqsqrt_dn8_slot;
        let mut var_dqsqrt: f64 = *var_dqsqrt_slot;
        let mut var_dqsqrt_dn3: f64 = *var_dqsqrt_dn3_slot;
        let mut var_dqsqrt_dn4: f64 = *var_dqsqrt_dn4_slot;
        let mut var_dqsqrt_dn5: f64 = *var_dqsqrt_dn5_slot;
        let mut var_dqsqrt_dn6: f64 = *var_dqsqrt_dn6_slot;
        let mut var_dqsqrt_dn7: f64 = *var_dqsqrt_dn7_slot;
        let mut var_dqsqrt_dn8: f64 = *var_dqsqrt_dn8_slot;
        let mut var_f: f64 = *var_f_slot;
        let mut var_f_dn3: f64 = *var_f_dn3_slot;
        let mut var_f_dn4: f64 = *var_f_dn4_slot;
        let mut var_f_dn5: f64 = *var_f_dn5_slot;
        let mut var_f_dn6: f64 = *var_f_dn6_slot;
        let mut var_f_dn7: f64 = *var_f_dn7_slot;
        let mut var_f_dn8: f64 = *var_f_dn8_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;

        let (assign5760_e6284, assign5760_e6284_d_n3, assign5760_e6284_d_n4, assign5760_e6284_d_n5, assign5760_e6284_d_n6, assign5760_e6284_d_n7, assign5760_e6284_d_n8,) = {
    if (var_guard77 != 0.0) {
        let assign5760_e6282: f64 = (var_csc1 * var_csc1);
        (assign5760_e6282, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign5760_e6284;
        var_t1_dn3 = assign5760_e6284_d_n3;
        var_t1_dn4 = assign5760_e6284_d_n4;
        var_t1_dn5 = assign5760_e6284_d_n5;
        var_t1_dn6 = assign5760_e6284_d_n6;
        var_t1_dn7 = assign5760_e6284_d_n7;
        var_t1_dn8 = assign5760_e6284_d_n8;

        let (assign5770_e6293, assign5770_e6293_d_n3, assign5770_e6293_d_n4, assign5770_e6293_d_n5, assign5770_e6293_d_n6, assign5770_e6293_d_n7, assign5770_e6293_d_n8,) = {
    if (var_guard77 != 0.0) {
        let assign5770_e6288: f64 = (0.5 * var_q);
        let assign5770_e6289: f64 = (assign5770_e6288).cos();
        let assign5770_e6291: f64 = (assign5770_e6289 * var_csc1);
        (assign5770_e6291, (((-(assign5770_e6288).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign5770_e6289 * var_csc1_dn3)), (((-(assign5770_e6288).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign5770_e6289 * var_csc1_dn4)), (((-(assign5770_e6288).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign5770_e6289 * var_csc1_dn5)), (((-(assign5770_e6288).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign5770_e6289 * var_csc1_dn6)), (((-(assign5770_e6288).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign5770_e6289 * var_csc1_dn7)), (((-(assign5770_e6288).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign5770_e6289 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign5770_e6293;
        var_coth1_dn3 = assign5770_e6293_d_n3;
        var_coth1_dn4 = assign5770_e6293_d_n4;
        var_coth1_dn5 = assign5770_e6293_d_n5;
        var_coth1_dn6 = assign5770_e6293_d_n6;
        var_coth1_dn7 = assign5770_e6293_d_n7;
        var_coth1_dn8 = assign5770_e6293_d_n8;

        let (assign5780_e6302, assign5780_e6302_d_n3, assign5780_e6302_d_n4, assign5780_e6302_d_n5, assign5780_e6302_d_n6, assign5780_e6302_d_n7, assign5780_e6302_d_n8,) = {
    if (var_guard77 != 0.0) {
        let assign5780_e6296: f64 = (-0.5);
        let assign5780_e6298: f64 = (assign5780_e6296 * var_coth1);
        let assign5780_e6300: f64 = (assign5780_e6298 / var_q);
        (assign5780_e6300, ((((assign5780_e6296 * var_coth1_dn3) * var_q) - (assign5780_e6298 * var_q_dn3)) / (var_q * var_q)), ((((assign5780_e6296 * var_coth1_dn4) * var_q) - (assign5780_e6298 * var_q_dn4)) / (var_q * var_q)), ((((assign5780_e6296 * var_coth1_dn5) * var_q) - (assign5780_e6298 * var_q_dn5)) / (var_q * var_q)), ((((assign5780_e6296 * var_coth1_dn6) * var_q) - (assign5780_e6298 * var_q_dn6)) / (var_q * var_q)), ((((assign5780_e6296 * var_coth1_dn7) * var_q) - (assign5780_e6298 * var_q_dn7)) / (var_q * var_q)), ((((assign5780_e6296 * var_coth1_dn8) * var_q) - (assign5780_e6298 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign5780_e6302;
        var_t0_dn3 = assign5780_e6302_d_n3;
        var_t0_dn4 = assign5780_e6302_d_n4;
        var_t0_dn5 = assign5780_e6302_d_n5;
        var_t0_dn6 = assign5780_e6302_d_n6;
        var_t0_dn7 = assign5780_e6302_d_n7;
        var_t0_dn8 = assign5780_e6302_d_n8;

        let (assign5790_e6310, assign5790_e6310_d_n3, assign5790_e6310_d_n4, assign5790_e6310_d_n5, assign5790_e6310_d_n6, assign5790_e6310_d_n7, assign5790_e6310_d_n8,) = {
    if (var_guard77 != 0.0) {
        let assign5790_e6306: f64 = (0.25 * var_t1);
        let assign5790_e6308: f64 = (assign5790_e6306 + var_t0);
        (assign5790_e6308, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign5790_e6310;
        var_dqcothqdqsqrt_dn3 = assign5790_e6310_d_n3;
        var_dqcothqdqsqrt_dn4 = assign5790_e6310_d_n4;
        var_dqcothqdqsqrt_dn5 = assign5790_e6310_d_n5;
        var_dqcothqdqsqrt_dn6 = assign5790_e6310_d_n6;
        var_dqcothqdqsqrt_dn7 = assign5790_e6310_d_n7;
        var_dqcothqdqsqrt_dn8 = assign5790_e6310_d_n8;

        let (assign5800_e6316, assign5800_e6316_d_n3, assign5800_e6316_d_n4, assign5800_e6316_d_n5, assign5800_e6316_d_n6, assign5800_e6316_d_n7, assign5800_e6316_d_n8,) = {
    if (var_guard77 == 0.0) {
        let assign5800_e6314: f64 = (var_qsqrt).sqrt();
        (assign5800_e6314, (var_qsqrt_dn3 / (2.0 * assign5800_e6314)), (var_qsqrt_dn4 / (2.0 * assign5800_e6314)), (var_qsqrt_dn5 / (2.0 * assign5800_e6314)), (var_qsqrt_dn6 / (2.0 * assign5800_e6314)), (var_qsqrt_dn7 / (2.0 * assign5800_e6314)), (var_qsqrt_dn8 / (2.0 * assign5800_e6314)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign5800_e6316;
        var_q_dn3 = assign5800_e6316_d_n3;
        var_q_dn4 = assign5800_e6316_d_n4;
        var_q_dn5 = assign5800_e6316_d_n5;
        var_q_dn6 = assign5800_e6316_d_n6;
        var_q_dn7 = assign5800_e6316_d_n7;
        var_q_dn8 = assign5800_e6316_d_n8;

        let (assign5810_e6326, assign5810_e6326_d_n3, assign5810_e6326_d_n4, assign5810_e6326_d_n5, assign5810_e6326_d_n6, assign5810_e6326_d_n7, assign5810_e6326_d_n8,) = {
    if (var_guard77 == 0.0) {
        let assign5810_e6322: f64 = (0.5 * var_q);
        let assign5810_e6323: f64 = (assign5810_e6322).sinh();
        let assign5810_e6324: f64 = (1.0 / assign5810_e6323);
        (assign5810_e6324, (-(((assign5810_e6322).cosh() * (0.5 * var_q_dn3)) / (assign5810_e6323 * assign5810_e6323))), (-(((assign5810_e6322).cosh() * (0.5 * var_q_dn4)) / (assign5810_e6323 * assign5810_e6323))), (-(((assign5810_e6322).cosh() * (0.5 * var_q_dn5)) / (assign5810_e6323 * assign5810_e6323))), (-(((assign5810_e6322).cosh() * (0.5 * var_q_dn6)) / (assign5810_e6323 * assign5810_e6323))), (-(((assign5810_e6322).cosh() * (0.5 * var_q_dn7)) / (assign5810_e6323 * assign5810_e6323))), (-(((assign5810_e6322).cosh() * (0.5 * var_q_dn8)) / (assign5810_e6323 * assign5810_e6323))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign5810_e6326;
        var_csc1_dn3 = assign5810_e6326_d_n3;
        var_csc1_dn4 = assign5810_e6326_d_n4;
        var_csc1_dn5 = assign5810_e6326_d_n5;
        var_csc1_dn6 = assign5810_e6326_d_n6;
        var_csc1_dn7 = assign5810_e6326_d_n7;
        var_csc1_dn8 = assign5810_e6326_d_n8;

        let (assign5820_e6333, assign5820_e6333_d_n3, assign5820_e6333_d_n4, assign5820_e6333_d_n5, assign5820_e6333_d_n6, assign5820_e6333_d_n7, assign5820_e6333_d_n8,) = {
    if (var_guard77 == 0.0) {
        let assign5820_e6331: f64 = (var_csc1 * var_csc1);
        (assign5820_e6331, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign5820_e6333;
        var_t1_dn3 = assign5820_e6333_d_n3;
        var_t1_dn4 = assign5820_e6333_d_n4;
        var_t1_dn5 = assign5820_e6333_d_n5;
        var_t1_dn6 = assign5820_e6333_d_n6;
        var_t1_dn7 = assign5820_e6333_d_n7;
        var_t1_dn8 = assign5820_e6333_d_n8;

        let (assign5830_e6341, assign5830_e6341_d_n3, assign5830_e6341_d_n4, assign5830_e6341_d_n5, assign5830_e6341_d_n6, assign5830_e6341_d_n7, assign5830_e6341_d_n8,) = {
    if (var_guard77 == 0.0) {
        let assign5830_e6338: f64 = (1.0 + var_t1);
        let assign5830_e6339: f64 = (assign5830_e6338).sqrt();
        (assign5830_e6339, (var_t1_dn3 / (2.0 * assign5830_e6339)), (var_t1_dn4 / (2.0 * assign5830_e6339)), (var_t1_dn5 / (2.0 * assign5830_e6339)), (var_t1_dn6 / (2.0 * assign5830_e6339)), (var_t1_dn7 / (2.0 * assign5830_e6339)), (var_t1_dn8 / (2.0 * assign5830_e6339)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign5830_e6341;
        var_coth1_dn3 = assign5830_e6341_d_n3;
        var_coth1_dn4 = assign5830_e6341_d_n4;
        var_coth1_dn5 = assign5830_e6341_d_n5;
        var_coth1_dn6 = assign5830_e6341_d_n6;
        var_coth1_dn7 = assign5830_e6341_d_n7;
        var_coth1_dn8 = assign5830_e6341_d_n8;

        let (assign5840_e6350, assign5840_e6350_d_n3, assign5840_e6350_d_n4, assign5840_e6350_d_n5, assign5840_e6350_d_n6, assign5840_e6350_d_n7, assign5840_e6350_d_n8,) = {
    if (var_guard77 == 0.0) {
        let assign5840_e6346: f64 = (0.5 * var_coth1);
        let assign5840_e6348: f64 = (assign5840_e6346 / var_q);
        (assign5840_e6348, ((((0.5 * var_coth1_dn3) * var_q) - (assign5840_e6346 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign5840_e6346 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign5840_e6346 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign5840_e6346 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign5840_e6346 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign5840_e6346 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign5840_e6350;
        var_t0_dn3 = assign5840_e6350_d_n3;
        var_t0_dn4 = assign5840_e6350_d_n4;
        var_t0_dn5 = assign5840_e6350_d_n5;
        var_t0_dn6 = assign5840_e6350_d_n6;
        var_t0_dn7 = assign5840_e6350_d_n7;
        var_t0_dn8 = assign5840_e6350_d_n8;

        let (assign5850_e6360, assign5850_e6360_d_n3, assign5850_e6360_d_n4, assign5850_e6360_d_n5, assign5850_e6360_d_n6, assign5850_e6360_d_n7, assign5850_e6360_d_n8,) = {
    if (var_guard77 == 0.0) {
        let assign5850_e6354: f64 = (-0.25);
        let assign5850_e6356: f64 = (assign5850_e6354 * var_t1);
        let assign5850_e6358: f64 = (assign5850_e6356 + var_t0);
        (assign5850_e6358, ((assign5850_e6354 * var_t1_dn3) + var_t0_dn3), ((assign5850_e6354 * var_t1_dn4) + var_t0_dn4), ((assign5850_e6354 * var_t1_dn5) + var_t0_dn5), ((assign5850_e6354 * var_t1_dn6) + var_t0_dn6), ((assign5850_e6354 * var_t1_dn7) + var_t0_dn7), ((assign5850_e6354 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign5850_e6360;
        var_dqcothqdqsqrt_dn3 = assign5850_e6360_d_n3;
        var_dqcothqdqsqrt_dn4 = assign5850_e6360_d_n4;
        var_dqcothqdqsqrt_dn5 = assign5850_e6360_d_n5;
        var_dqcothqdqsqrt_dn6 = assign5850_e6360_d_n6;
        var_dqcothqdqsqrt_dn7 = assign5850_e6360_d_n7;
        var_dqcothqdqsqrt_dn8 = assign5850_e6360_d_n8;

        let assign5860_e6363: f64 = (var_q * var_coth1);
        var_qcoth = assign5860_e6363;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign5870_e6366: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign5870_e6366;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign5880_e6369: f64 = (1.0 / var_t2);
        var_t3 = assign5880_e6369;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign5890_e6372: f64 = (var_xg2 - var_xg1);
        let assign5890_e6374: f64 = (assign5890_e6372 + var_q1);
        let assign5890_e6377: f64 = (var_qsqrt * var_t1);
        let assign5890_e6379: f64 = (assign5890_e6377 * var_t3);
        let assign5890_e6381: f64 = (assign5890_e6379 * var_t3);
        let assign5890_e6382: f64 = (assign5890_e6381).abs();
        let assign5890_e6383: f64 = (assign5890_e6382).ln();
        let assign5890_e6384: f64 = (assign5890_e6374 - assign5890_e6383);
        var_q2 = assign5890_e6384;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign5890_e6381 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign5890_e6377 * var_t3_dn3)) * var_t3) + (assign5890_e6379 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign5890_e6377 * var_t3_dn3)) * var_t3) + (assign5890_e6379 * var_t3_dn3))) } / assign5890_e6382));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign5890_e6381 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign5890_e6377 * var_t3_dn4)) * var_t3) + (assign5890_e6379 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign5890_e6377 * var_t3_dn4)) * var_t3) + (assign5890_e6379 * var_t3_dn4))) } / assign5890_e6382));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign5890_e6381 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign5890_e6377 * var_t3_dn5)) * var_t3) + (assign5890_e6379 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign5890_e6377 * var_t3_dn5)) * var_t3) + (assign5890_e6379 * var_t3_dn5))) } / assign5890_e6382));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign5890_e6381 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign5890_e6377 * var_t3_dn6)) * var_t3) + (assign5890_e6379 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign5890_e6377 * var_t3_dn6)) * var_t3) + (assign5890_e6379 * var_t3_dn6))) } / assign5890_e6382));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign5890_e6381 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign5890_e6377 * var_t3_dn7)) * var_t3) + (assign5890_e6379 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign5890_e6377 * var_t3_dn7)) * var_t3) + (assign5890_e6379 * var_t3_dn7))) } / assign5890_e6382));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign5890_e6381 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign5890_e6377 * var_t3_dn8)) * var_t3) + (assign5890_e6379 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign5890_e6377 * var_t3_dn8)) * var_t3) + (assign5890_e6379 * var_t3_dn8))) } / assign5890_e6382));

        let assign5900_e6388: f64 = (var_auxb1 + var_qcoth);
        let assign5900_e6391: f64 = (var_k2 * var_q2);
        let assign5900_e6393: f64 = (assign5900_e6391 + var_auxb1);
        let assign5900_e6394: f64 = (assign5900_e6388 * assign5900_e6393);
        let assign5900_e6395: f64 = (var_aaux + assign5900_e6394);
        var_f = assign5900_e6395;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign5900_e6393) + (assign5900_e6388 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign5900_e6393) + (assign5900_e6388 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign5900_e6393) + (assign5900_e6388 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign5900_e6393) + (assign5900_e6388 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign5900_e6393) + (assign5900_e6388 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign5900_e6393) + (assign5900_e6388 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign5910_e6398: f64 = (1.0 / var_qsqrt);
        let assign5910_e6400: f64 = (assign5910_e6398 - var_t0);
        var_dlogsinhqsqdqsqrt = assign5910_e6400;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign5920_e6402: f64 = (-2.0);
        let assign5920_e6404: f64 = (assign5920_e6402 * var_k1);
        let assign5920_e6406: f64 = (assign5920_e6404 * var_auxb1);
        let assign5920_e6408: f64 = (assign5920_e6406 + var_aaux);
        var_dqsqrt = assign5920_e6408;
        var_dqsqrt_dn3 = ((assign5920_e6404 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign5920_e6404 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign5920_e6404 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign5920_e6404 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign5920_e6404 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign5920_e6404 * var_auxb1_dn8) + var_aaux_dn8);

        let assign5930_e6411: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign5930_e6411;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign5940_e6413: f64 = (-1.0);
        let assign5940_e6416: f64 = (-var_k1);
        let assign5940_e6418: f64 = (assign5940_e6416 + var_dqcoth);
        let assign5940_e6420: f64 = (assign5940_e6418 * var_t3);
        let assign5940_e6421: f64 = (2.0 * assign5940_e6420);
        let assign5940_e6422: f64 = (assign5940_e6413 + assign5940_e6421);
        let assign5940_e6425: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign5940_e6426: f64 = (assign5940_e6422 - assign5940_e6425);
        var_dq2 = assign5940_e6426;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign5940_e6418 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign5940_e6418 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign5940_e6418 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign5940_e6418 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign5940_e6418 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign5940_e6418 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign5950_e6431: f64 = (var_auxb1 + var_t2);
        let assign5950_e6432: f64 = (var_k1 * assign5950_e6431);
        let assign5950_e6433: f64 = (var_aaux - assign5950_e6432);
        let assign5950_e6436: f64 = (var_auxb1 * var_dqcoth);
        let assign5950_e6437: f64 = (assign5950_e6433 + assign5950_e6436);
        let assign5950_e6441: f64 = (var_dq2 * var_t2);
        let assign5950_e6445: f64 = (var_dqcoth - var_k1);
        let assign5950_e6446: f64 = (var_q2 * assign5950_e6445);
        let assign5950_e6447: f64 = (assign5950_e6441 + assign5950_e6446);
        let assign5950_e6448: f64 = (var_k2 * assign5950_e6447);
        let assign5950_e6449: f64 = (assign5950_e6437 + assign5950_e6448);
        var_df = assign5950_e6449;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign5950_e6445) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign5950_e6445) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign5950_e6445) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign5950_e6445) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign5950_e6445) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign5950_e6445) + (var_q2 * var_dqcoth_dn8)))));

        let assign5960_e6451: f64 = (-var_f);
        let assign5960_e6453: f64 = (assign5960_e6451 / var_df);
        var_delta = assign5960_e6453;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign5960_e6451 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign5960_e6451 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign5960_e6451 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign5960_e6451 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign5960_e6451 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign5960_e6451 * var_df_dn8)) / (var_df * var_df));

        let assign5970_e6456: f64 = (var_phi1 + var_delta);
        var_phi1 = assign5970_e6456;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign5980_e6459: f64 = (var_xg1 - var_phi1);
        var_q1 = assign5980_e6459;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign5990_e6462: f64 = (var_k1 * var_q1);
        var_auxb1 = assign5990_e6462;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign6000_e6464: f64 = (-var_a0);
        let assign6000_e6466: f64 = (var_phi1).exp();
        let assign6000_e6467: f64 = (assign6000_e6464 * assign6000_e6466);
        var_aaux = assign6000_e6467;
        var_aaux_dn3 = (((-var_a0_dn3) * assign6000_e6466) + (assign6000_e6464 * (assign6000_e6466 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign6000_e6466) + (assign6000_e6464 * (assign6000_e6466 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign6000_e6466) + (assign6000_e6464 * (assign6000_e6466 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign6000_e6466) + (assign6000_e6464 * (assign6000_e6466 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign6000_e6466) + (assign6000_e6464 * (assign6000_e6466 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign6000_e6466) + (assign6000_e6464 * (assign6000_e6466 * var_phi1_dn8)));

        let assign6010_e6470: f64 = (var_auxb1 * var_auxb1);
        let assign6010_e6472: f64 = (assign6010_e6470 + var_aaux);
        var_qsqrt = assign6010_e6472;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign6020_e6475: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard78 = assign6020_e6475;

        let (assign6030_e6481, assign6030_e6481_d_n3, assign6030_e6481_d_n4, assign6030_e6481_d_n5, assign6030_e6481_d_n6, assign6030_e6481_d_n7, assign6030_e6481_d_n8,) = {
    if (var_guard78 != 0.0) {
        let assign6030_e6478: f64 = (-var_qsqrt);
        let assign6030_e6479: f64 = (assign6030_e6478).sqrt();
        (assign6030_e6479, ((-var_qsqrt_dn3) / (2.0 * assign6030_e6479)), ((-var_qsqrt_dn4) / (2.0 * assign6030_e6479)), ((-var_qsqrt_dn5) / (2.0 * assign6030_e6479)), ((-var_qsqrt_dn6) / (2.0 * assign6030_e6479)), ((-var_qsqrt_dn7) / (2.0 * assign6030_e6479)), ((-var_qsqrt_dn8) / (2.0 * assign6030_e6479)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign6030_e6481;
        var_q_dn3 = assign6030_e6481_d_n3;
        var_q_dn4 = assign6030_e6481_d_n4;
        var_q_dn5 = assign6030_e6481_d_n5;
        var_q_dn6 = assign6030_e6481_d_n6;
        var_q_dn7 = assign6030_e6481_d_n7;
        var_q_dn8 = assign6030_e6481_d_n8;

        let (assign6040_e6490, assign6040_e6490_d_n3, assign6040_e6490_d_n4, assign6040_e6490_d_n5, assign6040_e6490_d_n6, assign6040_e6490_d_n7, assign6040_e6490_d_n8,) = {
    if (var_guard78 != 0.0) {
        let assign6040_e6486: f64 = (0.5 * var_q);
        let assign6040_e6487: f64 = (assign6040_e6486).sin();
        let assign6040_e6488: f64 = (1.0 / assign6040_e6487);
        (assign6040_e6488, (-(((assign6040_e6486).cos() * (0.5 * var_q_dn3)) / (assign6040_e6487 * assign6040_e6487))), (-(((assign6040_e6486).cos() * (0.5 * var_q_dn4)) / (assign6040_e6487 * assign6040_e6487))), (-(((assign6040_e6486).cos() * (0.5 * var_q_dn5)) / (assign6040_e6487 * assign6040_e6487))), (-(((assign6040_e6486).cos() * (0.5 * var_q_dn6)) / (assign6040_e6487 * assign6040_e6487))), (-(((assign6040_e6486).cos() * (0.5 * var_q_dn7)) / (assign6040_e6487 * assign6040_e6487))), (-(((assign6040_e6486).cos() * (0.5 * var_q_dn8)) / (assign6040_e6487 * assign6040_e6487))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign6040_e6490;
        var_csc1_dn3 = assign6040_e6490_d_n3;
        var_csc1_dn4 = assign6040_e6490_d_n4;
        var_csc1_dn5 = assign6040_e6490_d_n5;
        var_csc1_dn6 = assign6040_e6490_d_n6;
        var_csc1_dn7 = assign6040_e6490_d_n7;
        var_csc1_dn8 = assign6040_e6490_d_n8;

        let (assign6050_e6496, assign6050_e6496_d_n3, assign6050_e6496_d_n4, assign6050_e6496_d_n5, assign6050_e6496_d_n6, assign6050_e6496_d_n7, assign6050_e6496_d_n8,) = {
    if (var_guard78 != 0.0) {
        let assign6050_e6494: f64 = (var_csc1 * var_csc1);
        (assign6050_e6494, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign6050_e6496;
        var_t1_dn3 = assign6050_e6496_d_n3;
        var_t1_dn4 = assign6050_e6496_d_n4;
        var_t1_dn5 = assign6050_e6496_d_n5;
        var_t1_dn6 = assign6050_e6496_d_n6;
        var_t1_dn7 = assign6050_e6496_d_n7;
        var_t1_dn8 = assign6050_e6496_d_n8;

        let (assign6060_e6505, assign6060_e6505_d_n3, assign6060_e6505_d_n4, assign6060_e6505_d_n5, assign6060_e6505_d_n6, assign6060_e6505_d_n7, assign6060_e6505_d_n8,) = {
    if (var_guard78 != 0.0) {
        let assign6060_e6500: f64 = (0.5 * var_q);
        let assign6060_e6501: f64 = (assign6060_e6500).cos();
        let assign6060_e6503: f64 = (assign6060_e6501 * var_csc1);
        (assign6060_e6503, (((-(assign6060_e6500).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign6060_e6501 * var_csc1_dn3)), (((-(assign6060_e6500).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign6060_e6501 * var_csc1_dn4)), (((-(assign6060_e6500).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign6060_e6501 * var_csc1_dn5)), (((-(assign6060_e6500).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign6060_e6501 * var_csc1_dn6)), (((-(assign6060_e6500).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign6060_e6501 * var_csc1_dn7)), (((-(assign6060_e6500).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign6060_e6501 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign6060_e6505;
        var_coth1_dn3 = assign6060_e6505_d_n3;
        var_coth1_dn4 = assign6060_e6505_d_n4;
        var_coth1_dn5 = assign6060_e6505_d_n5;
        var_coth1_dn6 = assign6060_e6505_d_n6;
        var_coth1_dn7 = assign6060_e6505_d_n7;
        var_coth1_dn8 = assign6060_e6505_d_n8;

        let (assign6070_e6514, assign6070_e6514_d_n3, assign6070_e6514_d_n4, assign6070_e6514_d_n5, assign6070_e6514_d_n6, assign6070_e6514_d_n7, assign6070_e6514_d_n8,) = {
    if (var_guard78 != 0.0) {
        let assign6070_e6508: f64 = (-0.5);
        let assign6070_e6510: f64 = (assign6070_e6508 * var_coth1);
        let assign6070_e6512: f64 = (assign6070_e6510 / var_q);
        (assign6070_e6512, ((((assign6070_e6508 * var_coth1_dn3) * var_q) - (assign6070_e6510 * var_q_dn3)) / (var_q * var_q)), ((((assign6070_e6508 * var_coth1_dn4) * var_q) - (assign6070_e6510 * var_q_dn4)) / (var_q * var_q)), ((((assign6070_e6508 * var_coth1_dn5) * var_q) - (assign6070_e6510 * var_q_dn5)) / (var_q * var_q)), ((((assign6070_e6508 * var_coth1_dn6) * var_q) - (assign6070_e6510 * var_q_dn6)) / (var_q * var_q)), ((((assign6070_e6508 * var_coth1_dn7) * var_q) - (assign6070_e6510 * var_q_dn7)) / (var_q * var_q)), ((((assign6070_e6508 * var_coth1_dn8) * var_q) - (assign6070_e6510 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign6070_e6514;
        var_t0_dn3 = assign6070_e6514_d_n3;
        var_t0_dn4 = assign6070_e6514_d_n4;
        var_t0_dn5 = assign6070_e6514_d_n5;
        var_t0_dn6 = assign6070_e6514_d_n6;
        var_t0_dn7 = assign6070_e6514_d_n7;
        var_t0_dn8 = assign6070_e6514_d_n8;

        let (assign6080_e6522, assign6080_e6522_d_n3, assign6080_e6522_d_n4, assign6080_e6522_d_n5, assign6080_e6522_d_n6, assign6080_e6522_d_n7, assign6080_e6522_d_n8,) = {
    if (var_guard78 != 0.0) {
        let assign6080_e6518: f64 = (0.25 * var_t1);
        let assign6080_e6520: f64 = (assign6080_e6518 + var_t0);
        (assign6080_e6520, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign6080_e6522;
        var_dqcothqdqsqrt_dn3 = assign6080_e6522_d_n3;
        var_dqcothqdqsqrt_dn4 = assign6080_e6522_d_n4;
        var_dqcothqdqsqrt_dn5 = assign6080_e6522_d_n5;
        var_dqcothqdqsqrt_dn6 = assign6080_e6522_d_n6;
        var_dqcothqdqsqrt_dn7 = assign6080_e6522_d_n7;
        var_dqcothqdqsqrt_dn8 = assign6080_e6522_d_n8;

        let (assign6090_e6528, assign6090_e6528_d_n3, assign6090_e6528_d_n4, assign6090_e6528_d_n5, assign6090_e6528_d_n6, assign6090_e6528_d_n7, assign6090_e6528_d_n8,) = {
    if (var_guard78 == 0.0) {
        let assign6090_e6526: f64 = (var_qsqrt).sqrt();
        (assign6090_e6526, (var_qsqrt_dn3 / (2.0 * assign6090_e6526)), (var_qsqrt_dn4 / (2.0 * assign6090_e6526)), (var_qsqrt_dn5 / (2.0 * assign6090_e6526)), (var_qsqrt_dn6 / (2.0 * assign6090_e6526)), (var_qsqrt_dn7 / (2.0 * assign6090_e6526)), (var_qsqrt_dn8 / (2.0 * assign6090_e6526)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign6090_e6528;
        var_q_dn3 = assign6090_e6528_d_n3;
        var_q_dn4 = assign6090_e6528_d_n4;
        var_q_dn5 = assign6090_e6528_d_n5;
        var_q_dn6 = assign6090_e6528_d_n6;
        var_q_dn7 = assign6090_e6528_d_n7;
        var_q_dn8 = assign6090_e6528_d_n8;

        let (assign6100_e6538, assign6100_e6538_d_n3, assign6100_e6538_d_n4, assign6100_e6538_d_n5, assign6100_e6538_d_n6, assign6100_e6538_d_n7, assign6100_e6538_d_n8,) = {
    if (var_guard78 == 0.0) {
        let assign6100_e6534: f64 = (0.5 * var_q);
        let assign6100_e6535: f64 = (assign6100_e6534).sinh();
        let assign6100_e6536: f64 = (1.0 / assign6100_e6535);
        (assign6100_e6536, (-(((assign6100_e6534).cosh() * (0.5 * var_q_dn3)) / (assign6100_e6535 * assign6100_e6535))), (-(((assign6100_e6534).cosh() * (0.5 * var_q_dn4)) / (assign6100_e6535 * assign6100_e6535))), (-(((assign6100_e6534).cosh() * (0.5 * var_q_dn5)) / (assign6100_e6535 * assign6100_e6535))), (-(((assign6100_e6534).cosh() * (0.5 * var_q_dn6)) / (assign6100_e6535 * assign6100_e6535))), (-(((assign6100_e6534).cosh() * (0.5 * var_q_dn7)) / (assign6100_e6535 * assign6100_e6535))), (-(((assign6100_e6534).cosh() * (0.5 * var_q_dn8)) / (assign6100_e6535 * assign6100_e6535))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign6100_e6538;
        var_csc1_dn3 = assign6100_e6538_d_n3;
        var_csc1_dn4 = assign6100_e6538_d_n4;
        var_csc1_dn5 = assign6100_e6538_d_n5;
        var_csc1_dn6 = assign6100_e6538_d_n6;
        var_csc1_dn7 = assign6100_e6538_d_n7;
        var_csc1_dn8 = assign6100_e6538_d_n8;

        let (assign6110_e6545, assign6110_e6545_d_n3, assign6110_e6545_d_n4, assign6110_e6545_d_n5, assign6110_e6545_d_n6, assign6110_e6545_d_n7, assign6110_e6545_d_n8,) = {
    if (var_guard78 == 0.0) {
        let assign6110_e6543: f64 = (var_csc1 * var_csc1);
        (assign6110_e6543, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign6110_e6545;
        var_t1_dn3 = assign6110_e6545_d_n3;
        var_t1_dn4 = assign6110_e6545_d_n4;
        var_t1_dn5 = assign6110_e6545_d_n5;
        var_t1_dn6 = assign6110_e6545_d_n6;
        var_t1_dn7 = assign6110_e6545_d_n7;
        var_t1_dn8 = assign6110_e6545_d_n8;

        *var_aaux_slot = var_aaux;
        *var_aaux_dn3_slot = var_aaux_dn3;
        *var_aaux_dn4_slot = var_aaux_dn4;
        *var_aaux_dn5_slot = var_aaux_dn5;
        *var_aaux_dn6_slot = var_aaux_dn6;
        *var_aaux_dn7_slot = var_aaux_dn7;
        *var_aaux_dn8_slot = var_aaux_dn8;
        *var_auxb1_slot = var_auxb1;
        *var_auxb1_dn3_slot = var_auxb1_dn3;
        *var_auxb1_dn4_slot = var_auxb1_dn4;
        *var_auxb1_dn5_slot = var_auxb1_dn5;
        *var_auxb1_dn6_slot = var_auxb1_dn6;
        *var_auxb1_dn7_slot = var_auxb1_dn7;
        *var_auxb1_dn8_slot = var_auxb1_dn8;
        *var_coth1_slot = var_coth1;
        *var_coth1_dn3_slot = var_coth1_dn3;
        *var_coth1_dn4_slot = var_coth1_dn4;
        *var_coth1_dn5_slot = var_coth1_dn5;
        *var_coth1_dn6_slot = var_coth1_dn6;
        *var_coth1_dn7_slot = var_coth1_dn7;
        *var_coth1_dn8_slot = var_coth1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_df_slot = var_df;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_dn6_slot = var_df_dn6;
        *var_df_dn7_slot = var_df_dn7;
        *var_df_dn8_slot = var_df_dn8;
        *var_dlogsinhqsqdqsqrt_slot = var_dlogsinhqsqdqsqrt;
        *var_dlogsinhqsqdqsqrt_dn3_slot = var_dlogsinhqsqdqsqrt_dn3;
        *var_dlogsinhqsqdqsqrt_dn4_slot = var_dlogsinhqsqdqsqrt_dn4;
        *var_dlogsinhqsqdqsqrt_dn5_slot = var_dlogsinhqsqdqsqrt_dn5;
        *var_dlogsinhqsqdqsqrt_dn6_slot = var_dlogsinhqsqdqsqrt_dn6;
        *var_dlogsinhqsqdqsqrt_dn7_slot = var_dlogsinhqsqdqsqrt_dn7;
        *var_dlogsinhqsqdqsqrt_dn8_slot = var_dlogsinhqsqdqsqrt_dn8;
        *var_dq2_slot = var_dq2;
        *var_dq2_dn3_slot = var_dq2_dn3;
        *var_dq2_dn4_slot = var_dq2_dn4;
        *var_dq2_dn5_slot = var_dq2_dn5;
        *var_dq2_dn6_slot = var_dq2_dn6;
        *var_dq2_dn7_slot = var_dq2_dn7;
        *var_dq2_dn8_slot = var_dq2_dn8;
        *var_dqcoth_slot = var_dqcoth;
        *var_dqcoth_dn3_slot = var_dqcoth_dn3;
        *var_dqcoth_dn4_slot = var_dqcoth_dn4;
        *var_dqcoth_dn5_slot = var_dqcoth_dn5;
        *var_dqcoth_dn6_slot = var_dqcoth_dn6;
        *var_dqcoth_dn7_slot = var_dqcoth_dn7;
        *var_dqcoth_dn8_slot = var_dqcoth_dn8;
        *var_dqcothqdqsqrt_slot = var_dqcothqdqsqrt;
        *var_dqcothqdqsqrt_dn3_slot = var_dqcothqdqsqrt_dn3;
        *var_dqcothqdqsqrt_dn4_slot = var_dqcothqdqsqrt_dn4;
        *var_dqcothqdqsqrt_dn5_slot = var_dqcothqdqsqrt_dn5;
        *var_dqcothqdqsqrt_dn6_slot = var_dqcothqdqsqrt_dn6;
        *var_dqcothqdqsqrt_dn7_slot = var_dqcothqdqsqrt_dn7;
        *var_dqcothqdqsqrt_dn8_slot = var_dqcothqdqsqrt_dn8;
        *var_dqsqrt_slot = var_dqsqrt;
        *var_dqsqrt_dn3_slot = var_dqsqrt_dn3;
        *var_dqsqrt_dn4_slot = var_dqsqrt_dn4;
        *var_dqsqrt_dn5_slot = var_dqsqrt_dn5;
        *var_dqsqrt_dn6_slot = var_dqsqrt_dn6;
        *var_dqsqrt_dn7_slot = var_dqsqrt_dn7;
        *var_dqsqrt_dn8_slot = var_dqsqrt_dn8;
        *var_f_slot = var_f;
        *var_f_dn3_slot = var_f_dn3;
        *var_f_dn4_slot = var_f_dn4;
        *var_f_dn5_slot = var_f_dn5;
        *var_f_dn6_slot = var_f_dn6;
        *var_f_dn7_slot = var_f_dn7;
        *var_f_dn8_slot = var_f_dn8;
        *var_guard78_slot = var_guard78;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
    }

    pub(super) fn stamp_transient_block_11(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_guard78: f64,
        var_k1: f64,
        var_k2: f64,
        var_xg1: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_aaux_slot: &mut f64,
        var_aaux_dn3_slot: &mut f64,
        var_aaux_dn4_slot: &mut f64,
        var_aaux_dn5_slot: &mut f64,
        var_aaux_dn6_slot: &mut f64,
        var_aaux_dn7_slot: &mut f64,
        var_aaux_dn8_slot: &mut f64,
        var_auxb1_slot: &mut f64,
        var_auxb1_dn3_slot: &mut f64,
        var_auxb1_dn4_slot: &mut f64,
        var_auxb1_dn5_slot: &mut f64,
        var_auxb1_dn6_slot: &mut f64,
        var_auxb1_dn7_slot: &mut f64,
        var_auxb1_dn8_slot: &mut f64,
        var_coth1_slot: &mut f64,
        var_coth1_dn3_slot: &mut f64,
        var_coth1_dn4_slot: &mut f64,
        var_coth1_dn5_slot: &mut f64,
        var_coth1_dn6_slot: &mut f64,
        var_coth1_dn7_slot: &mut f64,
        var_coth1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_df_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_dn6_slot: &mut f64,
        var_df_dn7_slot: &mut f64,
        var_df_dn8_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn3_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn4_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn5_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn6_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn7_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn8_slot: &mut f64,
        var_dq2_slot: &mut f64,
        var_dq2_dn3_slot: &mut f64,
        var_dq2_dn4_slot: &mut f64,
        var_dq2_dn5_slot: &mut f64,
        var_dq2_dn6_slot: &mut f64,
        var_dq2_dn7_slot: &mut f64,
        var_dq2_dn8_slot: &mut f64,
        var_dqcoth_slot: &mut f64,
        var_dqcoth_dn3_slot: &mut f64,
        var_dqcoth_dn4_slot: &mut f64,
        var_dqcoth_dn5_slot: &mut f64,
        var_dqcoth_dn6_slot: &mut f64,
        var_dqcoth_dn7_slot: &mut f64,
        var_dqcoth_dn8_slot: &mut f64,
        var_dqcothqdqsqrt_slot: &mut f64,
        var_dqcothqdqsqrt_dn3_slot: &mut f64,
        var_dqcothqdqsqrt_dn4_slot: &mut f64,
        var_dqcothqdqsqrt_dn5_slot: &mut f64,
        var_dqcothqdqsqrt_dn6_slot: &mut f64,
        var_dqcothqdqsqrt_dn7_slot: &mut f64,
        var_dqcothqdqsqrt_dn8_slot: &mut f64,
        var_dqsqrt_slot: &mut f64,
        var_dqsqrt_dn3_slot: &mut f64,
        var_dqsqrt_dn4_slot: &mut f64,
        var_dqsqrt_dn5_slot: &mut f64,
        var_dqsqrt_dn6_slot: &mut f64,
        var_dqsqrt_dn7_slot: &mut f64,
        var_dqsqrt_dn8_slot: &mut f64,
        var_f_slot: &mut f64,
        var_f_dn3_slot: &mut f64,
        var_f_dn4_slot: &mut f64,
        var_f_dn5_slot: &mut f64,
        var_f_dn6_slot: &mut f64,
        var_f_dn7_slot: &mut f64,
        var_f_dn8_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
    ) {
        let mut var_aaux: f64 = *var_aaux_slot;
        let mut var_aaux_dn3: f64 = *var_aaux_dn3_slot;
        let mut var_aaux_dn4: f64 = *var_aaux_dn4_slot;
        let mut var_aaux_dn5: f64 = *var_aaux_dn5_slot;
        let mut var_aaux_dn6: f64 = *var_aaux_dn6_slot;
        let mut var_aaux_dn7: f64 = *var_aaux_dn7_slot;
        let mut var_aaux_dn8: f64 = *var_aaux_dn8_slot;
        let mut var_auxb1: f64 = *var_auxb1_slot;
        let mut var_auxb1_dn3: f64 = *var_auxb1_dn3_slot;
        let mut var_auxb1_dn4: f64 = *var_auxb1_dn4_slot;
        let mut var_auxb1_dn5: f64 = *var_auxb1_dn5_slot;
        let mut var_auxb1_dn6: f64 = *var_auxb1_dn6_slot;
        let mut var_auxb1_dn7: f64 = *var_auxb1_dn7_slot;
        let mut var_auxb1_dn8: f64 = *var_auxb1_dn8_slot;
        let mut var_coth1: f64 = *var_coth1_slot;
        let mut var_coth1_dn3: f64 = *var_coth1_dn3_slot;
        let mut var_coth1_dn4: f64 = *var_coth1_dn4_slot;
        let mut var_coth1_dn5: f64 = *var_coth1_dn5_slot;
        let mut var_coth1_dn6: f64 = *var_coth1_dn6_slot;
        let mut var_coth1_dn7: f64 = *var_coth1_dn7_slot;
        let mut var_coth1_dn8: f64 = *var_coth1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_dn6: f64 = *var_df_dn6_slot;
        let mut var_df_dn7: f64 = *var_df_dn7_slot;
        let mut var_df_dn8: f64 = *var_df_dn8_slot;
        let mut var_dlogsinhqsqdqsqrt: f64 = *var_dlogsinhqsqdqsqrt_slot;
        let mut var_dlogsinhqsqdqsqrt_dn3: f64 = *var_dlogsinhqsqdqsqrt_dn3_slot;
        let mut var_dlogsinhqsqdqsqrt_dn4: f64 = *var_dlogsinhqsqdqsqrt_dn4_slot;
        let mut var_dlogsinhqsqdqsqrt_dn5: f64 = *var_dlogsinhqsqdqsqrt_dn5_slot;
        let mut var_dlogsinhqsqdqsqrt_dn6: f64 = *var_dlogsinhqsqdqsqrt_dn6_slot;
        let mut var_dlogsinhqsqdqsqrt_dn7: f64 = *var_dlogsinhqsqdqsqrt_dn7_slot;
        let mut var_dlogsinhqsqdqsqrt_dn8: f64 = *var_dlogsinhqsqdqsqrt_dn8_slot;
        let mut var_dq2: f64 = *var_dq2_slot;
        let mut var_dq2_dn3: f64 = *var_dq2_dn3_slot;
        let mut var_dq2_dn4: f64 = *var_dq2_dn4_slot;
        let mut var_dq2_dn5: f64 = *var_dq2_dn5_slot;
        let mut var_dq2_dn6: f64 = *var_dq2_dn6_slot;
        let mut var_dq2_dn7: f64 = *var_dq2_dn7_slot;
        let mut var_dq2_dn8: f64 = *var_dq2_dn8_slot;
        let mut var_dqcoth: f64 = *var_dqcoth_slot;
        let mut var_dqcoth_dn3: f64 = *var_dqcoth_dn3_slot;
        let mut var_dqcoth_dn4: f64 = *var_dqcoth_dn4_slot;
        let mut var_dqcoth_dn5: f64 = *var_dqcoth_dn5_slot;
        let mut var_dqcoth_dn6: f64 = *var_dqcoth_dn6_slot;
        let mut var_dqcoth_dn7: f64 = *var_dqcoth_dn7_slot;
        let mut var_dqcoth_dn8: f64 = *var_dqcoth_dn8_slot;
        let mut var_dqcothqdqsqrt: f64 = *var_dqcothqdqsqrt_slot;
        let mut var_dqcothqdqsqrt_dn3: f64 = *var_dqcothqdqsqrt_dn3_slot;
        let mut var_dqcothqdqsqrt_dn4: f64 = *var_dqcothqdqsqrt_dn4_slot;
        let mut var_dqcothqdqsqrt_dn5: f64 = *var_dqcothqdqsqrt_dn5_slot;
        let mut var_dqcothqdqsqrt_dn6: f64 = *var_dqcothqdqsqrt_dn6_slot;
        let mut var_dqcothqdqsqrt_dn7: f64 = *var_dqcothqdqsqrt_dn7_slot;
        let mut var_dqcothqdqsqrt_dn8: f64 = *var_dqcothqdqsqrt_dn8_slot;
        let mut var_dqsqrt: f64 = *var_dqsqrt_slot;
        let mut var_dqsqrt_dn3: f64 = *var_dqsqrt_dn3_slot;
        let mut var_dqsqrt_dn4: f64 = *var_dqsqrt_dn4_slot;
        let mut var_dqsqrt_dn5: f64 = *var_dqsqrt_dn5_slot;
        let mut var_dqsqrt_dn6: f64 = *var_dqsqrt_dn6_slot;
        let mut var_dqsqrt_dn7: f64 = *var_dqsqrt_dn7_slot;
        let mut var_dqsqrt_dn8: f64 = *var_dqsqrt_dn8_slot;
        let mut var_f: f64 = *var_f_slot;
        let mut var_f_dn3: f64 = *var_f_dn3_slot;
        let mut var_f_dn4: f64 = *var_f_dn4_slot;
        let mut var_f_dn5: f64 = *var_f_dn5_slot;
        let mut var_f_dn6: f64 = *var_f_dn6_slot;
        let mut var_f_dn7: f64 = *var_f_dn7_slot;
        let mut var_f_dn8: f64 = *var_f_dn8_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;

        let (assign6120_e6553, assign6120_e6553_d_n3, assign6120_e6553_d_n4, assign6120_e6553_d_n5, assign6120_e6553_d_n6, assign6120_e6553_d_n7, assign6120_e6553_d_n8,) = {
    if (var_guard78 == 0.0) {
        let assign6120_e6550: f64 = (1.0 + var_t1);
        let assign6120_e6551: f64 = (assign6120_e6550).sqrt();
        (assign6120_e6551, (var_t1_dn3 / (2.0 * assign6120_e6551)), (var_t1_dn4 / (2.0 * assign6120_e6551)), (var_t1_dn5 / (2.0 * assign6120_e6551)), (var_t1_dn6 / (2.0 * assign6120_e6551)), (var_t1_dn7 / (2.0 * assign6120_e6551)), (var_t1_dn8 / (2.0 * assign6120_e6551)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign6120_e6553;
        var_coth1_dn3 = assign6120_e6553_d_n3;
        var_coth1_dn4 = assign6120_e6553_d_n4;
        var_coth1_dn5 = assign6120_e6553_d_n5;
        var_coth1_dn6 = assign6120_e6553_d_n6;
        var_coth1_dn7 = assign6120_e6553_d_n7;
        var_coth1_dn8 = assign6120_e6553_d_n8;

        let (assign6130_e6562, assign6130_e6562_d_n3, assign6130_e6562_d_n4, assign6130_e6562_d_n5, assign6130_e6562_d_n6, assign6130_e6562_d_n7, assign6130_e6562_d_n8,) = {
    if (var_guard78 == 0.0) {
        let assign6130_e6558: f64 = (0.5 * var_coth1);
        let assign6130_e6560: f64 = (assign6130_e6558 / var_q);
        (assign6130_e6560, ((((0.5 * var_coth1_dn3) * var_q) - (assign6130_e6558 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign6130_e6558 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign6130_e6558 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign6130_e6558 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign6130_e6558 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign6130_e6558 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign6130_e6562;
        var_t0_dn3 = assign6130_e6562_d_n3;
        var_t0_dn4 = assign6130_e6562_d_n4;
        var_t0_dn5 = assign6130_e6562_d_n5;
        var_t0_dn6 = assign6130_e6562_d_n6;
        var_t0_dn7 = assign6130_e6562_d_n7;
        var_t0_dn8 = assign6130_e6562_d_n8;

        let (assign6140_e6572, assign6140_e6572_d_n3, assign6140_e6572_d_n4, assign6140_e6572_d_n5, assign6140_e6572_d_n6, assign6140_e6572_d_n7, assign6140_e6572_d_n8,) = {
    if (var_guard78 == 0.0) {
        let assign6140_e6566: f64 = (-0.25);
        let assign6140_e6568: f64 = (assign6140_e6566 * var_t1);
        let assign6140_e6570: f64 = (assign6140_e6568 + var_t0);
        (assign6140_e6570, ((assign6140_e6566 * var_t1_dn3) + var_t0_dn3), ((assign6140_e6566 * var_t1_dn4) + var_t0_dn4), ((assign6140_e6566 * var_t1_dn5) + var_t0_dn5), ((assign6140_e6566 * var_t1_dn6) + var_t0_dn6), ((assign6140_e6566 * var_t1_dn7) + var_t0_dn7), ((assign6140_e6566 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign6140_e6572;
        var_dqcothqdqsqrt_dn3 = assign6140_e6572_d_n3;
        var_dqcothqdqsqrt_dn4 = assign6140_e6572_d_n4;
        var_dqcothqdqsqrt_dn5 = assign6140_e6572_d_n5;
        var_dqcothqdqsqrt_dn6 = assign6140_e6572_d_n6;
        var_dqcothqdqsqrt_dn7 = assign6140_e6572_d_n7;
        var_dqcothqdqsqrt_dn8 = assign6140_e6572_d_n8;

        let assign6150_e6575: f64 = (var_q * var_coth1);
        var_qcoth = assign6150_e6575;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign6160_e6578: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign6160_e6578;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign6170_e6581: f64 = (1.0 / var_t2);
        var_t3 = assign6170_e6581;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign6180_e6584: f64 = (var_xg2 - var_xg1);
        let assign6180_e6586: f64 = (assign6180_e6584 + var_q1);
        let assign6180_e6589: f64 = (var_qsqrt * var_t1);
        let assign6180_e6591: f64 = (assign6180_e6589 * var_t3);
        let assign6180_e6593: f64 = (assign6180_e6591 * var_t3);
        let assign6180_e6594: f64 = (assign6180_e6593).abs();
        let assign6180_e6595: f64 = (assign6180_e6594).ln();
        let assign6180_e6596: f64 = (assign6180_e6586 - assign6180_e6595);
        var_q2 = assign6180_e6596;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign6180_e6593 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign6180_e6589 * var_t3_dn3)) * var_t3) + (assign6180_e6591 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign6180_e6589 * var_t3_dn3)) * var_t3) + (assign6180_e6591 * var_t3_dn3))) } / assign6180_e6594));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign6180_e6593 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign6180_e6589 * var_t3_dn4)) * var_t3) + (assign6180_e6591 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign6180_e6589 * var_t3_dn4)) * var_t3) + (assign6180_e6591 * var_t3_dn4))) } / assign6180_e6594));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign6180_e6593 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign6180_e6589 * var_t3_dn5)) * var_t3) + (assign6180_e6591 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign6180_e6589 * var_t3_dn5)) * var_t3) + (assign6180_e6591 * var_t3_dn5))) } / assign6180_e6594));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign6180_e6593 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign6180_e6589 * var_t3_dn6)) * var_t3) + (assign6180_e6591 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign6180_e6589 * var_t3_dn6)) * var_t3) + (assign6180_e6591 * var_t3_dn6))) } / assign6180_e6594));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign6180_e6593 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign6180_e6589 * var_t3_dn7)) * var_t3) + (assign6180_e6591 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign6180_e6589 * var_t3_dn7)) * var_t3) + (assign6180_e6591 * var_t3_dn7))) } / assign6180_e6594));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign6180_e6593 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign6180_e6589 * var_t3_dn8)) * var_t3) + (assign6180_e6591 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign6180_e6589 * var_t3_dn8)) * var_t3) + (assign6180_e6591 * var_t3_dn8))) } / assign6180_e6594));

        let assign6190_e6600: f64 = (var_auxb1 + var_qcoth);
        let assign6190_e6603: f64 = (var_k2 * var_q2);
        let assign6190_e6605: f64 = (assign6190_e6603 + var_auxb1);
        let assign6190_e6606: f64 = (assign6190_e6600 * assign6190_e6605);
        let assign6190_e6607: f64 = (var_aaux + assign6190_e6606);
        var_f = assign6190_e6607;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign6190_e6605) + (assign6190_e6600 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign6190_e6605) + (assign6190_e6600 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign6190_e6605) + (assign6190_e6600 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign6190_e6605) + (assign6190_e6600 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign6190_e6605) + (assign6190_e6600 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign6190_e6605) + (assign6190_e6600 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign6200_e6610: f64 = (1.0 / var_qsqrt);
        let assign6200_e6612: f64 = (assign6200_e6610 - var_t0);
        var_dlogsinhqsqdqsqrt = assign6200_e6612;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign6210_e6614: f64 = (-2.0);
        let assign6210_e6616: f64 = (assign6210_e6614 * var_k1);
        let assign6210_e6618: f64 = (assign6210_e6616 * var_auxb1);
        let assign6210_e6620: f64 = (assign6210_e6618 + var_aaux);
        var_dqsqrt = assign6210_e6620;
        var_dqsqrt_dn3 = ((assign6210_e6616 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign6210_e6616 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign6210_e6616 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign6210_e6616 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign6210_e6616 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign6210_e6616 * var_auxb1_dn8) + var_aaux_dn8);

        let assign6220_e6623: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign6220_e6623;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign6230_e6625: f64 = (-1.0);
        let assign6230_e6628: f64 = (-var_k1);
        let assign6230_e6630: f64 = (assign6230_e6628 + var_dqcoth);
        let assign6230_e6632: f64 = (assign6230_e6630 * var_t3);
        let assign6230_e6633: f64 = (2.0 * assign6230_e6632);
        let assign6230_e6634: f64 = (assign6230_e6625 + assign6230_e6633);
        let assign6230_e6637: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign6230_e6638: f64 = (assign6230_e6634 - assign6230_e6637);
        var_dq2 = assign6230_e6638;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign6230_e6630 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign6230_e6630 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign6230_e6630 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign6230_e6630 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign6230_e6630 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign6230_e6630 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign6240_e6643: f64 = (var_auxb1 + var_t2);
        let assign6240_e6644: f64 = (var_k1 * assign6240_e6643);
        let assign6240_e6645: f64 = (var_aaux - assign6240_e6644);
        let assign6240_e6648: f64 = (var_auxb1 * var_dqcoth);
        let assign6240_e6649: f64 = (assign6240_e6645 + assign6240_e6648);
        let assign6240_e6653: f64 = (var_dq2 * var_t2);
        let assign6240_e6657: f64 = (var_dqcoth - var_k1);
        let assign6240_e6658: f64 = (var_q2 * assign6240_e6657);
        let assign6240_e6659: f64 = (assign6240_e6653 + assign6240_e6658);
        let assign6240_e6660: f64 = (var_k2 * assign6240_e6659);
        let assign6240_e6661: f64 = (assign6240_e6649 + assign6240_e6660);
        var_df = assign6240_e6661;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign6240_e6657) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign6240_e6657) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign6240_e6657) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign6240_e6657) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign6240_e6657) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign6240_e6657) + (var_q2 * var_dqcoth_dn8)))));

        let assign6250_e6663: f64 = (-var_f);
        let assign6250_e6665: f64 = (assign6250_e6663 / var_df);
        var_delta = assign6250_e6665;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign6250_e6663 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign6250_e6663 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign6250_e6663 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign6250_e6663 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign6250_e6663 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign6250_e6663 * var_df_dn8)) / (var_df * var_df));

        let assign6260_e6668: f64 = (var_phi1 + var_delta);
        var_phi1 = assign6260_e6668;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign6270_e6671: f64 = (var_xg1 - var_phi1);
        var_q1 = assign6270_e6671;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign6280_e6674: f64 = (var_k1 * var_q1);
        var_auxb1 = assign6280_e6674;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign6290_e6676: f64 = (-var_a0);
        let assign6290_e6678: f64 = (var_phi1).exp();
        let assign6290_e6679: f64 = (assign6290_e6676 * assign6290_e6678);
        var_aaux = assign6290_e6679;
        var_aaux_dn3 = (((-var_a0_dn3) * assign6290_e6678) + (assign6290_e6676 * (assign6290_e6678 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign6290_e6678) + (assign6290_e6676 * (assign6290_e6678 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign6290_e6678) + (assign6290_e6676 * (assign6290_e6678 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign6290_e6678) + (assign6290_e6676 * (assign6290_e6678 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign6290_e6678) + (assign6290_e6676 * (assign6290_e6678 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign6290_e6678) + (assign6290_e6676 * (assign6290_e6678 * var_phi1_dn8)));

        let assign6300_e6682: f64 = (var_auxb1 * var_auxb1);
        let assign6300_e6684: f64 = (assign6300_e6682 + var_aaux);
        var_qsqrt = assign6300_e6684;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign6310_e6687: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard79 = assign6310_e6687;

        let (assign6320_e6693, assign6320_e6693_d_n3, assign6320_e6693_d_n4, assign6320_e6693_d_n5, assign6320_e6693_d_n6, assign6320_e6693_d_n7, assign6320_e6693_d_n8,) = {
    if (var_guard79 != 0.0) {
        let assign6320_e6690: f64 = (-var_qsqrt);
        let assign6320_e6691: f64 = (assign6320_e6690).sqrt();
        (assign6320_e6691, ((-var_qsqrt_dn3) / (2.0 * assign6320_e6691)), ((-var_qsqrt_dn4) / (2.0 * assign6320_e6691)), ((-var_qsqrt_dn5) / (2.0 * assign6320_e6691)), ((-var_qsqrt_dn6) / (2.0 * assign6320_e6691)), ((-var_qsqrt_dn7) / (2.0 * assign6320_e6691)), ((-var_qsqrt_dn8) / (2.0 * assign6320_e6691)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign6320_e6693;
        var_q_dn3 = assign6320_e6693_d_n3;
        var_q_dn4 = assign6320_e6693_d_n4;
        var_q_dn5 = assign6320_e6693_d_n5;
        var_q_dn6 = assign6320_e6693_d_n6;
        var_q_dn7 = assign6320_e6693_d_n7;
        var_q_dn8 = assign6320_e6693_d_n8;

        let (assign6330_e6702, assign6330_e6702_d_n3, assign6330_e6702_d_n4, assign6330_e6702_d_n5, assign6330_e6702_d_n6, assign6330_e6702_d_n7, assign6330_e6702_d_n8,) = {
    if (var_guard79 != 0.0) {
        let assign6330_e6698: f64 = (0.5 * var_q);
        let assign6330_e6699: f64 = (assign6330_e6698).sin();
        let assign6330_e6700: f64 = (1.0 / assign6330_e6699);
        (assign6330_e6700, (-(((assign6330_e6698).cos() * (0.5 * var_q_dn3)) / (assign6330_e6699 * assign6330_e6699))), (-(((assign6330_e6698).cos() * (0.5 * var_q_dn4)) / (assign6330_e6699 * assign6330_e6699))), (-(((assign6330_e6698).cos() * (0.5 * var_q_dn5)) / (assign6330_e6699 * assign6330_e6699))), (-(((assign6330_e6698).cos() * (0.5 * var_q_dn6)) / (assign6330_e6699 * assign6330_e6699))), (-(((assign6330_e6698).cos() * (0.5 * var_q_dn7)) / (assign6330_e6699 * assign6330_e6699))), (-(((assign6330_e6698).cos() * (0.5 * var_q_dn8)) / (assign6330_e6699 * assign6330_e6699))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign6330_e6702;
        var_csc1_dn3 = assign6330_e6702_d_n3;
        var_csc1_dn4 = assign6330_e6702_d_n4;
        var_csc1_dn5 = assign6330_e6702_d_n5;
        var_csc1_dn6 = assign6330_e6702_d_n6;
        var_csc1_dn7 = assign6330_e6702_d_n7;
        var_csc1_dn8 = assign6330_e6702_d_n8;

        let (assign6340_e6708, assign6340_e6708_d_n3, assign6340_e6708_d_n4, assign6340_e6708_d_n5, assign6340_e6708_d_n6, assign6340_e6708_d_n7, assign6340_e6708_d_n8,) = {
    if (var_guard79 != 0.0) {
        let assign6340_e6706: f64 = (var_csc1 * var_csc1);
        (assign6340_e6706, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign6340_e6708;
        var_t1_dn3 = assign6340_e6708_d_n3;
        var_t1_dn4 = assign6340_e6708_d_n4;
        var_t1_dn5 = assign6340_e6708_d_n5;
        var_t1_dn6 = assign6340_e6708_d_n6;
        var_t1_dn7 = assign6340_e6708_d_n7;
        var_t1_dn8 = assign6340_e6708_d_n8;

        let (assign6350_e6717, assign6350_e6717_d_n3, assign6350_e6717_d_n4, assign6350_e6717_d_n5, assign6350_e6717_d_n6, assign6350_e6717_d_n7, assign6350_e6717_d_n8,) = {
    if (var_guard79 != 0.0) {
        let assign6350_e6712: f64 = (0.5 * var_q);
        let assign6350_e6713: f64 = (assign6350_e6712).cos();
        let assign6350_e6715: f64 = (assign6350_e6713 * var_csc1);
        (assign6350_e6715, (((-(assign6350_e6712).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign6350_e6713 * var_csc1_dn3)), (((-(assign6350_e6712).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign6350_e6713 * var_csc1_dn4)), (((-(assign6350_e6712).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign6350_e6713 * var_csc1_dn5)), (((-(assign6350_e6712).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign6350_e6713 * var_csc1_dn6)), (((-(assign6350_e6712).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign6350_e6713 * var_csc1_dn7)), (((-(assign6350_e6712).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign6350_e6713 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign6350_e6717;
        var_coth1_dn3 = assign6350_e6717_d_n3;
        var_coth1_dn4 = assign6350_e6717_d_n4;
        var_coth1_dn5 = assign6350_e6717_d_n5;
        var_coth1_dn6 = assign6350_e6717_d_n6;
        var_coth1_dn7 = assign6350_e6717_d_n7;
        var_coth1_dn8 = assign6350_e6717_d_n8;

        let (assign6360_e6726, assign6360_e6726_d_n3, assign6360_e6726_d_n4, assign6360_e6726_d_n5, assign6360_e6726_d_n6, assign6360_e6726_d_n7, assign6360_e6726_d_n8,) = {
    if (var_guard79 != 0.0) {
        let assign6360_e6720: f64 = (-0.5);
        let assign6360_e6722: f64 = (assign6360_e6720 * var_coth1);
        let assign6360_e6724: f64 = (assign6360_e6722 / var_q);
        (assign6360_e6724, ((((assign6360_e6720 * var_coth1_dn3) * var_q) - (assign6360_e6722 * var_q_dn3)) / (var_q * var_q)), ((((assign6360_e6720 * var_coth1_dn4) * var_q) - (assign6360_e6722 * var_q_dn4)) / (var_q * var_q)), ((((assign6360_e6720 * var_coth1_dn5) * var_q) - (assign6360_e6722 * var_q_dn5)) / (var_q * var_q)), ((((assign6360_e6720 * var_coth1_dn6) * var_q) - (assign6360_e6722 * var_q_dn6)) / (var_q * var_q)), ((((assign6360_e6720 * var_coth1_dn7) * var_q) - (assign6360_e6722 * var_q_dn7)) / (var_q * var_q)), ((((assign6360_e6720 * var_coth1_dn8) * var_q) - (assign6360_e6722 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign6360_e6726;
        var_t0_dn3 = assign6360_e6726_d_n3;
        var_t0_dn4 = assign6360_e6726_d_n4;
        var_t0_dn5 = assign6360_e6726_d_n5;
        var_t0_dn6 = assign6360_e6726_d_n6;
        var_t0_dn7 = assign6360_e6726_d_n7;
        var_t0_dn8 = assign6360_e6726_d_n8;

        let (assign6370_e6734, assign6370_e6734_d_n3, assign6370_e6734_d_n4, assign6370_e6734_d_n5, assign6370_e6734_d_n6, assign6370_e6734_d_n7, assign6370_e6734_d_n8,) = {
    if (var_guard79 != 0.0) {
        let assign6370_e6730: f64 = (0.25 * var_t1);
        let assign6370_e6732: f64 = (assign6370_e6730 + var_t0);
        (assign6370_e6732, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign6370_e6734;
        var_dqcothqdqsqrt_dn3 = assign6370_e6734_d_n3;
        var_dqcothqdqsqrt_dn4 = assign6370_e6734_d_n4;
        var_dqcothqdqsqrt_dn5 = assign6370_e6734_d_n5;
        var_dqcothqdqsqrt_dn6 = assign6370_e6734_d_n6;
        var_dqcothqdqsqrt_dn7 = assign6370_e6734_d_n7;
        var_dqcothqdqsqrt_dn8 = assign6370_e6734_d_n8;

        let (assign6380_e6740, assign6380_e6740_d_n3, assign6380_e6740_d_n4, assign6380_e6740_d_n5, assign6380_e6740_d_n6, assign6380_e6740_d_n7, assign6380_e6740_d_n8,) = {
    if (var_guard79 == 0.0) {
        let assign6380_e6738: f64 = (var_qsqrt).sqrt();
        (assign6380_e6738, (var_qsqrt_dn3 / (2.0 * assign6380_e6738)), (var_qsqrt_dn4 / (2.0 * assign6380_e6738)), (var_qsqrt_dn5 / (2.0 * assign6380_e6738)), (var_qsqrt_dn6 / (2.0 * assign6380_e6738)), (var_qsqrt_dn7 / (2.0 * assign6380_e6738)), (var_qsqrt_dn8 / (2.0 * assign6380_e6738)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign6380_e6740;
        var_q_dn3 = assign6380_e6740_d_n3;
        var_q_dn4 = assign6380_e6740_d_n4;
        var_q_dn5 = assign6380_e6740_d_n5;
        var_q_dn6 = assign6380_e6740_d_n6;
        var_q_dn7 = assign6380_e6740_d_n7;
        var_q_dn8 = assign6380_e6740_d_n8;

        let (assign6390_e6750, assign6390_e6750_d_n3, assign6390_e6750_d_n4, assign6390_e6750_d_n5, assign6390_e6750_d_n6, assign6390_e6750_d_n7, assign6390_e6750_d_n8,) = {
    if (var_guard79 == 0.0) {
        let assign6390_e6746: f64 = (0.5 * var_q);
        let assign6390_e6747: f64 = (assign6390_e6746).sinh();
        let assign6390_e6748: f64 = (1.0 / assign6390_e6747);
        (assign6390_e6748, (-(((assign6390_e6746).cosh() * (0.5 * var_q_dn3)) / (assign6390_e6747 * assign6390_e6747))), (-(((assign6390_e6746).cosh() * (0.5 * var_q_dn4)) / (assign6390_e6747 * assign6390_e6747))), (-(((assign6390_e6746).cosh() * (0.5 * var_q_dn5)) / (assign6390_e6747 * assign6390_e6747))), (-(((assign6390_e6746).cosh() * (0.5 * var_q_dn6)) / (assign6390_e6747 * assign6390_e6747))), (-(((assign6390_e6746).cosh() * (0.5 * var_q_dn7)) / (assign6390_e6747 * assign6390_e6747))), (-(((assign6390_e6746).cosh() * (0.5 * var_q_dn8)) / (assign6390_e6747 * assign6390_e6747))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign6390_e6750;
        var_csc1_dn3 = assign6390_e6750_d_n3;
        var_csc1_dn4 = assign6390_e6750_d_n4;
        var_csc1_dn5 = assign6390_e6750_d_n5;
        var_csc1_dn6 = assign6390_e6750_d_n6;
        var_csc1_dn7 = assign6390_e6750_d_n7;
        var_csc1_dn8 = assign6390_e6750_d_n8;

        let (assign6400_e6757, assign6400_e6757_d_n3, assign6400_e6757_d_n4, assign6400_e6757_d_n5, assign6400_e6757_d_n6, assign6400_e6757_d_n7, assign6400_e6757_d_n8,) = {
    if (var_guard79 == 0.0) {
        let assign6400_e6755: f64 = (var_csc1 * var_csc1);
        (assign6400_e6755, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign6400_e6757;
        var_t1_dn3 = assign6400_e6757_d_n3;
        var_t1_dn4 = assign6400_e6757_d_n4;
        var_t1_dn5 = assign6400_e6757_d_n5;
        var_t1_dn6 = assign6400_e6757_d_n6;
        var_t1_dn7 = assign6400_e6757_d_n7;
        var_t1_dn8 = assign6400_e6757_d_n8;

        let (assign6410_e6765, assign6410_e6765_d_n3, assign6410_e6765_d_n4, assign6410_e6765_d_n5, assign6410_e6765_d_n6, assign6410_e6765_d_n7, assign6410_e6765_d_n8,) = {
    if (var_guard79 == 0.0) {
        let assign6410_e6762: f64 = (1.0 + var_t1);
        let assign6410_e6763: f64 = (assign6410_e6762).sqrt();
        (assign6410_e6763, (var_t1_dn3 / (2.0 * assign6410_e6763)), (var_t1_dn4 / (2.0 * assign6410_e6763)), (var_t1_dn5 / (2.0 * assign6410_e6763)), (var_t1_dn6 / (2.0 * assign6410_e6763)), (var_t1_dn7 / (2.0 * assign6410_e6763)), (var_t1_dn8 / (2.0 * assign6410_e6763)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign6410_e6765;
        var_coth1_dn3 = assign6410_e6765_d_n3;
        var_coth1_dn4 = assign6410_e6765_d_n4;
        var_coth1_dn5 = assign6410_e6765_d_n5;
        var_coth1_dn6 = assign6410_e6765_d_n6;
        var_coth1_dn7 = assign6410_e6765_d_n7;
        var_coth1_dn8 = assign6410_e6765_d_n8;

        let (assign6420_e6774, assign6420_e6774_d_n3, assign6420_e6774_d_n4, assign6420_e6774_d_n5, assign6420_e6774_d_n6, assign6420_e6774_d_n7, assign6420_e6774_d_n8,) = {
    if (var_guard79 == 0.0) {
        let assign6420_e6770: f64 = (0.5 * var_coth1);
        let assign6420_e6772: f64 = (assign6420_e6770 / var_q);
        (assign6420_e6772, ((((0.5 * var_coth1_dn3) * var_q) - (assign6420_e6770 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign6420_e6770 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign6420_e6770 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign6420_e6770 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign6420_e6770 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign6420_e6770 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign6420_e6774;
        var_t0_dn3 = assign6420_e6774_d_n3;
        var_t0_dn4 = assign6420_e6774_d_n4;
        var_t0_dn5 = assign6420_e6774_d_n5;
        var_t0_dn6 = assign6420_e6774_d_n6;
        var_t0_dn7 = assign6420_e6774_d_n7;
        var_t0_dn8 = assign6420_e6774_d_n8;

        let (assign6430_e6784, assign6430_e6784_d_n3, assign6430_e6784_d_n4, assign6430_e6784_d_n5, assign6430_e6784_d_n6, assign6430_e6784_d_n7, assign6430_e6784_d_n8,) = {
    if (var_guard79 == 0.0) {
        let assign6430_e6778: f64 = (-0.25);
        let assign6430_e6780: f64 = (assign6430_e6778 * var_t1);
        let assign6430_e6782: f64 = (assign6430_e6780 + var_t0);
        (assign6430_e6782, ((assign6430_e6778 * var_t1_dn3) + var_t0_dn3), ((assign6430_e6778 * var_t1_dn4) + var_t0_dn4), ((assign6430_e6778 * var_t1_dn5) + var_t0_dn5), ((assign6430_e6778 * var_t1_dn6) + var_t0_dn6), ((assign6430_e6778 * var_t1_dn7) + var_t0_dn7), ((assign6430_e6778 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign6430_e6784;
        var_dqcothqdqsqrt_dn3 = assign6430_e6784_d_n3;
        var_dqcothqdqsqrt_dn4 = assign6430_e6784_d_n4;
        var_dqcothqdqsqrt_dn5 = assign6430_e6784_d_n5;
        var_dqcothqdqsqrt_dn6 = assign6430_e6784_d_n6;
        var_dqcothqdqsqrt_dn7 = assign6430_e6784_d_n7;
        var_dqcothqdqsqrt_dn8 = assign6430_e6784_d_n8;

        let assign6440_e6787: f64 = (var_q * var_coth1);
        var_qcoth = assign6440_e6787;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign6450_e6790: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign6450_e6790;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign6460_e6793: f64 = (1.0 / var_t2);
        var_t3 = assign6460_e6793;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign6470_e6796: f64 = (var_xg2 - var_xg1);
        let assign6470_e6798: f64 = (assign6470_e6796 + var_q1);
        let assign6470_e6801: f64 = (var_qsqrt * var_t1);
        let assign6470_e6803: f64 = (assign6470_e6801 * var_t3);
        let assign6470_e6805: f64 = (assign6470_e6803 * var_t3);
        let assign6470_e6806: f64 = (assign6470_e6805).abs();
        let assign6470_e6807: f64 = (assign6470_e6806).ln();
        let assign6470_e6808: f64 = (assign6470_e6798 - assign6470_e6807);
        var_q2 = assign6470_e6808;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign6470_e6805 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign6470_e6801 * var_t3_dn3)) * var_t3) + (assign6470_e6803 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign6470_e6801 * var_t3_dn3)) * var_t3) + (assign6470_e6803 * var_t3_dn3))) } / assign6470_e6806));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign6470_e6805 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign6470_e6801 * var_t3_dn4)) * var_t3) + (assign6470_e6803 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign6470_e6801 * var_t3_dn4)) * var_t3) + (assign6470_e6803 * var_t3_dn4))) } / assign6470_e6806));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign6470_e6805 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign6470_e6801 * var_t3_dn5)) * var_t3) + (assign6470_e6803 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign6470_e6801 * var_t3_dn5)) * var_t3) + (assign6470_e6803 * var_t3_dn5))) } / assign6470_e6806));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign6470_e6805 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign6470_e6801 * var_t3_dn6)) * var_t3) + (assign6470_e6803 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign6470_e6801 * var_t3_dn6)) * var_t3) + (assign6470_e6803 * var_t3_dn6))) } / assign6470_e6806));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign6470_e6805 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign6470_e6801 * var_t3_dn7)) * var_t3) + (assign6470_e6803 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign6470_e6801 * var_t3_dn7)) * var_t3) + (assign6470_e6803 * var_t3_dn7))) } / assign6470_e6806));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign6470_e6805 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign6470_e6801 * var_t3_dn8)) * var_t3) + (assign6470_e6803 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign6470_e6801 * var_t3_dn8)) * var_t3) + (assign6470_e6803 * var_t3_dn8))) } / assign6470_e6806));

        let assign6480_e6812: f64 = (var_auxb1 + var_qcoth);
        let assign6480_e6815: f64 = (var_k2 * var_q2);
        let assign6480_e6817: f64 = (assign6480_e6815 + var_auxb1);
        let assign6480_e6818: f64 = (assign6480_e6812 * assign6480_e6817);
        let assign6480_e6819: f64 = (var_aaux + assign6480_e6818);
        var_f = assign6480_e6819;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign6480_e6817) + (assign6480_e6812 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign6480_e6817) + (assign6480_e6812 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign6480_e6817) + (assign6480_e6812 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign6480_e6817) + (assign6480_e6812 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign6480_e6817) + (assign6480_e6812 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign6480_e6817) + (assign6480_e6812 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign6490_e6822: f64 = (1.0 / var_qsqrt);
        let assign6490_e6824: f64 = (assign6490_e6822 - var_t0);
        var_dlogsinhqsqdqsqrt = assign6490_e6824;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        *var_aaux_slot = var_aaux;
        *var_aaux_dn3_slot = var_aaux_dn3;
        *var_aaux_dn4_slot = var_aaux_dn4;
        *var_aaux_dn5_slot = var_aaux_dn5;
        *var_aaux_dn6_slot = var_aaux_dn6;
        *var_aaux_dn7_slot = var_aaux_dn7;
        *var_aaux_dn8_slot = var_aaux_dn8;
        *var_auxb1_slot = var_auxb1;
        *var_auxb1_dn3_slot = var_auxb1_dn3;
        *var_auxb1_dn4_slot = var_auxb1_dn4;
        *var_auxb1_dn5_slot = var_auxb1_dn5;
        *var_auxb1_dn6_slot = var_auxb1_dn6;
        *var_auxb1_dn7_slot = var_auxb1_dn7;
        *var_auxb1_dn8_slot = var_auxb1_dn8;
        *var_coth1_slot = var_coth1;
        *var_coth1_dn3_slot = var_coth1_dn3;
        *var_coth1_dn4_slot = var_coth1_dn4;
        *var_coth1_dn5_slot = var_coth1_dn5;
        *var_coth1_dn6_slot = var_coth1_dn6;
        *var_coth1_dn7_slot = var_coth1_dn7;
        *var_coth1_dn8_slot = var_coth1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_df_slot = var_df;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_dn6_slot = var_df_dn6;
        *var_df_dn7_slot = var_df_dn7;
        *var_df_dn8_slot = var_df_dn8;
        *var_dlogsinhqsqdqsqrt_slot = var_dlogsinhqsqdqsqrt;
        *var_dlogsinhqsqdqsqrt_dn3_slot = var_dlogsinhqsqdqsqrt_dn3;
        *var_dlogsinhqsqdqsqrt_dn4_slot = var_dlogsinhqsqdqsqrt_dn4;
        *var_dlogsinhqsqdqsqrt_dn5_slot = var_dlogsinhqsqdqsqrt_dn5;
        *var_dlogsinhqsqdqsqrt_dn6_slot = var_dlogsinhqsqdqsqrt_dn6;
        *var_dlogsinhqsqdqsqrt_dn7_slot = var_dlogsinhqsqdqsqrt_dn7;
        *var_dlogsinhqsqdqsqrt_dn8_slot = var_dlogsinhqsqdqsqrt_dn8;
        *var_dq2_slot = var_dq2;
        *var_dq2_dn3_slot = var_dq2_dn3;
        *var_dq2_dn4_slot = var_dq2_dn4;
        *var_dq2_dn5_slot = var_dq2_dn5;
        *var_dq2_dn6_slot = var_dq2_dn6;
        *var_dq2_dn7_slot = var_dq2_dn7;
        *var_dq2_dn8_slot = var_dq2_dn8;
        *var_dqcoth_slot = var_dqcoth;
        *var_dqcoth_dn3_slot = var_dqcoth_dn3;
        *var_dqcoth_dn4_slot = var_dqcoth_dn4;
        *var_dqcoth_dn5_slot = var_dqcoth_dn5;
        *var_dqcoth_dn6_slot = var_dqcoth_dn6;
        *var_dqcoth_dn7_slot = var_dqcoth_dn7;
        *var_dqcoth_dn8_slot = var_dqcoth_dn8;
        *var_dqcothqdqsqrt_slot = var_dqcothqdqsqrt;
        *var_dqcothqdqsqrt_dn3_slot = var_dqcothqdqsqrt_dn3;
        *var_dqcothqdqsqrt_dn4_slot = var_dqcothqdqsqrt_dn4;
        *var_dqcothqdqsqrt_dn5_slot = var_dqcothqdqsqrt_dn5;
        *var_dqcothqdqsqrt_dn6_slot = var_dqcothqdqsqrt_dn6;
        *var_dqcothqdqsqrt_dn7_slot = var_dqcothqdqsqrt_dn7;
        *var_dqcothqdqsqrt_dn8_slot = var_dqcothqdqsqrt_dn8;
        *var_dqsqrt_slot = var_dqsqrt;
        *var_dqsqrt_dn3_slot = var_dqsqrt_dn3;
        *var_dqsqrt_dn4_slot = var_dqsqrt_dn4;
        *var_dqsqrt_dn5_slot = var_dqsqrt_dn5;
        *var_dqsqrt_dn6_slot = var_dqsqrt_dn6;
        *var_dqsqrt_dn7_slot = var_dqsqrt_dn7;
        *var_dqsqrt_dn8_slot = var_dqsqrt_dn8;
        *var_f_slot = var_f;
        *var_f_dn3_slot = var_f_dn3;
        *var_f_dn4_slot = var_f_dn4;
        *var_f_dn5_slot = var_f_dn5;
        *var_f_dn6_slot = var_f_dn6;
        *var_f_dn7_slot = var_f_dn7;
        *var_f_dn8_slot = var_f_dn8;
        *var_guard79_slot = var_guard79;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
    }

    pub(super) fn stamp_transient_block_12(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_k1: f64,
        var_k2: f64,
        var_xg1: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_aaux_slot: &mut f64,
        var_aaux_dn3_slot: &mut f64,
        var_aaux_dn4_slot: &mut f64,
        var_aaux_dn5_slot: &mut f64,
        var_aaux_dn6_slot: &mut f64,
        var_aaux_dn7_slot: &mut f64,
        var_aaux_dn8_slot: &mut f64,
        var_auxb1_slot: &mut f64,
        var_auxb1_dn3_slot: &mut f64,
        var_auxb1_dn4_slot: &mut f64,
        var_auxb1_dn5_slot: &mut f64,
        var_auxb1_dn6_slot: &mut f64,
        var_auxb1_dn7_slot: &mut f64,
        var_auxb1_dn8_slot: &mut f64,
        var_coth1_slot: &mut f64,
        var_coth1_dn3_slot: &mut f64,
        var_coth1_dn4_slot: &mut f64,
        var_coth1_dn5_slot: &mut f64,
        var_coth1_dn6_slot: &mut f64,
        var_coth1_dn7_slot: &mut f64,
        var_coth1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_df_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_dn6_slot: &mut f64,
        var_df_dn7_slot: &mut f64,
        var_df_dn8_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn3_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn4_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn5_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn6_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn7_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn8_slot: &mut f64,
        var_dq2_slot: &mut f64,
        var_dq2_dn3_slot: &mut f64,
        var_dq2_dn4_slot: &mut f64,
        var_dq2_dn5_slot: &mut f64,
        var_dq2_dn6_slot: &mut f64,
        var_dq2_dn7_slot: &mut f64,
        var_dq2_dn8_slot: &mut f64,
        var_dqcoth_slot: &mut f64,
        var_dqcoth_dn3_slot: &mut f64,
        var_dqcoth_dn4_slot: &mut f64,
        var_dqcoth_dn5_slot: &mut f64,
        var_dqcoth_dn6_slot: &mut f64,
        var_dqcoth_dn7_slot: &mut f64,
        var_dqcoth_dn8_slot: &mut f64,
        var_dqcothqdqsqrt_slot: &mut f64,
        var_dqcothqdqsqrt_dn3_slot: &mut f64,
        var_dqcothqdqsqrt_dn4_slot: &mut f64,
        var_dqcothqdqsqrt_dn5_slot: &mut f64,
        var_dqcothqdqsqrt_dn6_slot: &mut f64,
        var_dqcothqdqsqrt_dn7_slot: &mut f64,
        var_dqcothqdqsqrt_dn8_slot: &mut f64,
        var_dqsqrt_slot: &mut f64,
        var_dqsqrt_dn3_slot: &mut f64,
        var_dqsqrt_dn4_slot: &mut f64,
        var_dqsqrt_dn5_slot: &mut f64,
        var_dqsqrt_dn6_slot: &mut f64,
        var_dqsqrt_dn7_slot: &mut f64,
        var_dqsqrt_dn8_slot: &mut f64,
        var_f_slot: &mut f64,
        var_f_dn3_slot: &mut f64,
        var_f_dn4_slot: &mut f64,
        var_f_dn5_slot: &mut f64,
        var_f_dn6_slot: &mut f64,
        var_f_dn7_slot: &mut f64,
        var_f_dn8_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
    ) {
        let mut var_aaux: f64 = *var_aaux_slot;
        let mut var_aaux_dn3: f64 = *var_aaux_dn3_slot;
        let mut var_aaux_dn4: f64 = *var_aaux_dn4_slot;
        let mut var_aaux_dn5: f64 = *var_aaux_dn5_slot;
        let mut var_aaux_dn6: f64 = *var_aaux_dn6_slot;
        let mut var_aaux_dn7: f64 = *var_aaux_dn7_slot;
        let mut var_aaux_dn8: f64 = *var_aaux_dn8_slot;
        let mut var_auxb1: f64 = *var_auxb1_slot;
        let mut var_auxb1_dn3: f64 = *var_auxb1_dn3_slot;
        let mut var_auxb1_dn4: f64 = *var_auxb1_dn4_slot;
        let mut var_auxb1_dn5: f64 = *var_auxb1_dn5_slot;
        let mut var_auxb1_dn6: f64 = *var_auxb1_dn6_slot;
        let mut var_auxb1_dn7: f64 = *var_auxb1_dn7_slot;
        let mut var_auxb1_dn8: f64 = *var_auxb1_dn8_slot;
        let mut var_coth1: f64 = *var_coth1_slot;
        let mut var_coth1_dn3: f64 = *var_coth1_dn3_slot;
        let mut var_coth1_dn4: f64 = *var_coth1_dn4_slot;
        let mut var_coth1_dn5: f64 = *var_coth1_dn5_slot;
        let mut var_coth1_dn6: f64 = *var_coth1_dn6_slot;
        let mut var_coth1_dn7: f64 = *var_coth1_dn7_slot;
        let mut var_coth1_dn8: f64 = *var_coth1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_dn6: f64 = *var_df_dn6_slot;
        let mut var_df_dn7: f64 = *var_df_dn7_slot;
        let mut var_df_dn8: f64 = *var_df_dn8_slot;
        let mut var_dlogsinhqsqdqsqrt: f64 = *var_dlogsinhqsqdqsqrt_slot;
        let mut var_dlogsinhqsqdqsqrt_dn3: f64 = *var_dlogsinhqsqdqsqrt_dn3_slot;
        let mut var_dlogsinhqsqdqsqrt_dn4: f64 = *var_dlogsinhqsqdqsqrt_dn4_slot;
        let mut var_dlogsinhqsqdqsqrt_dn5: f64 = *var_dlogsinhqsqdqsqrt_dn5_slot;
        let mut var_dlogsinhqsqdqsqrt_dn6: f64 = *var_dlogsinhqsqdqsqrt_dn6_slot;
        let mut var_dlogsinhqsqdqsqrt_dn7: f64 = *var_dlogsinhqsqdqsqrt_dn7_slot;
        let mut var_dlogsinhqsqdqsqrt_dn8: f64 = *var_dlogsinhqsqdqsqrt_dn8_slot;
        let mut var_dq2: f64 = *var_dq2_slot;
        let mut var_dq2_dn3: f64 = *var_dq2_dn3_slot;
        let mut var_dq2_dn4: f64 = *var_dq2_dn4_slot;
        let mut var_dq2_dn5: f64 = *var_dq2_dn5_slot;
        let mut var_dq2_dn6: f64 = *var_dq2_dn6_slot;
        let mut var_dq2_dn7: f64 = *var_dq2_dn7_slot;
        let mut var_dq2_dn8: f64 = *var_dq2_dn8_slot;
        let mut var_dqcoth: f64 = *var_dqcoth_slot;
        let mut var_dqcoth_dn3: f64 = *var_dqcoth_dn3_slot;
        let mut var_dqcoth_dn4: f64 = *var_dqcoth_dn4_slot;
        let mut var_dqcoth_dn5: f64 = *var_dqcoth_dn5_slot;
        let mut var_dqcoth_dn6: f64 = *var_dqcoth_dn6_slot;
        let mut var_dqcoth_dn7: f64 = *var_dqcoth_dn7_slot;
        let mut var_dqcoth_dn8: f64 = *var_dqcoth_dn8_slot;
        let mut var_dqcothqdqsqrt: f64 = *var_dqcothqdqsqrt_slot;
        let mut var_dqcothqdqsqrt_dn3: f64 = *var_dqcothqdqsqrt_dn3_slot;
        let mut var_dqcothqdqsqrt_dn4: f64 = *var_dqcothqdqsqrt_dn4_slot;
        let mut var_dqcothqdqsqrt_dn5: f64 = *var_dqcothqdqsqrt_dn5_slot;
        let mut var_dqcothqdqsqrt_dn6: f64 = *var_dqcothqdqsqrt_dn6_slot;
        let mut var_dqcothqdqsqrt_dn7: f64 = *var_dqcothqdqsqrt_dn7_slot;
        let mut var_dqcothqdqsqrt_dn8: f64 = *var_dqcothqdqsqrt_dn8_slot;
        let mut var_dqsqrt: f64 = *var_dqsqrt_slot;
        let mut var_dqsqrt_dn3: f64 = *var_dqsqrt_dn3_slot;
        let mut var_dqsqrt_dn4: f64 = *var_dqsqrt_dn4_slot;
        let mut var_dqsqrt_dn5: f64 = *var_dqsqrt_dn5_slot;
        let mut var_dqsqrt_dn6: f64 = *var_dqsqrt_dn6_slot;
        let mut var_dqsqrt_dn7: f64 = *var_dqsqrt_dn7_slot;
        let mut var_dqsqrt_dn8: f64 = *var_dqsqrt_dn8_slot;
        let mut var_f: f64 = *var_f_slot;
        let mut var_f_dn3: f64 = *var_f_dn3_slot;
        let mut var_f_dn4: f64 = *var_f_dn4_slot;
        let mut var_f_dn5: f64 = *var_f_dn5_slot;
        let mut var_f_dn6: f64 = *var_f_dn6_slot;
        let mut var_f_dn7: f64 = *var_f_dn7_slot;
        let mut var_f_dn8: f64 = *var_f_dn8_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;

        let assign6500_e6826: f64 = (-2.0);
        let assign6500_e6828: f64 = (assign6500_e6826 * var_k1);
        let assign6500_e6830: f64 = (assign6500_e6828 * var_auxb1);
        let assign6500_e6832: f64 = (assign6500_e6830 + var_aaux);
        var_dqsqrt = assign6500_e6832;
        var_dqsqrt_dn3 = ((assign6500_e6828 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign6500_e6828 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign6500_e6828 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign6500_e6828 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign6500_e6828 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign6500_e6828 * var_auxb1_dn8) + var_aaux_dn8);

        let assign6510_e6835: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign6510_e6835;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign6520_e6837: f64 = (-1.0);
        let assign6520_e6840: f64 = (-var_k1);
        let assign6520_e6842: f64 = (assign6520_e6840 + var_dqcoth);
        let assign6520_e6844: f64 = (assign6520_e6842 * var_t3);
        let assign6520_e6845: f64 = (2.0 * assign6520_e6844);
        let assign6520_e6846: f64 = (assign6520_e6837 + assign6520_e6845);
        let assign6520_e6849: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign6520_e6850: f64 = (assign6520_e6846 - assign6520_e6849);
        var_dq2 = assign6520_e6850;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign6520_e6842 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign6520_e6842 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign6520_e6842 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign6520_e6842 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign6520_e6842 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign6520_e6842 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign6530_e6855: f64 = (var_auxb1 + var_t2);
        let assign6530_e6856: f64 = (var_k1 * assign6530_e6855);
        let assign6530_e6857: f64 = (var_aaux - assign6530_e6856);
        let assign6530_e6860: f64 = (var_auxb1 * var_dqcoth);
        let assign6530_e6861: f64 = (assign6530_e6857 + assign6530_e6860);
        let assign6530_e6865: f64 = (var_dq2 * var_t2);
        let assign6530_e6869: f64 = (var_dqcoth - var_k1);
        let assign6530_e6870: f64 = (var_q2 * assign6530_e6869);
        let assign6530_e6871: f64 = (assign6530_e6865 + assign6530_e6870);
        let assign6530_e6872: f64 = (var_k2 * assign6530_e6871);
        let assign6530_e6873: f64 = (assign6530_e6861 + assign6530_e6872);
        var_df = assign6530_e6873;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign6530_e6869) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign6530_e6869) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign6530_e6869) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign6530_e6869) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign6530_e6869) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign6530_e6869) + (var_q2 * var_dqcoth_dn8)))));

        let assign6540_e6875: f64 = (-var_f);
        let assign6540_e6877: f64 = (assign6540_e6875 / var_df);
        var_delta = assign6540_e6877;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign6540_e6875 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign6540_e6875 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign6540_e6875 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign6540_e6875 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign6540_e6875 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign6540_e6875 * var_df_dn8)) / (var_df * var_df));

        let assign6550_e6880: f64 = (var_phi1 + var_delta);
        var_phi1 = assign6550_e6880;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign6560_e6883: f64 = (var_xg1 - var_phi1);
        var_q1 = assign6560_e6883;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign6570_e6886: f64 = (var_k1 * var_q1);
        var_auxb1 = assign6570_e6886;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign6580_e6888: f64 = (-var_a0);
        let assign6580_e6890: f64 = (var_phi1).exp();
        let assign6580_e6891: f64 = (assign6580_e6888 * assign6580_e6890);
        var_aaux = assign6580_e6891;
        var_aaux_dn3 = (((-var_a0_dn3) * assign6580_e6890) + (assign6580_e6888 * (assign6580_e6890 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign6580_e6890) + (assign6580_e6888 * (assign6580_e6890 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign6580_e6890) + (assign6580_e6888 * (assign6580_e6890 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign6580_e6890) + (assign6580_e6888 * (assign6580_e6890 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign6580_e6890) + (assign6580_e6888 * (assign6580_e6890 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign6580_e6890) + (assign6580_e6888 * (assign6580_e6890 * var_phi1_dn8)));

        let assign6590_e6894: f64 = (var_auxb1 * var_auxb1);
        let assign6590_e6896: f64 = (assign6590_e6894 + var_aaux);
        var_qsqrt = assign6590_e6896;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign6600_e6899: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard80 = assign6600_e6899;

        let (assign6610_e6905, assign6610_e6905_d_n3, assign6610_e6905_d_n4, assign6610_e6905_d_n5, assign6610_e6905_d_n6, assign6610_e6905_d_n7, assign6610_e6905_d_n8,) = {
    if (var_guard80 != 0.0) {
        let assign6610_e6902: f64 = (-var_qsqrt);
        let assign6610_e6903: f64 = (assign6610_e6902).sqrt();
        (assign6610_e6903, ((-var_qsqrt_dn3) / (2.0 * assign6610_e6903)), ((-var_qsqrt_dn4) / (2.0 * assign6610_e6903)), ((-var_qsqrt_dn5) / (2.0 * assign6610_e6903)), ((-var_qsqrt_dn6) / (2.0 * assign6610_e6903)), ((-var_qsqrt_dn7) / (2.0 * assign6610_e6903)), ((-var_qsqrt_dn8) / (2.0 * assign6610_e6903)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign6610_e6905;
        var_q_dn3 = assign6610_e6905_d_n3;
        var_q_dn4 = assign6610_e6905_d_n4;
        var_q_dn5 = assign6610_e6905_d_n5;
        var_q_dn6 = assign6610_e6905_d_n6;
        var_q_dn7 = assign6610_e6905_d_n7;
        var_q_dn8 = assign6610_e6905_d_n8;

        let (assign6620_e6914, assign6620_e6914_d_n3, assign6620_e6914_d_n4, assign6620_e6914_d_n5, assign6620_e6914_d_n6, assign6620_e6914_d_n7, assign6620_e6914_d_n8,) = {
    if (var_guard80 != 0.0) {
        let assign6620_e6910: f64 = (0.5 * var_q);
        let assign6620_e6911: f64 = (assign6620_e6910).sin();
        let assign6620_e6912: f64 = (1.0 / assign6620_e6911);
        (assign6620_e6912, (-(((assign6620_e6910).cos() * (0.5 * var_q_dn3)) / (assign6620_e6911 * assign6620_e6911))), (-(((assign6620_e6910).cos() * (0.5 * var_q_dn4)) / (assign6620_e6911 * assign6620_e6911))), (-(((assign6620_e6910).cos() * (0.5 * var_q_dn5)) / (assign6620_e6911 * assign6620_e6911))), (-(((assign6620_e6910).cos() * (0.5 * var_q_dn6)) / (assign6620_e6911 * assign6620_e6911))), (-(((assign6620_e6910).cos() * (0.5 * var_q_dn7)) / (assign6620_e6911 * assign6620_e6911))), (-(((assign6620_e6910).cos() * (0.5 * var_q_dn8)) / (assign6620_e6911 * assign6620_e6911))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign6620_e6914;
        var_csc1_dn3 = assign6620_e6914_d_n3;
        var_csc1_dn4 = assign6620_e6914_d_n4;
        var_csc1_dn5 = assign6620_e6914_d_n5;
        var_csc1_dn6 = assign6620_e6914_d_n6;
        var_csc1_dn7 = assign6620_e6914_d_n7;
        var_csc1_dn8 = assign6620_e6914_d_n8;

        let (assign6630_e6920, assign6630_e6920_d_n3, assign6630_e6920_d_n4, assign6630_e6920_d_n5, assign6630_e6920_d_n6, assign6630_e6920_d_n7, assign6630_e6920_d_n8,) = {
    if (var_guard80 != 0.0) {
        let assign6630_e6918: f64 = (var_csc1 * var_csc1);
        (assign6630_e6918, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign6630_e6920;
        var_t1_dn3 = assign6630_e6920_d_n3;
        var_t1_dn4 = assign6630_e6920_d_n4;
        var_t1_dn5 = assign6630_e6920_d_n5;
        var_t1_dn6 = assign6630_e6920_d_n6;
        var_t1_dn7 = assign6630_e6920_d_n7;
        var_t1_dn8 = assign6630_e6920_d_n8;

        let (assign6640_e6929, assign6640_e6929_d_n3, assign6640_e6929_d_n4, assign6640_e6929_d_n5, assign6640_e6929_d_n6, assign6640_e6929_d_n7, assign6640_e6929_d_n8,) = {
    if (var_guard80 != 0.0) {
        let assign6640_e6924: f64 = (0.5 * var_q);
        let assign6640_e6925: f64 = (assign6640_e6924).cos();
        let assign6640_e6927: f64 = (assign6640_e6925 * var_csc1);
        (assign6640_e6927, (((-(assign6640_e6924).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign6640_e6925 * var_csc1_dn3)), (((-(assign6640_e6924).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign6640_e6925 * var_csc1_dn4)), (((-(assign6640_e6924).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign6640_e6925 * var_csc1_dn5)), (((-(assign6640_e6924).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign6640_e6925 * var_csc1_dn6)), (((-(assign6640_e6924).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign6640_e6925 * var_csc1_dn7)), (((-(assign6640_e6924).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign6640_e6925 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign6640_e6929;
        var_coth1_dn3 = assign6640_e6929_d_n3;
        var_coth1_dn4 = assign6640_e6929_d_n4;
        var_coth1_dn5 = assign6640_e6929_d_n5;
        var_coth1_dn6 = assign6640_e6929_d_n6;
        var_coth1_dn7 = assign6640_e6929_d_n7;
        var_coth1_dn8 = assign6640_e6929_d_n8;

        let (assign6650_e6938, assign6650_e6938_d_n3, assign6650_e6938_d_n4, assign6650_e6938_d_n5, assign6650_e6938_d_n6, assign6650_e6938_d_n7, assign6650_e6938_d_n8,) = {
    if (var_guard80 != 0.0) {
        let assign6650_e6932: f64 = (-0.5);
        let assign6650_e6934: f64 = (assign6650_e6932 * var_coth1);
        let assign6650_e6936: f64 = (assign6650_e6934 / var_q);
        (assign6650_e6936, ((((assign6650_e6932 * var_coth1_dn3) * var_q) - (assign6650_e6934 * var_q_dn3)) / (var_q * var_q)), ((((assign6650_e6932 * var_coth1_dn4) * var_q) - (assign6650_e6934 * var_q_dn4)) / (var_q * var_q)), ((((assign6650_e6932 * var_coth1_dn5) * var_q) - (assign6650_e6934 * var_q_dn5)) / (var_q * var_q)), ((((assign6650_e6932 * var_coth1_dn6) * var_q) - (assign6650_e6934 * var_q_dn6)) / (var_q * var_q)), ((((assign6650_e6932 * var_coth1_dn7) * var_q) - (assign6650_e6934 * var_q_dn7)) / (var_q * var_q)), ((((assign6650_e6932 * var_coth1_dn8) * var_q) - (assign6650_e6934 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign6650_e6938;
        var_t0_dn3 = assign6650_e6938_d_n3;
        var_t0_dn4 = assign6650_e6938_d_n4;
        var_t0_dn5 = assign6650_e6938_d_n5;
        var_t0_dn6 = assign6650_e6938_d_n6;
        var_t0_dn7 = assign6650_e6938_d_n7;
        var_t0_dn8 = assign6650_e6938_d_n8;

        let (assign6660_e6946, assign6660_e6946_d_n3, assign6660_e6946_d_n4, assign6660_e6946_d_n5, assign6660_e6946_d_n6, assign6660_e6946_d_n7, assign6660_e6946_d_n8,) = {
    if (var_guard80 != 0.0) {
        let assign6660_e6942: f64 = (0.25 * var_t1);
        let assign6660_e6944: f64 = (assign6660_e6942 + var_t0);
        (assign6660_e6944, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign6660_e6946;
        var_dqcothqdqsqrt_dn3 = assign6660_e6946_d_n3;
        var_dqcothqdqsqrt_dn4 = assign6660_e6946_d_n4;
        var_dqcothqdqsqrt_dn5 = assign6660_e6946_d_n5;
        var_dqcothqdqsqrt_dn6 = assign6660_e6946_d_n6;
        var_dqcothqdqsqrt_dn7 = assign6660_e6946_d_n7;
        var_dqcothqdqsqrt_dn8 = assign6660_e6946_d_n8;

        let (assign6670_e6952, assign6670_e6952_d_n3, assign6670_e6952_d_n4, assign6670_e6952_d_n5, assign6670_e6952_d_n6, assign6670_e6952_d_n7, assign6670_e6952_d_n8,) = {
    if (var_guard80 == 0.0) {
        let assign6670_e6950: f64 = (var_qsqrt).sqrt();
        (assign6670_e6950, (var_qsqrt_dn3 / (2.0 * assign6670_e6950)), (var_qsqrt_dn4 / (2.0 * assign6670_e6950)), (var_qsqrt_dn5 / (2.0 * assign6670_e6950)), (var_qsqrt_dn6 / (2.0 * assign6670_e6950)), (var_qsqrt_dn7 / (2.0 * assign6670_e6950)), (var_qsqrt_dn8 / (2.0 * assign6670_e6950)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign6670_e6952;
        var_q_dn3 = assign6670_e6952_d_n3;
        var_q_dn4 = assign6670_e6952_d_n4;
        var_q_dn5 = assign6670_e6952_d_n5;
        var_q_dn6 = assign6670_e6952_d_n6;
        var_q_dn7 = assign6670_e6952_d_n7;
        var_q_dn8 = assign6670_e6952_d_n8;

        let (assign6680_e6962, assign6680_e6962_d_n3, assign6680_e6962_d_n4, assign6680_e6962_d_n5, assign6680_e6962_d_n6, assign6680_e6962_d_n7, assign6680_e6962_d_n8,) = {
    if (var_guard80 == 0.0) {
        let assign6680_e6958: f64 = (0.5 * var_q);
        let assign6680_e6959: f64 = (assign6680_e6958).sinh();
        let assign6680_e6960: f64 = (1.0 / assign6680_e6959);
        (assign6680_e6960, (-(((assign6680_e6958).cosh() * (0.5 * var_q_dn3)) / (assign6680_e6959 * assign6680_e6959))), (-(((assign6680_e6958).cosh() * (0.5 * var_q_dn4)) / (assign6680_e6959 * assign6680_e6959))), (-(((assign6680_e6958).cosh() * (0.5 * var_q_dn5)) / (assign6680_e6959 * assign6680_e6959))), (-(((assign6680_e6958).cosh() * (0.5 * var_q_dn6)) / (assign6680_e6959 * assign6680_e6959))), (-(((assign6680_e6958).cosh() * (0.5 * var_q_dn7)) / (assign6680_e6959 * assign6680_e6959))), (-(((assign6680_e6958).cosh() * (0.5 * var_q_dn8)) / (assign6680_e6959 * assign6680_e6959))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign6680_e6962;
        var_csc1_dn3 = assign6680_e6962_d_n3;
        var_csc1_dn4 = assign6680_e6962_d_n4;
        var_csc1_dn5 = assign6680_e6962_d_n5;
        var_csc1_dn6 = assign6680_e6962_d_n6;
        var_csc1_dn7 = assign6680_e6962_d_n7;
        var_csc1_dn8 = assign6680_e6962_d_n8;

        let (assign6690_e6969, assign6690_e6969_d_n3, assign6690_e6969_d_n4, assign6690_e6969_d_n5, assign6690_e6969_d_n6, assign6690_e6969_d_n7, assign6690_e6969_d_n8,) = {
    if (var_guard80 == 0.0) {
        let assign6690_e6967: f64 = (var_csc1 * var_csc1);
        (assign6690_e6967, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign6690_e6969;
        var_t1_dn3 = assign6690_e6969_d_n3;
        var_t1_dn4 = assign6690_e6969_d_n4;
        var_t1_dn5 = assign6690_e6969_d_n5;
        var_t1_dn6 = assign6690_e6969_d_n6;
        var_t1_dn7 = assign6690_e6969_d_n7;
        var_t1_dn8 = assign6690_e6969_d_n8;

        let (assign6700_e6977, assign6700_e6977_d_n3, assign6700_e6977_d_n4, assign6700_e6977_d_n5, assign6700_e6977_d_n6, assign6700_e6977_d_n7, assign6700_e6977_d_n8,) = {
    if (var_guard80 == 0.0) {
        let assign6700_e6974: f64 = (1.0 + var_t1);
        let assign6700_e6975: f64 = (assign6700_e6974).sqrt();
        (assign6700_e6975, (var_t1_dn3 / (2.0 * assign6700_e6975)), (var_t1_dn4 / (2.0 * assign6700_e6975)), (var_t1_dn5 / (2.0 * assign6700_e6975)), (var_t1_dn6 / (2.0 * assign6700_e6975)), (var_t1_dn7 / (2.0 * assign6700_e6975)), (var_t1_dn8 / (2.0 * assign6700_e6975)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign6700_e6977;
        var_coth1_dn3 = assign6700_e6977_d_n3;
        var_coth1_dn4 = assign6700_e6977_d_n4;
        var_coth1_dn5 = assign6700_e6977_d_n5;
        var_coth1_dn6 = assign6700_e6977_d_n6;
        var_coth1_dn7 = assign6700_e6977_d_n7;
        var_coth1_dn8 = assign6700_e6977_d_n8;

        let (assign6710_e6986, assign6710_e6986_d_n3, assign6710_e6986_d_n4, assign6710_e6986_d_n5, assign6710_e6986_d_n6, assign6710_e6986_d_n7, assign6710_e6986_d_n8,) = {
    if (var_guard80 == 0.0) {
        let assign6710_e6982: f64 = (0.5 * var_coth1);
        let assign6710_e6984: f64 = (assign6710_e6982 / var_q);
        (assign6710_e6984, ((((0.5 * var_coth1_dn3) * var_q) - (assign6710_e6982 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign6710_e6982 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign6710_e6982 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign6710_e6982 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign6710_e6982 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign6710_e6982 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign6710_e6986;
        var_t0_dn3 = assign6710_e6986_d_n3;
        var_t0_dn4 = assign6710_e6986_d_n4;
        var_t0_dn5 = assign6710_e6986_d_n5;
        var_t0_dn6 = assign6710_e6986_d_n6;
        var_t0_dn7 = assign6710_e6986_d_n7;
        var_t0_dn8 = assign6710_e6986_d_n8;

        let (assign6720_e6996, assign6720_e6996_d_n3, assign6720_e6996_d_n4, assign6720_e6996_d_n5, assign6720_e6996_d_n6, assign6720_e6996_d_n7, assign6720_e6996_d_n8,) = {
    if (var_guard80 == 0.0) {
        let assign6720_e6990: f64 = (-0.25);
        let assign6720_e6992: f64 = (assign6720_e6990 * var_t1);
        let assign6720_e6994: f64 = (assign6720_e6992 + var_t0);
        (assign6720_e6994, ((assign6720_e6990 * var_t1_dn3) + var_t0_dn3), ((assign6720_e6990 * var_t1_dn4) + var_t0_dn4), ((assign6720_e6990 * var_t1_dn5) + var_t0_dn5), ((assign6720_e6990 * var_t1_dn6) + var_t0_dn6), ((assign6720_e6990 * var_t1_dn7) + var_t0_dn7), ((assign6720_e6990 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign6720_e6996;
        var_dqcothqdqsqrt_dn3 = assign6720_e6996_d_n3;
        var_dqcothqdqsqrt_dn4 = assign6720_e6996_d_n4;
        var_dqcothqdqsqrt_dn5 = assign6720_e6996_d_n5;
        var_dqcothqdqsqrt_dn6 = assign6720_e6996_d_n6;
        var_dqcothqdqsqrt_dn7 = assign6720_e6996_d_n7;
        var_dqcothqdqsqrt_dn8 = assign6720_e6996_d_n8;

        let assign6730_e6999: f64 = (var_q * var_coth1);
        var_qcoth = assign6730_e6999;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign6740_e7002: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign6740_e7002;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign6750_e7005: f64 = (1.0 / var_t2);
        var_t3 = assign6750_e7005;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign6760_e7008: f64 = (var_xg2 - var_xg1);
        let assign6760_e7010: f64 = (assign6760_e7008 + var_q1);
        let assign6760_e7013: f64 = (var_qsqrt * var_t1);
        let assign6760_e7015: f64 = (assign6760_e7013 * var_t3);
        let assign6760_e7017: f64 = (assign6760_e7015 * var_t3);
        let assign6760_e7018: f64 = (assign6760_e7017).abs();
        let assign6760_e7019: f64 = (assign6760_e7018).ln();
        let assign6760_e7020: f64 = (assign6760_e7010 - assign6760_e7019);
        var_q2 = assign6760_e7020;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign6760_e7017 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign6760_e7013 * var_t3_dn3)) * var_t3) + (assign6760_e7015 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign6760_e7013 * var_t3_dn3)) * var_t3) + (assign6760_e7015 * var_t3_dn3))) } / assign6760_e7018));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign6760_e7017 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign6760_e7013 * var_t3_dn4)) * var_t3) + (assign6760_e7015 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign6760_e7013 * var_t3_dn4)) * var_t3) + (assign6760_e7015 * var_t3_dn4))) } / assign6760_e7018));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign6760_e7017 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign6760_e7013 * var_t3_dn5)) * var_t3) + (assign6760_e7015 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign6760_e7013 * var_t3_dn5)) * var_t3) + (assign6760_e7015 * var_t3_dn5))) } / assign6760_e7018));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign6760_e7017 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign6760_e7013 * var_t3_dn6)) * var_t3) + (assign6760_e7015 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign6760_e7013 * var_t3_dn6)) * var_t3) + (assign6760_e7015 * var_t3_dn6))) } / assign6760_e7018));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign6760_e7017 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign6760_e7013 * var_t3_dn7)) * var_t3) + (assign6760_e7015 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign6760_e7013 * var_t3_dn7)) * var_t3) + (assign6760_e7015 * var_t3_dn7))) } / assign6760_e7018));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign6760_e7017 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign6760_e7013 * var_t3_dn8)) * var_t3) + (assign6760_e7015 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign6760_e7013 * var_t3_dn8)) * var_t3) + (assign6760_e7015 * var_t3_dn8))) } / assign6760_e7018));

        let assign6770_e7024: f64 = (var_auxb1 + var_qcoth);
        let assign6770_e7027: f64 = (var_k2 * var_q2);
        let assign6770_e7029: f64 = (assign6770_e7027 + var_auxb1);
        let assign6770_e7030: f64 = (assign6770_e7024 * assign6770_e7029);
        let assign6770_e7031: f64 = (var_aaux + assign6770_e7030);
        var_f = assign6770_e7031;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign6770_e7029) + (assign6770_e7024 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign6770_e7029) + (assign6770_e7024 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign6770_e7029) + (assign6770_e7024 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign6770_e7029) + (assign6770_e7024 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign6770_e7029) + (assign6770_e7024 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign6770_e7029) + (assign6770_e7024 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign6780_e7034: f64 = (1.0 / var_qsqrt);
        let assign6780_e7036: f64 = (assign6780_e7034 - var_t0);
        var_dlogsinhqsqdqsqrt = assign6780_e7036;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign6790_e7038: f64 = (-2.0);
        let assign6790_e7040: f64 = (assign6790_e7038 * var_k1);
        let assign6790_e7042: f64 = (assign6790_e7040 * var_auxb1);
        let assign6790_e7044: f64 = (assign6790_e7042 + var_aaux);
        var_dqsqrt = assign6790_e7044;
        var_dqsqrt_dn3 = ((assign6790_e7040 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign6790_e7040 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign6790_e7040 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign6790_e7040 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign6790_e7040 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign6790_e7040 * var_auxb1_dn8) + var_aaux_dn8);

        let assign6800_e7047: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign6800_e7047;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign6810_e7049: f64 = (-1.0);
        let assign6810_e7052: f64 = (-var_k1);
        let assign6810_e7054: f64 = (assign6810_e7052 + var_dqcoth);
        let assign6810_e7056: f64 = (assign6810_e7054 * var_t3);
        let assign6810_e7057: f64 = (2.0 * assign6810_e7056);
        let assign6810_e7058: f64 = (assign6810_e7049 + assign6810_e7057);
        let assign6810_e7061: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign6810_e7062: f64 = (assign6810_e7058 - assign6810_e7061);
        var_dq2 = assign6810_e7062;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign6810_e7054 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign6810_e7054 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign6810_e7054 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign6810_e7054 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign6810_e7054 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign6810_e7054 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign6820_e7067: f64 = (var_auxb1 + var_t2);
        let assign6820_e7068: f64 = (var_k1 * assign6820_e7067);
        let assign6820_e7069: f64 = (var_aaux - assign6820_e7068);
        let assign6820_e7072: f64 = (var_auxb1 * var_dqcoth);
        let assign6820_e7073: f64 = (assign6820_e7069 + assign6820_e7072);
        let assign6820_e7077: f64 = (var_dq2 * var_t2);
        let assign6820_e7081: f64 = (var_dqcoth - var_k1);
        let assign6820_e7082: f64 = (var_q2 * assign6820_e7081);
        let assign6820_e7083: f64 = (assign6820_e7077 + assign6820_e7082);
        let assign6820_e7084: f64 = (var_k2 * assign6820_e7083);
        let assign6820_e7085: f64 = (assign6820_e7073 + assign6820_e7084);
        var_df = assign6820_e7085;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign6820_e7081) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign6820_e7081) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign6820_e7081) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign6820_e7081) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign6820_e7081) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign6820_e7081) + (var_q2 * var_dqcoth_dn8)))));

        let assign6830_e7087: f64 = (-var_f);
        let assign6830_e7089: f64 = (assign6830_e7087 / var_df);
        var_delta = assign6830_e7089;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign6830_e7087 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign6830_e7087 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign6830_e7087 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign6830_e7087 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign6830_e7087 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign6830_e7087 * var_df_dn8)) / (var_df * var_df));

        let assign6840_e7092: f64 = (var_phi1 + var_delta);
        var_phi1 = assign6840_e7092;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign6850_e7095: f64 = (var_xg1 - var_phi1);
        var_q1 = assign6850_e7095;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign6860_e7098: f64 = (var_k1 * var_q1);
        var_auxb1 = assign6860_e7098;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign6870_e7100: f64 = (-var_a0);
        let assign6870_e7102: f64 = (var_phi1).exp();
        let assign6870_e7103: f64 = (assign6870_e7100 * assign6870_e7102);
        var_aaux = assign6870_e7103;
        var_aaux_dn3 = (((-var_a0_dn3) * assign6870_e7102) + (assign6870_e7100 * (assign6870_e7102 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign6870_e7102) + (assign6870_e7100 * (assign6870_e7102 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign6870_e7102) + (assign6870_e7100 * (assign6870_e7102 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign6870_e7102) + (assign6870_e7100 * (assign6870_e7102 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign6870_e7102) + (assign6870_e7100 * (assign6870_e7102 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign6870_e7102) + (assign6870_e7100 * (assign6870_e7102 * var_phi1_dn8)));

        let assign6880_e7106: f64 = (var_auxb1 * var_auxb1);
        let assign6880_e7108: f64 = (assign6880_e7106 + var_aaux);
        var_qsqrt = assign6880_e7108;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign6890_e7111: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard81 = assign6890_e7111;

        *var_aaux_slot = var_aaux;
        *var_aaux_dn3_slot = var_aaux_dn3;
        *var_aaux_dn4_slot = var_aaux_dn4;
        *var_aaux_dn5_slot = var_aaux_dn5;
        *var_aaux_dn6_slot = var_aaux_dn6;
        *var_aaux_dn7_slot = var_aaux_dn7;
        *var_aaux_dn8_slot = var_aaux_dn8;
        *var_auxb1_slot = var_auxb1;
        *var_auxb1_dn3_slot = var_auxb1_dn3;
        *var_auxb1_dn4_slot = var_auxb1_dn4;
        *var_auxb1_dn5_slot = var_auxb1_dn5;
        *var_auxb1_dn6_slot = var_auxb1_dn6;
        *var_auxb1_dn7_slot = var_auxb1_dn7;
        *var_auxb1_dn8_slot = var_auxb1_dn8;
        *var_coth1_slot = var_coth1;
        *var_coth1_dn3_slot = var_coth1_dn3;
        *var_coth1_dn4_slot = var_coth1_dn4;
        *var_coth1_dn5_slot = var_coth1_dn5;
        *var_coth1_dn6_slot = var_coth1_dn6;
        *var_coth1_dn7_slot = var_coth1_dn7;
        *var_coth1_dn8_slot = var_coth1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_df_slot = var_df;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_dn6_slot = var_df_dn6;
        *var_df_dn7_slot = var_df_dn7;
        *var_df_dn8_slot = var_df_dn8;
        *var_dlogsinhqsqdqsqrt_slot = var_dlogsinhqsqdqsqrt;
        *var_dlogsinhqsqdqsqrt_dn3_slot = var_dlogsinhqsqdqsqrt_dn3;
        *var_dlogsinhqsqdqsqrt_dn4_slot = var_dlogsinhqsqdqsqrt_dn4;
        *var_dlogsinhqsqdqsqrt_dn5_slot = var_dlogsinhqsqdqsqrt_dn5;
        *var_dlogsinhqsqdqsqrt_dn6_slot = var_dlogsinhqsqdqsqrt_dn6;
        *var_dlogsinhqsqdqsqrt_dn7_slot = var_dlogsinhqsqdqsqrt_dn7;
        *var_dlogsinhqsqdqsqrt_dn8_slot = var_dlogsinhqsqdqsqrt_dn8;
        *var_dq2_slot = var_dq2;
        *var_dq2_dn3_slot = var_dq2_dn3;
        *var_dq2_dn4_slot = var_dq2_dn4;
        *var_dq2_dn5_slot = var_dq2_dn5;
        *var_dq2_dn6_slot = var_dq2_dn6;
        *var_dq2_dn7_slot = var_dq2_dn7;
        *var_dq2_dn8_slot = var_dq2_dn8;
        *var_dqcoth_slot = var_dqcoth;
        *var_dqcoth_dn3_slot = var_dqcoth_dn3;
        *var_dqcoth_dn4_slot = var_dqcoth_dn4;
        *var_dqcoth_dn5_slot = var_dqcoth_dn5;
        *var_dqcoth_dn6_slot = var_dqcoth_dn6;
        *var_dqcoth_dn7_slot = var_dqcoth_dn7;
        *var_dqcoth_dn8_slot = var_dqcoth_dn8;
        *var_dqcothqdqsqrt_slot = var_dqcothqdqsqrt;
        *var_dqcothqdqsqrt_dn3_slot = var_dqcothqdqsqrt_dn3;
        *var_dqcothqdqsqrt_dn4_slot = var_dqcothqdqsqrt_dn4;
        *var_dqcothqdqsqrt_dn5_slot = var_dqcothqdqsqrt_dn5;
        *var_dqcothqdqsqrt_dn6_slot = var_dqcothqdqsqrt_dn6;
        *var_dqcothqdqsqrt_dn7_slot = var_dqcothqdqsqrt_dn7;
        *var_dqcothqdqsqrt_dn8_slot = var_dqcothqdqsqrt_dn8;
        *var_dqsqrt_slot = var_dqsqrt;
        *var_dqsqrt_dn3_slot = var_dqsqrt_dn3;
        *var_dqsqrt_dn4_slot = var_dqsqrt_dn4;
        *var_dqsqrt_dn5_slot = var_dqsqrt_dn5;
        *var_dqsqrt_dn6_slot = var_dqsqrt_dn6;
        *var_dqsqrt_dn7_slot = var_dqsqrt_dn7;
        *var_dqsqrt_dn8_slot = var_dqsqrt_dn8;
        *var_f_slot = var_f;
        *var_f_dn3_slot = var_f_dn3;
        *var_f_dn4_slot = var_f_dn4;
        *var_f_dn5_slot = var_f_dn5;
        *var_f_dn6_slot = var_f_dn6;
        *var_f_dn7_slot = var_f_dn7;
        *var_f_dn8_slot = var_f_dn8;
        *var_guard80_slot = var_guard80;
        *var_guard81_slot = var_guard81;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
    }

    pub(super) fn stamp_transient_block_13(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_aaux: f64,
        var_aaux_dn3: f64,
        var_aaux_dn4: f64,
        var_aaux_dn5: f64,
        var_aaux_dn6: f64,
        var_aaux_dn7: f64,
        var_aaux_dn8: f64,
        var_auxb1: f64,
        var_auxb1_dn3: f64,
        var_auxb1_dn4: f64,
        var_auxb1_dn5: f64,
        var_auxb1_dn6: f64,
        var_auxb1_dn7: f64,
        var_auxb1_dn8: f64,
        var_guard81: f64,
        var_k1: f64,
        var_k1_2: f64,
        var_k2: f64,
        var_xg1: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_coth1_slot: &mut f64,
        var_coth1_dn3_slot: &mut f64,
        var_coth1_dn4_slot: &mut f64,
        var_coth1_dn5_slot: &mut f64,
        var_coth1_dn6_slot: &mut f64,
        var_coth1_dn7_slot: &mut f64,
        var_coth1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_df_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_dn6_slot: &mut f64,
        var_df_dn7_slot: &mut f64,
        var_df_dn8_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn3_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn4_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn5_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn6_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn7_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn8_slot: &mut f64,
        var_dq2_slot: &mut f64,
        var_dq2_dn3_slot: &mut f64,
        var_dq2_dn4_slot: &mut f64,
        var_dq2_dn5_slot: &mut f64,
        var_dq2_dn6_slot: &mut f64,
        var_dq2_dn7_slot: &mut f64,
        var_dq2_dn8_slot: &mut f64,
        var_dqcoth_slot: &mut f64,
        var_dqcoth_dn3_slot: &mut f64,
        var_dqcoth_dn4_slot: &mut f64,
        var_dqcoth_dn5_slot: &mut f64,
        var_dqcoth_dn6_slot: &mut f64,
        var_dqcoth_dn7_slot: &mut f64,
        var_dqcoth_dn8_slot: &mut f64,
        var_dqcothqdqsqrt_slot: &mut f64,
        var_dqcothqdqsqrt_dn3_slot: &mut f64,
        var_dqcothqdqsqrt_dn4_slot: &mut f64,
        var_dqcothqdqsqrt_dn5_slot: &mut f64,
        var_dqcothqdqsqrt_dn6_slot: &mut f64,
        var_dqcothqdqsqrt_dn7_slot: &mut f64,
        var_dqcothqdqsqrt_dn8_slot: &mut f64,
        var_dqsqrt_slot: &mut f64,
        var_dqsqrt_dn3_slot: &mut f64,
        var_dqsqrt_dn4_slot: &mut f64,
        var_dqsqrt_dn5_slot: &mut f64,
        var_dqsqrt_dn6_slot: &mut f64,
        var_dqsqrt_dn7_slot: &mut f64,
        var_dqsqrt_dn8_slot: &mut f64,
        var_f_slot: &mut f64,
        var_f_dn3_slot: &mut f64,
        var_f_dn4_slot: &mut f64,
        var_f_dn5_slot: &mut f64,
        var_f_dn6_slot: &mut f64,
        var_f_dn7_slot: &mut f64,
        var_f_dn8_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
    ) {
        let mut var_coth1: f64 = *var_coth1_slot;
        let mut var_coth1_dn3: f64 = *var_coth1_dn3_slot;
        let mut var_coth1_dn4: f64 = *var_coth1_dn4_slot;
        let mut var_coth1_dn5: f64 = *var_coth1_dn5_slot;
        let mut var_coth1_dn6: f64 = *var_coth1_dn6_slot;
        let mut var_coth1_dn7: f64 = *var_coth1_dn7_slot;
        let mut var_coth1_dn8: f64 = *var_coth1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_dn6: f64 = *var_df_dn6_slot;
        let mut var_df_dn7: f64 = *var_df_dn7_slot;
        let mut var_df_dn8: f64 = *var_df_dn8_slot;
        let mut var_dlogsinhqsqdqsqrt: f64 = *var_dlogsinhqsqdqsqrt_slot;
        let mut var_dlogsinhqsqdqsqrt_dn3: f64 = *var_dlogsinhqsqdqsqrt_dn3_slot;
        let mut var_dlogsinhqsqdqsqrt_dn4: f64 = *var_dlogsinhqsqdqsqrt_dn4_slot;
        let mut var_dlogsinhqsqdqsqrt_dn5: f64 = *var_dlogsinhqsqdqsqrt_dn5_slot;
        let mut var_dlogsinhqsqdqsqrt_dn6: f64 = *var_dlogsinhqsqdqsqrt_dn6_slot;
        let mut var_dlogsinhqsqdqsqrt_dn7: f64 = *var_dlogsinhqsqdqsqrt_dn7_slot;
        let mut var_dlogsinhqsqdqsqrt_dn8: f64 = *var_dlogsinhqsqdqsqrt_dn8_slot;
        let mut var_dq2: f64 = *var_dq2_slot;
        let mut var_dq2_dn3: f64 = *var_dq2_dn3_slot;
        let mut var_dq2_dn4: f64 = *var_dq2_dn4_slot;
        let mut var_dq2_dn5: f64 = *var_dq2_dn5_slot;
        let mut var_dq2_dn6: f64 = *var_dq2_dn6_slot;
        let mut var_dq2_dn7: f64 = *var_dq2_dn7_slot;
        let mut var_dq2_dn8: f64 = *var_dq2_dn8_slot;
        let mut var_dqcoth: f64 = *var_dqcoth_slot;
        let mut var_dqcoth_dn3: f64 = *var_dqcoth_dn3_slot;
        let mut var_dqcoth_dn4: f64 = *var_dqcoth_dn4_slot;
        let mut var_dqcoth_dn5: f64 = *var_dqcoth_dn5_slot;
        let mut var_dqcoth_dn6: f64 = *var_dqcoth_dn6_slot;
        let mut var_dqcoth_dn7: f64 = *var_dqcoth_dn7_slot;
        let mut var_dqcoth_dn8: f64 = *var_dqcoth_dn8_slot;
        let mut var_dqcothqdqsqrt: f64 = *var_dqcothqdqsqrt_slot;
        let mut var_dqcothqdqsqrt_dn3: f64 = *var_dqcothqdqsqrt_dn3_slot;
        let mut var_dqcothqdqsqrt_dn4: f64 = *var_dqcothqdqsqrt_dn4_slot;
        let mut var_dqcothqdqsqrt_dn5: f64 = *var_dqcothqdqsqrt_dn5_slot;
        let mut var_dqcothqdqsqrt_dn6: f64 = *var_dqcothqdqsqrt_dn6_slot;
        let mut var_dqcothqdqsqrt_dn7: f64 = *var_dqcothqdqsqrt_dn7_slot;
        let mut var_dqcothqdqsqrt_dn8: f64 = *var_dqcothqdqsqrt_dn8_slot;
        let mut var_dqsqrt: f64 = *var_dqsqrt_slot;
        let mut var_dqsqrt_dn3: f64 = *var_dqsqrt_dn3_slot;
        let mut var_dqsqrt_dn4: f64 = *var_dqsqrt_dn4_slot;
        let mut var_dqsqrt_dn5: f64 = *var_dqsqrt_dn5_slot;
        let mut var_dqsqrt_dn6: f64 = *var_dqsqrt_dn6_slot;
        let mut var_dqsqrt_dn7: f64 = *var_dqsqrt_dn7_slot;
        let mut var_dqsqrt_dn8: f64 = *var_dqsqrt_dn8_slot;
        let mut var_f: f64 = *var_f_slot;
        let mut var_f_dn3: f64 = *var_f_dn3_slot;
        let mut var_f_dn4: f64 = *var_f_dn4_slot;
        let mut var_f_dn5: f64 = *var_f_dn5_slot;
        let mut var_f_dn6: f64 = *var_f_dn6_slot;
        let mut var_f_dn7: f64 = *var_f_dn7_slot;
        let mut var_f_dn8: f64 = *var_f_dn8_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;

        let (assign6900_e7117, assign6900_e7117_d_n3, assign6900_e7117_d_n4, assign6900_e7117_d_n5, assign6900_e7117_d_n6, assign6900_e7117_d_n7, assign6900_e7117_d_n8,) = {
    if (var_guard81 != 0.0) {
        let assign6900_e7114: f64 = (-var_qsqrt);
        let assign6900_e7115: f64 = (assign6900_e7114).sqrt();
        (assign6900_e7115, ((-var_qsqrt_dn3) / (2.0 * assign6900_e7115)), ((-var_qsqrt_dn4) / (2.0 * assign6900_e7115)), ((-var_qsqrt_dn5) / (2.0 * assign6900_e7115)), ((-var_qsqrt_dn6) / (2.0 * assign6900_e7115)), ((-var_qsqrt_dn7) / (2.0 * assign6900_e7115)), ((-var_qsqrt_dn8) / (2.0 * assign6900_e7115)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign6900_e7117;
        var_q_dn3 = assign6900_e7117_d_n3;
        var_q_dn4 = assign6900_e7117_d_n4;
        var_q_dn5 = assign6900_e7117_d_n5;
        var_q_dn6 = assign6900_e7117_d_n6;
        var_q_dn7 = assign6900_e7117_d_n7;
        var_q_dn8 = assign6900_e7117_d_n8;

        let (assign6910_e7126, assign6910_e7126_d_n3, assign6910_e7126_d_n4, assign6910_e7126_d_n5, assign6910_e7126_d_n6, assign6910_e7126_d_n7, assign6910_e7126_d_n8,) = {
    if (var_guard81 != 0.0) {
        let assign6910_e7122: f64 = (0.5 * var_q);
        let assign6910_e7123: f64 = (assign6910_e7122).sin();
        let assign6910_e7124: f64 = (1.0 / assign6910_e7123);
        (assign6910_e7124, (-(((assign6910_e7122).cos() * (0.5 * var_q_dn3)) / (assign6910_e7123 * assign6910_e7123))), (-(((assign6910_e7122).cos() * (0.5 * var_q_dn4)) / (assign6910_e7123 * assign6910_e7123))), (-(((assign6910_e7122).cos() * (0.5 * var_q_dn5)) / (assign6910_e7123 * assign6910_e7123))), (-(((assign6910_e7122).cos() * (0.5 * var_q_dn6)) / (assign6910_e7123 * assign6910_e7123))), (-(((assign6910_e7122).cos() * (0.5 * var_q_dn7)) / (assign6910_e7123 * assign6910_e7123))), (-(((assign6910_e7122).cos() * (0.5 * var_q_dn8)) / (assign6910_e7123 * assign6910_e7123))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign6910_e7126;
        var_csc1_dn3 = assign6910_e7126_d_n3;
        var_csc1_dn4 = assign6910_e7126_d_n4;
        var_csc1_dn5 = assign6910_e7126_d_n5;
        var_csc1_dn6 = assign6910_e7126_d_n6;
        var_csc1_dn7 = assign6910_e7126_d_n7;
        var_csc1_dn8 = assign6910_e7126_d_n8;

        let (assign6920_e7132, assign6920_e7132_d_n3, assign6920_e7132_d_n4, assign6920_e7132_d_n5, assign6920_e7132_d_n6, assign6920_e7132_d_n7, assign6920_e7132_d_n8,) = {
    if (var_guard81 != 0.0) {
        let assign6920_e7130: f64 = (var_csc1 * var_csc1);
        (assign6920_e7130, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign6920_e7132;
        var_t1_dn3 = assign6920_e7132_d_n3;
        var_t1_dn4 = assign6920_e7132_d_n4;
        var_t1_dn5 = assign6920_e7132_d_n5;
        var_t1_dn6 = assign6920_e7132_d_n6;
        var_t1_dn7 = assign6920_e7132_d_n7;
        var_t1_dn8 = assign6920_e7132_d_n8;

        let (assign6930_e7141, assign6930_e7141_d_n3, assign6930_e7141_d_n4, assign6930_e7141_d_n5, assign6930_e7141_d_n6, assign6930_e7141_d_n7, assign6930_e7141_d_n8,) = {
    if (var_guard81 != 0.0) {
        let assign6930_e7136: f64 = (0.5 * var_q);
        let assign6930_e7137: f64 = (assign6930_e7136).cos();
        let assign6930_e7139: f64 = (assign6930_e7137 * var_csc1);
        (assign6930_e7139, (((-(assign6930_e7136).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign6930_e7137 * var_csc1_dn3)), (((-(assign6930_e7136).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign6930_e7137 * var_csc1_dn4)), (((-(assign6930_e7136).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign6930_e7137 * var_csc1_dn5)), (((-(assign6930_e7136).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign6930_e7137 * var_csc1_dn6)), (((-(assign6930_e7136).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign6930_e7137 * var_csc1_dn7)), (((-(assign6930_e7136).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign6930_e7137 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign6930_e7141;
        var_coth1_dn3 = assign6930_e7141_d_n3;
        var_coth1_dn4 = assign6930_e7141_d_n4;
        var_coth1_dn5 = assign6930_e7141_d_n5;
        var_coth1_dn6 = assign6930_e7141_d_n6;
        var_coth1_dn7 = assign6930_e7141_d_n7;
        var_coth1_dn8 = assign6930_e7141_d_n8;

        let (assign6940_e7150, assign6940_e7150_d_n3, assign6940_e7150_d_n4, assign6940_e7150_d_n5, assign6940_e7150_d_n6, assign6940_e7150_d_n7, assign6940_e7150_d_n8,) = {
    if (var_guard81 != 0.0) {
        let assign6940_e7144: f64 = (-0.5);
        let assign6940_e7146: f64 = (assign6940_e7144 * var_coth1);
        let assign6940_e7148: f64 = (assign6940_e7146 / var_q);
        (assign6940_e7148, ((((assign6940_e7144 * var_coth1_dn3) * var_q) - (assign6940_e7146 * var_q_dn3)) / (var_q * var_q)), ((((assign6940_e7144 * var_coth1_dn4) * var_q) - (assign6940_e7146 * var_q_dn4)) / (var_q * var_q)), ((((assign6940_e7144 * var_coth1_dn5) * var_q) - (assign6940_e7146 * var_q_dn5)) / (var_q * var_q)), ((((assign6940_e7144 * var_coth1_dn6) * var_q) - (assign6940_e7146 * var_q_dn6)) / (var_q * var_q)), ((((assign6940_e7144 * var_coth1_dn7) * var_q) - (assign6940_e7146 * var_q_dn7)) / (var_q * var_q)), ((((assign6940_e7144 * var_coth1_dn8) * var_q) - (assign6940_e7146 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign6940_e7150;
        var_t0_dn3 = assign6940_e7150_d_n3;
        var_t0_dn4 = assign6940_e7150_d_n4;
        var_t0_dn5 = assign6940_e7150_d_n5;
        var_t0_dn6 = assign6940_e7150_d_n6;
        var_t0_dn7 = assign6940_e7150_d_n7;
        var_t0_dn8 = assign6940_e7150_d_n8;

        let (assign6950_e7158, assign6950_e7158_d_n3, assign6950_e7158_d_n4, assign6950_e7158_d_n5, assign6950_e7158_d_n6, assign6950_e7158_d_n7, assign6950_e7158_d_n8,) = {
    if (var_guard81 != 0.0) {
        let assign6950_e7154: f64 = (0.25 * var_t1);
        let assign6950_e7156: f64 = (assign6950_e7154 + var_t0);
        (assign6950_e7156, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign6950_e7158;
        var_dqcothqdqsqrt_dn3 = assign6950_e7158_d_n3;
        var_dqcothqdqsqrt_dn4 = assign6950_e7158_d_n4;
        var_dqcothqdqsqrt_dn5 = assign6950_e7158_d_n5;
        var_dqcothqdqsqrt_dn6 = assign6950_e7158_d_n6;
        var_dqcothqdqsqrt_dn7 = assign6950_e7158_d_n7;
        var_dqcothqdqsqrt_dn8 = assign6950_e7158_d_n8;

        let (assign6960_e7164, assign6960_e7164_d_n3, assign6960_e7164_d_n4, assign6960_e7164_d_n5, assign6960_e7164_d_n6, assign6960_e7164_d_n7, assign6960_e7164_d_n8,) = {
    if (var_guard81 == 0.0) {
        let assign6960_e7162: f64 = (var_qsqrt).sqrt();
        (assign6960_e7162, (var_qsqrt_dn3 / (2.0 * assign6960_e7162)), (var_qsqrt_dn4 / (2.0 * assign6960_e7162)), (var_qsqrt_dn5 / (2.0 * assign6960_e7162)), (var_qsqrt_dn6 / (2.0 * assign6960_e7162)), (var_qsqrt_dn7 / (2.0 * assign6960_e7162)), (var_qsqrt_dn8 / (2.0 * assign6960_e7162)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign6960_e7164;
        var_q_dn3 = assign6960_e7164_d_n3;
        var_q_dn4 = assign6960_e7164_d_n4;
        var_q_dn5 = assign6960_e7164_d_n5;
        var_q_dn6 = assign6960_e7164_d_n6;
        var_q_dn7 = assign6960_e7164_d_n7;
        var_q_dn8 = assign6960_e7164_d_n8;

        let (assign6970_e7174, assign6970_e7174_d_n3, assign6970_e7174_d_n4, assign6970_e7174_d_n5, assign6970_e7174_d_n6, assign6970_e7174_d_n7, assign6970_e7174_d_n8,) = {
    if (var_guard81 == 0.0) {
        let assign6970_e7170: f64 = (0.5 * var_q);
        let assign6970_e7171: f64 = (assign6970_e7170).sinh();
        let assign6970_e7172: f64 = (1.0 / assign6970_e7171);
        (assign6970_e7172, (-(((assign6970_e7170).cosh() * (0.5 * var_q_dn3)) / (assign6970_e7171 * assign6970_e7171))), (-(((assign6970_e7170).cosh() * (0.5 * var_q_dn4)) / (assign6970_e7171 * assign6970_e7171))), (-(((assign6970_e7170).cosh() * (0.5 * var_q_dn5)) / (assign6970_e7171 * assign6970_e7171))), (-(((assign6970_e7170).cosh() * (0.5 * var_q_dn6)) / (assign6970_e7171 * assign6970_e7171))), (-(((assign6970_e7170).cosh() * (0.5 * var_q_dn7)) / (assign6970_e7171 * assign6970_e7171))), (-(((assign6970_e7170).cosh() * (0.5 * var_q_dn8)) / (assign6970_e7171 * assign6970_e7171))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign6970_e7174;
        var_csc1_dn3 = assign6970_e7174_d_n3;
        var_csc1_dn4 = assign6970_e7174_d_n4;
        var_csc1_dn5 = assign6970_e7174_d_n5;
        var_csc1_dn6 = assign6970_e7174_d_n6;
        var_csc1_dn7 = assign6970_e7174_d_n7;
        var_csc1_dn8 = assign6970_e7174_d_n8;

        let (assign6980_e7181, assign6980_e7181_d_n3, assign6980_e7181_d_n4, assign6980_e7181_d_n5, assign6980_e7181_d_n6, assign6980_e7181_d_n7, assign6980_e7181_d_n8,) = {
    if (var_guard81 == 0.0) {
        let assign6980_e7179: f64 = (var_csc1 * var_csc1);
        (assign6980_e7179, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign6980_e7181;
        var_t1_dn3 = assign6980_e7181_d_n3;
        var_t1_dn4 = assign6980_e7181_d_n4;
        var_t1_dn5 = assign6980_e7181_d_n5;
        var_t1_dn6 = assign6980_e7181_d_n6;
        var_t1_dn7 = assign6980_e7181_d_n7;
        var_t1_dn8 = assign6980_e7181_d_n8;

        let (assign6990_e7189, assign6990_e7189_d_n3, assign6990_e7189_d_n4, assign6990_e7189_d_n5, assign6990_e7189_d_n6, assign6990_e7189_d_n7, assign6990_e7189_d_n8,) = {
    if (var_guard81 == 0.0) {
        let assign6990_e7186: f64 = (1.0 + var_t1);
        let assign6990_e7187: f64 = (assign6990_e7186).sqrt();
        (assign6990_e7187, (var_t1_dn3 / (2.0 * assign6990_e7187)), (var_t1_dn4 / (2.0 * assign6990_e7187)), (var_t1_dn5 / (2.0 * assign6990_e7187)), (var_t1_dn6 / (2.0 * assign6990_e7187)), (var_t1_dn7 / (2.0 * assign6990_e7187)), (var_t1_dn8 / (2.0 * assign6990_e7187)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign6990_e7189;
        var_coth1_dn3 = assign6990_e7189_d_n3;
        var_coth1_dn4 = assign6990_e7189_d_n4;
        var_coth1_dn5 = assign6990_e7189_d_n5;
        var_coth1_dn6 = assign6990_e7189_d_n6;
        var_coth1_dn7 = assign6990_e7189_d_n7;
        var_coth1_dn8 = assign6990_e7189_d_n8;

        let (assign7000_e7198, assign7000_e7198_d_n3, assign7000_e7198_d_n4, assign7000_e7198_d_n5, assign7000_e7198_d_n6, assign7000_e7198_d_n7, assign7000_e7198_d_n8,) = {
    if (var_guard81 == 0.0) {
        let assign7000_e7194: f64 = (0.5 * var_coth1);
        let assign7000_e7196: f64 = (assign7000_e7194 / var_q);
        (assign7000_e7196, ((((0.5 * var_coth1_dn3) * var_q) - (assign7000_e7194 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign7000_e7194 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign7000_e7194 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign7000_e7194 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign7000_e7194 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign7000_e7194 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign7000_e7198;
        var_t0_dn3 = assign7000_e7198_d_n3;
        var_t0_dn4 = assign7000_e7198_d_n4;
        var_t0_dn5 = assign7000_e7198_d_n5;
        var_t0_dn6 = assign7000_e7198_d_n6;
        var_t0_dn7 = assign7000_e7198_d_n7;
        var_t0_dn8 = assign7000_e7198_d_n8;

        let (assign7010_e7208, assign7010_e7208_d_n3, assign7010_e7208_d_n4, assign7010_e7208_d_n5, assign7010_e7208_d_n6, assign7010_e7208_d_n7, assign7010_e7208_d_n8,) = {
    if (var_guard81 == 0.0) {
        let assign7010_e7202: f64 = (-0.25);
        let assign7010_e7204: f64 = (assign7010_e7202 * var_t1);
        let assign7010_e7206: f64 = (assign7010_e7204 + var_t0);
        (assign7010_e7206, ((assign7010_e7202 * var_t1_dn3) + var_t0_dn3), ((assign7010_e7202 * var_t1_dn4) + var_t0_dn4), ((assign7010_e7202 * var_t1_dn5) + var_t0_dn5), ((assign7010_e7202 * var_t1_dn6) + var_t0_dn6), ((assign7010_e7202 * var_t1_dn7) + var_t0_dn7), ((assign7010_e7202 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign7010_e7208;
        var_dqcothqdqsqrt_dn3 = assign7010_e7208_d_n3;
        var_dqcothqdqsqrt_dn4 = assign7010_e7208_d_n4;
        var_dqcothqdqsqrt_dn5 = assign7010_e7208_d_n5;
        var_dqcothqdqsqrt_dn6 = assign7010_e7208_d_n6;
        var_dqcothqdqsqrt_dn7 = assign7010_e7208_d_n7;
        var_dqcothqdqsqrt_dn8 = assign7010_e7208_d_n8;

        let assign7020_e7211: f64 = (var_q * var_coth1);
        var_qcoth = assign7020_e7211;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign7030_e7214: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign7030_e7214;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign7040_e7217: f64 = (1.0 / var_t2);
        var_t3 = assign7040_e7217;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign7050_e7220: f64 = (var_xg2 - var_xg1);
        let assign7050_e7222: f64 = (assign7050_e7220 + var_q1);
        let assign7050_e7225: f64 = (var_qsqrt * var_t1);
        let assign7050_e7227: f64 = (assign7050_e7225 * var_t3);
        let assign7050_e7229: f64 = (assign7050_e7227 * var_t3);
        let assign7050_e7230: f64 = (assign7050_e7229).abs();
        let assign7050_e7231: f64 = (assign7050_e7230).ln();
        let assign7050_e7232: f64 = (assign7050_e7222 - assign7050_e7231);
        var_q2 = assign7050_e7232;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign7050_e7229 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign7050_e7225 * var_t3_dn3)) * var_t3) + (assign7050_e7227 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign7050_e7225 * var_t3_dn3)) * var_t3) + (assign7050_e7227 * var_t3_dn3))) } / assign7050_e7230));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign7050_e7229 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign7050_e7225 * var_t3_dn4)) * var_t3) + (assign7050_e7227 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign7050_e7225 * var_t3_dn4)) * var_t3) + (assign7050_e7227 * var_t3_dn4))) } / assign7050_e7230));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign7050_e7229 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign7050_e7225 * var_t3_dn5)) * var_t3) + (assign7050_e7227 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign7050_e7225 * var_t3_dn5)) * var_t3) + (assign7050_e7227 * var_t3_dn5))) } / assign7050_e7230));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign7050_e7229 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign7050_e7225 * var_t3_dn6)) * var_t3) + (assign7050_e7227 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign7050_e7225 * var_t3_dn6)) * var_t3) + (assign7050_e7227 * var_t3_dn6))) } / assign7050_e7230));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign7050_e7229 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign7050_e7225 * var_t3_dn7)) * var_t3) + (assign7050_e7227 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign7050_e7225 * var_t3_dn7)) * var_t3) + (assign7050_e7227 * var_t3_dn7))) } / assign7050_e7230));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign7050_e7229 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign7050_e7225 * var_t3_dn8)) * var_t3) + (assign7050_e7227 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign7050_e7225 * var_t3_dn8)) * var_t3) + (assign7050_e7227 * var_t3_dn8))) } / assign7050_e7230));

        let assign7060_e7236: f64 = (var_auxb1 + var_qcoth);
        let assign7060_e7239: f64 = (var_k2 * var_q2);
        let assign7060_e7241: f64 = (assign7060_e7239 + var_auxb1);
        let assign7060_e7242: f64 = (assign7060_e7236 * assign7060_e7241);
        let assign7060_e7243: f64 = (var_aaux + assign7060_e7242);
        var_f = assign7060_e7243;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign7060_e7241) + (assign7060_e7236 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign7060_e7241) + (assign7060_e7236 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign7060_e7241) + (assign7060_e7236 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign7060_e7241) + (assign7060_e7236 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign7060_e7241) + (assign7060_e7236 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign7060_e7241) + (assign7060_e7236 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign7070_e7246: f64 = (1.0 / var_qsqrt);
        let assign7070_e7248: f64 = (assign7070_e7246 - var_t0);
        var_dlogsinhqsqdqsqrt = assign7070_e7248;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign7080_e7250: f64 = (-2.0);
        let assign7080_e7252: f64 = (assign7080_e7250 * var_k1);
        let assign7080_e7254: f64 = (assign7080_e7252 * var_auxb1);
        let assign7080_e7256: f64 = (assign7080_e7254 + var_aaux);
        var_dqsqrt = assign7080_e7256;
        var_dqsqrt_dn3 = ((assign7080_e7252 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign7080_e7252 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign7080_e7252 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign7080_e7252 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign7080_e7252 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign7080_e7252 * var_auxb1_dn8) + var_aaux_dn8);

        let assign7090_e7259: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign7090_e7259;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign7100_e7261: f64 = (-1.0);
        let assign7100_e7264: f64 = (-var_k1);
        let assign7100_e7266: f64 = (assign7100_e7264 + var_dqcoth);
        let assign7100_e7268: f64 = (assign7100_e7266 * var_t3);
        let assign7100_e7269: f64 = (2.0 * assign7100_e7268);
        let assign7100_e7270: f64 = (assign7100_e7261 + assign7100_e7269);
        let assign7100_e7273: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign7100_e7274: f64 = (assign7100_e7270 - assign7100_e7273);
        var_dq2 = assign7100_e7274;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign7100_e7266 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign7100_e7266 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign7100_e7266 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign7100_e7266 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign7100_e7266 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign7100_e7266 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign7110_e7279: f64 = (var_auxb1 + var_t2);
        let assign7110_e7280: f64 = (var_k1 * assign7110_e7279);
        let assign7110_e7281: f64 = (var_aaux - assign7110_e7280);
        let assign7110_e7284: f64 = (var_auxb1 * var_dqcoth);
        let assign7110_e7285: f64 = (assign7110_e7281 + assign7110_e7284);
        let assign7110_e7289: f64 = (var_dq2 * var_t2);
        let assign7110_e7293: f64 = (var_dqcoth - var_k1);
        let assign7110_e7294: f64 = (var_q2 * assign7110_e7293);
        let assign7110_e7295: f64 = (assign7110_e7289 + assign7110_e7294);
        let assign7110_e7296: f64 = (var_k2 * assign7110_e7295);
        let assign7110_e7297: f64 = (assign7110_e7285 + assign7110_e7296);
        var_df = assign7110_e7297;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign7110_e7293) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign7110_e7293) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign7110_e7293) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign7110_e7293) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign7110_e7293) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign7110_e7293) + (var_q2 * var_dqcoth_dn8)))));

        let assign7120_e7299: f64 = (-var_f);
        let assign7120_e7301: f64 = (assign7120_e7299 / var_df);
        var_delta = assign7120_e7301;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign7120_e7299 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign7120_e7299 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign7120_e7299 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign7120_e7299 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign7120_e7299 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign7120_e7299 * var_df_dn8)) / (var_df * var_df));

        let assign7130_e7304: f64 = (var_phi1 + var_delta);
        var_phi1 = assign7130_e7304;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign7140_e7307: f64 = (var_xg1 - var_phi1);
        var_q1 = assign7140_e7307;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign7150_e7310: f64 = (var_phi1).exp();
        let assign7150_e7311: f64 = (var_a0 * assign7150_e7310);
        var_t0 = assign7150_e7311;
        var_t0_dn3 = ((var_a0_dn3 * assign7150_e7310) + (var_a0 * (assign7150_e7310 * var_phi1_dn3)));
        var_t0_dn4 = ((var_a0_dn4 * assign7150_e7310) + (var_a0 * (assign7150_e7310 * var_phi1_dn4)));
        var_t0_dn5 = ((var_a0_dn5 * assign7150_e7310) + (var_a0 * (assign7150_e7310 * var_phi1_dn5)));
        var_t0_dn6 = ((var_a0_dn6 * assign7150_e7310) + (var_a0 * (assign7150_e7310 * var_phi1_dn6)));
        var_t0_dn7 = ((var_a0_dn7 * assign7150_e7310) + (var_a0 * (assign7150_e7310 * var_phi1_dn7)));
        var_t0_dn8 = ((var_a0_dn8 * assign7150_e7310) + (var_a0 * (assign7150_e7310 * var_phi1_dn8)));

        let assign7160_e7314: f64 = (var_k1_2 * var_q1);
        let assign7160_e7316: f64 = (assign7160_e7314 * var_q1);
        let assign7160_e7318: f64 = (assign7160_e7316 - var_t0);
        var_qsqrt = assign7160_e7318;
        var_qsqrt_dn3 = ((((var_k1_2 * var_q1_dn3) * var_q1) + (assign7160_e7314 * var_q1_dn3)) - var_t0_dn3);
        var_qsqrt_dn4 = ((((var_k1_2 * var_q1_dn4) * var_q1) + (assign7160_e7314 * var_q1_dn4)) - var_t0_dn4);
        var_qsqrt_dn5 = ((((var_k1_2 * var_q1_dn5) * var_q1) + (assign7160_e7314 * var_q1_dn5)) - var_t0_dn5);
        var_qsqrt_dn6 = ((((var_k1_2 * var_q1_dn6) * var_q1) + (assign7160_e7314 * var_q1_dn6)) - var_t0_dn6);
        var_qsqrt_dn7 = ((((var_k1_2 * var_q1_dn7) * var_q1) + (assign7160_e7314 * var_q1_dn7)) - var_t0_dn7);
        var_qsqrt_dn8 = ((((var_k1_2 * var_q1_dn8) * var_q1) + (assign7160_e7314 * var_q1_dn8)) - var_t0_dn8);

        let assign7170_e7321: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard82 = assign7170_e7321;

        let (assign7180_e7327, assign7180_e7327_d_n3, assign7180_e7327_d_n4, assign7180_e7327_d_n5, assign7180_e7327_d_n6, assign7180_e7327_d_n7, assign7180_e7327_d_n8,) = {
    if (var_guard82 != 0.0) {
        let assign7180_e7324: f64 = (-var_qsqrt);
        let assign7180_e7325: f64 = (assign7180_e7324).sqrt();
        (assign7180_e7325, ((-var_qsqrt_dn3) / (2.0 * assign7180_e7325)), ((-var_qsqrt_dn4) / (2.0 * assign7180_e7325)), ((-var_qsqrt_dn5) / (2.0 * assign7180_e7325)), ((-var_qsqrt_dn6) / (2.0 * assign7180_e7325)), ((-var_qsqrt_dn7) / (2.0 * assign7180_e7325)), ((-var_qsqrt_dn8) / (2.0 * assign7180_e7325)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign7180_e7327;
        var_q_dn3 = assign7180_e7327_d_n3;
        var_q_dn4 = assign7180_e7327_d_n4;
        var_q_dn5 = assign7180_e7327_d_n5;
        var_q_dn6 = assign7180_e7327_d_n6;
        var_q_dn7 = assign7180_e7327_d_n7;
        var_q_dn8 = assign7180_e7327_d_n8;

        let (assign7190_e7333, assign7190_e7333_d_n3, assign7190_e7333_d_n4, assign7190_e7333_d_n5, assign7190_e7333_d_n6, assign7190_e7333_d_n7, assign7190_e7333_d_n8,) = {
    if (var_guard82 != 0.0) {
        let assign7190_e7331: f64 = (0.5 * var_q);
        (assign7190_e7331, (0.5 * var_q_dn3), (0.5 * var_q_dn4), (0.5 * var_q_dn5), (0.5 * var_q_dn6), (0.5 * var_q_dn7), (0.5 * var_q_dn8),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign7190_e7333;
        var_t2_dn3 = assign7190_e7333_d_n3;
        var_t2_dn4 = assign7190_e7333_d_n4;
        var_t2_dn5 = assign7190_e7333_d_n5;
        var_t2_dn6 = assign7190_e7333_d_n6;
        var_t2_dn7 = assign7190_e7333_d_n7;
        var_t2_dn8 = assign7190_e7333_d_n8;

        let (assign7200_e7340, assign7200_e7340_d_n3, assign7200_e7340_d_n4, assign7200_e7340_d_n5, assign7200_e7340_d_n6, assign7200_e7340_d_n7, assign7200_e7340_d_n8,) = {
    if (var_guard82 != 0.0) {
        let assign7200_e7337: f64 = (var_t2).tan();
        let assign7200_e7338: f64 = (var_q / assign7200_e7337);
        (assign7200_e7338, (((var_q_dn3 * assign7200_e7337) - (var_q * (var_t2_dn3 / ((var_t2).cos() * (var_t2).cos())))) / (assign7200_e7337 * assign7200_e7337)), (((var_q_dn4 * assign7200_e7337) - (var_q * (var_t2_dn4 / ((var_t2).cos() * (var_t2).cos())))) / (assign7200_e7337 * assign7200_e7337)), (((var_q_dn5 * assign7200_e7337) - (var_q * (var_t2_dn5 / ((var_t2).cos() * (var_t2).cos())))) / (assign7200_e7337 * assign7200_e7337)), (((var_q_dn6 * assign7200_e7337) - (var_q * (var_t2_dn6 / ((var_t2).cos() * (var_t2).cos())))) / (assign7200_e7337 * assign7200_e7337)), (((var_q_dn7 * assign7200_e7337) - (var_q * (var_t2_dn7 / ((var_t2).cos() * (var_t2).cos())))) / (assign7200_e7337 * assign7200_e7337)), (((var_q_dn8 * assign7200_e7337) - (var_q * (var_t2_dn8 / ((var_t2).cos() * (var_t2).cos())))) / (assign7200_e7337 * assign7200_e7337)),)
    } else {
        (var_qcoth, var_qcoth_dn3, var_qcoth_dn4, var_qcoth_dn5, var_qcoth_dn6, var_qcoth_dn7, var_qcoth_dn8,)
    }
};
        var_qcoth = assign7200_e7340;
        var_qcoth_dn3 = assign7200_e7340_d_n3;
        var_qcoth_dn4 = assign7200_e7340_d_n4;
        var_qcoth_dn5 = assign7200_e7340_d_n5;
        var_qcoth_dn6 = assign7200_e7340_d_n6;
        var_qcoth_dn7 = assign7200_e7340_d_n7;
        var_qcoth_dn8 = assign7200_e7340_d_n8;

        let (assign7210_e7345, assign7210_e7345_d_n3, assign7210_e7345_d_n4, assign7210_e7345_d_n5, assign7210_e7345_d_n6, assign7210_e7345_d_n7, assign7210_e7345_d_n8,) = {
    if (var_guard82 != 0.0) {
        let assign7210_e7343: f64 = (var_t2).sin();
        (assign7210_e7343, ((var_t2).cos() * var_t2_dn3), ((var_t2).cos() * var_t2_dn4), ((var_t2).cos() * var_t2_dn5), ((var_t2).cos() * var_t2_dn6), ((var_t2).cos() * var_t2_dn7), ((var_t2).cos() * var_t2_dn8),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign7210_e7345;
        var_t6_dn3 = assign7210_e7345_d_n3;
        var_t6_dn4 = assign7210_e7345_d_n4;
        var_t6_dn5 = assign7210_e7345_d_n5;
        var_t6_dn6 = assign7210_e7345_d_n6;
        var_t6_dn7 = assign7210_e7345_d_n7;
        var_t6_dn8 = assign7210_e7345_d_n8;

        let (assign7220_e7352, assign7220_e7352_d_n3, assign7220_e7352_d_n4, assign7220_e7352_d_n5, assign7220_e7352_d_n6, assign7220_e7352_d_n7, assign7220_e7352_d_n8,) = {
    if (var_guard82 != 0.0) {
        let assign7220_e7348: f64 = (-var_t6);
        let assign7220_e7350: f64 = (assign7220_e7348 * var_t6);
        (assign7220_e7350, (((-var_t6_dn3) * var_t6) + (assign7220_e7348 * var_t6_dn3)), (((-var_t6_dn4) * var_t6) + (assign7220_e7348 * var_t6_dn4)), (((-var_t6_dn5) * var_t6) + (assign7220_e7348 * var_t6_dn5)), (((-var_t6_dn6) * var_t6) + (assign7220_e7348 * var_t6_dn6)), (((-var_t6_dn7) * var_t6) + (assign7220_e7348 * var_t6_dn7)), (((-var_t6_dn8) * var_t6) + (assign7220_e7348 * var_t6_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign7220_e7352;
        var_t1_dn3 = assign7220_e7352_d_n3;
        var_t1_dn4 = assign7220_e7352_d_n4;
        var_t1_dn5 = assign7220_e7352_d_n5;
        var_t1_dn6 = assign7220_e7352_d_n6;
        var_t1_dn7 = assign7220_e7352_d_n7;
        var_t1_dn8 = assign7220_e7352_d_n8;

        let (assign7230_e7358, assign7230_e7358_d_n3, assign7230_e7358_d_n4, assign7230_e7358_d_n5, assign7230_e7358_d_n6, assign7230_e7358_d_n7, assign7230_e7358_d_n8,) = {
    if (var_guard82 == 0.0) {
        let assign7230_e7356: f64 = (var_qsqrt).sqrt();
        (assign7230_e7356, (var_qsqrt_dn3 / (2.0 * assign7230_e7356)), (var_qsqrt_dn4 / (2.0 * assign7230_e7356)), (var_qsqrt_dn5 / (2.0 * assign7230_e7356)), (var_qsqrt_dn6 / (2.0 * assign7230_e7356)), (var_qsqrt_dn7 / (2.0 * assign7230_e7356)), (var_qsqrt_dn8 / (2.0 * assign7230_e7356)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign7230_e7358;
        var_q_dn3 = assign7230_e7358_d_n3;
        var_q_dn4 = assign7230_e7358_d_n4;
        var_q_dn5 = assign7230_e7358_d_n5;
        var_q_dn6 = assign7230_e7358_d_n6;
        var_q_dn7 = assign7230_e7358_d_n7;
        var_q_dn8 = assign7230_e7358_d_n8;

        let (assign7240_e7365, assign7240_e7365_d_n3, assign7240_e7365_d_n4, assign7240_e7365_d_n5, assign7240_e7365_d_n6, assign7240_e7365_d_n7, assign7240_e7365_d_n8,) = {
    if (var_guard82 == 0.0) {
        let assign7240_e7363: f64 = (0.5 * var_q);
        (assign7240_e7363, (0.5 * var_q_dn3), (0.5 * var_q_dn4), (0.5 * var_q_dn5), (0.5 * var_q_dn6), (0.5 * var_q_dn7), (0.5 * var_q_dn8),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign7240_e7365;
        var_t2_dn3 = assign7240_e7365_d_n3;
        var_t2_dn4 = assign7240_e7365_d_n4;
        var_t2_dn5 = assign7240_e7365_d_n5;
        var_t2_dn6 = assign7240_e7365_d_n6;
        var_t2_dn7 = assign7240_e7365_d_n7;
        var_t2_dn8 = assign7240_e7365_d_n8;

        let (assign7250_e7371, assign7250_e7371_d_n3, assign7250_e7371_d_n4, assign7250_e7371_d_n5, assign7250_e7371_d_n6, assign7250_e7371_d_n7, assign7250_e7371_d_n8,) = {
    if (var_guard82 == 0.0) {
        let assign7250_e7369: f64 = (var_t2).sinh();
        (assign7250_e7369, ((var_t2).cosh() * var_t2_dn3), ((var_t2).cosh() * var_t2_dn4), ((var_t2).cosh() * var_t2_dn5), ((var_t2).cosh() * var_t2_dn6), ((var_t2).cosh() * var_t2_dn7), ((var_t2).cosh() * var_t2_dn8),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign7250_e7371;
        var_t6_dn3 = assign7250_e7371_d_n3;
        var_t6_dn4 = assign7250_e7371_d_n4;
        var_t6_dn5 = assign7250_e7371_d_n5;
        var_t6_dn6 = assign7250_e7371_d_n6;
        var_t6_dn7 = assign7250_e7371_d_n7;
        var_t6_dn8 = assign7250_e7371_d_n8;

        *var_coth1_slot = var_coth1;
        *var_coth1_dn3_slot = var_coth1_dn3;
        *var_coth1_dn4_slot = var_coth1_dn4;
        *var_coth1_dn5_slot = var_coth1_dn5;
        *var_coth1_dn6_slot = var_coth1_dn6;
        *var_coth1_dn7_slot = var_coth1_dn7;
        *var_coth1_dn8_slot = var_coth1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_df_slot = var_df;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_dn6_slot = var_df_dn6;
        *var_df_dn7_slot = var_df_dn7;
        *var_df_dn8_slot = var_df_dn8;
        *var_dlogsinhqsqdqsqrt_slot = var_dlogsinhqsqdqsqrt;
        *var_dlogsinhqsqdqsqrt_dn3_slot = var_dlogsinhqsqdqsqrt_dn3;
        *var_dlogsinhqsqdqsqrt_dn4_slot = var_dlogsinhqsqdqsqrt_dn4;
        *var_dlogsinhqsqdqsqrt_dn5_slot = var_dlogsinhqsqdqsqrt_dn5;
        *var_dlogsinhqsqdqsqrt_dn6_slot = var_dlogsinhqsqdqsqrt_dn6;
        *var_dlogsinhqsqdqsqrt_dn7_slot = var_dlogsinhqsqdqsqrt_dn7;
        *var_dlogsinhqsqdqsqrt_dn8_slot = var_dlogsinhqsqdqsqrt_dn8;
        *var_dq2_slot = var_dq2;
        *var_dq2_dn3_slot = var_dq2_dn3;
        *var_dq2_dn4_slot = var_dq2_dn4;
        *var_dq2_dn5_slot = var_dq2_dn5;
        *var_dq2_dn6_slot = var_dq2_dn6;
        *var_dq2_dn7_slot = var_dq2_dn7;
        *var_dq2_dn8_slot = var_dq2_dn8;
        *var_dqcoth_slot = var_dqcoth;
        *var_dqcoth_dn3_slot = var_dqcoth_dn3;
        *var_dqcoth_dn4_slot = var_dqcoth_dn4;
        *var_dqcoth_dn5_slot = var_dqcoth_dn5;
        *var_dqcoth_dn6_slot = var_dqcoth_dn6;
        *var_dqcoth_dn7_slot = var_dqcoth_dn7;
        *var_dqcoth_dn8_slot = var_dqcoth_dn8;
        *var_dqcothqdqsqrt_slot = var_dqcothqdqsqrt;
        *var_dqcothqdqsqrt_dn3_slot = var_dqcothqdqsqrt_dn3;
        *var_dqcothqdqsqrt_dn4_slot = var_dqcothqdqsqrt_dn4;
        *var_dqcothqdqsqrt_dn5_slot = var_dqcothqdqsqrt_dn5;
        *var_dqcothqdqsqrt_dn6_slot = var_dqcothqdqsqrt_dn6;
        *var_dqcothqdqsqrt_dn7_slot = var_dqcothqdqsqrt_dn7;
        *var_dqcothqdqsqrt_dn8_slot = var_dqcothqdqsqrt_dn8;
        *var_dqsqrt_slot = var_dqsqrt;
        *var_dqsqrt_dn3_slot = var_dqsqrt_dn3;
        *var_dqsqrt_dn4_slot = var_dqsqrt_dn4;
        *var_dqsqrt_dn5_slot = var_dqsqrt_dn5;
        *var_dqsqrt_dn6_slot = var_dqsqrt_dn6;
        *var_dqsqrt_dn7_slot = var_dqsqrt_dn7;
        *var_dqsqrt_dn8_slot = var_dqsqrt_dn8;
        *var_f_slot = var_f;
        *var_f_dn3_slot = var_f_dn3;
        *var_f_dn4_slot = var_f_dn4;
        *var_f_dn5_slot = var_f_dn5;
        *var_f_dn6_slot = var_f_dn6;
        *var_f_dn7_slot = var_f_dn7;
        *var_f_dn8_slot = var_f_dn8;
        *var_guard82_slot = var_guard82;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        var_cox1: f64,
        var_cox2: f64,
        var_csi: f64,
        var_dvth_all: f64,
        var_dvth_all_dn3: f64,
        var_dvth_all_dn4: f64,
        var_dvth_all_dn5: f64,
        var_dvth_all_dn6: f64,
        var_dvth_all_dn7: f64,
        var_dvth_all_dn8: f64,
        var_eefffactor: f64,
        var_eefffactor2: f64,
        var_eta_mu: f64,
        var_eta_mu2: f64,
        var_eu2_i: f64,
        var_eu_i: f64,
        var_eub2_i: f64,
        var_eub_i: f64,
        var_guard82: f64,
        var_k1: f64,
        var_nbody_i: f64,
        var_nvtm: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_phi1: f64,
        var_phi1_dn3: f64,
        var_phi1_dn4: f64,
        var_phi1_dn5: f64,
        var_phi1_dn6: f64,
        var_phi1_dn7: f64,
        var_phi1_dn8: f64,
        var_prwg_i: f64,
        var_q: f64,
        var_q1: f64,
        var_q1_dn3: f64,
        var_q1_dn4: f64,
        var_q1_dn5: f64,
        var_q1_dn6: f64,
        var_q1_dn7: f64,
        var_q1_dn8: f64,
        var_q_dn3: f64,
        var_q_dn4: f64,
        var_q_dn5: f64,
        var_q_dn6: f64,
        var_q_dn7: f64,
        var_q_dn8: f64,
        var_qsqrt: f64,
        var_qsqrt_dn3: f64,
        var_qsqrt_dn4: f64,
        var_qsqrt_dn5: f64,
        var_qsqrt_dn6: f64,
        var_qsqrt_dn7: f64,
        var_qsqrt_dn8: f64,
        var_rdstemp: f64,
        var_rdstemp_dn4: f64,
        var_rdsw_i: f64,
        var_rdswmin_i: f64,
        var_t6: f64,
        var_t6_dn3: f64,
        var_t6_dn4: f64,
        var_t6_dn5: f64,
        var_t6_dn6: f64,
        var_t6_dn7: f64,
        var_t6_dn8: f64,
        var_u02_i: f64,
        var_u0_t: f64,
        var_u0_t_dn4: f64,
        var_ua2_i: f64,
        var_ua_t: f64,
        var_ua_t_dn4: f64,
        var_uc2_i: f64,
        var_uc_t: f64,
        var_uc_t_dn4: f64,
        var_ucs2_i: f64,
        var_ucs_t: f64,
        var_ucs_t_dn4: f64,
        var_ud2_i: f64,
        var_ud_t: f64,
        var_ud_t_dn4: f64,
        var_vbgs: f64,
        var_vbgs_dn3: f64,
        var_vbgs_dn5: f64,
        var_vbgs_dn6: f64,
        var_vgfb1eff: f64,
        var_vgfb1eff_dn3: f64,
        var_vgfb1eff_dn4: f64,
        var_vgfb1eff_dn5: f64,
        var_vgfb1eff_dn6: f64,
        var_vgfb1eff_dn7: f64,
        var_vgfb1eff_dn8: f64,
        var_vgfb2: f64,
        var_vgfb2_dn3: f64,
        var_vgfb2_dn4: f64,
        var_vgfb2_dn5: f64,
        var_vgfb2_dn6: f64,
        var_vgfb2_dn7: f64,
        var_vgfb2_dn8: f64,
        var_weffwrfactor: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_dmobs_slot: &mut f64,
        var_dmobs_dn3_slot: &mut f64,
        var_dmobs_dn4_slot: &mut f64,
        var_dmobs_dn5_slot: &mut f64,
        var_dmobs_dn6_slot: &mut f64,
        var_dmobs_dn7_slot: &mut f64,
        var_dmobs_dn8_slot: &mut f64,
        var_eeffs_slot: &mut f64,
        var_eeffs2_slot: &mut f64,
        var_eeffs2_dn3_slot: &mut f64,
        var_eeffs2_dn4_slot: &mut f64,
        var_eeffs2_dn5_slot: &mut f64,
        var_eeffs2_dn6_slot: &mut f64,
        var_eeffs2_dn7_slot: &mut f64,
        var_eeffs2_dn8_slot: &mut f64,
        var_eeffs_dn3_slot: &mut f64,
        var_eeffs_dn4_slot: &mut f64,
        var_eeffs_dn5_slot: &mut f64,
        var_eeffs_dn6_slot: &mut f64,
        var_eeffs_dn7_slot: &mut f64,
        var_eeffs_dn8_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_phi2_slot: &mut f64,
        var_phi2_dn3_slot: &mut f64,
        var_phi2_dn4_slot: &mut f64,
        var_phi2_dn5_slot: &mut f64,
        var_phi2_dn6_slot: &mut f64,
        var_phi2_dn7_slot: &mut f64,
        var_phi2_dn8_slot: &mut f64,
        var_phifs_slot: &mut f64,
        var_phifs_dn3_slot: &mut f64,
        var_phifs_dn4_slot: &mut f64,
        var_phifs_dn5_slot: &mut f64,
        var_phifs_dn6_slot: &mut f64,
        var_phifs_dn7_slot: &mut f64,
        var_phifs_dn8_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qbacks_slot: &mut f64,
        var_qbacks_dn3_slot: &mut f64,
        var_qbacks_dn4_slot: &mut f64,
        var_qbacks_dn5_slot: &mut f64,
        var_qbacks_dn6_slot: &mut f64,
        var_qbacks_dn7_slot: &mut f64,
        var_qbacks_dn8_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qfronts_slot: &mut f64,
        var_qfronts_dn3_slot: &mut f64,
        var_qfronts_dn4_slot: &mut f64,
        var_qfronts_dn5_slot: &mut f64,
        var_qfronts_dn6_slot: &mut f64,
        var_qfronts_dn7_slot: &mut f64,
        var_qfronts_dn8_slot: &mut f64,
        var_qicores_slot: &mut f64,
        var_qicores_dn3_slot: &mut f64,
        var_qicores_dn4_slot: &mut f64,
        var_qicores_dn5_slot: &mut f64,
        var_qicores_dn6_slot: &mut f64,
        var_qicores_dn7_slot: &mut f64,
        var_qicores_dn8_slot: &mut f64,
        var_qis_slot: &mut f64,
        var_qis_dn3_slot: &mut f64,
        var_qis_dn4_slot: &mut f64,
        var_qis_dn5_slot: &mut f64,
        var_qis_dn6_slot: &mut f64,
        var_qis_dn7_slot: &mut f64,
        var_qis_dn8_slot: &mut f64,
        var_qtots_slot: &mut f64,
        var_qtots_dn3_slot: &mut f64,
        var_qtots_dn4_slot: &mut f64,
        var_qtots_dn5_slot: &mut f64,
        var_qtots_dn6_slot: &mut f64,
        var_qtots_dn7_slot: &mut f64,
        var_qtots_dn8_slot: &mut f64,
        var_rdss_slot: &mut f64,
        var_rdss_dn3_slot: &mut f64,
        var_rdss_dn4_slot: &mut f64,
        var_rdss_dn5_slot: &mut f64,
        var_rdss_dn6_slot: &mut f64,
        var_rdss_dn7_slot: &mut f64,
        var_rdss_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2__blk83_slot: &mut f64,
        var_t2__blk83_dn3_slot: &mut f64,
        var_t2__blk83_dn4_slot: &mut f64,
        var_t2__blk83_dn5_slot: &mut f64,
        var_t2__blk83_dn6_slot: &mut f64,
        var_t2__blk83_dn7_slot: &mut f64,
        var_t2__blk83_dn8_slot: &mut f64,
        var_t2__blk85_slot: &mut f64,
        var_t2__blk85_dn3_slot: &mut f64,
        var_t2__blk85_dn4_slot: &mut f64,
        var_t2__blk85_dn5_slot: &mut f64,
        var_t2__blk85_dn6_slot: &mut f64,
        var_t2__blk85_dn7_slot: &mut f64,
        var_t2__blk85_dn8_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3__blk84_slot: &mut f64,
        var_t3__blk84_dn3_slot: &mut f64,
        var_t3__blk84_dn4_slot: &mut f64,
        var_t3__blk84_dn5_slot: &mut f64,
        var_t3__blk84_dn6_slot: &mut f64,
        var_t3__blk84_dn7_slot: &mut f64,
        var_t3__blk84_dn8_slot: &mut f64,
        var_t3__blk86_slot: &mut f64,
        var_t3__blk86_dn3_slot: &mut f64,
        var_t3__blk86_dn4_slot: &mut f64,
        var_t3__blk86_dn5_slot: &mut f64,
        var_t3__blk86_dn6_slot: &mut f64,
        var_t3__blk86_dn7_slot: &mut f64,
        var_t3__blk86_dn8_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_ueff1_slot: &mut f64,
        var_ueff1_dn3_slot: &mut f64,
        var_ueff1_dn4_slot: &mut f64,
        var_ueff1_dn5_slot: &mut f64,
        var_ueff1_dn6_slot: &mut f64,
        var_ueff1_dn7_slot: &mut f64,
        var_ueff1_dn8_slot: &mut f64,
        var_ueff2_slot: &mut f64,
        var_ueff2_dn3_slot: &mut f64,
        var_ueff2_dn4_slot: &mut f64,
        var_ueff2_dn5_slot: &mut f64,
        var_ueff2_dn6_slot: &mut f64,
        var_ueff2_dn7_slot: &mut f64,
        var_ueff2_dn8_slot: &mut f64,
        var_utotal_slot: &mut f64,
        var_utotal_dn3_slot: &mut f64,
        var_utotal_dn4_slot: &mut f64,
        var_utotal_dn5_slot: &mut f64,
        var_utotal_dn6_slot: &mut f64,
        var_utotal_dn7_slot: &mut f64,
        var_utotal_dn8_slot: &mut f64,
        var_w1_slot: &mut f64,
        var_w1_dn3_slot: &mut f64,
        var_w1_dn4_slot: &mut f64,
        var_w1_dn5_slot: &mut f64,
        var_w1_dn6_slot: &mut f64,
        var_w1_dn7_slot: &mut f64,
        var_w1_dn8_slot: &mut f64,
        var_w2_slot: &mut f64,
        var_w2_dn3_slot: &mut f64,
        var_w2_dn4_slot: &mut f64,
        var_w2_dn5_slot: &mut f64,
        var_w2_dn6_slot: &mut f64,
        var_w2_dn7_slot: &mut f64,
        var_w2_dn8_slot: &mut f64,
    ) {
        let mut var_dmobs: f64 = *var_dmobs_slot;
        let mut var_dmobs_dn3: f64 = *var_dmobs_dn3_slot;
        let mut var_dmobs_dn4: f64 = *var_dmobs_dn4_slot;
        let mut var_dmobs_dn5: f64 = *var_dmobs_dn5_slot;
        let mut var_dmobs_dn6: f64 = *var_dmobs_dn6_slot;
        let mut var_dmobs_dn7: f64 = *var_dmobs_dn7_slot;
        let mut var_dmobs_dn8: f64 = *var_dmobs_dn8_slot;
        let mut var_eeffs: f64 = *var_eeffs_slot;
        let mut var_eeffs2: f64 = *var_eeffs2_slot;
        let mut var_eeffs2_dn3: f64 = *var_eeffs2_dn3_slot;
        let mut var_eeffs2_dn4: f64 = *var_eeffs2_dn4_slot;
        let mut var_eeffs2_dn5: f64 = *var_eeffs2_dn5_slot;
        let mut var_eeffs2_dn6: f64 = *var_eeffs2_dn6_slot;
        let mut var_eeffs2_dn7: f64 = *var_eeffs2_dn7_slot;
        let mut var_eeffs2_dn8: f64 = *var_eeffs2_dn8_slot;
        let mut var_eeffs_dn3: f64 = *var_eeffs_dn3_slot;
        let mut var_eeffs_dn4: f64 = *var_eeffs_dn4_slot;
        let mut var_eeffs_dn5: f64 = *var_eeffs_dn5_slot;
        let mut var_eeffs_dn6: f64 = *var_eeffs_dn6_slot;
        let mut var_eeffs_dn7: f64 = *var_eeffs_dn7_slot;
        let mut var_eeffs_dn8: f64 = *var_eeffs_dn8_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_phi2: f64 = *var_phi2_slot;
        let mut var_phi2_dn3: f64 = *var_phi2_dn3_slot;
        let mut var_phi2_dn4: f64 = *var_phi2_dn4_slot;
        let mut var_phi2_dn5: f64 = *var_phi2_dn5_slot;
        let mut var_phi2_dn6: f64 = *var_phi2_dn6_slot;
        let mut var_phi2_dn7: f64 = *var_phi2_dn7_slot;
        let mut var_phi2_dn8: f64 = *var_phi2_dn8_slot;
        let mut var_phifs: f64 = *var_phifs_slot;
        let mut var_phifs_dn3: f64 = *var_phifs_dn3_slot;
        let mut var_phifs_dn4: f64 = *var_phifs_dn4_slot;
        let mut var_phifs_dn5: f64 = *var_phifs_dn5_slot;
        let mut var_phifs_dn6: f64 = *var_phifs_dn6_slot;
        let mut var_phifs_dn7: f64 = *var_phifs_dn7_slot;
        let mut var_phifs_dn8: f64 = *var_phifs_dn8_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qbacks: f64 = *var_qbacks_slot;
        let mut var_qbacks_dn3: f64 = *var_qbacks_dn3_slot;
        let mut var_qbacks_dn4: f64 = *var_qbacks_dn4_slot;
        let mut var_qbacks_dn5: f64 = *var_qbacks_dn5_slot;
        let mut var_qbacks_dn6: f64 = *var_qbacks_dn6_slot;
        let mut var_qbacks_dn7: f64 = *var_qbacks_dn7_slot;
        let mut var_qbacks_dn8: f64 = *var_qbacks_dn8_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qfronts: f64 = *var_qfronts_slot;
        let mut var_qfronts_dn3: f64 = *var_qfronts_dn3_slot;
        let mut var_qfronts_dn4: f64 = *var_qfronts_dn4_slot;
        let mut var_qfronts_dn5: f64 = *var_qfronts_dn5_slot;
        let mut var_qfronts_dn6: f64 = *var_qfronts_dn6_slot;
        let mut var_qfronts_dn7: f64 = *var_qfronts_dn7_slot;
        let mut var_qfronts_dn8: f64 = *var_qfronts_dn8_slot;
        let mut var_qicores: f64 = *var_qicores_slot;
        let mut var_qicores_dn3: f64 = *var_qicores_dn3_slot;
        let mut var_qicores_dn4: f64 = *var_qicores_dn4_slot;
        let mut var_qicores_dn5: f64 = *var_qicores_dn5_slot;
        let mut var_qicores_dn6: f64 = *var_qicores_dn6_slot;
        let mut var_qicores_dn7: f64 = *var_qicores_dn7_slot;
        let mut var_qicores_dn8: f64 = *var_qicores_dn8_slot;
        let mut var_qis: f64 = *var_qis_slot;
        let mut var_qis_dn3: f64 = *var_qis_dn3_slot;
        let mut var_qis_dn4: f64 = *var_qis_dn4_slot;
        let mut var_qis_dn5: f64 = *var_qis_dn5_slot;
        let mut var_qis_dn6: f64 = *var_qis_dn6_slot;
        let mut var_qis_dn7: f64 = *var_qis_dn7_slot;
        let mut var_qis_dn8: f64 = *var_qis_dn8_slot;
        let mut var_qtots: f64 = *var_qtots_slot;
        let mut var_qtots_dn3: f64 = *var_qtots_dn3_slot;
        let mut var_qtots_dn4: f64 = *var_qtots_dn4_slot;
        let mut var_qtots_dn5: f64 = *var_qtots_dn5_slot;
        let mut var_qtots_dn6: f64 = *var_qtots_dn6_slot;
        let mut var_qtots_dn7: f64 = *var_qtots_dn7_slot;
        let mut var_qtots_dn8: f64 = *var_qtots_dn8_slot;
        let mut var_rdss: f64 = *var_rdss_slot;
        let mut var_rdss_dn3: f64 = *var_rdss_dn3_slot;
        let mut var_rdss_dn4: f64 = *var_rdss_dn4_slot;
        let mut var_rdss_dn5: f64 = *var_rdss_dn5_slot;
        let mut var_rdss_dn6: f64 = *var_rdss_dn6_slot;
        let mut var_rdss_dn7: f64 = *var_rdss_dn7_slot;
        let mut var_rdss_dn8: f64 = *var_rdss_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2__blk83: f64 = *var_t2__blk83_slot;
        let mut var_t2__blk83_dn3: f64 = *var_t2__blk83_dn3_slot;
        let mut var_t2__blk83_dn4: f64 = *var_t2__blk83_dn4_slot;
        let mut var_t2__blk83_dn5: f64 = *var_t2__blk83_dn5_slot;
        let mut var_t2__blk83_dn6: f64 = *var_t2__blk83_dn6_slot;
        let mut var_t2__blk83_dn7: f64 = *var_t2__blk83_dn7_slot;
        let mut var_t2__blk83_dn8: f64 = *var_t2__blk83_dn8_slot;
        let mut var_t2__blk85: f64 = *var_t2__blk85_slot;
        let mut var_t2__blk85_dn3: f64 = *var_t2__blk85_dn3_slot;
        let mut var_t2__blk85_dn4: f64 = *var_t2__blk85_dn4_slot;
        let mut var_t2__blk85_dn5: f64 = *var_t2__blk85_dn5_slot;
        let mut var_t2__blk85_dn6: f64 = *var_t2__blk85_dn6_slot;
        let mut var_t2__blk85_dn7: f64 = *var_t2__blk85_dn7_slot;
        let mut var_t2__blk85_dn8: f64 = *var_t2__blk85_dn8_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3__blk84: f64 = *var_t3__blk84_slot;
        let mut var_t3__blk84_dn3: f64 = *var_t3__blk84_dn3_slot;
        let mut var_t3__blk84_dn4: f64 = *var_t3__blk84_dn4_slot;
        let mut var_t3__blk84_dn5: f64 = *var_t3__blk84_dn5_slot;
        let mut var_t3__blk84_dn6: f64 = *var_t3__blk84_dn6_slot;
        let mut var_t3__blk84_dn7: f64 = *var_t3__blk84_dn7_slot;
        let mut var_t3__blk84_dn8: f64 = *var_t3__blk84_dn8_slot;
        let mut var_t3__blk86: f64 = *var_t3__blk86_slot;
        let mut var_t3__blk86_dn3: f64 = *var_t3__blk86_dn3_slot;
        let mut var_t3__blk86_dn4: f64 = *var_t3__blk86_dn4_slot;
        let mut var_t3__blk86_dn5: f64 = *var_t3__blk86_dn5_slot;
        let mut var_t3__blk86_dn6: f64 = *var_t3__blk86_dn6_slot;
        let mut var_t3__blk86_dn7: f64 = *var_t3__blk86_dn7_slot;
        let mut var_t3__blk86_dn8: f64 = *var_t3__blk86_dn8_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_ueff1: f64 = *var_ueff1_slot;
        let mut var_ueff1_dn3: f64 = *var_ueff1_dn3_slot;
        let mut var_ueff1_dn4: f64 = *var_ueff1_dn4_slot;
        let mut var_ueff1_dn5: f64 = *var_ueff1_dn5_slot;
        let mut var_ueff1_dn6: f64 = *var_ueff1_dn6_slot;
        let mut var_ueff1_dn7: f64 = *var_ueff1_dn7_slot;
        let mut var_ueff1_dn8: f64 = *var_ueff1_dn8_slot;
        let mut var_ueff2: f64 = *var_ueff2_slot;
        let mut var_ueff2_dn3: f64 = *var_ueff2_dn3_slot;
        let mut var_ueff2_dn4: f64 = *var_ueff2_dn4_slot;
        let mut var_ueff2_dn5: f64 = *var_ueff2_dn5_slot;
        let mut var_ueff2_dn6: f64 = *var_ueff2_dn6_slot;
        let mut var_ueff2_dn7: f64 = *var_ueff2_dn7_slot;
        let mut var_ueff2_dn8: f64 = *var_ueff2_dn8_slot;
        let mut var_utotal: f64 = *var_utotal_slot;
        let mut var_utotal_dn3: f64 = *var_utotal_dn3_slot;
        let mut var_utotal_dn4: f64 = *var_utotal_dn4_slot;
        let mut var_utotal_dn5: f64 = *var_utotal_dn5_slot;
        let mut var_utotal_dn6: f64 = *var_utotal_dn6_slot;
        let mut var_utotal_dn7: f64 = *var_utotal_dn7_slot;
        let mut var_utotal_dn8: f64 = *var_utotal_dn8_slot;
        let mut var_w1: f64 = *var_w1_slot;
        let mut var_w1_dn3: f64 = *var_w1_dn3_slot;
        let mut var_w1_dn4: f64 = *var_w1_dn4_slot;
        let mut var_w1_dn5: f64 = *var_w1_dn5_slot;
        let mut var_w1_dn6: f64 = *var_w1_dn6_slot;
        let mut var_w1_dn7: f64 = *var_w1_dn7_slot;
        let mut var_w1_dn8: f64 = *var_w1_dn8_slot;
        let mut var_w2: f64 = *var_w2_slot;
        let mut var_w2_dn3: f64 = *var_w2_dn3_slot;
        let mut var_w2_dn4: f64 = *var_w2_dn4_slot;
        let mut var_w2_dn5: f64 = *var_w2_dn5_slot;
        let mut var_w2_dn6: f64 = *var_w2_dn6_slot;
        let mut var_w2_dn7: f64 = *var_w2_dn7_slot;
        let mut var_w2_dn8: f64 = *var_w2_dn8_slot;

        let (assign7260_e7378, assign7260_e7378_d_n3, assign7260_e7378_d_n4, assign7260_e7378_d_n5, assign7260_e7378_d_n6, assign7260_e7378_d_n7, assign7260_e7378_d_n8,) = {
    if (var_guard82 == 0.0) {
        let assign7260_e7376: f64 = (var_t6 * var_t6);
        (assign7260_e7376, ((var_t6_dn3 * var_t6) + (var_t6 * var_t6_dn3)), ((var_t6_dn4 * var_t6) + (var_t6 * var_t6_dn4)), ((var_t6_dn5 * var_t6) + (var_t6 * var_t6_dn5)), ((var_t6_dn6 * var_t6) + (var_t6 * var_t6_dn6)), ((var_t6_dn7 * var_t6) + (var_t6 * var_t6_dn7)), ((var_t6_dn8 * var_t6) + (var_t6 * var_t6_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign7260_e7378;
        var_t1_dn3 = assign7260_e7378_d_n3;
        var_t1_dn4 = assign7260_e7378_d_n4;
        var_t1_dn5 = assign7260_e7378_d_n5;
        var_t1_dn6 = assign7260_e7378_d_n6;
        var_t1_dn7 = assign7260_e7378_d_n7;
        var_t1_dn8 = assign7260_e7378_d_n8;

        let (assign7270_e7386, assign7270_e7386_d_n3, assign7270_e7386_d_n4, assign7270_e7386_d_n5, assign7270_e7386_d_n6, assign7270_e7386_d_n7, assign7270_e7386_d_n8,) = {
    if (var_guard82 == 0.0) {
        let assign7270_e7383: f64 = (var_t2).tanh();
        let assign7270_e7384: f64 = (var_q / assign7270_e7383);
        (assign7270_e7384, (((var_q_dn3 * assign7270_e7383) - (var_q * (var_t2_dn3 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign7270_e7383 * assign7270_e7383)), (((var_q_dn4 * assign7270_e7383) - (var_q * (var_t2_dn4 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign7270_e7383 * assign7270_e7383)), (((var_q_dn5 * assign7270_e7383) - (var_q * (var_t2_dn5 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign7270_e7383 * assign7270_e7383)), (((var_q_dn6 * assign7270_e7383) - (var_q * (var_t2_dn6 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign7270_e7383 * assign7270_e7383)), (((var_q_dn7 * assign7270_e7383) - (var_q * (var_t2_dn7 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign7270_e7383 * assign7270_e7383)), (((var_q_dn8 * assign7270_e7383) - (var_q * (var_t2_dn8 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign7270_e7383 * assign7270_e7383)),)
    } else {
        (var_qcoth, var_qcoth_dn3, var_qcoth_dn4, var_qcoth_dn5, var_qcoth_dn6, var_qcoth_dn7, var_qcoth_dn8,)
    }
};
        var_qcoth = assign7270_e7386;
        var_qcoth_dn3 = assign7270_e7386_d_n3;
        var_qcoth_dn4 = assign7270_e7386_d_n4;
        var_qcoth_dn5 = assign7270_e7386_d_n5;
        var_qcoth_dn6 = assign7270_e7386_d_n6;
        var_qcoth_dn7 = assign7270_e7386_d_n7;
        var_qcoth_dn8 = assign7270_e7386_d_n8;

        let assign7280_e7389: f64 = (var_k1 * var_q1);
        let assign7280_e7391: f64 = (assign7280_e7389 - var_qcoth);
        let assign7280_e7396: f64 = (var_t1 * var_t0);
        let assign7280_e7397: f64 = (var_qsqrt / assign7280_e7396);
        let assign7280_e7398: f64 = (1.0 - assign7280_e7397);
        let assign7280_e7399: f64 = (assign7280_e7391 / assign7280_e7398);
        var_qicores = assign7280_e7399;
        var_qicores_dn3 = (((((var_k1 * var_q1_dn3) - var_qcoth_dn3) * assign7280_e7398) - (assign7280_e7391 * (-(((var_qsqrt_dn3 * assign7280_e7396) - (var_qsqrt * ((var_t1_dn3 * var_t0) + (var_t1 * var_t0_dn3)))) / (assign7280_e7396 * assign7280_e7396))))) / (assign7280_e7398 * assign7280_e7398));
        var_qicores_dn4 = (((((var_k1 * var_q1_dn4) - var_qcoth_dn4) * assign7280_e7398) - (assign7280_e7391 * (-(((var_qsqrt_dn4 * assign7280_e7396) - (var_qsqrt * ((var_t1_dn4 * var_t0) + (var_t1 * var_t0_dn4)))) / (assign7280_e7396 * assign7280_e7396))))) / (assign7280_e7398 * assign7280_e7398));
        var_qicores_dn5 = (((((var_k1 * var_q1_dn5) - var_qcoth_dn5) * assign7280_e7398) - (assign7280_e7391 * (-(((var_qsqrt_dn5 * assign7280_e7396) - (var_qsqrt * ((var_t1_dn5 * var_t0) + (var_t1 * var_t0_dn5)))) / (assign7280_e7396 * assign7280_e7396))))) / (assign7280_e7398 * assign7280_e7398));
        var_qicores_dn6 = (((((var_k1 * var_q1_dn6) - var_qcoth_dn6) * assign7280_e7398) - (assign7280_e7391 * (-(((var_qsqrt_dn6 * assign7280_e7396) - (var_qsqrt * ((var_t1_dn6 * var_t0) + (var_t1 * var_t0_dn6)))) / (assign7280_e7396 * assign7280_e7396))))) / (assign7280_e7398 * assign7280_e7398));
        var_qicores_dn7 = (((((var_k1 * var_q1_dn7) - var_qcoth_dn7) * assign7280_e7398) - (assign7280_e7391 * (-(((var_qsqrt_dn7 * assign7280_e7396) - (var_qsqrt * ((var_t1_dn7 * var_t0) + (var_t1 * var_t0_dn7)))) / (assign7280_e7396 * assign7280_e7396))))) / (assign7280_e7398 * assign7280_e7398));
        var_qicores_dn8 = (((((var_k1 * var_q1_dn8) - var_qcoth_dn8) * assign7280_e7398) - (assign7280_e7391 * (-(((var_qsqrt_dn8 * assign7280_e7396) - (var_qsqrt * ((var_t1_dn8 * var_t0) + (var_t1 * var_t0_dn8)))) / (assign7280_e7396 * assign7280_e7396))))) / (assign7280_e7398 * assign7280_e7398));

        let assign7290_e7402: f64 = (var_q1 * var_cox1);
        let assign7290_e7404: f64 = (assign7290_e7402 * var_nvtm);
        var_qfronts = assign7290_e7404;
        var_qfronts_dn3 = (((var_q1_dn3 * var_cox1) * var_nvtm) + (assign7290_e7402 * var_nvtm_dn3));
        var_qfronts_dn4 = (((var_q1_dn4 * var_cox1) * var_nvtm) + (assign7290_e7402 * var_nvtm_dn4));
        var_qfronts_dn5 = (((var_q1_dn5 * var_cox1) * var_nvtm) + (assign7290_e7402 * var_nvtm_dn5));
        var_qfronts_dn6 = (((var_q1_dn6 * var_cox1) * var_nvtm) + (assign7290_e7402 * var_nvtm_dn6));
        var_qfronts_dn7 = (((var_q1_dn7 * var_cox1) * var_nvtm) + (assign7290_e7402 * var_nvtm_dn7));
        var_qfronts_dn8 = (((var_q1_dn8 * var_cox1) * var_nvtm) + (assign7290_e7402 * var_nvtm_dn8));

        let assign7300_e7407: f64 = (var_qicores * var_csi);
        let assign7300_e7409: f64 = (assign7300_e7407 * var_nvtm);
        var_qtots = assign7300_e7409;
        var_qtots_dn3 = (((var_qicores_dn3 * var_csi) * var_nvtm) + (assign7300_e7407 * var_nvtm_dn3));
        var_qtots_dn4 = (((var_qicores_dn4 * var_csi) * var_nvtm) + (assign7300_e7407 * var_nvtm_dn4));
        var_qtots_dn5 = (((var_qicores_dn5 * var_csi) * var_nvtm) + (assign7300_e7407 * var_nvtm_dn5));
        var_qtots_dn6 = (((var_qicores_dn6 * var_csi) * var_nvtm) + (assign7300_e7407 * var_nvtm_dn6));
        var_qtots_dn7 = (((var_qicores_dn7 * var_csi) * var_nvtm) + (assign7300_e7407 * var_nvtm_dn7));
        var_qtots_dn8 = (((var_qicores_dn8 * var_csi) * var_nvtm) + (assign7300_e7407 * var_nvtm_dn8));

        let assign7310_e7412: f64 = (var_qtots - var_qfronts);
        var_qbacks = assign7310_e7412;
        var_qbacks_dn3 = (var_qtots_dn3 - var_qfronts_dn3);
        var_qbacks_dn4 = (var_qtots_dn4 - var_qfronts_dn4);
        var_qbacks_dn5 = (var_qtots_dn5 - var_qfronts_dn5);
        var_qbacks_dn6 = (var_qtots_dn6 - var_qfronts_dn6);
        var_qbacks_dn7 = (var_qtots_dn7 - var_qfronts_dn7);
        var_qbacks_dn8 = (var_qtots_dn8 - var_qfronts_dn8);

        let assign7320_e7417: f64 = (var_cox2 * var_nvtm);
        let assign7320_e7418: f64 = (var_qbacks / assign7320_e7417);
        let assign7320_e7419: f64 = (var_xg2 - assign7320_e7418);
        var_phi2 = assign7320_e7419;
        var_phi2_dn3 = (var_xg2_dn3 - (((var_qbacks_dn3 * assign7320_e7417) - (var_qbacks * (var_cox2 * var_nvtm_dn3))) / (assign7320_e7417 * assign7320_e7417)));
        var_phi2_dn4 = (var_xg2_dn4 - (((var_qbacks_dn4 * assign7320_e7417) - (var_qbacks * (var_cox2 * var_nvtm_dn4))) / (assign7320_e7417 * assign7320_e7417)));
        var_phi2_dn5 = (var_xg2_dn5 - (((var_qbacks_dn5 * assign7320_e7417) - (var_qbacks * (var_cox2 * var_nvtm_dn5))) / (assign7320_e7417 * assign7320_e7417)));
        var_phi2_dn6 = (var_xg2_dn6 - (((var_qbacks_dn6 * assign7320_e7417) - (var_qbacks * (var_cox2 * var_nvtm_dn6))) / (assign7320_e7417 * assign7320_e7417)));
        var_phi2_dn7 = (var_xg2_dn7 - (((var_qbacks_dn7 * assign7320_e7417) - (var_qbacks * (var_cox2 * var_nvtm_dn7))) / (assign7320_e7417 * assign7320_e7417)));
        var_phi2_dn8 = (var_xg2_dn8 - (((var_qbacks_dn8 * assign7320_e7417) - (var_qbacks * (var_cox2 * var_nvtm_dn8))) / (assign7320_e7417 * assign7320_e7417)));

        let assign7330_e7422: f64 = (var_phi1 + var_phi2);
        let assign7330_e7424: f64 = (assign7330_e7422 * var_nvtm);
        let assign7330_e7426: f64 = (assign7330_e7424 / 2.0);
        var_phifs = assign7330_e7426;
        var_phifs_dn3 = ((((var_phi1_dn3 + var_phi2_dn3) * var_nvtm) + (assign7330_e7422 * var_nvtm_dn3)) / 2.0);
        var_phifs_dn4 = ((((var_phi1_dn4 + var_phi2_dn4) * var_nvtm) + (assign7330_e7422 * var_nvtm_dn4)) / 2.0);
        var_phifs_dn5 = ((((var_phi1_dn5 + var_phi2_dn5) * var_nvtm) + (assign7330_e7422 * var_nvtm_dn5)) / 2.0);
        var_phifs_dn6 = ((((var_phi1_dn6 + var_phi2_dn6) * var_nvtm) + (assign7330_e7422 * var_nvtm_dn6)) / 2.0);
        var_phifs_dn7 = ((((var_phi1_dn7 + var_phi2_dn7) * var_nvtm) + (assign7330_e7422 * var_nvtm_dn7)) / 2.0);
        var_phifs_dn8 = ((((var_phi1_dn8 + var_phi2_dn8) * var_nvtm) + (assign7330_e7422 * var_nvtm_dn8)) / 2.0);

        let assign7340_e7429: f64 = (var_qtots / var_cox1);
        var_qis = assign7340_e7429;
        var_qis_dn3 = (var_qtots_dn3 / var_cox1);
        var_qis_dn4 = (var_qtots_dn4 / var_cox1);
        var_qis_dn5 = (var_qtots_dn5 / var_cox1);
        var_qis_dn6 = (var_qtots_dn6 / var_cox1);
        var_qis_dn7 = (var_qtots_dn7 / var_cox1);
        var_qis_dn8 = (var_qtots_dn8 / var_cox1);

        let assign7350_e7432: f64 = (1.60219e-19 * var_nbody_i);
        let assign7350_e7434: f64 = (assign7350_e7432 * p.p49);
        let assign7350_e7436: f64 = (assign7350_e7434 / var_cox1);
        var_qbs = assign7350_e7436;

        let assign7360_e7439: f64 = (var_eta_mu * var_qfronts);
        let assign7360_e7441: f64 = (assign7360_e7439 / var_cox1);
        let assign7360_e7443: f64 = (assign7360_e7441 + var_qbs);
        var_t2 = assign7360_e7443;
        var_t2_dn3 = ((var_eta_mu * var_qfronts_dn3) / var_cox1);
        var_t2_dn4 = ((var_eta_mu * var_qfronts_dn4) / var_cox1);
        var_t2_dn5 = ((var_eta_mu * var_qfronts_dn5) / var_cox1);
        var_t2_dn6 = ((var_eta_mu * var_qfronts_dn6) / var_cox1);
        var_t2_dn7 = ((var_eta_mu * var_qfronts_dn7) / var_cox1);
        var_t2_dn8 = ((var_eta_mu * var_qfronts_dn8) / var_cox1);

        let assign7370_e7448: f64 = (var_t2 * var_t2);
        let assign7370_e7450: f64 = (assign7370_e7448 + 0.001);
        let assign7370_e7451: f64 = (assign7370_e7450).sqrt();
        let assign7370_e7452: f64 = (var_t2 + assign7370_e7451);
        let assign7370_e7453: f64 = (0.5 * assign7370_e7452);
        var_t3 = assign7370_e7453;
        var_t3_dn3 = (0.5 * (var_t2_dn3 + (((var_t2_dn3 * var_t2) + (var_t2 * var_t2_dn3)) / (2.0 * assign7370_e7451))));
        var_t3_dn4 = (0.5 * (var_t2_dn4 + (((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)) / (2.0 * assign7370_e7451))));
        var_t3_dn5 = (0.5 * (var_t2_dn5 + (((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)) / (2.0 * assign7370_e7451))));
        var_t3_dn6 = (0.5 * (var_t2_dn6 + (((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)) / (2.0 * assign7370_e7451))));
        var_t3_dn7 = (0.5 * (var_t2_dn7 + (((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)) / (2.0 * assign7370_e7451))));
        var_t3_dn8 = (0.5 * (var_t2_dn8 + (((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)) / (2.0 * assign7370_e7451))));

        let assign7380_e7456: f64 = (var_eefffactor * var_t3);
        var_eeffs = assign7380_e7456;
        var_eeffs_dn3 = (var_eefffactor * var_t3_dn3);
        var_eeffs_dn4 = (var_eefffactor * var_t3_dn4);
        var_eeffs_dn5 = (var_eefffactor * var_t3_dn5);
        var_eeffs_dn6 = (var_eefffactor * var_t3_dn6);
        var_eeffs_dn7 = (var_eefffactor * var_t3_dn7);
        var_eeffs_dn8 = (var_eefffactor * var_t3_dn8);

        let assign7390_e7459: f64 = (var_eta_mu2 * var_qbacks);
        let assign7390_e7461: f64 = (assign7390_e7459 / var_cox2);
        let assign7390_e7463: f64 = (assign7390_e7461 + var_qbs);
        var_t2 = assign7390_e7463;
        var_t2_dn3 = ((var_eta_mu2 * var_qbacks_dn3) / var_cox2);
        var_t2_dn4 = ((var_eta_mu2 * var_qbacks_dn4) / var_cox2);
        var_t2_dn5 = ((var_eta_mu2 * var_qbacks_dn5) / var_cox2);
        var_t2_dn6 = ((var_eta_mu2 * var_qbacks_dn6) / var_cox2);
        var_t2_dn7 = ((var_eta_mu2 * var_qbacks_dn7) / var_cox2);
        var_t2_dn8 = ((var_eta_mu2 * var_qbacks_dn8) / var_cox2);

        let assign7400_e7468: f64 = (var_t2 * var_t2);
        let assign7400_e7470: f64 = (assign7400_e7468 + 0.001);
        let assign7400_e7471: f64 = (assign7400_e7470).sqrt();
        let assign7400_e7472: f64 = (var_t2 + assign7400_e7471);
        let assign7400_e7473: f64 = (0.5 * assign7400_e7472);
        var_t3 = assign7400_e7473;
        var_t3_dn3 = (0.5 * (var_t2_dn3 + (((var_t2_dn3 * var_t2) + (var_t2 * var_t2_dn3)) / (2.0 * assign7400_e7471))));
        var_t3_dn4 = (0.5 * (var_t2_dn4 + (((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)) / (2.0 * assign7400_e7471))));
        var_t3_dn5 = (0.5 * (var_t2_dn5 + (((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)) / (2.0 * assign7400_e7471))));
        var_t3_dn6 = (0.5 * (var_t2_dn6 + (((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)) / (2.0 * assign7400_e7471))));
        var_t3_dn7 = (0.5 * (var_t2_dn7 + (((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)) / (2.0 * assign7400_e7471))));
        var_t3_dn8 = (0.5 * (var_t2_dn8 + (((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)) / (2.0 * assign7400_e7471))));

        let assign7410_e7476: f64 = (var_eefffactor2 * var_t3);
        var_eeffs2 = assign7410_e7476;
        var_eeffs2_dn3 = (var_eefffactor2 * var_t3_dn3);
        var_eeffs2_dn4 = (var_eefffactor2 * var_t3_dn4);
        var_eeffs2_dn5 = (var_eefffactor2 * var_t3_dn5);
        var_eeffs2_dn6 = (var_eefffactor2 * var_t3_dn6);
        var_eeffs2_dn7 = (var_eefffactor2 * var_t3_dn7);
        var_eeffs2_dn8 = (var_eefffactor2 * var_t3_dn8);

        let assign7420_e7479: f64 = (0.01 / var_cox1);
        var_qb0 = assign7420_e7479;

        let assign7430_e7484: f64 = (var_qis / var_qb0);
        let assign7430_e7485: f64 = (assign7430_e7484).abs();
        let assign7430_e7486: f64 = (1.0 + assign7430_e7485);
        let assign7430_e7487: f64 = (0.5 * assign7430_e7486);
        let assign7430_e7489: f64 = (assign7430_e7487).powf(var_ucs_t);
        var_t2__blk83 = assign7430_e7489;
        var_t2__blk83_dn3 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign7430_e7487).powf(var_ucs_t - 1.0) * (0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn3 / var_qb0) } else { (-(var_qis_dn3 / var_qb0)) }))) } } else { (assign7430_e7489 * (var_ucs_t * ((0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn3 / var_qb0) } else { (-(var_qis_dn3 / var_qb0)) }) / assign7430_e7487))) };
        var_t2__blk83_dn4 = if var_ucs_t_dn4 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign7430_e7487).powf(var_ucs_t - 1.0) * (0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn4 / var_qb0) } else { (-(var_qis_dn4 / var_qb0)) }))) } } else { (assign7430_e7489 * ((var_ucs_t_dn4 * (assign7430_e7487).ln()) + (var_ucs_t * ((0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn4 / var_qb0) } else { (-(var_qis_dn4 / var_qb0)) }) / assign7430_e7487)))) };
        var_t2__blk83_dn5 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign7430_e7487).powf(var_ucs_t - 1.0) * (0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn5 / var_qb0) } else { (-(var_qis_dn5 / var_qb0)) }))) } } else { (assign7430_e7489 * (var_ucs_t * ((0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn5 / var_qb0) } else { (-(var_qis_dn5 / var_qb0)) }) / assign7430_e7487))) };
        var_t2__blk83_dn6 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign7430_e7487).powf(var_ucs_t - 1.0) * (0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn6 / var_qb0) } else { (-(var_qis_dn6 / var_qb0)) }))) } } else { (assign7430_e7489 * (var_ucs_t * ((0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn6 / var_qb0) } else { (-(var_qis_dn6 / var_qb0)) }) / assign7430_e7487))) };
        var_t2__blk83_dn7 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign7430_e7487).powf(var_ucs_t - 1.0) * (0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn7 / var_qb0) } else { (-(var_qis_dn7 / var_qb0)) }))) } } else { (assign7430_e7489 * (var_ucs_t * ((0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn7 / var_qb0) } else { (-(var_qis_dn7 / var_qb0)) }) / assign7430_e7487))) };
        var_t2__blk83_dn8 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign7430_e7487).powf(var_ucs_t - 1.0) * (0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn8 / var_qb0) } else { (-(var_qis_dn8 / var_qb0)) }))) } } else { (assign7430_e7489 * (var_ucs_t * ((0.5 * if assign7430_e7484 >= 0.0 { (var_qis_dn8 / var_qb0) } else { (-(var_qis_dn8 / var_qb0)) }) / assign7430_e7487))) };

        let assign7440_e7493: f64 = (var_vbgs * var_uc_t);
        let assign7440_e7494: f64 = (var_ua_t + assign7440_e7493);
        let assign7440_e7496: f64 = (var_eeffs).abs();
        let assign7440_e7500: f64 = (var_eub_i * var_vbgs);
        let assign7440_e7501: f64 = (var_eu_i + assign7440_e7500);
        let assign7440_e7502: f64 = (assign7440_e7496).powf(assign7440_e7501);
        let assign7440_e7503: f64 = (assign7440_e7494 * assign7440_e7502);
        let assign7440_e7506: f64 = (var_ud_t / var_t2__blk83);
        let assign7440_e7507: f64 = (assign7440_e7503 + assign7440_e7506);
        var_t3__blk84 = assign7440_e7507;
        var_t3__blk84_dn3 = ((((var_vbgs_dn3 * var_uc_t) * assign7440_e7502) + (assign7440_e7494 * if (var_eub_i * var_vbgs_dn3) == 0.0 && ((assign7440_e7501) as f64).is_finite() && ((assign7440_e7501) as f64).fract() == 0.0 { if assign7440_e7501 == 0.0 { 0.0 } else { (assign7440_e7501 * ((assign7440_e7496).powf(assign7440_e7501 - 1.0) * if var_eeffs >= 0.0 { var_eeffs_dn3 } else { (-var_eeffs_dn3) })) } } else { (assign7440_e7502 * (((var_eub_i * var_vbgs_dn3) * (assign7440_e7496).ln()) + (assign7440_e7501 * (if var_eeffs >= 0.0 { var_eeffs_dn3 } else { (-var_eeffs_dn3) } / assign7440_e7496)))) })) + (-((var_ud_t * var_t2__blk83_dn3) / (var_t2__blk83 * var_t2__blk83))));
        var_t3__blk84_dn4 = ((((var_ua_t_dn4 + (var_vbgs * var_uc_t_dn4)) * assign7440_e7502) + (assign7440_e7494 * if 0.0 == 0.0 && ((assign7440_e7501) as f64).is_finite() && ((assign7440_e7501) as f64).fract() == 0.0 { if assign7440_e7501 == 0.0 { 0.0 } else { (assign7440_e7501 * ((assign7440_e7496).powf(assign7440_e7501 - 1.0) * if var_eeffs >= 0.0 { var_eeffs_dn4 } else { (-var_eeffs_dn4) })) } } else { (assign7440_e7502 * (assign7440_e7501 * (if var_eeffs >= 0.0 { var_eeffs_dn4 } else { (-var_eeffs_dn4) } / assign7440_e7496))) })) + (((var_ud_t_dn4 * var_t2__blk83) - (var_ud_t * var_t2__blk83_dn4)) / (var_t2__blk83 * var_t2__blk83)));
        var_t3__blk84_dn5 = ((((var_vbgs_dn5 * var_uc_t) * assign7440_e7502) + (assign7440_e7494 * if (var_eub_i * var_vbgs_dn5) == 0.0 && ((assign7440_e7501) as f64).is_finite() && ((assign7440_e7501) as f64).fract() == 0.0 { if assign7440_e7501 == 0.0 { 0.0 } else { (assign7440_e7501 * ((assign7440_e7496).powf(assign7440_e7501 - 1.0) * if var_eeffs >= 0.0 { var_eeffs_dn5 } else { (-var_eeffs_dn5) })) } } else { (assign7440_e7502 * (((var_eub_i * var_vbgs_dn5) * (assign7440_e7496).ln()) + (assign7440_e7501 * (if var_eeffs >= 0.0 { var_eeffs_dn5 } else { (-var_eeffs_dn5) } / assign7440_e7496)))) })) + (-((var_ud_t * var_t2__blk83_dn5) / (var_t2__blk83 * var_t2__blk83))));
        var_t3__blk84_dn6 = ((((var_vbgs_dn6 * var_uc_t) * assign7440_e7502) + (assign7440_e7494 * if (var_eub_i * var_vbgs_dn6) == 0.0 && ((assign7440_e7501) as f64).is_finite() && ((assign7440_e7501) as f64).fract() == 0.0 { if assign7440_e7501 == 0.0 { 0.0 } else { (assign7440_e7501 * ((assign7440_e7496).powf(assign7440_e7501 - 1.0) * if var_eeffs >= 0.0 { var_eeffs_dn6 } else { (-var_eeffs_dn6) })) } } else { (assign7440_e7502 * (((var_eub_i * var_vbgs_dn6) * (assign7440_e7496).ln()) + (assign7440_e7501 * (if var_eeffs >= 0.0 { var_eeffs_dn6 } else { (-var_eeffs_dn6) } / assign7440_e7496)))) })) + (-((var_ud_t * var_t2__blk83_dn6) / (var_t2__blk83 * var_t2__blk83))));
        var_t3__blk84_dn7 = ((assign7440_e7494 * if 0.0 == 0.0 && ((assign7440_e7501) as f64).is_finite() && ((assign7440_e7501) as f64).fract() == 0.0 { if assign7440_e7501 == 0.0 { 0.0 } else { (assign7440_e7501 * ((assign7440_e7496).powf(assign7440_e7501 - 1.0) * if var_eeffs >= 0.0 { var_eeffs_dn7 } else { (-var_eeffs_dn7) })) } } else { (assign7440_e7502 * (assign7440_e7501 * (if var_eeffs >= 0.0 { var_eeffs_dn7 } else { (-var_eeffs_dn7) } / assign7440_e7496))) }) + (-((var_ud_t * var_t2__blk83_dn7) / (var_t2__blk83 * var_t2__blk83))));
        var_t3__blk84_dn8 = ((assign7440_e7494 * if 0.0 == 0.0 && ((assign7440_e7501) as f64).is_finite() && ((assign7440_e7501) as f64).fract() == 0.0 { if assign7440_e7501 == 0.0 { 0.0 } else { (assign7440_e7501 * ((assign7440_e7496).powf(assign7440_e7501 - 1.0) * if var_eeffs >= 0.0 { var_eeffs_dn8 } else { (-var_eeffs_dn8) })) } } else { (assign7440_e7502 * (assign7440_e7501 * (if var_eeffs >= 0.0 { var_eeffs_dn8 } else { (-var_eeffs_dn8) } / assign7440_e7496))) }) + (-((var_ud_t * var_t2__blk83_dn8) / (var_t2__blk83 * var_t2__blk83))));

        let assign7450_e7510: f64 = (1.0 + var_t3__blk84);
        var_dmobs = assign7450_e7510;
        var_dmobs_dn3 = var_t3__blk84_dn3;
        var_dmobs_dn4 = var_t3__blk84_dn4;
        var_dmobs_dn5 = var_t3__blk84_dn5;
        var_dmobs_dn6 = var_t3__blk84_dn6;
        var_dmobs_dn7 = var_t3__blk84_dn7;
        var_dmobs_dn8 = var_t3__blk84_dn8;

        let assign7460_e7514: f64 = (var_dmobs + 1.0);
        let assign7460_e7517: f64 = (var_dmobs - 1.0);
        let assign7460_e7520: f64 = (var_dmobs - 1.0);
        let assign7460_e7521: f64 = (assign7460_e7517 * assign7460_e7520);
        let assign7460_e7524: f64 = (0.25 * p.p154);
        let assign7460_e7526: f64 = (assign7460_e7524 * p.p154);
        let assign7460_e7527: f64 = (assign7460_e7521 + assign7460_e7526);
        let assign7460_e7528: f64 = (assign7460_e7527).sqrt();
        let assign7460_e7529: f64 = (assign7460_e7514 + assign7460_e7528);
        let assign7460_e7530: f64 = (0.5 * assign7460_e7529);
        var_dmobs = assign7460_e7530;
        var_dmobs_dn3 = (0.5 * (var_dmobs_dn3 + (((var_dmobs_dn3 * assign7460_e7520) + (assign7460_e7517 * var_dmobs_dn3)) / (2.0 * assign7460_e7528))));
        var_dmobs_dn4 = (0.5 * (var_dmobs_dn4 + (((var_dmobs_dn4 * assign7460_e7520) + (assign7460_e7517 * var_dmobs_dn4)) / (2.0 * assign7460_e7528))));
        var_dmobs_dn5 = (0.5 * (var_dmobs_dn5 + (((var_dmobs_dn5 * assign7460_e7520) + (assign7460_e7517 * var_dmobs_dn5)) / (2.0 * assign7460_e7528))));
        var_dmobs_dn6 = (0.5 * (var_dmobs_dn6 + (((var_dmobs_dn6 * assign7460_e7520) + (assign7460_e7517 * var_dmobs_dn6)) / (2.0 * assign7460_e7528))));
        var_dmobs_dn7 = (0.5 * (var_dmobs_dn7 + (((var_dmobs_dn7 * assign7460_e7520) + (assign7460_e7517 * var_dmobs_dn7)) / (2.0 * assign7460_e7528))));
        var_dmobs_dn8 = (0.5 * (var_dmobs_dn8 + (((var_dmobs_dn8 * assign7460_e7520) + (assign7460_e7517 * var_dmobs_dn8)) / (2.0 * assign7460_e7528))));

        let assign7470_e7533: f64 = (var_dmobs / p.p11);
        var_dmobs = assign7470_e7533;
        var_dmobs_dn3 = (var_dmobs_dn3 / p.p11);
        var_dmobs_dn4 = (var_dmobs_dn4 / p.p11);
        var_dmobs_dn5 = (var_dmobs_dn5 / p.p11);
        var_dmobs_dn6 = (var_dmobs_dn6 / p.p11);
        var_dmobs_dn7 = (var_dmobs_dn7 / p.p11);
        var_dmobs_dn8 = (var_dmobs_dn8 / p.p11);

        let assign7480_e7536: f64 = (var_u0_t / var_dmobs);
        var_ueff1 = assign7480_e7536;
        var_ueff1_dn3 = (-((var_u0_t * var_dmobs_dn3) / (var_dmobs * var_dmobs)));
        var_ueff1_dn4 = (((var_u0_t_dn4 * var_dmobs) - (var_u0_t * var_dmobs_dn4)) / (var_dmobs * var_dmobs));
        var_ueff1_dn5 = (-((var_u0_t * var_dmobs_dn5) / (var_dmobs * var_dmobs)));
        var_ueff1_dn6 = (-((var_u0_t * var_dmobs_dn6) / (var_dmobs * var_dmobs)));
        var_ueff1_dn7 = (-((var_u0_t * var_dmobs_dn7) / (var_dmobs * var_dmobs)));
        var_ueff1_dn8 = (-((var_u0_t * var_dmobs_dn8) / (var_dmobs * var_dmobs)));

        let assign7490_e7541: f64 = (var_qis / var_qb0);
        let assign7490_e7542: f64 = (assign7490_e7541).abs();
        let assign7490_e7543: f64 = (1.0 + assign7490_e7542);
        let assign7490_e7544: f64 = (0.5 * assign7490_e7543);
        let assign7490_e7546: f64 = (assign7490_e7544).powf(var_ucs2_i);
        var_t2__blk85 = assign7490_e7546;
        var_t2__blk85_dn3 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign7490_e7544).powf(var_ucs2_i - 1.0) * (0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn3 / var_qb0) } else { (-(var_qis_dn3 / var_qb0)) }))) } } else { (assign7490_e7546 * (var_ucs2_i * ((0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn3 / var_qb0) } else { (-(var_qis_dn3 / var_qb0)) }) / assign7490_e7544))) };
        var_t2__blk85_dn4 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign7490_e7544).powf(var_ucs2_i - 1.0) * (0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn4 / var_qb0) } else { (-(var_qis_dn4 / var_qb0)) }))) } } else { (assign7490_e7546 * (var_ucs2_i * ((0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn4 / var_qb0) } else { (-(var_qis_dn4 / var_qb0)) }) / assign7490_e7544))) };
        var_t2__blk85_dn5 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign7490_e7544).powf(var_ucs2_i - 1.0) * (0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn5 / var_qb0) } else { (-(var_qis_dn5 / var_qb0)) }))) } } else { (assign7490_e7546 * (var_ucs2_i * ((0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn5 / var_qb0) } else { (-(var_qis_dn5 / var_qb0)) }) / assign7490_e7544))) };
        var_t2__blk85_dn6 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign7490_e7544).powf(var_ucs2_i - 1.0) * (0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn6 / var_qb0) } else { (-(var_qis_dn6 / var_qb0)) }))) } } else { (assign7490_e7546 * (var_ucs2_i * ((0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn6 / var_qb0) } else { (-(var_qis_dn6 / var_qb0)) }) / assign7490_e7544))) };
        var_t2__blk85_dn7 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign7490_e7544).powf(var_ucs2_i - 1.0) * (0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn7 / var_qb0) } else { (-(var_qis_dn7 / var_qb0)) }))) } } else { (assign7490_e7546 * (var_ucs2_i * ((0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn7 / var_qb0) } else { (-(var_qis_dn7 / var_qb0)) }) / assign7490_e7544))) };
        var_t2__blk85_dn8 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign7490_e7544).powf(var_ucs2_i - 1.0) * (0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn8 / var_qb0) } else { (-(var_qis_dn8 / var_qb0)) }))) } } else { (assign7490_e7546 * (var_ucs2_i * ((0.5 * if assign7490_e7541 >= 0.0 { (var_qis_dn8 / var_qb0) } else { (-(var_qis_dn8 / var_qb0)) }) / assign7490_e7544))) };

        let assign7500_e7550: f64 = (var_vbgs * var_uc2_i);
        let assign7500_e7551: f64 = (var_ua2_i + assign7500_e7550);
        let assign7500_e7553: f64 = (var_eeffs2).abs();
        let assign7500_e7557: f64 = (var_eub2_i * var_vbgs);
        let assign7500_e7558: f64 = (var_eu2_i + assign7500_e7557);
        let assign7500_e7559: f64 = (assign7500_e7553).powf(assign7500_e7558);
        let assign7500_e7560: f64 = (assign7500_e7551 * assign7500_e7559);
        let assign7500_e7563: f64 = (var_ud2_i / var_t2__blk85);
        let assign7500_e7564: f64 = (assign7500_e7560 + assign7500_e7563);
        var_t3__blk86 = assign7500_e7564;
        var_t3__blk86_dn3 = ((((var_vbgs_dn3 * var_uc2_i) * assign7500_e7559) + (assign7500_e7551 * if (var_eub2_i * var_vbgs_dn3) == 0.0 && ((assign7500_e7558) as f64).is_finite() && ((assign7500_e7558) as f64).fract() == 0.0 { if assign7500_e7558 == 0.0 { 0.0 } else { (assign7500_e7558 * ((assign7500_e7553).powf(assign7500_e7558 - 1.0) * if var_eeffs2 >= 0.0 { var_eeffs2_dn3 } else { (-var_eeffs2_dn3) })) } } else { (assign7500_e7559 * (((var_eub2_i * var_vbgs_dn3) * (assign7500_e7553).ln()) + (assign7500_e7558 * (if var_eeffs2 >= 0.0 { var_eeffs2_dn3 } else { (-var_eeffs2_dn3) } / assign7500_e7553)))) })) + (-((var_ud2_i * var_t2__blk85_dn3) / (var_t2__blk85 * var_t2__blk85))));
        var_t3__blk86_dn4 = ((assign7500_e7551 * if 0.0 == 0.0 && ((assign7500_e7558) as f64).is_finite() && ((assign7500_e7558) as f64).fract() == 0.0 { if assign7500_e7558 == 0.0 { 0.0 } else { (assign7500_e7558 * ((assign7500_e7553).powf(assign7500_e7558 - 1.0) * if var_eeffs2 >= 0.0 { var_eeffs2_dn4 } else { (-var_eeffs2_dn4) })) } } else { (assign7500_e7559 * (assign7500_e7558 * (if var_eeffs2 >= 0.0 { var_eeffs2_dn4 } else { (-var_eeffs2_dn4) } / assign7500_e7553))) }) + (-((var_ud2_i * var_t2__blk85_dn4) / (var_t2__blk85 * var_t2__blk85))));
        var_t3__blk86_dn5 = ((((var_vbgs_dn5 * var_uc2_i) * assign7500_e7559) + (assign7500_e7551 * if (var_eub2_i * var_vbgs_dn5) == 0.0 && ((assign7500_e7558) as f64).is_finite() && ((assign7500_e7558) as f64).fract() == 0.0 { if assign7500_e7558 == 0.0 { 0.0 } else { (assign7500_e7558 * ((assign7500_e7553).powf(assign7500_e7558 - 1.0) * if var_eeffs2 >= 0.0 { var_eeffs2_dn5 } else { (-var_eeffs2_dn5) })) } } else { (assign7500_e7559 * (((var_eub2_i * var_vbgs_dn5) * (assign7500_e7553).ln()) + (assign7500_e7558 * (if var_eeffs2 >= 0.0 { var_eeffs2_dn5 } else { (-var_eeffs2_dn5) } / assign7500_e7553)))) })) + (-((var_ud2_i * var_t2__blk85_dn5) / (var_t2__blk85 * var_t2__blk85))));
        var_t3__blk86_dn6 = ((((var_vbgs_dn6 * var_uc2_i) * assign7500_e7559) + (assign7500_e7551 * if (var_eub2_i * var_vbgs_dn6) == 0.0 && ((assign7500_e7558) as f64).is_finite() && ((assign7500_e7558) as f64).fract() == 0.0 { if assign7500_e7558 == 0.0 { 0.0 } else { (assign7500_e7558 * ((assign7500_e7553).powf(assign7500_e7558 - 1.0) * if var_eeffs2 >= 0.0 { var_eeffs2_dn6 } else { (-var_eeffs2_dn6) })) } } else { (assign7500_e7559 * (((var_eub2_i * var_vbgs_dn6) * (assign7500_e7553).ln()) + (assign7500_e7558 * (if var_eeffs2 >= 0.0 { var_eeffs2_dn6 } else { (-var_eeffs2_dn6) } / assign7500_e7553)))) })) + (-((var_ud2_i * var_t2__blk85_dn6) / (var_t2__blk85 * var_t2__blk85))));
        var_t3__blk86_dn7 = ((assign7500_e7551 * if 0.0 == 0.0 && ((assign7500_e7558) as f64).is_finite() && ((assign7500_e7558) as f64).fract() == 0.0 { if assign7500_e7558 == 0.0 { 0.0 } else { (assign7500_e7558 * ((assign7500_e7553).powf(assign7500_e7558 - 1.0) * if var_eeffs2 >= 0.0 { var_eeffs2_dn7 } else { (-var_eeffs2_dn7) })) } } else { (assign7500_e7559 * (assign7500_e7558 * (if var_eeffs2 >= 0.0 { var_eeffs2_dn7 } else { (-var_eeffs2_dn7) } / assign7500_e7553))) }) + (-((var_ud2_i * var_t2__blk85_dn7) / (var_t2__blk85 * var_t2__blk85))));
        var_t3__blk86_dn8 = ((assign7500_e7551 * if 0.0 == 0.0 && ((assign7500_e7558) as f64).is_finite() && ((assign7500_e7558) as f64).fract() == 0.0 { if assign7500_e7558 == 0.0 { 0.0 } else { (assign7500_e7558 * ((assign7500_e7553).powf(assign7500_e7558 - 1.0) * if var_eeffs2 >= 0.0 { var_eeffs2_dn8 } else { (-var_eeffs2_dn8) })) } } else { (assign7500_e7559 * (assign7500_e7558 * (if var_eeffs2 >= 0.0 { var_eeffs2_dn8 } else { (-var_eeffs2_dn8) } / assign7500_e7553))) }) + (-((var_ud2_i * var_t2__blk85_dn8) / (var_t2__blk85 * var_t2__blk85))));

        let assign7510_e7567: f64 = (1.0 + var_t3__blk86);
        var_dmobs = assign7510_e7567;
        var_dmobs_dn3 = var_t3__blk86_dn3;
        var_dmobs_dn4 = var_t3__blk86_dn4;
        var_dmobs_dn5 = var_t3__blk86_dn5;
        var_dmobs_dn6 = var_t3__blk86_dn6;
        var_dmobs_dn7 = var_t3__blk86_dn7;
        var_dmobs_dn8 = var_t3__blk86_dn8;

        let assign7520_e7571: f64 = (var_dmobs + 1.0);
        let assign7520_e7574: f64 = (var_dmobs - 1.0);
        let assign7520_e7577: f64 = (var_dmobs - 1.0);
        let assign7520_e7578: f64 = (assign7520_e7574 * assign7520_e7577);
        let assign7520_e7581: f64 = (0.25 * p.p154);
        let assign7520_e7583: f64 = (assign7520_e7581 * p.p154);
        let assign7520_e7584: f64 = (assign7520_e7578 + assign7520_e7583);
        let assign7520_e7585: f64 = (assign7520_e7584).sqrt();
        let assign7520_e7586: f64 = (assign7520_e7571 + assign7520_e7585);
        let assign7520_e7587: f64 = (0.5 * assign7520_e7586);
        var_dmobs = assign7520_e7587;
        var_dmobs_dn3 = (0.5 * (var_dmobs_dn3 + (((var_dmobs_dn3 * assign7520_e7577) + (assign7520_e7574 * var_dmobs_dn3)) / (2.0 * assign7520_e7585))));
        var_dmobs_dn4 = (0.5 * (var_dmobs_dn4 + (((var_dmobs_dn4 * assign7520_e7577) + (assign7520_e7574 * var_dmobs_dn4)) / (2.0 * assign7520_e7585))));
        var_dmobs_dn5 = (0.5 * (var_dmobs_dn5 + (((var_dmobs_dn5 * assign7520_e7577) + (assign7520_e7574 * var_dmobs_dn5)) / (2.0 * assign7520_e7585))));
        var_dmobs_dn6 = (0.5 * (var_dmobs_dn6 + (((var_dmobs_dn6 * assign7520_e7577) + (assign7520_e7574 * var_dmobs_dn6)) / (2.0 * assign7520_e7585))));
        var_dmobs_dn7 = (0.5 * (var_dmobs_dn7 + (((var_dmobs_dn7 * assign7520_e7577) + (assign7520_e7574 * var_dmobs_dn7)) / (2.0 * assign7520_e7585))));
        var_dmobs_dn8 = (0.5 * (var_dmobs_dn8 + (((var_dmobs_dn8 * assign7520_e7577) + (assign7520_e7574 * var_dmobs_dn8)) / (2.0 * assign7520_e7585))));

        let assign7530_e7590: f64 = (var_dmobs / p.p11);
        var_dmobs = assign7530_e7590;
        var_dmobs_dn3 = (var_dmobs_dn3 / p.p11);
        var_dmobs_dn4 = (var_dmobs_dn4 / p.p11);
        var_dmobs_dn5 = (var_dmobs_dn5 / p.p11);
        var_dmobs_dn6 = (var_dmobs_dn6 / p.p11);
        var_dmobs_dn7 = (var_dmobs_dn7 / p.p11);
        var_dmobs_dn8 = (var_dmobs_dn8 / p.p11);

        let assign7540_e7593: f64 = (var_u02_i / var_dmobs);
        var_ueff2 = assign7540_e7593;
        var_ueff2_dn3 = (-((var_u02_i * var_dmobs_dn3) / (var_dmobs * var_dmobs)));
        var_ueff2_dn4 = (-((var_u02_i * var_dmobs_dn4) / (var_dmobs * var_dmobs)));
        var_ueff2_dn5 = (-((var_u02_i * var_dmobs_dn5) / (var_dmobs * var_dmobs)));
        var_ueff2_dn6 = (-((var_u02_i * var_dmobs_dn6) / (var_dmobs * var_dmobs)));
        var_ueff2_dn7 = (-((var_u02_i * var_dmobs_dn7) / (var_dmobs * var_dmobs)));
        var_ueff2_dn8 = (-((var_u02_i * var_dmobs_dn8) / (var_dmobs * var_dmobs)));

        let assign7550_e7597: f64 = (var_qfronts / var_cox1);
        let assign7550_e7598: f64 = (var_vgfb1eff - assign7550_e7597);
        var_t0 = assign7550_e7598;
        var_t0_dn3 = (var_vgfb1eff_dn3 - (var_qfronts_dn3 / var_cox1));
        var_t0_dn4 = (var_vgfb1eff_dn4 - (var_qfronts_dn4 / var_cox1));
        var_t0_dn5 = (var_vgfb1eff_dn5 - (var_qfronts_dn5 / var_cox1));
        var_t0_dn6 = (var_vgfb1eff_dn6 - (var_qfronts_dn6 / var_cox1));
        var_t0_dn7 = (var_vgfb1eff_dn7 - (var_qfronts_dn7 / var_cox1));
        var_t0_dn8 = (var_vgfb1eff_dn8 - (var_qfronts_dn8 / var_cox1));

        let assign7560_e7601: f64 = (var_vgfb2 - var_dvth_all);
        let assign7560_e7604: f64 = (var_qbacks / var_cox2);
        let assign7560_e7605: f64 = (assign7560_e7601 - assign7560_e7604);
        var_t1 = assign7560_e7605;
        var_t1_dn3 = ((var_vgfb2_dn3 - var_dvth_all_dn3) - (var_qbacks_dn3 / var_cox2));
        var_t1_dn4 = ((var_vgfb2_dn4 - var_dvth_all_dn4) - (var_qbacks_dn4 / var_cox2));
        var_t1_dn5 = ((var_vgfb2_dn5 - var_dvth_all_dn5) - (var_qbacks_dn5 / var_cox2));
        var_t1_dn6 = ((var_vgfb2_dn6 - var_dvth_all_dn6) - (var_qbacks_dn6 / var_cox2));
        var_t1_dn7 = ((var_vgfb2_dn7 - var_dvth_all_dn7) - (var_qbacks_dn7 / var_cox2));
        var_t1_dn8 = ((var_vgfb2_dn8 - var_dvth_all_dn8) - (var_qbacks_dn8 / var_cox2));

        let assign7570_e7608: f64 = (var_t0 / var_nvtm);
        let assign7570_e7609: f64 = (assign7570_e7608).exp();
        let assign7570_e7612: f64 = (var_t0 / var_nvtm);
        let assign7570_e7613: f64 = (assign7570_e7612).exp();
        let assign7570_e7616: f64 = (var_t1 / var_nvtm);
        let assign7570_e7617: f64 = (assign7570_e7616).exp();
        let assign7570_e7618: f64 = (assign7570_e7613 + assign7570_e7617);
        let assign7570_e7619: f64 = (assign7570_e7609 / assign7570_e7618);
        var_w1 = assign7570_e7619;
        var_w1_dn3 = ((((assign7570_e7609 * (((var_t0_dn3 * var_nvtm) - (var_t0 * var_nvtm_dn3)) / (var_nvtm * var_nvtm))) * assign7570_e7618) - (assign7570_e7609 * ((assign7570_e7613 * (((var_t0_dn3 * var_nvtm) - (var_t0 * var_nvtm_dn3)) / (var_nvtm * var_nvtm))) + (assign7570_e7617 * (((var_t1_dn3 * var_nvtm) - (var_t1 * var_nvtm_dn3)) / (var_nvtm * var_nvtm)))))) / (assign7570_e7618 * assign7570_e7618));
        var_w1_dn4 = ((((assign7570_e7609 * (((var_t0_dn4 * var_nvtm) - (var_t0 * var_nvtm_dn4)) / (var_nvtm * var_nvtm))) * assign7570_e7618) - (assign7570_e7609 * ((assign7570_e7613 * (((var_t0_dn4 * var_nvtm) - (var_t0 * var_nvtm_dn4)) / (var_nvtm * var_nvtm))) + (assign7570_e7617 * (((var_t1_dn4 * var_nvtm) - (var_t1 * var_nvtm_dn4)) / (var_nvtm * var_nvtm)))))) / (assign7570_e7618 * assign7570_e7618));
        var_w1_dn5 = ((((assign7570_e7609 * (((var_t0_dn5 * var_nvtm) - (var_t0 * var_nvtm_dn5)) / (var_nvtm * var_nvtm))) * assign7570_e7618) - (assign7570_e7609 * ((assign7570_e7613 * (((var_t0_dn5 * var_nvtm) - (var_t0 * var_nvtm_dn5)) / (var_nvtm * var_nvtm))) + (assign7570_e7617 * (((var_t1_dn5 * var_nvtm) - (var_t1 * var_nvtm_dn5)) / (var_nvtm * var_nvtm)))))) / (assign7570_e7618 * assign7570_e7618));
        var_w1_dn6 = ((((assign7570_e7609 * (((var_t0_dn6 * var_nvtm) - (var_t0 * var_nvtm_dn6)) / (var_nvtm * var_nvtm))) * assign7570_e7618) - (assign7570_e7609 * ((assign7570_e7613 * (((var_t0_dn6 * var_nvtm) - (var_t0 * var_nvtm_dn6)) / (var_nvtm * var_nvtm))) + (assign7570_e7617 * (((var_t1_dn6 * var_nvtm) - (var_t1 * var_nvtm_dn6)) / (var_nvtm * var_nvtm)))))) / (assign7570_e7618 * assign7570_e7618));
        var_w1_dn7 = ((((assign7570_e7609 * (((var_t0_dn7 * var_nvtm) - (var_t0 * var_nvtm_dn7)) / (var_nvtm * var_nvtm))) * assign7570_e7618) - (assign7570_e7609 * ((assign7570_e7613 * (((var_t0_dn7 * var_nvtm) - (var_t0 * var_nvtm_dn7)) / (var_nvtm * var_nvtm))) + (assign7570_e7617 * (((var_t1_dn7 * var_nvtm) - (var_t1 * var_nvtm_dn7)) / (var_nvtm * var_nvtm)))))) / (assign7570_e7618 * assign7570_e7618));
        var_w1_dn8 = ((((assign7570_e7609 * (((var_t0_dn8 * var_nvtm) - (var_t0 * var_nvtm_dn8)) / (var_nvtm * var_nvtm))) * assign7570_e7618) - (assign7570_e7609 * ((assign7570_e7613 * (((var_t0_dn8 * var_nvtm) - (var_t0 * var_nvtm_dn8)) / (var_nvtm * var_nvtm))) + (assign7570_e7617 * (((var_t1_dn8 * var_nvtm) - (var_t1 * var_nvtm_dn8)) / (var_nvtm * var_nvtm)))))) / (assign7570_e7618 * assign7570_e7618));

        let assign7580_e7622: f64 = (var_t1 / var_nvtm);
        let assign7580_e7623: f64 = (assign7580_e7622).exp();
        let assign7580_e7626: f64 = (var_t0 / var_nvtm);
        let assign7580_e7627: f64 = (assign7580_e7626).exp();
        let assign7580_e7630: f64 = (var_t1 / var_nvtm);
        let assign7580_e7631: f64 = (assign7580_e7630).exp();
        let assign7580_e7632: f64 = (assign7580_e7627 + assign7580_e7631);
        let assign7580_e7633: f64 = (assign7580_e7623 / assign7580_e7632);
        var_w2 = assign7580_e7633;
        var_w2_dn3 = ((((assign7580_e7623 * (((var_t1_dn3 * var_nvtm) - (var_t1 * var_nvtm_dn3)) / (var_nvtm * var_nvtm))) * assign7580_e7632) - (assign7580_e7623 * ((assign7580_e7627 * (((var_t0_dn3 * var_nvtm) - (var_t0 * var_nvtm_dn3)) / (var_nvtm * var_nvtm))) + (assign7580_e7631 * (((var_t1_dn3 * var_nvtm) - (var_t1 * var_nvtm_dn3)) / (var_nvtm * var_nvtm)))))) / (assign7580_e7632 * assign7580_e7632));
        var_w2_dn4 = ((((assign7580_e7623 * (((var_t1_dn4 * var_nvtm) - (var_t1 * var_nvtm_dn4)) / (var_nvtm * var_nvtm))) * assign7580_e7632) - (assign7580_e7623 * ((assign7580_e7627 * (((var_t0_dn4 * var_nvtm) - (var_t0 * var_nvtm_dn4)) / (var_nvtm * var_nvtm))) + (assign7580_e7631 * (((var_t1_dn4 * var_nvtm) - (var_t1 * var_nvtm_dn4)) / (var_nvtm * var_nvtm)))))) / (assign7580_e7632 * assign7580_e7632));
        var_w2_dn5 = ((((assign7580_e7623 * (((var_t1_dn5 * var_nvtm) - (var_t1 * var_nvtm_dn5)) / (var_nvtm * var_nvtm))) * assign7580_e7632) - (assign7580_e7623 * ((assign7580_e7627 * (((var_t0_dn5 * var_nvtm) - (var_t0 * var_nvtm_dn5)) / (var_nvtm * var_nvtm))) + (assign7580_e7631 * (((var_t1_dn5 * var_nvtm) - (var_t1 * var_nvtm_dn5)) / (var_nvtm * var_nvtm)))))) / (assign7580_e7632 * assign7580_e7632));
        var_w2_dn6 = ((((assign7580_e7623 * (((var_t1_dn6 * var_nvtm) - (var_t1 * var_nvtm_dn6)) / (var_nvtm * var_nvtm))) * assign7580_e7632) - (assign7580_e7623 * ((assign7580_e7627 * (((var_t0_dn6 * var_nvtm) - (var_t0 * var_nvtm_dn6)) / (var_nvtm * var_nvtm))) + (assign7580_e7631 * (((var_t1_dn6 * var_nvtm) - (var_t1 * var_nvtm_dn6)) / (var_nvtm * var_nvtm)))))) / (assign7580_e7632 * assign7580_e7632));
        var_w2_dn7 = ((((assign7580_e7623 * (((var_t1_dn7 * var_nvtm) - (var_t1 * var_nvtm_dn7)) / (var_nvtm * var_nvtm))) * assign7580_e7632) - (assign7580_e7623 * ((assign7580_e7627 * (((var_t0_dn7 * var_nvtm) - (var_t0 * var_nvtm_dn7)) / (var_nvtm * var_nvtm))) + (assign7580_e7631 * (((var_t1_dn7 * var_nvtm) - (var_t1 * var_nvtm_dn7)) / (var_nvtm * var_nvtm)))))) / (assign7580_e7632 * assign7580_e7632));
        var_w2_dn8 = ((((assign7580_e7623 * (((var_t1_dn8 * var_nvtm) - (var_t1 * var_nvtm_dn8)) / (var_nvtm * var_nvtm))) * assign7580_e7632) - (assign7580_e7623 * ((assign7580_e7627 * (((var_t0_dn8 * var_nvtm) - (var_t0 * var_nvtm_dn8)) / (var_nvtm * var_nvtm))) + (assign7580_e7631 * (((var_t1_dn8 * var_nvtm) - (var_t1 * var_nvtm_dn8)) / (var_nvtm * var_nvtm)))))) / (assign7580_e7632 * assign7580_e7632));

        let assign7590_e7636: f64 = (var_w1 * var_ueff1);
        let assign7590_e7639: f64 = (var_w2 * var_ueff2);
        let assign7590_e7640: f64 = (assign7590_e7636 + assign7590_e7639);
        var_utotal = assign7590_e7640;
        var_utotal_dn3 = (((var_w1_dn3 * var_ueff1) + (var_w1 * var_ueff1_dn3)) + ((var_w2_dn3 * var_ueff2) + (var_w2 * var_ueff2_dn3)));
        var_utotal_dn4 = (((var_w1_dn4 * var_ueff1) + (var_w1 * var_ueff1_dn4)) + ((var_w2_dn4 * var_ueff2) + (var_w2 * var_ueff2_dn4)));
        var_utotal_dn5 = (((var_w1_dn5 * var_ueff1) + (var_w1 * var_ueff1_dn5)) + ((var_w2_dn5 * var_ueff2) + (var_w2 * var_ueff2_dn5)));
        var_utotal_dn6 = (((var_w1_dn6 * var_ueff1) + (var_w1 * var_ueff1_dn6)) + ((var_w2_dn6 * var_ueff2) + (var_w2 * var_ueff2_dn6)));
        var_utotal_dn7 = (((var_w1_dn7 * var_ueff1) + (var_w1 * var_ueff1_dn7)) + ((var_w2_dn7 * var_ueff2) + (var_w2 * var_ueff2_dn7)));
        var_utotal_dn8 = (((var_w1_dn8 * var_ueff1) + (var_w1 * var_ueff1_dn8)) + ((var_w2_dn8 * var_ueff2) + (var_w2 * var_ueff2_dn8)));

        let assign7600_e7643: f64 = if p.p14 == 1.0 { 1.0 } else { 0.0 };
        var_guard87 = assign7600_e7643;

        let (assign7610_e7647, assign7610_e7647_d_n3, assign7610_e7647_d_n4, assign7610_e7647_d_n5, assign7610_e7647_d_n6, assign7610_e7647_d_n7, assign7610_e7647_d_n8,) = {
    if (var_guard87 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdss, var_rdss_dn3, var_rdss_dn4, var_rdss_dn5, var_rdss_dn6, var_rdss_dn7, var_rdss_dn8,)
    }
};
        var_rdss = assign7610_e7647;
        var_rdss_dn3 = assign7610_e7647_d_n3;
        var_rdss_dn4 = assign7610_e7647_d_n4;
        var_rdss_dn5 = assign7610_e7647_d_n5;
        var_rdss_dn6 = assign7610_e7647_d_n6;
        var_rdss_dn7 = assign7610_e7647_d_n7;
        var_rdss_dn8 = assign7610_e7647_d_n8;

        let assign7620_e7650: f64 = if p.p14 == 0.0 { 1.0 } else { 0.0 };
        var_guard88 = assign7620_e7650;

        let (assign7630_e7661, assign7630_e7661_d_n3, assign7630_e7661_d_n4, assign7630_e7661_d_n5, assign7630_e7661_d_n6, assign7630_e7661_d_n7, assign7630_e7661_d_n8,) = {
    if ((var_guard87 == 0.0) && (var_guard88 != 0.0)) {
        let assign7630_e7658: f64 = (var_prwg_i * var_qis);
        let assign7630_e7659: f64 = (1.0 + assign7630_e7658);
        (assign7630_e7659, (var_prwg_i * var_qis_dn3), (var_prwg_i * var_qis_dn4), (var_prwg_i * var_qis_dn5), (var_prwg_i * var_qis_dn6), (var_prwg_i * var_qis_dn7), (var_prwg_i * var_qis_dn8),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign7630_e7661;
        var_t4_dn3 = assign7630_e7661_d_n3;
        var_t4_dn4 = assign7630_e7661_d_n4;
        var_t4_dn5 = assign7630_e7661_d_n5;
        var_t4_dn6 = assign7630_e7661_d_n6;
        var_t4_dn7 = assign7630_e7661_d_n7;
        var_t4_dn8 = assign7630_e7661_d_n8;

        let (assign7640_e7670, assign7640_e7670_d_n3, assign7640_e7670_d_n4, assign7640_e7670_d_n5, assign7640_e7670_d_n6, assign7640_e7670_d_n7, assign7640_e7670_d_n8,) = {
    if ((var_guard87 == 0.0) && (var_guard88 != 0.0)) {
        let assign7640_e7668: f64 = (1.0 / var_t4);
        (assign7640_e7668, (-(var_t4_dn3 / (var_t4 * var_t4))), (-(var_t4_dn4 / (var_t4 * var_t4))), (-(var_t4_dn5 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn8 / (var_t4 * var_t4))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign7640_e7670;
        var_t1_dn3 = assign7640_e7670_d_n3;
        var_t1_dn4 = assign7640_e7670_d_n4;
        var_t1_dn5 = assign7640_e7670_d_n5;
        var_t1_dn6 = assign7640_e7670_d_n6;
        var_t1_dn7 = assign7640_e7670_d_n7;
        var_t1_dn8 = assign7640_e7670_d_n8;

        let (assign7650_e7686, assign7650_e7686_d_n3, assign7650_e7686_d_n4, assign7650_e7686_d_n5, assign7650_e7686_d_n6, assign7650_e7686_d_n7, assign7650_e7686_d_n8,) = {
    if ((var_guard87 == 0.0) && (var_guard88 != 0.0)) {
        let assign7650_e7679: f64 = (var_t1 * var_t1);
        let assign7650_e7681: f64 = (assign7650_e7679 + 0.01);
        let assign7650_e7682: f64 = (assign7650_e7681).sqrt();
        let assign7650_e7683: f64 = (var_t1 + assign7650_e7682);
        let assign7650_e7684: f64 = (0.5 * assign7650_e7683);
        (assign7650_e7684, (0.5 * (var_t1_dn3 + (((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)) / (2.0 * assign7650_e7682)))), (0.5 * (var_t1_dn4 + (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign7650_e7682)))), (0.5 * (var_t1_dn5 + (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign7650_e7682)))), (0.5 * (var_t1_dn6 + (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign7650_e7682)))), (0.5 * (var_t1_dn7 + (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign7650_e7682)))), (0.5 * (var_t1_dn8 + (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign7650_e7682)))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign7650_e7686;
        var_t0_dn3 = assign7650_e7686_d_n3;
        var_t0_dn4 = assign7650_e7686_d_n4;
        var_t0_dn5 = assign7650_e7686_d_n5;
        var_t0_dn6 = assign7650_e7686_d_n6;
        var_t0_dn7 = assign7650_e7686_d_n7;
        var_t0_dn8 = assign7650_e7686_d_n8;

        let (assign7660_e7703, assign7660_e7703_d_n3, assign7660_e7703_d_n4, assign7660_e7703_d_n5, assign7660_e7703_d_n6, assign7660_e7703_d_n7, assign7660_e7703_d_n8,) = {
    if ((var_guard87 == 0.0) && (var_guard88 != 0.0)) {
        let assign7660_e7694: f64 = (var_rdsw_i * var_t0);
        let assign7660_e7695: f64 = (var_rdswmin_i + assign7660_e7694);
        let assign7660_e7697: f64 = (assign7660_e7695 * var_weffwrfactor);
        let assign7660_e7699: f64 = (assign7660_e7697 * p.p2);
        let assign7660_e7701: f64 = (assign7660_e7699 * var_rdstemp);
        (assign7660_e7701, ((((var_rdsw_i * var_t0_dn3) * var_weffwrfactor) * p.p2) * var_rdstemp), (((((var_rdsw_i * var_t0_dn4) * var_weffwrfactor) * p.p2) * var_rdstemp) + (assign7660_e7699 * var_rdstemp_dn4)), ((((var_rdsw_i * var_t0_dn5) * var_weffwrfactor) * p.p2) * var_rdstemp), ((((var_rdsw_i * var_t0_dn6) * var_weffwrfactor) * p.p2) * var_rdstemp), ((((var_rdsw_i * var_t0_dn7) * var_weffwrfactor) * p.p2) * var_rdstemp), ((((var_rdsw_i * var_t0_dn8) * var_weffwrfactor) * p.p2) * var_rdstemp),)
    } else {
        (var_rdss, var_rdss_dn3, var_rdss_dn4, var_rdss_dn5, var_rdss_dn6, var_rdss_dn7, var_rdss_dn8,)
    }
};
        var_rdss = assign7660_e7703;
        var_rdss_dn3 = assign7660_e7703_d_n3;
        var_rdss_dn4 = assign7660_e7703_d_n4;
        var_rdss_dn5 = assign7660_e7703_d_n5;
        var_rdss_dn6 = assign7660_e7703_d_n6;
        var_rdss_dn7 = assign7660_e7703_d_n7;
        var_rdss_dn8 = assign7660_e7703_d_n8;

        let (assign7670_e7715, assign7670_e7715_d_n3, assign7670_e7715_d_n4, assign7670_e7715_d_n5, assign7670_e7715_d_n6, assign7670_e7715_d_n7, assign7670_e7715_d_n8,) = {
    if ((var_guard87 == 0.0) && (var_guard88 == 0.0)) {
        let assign7670_e7712: f64 = (var_prwg_i * var_qis);
        let assign7670_e7713: f64 = (1.0 + assign7670_e7712);
        (assign7670_e7713, (var_prwg_i * var_qis_dn3), (var_prwg_i * var_qis_dn4), (var_prwg_i * var_qis_dn5), (var_prwg_i * var_qis_dn6), (var_prwg_i * var_qis_dn7), (var_prwg_i * var_qis_dn8),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign7670_e7715;
        var_t4_dn3 = assign7670_e7715_d_n3;
        var_t4_dn4 = assign7670_e7715_d_n4;
        var_t4_dn5 = assign7670_e7715_d_n5;
        var_t4_dn6 = assign7670_e7715_d_n6;
        var_t4_dn7 = assign7670_e7715_d_n7;
        var_t4_dn8 = assign7670_e7715_d_n8;

        *var_dmobs_slot = var_dmobs;
        *var_dmobs_dn3_slot = var_dmobs_dn3;
        *var_dmobs_dn4_slot = var_dmobs_dn4;
        *var_dmobs_dn5_slot = var_dmobs_dn5;
        *var_dmobs_dn6_slot = var_dmobs_dn6;
        *var_dmobs_dn7_slot = var_dmobs_dn7;
        *var_dmobs_dn8_slot = var_dmobs_dn8;
        *var_eeffs_slot = var_eeffs;
        *var_eeffs2_slot = var_eeffs2;
        *var_eeffs2_dn3_slot = var_eeffs2_dn3;
        *var_eeffs2_dn4_slot = var_eeffs2_dn4;
        *var_eeffs2_dn5_slot = var_eeffs2_dn5;
        *var_eeffs2_dn6_slot = var_eeffs2_dn6;
        *var_eeffs2_dn7_slot = var_eeffs2_dn7;
        *var_eeffs2_dn8_slot = var_eeffs2_dn8;
        *var_eeffs_dn3_slot = var_eeffs_dn3;
        *var_eeffs_dn4_slot = var_eeffs_dn4;
        *var_eeffs_dn5_slot = var_eeffs_dn5;
        *var_eeffs_dn6_slot = var_eeffs_dn6;
        *var_eeffs_dn7_slot = var_eeffs_dn7;
        *var_eeffs_dn8_slot = var_eeffs_dn8;
        *var_guard87_slot = var_guard87;
        *var_guard88_slot = var_guard88;
        *var_phi2_slot = var_phi2;
        *var_phi2_dn3_slot = var_phi2_dn3;
        *var_phi2_dn4_slot = var_phi2_dn4;
        *var_phi2_dn5_slot = var_phi2_dn5;
        *var_phi2_dn6_slot = var_phi2_dn6;
        *var_phi2_dn7_slot = var_phi2_dn7;
        *var_phi2_dn8_slot = var_phi2_dn8;
        *var_phifs_slot = var_phifs;
        *var_phifs_dn3_slot = var_phifs_dn3;
        *var_phifs_dn4_slot = var_phifs_dn4;
        *var_phifs_dn5_slot = var_phifs_dn5;
        *var_phifs_dn6_slot = var_phifs_dn6;
        *var_phifs_dn7_slot = var_phifs_dn7;
        *var_phifs_dn8_slot = var_phifs_dn8;
        *var_qb0_slot = var_qb0;
        *var_qbacks_slot = var_qbacks;
        *var_qbacks_dn3_slot = var_qbacks_dn3;
        *var_qbacks_dn4_slot = var_qbacks_dn4;
        *var_qbacks_dn5_slot = var_qbacks_dn5;
        *var_qbacks_dn6_slot = var_qbacks_dn6;
        *var_qbacks_dn7_slot = var_qbacks_dn7;
        *var_qbacks_dn8_slot = var_qbacks_dn8;
        *var_qbs_slot = var_qbs;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qfronts_slot = var_qfronts;
        *var_qfronts_dn3_slot = var_qfronts_dn3;
        *var_qfronts_dn4_slot = var_qfronts_dn4;
        *var_qfronts_dn5_slot = var_qfronts_dn5;
        *var_qfronts_dn6_slot = var_qfronts_dn6;
        *var_qfronts_dn7_slot = var_qfronts_dn7;
        *var_qfronts_dn8_slot = var_qfronts_dn8;
        *var_qicores_slot = var_qicores;
        *var_qicores_dn3_slot = var_qicores_dn3;
        *var_qicores_dn4_slot = var_qicores_dn4;
        *var_qicores_dn5_slot = var_qicores_dn5;
        *var_qicores_dn6_slot = var_qicores_dn6;
        *var_qicores_dn7_slot = var_qicores_dn7;
        *var_qicores_dn8_slot = var_qicores_dn8;
        *var_qis_slot = var_qis;
        *var_qis_dn3_slot = var_qis_dn3;
        *var_qis_dn4_slot = var_qis_dn4;
        *var_qis_dn5_slot = var_qis_dn5;
        *var_qis_dn6_slot = var_qis_dn6;
        *var_qis_dn7_slot = var_qis_dn7;
        *var_qis_dn8_slot = var_qis_dn8;
        *var_qtots_slot = var_qtots;
        *var_qtots_dn3_slot = var_qtots_dn3;
        *var_qtots_dn4_slot = var_qtots_dn4;
        *var_qtots_dn5_slot = var_qtots_dn5;
        *var_qtots_dn6_slot = var_qtots_dn6;
        *var_qtots_dn7_slot = var_qtots_dn7;
        *var_qtots_dn8_slot = var_qtots_dn8;
        *var_rdss_slot = var_rdss;
        *var_rdss_dn3_slot = var_rdss_dn3;
        *var_rdss_dn4_slot = var_rdss_dn4;
        *var_rdss_dn5_slot = var_rdss_dn5;
        *var_rdss_dn6_slot = var_rdss_dn6;
        *var_rdss_dn7_slot = var_rdss_dn7;
        *var_rdss_dn8_slot = var_rdss_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2__blk83_slot = var_t2__blk83;
        *var_t2__blk83_dn3_slot = var_t2__blk83_dn3;
        *var_t2__blk83_dn4_slot = var_t2__blk83_dn4;
        *var_t2__blk83_dn5_slot = var_t2__blk83_dn5;
        *var_t2__blk83_dn6_slot = var_t2__blk83_dn6;
        *var_t2__blk83_dn7_slot = var_t2__blk83_dn7;
        *var_t2__blk83_dn8_slot = var_t2__blk83_dn8;
        *var_t2__blk85_slot = var_t2__blk85;
        *var_t2__blk85_dn3_slot = var_t2__blk85_dn3;
        *var_t2__blk85_dn4_slot = var_t2__blk85_dn4;
        *var_t2__blk85_dn5_slot = var_t2__blk85_dn5;
        *var_t2__blk85_dn6_slot = var_t2__blk85_dn6;
        *var_t2__blk85_dn7_slot = var_t2__blk85_dn7;
        *var_t2__blk85_dn8_slot = var_t2__blk85_dn8;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3__blk84_slot = var_t3__blk84;
        *var_t3__blk84_dn3_slot = var_t3__blk84_dn3;
        *var_t3__blk84_dn4_slot = var_t3__blk84_dn4;
        *var_t3__blk84_dn5_slot = var_t3__blk84_dn5;
        *var_t3__blk84_dn6_slot = var_t3__blk84_dn6;
        *var_t3__blk84_dn7_slot = var_t3__blk84_dn7;
        *var_t3__blk84_dn8_slot = var_t3__blk84_dn8;
        *var_t3__blk86_slot = var_t3__blk86;
        *var_t3__blk86_dn3_slot = var_t3__blk86_dn3;
        *var_t3__blk86_dn4_slot = var_t3__blk86_dn4;
        *var_t3__blk86_dn5_slot = var_t3__blk86_dn5;
        *var_t3__blk86_dn6_slot = var_t3__blk86_dn6;
        *var_t3__blk86_dn7_slot = var_t3__blk86_dn7;
        *var_t3__blk86_dn8_slot = var_t3__blk86_dn8;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_ueff1_slot = var_ueff1;
        *var_ueff1_dn3_slot = var_ueff1_dn3;
        *var_ueff1_dn4_slot = var_ueff1_dn4;
        *var_ueff1_dn5_slot = var_ueff1_dn5;
        *var_ueff1_dn6_slot = var_ueff1_dn6;
        *var_ueff1_dn7_slot = var_ueff1_dn7;
        *var_ueff1_dn8_slot = var_ueff1_dn8;
        *var_ueff2_slot = var_ueff2;
        *var_ueff2_dn3_slot = var_ueff2_dn3;
        *var_ueff2_dn4_slot = var_ueff2_dn4;
        *var_ueff2_dn5_slot = var_ueff2_dn5;
        *var_ueff2_dn6_slot = var_ueff2_dn6;
        *var_ueff2_dn7_slot = var_ueff2_dn7;
        *var_ueff2_dn8_slot = var_ueff2_dn8;
        *var_utotal_slot = var_utotal;
        *var_utotal_dn3_slot = var_utotal_dn3;
        *var_utotal_dn4_slot = var_utotal_dn4;
        *var_utotal_dn5_slot = var_utotal_dn5;
        *var_utotal_dn6_slot = var_utotal_dn6;
        *var_utotal_dn7_slot = var_utotal_dn7;
        *var_utotal_dn8_slot = var_utotal_dn8;
        *var_w1_slot = var_w1;
        *var_w1_dn3_slot = var_w1_dn3;
        *var_w1_dn4_slot = var_w1_dn4;
        *var_w1_dn5_slot = var_w1_dn5;
        *var_w1_dn6_slot = var_w1_dn6;
        *var_w1_dn7_slot = var_w1_dn7;
        *var_w1_dn8_slot = var_w1_dn8;
        *var_w2_slot = var_w2;
        *var_w2_dn3_slot = var_w2_dn3;
        *var_w2_dn4_slot = var_w2_dn4;
        *var_w2_dn5_slot = var_w2_dn5;
        *var_w2_dn6_slot = var_w2_dn6;
        *var_w2_dn7_slot = var_w2_dn7;
        *var_w2_dn8_slot = var_w2_dn8;
    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_cox1: f64,
        var_dvth_all: f64,
        var_dvth_all_dn3: f64,
        var_dvth_all_dn4: f64,
        var_dvth_all_dn5: f64,
        var_dvth_all_dn6: f64,
        var_dvth_all_dn7: f64,
        var_dvth_all_dn8: f64,
        var_guard87: f64,
        var_guard88: f64,
        var_inv_mexp: f64,
        var_k1: f64,
        var_k1_2: f64,
        var_k2: f64,
        var_keq_k2: f64,
        var_ksativ_i: f64,
        var_ksativb_i: f64,
        var_ksubiv_i: f64,
        var_leff: f64,
        var_lna0: f64,
        var_lna0_dn3: f64,
        var_lna0_dn4: f64,
        var_lna0_dn5: f64,
        var_lna0_dn6: f64,
        var_lna0_dn7: f64,
        var_lna0_dn8: f64,
        var_mexp_t: f64,
        var_mexp_t_dn4: f64,
        var_nvtm: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_phi1_0: f64,
        var_phi1_0_dn3: f64,
        var_phi1_0_dn4: f64,
        var_phi1_0_dn5: f64,
        var_phi1_0_dn6: f64,
        var_phi1_0_dn7: f64,
        var_phi1_0_dn8: f64,
        var_qis: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_rdraingeo: f64,
        var_rdstemp: f64,
        var_rdstemp_dn4: f64,
        var_rdsw_i: f64,
        var_rdswmin_i: f64,
        var_rsourcegeo: f64,
        var_utotal: f64,
        var_utotal_dn3: f64,
        var_utotal_dn4: f64,
        var_utotal_dn5: f64,
        var_utotal_dn6: f64,
        var_utotal_dn7: f64,
        var_utotal_dn8: f64,
        var_vbgxpos: f64,
        var_vbgxpos_dn3: f64,
        var_vbgxpos_dn5: f64,
        var_vbgxpos_dn6: f64,
        var_vds: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vgfb1eff: f64,
        var_vgfb1eff_dn3: f64,
        var_vgfb1eff_dn4: f64,
        var_vgfb1eff_dn5: f64,
        var_vgfb1eff_dn6: f64,
        var_vgfb1eff_dn7: f64,
        var_vgfb1eff_dn8: f64,
        var_vgfb2: f64,
        var_vgfb2_dn3: f64,
        var_vgfb2_dn4: f64,
        var_vgfb2_dn5: f64,
        var_vgfb2_dn6: f64,
        var_vgfb2_dn7: f64,
        var_vgfb2_dn8: f64,
        var_vsat_t: f64,
        var_vsat_t_dn4: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_weff: f64,
        var_weffwrfactor: f64,
        var_esat_slot: &mut f64,
        var_esat_dn3_slot: &mut f64,
        var_esat_dn4_slot: &mut f64,
        var_esat_dn5_slot: &mut f64,
        var_esat_dn6_slot: &mut f64,
        var_esat_dn7_slot: &mut f64,
        var_esat_dn8_slot: &mut f64,
        var_esatl_slot: &mut f64,
        var_esatl_dn3_slot: &mut f64,
        var_esatl_dn4_slot: &mut f64,
        var_esatl_dn5_slot: &mut f64,
        var_esatl_dn6_slot: &mut f64,
        var_esatl_dn7_slot: &mut f64,
        var_esatl_dn8_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_phi2_slot: &mut f64,
        var_phi2_dn3_slot: &mut f64,
        var_phi2_dn4_slot: &mut f64,
        var_phi2_dn5_slot: &mut f64,
        var_phi2_dn6_slot: &mut f64,
        var_phi2_dn7_slot: &mut f64,
        var_phi2_dn8_slot: &mut f64,
        var_phi2sub_slot: &mut f64,
        var_phi2sub_dn3_slot: &mut f64,
        var_phi2sub_dn4_slot: &mut f64,
        var_phi2sub_dn5_slot: &mut f64,
        var_phi2sub_dn6_slot: &mut f64,
        var_phi2sub_dn7_slot: &mut f64,
        var_phi2sub_dn8_slot: &mut f64,
        var_phissat_slot: &mut f64,
        var_phissat_dn3_slot: &mut f64,
        var_phissat_dn4_slot: &mut f64,
        var_phissat_dn5_slot: &mut f64,
        var_phissat_dn6_slot: &mut f64,
        var_phissat_dn7_slot: &mut f64,
        var_phissat_dn8_slot: &mut f64,
        var_phissatback_slot: &mut f64,
        var_phissatback2_slot: &mut f64,
        var_phissatback2_dn3_slot: &mut f64,
        var_phissatback2_dn4_slot: &mut f64,
        var_phissatback2_dn5_slot: &mut f64,
        var_phissatback2_dn6_slot: &mut f64,
        var_phissatback2_dn7_slot: &mut f64,
        var_phissatback2_dn8_slot: &mut f64,
        var_phissatback_dn3_slot: &mut f64,
        var_phissatback_dn4_slot: &mut f64,
        var_phissatback_dn5_slot: &mut f64,
        var_phissatback_dn6_slot: &mut f64,
        var_phissatback_dn7_slot: &mut f64,
        var_phissatback_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_rdss_slot: &mut f64,
        var_rdss_dn3_slot: &mut f64,
        var_rdss_dn4_slot: &mut f64,
        var_rdss_dn5_slot: &mut f64,
        var_rdss_dn6_slot: &mut f64,
        var_rdss_dn7_slot: &mut f64,
        var_rdss_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_ta_slot: &mut f64,
        var_ta_dn3_slot: &mut f64,
        var_ta_dn4_slot: &mut f64,
        var_ta_dn5_slot: &mut f64,
        var_ta_dn6_slot: &mut f64,
        var_ta_dn7_slot: &mut f64,
        var_ta_dn8_slot: &mut f64,
        var_tb_slot: &mut f64,
        var_tb_dn3_slot: &mut f64,
        var_tb_dn4_slot: &mut f64,
        var_tb_dn5_slot: &mut f64,
        var_tb_dn6_slot: &mut f64,
        var_tb_dn7_slot: &mut f64,
        var_tb_dn8_slot: &mut f64,
        var_tc_slot: &mut f64,
        var_tc_dn3_slot: &mut f64,
        var_tc_dn4_slot: &mut f64,
        var_tc_dn5_slot: &mut f64,
        var_tc_dn6_slot: &mut f64,
        var_tc_dn7_slot: &mut f64,
        var_tc_dn8_slot: &mut f64,
        var_vdsat_slot: &mut f64,
        var_vdsat_dn3_slot: &mut f64,
        var_vdsat_dn4_slot: &mut f64,
        var_vdsat_dn5_slot: &mut f64,
        var_vdsat_dn6_slot: &mut f64,
        var_vdsat_dn7_slot: &mut f64,
        var_vdsat_dn8_slot: &mut f64,
        var_vdseff_slot: &mut f64,
        var_vdseff_dn3_slot: &mut f64,
        var_vdseff_dn4_slot: &mut f64,
        var_vdseff_dn5_slot: &mut f64,
        var_vdseff_dn6_slot: &mut f64,
        var_vdseff_dn7_slot: &mut f64,
        var_vdseff_dn8_slot: &mut f64,
        var_wvcox_slot: &mut f64,
        var_wvcox_dn4_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_dn3_slot: &mut f64,
        var_xg1_dn4_slot: &mut f64,
        var_xg1_dn5_slot: &mut f64,
        var_xg1_dn6_slot: &mut f64,
        var_xg1_dn7_slot: &mut f64,
        var_xg1_dn8_slot: &mut f64,
        var_xg2_slot: &mut f64,
        var_xg2_dn3_slot: &mut f64,
        var_xg2_dn4_slot: &mut f64,
        var_xg2_dn5_slot: &mut f64,
        var_xg2_dn6_slot: &mut f64,
        var_xg2_dn7_slot: &mut f64,
        var_xg2_dn8_slot: &mut f64,
    ) {
        let mut var_esat: f64 = *var_esat_slot;
        let mut var_esat_dn3: f64 = *var_esat_dn3_slot;
        let mut var_esat_dn4: f64 = *var_esat_dn4_slot;
        let mut var_esat_dn5: f64 = *var_esat_dn5_slot;
        let mut var_esat_dn6: f64 = *var_esat_dn6_slot;
        let mut var_esat_dn7: f64 = *var_esat_dn7_slot;
        let mut var_esat_dn8: f64 = *var_esat_dn8_slot;
        let mut var_esatl: f64 = *var_esatl_slot;
        let mut var_esatl_dn3: f64 = *var_esatl_dn3_slot;
        let mut var_esatl_dn4: f64 = *var_esatl_dn4_slot;
        let mut var_esatl_dn5: f64 = *var_esatl_dn5_slot;
        let mut var_esatl_dn6: f64 = *var_esatl_dn6_slot;
        let mut var_esatl_dn7: f64 = *var_esatl_dn7_slot;
        let mut var_esatl_dn8: f64 = *var_esatl_dn8_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_phi2: f64 = *var_phi2_slot;
        let mut var_phi2_dn3: f64 = *var_phi2_dn3_slot;
        let mut var_phi2_dn4: f64 = *var_phi2_dn4_slot;
        let mut var_phi2_dn5: f64 = *var_phi2_dn5_slot;
        let mut var_phi2_dn6: f64 = *var_phi2_dn6_slot;
        let mut var_phi2_dn7: f64 = *var_phi2_dn7_slot;
        let mut var_phi2_dn8: f64 = *var_phi2_dn8_slot;
        let mut var_phi2sub: f64 = *var_phi2sub_slot;
        let mut var_phi2sub_dn3: f64 = *var_phi2sub_dn3_slot;
        let mut var_phi2sub_dn4: f64 = *var_phi2sub_dn4_slot;
        let mut var_phi2sub_dn5: f64 = *var_phi2sub_dn5_slot;
        let mut var_phi2sub_dn6: f64 = *var_phi2sub_dn6_slot;
        let mut var_phi2sub_dn7: f64 = *var_phi2sub_dn7_slot;
        let mut var_phi2sub_dn8: f64 = *var_phi2sub_dn8_slot;
        let mut var_phissat: f64 = *var_phissat_slot;
        let mut var_phissat_dn3: f64 = *var_phissat_dn3_slot;
        let mut var_phissat_dn4: f64 = *var_phissat_dn4_slot;
        let mut var_phissat_dn5: f64 = *var_phissat_dn5_slot;
        let mut var_phissat_dn6: f64 = *var_phissat_dn6_slot;
        let mut var_phissat_dn7: f64 = *var_phissat_dn7_slot;
        let mut var_phissat_dn8: f64 = *var_phissat_dn8_slot;
        let mut var_phissatback: f64 = *var_phissatback_slot;
        let mut var_phissatback2: f64 = *var_phissatback2_slot;
        let mut var_phissatback2_dn3: f64 = *var_phissatback2_dn3_slot;
        let mut var_phissatback2_dn4: f64 = *var_phissatback2_dn4_slot;
        let mut var_phissatback2_dn5: f64 = *var_phissatback2_dn5_slot;
        let mut var_phissatback2_dn6: f64 = *var_phissatback2_dn6_slot;
        let mut var_phissatback2_dn7: f64 = *var_phissatback2_dn7_slot;
        let mut var_phissatback2_dn8: f64 = *var_phissatback2_dn8_slot;
        let mut var_phissatback_dn3: f64 = *var_phissatback_dn3_slot;
        let mut var_phissatback_dn4: f64 = *var_phissatback_dn4_slot;
        let mut var_phissatback_dn5: f64 = *var_phissatback_dn5_slot;
        let mut var_phissatback_dn6: f64 = *var_phissatback_dn6_slot;
        let mut var_phissatback_dn7: f64 = *var_phissatback_dn7_slot;
        let mut var_phissatback_dn8: f64 = *var_phissatback_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_rdss: f64 = *var_rdss_slot;
        let mut var_rdss_dn3: f64 = *var_rdss_dn3_slot;
        let mut var_rdss_dn4: f64 = *var_rdss_dn4_slot;
        let mut var_rdss_dn5: f64 = *var_rdss_dn5_slot;
        let mut var_rdss_dn6: f64 = *var_rdss_dn6_slot;
        let mut var_rdss_dn7: f64 = *var_rdss_dn7_slot;
        let mut var_rdss_dn8: f64 = *var_rdss_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_ta: f64 = *var_ta_slot;
        let mut var_ta_dn3: f64 = *var_ta_dn3_slot;
        let mut var_ta_dn4: f64 = *var_ta_dn4_slot;
        let mut var_ta_dn5: f64 = *var_ta_dn5_slot;
        let mut var_ta_dn6: f64 = *var_ta_dn6_slot;
        let mut var_ta_dn7: f64 = *var_ta_dn7_slot;
        let mut var_ta_dn8: f64 = *var_ta_dn8_slot;
        let mut var_tb: f64 = *var_tb_slot;
        let mut var_tb_dn3: f64 = *var_tb_dn3_slot;
        let mut var_tb_dn4: f64 = *var_tb_dn4_slot;
        let mut var_tb_dn5: f64 = *var_tb_dn5_slot;
        let mut var_tb_dn6: f64 = *var_tb_dn6_slot;
        let mut var_tb_dn7: f64 = *var_tb_dn7_slot;
        let mut var_tb_dn8: f64 = *var_tb_dn8_slot;
        let mut var_tc: f64 = *var_tc_slot;
        let mut var_tc_dn3: f64 = *var_tc_dn3_slot;
        let mut var_tc_dn4: f64 = *var_tc_dn4_slot;
        let mut var_tc_dn5: f64 = *var_tc_dn5_slot;
        let mut var_tc_dn6: f64 = *var_tc_dn6_slot;
        let mut var_tc_dn7: f64 = *var_tc_dn7_slot;
        let mut var_tc_dn8: f64 = *var_tc_dn8_slot;
        let mut var_vdsat: f64 = *var_vdsat_slot;
        let mut var_vdsat_dn3: f64 = *var_vdsat_dn3_slot;
        let mut var_vdsat_dn4: f64 = *var_vdsat_dn4_slot;
        let mut var_vdsat_dn5: f64 = *var_vdsat_dn5_slot;
        let mut var_vdsat_dn6: f64 = *var_vdsat_dn6_slot;
        let mut var_vdsat_dn7: f64 = *var_vdsat_dn7_slot;
        let mut var_vdsat_dn8: f64 = *var_vdsat_dn8_slot;
        let mut var_vdseff: f64 = *var_vdseff_slot;
        let mut var_vdseff_dn3: f64 = *var_vdseff_dn3_slot;
        let mut var_vdseff_dn4: f64 = *var_vdseff_dn4_slot;
        let mut var_vdseff_dn5: f64 = *var_vdseff_dn5_slot;
        let mut var_vdseff_dn6: f64 = *var_vdseff_dn6_slot;
        let mut var_vdseff_dn7: f64 = *var_vdseff_dn7_slot;
        let mut var_vdseff_dn8: f64 = *var_vdseff_dn8_slot;
        let mut var_wvcox: f64 = *var_wvcox_slot;
        let mut var_wvcox_dn4: f64 = *var_wvcox_dn4_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_dn3: f64 = *var_xg1_dn3_slot;
        let mut var_xg1_dn4: f64 = *var_xg1_dn4_slot;
        let mut var_xg1_dn5: f64 = *var_xg1_dn5_slot;
        let mut var_xg1_dn6: f64 = *var_xg1_dn6_slot;
        let mut var_xg1_dn7: f64 = *var_xg1_dn7_slot;
        let mut var_xg1_dn8: f64 = *var_xg1_dn8_slot;
        let mut var_xg2: f64 = *var_xg2_slot;
        let mut var_xg2_dn3: f64 = *var_xg2_dn3_slot;
        let mut var_xg2_dn4: f64 = *var_xg2_dn4_slot;
        let mut var_xg2_dn5: f64 = *var_xg2_dn5_slot;
        let mut var_xg2_dn6: f64 = *var_xg2_dn6_slot;
        let mut var_xg2_dn7: f64 = *var_xg2_dn7_slot;
        let mut var_xg2_dn8: f64 = *var_xg2_dn8_slot;

        let (assign7680_e7725, assign7680_e7725_d_n3, assign7680_e7725_d_n4, assign7680_e7725_d_n5, assign7680_e7725_d_n6, assign7680_e7725_d_n7, assign7680_e7725_d_n8,) = {
    if ((var_guard87 == 0.0) && (var_guard88 == 0.0)) {
        let assign7680_e7723: f64 = (1.0 / var_t4);
        (assign7680_e7723, (-(var_t4_dn3 / (var_t4 * var_t4))), (-(var_t4_dn4 / (var_t4 * var_t4))), (-(var_t4_dn5 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn8 / (var_t4 * var_t4))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign7680_e7725;
        var_t1_dn3 = assign7680_e7725_d_n3;
        var_t1_dn4 = assign7680_e7725_d_n4;
        var_t1_dn5 = assign7680_e7725_d_n5;
        var_t1_dn6 = assign7680_e7725_d_n6;
        var_t1_dn7 = assign7680_e7725_d_n7;
        var_t1_dn8 = assign7680_e7725_d_n8;

        let (assign7690_e7742, assign7690_e7742_d_n3, assign7690_e7742_d_n4, assign7690_e7742_d_n5, assign7690_e7742_d_n6, assign7690_e7742_d_n7, assign7690_e7742_d_n8,) = {
    if ((var_guard87 == 0.0) && (var_guard88 == 0.0)) {
        let assign7690_e7735: f64 = (var_t1 * var_t1);
        let assign7690_e7737: f64 = (assign7690_e7735 + 0.01);
        let assign7690_e7738: f64 = (assign7690_e7737).sqrt();
        let assign7690_e7739: f64 = (var_t1 + assign7690_e7738);
        let assign7690_e7740: f64 = (0.5 * assign7690_e7739);
        (assign7690_e7740, (0.5 * (var_t1_dn3 + (((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)) / (2.0 * assign7690_e7738)))), (0.5 * (var_t1_dn4 + (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign7690_e7738)))), (0.5 * (var_t1_dn5 + (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign7690_e7738)))), (0.5 * (var_t1_dn6 + (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign7690_e7738)))), (0.5 * (var_t1_dn7 + (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign7690_e7738)))), (0.5 * (var_t1_dn8 + (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign7690_e7738)))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign7690_e7742;
        var_t0_dn3 = assign7690_e7742_d_n3;
        var_t0_dn4 = assign7690_e7742_d_n4;
        var_t0_dn5 = assign7690_e7742_d_n5;
        var_t0_dn6 = assign7690_e7742_d_n6;
        var_t0_dn7 = assign7690_e7742_d_n7;
        var_t0_dn8 = assign7690_e7742_d_n8;

        let (assign7700_e7764, assign7700_e7764_d_n3, assign7700_e7764_d_n4, assign7700_e7764_d_n5, assign7700_e7764_d_n6, assign7700_e7764_d_n7, assign7700_e7764_d_n8,) = {
    if ((var_guard87 == 0.0) && (var_guard88 == 0.0)) {
        let assign7700_e7750: f64 = (var_rsourcegeo + var_rdraingeo);
        let assign7700_e7752: f64 = (assign7700_e7750 + var_rdswmin_i);
        let assign7700_e7755: f64 = (var_rdsw_i * var_t0);
        let assign7700_e7756: f64 = (assign7700_e7752 + assign7700_e7755);
        let assign7700_e7758: f64 = (assign7700_e7756 * var_weffwrfactor);
        let assign7700_e7760: f64 = (assign7700_e7758 * p.p2);
        let assign7700_e7762: f64 = (assign7700_e7760 * var_rdstemp);
        (assign7700_e7762, ((((var_rdsw_i * var_t0_dn3) * var_weffwrfactor) * p.p2) * var_rdstemp), (((((var_rdsw_i * var_t0_dn4) * var_weffwrfactor) * p.p2) * var_rdstemp) + (assign7700_e7760 * var_rdstemp_dn4)), ((((var_rdsw_i * var_t0_dn5) * var_weffwrfactor) * p.p2) * var_rdstemp), ((((var_rdsw_i * var_t0_dn6) * var_weffwrfactor) * p.p2) * var_rdstemp), ((((var_rdsw_i * var_t0_dn7) * var_weffwrfactor) * p.p2) * var_rdstemp), ((((var_rdsw_i * var_t0_dn8) * var_weffwrfactor) * p.p2) * var_rdstemp),)
    } else {
        (var_rdss, var_rdss_dn3, var_rdss_dn4, var_rdss_dn5, var_rdss_dn6, var_rdss_dn7, var_rdss_dn8,)
    }
};
        var_rdss = assign7700_e7764;
        var_rdss_dn3 = assign7700_e7764_d_n3;
        var_rdss_dn4 = assign7700_e7764_d_n4;
        var_rdss_dn5 = assign7700_e7764_d_n5;
        var_rdss_dn6 = assign7700_e7764_d_n6;
        var_rdss_dn7 = assign7700_e7764_d_n7;
        var_rdss_dn8 = assign7700_e7764_d_n8;

        let assign7710_e7767: f64 = (2.0 * var_vsat_t);
        let assign7710_e7769: f64 = (assign7710_e7767 / var_utotal);
        var_esat = assign7710_e7769;
        var_esat_dn3 = (-((assign7710_e7767 * var_utotal_dn3) / (var_utotal * var_utotal)));
        var_esat_dn4 = ((((2.0 * var_vsat_t_dn4) * var_utotal) - (assign7710_e7767 * var_utotal_dn4)) / (var_utotal * var_utotal));
        var_esat_dn5 = (-((assign7710_e7767 * var_utotal_dn5) / (var_utotal * var_utotal)));
        var_esat_dn6 = (-((assign7710_e7767 * var_utotal_dn6) / (var_utotal * var_utotal)));
        var_esat_dn7 = (-((assign7710_e7767 * var_utotal_dn7) / (var_utotal * var_utotal)));
        var_esat_dn8 = (-((assign7710_e7767 * var_utotal_dn8) / (var_utotal * var_utotal)));

        let assign7720_e7772: f64 = (var_esat * var_leff);
        var_esatl = assign7720_e7772;
        var_esatl_dn3 = (var_esat_dn3 * var_leff);
        var_esatl_dn4 = (var_esat_dn4 * var_leff);
        var_esatl_dn5 = (var_esat_dn5 * var_leff);
        var_esatl_dn6 = (var_esat_dn6 * var_leff);
        var_esatl_dn7 = (var_esat_dn7 * var_leff);
        var_esatl_dn8 = (var_esat_dn8 * var_leff);

        let assign7730_e7777: f64 = (var_ksativb_i * var_vbgxpos);
        let assign7730_e7778: f64 = (var_qis + assign7730_e7777);
        let assign7730_e7781: f64 = (2.0 * var_vtm);
        let assign7730_e7783: f64 = (assign7730_e7781 * var_ksubiv_i);
        let assign7730_e7784: f64 = (assign7730_e7778 + assign7730_e7783);
        let assign7730_e7785: f64 = (var_ksativ_i * assign7730_e7784);
        var_t6 = assign7730_e7785;
        var_t6_dn3 = (var_ksativ_i * (var_qis_dn3 + (var_ksativb_i * var_vbgxpos_dn3)));
        var_t6_dn4 = (var_ksativ_i * (var_qis_dn4 + ((2.0 * var_vtm_dn4) * var_ksubiv_i)));
        var_t6_dn5 = (var_ksativ_i * (var_qis_dn5 + (var_ksativb_i * var_vbgxpos_dn5)));
        var_t6_dn6 = (var_ksativ_i * (var_qis_dn6 + (var_ksativb_i * var_vbgxpos_dn6)));
        var_t6_dn7 = (var_ksativ_i * var_qis_dn7);
        var_t6_dn8 = (var_ksativ_i * var_qis_dn8);

        let assign7740_e7788: f64 = if var_rdss == 0.0 { 1.0 } else { 0.0 };
        var_guard89 = assign7740_e7788;

        let (assign7750_e7798, assign7750_e7798_d_n3, assign7750_e7798_d_n4, assign7750_e7798_d_n5, assign7750_e7798_d_n6, assign7750_e7798_d_n7, assign7750_e7798_d_n8,) = {
    if (var_guard89 != 0.0) {
        let assign7750_e7792: f64 = (var_esatl * var_t6);
        let assign7750_e7795: f64 = (var_esatl + var_t6);
        let assign7750_e7796: f64 = (assign7750_e7792 / assign7750_e7795);
        (assign7750_e7796, (((((var_esatl_dn3 * var_t6) + (var_esatl * var_t6_dn3)) * assign7750_e7795) - (assign7750_e7792 * (var_esatl_dn3 + var_t6_dn3))) / (assign7750_e7795 * assign7750_e7795)), (((((var_esatl_dn4 * var_t6) + (var_esatl * var_t6_dn4)) * assign7750_e7795) - (assign7750_e7792 * (var_esatl_dn4 + var_t6_dn4))) / (assign7750_e7795 * assign7750_e7795)), (((((var_esatl_dn5 * var_t6) + (var_esatl * var_t6_dn5)) * assign7750_e7795) - (assign7750_e7792 * (var_esatl_dn5 + var_t6_dn5))) / (assign7750_e7795 * assign7750_e7795)), (((((var_esatl_dn6 * var_t6) + (var_esatl * var_t6_dn6)) * assign7750_e7795) - (assign7750_e7792 * (var_esatl_dn6 + var_t6_dn6))) / (assign7750_e7795 * assign7750_e7795)), (((((var_esatl_dn7 * var_t6) + (var_esatl * var_t6_dn7)) * assign7750_e7795) - (assign7750_e7792 * (var_esatl_dn7 + var_t6_dn7))) / (assign7750_e7795 * assign7750_e7795)), (((((var_esatl_dn8 * var_t6) + (var_esatl * var_t6_dn8)) * assign7750_e7795) - (assign7750_e7792 * (var_esatl_dn8 + var_t6_dn8))) / (assign7750_e7795 * assign7750_e7795)),)
    } else {
        (var_vdsat, var_vdsat_dn3, var_vdsat_dn4, var_vdsat_dn5, var_vdsat_dn6, var_vdsat_dn7, var_vdsat_dn8,)
    }
};
        var_vdsat = assign7750_e7798;
        var_vdsat_dn3 = assign7750_e7798_d_n3;
        var_vdsat_dn4 = assign7750_e7798_d_n4;
        var_vdsat_dn5 = assign7750_e7798_d_n5;
        var_vdsat_dn6 = assign7750_e7798_d_n6;
        var_vdsat_dn7 = assign7750_e7798_d_n7;
        var_vdsat_dn8 = assign7750_e7798_d_n8;

        let (assign7760_e7807, assign7760_e7807_d_n4,) = {
    if (var_guard89 == 0.0) {
        let assign7760_e7803: f64 = (var_weff * var_vsat_t);
        let assign7760_e7805: f64 = (assign7760_e7803 * var_cox1);
        (assign7760_e7805, ((var_weff * var_vsat_t_dn4) * var_cox1),)
    } else {
        (var_wvcox, var_wvcox_dn4,)
    }
};
        var_wvcox = assign7760_e7807;
        var_wvcox_dn4 = assign7760_e7807_d_n4;

        let (assign7770_e7814, assign7770_e7814_d_n3, assign7770_e7814_d_n4, assign7770_e7814_d_n5, assign7770_e7814_d_n6, assign7770_e7814_d_n7, assign7770_e7814_d_n8,) = {
    if (var_guard89 == 0.0) {
        let assign7770_e7812: f64 = (var_wvcox * var_rdss);
        (assign7770_e7812, (var_wvcox * var_rdss_dn3), ((var_wvcox_dn4 * var_rdss) + (var_wvcox * var_rdss_dn4)), (var_wvcox * var_rdss_dn5), (var_wvcox * var_rdss_dn6), (var_wvcox * var_rdss_dn7), (var_wvcox * var_rdss_dn8),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign7770_e7814;
        var_t0_dn3 = assign7770_e7814_d_n3;
        var_t0_dn4 = assign7770_e7814_d_n4;
        var_t0_dn5 = assign7770_e7814_d_n5;
        var_t0_dn6 = assign7770_e7814_d_n6;
        var_t0_dn7 = assign7770_e7814_d_n7;
        var_t0_dn8 = assign7770_e7814_d_n8;

        let (assign7780_e7821, assign7780_e7821_d_n3, assign7780_e7821_d_n4, assign7780_e7821_d_n5, assign7780_e7821_d_n6, assign7780_e7821_d_n7, assign7780_e7821_d_n8,) = {
    if (var_guard89 == 0.0) {
        let assign7780_e7819: f64 = (2.0 * var_t0);
        (assign7780_e7819, (2.0 * var_t0_dn3), (2.0 * var_t0_dn4), (2.0 * var_t0_dn5), (2.0 * var_t0_dn6), (2.0 * var_t0_dn7), (2.0 * var_t0_dn8),)
    } else {
        (var_ta, var_ta_dn3, var_ta_dn4, var_ta_dn5, var_ta_dn6, var_ta_dn7, var_ta_dn8,)
    }
};
        var_ta = assign7780_e7821;
        var_ta_dn3 = assign7780_e7821_d_n3;
        var_ta_dn4 = assign7780_e7821_d_n4;
        var_ta_dn5 = assign7780_e7821_d_n5;
        var_ta_dn6 = assign7780_e7821_d_n6;
        var_ta_dn7 = assign7780_e7821_d_n7;
        var_ta_dn8 = assign7780_e7821_d_n8;

        let (assign7790_e7834, assign7790_e7834_d_n3, assign7790_e7834_d_n4, assign7790_e7834_d_n5, assign7790_e7834_d_n6, assign7790_e7834_d_n7, assign7790_e7834_d_n8,) = {
    if (var_guard89 == 0.0) {
        let assign7790_e7826: f64 = (var_t6 + var_esatl);
        let assign7790_e7829: f64 = (3.0 * var_t6);
        let assign7790_e7831: f64 = (assign7790_e7829 * var_t0);
        let assign7790_e7832: f64 = (assign7790_e7826 + assign7790_e7831);
        (assign7790_e7832, ((var_t6_dn3 + var_esatl_dn3) + (((3.0 * var_t6_dn3) * var_t0) + (assign7790_e7829 * var_t0_dn3))), ((var_t6_dn4 + var_esatl_dn4) + (((3.0 * var_t6_dn4) * var_t0) + (assign7790_e7829 * var_t0_dn4))), ((var_t6_dn5 + var_esatl_dn5) + (((3.0 * var_t6_dn5) * var_t0) + (assign7790_e7829 * var_t0_dn5))), ((var_t6_dn6 + var_esatl_dn6) + (((3.0 * var_t6_dn6) * var_t0) + (assign7790_e7829 * var_t0_dn6))), ((var_t6_dn7 + var_esatl_dn7) + (((3.0 * var_t6_dn7) * var_t0) + (assign7790_e7829 * var_t0_dn7))), ((var_t6_dn8 + var_esatl_dn8) + (((3.0 * var_t6_dn8) * var_t0) + (assign7790_e7829 * var_t0_dn8))),)
    } else {
        (var_tb, var_tb_dn3, var_tb_dn4, var_tb_dn5, var_tb_dn6, var_tb_dn7, var_tb_dn8,)
    }
};
        var_tb = assign7790_e7834;
        var_tb_dn3 = assign7790_e7834_d_n3;
        var_tb_dn4 = assign7790_e7834_d_n4;
        var_tb_dn5 = assign7790_e7834_d_n5;
        var_tb_dn6 = assign7790_e7834_d_n6;
        var_tb_dn7 = assign7790_e7834_d_n7;
        var_tb_dn8 = assign7790_e7834_d_n8;

        let (assign7800_e7847, assign7800_e7847_d_n3, assign7800_e7847_d_n4, assign7800_e7847_d_n5, assign7800_e7847_d_n6, assign7800_e7847_d_n7, assign7800_e7847_d_n8,) = {
    if (var_guard89 == 0.0) {
        let assign7800_e7841: f64 = (2.0 * var_t6);
        let assign7800_e7843: f64 = (assign7800_e7841 * var_t0);
        let assign7800_e7844: f64 = (var_esatl + assign7800_e7843);
        let assign7800_e7845: f64 = (var_t6 * assign7800_e7844);
        (assign7800_e7845, ((var_t6_dn3 * assign7800_e7844) + (var_t6 * (var_esatl_dn3 + (((2.0 * var_t6_dn3) * var_t0) + (assign7800_e7841 * var_t0_dn3))))), ((var_t6_dn4 * assign7800_e7844) + (var_t6 * (var_esatl_dn4 + (((2.0 * var_t6_dn4) * var_t0) + (assign7800_e7841 * var_t0_dn4))))), ((var_t6_dn5 * assign7800_e7844) + (var_t6 * (var_esatl_dn5 + (((2.0 * var_t6_dn5) * var_t0) + (assign7800_e7841 * var_t0_dn5))))), ((var_t6_dn6 * assign7800_e7844) + (var_t6 * (var_esatl_dn6 + (((2.0 * var_t6_dn6) * var_t0) + (assign7800_e7841 * var_t0_dn6))))), ((var_t6_dn7 * assign7800_e7844) + (var_t6 * (var_esatl_dn7 + (((2.0 * var_t6_dn7) * var_t0) + (assign7800_e7841 * var_t0_dn7))))), ((var_t6_dn8 * assign7800_e7844) + (var_t6 * (var_esatl_dn8 + (((2.0 * var_t6_dn8) * var_t0) + (assign7800_e7841 * var_t0_dn8))))),)
    } else {
        (var_tc, var_tc_dn3, var_tc_dn4, var_tc_dn5, var_tc_dn6, var_tc_dn7, var_tc_dn8,)
    }
};
        var_tc = assign7800_e7847;
        var_tc_dn3 = assign7800_e7847_d_n3;
        var_tc_dn4 = assign7800_e7847_d_n4;
        var_tc_dn5 = assign7800_e7847_d_n5;
        var_tc_dn6 = assign7800_e7847_d_n6;
        var_tc_dn7 = assign7800_e7847_d_n7;
        var_tc_dn8 = assign7800_e7847_d_n8;

        let (assign7810_e7865, assign7810_e7865_d_n3, assign7810_e7865_d_n4, assign7810_e7865_d_n5, assign7810_e7865_d_n6, assign7810_e7865_d_n7, assign7810_e7865_d_n8,) = {
    if (var_guard89 == 0.0) {
        let assign7810_e7853: f64 = (var_tb * var_tb);
        let assign7810_e7856: f64 = (2.0 * var_ta);
        let assign7810_e7858: f64 = (assign7810_e7856 * var_tc);
        let assign7810_e7859: f64 = (assign7810_e7853 - assign7810_e7858);
        let assign7810_e7860: f64 = (assign7810_e7859).sqrt();
        let assign7810_e7861: f64 = (var_tb - assign7810_e7860);
        let assign7810_e7863: f64 = (assign7810_e7861 / var_ta);
        (assign7810_e7863, ((((var_tb_dn3 - ((((var_tb_dn3 * var_tb) + (var_tb * var_tb_dn3)) - (((2.0 * var_ta_dn3) * var_tc) + (assign7810_e7856 * var_tc_dn3))) / (2.0 * assign7810_e7860))) * var_ta) - (assign7810_e7861 * var_ta_dn3)) / (var_ta * var_ta)), ((((var_tb_dn4 - ((((var_tb_dn4 * var_tb) + (var_tb * var_tb_dn4)) - (((2.0 * var_ta_dn4) * var_tc) + (assign7810_e7856 * var_tc_dn4))) / (2.0 * assign7810_e7860))) * var_ta) - (assign7810_e7861 * var_ta_dn4)) / (var_ta * var_ta)), ((((var_tb_dn5 - ((((var_tb_dn5 * var_tb) + (var_tb * var_tb_dn5)) - (((2.0 * var_ta_dn5) * var_tc) + (assign7810_e7856 * var_tc_dn5))) / (2.0 * assign7810_e7860))) * var_ta) - (assign7810_e7861 * var_ta_dn5)) / (var_ta * var_ta)), ((((var_tb_dn6 - ((((var_tb_dn6 * var_tb) + (var_tb * var_tb_dn6)) - (((2.0 * var_ta_dn6) * var_tc) + (assign7810_e7856 * var_tc_dn6))) / (2.0 * assign7810_e7860))) * var_ta) - (assign7810_e7861 * var_ta_dn6)) / (var_ta * var_ta)), ((((var_tb_dn7 - ((((var_tb_dn7 * var_tb) + (var_tb * var_tb_dn7)) - (((2.0 * var_ta_dn7) * var_tc) + (assign7810_e7856 * var_tc_dn7))) / (2.0 * assign7810_e7860))) * var_ta) - (assign7810_e7861 * var_ta_dn7)) / (var_ta * var_ta)), ((((var_tb_dn8 - ((((var_tb_dn8 * var_tb) + (var_tb * var_tb_dn8)) - (((2.0 * var_ta_dn8) * var_tc) + (assign7810_e7856 * var_tc_dn8))) / (2.0 * assign7810_e7860))) * var_ta) - (assign7810_e7861 * var_ta_dn8)) / (var_ta * var_ta)),)
    } else {
        (var_vdsat, var_vdsat_dn3, var_vdsat_dn4, var_vdsat_dn5, var_vdsat_dn6, var_vdsat_dn7, var_vdsat_dn8,)
    }
};
        var_vdsat = assign7810_e7865;
        var_vdsat_dn3 = assign7810_e7865_d_n3;
        var_vdsat_dn4 = assign7810_e7865_d_n4;
        var_vdsat_dn5 = assign7810_e7865_d_n5;
        var_vdsat_dn6 = assign7810_e7865_d_n6;
        var_vdsat_dn7 = assign7810_e7865_d_n7;
        var_vdsat_dn8 = assign7810_e7865_d_n8;

        let assign7820_e7869: f64 = (var_vdsat - 0.001);
        let assign7820_e7872: f64 = (var_vdsat - 0.001);
        let assign7820_e7875: f64 = (var_vdsat - 0.001);
        let assign7820_e7876: f64 = (assign7820_e7872 * assign7820_e7875);
        let assign7820_e7879: f64 = (4.0 * 1e-5);
        let assign7820_e7881: f64 = (assign7820_e7879 * 1e-5);
        let assign7820_e7882: f64 = (assign7820_e7876 + assign7820_e7881);
        let assign7820_e7883: f64 = (assign7820_e7882).sqrt();
        let assign7820_e7884: f64 = (assign7820_e7869 + assign7820_e7883);
        let assign7820_e7885: f64 = (0.5 * assign7820_e7884);
        let assign7820_e7887: f64 = (assign7820_e7885 + 0.001);
        var_vdsat = assign7820_e7887;
        var_vdsat_dn3 = (0.5 * (var_vdsat_dn3 + (((var_vdsat_dn3 * assign7820_e7875) + (assign7820_e7872 * var_vdsat_dn3)) / (2.0 * assign7820_e7883))));
        var_vdsat_dn4 = (0.5 * (var_vdsat_dn4 + (((var_vdsat_dn4 * assign7820_e7875) + (assign7820_e7872 * var_vdsat_dn4)) / (2.0 * assign7820_e7883))));
        var_vdsat_dn5 = (0.5 * (var_vdsat_dn5 + (((var_vdsat_dn5 * assign7820_e7875) + (assign7820_e7872 * var_vdsat_dn5)) / (2.0 * assign7820_e7883))));
        var_vdsat_dn6 = (0.5 * (var_vdsat_dn6 + (((var_vdsat_dn6 * assign7820_e7875) + (assign7820_e7872 * var_vdsat_dn6)) / (2.0 * assign7820_e7883))));
        var_vdsat_dn7 = (0.5 * (var_vdsat_dn7 + (((var_vdsat_dn7 * assign7820_e7875) + (assign7820_e7872 * var_vdsat_dn7)) / (2.0 * assign7820_e7883))));
        var_vdsat_dn8 = (0.5 * (var_vdsat_dn8 + (((var_vdsat_dn8 * assign7820_e7875) + (assign7820_e7872 * var_vdsat_dn8)) / (2.0 * assign7820_e7883))));

        let assign7830_e7890: f64 = (var_vds / var_vdsat);
        let assign7830_e7892: f64 = (assign7830_e7890).powf(var_mexp_t);
        var_t7 = assign7830_e7892;
        var_t7_dn3 = if 0.0 == 0.0 && ((var_mexp_t) as f64).is_finite() && ((var_mexp_t) as f64).fract() == 0.0 { if var_mexp_t == 0.0 { 0.0 } else { (var_mexp_t * ((assign7830_e7890).powf(var_mexp_t - 1.0) * (-((var_vds * var_vdsat_dn3) / (var_vdsat * var_vdsat))))) } } else { (assign7830_e7892 * (var_mexp_t * ((-((var_vds * var_vdsat_dn3) / (var_vdsat * var_vdsat))) / assign7830_e7890))) };
        var_t7_dn4 = if var_mexp_t_dn4 == 0.0 && ((var_mexp_t) as f64).is_finite() && ((var_mexp_t) as f64).fract() == 0.0 { if var_mexp_t == 0.0 { 0.0 } else { (var_mexp_t * ((assign7830_e7890).powf(var_mexp_t - 1.0) * (-((var_vds * var_vdsat_dn4) / (var_vdsat * var_vdsat))))) } } else { (assign7830_e7892 * ((var_mexp_t_dn4 * (assign7830_e7890).ln()) + (var_mexp_t * ((-((var_vds * var_vdsat_dn4) / (var_vdsat * var_vdsat))) / assign7830_e7890)))) };
        var_t7_dn5 = if 0.0 == 0.0 && ((var_mexp_t) as f64).is_finite() && ((var_mexp_t) as f64).fract() == 0.0 { if var_mexp_t == 0.0 { 0.0 } else { (var_mexp_t * ((assign7830_e7890).powf(var_mexp_t - 1.0) * (((var_vds_dn5 * var_vdsat) - (var_vds * var_vdsat_dn5)) / (var_vdsat * var_vdsat)))) } } else { (assign7830_e7892 * (var_mexp_t * ((((var_vds_dn5 * var_vdsat) - (var_vds * var_vdsat_dn5)) / (var_vdsat * var_vdsat)) / assign7830_e7890))) };
        var_t7_dn6 = if 0.0 == 0.0 && ((var_mexp_t) as f64).is_finite() && ((var_mexp_t) as f64).fract() == 0.0 { if var_mexp_t == 0.0 { 0.0 } else { (var_mexp_t * ((assign7830_e7890).powf(var_mexp_t - 1.0) * (((var_vds_dn6 * var_vdsat) - (var_vds * var_vdsat_dn6)) / (var_vdsat * var_vdsat)))) } } else { (assign7830_e7892 * (var_mexp_t * ((((var_vds_dn6 * var_vdsat) - (var_vds * var_vdsat_dn6)) / (var_vdsat * var_vdsat)) / assign7830_e7890))) };
        var_t7_dn7 = if 0.0 == 0.0 && ((var_mexp_t) as f64).is_finite() && ((var_mexp_t) as f64).fract() == 0.0 { if var_mexp_t == 0.0 { 0.0 } else { (var_mexp_t * ((assign7830_e7890).powf(var_mexp_t - 1.0) * (-((var_vds * var_vdsat_dn7) / (var_vdsat * var_vdsat))))) } } else { (assign7830_e7892 * (var_mexp_t * ((-((var_vds * var_vdsat_dn7) / (var_vdsat * var_vdsat))) / assign7830_e7890))) };
        var_t7_dn8 = if 0.0 == 0.0 && ((var_mexp_t) as f64).is_finite() && ((var_mexp_t) as f64).fract() == 0.0 { if var_mexp_t == 0.0 { 0.0 } else { (var_mexp_t * ((assign7830_e7890).powf(var_mexp_t - 1.0) * (-((var_vds * var_vdsat_dn8) / (var_vdsat * var_vdsat))))) } } else { (assign7830_e7892 * (var_mexp_t * ((-((var_vds * var_vdsat_dn8) / (var_vdsat * var_vdsat))) / assign7830_e7890))) };

        let assign7840_e7895: f64 = (1.0 + var_t7);
        let assign7840_e7897: f64 = (assign7840_e7895).powf(var_inv_mexp);
        var_t8 = assign7840_e7897;
        var_t8_dn3 = if 0.0 == 0.0 && ((var_inv_mexp) as f64).is_finite() && ((var_inv_mexp) as f64).fract() == 0.0 { if var_inv_mexp == 0.0 { 0.0 } else { (var_inv_mexp * ((assign7840_e7895).powf(var_inv_mexp - 1.0) * var_t7_dn3)) } } else { (assign7840_e7897 * (var_inv_mexp * (var_t7_dn3 / assign7840_e7895))) };
        var_t8_dn4 = if 0.0 == 0.0 && ((var_inv_mexp) as f64).is_finite() && ((var_inv_mexp) as f64).fract() == 0.0 { if var_inv_mexp == 0.0 { 0.0 } else { (var_inv_mexp * ((assign7840_e7895).powf(var_inv_mexp - 1.0) * var_t7_dn4)) } } else { (assign7840_e7897 * (var_inv_mexp * (var_t7_dn4 / assign7840_e7895))) };
        var_t8_dn5 = if 0.0 == 0.0 && ((var_inv_mexp) as f64).is_finite() && ((var_inv_mexp) as f64).fract() == 0.0 { if var_inv_mexp == 0.0 { 0.0 } else { (var_inv_mexp * ((assign7840_e7895).powf(var_inv_mexp - 1.0) * var_t7_dn5)) } } else { (assign7840_e7897 * (var_inv_mexp * (var_t7_dn5 / assign7840_e7895))) };
        var_t8_dn6 = if 0.0 == 0.0 && ((var_inv_mexp) as f64).is_finite() && ((var_inv_mexp) as f64).fract() == 0.0 { if var_inv_mexp == 0.0 { 0.0 } else { (var_inv_mexp * ((assign7840_e7895).powf(var_inv_mexp - 1.0) * var_t7_dn6)) } } else { (assign7840_e7897 * (var_inv_mexp * (var_t7_dn6 / assign7840_e7895))) };
        var_t8_dn7 = if 0.0 == 0.0 && ((var_inv_mexp) as f64).is_finite() && ((var_inv_mexp) as f64).fract() == 0.0 { if var_inv_mexp == 0.0 { 0.0 } else { (var_inv_mexp * ((assign7840_e7895).powf(var_inv_mexp - 1.0) * var_t7_dn7)) } } else { (assign7840_e7897 * (var_inv_mexp * (var_t7_dn7 / assign7840_e7895))) };
        var_t8_dn8 = if 0.0 == 0.0 && ((var_inv_mexp) as f64).is_finite() && ((var_inv_mexp) as f64).fract() == 0.0 { if var_inv_mexp == 0.0 { 0.0 } else { (var_inv_mexp * ((assign7840_e7895).powf(var_inv_mexp - 1.0) * var_t7_dn8)) } } else { (assign7840_e7897 * (var_inv_mexp * (var_t7_dn8 / assign7840_e7895))) };

        let assign7850_e7900: f64 = (var_vds / var_t8);
        var_vdseff = assign7850_e7900;
        var_vdseff_dn3 = (-((var_vds * var_t8_dn3) / (var_t8 * var_t8)));
        var_vdseff_dn4 = (-((var_vds * var_t8_dn4) / (var_t8 * var_t8)));
        var_vdseff_dn5 = (((var_vds_dn5 * var_t8) - (var_vds * var_t8_dn5)) / (var_t8 * var_t8));
        var_vdseff_dn6 = (((var_vds_dn6 * var_t8) - (var_vds * var_t8_dn6)) / (var_t8 * var_t8));
        var_vdseff_dn7 = (-((var_vds * var_t8_dn7) / (var_t8 * var_t8)));
        var_vdseff_dn8 = (-((var_vds * var_t8_dn8) / (var_t8 * var_t8)));

        let assign7860_e7903: f64 = if var_vdseff > var_vds { 1.0 } else { 0.0 };
        var_guard90 = assign7860_e7903;

        let (assign7870_e7907, assign7870_e7907_d_n3, assign7870_e7907_d_n4, assign7870_e7907_d_n5, assign7870_e7907_d_n6, assign7870_e7907_d_n7, assign7870_e7907_d_n8,) = {
    if (var_guard90 != 0.0) {
        (var_vds, 0.0, 0.0, var_vds_dn5, var_vds_dn6, 0.0, 0.0,)
    } else {
        (var_vdseff, var_vdseff_dn3, var_vdseff_dn4, var_vdseff_dn5, var_vdseff_dn6, var_vdseff_dn7, var_vdseff_dn8,)
    }
};
        var_vdseff = assign7870_e7907;
        var_vdseff_dn3 = assign7870_e7907_d_n3;
        var_vdseff_dn4 = assign7870_e7907_d_n4;
        var_vdseff_dn5 = assign7870_e7907_d_n5;
        var_vdseff_dn6 = assign7870_e7907_d_n6;
        var_vdseff_dn7 = assign7870_e7907_d_n7;
        var_vdseff_dn8 = assign7870_e7907_d_n8;

        let assign7880_e7910: f64 = (var_vgfb1eff - var_vdseff);
        let assign7880_e7912: f64 = (assign7880_e7910 / var_nvtm);
        var_xg1 = assign7880_e7912;
        var_xg1_dn3 = ((((var_vgfb1eff_dn3 - var_vdseff_dn3) * var_nvtm) - (assign7880_e7910 * var_nvtm_dn3)) / (var_nvtm * var_nvtm));
        var_xg1_dn4 = ((((var_vgfb1eff_dn4 - var_vdseff_dn4) * var_nvtm) - (assign7880_e7910 * var_nvtm_dn4)) / (var_nvtm * var_nvtm));
        var_xg1_dn5 = ((((var_vgfb1eff_dn5 - var_vdseff_dn5) * var_nvtm) - (assign7880_e7910 * var_nvtm_dn5)) / (var_nvtm * var_nvtm));
        var_xg1_dn6 = ((((var_vgfb1eff_dn6 - var_vdseff_dn6) * var_nvtm) - (assign7880_e7910 * var_nvtm_dn6)) / (var_nvtm * var_nvtm));
        var_xg1_dn7 = ((((var_vgfb1eff_dn7 - var_vdseff_dn7) * var_nvtm) - (assign7880_e7910 * var_nvtm_dn7)) / (var_nvtm * var_nvtm));
        var_xg1_dn8 = ((((var_vgfb1eff_dn8 - var_vdseff_dn8) * var_nvtm) - (assign7880_e7910 * var_nvtm_dn8)) / (var_nvtm * var_nvtm));

        let assign7890_e7915: f64 = (var_vgfb2 - var_dvth_all);
        let assign7890_e7917: f64 = (assign7890_e7915 + p.p10);
        let assign7890_e7919: f64 = (assign7890_e7917 - var_vdseff);
        let assign7890_e7921: f64 = (assign7890_e7919 / var_nvtm);
        var_xg2 = assign7890_e7921;
        var_xg2_dn3 = (((((var_vgfb2_dn3 - var_dvth_all_dn3) - var_vdseff_dn3) * var_nvtm) - (assign7890_e7919 * var_nvtm_dn3)) / (var_nvtm * var_nvtm));
        var_xg2_dn4 = (((((var_vgfb2_dn4 - var_dvth_all_dn4) - var_vdseff_dn4) * var_nvtm) - (assign7890_e7919 * var_nvtm_dn4)) / (var_nvtm * var_nvtm));
        var_xg2_dn5 = (((((var_vgfb2_dn5 - var_dvth_all_dn5) - var_vdseff_dn5) * var_nvtm) - (assign7890_e7919 * var_nvtm_dn5)) / (var_nvtm * var_nvtm));
        var_xg2_dn6 = (((((var_vgfb2_dn6 - var_dvth_all_dn6) - var_vdseff_dn6) * var_nvtm) - (assign7890_e7919 * var_nvtm_dn6)) / (var_nvtm * var_nvtm));
        var_xg2_dn7 = (((((var_vgfb2_dn7 - var_dvth_all_dn7) - var_vdseff_dn7) * var_nvtm) - (assign7890_e7919 * var_nvtm_dn7)) / (var_nvtm * var_nvtm));
        var_xg2_dn8 = (((((var_vgfb2_dn8 - var_dvth_all_dn8) - var_vdseff_dn8) * var_nvtm) - (assign7890_e7919 * var_nvtm_dn8)) / (var_nvtm * var_nvtm));

        let assign7900_e7925: f64 = (var_xg1 - var_phi1_0);
        let assign7900_e7926: f64 = (var_k1_2 * assign7900_e7925);
        let assign7900_e7929: f64 = (var_xg1 - var_phi1_0);
        let assign7900_e7930: f64 = (assign7900_e7926 * assign7900_e7929);
        let assign7900_e7932: f64 = (assign7900_e7930 + 39.47841);
        let assign7900_e7933: f64 = (assign7900_e7932).ln();
        let assign7900_e7935: f64 = (assign7900_e7933 - var_lna0);
        var_phissatback = assign7900_e7935;
        var_phissatback_dn3 = (((((var_k1_2 * (var_xg1_dn3 - var_phi1_0_dn3)) * assign7900_e7929) + (assign7900_e7926 * (var_xg1_dn3 - var_phi1_0_dn3))) / assign7900_e7932) - var_lna0_dn3);
        var_phissatback_dn4 = (((((var_k1_2 * (var_xg1_dn4 - var_phi1_0_dn4)) * assign7900_e7929) + (assign7900_e7926 * (var_xg1_dn4 - var_phi1_0_dn4))) / assign7900_e7932) - var_lna0_dn4);
        var_phissatback_dn5 = (((((var_k1_2 * (var_xg1_dn5 - var_phi1_0_dn5)) * assign7900_e7929) + (assign7900_e7926 * (var_xg1_dn5 - var_phi1_0_dn5))) / assign7900_e7932) - var_lna0_dn5);
        var_phissatback_dn6 = (((((var_k1_2 * (var_xg1_dn6 - var_phi1_0_dn6)) * assign7900_e7929) + (assign7900_e7926 * (var_xg1_dn6 - var_phi1_0_dn6))) / assign7900_e7932) - var_lna0_dn6);
        var_phissatback_dn7 = (((((var_k1_2 * (var_xg1_dn7 - var_phi1_0_dn7)) * assign7900_e7929) + (assign7900_e7926 * (var_xg1_dn7 - var_phi1_0_dn7))) / assign7900_e7932) - var_lna0_dn7);
        var_phissatback_dn8 = (((((var_k1_2 * (var_xg1_dn8 - var_phi1_0_dn8)) * assign7900_e7929) + (assign7900_e7926 * (var_xg1_dn8 - var_phi1_0_dn8))) / assign7900_e7932) - var_lna0_dn8);

        let assign7910_e7939: f64 = (var_xg1 - var_phi1_0);
        let assign7910_e7940: f64 = (var_k1_2 * assign7910_e7939);
        let assign7910_e7943: f64 = (var_xg1 - var_phi1_0);
        let assign7910_e7944: f64 = (assign7910_e7940 * assign7910_e7943);
        let assign7910_e7946: f64 = (assign7910_e7944 + 39.47841);
        let assign7910_e7947: f64 = (assign7910_e7946).ln();
        let assign7910_e7949: f64 = (assign7910_e7947 - var_lna0);
        var_phissat = assign7910_e7949;
        var_phissat_dn3 = (((((var_k1_2 * (var_xg1_dn3 - var_phi1_0_dn3)) * assign7910_e7943) + (assign7910_e7940 * (var_xg1_dn3 - var_phi1_0_dn3))) / assign7910_e7946) - var_lna0_dn3);
        var_phissat_dn4 = (((((var_k1_2 * (var_xg1_dn4 - var_phi1_0_dn4)) * assign7910_e7943) + (assign7910_e7940 * (var_xg1_dn4 - var_phi1_0_dn4))) / assign7910_e7946) - var_lna0_dn4);
        var_phissat_dn5 = (((((var_k1_2 * (var_xg1_dn5 - var_phi1_0_dn5)) * assign7910_e7943) + (assign7910_e7940 * (var_xg1_dn5 - var_phi1_0_dn5))) / assign7910_e7946) - var_lna0_dn5);
        var_phissat_dn6 = (((((var_k1_2 * (var_xg1_dn6 - var_phi1_0_dn6)) * assign7910_e7943) + (assign7910_e7940 * (var_xg1_dn6 - var_phi1_0_dn6))) / assign7910_e7946) - var_lna0_dn6);
        var_phissat_dn7 = (((((var_k1_2 * (var_xg1_dn7 - var_phi1_0_dn7)) * assign7910_e7943) + (assign7910_e7940 * (var_xg1_dn7 - var_phi1_0_dn7))) / assign7910_e7946) - var_lna0_dn7);
        var_phissat_dn8 = (((((var_k1_2 * (var_xg1_dn8 - var_phi1_0_dn8)) * assign7910_e7943) + (assign7910_e7940 * (var_xg1_dn8 - var_phi1_0_dn8))) / assign7910_e7946) - var_lna0_dn8);

        let assign7920_e7953: f64 = (1.0 + var_k1);
        let assign7920_e7954: f64 = (var_phi1_0 * assign7920_e7953);
        let assign7920_e7956: f64 = (assign7920_e7954 - var_phi2);
        let assign7920_e7958: f64 = (assign7920_e7956 / var_k1);
        var_t3 = assign7920_e7958;
        var_t3_dn3 = (((var_phi1_0_dn3 * assign7920_e7953) - var_phi2_dn3) / var_k1);
        var_t3_dn4 = (((var_phi1_0_dn4 * assign7920_e7953) - var_phi2_dn4) / var_k1);
        var_t3_dn5 = (((var_phi1_0_dn5 * assign7920_e7953) - var_phi2_dn5) / var_k1);
        var_t3_dn6 = (((var_phi1_0_dn6 * assign7920_e7953) - var_phi2_dn6) / var_k1);
        var_t3_dn7 = (((var_phi1_0_dn7 * assign7920_e7953) - var_phi2_dn7) / var_k1);
        var_t3_dn8 = (((var_phi1_0_dn8 * assign7920_e7953) - var_phi2_dn8) / var_k1);

        let assign7930_e7962: f64 = (var_t3 - var_phi1_0);
        let assign7930_e7963: f64 = (var_k1_2 * assign7930_e7962);
        let assign7930_e7966: f64 = (var_t3 - var_phi1_0);
        let assign7930_e7967: f64 = (assign7930_e7963 * assign7930_e7966);
        let assign7930_e7969: f64 = (assign7930_e7967 + 39.47841);
        let assign7930_e7970: f64 = (assign7930_e7969).ln();
        let assign7930_e7972: f64 = (assign7930_e7970 - var_lna0);
        var_t4 = assign7930_e7972;
        var_t4_dn3 = (((((var_k1_2 * (var_t3_dn3 - var_phi1_0_dn3)) * assign7930_e7966) + (assign7930_e7963 * (var_t3_dn3 - var_phi1_0_dn3))) / assign7930_e7969) - var_lna0_dn3);
        var_t4_dn4 = (((((var_k1_2 * (var_t3_dn4 - var_phi1_0_dn4)) * assign7930_e7966) + (assign7930_e7963 * (var_t3_dn4 - var_phi1_0_dn4))) / assign7930_e7969) - var_lna0_dn4);
        var_t4_dn5 = (((((var_k1_2 * (var_t3_dn5 - var_phi1_0_dn5)) * assign7930_e7966) + (assign7930_e7963 * (var_t3_dn5 - var_phi1_0_dn5))) / assign7930_e7969) - var_lna0_dn5);
        var_t4_dn6 = (((((var_k1_2 * (var_t3_dn6 - var_phi1_0_dn6)) * assign7930_e7966) + (assign7930_e7963 * (var_t3_dn6 - var_phi1_0_dn6))) / assign7930_e7969) - var_lna0_dn6);
        var_t4_dn7 = (((((var_k1_2 * (var_t3_dn7 - var_phi1_0_dn7)) * assign7930_e7966) + (assign7930_e7963 * (var_t3_dn7 - var_phi1_0_dn7))) / assign7930_e7969) - var_lna0_dn7);
        var_t4_dn8 = (((((var_k1_2 * (var_t3_dn8 - var_phi1_0_dn8)) * assign7930_e7966) + (assign7930_e7963 * (var_t3_dn8 - var_phi1_0_dn8))) / assign7930_e7969) - var_lna0_dn8);

        let assign7940_e7975: f64 = (var_t4 - var_phi1_0);
        var_t5 = assign7940_e7975;
        var_t5_dn3 = (var_t4_dn3 - var_phi1_0_dn3);
        var_t5_dn4 = (var_t4_dn4 - var_phi1_0_dn4);
        var_t5_dn5 = (var_t4_dn5 - var_phi1_0_dn5);
        var_t5_dn6 = (var_t4_dn6 - var_phi1_0_dn6);
        var_t5_dn7 = (var_t4_dn7 - var_phi1_0_dn7);
        var_t5_dn8 = (var_t4_dn8 - var_phi1_0_dn8);

        let assign7950_e7978: f64 = (var_phissat - var_t5);
        var_phissat = assign7950_e7978;
        var_phissat_dn3 = (var_phissat_dn3 - var_t5_dn3);
        var_phissat_dn4 = (var_phissat_dn4 - var_t5_dn4);
        var_phissat_dn5 = (var_phissat_dn5 - var_t5_dn5);
        var_phissat_dn6 = (var_phissat_dn6 - var_t5_dn6);
        var_phissat_dn7 = (var_phissat_dn7 - var_t5_dn7);
        var_phissat_dn8 = (var_phissat_dn8 - var_t5_dn8);

        let assign7960_e7982: f64 = (var_k2 * var_xg2);
        let assign7960_e7983: f64 = (var_phissat + assign7960_e7982);
        let assign7960_e7986: f64 = (1.0 + var_k2);
        let assign7960_e7987: f64 = (assign7960_e7983 / assign7960_e7986);
        var_phissatback2 = assign7960_e7987;
        var_phissatback2_dn3 = ((var_phissat_dn3 + (var_k2 * var_xg2_dn3)) / assign7960_e7986);
        var_phissatback2_dn4 = ((var_phissat_dn4 + (var_k2 * var_xg2_dn4)) / assign7960_e7986);
        var_phissatback2_dn5 = ((var_phissat_dn5 + (var_k2 * var_xg2_dn5)) / assign7960_e7986);
        var_phissatback2_dn6 = ((var_phissat_dn6 + (var_k2 * var_xg2_dn6)) / assign7960_e7986);
        var_phissatback2_dn7 = ((var_phissat_dn7 + (var_k2 * var_xg2_dn7)) / assign7960_e7986);
        var_phissatback2_dn8 = ((var_phissat_dn8 + (var_k2 * var_xg2_dn8)) / assign7960_e7986);

        let assign7970_e7992: f64 = (var_xg1 - var_xg2);
        let assign7970_e7993: f64 = (var_keq_k2 * assign7970_e7992);
        let assign7970_e7994: f64 = (var_xg2 + assign7970_e7993);
        var_phi2sub = assign7970_e7994;
        var_phi2sub_dn3 = (var_xg2_dn3 + (var_keq_k2 * (var_xg1_dn3 - var_xg2_dn3)));
        var_phi2sub_dn4 = (var_xg2_dn4 + (var_keq_k2 * (var_xg1_dn4 - var_xg2_dn4)));
        var_phi2sub_dn5 = (var_xg2_dn5 + (var_keq_k2 * (var_xg1_dn5 - var_xg2_dn5)));
        var_phi2sub_dn6 = (var_xg2_dn6 + (var_keq_k2 * (var_xg1_dn6 - var_xg2_dn6)));
        var_phi2sub_dn7 = (var_xg2_dn7 + (var_keq_k2 * (var_xg1_dn7 - var_xg2_dn7)));
        var_phi2sub_dn8 = (var_xg2_dn8 + (var_keq_k2 * (var_xg1_dn8 - var_xg2_dn8)));

        let assign7980_e7997: f64 = (var_phi2sub).min(var_phissatback);
        var_phi2 = assign7980_e7997;
        var_phi2_dn3 = if var_phi2sub <= var_phissatback { var_phi2sub_dn3 } else { var_phissatback_dn3 };
        var_phi2_dn4 = if var_phi2sub <= var_phissatback { var_phi2sub_dn4 } else { var_phissatback_dn4 };
        var_phi2_dn5 = if var_phi2sub <= var_phissatback { var_phi2sub_dn5 } else { var_phissatback_dn5 };
        var_phi2_dn6 = if var_phi2sub <= var_phissatback { var_phi2sub_dn6 } else { var_phissatback_dn6 };
        var_phi2_dn7 = if var_phi2sub <= var_phissatback { var_phi2sub_dn7 } else { var_phissatback_dn7 };
        var_phi2_dn8 = if var_phi2sub <= var_phissatback { var_phi2sub_dn8 } else { var_phissatback_dn8 };

        let assign7990_e8000: f64 = (var_phi2).min(var_phi1_0);
        var_phi2 = assign7990_e8000;
        var_phi2_dn3 = if var_phi2 <= var_phi1_0 { var_phi2_dn3 } else { var_phi1_0_dn3 };
        var_phi2_dn4 = if var_phi2 <= var_phi1_0 { var_phi2_dn4 } else { var_phi1_0_dn4 };
        var_phi2_dn5 = if var_phi2 <= var_phi1_0 { var_phi2_dn5 } else { var_phi1_0_dn5 };
        var_phi2_dn6 = if var_phi2 <= var_phi1_0 { var_phi2_dn6 } else { var_phi1_0_dn6 };
        var_phi2_dn7 = if var_phi2 <= var_phi1_0 { var_phi2_dn7 } else { var_phi1_0_dn7 };
        var_phi2_dn8 = if var_phi2 <= var_phi1_0 { var_phi2_dn8 } else { var_phi1_0_dn8 };

        let assign8000_e8004: f64 = (var_k1 * var_xg1);
        let assign8000_e8005: f64 = (var_phi2 + assign8000_e8004);
        let assign8000_e8008: f64 = (1.0 + var_k1);
        let assign8000_e8009: f64 = (assign8000_e8005 / assign8000_e8008);
        var_phi1 = assign8000_e8009;
        var_phi1_dn3 = ((var_phi2_dn3 + (var_k1 * var_xg1_dn3)) / assign8000_e8008);
        var_phi1_dn4 = ((var_phi2_dn4 + (var_k1 * var_xg1_dn4)) / assign8000_e8008);
        var_phi1_dn5 = ((var_phi2_dn5 + (var_k1 * var_xg1_dn5)) / assign8000_e8008);
        var_phi1_dn6 = ((var_phi2_dn6 + (var_k1 * var_xg1_dn6)) / assign8000_e8008);
        var_phi1_dn7 = ((var_phi2_dn7 + (var_k1 * var_xg1_dn7)) / assign8000_e8008);
        var_phi1_dn8 = ((var_phi2_dn8 + (var_k1 * var_xg1_dn8)) / assign8000_e8008);

        let assign8010_e8012: f64 = (var_phi1 - var_phi2);
        var_t0 = assign8010_e8012;
        var_t0_dn3 = (var_phi1_dn3 - var_phi2_dn3);
        var_t0_dn4 = (var_phi1_dn4 - var_phi2_dn4);
        var_t0_dn5 = (var_phi1_dn5 - var_phi2_dn5);
        var_t0_dn6 = (var_phi1_dn6 - var_phi2_dn6);
        var_t0_dn7 = (var_phi1_dn7 - var_phi2_dn7);
        var_t0_dn8 = (var_phi1_dn8 - var_phi2_dn8);

        let assign8020_e8014: f64 = { let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign8020_e8016: f64 = { let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign8020_e8018: f64 = (assign8020_e8016 - 1.0);
        let assign8020_e8019: f64 = (assign8020_e8014 * assign8020_e8018);
        let assign8020_e8021: f64 = (assign8020_e8019 / var_t0);
        var_t3 = assign8020_e8021;
        var_t3_dn3 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn3) * assign8020_e8018) + (assign8020_e8014 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn3))) * var_t0) - (assign8020_e8019 * var_t0_dn3)) / (var_t0 * var_t0));
        var_t3_dn4 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn4) * assign8020_e8018) + (assign8020_e8014 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn4))) * var_t0) - (assign8020_e8019 * var_t0_dn4)) / (var_t0 * var_t0));
        var_t3_dn5 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn5) * assign8020_e8018) + (assign8020_e8014 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn5))) * var_t0) - (assign8020_e8019 * var_t0_dn5)) / (var_t0 * var_t0));
        var_t3_dn6 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn6) * assign8020_e8018) + (assign8020_e8014 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn6))) * var_t0) - (assign8020_e8019 * var_t0_dn6)) / (var_t0 * var_t0));
        var_t3_dn7 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn7) * assign8020_e8018) + (assign8020_e8014 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn7))) * var_t0) - (assign8020_e8019 * var_t0_dn7)) / (var_t0 * var_t0));
        var_t3_dn8 = (((((({ let limited_exp_arg = var_phi2; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_phi2_dn8) * assign8020_e8018) + (assign8020_e8014 * ({ let limited_exp_arg = var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t0_dn8))) * var_t0) - (assign8020_e8019 * var_t0_dn8)) / (var_t0 * var_t0));

        let assign8030_e8024: f64 = (var_xg2 - var_phissatback2);
        var_q2 = assign8030_e8024;
        var_q2_dn3 = (var_xg2_dn3 - var_phissatback2_dn3);
        var_q2_dn4 = (var_xg2_dn4 - var_phissatback2_dn4);
        var_q2_dn5 = (var_xg2_dn5 - var_phissatback2_dn5);
        var_q2_dn6 = (var_xg2_dn6 - var_phissatback2_dn6);
        var_q2_dn7 = (var_xg2_dn7 - var_phissatback2_dn7);
        var_q2_dn8 = (var_xg2_dn8 - var_phissatback2_dn8);

        let assign8040_e8027: f64 = (var_k2 * var_k2);
        let assign8040_e8029: f64 = (assign8040_e8027 * var_q2);
        let assign8040_e8031: f64 = (assign8040_e8029 * var_q2);
        let assign8040_e8034: f64 = (var_phissatback2).exp();
        let assign8040_e8035: f64 = (var_a0 * assign8040_e8034);
        let assign8040_e8036: f64 = (assign8040_e8031 - assign8040_e8035);
        var_qsqrt = assign8040_e8036;
        var_qsqrt_dn3 = ((((assign8040_e8027 * var_q2_dn3) * var_q2) + (assign8040_e8029 * var_q2_dn3)) - ((var_a0_dn3 * assign8040_e8034) + (var_a0 * (assign8040_e8034 * var_phissatback2_dn3))));
        var_qsqrt_dn4 = ((((assign8040_e8027 * var_q2_dn4) * var_q2) + (assign8040_e8029 * var_q2_dn4)) - ((var_a0_dn4 * assign8040_e8034) + (var_a0 * (assign8040_e8034 * var_phissatback2_dn4))));
        var_qsqrt_dn5 = ((((assign8040_e8027 * var_q2_dn5) * var_q2) + (assign8040_e8029 * var_q2_dn5)) - ((var_a0_dn5 * assign8040_e8034) + (var_a0 * (assign8040_e8034 * var_phissatback2_dn5))));
        var_qsqrt_dn6 = ((((assign8040_e8027 * var_q2_dn6) * var_q2) + (assign8040_e8029 * var_q2_dn6)) - ((var_a0_dn6 * assign8040_e8034) + (var_a0 * (assign8040_e8034 * var_phissatback2_dn6))));
        var_qsqrt_dn7 = ((((assign8040_e8027 * var_q2_dn7) * var_q2) + (assign8040_e8029 * var_q2_dn7)) - ((var_a0_dn7 * assign8040_e8034) + (var_a0 * (assign8040_e8034 * var_phissatback2_dn7))));
        var_qsqrt_dn8 = ((((assign8040_e8027 * var_q2_dn8) * var_q2) + (assign8040_e8029 * var_q2_dn8)) - ((var_a0_dn8 * assign8040_e8034) + (var_a0 * (assign8040_e8034 * var_phissatback2_dn8))));

        let assign8050_e8039: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard91 = assign8050_e8039;

        let (assign8060_e8047, assign8060_e8047_d_n3, assign8060_e8047_d_n4, assign8060_e8047_d_n5, assign8060_e8047_d_n6, assign8060_e8047_d_n7, assign8060_e8047_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8060_e8043: f64 = (var_xg2 - var_phi2);
        let assign8060_e8045: f64 = (assign8060_e8043 * var_k2);
        (assign8060_e8045, ((var_xg2_dn3 - var_phi2_dn3) * var_k2), ((var_xg2_dn4 - var_phi2_dn4) * var_k2), ((var_xg2_dn5 - var_phi2_dn5) * var_k2), ((var_xg2_dn6 - var_phi2_dn6) * var_k2), ((var_xg2_dn7 - var_phi2_dn7) * var_k2), ((var_xg2_dn8 - var_phi2_dn8) * var_k2),)
    } else {
        (var_q2, var_q2_dn3, var_q2_dn4, var_q2_dn5, var_q2_dn6, var_q2_dn7, var_q2_dn8,)
    }
};
        var_q2 = assign8060_e8047;
        var_q2_dn3 = assign8060_e8047_d_n3;
        var_q2_dn4 = assign8060_e8047_d_n4;
        var_q2_dn5 = assign8060_e8047_d_n5;
        var_q2_dn6 = assign8060_e8047_d_n6;
        var_q2_dn7 = assign8060_e8047_d_n7;
        var_q2_dn8 = assign8060_e8047_d_n8;

        *var_esat_slot = var_esat;
        *var_esat_dn3_slot = var_esat_dn3;
        *var_esat_dn4_slot = var_esat_dn4;
        *var_esat_dn5_slot = var_esat_dn5;
        *var_esat_dn6_slot = var_esat_dn6;
        *var_esat_dn7_slot = var_esat_dn7;
        *var_esat_dn8_slot = var_esat_dn8;
        *var_esatl_slot = var_esatl;
        *var_esatl_dn3_slot = var_esatl_dn3;
        *var_esatl_dn4_slot = var_esatl_dn4;
        *var_esatl_dn5_slot = var_esatl_dn5;
        *var_esatl_dn6_slot = var_esatl_dn6;
        *var_esatl_dn7_slot = var_esatl_dn7;
        *var_esatl_dn8_slot = var_esatl_dn8;
        *var_guard89_slot = var_guard89;
        *var_guard90_slot = var_guard90;
        *var_guard91_slot = var_guard91;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_phi2_slot = var_phi2;
        *var_phi2_dn3_slot = var_phi2_dn3;
        *var_phi2_dn4_slot = var_phi2_dn4;
        *var_phi2_dn5_slot = var_phi2_dn5;
        *var_phi2_dn6_slot = var_phi2_dn6;
        *var_phi2_dn7_slot = var_phi2_dn7;
        *var_phi2_dn8_slot = var_phi2_dn8;
        *var_phi2sub_slot = var_phi2sub;
        *var_phi2sub_dn3_slot = var_phi2sub_dn3;
        *var_phi2sub_dn4_slot = var_phi2sub_dn4;
        *var_phi2sub_dn5_slot = var_phi2sub_dn5;
        *var_phi2sub_dn6_slot = var_phi2sub_dn6;
        *var_phi2sub_dn7_slot = var_phi2sub_dn7;
        *var_phi2sub_dn8_slot = var_phi2sub_dn8;
        *var_phissat_slot = var_phissat;
        *var_phissat_dn3_slot = var_phissat_dn3;
        *var_phissat_dn4_slot = var_phissat_dn4;
        *var_phissat_dn5_slot = var_phissat_dn5;
        *var_phissat_dn6_slot = var_phissat_dn6;
        *var_phissat_dn7_slot = var_phissat_dn7;
        *var_phissat_dn8_slot = var_phissat_dn8;
        *var_phissatback_slot = var_phissatback;
        *var_phissatback2_slot = var_phissatback2;
        *var_phissatback2_dn3_slot = var_phissatback2_dn3;
        *var_phissatback2_dn4_slot = var_phissatback2_dn4;
        *var_phissatback2_dn5_slot = var_phissatback2_dn5;
        *var_phissatback2_dn6_slot = var_phissatback2_dn6;
        *var_phissatback2_dn7_slot = var_phissatback2_dn7;
        *var_phissatback2_dn8_slot = var_phissatback2_dn8;
        *var_phissatback_dn3_slot = var_phissatback_dn3;
        *var_phissatback_dn4_slot = var_phissatback_dn4;
        *var_phissatback_dn5_slot = var_phissatback_dn5;
        *var_phissatback_dn6_slot = var_phissatback_dn6;
        *var_phissatback_dn7_slot = var_phissatback_dn7;
        *var_phissatback_dn8_slot = var_phissatback_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_rdss_slot = var_rdss;
        *var_rdss_dn3_slot = var_rdss_dn3;
        *var_rdss_dn4_slot = var_rdss_dn4;
        *var_rdss_dn5_slot = var_rdss_dn5;
        *var_rdss_dn6_slot = var_rdss_dn6;
        *var_rdss_dn7_slot = var_rdss_dn7;
        *var_rdss_dn8_slot = var_rdss_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t7_slot = var_t7;
        *var_t7_dn3_slot = var_t7_dn3;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t8_slot = var_t8;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_ta_slot = var_ta;
        *var_ta_dn3_slot = var_ta_dn3;
        *var_ta_dn4_slot = var_ta_dn4;
        *var_ta_dn5_slot = var_ta_dn5;
        *var_ta_dn6_slot = var_ta_dn6;
        *var_ta_dn7_slot = var_ta_dn7;
        *var_ta_dn8_slot = var_ta_dn8;
        *var_tb_slot = var_tb;
        *var_tb_dn3_slot = var_tb_dn3;
        *var_tb_dn4_slot = var_tb_dn4;
        *var_tb_dn5_slot = var_tb_dn5;
        *var_tb_dn6_slot = var_tb_dn6;
        *var_tb_dn7_slot = var_tb_dn7;
        *var_tb_dn8_slot = var_tb_dn8;
        *var_tc_slot = var_tc;
        *var_tc_dn3_slot = var_tc_dn3;
        *var_tc_dn4_slot = var_tc_dn4;
        *var_tc_dn5_slot = var_tc_dn5;
        *var_tc_dn6_slot = var_tc_dn6;
        *var_tc_dn7_slot = var_tc_dn7;
        *var_tc_dn8_slot = var_tc_dn8;
        *var_vdsat_slot = var_vdsat;
        *var_vdsat_dn3_slot = var_vdsat_dn3;
        *var_vdsat_dn4_slot = var_vdsat_dn4;
        *var_vdsat_dn5_slot = var_vdsat_dn5;
        *var_vdsat_dn6_slot = var_vdsat_dn6;
        *var_vdsat_dn7_slot = var_vdsat_dn7;
        *var_vdsat_dn8_slot = var_vdsat_dn8;
        *var_vdseff_slot = var_vdseff;
        *var_vdseff_dn3_slot = var_vdseff_dn3;
        *var_vdseff_dn4_slot = var_vdseff_dn4;
        *var_vdseff_dn5_slot = var_vdseff_dn5;
        *var_vdseff_dn6_slot = var_vdseff_dn6;
        *var_vdseff_dn7_slot = var_vdseff_dn7;
        *var_vdseff_dn8_slot = var_vdseff_dn8;
        *var_wvcox_slot = var_wvcox;
        *var_wvcox_dn4_slot = var_wvcox_dn4;
        *var_xg1_slot = var_xg1;
        *var_xg1_dn3_slot = var_xg1_dn3;
        *var_xg1_dn4_slot = var_xg1_dn4;
        *var_xg1_dn5_slot = var_xg1_dn5;
        *var_xg1_dn6_slot = var_xg1_dn6;
        *var_xg1_dn7_slot = var_xg1_dn7;
        *var_xg1_dn8_slot = var_xg1_dn8;
        *var_xg2_slot = var_xg2;
        *var_xg2_dn3_slot = var_xg2_dn3;
        *var_xg2_dn4_slot = var_xg2_dn4;
        *var_xg2_dn5_slot = var_xg2_dn5;
        *var_xg2_dn6_slot = var_xg2_dn6;
        *var_xg2_dn7_slot = var_xg2_dn7;
        *var_xg2_dn8_slot = var_xg2_dn8;
    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_aigen_i_slot: &mut f64,
        var_aigen_i_rv_slot: &mut f64,
        var_alpha0_i_slot: &mut f64,
        var_alpha0_i_rv_slot: &mut f64,
        var_alpha1_i_slot: &mut f64,
        var_alpha1_i_rv_slot: &mut f64,
        var_alphaii0_i_slot: &mut f64,
        var_alphaii0_i_rv_slot: &mut f64,
        var_alphaii1_i_slot: &mut f64,
        var_alphaii1_i_rv_slot: &mut f64,
        var_atats_i_slot: &mut f64,
        var_atats_i_rv_slot: &mut f64,
        var_beta0_i_slot: &mut f64,
        var_beta0_i_rv_slot: &mut f64,
        var_betaii0_i_slot: &mut f64,
        var_betaii0_i_rv_slot: &mut f64,
        var_betaii1_i_slot: &mut f64,
        var_betaii1_i_rv_slot: &mut f64,
        var_betaii2_i_slot: &mut f64,
        var_betaii2_i_rv_slot: &mut f64,
        var_bgisl_i_slot: &mut f64,
        var_bgisl_i_rv_slot: &mut f64,
        var_bigen_i_slot: &mut f64,
        var_bigen_i_rv_slot: &mut f64,
        var_btats_i_slot: &mut f64,
        var_btats_i_rv_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfd_i_rv_slot: &mut f64,
        var_cfs_i_slot: &mut f64,
        var_cfs_i_rv_slot: &mut f64,
        var_cgbl_i_slot: &mut f64,
        var_cgbl_i_rv_slot: &mut f64,
        var_cgdl_i_slot: &mut f64,
        var_cgdl_i_rv_slot: &mut f64,
        var_cgisl_i_slot: &mut f64,
        var_cgisl_i_rv_slot: &mut f64,
        var_cgsl_i_slot: &mut f64,
        var_cgsl_i_rv_slot: &mut f64,
        var_ckappab_i_slot: &mut f64,
        var_ckappab_i_rv_slot: &mut f64,
        var_ckappad_i_slot: &mut f64,
        var_ckappad_i_rv_slot: &mut f64,
        var_ckappas_i_slot: &mut f64,
        var_ckappas_i_rv_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn0_slot: &mut f64,
        var_covd_i_dn10_slot: &mut f64,
        var_covd_i_dn11_slot: &mut f64,
        var_covd_i_dn13_slot: &mut f64,
        var_covd_i_dn14_slot: &mut f64,
        var_covd_i_dn2_slot: &mut f64,
        var_covd_i_dn3_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn5_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covd_i_rv_slot: &mut f64,
        var_covs_i_slot: &mut f64,
        var_covs_i_dn0_slot: &mut f64,
        var_covs_i_dn10_slot: &mut f64,
        var_covs_i_dn11_slot: &mut f64,
        var_covs_i_dn13_slot: &mut f64,
        var_covs_i_dn14_slot: &mut f64,
        var_covs_i_dn2_slot: &mut f64,
        var_covs_i_dn3_slot: &mut f64,
        var_covs_i_dn4_slot: &mut f64,
        var_covs_i_dn5_slot: &mut f64,
        var_covs_i_dn6_slot: &mut f64,
        var_covs_i_dn7_slot: &mut f64,
        var_covs_i_dn8_slot: &mut f64,
        var_covs_i_dn9_slot: &mut f64,
        var_covs_i_rv_slot: &mut f64,
        var_ctats_i_slot: &mut f64,
        var_ctats_i_rv_slot: &mut f64,
        var_dtats_i_slot: &mut f64,
        var_dtats_i_rv_slot: &mut f64,
        var_egisl_i_slot: &mut f64,
        var_egisl_i_rv_slot: &mut f64,
        var_esatii_i_slot: &mut f64,
        var_esatii_i_rv_slot: &mut f64,
        var_lii_i_slot: &mut f64,
        var_lii_i_rv_slot: &mut f64,
        var_pgisl_i_slot: &mut f64,
        var_pgisl_i_rv_slot: &mut f64,
        var_sii0_i_slot: &mut f64,
        var_sii0_i_rv_slot: &mut f64,
        var_sii1_i_slot: &mut f64,
        var_sii1_i_rv_slot: &mut f64,
        var_sii2_i_slot: &mut f64,
        var_sii2_i_rv_slot: &mut f64,
        var_siid_i_slot: &mut f64,
        var_siid_i_rv_slot: &mut f64,
        var_tii_i_slot: &mut f64,
        var_tii_i_rv_slot: &mut f64,
        var_ute1_i_slot: &mut f64,
        var_ute1_i_rv_slot: &mut f64,
        var_ute_i_slot: &mut f64,
        var_ute_i_rv_slot: &mut f64,
    ) {
        let mut var_aigen_i: f64 = *var_aigen_i_slot;
        let mut var_aigen_i_rv: f64 = *var_aigen_i_rv_slot;
        let mut var_alpha0_i: f64 = *var_alpha0_i_slot;
        let mut var_alpha0_i_rv: f64 = *var_alpha0_i_rv_slot;
        let mut var_alpha1_i: f64 = *var_alpha1_i_slot;
        let mut var_alpha1_i_rv: f64 = *var_alpha1_i_rv_slot;
        let mut var_alphaii0_i: f64 = *var_alphaii0_i_slot;
        let mut var_alphaii0_i_rv: f64 = *var_alphaii0_i_rv_slot;
        let mut var_alphaii1_i: f64 = *var_alphaii1_i_slot;
        let mut var_alphaii1_i_rv: f64 = *var_alphaii1_i_rv_slot;
        let mut var_atats_i: f64 = *var_atats_i_slot;
        let mut var_atats_i_rv: f64 = *var_atats_i_rv_slot;
        let mut var_beta0_i: f64 = *var_beta0_i_slot;
        let mut var_beta0_i_rv: f64 = *var_beta0_i_rv_slot;
        let mut var_betaii0_i: f64 = *var_betaii0_i_slot;
        let mut var_betaii0_i_rv: f64 = *var_betaii0_i_rv_slot;
        let mut var_betaii1_i: f64 = *var_betaii1_i_slot;
        let mut var_betaii1_i_rv: f64 = *var_betaii1_i_rv_slot;
        let mut var_betaii2_i: f64 = *var_betaii2_i_slot;
        let mut var_betaii2_i_rv: f64 = *var_betaii2_i_rv_slot;
        let mut var_bgisl_i: f64 = *var_bgisl_i_slot;
        let mut var_bgisl_i_rv: f64 = *var_bgisl_i_rv_slot;
        let mut var_bigen_i: f64 = *var_bigen_i_slot;
        let mut var_bigen_i_rv: f64 = *var_bigen_i_rv_slot;
        let mut var_btats_i: f64 = *var_btats_i_slot;
        let mut var_btats_i_rv: f64 = *var_btats_i_rv_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfd_i_rv: f64 = *var_cfd_i_rv_slot;
        let mut var_cfs_i: f64 = *var_cfs_i_slot;
        let mut var_cfs_i_rv: f64 = *var_cfs_i_rv_slot;
        let mut var_cgbl_i: f64 = *var_cgbl_i_slot;
        let mut var_cgbl_i_rv: f64 = *var_cgbl_i_rv_slot;
        let mut var_cgdl_i: f64 = *var_cgdl_i_slot;
        let mut var_cgdl_i_rv: f64 = *var_cgdl_i_rv_slot;
        let mut var_cgisl_i: f64 = *var_cgisl_i_slot;
        let mut var_cgisl_i_rv: f64 = *var_cgisl_i_rv_slot;
        let mut var_cgsl_i: f64 = *var_cgsl_i_slot;
        let mut var_cgsl_i_rv: f64 = *var_cgsl_i_rv_slot;
        let mut var_ckappab_i: f64 = *var_ckappab_i_slot;
        let mut var_ckappab_i_rv: f64 = *var_ckappab_i_rv_slot;
        let mut var_ckappad_i: f64 = *var_ckappad_i_slot;
        let mut var_ckappad_i_rv: f64 = *var_ckappad_i_rv_slot;
        let mut var_ckappas_i: f64 = *var_ckappas_i_slot;
        let mut var_ckappas_i_rv: f64 = *var_ckappas_i_rv_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn0: f64 = *var_covd_i_dn0_slot;
        let mut var_covd_i_dn10: f64 = *var_covd_i_dn10_slot;
        let mut var_covd_i_dn11: f64 = *var_covd_i_dn11_slot;
        let mut var_covd_i_dn13: f64 = *var_covd_i_dn13_slot;
        let mut var_covd_i_dn14: f64 = *var_covd_i_dn14_slot;
        let mut var_covd_i_dn2: f64 = *var_covd_i_dn2_slot;
        let mut var_covd_i_dn3: f64 = *var_covd_i_dn3_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn5: f64 = *var_covd_i_dn5_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covd_i_rv: f64 = *var_covd_i_rv_slot;
        let mut var_covs_i: f64 = *var_covs_i_slot;
        let mut var_covs_i_dn0: f64 = *var_covs_i_dn0_slot;
        let mut var_covs_i_dn10: f64 = *var_covs_i_dn10_slot;
        let mut var_covs_i_dn11: f64 = *var_covs_i_dn11_slot;
        let mut var_covs_i_dn13: f64 = *var_covs_i_dn13_slot;
        let mut var_covs_i_dn14: f64 = *var_covs_i_dn14_slot;
        let mut var_covs_i_dn2: f64 = *var_covs_i_dn2_slot;
        let mut var_covs_i_dn3: f64 = *var_covs_i_dn3_slot;
        let mut var_covs_i_dn4: f64 = *var_covs_i_dn4_slot;
        let mut var_covs_i_dn5: f64 = *var_covs_i_dn5_slot;
        let mut var_covs_i_dn6: f64 = *var_covs_i_dn6_slot;
        let mut var_covs_i_dn7: f64 = *var_covs_i_dn7_slot;
        let mut var_covs_i_dn8: f64 = *var_covs_i_dn8_slot;
        let mut var_covs_i_dn9: f64 = *var_covs_i_dn9_slot;
        let mut var_covs_i_rv: f64 = *var_covs_i_rv_slot;
        let mut var_ctats_i: f64 = *var_ctats_i_slot;
        let mut var_ctats_i_rv: f64 = *var_ctats_i_rv_slot;
        let mut var_dtats_i: f64 = *var_dtats_i_slot;
        let mut var_dtats_i_rv: f64 = *var_dtats_i_rv_slot;
        let mut var_egisl_i: f64 = *var_egisl_i_slot;
        let mut var_egisl_i_rv: f64 = *var_egisl_i_rv_slot;
        let mut var_esatii_i: f64 = *var_esatii_i_slot;
        let mut var_esatii_i_rv: f64 = *var_esatii_i_rv_slot;
        let mut var_lii_i: f64 = *var_lii_i_slot;
        let mut var_lii_i_rv: f64 = *var_lii_i_rv_slot;
        let mut var_pgisl_i: f64 = *var_pgisl_i_slot;
        let mut var_pgisl_i_rv: f64 = *var_pgisl_i_rv_slot;
        let mut var_sii0_i: f64 = *var_sii0_i_slot;
        let mut var_sii0_i_rv: f64 = *var_sii0_i_rv_slot;
        let mut var_sii1_i: f64 = *var_sii1_i_slot;
        let mut var_sii1_i_rv: f64 = *var_sii1_i_rv_slot;
        let mut var_sii2_i: f64 = *var_sii2_i_slot;
        let mut var_sii2_i_rv: f64 = *var_sii2_i_rv_slot;
        let mut var_siid_i: f64 = *var_siid_i_slot;
        let mut var_siid_i_rv: f64 = *var_siid_i_rv_slot;
        let mut var_tii_i: f64 = *var_tii_i_slot;
        let mut var_tii_i_rv: f64 = *var_tii_i_rv_slot;
        let mut var_ute1_i: f64 = *var_ute1_i_slot;
        let mut var_ute1_i_rv: f64 = *var_ute1_i_rv_slot;
        let mut var_ute_i: f64 = *var_ute_i_slot;
        let mut var_ute_i_rv: f64 = *var_ute_i_rv_slot;

        let assign5130_e6903: f64 = (var_inv_l * p.p1306);
        let assign5130_e6904: f64 = (p.p1305 + assign5130_e6903);
        let assign5130_e6907: f64 = (var_inv_nfin * p.p1307);
        let assign5130_e6908: f64 = (assign5130_e6904 + assign5130_e6907);
        let assign5130_e6911: f64 = (var_inv_lnfin * p.p1308);
        let assign5130_e6912: f64 = (assign5130_e6908 + assign5130_e6911);
        let assign5130_e6915: f64 = (var_inv_w * p.p1309);
        let assign5130_e6916: f64 = (assign5130_e6912 + assign5130_e6915);
        let assign5130_e6919: f64 = (var_inv_wl * p.p1310);
        let assign5130_e6920: f64 = (assign5130_e6916 + assign5130_e6919);
        var_bgisl_i = assign5130_e6920;
        var_bgisl_i_rv = 0.0;

        let assign5140_e6924: f64 = (var_inv_l * p.p1312);
        let assign5140_e6925: f64 = (p.p1311 + assign5140_e6924);
        let assign5140_e6928: f64 = (var_inv_nfin * p.p1313);
        let assign5140_e6929: f64 = (assign5140_e6925 + assign5140_e6928);
        let assign5140_e6932: f64 = (var_inv_lnfin * p.p1314);
        let assign5140_e6933: f64 = (assign5140_e6929 + assign5140_e6932);
        let assign5140_e6936: f64 = (var_inv_w * p.p1315);
        let assign5140_e6937: f64 = (assign5140_e6933 + assign5140_e6936);
        let assign5140_e6940: f64 = (var_inv_wl * p.p1316);
        let assign5140_e6941: f64 = (assign5140_e6937 + assign5140_e6940);
        var_cgisl_i = assign5140_e6941;
        var_cgisl_i_rv = 0.0;

        let assign5150_e6945: f64 = (var_inv_l * p.p1318);
        let assign5150_e6946: f64 = (p.p1317 + assign5150_e6945);
        let assign5150_e6949: f64 = (var_inv_nfin * p.p1319);
        let assign5150_e6950: f64 = (assign5150_e6946 + assign5150_e6949);
        let assign5150_e6953: f64 = (var_inv_lnfin * p.p1320);
        let assign5150_e6954: f64 = (assign5150_e6950 + assign5150_e6953);
        let assign5150_e6957: f64 = (var_inv_w * p.p1321);
        let assign5150_e6958: f64 = (assign5150_e6954 + assign5150_e6957);
        let assign5150_e6961: f64 = (var_inv_wl * p.p1322);
        let assign5150_e6962: f64 = (assign5150_e6958 + assign5150_e6961);
        var_egisl_i = assign5150_e6962;
        var_egisl_i_rv = 0.0;

        let assign5160_e6966: f64 = (var_inv_l * p.p1324);
        let assign5160_e6967: f64 = (p.p1323 + assign5160_e6966);
        let assign5160_e6970: f64 = (var_inv_nfin * p.p1325);
        let assign5160_e6971: f64 = (assign5160_e6967 + assign5160_e6970);
        let assign5160_e6974: f64 = (var_inv_lnfin * p.p1326);
        let assign5160_e6975: f64 = (assign5160_e6971 + assign5160_e6974);
        let assign5160_e6978: f64 = (var_inv_w * p.p1327);
        let assign5160_e6979: f64 = (assign5160_e6975 + assign5160_e6978);
        let assign5160_e6982: f64 = (var_inv_wl * p.p1328);
        let assign5160_e6983: f64 = (assign5160_e6979 + assign5160_e6982);
        var_pgisl_i = assign5160_e6983;
        var_pgisl_i_rv = 0.0;

        let assign5170_e6987: f64 = (var_inv_l * p.p1354);
        let assign5170_e6988: f64 = (p.p1353 + assign5170_e6987);
        let assign5170_e6991: f64 = (var_inv_nfin * p.p1355);
        let assign5170_e6992: f64 = (assign5170_e6988 + assign5170_e6991);
        let assign5170_e6995: f64 = (var_inv_lnfin * p.p1356);
        let assign5170_e6996: f64 = (assign5170_e6992 + assign5170_e6995);
        let assign5170_e6999: f64 = (var_inv_w * p.p1357);
        let assign5170_e7000: f64 = (assign5170_e6996 + assign5170_e6999);
        let assign5170_e7003: f64 = (var_inv_wl * p.p1358);
        let assign5170_e7004: f64 = (assign5170_e7000 + assign5170_e7003);
        var_atats_i = assign5170_e7004;
        var_atats_i_rv = 0.0;

        let assign5180_e7008: f64 = (var_inv_l * p.p1360);
        let assign5180_e7009: f64 = (p.p1359 + assign5180_e7008);
        let assign5180_e7012: f64 = (var_inv_nfin * p.p1361);
        let assign5180_e7013: f64 = (assign5180_e7009 + assign5180_e7012);
        let assign5180_e7016: f64 = (var_inv_lnfin * p.p1362);
        let assign5180_e7017: f64 = (assign5180_e7013 + assign5180_e7016);
        let assign5180_e7020: f64 = (var_inv_w * p.p1363);
        let assign5180_e7021: f64 = (assign5180_e7017 + assign5180_e7020);
        let assign5180_e7024: f64 = (var_inv_wl * p.p1364);
        let assign5180_e7025: f64 = (assign5180_e7021 + assign5180_e7024);
        var_btats_i = assign5180_e7025;
        var_btats_i_rv = 0.0;

        let assign5190_e7029: f64 = (var_inv_l * p.p1366);
        let assign5190_e7030: f64 = (p.p1365 + assign5190_e7029);
        let assign5190_e7033: f64 = (var_inv_nfin * p.p1367);
        let assign5190_e7034: f64 = (assign5190_e7030 + assign5190_e7033);
        let assign5190_e7037: f64 = (var_inv_lnfin * p.p1368);
        let assign5190_e7038: f64 = (assign5190_e7034 + assign5190_e7037);
        let assign5190_e7041: f64 = (var_inv_w * p.p1369);
        let assign5190_e7042: f64 = (assign5190_e7038 + assign5190_e7041);
        let assign5190_e7045: f64 = (var_inv_wl * p.p1370);
        let assign5190_e7046: f64 = (assign5190_e7042 + assign5190_e7045);
        var_ctats_i = assign5190_e7046;
        var_ctats_i_rv = 0.0;

        let assign5200_e7050: f64 = (var_inv_l * p.p1372);
        let assign5200_e7051: f64 = (p.p1371 + assign5200_e7050);
        let assign5200_e7054: f64 = (var_inv_nfin * p.p1373);
        let assign5200_e7055: f64 = (assign5200_e7051 + assign5200_e7054);
        let assign5200_e7058: f64 = (var_inv_lnfin * p.p1374);
        let assign5200_e7059: f64 = (assign5200_e7055 + assign5200_e7058);
        let assign5200_e7062: f64 = (var_inv_w * p.p1375);
        let assign5200_e7063: f64 = (assign5200_e7059 + assign5200_e7062);
        let assign5200_e7066: f64 = (var_inv_wl * p.p1376);
        let assign5200_e7067: f64 = (assign5200_e7063 + assign5200_e7066);
        var_dtats_i = assign5200_e7067;
        var_dtats_i_rv = 0.0;

        let assign5210_e7071: f64 = (var_inv_l * p.p1445);
        let assign5210_e7072: f64 = (p.p1444 + assign5210_e7071);
        let assign5210_e7075: f64 = (var_inv_nfin * p.p1446);
        let assign5210_e7076: f64 = (assign5210_e7072 + assign5210_e7075);
        let assign5210_e7079: f64 = (var_inv_lnfin * p.p1447);
        let assign5210_e7080: f64 = (assign5210_e7076 + assign5210_e7079);
        let assign5210_e7083: f64 = (var_inv_w * p.p1448);
        let assign5210_e7084: f64 = (assign5210_e7080 + assign5210_e7083);
        let assign5210_e7087: f64 = (var_inv_wl * p.p1449);
        let assign5210_e7088: f64 = (assign5210_e7084 + assign5210_e7087);
        var_alpha0_i = assign5210_e7088;
        var_alpha0_i_rv = 0.0;

        let assign5220_e7092: f64 = (var_inv_l * p.p1451);
        let assign5220_e7093: f64 = (p.p1450 + assign5220_e7092);
        let assign5220_e7096: f64 = (var_inv_nfin * p.p1452);
        let assign5220_e7097: f64 = (assign5220_e7093 + assign5220_e7096);
        let assign5220_e7100: f64 = (var_inv_lnfin * p.p1453);
        let assign5220_e7101: f64 = (assign5220_e7097 + assign5220_e7100);
        let assign5220_e7104: f64 = (var_inv_w * p.p1454);
        let assign5220_e7105: f64 = (assign5220_e7101 + assign5220_e7104);
        let assign5220_e7108: f64 = (var_inv_wl * p.p1455);
        let assign5220_e7109: f64 = (assign5220_e7105 + assign5220_e7108);
        var_alpha1_i = assign5220_e7109;
        var_alpha1_i_rv = 0.0;

        let assign5230_e7113: f64 = (var_inv_l * p.p1463);
        let assign5230_e7114: f64 = (p.p1462 + assign5230_e7113);
        let assign5230_e7117: f64 = (var_inv_nfin * p.p1464);
        let assign5230_e7118: f64 = (assign5230_e7114 + assign5230_e7117);
        let assign5230_e7121: f64 = (var_inv_lnfin * p.p1465);
        let assign5230_e7122: f64 = (assign5230_e7118 + assign5230_e7121);
        let assign5230_e7125: f64 = (var_inv_w * p.p1466);
        let assign5230_e7126: f64 = (assign5230_e7122 + assign5230_e7125);
        let assign5230_e7129: f64 = (var_inv_wl * p.p1467);
        let assign5230_e7130: f64 = (assign5230_e7126 + assign5230_e7129);
        var_alphaii0_i = assign5230_e7130;
        var_alphaii0_i_rv = 0.0;

        let assign5240_e7134: f64 = (var_inv_l * p.p1469);
        let assign5240_e7135: f64 = (p.p1468 + assign5240_e7134);
        let assign5240_e7138: f64 = (var_inv_nfin * p.p1470);
        let assign5240_e7139: f64 = (assign5240_e7135 + assign5240_e7138);
        let assign5240_e7142: f64 = (var_inv_lnfin * p.p1471);
        let assign5240_e7143: f64 = (assign5240_e7139 + assign5240_e7142);
        let assign5240_e7146: f64 = (var_inv_w * p.p1472);
        let assign5240_e7147: f64 = (assign5240_e7143 + assign5240_e7146);
        let assign5240_e7150: f64 = (var_inv_wl * p.p1473);
        let assign5240_e7151: f64 = (assign5240_e7147 + assign5240_e7150);
        var_alphaii1_i = assign5240_e7151;
        var_alphaii1_i_rv = 0.0;

        let assign5250_e7155: f64 = (var_inv_l * p.p1457);
        let assign5250_e7156: f64 = (p.p1456 + assign5250_e7155);
        let assign5250_e7159: f64 = (var_inv_nfin * p.p1458);
        let assign5250_e7160: f64 = (assign5250_e7156 + assign5250_e7159);
        let assign5250_e7163: f64 = (var_inv_lnfin * p.p1459);
        let assign5250_e7164: f64 = (assign5250_e7160 + assign5250_e7163);
        let assign5250_e7167: f64 = (var_inv_w * p.p1460);
        let assign5250_e7168: f64 = (assign5250_e7164 + assign5250_e7167);
        let assign5250_e7171: f64 = (var_inv_wl * p.p1461);
        let assign5250_e7172: f64 = (assign5250_e7168 + assign5250_e7171);
        var_beta0_i = assign5250_e7172;
        var_beta0_i_rv = 0.0;

        let assign5260_e7176: f64 = (var_inv_l * p.p1475);
        let assign5260_e7177: f64 = (p.p1474 + assign5260_e7176);
        let assign5260_e7180: f64 = (var_inv_nfin * p.p1476);
        let assign5260_e7181: f64 = (assign5260_e7177 + assign5260_e7180);
        let assign5260_e7184: f64 = (var_inv_lnfin * p.p1477);
        let assign5260_e7185: f64 = (assign5260_e7181 + assign5260_e7184);
        let assign5260_e7188: f64 = (var_inv_w * p.p1478);
        let assign5260_e7189: f64 = (assign5260_e7185 + assign5260_e7188);
        let assign5260_e7192: f64 = (var_inv_wl * p.p1479);
        let assign5260_e7193: f64 = (assign5260_e7189 + assign5260_e7192);
        var_betaii0_i = assign5260_e7193;
        var_betaii0_i_rv = 0.0;

        let assign5270_e7197: f64 = (var_inv_l * p.p1481);
        let assign5270_e7198: f64 = (p.p1480 + assign5270_e7197);
        let assign5270_e7201: f64 = (var_inv_nfin * p.p1482);
        let assign5270_e7202: f64 = (assign5270_e7198 + assign5270_e7201);
        let assign5270_e7205: f64 = (var_inv_lnfin * p.p1483);
        let assign5270_e7206: f64 = (assign5270_e7202 + assign5270_e7205);
        let assign5270_e7209: f64 = (var_inv_w * p.p1484);
        let assign5270_e7210: f64 = (assign5270_e7206 + assign5270_e7209);
        let assign5270_e7213: f64 = (var_inv_wl * p.p1485);
        let assign5270_e7214: f64 = (assign5270_e7210 + assign5270_e7213);
        var_betaii1_i = assign5270_e7214;
        var_betaii1_i_rv = 0.0;

        let assign5280_e7218: f64 = (var_inv_l * p.p1487);
        let assign5280_e7219: f64 = (p.p1486 + assign5280_e7218);
        let assign5280_e7222: f64 = (var_inv_nfin * p.p1488);
        let assign5280_e7223: f64 = (assign5280_e7219 + assign5280_e7222);
        let assign5280_e7226: f64 = (var_inv_lnfin * p.p1489);
        let assign5280_e7227: f64 = (assign5280_e7223 + assign5280_e7226);
        let assign5280_e7230: f64 = (var_inv_w * p.p1490);
        let assign5280_e7231: f64 = (assign5280_e7227 + assign5280_e7230);
        let assign5280_e7234: f64 = (var_inv_wl * p.p1491);
        let assign5280_e7235: f64 = (assign5280_e7231 + assign5280_e7234);
        var_betaii2_i = assign5280_e7235;
        var_betaii2_i_rv = 0.0;

        let assign5290_e7239: f64 = (var_inv_l * p.p1493);
        let assign5290_e7240: f64 = (p.p1492 + assign5290_e7239);
        let assign5290_e7243: f64 = (var_inv_nfin * p.p1494);
        let assign5290_e7244: f64 = (assign5290_e7240 + assign5290_e7243);
        let assign5290_e7247: f64 = (var_inv_lnfin * p.p1495);
        let assign5290_e7248: f64 = (assign5290_e7244 + assign5290_e7247);
        let assign5290_e7251: f64 = (var_inv_w * p.p1496);
        let assign5290_e7252: f64 = (assign5290_e7248 + assign5290_e7251);
        let assign5290_e7255: f64 = (var_inv_wl * p.p1497);
        let assign5290_e7256: f64 = (assign5290_e7252 + assign5290_e7255);
        var_esatii_i = assign5290_e7256;
        var_esatii_i_rv = 0.0;

        let assign5300_e7260: f64 = (var_inv_l * p.p1499);
        let assign5300_e7261: f64 = (p.p1498 + assign5300_e7260);
        let assign5300_e7264: f64 = (var_inv_nfin * p.p1500);
        let assign5300_e7265: f64 = (assign5300_e7261 + assign5300_e7264);
        let assign5300_e7268: f64 = (var_inv_lnfin * p.p1501);
        let assign5300_e7269: f64 = (assign5300_e7265 + assign5300_e7268);
        let assign5300_e7272: f64 = (var_inv_w * p.p1502);
        let assign5300_e7273: f64 = (assign5300_e7269 + assign5300_e7272);
        let assign5300_e7276: f64 = (var_inv_wl * p.p1503);
        let assign5300_e7277: f64 = (assign5300_e7273 + assign5300_e7276);
        var_lii_i = assign5300_e7277;
        var_lii_i_rv = 0.0;

        let assign5310_e7281: f64 = (var_inv_l * p.p1505);
        let assign5310_e7282: f64 = (p.p1504 + assign5310_e7281);
        let assign5310_e7285: f64 = (var_inv_nfin * p.p1506);
        let assign5310_e7286: f64 = (assign5310_e7282 + assign5310_e7285);
        let assign5310_e7289: f64 = (var_inv_lnfin * p.p1507);
        let assign5310_e7290: f64 = (assign5310_e7286 + assign5310_e7289);
        let assign5310_e7293: f64 = (var_inv_w * p.p1508);
        let assign5310_e7294: f64 = (assign5310_e7290 + assign5310_e7293);
        let assign5310_e7297: f64 = (var_inv_wl * p.p1509);
        let assign5310_e7298: f64 = (assign5310_e7294 + assign5310_e7297);
        var_sii0_i = assign5310_e7298;
        var_sii0_i_rv = 0.0;

        let assign5320_e7302: f64 = (var_inv_l * p.p1511);
        let assign5320_e7303: f64 = (p.p1510 + assign5320_e7302);
        let assign5320_e7306: f64 = (var_inv_nfin * p.p1512);
        let assign5320_e7307: f64 = (assign5320_e7303 + assign5320_e7306);
        let assign5320_e7310: f64 = (var_inv_lnfin * p.p1513);
        let assign5320_e7311: f64 = (assign5320_e7307 + assign5320_e7310);
        let assign5320_e7314: f64 = (var_inv_w * p.p1514);
        let assign5320_e7315: f64 = (assign5320_e7311 + assign5320_e7314);
        let assign5320_e7318: f64 = (var_inv_wl * p.p1515);
        let assign5320_e7319: f64 = (assign5320_e7315 + assign5320_e7318);
        var_sii1_i = assign5320_e7319;
        var_sii1_i_rv = 0.0;

        let assign5330_e7323: f64 = (var_inv_l * p.p1517);
        let assign5330_e7324: f64 = (p.p1516 + assign5330_e7323);
        let assign5330_e7327: f64 = (var_inv_nfin * p.p1518);
        let assign5330_e7328: f64 = (assign5330_e7324 + assign5330_e7327);
        let assign5330_e7331: f64 = (var_inv_lnfin * p.p1519);
        let assign5330_e7332: f64 = (assign5330_e7328 + assign5330_e7331);
        let assign5330_e7335: f64 = (var_inv_w * p.p1520);
        let assign5330_e7336: f64 = (assign5330_e7332 + assign5330_e7335);
        let assign5330_e7339: f64 = (var_inv_wl * p.p1521);
        let assign5330_e7340: f64 = (assign5330_e7336 + assign5330_e7339);
        var_sii2_i = assign5330_e7340;
        var_sii2_i_rv = 0.0;

        let assign5340_e7344: f64 = (var_inv_l * p.p1523);
        let assign5340_e7345: f64 = (p.p1522 + assign5340_e7344);
        let assign5340_e7348: f64 = (var_inv_nfin * p.p1524);
        let assign5340_e7349: f64 = (assign5340_e7345 + assign5340_e7348);
        let assign5340_e7352: f64 = (var_inv_lnfin * p.p1525);
        let assign5340_e7353: f64 = (assign5340_e7349 + assign5340_e7352);
        let assign5340_e7356: f64 = (var_inv_w * p.p1526);
        let assign5340_e7357: f64 = (assign5340_e7353 + assign5340_e7356);
        let assign5340_e7360: f64 = (var_inv_wl * p.p1527);
        let assign5340_e7361: f64 = (assign5340_e7357 + assign5340_e7360);
        var_siid_i = assign5340_e7361;
        var_siid_i_rv = 0.0;

        let assign5350_e7365: f64 = (var_inv_l * p.p1763);
        let assign5350_e7366: f64 = (p.p1762 + assign5350_e7365);
        let assign5350_e7369: f64 = (var_inv_nfin * p.p1764);
        let assign5350_e7370: f64 = (assign5350_e7366 + assign5350_e7369);
        let assign5350_e7373: f64 = (var_inv_lnfin * p.p1765);
        let assign5350_e7374: f64 = (assign5350_e7370 + assign5350_e7373);
        let assign5350_e7377: f64 = (var_inv_w * p.p1766);
        let assign5350_e7378: f64 = (assign5350_e7374 + assign5350_e7377);
        let assign5350_e7381: f64 = (var_inv_wl * p.p1767);
        let assign5350_e7382: f64 = (assign5350_e7378 + assign5350_e7381);
        var_tii_i = assign5350_e7382;
        var_tii_i_rv = 0.0;

        let assign5360_e7386: f64 = (var_inv_l * p.p1531);
        let assign5360_e7387: f64 = (p.p1530 + assign5360_e7386);
        let assign5360_e7390: f64 = (var_inv_nfin * p.p1532);
        let assign5360_e7391: f64 = (assign5360_e7387 + assign5360_e7390);
        let assign5360_e7394: f64 = (var_inv_lnfin * p.p1533);
        let assign5360_e7395: f64 = (assign5360_e7391 + assign5360_e7394);
        let assign5360_e7398: f64 = (var_inv_w * p.p1534);
        let assign5360_e7399: f64 = (assign5360_e7395 + assign5360_e7398);
        let assign5360_e7402: f64 = (var_inv_wl * p.p1535);
        let assign5360_e7403: f64 = (assign5360_e7399 + assign5360_e7402);
        var_cfs_i = assign5360_e7403;
        var_cfs_i_rv = 0.0;

        let assign5370_e7407: f64 = (var_inv_l * p.p1537);
        let assign5370_e7408: f64 = (p.p1536 + assign5370_e7407);
        let assign5370_e7411: f64 = (var_inv_nfin * p.p1538);
        let assign5370_e7412: f64 = (assign5370_e7408 + assign5370_e7411);
        let assign5370_e7415: f64 = (var_inv_lnfin * p.p1539);
        let assign5370_e7416: f64 = (assign5370_e7412 + assign5370_e7415);
        let assign5370_e7419: f64 = (var_inv_w * p.p1540);
        let assign5370_e7420: f64 = (assign5370_e7416 + assign5370_e7419);
        let assign5370_e7423: f64 = (var_inv_wl * p.p1541);
        let assign5370_e7424: f64 = (assign5370_e7420 + assign5370_e7423);
        var_cfd_i = assign5370_e7424;
        var_cfd_i_rv = 0.0;

        let assign5380_e7428: f64 = (var_inv_l * p.p29);
        let assign5380_e7429: f64 = (p.p28 + assign5380_e7428);
        let assign5380_e7432: f64 = (var_inv_nfin * p.p30);
        let assign5380_e7433: f64 = (assign5380_e7429 + assign5380_e7432);
        let assign5380_e7436: f64 = (var_inv_lnfin * p.p31);
        let assign5380_e7437: f64 = (assign5380_e7433 + assign5380_e7436);
        let assign5380_e7440: f64 = (var_inv_w * p.p32);
        let assign5380_e7441: f64 = (assign5380_e7437 + assign5380_e7440);
        let assign5380_e7444: f64 = (var_inv_wl * p.p33);
        let assign5380_e7445: f64 = (assign5380_e7441 + assign5380_e7444);
        var_covs_i = assign5380_e7445;
        var_covs_i_dn0 = 0.0;
        var_covs_i_dn2 = 0.0;
        var_covs_i_dn3 = 0.0;
        var_covs_i_dn4 = 0.0;
        var_covs_i_dn5 = 0.0;
        var_covs_i_dn6 = 0.0;
        var_covs_i_dn7 = 0.0;
        var_covs_i_dn8 = 0.0;
        var_covs_i_dn9 = 0.0;
        var_covs_i_dn10 = 0.0;
        var_covs_i_dn11 = 0.0;
        var_covs_i_dn13 = 0.0;
        var_covs_i_dn14 = 0.0;
        var_covs_i_rv = 0.0;

        let assign5390_e7449: f64 = (var_inv_l * p.p35);
        let assign5390_e7450: f64 = (p.p34 + assign5390_e7449);
        let assign5390_e7453: f64 = (var_inv_nfin * p.p36);
        let assign5390_e7454: f64 = (assign5390_e7450 + assign5390_e7453);
        let assign5390_e7457: f64 = (var_inv_lnfin * p.p37);
        let assign5390_e7458: f64 = (assign5390_e7454 + assign5390_e7457);
        let assign5390_e7461: f64 = (var_inv_w * p.p38);
        let assign5390_e7462: f64 = (assign5390_e7458 + assign5390_e7461);
        let assign5390_e7465: f64 = (var_inv_wl * p.p39);
        let assign5390_e7466: f64 = (assign5390_e7462 + assign5390_e7465);
        var_covd_i = assign5390_e7466;
        var_covd_i_dn0 = 0.0;
        var_covd_i_dn2 = 0.0;
        var_covd_i_dn3 = 0.0;
        var_covd_i_dn4 = 0.0;
        var_covd_i_dn5 = 0.0;
        var_covd_i_dn6 = 0.0;
        var_covd_i_dn7 = 0.0;
        var_covd_i_dn8 = 0.0;
        var_covd_i_dn9 = 0.0;
        var_covd_i_dn10 = 0.0;
        var_covd_i_dn11 = 0.0;
        var_covd_i_dn13 = 0.0;
        var_covd_i_dn14 = 0.0;
        var_covd_i_rv = 0.0;

        let assign5400_e7470: f64 = (var_inv_l * p.p1548);
        let assign5400_e7471: f64 = (p.p1547 + assign5400_e7470);
        let assign5400_e7474: f64 = (var_inv_nfin * p.p1549);
        let assign5400_e7475: f64 = (assign5400_e7471 + assign5400_e7474);
        let assign5400_e7478: f64 = (var_inv_lnfin * p.p1550);
        let assign5400_e7479: f64 = (assign5400_e7475 + assign5400_e7478);
        let assign5400_e7482: f64 = (var_inv_w * p.p1551);
        let assign5400_e7483: f64 = (assign5400_e7479 + assign5400_e7482);
        let assign5400_e7486: f64 = (var_inv_wl * p.p1552);
        let assign5400_e7487: f64 = (assign5400_e7483 + assign5400_e7486);
        var_cgsl_i = assign5400_e7487;
        var_cgsl_i_rv = 0.0;

        let assign5410_e7491: f64 = (var_inv_l * p.p1554);
        let assign5410_e7492: f64 = (p.p1553 + assign5410_e7491);
        let assign5410_e7495: f64 = (var_inv_nfin * p.p1555);
        let assign5410_e7496: f64 = (assign5410_e7492 + assign5410_e7495);
        let assign5410_e7499: f64 = (var_inv_lnfin * p.p1556);
        let assign5410_e7500: f64 = (assign5410_e7496 + assign5410_e7499);
        let assign5410_e7503: f64 = (var_inv_w * p.p1557);
        let assign5410_e7504: f64 = (assign5410_e7500 + assign5410_e7503);
        let assign5410_e7507: f64 = (var_inv_wl * p.p1558);
        let assign5410_e7508: f64 = (assign5410_e7504 + assign5410_e7507);
        var_cgdl_i = assign5410_e7508;
        var_cgdl_i_rv = 0.0;

        let assign5420_e7512: f64 = (var_inv_l * p.p1560);
        let assign5420_e7513: f64 = (p.p1559 + assign5420_e7512);
        let assign5420_e7516: f64 = (var_inv_nfin * p.p1561);
        let assign5420_e7517: f64 = (assign5420_e7513 + assign5420_e7516);
        let assign5420_e7520: f64 = (var_inv_lnfin * p.p1562);
        let assign5420_e7521: f64 = (assign5420_e7517 + assign5420_e7520);
        let assign5420_e7524: f64 = (var_inv_w * p.p1563);
        let assign5420_e7525: f64 = (assign5420_e7521 + assign5420_e7524);
        let assign5420_e7528: f64 = (var_inv_wl * p.p1564);
        let assign5420_e7529: f64 = (assign5420_e7525 + assign5420_e7528);
        var_cgbl_i = assign5420_e7529;
        var_cgbl_i_rv = 0.0;

        let assign5430_e7533: f64 = (var_inv_l * p.p1566);
        let assign5430_e7534: f64 = (p.p1565 + assign5430_e7533);
        let assign5430_e7537: f64 = (var_inv_nfin * p.p1567);
        let assign5430_e7538: f64 = (assign5430_e7534 + assign5430_e7537);
        let assign5430_e7541: f64 = (var_inv_lnfin * p.p1568);
        let assign5430_e7542: f64 = (assign5430_e7538 + assign5430_e7541);
        let assign5430_e7545: f64 = (var_inv_w * p.p1569);
        let assign5430_e7546: f64 = (assign5430_e7542 + assign5430_e7545);
        let assign5430_e7549: f64 = (var_inv_wl * p.p1570);
        let assign5430_e7550: f64 = (assign5430_e7546 + assign5430_e7549);
        var_ckappas_i = assign5430_e7550;
        var_ckappas_i_rv = 0.0;

        let assign5440_e7554: f64 = (var_inv_l * p.p1572);
        let assign5440_e7555: f64 = (p.p1571 + assign5440_e7554);
        let assign5440_e7558: f64 = (var_inv_nfin * p.p1573);
        let assign5440_e7559: f64 = (assign5440_e7555 + assign5440_e7558);
        let assign5440_e7562: f64 = (var_inv_lnfin * p.p1574);
        let assign5440_e7563: f64 = (assign5440_e7559 + assign5440_e7562);
        let assign5440_e7566: f64 = (var_inv_w * p.p1575);
        let assign5440_e7567: f64 = (assign5440_e7563 + assign5440_e7566);
        let assign5440_e7570: f64 = (var_inv_wl * p.p1576);
        let assign5440_e7571: f64 = (assign5440_e7567 + assign5440_e7570);
        var_ckappad_i = assign5440_e7571;
        var_ckappad_i_rv = 0.0;

        let assign5450_e7575: f64 = (var_inv_l * p.p1578);
        let assign5450_e7576: f64 = (p.p1577 + assign5450_e7575);
        let assign5450_e7579: f64 = (var_inv_nfin * p.p1579);
        let assign5450_e7580: f64 = (assign5450_e7576 + assign5450_e7579);
        let assign5450_e7583: f64 = (var_inv_lnfin * p.p1580);
        let assign5450_e7584: f64 = (assign5450_e7580 + assign5450_e7583);
        let assign5450_e7587: f64 = (var_inv_w * p.p1581);
        let assign5450_e7588: f64 = (assign5450_e7584 + assign5450_e7587);
        let assign5450_e7591: f64 = (var_inv_wl * p.p1582);
        let assign5450_e7592: f64 = (assign5450_e7588 + assign5450_e7591);
        var_ckappab_i = assign5450_e7592;
        var_ckappab_i_rv = 0.0;

        let assign5470_e7617: f64 = (var_inv_l * p.p1657);
        let assign5470_e7618: f64 = (p.p1656 + assign5470_e7617);
        let assign5470_e7621: f64 = (var_inv_nfin * p.p1658);
        let assign5470_e7622: f64 = (assign5470_e7618 + assign5470_e7621);
        let assign5470_e7625: f64 = (var_inv_lnfin * p.p1659);
        let assign5470_e7626: f64 = (assign5470_e7622 + assign5470_e7625);
        let assign5470_e7629: f64 = (var_inv_w * p.p1660);
        let assign5470_e7630: f64 = (assign5470_e7626 + assign5470_e7629);
        let assign5470_e7633: f64 = (var_inv_wl * p.p1661);
        let assign5470_e7634: f64 = (assign5470_e7630 + assign5470_e7633);
        var_aigen_i = assign5470_e7634;
        var_aigen_i_rv = 0.0;

        let assign5480_e7638: f64 = (var_inv_l * p.p1663);
        let assign5480_e7639: f64 = (p.p1662 + assign5480_e7638);
        let assign5480_e7642: f64 = (var_inv_nfin * p.p1664);
        let assign5480_e7643: f64 = (assign5480_e7639 + assign5480_e7642);
        let assign5480_e7646: f64 = (var_inv_lnfin * p.p1665);
        let assign5480_e7647: f64 = (assign5480_e7643 + assign5480_e7646);
        let assign5480_e7650: f64 = (var_inv_w * p.p1666);
        let assign5480_e7651: f64 = (assign5480_e7647 + assign5480_e7650);
        let assign5480_e7654: f64 = (var_inv_wl * p.p1667);
        let assign5480_e7655: f64 = (assign5480_e7651 + assign5480_e7654);
        var_bigen_i = assign5480_e7655;
        var_bigen_i_rv = 0.0;

        let assign5490_e7659: f64 = (var_inv_l * p.p738);
        let assign5490_e7660: f64 = (p.p737 + assign5490_e7659);
        let assign5490_e7663: f64 = (var_inv_nfin * p.p739);
        let assign5490_e7664: f64 = (assign5490_e7660 + assign5490_e7663);
        let assign5490_e7667: f64 = (var_inv_lnfin * p.p740);
        let assign5490_e7668: f64 = (assign5490_e7664 + assign5490_e7667);
        let assign5490_e7671: f64 = (var_inv_w * p.p741);
        let assign5490_e7672: f64 = (assign5490_e7668 + assign5490_e7671);
        let assign5490_e7675: f64 = (var_inv_wl * p.p742);
        let assign5490_e7676: f64 = (assign5490_e7672 + assign5490_e7675);
        var_ute_i = assign5490_e7676;
        var_ute_i_rv = 0.0;

        let assign5500_e7680: f64 = (var_inv_l * p.p756);
        let assign5500_e7681: f64 = (p.p755 + assign5500_e7680);
        let assign5500_e7684: f64 = (var_inv_nfin * p.p757);
        let assign5500_e7685: f64 = (assign5500_e7681 + assign5500_e7684);
        let assign5500_e7688: f64 = (var_inv_lnfin * p.p758);
        let assign5500_e7689: f64 = (assign5500_e7685 + assign5500_e7688);
        let assign5500_e7692: f64 = (var_inv_w * p.p759);
        let assign5500_e7693: f64 = (assign5500_e7689 + assign5500_e7692);
        let assign5500_e7696: f64 = (var_inv_wl * p.p760);
        let assign5500_e7697: f64 = (assign5500_e7693 + assign5500_e7696);
        var_ute1_i = assign5500_e7697;
        var_ute1_i_rv = 0.0;

        *var_aigen_i_slot = var_aigen_i;
        *var_aigen_i_rv_slot = var_aigen_i_rv;
        *var_alpha0_i_slot = var_alpha0_i;
        *var_alpha0_i_rv_slot = var_alpha0_i_rv;
        *var_alpha1_i_slot = var_alpha1_i;
        *var_alpha1_i_rv_slot = var_alpha1_i_rv;
        *var_alphaii0_i_slot = var_alphaii0_i;
        *var_alphaii0_i_rv_slot = var_alphaii0_i_rv;
        *var_alphaii1_i_slot = var_alphaii1_i;
        *var_alphaii1_i_rv_slot = var_alphaii1_i_rv;
        *var_atats_i_slot = var_atats_i;
        *var_atats_i_rv_slot = var_atats_i_rv;
        *var_beta0_i_slot = var_beta0_i;
        *var_beta0_i_rv_slot = var_beta0_i_rv;
        *var_betaii0_i_slot = var_betaii0_i;
        *var_betaii0_i_rv_slot = var_betaii0_i_rv;
        *var_betaii1_i_slot = var_betaii1_i;
        *var_betaii1_i_rv_slot = var_betaii1_i_rv;
        *var_betaii2_i_slot = var_betaii2_i;
        *var_betaii2_i_rv_slot = var_betaii2_i_rv;
        *var_bgisl_i_slot = var_bgisl_i;
        *var_bgisl_i_rv_slot = var_bgisl_i_rv;
        *var_bigen_i_slot = var_bigen_i;
        *var_bigen_i_rv_slot = var_bigen_i_rv;
        *var_btats_i_slot = var_btats_i;
        *var_btats_i_rv_slot = var_btats_i_rv;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfd_i_rv_slot = var_cfd_i_rv;
        *var_cfs_i_slot = var_cfs_i;
        *var_cfs_i_rv_slot = var_cfs_i_rv;
        *var_cgbl_i_slot = var_cgbl_i;
        *var_cgbl_i_rv_slot = var_cgbl_i_rv;
        *var_cgdl_i_slot = var_cgdl_i;
        *var_cgdl_i_rv_slot = var_cgdl_i_rv;
        *var_cgisl_i_slot = var_cgisl_i;
        *var_cgisl_i_rv_slot = var_cgisl_i_rv;
        *var_cgsl_i_slot = var_cgsl_i;
        *var_cgsl_i_rv_slot = var_cgsl_i_rv;
        *var_ckappab_i_slot = var_ckappab_i;
        *var_ckappab_i_rv_slot = var_ckappab_i_rv;
        *var_ckappad_i_slot = var_ckappad_i;
        *var_ckappad_i_rv_slot = var_ckappad_i_rv;
        *var_ckappas_i_slot = var_ckappas_i;
        *var_ckappas_i_rv_slot = var_ckappas_i_rv;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn0_slot = var_covd_i_dn0;
        *var_covd_i_dn10_slot = var_covd_i_dn10;
        *var_covd_i_dn11_slot = var_covd_i_dn11;
        *var_covd_i_dn13_slot = var_covd_i_dn13;
        *var_covd_i_dn14_slot = var_covd_i_dn14;
        *var_covd_i_dn2_slot = var_covd_i_dn2;
        *var_covd_i_dn3_slot = var_covd_i_dn3;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn5_slot = var_covd_i_dn5;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covd_i_rv_slot = var_covd_i_rv;
        *var_covs_i_slot = var_covs_i;
        *var_covs_i_dn0_slot = var_covs_i_dn0;
        *var_covs_i_dn10_slot = var_covs_i_dn10;
        *var_covs_i_dn11_slot = var_covs_i_dn11;
        *var_covs_i_dn13_slot = var_covs_i_dn13;
        *var_covs_i_dn14_slot = var_covs_i_dn14;
        *var_covs_i_dn2_slot = var_covs_i_dn2;
        *var_covs_i_dn3_slot = var_covs_i_dn3;
        *var_covs_i_dn4_slot = var_covs_i_dn4;
        *var_covs_i_dn5_slot = var_covs_i_dn5;
        *var_covs_i_dn6_slot = var_covs_i_dn6;
        *var_covs_i_dn7_slot = var_covs_i_dn7;
        *var_covs_i_dn8_slot = var_covs_i_dn8;
        *var_covs_i_dn9_slot = var_covs_i_dn9;
        *var_covs_i_rv_slot = var_covs_i_rv;
        *var_ctats_i_slot = var_ctats_i;
        *var_ctats_i_rv_slot = var_ctats_i_rv;
        *var_dtats_i_slot = var_dtats_i;
        *var_dtats_i_rv_slot = var_dtats_i_rv;
        *var_egisl_i_slot = var_egisl_i;
        *var_egisl_i_rv_slot = var_egisl_i_rv;
        *var_esatii_i_slot = var_esatii_i;
        *var_esatii_i_rv_slot = var_esatii_i_rv;
        *var_lii_i_slot = var_lii_i;
        *var_lii_i_rv_slot = var_lii_i_rv;
        *var_pgisl_i_slot = var_pgisl_i;
        *var_pgisl_i_rv_slot = var_pgisl_i_rv;
        *var_sii0_i_slot = var_sii0_i;
        *var_sii0_i_rv_slot = var_sii0_i_rv;
        *var_sii1_i_slot = var_sii1_i;
        *var_sii1_i_rv_slot = var_sii1_i_rv;
        *var_sii2_i_slot = var_sii2_i;
        *var_sii2_i_rv_slot = var_sii2_i_rv;
        *var_siid_i_slot = var_siid_i;
        *var_siid_i_rv_slot = var_siid_i_rv;
        *var_tii_i_slot = var_tii_i;
        *var_tii_i_rv_slot = var_tii_i_rv;
        *var_ute1_i_slot = var_ute1_i;
        *var_ute1_i_rv_slot = var_ute1_i_rv;
        *var_ute_i_slot = var_ute_i;
        *var_ute_i_rv_slot = var_ute_i_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_at_i_slot: &mut f64,
        var_at_i_rv_slot: &mut f64,
        var_atcv_i_slot: &mut f64,
        var_atcv_i_rv_slot: &mut f64,
        var_dvtp0_i_slot: &mut f64,
        var_dvtp0_i_dn0_slot: &mut f64,
        var_dvtp0_i_dn10_slot: &mut f64,
        var_dvtp0_i_dn11_slot: &mut f64,
        var_dvtp0_i_dn13_slot: &mut f64,
        var_dvtp0_i_dn14_slot: &mut f64,
        var_dvtp0_i_dn2_slot: &mut f64,
        var_dvtp0_i_dn3_slot: &mut f64,
        var_dvtp0_i_dn4_slot: &mut f64,
        var_dvtp0_i_dn5_slot: &mut f64,
        var_dvtp0_i_dn6_slot: &mut f64,
        var_dvtp0_i_dn7_slot: &mut f64,
        var_dvtp0_i_dn8_slot: &mut f64,
        var_dvtp0_i_dn9_slot: &mut f64,
        var_dvtp0_i_rv_slot: &mut f64,
        var_dvtp1_i_slot: &mut f64,
        var_dvtp1_i_dn0_slot: &mut f64,
        var_dvtp1_i_dn10_slot: &mut f64,
        var_dvtp1_i_dn11_slot: &mut f64,
        var_dvtp1_i_dn13_slot: &mut f64,
        var_dvtp1_i_dn14_slot: &mut f64,
        var_dvtp1_i_dn2_slot: &mut f64,
        var_dvtp1_i_dn3_slot: &mut f64,
        var_dvtp1_i_dn4_slot: &mut f64,
        var_dvtp1_i_dn5_slot: &mut f64,
        var_dvtp1_i_dn6_slot: &mut f64,
        var_dvtp1_i_dn7_slot: &mut f64,
        var_dvtp1_i_dn8_slot: &mut f64,
        var_dvtp1_i_dn9_slot: &mut f64,
        var_dvtp1_i_rv_slot: &mut f64,
        var_emobt_i_slot: &mut f64,
        var_emobt_i_rv_slot: &mut f64,
        var_eu1_i_slot: &mut f64,
        var_eu1_i_rv_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard42_rv_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard43_rv_slot: &mut f64,
        var_iit_i_slot: &mut f64,
        var_iit_i_rv_slot: &mut f64,
        var_k11_i_slot: &mut f64,
        var_k11_i_rv_slot: &mut f64,
        var_k1_i_slot: &mut f64,
        var_k1_i_rv_slot: &mut f64,
        var_k21_i_slot: &mut f64,
        var_k21_i_rv_slot: &mut f64,
        var_k2_i_slot: &mut f64,
        var_k2_i_rv_slot: &mut f64,
        var_kt1_i_slot: &mut f64,
        var_kt1_i_rv_slot: &mut f64,
        var_mpower_i_slot: &mut f64,
        var_mpower_i_rv_slot: &mut f64,
        var_noia2_i_slot: &mut f64,
        var_noia2_i_rv_slot: &mut f64,
        var_phibe_i_slot: &mut f64,
        var_phibe_i_rv_slot: &mut f64,
        var_prt1_i_slot: &mut f64,
        var_prt1_i_rv_slot: &mut f64,
        var_prt_i_slot: &mut f64,
        var_prt_i_rv_slot: &mut f64,
        var_ptwgt_i_slot: &mut f64,
        var_ptwgt_i_rv_slot: &mut f64,
        var_qsref_i_slot: &mut f64,
        var_qsref_i_rv_slot: &mut f64,
        var_sprt_i_slot: &mut f64,
        var_sprt_i_rv_slot: &mut f64,
        var_tgidl_i_slot: &mut f64,
        var_tgidl_i_rv_slot: &mut f64,
        var_tr0_i_slot: &mut f64,
        var_tr0_i_rv_slot: &mut f64,
        var_tss_i_slot: &mut f64,
        var_tss_i_rv_slot: &mut f64,
        var_ttat_i_slot: &mut f64,
        var_ttat_i_rv_slot: &mut f64,
        var_ua1_i_slot: &mut f64,
        var_ua1_i_rv_slot: &mut f64,
        var_ua2_i_slot: &mut f64,
        var_ua2_i_rv_slot: &mut f64,
        var_uc1_i_slot: &mut f64,
        var_uc1_i_rv_slot: &mut f64,
        var_uc_i_slot: &mut f64,
        var_uc_i_rv_slot: &mut f64,
        var_ucste1_i_slot: &mut f64,
        var_ucste1_i_rv_slot: &mut f64,
        var_ucste_i_slot: &mut f64,
        var_ucste_i_rv_slot: &mut f64,
        var_ud1_i_slot: &mut f64,
        var_ud1_i_rv_slot: &mut f64,
        var_ud2_i_slot: &mut f64,
        var_ud2_i_rv_slot: &mut f64,
        var_utl_i_slot: &mut f64,
        var_utl_i_rv_slot: &mut f64,
    ) {
        let mut var_at_i: f64 = *var_at_i_slot;
        let mut var_at_i_rv: f64 = *var_at_i_rv_slot;
        let mut var_atcv_i: f64 = *var_atcv_i_slot;
        let mut var_atcv_i_rv: f64 = *var_atcv_i_rv_slot;
        let mut var_dvtp0_i: f64 = *var_dvtp0_i_slot;
        let mut var_dvtp0_i_dn0: f64 = *var_dvtp0_i_dn0_slot;
        let mut var_dvtp0_i_dn10: f64 = *var_dvtp0_i_dn10_slot;
        let mut var_dvtp0_i_dn11: f64 = *var_dvtp0_i_dn11_slot;
        let mut var_dvtp0_i_dn13: f64 = *var_dvtp0_i_dn13_slot;
        let mut var_dvtp0_i_dn14: f64 = *var_dvtp0_i_dn14_slot;
        let mut var_dvtp0_i_dn2: f64 = *var_dvtp0_i_dn2_slot;
        let mut var_dvtp0_i_dn3: f64 = *var_dvtp0_i_dn3_slot;
        let mut var_dvtp0_i_dn4: f64 = *var_dvtp0_i_dn4_slot;
        let mut var_dvtp0_i_dn5: f64 = *var_dvtp0_i_dn5_slot;
        let mut var_dvtp0_i_dn6: f64 = *var_dvtp0_i_dn6_slot;
        let mut var_dvtp0_i_dn7: f64 = *var_dvtp0_i_dn7_slot;
        let mut var_dvtp0_i_dn8: f64 = *var_dvtp0_i_dn8_slot;
        let mut var_dvtp0_i_dn9: f64 = *var_dvtp0_i_dn9_slot;
        let mut var_dvtp0_i_rv: f64 = *var_dvtp0_i_rv_slot;
        let mut var_dvtp1_i: f64 = *var_dvtp1_i_slot;
        let mut var_dvtp1_i_dn0: f64 = *var_dvtp1_i_dn0_slot;
        let mut var_dvtp1_i_dn10: f64 = *var_dvtp1_i_dn10_slot;
        let mut var_dvtp1_i_dn11: f64 = *var_dvtp1_i_dn11_slot;
        let mut var_dvtp1_i_dn13: f64 = *var_dvtp1_i_dn13_slot;
        let mut var_dvtp1_i_dn14: f64 = *var_dvtp1_i_dn14_slot;
        let mut var_dvtp1_i_dn2: f64 = *var_dvtp1_i_dn2_slot;
        let mut var_dvtp1_i_dn3: f64 = *var_dvtp1_i_dn3_slot;
        let mut var_dvtp1_i_dn4: f64 = *var_dvtp1_i_dn4_slot;
        let mut var_dvtp1_i_dn5: f64 = *var_dvtp1_i_dn5_slot;
        let mut var_dvtp1_i_dn6: f64 = *var_dvtp1_i_dn6_slot;
        let mut var_dvtp1_i_dn7: f64 = *var_dvtp1_i_dn7_slot;
        let mut var_dvtp1_i_dn8: f64 = *var_dvtp1_i_dn8_slot;
        let mut var_dvtp1_i_dn9: f64 = *var_dvtp1_i_dn9_slot;
        let mut var_dvtp1_i_rv: f64 = *var_dvtp1_i_rv_slot;
        let mut var_emobt_i: f64 = *var_emobt_i_slot;
        let mut var_emobt_i_rv: f64 = *var_emobt_i_rv_slot;
        let mut var_eu1_i: f64 = *var_eu1_i_slot;
        let mut var_eu1_i_rv: f64 = *var_eu1_i_rv_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard42_rv: f64 = *var_guard42_rv_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard43_rv: f64 = *var_guard43_rv_slot;
        let mut var_iit_i: f64 = *var_iit_i_slot;
        let mut var_iit_i_rv: f64 = *var_iit_i_rv_slot;
        let mut var_k11_i: f64 = *var_k11_i_slot;
        let mut var_k11_i_rv: f64 = *var_k11_i_rv_slot;
        let mut var_k1_i: f64 = *var_k1_i_slot;
        let mut var_k1_i_rv: f64 = *var_k1_i_rv_slot;
        let mut var_k21_i: f64 = *var_k21_i_slot;
        let mut var_k21_i_rv: f64 = *var_k21_i_rv_slot;
        let mut var_k2_i: f64 = *var_k2_i_slot;
        let mut var_k2_i_rv: f64 = *var_k2_i_rv_slot;
        let mut var_kt1_i: f64 = *var_kt1_i_slot;
        let mut var_kt1_i_rv: f64 = *var_kt1_i_rv_slot;
        let mut var_mpower_i: f64 = *var_mpower_i_slot;
        let mut var_mpower_i_rv: f64 = *var_mpower_i_rv_slot;
        let mut var_noia2_i: f64 = *var_noia2_i_slot;
        let mut var_noia2_i_rv: f64 = *var_noia2_i_rv_slot;
        let mut var_phibe_i: f64 = *var_phibe_i_slot;
        let mut var_phibe_i_rv: f64 = *var_phibe_i_rv_slot;
        let mut var_prt1_i: f64 = *var_prt1_i_slot;
        let mut var_prt1_i_rv: f64 = *var_prt1_i_rv_slot;
        let mut var_prt_i: f64 = *var_prt_i_slot;
        let mut var_prt_i_rv: f64 = *var_prt_i_rv_slot;
        let mut var_ptwgt_i: f64 = *var_ptwgt_i_slot;
        let mut var_ptwgt_i_rv: f64 = *var_ptwgt_i_rv_slot;
        let mut var_qsref_i: f64 = *var_qsref_i_slot;
        let mut var_qsref_i_rv: f64 = *var_qsref_i_rv_slot;
        let mut var_sprt_i: f64 = *var_sprt_i_slot;
        let mut var_sprt_i_rv: f64 = *var_sprt_i_rv_slot;
        let mut var_tgidl_i: f64 = *var_tgidl_i_slot;
        let mut var_tgidl_i_rv: f64 = *var_tgidl_i_rv_slot;
        let mut var_tr0_i: f64 = *var_tr0_i_slot;
        let mut var_tr0_i_rv: f64 = *var_tr0_i_rv_slot;
        let mut var_tss_i: f64 = *var_tss_i_slot;
        let mut var_tss_i_rv: f64 = *var_tss_i_rv_slot;
        let mut var_ttat_i: f64 = *var_ttat_i_slot;
        let mut var_ttat_i_rv: f64 = *var_ttat_i_rv_slot;
        let mut var_ua1_i: f64 = *var_ua1_i_slot;
        let mut var_ua1_i_rv: f64 = *var_ua1_i_rv_slot;
        let mut var_ua2_i: f64 = *var_ua2_i_slot;
        let mut var_ua2_i_rv: f64 = *var_ua2_i_rv_slot;
        let mut var_uc1_i: f64 = *var_uc1_i_slot;
        let mut var_uc1_i_rv: f64 = *var_uc1_i_rv_slot;
        let mut var_uc_i: f64 = *var_uc_i_slot;
        let mut var_uc_i_rv: f64 = *var_uc_i_rv_slot;
        let mut var_ucste1_i: f64 = *var_ucste1_i_slot;
        let mut var_ucste1_i_rv: f64 = *var_ucste1_i_rv_slot;
        let mut var_ucste_i: f64 = *var_ucste_i_slot;
        let mut var_ucste_i_rv: f64 = *var_ucste_i_rv_slot;
        let mut var_ud1_i: f64 = *var_ud1_i_slot;
        let mut var_ud1_i_rv: f64 = *var_ud1_i_rv_slot;
        let mut var_ud2_i: f64 = *var_ud2_i_slot;
        let mut var_ud2_i_rv: f64 = *var_ud2_i_rv_slot;
        let mut var_utl_i: f64 = *var_utl_i_slot;
        let mut var_utl_i_rv: f64 = *var_utl_i_rv_slot;

        let assign5510_e7701: f64 = (var_inv_l * p.p768);
        let assign5510_e7702: f64 = (p.p767 + assign5510_e7701);
        let assign5510_e7705: f64 = (var_inv_nfin * p.p769);
        let assign5510_e7706: f64 = (assign5510_e7702 + assign5510_e7705);
        let assign5510_e7709: f64 = (var_inv_lnfin * p.p770);
        let assign5510_e7710: f64 = (assign5510_e7706 + assign5510_e7709);
        let assign5510_e7713: f64 = (var_inv_w * p.p771);
        let assign5510_e7714: f64 = (assign5510_e7710 + assign5510_e7713);
        let assign5510_e7717: f64 = (var_inv_wl * p.p772);
        let assign5510_e7718: f64 = (assign5510_e7714 + assign5510_e7717);
        var_utl_i = assign5510_e7718;
        var_utl_i_rv = 0.0;

        let assign5520_e7722: f64 = (var_inv_l * p.p786);
        let assign5520_e7723: f64 = (p.p785 + assign5520_e7722);
        let assign5520_e7726: f64 = (var_inv_nfin * p.p787);
        let assign5520_e7727: f64 = (assign5520_e7723 + assign5520_e7726);
        let assign5520_e7730: f64 = (var_inv_lnfin * p.p788);
        let assign5520_e7731: f64 = (assign5520_e7727 + assign5520_e7730);
        let assign5520_e7734: f64 = (var_inv_w * p.p789);
        let assign5520_e7735: f64 = (assign5520_e7731 + assign5520_e7734);
        let assign5520_e7738: f64 = (var_inv_wl * p.p790);
        let assign5520_e7739: f64 = (assign5520_e7735 + assign5520_e7738);
        var_emobt_i = assign5520_e7739;
        var_emobt_i_rv = 0.0;

        let assign5530_e7743: f64 = (var_inv_l * p.p792);
        let assign5530_e7744: f64 = (p.p791 + assign5530_e7743);
        let assign5530_e7747: f64 = (var_inv_nfin * p.p793);
        let assign5530_e7748: f64 = (assign5530_e7744 + assign5530_e7747);
        let assign5530_e7751: f64 = (var_inv_lnfin * p.p794);
        let assign5530_e7752: f64 = (assign5530_e7748 + assign5530_e7751);
        let assign5530_e7755: f64 = (var_inv_w * p.p795);
        let assign5530_e7756: f64 = (assign5530_e7752 + assign5530_e7755);
        let assign5530_e7759: f64 = (var_inv_wl * p.p796);
        let assign5530_e7760: f64 = (assign5530_e7756 + assign5530_e7759);
        var_ua1_i = assign5530_e7760;
        var_ua1_i_rv = 0.0;

        let assign5540_e7764: f64 = (var_inv_l * p.p810);
        let assign5540_e7765: f64 = (p.p809 + assign5540_e7764);
        let assign5540_e7768: f64 = (var_inv_nfin * p.p811);
        let assign5540_e7769: f64 = (assign5540_e7765 + assign5540_e7768);
        let assign5540_e7772: f64 = (var_inv_lnfin * p.p812);
        let assign5540_e7773: f64 = (assign5540_e7769 + assign5540_e7772);
        let assign5540_e7776: f64 = (var_inv_w * p.p813);
        let assign5540_e7777: f64 = (assign5540_e7773 + assign5540_e7776);
        let assign5540_e7780: f64 = (var_inv_wl * p.p814);
        let assign5540_e7781: f64 = (assign5540_e7777 + assign5540_e7780);
        var_ua2_i = assign5540_e7781;
        var_ua2_i_rv = 0.0;

        let assign5550_e7785: f64 = (var_inv_l * p.p822);
        let assign5550_e7786: f64 = (p.p821 + assign5550_e7785);
        let assign5550_e7789: f64 = (var_inv_nfin * p.p823);
        let assign5550_e7790: f64 = (assign5550_e7786 + assign5550_e7789);
        let assign5550_e7793: f64 = (var_inv_lnfin * p.p824);
        let assign5550_e7794: f64 = (assign5550_e7790 + assign5550_e7793);
        let assign5550_e7797: f64 = (var_inv_w * p.p825);
        let assign5550_e7798: f64 = (assign5550_e7794 + assign5550_e7797);
        let assign5550_e7801: f64 = (var_inv_wl * p.p826);
        let assign5550_e7802: f64 = (assign5550_e7798 + assign5550_e7801);
        var_eu1_i = assign5550_e7802;
        var_eu1_i_rv = 0.0;

        let assign5560_e7806: f64 = (var_inv_l * p.p846);
        let assign5560_e7807: f64 = (p.p845 + assign5560_e7806);
        let assign5560_e7810: f64 = (var_inv_nfin * p.p847);
        let assign5560_e7811: f64 = (assign5560_e7807 + assign5560_e7810);
        let assign5560_e7814: f64 = (var_inv_lnfin * p.p848);
        let assign5560_e7815: f64 = (assign5560_e7811 + assign5560_e7814);
        let assign5560_e7818: f64 = (var_inv_w * p.p849);
        let assign5560_e7819: f64 = (assign5560_e7815 + assign5560_e7818);
        let assign5560_e7822: f64 = (var_inv_wl * p.p850);
        let assign5560_e7823: f64 = (assign5560_e7819 + assign5560_e7822);
        var_ud1_i = assign5560_e7823;
        var_ud1_i_rv = 0.0;

        let assign5570_e7827: f64 = (var_inv_l * p.p864);
        let assign5570_e7828: f64 = (p.p863 + assign5570_e7827);
        let assign5570_e7831: f64 = (var_inv_nfin * p.p865);
        let assign5570_e7832: f64 = (assign5570_e7828 + assign5570_e7831);
        let assign5570_e7835: f64 = (var_inv_lnfin * p.p866);
        let assign5570_e7836: f64 = (assign5570_e7832 + assign5570_e7835);
        let assign5570_e7839: f64 = (var_inv_w * p.p867);
        let assign5570_e7840: f64 = (assign5570_e7836 + assign5570_e7839);
        let assign5570_e7843: f64 = (var_inv_wl * p.p868);
        let assign5570_e7844: f64 = (assign5570_e7840 + assign5570_e7843);
        var_ud2_i = assign5570_e7844;
        var_ud2_i_rv = 0.0;

        let assign5580_e7848: f64 = (var_inv_l * p.p876);
        let assign5580_e7849: f64 = (p.p875 + assign5580_e7848);
        let assign5580_e7852: f64 = (var_inv_nfin * p.p877);
        let assign5580_e7853: f64 = (assign5580_e7849 + assign5580_e7852);
        let assign5580_e7856: f64 = (var_inv_lnfin * p.p878);
        let assign5580_e7857: f64 = (assign5580_e7853 + assign5580_e7856);
        let assign5580_e7860: f64 = (var_inv_w * p.p879);
        let assign5580_e7861: f64 = (assign5580_e7857 + assign5580_e7860);
        let assign5580_e7864: f64 = (var_inv_wl * p.p880);
        let assign5580_e7865: f64 = (assign5580_e7861 + assign5580_e7864);
        var_ucste_i = assign5580_e7865;
        var_ucste_i_rv = 0.0;

        let assign5590_e7869: f64 = (var_inv_l * p.p882);
        let assign5590_e7870: f64 = (p.p881 + assign5590_e7869);
        let assign5590_e7873: f64 = (var_inv_nfin * p.p883);
        let assign5590_e7874: f64 = (assign5590_e7870 + assign5590_e7873);
        let assign5590_e7877: f64 = (var_inv_lnfin * p.p884);
        let assign5590_e7878: f64 = (assign5590_e7874 + assign5590_e7877);
        let assign5590_e7881: f64 = (var_inv_w * p.p885);
        let assign5590_e7882: f64 = (assign5590_e7878 + assign5590_e7881);
        let assign5590_e7885: f64 = (var_inv_wl * p.p886);
        let assign5590_e7886: f64 = (assign5590_e7882 + assign5590_e7885);
        var_ucste1_i = assign5590_e7886;
        var_ucste1_i_rv = 0.0;

        let assign5600_e7890: f64 = (var_inv_l * p.p576);
        let assign5600_e7891: f64 = (p.p575 + assign5600_e7890);
        let assign5600_e7894: f64 = (var_inv_nfin * p.p577);
        let assign5600_e7895: f64 = (assign5600_e7891 + assign5600_e7894);
        let assign5600_e7898: f64 = (var_inv_lnfin * p.p578);
        let assign5600_e7899: f64 = (assign5600_e7895 + assign5600_e7898);
        let assign5600_e7902: f64 = (var_inv_w * p.p579);
        let assign5600_e7903: f64 = (assign5600_e7899 + assign5600_e7902);
        let assign5600_e7906: f64 = (var_inv_wl * p.p580);
        let assign5600_e7907: f64 = (assign5600_e7903 + assign5600_e7906);
        var_ptwgt_i = assign5600_e7907;
        var_ptwgt_i_rv = 0.0;

        let assign5610_e7911: f64 = (var_inv_l * p.p556);
        let assign5610_e7912: f64 = (p.p555 + assign5610_e7911);
        let assign5610_e7915: f64 = (var_inv_nfin * p.p557);
        let assign5610_e7916: f64 = (assign5610_e7912 + assign5610_e7915);
        let assign5610_e7919: f64 = (var_inv_lnfin * p.p558);
        let assign5610_e7920: f64 = (assign5610_e7916 + assign5610_e7919);
        let assign5610_e7923: f64 = (var_inv_w * p.p559);
        let assign5610_e7924: f64 = (assign5610_e7920 + assign5610_e7923);
        let assign5610_e7927: f64 = (var_inv_wl * p.p560);
        let assign5610_e7928: f64 = (assign5610_e7924 + assign5610_e7927);
        var_at_i = assign5610_e7928;
        var_at_i_rv = 0.0;

        let assign5620_e7932: f64 = (var_inv_l * p.p569);
        let assign5620_e7933: f64 = (p.p568 + assign5620_e7932);
        let assign5620_e7936: f64 = (var_inv_nfin * p.p570);
        let assign5620_e7937: f64 = (assign5620_e7933 + assign5620_e7936);
        let assign5620_e7940: f64 = (var_inv_lnfin * p.p571);
        let assign5620_e7941: f64 = (assign5620_e7937 + assign5620_e7940);
        let assign5620_e7944: f64 = (var_inv_w * p.p572);
        let assign5620_e7945: f64 = (assign5620_e7941 + assign5620_e7944);
        let assign5620_e7948: f64 = (var_inv_wl * p.p573);
        let assign5620_e7949: f64 = (assign5620_e7945 + assign5620_e7948);
        var_atcv_i = assign5620_e7949;
        var_atcv_i_rv = 0.0;

        let assign5630_e7953: f64 = (var_inv_l * p.p962);
        let assign5630_e7954: f64 = (p.p961 + assign5630_e7953);
        let assign5630_e7957: f64 = (var_inv_nfin * p.p963);
        let assign5630_e7958: f64 = (assign5630_e7954 + assign5630_e7957);
        let assign5630_e7961: f64 = (var_inv_lnfin * p.p964);
        let assign5630_e7962: f64 = (assign5630_e7958 + assign5630_e7961);
        let assign5630_e7965: f64 = (var_inv_w * p.p965);
        let assign5630_e7966: f64 = (assign5630_e7962 + assign5630_e7965);
        let assign5630_e7969: f64 = (var_inv_wl * p.p966);
        let assign5630_e7970: f64 = (assign5630_e7966 + assign5630_e7969);
        var_prt_i = assign5630_e7970;
        var_prt_i_rv = 0.0;

        let assign5640_e7974: f64 = (var_inv_l * p.p968);
        let assign5640_e7975: f64 = (p.p967 + assign5640_e7974);
        let assign5640_e7978: f64 = (var_inv_nfin * p.p969);
        let assign5640_e7979: f64 = (assign5640_e7975 + assign5640_e7978);
        let assign5640_e7982: f64 = (var_inv_lnfin * p.p970);
        let assign5640_e7983: f64 = (assign5640_e7979 + assign5640_e7982);
        let assign5640_e7986: f64 = (var_inv_w * p.p971);
        let assign5640_e7987: f64 = (assign5640_e7983 + assign5640_e7986);
        let assign5640_e7990: f64 = (var_inv_wl * p.p972);
        let assign5640_e7991: f64 = (assign5640_e7987 + assign5640_e7990);
        var_prt1_i = assign5640_e7991;
        var_prt1_i_rv = 0.0;

        let assign5650_e7995: f64 = (var_inv_l * p.p974);
        let assign5650_e7996: f64 = (p.p973 + assign5650_e7995);
        let assign5650_e7999: f64 = (var_inv_nfin * p.p975);
        let assign5650_e8000: f64 = (assign5650_e7996 + assign5650_e7999);
        let assign5650_e8003: f64 = (var_inv_lnfin * p.p976);
        let assign5650_e8004: f64 = (assign5650_e8000 + assign5650_e8003);
        let assign5650_e8007: f64 = (var_inv_w * p.p977);
        let assign5650_e8008: f64 = (assign5650_e8004 + assign5650_e8007);
        let assign5650_e8011: f64 = (var_inv_wl * p.p978);
        let assign5650_e8012: f64 = (assign5650_e8008 + assign5650_e8011);
        var_tr0_i = assign5650_e8012;
        var_tr0_i_rv = 0.0;

        let assign5660_e8016: f64 = (var_inv_l * p.p980);
        let assign5660_e8017: f64 = (p.p979 + assign5660_e8016);
        let assign5660_e8020: f64 = (var_inv_nfin * p.p981);
        let assign5660_e8021: f64 = (assign5660_e8017 + assign5660_e8020);
        let assign5660_e8024: f64 = (var_inv_lnfin * p.p982);
        let assign5660_e8025: f64 = (assign5660_e8021 + assign5660_e8024);
        let assign5660_e8028: f64 = (var_inv_w * p.p983);
        let assign5660_e8029: f64 = (assign5660_e8025 + assign5660_e8028);
        let assign5660_e8032: f64 = (var_inv_wl * p.p984);
        let assign5660_e8033: f64 = (assign5660_e8029 + assign5660_e8032);
        var_sprt_i = assign5660_e8033;
        var_sprt_i_rv = 0.0;

        let assign5670_e8037: f64 = (var_inv_l * p.p1742);
        let assign5670_e8038: f64 = (p.p1741 + assign5670_e8037);
        let assign5670_e8041: f64 = (var_inv_nfin * p.p1743);
        let assign5670_e8042: f64 = (assign5670_e8038 + assign5670_e8041);
        let assign5670_e8045: f64 = (var_inv_lnfin * p.p1744);
        let assign5670_e8046: f64 = (assign5670_e8042 + assign5670_e8045);
        let assign5670_e8049: f64 = (var_inv_w * p.p1745);
        let assign5670_e8050: f64 = (assign5670_e8046 + assign5670_e8049);
        let assign5670_e8053: f64 = (var_inv_wl * p.p1746);
        let assign5670_e8054: f64 = (assign5670_e8050 + assign5670_e8053);
        var_kt1_i = assign5670_e8054;
        var_kt1_i_rv = 0.0;

        let assign5680_e8058: f64 = (var_inv_l * p.p1751);
        let assign5680_e8059: f64 = (p.p1750 + assign5680_e8058);
        let assign5680_e8062: f64 = (var_inv_nfin * p.p1752);
        let assign5680_e8063: f64 = (assign5680_e8059 + assign5680_e8062);
        let assign5680_e8066: f64 = (var_inv_lnfin * p.p1753);
        let assign5680_e8067: f64 = (assign5680_e8063 + assign5680_e8066);
        let assign5680_e8070: f64 = (var_inv_w * p.p1754);
        let assign5680_e8071: f64 = (assign5680_e8067 + assign5680_e8070);
        let assign5680_e8074: f64 = (var_inv_wl * p.p1755);
        let assign5680_e8075: f64 = (assign5680_e8071 + assign5680_e8074);
        var_tss_i = assign5680_e8075;
        var_tss_i_rv = 0.0;

        let assign5690_e8079: f64 = (var_inv_l * p.p1757);
        let assign5690_e8080: f64 = (p.p1756 + assign5690_e8079);
        let assign5690_e8083: f64 = (var_inv_nfin * p.p1758);
        let assign5690_e8084: f64 = (assign5690_e8080 + assign5690_e8083);
        let assign5690_e8087: f64 = (var_inv_lnfin * p.p1759);
        let assign5690_e8088: f64 = (assign5690_e8084 + assign5690_e8087);
        let assign5690_e8091: f64 = (var_inv_w * p.p1760);
        let assign5690_e8092: f64 = (assign5690_e8088 + assign5690_e8091);
        let assign5690_e8095: f64 = (var_inv_wl * p.p1761);
        let assign5690_e8096: f64 = (assign5690_e8092 + assign5690_e8095);
        var_iit_i = assign5690_e8096;
        var_iit_i_rv = 0.0;

        let assign5700_e8100: f64 = (var_inv_l * p.p1769);
        let assign5700_e8101: f64 = (p.p1768 + assign5700_e8100);
        let assign5700_e8104: f64 = (var_inv_nfin * p.p1770);
        let assign5700_e8105: f64 = (assign5700_e8101 + assign5700_e8104);
        let assign5700_e8108: f64 = (var_inv_lnfin * p.p1771);
        let assign5700_e8109: f64 = (assign5700_e8105 + assign5700_e8108);
        let assign5700_e8112: f64 = (var_inv_w * p.p1772);
        let assign5700_e8113: f64 = (assign5700_e8109 + assign5700_e8112);
        let assign5700_e8116: f64 = (var_inv_wl * p.p1773);
        let assign5700_e8117: f64 = (assign5700_e8113 + assign5700_e8116);
        var_tgidl_i = assign5700_e8117;
        var_tgidl_i_rv = 0.0;

        let assign5710_e8121: f64 = (var_inv_l * p.p1775);
        let assign5710_e8122: f64 = (p.p1774 + assign5710_e8121);
        let assign5710_e8125: f64 = (var_inv_nfin * p.p1776);
        let assign5710_e8126: f64 = (assign5710_e8122 + assign5710_e8125);
        let assign5710_e8129: f64 = (var_inv_lnfin * p.p1777);
        let assign5710_e8130: f64 = (assign5710_e8126 + assign5710_e8129);
        let assign5710_e8133: f64 = (var_inv_w * p.p1778);
        let assign5710_e8134: f64 = (assign5710_e8130 + assign5710_e8133);
        let assign5710_e8137: f64 = (var_inv_wl * p.p1779);
        let assign5710_e8138: f64 = (assign5710_e8134 + assign5710_e8137);
        var_ttat_i = assign5710_e8138;
        var_ttat_i_rv = 0.0;

        let assign5730_e8163: f64 = (var_inv_l * p.p177);
        let assign5730_e8164: f64 = (p.p176 + assign5730_e8163);
        let assign5730_e8167: f64 = (var_inv_nfin * p.p178);
        let assign5730_e8168: f64 = (assign5730_e8164 + assign5730_e8167);
        let assign5730_e8171: f64 = (var_inv_lnfin * p.p179);
        let assign5730_e8172: f64 = (assign5730_e8168 + assign5730_e8171);
        let assign5730_e8175: f64 = (var_inv_w * p.p180);
        let assign5730_e8176: f64 = (assign5730_e8172 + assign5730_e8175);
        let assign5730_e8179: f64 = (var_inv_wl * p.p181);
        let assign5730_e8180: f64 = (assign5730_e8176 + assign5730_e8179);
        var_dvtp0_i = assign5730_e8180;
        var_dvtp0_i_dn0 = 0.0;
        var_dvtp0_i_dn2 = 0.0;
        var_dvtp0_i_dn3 = 0.0;
        var_dvtp0_i_dn4 = 0.0;
        var_dvtp0_i_dn5 = 0.0;
        var_dvtp0_i_dn6 = 0.0;
        var_dvtp0_i_dn7 = 0.0;
        var_dvtp0_i_dn8 = 0.0;
        var_dvtp0_i_dn9 = 0.0;
        var_dvtp0_i_dn10 = 0.0;
        var_dvtp0_i_dn11 = 0.0;
        var_dvtp0_i_dn13 = 0.0;
        var_dvtp0_i_dn14 = 0.0;
        var_dvtp0_i_rv = 0.0;

        let assign5740_e8184: f64 = (var_inv_l * p.p183);
        let assign5740_e8185: f64 = (p.p182 + assign5740_e8184);
        let assign5740_e8188: f64 = (var_inv_nfin * p.p184);
        let assign5740_e8189: f64 = (assign5740_e8185 + assign5740_e8188);
        let assign5740_e8192: f64 = (var_inv_lnfin * p.p185);
        let assign5740_e8193: f64 = (assign5740_e8189 + assign5740_e8192);
        let assign5740_e8196: f64 = (var_inv_w * p.p186);
        let assign5740_e8197: f64 = (assign5740_e8193 + assign5740_e8196);
        let assign5740_e8200: f64 = (var_inv_wl * p.p187);
        let assign5740_e8201: f64 = (assign5740_e8197 + assign5740_e8200);
        var_dvtp1_i = assign5740_e8201;
        var_dvtp1_i_dn0 = 0.0;
        var_dvtp1_i_dn2 = 0.0;
        var_dvtp1_i_dn3 = 0.0;
        var_dvtp1_i_dn4 = 0.0;
        var_dvtp1_i_dn5 = 0.0;
        var_dvtp1_i_dn6 = 0.0;
        var_dvtp1_i_dn7 = 0.0;
        var_dvtp1_i_dn8 = 0.0;
        var_dvtp1_i_dn9 = 0.0;
        var_dvtp1_i_dn10 = 0.0;
        var_dvtp1_i_dn11 = 0.0;
        var_dvtp1_i_dn13 = 0.0;
        var_dvtp1_i_dn14 = 0.0;
        var_dvtp1_i_rv = 0.0;

        let assign5750_e8205: f64 = (var_inv_l * p.p1690);
        let assign5750_e8206: f64 = (p.p1689 + assign5750_e8205);
        let assign5750_e8209: f64 = (var_inv_nfin * p.p1691);
        let assign5750_e8210: f64 = (assign5750_e8206 + assign5750_e8209);
        let assign5750_e8213: f64 = (var_inv_lnfin * p.p1692);
        let assign5750_e8214: f64 = (assign5750_e8210 + assign5750_e8213);
        let assign5750_e8217: f64 = (var_inv_w * p.p1693);
        let assign5750_e8218: f64 = (assign5750_e8214 + assign5750_e8217);
        let assign5750_e8221: f64 = (var_inv_wl * p.p1694);
        let assign5750_e8222: f64 = (assign5750_e8218 + assign5750_e8221);
        var_noia2_i = assign5750_e8222;
        var_noia2_i_rv = 0.0;

        let assign5760_e8226: f64 = (var_inv_l * p.p1702);
        let assign5760_e8227: f64 = (p.p1701 + assign5760_e8226);
        let assign5760_e8230: f64 = (var_inv_nfin * p.p1703);
        let assign5760_e8231: f64 = (assign5760_e8227 + assign5760_e8230);
        let assign5760_e8234: f64 = (var_inv_lnfin * p.p1704);
        let assign5760_e8235: f64 = (assign5760_e8231 + assign5760_e8234);
        let assign5760_e8238: f64 = (var_inv_w * p.p1705);
        let assign5760_e8239: f64 = (assign5760_e8235 + assign5760_e8238);
        let assign5760_e8242: f64 = (var_inv_wl * p.p1706);
        let assign5760_e8243: f64 = (assign5760_e8239 + assign5760_e8242);
        var_qsref_i = assign5760_e8243;
        var_qsref_i_rv = 0.0;

        let assign5770_e8247: f64 = (var_inv_l * p.p1696);
        let assign5770_e8248: f64 = (p.p1695 + assign5770_e8247);
        let assign5770_e8251: f64 = (var_inv_nfin * p.p1697);
        let assign5770_e8252: f64 = (assign5770_e8248 + assign5770_e8251);
        let assign5770_e8255: f64 = (var_inv_lnfin * p.p1698);
        let assign5770_e8256: f64 = (assign5770_e8252 + assign5770_e8255);
        let assign5770_e8259: f64 = (var_inv_w * p.p1699);
        let assign5770_e8260: f64 = (assign5770_e8256 + assign5770_e8259);
        let assign5770_e8263: f64 = (var_inv_wl * p.p1700);
        let assign5770_e8264: f64 = (assign5770_e8260 + assign5770_e8263);
        var_mpower_i = assign5770_e8264;
        var_mpower_i_rv = 0.0;

        let assign5780_e8267: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard42 = assign5780_e8267;
        var_guard42_rv = 0.0;

        let (assign5790_e8291,) = {
    if (var_guard42 != 0.0) {
        let assign5790_e8272: f64 = (var_inv_l * p.p357);
        let assign5790_e8273: f64 = (p.p356 + assign5790_e8272);
        let assign5790_e8276: f64 = (var_inv_nfin * p.p358);
        let assign5790_e8277: f64 = (assign5790_e8273 + assign5790_e8276);
        let assign5790_e8280: f64 = (var_inv_lnfin * p.p359);
        let assign5790_e8281: f64 = (assign5790_e8277 + assign5790_e8280);
        let assign5790_e8284: f64 = (var_inv_w * p.p360);
        let assign5790_e8285: f64 = (assign5790_e8281 + assign5790_e8284);
        let assign5790_e8288: f64 = (var_inv_wl * p.p361);
        let assign5790_e8289: f64 = (assign5790_e8285 + assign5790_e8288);
        (assign5790_e8289,)
    } else {
        (var_phibe_i,)
    }
};
        var_phibe_i = assign5790_e8291;
        var_phibe_i_rv = 0.0;

        let (assign5800_e8315,) = {
    if (var_guard42 != 0.0) {
        let assign5800_e8296: f64 = (var_inv_l * p.p363);
        let assign5800_e8297: f64 = (p.p362 + assign5800_e8296);
        let assign5800_e8300: f64 = (var_inv_nfin * p.p364);
        let assign5800_e8301: f64 = (assign5800_e8297 + assign5800_e8300);
        let assign5800_e8304: f64 = (var_inv_lnfin * p.p365);
        let assign5800_e8305: f64 = (assign5800_e8301 + assign5800_e8304);
        let assign5800_e8308: f64 = (var_inv_w * p.p366);
        let assign5800_e8309: f64 = (assign5800_e8305 + assign5800_e8308);
        let assign5800_e8312: f64 = (var_inv_wl * p.p367);
        let assign5800_e8313: f64 = (assign5800_e8309 + assign5800_e8312);
        (assign5800_e8313,)
    } else {
        (var_k1_i,)
    }
};
        var_k1_i = assign5800_e8315;
        var_k1_i_rv = 0.0;

        let (assign5810_e8339,) = {
    if (var_guard42 != 0.0) {
        let assign5810_e8320: f64 = (var_inv_l * p.p369);
        let assign5810_e8321: f64 = (p.p368 + assign5810_e8320);
        let assign5810_e8324: f64 = (var_inv_nfin * p.p370);
        let assign5810_e8325: f64 = (assign5810_e8321 + assign5810_e8324);
        let assign5810_e8328: f64 = (var_inv_lnfin * p.p371);
        let assign5810_e8329: f64 = (assign5810_e8325 + assign5810_e8328);
        let assign5810_e8332: f64 = (var_inv_w * p.p372);
        let assign5810_e8333: f64 = (assign5810_e8329 + assign5810_e8332);
        let assign5810_e8336: f64 = (var_inv_wl * p.p373);
        let assign5810_e8337: f64 = (assign5810_e8333 + assign5810_e8336);
        (assign5810_e8337,)
    } else {
        (var_k11_i,)
    }
};
        var_k11_i = assign5810_e8339;
        var_k11_i_rv = 0.0;

        let (assign5820_e8363,) = {
    if (var_guard42 != 0.0) {
        let assign5820_e8344: f64 = (var_inv_l * p.p660);
        let assign5820_e8345: f64 = (p.p659 + assign5820_e8344);
        let assign5820_e8348: f64 = (var_inv_nfin * p.p661);
        let assign5820_e8349: f64 = (assign5820_e8345 + assign5820_e8348);
        let assign5820_e8352: f64 = (var_inv_lnfin * p.p662);
        let assign5820_e8353: f64 = (assign5820_e8349 + assign5820_e8352);
        let assign5820_e8356: f64 = (var_inv_w * p.p663);
        let assign5820_e8357: f64 = (assign5820_e8353 + assign5820_e8356);
        let assign5820_e8360: f64 = (var_inv_wl * p.p664);
        let assign5820_e8361: f64 = (assign5820_e8357 + assign5820_e8360);
        (assign5820_e8361,)
    } else {
        (var_uc_i,)
    }
};
        var_uc_i = assign5820_e8363;
        var_uc_i_rv = 0.0;

        let (assign5830_e8387,) = {
    if (var_guard42 != 0.0) {
        let assign5830_e8368: f64 = (var_inv_l * p.p828);
        let assign5830_e8369: f64 = (p.p827 + assign5830_e8368);
        let assign5830_e8372: f64 = (var_inv_nfin * p.p829);
        let assign5830_e8373: f64 = (assign5830_e8369 + assign5830_e8372);
        let assign5830_e8376: f64 = (var_inv_lnfin * p.p830);
        let assign5830_e8377: f64 = (assign5830_e8373 + assign5830_e8376);
        let assign5830_e8380: f64 = (var_inv_w * p.p831);
        let assign5830_e8381: f64 = (assign5830_e8377 + assign5830_e8380);
        let assign5830_e8384: f64 = (var_inv_wl * p.p832);
        let assign5830_e8385: f64 = (assign5830_e8381 + assign5830_e8384);
        (assign5830_e8385,)
    } else {
        (var_uc1_i,)
    }
};
        var_uc1_i = assign5830_e8387;
        var_uc1_i_rv = 0.0;

        let assign5840_e8390: f64 = if p.p61 == 2.0 { 1.0 } else { 0.0 };
        var_guard43 = assign5840_e8390;
        var_guard43_rv = 0.0;

        let (assign5850_e8416,) = {
    if ((var_guard42 != 0.0) && (var_guard43 != 0.0)) {
        let assign5850_e8397: f64 = (var_inv_l * p.p387);
        let assign5850_e8398: f64 = (p.p386 + assign5850_e8397);
        let assign5850_e8401: f64 = (var_inv_nfin * p.p388);
        let assign5850_e8402: f64 = (assign5850_e8398 + assign5850_e8401);
        let assign5850_e8405: f64 = (var_inv_lnfin * p.p389);
        let assign5850_e8406: f64 = (assign5850_e8402 + assign5850_e8405);
        let assign5850_e8409: f64 = (var_inv_w * p.p390);
        let assign5850_e8410: f64 = (assign5850_e8406 + assign5850_e8409);
        let assign5850_e8413: f64 = (var_inv_wl * p.p391);
        let assign5850_e8414: f64 = (assign5850_e8410 + assign5850_e8413);
        (assign5850_e8414,)
    } else {
        (var_k2_i,)
    }
};
        var_k2_i = assign5850_e8416;
        var_k2_i_rv = 0.0;

        let (assign5860_e8442,) = {
    if ((var_guard42 != 0.0) && (var_guard43 != 0.0)) {
        let assign5860_e8423: f64 = (var_inv_l * p.p393);
        let assign5860_e8424: f64 = (p.p392 + assign5860_e8423);
        let assign5860_e8427: f64 = (var_inv_nfin * p.p394);
        let assign5860_e8428: f64 = (assign5860_e8424 + assign5860_e8427);
        let assign5860_e8431: f64 = (var_inv_lnfin * p.p395);
        let assign5860_e8432: f64 = (assign5860_e8428 + assign5860_e8431);
        let assign5860_e8435: f64 = (var_inv_w * p.p396);
        let assign5860_e8436: f64 = (assign5860_e8432 + assign5860_e8435);
        let assign5860_e8439: f64 = (var_inv_wl * p.p397);
        let assign5860_e8440: f64 = (assign5860_e8436 + assign5860_e8439);
        (assign5860_e8440,)
    } else {
        (var_k21_i,)
    }
};
        var_k21_i = assign5860_e8442;
        var_k21_i_rv = 0.0;

        *var_at_i_slot = var_at_i;
        *var_at_i_rv_slot = var_at_i_rv;
        *var_atcv_i_slot = var_atcv_i;
        *var_atcv_i_rv_slot = var_atcv_i_rv;
        *var_dvtp0_i_slot = var_dvtp0_i;
        *var_dvtp0_i_dn0_slot = var_dvtp0_i_dn0;
        *var_dvtp0_i_dn10_slot = var_dvtp0_i_dn10;
        *var_dvtp0_i_dn11_slot = var_dvtp0_i_dn11;
        *var_dvtp0_i_dn13_slot = var_dvtp0_i_dn13;
        *var_dvtp0_i_dn14_slot = var_dvtp0_i_dn14;
        *var_dvtp0_i_dn2_slot = var_dvtp0_i_dn2;
        *var_dvtp0_i_dn3_slot = var_dvtp0_i_dn3;
        *var_dvtp0_i_dn4_slot = var_dvtp0_i_dn4;
        *var_dvtp0_i_dn5_slot = var_dvtp0_i_dn5;
        *var_dvtp0_i_dn6_slot = var_dvtp0_i_dn6;
        *var_dvtp0_i_dn7_slot = var_dvtp0_i_dn7;
        *var_dvtp0_i_dn8_slot = var_dvtp0_i_dn8;
        *var_dvtp0_i_dn9_slot = var_dvtp0_i_dn9;
        *var_dvtp0_i_rv_slot = var_dvtp0_i_rv;
        *var_dvtp1_i_slot = var_dvtp1_i;
        *var_dvtp1_i_dn0_slot = var_dvtp1_i_dn0;
        *var_dvtp1_i_dn10_slot = var_dvtp1_i_dn10;
        *var_dvtp1_i_dn11_slot = var_dvtp1_i_dn11;
        *var_dvtp1_i_dn13_slot = var_dvtp1_i_dn13;
        *var_dvtp1_i_dn14_slot = var_dvtp1_i_dn14;
        *var_dvtp1_i_dn2_slot = var_dvtp1_i_dn2;
        *var_dvtp1_i_dn3_slot = var_dvtp1_i_dn3;
        *var_dvtp1_i_dn4_slot = var_dvtp1_i_dn4;
        *var_dvtp1_i_dn5_slot = var_dvtp1_i_dn5;
        *var_dvtp1_i_dn6_slot = var_dvtp1_i_dn6;
        *var_dvtp1_i_dn7_slot = var_dvtp1_i_dn7;
        *var_dvtp1_i_dn8_slot = var_dvtp1_i_dn8;
        *var_dvtp1_i_dn9_slot = var_dvtp1_i_dn9;
        *var_dvtp1_i_rv_slot = var_dvtp1_i_rv;
        *var_emobt_i_slot = var_emobt_i;
        *var_emobt_i_rv_slot = var_emobt_i_rv;
        *var_eu1_i_slot = var_eu1_i;
        *var_eu1_i_rv_slot = var_eu1_i_rv;
        *var_guard42_slot = var_guard42;
        *var_guard42_rv_slot = var_guard42_rv;
        *var_guard43_slot = var_guard43;
        *var_guard43_rv_slot = var_guard43_rv;
        *var_iit_i_slot = var_iit_i;
        *var_iit_i_rv_slot = var_iit_i_rv;
        *var_k11_i_slot = var_k11_i;
        *var_k11_i_rv_slot = var_k11_i_rv;
        *var_k1_i_slot = var_k1_i;
        *var_k1_i_rv_slot = var_k1_i_rv;
        *var_k21_i_slot = var_k21_i;
        *var_k21_i_rv_slot = var_k21_i_rv;
        *var_k2_i_slot = var_k2_i;
        *var_k2_i_rv_slot = var_k2_i_rv;
        *var_kt1_i_slot = var_kt1_i;
        *var_kt1_i_rv_slot = var_kt1_i_rv;
        *var_mpower_i_slot = var_mpower_i;
        *var_mpower_i_rv_slot = var_mpower_i_rv;
        *var_noia2_i_slot = var_noia2_i;
        *var_noia2_i_rv_slot = var_noia2_i_rv;
        *var_phibe_i_slot = var_phibe_i;
        *var_phibe_i_rv_slot = var_phibe_i_rv;
        *var_prt1_i_slot = var_prt1_i;
        *var_prt1_i_rv_slot = var_prt1_i_rv;
        *var_prt_i_slot = var_prt_i;
        *var_prt_i_rv_slot = var_prt_i_rv;
        *var_ptwgt_i_slot = var_ptwgt_i;
        *var_ptwgt_i_rv_slot = var_ptwgt_i_rv;
        *var_qsref_i_slot = var_qsref_i;
        *var_qsref_i_rv_slot = var_qsref_i_rv;
        *var_sprt_i_slot = var_sprt_i;
        *var_sprt_i_rv_slot = var_sprt_i_rv;
        *var_tgidl_i_slot = var_tgidl_i;
        *var_tgidl_i_rv_slot = var_tgidl_i_rv;
        *var_tr0_i_slot = var_tr0_i;
        *var_tr0_i_rv_slot = var_tr0_i_rv;
        *var_tss_i_slot = var_tss_i;
        *var_tss_i_rv_slot = var_tss_i_rv;
        *var_ttat_i_slot = var_ttat_i;
        *var_ttat_i_rv_slot = var_ttat_i_rv;
        *var_ua1_i_slot = var_ua1_i;
        *var_ua1_i_rv_slot = var_ua1_i_rv;
        *var_ua2_i_slot = var_ua2_i;
        *var_ua2_i_rv_slot = var_ua2_i_rv;
        *var_uc1_i_slot = var_uc1_i;
        *var_uc1_i_rv_slot = var_uc1_i_rv;
        *var_uc_i_slot = var_uc_i;
        *var_uc_i_rv_slot = var_uc_i_rv;
        *var_ucste1_i_slot = var_ucste1_i;
        *var_ucste1_i_rv_slot = var_ucste1_i_rv;
        *var_ucste_i_slot = var_ucste_i;
        *var_ucste_i_rv_slot = var_ucste_i_rv;
        *var_ud1_i_slot = var_ud1_i;
        *var_ud1_i_rv_slot = var_ud1_i_rv;
        *var_ud2_i_slot = var_ud2_i;
        *var_ud2_i_rv_slot = var_ud2_i_rv;
        *var_utl_i_slot = var_utl_i;
        *var_utl_i_rv_slot = var_utl_i_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        var_guard42: f64,
        var_guard43: f64,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_agidlb_i_slot: &mut f64,
        var_agidlb_i_rv_slot: &mut f64,
        var_agislb_i_slot: &mut f64,
        var_agislb_i_rv_slot: &mut f64,
        var_bgidlb_i_slot: &mut f64,
        var_bgidlb_i_rv_slot: &mut f64,
        var_bgislb_i_slot: &mut f64,
        var_bgislb_i_rv_slot: &mut f64,
        var_cdscdr_i_slot: &mut f64,
        var_cdscdr_i_rv_slot: &mut f64,
        var_cgidlb_i_slot: &mut f64,
        var_cgidlb_i_rv_slot: &mut f64,
        var_cgislb_i_slot: &mut f64,
        var_cgislb_i_rv_slot: &mut f64,
        var_citr_i_slot: &mut f64,
        var_citr_i_rv_slot: &mut f64,
        var_dvtshiftr_i_slot: &mut f64,
        var_dvtshiftr_i_rv_slot: &mut f64,
        var_egidlb_i_slot: &mut f64,
        var_egidlb_i_rv_slot: &mut f64,
        var_egislb_i_slot: &mut f64,
        var_egislb_i_rv_slot: &mut f64,
        var_eta0r_i_slot: &mut f64,
        var_eta0r_i_rv_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard44_rv_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard45_rv_slot: &mut f64,
        var_k2sat1_i_slot: &mut f64,
        var_k2sat1_i_rv_slot: &mut f64,
        var_k2sat_i_slot: &mut f64,
        var_k2sat_i_rv_slot: &mut f64,
        var_mexpr_i_slot: &mut f64,
        var_mexpr_i_dn0_slot: &mut f64,
        var_mexpr_i_dn10_slot: &mut f64,
        var_mexpr_i_dn11_slot: &mut f64,
        var_mexpr_i_dn13_slot: &mut f64,
        var_mexpr_i_dn14_slot: &mut f64,
        var_mexpr_i_dn2_slot: &mut f64,
        var_mexpr_i_dn3_slot: &mut f64,
        var_mexpr_i_dn4_slot: &mut f64,
        var_mexpr_i_dn5_slot: &mut f64,
        var_mexpr_i_dn6_slot: &mut f64,
        var_mexpr_i_dn7_slot: &mut f64,
        var_mexpr_i_dn8_slot: &mut f64,
        var_mexpr_i_dn9_slot: &mut f64,
        var_mexpr_i_rv_slot: &mut f64,
        var_pclmr_i_slot: &mut f64,
        var_pclmr_i_dn0_slot: &mut f64,
        var_pclmr_i_dn10_slot: &mut f64,
        var_pclmr_i_dn11_slot: &mut f64,
        var_pclmr_i_dn13_slot: &mut f64,
        var_pclmr_i_dn14_slot: &mut f64,
        var_pclmr_i_dn2_slot: &mut f64,
        var_pclmr_i_dn3_slot: &mut f64,
        var_pclmr_i_dn4_slot: &mut f64,
        var_pclmr_i_dn5_slot: &mut f64,
        var_pclmr_i_dn6_slot: &mut f64,
        var_pclmr_i_dn7_slot: &mut f64,
        var_pclmr_i_dn8_slot: &mut f64,
        var_pclmr_i_dn9_slot: &mut f64,
        var_pclmr_i_rv_slot: &mut f64,
        var_pdibl1r_i_slot: &mut f64,
        var_pdibl1r_i_rv_slot: &mut f64,
        var_pdibl2r_i_slot: &mut f64,
        var_pdibl2r_i_rv_slot: &mut f64,
        var_pgidlb_i_slot: &mut f64,
        var_pgidlb_i_rv_slot: &mut f64,
        var_pgislb_i_slot: &mut f64,
        var_pgislb_i_rv_slot: &mut f64,
        var_ptwgr_i_slot: &mut f64,
        var_ptwgr_i_dn0_slot: &mut f64,
        var_ptwgr_i_dn10_slot: &mut f64,
        var_ptwgr_i_dn11_slot: &mut f64,
        var_ptwgr_i_dn13_slot: &mut f64,
        var_ptwgr_i_dn14_slot: &mut f64,
        var_ptwgr_i_dn2_slot: &mut f64,
        var_ptwgr_i_dn3_slot: &mut f64,
        var_ptwgr_i_dn4_slot: &mut f64,
        var_ptwgr_i_dn5_slot: &mut f64,
        var_ptwgr_i_dn6_slot: &mut f64,
        var_ptwgr_i_dn7_slot: &mut f64,
        var_ptwgr_i_dn8_slot: &mut f64,
        var_ptwgr_i_dn9_slot: &mut f64,
        var_ptwgr_i_rv_slot: &mut f64,
        var_vsat1r_i_slot: &mut f64,
        var_vsat1r_i_dn0_slot: &mut f64,
        var_vsat1r_i_dn10_slot: &mut f64,
        var_vsat1r_i_dn11_slot: &mut f64,
        var_vsat1r_i_dn13_slot: &mut f64,
        var_vsat1r_i_dn14_slot: &mut f64,
        var_vsat1r_i_dn2_slot: &mut f64,
        var_vsat1r_i_dn3_slot: &mut f64,
        var_vsat1r_i_dn4_slot: &mut f64,
        var_vsat1r_i_dn5_slot: &mut f64,
        var_vsat1r_i_dn6_slot: &mut f64,
        var_vsat1r_i_dn7_slot: &mut f64,
        var_vsat1r_i_dn8_slot: &mut f64,
        var_vsat1r_i_dn9_slot: &mut f64,
        var_vsat1r_i_rv_slot: &mut f64,
    ) {
        let mut var_agidlb_i: f64 = *var_agidlb_i_slot;
        let mut var_agidlb_i_rv: f64 = *var_agidlb_i_rv_slot;
        let mut var_agislb_i: f64 = *var_agislb_i_slot;
        let mut var_agislb_i_rv: f64 = *var_agislb_i_rv_slot;
        let mut var_bgidlb_i: f64 = *var_bgidlb_i_slot;
        let mut var_bgidlb_i_rv: f64 = *var_bgidlb_i_rv_slot;
        let mut var_bgislb_i: f64 = *var_bgislb_i_slot;
        let mut var_bgislb_i_rv: f64 = *var_bgislb_i_rv_slot;
        let mut var_cdscdr_i: f64 = *var_cdscdr_i_slot;
        let mut var_cdscdr_i_rv: f64 = *var_cdscdr_i_rv_slot;
        let mut var_cgidlb_i: f64 = *var_cgidlb_i_slot;
        let mut var_cgidlb_i_rv: f64 = *var_cgidlb_i_rv_slot;
        let mut var_cgislb_i: f64 = *var_cgislb_i_slot;
        let mut var_cgislb_i_rv: f64 = *var_cgislb_i_rv_slot;
        let mut var_citr_i: f64 = *var_citr_i_slot;
        let mut var_citr_i_rv: f64 = *var_citr_i_rv_slot;
        let mut var_dvtshiftr_i: f64 = *var_dvtshiftr_i_slot;
        let mut var_dvtshiftr_i_rv: f64 = *var_dvtshiftr_i_rv_slot;
        let mut var_egidlb_i: f64 = *var_egidlb_i_slot;
        let mut var_egidlb_i_rv: f64 = *var_egidlb_i_rv_slot;
        let mut var_egislb_i: f64 = *var_egislb_i_slot;
        let mut var_egislb_i_rv: f64 = *var_egislb_i_rv_slot;
        let mut var_eta0r_i: f64 = *var_eta0r_i_slot;
        let mut var_eta0r_i_rv: f64 = *var_eta0r_i_rv_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard44_rv: f64 = *var_guard44_rv_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard45_rv: f64 = *var_guard45_rv_slot;
        let mut var_k2sat1_i: f64 = *var_k2sat1_i_slot;
        let mut var_k2sat1_i_rv: f64 = *var_k2sat1_i_rv_slot;
        let mut var_k2sat_i: f64 = *var_k2sat_i_slot;
        let mut var_k2sat_i_rv: f64 = *var_k2sat_i_rv_slot;
        let mut var_mexpr_i: f64 = *var_mexpr_i_slot;
        let mut var_mexpr_i_dn0: f64 = *var_mexpr_i_dn0_slot;
        let mut var_mexpr_i_dn10: f64 = *var_mexpr_i_dn10_slot;
        let mut var_mexpr_i_dn11: f64 = *var_mexpr_i_dn11_slot;
        let mut var_mexpr_i_dn13: f64 = *var_mexpr_i_dn13_slot;
        let mut var_mexpr_i_dn14: f64 = *var_mexpr_i_dn14_slot;
        let mut var_mexpr_i_dn2: f64 = *var_mexpr_i_dn2_slot;
        let mut var_mexpr_i_dn3: f64 = *var_mexpr_i_dn3_slot;
        let mut var_mexpr_i_dn4: f64 = *var_mexpr_i_dn4_slot;
        let mut var_mexpr_i_dn5: f64 = *var_mexpr_i_dn5_slot;
        let mut var_mexpr_i_dn6: f64 = *var_mexpr_i_dn6_slot;
        let mut var_mexpr_i_dn7: f64 = *var_mexpr_i_dn7_slot;
        let mut var_mexpr_i_dn8: f64 = *var_mexpr_i_dn8_slot;
        let mut var_mexpr_i_dn9: f64 = *var_mexpr_i_dn9_slot;
        let mut var_mexpr_i_rv: f64 = *var_mexpr_i_rv_slot;
        let mut var_pclmr_i: f64 = *var_pclmr_i_slot;
        let mut var_pclmr_i_dn0: f64 = *var_pclmr_i_dn0_slot;
        let mut var_pclmr_i_dn10: f64 = *var_pclmr_i_dn10_slot;
        let mut var_pclmr_i_dn11: f64 = *var_pclmr_i_dn11_slot;
        let mut var_pclmr_i_dn13: f64 = *var_pclmr_i_dn13_slot;
        let mut var_pclmr_i_dn14: f64 = *var_pclmr_i_dn14_slot;
        let mut var_pclmr_i_dn2: f64 = *var_pclmr_i_dn2_slot;
        let mut var_pclmr_i_dn3: f64 = *var_pclmr_i_dn3_slot;
        let mut var_pclmr_i_dn4: f64 = *var_pclmr_i_dn4_slot;
        let mut var_pclmr_i_dn5: f64 = *var_pclmr_i_dn5_slot;
        let mut var_pclmr_i_dn6: f64 = *var_pclmr_i_dn6_slot;
        let mut var_pclmr_i_dn7: f64 = *var_pclmr_i_dn7_slot;
        let mut var_pclmr_i_dn8: f64 = *var_pclmr_i_dn8_slot;
        let mut var_pclmr_i_dn9: f64 = *var_pclmr_i_dn9_slot;
        let mut var_pclmr_i_rv: f64 = *var_pclmr_i_rv_slot;
        let mut var_pdibl1r_i: f64 = *var_pdibl1r_i_slot;
        let mut var_pdibl1r_i_rv: f64 = *var_pdibl1r_i_rv_slot;
        let mut var_pdibl2r_i: f64 = *var_pdibl2r_i_slot;
        let mut var_pdibl2r_i_rv: f64 = *var_pdibl2r_i_rv_slot;
        let mut var_pgidlb_i: f64 = *var_pgidlb_i_slot;
        let mut var_pgidlb_i_rv: f64 = *var_pgidlb_i_rv_slot;
        let mut var_pgislb_i: f64 = *var_pgislb_i_slot;
        let mut var_pgislb_i_rv: f64 = *var_pgislb_i_rv_slot;
        let mut var_ptwgr_i: f64 = *var_ptwgr_i_slot;
        let mut var_ptwgr_i_dn0: f64 = *var_ptwgr_i_dn0_slot;
        let mut var_ptwgr_i_dn10: f64 = *var_ptwgr_i_dn10_slot;
        let mut var_ptwgr_i_dn11: f64 = *var_ptwgr_i_dn11_slot;
        let mut var_ptwgr_i_dn13: f64 = *var_ptwgr_i_dn13_slot;
        let mut var_ptwgr_i_dn14: f64 = *var_ptwgr_i_dn14_slot;
        let mut var_ptwgr_i_dn2: f64 = *var_ptwgr_i_dn2_slot;
        let mut var_ptwgr_i_dn3: f64 = *var_ptwgr_i_dn3_slot;
        let mut var_ptwgr_i_dn4: f64 = *var_ptwgr_i_dn4_slot;
        let mut var_ptwgr_i_dn5: f64 = *var_ptwgr_i_dn5_slot;
        let mut var_ptwgr_i_dn6: f64 = *var_ptwgr_i_dn6_slot;
        let mut var_ptwgr_i_dn7: f64 = *var_ptwgr_i_dn7_slot;
        let mut var_ptwgr_i_dn8: f64 = *var_ptwgr_i_dn8_slot;
        let mut var_ptwgr_i_dn9: f64 = *var_ptwgr_i_dn9_slot;
        let mut var_ptwgr_i_rv: f64 = *var_ptwgr_i_rv_slot;
        let mut var_vsat1r_i: f64 = *var_vsat1r_i_slot;
        let mut var_vsat1r_i_dn0: f64 = *var_vsat1r_i_dn0_slot;
        let mut var_vsat1r_i_dn10: f64 = *var_vsat1r_i_dn10_slot;
        let mut var_vsat1r_i_dn11: f64 = *var_vsat1r_i_dn11_slot;
        let mut var_vsat1r_i_dn13: f64 = *var_vsat1r_i_dn13_slot;
        let mut var_vsat1r_i_dn14: f64 = *var_vsat1r_i_dn14_slot;
        let mut var_vsat1r_i_dn2: f64 = *var_vsat1r_i_dn2_slot;
        let mut var_vsat1r_i_dn3: f64 = *var_vsat1r_i_dn3_slot;
        let mut var_vsat1r_i_dn4: f64 = *var_vsat1r_i_dn4_slot;
        let mut var_vsat1r_i_dn5: f64 = *var_vsat1r_i_dn5_slot;
        let mut var_vsat1r_i_dn6: f64 = *var_vsat1r_i_dn6_slot;
        let mut var_vsat1r_i_dn7: f64 = *var_vsat1r_i_dn7_slot;
        let mut var_vsat1r_i_dn8: f64 = *var_vsat1r_i_dn8_slot;
        let mut var_vsat1r_i_dn9: f64 = *var_vsat1r_i_dn9_slot;
        let mut var_vsat1r_i_rv: f64 = *var_vsat1r_i_rv_slot;

        let (assign5870_e8468,) = {
    if ((var_guard42 != 0.0) && (var_guard43 != 0.0)) {
        let assign5870_e8449: f64 = (var_inv_l * p.p375);
        let assign5870_e8450: f64 = (p.p374 + assign5870_e8449);
        let assign5870_e8453: f64 = (var_inv_nfin * p.p376);
        let assign5870_e8454: f64 = (assign5870_e8450 + assign5870_e8453);
        let assign5870_e8457: f64 = (var_inv_lnfin * p.p377);
        let assign5870_e8458: f64 = (assign5870_e8454 + assign5870_e8457);
        let assign5870_e8461: f64 = (var_inv_w * p.p378);
        let assign5870_e8462: f64 = (assign5870_e8458 + assign5870_e8461);
        let assign5870_e8465: f64 = (var_inv_wl * p.p379);
        let assign5870_e8466: f64 = (assign5870_e8462 + assign5870_e8465);
        (assign5870_e8466,)
    } else {
        (var_k2sat_i,)
    }
};
        var_k2sat_i = assign5870_e8468;
        var_k2sat_i_rv = 0.0;

        let (assign5880_e8494,) = {
    if ((var_guard42 != 0.0) && (var_guard43 != 0.0)) {
        let assign5880_e8475: f64 = (var_inv_l * p.p381);
        let assign5880_e8476: f64 = (p.p380 + assign5880_e8475);
        let assign5880_e8479: f64 = (var_inv_nfin * p.p382);
        let assign5880_e8480: f64 = (assign5880_e8476 + assign5880_e8479);
        let assign5880_e8483: f64 = (var_inv_lnfin * p.p383);
        let assign5880_e8484: f64 = (assign5880_e8480 + assign5880_e8483);
        let assign5880_e8487: f64 = (var_inv_w * p.p384);
        let assign5880_e8488: f64 = (assign5880_e8484 + assign5880_e8487);
        let assign5880_e8491: f64 = (var_inv_wl * p.p385);
        let assign5880_e8492: f64 = (assign5880_e8488 + assign5880_e8491);
        (assign5880_e8492,)
    } else {
        (var_k2sat1_i,)
    }
};
        var_k2sat1_i = assign5880_e8494;
        var_k2sat1_i_rv = 0.0;

        let assign5890_e8513: f64 = if (((p.p70 == 2.0) || (p.p70 == 3.0)) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };
        var_guard44 = assign5890_e8513;
        var_guard44_rv = 0.0;

        let (assign5900_e8539,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5900_e8520: f64 = (var_inv_l * p.p1378);
        let assign5900_e8521: f64 = (p.p1377 + assign5900_e8520);
        let assign5900_e8524: f64 = (var_inv_nfin * p.p1379);
        let assign5900_e8525: f64 = (assign5900_e8521 + assign5900_e8524);
        let assign5900_e8528: f64 = (var_inv_lnfin * p.p1380);
        let assign5900_e8529: f64 = (assign5900_e8525 + assign5900_e8528);
        let assign5900_e8532: f64 = (var_inv_w * p.p1381);
        let assign5900_e8533: f64 = (assign5900_e8529 + assign5900_e8532);
        let assign5900_e8536: f64 = (var_inv_wl * p.p1382);
        let assign5900_e8537: f64 = (assign5900_e8533 + assign5900_e8536);
        (assign5900_e8537,)
    } else {
        (var_agidlb_i,)
    }
};
        var_agidlb_i = assign5900_e8539;
        var_agidlb_i_rv = 0.0;

        let (assign5910_e8565,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5910_e8546: f64 = (var_inv_l * p.p1384);
        let assign5910_e8547: f64 = (p.p1383 + assign5910_e8546);
        let assign5910_e8550: f64 = (var_inv_nfin * p.p1385);
        let assign5910_e8551: f64 = (assign5910_e8547 + assign5910_e8550);
        let assign5910_e8554: f64 = (var_inv_lnfin * p.p1386);
        let assign5910_e8555: f64 = (assign5910_e8551 + assign5910_e8554);
        let assign5910_e8558: f64 = (var_inv_w * p.p1387);
        let assign5910_e8559: f64 = (assign5910_e8555 + assign5910_e8558);
        let assign5910_e8562: f64 = (var_inv_wl * p.p1388);
        let assign5910_e8563: f64 = (assign5910_e8559 + assign5910_e8562);
        (assign5910_e8563,)
    } else {
        (var_bgidlb_i,)
    }
};
        var_bgidlb_i = assign5910_e8565;
        var_bgidlb_i_rv = 0.0;

        let (assign5920_e8591,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5920_e8572: f64 = (var_inv_l * p.p1390);
        let assign5920_e8573: f64 = (p.p1389 + assign5920_e8572);
        let assign5920_e8576: f64 = (var_inv_nfin * p.p1391);
        let assign5920_e8577: f64 = (assign5920_e8573 + assign5920_e8576);
        let assign5920_e8580: f64 = (var_inv_lnfin * p.p1392);
        let assign5920_e8581: f64 = (assign5920_e8577 + assign5920_e8580);
        let assign5920_e8584: f64 = (var_inv_w * p.p1393);
        let assign5920_e8585: f64 = (assign5920_e8581 + assign5920_e8584);
        let assign5920_e8588: f64 = (var_inv_wl * p.p1394);
        let assign5920_e8589: f64 = (assign5920_e8585 + assign5920_e8588);
        (assign5920_e8589,)
    } else {
        (var_cgidlb_i,)
    }
};
        var_cgidlb_i = assign5920_e8591;
        var_cgidlb_i_rv = 0.0;

        let (assign5930_e8617,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5930_e8598: f64 = (var_inv_l * p.p1396);
        let assign5930_e8599: f64 = (p.p1395 + assign5930_e8598);
        let assign5930_e8602: f64 = (var_inv_nfin * p.p1397);
        let assign5930_e8603: f64 = (assign5930_e8599 + assign5930_e8602);
        let assign5930_e8606: f64 = (var_inv_lnfin * p.p1398);
        let assign5930_e8607: f64 = (assign5930_e8603 + assign5930_e8606);
        let assign5930_e8610: f64 = (var_inv_w * p.p1399);
        let assign5930_e8611: f64 = (assign5930_e8607 + assign5930_e8610);
        let assign5930_e8614: f64 = (var_inv_wl * p.p1400);
        let assign5930_e8615: f64 = (assign5930_e8611 + assign5930_e8614);
        (assign5930_e8615,)
    } else {
        (var_egidlb_i,)
    }
};
        var_egidlb_i = assign5930_e8617;
        var_egidlb_i_rv = 0.0;

        let (assign5940_e8643,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5940_e8624: f64 = (var_inv_l * p.p1402);
        let assign5940_e8625: f64 = (p.p1401 + assign5940_e8624);
        let assign5940_e8628: f64 = (var_inv_nfin * p.p1403);
        let assign5940_e8629: f64 = (assign5940_e8625 + assign5940_e8628);
        let assign5940_e8632: f64 = (var_inv_lnfin * p.p1404);
        let assign5940_e8633: f64 = (assign5940_e8629 + assign5940_e8632);
        let assign5940_e8636: f64 = (var_inv_w * p.p1405);
        let assign5940_e8637: f64 = (assign5940_e8633 + assign5940_e8636);
        let assign5940_e8640: f64 = (var_inv_wl * p.p1406);
        let assign5940_e8641: f64 = (assign5940_e8637 + assign5940_e8640);
        (assign5940_e8641,)
    } else {
        (var_pgidlb_i,)
    }
};
        var_pgidlb_i = assign5940_e8643;
        var_pgidlb_i_rv = 0.0;

        let (assign5950_e8669,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5950_e8650: f64 = (var_inv_l * p.p1408);
        let assign5950_e8651: f64 = (p.p1407 + assign5950_e8650);
        let assign5950_e8654: f64 = (var_inv_nfin * p.p1409);
        let assign5950_e8655: f64 = (assign5950_e8651 + assign5950_e8654);
        let assign5950_e8658: f64 = (var_inv_lnfin * p.p1410);
        let assign5950_e8659: f64 = (assign5950_e8655 + assign5950_e8658);
        let assign5950_e8662: f64 = (var_inv_w * p.p1411);
        let assign5950_e8663: f64 = (assign5950_e8659 + assign5950_e8662);
        let assign5950_e8666: f64 = (var_inv_wl * p.p1412);
        let assign5950_e8667: f64 = (assign5950_e8663 + assign5950_e8666);
        (assign5950_e8667,)
    } else {
        (var_agislb_i,)
    }
};
        var_agislb_i = assign5950_e8669;
        var_agislb_i_rv = 0.0;

        let (assign5960_e8695,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5960_e8676: f64 = (var_inv_l * p.p1414);
        let assign5960_e8677: f64 = (p.p1413 + assign5960_e8676);
        let assign5960_e8680: f64 = (var_inv_nfin * p.p1415);
        let assign5960_e8681: f64 = (assign5960_e8677 + assign5960_e8680);
        let assign5960_e8684: f64 = (var_inv_lnfin * p.p1416);
        let assign5960_e8685: f64 = (assign5960_e8681 + assign5960_e8684);
        let assign5960_e8688: f64 = (var_inv_w * p.p1417);
        let assign5960_e8689: f64 = (assign5960_e8685 + assign5960_e8688);
        let assign5960_e8692: f64 = (var_inv_wl * p.p1418);
        let assign5960_e8693: f64 = (assign5960_e8689 + assign5960_e8692);
        (assign5960_e8693,)
    } else {
        (var_bgislb_i,)
    }
};
        var_bgislb_i = assign5960_e8695;
        var_bgislb_i_rv = 0.0;

        let (assign5970_e8721,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5970_e8702: f64 = (var_inv_l * p.p1420);
        let assign5970_e8703: f64 = (p.p1419 + assign5970_e8702);
        let assign5970_e8706: f64 = (var_inv_nfin * p.p1421);
        let assign5970_e8707: f64 = (assign5970_e8703 + assign5970_e8706);
        let assign5970_e8710: f64 = (var_inv_lnfin * p.p1422);
        let assign5970_e8711: f64 = (assign5970_e8707 + assign5970_e8710);
        let assign5970_e8714: f64 = (var_inv_w * p.p1423);
        let assign5970_e8715: f64 = (assign5970_e8711 + assign5970_e8714);
        let assign5970_e8718: f64 = (var_inv_wl * p.p1424);
        let assign5970_e8719: f64 = (assign5970_e8715 + assign5970_e8718);
        (assign5970_e8719,)
    } else {
        (var_cgislb_i,)
    }
};
        var_cgislb_i = assign5970_e8721;
        var_cgislb_i_rv = 0.0;

        let (assign5980_e8747,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5980_e8728: f64 = (var_inv_l * p.p1426);
        let assign5980_e8729: f64 = (p.p1425 + assign5980_e8728);
        let assign5980_e8732: f64 = (var_inv_nfin * p.p1427);
        let assign5980_e8733: f64 = (assign5980_e8729 + assign5980_e8732);
        let assign5980_e8736: f64 = (var_inv_lnfin * p.p1428);
        let assign5980_e8737: f64 = (assign5980_e8733 + assign5980_e8736);
        let assign5980_e8740: f64 = (var_inv_w * p.p1429);
        let assign5980_e8741: f64 = (assign5980_e8737 + assign5980_e8740);
        let assign5980_e8744: f64 = (var_inv_wl * p.p1430);
        let assign5980_e8745: f64 = (assign5980_e8741 + assign5980_e8744);
        (assign5980_e8745,)
    } else {
        (var_egislb_i,)
    }
};
        var_egislb_i = assign5980_e8747;
        var_egislb_i_rv = 0.0;

        let (assign5990_e8773,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5990_e8754: f64 = (var_inv_l * p.p1432);
        let assign5990_e8755: f64 = (p.p1431 + assign5990_e8754);
        let assign5990_e8758: f64 = (var_inv_nfin * p.p1433);
        let assign5990_e8759: f64 = (assign5990_e8755 + assign5990_e8758);
        let assign5990_e8762: f64 = (var_inv_lnfin * p.p1434);
        let assign5990_e8763: f64 = (assign5990_e8759 + assign5990_e8762);
        let assign5990_e8766: f64 = (var_inv_w * p.p1435);
        let assign5990_e8767: f64 = (assign5990_e8763 + assign5990_e8766);
        let assign5990_e8770: f64 = (var_inv_wl * p.p1436);
        let assign5990_e8771: f64 = (assign5990_e8767 + assign5990_e8770);
        (assign5990_e8771,)
    } else {
        (var_pgislb_i,)
    }
};
        var_pgislb_i = assign5990_e8773;
        var_pgislb_i_rv = 0.0;

        let assign6000_e8776: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard45 = assign6000_e8776;
        var_guard45_rv = 0.0;

        let (assign6010_e8800,) = {
    if (var_guard45 != 0.0) {
        let assign6010_e8781: f64 = (var_inv_l * p.p213);
        let assign6010_e8782: f64 = (p.p212 + assign6010_e8781);
        let assign6010_e8785: f64 = (var_inv_nfin * p.p214);
        let assign6010_e8786: f64 = (assign6010_e8782 + assign6010_e8785);
        let assign6010_e8789: f64 = (var_inv_lnfin * p.p215);
        let assign6010_e8790: f64 = (assign6010_e8786 + assign6010_e8789);
        let assign6010_e8793: f64 = (var_inv_w * p.p216);
        let assign6010_e8794: f64 = (assign6010_e8790 + assign6010_e8793);
        let assign6010_e8797: f64 = (var_inv_wl * p.p217);
        let assign6010_e8798: f64 = (assign6010_e8794 + assign6010_e8797);
        (assign6010_e8798,)
    } else {
        (var_cdscdr_i,)
    }
};
        var_cdscdr_i = assign6010_e8800;
        var_cdscdr_i_rv = 0.0;

        let (assign6020_e8824,) = {
    if (var_guard45 != 0.0) {
        let assign6020_e8805: f64 = (var_inv_l * p.p195);
        let assign6020_e8806: f64 = (p.p194 + assign6020_e8805);
        let assign6020_e8809: f64 = (var_inv_nfin * p.p196);
        let assign6020_e8810: f64 = (assign6020_e8806 + assign6020_e8809);
        let assign6020_e8813: f64 = (var_inv_lnfin * p.p197);
        let assign6020_e8814: f64 = (assign6020_e8810 + assign6020_e8813);
        let assign6020_e8817: f64 = (var_inv_w * p.p198);
        let assign6020_e8818: f64 = (assign6020_e8814 + assign6020_e8817);
        let assign6020_e8821: f64 = (var_inv_wl * p.p199);
        let assign6020_e8822: f64 = (assign6020_e8818 + assign6020_e8821);
        (assign6020_e8822,)
    } else {
        (var_citr_i,)
    }
};
        var_citr_i = assign6020_e8824;
        var_citr_i_rv = 0.0;

        let (assign6030_e8848,) = {
    if (var_guard45 != 0.0) {
        let assign6030_e8829: f64 = (var_inv_l * p.p255);
        let assign6030_e8830: f64 = (p.p254 + assign6030_e8829);
        let assign6030_e8833: f64 = (var_inv_nfin * p.p256);
        let assign6030_e8834: f64 = (assign6030_e8830 + assign6030_e8833);
        let assign6030_e8837: f64 = (var_inv_lnfin * p.p257);
        let assign6030_e8838: f64 = (assign6030_e8834 + assign6030_e8837);
        let assign6030_e8841: f64 = (var_inv_w * p.p258);
        let assign6030_e8842: f64 = (assign6030_e8838 + assign6030_e8841);
        let assign6030_e8845: f64 = (var_inv_wl * p.p259);
        let assign6030_e8846: f64 = (assign6030_e8842 + assign6030_e8845);
        (assign6030_e8846,)
    } else {
        (var_eta0r_i,)
    }
};
        var_eta0r_i = assign6030_e8848;
        var_eta0r_i_rv = 0.0;

        let (assign6040_e8872, assign6040_e8872_d_n0, assign6040_e8872_d_n2, assign6040_e8872_d_n3, assign6040_e8872_d_n4, assign6040_e8872_d_n5, assign6040_e8872_d_n6, assign6040_e8872_d_n7, assign6040_e8872_d_n8, assign6040_e8872_d_n9, assign6040_e8872_d_n10, assign6040_e8872_d_n11, assign6040_e8872_d_n13, assign6040_e8872_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6040_e8853: f64 = (var_inv_l * p.p474);
        let assign6040_e8854: f64 = (p.p473 + assign6040_e8853);
        let assign6040_e8857: f64 = (var_inv_nfin * p.p475);
        let assign6040_e8858: f64 = (assign6040_e8854 + assign6040_e8857);
        let assign6040_e8861: f64 = (var_inv_lnfin * p.p476);
        let assign6040_e8862: f64 = (assign6040_e8858 + assign6040_e8861);
        let assign6040_e8865: f64 = (var_inv_w * p.p477);
        let assign6040_e8866: f64 = (assign6040_e8862 + assign6040_e8865);
        let assign6040_e8869: f64 = (var_inv_wl * p.p478);
        let assign6040_e8870: f64 = (assign6040_e8866 + assign6040_e8869);
        (assign6040_e8870, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat1r_i, var_vsat1r_i_dn0, var_vsat1r_i_dn2, var_vsat1r_i_dn3, var_vsat1r_i_dn4, var_vsat1r_i_dn5, var_vsat1r_i_dn6, var_vsat1r_i_dn7, var_vsat1r_i_dn8, var_vsat1r_i_dn9, var_vsat1r_i_dn10, var_vsat1r_i_dn11, var_vsat1r_i_dn13, var_vsat1r_i_dn14,)
    }
};
        var_vsat1r_i = assign6040_e8872;
        var_vsat1r_i_dn0 = assign6040_e8872_d_n0;
        var_vsat1r_i_dn2 = assign6040_e8872_d_n2;
        var_vsat1r_i_dn3 = assign6040_e8872_d_n3;
        var_vsat1r_i_dn4 = assign6040_e8872_d_n4;
        var_vsat1r_i_dn5 = assign6040_e8872_d_n5;
        var_vsat1r_i_dn6 = assign6040_e8872_d_n6;
        var_vsat1r_i_dn7 = assign6040_e8872_d_n7;
        var_vsat1r_i_dn8 = assign6040_e8872_d_n8;
        var_vsat1r_i_dn9 = assign6040_e8872_d_n9;
        var_vsat1r_i_dn10 = assign6040_e8872_d_n10;
        var_vsat1r_i_dn11 = assign6040_e8872_d_n11;
        var_vsat1r_i_dn13 = assign6040_e8872_d_n13;
        var_vsat1r_i_dn14 = assign6040_e8872_d_n14;
        var_vsat1r_i_rv = 0.0;

        let (assign6050_e8896, assign6050_e8896_d_n0, assign6050_e8896_d_n2, assign6050_e8896_d_n3, assign6050_e8896_d_n4, assign6050_e8896_d_n5, assign6050_e8896_d_n6, assign6050_e8896_d_n7, assign6050_e8896_d_n8, assign6050_e8896_d_n9, assign6050_e8896_d_n10, assign6050_e8896_d_n11, assign6050_e8896_d_n13, assign6050_e8896_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6050_e8877: f64 = (var_inv_l * p.p538);
        let assign6050_e8878: f64 = (p.p537 + assign6050_e8877);
        let assign6050_e8881: f64 = (var_inv_nfin * p.p539);
        let assign6050_e8882: f64 = (assign6050_e8878 + assign6050_e8881);
        let assign6050_e8885: f64 = (var_inv_lnfin * p.p540);
        let assign6050_e8886: f64 = (assign6050_e8882 + assign6050_e8885);
        let assign6050_e8889: f64 = (var_inv_w * p.p541);
        let assign6050_e8890: f64 = (assign6050_e8886 + assign6050_e8889);
        let assign6050_e8893: f64 = (var_inv_wl * p.p542);
        let assign6050_e8894: f64 = (assign6050_e8890 + assign6050_e8893);
        (assign6050_e8894, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mexpr_i, var_mexpr_i_dn0, var_mexpr_i_dn2, var_mexpr_i_dn3, var_mexpr_i_dn4, var_mexpr_i_dn5, var_mexpr_i_dn6, var_mexpr_i_dn7, var_mexpr_i_dn8, var_mexpr_i_dn9, var_mexpr_i_dn10, var_mexpr_i_dn11, var_mexpr_i_dn13, var_mexpr_i_dn14,)
    }
};
        var_mexpr_i = assign6050_e8896;
        var_mexpr_i_dn0 = assign6050_e8896_d_n0;
        var_mexpr_i_dn2 = assign6050_e8896_d_n2;
        var_mexpr_i_dn3 = assign6050_e8896_d_n3;
        var_mexpr_i_dn4 = assign6050_e8896_d_n4;
        var_mexpr_i_dn5 = assign6050_e8896_d_n5;
        var_mexpr_i_dn6 = assign6050_e8896_d_n6;
        var_mexpr_i_dn7 = assign6050_e8896_d_n7;
        var_mexpr_i_dn8 = assign6050_e8896_d_n8;
        var_mexpr_i_dn9 = assign6050_e8896_d_n9;
        var_mexpr_i_dn10 = assign6050_e8896_d_n10;
        var_mexpr_i_dn11 = assign6050_e8896_d_n11;
        var_mexpr_i_dn13 = assign6050_e8896_d_n13;
        var_mexpr_i_dn14 = assign6050_e8896_d_n14;
        var_mexpr_i_rv = 0.0;

        let (assign6060_e8920, assign6060_e8920_d_n0, assign6060_e8920_d_n2, assign6060_e8920_d_n3, assign6060_e8920_d_n4, assign6060_e8920_d_n5, assign6060_e8920_d_n6, assign6060_e8920_d_n7, assign6060_e8920_d_n8, assign6060_e8920_d_n9, assign6060_e8920_d_n10, assign6060_e8920_d_n11, assign6060_e8920_d_n13, assign6060_e8920_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6060_e8901: f64 = (var_inv_l * p.p550);
        let assign6060_e8902: f64 = (p.p549 + assign6060_e8901);
        let assign6060_e8905: f64 = (var_inv_nfin * p.p551);
        let assign6060_e8906: f64 = (assign6060_e8902 + assign6060_e8905);
        let assign6060_e8909: f64 = (var_inv_lnfin * p.p552);
        let assign6060_e8910: f64 = (assign6060_e8906 + assign6060_e8909);
        let assign6060_e8913: f64 = (var_inv_w * p.p553);
        let assign6060_e8914: f64 = (assign6060_e8910 + assign6060_e8913);
        let assign6060_e8917: f64 = (var_inv_wl * p.p554);
        let assign6060_e8918: f64 = (assign6060_e8914 + assign6060_e8917);
        (assign6060_e8918, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptwgr_i, var_ptwgr_i_dn0, var_ptwgr_i_dn2, var_ptwgr_i_dn3, var_ptwgr_i_dn4, var_ptwgr_i_dn5, var_ptwgr_i_dn6, var_ptwgr_i_dn7, var_ptwgr_i_dn8, var_ptwgr_i_dn9, var_ptwgr_i_dn10, var_ptwgr_i_dn11, var_ptwgr_i_dn13, var_ptwgr_i_dn14,)
    }
};
        var_ptwgr_i = assign6060_e8920;
        var_ptwgr_i_dn0 = assign6060_e8920_d_n0;
        var_ptwgr_i_dn2 = assign6060_e8920_d_n2;
        var_ptwgr_i_dn3 = assign6060_e8920_d_n3;
        var_ptwgr_i_dn4 = assign6060_e8920_d_n4;
        var_ptwgr_i_dn5 = assign6060_e8920_d_n5;
        var_ptwgr_i_dn6 = assign6060_e8920_d_n6;
        var_ptwgr_i_dn7 = assign6060_e8920_d_n7;
        var_ptwgr_i_dn8 = assign6060_e8920_d_n8;
        var_ptwgr_i_dn9 = assign6060_e8920_d_n9;
        var_ptwgr_i_dn10 = assign6060_e8920_d_n10;
        var_ptwgr_i_dn11 = assign6060_e8920_d_n11;
        var_ptwgr_i_dn13 = assign6060_e8920_d_n13;
        var_ptwgr_i_dn14 = assign6060_e8920_d_n14;
        var_ptwgr_i_rv = 0.0;

        let (assign6070_e8944,) = {
    if (var_guard45 != 0.0) {
        let assign6070_e8925: f64 = (var_inv_l * p.p998);
        let assign6070_e8926: f64 = (p.p997 + assign6070_e8925);
        let assign6070_e8929: f64 = (var_inv_nfin * p.p999);
        let assign6070_e8930: f64 = (assign6070_e8926 + assign6070_e8929);
        let assign6070_e8933: f64 = (var_inv_lnfin * p.p1000);
        let assign6070_e8934: f64 = (assign6070_e8930 + assign6070_e8933);
        let assign6070_e8937: f64 = (var_inv_w * p.p1001);
        let assign6070_e8938: f64 = (assign6070_e8934 + assign6070_e8937);
        let assign6070_e8941: f64 = (var_inv_wl * p.p1002);
        let assign6070_e8942: f64 = (assign6070_e8938 + assign6070_e8941);
        (assign6070_e8942,)
    } else {
        (var_pdibl1r_i,)
    }
};
        var_pdibl1r_i = assign6070_e8944;
        var_pdibl1r_i_rv = 0.0;

        let (assign6080_e8968,) = {
    if (var_guard45 != 0.0) {
        let assign6080_e8949: f64 = (var_inv_l * p.p1004);
        let assign6080_e8950: f64 = (p.p1003 + assign6080_e8949);
        let assign6080_e8953: f64 = (var_inv_nfin * p.p1005);
        let assign6080_e8954: f64 = (assign6080_e8950 + assign6080_e8953);
        let assign6080_e8957: f64 = (var_inv_lnfin * p.p1006);
        let assign6080_e8958: f64 = (assign6080_e8954 + assign6080_e8957);
        let assign6080_e8961: f64 = (var_inv_w * p.p1007);
        let assign6080_e8962: f64 = (assign6080_e8958 + assign6080_e8961);
        let assign6080_e8965: f64 = (var_inv_wl * p.p1008);
        let assign6080_e8966: f64 = (assign6080_e8962 + assign6080_e8965);
        (assign6080_e8966,)
    } else {
        (var_pdibl2r_i,)
    }
};
        var_pdibl2r_i = assign6080_e8968;
        var_pdibl2r_i_rv = 0.0;

        let (assign6090_e8992, assign6090_e8992_d_n0, assign6090_e8992_d_n2, assign6090_e8992_d_n3, assign6090_e8992_d_n4, assign6090_e8992_d_n5, assign6090_e8992_d_n6, assign6090_e8992_d_n7, assign6090_e8992_d_n8, assign6090_e8992_d_n9, assign6090_e8992_d_n10, assign6090_e8992_d_n11, assign6090_e8992_d_n13, assign6090_e8992_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6090_e8973: f64 = (var_inv_l * p.p1033);
        let assign6090_e8974: f64 = (p.p1032 + assign6090_e8973);
        let assign6090_e8977: f64 = (var_inv_nfin * p.p1034);
        let assign6090_e8978: f64 = (assign6090_e8974 + assign6090_e8977);
        let assign6090_e8981: f64 = (var_inv_lnfin * p.p1035);
        let assign6090_e8982: f64 = (assign6090_e8978 + assign6090_e8981);
        let assign6090_e8985: f64 = (var_inv_w * p.p1036);
        let assign6090_e8986: f64 = (assign6090_e8982 + assign6090_e8985);
        let assign6090_e8989: f64 = (var_inv_wl * p.p1037);
        let assign6090_e8990: f64 = (assign6090_e8986 + assign6090_e8989);
        (assign6090_e8990, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pclmr_i, var_pclmr_i_dn0, var_pclmr_i_dn2, var_pclmr_i_dn3, var_pclmr_i_dn4, var_pclmr_i_dn5, var_pclmr_i_dn6, var_pclmr_i_dn7, var_pclmr_i_dn8, var_pclmr_i_dn9, var_pclmr_i_dn10, var_pclmr_i_dn11, var_pclmr_i_dn13, var_pclmr_i_dn14,)
    }
};
        var_pclmr_i = assign6090_e8992;
        var_pclmr_i_dn0 = assign6090_e8992_d_n0;
        var_pclmr_i_dn2 = assign6090_e8992_d_n2;
        var_pclmr_i_dn3 = assign6090_e8992_d_n3;
        var_pclmr_i_dn4 = assign6090_e8992_d_n4;
        var_pclmr_i_dn5 = assign6090_e8992_d_n5;
        var_pclmr_i_dn6 = assign6090_e8992_d_n6;
        var_pclmr_i_dn7 = assign6090_e8992_d_n7;
        var_pclmr_i_dn8 = assign6090_e8992_d_n8;
        var_pclmr_i_dn9 = assign6090_e8992_d_n9;
        var_pclmr_i_dn10 = assign6090_e8992_d_n10;
        var_pclmr_i_dn11 = assign6090_e8992_d_n11;
        var_pclmr_i_dn13 = assign6090_e8992_d_n13;
        var_pclmr_i_dn14 = assign6090_e8992_d_n14;
        var_pclmr_i_rv = 0.0;

        let (assign6100_e9016,) = {
    if (var_guard45 != 0.0) {
        let assign6100_e8997: f64 = (var_inv_l * p.p291);
        let assign6100_e8998: f64 = (p.p290 + assign6100_e8997);
        let assign6100_e9001: f64 = (var_inv_nfin * p.p292);
        let assign6100_e9002: f64 = (assign6100_e8998 + assign6100_e9001);
        let assign6100_e9005: f64 = (var_inv_lnfin * p.p293);
        let assign6100_e9006: f64 = (assign6100_e9002 + assign6100_e9005);
        let assign6100_e9009: f64 = (var_inv_w * p.p294);
        let assign6100_e9010: f64 = (assign6100_e9006 + assign6100_e9009);
        let assign6100_e9013: f64 = (var_inv_wl * p.p295);
        let assign6100_e9014: f64 = (assign6100_e9010 + assign6100_e9013);
        (assign6100_e9014,)
    } else {
        (var_dvtshiftr_i,)
    }
};
        var_dvtshiftr_i = assign6100_e9016;
        var_dvtshiftr_i_rv = 0.0;

        *var_agidlb_i_slot = var_agidlb_i;
        *var_agidlb_i_rv_slot = var_agidlb_i_rv;
        *var_agislb_i_slot = var_agislb_i;
        *var_agislb_i_rv_slot = var_agislb_i_rv;
        *var_bgidlb_i_slot = var_bgidlb_i;
        *var_bgidlb_i_rv_slot = var_bgidlb_i_rv;
        *var_bgislb_i_slot = var_bgislb_i;
        *var_bgislb_i_rv_slot = var_bgislb_i_rv;
        *var_cdscdr_i_slot = var_cdscdr_i;
        *var_cdscdr_i_rv_slot = var_cdscdr_i_rv;
        *var_cgidlb_i_slot = var_cgidlb_i;
        *var_cgidlb_i_rv_slot = var_cgidlb_i_rv;
        *var_cgislb_i_slot = var_cgislb_i;
        *var_cgislb_i_rv_slot = var_cgislb_i_rv;
        *var_citr_i_slot = var_citr_i;
        *var_citr_i_rv_slot = var_citr_i_rv;
        *var_dvtshiftr_i_slot = var_dvtshiftr_i;
        *var_dvtshiftr_i_rv_slot = var_dvtshiftr_i_rv;
        *var_egidlb_i_slot = var_egidlb_i;
        *var_egidlb_i_rv_slot = var_egidlb_i_rv;
        *var_egislb_i_slot = var_egislb_i;
        *var_egislb_i_rv_slot = var_egislb_i_rv;
        *var_eta0r_i_slot = var_eta0r_i;
        *var_eta0r_i_rv_slot = var_eta0r_i_rv;
        *var_guard44_slot = var_guard44;
        *var_guard44_rv_slot = var_guard44_rv;
        *var_guard45_slot = var_guard45;
        *var_guard45_rv_slot = var_guard45_rv;
        *var_k2sat1_i_slot = var_k2sat1_i;
        *var_k2sat1_i_rv_slot = var_k2sat1_i_rv;
        *var_k2sat_i_slot = var_k2sat_i;
        *var_k2sat_i_rv_slot = var_k2sat_i_rv;
        *var_mexpr_i_slot = var_mexpr_i;
        *var_mexpr_i_dn0_slot = var_mexpr_i_dn0;
        *var_mexpr_i_dn10_slot = var_mexpr_i_dn10;
        *var_mexpr_i_dn11_slot = var_mexpr_i_dn11;
        *var_mexpr_i_dn13_slot = var_mexpr_i_dn13;
        *var_mexpr_i_dn14_slot = var_mexpr_i_dn14;
        *var_mexpr_i_dn2_slot = var_mexpr_i_dn2;
        *var_mexpr_i_dn3_slot = var_mexpr_i_dn3;
        *var_mexpr_i_dn4_slot = var_mexpr_i_dn4;
        *var_mexpr_i_dn5_slot = var_mexpr_i_dn5;
        *var_mexpr_i_dn6_slot = var_mexpr_i_dn6;
        *var_mexpr_i_dn7_slot = var_mexpr_i_dn7;
        *var_mexpr_i_dn8_slot = var_mexpr_i_dn8;
        *var_mexpr_i_dn9_slot = var_mexpr_i_dn9;
        *var_mexpr_i_rv_slot = var_mexpr_i_rv;
        *var_pclmr_i_slot = var_pclmr_i;
        *var_pclmr_i_dn0_slot = var_pclmr_i_dn0;
        *var_pclmr_i_dn10_slot = var_pclmr_i_dn10;
        *var_pclmr_i_dn11_slot = var_pclmr_i_dn11;
        *var_pclmr_i_dn13_slot = var_pclmr_i_dn13;
        *var_pclmr_i_dn14_slot = var_pclmr_i_dn14;
        *var_pclmr_i_dn2_slot = var_pclmr_i_dn2;
        *var_pclmr_i_dn3_slot = var_pclmr_i_dn3;
        *var_pclmr_i_dn4_slot = var_pclmr_i_dn4;
        *var_pclmr_i_dn5_slot = var_pclmr_i_dn5;
        *var_pclmr_i_dn6_slot = var_pclmr_i_dn6;
        *var_pclmr_i_dn7_slot = var_pclmr_i_dn7;
        *var_pclmr_i_dn8_slot = var_pclmr_i_dn8;
        *var_pclmr_i_dn9_slot = var_pclmr_i_dn9;
        *var_pclmr_i_rv_slot = var_pclmr_i_rv;
        *var_pdibl1r_i_slot = var_pdibl1r_i;
        *var_pdibl1r_i_rv_slot = var_pdibl1r_i_rv;
        *var_pdibl2r_i_slot = var_pdibl2r_i;
        *var_pdibl2r_i_rv_slot = var_pdibl2r_i_rv;
        *var_pgidlb_i_slot = var_pgidlb_i;
        *var_pgidlb_i_rv_slot = var_pgidlb_i_rv;
        *var_pgislb_i_slot = var_pgislb_i;
        *var_pgislb_i_rv_slot = var_pgislb_i_rv;
        *var_ptwgr_i_slot = var_ptwgr_i;
        *var_ptwgr_i_dn0_slot = var_ptwgr_i_dn0;
        *var_ptwgr_i_dn10_slot = var_ptwgr_i_dn10;
        *var_ptwgr_i_dn11_slot = var_ptwgr_i_dn11;
        *var_ptwgr_i_dn13_slot = var_ptwgr_i_dn13;
        *var_ptwgr_i_dn14_slot = var_ptwgr_i_dn14;
        *var_ptwgr_i_dn2_slot = var_ptwgr_i_dn2;
        *var_ptwgr_i_dn3_slot = var_ptwgr_i_dn3;
        *var_ptwgr_i_dn4_slot = var_ptwgr_i_dn4;
        *var_ptwgr_i_dn5_slot = var_ptwgr_i_dn5;
        *var_ptwgr_i_dn6_slot = var_ptwgr_i_dn6;
        *var_ptwgr_i_dn7_slot = var_ptwgr_i_dn7;
        *var_ptwgr_i_dn8_slot = var_ptwgr_i_dn8;
        *var_ptwgr_i_dn9_slot = var_ptwgr_i_dn9;
        *var_ptwgr_i_rv_slot = var_ptwgr_i_rv;
        *var_vsat1r_i_slot = var_vsat1r_i;
        *var_vsat1r_i_dn0_slot = var_vsat1r_i_dn0;
        *var_vsat1r_i_dn10_slot = var_vsat1r_i_dn10;
        *var_vsat1r_i_dn11_slot = var_vsat1r_i_dn11;
        *var_vsat1r_i_dn13_slot = var_vsat1r_i_dn13;
        *var_vsat1r_i_dn14_slot = var_vsat1r_i_dn14;
        *var_vsat1r_i_dn2_slot = var_vsat1r_i_dn2;
        *var_vsat1r_i_dn3_slot = var_vsat1r_i_dn3;
        *var_vsat1r_i_dn4_slot = var_vsat1r_i_dn4;
        *var_vsat1r_i_dn5_slot = var_vsat1r_i_dn5;
        *var_vsat1r_i_dn6_slot = var_vsat1r_i_dn6;
        *var_vsat1r_i_dn7_slot = var_vsat1r_i_dn7;
        *var_vsat1r_i_dn8_slot = var_vsat1r_i_dn8;
        *var_vsat1r_i_dn9_slot = var_vsat1r_i_dn9;
        *var_vsat1r_i_rv_slot = var_vsat1r_i_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_guard45: f64,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_atr_i_slot: &mut f64,
        var_atr_i_rv_slot: &mut f64,
        var_eur_i_slot: &mut f64,
        var_eur_i_dn0_slot: &mut f64,
        var_eur_i_dn10_slot: &mut f64,
        var_eur_i_dn11_slot: &mut f64,
        var_eur_i_dn13_slot: &mut f64,
        var_eur_i_dn14_slot: &mut f64,
        var_eur_i_dn2_slot: &mut f64,
        var_eur_i_dn3_slot: &mut f64,
        var_eur_i_dn4_slot: &mut f64,
        var_eur_i_dn5_slot: &mut f64,
        var_eur_i_dn6_slot: &mut f64,
        var_eur_i_dn7_slot: &mut f64,
        var_eur_i_dn8_slot: &mut f64,
        var_eur_i_dn9_slot: &mut f64,
        var_eur_i_rv_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard46_rv_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard47_rv_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard48_rv_slot: &mut f64,
        var_ksativr_i_slot: &mut f64,
        var_ksativr_i_rv_slot: &mut f64,
        var_u0cv_i_slot: &mut f64,
        var_u0cv_i_dn0_slot: &mut f64,
        var_u0cv_i_dn10_slot: &mut f64,
        var_u0cv_i_dn11_slot: &mut f64,
        var_u0cv_i_dn13_slot: &mut f64,
        var_u0cv_i_dn14_slot: &mut f64,
        var_u0cv_i_dn2_slot: &mut f64,
        var_u0cv_i_dn3_slot: &mut f64,
        var_u0cv_i_dn4_slot: &mut f64,
        var_u0cv_i_dn5_slot: &mut f64,
        var_u0cv_i_dn6_slot: &mut f64,
        var_u0cv_i_dn7_slot: &mut f64,
        var_u0cv_i_dn8_slot: &mut f64,
        var_u0cv_i_dn9_slot: &mut f64,
        var_u0cv_i_rv_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_dn0_slot: &mut f64,
        var_u0r_i_dn10_slot: &mut f64,
        var_u0r_i_dn11_slot: &mut f64,
        var_u0r_i_dn13_slot: &mut f64,
        var_u0r_i_dn14_slot: &mut f64,
        var_u0r_i_dn2_slot: &mut f64,
        var_u0r_i_dn3_slot: &mut f64,
        var_u0r_i_dn4_slot: &mut f64,
        var_u0r_i_dn5_slot: &mut f64,
        var_u0r_i_dn6_slot: &mut f64,
        var_u0r_i_dn7_slot: &mut f64,
        var_u0r_i_dn8_slot: &mut f64,
        var_u0r_i_dn9_slot: &mut f64,
        var_u0r_i_rv_slot: &mut f64,
        var_ua1r_i_slot: &mut f64,
        var_ua1r_i_rv_slot: &mut f64,
        var_uacv_i_slot: &mut f64,
        var_uacv_i_rv_slot: &mut f64,
        var_uar_i_slot: &mut f64,
        var_uar_i_dn0_slot: &mut f64,
        var_uar_i_dn10_slot: &mut f64,
        var_uar_i_dn11_slot: &mut f64,
        var_uar_i_dn13_slot: &mut f64,
        var_uar_i_dn14_slot: &mut f64,
        var_uar_i_dn2_slot: &mut f64,
        var_uar_i_dn3_slot: &mut f64,
        var_uar_i_dn4_slot: &mut f64,
        var_uar_i_dn5_slot: &mut f64,
        var_uar_i_dn6_slot: &mut f64,
        var_uar_i_dn7_slot: &mut f64,
        var_uar_i_dn8_slot: &mut f64,
        var_uar_i_dn9_slot: &mut f64,
        var_uar_i_rv_slot: &mut f64,
        var_uc1r_i_slot: &mut f64,
        var_uc1r_i_rv_slot: &mut f64,
        var_ucr_i_slot: &mut f64,
        var_ucr_i_rv_slot: &mut f64,
        var_ud1r_i_slot: &mut f64,
        var_ud1r_i_rv_slot: &mut f64,
        var_udcv_i_slot: &mut f64,
        var_udcv_i_rv_slot: &mut f64,
        var_udr_i_slot: &mut f64,
        var_udr_i_dn0_slot: &mut f64,
        var_udr_i_dn10_slot: &mut f64,
        var_udr_i_dn11_slot: &mut f64,
        var_udr_i_dn13_slot: &mut f64,
        var_udr_i_dn14_slot: &mut f64,
        var_udr_i_dn2_slot: &mut f64,
        var_udr_i_dn3_slot: &mut f64,
        var_udr_i_dn4_slot: &mut f64,
        var_udr_i_dn5_slot: &mut f64,
        var_udr_i_dn6_slot: &mut f64,
        var_udr_i_dn7_slot: &mut f64,
        var_udr_i_dn8_slot: &mut f64,
        var_udr_i_dn9_slot: &mut f64,
        var_udr_i_rv_slot: &mut f64,
        var_upr_i_slot: &mut f64,
        var_upr_i_rv_slot: &mut f64,
        var_ute1cv_i_slot: &mut f64,
        var_ute1cv_i_rv_slot: &mut f64,
        var_utecv_i_slot: &mut f64,
        var_utecv_i_rv_slot: &mut f64,
        var_uter_i_slot: &mut f64,
        var_uter_i_rv_slot: &mut f64,
        var_utlr_i_slot: &mut f64,
        var_utlr_i_rv_slot: &mut f64,
        var_vsatr_i_slot: &mut f64,
        var_vsatr_i_rv_slot: &mut f64,
    ) {
        let mut var_atr_i: f64 = *var_atr_i_slot;
        let mut var_atr_i_rv: f64 = *var_atr_i_rv_slot;
        let mut var_eur_i: f64 = *var_eur_i_slot;
        let mut var_eur_i_dn0: f64 = *var_eur_i_dn0_slot;
        let mut var_eur_i_dn10: f64 = *var_eur_i_dn10_slot;
        let mut var_eur_i_dn11: f64 = *var_eur_i_dn11_slot;
        let mut var_eur_i_dn13: f64 = *var_eur_i_dn13_slot;
        let mut var_eur_i_dn14: f64 = *var_eur_i_dn14_slot;
        let mut var_eur_i_dn2: f64 = *var_eur_i_dn2_slot;
        let mut var_eur_i_dn3: f64 = *var_eur_i_dn3_slot;
        let mut var_eur_i_dn4: f64 = *var_eur_i_dn4_slot;
        let mut var_eur_i_dn5: f64 = *var_eur_i_dn5_slot;
        let mut var_eur_i_dn6: f64 = *var_eur_i_dn6_slot;
        let mut var_eur_i_dn7: f64 = *var_eur_i_dn7_slot;
        let mut var_eur_i_dn8: f64 = *var_eur_i_dn8_slot;
        let mut var_eur_i_dn9: f64 = *var_eur_i_dn9_slot;
        let mut var_eur_i_rv: f64 = *var_eur_i_rv_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard46_rv: f64 = *var_guard46_rv_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard47_rv: f64 = *var_guard47_rv_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard48_rv: f64 = *var_guard48_rv_slot;
        let mut var_ksativr_i: f64 = *var_ksativr_i_slot;
        let mut var_ksativr_i_rv: f64 = *var_ksativr_i_rv_slot;
        let mut var_u0cv_i: f64 = *var_u0cv_i_slot;
        let mut var_u0cv_i_dn0: f64 = *var_u0cv_i_dn0_slot;
        let mut var_u0cv_i_dn10: f64 = *var_u0cv_i_dn10_slot;
        let mut var_u0cv_i_dn11: f64 = *var_u0cv_i_dn11_slot;
        let mut var_u0cv_i_dn13: f64 = *var_u0cv_i_dn13_slot;
        let mut var_u0cv_i_dn14: f64 = *var_u0cv_i_dn14_slot;
        let mut var_u0cv_i_dn2: f64 = *var_u0cv_i_dn2_slot;
        let mut var_u0cv_i_dn3: f64 = *var_u0cv_i_dn3_slot;
        let mut var_u0cv_i_dn4: f64 = *var_u0cv_i_dn4_slot;
        let mut var_u0cv_i_dn5: f64 = *var_u0cv_i_dn5_slot;
        let mut var_u0cv_i_dn6: f64 = *var_u0cv_i_dn6_slot;
        let mut var_u0cv_i_dn7: f64 = *var_u0cv_i_dn7_slot;
        let mut var_u0cv_i_dn8: f64 = *var_u0cv_i_dn8_slot;
        let mut var_u0cv_i_dn9: f64 = *var_u0cv_i_dn9_slot;
        let mut var_u0cv_i_rv: f64 = *var_u0cv_i_rv_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_dn0: f64 = *var_u0r_i_dn0_slot;
        let mut var_u0r_i_dn10: f64 = *var_u0r_i_dn10_slot;
        let mut var_u0r_i_dn11: f64 = *var_u0r_i_dn11_slot;
        let mut var_u0r_i_dn13: f64 = *var_u0r_i_dn13_slot;
        let mut var_u0r_i_dn14: f64 = *var_u0r_i_dn14_slot;
        let mut var_u0r_i_dn2: f64 = *var_u0r_i_dn2_slot;
        let mut var_u0r_i_dn3: f64 = *var_u0r_i_dn3_slot;
        let mut var_u0r_i_dn4: f64 = *var_u0r_i_dn4_slot;
        let mut var_u0r_i_dn5: f64 = *var_u0r_i_dn5_slot;
        let mut var_u0r_i_dn6: f64 = *var_u0r_i_dn6_slot;
        let mut var_u0r_i_dn7: f64 = *var_u0r_i_dn7_slot;
        let mut var_u0r_i_dn8: f64 = *var_u0r_i_dn8_slot;
        let mut var_u0r_i_dn9: f64 = *var_u0r_i_dn9_slot;
        let mut var_u0r_i_rv: f64 = *var_u0r_i_rv_slot;
        let mut var_ua1r_i: f64 = *var_ua1r_i_slot;
        let mut var_ua1r_i_rv: f64 = *var_ua1r_i_rv_slot;
        let mut var_uacv_i: f64 = *var_uacv_i_slot;
        let mut var_uacv_i_rv: f64 = *var_uacv_i_rv_slot;
        let mut var_uar_i: f64 = *var_uar_i_slot;
        let mut var_uar_i_dn0: f64 = *var_uar_i_dn0_slot;
        let mut var_uar_i_dn10: f64 = *var_uar_i_dn10_slot;
        let mut var_uar_i_dn11: f64 = *var_uar_i_dn11_slot;
        let mut var_uar_i_dn13: f64 = *var_uar_i_dn13_slot;
        let mut var_uar_i_dn14: f64 = *var_uar_i_dn14_slot;
        let mut var_uar_i_dn2: f64 = *var_uar_i_dn2_slot;
        let mut var_uar_i_dn3: f64 = *var_uar_i_dn3_slot;
        let mut var_uar_i_dn4: f64 = *var_uar_i_dn4_slot;
        let mut var_uar_i_dn5: f64 = *var_uar_i_dn5_slot;
        let mut var_uar_i_dn6: f64 = *var_uar_i_dn6_slot;
        let mut var_uar_i_dn7: f64 = *var_uar_i_dn7_slot;
        let mut var_uar_i_dn8: f64 = *var_uar_i_dn8_slot;
        let mut var_uar_i_dn9: f64 = *var_uar_i_dn9_slot;
        let mut var_uar_i_rv: f64 = *var_uar_i_rv_slot;
        let mut var_uc1r_i: f64 = *var_uc1r_i_slot;
        let mut var_uc1r_i_rv: f64 = *var_uc1r_i_rv_slot;
        let mut var_ucr_i: f64 = *var_ucr_i_slot;
        let mut var_ucr_i_rv: f64 = *var_ucr_i_rv_slot;
        let mut var_ud1r_i: f64 = *var_ud1r_i_slot;
        let mut var_ud1r_i_rv: f64 = *var_ud1r_i_rv_slot;
        let mut var_udcv_i: f64 = *var_udcv_i_slot;
        let mut var_udcv_i_rv: f64 = *var_udcv_i_rv_slot;
        let mut var_udr_i: f64 = *var_udr_i_slot;
        let mut var_udr_i_dn0: f64 = *var_udr_i_dn0_slot;
        let mut var_udr_i_dn10: f64 = *var_udr_i_dn10_slot;
        let mut var_udr_i_dn11: f64 = *var_udr_i_dn11_slot;
        let mut var_udr_i_dn13: f64 = *var_udr_i_dn13_slot;
        let mut var_udr_i_dn14: f64 = *var_udr_i_dn14_slot;
        let mut var_udr_i_dn2: f64 = *var_udr_i_dn2_slot;
        let mut var_udr_i_dn3: f64 = *var_udr_i_dn3_slot;
        let mut var_udr_i_dn4: f64 = *var_udr_i_dn4_slot;
        let mut var_udr_i_dn5: f64 = *var_udr_i_dn5_slot;
        let mut var_udr_i_dn6: f64 = *var_udr_i_dn6_slot;
        let mut var_udr_i_dn7: f64 = *var_udr_i_dn7_slot;
        let mut var_udr_i_dn8: f64 = *var_udr_i_dn8_slot;
        let mut var_udr_i_dn9: f64 = *var_udr_i_dn9_slot;
        let mut var_udr_i_rv: f64 = *var_udr_i_rv_slot;
        let mut var_upr_i: f64 = *var_upr_i_slot;
        let mut var_upr_i_rv: f64 = *var_upr_i_rv_slot;
        let mut var_ute1cv_i: f64 = *var_ute1cv_i_slot;
        let mut var_ute1cv_i_rv: f64 = *var_ute1cv_i_rv_slot;
        let mut var_utecv_i: f64 = *var_utecv_i_slot;
        let mut var_utecv_i_rv: f64 = *var_utecv_i_rv_slot;
        let mut var_uter_i: f64 = *var_uter_i_slot;
        let mut var_uter_i_rv: f64 = *var_uter_i_rv_slot;
        let mut var_utlr_i: f64 = *var_utlr_i_slot;
        let mut var_utlr_i_rv: f64 = *var_utlr_i_rv_slot;
        let mut var_vsatr_i: f64 = *var_vsatr_i_slot;
        let mut var_vsatr_i_rv: f64 = *var_vsatr_i_rv_slot;

        let (assign6110_e9040,) = {
    if (var_guard45 != 0.0) {
        let assign6110_e9021: f64 = (var_inv_l * p.p462);
        let assign6110_e9022: f64 = (p.p461 + assign6110_e9021);
        let assign6110_e9025: f64 = (var_inv_nfin * p.p463);
        let assign6110_e9026: f64 = (assign6110_e9022 + assign6110_e9025);
        let assign6110_e9029: f64 = (var_inv_lnfin * p.p464);
        let assign6110_e9030: f64 = (assign6110_e9026 + assign6110_e9029);
        let assign6110_e9033: f64 = (var_inv_w * p.p465);
        let assign6110_e9034: f64 = (assign6110_e9030 + assign6110_e9033);
        let assign6110_e9037: f64 = (var_inv_wl * p.p466);
        let assign6110_e9038: f64 = (assign6110_e9034 + assign6110_e9037);
        (assign6110_e9038,)
    } else {
        (var_vsatr_i,)
    }
};
        var_vsatr_i = assign6110_e9040;
        var_vsatr_i_rv = 0.0;

        let (assign6120_e9064,) = {
    if (var_guard45 != 0.0) {
        let assign6120_e9045: f64 = (var_inv_l * p.p501);
        let assign6120_e9046: f64 = (p.p500 + assign6120_e9045);
        let assign6120_e9049: f64 = (var_inv_nfin * p.p502);
        let assign6120_e9050: f64 = (assign6120_e9046 + assign6120_e9049);
        let assign6120_e9053: f64 = (var_inv_lnfin * p.p503);
        let assign6120_e9054: f64 = (assign6120_e9050 + assign6120_e9053);
        let assign6120_e9057: f64 = (var_inv_w * p.p504);
        let assign6120_e9058: f64 = (assign6120_e9054 + assign6120_e9057);
        let assign6120_e9061: f64 = (var_inv_wl * p.p505);
        let assign6120_e9062: f64 = (assign6120_e9058 + assign6120_e9061);
        (assign6120_e9062,)
    } else {
        (var_ksativr_i,)
    }
};
        var_ksativr_i = assign6120_e9064;
        var_ksativr_i_rv = 0.0;

        let (assign6130_e9088, assign6130_e9088_d_n0, assign6130_e9088_d_n2, assign6130_e9088_d_n3, assign6130_e9088_d_n4, assign6130_e9088_d_n5, assign6130_e9088_d_n6, assign6130_e9088_d_n7, assign6130_e9088_d_n8, assign6130_e9088_d_n9, assign6130_e9088_d_n10, assign6130_e9088_d_n11, assign6130_e9088_d_n13, assign6130_e9088_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6130_e9069: f64 = (var_inv_l * p.p612);
        let assign6130_e9070: f64 = (p.p611 + assign6130_e9069);
        let assign6130_e9073: f64 = (var_inv_nfin * p.p613);
        let assign6130_e9074: f64 = (assign6130_e9070 + assign6130_e9073);
        let assign6130_e9077: f64 = (var_inv_lnfin * p.p614);
        let assign6130_e9078: f64 = (assign6130_e9074 + assign6130_e9077);
        let assign6130_e9081: f64 = (var_inv_w * p.p615);
        let assign6130_e9082: f64 = (assign6130_e9078 + assign6130_e9081);
        let assign6130_e9085: f64 = (var_inv_wl * p.p616);
        let assign6130_e9086: f64 = (assign6130_e9082 + assign6130_e9085);
        (assign6130_e9086, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign6130_e9088;
        var_u0r_i_dn0 = assign6130_e9088_d_n0;
        var_u0r_i_dn2 = assign6130_e9088_d_n2;
        var_u0r_i_dn3 = assign6130_e9088_d_n3;
        var_u0r_i_dn4 = assign6130_e9088_d_n4;
        var_u0r_i_dn5 = assign6130_e9088_d_n5;
        var_u0r_i_dn6 = assign6130_e9088_d_n6;
        var_u0r_i_dn7 = assign6130_e9088_d_n7;
        var_u0r_i_dn8 = assign6130_e9088_d_n8;
        var_u0r_i_dn9 = assign6130_e9088_d_n9;
        var_u0r_i_dn10 = assign6130_e9088_d_n10;
        var_u0r_i_dn11 = assign6130_e9088_d_n11;
        var_u0r_i_dn13 = assign6130_e9088_d_n13;
        var_u0r_i_dn14 = assign6130_e9088_d_n14;
        var_u0r_i_rv = 0.0;

        let (assign6140_e9112, assign6140_e9112_d_n0, assign6140_e9112_d_n2, assign6140_e9112_d_n3, assign6140_e9112_d_n4, assign6140_e9112_d_n5, assign6140_e9112_d_n6, assign6140_e9112_d_n7, assign6140_e9112_d_n8, assign6140_e9112_d_n9, assign6140_e9112_d_n10, assign6140_e9112_d_n11, assign6140_e9112_d_n13, assign6140_e9112_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6140_e9093: f64 = (var_inv_l * p.p648);
        let assign6140_e9094: f64 = (p.p647 + assign6140_e9093);
        let assign6140_e9097: f64 = (var_inv_nfin * p.p649);
        let assign6140_e9098: f64 = (assign6140_e9094 + assign6140_e9097);
        let assign6140_e9101: f64 = (var_inv_lnfin * p.p650);
        let assign6140_e9102: f64 = (assign6140_e9098 + assign6140_e9101);
        let assign6140_e9105: f64 = (var_inv_w * p.p651);
        let assign6140_e9106: f64 = (assign6140_e9102 + assign6140_e9105);
        let assign6140_e9109: f64 = (var_inv_wl * p.p652);
        let assign6140_e9110: f64 = (assign6140_e9106 + assign6140_e9109);
        (assign6140_e9110, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uar_i, var_uar_i_dn0, var_uar_i_dn2, var_uar_i_dn3, var_uar_i_dn4, var_uar_i_dn5, var_uar_i_dn6, var_uar_i_dn7, var_uar_i_dn8, var_uar_i_dn9, var_uar_i_dn10, var_uar_i_dn11, var_uar_i_dn13, var_uar_i_dn14,)
    }
};
        var_uar_i = assign6140_e9112;
        var_uar_i_dn0 = assign6140_e9112_d_n0;
        var_uar_i_dn2 = assign6140_e9112_d_n2;
        var_uar_i_dn3 = assign6140_e9112_d_n3;
        var_uar_i_dn4 = assign6140_e9112_d_n4;
        var_uar_i_dn5 = assign6140_e9112_d_n5;
        var_uar_i_dn6 = assign6140_e9112_d_n6;
        var_uar_i_dn7 = assign6140_e9112_d_n7;
        var_uar_i_dn8 = assign6140_e9112_d_n8;
        var_uar_i_dn9 = assign6140_e9112_d_n9;
        var_uar_i_dn10 = assign6140_e9112_d_n10;
        var_uar_i_dn11 = assign6140_e9112_d_n11;
        var_uar_i_dn13 = assign6140_e9112_d_n13;
        var_uar_i_dn14 = assign6140_e9112_d_n14;
        var_uar_i_rv = 0.0;

        let (assign6150_e9136,) = {
    if (var_guard45 != 0.0) {
        let assign6150_e9117: f64 = (var_inv_l * p.p636);
        let assign6150_e9118: f64 = (p.p635 + assign6150_e9117);
        let assign6150_e9121: f64 = (var_inv_nfin * p.p637);
        let assign6150_e9122: f64 = (assign6150_e9118 + assign6150_e9121);
        let assign6150_e9125: f64 = (var_inv_lnfin * p.p638);
        let assign6150_e9126: f64 = (assign6150_e9122 + assign6150_e9125);
        let assign6150_e9129: f64 = (var_inv_w * p.p639);
        let assign6150_e9130: f64 = (assign6150_e9126 + assign6150_e9129);
        let assign6150_e9133: f64 = (var_inv_wl * p.p640);
        let assign6150_e9134: f64 = (assign6150_e9130 + assign6150_e9133);
        (assign6150_e9134,)
    } else {
        (var_upr_i,)
    }
};
        var_upr_i = assign6150_e9136;
        var_upr_i_rv = 0.0;

        let (assign6160_e9160, assign6160_e9160_d_n0, assign6160_e9160_d_n2, assign6160_e9160_d_n3, assign6160_e9160_d_n4, assign6160_e9160_d_n5, assign6160_e9160_d_n6, assign6160_e9160_d_n7, assign6160_e9160_d_n8, assign6160_e9160_d_n9, assign6160_e9160_d_n10, assign6160_e9160_d_n11, assign6160_e9160_d_n13, assign6160_e9160_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6160_e9141: f64 = (var_inv_l * p.p684);
        let assign6160_e9142: f64 = (p.p683 + assign6160_e9141);
        let assign6160_e9145: f64 = (var_inv_nfin * p.p685);
        let assign6160_e9146: f64 = (assign6160_e9142 + assign6160_e9145);
        let assign6160_e9149: f64 = (var_inv_lnfin * p.p686);
        let assign6160_e9150: f64 = (assign6160_e9146 + assign6160_e9149);
        let assign6160_e9153: f64 = (var_inv_w * p.p687);
        let assign6160_e9154: f64 = (assign6160_e9150 + assign6160_e9153);
        let assign6160_e9157: f64 = (var_inv_wl * p.p688);
        let assign6160_e9158: f64 = (assign6160_e9154 + assign6160_e9157);
        (assign6160_e9158, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eur_i, var_eur_i_dn0, var_eur_i_dn2, var_eur_i_dn3, var_eur_i_dn4, var_eur_i_dn5, var_eur_i_dn6, var_eur_i_dn7, var_eur_i_dn8, var_eur_i_dn9, var_eur_i_dn10, var_eur_i_dn11, var_eur_i_dn13, var_eur_i_dn14,)
    }
};
        var_eur_i = assign6160_e9160;
        var_eur_i_dn0 = assign6160_e9160_d_n0;
        var_eur_i_dn2 = assign6160_e9160_d_n2;
        var_eur_i_dn3 = assign6160_e9160_d_n3;
        var_eur_i_dn4 = assign6160_e9160_d_n4;
        var_eur_i_dn5 = assign6160_e9160_d_n5;
        var_eur_i_dn6 = assign6160_e9160_d_n6;
        var_eur_i_dn7 = assign6160_e9160_d_n7;
        var_eur_i_dn8 = assign6160_e9160_d_n8;
        var_eur_i_dn9 = assign6160_e9160_d_n9;
        var_eur_i_dn10 = assign6160_e9160_d_n10;
        var_eur_i_dn11 = assign6160_e9160_d_n11;
        var_eur_i_dn13 = assign6160_e9160_d_n13;
        var_eur_i_dn14 = assign6160_e9160_d_n14;
        var_eur_i_rv = 0.0;

        let (assign6170_e9184, assign6170_e9184_d_n0, assign6170_e9184_d_n2, assign6170_e9184_d_n3, assign6170_e9184_d_n4, assign6170_e9184_d_n5, assign6170_e9184_d_n6, assign6170_e9184_d_n7, assign6170_e9184_d_n8, assign6170_e9184_d_n9, assign6170_e9184_d_n10, assign6170_e9184_d_n11, assign6170_e9184_d_n13, assign6170_e9184_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6170_e9165: f64 = (var_inv_l * p.p696);
        let assign6170_e9166: f64 = (p.p695 + assign6170_e9165);
        let assign6170_e9169: f64 = (var_inv_nfin * p.p697);
        let assign6170_e9170: f64 = (assign6170_e9166 + assign6170_e9169);
        let assign6170_e9173: f64 = (var_inv_lnfin * p.p698);
        let assign6170_e9174: f64 = (assign6170_e9170 + assign6170_e9173);
        let assign6170_e9177: f64 = (var_inv_w * p.p699);
        let assign6170_e9178: f64 = (assign6170_e9174 + assign6170_e9177);
        let assign6170_e9181: f64 = (var_inv_wl * p.p700);
        let assign6170_e9182: f64 = (assign6170_e9178 + assign6170_e9181);
        (assign6170_e9182, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_udr_i, var_udr_i_dn0, var_udr_i_dn2, var_udr_i_dn3, var_udr_i_dn4, var_udr_i_dn5, var_udr_i_dn6, var_udr_i_dn7, var_udr_i_dn8, var_udr_i_dn9, var_udr_i_dn10, var_udr_i_dn11, var_udr_i_dn13, var_udr_i_dn14,)
    }
};
        var_udr_i = assign6170_e9184;
        var_udr_i_dn0 = assign6170_e9184_d_n0;
        var_udr_i_dn2 = assign6170_e9184_d_n2;
        var_udr_i_dn3 = assign6170_e9184_d_n3;
        var_udr_i_dn4 = assign6170_e9184_d_n4;
        var_udr_i_dn5 = assign6170_e9184_d_n5;
        var_udr_i_dn6 = assign6170_e9184_d_n6;
        var_udr_i_dn7 = assign6170_e9184_d_n7;
        var_udr_i_dn8 = assign6170_e9184_d_n8;
        var_udr_i_dn9 = assign6170_e9184_d_n9;
        var_udr_i_dn10 = assign6170_e9184_d_n10;
        var_udr_i_dn11 = assign6170_e9184_d_n11;
        var_udr_i_dn13 = assign6170_e9184_d_n13;
        var_udr_i_dn14 = assign6170_e9184_d_n14;
        var_udr_i_rv = 0.0;

        let (assign6180_e9208,) = {
    if (var_guard45 != 0.0) {
        let assign6180_e9189: f64 = (var_inv_l * p.p744);
        let assign6180_e9190: f64 = (p.p743 + assign6180_e9189);
        let assign6180_e9193: f64 = (var_inv_nfin * p.p745);
        let assign6180_e9194: f64 = (assign6180_e9190 + assign6180_e9193);
        let assign6180_e9197: f64 = (var_inv_lnfin * p.p746);
        let assign6180_e9198: f64 = (assign6180_e9194 + assign6180_e9197);
        let assign6180_e9201: f64 = (var_inv_w * p.p747);
        let assign6180_e9202: f64 = (assign6180_e9198 + assign6180_e9201);
        let assign6180_e9205: f64 = (var_inv_wl * p.p748);
        let assign6180_e9206: f64 = (assign6180_e9202 + assign6180_e9205);
        (assign6180_e9206,)
    } else {
        (var_uter_i,)
    }
};
        var_uter_i = assign6180_e9208;
        var_uter_i_rv = 0.0;

        let (assign6190_e9232,) = {
    if (var_guard45 != 0.0) {
        let assign6190_e9213: f64 = (var_inv_l * p.p774);
        let assign6190_e9214: f64 = (p.p773 + assign6190_e9213);
        let assign6190_e9217: f64 = (var_inv_nfin * p.p775);
        let assign6190_e9218: f64 = (assign6190_e9214 + assign6190_e9217);
        let assign6190_e9221: f64 = (var_inv_lnfin * p.p776);
        let assign6190_e9222: f64 = (assign6190_e9218 + assign6190_e9221);
        let assign6190_e9225: f64 = (var_inv_w * p.p777);
        let assign6190_e9226: f64 = (assign6190_e9222 + assign6190_e9225);
        let assign6190_e9229: f64 = (var_inv_wl * p.p778);
        let assign6190_e9230: f64 = (assign6190_e9226 + assign6190_e9229);
        (assign6190_e9230,)
    } else {
        (var_utlr_i,)
    }
};
        var_utlr_i = assign6190_e9232;
        var_utlr_i_rv = 0.0;

        let (assign6200_e9256,) = {
    if (var_guard45 != 0.0) {
        let assign6200_e9237: f64 = (var_inv_l * p.p798);
        let assign6200_e9238: f64 = (p.p797 + assign6200_e9237);
        let assign6200_e9241: f64 = (var_inv_nfin * p.p799);
        let assign6200_e9242: f64 = (assign6200_e9238 + assign6200_e9241);
        let assign6200_e9245: f64 = (var_inv_lnfin * p.p800);
        let assign6200_e9246: f64 = (assign6200_e9242 + assign6200_e9245);
        let assign6200_e9249: f64 = (var_inv_w * p.p801);
        let assign6200_e9250: f64 = (assign6200_e9246 + assign6200_e9249);
        let assign6200_e9253: f64 = (var_inv_wl * p.p802);
        let assign6200_e9254: f64 = (assign6200_e9250 + assign6200_e9253);
        (assign6200_e9254,)
    } else {
        (var_ua1r_i,)
    }
};
        var_ua1r_i = assign6200_e9256;
        var_ua1r_i_rv = 0.0;

        let (assign6210_e9280,) = {
    if (var_guard45 != 0.0) {
        let assign6210_e9261: f64 = (var_inv_l * p.p852);
        let assign6210_e9262: f64 = (p.p851 + assign6210_e9261);
        let assign6210_e9265: f64 = (var_inv_nfin * p.p853);
        let assign6210_e9266: f64 = (assign6210_e9262 + assign6210_e9265);
        let assign6210_e9269: f64 = (var_inv_lnfin * p.p854);
        let assign6210_e9270: f64 = (assign6210_e9266 + assign6210_e9269);
        let assign6210_e9273: f64 = (var_inv_w * p.p855);
        let assign6210_e9274: f64 = (assign6210_e9270 + assign6210_e9273);
        let assign6210_e9277: f64 = (var_inv_wl * p.p856);
        let assign6210_e9278: f64 = (assign6210_e9274 + assign6210_e9277);
        (assign6210_e9278,)
    } else {
        (var_ud1r_i,)
    }
};
        var_ud1r_i = assign6210_e9280;
        var_ud1r_i_rv = 0.0;

        let (assign6220_e9304,) = {
    if (var_guard45 != 0.0) {
        let assign6220_e9285: f64 = (var_inv_l * p.p563);
        let assign6220_e9286: f64 = (p.p562 + assign6220_e9285);
        let assign6220_e9289: f64 = (var_inv_nfin * p.p564);
        let assign6220_e9290: f64 = (assign6220_e9286 + assign6220_e9289);
        let assign6220_e9293: f64 = (var_inv_lnfin * p.p565);
        let assign6220_e9294: f64 = (assign6220_e9290 + assign6220_e9293);
        let assign6220_e9297: f64 = (var_inv_w * p.p566);
        let assign6220_e9298: f64 = (assign6220_e9294 + assign6220_e9297);
        let assign6220_e9301: f64 = (var_inv_wl * p.p567);
        let assign6220_e9302: f64 = (assign6220_e9298 + assign6220_e9301);
        (assign6220_e9302,)
    } else {
        (var_atr_i,)
    }
};
        var_atr_i = assign6220_e9304;
        var_atr_i_rv = 0.0;

        let assign6230_e9307: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard46 = assign6230_e9307;
        var_guard46_rv = 0.0;

        let (assign6240_e9333,) = {
    if ((var_guard45 != 0.0) && (var_guard46 != 0.0)) {
        let assign6240_e9314: f64 = (var_inv_l * p.p666);
        let assign6240_e9315: f64 = (p.p665 + assign6240_e9314);
        let assign6240_e9318: f64 = (var_inv_nfin * p.p667);
        let assign6240_e9319: f64 = (assign6240_e9315 + assign6240_e9318);
        let assign6240_e9322: f64 = (var_inv_lnfin * p.p668);
        let assign6240_e9323: f64 = (assign6240_e9319 + assign6240_e9322);
        let assign6240_e9326: f64 = (var_inv_w * p.p669);
        let assign6240_e9327: f64 = (assign6240_e9323 + assign6240_e9326);
        let assign6240_e9330: f64 = (var_inv_wl * p.p670);
        let assign6240_e9331: f64 = (assign6240_e9327 + assign6240_e9330);
        (assign6240_e9331,)
    } else {
        (var_ucr_i,)
    }
};
        var_ucr_i = assign6240_e9333;
        var_ucr_i_rv = 0.0;

        let (assign6250_e9359,) = {
    if ((var_guard45 != 0.0) && (var_guard46 != 0.0)) {
        let assign6250_e9340: f64 = (var_inv_l * p.p834);
        let assign6250_e9341: f64 = (p.p833 + assign6250_e9340);
        let assign6250_e9344: f64 = (var_inv_nfin * p.p835);
        let assign6250_e9345: f64 = (assign6250_e9341 + assign6250_e9344);
        let assign6250_e9348: f64 = (var_inv_lnfin * p.p836);
        let assign6250_e9349: f64 = (assign6250_e9345 + assign6250_e9348);
        let assign6250_e9352: f64 = (var_inv_w * p.p837);
        let assign6250_e9353: f64 = (assign6250_e9349 + assign6250_e9352);
        let assign6250_e9356: f64 = (var_inv_wl * p.p838);
        let assign6250_e9357: f64 = (assign6250_e9353 + assign6250_e9356);
        (assign6250_e9357,)
    } else {
        (var_uc1r_i,)
    }
};
        var_uc1r_i = assign6250_e9359;
        var_uc1r_i_rv = 0.0;

        let assign6260_e9362: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign6260_e9362;
        var_guard47_rv = 0.0;

        let (assign6270_e9386, assign6270_e9386_d_n0, assign6270_e9386_d_n2, assign6270_e9386_d_n3, assign6270_e9386_d_n4, assign6270_e9386_d_n5, assign6270_e9386_d_n6, assign6270_e9386_d_n7, assign6270_e9386_d_n8, assign6270_e9386_d_n9, assign6270_e9386_d_n10, assign6270_e9386_d_n11, assign6270_e9386_d_n13, assign6270_e9386_d_n14,) = {
    if (var_guard47 != 0.0) {
        let assign6270_e9367: f64 = (var_inv_l * p.p618);
        let assign6270_e9368: f64 = (p.p617 + assign6270_e9367);
        let assign6270_e9371: f64 = (var_inv_nfin * p.p619);
        let assign6270_e9372: f64 = (assign6270_e9368 + assign6270_e9371);
        let assign6270_e9375: f64 = (var_inv_lnfin * p.p620);
        let assign6270_e9376: f64 = (assign6270_e9372 + assign6270_e9375);
        let assign6270_e9379: f64 = (var_inv_w * p.p621);
        let assign6270_e9380: f64 = (assign6270_e9376 + assign6270_e9379);
        let assign6270_e9383: f64 = (var_inv_wl * p.p622);
        let assign6270_e9384: f64 = (assign6270_e9380 + assign6270_e9383);
        (assign6270_e9384, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_u0cv_i, var_u0cv_i_dn0, var_u0cv_i_dn2, var_u0cv_i_dn3, var_u0cv_i_dn4, var_u0cv_i_dn5, var_u0cv_i_dn6, var_u0cv_i_dn7, var_u0cv_i_dn8, var_u0cv_i_dn9, var_u0cv_i_dn10, var_u0cv_i_dn11, var_u0cv_i_dn13, var_u0cv_i_dn14,)
    }
};
        var_u0cv_i = assign6270_e9386;
        var_u0cv_i_dn0 = assign6270_e9386_d_n0;
        var_u0cv_i_dn2 = assign6270_e9386_d_n2;
        var_u0cv_i_dn3 = assign6270_e9386_d_n3;
        var_u0cv_i_dn4 = assign6270_e9386_d_n4;
        var_u0cv_i_dn5 = assign6270_e9386_d_n5;
        var_u0cv_i_dn6 = assign6270_e9386_d_n6;
        var_u0cv_i_dn7 = assign6270_e9386_d_n7;
        var_u0cv_i_dn8 = assign6270_e9386_d_n8;
        var_u0cv_i_dn9 = assign6270_e9386_d_n9;
        var_u0cv_i_dn10 = assign6270_e9386_d_n10;
        var_u0cv_i_dn11 = assign6270_e9386_d_n11;
        var_u0cv_i_dn13 = assign6270_e9386_d_n13;
        var_u0cv_i_dn14 = assign6270_e9386_d_n14;
        var_u0cv_i_rv = 0.0;

        let assign6280_e9389: f64 = if p.p582 != 0.0 { 1.0 } else { 0.0 };
        var_guard48 = assign6280_e9389;
        var_guard48_rv = 0.0;

        let (assign6290_e9428, assign6290_e9428_d_n0, assign6290_e9428_d_n2, assign6290_e9428_d_n3, assign6290_e9428_d_n4, assign6290_e9428_d_n5, assign6290_e9428_d_n6, assign6290_e9428_d_n7, assign6290_e9428_d_n8, assign6290_e9428_d_n9, assign6290_e9428_d_n10, assign6290_e9428_d_n11, assign6290_e9428_d_n13, assign6290_e9428_d_n14,) = {
    if ((var_guard47 != 0.0) && (var_guard48 != 0.0)) {
        let assign6290_e9397: f64 = (p.p582 / p.p5);
        let assign6290_e9401: f64 = (p.p5 / p.p585);
        let assign6290_e9402: f64 = (1.0 + assign6290_e9401);
        let (assign6290_e9423,) = {
            if (!(assign6290_e9402 > 1e-38)) {
                let assign6290_e9407: f64 = (-87.498233534);
                (assign6290_e9407,)
            } else {
                let assign6290_e9411: f64 = (p.p5 / p.p585);
                let assign6290_e9412: f64 = (1.0 + assign6290_e9411);
                let (assign6290_e9422,) = {
                    if (assign6290_e9412 > 1e-38) {
                        let assign6290_e9418: f64 = (p.p5 / p.p585);
                        let assign6290_e9419: f64 = (1.0 + assign6290_e9418);
                        let assign6290_e9420: f64 = (assign6290_e9419).ln();
                        (assign6290_e9420,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6290_e9422,)
            }
        };
        let assign6290_e9424: f64 = (assign6290_e9397 * assign6290_e9423);
        let assign6290_e9425: f64 = (1.0 + assign6290_e9424);
        let assign6290_e9426: f64 = (var_u0cv_i * assign6290_e9425);
        (assign6290_e9426, (var_u0cv_i_dn0 * assign6290_e9425), (var_u0cv_i_dn2 * assign6290_e9425), (var_u0cv_i_dn3 * assign6290_e9425), (var_u0cv_i_dn4 * assign6290_e9425), (var_u0cv_i_dn5 * assign6290_e9425), (var_u0cv_i_dn6 * assign6290_e9425), (var_u0cv_i_dn7 * assign6290_e9425), (var_u0cv_i_dn8 * assign6290_e9425), (var_u0cv_i_dn9 * assign6290_e9425), (var_u0cv_i_dn10 * assign6290_e9425), (var_u0cv_i_dn11 * assign6290_e9425), (var_u0cv_i_dn13 * assign6290_e9425), (var_u0cv_i_dn14 * assign6290_e9425),)
    } else {
        (var_u0cv_i, var_u0cv_i_dn0, var_u0cv_i_dn2, var_u0cv_i_dn3, var_u0cv_i_dn4, var_u0cv_i_dn5, var_u0cv_i_dn6, var_u0cv_i_dn7, var_u0cv_i_dn8, var_u0cv_i_dn9, var_u0cv_i_dn10, var_u0cv_i_dn11, var_u0cv_i_dn13, var_u0cv_i_dn14,)
    }
};
        var_u0cv_i = assign6290_e9428;
        var_u0cv_i_dn0 = assign6290_e9428_d_n0;
        var_u0cv_i_dn2 = assign6290_e9428_d_n2;
        var_u0cv_i_dn3 = assign6290_e9428_d_n3;
        var_u0cv_i_dn4 = assign6290_e9428_d_n4;
        var_u0cv_i_dn5 = assign6290_e9428_d_n5;
        var_u0cv_i_dn6 = assign6290_e9428_d_n6;
        var_u0cv_i_dn7 = assign6290_e9428_d_n7;
        var_u0cv_i_dn8 = assign6290_e9428_d_n8;
        var_u0cv_i_dn9 = assign6290_e9428_d_n9;
        var_u0cv_i_dn10 = assign6290_e9428_d_n10;
        var_u0cv_i_dn11 = assign6290_e9428_d_n11;
        var_u0cv_i_dn13 = assign6290_e9428_d_n13;
        var_u0cv_i_dn14 = assign6290_e9428_d_n14;
        var_u0cv_i_rv = 0.0;

        let (assign6300_e9452,) = {
    if (var_guard47 != 0.0) {
        let assign6300_e9433: f64 = (var_inv_l * p.p654);
        let assign6300_e9434: f64 = (p.p653 + assign6300_e9433);
        let assign6300_e9437: f64 = (var_inv_nfin * p.p655);
        let assign6300_e9438: f64 = (assign6300_e9434 + assign6300_e9437);
        let assign6300_e9441: f64 = (var_inv_lnfin * p.p656);
        let assign6300_e9442: f64 = (assign6300_e9438 + assign6300_e9441);
        let assign6300_e9445: f64 = (var_inv_w * p.p657);
        let assign6300_e9446: f64 = (assign6300_e9442 + assign6300_e9445);
        let assign6300_e9449: f64 = (var_inv_wl * p.p658);
        let assign6300_e9450: f64 = (assign6300_e9446 + assign6300_e9449);
        (assign6300_e9450,)
    } else {
        (var_uacv_i,)
    }
};
        var_uacv_i = assign6300_e9452;
        var_uacv_i_rv = 0.0;

        let (assign6310_e9476,) = {
    if (var_guard47 != 0.0) {
        let assign6310_e9457: f64 = (var_inv_l * p.p702);
        let assign6310_e9458: f64 = (p.p701 + assign6310_e9457);
        let assign6310_e9461: f64 = (var_inv_nfin * p.p703);
        let assign6310_e9462: f64 = (assign6310_e9458 + assign6310_e9461);
        let assign6310_e9465: f64 = (var_inv_lnfin * p.p704);
        let assign6310_e9466: f64 = (assign6310_e9462 + assign6310_e9465);
        let assign6310_e9469: f64 = (var_inv_w * p.p705);
        let assign6310_e9470: f64 = (assign6310_e9466 + assign6310_e9469);
        let assign6310_e9473: f64 = (var_inv_wl * p.p706);
        let assign6310_e9474: f64 = (assign6310_e9470 + assign6310_e9473);
        (assign6310_e9474,)
    } else {
        (var_udcv_i,)
    }
};
        var_udcv_i = assign6310_e9476;
        var_udcv_i_rv = 0.0;

        let (assign6320_e9500,) = {
    if (var_guard47 != 0.0) {
        let assign6320_e9481: f64 = (var_inv_l * p.p750);
        let assign6320_e9482: f64 = (p.p749 + assign6320_e9481);
        let assign6320_e9485: f64 = (var_inv_nfin * p.p751);
        let assign6320_e9486: f64 = (assign6320_e9482 + assign6320_e9485);
        let assign6320_e9489: f64 = (var_inv_lnfin * p.p752);
        let assign6320_e9490: f64 = (assign6320_e9486 + assign6320_e9489);
        let assign6320_e9493: f64 = (var_inv_w * p.p753);
        let assign6320_e9494: f64 = (assign6320_e9490 + assign6320_e9493);
        let assign6320_e9497: f64 = (var_inv_wl * p.p754);
        let assign6320_e9498: f64 = (assign6320_e9494 + assign6320_e9497);
        (assign6320_e9498,)
    } else {
        (var_utecv_i,)
    }
};
        var_utecv_i = assign6320_e9500;
        var_utecv_i_rv = 0.0;

        let (assign6330_e9524,) = {
    if (var_guard47 != 0.0) {
        let assign6330_e9505: f64 = (var_inv_l * p.p762);
        let assign6330_e9506: f64 = (p.p761 + assign6330_e9505);
        let assign6330_e9509: f64 = (var_inv_nfin * p.p763);
        let assign6330_e9510: f64 = (assign6330_e9506 + assign6330_e9509);
        let assign6330_e9513: f64 = (var_inv_lnfin * p.p764);
        let assign6330_e9514: f64 = (assign6330_e9510 + assign6330_e9513);
        let assign6330_e9517: f64 = (var_inv_w * p.p765);
        let assign6330_e9518: f64 = (assign6330_e9514 + assign6330_e9517);
        let assign6330_e9521: f64 = (var_inv_wl * p.p766);
        let assign6330_e9522: f64 = (assign6330_e9518 + assign6330_e9521);
        (assign6330_e9522,)
    } else {
        (var_ute1cv_i,)
    }
};
        var_ute1cv_i = assign6330_e9524;
        var_ute1cv_i_rv = 0.0;

        *var_atr_i_slot = var_atr_i;
        *var_atr_i_rv_slot = var_atr_i_rv;
        *var_eur_i_slot = var_eur_i;
        *var_eur_i_dn0_slot = var_eur_i_dn0;
        *var_eur_i_dn10_slot = var_eur_i_dn10;
        *var_eur_i_dn11_slot = var_eur_i_dn11;
        *var_eur_i_dn13_slot = var_eur_i_dn13;
        *var_eur_i_dn14_slot = var_eur_i_dn14;
        *var_eur_i_dn2_slot = var_eur_i_dn2;
        *var_eur_i_dn3_slot = var_eur_i_dn3;
        *var_eur_i_dn4_slot = var_eur_i_dn4;
        *var_eur_i_dn5_slot = var_eur_i_dn5;
        *var_eur_i_dn6_slot = var_eur_i_dn6;
        *var_eur_i_dn7_slot = var_eur_i_dn7;
        *var_eur_i_dn8_slot = var_eur_i_dn8;
        *var_eur_i_dn9_slot = var_eur_i_dn9;
        *var_eur_i_rv_slot = var_eur_i_rv;
        *var_guard46_slot = var_guard46;
        *var_guard46_rv_slot = var_guard46_rv;
        *var_guard47_slot = var_guard47;
        *var_guard47_rv_slot = var_guard47_rv;
        *var_guard48_slot = var_guard48;
        *var_guard48_rv_slot = var_guard48_rv;
        *var_ksativr_i_slot = var_ksativr_i;
        *var_ksativr_i_rv_slot = var_ksativr_i_rv;
        *var_u0cv_i_slot = var_u0cv_i;
        *var_u0cv_i_dn0_slot = var_u0cv_i_dn0;
        *var_u0cv_i_dn10_slot = var_u0cv_i_dn10;
        *var_u0cv_i_dn11_slot = var_u0cv_i_dn11;
        *var_u0cv_i_dn13_slot = var_u0cv_i_dn13;
        *var_u0cv_i_dn14_slot = var_u0cv_i_dn14;
        *var_u0cv_i_dn2_slot = var_u0cv_i_dn2;
        *var_u0cv_i_dn3_slot = var_u0cv_i_dn3;
        *var_u0cv_i_dn4_slot = var_u0cv_i_dn4;
        *var_u0cv_i_dn5_slot = var_u0cv_i_dn5;
        *var_u0cv_i_dn6_slot = var_u0cv_i_dn6;
        *var_u0cv_i_dn7_slot = var_u0cv_i_dn7;
        *var_u0cv_i_dn8_slot = var_u0cv_i_dn8;
        *var_u0cv_i_dn9_slot = var_u0cv_i_dn9;
        *var_u0cv_i_rv_slot = var_u0cv_i_rv;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_dn0_slot = var_u0r_i_dn0;
        *var_u0r_i_dn10_slot = var_u0r_i_dn10;
        *var_u0r_i_dn11_slot = var_u0r_i_dn11;
        *var_u0r_i_dn13_slot = var_u0r_i_dn13;
        *var_u0r_i_dn14_slot = var_u0r_i_dn14;
        *var_u0r_i_dn2_slot = var_u0r_i_dn2;
        *var_u0r_i_dn3_slot = var_u0r_i_dn3;
        *var_u0r_i_dn4_slot = var_u0r_i_dn4;
        *var_u0r_i_dn5_slot = var_u0r_i_dn5;
        *var_u0r_i_dn6_slot = var_u0r_i_dn6;
        *var_u0r_i_dn7_slot = var_u0r_i_dn7;
        *var_u0r_i_dn8_slot = var_u0r_i_dn8;
        *var_u0r_i_dn9_slot = var_u0r_i_dn9;
        *var_u0r_i_rv_slot = var_u0r_i_rv;
        *var_ua1r_i_slot = var_ua1r_i;
        *var_ua1r_i_rv_slot = var_ua1r_i_rv;
        *var_uacv_i_slot = var_uacv_i;
        *var_uacv_i_rv_slot = var_uacv_i_rv;
        *var_uar_i_slot = var_uar_i;
        *var_uar_i_dn0_slot = var_uar_i_dn0;
        *var_uar_i_dn10_slot = var_uar_i_dn10;
        *var_uar_i_dn11_slot = var_uar_i_dn11;
        *var_uar_i_dn13_slot = var_uar_i_dn13;
        *var_uar_i_dn14_slot = var_uar_i_dn14;
        *var_uar_i_dn2_slot = var_uar_i_dn2;
        *var_uar_i_dn3_slot = var_uar_i_dn3;
        *var_uar_i_dn4_slot = var_uar_i_dn4;
        *var_uar_i_dn5_slot = var_uar_i_dn5;
        *var_uar_i_dn6_slot = var_uar_i_dn6;
        *var_uar_i_dn7_slot = var_uar_i_dn7;
        *var_uar_i_dn8_slot = var_uar_i_dn8;
        *var_uar_i_dn9_slot = var_uar_i_dn9;
        *var_uar_i_rv_slot = var_uar_i_rv;
        *var_uc1r_i_slot = var_uc1r_i;
        *var_uc1r_i_rv_slot = var_uc1r_i_rv;
        *var_ucr_i_slot = var_ucr_i;
        *var_ucr_i_rv_slot = var_ucr_i_rv;
        *var_ud1r_i_slot = var_ud1r_i;
        *var_ud1r_i_rv_slot = var_ud1r_i_rv;
        *var_udcv_i_slot = var_udcv_i;
        *var_udcv_i_rv_slot = var_udcv_i_rv;
        *var_udr_i_slot = var_udr_i;
        *var_udr_i_dn0_slot = var_udr_i_dn0;
        *var_udr_i_dn10_slot = var_udr_i_dn10;
        *var_udr_i_dn11_slot = var_udr_i_dn11;
        *var_udr_i_dn13_slot = var_udr_i_dn13;
        *var_udr_i_dn14_slot = var_udr_i_dn14;
        *var_udr_i_dn2_slot = var_udr_i_dn2;
        *var_udr_i_dn3_slot = var_udr_i_dn3;
        *var_udr_i_dn4_slot = var_udr_i_dn4;
        *var_udr_i_dn5_slot = var_udr_i_dn5;
        *var_udr_i_dn6_slot = var_udr_i_dn6;
        *var_udr_i_dn7_slot = var_udr_i_dn7;
        *var_udr_i_dn8_slot = var_udr_i_dn8;
        *var_udr_i_dn9_slot = var_udr_i_dn9;
        *var_udr_i_rv_slot = var_udr_i_rv;
        *var_upr_i_slot = var_upr_i;
        *var_upr_i_rv_slot = var_upr_i_rv;
        *var_ute1cv_i_slot = var_ute1cv_i;
        *var_ute1cv_i_rv_slot = var_ute1cv_i_rv;
        *var_utecv_i_slot = var_utecv_i;
        *var_utecv_i_rv_slot = var_utecv_i_rv;
        *var_uter_i_slot = var_uter_i;
        *var_uter_i_rv_slot = var_uter_i_rv;
        *var_utlr_i_slot = var_utlr_i;
        *var_utlr_i_rv_slot = var_utlr_i_rv;
        *var_vsatr_i_slot = var_vsatr_i;
        *var_vsatr_i_rv_slot = var_vsatr_i_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_guard47: f64,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_leff_1: f64,
        var_leff_1_dn0: f64,
        var_leff_1_dn10: f64,
        var_leff_1_dn11: f64,
        var_leff_1_dn13: f64,
        var_leff_1_dn14: f64,
        var_leff_1_dn2: f64,
        var_leff_1_dn3: f64,
        var_leff_1_dn4: f64,
        var_leff_1_dn5: f64,
        var_leff_1_dn6: f64,
        var_leff_1_dn7: f64,
        var_leff_1_dn8: f64,
        var_leff_1_dn9: f64,
        var_dimension1_i_slot: &mut f64,
        var_dimension1_i_rv_slot: &mut f64,
        var_dimension2_i_slot: &mut f64,
        var_dimension2_i_rv_slot: &mut f64,
        var_dimension3_i_slot: &mut f64,
        var_dimension3_i_rv_slot: &mut f64,
        var_e2nom_i_slot: &mut f64,
        var_e2nom_i_rv_slot: &mut f64,
        var_e3nom_i_slot: &mut f64,
        var_e3nom_i_rv_slot: &mut f64,
        var_eta0cv_i_slot: &mut f64,
        var_eta0cv_i_dn0_slot: &mut f64,
        var_eta0cv_i_dn10_slot: &mut f64,
        var_eta0cv_i_dn11_slot: &mut f64,
        var_eta0cv_i_dn13_slot: &mut f64,
        var_eta0cv_i_dn14_slot: &mut f64,
        var_eta0cv_i_dn2_slot: &mut f64,
        var_eta0cv_i_dn3_slot: &mut f64,
        var_eta0cv_i_dn4_slot: &mut f64,
        var_eta0cv_i_dn5_slot: &mut f64,
        var_eta0cv_i_dn6_slot: &mut f64,
        var_eta0cv_i_dn7_slot: &mut f64,
        var_eta0cv_i_dn8_slot: &mut f64,
        var_eta0cv_i_dn9_slot: &mut f64,
        var_eta0cv_i_rv_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard49_rv_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard50_rv_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard51_rv_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard53_rv_slot: &mut f64,
        var_mfq1nom_i_slot: &mut f64,
        var_mfq1nom_i_rv_slot: &mut f64,
        var_mfq2nom_i_slot: &mut f64,
        var_mfq2nom_i_rv_slot: &mut f64,
        var_ssp1_i_slot: &mut f64,
        var_ssp1_i_rv_slot: &mut f64,
        var_ssp2_i_slot: &mut f64,
        var_ssp2_i_rv_slot: &mut f64,
        var_ssp3_i_slot: &mut f64,
        var_ssp3_i_rv_slot: &mut f64,
        var_u0cv_i_slot: &mut f64,
        var_u0cv_i_dn0_slot: &mut f64,
        var_u0cv_i_dn10_slot: &mut f64,
        var_u0cv_i_dn11_slot: &mut f64,
        var_u0cv_i_dn13_slot: &mut f64,
        var_u0cv_i_dn14_slot: &mut f64,
        var_u0cv_i_dn2_slot: &mut f64,
        var_u0cv_i_dn3_slot: &mut f64,
        var_u0cv_i_dn4_slot: &mut f64,
        var_u0cv_i_dn5_slot: &mut f64,
        var_u0cv_i_dn6_slot: &mut f64,
        var_u0cv_i_dn7_slot: &mut f64,
        var_u0cv_i_dn8_slot: &mut f64,
        var_u0cv_i_dn9_slot: &mut f64,
        var_u0cv_i_rv_slot: &mut f64,
        var_ua1cv_i_slot: &mut f64,
        var_ua1cv_i_rv_slot: &mut f64,
        var_ua2cv_i_slot: &mut f64,
        var_ua2cv_i_rv_slot: &mut f64,
        var_uc1cv_i_slot: &mut f64,
        var_uc1cv_i_rv_slot: &mut f64,
        var_uccv_i_slot: &mut f64,
        var_uccv_i_rv_slot: &mut f64,
        var_ud1cv_i_slot: &mut f64,
        var_ud1cv_i_rv_slot: &mut f64,
        var_ud2cv_i_slot: &mut f64,
        var_ud2cv_i_rv_slot: &mut f64,
        var_utlcv_i_slot: &mut f64,
        var_utlcv_i_rv_slot: &mut f64,
    ) {
        let mut var_dimension1_i: f64 = *var_dimension1_i_slot;
        let mut var_dimension1_i_rv: f64 = *var_dimension1_i_rv_slot;
        let mut var_dimension2_i: f64 = *var_dimension2_i_slot;
        let mut var_dimension2_i_rv: f64 = *var_dimension2_i_rv_slot;
        let mut var_dimension3_i: f64 = *var_dimension3_i_slot;
        let mut var_dimension3_i_rv: f64 = *var_dimension3_i_rv_slot;
        let mut var_e2nom_i: f64 = *var_e2nom_i_slot;
        let mut var_e2nom_i_rv: f64 = *var_e2nom_i_rv_slot;
        let mut var_e3nom_i: f64 = *var_e3nom_i_slot;
        let mut var_e3nom_i_rv: f64 = *var_e3nom_i_rv_slot;
        let mut var_eta0cv_i: f64 = *var_eta0cv_i_slot;
        let mut var_eta0cv_i_dn0: f64 = *var_eta0cv_i_dn0_slot;
        let mut var_eta0cv_i_dn10: f64 = *var_eta0cv_i_dn10_slot;
        let mut var_eta0cv_i_dn11: f64 = *var_eta0cv_i_dn11_slot;
        let mut var_eta0cv_i_dn13: f64 = *var_eta0cv_i_dn13_slot;
        let mut var_eta0cv_i_dn14: f64 = *var_eta0cv_i_dn14_slot;
        let mut var_eta0cv_i_dn2: f64 = *var_eta0cv_i_dn2_slot;
        let mut var_eta0cv_i_dn3: f64 = *var_eta0cv_i_dn3_slot;
        let mut var_eta0cv_i_dn4: f64 = *var_eta0cv_i_dn4_slot;
        let mut var_eta0cv_i_dn5: f64 = *var_eta0cv_i_dn5_slot;
        let mut var_eta0cv_i_dn6: f64 = *var_eta0cv_i_dn6_slot;
        let mut var_eta0cv_i_dn7: f64 = *var_eta0cv_i_dn7_slot;
        let mut var_eta0cv_i_dn8: f64 = *var_eta0cv_i_dn8_slot;
        let mut var_eta0cv_i_dn9: f64 = *var_eta0cv_i_dn9_slot;
        let mut var_eta0cv_i_rv: f64 = *var_eta0cv_i_rv_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard49_rv: f64 = *var_guard49_rv_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard50_rv: f64 = *var_guard50_rv_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard51_rv: f64 = *var_guard51_rv_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard53_rv: f64 = *var_guard53_rv_slot;
        let mut var_mfq1nom_i: f64 = *var_mfq1nom_i_slot;
        let mut var_mfq1nom_i_rv: f64 = *var_mfq1nom_i_rv_slot;
        let mut var_mfq2nom_i: f64 = *var_mfq2nom_i_slot;
        let mut var_mfq2nom_i_rv: f64 = *var_mfq2nom_i_rv_slot;
        let mut var_ssp1_i: f64 = *var_ssp1_i_slot;
        let mut var_ssp1_i_rv: f64 = *var_ssp1_i_rv_slot;
        let mut var_ssp2_i: f64 = *var_ssp2_i_slot;
        let mut var_ssp2_i_rv: f64 = *var_ssp2_i_rv_slot;
        let mut var_ssp3_i: f64 = *var_ssp3_i_slot;
        let mut var_ssp3_i_rv: f64 = *var_ssp3_i_rv_slot;
        let mut var_u0cv_i: f64 = *var_u0cv_i_slot;
        let mut var_u0cv_i_dn0: f64 = *var_u0cv_i_dn0_slot;
        let mut var_u0cv_i_dn10: f64 = *var_u0cv_i_dn10_slot;
        let mut var_u0cv_i_dn11: f64 = *var_u0cv_i_dn11_slot;
        let mut var_u0cv_i_dn13: f64 = *var_u0cv_i_dn13_slot;
        let mut var_u0cv_i_dn14: f64 = *var_u0cv_i_dn14_slot;
        let mut var_u0cv_i_dn2: f64 = *var_u0cv_i_dn2_slot;
        let mut var_u0cv_i_dn3: f64 = *var_u0cv_i_dn3_slot;
        let mut var_u0cv_i_dn4: f64 = *var_u0cv_i_dn4_slot;
        let mut var_u0cv_i_dn5: f64 = *var_u0cv_i_dn5_slot;
        let mut var_u0cv_i_dn6: f64 = *var_u0cv_i_dn6_slot;
        let mut var_u0cv_i_dn7: f64 = *var_u0cv_i_dn7_slot;
        let mut var_u0cv_i_dn8: f64 = *var_u0cv_i_dn8_slot;
        let mut var_u0cv_i_dn9: f64 = *var_u0cv_i_dn9_slot;
        let mut var_u0cv_i_rv: f64 = *var_u0cv_i_rv_slot;
        let mut var_ua1cv_i: f64 = *var_ua1cv_i_slot;
        let mut var_ua1cv_i_rv: f64 = *var_ua1cv_i_rv_slot;
        let mut var_ua2cv_i: f64 = *var_ua2cv_i_slot;
        let mut var_ua2cv_i_rv: f64 = *var_ua2cv_i_rv_slot;
        let mut var_uc1cv_i: f64 = *var_uc1cv_i_slot;
        let mut var_uc1cv_i_rv: f64 = *var_uc1cv_i_rv_slot;
        let mut var_uccv_i: f64 = *var_uccv_i_slot;
        let mut var_uccv_i_rv: f64 = *var_uccv_i_rv_slot;
        let mut var_ud1cv_i: f64 = *var_ud1cv_i_slot;
        let mut var_ud1cv_i_rv: f64 = *var_ud1cv_i_rv_slot;
        let mut var_ud2cv_i: f64 = *var_ud2cv_i_slot;
        let mut var_ud2cv_i_rv: f64 = *var_ud2cv_i_rv_slot;
        let mut var_utlcv_i: f64 = *var_utlcv_i_slot;
        let mut var_utlcv_i_rv: f64 = *var_utlcv_i_rv_slot;

        let (assign6340_e9548,) = {
    if (var_guard47 != 0.0) {
        let assign6340_e9529: f64 = (var_inv_l * p.p780);
        let assign6340_e9530: f64 = (p.p779 + assign6340_e9529);
        let assign6340_e9533: f64 = (var_inv_nfin * p.p781);
        let assign6340_e9534: f64 = (assign6340_e9530 + assign6340_e9533);
        let assign6340_e9537: f64 = (var_inv_lnfin * p.p782);
        let assign6340_e9538: f64 = (assign6340_e9534 + assign6340_e9537);
        let assign6340_e9541: f64 = (var_inv_w * p.p783);
        let assign6340_e9542: f64 = (assign6340_e9538 + assign6340_e9541);
        let assign6340_e9545: f64 = (var_inv_wl * p.p784);
        let assign6340_e9546: f64 = (assign6340_e9542 + assign6340_e9545);
        (assign6340_e9546,)
    } else {
        (var_utlcv_i,)
    }
};
        var_utlcv_i = assign6340_e9548;
        var_utlcv_i_rv = 0.0;

        let (assign6350_e9572,) = {
    if (var_guard47 != 0.0) {
        let assign6350_e9553: f64 = (var_inv_l * p.p804);
        let assign6350_e9554: f64 = (p.p803 + assign6350_e9553);
        let assign6350_e9557: f64 = (var_inv_nfin * p.p805);
        let assign6350_e9558: f64 = (assign6350_e9554 + assign6350_e9557);
        let assign6350_e9561: f64 = (var_inv_lnfin * p.p806);
        let assign6350_e9562: f64 = (assign6350_e9558 + assign6350_e9561);
        let assign6350_e9565: f64 = (var_inv_w * p.p807);
        let assign6350_e9566: f64 = (assign6350_e9562 + assign6350_e9565);
        let assign6350_e9569: f64 = (var_inv_wl * p.p808);
        let assign6350_e9570: f64 = (assign6350_e9566 + assign6350_e9569);
        (assign6350_e9570,)
    } else {
        (var_ua1cv_i,)
    }
};
        var_ua1cv_i = assign6350_e9572;
        var_ua1cv_i_rv = 0.0;

        let (assign6360_e9596,) = {
    if (var_guard47 != 0.0) {
        let assign6360_e9577: f64 = (var_inv_l * p.p816);
        let assign6360_e9578: f64 = (p.p815 + assign6360_e9577);
        let assign6360_e9581: f64 = (var_inv_nfin * p.p817);
        let assign6360_e9582: f64 = (assign6360_e9578 + assign6360_e9581);
        let assign6360_e9585: f64 = (var_inv_lnfin * p.p818);
        let assign6360_e9586: f64 = (assign6360_e9582 + assign6360_e9585);
        let assign6360_e9589: f64 = (var_inv_w * p.p819);
        let assign6360_e9590: f64 = (assign6360_e9586 + assign6360_e9589);
        let assign6360_e9593: f64 = (var_inv_wl * p.p820);
        let assign6360_e9594: f64 = (assign6360_e9590 + assign6360_e9593);
        (assign6360_e9594,)
    } else {
        (var_ua2cv_i,)
    }
};
        var_ua2cv_i = assign6360_e9596;
        var_ua2cv_i_rv = 0.0;

        let (assign6370_e9620,) = {
    if (var_guard47 != 0.0) {
        let assign6370_e9601: f64 = (var_inv_l * p.p858);
        let assign6370_e9602: f64 = (p.p857 + assign6370_e9601);
        let assign6370_e9605: f64 = (var_inv_nfin * p.p859);
        let assign6370_e9606: f64 = (assign6370_e9602 + assign6370_e9605);
        let assign6370_e9609: f64 = (var_inv_lnfin * p.p860);
        let assign6370_e9610: f64 = (assign6370_e9606 + assign6370_e9609);
        let assign6370_e9613: f64 = (var_inv_w * p.p861);
        let assign6370_e9614: f64 = (assign6370_e9610 + assign6370_e9613);
        let assign6370_e9617: f64 = (var_inv_wl * p.p862);
        let assign6370_e9618: f64 = (assign6370_e9614 + assign6370_e9617);
        (assign6370_e9618,)
    } else {
        (var_ud1cv_i,)
    }
};
        var_ud1cv_i = assign6370_e9620;
        var_ud1cv_i_rv = 0.0;

        let (assign6380_e9644,) = {
    if (var_guard47 != 0.0) {
        let assign6380_e9625: f64 = (var_inv_l * p.p870);
        let assign6380_e9626: f64 = (p.p869 + assign6380_e9625);
        let assign6380_e9629: f64 = (var_inv_nfin * p.p871);
        let assign6380_e9630: f64 = (assign6380_e9626 + assign6380_e9629);
        let assign6380_e9633: f64 = (var_inv_lnfin * p.p872);
        let assign6380_e9634: f64 = (assign6380_e9630 + assign6380_e9633);
        let assign6380_e9637: f64 = (var_inv_w * p.p873);
        let assign6380_e9638: f64 = (assign6380_e9634 + assign6380_e9637);
        let assign6380_e9641: f64 = (var_inv_wl * p.p874);
        let assign6380_e9642: f64 = (assign6380_e9638 + assign6380_e9641);
        (assign6380_e9642,)
    } else {
        (var_ud2cv_i,)
    }
};
        var_ud2cv_i = assign6380_e9644;
        var_ud2cv_i_rv = 0.0;

        let assign6390_e9647: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard49 = assign6390_e9647;
        var_guard49_rv = 0.0;

        let (assign6400_e9673,) = {
    if ((var_guard47 != 0.0) && (var_guard49 != 0.0)) {
        let assign6400_e9654: f64 = (var_inv_l * p.p672);
        let assign6400_e9655: f64 = (p.p671 + assign6400_e9654);
        let assign6400_e9658: f64 = (var_inv_nfin * p.p673);
        let assign6400_e9659: f64 = (assign6400_e9655 + assign6400_e9658);
        let assign6400_e9662: f64 = (var_inv_lnfin * p.p674);
        let assign6400_e9663: f64 = (assign6400_e9659 + assign6400_e9662);
        let assign6400_e9666: f64 = (var_inv_w * p.p675);
        let assign6400_e9667: f64 = (assign6400_e9663 + assign6400_e9666);
        let assign6400_e9670: f64 = (var_inv_wl * p.p676);
        let assign6400_e9671: f64 = (assign6400_e9667 + assign6400_e9670);
        (assign6400_e9671,)
    } else {
        (var_uccv_i,)
    }
};
        var_uccv_i = assign6400_e9673;
        var_uccv_i_rv = 0.0;

        let (assign6410_e9699,) = {
    if ((var_guard47 != 0.0) && (var_guard49 != 0.0)) {
        let assign6410_e9680: f64 = (var_inv_l * p.p840);
        let assign6410_e9681: f64 = (p.p839 + assign6410_e9680);
        let assign6410_e9684: f64 = (var_inv_nfin * p.p841);
        let assign6410_e9685: f64 = (assign6410_e9681 + assign6410_e9684);
        let assign6410_e9688: f64 = (var_inv_lnfin * p.p842);
        let assign6410_e9689: f64 = (assign6410_e9685 + assign6410_e9688);
        let assign6410_e9692: f64 = (var_inv_w * p.p843);
        let assign6410_e9693: f64 = (assign6410_e9689 + assign6410_e9692);
        let assign6410_e9696: f64 = (var_inv_wl * p.p844);
        let assign6410_e9697: f64 = (assign6410_e9693 + assign6410_e9696);
        (assign6410_e9697,)
    } else {
        (var_uc1cv_i,)
    }
};
        var_uc1cv_i = assign6410_e9699;
        var_uc1cv_i_rv = 0.0;

        let (assign6420_e9723, assign6420_e9723_d_n0, assign6420_e9723_d_n2, assign6420_e9723_d_n3, assign6420_e9723_d_n4, assign6420_e9723_d_n5, assign6420_e9723_d_n6, assign6420_e9723_d_n7, assign6420_e9723_d_n8, assign6420_e9723_d_n9, assign6420_e9723_d_n10, assign6420_e9723_d_n11, assign6420_e9723_d_n13, assign6420_e9723_d_n14,) = {
    if (var_guard47 != 0.0) {
        let assign6420_e9704: f64 = (var_inv_l * p.p261);
        let assign6420_e9705: f64 = (p.p260 + assign6420_e9704);
        let assign6420_e9708: f64 = (var_inv_nfin * p.p262);
        let assign6420_e9709: f64 = (assign6420_e9705 + assign6420_e9708);
        let assign6420_e9712: f64 = (var_inv_lnfin * p.p263);
        let assign6420_e9713: f64 = (assign6420_e9709 + assign6420_e9712);
        let assign6420_e9716: f64 = (var_inv_w * p.p264);
        let assign6420_e9717: f64 = (assign6420_e9713 + assign6420_e9716);
        let assign6420_e9720: f64 = (var_inv_wl * p.p265);
        let assign6420_e9721: f64 = (assign6420_e9717 + assign6420_e9720);
        (assign6420_e9721, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eta0cv_i, var_eta0cv_i_dn0, var_eta0cv_i_dn2, var_eta0cv_i_dn3, var_eta0cv_i_dn4, var_eta0cv_i_dn5, var_eta0cv_i_dn6, var_eta0cv_i_dn7, var_eta0cv_i_dn8, var_eta0cv_i_dn9, var_eta0cv_i_dn10, var_eta0cv_i_dn11, var_eta0cv_i_dn13, var_eta0cv_i_dn14,)
    }
};
        var_eta0cv_i = assign6420_e9723;
        var_eta0cv_i_dn0 = assign6420_e9723_d_n0;
        var_eta0cv_i_dn2 = assign6420_e9723_d_n2;
        var_eta0cv_i_dn3 = assign6420_e9723_d_n3;
        var_eta0cv_i_dn4 = assign6420_e9723_d_n4;
        var_eta0cv_i_dn5 = assign6420_e9723_d_n5;
        var_eta0cv_i_dn6 = assign6420_e9723_d_n6;
        var_eta0cv_i_dn7 = assign6420_e9723_d_n7;
        var_eta0cv_i_dn8 = assign6420_e9723_d_n8;
        var_eta0cv_i_dn9 = assign6420_e9723_d_n9;
        var_eta0cv_i_dn10 = assign6420_e9723_d_n10;
        var_eta0cv_i_dn11 = assign6420_e9723_d_n11;
        var_eta0cv_i_dn13 = assign6420_e9723_d_n13;
        var_eta0cv_i_dn14 = assign6420_e9723_d_n14;
        var_eta0cv_i_rv = 0.0;

        let assign6430_e9726: f64 = if p.p161 != 0.0 { 1.0 } else { 0.0 };
        var_guard50 = assign6430_e9726;
        var_guard50_rv = 0.0;

        let (assign6440_e9765, assign6440_e9765_d_n0, assign6440_e9765_d_n2, assign6440_e9765_d_n3, assign6440_e9765_d_n4, assign6440_e9765_d_n5, assign6440_e9765_d_n6, assign6440_e9765_d_n7, assign6440_e9765_d_n8, assign6440_e9765_d_n9, assign6440_e9765_d_n10, assign6440_e9765_d_n11, assign6440_e9765_d_n13, assign6440_e9765_d_n14,) = {
    if ((var_guard47 != 0.0) && (var_guard50 != 0.0)) {
        let assign6440_e9734: f64 = (p.p161 / p.p5);
        let assign6440_e9738: f64 = (p.p5 / p.p162);
        let assign6440_e9739: f64 = (1.0 + assign6440_e9738);
        let (assign6440_e9760,) = {
            if (!(assign6440_e9739 > 1e-38)) {
                let assign6440_e9744: f64 = (-87.498233534);
                (assign6440_e9744,)
            } else {
                let assign6440_e9748: f64 = (p.p5 / p.p162);
                let assign6440_e9749: f64 = (1.0 + assign6440_e9748);
                let (assign6440_e9759,) = {
                    if (assign6440_e9749 > 1e-38) {
                        let assign6440_e9755: f64 = (p.p5 / p.p162);
                        let assign6440_e9756: f64 = (1.0 + assign6440_e9755);
                        let assign6440_e9757: f64 = (assign6440_e9756).ln();
                        (assign6440_e9757,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6440_e9759,)
            }
        };
        let assign6440_e9761: f64 = (assign6440_e9734 * assign6440_e9760);
        let assign6440_e9762: f64 = (1.0 + assign6440_e9761);
        let assign6440_e9763: f64 = (var_eta0cv_i * assign6440_e9762);
        (assign6440_e9763, (var_eta0cv_i_dn0 * assign6440_e9762), (var_eta0cv_i_dn2 * assign6440_e9762), (var_eta0cv_i_dn3 * assign6440_e9762), (var_eta0cv_i_dn4 * assign6440_e9762), (var_eta0cv_i_dn5 * assign6440_e9762), (var_eta0cv_i_dn6 * assign6440_e9762), (var_eta0cv_i_dn7 * assign6440_e9762), (var_eta0cv_i_dn8 * assign6440_e9762), (var_eta0cv_i_dn9 * assign6440_e9762), (var_eta0cv_i_dn10 * assign6440_e9762), (var_eta0cv_i_dn11 * assign6440_e9762), (var_eta0cv_i_dn13 * assign6440_e9762), (var_eta0cv_i_dn14 * assign6440_e9762),)
    } else {
        (var_eta0cv_i, var_eta0cv_i_dn0, var_eta0cv_i_dn2, var_eta0cv_i_dn3, var_eta0cv_i_dn4, var_eta0cv_i_dn5, var_eta0cv_i_dn6, var_eta0cv_i_dn7, var_eta0cv_i_dn8, var_eta0cv_i_dn9, var_eta0cv_i_dn10, var_eta0cv_i_dn11, var_eta0cv_i_dn13, var_eta0cv_i_dn14,)
    }
};
        var_eta0cv_i = assign6440_e9765;
        var_eta0cv_i_dn0 = assign6440_e9765_d_n0;
        var_eta0cv_i_dn2 = assign6440_e9765_d_n2;
        var_eta0cv_i_dn3 = assign6440_e9765_d_n3;
        var_eta0cv_i_dn4 = assign6440_e9765_d_n4;
        var_eta0cv_i_dn5 = assign6440_e9765_d_n5;
        var_eta0cv_i_dn6 = assign6440_e9765_d_n6;
        var_eta0cv_i_dn7 = assign6440_e9765_d_n7;
        var_eta0cv_i_dn8 = assign6440_e9765_d_n8;
        var_eta0cv_i_dn9 = assign6440_e9765_d_n9;
        var_eta0cv_i_dn10 = assign6440_e9765_d_n10;
        var_eta0cv_i_dn11 = assign6440_e9765_d_n11;
        var_eta0cv_i_dn13 = assign6440_e9765_d_n13;
        var_eta0cv_i_dn14 = assign6440_e9765_d_n14;
        var_eta0cv_i_rv = 0.0;

        let assign6450_e9768: f64 = if p.p21 != 0.0 { 1.0 } else { 0.0 };
        var_guard51 = assign6450_e9768;
        var_guard51_rv = 0.0;

        let (assign6460_e9784, assign6460_e9784_d_n0, assign6460_e9784_d_n2, assign6460_e9784_d_n3, assign6460_e9784_d_n4, assign6460_e9784_d_n5, assign6460_e9784_d_n6, assign6460_e9784_d_n7, assign6460_e9784_d_n8, assign6460_e9784_d_n9, assign6460_e9784_d_n10, assign6460_e9784_d_n11, assign6460_e9784_d_n13, assign6460_e9784_d_n14,) = {
    if ((var_guard47 != 0.0) && (var_guard51 != 0.0)) {
        let assign6460_e9776: f64 = (p.p5 - p.p21);
        let assign6460_e9778: f64 = (assign6460_e9776 * p.p588);
        let assign6460_e9780: f64 = (assign6460_e9778 * var_leff_1);
        let assign6460_e9781: f64 = (1.0 + assign6460_e9780);
        let assign6460_e9782: f64 = (var_u0cv_i * assign6460_e9781);
        (assign6460_e9782, ((var_u0cv_i_dn0 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn0))), ((var_u0cv_i_dn2 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn2))), ((var_u0cv_i_dn3 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn3))), ((var_u0cv_i_dn4 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn4))), ((var_u0cv_i_dn5 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn5))), ((var_u0cv_i_dn6 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn6))), ((var_u0cv_i_dn7 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn7))), ((var_u0cv_i_dn8 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn8))), ((var_u0cv_i_dn9 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn9))), ((var_u0cv_i_dn10 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn10))), ((var_u0cv_i_dn11 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn11))), ((var_u0cv_i_dn13 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn13))), ((var_u0cv_i_dn14 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn14))),)
    } else {
        (var_u0cv_i, var_u0cv_i_dn0, var_u0cv_i_dn2, var_u0cv_i_dn3, var_u0cv_i_dn4, var_u0cv_i_dn5, var_u0cv_i_dn6, var_u0cv_i_dn7, var_u0cv_i_dn8, var_u0cv_i_dn9, var_u0cv_i_dn10, var_u0cv_i_dn11, var_u0cv_i_dn13, var_u0cv_i_dn14,)
    }
};
        var_u0cv_i = assign6460_e9784;
        var_u0cv_i_dn0 = assign6460_e9784_d_n0;
        var_u0cv_i_dn2 = assign6460_e9784_d_n2;
        var_u0cv_i_dn3 = assign6460_e9784_d_n3;
        var_u0cv_i_dn4 = assign6460_e9784_d_n4;
        var_u0cv_i_dn5 = assign6460_e9784_d_n5;
        var_u0cv_i_dn6 = assign6460_e9784_d_n6;
        var_u0cv_i_dn7 = assign6460_e9784_d_n7;
        var_u0cv_i_dn8 = assign6460_e9784_d_n8;
        var_u0cv_i_dn9 = assign6460_e9784_d_n9;
        var_u0cv_i_dn10 = assign6460_e9784_d_n10;
        var_u0cv_i_dn11 = assign6460_e9784_d_n11;
        var_u0cv_i_dn13 = assign6460_e9784_d_n13;
        var_u0cv_i_dn14 = assign6460_e9784_d_n14;
        var_u0cv_i_rv = 0.0;

        let (assign6470_e9800, assign6470_e9800_d_n0, assign6470_e9800_d_n2, assign6470_e9800_d_n3, assign6470_e9800_d_n4, assign6470_e9800_d_n5, assign6470_e9800_d_n6, assign6470_e9800_d_n7, assign6470_e9800_d_n8, assign6470_e9800_d_n9, assign6470_e9800_d_n10, assign6470_e9800_d_n11, assign6470_e9800_d_n13, assign6470_e9800_d_n14,) = {
    if ((var_guard47 != 0.0) && (var_guard51 != 0.0)) {
        let assign6470_e9792: f64 = (p.p5 - p.p21);
        let assign6470_e9794: f64 = (assign6470_e9792 * p.p163);
        let assign6470_e9796: f64 = (assign6470_e9794 * var_leff_1);
        let assign6470_e9797: f64 = (1.0 + assign6470_e9796);
        let assign6470_e9798: f64 = (var_eta0cv_i * assign6470_e9797);
        (assign6470_e9798, ((var_eta0cv_i_dn0 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn0))), ((var_eta0cv_i_dn2 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn2))), ((var_eta0cv_i_dn3 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn3))), ((var_eta0cv_i_dn4 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn4))), ((var_eta0cv_i_dn5 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn5))), ((var_eta0cv_i_dn6 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn6))), ((var_eta0cv_i_dn7 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn7))), ((var_eta0cv_i_dn8 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn8))), ((var_eta0cv_i_dn9 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn9))), ((var_eta0cv_i_dn10 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn10))), ((var_eta0cv_i_dn11 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn11))), ((var_eta0cv_i_dn13 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn13))), ((var_eta0cv_i_dn14 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn14))),)
    } else {
        (var_eta0cv_i, var_eta0cv_i_dn0, var_eta0cv_i_dn2, var_eta0cv_i_dn3, var_eta0cv_i_dn4, var_eta0cv_i_dn5, var_eta0cv_i_dn6, var_eta0cv_i_dn7, var_eta0cv_i_dn8, var_eta0cv_i_dn9, var_eta0cv_i_dn10, var_eta0cv_i_dn11, var_eta0cv_i_dn13, var_eta0cv_i_dn14,)
    }
};
        var_eta0cv_i = assign6470_e9800;
        var_eta0cv_i_dn0 = assign6470_e9800_d_n0;
        var_eta0cv_i_dn2 = assign6470_e9800_d_n2;
        var_eta0cv_i_dn3 = assign6470_e9800_d_n3;
        var_eta0cv_i_dn4 = assign6470_e9800_d_n4;
        var_eta0cv_i_dn5 = assign6470_e9800_d_n5;
        var_eta0cv_i_dn6 = assign6470_e9800_d_n6;
        var_eta0cv_i_dn7 = assign6470_e9800_d_n7;
        var_eta0cv_i_dn8 = assign6470_e9800_d_n8;
        var_eta0cv_i_dn9 = assign6470_e9800_d_n9;
        var_eta0cv_i_dn10 = assign6470_e9800_d_n10;
        var_eta0cv_i_dn11 = assign6470_e9800_d_n11;
        var_eta0cv_i_dn13 = assign6470_e9800_d_n13;
        var_eta0cv_i_dn14 = assign6470_e9800_d_n14;
        var_eta0cv_i_rv = 0.0;

        let assign6510_e9858: f64 = if p.p57 == 1.0 { 1.0 } else { 0.0 };
        var_guard53 = assign6510_e9858;
        var_guard53_rv = 0.0;

        let (assign6520_e9882,) = {
    if (var_guard53 != 0.0) {
        let assign6520_e9863: f64 = (var_inv_l * p.p1808);
        let assign6520_e9864: f64 = (p.p1807 + assign6520_e9863);
        let assign6520_e9867: f64 = (var_inv_nfin * p.p1809);
        let assign6520_e9868: f64 = (assign6520_e9864 + assign6520_e9867);
        let assign6520_e9871: f64 = (var_inv_lnfin * p.p1810);
        let assign6520_e9872: f64 = (assign6520_e9868 + assign6520_e9871);
        let assign6520_e9875: f64 = (var_inv_w * p.p1811);
        let assign6520_e9876: f64 = (assign6520_e9872 + assign6520_e9875);
        let assign6520_e9879: f64 = (var_inv_wl * p.p1812);
        let assign6520_e9880: f64 = (assign6520_e9876 + assign6520_e9879);
        (assign6520_e9880,)
    } else {
        (var_dimension1_i,)
    }
};
        var_dimension1_i = assign6520_e9882;
        var_dimension1_i_rv = 0.0;

        let (assign6530_e9906,) = {
    if (var_guard53 != 0.0) {
        let assign6530_e9887: f64 = (var_inv_l * p.p1815);
        let assign6530_e9888: f64 = (p.p1814 + assign6530_e9887);
        let assign6530_e9891: f64 = (var_inv_nfin * p.p1816);
        let assign6530_e9892: f64 = (assign6530_e9888 + assign6530_e9891);
        let assign6530_e9895: f64 = (var_inv_lnfin * p.p1817);
        let assign6530_e9896: f64 = (assign6530_e9892 + assign6530_e9895);
        let assign6530_e9899: f64 = (var_inv_w * p.p1818);
        let assign6530_e9900: f64 = (assign6530_e9896 + assign6530_e9899);
        let assign6530_e9903: f64 = (var_inv_wl * p.p1819);
        let assign6530_e9904: f64 = (assign6530_e9900 + assign6530_e9903);
        (assign6530_e9904,)
    } else {
        (var_dimension2_i,)
    }
};
        var_dimension2_i = assign6530_e9906;
        var_dimension2_i_rv = 0.0;

        let (assign6540_e9930,) = {
    if (var_guard53 != 0.0) {
        let assign6540_e9911: f64 = (var_inv_l * p.p1822);
        let assign6540_e9912: f64 = (p.p1821 + assign6540_e9911);
        let assign6540_e9915: f64 = (var_inv_nfin * p.p1823);
        let assign6540_e9916: f64 = (assign6540_e9912 + assign6540_e9915);
        let assign6540_e9919: f64 = (var_inv_lnfin * p.p1824);
        let assign6540_e9920: f64 = (assign6540_e9916 + assign6540_e9919);
        let assign6540_e9923: f64 = (var_inv_w * p.p1825);
        let assign6540_e9924: f64 = (assign6540_e9920 + assign6540_e9923);
        let assign6540_e9927: f64 = (var_inv_wl * p.p1826);
        let assign6540_e9928: f64 = (assign6540_e9924 + assign6540_e9927);
        (assign6540_e9928,)
    } else {
        (var_dimension3_i,)
    }
};
        var_dimension3_i = assign6540_e9930;
        var_dimension3_i_rv = 0.0;

        let (assign6550_e9954,) = {
    if (var_guard53 != 0.0) {
        let assign6550_e9935: f64 = (var_inv_l * p.p1830);
        let assign6550_e9936: f64 = (p.p1829 + assign6550_e9935);
        let assign6550_e9939: f64 = (var_inv_nfin * p.p1831);
        let assign6550_e9940: f64 = (assign6550_e9936 + assign6550_e9939);
        let assign6550_e9943: f64 = (var_inv_lnfin * p.p1832);
        let assign6550_e9944: f64 = (assign6550_e9940 + assign6550_e9943);
        let assign6550_e9947: f64 = (var_inv_w * p.p1833);
        let assign6550_e9948: f64 = (assign6550_e9944 + assign6550_e9947);
        let assign6550_e9951: f64 = (var_inv_wl * p.p1834);
        let assign6550_e9952: f64 = (assign6550_e9948 + assign6550_e9951);
        (assign6550_e9952,)
    } else {
        (var_ssp1_i,)
    }
};
        var_ssp1_i = assign6550_e9954;
        var_ssp1_i_rv = 0.0;

        let (assign6560_e9978,) = {
    if (var_guard53 != 0.0) {
        let assign6560_e9959: f64 = (var_inv_l * p.p1836);
        let assign6560_e9960: f64 = (p.p1835 + assign6560_e9959);
        let assign6560_e9963: f64 = (var_inv_nfin * p.p1837);
        let assign6560_e9964: f64 = (assign6560_e9960 + assign6560_e9963);
        let assign6560_e9967: f64 = (var_inv_lnfin * p.p1838);
        let assign6560_e9968: f64 = (assign6560_e9964 + assign6560_e9967);
        let assign6560_e9971: f64 = (var_inv_w * p.p1839);
        let assign6560_e9972: f64 = (assign6560_e9968 + assign6560_e9971);
        let assign6560_e9975: f64 = (var_inv_wl * p.p1840);
        let assign6560_e9976: f64 = (assign6560_e9972 + assign6560_e9975);
        (assign6560_e9976,)
    } else {
        (var_ssp2_i,)
    }
};
        var_ssp2_i = assign6560_e9978;
        var_ssp2_i_rv = 0.0;

        let (assign6570_e10002,) = {
    if (var_guard53 != 0.0) {
        let assign6570_e9983: f64 = (var_inv_l * p.p1842);
        let assign6570_e9984: f64 = (p.p1841 + assign6570_e9983);
        let assign6570_e9987: f64 = (var_inv_nfin * p.p1843);
        let assign6570_e9988: f64 = (assign6570_e9984 + assign6570_e9987);
        let assign6570_e9991: f64 = (var_inv_lnfin * p.p1844);
        let assign6570_e9992: f64 = (assign6570_e9988 + assign6570_e9991);
        let assign6570_e9995: f64 = (var_inv_w * p.p1845);
        let assign6570_e9996: f64 = (assign6570_e9992 + assign6570_e9995);
        let assign6570_e9999: f64 = (var_inv_wl * p.p1846);
        let assign6570_e10000: f64 = (assign6570_e9996 + assign6570_e9999);
        (assign6570_e10000,)
    } else {
        (var_ssp3_i,)
    }
};
        var_ssp3_i = assign6570_e10002;
        var_ssp3_i_rv = 0.0;

        let (assign6580_e10026,) = {
    if (var_guard53 != 0.0) {
        let assign6580_e10007: f64 = (var_inv_l * p.p1854);
        let assign6580_e10008: f64 = (p.p1853 + assign6580_e10007);
        let assign6580_e10011: f64 = (var_inv_nfin * p.p1855);
        let assign6580_e10012: f64 = (assign6580_e10008 + assign6580_e10011);
        let assign6580_e10015: f64 = (var_inv_lnfin * p.p1856);
        let assign6580_e10016: f64 = (assign6580_e10012 + assign6580_e10015);
        let assign6580_e10019: f64 = (var_inv_w * p.p1857);
        let assign6580_e10020: f64 = (assign6580_e10016 + assign6580_e10019);
        let assign6580_e10023: f64 = (var_inv_wl * p.p1858);
        let assign6580_e10024: f64 = (assign6580_e10020 + assign6580_e10023);
        (assign6580_e10024,)
    } else {
        (var_e2nom_i,)
    }
};
        var_e2nom_i = assign6580_e10026;
        var_e2nom_i_rv = 0.0;

        let (assign6590_e10050,) = {
    if (var_guard53 != 0.0) {
        let assign6590_e10031: f64 = (var_inv_l * p.p1860);
        let assign6590_e10032: f64 = (p.p1859 + assign6590_e10031);
        let assign6590_e10035: f64 = (var_inv_nfin * p.p1861);
        let assign6590_e10036: f64 = (assign6590_e10032 + assign6590_e10035);
        let assign6590_e10039: f64 = (var_inv_lnfin * p.p1862);
        let assign6590_e10040: f64 = (assign6590_e10036 + assign6590_e10039);
        let assign6590_e10043: f64 = (var_inv_w * p.p1863);
        let assign6590_e10044: f64 = (assign6590_e10040 + assign6590_e10043);
        let assign6590_e10047: f64 = (var_inv_wl * p.p1864);
        let assign6590_e10048: f64 = (assign6590_e10044 + assign6590_e10047);
        (assign6590_e10048,)
    } else {
        (var_e3nom_i,)
    }
};
        var_e3nom_i = assign6590_e10050;
        var_e3nom_i_rv = 0.0;

        let (assign6600_e10074,) = {
    if (var_guard53 != 0.0) {
        let assign6600_e10055: f64 = (var_inv_l * p.p1870);
        let assign6600_e10056: f64 = (p.p1869 + assign6600_e10055);
        let assign6600_e10059: f64 = (var_inv_nfin * p.p1871);
        let assign6600_e10060: f64 = (assign6600_e10056 + assign6600_e10059);
        let assign6600_e10063: f64 = (var_inv_lnfin * p.p1872);
        let assign6600_e10064: f64 = (assign6600_e10060 + assign6600_e10063);
        let assign6600_e10067: f64 = (var_inv_w * p.p1873);
        let assign6600_e10068: f64 = (assign6600_e10064 + assign6600_e10067);
        let assign6600_e10071: f64 = (var_inv_wl * p.p1874);
        let assign6600_e10072: f64 = (assign6600_e10068 + assign6600_e10071);
        (assign6600_e10072,)
    } else {
        (var_mfq1nom_i,)
    }
};
        var_mfq1nom_i = assign6600_e10074;
        var_mfq1nom_i_rv = 0.0;

        let (assign6610_e10098,) = {
    if (var_guard53 != 0.0) {
        let assign6610_e10079: f64 = (var_inv_l * p.p1876);
        let assign6610_e10080: f64 = (p.p1875 + assign6610_e10079);
        let assign6610_e10083: f64 = (var_inv_nfin * p.p1877);
        let assign6610_e10084: f64 = (assign6610_e10080 + assign6610_e10083);
        let assign6610_e10087: f64 = (var_inv_lnfin * p.p1878);
        let assign6610_e10088: f64 = (assign6610_e10084 + assign6610_e10087);
        let assign6610_e10091: f64 = (var_inv_w * p.p1879);
        let assign6610_e10092: f64 = (assign6610_e10088 + assign6610_e10091);
        let assign6610_e10095: f64 = (var_inv_wl * p.p1880);
        let assign6610_e10096: f64 = (assign6610_e10092 + assign6610_e10095);
        (assign6610_e10096,)
    } else {
        (var_mfq2nom_i,)
    }
};
        var_mfq2nom_i = assign6610_e10098;
        var_mfq2nom_i_rv = 0.0;

        *var_dimension1_i_slot = var_dimension1_i;
        *var_dimension1_i_rv_slot = var_dimension1_i_rv;
        *var_dimension2_i_slot = var_dimension2_i;
        *var_dimension2_i_rv_slot = var_dimension2_i_rv;
        *var_dimension3_i_slot = var_dimension3_i;
        *var_dimension3_i_rv_slot = var_dimension3_i_rv;
        *var_e2nom_i_slot = var_e2nom_i;
        *var_e2nom_i_rv_slot = var_e2nom_i_rv;
        *var_e3nom_i_slot = var_e3nom_i;
        *var_e3nom_i_rv_slot = var_e3nom_i_rv;
        *var_eta0cv_i_slot = var_eta0cv_i;
        *var_eta0cv_i_dn0_slot = var_eta0cv_i_dn0;
        *var_eta0cv_i_dn10_slot = var_eta0cv_i_dn10;
        *var_eta0cv_i_dn11_slot = var_eta0cv_i_dn11;
        *var_eta0cv_i_dn13_slot = var_eta0cv_i_dn13;
        *var_eta0cv_i_dn14_slot = var_eta0cv_i_dn14;
        *var_eta0cv_i_dn2_slot = var_eta0cv_i_dn2;
        *var_eta0cv_i_dn3_slot = var_eta0cv_i_dn3;
        *var_eta0cv_i_dn4_slot = var_eta0cv_i_dn4;
        *var_eta0cv_i_dn5_slot = var_eta0cv_i_dn5;
        *var_eta0cv_i_dn6_slot = var_eta0cv_i_dn6;
        *var_eta0cv_i_dn7_slot = var_eta0cv_i_dn7;
        *var_eta0cv_i_dn8_slot = var_eta0cv_i_dn8;
        *var_eta0cv_i_dn9_slot = var_eta0cv_i_dn9;
        *var_eta0cv_i_rv_slot = var_eta0cv_i_rv;
        *var_guard49_slot = var_guard49;
        *var_guard49_rv_slot = var_guard49_rv;
        *var_guard50_slot = var_guard50;
        *var_guard50_rv_slot = var_guard50_rv;
        *var_guard51_slot = var_guard51;
        *var_guard51_rv_slot = var_guard51_rv;
        *var_guard53_slot = var_guard53;
        *var_guard53_rv_slot = var_guard53_rv;
        *var_mfq1nom_i_slot = var_mfq1nom_i;
        *var_mfq1nom_i_rv_slot = var_mfq1nom_i_rv;
        *var_mfq2nom_i_slot = var_mfq2nom_i;
        *var_mfq2nom_i_rv_slot = var_mfq2nom_i_rv;
        *var_ssp1_i_slot = var_ssp1_i;
        *var_ssp1_i_rv_slot = var_ssp1_i_rv;
        *var_ssp2_i_slot = var_ssp2_i;
        *var_ssp2_i_rv_slot = var_ssp2_i_rv;
        *var_ssp3_i_slot = var_ssp3_i;
        *var_ssp3_i_rv_slot = var_ssp3_i_rv;
        *var_u0cv_i_slot = var_u0cv_i;
        *var_u0cv_i_dn0_slot = var_u0cv_i_dn0;
        *var_u0cv_i_dn10_slot = var_u0cv_i_dn10;
        *var_u0cv_i_dn11_slot = var_u0cv_i_dn11;
        *var_u0cv_i_dn13_slot = var_u0cv_i_dn13;
        *var_u0cv_i_dn14_slot = var_u0cv_i_dn14;
        *var_u0cv_i_dn2_slot = var_u0cv_i_dn2;
        *var_u0cv_i_dn3_slot = var_u0cv_i_dn3;
        *var_u0cv_i_dn4_slot = var_u0cv_i_dn4;
        *var_u0cv_i_dn5_slot = var_u0cv_i_dn5;
        *var_u0cv_i_dn6_slot = var_u0cv_i_dn6;
        *var_u0cv_i_dn7_slot = var_u0cv_i_dn7;
        *var_u0cv_i_dn8_slot = var_u0cv_i_dn8;
        *var_u0cv_i_dn9_slot = var_u0cv_i_dn9;
        *var_u0cv_i_rv_slot = var_u0cv_i_rv;
        *var_ua1cv_i_slot = var_ua1cv_i;
        *var_ua1cv_i_rv_slot = var_ua1cv_i_rv;
        *var_ua2cv_i_slot = var_ua2cv_i;
        *var_ua2cv_i_rv_slot = var_ua2cv_i_rv;
        *var_uc1cv_i_slot = var_uc1cv_i;
        *var_uc1cv_i_rv_slot = var_uc1cv_i_rv;
        *var_uccv_i_slot = var_uccv_i;
        *var_uccv_i_rv_slot = var_uccv_i_rv;
        *var_ud1cv_i_slot = var_ud1cv_i;
        *var_ud1cv_i_rv_slot = var_ud1cv_i_rv;
        *var_ud2cv_i_slot = var_ud2cv_i;
        *var_ud2cv_i_rv_slot = var_ud2cv_i_rv;
        *var_utlcv_i_slot = var_utlcv_i;
        *var_utlcv_i_rv_slot = var_utlcv_i_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_guard53: f64,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_cdsc_i_slot: &mut f64,
        var_cdsc_i_rv_slot: &mut f64,
        var_cdscd_i_slot: &mut f64,
        var_cdscd_i_rv_slot: &mut f64,
        var_cdscdr_i_slot: &mut f64,
        var_cdscdr_i_rv_slot: &mut f64,
        var_eta0_i_slot: &mut f64,
        var_eta0_i_dn0_slot: &mut f64,
        var_eta0_i_dn10_slot: &mut f64,
        var_eta0_i_dn11_slot: &mut f64,
        var_eta0_i_dn13_slot: &mut f64,
        var_eta0_i_dn14_slot: &mut f64,
        var_eta0_i_dn2_slot: &mut f64,
        var_eta0_i_dn3_slot: &mut f64,
        var_eta0_i_dn4_slot: &mut f64,
        var_eta0_i_dn5_slot: &mut f64,
        var_eta0_i_dn6_slot: &mut f64,
        var_eta0_i_dn7_slot: &mut f64,
        var_eta0_i_dn8_slot: &mut f64,
        var_eta0_i_dn9_slot: &mut f64,
        var_eta0_i_rv_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard54_rv_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard55_rv_slot: &mut f64,
        var_guard56_slot: &mut f64,
        var_guard56_rv_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard57_rv_slot: &mut f64,
        var_guard58_slot: &mut f64,
        var_guard58_rv_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard59_rv_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard60_rv_slot: &mut f64,
        var_guard61_slot: &mut f64,
        var_guard61_rv_slot: &mut f64,
        var_guard62_slot: &mut f64,
        var_guard62_rv_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard63_rv_slot: &mut f64,
        var_mfq3nom_i_slot: &mut f64,
        var_mfq3nom_i_rv_slot: &mut f64,
        var_phig_i_slot: &mut f64,
        var_phig_i_dn0_slot: &mut f64,
        var_phig_i_dn10_slot: &mut f64,
        var_phig_i_dn11_slot: &mut f64,
        var_phig_i_dn13_slot: &mut f64,
        var_phig_i_dn14_slot: &mut f64,
        var_phig_i_dn2_slot: &mut f64,
        var_phig_i_dn3_slot: &mut f64,
        var_phig_i_dn4_slot: &mut f64,
        var_phig_i_dn5_slot: &mut f64,
        var_phig_i_dn6_slot: &mut f64,
        var_phig_i_dn7_slot: &mut f64,
        var_phig_i_dn8_slot: &mut f64,
        var_phig_i_dn9_slot: &mut f64,
        var_phig_i_rv_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_dn0_slot: &mut f64,
        var_u0_i_dn10_slot: &mut f64,
        var_u0_i_dn11_slot: &mut f64,
        var_u0_i_dn13_slot: &mut f64,
        var_u0_i_dn14_slot: &mut f64,
        var_u0_i_dn2_slot: &mut f64,
        var_u0_i_dn3_slot: &mut f64,
        var_u0_i_dn4_slot: &mut f64,
        var_u0_i_dn5_slot: &mut f64,
        var_u0_i_dn6_slot: &mut f64,
        var_u0_i_dn7_slot: &mut f64,
        var_u0_i_dn8_slot: &mut f64,
        var_u0_i_dn9_slot: &mut f64,
        var_u0_i_rv_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_dn0_slot: &mut f64,
        var_u0r_i_dn10_slot: &mut f64,
        var_u0r_i_dn11_slot: &mut f64,
        var_u0r_i_dn13_slot: &mut f64,
        var_u0r_i_dn14_slot: &mut f64,
        var_u0r_i_dn2_slot: &mut f64,
        var_u0r_i_dn3_slot: &mut f64,
        var_u0r_i_dn4_slot: &mut f64,
        var_u0r_i_dn5_slot: &mut f64,
        var_u0r_i_dn6_slot: &mut f64,
        var_u0r_i_dn7_slot: &mut f64,
        var_u0r_i_dn8_slot: &mut f64,
        var_u0r_i_dn9_slot: &mut f64,
        var_u0r_i_rv_slot: &mut f64,
        var_vsat1_i_slot: &mut f64,
        var_vsat1_i_dn0_slot: &mut f64,
        var_vsat1_i_dn10_slot: &mut f64,
        var_vsat1_i_dn11_slot: &mut f64,
        var_vsat1_i_dn13_slot: &mut f64,
        var_vsat1_i_dn14_slot: &mut f64,
        var_vsat1_i_dn2_slot: &mut f64,
        var_vsat1_i_dn3_slot: &mut f64,
        var_vsat1_i_dn4_slot: &mut f64,
        var_vsat1_i_dn5_slot: &mut f64,
        var_vsat1_i_dn6_slot: &mut f64,
        var_vsat1_i_dn7_slot: &mut f64,
        var_vsat1_i_dn8_slot: &mut f64,
        var_vsat1_i_dn9_slot: &mut f64,
        var_vsat1_i_rv_slot: &mut f64,
        var_vsat1r_i_slot: &mut f64,
        var_vsat1r_i_dn0_slot: &mut f64,
        var_vsat1r_i_dn10_slot: &mut f64,
        var_vsat1r_i_dn11_slot: &mut f64,
        var_vsat1r_i_dn13_slot: &mut f64,
        var_vsat1r_i_dn14_slot: &mut f64,
        var_vsat1r_i_dn2_slot: &mut f64,
        var_vsat1r_i_dn3_slot: &mut f64,
        var_vsat1r_i_dn4_slot: &mut f64,
        var_vsat1r_i_dn5_slot: &mut f64,
        var_vsat1r_i_dn6_slot: &mut f64,
        var_vsat1r_i_dn7_slot: &mut f64,
        var_vsat1r_i_dn8_slot: &mut f64,
        var_vsat1r_i_dn9_slot: &mut f64,
        var_vsat1r_i_rv_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsat_i_dn0_slot: &mut f64,
        var_vsat_i_dn10_slot: &mut f64,
        var_vsat_i_dn11_slot: &mut f64,
        var_vsat_i_dn13_slot: &mut f64,
        var_vsat_i_dn14_slot: &mut f64,
        var_vsat_i_dn2_slot: &mut f64,
        var_vsat_i_dn3_slot: &mut f64,
        var_vsat_i_dn4_slot: &mut f64,
        var_vsat_i_dn5_slot: &mut f64,
        var_vsat_i_dn6_slot: &mut f64,
        var_vsat_i_dn7_slot: &mut f64,
        var_vsat_i_dn8_slot: &mut f64,
        var_vsat_i_dn9_slot: &mut f64,
        var_vsat_i_rv_slot: &mut f64,
    ) {
        let mut var_cdsc_i: f64 = *var_cdsc_i_slot;
        let mut var_cdsc_i_rv: f64 = *var_cdsc_i_rv_slot;
        let mut var_cdscd_i: f64 = *var_cdscd_i_slot;
        let mut var_cdscd_i_rv: f64 = *var_cdscd_i_rv_slot;
        let mut var_cdscdr_i: f64 = *var_cdscdr_i_slot;
        let mut var_cdscdr_i_rv: f64 = *var_cdscdr_i_rv_slot;
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta0_i_dn0: f64 = *var_eta0_i_dn0_slot;
        let mut var_eta0_i_dn10: f64 = *var_eta0_i_dn10_slot;
        let mut var_eta0_i_dn11: f64 = *var_eta0_i_dn11_slot;
        let mut var_eta0_i_dn13: f64 = *var_eta0_i_dn13_slot;
        let mut var_eta0_i_dn14: f64 = *var_eta0_i_dn14_slot;
        let mut var_eta0_i_dn2: f64 = *var_eta0_i_dn2_slot;
        let mut var_eta0_i_dn3: f64 = *var_eta0_i_dn3_slot;
        let mut var_eta0_i_dn4: f64 = *var_eta0_i_dn4_slot;
        let mut var_eta0_i_dn5: f64 = *var_eta0_i_dn5_slot;
        let mut var_eta0_i_dn6: f64 = *var_eta0_i_dn6_slot;
        let mut var_eta0_i_dn7: f64 = *var_eta0_i_dn7_slot;
        let mut var_eta0_i_dn8: f64 = *var_eta0_i_dn8_slot;
        let mut var_eta0_i_dn9: f64 = *var_eta0_i_dn9_slot;
        let mut var_eta0_i_rv: f64 = *var_eta0_i_rv_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard54_rv: f64 = *var_guard54_rv_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard55_rv: f64 = *var_guard55_rv_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
        let mut var_guard56_rv: f64 = *var_guard56_rv_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard57_rv: f64 = *var_guard57_rv_slot;
        let mut var_guard58: f64 = *var_guard58_slot;
        let mut var_guard58_rv: f64 = *var_guard58_rv_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard59_rv: f64 = *var_guard59_rv_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard60_rv: f64 = *var_guard60_rv_slot;
        let mut var_guard61: f64 = *var_guard61_slot;
        let mut var_guard61_rv: f64 = *var_guard61_rv_slot;
        let mut var_guard62: f64 = *var_guard62_slot;
        let mut var_guard62_rv: f64 = *var_guard62_rv_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard63_rv: f64 = *var_guard63_rv_slot;
        let mut var_mfq3nom_i: f64 = *var_mfq3nom_i_slot;
        let mut var_mfq3nom_i_rv: f64 = *var_mfq3nom_i_rv_slot;
        let mut var_phig_i: f64 = *var_phig_i_slot;
        let mut var_phig_i_dn0: f64 = *var_phig_i_dn0_slot;
        let mut var_phig_i_dn10: f64 = *var_phig_i_dn10_slot;
        let mut var_phig_i_dn11: f64 = *var_phig_i_dn11_slot;
        let mut var_phig_i_dn13: f64 = *var_phig_i_dn13_slot;
        let mut var_phig_i_dn14: f64 = *var_phig_i_dn14_slot;
        let mut var_phig_i_dn2: f64 = *var_phig_i_dn2_slot;
        let mut var_phig_i_dn3: f64 = *var_phig_i_dn3_slot;
        let mut var_phig_i_dn4: f64 = *var_phig_i_dn4_slot;
        let mut var_phig_i_dn5: f64 = *var_phig_i_dn5_slot;
        let mut var_phig_i_dn6: f64 = *var_phig_i_dn6_slot;
        let mut var_phig_i_dn7: f64 = *var_phig_i_dn7_slot;
        let mut var_phig_i_dn8: f64 = *var_phig_i_dn8_slot;
        let mut var_phig_i_dn9: f64 = *var_phig_i_dn9_slot;
        let mut var_phig_i_rv: f64 = *var_phig_i_rv_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_dn0: f64 = *var_u0_i_dn0_slot;
        let mut var_u0_i_dn10: f64 = *var_u0_i_dn10_slot;
        let mut var_u0_i_dn11: f64 = *var_u0_i_dn11_slot;
        let mut var_u0_i_dn13: f64 = *var_u0_i_dn13_slot;
        let mut var_u0_i_dn14: f64 = *var_u0_i_dn14_slot;
        let mut var_u0_i_dn2: f64 = *var_u0_i_dn2_slot;
        let mut var_u0_i_dn3: f64 = *var_u0_i_dn3_slot;
        let mut var_u0_i_dn4: f64 = *var_u0_i_dn4_slot;
        let mut var_u0_i_dn5: f64 = *var_u0_i_dn5_slot;
        let mut var_u0_i_dn6: f64 = *var_u0_i_dn6_slot;
        let mut var_u0_i_dn7: f64 = *var_u0_i_dn7_slot;
        let mut var_u0_i_dn8: f64 = *var_u0_i_dn8_slot;
        let mut var_u0_i_dn9: f64 = *var_u0_i_dn9_slot;
        let mut var_u0_i_rv: f64 = *var_u0_i_rv_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_dn0: f64 = *var_u0r_i_dn0_slot;
        let mut var_u0r_i_dn10: f64 = *var_u0r_i_dn10_slot;
        let mut var_u0r_i_dn11: f64 = *var_u0r_i_dn11_slot;
        let mut var_u0r_i_dn13: f64 = *var_u0r_i_dn13_slot;
        let mut var_u0r_i_dn14: f64 = *var_u0r_i_dn14_slot;
        let mut var_u0r_i_dn2: f64 = *var_u0r_i_dn2_slot;
        let mut var_u0r_i_dn3: f64 = *var_u0r_i_dn3_slot;
        let mut var_u0r_i_dn4: f64 = *var_u0r_i_dn4_slot;
        let mut var_u0r_i_dn5: f64 = *var_u0r_i_dn5_slot;
        let mut var_u0r_i_dn6: f64 = *var_u0r_i_dn6_slot;
        let mut var_u0r_i_dn7: f64 = *var_u0r_i_dn7_slot;
        let mut var_u0r_i_dn8: f64 = *var_u0r_i_dn8_slot;
        let mut var_u0r_i_dn9: f64 = *var_u0r_i_dn9_slot;
        let mut var_u0r_i_rv: f64 = *var_u0r_i_rv_slot;
        let mut var_vsat1_i: f64 = *var_vsat1_i_slot;
        let mut var_vsat1_i_dn0: f64 = *var_vsat1_i_dn0_slot;
        let mut var_vsat1_i_dn10: f64 = *var_vsat1_i_dn10_slot;
        let mut var_vsat1_i_dn11: f64 = *var_vsat1_i_dn11_slot;
        let mut var_vsat1_i_dn13: f64 = *var_vsat1_i_dn13_slot;
        let mut var_vsat1_i_dn14: f64 = *var_vsat1_i_dn14_slot;
        let mut var_vsat1_i_dn2: f64 = *var_vsat1_i_dn2_slot;
        let mut var_vsat1_i_dn3: f64 = *var_vsat1_i_dn3_slot;
        let mut var_vsat1_i_dn4: f64 = *var_vsat1_i_dn4_slot;
        let mut var_vsat1_i_dn5: f64 = *var_vsat1_i_dn5_slot;
        let mut var_vsat1_i_dn6: f64 = *var_vsat1_i_dn6_slot;
        let mut var_vsat1_i_dn7: f64 = *var_vsat1_i_dn7_slot;
        let mut var_vsat1_i_dn8: f64 = *var_vsat1_i_dn8_slot;
        let mut var_vsat1_i_dn9: f64 = *var_vsat1_i_dn9_slot;
        let mut var_vsat1_i_rv: f64 = *var_vsat1_i_rv_slot;
        let mut var_vsat1r_i: f64 = *var_vsat1r_i_slot;
        let mut var_vsat1r_i_dn0: f64 = *var_vsat1r_i_dn0_slot;
        let mut var_vsat1r_i_dn10: f64 = *var_vsat1r_i_dn10_slot;
        let mut var_vsat1r_i_dn11: f64 = *var_vsat1r_i_dn11_slot;
        let mut var_vsat1r_i_dn13: f64 = *var_vsat1r_i_dn13_slot;
        let mut var_vsat1r_i_dn14: f64 = *var_vsat1r_i_dn14_slot;
        let mut var_vsat1r_i_dn2: f64 = *var_vsat1r_i_dn2_slot;
        let mut var_vsat1r_i_dn3: f64 = *var_vsat1r_i_dn3_slot;
        let mut var_vsat1r_i_dn4: f64 = *var_vsat1r_i_dn4_slot;
        let mut var_vsat1r_i_dn5: f64 = *var_vsat1r_i_dn5_slot;
        let mut var_vsat1r_i_dn6: f64 = *var_vsat1r_i_dn6_slot;
        let mut var_vsat1r_i_dn7: f64 = *var_vsat1r_i_dn7_slot;
        let mut var_vsat1r_i_dn8: f64 = *var_vsat1r_i_dn8_slot;
        let mut var_vsat1r_i_dn9: f64 = *var_vsat1r_i_dn9_slot;
        let mut var_vsat1r_i_rv: f64 = *var_vsat1r_i_rv_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsat_i_dn0: f64 = *var_vsat_i_dn0_slot;
        let mut var_vsat_i_dn10: f64 = *var_vsat_i_dn10_slot;
        let mut var_vsat_i_dn11: f64 = *var_vsat_i_dn11_slot;
        let mut var_vsat_i_dn13: f64 = *var_vsat_i_dn13_slot;
        let mut var_vsat_i_dn14: f64 = *var_vsat_i_dn14_slot;
        let mut var_vsat_i_dn2: f64 = *var_vsat_i_dn2_slot;
        let mut var_vsat_i_dn3: f64 = *var_vsat_i_dn3_slot;
        let mut var_vsat_i_dn4: f64 = *var_vsat_i_dn4_slot;
        let mut var_vsat_i_dn5: f64 = *var_vsat_i_dn5_slot;
        let mut var_vsat_i_dn6: f64 = *var_vsat_i_dn6_slot;
        let mut var_vsat_i_dn7: f64 = *var_vsat_i_dn7_slot;
        let mut var_vsat_i_dn8: f64 = *var_vsat_i_dn8_slot;
        let mut var_vsat_i_dn9: f64 = *var_vsat_i_dn9_slot;
        let mut var_vsat_i_rv: f64 = *var_vsat_i_rv_slot;

        let (assign6620_e10122,) = {
    if (var_guard53 != 0.0) {
        let assign6620_e10103: f64 = (var_inv_l * p.p1882);
        let assign6620_e10104: f64 = (p.p1881 + assign6620_e10103);
        let assign6620_e10107: f64 = (var_inv_nfin * p.p1883);
        let assign6620_e10108: f64 = (assign6620_e10104 + assign6620_e10107);
        let assign6620_e10111: f64 = (var_inv_lnfin * p.p1884);
        let assign6620_e10112: f64 = (assign6620_e10108 + assign6620_e10111);
        let assign6620_e10115: f64 = (var_inv_w * p.p1885);
        let assign6620_e10116: f64 = (assign6620_e10112 + assign6620_e10115);
        let assign6620_e10119: f64 = (var_inv_wl * p.p1886);
        let assign6620_e10120: f64 = (assign6620_e10116 + assign6620_e10119);
        (assign6620_e10120,)
    } else {
        (var_mfq3nom_i,)
    }
};
        var_mfq3nom_i = assign6620_e10122;
        var_mfq3nom_i_rv = 0.0;

        let assign6630_e10125: f64 = if p.p100 != 0.0 { 1.0 } else { 0.0 };
        var_guard54 = assign6630_e10125;
        var_guard54_rv = 0.0;

        let (assign6640_e10162, assign6640_e10162_d_n0, assign6640_e10162_d_n2, assign6640_e10162_d_n3, assign6640_e10162_d_n4, assign6640_e10162_d_n5, assign6640_e10162_d_n6, assign6640_e10162_d_n7, assign6640_e10162_d_n8, assign6640_e10162_d_n9, assign6640_e10162_d_n10, assign6640_e10162_d_n11, assign6640_e10162_d_n13, assign6640_e10162_d_n14,) = {
    if (var_guard54 != 0.0) {
        let assign6640_e10131: f64 = (p.p100 / p.p5);
        let assign6640_e10135: f64 = (p.p5 / p.p101);
        let assign6640_e10136: f64 = (1.0 + assign6640_e10135);
        let (assign6640_e10157,) = {
            if (!(assign6640_e10136 > 1e-38)) {
                let assign6640_e10141: f64 = (-87.498233534);
                (assign6640_e10141,)
            } else {
                let assign6640_e10145: f64 = (p.p5 / p.p101);
                let assign6640_e10146: f64 = (1.0 + assign6640_e10145);
                let (assign6640_e10156,) = {
                    if (assign6640_e10146 > 1e-38) {
                        let assign6640_e10152: f64 = (p.p5 / p.p101);
                        let assign6640_e10153: f64 = (1.0 + assign6640_e10152);
                        let assign6640_e10154: f64 = (assign6640_e10153).ln();
                        (assign6640_e10154,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6640_e10156,)
            }
        };
        let assign6640_e10158: f64 = (assign6640_e10131 * assign6640_e10157);
        let assign6640_e10159: f64 = (1.0 + assign6640_e10158);
        let assign6640_e10160: f64 = (var_phig_i * assign6640_e10159);
        (assign6640_e10160, (var_phig_i_dn0 * assign6640_e10159), (var_phig_i_dn2 * assign6640_e10159), (var_phig_i_dn3 * assign6640_e10159), (var_phig_i_dn4 * assign6640_e10159), (var_phig_i_dn5 * assign6640_e10159), (var_phig_i_dn6 * assign6640_e10159), (var_phig_i_dn7 * assign6640_e10159), (var_phig_i_dn8 * assign6640_e10159), (var_phig_i_dn9 * assign6640_e10159), (var_phig_i_dn10 * assign6640_e10159), (var_phig_i_dn11 * assign6640_e10159), (var_phig_i_dn13 * assign6640_e10159), (var_phig_i_dn14 * assign6640_e10159),)
    } else {
        (var_phig_i, var_phig_i_dn0, var_phig_i_dn2, var_phig_i_dn3, var_phig_i_dn4, var_phig_i_dn5, var_phig_i_dn6, var_phig_i_dn7, var_phig_i_dn8, var_phig_i_dn9, var_phig_i_dn10, var_phig_i_dn11, var_phig_i_dn13, var_phig_i_dn14,)
    }
};
        var_phig_i = assign6640_e10162;
        var_phig_i_dn0 = assign6640_e10162_d_n0;
        var_phig_i_dn2 = assign6640_e10162_d_n2;
        var_phig_i_dn3 = assign6640_e10162_d_n3;
        var_phig_i_dn4 = assign6640_e10162_d_n4;
        var_phig_i_dn5 = assign6640_e10162_d_n5;
        var_phig_i_dn6 = assign6640_e10162_d_n6;
        var_phig_i_dn7 = assign6640_e10162_d_n7;
        var_phig_i_dn8 = assign6640_e10162_d_n8;
        var_phig_i_dn9 = assign6640_e10162_d_n9;
        var_phig_i_dn10 = assign6640_e10162_d_n10;
        var_phig_i_dn11 = assign6640_e10162_d_n11;
        var_phig_i_dn13 = assign6640_e10162_d_n13;
        var_phig_i_dn14 = assign6640_e10162_d_n14;
        var_phig_i_rv = 0.0;

        let assign6650_e10165: f64 = if p.p158 != 0.0 { 1.0 } else { 0.0 };
        var_guard55 = assign6650_e10165;
        var_guard55_rv = 0.0;

        let (assign6660_e10202, assign6660_e10202_d_n0, assign6660_e10202_d_n2, assign6660_e10202_d_n3, assign6660_e10202_d_n4, assign6660_e10202_d_n5, assign6660_e10202_d_n6, assign6660_e10202_d_n7, assign6660_e10202_d_n8, assign6660_e10202_d_n9, assign6660_e10202_d_n10, assign6660_e10202_d_n11, assign6660_e10202_d_n13, assign6660_e10202_d_n14,) = {
    if (var_guard55 != 0.0) {
        let assign6660_e10171: f64 = (p.p158 / p.p5);
        let assign6660_e10175: f64 = (p.p5 / p.p159);
        let assign6660_e10176: f64 = (1.0 + assign6660_e10175);
        let (assign6660_e10197,) = {
            if (!(assign6660_e10176 > 1e-38)) {
                let assign6660_e10181: f64 = (-87.498233534);
                (assign6660_e10181,)
            } else {
                let assign6660_e10185: f64 = (p.p5 / p.p159);
                let assign6660_e10186: f64 = (1.0 + assign6660_e10185);
                let (assign6660_e10196,) = {
                    if (assign6660_e10186 > 1e-38) {
                        let assign6660_e10192: f64 = (p.p5 / p.p159);
                        let assign6660_e10193: f64 = (1.0 + assign6660_e10192);
                        let assign6660_e10194: f64 = (assign6660_e10193).ln();
                        (assign6660_e10194,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6660_e10196,)
            }
        };
        let assign6660_e10198: f64 = (assign6660_e10171 * assign6660_e10197);
        let assign6660_e10199: f64 = (1.0 + assign6660_e10198);
        let assign6660_e10200: f64 = (var_eta0_i * assign6660_e10199);
        (assign6660_e10200, (var_eta0_i_dn0 * assign6660_e10199), (var_eta0_i_dn2 * assign6660_e10199), (var_eta0_i_dn3 * assign6660_e10199), (var_eta0_i_dn4 * assign6660_e10199), (var_eta0_i_dn5 * assign6660_e10199), (var_eta0_i_dn6 * assign6660_e10199), (var_eta0_i_dn7 * assign6660_e10199), (var_eta0_i_dn8 * assign6660_e10199), (var_eta0_i_dn9 * assign6660_e10199), (var_eta0_i_dn10 * assign6660_e10199), (var_eta0_i_dn11 * assign6660_e10199), (var_eta0_i_dn13 * assign6660_e10199), (var_eta0_i_dn14 * assign6660_e10199),)
    } else {
        (var_eta0_i, var_eta0_i_dn0, var_eta0_i_dn2, var_eta0_i_dn3, var_eta0_i_dn4, var_eta0_i_dn5, var_eta0_i_dn6, var_eta0_i_dn7, var_eta0_i_dn8, var_eta0_i_dn9, var_eta0_i_dn10, var_eta0_i_dn11, var_eta0_i_dn13, var_eta0_i_dn14,)
    }
};
        var_eta0_i = assign6660_e10202;
        var_eta0_i_dn0 = assign6660_e10202_d_n0;
        var_eta0_i_dn2 = assign6660_e10202_d_n2;
        var_eta0_i_dn3 = assign6660_e10202_d_n3;
        var_eta0_i_dn4 = assign6660_e10202_d_n4;
        var_eta0_i_dn5 = assign6660_e10202_d_n5;
        var_eta0_i_dn6 = assign6660_e10202_d_n6;
        var_eta0_i_dn7 = assign6660_e10202_d_n7;
        var_eta0_i_dn8 = assign6660_e10202_d_n8;
        var_eta0_i_dn9 = assign6660_e10202_d_n9;
        var_eta0_i_dn10 = assign6660_e10202_d_n10;
        var_eta0_i_dn11 = assign6660_e10202_d_n11;
        var_eta0_i_dn13 = assign6660_e10202_d_n13;
        var_eta0_i_dn14 = assign6660_e10202_d_n14;
        var_eta0_i_rv = 0.0;

        let assign6670_e10205: f64 = if p.p152 != 0.0 { 1.0 } else { 0.0 };
        var_guard56 = assign6670_e10205;
        var_guard56_rv = 0.0;

        let (assign6680_e10242,) = {
    if (var_guard56 != 0.0) {
        let assign6680_e10211: f64 = (p.p152 / p.p5);
        let assign6680_e10215: f64 = (p.p5 / p.p153);
        let assign6680_e10216: f64 = (1.0 + assign6680_e10215);
        let (assign6680_e10237,) = {
            if (!(assign6680_e10216 > 1e-38)) {
                let assign6680_e10221: f64 = (-87.498233534);
                (assign6680_e10221,)
            } else {
                let assign6680_e10225: f64 = (p.p5 / p.p153);
                let assign6680_e10226: f64 = (1.0 + assign6680_e10225);
                let (assign6680_e10236,) = {
                    if (assign6680_e10226 > 1e-38) {
                        let assign6680_e10232: f64 = (p.p5 / p.p153);
                        let assign6680_e10233: f64 = (1.0 + assign6680_e10232);
                        let assign6680_e10234: f64 = (assign6680_e10233).ln();
                        (assign6680_e10234,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6680_e10236,)
            }
        };
        let assign6680_e10238: f64 = (assign6680_e10211 * assign6680_e10237);
        let assign6680_e10239: f64 = (1.0 + assign6680_e10238);
        let assign6680_e10240: f64 = (var_cdsc_i * assign6680_e10239);
        (assign6680_e10240,)
    } else {
        (var_cdsc_i,)
    }
};
        var_cdsc_i = assign6680_e10242;
        var_cdsc_i_rv = 0.0;

        let assign6690_e10245: f64 = if p.p154 != 0.0 { 1.0 } else { 0.0 };
        var_guard57 = assign6690_e10245;
        var_guard57_rv = 0.0;

        let (assign6700_e10282,) = {
    if (var_guard57 != 0.0) {
        let assign6700_e10251: f64 = (p.p154 / p.p5);
        let assign6700_e10255: f64 = (p.p5 / p.p155);
        let assign6700_e10256: f64 = (1.0 + assign6700_e10255);
        let (assign6700_e10277,) = {
            if (!(assign6700_e10256 > 1e-38)) {
                let assign6700_e10261: f64 = (-87.498233534);
                (assign6700_e10261,)
            } else {
                let assign6700_e10265: f64 = (p.p5 / p.p155);
                let assign6700_e10266: f64 = (1.0 + assign6700_e10265);
                let (assign6700_e10276,) = {
                    if (assign6700_e10266 > 1e-38) {
                        let assign6700_e10272: f64 = (p.p5 / p.p155);
                        let assign6700_e10273: f64 = (1.0 + assign6700_e10272);
                        let assign6700_e10274: f64 = (assign6700_e10273).ln();
                        (assign6700_e10274,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6700_e10276,)
            }
        };
        let assign6700_e10278: f64 = (assign6700_e10251 * assign6700_e10277);
        let assign6700_e10279: f64 = (1.0 + assign6700_e10278);
        let assign6700_e10280: f64 = (var_cdscd_i * assign6700_e10279);
        (assign6700_e10280,)
    } else {
        (var_cdscd_i,)
    }
};
        var_cdscd_i = assign6700_e10282;
        var_cdscd_i_rv = 0.0;

        let assign6710_e10285: f64 = if p.p156 != 0.0 { 1.0 } else { 0.0 };
        var_guard58 = assign6710_e10285;
        var_guard58_rv = 0.0;

        let (assign6720_e10322,) = {
    if (var_guard58 != 0.0) {
        let assign6720_e10291: f64 = (p.p156 / p.p5);
        let assign6720_e10295: f64 = (p.p5 / p.p157);
        let assign6720_e10296: f64 = (1.0 + assign6720_e10295);
        let (assign6720_e10317,) = {
            if (!(assign6720_e10296 > 1e-38)) {
                let assign6720_e10301: f64 = (-87.498233534);
                (assign6720_e10301,)
            } else {
                let assign6720_e10305: f64 = (p.p5 / p.p157);
                let assign6720_e10306: f64 = (1.0 + assign6720_e10305);
                let (assign6720_e10316,) = {
                    if (assign6720_e10306 > 1e-38) {
                        let assign6720_e10312: f64 = (p.p5 / p.p157);
                        let assign6720_e10313: f64 = (1.0 + assign6720_e10312);
                        let assign6720_e10314: f64 = (assign6720_e10313).ln();
                        (assign6720_e10314,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6720_e10316,)
            }
        };
        let assign6720_e10318: f64 = (assign6720_e10291 * assign6720_e10317);
        let assign6720_e10319: f64 = (1.0 + assign6720_e10318);
        let assign6720_e10320: f64 = (var_cdscdr_i * assign6720_e10319);
        (assign6720_e10320,)
    } else {
        (var_cdscdr_i,)
    }
};
        var_cdscdr_i = assign6720_e10322;
        var_cdscdr_i_rv = 0.0;

        let assign6730_e10325: f64 = if p.p428 != 0.0 { 1.0 } else { 0.0 };
        var_guard59 = assign6730_e10325;
        var_guard59_rv = 0.0;

        let (assign6740_e10362, assign6740_e10362_d_n0, assign6740_e10362_d_n2, assign6740_e10362_d_n3, assign6740_e10362_d_n4, assign6740_e10362_d_n5, assign6740_e10362_d_n6, assign6740_e10362_d_n7, assign6740_e10362_d_n8, assign6740_e10362_d_n9, assign6740_e10362_d_n10, assign6740_e10362_d_n11, assign6740_e10362_d_n13, assign6740_e10362_d_n14,) = {
    if (var_guard59 != 0.0) {
        let assign6740_e10331: f64 = (p.p428 / p.p5);
        let assign6740_e10335: f64 = (p.p5 / p.p429);
        let assign6740_e10336: f64 = (1.0 + assign6740_e10335);
        let (assign6740_e10357,) = {
            if (!(assign6740_e10336 > 1e-38)) {
                let assign6740_e10341: f64 = (-87.498233534);
                (assign6740_e10341,)
            } else {
                let assign6740_e10345: f64 = (p.p5 / p.p429);
                let assign6740_e10346: f64 = (1.0 + assign6740_e10345);
                let (assign6740_e10356,) = {
                    if (assign6740_e10346 > 1e-38) {
                        let assign6740_e10352: f64 = (p.p5 / p.p429);
                        let assign6740_e10353: f64 = (1.0 + assign6740_e10352);
                        let assign6740_e10354: f64 = (assign6740_e10353).ln();
                        (assign6740_e10354,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6740_e10356,)
            }
        };
        let assign6740_e10358: f64 = (assign6740_e10331 * assign6740_e10357);
        let assign6740_e10359: f64 = (1.0 + assign6740_e10358);
        let assign6740_e10360: f64 = (var_vsat_i * assign6740_e10359);
        (assign6740_e10360, (var_vsat_i_dn0 * assign6740_e10359), (var_vsat_i_dn2 * assign6740_e10359), (var_vsat_i_dn3 * assign6740_e10359), (var_vsat_i_dn4 * assign6740_e10359), (var_vsat_i_dn5 * assign6740_e10359), (var_vsat_i_dn6 * assign6740_e10359), (var_vsat_i_dn7 * assign6740_e10359), (var_vsat_i_dn8 * assign6740_e10359), (var_vsat_i_dn9 * assign6740_e10359), (var_vsat_i_dn10 * assign6740_e10359), (var_vsat_i_dn11 * assign6740_e10359), (var_vsat_i_dn13 * assign6740_e10359), (var_vsat_i_dn14 * assign6740_e10359),)
    } else {
        (var_vsat_i, var_vsat_i_dn0, var_vsat_i_dn2, var_vsat_i_dn3, var_vsat_i_dn4, var_vsat_i_dn5, var_vsat_i_dn6, var_vsat_i_dn7, var_vsat_i_dn8, var_vsat_i_dn9, var_vsat_i_dn10, var_vsat_i_dn11, var_vsat_i_dn13, var_vsat_i_dn14,)
    }
};
        var_vsat_i = assign6740_e10362;
        var_vsat_i_dn0 = assign6740_e10362_d_n0;
        var_vsat_i_dn2 = assign6740_e10362_d_n2;
        var_vsat_i_dn3 = assign6740_e10362_d_n3;
        var_vsat_i_dn4 = assign6740_e10362_d_n4;
        var_vsat_i_dn5 = assign6740_e10362_d_n5;
        var_vsat_i_dn6 = assign6740_e10362_d_n6;
        var_vsat_i_dn7 = assign6740_e10362_d_n7;
        var_vsat_i_dn8 = assign6740_e10362_d_n8;
        var_vsat_i_dn9 = assign6740_e10362_d_n9;
        var_vsat_i_dn10 = assign6740_e10362_d_n10;
        var_vsat_i_dn11 = assign6740_e10362_d_n11;
        var_vsat_i_dn13 = assign6740_e10362_d_n13;
        var_vsat_i_dn14 = assign6740_e10362_d_n14;
        var_vsat_i_rv = 0.0;

        let assign6750_e10365: f64 = if p.p432 != 0.0 { 1.0 } else { 0.0 };
        var_guard60 = assign6750_e10365;
        var_guard60_rv = 0.0;

        let (assign6760_e10402, assign6760_e10402_d_n0, assign6760_e10402_d_n2, assign6760_e10402_d_n3, assign6760_e10402_d_n4, assign6760_e10402_d_n5, assign6760_e10402_d_n6, assign6760_e10402_d_n7, assign6760_e10402_d_n8, assign6760_e10402_d_n9, assign6760_e10402_d_n10, assign6760_e10402_d_n11, assign6760_e10402_d_n13, assign6760_e10402_d_n14,) = {
    if (var_guard60 != 0.0) {
        let assign6760_e10371: f64 = (p.p432 / p.p5);
        let assign6760_e10375: f64 = (p.p5 / p.p433);
        let assign6760_e10376: f64 = (1.0 + assign6760_e10375);
        let (assign6760_e10397,) = {
            if (!(assign6760_e10376 > 1e-38)) {
                let assign6760_e10381: f64 = (-87.498233534);
                (assign6760_e10381,)
            } else {
                let assign6760_e10385: f64 = (p.p5 / p.p433);
                let assign6760_e10386: f64 = (1.0 + assign6760_e10385);
                let (assign6760_e10396,) = {
                    if (assign6760_e10386 > 1e-38) {
                        let assign6760_e10392: f64 = (p.p5 / p.p433);
                        let assign6760_e10393: f64 = (1.0 + assign6760_e10392);
                        let assign6760_e10394: f64 = (assign6760_e10393).ln();
                        (assign6760_e10394,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6760_e10396,)
            }
        };
        let assign6760_e10398: f64 = (assign6760_e10371 * assign6760_e10397);
        let assign6760_e10399: f64 = (1.0 + assign6760_e10398);
        let assign6760_e10400: f64 = (var_vsat1_i * assign6760_e10399);
        (assign6760_e10400, (var_vsat1_i_dn0 * assign6760_e10399), (var_vsat1_i_dn2 * assign6760_e10399), (var_vsat1_i_dn3 * assign6760_e10399), (var_vsat1_i_dn4 * assign6760_e10399), (var_vsat1_i_dn5 * assign6760_e10399), (var_vsat1_i_dn6 * assign6760_e10399), (var_vsat1_i_dn7 * assign6760_e10399), (var_vsat1_i_dn8 * assign6760_e10399), (var_vsat1_i_dn9 * assign6760_e10399), (var_vsat1_i_dn10 * assign6760_e10399), (var_vsat1_i_dn11 * assign6760_e10399), (var_vsat1_i_dn13 * assign6760_e10399), (var_vsat1_i_dn14 * assign6760_e10399),)
    } else {
        (var_vsat1_i, var_vsat1_i_dn0, var_vsat1_i_dn2, var_vsat1_i_dn3, var_vsat1_i_dn4, var_vsat1_i_dn5, var_vsat1_i_dn6, var_vsat1_i_dn7, var_vsat1_i_dn8, var_vsat1_i_dn9, var_vsat1_i_dn10, var_vsat1_i_dn11, var_vsat1_i_dn13, var_vsat1_i_dn14,)
    }
};
        var_vsat1_i = assign6760_e10402;
        var_vsat1_i_dn0 = assign6760_e10402_d_n0;
        var_vsat1_i_dn2 = assign6760_e10402_d_n2;
        var_vsat1_i_dn3 = assign6760_e10402_d_n3;
        var_vsat1_i_dn4 = assign6760_e10402_d_n4;
        var_vsat1_i_dn5 = assign6760_e10402_d_n5;
        var_vsat1_i_dn6 = assign6760_e10402_d_n6;
        var_vsat1_i_dn7 = assign6760_e10402_d_n7;
        var_vsat1_i_dn8 = assign6760_e10402_d_n8;
        var_vsat1_i_dn9 = assign6760_e10402_d_n9;
        var_vsat1_i_dn10 = assign6760_e10402_d_n10;
        var_vsat1_i_dn11 = assign6760_e10402_d_n11;
        var_vsat1_i_dn13 = assign6760_e10402_d_n13;
        var_vsat1_i_dn14 = assign6760_e10402_d_n14;
        var_vsat1_i_rv = 0.0;

        let assign6770_e10405: f64 = if p.p434 != 0.0 { 1.0 } else { 0.0 };
        var_guard61 = assign6770_e10405;
        var_guard61_rv = 0.0;

        let (assign6780_e10442, assign6780_e10442_d_n0, assign6780_e10442_d_n2, assign6780_e10442_d_n3, assign6780_e10442_d_n4, assign6780_e10442_d_n5, assign6780_e10442_d_n6, assign6780_e10442_d_n7, assign6780_e10442_d_n8, assign6780_e10442_d_n9, assign6780_e10442_d_n10, assign6780_e10442_d_n11, assign6780_e10442_d_n13, assign6780_e10442_d_n14,) = {
    if (var_guard61 != 0.0) {
        let assign6780_e10411: f64 = (p.p434 / p.p5);
        let assign6780_e10415: f64 = (p.p5 / p.p435);
        let assign6780_e10416: f64 = (1.0 + assign6780_e10415);
        let (assign6780_e10437,) = {
            if (!(assign6780_e10416 > 1e-38)) {
                let assign6780_e10421: f64 = (-87.498233534);
                (assign6780_e10421,)
            } else {
                let assign6780_e10425: f64 = (p.p5 / p.p435);
                let assign6780_e10426: f64 = (1.0 + assign6780_e10425);
                let (assign6780_e10436,) = {
                    if (assign6780_e10426 > 1e-38) {
                        let assign6780_e10432: f64 = (p.p5 / p.p435);
                        let assign6780_e10433: f64 = (1.0 + assign6780_e10432);
                        let assign6780_e10434: f64 = (assign6780_e10433).ln();
                        (assign6780_e10434,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6780_e10436,)
            }
        };
        let assign6780_e10438: f64 = (assign6780_e10411 * assign6780_e10437);
        let assign6780_e10439: f64 = (1.0 + assign6780_e10438);
        let assign6780_e10440: f64 = (var_vsat1r_i * assign6780_e10439);
        (assign6780_e10440, (var_vsat1r_i_dn0 * assign6780_e10439), (var_vsat1r_i_dn2 * assign6780_e10439), (var_vsat1r_i_dn3 * assign6780_e10439), (var_vsat1r_i_dn4 * assign6780_e10439), (var_vsat1r_i_dn5 * assign6780_e10439), (var_vsat1r_i_dn6 * assign6780_e10439), (var_vsat1r_i_dn7 * assign6780_e10439), (var_vsat1r_i_dn8 * assign6780_e10439), (var_vsat1r_i_dn9 * assign6780_e10439), (var_vsat1r_i_dn10 * assign6780_e10439), (var_vsat1r_i_dn11 * assign6780_e10439), (var_vsat1r_i_dn13 * assign6780_e10439), (var_vsat1r_i_dn14 * assign6780_e10439),)
    } else {
        (var_vsat1r_i, var_vsat1r_i_dn0, var_vsat1r_i_dn2, var_vsat1r_i_dn3, var_vsat1r_i_dn4, var_vsat1r_i_dn5, var_vsat1r_i_dn6, var_vsat1r_i_dn7, var_vsat1r_i_dn8, var_vsat1r_i_dn9, var_vsat1r_i_dn10, var_vsat1r_i_dn11, var_vsat1r_i_dn13, var_vsat1r_i_dn14,)
    }
};
        var_vsat1r_i = assign6780_e10442;
        var_vsat1r_i_dn0 = assign6780_e10442_d_n0;
        var_vsat1r_i_dn2 = assign6780_e10442_d_n2;
        var_vsat1r_i_dn3 = assign6780_e10442_d_n3;
        var_vsat1r_i_dn4 = assign6780_e10442_d_n4;
        var_vsat1r_i_dn5 = assign6780_e10442_d_n5;
        var_vsat1r_i_dn6 = assign6780_e10442_d_n6;
        var_vsat1r_i_dn7 = assign6780_e10442_d_n7;
        var_vsat1r_i_dn8 = assign6780_e10442_d_n8;
        var_vsat1r_i_dn9 = assign6780_e10442_d_n9;
        var_vsat1r_i_dn10 = assign6780_e10442_d_n10;
        var_vsat1r_i_dn11 = assign6780_e10442_d_n11;
        var_vsat1r_i_dn13 = assign6780_e10442_d_n13;
        var_vsat1r_i_dn14 = assign6780_e10442_d_n14;
        var_vsat1r_i_rv = 0.0;

        let assign6790_e10445: f64 = if p.p581 != 0.0 { 1.0 } else { 0.0 };
        var_guard62 = assign6790_e10445;
        var_guard62_rv = 0.0;

        let (assign6800_e10482, assign6800_e10482_d_n0, assign6800_e10482_d_n2, assign6800_e10482_d_n3, assign6800_e10482_d_n4, assign6800_e10482_d_n5, assign6800_e10482_d_n6, assign6800_e10482_d_n7, assign6800_e10482_d_n8, assign6800_e10482_d_n9, assign6800_e10482_d_n10, assign6800_e10482_d_n11, assign6800_e10482_d_n13, assign6800_e10482_d_n14,) = {
    if (var_guard62 != 0.0) {
        let assign6800_e10451: f64 = (p.p581 / p.p5);
        let assign6800_e10455: f64 = (p.p5 / p.p584);
        let assign6800_e10456: f64 = (1.0 + assign6800_e10455);
        let (assign6800_e10477,) = {
            if (!(assign6800_e10456 > 1e-38)) {
                let assign6800_e10461: f64 = (-87.498233534);
                (assign6800_e10461,)
            } else {
                let assign6800_e10465: f64 = (p.p5 / p.p584);
                let assign6800_e10466: f64 = (1.0 + assign6800_e10465);
                let (assign6800_e10476,) = {
                    if (assign6800_e10466 > 1e-38) {
                        let assign6800_e10472: f64 = (p.p5 / p.p584);
                        let assign6800_e10473: f64 = (1.0 + assign6800_e10472);
                        let assign6800_e10474: f64 = (assign6800_e10473).ln();
                        (assign6800_e10474,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6800_e10476,)
            }
        };
        let assign6800_e10478: f64 = (assign6800_e10451 * assign6800_e10477);
        let assign6800_e10479: f64 = (1.0 + assign6800_e10478);
        let assign6800_e10480: f64 = (var_u0_i * assign6800_e10479);
        (assign6800_e10480, (var_u0_i_dn0 * assign6800_e10479), (var_u0_i_dn2 * assign6800_e10479), (var_u0_i_dn3 * assign6800_e10479), (var_u0_i_dn4 * assign6800_e10479), (var_u0_i_dn5 * assign6800_e10479), (var_u0_i_dn6 * assign6800_e10479), (var_u0_i_dn7 * assign6800_e10479), (var_u0_i_dn8 * assign6800_e10479), (var_u0_i_dn9 * assign6800_e10479), (var_u0_i_dn10 * assign6800_e10479), (var_u0_i_dn11 * assign6800_e10479), (var_u0_i_dn13 * assign6800_e10479), (var_u0_i_dn14 * assign6800_e10479),)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign6800_e10482;
        var_u0_i_dn0 = assign6800_e10482_d_n0;
        var_u0_i_dn2 = assign6800_e10482_d_n2;
        var_u0_i_dn3 = assign6800_e10482_d_n3;
        var_u0_i_dn4 = assign6800_e10482_d_n4;
        var_u0_i_dn5 = assign6800_e10482_d_n5;
        var_u0_i_dn6 = assign6800_e10482_d_n6;
        var_u0_i_dn7 = assign6800_e10482_d_n7;
        var_u0_i_dn8 = assign6800_e10482_d_n8;
        var_u0_i_dn9 = assign6800_e10482_d_n9;
        var_u0_i_dn10 = assign6800_e10482_d_n10;
        var_u0_i_dn11 = assign6800_e10482_d_n11;
        var_u0_i_dn13 = assign6800_e10482_d_n13;
        var_u0_i_dn14 = assign6800_e10482_d_n14;
        var_u0_i_rv = 0.0;

        let assign6810_e10485: f64 = if p.p583 != 0.0 { 1.0 } else { 0.0 };
        var_guard63 = assign6810_e10485;
        var_guard63_rv = 0.0;

        let (assign6820_e10522, assign6820_e10522_d_n0, assign6820_e10522_d_n2, assign6820_e10522_d_n3, assign6820_e10522_d_n4, assign6820_e10522_d_n5, assign6820_e10522_d_n6, assign6820_e10522_d_n7, assign6820_e10522_d_n8, assign6820_e10522_d_n9, assign6820_e10522_d_n10, assign6820_e10522_d_n11, assign6820_e10522_d_n13, assign6820_e10522_d_n14,) = {
    if (var_guard63 != 0.0) {
        let assign6820_e10491: f64 = (p.p583 / p.p5);
        let assign6820_e10495: f64 = (p.p5 / p.p586);
        let assign6820_e10496: f64 = (1.0 + assign6820_e10495);
        let (assign6820_e10517,) = {
            if (!(assign6820_e10496 > 1e-38)) {
                let assign6820_e10501: f64 = (-87.498233534);
                (assign6820_e10501,)
            } else {
                let assign6820_e10505: f64 = (p.p5 / p.p586);
                let assign6820_e10506: f64 = (1.0 + assign6820_e10505);
                let (assign6820_e10516,) = {
                    if (assign6820_e10506 > 1e-38) {
                        let assign6820_e10512: f64 = (p.p5 / p.p586);
                        let assign6820_e10513: f64 = (1.0 + assign6820_e10512);
                        let assign6820_e10514: f64 = (assign6820_e10513).ln();
                        (assign6820_e10514,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6820_e10516,)
            }
        };
        let assign6820_e10518: f64 = (assign6820_e10491 * assign6820_e10517);
        let assign6820_e10519: f64 = (1.0 + assign6820_e10518);
        let assign6820_e10520: f64 = (var_u0r_i * assign6820_e10519);
        (assign6820_e10520, (var_u0r_i_dn0 * assign6820_e10519), (var_u0r_i_dn2 * assign6820_e10519), (var_u0r_i_dn3 * assign6820_e10519), (var_u0r_i_dn4 * assign6820_e10519), (var_u0r_i_dn5 * assign6820_e10519), (var_u0r_i_dn6 * assign6820_e10519), (var_u0r_i_dn7 * assign6820_e10519), (var_u0r_i_dn8 * assign6820_e10519), (var_u0r_i_dn9 * assign6820_e10519), (var_u0r_i_dn10 * assign6820_e10519), (var_u0r_i_dn11 * assign6820_e10519), (var_u0r_i_dn13 * assign6820_e10519), (var_u0r_i_dn14 * assign6820_e10519),)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign6820_e10522;
        var_u0r_i_dn0 = assign6820_e10522_d_n0;
        var_u0r_i_dn2 = assign6820_e10522_d_n2;
        var_u0r_i_dn3 = assign6820_e10522_d_n3;
        var_u0r_i_dn4 = assign6820_e10522_d_n4;
        var_u0r_i_dn5 = assign6820_e10522_d_n5;
        var_u0r_i_dn6 = assign6820_e10522_d_n6;
        var_u0r_i_dn7 = assign6820_e10522_d_n7;
        var_u0r_i_dn8 = assign6820_e10522_d_n8;
        var_u0r_i_dn9 = assign6820_e10522_d_n9;
        var_u0r_i_dn10 = assign6820_e10522_d_n10;
        var_u0r_i_dn11 = assign6820_e10522_d_n11;
        var_u0r_i_dn13 = assign6820_e10522_d_n13;
        var_u0r_i_dn14 = assign6820_e10522_d_n14;
        var_u0r_i_rv = 0.0;

        *var_cdsc_i_slot = var_cdsc_i;
        *var_cdsc_i_rv_slot = var_cdsc_i_rv;
        *var_cdscd_i_slot = var_cdscd_i;
        *var_cdscd_i_rv_slot = var_cdscd_i_rv;
        *var_cdscdr_i_slot = var_cdscdr_i;
        *var_cdscdr_i_rv_slot = var_cdscdr_i_rv;
        *var_eta0_i_slot = var_eta0_i;
        *var_eta0_i_dn0_slot = var_eta0_i_dn0;
        *var_eta0_i_dn10_slot = var_eta0_i_dn10;
        *var_eta0_i_dn11_slot = var_eta0_i_dn11;
        *var_eta0_i_dn13_slot = var_eta0_i_dn13;
        *var_eta0_i_dn14_slot = var_eta0_i_dn14;
        *var_eta0_i_dn2_slot = var_eta0_i_dn2;
        *var_eta0_i_dn3_slot = var_eta0_i_dn3;
        *var_eta0_i_dn4_slot = var_eta0_i_dn4;
        *var_eta0_i_dn5_slot = var_eta0_i_dn5;
        *var_eta0_i_dn6_slot = var_eta0_i_dn6;
        *var_eta0_i_dn7_slot = var_eta0_i_dn7;
        *var_eta0_i_dn8_slot = var_eta0_i_dn8;
        *var_eta0_i_dn9_slot = var_eta0_i_dn9;
        *var_eta0_i_rv_slot = var_eta0_i_rv;
        *var_guard54_slot = var_guard54;
        *var_guard54_rv_slot = var_guard54_rv;
        *var_guard55_slot = var_guard55;
        *var_guard55_rv_slot = var_guard55_rv;
        *var_guard56_slot = var_guard56;
        *var_guard56_rv_slot = var_guard56_rv;
        *var_guard57_slot = var_guard57;
        *var_guard57_rv_slot = var_guard57_rv;
        *var_guard58_slot = var_guard58;
        *var_guard58_rv_slot = var_guard58_rv;
        *var_guard59_slot = var_guard59;
        *var_guard59_rv_slot = var_guard59_rv;
        *var_guard60_slot = var_guard60;
        *var_guard60_rv_slot = var_guard60_rv;
        *var_guard61_slot = var_guard61;
        *var_guard61_rv_slot = var_guard61_rv;
        *var_guard62_slot = var_guard62;
        *var_guard62_rv_slot = var_guard62_rv;
        *var_guard63_slot = var_guard63;
        *var_guard63_rv_slot = var_guard63_rv;
        *var_mfq3nom_i_slot = var_mfq3nom_i;
        *var_mfq3nom_i_rv_slot = var_mfq3nom_i_rv;
        *var_phig_i_slot = var_phig_i;
        *var_phig_i_dn0_slot = var_phig_i_dn0;
        *var_phig_i_dn10_slot = var_phig_i_dn10;
        *var_phig_i_dn11_slot = var_phig_i_dn11;
        *var_phig_i_dn13_slot = var_phig_i_dn13;
        *var_phig_i_dn14_slot = var_phig_i_dn14;
        *var_phig_i_dn2_slot = var_phig_i_dn2;
        *var_phig_i_dn3_slot = var_phig_i_dn3;
        *var_phig_i_dn4_slot = var_phig_i_dn4;
        *var_phig_i_dn5_slot = var_phig_i_dn5;
        *var_phig_i_dn6_slot = var_phig_i_dn6;
        *var_phig_i_dn7_slot = var_phig_i_dn7;
        *var_phig_i_dn8_slot = var_phig_i_dn8;
        *var_phig_i_dn9_slot = var_phig_i_dn9;
        *var_phig_i_rv_slot = var_phig_i_rv;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_dn0_slot = var_u0_i_dn0;
        *var_u0_i_dn10_slot = var_u0_i_dn10;
        *var_u0_i_dn11_slot = var_u0_i_dn11;
        *var_u0_i_dn13_slot = var_u0_i_dn13;
        *var_u0_i_dn14_slot = var_u0_i_dn14;
        *var_u0_i_dn2_slot = var_u0_i_dn2;
        *var_u0_i_dn3_slot = var_u0_i_dn3;
        *var_u0_i_dn4_slot = var_u0_i_dn4;
        *var_u0_i_dn5_slot = var_u0_i_dn5;
        *var_u0_i_dn6_slot = var_u0_i_dn6;
        *var_u0_i_dn7_slot = var_u0_i_dn7;
        *var_u0_i_dn8_slot = var_u0_i_dn8;
        *var_u0_i_dn9_slot = var_u0_i_dn9;
        *var_u0_i_rv_slot = var_u0_i_rv;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_dn0_slot = var_u0r_i_dn0;
        *var_u0r_i_dn10_slot = var_u0r_i_dn10;
        *var_u0r_i_dn11_slot = var_u0r_i_dn11;
        *var_u0r_i_dn13_slot = var_u0r_i_dn13;
        *var_u0r_i_dn14_slot = var_u0r_i_dn14;
        *var_u0r_i_dn2_slot = var_u0r_i_dn2;
        *var_u0r_i_dn3_slot = var_u0r_i_dn3;
        *var_u0r_i_dn4_slot = var_u0r_i_dn4;
        *var_u0r_i_dn5_slot = var_u0r_i_dn5;
        *var_u0r_i_dn6_slot = var_u0r_i_dn6;
        *var_u0r_i_dn7_slot = var_u0r_i_dn7;
        *var_u0r_i_dn8_slot = var_u0r_i_dn8;
        *var_u0r_i_dn9_slot = var_u0r_i_dn9;
        *var_u0r_i_rv_slot = var_u0r_i_rv;
        *var_vsat1_i_slot = var_vsat1_i;
        *var_vsat1_i_dn0_slot = var_vsat1_i_dn0;
        *var_vsat1_i_dn10_slot = var_vsat1_i_dn10;
        *var_vsat1_i_dn11_slot = var_vsat1_i_dn11;
        *var_vsat1_i_dn13_slot = var_vsat1_i_dn13;
        *var_vsat1_i_dn14_slot = var_vsat1_i_dn14;
        *var_vsat1_i_dn2_slot = var_vsat1_i_dn2;
        *var_vsat1_i_dn3_slot = var_vsat1_i_dn3;
        *var_vsat1_i_dn4_slot = var_vsat1_i_dn4;
        *var_vsat1_i_dn5_slot = var_vsat1_i_dn5;
        *var_vsat1_i_dn6_slot = var_vsat1_i_dn6;
        *var_vsat1_i_dn7_slot = var_vsat1_i_dn7;
        *var_vsat1_i_dn8_slot = var_vsat1_i_dn8;
        *var_vsat1_i_dn9_slot = var_vsat1_i_dn9;
        *var_vsat1_i_rv_slot = var_vsat1_i_rv;
        *var_vsat1r_i_slot = var_vsat1r_i;
        *var_vsat1r_i_dn0_slot = var_vsat1r_i_dn0;
        *var_vsat1r_i_dn10_slot = var_vsat1r_i_dn10;
        *var_vsat1r_i_dn11_slot = var_vsat1r_i_dn11;
        *var_vsat1r_i_dn13_slot = var_vsat1r_i_dn13;
        *var_vsat1r_i_dn14_slot = var_vsat1r_i_dn14;
        *var_vsat1r_i_dn2_slot = var_vsat1r_i_dn2;
        *var_vsat1r_i_dn3_slot = var_vsat1r_i_dn3;
        *var_vsat1r_i_dn4_slot = var_vsat1r_i_dn4;
        *var_vsat1r_i_dn5_slot = var_vsat1r_i_dn5;
        *var_vsat1r_i_dn6_slot = var_vsat1r_i_dn6;
        *var_vsat1r_i_dn7_slot = var_vsat1r_i_dn7;
        *var_vsat1r_i_dn8_slot = var_vsat1r_i_dn8;
        *var_vsat1r_i_dn9_slot = var_vsat1r_i_dn9;
        *var_vsat1r_i_rv_slot = var_vsat1r_i_rv;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsat_i_dn0_slot = var_vsat_i_dn0;
        *var_vsat_i_dn10_slot = var_vsat_i_dn10;
        *var_vsat_i_dn11_slot = var_vsat_i_dn11;
        *var_vsat_i_dn13_slot = var_vsat_i_dn13;
        *var_vsat_i_dn14_slot = var_vsat_i_dn14;
        *var_vsat_i_dn2_slot = var_vsat_i_dn2;
        *var_vsat_i_dn3_slot = var_vsat_i_dn3;
        *var_vsat_i_dn4_slot = var_vsat_i_dn4;
        *var_vsat_i_dn5_slot = var_vsat_i_dn5;
        *var_vsat_i_dn6_slot = var_vsat_i_dn6;
        *var_vsat_i_dn7_slot = var_vsat_i_dn7;
        *var_vsat_i_dn8_slot = var_vsat_i_dn8;
        *var_vsat_i_dn9_slot = var_vsat_i_dn9;
        *var_vsat_i_rv_slot = var_vsat_i_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_leff_1: f64,
        var_leff_1_dn0: f64,
        var_leff_1_dn10: f64,
        var_leff_1_dn11: f64,
        var_leff_1_dn13: f64,
        var_leff_1_dn14: f64,
        var_leff_1_dn2: f64,
        var_leff_1_dn3: f64,
        var_leff_1_dn4: f64,
        var_leff_1_dn5: f64,
        var_leff_1_dn6: f64,
        var_leff_1_dn7: f64,
        var_leff_1_dn8: f64,
        var_leff_1_dn9: f64,
        var_up_i: f64,
        var_upr_i: f64,
        var_eta0_i_slot: &mut f64,
        var_eta0_i_dn0_slot: &mut f64,
        var_eta0_i_dn10_slot: &mut f64,
        var_eta0_i_dn11_slot: &mut f64,
        var_eta0_i_dn13_slot: &mut f64,
        var_eta0_i_dn14_slot: &mut f64,
        var_eta0_i_dn2_slot: &mut f64,
        var_eta0_i_dn3_slot: &mut f64,
        var_eta0_i_dn4_slot: &mut f64,
        var_eta0_i_dn5_slot: &mut f64,
        var_eta0_i_dn6_slot: &mut f64,
        var_eta0_i_dn7_slot: &mut f64,
        var_eta0_i_dn8_slot: &mut f64,
        var_eta0_i_dn9_slot: &mut f64,
        var_eta0_i_rv_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eu_i_dn0_slot: &mut f64,
        var_eu_i_dn10_slot: &mut f64,
        var_eu_i_dn11_slot: &mut f64,
        var_eu_i_dn13_slot: &mut f64,
        var_eu_i_dn14_slot: &mut f64,
        var_eu_i_dn2_slot: &mut f64,
        var_eu_i_dn3_slot: &mut f64,
        var_eu_i_dn4_slot: &mut f64,
        var_eu_i_dn5_slot: &mut f64,
        var_eu_i_dn6_slot: &mut f64,
        var_eu_i_dn7_slot: &mut f64,
        var_eu_i_dn8_slot: &mut f64,
        var_eu_i_dn9_slot: &mut f64,
        var_eu_i_rv_slot: &mut f64,
        var_eur_i_slot: &mut f64,
        var_eur_i_dn0_slot: &mut f64,
        var_eur_i_dn10_slot: &mut f64,
        var_eur_i_dn11_slot: &mut f64,
        var_eur_i_dn13_slot: &mut f64,
        var_eur_i_dn14_slot: &mut f64,
        var_eur_i_dn2_slot: &mut f64,
        var_eur_i_dn3_slot: &mut f64,
        var_eur_i_dn4_slot: &mut f64,
        var_eur_i_dn5_slot: &mut f64,
        var_eur_i_dn6_slot: &mut f64,
        var_eur_i_dn7_slot: &mut f64,
        var_eur_i_dn8_slot: &mut f64,
        var_eur_i_dn9_slot: &mut f64,
        var_eur_i_rv_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_guard64_rv_slot: &mut f64,
        var_guard65_slot: &mut f64,
        var_guard65_rv_slot: &mut f64,
        var_guard66_slot: &mut f64,
        var_guard66_rv_slot: &mut f64,
        var_guard67_slot: &mut f64,
        var_guard67_rv_slot: &mut f64,
        var_guard68_slot: &mut f64,
        var_guard68_rv_slot: &mut f64,
        var_leff_ln_slot: &mut f64,
        var_leff_ln_dn0_slot: &mut f64,
        var_leff_ln_dn10_slot: &mut f64,
        var_leff_ln_dn11_slot: &mut f64,
        var_leff_ln_dn13_slot: &mut f64,
        var_leff_ln_dn14_slot: &mut f64,
        var_leff_ln_dn2_slot: &mut f64,
        var_leff_ln_dn3_slot: &mut f64,
        var_leff_ln_dn4_slot: &mut f64,
        var_leff_ln_dn5_slot: &mut f64,
        var_leff_ln_dn6_slot: &mut f64,
        var_leff_ln_dn7_slot: &mut f64,
        var_leff_ln_dn8_slot: &mut f64,
        var_leff_ln_dn9_slot: &mut f64,
        var_leff_ln_rv_slot: &mut f64,
        var_phig_i_slot: &mut f64,
        var_phig_i_dn0_slot: &mut f64,
        var_phig_i_dn10_slot: &mut f64,
        var_phig_i_dn11_slot: &mut f64,
        var_phig_i_dn13_slot: &mut f64,
        var_phig_i_dn14_slot: &mut f64,
        var_phig_i_dn2_slot: &mut f64,
        var_phig_i_dn3_slot: &mut f64,
        var_phig_i_dn4_slot: &mut f64,
        var_phig_i_dn5_slot: &mut f64,
        var_phig_i_dn6_slot: &mut f64,
        var_phig_i_dn7_slot: &mut f64,
        var_phig_i_dn8_slot: &mut f64,
        var_phig_i_dn9_slot: &mut f64,
        var_phig_i_rv_slot: &mut f64,
        var_pqm_i_slot: &mut f64,
        var_pqm_i_dn0_slot: &mut f64,
        var_pqm_i_dn10_slot: &mut f64,
        var_pqm_i_dn11_slot: &mut f64,
        var_pqm_i_dn13_slot: &mut f64,
        var_pqm_i_dn14_slot: &mut f64,
        var_pqm_i_dn2_slot: &mut f64,
        var_pqm_i_dn3_slot: &mut f64,
        var_pqm_i_dn4_slot: &mut f64,
        var_pqm_i_dn5_slot: &mut f64,
        var_pqm_i_dn6_slot: &mut f64,
        var_pqm_i_dn7_slot: &mut f64,
        var_pqm_i_dn8_slot: &mut f64,
        var_pqm_i_dn9_slot: &mut f64,
        var_pqm_i_rv_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdsw_i_dn0_slot: &mut f64,
        var_rdsw_i_dn10_slot: &mut f64,
        var_rdsw_i_dn11_slot: &mut f64,
        var_rdsw_i_dn13_slot: &mut f64,
        var_rdsw_i_dn14_slot: &mut f64,
        var_rdsw_i_dn2_slot: &mut f64,
        var_rdsw_i_dn3_slot: &mut f64,
        var_rdsw_i_dn4_slot: &mut f64,
        var_rdsw_i_dn5_slot: &mut f64,
        var_rdsw_i_dn6_slot: &mut f64,
        var_rdsw_i_dn7_slot: &mut f64,
        var_rdsw_i_dn8_slot: &mut f64,
        var_rdsw_i_dn9_slot: &mut f64,
        var_rdsw_i_rv_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdw_i_dn0_slot: &mut f64,
        var_rdw_i_dn10_slot: &mut f64,
        var_rdw_i_dn11_slot: &mut f64,
        var_rdw_i_dn13_slot: &mut f64,
        var_rdw_i_dn14_slot: &mut f64,
        var_rdw_i_dn2_slot: &mut f64,
        var_rdw_i_dn3_slot: &mut f64,
        var_rdw_i_dn4_slot: &mut f64,
        var_rdw_i_dn5_slot: &mut f64,
        var_rdw_i_dn6_slot: &mut f64,
        var_rdw_i_dn7_slot: &mut f64,
        var_rdw_i_dn8_slot: &mut f64,
        var_rdw_i_dn9_slot: &mut f64,
        var_rdw_i_rv_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rsw_i_dn0_slot: &mut f64,
        var_rsw_i_dn10_slot: &mut f64,
        var_rsw_i_dn11_slot: &mut f64,
        var_rsw_i_dn13_slot: &mut f64,
        var_rsw_i_dn14_slot: &mut f64,
        var_rsw_i_dn2_slot: &mut f64,
        var_rsw_i_dn3_slot: &mut f64,
        var_rsw_i_dn4_slot: &mut f64,
        var_rsw_i_dn5_slot: &mut f64,
        var_rsw_i_dn6_slot: &mut f64,
        var_rsw_i_dn7_slot: &mut f64,
        var_rsw_i_dn8_slot: &mut f64,
        var_rsw_i_dn9_slot: &mut f64,
        var_rsw_i_rv_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_dn0_slot: &mut f64,
        var_u0_i_dn10_slot: &mut f64,
        var_u0_i_dn11_slot: &mut f64,
        var_u0_i_dn13_slot: &mut f64,
        var_u0_i_dn14_slot: &mut f64,
        var_u0_i_dn2_slot: &mut f64,
        var_u0_i_dn3_slot: &mut f64,
        var_u0_i_dn4_slot: &mut f64,
        var_u0_i_dn5_slot: &mut f64,
        var_u0_i_dn6_slot: &mut f64,
        var_u0_i_dn7_slot: &mut f64,
        var_u0_i_dn8_slot: &mut f64,
        var_u0_i_dn9_slot: &mut f64,
        var_u0_i_rv_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_dn0_slot: &mut f64,
        var_u0r_i_dn10_slot: &mut f64,
        var_u0r_i_dn11_slot: &mut f64,
        var_u0r_i_dn13_slot: &mut f64,
        var_u0r_i_dn14_slot: &mut f64,
        var_u0r_i_dn2_slot: &mut f64,
        var_u0r_i_dn3_slot: &mut f64,
        var_u0r_i_dn4_slot: &mut f64,
        var_u0r_i_dn5_slot: &mut f64,
        var_u0r_i_dn6_slot: &mut f64,
        var_u0r_i_dn7_slot: &mut f64,
        var_u0r_i_dn8_slot: &mut f64,
        var_u0r_i_dn9_slot: &mut f64,
        var_u0r_i_rv_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_ua_i_dn0_slot: &mut f64,
        var_ua_i_dn10_slot: &mut f64,
        var_ua_i_dn11_slot: &mut f64,
        var_ua_i_dn13_slot: &mut f64,
        var_ua_i_dn14_slot: &mut f64,
        var_ua_i_dn2_slot: &mut f64,
        var_ua_i_dn3_slot: &mut f64,
        var_ua_i_dn4_slot: &mut f64,
        var_ua_i_dn5_slot: &mut f64,
        var_ua_i_dn6_slot: &mut f64,
        var_ua_i_dn7_slot: &mut f64,
        var_ua_i_dn8_slot: &mut f64,
        var_ua_i_dn9_slot: &mut f64,
        var_ua_i_rv_slot: &mut f64,
        var_uar_i_slot: &mut f64,
        var_uar_i_dn0_slot: &mut f64,
        var_uar_i_dn10_slot: &mut f64,
        var_uar_i_dn11_slot: &mut f64,
        var_uar_i_dn13_slot: &mut f64,
        var_uar_i_dn14_slot: &mut f64,
        var_uar_i_dn2_slot: &mut f64,
        var_uar_i_dn3_slot: &mut f64,
        var_uar_i_dn4_slot: &mut f64,
        var_uar_i_dn5_slot: &mut f64,
        var_uar_i_dn6_slot: &mut f64,
        var_uar_i_dn7_slot: &mut f64,
        var_uar_i_dn8_slot: &mut f64,
        var_uar_i_dn9_slot: &mut f64,
        var_uar_i_rv_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_ud_i_dn0_slot: &mut f64,
        var_ud_i_dn10_slot: &mut f64,
        var_ud_i_dn11_slot: &mut f64,
        var_ud_i_dn13_slot: &mut f64,
        var_ud_i_dn14_slot: &mut f64,
        var_ud_i_dn2_slot: &mut f64,
        var_ud_i_dn3_slot: &mut f64,
        var_ud_i_dn4_slot: &mut f64,
        var_ud_i_dn5_slot: &mut f64,
        var_ud_i_dn6_slot: &mut f64,
        var_ud_i_dn7_slot: &mut f64,
        var_ud_i_dn8_slot: &mut f64,
        var_ud_i_dn9_slot: &mut f64,
        var_ud_i_rv_slot: &mut f64,
        var_udr_i_slot: &mut f64,
        var_udr_i_dn0_slot: &mut f64,
        var_udr_i_dn10_slot: &mut f64,
        var_udr_i_dn11_slot: &mut f64,
        var_udr_i_dn13_slot: &mut f64,
        var_udr_i_dn14_slot: &mut f64,
        var_udr_i_dn2_slot: &mut f64,
        var_udr_i_dn3_slot: &mut f64,
        var_udr_i_dn4_slot: &mut f64,
        var_udr_i_dn5_slot: &mut f64,
        var_udr_i_dn6_slot: &mut f64,
        var_udr_i_dn7_slot: &mut f64,
        var_udr_i_dn8_slot: &mut f64,
        var_udr_i_dn9_slot: &mut f64,
        var_udr_i_rv_slot: &mut f64,
    ) {
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta0_i_dn0: f64 = *var_eta0_i_dn0_slot;
        let mut var_eta0_i_dn10: f64 = *var_eta0_i_dn10_slot;
        let mut var_eta0_i_dn11: f64 = *var_eta0_i_dn11_slot;
        let mut var_eta0_i_dn13: f64 = *var_eta0_i_dn13_slot;
        let mut var_eta0_i_dn14: f64 = *var_eta0_i_dn14_slot;
        let mut var_eta0_i_dn2: f64 = *var_eta0_i_dn2_slot;
        let mut var_eta0_i_dn3: f64 = *var_eta0_i_dn3_slot;
        let mut var_eta0_i_dn4: f64 = *var_eta0_i_dn4_slot;
        let mut var_eta0_i_dn5: f64 = *var_eta0_i_dn5_slot;
        let mut var_eta0_i_dn6: f64 = *var_eta0_i_dn6_slot;
        let mut var_eta0_i_dn7: f64 = *var_eta0_i_dn7_slot;
        let mut var_eta0_i_dn8: f64 = *var_eta0_i_dn8_slot;
        let mut var_eta0_i_dn9: f64 = *var_eta0_i_dn9_slot;
        let mut var_eta0_i_rv: f64 = *var_eta0_i_rv_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eu_i_dn0: f64 = *var_eu_i_dn0_slot;
        let mut var_eu_i_dn10: f64 = *var_eu_i_dn10_slot;
        let mut var_eu_i_dn11: f64 = *var_eu_i_dn11_slot;
        let mut var_eu_i_dn13: f64 = *var_eu_i_dn13_slot;
        let mut var_eu_i_dn14: f64 = *var_eu_i_dn14_slot;
        let mut var_eu_i_dn2: f64 = *var_eu_i_dn2_slot;
        let mut var_eu_i_dn3: f64 = *var_eu_i_dn3_slot;
        let mut var_eu_i_dn4: f64 = *var_eu_i_dn4_slot;
        let mut var_eu_i_dn5: f64 = *var_eu_i_dn5_slot;
        let mut var_eu_i_dn6: f64 = *var_eu_i_dn6_slot;
        let mut var_eu_i_dn7: f64 = *var_eu_i_dn7_slot;
        let mut var_eu_i_dn8: f64 = *var_eu_i_dn8_slot;
        let mut var_eu_i_dn9: f64 = *var_eu_i_dn9_slot;
        let mut var_eu_i_rv: f64 = *var_eu_i_rv_slot;
        let mut var_eur_i: f64 = *var_eur_i_slot;
        let mut var_eur_i_dn0: f64 = *var_eur_i_dn0_slot;
        let mut var_eur_i_dn10: f64 = *var_eur_i_dn10_slot;
        let mut var_eur_i_dn11: f64 = *var_eur_i_dn11_slot;
        let mut var_eur_i_dn13: f64 = *var_eur_i_dn13_slot;
        let mut var_eur_i_dn14: f64 = *var_eur_i_dn14_slot;
        let mut var_eur_i_dn2: f64 = *var_eur_i_dn2_slot;
        let mut var_eur_i_dn3: f64 = *var_eur_i_dn3_slot;
        let mut var_eur_i_dn4: f64 = *var_eur_i_dn4_slot;
        let mut var_eur_i_dn5: f64 = *var_eur_i_dn5_slot;
        let mut var_eur_i_dn6: f64 = *var_eur_i_dn6_slot;
        let mut var_eur_i_dn7: f64 = *var_eur_i_dn7_slot;
        let mut var_eur_i_dn8: f64 = *var_eur_i_dn8_slot;
        let mut var_eur_i_dn9: f64 = *var_eur_i_dn9_slot;
        let mut var_eur_i_rv: f64 = *var_eur_i_rv_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_guard64_rv: f64 = *var_guard64_rv_slot;
        let mut var_guard65: f64 = *var_guard65_slot;
        let mut var_guard65_rv: f64 = *var_guard65_rv_slot;
        let mut var_guard66: f64 = *var_guard66_slot;
        let mut var_guard66_rv: f64 = *var_guard66_rv_slot;
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_guard67_rv: f64 = *var_guard67_rv_slot;
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard68_rv: f64 = *var_guard68_rv_slot;
        let mut var_leff_ln: f64 = *var_leff_ln_slot;
        let mut var_leff_ln_dn0: f64 = *var_leff_ln_dn0_slot;
        let mut var_leff_ln_dn10: f64 = *var_leff_ln_dn10_slot;
        let mut var_leff_ln_dn11: f64 = *var_leff_ln_dn11_slot;
        let mut var_leff_ln_dn13: f64 = *var_leff_ln_dn13_slot;
        let mut var_leff_ln_dn14: f64 = *var_leff_ln_dn14_slot;
        let mut var_leff_ln_dn2: f64 = *var_leff_ln_dn2_slot;
        let mut var_leff_ln_dn3: f64 = *var_leff_ln_dn3_slot;
        let mut var_leff_ln_dn4: f64 = *var_leff_ln_dn4_slot;
        let mut var_leff_ln_dn5: f64 = *var_leff_ln_dn5_slot;
        let mut var_leff_ln_dn6: f64 = *var_leff_ln_dn6_slot;
        let mut var_leff_ln_dn7: f64 = *var_leff_ln_dn7_slot;
        let mut var_leff_ln_dn8: f64 = *var_leff_ln_dn8_slot;
        let mut var_leff_ln_dn9: f64 = *var_leff_ln_dn9_slot;
        let mut var_leff_ln_rv: f64 = *var_leff_ln_rv_slot;
        let mut var_phig_i: f64 = *var_phig_i_slot;
        let mut var_phig_i_dn0: f64 = *var_phig_i_dn0_slot;
        let mut var_phig_i_dn10: f64 = *var_phig_i_dn10_slot;
        let mut var_phig_i_dn11: f64 = *var_phig_i_dn11_slot;
        let mut var_phig_i_dn13: f64 = *var_phig_i_dn13_slot;
        let mut var_phig_i_dn14: f64 = *var_phig_i_dn14_slot;
        let mut var_phig_i_dn2: f64 = *var_phig_i_dn2_slot;
        let mut var_phig_i_dn3: f64 = *var_phig_i_dn3_slot;
        let mut var_phig_i_dn4: f64 = *var_phig_i_dn4_slot;
        let mut var_phig_i_dn5: f64 = *var_phig_i_dn5_slot;
        let mut var_phig_i_dn6: f64 = *var_phig_i_dn6_slot;
        let mut var_phig_i_dn7: f64 = *var_phig_i_dn7_slot;
        let mut var_phig_i_dn8: f64 = *var_phig_i_dn8_slot;
        let mut var_phig_i_dn9: f64 = *var_phig_i_dn9_slot;
        let mut var_phig_i_rv: f64 = *var_phig_i_rv_slot;
        let mut var_pqm_i: f64 = *var_pqm_i_slot;
        let mut var_pqm_i_dn0: f64 = *var_pqm_i_dn0_slot;
        let mut var_pqm_i_dn10: f64 = *var_pqm_i_dn10_slot;
        let mut var_pqm_i_dn11: f64 = *var_pqm_i_dn11_slot;
        let mut var_pqm_i_dn13: f64 = *var_pqm_i_dn13_slot;
        let mut var_pqm_i_dn14: f64 = *var_pqm_i_dn14_slot;
        let mut var_pqm_i_dn2: f64 = *var_pqm_i_dn2_slot;
        let mut var_pqm_i_dn3: f64 = *var_pqm_i_dn3_slot;
        let mut var_pqm_i_dn4: f64 = *var_pqm_i_dn4_slot;
        let mut var_pqm_i_dn5: f64 = *var_pqm_i_dn5_slot;
        let mut var_pqm_i_dn6: f64 = *var_pqm_i_dn6_slot;
        let mut var_pqm_i_dn7: f64 = *var_pqm_i_dn7_slot;
        let mut var_pqm_i_dn8: f64 = *var_pqm_i_dn8_slot;
        let mut var_pqm_i_dn9: f64 = *var_pqm_i_dn9_slot;
        let mut var_pqm_i_rv: f64 = *var_pqm_i_rv_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdsw_i_dn0: f64 = *var_rdsw_i_dn0_slot;
        let mut var_rdsw_i_dn10: f64 = *var_rdsw_i_dn10_slot;
        let mut var_rdsw_i_dn11: f64 = *var_rdsw_i_dn11_slot;
        let mut var_rdsw_i_dn13: f64 = *var_rdsw_i_dn13_slot;
        let mut var_rdsw_i_dn14: f64 = *var_rdsw_i_dn14_slot;
        let mut var_rdsw_i_dn2: f64 = *var_rdsw_i_dn2_slot;
        let mut var_rdsw_i_dn3: f64 = *var_rdsw_i_dn3_slot;
        let mut var_rdsw_i_dn4: f64 = *var_rdsw_i_dn4_slot;
        let mut var_rdsw_i_dn5: f64 = *var_rdsw_i_dn5_slot;
        let mut var_rdsw_i_dn6: f64 = *var_rdsw_i_dn6_slot;
        let mut var_rdsw_i_dn7: f64 = *var_rdsw_i_dn7_slot;
        let mut var_rdsw_i_dn8: f64 = *var_rdsw_i_dn8_slot;
        let mut var_rdsw_i_dn9: f64 = *var_rdsw_i_dn9_slot;
        let mut var_rdsw_i_rv: f64 = *var_rdsw_i_rv_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdw_i_dn0: f64 = *var_rdw_i_dn0_slot;
        let mut var_rdw_i_dn10: f64 = *var_rdw_i_dn10_slot;
        let mut var_rdw_i_dn11: f64 = *var_rdw_i_dn11_slot;
        let mut var_rdw_i_dn13: f64 = *var_rdw_i_dn13_slot;
        let mut var_rdw_i_dn14: f64 = *var_rdw_i_dn14_slot;
        let mut var_rdw_i_dn2: f64 = *var_rdw_i_dn2_slot;
        let mut var_rdw_i_dn3: f64 = *var_rdw_i_dn3_slot;
        let mut var_rdw_i_dn4: f64 = *var_rdw_i_dn4_slot;
        let mut var_rdw_i_dn5: f64 = *var_rdw_i_dn5_slot;
        let mut var_rdw_i_dn6: f64 = *var_rdw_i_dn6_slot;
        let mut var_rdw_i_dn7: f64 = *var_rdw_i_dn7_slot;
        let mut var_rdw_i_dn8: f64 = *var_rdw_i_dn8_slot;
        let mut var_rdw_i_dn9: f64 = *var_rdw_i_dn9_slot;
        let mut var_rdw_i_rv: f64 = *var_rdw_i_rv_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rsw_i_dn0: f64 = *var_rsw_i_dn0_slot;
        let mut var_rsw_i_dn10: f64 = *var_rsw_i_dn10_slot;
        let mut var_rsw_i_dn11: f64 = *var_rsw_i_dn11_slot;
        let mut var_rsw_i_dn13: f64 = *var_rsw_i_dn13_slot;
        let mut var_rsw_i_dn14: f64 = *var_rsw_i_dn14_slot;
        let mut var_rsw_i_dn2: f64 = *var_rsw_i_dn2_slot;
        let mut var_rsw_i_dn3: f64 = *var_rsw_i_dn3_slot;
        let mut var_rsw_i_dn4: f64 = *var_rsw_i_dn4_slot;
        let mut var_rsw_i_dn5: f64 = *var_rsw_i_dn5_slot;
        let mut var_rsw_i_dn6: f64 = *var_rsw_i_dn6_slot;
        let mut var_rsw_i_dn7: f64 = *var_rsw_i_dn7_slot;
        let mut var_rsw_i_dn8: f64 = *var_rsw_i_dn8_slot;
        let mut var_rsw_i_dn9: f64 = *var_rsw_i_dn9_slot;
        let mut var_rsw_i_rv: f64 = *var_rsw_i_rv_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_dn0: f64 = *var_u0_i_dn0_slot;
        let mut var_u0_i_dn10: f64 = *var_u0_i_dn10_slot;
        let mut var_u0_i_dn11: f64 = *var_u0_i_dn11_slot;
        let mut var_u0_i_dn13: f64 = *var_u0_i_dn13_slot;
        let mut var_u0_i_dn14: f64 = *var_u0_i_dn14_slot;
        let mut var_u0_i_dn2: f64 = *var_u0_i_dn2_slot;
        let mut var_u0_i_dn3: f64 = *var_u0_i_dn3_slot;
        let mut var_u0_i_dn4: f64 = *var_u0_i_dn4_slot;
        let mut var_u0_i_dn5: f64 = *var_u0_i_dn5_slot;
        let mut var_u0_i_dn6: f64 = *var_u0_i_dn6_slot;
        let mut var_u0_i_dn7: f64 = *var_u0_i_dn7_slot;
        let mut var_u0_i_dn8: f64 = *var_u0_i_dn8_slot;
        let mut var_u0_i_dn9: f64 = *var_u0_i_dn9_slot;
        let mut var_u0_i_rv: f64 = *var_u0_i_rv_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_dn0: f64 = *var_u0r_i_dn0_slot;
        let mut var_u0r_i_dn10: f64 = *var_u0r_i_dn10_slot;
        let mut var_u0r_i_dn11: f64 = *var_u0r_i_dn11_slot;
        let mut var_u0r_i_dn13: f64 = *var_u0r_i_dn13_slot;
        let mut var_u0r_i_dn14: f64 = *var_u0r_i_dn14_slot;
        let mut var_u0r_i_dn2: f64 = *var_u0r_i_dn2_slot;
        let mut var_u0r_i_dn3: f64 = *var_u0r_i_dn3_slot;
        let mut var_u0r_i_dn4: f64 = *var_u0r_i_dn4_slot;
        let mut var_u0r_i_dn5: f64 = *var_u0r_i_dn5_slot;
        let mut var_u0r_i_dn6: f64 = *var_u0r_i_dn6_slot;
        let mut var_u0r_i_dn7: f64 = *var_u0r_i_dn7_slot;
        let mut var_u0r_i_dn8: f64 = *var_u0r_i_dn8_slot;
        let mut var_u0r_i_dn9: f64 = *var_u0r_i_dn9_slot;
        let mut var_u0r_i_rv: f64 = *var_u0r_i_rv_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_ua_i_dn0: f64 = *var_ua_i_dn0_slot;
        let mut var_ua_i_dn10: f64 = *var_ua_i_dn10_slot;
        let mut var_ua_i_dn11: f64 = *var_ua_i_dn11_slot;
        let mut var_ua_i_dn13: f64 = *var_ua_i_dn13_slot;
        let mut var_ua_i_dn14: f64 = *var_ua_i_dn14_slot;
        let mut var_ua_i_dn2: f64 = *var_ua_i_dn2_slot;
        let mut var_ua_i_dn3: f64 = *var_ua_i_dn3_slot;
        let mut var_ua_i_dn4: f64 = *var_ua_i_dn4_slot;
        let mut var_ua_i_dn5: f64 = *var_ua_i_dn5_slot;
        let mut var_ua_i_dn6: f64 = *var_ua_i_dn6_slot;
        let mut var_ua_i_dn7: f64 = *var_ua_i_dn7_slot;
        let mut var_ua_i_dn8: f64 = *var_ua_i_dn8_slot;
        let mut var_ua_i_dn9: f64 = *var_ua_i_dn9_slot;
        let mut var_ua_i_rv: f64 = *var_ua_i_rv_slot;
        let mut var_uar_i: f64 = *var_uar_i_slot;
        let mut var_uar_i_dn0: f64 = *var_uar_i_dn0_slot;
        let mut var_uar_i_dn10: f64 = *var_uar_i_dn10_slot;
        let mut var_uar_i_dn11: f64 = *var_uar_i_dn11_slot;
        let mut var_uar_i_dn13: f64 = *var_uar_i_dn13_slot;
        let mut var_uar_i_dn14: f64 = *var_uar_i_dn14_slot;
        let mut var_uar_i_dn2: f64 = *var_uar_i_dn2_slot;
        let mut var_uar_i_dn3: f64 = *var_uar_i_dn3_slot;
        let mut var_uar_i_dn4: f64 = *var_uar_i_dn4_slot;
        let mut var_uar_i_dn5: f64 = *var_uar_i_dn5_slot;
        let mut var_uar_i_dn6: f64 = *var_uar_i_dn6_slot;
        let mut var_uar_i_dn7: f64 = *var_uar_i_dn7_slot;
        let mut var_uar_i_dn8: f64 = *var_uar_i_dn8_slot;
        let mut var_uar_i_dn9: f64 = *var_uar_i_dn9_slot;
        let mut var_uar_i_rv: f64 = *var_uar_i_rv_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_ud_i_dn0: f64 = *var_ud_i_dn0_slot;
        let mut var_ud_i_dn10: f64 = *var_ud_i_dn10_slot;
        let mut var_ud_i_dn11: f64 = *var_ud_i_dn11_slot;
        let mut var_ud_i_dn13: f64 = *var_ud_i_dn13_slot;
        let mut var_ud_i_dn14: f64 = *var_ud_i_dn14_slot;
        let mut var_ud_i_dn2: f64 = *var_ud_i_dn2_slot;
        let mut var_ud_i_dn3: f64 = *var_ud_i_dn3_slot;
        let mut var_ud_i_dn4: f64 = *var_ud_i_dn4_slot;
        let mut var_ud_i_dn5: f64 = *var_ud_i_dn5_slot;
        let mut var_ud_i_dn6: f64 = *var_ud_i_dn6_slot;
        let mut var_ud_i_dn7: f64 = *var_ud_i_dn7_slot;
        let mut var_ud_i_dn8: f64 = *var_ud_i_dn8_slot;
        let mut var_ud_i_dn9: f64 = *var_ud_i_dn9_slot;
        let mut var_ud_i_rv: f64 = *var_ud_i_rv_slot;
        let mut var_udr_i: f64 = *var_udr_i_slot;
        let mut var_udr_i_dn0: f64 = *var_udr_i_dn0_slot;
        let mut var_udr_i_dn10: f64 = *var_udr_i_dn10_slot;
        let mut var_udr_i_dn11: f64 = *var_udr_i_dn11_slot;
        let mut var_udr_i_dn13: f64 = *var_udr_i_dn13_slot;
        let mut var_udr_i_dn14: f64 = *var_udr_i_dn14_slot;
        let mut var_udr_i_dn2: f64 = *var_udr_i_dn2_slot;
        let mut var_udr_i_dn3: f64 = *var_udr_i_dn3_slot;
        let mut var_udr_i_dn4: f64 = *var_udr_i_dn4_slot;
        let mut var_udr_i_dn5: f64 = *var_udr_i_dn5_slot;
        let mut var_udr_i_dn6: f64 = *var_udr_i_dn6_slot;
        let mut var_udr_i_dn7: f64 = *var_udr_i_dn7_slot;
        let mut var_udr_i_dn8: f64 = *var_udr_i_dn8_slot;
        let mut var_udr_i_dn9: f64 = *var_udr_i_dn9_slot;
        let mut var_udr_i_rv: f64 = *var_udr_i_rv_slot;

        let assign6830_e10525: f64 = if p.p21 != 0.0 { 1.0 } else { 0.0 };
        var_guard64 = assign6830_e10525;
        var_guard64_rv = 0.0;

        let (assign6840_e10539, assign6840_e10539_d_n0, assign6840_e10539_d_n2, assign6840_e10539_d_n3, assign6840_e10539_d_n4, assign6840_e10539_d_n5, assign6840_e10539_d_n6, assign6840_e10539_d_n7, assign6840_e10539_d_n8, assign6840_e10539_d_n9, assign6840_e10539_d_n10, assign6840_e10539_d_n11, assign6840_e10539_d_n13, assign6840_e10539_d_n14,) = {
    if (var_guard64 != 0.0) {
        let assign6840_e10531: f64 = (p.p5 - p.p21);
        let assign6840_e10533: f64 = (assign6840_e10531 * p.p99);
        let assign6840_e10535: f64 = (assign6840_e10533 * var_leff_1);
        let assign6840_e10536: f64 = (1.0 + assign6840_e10535);
        let assign6840_e10537: f64 = (var_phig_i * assign6840_e10536);
        (assign6840_e10537, ((var_phig_i_dn0 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn0))), ((var_phig_i_dn2 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn2))), ((var_phig_i_dn3 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn3))), ((var_phig_i_dn4 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn4))), ((var_phig_i_dn5 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn5))), ((var_phig_i_dn6 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn6))), ((var_phig_i_dn7 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn7))), ((var_phig_i_dn8 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn8))), ((var_phig_i_dn9 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn9))), ((var_phig_i_dn10 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn10))), ((var_phig_i_dn11 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn11))), ((var_phig_i_dn13 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn13))), ((var_phig_i_dn14 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn14))),)
    } else {
        (var_phig_i, var_phig_i_dn0, var_phig_i_dn2, var_phig_i_dn3, var_phig_i_dn4, var_phig_i_dn5, var_phig_i_dn6, var_phig_i_dn7, var_phig_i_dn8, var_phig_i_dn9, var_phig_i_dn10, var_phig_i_dn11, var_phig_i_dn13, var_phig_i_dn14,)
    }
};
        var_phig_i = assign6840_e10539;
        var_phig_i_dn0 = assign6840_e10539_d_n0;
        var_phig_i_dn2 = assign6840_e10539_d_n2;
        var_phig_i_dn3 = assign6840_e10539_d_n3;
        var_phig_i_dn4 = assign6840_e10539_d_n4;
        var_phig_i_dn5 = assign6840_e10539_d_n5;
        var_phig_i_dn6 = assign6840_e10539_d_n6;
        var_phig_i_dn7 = assign6840_e10539_d_n7;
        var_phig_i_dn8 = assign6840_e10539_d_n8;
        var_phig_i_dn9 = assign6840_e10539_d_n9;
        var_phig_i_dn10 = assign6840_e10539_d_n10;
        var_phig_i_dn11 = assign6840_e10539_d_n11;
        var_phig_i_dn13 = assign6840_e10539_d_n13;
        var_phig_i_dn14 = assign6840_e10539_d_n14;
        var_phig_i_rv = 0.0;

        let (assign6850_e10553, assign6850_e10553_d_n0, assign6850_e10553_d_n2, assign6850_e10553_d_n3, assign6850_e10553_d_n4, assign6850_e10553_d_n5, assign6850_e10553_d_n6, assign6850_e10553_d_n7, assign6850_e10553_d_n8, assign6850_e10553_d_n9, assign6850_e10553_d_n10, assign6850_e10553_d_n11, assign6850_e10553_d_n13, assign6850_e10553_d_n14,) = {
    if (var_guard64 != 0.0) {
        let assign6850_e10545: f64 = (p.p5 - p.p21);
        let assign6850_e10547: f64 = (assign6850_e10545 * p.p160);
        let assign6850_e10549: f64 = (assign6850_e10547 * var_leff_1);
        let assign6850_e10550: f64 = (1.0 + assign6850_e10549);
        let assign6850_e10551: f64 = (var_eta0_i * assign6850_e10550);
        (assign6850_e10551, ((var_eta0_i_dn0 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn0))), ((var_eta0_i_dn2 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn2))), ((var_eta0_i_dn3 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn3))), ((var_eta0_i_dn4 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn4))), ((var_eta0_i_dn5 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn5))), ((var_eta0_i_dn6 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn6))), ((var_eta0_i_dn7 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn7))), ((var_eta0_i_dn8 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn8))), ((var_eta0_i_dn9 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn9))), ((var_eta0_i_dn10 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn10))), ((var_eta0_i_dn11 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn11))), ((var_eta0_i_dn13 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn13))), ((var_eta0_i_dn14 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn14))),)
    } else {
        (var_eta0_i, var_eta0_i_dn0, var_eta0_i_dn2, var_eta0_i_dn3, var_eta0_i_dn4, var_eta0_i_dn5, var_eta0_i_dn6, var_eta0_i_dn7, var_eta0_i_dn8, var_eta0_i_dn9, var_eta0_i_dn10, var_eta0_i_dn11, var_eta0_i_dn13, var_eta0_i_dn14,)
    }
};
        var_eta0_i = assign6850_e10553;
        var_eta0_i_dn0 = assign6850_e10553_d_n0;
        var_eta0_i_dn2 = assign6850_e10553_d_n2;
        var_eta0_i_dn3 = assign6850_e10553_d_n3;
        var_eta0_i_dn4 = assign6850_e10553_d_n4;
        var_eta0_i_dn5 = assign6850_e10553_d_n5;
        var_eta0_i_dn6 = assign6850_e10553_d_n6;
        var_eta0_i_dn7 = assign6850_e10553_d_n7;
        var_eta0_i_dn8 = assign6850_e10553_d_n8;
        var_eta0_i_dn9 = assign6850_e10553_d_n9;
        var_eta0_i_dn10 = assign6850_e10553_d_n10;
        var_eta0_i_dn11 = assign6850_e10553_d_n11;
        var_eta0_i_dn13 = assign6850_e10553_d_n13;
        var_eta0_i_dn14 = assign6850_e10553_d_n14;
        var_eta0_i_rv = 0.0;

        let (assign6860_e10567, assign6860_e10567_d_n0, assign6860_e10567_d_n2, assign6860_e10567_d_n3, assign6860_e10567_d_n4, assign6860_e10567_d_n5, assign6860_e10567_d_n6, assign6860_e10567_d_n7, assign6860_e10567_d_n8, assign6860_e10567_d_n9, assign6860_e10567_d_n10, assign6860_e10567_d_n11, assign6860_e10567_d_n13, assign6860_e10567_d_n14,) = {
    if (var_guard64 != 0.0) {
        let assign6860_e10559: f64 = (p.p5 - p.p21);
        let assign6860_e10561: f64 = (assign6860_e10559 * p.p587);
        let assign6860_e10563: f64 = (assign6860_e10561 * var_leff_1);
        let assign6860_e10564: f64 = (1.0 + assign6860_e10563);
        let assign6860_e10565: f64 = (var_u0_i * assign6860_e10564);
        (assign6860_e10565, ((var_u0_i_dn0 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn0))), ((var_u0_i_dn2 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn2))), ((var_u0_i_dn3 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn3))), ((var_u0_i_dn4 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn4))), ((var_u0_i_dn5 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn5))), ((var_u0_i_dn6 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn6))), ((var_u0_i_dn7 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn7))), ((var_u0_i_dn8 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn8))), ((var_u0_i_dn9 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn9))), ((var_u0_i_dn10 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn10))), ((var_u0_i_dn11 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn11))), ((var_u0_i_dn13 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn13))), ((var_u0_i_dn14 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn14))),)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign6860_e10567;
        var_u0_i_dn0 = assign6860_e10567_d_n0;
        var_u0_i_dn2 = assign6860_e10567_d_n2;
        var_u0_i_dn3 = assign6860_e10567_d_n3;
        var_u0_i_dn4 = assign6860_e10567_d_n4;
        var_u0_i_dn5 = assign6860_e10567_d_n5;
        var_u0_i_dn6 = assign6860_e10567_d_n6;
        var_u0_i_dn7 = assign6860_e10567_d_n7;
        var_u0_i_dn8 = assign6860_e10567_d_n8;
        var_u0_i_dn9 = assign6860_e10567_d_n9;
        var_u0_i_dn10 = assign6860_e10567_d_n10;
        var_u0_i_dn11 = assign6860_e10567_d_n11;
        var_u0_i_dn13 = assign6860_e10567_d_n13;
        var_u0_i_dn14 = assign6860_e10567_d_n14;
        var_u0_i_rv = 0.0;

        let assign6870_e10569: f64 = (var_leff_1).ln();
        var_leff_ln = assign6870_e10569;
        var_leff_ln_dn0 = (var_leff_1_dn0 / var_leff_1);
        var_leff_ln_dn2 = (var_leff_1_dn2 / var_leff_1);
        var_leff_ln_dn3 = (var_leff_1_dn3 / var_leff_1);
        var_leff_ln_dn4 = (var_leff_1_dn4 / var_leff_1);
        var_leff_ln_dn5 = (var_leff_1_dn5 / var_leff_1);
        var_leff_ln_dn6 = (var_leff_1_dn6 / var_leff_1);
        var_leff_ln_dn7 = (var_leff_1_dn7 / var_leff_1);
        var_leff_ln_dn8 = (var_leff_1_dn8 / var_leff_1);
        var_leff_ln_dn9 = (var_leff_1_dn9 / var_leff_1);
        var_leff_ln_dn10 = (var_leff_1_dn10 / var_leff_1);
        var_leff_ln_dn11 = (var_leff_1_dn11 / var_leff_1);
        var_leff_ln_dn13 = (var_leff_1_dn13 / var_leff_1);
        var_leff_ln_dn14 = (var_leff_1_dn14 / var_leff_1);
        var_leff_ln_rv = 0.0;

        let assign6880_e10573: f64 = (p.p98 * var_leff_1);
        let assign6880_e10574: f64 = (var_phig_i + assign6880_e10573);
        var_phig_i = assign6880_e10574;
        var_phig_i_dn0 = (var_phig_i_dn0 + (p.p98 * var_leff_1_dn0));
        var_phig_i_dn2 = (var_phig_i_dn2 + (p.p98 * var_leff_1_dn2));
        var_phig_i_dn3 = (var_phig_i_dn3 + (p.p98 * var_leff_1_dn3));
        var_phig_i_dn4 = (var_phig_i_dn4 + (p.p98 * var_leff_1_dn4));
        var_phig_i_dn5 = (var_phig_i_dn5 + (p.p98 * var_leff_1_dn5));
        var_phig_i_dn6 = (var_phig_i_dn6 + (p.p98 * var_leff_1_dn6));
        var_phig_i_dn7 = (var_phig_i_dn7 + (p.p98 * var_leff_1_dn7));
        var_phig_i_dn8 = (var_phig_i_dn8 + (p.p98 * var_leff_1_dn8));
        var_phig_i_dn9 = (var_phig_i_dn9 + (p.p98 * var_leff_1_dn9));
        var_phig_i_dn10 = (var_phig_i_dn10 + (p.p98 * var_leff_1_dn10));
        var_phig_i_dn11 = (var_phig_i_dn11 + (p.p98 * var_leff_1_dn11));
        var_phig_i_dn13 = (var_phig_i_dn13 + (p.p98 * var_leff_1_dn13));
        var_phig_i_dn14 = (var_phig_i_dn14 + (p.p98 * var_leff_1_dn14));
        var_phig_i_rv = 0.0;

        let assign6890_e10578: f64 = (p.p427 * var_leff_1);
        let assign6890_e10579: f64 = (var_pqm_i + assign6890_e10578);
        var_pqm_i = assign6890_e10579;
        var_pqm_i_dn0 = (var_pqm_i_dn0 + (p.p427 * var_leff_1_dn0));
        var_pqm_i_dn2 = (var_pqm_i_dn2 + (p.p427 * var_leff_1_dn2));
        var_pqm_i_dn3 = (var_pqm_i_dn3 + (p.p427 * var_leff_1_dn3));
        var_pqm_i_dn4 = (var_pqm_i_dn4 + (p.p427 * var_leff_1_dn4));
        var_pqm_i_dn5 = (var_pqm_i_dn5 + (p.p427 * var_leff_1_dn5));
        var_pqm_i_dn6 = (var_pqm_i_dn6 + (p.p427 * var_leff_1_dn6));
        var_pqm_i_dn7 = (var_pqm_i_dn7 + (p.p427 * var_leff_1_dn7));
        var_pqm_i_dn8 = (var_pqm_i_dn8 + (p.p427 * var_leff_1_dn8));
        var_pqm_i_dn9 = (var_pqm_i_dn9 + (p.p427 * var_leff_1_dn9));
        var_pqm_i_dn10 = (var_pqm_i_dn10 + (p.p427 * var_leff_1_dn10));
        var_pqm_i_dn11 = (var_pqm_i_dn11 + (p.p427 * var_leff_1_dn11));
        var_pqm_i_dn13 = (var_pqm_i_dn13 + (p.p427 * var_leff_1_dn13));
        var_pqm_i_dn14 = (var_pqm_i_dn14 + (p.p427 * var_leff_1_dn14));
        var_pqm_i_rv = 0.0;

        let assign6900_e10582: f64 = if p.p589 > 0.0 { 1.0 } else { 0.0 };
        var_guard65 = assign6900_e10582;
        var_guard65_rv = 0.0;

        let (assign6910_e10596, assign6910_e10596_d_n0, assign6910_e10596_d_n2, assign6910_e10596_d_n3, assign6910_e10596_d_n4, assign6910_e10596_d_n5, assign6910_e10596_d_n6, assign6910_e10596_d_n7, assign6910_e10596_d_n8, assign6910_e10596_d_n9, assign6910_e10596_d_n10, assign6910_e10596_d_n11, assign6910_e10596_d_n13, assign6910_e10596_d_n14,) = {
    if (var_guard65 != 0.0) {
        let assign6910_e10588: f64 = (-p.p589);
        let assign6910_e10590: f64 = (assign6910_e10588 * var_leff_ln);
        let assign6910_e10591: f64 = (assign6910_e10590).exp();
        let assign6910_e10592: f64 = (var_up_i * assign6910_e10591);
        let assign6910_e10593: f64 = (1.0 - assign6910_e10592);
        let assign6910_e10594: f64 = (var_u0_i * assign6910_e10593);
        (assign6910_e10594, ((var_u0_i_dn0 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn0)))))), ((var_u0_i_dn2 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn2)))))), ((var_u0_i_dn3 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn3)))))), ((var_u0_i_dn4 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn4)))))), ((var_u0_i_dn5 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn5)))))), ((var_u0_i_dn6 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn6)))))), ((var_u0_i_dn7 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn7)))))), ((var_u0_i_dn8 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn8)))))), ((var_u0_i_dn9 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn9)))))), ((var_u0_i_dn10 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn10)))))), ((var_u0_i_dn11 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn11)))))), ((var_u0_i_dn13 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn13)))))), ((var_u0_i_dn14 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn14)))))),)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign6910_e10596;
        var_u0_i_dn0 = assign6910_e10596_d_n0;
        var_u0_i_dn2 = assign6910_e10596_d_n2;
        var_u0_i_dn3 = assign6910_e10596_d_n3;
        var_u0_i_dn4 = assign6910_e10596_d_n4;
        var_u0_i_dn5 = assign6910_e10596_d_n5;
        var_u0_i_dn6 = assign6910_e10596_d_n6;
        var_u0_i_dn7 = assign6910_e10596_d_n7;
        var_u0_i_dn8 = assign6910_e10596_d_n8;
        var_u0_i_dn9 = assign6910_e10596_d_n9;
        var_u0_i_dn10 = assign6910_e10596_d_n10;
        var_u0_i_dn11 = assign6910_e10596_d_n11;
        var_u0_i_dn13 = assign6910_e10596_d_n13;
        var_u0_i_dn14 = assign6910_e10596_d_n14;
        var_u0_i_rv = 0.0;

        let (assign6920_e10605, assign6920_e10605_d_n0, assign6920_e10605_d_n2, assign6920_e10605_d_n3, assign6920_e10605_d_n4, assign6920_e10605_d_n5, assign6920_e10605_d_n6, assign6920_e10605_d_n7, assign6920_e10605_d_n8, assign6920_e10605_d_n9, assign6920_e10605_d_n10, assign6920_e10605_d_n11, assign6920_e10605_d_n13, assign6920_e10605_d_n14,) = {
    if (var_guard65 == 0.0) {
        let assign6920_e10602: f64 = (1.0 - var_up_i);
        let assign6920_e10603: f64 = (var_u0_i * assign6920_e10602);
        (assign6920_e10603, (var_u0_i_dn0 * assign6920_e10602), (var_u0_i_dn2 * assign6920_e10602), (var_u0_i_dn3 * assign6920_e10602), (var_u0_i_dn4 * assign6920_e10602), (var_u0_i_dn5 * assign6920_e10602), (var_u0_i_dn6 * assign6920_e10602), (var_u0_i_dn7 * assign6920_e10602), (var_u0_i_dn8 * assign6920_e10602), (var_u0_i_dn9 * assign6920_e10602), (var_u0_i_dn10 * assign6920_e10602), (var_u0_i_dn11 * assign6920_e10602), (var_u0_i_dn13 * assign6920_e10602), (var_u0_i_dn14 * assign6920_e10602),)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign6920_e10605;
        var_u0_i_dn0 = assign6920_e10605_d_n0;
        var_u0_i_dn2 = assign6920_e10605_d_n2;
        var_u0_i_dn3 = assign6920_e10605_d_n3;
        var_u0_i_dn4 = assign6920_e10605_d_n4;
        var_u0_i_dn5 = assign6920_e10605_d_n5;
        var_u0_i_dn6 = assign6920_e10605_d_n6;
        var_u0_i_dn7 = assign6920_e10605_d_n7;
        var_u0_i_dn8 = assign6920_e10605_d_n8;
        var_u0_i_dn9 = assign6920_e10605_d_n9;
        var_u0_i_dn10 = assign6920_e10605_d_n10;
        var_u0_i_dn11 = assign6920_e10605_d_n11;
        var_u0_i_dn13 = assign6920_e10605_d_n13;
        var_u0_i_dn14 = assign6920_e10605_d_n14;
        var_u0_i_rv = 0.0;

        let assign6930_e10609: f64 = (-var_leff_1);
        let assign6930_e10611: f64 = (assign6930_e10609 / p.p593);
        let assign6930_e10612: f64 = { let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6930_e10613: f64 = (p.p591 * assign6930_e10612);
        let assign6930_e10614: f64 = (var_ua_i + assign6930_e10613);
        var_ua_i = assign6930_e10614;
        var_ua_i_dn0 = (var_ua_i_dn0 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p593))));
        var_ua_i_dn2 = (var_ua_i_dn2 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p593))));
        var_ua_i_dn3 = (var_ua_i_dn3 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p593))));
        var_ua_i_dn4 = (var_ua_i_dn4 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p593))));
        var_ua_i_dn5 = (var_ua_i_dn5 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p593))));
        var_ua_i_dn6 = (var_ua_i_dn6 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p593))));
        var_ua_i_dn7 = (var_ua_i_dn7 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p593))));
        var_ua_i_dn8 = (var_ua_i_dn8 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p593))));
        var_ua_i_dn9 = (var_ua_i_dn9 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p593))));
        var_ua_i_dn10 = (var_ua_i_dn10 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p593))));
        var_ua_i_dn11 = (var_ua_i_dn11 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p593))));
        var_ua_i_dn13 = (var_ua_i_dn13 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p593))));
        var_ua_i_dn14 = (var_ua_i_dn14 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p593))));
        var_ua_i_rv = 0.0;

        let assign6940_e10618: f64 = (-var_leff_1);
        let assign6940_e10620: f64 = (assign6940_e10618 / p.p601);
        let assign6940_e10621: f64 = { let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6940_e10622: f64 = (p.p599 * assign6940_e10621);
        let assign6940_e10623: f64 = (var_ud_i + assign6940_e10622);
        var_ud_i = assign6940_e10623;
        var_ud_i_dn0 = (var_ud_i_dn0 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p601))));
        var_ud_i_dn2 = (var_ud_i_dn2 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p601))));
        var_ud_i_dn3 = (var_ud_i_dn3 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p601))));
        var_ud_i_dn4 = (var_ud_i_dn4 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p601))));
        var_ud_i_dn5 = (var_ud_i_dn5 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p601))));
        var_ud_i_dn6 = (var_ud_i_dn6 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p601))));
        var_ud_i_dn7 = (var_ud_i_dn7 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p601))));
        var_ud_i_dn8 = (var_ud_i_dn8 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p601))));
        var_ud_i_dn9 = (var_ud_i_dn9 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p601))));
        var_ud_i_dn10 = (var_ud_i_dn10 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p601))));
        var_ud_i_dn11 = (var_ud_i_dn11 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p601))));
        var_ud_i_dn13 = (var_ud_i_dn13 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p601))));
        var_ud_i_dn14 = (var_ud_i_dn14 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p601))));
        var_ud_i_rv = 0.0;

        let assign6950_e10627: f64 = (-var_leff_1);
        let assign6950_e10629: f64 = (assign6950_e10627 / p.p597);
        let assign6950_e10630: f64 = { let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6950_e10631: f64 = (p.p595 * assign6950_e10630);
        let assign6950_e10632: f64 = (var_eu_i + assign6950_e10631);
        var_eu_i = assign6950_e10632;
        var_eu_i_dn0 = (var_eu_i_dn0 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p597))));
        var_eu_i_dn2 = (var_eu_i_dn2 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p597))));
        var_eu_i_dn3 = (var_eu_i_dn3 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p597))));
        var_eu_i_dn4 = (var_eu_i_dn4 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p597))));
        var_eu_i_dn5 = (var_eu_i_dn5 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p597))));
        var_eu_i_dn6 = (var_eu_i_dn6 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p597))));
        var_eu_i_dn7 = (var_eu_i_dn7 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p597))));
        var_eu_i_dn8 = (var_eu_i_dn8 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p597))));
        var_eu_i_dn9 = (var_eu_i_dn9 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p597))));
        var_eu_i_dn10 = (var_eu_i_dn10 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p597))));
        var_eu_i_dn11 = (var_eu_i_dn11 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p597))));
        var_eu_i_dn13 = (var_eu_i_dn13 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p597))));
        var_eu_i_dn14 = (var_eu_i_dn14 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p597))));
        var_eu_i_rv = 0.0;

        let assign6960_e10635: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard66 = assign6960_e10635;
        var_guard66_rv = 0.0;

        let (assign6970_e10647, assign6970_e10647_d_n0, assign6970_e10647_d_n2, assign6970_e10647_d_n3, assign6970_e10647_d_n4, assign6970_e10647_d_n5, assign6970_e10647_d_n6, assign6970_e10647_d_n7, assign6970_e10647_d_n8, assign6970_e10647_d_n9, assign6970_e10647_d_n10, assign6970_e10647_d_n11, assign6970_e10647_d_n13, assign6970_e10647_d_n14,) = {
    if (var_guard66 != 0.0) {
        let assign6970_e10640: f64 = (-var_leff_1);
        let assign6970_e10642: f64 = (assign6970_e10640 / p.p594);
        let assign6970_e10643: f64 = { let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6970_e10644: f64 = (p.p592 * assign6970_e10643);
        let assign6970_e10645: f64 = (var_uar_i + assign6970_e10644);
        (assign6970_e10645, (var_uar_i_dn0 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p594)))), (var_uar_i_dn2 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p594)))), (var_uar_i_dn3 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p594)))), (var_uar_i_dn4 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p594)))), (var_uar_i_dn5 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p594)))), (var_uar_i_dn6 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p594)))), (var_uar_i_dn7 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p594)))), (var_uar_i_dn8 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p594)))), (var_uar_i_dn9 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p594)))), (var_uar_i_dn10 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p594)))), (var_uar_i_dn11 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p594)))), (var_uar_i_dn13 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p594)))), (var_uar_i_dn14 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p594)))),)
    } else {
        (var_uar_i, var_uar_i_dn0, var_uar_i_dn2, var_uar_i_dn3, var_uar_i_dn4, var_uar_i_dn5, var_uar_i_dn6, var_uar_i_dn7, var_uar_i_dn8, var_uar_i_dn9, var_uar_i_dn10, var_uar_i_dn11, var_uar_i_dn13, var_uar_i_dn14,)
    }
};
        var_uar_i = assign6970_e10647;
        var_uar_i_dn0 = assign6970_e10647_d_n0;
        var_uar_i_dn2 = assign6970_e10647_d_n2;
        var_uar_i_dn3 = assign6970_e10647_d_n3;
        var_uar_i_dn4 = assign6970_e10647_d_n4;
        var_uar_i_dn5 = assign6970_e10647_d_n5;
        var_uar_i_dn6 = assign6970_e10647_d_n6;
        var_uar_i_dn7 = assign6970_e10647_d_n7;
        var_uar_i_dn8 = assign6970_e10647_d_n8;
        var_uar_i_dn9 = assign6970_e10647_d_n9;
        var_uar_i_dn10 = assign6970_e10647_d_n10;
        var_uar_i_dn11 = assign6970_e10647_d_n11;
        var_uar_i_dn13 = assign6970_e10647_d_n13;
        var_uar_i_dn14 = assign6970_e10647_d_n14;
        var_uar_i_rv = 0.0;

        let (assign6980_e10659, assign6980_e10659_d_n0, assign6980_e10659_d_n2, assign6980_e10659_d_n3, assign6980_e10659_d_n4, assign6980_e10659_d_n5, assign6980_e10659_d_n6, assign6980_e10659_d_n7, assign6980_e10659_d_n8, assign6980_e10659_d_n9, assign6980_e10659_d_n10, assign6980_e10659_d_n11, assign6980_e10659_d_n13, assign6980_e10659_d_n14,) = {
    if (var_guard66 != 0.0) {
        let assign6980_e10652: f64 = (-var_leff_1);
        let assign6980_e10654: f64 = (assign6980_e10652 / p.p602);
        let assign6980_e10655: f64 = { let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6980_e10656: f64 = (p.p600 * assign6980_e10655);
        let assign6980_e10657: f64 = (var_udr_i + assign6980_e10656);
        (assign6980_e10657, (var_udr_i_dn0 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p602)))), (var_udr_i_dn2 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p602)))), (var_udr_i_dn3 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p602)))), (var_udr_i_dn4 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p602)))), (var_udr_i_dn5 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p602)))), (var_udr_i_dn6 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p602)))), (var_udr_i_dn7 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p602)))), (var_udr_i_dn8 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p602)))), (var_udr_i_dn9 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p602)))), (var_udr_i_dn10 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p602)))), (var_udr_i_dn11 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p602)))), (var_udr_i_dn13 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p602)))), (var_udr_i_dn14 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p602)))),)
    } else {
        (var_udr_i, var_udr_i_dn0, var_udr_i_dn2, var_udr_i_dn3, var_udr_i_dn4, var_udr_i_dn5, var_udr_i_dn6, var_udr_i_dn7, var_udr_i_dn8, var_udr_i_dn9, var_udr_i_dn10, var_udr_i_dn11, var_udr_i_dn13, var_udr_i_dn14,)
    }
};
        var_udr_i = assign6980_e10659;
        var_udr_i_dn0 = assign6980_e10659_d_n0;
        var_udr_i_dn2 = assign6980_e10659_d_n2;
        var_udr_i_dn3 = assign6980_e10659_d_n3;
        var_udr_i_dn4 = assign6980_e10659_d_n4;
        var_udr_i_dn5 = assign6980_e10659_d_n5;
        var_udr_i_dn6 = assign6980_e10659_d_n6;
        var_udr_i_dn7 = assign6980_e10659_d_n7;
        var_udr_i_dn8 = assign6980_e10659_d_n8;
        var_udr_i_dn9 = assign6980_e10659_d_n9;
        var_udr_i_dn10 = assign6980_e10659_d_n10;
        var_udr_i_dn11 = assign6980_e10659_d_n11;
        var_udr_i_dn13 = assign6980_e10659_d_n13;
        var_udr_i_dn14 = assign6980_e10659_d_n14;
        var_udr_i_rv = 0.0;

        let (assign6990_e10671, assign6990_e10671_d_n0, assign6990_e10671_d_n2, assign6990_e10671_d_n3, assign6990_e10671_d_n4, assign6990_e10671_d_n5, assign6990_e10671_d_n6, assign6990_e10671_d_n7, assign6990_e10671_d_n8, assign6990_e10671_d_n9, assign6990_e10671_d_n10, assign6990_e10671_d_n11, assign6990_e10671_d_n13, assign6990_e10671_d_n14,) = {
    if (var_guard66 != 0.0) {
        let assign6990_e10664: f64 = (-var_leff_1);
        let assign6990_e10666: f64 = (assign6990_e10664 / p.p598);
        let assign6990_e10667: f64 = { let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6990_e10668: f64 = (p.p596 * assign6990_e10667);
        let assign6990_e10669: f64 = (var_eur_i + assign6990_e10668);
        (assign6990_e10669, (var_eur_i_dn0 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p598)))), (var_eur_i_dn2 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p598)))), (var_eur_i_dn3 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p598)))), (var_eur_i_dn4 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p598)))), (var_eur_i_dn5 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p598)))), (var_eur_i_dn6 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p598)))), (var_eur_i_dn7 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p598)))), (var_eur_i_dn8 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p598)))), (var_eur_i_dn9 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p598)))), (var_eur_i_dn10 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p598)))), (var_eur_i_dn11 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p598)))), (var_eur_i_dn13 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p598)))), (var_eur_i_dn14 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p598)))),)
    } else {
        (var_eur_i, var_eur_i_dn0, var_eur_i_dn2, var_eur_i_dn3, var_eur_i_dn4, var_eur_i_dn5, var_eur_i_dn6, var_eur_i_dn7, var_eur_i_dn8, var_eur_i_dn9, var_eur_i_dn10, var_eur_i_dn11, var_eur_i_dn13, var_eur_i_dn14,)
    }
};
        var_eur_i = assign6990_e10671;
        var_eur_i_dn0 = assign6990_e10671_d_n0;
        var_eur_i_dn2 = assign6990_e10671_d_n2;
        var_eur_i_dn3 = assign6990_e10671_d_n3;
        var_eur_i_dn4 = assign6990_e10671_d_n4;
        var_eur_i_dn5 = assign6990_e10671_d_n5;
        var_eur_i_dn6 = assign6990_e10671_d_n6;
        var_eur_i_dn7 = assign6990_e10671_d_n7;
        var_eur_i_dn8 = assign6990_e10671_d_n8;
        var_eur_i_dn9 = assign6990_e10671_d_n9;
        var_eur_i_dn10 = assign6990_e10671_d_n10;
        var_eur_i_dn11 = assign6990_e10671_d_n11;
        var_eur_i_dn13 = assign6990_e10671_d_n13;
        var_eur_i_dn14 = assign6990_e10671_d_n14;
        var_eur_i_rv = 0.0;

        let assign7000_e10674: f64 = if p.p590 > 0.0 { 1.0 } else { 0.0 };
        var_guard67 = assign7000_e10674;
        var_guard67_rv = 0.0;

        let (assign7010_e10690, assign7010_e10690_d_n0, assign7010_e10690_d_n2, assign7010_e10690_d_n3, assign7010_e10690_d_n4, assign7010_e10690_d_n5, assign7010_e10690_d_n6, assign7010_e10690_d_n7, assign7010_e10690_d_n8, assign7010_e10690_d_n9, assign7010_e10690_d_n10, assign7010_e10690_d_n11, assign7010_e10690_d_n13, assign7010_e10690_d_n14,) = {
    if ((var_guard66 != 0.0) && (var_guard67 != 0.0)) {
        let assign7010_e10682: f64 = (-p.p590);
        let assign7010_e10684: f64 = (assign7010_e10682 * var_leff_ln);
        let assign7010_e10685: f64 = (assign7010_e10684).exp();
        let assign7010_e10686: f64 = (var_upr_i * assign7010_e10685);
        let assign7010_e10687: f64 = (1.0 - assign7010_e10686);
        let assign7010_e10688: f64 = (var_u0r_i * assign7010_e10687);
        (assign7010_e10688, ((var_u0r_i_dn0 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn0)))))), ((var_u0r_i_dn2 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn2)))))), ((var_u0r_i_dn3 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn3)))))), ((var_u0r_i_dn4 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn4)))))), ((var_u0r_i_dn5 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn5)))))), ((var_u0r_i_dn6 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn6)))))), ((var_u0r_i_dn7 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn7)))))), ((var_u0r_i_dn8 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn8)))))), ((var_u0r_i_dn9 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn9)))))), ((var_u0r_i_dn10 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn10)))))), ((var_u0r_i_dn11 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn11)))))), ((var_u0r_i_dn13 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn13)))))), ((var_u0r_i_dn14 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn14)))))),)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign7010_e10690;
        var_u0r_i_dn0 = assign7010_e10690_d_n0;
        var_u0r_i_dn2 = assign7010_e10690_d_n2;
        var_u0r_i_dn3 = assign7010_e10690_d_n3;
        var_u0r_i_dn4 = assign7010_e10690_d_n4;
        var_u0r_i_dn5 = assign7010_e10690_d_n5;
        var_u0r_i_dn6 = assign7010_e10690_d_n6;
        var_u0r_i_dn7 = assign7010_e10690_d_n7;
        var_u0r_i_dn8 = assign7010_e10690_d_n8;
        var_u0r_i_dn9 = assign7010_e10690_d_n9;
        var_u0r_i_dn10 = assign7010_e10690_d_n10;
        var_u0r_i_dn11 = assign7010_e10690_d_n11;
        var_u0r_i_dn13 = assign7010_e10690_d_n13;
        var_u0r_i_dn14 = assign7010_e10690_d_n14;
        var_u0r_i_rv = 0.0;

        let (assign7020_e10701, assign7020_e10701_d_n0, assign7020_e10701_d_n2, assign7020_e10701_d_n3, assign7020_e10701_d_n4, assign7020_e10701_d_n5, assign7020_e10701_d_n6, assign7020_e10701_d_n7, assign7020_e10701_d_n8, assign7020_e10701_d_n9, assign7020_e10701_d_n10, assign7020_e10701_d_n11, assign7020_e10701_d_n13, assign7020_e10701_d_n14,) = {
    if ((var_guard66 != 0.0) && (var_guard67 == 0.0)) {
        let assign7020_e10698: f64 = (1.0 - var_upr_i);
        let assign7020_e10699: f64 = (var_u0r_i * assign7020_e10698);
        (assign7020_e10699, (var_u0r_i_dn0 * assign7020_e10698), (var_u0r_i_dn2 * assign7020_e10698), (var_u0r_i_dn3 * assign7020_e10698), (var_u0r_i_dn4 * assign7020_e10698), (var_u0r_i_dn5 * assign7020_e10698), (var_u0r_i_dn6 * assign7020_e10698), (var_u0r_i_dn7 * assign7020_e10698), (var_u0r_i_dn8 * assign7020_e10698), (var_u0r_i_dn9 * assign7020_e10698), (var_u0r_i_dn10 * assign7020_e10698), (var_u0r_i_dn11 * assign7020_e10698), (var_u0r_i_dn13 * assign7020_e10698), (var_u0r_i_dn14 * assign7020_e10698),)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign7020_e10701;
        var_u0r_i_dn0 = assign7020_e10701_d_n0;
        var_u0r_i_dn2 = assign7020_e10701_d_n2;
        var_u0r_i_dn3 = assign7020_e10701_d_n3;
        var_u0r_i_dn4 = assign7020_e10701_d_n4;
        var_u0r_i_dn5 = assign7020_e10701_d_n5;
        var_u0r_i_dn6 = assign7020_e10701_d_n6;
        var_u0r_i_dn7 = assign7020_e10701_d_n7;
        var_u0r_i_dn8 = assign7020_e10701_d_n8;
        var_u0r_i_dn9 = assign7020_e10701_d_n9;
        var_u0r_i_dn10 = assign7020_e10701_d_n10;
        var_u0r_i_dn11 = assign7020_e10701_d_n11;
        var_u0r_i_dn13 = assign7020_e10701_d_n13;
        var_u0r_i_dn14 = assign7020_e10701_d_n14;
        var_u0r_i_rv = 0.0;

        let assign7030_e10704: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        var_guard68 = assign7030_e10704;
        var_guard68_rv = 0.0;

        let (assign7040_e10716, assign7040_e10716_d_n0, assign7040_e10716_d_n2, assign7040_e10716_d_n3, assign7040_e10716_d_n4, assign7040_e10716_d_n5, assign7040_e10716_d_n6, assign7040_e10716_d_n7, assign7040_e10716_d_n8, assign7040_e10716_d_n9, assign7040_e10716_d_n10, assign7040_e10716_d_n11, assign7040_e10716_d_n13, assign7040_e10716_d_n14,) = {
    if (var_guard68 != 0.0) {
        let assign7040_e10709: f64 = (-var_leff_1);
        let assign7040_e10711: f64 = (assign7040_e10709 / p.p913);
        let assign7040_e10712: f64 = { let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7040_e10713: f64 = (p.p912 * assign7040_e10712);
        let assign7040_e10714: f64 = (var_rsw_i + assign7040_e10713);
        (assign7040_e10714, (var_rsw_i_dn0 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p913)))), (var_rsw_i_dn2 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p913)))), (var_rsw_i_dn3 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p913)))), (var_rsw_i_dn4 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p913)))), (var_rsw_i_dn5 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p913)))), (var_rsw_i_dn6 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p913)))), (var_rsw_i_dn7 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p913)))), (var_rsw_i_dn8 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p913)))), (var_rsw_i_dn9 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p913)))), (var_rsw_i_dn10 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p913)))), (var_rsw_i_dn11 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p913)))), (var_rsw_i_dn13 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p913)))), (var_rsw_i_dn14 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p913)))),)
    } else {
        (var_rsw_i, var_rsw_i_dn0, var_rsw_i_dn2, var_rsw_i_dn3, var_rsw_i_dn4, var_rsw_i_dn5, var_rsw_i_dn6, var_rsw_i_dn7, var_rsw_i_dn8, var_rsw_i_dn9, var_rsw_i_dn10, var_rsw_i_dn11, var_rsw_i_dn13, var_rsw_i_dn14,)
    }
};
        var_rsw_i = assign7040_e10716;
        var_rsw_i_dn0 = assign7040_e10716_d_n0;
        var_rsw_i_dn2 = assign7040_e10716_d_n2;
        var_rsw_i_dn3 = assign7040_e10716_d_n3;
        var_rsw_i_dn4 = assign7040_e10716_d_n4;
        var_rsw_i_dn5 = assign7040_e10716_d_n5;
        var_rsw_i_dn6 = assign7040_e10716_d_n6;
        var_rsw_i_dn7 = assign7040_e10716_d_n7;
        var_rsw_i_dn8 = assign7040_e10716_d_n8;
        var_rsw_i_dn9 = assign7040_e10716_d_n9;
        var_rsw_i_dn10 = assign7040_e10716_d_n10;
        var_rsw_i_dn11 = assign7040_e10716_d_n11;
        var_rsw_i_dn13 = assign7040_e10716_d_n13;
        var_rsw_i_dn14 = assign7040_e10716_d_n14;
        var_rsw_i_rv = 0.0;

        let (assign7050_e10728, assign7050_e10728_d_n0, assign7050_e10728_d_n2, assign7050_e10728_d_n3, assign7050_e10728_d_n4, assign7050_e10728_d_n5, assign7050_e10728_d_n6, assign7050_e10728_d_n7, assign7050_e10728_d_n8, assign7050_e10728_d_n9, assign7050_e10728_d_n10, assign7050_e10728_d_n11, assign7050_e10728_d_n13, assign7050_e10728_d_n14,) = {
    if (var_guard68 != 0.0) {
        let assign7050_e10721: f64 = (-var_leff_1);
        let assign7050_e10723: f64 = (assign7050_e10721 / p.p916);
        let assign7050_e10724: f64 = { let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7050_e10725: f64 = (p.p915 * assign7050_e10724);
        let assign7050_e10726: f64 = (var_rdw_i + assign7050_e10725);
        (assign7050_e10726, (var_rdw_i_dn0 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p916)))), (var_rdw_i_dn2 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p916)))), (var_rdw_i_dn3 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p916)))), (var_rdw_i_dn4 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p916)))), (var_rdw_i_dn5 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p916)))), (var_rdw_i_dn6 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p916)))), (var_rdw_i_dn7 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p916)))), (var_rdw_i_dn8 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p916)))), (var_rdw_i_dn9 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p916)))), (var_rdw_i_dn10 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p916)))), (var_rdw_i_dn11 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p916)))), (var_rdw_i_dn13 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p916)))), (var_rdw_i_dn14 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p916)))),)
    } else {
        (var_rdw_i, var_rdw_i_dn0, var_rdw_i_dn2, var_rdw_i_dn3, var_rdw_i_dn4, var_rdw_i_dn5, var_rdw_i_dn6, var_rdw_i_dn7, var_rdw_i_dn8, var_rdw_i_dn9, var_rdw_i_dn10, var_rdw_i_dn11, var_rdw_i_dn13, var_rdw_i_dn14,)
    }
};
        var_rdw_i = assign7050_e10728;
        var_rdw_i_dn0 = assign7050_e10728_d_n0;
        var_rdw_i_dn2 = assign7050_e10728_d_n2;
        var_rdw_i_dn3 = assign7050_e10728_d_n3;
        var_rdw_i_dn4 = assign7050_e10728_d_n4;
        var_rdw_i_dn5 = assign7050_e10728_d_n5;
        var_rdw_i_dn6 = assign7050_e10728_d_n6;
        var_rdw_i_dn7 = assign7050_e10728_d_n7;
        var_rdw_i_dn8 = assign7050_e10728_d_n8;
        var_rdw_i_dn9 = assign7050_e10728_d_n9;
        var_rdw_i_dn10 = assign7050_e10728_d_n10;
        var_rdw_i_dn11 = assign7050_e10728_d_n11;
        var_rdw_i_dn13 = assign7050_e10728_d_n13;
        var_rdw_i_dn14 = assign7050_e10728_d_n14;
        var_rdw_i_rv = 0.0;

        let (assign7060_e10741, assign7060_e10741_d_n0, assign7060_e10741_d_n2, assign7060_e10741_d_n3, assign7060_e10741_d_n4, assign7060_e10741_d_n5, assign7060_e10741_d_n6, assign7060_e10741_d_n7, assign7060_e10741_d_n8, assign7060_e10741_d_n9, assign7060_e10741_d_n10, assign7060_e10741_d_n11, assign7060_e10741_d_n13, assign7060_e10741_d_n14,) = {
    if (var_guard68 == 0.0) {
        let assign7060_e10734: f64 = (-var_leff_1);
        let assign7060_e10736: f64 = (assign7060_e10734 / p.p910);
        let assign7060_e10737: f64 = { let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7060_e10738: f64 = (p.p909 * assign7060_e10737);
        let assign7060_e10739: f64 = (var_rdsw_i + assign7060_e10738);
        (assign7060_e10739, (var_rdsw_i_dn0 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p910)))), (var_rdsw_i_dn2 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p910)))), (var_rdsw_i_dn3 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p910)))), (var_rdsw_i_dn4 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p910)))), (var_rdsw_i_dn5 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p910)))), (var_rdsw_i_dn6 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p910)))), (var_rdsw_i_dn7 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p910)))), (var_rdsw_i_dn8 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p910)))), (var_rdsw_i_dn9 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p910)))), (var_rdsw_i_dn10 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p910)))), (var_rdsw_i_dn11 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p910)))), (var_rdsw_i_dn13 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p910)))), (var_rdsw_i_dn14 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p910)))),)
    } else {
        (var_rdsw_i, var_rdsw_i_dn0, var_rdsw_i_dn2, var_rdsw_i_dn3, var_rdsw_i_dn4, var_rdsw_i_dn5, var_rdsw_i_dn6, var_rdsw_i_dn7, var_rdsw_i_dn8, var_rdsw_i_dn9, var_rdsw_i_dn10, var_rdsw_i_dn11, var_rdsw_i_dn13, var_rdsw_i_dn14,)
    }
};
        var_rdsw_i = assign7060_e10741;
        var_rdsw_i_dn0 = assign7060_e10741_d_n0;
        var_rdsw_i_dn2 = assign7060_e10741_d_n2;
        var_rdsw_i_dn3 = assign7060_e10741_d_n3;
        var_rdsw_i_dn4 = assign7060_e10741_d_n4;
        var_rdsw_i_dn5 = assign7060_e10741_d_n5;
        var_rdsw_i_dn6 = assign7060_e10741_d_n6;
        var_rdsw_i_dn7 = assign7060_e10741_d_n7;
        var_rdsw_i_dn8 = assign7060_e10741_d_n8;
        var_rdsw_i_dn9 = assign7060_e10741_d_n9;
        var_rdsw_i_dn10 = assign7060_e10741_d_n10;
        var_rdsw_i_dn11 = assign7060_e10741_d_n11;
        var_rdsw_i_dn13 = assign7060_e10741_d_n13;
        var_rdsw_i_dn14 = assign7060_e10741_d_n14;
        var_rdsw_i_rv = 0.0;

        *var_eta0_i_slot = var_eta0_i;
        *var_eta0_i_dn0_slot = var_eta0_i_dn0;
        *var_eta0_i_dn10_slot = var_eta0_i_dn10;
        *var_eta0_i_dn11_slot = var_eta0_i_dn11;
        *var_eta0_i_dn13_slot = var_eta0_i_dn13;
        *var_eta0_i_dn14_slot = var_eta0_i_dn14;
        *var_eta0_i_dn2_slot = var_eta0_i_dn2;
        *var_eta0_i_dn3_slot = var_eta0_i_dn3;
        *var_eta0_i_dn4_slot = var_eta0_i_dn4;
        *var_eta0_i_dn5_slot = var_eta0_i_dn5;
        *var_eta0_i_dn6_slot = var_eta0_i_dn6;
        *var_eta0_i_dn7_slot = var_eta0_i_dn7;
        *var_eta0_i_dn8_slot = var_eta0_i_dn8;
        *var_eta0_i_dn9_slot = var_eta0_i_dn9;
        *var_eta0_i_rv_slot = var_eta0_i_rv;
        *var_eu_i_slot = var_eu_i;
        *var_eu_i_dn0_slot = var_eu_i_dn0;
        *var_eu_i_dn10_slot = var_eu_i_dn10;
        *var_eu_i_dn11_slot = var_eu_i_dn11;
        *var_eu_i_dn13_slot = var_eu_i_dn13;
        *var_eu_i_dn14_slot = var_eu_i_dn14;
        *var_eu_i_dn2_slot = var_eu_i_dn2;
        *var_eu_i_dn3_slot = var_eu_i_dn3;
        *var_eu_i_dn4_slot = var_eu_i_dn4;
        *var_eu_i_dn5_slot = var_eu_i_dn5;
        *var_eu_i_dn6_slot = var_eu_i_dn6;
        *var_eu_i_dn7_slot = var_eu_i_dn7;
        *var_eu_i_dn8_slot = var_eu_i_dn8;
        *var_eu_i_dn9_slot = var_eu_i_dn9;
        *var_eu_i_rv_slot = var_eu_i_rv;
        *var_eur_i_slot = var_eur_i;
        *var_eur_i_dn0_slot = var_eur_i_dn0;
        *var_eur_i_dn10_slot = var_eur_i_dn10;
        *var_eur_i_dn11_slot = var_eur_i_dn11;
        *var_eur_i_dn13_slot = var_eur_i_dn13;
        *var_eur_i_dn14_slot = var_eur_i_dn14;
        *var_eur_i_dn2_slot = var_eur_i_dn2;
        *var_eur_i_dn3_slot = var_eur_i_dn3;
        *var_eur_i_dn4_slot = var_eur_i_dn4;
        *var_eur_i_dn5_slot = var_eur_i_dn5;
        *var_eur_i_dn6_slot = var_eur_i_dn6;
        *var_eur_i_dn7_slot = var_eur_i_dn7;
        *var_eur_i_dn8_slot = var_eur_i_dn8;
        *var_eur_i_dn9_slot = var_eur_i_dn9;
        *var_eur_i_rv_slot = var_eur_i_rv;
        *var_guard64_slot = var_guard64;
        *var_guard64_rv_slot = var_guard64_rv;
        *var_guard65_slot = var_guard65;
        *var_guard65_rv_slot = var_guard65_rv;
        *var_guard66_slot = var_guard66;
        *var_guard66_rv_slot = var_guard66_rv;
        *var_guard67_slot = var_guard67;
        *var_guard67_rv_slot = var_guard67_rv;
        *var_guard68_slot = var_guard68;
        *var_guard68_rv_slot = var_guard68_rv;
        *var_leff_ln_slot = var_leff_ln;
        *var_leff_ln_dn0_slot = var_leff_ln_dn0;
        *var_leff_ln_dn10_slot = var_leff_ln_dn10;
        *var_leff_ln_dn11_slot = var_leff_ln_dn11;
        *var_leff_ln_dn13_slot = var_leff_ln_dn13;
        *var_leff_ln_dn14_slot = var_leff_ln_dn14;
        *var_leff_ln_dn2_slot = var_leff_ln_dn2;
        *var_leff_ln_dn3_slot = var_leff_ln_dn3;
        *var_leff_ln_dn4_slot = var_leff_ln_dn4;
        *var_leff_ln_dn5_slot = var_leff_ln_dn5;
        *var_leff_ln_dn6_slot = var_leff_ln_dn6;
        *var_leff_ln_dn7_slot = var_leff_ln_dn7;
        *var_leff_ln_dn8_slot = var_leff_ln_dn8;
        *var_leff_ln_dn9_slot = var_leff_ln_dn9;
        *var_leff_ln_rv_slot = var_leff_ln_rv;
        *var_phig_i_slot = var_phig_i;
        *var_phig_i_dn0_slot = var_phig_i_dn0;
        *var_phig_i_dn10_slot = var_phig_i_dn10;
        *var_phig_i_dn11_slot = var_phig_i_dn11;
        *var_phig_i_dn13_slot = var_phig_i_dn13;
        *var_phig_i_dn14_slot = var_phig_i_dn14;
        *var_phig_i_dn2_slot = var_phig_i_dn2;
        *var_phig_i_dn3_slot = var_phig_i_dn3;
        *var_phig_i_dn4_slot = var_phig_i_dn4;
        *var_phig_i_dn5_slot = var_phig_i_dn5;
        *var_phig_i_dn6_slot = var_phig_i_dn6;
        *var_phig_i_dn7_slot = var_phig_i_dn7;
        *var_phig_i_dn8_slot = var_phig_i_dn8;
        *var_phig_i_dn9_slot = var_phig_i_dn9;
        *var_phig_i_rv_slot = var_phig_i_rv;
        *var_pqm_i_slot = var_pqm_i;
        *var_pqm_i_dn0_slot = var_pqm_i_dn0;
        *var_pqm_i_dn10_slot = var_pqm_i_dn10;
        *var_pqm_i_dn11_slot = var_pqm_i_dn11;
        *var_pqm_i_dn13_slot = var_pqm_i_dn13;
        *var_pqm_i_dn14_slot = var_pqm_i_dn14;
        *var_pqm_i_dn2_slot = var_pqm_i_dn2;
        *var_pqm_i_dn3_slot = var_pqm_i_dn3;
        *var_pqm_i_dn4_slot = var_pqm_i_dn4;
        *var_pqm_i_dn5_slot = var_pqm_i_dn5;
        *var_pqm_i_dn6_slot = var_pqm_i_dn6;
        *var_pqm_i_dn7_slot = var_pqm_i_dn7;
        *var_pqm_i_dn8_slot = var_pqm_i_dn8;
        *var_pqm_i_dn9_slot = var_pqm_i_dn9;
        *var_pqm_i_rv_slot = var_pqm_i_rv;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdsw_i_dn0_slot = var_rdsw_i_dn0;
        *var_rdsw_i_dn10_slot = var_rdsw_i_dn10;
        *var_rdsw_i_dn11_slot = var_rdsw_i_dn11;
        *var_rdsw_i_dn13_slot = var_rdsw_i_dn13;
        *var_rdsw_i_dn14_slot = var_rdsw_i_dn14;
        *var_rdsw_i_dn2_slot = var_rdsw_i_dn2;
        *var_rdsw_i_dn3_slot = var_rdsw_i_dn3;
        *var_rdsw_i_dn4_slot = var_rdsw_i_dn4;
        *var_rdsw_i_dn5_slot = var_rdsw_i_dn5;
        *var_rdsw_i_dn6_slot = var_rdsw_i_dn6;
        *var_rdsw_i_dn7_slot = var_rdsw_i_dn7;
        *var_rdsw_i_dn8_slot = var_rdsw_i_dn8;
        *var_rdsw_i_dn9_slot = var_rdsw_i_dn9;
        *var_rdsw_i_rv_slot = var_rdsw_i_rv;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdw_i_dn0_slot = var_rdw_i_dn0;
        *var_rdw_i_dn10_slot = var_rdw_i_dn10;
        *var_rdw_i_dn11_slot = var_rdw_i_dn11;
        *var_rdw_i_dn13_slot = var_rdw_i_dn13;
        *var_rdw_i_dn14_slot = var_rdw_i_dn14;
        *var_rdw_i_dn2_slot = var_rdw_i_dn2;
        *var_rdw_i_dn3_slot = var_rdw_i_dn3;
        *var_rdw_i_dn4_slot = var_rdw_i_dn4;
        *var_rdw_i_dn5_slot = var_rdw_i_dn5;
        *var_rdw_i_dn6_slot = var_rdw_i_dn6;
        *var_rdw_i_dn7_slot = var_rdw_i_dn7;
        *var_rdw_i_dn8_slot = var_rdw_i_dn8;
        *var_rdw_i_dn9_slot = var_rdw_i_dn9;
        *var_rdw_i_rv_slot = var_rdw_i_rv;
        *var_rsw_i_slot = var_rsw_i;
        *var_rsw_i_dn0_slot = var_rsw_i_dn0;
        *var_rsw_i_dn10_slot = var_rsw_i_dn10;
        *var_rsw_i_dn11_slot = var_rsw_i_dn11;
        *var_rsw_i_dn13_slot = var_rsw_i_dn13;
        *var_rsw_i_dn14_slot = var_rsw_i_dn14;
        *var_rsw_i_dn2_slot = var_rsw_i_dn2;
        *var_rsw_i_dn3_slot = var_rsw_i_dn3;
        *var_rsw_i_dn4_slot = var_rsw_i_dn4;
        *var_rsw_i_dn5_slot = var_rsw_i_dn5;
        *var_rsw_i_dn6_slot = var_rsw_i_dn6;
        *var_rsw_i_dn7_slot = var_rsw_i_dn7;
        *var_rsw_i_dn8_slot = var_rsw_i_dn8;
        *var_rsw_i_dn9_slot = var_rsw_i_dn9;
        *var_rsw_i_rv_slot = var_rsw_i_rv;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_dn0_slot = var_u0_i_dn0;
        *var_u0_i_dn10_slot = var_u0_i_dn10;
        *var_u0_i_dn11_slot = var_u0_i_dn11;
        *var_u0_i_dn13_slot = var_u0_i_dn13;
        *var_u0_i_dn14_slot = var_u0_i_dn14;
        *var_u0_i_dn2_slot = var_u0_i_dn2;
        *var_u0_i_dn3_slot = var_u0_i_dn3;
        *var_u0_i_dn4_slot = var_u0_i_dn4;
        *var_u0_i_dn5_slot = var_u0_i_dn5;
        *var_u0_i_dn6_slot = var_u0_i_dn6;
        *var_u0_i_dn7_slot = var_u0_i_dn7;
        *var_u0_i_dn8_slot = var_u0_i_dn8;
        *var_u0_i_dn9_slot = var_u0_i_dn9;
        *var_u0_i_rv_slot = var_u0_i_rv;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_dn0_slot = var_u0r_i_dn0;
        *var_u0r_i_dn10_slot = var_u0r_i_dn10;
        *var_u0r_i_dn11_slot = var_u0r_i_dn11;
        *var_u0r_i_dn13_slot = var_u0r_i_dn13;
        *var_u0r_i_dn14_slot = var_u0r_i_dn14;
        *var_u0r_i_dn2_slot = var_u0r_i_dn2;
        *var_u0r_i_dn3_slot = var_u0r_i_dn3;
        *var_u0r_i_dn4_slot = var_u0r_i_dn4;
        *var_u0r_i_dn5_slot = var_u0r_i_dn5;
        *var_u0r_i_dn6_slot = var_u0r_i_dn6;
        *var_u0r_i_dn7_slot = var_u0r_i_dn7;
        *var_u0r_i_dn8_slot = var_u0r_i_dn8;
        *var_u0r_i_dn9_slot = var_u0r_i_dn9;
        *var_u0r_i_rv_slot = var_u0r_i_rv;
        *var_ua_i_slot = var_ua_i;
        *var_ua_i_dn0_slot = var_ua_i_dn0;
        *var_ua_i_dn10_slot = var_ua_i_dn10;
        *var_ua_i_dn11_slot = var_ua_i_dn11;
        *var_ua_i_dn13_slot = var_ua_i_dn13;
        *var_ua_i_dn14_slot = var_ua_i_dn14;
        *var_ua_i_dn2_slot = var_ua_i_dn2;
        *var_ua_i_dn3_slot = var_ua_i_dn3;
        *var_ua_i_dn4_slot = var_ua_i_dn4;
        *var_ua_i_dn5_slot = var_ua_i_dn5;
        *var_ua_i_dn6_slot = var_ua_i_dn6;
        *var_ua_i_dn7_slot = var_ua_i_dn7;
        *var_ua_i_dn8_slot = var_ua_i_dn8;
        *var_ua_i_dn9_slot = var_ua_i_dn9;
        *var_ua_i_rv_slot = var_ua_i_rv;
        *var_uar_i_slot = var_uar_i;
        *var_uar_i_dn0_slot = var_uar_i_dn0;
        *var_uar_i_dn10_slot = var_uar_i_dn10;
        *var_uar_i_dn11_slot = var_uar_i_dn11;
        *var_uar_i_dn13_slot = var_uar_i_dn13;
        *var_uar_i_dn14_slot = var_uar_i_dn14;
        *var_uar_i_dn2_slot = var_uar_i_dn2;
        *var_uar_i_dn3_slot = var_uar_i_dn3;
        *var_uar_i_dn4_slot = var_uar_i_dn4;
        *var_uar_i_dn5_slot = var_uar_i_dn5;
        *var_uar_i_dn6_slot = var_uar_i_dn6;
        *var_uar_i_dn7_slot = var_uar_i_dn7;
        *var_uar_i_dn8_slot = var_uar_i_dn8;
        *var_uar_i_dn9_slot = var_uar_i_dn9;
        *var_uar_i_rv_slot = var_uar_i_rv;
        *var_ud_i_slot = var_ud_i;
        *var_ud_i_dn0_slot = var_ud_i_dn0;
        *var_ud_i_dn10_slot = var_ud_i_dn10;
        *var_ud_i_dn11_slot = var_ud_i_dn11;
        *var_ud_i_dn13_slot = var_ud_i_dn13;
        *var_ud_i_dn14_slot = var_ud_i_dn14;
        *var_ud_i_dn2_slot = var_ud_i_dn2;
        *var_ud_i_dn3_slot = var_ud_i_dn3;
        *var_ud_i_dn4_slot = var_ud_i_dn4;
        *var_ud_i_dn5_slot = var_ud_i_dn5;
        *var_ud_i_dn6_slot = var_ud_i_dn6;
        *var_ud_i_dn7_slot = var_ud_i_dn7;
        *var_ud_i_dn8_slot = var_ud_i_dn8;
        *var_ud_i_dn9_slot = var_ud_i_dn9;
        *var_ud_i_rv_slot = var_ud_i_rv;
        *var_udr_i_slot = var_udr_i;
        *var_udr_i_dn0_slot = var_udr_i_dn0;
        *var_udr_i_dn10_slot = var_udr_i_dn10;
        *var_udr_i_dn11_slot = var_udr_i_dn11;
        *var_udr_i_dn13_slot = var_udr_i_dn13;
        *var_udr_i_dn14_slot = var_udr_i_dn14;
        *var_udr_i_dn2_slot = var_udr_i_dn2;
        *var_udr_i_dn3_slot = var_udr_i_dn3;
        *var_udr_i_dn4_slot = var_udr_i_dn4;
        *var_udr_i_dn5_slot = var_udr_i_dn5;
        *var_udr_i_dn6_slot = var_udr_i_dn6;
        *var_udr_i_dn7_slot = var_udr_i_dn7;
        *var_udr_i_dn8_slot = var_udr_i_dn8;
        *var_udr_i_dn9_slot = var_udr_i_dn9;
        *var_udr_i_rv_slot = var_udr_i_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_ach: f64,
        var_covs_i: f64,
        var_leff_1: f64,
        var_leff_1_dn0: f64,
        var_leff_1_dn10: f64,
        var_leff_1_dn11: f64,
        var_leff_1_dn13: f64,
        var_leff_1_dn14: f64,
        var_leff_1_dn2: f64,
        var_leff_1_dn3: f64,
        var_leff_1_dn4: f64,
        var_leff_1_dn5: f64,
        var_leff_1_dn6: f64,
        var_leff_1_dn7: f64,
        var_leff_1_dn8: f64,
        var_leff_1_dn9: f64,
        var_leff_ln: f64,
        var_leff_ln_dn0: f64,
        var_leff_ln_dn10: f64,
        var_leff_ln_dn11: f64,
        var_leff_ln_dn13: f64,
        var_leff_ln_dn14: f64,
        var_leff_ln_dn2: f64,
        var_leff_ln_dn3: f64,
        var_leff_ln_dn4: f64,
        var_leff_ln_dn5: f64,
        var_leff_ln_dn6: f64,
        var_leff_ln_dn7: f64,
        var_leff_ln_dn8: f64,
        var_leff_ln_dn9: f64,
        var_leffcv_1: f64,
        var_leffcv_1_dn0: f64,
        var_leffcv_1_dn10: f64,
        var_leffcv_1_dn11: f64,
        var_leffcv_1_dn13: f64,
        var_leffcv_1_dn14: f64,
        var_leffcv_1_dn2: f64,
        var_leffcv_1_dn3: f64,
        var_leffcv_1_dn4: f64,
        var_leffcv_1_dn5: f64,
        var_leffcv_1_dn6: f64,
        var_leffcv_1_dn7: f64,
        var_leffcv_1_dn8: f64,
        var_leffcv_1_dn9: f64,
        var_qmtcencv_i: f64,
        var_qmtcencva_i: f64,
        var_weff_ufcm: f64,
        var_dvtp0_i_slot: &mut f64,
        var_dvtp0_i_dn0_slot: &mut f64,
        var_dvtp0_i_dn10_slot: &mut f64,
        var_dvtp0_i_dn11_slot: &mut f64,
        var_dvtp0_i_dn13_slot: &mut f64,
        var_dvtp0_i_dn14_slot: &mut f64,
        var_dvtp0_i_dn2_slot: &mut f64,
        var_dvtp0_i_dn3_slot: &mut f64,
        var_dvtp0_i_dn4_slot: &mut f64,
        var_dvtp0_i_dn5_slot: &mut f64,
        var_dvtp0_i_dn6_slot: &mut f64,
        var_dvtp0_i_dn7_slot: &mut f64,
        var_dvtp0_i_dn8_slot: &mut f64,
        var_dvtp0_i_dn9_slot: &mut f64,
        var_dvtp0_i_rv_slot: &mut f64,
        var_dvtp1_i_slot: &mut f64,
        var_dvtp1_i_dn0_slot: &mut f64,
        var_dvtp1_i_dn10_slot: &mut f64,
        var_dvtp1_i_dn11_slot: &mut f64,
        var_dvtp1_i_dn13_slot: &mut f64,
        var_dvtp1_i_dn14_slot: &mut f64,
        var_dvtp1_i_dn2_slot: &mut f64,
        var_dvtp1_i_dn3_slot: &mut f64,
        var_dvtp1_i_dn4_slot: &mut f64,
        var_dvtp1_i_dn5_slot: &mut f64,
        var_dvtp1_i_dn6_slot: &mut f64,
        var_dvtp1_i_dn7_slot: &mut f64,
        var_dvtp1_i_dn8_slot: &mut f64,
        var_dvtp1_i_dn9_slot: &mut f64,
        var_dvtp1_i_rv_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_guard69_rv_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard70_rv_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard71_rv_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard72_rv_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard73_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_guard81_rv_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard82_rv_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard83_rv_slot: &mut f64,
        var_guard84_slot: &mut f64,
        var_guard84_rv_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_guard85_rv_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard87_rv_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard88_rv_slot: &mut f64,
        var_k1_i_slot: &mut f64,
        var_k1_i_rv_slot: &mut f64,
        var_mexp_i_slot: &mut f64,
        var_mexp_i_dn0_slot: &mut f64,
        var_mexp_i_dn10_slot: &mut f64,
        var_mexp_i_dn11_slot: &mut f64,
        var_mexp_i_dn13_slot: &mut f64,
        var_mexp_i_dn14_slot: &mut f64,
        var_mexp_i_dn2_slot: &mut f64,
        var_mexp_i_dn3_slot: &mut f64,
        var_mexp_i_dn4_slot: &mut f64,
        var_mexp_i_dn5_slot: &mut f64,
        var_mexp_i_dn6_slot: &mut f64,
        var_mexp_i_dn7_slot: &mut f64,
        var_mexp_i_dn8_slot: &mut f64,
        var_mexp_i_dn9_slot: &mut f64,
        var_mexp_i_rv_slot: &mut f64,
        var_mexpr_i_slot: &mut f64,
        var_mexpr_i_dn0_slot: &mut f64,
        var_mexpr_i_dn10_slot: &mut f64,
        var_mexpr_i_dn11_slot: &mut f64,
        var_mexpr_i_dn13_slot: &mut f64,
        var_mexpr_i_dn14_slot: &mut f64,
        var_mexpr_i_dn2_slot: &mut f64,
        var_mexpr_i_dn3_slot: &mut f64,
        var_mexpr_i_dn4_slot: &mut f64,
        var_mexpr_i_dn5_slot: &mut f64,
        var_mexpr_i_dn6_slot: &mut f64,
        var_mexpr_i_dn7_slot: &mut f64,
        var_mexpr_i_dn8_slot: &mut f64,
        var_mexpr_i_dn9_slot: &mut f64,
        var_mexpr_i_rv_slot: &mut f64,
        var_mpower_i_slot: &mut f64,
        var_mpower_i_rv_slot: &mut f64,
        var_mtcen_slot: &mut f64,
        var_mtcen_rv_slot: &mut f64,
        var_noia2_i_slot: &mut f64,
        var_noia2_i_rv_slot: &mut f64,
        var_pclm_i_slot: &mut f64,
        var_pclm_i_dn0_slot: &mut f64,
        var_pclm_i_dn10_slot: &mut f64,
        var_pclm_i_dn11_slot: &mut f64,
        var_pclm_i_dn13_slot: &mut f64,
        var_pclm_i_dn14_slot: &mut f64,
        var_pclm_i_dn2_slot: &mut f64,
        var_pclm_i_dn3_slot: &mut f64,
        var_pclm_i_dn4_slot: &mut f64,
        var_pclm_i_dn5_slot: &mut f64,
        var_pclm_i_dn6_slot: &mut f64,
        var_pclm_i_dn7_slot: &mut f64,
        var_pclm_i_dn8_slot: &mut f64,
        var_pclm_i_dn9_slot: &mut f64,
        var_pclm_i_rv_slot: &mut f64,
        var_pclmr_i_slot: &mut f64,
        var_pclmr_i_dn0_slot: &mut f64,
        var_pclmr_i_dn10_slot: &mut f64,
        var_pclmr_i_dn11_slot: &mut f64,
        var_pclmr_i_dn13_slot: &mut f64,
        var_pclmr_i_dn14_slot: &mut f64,
        var_pclmr_i_dn2_slot: &mut f64,
        var_pclmr_i_dn3_slot: &mut f64,
        var_pclmr_i_dn4_slot: &mut f64,
        var_pclmr_i_dn5_slot: &mut f64,
        var_pclmr_i_dn6_slot: &mut f64,
        var_pclmr_i_dn7_slot: &mut f64,
        var_pclmr_i_dn8_slot: &mut f64,
        var_pclmr_i_dn9_slot: &mut f64,
        var_pclmr_i_rv_slot: &mut f64,
        var_phig_i_slot: &mut f64,
        var_phig_i_dn0_slot: &mut f64,
        var_phig_i_dn10_slot: &mut f64,
        var_phig_i_dn11_slot: &mut f64,
        var_phig_i_dn13_slot: &mut f64,
        var_phig_i_dn14_slot: &mut f64,
        var_phig_i_dn2_slot: &mut f64,
        var_phig_i_dn3_slot: &mut f64,
        var_phig_i_dn4_slot: &mut f64,
        var_phig_i_dn5_slot: &mut f64,
        var_phig_i_dn6_slot: &mut f64,
        var_phig_i_dn7_slot: &mut f64,
        var_phig_i_dn8_slot: &mut f64,
        var_phig_i_dn9_slot: &mut f64,
        var_phig_i_rv_slot: &mut f64,
        var_psat_i_slot: &mut f64,
        var_psat_i_dn0_slot: &mut f64,
        var_psat_i_dn10_slot: &mut f64,
        var_psat_i_dn11_slot: &mut f64,
        var_psat_i_dn13_slot: &mut f64,
        var_psat_i_dn14_slot: &mut f64,
        var_psat_i_dn2_slot: &mut f64,
        var_psat_i_dn3_slot: &mut f64,
        var_psat_i_dn4_slot: &mut f64,
        var_psat_i_dn5_slot: &mut f64,
        var_psat_i_dn6_slot: &mut f64,
        var_psat_i_dn7_slot: &mut f64,
        var_psat_i_dn8_slot: &mut f64,
        var_psat_i_dn9_slot: &mut f64,
        var_psat_i_rv_slot: &mut f64,
        var_psatcv_i_slot: &mut f64,
        var_psatcv_i_dn0_slot: &mut f64,
        var_psatcv_i_dn10_slot: &mut f64,
        var_psatcv_i_dn11_slot: &mut f64,
        var_psatcv_i_dn13_slot: &mut f64,
        var_psatcv_i_dn14_slot: &mut f64,
        var_psatcv_i_dn2_slot: &mut f64,
        var_psatcv_i_dn3_slot: &mut f64,
        var_psatcv_i_dn4_slot: &mut f64,
        var_psatcv_i_dn5_slot: &mut f64,
        var_psatcv_i_dn6_slot: &mut f64,
        var_psatcv_i_dn7_slot: &mut f64,
        var_psatcv_i_dn8_slot: &mut f64,
        var_psatcv_i_dn9_slot: &mut f64,
        var_psatcv_i_rv_slot: &mut f64,
        var_ptwg_i_slot: &mut f64,
        var_ptwg_i_dn0_slot: &mut f64,
        var_ptwg_i_dn10_slot: &mut f64,
        var_ptwg_i_dn11_slot: &mut f64,
        var_ptwg_i_dn13_slot: &mut f64,
        var_ptwg_i_dn14_slot: &mut f64,
        var_ptwg_i_dn2_slot: &mut f64,
        var_ptwg_i_dn3_slot: &mut f64,
        var_ptwg_i_dn4_slot: &mut f64,
        var_ptwg_i_dn5_slot: &mut f64,
        var_ptwg_i_dn6_slot: &mut f64,
        var_ptwg_i_dn7_slot: &mut f64,
        var_ptwg_i_dn8_slot: &mut f64,
        var_ptwg_i_dn9_slot: &mut f64,
        var_ptwg_i_rv_slot: &mut f64,
        var_ptwgr_i_slot: &mut f64,
        var_ptwgr_i_dn0_slot: &mut f64,
        var_ptwgr_i_dn10_slot: &mut f64,
        var_ptwgr_i_dn11_slot: &mut f64,
        var_ptwgr_i_dn13_slot: &mut f64,
        var_ptwgr_i_dn14_slot: &mut f64,
        var_ptwgr_i_dn2_slot: &mut f64,
        var_ptwgr_i_dn3_slot: &mut f64,
        var_ptwgr_i_dn4_slot: &mut f64,
        var_ptwgr_i_dn5_slot: &mut f64,
        var_ptwgr_i_dn6_slot: &mut f64,
        var_ptwgr_i_dn7_slot: &mut f64,
        var_ptwgr_i_dn8_slot: &mut f64,
        var_ptwgr_i_dn9_slot: &mut f64,
        var_ptwgr_i_rv_slot: &mut f64,
        var_qsref_i_slot: &mut f64,
        var_qsref_i_rv_slot: &mut f64,
        var_sprt_i_slot: &mut f64,
        var_sprt_i_rv_slot: &mut f64,
        var_tcen0_slot: &mut f64,
        var_tcen0_rv_slot: &mut f64,
        var_vsat1_i_slot: &mut f64,
        var_vsat1_i_dn0_slot: &mut f64,
        var_vsat1_i_dn10_slot: &mut f64,
        var_vsat1_i_dn11_slot: &mut f64,
        var_vsat1_i_dn13_slot: &mut f64,
        var_vsat1_i_dn14_slot: &mut f64,
        var_vsat1_i_dn2_slot: &mut f64,
        var_vsat1_i_dn3_slot: &mut f64,
        var_vsat1_i_dn4_slot: &mut f64,
        var_vsat1_i_dn5_slot: &mut f64,
        var_vsat1_i_dn6_slot: &mut f64,
        var_vsat1_i_dn7_slot: &mut f64,
        var_vsat1_i_dn8_slot: &mut f64,
        var_vsat1_i_dn9_slot: &mut f64,
        var_vsat1_i_rv_slot: &mut f64,
        var_vsat1r_i_slot: &mut f64,
        var_vsat1r_i_dn0_slot: &mut f64,
        var_vsat1r_i_dn10_slot: &mut f64,
        var_vsat1r_i_dn11_slot: &mut f64,
        var_vsat1r_i_dn13_slot: &mut f64,
        var_vsat1r_i_dn14_slot: &mut f64,
        var_vsat1r_i_dn2_slot: &mut f64,
        var_vsat1r_i_dn3_slot: &mut f64,
        var_vsat1r_i_dn4_slot: &mut f64,
        var_vsat1r_i_dn5_slot: &mut f64,
        var_vsat1r_i_dn6_slot: &mut f64,
        var_vsat1r_i_dn7_slot: &mut f64,
        var_vsat1r_i_dn8_slot: &mut f64,
        var_vsat1r_i_dn9_slot: &mut f64,
        var_vsat1r_i_rv_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsat_i_dn0_slot: &mut f64,
        var_vsat_i_dn10_slot: &mut f64,
        var_vsat_i_dn11_slot: &mut f64,
        var_vsat_i_dn13_slot: &mut f64,
        var_vsat_i_dn14_slot: &mut f64,
        var_vsat_i_dn2_slot: &mut f64,
        var_vsat_i_dn3_slot: &mut f64,
        var_vsat_i_dn4_slot: &mut f64,
        var_vsat_i_dn5_slot: &mut f64,
        var_vsat_i_dn6_slot: &mut f64,
        var_vsat_i_dn7_slot: &mut f64,
        var_vsat_i_dn8_slot: &mut f64,
        var_vsat_i_dn9_slot: &mut f64,
        var_vsat_i_rv_slot: &mut f64,
        var_vsatcv_i_slot: &mut f64,
        var_vsatcv_i_dn0_slot: &mut f64,
        var_vsatcv_i_dn10_slot: &mut f64,
        var_vsatcv_i_dn11_slot: &mut f64,
        var_vsatcv_i_dn13_slot: &mut f64,
        var_vsatcv_i_dn14_slot: &mut f64,
        var_vsatcv_i_dn2_slot: &mut f64,
        var_vsatcv_i_dn3_slot: &mut f64,
        var_vsatcv_i_dn4_slot: &mut f64,
        var_vsatcv_i_dn5_slot: &mut f64,
        var_vsatcv_i_dn6_slot: &mut f64,
        var_vsatcv_i_dn7_slot: &mut f64,
        var_vsatcv_i_dn8_slot: &mut f64,
        var_vsatcv_i_dn9_slot: &mut f64,
        var_vsatcv_i_rv_slot: &mut f64,
    ) {
        let mut var_dvtp0_i: f64 = *var_dvtp0_i_slot;
        let mut var_dvtp0_i_dn0: f64 = *var_dvtp0_i_dn0_slot;
        let mut var_dvtp0_i_dn10: f64 = *var_dvtp0_i_dn10_slot;
        let mut var_dvtp0_i_dn11: f64 = *var_dvtp0_i_dn11_slot;
        let mut var_dvtp0_i_dn13: f64 = *var_dvtp0_i_dn13_slot;
        let mut var_dvtp0_i_dn14: f64 = *var_dvtp0_i_dn14_slot;
        let mut var_dvtp0_i_dn2: f64 = *var_dvtp0_i_dn2_slot;
        let mut var_dvtp0_i_dn3: f64 = *var_dvtp0_i_dn3_slot;
        let mut var_dvtp0_i_dn4: f64 = *var_dvtp0_i_dn4_slot;
        let mut var_dvtp0_i_dn5: f64 = *var_dvtp0_i_dn5_slot;
        let mut var_dvtp0_i_dn6: f64 = *var_dvtp0_i_dn6_slot;
        let mut var_dvtp0_i_dn7: f64 = *var_dvtp0_i_dn7_slot;
        let mut var_dvtp0_i_dn8: f64 = *var_dvtp0_i_dn8_slot;
        let mut var_dvtp0_i_dn9: f64 = *var_dvtp0_i_dn9_slot;
        let mut var_dvtp0_i_rv: f64 = *var_dvtp0_i_rv_slot;
        let mut var_dvtp1_i: f64 = *var_dvtp1_i_slot;
        let mut var_dvtp1_i_dn0: f64 = *var_dvtp1_i_dn0_slot;
        let mut var_dvtp1_i_dn10: f64 = *var_dvtp1_i_dn10_slot;
        let mut var_dvtp1_i_dn11: f64 = *var_dvtp1_i_dn11_slot;
        let mut var_dvtp1_i_dn13: f64 = *var_dvtp1_i_dn13_slot;
        let mut var_dvtp1_i_dn14: f64 = *var_dvtp1_i_dn14_slot;
        let mut var_dvtp1_i_dn2: f64 = *var_dvtp1_i_dn2_slot;
        let mut var_dvtp1_i_dn3: f64 = *var_dvtp1_i_dn3_slot;
        let mut var_dvtp1_i_dn4: f64 = *var_dvtp1_i_dn4_slot;
        let mut var_dvtp1_i_dn5: f64 = *var_dvtp1_i_dn5_slot;
        let mut var_dvtp1_i_dn6: f64 = *var_dvtp1_i_dn6_slot;
        let mut var_dvtp1_i_dn7: f64 = *var_dvtp1_i_dn7_slot;
        let mut var_dvtp1_i_dn8: f64 = *var_dvtp1_i_dn8_slot;
        let mut var_dvtp1_i_dn9: f64 = *var_dvtp1_i_dn9_slot;
        let mut var_dvtp1_i_rv: f64 = *var_dvtp1_i_rv_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard69_rv: f64 = *var_guard69_rv_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard70_rv: f64 = *var_guard70_rv_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard71_rv: f64 = *var_guard71_rv_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard72_rv: f64 = *var_guard72_rv_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard73_rv: f64 = *var_guard73_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard81_rv: f64 = *var_guard81_rv_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard82_rv: f64 = *var_guard82_rv_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard83_rv: f64 = *var_guard83_rv_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard84_rv: f64 = *var_guard84_rv_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_guard85_rv: f64 = *var_guard85_rv_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard87_rv: f64 = *var_guard87_rv_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard88_rv: f64 = *var_guard88_rv_slot;
        let mut var_k1_i: f64 = *var_k1_i_slot;
        let mut var_k1_i_rv: f64 = *var_k1_i_rv_slot;
        let mut var_mexp_i: f64 = *var_mexp_i_slot;
        let mut var_mexp_i_dn0: f64 = *var_mexp_i_dn0_slot;
        let mut var_mexp_i_dn10: f64 = *var_mexp_i_dn10_slot;
        let mut var_mexp_i_dn11: f64 = *var_mexp_i_dn11_slot;
        let mut var_mexp_i_dn13: f64 = *var_mexp_i_dn13_slot;
        let mut var_mexp_i_dn14: f64 = *var_mexp_i_dn14_slot;
        let mut var_mexp_i_dn2: f64 = *var_mexp_i_dn2_slot;
        let mut var_mexp_i_dn3: f64 = *var_mexp_i_dn3_slot;
        let mut var_mexp_i_dn4: f64 = *var_mexp_i_dn4_slot;
        let mut var_mexp_i_dn5: f64 = *var_mexp_i_dn5_slot;
        let mut var_mexp_i_dn6: f64 = *var_mexp_i_dn6_slot;
        let mut var_mexp_i_dn7: f64 = *var_mexp_i_dn7_slot;
        let mut var_mexp_i_dn8: f64 = *var_mexp_i_dn8_slot;
        let mut var_mexp_i_dn9: f64 = *var_mexp_i_dn9_slot;
        let mut var_mexp_i_rv: f64 = *var_mexp_i_rv_slot;
        let mut var_mexpr_i: f64 = *var_mexpr_i_slot;
        let mut var_mexpr_i_dn0: f64 = *var_mexpr_i_dn0_slot;
        let mut var_mexpr_i_dn10: f64 = *var_mexpr_i_dn10_slot;
        let mut var_mexpr_i_dn11: f64 = *var_mexpr_i_dn11_slot;
        let mut var_mexpr_i_dn13: f64 = *var_mexpr_i_dn13_slot;
        let mut var_mexpr_i_dn14: f64 = *var_mexpr_i_dn14_slot;
        let mut var_mexpr_i_dn2: f64 = *var_mexpr_i_dn2_slot;
        let mut var_mexpr_i_dn3: f64 = *var_mexpr_i_dn3_slot;
        let mut var_mexpr_i_dn4: f64 = *var_mexpr_i_dn4_slot;
        let mut var_mexpr_i_dn5: f64 = *var_mexpr_i_dn5_slot;
        let mut var_mexpr_i_dn6: f64 = *var_mexpr_i_dn6_slot;
        let mut var_mexpr_i_dn7: f64 = *var_mexpr_i_dn7_slot;
        let mut var_mexpr_i_dn8: f64 = *var_mexpr_i_dn8_slot;
        let mut var_mexpr_i_dn9: f64 = *var_mexpr_i_dn9_slot;
        let mut var_mexpr_i_rv: f64 = *var_mexpr_i_rv_slot;
        let mut var_mpower_i: f64 = *var_mpower_i_slot;
        let mut var_mpower_i_rv: f64 = *var_mpower_i_rv_slot;
        let mut var_mtcen: f64 = *var_mtcen_slot;
        let mut var_mtcen_rv: f64 = *var_mtcen_rv_slot;
        let mut var_noia2_i: f64 = *var_noia2_i_slot;
        let mut var_noia2_i_rv: f64 = *var_noia2_i_rv_slot;
        let mut var_pclm_i: f64 = *var_pclm_i_slot;
        let mut var_pclm_i_dn0: f64 = *var_pclm_i_dn0_slot;
        let mut var_pclm_i_dn10: f64 = *var_pclm_i_dn10_slot;
        let mut var_pclm_i_dn11: f64 = *var_pclm_i_dn11_slot;
        let mut var_pclm_i_dn13: f64 = *var_pclm_i_dn13_slot;
        let mut var_pclm_i_dn14: f64 = *var_pclm_i_dn14_slot;
        let mut var_pclm_i_dn2: f64 = *var_pclm_i_dn2_slot;
        let mut var_pclm_i_dn3: f64 = *var_pclm_i_dn3_slot;
        let mut var_pclm_i_dn4: f64 = *var_pclm_i_dn4_slot;
        let mut var_pclm_i_dn5: f64 = *var_pclm_i_dn5_slot;
        let mut var_pclm_i_dn6: f64 = *var_pclm_i_dn6_slot;
        let mut var_pclm_i_dn7: f64 = *var_pclm_i_dn7_slot;
        let mut var_pclm_i_dn8: f64 = *var_pclm_i_dn8_slot;
        let mut var_pclm_i_dn9: f64 = *var_pclm_i_dn9_slot;
        let mut var_pclm_i_rv: f64 = *var_pclm_i_rv_slot;
        let mut var_pclmr_i: f64 = *var_pclmr_i_slot;
        let mut var_pclmr_i_dn0: f64 = *var_pclmr_i_dn0_slot;
        let mut var_pclmr_i_dn10: f64 = *var_pclmr_i_dn10_slot;
        let mut var_pclmr_i_dn11: f64 = *var_pclmr_i_dn11_slot;
        let mut var_pclmr_i_dn13: f64 = *var_pclmr_i_dn13_slot;
        let mut var_pclmr_i_dn14: f64 = *var_pclmr_i_dn14_slot;
        let mut var_pclmr_i_dn2: f64 = *var_pclmr_i_dn2_slot;
        let mut var_pclmr_i_dn3: f64 = *var_pclmr_i_dn3_slot;
        let mut var_pclmr_i_dn4: f64 = *var_pclmr_i_dn4_slot;
        let mut var_pclmr_i_dn5: f64 = *var_pclmr_i_dn5_slot;
        let mut var_pclmr_i_dn6: f64 = *var_pclmr_i_dn6_slot;
        let mut var_pclmr_i_dn7: f64 = *var_pclmr_i_dn7_slot;
        let mut var_pclmr_i_dn8: f64 = *var_pclmr_i_dn8_slot;
        let mut var_pclmr_i_dn9: f64 = *var_pclmr_i_dn9_slot;
        let mut var_pclmr_i_rv: f64 = *var_pclmr_i_rv_slot;
        let mut var_phig_i: f64 = *var_phig_i_slot;
        let mut var_phig_i_dn0: f64 = *var_phig_i_dn0_slot;
        let mut var_phig_i_dn10: f64 = *var_phig_i_dn10_slot;
        let mut var_phig_i_dn11: f64 = *var_phig_i_dn11_slot;
        let mut var_phig_i_dn13: f64 = *var_phig_i_dn13_slot;
        let mut var_phig_i_dn14: f64 = *var_phig_i_dn14_slot;
        let mut var_phig_i_dn2: f64 = *var_phig_i_dn2_slot;
        let mut var_phig_i_dn3: f64 = *var_phig_i_dn3_slot;
        let mut var_phig_i_dn4: f64 = *var_phig_i_dn4_slot;
        let mut var_phig_i_dn5: f64 = *var_phig_i_dn5_slot;
        let mut var_phig_i_dn6: f64 = *var_phig_i_dn6_slot;
        let mut var_phig_i_dn7: f64 = *var_phig_i_dn7_slot;
        let mut var_phig_i_dn8: f64 = *var_phig_i_dn8_slot;
        let mut var_phig_i_dn9: f64 = *var_phig_i_dn9_slot;
        let mut var_phig_i_rv: f64 = *var_phig_i_rv_slot;
        let mut var_psat_i: f64 = *var_psat_i_slot;
        let mut var_psat_i_dn0: f64 = *var_psat_i_dn0_slot;
        let mut var_psat_i_dn10: f64 = *var_psat_i_dn10_slot;
        let mut var_psat_i_dn11: f64 = *var_psat_i_dn11_slot;
        let mut var_psat_i_dn13: f64 = *var_psat_i_dn13_slot;
        let mut var_psat_i_dn14: f64 = *var_psat_i_dn14_slot;
        let mut var_psat_i_dn2: f64 = *var_psat_i_dn2_slot;
        let mut var_psat_i_dn3: f64 = *var_psat_i_dn3_slot;
        let mut var_psat_i_dn4: f64 = *var_psat_i_dn4_slot;
        let mut var_psat_i_dn5: f64 = *var_psat_i_dn5_slot;
        let mut var_psat_i_dn6: f64 = *var_psat_i_dn6_slot;
        let mut var_psat_i_dn7: f64 = *var_psat_i_dn7_slot;
        let mut var_psat_i_dn8: f64 = *var_psat_i_dn8_slot;
        let mut var_psat_i_dn9: f64 = *var_psat_i_dn9_slot;
        let mut var_psat_i_rv: f64 = *var_psat_i_rv_slot;
        let mut var_psatcv_i: f64 = *var_psatcv_i_slot;
        let mut var_psatcv_i_dn0: f64 = *var_psatcv_i_dn0_slot;
        let mut var_psatcv_i_dn10: f64 = *var_psatcv_i_dn10_slot;
        let mut var_psatcv_i_dn11: f64 = *var_psatcv_i_dn11_slot;
        let mut var_psatcv_i_dn13: f64 = *var_psatcv_i_dn13_slot;
        let mut var_psatcv_i_dn14: f64 = *var_psatcv_i_dn14_slot;
        let mut var_psatcv_i_dn2: f64 = *var_psatcv_i_dn2_slot;
        let mut var_psatcv_i_dn3: f64 = *var_psatcv_i_dn3_slot;
        let mut var_psatcv_i_dn4: f64 = *var_psatcv_i_dn4_slot;
        let mut var_psatcv_i_dn5: f64 = *var_psatcv_i_dn5_slot;
        let mut var_psatcv_i_dn6: f64 = *var_psatcv_i_dn6_slot;
        let mut var_psatcv_i_dn7: f64 = *var_psatcv_i_dn7_slot;
        let mut var_psatcv_i_dn8: f64 = *var_psatcv_i_dn8_slot;
        let mut var_psatcv_i_dn9: f64 = *var_psatcv_i_dn9_slot;
        let mut var_psatcv_i_rv: f64 = *var_psatcv_i_rv_slot;
        let mut var_ptwg_i: f64 = *var_ptwg_i_slot;
        let mut var_ptwg_i_dn0: f64 = *var_ptwg_i_dn0_slot;
        let mut var_ptwg_i_dn10: f64 = *var_ptwg_i_dn10_slot;
        let mut var_ptwg_i_dn11: f64 = *var_ptwg_i_dn11_slot;
        let mut var_ptwg_i_dn13: f64 = *var_ptwg_i_dn13_slot;
        let mut var_ptwg_i_dn14: f64 = *var_ptwg_i_dn14_slot;
        let mut var_ptwg_i_dn2: f64 = *var_ptwg_i_dn2_slot;
        let mut var_ptwg_i_dn3: f64 = *var_ptwg_i_dn3_slot;
        let mut var_ptwg_i_dn4: f64 = *var_ptwg_i_dn4_slot;
        let mut var_ptwg_i_dn5: f64 = *var_ptwg_i_dn5_slot;
        let mut var_ptwg_i_dn6: f64 = *var_ptwg_i_dn6_slot;
        let mut var_ptwg_i_dn7: f64 = *var_ptwg_i_dn7_slot;
        let mut var_ptwg_i_dn8: f64 = *var_ptwg_i_dn8_slot;
        let mut var_ptwg_i_dn9: f64 = *var_ptwg_i_dn9_slot;
        let mut var_ptwg_i_rv: f64 = *var_ptwg_i_rv_slot;
        let mut var_ptwgr_i: f64 = *var_ptwgr_i_slot;
        let mut var_ptwgr_i_dn0: f64 = *var_ptwgr_i_dn0_slot;
        let mut var_ptwgr_i_dn10: f64 = *var_ptwgr_i_dn10_slot;
        let mut var_ptwgr_i_dn11: f64 = *var_ptwgr_i_dn11_slot;
        let mut var_ptwgr_i_dn13: f64 = *var_ptwgr_i_dn13_slot;
        let mut var_ptwgr_i_dn14: f64 = *var_ptwgr_i_dn14_slot;
        let mut var_ptwgr_i_dn2: f64 = *var_ptwgr_i_dn2_slot;
        let mut var_ptwgr_i_dn3: f64 = *var_ptwgr_i_dn3_slot;
        let mut var_ptwgr_i_dn4: f64 = *var_ptwgr_i_dn4_slot;
        let mut var_ptwgr_i_dn5: f64 = *var_ptwgr_i_dn5_slot;
        let mut var_ptwgr_i_dn6: f64 = *var_ptwgr_i_dn6_slot;
        let mut var_ptwgr_i_dn7: f64 = *var_ptwgr_i_dn7_slot;
        let mut var_ptwgr_i_dn8: f64 = *var_ptwgr_i_dn8_slot;
        let mut var_ptwgr_i_dn9: f64 = *var_ptwgr_i_dn9_slot;
        let mut var_ptwgr_i_rv: f64 = *var_ptwgr_i_rv_slot;
        let mut var_qsref_i: f64 = *var_qsref_i_slot;
        let mut var_qsref_i_rv: f64 = *var_qsref_i_rv_slot;
        let mut var_sprt_i: f64 = *var_sprt_i_slot;
        let mut var_sprt_i_rv: f64 = *var_sprt_i_rv_slot;
        let mut var_tcen0: f64 = *var_tcen0_slot;
        let mut var_tcen0_rv: f64 = *var_tcen0_rv_slot;
        let mut var_vsat1_i: f64 = *var_vsat1_i_slot;
        let mut var_vsat1_i_dn0: f64 = *var_vsat1_i_dn0_slot;
        let mut var_vsat1_i_dn10: f64 = *var_vsat1_i_dn10_slot;
        let mut var_vsat1_i_dn11: f64 = *var_vsat1_i_dn11_slot;
        let mut var_vsat1_i_dn13: f64 = *var_vsat1_i_dn13_slot;
        let mut var_vsat1_i_dn14: f64 = *var_vsat1_i_dn14_slot;
        let mut var_vsat1_i_dn2: f64 = *var_vsat1_i_dn2_slot;
        let mut var_vsat1_i_dn3: f64 = *var_vsat1_i_dn3_slot;
        let mut var_vsat1_i_dn4: f64 = *var_vsat1_i_dn4_slot;
        let mut var_vsat1_i_dn5: f64 = *var_vsat1_i_dn5_slot;
        let mut var_vsat1_i_dn6: f64 = *var_vsat1_i_dn6_slot;
        let mut var_vsat1_i_dn7: f64 = *var_vsat1_i_dn7_slot;
        let mut var_vsat1_i_dn8: f64 = *var_vsat1_i_dn8_slot;
        let mut var_vsat1_i_dn9: f64 = *var_vsat1_i_dn9_slot;
        let mut var_vsat1_i_rv: f64 = *var_vsat1_i_rv_slot;
        let mut var_vsat1r_i: f64 = *var_vsat1r_i_slot;
        let mut var_vsat1r_i_dn0: f64 = *var_vsat1r_i_dn0_slot;
        let mut var_vsat1r_i_dn10: f64 = *var_vsat1r_i_dn10_slot;
        let mut var_vsat1r_i_dn11: f64 = *var_vsat1r_i_dn11_slot;
        let mut var_vsat1r_i_dn13: f64 = *var_vsat1r_i_dn13_slot;
        let mut var_vsat1r_i_dn14: f64 = *var_vsat1r_i_dn14_slot;
        let mut var_vsat1r_i_dn2: f64 = *var_vsat1r_i_dn2_slot;
        let mut var_vsat1r_i_dn3: f64 = *var_vsat1r_i_dn3_slot;
        let mut var_vsat1r_i_dn4: f64 = *var_vsat1r_i_dn4_slot;
        let mut var_vsat1r_i_dn5: f64 = *var_vsat1r_i_dn5_slot;
        let mut var_vsat1r_i_dn6: f64 = *var_vsat1r_i_dn6_slot;
        let mut var_vsat1r_i_dn7: f64 = *var_vsat1r_i_dn7_slot;
        let mut var_vsat1r_i_dn8: f64 = *var_vsat1r_i_dn8_slot;
        let mut var_vsat1r_i_dn9: f64 = *var_vsat1r_i_dn9_slot;
        let mut var_vsat1r_i_rv: f64 = *var_vsat1r_i_rv_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsat_i_dn0: f64 = *var_vsat_i_dn0_slot;
        let mut var_vsat_i_dn10: f64 = *var_vsat_i_dn10_slot;
        let mut var_vsat_i_dn11: f64 = *var_vsat_i_dn11_slot;
        let mut var_vsat_i_dn13: f64 = *var_vsat_i_dn13_slot;
        let mut var_vsat_i_dn14: f64 = *var_vsat_i_dn14_slot;
        let mut var_vsat_i_dn2: f64 = *var_vsat_i_dn2_slot;
        let mut var_vsat_i_dn3: f64 = *var_vsat_i_dn3_slot;
        let mut var_vsat_i_dn4: f64 = *var_vsat_i_dn4_slot;
        let mut var_vsat_i_dn5: f64 = *var_vsat_i_dn5_slot;
        let mut var_vsat_i_dn6: f64 = *var_vsat_i_dn6_slot;
        let mut var_vsat_i_dn7: f64 = *var_vsat_i_dn7_slot;
        let mut var_vsat_i_dn8: f64 = *var_vsat_i_dn8_slot;
        let mut var_vsat_i_dn9: f64 = *var_vsat_i_dn9_slot;
        let mut var_vsat_i_rv: f64 = *var_vsat_i_rv_slot;
        let mut var_vsatcv_i: f64 = *var_vsatcv_i_slot;
        let mut var_vsatcv_i_dn0: f64 = *var_vsatcv_i_dn0_slot;
        let mut var_vsatcv_i_dn10: f64 = *var_vsatcv_i_dn10_slot;
        let mut var_vsatcv_i_dn11: f64 = *var_vsatcv_i_dn11_slot;
        let mut var_vsatcv_i_dn13: f64 = *var_vsatcv_i_dn13_slot;
        let mut var_vsatcv_i_dn14: f64 = *var_vsatcv_i_dn14_slot;
        let mut var_vsatcv_i_dn2: f64 = *var_vsatcv_i_dn2_slot;
        let mut var_vsatcv_i_dn3: f64 = *var_vsatcv_i_dn3_slot;
        let mut var_vsatcv_i_dn4: f64 = *var_vsatcv_i_dn4_slot;
        let mut var_vsatcv_i_dn5: f64 = *var_vsatcv_i_dn5_slot;
        let mut var_vsatcv_i_dn6: f64 = *var_vsatcv_i_dn6_slot;
        let mut var_vsatcv_i_dn7: f64 = *var_vsatcv_i_dn7_slot;
        let mut var_vsatcv_i_dn8: f64 = *var_vsatcv_i_dn8_slot;
        let mut var_vsatcv_i_dn9: f64 = *var_vsatcv_i_dn9_slot;
        let mut var_vsatcv_i_rv: f64 = *var_vsatcv_i_rv_slot;

        let assign7070_e10745: f64 = (-var_leff_1);
        let assign7070_e10747: f64 = (assign7070_e10745 / p.p1023);
        let assign7070_e10748: f64 = { let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7070_e10749: f64 = (p.p1021 * assign7070_e10748);
        let assign7070_e10750: f64 = (var_pclm_i + assign7070_e10749);
        var_pclm_i = assign7070_e10750;
        var_pclm_i_dn0 = (var_pclm_i_dn0 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p1023))));
        var_pclm_i_dn2 = (var_pclm_i_dn2 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p1023))));
        var_pclm_i_dn3 = (var_pclm_i_dn3 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p1023))));
        var_pclm_i_dn4 = (var_pclm_i_dn4 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p1023))));
        var_pclm_i_dn5 = (var_pclm_i_dn5 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p1023))));
        var_pclm_i_dn6 = (var_pclm_i_dn6 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p1023))));
        var_pclm_i_dn7 = (var_pclm_i_dn7 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p1023))));
        var_pclm_i_dn8 = (var_pclm_i_dn8 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p1023))));
        var_pclm_i_dn9 = (var_pclm_i_dn9 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p1023))));
        var_pclm_i_dn10 = (var_pclm_i_dn10 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p1023))));
        var_pclm_i_dn11 = (var_pclm_i_dn11 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p1023))));
        var_pclm_i_dn13 = (var_pclm_i_dn13 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p1023))));
        var_pclm_i_dn14 = (var_pclm_i_dn14 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p1023))));
        var_pclm_i_rv = 0.0;

        let assign7080_e10753: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard69 = assign7080_e10753;
        var_guard69_rv = 0.0;

        let (assign7090_e10765, assign7090_e10765_d_n0, assign7090_e10765_d_n2, assign7090_e10765_d_n3, assign7090_e10765_d_n4, assign7090_e10765_d_n5, assign7090_e10765_d_n6, assign7090_e10765_d_n7, assign7090_e10765_d_n8, assign7090_e10765_d_n9, assign7090_e10765_d_n10, assign7090_e10765_d_n11, assign7090_e10765_d_n13, assign7090_e10765_d_n14,) = {
    if (var_guard69 != 0.0) {
        let assign7090_e10758: f64 = (-p.p1024);
        let assign7090_e10760: f64 = (assign7090_e10758 * var_leff_ln);
        let assign7090_e10761: f64 = (assign7090_e10760).exp();
        let assign7090_e10762: f64 = (p.p1022 * assign7090_e10761);
        let assign7090_e10763: f64 = (var_pclmr_i + assign7090_e10762);
        (assign7090_e10763, (var_pclmr_i_dn0 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn0)))), (var_pclmr_i_dn2 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn2)))), (var_pclmr_i_dn3 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn3)))), (var_pclmr_i_dn4 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn4)))), (var_pclmr_i_dn5 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn5)))), (var_pclmr_i_dn6 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn6)))), (var_pclmr_i_dn7 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn7)))), (var_pclmr_i_dn8 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn8)))), (var_pclmr_i_dn9 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn9)))), (var_pclmr_i_dn10 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn10)))), (var_pclmr_i_dn11 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn11)))), (var_pclmr_i_dn13 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn13)))), (var_pclmr_i_dn14 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn14)))),)
    } else {
        (var_pclmr_i, var_pclmr_i_dn0, var_pclmr_i_dn2, var_pclmr_i_dn3, var_pclmr_i_dn4, var_pclmr_i_dn5, var_pclmr_i_dn6, var_pclmr_i_dn7, var_pclmr_i_dn8, var_pclmr_i_dn9, var_pclmr_i_dn10, var_pclmr_i_dn11, var_pclmr_i_dn13, var_pclmr_i_dn14,)
    }
};
        var_pclmr_i = assign7090_e10765;
        var_pclmr_i_dn0 = assign7090_e10765_d_n0;
        var_pclmr_i_dn2 = assign7090_e10765_d_n2;
        var_pclmr_i_dn3 = assign7090_e10765_d_n3;
        var_pclmr_i_dn4 = assign7090_e10765_d_n4;
        var_pclmr_i_dn5 = assign7090_e10765_d_n5;
        var_pclmr_i_dn6 = assign7090_e10765_d_n6;
        var_pclmr_i_dn7 = assign7090_e10765_d_n7;
        var_pclmr_i_dn8 = assign7090_e10765_d_n8;
        var_pclmr_i_dn9 = assign7090_e10765_d_n9;
        var_pclmr_i_dn10 = assign7090_e10765_d_n10;
        var_pclmr_i_dn11 = assign7090_e10765_d_n11;
        var_pclmr_i_dn13 = assign7090_e10765_d_n13;
        var_pclmr_i_dn14 = assign7090_e10765_d_n14;
        var_pclmr_i_rv = 0.0;

        let assign7100_e10769: f64 = (-p.p445);
        let assign7100_e10771: f64 = (assign7100_e10769 * var_leff_ln);
        let assign7100_e10772: f64 = (assign7100_e10771).exp();
        let assign7100_e10773: f64 = (p.p444 * assign7100_e10772);
        let assign7100_e10774: f64 = (var_mexp_i + assign7100_e10773);
        var_mexp_i = assign7100_e10774;
        var_mexp_i_dn0 = (var_mexp_i_dn0 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn0))));
        var_mexp_i_dn2 = (var_mexp_i_dn2 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn2))));
        var_mexp_i_dn3 = (var_mexp_i_dn3 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn3))));
        var_mexp_i_dn4 = (var_mexp_i_dn4 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn4))));
        var_mexp_i_dn5 = (var_mexp_i_dn5 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn5))));
        var_mexp_i_dn6 = (var_mexp_i_dn6 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn6))));
        var_mexp_i_dn7 = (var_mexp_i_dn7 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn7))));
        var_mexp_i_dn8 = (var_mexp_i_dn8 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn8))));
        var_mexp_i_dn9 = (var_mexp_i_dn9 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn9))));
        var_mexp_i_dn10 = (var_mexp_i_dn10 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn10))));
        var_mexp_i_dn11 = (var_mexp_i_dn11 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn11))));
        var_mexp_i_dn13 = (var_mexp_i_dn13 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn13))));
        var_mexp_i_dn14 = (var_mexp_i_dn14 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn14))));
        var_mexp_i_rv = 0.0;

        let assign7110_e10777: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard70 = assign7110_e10777;
        var_guard70_rv = 0.0;

        let (assign7120_e10789, assign7120_e10789_d_n0, assign7120_e10789_d_n2, assign7120_e10789_d_n3, assign7120_e10789_d_n4, assign7120_e10789_d_n5, assign7120_e10789_d_n6, assign7120_e10789_d_n7, assign7120_e10789_d_n8, assign7120_e10789_d_n9, assign7120_e10789_d_n10, assign7120_e10789_d_n11, assign7120_e10789_d_n13, assign7120_e10789_d_n14,) = {
    if (var_guard70 != 0.0) {
        let assign7120_e10782: f64 = (-p.p447);
        let assign7120_e10784: f64 = (assign7120_e10782 * var_leff_ln);
        let assign7120_e10785: f64 = (assign7120_e10784).exp();
        let assign7120_e10786: f64 = (p.p446 * assign7120_e10785);
        let assign7120_e10787: f64 = (var_mexpr_i + assign7120_e10786);
        (assign7120_e10787, (var_mexpr_i_dn0 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn0)))), (var_mexpr_i_dn2 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn2)))), (var_mexpr_i_dn3 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn3)))), (var_mexpr_i_dn4 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn4)))), (var_mexpr_i_dn5 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn5)))), (var_mexpr_i_dn6 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn6)))), (var_mexpr_i_dn7 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn7)))), (var_mexpr_i_dn8 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn8)))), (var_mexpr_i_dn9 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn9)))), (var_mexpr_i_dn10 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn10)))), (var_mexpr_i_dn11 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn11)))), (var_mexpr_i_dn13 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn13)))), (var_mexpr_i_dn14 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * var_leff_ln_dn14)))),)
    } else {
        (var_mexpr_i, var_mexpr_i_dn0, var_mexpr_i_dn2, var_mexpr_i_dn3, var_mexpr_i_dn4, var_mexpr_i_dn5, var_mexpr_i_dn6, var_mexpr_i_dn7, var_mexpr_i_dn8, var_mexpr_i_dn9, var_mexpr_i_dn10, var_mexpr_i_dn11, var_mexpr_i_dn13, var_mexpr_i_dn14,)
    }
};
        var_mexpr_i = assign7120_e10789;
        var_mexpr_i_dn0 = assign7120_e10789_d_n0;
        var_mexpr_i_dn2 = assign7120_e10789_d_n2;
        var_mexpr_i_dn3 = assign7120_e10789_d_n3;
        var_mexpr_i_dn4 = assign7120_e10789_d_n4;
        var_mexpr_i_dn5 = assign7120_e10789_d_n5;
        var_mexpr_i_dn6 = assign7120_e10789_d_n6;
        var_mexpr_i_dn7 = assign7120_e10789_d_n7;
        var_mexpr_i_dn8 = assign7120_e10789_d_n8;
        var_mexpr_i_dn9 = assign7120_e10789_d_n9;
        var_mexpr_i_dn10 = assign7120_e10789_d_n10;
        var_mexpr_i_dn11 = assign7120_e10789_d_n11;
        var_mexpr_i_dn13 = assign7120_e10789_d_n13;
        var_mexpr_i_dn14 = assign7120_e10789_d_n14;
        var_mexpr_i_rv = 0.0;

        let assign7130_e10793: f64 = (-var_leff_1);
        let assign7130_e10795: f64 = (assign7130_e10793 / p.p449);
        let assign7130_e10796: f64 = { let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7130_e10797: f64 = (p.p448 * assign7130_e10796);
        let assign7130_e10798: f64 = (var_ptwg_i + assign7130_e10797);
        var_ptwg_i = assign7130_e10798;
        var_ptwg_i_dn0 = (var_ptwg_i_dn0 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p449))));
        var_ptwg_i_dn2 = (var_ptwg_i_dn2 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p449))));
        var_ptwg_i_dn3 = (var_ptwg_i_dn3 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p449))));
        var_ptwg_i_dn4 = (var_ptwg_i_dn4 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p449))));
        var_ptwg_i_dn5 = (var_ptwg_i_dn5 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p449))));
        var_ptwg_i_dn6 = (var_ptwg_i_dn6 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p449))));
        var_ptwg_i_dn7 = (var_ptwg_i_dn7 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p449))));
        var_ptwg_i_dn8 = (var_ptwg_i_dn8 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p449))));
        var_ptwg_i_dn9 = (var_ptwg_i_dn9 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p449))));
        var_ptwg_i_dn10 = (var_ptwg_i_dn10 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p449))));
        var_ptwg_i_dn11 = (var_ptwg_i_dn11 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p449))));
        var_ptwg_i_dn13 = (var_ptwg_i_dn13 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p449))));
        var_ptwg_i_dn14 = (var_ptwg_i_dn14 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p449))));
        var_ptwg_i_rv = 0.0;

        let assign7140_e10801: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard71 = assign7140_e10801;
        var_guard71_rv = 0.0;

        let (assign7150_e10813, assign7150_e10813_d_n0, assign7150_e10813_d_n2, assign7150_e10813_d_n3, assign7150_e10813_d_n4, assign7150_e10813_d_n5, assign7150_e10813_d_n6, assign7150_e10813_d_n7, assign7150_e10813_d_n8, assign7150_e10813_d_n9, assign7150_e10813_d_n10, assign7150_e10813_d_n11, assign7150_e10813_d_n13, assign7150_e10813_d_n14,) = {
    if (var_guard71 != 0.0) {
        let assign7150_e10806: f64 = (-var_leff_1);
        let assign7150_e10808: f64 = (assign7150_e10806 / p.p449);
        let assign7150_e10809: f64 = { let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7150_e10810: f64 = (p.p448 * assign7150_e10809);
        let assign7150_e10811: f64 = (var_ptwgr_i + assign7150_e10810);
        (assign7150_e10811, (var_ptwgr_i_dn0 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p449)))), (var_ptwgr_i_dn2 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p449)))), (var_ptwgr_i_dn3 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p449)))), (var_ptwgr_i_dn4 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p449)))), (var_ptwgr_i_dn5 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p449)))), (var_ptwgr_i_dn6 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p449)))), (var_ptwgr_i_dn7 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p449)))), (var_ptwgr_i_dn8 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p449)))), (var_ptwgr_i_dn9 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p449)))), (var_ptwgr_i_dn10 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p449)))), (var_ptwgr_i_dn11 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p449)))), (var_ptwgr_i_dn13 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p449)))), (var_ptwgr_i_dn14 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p449)))),)
    } else {
        (var_ptwgr_i, var_ptwgr_i_dn0, var_ptwgr_i_dn2, var_ptwgr_i_dn3, var_ptwgr_i_dn4, var_ptwgr_i_dn5, var_ptwgr_i_dn6, var_ptwgr_i_dn7, var_ptwgr_i_dn8, var_ptwgr_i_dn9, var_ptwgr_i_dn10, var_ptwgr_i_dn11, var_ptwgr_i_dn13, var_ptwgr_i_dn14,)
    }
};
        var_ptwgr_i = assign7150_e10813;
        var_ptwgr_i_dn0 = assign7150_e10813_d_n0;
        var_ptwgr_i_dn2 = assign7150_e10813_d_n2;
        var_ptwgr_i_dn3 = assign7150_e10813_d_n3;
        var_ptwgr_i_dn4 = assign7150_e10813_d_n4;
        var_ptwgr_i_dn5 = assign7150_e10813_d_n5;
        var_ptwgr_i_dn6 = assign7150_e10813_d_n6;
        var_ptwgr_i_dn7 = assign7150_e10813_d_n7;
        var_ptwgr_i_dn8 = assign7150_e10813_d_n8;
        var_ptwgr_i_dn9 = assign7150_e10813_d_n9;
        var_ptwgr_i_dn10 = assign7150_e10813_d_n10;
        var_ptwgr_i_dn11 = assign7150_e10813_d_n11;
        var_ptwgr_i_dn13 = assign7150_e10813_d_n13;
        var_ptwgr_i_dn14 = assign7150_e10813_d_n14;
        var_ptwgr_i_rv = 0.0;

        let assign7160_e10817: f64 = (-var_leff_1);
        let assign7160_e10819: f64 = (assign7160_e10817 / p.p431);
        let assign7160_e10820: f64 = { let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7160_e10821: f64 = (p.p430 * assign7160_e10820);
        let assign7160_e10822: f64 = (var_vsat_i + assign7160_e10821);
        var_vsat_i = assign7160_e10822;
        var_vsat_i_dn0 = (var_vsat_i_dn0 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p431))));
        var_vsat_i_dn2 = (var_vsat_i_dn2 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p431))));
        var_vsat_i_dn3 = (var_vsat_i_dn3 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p431))));
        var_vsat_i_dn4 = (var_vsat_i_dn4 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p431))));
        var_vsat_i_dn5 = (var_vsat_i_dn5 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p431))));
        var_vsat_i_dn6 = (var_vsat_i_dn6 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p431))));
        var_vsat_i_dn7 = (var_vsat_i_dn7 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p431))));
        var_vsat_i_dn8 = (var_vsat_i_dn8 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p431))));
        var_vsat_i_dn9 = (var_vsat_i_dn9 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p431))));
        var_vsat_i_dn10 = (var_vsat_i_dn10 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p431))));
        var_vsat_i_dn11 = (var_vsat_i_dn11 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p431))));
        var_vsat_i_dn13 = (var_vsat_i_dn13 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p431))));
        var_vsat_i_dn14 = (var_vsat_i_dn14 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p431))));
        var_vsat_i_rv = 0.0;

        let assign7170_e10826: f64 = (-var_leff_1);
        let assign7170_e10828: f64 = (assign7170_e10826 / p.p437);
        let assign7170_e10829: f64 = { let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7170_e10830: f64 = (p.p436 * assign7170_e10829);
        let assign7170_e10831: f64 = (var_vsat1_i + assign7170_e10830);
        var_vsat1_i = assign7170_e10831;
        var_vsat1_i_dn0 = (var_vsat1_i_dn0 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p437))));
        var_vsat1_i_dn2 = (var_vsat1_i_dn2 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p437))));
        var_vsat1_i_dn3 = (var_vsat1_i_dn3 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p437))));
        var_vsat1_i_dn4 = (var_vsat1_i_dn4 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p437))));
        var_vsat1_i_dn5 = (var_vsat1_i_dn5 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p437))));
        var_vsat1_i_dn6 = (var_vsat1_i_dn6 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p437))));
        var_vsat1_i_dn7 = (var_vsat1_i_dn7 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p437))));
        var_vsat1_i_dn8 = (var_vsat1_i_dn8 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p437))));
        var_vsat1_i_dn9 = (var_vsat1_i_dn9 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p437))));
        var_vsat1_i_dn10 = (var_vsat1_i_dn10 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p437))));
        var_vsat1_i_dn11 = (var_vsat1_i_dn11 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p437))));
        var_vsat1_i_dn13 = (var_vsat1_i_dn13 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p437))));
        var_vsat1_i_dn14 = (var_vsat1_i_dn14 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p437))));
        var_vsat1_i_rv = 0.0;

        let assign7180_e10834: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard72 = assign7180_e10834;
        var_guard72_rv = 0.0;

        let (assign7190_e10846, assign7190_e10846_d_n0, assign7190_e10846_d_n2, assign7190_e10846_d_n3, assign7190_e10846_d_n4, assign7190_e10846_d_n5, assign7190_e10846_d_n6, assign7190_e10846_d_n7, assign7190_e10846_d_n8, assign7190_e10846_d_n9, assign7190_e10846_d_n10, assign7190_e10846_d_n11, assign7190_e10846_d_n13, assign7190_e10846_d_n14,) = {
    if (var_guard72 != 0.0) {
        let assign7190_e10839: f64 = (-var_leff_1);
        let assign7190_e10841: f64 = (assign7190_e10839 / p.p437);
        let assign7190_e10842: f64 = { let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7190_e10843: f64 = (p.p436 * assign7190_e10842);
        let assign7190_e10844: f64 = (var_vsat1r_i + assign7190_e10843);
        (assign7190_e10844, (var_vsat1r_i_dn0 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p437)))), (var_vsat1r_i_dn2 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p437)))), (var_vsat1r_i_dn3 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p437)))), (var_vsat1r_i_dn4 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p437)))), (var_vsat1r_i_dn5 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p437)))), (var_vsat1r_i_dn6 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p437)))), (var_vsat1r_i_dn7 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p437)))), (var_vsat1r_i_dn8 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p437)))), (var_vsat1r_i_dn9 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p437)))), (var_vsat1r_i_dn10 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p437)))), (var_vsat1r_i_dn11 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p437)))), (var_vsat1r_i_dn13 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p437)))), (var_vsat1r_i_dn14 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p437)))),)
    } else {
        (var_vsat1r_i, var_vsat1r_i_dn0, var_vsat1r_i_dn2, var_vsat1r_i_dn3, var_vsat1r_i_dn4, var_vsat1r_i_dn5, var_vsat1r_i_dn6, var_vsat1r_i_dn7, var_vsat1r_i_dn8, var_vsat1r_i_dn9, var_vsat1r_i_dn10, var_vsat1r_i_dn11, var_vsat1r_i_dn13, var_vsat1r_i_dn14,)
    }
};
        var_vsat1r_i = assign7190_e10846;
        var_vsat1r_i_dn0 = assign7190_e10846_d_n0;
        var_vsat1r_i_dn2 = assign7190_e10846_d_n2;
        var_vsat1r_i_dn3 = assign7190_e10846_d_n3;
        var_vsat1r_i_dn4 = assign7190_e10846_d_n4;
        var_vsat1r_i_dn5 = assign7190_e10846_d_n5;
        var_vsat1r_i_dn6 = assign7190_e10846_d_n6;
        var_vsat1r_i_dn7 = assign7190_e10846_d_n7;
        var_vsat1r_i_dn8 = assign7190_e10846_d_n8;
        var_vsat1r_i_dn9 = assign7190_e10846_d_n9;
        var_vsat1r_i_dn10 = assign7190_e10846_d_n10;
        var_vsat1r_i_dn11 = assign7190_e10846_d_n11;
        var_vsat1r_i_dn13 = assign7190_e10846_d_n13;
        var_vsat1r_i_dn14 = assign7190_e10846_d_n14;
        var_vsat1r_i_rv = 0.0;

        let assign7200_e10850: f64 = (-var_leff_1);
        let assign7200_e10852: f64 = (assign7200_e10850 / p.p439);
        let assign7200_e10853: f64 = { let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7200_e10854: f64 = (p.p438 * assign7200_e10853);
        let assign7200_e10855: f64 = (var_psat_i + assign7200_e10854);
        var_psat_i = assign7200_e10855;
        var_psat_i_dn0 = (var_psat_i_dn0 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p439))));
        var_psat_i_dn2 = (var_psat_i_dn2 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p439))));
        var_psat_i_dn3 = (var_psat_i_dn3 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p439))));
        var_psat_i_dn4 = (var_psat_i_dn4 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p439))));
        var_psat_i_dn5 = (var_psat_i_dn5 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p439))));
        var_psat_i_dn6 = (var_psat_i_dn6 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p439))));
        var_psat_i_dn7 = (var_psat_i_dn7 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p439))));
        var_psat_i_dn8 = (var_psat_i_dn8 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p439))));
        var_psat_i_dn9 = (var_psat_i_dn9 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p439))));
        var_psat_i_dn10 = (var_psat_i_dn10 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p439))));
        var_psat_i_dn11 = (var_psat_i_dn11 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p439))));
        var_psat_i_dn13 = (var_psat_i_dn13 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p439))));
        var_psat_i_dn14 = (var_psat_i_dn14 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p439))));
        var_psat_i_rv = 0.0;

        let assign7210_e10859: f64 = (-var_leffcv_1);
        let assign7210_e10861: f64 = (assign7210_e10859 / p.p443);
        let assign7210_e10862: f64 = { let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7210_e10863: f64 = (p.p442 * assign7210_e10862);
        let assign7210_e10864: f64 = (var_psatcv_i + assign7210_e10863);
        var_psatcv_i = assign7210_e10864;
        var_psatcv_i_dn0 = (var_psatcv_i_dn0 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn0) / p.p443))));
        var_psatcv_i_dn2 = (var_psatcv_i_dn2 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn2) / p.p443))));
        var_psatcv_i_dn3 = (var_psatcv_i_dn3 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn3) / p.p443))));
        var_psatcv_i_dn4 = (var_psatcv_i_dn4 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn4) / p.p443))));
        var_psatcv_i_dn5 = (var_psatcv_i_dn5 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn5) / p.p443))));
        var_psatcv_i_dn6 = (var_psatcv_i_dn6 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn6) / p.p443))));
        var_psatcv_i_dn7 = (var_psatcv_i_dn7 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn7) / p.p443))));
        var_psatcv_i_dn8 = (var_psatcv_i_dn8 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn8) / p.p443))));
        var_psatcv_i_dn9 = (var_psatcv_i_dn9 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn9) / p.p443))));
        var_psatcv_i_dn10 = (var_psatcv_i_dn10 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn10) / p.p443))));
        var_psatcv_i_dn11 = (var_psatcv_i_dn11 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn11) / p.p443))));
        var_psatcv_i_dn13 = (var_psatcv_i_dn13 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn13) / p.p443))));
        var_psatcv_i_dn14 = (var_psatcv_i_dn14 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn14) / p.p443))));
        var_psatcv_i_rv = 0.0;

        let assign7220_e10868: f64 = (-var_leffcv_1);
        let assign7220_e10870: f64 = (assign7220_e10868 / p.p441);
        let assign7220_e10871: f64 = { let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7220_e10872: f64 = (p.p440 * assign7220_e10871);
        let assign7220_e10873: f64 = (var_vsatcv_i + assign7220_e10872);
        var_vsatcv_i = assign7220_e10873;
        var_vsatcv_i_dn0 = (var_vsatcv_i_dn0 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn0) / p.p441))));
        var_vsatcv_i_dn2 = (var_vsatcv_i_dn2 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn2) / p.p441))));
        var_vsatcv_i_dn3 = (var_vsatcv_i_dn3 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn3) / p.p441))));
        var_vsatcv_i_dn4 = (var_vsatcv_i_dn4 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn4) / p.p441))));
        var_vsatcv_i_dn5 = (var_vsatcv_i_dn5 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn5) / p.p441))));
        var_vsatcv_i_dn6 = (var_vsatcv_i_dn6 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn6) / p.p441))));
        var_vsatcv_i_dn7 = (var_vsatcv_i_dn7 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn7) / p.p441))));
        var_vsatcv_i_dn8 = (var_vsatcv_i_dn8 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn8) / p.p441))));
        var_vsatcv_i_dn9 = (var_vsatcv_i_dn9 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn9) / p.p441))));
        var_vsatcv_i_dn10 = (var_vsatcv_i_dn10 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn10) / p.p441))));
        var_vsatcv_i_dn11 = (var_vsatcv_i_dn11 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn11) / p.p441))));
        var_vsatcv_i_dn13 = (var_vsatcv_i_dn13 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn13) / p.p441))));
        var_vsatcv_i_dn14 = (var_vsatcv_i_dn14 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leffcv_1_dn14) / p.p441))));
        var_vsatcv_i_rv = 0.0;

        let assign7230_e10877: f64 = (-var_leff_1);
        let assign7230_e10879: f64 = (assign7230_e10877 / p.p168);
        let assign7230_e10880: f64 = { let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7230_e10881: f64 = (p.p167 * assign7230_e10880);
        let assign7230_e10882: f64 = (var_dvtp0_i + assign7230_e10881);
        var_dvtp0_i = assign7230_e10882;
        var_dvtp0_i_dn0 = (var_dvtp0_i_dn0 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p168))));
        var_dvtp0_i_dn2 = (var_dvtp0_i_dn2 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p168))));
        var_dvtp0_i_dn3 = (var_dvtp0_i_dn3 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p168))));
        var_dvtp0_i_dn4 = (var_dvtp0_i_dn4 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p168))));
        var_dvtp0_i_dn5 = (var_dvtp0_i_dn5 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p168))));
        var_dvtp0_i_dn6 = (var_dvtp0_i_dn6 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p168))));
        var_dvtp0_i_dn7 = (var_dvtp0_i_dn7 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p168))));
        var_dvtp0_i_dn8 = (var_dvtp0_i_dn8 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p168))));
        var_dvtp0_i_dn9 = (var_dvtp0_i_dn9 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p168))));
        var_dvtp0_i_dn10 = (var_dvtp0_i_dn10 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p168))));
        var_dvtp0_i_dn11 = (var_dvtp0_i_dn11 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p168))));
        var_dvtp0_i_dn13 = (var_dvtp0_i_dn13 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p168))));
        var_dvtp0_i_dn14 = (var_dvtp0_i_dn14 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p168))));
        var_dvtp0_i_rv = 0.0;

        let assign7240_e10886: f64 = (-var_leff_1);
        let assign7240_e10888: f64 = (assign7240_e10886 / p.p170);
        let assign7240_e10889: f64 = { let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7240_e10890: f64 = (p.p169 * assign7240_e10889);
        let assign7240_e10891: f64 = (var_dvtp1_i + assign7240_e10890);
        var_dvtp1_i = assign7240_e10891;
        var_dvtp1_i_dn0 = (var_dvtp1_i_dn0 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p170))));
        var_dvtp1_i_dn2 = (var_dvtp1_i_dn2 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p170))));
        var_dvtp1_i_dn3 = (var_dvtp1_i_dn3 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p170))));
        var_dvtp1_i_dn4 = (var_dvtp1_i_dn4 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p170))));
        var_dvtp1_i_dn5 = (var_dvtp1_i_dn5 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p170))));
        var_dvtp1_i_dn6 = (var_dvtp1_i_dn6 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p170))));
        var_dvtp1_i_dn7 = (var_dvtp1_i_dn7 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p170))));
        var_dvtp1_i_dn8 = (var_dvtp1_i_dn8 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p170))));
        var_dvtp1_i_dn9 = (var_dvtp1_i_dn9 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p170))));
        var_dvtp1_i_dn10 = (var_dvtp1_i_dn10 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p170))));
        var_dvtp1_i_dn11 = (var_dvtp1_i_dn11 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p170))));
        var_dvtp1_i_dn13 = (var_dvtp1_i_dn13 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p170))));
        var_dvtp1_i_dn14 = (var_dvtp1_i_dn14 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p170))));
        var_dvtp1_i_rv = 0.0;

        let assign7250_e10898: f64 = if ((var_qmtcencv_i > 0.0) || (var_qmtcencva_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard73 = assign7250_e10898;
        var_guard73_rv = 0.0;

        let (assign7260_e10914,) = {
    if (var_guard73 != 0.0) {
        let assign7260_e10904: f64 = (2.0 * var_ach);
        let assign7260_e10906: f64 = (assign7260_e10904 / var_weff_ufcm);
        let assign7260_e10907: f64 = (-assign7260_e10906);
        let assign7260_e10909: f64 = (assign7260_e10907 / p.p399);
        let assign7260_e10910: f64 = { let limited_exp_arg = assign7260_e10909; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7260_e10911: f64 = (p.p398 * assign7260_e10910);
        let assign7260_e10912: f64 = (1.0 + assign7260_e10911);
        (assign7260_e10912,)
    } else {
        (var_mtcen,)
    }
};
        var_mtcen = assign7260_e10914;
        var_mtcen_rv = 0.0;

        let (assign7270_e10924,) = {
    if (var_guard73 != 0.0) {
        let assign7270_e10918: f64 = (2.0 * var_ach);
        let assign7270_e10920: f64 = (assign7270_e10918 / var_weff_ufcm);
        let assign7270_e10922: f64 = (assign7270_e10920 * var_mtcen);
        (assign7270_e10922,)
    } else {
        (var_tcen0,)
    }
};
        var_tcen0 = assign7270_e10924;
        var_tcen0_rv = 0.0;

        let assign7300_e10933: f64 = if var_qsref_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard76 = assign7300_e10933;
        var_guard76_rv = 0.0;

        let (assign7310_e10937,) = {
    if (var_guard76 != 0.0) {
        (0.05,)
    } else {
        (var_qsref_i,)
    }
};
        var_qsref_i = assign7310_e10937;
        var_qsref_i_rv = 0.0;

        let assign7380_e10970: f64 = if var_phig_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard81 = assign7380_e10970;
        var_guard81_rv = 0.0;

        let (assign7390_e10974, assign7390_e10974_d_n0, assign7390_e10974_d_n2, assign7390_e10974_d_n3, assign7390_e10974_d_n4, assign7390_e10974_d_n5, assign7390_e10974_d_n6, assign7390_e10974_d_n7, assign7390_e10974_d_n8, assign7390_e10974_d_n9, assign7390_e10974_d_n10, assign7390_e10974_d_n11, assign7390_e10974_d_n13, assign7390_e10974_d_n14,) = {
    if (var_guard81 != 0.0) {
        (4.61, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_phig_i, var_phig_i_dn0, var_phig_i_dn2, var_phig_i_dn3, var_phig_i_dn4, var_phig_i_dn5, var_phig_i_dn6, var_phig_i_dn7, var_phig_i_dn8, var_phig_i_dn9, var_phig_i_dn10, var_phig_i_dn11, var_phig_i_dn13, var_phig_i_dn14,)
    }
};
        var_phig_i = assign7390_e10974;
        var_phig_i_dn0 = assign7390_e10974_d_n0;
        var_phig_i_dn2 = assign7390_e10974_d_n2;
        var_phig_i_dn3 = assign7390_e10974_d_n3;
        var_phig_i_dn4 = assign7390_e10974_d_n4;
        var_phig_i_dn5 = assign7390_e10974_d_n5;
        var_phig_i_dn6 = assign7390_e10974_d_n6;
        var_phig_i_dn7 = assign7390_e10974_d_n7;
        var_phig_i_dn8 = assign7390_e10974_d_n8;
        var_phig_i_dn9 = assign7390_e10974_d_n9;
        var_phig_i_dn10 = assign7390_e10974_d_n10;
        var_phig_i_dn11 = assign7390_e10974_d_n11;
        var_phig_i_dn13 = assign7390_e10974_d_n13;
        var_phig_i_dn14 = assign7390_e10974_d_n14;
        var_phig_i_rv = 0.0;

        let assign7400_e10977: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard82 = assign7400_e10977;
        var_guard82_rv = 0.0;

        let assign7410_e10980: f64 = if var_k1_i < 1e-6 { 1.0 } else { 0.0 };
        var_guard83 = assign7410_e10980;
        var_guard83_rv = 0.0;

        let (assign7420_e10986,) = {
    if ((var_guard82 != 0.0) && (var_guard83 != 0.0)) {
        (1e-6,)
    } else {
        (var_k1_i,)
    }
};
        var_k1_i = assign7420_e10986;
        var_k1_i_rv = 0.0;

        let assign7430_e10989: f64 = if var_sprt_i < 0.0 { 1.0 } else { 0.0 };
        var_guard84 = assign7430_e10989;
        var_guard84_rv = 0.0;

        let (assign7440_e10993,) = {
    if (var_guard84 != 0.0) {
        (0.01,)
    } else {
        (var_sprt_i,)
    }
};
        var_sprt_i = assign7440_e10993;
        var_sprt_i_rv = 0.0;

        let assign7450_e10996: f64 = if var_qsref_i < 0.0 { 1.0 } else { 0.0 };
        var_guard85 = assign7450_e10996;
        var_guard85_rv = 0.0;

        let (assign7460_e11000,) = {
    if (var_guard85 != 0.0) {
        (0.05,)
    } else {
        (var_qsref_i,)
    }
};
        var_qsref_i = assign7460_e11000;
        var_qsref_i_rv = 0.0;

        let assign7470_e11003: f64 = if var_noia2_i < 0.0 { 1.0 } else { 0.0 };
        var_guard86 = assign7470_e11003;
        var_guard86_rv = 0.0;

        let (assign7480_e11007,) = {
    if (var_guard86 != 0.0) {
        (p.p1682,)
    } else {
        (var_noia2_i,)
    }
};
        var_noia2_i = assign7480_e11007;
        var_noia2_i_rv = 0.0;

        let assign7490_e11010: f64 = if var_mpower_i < 0.0 { 1.0 } else { 0.0 };
        var_guard87 = assign7490_e11010;
        var_guard87_rv = 0.0;

        let (assign7500_e11014,) = {
    if (var_guard87 != 0.0) {
        (1.2,)
    } else {
        (var_mpower_i,)
    }
};
        var_mpower_i = assign7500_e11014;
        var_mpower_i_rv = 0.0;

        let assign7510_e11017: f64 = if var_covs_i < 0.0 { 1.0 } else { 0.0 };
        var_guard88 = assign7510_e11017;
        var_guard88_rv = 0.0;

        *var_dvtp0_i_slot = var_dvtp0_i;
        *var_dvtp0_i_dn0_slot = var_dvtp0_i_dn0;
        *var_dvtp0_i_dn10_slot = var_dvtp0_i_dn10;
        *var_dvtp0_i_dn11_slot = var_dvtp0_i_dn11;
        *var_dvtp0_i_dn13_slot = var_dvtp0_i_dn13;
        *var_dvtp0_i_dn14_slot = var_dvtp0_i_dn14;
        *var_dvtp0_i_dn2_slot = var_dvtp0_i_dn2;
        *var_dvtp0_i_dn3_slot = var_dvtp0_i_dn3;
        *var_dvtp0_i_dn4_slot = var_dvtp0_i_dn4;
        *var_dvtp0_i_dn5_slot = var_dvtp0_i_dn5;
        *var_dvtp0_i_dn6_slot = var_dvtp0_i_dn6;
        *var_dvtp0_i_dn7_slot = var_dvtp0_i_dn7;
        *var_dvtp0_i_dn8_slot = var_dvtp0_i_dn8;
        *var_dvtp0_i_dn9_slot = var_dvtp0_i_dn9;
        *var_dvtp0_i_rv_slot = var_dvtp0_i_rv;
        *var_dvtp1_i_slot = var_dvtp1_i;
        *var_dvtp1_i_dn0_slot = var_dvtp1_i_dn0;
        *var_dvtp1_i_dn10_slot = var_dvtp1_i_dn10;
        *var_dvtp1_i_dn11_slot = var_dvtp1_i_dn11;
        *var_dvtp1_i_dn13_slot = var_dvtp1_i_dn13;
        *var_dvtp1_i_dn14_slot = var_dvtp1_i_dn14;
        *var_dvtp1_i_dn2_slot = var_dvtp1_i_dn2;
        *var_dvtp1_i_dn3_slot = var_dvtp1_i_dn3;
        *var_dvtp1_i_dn4_slot = var_dvtp1_i_dn4;
        *var_dvtp1_i_dn5_slot = var_dvtp1_i_dn5;
        *var_dvtp1_i_dn6_slot = var_dvtp1_i_dn6;
        *var_dvtp1_i_dn7_slot = var_dvtp1_i_dn7;
        *var_dvtp1_i_dn8_slot = var_dvtp1_i_dn8;
        *var_dvtp1_i_dn9_slot = var_dvtp1_i_dn9;
        *var_dvtp1_i_rv_slot = var_dvtp1_i_rv;
        *var_guard69_slot = var_guard69;
        *var_guard69_rv_slot = var_guard69_rv;
        *var_guard70_slot = var_guard70;
        *var_guard70_rv_slot = var_guard70_rv;
        *var_guard71_slot = var_guard71;
        *var_guard71_rv_slot = var_guard71_rv;
        *var_guard72_slot = var_guard72;
        *var_guard72_rv_slot = var_guard72_rv;
        *var_guard73_slot = var_guard73;
        *var_guard73_rv_slot = var_guard73_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_guard81_slot = var_guard81;
        *var_guard81_rv_slot = var_guard81_rv;
        *var_guard82_slot = var_guard82;
        *var_guard82_rv_slot = var_guard82_rv;
        *var_guard83_slot = var_guard83;
        *var_guard83_rv_slot = var_guard83_rv;
        *var_guard84_slot = var_guard84;
        *var_guard84_rv_slot = var_guard84_rv;
        *var_guard85_slot = var_guard85;
        *var_guard85_rv_slot = var_guard85_rv;
        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
        *var_guard87_slot = var_guard87;
        *var_guard87_rv_slot = var_guard87_rv;
        *var_guard88_slot = var_guard88;
        *var_guard88_rv_slot = var_guard88_rv;
        *var_k1_i_slot = var_k1_i;
        *var_k1_i_rv_slot = var_k1_i_rv;
        *var_mexp_i_slot = var_mexp_i;
        *var_mexp_i_dn0_slot = var_mexp_i_dn0;
        *var_mexp_i_dn10_slot = var_mexp_i_dn10;
        *var_mexp_i_dn11_slot = var_mexp_i_dn11;
        *var_mexp_i_dn13_slot = var_mexp_i_dn13;
        *var_mexp_i_dn14_slot = var_mexp_i_dn14;
        *var_mexp_i_dn2_slot = var_mexp_i_dn2;
        *var_mexp_i_dn3_slot = var_mexp_i_dn3;
        *var_mexp_i_dn4_slot = var_mexp_i_dn4;
        *var_mexp_i_dn5_slot = var_mexp_i_dn5;
        *var_mexp_i_dn6_slot = var_mexp_i_dn6;
        *var_mexp_i_dn7_slot = var_mexp_i_dn7;
        *var_mexp_i_dn8_slot = var_mexp_i_dn8;
        *var_mexp_i_dn9_slot = var_mexp_i_dn9;
        *var_mexp_i_rv_slot = var_mexp_i_rv;
        *var_mexpr_i_slot = var_mexpr_i;
        *var_mexpr_i_dn0_slot = var_mexpr_i_dn0;
        *var_mexpr_i_dn10_slot = var_mexpr_i_dn10;
        *var_mexpr_i_dn11_slot = var_mexpr_i_dn11;
        *var_mexpr_i_dn13_slot = var_mexpr_i_dn13;
        *var_mexpr_i_dn14_slot = var_mexpr_i_dn14;
        *var_mexpr_i_dn2_slot = var_mexpr_i_dn2;
        *var_mexpr_i_dn3_slot = var_mexpr_i_dn3;
        *var_mexpr_i_dn4_slot = var_mexpr_i_dn4;
        *var_mexpr_i_dn5_slot = var_mexpr_i_dn5;
        *var_mexpr_i_dn6_slot = var_mexpr_i_dn6;
        *var_mexpr_i_dn7_slot = var_mexpr_i_dn7;
        *var_mexpr_i_dn8_slot = var_mexpr_i_dn8;
        *var_mexpr_i_dn9_slot = var_mexpr_i_dn9;
        *var_mexpr_i_rv_slot = var_mexpr_i_rv;
        *var_mpower_i_slot = var_mpower_i;
        *var_mpower_i_rv_slot = var_mpower_i_rv;
        *var_mtcen_slot = var_mtcen;
        *var_mtcen_rv_slot = var_mtcen_rv;
        *var_noia2_i_slot = var_noia2_i;
        *var_noia2_i_rv_slot = var_noia2_i_rv;
        *var_pclm_i_slot = var_pclm_i;
        *var_pclm_i_dn0_slot = var_pclm_i_dn0;
        *var_pclm_i_dn10_slot = var_pclm_i_dn10;
        *var_pclm_i_dn11_slot = var_pclm_i_dn11;
        *var_pclm_i_dn13_slot = var_pclm_i_dn13;
        *var_pclm_i_dn14_slot = var_pclm_i_dn14;
        *var_pclm_i_dn2_slot = var_pclm_i_dn2;
        *var_pclm_i_dn3_slot = var_pclm_i_dn3;
        *var_pclm_i_dn4_slot = var_pclm_i_dn4;
        *var_pclm_i_dn5_slot = var_pclm_i_dn5;
        *var_pclm_i_dn6_slot = var_pclm_i_dn6;
        *var_pclm_i_dn7_slot = var_pclm_i_dn7;
        *var_pclm_i_dn8_slot = var_pclm_i_dn8;
        *var_pclm_i_dn9_slot = var_pclm_i_dn9;
        *var_pclm_i_rv_slot = var_pclm_i_rv;
        *var_pclmr_i_slot = var_pclmr_i;
        *var_pclmr_i_dn0_slot = var_pclmr_i_dn0;
        *var_pclmr_i_dn10_slot = var_pclmr_i_dn10;
        *var_pclmr_i_dn11_slot = var_pclmr_i_dn11;
        *var_pclmr_i_dn13_slot = var_pclmr_i_dn13;
        *var_pclmr_i_dn14_slot = var_pclmr_i_dn14;
        *var_pclmr_i_dn2_slot = var_pclmr_i_dn2;
        *var_pclmr_i_dn3_slot = var_pclmr_i_dn3;
        *var_pclmr_i_dn4_slot = var_pclmr_i_dn4;
        *var_pclmr_i_dn5_slot = var_pclmr_i_dn5;
        *var_pclmr_i_dn6_slot = var_pclmr_i_dn6;
        *var_pclmr_i_dn7_slot = var_pclmr_i_dn7;
        *var_pclmr_i_dn8_slot = var_pclmr_i_dn8;
        *var_pclmr_i_dn9_slot = var_pclmr_i_dn9;
        *var_pclmr_i_rv_slot = var_pclmr_i_rv;
        *var_phig_i_slot = var_phig_i;
        *var_phig_i_dn0_slot = var_phig_i_dn0;
        *var_phig_i_dn10_slot = var_phig_i_dn10;
        *var_phig_i_dn11_slot = var_phig_i_dn11;
        *var_phig_i_dn13_slot = var_phig_i_dn13;
        *var_phig_i_dn14_slot = var_phig_i_dn14;
        *var_phig_i_dn2_slot = var_phig_i_dn2;
        *var_phig_i_dn3_slot = var_phig_i_dn3;
        *var_phig_i_dn4_slot = var_phig_i_dn4;
        *var_phig_i_dn5_slot = var_phig_i_dn5;
        *var_phig_i_dn6_slot = var_phig_i_dn6;
        *var_phig_i_dn7_slot = var_phig_i_dn7;
        *var_phig_i_dn8_slot = var_phig_i_dn8;
        *var_phig_i_dn9_slot = var_phig_i_dn9;
        *var_phig_i_rv_slot = var_phig_i_rv;
        *var_psat_i_slot = var_psat_i;
        *var_psat_i_dn0_slot = var_psat_i_dn0;
        *var_psat_i_dn10_slot = var_psat_i_dn10;
        *var_psat_i_dn11_slot = var_psat_i_dn11;
        *var_psat_i_dn13_slot = var_psat_i_dn13;
        *var_psat_i_dn14_slot = var_psat_i_dn14;
        *var_psat_i_dn2_slot = var_psat_i_dn2;
        *var_psat_i_dn3_slot = var_psat_i_dn3;
        *var_psat_i_dn4_slot = var_psat_i_dn4;
        *var_psat_i_dn5_slot = var_psat_i_dn5;
        *var_psat_i_dn6_slot = var_psat_i_dn6;
        *var_psat_i_dn7_slot = var_psat_i_dn7;
        *var_psat_i_dn8_slot = var_psat_i_dn8;
        *var_psat_i_dn9_slot = var_psat_i_dn9;
        *var_psat_i_rv_slot = var_psat_i_rv;
        *var_psatcv_i_slot = var_psatcv_i;
        *var_psatcv_i_dn0_slot = var_psatcv_i_dn0;
        *var_psatcv_i_dn10_slot = var_psatcv_i_dn10;
        *var_psatcv_i_dn11_slot = var_psatcv_i_dn11;
        *var_psatcv_i_dn13_slot = var_psatcv_i_dn13;
        *var_psatcv_i_dn14_slot = var_psatcv_i_dn14;
        *var_psatcv_i_dn2_slot = var_psatcv_i_dn2;
        *var_psatcv_i_dn3_slot = var_psatcv_i_dn3;
        *var_psatcv_i_dn4_slot = var_psatcv_i_dn4;
        *var_psatcv_i_dn5_slot = var_psatcv_i_dn5;
        *var_psatcv_i_dn6_slot = var_psatcv_i_dn6;
        *var_psatcv_i_dn7_slot = var_psatcv_i_dn7;
        *var_psatcv_i_dn8_slot = var_psatcv_i_dn8;
        *var_psatcv_i_dn9_slot = var_psatcv_i_dn9;
        *var_psatcv_i_rv_slot = var_psatcv_i_rv;
        *var_ptwg_i_slot = var_ptwg_i;
        *var_ptwg_i_dn0_slot = var_ptwg_i_dn0;
        *var_ptwg_i_dn10_slot = var_ptwg_i_dn10;
        *var_ptwg_i_dn11_slot = var_ptwg_i_dn11;
        *var_ptwg_i_dn13_slot = var_ptwg_i_dn13;
        *var_ptwg_i_dn14_slot = var_ptwg_i_dn14;
        *var_ptwg_i_dn2_slot = var_ptwg_i_dn2;
        *var_ptwg_i_dn3_slot = var_ptwg_i_dn3;
        *var_ptwg_i_dn4_slot = var_ptwg_i_dn4;
        *var_ptwg_i_dn5_slot = var_ptwg_i_dn5;
        *var_ptwg_i_dn6_slot = var_ptwg_i_dn6;
        *var_ptwg_i_dn7_slot = var_ptwg_i_dn7;
        *var_ptwg_i_dn8_slot = var_ptwg_i_dn8;
        *var_ptwg_i_dn9_slot = var_ptwg_i_dn9;
        *var_ptwg_i_rv_slot = var_ptwg_i_rv;
        *var_ptwgr_i_slot = var_ptwgr_i;
        *var_ptwgr_i_dn0_slot = var_ptwgr_i_dn0;
        *var_ptwgr_i_dn10_slot = var_ptwgr_i_dn10;
        *var_ptwgr_i_dn11_slot = var_ptwgr_i_dn11;
        *var_ptwgr_i_dn13_slot = var_ptwgr_i_dn13;
        *var_ptwgr_i_dn14_slot = var_ptwgr_i_dn14;
        *var_ptwgr_i_dn2_slot = var_ptwgr_i_dn2;
        *var_ptwgr_i_dn3_slot = var_ptwgr_i_dn3;
        *var_ptwgr_i_dn4_slot = var_ptwgr_i_dn4;
        *var_ptwgr_i_dn5_slot = var_ptwgr_i_dn5;
        *var_ptwgr_i_dn6_slot = var_ptwgr_i_dn6;
        *var_ptwgr_i_dn7_slot = var_ptwgr_i_dn7;
        *var_ptwgr_i_dn8_slot = var_ptwgr_i_dn8;
        *var_ptwgr_i_dn9_slot = var_ptwgr_i_dn9;
        *var_ptwgr_i_rv_slot = var_ptwgr_i_rv;
        *var_qsref_i_slot = var_qsref_i;
        *var_qsref_i_rv_slot = var_qsref_i_rv;
        *var_sprt_i_slot = var_sprt_i;
        *var_sprt_i_rv_slot = var_sprt_i_rv;
        *var_tcen0_slot = var_tcen0;
        *var_tcen0_rv_slot = var_tcen0_rv;
        *var_vsat1_i_slot = var_vsat1_i;
        *var_vsat1_i_dn0_slot = var_vsat1_i_dn0;
        *var_vsat1_i_dn10_slot = var_vsat1_i_dn10;
        *var_vsat1_i_dn11_slot = var_vsat1_i_dn11;
        *var_vsat1_i_dn13_slot = var_vsat1_i_dn13;
        *var_vsat1_i_dn14_slot = var_vsat1_i_dn14;
        *var_vsat1_i_dn2_slot = var_vsat1_i_dn2;
        *var_vsat1_i_dn3_slot = var_vsat1_i_dn3;
        *var_vsat1_i_dn4_slot = var_vsat1_i_dn4;
        *var_vsat1_i_dn5_slot = var_vsat1_i_dn5;
        *var_vsat1_i_dn6_slot = var_vsat1_i_dn6;
        *var_vsat1_i_dn7_slot = var_vsat1_i_dn7;
        *var_vsat1_i_dn8_slot = var_vsat1_i_dn8;
        *var_vsat1_i_dn9_slot = var_vsat1_i_dn9;
        *var_vsat1_i_rv_slot = var_vsat1_i_rv;
        *var_vsat1r_i_slot = var_vsat1r_i;
        *var_vsat1r_i_dn0_slot = var_vsat1r_i_dn0;
        *var_vsat1r_i_dn10_slot = var_vsat1r_i_dn10;
        *var_vsat1r_i_dn11_slot = var_vsat1r_i_dn11;
        *var_vsat1r_i_dn13_slot = var_vsat1r_i_dn13;
        *var_vsat1r_i_dn14_slot = var_vsat1r_i_dn14;
        *var_vsat1r_i_dn2_slot = var_vsat1r_i_dn2;
        *var_vsat1r_i_dn3_slot = var_vsat1r_i_dn3;
        *var_vsat1r_i_dn4_slot = var_vsat1r_i_dn4;
        *var_vsat1r_i_dn5_slot = var_vsat1r_i_dn5;
        *var_vsat1r_i_dn6_slot = var_vsat1r_i_dn6;
        *var_vsat1r_i_dn7_slot = var_vsat1r_i_dn7;
        *var_vsat1r_i_dn8_slot = var_vsat1r_i_dn8;
        *var_vsat1r_i_dn9_slot = var_vsat1r_i_dn9;
        *var_vsat1r_i_rv_slot = var_vsat1r_i_rv;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsat_i_dn0_slot = var_vsat_i_dn0;
        *var_vsat_i_dn10_slot = var_vsat_i_dn10;
        *var_vsat_i_dn11_slot = var_vsat_i_dn11;
        *var_vsat_i_dn13_slot = var_vsat_i_dn13;
        *var_vsat_i_dn14_slot = var_vsat_i_dn14;
        *var_vsat_i_dn2_slot = var_vsat_i_dn2;
        *var_vsat_i_dn3_slot = var_vsat_i_dn3;
        *var_vsat_i_dn4_slot = var_vsat_i_dn4;
        *var_vsat_i_dn5_slot = var_vsat_i_dn5;
        *var_vsat_i_dn6_slot = var_vsat_i_dn6;
        *var_vsat_i_dn7_slot = var_vsat_i_dn7;
        *var_vsat_i_dn8_slot = var_vsat_i_dn8;
        *var_vsat_i_dn9_slot = var_vsat_i_dn9;
        *var_vsat_i_rv_slot = var_vsat_i_rv;
        *var_vsatcv_i_slot = var_vsatcv_i;
        *var_vsatcv_i_dn0_slot = var_vsatcv_i_dn0;
        *var_vsatcv_i_dn10_slot = var_vsatcv_i_dn10;
        *var_vsatcv_i_dn11_slot = var_vsatcv_i_dn11;
        *var_vsatcv_i_dn13_slot = var_vsatcv_i_dn13;
        *var_vsatcv_i_dn14_slot = var_vsatcv_i_dn14;
        *var_vsatcv_i_dn2_slot = var_vsatcv_i_dn2;
        *var_vsatcv_i_dn3_slot = var_vsatcv_i_dn3;
        *var_vsatcv_i_dn4_slot = var_vsatcv_i_dn4;
        *var_vsatcv_i_dn5_slot = var_vsatcv_i_dn5;
        *var_vsatcv_i_dn6_slot = var_vsatcv_i_dn6;
        *var_vsatcv_i_dn7_slot = var_vsatcv_i_dn7;
        *var_vsatcv_i_dn8_slot = var_vsatcv_i_dn8;
        *var_vsatcv_i_dn9_slot = var_vsatcv_i_dn9;
        *var_vsatcv_i_rv_slot = var_vsatcv_i_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_guard88: f64,
        var_leff_1: f64,
        var_rsw_i: f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn0_slot: &mut f64,
        var_covd_i_dn10_slot: &mut f64,
        var_covd_i_dn11_slot: &mut f64,
        var_covd_i_dn13_slot: &mut f64,
        var_covd_i_dn14_slot: &mut f64,
        var_covd_i_dn2_slot: &mut f64,
        var_covd_i_dn3_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn5_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covd_i_rv_slot: &mut f64,
        var_covs_i_slot: &mut f64,
        var_covs_i_dn0_slot: &mut f64,
        var_covs_i_dn10_slot: &mut f64,
        var_covs_i_dn11_slot: &mut f64,
        var_covs_i_dn13_slot: &mut f64,
        var_covs_i_dn14_slot: &mut f64,
        var_covs_i_dn2_slot: &mut f64,
        var_covs_i_dn3_slot: &mut f64,
        var_covs_i_dn4_slot: &mut f64,
        var_covs_i_dn5_slot: &mut f64,
        var_covs_i_dn6_slot: &mut f64,
        var_covs_i_dn7_slot: &mut f64,
        var_covs_i_dn8_slot: &mut f64,
        var_covs_i_dn9_slot: &mut f64,
        var_covs_i_rv_slot: &mut f64,
        var_dsub_i_slot: &mut f64,
        var_dsub_i_rv_slot: &mut f64,
        var_dvt1_i_slot: &mut f64,
        var_dvt1_i_rv_slot: &mut f64,
        var_dvt1ss_i_slot: &mut f64,
        var_dvt1ss_i_rv_slot: &mut f64,
        var_eta0_i_slot: &mut f64,
        var_eta0_i_dn0_slot: &mut f64,
        var_eta0_i_dn10_slot: &mut f64,
        var_eta0_i_dn11_slot: &mut f64,
        var_eta0_i_dn13_slot: &mut f64,
        var_eta0_i_dn14_slot: &mut f64,
        var_eta0_i_dn2_slot: &mut f64,
        var_eta0_i_dn3_slot: &mut f64,
        var_eta0_i_dn4_slot: &mut f64,
        var_eta0_i_dn5_slot: &mut f64,
        var_eta0_i_dn6_slot: &mut f64,
        var_eta0_i_dn7_slot: &mut f64,
        var_eta0_i_dn8_slot: &mut f64,
        var_eta0_i_dn9_slot: &mut f64,
        var_eta0_i_rv_slot: &mut f64,
        var_eta0r_i_slot: &mut f64,
        var_eta0r_i_rv_slot: &mut f64,
        var_etamob_i_slot: &mut f64,
        var_etamob_i_rv_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eu_i_dn0_slot: &mut f64,
        var_eu_i_dn10_slot: &mut f64,
        var_eu_i_dn11_slot: &mut f64,
        var_eu_i_dn13_slot: &mut f64,
        var_eu_i_dn14_slot: &mut f64,
        var_eu_i_dn2_slot: &mut f64,
        var_eu_i_dn3_slot: &mut f64,
        var_eu_i_dn4_slot: &mut f64,
        var_eu_i_dn5_slot: &mut f64,
        var_eu_i_dn6_slot: &mut f64,
        var_eu_i_dn7_slot: &mut f64,
        var_eu_i_dn8_slot: &mut f64,
        var_eu_i_dn9_slot: &mut f64,
        var_eu_i_rv_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard100_rv_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard101_rv_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard102_rv_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard103_rv_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard112_rv_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard114_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard89_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard93_rv_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard94_rv_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard98_rv_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_guard99_rv_slot: &mut f64,
        var_k0si_i_slot: &mut f64,
        var_k0si_i_rv_slot: &mut f64,
        var_k2si_i_slot: &mut f64,
        var_k2si_i_rv_slot: &mut f64,
        var_lpe0_i_slot: &mut f64,
        var_lpe0_i_rv_slot: &mut f64,
        var_phibe_i_slot: &mut f64,
        var_phibe_i_rv_slot: &mut f64,
        var_psat_i_slot: &mut f64,
        var_psat_i_dn0_slot: &mut f64,
        var_psat_i_dn10_slot: &mut f64,
        var_psat_i_dn11_slot: &mut f64,
        var_psat_i_dn13_slot: &mut f64,
        var_psat_i_dn14_slot: &mut f64,
        var_psat_i_dn2_slot: &mut f64,
        var_psat_i_dn3_slot: &mut f64,
        var_psat_i_dn4_slot: &mut f64,
        var_psat_i_dn5_slot: &mut f64,
        var_psat_i_dn6_slot: &mut f64,
        var_psat_i_dn7_slot: &mut f64,
        var_psat_i_dn8_slot: &mut f64,
        var_psat_i_dn9_slot: &mut f64,
        var_psat_i_rv_slot: &mut f64,
        var_psatcv_i_slot: &mut f64,
        var_psatcv_i_dn0_slot: &mut f64,
        var_psatcv_i_dn10_slot: &mut f64,
        var_psatcv_i_dn11_slot: &mut f64,
        var_psatcv_i_dn13_slot: &mut f64,
        var_psatcv_i_dn14_slot: &mut f64,
        var_psatcv_i_dn2_slot: &mut f64,
        var_psatcv_i_dn3_slot: &mut f64,
        var_psatcv_i_dn4_slot: &mut f64,
        var_psatcv_i_dn5_slot: &mut f64,
        var_psatcv_i_dn6_slot: &mut f64,
        var_psatcv_i_dn7_slot: &mut f64,
        var_psatcv_i_dn8_slot: &mut f64,
        var_psatcv_i_dn9_slot: &mut f64,
        var_psatcv_i_rv_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdsw_i_dn0_slot: &mut f64,
        var_rdsw_i_dn10_slot: &mut f64,
        var_rdsw_i_dn11_slot: &mut f64,
        var_rdsw_i_dn13_slot: &mut f64,
        var_rdsw_i_dn14_slot: &mut f64,
        var_rdsw_i_dn2_slot: &mut f64,
        var_rdsw_i_dn3_slot: &mut f64,
        var_rdsw_i_dn4_slot: &mut f64,
        var_rdsw_i_dn5_slot: &mut f64,
        var_rdsw_i_dn6_slot: &mut f64,
        var_rdsw_i_dn7_slot: &mut f64,
        var_rdsw_i_dn8_slot: &mut f64,
        var_rdsw_i_dn9_slot: &mut f64,
        var_rdsw_i_rv_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_dn0_slot: &mut f64,
        var_u0_i_dn10_slot: &mut f64,
        var_u0_i_dn11_slot: &mut f64,
        var_u0_i_dn13_slot: &mut f64,
        var_u0_i_dn14_slot: &mut f64,
        var_u0_i_dn2_slot: &mut f64,
        var_u0_i_dn3_slot: &mut f64,
        var_u0_i_dn4_slot: &mut f64,
        var_u0_i_dn5_slot: &mut f64,
        var_u0_i_dn6_slot: &mut f64,
        var_u0_i_dn7_slot: &mut f64,
        var_u0_i_dn8_slot: &mut f64,
        var_u0_i_dn9_slot: &mut f64,
        var_u0_i_rv_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_ua_i_dn0_slot: &mut f64,
        var_ua_i_dn10_slot: &mut f64,
        var_ua_i_dn11_slot: &mut f64,
        var_ua_i_dn13_slot: &mut f64,
        var_ua_i_dn14_slot: &mut f64,
        var_ua_i_dn2_slot: &mut f64,
        var_ua_i_dn3_slot: &mut f64,
        var_ua_i_dn4_slot: &mut f64,
        var_ua_i_dn5_slot: &mut f64,
        var_ua_i_dn6_slot: &mut f64,
        var_ua_i_dn7_slot: &mut f64,
        var_ua_i_dn8_slot: &mut f64,
        var_ua_i_dn9_slot: &mut f64,
        var_ua_i_rv_slot: &mut f64,
        var_ucs_i_slot: &mut f64,
        var_ucs_i_rv_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_ud_i_dn0_slot: &mut f64,
        var_ud_i_dn10_slot: &mut f64,
        var_ud_i_dn11_slot: &mut f64,
        var_ud_i_dn13_slot: &mut f64,
        var_ud_i_dn14_slot: &mut f64,
        var_ud_i_dn2_slot: &mut f64,
        var_ud_i_dn3_slot: &mut f64,
        var_ud_i_dn4_slot: &mut f64,
        var_ud_i_dn5_slot: &mut f64,
        var_ud_i_dn6_slot: &mut f64,
        var_ud_i_dn7_slot: &mut f64,
        var_ud_i_dn8_slot: &mut f64,
        var_ud_i_dn9_slot: &mut f64,
        var_ud_i_rv_slot: &mut f64,
        var_vsat1_i_slot: &mut f64,
        var_vsat1_i_dn0_slot: &mut f64,
        var_vsat1_i_dn10_slot: &mut f64,
        var_vsat1_i_dn11_slot: &mut f64,
        var_vsat1_i_dn13_slot: &mut f64,
        var_vsat1_i_dn14_slot: &mut f64,
        var_vsat1_i_dn2_slot: &mut f64,
        var_vsat1_i_dn3_slot: &mut f64,
        var_vsat1_i_dn4_slot: &mut f64,
        var_vsat1_i_dn5_slot: &mut f64,
        var_vsat1_i_dn6_slot: &mut f64,
        var_vsat1_i_dn7_slot: &mut f64,
        var_vsat1_i_dn8_slot: &mut f64,
        var_vsat1_i_dn9_slot: &mut f64,
        var_vsat1_i_rv_slot: &mut f64,
        var_vsat1r_i_slot: &mut f64,
        var_vsat1r_i_dn0_slot: &mut f64,
        var_vsat1r_i_dn10_slot: &mut f64,
        var_vsat1r_i_dn11_slot: &mut f64,
        var_vsat1r_i_dn13_slot: &mut f64,
        var_vsat1r_i_dn14_slot: &mut f64,
        var_vsat1r_i_dn2_slot: &mut f64,
        var_vsat1r_i_dn3_slot: &mut f64,
        var_vsat1r_i_dn4_slot: &mut f64,
        var_vsat1r_i_dn5_slot: &mut f64,
        var_vsat1r_i_dn6_slot: &mut f64,
        var_vsat1r_i_dn7_slot: &mut f64,
        var_vsat1r_i_dn8_slot: &mut f64,
        var_vsat1r_i_dn9_slot: &mut f64,
        var_vsat1r_i_rv_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsat_i_dn0_slot: &mut f64,
        var_vsat_i_dn10_slot: &mut f64,
        var_vsat_i_dn11_slot: &mut f64,
        var_vsat_i_dn13_slot: &mut f64,
        var_vsat_i_dn14_slot: &mut f64,
        var_vsat_i_dn2_slot: &mut f64,
        var_vsat_i_dn3_slot: &mut f64,
        var_vsat_i_dn4_slot: &mut f64,
        var_vsat_i_dn5_slot: &mut f64,
        var_vsat_i_dn6_slot: &mut f64,
        var_vsat_i_dn7_slot: &mut f64,
        var_vsat_i_dn8_slot: &mut f64,
        var_vsat_i_dn9_slot: &mut f64,
        var_vsat_i_rv_slot: &mut f64,
    ) {
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn0: f64 = *var_covd_i_dn0_slot;
        let mut var_covd_i_dn10: f64 = *var_covd_i_dn10_slot;
        let mut var_covd_i_dn11: f64 = *var_covd_i_dn11_slot;
        let mut var_covd_i_dn13: f64 = *var_covd_i_dn13_slot;
        let mut var_covd_i_dn14: f64 = *var_covd_i_dn14_slot;
        let mut var_covd_i_dn2: f64 = *var_covd_i_dn2_slot;
        let mut var_covd_i_dn3: f64 = *var_covd_i_dn3_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn5: f64 = *var_covd_i_dn5_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covd_i_rv: f64 = *var_covd_i_rv_slot;
        let mut var_covs_i: f64 = *var_covs_i_slot;
        let mut var_covs_i_dn0: f64 = *var_covs_i_dn0_slot;
        let mut var_covs_i_dn10: f64 = *var_covs_i_dn10_slot;
        let mut var_covs_i_dn11: f64 = *var_covs_i_dn11_slot;
        let mut var_covs_i_dn13: f64 = *var_covs_i_dn13_slot;
        let mut var_covs_i_dn14: f64 = *var_covs_i_dn14_slot;
        let mut var_covs_i_dn2: f64 = *var_covs_i_dn2_slot;
        let mut var_covs_i_dn3: f64 = *var_covs_i_dn3_slot;
        let mut var_covs_i_dn4: f64 = *var_covs_i_dn4_slot;
        let mut var_covs_i_dn5: f64 = *var_covs_i_dn5_slot;
        let mut var_covs_i_dn6: f64 = *var_covs_i_dn6_slot;
        let mut var_covs_i_dn7: f64 = *var_covs_i_dn7_slot;
        let mut var_covs_i_dn8: f64 = *var_covs_i_dn8_slot;
        let mut var_covs_i_dn9: f64 = *var_covs_i_dn9_slot;
        let mut var_covs_i_rv: f64 = *var_covs_i_rv_slot;
        let mut var_dsub_i: f64 = *var_dsub_i_slot;
        let mut var_dsub_i_rv: f64 = *var_dsub_i_rv_slot;
        let mut var_dvt1_i: f64 = *var_dvt1_i_slot;
        let mut var_dvt1_i_rv: f64 = *var_dvt1_i_rv_slot;
        let mut var_dvt1ss_i: f64 = *var_dvt1ss_i_slot;
        let mut var_dvt1ss_i_rv: f64 = *var_dvt1ss_i_rv_slot;
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta0_i_dn0: f64 = *var_eta0_i_dn0_slot;
        let mut var_eta0_i_dn10: f64 = *var_eta0_i_dn10_slot;
        let mut var_eta0_i_dn11: f64 = *var_eta0_i_dn11_slot;
        let mut var_eta0_i_dn13: f64 = *var_eta0_i_dn13_slot;
        let mut var_eta0_i_dn14: f64 = *var_eta0_i_dn14_slot;
        let mut var_eta0_i_dn2: f64 = *var_eta0_i_dn2_slot;
        let mut var_eta0_i_dn3: f64 = *var_eta0_i_dn3_slot;
        let mut var_eta0_i_dn4: f64 = *var_eta0_i_dn4_slot;
        let mut var_eta0_i_dn5: f64 = *var_eta0_i_dn5_slot;
        let mut var_eta0_i_dn6: f64 = *var_eta0_i_dn6_slot;
        let mut var_eta0_i_dn7: f64 = *var_eta0_i_dn7_slot;
        let mut var_eta0_i_dn8: f64 = *var_eta0_i_dn8_slot;
        let mut var_eta0_i_dn9: f64 = *var_eta0_i_dn9_slot;
        let mut var_eta0_i_rv: f64 = *var_eta0_i_rv_slot;
        let mut var_eta0r_i: f64 = *var_eta0r_i_slot;
        let mut var_eta0r_i_rv: f64 = *var_eta0r_i_rv_slot;
        let mut var_etamob_i: f64 = *var_etamob_i_slot;
        let mut var_etamob_i_rv: f64 = *var_etamob_i_rv_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eu_i_dn0: f64 = *var_eu_i_dn0_slot;
        let mut var_eu_i_dn10: f64 = *var_eu_i_dn10_slot;
        let mut var_eu_i_dn11: f64 = *var_eu_i_dn11_slot;
        let mut var_eu_i_dn13: f64 = *var_eu_i_dn13_slot;
        let mut var_eu_i_dn14: f64 = *var_eu_i_dn14_slot;
        let mut var_eu_i_dn2: f64 = *var_eu_i_dn2_slot;
        let mut var_eu_i_dn3: f64 = *var_eu_i_dn3_slot;
        let mut var_eu_i_dn4: f64 = *var_eu_i_dn4_slot;
        let mut var_eu_i_dn5: f64 = *var_eu_i_dn5_slot;
        let mut var_eu_i_dn6: f64 = *var_eu_i_dn6_slot;
        let mut var_eu_i_dn7: f64 = *var_eu_i_dn7_slot;
        let mut var_eu_i_dn8: f64 = *var_eu_i_dn8_slot;
        let mut var_eu_i_dn9: f64 = *var_eu_i_dn9_slot;
        let mut var_eu_i_rv: f64 = *var_eu_i_rv_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard100_rv: f64 = *var_guard100_rv_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard101_rv: f64 = *var_guard101_rv_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard102_rv: f64 = *var_guard102_rv_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard103_rv: f64 = *var_guard103_rv_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard112_rv: f64 = *var_guard112_rv_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard114_rv: f64 = *var_guard114_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard89_rv: f64 = *var_guard89_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard93_rv: f64 = *var_guard93_rv_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard94_rv: f64 = *var_guard94_rv_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard98_rv: f64 = *var_guard98_rv_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_guard99_rv: f64 = *var_guard99_rv_slot;
        let mut var_k0si_i: f64 = *var_k0si_i_slot;
        let mut var_k0si_i_rv: f64 = *var_k0si_i_rv_slot;
        let mut var_k2si_i: f64 = *var_k2si_i_slot;
        let mut var_k2si_i_rv: f64 = *var_k2si_i_rv_slot;
        let mut var_lpe0_i: f64 = *var_lpe0_i_slot;
        let mut var_lpe0_i_rv: f64 = *var_lpe0_i_rv_slot;
        let mut var_phibe_i: f64 = *var_phibe_i_slot;
        let mut var_phibe_i_rv: f64 = *var_phibe_i_rv_slot;
        let mut var_psat_i: f64 = *var_psat_i_slot;
        let mut var_psat_i_dn0: f64 = *var_psat_i_dn0_slot;
        let mut var_psat_i_dn10: f64 = *var_psat_i_dn10_slot;
        let mut var_psat_i_dn11: f64 = *var_psat_i_dn11_slot;
        let mut var_psat_i_dn13: f64 = *var_psat_i_dn13_slot;
        let mut var_psat_i_dn14: f64 = *var_psat_i_dn14_slot;
        let mut var_psat_i_dn2: f64 = *var_psat_i_dn2_slot;
        let mut var_psat_i_dn3: f64 = *var_psat_i_dn3_slot;
        let mut var_psat_i_dn4: f64 = *var_psat_i_dn4_slot;
        let mut var_psat_i_dn5: f64 = *var_psat_i_dn5_slot;
        let mut var_psat_i_dn6: f64 = *var_psat_i_dn6_slot;
        let mut var_psat_i_dn7: f64 = *var_psat_i_dn7_slot;
        let mut var_psat_i_dn8: f64 = *var_psat_i_dn8_slot;
        let mut var_psat_i_dn9: f64 = *var_psat_i_dn9_slot;
        let mut var_psat_i_rv: f64 = *var_psat_i_rv_slot;
        let mut var_psatcv_i: f64 = *var_psatcv_i_slot;
        let mut var_psatcv_i_dn0: f64 = *var_psatcv_i_dn0_slot;
        let mut var_psatcv_i_dn10: f64 = *var_psatcv_i_dn10_slot;
        let mut var_psatcv_i_dn11: f64 = *var_psatcv_i_dn11_slot;
        let mut var_psatcv_i_dn13: f64 = *var_psatcv_i_dn13_slot;
        let mut var_psatcv_i_dn14: f64 = *var_psatcv_i_dn14_slot;
        let mut var_psatcv_i_dn2: f64 = *var_psatcv_i_dn2_slot;
        let mut var_psatcv_i_dn3: f64 = *var_psatcv_i_dn3_slot;
        let mut var_psatcv_i_dn4: f64 = *var_psatcv_i_dn4_slot;
        let mut var_psatcv_i_dn5: f64 = *var_psatcv_i_dn5_slot;
        let mut var_psatcv_i_dn6: f64 = *var_psatcv_i_dn6_slot;
        let mut var_psatcv_i_dn7: f64 = *var_psatcv_i_dn7_slot;
        let mut var_psatcv_i_dn8: f64 = *var_psatcv_i_dn8_slot;
        let mut var_psatcv_i_dn9: f64 = *var_psatcv_i_dn9_slot;
        let mut var_psatcv_i_rv: f64 = *var_psatcv_i_rv_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdsw_i_dn0: f64 = *var_rdsw_i_dn0_slot;
        let mut var_rdsw_i_dn10: f64 = *var_rdsw_i_dn10_slot;
        let mut var_rdsw_i_dn11: f64 = *var_rdsw_i_dn11_slot;
        let mut var_rdsw_i_dn13: f64 = *var_rdsw_i_dn13_slot;
        let mut var_rdsw_i_dn14: f64 = *var_rdsw_i_dn14_slot;
        let mut var_rdsw_i_dn2: f64 = *var_rdsw_i_dn2_slot;
        let mut var_rdsw_i_dn3: f64 = *var_rdsw_i_dn3_slot;
        let mut var_rdsw_i_dn4: f64 = *var_rdsw_i_dn4_slot;
        let mut var_rdsw_i_dn5: f64 = *var_rdsw_i_dn5_slot;
        let mut var_rdsw_i_dn6: f64 = *var_rdsw_i_dn6_slot;
        let mut var_rdsw_i_dn7: f64 = *var_rdsw_i_dn7_slot;
        let mut var_rdsw_i_dn8: f64 = *var_rdsw_i_dn8_slot;
        let mut var_rdsw_i_dn9: f64 = *var_rdsw_i_dn9_slot;
        let mut var_rdsw_i_rv: f64 = *var_rdsw_i_rv_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_dn0: f64 = *var_u0_i_dn0_slot;
        let mut var_u0_i_dn10: f64 = *var_u0_i_dn10_slot;
        let mut var_u0_i_dn11: f64 = *var_u0_i_dn11_slot;
        let mut var_u0_i_dn13: f64 = *var_u0_i_dn13_slot;
        let mut var_u0_i_dn14: f64 = *var_u0_i_dn14_slot;
        let mut var_u0_i_dn2: f64 = *var_u0_i_dn2_slot;
        let mut var_u0_i_dn3: f64 = *var_u0_i_dn3_slot;
        let mut var_u0_i_dn4: f64 = *var_u0_i_dn4_slot;
        let mut var_u0_i_dn5: f64 = *var_u0_i_dn5_slot;
        let mut var_u0_i_dn6: f64 = *var_u0_i_dn6_slot;
        let mut var_u0_i_dn7: f64 = *var_u0_i_dn7_slot;
        let mut var_u0_i_dn8: f64 = *var_u0_i_dn8_slot;
        let mut var_u0_i_dn9: f64 = *var_u0_i_dn9_slot;
        let mut var_u0_i_rv: f64 = *var_u0_i_rv_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_ua_i_dn0: f64 = *var_ua_i_dn0_slot;
        let mut var_ua_i_dn10: f64 = *var_ua_i_dn10_slot;
        let mut var_ua_i_dn11: f64 = *var_ua_i_dn11_slot;
        let mut var_ua_i_dn13: f64 = *var_ua_i_dn13_slot;
        let mut var_ua_i_dn14: f64 = *var_ua_i_dn14_slot;
        let mut var_ua_i_dn2: f64 = *var_ua_i_dn2_slot;
        let mut var_ua_i_dn3: f64 = *var_ua_i_dn3_slot;
        let mut var_ua_i_dn4: f64 = *var_ua_i_dn4_slot;
        let mut var_ua_i_dn5: f64 = *var_ua_i_dn5_slot;
        let mut var_ua_i_dn6: f64 = *var_ua_i_dn6_slot;
        let mut var_ua_i_dn7: f64 = *var_ua_i_dn7_slot;
        let mut var_ua_i_dn8: f64 = *var_ua_i_dn8_slot;
        let mut var_ua_i_dn9: f64 = *var_ua_i_dn9_slot;
        let mut var_ua_i_rv: f64 = *var_ua_i_rv_slot;
        let mut var_ucs_i: f64 = *var_ucs_i_slot;
        let mut var_ucs_i_rv: f64 = *var_ucs_i_rv_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_ud_i_dn0: f64 = *var_ud_i_dn0_slot;
        let mut var_ud_i_dn10: f64 = *var_ud_i_dn10_slot;
        let mut var_ud_i_dn11: f64 = *var_ud_i_dn11_slot;
        let mut var_ud_i_dn13: f64 = *var_ud_i_dn13_slot;
        let mut var_ud_i_dn14: f64 = *var_ud_i_dn14_slot;
        let mut var_ud_i_dn2: f64 = *var_ud_i_dn2_slot;
        let mut var_ud_i_dn3: f64 = *var_ud_i_dn3_slot;
        let mut var_ud_i_dn4: f64 = *var_ud_i_dn4_slot;
        let mut var_ud_i_dn5: f64 = *var_ud_i_dn5_slot;
        let mut var_ud_i_dn6: f64 = *var_ud_i_dn6_slot;
        let mut var_ud_i_dn7: f64 = *var_ud_i_dn7_slot;
        let mut var_ud_i_dn8: f64 = *var_ud_i_dn8_slot;
        let mut var_ud_i_dn9: f64 = *var_ud_i_dn9_slot;
        let mut var_ud_i_rv: f64 = *var_ud_i_rv_slot;
        let mut var_vsat1_i: f64 = *var_vsat1_i_slot;
        let mut var_vsat1_i_dn0: f64 = *var_vsat1_i_dn0_slot;
        let mut var_vsat1_i_dn10: f64 = *var_vsat1_i_dn10_slot;
        let mut var_vsat1_i_dn11: f64 = *var_vsat1_i_dn11_slot;
        let mut var_vsat1_i_dn13: f64 = *var_vsat1_i_dn13_slot;
        let mut var_vsat1_i_dn14: f64 = *var_vsat1_i_dn14_slot;
        let mut var_vsat1_i_dn2: f64 = *var_vsat1_i_dn2_slot;
        let mut var_vsat1_i_dn3: f64 = *var_vsat1_i_dn3_slot;
        let mut var_vsat1_i_dn4: f64 = *var_vsat1_i_dn4_slot;
        let mut var_vsat1_i_dn5: f64 = *var_vsat1_i_dn5_slot;
        let mut var_vsat1_i_dn6: f64 = *var_vsat1_i_dn6_slot;
        let mut var_vsat1_i_dn7: f64 = *var_vsat1_i_dn7_slot;
        let mut var_vsat1_i_dn8: f64 = *var_vsat1_i_dn8_slot;
        let mut var_vsat1_i_dn9: f64 = *var_vsat1_i_dn9_slot;
        let mut var_vsat1_i_rv: f64 = *var_vsat1_i_rv_slot;
        let mut var_vsat1r_i: f64 = *var_vsat1r_i_slot;
        let mut var_vsat1r_i_dn0: f64 = *var_vsat1r_i_dn0_slot;
        let mut var_vsat1r_i_dn10: f64 = *var_vsat1r_i_dn10_slot;
        let mut var_vsat1r_i_dn11: f64 = *var_vsat1r_i_dn11_slot;
        let mut var_vsat1r_i_dn13: f64 = *var_vsat1r_i_dn13_slot;
        let mut var_vsat1r_i_dn14: f64 = *var_vsat1r_i_dn14_slot;
        let mut var_vsat1r_i_dn2: f64 = *var_vsat1r_i_dn2_slot;
        let mut var_vsat1r_i_dn3: f64 = *var_vsat1r_i_dn3_slot;
        let mut var_vsat1r_i_dn4: f64 = *var_vsat1r_i_dn4_slot;
        let mut var_vsat1r_i_dn5: f64 = *var_vsat1r_i_dn5_slot;
        let mut var_vsat1r_i_dn6: f64 = *var_vsat1r_i_dn6_slot;
        let mut var_vsat1r_i_dn7: f64 = *var_vsat1r_i_dn7_slot;
        let mut var_vsat1r_i_dn8: f64 = *var_vsat1r_i_dn8_slot;
        let mut var_vsat1r_i_dn9: f64 = *var_vsat1r_i_dn9_slot;
        let mut var_vsat1r_i_rv: f64 = *var_vsat1r_i_rv_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsat_i_dn0: f64 = *var_vsat_i_dn0_slot;
        let mut var_vsat_i_dn10: f64 = *var_vsat_i_dn10_slot;
        let mut var_vsat_i_dn11: f64 = *var_vsat_i_dn11_slot;
        let mut var_vsat_i_dn13: f64 = *var_vsat_i_dn13_slot;
        let mut var_vsat_i_dn14: f64 = *var_vsat_i_dn14_slot;
        let mut var_vsat_i_dn2: f64 = *var_vsat_i_dn2_slot;
        let mut var_vsat_i_dn3: f64 = *var_vsat_i_dn3_slot;
        let mut var_vsat_i_dn4: f64 = *var_vsat_i_dn4_slot;
        let mut var_vsat_i_dn5: f64 = *var_vsat_i_dn5_slot;
        let mut var_vsat_i_dn6: f64 = *var_vsat_i_dn6_slot;
        let mut var_vsat_i_dn7: f64 = *var_vsat_i_dn7_slot;
        let mut var_vsat_i_dn8: f64 = *var_vsat_i_dn8_slot;
        let mut var_vsat_i_dn9: f64 = *var_vsat_i_dn9_slot;
        let mut var_vsat_i_rv: f64 = *var_vsat_i_rv_slot;

        let (assign7520_e11021, assign7520_e11021_d_n0, assign7520_e11021_d_n2, assign7520_e11021_d_n3, assign7520_e11021_d_n4, assign7520_e11021_d_n5, assign7520_e11021_d_n6, assign7520_e11021_d_n7, assign7520_e11021_d_n8, assign7520_e11021_d_n9, assign7520_e11021_d_n10, assign7520_e11021_d_n11, assign7520_e11021_d_n13, assign7520_e11021_d_n14,) = {
    if (var_guard88 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_covs_i, var_covs_i_dn0, var_covs_i_dn2, var_covs_i_dn3, var_covs_i_dn4, var_covs_i_dn5, var_covs_i_dn6, var_covs_i_dn7, var_covs_i_dn8, var_covs_i_dn9, var_covs_i_dn10, var_covs_i_dn11, var_covs_i_dn13, var_covs_i_dn14,)
    }
};
        var_covs_i = assign7520_e11021;
        var_covs_i_dn0 = assign7520_e11021_d_n0;
        var_covs_i_dn2 = assign7520_e11021_d_n2;
        var_covs_i_dn3 = assign7520_e11021_d_n3;
        var_covs_i_dn4 = assign7520_e11021_d_n4;
        var_covs_i_dn5 = assign7520_e11021_d_n5;
        var_covs_i_dn6 = assign7520_e11021_d_n6;
        var_covs_i_dn7 = assign7520_e11021_d_n7;
        var_covs_i_dn8 = assign7520_e11021_d_n8;
        var_covs_i_dn9 = assign7520_e11021_d_n9;
        var_covs_i_dn10 = assign7520_e11021_d_n10;
        var_covs_i_dn11 = assign7520_e11021_d_n11;
        var_covs_i_dn13 = assign7520_e11021_d_n13;
        var_covs_i_dn14 = assign7520_e11021_d_n14;
        var_covs_i_rv = 0.0;

        let assign7530_e11024: f64 = if var_covd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard89 = assign7530_e11024;
        var_guard89_rv = 0.0;

        let (assign7540_e11028, assign7540_e11028_d_n0, assign7540_e11028_d_n2, assign7540_e11028_d_n3, assign7540_e11028_d_n4, assign7540_e11028_d_n5, assign7540_e11028_d_n6, assign7540_e11028_d_n7, assign7540_e11028_d_n8, assign7540_e11028_d_n9, assign7540_e11028_d_n10, assign7540_e11028_d_n11, assign7540_e11028_d_n13, assign7540_e11028_d_n14,) = {
    if (var_guard89 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_covd_i, var_covd_i_dn0, var_covd_i_dn2, var_covd_i_dn3, var_covd_i_dn4, var_covd_i_dn5, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9, var_covd_i_dn10, var_covd_i_dn11, var_covd_i_dn13, var_covd_i_dn14,)
    }
};
        var_covd_i = assign7540_e11028;
        var_covd_i_dn0 = assign7540_e11028_d_n0;
        var_covd_i_dn2 = assign7540_e11028_d_n2;
        var_covd_i_dn3 = assign7540_e11028_d_n3;
        var_covd_i_dn4 = assign7540_e11028_d_n4;
        var_covd_i_dn5 = assign7540_e11028_d_n5;
        var_covd_i_dn6 = assign7540_e11028_d_n6;
        var_covd_i_dn7 = assign7540_e11028_d_n7;
        var_covd_i_dn8 = assign7540_e11028_d_n8;
        var_covd_i_dn9 = assign7540_e11028_d_n9;
        var_covd_i_dn10 = assign7540_e11028_d_n10;
        var_covd_i_dn11 = assign7540_e11028_d_n11;
        var_covd_i_dn13 = assign7540_e11028_d_n13;
        var_covd_i_dn14 = assign7540_e11028_d_n14;
        var_covd_i_rv = 0.0;

        let assign7550_e11031: f64 = if var_vsat_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard90 = assign7550_e11031;
        var_guard90_rv = 0.0;

        let (assign7560_e11035, assign7560_e11035_d_n0, assign7560_e11035_d_n2, assign7560_e11035_d_n3, assign7560_e11035_d_n4, assign7560_e11035_d_n5, assign7560_e11035_d_n6, assign7560_e11035_d_n7, assign7560_e11035_d_n8, assign7560_e11035_d_n9, assign7560_e11035_d_n10, assign7560_e11035_d_n11, assign7560_e11035_d_n13, assign7560_e11035_d_n14,) = {
    if (var_guard90 != 0.0) {
        (85000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat_i, var_vsat_i_dn0, var_vsat_i_dn2, var_vsat_i_dn3, var_vsat_i_dn4, var_vsat_i_dn5, var_vsat_i_dn6, var_vsat_i_dn7, var_vsat_i_dn8, var_vsat_i_dn9, var_vsat_i_dn10, var_vsat_i_dn11, var_vsat_i_dn13, var_vsat_i_dn14,)
    }
};
        var_vsat_i = assign7560_e11035;
        var_vsat_i_dn0 = assign7560_e11035_d_n0;
        var_vsat_i_dn2 = assign7560_e11035_d_n2;
        var_vsat_i_dn3 = assign7560_e11035_d_n3;
        var_vsat_i_dn4 = assign7560_e11035_d_n4;
        var_vsat_i_dn5 = assign7560_e11035_d_n5;
        var_vsat_i_dn6 = assign7560_e11035_d_n6;
        var_vsat_i_dn7 = assign7560_e11035_d_n7;
        var_vsat_i_dn8 = assign7560_e11035_d_n8;
        var_vsat_i_dn9 = assign7560_e11035_d_n9;
        var_vsat_i_dn10 = assign7560_e11035_d_n10;
        var_vsat_i_dn11 = assign7560_e11035_d_n11;
        var_vsat_i_dn13 = assign7560_e11035_d_n13;
        var_vsat_i_dn14 = assign7560_e11035_d_n14;
        var_vsat_i_rv = 0.0;

        let assign7570_e11038: f64 = if var_vsat1_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard91 = assign7570_e11038;
        var_guard91_rv = 0.0;

        let (assign7580_e11042, assign7580_e11042_d_n0, assign7580_e11042_d_n2, assign7580_e11042_d_n3, assign7580_e11042_d_n4, assign7580_e11042_d_n5, assign7580_e11042_d_n6, assign7580_e11042_d_n7, assign7580_e11042_d_n8, assign7580_e11042_d_n9, assign7580_e11042_d_n10, assign7580_e11042_d_n11, assign7580_e11042_d_n13, assign7580_e11042_d_n14,) = {
    if (var_guard91 != 0.0) {
        (85000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat1_i, var_vsat1_i_dn0, var_vsat1_i_dn2, var_vsat1_i_dn3, var_vsat1_i_dn4, var_vsat1_i_dn5, var_vsat1_i_dn6, var_vsat1_i_dn7, var_vsat1_i_dn8, var_vsat1_i_dn9, var_vsat1_i_dn10, var_vsat1_i_dn11, var_vsat1_i_dn13, var_vsat1_i_dn14,)
    }
};
        var_vsat1_i = assign7580_e11042;
        var_vsat1_i_dn0 = assign7580_e11042_d_n0;
        var_vsat1_i_dn2 = assign7580_e11042_d_n2;
        var_vsat1_i_dn3 = assign7580_e11042_d_n3;
        var_vsat1_i_dn4 = assign7580_e11042_d_n4;
        var_vsat1_i_dn5 = assign7580_e11042_d_n5;
        var_vsat1_i_dn6 = assign7580_e11042_d_n6;
        var_vsat1_i_dn7 = assign7580_e11042_d_n7;
        var_vsat1_i_dn8 = assign7580_e11042_d_n8;
        var_vsat1_i_dn9 = assign7580_e11042_d_n9;
        var_vsat1_i_dn10 = assign7580_e11042_d_n10;
        var_vsat1_i_dn11 = assign7580_e11042_d_n11;
        var_vsat1_i_dn13 = assign7580_e11042_d_n13;
        var_vsat1_i_dn14 = assign7580_e11042_d_n14;
        var_vsat1_i_rv = 0.0;

        let assign7590_e11049: f64 = if ((p.p66 != 0.0) && (var_vsat1r_i <= 0.0)) { 1.0 } else { 0.0 };
        var_guard92 = assign7590_e11049;
        var_guard92_rv = 0.0;

        let (assign7600_e11053, assign7600_e11053_d_n0, assign7600_e11053_d_n2, assign7600_e11053_d_n3, assign7600_e11053_d_n4, assign7600_e11053_d_n5, assign7600_e11053_d_n6, assign7600_e11053_d_n7, assign7600_e11053_d_n8, assign7600_e11053_d_n9, assign7600_e11053_d_n10, assign7600_e11053_d_n11, assign7600_e11053_d_n13, assign7600_e11053_d_n14,) = {
    if (var_guard92 != 0.0) {
        (85000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat1r_i, var_vsat1r_i_dn0, var_vsat1r_i_dn2, var_vsat1r_i_dn3, var_vsat1r_i_dn4, var_vsat1r_i_dn5, var_vsat1r_i_dn6, var_vsat1r_i_dn7, var_vsat1r_i_dn8, var_vsat1r_i_dn9, var_vsat1r_i_dn10, var_vsat1r_i_dn11, var_vsat1r_i_dn13, var_vsat1r_i_dn14,)
    }
};
        var_vsat1r_i = assign7600_e11053;
        var_vsat1r_i_dn0 = assign7600_e11053_d_n0;
        var_vsat1r_i_dn2 = assign7600_e11053_d_n2;
        var_vsat1r_i_dn3 = assign7600_e11053_d_n3;
        var_vsat1r_i_dn4 = assign7600_e11053_d_n4;
        var_vsat1r_i_dn5 = assign7600_e11053_d_n5;
        var_vsat1r_i_dn6 = assign7600_e11053_d_n6;
        var_vsat1r_i_dn7 = assign7600_e11053_d_n7;
        var_vsat1r_i_dn8 = assign7600_e11053_d_n8;
        var_vsat1r_i_dn9 = assign7600_e11053_d_n9;
        var_vsat1r_i_dn10 = assign7600_e11053_d_n10;
        var_vsat1r_i_dn11 = assign7600_e11053_d_n11;
        var_vsat1r_i_dn13 = assign7600_e11053_d_n13;
        var_vsat1r_i_dn14 = assign7600_e11053_d_n14;
        var_vsat1r_i_rv = 0.0;

        let assign7610_e11056: f64 = if var_dvt1_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard93 = assign7610_e11056;
        var_guard93_rv = 0.0;

        let (assign7620_e11060,) = {
    if (var_guard93 != 0.0) {
        (0.6,)
    } else {
        (var_dvt1_i,)
    }
};
        var_dvt1_i = assign7620_e11060;
        var_dvt1_i_rv = 0.0;

        let assign7630_e11063: f64 = if var_dvt1ss_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard94 = assign7630_e11063;
        var_guard94_rv = 0.0;

        let (assign7640_e11067,) = {
    if (var_guard94 != 0.0) {
        (0.6,)
    } else {
        (var_dvt1ss_i,)
    }
};
        var_dvt1ss_i = assign7640_e11067;
        var_dvt1ss_i_rv = 0.0;

        let assign7680_e11083: f64 = if var_dsub_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard98 = assign7680_e11083;
        var_guard98_rv = 0.0;

        let (assign7690_e11087,) = {
    if (var_guard98 != 0.0) {
        (1.06,)
    } else {
        (var_dsub_i,)
    }
};
        var_dsub_i = assign7690_e11087;
        var_dsub_i_rv = 0.0;

        let assign7700_e11090: f64 = if var_eta0_i < 0.0 { 1.0 } else { 0.0 };
        var_guard99 = assign7700_e11090;
        var_guard99_rv = 0.0;

        let (assign7710_e11094, assign7710_e11094_d_n0, assign7710_e11094_d_n2, assign7710_e11094_d_n3, assign7710_e11094_d_n4, assign7710_e11094_d_n5, assign7710_e11094_d_n6, assign7710_e11094_d_n7, assign7710_e11094_d_n8, assign7710_e11094_d_n9, assign7710_e11094_d_n10, assign7710_e11094_d_n11, assign7710_e11094_d_n13, assign7710_e11094_d_n14,) = {
    if (var_guard99 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eta0_i, var_eta0_i_dn0, var_eta0_i_dn2, var_eta0_i_dn3, var_eta0_i_dn4, var_eta0_i_dn5, var_eta0_i_dn6, var_eta0_i_dn7, var_eta0_i_dn8, var_eta0_i_dn9, var_eta0_i_dn10, var_eta0_i_dn11, var_eta0_i_dn13, var_eta0_i_dn14,)
    }
};
        var_eta0_i = assign7710_e11094;
        var_eta0_i_dn0 = assign7710_e11094_d_n0;
        var_eta0_i_dn2 = assign7710_e11094_d_n2;
        var_eta0_i_dn3 = assign7710_e11094_d_n3;
        var_eta0_i_dn4 = assign7710_e11094_d_n4;
        var_eta0_i_dn5 = assign7710_e11094_d_n5;
        var_eta0_i_dn6 = assign7710_e11094_d_n6;
        var_eta0_i_dn7 = assign7710_e11094_d_n7;
        var_eta0_i_dn8 = assign7710_e11094_d_n8;
        var_eta0_i_dn9 = assign7710_e11094_d_n9;
        var_eta0_i_dn10 = assign7710_e11094_d_n10;
        var_eta0_i_dn11 = assign7710_e11094_d_n11;
        var_eta0_i_dn13 = assign7710_e11094_d_n13;
        var_eta0_i_dn14 = assign7710_e11094_d_n14;
        var_eta0_i_rv = 0.0;

        let assign7720_e11097: f64 = if var_eta0r_i < 0.0 { 1.0 } else { 0.0 };
        var_guard100 = assign7720_e11097;
        var_guard100_rv = 0.0;

        let (assign7730_e11101,) = {
    if (var_guard100 != 0.0) {
        (0.0,)
    } else {
        (var_eta0r_i,)
    }
};
        var_eta0r_i = assign7730_e11101;
        var_eta0r_i_rv = 0.0;

        let assign7740_e11104: f64 = (-var_leff_1);
        let assign7740_e11105: f64 = if var_lpe0_i < assign7740_e11104 { 1.0 } else { 0.0 };
        var_guard101 = assign7740_e11105;
        var_guard101_rv = 0.0;

        let (assign7750_e11109,) = {
    if (var_guard101 != 0.0) {
        (0.0,)
    } else {
        (var_lpe0_i,)
    }
};
        var_lpe0_i = assign7750_e11109;
        var_lpe0_i_rv = 0.0;

        let assign7760_e11112: f64 = if var_k0si_i < 0.0 { 1.0 } else { 0.0 };
        var_guard102 = assign7760_e11112;
        var_guard102_rv = 0.0;

        let (assign7770_e11116,) = {
    if (var_guard102 != 0.0) {
        (0.0,)
    } else {
        (var_k0si_i,)
    }
};
        var_k0si_i = assign7770_e11116;
        var_k0si_i_rv = 0.0;

        let assign7780_e11119: f64 = if var_k2si_i < 0.0 { 1.0 } else { 0.0 };
        var_guard103 = assign7780_e11119;
        var_guard103_rv = 0.0;

        let (assign7790_e11123,) = {
    if (var_guard103 != 0.0) {
        (0.0,)
    } else {
        (var_k2si_i,)
    }
};
        var_k2si_i = assign7790_e11123;
        var_k2si_i_rv = 0.0;

        let assign7800_e11130: f64 = if ((p.p61 != 0.0) && (var_phibe_i < 0.2)) { 1.0 } else { 0.0 };
        var_guard104 = assign7800_e11130;
        var_guard104_rv = 0.0;

        let (assign7810_e11134,) = {
    if (var_guard104 != 0.0) {
        (0.2,)
    } else {
        (var_phibe_i,)
    }
};
        var_phibe_i = assign7810_e11134;
        var_phibe_i_rv = 0.0;

        let assign7820_e11141: f64 = if ((p.p61 != 0.0) && (var_phibe_i > 1.2)) { 1.0 } else { 0.0 };
        var_guard105 = assign7820_e11141;
        var_guard105_rv = 0.0;

        let (assign7830_e11145,) = {
    if (var_guard105 != 0.0) {
        (1.2,)
    } else {
        (var_phibe_i,)
    }
};
        var_phibe_i = assign7830_e11145;
        var_phibe_i_rv = 0.0;

        let assign7840_e11148: f64 = if var_psat_i < 2.0 { 1.0 } else { 0.0 };
        var_guard106 = assign7840_e11148;
        var_guard106_rv = 0.0;

        let (assign7850_e11152, assign7850_e11152_d_n0, assign7850_e11152_d_n2, assign7850_e11152_d_n3, assign7850_e11152_d_n4, assign7850_e11152_d_n5, assign7850_e11152_d_n6, assign7850_e11152_d_n7, assign7850_e11152_d_n8, assign7850_e11152_d_n9, assign7850_e11152_d_n10, assign7850_e11152_d_n11, assign7850_e11152_d_n13, assign7850_e11152_d_n14,) = {
    if (var_guard106 != 0.0) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_psat_i, var_psat_i_dn0, var_psat_i_dn2, var_psat_i_dn3, var_psat_i_dn4, var_psat_i_dn5, var_psat_i_dn6, var_psat_i_dn7, var_psat_i_dn8, var_psat_i_dn9, var_psat_i_dn10, var_psat_i_dn11, var_psat_i_dn13, var_psat_i_dn14,)
    }
};
        var_psat_i = assign7850_e11152;
        var_psat_i_dn0 = assign7850_e11152_d_n0;
        var_psat_i_dn2 = assign7850_e11152_d_n2;
        var_psat_i_dn3 = assign7850_e11152_d_n3;
        var_psat_i_dn4 = assign7850_e11152_d_n4;
        var_psat_i_dn5 = assign7850_e11152_d_n5;
        var_psat_i_dn6 = assign7850_e11152_d_n6;
        var_psat_i_dn7 = assign7850_e11152_d_n7;
        var_psat_i_dn8 = assign7850_e11152_d_n8;
        var_psat_i_dn9 = assign7850_e11152_d_n9;
        var_psat_i_dn10 = assign7850_e11152_d_n10;
        var_psat_i_dn11 = assign7850_e11152_d_n11;
        var_psat_i_dn13 = assign7850_e11152_d_n13;
        var_psat_i_dn14 = assign7850_e11152_d_n14;
        var_psat_i_rv = 0.0;

        let assign7860_e11155: f64 = if var_psatcv_i < 2.0 { 1.0 } else { 0.0 };
        var_guard107 = assign7860_e11155;
        var_guard107_rv = 0.0;

        let (assign7870_e11159, assign7870_e11159_d_n0, assign7870_e11159_d_n2, assign7870_e11159_d_n3, assign7870_e11159_d_n4, assign7870_e11159_d_n5, assign7870_e11159_d_n6, assign7870_e11159_d_n7, assign7870_e11159_d_n8, assign7870_e11159_d_n9, assign7870_e11159_d_n10, assign7870_e11159_d_n11, assign7870_e11159_d_n13, assign7870_e11159_d_n14,) = {
    if (var_guard107 != 0.0) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_psatcv_i, var_psatcv_i_dn0, var_psatcv_i_dn2, var_psatcv_i_dn3, var_psatcv_i_dn4, var_psatcv_i_dn5, var_psatcv_i_dn6, var_psatcv_i_dn7, var_psatcv_i_dn8, var_psatcv_i_dn9, var_psatcv_i_dn10, var_psatcv_i_dn11, var_psatcv_i_dn13, var_psatcv_i_dn14,)
    }
};
        var_psatcv_i = assign7870_e11159;
        var_psatcv_i_dn0 = assign7870_e11159_d_n0;
        var_psatcv_i_dn2 = assign7870_e11159_d_n2;
        var_psatcv_i_dn3 = assign7870_e11159_d_n3;
        var_psatcv_i_dn4 = assign7870_e11159_d_n4;
        var_psatcv_i_dn5 = assign7870_e11159_d_n5;
        var_psatcv_i_dn6 = assign7870_e11159_d_n6;
        var_psatcv_i_dn7 = assign7870_e11159_d_n7;
        var_psatcv_i_dn8 = assign7870_e11159_d_n8;
        var_psatcv_i_dn9 = assign7870_e11159_d_n9;
        var_psatcv_i_dn10 = assign7870_e11159_d_n10;
        var_psatcv_i_dn11 = assign7870_e11159_d_n11;
        var_psatcv_i_dn13 = assign7870_e11159_d_n13;
        var_psatcv_i_dn14 = assign7870_e11159_d_n14;
        var_psatcv_i_rv = 0.0;

        let assign7880_e11162: f64 = if var_u0_i < 0.0 { 1.0 } else { 0.0 };
        var_guard108 = assign7880_e11162;
        var_guard108_rv = 0.0;

        let (assign7890_e11166, assign7890_e11166_d_n0, assign7890_e11166_d_n2, assign7890_e11166_d_n3, assign7890_e11166_d_n4, assign7890_e11166_d_n5, assign7890_e11166_d_n6, assign7890_e11166_d_n7, assign7890_e11166_d_n8, assign7890_e11166_d_n9, assign7890_e11166_d_n10, assign7890_e11166_d_n11, assign7890_e11166_d_n13, assign7890_e11166_d_n14,) = {
    if (var_guard108 != 0.0) {
        (0.03, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign7890_e11166;
        var_u0_i_dn0 = assign7890_e11166_d_n0;
        var_u0_i_dn2 = assign7890_e11166_d_n2;
        var_u0_i_dn3 = assign7890_e11166_d_n3;
        var_u0_i_dn4 = assign7890_e11166_d_n4;
        var_u0_i_dn5 = assign7890_e11166_d_n5;
        var_u0_i_dn6 = assign7890_e11166_d_n6;
        var_u0_i_dn7 = assign7890_e11166_d_n7;
        var_u0_i_dn8 = assign7890_e11166_d_n8;
        var_u0_i_dn9 = assign7890_e11166_d_n9;
        var_u0_i_dn10 = assign7890_e11166_d_n10;
        var_u0_i_dn11 = assign7890_e11166_d_n11;
        var_u0_i_dn13 = assign7890_e11166_d_n13;
        var_u0_i_dn14 = assign7890_e11166_d_n14;
        var_u0_i_rv = 0.0;

        let assign7900_e11169: f64 = if var_ua_i < 0.0 { 1.0 } else { 0.0 };
        var_guard109 = assign7900_e11169;
        var_guard109_rv = 0.0;

        let (assign7910_e11173, assign7910_e11173_d_n0, assign7910_e11173_d_n2, assign7910_e11173_d_n3, assign7910_e11173_d_n4, assign7910_e11173_d_n5, assign7910_e11173_d_n6, assign7910_e11173_d_n7, assign7910_e11173_d_n8, assign7910_e11173_d_n9, assign7910_e11173_d_n10, assign7910_e11173_d_n11, assign7910_e11173_d_n13, assign7910_e11173_d_n14,) = {
    if (var_guard109 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ua_i, var_ua_i_dn0, var_ua_i_dn2, var_ua_i_dn3, var_ua_i_dn4, var_ua_i_dn5, var_ua_i_dn6, var_ua_i_dn7, var_ua_i_dn8, var_ua_i_dn9, var_ua_i_dn10, var_ua_i_dn11, var_ua_i_dn13, var_ua_i_dn14,)
    }
};
        var_ua_i = assign7910_e11173;
        var_ua_i_dn0 = assign7910_e11173_d_n0;
        var_ua_i_dn2 = assign7910_e11173_d_n2;
        var_ua_i_dn3 = assign7910_e11173_d_n3;
        var_ua_i_dn4 = assign7910_e11173_d_n4;
        var_ua_i_dn5 = assign7910_e11173_d_n5;
        var_ua_i_dn6 = assign7910_e11173_d_n6;
        var_ua_i_dn7 = assign7910_e11173_d_n7;
        var_ua_i_dn8 = assign7910_e11173_d_n8;
        var_ua_i_dn9 = assign7910_e11173_d_n9;
        var_ua_i_dn10 = assign7910_e11173_d_n10;
        var_ua_i_dn11 = assign7910_e11173_d_n11;
        var_ua_i_dn13 = assign7910_e11173_d_n13;
        var_ua_i_dn14 = assign7910_e11173_d_n14;
        var_ua_i_rv = 0.0;

        let assign7920_e11176: f64 = if var_eu_i < 0.0 { 1.0 } else { 0.0 };
        var_guard110 = assign7920_e11176;
        var_guard110_rv = 0.0;

        let (assign7930_e11180, assign7930_e11180_d_n0, assign7930_e11180_d_n2, assign7930_e11180_d_n3, assign7930_e11180_d_n4, assign7930_e11180_d_n5, assign7930_e11180_d_n6, assign7930_e11180_d_n7, assign7930_e11180_d_n8, assign7930_e11180_d_n9, assign7930_e11180_d_n10, assign7930_e11180_d_n11, assign7930_e11180_d_n13, assign7930_e11180_d_n14,) = {
    if (var_guard110 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eu_i, var_eu_i_dn0, var_eu_i_dn2, var_eu_i_dn3, var_eu_i_dn4, var_eu_i_dn5, var_eu_i_dn6, var_eu_i_dn7, var_eu_i_dn8, var_eu_i_dn9, var_eu_i_dn10, var_eu_i_dn11, var_eu_i_dn13, var_eu_i_dn14,)
    }
};
        var_eu_i = assign7930_e11180;
        var_eu_i_dn0 = assign7930_e11180_d_n0;
        var_eu_i_dn2 = assign7930_e11180_d_n2;
        var_eu_i_dn3 = assign7930_e11180_d_n3;
        var_eu_i_dn4 = assign7930_e11180_d_n4;
        var_eu_i_dn5 = assign7930_e11180_d_n5;
        var_eu_i_dn6 = assign7930_e11180_d_n6;
        var_eu_i_dn7 = assign7930_e11180_d_n7;
        var_eu_i_dn8 = assign7930_e11180_d_n8;
        var_eu_i_dn9 = assign7930_e11180_d_n9;
        var_eu_i_dn10 = assign7930_e11180_d_n10;
        var_eu_i_dn11 = assign7930_e11180_d_n11;
        var_eu_i_dn13 = assign7930_e11180_d_n13;
        var_eu_i_dn14 = assign7930_e11180_d_n14;
        var_eu_i_rv = 0.0;

        let assign7940_e11183: f64 = if var_ud_i < 0.0 { 1.0 } else { 0.0 };
        var_guard111 = assign7940_e11183;
        var_guard111_rv = 0.0;

        let (assign7950_e11187, assign7950_e11187_d_n0, assign7950_e11187_d_n2, assign7950_e11187_d_n3, assign7950_e11187_d_n4, assign7950_e11187_d_n5, assign7950_e11187_d_n6, assign7950_e11187_d_n7, assign7950_e11187_d_n8, assign7950_e11187_d_n9, assign7950_e11187_d_n10, assign7950_e11187_d_n11, assign7950_e11187_d_n13, assign7950_e11187_d_n14,) = {
    if (var_guard111 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ud_i, var_ud_i_dn0, var_ud_i_dn2, var_ud_i_dn3, var_ud_i_dn4, var_ud_i_dn5, var_ud_i_dn6, var_ud_i_dn7, var_ud_i_dn8, var_ud_i_dn9, var_ud_i_dn10, var_ud_i_dn11, var_ud_i_dn13, var_ud_i_dn14,)
    }
};
        var_ud_i = assign7950_e11187;
        var_ud_i_dn0 = assign7950_e11187_d_n0;
        var_ud_i_dn2 = assign7950_e11187_d_n2;
        var_ud_i_dn3 = assign7950_e11187_d_n3;
        var_ud_i_dn4 = assign7950_e11187_d_n4;
        var_ud_i_dn5 = assign7950_e11187_d_n5;
        var_ud_i_dn6 = assign7950_e11187_d_n6;
        var_ud_i_dn7 = assign7950_e11187_d_n7;
        var_ud_i_dn8 = assign7950_e11187_d_n8;
        var_ud_i_dn9 = assign7950_e11187_d_n9;
        var_ud_i_dn10 = assign7950_e11187_d_n10;
        var_ud_i_dn11 = assign7950_e11187_d_n11;
        var_ud_i_dn13 = assign7950_e11187_d_n13;
        var_ud_i_dn14 = assign7950_e11187_d_n14;
        var_ud_i_rv = 0.0;

        let assign7960_e11190: f64 = if var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        var_guard112 = assign7960_e11190;
        var_guard112_rv = 0.0;

        let (assign7970_e11194,) = {
    if (var_guard112 != 0.0) {
        (0.0,)
    } else {
        (var_ucs_i,)
    }
};
        var_ucs_i = assign7970_e11194;
        var_ucs_i_rv = 0.0;

        let assign7980_e11197: f64 = if var_etamob_i < 0.0 { 1.0 } else { 0.0 };
        var_guard113 = assign7980_e11197;
        var_guard113_rv = 0.0;

        let (assign7990_e11201,) = {
    if (var_guard113 != 0.0) {
        (0.0,)
    } else {
        (var_etamob_i,)
    }
};
        var_etamob_i = assign7990_e11201;
        var_etamob_i_rv = 0.0;

        let assign8000_e11204: f64 = if var_rdsw_i < 0.0 { 1.0 } else { 0.0 };
        var_guard114 = assign8000_e11204;
        var_guard114_rv = 0.0;

        let (assign8010_e11208, assign8010_e11208_d_n0, assign8010_e11208_d_n2, assign8010_e11208_d_n3, assign8010_e11208_d_n4, assign8010_e11208_d_n5, assign8010_e11208_d_n6, assign8010_e11208_d_n7, assign8010_e11208_d_n8, assign8010_e11208_d_n9, assign8010_e11208_d_n10, assign8010_e11208_d_n11, assign8010_e11208_d_n13, assign8010_e11208_d_n14,) = {
    if (var_guard114 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdsw_i, var_rdsw_i_dn0, var_rdsw_i_dn2, var_rdsw_i_dn3, var_rdsw_i_dn4, var_rdsw_i_dn5, var_rdsw_i_dn6, var_rdsw_i_dn7, var_rdsw_i_dn8, var_rdsw_i_dn9, var_rdsw_i_dn10, var_rdsw_i_dn11, var_rdsw_i_dn13, var_rdsw_i_dn14,)
    }
};
        var_rdsw_i = assign8010_e11208;
        var_rdsw_i_dn0 = assign8010_e11208_d_n0;
        var_rdsw_i_dn2 = assign8010_e11208_d_n2;
        var_rdsw_i_dn3 = assign8010_e11208_d_n3;
        var_rdsw_i_dn4 = assign8010_e11208_d_n4;
        var_rdsw_i_dn5 = assign8010_e11208_d_n5;
        var_rdsw_i_dn6 = assign8010_e11208_d_n6;
        var_rdsw_i_dn7 = assign8010_e11208_d_n7;
        var_rdsw_i_dn8 = assign8010_e11208_d_n8;
        var_rdsw_i_dn9 = assign8010_e11208_d_n9;
        var_rdsw_i_dn10 = assign8010_e11208_d_n10;
        var_rdsw_i_dn11 = assign8010_e11208_d_n11;
        var_rdsw_i_dn13 = assign8010_e11208_d_n13;
        var_rdsw_i_dn14 = assign8010_e11208_d_n14;
        var_rdsw_i_rv = 0.0;

        let assign8020_e11211: f64 = if var_rsw_i < 0.0 { 1.0 } else { 0.0 };
        var_guard115 = assign8020_e11211;
        var_guard115_rv = 0.0;

        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn0_slot = var_covd_i_dn0;
        *var_covd_i_dn10_slot = var_covd_i_dn10;
        *var_covd_i_dn11_slot = var_covd_i_dn11;
        *var_covd_i_dn13_slot = var_covd_i_dn13;
        *var_covd_i_dn14_slot = var_covd_i_dn14;
        *var_covd_i_dn2_slot = var_covd_i_dn2;
        *var_covd_i_dn3_slot = var_covd_i_dn3;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn5_slot = var_covd_i_dn5;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covd_i_rv_slot = var_covd_i_rv;
        *var_covs_i_slot = var_covs_i;
        *var_covs_i_dn0_slot = var_covs_i_dn0;
        *var_covs_i_dn10_slot = var_covs_i_dn10;
        *var_covs_i_dn11_slot = var_covs_i_dn11;
        *var_covs_i_dn13_slot = var_covs_i_dn13;
        *var_covs_i_dn14_slot = var_covs_i_dn14;
        *var_covs_i_dn2_slot = var_covs_i_dn2;
        *var_covs_i_dn3_slot = var_covs_i_dn3;
        *var_covs_i_dn4_slot = var_covs_i_dn4;
        *var_covs_i_dn5_slot = var_covs_i_dn5;
        *var_covs_i_dn6_slot = var_covs_i_dn6;
        *var_covs_i_dn7_slot = var_covs_i_dn7;
        *var_covs_i_dn8_slot = var_covs_i_dn8;
        *var_covs_i_dn9_slot = var_covs_i_dn9;
        *var_covs_i_rv_slot = var_covs_i_rv;
        *var_dsub_i_slot = var_dsub_i;
        *var_dsub_i_rv_slot = var_dsub_i_rv;
        *var_dvt1_i_slot = var_dvt1_i;
        *var_dvt1_i_rv_slot = var_dvt1_i_rv;
        *var_dvt1ss_i_slot = var_dvt1ss_i;
        *var_dvt1ss_i_rv_slot = var_dvt1ss_i_rv;
        *var_eta0_i_slot = var_eta0_i;
        *var_eta0_i_dn0_slot = var_eta0_i_dn0;
        *var_eta0_i_dn10_slot = var_eta0_i_dn10;
        *var_eta0_i_dn11_slot = var_eta0_i_dn11;
        *var_eta0_i_dn13_slot = var_eta0_i_dn13;
        *var_eta0_i_dn14_slot = var_eta0_i_dn14;
        *var_eta0_i_dn2_slot = var_eta0_i_dn2;
        *var_eta0_i_dn3_slot = var_eta0_i_dn3;
        *var_eta0_i_dn4_slot = var_eta0_i_dn4;
        *var_eta0_i_dn5_slot = var_eta0_i_dn5;
        *var_eta0_i_dn6_slot = var_eta0_i_dn6;
        *var_eta0_i_dn7_slot = var_eta0_i_dn7;
        *var_eta0_i_dn8_slot = var_eta0_i_dn8;
        *var_eta0_i_dn9_slot = var_eta0_i_dn9;
        *var_eta0_i_rv_slot = var_eta0_i_rv;
        *var_eta0r_i_slot = var_eta0r_i;
        *var_eta0r_i_rv_slot = var_eta0r_i_rv;
        *var_etamob_i_slot = var_etamob_i;
        *var_etamob_i_rv_slot = var_etamob_i_rv;
        *var_eu_i_slot = var_eu_i;
        *var_eu_i_dn0_slot = var_eu_i_dn0;
        *var_eu_i_dn10_slot = var_eu_i_dn10;
        *var_eu_i_dn11_slot = var_eu_i_dn11;
        *var_eu_i_dn13_slot = var_eu_i_dn13;
        *var_eu_i_dn14_slot = var_eu_i_dn14;
        *var_eu_i_dn2_slot = var_eu_i_dn2;
        *var_eu_i_dn3_slot = var_eu_i_dn3;
        *var_eu_i_dn4_slot = var_eu_i_dn4;
        *var_eu_i_dn5_slot = var_eu_i_dn5;
        *var_eu_i_dn6_slot = var_eu_i_dn6;
        *var_eu_i_dn7_slot = var_eu_i_dn7;
        *var_eu_i_dn8_slot = var_eu_i_dn8;
        *var_eu_i_dn9_slot = var_eu_i_dn9;
        *var_eu_i_rv_slot = var_eu_i_rv;
        *var_guard100_slot = var_guard100;
        *var_guard100_rv_slot = var_guard100_rv;
        *var_guard101_slot = var_guard101;
        *var_guard101_rv_slot = var_guard101_rv;
        *var_guard102_slot = var_guard102;
        *var_guard102_rv_slot = var_guard102_rv;
        *var_guard103_slot = var_guard103;
        *var_guard103_rv_slot = var_guard103_rv;
        *var_guard104_slot = var_guard104;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_guard112_slot = var_guard112;
        *var_guard112_rv_slot = var_guard112_rv;
        *var_guard113_slot = var_guard113;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_guard114_slot = var_guard114;
        *var_guard114_rv_slot = var_guard114_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard89_slot = var_guard89;
        *var_guard89_rv_slot = var_guard89_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_guard93_slot = var_guard93;
        *var_guard93_rv_slot = var_guard93_rv;
        *var_guard94_slot = var_guard94;
        *var_guard94_rv_slot = var_guard94_rv;
        *var_guard98_slot = var_guard98;
        *var_guard98_rv_slot = var_guard98_rv;
        *var_guard99_slot = var_guard99;
        *var_guard99_rv_slot = var_guard99_rv;
        *var_k0si_i_slot = var_k0si_i;
        *var_k0si_i_rv_slot = var_k0si_i_rv;
        *var_k2si_i_slot = var_k2si_i;
        *var_k2si_i_rv_slot = var_k2si_i_rv;
        *var_lpe0_i_slot = var_lpe0_i;
        *var_lpe0_i_rv_slot = var_lpe0_i_rv;
        *var_phibe_i_slot = var_phibe_i;
        *var_phibe_i_rv_slot = var_phibe_i_rv;
        *var_psat_i_slot = var_psat_i;
        *var_psat_i_dn0_slot = var_psat_i_dn0;
        *var_psat_i_dn10_slot = var_psat_i_dn10;
        *var_psat_i_dn11_slot = var_psat_i_dn11;
        *var_psat_i_dn13_slot = var_psat_i_dn13;
        *var_psat_i_dn14_slot = var_psat_i_dn14;
        *var_psat_i_dn2_slot = var_psat_i_dn2;
        *var_psat_i_dn3_slot = var_psat_i_dn3;
        *var_psat_i_dn4_slot = var_psat_i_dn4;
        *var_psat_i_dn5_slot = var_psat_i_dn5;
        *var_psat_i_dn6_slot = var_psat_i_dn6;
        *var_psat_i_dn7_slot = var_psat_i_dn7;
        *var_psat_i_dn8_slot = var_psat_i_dn8;
        *var_psat_i_dn9_slot = var_psat_i_dn9;
        *var_psat_i_rv_slot = var_psat_i_rv;
        *var_psatcv_i_slot = var_psatcv_i;
        *var_psatcv_i_dn0_slot = var_psatcv_i_dn0;
        *var_psatcv_i_dn10_slot = var_psatcv_i_dn10;
        *var_psatcv_i_dn11_slot = var_psatcv_i_dn11;
        *var_psatcv_i_dn13_slot = var_psatcv_i_dn13;
        *var_psatcv_i_dn14_slot = var_psatcv_i_dn14;
        *var_psatcv_i_dn2_slot = var_psatcv_i_dn2;
        *var_psatcv_i_dn3_slot = var_psatcv_i_dn3;
        *var_psatcv_i_dn4_slot = var_psatcv_i_dn4;
        *var_psatcv_i_dn5_slot = var_psatcv_i_dn5;
        *var_psatcv_i_dn6_slot = var_psatcv_i_dn6;
        *var_psatcv_i_dn7_slot = var_psatcv_i_dn7;
        *var_psatcv_i_dn8_slot = var_psatcv_i_dn8;
        *var_psatcv_i_dn9_slot = var_psatcv_i_dn9;
        *var_psatcv_i_rv_slot = var_psatcv_i_rv;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdsw_i_dn0_slot = var_rdsw_i_dn0;
        *var_rdsw_i_dn10_slot = var_rdsw_i_dn10;
        *var_rdsw_i_dn11_slot = var_rdsw_i_dn11;
        *var_rdsw_i_dn13_slot = var_rdsw_i_dn13;
        *var_rdsw_i_dn14_slot = var_rdsw_i_dn14;
        *var_rdsw_i_dn2_slot = var_rdsw_i_dn2;
        *var_rdsw_i_dn3_slot = var_rdsw_i_dn3;
        *var_rdsw_i_dn4_slot = var_rdsw_i_dn4;
        *var_rdsw_i_dn5_slot = var_rdsw_i_dn5;
        *var_rdsw_i_dn6_slot = var_rdsw_i_dn6;
        *var_rdsw_i_dn7_slot = var_rdsw_i_dn7;
        *var_rdsw_i_dn8_slot = var_rdsw_i_dn8;
        *var_rdsw_i_dn9_slot = var_rdsw_i_dn9;
        *var_rdsw_i_rv_slot = var_rdsw_i_rv;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_dn0_slot = var_u0_i_dn0;
        *var_u0_i_dn10_slot = var_u0_i_dn10;
        *var_u0_i_dn11_slot = var_u0_i_dn11;
        *var_u0_i_dn13_slot = var_u0_i_dn13;
        *var_u0_i_dn14_slot = var_u0_i_dn14;
        *var_u0_i_dn2_slot = var_u0_i_dn2;
        *var_u0_i_dn3_slot = var_u0_i_dn3;
        *var_u0_i_dn4_slot = var_u0_i_dn4;
        *var_u0_i_dn5_slot = var_u0_i_dn5;
        *var_u0_i_dn6_slot = var_u0_i_dn6;
        *var_u0_i_dn7_slot = var_u0_i_dn7;
        *var_u0_i_dn8_slot = var_u0_i_dn8;
        *var_u0_i_dn9_slot = var_u0_i_dn9;
        *var_u0_i_rv_slot = var_u0_i_rv;
        *var_ua_i_slot = var_ua_i;
        *var_ua_i_dn0_slot = var_ua_i_dn0;
        *var_ua_i_dn10_slot = var_ua_i_dn10;
        *var_ua_i_dn11_slot = var_ua_i_dn11;
        *var_ua_i_dn13_slot = var_ua_i_dn13;
        *var_ua_i_dn14_slot = var_ua_i_dn14;
        *var_ua_i_dn2_slot = var_ua_i_dn2;
        *var_ua_i_dn3_slot = var_ua_i_dn3;
        *var_ua_i_dn4_slot = var_ua_i_dn4;
        *var_ua_i_dn5_slot = var_ua_i_dn5;
        *var_ua_i_dn6_slot = var_ua_i_dn6;
        *var_ua_i_dn7_slot = var_ua_i_dn7;
        *var_ua_i_dn8_slot = var_ua_i_dn8;
        *var_ua_i_dn9_slot = var_ua_i_dn9;
        *var_ua_i_rv_slot = var_ua_i_rv;
        *var_ucs_i_slot = var_ucs_i;
        *var_ucs_i_rv_slot = var_ucs_i_rv;
        *var_ud_i_slot = var_ud_i;
        *var_ud_i_dn0_slot = var_ud_i_dn0;
        *var_ud_i_dn10_slot = var_ud_i_dn10;
        *var_ud_i_dn11_slot = var_ud_i_dn11;
        *var_ud_i_dn13_slot = var_ud_i_dn13;
        *var_ud_i_dn14_slot = var_ud_i_dn14;
        *var_ud_i_dn2_slot = var_ud_i_dn2;
        *var_ud_i_dn3_slot = var_ud_i_dn3;
        *var_ud_i_dn4_slot = var_ud_i_dn4;
        *var_ud_i_dn5_slot = var_ud_i_dn5;
        *var_ud_i_dn6_slot = var_ud_i_dn6;
        *var_ud_i_dn7_slot = var_ud_i_dn7;
        *var_ud_i_dn8_slot = var_ud_i_dn8;
        *var_ud_i_dn9_slot = var_ud_i_dn9;
        *var_ud_i_rv_slot = var_ud_i_rv;
        *var_vsat1_i_slot = var_vsat1_i;
        *var_vsat1_i_dn0_slot = var_vsat1_i_dn0;
        *var_vsat1_i_dn10_slot = var_vsat1_i_dn10;
        *var_vsat1_i_dn11_slot = var_vsat1_i_dn11;
        *var_vsat1_i_dn13_slot = var_vsat1_i_dn13;
        *var_vsat1_i_dn14_slot = var_vsat1_i_dn14;
        *var_vsat1_i_dn2_slot = var_vsat1_i_dn2;
        *var_vsat1_i_dn3_slot = var_vsat1_i_dn3;
        *var_vsat1_i_dn4_slot = var_vsat1_i_dn4;
        *var_vsat1_i_dn5_slot = var_vsat1_i_dn5;
        *var_vsat1_i_dn6_slot = var_vsat1_i_dn6;
        *var_vsat1_i_dn7_slot = var_vsat1_i_dn7;
        *var_vsat1_i_dn8_slot = var_vsat1_i_dn8;
        *var_vsat1_i_dn9_slot = var_vsat1_i_dn9;
        *var_vsat1_i_rv_slot = var_vsat1_i_rv;
        *var_vsat1r_i_slot = var_vsat1r_i;
        *var_vsat1r_i_dn0_slot = var_vsat1r_i_dn0;
        *var_vsat1r_i_dn10_slot = var_vsat1r_i_dn10;
        *var_vsat1r_i_dn11_slot = var_vsat1r_i_dn11;
        *var_vsat1r_i_dn13_slot = var_vsat1r_i_dn13;
        *var_vsat1r_i_dn14_slot = var_vsat1r_i_dn14;
        *var_vsat1r_i_dn2_slot = var_vsat1r_i_dn2;
        *var_vsat1r_i_dn3_slot = var_vsat1r_i_dn3;
        *var_vsat1r_i_dn4_slot = var_vsat1r_i_dn4;
        *var_vsat1r_i_dn5_slot = var_vsat1r_i_dn5;
        *var_vsat1r_i_dn6_slot = var_vsat1r_i_dn6;
        *var_vsat1r_i_dn7_slot = var_vsat1r_i_dn7;
        *var_vsat1r_i_dn8_slot = var_vsat1r_i_dn8;
        *var_vsat1r_i_dn9_slot = var_vsat1r_i_dn9;
        *var_vsat1r_i_rv_slot = var_vsat1r_i_rv;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsat_i_dn0_slot = var_vsat_i_dn0;
        *var_vsat_i_dn10_slot = var_vsat_i_dn10;
        *var_vsat_i_dn11_slot = var_vsat_i_dn11;
        *var_vsat_i_dn13_slot = var_vsat_i_dn13;
        *var_vsat_i_dn14_slot = var_vsat_i_dn14;
        *var_vsat_i_dn2_slot = var_vsat_i_dn2;
        *var_vsat_i_dn3_slot = var_vsat_i_dn3;
        *var_vsat_i_dn4_slot = var_vsat_i_dn4;
        *var_vsat_i_dn5_slot = var_vsat_i_dn5;
        *var_vsat_i_dn6_slot = var_vsat_i_dn6;
        *var_vsat_i_dn7_slot = var_vsat_i_dn7;
        *var_vsat_i_dn8_slot = var_vsat_i_dn8;
        *var_vsat_i_dn9_slot = var_vsat_i_dn9;
        *var_vsat_i_rv_slot = var_vsat_i_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_guard115: f64,
        var_cfd_i_slot: &mut f64,
        var_cfd_i_rv_slot: &mut f64,
        var_cfs_i_slot: &mut f64,
        var_cfs_i_rv_slot: &mut f64,
        var_cgbl_i_slot: &mut f64,
        var_cgbl_i_rv_slot: &mut f64,
        var_cgdl_i_slot: &mut f64,
        var_cgdl_i_rv_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidl_i_rv_slot: &mut f64,
        var_cgisl_i_slot: &mut f64,
        var_cgisl_i_rv_slot: &mut f64,
        var_cgsl_i_slot: &mut f64,
        var_cgsl_i_rv_slot: &mut f64,
        var_ckappab_i_slot: &mut f64,
        var_ckappab_i_rv_slot: &mut f64,
        var_ckappad_i_slot: &mut f64,
        var_ckappad_i_rv_slot: &mut f64,
        var_ckappas_i_slot: &mut f64,
        var_ckappas_i_rv_slot: &mut f64,
        var_deltaprsd_v_slot: &mut f64,
        var_deltaprsd_v_rv_slot: &mut f64,
        var_drout_i_slot: &mut f64,
        var_drout_i_rv_slot: &mut f64,
        var_eur_i_slot: &mut f64,
        var_eur_i_dn0_slot: &mut f64,
        var_eur_i_dn10_slot: &mut f64,
        var_eur_i_dn11_slot: &mut f64,
        var_eur_i_dn13_slot: &mut f64,
        var_eur_i_dn14_slot: &mut f64,
        var_eur_i_dn2_slot: &mut f64,
        var_eur_i_dn3_slot: &mut f64,
        var_eur_i_dn4_slot: &mut f64,
        var_eur_i_dn5_slot: &mut f64,
        var_eur_i_dn6_slot: &mut f64,
        var_eur_i_dn7_slot: &mut f64,
        var_eur_i_dn8_slot: &mut f64,
        var_eur_i_dn9_slot: &mut f64,
        var_eur_i_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard117_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_guard124_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard129_rv_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard130_rv_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard131_rv_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard132_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard135_rv_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard136_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_guard140_slot: &mut f64,
        var_guard140_rv_slot: &mut f64,
        var_guard141_slot: &mut f64,
        var_guard141_rv_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard159_rv_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard160_rv_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard161_rv_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_guard162_rv_slot: &mut f64,
        var_guard163_slot: &mut f64,
        var_guard163_rv_slot: &mut f64,
        var_guard164_slot: &mut f64,
        var_guard164_rv_slot: &mut f64,
        var_guard165_slot: &mut f64,
        var_guard165_rv_slot: &mut f64,
        var_guard166_slot: &mut f64,
        var_guard166_rv_slot: &mut f64,
        var_guard167_slot: &mut f64,
        var_guard167_rv_slot: &mut f64,
        var_guard168_slot: &mut f64,
        var_guard168_rv_slot: &mut f64,
        var_mexp_i_slot: &mut f64,
        var_mexp_i_dn0_slot: &mut f64,
        var_mexp_i_dn10_slot: &mut f64,
        var_mexp_i_dn11_slot: &mut f64,
        var_mexp_i_dn13_slot: &mut f64,
        var_mexp_i_dn14_slot: &mut f64,
        var_mexp_i_dn2_slot: &mut f64,
        var_mexp_i_dn3_slot: &mut f64,
        var_mexp_i_dn4_slot: &mut f64,
        var_mexp_i_dn5_slot: &mut f64,
        var_mexp_i_dn6_slot: &mut f64,
        var_mexp_i_dn7_slot: &mut f64,
        var_mexp_i_dn8_slot: &mut f64,
        var_mexp_i_dn9_slot: &mut f64,
        var_mexp_i_rv_slot: &mut f64,
        var_mexpr_i_slot: &mut f64,
        var_mexpr_i_dn0_slot: &mut f64,
        var_mexpr_i_dn10_slot: &mut f64,
        var_mexpr_i_dn11_slot: &mut f64,
        var_mexpr_i_dn13_slot: &mut f64,
        var_mexpr_i_dn14_slot: &mut f64,
        var_mexpr_i_dn2_slot: &mut f64,
        var_mexpr_i_dn3_slot: &mut f64,
        var_mexpr_i_dn4_slot: &mut f64,
        var_mexpr_i_dn5_slot: &mut f64,
        var_mexpr_i_dn6_slot: &mut f64,
        var_mexpr_i_dn7_slot: &mut f64,
        var_mexpr_i_dn8_slot: &mut f64,
        var_mexpr_i_dn9_slot: &mut f64,
        var_mexpr_i_rv_slot: &mut f64,
        var_nigbacc_i_slot: &mut f64,
        var_nigbacc_i_rv_slot: &mut f64,
        var_nigbinv_i_slot: &mut f64,
        var_nigbinv_i_rv_slot: &mut f64,
        var_pigcd_i_slot: &mut f64,
        var_pigcd_i_rv_slot: &mut f64,
        var_poxedge_i_slot: &mut f64,
        var_poxedge_i_rv_slot: &mut f64,
        var_prwgd_i_slot: &mut f64,
        var_prwgd_i_rv_slot: &mut f64,
        var_prwgs_i_slot: &mut f64,
        var_prwgs_i_rv_slot: &mut f64,
        var_ptwg_i_slot: &mut f64,
        var_ptwg_i_dn0_slot: &mut f64,
        var_ptwg_i_dn10_slot: &mut f64,
        var_ptwg_i_dn11_slot: &mut f64,
        var_ptwg_i_dn13_slot: &mut f64,
        var_ptwg_i_dn14_slot: &mut f64,
        var_ptwg_i_dn2_slot: &mut f64,
        var_ptwg_i_dn3_slot: &mut f64,
        var_ptwg_i_dn4_slot: &mut f64,
        var_ptwg_i_dn5_slot: &mut f64,
        var_ptwg_i_dn6_slot: &mut f64,
        var_ptwg_i_dn7_slot: &mut f64,
        var_ptwg_i_dn8_slot: &mut f64,
        var_ptwg_i_dn9_slot: &mut f64,
        var_ptwg_i_rv_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdw_i_dn0_slot: &mut f64,
        var_rdw_i_dn10_slot: &mut f64,
        var_rdw_i_dn11_slot: &mut f64,
        var_rdw_i_dn13_slot: &mut f64,
        var_rdw_i_dn14_slot: &mut f64,
        var_rdw_i_dn2_slot: &mut f64,
        var_rdw_i_dn3_slot: &mut f64,
        var_rdw_i_dn4_slot: &mut f64,
        var_rdw_i_dn5_slot: &mut f64,
        var_rdw_i_dn6_slot: &mut f64,
        var_rdw_i_dn7_slot: &mut f64,
        var_rdw_i_dn8_slot: &mut f64,
        var_rdw_i_dn9_slot: &mut f64,
        var_rdw_i_rv_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rsw_i_dn0_slot: &mut f64,
        var_rsw_i_dn10_slot: &mut f64,
        var_rsw_i_dn11_slot: &mut f64,
        var_rsw_i_dn13_slot: &mut f64,
        var_rsw_i_dn14_slot: &mut f64,
        var_rsw_i_dn2_slot: &mut f64,
        var_rsw_i_dn3_slot: &mut f64,
        var_rsw_i_dn4_slot: &mut f64,
        var_rsw_i_dn5_slot: &mut f64,
        var_rsw_i_dn6_slot: &mut f64,
        var_rsw_i_dn7_slot: &mut f64,
        var_rsw_i_dn8_slot: &mut f64,
        var_rsw_i_dn9_slot: &mut f64,
        var_rsw_i_rv_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_dn0_slot: &mut f64,
        var_u0r_i_dn10_slot: &mut f64,
        var_u0r_i_dn11_slot: &mut f64,
        var_u0r_i_dn13_slot: &mut f64,
        var_u0r_i_dn14_slot: &mut f64,
        var_u0r_i_dn2_slot: &mut f64,
        var_u0r_i_dn3_slot: &mut f64,
        var_u0r_i_dn4_slot: &mut f64,
        var_u0r_i_dn5_slot: &mut f64,
        var_u0r_i_dn6_slot: &mut f64,
        var_u0r_i_dn7_slot: &mut f64,
        var_u0r_i_dn8_slot: &mut f64,
        var_u0r_i_dn9_slot: &mut f64,
        var_u0r_i_rv_slot: &mut f64,
        var_uar_i_slot: &mut f64,
        var_uar_i_dn0_slot: &mut f64,
        var_uar_i_dn10_slot: &mut f64,
        var_uar_i_dn11_slot: &mut f64,
        var_uar_i_dn13_slot: &mut f64,
        var_uar_i_dn14_slot: &mut f64,
        var_uar_i_dn2_slot: &mut f64,
        var_uar_i_dn3_slot: &mut f64,
        var_uar_i_dn4_slot: &mut f64,
        var_uar_i_dn5_slot: &mut f64,
        var_uar_i_dn6_slot: &mut f64,
        var_uar_i_dn7_slot: &mut f64,
        var_uar_i_dn8_slot: &mut f64,
        var_uar_i_dn9_slot: &mut f64,
        var_uar_i_rv_slot: &mut f64,
        var_udr_i_slot: &mut f64,
        var_udr_i_dn0_slot: &mut f64,
        var_udr_i_dn10_slot: &mut f64,
        var_udr_i_dn11_slot: &mut f64,
        var_udr_i_dn13_slot: &mut f64,
        var_udr_i_dn14_slot: &mut f64,
        var_udr_i_dn2_slot: &mut f64,
        var_udr_i_dn3_slot: &mut f64,
        var_udr_i_dn4_slot: &mut f64,
        var_udr_i_dn5_slot: &mut f64,
        var_udr_i_dn6_slot: &mut f64,
        var_udr_i_dn7_slot: &mut f64,
        var_udr_i_dn8_slot: &mut f64,
        var_udr_i_dn9_slot: &mut f64,
        var_udr_i_rv_slot: &mut f64,
    ) {
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfd_i_rv: f64 = *var_cfd_i_rv_slot;
        let mut var_cfs_i: f64 = *var_cfs_i_slot;
        let mut var_cfs_i_rv: f64 = *var_cfs_i_rv_slot;
        let mut var_cgbl_i: f64 = *var_cgbl_i_slot;
        let mut var_cgbl_i_rv: f64 = *var_cgbl_i_rv_slot;
        let mut var_cgdl_i: f64 = *var_cgdl_i_slot;
        let mut var_cgdl_i_rv: f64 = *var_cgdl_i_rv_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidl_i_rv: f64 = *var_cgidl_i_rv_slot;
        let mut var_cgisl_i: f64 = *var_cgisl_i_slot;
        let mut var_cgisl_i_rv: f64 = *var_cgisl_i_rv_slot;
        let mut var_cgsl_i: f64 = *var_cgsl_i_slot;
        let mut var_cgsl_i_rv: f64 = *var_cgsl_i_rv_slot;
        let mut var_ckappab_i: f64 = *var_ckappab_i_slot;
        let mut var_ckappab_i_rv: f64 = *var_ckappab_i_rv_slot;
        let mut var_ckappad_i: f64 = *var_ckappad_i_slot;
        let mut var_ckappad_i_rv: f64 = *var_ckappad_i_rv_slot;
        let mut var_ckappas_i: f64 = *var_ckappas_i_slot;
        let mut var_ckappas_i_rv: f64 = *var_ckappas_i_rv_slot;
        let mut var_deltaprsd_v: f64 = *var_deltaprsd_v_slot;
        let mut var_deltaprsd_v_rv: f64 = *var_deltaprsd_v_rv_slot;
        let mut var_drout_i: f64 = *var_drout_i_slot;
        let mut var_drout_i_rv: f64 = *var_drout_i_rv_slot;
        let mut var_eur_i: f64 = *var_eur_i_slot;
        let mut var_eur_i_dn0: f64 = *var_eur_i_dn0_slot;
        let mut var_eur_i_dn10: f64 = *var_eur_i_dn10_slot;
        let mut var_eur_i_dn11: f64 = *var_eur_i_dn11_slot;
        let mut var_eur_i_dn13: f64 = *var_eur_i_dn13_slot;
        let mut var_eur_i_dn14: f64 = *var_eur_i_dn14_slot;
        let mut var_eur_i_dn2: f64 = *var_eur_i_dn2_slot;
        let mut var_eur_i_dn3: f64 = *var_eur_i_dn3_slot;
        let mut var_eur_i_dn4: f64 = *var_eur_i_dn4_slot;
        let mut var_eur_i_dn5: f64 = *var_eur_i_dn5_slot;
        let mut var_eur_i_dn6: f64 = *var_eur_i_dn6_slot;
        let mut var_eur_i_dn7: f64 = *var_eur_i_dn7_slot;
        let mut var_eur_i_dn8: f64 = *var_eur_i_dn8_slot;
        let mut var_eur_i_dn9: f64 = *var_eur_i_dn9_slot;
        let mut var_eur_i_rv: f64 = *var_eur_i_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard117_rv: f64 = *var_guard117_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard129_rv: f64 = *var_guard129_rv_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard130_rv: f64 = *var_guard130_rv_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard131_rv: f64 = *var_guard131_rv_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard132_rv: f64 = *var_guard132_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard135_rv: f64 = *var_guard135_rv_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard136_rv: f64 = *var_guard136_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_guard140: f64 = *var_guard140_slot;
        let mut var_guard140_rv: f64 = *var_guard140_rv_slot;
        let mut var_guard141: f64 = *var_guard141_slot;
        let mut var_guard141_rv: f64 = *var_guard141_rv_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard159_rv: f64 = *var_guard159_rv_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard160_rv: f64 = *var_guard160_rv_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard161_rv: f64 = *var_guard161_rv_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_guard162_rv: f64 = *var_guard162_rv_slot;
        let mut var_guard163: f64 = *var_guard163_slot;
        let mut var_guard163_rv: f64 = *var_guard163_rv_slot;
        let mut var_guard164: f64 = *var_guard164_slot;
        let mut var_guard164_rv: f64 = *var_guard164_rv_slot;
        let mut var_guard165: f64 = *var_guard165_slot;
        let mut var_guard165_rv: f64 = *var_guard165_rv_slot;
        let mut var_guard166: f64 = *var_guard166_slot;
        let mut var_guard166_rv: f64 = *var_guard166_rv_slot;
        let mut var_guard167: f64 = *var_guard167_slot;
        let mut var_guard167_rv: f64 = *var_guard167_rv_slot;
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard168_rv: f64 = *var_guard168_rv_slot;
        let mut var_mexp_i: f64 = *var_mexp_i_slot;
        let mut var_mexp_i_dn0: f64 = *var_mexp_i_dn0_slot;
        let mut var_mexp_i_dn10: f64 = *var_mexp_i_dn10_slot;
        let mut var_mexp_i_dn11: f64 = *var_mexp_i_dn11_slot;
        let mut var_mexp_i_dn13: f64 = *var_mexp_i_dn13_slot;
        let mut var_mexp_i_dn14: f64 = *var_mexp_i_dn14_slot;
        let mut var_mexp_i_dn2: f64 = *var_mexp_i_dn2_slot;
        let mut var_mexp_i_dn3: f64 = *var_mexp_i_dn3_slot;
        let mut var_mexp_i_dn4: f64 = *var_mexp_i_dn4_slot;
        let mut var_mexp_i_dn5: f64 = *var_mexp_i_dn5_slot;
        let mut var_mexp_i_dn6: f64 = *var_mexp_i_dn6_slot;
        let mut var_mexp_i_dn7: f64 = *var_mexp_i_dn7_slot;
        let mut var_mexp_i_dn8: f64 = *var_mexp_i_dn8_slot;
        let mut var_mexp_i_dn9: f64 = *var_mexp_i_dn9_slot;
        let mut var_mexp_i_rv: f64 = *var_mexp_i_rv_slot;
        let mut var_mexpr_i: f64 = *var_mexpr_i_slot;
        let mut var_mexpr_i_dn0: f64 = *var_mexpr_i_dn0_slot;
        let mut var_mexpr_i_dn10: f64 = *var_mexpr_i_dn10_slot;
        let mut var_mexpr_i_dn11: f64 = *var_mexpr_i_dn11_slot;
        let mut var_mexpr_i_dn13: f64 = *var_mexpr_i_dn13_slot;
        let mut var_mexpr_i_dn14: f64 = *var_mexpr_i_dn14_slot;
        let mut var_mexpr_i_dn2: f64 = *var_mexpr_i_dn2_slot;
        let mut var_mexpr_i_dn3: f64 = *var_mexpr_i_dn3_slot;
        let mut var_mexpr_i_dn4: f64 = *var_mexpr_i_dn4_slot;
        let mut var_mexpr_i_dn5: f64 = *var_mexpr_i_dn5_slot;
        let mut var_mexpr_i_dn6: f64 = *var_mexpr_i_dn6_slot;
        let mut var_mexpr_i_dn7: f64 = *var_mexpr_i_dn7_slot;
        let mut var_mexpr_i_dn8: f64 = *var_mexpr_i_dn8_slot;
        let mut var_mexpr_i_dn9: f64 = *var_mexpr_i_dn9_slot;
        let mut var_mexpr_i_rv: f64 = *var_mexpr_i_rv_slot;
        let mut var_nigbacc_i: f64 = *var_nigbacc_i_slot;
        let mut var_nigbacc_i_rv: f64 = *var_nigbacc_i_rv_slot;
        let mut var_nigbinv_i: f64 = *var_nigbinv_i_slot;
        let mut var_nigbinv_i_rv: f64 = *var_nigbinv_i_rv_slot;
        let mut var_pigcd_i: f64 = *var_pigcd_i_slot;
        let mut var_pigcd_i_rv: f64 = *var_pigcd_i_rv_slot;
        let mut var_poxedge_i: f64 = *var_poxedge_i_slot;
        let mut var_poxedge_i_rv: f64 = *var_poxedge_i_rv_slot;
        let mut var_prwgd_i: f64 = *var_prwgd_i_slot;
        let mut var_prwgd_i_rv: f64 = *var_prwgd_i_rv_slot;
        let mut var_prwgs_i: f64 = *var_prwgs_i_slot;
        let mut var_prwgs_i_rv: f64 = *var_prwgs_i_rv_slot;
        let mut var_ptwg_i: f64 = *var_ptwg_i_slot;
        let mut var_ptwg_i_dn0: f64 = *var_ptwg_i_dn0_slot;
        let mut var_ptwg_i_dn10: f64 = *var_ptwg_i_dn10_slot;
        let mut var_ptwg_i_dn11: f64 = *var_ptwg_i_dn11_slot;
        let mut var_ptwg_i_dn13: f64 = *var_ptwg_i_dn13_slot;
        let mut var_ptwg_i_dn14: f64 = *var_ptwg_i_dn14_slot;
        let mut var_ptwg_i_dn2: f64 = *var_ptwg_i_dn2_slot;
        let mut var_ptwg_i_dn3: f64 = *var_ptwg_i_dn3_slot;
        let mut var_ptwg_i_dn4: f64 = *var_ptwg_i_dn4_slot;
        let mut var_ptwg_i_dn5: f64 = *var_ptwg_i_dn5_slot;
        let mut var_ptwg_i_dn6: f64 = *var_ptwg_i_dn6_slot;
        let mut var_ptwg_i_dn7: f64 = *var_ptwg_i_dn7_slot;
        let mut var_ptwg_i_dn8: f64 = *var_ptwg_i_dn8_slot;
        let mut var_ptwg_i_dn9: f64 = *var_ptwg_i_dn9_slot;
        let mut var_ptwg_i_rv: f64 = *var_ptwg_i_rv_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdw_i_dn0: f64 = *var_rdw_i_dn0_slot;
        let mut var_rdw_i_dn10: f64 = *var_rdw_i_dn10_slot;
        let mut var_rdw_i_dn11: f64 = *var_rdw_i_dn11_slot;
        let mut var_rdw_i_dn13: f64 = *var_rdw_i_dn13_slot;
        let mut var_rdw_i_dn14: f64 = *var_rdw_i_dn14_slot;
        let mut var_rdw_i_dn2: f64 = *var_rdw_i_dn2_slot;
        let mut var_rdw_i_dn3: f64 = *var_rdw_i_dn3_slot;
        let mut var_rdw_i_dn4: f64 = *var_rdw_i_dn4_slot;
        let mut var_rdw_i_dn5: f64 = *var_rdw_i_dn5_slot;
        let mut var_rdw_i_dn6: f64 = *var_rdw_i_dn6_slot;
        let mut var_rdw_i_dn7: f64 = *var_rdw_i_dn7_slot;
        let mut var_rdw_i_dn8: f64 = *var_rdw_i_dn8_slot;
        let mut var_rdw_i_dn9: f64 = *var_rdw_i_dn9_slot;
        let mut var_rdw_i_rv: f64 = *var_rdw_i_rv_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rsw_i_dn0: f64 = *var_rsw_i_dn0_slot;
        let mut var_rsw_i_dn10: f64 = *var_rsw_i_dn10_slot;
        let mut var_rsw_i_dn11: f64 = *var_rsw_i_dn11_slot;
        let mut var_rsw_i_dn13: f64 = *var_rsw_i_dn13_slot;
        let mut var_rsw_i_dn14: f64 = *var_rsw_i_dn14_slot;
        let mut var_rsw_i_dn2: f64 = *var_rsw_i_dn2_slot;
        let mut var_rsw_i_dn3: f64 = *var_rsw_i_dn3_slot;
        let mut var_rsw_i_dn4: f64 = *var_rsw_i_dn4_slot;
        let mut var_rsw_i_dn5: f64 = *var_rsw_i_dn5_slot;
        let mut var_rsw_i_dn6: f64 = *var_rsw_i_dn6_slot;
        let mut var_rsw_i_dn7: f64 = *var_rsw_i_dn7_slot;
        let mut var_rsw_i_dn8: f64 = *var_rsw_i_dn8_slot;
        let mut var_rsw_i_dn9: f64 = *var_rsw_i_dn9_slot;
        let mut var_rsw_i_rv: f64 = *var_rsw_i_rv_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_dn0: f64 = *var_u0r_i_dn0_slot;
        let mut var_u0r_i_dn10: f64 = *var_u0r_i_dn10_slot;
        let mut var_u0r_i_dn11: f64 = *var_u0r_i_dn11_slot;
        let mut var_u0r_i_dn13: f64 = *var_u0r_i_dn13_slot;
        let mut var_u0r_i_dn14: f64 = *var_u0r_i_dn14_slot;
        let mut var_u0r_i_dn2: f64 = *var_u0r_i_dn2_slot;
        let mut var_u0r_i_dn3: f64 = *var_u0r_i_dn3_slot;
        let mut var_u0r_i_dn4: f64 = *var_u0r_i_dn4_slot;
        let mut var_u0r_i_dn5: f64 = *var_u0r_i_dn5_slot;
        let mut var_u0r_i_dn6: f64 = *var_u0r_i_dn6_slot;
        let mut var_u0r_i_dn7: f64 = *var_u0r_i_dn7_slot;
        let mut var_u0r_i_dn8: f64 = *var_u0r_i_dn8_slot;
        let mut var_u0r_i_dn9: f64 = *var_u0r_i_dn9_slot;
        let mut var_u0r_i_rv: f64 = *var_u0r_i_rv_slot;
        let mut var_uar_i: f64 = *var_uar_i_slot;
        let mut var_uar_i_dn0: f64 = *var_uar_i_dn0_slot;
        let mut var_uar_i_dn10: f64 = *var_uar_i_dn10_slot;
        let mut var_uar_i_dn11: f64 = *var_uar_i_dn11_slot;
        let mut var_uar_i_dn13: f64 = *var_uar_i_dn13_slot;
        let mut var_uar_i_dn14: f64 = *var_uar_i_dn14_slot;
        let mut var_uar_i_dn2: f64 = *var_uar_i_dn2_slot;
        let mut var_uar_i_dn3: f64 = *var_uar_i_dn3_slot;
        let mut var_uar_i_dn4: f64 = *var_uar_i_dn4_slot;
        let mut var_uar_i_dn5: f64 = *var_uar_i_dn5_slot;
        let mut var_uar_i_dn6: f64 = *var_uar_i_dn6_slot;
        let mut var_uar_i_dn7: f64 = *var_uar_i_dn7_slot;
        let mut var_uar_i_dn8: f64 = *var_uar_i_dn8_slot;
        let mut var_uar_i_dn9: f64 = *var_uar_i_dn9_slot;
        let mut var_uar_i_rv: f64 = *var_uar_i_rv_slot;
        let mut var_udr_i: f64 = *var_udr_i_slot;
        let mut var_udr_i_dn0: f64 = *var_udr_i_dn0_slot;
        let mut var_udr_i_dn10: f64 = *var_udr_i_dn10_slot;
        let mut var_udr_i_dn11: f64 = *var_udr_i_dn11_slot;
        let mut var_udr_i_dn13: f64 = *var_udr_i_dn13_slot;
        let mut var_udr_i_dn14: f64 = *var_udr_i_dn14_slot;
        let mut var_udr_i_dn2: f64 = *var_udr_i_dn2_slot;
        let mut var_udr_i_dn3: f64 = *var_udr_i_dn3_slot;
        let mut var_udr_i_dn4: f64 = *var_udr_i_dn4_slot;
        let mut var_udr_i_dn5: f64 = *var_udr_i_dn5_slot;
        let mut var_udr_i_dn6: f64 = *var_udr_i_dn6_slot;
        let mut var_udr_i_dn7: f64 = *var_udr_i_dn7_slot;
        let mut var_udr_i_dn8: f64 = *var_udr_i_dn8_slot;
        let mut var_udr_i_dn9: f64 = *var_udr_i_dn9_slot;
        let mut var_udr_i_rv: f64 = *var_udr_i_rv_slot;

        let (assign8030_e11215, assign8030_e11215_d_n0, assign8030_e11215_d_n2, assign8030_e11215_d_n3, assign8030_e11215_d_n4, assign8030_e11215_d_n5, assign8030_e11215_d_n6, assign8030_e11215_d_n7, assign8030_e11215_d_n8, assign8030_e11215_d_n9, assign8030_e11215_d_n10, assign8030_e11215_d_n11, assign8030_e11215_d_n13, assign8030_e11215_d_n14,) = {
    if (var_guard115 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsw_i, var_rsw_i_dn0, var_rsw_i_dn2, var_rsw_i_dn3, var_rsw_i_dn4, var_rsw_i_dn5, var_rsw_i_dn6, var_rsw_i_dn7, var_rsw_i_dn8, var_rsw_i_dn9, var_rsw_i_dn10, var_rsw_i_dn11, var_rsw_i_dn13, var_rsw_i_dn14,)
    }
};
        var_rsw_i = assign8030_e11215;
        var_rsw_i_dn0 = assign8030_e11215_d_n0;
        var_rsw_i_dn2 = assign8030_e11215_d_n2;
        var_rsw_i_dn3 = assign8030_e11215_d_n3;
        var_rsw_i_dn4 = assign8030_e11215_d_n4;
        var_rsw_i_dn5 = assign8030_e11215_d_n5;
        var_rsw_i_dn6 = assign8030_e11215_d_n6;
        var_rsw_i_dn7 = assign8030_e11215_d_n7;
        var_rsw_i_dn8 = assign8030_e11215_d_n8;
        var_rsw_i_dn9 = assign8030_e11215_d_n9;
        var_rsw_i_dn10 = assign8030_e11215_d_n10;
        var_rsw_i_dn11 = assign8030_e11215_d_n11;
        var_rsw_i_dn13 = assign8030_e11215_d_n13;
        var_rsw_i_dn14 = assign8030_e11215_d_n14;
        var_rsw_i_rv = 0.0;

        let assign8040_e11218: f64 = if var_rdw_i < 0.0 { 1.0 } else { 0.0 };
        var_guard116 = assign8040_e11218;
        var_guard116_rv = 0.0;

        let (assign8050_e11222, assign8050_e11222_d_n0, assign8050_e11222_d_n2, assign8050_e11222_d_n3, assign8050_e11222_d_n4, assign8050_e11222_d_n5, assign8050_e11222_d_n6, assign8050_e11222_d_n7, assign8050_e11222_d_n8, assign8050_e11222_d_n9, assign8050_e11222_d_n10, assign8050_e11222_d_n11, assign8050_e11222_d_n13, assign8050_e11222_d_n14,) = {
    if (var_guard116 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdw_i, var_rdw_i_dn0, var_rdw_i_dn2, var_rdw_i_dn3, var_rdw_i_dn4, var_rdw_i_dn5, var_rdw_i_dn6, var_rdw_i_dn7, var_rdw_i_dn8, var_rdw_i_dn9, var_rdw_i_dn10, var_rdw_i_dn11, var_rdw_i_dn13, var_rdw_i_dn14,)
    }
};
        var_rdw_i = assign8050_e11222;
        var_rdw_i_dn0 = assign8050_e11222_d_n0;
        var_rdw_i_dn2 = assign8050_e11222_d_n2;
        var_rdw_i_dn3 = assign8050_e11222_d_n3;
        var_rdw_i_dn4 = assign8050_e11222_d_n4;
        var_rdw_i_dn5 = assign8050_e11222_d_n5;
        var_rdw_i_dn6 = assign8050_e11222_d_n6;
        var_rdw_i_dn7 = assign8050_e11222_d_n7;
        var_rdw_i_dn8 = assign8050_e11222_d_n8;
        var_rdw_i_dn9 = assign8050_e11222_d_n9;
        var_rdw_i_dn10 = assign8050_e11222_d_n10;
        var_rdw_i_dn11 = assign8050_e11222_d_n11;
        var_rdw_i_dn13 = assign8050_e11222_d_n13;
        var_rdw_i_dn14 = assign8050_e11222_d_n14;
        var_rdw_i_rv = 0.0;

        let assign8060_e11225: f64 = if var_prwgd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard117 = assign8060_e11225;
        var_guard117_rv = 0.0;

        let (assign8070_e11229,) = {
    if (var_guard117 != 0.0) {
        (0.0,)
    } else {
        (var_prwgd_i,)
    }
};
        var_prwgd_i = assign8070_e11229;
        var_prwgd_i_rv = 0.0;

        let assign8080_e11232: f64 = if var_prwgs_i < 0.0 { 1.0 } else { 0.0 };
        var_guard118 = assign8080_e11232;
        var_guard118_rv = 0.0;

        let (assign8090_e11236,) = {
    if (var_guard118 != 0.0) {
        (0.0,)
    } else {
        (var_prwgs_i,)
    }
};
        var_prwgs_i = assign8090_e11236;
        var_prwgs_i_rv = 0.0;

        let assign8120_e11245: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard121 = assign8120_e11245;
        var_guard121_rv = 0.0;

        let assign8150_e11254: f64 = if var_u0r_i < 0.0 { 1.0 } else { 0.0 };
        var_guard124 = assign8150_e11254;
        var_guard124_rv = 0.0;

        let (assign8160_e11260, assign8160_e11260_d_n0, assign8160_e11260_d_n2, assign8160_e11260_d_n3, assign8160_e11260_d_n4, assign8160_e11260_d_n5, assign8160_e11260_d_n6, assign8160_e11260_d_n7, assign8160_e11260_d_n8, assign8160_e11260_d_n9, assign8160_e11260_d_n10, assign8160_e11260_d_n11, assign8160_e11260_d_n13, assign8160_e11260_d_n14,) = {
    if ((var_guard121 != 0.0) && (var_guard124 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign8160_e11260;
        var_u0r_i_dn0 = assign8160_e11260_d_n0;
        var_u0r_i_dn2 = assign8160_e11260_d_n2;
        var_u0r_i_dn3 = assign8160_e11260_d_n3;
        var_u0r_i_dn4 = assign8160_e11260_d_n4;
        var_u0r_i_dn5 = assign8160_e11260_d_n5;
        var_u0r_i_dn6 = assign8160_e11260_d_n6;
        var_u0r_i_dn7 = assign8160_e11260_d_n7;
        var_u0r_i_dn8 = assign8160_e11260_d_n8;
        var_u0r_i_dn9 = assign8160_e11260_d_n9;
        var_u0r_i_dn10 = assign8160_e11260_d_n10;
        var_u0r_i_dn11 = assign8160_e11260_d_n11;
        var_u0r_i_dn13 = assign8160_e11260_d_n13;
        var_u0r_i_dn14 = assign8160_e11260_d_n14;
        var_u0r_i_rv = 0.0;

        let assign8170_e11263: f64 = if var_uar_i < 0.0 { 1.0 } else { 0.0 };
        var_guard125 = assign8170_e11263;
        var_guard125_rv = 0.0;

        let (assign8180_e11269, assign8180_e11269_d_n0, assign8180_e11269_d_n2, assign8180_e11269_d_n3, assign8180_e11269_d_n4, assign8180_e11269_d_n5, assign8180_e11269_d_n6, assign8180_e11269_d_n7, assign8180_e11269_d_n8, assign8180_e11269_d_n9, assign8180_e11269_d_n10, assign8180_e11269_d_n11, assign8180_e11269_d_n13, assign8180_e11269_d_n14,) = {
    if ((var_guard121 != 0.0) && (var_guard125 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uar_i, var_uar_i_dn0, var_uar_i_dn2, var_uar_i_dn3, var_uar_i_dn4, var_uar_i_dn5, var_uar_i_dn6, var_uar_i_dn7, var_uar_i_dn8, var_uar_i_dn9, var_uar_i_dn10, var_uar_i_dn11, var_uar_i_dn13, var_uar_i_dn14,)
    }
};
        var_uar_i = assign8180_e11269;
        var_uar_i_dn0 = assign8180_e11269_d_n0;
        var_uar_i_dn2 = assign8180_e11269_d_n2;
        var_uar_i_dn3 = assign8180_e11269_d_n3;
        var_uar_i_dn4 = assign8180_e11269_d_n4;
        var_uar_i_dn5 = assign8180_e11269_d_n5;
        var_uar_i_dn6 = assign8180_e11269_d_n6;
        var_uar_i_dn7 = assign8180_e11269_d_n7;
        var_uar_i_dn8 = assign8180_e11269_d_n8;
        var_uar_i_dn9 = assign8180_e11269_d_n9;
        var_uar_i_dn10 = assign8180_e11269_d_n10;
        var_uar_i_dn11 = assign8180_e11269_d_n11;
        var_uar_i_dn13 = assign8180_e11269_d_n13;
        var_uar_i_dn14 = assign8180_e11269_d_n14;
        var_uar_i_rv = 0.0;

        let assign8190_e11272: f64 = if var_eur_i < 0.0 { 1.0 } else { 0.0 };
        var_guard126 = assign8190_e11272;
        var_guard126_rv = 0.0;

        let (assign8200_e11278, assign8200_e11278_d_n0, assign8200_e11278_d_n2, assign8200_e11278_d_n3, assign8200_e11278_d_n4, assign8200_e11278_d_n5, assign8200_e11278_d_n6, assign8200_e11278_d_n7, assign8200_e11278_d_n8, assign8200_e11278_d_n9, assign8200_e11278_d_n10, assign8200_e11278_d_n11, assign8200_e11278_d_n13, assign8200_e11278_d_n14,) = {
    if ((var_guard121 != 0.0) && (var_guard126 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eur_i, var_eur_i_dn0, var_eur_i_dn2, var_eur_i_dn3, var_eur_i_dn4, var_eur_i_dn5, var_eur_i_dn6, var_eur_i_dn7, var_eur_i_dn8, var_eur_i_dn9, var_eur_i_dn10, var_eur_i_dn11, var_eur_i_dn13, var_eur_i_dn14,)
    }
};
        var_eur_i = assign8200_e11278;
        var_eur_i_dn0 = assign8200_e11278_d_n0;
        var_eur_i_dn2 = assign8200_e11278_d_n2;
        var_eur_i_dn3 = assign8200_e11278_d_n3;
        var_eur_i_dn4 = assign8200_e11278_d_n4;
        var_eur_i_dn5 = assign8200_e11278_d_n5;
        var_eur_i_dn6 = assign8200_e11278_d_n6;
        var_eur_i_dn7 = assign8200_e11278_d_n7;
        var_eur_i_dn8 = assign8200_e11278_d_n8;
        var_eur_i_dn9 = assign8200_e11278_d_n9;
        var_eur_i_dn10 = assign8200_e11278_d_n10;
        var_eur_i_dn11 = assign8200_e11278_d_n11;
        var_eur_i_dn13 = assign8200_e11278_d_n13;
        var_eur_i_dn14 = assign8200_e11278_d_n14;
        var_eur_i_rv = 0.0;

        let assign8210_e11281: f64 = if var_udr_i < 0.0 { 1.0 } else { 0.0 };
        var_guard127 = assign8210_e11281;
        var_guard127_rv = 0.0;

        let (assign8220_e11287, assign8220_e11287_d_n0, assign8220_e11287_d_n2, assign8220_e11287_d_n3, assign8220_e11287_d_n4, assign8220_e11287_d_n5, assign8220_e11287_d_n6, assign8220_e11287_d_n7, assign8220_e11287_d_n8, assign8220_e11287_d_n9, assign8220_e11287_d_n10, assign8220_e11287_d_n11, assign8220_e11287_d_n13, assign8220_e11287_d_n14,) = {
    if ((var_guard121 != 0.0) && (var_guard127 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_udr_i, var_udr_i_dn0, var_udr_i_dn2, var_udr_i_dn3, var_udr_i_dn4, var_udr_i_dn5, var_udr_i_dn6, var_udr_i_dn7, var_udr_i_dn8, var_udr_i_dn9, var_udr_i_dn10, var_udr_i_dn11, var_udr_i_dn13, var_udr_i_dn14,)
    }
};
        var_udr_i = assign8220_e11287;
        var_udr_i_dn0 = assign8220_e11287_d_n0;
        var_udr_i_dn2 = assign8220_e11287_d_n2;
        var_udr_i_dn3 = assign8220_e11287_d_n3;
        var_udr_i_dn4 = assign8220_e11287_d_n4;
        var_udr_i_dn5 = assign8220_e11287_d_n5;
        var_udr_i_dn6 = assign8220_e11287_d_n6;
        var_udr_i_dn7 = assign8220_e11287_d_n7;
        var_udr_i_dn8 = assign8220_e11287_d_n8;
        var_udr_i_dn9 = assign8220_e11287_d_n9;
        var_udr_i_dn10 = assign8220_e11287_d_n10;
        var_udr_i_dn11 = assign8220_e11287_d_n11;
        var_udr_i_dn13 = assign8220_e11287_d_n13;
        var_udr_i_dn14 = assign8220_e11287_d_n14;
        var_udr_i_rv = 0.0;

        let assign8240_e11293: f64 = if var_drout_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard129 = assign8240_e11293;
        var_guard129_rv = 0.0;

        let (assign8250_e11297,) = {
    if (var_guard129 != 0.0) {
        (1.06,)
    } else {
        (var_drout_i,)
    }
};
        var_drout_i = assign8250_e11297;
        var_drout_i_rv = 0.0;

        let assign8260_e11300: f64 = if var_mexp_i < 2.0 { 1.0 } else { 0.0 };
        var_guard130 = assign8260_e11300;
        var_guard130_rv = 0.0;

        let (assign8270_e11304, assign8270_e11304_d_n0, assign8270_e11304_d_n2, assign8270_e11304_d_n3, assign8270_e11304_d_n4, assign8270_e11304_d_n5, assign8270_e11304_d_n6, assign8270_e11304_d_n7, assign8270_e11304_d_n8, assign8270_e11304_d_n9, assign8270_e11304_d_n10, assign8270_e11304_d_n11, assign8270_e11304_d_n13, assign8270_e11304_d_n14,) = {
    if (var_guard130 != 0.0) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mexp_i, var_mexp_i_dn0, var_mexp_i_dn2, var_mexp_i_dn3, var_mexp_i_dn4, var_mexp_i_dn5, var_mexp_i_dn6, var_mexp_i_dn7, var_mexp_i_dn8, var_mexp_i_dn9, var_mexp_i_dn10, var_mexp_i_dn11, var_mexp_i_dn13, var_mexp_i_dn14,)
    }
};
        var_mexp_i = assign8270_e11304;
        var_mexp_i_dn0 = assign8270_e11304_d_n0;
        var_mexp_i_dn2 = assign8270_e11304_d_n2;
        var_mexp_i_dn3 = assign8270_e11304_d_n3;
        var_mexp_i_dn4 = assign8270_e11304_d_n4;
        var_mexp_i_dn5 = assign8270_e11304_d_n5;
        var_mexp_i_dn6 = assign8270_e11304_d_n6;
        var_mexp_i_dn7 = assign8270_e11304_d_n7;
        var_mexp_i_dn8 = assign8270_e11304_d_n8;
        var_mexp_i_dn9 = assign8270_e11304_d_n9;
        var_mexp_i_dn10 = assign8270_e11304_d_n10;
        var_mexp_i_dn11 = assign8270_e11304_d_n11;
        var_mexp_i_dn13 = assign8270_e11304_d_n13;
        var_mexp_i_dn14 = assign8270_e11304_d_n14;
        var_mexp_i_rv = 0.0;

        let assign8280_e11307: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard131 = assign8280_e11307;
        var_guard131_rv = 0.0;

        let assign8290_e11310: f64 = if var_mexpr_i < 2.0 { 1.0 } else { 0.0 };
        var_guard132 = assign8290_e11310;
        var_guard132_rv = 0.0;

        let (assign8300_e11316, assign8300_e11316_d_n0, assign8300_e11316_d_n2, assign8300_e11316_d_n3, assign8300_e11316_d_n4, assign8300_e11316_d_n5, assign8300_e11316_d_n6, assign8300_e11316_d_n7, assign8300_e11316_d_n8, assign8300_e11316_d_n9, assign8300_e11316_d_n10, assign8300_e11316_d_n11, assign8300_e11316_d_n13, assign8300_e11316_d_n14,) = {
    if ((var_guard131 != 0.0) && (var_guard132 != 0.0)) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mexpr_i, var_mexpr_i_dn0, var_mexpr_i_dn2, var_mexpr_i_dn3, var_mexpr_i_dn4, var_mexpr_i_dn5, var_mexpr_i_dn6, var_mexpr_i_dn7, var_mexpr_i_dn8, var_mexpr_i_dn9, var_mexpr_i_dn10, var_mexpr_i_dn11, var_mexpr_i_dn13, var_mexpr_i_dn14,)
    }
};
        var_mexpr_i = assign8300_e11316;
        var_mexpr_i_dn0 = assign8300_e11316_d_n0;
        var_mexpr_i_dn2 = assign8300_e11316_d_n2;
        var_mexpr_i_dn3 = assign8300_e11316_d_n3;
        var_mexpr_i_dn4 = assign8300_e11316_d_n4;
        var_mexpr_i_dn5 = assign8300_e11316_d_n5;
        var_mexpr_i_dn6 = assign8300_e11316_d_n6;
        var_mexpr_i_dn7 = assign8300_e11316_d_n7;
        var_mexpr_i_dn8 = assign8300_e11316_d_n8;
        var_mexpr_i_dn9 = assign8300_e11316_d_n9;
        var_mexpr_i_dn10 = assign8300_e11316_d_n10;
        var_mexpr_i_dn11 = assign8300_e11316_d_n11;
        var_mexpr_i_dn13 = assign8300_e11316_d_n13;
        var_mexpr_i_dn14 = assign8300_e11316_d_n14;
        var_mexpr_i_rv = 0.0;

        let assign8310_e11319: f64 = if var_ptwg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard133 = assign8310_e11319;
        var_guard133_rv = 0.0;

        let (assign8320_e11323, assign8320_e11323_d_n0, assign8320_e11323_d_n2, assign8320_e11323_d_n3, assign8320_e11323_d_n4, assign8320_e11323_d_n5, assign8320_e11323_d_n6, assign8320_e11323_d_n7, assign8320_e11323_d_n8, assign8320_e11323_d_n9, assign8320_e11323_d_n10, assign8320_e11323_d_n11, assign8320_e11323_d_n13, assign8320_e11323_d_n14,) = {
    if (var_guard133 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptwg_i, var_ptwg_i_dn0, var_ptwg_i_dn2, var_ptwg_i_dn3, var_ptwg_i_dn4, var_ptwg_i_dn5, var_ptwg_i_dn6, var_ptwg_i_dn7, var_ptwg_i_dn8, var_ptwg_i_dn9, var_ptwg_i_dn10, var_ptwg_i_dn11, var_ptwg_i_dn13, var_ptwg_i_dn14,)
    }
};
        var_ptwg_i = assign8320_e11323;
        var_ptwg_i_dn0 = assign8320_e11323_d_n0;
        var_ptwg_i_dn2 = assign8320_e11323_d_n2;
        var_ptwg_i_dn3 = assign8320_e11323_d_n3;
        var_ptwg_i_dn4 = assign8320_e11323_d_n4;
        var_ptwg_i_dn5 = assign8320_e11323_d_n5;
        var_ptwg_i_dn6 = assign8320_e11323_d_n6;
        var_ptwg_i_dn7 = assign8320_e11323_d_n7;
        var_ptwg_i_dn8 = assign8320_e11323_d_n8;
        var_ptwg_i_dn9 = assign8320_e11323_d_n9;
        var_ptwg_i_dn10 = assign8320_e11323_d_n10;
        var_ptwg_i_dn11 = assign8320_e11323_d_n11;
        var_ptwg_i_dn13 = assign8320_e11323_d_n13;
        var_ptwg_i_dn14 = assign8320_e11323_d_n14;
        var_ptwg_i_rv = 0.0;

        let assign8330_e11326: f64 = if var_cgidl_i < 0.0 { 1.0 } else { 0.0 };
        var_guard134 = assign8330_e11326;
        var_guard134_rv = 0.0;

        let (assign8340_e11330,) = {
    if (var_guard134 != 0.0) {
        (0.0,)
    } else {
        (var_cgidl_i,)
    }
};
        var_cgidl_i = assign8340_e11330;
        var_cgidl_i_rv = 0.0;

        let assign8350_e11333: f64 = if var_cgisl_i < 0.0 { 1.0 } else { 0.0 };
        var_guard135 = assign8350_e11333;
        var_guard135_rv = 0.0;

        let (assign8360_e11337,) = {
    if (var_guard135 != 0.0) {
        (0.0,)
    } else {
        (var_cgisl_i,)
    }
};
        var_cgisl_i = assign8360_e11337;
        var_cgisl_i_rv = 0.0;

        let assign8370_e11340: f64 = if p.p69 != 0.0 { 1.0 } else { 0.0 };
        var_guard136 = assign8370_e11340;
        var_guard136_rv = 0.0;

        let assign8380_e11343: f64 = if var_nigbinv_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard137 = assign8380_e11343;
        var_guard137_rv = 0.0;

        let (assign8390_e11349,) = {
    if ((var_guard136 != 0.0) && (var_guard137 != 0.0)) {
        (3.0,)
    } else {
        (var_nigbinv_i,)
    }
};
        var_nigbinv_i = assign8390_e11349;
        var_nigbinv_i_rv = 0.0;

        let assign8400_e11352: f64 = if var_nigbacc_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard138 = assign8400_e11352;
        var_guard138_rv = 0.0;

        let (assign8410_e11358,) = {
    if ((var_guard136 != 0.0) && (var_guard138 != 0.0)) {
        (1.0,)
    } else {
        (var_nigbacc_i,)
    }
};
        var_nigbacc_i = assign8410_e11358;
        var_nigbacc_i_rv = 0.0;

        let assign8420_e11361: f64 = if p.p68 != 0.0 { 1.0 } else { 0.0 };
        var_guard139 = assign8420_e11361;
        var_guard139_rv = 0.0;

        let assign8430_e11364: f64 = if var_poxedge_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard140 = assign8430_e11364;
        var_guard140_rv = 0.0;

        let (assign8440_e11370,) = {
    if ((var_guard139 != 0.0) && (var_guard140 != 0.0)) {
        (1.0,)
    } else {
        (var_poxedge_i,)
    }
};
        var_poxedge_i = assign8440_e11370;
        var_poxedge_i_rv = 0.0;

        let assign8450_e11373: f64 = if var_pigcd_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard141 = assign8450_e11373;
        var_guard141_rv = 0.0;

        let (assign8460_e11379,) = {
    if ((var_guard139 != 0.0) && (var_guard141 != 0.0)) {
        (1.0,)
    } else {
        (var_pigcd_i,)
    }
};
        var_pigcd_i = assign8460_e11379;
        var_pigcd_i_rv = 0.0;

        let assign8700_e11473: f64 = if var_cgsl_i < 0.0 { 1.0 } else { 0.0 };
        var_guard159 = assign8700_e11473;
        var_guard159_rv = 0.0;

        let (assign8710_e11477,) = {
    if (var_guard159 != 0.0) {
        (0.0,)
    } else {
        (var_cgsl_i,)
    }
};
        var_cgsl_i = assign8710_e11477;
        var_cgsl_i_rv = 0.0;

        let assign8720_e11480: f64 = if var_cgdl_i < 0.0 { 1.0 } else { 0.0 };
        var_guard160 = assign8720_e11480;
        var_guard160_rv = 0.0;

        let (assign8730_e11484,) = {
    if (var_guard160 != 0.0) {
        (0.0,)
    } else {
        (var_cgdl_i,)
    }
};
        var_cgdl_i = assign8730_e11484;
        var_cgdl_i_rv = 0.0;

        let assign8740_e11487: f64 = if var_cfs_i < 0.0 { 1.0 } else { 0.0 };
        var_guard161 = assign8740_e11487;
        var_guard161_rv = 0.0;

        let (assign8750_e11491,) = {
    if (var_guard161 != 0.0) {
        (0.0,)
    } else {
        (var_cfs_i,)
    }
};
        var_cfs_i = assign8750_e11491;
        var_cfs_i_rv = 0.0;

        let assign8760_e11494: f64 = if var_cfd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard162 = assign8760_e11494;
        var_guard162_rv = 0.0;

        let (assign8770_e11498,) = {
    if (var_guard162 != 0.0) {
        (0.0,)
    } else {
        (var_cfd_i,)
    }
};
        var_cfd_i = assign8770_e11498;
        var_cfd_i_rv = 0.0;

        let assign8780_e11501: f64 = if var_cgbl_i < 0.0 { 1.0 } else { 0.0 };
        var_guard163 = assign8780_e11501;
        var_guard163_rv = 0.0;

        let (assign8790_e11505,) = {
    if (var_guard163 != 0.0) {
        (0.0,)
    } else {
        (var_cgbl_i,)
    }
};
        var_cgbl_i = assign8790_e11505;
        var_cgbl_i_rv = 0.0;

        let assign8800_e11508: f64 = if var_ckappas_i <= 0.02 { 1.0 } else { 0.0 };
        var_guard164 = assign8800_e11508;
        var_guard164_rv = 0.0;

        let (assign8810_e11512,) = {
    if (var_guard164 != 0.0) {
        (0.02,)
    } else {
        (var_ckappas_i,)
    }
};
        var_ckappas_i = assign8810_e11512;
        var_ckappas_i_rv = 0.0;

        let assign8820_e11515: f64 = if var_ckappad_i <= 0.02 { 1.0 } else { 0.0 };
        var_guard165 = assign8820_e11515;
        var_guard165_rv = 0.0;

        let (assign8830_e11519,) = {
    if (var_guard165 != 0.0) {
        (0.02,)
    } else {
        (var_ckappad_i,)
    }
};
        var_ckappad_i = assign8830_e11519;
        var_ckappad_i_rv = 0.0;

        let assign8840_e11522: f64 = if var_ckappab_i <= 0.02 { 1.0 } else { 0.0 };
        var_guard166 = assign8840_e11522;
        var_guard166_rv = 0.0;

        let (assign8850_e11526,) = {
    if (var_guard166 != 0.0) {
        (0.02,)
    } else {
        (var_ckappab_i,)
    }
};
        var_ckappab_i = assign8850_e11526;
        var_ckappab_i_rv = 0.0;

        let assign8860_e11529: f64 = (-p.p4);
        let assign8860_e11530: f64 = if var_deltaprsd_v < assign8860_e11529 { 1.0 } else { 0.0 };
        var_guard167 = assign8860_e11530;
        var_guard167_rv = 0.0;

        let (assign8870_e11534,) = {
    if (var_guard167 != 0.0) {
        (0.0,)
    } else {
        (var_deltaprsd_v,)
    }
};
        var_deltaprsd_v = assign8870_e11534;
        var_deltaprsd_v_rv = 0.0;

        let assign8880_e11537: f64 = if p.p57 == 1.0 { 1.0 } else { 0.0 };
        var_guard168 = assign8880_e11537;
        var_guard168_rv = 0.0;

        *var_cfd_i_slot = var_cfd_i;
        *var_cfd_i_rv_slot = var_cfd_i_rv;
        *var_cfs_i_slot = var_cfs_i;
        *var_cfs_i_rv_slot = var_cfs_i_rv;
        *var_cgbl_i_slot = var_cgbl_i;
        *var_cgbl_i_rv_slot = var_cgbl_i_rv;
        *var_cgdl_i_slot = var_cgdl_i;
        *var_cgdl_i_rv_slot = var_cgdl_i_rv;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidl_i_rv_slot = var_cgidl_i_rv;
        *var_cgisl_i_slot = var_cgisl_i;
        *var_cgisl_i_rv_slot = var_cgisl_i_rv;
        *var_cgsl_i_slot = var_cgsl_i;
        *var_cgsl_i_rv_slot = var_cgsl_i_rv;
        *var_ckappab_i_slot = var_ckappab_i;
        *var_ckappab_i_rv_slot = var_ckappab_i_rv;
        *var_ckappad_i_slot = var_ckappad_i;
        *var_ckappad_i_rv_slot = var_ckappad_i_rv;
        *var_ckappas_i_slot = var_ckappas_i;
        *var_ckappas_i_rv_slot = var_ckappas_i_rv;
        *var_deltaprsd_v_slot = var_deltaprsd_v;
        *var_deltaprsd_v_rv_slot = var_deltaprsd_v_rv;
        *var_drout_i_slot = var_drout_i;
        *var_drout_i_rv_slot = var_drout_i_rv;
        *var_eur_i_slot = var_eur_i;
        *var_eur_i_dn0_slot = var_eur_i_dn0;
        *var_eur_i_dn10_slot = var_eur_i_dn10;
        *var_eur_i_dn11_slot = var_eur_i_dn11;
        *var_eur_i_dn13_slot = var_eur_i_dn13;
        *var_eur_i_dn14_slot = var_eur_i_dn14;
        *var_eur_i_dn2_slot = var_eur_i_dn2;
        *var_eur_i_dn3_slot = var_eur_i_dn3;
        *var_eur_i_dn4_slot = var_eur_i_dn4;
        *var_eur_i_dn5_slot = var_eur_i_dn5;
        *var_eur_i_dn6_slot = var_eur_i_dn6;
        *var_eur_i_dn7_slot = var_eur_i_dn7;
        *var_eur_i_dn8_slot = var_eur_i_dn8;
        *var_eur_i_dn9_slot = var_eur_i_dn9;
        *var_eur_i_rv_slot = var_eur_i_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard117_slot = var_guard117;
        *var_guard117_rv_slot = var_guard117_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_guard124_slot = var_guard124;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_guard126_slot = var_guard126;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard129_slot = var_guard129;
        *var_guard129_rv_slot = var_guard129_rv;
        *var_guard130_slot = var_guard130;
        *var_guard130_rv_slot = var_guard130_rv;
        *var_guard131_slot = var_guard131;
        *var_guard131_rv_slot = var_guard131_rv;
        *var_guard132_slot = var_guard132;
        *var_guard132_rv_slot = var_guard132_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_guard134_slot = var_guard134;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_guard135_slot = var_guard135;
        *var_guard135_rv_slot = var_guard135_rv;
        *var_guard136_slot = var_guard136;
        *var_guard136_rv_slot = var_guard136_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_guard140_slot = var_guard140;
        *var_guard140_rv_slot = var_guard140_rv;
        *var_guard141_slot = var_guard141;
        *var_guard141_rv_slot = var_guard141_rv;
        *var_guard159_slot = var_guard159;
        *var_guard159_rv_slot = var_guard159_rv;
        *var_guard160_slot = var_guard160;
        *var_guard160_rv_slot = var_guard160_rv;
        *var_guard161_slot = var_guard161;
        *var_guard161_rv_slot = var_guard161_rv;
        *var_guard162_slot = var_guard162;
        *var_guard162_rv_slot = var_guard162_rv;
        *var_guard163_slot = var_guard163;
        *var_guard163_rv_slot = var_guard163_rv;
        *var_guard164_slot = var_guard164;
        *var_guard164_rv_slot = var_guard164_rv;
        *var_guard165_slot = var_guard165;
        *var_guard165_rv_slot = var_guard165_rv;
        *var_guard166_slot = var_guard166;
        *var_guard166_rv_slot = var_guard166_rv;
        *var_guard167_slot = var_guard167;
        *var_guard167_rv_slot = var_guard167_rv;
        *var_guard168_slot = var_guard168;
        *var_guard168_rv_slot = var_guard168_rv;
        *var_mexp_i_slot = var_mexp_i;
        *var_mexp_i_dn0_slot = var_mexp_i_dn0;
        *var_mexp_i_dn10_slot = var_mexp_i_dn10;
        *var_mexp_i_dn11_slot = var_mexp_i_dn11;
        *var_mexp_i_dn13_slot = var_mexp_i_dn13;
        *var_mexp_i_dn14_slot = var_mexp_i_dn14;
        *var_mexp_i_dn2_slot = var_mexp_i_dn2;
        *var_mexp_i_dn3_slot = var_mexp_i_dn3;
        *var_mexp_i_dn4_slot = var_mexp_i_dn4;
        *var_mexp_i_dn5_slot = var_mexp_i_dn5;
        *var_mexp_i_dn6_slot = var_mexp_i_dn6;
        *var_mexp_i_dn7_slot = var_mexp_i_dn7;
        *var_mexp_i_dn8_slot = var_mexp_i_dn8;
        *var_mexp_i_dn9_slot = var_mexp_i_dn9;
        *var_mexp_i_rv_slot = var_mexp_i_rv;
        *var_mexpr_i_slot = var_mexpr_i;
        *var_mexpr_i_dn0_slot = var_mexpr_i_dn0;
        *var_mexpr_i_dn10_slot = var_mexpr_i_dn10;
        *var_mexpr_i_dn11_slot = var_mexpr_i_dn11;
        *var_mexpr_i_dn13_slot = var_mexpr_i_dn13;
        *var_mexpr_i_dn14_slot = var_mexpr_i_dn14;
        *var_mexpr_i_dn2_slot = var_mexpr_i_dn2;
        *var_mexpr_i_dn3_slot = var_mexpr_i_dn3;
        *var_mexpr_i_dn4_slot = var_mexpr_i_dn4;
        *var_mexpr_i_dn5_slot = var_mexpr_i_dn5;
        *var_mexpr_i_dn6_slot = var_mexpr_i_dn6;
        *var_mexpr_i_dn7_slot = var_mexpr_i_dn7;
        *var_mexpr_i_dn8_slot = var_mexpr_i_dn8;
        *var_mexpr_i_dn9_slot = var_mexpr_i_dn9;
        *var_mexpr_i_rv_slot = var_mexpr_i_rv;
        *var_nigbacc_i_slot = var_nigbacc_i;
        *var_nigbacc_i_rv_slot = var_nigbacc_i_rv;
        *var_nigbinv_i_slot = var_nigbinv_i;
        *var_nigbinv_i_rv_slot = var_nigbinv_i_rv;
        *var_pigcd_i_slot = var_pigcd_i;
        *var_pigcd_i_rv_slot = var_pigcd_i_rv;
        *var_poxedge_i_slot = var_poxedge_i;
        *var_poxedge_i_rv_slot = var_poxedge_i_rv;
        *var_prwgd_i_slot = var_prwgd_i;
        *var_prwgd_i_rv_slot = var_prwgd_i_rv;
        *var_prwgs_i_slot = var_prwgs_i;
        *var_prwgs_i_rv_slot = var_prwgs_i_rv;
        *var_ptwg_i_slot = var_ptwg_i;
        *var_ptwg_i_dn0_slot = var_ptwg_i_dn0;
        *var_ptwg_i_dn10_slot = var_ptwg_i_dn10;
        *var_ptwg_i_dn11_slot = var_ptwg_i_dn11;
        *var_ptwg_i_dn13_slot = var_ptwg_i_dn13;
        *var_ptwg_i_dn14_slot = var_ptwg_i_dn14;
        *var_ptwg_i_dn2_slot = var_ptwg_i_dn2;
        *var_ptwg_i_dn3_slot = var_ptwg_i_dn3;
        *var_ptwg_i_dn4_slot = var_ptwg_i_dn4;
        *var_ptwg_i_dn5_slot = var_ptwg_i_dn5;
        *var_ptwg_i_dn6_slot = var_ptwg_i_dn6;
        *var_ptwg_i_dn7_slot = var_ptwg_i_dn7;
        *var_ptwg_i_dn8_slot = var_ptwg_i_dn8;
        *var_ptwg_i_dn9_slot = var_ptwg_i_dn9;
        *var_ptwg_i_rv_slot = var_ptwg_i_rv;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdw_i_dn0_slot = var_rdw_i_dn0;
        *var_rdw_i_dn10_slot = var_rdw_i_dn10;
        *var_rdw_i_dn11_slot = var_rdw_i_dn11;
        *var_rdw_i_dn13_slot = var_rdw_i_dn13;
        *var_rdw_i_dn14_slot = var_rdw_i_dn14;
        *var_rdw_i_dn2_slot = var_rdw_i_dn2;
        *var_rdw_i_dn3_slot = var_rdw_i_dn3;
        *var_rdw_i_dn4_slot = var_rdw_i_dn4;
        *var_rdw_i_dn5_slot = var_rdw_i_dn5;
        *var_rdw_i_dn6_slot = var_rdw_i_dn6;
        *var_rdw_i_dn7_slot = var_rdw_i_dn7;
        *var_rdw_i_dn8_slot = var_rdw_i_dn8;
        *var_rdw_i_dn9_slot = var_rdw_i_dn9;
        *var_rdw_i_rv_slot = var_rdw_i_rv;
        *var_rsw_i_slot = var_rsw_i;
        *var_rsw_i_dn0_slot = var_rsw_i_dn0;
        *var_rsw_i_dn10_slot = var_rsw_i_dn10;
        *var_rsw_i_dn11_slot = var_rsw_i_dn11;
        *var_rsw_i_dn13_slot = var_rsw_i_dn13;
        *var_rsw_i_dn14_slot = var_rsw_i_dn14;
        *var_rsw_i_dn2_slot = var_rsw_i_dn2;
        *var_rsw_i_dn3_slot = var_rsw_i_dn3;
        *var_rsw_i_dn4_slot = var_rsw_i_dn4;
        *var_rsw_i_dn5_slot = var_rsw_i_dn5;
        *var_rsw_i_dn6_slot = var_rsw_i_dn6;
        *var_rsw_i_dn7_slot = var_rsw_i_dn7;
        *var_rsw_i_dn8_slot = var_rsw_i_dn8;
        *var_rsw_i_dn9_slot = var_rsw_i_dn9;
        *var_rsw_i_rv_slot = var_rsw_i_rv;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_dn0_slot = var_u0r_i_dn0;
        *var_u0r_i_dn10_slot = var_u0r_i_dn10;
        *var_u0r_i_dn11_slot = var_u0r_i_dn11;
        *var_u0r_i_dn13_slot = var_u0r_i_dn13;
        *var_u0r_i_dn14_slot = var_u0r_i_dn14;
        *var_u0r_i_dn2_slot = var_u0r_i_dn2;
        *var_u0r_i_dn3_slot = var_u0r_i_dn3;
        *var_u0r_i_dn4_slot = var_u0r_i_dn4;
        *var_u0r_i_dn5_slot = var_u0r_i_dn5;
        *var_u0r_i_dn6_slot = var_u0r_i_dn6;
        *var_u0r_i_dn7_slot = var_u0r_i_dn7;
        *var_u0r_i_dn8_slot = var_u0r_i_dn8;
        *var_u0r_i_dn9_slot = var_u0r_i_dn9;
        *var_u0r_i_rv_slot = var_u0r_i_rv;
        *var_uar_i_slot = var_uar_i;
        *var_uar_i_dn0_slot = var_uar_i_dn0;
        *var_uar_i_dn10_slot = var_uar_i_dn10;
        *var_uar_i_dn11_slot = var_uar_i_dn11;
        *var_uar_i_dn13_slot = var_uar_i_dn13;
        *var_uar_i_dn14_slot = var_uar_i_dn14;
        *var_uar_i_dn2_slot = var_uar_i_dn2;
        *var_uar_i_dn3_slot = var_uar_i_dn3;
        *var_uar_i_dn4_slot = var_uar_i_dn4;
        *var_uar_i_dn5_slot = var_uar_i_dn5;
        *var_uar_i_dn6_slot = var_uar_i_dn6;
        *var_uar_i_dn7_slot = var_uar_i_dn7;
        *var_uar_i_dn8_slot = var_uar_i_dn8;
        *var_uar_i_dn9_slot = var_uar_i_dn9;
        *var_uar_i_rv_slot = var_uar_i_rv;
        *var_udr_i_slot = var_udr_i;
        *var_udr_i_dn0_slot = var_udr_i_dn0;
        *var_udr_i_dn10_slot = var_udr_i_dn10;
        *var_udr_i_dn11_slot = var_udr_i_dn11;
        *var_udr_i_dn13_slot = var_udr_i_dn13;
        *var_udr_i_dn14_slot = var_udr_i_dn14;
        *var_udr_i_dn2_slot = var_udr_i_dn2;
        *var_udr_i_dn3_slot = var_udr_i_dn3;
        *var_udr_i_dn4_slot = var_udr_i_dn4;
        *var_udr_i_dn5_slot = var_udr_i_dn5;
        *var_udr_i_dn6_slot = var_udr_i_dn6;
        *var_udr_i_dn7_slot = var_udr_i_dn7;
        *var_udr_i_dn8_slot = var_udr_i_dn8;
        *var_udr_i_dn9_slot = var_udr_i_dn9;
        *var_udr_i_rv_slot = var_udr_i_rv;
    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_deltaprsd_v: f64,
        var_guard168: f64,
        var_nfintotal: f64,
        var_arsd_slot: &mut f64,
        var_arsd_rv_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_cth_dn0_slot: &mut f64,
        var_cth_dn10_slot: &mut f64,
        var_cth_dn11_slot: &mut f64,
        var_cth_dn13_slot: &mut f64,
        var_cth_dn14_slot: &mut f64,
        var_cth_dn2_slot: &mut f64,
        var_cth_dn3_slot: &mut f64,
        var_cth_dn4_slot: &mut f64,
        var_cth_dn5_slot: &mut f64,
        var_cth_dn6_slot: &mut f64,
        var_cth_dn7_slot: &mut f64,
        var_cth_dn8_slot: &mut f64,
        var_cth_dn9_slot: &mut f64,
        var_cth_rv_slot: &mut f64,
        var_dimension1_i_slot: &mut f64,
        var_dimension1_i_rv_slot: &mut f64,
        var_dimension2_i_slot: &mut f64,
        var_dimension2_i_rv_slot: &mut f64,
        var_dimension3_i_slot: &mut f64,
        var_dimension3_i_rv_slot: &mut f64,
        var_e2nom_i_slot: &mut f64,
        var_e2nom_i_rv_slot: &mut f64,
        var_e3nom_i_slot: &mut f64,
        var_e3nom_i_rv_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_guard169_rv_slot: &mut f64,
        var_guard170_slot: &mut f64,
        var_guard170_rv_slot: &mut f64,
        var_guard171_slot: &mut f64,
        var_guard171_rv_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard172_rv_slot: &mut f64,
        var_guard173_slot: &mut f64,
        var_guard173_rv_slot: &mut f64,
        var_guard174_slot: &mut f64,
        var_guard174_rv_slot: &mut f64,
        var_guard175_slot: &mut f64,
        var_guard175_rv_slot: &mut f64,
        var_guard176_slot: &mut f64,
        var_guard176_rv_slot: &mut f64,
        var_guard177_slot: &mut f64,
        var_guard177_rv_slot: &mut f64,
        var_guard178_slot: &mut f64,
        var_guard178_rv_slot: &mut f64,
        var_guard179_slot: &mut f64,
        var_guard179_rv_slot: &mut f64,
        var_guard180_slot: &mut f64,
        var_guard180_rv_slot: &mut f64,
        var_guard181_slot: &mut f64,
        var_guard181_rv_slot: &mut f64,
        var_guard182_slot: &mut f64,
        var_guard182_rv_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard183_rv_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard184_rv_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard187_rv_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard188_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_mfq1nom_i_slot: &mut f64,
        var_mfq1nom_i_rv_slot: &mut f64,
        var_mfq2nom_i_slot: &mut f64,
        var_mfq2nom_i_rv_slot: &mut f64,
        var_mfq3nom_i_slot: &mut f64,
        var_mfq3nom_i_rv_slot: &mut f64,
        var_prsd_slot: &mut f64,
        var_prsd_rv_slot: &mut f64,
        var_rdraingeo_slot: &mut f64,
        var_rdraingeo_dn0_slot: &mut f64,
        var_rdraingeo_dn10_slot: &mut f64,
        var_rdraingeo_dn11_slot: &mut f64,
        var_rdraingeo_dn13_slot: &mut f64,
        var_rdraingeo_dn14_slot: &mut f64,
        var_rdraingeo_dn2_slot: &mut f64,
        var_rdraingeo_dn3_slot: &mut f64,
        var_rdraingeo_dn4_slot: &mut f64,
        var_rdraingeo_dn5_slot: &mut f64,
        var_rdraingeo_dn6_slot: &mut f64,
        var_rdraingeo_dn7_slot: &mut f64,
        var_rdraingeo_dn8_slot: &mut f64,
        var_rdraingeo_dn9_slot: &mut f64,
        var_rdraingeo_rv_slot: &mut f64,
        var_rhorsd_v_slot: &mut f64,
        var_rhorsd_v_dn0_slot: &mut f64,
        var_rhorsd_v_dn10_slot: &mut f64,
        var_rhorsd_v_dn11_slot: &mut f64,
        var_rhorsd_v_dn13_slot: &mut f64,
        var_rhorsd_v_dn14_slot: &mut f64,
        var_rhorsd_v_dn2_slot: &mut f64,
        var_rhorsd_v_dn3_slot: &mut f64,
        var_rhorsd_v_dn4_slot: &mut f64,
        var_rhorsd_v_dn5_slot: &mut f64,
        var_rhorsd_v_dn6_slot: &mut f64,
        var_rhorsd_v_dn7_slot: &mut f64,
        var_rhorsd_v_dn8_slot: &mut f64,
        var_rhorsd_v_dn9_slot: &mut f64,
        var_rhorsd_v_rv_slot: &mut f64,
        var_rsourcegeo_slot: &mut f64,
        var_rsourcegeo_dn0_slot: &mut f64,
        var_rsourcegeo_dn10_slot: &mut f64,
        var_rsourcegeo_dn11_slot: &mut f64,
        var_rsourcegeo_dn13_slot: &mut f64,
        var_rsourcegeo_dn14_slot: &mut f64,
        var_rsourcegeo_dn2_slot: &mut f64,
        var_rsourcegeo_dn3_slot: &mut f64,
        var_rsourcegeo_dn4_slot: &mut f64,
        var_rsourcegeo_dn5_slot: &mut f64,
        var_rsourcegeo_dn6_slot: &mut f64,
        var_rsourcegeo_dn7_slot: &mut f64,
        var_rsourcegeo_dn8_slot: &mut f64,
        var_rsourcegeo_dn9_slot: &mut f64,
        var_rsourcegeo_rv_slot: &mut f64,
        var_ssp1_i_slot: &mut f64,
        var_ssp1_i_rv_slot: &mut f64,
        var_ssp2_i_slot: &mut f64,
        var_ssp2_i_rv_slot: &mut f64,
        var_ssp3_i_slot: &mut f64,
        var_ssp3_i_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
    ) {
        let mut var_arsd: f64 = *var_arsd_slot;
        let mut var_arsd_rv: f64 = *var_arsd_rv_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_cth_dn0: f64 = *var_cth_dn0_slot;
        let mut var_cth_dn10: f64 = *var_cth_dn10_slot;
        let mut var_cth_dn11: f64 = *var_cth_dn11_slot;
        let mut var_cth_dn13: f64 = *var_cth_dn13_slot;
        let mut var_cth_dn14: f64 = *var_cth_dn14_slot;
        let mut var_cth_dn2: f64 = *var_cth_dn2_slot;
        let mut var_cth_dn3: f64 = *var_cth_dn3_slot;
        let mut var_cth_dn4: f64 = *var_cth_dn4_slot;
        let mut var_cth_dn5: f64 = *var_cth_dn5_slot;
        let mut var_cth_dn6: f64 = *var_cth_dn6_slot;
        let mut var_cth_dn7: f64 = *var_cth_dn7_slot;
        let mut var_cth_dn8: f64 = *var_cth_dn8_slot;
        let mut var_cth_dn9: f64 = *var_cth_dn9_slot;
        let mut var_cth_rv: f64 = *var_cth_rv_slot;
        let mut var_dimension1_i: f64 = *var_dimension1_i_slot;
        let mut var_dimension1_i_rv: f64 = *var_dimension1_i_rv_slot;
        let mut var_dimension2_i: f64 = *var_dimension2_i_slot;
        let mut var_dimension2_i_rv: f64 = *var_dimension2_i_rv_slot;
        let mut var_dimension3_i: f64 = *var_dimension3_i_slot;
        let mut var_dimension3_i_rv: f64 = *var_dimension3_i_rv_slot;
        let mut var_e2nom_i: f64 = *var_e2nom_i_slot;
        let mut var_e2nom_i_rv: f64 = *var_e2nom_i_rv_slot;
        let mut var_e3nom_i: f64 = *var_e3nom_i_slot;
        let mut var_e3nom_i_rv: f64 = *var_e3nom_i_rv_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_guard169_rv: f64 = *var_guard169_rv_slot;
        let mut var_guard170: f64 = *var_guard170_slot;
        let mut var_guard170_rv: f64 = *var_guard170_rv_slot;
        let mut var_guard171: f64 = *var_guard171_slot;
        let mut var_guard171_rv: f64 = *var_guard171_rv_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard172_rv: f64 = *var_guard172_rv_slot;
        let mut var_guard173: f64 = *var_guard173_slot;
        let mut var_guard173_rv: f64 = *var_guard173_rv_slot;
        let mut var_guard174: f64 = *var_guard174_slot;
        let mut var_guard174_rv: f64 = *var_guard174_rv_slot;
        let mut var_guard175: f64 = *var_guard175_slot;
        let mut var_guard175_rv: f64 = *var_guard175_rv_slot;
        let mut var_guard176: f64 = *var_guard176_slot;
        let mut var_guard176_rv: f64 = *var_guard176_rv_slot;
        let mut var_guard177: f64 = *var_guard177_slot;
        let mut var_guard177_rv: f64 = *var_guard177_rv_slot;
        let mut var_guard178: f64 = *var_guard178_slot;
        let mut var_guard178_rv: f64 = *var_guard178_rv_slot;
        let mut var_guard179: f64 = *var_guard179_slot;
        let mut var_guard179_rv: f64 = *var_guard179_rv_slot;
        let mut var_guard180: f64 = *var_guard180_slot;
        let mut var_guard180_rv: f64 = *var_guard180_rv_slot;
        let mut var_guard181: f64 = *var_guard181_slot;
        let mut var_guard181_rv: f64 = *var_guard181_rv_slot;
        let mut var_guard182: f64 = *var_guard182_slot;
        let mut var_guard182_rv: f64 = *var_guard182_rv_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard183_rv: f64 = *var_guard183_rv_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard184_rv: f64 = *var_guard184_rv_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard187_rv: f64 = *var_guard187_rv_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard188_rv: f64 = *var_guard188_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_mfq1nom_i: f64 = *var_mfq1nom_i_slot;
        let mut var_mfq1nom_i_rv: f64 = *var_mfq1nom_i_rv_slot;
        let mut var_mfq2nom_i: f64 = *var_mfq2nom_i_slot;
        let mut var_mfq2nom_i_rv: f64 = *var_mfq2nom_i_rv_slot;
        let mut var_mfq3nom_i: f64 = *var_mfq3nom_i_slot;
        let mut var_mfq3nom_i_rv: f64 = *var_mfq3nom_i_rv_slot;
        let mut var_prsd: f64 = *var_prsd_slot;
        let mut var_prsd_rv: f64 = *var_prsd_rv_slot;
        let mut var_rdraingeo: f64 = *var_rdraingeo_slot;
        let mut var_rdraingeo_dn0: f64 = *var_rdraingeo_dn0_slot;
        let mut var_rdraingeo_dn10: f64 = *var_rdraingeo_dn10_slot;
        let mut var_rdraingeo_dn11: f64 = *var_rdraingeo_dn11_slot;
        let mut var_rdraingeo_dn13: f64 = *var_rdraingeo_dn13_slot;
        let mut var_rdraingeo_dn14: f64 = *var_rdraingeo_dn14_slot;
        let mut var_rdraingeo_dn2: f64 = *var_rdraingeo_dn2_slot;
        let mut var_rdraingeo_dn3: f64 = *var_rdraingeo_dn3_slot;
        let mut var_rdraingeo_dn4: f64 = *var_rdraingeo_dn4_slot;
        let mut var_rdraingeo_dn5: f64 = *var_rdraingeo_dn5_slot;
        let mut var_rdraingeo_dn6: f64 = *var_rdraingeo_dn6_slot;
        let mut var_rdraingeo_dn7: f64 = *var_rdraingeo_dn7_slot;
        let mut var_rdraingeo_dn8: f64 = *var_rdraingeo_dn8_slot;
        let mut var_rdraingeo_dn9: f64 = *var_rdraingeo_dn9_slot;
        let mut var_rdraingeo_rv: f64 = *var_rdraingeo_rv_slot;
        let mut var_rhorsd_v: f64 = *var_rhorsd_v_slot;
        let mut var_rhorsd_v_dn0: f64 = *var_rhorsd_v_dn0_slot;
        let mut var_rhorsd_v_dn10: f64 = *var_rhorsd_v_dn10_slot;
        let mut var_rhorsd_v_dn11: f64 = *var_rhorsd_v_dn11_slot;
        let mut var_rhorsd_v_dn13: f64 = *var_rhorsd_v_dn13_slot;
        let mut var_rhorsd_v_dn14: f64 = *var_rhorsd_v_dn14_slot;
        let mut var_rhorsd_v_dn2: f64 = *var_rhorsd_v_dn2_slot;
        let mut var_rhorsd_v_dn3: f64 = *var_rhorsd_v_dn3_slot;
        let mut var_rhorsd_v_dn4: f64 = *var_rhorsd_v_dn4_slot;
        let mut var_rhorsd_v_dn5: f64 = *var_rhorsd_v_dn5_slot;
        let mut var_rhorsd_v_dn6: f64 = *var_rhorsd_v_dn6_slot;
        let mut var_rhorsd_v_dn7: f64 = *var_rhorsd_v_dn7_slot;
        let mut var_rhorsd_v_dn8: f64 = *var_rhorsd_v_dn8_slot;
        let mut var_rhorsd_v_dn9: f64 = *var_rhorsd_v_dn9_slot;
        let mut var_rhorsd_v_rv: f64 = *var_rhorsd_v_rv_slot;
        let mut var_rsourcegeo: f64 = *var_rsourcegeo_slot;
        let mut var_rsourcegeo_dn0: f64 = *var_rsourcegeo_dn0_slot;
        let mut var_rsourcegeo_dn10: f64 = *var_rsourcegeo_dn10_slot;
        let mut var_rsourcegeo_dn11: f64 = *var_rsourcegeo_dn11_slot;
        let mut var_rsourcegeo_dn13: f64 = *var_rsourcegeo_dn13_slot;
        let mut var_rsourcegeo_dn14: f64 = *var_rsourcegeo_dn14_slot;
        let mut var_rsourcegeo_dn2: f64 = *var_rsourcegeo_dn2_slot;
        let mut var_rsourcegeo_dn3: f64 = *var_rsourcegeo_dn3_slot;
        let mut var_rsourcegeo_dn4: f64 = *var_rsourcegeo_dn4_slot;
        let mut var_rsourcegeo_dn5: f64 = *var_rsourcegeo_dn5_slot;
        let mut var_rsourcegeo_dn6: f64 = *var_rsourcegeo_dn6_slot;
        let mut var_rsourcegeo_dn7: f64 = *var_rsourcegeo_dn7_slot;
        let mut var_rsourcegeo_dn8: f64 = *var_rsourcegeo_dn8_slot;
        let mut var_rsourcegeo_dn9: f64 = *var_rsourcegeo_dn9_slot;
        let mut var_rsourcegeo_rv: f64 = *var_rsourcegeo_rv_slot;
        let mut var_ssp1_i: f64 = *var_ssp1_i_slot;
        let mut var_ssp1_i_rv: f64 = *var_ssp1_i_rv_slot;
        let mut var_ssp2_i: f64 = *var_ssp2_i_slot;
        let mut var_ssp2_i_rv: f64 = *var_ssp2_i_rv_slot;
        let mut var_ssp3_i: f64 = *var_ssp3_i_slot;
        let mut var_ssp3_i_rv: f64 = *var_ssp3_i_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;

        let assign8890_e11544: f64 = if ((var_dimension1_i < 1.0) || (var_dimension1_i > 3.0)) { 1.0 } else { 0.0 };
        var_guard169 = assign8890_e11544;
        var_guard169_rv = 0.0;

        let (assign8900_e11550,) = {
    if ((var_guard168 != 0.0) && (var_guard169 != 0.0)) {
        (2.0,)
    } else {
        (var_dimension1_i,)
    }
};
        var_dimension1_i = assign8900_e11550;
        var_dimension1_i_rv = 0.0;

        let assign8910_e11557: f64 = if ((var_dimension2_i < 1.0) || (var_dimension2_i > 3.0)) { 1.0 } else { 0.0 };
        var_guard170 = assign8910_e11557;
        var_guard170_rv = 0.0;

        let (assign8920_e11563,) = {
    if ((var_guard168 != 0.0) && (var_guard170 != 0.0)) {
        (2.6,)
    } else {
        (var_dimension2_i,)
    }
};
        var_dimension2_i = assign8920_e11563;
        var_dimension2_i_rv = 0.0;

        let assign8930_e11570: f64 = if ((var_dimension3_i < 1.0) || (var_dimension3_i > 3.0)) { 1.0 } else { 0.0 };
        var_guard171 = assign8930_e11570;
        var_guard171_rv = 0.0;

        let (assign8940_e11576,) = {
    if ((var_guard168 != 0.0) && (var_guard171 != 0.0)) {
        (2.6,)
    } else {
        (var_dimension3_i,)
    }
};
        var_dimension3_i = assign8940_e11576;
        var_dimension3_i_rv = 0.0;

        let assign8950_e11579: f64 = if var_ssp1_i < 0.0 { 1.0 } else { 0.0 };
        var_guard172 = assign8950_e11579;
        var_guard172_rv = 0.0;

        let (assign8960_e11585,) = {
    if ((var_guard168 != 0.0) && (var_guard172 != 0.0)) {
        (14.0,)
    } else {
        (var_ssp1_i,)
    }
};
        var_ssp1_i = assign8960_e11585;
        var_ssp1_i_rv = 0.0;

        let assign8970_e11588: f64 = if var_ssp2_i < 0.0 { 1.0 } else { 0.0 };
        var_guard173 = assign8970_e11588;
        var_guard173_rv = 0.0;

        let (assign8980_e11594,) = {
    if ((var_guard168 != 0.0) && (var_guard173 != 0.0)) {
        (24.0,)
    } else {
        (var_ssp2_i,)
    }
};
        var_ssp2_i = assign8980_e11594;
        var_ssp2_i_rv = 0.0;

        let assign8990_e11597: f64 = if var_ssp3_i < 0.0 { 1.0 } else { 0.0 };
        var_guard174 = assign8990_e11597;
        var_guard174_rv = 0.0;

        let (assign9000_e11603,) = {
    if ((var_guard168 != 0.0) && (var_guard174 != 0.0)) {
        (24.0,)
    } else {
        (var_ssp3_i,)
    }
};
        var_ssp3_i = assign9000_e11603;
        var_ssp3_i_rv = 0.0;

        let assign9010_e11606: f64 = if var_e2nom_i < 0.0 { 1.0 } else { 0.0 };
        var_guard175 = assign9010_e11606;
        var_guard175_rv = 0.0;

        let (assign9020_e11612,) = {
    if ((var_guard168 != 0.0) && (var_guard175 != 0.0)) {
        (0.139,)
    } else {
        (var_e2nom_i,)
    }
};
        var_e2nom_i = assign9020_e11612;
        var_e2nom_i_rv = 0.0;

        let assign9030_e11615: f64 = if var_e3nom_i < 0.0 { 1.0 } else { 0.0 };
        var_guard176 = assign9030_e11615;
        var_guard176_rv = 0.0;

        let (assign9040_e11621,) = {
    if ((var_guard168 != 0.0) && (var_guard176 != 0.0)) {
        (2.0,)
    } else {
        (var_e3nom_i,)
    }
};
        var_e3nom_i = assign9040_e11621;
        var_e3nom_i_rv = 0.0;

        let assign9050_e11624: f64 = if var_mfq1nom_i < 0.0 { 1.0 } else { 0.0 };
        var_guard177 = assign9050_e11624;
        var_guard177_rv = 0.0;

        let (assign9060_e11630,) = {
    if ((var_guard168 != 0.0) && (var_guard177 != 0.0)) {
        (11.2,)
    } else {
        (var_mfq1nom_i,)
    }
};
        var_mfq1nom_i = assign9060_e11630;
        var_mfq1nom_i_rv = 0.0;

        let assign9070_e11633: f64 = if var_mfq2nom_i < 0.0 { 1.0 } else { 0.0 };
        var_guard178 = assign9070_e11633;
        var_guard178_rv = 0.0;

        let (assign9080_e11639,) = {
    if ((var_guard168 != 0.0) && (var_guard178 != 0.0)) {
        (8.02,)
    } else {
        (var_mfq2nom_i,)
    }
};
        var_mfq2nom_i = assign9080_e11639;
        var_mfq2nom_i_rv = 0.0;

        let assign9090_e11642: f64 = if var_mfq3nom_i < 0.0 { 1.0 } else { 0.0 };
        var_guard179 = assign9090_e11642;
        var_guard179_rv = 0.0;

        let (assign9100_e11648,) = {
    if ((var_guard168 != 0.0) && (var_guard179 != 0.0)) {
        (6.18,)
    } else {
        (var_mfq3nom_i,)
    }
};
        var_mfq3nom_i = assign9100_e11648;
        var_mfq3nom_i_rv = 0.0;

        let assign9110_e11655: f64 = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };
        var_guard180 = assign9110_e11655;
        var_guard180_rv = 0.0;

        let assign9120_e11658: f64 = if p.p1795 != 0.0 { 1.0 } else { 0.0 };
        var_guard181 = assign9120_e11658;
        var_guard181_rv = 0.0;

        let (assign9130_e11668, assign9130_e11668_d_n0, assign9130_e11668_d_n2, assign9130_e11668_d_n3, assign9130_e11668_d_n4, assign9130_e11668_d_n5, assign9130_e11668_d_n6, assign9130_e11668_d_n7, assign9130_e11668_d_n8, assign9130_e11668_d_n9, assign9130_e11668_d_n10, assign9130_e11668_d_n11, assign9130_e11668_d_n13, assign9130_e11668_d_n14,) = {
    if ((var_guard180 != 0.0) && (var_guard181 != 0.0)) {
        let assign9130_e11665: f64 = (p.p59).powf(p.p1795);
        let assign9130_e11666: f64 = (p.p1793 * assign9130_e11665);
        (assign9130_e11666, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign9130_e11668;
        var_t1_dn0 = assign9130_e11668_d_n0;
        var_t1_dn2 = assign9130_e11668_d_n2;
        var_t1_dn3 = assign9130_e11668_d_n3;
        var_t1_dn4 = assign9130_e11668_d_n4;
        var_t1_dn5 = assign9130_e11668_d_n5;
        var_t1_dn6 = assign9130_e11668_d_n6;
        var_t1_dn7 = assign9130_e11668_d_n7;
        var_t1_dn8 = assign9130_e11668_d_n8;
        var_t1_dn9 = assign9130_e11668_d_n9;
        var_t1_dn10 = assign9130_e11668_d_n10;
        var_t1_dn11 = assign9130_e11668_d_n11;
        var_t1_dn13 = assign9130_e11668_d_n13;
        var_t1_dn14 = assign9130_e11668_d_n14;
        var_t1_rv = 0.0;

        let (assign9140_e11675, assign9140_e11675_d_n0, assign9140_e11675_d_n2, assign9140_e11675_d_n3, assign9140_e11675_d_n4, assign9140_e11675_d_n5, assign9140_e11675_d_n6, assign9140_e11675_d_n7, assign9140_e11675_d_n8, assign9140_e11675_d_n9, assign9140_e11675_d_n10, assign9140_e11675_d_n11, assign9140_e11675_d_n13, assign9140_e11675_d_n14,) = {
    if ((var_guard180 != 0.0) && (var_guard181 == 0.0)) {
        (p.p1793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign9140_e11675;
        var_t1_dn0 = assign9140_e11675_d_n0;
        var_t1_dn2 = assign9140_e11675_d_n2;
        var_t1_dn3 = assign9140_e11675_d_n3;
        var_t1_dn4 = assign9140_e11675_d_n4;
        var_t1_dn5 = assign9140_e11675_d_n5;
        var_t1_dn6 = assign9140_e11675_d_n6;
        var_t1_dn7 = assign9140_e11675_d_n7;
        var_t1_dn8 = assign9140_e11675_d_n8;
        var_t1_dn9 = assign9140_e11675_d_n9;
        var_t1_dn10 = assign9140_e11675_d_n10;
        var_t1_dn11 = assign9140_e11675_d_n11;
        var_t1_dn13 = assign9140_e11675_d_n13;
        var_t1_dn14 = assign9140_e11675_d_n14;
        var_t1_rv = 0.0;

        let assign9150_e11678: f64 = if p.p1794 != 0.0 { 1.0 } else { 0.0 };
        var_guard182 = assign9150_e11678;
        var_guard182_rv = 0.0;

        let (assign9160_e11690, assign9160_e11690_d_n0, assign9160_e11690_d_n2, assign9160_e11690_d_n3, assign9160_e11690_d_n4, assign9160_e11690_d_n5, assign9160_e11690_d_n6, assign9160_e11690_d_n7, assign9160_e11690_d_n8, assign9160_e11690_d_n9, assign9160_e11690_d_n10, assign9160_e11690_d_n11, assign9160_e11690_d_n13, assign9160_e11690_d_n14,) = {
    if ((var_guard180 != 0.0) && (var_guard182 != 0.0)) {
        let assign9160_e11684: f64 = (p.p1797 * p.p4);
        let assign9160_e11687: f64 = (var_nfintotal).powf(p.p1794);
        let assign9160_e11688: f64 = (assign9160_e11684 * assign9160_e11687);
        (assign9160_e11688, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign9160_e11690;
        var_t2_dn0 = assign9160_e11690_d_n0;
        var_t2_dn2 = assign9160_e11690_d_n2;
        var_t2_dn3 = assign9160_e11690_d_n3;
        var_t2_dn4 = assign9160_e11690_d_n4;
        var_t2_dn5 = assign9160_e11690_d_n5;
        var_t2_dn6 = assign9160_e11690_d_n6;
        var_t2_dn7 = assign9160_e11690_d_n7;
        var_t2_dn8 = assign9160_e11690_d_n8;
        var_t2_dn9 = assign9160_e11690_d_n9;
        var_t2_dn10 = assign9160_e11690_d_n10;
        var_t2_dn11 = assign9160_e11690_d_n11;
        var_t2_dn13 = assign9160_e11690_d_n13;
        var_t2_dn14 = assign9160_e11690_d_n14;
        var_t2_rv = 0.0;

        let (assign9170_e11699, assign9170_e11699_d_n0, assign9170_e11699_d_n2, assign9170_e11699_d_n3, assign9170_e11699_d_n4, assign9170_e11699_d_n5, assign9170_e11699_d_n6, assign9170_e11699_d_n7, assign9170_e11699_d_n8, assign9170_e11699_d_n9, assign9170_e11699_d_n10, assign9170_e11699_d_n11, assign9170_e11699_d_n13, assign9170_e11699_d_n14,) = {
    if ((var_guard180 != 0.0) && (var_guard182 == 0.0)) {
        let assign9170_e11697: f64 = (p.p1797 * p.p4);
        (assign9170_e11697, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign9170_e11699;
        var_t2_dn0 = assign9170_e11699_d_n0;
        var_t2_dn2 = assign9170_e11699_d_n2;
        var_t2_dn3 = assign9170_e11699_d_n3;
        var_t2_dn4 = assign9170_e11699_d_n4;
        var_t2_dn5 = assign9170_e11699_d_n5;
        var_t2_dn6 = assign9170_e11699_d_n6;
        var_t2_dn7 = assign9170_e11699_d_n7;
        var_t2_dn8 = assign9170_e11699_d_n8;
        var_t2_dn9 = assign9170_e11699_d_n9;
        var_t2_dn10 = assign9170_e11699_d_n10;
        var_t2_dn11 = assign9170_e11699_d_n11;
        var_t2_dn13 = assign9170_e11699_d_n13;
        var_t2_dn14 = assign9170_e11699_d_n14;
        var_t2_rv = 0.0;

        let assign9180_e11702: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        var_guard183 = assign9180_e11702;
        var_guard183_rv = 0.0;

        let assign9190_e11705: f64 = if p.p1796 != 0.0 { 1.0 } else { 0.0 };
        var_guard184 = assign9190_e11705;
        var_guard184_rv = 0.0;

        let (assign9200_e11721, assign9200_e11721_d_n0, assign9200_e11721_d_n2, assign9200_e11721_d_n3, assign9200_e11721_d_n4, assign9200_e11721_d_n5, assign9200_e11721_d_n6, assign9200_e11721_d_n7, assign9200_e11721_d_n8, assign9200_e11721_d_n9, assign9200_e11721_d_n10, assign9200_e11721_d_n11, assign9200_e11721_d_n13, assign9200_e11721_d_n14,) = {
    if (((var_guard180 != 0.0) && (var_guard183 != 0.0)) && (var_guard184 != 0.0)) {
        let assign9200_e11713: f64 = (p.p1798 * p.p59);
        let assign9200_e11715: f64 = (assign9200_e11713 * p.p43);
        let assign9200_e11718: f64 = (p.p56).powf(p.p1796);
        let assign9200_e11719: f64 = (assign9200_e11715 * assign9200_e11718);
        (assign9200_e11719, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign9200_e11721;
        var_t3_dn0 = assign9200_e11721_d_n0;
        var_t3_dn2 = assign9200_e11721_d_n2;
        var_t3_dn3 = assign9200_e11721_d_n3;
        var_t3_dn4 = assign9200_e11721_d_n4;
        var_t3_dn5 = assign9200_e11721_d_n5;
        var_t3_dn6 = assign9200_e11721_d_n6;
        var_t3_dn7 = assign9200_e11721_d_n7;
        var_t3_dn8 = assign9200_e11721_d_n8;
        var_t3_dn9 = assign9200_e11721_d_n9;
        var_t3_dn10 = assign9200_e11721_d_n10;
        var_t3_dn11 = assign9200_e11721_d_n11;
        var_t3_dn13 = assign9200_e11721_d_n13;
        var_t3_dn14 = assign9200_e11721_d_n14;
        var_t3_rv = 0.0;

        let (assign9210_e11734, assign9210_e11734_d_n0, assign9210_e11734_d_n2, assign9210_e11734_d_n3, assign9210_e11734_d_n4, assign9210_e11734_d_n5, assign9210_e11734_d_n6, assign9210_e11734_d_n7, assign9210_e11734_d_n8, assign9210_e11734_d_n9, assign9210_e11734_d_n10, assign9210_e11734_d_n11, assign9210_e11734_d_n13, assign9210_e11734_d_n14,) = {
    if (((var_guard180 != 0.0) && (var_guard183 != 0.0)) && (var_guard184 == 0.0)) {
        let assign9210_e11730: f64 = (p.p1798 * p.p59);
        let assign9210_e11732: f64 = (assign9210_e11730 * p.p43);
        (assign9210_e11732, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign9210_e11734;
        var_t3_dn0 = assign9210_e11734_d_n0;
        var_t3_dn2 = assign9210_e11734_d_n2;
        var_t3_dn3 = assign9210_e11734_d_n3;
        var_t3_dn4 = assign9210_e11734_d_n4;
        var_t3_dn5 = assign9210_e11734_d_n5;
        var_t3_dn6 = assign9210_e11734_d_n6;
        var_t3_dn7 = assign9210_e11734_d_n7;
        var_t3_dn8 = assign9210_e11734_d_n8;
        var_t3_dn9 = assign9210_e11734_d_n9;
        var_t3_dn10 = assign9210_e11734_d_n10;
        var_t3_dn11 = assign9210_e11734_d_n11;
        var_t3_dn13 = assign9210_e11734_d_n13;
        var_t3_dn14 = assign9210_e11734_d_n14;
        var_t3_rv = 0.0;

        let (assign9220_e11741, assign9220_e11741_d_n0, assign9220_e11741_d_n2, assign9220_e11741_d_n3, assign9220_e11741_d_n4, assign9220_e11741_d_n5, assign9220_e11741_d_n6, assign9220_e11741_d_n7, assign9220_e11741_d_n8, assign9220_e11741_d_n9, assign9220_e11741_d_n10, assign9220_e11741_d_n11, assign9220_e11741_d_n13, assign9220_e11741_d_n14,) = {
    if ((var_guard180 != 0.0) && (var_guard183 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign9220_e11741;
        var_t3_dn0 = assign9220_e11741_d_n0;
        var_t3_dn2 = assign9220_e11741_d_n2;
        var_t3_dn3 = assign9220_e11741_d_n3;
        var_t3_dn4 = assign9220_e11741_d_n4;
        var_t3_dn5 = assign9220_e11741_d_n5;
        var_t3_dn6 = assign9220_e11741_d_n6;
        var_t3_dn7 = assign9220_e11741_d_n7;
        var_t3_dn8 = assign9220_e11741_d_n8;
        var_t3_dn9 = assign9220_e11741_d_n9;
        var_t3_dn10 = assign9220_e11741_d_n10;
        var_t3_dn11 = assign9220_e11741_d_n11;
        var_t3_dn13 = assign9220_e11741_d_n13;
        var_t3_dn14 = assign9220_e11741_d_n14;
        var_t3_rv = 0.0;

        let (assign9240_e11761, assign9240_e11761_d_n0, assign9240_e11761_d_n2, assign9240_e11761_d_n3, assign9240_e11761_d_n4, assign9240_e11761_d_n5, assign9240_e11761_d_n6, assign9240_e11761_d_n7, assign9240_e11761_d_n8, assign9240_e11761_d_n9, assign9240_e11761_d_n10, assign9240_e11761_d_n11, assign9240_e11761_d_n13, assign9240_e11761_d_n14,) = {
    if (var_guard180 != 0.0) {
        let assign9240_e11756: f64 = (var_t1 + var_t2);
        let assign9240_e11758: f64 = (assign9240_e11756 + var_t3);
        let assign9240_e11759: f64 = (p.p1792 * assign9240_e11758);
        (assign9240_e11759, (p.p1792 * ((var_t1_dn0 + var_t2_dn0) + var_t3_dn0)), (p.p1792 * ((var_t1_dn2 + var_t2_dn2) + var_t3_dn2)), (p.p1792 * ((var_t1_dn3 + var_t2_dn3) + var_t3_dn3)), (p.p1792 * ((var_t1_dn4 + var_t2_dn4) + var_t3_dn4)), (p.p1792 * ((var_t1_dn5 + var_t2_dn5) + var_t3_dn5)), (p.p1792 * ((var_t1_dn6 + var_t2_dn6) + var_t3_dn6)), (p.p1792 * ((var_t1_dn7 + var_t2_dn7) + var_t3_dn7)), (p.p1792 * ((var_t1_dn8 + var_t2_dn8) + var_t3_dn8)), (p.p1792 * ((var_t1_dn9 + var_t2_dn9) + var_t3_dn9)), (p.p1792 * ((var_t1_dn10 + var_t2_dn10) + var_t3_dn10)), (p.p1792 * ((var_t1_dn11 + var_t2_dn11) + var_t3_dn11)), (p.p1792 * ((var_t1_dn13 + var_t2_dn13) + var_t3_dn13)), (p.p1792 * ((var_t1_dn14 + var_t2_dn14) + var_t3_dn14)),)
    } else {
        (var_cth, var_cth_dn0, var_cth_dn2, var_cth_dn3, var_cth_dn4, var_cth_dn5, var_cth_dn6, var_cth_dn7, var_cth_dn8, var_cth_dn9, var_cth_dn10, var_cth_dn11, var_cth_dn13, var_cth_dn14,)
    }
};
        var_cth = assign9240_e11761;
        var_cth_dn0 = assign9240_e11761_d_n0;
        var_cth_dn2 = assign9240_e11761_d_n2;
        var_cth_dn3 = assign9240_e11761_d_n3;
        var_cth_dn4 = assign9240_e11761_d_n4;
        var_cth_dn5 = assign9240_e11761_d_n5;
        var_cth_dn6 = assign9240_e11761_d_n6;
        var_cth_dn7 = assign9240_e11761_d_n7;
        var_cth_dn8 = assign9240_e11761_d_n8;
        var_cth_dn9 = assign9240_e11761_d_n9;
        var_cth_dn10 = assign9240_e11761_d_n10;
        var_cth_dn11 = assign9240_e11761_d_n11;
        var_cth_dn13 = assign9240_e11761_d_n13;
        var_cth_dn14 = assign9240_e11761_d_n14;
        var_cth_rv = 0.0;

        let assign9310_e11817: f64 = if p.p77 == 0.0 { 1.0 } else { 0.0 };
        var_guard187 = assign9310_e11817;
        var_guard187_rv = 0.0;

        let (assign9320_e11823, assign9320_e11823_d_n0, assign9320_e11823_d_n2, assign9320_e11823_d_n3, assign9320_e11823_d_n4, assign9320_e11823_d_n5, assign9320_e11823_d_n6, assign9320_e11823_d_n7, assign9320_e11823_d_n8, assign9320_e11823_d_n9, assign9320_e11823_d_n10, assign9320_e11823_d_n11, assign9320_e11823_d_n13, assign9320_e11823_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9320_e11821: f64 = (p.p1078 * p.p18);
        (assign9320_e11821, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsourcegeo, var_rsourcegeo_dn0, var_rsourcegeo_dn2, var_rsourcegeo_dn3, var_rsourcegeo_dn4, var_rsourcegeo_dn5, var_rsourcegeo_dn6, var_rsourcegeo_dn7, var_rsourcegeo_dn8, var_rsourcegeo_dn9, var_rsourcegeo_dn10, var_rsourcegeo_dn11, var_rsourcegeo_dn13, var_rsourcegeo_dn14,)
    }
};
        var_rsourcegeo = assign9320_e11823;
        var_rsourcegeo_dn0 = assign9320_e11823_d_n0;
        var_rsourcegeo_dn2 = assign9320_e11823_d_n2;
        var_rsourcegeo_dn3 = assign9320_e11823_d_n3;
        var_rsourcegeo_dn4 = assign9320_e11823_d_n4;
        var_rsourcegeo_dn5 = assign9320_e11823_d_n5;
        var_rsourcegeo_dn6 = assign9320_e11823_d_n6;
        var_rsourcegeo_dn7 = assign9320_e11823_d_n7;
        var_rsourcegeo_dn8 = assign9320_e11823_d_n8;
        var_rsourcegeo_dn9 = assign9320_e11823_d_n9;
        var_rsourcegeo_dn10 = assign9320_e11823_d_n10;
        var_rsourcegeo_dn11 = assign9320_e11823_d_n11;
        var_rsourcegeo_dn13 = assign9320_e11823_d_n13;
        var_rsourcegeo_dn14 = assign9320_e11823_d_n14;
        var_rsourcegeo_rv = 0.0;

        let (assign9330_e11829, assign9330_e11829_d_n0, assign9330_e11829_d_n2, assign9330_e11829_d_n3, assign9330_e11829_d_n4, assign9330_e11829_d_n5, assign9330_e11829_d_n6, assign9330_e11829_d_n7, assign9330_e11829_d_n8, assign9330_e11829_d_n9, assign9330_e11829_d_n10, assign9330_e11829_d_n11, assign9330_e11829_d_n13, assign9330_e11829_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9330_e11827: f64 = (p.p1079 * p.p19);
        (assign9330_e11827, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdraingeo, var_rdraingeo_dn0, var_rdraingeo_dn2, var_rdraingeo_dn3, var_rdraingeo_dn4, var_rdraingeo_dn5, var_rdraingeo_dn6, var_rdraingeo_dn7, var_rdraingeo_dn8, var_rdraingeo_dn9, var_rdraingeo_dn10, var_rdraingeo_dn11, var_rdraingeo_dn13, var_rdraingeo_dn14,)
    }
};
        var_rdraingeo = assign9330_e11829;
        var_rdraingeo_dn0 = assign9330_e11829_d_n0;
        var_rdraingeo_dn2 = assign9330_e11829_d_n2;
        var_rdraingeo_dn3 = assign9330_e11829_d_n3;
        var_rdraingeo_dn4 = assign9330_e11829_d_n4;
        var_rdraingeo_dn5 = assign9330_e11829_d_n5;
        var_rdraingeo_dn6 = assign9330_e11829_d_n6;
        var_rdraingeo_dn7 = assign9330_e11829_d_n7;
        var_rdraingeo_dn8 = assign9330_e11829_d_n8;
        var_rdraingeo_dn9 = assign9330_e11829_d_n9;
        var_rdraingeo_dn10 = assign9330_e11829_d_n10;
        var_rdraingeo_dn11 = assign9330_e11829_d_n11;
        var_rdraingeo_dn13 = assign9330_e11829_d_n13;
        var_rdraingeo_dn14 = assign9330_e11829_d_n14;
        var_rdraingeo_rv = 0.0;

        let assign9340_e11832: f64 = if p.p1080 > 0.0 { 1.0 } else { 0.0 };
        var_guard188 = assign9340_e11832;
        var_guard188_rv = 0.0;

        let (assign9350_e11851,) = {
    if ((var_guard187 == 0.0) && (var_guard188 != 0.0)) {
        let assign9350_e11839: f64 = (p.p4 * p.p92);
        let assign9350_e11843: f64 = (p.p4 - p.p3);
        let assign9350_e11845: f64 = (assign9350_e11843 * p.p1084);
        let assign9350_e11846: f64 = (p.p3 + assign9350_e11845);
        let assign9350_e11848: f64 = (assign9350_e11846 * p.p1080);
        let assign9350_e11849: f64 = (assign9350_e11839 + assign9350_e11848);
        (assign9350_e11849,)
    } else {
        (var_arsd,)
    }
};
        var_arsd = assign9350_e11851;
        var_arsd_rv = 0.0;

        let (assign9360_e11865,) = {
    if ((var_guard187 == 0.0) && (var_guard188 == 0.0)) {
        let assign9360_e11861: f64 = (p.p92 + p.p1080);
        let assign9360_e11862: f64 = (1e-9_f64).max(assign9360_e11861);
        let assign9360_e11863: f64 = (p.p4 * assign9360_e11862);
        (assign9360_e11863,)
    } else {
        (var_arsd,)
    }
};
        var_arsd = assign9360_e11865;
        var_arsd_rv = 0.0;

        let (assign9370_e11872,) = {
    if (var_guard187 == 0.0) {
        let assign9370_e11870: f64 = (p.p4 + var_deltaprsd_v);
        (assign9370_e11870,)
    } else {
        (var_prsd,)
    }
};
        var_prsd = assign9370_e11872;
        var_prsd_rv = 0.0;

        let assign9380_e11874: f64 = if param_given[1083] { 1.0 } else { 0.0 };
        var_guard189 = assign9380_e11874;
        var_guard189_rv = 0.0;

        let (assign9390_e11881, assign9390_e11881_d_n0, assign9390_e11881_d_n2, assign9390_e11881_d_n3, assign9390_e11881_d_n4, assign9390_e11881_d_n5, assign9390_e11881_d_n6, assign9390_e11881_d_n7, assign9390_e11881_d_n8, assign9390_e11881_d_n9, assign9390_e11881_d_n10, assign9390_e11881_d_n11, assign9390_e11881_d_n13, assign9390_e11881_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard189 != 0.0)) {
        (p.p1083, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rhorsd_v, var_rhorsd_v_dn0, var_rhorsd_v_dn2, var_rhorsd_v_dn3, var_rhorsd_v_dn4, var_rhorsd_v_dn5, var_rhorsd_v_dn6, var_rhorsd_v_dn7, var_rhorsd_v_dn8, var_rhorsd_v_dn9, var_rhorsd_v_dn10, var_rhorsd_v_dn11, var_rhorsd_v_dn13, var_rhorsd_v_dn14,)
    }
};
        var_rhorsd_v = assign9390_e11881;
        var_rhorsd_v_dn0 = assign9390_e11881_d_n0;
        var_rhorsd_v_dn2 = assign9390_e11881_d_n2;
        var_rhorsd_v_dn3 = assign9390_e11881_d_n3;
        var_rhorsd_v_dn4 = assign9390_e11881_d_n4;
        var_rhorsd_v_dn5 = assign9390_e11881_d_n5;
        var_rhorsd_v_dn6 = assign9390_e11881_d_n6;
        var_rhorsd_v_dn7 = assign9390_e11881_d_n7;
        var_rhorsd_v_dn8 = assign9390_e11881_d_n8;
        var_rhorsd_v_dn9 = assign9390_e11881_d_n9;
        var_rhorsd_v_dn10 = assign9390_e11881_d_n10;
        var_rhorsd_v_dn11 = assign9390_e11881_d_n11;
        var_rhorsd_v_dn13 = assign9390_e11881_d_n13;
        var_rhorsd_v_dn14 = assign9390_e11881_d_n14;
        var_rhorsd_v_rv = 0.0;

        *var_arsd_slot = var_arsd;
        *var_arsd_rv_slot = var_arsd_rv;
        *var_cth_slot = var_cth;
        *var_cth_dn0_slot = var_cth_dn0;
        *var_cth_dn10_slot = var_cth_dn10;
        *var_cth_dn11_slot = var_cth_dn11;
        *var_cth_dn13_slot = var_cth_dn13;
        *var_cth_dn14_slot = var_cth_dn14;
        *var_cth_dn2_slot = var_cth_dn2;
        *var_cth_dn3_slot = var_cth_dn3;
        *var_cth_dn4_slot = var_cth_dn4;
        *var_cth_dn5_slot = var_cth_dn5;
        *var_cth_dn6_slot = var_cth_dn6;
        *var_cth_dn7_slot = var_cth_dn7;
        *var_cth_dn8_slot = var_cth_dn8;
        *var_cth_dn9_slot = var_cth_dn9;
        *var_cth_rv_slot = var_cth_rv;
        *var_dimension1_i_slot = var_dimension1_i;
        *var_dimension1_i_rv_slot = var_dimension1_i_rv;
        *var_dimension2_i_slot = var_dimension2_i;
        *var_dimension2_i_rv_slot = var_dimension2_i_rv;
        *var_dimension3_i_slot = var_dimension3_i;
        *var_dimension3_i_rv_slot = var_dimension3_i_rv;
        *var_e2nom_i_slot = var_e2nom_i;
        *var_e2nom_i_rv_slot = var_e2nom_i_rv;
        *var_e3nom_i_slot = var_e3nom_i;
        *var_e3nom_i_rv_slot = var_e3nom_i_rv;
        *var_guard169_slot = var_guard169;
        *var_guard169_rv_slot = var_guard169_rv;
        *var_guard170_slot = var_guard170;
        *var_guard170_rv_slot = var_guard170_rv;
        *var_guard171_slot = var_guard171;
        *var_guard171_rv_slot = var_guard171_rv;
        *var_guard172_slot = var_guard172;
        *var_guard172_rv_slot = var_guard172_rv;
        *var_guard173_slot = var_guard173;
        *var_guard173_rv_slot = var_guard173_rv;
        *var_guard174_slot = var_guard174;
        *var_guard174_rv_slot = var_guard174_rv;
        *var_guard175_slot = var_guard175;
        *var_guard175_rv_slot = var_guard175_rv;
        *var_guard176_slot = var_guard176;
        *var_guard176_rv_slot = var_guard176_rv;
        *var_guard177_slot = var_guard177;
        *var_guard177_rv_slot = var_guard177_rv;
        *var_guard178_slot = var_guard178;
        *var_guard178_rv_slot = var_guard178_rv;
        *var_guard179_slot = var_guard179;
        *var_guard179_rv_slot = var_guard179_rv;
        *var_guard180_slot = var_guard180;
        *var_guard180_rv_slot = var_guard180_rv;
        *var_guard181_slot = var_guard181;
        *var_guard181_rv_slot = var_guard181_rv;
        *var_guard182_slot = var_guard182;
        *var_guard182_rv_slot = var_guard182_rv;
        *var_guard183_slot = var_guard183;
        *var_guard183_rv_slot = var_guard183_rv;
        *var_guard184_slot = var_guard184;
        *var_guard184_rv_slot = var_guard184_rv;
        *var_guard187_slot = var_guard187;
        *var_guard187_rv_slot = var_guard187_rv;
        *var_guard188_slot = var_guard188;
        *var_guard188_rv_slot = var_guard188_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_mfq1nom_i_slot = var_mfq1nom_i;
        *var_mfq1nom_i_rv_slot = var_mfq1nom_i_rv;
        *var_mfq2nom_i_slot = var_mfq2nom_i;
        *var_mfq2nom_i_rv_slot = var_mfq2nom_i_rv;
        *var_mfq3nom_i_slot = var_mfq3nom_i;
        *var_mfq3nom_i_rv_slot = var_mfq3nom_i_rv;
        *var_prsd_slot = var_prsd;
        *var_prsd_rv_slot = var_prsd_rv;
        *var_rdraingeo_slot = var_rdraingeo;
        *var_rdraingeo_dn0_slot = var_rdraingeo_dn0;
        *var_rdraingeo_dn10_slot = var_rdraingeo_dn10;
        *var_rdraingeo_dn11_slot = var_rdraingeo_dn11;
        *var_rdraingeo_dn13_slot = var_rdraingeo_dn13;
        *var_rdraingeo_dn14_slot = var_rdraingeo_dn14;
        *var_rdraingeo_dn2_slot = var_rdraingeo_dn2;
        *var_rdraingeo_dn3_slot = var_rdraingeo_dn3;
        *var_rdraingeo_dn4_slot = var_rdraingeo_dn4;
        *var_rdraingeo_dn5_slot = var_rdraingeo_dn5;
        *var_rdraingeo_dn6_slot = var_rdraingeo_dn6;
        *var_rdraingeo_dn7_slot = var_rdraingeo_dn7;
        *var_rdraingeo_dn8_slot = var_rdraingeo_dn8;
        *var_rdraingeo_dn9_slot = var_rdraingeo_dn9;
        *var_rdraingeo_rv_slot = var_rdraingeo_rv;
        *var_rhorsd_v_slot = var_rhorsd_v;
        *var_rhorsd_v_dn0_slot = var_rhorsd_v_dn0;
        *var_rhorsd_v_dn10_slot = var_rhorsd_v_dn10;
        *var_rhorsd_v_dn11_slot = var_rhorsd_v_dn11;
        *var_rhorsd_v_dn13_slot = var_rhorsd_v_dn13;
        *var_rhorsd_v_dn14_slot = var_rhorsd_v_dn14;
        *var_rhorsd_v_dn2_slot = var_rhorsd_v_dn2;
        *var_rhorsd_v_dn3_slot = var_rhorsd_v_dn3;
        *var_rhorsd_v_dn4_slot = var_rhorsd_v_dn4;
        *var_rhorsd_v_dn5_slot = var_rhorsd_v_dn5;
        *var_rhorsd_v_dn6_slot = var_rhorsd_v_dn6;
        *var_rhorsd_v_dn7_slot = var_rhorsd_v_dn7;
        *var_rhorsd_v_dn8_slot = var_rhorsd_v_dn8;
        *var_rhorsd_v_dn9_slot = var_rhorsd_v_dn9;
        *var_rhorsd_v_rv_slot = var_rhorsd_v_rv;
        *var_rsourcegeo_slot = var_rsourcegeo;
        *var_rsourcegeo_dn0_slot = var_rsourcegeo_dn0;
        *var_rsourcegeo_dn10_slot = var_rsourcegeo_dn10;
        *var_rsourcegeo_dn11_slot = var_rsourcegeo_dn11;
        *var_rsourcegeo_dn13_slot = var_rsourcegeo_dn13;
        *var_rsourcegeo_dn14_slot = var_rsourcegeo_dn14;
        *var_rsourcegeo_dn2_slot = var_rsourcegeo_dn2;
        *var_rsourcegeo_dn3_slot = var_rsourcegeo_dn3;
        *var_rsourcegeo_dn4_slot = var_rsourcegeo_dn4;
        *var_rsourcegeo_dn5_slot = var_rsourcegeo_dn5;
        *var_rsourcegeo_dn6_slot = var_rsourcegeo_dn6;
        *var_rsourcegeo_dn7_slot = var_rsourcegeo_dn7;
        *var_rsourcegeo_dn8_slot = var_rsourcegeo_dn8;
        *var_rsourcegeo_dn9_slot = var_rsourcegeo_dn9;
        *var_rsourcegeo_rv_slot = var_rsourcegeo_rv;
        *var_ssp1_i_slot = var_ssp1_i;
        *var_ssp1_i_rv_slot = var_ssp1_i_rv;
        *var_ssp2_i_slot = var_ssp2_i;
        *var_ssp2_i_rv_slot = var_ssp2_i_rv;
        *var_ssp3_i_slot = var_ssp3_i;
        *var_ssp3_i_rv_slot = var_ssp3_i_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        var_arsd: f64,
        var_guard187: f64,
        var_guard189: f64,
        var_prsd: f64,
        var_afin_slot: &mut f64,
        var_afin_rv_slot: &mut f64,
        var_alpha_slot: &mut f64,
        var_alpha_dn0_slot: &mut f64,
        var_alpha_dn10_slot: &mut f64,
        var_alpha_dn11_slot: &mut f64,
        var_alpha_dn13_slot: &mut f64,
        var_alpha_dn14_slot: &mut f64,
        var_alpha_dn2_slot: &mut f64,
        var_alpha_dn3_slot: &mut f64,
        var_alpha_dn4_slot: &mut f64,
        var_alpha_dn5_slot: &mut f64,
        var_alpha_dn6_slot: &mut f64,
        var_alpha_dn7_slot: &mut f64,
        var_alpha_dn8_slot: &mut f64,
        var_alpha_dn9_slot: &mut f64,
        var_alpha_rv_slot: &mut f64,
        var_arsd_total_slot: &mut f64,
        var_arsd_total_rv_slot: &mut f64,
        var_eta_slot: &mut f64,
        var_eta_dn0_slot: &mut f64,
        var_eta_dn10_slot: &mut f64,
        var_eta_dn11_slot: &mut f64,
        var_eta_dn13_slot: &mut f64,
        var_eta_dn14_slot: &mut f64,
        var_eta_dn2_slot: &mut f64,
        var_eta_dn3_slot: &mut f64,
        var_eta_dn4_slot: &mut f64,
        var_eta_dn5_slot: &mut f64,
        var_eta_dn6_slot: &mut f64,
        var_eta_dn7_slot: &mut f64,
        var_eta_dn8_slot: &mut f64,
        var_eta_dn9_slot: &mut f64,
        var_eta_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_lt_slot: &mut f64,
        var_lt_dn0_slot: &mut f64,
        var_lt_dn10_slot: &mut f64,
        var_lt_dn11_slot: &mut f64,
        var_lt_dn13_slot: &mut f64,
        var_lt_dn14_slot: &mut f64,
        var_lt_dn2_slot: &mut f64,
        var_lt_dn3_slot: &mut f64,
        var_lt_dn4_slot: &mut f64,
        var_lt_dn5_slot: &mut f64,
        var_lt_dn6_slot: &mut f64,
        var_lt_dn7_slot: &mut f64,
        var_lt_dn8_slot: &mut f64,
        var_lt_dn9_slot: &mut f64,
        var_lt_rv_slot: &mut f64,
        var_mu_max_slot: &mut f64,
        var_mu_max_rv_slot: &mut f64,
        var_mu_rsd_slot: &mut f64,
        var_mu_rsd_dn0_slot: &mut f64,
        var_mu_rsd_dn10_slot: &mut f64,
        var_mu_rsd_dn11_slot: &mut f64,
        var_mu_rsd_dn13_slot: &mut f64,
        var_mu_rsd_dn14_slot: &mut f64,
        var_mu_rsd_dn2_slot: &mut f64,
        var_mu_rsd_dn3_slot: &mut f64,
        var_mu_rsd_dn4_slot: &mut f64,
        var_mu_rsd_dn5_slot: &mut f64,
        var_mu_rsd_dn6_slot: &mut f64,
        var_mu_rsd_dn7_slot: &mut f64,
        var_mu_rsd_dn8_slot: &mut f64,
        var_mu_rsd_dn9_slot: &mut f64,
        var_mu_rsd_rv_slot: &mut f64,
        var_prsd_total_slot: &mut f64,
        var_prsd_total_rv_slot: &mut f64,
        var_rhorsd_v_slot: &mut f64,
        var_rhorsd_v_dn0_slot: &mut f64,
        var_rhorsd_v_dn10_slot: &mut f64,
        var_rhorsd_v_dn11_slot: &mut f64,
        var_rhorsd_v_dn13_slot: &mut f64,
        var_rhorsd_v_dn14_slot: &mut f64,
        var_rhorsd_v_dn2_slot: &mut f64,
        var_rhorsd_v_dn3_slot: &mut f64,
        var_rhorsd_v_dn4_slot: &mut f64,
        var_rhorsd_v_dn5_slot: &mut f64,
        var_rhorsd_v_dn6_slot: &mut f64,
        var_rhorsd_v_dn7_slot: &mut f64,
        var_rhorsd_v_dn8_slot: &mut f64,
        var_rhorsd_v_dn9_slot: &mut f64,
        var_rhorsd_v_rv_slot: &mut f64,
        var_rsp_slot: &mut f64,
        var_rsp_dn0_slot: &mut f64,
        var_rsp_dn10_slot: &mut f64,
        var_rsp_dn11_slot: &mut f64,
        var_rsp_dn13_slot: &mut f64,
        var_rsp_dn14_slot: &mut f64,
        var_rsp_dn2_slot: &mut f64,
        var_rsp_dn3_slot: &mut f64,
        var_rsp_dn4_slot: &mut f64,
        var_rsp_dn5_slot: &mut f64,
        var_rsp_dn6_slot: &mut f64,
        var_rsp_dn7_slot: &mut f64,
        var_rsp_dn8_slot: &mut f64,
        var_rsp_dn9_slot: &mut f64,
        var_rsp_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_thetarsp_slot: &mut f64,
        var_thetarsp_rv_slot: &mut f64,
    ) {
        let mut var_afin: f64 = *var_afin_slot;
        let mut var_afin_rv: f64 = *var_afin_rv_slot;
        let mut var_alpha: f64 = *var_alpha_slot;
        let mut var_alpha_dn0: f64 = *var_alpha_dn0_slot;
        let mut var_alpha_dn10: f64 = *var_alpha_dn10_slot;
        let mut var_alpha_dn11: f64 = *var_alpha_dn11_slot;
        let mut var_alpha_dn13: f64 = *var_alpha_dn13_slot;
        let mut var_alpha_dn14: f64 = *var_alpha_dn14_slot;
        let mut var_alpha_dn2: f64 = *var_alpha_dn2_slot;
        let mut var_alpha_dn3: f64 = *var_alpha_dn3_slot;
        let mut var_alpha_dn4: f64 = *var_alpha_dn4_slot;
        let mut var_alpha_dn5: f64 = *var_alpha_dn5_slot;
        let mut var_alpha_dn6: f64 = *var_alpha_dn6_slot;
        let mut var_alpha_dn7: f64 = *var_alpha_dn7_slot;
        let mut var_alpha_dn8: f64 = *var_alpha_dn8_slot;
        let mut var_alpha_dn9: f64 = *var_alpha_dn9_slot;
        let mut var_alpha_rv: f64 = *var_alpha_rv_slot;
        let mut var_arsd_total: f64 = *var_arsd_total_slot;
        let mut var_arsd_total_rv: f64 = *var_arsd_total_rv_slot;
        let mut var_eta: f64 = *var_eta_slot;
        let mut var_eta_dn0: f64 = *var_eta_dn0_slot;
        let mut var_eta_dn10: f64 = *var_eta_dn10_slot;
        let mut var_eta_dn11: f64 = *var_eta_dn11_slot;
        let mut var_eta_dn13: f64 = *var_eta_dn13_slot;
        let mut var_eta_dn14: f64 = *var_eta_dn14_slot;
        let mut var_eta_dn2: f64 = *var_eta_dn2_slot;
        let mut var_eta_dn3: f64 = *var_eta_dn3_slot;
        let mut var_eta_dn4: f64 = *var_eta_dn4_slot;
        let mut var_eta_dn5: f64 = *var_eta_dn5_slot;
        let mut var_eta_dn6: f64 = *var_eta_dn6_slot;
        let mut var_eta_dn7: f64 = *var_eta_dn7_slot;
        let mut var_eta_dn8: f64 = *var_eta_dn8_slot;
        let mut var_eta_dn9: f64 = *var_eta_dn9_slot;
        let mut var_eta_rv: f64 = *var_eta_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_lt: f64 = *var_lt_slot;
        let mut var_lt_dn0: f64 = *var_lt_dn0_slot;
        let mut var_lt_dn10: f64 = *var_lt_dn10_slot;
        let mut var_lt_dn11: f64 = *var_lt_dn11_slot;
        let mut var_lt_dn13: f64 = *var_lt_dn13_slot;
        let mut var_lt_dn14: f64 = *var_lt_dn14_slot;
        let mut var_lt_dn2: f64 = *var_lt_dn2_slot;
        let mut var_lt_dn3: f64 = *var_lt_dn3_slot;
        let mut var_lt_dn4: f64 = *var_lt_dn4_slot;
        let mut var_lt_dn5: f64 = *var_lt_dn5_slot;
        let mut var_lt_dn6: f64 = *var_lt_dn6_slot;
        let mut var_lt_dn7: f64 = *var_lt_dn7_slot;
        let mut var_lt_dn8: f64 = *var_lt_dn8_slot;
        let mut var_lt_dn9: f64 = *var_lt_dn9_slot;
        let mut var_lt_rv: f64 = *var_lt_rv_slot;
        let mut var_mu_max: f64 = *var_mu_max_slot;
        let mut var_mu_max_rv: f64 = *var_mu_max_rv_slot;
        let mut var_mu_rsd: f64 = *var_mu_rsd_slot;
        let mut var_mu_rsd_dn0: f64 = *var_mu_rsd_dn0_slot;
        let mut var_mu_rsd_dn10: f64 = *var_mu_rsd_dn10_slot;
        let mut var_mu_rsd_dn11: f64 = *var_mu_rsd_dn11_slot;
        let mut var_mu_rsd_dn13: f64 = *var_mu_rsd_dn13_slot;
        let mut var_mu_rsd_dn14: f64 = *var_mu_rsd_dn14_slot;
        let mut var_mu_rsd_dn2: f64 = *var_mu_rsd_dn2_slot;
        let mut var_mu_rsd_dn3: f64 = *var_mu_rsd_dn3_slot;
        let mut var_mu_rsd_dn4: f64 = *var_mu_rsd_dn4_slot;
        let mut var_mu_rsd_dn5: f64 = *var_mu_rsd_dn5_slot;
        let mut var_mu_rsd_dn6: f64 = *var_mu_rsd_dn6_slot;
        let mut var_mu_rsd_dn7: f64 = *var_mu_rsd_dn7_slot;
        let mut var_mu_rsd_dn8: f64 = *var_mu_rsd_dn8_slot;
        let mut var_mu_rsd_dn9: f64 = *var_mu_rsd_dn9_slot;
        let mut var_mu_rsd_rv: f64 = *var_mu_rsd_rv_slot;
        let mut var_prsd_total: f64 = *var_prsd_total_slot;
        let mut var_prsd_total_rv: f64 = *var_prsd_total_rv_slot;
        let mut var_rhorsd_v: f64 = *var_rhorsd_v_slot;
        let mut var_rhorsd_v_dn0: f64 = *var_rhorsd_v_dn0_slot;
        let mut var_rhorsd_v_dn10: f64 = *var_rhorsd_v_dn10_slot;
        let mut var_rhorsd_v_dn11: f64 = *var_rhorsd_v_dn11_slot;
        let mut var_rhorsd_v_dn13: f64 = *var_rhorsd_v_dn13_slot;
        let mut var_rhorsd_v_dn14: f64 = *var_rhorsd_v_dn14_slot;
        let mut var_rhorsd_v_dn2: f64 = *var_rhorsd_v_dn2_slot;
        let mut var_rhorsd_v_dn3: f64 = *var_rhorsd_v_dn3_slot;
        let mut var_rhorsd_v_dn4: f64 = *var_rhorsd_v_dn4_slot;
        let mut var_rhorsd_v_dn5: f64 = *var_rhorsd_v_dn5_slot;
        let mut var_rhorsd_v_dn6: f64 = *var_rhorsd_v_dn6_slot;
        let mut var_rhorsd_v_dn7: f64 = *var_rhorsd_v_dn7_slot;
        let mut var_rhorsd_v_dn8: f64 = *var_rhorsd_v_dn8_slot;
        let mut var_rhorsd_v_dn9: f64 = *var_rhorsd_v_dn9_slot;
        let mut var_rhorsd_v_rv: f64 = *var_rhorsd_v_rv_slot;
        let mut var_rsp: f64 = *var_rsp_slot;
        let mut var_rsp_dn0: f64 = *var_rsp_dn0_slot;
        let mut var_rsp_dn10: f64 = *var_rsp_dn10_slot;
        let mut var_rsp_dn11: f64 = *var_rsp_dn11_slot;
        let mut var_rsp_dn13: f64 = *var_rsp_dn13_slot;
        let mut var_rsp_dn14: f64 = *var_rsp_dn14_slot;
        let mut var_rsp_dn2: f64 = *var_rsp_dn2_slot;
        let mut var_rsp_dn3: f64 = *var_rsp_dn3_slot;
        let mut var_rsp_dn4: f64 = *var_rsp_dn4_slot;
        let mut var_rsp_dn5: f64 = *var_rsp_dn5_slot;
        let mut var_rsp_dn6: f64 = *var_rsp_dn6_slot;
        let mut var_rsp_dn7: f64 = *var_rsp_dn7_slot;
        let mut var_rsp_dn8: f64 = *var_rsp_dn8_slot;
        let mut var_rsp_dn9: f64 = *var_rsp_dn9_slot;
        let mut var_rsp_rv: f64 = *var_rsp_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_thetarsp: f64 = *var_thetarsp_slot;
        let mut var_thetarsp_rv: f64 = *var_thetarsp_rv_slot;

        let (assign9400_e11894,) = {
    if ((var_guard187 == 0.0) && (var_guard189 == 0.0)) {
        let (assign9400_e11892,) = {
            if (p.p60 == 1.0) {
                (1417.0,)
            } else {
                (470.5,)
            }
        };
        (assign9400_e11892,)
    } else {
        (var_mu_max,)
    }
};
        var_mu_max = assign9400_e11894;
        var_mu_max_rv = 0.0;

        let assign9410_e11897: f64 = if p.p60 == 1.0 { 1.0 } else { 0.0 };
        var_guard190 = assign9410_e11897;
        var_guard190_rv = 0.0;

        let (assign9420_e11911, assign9420_e11911_d_n0, assign9420_e11911_d_n2, assign9420_e11911_d_n3, assign9420_e11911_d_n4, assign9420_e11911_d_n5, assign9420_e11911_d_n6, assign9420_e11911_d_n7, assign9420_e11911_d_n8, assign9420_e11911_d_n9, assign9420_e11911_d_n10, assign9420_e11911_d_n11, assign9420_e11911_d_n13, assign9420_e11911_d_n14,) = {
    if (((var_guard187 == 0.0) && (var_guard189 == 0.0)) && (var_guard190 != 0.0)) {
        let assign9420_e11907: f64 = (p.p97 / 9.68e22);
        let assign9420_e11909: f64 = (assign9420_e11907).powf(0.68);
        (assign9420_e11909, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign9420_e11911;
        var_t0_dn0 = assign9420_e11911_d_n0;
        var_t0_dn2 = assign9420_e11911_d_n2;
        var_t0_dn3 = assign9420_e11911_d_n3;
        var_t0_dn4 = assign9420_e11911_d_n4;
        var_t0_dn5 = assign9420_e11911_d_n5;
        var_t0_dn6 = assign9420_e11911_d_n6;
        var_t0_dn7 = assign9420_e11911_d_n7;
        var_t0_dn8 = assign9420_e11911_d_n8;
        var_t0_dn9 = assign9420_e11911_d_n9;
        var_t0_dn10 = assign9420_e11911_d_n10;
        var_t0_dn11 = assign9420_e11911_d_n11;
        var_t0_dn13 = assign9420_e11911_d_n13;
        var_t0_dn14 = assign9420_e11911_d_n14;
        var_t0_rv = 0.0;

        let (assign9430_e11923, assign9430_e11923_d_n0, assign9430_e11923_d_n2, assign9430_e11923_d_n3, assign9430_e11923_d_n4, assign9430_e11923_d_n5, assign9430_e11923_d_n6, assign9430_e11923_d_n7, assign9430_e11923_d_n8, assign9430_e11923_d_n9, assign9430_e11923_d_n10, assign9430_e11923_d_n11, assign9430_e11923_d_n13, assign9430_e11923_d_n14,) = {
    if (((var_guard187 == 0.0) && (var_guard189 == 0.0)) && (var_guard190 != 0.0)) {
        let assign9430_e11921: f64 = (3.43e26 / p.p97);
        (assign9430_e11921, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign9430_e11923;
        var_t1_dn0 = assign9430_e11923_d_n0;
        var_t1_dn2 = assign9430_e11923_d_n2;
        var_t1_dn3 = assign9430_e11923_d_n3;
        var_t1_dn4 = assign9430_e11923_d_n4;
        var_t1_dn5 = assign9430_e11923_d_n5;
        var_t1_dn6 = assign9430_e11923_d_n6;
        var_t1_dn7 = assign9430_e11923_d_n7;
        var_t1_dn8 = assign9430_e11923_d_n8;
        var_t1_dn9 = assign9430_e11923_d_n9;
        var_t1_dn10 = assign9430_e11923_d_n10;
        var_t1_dn11 = assign9430_e11923_d_n11;
        var_t1_dn13 = assign9430_e11923_d_n13;
        var_t1_dn14 = assign9430_e11923_d_n14;
        var_t1_rv = 0.0;

        let (assign9440_e11951, assign9440_e11951_d_n0, assign9440_e11951_d_n2, assign9440_e11951_d_n3, assign9440_e11951_d_n4, assign9440_e11951_d_n5, assign9440_e11951_d_n6, assign9440_e11951_d_n7, assign9440_e11951_d_n8, assign9440_e11951_d_n9, assign9440_e11951_d_n10, assign9440_e11951_d_n11, assign9440_e11951_d_n13, assign9440_e11951_d_n14,) = {
    if (((var_guard187 == 0.0) && (var_guard189 == 0.0)) && (var_guard190 != 0.0)) {
        let assign9440_e11934: f64 = (var_mu_max - 52.2);
        let assign9440_e11937: f64 = (1.0 + var_t0);
        let assign9440_e11938: f64 = (assign9440_e11934 / assign9440_e11937);
        let assign9440_e11939: f64 = (52.2 + assign9440_e11938);
        let assign9440_e11944: f64 = (var_t1 * var_t1);
        let assign9440_e11945: f64 = (1.0 + assign9440_e11944);
        let assign9440_e11946: f64 = (43.4 / assign9440_e11945);
        let assign9440_e11947: f64 = (assign9440_e11939 - assign9440_e11946);
        let assign9440_e11949: f64 = (assign9440_e11947 * 0.0001);
        (assign9440_e11949, (((-((assign9440_e11934 * var_t0_dn0) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn2) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn3) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn4) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn5) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn6) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn7) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn8) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn9) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn10) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn11) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn13) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * var_t0_dn14) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((var_t1_dn14 * var_t1) + (var_t1 * var_t1_dn14))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001),)
    } else {
        (var_mu_rsd, var_mu_rsd_dn0, var_mu_rsd_dn2, var_mu_rsd_dn3, var_mu_rsd_dn4, var_mu_rsd_dn5, var_mu_rsd_dn6, var_mu_rsd_dn7, var_mu_rsd_dn8, var_mu_rsd_dn9, var_mu_rsd_dn10, var_mu_rsd_dn11, var_mu_rsd_dn13, var_mu_rsd_dn14,)
    }
};
        var_mu_rsd = assign9440_e11951;
        var_mu_rsd_dn0 = assign9440_e11951_d_n0;
        var_mu_rsd_dn2 = assign9440_e11951_d_n2;
        var_mu_rsd_dn3 = assign9440_e11951_d_n3;
        var_mu_rsd_dn4 = assign9440_e11951_d_n4;
        var_mu_rsd_dn5 = assign9440_e11951_d_n5;
        var_mu_rsd_dn6 = assign9440_e11951_d_n6;
        var_mu_rsd_dn7 = assign9440_e11951_d_n7;
        var_mu_rsd_dn8 = assign9440_e11951_d_n8;
        var_mu_rsd_dn9 = assign9440_e11951_d_n9;
        var_mu_rsd_dn10 = assign9440_e11951_d_n10;
        var_mu_rsd_dn11 = assign9440_e11951_d_n11;
        var_mu_rsd_dn13 = assign9440_e11951_d_n13;
        var_mu_rsd_dn14 = assign9440_e11951_d_n14;
        var_mu_rsd_rv = 0.0;

        let (assign9450_e11966, assign9450_e11966_d_n0, assign9450_e11966_d_n2, assign9450_e11966_d_n3, assign9450_e11966_d_n4, assign9450_e11966_d_n5, assign9450_e11966_d_n6, assign9450_e11966_d_n7, assign9450_e11966_d_n8, assign9450_e11966_d_n9, assign9450_e11966_d_n10, assign9450_e11966_d_n11, assign9450_e11966_d_n13, assign9450_e11966_d_n14,) = {
    if (((var_guard187 == 0.0) && (var_guard189 == 0.0)) && (var_guard190 == 0.0)) {
        let assign9450_e11962: f64 = (p.p97 / 2.23e22);
        let assign9450_e11964: f64 = (assign9450_e11962).powf(0.719);
        (assign9450_e11964, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign9450_e11966;
        var_t0_dn0 = assign9450_e11966_d_n0;
        var_t0_dn2 = assign9450_e11966_d_n2;
        var_t0_dn3 = assign9450_e11966_d_n3;
        var_t0_dn4 = assign9450_e11966_d_n4;
        var_t0_dn5 = assign9450_e11966_d_n5;
        var_t0_dn6 = assign9450_e11966_d_n6;
        var_t0_dn7 = assign9450_e11966_d_n7;
        var_t0_dn8 = assign9450_e11966_d_n8;
        var_t0_dn9 = assign9450_e11966_d_n9;
        var_t0_dn10 = assign9450_e11966_d_n10;
        var_t0_dn11 = assign9450_e11966_d_n11;
        var_t0_dn13 = assign9450_e11966_d_n13;
        var_t0_dn14 = assign9450_e11966_d_n14;
        var_t0_rv = 0.0;

        let (assign9460_e11979, assign9460_e11979_d_n0, assign9460_e11979_d_n2, assign9460_e11979_d_n3, assign9460_e11979_d_n4, assign9460_e11979_d_n5, assign9460_e11979_d_n6, assign9460_e11979_d_n7, assign9460_e11979_d_n8, assign9460_e11979_d_n9, assign9460_e11979_d_n10, assign9460_e11979_d_n11, assign9460_e11979_d_n13, assign9460_e11979_d_n14,) = {
    if (((var_guard187 == 0.0) && (var_guard189 == 0.0)) && (var_guard190 == 0.0)) {
        let assign9460_e11977: f64 = (6.1e26 / p.p97);
        (assign9460_e11977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign9460_e11979;
        var_t1_dn0 = assign9460_e11979_d_n0;
        var_t1_dn2 = assign9460_e11979_d_n2;
        var_t1_dn3 = assign9460_e11979_d_n3;
        var_t1_dn4 = assign9460_e11979_d_n4;
        var_t1_dn5 = assign9460_e11979_d_n5;
        var_t1_dn6 = assign9460_e11979_d_n6;
        var_t1_dn7 = assign9460_e11979_d_n7;
        var_t1_dn8 = assign9460_e11979_d_n8;
        var_t1_dn9 = assign9460_e11979_d_n9;
        var_t1_dn10 = assign9460_e11979_d_n10;
        var_t1_dn11 = assign9460_e11979_d_n11;
        var_t1_dn13 = assign9460_e11979_d_n13;
        var_t1_dn14 = assign9460_e11979_d_n14;
        var_t1_rv = 0.0;

        let (assign9470_e12008, assign9470_e12008_d_n0, assign9470_e12008_d_n2, assign9470_e12008_d_n3, assign9470_e12008_d_n4, assign9470_e12008_d_n5, assign9470_e12008_d_n6, assign9470_e12008_d_n7, assign9470_e12008_d_n8, assign9470_e12008_d_n9, assign9470_e12008_d_n10, assign9470_e12008_d_n11, assign9470_e12008_d_n13, assign9470_e12008_d_n14,) = {
    if (((var_guard187 == 0.0) && (var_guard189 == 0.0)) && (var_guard190 == 0.0)) {
        let assign9470_e11991: f64 = (var_mu_max - 44.9);
        let assign9470_e11994: f64 = (1.0 + var_t0);
        let assign9470_e11995: f64 = (assign9470_e11991 / assign9470_e11994);
        let assign9470_e11996: f64 = (44.9 + assign9470_e11995);
        let assign9470_e12001: f64 = (var_t1 * var_t1);
        let assign9470_e12002: f64 = (1.0 + assign9470_e12001);
        let assign9470_e12003: f64 = (29.0 / assign9470_e12002);
        let assign9470_e12004: f64 = (assign9470_e11996 - assign9470_e12003);
        let assign9470_e12006: f64 = (assign9470_e12004 * 0.0001);
        (assign9470_e12006, (((-((assign9470_e11991 * var_t0_dn0) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn2) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn3) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn4) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn5) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn6) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn7) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn8) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn9) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn10) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn11) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn13) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * var_t0_dn14) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((var_t1_dn14 * var_t1) + (var_t1 * var_t1_dn14))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001),)
    } else {
        (var_mu_rsd, var_mu_rsd_dn0, var_mu_rsd_dn2, var_mu_rsd_dn3, var_mu_rsd_dn4, var_mu_rsd_dn5, var_mu_rsd_dn6, var_mu_rsd_dn7, var_mu_rsd_dn8, var_mu_rsd_dn9, var_mu_rsd_dn10, var_mu_rsd_dn11, var_mu_rsd_dn13, var_mu_rsd_dn14,)
    }
};
        var_mu_rsd = assign9470_e12008;
        var_mu_rsd_dn0 = assign9470_e12008_d_n0;
        var_mu_rsd_dn2 = assign9470_e12008_d_n2;
        var_mu_rsd_dn3 = assign9470_e12008_d_n3;
        var_mu_rsd_dn4 = assign9470_e12008_d_n4;
        var_mu_rsd_dn5 = assign9470_e12008_d_n5;
        var_mu_rsd_dn6 = assign9470_e12008_d_n6;
        var_mu_rsd_dn7 = assign9470_e12008_d_n7;
        var_mu_rsd_dn8 = assign9470_e12008_d_n8;
        var_mu_rsd_dn9 = assign9470_e12008_d_n9;
        var_mu_rsd_dn10 = assign9470_e12008_d_n10;
        var_mu_rsd_dn11 = assign9470_e12008_d_n11;
        var_mu_rsd_dn13 = assign9470_e12008_d_n13;
        var_mu_rsd_dn14 = assign9470_e12008_d_n14;
        var_mu_rsd_rv = 0.0;

        let (assign9480_e12022, assign9480_e12022_d_n0, assign9480_e12022_d_n2, assign9480_e12022_d_n3, assign9480_e12022_d_n4, assign9480_e12022_d_n5, assign9480_e12022_d_n6, assign9480_e12022_d_n7, assign9480_e12022_d_n8, assign9480_e12022_d_n9, assign9480_e12022_d_n10, assign9480_e12022_d_n11, assign9480_e12022_d_n13, assign9480_e12022_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard189 == 0.0)) {
        let assign9480_e12017: f64 = (1.60219e-19 * p.p97);
        let assign9480_e12019: f64 = (assign9480_e12017 * var_mu_rsd);
        let assign9480_e12020: f64 = (1.0 / assign9480_e12019);
        (assign9480_e12020, (-((assign9480_e12017 * var_mu_rsd_dn0) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn2) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn3) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn4) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn5) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn6) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn7) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn8) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn9) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn10) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn11) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn13) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * var_mu_rsd_dn14) / (assign9480_e12019 * assign9480_e12019))),)
    } else {
        (var_rhorsd_v, var_rhorsd_v_dn0, var_rhorsd_v_dn2, var_rhorsd_v_dn3, var_rhorsd_v_dn4, var_rhorsd_v_dn5, var_rhorsd_v_dn6, var_rhorsd_v_dn7, var_rhorsd_v_dn8, var_rhorsd_v_dn9, var_rhorsd_v_dn10, var_rhorsd_v_dn11, var_rhorsd_v_dn13, var_rhorsd_v_dn14,)
    }
};
        var_rhorsd_v = assign9480_e12022;
        var_rhorsd_v_dn0 = assign9480_e12022_d_n0;
        var_rhorsd_v_dn2 = assign9480_e12022_d_n2;
        var_rhorsd_v_dn3 = assign9480_e12022_d_n3;
        var_rhorsd_v_dn4 = assign9480_e12022_d_n4;
        var_rhorsd_v_dn5 = assign9480_e12022_d_n5;
        var_rhorsd_v_dn6 = assign9480_e12022_d_n6;
        var_rhorsd_v_dn7 = assign9480_e12022_d_n7;
        var_rhorsd_v_dn8 = assign9480_e12022_d_n8;
        var_rhorsd_v_dn9 = assign9480_e12022_d_n9;
        var_rhorsd_v_dn10 = assign9480_e12022_d_n10;
        var_rhorsd_v_dn11 = assign9480_e12022_d_n11;
        var_rhorsd_v_dn13 = assign9480_e12022_d_n13;
        var_rhorsd_v_dn14 = assign9480_e12022_d_n14;
        var_rhorsd_v_rv = 0.0;

        let (assign9490_e12031,) = {
    if (var_guard187 == 0.0) {
        let assign9490_e12027: f64 = (55.0 * 3.141592653589793);
        let assign9490_e12029: f64 = (assign9490_e12027 / 180.0);
        (assign9490_e12029,)
    } else {
        (var_thetarsp,)
    }
};
        var_thetarsp = assign9490_e12031;
        var_thetarsp_rv = 0.0;

        let (assign9500_e12046,) = {
    if (var_guard187 == 0.0) {
        let assign9500_e12040: f64 = (0.0_f64).min(p.p1080);
        let assign9500_e12041: f64 = (p.p92 + assign9500_e12040);
        let assign9500_e12042: f64 = (p.p3 * assign9500_e12041);
        let assign9500_e12043: f64 = (1e-18_f64).max(assign9500_e12042);
        let assign9500_e12044: f64 = (var_arsd).min(assign9500_e12043);
        (assign9500_e12044,)
    } else {
        (var_afin,)
    }
};
        var_afin = assign9500_e12046;
        var_afin_rv = 0.0;

        let (assign9510_e12076, assign9510_e12076_d_n0, assign9510_e12076_d_n2, assign9510_e12076_d_n3, assign9510_e12076_d_n4, assign9510_e12076_d_n5, assign9510_e12076_d_n6, assign9510_e12076_d_n7, assign9510_e12076_d_n8, assign9510_e12076_d_n9, assign9510_e12076_d_n10, assign9510_e12076_d_n11, assign9510_e12076_d_n13, assign9510_e12076_d_n14,) = {
    if (var_guard187 == 0.0) {
        let assign9510_e12051: f64 = (var_thetarsp).tan();
        let assign9510_e12052: f64 = (var_rhorsd_v / assign9510_e12051);
        let assign9510_e12054: f64 = (3.141592653589793_f64).sqrt();
        let assign9510_e12056: f64 = (assign9510_e12054 * p.p5);
        let assign9510_e12057: f64 = (assign9510_e12052 / assign9510_e12056);
        let assign9510_e12060: f64 = (var_afin).sqrt();
        let assign9510_e12061: f64 = (1.0 / assign9510_e12060);
        let assign9510_e12064: f64 = (var_arsd).sqrt();
        let assign9510_e12065: f64 = (2.0 / assign9510_e12064);
        let assign9510_e12066: f64 = (assign9510_e12061 - assign9510_e12065);
        let assign9510_e12070: f64 = (var_arsd * var_arsd);
        let assign9510_e12071: f64 = (var_afin / assign9510_e12070);
        let assign9510_e12072: f64 = (assign9510_e12071).sqrt();
        let assign9510_e12073: f64 = (assign9510_e12066 + assign9510_e12072);
        let assign9510_e12074: f64 = (assign9510_e12057 * assign9510_e12073);
        (assign9510_e12074, (((var_rhorsd_v_dn0 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn2 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn3 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn4 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn5 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn6 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn7 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn8 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn9 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn10 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn11 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn13 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((var_rhorsd_v_dn14 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073),)
    } else {
        (var_rsp, var_rsp_dn0, var_rsp_dn2, var_rsp_dn3, var_rsp_dn4, var_rsp_dn5, var_rsp_dn6, var_rsp_dn7, var_rsp_dn8, var_rsp_dn9, var_rsp_dn10, var_rsp_dn11, var_rsp_dn13, var_rsp_dn14,)
    }
};
        var_rsp = assign9510_e12076;
        var_rsp_dn0 = assign9510_e12076_d_n0;
        var_rsp_dn2 = assign9510_e12076_d_n2;
        var_rsp_dn3 = assign9510_e12076_d_n3;
        var_rsp_dn4 = assign9510_e12076_d_n4;
        var_rsp_dn5 = assign9510_e12076_d_n5;
        var_rsp_dn6 = assign9510_e12076_d_n6;
        var_rsp_dn7 = assign9510_e12076_d_n7;
        var_rsp_dn8 = assign9510_e12076_d_n8;
        var_rsp_dn9 = assign9510_e12076_d_n9;
        var_rsp_dn10 = assign9510_e12076_d_n10;
        var_rsp_dn11 = assign9510_e12076_d_n11;
        var_rsp_dn13 = assign9510_e12076_d_n13;
        var_rsp_dn14 = assign9510_e12076_d_n14;
        var_rsp_rv = 0.0;

        let (assign9520_e12085,) = {
    if (var_guard187 == 0.0) {
        let assign9520_e12081: f64 = (var_arsd * p.p5);
        let assign9520_e12083: f64 = (assign9520_e12081 + p.p1092);
        (assign9520_e12083,)
    } else {
        (var_arsd_total,)
    }
};
        var_arsd_total = assign9520_e12085;
        var_arsd_total_rv = 0.0;

        let (assign9530_e12094,) = {
    if (var_guard187 == 0.0) {
        let assign9530_e12090: f64 = (var_prsd * p.p5);
        let assign9530_e12092: f64 = (assign9530_e12090 + p.p1093);
        (assign9530_e12092,)
    } else {
        (var_prsd_total,)
    }
};
        var_prsd_total = assign9530_e12094;
        var_prsd_total_rv = 0.0;

        let (assign9540_e12106, assign9540_e12106_d_n0, assign9540_e12106_d_n2, assign9540_e12106_d_n3, assign9540_e12106_d_n4, assign9540_e12106_d_n5, assign9540_e12106_d_n6, assign9540_e12106_d_n7, assign9540_e12106_d_n8, assign9540_e12106_d_n9, assign9540_e12106_d_n10, assign9540_e12106_d_n11, assign9540_e12106_d_n13, assign9540_e12106_d_n14,) = {
    if (var_guard187 == 0.0) {
        let assign9540_e12099: f64 = (p.p1082 * var_arsd_total);
        let assign9540_e12102: f64 = (var_rhorsd_v * var_prsd_total);
        let assign9540_e12103: f64 = (assign9540_e12099 / assign9540_e12102);
        let assign9540_e12104: f64 = (assign9540_e12103).sqrt();
        (assign9540_e12104, ((-((assign9540_e12099 * (var_rhorsd_v_dn0 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn2 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn3 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn4 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn5 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn6 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn7 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn8 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn9 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn10 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn11 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn13 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (var_rhorsd_v_dn14 * var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)),)
    } else {
        (var_lt, var_lt_dn0, var_lt_dn2, var_lt_dn3, var_lt_dn4, var_lt_dn5, var_lt_dn6, var_lt_dn7, var_lt_dn8, var_lt_dn9, var_lt_dn10, var_lt_dn11, var_lt_dn13, var_lt_dn14,)
    }
};
        var_lt = assign9540_e12106;
        var_lt_dn0 = assign9540_e12106_d_n0;
        var_lt_dn2 = assign9540_e12106_d_n2;
        var_lt_dn3 = assign9540_e12106_d_n3;
        var_lt_dn4 = assign9540_e12106_d_n4;
        var_lt_dn5 = assign9540_e12106_d_n5;
        var_lt_dn6 = assign9540_e12106_d_n6;
        var_lt_dn7 = assign9540_e12106_d_n7;
        var_lt_dn8 = assign9540_e12106_d_n8;
        var_lt_dn9 = assign9540_e12106_d_n9;
        var_lt_dn10 = assign9540_e12106_d_n10;
        var_lt_dn11 = assign9540_e12106_d_n11;
        var_lt_dn13 = assign9540_e12106_d_n13;
        var_lt_dn14 = assign9540_e12106_d_n14;
        var_lt_rv = 0.0;

        let (assign9550_e12113, assign9550_e12113_d_n0, assign9550_e12113_d_n2, assign9550_e12113_d_n3, assign9550_e12113_d_n4, assign9550_e12113_d_n5, assign9550_e12113_d_n6, assign9550_e12113_d_n7, assign9550_e12113_d_n8, assign9550_e12113_d_n9, assign9550_e12113_d_n10, assign9550_e12113_d_n11, assign9550_e12113_d_n13, assign9550_e12113_d_n14,) = {
    if (var_guard187 == 0.0) {
        let assign9550_e12111: f64 = (p.p20 / var_lt);
        (assign9550_e12111, (-((p.p20 * var_lt_dn0) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn2) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn3) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn4) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn5) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn6) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn7) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn8) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn9) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn10) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn11) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn13) / (var_lt * var_lt))), (-((p.p20 * var_lt_dn14) / (var_lt * var_lt))),)
    } else {
        (var_alpha, var_alpha_dn0, var_alpha_dn2, var_alpha_dn3, var_alpha_dn4, var_alpha_dn5, var_alpha_dn6, var_alpha_dn7, var_alpha_dn8, var_alpha_dn9, var_alpha_dn10, var_alpha_dn11, var_alpha_dn13, var_alpha_dn14,)
    }
};
        var_alpha = assign9550_e12113;
        var_alpha_dn0 = assign9550_e12113_d_n0;
        var_alpha_dn2 = assign9550_e12113_d_n2;
        var_alpha_dn3 = assign9550_e12113_d_n3;
        var_alpha_dn4 = assign9550_e12113_d_n4;
        var_alpha_dn5 = assign9550_e12113_d_n5;
        var_alpha_dn6 = assign9550_e12113_d_n6;
        var_alpha_dn7 = assign9550_e12113_d_n7;
        var_alpha_dn8 = assign9550_e12113_d_n8;
        var_alpha_dn9 = assign9550_e12113_d_n9;
        var_alpha_dn10 = assign9550_e12113_d_n10;
        var_alpha_dn11 = assign9550_e12113_d_n11;
        var_alpha_dn13 = assign9550_e12113_d_n13;
        var_alpha_dn14 = assign9550_e12113_d_n14;
        var_alpha_rv = 0.0;

        let (assign9560_e12121, assign9560_e12121_d_n0, assign9560_e12121_d_n2, assign9560_e12121_d_n3, assign9560_e12121_d_n4, assign9560_e12121_d_n5, assign9560_e12121_d_n6, assign9560_e12121_d_n7, assign9560_e12121_d_n8, assign9560_e12121_d_n9, assign9560_e12121_d_n10, assign9560_e12121_d_n11, assign9560_e12121_d_n13, assign9560_e12121_d_n14,) = {
    if (var_guard187 == 0.0) {
        let assign9560_e12118: f64 = (2.0 * var_alpha);
        let assign9560_e12119: f64 = { let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign9560_e12119, ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn0)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn2)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn3)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn4)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn5)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn6)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn7)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn8)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn9)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn10)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn11)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn13)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * var_alpha_dn14)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign9560_e12121;
        var_t0_dn0 = assign9560_e12121_d_n0;
        var_t0_dn2 = assign9560_e12121_d_n2;
        var_t0_dn3 = assign9560_e12121_d_n3;
        var_t0_dn4 = assign9560_e12121_d_n4;
        var_t0_dn5 = assign9560_e12121_d_n5;
        var_t0_dn6 = assign9560_e12121_d_n6;
        var_t0_dn7 = assign9560_e12121_d_n7;
        var_t0_dn8 = assign9560_e12121_d_n8;
        var_t0_dn9 = assign9560_e12121_d_n9;
        var_t0_dn10 = assign9560_e12121_d_n10;
        var_t0_dn11 = assign9560_e12121_d_n11;
        var_t0_dn13 = assign9560_e12121_d_n13;
        var_t0_dn14 = assign9560_e12121_d_n14;
        var_t0_rv = 0.0;

        let assign9570_e12124: f64 = if p.p1086 == 1.0 { 1.0 } else { 0.0 };
        var_guard191 = assign9570_e12124;
        var_guard191_rv = 0.0;

        let (assign9580_e12135, assign9580_e12135_d_n0, assign9580_e12135_d_n2, assign9580_e12135_d_n3, assign9580_e12135_d_n4, assign9580_e12135_d_n5, assign9580_e12135_d_n6, assign9580_e12135_d_n7, assign9580_e12135_d_n8, assign9580_e12135_d_n9, assign9580_e12135_d_n10, assign9580_e12135_d_n11, assign9580_e12135_d_n13, assign9580_e12135_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard191 != 0.0)) {
        let assign9580_e12131: f64 = (var_rhorsd_v * var_lt);
        let assign9580_e12133: f64 = (assign9580_e12131 / p.p1082);
        (assign9580_e12133, (((var_rhorsd_v_dn0 * var_lt) + (var_rhorsd_v * var_lt_dn0)) / p.p1082), (((var_rhorsd_v_dn2 * var_lt) + (var_rhorsd_v * var_lt_dn2)) / p.p1082), (((var_rhorsd_v_dn3 * var_lt) + (var_rhorsd_v * var_lt_dn3)) / p.p1082), (((var_rhorsd_v_dn4 * var_lt) + (var_rhorsd_v * var_lt_dn4)) / p.p1082), (((var_rhorsd_v_dn5 * var_lt) + (var_rhorsd_v * var_lt_dn5)) / p.p1082), (((var_rhorsd_v_dn6 * var_lt) + (var_rhorsd_v * var_lt_dn6)) / p.p1082), (((var_rhorsd_v_dn7 * var_lt) + (var_rhorsd_v * var_lt_dn7)) / p.p1082), (((var_rhorsd_v_dn8 * var_lt) + (var_rhorsd_v * var_lt_dn8)) / p.p1082), (((var_rhorsd_v_dn9 * var_lt) + (var_rhorsd_v * var_lt_dn9)) / p.p1082), (((var_rhorsd_v_dn10 * var_lt) + (var_rhorsd_v * var_lt_dn10)) / p.p1082), (((var_rhorsd_v_dn11 * var_lt) + (var_rhorsd_v * var_lt_dn11)) / p.p1082), (((var_rhorsd_v_dn13 * var_lt) + (var_rhorsd_v * var_lt_dn13)) / p.p1082), (((var_rhorsd_v_dn14 * var_lt) + (var_rhorsd_v * var_lt_dn14)) / p.p1082),)
    } else {
        (var_eta, var_eta_dn0, var_eta_dn2, var_eta_dn3, var_eta_dn4, var_eta_dn5, var_eta_dn6, var_eta_dn7, var_eta_dn8, var_eta_dn9, var_eta_dn10, var_eta_dn11, var_eta_dn13, var_eta_dn14,)
    }
};
        var_eta = assign9580_e12135;
        var_eta_dn0 = assign9580_e12135_d_n0;
        var_eta_dn2 = assign9580_e12135_d_n2;
        var_eta_dn3 = assign9580_e12135_d_n3;
        var_eta_dn4 = assign9580_e12135_d_n4;
        var_eta_dn5 = assign9580_e12135_d_n5;
        var_eta_dn6 = assign9580_e12135_d_n6;
        var_eta_dn7 = assign9580_e12135_d_n7;
        var_eta_dn8 = assign9580_e12135_d_n8;
        var_eta_dn9 = assign9580_e12135_d_n9;
        var_eta_dn10 = assign9580_e12135_d_n10;
        var_eta_dn11 = assign9580_e12135_d_n11;
        var_eta_dn13 = assign9580_e12135_d_n13;
        var_eta_dn14 = assign9580_e12135_d_n14;
        var_eta_rv = 0.0;

        let (assign9590_e12146, assign9590_e12146_d_n0, assign9590_e12146_d_n2, assign9590_e12146_d_n3, assign9590_e12146_d_n4, assign9590_e12146_d_n5, assign9590_e12146_d_n6, assign9590_e12146_d_n7, assign9590_e12146_d_n8, assign9590_e12146_d_n9, assign9590_e12146_d_n10, assign9590_e12146_d_n11, assign9590_e12146_d_n13, assign9590_e12146_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard191 != 0.0)) {
        let assign9590_e12143: f64 = (1.0 + var_eta);
        let assign9590_e12144: f64 = (var_t0 * assign9590_e12143);
        (assign9590_e12144, ((var_t0_dn0 * assign9590_e12143) + (var_t0 * var_eta_dn0)), ((var_t0_dn2 * assign9590_e12143) + (var_t0 * var_eta_dn2)), ((var_t0_dn3 * assign9590_e12143) + (var_t0 * var_eta_dn3)), ((var_t0_dn4 * assign9590_e12143) + (var_t0 * var_eta_dn4)), ((var_t0_dn5 * assign9590_e12143) + (var_t0 * var_eta_dn5)), ((var_t0_dn6 * assign9590_e12143) + (var_t0 * var_eta_dn6)), ((var_t0_dn7 * assign9590_e12143) + (var_t0 * var_eta_dn7)), ((var_t0_dn8 * assign9590_e12143) + (var_t0 * var_eta_dn8)), ((var_t0_dn9 * assign9590_e12143) + (var_t0 * var_eta_dn9)), ((var_t0_dn10 * assign9590_e12143) + (var_t0 * var_eta_dn10)), ((var_t0_dn11 * assign9590_e12143) + (var_t0 * var_eta_dn11)), ((var_t0_dn13 * assign9590_e12143) + (var_t0 * var_eta_dn13)), ((var_t0_dn14 * assign9590_e12143) + (var_t0 * var_eta_dn14)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign9590_e12146;
        var_t1_dn0 = assign9590_e12146_d_n0;
        var_t1_dn2 = assign9590_e12146_d_n2;
        var_t1_dn3 = assign9590_e12146_d_n3;
        var_t1_dn4 = assign9590_e12146_d_n4;
        var_t1_dn5 = assign9590_e12146_d_n5;
        var_t1_dn6 = assign9590_e12146_d_n6;
        var_t1_dn7 = assign9590_e12146_d_n7;
        var_t1_dn8 = assign9590_e12146_d_n8;
        var_t1_dn9 = assign9590_e12146_d_n9;
        var_t1_dn10 = assign9590_e12146_d_n10;
        var_t1_dn11 = assign9590_e12146_d_n11;
        var_t1_dn13 = assign9590_e12146_d_n13;
        var_t1_dn14 = assign9590_e12146_d_n14;
        var_t1_rv = 0.0;

        let (assign9600_e12157, assign9600_e12157_d_n0, assign9600_e12157_d_n2, assign9600_e12157_d_n3, assign9600_e12157_d_n4, assign9600_e12157_d_n5, assign9600_e12157_d_n6, assign9600_e12157_d_n7, assign9600_e12157_d_n8, assign9600_e12157_d_n9, assign9600_e12157_d_n10, assign9600_e12157_d_n11, assign9600_e12157_d_n13, assign9600_e12157_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard191 != 0.0)) {
        let assign9600_e12153: f64 = (var_t1 + 1.0);
        let assign9600_e12155: f64 = (assign9600_e12153 - var_eta);
        (assign9600_e12155, (var_t1_dn0 - var_eta_dn0), (var_t1_dn2 - var_eta_dn2), (var_t1_dn3 - var_eta_dn3), (var_t1_dn4 - var_eta_dn4), (var_t1_dn5 - var_eta_dn5), (var_t1_dn6 - var_eta_dn6), (var_t1_dn7 - var_eta_dn7), (var_t1_dn8 - var_eta_dn8), (var_t1_dn9 - var_eta_dn9), (var_t1_dn10 - var_eta_dn10), (var_t1_dn11 - var_eta_dn11), (var_t1_dn13 - var_eta_dn13), (var_t1_dn14 - var_eta_dn14),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign9600_e12157;
        var_t2_dn0 = assign9600_e12157_d_n0;
        var_t2_dn2 = assign9600_e12157_d_n2;
        var_t2_dn3 = assign9600_e12157_d_n3;
        var_t2_dn4 = assign9600_e12157_d_n4;
        var_t2_dn5 = assign9600_e12157_d_n5;
        var_t2_dn6 = assign9600_e12157_d_n6;
        var_t2_dn7 = assign9600_e12157_d_n7;
        var_t2_dn8 = assign9600_e12157_d_n8;
        var_t2_dn9 = assign9600_e12157_d_n9;
        var_t2_dn10 = assign9600_e12157_d_n10;
        var_t2_dn11 = assign9600_e12157_d_n11;
        var_t2_dn13 = assign9600_e12157_d_n13;
        var_t2_dn14 = assign9600_e12157_d_n14;
        var_t2_rv = 0.0;

        let (assign9610_e12168, assign9610_e12168_d_n0, assign9610_e12168_d_n2, assign9610_e12168_d_n3, assign9610_e12168_d_n4, assign9610_e12168_d_n5, assign9610_e12168_d_n6, assign9610_e12168_d_n7, assign9610_e12168_d_n8, assign9610_e12168_d_n9, assign9610_e12168_d_n10, assign9610_e12168_d_n11, assign9610_e12168_d_n13, assign9610_e12168_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard191 != 0.0)) {
        let assign9610_e12164: f64 = (var_t1 - 1.0);
        let assign9610_e12166: f64 = (assign9610_e12164 + var_eta);
        (assign9610_e12166, (var_t1_dn0 + var_eta_dn0), (var_t1_dn2 + var_eta_dn2), (var_t1_dn3 + var_eta_dn3), (var_t1_dn4 + var_eta_dn4), (var_t1_dn5 + var_eta_dn5), (var_t1_dn6 + var_eta_dn6), (var_t1_dn7 + var_eta_dn7), (var_t1_dn8 + var_eta_dn8), (var_t1_dn9 + var_eta_dn9), (var_t1_dn10 + var_eta_dn10), (var_t1_dn11 + var_eta_dn11), (var_t1_dn13 + var_eta_dn13), (var_t1_dn14 + var_eta_dn14),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign9610_e12168;
        var_t3_dn0 = assign9610_e12168_d_n0;
        var_t3_dn2 = assign9610_e12168_d_n2;
        var_t3_dn3 = assign9610_e12168_d_n3;
        var_t3_dn4 = assign9610_e12168_d_n4;
        var_t3_dn5 = assign9610_e12168_d_n5;
        var_t3_dn6 = assign9610_e12168_d_n6;
        var_t3_dn7 = assign9610_e12168_d_n7;
        var_t3_dn8 = assign9610_e12168_d_n8;
        var_t3_dn9 = assign9610_e12168_d_n9;
        var_t3_dn10 = assign9610_e12168_d_n10;
        var_t3_dn11 = assign9610_e12168_d_n11;
        var_t3_dn13 = assign9610_e12168_d_n13;
        var_t3_dn14 = assign9610_e12168_d_n14;
        var_t3_rv = 0.0;

        let (assign9620_e12178, assign9620_e12178_d_n0, assign9620_e12178_d_n2, assign9620_e12178_d_n3, assign9620_e12178_d_n4, assign9620_e12178_d_n5, assign9620_e12178_d_n6, assign9620_e12178_d_n7, assign9620_e12178_d_n8, assign9620_e12178_d_n9, assign9620_e12178_d_n10, assign9620_e12178_d_n11, assign9620_e12178_d_n13, assign9620_e12178_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard191 == 0.0)) {
        let assign9620_e12176: f64 = (var_t0 + 1.0);
        (assign9620_e12176, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign9620_e12178;
        var_t2_dn0 = assign9620_e12178_d_n0;
        var_t2_dn2 = assign9620_e12178_d_n2;
        var_t2_dn3 = assign9620_e12178_d_n3;
        var_t2_dn4 = assign9620_e12178_d_n4;
        var_t2_dn5 = assign9620_e12178_d_n5;
        var_t2_dn6 = assign9620_e12178_d_n6;
        var_t2_dn7 = assign9620_e12178_d_n7;
        var_t2_dn8 = assign9620_e12178_d_n8;
        var_t2_dn9 = assign9620_e12178_d_n9;
        var_t2_dn10 = assign9620_e12178_d_n10;
        var_t2_dn11 = assign9620_e12178_d_n11;
        var_t2_dn13 = assign9620_e12178_d_n13;
        var_t2_dn14 = assign9620_e12178_d_n14;
        var_t2_rv = 0.0;

        *var_afin_slot = var_afin;
        *var_afin_rv_slot = var_afin_rv;
        *var_alpha_slot = var_alpha;
        *var_alpha_dn0_slot = var_alpha_dn0;
        *var_alpha_dn10_slot = var_alpha_dn10;
        *var_alpha_dn11_slot = var_alpha_dn11;
        *var_alpha_dn13_slot = var_alpha_dn13;
        *var_alpha_dn14_slot = var_alpha_dn14;
        *var_alpha_dn2_slot = var_alpha_dn2;
        *var_alpha_dn3_slot = var_alpha_dn3;
        *var_alpha_dn4_slot = var_alpha_dn4;
        *var_alpha_dn5_slot = var_alpha_dn5;
        *var_alpha_dn6_slot = var_alpha_dn6;
        *var_alpha_dn7_slot = var_alpha_dn7;
        *var_alpha_dn8_slot = var_alpha_dn8;
        *var_alpha_dn9_slot = var_alpha_dn9;
        *var_alpha_rv_slot = var_alpha_rv;
        *var_arsd_total_slot = var_arsd_total;
        *var_arsd_total_rv_slot = var_arsd_total_rv;
        *var_eta_slot = var_eta;
        *var_eta_dn0_slot = var_eta_dn0;
        *var_eta_dn10_slot = var_eta_dn10;
        *var_eta_dn11_slot = var_eta_dn11;
        *var_eta_dn13_slot = var_eta_dn13;
        *var_eta_dn14_slot = var_eta_dn14;
        *var_eta_dn2_slot = var_eta_dn2;
        *var_eta_dn3_slot = var_eta_dn3;
        *var_eta_dn4_slot = var_eta_dn4;
        *var_eta_dn5_slot = var_eta_dn5;
        *var_eta_dn6_slot = var_eta_dn6;
        *var_eta_dn7_slot = var_eta_dn7;
        *var_eta_dn8_slot = var_eta_dn8;
        *var_eta_dn9_slot = var_eta_dn9;
        *var_eta_rv_slot = var_eta_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_lt_slot = var_lt;
        *var_lt_dn0_slot = var_lt_dn0;
        *var_lt_dn10_slot = var_lt_dn10;
        *var_lt_dn11_slot = var_lt_dn11;
        *var_lt_dn13_slot = var_lt_dn13;
        *var_lt_dn14_slot = var_lt_dn14;
        *var_lt_dn2_slot = var_lt_dn2;
        *var_lt_dn3_slot = var_lt_dn3;
        *var_lt_dn4_slot = var_lt_dn4;
        *var_lt_dn5_slot = var_lt_dn5;
        *var_lt_dn6_slot = var_lt_dn6;
        *var_lt_dn7_slot = var_lt_dn7;
        *var_lt_dn8_slot = var_lt_dn8;
        *var_lt_dn9_slot = var_lt_dn9;
        *var_lt_rv_slot = var_lt_rv;
        *var_mu_max_slot = var_mu_max;
        *var_mu_max_rv_slot = var_mu_max_rv;
        *var_mu_rsd_slot = var_mu_rsd;
        *var_mu_rsd_dn0_slot = var_mu_rsd_dn0;
        *var_mu_rsd_dn10_slot = var_mu_rsd_dn10;
        *var_mu_rsd_dn11_slot = var_mu_rsd_dn11;
        *var_mu_rsd_dn13_slot = var_mu_rsd_dn13;
        *var_mu_rsd_dn14_slot = var_mu_rsd_dn14;
        *var_mu_rsd_dn2_slot = var_mu_rsd_dn2;
        *var_mu_rsd_dn3_slot = var_mu_rsd_dn3;
        *var_mu_rsd_dn4_slot = var_mu_rsd_dn4;
        *var_mu_rsd_dn5_slot = var_mu_rsd_dn5;
        *var_mu_rsd_dn6_slot = var_mu_rsd_dn6;
        *var_mu_rsd_dn7_slot = var_mu_rsd_dn7;
        *var_mu_rsd_dn8_slot = var_mu_rsd_dn8;
        *var_mu_rsd_dn9_slot = var_mu_rsd_dn9;
        *var_mu_rsd_rv_slot = var_mu_rsd_rv;
        *var_prsd_total_slot = var_prsd_total;
        *var_prsd_total_rv_slot = var_prsd_total_rv;
        *var_rhorsd_v_slot = var_rhorsd_v;
        *var_rhorsd_v_dn0_slot = var_rhorsd_v_dn0;
        *var_rhorsd_v_dn10_slot = var_rhorsd_v_dn10;
        *var_rhorsd_v_dn11_slot = var_rhorsd_v_dn11;
        *var_rhorsd_v_dn13_slot = var_rhorsd_v_dn13;
        *var_rhorsd_v_dn14_slot = var_rhorsd_v_dn14;
        *var_rhorsd_v_dn2_slot = var_rhorsd_v_dn2;
        *var_rhorsd_v_dn3_slot = var_rhorsd_v_dn3;
        *var_rhorsd_v_dn4_slot = var_rhorsd_v_dn4;
        *var_rhorsd_v_dn5_slot = var_rhorsd_v_dn5;
        *var_rhorsd_v_dn6_slot = var_rhorsd_v_dn6;
        *var_rhorsd_v_dn7_slot = var_rhorsd_v_dn7;
        *var_rhorsd_v_dn8_slot = var_rhorsd_v_dn8;
        *var_rhorsd_v_dn9_slot = var_rhorsd_v_dn9;
        *var_rhorsd_v_rv_slot = var_rhorsd_v_rv;
        *var_rsp_slot = var_rsp;
        *var_rsp_dn0_slot = var_rsp_dn0;
        *var_rsp_dn10_slot = var_rsp_dn10;
        *var_rsp_dn11_slot = var_rsp_dn11;
        *var_rsp_dn13_slot = var_rsp_dn13;
        *var_rsp_dn14_slot = var_rsp_dn14;
        *var_rsp_dn2_slot = var_rsp_dn2;
        *var_rsp_dn3_slot = var_rsp_dn3;
        *var_rsp_dn4_slot = var_rsp_dn4;
        *var_rsp_dn5_slot = var_rsp_dn5;
        *var_rsp_dn6_slot = var_rsp_dn6;
        *var_rsp_dn7_slot = var_rsp_dn7;
        *var_rsp_dn8_slot = var_rsp_dn8;
        *var_rsp_dn9_slot = var_rsp_dn9;
        *var_rsp_rv_slot = var_rsp_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_thetarsp_slot = var_thetarsp;
        *var_thetarsp_rv_slot = var_thetarsp_rv;
    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_arsd_total: f64,
        var_cgdl_i: f64,
        var_cgsl_i: f64,
        var_cox: f64,
        var_guard187: f64,
        var_guard191: f64,
        var_lt: f64,
        var_lt_dn0: f64,
        var_lt_dn10: f64,
        var_lt_dn11: f64,
        var_lt_dn13: f64,
        var_lt_dn14: f64,
        var_lt_dn2: f64,
        var_lt_dn3: f64,
        var_lt_dn4: f64,
        var_lt_dn5: f64,
        var_lt_dn6: f64,
        var_lt_dn7: f64,
        var_lt_dn8: f64,
        var_lt_dn9: f64,
        var_rhorsd_v: f64,
        var_rhorsd_v_dn0: f64,
        var_rhorsd_v_dn10: f64,
        var_rhorsd_v_dn11: f64,
        var_rhorsd_v_dn13: f64,
        var_rhorsd_v_dn14: f64,
        var_rhorsd_v_dn2: f64,
        var_rhorsd_v_dn3: f64,
        var_rhorsd_v_dn4: f64,
        var_rhorsd_v_dn5: f64,
        var_rhorsd_v_dn6: f64,
        var_rhorsd_v_dn7: f64,
        var_rhorsd_v_dn8: f64,
        var_rhorsd_v_dn9: f64,
        var_rsp: f64,
        var_rsp_dn0: f64,
        var_rsp_dn10: f64,
        var_rsp_dn11: f64,
        var_rsp_dn13: f64,
        var_rsp_dn14: f64,
        var_rsp_dn2: f64,
        var_rsp_dn3: f64,
        var_rsp_dn4: f64,
        var_rsp_dn5: f64,
        var_rsp_dn6: f64,
        var_rsp_dn7: f64,
        var_rsp_dn8: f64,
        var_rsp_dn9: f64,
        var_t0: f64,
        var_t0_dn0: f64,
        var_t0_dn10: f64,
        var_t0_dn11: f64,
        var_t0_dn13: f64,
        var_t0_dn14: f64,
        var_t0_dn2: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_t2: f64,
        var_t2_dn0: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn13: f64,
        var_t2_dn14: f64,
        var_t2_dn2: f64,
        var_t2_dn3: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t2_dn9: f64,
        var_cgdo_i_slot: &mut f64,
        var_cgdo_i_rv_slot: &mut f64,
        var_cgso_i_slot: &mut f64,
        var_cgso_i_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard193_slot: &mut f64,
        var_guard193_rv_slot: &mut f64,
        var_guard194_slot: &mut f64,
        var_guard194_rv_slot: &mut f64,
        var_guard195_slot: &mut f64,
        var_guard195_rv_slot: &mut f64,
        var_guard196_slot: &mut f64,
        var_guard196_rv_slot: &mut f64,
        var_guard197_slot: &mut f64,
        var_guard197_rv_slot: &mut f64,
        var_guard198_slot: &mut f64,
        var_guard198_rv_slot: &mut f64,
        var_guard199_slot: &mut f64,
        var_guard199_rv_slot: &mut f64,
        var_guard200_slot: &mut f64,
        var_guard200_rv_slot: &mut f64,
        var_guard201_slot: &mut f64,
        var_guard201_rv_slot: &mut f64,
        var_guard202_slot: &mut f64,
        var_guard202_rv_slot: &mut f64,
        var_guard203_slot: &mut f64,
        var_guard203_rv_slot: &mut f64,
        var_guard204_slot: &mut f64,
        var_guard204_rv_slot: &mut f64,
        var_guard205_slot: &mut f64,
        var_guard205_rv_slot: &mut f64,
        var_guard206_slot: &mut f64,
        var_guard206_rv_slot: &mut f64,
        var_hg_slot: &mut f64,
        var_hg_rv_slot: &mut f64,
        var_hrsd_slot: &mut f64,
        var_hrsd_rv_slot: &mut f64,
        var_rdraingeo_slot: &mut f64,
        var_rdraingeo_dn0_slot: &mut f64,
        var_rdraingeo_dn10_slot: &mut f64,
        var_rdraingeo_dn11_slot: &mut f64,
        var_rdraingeo_dn13_slot: &mut f64,
        var_rdraingeo_dn14_slot: &mut f64,
        var_rdraingeo_dn2_slot: &mut f64,
        var_rdraingeo_dn3_slot: &mut f64,
        var_rdraingeo_dn4_slot: &mut f64,
        var_rdraingeo_dn5_slot: &mut f64,
        var_rdraingeo_dn6_slot: &mut f64,
        var_rdraingeo_dn7_slot: &mut f64,
        var_rdraingeo_dn8_slot: &mut f64,
        var_rdraingeo_dn9_slot: &mut f64,
        var_rdraingeo_rv_slot: &mut f64,
        var_rdsgeo_slot: &mut f64,
        var_rdsgeo_dn0_slot: &mut f64,
        var_rdsgeo_dn10_slot: &mut f64,
        var_rdsgeo_dn11_slot: &mut f64,
        var_rdsgeo_dn13_slot: &mut f64,
        var_rdsgeo_dn14_slot: &mut f64,
        var_rdsgeo_dn2_slot: &mut f64,
        var_rdsgeo_dn3_slot: &mut f64,
        var_rdsgeo_dn4_slot: &mut f64,
        var_rdsgeo_dn5_slot: &mut f64,
        var_rdsgeo_dn6_slot: &mut f64,
        var_rdsgeo_dn7_slot: &mut f64,
        var_rdsgeo_dn8_slot: &mut f64,
        var_rdsgeo_dn9_slot: &mut f64,
        var_rdsgeo_rv_slot: &mut f64,
        var_rrsd_slot: &mut f64,
        var_rrsd_dn0_slot: &mut f64,
        var_rrsd_dn10_slot: &mut f64,
        var_rrsd_dn11_slot: &mut f64,
        var_rrsd_dn13_slot: &mut f64,
        var_rrsd_dn14_slot: &mut f64,
        var_rrsd_dn2_slot: &mut f64,
        var_rrsd_dn3_slot: &mut f64,
        var_rrsd_dn4_slot: &mut f64,
        var_rrsd_dn5_slot: &mut f64,
        var_rrsd_dn6_slot: &mut f64,
        var_rrsd_dn7_slot: &mut f64,
        var_rrsd_dn8_slot: &mut f64,
        var_rrsd_dn9_slot: &mut f64,
        var_rrsd_rv_slot: &mut f64,
        var_rrsdside_slot: &mut f64,
        var_rrsdside_rv_slot: &mut f64,
        var_rrsdtml_slot: &mut f64,
        var_rrsdtml_dn0_slot: &mut f64,
        var_rrsdtml_dn10_slot: &mut f64,
        var_rrsdtml_dn11_slot: &mut f64,
        var_rrsdtml_dn13_slot: &mut f64,
        var_rrsdtml_dn14_slot: &mut f64,
        var_rrsdtml_dn2_slot: &mut f64,
        var_rrsdtml_dn3_slot: &mut f64,
        var_rrsdtml_dn4_slot: &mut f64,
        var_rrsdtml_dn5_slot: &mut f64,
        var_rrsdtml_dn6_slot: &mut f64,
        var_rrsdtml_dn7_slot: &mut f64,
        var_rrsdtml_dn8_slot: &mut f64,
        var_rrsdtml_dn9_slot: &mut f64,
        var_rrsdtml_rv_slot: &mut f64,
        var_rsourcegeo_slot: &mut f64,
        var_rsourcegeo_dn0_slot: &mut f64,
        var_rsourcegeo_dn10_slot: &mut f64,
        var_rsourcegeo_dn11_slot: &mut f64,
        var_rsourcegeo_dn13_slot: &mut f64,
        var_rsourcegeo_dn14_slot: &mut f64,
        var_rsourcegeo_dn2_slot: &mut f64,
        var_rsourcegeo_dn3_slot: &mut f64,
        var_rsourcegeo_dn4_slot: &mut f64,
        var_rsourcegeo_dn5_slot: &mut f64,
        var_rsourcegeo_dn6_slot: &mut f64,
        var_rsourcegeo_dn7_slot: &mut f64,
        var_rsourcegeo_dn8_slot: &mut f64,
        var_rsourcegeo_dn9_slot: &mut f64,
        var_rsourcegeo_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_trsd_slot: &mut f64,
        var_trsd_rv_slot: &mut f64,
        var_wg_slot: &mut f64,
        var_wg_rv_slot: &mut f64,
    ) {
        let mut var_cgdo_i: f64 = *var_cgdo_i_slot;
        let mut var_cgdo_i_rv: f64 = *var_cgdo_i_rv_slot;
        let mut var_cgso_i: f64 = *var_cgso_i_slot;
        let mut var_cgso_i_rv: f64 = *var_cgso_i_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard193: f64 = *var_guard193_slot;
        let mut var_guard193_rv: f64 = *var_guard193_rv_slot;
        let mut var_guard194: f64 = *var_guard194_slot;
        let mut var_guard194_rv: f64 = *var_guard194_rv_slot;
        let mut var_guard195: f64 = *var_guard195_slot;
        let mut var_guard195_rv: f64 = *var_guard195_rv_slot;
        let mut var_guard196: f64 = *var_guard196_slot;
        let mut var_guard196_rv: f64 = *var_guard196_rv_slot;
        let mut var_guard197: f64 = *var_guard197_slot;
        let mut var_guard197_rv: f64 = *var_guard197_rv_slot;
        let mut var_guard198: f64 = *var_guard198_slot;
        let mut var_guard198_rv: f64 = *var_guard198_rv_slot;
        let mut var_guard199: f64 = *var_guard199_slot;
        let mut var_guard199_rv: f64 = *var_guard199_rv_slot;
        let mut var_guard200: f64 = *var_guard200_slot;
        let mut var_guard200_rv: f64 = *var_guard200_rv_slot;
        let mut var_guard201: f64 = *var_guard201_slot;
        let mut var_guard201_rv: f64 = *var_guard201_rv_slot;
        let mut var_guard202: f64 = *var_guard202_slot;
        let mut var_guard202_rv: f64 = *var_guard202_rv_slot;
        let mut var_guard203: f64 = *var_guard203_slot;
        let mut var_guard203_rv: f64 = *var_guard203_rv_slot;
        let mut var_guard204: f64 = *var_guard204_slot;
        let mut var_guard204_rv: f64 = *var_guard204_rv_slot;
        let mut var_guard205: f64 = *var_guard205_slot;
        let mut var_guard205_rv: f64 = *var_guard205_rv_slot;
        let mut var_guard206: f64 = *var_guard206_slot;
        let mut var_guard206_rv: f64 = *var_guard206_rv_slot;
        let mut var_hg: f64 = *var_hg_slot;
        let mut var_hg_rv: f64 = *var_hg_rv_slot;
        let mut var_hrsd: f64 = *var_hrsd_slot;
        let mut var_hrsd_rv: f64 = *var_hrsd_rv_slot;
        let mut var_rdraingeo: f64 = *var_rdraingeo_slot;
        let mut var_rdraingeo_dn0: f64 = *var_rdraingeo_dn0_slot;
        let mut var_rdraingeo_dn10: f64 = *var_rdraingeo_dn10_slot;
        let mut var_rdraingeo_dn11: f64 = *var_rdraingeo_dn11_slot;
        let mut var_rdraingeo_dn13: f64 = *var_rdraingeo_dn13_slot;
        let mut var_rdraingeo_dn14: f64 = *var_rdraingeo_dn14_slot;
        let mut var_rdraingeo_dn2: f64 = *var_rdraingeo_dn2_slot;
        let mut var_rdraingeo_dn3: f64 = *var_rdraingeo_dn3_slot;
        let mut var_rdraingeo_dn4: f64 = *var_rdraingeo_dn4_slot;
        let mut var_rdraingeo_dn5: f64 = *var_rdraingeo_dn5_slot;
        let mut var_rdraingeo_dn6: f64 = *var_rdraingeo_dn6_slot;
        let mut var_rdraingeo_dn7: f64 = *var_rdraingeo_dn7_slot;
        let mut var_rdraingeo_dn8: f64 = *var_rdraingeo_dn8_slot;
        let mut var_rdraingeo_dn9: f64 = *var_rdraingeo_dn9_slot;
        let mut var_rdraingeo_rv: f64 = *var_rdraingeo_rv_slot;
        let mut var_rdsgeo: f64 = *var_rdsgeo_slot;
        let mut var_rdsgeo_dn0: f64 = *var_rdsgeo_dn0_slot;
        let mut var_rdsgeo_dn10: f64 = *var_rdsgeo_dn10_slot;
        let mut var_rdsgeo_dn11: f64 = *var_rdsgeo_dn11_slot;
        let mut var_rdsgeo_dn13: f64 = *var_rdsgeo_dn13_slot;
        let mut var_rdsgeo_dn14: f64 = *var_rdsgeo_dn14_slot;
        let mut var_rdsgeo_dn2: f64 = *var_rdsgeo_dn2_slot;
        let mut var_rdsgeo_dn3: f64 = *var_rdsgeo_dn3_slot;
        let mut var_rdsgeo_dn4: f64 = *var_rdsgeo_dn4_slot;
        let mut var_rdsgeo_dn5: f64 = *var_rdsgeo_dn5_slot;
        let mut var_rdsgeo_dn6: f64 = *var_rdsgeo_dn6_slot;
        let mut var_rdsgeo_dn7: f64 = *var_rdsgeo_dn7_slot;
        let mut var_rdsgeo_dn8: f64 = *var_rdsgeo_dn8_slot;
        let mut var_rdsgeo_dn9: f64 = *var_rdsgeo_dn9_slot;
        let mut var_rdsgeo_rv: f64 = *var_rdsgeo_rv_slot;
        let mut var_rrsd: f64 = *var_rrsd_slot;
        let mut var_rrsd_dn0: f64 = *var_rrsd_dn0_slot;
        let mut var_rrsd_dn10: f64 = *var_rrsd_dn10_slot;
        let mut var_rrsd_dn11: f64 = *var_rrsd_dn11_slot;
        let mut var_rrsd_dn13: f64 = *var_rrsd_dn13_slot;
        let mut var_rrsd_dn14: f64 = *var_rrsd_dn14_slot;
        let mut var_rrsd_dn2: f64 = *var_rrsd_dn2_slot;
        let mut var_rrsd_dn3: f64 = *var_rrsd_dn3_slot;
        let mut var_rrsd_dn4: f64 = *var_rrsd_dn4_slot;
        let mut var_rrsd_dn5: f64 = *var_rrsd_dn5_slot;
        let mut var_rrsd_dn6: f64 = *var_rrsd_dn6_slot;
        let mut var_rrsd_dn7: f64 = *var_rrsd_dn7_slot;
        let mut var_rrsd_dn8: f64 = *var_rrsd_dn8_slot;
        let mut var_rrsd_dn9: f64 = *var_rrsd_dn9_slot;
        let mut var_rrsd_rv: f64 = *var_rrsd_rv_slot;
        let mut var_rrsdside: f64 = *var_rrsdside_slot;
        let mut var_rrsdside_rv: f64 = *var_rrsdside_rv_slot;
        let mut var_rrsdtml: f64 = *var_rrsdtml_slot;
        let mut var_rrsdtml_dn0: f64 = *var_rrsdtml_dn0_slot;
        let mut var_rrsdtml_dn10: f64 = *var_rrsdtml_dn10_slot;
        let mut var_rrsdtml_dn11: f64 = *var_rrsdtml_dn11_slot;
        let mut var_rrsdtml_dn13: f64 = *var_rrsdtml_dn13_slot;
        let mut var_rrsdtml_dn14: f64 = *var_rrsdtml_dn14_slot;
        let mut var_rrsdtml_dn2: f64 = *var_rrsdtml_dn2_slot;
        let mut var_rrsdtml_dn3: f64 = *var_rrsdtml_dn3_slot;
        let mut var_rrsdtml_dn4: f64 = *var_rrsdtml_dn4_slot;
        let mut var_rrsdtml_dn5: f64 = *var_rrsdtml_dn5_slot;
        let mut var_rrsdtml_dn6: f64 = *var_rrsdtml_dn6_slot;
        let mut var_rrsdtml_dn7: f64 = *var_rrsdtml_dn7_slot;
        let mut var_rrsdtml_dn8: f64 = *var_rrsdtml_dn8_slot;
        let mut var_rrsdtml_dn9: f64 = *var_rrsdtml_dn9_slot;
        let mut var_rrsdtml_rv: f64 = *var_rrsdtml_rv_slot;
        let mut var_rsourcegeo: f64 = *var_rsourcegeo_slot;
        let mut var_rsourcegeo_dn0: f64 = *var_rsourcegeo_dn0_slot;
        let mut var_rsourcegeo_dn10: f64 = *var_rsourcegeo_dn10_slot;
        let mut var_rsourcegeo_dn11: f64 = *var_rsourcegeo_dn11_slot;
        let mut var_rsourcegeo_dn13: f64 = *var_rsourcegeo_dn13_slot;
        let mut var_rsourcegeo_dn14: f64 = *var_rsourcegeo_dn14_slot;
        let mut var_rsourcegeo_dn2: f64 = *var_rsourcegeo_dn2_slot;
        let mut var_rsourcegeo_dn3: f64 = *var_rsourcegeo_dn3_slot;
        let mut var_rsourcegeo_dn4: f64 = *var_rsourcegeo_dn4_slot;
        let mut var_rsourcegeo_dn5: f64 = *var_rsourcegeo_dn5_slot;
        let mut var_rsourcegeo_dn6: f64 = *var_rsourcegeo_dn6_slot;
        let mut var_rsourcegeo_dn7: f64 = *var_rsourcegeo_dn7_slot;
        let mut var_rsourcegeo_dn8: f64 = *var_rsourcegeo_dn8_slot;
        let mut var_rsourcegeo_dn9: f64 = *var_rsourcegeo_dn9_slot;
        let mut var_rsourcegeo_rv: f64 = *var_rsourcegeo_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_trsd: f64 = *var_trsd_slot;
        let mut var_trsd_rv: f64 = *var_trsd_rv_slot;
        let mut var_wg: f64 = *var_wg_slot;
        let mut var_wg_rv: f64 = *var_wg_rv_slot;

        let (assign9630_e12188, assign9630_e12188_d_n0, assign9630_e12188_d_n2, assign9630_e12188_d_n3, assign9630_e12188_d_n4, assign9630_e12188_d_n5, assign9630_e12188_d_n6, assign9630_e12188_d_n7, assign9630_e12188_d_n8, assign9630_e12188_d_n9, assign9630_e12188_d_n10, assign9630_e12188_d_n11, assign9630_e12188_d_n13, assign9630_e12188_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard191 == 0.0)) {
        let assign9630_e12186: f64 = (var_t0 - 1.0);
        (assign9630_e12186, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign9630_e12188;
        var_t3_dn0 = assign9630_e12188_d_n0;
        var_t3_dn2 = assign9630_e12188_d_n2;
        var_t3_dn3 = assign9630_e12188_d_n3;
        var_t3_dn4 = assign9630_e12188_d_n4;
        var_t3_dn5 = assign9630_e12188_d_n5;
        var_t3_dn6 = assign9630_e12188_d_n6;
        var_t3_dn7 = assign9630_e12188_d_n7;
        var_t3_dn8 = assign9630_e12188_d_n8;
        var_t3_dn9 = assign9630_e12188_d_n9;
        var_t3_dn10 = assign9630_e12188_d_n10;
        var_t3_dn11 = assign9630_e12188_d_n11;
        var_t3_dn13 = assign9630_e12188_d_n13;
        var_t3_dn14 = assign9630_e12188_d_n14;
        var_t3_rv = 0.0;

        let (assign9640_e12201, assign9640_e12201_d_n0, assign9640_e12201_d_n2, assign9640_e12201_d_n3, assign9640_e12201_d_n4, assign9640_e12201_d_n5, assign9640_e12201_d_n6, assign9640_e12201_d_n7, assign9640_e12201_d_n8, assign9640_e12201_d_n9, assign9640_e12201_d_n10, assign9640_e12201_d_n11, assign9640_e12201_d_n13, assign9640_e12201_d_n14,) = {
    if (var_guard187 == 0.0) {
        let assign9640_e12193: f64 = (var_rhorsd_v * var_lt);
        let assign9640_e12195: f64 = (assign9640_e12193 * var_t2);
        let assign9640_e12198: f64 = (var_arsd_total * var_t3);
        let assign9640_e12199: f64 = (assign9640_e12195 / assign9640_e12198);
        (assign9640_e12199, (((((((var_rhorsd_v_dn0 * var_lt) + (var_rhorsd_v * var_lt_dn0)) * var_t2) + (assign9640_e12193 * var_t2_dn0)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn0))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn2 * var_lt) + (var_rhorsd_v * var_lt_dn2)) * var_t2) + (assign9640_e12193 * var_t2_dn2)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn2))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn3 * var_lt) + (var_rhorsd_v * var_lt_dn3)) * var_t2) + (assign9640_e12193 * var_t2_dn3)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn3))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn4 * var_lt) + (var_rhorsd_v * var_lt_dn4)) * var_t2) + (assign9640_e12193 * var_t2_dn4)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn4))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn5 * var_lt) + (var_rhorsd_v * var_lt_dn5)) * var_t2) + (assign9640_e12193 * var_t2_dn5)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn5))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn6 * var_lt) + (var_rhorsd_v * var_lt_dn6)) * var_t2) + (assign9640_e12193 * var_t2_dn6)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn6))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn7 * var_lt) + (var_rhorsd_v * var_lt_dn7)) * var_t2) + (assign9640_e12193 * var_t2_dn7)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn7))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn8 * var_lt) + (var_rhorsd_v * var_lt_dn8)) * var_t2) + (assign9640_e12193 * var_t2_dn8)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn8))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn9 * var_lt) + (var_rhorsd_v * var_lt_dn9)) * var_t2) + (assign9640_e12193 * var_t2_dn9)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn9))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn10 * var_lt) + (var_rhorsd_v * var_lt_dn10)) * var_t2) + (assign9640_e12193 * var_t2_dn10)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn10))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn11 * var_lt) + (var_rhorsd_v * var_lt_dn11)) * var_t2) + (assign9640_e12193 * var_t2_dn11)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn11))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn13 * var_lt) + (var_rhorsd_v * var_lt_dn13)) * var_t2) + (assign9640_e12193 * var_t2_dn13)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn13))) / (assign9640_e12198 * assign9640_e12198)), (((((((var_rhorsd_v_dn14 * var_lt) + (var_rhorsd_v * var_lt_dn14)) * var_t2) + (assign9640_e12193 * var_t2_dn14)) * assign9640_e12198) - (assign9640_e12195 * (var_arsd_total * var_t3_dn14))) / (assign9640_e12198 * assign9640_e12198)),)
    } else {
        (var_rrsdtml, var_rrsdtml_dn0, var_rrsdtml_dn2, var_rrsdtml_dn3, var_rrsdtml_dn4, var_rrsdtml_dn5, var_rrsdtml_dn6, var_rrsdtml_dn7, var_rrsdtml_dn8, var_rrsdtml_dn9, var_rrsdtml_dn10, var_rrsdtml_dn11, var_rrsdtml_dn13, var_rrsdtml_dn14,)
    }
};
        var_rrsdtml = assign9640_e12201;
        var_rrsdtml_dn0 = assign9640_e12201_d_n0;
        var_rrsdtml_dn2 = assign9640_e12201_d_n2;
        var_rrsdtml_dn3 = assign9640_e12201_d_n3;
        var_rrsdtml_dn4 = assign9640_e12201_d_n4;
        var_rrsdtml_dn5 = assign9640_e12201_d_n5;
        var_rrsdtml_dn6 = assign9640_e12201_d_n6;
        var_rrsdtml_dn7 = assign9640_e12201_d_n7;
        var_rrsdtml_dn8 = assign9640_e12201_d_n8;
        var_rrsdtml_dn9 = assign9640_e12201_d_n9;
        var_rrsdtml_dn10 = assign9640_e12201_d_n10;
        var_rrsdtml_dn11 = assign9640_e12201_d_n11;
        var_rrsdtml_dn13 = assign9640_e12201_d_n13;
        var_rrsdtml_dn14 = assign9640_e12201_d_n14;
        var_rrsdtml_rv = 0.0;

        let assign9650_e12204: f64 = (-1e-10);
        let assign9650_e12205: f64 = if p.p1080 < assign9650_e12204 { 1.0 } else { 0.0 };
        var_guard192 = assign9650_e12205;
        var_guard192_rv = 0.0;

        let (assign9660_e12219,) = {
    if ((var_guard187 == 0.0) && (var_guard192 != 0.0)) {
        let assign9660_e12212: f64 = (-p.p1080);
        let assign9660_e12214: f64 = (assign9660_e12212 * p.p3);
        let assign9660_e12216: f64 = (assign9660_e12214 * p.p5);
        let assign9660_e12217: f64 = (p.p1082 / assign9660_e12216);
        (assign9660_e12217,)
    } else {
        (var_rrsdside,)
    }
};
        var_rrsdside = assign9660_e12219;
        var_rrsdside_rv = 0.0;

        let (assign9670_e12236, assign9670_e12236_d_n0, assign9670_e12236_d_n2, assign9670_e12236_d_n3, assign9670_e12236_d_n4, assign9670_e12236_d_n5, assign9670_e12236_d_n6, assign9670_e12236_d_n7, assign9670_e12236_d_n8, assign9670_e12236_d_n9, assign9670_e12236_d_n10, assign9670_e12236_d_n11, assign9670_e12236_d_n13, assign9670_e12236_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard192 != 0.0)) {
        let assign9670_e12226: f64 = (var_rrsdtml + var_rsp);
        let assign9670_e12228: f64 = (assign9670_e12226 * var_rrsdside);
        let assign9670_e12231: f64 = (var_rrsdtml + var_rsp);
        let assign9670_e12233: f64 = (assign9670_e12231 + var_rrsdside);
        let assign9670_e12234: f64 = (assign9670_e12228 / assign9670_e12233);
        (assign9670_e12234, (((((var_rrsdtml_dn0 + var_rsp_dn0) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn0 + var_rsp_dn0))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn2 + var_rsp_dn2) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn2 + var_rsp_dn2))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn3 + var_rsp_dn3) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn3 + var_rsp_dn3))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn4 + var_rsp_dn4) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn4 + var_rsp_dn4))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn5 + var_rsp_dn5) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn5 + var_rsp_dn5))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn6 + var_rsp_dn6) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn6 + var_rsp_dn6))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn7 + var_rsp_dn7) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn7 + var_rsp_dn7))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn8 + var_rsp_dn8) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn8 + var_rsp_dn8))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn9 + var_rsp_dn9) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn9 + var_rsp_dn9))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn10 + var_rsp_dn10) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn10 + var_rsp_dn10))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn11 + var_rsp_dn11) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn11 + var_rsp_dn11))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn13 + var_rsp_dn13) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn13 + var_rsp_dn13))) / (assign9670_e12233 * assign9670_e12233)), (((((var_rrsdtml_dn14 + var_rsp_dn14) * var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (var_rrsdtml_dn14 + var_rsp_dn14))) / (assign9670_e12233 * assign9670_e12233)),)
    } else {
        (var_rrsd, var_rrsd_dn0, var_rrsd_dn2, var_rrsd_dn3, var_rrsd_dn4, var_rrsd_dn5, var_rrsd_dn6, var_rrsd_dn7, var_rrsd_dn8, var_rrsd_dn9, var_rrsd_dn10, var_rrsd_dn11, var_rrsd_dn13, var_rrsd_dn14,)
    }
};
        var_rrsd = assign9670_e12236;
        var_rrsd_dn0 = assign9670_e12236_d_n0;
        var_rrsd_dn2 = assign9670_e12236_d_n2;
        var_rrsd_dn3 = assign9670_e12236_d_n3;
        var_rrsd_dn4 = assign9670_e12236_d_n4;
        var_rrsd_dn5 = assign9670_e12236_d_n5;
        var_rrsd_dn6 = assign9670_e12236_d_n6;
        var_rrsd_dn7 = assign9670_e12236_d_n7;
        var_rrsd_dn8 = assign9670_e12236_d_n8;
        var_rrsd_dn9 = assign9670_e12236_d_n9;
        var_rrsd_dn10 = assign9670_e12236_d_n10;
        var_rrsd_dn11 = assign9670_e12236_d_n11;
        var_rrsd_dn13 = assign9670_e12236_d_n13;
        var_rrsd_dn14 = assign9670_e12236_d_n14;
        var_rrsd_rv = 0.0;

        let (assign9680_e12246, assign9680_e12246_d_n0, assign9680_e12246_d_n2, assign9680_e12246_d_n3, assign9680_e12246_d_n4, assign9680_e12246_d_n5, assign9680_e12246_d_n6, assign9680_e12246_d_n7, assign9680_e12246_d_n8, assign9680_e12246_d_n9, assign9680_e12246_d_n10, assign9680_e12246_d_n11, assign9680_e12246_d_n13, assign9680_e12246_d_n14,) = {
    if ((var_guard187 == 0.0) && (var_guard192 == 0.0)) {
        let assign9680_e12244: f64 = (var_rrsdtml + var_rsp);
        (assign9680_e12244, (var_rrsdtml_dn0 + var_rsp_dn0), (var_rrsdtml_dn2 + var_rsp_dn2), (var_rrsdtml_dn3 + var_rsp_dn3), (var_rrsdtml_dn4 + var_rsp_dn4), (var_rrsdtml_dn5 + var_rsp_dn5), (var_rrsdtml_dn6 + var_rsp_dn6), (var_rrsdtml_dn7 + var_rsp_dn7), (var_rrsdtml_dn8 + var_rsp_dn8), (var_rrsdtml_dn9 + var_rsp_dn9), (var_rrsdtml_dn10 + var_rsp_dn10), (var_rrsdtml_dn11 + var_rsp_dn11), (var_rrsdtml_dn13 + var_rsp_dn13), (var_rrsdtml_dn14 + var_rsp_dn14),)
    } else {
        (var_rrsd, var_rrsd_dn0, var_rrsd_dn2, var_rrsd_dn3, var_rrsd_dn4, var_rrsd_dn5, var_rrsd_dn6, var_rrsd_dn7, var_rrsd_dn8, var_rrsd_dn9, var_rrsd_dn10, var_rrsd_dn11, var_rrsd_dn13, var_rrsd_dn14,)
    }
};
        var_rrsd = assign9680_e12246;
        var_rrsd_dn0 = assign9680_e12246_d_n0;
        var_rrsd_dn2 = assign9680_e12246_d_n2;
        var_rrsd_dn3 = assign9680_e12246_d_n3;
        var_rrsd_dn4 = assign9680_e12246_d_n4;
        var_rrsd_dn5 = assign9680_e12246_d_n5;
        var_rrsd_dn6 = assign9680_e12246_d_n6;
        var_rrsd_dn7 = assign9680_e12246_d_n7;
        var_rrsd_dn8 = assign9680_e12246_d_n8;
        var_rrsd_dn9 = assign9680_e12246_d_n9;
        var_rrsd_dn10 = assign9680_e12246_d_n10;
        var_rrsd_dn11 = assign9680_e12246_d_n11;
        var_rrsd_dn13 = assign9680_e12246_d_n13;
        var_rrsd_dn14 = assign9680_e12246_d_n14;
        var_rrsd_rv = 0.0;

        let (assign9690_e12273, assign9690_e12273_d_n0, assign9690_e12273_d_n2, assign9690_e12273_d_n3, assign9690_e12273_d_n4, assign9690_e12273_d_n5, assign9690_e12273_d_n6, assign9690_e12273_d_n7, assign9690_e12273_d_n8, assign9690_e12273_d_n9, assign9690_e12273_d_n10, assign9690_e12273_d_n11, assign9690_e12273_d_n13, assign9690_e12273_d_n14,) = {
    if (var_guard187 == 0.0) {
        let assign9690_e12251: f64 = (var_rrsd / p.p59);
        let assign9690_e12256: f64 = (p.p1095 * p.p3);
        let assign9690_e12257: f64 = (p.p1094 + assign9690_e12256);
        let assign9690_e12260: f64 = (p.p1096 * p.p4);
        let assign9690_e12261: f64 = (assign9690_e12257 + assign9690_e12260);
        let assign9690_e12264: f64 = (p.p1097 * p.p20);
        let assign9690_e12265: f64 = (assign9690_e12261 + assign9690_e12264);
        let assign9690_e12268: f64 = (p.p1098 * p.p1080);
        let assign9690_e12269: f64 = (assign9690_e12265 + assign9690_e12268);
        let assign9690_e12270: f64 = (0.0_f64).max(assign9690_e12269);
        let assign9690_e12271: f64 = (assign9690_e12251 * assign9690_e12270);
        (assign9690_e12271, ((var_rrsd_dn0 / p.p59) * assign9690_e12270), ((var_rrsd_dn2 / p.p59) * assign9690_e12270), ((var_rrsd_dn3 / p.p59) * assign9690_e12270), ((var_rrsd_dn4 / p.p59) * assign9690_e12270), ((var_rrsd_dn5 / p.p59) * assign9690_e12270), ((var_rrsd_dn6 / p.p59) * assign9690_e12270), ((var_rrsd_dn7 / p.p59) * assign9690_e12270), ((var_rrsd_dn8 / p.p59) * assign9690_e12270), ((var_rrsd_dn9 / p.p59) * assign9690_e12270), ((var_rrsd_dn10 / p.p59) * assign9690_e12270), ((var_rrsd_dn11 / p.p59) * assign9690_e12270), ((var_rrsd_dn13 / p.p59) * assign9690_e12270), ((var_rrsd_dn14 / p.p59) * assign9690_e12270),)
    } else {
        (var_rdsgeo, var_rdsgeo_dn0, var_rdsgeo_dn2, var_rdsgeo_dn3, var_rdsgeo_dn4, var_rdsgeo_dn5, var_rdsgeo_dn6, var_rdsgeo_dn7, var_rdsgeo_dn8, var_rdsgeo_dn9, var_rdsgeo_dn10, var_rdsgeo_dn11, var_rdsgeo_dn13, var_rdsgeo_dn14,)
    }
};
        var_rdsgeo = assign9690_e12273;
        var_rdsgeo_dn0 = assign9690_e12273_d_n0;
        var_rdsgeo_dn2 = assign9690_e12273_d_n2;
        var_rdsgeo_dn3 = assign9690_e12273_d_n3;
        var_rdsgeo_dn4 = assign9690_e12273_d_n4;
        var_rdsgeo_dn5 = assign9690_e12273_d_n5;
        var_rdsgeo_dn6 = assign9690_e12273_d_n6;
        var_rdsgeo_dn7 = assign9690_e12273_d_n7;
        var_rdsgeo_dn8 = assign9690_e12273_d_n8;
        var_rdsgeo_dn9 = assign9690_e12273_d_n9;
        var_rdsgeo_dn10 = assign9690_e12273_d_n10;
        var_rdsgeo_dn11 = assign9690_e12273_d_n11;
        var_rdsgeo_dn13 = assign9690_e12273_d_n13;
        var_rdsgeo_dn14 = assign9690_e12273_d_n14;
        var_rdsgeo_rv = 0.0;

        let (assign9700_e12278, assign9700_e12278_d_n0, assign9700_e12278_d_n2, assign9700_e12278_d_n3, assign9700_e12278_d_n4, assign9700_e12278_d_n5, assign9700_e12278_d_n6, assign9700_e12278_d_n7, assign9700_e12278_d_n8, assign9700_e12278_d_n9, assign9700_e12278_d_n10, assign9700_e12278_d_n11, assign9700_e12278_d_n13, assign9700_e12278_d_n14,) = {
    if (var_guard187 == 0.0) {
        (var_rdsgeo, var_rdsgeo_dn0, var_rdsgeo_dn2, var_rdsgeo_dn3, var_rdsgeo_dn4, var_rdsgeo_dn5, var_rdsgeo_dn6, var_rdsgeo_dn7, var_rdsgeo_dn8, var_rdsgeo_dn9, var_rdsgeo_dn10, var_rdsgeo_dn11, var_rdsgeo_dn13, var_rdsgeo_dn14,)
    } else {
        (var_rsourcegeo, var_rsourcegeo_dn0, var_rsourcegeo_dn2, var_rsourcegeo_dn3, var_rsourcegeo_dn4, var_rsourcegeo_dn5, var_rsourcegeo_dn6, var_rsourcegeo_dn7, var_rsourcegeo_dn8, var_rsourcegeo_dn9, var_rsourcegeo_dn10, var_rsourcegeo_dn11, var_rsourcegeo_dn13, var_rsourcegeo_dn14,)
    }
};
        var_rsourcegeo = assign9700_e12278;
        var_rsourcegeo_dn0 = assign9700_e12278_d_n0;
        var_rsourcegeo_dn2 = assign9700_e12278_d_n2;
        var_rsourcegeo_dn3 = assign9700_e12278_d_n3;
        var_rsourcegeo_dn4 = assign9700_e12278_d_n4;
        var_rsourcegeo_dn5 = assign9700_e12278_d_n5;
        var_rsourcegeo_dn6 = assign9700_e12278_d_n6;
        var_rsourcegeo_dn7 = assign9700_e12278_d_n7;
        var_rsourcegeo_dn8 = assign9700_e12278_d_n8;
        var_rsourcegeo_dn9 = assign9700_e12278_d_n9;
        var_rsourcegeo_dn10 = assign9700_e12278_d_n10;
        var_rsourcegeo_dn11 = assign9700_e12278_d_n11;
        var_rsourcegeo_dn13 = assign9700_e12278_d_n13;
        var_rsourcegeo_dn14 = assign9700_e12278_d_n14;
        var_rsourcegeo_rv = 0.0;

        let (assign9710_e12283, assign9710_e12283_d_n0, assign9710_e12283_d_n2, assign9710_e12283_d_n3, assign9710_e12283_d_n4, assign9710_e12283_d_n5, assign9710_e12283_d_n6, assign9710_e12283_d_n7, assign9710_e12283_d_n8, assign9710_e12283_d_n9, assign9710_e12283_d_n10, assign9710_e12283_d_n11, assign9710_e12283_d_n13, assign9710_e12283_d_n14,) = {
    if (var_guard187 == 0.0) {
        (var_rdsgeo, var_rdsgeo_dn0, var_rdsgeo_dn2, var_rdsgeo_dn3, var_rdsgeo_dn4, var_rdsgeo_dn5, var_rdsgeo_dn6, var_rdsgeo_dn7, var_rdsgeo_dn8, var_rdsgeo_dn9, var_rdsgeo_dn10, var_rdsgeo_dn11, var_rdsgeo_dn13, var_rdsgeo_dn14,)
    } else {
        (var_rdraingeo, var_rdraingeo_dn0, var_rdraingeo_dn2, var_rdraingeo_dn3, var_rdraingeo_dn4, var_rdraingeo_dn5, var_rdraingeo_dn6, var_rdraingeo_dn7, var_rdraingeo_dn8, var_rdraingeo_dn9, var_rdraingeo_dn10, var_rdraingeo_dn11, var_rdraingeo_dn13, var_rdraingeo_dn14,)
    }
};
        var_rdraingeo = assign9710_e12283;
        var_rdraingeo_dn0 = assign9710_e12283_d_n0;
        var_rdraingeo_dn2 = assign9710_e12283_d_n2;
        var_rdraingeo_dn3 = assign9710_e12283_d_n3;
        var_rdraingeo_dn4 = assign9710_e12283_d_n4;
        var_rdraingeo_dn5 = assign9710_e12283_d_n5;
        var_rdraingeo_dn6 = assign9710_e12283_d_n6;
        var_rdraingeo_dn7 = assign9710_e12283_d_n7;
        var_rdraingeo_dn8 = assign9710_e12283_d_n8;
        var_rdraingeo_dn9 = assign9710_e12283_d_n9;
        var_rdraingeo_dn10 = assign9710_e12283_d_n10;
        var_rdraingeo_dn11 = assign9710_e12283_d_n11;
        var_rdraingeo_dn13 = assign9710_e12283_d_n13;
        var_rdraingeo_dn14 = assign9710_e12283_d_n14;
        var_rdraingeo_rv = 0.0;

        let assign9720_e12286: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        var_guard193 = assign9720_e12286;
        var_guard193_rv = 0.0;

        let assign9730_e12289: f64 = if var_rsourcegeo < p.p151 { 1.0 } else { 0.0 };
        var_guard194 = assign9730_e12289;
        var_guard194_rv = 0.0;

        let (assign9740_e12295, assign9740_e12295_d_n0, assign9740_e12295_d_n2, assign9740_e12295_d_n3, assign9740_e12295_d_n4, assign9740_e12295_d_n5, assign9740_e12295_d_n6, assign9740_e12295_d_n7, assign9740_e12295_d_n8, assign9740_e12295_d_n9, assign9740_e12295_d_n10, assign9740_e12295_d_n11, assign9740_e12295_d_n13, assign9740_e12295_d_n14,) = {
    if ((var_guard193 != 0.0) && (var_guard194 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsourcegeo, var_rsourcegeo_dn0, var_rsourcegeo_dn2, var_rsourcegeo_dn3, var_rsourcegeo_dn4, var_rsourcegeo_dn5, var_rsourcegeo_dn6, var_rsourcegeo_dn7, var_rsourcegeo_dn8, var_rsourcegeo_dn9, var_rsourcegeo_dn10, var_rsourcegeo_dn11, var_rsourcegeo_dn13, var_rsourcegeo_dn14,)
    }
};
        var_rsourcegeo = assign9740_e12295;
        var_rsourcegeo_dn0 = assign9740_e12295_d_n0;
        var_rsourcegeo_dn2 = assign9740_e12295_d_n2;
        var_rsourcegeo_dn3 = assign9740_e12295_d_n3;
        var_rsourcegeo_dn4 = assign9740_e12295_d_n4;
        var_rsourcegeo_dn5 = assign9740_e12295_d_n5;
        var_rsourcegeo_dn6 = assign9740_e12295_d_n6;
        var_rsourcegeo_dn7 = assign9740_e12295_d_n7;
        var_rsourcegeo_dn8 = assign9740_e12295_d_n8;
        var_rsourcegeo_dn9 = assign9740_e12295_d_n9;
        var_rsourcegeo_dn10 = assign9740_e12295_d_n10;
        var_rsourcegeo_dn11 = assign9740_e12295_d_n11;
        var_rsourcegeo_dn13 = assign9740_e12295_d_n13;
        var_rsourcegeo_dn14 = assign9740_e12295_d_n14;
        var_rsourcegeo_rv = 0.0;

        let assign9750_e12298: f64 = if var_rdraingeo < p.p151 { 1.0 } else { 0.0 };
        var_guard195 = assign9750_e12298;
        var_guard195_rv = 0.0;

        let (assign9760_e12304, assign9760_e12304_d_n0, assign9760_e12304_d_n2, assign9760_e12304_d_n3, assign9760_e12304_d_n4, assign9760_e12304_d_n5, assign9760_e12304_d_n6, assign9760_e12304_d_n7, assign9760_e12304_d_n8, assign9760_e12304_d_n9, assign9760_e12304_d_n10, assign9760_e12304_d_n11, assign9760_e12304_d_n13, assign9760_e12304_d_n14,) = {
    if ((var_guard193 != 0.0) && (var_guard195 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdraingeo, var_rdraingeo_dn0, var_rdraingeo_dn2, var_rdraingeo_dn3, var_rdraingeo_dn4, var_rdraingeo_dn5, var_rdraingeo_dn6, var_rdraingeo_dn7, var_rdraingeo_dn8, var_rdraingeo_dn9, var_rdraingeo_dn10, var_rdraingeo_dn11, var_rdraingeo_dn13, var_rdraingeo_dn14,)
    }
};
        var_rdraingeo = assign9760_e12304;
        var_rdraingeo_dn0 = assign9760_e12304_d_n0;
        var_rdraingeo_dn2 = assign9760_e12304_d_n2;
        var_rdraingeo_dn3 = assign9760_e12304_d_n3;
        var_rdraingeo_dn4 = assign9760_e12304_d_n4;
        var_rdraingeo_dn5 = assign9760_e12304_d_n5;
        var_rdraingeo_dn6 = assign9760_e12304_d_n6;
        var_rdraingeo_dn7 = assign9760_e12304_d_n7;
        var_rdraingeo_dn8 = assign9760_e12304_d_n8;
        var_rdraingeo_dn9 = assign9760_e12304_d_n9;
        var_rdraingeo_dn10 = assign9760_e12304_d_n10;
        var_rdraingeo_dn11 = assign9760_e12304_d_n11;
        var_rdraingeo_dn13 = assign9760_e12304_d_n13;
        var_rdraingeo_dn14 = assign9760_e12304_d_n14;
        var_rdraingeo_rv = 0.0;

        let assign9770_e12307: f64 = if var_rsourcegeo <= p.p151 { 1.0 } else { 0.0 };
        var_guard196 = assign9770_e12307;
        var_guard196_rv = 0.0;

        let (assign9780_e12314, assign9780_e12314_d_n0, assign9780_e12314_d_n2, assign9780_e12314_d_n3, assign9780_e12314_d_n4, assign9780_e12314_d_n5, assign9780_e12314_d_n6, assign9780_e12314_d_n7, assign9780_e12314_d_n8, assign9780_e12314_d_n9, assign9780_e12314_d_n10, assign9780_e12314_d_n11, assign9780_e12314_d_n13, assign9780_e12314_d_n14,) = {
    if ((var_guard193 == 0.0) && (var_guard196 != 0.0)) {
        (p.p151, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsourcegeo, var_rsourcegeo_dn0, var_rsourcegeo_dn2, var_rsourcegeo_dn3, var_rsourcegeo_dn4, var_rsourcegeo_dn5, var_rsourcegeo_dn6, var_rsourcegeo_dn7, var_rsourcegeo_dn8, var_rsourcegeo_dn9, var_rsourcegeo_dn10, var_rsourcegeo_dn11, var_rsourcegeo_dn13, var_rsourcegeo_dn14,)
    }
};
        var_rsourcegeo = assign9780_e12314;
        var_rsourcegeo_dn0 = assign9780_e12314_d_n0;
        var_rsourcegeo_dn2 = assign9780_e12314_d_n2;
        var_rsourcegeo_dn3 = assign9780_e12314_d_n3;
        var_rsourcegeo_dn4 = assign9780_e12314_d_n4;
        var_rsourcegeo_dn5 = assign9780_e12314_d_n5;
        var_rsourcegeo_dn6 = assign9780_e12314_d_n6;
        var_rsourcegeo_dn7 = assign9780_e12314_d_n7;
        var_rsourcegeo_dn8 = assign9780_e12314_d_n8;
        var_rsourcegeo_dn9 = assign9780_e12314_d_n9;
        var_rsourcegeo_dn10 = assign9780_e12314_d_n10;
        var_rsourcegeo_dn11 = assign9780_e12314_d_n11;
        var_rsourcegeo_dn13 = assign9780_e12314_d_n13;
        var_rsourcegeo_dn14 = assign9780_e12314_d_n14;
        var_rsourcegeo_rv = 0.0;

        let assign9790_e12317: f64 = if var_rdraingeo <= p.p151 { 1.0 } else { 0.0 };
        var_guard197 = assign9790_e12317;
        var_guard197_rv = 0.0;

        let (assign9800_e12324, assign9800_e12324_d_n0, assign9800_e12324_d_n2, assign9800_e12324_d_n3, assign9800_e12324_d_n4, assign9800_e12324_d_n5, assign9800_e12324_d_n6, assign9800_e12324_d_n7, assign9800_e12324_d_n8, assign9800_e12324_d_n9, assign9800_e12324_d_n10, assign9800_e12324_d_n11, assign9800_e12324_d_n13, assign9800_e12324_d_n14,) = {
    if ((var_guard193 == 0.0) && (var_guard197 != 0.0)) {
        (p.p151, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdraingeo, var_rdraingeo_dn0, var_rdraingeo_dn2, var_rdraingeo_dn3, var_rdraingeo_dn4, var_rdraingeo_dn5, var_rdraingeo_dn6, var_rdraingeo_dn7, var_rdraingeo_dn8, var_rdraingeo_dn9, var_rdraingeo_dn10, var_rdraingeo_dn11, var_rdraingeo_dn13, var_rdraingeo_dn14,)
    }
};
        var_rdraingeo = assign9800_e12324;
        var_rdraingeo_dn0 = assign9800_e12324_d_n0;
        var_rdraingeo_dn2 = assign9800_e12324_d_n2;
        var_rdraingeo_dn3 = assign9800_e12324_d_n3;
        var_rdraingeo_dn4 = assign9800_e12324_d_n4;
        var_rdraingeo_dn5 = assign9800_e12324_d_n5;
        var_rdraingeo_dn6 = assign9800_e12324_d_n6;
        var_rdraingeo_dn7 = assign9800_e12324_d_n7;
        var_rdraingeo_dn8 = assign9800_e12324_d_n8;
        var_rdraingeo_dn9 = assign9800_e12324_d_n9;
        var_rdraingeo_dn10 = assign9800_e12324_d_n10;
        var_rdraingeo_dn11 = assign9800_e12324_d_n11;
        var_rdraingeo_dn13 = assign9800_e12324_d_n13;
        var_rdraingeo_dn14 = assign9800_e12324_d_n14;
        var_rdraingeo_rv = 0.0;

        let assign9810_e12327: f64 = if p.p78 != 1.0 { 1.0 } else { 0.0 };
        var_guard198 = assign9810_e12327;
        var_guard198_rv = 0.0;

        let assign9820_e12329: f64 = if param_given[1542] { 1.0 } else { 0.0 };
        var_guard199 = assign9820_e12329;
        var_guard199_rv = 0.0;

        let (assign9830_e12335,) = {
    if ((var_guard198 != 0.0) && (var_guard199 != 0.0)) {
        (p.p1542,)
    } else {
        (var_cgso_i,)
    }
};
        var_cgso_i = assign9830_e12335;
        var_cgso_i_rv = 0.0;

        let assign9840_e12341: f64 = if (param_given[85] && (p.p85 > 0.0)) { 1.0 } else { 0.0 };
        var_guard200 = assign9840_e12341;
        var_guard200_rv = 0.0;

        let (assign9850_e12356,) = {
    if (((var_guard198 != 0.0) && (var_guard199 == 0.0)) && (var_guard200 != 0.0)) {
        let assign9850_e12351: f64 = (p.p85 * var_cox);
        let assign9850_e12353: f64 = (assign9850_e12351 - var_cgsl_i);
        let assign9850_e12354: f64 = (0.0_f64).max(assign9850_e12353);
        (assign9850_e12354,)
    } else {
        (var_cgso_i,)
    }
};
        var_cgso_i = assign9850_e12356;
        var_cgso_i_rv = 0.0;

        let assign9860_e12359: f64 = if p.p78 == 3.0 { 1.0 } else { 0.0 };
        var_guard201 = assign9860_e12359;
        var_guard201_rv = 0.0;

        let (assign9870_e12375,) = {
    if ((((var_guard198 != 0.0) && (var_guard199 == 0.0)) && (var_guard200 == 0.0)) && (var_guard201 != 0.0)) {
        let assign9870_e12371: f64 = (0.3 * p.p43);
        let assign9870_e12373: f64 = (assign9870_e12371 * var_cox);
        (assign9870_e12373,)
    } else {
        (var_cgso_i,)
    }
};
        var_cgso_i = assign9870_e12375;
        var_cgso_i_rv = 0.0;

        let (assign9880_e12392,) = {
    if ((((var_guard198 != 0.0) && (var_guard199 == 0.0)) && (var_guard200 == 0.0)) && (var_guard201 == 0.0)) {
        let assign9880_e12388: f64 = (0.3 * p.p3);
        let assign9880_e12390: f64 = (assign9880_e12388 * var_cox);
        (assign9880_e12390,)
    } else {
        (var_cgso_i,)
    }
};
        var_cgso_i = assign9880_e12392;
        var_cgso_i_rv = 0.0;

        let assign9890_e12394: f64 = if param_given[1543] { 1.0 } else { 0.0 };
        var_guard202 = assign9890_e12394;
        var_guard202_rv = 0.0;

        let (assign9900_e12400,) = {
    if ((var_guard198 != 0.0) && (var_guard202 != 0.0)) {
        (p.p1543,)
    } else {
        (var_cgdo_i,)
    }
};
        var_cgdo_i = assign9900_e12400;
        var_cgdo_i_rv = 0.0;

        let assign9910_e12406: f64 = if (param_given[85] && (p.p85 > 0.0)) { 1.0 } else { 0.0 };
        var_guard203 = assign9910_e12406;
        var_guard203_rv = 0.0;

        let (assign9920_e12421,) = {
    if (((var_guard198 != 0.0) && (var_guard202 == 0.0)) && (var_guard203 != 0.0)) {
        let assign9920_e12416: f64 = (p.p85 * var_cox);
        let assign9920_e12418: f64 = (assign9920_e12416 - var_cgdl_i);
        let assign9920_e12419: f64 = (0.0_f64).max(assign9920_e12418);
        (assign9920_e12419,)
    } else {
        (var_cgdo_i,)
    }
};
        var_cgdo_i = assign9920_e12421;
        var_cgdo_i_rv = 0.0;

        let assign9930_e12424: f64 = if p.p78 == 3.0 { 1.0 } else { 0.0 };
        var_guard204 = assign9930_e12424;
        var_guard204_rv = 0.0;

        let (assign9940_e12440,) = {
    if ((((var_guard198 != 0.0) && (var_guard202 == 0.0)) && (var_guard203 == 0.0)) && (var_guard204 != 0.0)) {
        let assign9940_e12436: f64 = (0.3 * p.p43);
        let assign9940_e12438: f64 = (assign9940_e12436 * var_cox);
        (assign9940_e12438,)
    } else {
        (var_cgdo_i,)
    }
};
        var_cgdo_i = assign9940_e12440;
        var_cgdo_i_rv = 0.0;

        let (assign9950_e12457,) = {
    if ((((var_guard198 != 0.0) && (var_guard202 == 0.0)) && (var_guard203 == 0.0)) && (var_guard204 == 0.0)) {
        let assign9950_e12453: f64 = (0.3 * p.p3);
        let assign9950_e12455: f64 = (assign9950_e12453 * var_cox);
        (assign9950_e12455,)
    } else {
        (var_cgdo_i,)
    }
};
        var_cgdo_i = assign9950_e12457;
        var_cgdo_i_rv = 0.0;

        let assign9960_e12460: f64 = if p.p78 == 2.0 { 1.0 } else { 0.0 };
        var_guard205 = assign9960_e12460;
        var_guard205_rv = 0.0;

        let (assign9970_e12466,) = {
    if (var_guard205 != 0.0) {
        let assign9970_e12464: f64 = (p.p1089 + p.p1090);
        (assign9970_e12464,)
    } else {
        (var_hg,)
    }
};
        var_hg = assign9970_e12466;
        var_hg_rv = 0.0;

        let (assign9980_e12474,) = {
    if (var_guard205 != 0.0) {
        let assign9980_e12471: f64 = (p.p4 - p.p3);
        let assign9980_e12472: f64 = (0.5 * assign9980_e12471);
        (assign9980_e12472,)
    } else {
        (var_trsd,)
    }
};
        var_trsd = assign9980_e12474;
        var_trsd_rv = 0.0;

        let (assign9990_e12482,) = {
    if (var_guard205 != 0.0) {
        let assign9990_e12479: f64 = (var_trsd - p.p90);
        let assign9990_e12480: f64 = (0.0_f64).max(assign9990_e12479);
        (assign9990_e12480,)
    } else {
        (var_wg,)
    }
};
        var_wg = assign9990_e12482;
        var_wg_rv = 0.0;

        let (assign10000_e12490,) = {
    if (var_guard205 != 0.0) {
        let assign10000_e12487: f64 = (p.p1080 + p.p1081);
        let assign10000_e12488: f64 = (0.0_f64).max(assign10000_e12487);
        (assign10000_e12488,)
    } else {
        (var_hrsd,)
    }
};
        var_hrsd = assign10000_e12490;
        var_hrsd_rv = 0.0;

        let assign10010_e12493: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        var_guard206 = assign10010_e12493;
        var_guard206_rv = 0.0;

        *var_cgdo_i_slot = var_cgdo_i;
        *var_cgdo_i_rv_slot = var_cgdo_i_rv;
        *var_cgso_i_slot = var_cgso_i;
        *var_cgso_i_rv_slot = var_cgso_i_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard193_slot = var_guard193;
        *var_guard193_rv_slot = var_guard193_rv;
        *var_guard194_slot = var_guard194;
        *var_guard194_rv_slot = var_guard194_rv;
        *var_guard195_slot = var_guard195;
        *var_guard195_rv_slot = var_guard195_rv;
        *var_guard196_slot = var_guard196;
        *var_guard196_rv_slot = var_guard196_rv;
        *var_guard197_slot = var_guard197;
        *var_guard197_rv_slot = var_guard197_rv;
        *var_guard198_slot = var_guard198;
        *var_guard198_rv_slot = var_guard198_rv;
        *var_guard199_slot = var_guard199;
        *var_guard199_rv_slot = var_guard199_rv;
        *var_guard200_slot = var_guard200;
        *var_guard200_rv_slot = var_guard200_rv;
        *var_guard201_slot = var_guard201;
        *var_guard201_rv_slot = var_guard201_rv;
        *var_guard202_slot = var_guard202;
        *var_guard202_rv_slot = var_guard202_rv;
        *var_guard203_slot = var_guard203;
        *var_guard203_rv_slot = var_guard203_rv;
        *var_guard204_slot = var_guard204;
        *var_guard204_rv_slot = var_guard204_rv;
        *var_guard205_slot = var_guard205;
        *var_guard205_rv_slot = var_guard205_rv;
        *var_guard206_slot = var_guard206;
        *var_guard206_rv_slot = var_guard206_rv;
        *var_hg_slot = var_hg;
        *var_hg_rv_slot = var_hg_rv;
        *var_hrsd_slot = var_hrsd;
        *var_hrsd_rv_slot = var_hrsd_rv;
        *var_rdraingeo_slot = var_rdraingeo;
        *var_rdraingeo_dn0_slot = var_rdraingeo_dn0;
        *var_rdraingeo_dn10_slot = var_rdraingeo_dn10;
        *var_rdraingeo_dn11_slot = var_rdraingeo_dn11;
        *var_rdraingeo_dn13_slot = var_rdraingeo_dn13;
        *var_rdraingeo_dn14_slot = var_rdraingeo_dn14;
        *var_rdraingeo_dn2_slot = var_rdraingeo_dn2;
        *var_rdraingeo_dn3_slot = var_rdraingeo_dn3;
        *var_rdraingeo_dn4_slot = var_rdraingeo_dn4;
        *var_rdraingeo_dn5_slot = var_rdraingeo_dn5;
        *var_rdraingeo_dn6_slot = var_rdraingeo_dn6;
        *var_rdraingeo_dn7_slot = var_rdraingeo_dn7;
        *var_rdraingeo_dn8_slot = var_rdraingeo_dn8;
        *var_rdraingeo_dn9_slot = var_rdraingeo_dn9;
        *var_rdraingeo_rv_slot = var_rdraingeo_rv;
        *var_rdsgeo_slot = var_rdsgeo;
        *var_rdsgeo_dn0_slot = var_rdsgeo_dn0;
        *var_rdsgeo_dn10_slot = var_rdsgeo_dn10;
        *var_rdsgeo_dn11_slot = var_rdsgeo_dn11;
        *var_rdsgeo_dn13_slot = var_rdsgeo_dn13;
        *var_rdsgeo_dn14_slot = var_rdsgeo_dn14;
        *var_rdsgeo_dn2_slot = var_rdsgeo_dn2;
        *var_rdsgeo_dn3_slot = var_rdsgeo_dn3;
        *var_rdsgeo_dn4_slot = var_rdsgeo_dn4;
        *var_rdsgeo_dn5_slot = var_rdsgeo_dn5;
        *var_rdsgeo_dn6_slot = var_rdsgeo_dn6;
        *var_rdsgeo_dn7_slot = var_rdsgeo_dn7;
        *var_rdsgeo_dn8_slot = var_rdsgeo_dn8;
        *var_rdsgeo_dn9_slot = var_rdsgeo_dn9;
        *var_rdsgeo_rv_slot = var_rdsgeo_rv;
        *var_rrsd_slot = var_rrsd;
        *var_rrsd_dn0_slot = var_rrsd_dn0;
        *var_rrsd_dn10_slot = var_rrsd_dn10;
        *var_rrsd_dn11_slot = var_rrsd_dn11;
        *var_rrsd_dn13_slot = var_rrsd_dn13;
        *var_rrsd_dn14_slot = var_rrsd_dn14;
        *var_rrsd_dn2_slot = var_rrsd_dn2;
        *var_rrsd_dn3_slot = var_rrsd_dn3;
        *var_rrsd_dn4_slot = var_rrsd_dn4;
        *var_rrsd_dn5_slot = var_rrsd_dn5;
        *var_rrsd_dn6_slot = var_rrsd_dn6;
        *var_rrsd_dn7_slot = var_rrsd_dn7;
        *var_rrsd_dn8_slot = var_rrsd_dn8;
        *var_rrsd_dn9_slot = var_rrsd_dn9;
        *var_rrsd_rv_slot = var_rrsd_rv;
        *var_rrsdside_slot = var_rrsdside;
        *var_rrsdside_rv_slot = var_rrsdside_rv;
        *var_rrsdtml_slot = var_rrsdtml;
        *var_rrsdtml_dn0_slot = var_rrsdtml_dn0;
        *var_rrsdtml_dn10_slot = var_rrsdtml_dn10;
        *var_rrsdtml_dn11_slot = var_rrsdtml_dn11;
        *var_rrsdtml_dn13_slot = var_rrsdtml_dn13;
        *var_rrsdtml_dn14_slot = var_rrsdtml_dn14;
        *var_rrsdtml_dn2_slot = var_rrsdtml_dn2;
        *var_rrsdtml_dn3_slot = var_rrsdtml_dn3;
        *var_rrsdtml_dn4_slot = var_rrsdtml_dn4;
        *var_rrsdtml_dn5_slot = var_rrsdtml_dn5;
        *var_rrsdtml_dn6_slot = var_rrsdtml_dn6;
        *var_rrsdtml_dn7_slot = var_rrsdtml_dn7;
        *var_rrsdtml_dn8_slot = var_rrsdtml_dn8;
        *var_rrsdtml_dn9_slot = var_rrsdtml_dn9;
        *var_rrsdtml_rv_slot = var_rrsdtml_rv;
        *var_rsourcegeo_slot = var_rsourcegeo;
        *var_rsourcegeo_dn0_slot = var_rsourcegeo_dn0;
        *var_rsourcegeo_dn10_slot = var_rsourcegeo_dn10;
        *var_rsourcegeo_dn11_slot = var_rsourcegeo_dn11;
        *var_rsourcegeo_dn13_slot = var_rsourcegeo_dn13;
        *var_rsourcegeo_dn14_slot = var_rsourcegeo_dn14;
        *var_rsourcegeo_dn2_slot = var_rsourcegeo_dn2;
        *var_rsourcegeo_dn3_slot = var_rsourcegeo_dn3;
        *var_rsourcegeo_dn4_slot = var_rsourcegeo_dn4;
        *var_rsourcegeo_dn5_slot = var_rsourcegeo_dn5;
        *var_rsourcegeo_dn6_slot = var_rsourcegeo_dn6;
        *var_rsourcegeo_dn7_slot = var_rsourcegeo_dn7;
        *var_rsourcegeo_dn8_slot = var_rsourcegeo_dn8;
        *var_rsourcegeo_dn9_slot = var_rsourcegeo_dn9;
        *var_rsourcegeo_rv_slot = var_rsourcegeo_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_trsd_slot = var_trsd;
        *var_trsd_rv_slot = var_trsd_rv;
        *var_wg_slot = var_wg;
        *var_wg_rv_slot = var_wg_rv;
    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        var_epssp: f64,
        var_guard205: f64,
        var_guard206: f64,
        var_hg: f64,
        var_hrsd: f64,
        var_c1_slot: &mut f64,
        var_c1_rv_slot: &mut f64,
        var_c2_slot: &mut f64,
        var_c2_rv_slot: &mut f64,
        var_c3_slot: &mut f64,
        var_c3_rv_slot: &mut f64,
        var_ccg_slot: &mut f64,
        var_ccg1_slot: &mut f64,
        var_ccg1_dn0_slot: &mut f64,
        var_ccg1_dn10_slot: &mut f64,
        var_ccg1_dn11_slot: &mut f64,
        var_ccg1_dn13_slot: &mut f64,
        var_ccg1_dn14_slot: &mut f64,
        var_ccg1_dn2_slot: &mut f64,
        var_ccg1_dn3_slot: &mut f64,
        var_ccg1_dn4_slot: &mut f64,
        var_ccg1_dn5_slot: &mut f64,
        var_ccg1_dn6_slot: &mut f64,
        var_ccg1_dn7_slot: &mut f64,
        var_ccg1_dn8_slot: &mut f64,
        var_ccg1_dn9_slot: &mut f64,
        var_ccg1_rv_slot: &mut f64,
        var_ccg2_slot: &mut f64,
        var_ccg2_rv_slot: &mut f64,
        var_ccg_dn0_slot: &mut f64,
        var_ccg_dn10_slot: &mut f64,
        var_ccg_dn11_slot: &mut f64,
        var_ccg_dn13_slot: &mut f64,
        var_ccg_dn14_slot: &mut f64,
        var_ccg_dn2_slot: &mut f64,
        var_ccg_dn3_slot: &mut f64,
        var_ccg_dn4_slot: &mut f64,
        var_ccg_dn5_slot: &mut f64,
        var_ccg_dn6_slot: &mut f64,
        var_ccg_dn7_slot: &mut f64,
        var_ccg_dn8_slot: &mut f64,
        var_ccg_dn9_slot: &mut f64,
        var_ccg_rv_slot: &mut f64,
        var_ccgsat_slot: &mut f64,
        var_ccgsat_rv_slot: &mut f64,
        var_cfglog_slot: &mut f64,
        var_cfglog_rv_slot: &mut f64,
        var_cgg_top_slot: &mut f64,
        var_cgg_top_dn0_slot: &mut f64,
        var_cgg_top_dn10_slot: &mut f64,
        var_cgg_top_dn11_slot: &mut f64,
        var_cgg_top_dn13_slot: &mut f64,
        var_cgg_top_dn14_slot: &mut f64,
        var_cgg_top_dn2_slot: &mut f64,
        var_cgg_top_dn3_slot: &mut f64,
        var_cgg_top_dn4_slot: &mut f64,
        var_cgg_top_dn5_slot: &mut f64,
        var_cgg_top_dn6_slot: &mut f64,
        var_cgg_top_dn7_slot: &mut f64,
        var_cgg_top_dn8_slot: &mut f64,
        var_cgg_top_dn9_slot: &mut f64,
        var_cgg_top_rv_slot: &mut f64,
        var_cnon_slot: &mut f64,
        var_cnon_rv_slot: &mut f64,
        var_dcf_slot: &mut f64,
        var_dcf_rv_slot: &mut f64,
        var_guard207_slot: &mut f64,
        var_guard207_rv_slot: &mut f64,
        var_hgdelta_slot: &mut f64,
        var_hgdelta_rv_slot: &mut f64,
        var_hr_slot: &mut f64,
        var_hr_rv_slot: &mut f64,
        var_lmax_slot: &mut f64,
        var_lmax_rv_slot: &mut f64,
        var_lr_slot: &mut f64,
        var_lr_rv_slot: &mut f64,
        var_r1cf_slot: &mut f64,
        var_r1cf_rv_slot: &mut f64,
        var_rcf_slot: &mut f64,
        var_rcf_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_tt1_slot: &mut f64,
        var_tt1_dn0_slot: &mut f64,
        var_tt1_dn10_slot: &mut f64,
        var_tt1_dn11_slot: &mut f64,
        var_tt1_dn13_slot: &mut f64,
        var_tt1_dn14_slot: &mut f64,
        var_tt1_dn2_slot: &mut f64,
        var_tt1_dn3_slot: &mut f64,
        var_tt1_dn4_slot: &mut f64,
        var_tt1_dn5_slot: &mut f64,
        var_tt1_dn6_slot: &mut f64,
        var_tt1_dn7_slot: &mut f64,
        var_tt1_dn8_slot: &mut f64,
        var_tt1_dn9_slot: &mut f64,
        var_tt1_rv_slot: &mut f64,
        var_x_slot: &mut f64,
        var_x_rv_slot: &mut f64,
        var_y_slot: &mut f64,
        var_y_rv_slot: &mut f64,
    ) {
        let mut var_c1: f64 = *var_c1_slot;
        let mut var_c1_rv: f64 = *var_c1_rv_slot;
        let mut var_c2: f64 = *var_c2_slot;
        let mut var_c2_rv: f64 = *var_c2_rv_slot;
        let mut var_c3: f64 = *var_c3_slot;
        let mut var_c3_rv: f64 = *var_c3_rv_slot;
        let mut var_ccg: f64 = *var_ccg_slot;
        let mut var_ccg1: f64 = *var_ccg1_slot;
        let mut var_ccg1_dn0: f64 = *var_ccg1_dn0_slot;
        let mut var_ccg1_dn10: f64 = *var_ccg1_dn10_slot;
        let mut var_ccg1_dn11: f64 = *var_ccg1_dn11_slot;
        let mut var_ccg1_dn13: f64 = *var_ccg1_dn13_slot;
        let mut var_ccg1_dn14: f64 = *var_ccg1_dn14_slot;
        let mut var_ccg1_dn2: f64 = *var_ccg1_dn2_slot;
        let mut var_ccg1_dn3: f64 = *var_ccg1_dn3_slot;
        let mut var_ccg1_dn4: f64 = *var_ccg1_dn4_slot;
        let mut var_ccg1_dn5: f64 = *var_ccg1_dn5_slot;
        let mut var_ccg1_dn6: f64 = *var_ccg1_dn6_slot;
        let mut var_ccg1_dn7: f64 = *var_ccg1_dn7_slot;
        let mut var_ccg1_dn8: f64 = *var_ccg1_dn8_slot;
        let mut var_ccg1_dn9: f64 = *var_ccg1_dn9_slot;
        let mut var_ccg1_rv: f64 = *var_ccg1_rv_slot;
        let mut var_ccg2: f64 = *var_ccg2_slot;
        let mut var_ccg2_rv: f64 = *var_ccg2_rv_slot;
        let mut var_ccg_dn0: f64 = *var_ccg_dn0_slot;
        let mut var_ccg_dn10: f64 = *var_ccg_dn10_slot;
        let mut var_ccg_dn11: f64 = *var_ccg_dn11_slot;
        let mut var_ccg_dn13: f64 = *var_ccg_dn13_slot;
        let mut var_ccg_dn14: f64 = *var_ccg_dn14_slot;
        let mut var_ccg_dn2: f64 = *var_ccg_dn2_slot;
        let mut var_ccg_dn3: f64 = *var_ccg_dn3_slot;
        let mut var_ccg_dn4: f64 = *var_ccg_dn4_slot;
        let mut var_ccg_dn5: f64 = *var_ccg_dn5_slot;
        let mut var_ccg_dn6: f64 = *var_ccg_dn6_slot;
        let mut var_ccg_dn7: f64 = *var_ccg_dn7_slot;
        let mut var_ccg_dn8: f64 = *var_ccg_dn8_slot;
        let mut var_ccg_dn9: f64 = *var_ccg_dn9_slot;
        let mut var_ccg_rv: f64 = *var_ccg_rv_slot;
        let mut var_ccgsat: f64 = *var_ccgsat_slot;
        let mut var_ccgsat_rv: f64 = *var_ccgsat_rv_slot;
        let mut var_cfglog: f64 = *var_cfglog_slot;
        let mut var_cfglog_rv: f64 = *var_cfglog_rv_slot;
        let mut var_cgg_top: f64 = *var_cgg_top_slot;
        let mut var_cgg_top_dn0: f64 = *var_cgg_top_dn0_slot;
        let mut var_cgg_top_dn10: f64 = *var_cgg_top_dn10_slot;
        let mut var_cgg_top_dn11: f64 = *var_cgg_top_dn11_slot;
        let mut var_cgg_top_dn13: f64 = *var_cgg_top_dn13_slot;
        let mut var_cgg_top_dn14: f64 = *var_cgg_top_dn14_slot;
        let mut var_cgg_top_dn2: f64 = *var_cgg_top_dn2_slot;
        let mut var_cgg_top_dn3: f64 = *var_cgg_top_dn3_slot;
        let mut var_cgg_top_dn4: f64 = *var_cgg_top_dn4_slot;
        let mut var_cgg_top_dn5: f64 = *var_cgg_top_dn5_slot;
        let mut var_cgg_top_dn6: f64 = *var_cgg_top_dn6_slot;
        let mut var_cgg_top_dn7: f64 = *var_cgg_top_dn7_slot;
        let mut var_cgg_top_dn8: f64 = *var_cgg_top_dn8_slot;
        let mut var_cgg_top_dn9: f64 = *var_cgg_top_dn9_slot;
        let mut var_cgg_top_rv: f64 = *var_cgg_top_rv_slot;
        let mut var_cnon: f64 = *var_cnon_slot;
        let mut var_cnon_rv: f64 = *var_cnon_rv_slot;
        let mut var_dcf: f64 = *var_dcf_slot;
        let mut var_dcf_rv: f64 = *var_dcf_rv_slot;
        let mut var_guard207: f64 = *var_guard207_slot;
        let mut var_guard207_rv: f64 = *var_guard207_rv_slot;
        let mut var_hgdelta: f64 = *var_hgdelta_slot;
        let mut var_hgdelta_rv: f64 = *var_hgdelta_rv_slot;
        let mut var_hr: f64 = *var_hr_slot;
        let mut var_hr_rv: f64 = *var_hr_rv_slot;
        let mut var_lmax: f64 = *var_lmax_slot;
        let mut var_lmax_rv: f64 = *var_lmax_rv_slot;
        let mut var_lr: f64 = *var_lr_slot;
        let mut var_lr_rv: f64 = *var_lr_rv_slot;
        let mut var_r1cf: f64 = *var_r1cf_slot;
        let mut var_r1cf_rv: f64 = *var_r1cf_rv_slot;
        let mut var_rcf: f64 = *var_rcf_slot;
        let mut var_rcf_rv: f64 = *var_rcf_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_tt1: f64 = *var_tt1_slot;
        let mut var_tt1_dn0: f64 = *var_tt1_dn0_slot;
        let mut var_tt1_dn10: f64 = *var_tt1_dn10_slot;
        let mut var_tt1_dn11: f64 = *var_tt1_dn11_slot;
        let mut var_tt1_dn13: f64 = *var_tt1_dn13_slot;
        let mut var_tt1_dn14: f64 = *var_tt1_dn14_slot;
        let mut var_tt1_dn2: f64 = *var_tt1_dn2_slot;
        let mut var_tt1_dn3: f64 = *var_tt1_dn3_slot;
        let mut var_tt1_dn4: f64 = *var_tt1_dn4_slot;
        let mut var_tt1_dn5: f64 = *var_tt1_dn5_slot;
        let mut var_tt1_dn6: f64 = *var_tt1_dn6_slot;
        let mut var_tt1_dn7: f64 = *var_tt1_dn7_slot;
        let mut var_tt1_dn8: f64 = *var_tt1_dn8_slot;
        let mut var_tt1_dn9: f64 = *var_tt1_dn9_slot;
        let mut var_tt1_rv: f64 = *var_tt1_rv_slot;
        let mut var_x: f64 = *var_x_slot;
        let mut var_x_rv: f64 = *var_x_rv_slot;
        let mut var_y: f64 = *var_y_slot;
        let mut var_y_rv: f64 = *var_y_rv_slot;

        let (assign10020_e12532, assign10020_e12532_d_n0, assign10020_e12532_d_n2, assign10020_e12532_d_n3, assign10020_e12532_d_n4, assign10020_e12532_d_n5, assign10020_e12532_d_n6, assign10020_e12532_d_n7, assign10020_e12532_d_n8, assign10020_e12532_d_n9, assign10020_e12532_d_n10, assign10020_e12532_d_n11, assign10020_e12532_d_n13, assign10020_e12532_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 != 0.0)) {
        let assign10020_e12500: f64 = (1e-7 * p.p1088);
        let assign10020_e12503: f64 = (3.9 * p.p1087);
        let assign10020_e12504: f64 = (assign10020_e12500 / assign10020_e12503);
        let (assign10020_e12529,) = {
            if (!(assign10020_e12504 > 1e-38)) {
                let assign10020_e12509: f64 = (-87.498233534);
                (assign10020_e12509,)
            } else {
                let assign10020_e12512: f64 = (1e-7 * p.p1088);
                let assign10020_e12515: f64 = (3.9 * p.p1087);
                let assign10020_e12516: f64 = (assign10020_e12512 / assign10020_e12515);
                let (assign10020_e12528,) = {
                    if (assign10020_e12516 > 1e-38) {
                        let assign10020_e12521: f64 = (1e-7 * p.p1088);
                        let assign10020_e12524: f64 = (3.9 * p.p1087);
                        let assign10020_e12525: f64 = (assign10020_e12521 / assign10020_e12524);
                        let assign10020_e12526: f64 = (assign10020_e12525).ln();
                        (assign10020_e12526,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10020_e12528,)
            }
        };
        let assign10020_e12530: f64 = (3.467e-11 * assign10020_e12529);
        (assign10020_e12530, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign10020_e12532;
        var_t0_dn0 = assign10020_e12532_d_n0;
        var_t0_dn2 = assign10020_e12532_d_n2;
        var_t0_dn3 = assign10020_e12532_d_n3;
        var_t0_dn4 = assign10020_e12532_d_n4;
        var_t0_dn5 = assign10020_e12532_d_n5;
        var_t0_dn6 = assign10020_e12532_d_n6;
        var_t0_dn7 = assign10020_e12532_d_n7;
        var_t0_dn8 = assign10020_e12532_d_n8;
        var_t0_dn9 = assign10020_e12532_d_n9;
        var_t0_dn10 = assign10020_e12532_d_n10;
        var_t0_dn11 = assign10020_e12532_d_n11;
        var_t0_dn13 = assign10020_e12532_d_n13;
        var_t0_dn14 = assign10020_e12532_d_n14;
        var_t0_rv = 0.0;

        let (assign10030_e12544, assign10030_e12544_d_n0, assign10030_e12544_d_n2, assign10030_e12544_d_n3, assign10030_e12544_d_n4, assign10030_e12544_d_n5, assign10030_e12544_d_n6, assign10030_e12544_d_n7, assign10030_e12544_d_n8, assign10030_e12544_d_n9, assign10030_e12544_d_n10, assign10030_e12544_d_n11, assign10030_e12544_d_n13, assign10030_e12544_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 != 0.0)) {
        let assign10030_e12538: f64 = (0.942 * var_hrsd);
        let assign10030_e12540: f64 = (assign10030_e12538 * var_epssp);
        let assign10030_e12542: f64 = (assign10030_e12540 / p.p1087);
        (assign10030_e12542, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign10030_e12544;
        var_t1_dn0 = assign10030_e12544_d_n0;
        var_t1_dn2 = assign10030_e12544_d_n2;
        var_t1_dn3 = assign10030_e12544_d_n3;
        var_t1_dn4 = assign10030_e12544_d_n4;
        var_t1_dn5 = assign10030_e12544_d_n5;
        var_t1_dn6 = assign10030_e12544_d_n6;
        var_t1_dn7 = assign10030_e12544_d_n7;
        var_t1_dn8 = assign10030_e12544_d_n8;
        var_t1_dn9 = assign10030_e12544_d_n9;
        var_t1_dn10 = assign10030_e12544_d_n10;
        var_t1_dn11 = assign10030_e12544_d_n11;
        var_t1_dn13 = assign10030_e12544_d_n13;
        var_t1_dn14 = assign10030_e12544_d_n14;
        var_t1_rv = 0.0;

        let (assign10040_e12560, assign10040_e12560_d_n0, assign10040_e12560_d_n2, assign10040_e12560_d_n3, assign10040_e12560_d_n4, assign10040_e12560_d_n5, assign10040_e12560_d_n6, assign10040_e12560_d_n7, assign10040_e12560_d_n8, assign10040_e12560_d_n9, assign10040_e12560_d_n10, assign10040_e12560_d_n11, assign10040_e12560_d_n13, assign10040_e12560_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 != 0.0)) {
        let assign10040_e12550: f64 = (var_t0 + var_t1);
        let assign10040_e12554: f64 = (p.p4 - p.p3);
        let assign10040_e12556: f64 = (assign10040_e12554 * p.p1084);
        let assign10040_e12557: f64 = (p.p3 + assign10040_e12556);
        let assign10040_e12558: f64 = (assign10040_e12550 * assign10040_e12557);
        (assign10040_e12558, ((var_t0_dn0 + var_t1_dn0) * assign10040_e12557), ((var_t0_dn2 + var_t1_dn2) * assign10040_e12557), ((var_t0_dn3 + var_t1_dn3) * assign10040_e12557), ((var_t0_dn4 + var_t1_dn4) * assign10040_e12557), ((var_t0_dn5 + var_t1_dn5) * assign10040_e12557), ((var_t0_dn6 + var_t1_dn6) * assign10040_e12557), ((var_t0_dn7 + var_t1_dn7) * assign10040_e12557), ((var_t0_dn8 + var_t1_dn8) * assign10040_e12557), ((var_t0_dn9 + var_t1_dn9) * assign10040_e12557), ((var_t0_dn10 + var_t1_dn10) * assign10040_e12557), ((var_t0_dn11 + var_t1_dn11) * assign10040_e12557), ((var_t0_dn13 + var_t1_dn13) * assign10040_e12557), ((var_t0_dn14 + var_t1_dn14) * assign10040_e12557),)
    } else {
        (var_cgg_top, var_cgg_top_dn0, var_cgg_top_dn2, var_cgg_top_dn3, var_cgg_top_dn4, var_cgg_top_dn5, var_cgg_top_dn6, var_cgg_top_dn7, var_cgg_top_dn8, var_cgg_top_dn9, var_cgg_top_dn10, var_cgg_top_dn11, var_cgg_top_dn13, var_cgg_top_dn14,)
    }
};
        var_cgg_top = assign10040_e12560;
        var_cgg_top_dn0 = assign10040_e12560_d_n0;
        var_cgg_top_dn2 = assign10040_e12560_d_n2;
        var_cgg_top_dn3 = assign10040_e12560_d_n3;
        var_cgg_top_dn4 = assign10040_e12560_d_n4;
        var_cgg_top_dn5 = assign10040_e12560_d_n5;
        var_cgg_top_dn6 = assign10040_e12560_d_n6;
        var_cgg_top_dn7 = assign10040_e12560_d_n7;
        var_cgg_top_dn8 = assign10040_e12560_d_n8;
        var_cgg_top_dn9 = assign10040_e12560_d_n9;
        var_cgg_top_dn10 = assign10040_e12560_d_n10;
        var_cgg_top_dn11 = assign10040_e12560_d_n11;
        var_cgg_top_dn13 = assign10040_e12560_d_n13;
        var_cgg_top_dn14 = assign10040_e12560_d_n14;
        var_cgg_top_rv = 0.0;

        let (assign10050_e12575,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10050_e12569: f64 = (var_hg + p.p90);
        let assign10050_e12570: f64 = (0.2 * assign10050_e12569);
        let assign10050_e12572: f64 = (assign10050_e12570 / var_hrsd);
        let assign10050_e12573: f64 = (2.3 + assign10050_e12572);
        (assign10050_e12573,)
    } else {
        (var_hr,)
    }
};
        var_hr = assign10050_e12575;
        var_hr_rv = 0.0;

        let (assign10060_e12582,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        (1.05,)
    } else {
        (var_lr,)
    }
};
        var_lr = assign10060_e12582;
        var_lr_rv = 0.0;

        let (assign10070_e12594,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10070_e12589: f64 = (var_hg + p.p90);
        let assign10070_e12591: f64 = (assign10070_e12589 - var_hrsd);
        let assign10070_e12592: f64 = (assign10070_e12591).abs();
        (assign10070_e12592,)
    } else {
        (var_hgdelta,)
    }
};
        var_hgdelta = assign10070_e12594;
        var_hgdelta_rv = 0.0;

        let (assign10080_e12603,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10080_e12601: f64 = (p.p1087 * var_lr);
        (assign10080_e12601,)
    } else {
        (var_lmax,)
    }
};
        var_lmax = assign10080_e12603;
        var_lmax_rv = 0.0;

        let (assign10090_e12614,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10090_e12611: f64 = (var_hg + p.p90);
        let assign10090_e12612: f64 = (var_hrsd).min(assign10090_e12611);
        (assign10090_e12612,)
    } else {
        (var_y,)
    }
};
        var_y = assign10090_e12614;
        var_y_rv = 0.0;

        let (assign10100_e12625,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10100_e12622: f64 = (var_hr + 1.0);
        let assign10100_e12623: f64 = (p.p1087 / assign10100_e12622);
        (assign10100_e12623,)
    } else {
        (var_x,)
    }
};
        var_x = assign10100_e12625;
        var_x_rv = 0.0;

        let (assign10110_e12632,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        (1700000000000.0,)
    } else {
        (var_cnon,)
    }
};
        var_cnon = assign10110_e12632;
        var_cnon_rv = 0.0;

        let (assign10120_e12645,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10120_e12640: f64 = (var_y - var_x);
        let assign10120_e12641: f64 = (var_epssp * assign10120_e12640);
        let assign10120_e12643: f64 = (assign10120_e12641 / p.p1087);
        (assign10120_e12643,)
    } else {
        (var_ccgsat,)
    }
};
        var_ccgsat = assign10120_e12645;
        var_ccgsat_rv = 0.0;

        let (assign10130_e12654, assign10130_e12654_d_n0, assign10130_e12654_d_n2, assign10130_e12654_d_n3, assign10130_e12654_d_n4, assign10130_e12654_d_n5, assign10130_e12654_d_n6, assign10130_e12654_d_n7, assign10130_e12654_d_n8, assign10130_e12654_d_n9, assign10130_e12654_d_n10, assign10130_e12654_d_n11, assign10130_e12654_d_n13, assign10130_e12654_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10130_e12652: f64 = (var_cnon * var_ccgsat);
        (assign10130_e12652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
    }
};
        var_tt1 = assign10130_e12654;
        var_tt1_dn0 = assign10130_e12654_d_n0;
        var_tt1_dn2 = assign10130_e12654_d_n2;
        var_tt1_dn3 = assign10130_e12654_d_n3;
        var_tt1_dn4 = assign10130_e12654_d_n4;
        var_tt1_dn5 = assign10130_e12654_d_n5;
        var_tt1_dn6 = assign10130_e12654_d_n6;
        var_tt1_dn7 = assign10130_e12654_d_n7;
        var_tt1_dn8 = assign10130_e12654_d_n8;
        var_tt1_dn9 = assign10130_e12654_d_n9;
        var_tt1_dn10 = assign10130_e12654_d_n10;
        var_tt1_dn11 = assign10130_e12654_d_n11;
        var_tt1_dn13 = assign10130_e12654_d_n13;
        var_tt1_dn14 = assign10130_e12654_d_n14;
        var_tt1_rv = 0.0;

        let assign10140_e12657: f64 = if var_tt1 > 80.0 { 1.0 } else { 0.0 };
        var_guard207 = assign10140_e12657;
        var_guard207_rv = 0.0;

        let (assign10150_e12666, assign10150_e12666_d_n0, assign10150_e12666_d_n2, assign10150_e12666_d_n3, assign10150_e12666_d_n4, assign10150_e12666_d_n5, assign10150_e12666_d_n6, assign10150_e12666_d_n7, assign10150_e12666_d_n8, assign10150_e12666_d_n9, assign10150_e12666_d_n10, assign10150_e12666_d_n11, assign10150_e12666_d_n13, assign10150_e12666_d_n14,) = {
    if (((var_guard205 != 0.0) && (var_guard206 == 0.0)) && (var_guard207 != 0.0)) {
        (var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ccg1, var_ccg1_dn0, var_ccg1_dn2, var_ccg1_dn3, var_ccg1_dn4, var_ccg1_dn5, var_ccg1_dn6, var_ccg1_dn7, var_ccg1_dn8, var_ccg1_dn9, var_ccg1_dn10, var_ccg1_dn11, var_ccg1_dn13, var_ccg1_dn14,)
    }
};
        var_ccg1 = assign10150_e12666;
        var_ccg1_dn0 = assign10150_e12666_d_n0;
        var_ccg1_dn2 = assign10150_e12666_d_n2;
        var_ccg1_dn3 = assign10150_e12666_d_n3;
        var_ccg1_dn4 = assign10150_e12666_d_n4;
        var_ccg1_dn5 = assign10150_e12666_d_n5;
        var_ccg1_dn6 = assign10150_e12666_d_n6;
        var_ccg1_dn7 = assign10150_e12666_d_n7;
        var_ccg1_dn8 = assign10150_e12666_d_n8;
        var_ccg1_dn9 = assign10150_e12666_d_n9;
        var_ccg1_dn10 = assign10150_e12666_d_n10;
        var_ccg1_dn11 = assign10150_e12666_d_n11;
        var_ccg1_dn13 = assign10150_e12666_d_n13;
        var_ccg1_dn14 = assign10150_e12666_d_n14;
        var_ccg1_rv = 0.0;

        let (assign10160_e12713, assign10160_e12713_d_n0, assign10160_e12713_d_n2, assign10160_e12713_d_n3, assign10160_e12713_d_n4, assign10160_e12713_d_n5, assign10160_e12713_d_n6, assign10160_e12713_d_n7, assign10160_e12713_d_n8, assign10160_e12713_d_n9, assign10160_e12713_d_n10, assign10160_e12713_d_n11, assign10160_e12713_d_n13, assign10160_e12713_d_n14,) = {
    if (((var_guard205 != 0.0) && (var_guard206 == 0.0)) && (var_guard207 == 0.0)) {
        let assign10160_e12676: f64 = (1.0 / var_cnon);
        let assign10160_e12683: f64 = (-37.0);
        let (assign10160_e12710, assign10160_e12710_d_n0, assign10160_e12710_d_n2, assign10160_e12710_d_n3, assign10160_e12710_d_n4, assign10160_e12710_d_n5, assign10160_e12710_d_n6, assign10160_e12710_d_n7, assign10160_e12710_d_n8, assign10160_e12710_d_n9, assign10160_e12710_d_n10, assign10160_e12710_d_n11, assign10160_e12710_d_n13, assign10160_e12710_d_n14,) = {
            if ((!(var_tt1 > 37.0)) && (!(var_tt1 < assign10160_e12683))) {
                let assign10160_e12689: f64 = (var_tt1).exp();
                let assign10160_e12690: f64 = (1.0 + assign10160_e12689);
                let assign10160_e12691: f64 = (assign10160_e12690).ln();
                (assign10160_e12691, ((assign10160_e12689 * var_tt1_dn0) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn2) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn3) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn4) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn5) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn6) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn7) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn8) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn9) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn10) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn11) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn13) / assign10160_e12690), ((assign10160_e12689 * var_tt1_dn14) / assign10160_e12690),)
            } else {
                let assign10160_e12698: f64 = (-37.0);
                let (assign10160_e12709, assign10160_e12709_d_n0, assign10160_e12709_d_n2, assign10160_e12709_d_n3, assign10160_e12709_d_n4, assign10160_e12709_d_n5, assign10160_e12709_d_n6, assign10160_e12709_d_n7, assign10160_e12709_d_n8, assign10160_e12709_d_n9, assign10160_e12709_d_n10, assign10160_e12709_d_n11, assign10160_e12709_d_n13, assign10160_e12709_d_n14,) = {
                    if ((!(var_tt1 > 37.0)) && (var_tt1 < assign10160_e12698)) {
                        let assign10160_e12702: f64 = (var_tt1).exp();
                        (assign10160_e12702, (assign10160_e12702 * var_tt1_dn0), (assign10160_e12702 * var_tt1_dn2), (assign10160_e12702 * var_tt1_dn3), (assign10160_e12702 * var_tt1_dn4), (assign10160_e12702 * var_tt1_dn5), (assign10160_e12702 * var_tt1_dn6), (assign10160_e12702 * var_tt1_dn7), (assign10160_e12702 * var_tt1_dn8), (assign10160_e12702 * var_tt1_dn9), (assign10160_e12702 * var_tt1_dn10), (assign10160_e12702 * var_tt1_dn11), (assign10160_e12702 * var_tt1_dn13), (assign10160_e12702 * var_tt1_dn14),)
                    } else {
                        let (assign10160_e12708, assign10160_e12708_d_n0, assign10160_e12708_d_n2, assign10160_e12708_d_n3, assign10160_e12708_d_n4, assign10160_e12708_d_n5, assign10160_e12708_d_n6, assign10160_e12708_d_n7, assign10160_e12708_d_n8, assign10160_e12708_d_n9, assign10160_e12708_d_n10, assign10160_e12708_d_n11, assign10160_e12708_d_n13, assign10160_e12708_d_n14,) = {
                            if (var_tt1 > 37.0) {
                                (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign10160_e12708, assign10160_e12708_d_n0, assign10160_e12708_d_n2, assign10160_e12708_d_n3, assign10160_e12708_d_n4, assign10160_e12708_d_n5, assign10160_e12708_d_n6, assign10160_e12708_d_n7, assign10160_e12708_d_n8, assign10160_e12708_d_n9, assign10160_e12708_d_n10, assign10160_e12708_d_n11, assign10160_e12708_d_n13, assign10160_e12708_d_n14,)
                    }
                };
                (assign10160_e12709, assign10160_e12709_d_n0, assign10160_e12709_d_n2, assign10160_e12709_d_n3, assign10160_e12709_d_n4, assign10160_e12709_d_n5, assign10160_e12709_d_n6, assign10160_e12709_d_n7, assign10160_e12709_d_n8, assign10160_e12709_d_n9, assign10160_e12709_d_n10, assign10160_e12709_d_n11, assign10160_e12709_d_n13, assign10160_e12709_d_n14,)
            }
        };
        let assign10160_e12711: f64 = (assign10160_e12676 * assign10160_e12710);
        (assign10160_e12711, (assign10160_e12676 * assign10160_e12710_d_n0), (assign10160_e12676 * assign10160_e12710_d_n2), (assign10160_e12676 * assign10160_e12710_d_n3), (assign10160_e12676 * assign10160_e12710_d_n4), (assign10160_e12676 * assign10160_e12710_d_n5), (assign10160_e12676 * assign10160_e12710_d_n6), (assign10160_e12676 * assign10160_e12710_d_n7), (assign10160_e12676 * assign10160_e12710_d_n8), (assign10160_e12676 * assign10160_e12710_d_n9), (assign10160_e12676 * assign10160_e12710_d_n10), (assign10160_e12676 * assign10160_e12710_d_n11), (assign10160_e12676 * assign10160_e12710_d_n13), (assign10160_e12676 * assign10160_e12710_d_n14),)
    } else {
        (var_ccg1, var_ccg1_dn0, var_ccg1_dn2, var_ccg1_dn3, var_ccg1_dn4, var_ccg1_dn5, var_ccg1_dn6, var_ccg1_dn7, var_ccg1_dn8, var_ccg1_dn9, var_ccg1_dn10, var_ccg1_dn11, var_ccg1_dn13, var_ccg1_dn14,)
    }
};
        var_ccg1 = assign10160_e12713;
        var_ccg1_dn0 = assign10160_e12713_d_n0;
        var_ccg1_dn2 = assign10160_e12713_d_n2;
        var_ccg1_dn3 = assign10160_e12713_d_n3;
        var_ccg1_dn4 = assign10160_e12713_d_n4;
        var_ccg1_dn5 = assign10160_e12713_d_n5;
        var_ccg1_dn6 = assign10160_e12713_d_n6;
        var_ccg1_dn7 = assign10160_e12713_d_n7;
        var_ccg1_dn8 = assign10160_e12713_d_n8;
        var_ccg1_dn9 = assign10160_e12713_d_n9;
        var_ccg1_dn10 = assign10160_e12713_d_n10;
        var_ccg1_dn11 = assign10160_e12713_d_n11;
        var_ccg1_dn13 = assign10160_e12713_d_n13;
        var_ccg1_dn14 = assign10160_e12713_d_n14;
        var_ccg1_rv = 0.0;

        let (assign10170_e12732,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10170_e12722: f64 = (var_hg + p.p90);
        let assign10170_e12723: f64 = (var_hrsd / assign10170_e12722);
        let assign10170_e12726: f64 = (var_hg + p.p90);
        let assign10170_e12728: f64 = (assign10170_e12726 / var_hrsd);
        let assign10170_e12729: f64 = (assign10170_e12723).min(assign10170_e12728);
        let assign10170_e12730: f64 = (0.5 * assign10170_e12729);
        (assign10170_e12730,)
    } else {
        (var_r1cf,)
    }
};
        var_r1cf = assign10170_e12732;
        var_r1cf_rv = 0.0;

        let (assign10180_e12741,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10180_e12739: f64 = (var_hgdelta * var_r1cf);
        (assign10180_e12739,)
    } else {
        (var_rcf,)
    }
};
        var_rcf = assign10180_e12741;
        var_rcf_rv = 0.0;

        let (assign10190_e12791,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10190_e12748: f64 = (var_epssp * 2.0);
        let assign10190_e12750: f64 = (assign10190_e12748 / 3.141592653589793);
        let assign10190_e12754: f64 = (0.5 * 3.141592653589793);
        let assign10190_e12756: f64 = (assign10190_e12754 * var_rcf);
        let assign10190_e12757: f64 = (p.p1087 + assign10190_e12756);
        let assign10190_e12759: f64 = (assign10190_e12757 / p.p1087);
        let (assign10190_e12788,) = {
            if (!(assign10190_e12759 > 1e-38)) {
                let assign10190_e12764: f64 = (-87.498233534);
                (assign10190_e12764,)
            } else {
                let assign10190_e12768: f64 = (0.5 * 3.141592653589793);
                let assign10190_e12770: f64 = (assign10190_e12768 * var_rcf);
                let assign10190_e12771: f64 = (p.p1087 + assign10190_e12770);
                let assign10190_e12773: f64 = (assign10190_e12771 / p.p1087);
                let (assign10190_e12787,) = {
                    if (assign10190_e12773 > 1e-38) {
                        let assign10190_e12779: f64 = (0.5 * 3.141592653589793);
                        let assign10190_e12781: f64 = (assign10190_e12779 * var_rcf);
                        let assign10190_e12782: f64 = (p.p1087 + assign10190_e12781);
                        let assign10190_e12784: f64 = (assign10190_e12782 / p.p1087);
                        let assign10190_e12785: f64 = (assign10190_e12784).ln();
                        (assign10190_e12785,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10190_e12787,)
            }
        };
        let assign10190_e12789: f64 = (assign10190_e12750 * assign10190_e12788);
        (assign10190_e12789,)
    } else {
        (var_ccg2,)
    }
};
        var_ccg2 = assign10190_e12791;
        var_ccg2_rv = 0.0;

        let (assign10200_e12802, assign10200_e12802_d_n0, assign10200_e12802_d_n2, assign10200_e12802_d_n3, assign10200_e12802_d_n4, assign10200_e12802_d_n5, assign10200_e12802_d_n6, assign10200_e12802_d_n7, assign10200_e12802_d_n8, assign10200_e12802_d_n9, assign10200_e12802_d_n10, assign10200_e12802_d_n11, assign10200_e12802_d_n13, assign10200_e12802_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10200_e12799: f64 = (var_ccg1 + var_ccg2);
        let assign10200_e12800: f64 = (p.p3 * assign10200_e12799);
        (assign10200_e12800, (p.p3 * var_ccg1_dn0), (p.p3 * var_ccg1_dn2), (p.p3 * var_ccg1_dn3), (p.p3 * var_ccg1_dn4), (p.p3 * var_ccg1_dn5), (p.p3 * var_ccg1_dn6), (p.p3 * var_ccg1_dn7), (p.p3 * var_ccg1_dn8), (p.p3 * var_ccg1_dn9), (p.p3 * var_ccg1_dn10), (p.p3 * var_ccg1_dn11), (p.p3 * var_ccg1_dn13), (p.p3 * var_ccg1_dn14),)
    } else {
        (var_ccg, var_ccg_dn0, var_ccg_dn2, var_ccg_dn3, var_ccg_dn4, var_ccg_dn5, var_ccg_dn6, var_ccg_dn7, var_ccg_dn8, var_ccg_dn9, var_ccg_dn10, var_ccg_dn11, var_ccg_dn13, var_ccg_dn14,)
    }
};
        var_ccg = assign10200_e12802;
        var_ccg_dn0 = assign10200_e12802_d_n0;
        var_ccg_dn2 = assign10200_e12802_d_n2;
        var_ccg_dn3 = assign10200_e12802_d_n3;
        var_ccg_dn4 = assign10200_e12802_d_n4;
        var_ccg_dn5 = assign10200_e12802_d_n5;
        var_ccg_dn6 = assign10200_e12802_d_n6;
        var_ccg_dn7 = assign10200_e12802_d_n7;
        var_ccg_dn8 = assign10200_e12802_d_n8;
        var_ccg_dn9 = assign10200_e12802_d_n9;
        var_ccg_dn10 = assign10200_e12802_d_n10;
        var_ccg_dn11 = assign10200_e12802_d_n11;
        var_ccg_dn13 = assign10200_e12802_d_n13;
        var_ccg_dn14 = assign10200_e12802_d_n14;
        var_ccg_rv = 0.0;

        let (assign10210_e12811,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10210_e12809: f64 = (var_lmax / var_hg);
        (assign10210_e12809,)
    } else {
        (var_x,)
    }
};
        var_x = assign10210_e12811;
        var_x_rv = 0.0;

        let (assign10220_e12827,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10220_e12820: f64 = (var_x + 1.0);
        let assign10220_e12821: f64 = (2.0 * assign10220_e12820);
        let assign10220_e12822: f64 = (assign10220_e12821).sqrt();
        let assign10220_e12824: f64 = (assign10220_e12822 * 3.141592653589793);
        let assign10220_e12825: f64 = (4.0 / assign10220_e12824);
        (assign10220_e12825,)
    } else {
        (var_c1,)
    }
};
        var_c1 = assign10220_e12827;
        var_c1_rv = 0.0;

        let (assign10230_e12864,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10230_e12834: f64 = (p.p90 * p.p90);
        let assign10230_e12837: f64 = (2.0 * var_hg);
        let assign10230_e12839: f64 = (assign10230_e12837 * p.p90);
        let assign10230_e12840: f64 = (assign10230_e12834 + assign10230_e12839);
        let assign10230_e12843: f64 = (var_hg * var_hg);
        let assign10230_e12846: f64 = (var_x + 1.0);
        let assign10230_e12847: f64 = (assign10230_e12843 * assign10230_e12846);
        let assign10230_e12848: f64 = (assign10230_e12840 + assign10230_e12847);
        let assign10230_e12849: f64 = (assign10230_e12848).sqrt();
        let assign10230_e12852: f64 = (var_x + 1.0);
        let assign10230_e12853: f64 = (assign10230_e12852).sqrt();
        let assign10230_e12854: f64 = (assign10230_e12849 * assign10230_e12853);
        let assign10230_e12856: f64 = (assign10230_e12854 + p.p90);
        let assign10230_e12859: f64 = (var_hg * var_x);
        let assign10230_e12860: f64 = (assign10230_e12856 + assign10230_e12859);
        let assign10230_e12862: f64 = (assign10230_e12860 + var_hg);
        (assign10230_e12862,)
    } else {
        (var_c2,)
    }
};
        var_c2 = assign10230_e12864;
        var_c2_rv = 0.0;

        let (assign10240_e12886,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10240_e12872: f64 = (var_x + 1.0);
        let assign10240_e12875: f64 = (var_x + 4.0);
        let assign10240_e12876: f64 = (assign10240_e12872 * assign10240_e12875);
        let assign10240_e12877: f64 = (assign10240_e12876).sqrt();
        let assign10240_e12878: f64 = (p.p90 * assign10240_e12877);
        let assign10240_e12882: f64 = (var_x + 2.0);
        let assign10240_e12883: f64 = (p.p90 * assign10240_e12882);
        let assign10240_e12884: f64 = (assign10240_e12878 + assign10240_e12883);
        (assign10240_e12884,)
    } else {
        (var_c3,)
    }
};
        var_c3 = assign10240_e12886;
        var_c3_rv = 0.0;

        let (assign10250_e12918,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10250_e12895: f64 = (var_c2 / var_c3);
        let (assign10250_e12912,) = {
            if (!(assign10250_e12895 > 1e-38)) {
                let assign10250_e12900: f64 = (-87.498233534);
                (assign10250_e12900,)
            } else {
                let assign10250_e12903: f64 = (var_c2 / var_c3);
                let (assign10250_e12911,) = {
                    if (assign10250_e12903 > 1e-38) {
                        let assign10250_e12908: f64 = (var_c2 / var_c3);
                        let assign10250_e12909: f64 = (assign10250_e12908).ln();
                        (assign10250_e12909,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10250_e12911,)
            }
        };
        let assign10250_e12913: f64 = (var_c1 * assign10250_e12912);
        let assign10250_e12915: f64 = (assign10250_e12913 + 12.27);
        let assign10250_e12916: f64 = (var_epssp * assign10250_e12915);
        (assign10250_e12916,)
    } else {
        (var_cfglog,)
    }
};
        var_cfglog = assign10250_e12918;
        var_cfglog_rv = 0.0;

        let (assign10260_e12927,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10260_e12925: f64 = (var_hr * var_lr);
        (assign10260_e12925,)
    } else {
        (var_dcf,)
    }
};
        var_dcf = assign10260_e12927;
        var_dcf_rv = 0.0;

        *var_c1_slot = var_c1;
        *var_c1_rv_slot = var_c1_rv;
        *var_c2_slot = var_c2;
        *var_c2_rv_slot = var_c2_rv;
        *var_c3_slot = var_c3;
        *var_c3_rv_slot = var_c3_rv;
        *var_ccg_slot = var_ccg;
        *var_ccg1_slot = var_ccg1;
        *var_ccg1_dn0_slot = var_ccg1_dn0;
        *var_ccg1_dn10_slot = var_ccg1_dn10;
        *var_ccg1_dn11_slot = var_ccg1_dn11;
        *var_ccg1_dn13_slot = var_ccg1_dn13;
        *var_ccg1_dn14_slot = var_ccg1_dn14;
        *var_ccg1_dn2_slot = var_ccg1_dn2;
        *var_ccg1_dn3_slot = var_ccg1_dn3;
        *var_ccg1_dn4_slot = var_ccg1_dn4;
        *var_ccg1_dn5_slot = var_ccg1_dn5;
        *var_ccg1_dn6_slot = var_ccg1_dn6;
        *var_ccg1_dn7_slot = var_ccg1_dn7;
        *var_ccg1_dn8_slot = var_ccg1_dn8;
        *var_ccg1_dn9_slot = var_ccg1_dn9;
        *var_ccg1_rv_slot = var_ccg1_rv;
        *var_ccg2_slot = var_ccg2;
        *var_ccg2_rv_slot = var_ccg2_rv;
        *var_ccg_dn0_slot = var_ccg_dn0;
        *var_ccg_dn10_slot = var_ccg_dn10;
        *var_ccg_dn11_slot = var_ccg_dn11;
        *var_ccg_dn13_slot = var_ccg_dn13;
        *var_ccg_dn14_slot = var_ccg_dn14;
        *var_ccg_dn2_slot = var_ccg_dn2;
        *var_ccg_dn3_slot = var_ccg_dn3;
        *var_ccg_dn4_slot = var_ccg_dn4;
        *var_ccg_dn5_slot = var_ccg_dn5;
        *var_ccg_dn6_slot = var_ccg_dn6;
        *var_ccg_dn7_slot = var_ccg_dn7;
        *var_ccg_dn8_slot = var_ccg_dn8;
        *var_ccg_dn9_slot = var_ccg_dn9;
        *var_ccg_rv_slot = var_ccg_rv;
        *var_ccgsat_slot = var_ccgsat;
        *var_ccgsat_rv_slot = var_ccgsat_rv;
        *var_cfglog_slot = var_cfglog;
        *var_cfglog_rv_slot = var_cfglog_rv;
        *var_cgg_top_slot = var_cgg_top;
        *var_cgg_top_dn0_slot = var_cgg_top_dn0;
        *var_cgg_top_dn10_slot = var_cgg_top_dn10;
        *var_cgg_top_dn11_slot = var_cgg_top_dn11;
        *var_cgg_top_dn13_slot = var_cgg_top_dn13;
        *var_cgg_top_dn14_slot = var_cgg_top_dn14;
        *var_cgg_top_dn2_slot = var_cgg_top_dn2;
        *var_cgg_top_dn3_slot = var_cgg_top_dn3;
        *var_cgg_top_dn4_slot = var_cgg_top_dn4;
        *var_cgg_top_dn5_slot = var_cgg_top_dn5;
        *var_cgg_top_dn6_slot = var_cgg_top_dn6;
        *var_cgg_top_dn7_slot = var_cgg_top_dn7;
        *var_cgg_top_dn8_slot = var_cgg_top_dn8;
        *var_cgg_top_dn9_slot = var_cgg_top_dn9;
        *var_cgg_top_rv_slot = var_cgg_top_rv;
        *var_cnon_slot = var_cnon;
        *var_cnon_rv_slot = var_cnon_rv;
        *var_dcf_slot = var_dcf;
        *var_dcf_rv_slot = var_dcf_rv;
        *var_guard207_slot = var_guard207;
        *var_guard207_rv_slot = var_guard207_rv;
        *var_hgdelta_slot = var_hgdelta;
        *var_hgdelta_rv_slot = var_hgdelta_rv;
        *var_hr_slot = var_hr;
        *var_hr_rv_slot = var_hr_rv;
        *var_lmax_slot = var_lmax;
        *var_lmax_rv_slot = var_lmax_rv;
        *var_lr_slot = var_lr;
        *var_lr_rv_slot = var_lr_rv;
        *var_r1cf_slot = var_r1cf;
        *var_r1cf_rv_slot = var_r1cf_rv;
        *var_rcf_slot = var_rcf;
        *var_rcf_rv_slot = var_rcf_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_tt1_slot = var_tt1;
        *var_tt1_dn0_slot = var_tt1_dn0;
        *var_tt1_dn10_slot = var_tt1_dn10;
        *var_tt1_dn11_slot = var_tt1_dn11;
        *var_tt1_dn13_slot = var_tt1_dn13;
        *var_tt1_dn14_slot = var_tt1_dn14;
        *var_tt1_dn2_slot = var_tt1_dn2;
        *var_tt1_dn3_slot = var_tt1_dn3;
        *var_tt1_dn4_slot = var_tt1_dn4;
        *var_tt1_dn5_slot = var_tt1_dn5;
        *var_tt1_dn6_slot = var_tt1_dn6;
        *var_tt1_dn7_slot = var_tt1_dn7;
        *var_tt1_dn8_slot = var_tt1_dn8;
        *var_tt1_dn9_slot = var_tt1_dn9;
        *var_tt1_rv_slot = var_tt1_rv;
        *var_x_slot = var_x;
        *var_x_rv_slot = var_x_rv;
        *var_y_slot = var_y;
        *var_y_rv_slot = var_y_rv;
    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        var_ccg: f64,
        var_ccg_dn0: f64,
        var_ccg_dn10: f64,
        var_ccg_dn11: f64,
        var_ccg_dn13: f64,
        var_ccg_dn14: f64,
        var_ccg_dn2: f64,
        var_ccg_dn3: f64,
        var_ccg_dn4: f64,
        var_ccg_dn5: f64,
        var_ccg_dn6: f64,
        var_ccg_dn7: f64,
        var_ccg_dn8: f64,
        var_ccg_dn9: f64,
        var_cfglog: f64,
        var_dcf: f64,
        var_epssp: f64,
        var_guard205: f64,
        var_guard206: f64,
        var_trsd: f64,
        var_wg: f64,
        var_ccg1_slot: &mut f64,
        var_ccg1_dn0_slot: &mut f64,
        var_ccg1_dn10_slot: &mut f64,
        var_ccg1_dn11_slot: &mut f64,
        var_ccg1_dn13_slot: &mut f64,
        var_ccg1_dn14_slot: &mut f64,
        var_ccg1_dn2_slot: &mut f64,
        var_ccg1_dn3_slot: &mut f64,
        var_ccg1_dn4_slot: &mut f64,
        var_ccg1_dn5_slot: &mut f64,
        var_ccg1_dn6_slot: &mut f64,
        var_ccg1_dn7_slot: &mut f64,
        var_ccg1_dn8_slot: &mut f64,
        var_ccg1_dn9_slot: &mut f64,
        var_ccg1_rv_slot: &mut f64,
        var_ccg2_slot: &mut f64,
        var_ccg2_rv_slot: &mut f64,
        var_ccgsat_slot: &mut f64,
        var_ccgsat_rv_slot: &mut f64,
        var_cfg_slot: &mut f64,
        var_cfg_dn0_slot: &mut f64,
        var_cfg_dn10_slot: &mut f64,
        var_cfg_dn11_slot: &mut f64,
        var_cfg_dn13_slot: &mut f64,
        var_cfg_dn14_slot: &mut f64,
        var_cfg_dn2_slot: &mut f64,
        var_cfg_dn3_slot: &mut f64,
        var_cfg_dn4_slot: &mut f64,
        var_cfg_dn5_slot: &mut f64,
        var_cfg_dn6_slot: &mut f64,
        var_cfg_dn7_slot: &mut f64,
        var_cfg_dn8_slot: &mut f64,
        var_cfg_dn9_slot: &mut f64,
        var_cfg_rv_slot: &mut f64,
        var_cfgsat_slot: &mut f64,
        var_cfgsat_dn0_slot: &mut f64,
        var_cfgsat_dn10_slot: &mut f64,
        var_cfgsat_dn11_slot: &mut f64,
        var_cfgsat_dn13_slot: &mut f64,
        var_cfgsat_dn14_slot: &mut f64,
        var_cfgsat_dn2_slot: &mut f64,
        var_cfgsat_dn3_slot: &mut f64,
        var_cfgsat_dn4_slot: &mut f64,
        var_cfgsat_dn5_slot: &mut f64,
        var_cfgsat_dn6_slot: &mut f64,
        var_cfgsat_dn7_slot: &mut f64,
        var_cfgsat_dn8_slot: &mut f64,
        var_cfgsat_dn9_slot: &mut f64,
        var_cfgsat_rv_slot: &mut f64,
        var_cgg_top_slot: &mut f64,
        var_cgg_top_dn0_slot: &mut f64,
        var_cgg_top_dn10_slot: &mut f64,
        var_cgg_top_dn11_slot: &mut f64,
        var_cgg_top_dn13_slot: &mut f64,
        var_cgg_top_dn14_slot: &mut f64,
        var_cgg_top_dn2_slot: &mut f64,
        var_cgg_top_dn3_slot: &mut f64,
        var_cgg_top_dn4_slot: &mut f64,
        var_cgg_top_dn5_slot: &mut f64,
        var_cgg_top_dn6_slot: &mut f64,
        var_cgg_top_dn7_slot: &mut f64,
        var_cgg_top_dn8_slot: &mut f64,
        var_cgg_top_dn9_slot: &mut f64,
        var_cgg_top_rv_slot: &mut f64,
        var_cnon_slot: &mut f64,
        var_cnon_rv_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn0_slot: &mut f64,
        var_delta_dn10_slot: &mut f64,
        var_delta_dn11_slot: &mut f64,
        var_delta_dn13_slot: &mut f64,
        var_delta_dn14_slot: &mut f64,
        var_delta_dn2_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_delta_dn9_slot: &mut f64,
        var_delta_rv_slot: &mut f64,
        var_guard208_slot: &mut f64,
        var_guard208_rv_slot: &mut f64,
        var_guard209_slot: &mut f64,
        var_guard209_rv_slot: &mut f64,
        var_hgdelta_slot: &mut f64,
        var_hgdelta_rv_slot: &mut f64,
        var_hr_slot: &mut f64,
        var_hr_rv_slot: &mut f64,
        var_lmax_slot: &mut f64,
        var_lmax_rv_slot: &mut f64,
        var_lr_slot: &mut f64,
        var_lr_rv_slot: &mut f64,
        var_r1cf_slot: &mut f64,
        var_r1cf_rv_slot: &mut f64,
        var_rcf_slot: &mut f64,
        var_rcf_rv_slot: &mut f64,
        var_tt0_slot: &mut f64,
        var_tt0_rv_slot: &mut f64,
        var_tt1_slot: &mut f64,
        var_tt1_dn0_slot: &mut f64,
        var_tt1_dn10_slot: &mut f64,
        var_tt1_dn11_slot: &mut f64,
        var_tt1_dn13_slot: &mut f64,
        var_tt1_dn14_slot: &mut f64,
        var_tt1_dn2_slot: &mut f64,
        var_tt1_dn3_slot: &mut f64,
        var_tt1_dn4_slot: &mut f64,
        var_tt1_dn5_slot: &mut f64,
        var_tt1_dn6_slot: &mut f64,
        var_tt1_dn7_slot: &mut f64,
        var_tt1_dn8_slot: &mut f64,
        var_tt1_dn9_slot: &mut f64,
        var_tt1_rv_slot: &mut f64,
        var_tt2_slot: &mut f64,
        var_tt2_rv_slot: &mut f64,
        var_x_slot: &mut f64,
        var_x_rv_slot: &mut f64,
        var_y_slot: &mut f64,
        var_y_rv_slot: &mut f64,
    ) {
        let mut var_ccg1: f64 = *var_ccg1_slot;
        let mut var_ccg1_dn0: f64 = *var_ccg1_dn0_slot;
        let mut var_ccg1_dn10: f64 = *var_ccg1_dn10_slot;
        let mut var_ccg1_dn11: f64 = *var_ccg1_dn11_slot;
        let mut var_ccg1_dn13: f64 = *var_ccg1_dn13_slot;
        let mut var_ccg1_dn14: f64 = *var_ccg1_dn14_slot;
        let mut var_ccg1_dn2: f64 = *var_ccg1_dn2_slot;
        let mut var_ccg1_dn3: f64 = *var_ccg1_dn3_slot;
        let mut var_ccg1_dn4: f64 = *var_ccg1_dn4_slot;
        let mut var_ccg1_dn5: f64 = *var_ccg1_dn5_slot;
        let mut var_ccg1_dn6: f64 = *var_ccg1_dn6_slot;
        let mut var_ccg1_dn7: f64 = *var_ccg1_dn7_slot;
        let mut var_ccg1_dn8: f64 = *var_ccg1_dn8_slot;
        let mut var_ccg1_dn9: f64 = *var_ccg1_dn9_slot;
        let mut var_ccg1_rv: f64 = *var_ccg1_rv_slot;
        let mut var_ccg2: f64 = *var_ccg2_slot;
        let mut var_ccg2_rv: f64 = *var_ccg2_rv_slot;
        let mut var_ccgsat: f64 = *var_ccgsat_slot;
        let mut var_ccgsat_rv: f64 = *var_ccgsat_rv_slot;
        let mut var_cfg: f64 = *var_cfg_slot;
        let mut var_cfg_dn0: f64 = *var_cfg_dn0_slot;
        let mut var_cfg_dn10: f64 = *var_cfg_dn10_slot;
        let mut var_cfg_dn11: f64 = *var_cfg_dn11_slot;
        let mut var_cfg_dn13: f64 = *var_cfg_dn13_slot;
        let mut var_cfg_dn14: f64 = *var_cfg_dn14_slot;
        let mut var_cfg_dn2: f64 = *var_cfg_dn2_slot;
        let mut var_cfg_dn3: f64 = *var_cfg_dn3_slot;
        let mut var_cfg_dn4: f64 = *var_cfg_dn4_slot;
        let mut var_cfg_dn5: f64 = *var_cfg_dn5_slot;
        let mut var_cfg_dn6: f64 = *var_cfg_dn6_slot;
        let mut var_cfg_dn7: f64 = *var_cfg_dn7_slot;
        let mut var_cfg_dn8: f64 = *var_cfg_dn8_slot;
        let mut var_cfg_dn9: f64 = *var_cfg_dn9_slot;
        let mut var_cfg_rv: f64 = *var_cfg_rv_slot;
        let mut var_cfgsat: f64 = *var_cfgsat_slot;
        let mut var_cfgsat_dn0: f64 = *var_cfgsat_dn0_slot;
        let mut var_cfgsat_dn10: f64 = *var_cfgsat_dn10_slot;
        let mut var_cfgsat_dn11: f64 = *var_cfgsat_dn11_slot;
        let mut var_cfgsat_dn13: f64 = *var_cfgsat_dn13_slot;
        let mut var_cfgsat_dn14: f64 = *var_cfgsat_dn14_slot;
        let mut var_cfgsat_dn2: f64 = *var_cfgsat_dn2_slot;
        let mut var_cfgsat_dn3: f64 = *var_cfgsat_dn3_slot;
        let mut var_cfgsat_dn4: f64 = *var_cfgsat_dn4_slot;
        let mut var_cfgsat_dn5: f64 = *var_cfgsat_dn5_slot;
        let mut var_cfgsat_dn6: f64 = *var_cfgsat_dn6_slot;
        let mut var_cfgsat_dn7: f64 = *var_cfgsat_dn7_slot;
        let mut var_cfgsat_dn8: f64 = *var_cfgsat_dn8_slot;
        let mut var_cfgsat_dn9: f64 = *var_cfgsat_dn9_slot;
        let mut var_cfgsat_rv: f64 = *var_cfgsat_rv_slot;
        let mut var_cgg_top: f64 = *var_cgg_top_slot;
        let mut var_cgg_top_dn0: f64 = *var_cgg_top_dn0_slot;
        let mut var_cgg_top_dn10: f64 = *var_cgg_top_dn10_slot;
        let mut var_cgg_top_dn11: f64 = *var_cgg_top_dn11_slot;
        let mut var_cgg_top_dn13: f64 = *var_cgg_top_dn13_slot;
        let mut var_cgg_top_dn14: f64 = *var_cgg_top_dn14_slot;
        let mut var_cgg_top_dn2: f64 = *var_cgg_top_dn2_slot;
        let mut var_cgg_top_dn3: f64 = *var_cgg_top_dn3_slot;
        let mut var_cgg_top_dn4: f64 = *var_cgg_top_dn4_slot;
        let mut var_cgg_top_dn5: f64 = *var_cgg_top_dn5_slot;
        let mut var_cgg_top_dn6: f64 = *var_cgg_top_dn6_slot;
        let mut var_cgg_top_dn7: f64 = *var_cgg_top_dn7_slot;
        let mut var_cgg_top_dn8: f64 = *var_cgg_top_dn8_slot;
        let mut var_cgg_top_dn9: f64 = *var_cgg_top_dn9_slot;
        let mut var_cgg_top_rv: f64 = *var_cgg_top_rv_slot;
        let mut var_cnon: f64 = *var_cnon_slot;
        let mut var_cnon_rv: f64 = *var_cnon_rv_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn0: f64 = *var_delta_dn0_slot;
        let mut var_delta_dn10: f64 = *var_delta_dn10_slot;
        let mut var_delta_dn11: f64 = *var_delta_dn11_slot;
        let mut var_delta_dn13: f64 = *var_delta_dn13_slot;
        let mut var_delta_dn14: f64 = *var_delta_dn14_slot;
        let mut var_delta_dn2: f64 = *var_delta_dn2_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_delta_dn9: f64 = *var_delta_dn9_slot;
        let mut var_delta_rv: f64 = *var_delta_rv_slot;
        let mut var_guard208: f64 = *var_guard208_slot;
        let mut var_guard208_rv: f64 = *var_guard208_rv_slot;
        let mut var_guard209: f64 = *var_guard209_slot;
        let mut var_guard209_rv: f64 = *var_guard209_rv_slot;
        let mut var_hgdelta: f64 = *var_hgdelta_slot;
        let mut var_hgdelta_rv: f64 = *var_hgdelta_rv_slot;
        let mut var_hr: f64 = *var_hr_slot;
        let mut var_hr_rv: f64 = *var_hr_rv_slot;
        let mut var_lmax: f64 = *var_lmax_slot;
        let mut var_lmax_rv: f64 = *var_lmax_rv_slot;
        let mut var_lr: f64 = *var_lr_slot;
        let mut var_lr_rv: f64 = *var_lr_rv_slot;
        let mut var_r1cf: f64 = *var_r1cf_slot;
        let mut var_r1cf_rv: f64 = *var_r1cf_rv_slot;
        let mut var_rcf: f64 = *var_rcf_slot;
        let mut var_rcf_rv: f64 = *var_rcf_rv_slot;
        let mut var_tt0: f64 = *var_tt0_slot;
        let mut var_tt0_rv: f64 = *var_tt0_rv_slot;
        let mut var_tt1: f64 = *var_tt1_slot;
        let mut var_tt1_dn0: f64 = *var_tt1_dn0_slot;
        let mut var_tt1_dn10: f64 = *var_tt1_dn10_slot;
        let mut var_tt1_dn11: f64 = *var_tt1_dn11_slot;
        let mut var_tt1_dn13: f64 = *var_tt1_dn13_slot;
        let mut var_tt1_dn14: f64 = *var_tt1_dn14_slot;
        let mut var_tt1_dn2: f64 = *var_tt1_dn2_slot;
        let mut var_tt1_dn3: f64 = *var_tt1_dn3_slot;
        let mut var_tt1_dn4: f64 = *var_tt1_dn4_slot;
        let mut var_tt1_dn5: f64 = *var_tt1_dn5_slot;
        let mut var_tt1_dn6: f64 = *var_tt1_dn6_slot;
        let mut var_tt1_dn7: f64 = *var_tt1_dn7_slot;
        let mut var_tt1_dn8: f64 = *var_tt1_dn8_slot;
        let mut var_tt1_dn9: f64 = *var_tt1_dn9_slot;
        let mut var_tt1_rv: f64 = *var_tt1_rv_slot;
        let mut var_tt2: f64 = *var_tt2_slot;
        let mut var_tt2_rv: f64 = *var_tt2_rv_slot;
        let mut var_x: f64 = *var_x_slot;
        let mut var_x_rv: f64 = *var_x_rv_slot;
        let mut var_y: f64 = *var_y_slot;
        let mut var_y_rv: f64 = *var_y_rv_slot;

        let (assign10270_e12939,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10270_e12934: f64 = (var_dcf * var_dcf);
        let assign10270_e12936: f64 = (assign10270_e12934 + 1.0);
        let assign10270_e12937: f64 = (assign10270_e12936).sqrt();
        (assign10270_e12937,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign10270_e12939;
        var_tt0_rv = 0.0;

        let (assign10280_e12989, assign10280_e12989_d_n0, assign10280_e12989_d_n2, assign10280_e12989_d_n3, assign10280_e12989_d_n4, assign10280_e12989_d_n5, assign10280_e12989_d_n6, assign10280_e12989_d_n7, assign10280_e12989_d_n8, assign10280_e12989_d_n9, assign10280_e12989_d_n10, assign10280_e12989_d_n11, assign10280_e12989_d_n13, assign10280_e12989_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10280_e12946: f64 = (var_dcf * var_dcf);
        let assign10280_e12948: f64 = (assign10280_e12946 + 1.0);
        let assign10280_e12951: f64 = (var_dcf * p.p90);
        let assign10280_e12954: f64 = (var_dcf * p.p90);
        let assign10280_e12955: f64 = (assign10280_e12951 * assign10280_e12954);
        let assign10280_e12958: f64 = (2.0 * var_dcf);
        let assign10280_e12960: f64 = (assign10280_e12958 * var_lmax);
        let assign10280_e12962: f64 = (assign10280_e12960 * p.p90);
        let assign10280_e12963: f64 = (assign10280_e12955 + assign10280_e12962);
        let assign10280_e12966: f64 = (var_dcf * var_dcf);
        let assign10280_e12968: f64 = (assign10280_e12966 + 1.0);
        let assign10280_e12970: f64 = (assign10280_e12968 * var_lmax);
        let assign10280_e12972: f64 = (assign10280_e12970 * var_lmax);
        let assign10280_e12973: f64 = (assign10280_e12963 + assign10280_e12972);
        let assign10280_e12974: f64 = (assign10280_e12948 * assign10280_e12973);
        let assign10280_e12975: f64 = (assign10280_e12974).sqrt();
        let assign10280_e12978: f64 = (var_dcf * p.p90);
        let assign10280_e12979: f64 = (assign10280_e12975 + assign10280_e12978);
        let assign10280_e12982: f64 = (var_dcf * var_dcf);
        let assign10280_e12984: f64 = (assign10280_e12982 * var_lmax);
        let assign10280_e12985: f64 = (assign10280_e12979 + assign10280_e12984);
        let assign10280_e12987: f64 = (assign10280_e12985 + var_lmax);
        (assign10280_e12987, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
    }
};
        var_tt1 = assign10280_e12989;
        var_tt1_dn0 = assign10280_e12989_d_n0;
        var_tt1_dn2 = assign10280_e12989_d_n2;
        var_tt1_dn3 = assign10280_e12989_d_n3;
        var_tt1_dn4 = assign10280_e12989_d_n4;
        var_tt1_dn5 = assign10280_e12989_d_n5;
        var_tt1_dn6 = assign10280_e12989_d_n6;
        var_tt1_dn7 = assign10280_e12989_d_n7;
        var_tt1_dn8 = assign10280_e12989_d_n8;
        var_tt1_dn9 = assign10280_e12989_d_n9;
        var_tt1_dn10 = assign10280_e12989_d_n10;
        var_tt1_dn11 = assign10280_e12989_d_n11;
        var_tt1_dn13 = assign10280_e12989_d_n13;
        var_tt1_dn14 = assign10280_e12989_d_n14;
        var_tt1_rv = 0.0;

        let (assign10290_e13002,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10290_e12996: f64 = (var_tt0 + 1.0);
        let assign10290_e12999: f64 = (var_dcf * p.p90);
        let assign10290_e13000: f64 = (assign10290_e12996 * assign10290_e12999);
        (assign10290_e13000,)
    } else {
        (var_tt2,)
    }
};
        var_tt2 = assign10290_e13002;
        var_tt2_rv = 0.0;

        let (assign10300_e13043, assign10300_e13043_d_n0, assign10300_e13043_d_n2, assign10300_e13043_d_n3, assign10300_e13043_d_n4, assign10300_e13043_d_n5, assign10300_e13043_d_n6, assign10300_e13043_d_n7, assign10300_e13043_d_n8, assign10300_e13043_d_n9, assign10300_e13043_d_n10, assign10300_e13043_d_n11, assign10300_e13043_d_n13, assign10300_e13043_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10300_e13009: f64 = (2.0 * var_epssp);
        let assign10300_e13011: f64 = (2.0_f64).sqrt();
        let assign10300_e13012: f64 = (assign10300_e13009 * assign10300_e13011);
        let assign10300_e13014: f64 = (assign10300_e13012 / 3.141592653589793);
        let assign10300_e13016: f64 = (assign10300_e13014 * 0.85);
        let assign10300_e13018: f64 = (assign10300_e13016 * var_dcf);
        let assign10300_e13020: f64 = (assign10300_e13018 / var_tt0);
        let assign10300_e13023: f64 = (var_tt1 / var_tt2);
        let (assign10300_e13040, assign10300_e13040_d_n0, assign10300_e13040_d_n2, assign10300_e13040_d_n3, assign10300_e13040_d_n4, assign10300_e13040_d_n5, assign10300_e13040_d_n6, assign10300_e13040_d_n7, assign10300_e13040_d_n8, assign10300_e13040_d_n9, assign10300_e13040_d_n10, assign10300_e13040_d_n11, assign10300_e13040_d_n13, assign10300_e13040_d_n14,) = {
            if (!(assign10300_e13023 > 1e-38)) {
                let assign10300_e13028: f64 = (-87.498233534);
                (assign10300_e13028, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign10300_e13031: f64 = (var_tt1 / var_tt2);
                let (assign10300_e13039, assign10300_e13039_d_n0, assign10300_e13039_d_n2, assign10300_e13039_d_n3, assign10300_e13039_d_n4, assign10300_e13039_d_n5, assign10300_e13039_d_n6, assign10300_e13039_d_n7, assign10300_e13039_d_n8, assign10300_e13039_d_n9, assign10300_e13039_d_n10, assign10300_e13039_d_n11, assign10300_e13039_d_n13, assign10300_e13039_d_n14,) = {
                    if (assign10300_e13031 > 1e-38) {
                        let assign10300_e13036: f64 = (var_tt1 / var_tt2);
                        let assign10300_e13037: f64 = (assign10300_e13036).ln();
                        (assign10300_e13037, ((var_tt1_dn0 / var_tt2) / assign10300_e13036), ((var_tt1_dn2 / var_tt2) / assign10300_e13036), ((var_tt1_dn3 / var_tt2) / assign10300_e13036), ((var_tt1_dn4 / var_tt2) / assign10300_e13036), ((var_tt1_dn5 / var_tt2) / assign10300_e13036), ((var_tt1_dn6 / var_tt2) / assign10300_e13036), ((var_tt1_dn7 / var_tt2) / assign10300_e13036), ((var_tt1_dn8 / var_tt2) / assign10300_e13036), ((var_tt1_dn9 / var_tt2) / assign10300_e13036), ((var_tt1_dn10 / var_tt2) / assign10300_e13036), ((var_tt1_dn11 / var_tt2) / assign10300_e13036), ((var_tt1_dn13 / var_tt2) / assign10300_e13036), ((var_tt1_dn14 / var_tt2) / assign10300_e13036),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign10300_e13039, assign10300_e13039_d_n0, assign10300_e13039_d_n2, assign10300_e13039_d_n3, assign10300_e13039_d_n4, assign10300_e13039_d_n5, assign10300_e13039_d_n6, assign10300_e13039_d_n7, assign10300_e13039_d_n8, assign10300_e13039_d_n9, assign10300_e13039_d_n10, assign10300_e13039_d_n11, assign10300_e13039_d_n13, assign10300_e13039_d_n14,)
            }
        };
        let assign10300_e13041: f64 = (assign10300_e13020 * assign10300_e13040);
        (assign10300_e13041, (assign10300_e13020 * assign10300_e13040_d_n0), (assign10300_e13020 * assign10300_e13040_d_n2), (assign10300_e13020 * assign10300_e13040_d_n3), (assign10300_e13020 * assign10300_e13040_d_n4), (assign10300_e13020 * assign10300_e13040_d_n5), (assign10300_e13020 * assign10300_e13040_d_n6), (assign10300_e13020 * assign10300_e13040_d_n7), (assign10300_e13020 * assign10300_e13040_d_n8), (assign10300_e13020 * assign10300_e13040_d_n9), (assign10300_e13020 * assign10300_e13040_d_n10), (assign10300_e13020 * assign10300_e13040_d_n11), (assign10300_e13020 * assign10300_e13040_d_n13), (assign10300_e13020 * assign10300_e13040_d_n14),)
    } else {
        (var_cfgsat, var_cfgsat_dn0, var_cfgsat_dn2, var_cfgsat_dn3, var_cfgsat_dn4, var_cfgsat_dn5, var_cfgsat_dn6, var_cfgsat_dn7, var_cfgsat_dn8, var_cfgsat_dn9, var_cfgsat_dn10, var_cfgsat_dn11, var_cfgsat_dn13, var_cfgsat_dn14,)
    }
};
        var_cfgsat = assign10300_e13043;
        var_cfgsat_dn0 = assign10300_e13043_d_n0;
        var_cfgsat_dn2 = assign10300_e13043_d_n2;
        var_cfgsat_dn3 = assign10300_e13043_d_n3;
        var_cfgsat_dn4 = assign10300_e13043_d_n4;
        var_cfgsat_dn5 = assign10300_e13043_d_n5;
        var_cfgsat_dn6 = assign10300_e13043_d_n6;
        var_cfgsat_dn7 = assign10300_e13043_d_n7;
        var_cfgsat_dn8 = assign10300_e13043_d_n8;
        var_cfgsat_dn9 = assign10300_e13043_d_n9;
        var_cfgsat_dn10 = assign10300_e13043_d_n10;
        var_cfgsat_dn11 = assign10300_e13043_d_n11;
        var_cfgsat_dn13 = assign10300_e13043_d_n13;
        var_cfgsat_dn14 = assign10300_e13043_d_n14;
        var_cfgsat_rv = 0.0;

        let (assign10310_e13050, assign10310_e13050_d_n0, assign10310_e13050_d_n2, assign10310_e13050_d_n3, assign10310_e13050_d_n4, assign10310_e13050_d_n5, assign10310_e13050_d_n6, assign10310_e13050_d_n7, assign10310_e13050_d_n8, assign10310_e13050_d_n9, assign10310_e13050_d_n10, assign10310_e13050_d_n11, assign10310_e13050_d_n13, assign10310_e13050_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delta, var_delta_dn0, var_delta_dn2, var_delta_dn3, var_delta_dn4, var_delta_dn5, var_delta_dn6, var_delta_dn7, var_delta_dn8, var_delta_dn9, var_delta_dn10, var_delta_dn11, var_delta_dn13, var_delta_dn14,)
    }
};
        var_delta = assign10310_e13050;
        var_delta_dn0 = assign10310_e13050_d_n0;
        var_delta_dn2 = assign10310_e13050_d_n2;
        var_delta_dn3 = assign10310_e13050_d_n3;
        var_delta_dn4 = assign10310_e13050_d_n4;
        var_delta_dn5 = assign10310_e13050_d_n5;
        var_delta_dn6 = assign10310_e13050_d_n6;
        var_delta_dn7 = assign10310_e13050_d_n7;
        var_delta_dn8 = assign10310_e13050_d_n8;
        var_delta_dn9 = assign10310_e13050_d_n9;
        var_delta_dn10 = assign10310_e13050_d_n10;
        var_delta_dn11 = assign10310_e13050_d_n11;
        var_delta_dn13 = assign10310_e13050_d_n13;
        var_delta_dn14 = assign10310_e13050_d_n14;
        var_delta_rv = 0.0;

        let (assign10320_e13061, assign10320_e13061_d_n0, assign10320_e13061_d_n2, assign10320_e13061_d_n3, assign10320_e13061_d_n4, assign10320_e13061_d_n5, assign10320_e13061_d_n6, assign10320_e13061_d_n7, assign10320_e13061_d_n8, assign10320_e13061_d_n9, assign10320_e13061_d_n10, assign10320_e13061_d_n11, assign10320_e13061_d_n13, assign10320_e13061_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10320_e13057: f64 = (var_cfgsat - var_cfglog);
        let assign10320_e13059: f64 = (assign10320_e13057 - var_delta);
        (assign10320_e13059, (var_cfgsat_dn0 - var_delta_dn0), (var_cfgsat_dn2 - var_delta_dn2), (var_cfgsat_dn3 - var_delta_dn3), (var_cfgsat_dn4 - var_delta_dn4), (var_cfgsat_dn5 - var_delta_dn5), (var_cfgsat_dn6 - var_delta_dn6), (var_cfgsat_dn7 - var_delta_dn7), (var_cfgsat_dn8 - var_delta_dn8), (var_cfgsat_dn9 - var_delta_dn9), (var_cfgsat_dn10 - var_delta_dn10), (var_cfgsat_dn11 - var_delta_dn11), (var_cfgsat_dn13 - var_delta_dn13), (var_cfgsat_dn14 - var_delta_dn14),)
    } else {
        (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
    }
};
        var_tt1 = assign10320_e13061;
        var_tt1_dn0 = assign10320_e13061_d_n0;
        var_tt1_dn2 = assign10320_e13061_d_n2;
        var_tt1_dn3 = assign10320_e13061_d_n3;
        var_tt1_dn4 = assign10320_e13061_d_n4;
        var_tt1_dn5 = assign10320_e13061_d_n5;
        var_tt1_dn6 = assign10320_e13061_d_n6;
        var_tt1_dn7 = assign10320_e13061_d_n7;
        var_tt1_dn8 = assign10320_e13061_d_n8;
        var_tt1_dn9 = assign10320_e13061_d_n9;
        var_tt1_dn10 = assign10320_e13061_d_n10;
        var_tt1_dn11 = assign10320_e13061_d_n11;
        var_tt1_dn13 = assign10320_e13061_d_n13;
        var_tt1_dn14 = assign10320_e13061_d_n14;
        var_tt1_rv = 0.0;

        let (assign10330_e13085, assign10330_e13085_d_n0, assign10330_e13085_d_n2, assign10330_e13085_d_n3, assign10330_e13085_d_n4, assign10330_e13085_d_n5, assign10330_e13085_d_n6, assign10330_e13085_d_n7, assign10330_e13085_d_n8, assign10330_e13085_d_n9, assign10330_e13085_d_n10, assign10330_e13085_d_n11, assign10330_e13085_d_n13, assign10330_e13085_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10330_e13072: f64 = (var_tt1 * var_tt1);
        let assign10330_e13075: f64 = (4.0 * var_delta);
        let assign10330_e13077: f64 = (assign10330_e13075 * var_cfgsat);
        let assign10330_e13078: f64 = (assign10330_e13072 + assign10330_e13077);
        let assign10330_e13079: f64 = (assign10330_e13078).sqrt();
        let assign10330_e13080: f64 = (var_tt1 + assign10330_e13079);
        let assign10330_e13081: f64 = (0.5 * assign10330_e13080);
        let assign10330_e13082: f64 = (var_cfgsat - assign10330_e13081);
        let assign10330_e13083: f64 = (p.p3 * assign10330_e13082);
        (assign10330_e13083, (p.p3 * (var_cfgsat_dn0 - (0.5 * (var_tt1_dn0 + ((((var_tt1_dn0 * var_tt1) + (var_tt1 * var_tt1_dn0)) + (((4.0 * var_delta_dn0) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn0))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn2 - (0.5 * (var_tt1_dn2 + ((((var_tt1_dn2 * var_tt1) + (var_tt1 * var_tt1_dn2)) + (((4.0 * var_delta_dn2) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn2))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn3 - (0.5 * (var_tt1_dn3 + ((((var_tt1_dn3 * var_tt1) + (var_tt1 * var_tt1_dn3)) + (((4.0 * var_delta_dn3) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn3))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn4 - (0.5 * (var_tt1_dn4 + ((((var_tt1_dn4 * var_tt1) + (var_tt1 * var_tt1_dn4)) + (((4.0 * var_delta_dn4) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn4))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn5 - (0.5 * (var_tt1_dn5 + ((((var_tt1_dn5 * var_tt1) + (var_tt1 * var_tt1_dn5)) + (((4.0 * var_delta_dn5) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn5))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn6 - (0.5 * (var_tt1_dn6 + ((((var_tt1_dn6 * var_tt1) + (var_tt1 * var_tt1_dn6)) + (((4.0 * var_delta_dn6) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn6))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn7 - (0.5 * (var_tt1_dn7 + ((((var_tt1_dn7 * var_tt1) + (var_tt1 * var_tt1_dn7)) + (((4.0 * var_delta_dn7) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn7))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn8 - (0.5 * (var_tt1_dn8 + ((((var_tt1_dn8 * var_tt1) + (var_tt1 * var_tt1_dn8)) + (((4.0 * var_delta_dn8) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn8))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn9 - (0.5 * (var_tt1_dn9 + ((((var_tt1_dn9 * var_tt1) + (var_tt1 * var_tt1_dn9)) + (((4.0 * var_delta_dn9) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn9))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn10 - (0.5 * (var_tt1_dn10 + ((((var_tt1_dn10 * var_tt1) + (var_tt1 * var_tt1_dn10)) + (((4.0 * var_delta_dn10) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn10))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn11 - (0.5 * (var_tt1_dn11 + ((((var_tt1_dn11 * var_tt1) + (var_tt1 * var_tt1_dn11)) + (((4.0 * var_delta_dn11) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn11))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn13 - (0.5 * (var_tt1_dn13 + ((((var_tt1_dn13 * var_tt1) + (var_tt1 * var_tt1_dn13)) + (((4.0 * var_delta_dn13) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn13))) / (2.0 * assign10330_e13079)))))), (p.p3 * (var_cfgsat_dn14 - (0.5 * (var_tt1_dn14 + ((((var_tt1_dn14 * var_tt1) + (var_tt1 * var_tt1_dn14)) + (((4.0 * var_delta_dn14) * var_cfgsat) + (assign10330_e13075 * var_cfgsat_dn14))) / (2.0 * assign10330_e13079)))))),)
    } else {
        (var_cfg, var_cfg_dn0, var_cfg_dn2, var_cfg_dn3, var_cfg_dn4, var_cfg_dn5, var_cfg_dn6, var_cfg_dn7, var_cfg_dn8, var_cfg_dn9, var_cfg_dn10, var_cfg_dn11, var_cfg_dn13, var_cfg_dn14,)
    }
};
        var_cfg = assign10330_e13085;
        var_cfg_dn0 = assign10330_e13085_d_n0;
        var_cfg_dn2 = assign10330_e13085_d_n2;
        var_cfg_dn3 = assign10330_e13085_d_n3;
        var_cfg_dn4 = assign10330_e13085_d_n4;
        var_cfg_dn5 = assign10330_e13085_d_n5;
        var_cfg_dn6 = assign10330_e13085_d_n6;
        var_cfg_dn7 = assign10330_e13085_d_n7;
        var_cfg_dn8 = assign10330_e13085_d_n8;
        var_cfg_dn9 = assign10330_e13085_d_n9;
        var_cfg_dn10 = assign10330_e13085_d_n10;
        var_cfg_dn11 = assign10330_e13085_d_n11;
        var_cfg_dn13 = assign10330_e13085_d_n13;
        var_cfg_dn14 = assign10330_e13085_d_n14;
        var_cfg_rv = 0.0;

        let (assign10340_e13094, assign10340_e13094_d_n0, assign10340_e13094_d_n2, assign10340_e13094_d_n3, assign10340_e13094_d_n4, assign10340_e13094_d_n5, assign10340_e13094_d_n6, assign10340_e13094_d_n7, assign10340_e13094_d_n8, assign10340_e13094_d_n9, assign10340_e13094_d_n10, assign10340_e13094_d_n11, assign10340_e13094_d_n13, assign10340_e13094_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard206 == 0.0)) {
        let assign10340_e13092: f64 = (var_ccg + var_cfg);
        (assign10340_e13092, (var_ccg_dn0 + var_cfg_dn0), (var_ccg_dn2 + var_cfg_dn2), (var_ccg_dn3 + var_cfg_dn3), (var_ccg_dn4 + var_cfg_dn4), (var_ccg_dn5 + var_cfg_dn5), (var_ccg_dn6 + var_cfg_dn6), (var_ccg_dn7 + var_cfg_dn7), (var_ccg_dn8 + var_cfg_dn8), (var_ccg_dn9 + var_cfg_dn9), (var_ccg_dn10 + var_cfg_dn10), (var_ccg_dn11 + var_cfg_dn11), (var_ccg_dn13 + var_cfg_dn13), (var_ccg_dn14 + var_cfg_dn14),)
    } else {
        (var_cgg_top, var_cgg_top_dn0, var_cgg_top_dn2, var_cgg_top_dn3, var_cgg_top_dn4, var_cgg_top_dn5, var_cgg_top_dn6, var_cgg_top_dn7, var_cgg_top_dn8, var_cgg_top_dn9, var_cgg_top_dn10, var_cgg_top_dn11, var_cgg_top_dn13, var_cgg_top_dn14,)
    }
};
        var_cgg_top = assign10340_e13094;
        var_cgg_top_dn0 = assign10340_e13094_d_n0;
        var_cgg_top_dn2 = assign10340_e13094_d_n2;
        var_cgg_top_dn3 = assign10340_e13094_d_n3;
        var_cgg_top_dn4 = assign10340_e13094_d_n4;
        var_cgg_top_dn5 = assign10340_e13094_d_n5;
        var_cgg_top_dn6 = assign10340_e13094_d_n6;
        var_cgg_top_dn7 = assign10340_e13094_d_n7;
        var_cgg_top_dn8 = assign10340_e13094_d_n8;
        var_cgg_top_dn9 = assign10340_e13094_d_n9;
        var_cgg_top_dn10 = assign10340_e13094_d_n10;
        var_cgg_top_dn11 = assign10340_e13094_d_n11;
        var_cgg_top_dn13 = assign10340_e13094_d_n13;
        var_cgg_top_dn14 = assign10340_e13094_d_n14;
        var_cgg_top_rv = 0.0;

        let assign10350_e13097: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        var_guard208 = assign10350_e13097;
        var_guard208_rv = 0.0;

        let (assign10360_e13111,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10360_e13105: f64 = (var_wg + p.p90);
        let assign10360_e13106: f64 = (0.2 * assign10360_e13105);
        let assign10360_e13108: f64 = (assign10360_e13106 / var_trsd);
        let assign10360_e13109: f64 = (2.3 + assign10360_e13108);
        (assign10360_e13109,)
    } else {
        (var_hr,)
    }
};
        var_hr = assign10360_e13111;
        var_hr_rv = 0.0;

        let (assign10370_e13117,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        (1.05,)
    } else {
        (var_lr,)
    }
};
        var_lr = assign10370_e13117;
        var_lr_rv = 0.0;

        let (assign10380_e13128,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10380_e13123: f64 = (var_wg + p.p90);
        let assign10380_e13125: f64 = (assign10380_e13123 - var_trsd);
        let assign10380_e13126: f64 = (assign10380_e13125).abs();
        (assign10380_e13126,)
    } else {
        (var_hgdelta,)
    }
};
        var_hgdelta = assign10380_e13128;
        var_hgdelta_rv = 0.0;

        let (assign10390_e13136,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10390_e13134: f64 = (p.p1087 * var_lr);
        (assign10390_e13134,)
    } else {
        (var_lmax,)
    }
};
        var_lmax = assign10390_e13136;
        var_lmax_rv = 0.0;

        let (assign10400_e13146,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10400_e13143: f64 = (var_wg + p.p90);
        let assign10400_e13144: f64 = (var_trsd).min(assign10400_e13143);
        (assign10400_e13144,)
    } else {
        (var_y,)
    }
};
        var_y = assign10400_e13146;
        var_y_rv = 0.0;

        let (assign10410_e13156,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10410_e13153: f64 = (var_hr + 1.0);
        let assign10410_e13154: f64 = (p.p1087 / assign10410_e13153);
        (assign10410_e13154,)
    } else {
        (var_x,)
    }
};
        var_x = assign10410_e13156;
        var_x_rv = 0.0;

        let (assign10420_e13162,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        (1700000000000.0,)
    } else {
        (var_cnon,)
    }
};
        var_cnon = assign10420_e13162;
        var_cnon_rv = 0.0;

        let (assign10430_e13174,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10430_e13169: f64 = (var_y - var_x);
        let assign10430_e13170: f64 = (var_epssp * assign10430_e13169);
        let assign10430_e13172: f64 = (assign10430_e13170 / p.p1087);
        (assign10430_e13172,)
    } else {
        (var_ccgsat,)
    }
};
        var_ccgsat = assign10430_e13174;
        var_ccgsat_rv = 0.0;

        let (assign10440_e13182, assign10440_e13182_d_n0, assign10440_e13182_d_n2, assign10440_e13182_d_n3, assign10440_e13182_d_n4, assign10440_e13182_d_n5, assign10440_e13182_d_n6, assign10440_e13182_d_n7, assign10440_e13182_d_n8, assign10440_e13182_d_n9, assign10440_e13182_d_n10, assign10440_e13182_d_n11, assign10440_e13182_d_n13, assign10440_e13182_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10440_e13180: f64 = (var_cnon * var_ccgsat);
        (assign10440_e13180, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
    }
};
        var_tt1 = assign10440_e13182;
        var_tt1_dn0 = assign10440_e13182_d_n0;
        var_tt1_dn2 = assign10440_e13182_d_n2;
        var_tt1_dn3 = assign10440_e13182_d_n3;
        var_tt1_dn4 = assign10440_e13182_d_n4;
        var_tt1_dn5 = assign10440_e13182_d_n5;
        var_tt1_dn6 = assign10440_e13182_d_n6;
        var_tt1_dn7 = assign10440_e13182_d_n7;
        var_tt1_dn8 = assign10440_e13182_d_n8;
        var_tt1_dn9 = assign10440_e13182_d_n9;
        var_tt1_dn10 = assign10440_e13182_d_n10;
        var_tt1_dn11 = assign10440_e13182_d_n11;
        var_tt1_dn13 = assign10440_e13182_d_n13;
        var_tt1_dn14 = assign10440_e13182_d_n14;
        var_tt1_rv = 0.0;

        let assign10450_e13185: f64 = if var_tt1 > 80.0 { 1.0 } else { 0.0 };
        var_guard209 = assign10450_e13185;
        var_guard209_rv = 0.0;

        let (assign10460_e13193, assign10460_e13193_d_n0, assign10460_e13193_d_n2, assign10460_e13193_d_n3, assign10460_e13193_d_n4, assign10460_e13193_d_n5, assign10460_e13193_d_n6, assign10460_e13193_d_n7, assign10460_e13193_d_n8, assign10460_e13193_d_n9, assign10460_e13193_d_n10, assign10460_e13193_d_n11, assign10460_e13193_d_n13, assign10460_e13193_d_n14,) = {
    if (((var_guard205 != 0.0) && (var_guard208 != 0.0)) && (var_guard209 != 0.0)) {
        (var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ccg1, var_ccg1_dn0, var_ccg1_dn2, var_ccg1_dn3, var_ccg1_dn4, var_ccg1_dn5, var_ccg1_dn6, var_ccg1_dn7, var_ccg1_dn8, var_ccg1_dn9, var_ccg1_dn10, var_ccg1_dn11, var_ccg1_dn13, var_ccg1_dn14,)
    }
};
        var_ccg1 = assign10460_e13193;
        var_ccg1_dn0 = assign10460_e13193_d_n0;
        var_ccg1_dn2 = assign10460_e13193_d_n2;
        var_ccg1_dn3 = assign10460_e13193_d_n3;
        var_ccg1_dn4 = assign10460_e13193_d_n4;
        var_ccg1_dn5 = assign10460_e13193_d_n5;
        var_ccg1_dn6 = assign10460_e13193_d_n6;
        var_ccg1_dn7 = assign10460_e13193_d_n7;
        var_ccg1_dn8 = assign10460_e13193_d_n8;
        var_ccg1_dn9 = assign10460_e13193_d_n9;
        var_ccg1_dn10 = assign10460_e13193_d_n10;
        var_ccg1_dn11 = assign10460_e13193_d_n11;
        var_ccg1_dn13 = assign10460_e13193_d_n13;
        var_ccg1_dn14 = assign10460_e13193_d_n14;
        var_ccg1_rv = 0.0;

        let (assign10470_e13239, assign10470_e13239_d_n0, assign10470_e13239_d_n2, assign10470_e13239_d_n3, assign10470_e13239_d_n4, assign10470_e13239_d_n5, assign10470_e13239_d_n6, assign10470_e13239_d_n7, assign10470_e13239_d_n8, assign10470_e13239_d_n9, assign10470_e13239_d_n10, assign10470_e13239_d_n11, assign10470_e13239_d_n13, assign10470_e13239_d_n14,) = {
    if (((var_guard205 != 0.0) && (var_guard208 != 0.0)) && (var_guard209 == 0.0)) {
        let assign10470_e13202: f64 = (1.0 / var_cnon);
        let assign10470_e13209: f64 = (-37.0);
        let (assign10470_e13236, assign10470_e13236_d_n0, assign10470_e13236_d_n2, assign10470_e13236_d_n3, assign10470_e13236_d_n4, assign10470_e13236_d_n5, assign10470_e13236_d_n6, assign10470_e13236_d_n7, assign10470_e13236_d_n8, assign10470_e13236_d_n9, assign10470_e13236_d_n10, assign10470_e13236_d_n11, assign10470_e13236_d_n13, assign10470_e13236_d_n14,) = {
            if ((!(var_tt1 > 37.0)) && (!(var_tt1 < assign10470_e13209))) {
                let assign10470_e13215: f64 = (var_tt1).exp();
                let assign10470_e13216: f64 = (1.0 + assign10470_e13215);
                let assign10470_e13217: f64 = (assign10470_e13216).ln();
                (assign10470_e13217, ((assign10470_e13215 * var_tt1_dn0) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn2) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn3) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn4) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn5) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn6) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn7) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn8) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn9) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn10) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn11) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn13) / assign10470_e13216), ((assign10470_e13215 * var_tt1_dn14) / assign10470_e13216),)
            } else {
                let assign10470_e13224: f64 = (-37.0);
                let (assign10470_e13235, assign10470_e13235_d_n0, assign10470_e13235_d_n2, assign10470_e13235_d_n3, assign10470_e13235_d_n4, assign10470_e13235_d_n5, assign10470_e13235_d_n6, assign10470_e13235_d_n7, assign10470_e13235_d_n8, assign10470_e13235_d_n9, assign10470_e13235_d_n10, assign10470_e13235_d_n11, assign10470_e13235_d_n13, assign10470_e13235_d_n14,) = {
                    if ((!(var_tt1 > 37.0)) && (var_tt1 < assign10470_e13224)) {
                        let assign10470_e13228: f64 = (var_tt1).exp();
                        (assign10470_e13228, (assign10470_e13228 * var_tt1_dn0), (assign10470_e13228 * var_tt1_dn2), (assign10470_e13228 * var_tt1_dn3), (assign10470_e13228 * var_tt1_dn4), (assign10470_e13228 * var_tt1_dn5), (assign10470_e13228 * var_tt1_dn6), (assign10470_e13228 * var_tt1_dn7), (assign10470_e13228 * var_tt1_dn8), (assign10470_e13228 * var_tt1_dn9), (assign10470_e13228 * var_tt1_dn10), (assign10470_e13228 * var_tt1_dn11), (assign10470_e13228 * var_tt1_dn13), (assign10470_e13228 * var_tt1_dn14),)
                    } else {
                        let (assign10470_e13234, assign10470_e13234_d_n0, assign10470_e13234_d_n2, assign10470_e13234_d_n3, assign10470_e13234_d_n4, assign10470_e13234_d_n5, assign10470_e13234_d_n6, assign10470_e13234_d_n7, assign10470_e13234_d_n8, assign10470_e13234_d_n9, assign10470_e13234_d_n10, assign10470_e13234_d_n11, assign10470_e13234_d_n13, assign10470_e13234_d_n14,) = {
                            if (var_tt1 > 37.0) {
                                (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign10470_e13234, assign10470_e13234_d_n0, assign10470_e13234_d_n2, assign10470_e13234_d_n3, assign10470_e13234_d_n4, assign10470_e13234_d_n5, assign10470_e13234_d_n6, assign10470_e13234_d_n7, assign10470_e13234_d_n8, assign10470_e13234_d_n9, assign10470_e13234_d_n10, assign10470_e13234_d_n11, assign10470_e13234_d_n13, assign10470_e13234_d_n14,)
                    }
                };
                (assign10470_e13235, assign10470_e13235_d_n0, assign10470_e13235_d_n2, assign10470_e13235_d_n3, assign10470_e13235_d_n4, assign10470_e13235_d_n5, assign10470_e13235_d_n6, assign10470_e13235_d_n7, assign10470_e13235_d_n8, assign10470_e13235_d_n9, assign10470_e13235_d_n10, assign10470_e13235_d_n11, assign10470_e13235_d_n13, assign10470_e13235_d_n14,)
            }
        };
        let assign10470_e13237: f64 = (assign10470_e13202 * assign10470_e13236);
        (assign10470_e13237, (assign10470_e13202 * assign10470_e13236_d_n0), (assign10470_e13202 * assign10470_e13236_d_n2), (assign10470_e13202 * assign10470_e13236_d_n3), (assign10470_e13202 * assign10470_e13236_d_n4), (assign10470_e13202 * assign10470_e13236_d_n5), (assign10470_e13202 * assign10470_e13236_d_n6), (assign10470_e13202 * assign10470_e13236_d_n7), (assign10470_e13202 * assign10470_e13236_d_n8), (assign10470_e13202 * assign10470_e13236_d_n9), (assign10470_e13202 * assign10470_e13236_d_n10), (assign10470_e13202 * assign10470_e13236_d_n11), (assign10470_e13202 * assign10470_e13236_d_n13), (assign10470_e13202 * assign10470_e13236_d_n14),)
    } else {
        (var_ccg1, var_ccg1_dn0, var_ccg1_dn2, var_ccg1_dn3, var_ccg1_dn4, var_ccg1_dn5, var_ccg1_dn6, var_ccg1_dn7, var_ccg1_dn8, var_ccg1_dn9, var_ccg1_dn10, var_ccg1_dn11, var_ccg1_dn13, var_ccg1_dn14,)
    }
};
        var_ccg1 = assign10470_e13239;
        var_ccg1_dn0 = assign10470_e13239_d_n0;
        var_ccg1_dn2 = assign10470_e13239_d_n2;
        var_ccg1_dn3 = assign10470_e13239_d_n3;
        var_ccg1_dn4 = assign10470_e13239_d_n4;
        var_ccg1_dn5 = assign10470_e13239_d_n5;
        var_ccg1_dn6 = assign10470_e13239_d_n6;
        var_ccg1_dn7 = assign10470_e13239_d_n7;
        var_ccg1_dn8 = assign10470_e13239_d_n8;
        var_ccg1_dn9 = assign10470_e13239_d_n9;
        var_ccg1_dn10 = assign10470_e13239_d_n10;
        var_ccg1_dn11 = assign10470_e13239_d_n11;
        var_ccg1_dn13 = assign10470_e13239_d_n13;
        var_ccg1_dn14 = assign10470_e13239_d_n14;
        var_ccg1_rv = 0.0;

        let (assign10480_e13257,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10480_e13247: f64 = (var_wg + p.p90);
        let assign10480_e13248: f64 = (var_trsd / assign10480_e13247);
        let assign10480_e13251: f64 = (var_wg + p.p90);
        let assign10480_e13253: f64 = (assign10480_e13251 / var_trsd);
        let assign10480_e13254: f64 = (assign10480_e13248).min(assign10480_e13253);
        let assign10480_e13255: f64 = (0.5 * assign10480_e13254);
        (assign10480_e13255,)
    } else {
        (var_r1cf,)
    }
};
        var_r1cf = assign10480_e13257;
        var_r1cf_rv = 0.0;

        let (assign10490_e13265,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10490_e13263: f64 = (var_hgdelta * var_r1cf);
        (assign10490_e13263,)
    } else {
        (var_rcf,)
    }
};
        var_rcf = assign10490_e13265;
        var_rcf_rv = 0.0;

        let (assign10500_e13314,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10500_e13271: f64 = (var_epssp * 2.0);
        let assign10500_e13273: f64 = (assign10500_e13271 / 3.141592653589793);
        let assign10500_e13277: f64 = (0.5 * 3.141592653589793);
        let assign10500_e13279: f64 = (assign10500_e13277 * var_rcf);
        let assign10500_e13280: f64 = (p.p1087 + assign10500_e13279);
        let assign10500_e13282: f64 = (assign10500_e13280 / p.p1087);
        let (assign10500_e13311,) = {
            if (!(assign10500_e13282 > 1e-38)) {
                let assign10500_e13287: f64 = (-87.498233534);
                (assign10500_e13287,)
            } else {
                let assign10500_e13291: f64 = (0.5 * 3.141592653589793);
                let assign10500_e13293: f64 = (assign10500_e13291 * var_rcf);
                let assign10500_e13294: f64 = (p.p1087 + assign10500_e13293);
                let assign10500_e13296: f64 = (assign10500_e13294 / p.p1087);
                let (assign10500_e13310,) = {
                    if (assign10500_e13296 > 1e-38) {
                        let assign10500_e13302: f64 = (0.5 * 3.141592653589793);
                        let assign10500_e13304: f64 = (assign10500_e13302 * var_rcf);
                        let assign10500_e13305: f64 = (p.p1087 + assign10500_e13304);
                        let assign10500_e13307: f64 = (assign10500_e13305 / p.p1087);
                        let assign10500_e13308: f64 = (assign10500_e13307).ln();
                        (assign10500_e13308,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10500_e13310,)
            }
        };
        let assign10500_e13312: f64 = (assign10500_e13273 * assign10500_e13311);
        (assign10500_e13312,)
    } else {
        (var_ccg2,)
    }
};
        var_ccg2 = assign10500_e13314;
        var_ccg2_rv = 0.0;

        *var_ccg1_slot = var_ccg1;
        *var_ccg1_dn0_slot = var_ccg1_dn0;
        *var_ccg1_dn10_slot = var_ccg1_dn10;
        *var_ccg1_dn11_slot = var_ccg1_dn11;
        *var_ccg1_dn13_slot = var_ccg1_dn13;
        *var_ccg1_dn14_slot = var_ccg1_dn14;
        *var_ccg1_dn2_slot = var_ccg1_dn2;
        *var_ccg1_dn3_slot = var_ccg1_dn3;
        *var_ccg1_dn4_slot = var_ccg1_dn4;
        *var_ccg1_dn5_slot = var_ccg1_dn5;
        *var_ccg1_dn6_slot = var_ccg1_dn6;
        *var_ccg1_dn7_slot = var_ccg1_dn7;
        *var_ccg1_dn8_slot = var_ccg1_dn8;
        *var_ccg1_dn9_slot = var_ccg1_dn9;
        *var_ccg1_rv_slot = var_ccg1_rv;
        *var_ccg2_slot = var_ccg2;
        *var_ccg2_rv_slot = var_ccg2_rv;
        *var_ccgsat_slot = var_ccgsat;
        *var_ccgsat_rv_slot = var_ccgsat_rv;
        *var_cfg_slot = var_cfg;
        *var_cfg_dn0_slot = var_cfg_dn0;
        *var_cfg_dn10_slot = var_cfg_dn10;
        *var_cfg_dn11_slot = var_cfg_dn11;
        *var_cfg_dn13_slot = var_cfg_dn13;
        *var_cfg_dn14_slot = var_cfg_dn14;
        *var_cfg_dn2_slot = var_cfg_dn2;
        *var_cfg_dn3_slot = var_cfg_dn3;
        *var_cfg_dn4_slot = var_cfg_dn4;
        *var_cfg_dn5_slot = var_cfg_dn5;
        *var_cfg_dn6_slot = var_cfg_dn6;
        *var_cfg_dn7_slot = var_cfg_dn7;
        *var_cfg_dn8_slot = var_cfg_dn8;
        *var_cfg_dn9_slot = var_cfg_dn9;
        *var_cfg_rv_slot = var_cfg_rv;
        *var_cfgsat_slot = var_cfgsat;
        *var_cfgsat_dn0_slot = var_cfgsat_dn0;
        *var_cfgsat_dn10_slot = var_cfgsat_dn10;
        *var_cfgsat_dn11_slot = var_cfgsat_dn11;
        *var_cfgsat_dn13_slot = var_cfgsat_dn13;
        *var_cfgsat_dn14_slot = var_cfgsat_dn14;
        *var_cfgsat_dn2_slot = var_cfgsat_dn2;
        *var_cfgsat_dn3_slot = var_cfgsat_dn3;
        *var_cfgsat_dn4_slot = var_cfgsat_dn4;
        *var_cfgsat_dn5_slot = var_cfgsat_dn5;
        *var_cfgsat_dn6_slot = var_cfgsat_dn6;
        *var_cfgsat_dn7_slot = var_cfgsat_dn7;
        *var_cfgsat_dn8_slot = var_cfgsat_dn8;
        *var_cfgsat_dn9_slot = var_cfgsat_dn9;
        *var_cfgsat_rv_slot = var_cfgsat_rv;
        *var_cgg_top_slot = var_cgg_top;
        *var_cgg_top_dn0_slot = var_cgg_top_dn0;
        *var_cgg_top_dn10_slot = var_cgg_top_dn10;
        *var_cgg_top_dn11_slot = var_cgg_top_dn11;
        *var_cgg_top_dn13_slot = var_cgg_top_dn13;
        *var_cgg_top_dn14_slot = var_cgg_top_dn14;
        *var_cgg_top_dn2_slot = var_cgg_top_dn2;
        *var_cgg_top_dn3_slot = var_cgg_top_dn3;
        *var_cgg_top_dn4_slot = var_cgg_top_dn4;
        *var_cgg_top_dn5_slot = var_cgg_top_dn5;
        *var_cgg_top_dn6_slot = var_cgg_top_dn6;
        *var_cgg_top_dn7_slot = var_cgg_top_dn7;
        *var_cgg_top_dn8_slot = var_cgg_top_dn8;
        *var_cgg_top_dn9_slot = var_cgg_top_dn9;
        *var_cgg_top_rv_slot = var_cgg_top_rv;
        *var_cnon_slot = var_cnon;
        *var_cnon_rv_slot = var_cnon_rv;
        *var_delta_slot = var_delta;
        *var_delta_dn0_slot = var_delta_dn0;
        *var_delta_dn10_slot = var_delta_dn10;
        *var_delta_dn11_slot = var_delta_dn11;
        *var_delta_dn13_slot = var_delta_dn13;
        *var_delta_dn14_slot = var_delta_dn14;
        *var_delta_dn2_slot = var_delta_dn2;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_delta_dn9_slot = var_delta_dn9;
        *var_delta_rv_slot = var_delta_rv;
        *var_guard208_slot = var_guard208;
        *var_guard208_rv_slot = var_guard208_rv;
        *var_guard209_slot = var_guard209;
        *var_guard209_rv_slot = var_guard209_rv;
        *var_hgdelta_slot = var_hgdelta;
        *var_hgdelta_rv_slot = var_hgdelta_rv;
        *var_hr_slot = var_hr;
        *var_hr_rv_slot = var_hr_rv;
        *var_lmax_slot = var_lmax;
        *var_lmax_rv_slot = var_lmax_rv;
        *var_lr_slot = var_lr;
        *var_lr_rv_slot = var_lr_rv;
        *var_r1cf_slot = var_r1cf;
        *var_r1cf_rv_slot = var_r1cf_rv;
        *var_rcf_slot = var_rcf;
        *var_rcf_rv_slot = var_rcf_rv;
        *var_tt0_slot = var_tt0;
        *var_tt0_rv_slot = var_tt0_rv;
        *var_tt1_slot = var_tt1;
        *var_tt1_dn0_slot = var_tt1_dn0;
        *var_tt1_dn10_slot = var_tt1_dn10;
        *var_tt1_dn11_slot = var_tt1_dn11;
        *var_tt1_dn13_slot = var_tt1_dn13;
        *var_tt1_dn14_slot = var_tt1_dn14;
        *var_tt1_dn2_slot = var_tt1_dn2;
        *var_tt1_dn3_slot = var_tt1_dn3;
        *var_tt1_dn4_slot = var_tt1_dn4;
        *var_tt1_dn5_slot = var_tt1_dn5;
        *var_tt1_dn6_slot = var_tt1_dn6;
        *var_tt1_dn7_slot = var_tt1_dn7;
        *var_tt1_dn8_slot = var_tt1_dn8;
        *var_tt1_dn9_slot = var_tt1_dn9;
        *var_tt1_rv_slot = var_tt1_rv;
        *var_tt2_slot = var_tt2;
        *var_tt2_rv_slot = var_tt2_rv;
        *var_x_slot = var_x;
        *var_x_rv_slot = var_x_rv;
        *var_y_slot = var_y;
        *var_y_rv_slot = var_y_rv;
    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        var_ccg2: f64,
        var_epssp: f64,
        var_guard205: f64,
        var_guard208: f64,
        var_trsd: f64,
        var_wg: f64,
        var_c1_slot: &mut f64,
        var_c1_rv_slot: &mut f64,
        var_c2_slot: &mut f64,
        var_c2_rv_slot: &mut f64,
        var_c3_slot: &mut f64,
        var_c3_rv_slot: &mut f64,
        var_ccg_slot: &mut f64,
        var_ccg1_slot: &mut f64,
        var_ccg1_dn0_slot: &mut f64,
        var_ccg1_dn10_slot: &mut f64,
        var_ccg1_dn11_slot: &mut f64,
        var_ccg1_dn13_slot: &mut f64,
        var_ccg1_dn14_slot: &mut f64,
        var_ccg1_dn2_slot: &mut f64,
        var_ccg1_dn3_slot: &mut f64,
        var_ccg1_dn4_slot: &mut f64,
        var_ccg1_dn5_slot: &mut f64,
        var_ccg1_dn6_slot: &mut f64,
        var_ccg1_dn7_slot: &mut f64,
        var_ccg1_dn8_slot: &mut f64,
        var_ccg1_dn9_slot: &mut f64,
        var_ccg1_rv_slot: &mut f64,
        var_ccg_dn0_slot: &mut f64,
        var_ccg_dn10_slot: &mut f64,
        var_ccg_dn11_slot: &mut f64,
        var_ccg_dn13_slot: &mut f64,
        var_ccg_dn14_slot: &mut f64,
        var_ccg_dn2_slot: &mut f64,
        var_ccg_dn3_slot: &mut f64,
        var_ccg_dn4_slot: &mut f64,
        var_ccg_dn5_slot: &mut f64,
        var_ccg_dn6_slot: &mut f64,
        var_ccg_dn7_slot: &mut f64,
        var_ccg_dn8_slot: &mut f64,
        var_ccg_dn9_slot: &mut f64,
        var_ccg_rv_slot: &mut f64,
        var_ccgsat_slot: &mut f64,
        var_ccgsat_rv_slot: &mut f64,
        var_cfg_slot: &mut f64,
        var_cfg_dn0_slot: &mut f64,
        var_cfg_dn10_slot: &mut f64,
        var_cfg_dn11_slot: &mut f64,
        var_cfg_dn13_slot: &mut f64,
        var_cfg_dn14_slot: &mut f64,
        var_cfg_dn2_slot: &mut f64,
        var_cfg_dn3_slot: &mut f64,
        var_cfg_dn4_slot: &mut f64,
        var_cfg_dn5_slot: &mut f64,
        var_cfg_dn6_slot: &mut f64,
        var_cfg_dn7_slot: &mut f64,
        var_cfg_dn8_slot: &mut f64,
        var_cfg_dn9_slot: &mut f64,
        var_cfg_rv_slot: &mut f64,
        var_cfglog_slot: &mut f64,
        var_cfglog_rv_slot: &mut f64,
        var_cfgsat_slot: &mut f64,
        var_cfgsat_dn0_slot: &mut f64,
        var_cfgsat_dn10_slot: &mut f64,
        var_cfgsat_dn11_slot: &mut f64,
        var_cfgsat_dn13_slot: &mut f64,
        var_cfgsat_dn14_slot: &mut f64,
        var_cfgsat_dn2_slot: &mut f64,
        var_cfgsat_dn3_slot: &mut f64,
        var_cfgsat_dn4_slot: &mut f64,
        var_cfgsat_dn5_slot: &mut f64,
        var_cfgsat_dn6_slot: &mut f64,
        var_cfgsat_dn7_slot: &mut f64,
        var_cfgsat_dn8_slot: &mut f64,
        var_cfgsat_dn9_slot: &mut f64,
        var_cfgsat_rv_slot: &mut f64,
        var_cgg_side_slot: &mut f64,
        var_cgg_side_dn0_slot: &mut f64,
        var_cgg_side_dn10_slot: &mut f64,
        var_cgg_side_dn11_slot: &mut f64,
        var_cgg_side_dn13_slot: &mut f64,
        var_cgg_side_dn14_slot: &mut f64,
        var_cgg_side_dn2_slot: &mut f64,
        var_cgg_side_dn3_slot: &mut f64,
        var_cgg_side_dn4_slot: &mut f64,
        var_cgg_side_dn5_slot: &mut f64,
        var_cgg_side_dn6_slot: &mut f64,
        var_cgg_side_dn7_slot: &mut f64,
        var_cgg_side_dn8_slot: &mut f64,
        var_cgg_side_dn9_slot: &mut f64,
        var_cgg_side_rv_slot: &mut f64,
        var_cnon_slot: &mut f64,
        var_cnon_rv_slot: &mut f64,
        var_dcf_slot: &mut f64,
        var_dcf_rv_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn0_slot: &mut f64,
        var_delta_dn10_slot: &mut f64,
        var_delta_dn11_slot: &mut f64,
        var_delta_dn13_slot: &mut f64,
        var_delta_dn14_slot: &mut f64,
        var_delta_dn2_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_delta_dn9_slot: &mut f64,
        var_delta_rv_slot: &mut f64,
        var_guard210_slot: &mut f64,
        var_guard210_rv_slot: &mut f64,
        var_hgdelta_slot: &mut f64,
        var_hgdelta_rv_slot: &mut f64,
        var_hr_slot: &mut f64,
        var_hr_rv_slot: &mut f64,
        var_lmax_slot: &mut f64,
        var_lmax_rv_slot: &mut f64,
        var_lr_slot: &mut f64,
        var_lr_rv_slot: &mut f64,
        var_tt0_slot: &mut f64,
        var_tt0_rv_slot: &mut f64,
        var_tt1_slot: &mut f64,
        var_tt1_dn0_slot: &mut f64,
        var_tt1_dn10_slot: &mut f64,
        var_tt1_dn11_slot: &mut f64,
        var_tt1_dn13_slot: &mut f64,
        var_tt1_dn14_slot: &mut f64,
        var_tt1_dn2_slot: &mut f64,
        var_tt1_dn3_slot: &mut f64,
        var_tt1_dn4_slot: &mut f64,
        var_tt1_dn5_slot: &mut f64,
        var_tt1_dn6_slot: &mut f64,
        var_tt1_dn7_slot: &mut f64,
        var_tt1_dn8_slot: &mut f64,
        var_tt1_dn9_slot: &mut f64,
        var_tt1_rv_slot: &mut f64,
        var_tt2_slot: &mut f64,
        var_tt2_rv_slot: &mut f64,
        var_x_slot: &mut f64,
        var_x_rv_slot: &mut f64,
        var_y_slot: &mut f64,
        var_y_rv_slot: &mut f64,
    ) {
        let mut var_c1: f64 = *var_c1_slot;
        let mut var_c1_rv: f64 = *var_c1_rv_slot;
        let mut var_c2: f64 = *var_c2_slot;
        let mut var_c2_rv: f64 = *var_c2_rv_slot;
        let mut var_c3: f64 = *var_c3_slot;
        let mut var_c3_rv: f64 = *var_c3_rv_slot;
        let mut var_ccg: f64 = *var_ccg_slot;
        let mut var_ccg1: f64 = *var_ccg1_slot;
        let mut var_ccg1_dn0: f64 = *var_ccg1_dn0_slot;
        let mut var_ccg1_dn10: f64 = *var_ccg1_dn10_slot;
        let mut var_ccg1_dn11: f64 = *var_ccg1_dn11_slot;
        let mut var_ccg1_dn13: f64 = *var_ccg1_dn13_slot;
        let mut var_ccg1_dn14: f64 = *var_ccg1_dn14_slot;
        let mut var_ccg1_dn2: f64 = *var_ccg1_dn2_slot;
        let mut var_ccg1_dn3: f64 = *var_ccg1_dn3_slot;
        let mut var_ccg1_dn4: f64 = *var_ccg1_dn4_slot;
        let mut var_ccg1_dn5: f64 = *var_ccg1_dn5_slot;
        let mut var_ccg1_dn6: f64 = *var_ccg1_dn6_slot;
        let mut var_ccg1_dn7: f64 = *var_ccg1_dn7_slot;
        let mut var_ccg1_dn8: f64 = *var_ccg1_dn8_slot;
        let mut var_ccg1_dn9: f64 = *var_ccg1_dn9_slot;
        let mut var_ccg1_rv: f64 = *var_ccg1_rv_slot;
        let mut var_ccg_dn0: f64 = *var_ccg_dn0_slot;
        let mut var_ccg_dn10: f64 = *var_ccg_dn10_slot;
        let mut var_ccg_dn11: f64 = *var_ccg_dn11_slot;
        let mut var_ccg_dn13: f64 = *var_ccg_dn13_slot;
        let mut var_ccg_dn14: f64 = *var_ccg_dn14_slot;
        let mut var_ccg_dn2: f64 = *var_ccg_dn2_slot;
        let mut var_ccg_dn3: f64 = *var_ccg_dn3_slot;
        let mut var_ccg_dn4: f64 = *var_ccg_dn4_slot;
        let mut var_ccg_dn5: f64 = *var_ccg_dn5_slot;
        let mut var_ccg_dn6: f64 = *var_ccg_dn6_slot;
        let mut var_ccg_dn7: f64 = *var_ccg_dn7_slot;
        let mut var_ccg_dn8: f64 = *var_ccg_dn8_slot;
        let mut var_ccg_dn9: f64 = *var_ccg_dn9_slot;
        let mut var_ccg_rv: f64 = *var_ccg_rv_slot;
        let mut var_ccgsat: f64 = *var_ccgsat_slot;
        let mut var_ccgsat_rv: f64 = *var_ccgsat_rv_slot;
        let mut var_cfg: f64 = *var_cfg_slot;
        let mut var_cfg_dn0: f64 = *var_cfg_dn0_slot;
        let mut var_cfg_dn10: f64 = *var_cfg_dn10_slot;
        let mut var_cfg_dn11: f64 = *var_cfg_dn11_slot;
        let mut var_cfg_dn13: f64 = *var_cfg_dn13_slot;
        let mut var_cfg_dn14: f64 = *var_cfg_dn14_slot;
        let mut var_cfg_dn2: f64 = *var_cfg_dn2_slot;
        let mut var_cfg_dn3: f64 = *var_cfg_dn3_slot;
        let mut var_cfg_dn4: f64 = *var_cfg_dn4_slot;
        let mut var_cfg_dn5: f64 = *var_cfg_dn5_slot;
        let mut var_cfg_dn6: f64 = *var_cfg_dn6_slot;
        let mut var_cfg_dn7: f64 = *var_cfg_dn7_slot;
        let mut var_cfg_dn8: f64 = *var_cfg_dn8_slot;
        let mut var_cfg_dn9: f64 = *var_cfg_dn9_slot;
        let mut var_cfg_rv: f64 = *var_cfg_rv_slot;
        let mut var_cfglog: f64 = *var_cfglog_slot;
        let mut var_cfglog_rv: f64 = *var_cfglog_rv_slot;
        let mut var_cfgsat: f64 = *var_cfgsat_slot;
        let mut var_cfgsat_dn0: f64 = *var_cfgsat_dn0_slot;
        let mut var_cfgsat_dn10: f64 = *var_cfgsat_dn10_slot;
        let mut var_cfgsat_dn11: f64 = *var_cfgsat_dn11_slot;
        let mut var_cfgsat_dn13: f64 = *var_cfgsat_dn13_slot;
        let mut var_cfgsat_dn14: f64 = *var_cfgsat_dn14_slot;
        let mut var_cfgsat_dn2: f64 = *var_cfgsat_dn2_slot;
        let mut var_cfgsat_dn3: f64 = *var_cfgsat_dn3_slot;
        let mut var_cfgsat_dn4: f64 = *var_cfgsat_dn4_slot;
        let mut var_cfgsat_dn5: f64 = *var_cfgsat_dn5_slot;
        let mut var_cfgsat_dn6: f64 = *var_cfgsat_dn6_slot;
        let mut var_cfgsat_dn7: f64 = *var_cfgsat_dn7_slot;
        let mut var_cfgsat_dn8: f64 = *var_cfgsat_dn8_slot;
        let mut var_cfgsat_dn9: f64 = *var_cfgsat_dn9_slot;
        let mut var_cfgsat_rv: f64 = *var_cfgsat_rv_slot;
        let mut var_cgg_side: f64 = *var_cgg_side_slot;
        let mut var_cgg_side_dn0: f64 = *var_cgg_side_dn0_slot;
        let mut var_cgg_side_dn10: f64 = *var_cgg_side_dn10_slot;
        let mut var_cgg_side_dn11: f64 = *var_cgg_side_dn11_slot;
        let mut var_cgg_side_dn13: f64 = *var_cgg_side_dn13_slot;
        let mut var_cgg_side_dn14: f64 = *var_cgg_side_dn14_slot;
        let mut var_cgg_side_dn2: f64 = *var_cgg_side_dn2_slot;
        let mut var_cgg_side_dn3: f64 = *var_cgg_side_dn3_slot;
        let mut var_cgg_side_dn4: f64 = *var_cgg_side_dn4_slot;
        let mut var_cgg_side_dn5: f64 = *var_cgg_side_dn5_slot;
        let mut var_cgg_side_dn6: f64 = *var_cgg_side_dn6_slot;
        let mut var_cgg_side_dn7: f64 = *var_cgg_side_dn7_slot;
        let mut var_cgg_side_dn8: f64 = *var_cgg_side_dn8_slot;
        let mut var_cgg_side_dn9: f64 = *var_cgg_side_dn9_slot;
        let mut var_cgg_side_rv: f64 = *var_cgg_side_rv_slot;
        let mut var_cnon: f64 = *var_cnon_slot;
        let mut var_cnon_rv: f64 = *var_cnon_rv_slot;
        let mut var_dcf: f64 = *var_dcf_slot;
        let mut var_dcf_rv: f64 = *var_dcf_rv_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn0: f64 = *var_delta_dn0_slot;
        let mut var_delta_dn10: f64 = *var_delta_dn10_slot;
        let mut var_delta_dn11: f64 = *var_delta_dn11_slot;
        let mut var_delta_dn13: f64 = *var_delta_dn13_slot;
        let mut var_delta_dn14: f64 = *var_delta_dn14_slot;
        let mut var_delta_dn2: f64 = *var_delta_dn2_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_delta_dn9: f64 = *var_delta_dn9_slot;
        let mut var_delta_rv: f64 = *var_delta_rv_slot;
        let mut var_guard210: f64 = *var_guard210_slot;
        let mut var_guard210_rv: f64 = *var_guard210_rv_slot;
        let mut var_hgdelta: f64 = *var_hgdelta_slot;
        let mut var_hgdelta_rv: f64 = *var_hgdelta_rv_slot;
        let mut var_hr: f64 = *var_hr_slot;
        let mut var_hr_rv: f64 = *var_hr_rv_slot;
        let mut var_lmax: f64 = *var_lmax_slot;
        let mut var_lmax_rv: f64 = *var_lmax_rv_slot;
        let mut var_lr: f64 = *var_lr_slot;
        let mut var_lr_rv: f64 = *var_lr_rv_slot;
        let mut var_tt0: f64 = *var_tt0_slot;
        let mut var_tt0_rv: f64 = *var_tt0_rv_slot;
        let mut var_tt1: f64 = *var_tt1_slot;
        let mut var_tt1_dn0: f64 = *var_tt1_dn0_slot;
        let mut var_tt1_dn10: f64 = *var_tt1_dn10_slot;
        let mut var_tt1_dn11: f64 = *var_tt1_dn11_slot;
        let mut var_tt1_dn13: f64 = *var_tt1_dn13_slot;
        let mut var_tt1_dn14: f64 = *var_tt1_dn14_slot;
        let mut var_tt1_dn2: f64 = *var_tt1_dn2_slot;
        let mut var_tt1_dn3: f64 = *var_tt1_dn3_slot;
        let mut var_tt1_dn4: f64 = *var_tt1_dn4_slot;
        let mut var_tt1_dn5: f64 = *var_tt1_dn5_slot;
        let mut var_tt1_dn6: f64 = *var_tt1_dn6_slot;
        let mut var_tt1_dn7: f64 = *var_tt1_dn7_slot;
        let mut var_tt1_dn8: f64 = *var_tt1_dn8_slot;
        let mut var_tt1_dn9: f64 = *var_tt1_dn9_slot;
        let mut var_tt1_rv: f64 = *var_tt1_rv_slot;
        let mut var_tt2: f64 = *var_tt2_slot;
        let mut var_tt2_rv: f64 = *var_tt2_rv_slot;
        let mut var_x: f64 = *var_x_slot;
        let mut var_x_rv: f64 = *var_x_rv_slot;
        let mut var_y: f64 = *var_y_slot;
        let mut var_y_rv: f64 = *var_y_rv_slot;

        let (assign10510_e13324, assign10510_e13324_d_n0, assign10510_e13324_d_n2, assign10510_e13324_d_n3, assign10510_e13324_d_n4, assign10510_e13324_d_n5, assign10510_e13324_d_n6, assign10510_e13324_d_n7, assign10510_e13324_d_n8, assign10510_e13324_d_n9, assign10510_e13324_d_n10, assign10510_e13324_d_n11, assign10510_e13324_d_n13, assign10510_e13324_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10510_e13321: f64 = (var_ccg1 + var_ccg2);
        let assign10510_e13322: f64 = (p.p92 * assign10510_e13321);
        (assign10510_e13322, (p.p92 * var_ccg1_dn0), (p.p92 * var_ccg1_dn2), (p.p92 * var_ccg1_dn3), (p.p92 * var_ccg1_dn4), (p.p92 * var_ccg1_dn5), (p.p92 * var_ccg1_dn6), (p.p92 * var_ccg1_dn7), (p.p92 * var_ccg1_dn8), (p.p92 * var_ccg1_dn9), (p.p92 * var_ccg1_dn10), (p.p92 * var_ccg1_dn11), (p.p92 * var_ccg1_dn13), (p.p92 * var_ccg1_dn14),)
    } else {
        (var_ccg, var_ccg_dn0, var_ccg_dn2, var_ccg_dn3, var_ccg_dn4, var_ccg_dn5, var_ccg_dn6, var_ccg_dn7, var_ccg_dn8, var_ccg_dn9, var_ccg_dn10, var_ccg_dn11, var_ccg_dn13, var_ccg_dn14,)
    }
};
        var_ccg = assign10510_e13324;
        var_ccg_dn0 = assign10510_e13324_d_n0;
        var_ccg_dn2 = assign10510_e13324_d_n2;
        var_ccg_dn3 = assign10510_e13324_d_n3;
        var_ccg_dn4 = assign10510_e13324_d_n4;
        var_ccg_dn5 = assign10510_e13324_d_n5;
        var_ccg_dn6 = assign10510_e13324_d_n6;
        var_ccg_dn7 = assign10510_e13324_d_n7;
        var_ccg_dn8 = assign10510_e13324_d_n8;
        var_ccg_dn9 = assign10510_e13324_d_n9;
        var_ccg_dn10 = assign10510_e13324_d_n10;
        var_ccg_dn11 = assign10510_e13324_d_n11;
        var_ccg_dn13 = assign10510_e13324_d_n13;
        var_ccg_dn14 = assign10510_e13324_d_n14;
        var_ccg_rv = 0.0;

        let (assign10520_e13332,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10520_e13330: f64 = (var_lmax / var_wg);
        (assign10520_e13330,)
    } else {
        (var_x,)
    }
};
        var_x = assign10520_e13332;
        var_x_rv = 0.0;

        let (assign10530_e13347,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10530_e13340: f64 = (var_x + 1.0);
        let assign10530_e13341: f64 = (2.0 * assign10530_e13340);
        let assign10530_e13342: f64 = (assign10530_e13341).sqrt();
        let assign10530_e13344: f64 = (assign10530_e13342 * 3.141592653589793);
        let assign10530_e13345: f64 = (4.0 / assign10530_e13344);
        (assign10530_e13345,)
    } else {
        (var_c1,)
    }
};
        var_c1 = assign10530_e13347;
        var_c1_rv = 0.0;

        let (assign10540_e13383,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10540_e13353: f64 = (p.p90 * p.p90);
        let assign10540_e13356: f64 = (2.0 * var_wg);
        let assign10540_e13358: f64 = (assign10540_e13356 * p.p90);
        let assign10540_e13359: f64 = (assign10540_e13353 + assign10540_e13358);
        let assign10540_e13362: f64 = (var_wg * var_wg);
        let assign10540_e13365: f64 = (var_x + 1.0);
        let assign10540_e13366: f64 = (assign10540_e13362 * assign10540_e13365);
        let assign10540_e13367: f64 = (assign10540_e13359 + assign10540_e13366);
        let assign10540_e13368: f64 = (assign10540_e13367).sqrt();
        let assign10540_e13371: f64 = (var_x + 1.0);
        let assign10540_e13372: f64 = (assign10540_e13371).sqrt();
        let assign10540_e13373: f64 = (assign10540_e13368 * assign10540_e13372);
        let assign10540_e13375: f64 = (assign10540_e13373 + p.p90);
        let assign10540_e13378: f64 = (var_wg * var_x);
        let assign10540_e13379: f64 = (assign10540_e13375 + assign10540_e13378);
        let assign10540_e13381: f64 = (assign10540_e13379 + var_wg);
        (assign10540_e13381,)
    } else {
        (var_c2,)
    }
};
        var_c2 = assign10540_e13383;
        var_c2_rv = 0.0;

        let (assign10550_e13404,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10550_e13390: f64 = (var_x + 1.0);
        let assign10550_e13393: f64 = (var_x + 4.0);
        let assign10550_e13394: f64 = (assign10550_e13390 * assign10550_e13393);
        let assign10550_e13395: f64 = (assign10550_e13394).sqrt();
        let assign10550_e13396: f64 = (p.p90 * assign10550_e13395);
        let assign10550_e13400: f64 = (var_x + 2.0);
        let assign10550_e13401: f64 = (p.p90 * assign10550_e13400);
        let assign10550_e13402: f64 = (assign10550_e13396 + assign10550_e13401);
        (assign10550_e13402,)
    } else {
        (var_c3,)
    }
};
        var_c3 = assign10550_e13404;
        var_c3_rv = 0.0;

        let (assign10560_e13435,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10560_e13412: f64 = (var_c2 / var_c3);
        let (assign10560_e13429,) = {
            if (!(assign10560_e13412 > 1e-38)) {
                let assign10560_e13417: f64 = (-87.498233534);
                (assign10560_e13417,)
            } else {
                let assign10560_e13420: f64 = (var_c2 / var_c3);
                let (assign10560_e13428,) = {
                    if (assign10560_e13420 > 1e-38) {
                        let assign10560_e13425: f64 = (var_c2 / var_c3);
                        let assign10560_e13426: f64 = (assign10560_e13425).ln();
                        (assign10560_e13426,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10560_e13428,)
            }
        };
        let assign10560_e13430: f64 = (var_c1 * assign10560_e13429);
        let assign10560_e13432: f64 = (assign10560_e13430 + 12.27);
        let assign10560_e13433: f64 = (var_epssp * assign10560_e13432);
        (assign10560_e13433,)
    } else {
        (var_cfglog,)
    }
};
        var_cfglog = assign10560_e13435;
        var_cfglog_rv = 0.0;

        let (assign10570_e13443,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10570_e13441: f64 = (var_hr * var_lr);
        (assign10570_e13441,)
    } else {
        (var_dcf,)
    }
};
        var_dcf = assign10570_e13443;
        var_dcf_rv = 0.0;

        let (assign10580_e13454,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10580_e13449: f64 = (var_dcf * var_dcf);
        let assign10580_e13451: f64 = (assign10580_e13449 + 1.0);
        let assign10580_e13452: f64 = (assign10580_e13451).sqrt();
        (assign10580_e13452,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign10580_e13454;
        var_tt0_rv = 0.0;

        let (assign10590_e13503, assign10590_e13503_d_n0, assign10590_e13503_d_n2, assign10590_e13503_d_n3, assign10590_e13503_d_n4, assign10590_e13503_d_n5, assign10590_e13503_d_n6, assign10590_e13503_d_n7, assign10590_e13503_d_n8, assign10590_e13503_d_n9, assign10590_e13503_d_n10, assign10590_e13503_d_n11, assign10590_e13503_d_n13, assign10590_e13503_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10590_e13460: f64 = (var_dcf * var_dcf);
        let assign10590_e13462: f64 = (assign10590_e13460 + 1.0);
        let assign10590_e13465: f64 = (var_dcf * p.p90);
        let assign10590_e13468: f64 = (var_dcf * p.p90);
        let assign10590_e13469: f64 = (assign10590_e13465 * assign10590_e13468);
        let assign10590_e13472: f64 = (2.0 * var_dcf);
        let assign10590_e13474: f64 = (assign10590_e13472 * var_lmax);
        let assign10590_e13476: f64 = (assign10590_e13474 * p.p90);
        let assign10590_e13477: f64 = (assign10590_e13469 + assign10590_e13476);
        let assign10590_e13480: f64 = (var_dcf * var_dcf);
        let assign10590_e13482: f64 = (assign10590_e13480 + 1.0);
        let assign10590_e13484: f64 = (assign10590_e13482 * var_lmax);
        let assign10590_e13486: f64 = (assign10590_e13484 * var_lmax);
        let assign10590_e13487: f64 = (assign10590_e13477 + assign10590_e13486);
        let assign10590_e13488: f64 = (assign10590_e13462 * assign10590_e13487);
        let assign10590_e13489: f64 = (assign10590_e13488).sqrt();
        let assign10590_e13492: f64 = (var_dcf * p.p90);
        let assign10590_e13493: f64 = (assign10590_e13489 + assign10590_e13492);
        let assign10590_e13496: f64 = (var_dcf * var_dcf);
        let assign10590_e13498: f64 = (assign10590_e13496 * var_lmax);
        let assign10590_e13499: f64 = (assign10590_e13493 + assign10590_e13498);
        let assign10590_e13501: f64 = (assign10590_e13499 + var_lmax);
        (assign10590_e13501, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
    }
};
        var_tt1 = assign10590_e13503;
        var_tt1_dn0 = assign10590_e13503_d_n0;
        var_tt1_dn2 = assign10590_e13503_d_n2;
        var_tt1_dn3 = assign10590_e13503_d_n3;
        var_tt1_dn4 = assign10590_e13503_d_n4;
        var_tt1_dn5 = assign10590_e13503_d_n5;
        var_tt1_dn6 = assign10590_e13503_d_n6;
        var_tt1_dn7 = assign10590_e13503_d_n7;
        var_tt1_dn8 = assign10590_e13503_d_n8;
        var_tt1_dn9 = assign10590_e13503_d_n9;
        var_tt1_dn10 = assign10590_e13503_d_n10;
        var_tt1_dn11 = assign10590_e13503_d_n11;
        var_tt1_dn13 = assign10590_e13503_d_n13;
        var_tt1_dn14 = assign10590_e13503_d_n14;
        var_tt1_rv = 0.0;

        let (assign10600_e13515,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10600_e13509: f64 = (var_tt0 + 1.0);
        let assign10600_e13512: f64 = (var_dcf * p.p90);
        let assign10600_e13513: f64 = (assign10600_e13509 * assign10600_e13512);
        (assign10600_e13513,)
    } else {
        (var_tt2,)
    }
};
        var_tt2 = assign10600_e13515;
        var_tt2_rv = 0.0;

        let (assign10610_e13555, assign10610_e13555_d_n0, assign10610_e13555_d_n2, assign10610_e13555_d_n3, assign10610_e13555_d_n4, assign10610_e13555_d_n5, assign10610_e13555_d_n6, assign10610_e13555_d_n7, assign10610_e13555_d_n8, assign10610_e13555_d_n9, assign10610_e13555_d_n10, assign10610_e13555_d_n11, assign10610_e13555_d_n13, assign10610_e13555_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10610_e13521: f64 = (2.0 * var_epssp);
        let assign10610_e13523: f64 = (2.0_f64).sqrt();
        let assign10610_e13524: f64 = (assign10610_e13521 * assign10610_e13523);
        let assign10610_e13526: f64 = (assign10610_e13524 / 3.141592653589793);
        let assign10610_e13528: f64 = (assign10610_e13526 * 0.7);
        let assign10610_e13530: f64 = (assign10610_e13528 * var_dcf);
        let assign10610_e13532: f64 = (assign10610_e13530 / var_tt0);
        let assign10610_e13535: f64 = (var_tt1 / var_tt2);
        let (assign10610_e13552, assign10610_e13552_d_n0, assign10610_e13552_d_n2, assign10610_e13552_d_n3, assign10610_e13552_d_n4, assign10610_e13552_d_n5, assign10610_e13552_d_n6, assign10610_e13552_d_n7, assign10610_e13552_d_n8, assign10610_e13552_d_n9, assign10610_e13552_d_n10, assign10610_e13552_d_n11, assign10610_e13552_d_n13, assign10610_e13552_d_n14,) = {
            if (!(assign10610_e13535 > 1e-38)) {
                let assign10610_e13540: f64 = (-87.498233534);
                (assign10610_e13540, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign10610_e13543: f64 = (var_tt1 / var_tt2);
                let (assign10610_e13551, assign10610_e13551_d_n0, assign10610_e13551_d_n2, assign10610_e13551_d_n3, assign10610_e13551_d_n4, assign10610_e13551_d_n5, assign10610_e13551_d_n6, assign10610_e13551_d_n7, assign10610_e13551_d_n8, assign10610_e13551_d_n9, assign10610_e13551_d_n10, assign10610_e13551_d_n11, assign10610_e13551_d_n13, assign10610_e13551_d_n14,) = {
                    if (assign10610_e13543 > 1e-38) {
                        let assign10610_e13548: f64 = (var_tt1 / var_tt2);
                        let assign10610_e13549: f64 = (assign10610_e13548).ln();
                        (assign10610_e13549, ((var_tt1_dn0 / var_tt2) / assign10610_e13548), ((var_tt1_dn2 / var_tt2) / assign10610_e13548), ((var_tt1_dn3 / var_tt2) / assign10610_e13548), ((var_tt1_dn4 / var_tt2) / assign10610_e13548), ((var_tt1_dn5 / var_tt2) / assign10610_e13548), ((var_tt1_dn6 / var_tt2) / assign10610_e13548), ((var_tt1_dn7 / var_tt2) / assign10610_e13548), ((var_tt1_dn8 / var_tt2) / assign10610_e13548), ((var_tt1_dn9 / var_tt2) / assign10610_e13548), ((var_tt1_dn10 / var_tt2) / assign10610_e13548), ((var_tt1_dn11 / var_tt2) / assign10610_e13548), ((var_tt1_dn13 / var_tt2) / assign10610_e13548), ((var_tt1_dn14 / var_tt2) / assign10610_e13548),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign10610_e13551, assign10610_e13551_d_n0, assign10610_e13551_d_n2, assign10610_e13551_d_n3, assign10610_e13551_d_n4, assign10610_e13551_d_n5, assign10610_e13551_d_n6, assign10610_e13551_d_n7, assign10610_e13551_d_n8, assign10610_e13551_d_n9, assign10610_e13551_d_n10, assign10610_e13551_d_n11, assign10610_e13551_d_n13, assign10610_e13551_d_n14,)
            }
        };
        let assign10610_e13553: f64 = (assign10610_e13532 * assign10610_e13552);
        (assign10610_e13553, (assign10610_e13532 * assign10610_e13552_d_n0), (assign10610_e13532 * assign10610_e13552_d_n2), (assign10610_e13532 * assign10610_e13552_d_n3), (assign10610_e13532 * assign10610_e13552_d_n4), (assign10610_e13532 * assign10610_e13552_d_n5), (assign10610_e13532 * assign10610_e13552_d_n6), (assign10610_e13532 * assign10610_e13552_d_n7), (assign10610_e13532 * assign10610_e13552_d_n8), (assign10610_e13532 * assign10610_e13552_d_n9), (assign10610_e13532 * assign10610_e13552_d_n10), (assign10610_e13532 * assign10610_e13552_d_n11), (assign10610_e13532 * assign10610_e13552_d_n13), (assign10610_e13532 * assign10610_e13552_d_n14),)
    } else {
        (var_cfgsat, var_cfgsat_dn0, var_cfgsat_dn2, var_cfgsat_dn3, var_cfgsat_dn4, var_cfgsat_dn5, var_cfgsat_dn6, var_cfgsat_dn7, var_cfgsat_dn8, var_cfgsat_dn9, var_cfgsat_dn10, var_cfgsat_dn11, var_cfgsat_dn13, var_cfgsat_dn14,)
    }
};
        var_cfgsat = assign10610_e13555;
        var_cfgsat_dn0 = assign10610_e13555_d_n0;
        var_cfgsat_dn2 = assign10610_e13555_d_n2;
        var_cfgsat_dn3 = assign10610_e13555_d_n3;
        var_cfgsat_dn4 = assign10610_e13555_d_n4;
        var_cfgsat_dn5 = assign10610_e13555_d_n5;
        var_cfgsat_dn6 = assign10610_e13555_d_n6;
        var_cfgsat_dn7 = assign10610_e13555_d_n7;
        var_cfgsat_dn8 = assign10610_e13555_d_n8;
        var_cfgsat_dn9 = assign10610_e13555_d_n9;
        var_cfgsat_dn10 = assign10610_e13555_d_n10;
        var_cfgsat_dn11 = assign10610_e13555_d_n11;
        var_cfgsat_dn13 = assign10610_e13555_d_n13;
        var_cfgsat_dn14 = assign10610_e13555_d_n14;
        var_cfgsat_rv = 0.0;

        let (assign10620_e13561, assign10620_e13561_d_n0, assign10620_e13561_d_n2, assign10620_e13561_d_n3, assign10620_e13561_d_n4, assign10620_e13561_d_n5, assign10620_e13561_d_n6, assign10620_e13561_d_n7, assign10620_e13561_d_n8, assign10620_e13561_d_n9, assign10620_e13561_d_n10, assign10620_e13561_d_n11, assign10620_e13561_d_n13, assign10620_e13561_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delta, var_delta_dn0, var_delta_dn2, var_delta_dn3, var_delta_dn4, var_delta_dn5, var_delta_dn6, var_delta_dn7, var_delta_dn8, var_delta_dn9, var_delta_dn10, var_delta_dn11, var_delta_dn13, var_delta_dn14,)
    }
};
        var_delta = assign10620_e13561;
        var_delta_dn0 = assign10620_e13561_d_n0;
        var_delta_dn2 = assign10620_e13561_d_n2;
        var_delta_dn3 = assign10620_e13561_d_n3;
        var_delta_dn4 = assign10620_e13561_d_n4;
        var_delta_dn5 = assign10620_e13561_d_n5;
        var_delta_dn6 = assign10620_e13561_d_n6;
        var_delta_dn7 = assign10620_e13561_d_n7;
        var_delta_dn8 = assign10620_e13561_d_n8;
        var_delta_dn9 = assign10620_e13561_d_n9;
        var_delta_dn10 = assign10620_e13561_d_n10;
        var_delta_dn11 = assign10620_e13561_d_n11;
        var_delta_dn13 = assign10620_e13561_d_n13;
        var_delta_dn14 = assign10620_e13561_d_n14;
        var_delta_rv = 0.0;

        let (assign10630_e13571, assign10630_e13571_d_n0, assign10630_e13571_d_n2, assign10630_e13571_d_n3, assign10630_e13571_d_n4, assign10630_e13571_d_n5, assign10630_e13571_d_n6, assign10630_e13571_d_n7, assign10630_e13571_d_n8, assign10630_e13571_d_n9, assign10630_e13571_d_n10, assign10630_e13571_d_n11, assign10630_e13571_d_n13, assign10630_e13571_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10630_e13567: f64 = (var_cfgsat - var_cfglog);
        let assign10630_e13569: f64 = (assign10630_e13567 - var_delta);
        (assign10630_e13569, (var_cfgsat_dn0 - var_delta_dn0), (var_cfgsat_dn2 - var_delta_dn2), (var_cfgsat_dn3 - var_delta_dn3), (var_cfgsat_dn4 - var_delta_dn4), (var_cfgsat_dn5 - var_delta_dn5), (var_cfgsat_dn6 - var_delta_dn6), (var_cfgsat_dn7 - var_delta_dn7), (var_cfgsat_dn8 - var_delta_dn8), (var_cfgsat_dn9 - var_delta_dn9), (var_cfgsat_dn10 - var_delta_dn10), (var_cfgsat_dn11 - var_delta_dn11), (var_cfgsat_dn13 - var_delta_dn13), (var_cfgsat_dn14 - var_delta_dn14),)
    } else {
        (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
    }
};
        var_tt1 = assign10630_e13571;
        var_tt1_dn0 = assign10630_e13571_d_n0;
        var_tt1_dn2 = assign10630_e13571_d_n2;
        var_tt1_dn3 = assign10630_e13571_d_n3;
        var_tt1_dn4 = assign10630_e13571_d_n4;
        var_tt1_dn5 = assign10630_e13571_d_n5;
        var_tt1_dn6 = assign10630_e13571_d_n6;
        var_tt1_dn7 = assign10630_e13571_d_n7;
        var_tt1_dn8 = assign10630_e13571_d_n8;
        var_tt1_dn9 = assign10630_e13571_d_n9;
        var_tt1_dn10 = assign10630_e13571_d_n10;
        var_tt1_dn11 = assign10630_e13571_d_n11;
        var_tt1_dn13 = assign10630_e13571_d_n13;
        var_tt1_dn14 = assign10630_e13571_d_n14;
        var_tt1_rv = 0.0;

        let (assign10640_e13594, assign10640_e13594_d_n0, assign10640_e13594_d_n2, assign10640_e13594_d_n3, assign10640_e13594_d_n4, assign10640_e13594_d_n5, assign10640_e13594_d_n6, assign10640_e13594_d_n7, assign10640_e13594_d_n8, assign10640_e13594_d_n9, assign10640_e13594_d_n10, assign10640_e13594_d_n11, assign10640_e13594_d_n13, assign10640_e13594_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10640_e13581: f64 = (var_tt1 * var_tt1);
        let assign10640_e13584: f64 = (4.0 * var_delta);
        let assign10640_e13586: f64 = (assign10640_e13584 * var_cfgsat);
        let assign10640_e13587: f64 = (assign10640_e13581 + assign10640_e13586);
        let assign10640_e13588: f64 = (assign10640_e13587).sqrt();
        let assign10640_e13589: f64 = (var_tt1 + assign10640_e13588);
        let assign10640_e13590: f64 = (0.5 * assign10640_e13589);
        let assign10640_e13591: f64 = (var_cfgsat - assign10640_e13590);
        let assign10640_e13592: f64 = (p.p92 * assign10640_e13591);
        (assign10640_e13592, (p.p92 * (var_cfgsat_dn0 - (0.5 * (var_tt1_dn0 + ((((var_tt1_dn0 * var_tt1) + (var_tt1 * var_tt1_dn0)) + (((4.0 * var_delta_dn0) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn0))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn2 - (0.5 * (var_tt1_dn2 + ((((var_tt1_dn2 * var_tt1) + (var_tt1 * var_tt1_dn2)) + (((4.0 * var_delta_dn2) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn2))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn3 - (0.5 * (var_tt1_dn3 + ((((var_tt1_dn3 * var_tt1) + (var_tt1 * var_tt1_dn3)) + (((4.0 * var_delta_dn3) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn3))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn4 - (0.5 * (var_tt1_dn4 + ((((var_tt1_dn4 * var_tt1) + (var_tt1 * var_tt1_dn4)) + (((4.0 * var_delta_dn4) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn4))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn5 - (0.5 * (var_tt1_dn5 + ((((var_tt1_dn5 * var_tt1) + (var_tt1 * var_tt1_dn5)) + (((4.0 * var_delta_dn5) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn5))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn6 - (0.5 * (var_tt1_dn6 + ((((var_tt1_dn6 * var_tt1) + (var_tt1 * var_tt1_dn6)) + (((4.0 * var_delta_dn6) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn6))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn7 - (0.5 * (var_tt1_dn7 + ((((var_tt1_dn7 * var_tt1) + (var_tt1 * var_tt1_dn7)) + (((4.0 * var_delta_dn7) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn7))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn8 - (0.5 * (var_tt1_dn8 + ((((var_tt1_dn8 * var_tt1) + (var_tt1 * var_tt1_dn8)) + (((4.0 * var_delta_dn8) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn8))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn9 - (0.5 * (var_tt1_dn9 + ((((var_tt1_dn9 * var_tt1) + (var_tt1 * var_tt1_dn9)) + (((4.0 * var_delta_dn9) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn9))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn10 - (0.5 * (var_tt1_dn10 + ((((var_tt1_dn10 * var_tt1) + (var_tt1 * var_tt1_dn10)) + (((4.0 * var_delta_dn10) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn10))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn11 - (0.5 * (var_tt1_dn11 + ((((var_tt1_dn11 * var_tt1) + (var_tt1 * var_tt1_dn11)) + (((4.0 * var_delta_dn11) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn11))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn13 - (0.5 * (var_tt1_dn13 + ((((var_tt1_dn13 * var_tt1) + (var_tt1 * var_tt1_dn13)) + (((4.0 * var_delta_dn13) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn13))) / (2.0 * assign10640_e13588)))))), (p.p92 * (var_cfgsat_dn14 - (0.5 * (var_tt1_dn14 + ((((var_tt1_dn14 * var_tt1) + (var_tt1 * var_tt1_dn14)) + (((4.0 * var_delta_dn14) * var_cfgsat) + (assign10640_e13584 * var_cfgsat_dn14))) / (2.0 * assign10640_e13588)))))),)
    } else {
        (var_cfg, var_cfg_dn0, var_cfg_dn2, var_cfg_dn3, var_cfg_dn4, var_cfg_dn5, var_cfg_dn6, var_cfg_dn7, var_cfg_dn8, var_cfg_dn9, var_cfg_dn10, var_cfg_dn11, var_cfg_dn13, var_cfg_dn14,)
    }
};
        var_cfg = assign10640_e13594;
        var_cfg_dn0 = assign10640_e13594_d_n0;
        var_cfg_dn2 = assign10640_e13594_d_n2;
        var_cfg_dn3 = assign10640_e13594_d_n3;
        var_cfg_dn4 = assign10640_e13594_d_n4;
        var_cfg_dn5 = assign10640_e13594_d_n5;
        var_cfg_dn6 = assign10640_e13594_d_n6;
        var_cfg_dn7 = assign10640_e13594_d_n7;
        var_cfg_dn8 = assign10640_e13594_d_n8;
        var_cfg_dn9 = assign10640_e13594_d_n9;
        var_cfg_dn10 = assign10640_e13594_d_n10;
        var_cfg_dn11 = assign10640_e13594_d_n11;
        var_cfg_dn13 = assign10640_e13594_d_n13;
        var_cfg_dn14 = assign10640_e13594_d_n14;
        var_cfg_rv = 0.0;

        let (assign10650_e13602, assign10650_e13602_d_n0, assign10650_e13602_d_n2, assign10650_e13602_d_n3, assign10650_e13602_d_n4, assign10650_e13602_d_n5, assign10650_e13602_d_n6, assign10650_e13602_d_n7, assign10650_e13602_d_n8, assign10650_e13602_d_n9, assign10650_e13602_d_n10, assign10650_e13602_d_n11, assign10650_e13602_d_n13, assign10650_e13602_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 != 0.0)) {
        let assign10650_e13600: f64 = (var_ccg + var_cfg);
        (assign10650_e13600, (var_ccg_dn0 + var_cfg_dn0), (var_ccg_dn2 + var_cfg_dn2), (var_ccg_dn3 + var_cfg_dn3), (var_ccg_dn4 + var_cfg_dn4), (var_ccg_dn5 + var_cfg_dn5), (var_ccg_dn6 + var_cfg_dn6), (var_ccg_dn7 + var_cfg_dn7), (var_ccg_dn8 + var_cfg_dn8), (var_ccg_dn9 + var_cfg_dn9), (var_ccg_dn10 + var_cfg_dn10), (var_ccg_dn11 + var_cfg_dn11), (var_ccg_dn13 + var_cfg_dn13), (var_ccg_dn14 + var_cfg_dn14),)
    } else {
        (var_cgg_side, var_cgg_side_dn0, var_cgg_side_dn2, var_cgg_side_dn3, var_cgg_side_dn4, var_cgg_side_dn5, var_cgg_side_dn6, var_cgg_side_dn7, var_cgg_side_dn8, var_cgg_side_dn9, var_cgg_side_dn10, var_cgg_side_dn11, var_cgg_side_dn13, var_cgg_side_dn14,)
    }
};
        var_cgg_side = assign10650_e13602;
        var_cgg_side_dn0 = assign10650_e13602_d_n0;
        var_cgg_side_dn2 = assign10650_e13602_d_n2;
        var_cgg_side_dn3 = assign10650_e13602_d_n3;
        var_cgg_side_dn4 = assign10650_e13602_d_n4;
        var_cgg_side_dn5 = assign10650_e13602_d_n5;
        var_cgg_side_dn6 = assign10650_e13602_d_n6;
        var_cgg_side_dn7 = assign10650_e13602_d_n7;
        var_cgg_side_dn8 = assign10650_e13602_d_n8;
        var_cgg_side_dn9 = assign10650_e13602_d_n9;
        var_cgg_side_dn10 = assign10650_e13602_d_n10;
        var_cgg_side_dn11 = assign10650_e13602_d_n11;
        var_cgg_side_dn13 = assign10650_e13602_d_n13;
        var_cgg_side_dn14 = assign10650_e13602_d_n14;
        var_cgg_side_rv = 0.0;

        let (assign10660_e13617,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        let assign10660_e13611: f64 = (var_wg + p.p90);
        let assign10660_e13612: f64 = (0.2 * assign10660_e13611);
        let assign10660_e13614: f64 = (assign10660_e13612 / var_trsd);
        let assign10660_e13615: f64 = (2.3 + assign10660_e13614);
        (assign10660_e13615,)
    } else {
        (var_hr,)
    }
};
        var_hr = assign10660_e13617;
        var_hr_rv = 0.0;

        let (assign10670_e13624,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        (1.05,)
    } else {
        (var_lr,)
    }
};
        var_lr = assign10670_e13624;
        var_lr_rv = 0.0;

        let (assign10680_e13636,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        let assign10680_e13631: f64 = (var_wg + p.p90);
        let assign10680_e13633: f64 = (assign10680_e13631 - var_trsd);
        let assign10680_e13634: f64 = (assign10680_e13633).abs();
        (assign10680_e13634,)
    } else {
        (var_hgdelta,)
    }
};
        var_hgdelta = assign10680_e13636;
        var_hgdelta_rv = 0.0;

        let (assign10690_e13645,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        let assign10690_e13643: f64 = (p.p1087 * var_lr);
        (assign10690_e13643,)
    } else {
        (var_lmax,)
    }
};
        var_lmax = assign10690_e13645;
        var_lmax_rv = 0.0;

        let (assign10700_e13656,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        let assign10700_e13653: f64 = (var_wg + p.p90);
        let assign10700_e13654: f64 = (var_trsd).min(assign10700_e13653);
        (assign10700_e13654,)
    } else {
        (var_y,)
    }
};
        var_y = assign10700_e13656;
        var_y_rv = 0.0;

        let (assign10710_e13667,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        let assign10710_e13664: f64 = (var_hr + 1.0);
        let assign10710_e13665: f64 = (p.p1087 / assign10710_e13664);
        (assign10710_e13665,)
    } else {
        (var_x,)
    }
};
        var_x = assign10710_e13667;
        var_x_rv = 0.0;

        let (assign10720_e13674,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        (1700000000000.0,)
    } else {
        (var_cnon,)
    }
};
        var_cnon = assign10720_e13674;
        var_cnon_rv = 0.0;

        let (assign10730_e13687,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        let assign10730_e13682: f64 = (var_y - var_x);
        let assign10730_e13683: f64 = (var_epssp * assign10730_e13682);
        let assign10730_e13685: f64 = (assign10730_e13683 / p.p1087);
        (assign10730_e13685,)
    } else {
        (var_ccgsat,)
    }
};
        var_ccgsat = assign10730_e13687;
        var_ccgsat_rv = 0.0;

        let (assign10740_e13696, assign10740_e13696_d_n0, assign10740_e13696_d_n2, assign10740_e13696_d_n3, assign10740_e13696_d_n4, assign10740_e13696_d_n5, assign10740_e13696_d_n6, assign10740_e13696_d_n7, assign10740_e13696_d_n8, assign10740_e13696_d_n9, assign10740_e13696_d_n10, assign10740_e13696_d_n11, assign10740_e13696_d_n13, assign10740_e13696_d_n14,) = {
    if ((var_guard205 != 0.0) && (var_guard208 == 0.0)) {
        let assign10740_e13694: f64 = (var_cnon * var_ccgsat);
        (assign10740_e13694, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tt1, var_tt1_dn0, var_tt1_dn2, var_tt1_dn3, var_tt1_dn4, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9, var_tt1_dn10, var_tt1_dn11, var_tt1_dn13, var_tt1_dn14,)
    }
};
        var_tt1 = assign10740_e13696;
        var_tt1_dn0 = assign10740_e13696_d_n0;
        var_tt1_dn2 = assign10740_e13696_d_n2;
        var_tt1_dn3 = assign10740_e13696_d_n3;
        var_tt1_dn4 = assign10740_e13696_d_n4;
        var_tt1_dn5 = assign10740_e13696_d_n5;
        var_tt1_dn6 = assign10740_e13696_d_n6;
        var_tt1_dn7 = assign10740_e13696_d_n7;
        var_tt1_dn8 = assign10740_e13696_d_n8;
        var_tt1_dn9 = assign10740_e13696_d_n9;
        var_tt1_dn10 = assign10740_e13696_d_n10;
        var_tt1_dn11 = assign10740_e13696_d_n11;
        var_tt1_dn13 = assign10740_e13696_d_n13;
        var_tt1_dn14 = assign10740_e13696_d_n14;
        var_tt1_rv = 0.0;

        let assign10750_e13699: f64 = if var_tt1 > 80.0 { 1.0 } else { 0.0 };
        var_guard210 = assign10750_e13699;
        var_guard210_rv = 0.0;

        let (assign10760_e13708, assign10760_e13708_d_n0, assign10760_e13708_d_n2, assign10760_e13708_d_n3, assign10760_e13708_d_n4, assign10760_e13708_d_n5, assign10760_e13708_d_n6, assign10760_e13708_d_n7, assign10760_e13708_d_n8, assign10760_e13708_d_n9, assign10760_e13708_d_n10, assign10760_e13708_d_n11, assign10760_e13708_d_n13, assign10760_e13708_d_n14,) = {
    if (((var_guard205 != 0.0) && (var_guard208 == 0.0)) && (var_guard210 != 0.0)) {
        (var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ccg1, var_ccg1_dn0, var_ccg1_dn2, var_ccg1_dn3, var_ccg1_dn4, var_ccg1_dn5, var_ccg1_dn6, var_ccg1_dn7, var_ccg1_dn8, var_ccg1_dn9, var_ccg1_dn10, var_ccg1_dn11, var_ccg1_dn13, var_ccg1_dn14,)
    }
};
        var_ccg1 = assign10760_e13708;
        var_ccg1_dn0 = assign10760_e13708_d_n0;
        var_ccg1_dn2 = assign10760_e13708_d_n2;
        var_ccg1_dn3 = assign10760_e13708_d_n3;
        var_ccg1_dn4 = assign10760_e13708_d_n4;
        var_ccg1_dn5 = assign10760_e13708_d_n5;
        var_ccg1_dn6 = assign10760_e13708_d_n6;
        var_ccg1_dn7 = assign10760_e13708_d_n7;
        var_ccg1_dn8 = assign10760_e13708_d_n8;
        var_ccg1_dn9 = assign10760_e13708_d_n9;
        var_ccg1_dn10 = assign10760_e13708_d_n10;
        var_ccg1_dn11 = assign10760_e13708_d_n11;
        var_ccg1_dn13 = assign10760_e13708_d_n13;
        var_ccg1_dn14 = assign10760_e13708_d_n14;
        var_ccg1_rv = 0.0;

        *var_c1_slot = var_c1;
        *var_c1_rv_slot = var_c1_rv;
        *var_c2_slot = var_c2;
        *var_c2_rv_slot = var_c2_rv;
        *var_c3_slot = var_c3;
        *var_c3_rv_slot = var_c3_rv;
        *var_ccg_slot = var_ccg;
        *var_ccg1_slot = var_ccg1;
        *var_ccg1_dn0_slot = var_ccg1_dn0;
        *var_ccg1_dn10_slot = var_ccg1_dn10;
        *var_ccg1_dn11_slot = var_ccg1_dn11;
        *var_ccg1_dn13_slot = var_ccg1_dn13;
        *var_ccg1_dn14_slot = var_ccg1_dn14;
        *var_ccg1_dn2_slot = var_ccg1_dn2;
        *var_ccg1_dn3_slot = var_ccg1_dn3;
        *var_ccg1_dn4_slot = var_ccg1_dn4;
        *var_ccg1_dn5_slot = var_ccg1_dn5;
        *var_ccg1_dn6_slot = var_ccg1_dn6;
        *var_ccg1_dn7_slot = var_ccg1_dn7;
        *var_ccg1_dn8_slot = var_ccg1_dn8;
        *var_ccg1_dn9_slot = var_ccg1_dn9;
        *var_ccg1_rv_slot = var_ccg1_rv;
        *var_ccg_dn0_slot = var_ccg_dn0;
        *var_ccg_dn10_slot = var_ccg_dn10;
        *var_ccg_dn11_slot = var_ccg_dn11;
        *var_ccg_dn13_slot = var_ccg_dn13;
        *var_ccg_dn14_slot = var_ccg_dn14;
        *var_ccg_dn2_slot = var_ccg_dn2;
        *var_ccg_dn3_slot = var_ccg_dn3;
        *var_ccg_dn4_slot = var_ccg_dn4;
        *var_ccg_dn5_slot = var_ccg_dn5;
        *var_ccg_dn6_slot = var_ccg_dn6;
        *var_ccg_dn7_slot = var_ccg_dn7;
        *var_ccg_dn8_slot = var_ccg_dn8;
        *var_ccg_dn9_slot = var_ccg_dn9;
        *var_ccg_rv_slot = var_ccg_rv;
        *var_ccgsat_slot = var_ccgsat;
        *var_ccgsat_rv_slot = var_ccgsat_rv;
        *var_cfg_slot = var_cfg;
        *var_cfg_dn0_slot = var_cfg_dn0;
        *var_cfg_dn10_slot = var_cfg_dn10;
        *var_cfg_dn11_slot = var_cfg_dn11;
        *var_cfg_dn13_slot = var_cfg_dn13;
        *var_cfg_dn14_slot = var_cfg_dn14;
        *var_cfg_dn2_slot = var_cfg_dn2;
        *var_cfg_dn3_slot = var_cfg_dn3;
        *var_cfg_dn4_slot = var_cfg_dn4;
        *var_cfg_dn5_slot = var_cfg_dn5;
        *var_cfg_dn6_slot = var_cfg_dn6;
        *var_cfg_dn7_slot = var_cfg_dn7;
        *var_cfg_dn8_slot = var_cfg_dn8;
        *var_cfg_dn9_slot = var_cfg_dn9;
        *var_cfg_rv_slot = var_cfg_rv;
        *var_cfglog_slot = var_cfglog;
        *var_cfglog_rv_slot = var_cfglog_rv;
        *var_cfgsat_slot = var_cfgsat;
        *var_cfgsat_dn0_slot = var_cfgsat_dn0;
        *var_cfgsat_dn10_slot = var_cfgsat_dn10;
        *var_cfgsat_dn11_slot = var_cfgsat_dn11;
        *var_cfgsat_dn13_slot = var_cfgsat_dn13;
        *var_cfgsat_dn14_slot = var_cfgsat_dn14;
        *var_cfgsat_dn2_slot = var_cfgsat_dn2;
        *var_cfgsat_dn3_slot = var_cfgsat_dn3;
        *var_cfgsat_dn4_slot = var_cfgsat_dn4;
        *var_cfgsat_dn5_slot = var_cfgsat_dn5;
        *var_cfgsat_dn6_slot = var_cfgsat_dn6;
        *var_cfgsat_dn7_slot = var_cfgsat_dn7;
        *var_cfgsat_dn8_slot = var_cfgsat_dn8;
        *var_cfgsat_dn9_slot = var_cfgsat_dn9;
        *var_cfgsat_rv_slot = var_cfgsat_rv;
        *var_cgg_side_slot = var_cgg_side;
        *var_cgg_side_dn0_slot = var_cgg_side_dn0;
        *var_cgg_side_dn10_slot = var_cgg_side_dn10;
        *var_cgg_side_dn11_slot = var_cgg_side_dn11;
        *var_cgg_side_dn13_slot = var_cgg_side_dn13;
        *var_cgg_side_dn14_slot = var_cgg_side_dn14;
        *var_cgg_side_dn2_slot = var_cgg_side_dn2;
        *var_cgg_side_dn3_slot = var_cgg_side_dn3;
        *var_cgg_side_dn4_slot = var_cgg_side_dn4;
        *var_cgg_side_dn5_slot = var_cgg_side_dn5;
        *var_cgg_side_dn6_slot = var_cgg_side_dn6;
        *var_cgg_side_dn7_slot = var_cgg_side_dn7;
        *var_cgg_side_dn8_slot = var_cgg_side_dn8;
        *var_cgg_side_dn9_slot = var_cgg_side_dn9;
        *var_cgg_side_rv_slot = var_cgg_side_rv;
        *var_cnon_slot = var_cnon;
        *var_cnon_rv_slot = var_cnon_rv;
        *var_dcf_slot = var_dcf;
        *var_dcf_rv_slot = var_dcf_rv;
        *var_delta_slot = var_delta;
        *var_delta_dn0_slot = var_delta_dn0;
        *var_delta_dn10_slot = var_delta_dn10;
        *var_delta_dn11_slot = var_delta_dn11;
        *var_delta_dn13_slot = var_delta_dn13;
        *var_delta_dn14_slot = var_delta_dn14;
        *var_delta_dn2_slot = var_delta_dn2;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_delta_dn9_slot = var_delta_dn9;
        *var_delta_rv_slot = var_delta_rv;
        *var_guard210_slot = var_guard210;
        *var_guard210_rv_slot = var_guard210_rv;
        *var_hgdelta_slot = var_hgdelta;
        *var_hgdelta_rv_slot = var_hgdelta_rv;
        *var_hr_slot = var_hr;
        *var_hr_rv_slot = var_hr_rv;
        *var_lmax_slot = var_lmax;
        *var_lmax_rv_slot = var_lmax_rv;
        *var_lr_slot = var_lr;
        *var_lr_rv_slot = var_lr_rv;
        *var_tt0_slot = var_tt0;
        *var_tt0_rv_slot = var_tt0_rv;
        *var_tt1_slot = var_tt1;
        *var_tt1_dn0_slot = var_tt1_dn0;
        *var_tt1_dn10_slot = var_tt1_dn10;
        *var_tt1_dn11_slot = var_tt1_dn11;
        *var_tt1_dn13_slot = var_tt1_dn13;
        *var_tt1_dn14_slot = var_tt1_dn14;
        *var_tt1_dn2_slot = var_tt1_dn2;
        *var_tt1_dn3_slot = var_tt1_dn3;
        *var_tt1_dn4_slot = var_tt1_dn4;
        *var_tt1_dn5_slot = var_tt1_dn5;
        *var_tt1_dn6_slot = var_tt1_dn6;
        *var_tt1_dn7_slot = var_tt1_dn7;
        *var_tt1_dn8_slot = var_tt1_dn8;
        *var_tt1_dn9_slot = var_tt1_dn9;
        *var_tt1_rv_slot = var_tt1_rv;
        *var_tt2_slot = var_tt2;
        *var_tt2_rv_slot = var_tt2_rv;
        *var_x_slot = var_x;
        *var_x_rv_slot = var_x_rv;
        *var_y_slot = var_y;
        *var_y_rv_slot = var_y_rv;
    }
}

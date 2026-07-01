#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e188: f64 = (8.8541878176e-12 * 11.8);
        locals.var_epssi = assign00_e188;

        let assign10_e191: f64 = (-250.0);
        let (assign10_e196,) = {
    if (p.p6 > assign10_e191) {
        (p.p6,)
    } else {
        let assign10_e195: f64 = (-250.0);
        (assign10_e195,)
    }
};
        locals.var_trj_i = assign10_e196;

        let assign20_e202: f64 = if ((!param_given[6]) && param_given[96]) { 1.0 } else { 0.0 };
        locals.var_guard1 = assign20_e202;

        let (assign30_e213,) = {
    if (locals.var_guard1 != 0.0) {
        let assign30_e206: f64 = (-250.0);
        let (assign30_e211,) = {
            if (p.p96 > assign30_e206) {
                (p.p96,)
            } else {
                let assign30_e210: f64 = (-250.0);
                (assign30_e210,)
            }
        };
        (assign30_e211,)
    } else {
        (locals.var_trj_i,)
    }
};
        locals.var_trj_i = assign30_e213;

        let (assign40_e219,) = {
    if (p.p5 > 1e-12) {
        (p.p5,)
    } else {
        (1e-12,)
    }
};
        locals.var_imax_i = assign40_e219;

        locals.var_phigbot_i = p.p17;

        locals.var_phigsti_i = p.p18;

        locals.var_phiggat_i = p.p19;

        let (assign170_e297,) = {
    if (p.p20 > 0.0) {
        (p.p20,)
    } else {
        (0.0,)
    }
};
        locals.var_idsatrbot_i = assign170_e297;

        let (assign180_e303,) = {
    if (p.p21 > 0.0) {
        (p.p21,)
    } else {
        (0.0,)
    }
};
        locals.var_idsatrsti_i = assign180_e303;

        let (assign190_e309,) = {
    if (p.p22 > 0.0) {
        (p.p22,)
    } else {
        (0.0,)
    }
};
        locals.var_idsatrgat_i = assign190_e309;

        let (assign610_e496,) = {
    if (p.p63 > 0.1) {
        (p.p63,)
    } else {
        (0.1,)
    }
};
        locals.var_nfabot_i = assign610_e496;

        let (assign620_e502,) = {
    if (p.p64 > 0.1) {
        (p.p64,)
    } else {
        (0.1,)
    }
};
        locals.var_nfasti_i = assign620_e502;

        let (assign630_e508,) = {
    if (p.p65 > 0.1) {
        (p.p65,)
    } else {
        (0.1,)
    }
};
        locals.var_nfagat_i = assign630_e508;

        let (assign740_e578,) = {
    if (p.p76 > 0.1) {
        (p.p76,)
    } else {
        (0.1,)
    }
};
        locals.var_xti_i = assign740_e578;

        let (assign750_e584,) = {
    if (p.p77 > 0.0) {
        (p.p77,)
    } else {
        (0.0,)
    }
};
        locals.var_scale_i = assign750_e584;

        let (assign760_e590,) = {
    if (p.p78 > 0.0) {
        (p.p78,)
    } else {
        (0.0,)
    }
};
        locals.var_shrink_i = assign760_e590;

        locals.var_swjunexp_i = 0.0;

        let assign780_e594: f64 = if p.p81 > 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign780_e594;

        let (assign790_e598,) = {
    if (locals.var_guard2 != 0.0) {
        (1.0,)
    } else {
        (locals.var_swjunexp_i,)
    }
};
        locals.var_swjunexp_i = assign790_e598;

        let (assign800_e603,) = {
    if (locals.var_guard2 == 0.0) {
        (0.0,)
    } else {
        (locals.var_swjunexp_i,)
    }
};
        locals.var_swjunexp_i = assign800_e603;

        let (assign810_e609,) = {
    if (p.p82 > 0.5) {
        (p.p82,)
    } else {
        (0.5,)
    }
};
        locals.var_vjunref_i = assign810_e609;

        let assign830_e618: f64 = (273.15 + locals.var_trj_i);
        locals.var_tkr = assign830_e618;

        let assign840_e619: f64 = ctx_temp;
        let assign840_e621: f64 = (assign840_e619 + p.p102);
        let assign840_e624: f64 = (-250.0);
        let assign840_e625: f64 = (273.15 + assign840_e624);
        let assign840_e626: f64 = (assign840_e621).max(assign840_e625);
        locals.var_tkd = assign840_e626;

        let assign850_e629: f64 = (locals.var_tkd / locals.var_tkr);
        locals.var_auxt = assign850_e629;

        let assign860_e632: f64 = (1.3806505e-23 / 1.6021918e-19);
        locals.var_kbol_over_qele = assign860_e632;

        let assign870_e635: f64 = (locals.var_kbol_over_qele * locals.var_tkr);
        locals.var_phitr = assign870_e635;

        let assign880_e638: f64 = (1.0 / locals.var_phitr);
        locals.var_phitrinv = assign880_e638;

        let assign890_e641: f64 = (locals.var_kbol_over_qele * locals.var_tkd);
        locals.var_phitd = assign890_e641;

        let assign900_e644: f64 = (1.0 / locals.var_phitd);
        locals.var_phitdinv = assign900_e644;

        let assign910_e647: f64 = (0.000702 * locals.var_tkr);
        let assign910_e649: f64 = (assign910_e647 * locals.var_tkr);
        let assign910_e650: f64 = (-assign910_e649);
        let assign910_e653: f64 = (1108.0 + locals.var_tkr);
        let assign910_e654: f64 = (assign910_e650 / assign910_e653);
        locals.var_deltaphigr = assign910_e654;

        let assign920_e657: f64 = (locals.var_phigbot_i + locals.var_deltaphigr);
        locals.var_phigrbot = assign920_e657;

        let assign930_e660: f64 = (locals.var_phigsti_i + locals.var_deltaphigr);
        locals.var_phigrsti = assign930_e660;

        let assign940_e663: f64 = (locals.var_phiggat_i + locals.var_deltaphigr);
        locals.var_phigrgat = assign940_e663;

        let assign950_e666: f64 = (0.000702 * locals.var_tkd);
        let assign950_e668: f64 = (assign950_e666 * locals.var_tkd);
        let assign950_e669: f64 = (-assign950_e668);
        let assign950_e672: f64 = (1108.0 + locals.var_tkd);
        let assign950_e673: f64 = (assign950_e669 / assign950_e672);
        locals.var_deltaphigd = assign950_e673;

        let assign960_e676: f64 = (locals.var_phigbot_i + locals.var_deltaphigd);
        locals.var_phigdbot = assign960_e676;

        let assign970_e679: f64 = (locals.var_phigsti_i + locals.var_deltaphigd);
        locals.var_phigdsti = assign970_e679;

        let assign980_e682: f64 = (locals.var_phiggat_i + locals.var_deltaphigd);
        locals.var_phigdgat = assign980_e682;

        let assign1020_e734: f64 = (locals.var_xti_i / 2.0);
        let assign1020_e736: f64 = (assign1020_e734 / locals.var_nfabot_i);
        let assign1020_e737: f64 = (locals.var_auxt).powf(assign1020_e736);
        let assign1020_e741: f64 = (locals.var_phigrbot * locals.var_phitrinv);
        let assign1020_e744: f64 = (locals.var_phigdbot * locals.var_phitdinv);
        let assign1020_e745: f64 = (assign1020_e741 - assign1020_e744);
        let assign1020_e746: f64 = (0.5 * assign1020_e745);
        let assign1020_e748: f64 = (assign1020_e746 / locals.var_nfabot_i);
        let assign1020_e749: f64 = (assign1020_e748).exp();
        let assign1020_e750: f64 = (assign1020_e737 * assign1020_e749);
        locals.var_ftdbot2 = assign1020_e750;

        let assign1030_e754: f64 = (locals.var_xti_i / 2.0);
        let assign1030_e756: f64 = (assign1030_e754 / locals.var_nfasti_i);
        let assign1030_e757: f64 = (locals.var_auxt).powf(assign1030_e756);
        let assign1030_e761: f64 = (locals.var_phigrsti * locals.var_phitrinv);
        let assign1030_e764: f64 = (locals.var_phigdsti * locals.var_phitdinv);
        let assign1030_e765: f64 = (assign1030_e761 - assign1030_e764);
        let assign1030_e766: f64 = (0.5 * assign1030_e765);
        let assign1030_e768: f64 = (assign1030_e766 / locals.var_nfasti_i);
        let assign1030_e769: f64 = (assign1030_e768).exp();
        let assign1030_e770: f64 = (assign1030_e757 * assign1030_e769);
        locals.var_ftdsti2 = assign1030_e770;

        let assign1040_e774: f64 = (locals.var_xti_i / 2.0);
        let assign1040_e776: f64 = (assign1040_e774 / locals.var_nfagat_i);
        let assign1040_e777: f64 = (locals.var_auxt).powf(assign1040_e776);
        let assign1040_e781: f64 = (locals.var_phigrgat * locals.var_phitrinv);
        let assign1040_e784: f64 = (locals.var_phigdgat * locals.var_phitdinv);
        let assign1040_e785: f64 = (assign1040_e781 - assign1040_e784);
        let assign1040_e786: f64 = (0.5 * assign1040_e785);
        let assign1040_e788: f64 = (assign1040_e786 / locals.var_nfagat_i);
        let assign1040_e789: f64 = (assign1040_e788).exp();
        let assign1040_e790: f64 = (assign1040_e777 * assign1040_e789);
        locals.var_ftdgat2 = assign1040_e790;

        let assign1050_e793: f64 = (locals.var_idsatrbot_i * locals.var_ftdbot2);
        let assign1050_e795: f64 = (assign1050_e793 * locals.var_ftdbot2);
        locals.var_idsatbot = assign1050_e795;

        let assign1060_e798: f64 = (locals.var_idsatrsti_i * locals.var_ftdsti2);
        let assign1060_e800: f64 = (assign1060_e798 * locals.var_ftdsti2);
        locals.var_idsatsti = assign1060_e800;

        let assign1070_e803: f64 = (locals.var_idsatrgat_i * locals.var_ftdgat2);
        let assign1070_e805: f64 = (assign1070_e803 * locals.var_ftdgat2);
        locals.var_idsatgat = assign1070_e805;

        let assign1780_e1250: f64 = (0.01 * locals.var_shrink_i);
        let assign1780_e1251: f64 = (1.0 - assign1780_e1250);
        locals.var_shrinkl = assign1780_e1251;

        let assign1860_e1316: f64 = (p.p87 * 1000000.0);
        locals.var_ndibot_i = assign1860_e1316;

        let assign1870_e1319: f64 = (p.p89 * 1000000.0);
        locals.var_ndisti_i = assign1870_e1319;

        let assign1880_e1322: f64 = (p.p88 * 1000000.0);
        locals.var_ndigat_i = assign1880_e1322;

        locals.var_ndi_i = locals.var_ndibot_i;

        locals.var_njl = locals.var_nfabot_i;

        let assign1910_e1327: f64 = (1450.0 * 0.0001);
        locals.var_muen_i = assign1910_e1327;

        let assign1920_e1330: f64 = (500.0 * 0.0001);
        locals.var_muep_i = assign1920_e1330;

        locals.var_pb = 0.6;

        locals.var_juncdlt = 0.001;

        let assign1950_e1335: f64 = (1.45e16 * locals.var_ftdbot2);
        locals.var_nin = assign1950_e1335;

        let assign1960_e1338: f64 = (locals.var_nin * locals.var_nin);
        let assign1960_e1340: f64 = (assign1960_e1338 / locals.var_ndi_i);
        locals.var_pn0 = assign1960_e1340;

        let assign1970_e1343: f64 = (-1.5);
        let assign1970_e1344: f64 = (locals.var_auxt).powf(assign1970_e1343);
        locals.var_t1 = assign1970_e1344;

        let assign1980_e1347: f64 = (locals.var_muen_i * locals.var_t1);
        let assign1980_e1349: f64 = (assign1980_e1347 / locals.var_phitdinv);
        locals.var_dn = assign1980_e1349;

        let assign1990_e1352: f64 = (locals.var_muep_i * locals.var_t1);
        let assign1990_e1354: f64 = (assign1990_e1352 / locals.var_phitdinv);
        locals.var_dp = assign1990_e1354;

        let assign2000_e1357: f64 = (2.0 * locals.var_dn);
        let assign2000_e1359: f64 = (assign2000_e1357 * locals.var_dp);
        let assign2000_e1362: f64 = (locals.var_dn + locals.var_dp);
        let assign2000_e1363: f64 = (assign2000_e1359 / assign2000_e1362);
        locals.var_da = assign2000_e1363;

        let assign2010_e1366: f64 = (locals.var_auxt).powf(p.p97);
        locals.var_t2 = assign2010_e1366;

        let assign2020_e1369: f64 = (p.p93 * locals.var_t2);
        locals.var_tau_hl = assign2020_e1369;

        let assign2030_e1372: f64 = (locals.var_tau_hl * locals.var_da);
        let assign2030_e1373: f64 = (assign2030_e1372).sqrt();
        locals.var_la = assign2030_e1373;

        let assign2040_e1376: f64 = (locals.var_njl / locals.var_phitdinv);
        let assign2040_e1379: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign2040_e1380: f64 = (assign2040_e1379).ln();
        let assign2040_e1381: f64 = (assign2040_e1376 * assign2040_e1380);
        locals.var_v_ha = assign2040_e1381;

        let assign2050_e1384: f64 = (locals.var_njl / locals.var_phitdinv);
        let assign2050_e1387: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign2050_e1388: f64 = (assign2050_e1387).ln();
        let assign2050_e1391: f64 = (p.p94 / locals.var_la);
        let assign2050_e1392: f64 = (assign2050_e1388 + assign2050_e1391);
        let assign2050_e1393: f64 = (assign2050_e1384 * assign2050_e1392);
        locals.var_v_hk = assign2050_e1393;

        let (assign2060_e1399,) = {
    if (p.p99 > 0.0) {
        (p.p99,)
    } else {
        (0.0,)
    }
};
        let assign2060_e1401: f64 = (assign2060_e1399 * locals.var_scale_i);
        let assign2060_e1403: f64 = (assign2060_e1401 * locals.var_scale_i);
        let assign2060_e1405: f64 = (assign2060_e1403 * locals.var_shrinkl);
        let assign2060_e1407: f64 = (assign2060_e1405 * locals.var_shrinkl);
        locals.var_ab_i = assign2060_e1407;

        let (assign2070_e1413,) = {
    if (p.p100 > 0.0) {
        (p.p100,)
    } else {
        (0.0,)
    }
};
        let assign2070_e1415: f64 = (assign2070_e1413 * locals.var_scale_i);
        let assign2070_e1417: f64 = (assign2070_e1415 * locals.var_shrinkl);
        locals.var_ls_i = assign2070_e1417;

        let (assign2080_e1423,) = {
    if (p.p101 > 0.0) {
        (p.p101,)
    } else {
        (0.0,)
    }
};
        let assign2080_e1425: f64 = (assign2080_e1423 * locals.var_scale_i);
        let assign2080_e1427: f64 = (assign2080_e1425 * locals.var_shrinkl);
        locals.var_lg_i = assign2080_e1427;

        locals.var_exp_vmax_over_phitd_bot = 0.0;
        locals.var_exp_vmax_over_phitd_bot_dn0 = 0.0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = 0.0;

        let assign2130_e1434: f64 = (locals.var_idsatbot * locals.var_ab_i);
        let assign2130_e1436: f64 = if assign2130_e1434 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign2130_e1436;

        let (assign2140_e1451,) = {
    if (locals.var_guard6 != 0.0) {
        let assign2140_e1442: f64 = (locals.var_idsatbot * locals.var_ab_i);
        let assign2140_e1443: f64 = (locals.var_imax_i / assign2140_e1442);
        let assign2140_e1445: f64 = (assign2140_e1443 + 1.0);
        let assign2140_e1446: f64 = (assign2140_e1445).ln();
        let assign2140_e1447: f64 = (locals.var_phitd * assign2140_e1446);
        let assign2140_e1449: f64 = (assign2140_e1447 * locals.var_nfabot_i);
        (assign2140_e1449,)
    } else {
        (locals.var_vmaxbot,)
    }
};
        locals.var_vmaxbot = assign2140_e1451;

        let (assign2150_e1456,) = {
    if (locals.var_guard6 == 0.0) {
        (100000000.0,)
    } else {
        (locals.var_vmaxbot,)
    }
};
        locals.var_vmaxbot = assign2150_e1456;

        let assign2160_e1459: f64 = (locals.var_idsatsti * locals.var_ls_i);
        let assign2160_e1461: f64 = if assign2160_e1459 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign2160_e1461;

        let (assign2170_e1476,) = {
    if (locals.var_guard7 != 0.0) {
        let assign2170_e1467: f64 = (locals.var_idsatsti * locals.var_ls_i);
        let assign2170_e1468: f64 = (locals.var_imax_i / assign2170_e1467);
        let assign2170_e1470: f64 = (assign2170_e1468 + 1.0);
        let assign2170_e1471: f64 = (assign2170_e1470).ln();
        let assign2170_e1472: f64 = (locals.var_phitd * assign2170_e1471);
        let assign2170_e1474: f64 = (assign2170_e1472 * locals.var_nfasti_i);
        (assign2170_e1474,)
    } else {
        (locals.var_vmaxsti,)
    }
};
        locals.var_vmaxsti = assign2170_e1476;

        let (assign2180_e1481,) = {
    if (locals.var_guard7 == 0.0) {
        (100000000.0,)
    } else {
        (locals.var_vmaxsti,)
    }
};
        locals.var_vmaxsti = assign2180_e1481;

        let assign2190_e1484: f64 = (locals.var_idsatgat * locals.var_lg_i);
        let assign2190_e1486: f64 = if assign2190_e1484 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign2190_e1486;

        let (assign2200_e1501,) = {
    if (locals.var_guard8 != 0.0) {
        let assign2200_e1492: f64 = (locals.var_idsatgat * locals.var_lg_i);
        let assign2200_e1493: f64 = (locals.var_imax_i / assign2200_e1492);
        let assign2200_e1495: f64 = (assign2200_e1493 + 1.0);
        let assign2200_e1496: f64 = (assign2200_e1495).ln();
        let assign2200_e1497: f64 = (locals.var_phitd * assign2200_e1496);
        let assign2200_e1499: f64 = (assign2200_e1497 * locals.var_nfagat_i);
        (assign2200_e1499,)
    } else {
        (locals.var_vmaxgat,)
    }
};
        locals.var_vmaxgat = assign2200_e1501;

        let (assign2210_e1506,) = {
    if (locals.var_guard8 == 0.0) {
        (100000000.0,)
    } else {
        (locals.var_vmaxgat,)
    }
};
        locals.var_vmaxgat = assign2210_e1506;

        let assign2220_e1509: f64 = (locals.var_vmaxbot).min(locals.var_vmaxsti);
        let assign2220_e1511: f64 = (assign2220_e1509).min(locals.var_vmaxgat);
        locals.var_vmax = assign2220_e1511;

        locals.var_idmultbot = 0.0;
        locals.var_idmultbot_dn0 = 0.0;
        locals.var_idmultbot_dn2 = 0.0;

        locals.var_iwnqs0_a = 0.0;
        locals.var_iwnqs0_a_dn0 = 0.0;
        locals.var_iwnqs0_a_dn2 = 0.0;
        locals.var_iwnqs0_a_dn5 = 0.0;

        locals.var_inqs0_a = 0.0;
        locals.var_inqs0_a_dn0 = 0.0;
        locals.var_inqs0_a_dn2 = 0.0;
        locals.var_inqs0_a_dn3 = 0.0;

        locals.var_inqs0_k = 0.0;
        locals.var_inqs0_k_dn0 = 0.0;
        locals.var_inqs0_k_dn2 = 0.0;
        locals.var_inqs0_k_dn4 = 0.0;

        locals.var_q_nqs_a = 0.0;
        locals.var_q_nqs_a_dn3 = 0.0;

        locals.var_q_nqs_k = 0.0;
        locals.var_q_nqs_k_dn4 = 0.0;

        locals.var_w_nqs_a = 0.0;
        locals.var_w_nqs_a_dn5 = 0.0;

        let assign3120_e1859: f64 = (1.6021918e-19 * locals.var_ab_i);
        locals.var_q_pex0 = assign3120_e1859;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3130_e1862: f64 = (2.0 * locals.var_epssi);
        let assign3130_e1865: f64 = (1.6021918e-19 * locals.var_ndi_i);
        let assign3130_e1866: f64 = (assign3130_e1862 / assign3130_e1865);
        let assign3130_e1867: f64 = (assign3130_e1866).sqrt();
        locals.var_w_depa0 = assign3130_e1867;
        locals.var_w_depa0_dn0 = 0.0;
        locals.var_w_depa0_dn2 = 0.0;

        let assign3140_e1870: f64 = (p.p94 - locals.var_w_depa0);
        let assign3140_e1872: f64 = (assign3140_e1870 - 1e-7);
        locals.var_tmf1 = assign3140_e1872;
        locals.var_tmf1_dn0 = (-locals.var_w_depa0_dn0);
        locals.var_tmf1_dn2 = (-locals.var_w_depa0_dn2);

        let assign3150_e1875: f64 = (4.0 * p.p94);
        let assign3150_e1877: f64 = (assign3150_e1875 * 1e-7);
        locals.var_tmf2 = assign3150_e1877;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;

        let (assign3160_e1884, assign3160_e1884_d_n0, assign3160_e1884_d_n2,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    } else {
        let assign3160_e1883: f64 = (-locals.var_tmf2);
        (assign3160_e1883, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
    }
};
        locals.var_tmf2 = assign3160_e1884;
        locals.var_tmf2_dn0 = assign3160_e1884_d_n0;
        locals.var_tmf2_dn2 = assign3160_e1884_d_n2;

        let assign3170_e1887: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign3170_e1889: f64 = (assign3170_e1887 + locals.var_tmf2);
        let assign3170_e1890: f64 = (assign3170_e1889).sqrt();
        locals.var_tmf2 = assign3170_e1890;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign3170_e1890));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign3170_e1890));

        let assign3180_e1895: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign3180_e1896: f64 = (0.5 * assign3180_e1895);
        let assign3180_e1897: f64 = (p.p94 - assign3180_e1896);
        locals.var_w_depa0 = assign3180_e1897;
        locals.var_w_depa0_dn0 = (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_w_depa0_dn2 = (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));

        let assign3190_e1900: f64 = if locals.var_swjunexp_i > 0.9 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign3190_e1900;

        let assign3200_e1903: f64 = (locals.var_nfabot_i - locals.var_nfagat_i);
        let assign3200_e1904: f64 = (assign3200_e1903).abs();
        let assign3200_e1917: f64 = (locals.var_nfabot_i - locals.var_nfasti_i);
        let assign3200_e1918: f64 = (assign3200_e1917).abs();
        let assign3200_e1932: f64 = (locals.var_nfagat_i - locals.var_nfasti_i);
        let assign3200_e1933: f64 = (assign3200_e1932).abs();
        let assign3200_e1944: f64 = if (((((assign3200_e1904 > 1e-6) && (locals.var_ab_i > 0.0)) && (locals.var_lg_i > 0.0)) || (((assign3200_e1918 > 1e-6) && (locals.var_ab_i > 0.0)) && (locals.var_ls_i > 0.0))) || (((assign3200_e1933 > 1e-6) && (locals.var_lg_i > 0.0)) && (locals.var_ls_i > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3200_e1944;

        let (assign3210_e1950,) = {
    if ((locals.var_guard26 != 0.0) && (locals.var_guard27 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_swjunexp_i,)
    }
};
        locals.var_swjunexp_i = assign3210_e1950;

        let assign3280_e1989: f64 = if locals.var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3280_e1989;

        let (assign3380_e2029, assign3380_e2029_d_n0, assign3380_e2029_d_n2,) = {
    if (locals.var_guard31 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vak, locals.var_vak_dn0, locals.var_vak_dn2,)
    }
};
        locals.var_vak = assign3380_e2029;
        locals.var_vak_dn0 = assign3380_e2029_d_n0;
        locals.var_vak_dn2 = assign3380_e2029_d_n2;

        let (assign3770_e2185,) = {
    if (locals.var_guard31 != 0.0) {
        (0.4,)
    } else {
        (locals.var_fracna,)
    }
};
        locals.var_fracna = assign3770_e2185;

        let (assign3780_e2189,) = {
    if (locals.var_guard31 != 0.0) {
        (0.65,)
    } else {
        (locals.var_fracnb,)
    }
};
        locals.var_fracnb = assign3780_e2189;

        let (assign3790_e2193,) = {
    if (locals.var_guard31 != 0.0) {
        (0.8,)
    } else {
        (locals.var_fraci,)
    }
};
        locals.var_fraci = assign3790_e2193;

        let (assign3800_e2200,) = {
    if (locals.var_guard31 != 0.0) {
        let assign3800_e2196: f64 = (-locals.var_fracna);
        let assign3800_e2198: f64 = (assign3800_e2196 * locals.var_vjunref_i);
        (assign3800_e2198,)
    } else {
        (locals.var_v1,)
    }
};
        locals.var_v1 = assign3800_e2200;

        let (assign3810_e2207,) = {
    if (locals.var_guard31 != 0.0) {
        let assign3810_e2203: f64 = (-locals.var_fracnb);
        let assign3810_e2205: f64 = (assign3810_e2203 * locals.var_vjunref_i);
        (assign3810_e2205,)
    } else {
        (locals.var_v2,)
    }
};
        locals.var_v2 = assign3810_e2207;

        let (assign3820_e2214,) = {
    if (locals.var_guard31 != 0.0) {
        let assign3820_e2210: f64 = (-locals.var_fraci);
        let assign3820_e2212: f64 = (assign3820_e2210 * locals.var_vjunref_i);
        (assign3820_e2212,)
    } else {
        (locals.var_v3,)
    }
};
        locals.var_v3 = assign3820_e2214;

        let (assign3830_e2218,) = {
    if (locals.var_guard31 != 0.0) {
        (0.1,)
    } else {
        (locals.var_v4,)
    }
};
        locals.var_v4 = assign3830_e2218;

        let (assign3840_e2222,) = {
    if (locals.var_guard31 != 0.0) {
        (0.2,)
    } else {
        (locals.var_v5,)
    }
};
        locals.var_v5 = assign3840_e2222;

        let assign3850_e2234: f64 = if (!(((locals.var_ab_i == 0.0) && (locals.var_ls_i == 0.0)) && (locals.var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign3850_e2234;

        let assign3930_e2306: f64 = if locals.var_v1 < locals.var_vmax { 1.0 } else { 0.0 };
        locals.var_guard77 = assign3930_e2306;

        let (assign3990_e2447,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign3990_e2443: f64 = (locals.var_nin * locals.var_nin);
        let assign3990_e2445: f64 = (assign3990_e2443 / locals.var_ndibot_i);
        (assign3990_e2445,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign3990_e2447;

        let (assign4000_e2462,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign4000_e2455: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign4000_e2458: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign4000_e2459: f64 = (assign4000_e2458).ln();
        let assign4000_e2460: f64 = (assign4000_e2455 * assign4000_e2459);
        (assign4000_e2460,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign4000_e2462;

        let assign4010_e2465: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign4010_e2465;

        let (assign4020_e2481, assign4020_e2481_d_n0, assign4020_e2481_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4020_e2476: f64 = (locals.var_v1 - locals.var_vha1);
        let assign4020_e2477: f64 = (p.p86 * assign4020_e2476);
        let assign4020_e2479: f64 = (assign4020_e2477 + locals.var_nfabot_i);
        (assign4020_e2479, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign4020_e2481;
        locals.var_nja10_dn0 = assign4020_e2481_d_n0;
        locals.var_nja10_dn2 = assign4020_e2481_d_n2;

        let (assign4030_e2495, assign4030_e2495_d_n0, assign4030_e2495_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4030_e2492: f64 = (p.p86 * locals.var_vha1);
        let assign4030_e2493: f64 = (locals.var_nfabot_i - assign4030_e2492);
        (assign4030_e2493, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4030_e2495;
        locals.var_nj0_dn0 = assign4030_e2495_d_n0;
        locals.var_nj0_dn2 = assign4030_e2495_d_n2;

        let (assign4040_e2509, assign4040_e2509_d_n0, assign4040_e2509_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4040_e2505: f64 = (p.p85 - locals.var_nja10);
        let assign4040_e2507: f64 = (assign4040_e2505 - 0.01);
        (assign4040_e2507, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4040_e2509;
        locals.var_tmf1_dn0 = assign4040_e2509_d_n0;
        locals.var_tmf1_dn2 = assign4040_e2509_d_n2;

        let (assign4050_e2523, assign4050_e2523_d_n0, assign4050_e2523_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4050_e2519: f64 = (4.0 * p.p85);
        let assign4050_e2521: f64 = (assign4050_e2519 * 0.01);
        (assign4050_e2521, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4050_e2523;
        locals.var_tmf2_dn0 = assign4050_e2523_d_n0;
        locals.var_tmf2_dn2 = assign4050_e2523_d_n2;

        let (assign4060_e2539, assign4060_e2539_d_n0, assign4060_e2539_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let (assign4060_e2537, assign4060_e2537_d_n0, assign4060_e2537_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4060_e2536: f64 = (-locals.var_tmf2);
                (assign4060_e2536, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4060_e2537, assign4060_e2537_d_n0, assign4060_e2537_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4060_e2539;
        locals.var_tmf2_dn0 = assign4060_e2539_d_n0;
        locals.var_tmf2_dn2 = assign4060_e2539_d_n2;

        let (assign4070_e2554, assign4070_e2554_d_n0, assign4070_e2554_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4070_e2549: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4070_e2551: f64 = (assign4070_e2549 + locals.var_tmf2);
        let assign4070_e2552: f64 = (assign4070_e2551).sqrt();
        (assign4070_e2552, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4070_e2552)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4070_e2552)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4070_e2554;
        locals.var_tmf2_dn0 = assign4070_e2554_d_n0;
        locals.var_tmf2_dn2 = assign4070_e2554_d_n2;

        let (assign4080_e2570, assign4080_e2570_d_n0, assign4080_e2570_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4080_e2566: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4080_e2567: f64 = (0.5 * assign4080_e2566);
        let assign4080_e2568: f64 = (p.p85 - assign4080_e2567);
        (assign4080_e2568, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign4080_e2570;
        locals.var_nja11_dn0 = assign4080_e2570_d_n0;
        locals.var_nja11_dn2 = assign4080_e2570_d_n2;

        let (assign4090_e2584, assign4090_e2584_d_n0, assign4090_e2584_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4090_e2580: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign4090_e2582: f64 = (assign4090_e2580 - 0.01);
        (assign4090_e2582, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4090_e2584;
        locals.var_tmf1_dn0 = assign4090_e2584_d_n0;
        locals.var_tmf1_dn2 = assign4090_e2584_d_n2;

        let (assign4100_e2598, assign4100_e2598_d_n0, assign4100_e2598_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4100_e2594: f64 = (4.0 * locals.var_nfabot_i);
        let assign4100_e2596: f64 = (assign4100_e2594 * 0.01);
        (assign4100_e2596, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4100_e2598;
        locals.var_tmf2_dn0 = assign4100_e2598_d_n0;
        locals.var_tmf2_dn2 = assign4100_e2598_d_n2;

        let (assign4110_e2614, assign4110_e2614_d_n0, assign4110_e2614_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let (assign4110_e2612, assign4110_e2612_d_n0, assign4110_e2612_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4110_e2611: f64 = (-locals.var_tmf2);
                (assign4110_e2611, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4110_e2612, assign4110_e2612_d_n0, assign4110_e2612_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4110_e2614;
        locals.var_tmf2_dn0 = assign4110_e2614_d_n0;
        locals.var_tmf2_dn2 = assign4110_e2614_d_n2;

        let (assign4120_e2629, assign4120_e2629_d_n0, assign4120_e2629_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4120_e2624: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4120_e2626: f64 = (assign4120_e2624 + locals.var_tmf2);
        let assign4120_e2627: f64 = (assign4120_e2626).sqrt();
        (assign4120_e2627, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4120_e2627)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4120_e2627)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4120_e2629;
        locals.var_tmf2_dn0 = assign4120_e2629_d_n0;
        locals.var_tmf2_dn2 = assign4120_e2629_d_n2;

        let (assign4130_e2645, assign4130_e2645_d_n0, assign4130_e2645_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4130_e2641: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4130_e2642: f64 = (0.5 * assign4130_e2641);
        let assign4130_e2643: f64 = (locals.var_nfabot_i + assign4130_e2642);
        (assign4130_e2643, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign4130_e2645;
        locals.var_nj1_dn0 = assign4130_e2645_d_n0;
        locals.var_nj1_dn2 = assign4130_e2645_d_n2;

        let (assign4140_e2659, assign4140_e2659_d_n0, assign4140_e2659_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4140_e2655: f64 = (p.p85 - locals.var_nj0);
        let assign4140_e2657: f64 = (assign4140_e2655 - 0.01);
        (assign4140_e2657, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4140_e2659;
        locals.var_tmf1_dn0 = assign4140_e2659_d_n0;
        locals.var_tmf1_dn2 = assign4140_e2659_d_n2;

        let (assign4150_e2673, assign4150_e2673_d_n0, assign4150_e2673_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4150_e2669: f64 = (4.0 * p.p85);
        let assign4150_e2671: f64 = (assign4150_e2669 * 0.01);
        (assign4150_e2671, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4150_e2673;
        locals.var_tmf2_dn0 = assign4150_e2673_d_n0;
        locals.var_tmf2_dn2 = assign4150_e2673_d_n2;

        let (assign4160_e2689, assign4160_e2689_d_n0, assign4160_e2689_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let (assign4160_e2687, assign4160_e2687_d_n0, assign4160_e2687_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4160_e2686: f64 = (-locals.var_tmf2);
                (assign4160_e2686, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4160_e2687, assign4160_e2687_d_n0, assign4160_e2687_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4160_e2689;
        locals.var_tmf2_dn0 = assign4160_e2689_d_n0;
        locals.var_tmf2_dn2 = assign4160_e2689_d_n2;

        let (assign4170_e2704, assign4170_e2704_d_n0, assign4170_e2704_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4170_e2699: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4170_e2701: f64 = (assign4170_e2699 + locals.var_tmf2);
        let assign4170_e2702: f64 = (assign4170_e2701).sqrt();
        (assign4170_e2702, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4170_e2702)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4170_e2702)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4170_e2704;
        locals.var_tmf2_dn0 = assign4170_e2704_d_n0;
        locals.var_tmf2_dn2 = assign4170_e2704_d_n2;

        let (assign4180_e2720, assign4180_e2720_d_n0, assign4180_e2720_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4180_e2716: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4180_e2717: f64 = (0.5 * assign4180_e2716);
        let assign4180_e2718: f64 = (p.p85 - assign4180_e2717);
        (assign4180_e2718, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4180_e2720;
        locals.var_nj0_dn0 = assign4180_e2720_d_n0;
        locals.var_nj0_dn2 = assign4180_e2720_d_n2;

        let (assign4190_e2734, assign4190_e2734_d_n0, assign4190_e2734_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4190_e2730: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign4190_e2732: f64 = (assign4190_e2730 - 0.01);
        (assign4190_e2732, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4190_e2734;
        locals.var_tmf1_dn0 = assign4190_e2734_d_n0;
        locals.var_tmf1_dn2 = assign4190_e2734_d_n2;

        let (assign4200_e2748, assign4200_e2748_d_n0, assign4200_e2748_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4200_e2744: f64 = (4.0 * locals.var_nfabot_i);
        let assign4200_e2746: f64 = (assign4200_e2744 * 0.01);
        (assign4200_e2746, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4200_e2748;
        locals.var_tmf2_dn0 = assign4200_e2748_d_n0;
        locals.var_tmf2_dn2 = assign4200_e2748_d_n2;

        let (assign4210_e2764, assign4210_e2764_d_n0, assign4210_e2764_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let (assign4210_e2762, assign4210_e2762_d_n0, assign4210_e2762_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4210_e2761: f64 = (-locals.var_tmf2);
                (assign4210_e2761, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4210_e2762, assign4210_e2762_d_n0, assign4210_e2762_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4210_e2764;
        locals.var_tmf2_dn0 = assign4210_e2764_d_n0;
        locals.var_tmf2_dn2 = assign4210_e2764_d_n2;

        let (assign4220_e2779, assign4220_e2779_d_n0, assign4220_e2779_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4220_e2774: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4220_e2776: f64 = (assign4220_e2774 + locals.var_tmf2);
        let assign4220_e2777: f64 = (assign4220_e2776).sqrt();
        (assign4220_e2777, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4220_e2777)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4220_e2777)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4220_e2779;
        locals.var_tmf2_dn0 = assign4220_e2779_d_n0;
        locals.var_tmf2_dn2 = assign4220_e2779_d_n2;

        let (assign4230_e2795, assign4230_e2795_d_n0, assign4230_e2795_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign4230_e2791: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4230_e2792: f64 = (0.5 * assign4230_e2791);
        let assign4230_e2793: f64 = (locals.var_nfabot_i + assign4230_e2792);
        (assign4230_e2793, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4230_e2795;
        locals.var_nj0_dn0 = assign4230_e2795_d_n0;
        locals.var_nj0_dn2 = assign4230_e2795_d_n2;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4240_e2806, assign4240_e2806_d_n0, assign4240_e2806_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4240_e2806;
        locals.var_nj0_dn0 = assign4240_e2806_d_n0;
        locals.var_nj0_dn2 = assign4240_e2806_d_n2;

        let (assign4250_e2817, assign4250_e2817_d_n0, assign4250_e2817_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard80 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign4250_e2817;
        locals.var_nj1_dn0 = assign4250_e2817_d_n0;
        locals.var_nj1_dn2 = assign4250_e2817_d_n2;

        let assign4260_e2821: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4260_e2825: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4260_e2826: f64 = (locals.var_vha1 * assign4260_e2825);
        let assign4260_e2829: f64 = (locals.var_nj0 * p.p85);
        let assign4260_e2830: f64 = (assign4260_e2826 / assign4260_e2829);
        let assign4260_e2831: f64 = (assign4260_e2821 + assign4260_e2830);
        let assign4260_e2832: f64 = (locals.var_phitdinv * assign4260_e2831);
        let assign4260_e2833: f64 = (assign4260_e2832).abs();
        let assign4260_e2835: f64 = if assign4260_e2833 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign4260_e2835;

        let (assign4270_e2860, assign4270_e2860_d_n0, assign4270_e2860_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard81 != 0.0)) {
        let assign4270_e2846: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4270_e2850: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4270_e2851: f64 = (locals.var_vha1 * assign4270_e2850);
        let assign4270_e2854: f64 = (locals.var_nj0 * p.p85);
        let assign4270_e2855: f64 = (assign4270_e2851 / assign4270_e2854);
        let assign4270_e2856: f64 = (assign4270_e2846 + assign4270_e2855);
        let assign4270_e2857: f64 = (locals.var_phitdinv * assign4270_e2856);
        let assign4270_e2858: f64 = (assign4270_e2857).exp();
        (assign4270_e2858, (assign4270_e2858 * (locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign4270_e2854) - (assign4270_e2851 * (locals.var_nj0_dn0 * p.p85))) / (assign4270_e2854 * assign4270_e2854))))), (assign4270_e2858 * (locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign4270_e2854) - (assign4270_e2851 * (locals.var_nj0_dn2 * p.p85))) / (assign4270_e2854 * assign4270_e2854))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign4270_e2860;
        locals.var_idmultbot_dn0 = assign4270_e2860_d_n0;
        locals.var_idmultbot_dn2 = assign4270_e2860_d_n2;

        let assign4280_e2864: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4280_e2868: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4280_e2869: f64 = (locals.var_vha1 * assign4280_e2868);
        let assign4280_e2872: f64 = (locals.var_nj0 * p.p85);
        let assign4280_e2873: f64 = (assign4280_e2869 / assign4280_e2872);
        let assign4280_e2874: f64 = (assign4280_e2864 + assign4280_e2873);
        let assign4280_e2875: f64 = (locals.var_phitdinv * assign4280_e2874);
        let assign4280_e2877: f64 = (-230.25850929940458);
        let assign4280_e2878: f64 = if assign4280_e2875 < assign4280_e2877 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign4280_e2878;

        let (assign4290_e2958, assign4290_e2958_d_n0, assign4290_e2958_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        let assign4290_e2892: f64 = (-230.25850929940458);
        let assign4290_e2896: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4290_e2900: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4290_e2901: f64 = (locals.var_vha1 * assign4290_e2900);
        let assign4290_e2904: f64 = (locals.var_nj0 * p.p85);
        let assign4290_e2905: f64 = (assign4290_e2901 / assign4290_e2904);
        let assign4290_e2906: f64 = (assign4290_e2896 + assign4290_e2905);
        let assign4290_e2907: f64 = (locals.var_phitdinv * assign4290_e2906);
        let assign4290_e2908: f64 = (assign4290_e2892 - assign4290_e2907);
        let assign4290_e2912: f64 = (-230.25850929940458);
        let assign4290_e2916: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4290_e2920: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4290_e2921: f64 = (locals.var_vha1 * assign4290_e2920);
        let assign4290_e2924: f64 = (locals.var_nj0 * p.p85);
        let assign4290_e2925: f64 = (assign4290_e2921 / assign4290_e2924);
        let assign4290_e2926: f64 = (assign4290_e2916 + assign4290_e2925);
        let assign4290_e2927: f64 = (locals.var_phitdinv * assign4290_e2926);
        let assign4290_e2928: f64 = (assign4290_e2912 - assign4290_e2927);
        let assign4290_e2931: f64 = (-230.25850929940458);
        let assign4290_e2935: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4290_e2939: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4290_e2940: f64 = (locals.var_vha1 * assign4290_e2939);
        let assign4290_e2943: f64 = (locals.var_nj0 * p.p85);
        let assign4290_e2944: f64 = (assign4290_e2940 / assign4290_e2943);
        let assign4290_e2945: f64 = (assign4290_e2935 + assign4290_e2944);
        let assign4290_e2946: f64 = (locals.var_phitdinv * assign4290_e2945);
        let assign4290_e2947: f64 = (assign4290_e2931 - assign4290_e2946);
        let assign4290_e2949: f64 = (assign4290_e2947 * 0.3333333333333333);
        let assign4290_e2950: f64 = (1.0 + assign4290_e2949);
        let assign4290_e2951: f64 = (assign4290_e2928 * assign4290_e2950);
        let assign4290_e2952: f64 = (0.5 * assign4290_e2951);
        let assign4290_e2953: f64 = (1.0 + assign4290_e2952);
        let assign4290_e2954: f64 = (assign4290_e2908 * assign4290_e2953);
        let assign4290_e2955: f64 = (1.0 + assign4290_e2954);
        let assign4290_e2956: f64 = (1e-100 / assign4290_e2955);
        (assign4290_e2956, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign4290_e2904) - (assign4290_e2901 * (locals.var_nj0_dn0 * p.p85))) / (assign4290_e2904 * assign4290_e2904))))) * assign4290_e2953) + (assign4290_e2908 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign4290_e2924) - (assign4290_e2921 * (locals.var_nj0_dn0 * p.p85))) / (assign4290_e2924 * assign4290_e2924))))) * assign4290_e2950) + (assign4290_e2928 * ((-(locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign4290_e2943) - (assign4290_e2940 * (locals.var_nj0_dn0 * p.p85))) / (assign4290_e2943 * assign4290_e2943))))) * 0.3333333333333333))))))) / (assign4290_e2955 * assign4290_e2955))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign4290_e2904) - (assign4290_e2901 * (locals.var_nj0_dn2 * p.p85))) / (assign4290_e2904 * assign4290_e2904))))) * assign4290_e2953) + (assign4290_e2908 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign4290_e2924) - (assign4290_e2921 * (locals.var_nj0_dn2 * p.p85))) / (assign4290_e2924 * assign4290_e2924))))) * assign4290_e2950) + (assign4290_e2928 * ((-(locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign4290_e2943) - (assign4290_e2940 * (locals.var_nj0_dn2 * p.p85))) / (assign4290_e2943 * assign4290_e2943))))) * 0.3333333333333333))))))) / (assign4290_e2955 * assign4290_e2955))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign4290_e2958;
        locals.var_idmultbot_dn0 = assign4290_e2958_d_n0;
        locals.var_idmultbot_dn2 = assign4290_e2958_d_n2;

        let (assign4300_e3036, assign4300_e3036_d_n0, assign4300_e3036_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        let assign4300_e2975: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4300_e2979: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4300_e2980: f64 = (locals.var_vha1 * assign4300_e2979);
        let assign4300_e2983: f64 = (locals.var_nj0 * p.p85);
        let assign4300_e2984: f64 = (assign4300_e2980 / assign4300_e2983);
        let assign4300_e2985: f64 = (assign4300_e2975 + assign4300_e2984);
        let assign4300_e2986: f64 = (locals.var_phitdinv * assign4300_e2985);
        let assign4300_e2988: f64 = (assign4300_e2986 - 230.25850929940458);
        let assign4300_e2994: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4300_e2998: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4300_e2999: f64 = (locals.var_vha1 * assign4300_e2998);
        let assign4300_e3002: f64 = (locals.var_nj0 * p.p85);
        let assign4300_e3003: f64 = (assign4300_e2999 / assign4300_e3002);
        let assign4300_e3004: f64 = (assign4300_e2994 + assign4300_e3003);
        let assign4300_e3005: f64 = (locals.var_phitdinv * assign4300_e3004);
        let assign4300_e3007: f64 = (assign4300_e3005 - 230.25850929940458);
        let assign4300_e3012: f64 = (locals.var_v1 / locals.var_nj1);
        let assign4300_e3016: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign4300_e3017: f64 = (locals.var_vha1 * assign4300_e3016);
        let assign4300_e3020: f64 = (locals.var_nj0 * p.p85);
        let assign4300_e3021: f64 = (assign4300_e3017 / assign4300_e3020);
        let assign4300_e3022: f64 = (assign4300_e3012 + assign4300_e3021);
        let assign4300_e3023: f64 = (locals.var_phitdinv * assign4300_e3022);
        let assign4300_e3025: f64 = (assign4300_e3023 - 230.25850929940458);
        let assign4300_e3027: f64 = (assign4300_e3025 * 0.3333333333333333);
        let assign4300_e3028: f64 = (1.0 + assign4300_e3027);
        let assign4300_e3029: f64 = (assign4300_e3007 * assign4300_e3028);
        let assign4300_e3030: f64 = (0.5 * assign4300_e3029);
        let assign4300_e3031: f64 = (1.0 + assign4300_e3030);
        let assign4300_e3032: f64 = (assign4300_e2988 * assign4300_e3031);
        let assign4300_e3033: f64 = (1.0 + assign4300_e3032);
        let assign4300_e3034: f64 = (1e100 * assign4300_e3033);
        (assign4300_e3034, (1e100 * (((locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign4300_e2983) - (assign4300_e2980 * (locals.var_nj0_dn0 * p.p85))) / (assign4300_e2983 * assign4300_e2983)))) * assign4300_e3031) + (assign4300_e2988 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign4300_e3002) - (assign4300_e2999 * (locals.var_nj0_dn0 * p.p85))) / (assign4300_e3002 * assign4300_e3002)))) * assign4300_e3028) + (assign4300_e3007 * ((locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign4300_e3020) - (assign4300_e3017 * (locals.var_nj0_dn0 * p.p85))) / (assign4300_e3020 * assign4300_e3020)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign4300_e2983) - (assign4300_e2980 * (locals.var_nj0_dn2 * p.p85))) / (assign4300_e2983 * assign4300_e2983)))) * assign4300_e3031) + (assign4300_e2988 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign4300_e3002) - (assign4300_e2999 * (locals.var_nj0_dn2 * p.p85))) / (assign4300_e3002 * assign4300_e3002)))) * assign4300_e3028) + (assign4300_e3007 * ((locals.var_phitdinv * ((-((locals.var_v1 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign4300_e3020) - (assign4300_e3017 * (locals.var_nj0_dn2 * p.p85))) / (assign4300_e3020 * assign4300_e3020)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign4300_e3036;
        locals.var_idmultbot_dn0 = assign4300_e3036_d_n0;
        locals.var_idmultbot_dn2 = assign4300_e3036_d_n2;

        let (assign4310_e3048,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign4310_e3044: f64 = (locals.var_nin * locals.var_nin);
        let assign4310_e3046: f64 = (assign4310_e3044 / locals.var_ndisti_i);
        (assign4310_e3046,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign4310_e3048;

        let (assign4320_e3063,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign4320_e3056: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign4320_e3059: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign4320_e3060: f64 = (assign4320_e3059).ln();
        let assign4320_e3061: f64 = (assign4320_e3056 * assign4320_e3060);
        (assign4320_e3061,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign4320_e3063;

        let assign4330_e3066: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign4330_e3066;

        let (assign4340_e3082, assign4340_e3082_d_n0, assign4340_e3082_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4340_e3077: f64 = (locals.var_v1 - locals.var_vha1);
        let assign4340_e3078: f64 = (p.p86 * assign4340_e3077);
        let assign4340_e3080: f64 = (assign4340_e3078 + locals.var_nfasti_i);
        (assign4340_e3080, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign4340_e3082;
        locals.var_nja10_dn0 = assign4340_e3082_d_n0;
        locals.var_nja10_dn2 = assign4340_e3082_d_n2;

        let (assign4350_e3096, assign4350_e3096_d_n0, assign4350_e3096_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4350_e3093: f64 = (p.p86 * locals.var_vha1);
        let assign4350_e3094: f64 = (locals.var_nfasti_i - assign4350_e3093);
        (assign4350_e3094, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4350_e3096;
        locals.var_nj0_dn0 = assign4350_e3096_d_n0;
        locals.var_nj0_dn2 = assign4350_e3096_d_n2;

        let (assign4360_e3110, assign4360_e3110_d_n0, assign4360_e3110_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4360_e3106: f64 = (p.p85 - locals.var_nja10);
        let assign4360_e3108: f64 = (assign4360_e3106 - 0.01);
        (assign4360_e3108, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4360_e3110;
        locals.var_tmf1_dn0 = assign4360_e3110_d_n0;
        locals.var_tmf1_dn2 = assign4360_e3110_d_n2;

        let (assign4370_e3124, assign4370_e3124_d_n0, assign4370_e3124_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4370_e3120: f64 = (4.0 * p.p85);
        let assign4370_e3122: f64 = (assign4370_e3120 * 0.01);
        (assign4370_e3122, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4370_e3124;
        locals.var_tmf2_dn0 = assign4370_e3124_d_n0;
        locals.var_tmf2_dn2 = assign4370_e3124_d_n2;

        let (assign4380_e3140, assign4380_e3140_d_n0, assign4380_e3140_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let (assign4380_e3138, assign4380_e3138_d_n0, assign4380_e3138_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4380_e3137: f64 = (-locals.var_tmf2);
                (assign4380_e3137, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4380_e3138, assign4380_e3138_d_n0, assign4380_e3138_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4380_e3140;
        locals.var_tmf2_dn0 = assign4380_e3140_d_n0;
        locals.var_tmf2_dn2 = assign4380_e3140_d_n2;

        let (assign4390_e3155, assign4390_e3155_d_n0, assign4390_e3155_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4390_e3150: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4390_e3152: f64 = (assign4390_e3150 + locals.var_tmf2);
        let assign4390_e3153: f64 = (assign4390_e3152).sqrt();
        (assign4390_e3153, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4390_e3153)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4390_e3153)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4390_e3155;
        locals.var_tmf2_dn0 = assign4390_e3155_d_n0;
        locals.var_tmf2_dn2 = assign4390_e3155_d_n2;

        let (assign4400_e3171, assign4400_e3171_d_n0, assign4400_e3171_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4400_e3167: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4400_e3168: f64 = (0.5 * assign4400_e3167);
        let assign4400_e3169: f64 = (p.p85 - assign4400_e3168);
        (assign4400_e3169, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign4400_e3171;
        locals.var_nja11_dn0 = assign4400_e3171_d_n0;
        locals.var_nja11_dn2 = assign4400_e3171_d_n2;

        let (assign4410_e3185, assign4410_e3185_d_n0, assign4410_e3185_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4410_e3181: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign4410_e3183: f64 = (assign4410_e3181 - 0.01);
        (assign4410_e3183, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4410_e3185;
        locals.var_tmf1_dn0 = assign4410_e3185_d_n0;
        locals.var_tmf1_dn2 = assign4410_e3185_d_n2;

        let (assign4420_e3199, assign4420_e3199_d_n0, assign4420_e3199_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4420_e3195: f64 = (4.0 * locals.var_nfasti_i);
        let assign4420_e3197: f64 = (assign4420_e3195 * 0.01);
        (assign4420_e3197, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4420_e3199;
        locals.var_tmf2_dn0 = assign4420_e3199_d_n0;
        locals.var_tmf2_dn2 = assign4420_e3199_d_n2;

        let (assign4430_e3215, assign4430_e3215_d_n0, assign4430_e3215_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let (assign4430_e3213, assign4430_e3213_d_n0, assign4430_e3213_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4430_e3212: f64 = (-locals.var_tmf2);
                (assign4430_e3212, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4430_e3213, assign4430_e3213_d_n0, assign4430_e3213_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4430_e3215;
        locals.var_tmf2_dn0 = assign4430_e3215_d_n0;
        locals.var_tmf2_dn2 = assign4430_e3215_d_n2;

        let (assign4440_e3230, assign4440_e3230_d_n0, assign4440_e3230_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4440_e3225: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4440_e3227: f64 = (assign4440_e3225 + locals.var_tmf2);
        let assign4440_e3228: f64 = (assign4440_e3227).sqrt();
        (assign4440_e3228, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4440_e3228)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4440_e3228)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4440_e3230;
        locals.var_tmf2_dn0 = assign4440_e3230_d_n0;
        locals.var_tmf2_dn2 = assign4440_e3230_d_n2;

        let (assign4450_e3246, assign4450_e3246_d_n0, assign4450_e3246_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4450_e3242: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4450_e3243: f64 = (0.5 * assign4450_e3242);
        let assign4450_e3244: f64 = (locals.var_nfasti_i + assign4450_e3243);
        (assign4450_e3244, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign4450_e3246;
        locals.var_nj1_dn0 = assign4450_e3246_d_n0;
        locals.var_nj1_dn2 = assign4450_e3246_d_n2;

        let (assign4460_e3260, assign4460_e3260_d_n0, assign4460_e3260_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4460_e3256: f64 = (p.p85 - locals.var_nj0);
        let assign4460_e3258: f64 = (assign4460_e3256 - 0.01);
        (assign4460_e3258, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4460_e3260;
        locals.var_tmf1_dn0 = assign4460_e3260_d_n0;
        locals.var_tmf1_dn2 = assign4460_e3260_d_n2;

        let (assign4470_e3274, assign4470_e3274_d_n0, assign4470_e3274_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4470_e3270: f64 = (4.0 * p.p85);
        let assign4470_e3272: f64 = (assign4470_e3270 * 0.01);
        (assign4470_e3272, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4470_e3274;
        locals.var_tmf2_dn0 = assign4470_e3274_d_n0;
        locals.var_tmf2_dn2 = assign4470_e3274_d_n2;

        let (assign4480_e3290, assign4480_e3290_d_n0, assign4480_e3290_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let (assign4480_e3288, assign4480_e3288_d_n0, assign4480_e3288_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4480_e3287: f64 = (-locals.var_tmf2);
                (assign4480_e3287, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4480_e3288, assign4480_e3288_d_n0, assign4480_e3288_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4480_e3290;
        locals.var_tmf2_dn0 = assign4480_e3290_d_n0;
        locals.var_tmf2_dn2 = assign4480_e3290_d_n2;

        let (assign4490_e3305, assign4490_e3305_d_n0, assign4490_e3305_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4490_e3300: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4490_e3302: f64 = (assign4490_e3300 + locals.var_tmf2);
        let assign4490_e3303: f64 = (assign4490_e3302).sqrt();
        (assign4490_e3303, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4490_e3303)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4490_e3303)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4490_e3305;
        locals.var_tmf2_dn0 = assign4490_e3305_d_n0;
        locals.var_tmf2_dn2 = assign4490_e3305_d_n2;

        let (assign4500_e3321, assign4500_e3321_d_n0, assign4500_e3321_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4500_e3317: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4500_e3318: f64 = (0.5 * assign4500_e3317);
        let assign4500_e3319: f64 = (p.p85 - assign4500_e3318);
        (assign4500_e3319, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4500_e3321;
        locals.var_nj0_dn0 = assign4500_e3321_d_n0;
        locals.var_nj0_dn2 = assign4500_e3321_d_n2;

        let (assign4510_e3335, assign4510_e3335_d_n0, assign4510_e3335_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4510_e3331: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign4510_e3333: f64 = (assign4510_e3331 - 0.01);
        (assign4510_e3333, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4510_e3335;
        locals.var_tmf1_dn0 = assign4510_e3335_d_n0;
        locals.var_tmf1_dn2 = assign4510_e3335_d_n2;

        let (assign4520_e3349, assign4520_e3349_d_n0, assign4520_e3349_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4520_e3345: f64 = (4.0 * locals.var_nfasti_i);
        let assign4520_e3347: f64 = (assign4520_e3345 * 0.01);
        (assign4520_e3347, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4520_e3349;
        locals.var_tmf2_dn0 = assign4520_e3349_d_n0;
        locals.var_tmf2_dn2 = assign4520_e3349_d_n2;

        let (assign4530_e3365, assign4530_e3365_d_n0, assign4530_e3365_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let (assign4530_e3363, assign4530_e3363_d_n0, assign4530_e3363_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4530_e3362: f64 = (-locals.var_tmf2);
                (assign4530_e3362, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4530_e3363, assign4530_e3363_d_n0, assign4530_e3363_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4530_e3365;
        locals.var_tmf2_dn0 = assign4530_e3365_d_n0;
        locals.var_tmf2_dn2 = assign4530_e3365_d_n2;

        let (assign4540_e3380, assign4540_e3380_d_n0, assign4540_e3380_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4540_e3375: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4540_e3377: f64 = (assign4540_e3375 + locals.var_tmf2);
        let assign4540_e3378: f64 = (assign4540_e3377).sqrt();
        (assign4540_e3378, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4540_e3378)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4540_e3378)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4540_e3380;
        locals.var_tmf2_dn0 = assign4540_e3380_d_n0;
        locals.var_tmf2_dn2 = assign4540_e3380_d_n2;

        let (assign4550_e3396, assign4550_e3396_d_n0, assign4550_e3396_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 != 0.0)) {
        let assign4550_e3392: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4550_e3393: f64 = (0.5 * assign4550_e3392);
        let assign4550_e3394: f64 = (locals.var_nfasti_i + assign4550_e3393);
        (assign4550_e3394, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4550_e3396;
        locals.var_nj0_dn0 = assign4550_e3396_d_n0;
        locals.var_nj0_dn2 = assign4550_e3396_d_n2;

        let (assign4560_e3407, assign4560_e3407_d_n0, assign4560_e3407_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4560_e3407;
        locals.var_nj0_dn0 = assign4560_e3407_d_n0;
        locals.var_nj0_dn2 = assign4560_e3407_d_n2;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4570_e3418, assign4570_e3418_d_n0, assign4570_e3418_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard83 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign4570_e3418;
        locals.var_nj1_dn0 = assign4570_e3418_d_n0;
        locals.var_nj1_dn2 = assign4570_e3418_d_n2;

        let (assign4630_e3649,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign4630_e3645: f64 = (locals.var_nin * locals.var_nin);
        let assign4630_e3647: f64 = (assign4630_e3645 / locals.var_ndigat_i);
        (assign4630_e3647,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign4630_e3649;

        let (assign4640_e3664,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign4640_e3657: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign4640_e3660: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign4640_e3661: f64 = (assign4640_e3660).ln();
        let assign4640_e3662: f64 = (assign4640_e3657 * assign4640_e3661);
        (assign4640_e3662,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign4640_e3664;

        let assign4650_e3667: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign4650_e3667;

        let (assign4660_e3683, assign4660_e3683_d_n0, assign4660_e3683_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4660_e3678: f64 = (locals.var_v1 - locals.var_vha1);
        let assign4660_e3679: f64 = (p.p86 * assign4660_e3678);
        let assign4660_e3681: f64 = (assign4660_e3679 + locals.var_nfagat_i);
        (assign4660_e3681, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign4660_e3683;
        locals.var_nja10_dn0 = assign4660_e3683_d_n0;
        locals.var_nja10_dn2 = assign4660_e3683_d_n2;

        let (assign4670_e3697, assign4670_e3697_d_n0, assign4670_e3697_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4670_e3694: f64 = (p.p86 * locals.var_vha1);
        let assign4670_e3695: f64 = (locals.var_nfagat_i - assign4670_e3694);
        (assign4670_e3695, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4670_e3697;
        locals.var_nj0_dn0 = assign4670_e3697_d_n0;
        locals.var_nj0_dn2 = assign4670_e3697_d_n2;

        let (assign4680_e3711, assign4680_e3711_d_n0, assign4680_e3711_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4680_e3707: f64 = (p.p85 - locals.var_nja10);
        let assign4680_e3709: f64 = (assign4680_e3707 - 0.01);
        (assign4680_e3709, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4680_e3711;
        locals.var_tmf1_dn0 = assign4680_e3711_d_n0;
        locals.var_tmf1_dn2 = assign4680_e3711_d_n2;

        let (assign4690_e3725, assign4690_e3725_d_n0, assign4690_e3725_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4690_e3721: f64 = (4.0 * p.p85);
        let assign4690_e3723: f64 = (assign4690_e3721 * 0.01);
        (assign4690_e3723, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4690_e3725;
        locals.var_tmf2_dn0 = assign4690_e3725_d_n0;
        locals.var_tmf2_dn2 = assign4690_e3725_d_n2;

        let (assign4700_e3741, assign4700_e3741_d_n0, assign4700_e3741_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let (assign4700_e3739, assign4700_e3739_d_n0, assign4700_e3739_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4700_e3738: f64 = (-locals.var_tmf2);
                (assign4700_e3738, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4700_e3739, assign4700_e3739_d_n0, assign4700_e3739_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4700_e3741;
        locals.var_tmf2_dn0 = assign4700_e3741_d_n0;
        locals.var_tmf2_dn2 = assign4700_e3741_d_n2;

        let (assign4710_e3756, assign4710_e3756_d_n0, assign4710_e3756_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4710_e3751: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4710_e3753: f64 = (assign4710_e3751 + locals.var_tmf2);
        let assign4710_e3754: f64 = (assign4710_e3753).sqrt();
        (assign4710_e3754, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4710_e3754)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4710_e3754)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4710_e3756;
        locals.var_tmf2_dn0 = assign4710_e3756_d_n0;
        locals.var_tmf2_dn2 = assign4710_e3756_d_n2;

        let (assign4720_e3772, assign4720_e3772_d_n0, assign4720_e3772_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4720_e3768: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4720_e3769: f64 = (0.5 * assign4720_e3768);
        let assign4720_e3770: f64 = (p.p85 - assign4720_e3769);
        (assign4720_e3770, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign4720_e3772;
        locals.var_nja11_dn0 = assign4720_e3772_d_n0;
        locals.var_nja11_dn2 = assign4720_e3772_d_n2;

        let (assign4730_e3786, assign4730_e3786_d_n0, assign4730_e3786_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4730_e3782: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign4730_e3784: f64 = (assign4730_e3782 - 0.01);
        (assign4730_e3784, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4730_e3786;
        locals.var_tmf1_dn0 = assign4730_e3786_d_n0;
        locals.var_tmf1_dn2 = assign4730_e3786_d_n2;

        let (assign4740_e3800, assign4740_e3800_d_n0, assign4740_e3800_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4740_e3796: f64 = (4.0 * locals.var_nfagat_i);
        let assign4740_e3798: f64 = (assign4740_e3796 * 0.01);
        (assign4740_e3798, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4740_e3800;
        locals.var_tmf2_dn0 = assign4740_e3800_d_n0;
        locals.var_tmf2_dn2 = assign4740_e3800_d_n2;

        let (assign4750_e3816, assign4750_e3816_d_n0, assign4750_e3816_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let (assign4750_e3814, assign4750_e3814_d_n0, assign4750_e3814_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4750_e3813: f64 = (-locals.var_tmf2);
                (assign4750_e3813, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4750_e3814, assign4750_e3814_d_n0, assign4750_e3814_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4750_e3816;
        locals.var_tmf2_dn0 = assign4750_e3816_d_n0;
        locals.var_tmf2_dn2 = assign4750_e3816_d_n2;

        let (assign4760_e3831, assign4760_e3831_d_n0, assign4760_e3831_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4760_e3826: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4760_e3828: f64 = (assign4760_e3826 + locals.var_tmf2);
        let assign4760_e3829: f64 = (assign4760_e3828).sqrt();
        (assign4760_e3829, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4760_e3829)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4760_e3829)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4760_e3831;
        locals.var_tmf2_dn0 = assign4760_e3831_d_n0;
        locals.var_tmf2_dn2 = assign4760_e3831_d_n2;

        let (assign4770_e3847, assign4770_e3847_d_n0, assign4770_e3847_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4770_e3843: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4770_e3844: f64 = (0.5 * assign4770_e3843);
        let assign4770_e3845: f64 = (locals.var_nfagat_i + assign4770_e3844);
        (assign4770_e3845, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign4770_e3847;
        locals.var_nj1_dn0 = assign4770_e3847_d_n0;
        locals.var_nj1_dn2 = assign4770_e3847_d_n2;

        let (assign4780_e3861, assign4780_e3861_d_n0, assign4780_e3861_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4780_e3857: f64 = (p.p85 - locals.var_nj0);
        let assign4780_e3859: f64 = (assign4780_e3857 - 0.01);
        (assign4780_e3859, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4780_e3861;
        locals.var_tmf1_dn0 = assign4780_e3861_d_n0;
        locals.var_tmf1_dn2 = assign4780_e3861_d_n2;

        let (assign4790_e3875, assign4790_e3875_d_n0, assign4790_e3875_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4790_e3871: f64 = (4.0 * p.p85);
        let assign4790_e3873: f64 = (assign4790_e3871 * 0.01);
        (assign4790_e3873, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4790_e3875;
        locals.var_tmf2_dn0 = assign4790_e3875_d_n0;
        locals.var_tmf2_dn2 = assign4790_e3875_d_n2;

        let (assign4800_e3891, assign4800_e3891_d_n0, assign4800_e3891_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let (assign4800_e3889, assign4800_e3889_d_n0, assign4800_e3889_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4800_e3888: f64 = (-locals.var_tmf2);
                (assign4800_e3888, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4800_e3889, assign4800_e3889_d_n0, assign4800_e3889_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4800_e3891;
        locals.var_tmf2_dn0 = assign4800_e3891_d_n0;
        locals.var_tmf2_dn2 = assign4800_e3891_d_n2;

        let (assign4810_e3906, assign4810_e3906_d_n0, assign4810_e3906_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4810_e3901: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4810_e3903: f64 = (assign4810_e3901 + locals.var_tmf2);
        let assign4810_e3904: f64 = (assign4810_e3903).sqrt();
        (assign4810_e3904, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4810_e3904)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4810_e3904)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4810_e3906;
        locals.var_tmf2_dn0 = assign4810_e3906_d_n0;
        locals.var_tmf2_dn2 = assign4810_e3906_d_n2;

        let (assign4820_e3922, assign4820_e3922_d_n0, assign4820_e3922_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4820_e3918: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4820_e3919: f64 = (0.5 * assign4820_e3918);
        let assign4820_e3920: f64 = (p.p85 - assign4820_e3919);
        (assign4820_e3920, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4820_e3922;
        locals.var_nj0_dn0 = assign4820_e3922_d_n0;
        locals.var_nj0_dn2 = assign4820_e3922_d_n2;

        let (assign4830_e3936, assign4830_e3936_d_n0, assign4830_e3936_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4830_e3932: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign4830_e3934: f64 = (assign4830_e3932 - 0.01);
        (assign4830_e3934, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign4830_e3936;
        locals.var_tmf1_dn0 = assign4830_e3936_d_n0;
        locals.var_tmf1_dn2 = assign4830_e3936_d_n2;

        let (assign4840_e3950, assign4840_e3950_d_n0, assign4840_e3950_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4840_e3946: f64 = (4.0 * locals.var_nfagat_i);
        let assign4840_e3948: f64 = (assign4840_e3946 * 0.01);
        (assign4840_e3948, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4840_e3950;
        locals.var_tmf2_dn0 = assign4840_e3950_d_n0;
        locals.var_tmf2_dn2 = assign4840_e3950_d_n2;

        let (assign4850_e3966, assign4850_e3966_d_n0, assign4850_e3966_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let (assign4850_e3964, assign4850_e3964_d_n0, assign4850_e3964_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign4850_e3963: f64 = (-locals.var_tmf2);
                (assign4850_e3963, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign4850_e3964, assign4850_e3964_d_n0, assign4850_e3964_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4850_e3966;
        locals.var_tmf2_dn0 = assign4850_e3966_d_n0;
        locals.var_tmf2_dn2 = assign4850_e3966_d_n2;

        let (assign4860_e3981, assign4860_e3981_d_n0, assign4860_e3981_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4860_e3976: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign4860_e3978: f64 = (assign4860_e3976 + locals.var_tmf2);
        let assign4860_e3979: f64 = (assign4860_e3978).sqrt();
        (assign4860_e3979, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign4860_e3979)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign4860_e3979)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign4860_e3981;
        locals.var_tmf2_dn0 = assign4860_e3981_d_n0;
        locals.var_tmf2_dn2 = assign4860_e3981_d_n2;

        let (assign4870_e3997, assign4870_e3997_d_n0, assign4870_e3997_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign4870_e3993: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign4870_e3994: f64 = (0.5 * assign4870_e3993);
        let assign4870_e3995: f64 = (locals.var_nfagat_i + assign4870_e3994);
        (assign4870_e3995, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4870_e3997;
        locals.var_nj0_dn0 = assign4870_e3997_d_n0;
        locals.var_nj0_dn2 = assign4870_e3997_d_n2;

        let (assign4880_e4008, assign4880_e4008_d_n0, assign4880_e4008_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign4880_e4008;
        locals.var_nj0_dn0 = assign4880_e4008_d_n0;
        locals.var_nj0_dn2 = assign4880_e4008_d_n2;

        let (assign4890_e4019, assign4890_e4019_d_n0, assign4890_e4019_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) && (locals.var_guard86 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign4890_e4019;
        locals.var_nj1_dn0 = assign4890_e4019_d_n0;
        locals.var_nj1_dn2 = assign4890_e4019_d_n2;

        let (assign4960_e4269,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign4960_e4265: f64 = (locals.var_nin * locals.var_nin);
        let assign4960_e4267: f64 = (assign4960_e4265 / locals.var_ndibot_i);
        (assign4960_e4267,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign4960_e4269;

        let (assign4970_e4285,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign4970_e4278: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign4970_e4281: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign4970_e4282: f64 = (assign4970_e4281).ln();
        let assign4970_e4283: f64 = (assign4970_e4278 * assign4970_e4282);
        (assign4970_e4283,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign4970_e4285;

        let assign4980_e4288: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign4980_e4288;

        let (assign4990_e4305, assign4990_e4305_d_n0, assign4990_e4305_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign4990_e4300: f64 = (locals.var_vmax - locals.var_vha1);
        let assign4990_e4301: f64 = (p.p86 * assign4990_e4300);
        let assign4990_e4303: f64 = (assign4990_e4301 + locals.var_nfabot_i);
        (assign4990_e4303, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign4990_e4305;
        locals.var_nja10_dn0 = assign4990_e4305_d_n0;
        locals.var_nja10_dn2 = assign4990_e4305_d_n2;

        let (assign5000_e4320, assign5000_e4320_d_n0, assign5000_e4320_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5000_e4317: f64 = (p.p86 * locals.var_vha1);
        let assign5000_e4318: f64 = (locals.var_nfabot_i - assign5000_e4317);
        (assign5000_e4318, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5000_e4320;
        locals.var_nj0_dn0 = assign5000_e4320_d_n0;
        locals.var_nj0_dn2 = assign5000_e4320_d_n2;

        let (assign5010_e4335, assign5010_e4335_d_n0, assign5010_e4335_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5010_e4331: f64 = (p.p85 - locals.var_nja10);
        let assign5010_e4333: f64 = (assign5010_e4331 - 0.01);
        (assign5010_e4333, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5010_e4335;
        locals.var_tmf1_dn0 = assign5010_e4335_d_n0;
        locals.var_tmf1_dn2 = assign5010_e4335_d_n2;

        let (assign5020_e4350, assign5020_e4350_d_n0, assign5020_e4350_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5020_e4346: f64 = (4.0 * p.p85);
        let assign5020_e4348: f64 = (assign5020_e4346 * 0.01);
        (assign5020_e4348, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5020_e4350;
        locals.var_tmf2_dn0 = assign5020_e4350_d_n0;
        locals.var_tmf2_dn2 = assign5020_e4350_d_n2;

        let (assign5030_e4367, assign5030_e4367_d_n0, assign5030_e4367_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let (assign5030_e4365, assign5030_e4365_d_n0, assign5030_e4365_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5030_e4364: f64 = (-locals.var_tmf2);
                (assign5030_e4364, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5030_e4365, assign5030_e4365_d_n0, assign5030_e4365_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5030_e4367;
        locals.var_tmf2_dn0 = assign5030_e4367_d_n0;
        locals.var_tmf2_dn2 = assign5030_e4367_d_n2;

        let (assign5040_e4383, assign5040_e4383_d_n0, assign5040_e4383_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5040_e4378: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5040_e4380: f64 = (assign5040_e4378 + locals.var_tmf2);
        let assign5040_e4381: f64 = (assign5040_e4380).sqrt();
        (assign5040_e4381, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5040_e4381)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5040_e4381)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5040_e4383;
        locals.var_tmf2_dn0 = assign5040_e4383_d_n0;
        locals.var_tmf2_dn2 = assign5040_e4383_d_n2;

        let (assign5050_e4400, assign5050_e4400_d_n0, assign5050_e4400_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5050_e4396: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5050_e4397: f64 = (1.0 + assign5050_e4396);
        let assign5050_e4398: f64 = (0.5 * assign5050_e4397);
        (assign5050_e4398, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign5050_e4400;
        locals.var_dfn_su_dn0 = assign5050_e4400_d_n0;
        locals.var_dfn_su_dn2 = assign5050_e4400_d_n2;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5060_e4417, assign5060_e4417_d_n0, assign5060_e4417_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5060_e4413: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5060_e4414: f64 = (0.5 * assign5060_e4413);
        let assign5060_e4415: f64 = (p.p85 - assign5060_e4414);
        (assign5060_e4415, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign5060_e4417;
        locals.var_nja11_dn0 = assign5060_e4417_d_n0;
        locals.var_nja11_dn2 = assign5060_e4417_d_n2;

        let (assign5070_e4432, assign5070_e4432_d_n0, assign5070_e4432_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5070_e4428: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign5070_e4430: f64 = (assign5070_e4428 - 0.01);
        (assign5070_e4430, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5070_e4432;
        locals.var_tmf1_dn0 = assign5070_e4432_d_n0;
        locals.var_tmf1_dn2 = assign5070_e4432_d_n2;

        let (assign5080_e4447, assign5080_e4447_d_n0, assign5080_e4447_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5080_e4443: f64 = (4.0 * locals.var_nfabot_i);
        let assign5080_e4445: f64 = (assign5080_e4443 * 0.01);
        (assign5080_e4445, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5080_e4447;
        locals.var_tmf2_dn0 = assign5080_e4447_d_n0;
        locals.var_tmf2_dn2 = assign5080_e4447_d_n2;

        let (assign5090_e4464, assign5090_e4464_d_n0, assign5090_e4464_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let (assign5090_e4462, assign5090_e4462_d_n0, assign5090_e4462_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5090_e4461: f64 = (-locals.var_tmf2);
                (assign5090_e4461, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5090_e4462, assign5090_e4462_d_n0, assign5090_e4462_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5090_e4464;
        locals.var_tmf2_dn0 = assign5090_e4464_d_n0;
        locals.var_tmf2_dn2 = assign5090_e4464_d_n2;

        let (assign5100_e4480, assign5100_e4480_d_n0, assign5100_e4480_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5100_e4475: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5100_e4477: f64 = (assign5100_e4475 + locals.var_tmf2);
        let assign5100_e4478: f64 = (assign5100_e4477).sqrt();
        (assign5100_e4478, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5100_e4478)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5100_e4478)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5100_e4480;
        locals.var_tmf2_dn0 = assign5100_e4480_d_n0;
        locals.var_tmf2_dn2 = assign5100_e4480_d_n2;

        let (assign5110_e4497, assign5110_e4497_d_n0, assign5110_e4497_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5110_e4493: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5110_e4494: f64 = (1.0 + assign5110_e4493);
        let assign5110_e4495: f64 = (0.5 * assign5110_e4494);
        (assign5110_e4495, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign5110_e4497;
        locals.var_dfn_sl_dn0 = assign5110_e4497_d_n0;
        locals.var_dfn_sl_dn2 = assign5110_e4497_d_n2;

        let (assign5120_e4514, assign5120_e4514_d_n0, assign5120_e4514_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5120_e4510: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5120_e4511: f64 = (0.5 * assign5120_e4510);
        let assign5120_e4512: f64 = (locals.var_nfabot_i + assign5120_e4511);
        (assign5120_e4512, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign5120_e4514;
        locals.var_nj1_dn0 = assign5120_e4514_d_n0;
        locals.var_nj1_dn2 = assign5120_e4514_d_n2;

        let (assign5130_e4529, assign5130_e4529_d_n0, assign5130_e4529_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5130_e4525: f64 = (p.p85 - locals.var_nj0);
        let assign5130_e4527: f64 = (assign5130_e4525 - 0.01);
        (assign5130_e4527, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5130_e4529;
        locals.var_tmf1_dn0 = assign5130_e4529_d_n0;
        locals.var_tmf1_dn2 = assign5130_e4529_d_n2;

        let (assign5140_e4544, assign5140_e4544_d_n0, assign5140_e4544_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5140_e4540: f64 = (4.0 * p.p85);
        let assign5140_e4542: f64 = (assign5140_e4540 * 0.01);
        (assign5140_e4542, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5140_e4544;
        locals.var_tmf2_dn0 = assign5140_e4544_d_n0;
        locals.var_tmf2_dn2 = assign5140_e4544_d_n2;

        let (assign5150_e4561, assign5150_e4561_d_n0, assign5150_e4561_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let (assign5150_e4559, assign5150_e4559_d_n0, assign5150_e4559_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5150_e4558: f64 = (-locals.var_tmf2);
                (assign5150_e4558, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5150_e4559, assign5150_e4559_d_n0, assign5150_e4559_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5150_e4561;
        locals.var_tmf2_dn0 = assign5150_e4561_d_n0;
        locals.var_tmf2_dn2 = assign5150_e4561_d_n2;

        let (assign5160_e4577, assign5160_e4577_d_n0, assign5160_e4577_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5160_e4572: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5160_e4574: f64 = (assign5160_e4572 + locals.var_tmf2);
        let assign5160_e4575: f64 = (assign5160_e4574).sqrt();
        (assign5160_e4575, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5160_e4575)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5160_e4575)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5160_e4577;
        locals.var_tmf2_dn0 = assign5160_e4577_d_n0;
        locals.var_tmf2_dn2 = assign5160_e4577_d_n2;

        let (assign5170_e4594, assign5170_e4594_d_n0, assign5170_e4594_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5170_e4590: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5170_e4591: f64 = (0.5 * assign5170_e4590);
        let assign5170_e4592: f64 = (p.p85 - assign5170_e4591);
        (assign5170_e4592, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5170_e4594;
        locals.var_nj0_dn0 = assign5170_e4594_d_n0;
        locals.var_nj0_dn2 = assign5170_e4594_d_n2;

        let (assign5180_e4609, assign5180_e4609_d_n0, assign5180_e4609_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5180_e4605: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign5180_e4607: f64 = (assign5180_e4605 - 0.01);
        (assign5180_e4607, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5180_e4609;
        locals.var_tmf1_dn0 = assign5180_e4609_d_n0;
        locals.var_tmf1_dn2 = assign5180_e4609_d_n2;

        let (assign5190_e4624, assign5190_e4624_d_n0, assign5190_e4624_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5190_e4620: f64 = (4.0 * locals.var_nfabot_i);
        let assign5190_e4622: f64 = (assign5190_e4620 * 0.01);
        (assign5190_e4622, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5190_e4624;
        locals.var_tmf2_dn0 = assign5190_e4624_d_n0;
        locals.var_tmf2_dn2 = assign5190_e4624_d_n2;

        let (assign5200_e4641, assign5200_e4641_d_n0, assign5200_e4641_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let (assign5200_e4639, assign5200_e4639_d_n0, assign5200_e4639_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5200_e4638: f64 = (-locals.var_tmf2);
                (assign5200_e4638, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5200_e4639, assign5200_e4639_d_n0, assign5200_e4639_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5200_e4641;
        locals.var_tmf2_dn0 = assign5200_e4641_d_n0;
        locals.var_tmf2_dn2 = assign5200_e4641_d_n2;

        let (assign5210_e4657, assign5210_e4657_d_n0, assign5210_e4657_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5210_e4652: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5210_e4654: f64 = (assign5210_e4652 + locals.var_tmf2);
        let assign5210_e4655: f64 = (assign5210_e4654).sqrt();
        (assign5210_e4655, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5210_e4655)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5210_e4655)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5210_e4657;
        locals.var_tmf2_dn0 = assign5210_e4657_d_n0;
        locals.var_tmf2_dn2 = assign5210_e4657_d_n2;

        let (assign5220_e4674, assign5220_e4674_d_n0, assign5220_e4674_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5220_e4670: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5220_e4671: f64 = (0.5 * assign5220_e4670);
        let assign5220_e4672: f64 = (locals.var_nfabot_i + assign5220_e4671);
        (assign5220_e4672, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5220_e4674;
        locals.var_nj0_dn0 = assign5220_e4674_d_n0;
        locals.var_nj0_dn2 = assign5220_e4674_d_n2;

        let (assign5230_e4689, assign5230_e4689_d_n0, assign5230_e4689_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5230_e4685: f64 = (p.p86 * locals.var_dfn_su);
        let assign5230_e4687: f64 = (assign5230_e4685 * locals.var_dfn_sl);
        (assign5230_e4687, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign5230_e4685 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign5230_e4685 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign5230_e4689;
        locals.var_dnj1_dv_dn0 = assign5230_e4689_d_n0;
        locals.var_dnj1_dv_dn2 = assign5230_e4689_d_n2;

        let (assign5240_e4701, assign5240_e4701_d_n0, assign5240_e4701_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5240_e4701;
        locals.var_nj0_dn0 = assign5240_e4701_d_n0;
        locals.var_nj0_dn2 = assign5240_e4701_d_n2;

        let (assign5250_e4713, assign5250_e4713_d_n0, assign5250_e4713_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign5250_e4713;
        locals.var_nj1_dn0 = assign5250_e4713_d_n0;
        locals.var_nj1_dn2 = assign5250_e4713_d_n2;

        let (assign5260_e4725, assign5260_e4725_d_n0, assign5260_e4725_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard89 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign5260_e4725;
        locals.var_dnj1_dv_dn0 = assign5260_e4725_d_n0;
        locals.var_dnj1_dv_dn2 = assign5260_e4725_d_n2;

        let assign5270_e4729: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5270_e4733: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5270_e4734: f64 = (locals.var_vha1 * assign5270_e4733);
        let assign5270_e4737: f64 = (locals.var_nj0 * p.p85);
        let assign5270_e4738: f64 = (assign5270_e4734 / assign5270_e4737);
        let assign5270_e4739: f64 = (assign5270_e4729 + assign5270_e4738);
        let assign5270_e4740: f64 = (locals.var_phitdinv * assign5270_e4739);
        let assign5270_e4741: f64 = (assign5270_e4740).abs();
        let assign5270_e4743: f64 = if assign5270_e4741 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign5270_e4743;

        let (assign5280_e4769, assign5280_e4769_d_n0, assign5280_e4769_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard90 != 0.0)) {
        let assign5280_e4755: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5280_e4759: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5280_e4760: f64 = (locals.var_vha1 * assign5280_e4759);
        let assign5280_e4763: f64 = (locals.var_nj0 * p.p85);
        let assign5280_e4764: f64 = (assign5280_e4760 / assign5280_e4763);
        let assign5280_e4765: f64 = (assign5280_e4755 + assign5280_e4764);
        let assign5280_e4766: f64 = (locals.var_phitdinv * assign5280_e4765);
        let assign5280_e4767: f64 = (assign5280_e4766).exp();
        (assign5280_e4767, (assign5280_e4767 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign5280_e4763) - (assign5280_e4760 * (locals.var_nj0_dn0 * p.p85))) / (assign5280_e4763 * assign5280_e4763))))), (assign5280_e4767 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign5280_e4763) - (assign5280_e4760 * (locals.var_nj0_dn2 * p.p85))) / (assign5280_e4763 * assign5280_e4763))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign5280_e4769;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign5280_e4769_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign5280_e4769_d_n2;

        let assign5290_e4773: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5290_e4777: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5290_e4778: f64 = (locals.var_vha1 * assign5290_e4777);
        let assign5290_e4781: f64 = (locals.var_nj0 * p.p85);
        let assign5290_e4782: f64 = (assign5290_e4778 / assign5290_e4781);
        let assign5290_e4783: f64 = (assign5290_e4773 + assign5290_e4782);
        let assign5290_e4784: f64 = (locals.var_phitdinv * assign5290_e4783);
        let assign5290_e4786: f64 = (-230.25850929940458);
        let assign5290_e4787: f64 = if assign5290_e4784 < assign5290_e4786 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign5290_e4787;

        let (assign5300_e4868, assign5300_e4868_d_n0, assign5300_e4868_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5300_e4802: f64 = (-230.25850929940458);
        let assign5300_e4806: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5300_e4810: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5300_e4811: f64 = (locals.var_vha1 * assign5300_e4810);
        let assign5300_e4814: f64 = (locals.var_nj0 * p.p85);
        let assign5300_e4815: f64 = (assign5300_e4811 / assign5300_e4814);
        let assign5300_e4816: f64 = (assign5300_e4806 + assign5300_e4815);
        let assign5300_e4817: f64 = (locals.var_phitdinv * assign5300_e4816);
        let assign5300_e4818: f64 = (assign5300_e4802 - assign5300_e4817);
        let assign5300_e4822: f64 = (-230.25850929940458);
        let assign5300_e4826: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5300_e4830: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5300_e4831: f64 = (locals.var_vha1 * assign5300_e4830);
        let assign5300_e4834: f64 = (locals.var_nj0 * p.p85);
        let assign5300_e4835: f64 = (assign5300_e4831 / assign5300_e4834);
        let assign5300_e4836: f64 = (assign5300_e4826 + assign5300_e4835);
        let assign5300_e4837: f64 = (locals.var_phitdinv * assign5300_e4836);
        let assign5300_e4838: f64 = (assign5300_e4822 - assign5300_e4837);
        let assign5300_e4841: f64 = (-230.25850929940458);
        let assign5300_e4845: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5300_e4849: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5300_e4850: f64 = (locals.var_vha1 * assign5300_e4849);
        let assign5300_e4853: f64 = (locals.var_nj0 * p.p85);
        let assign5300_e4854: f64 = (assign5300_e4850 / assign5300_e4853);
        let assign5300_e4855: f64 = (assign5300_e4845 + assign5300_e4854);
        let assign5300_e4856: f64 = (locals.var_phitdinv * assign5300_e4855);
        let assign5300_e4857: f64 = (assign5300_e4841 - assign5300_e4856);
        let assign5300_e4859: f64 = (assign5300_e4857 * 0.3333333333333333);
        let assign5300_e4860: f64 = (1.0 + assign5300_e4859);
        let assign5300_e4861: f64 = (assign5300_e4838 * assign5300_e4860);
        let assign5300_e4862: f64 = (0.5 * assign5300_e4861);
        let assign5300_e4863: f64 = (1.0 + assign5300_e4862);
        let assign5300_e4864: f64 = (assign5300_e4818 * assign5300_e4863);
        let assign5300_e4865: f64 = (1.0 + assign5300_e4864);
        let assign5300_e4866: f64 = (1e-100 / assign5300_e4865);
        (assign5300_e4866, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign5300_e4814) - (assign5300_e4811 * (locals.var_nj0_dn0 * p.p85))) / (assign5300_e4814 * assign5300_e4814))))) * assign5300_e4863) + (assign5300_e4818 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign5300_e4834) - (assign5300_e4831 * (locals.var_nj0_dn0 * p.p85))) / (assign5300_e4834 * assign5300_e4834))))) * assign5300_e4860) + (assign5300_e4838 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign5300_e4853) - (assign5300_e4850 * (locals.var_nj0_dn0 * p.p85))) / (assign5300_e4853 * assign5300_e4853))))) * 0.3333333333333333))))))) / (assign5300_e4865 * assign5300_e4865))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign5300_e4814) - (assign5300_e4811 * (locals.var_nj0_dn2 * p.p85))) / (assign5300_e4814 * assign5300_e4814))))) * assign5300_e4863) + (assign5300_e4818 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign5300_e4834) - (assign5300_e4831 * (locals.var_nj0_dn2 * p.p85))) / (assign5300_e4834 * assign5300_e4834))))) * assign5300_e4860) + (assign5300_e4838 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign5300_e4853) - (assign5300_e4850 * (locals.var_nj0_dn2 * p.p85))) / (assign5300_e4853 * assign5300_e4853))))) * 0.3333333333333333))))))) / (assign5300_e4865 * assign5300_e4865))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign5300_e4868;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign5300_e4868_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign5300_e4868_d_n2;

        let (assign5310_e4947, assign5310_e4947_d_n0, assign5310_e4947_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard91 == 0.0)) {
        let assign5310_e4886: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5310_e4890: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5310_e4891: f64 = (locals.var_vha1 * assign5310_e4890);
        let assign5310_e4894: f64 = (locals.var_nj0 * p.p85);
        let assign5310_e4895: f64 = (assign5310_e4891 / assign5310_e4894);
        let assign5310_e4896: f64 = (assign5310_e4886 + assign5310_e4895);
        let assign5310_e4897: f64 = (locals.var_phitdinv * assign5310_e4896);
        let assign5310_e4899: f64 = (assign5310_e4897 - 230.25850929940458);
        let assign5310_e4905: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5310_e4909: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5310_e4910: f64 = (locals.var_vha1 * assign5310_e4909);
        let assign5310_e4913: f64 = (locals.var_nj0 * p.p85);
        let assign5310_e4914: f64 = (assign5310_e4910 / assign5310_e4913);
        let assign5310_e4915: f64 = (assign5310_e4905 + assign5310_e4914);
        let assign5310_e4916: f64 = (locals.var_phitdinv * assign5310_e4915);
        let assign5310_e4918: f64 = (assign5310_e4916 - 230.25850929940458);
        let assign5310_e4923: f64 = (locals.var_vmax / locals.var_nj1);
        let assign5310_e4927: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign5310_e4928: f64 = (locals.var_vha1 * assign5310_e4927);
        let assign5310_e4931: f64 = (locals.var_nj0 * p.p85);
        let assign5310_e4932: f64 = (assign5310_e4928 / assign5310_e4931);
        let assign5310_e4933: f64 = (assign5310_e4923 + assign5310_e4932);
        let assign5310_e4934: f64 = (locals.var_phitdinv * assign5310_e4933);
        let assign5310_e4936: f64 = (assign5310_e4934 - 230.25850929940458);
        let assign5310_e4938: f64 = (assign5310_e4936 * 0.3333333333333333);
        let assign5310_e4939: f64 = (1.0 + assign5310_e4938);
        let assign5310_e4940: f64 = (assign5310_e4918 * assign5310_e4939);
        let assign5310_e4941: f64 = (0.5 * assign5310_e4940);
        let assign5310_e4942: f64 = (1.0 + assign5310_e4941);
        let assign5310_e4943: f64 = (assign5310_e4899 * assign5310_e4942);
        let assign5310_e4944: f64 = (1.0 + assign5310_e4943);
        let assign5310_e4945: f64 = (1e100 * assign5310_e4944);
        (assign5310_e4945, (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign5310_e4894) - (assign5310_e4891 * (locals.var_nj0_dn0 * p.p85))) / (assign5310_e4894 * assign5310_e4894)))) * assign5310_e4942) + (assign5310_e4899 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign5310_e4913) - (assign5310_e4910 * (locals.var_nj0_dn0 * p.p85))) / (assign5310_e4913 * assign5310_e4913)))) * assign5310_e4939) + (assign5310_e4918 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign5310_e4931) - (assign5310_e4928 * (locals.var_nj0_dn0 * p.p85))) / (assign5310_e4931 * assign5310_e4931)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign5310_e4894) - (assign5310_e4891 * (locals.var_nj0_dn2 * p.p85))) / (assign5310_e4894 * assign5310_e4894)))) * assign5310_e4942) + (assign5310_e4899 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign5310_e4913) - (assign5310_e4910 * (locals.var_nj0_dn2 * p.p85))) / (assign5310_e4913 * assign5310_e4913)))) * assign5310_e4939) + (assign5310_e4918 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign5310_e4931) - (assign5310_e4928 * (locals.var_nj0_dn2 * p.p85))) / (assign5310_e4931 * assign5310_e4931)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign5310_e4947;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign5310_e4947_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign5310_e4947_d_n2;

        let (assign5320_e4974, assign5320_e4974_d_n0, assign5320_e4974_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign5320_e4958: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign5320_e4959: f64 = (locals.var_nj1 - assign5320_e4958);
        let assign5320_e4962: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign5320_e4963: f64 = (assign5320_e4959 / assign5320_e4962);
        let assign5320_e4966: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign5320_e4969: f64 = (locals.var_nj0 * p.p85);
        let assign5320_e4970: f64 = (assign5320_e4966 / assign5320_e4969);
        let assign5320_e4971: f64 = (assign5320_e4963 + assign5320_e4970);
        let assign5320_e4972: f64 = (locals.var_phitdinv * assign5320_e4971);
        (assign5320_e4972, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign5320_e4962) - (assign5320_e4959 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign5320_e4962 * assign5320_e4962)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign5320_e4969) - (assign5320_e4966 * (locals.var_nj0_dn0 * p.p85))) / (assign5320_e4969 * assign5320_e4969)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign5320_e4962) - (assign5320_e4959 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign5320_e4962 * assign5320_e4962)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign5320_e4969) - (assign5320_e4966 * (locals.var_nj0_dn2 * p.p85))) / (assign5320_e4969 * assign5320_e4969)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign5320_e4974;
        locals.var_dvmax_over_phitd_dv_dn0 = assign5320_e4974_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign5320_e4974_d_n2;

        let (assign5330_e4991, assign5330_e4991_d_n0, assign5330_e4991_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign5330_e4984: f64 = (locals.var_v1 - locals.var_vmax);
        let assign5330_e4986: f64 = (assign5330_e4984 * locals.var_dvmax_over_phitd_dv);
        let assign5330_e4987: f64 = (1.0 + assign5330_e4986);
        let assign5330_e4989: f64 = (assign5330_e4987 * locals.var_exp_vmax_over_phitd_bot);
        (assign5330_e4989, (((assign5330_e4984 * locals.var_dvmax_over_phitd_dv_dn0) * locals.var_exp_vmax_over_phitd_bot) + (assign5330_e4987 * locals.var_exp_vmax_over_phitd_bot_dn0)), (((assign5330_e4984 * locals.var_dvmax_over_phitd_dv_dn2) * locals.var_exp_vmax_over_phitd_bot) + (assign5330_e4987 * locals.var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign5330_e4991;
        locals.var_idmultbot_dn0 = assign5330_e4991_d_n0;
        locals.var_idmultbot_dn2 = assign5330_e4991_d_n2;

        let (assign5340_e5004,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign5340_e5000: f64 = (locals.var_nin * locals.var_nin);
        let assign5340_e5002: f64 = (assign5340_e5000 / locals.var_ndisti_i);
        (assign5340_e5002,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign5340_e5004;

        let (assign5350_e5020,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign5350_e5013: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign5350_e5016: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign5350_e5017: f64 = (assign5350_e5016).ln();
        let assign5350_e5018: f64 = (assign5350_e5013 * assign5350_e5017);
        (assign5350_e5018,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign5350_e5020;

        let assign5360_e5023: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign5360_e5023;

        let (assign5370_e5040, assign5370_e5040_d_n0, assign5370_e5040_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5370_e5035: f64 = (locals.var_vmax - locals.var_vha1);
        let assign5370_e5036: f64 = (p.p86 * assign5370_e5035);
        let assign5370_e5038: f64 = (assign5370_e5036 + locals.var_nfasti_i);
        (assign5370_e5038, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign5370_e5040;
        locals.var_nja10_dn0 = assign5370_e5040_d_n0;
        locals.var_nja10_dn2 = assign5370_e5040_d_n2;

        let (assign5380_e5055, assign5380_e5055_d_n0, assign5380_e5055_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5380_e5052: f64 = (p.p86 * locals.var_vha1);
        let assign5380_e5053: f64 = (locals.var_nfasti_i - assign5380_e5052);
        (assign5380_e5053, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5380_e5055;
        locals.var_nj0_dn0 = assign5380_e5055_d_n0;
        locals.var_nj0_dn2 = assign5380_e5055_d_n2;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5390_e5070, assign5390_e5070_d_n0, assign5390_e5070_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5390_e5066: f64 = (p.p85 - locals.var_nja10);
        let assign5390_e5068: f64 = (assign5390_e5066 - 0.01);
        (assign5390_e5068, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5390_e5070;
        locals.var_tmf1_dn0 = assign5390_e5070_d_n0;
        locals.var_tmf1_dn2 = assign5390_e5070_d_n2;

        let (assign5400_e5085, assign5400_e5085_d_n0, assign5400_e5085_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5400_e5081: f64 = (4.0 * p.p85);
        let assign5400_e5083: f64 = (assign5400_e5081 * 0.01);
        (assign5400_e5083, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5400_e5085;
        locals.var_tmf2_dn0 = assign5400_e5085_d_n0;
        locals.var_tmf2_dn2 = assign5400_e5085_d_n2;

        let (assign5410_e5102, assign5410_e5102_d_n0, assign5410_e5102_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let (assign5410_e5100, assign5410_e5100_d_n0, assign5410_e5100_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5410_e5099: f64 = (-locals.var_tmf2);
                (assign5410_e5099, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5410_e5100, assign5410_e5100_d_n0, assign5410_e5100_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5410_e5102;
        locals.var_tmf2_dn0 = assign5410_e5102_d_n0;
        locals.var_tmf2_dn2 = assign5410_e5102_d_n2;

        let (assign5420_e5118, assign5420_e5118_d_n0, assign5420_e5118_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5420_e5113: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5420_e5115: f64 = (assign5420_e5113 + locals.var_tmf2);
        let assign5420_e5116: f64 = (assign5420_e5115).sqrt();
        (assign5420_e5116, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5420_e5116)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5420_e5116)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5420_e5118;
        locals.var_tmf2_dn0 = assign5420_e5118_d_n0;
        locals.var_tmf2_dn2 = assign5420_e5118_d_n2;

        let (assign5430_e5135, assign5430_e5135_d_n0, assign5430_e5135_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5430_e5131: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5430_e5132: f64 = (1.0 + assign5430_e5131);
        let assign5430_e5133: f64 = (0.5 * assign5430_e5132);
        (assign5430_e5133, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign5430_e5135;
        locals.var_dfn_su_dn0 = assign5430_e5135_d_n0;
        locals.var_dfn_su_dn2 = assign5430_e5135_d_n2;

        let (assign5440_e5152, assign5440_e5152_d_n0, assign5440_e5152_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5440_e5148: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5440_e5149: f64 = (0.5 * assign5440_e5148);
        let assign5440_e5150: f64 = (p.p85 - assign5440_e5149);
        (assign5440_e5150, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign5440_e5152;
        locals.var_nja11_dn0 = assign5440_e5152_d_n0;
        locals.var_nja11_dn2 = assign5440_e5152_d_n2;

        let (assign5450_e5167, assign5450_e5167_d_n0, assign5450_e5167_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5450_e5163: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign5450_e5165: f64 = (assign5450_e5163 - 0.01);
        (assign5450_e5165, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5450_e5167;
        locals.var_tmf1_dn0 = assign5450_e5167_d_n0;
        locals.var_tmf1_dn2 = assign5450_e5167_d_n2;

        let (assign5460_e5182, assign5460_e5182_d_n0, assign5460_e5182_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5460_e5178: f64 = (4.0 * locals.var_nfasti_i);
        let assign5460_e5180: f64 = (assign5460_e5178 * 0.01);
        (assign5460_e5180, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5460_e5182;
        locals.var_tmf2_dn0 = assign5460_e5182_d_n0;
        locals.var_tmf2_dn2 = assign5460_e5182_d_n2;

        let (assign5470_e5199, assign5470_e5199_d_n0, assign5470_e5199_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let (assign5470_e5197, assign5470_e5197_d_n0, assign5470_e5197_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5470_e5196: f64 = (-locals.var_tmf2);
                (assign5470_e5196, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5470_e5197, assign5470_e5197_d_n0, assign5470_e5197_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5470_e5199;
        locals.var_tmf2_dn0 = assign5470_e5199_d_n0;
        locals.var_tmf2_dn2 = assign5470_e5199_d_n2;

        let (assign5480_e5215, assign5480_e5215_d_n0, assign5480_e5215_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5480_e5210: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5480_e5212: f64 = (assign5480_e5210 + locals.var_tmf2);
        let assign5480_e5213: f64 = (assign5480_e5212).sqrt();
        (assign5480_e5213, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5480_e5213)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5480_e5213)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5480_e5215;
        locals.var_tmf2_dn0 = assign5480_e5215_d_n0;
        locals.var_tmf2_dn2 = assign5480_e5215_d_n2;

        let (assign5490_e5232, assign5490_e5232_d_n0, assign5490_e5232_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5490_e5228: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5490_e5229: f64 = (1.0 + assign5490_e5228);
        let assign5490_e5230: f64 = (0.5 * assign5490_e5229);
        (assign5490_e5230, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign5490_e5232;
        locals.var_dfn_sl_dn0 = assign5490_e5232_d_n0;
        locals.var_dfn_sl_dn2 = assign5490_e5232_d_n2;

        let (assign5500_e5249, assign5500_e5249_d_n0, assign5500_e5249_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5500_e5245: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5500_e5246: f64 = (0.5 * assign5500_e5245);
        let assign5500_e5247: f64 = (locals.var_nfasti_i + assign5500_e5246);
        (assign5500_e5247, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign5500_e5249;
        locals.var_nj1_dn0 = assign5500_e5249_d_n0;
        locals.var_nj1_dn2 = assign5500_e5249_d_n2;

        let (assign5510_e5264, assign5510_e5264_d_n0, assign5510_e5264_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5510_e5260: f64 = (p.p85 - locals.var_nj0);
        let assign5510_e5262: f64 = (assign5510_e5260 - 0.01);
        (assign5510_e5262, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5510_e5264;
        locals.var_tmf1_dn0 = assign5510_e5264_d_n0;
        locals.var_tmf1_dn2 = assign5510_e5264_d_n2;

        let (assign5520_e5279, assign5520_e5279_d_n0, assign5520_e5279_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5520_e5275: f64 = (4.0 * p.p85);
        let assign5520_e5277: f64 = (assign5520_e5275 * 0.01);
        (assign5520_e5277, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5520_e5279;
        locals.var_tmf2_dn0 = assign5520_e5279_d_n0;
        locals.var_tmf2_dn2 = assign5520_e5279_d_n2;

        let (assign5530_e5296, assign5530_e5296_d_n0, assign5530_e5296_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let (assign5530_e5294, assign5530_e5294_d_n0, assign5530_e5294_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5530_e5293: f64 = (-locals.var_tmf2);
                (assign5530_e5293, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5530_e5294, assign5530_e5294_d_n0, assign5530_e5294_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5530_e5296;
        locals.var_tmf2_dn0 = assign5530_e5296_d_n0;
        locals.var_tmf2_dn2 = assign5530_e5296_d_n2;

        let (assign5540_e5312, assign5540_e5312_d_n0, assign5540_e5312_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5540_e5307: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5540_e5309: f64 = (assign5540_e5307 + locals.var_tmf2);
        let assign5540_e5310: f64 = (assign5540_e5309).sqrt();
        (assign5540_e5310, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5540_e5310)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5540_e5310)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5540_e5312;
        locals.var_tmf2_dn0 = assign5540_e5312_d_n0;
        locals.var_tmf2_dn2 = assign5540_e5312_d_n2;

        let (assign5550_e5329, assign5550_e5329_d_n0, assign5550_e5329_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5550_e5325: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5550_e5326: f64 = (0.5 * assign5550_e5325);
        let assign5550_e5327: f64 = (p.p85 - assign5550_e5326);
        (assign5550_e5327, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5550_e5329;
        locals.var_nj0_dn0 = assign5550_e5329_d_n0;
        locals.var_nj0_dn2 = assign5550_e5329_d_n2;

        let (assign5560_e5344, assign5560_e5344_d_n0, assign5560_e5344_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5560_e5340: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign5560_e5342: f64 = (assign5560_e5340 - 0.01);
        (assign5560_e5342, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5560_e5344;
        locals.var_tmf1_dn0 = assign5560_e5344_d_n0;
        locals.var_tmf1_dn2 = assign5560_e5344_d_n2;

        let (assign5570_e5359, assign5570_e5359_d_n0, assign5570_e5359_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5570_e5355: f64 = (4.0 * locals.var_nfasti_i);
        let assign5570_e5357: f64 = (assign5570_e5355 * 0.01);
        (assign5570_e5357, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5570_e5359;
        locals.var_tmf2_dn0 = assign5570_e5359_d_n0;
        locals.var_tmf2_dn2 = assign5570_e5359_d_n2;

        let (assign5580_e5376, assign5580_e5376_d_n0, assign5580_e5376_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let (assign5580_e5374, assign5580_e5374_d_n0, assign5580_e5374_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5580_e5373: f64 = (-locals.var_tmf2);
                (assign5580_e5373, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5580_e5374, assign5580_e5374_d_n0, assign5580_e5374_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5580_e5376;
        locals.var_tmf2_dn0 = assign5580_e5376_d_n0;
        locals.var_tmf2_dn2 = assign5580_e5376_d_n2;

        let (assign5590_e5392, assign5590_e5392_d_n0, assign5590_e5392_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5590_e5387: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5590_e5389: f64 = (assign5590_e5387 + locals.var_tmf2);
        let assign5590_e5390: f64 = (assign5590_e5389).sqrt();
        (assign5590_e5390, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5590_e5390)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5590_e5390)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5590_e5392;
        locals.var_tmf2_dn0 = assign5590_e5392_d_n0;
        locals.var_tmf2_dn2 = assign5590_e5392_d_n2;

        let (assign5600_e5409, assign5600_e5409_d_n0, assign5600_e5409_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5600_e5405: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5600_e5406: f64 = (0.5 * assign5600_e5405);
        let assign5600_e5407: f64 = (locals.var_nfasti_i + assign5600_e5406);
        (assign5600_e5407, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5600_e5409;
        locals.var_nj0_dn0 = assign5600_e5409_d_n0;
        locals.var_nj0_dn2 = assign5600_e5409_d_n2;

        let (assign5610_e5424, assign5610_e5424_d_n0, assign5610_e5424_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5610_e5420: f64 = (p.p86 * locals.var_dfn_su);
        let assign5610_e5422: f64 = (assign5610_e5420 * locals.var_dfn_sl);
        (assign5610_e5422, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign5610_e5420 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign5610_e5420 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign5610_e5424;
        locals.var_dnj1_dv_dn0 = assign5610_e5424_d_n0;
        locals.var_dnj1_dv_dn2 = assign5610_e5424_d_n2;

        let (assign5620_e5436, assign5620_e5436_d_n0, assign5620_e5436_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5620_e5436;
        locals.var_nj0_dn0 = assign5620_e5436_d_n0;
        locals.var_nj0_dn2 = assign5620_e5436_d_n2;

        let (assign5630_e5448, assign5630_e5448_d_n0, assign5630_e5448_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign5630_e5448;
        locals.var_nj1_dn0 = assign5630_e5448_d_n0;
        locals.var_nj1_dn2 = assign5630_e5448_d_n2;

        let (assign5640_e5460, assign5640_e5460_d_n0, assign5640_e5460_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard92 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign5640_e5460;
        locals.var_dnj1_dv_dn0 = assign5640_e5460_d_n0;
        locals.var_dnj1_dv_dn2 = assign5640_e5460_d_n2;

        let (assign5700_e5709, assign5700_e5709_d_n0, assign5700_e5709_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign5700_e5693: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign5700_e5694: f64 = (locals.var_nj1 - assign5700_e5693);
        let assign5700_e5697: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign5700_e5698: f64 = (assign5700_e5694 / assign5700_e5697);
        let assign5700_e5701: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign5700_e5704: f64 = (locals.var_nj0 * p.p85);
        let assign5700_e5705: f64 = (assign5700_e5701 / assign5700_e5704);
        let assign5700_e5706: f64 = (assign5700_e5698 + assign5700_e5705);
        let assign5700_e5707: f64 = (locals.var_phitdinv * assign5700_e5706);
        (assign5700_e5707, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign5700_e5697) - (assign5700_e5694 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign5700_e5697 * assign5700_e5697)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign5700_e5704) - (assign5700_e5701 * (locals.var_nj0_dn0 * p.p85))) / (assign5700_e5704 * assign5700_e5704)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign5700_e5697) - (assign5700_e5694 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign5700_e5697 * assign5700_e5697)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign5700_e5704) - (assign5700_e5701 * (locals.var_nj0_dn2 * p.p85))) / (assign5700_e5704 * assign5700_e5704)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign5700_e5709;
        locals.var_dvmax_over_phitd_dv_dn0 = assign5700_e5709_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign5700_e5709_d_n2;

        let (assign5720_e5739,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign5720_e5735: f64 = (locals.var_nin * locals.var_nin);
        let assign5720_e5737: f64 = (assign5720_e5735 / locals.var_ndigat_i);
        (assign5720_e5737,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign5720_e5739;

        let (assign5730_e5755,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign5730_e5748: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign5730_e5751: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign5730_e5752: f64 = (assign5730_e5751).ln();
        let assign5730_e5753: f64 = (assign5730_e5748 * assign5730_e5752);
        (assign5730_e5753,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign5730_e5755;

        let assign5740_e5758: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign5740_e5758;

        let (assign5750_e5775, assign5750_e5775_d_n0, assign5750_e5775_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5750_e5770: f64 = (locals.var_vmax - locals.var_vha1);
        let assign5750_e5771: f64 = (p.p86 * assign5750_e5770);
        let assign5750_e5773: f64 = (assign5750_e5771 + locals.var_nfagat_i);
        (assign5750_e5773, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign5750_e5775;
        locals.var_nja10_dn0 = assign5750_e5775_d_n0;
        locals.var_nja10_dn2 = assign5750_e5775_d_n2;

        let (assign5760_e5790, assign5760_e5790_d_n0, assign5760_e5790_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5760_e5787: f64 = (p.p86 * locals.var_vha1);
        let assign5760_e5788: f64 = (locals.var_nfagat_i - assign5760_e5787);
        (assign5760_e5788, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5760_e5790;
        locals.var_nj0_dn0 = assign5760_e5790_d_n0;
        locals.var_nj0_dn2 = assign5760_e5790_d_n2;

        let (assign5770_e5805, assign5770_e5805_d_n0, assign5770_e5805_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5770_e5801: f64 = (p.p85 - locals.var_nja10);
        let assign5770_e5803: f64 = (assign5770_e5801 - 0.01);
        (assign5770_e5803, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5770_e5805;
        locals.var_tmf1_dn0 = assign5770_e5805_d_n0;
        locals.var_tmf1_dn2 = assign5770_e5805_d_n2;

        let (assign5780_e5820, assign5780_e5820_d_n0, assign5780_e5820_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5780_e5816: f64 = (4.0 * p.p85);
        let assign5780_e5818: f64 = (assign5780_e5816 * 0.01);
        (assign5780_e5818, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5780_e5820;
        locals.var_tmf2_dn0 = assign5780_e5820_d_n0;
        locals.var_tmf2_dn2 = assign5780_e5820_d_n2;

        let (assign5790_e5837, assign5790_e5837_d_n0, assign5790_e5837_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let (assign5790_e5835, assign5790_e5835_d_n0, assign5790_e5835_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5790_e5834: f64 = (-locals.var_tmf2);
                (assign5790_e5834, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5790_e5835, assign5790_e5835_d_n0, assign5790_e5835_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5790_e5837;
        locals.var_tmf2_dn0 = assign5790_e5837_d_n0;
        locals.var_tmf2_dn2 = assign5790_e5837_d_n2;

        let (assign5800_e5853, assign5800_e5853_d_n0, assign5800_e5853_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5800_e5848: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5800_e5850: f64 = (assign5800_e5848 + locals.var_tmf2);
        let assign5800_e5851: f64 = (assign5800_e5850).sqrt();
        (assign5800_e5851, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5800_e5851)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5800_e5851)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5800_e5853;
        locals.var_tmf2_dn0 = assign5800_e5853_d_n0;
        locals.var_tmf2_dn2 = assign5800_e5853_d_n2;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5810_e5870, assign5810_e5870_d_n0, assign5810_e5870_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5810_e5866: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5810_e5867: f64 = (1.0 + assign5810_e5866);
        let assign5810_e5868: f64 = (0.5 * assign5810_e5867);
        (assign5810_e5868, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign5810_e5870;
        locals.var_dfn_su_dn0 = assign5810_e5870_d_n0;
        locals.var_dfn_su_dn2 = assign5810_e5870_d_n2;

        let (assign5820_e5887, assign5820_e5887_d_n0, assign5820_e5887_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5820_e5883: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5820_e5884: f64 = (0.5 * assign5820_e5883);
        let assign5820_e5885: f64 = (p.p85 - assign5820_e5884);
        (assign5820_e5885, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign5820_e5887;
        locals.var_nja11_dn0 = assign5820_e5887_d_n0;
        locals.var_nja11_dn2 = assign5820_e5887_d_n2;

        let (assign5830_e5902, assign5830_e5902_d_n0, assign5830_e5902_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5830_e5898: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign5830_e5900: f64 = (assign5830_e5898 - 0.01);
        (assign5830_e5900, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5830_e5902;
        locals.var_tmf1_dn0 = assign5830_e5902_d_n0;
        locals.var_tmf1_dn2 = assign5830_e5902_d_n2;

        let (assign5840_e5917, assign5840_e5917_d_n0, assign5840_e5917_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5840_e5913: f64 = (4.0 * locals.var_nfagat_i);
        let assign5840_e5915: f64 = (assign5840_e5913 * 0.01);
        (assign5840_e5915, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5840_e5917;
        locals.var_tmf2_dn0 = assign5840_e5917_d_n0;
        locals.var_tmf2_dn2 = assign5840_e5917_d_n2;

        let (assign5850_e5934, assign5850_e5934_d_n0, assign5850_e5934_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let (assign5850_e5932, assign5850_e5932_d_n0, assign5850_e5932_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5850_e5931: f64 = (-locals.var_tmf2);
                (assign5850_e5931, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5850_e5932, assign5850_e5932_d_n0, assign5850_e5932_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5850_e5934;
        locals.var_tmf2_dn0 = assign5850_e5934_d_n0;
        locals.var_tmf2_dn2 = assign5850_e5934_d_n2;

        let (assign5860_e5950, assign5860_e5950_d_n0, assign5860_e5950_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5860_e5945: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5860_e5947: f64 = (assign5860_e5945 + locals.var_tmf2);
        let assign5860_e5948: f64 = (assign5860_e5947).sqrt();
        (assign5860_e5948, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5860_e5948)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5860_e5948)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5860_e5950;
        locals.var_tmf2_dn0 = assign5860_e5950_d_n0;
        locals.var_tmf2_dn2 = assign5860_e5950_d_n2;

        let (assign5870_e5967, assign5870_e5967_d_n0, assign5870_e5967_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5870_e5963: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5870_e5964: f64 = (1.0 + assign5870_e5963);
        let assign5870_e5965: f64 = (0.5 * assign5870_e5964);
        (assign5870_e5965, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign5870_e5967;
        locals.var_dfn_sl_dn0 = assign5870_e5967_d_n0;
        locals.var_dfn_sl_dn2 = assign5870_e5967_d_n2;

        let (assign5880_e5984, assign5880_e5984_d_n0, assign5880_e5984_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5880_e5980: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5880_e5981: f64 = (0.5 * assign5880_e5980);
        let assign5880_e5982: f64 = (locals.var_nfagat_i + assign5880_e5981);
        (assign5880_e5982, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign5880_e5984;
        locals.var_nj1_dn0 = assign5880_e5984_d_n0;
        locals.var_nj1_dn2 = assign5880_e5984_d_n2;

        let (assign5890_e5999, assign5890_e5999_d_n0, assign5890_e5999_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5890_e5995: f64 = (p.p85 - locals.var_nj0);
        let assign5890_e5997: f64 = (assign5890_e5995 - 0.01);
        (assign5890_e5997, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5890_e5999;
        locals.var_tmf1_dn0 = assign5890_e5999_d_n0;
        locals.var_tmf1_dn2 = assign5890_e5999_d_n2;

        let (assign5900_e6014, assign5900_e6014_d_n0, assign5900_e6014_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5900_e6010: f64 = (4.0 * p.p85);
        let assign5900_e6012: f64 = (assign5900_e6010 * 0.01);
        (assign5900_e6012, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5900_e6014;
        locals.var_tmf2_dn0 = assign5900_e6014_d_n0;
        locals.var_tmf2_dn2 = assign5900_e6014_d_n2;

        let (assign5910_e6031, assign5910_e6031_d_n0, assign5910_e6031_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let (assign5910_e6029, assign5910_e6029_d_n0, assign5910_e6029_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5910_e6028: f64 = (-locals.var_tmf2);
                (assign5910_e6028, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5910_e6029, assign5910_e6029_d_n0, assign5910_e6029_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5910_e6031;
        locals.var_tmf2_dn0 = assign5910_e6031_d_n0;
        locals.var_tmf2_dn2 = assign5910_e6031_d_n2;

        let (assign5920_e6047, assign5920_e6047_d_n0, assign5920_e6047_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5920_e6042: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5920_e6044: f64 = (assign5920_e6042 + locals.var_tmf2);
        let assign5920_e6045: f64 = (assign5920_e6044).sqrt();
        (assign5920_e6045, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5920_e6045)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5920_e6045)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5920_e6047;
        locals.var_tmf2_dn0 = assign5920_e6047_d_n0;
        locals.var_tmf2_dn2 = assign5920_e6047_d_n2;

        let (assign5930_e6064, assign5930_e6064_d_n0, assign5930_e6064_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5930_e6060: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5930_e6061: f64 = (0.5 * assign5930_e6060);
        let assign5930_e6062: f64 = (p.p85 - assign5930_e6061);
        (assign5930_e6062, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5930_e6064;
        locals.var_nj0_dn0 = assign5930_e6064_d_n0;
        locals.var_nj0_dn2 = assign5930_e6064_d_n2;

        let (assign5940_e6079, assign5940_e6079_d_n0, assign5940_e6079_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5940_e6075: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign5940_e6077: f64 = (assign5940_e6075 - 0.01);
        (assign5940_e6077, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign5940_e6079;
        locals.var_tmf1_dn0 = assign5940_e6079_d_n0;
        locals.var_tmf1_dn2 = assign5940_e6079_d_n2;

        let (assign5950_e6094, assign5950_e6094_d_n0, assign5950_e6094_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5950_e6090: f64 = (4.0 * locals.var_nfagat_i);
        let assign5950_e6092: f64 = (assign5950_e6090 * 0.01);
        (assign5950_e6092, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5950_e6094;
        locals.var_tmf2_dn0 = assign5950_e6094_d_n0;
        locals.var_tmf2_dn2 = assign5950_e6094_d_n2;

        let (assign5960_e6111, assign5960_e6111_d_n0, assign5960_e6111_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let (assign5960_e6109, assign5960_e6109_d_n0, assign5960_e6109_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign5960_e6108: f64 = (-locals.var_tmf2);
                (assign5960_e6108, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign5960_e6109, assign5960_e6109_d_n0, assign5960_e6109_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5960_e6111;
        locals.var_tmf2_dn0 = assign5960_e6111_d_n0;
        locals.var_tmf2_dn2 = assign5960_e6111_d_n2;

        let (assign5970_e6127, assign5970_e6127_d_n0, assign5970_e6127_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5970_e6122: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5970_e6124: f64 = (assign5970_e6122 + locals.var_tmf2);
        let assign5970_e6125: f64 = (assign5970_e6124).sqrt();
        (assign5970_e6125, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5970_e6125)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5970_e6125)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign5970_e6127;
        locals.var_tmf2_dn0 = assign5970_e6127_d_n0;
        locals.var_tmf2_dn2 = assign5970_e6127_d_n2;

        let (assign5980_e6144, assign5980_e6144_d_n0, assign5980_e6144_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5980_e6140: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5980_e6141: f64 = (0.5 * assign5980_e6140);
        let assign5980_e6142: f64 = (locals.var_nfagat_i + assign5980_e6141);
        (assign5980_e6142, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign5980_e6144;
        locals.var_nj0_dn0 = assign5980_e6144_d_n0;
        locals.var_nj0_dn2 = assign5980_e6144_d_n2;

        let (assign5990_e6159, assign5990_e6159_d_n0, assign5990_e6159_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5990_e6155: f64 = (p.p86 * locals.var_dfn_su);
        let assign5990_e6157: f64 = (assign5990_e6155 * locals.var_dfn_sl);
        (assign5990_e6157, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign5990_e6155 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign5990_e6155 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign5990_e6159;
        locals.var_dnj1_dv_dn0 = assign5990_e6159_d_n0;
        locals.var_dnj1_dv_dn2 = assign5990_e6159_d_n2;

        let (assign6000_e6171, assign6000_e6171_d_n0, assign6000_e6171_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign6000_e6171;
        locals.var_nj0_dn0 = assign6000_e6171_d_n0;
        locals.var_nj0_dn2 = assign6000_e6171_d_n2;

        let (assign6010_e6183, assign6010_e6183_d_n0, assign6010_e6183_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign6010_e6183;
        locals.var_nj1_dn0 = assign6010_e6183_d_n0;
        locals.var_nj1_dn2 = assign6010_e6183_d_n2;

        let (assign6020_e6195, assign6020_e6195_d_n0, assign6020_e6195_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) && (locals.var_guard95 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign6020_e6195;
        locals.var_dnj1_dv_dn0 = assign6020_e6195_d_n0;
        locals.var_dnj1_dv_dn2 = assign6020_e6195_d_n2;

        let (assign6080_e6444, assign6080_e6444_d_n0, assign6080_e6444_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign6080_e6428: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign6080_e6429: f64 = (locals.var_nj1 - assign6080_e6428);
        let assign6080_e6432: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign6080_e6433: f64 = (assign6080_e6429 / assign6080_e6432);
        let assign6080_e6436: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign6080_e6439: f64 = (locals.var_nj0 * p.p85);
        let assign6080_e6440: f64 = (assign6080_e6436 / assign6080_e6439);
        let assign6080_e6441: f64 = (assign6080_e6433 + assign6080_e6440);
        let assign6080_e6442: f64 = (locals.var_phitdinv * assign6080_e6441);
        (assign6080_e6442, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign6080_e6432) - (assign6080_e6429 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign6080_e6432 * assign6080_e6432)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign6080_e6439) - (assign6080_e6436 * (locals.var_nj0_dn0 * p.p85))) / (assign6080_e6439 * assign6080_e6439)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign6080_e6432) - (assign6080_e6429 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign6080_e6432 * assign6080_e6432)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign6080_e6439) - (assign6080_e6436 * (locals.var_nj0_dn2 * p.p85))) / (assign6080_e6439 * assign6080_e6439)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign6080_e6444;
        locals.var_dvmax_over_phitd_dv_dn0 = assign6080_e6444_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign6080_e6444_d_n2;

        let (assign6100_e6469, assign6100_e6469_d_n0, assign6100_e6469_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign6100_e6467: f64 = (locals.var_idmultbot - 1.0);
        (assign6100_e6467, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign6100_e6469;
        locals.var_idmultbot_dn0 = assign6100_e6469_d_n0;
        locals.var_idmultbot_dn2 = assign6100_e6469_d_n2;

        let (assign6210_e6642, assign6210_e6642_d_n0, assign6210_e6642_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard76 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign6210_e6642;
        locals.var_idmultbot_dn0 = assign6210_e6642_d_n0;
        locals.var_idmultbot_dn2 = assign6210_e6642_d_n2;

        let assign8740_e10192: f64 = if (!(((locals.var_ab_i == 0.0) && (locals.var_ls_i == 0.0)) && (locals.var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard153 = assign8740_e10192;

        let assign8820_e10264: f64 = if locals.var_v2 < locals.var_vmax { 1.0 } else { 0.0 };
        locals.var_guard154 = assign8820_e10264;

        let (assign8880_e10405,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) {
        let assign8880_e10401: f64 = (locals.var_nin * locals.var_nin);
        let assign8880_e10403: f64 = (assign8880_e10401 / locals.var_ndibot_i);
        (assign8880_e10403,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign8880_e10405;

        let (assign8890_e10420,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) {
        let assign8890_e10413: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign8890_e10416: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign8890_e10417: f64 = (assign8890_e10416).ln();
        let assign8890_e10418: f64 = (assign8890_e10413 * assign8890_e10417);
        (assign8890_e10418,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign8890_e10420;

        let assign8900_e10423: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign8900_e10423;

        let (assign8910_e10439, assign8910_e10439_d_n0, assign8910_e10439_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8910_e10434: f64 = (locals.var_v2 - locals.var_vha1);
        let assign8910_e10435: f64 = (p.p86 * assign8910_e10434);
        let assign8910_e10437: f64 = (assign8910_e10435 + locals.var_nfabot_i);
        (assign8910_e10437, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign8910_e10439;
        locals.var_nja10_dn0 = assign8910_e10439_d_n0;
        locals.var_nja10_dn2 = assign8910_e10439_d_n2;

        let (assign8920_e10453, assign8920_e10453_d_n0, assign8920_e10453_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8920_e10450: f64 = (p.p86 * locals.var_vha1);
        let assign8920_e10451: f64 = (locals.var_nfabot_i - assign8920_e10450);
        (assign8920_e10451, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign8920_e10453;
        locals.var_nj0_dn0 = assign8920_e10453_d_n0;
        locals.var_nj0_dn2 = assign8920_e10453_d_n2;

        let (assign8930_e10467, assign8930_e10467_d_n0, assign8930_e10467_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8930_e10463: f64 = (p.p85 - locals.var_nja10);
        let assign8930_e10465: f64 = (assign8930_e10463 - 0.01);
        (assign8930_e10465, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign8930_e10467;
        locals.var_tmf1_dn0 = assign8930_e10467_d_n0;
        locals.var_tmf1_dn2 = assign8930_e10467_d_n2;

        let (assign8940_e10481, assign8940_e10481_d_n0, assign8940_e10481_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8940_e10477: f64 = (4.0 * p.p85);
        let assign8940_e10479: f64 = (assign8940_e10477 * 0.01);
        (assign8940_e10479, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign8940_e10481;
        locals.var_tmf2_dn0 = assign8940_e10481_d_n0;
        locals.var_tmf2_dn2 = assign8940_e10481_d_n2;

        let (assign8950_e10497, assign8950_e10497_d_n0, assign8950_e10497_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let (assign8950_e10495, assign8950_e10495_d_n0, assign8950_e10495_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign8950_e10494: f64 = (-locals.var_tmf2);
                (assign8950_e10494, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign8950_e10495, assign8950_e10495_d_n0, assign8950_e10495_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign8950_e10497;
        locals.var_tmf2_dn0 = assign8950_e10497_d_n0;
        locals.var_tmf2_dn2 = assign8950_e10497_d_n2;

        let (assign8960_e10512, assign8960_e10512_d_n0, assign8960_e10512_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8960_e10507: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign8960_e10509: f64 = (assign8960_e10507 + locals.var_tmf2);
        let assign8960_e10510: f64 = (assign8960_e10509).sqrt();
        (assign8960_e10510, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign8960_e10510)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign8960_e10510)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign8960_e10512;
        locals.var_tmf2_dn0 = assign8960_e10512_d_n0;
        locals.var_tmf2_dn2 = assign8960_e10512_d_n2;

        let (assign8970_e10528, assign8970_e10528_d_n0, assign8970_e10528_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8970_e10524: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign8970_e10525: f64 = (0.5 * assign8970_e10524);
        let assign8970_e10526: f64 = (p.p85 - assign8970_e10525);
        (assign8970_e10526, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign8970_e10528;
        locals.var_nja11_dn0 = assign8970_e10528_d_n0;
        locals.var_nja11_dn2 = assign8970_e10528_d_n2;

        let (assign8980_e10542, assign8980_e10542_d_n0, assign8980_e10542_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8980_e10538: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign8980_e10540: f64 = (assign8980_e10538 - 0.01);
        (assign8980_e10540, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign8980_e10542;
        locals.var_tmf1_dn0 = assign8980_e10542_d_n0;
        locals.var_tmf1_dn2 = assign8980_e10542_d_n2;

        let (assign8990_e10556, assign8990_e10556_d_n0, assign8990_e10556_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8990_e10552: f64 = (4.0 * locals.var_nfabot_i);
        let assign8990_e10554: f64 = (assign8990_e10552 * 0.01);
        (assign8990_e10554, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign8990_e10556;
        locals.var_tmf2_dn0 = assign8990_e10556_d_n0;
        locals.var_tmf2_dn2 = assign8990_e10556_d_n2;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9000_e10572, assign9000_e10572_d_n0, assign9000_e10572_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let (assign9000_e10570, assign9000_e10570_d_n0, assign9000_e10570_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9000_e10569: f64 = (-locals.var_tmf2);
                (assign9000_e10569, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9000_e10570, assign9000_e10570_d_n0, assign9000_e10570_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9000_e10572;
        locals.var_tmf2_dn0 = assign9000_e10572_d_n0;
        locals.var_tmf2_dn2 = assign9000_e10572_d_n2;

        let (assign9010_e10587, assign9010_e10587_d_n0, assign9010_e10587_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9010_e10582: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9010_e10584: f64 = (assign9010_e10582 + locals.var_tmf2);
        let assign9010_e10585: f64 = (assign9010_e10584).sqrt();
        (assign9010_e10585, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9010_e10585)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9010_e10585)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9010_e10587;
        locals.var_tmf2_dn0 = assign9010_e10587_d_n0;
        locals.var_tmf2_dn2 = assign9010_e10587_d_n2;

        let (assign9020_e10603, assign9020_e10603_d_n0, assign9020_e10603_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9020_e10599: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9020_e10600: f64 = (0.5 * assign9020_e10599);
        let assign9020_e10601: f64 = (locals.var_nfabot_i + assign9020_e10600);
        (assign9020_e10601, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign9020_e10603;
        locals.var_nj1_dn0 = assign9020_e10603_d_n0;
        locals.var_nj1_dn2 = assign9020_e10603_d_n2;

        let (assign9030_e10617, assign9030_e10617_d_n0, assign9030_e10617_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9030_e10613: f64 = (p.p85 - locals.var_nj0);
        let assign9030_e10615: f64 = (assign9030_e10613 - 0.01);
        (assign9030_e10615, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9030_e10617;
        locals.var_tmf1_dn0 = assign9030_e10617_d_n0;
        locals.var_tmf1_dn2 = assign9030_e10617_d_n2;

        let (assign9040_e10631, assign9040_e10631_d_n0, assign9040_e10631_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9040_e10627: f64 = (4.0 * p.p85);
        let assign9040_e10629: f64 = (assign9040_e10627 * 0.01);
        (assign9040_e10629, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9040_e10631;
        locals.var_tmf2_dn0 = assign9040_e10631_d_n0;
        locals.var_tmf2_dn2 = assign9040_e10631_d_n2;

        let (assign9050_e10647, assign9050_e10647_d_n0, assign9050_e10647_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let (assign9050_e10645, assign9050_e10645_d_n0, assign9050_e10645_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9050_e10644: f64 = (-locals.var_tmf2);
                (assign9050_e10644, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9050_e10645, assign9050_e10645_d_n0, assign9050_e10645_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9050_e10647;
        locals.var_tmf2_dn0 = assign9050_e10647_d_n0;
        locals.var_tmf2_dn2 = assign9050_e10647_d_n2;

        let (assign9060_e10662, assign9060_e10662_d_n0, assign9060_e10662_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9060_e10657: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9060_e10659: f64 = (assign9060_e10657 + locals.var_tmf2);
        let assign9060_e10660: f64 = (assign9060_e10659).sqrt();
        (assign9060_e10660, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9060_e10660)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9060_e10660)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9060_e10662;
        locals.var_tmf2_dn0 = assign9060_e10662_d_n0;
        locals.var_tmf2_dn2 = assign9060_e10662_d_n2;

        let (assign9070_e10678, assign9070_e10678_d_n0, assign9070_e10678_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9070_e10674: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9070_e10675: f64 = (0.5 * assign9070_e10674);
        let assign9070_e10676: f64 = (p.p85 - assign9070_e10675);
        (assign9070_e10676, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9070_e10678;
        locals.var_nj0_dn0 = assign9070_e10678_d_n0;
        locals.var_nj0_dn2 = assign9070_e10678_d_n2;

        let (assign9080_e10692, assign9080_e10692_d_n0, assign9080_e10692_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9080_e10688: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign9080_e10690: f64 = (assign9080_e10688 - 0.01);
        (assign9080_e10690, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9080_e10692;
        locals.var_tmf1_dn0 = assign9080_e10692_d_n0;
        locals.var_tmf1_dn2 = assign9080_e10692_d_n2;

        let (assign9090_e10706, assign9090_e10706_d_n0, assign9090_e10706_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9090_e10702: f64 = (4.0 * locals.var_nfabot_i);
        let assign9090_e10704: f64 = (assign9090_e10702 * 0.01);
        (assign9090_e10704, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9090_e10706;
        locals.var_tmf2_dn0 = assign9090_e10706_d_n0;
        locals.var_tmf2_dn2 = assign9090_e10706_d_n2;

        let (assign9100_e10722, assign9100_e10722_d_n0, assign9100_e10722_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let (assign9100_e10720, assign9100_e10720_d_n0, assign9100_e10720_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9100_e10719: f64 = (-locals.var_tmf2);
                (assign9100_e10719, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9100_e10720, assign9100_e10720_d_n0, assign9100_e10720_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9100_e10722;
        locals.var_tmf2_dn0 = assign9100_e10722_d_n0;
        locals.var_tmf2_dn2 = assign9100_e10722_d_n2;

        let (assign9110_e10737, assign9110_e10737_d_n0, assign9110_e10737_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9110_e10732: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9110_e10734: f64 = (assign9110_e10732 + locals.var_tmf2);
        let assign9110_e10735: f64 = (assign9110_e10734).sqrt();
        (assign9110_e10735, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9110_e10735)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9110_e10735)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9110_e10737;
        locals.var_tmf2_dn0 = assign9110_e10737_d_n0;
        locals.var_tmf2_dn2 = assign9110_e10737_d_n2;

        let (assign9120_e10753, assign9120_e10753_d_n0, assign9120_e10753_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign9120_e10749: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9120_e10750: f64 = (0.5 * assign9120_e10749);
        let assign9120_e10751: f64 = (locals.var_nfabot_i + assign9120_e10750);
        (assign9120_e10751, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9120_e10753;
        locals.var_nj0_dn0 = assign9120_e10753_d_n0;
        locals.var_nj0_dn2 = assign9120_e10753_d_n2;

        let (assign9130_e10764, assign9130_e10764_d_n0, assign9130_e10764_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9130_e10764;
        locals.var_nj0_dn0 = assign9130_e10764_d_n0;
        locals.var_nj0_dn2 = assign9130_e10764_d_n2;

        let (assign9140_e10775, assign9140_e10775_d_n0, assign9140_e10775_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard157 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign9140_e10775;
        locals.var_nj1_dn0 = assign9140_e10775_d_n0;
        locals.var_nj1_dn2 = assign9140_e10775_d_n2;

        let assign9150_e10779: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9150_e10783: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9150_e10784: f64 = (locals.var_vha1 * assign9150_e10783);
        let assign9150_e10787: f64 = (locals.var_nj0 * p.p85);
        let assign9150_e10788: f64 = (assign9150_e10784 / assign9150_e10787);
        let assign9150_e10789: f64 = (assign9150_e10779 + assign9150_e10788);
        let assign9150_e10790: f64 = (locals.var_phitdinv * assign9150_e10789);
        let assign9150_e10791: f64 = (assign9150_e10790).abs();
        let assign9150_e10793: f64 = if assign9150_e10791 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign9150_e10793;

        let (assign9160_e10818, assign9160_e10818_d_n0, assign9160_e10818_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard158 != 0.0)) {
        let assign9160_e10804: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9160_e10808: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9160_e10809: f64 = (locals.var_vha1 * assign9160_e10808);
        let assign9160_e10812: f64 = (locals.var_nj0 * p.p85);
        let assign9160_e10813: f64 = (assign9160_e10809 / assign9160_e10812);
        let assign9160_e10814: f64 = (assign9160_e10804 + assign9160_e10813);
        let assign9160_e10815: f64 = (locals.var_phitdinv * assign9160_e10814);
        let assign9160_e10816: f64 = (assign9160_e10815).exp();
        (assign9160_e10816, (assign9160_e10816 * (locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign9160_e10812) - (assign9160_e10809 * (locals.var_nj0_dn0 * p.p85))) / (assign9160_e10812 * assign9160_e10812))))), (assign9160_e10816 * (locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign9160_e10812) - (assign9160_e10809 * (locals.var_nj0_dn2 * p.p85))) / (assign9160_e10812 * assign9160_e10812))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign9160_e10818;
        locals.var_idmultbot_dn0 = assign9160_e10818_d_n0;
        locals.var_idmultbot_dn2 = assign9160_e10818_d_n2;

        let assign9170_e10822: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9170_e10826: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9170_e10827: f64 = (locals.var_vha1 * assign9170_e10826);
        let assign9170_e10830: f64 = (locals.var_nj0 * p.p85);
        let assign9170_e10831: f64 = (assign9170_e10827 / assign9170_e10830);
        let assign9170_e10832: f64 = (assign9170_e10822 + assign9170_e10831);
        let assign9170_e10833: f64 = (locals.var_phitdinv * assign9170_e10832);
        let assign9170_e10835: f64 = (-230.25850929940458);
        let assign9170_e10836: f64 = if assign9170_e10833 < assign9170_e10835 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign9170_e10836;

        let (assign9180_e10916, assign9180_e10916_d_n0, assign9180_e10916_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign9180_e10850: f64 = (-230.25850929940458);
        let assign9180_e10854: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9180_e10858: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9180_e10859: f64 = (locals.var_vha1 * assign9180_e10858);
        let assign9180_e10862: f64 = (locals.var_nj0 * p.p85);
        let assign9180_e10863: f64 = (assign9180_e10859 / assign9180_e10862);
        let assign9180_e10864: f64 = (assign9180_e10854 + assign9180_e10863);
        let assign9180_e10865: f64 = (locals.var_phitdinv * assign9180_e10864);
        let assign9180_e10866: f64 = (assign9180_e10850 - assign9180_e10865);
        let assign9180_e10870: f64 = (-230.25850929940458);
        let assign9180_e10874: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9180_e10878: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9180_e10879: f64 = (locals.var_vha1 * assign9180_e10878);
        let assign9180_e10882: f64 = (locals.var_nj0 * p.p85);
        let assign9180_e10883: f64 = (assign9180_e10879 / assign9180_e10882);
        let assign9180_e10884: f64 = (assign9180_e10874 + assign9180_e10883);
        let assign9180_e10885: f64 = (locals.var_phitdinv * assign9180_e10884);
        let assign9180_e10886: f64 = (assign9180_e10870 - assign9180_e10885);
        let assign9180_e10889: f64 = (-230.25850929940458);
        let assign9180_e10893: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9180_e10897: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9180_e10898: f64 = (locals.var_vha1 * assign9180_e10897);
        let assign9180_e10901: f64 = (locals.var_nj0 * p.p85);
        let assign9180_e10902: f64 = (assign9180_e10898 / assign9180_e10901);
        let assign9180_e10903: f64 = (assign9180_e10893 + assign9180_e10902);
        let assign9180_e10904: f64 = (locals.var_phitdinv * assign9180_e10903);
        let assign9180_e10905: f64 = (assign9180_e10889 - assign9180_e10904);
        let assign9180_e10907: f64 = (assign9180_e10905 * 0.3333333333333333);
        let assign9180_e10908: f64 = (1.0 + assign9180_e10907);
        let assign9180_e10909: f64 = (assign9180_e10886 * assign9180_e10908);
        let assign9180_e10910: f64 = (0.5 * assign9180_e10909);
        let assign9180_e10911: f64 = (1.0 + assign9180_e10910);
        let assign9180_e10912: f64 = (assign9180_e10866 * assign9180_e10911);
        let assign9180_e10913: f64 = (1.0 + assign9180_e10912);
        let assign9180_e10914: f64 = (1e-100 / assign9180_e10913);
        (assign9180_e10914, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign9180_e10862) - (assign9180_e10859 * (locals.var_nj0_dn0 * p.p85))) / (assign9180_e10862 * assign9180_e10862))))) * assign9180_e10911) + (assign9180_e10866 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign9180_e10882) - (assign9180_e10879 * (locals.var_nj0_dn0 * p.p85))) / (assign9180_e10882 * assign9180_e10882))))) * assign9180_e10908) + (assign9180_e10886 * ((-(locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign9180_e10901) - (assign9180_e10898 * (locals.var_nj0_dn0 * p.p85))) / (assign9180_e10901 * assign9180_e10901))))) * 0.3333333333333333))))))) / (assign9180_e10913 * assign9180_e10913))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign9180_e10862) - (assign9180_e10859 * (locals.var_nj0_dn2 * p.p85))) / (assign9180_e10862 * assign9180_e10862))))) * assign9180_e10911) + (assign9180_e10866 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign9180_e10882) - (assign9180_e10879 * (locals.var_nj0_dn2 * p.p85))) / (assign9180_e10882 * assign9180_e10882))))) * assign9180_e10908) + (assign9180_e10886 * ((-(locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign9180_e10901) - (assign9180_e10898 * (locals.var_nj0_dn2 * p.p85))) / (assign9180_e10901 * assign9180_e10901))))) * 0.3333333333333333))))))) / (assign9180_e10913 * assign9180_e10913))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign9180_e10916;
        locals.var_idmultbot_dn0 = assign9180_e10916_d_n0;
        locals.var_idmultbot_dn2 = assign9180_e10916_d_n2;

        let (assign9190_e10994, assign9190_e10994_d_n0, assign9190_e10994_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) {
        let assign9190_e10933: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9190_e10937: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9190_e10938: f64 = (locals.var_vha1 * assign9190_e10937);
        let assign9190_e10941: f64 = (locals.var_nj0 * p.p85);
        let assign9190_e10942: f64 = (assign9190_e10938 / assign9190_e10941);
        let assign9190_e10943: f64 = (assign9190_e10933 + assign9190_e10942);
        let assign9190_e10944: f64 = (locals.var_phitdinv * assign9190_e10943);
        let assign9190_e10946: f64 = (assign9190_e10944 - 230.25850929940458);
        let assign9190_e10952: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9190_e10956: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9190_e10957: f64 = (locals.var_vha1 * assign9190_e10956);
        let assign9190_e10960: f64 = (locals.var_nj0 * p.p85);
        let assign9190_e10961: f64 = (assign9190_e10957 / assign9190_e10960);
        let assign9190_e10962: f64 = (assign9190_e10952 + assign9190_e10961);
        let assign9190_e10963: f64 = (locals.var_phitdinv * assign9190_e10962);
        let assign9190_e10965: f64 = (assign9190_e10963 - 230.25850929940458);
        let assign9190_e10970: f64 = (locals.var_v2 / locals.var_nj1);
        let assign9190_e10974: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign9190_e10975: f64 = (locals.var_vha1 * assign9190_e10974);
        let assign9190_e10978: f64 = (locals.var_nj0 * p.p85);
        let assign9190_e10979: f64 = (assign9190_e10975 / assign9190_e10978);
        let assign9190_e10980: f64 = (assign9190_e10970 + assign9190_e10979);
        let assign9190_e10981: f64 = (locals.var_phitdinv * assign9190_e10980);
        let assign9190_e10983: f64 = (assign9190_e10981 - 230.25850929940458);
        let assign9190_e10985: f64 = (assign9190_e10983 * 0.3333333333333333);
        let assign9190_e10986: f64 = (1.0 + assign9190_e10985);
        let assign9190_e10987: f64 = (assign9190_e10965 * assign9190_e10986);
        let assign9190_e10988: f64 = (0.5 * assign9190_e10987);
        let assign9190_e10989: f64 = (1.0 + assign9190_e10988);
        let assign9190_e10990: f64 = (assign9190_e10946 * assign9190_e10989);
        let assign9190_e10991: f64 = (1.0 + assign9190_e10990);
        let assign9190_e10992: f64 = (1e100 * assign9190_e10991);
        (assign9190_e10992, (1e100 * (((locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign9190_e10941) - (assign9190_e10938 * (locals.var_nj0_dn0 * p.p85))) / (assign9190_e10941 * assign9190_e10941)))) * assign9190_e10989) + (assign9190_e10946 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign9190_e10960) - (assign9190_e10957 * (locals.var_nj0_dn0 * p.p85))) / (assign9190_e10960 * assign9190_e10960)))) * assign9190_e10986) + (assign9190_e10965 * ((locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign9190_e10978) - (assign9190_e10975 * (locals.var_nj0_dn0 * p.p85))) / (assign9190_e10978 * assign9190_e10978)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign9190_e10941) - (assign9190_e10938 * (locals.var_nj0_dn2 * p.p85))) / (assign9190_e10941 * assign9190_e10941)))) * assign9190_e10989) + (assign9190_e10946 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign9190_e10960) - (assign9190_e10957 * (locals.var_nj0_dn2 * p.p85))) / (assign9190_e10960 * assign9190_e10960)))) * assign9190_e10986) + (assign9190_e10965 * ((locals.var_phitdinv * ((-((locals.var_v2 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign9190_e10978) - (assign9190_e10975 * (locals.var_nj0_dn2 * p.p85))) / (assign9190_e10978 * assign9190_e10978)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign9190_e10994;
        locals.var_idmultbot_dn0 = assign9190_e10994_d_n0;
        locals.var_idmultbot_dn2 = assign9190_e10994_d_n2;

        let (assign9200_e11006,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) {
        let assign9200_e11002: f64 = (locals.var_nin * locals.var_nin);
        let assign9200_e11004: f64 = (assign9200_e11002 / locals.var_ndisti_i);
        (assign9200_e11004,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign9200_e11006;

        let (assign9210_e11021,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) {
        let assign9210_e11014: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign9210_e11017: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign9210_e11018: f64 = (assign9210_e11017).ln();
        let assign9210_e11019: f64 = (assign9210_e11014 * assign9210_e11018);
        (assign9210_e11019,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign9210_e11021;

        let assign9220_e11024: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign9220_e11024;

        let (assign9230_e11040, assign9230_e11040_d_n0, assign9230_e11040_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9230_e11035: f64 = (locals.var_v2 - locals.var_vha1);
        let assign9230_e11036: f64 = (p.p86 * assign9230_e11035);
        let assign9230_e11038: f64 = (assign9230_e11036 + locals.var_nfasti_i);
        (assign9230_e11038, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign9230_e11040;
        locals.var_nja10_dn0 = assign9230_e11040_d_n0;
        locals.var_nja10_dn2 = assign9230_e11040_d_n2;

        let (assign9240_e11054, assign9240_e11054_d_n0, assign9240_e11054_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9240_e11051: f64 = (p.p86 * locals.var_vha1);
        let assign9240_e11052: f64 = (locals.var_nfasti_i - assign9240_e11051);
        (assign9240_e11052, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9240_e11054;
        locals.var_nj0_dn0 = assign9240_e11054_d_n0;
        locals.var_nj0_dn2 = assign9240_e11054_d_n2;

        let (assign9250_e11068, assign9250_e11068_d_n0, assign9250_e11068_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9250_e11064: f64 = (p.p85 - locals.var_nja10);
        let assign9250_e11066: f64 = (assign9250_e11064 - 0.01);
        (assign9250_e11066, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9250_e11068;
        locals.var_tmf1_dn0 = assign9250_e11068_d_n0;
        locals.var_tmf1_dn2 = assign9250_e11068_d_n2;

        let (assign9260_e11082, assign9260_e11082_d_n0, assign9260_e11082_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9260_e11078: f64 = (4.0 * p.p85);
        let assign9260_e11080: f64 = (assign9260_e11078 * 0.01);
        (assign9260_e11080, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9260_e11082;
        locals.var_tmf2_dn0 = assign9260_e11082_d_n0;
        locals.var_tmf2_dn2 = assign9260_e11082_d_n2;

        let (assign9270_e11098, assign9270_e11098_d_n0, assign9270_e11098_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let (assign9270_e11096, assign9270_e11096_d_n0, assign9270_e11096_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9270_e11095: f64 = (-locals.var_tmf2);
                (assign9270_e11095, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9270_e11096, assign9270_e11096_d_n0, assign9270_e11096_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9270_e11098;
        locals.var_tmf2_dn0 = assign9270_e11098_d_n0;
        locals.var_tmf2_dn2 = assign9270_e11098_d_n2;

        let (assign9280_e11113, assign9280_e11113_d_n0, assign9280_e11113_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9280_e11108: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9280_e11110: f64 = (assign9280_e11108 + locals.var_tmf2);
        let assign9280_e11111: f64 = (assign9280_e11110).sqrt();
        (assign9280_e11111, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9280_e11111)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9280_e11111)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9280_e11113;
        locals.var_tmf2_dn0 = assign9280_e11113_d_n0;
        locals.var_tmf2_dn2 = assign9280_e11113_d_n2;

        let (assign9290_e11129, assign9290_e11129_d_n0, assign9290_e11129_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9290_e11125: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9290_e11126: f64 = (0.5 * assign9290_e11125);
        let assign9290_e11127: f64 = (p.p85 - assign9290_e11126);
        (assign9290_e11127, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign9290_e11129;
        locals.var_nja11_dn0 = assign9290_e11129_d_n0;
        locals.var_nja11_dn2 = assign9290_e11129_d_n2;

        let (assign9300_e11143, assign9300_e11143_d_n0, assign9300_e11143_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9300_e11139: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign9300_e11141: f64 = (assign9300_e11139 - 0.01);
        (assign9300_e11141, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9300_e11143;
        locals.var_tmf1_dn0 = assign9300_e11143_d_n0;
        locals.var_tmf1_dn2 = assign9300_e11143_d_n2;

        let (assign9310_e11157, assign9310_e11157_d_n0, assign9310_e11157_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9310_e11153: f64 = (4.0 * locals.var_nfasti_i);
        let assign9310_e11155: f64 = (assign9310_e11153 * 0.01);
        (assign9310_e11155, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9310_e11157;
        locals.var_tmf2_dn0 = assign9310_e11157_d_n0;
        locals.var_tmf2_dn2 = assign9310_e11157_d_n2;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9320_e11173, assign9320_e11173_d_n0, assign9320_e11173_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let (assign9320_e11171, assign9320_e11171_d_n0, assign9320_e11171_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9320_e11170: f64 = (-locals.var_tmf2);
                (assign9320_e11170, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9320_e11171, assign9320_e11171_d_n0, assign9320_e11171_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9320_e11173;
        locals.var_tmf2_dn0 = assign9320_e11173_d_n0;
        locals.var_tmf2_dn2 = assign9320_e11173_d_n2;

        let (assign9330_e11188, assign9330_e11188_d_n0, assign9330_e11188_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9330_e11183: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9330_e11185: f64 = (assign9330_e11183 + locals.var_tmf2);
        let assign9330_e11186: f64 = (assign9330_e11185).sqrt();
        (assign9330_e11186, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9330_e11186)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9330_e11186)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9330_e11188;
        locals.var_tmf2_dn0 = assign9330_e11188_d_n0;
        locals.var_tmf2_dn2 = assign9330_e11188_d_n2;

        let (assign9340_e11204, assign9340_e11204_d_n0, assign9340_e11204_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9340_e11200: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9340_e11201: f64 = (0.5 * assign9340_e11200);
        let assign9340_e11202: f64 = (locals.var_nfasti_i + assign9340_e11201);
        (assign9340_e11202, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign9340_e11204;
        locals.var_nj1_dn0 = assign9340_e11204_d_n0;
        locals.var_nj1_dn2 = assign9340_e11204_d_n2;

        let (assign9350_e11218, assign9350_e11218_d_n0, assign9350_e11218_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9350_e11214: f64 = (p.p85 - locals.var_nj0);
        let assign9350_e11216: f64 = (assign9350_e11214 - 0.01);
        (assign9350_e11216, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9350_e11218;
        locals.var_tmf1_dn0 = assign9350_e11218_d_n0;
        locals.var_tmf1_dn2 = assign9350_e11218_d_n2;

        let (assign9360_e11232, assign9360_e11232_d_n0, assign9360_e11232_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9360_e11228: f64 = (4.0 * p.p85);
        let assign9360_e11230: f64 = (assign9360_e11228 * 0.01);
        (assign9360_e11230, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9360_e11232;
        locals.var_tmf2_dn0 = assign9360_e11232_d_n0;
        locals.var_tmf2_dn2 = assign9360_e11232_d_n2;

        let (assign9370_e11248, assign9370_e11248_d_n0, assign9370_e11248_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let (assign9370_e11246, assign9370_e11246_d_n0, assign9370_e11246_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9370_e11245: f64 = (-locals.var_tmf2);
                (assign9370_e11245, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9370_e11246, assign9370_e11246_d_n0, assign9370_e11246_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9370_e11248;
        locals.var_tmf2_dn0 = assign9370_e11248_d_n0;
        locals.var_tmf2_dn2 = assign9370_e11248_d_n2;

        let (assign9380_e11263, assign9380_e11263_d_n0, assign9380_e11263_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9380_e11258: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9380_e11260: f64 = (assign9380_e11258 + locals.var_tmf2);
        let assign9380_e11261: f64 = (assign9380_e11260).sqrt();
        (assign9380_e11261, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9380_e11261)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9380_e11261)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9380_e11263;
        locals.var_tmf2_dn0 = assign9380_e11263_d_n0;
        locals.var_tmf2_dn2 = assign9380_e11263_d_n2;

        let (assign9390_e11279, assign9390_e11279_d_n0, assign9390_e11279_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9390_e11275: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9390_e11276: f64 = (0.5 * assign9390_e11275);
        let assign9390_e11277: f64 = (p.p85 - assign9390_e11276);
        (assign9390_e11277, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9390_e11279;
        locals.var_nj0_dn0 = assign9390_e11279_d_n0;
        locals.var_nj0_dn2 = assign9390_e11279_d_n2;

        let (assign9400_e11293, assign9400_e11293_d_n0, assign9400_e11293_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9400_e11289: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign9400_e11291: f64 = (assign9400_e11289 - 0.01);
        (assign9400_e11291, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9400_e11293;
        locals.var_tmf1_dn0 = assign9400_e11293_d_n0;
        locals.var_tmf1_dn2 = assign9400_e11293_d_n2;

        let (assign9410_e11307, assign9410_e11307_d_n0, assign9410_e11307_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9410_e11303: f64 = (4.0 * locals.var_nfasti_i);
        let assign9410_e11305: f64 = (assign9410_e11303 * 0.01);
        (assign9410_e11305, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9410_e11307;
        locals.var_tmf2_dn0 = assign9410_e11307_d_n0;
        locals.var_tmf2_dn2 = assign9410_e11307_d_n2;

        let (assign9420_e11323, assign9420_e11323_d_n0, assign9420_e11323_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let (assign9420_e11321, assign9420_e11321_d_n0, assign9420_e11321_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9420_e11320: f64 = (-locals.var_tmf2);
                (assign9420_e11320, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9420_e11321, assign9420_e11321_d_n0, assign9420_e11321_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9420_e11323;
        locals.var_tmf2_dn0 = assign9420_e11323_d_n0;
        locals.var_tmf2_dn2 = assign9420_e11323_d_n2;

        let (assign9430_e11338, assign9430_e11338_d_n0, assign9430_e11338_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9430_e11333: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9430_e11335: f64 = (assign9430_e11333 + locals.var_tmf2);
        let assign9430_e11336: f64 = (assign9430_e11335).sqrt();
        (assign9430_e11336, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9430_e11336)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9430_e11336)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9430_e11338;
        locals.var_tmf2_dn0 = assign9430_e11338_d_n0;
        locals.var_tmf2_dn2 = assign9430_e11338_d_n2;

        let (assign9440_e11354, assign9440_e11354_d_n0, assign9440_e11354_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign9440_e11350: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9440_e11351: f64 = (0.5 * assign9440_e11350);
        let assign9440_e11352: f64 = (locals.var_nfasti_i + assign9440_e11351);
        (assign9440_e11352, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9440_e11354;
        locals.var_nj0_dn0 = assign9440_e11354_d_n0;
        locals.var_nj0_dn2 = assign9440_e11354_d_n2;

        let (assign9450_e11365, assign9450_e11365_d_n0, assign9450_e11365_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9450_e11365;
        locals.var_nj0_dn0 = assign9450_e11365_d_n0;
        locals.var_nj0_dn2 = assign9450_e11365_d_n2;

        let (assign9460_e11376, assign9460_e11376_d_n0, assign9460_e11376_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard160 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign9460_e11376;
        locals.var_nj1_dn0 = assign9460_e11376_d_n0;
        locals.var_nj1_dn2 = assign9460_e11376_d_n2;

        let (assign9520_e11607,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) {
        let assign9520_e11603: f64 = (locals.var_nin * locals.var_nin);
        let assign9520_e11605: f64 = (assign9520_e11603 / locals.var_ndigat_i);
        (assign9520_e11605,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign9520_e11607;

        let (assign9530_e11622,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) {
        let assign9530_e11615: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign9530_e11618: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign9530_e11619: f64 = (assign9530_e11618).ln();
        let assign9530_e11620: f64 = (assign9530_e11615 * assign9530_e11619);
        (assign9530_e11620,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign9530_e11622;

        let assign9540_e11625: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign9540_e11625;

        let (assign9550_e11641, assign9550_e11641_d_n0, assign9550_e11641_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9550_e11636: f64 = (locals.var_v2 - locals.var_vha1);
        let assign9550_e11637: f64 = (p.p86 * assign9550_e11636);
        let assign9550_e11639: f64 = (assign9550_e11637 + locals.var_nfagat_i);
        (assign9550_e11639, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign9550_e11641;
        locals.var_nja10_dn0 = assign9550_e11641_d_n0;
        locals.var_nja10_dn2 = assign9550_e11641_d_n2;

        let (assign9560_e11655, assign9560_e11655_d_n0, assign9560_e11655_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9560_e11652: f64 = (p.p86 * locals.var_vha1);
        let assign9560_e11653: f64 = (locals.var_nfagat_i - assign9560_e11652);
        (assign9560_e11653, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9560_e11655;
        locals.var_nj0_dn0 = assign9560_e11655_d_n0;
        locals.var_nj0_dn2 = assign9560_e11655_d_n2;

        let (assign9570_e11669, assign9570_e11669_d_n0, assign9570_e11669_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9570_e11665: f64 = (p.p85 - locals.var_nja10);
        let assign9570_e11667: f64 = (assign9570_e11665 - 0.01);
        (assign9570_e11667, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9570_e11669;
        locals.var_tmf1_dn0 = assign9570_e11669_d_n0;
        locals.var_tmf1_dn2 = assign9570_e11669_d_n2;

        let (assign9580_e11683, assign9580_e11683_d_n0, assign9580_e11683_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9580_e11679: f64 = (4.0 * p.p85);
        let assign9580_e11681: f64 = (assign9580_e11679 * 0.01);
        (assign9580_e11681, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9580_e11683;
        locals.var_tmf2_dn0 = assign9580_e11683_d_n0;
        locals.var_tmf2_dn2 = assign9580_e11683_d_n2;

        let (assign9590_e11699, assign9590_e11699_d_n0, assign9590_e11699_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let (assign9590_e11697, assign9590_e11697_d_n0, assign9590_e11697_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9590_e11696: f64 = (-locals.var_tmf2);
                (assign9590_e11696, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9590_e11697, assign9590_e11697_d_n0, assign9590_e11697_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9590_e11699;
        locals.var_tmf2_dn0 = assign9590_e11699_d_n0;
        locals.var_tmf2_dn2 = assign9590_e11699_d_n2;

        let (assign9600_e11714, assign9600_e11714_d_n0, assign9600_e11714_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9600_e11709: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9600_e11711: f64 = (assign9600_e11709 + locals.var_tmf2);
        let assign9600_e11712: f64 = (assign9600_e11711).sqrt();
        (assign9600_e11712, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9600_e11712)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9600_e11712)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9600_e11714;
        locals.var_tmf2_dn0 = assign9600_e11714_d_n0;
        locals.var_tmf2_dn2 = assign9600_e11714_d_n2;

        let (assign9610_e11730, assign9610_e11730_d_n0, assign9610_e11730_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9610_e11726: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9610_e11727: f64 = (0.5 * assign9610_e11726);
        let assign9610_e11728: f64 = (p.p85 - assign9610_e11727);
        (assign9610_e11728, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign9610_e11730;
        locals.var_nja11_dn0 = assign9610_e11730_d_n0;
        locals.var_nja11_dn2 = assign9610_e11730_d_n2;

        let (assign9620_e11744, assign9620_e11744_d_n0, assign9620_e11744_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9620_e11740: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign9620_e11742: f64 = (assign9620_e11740 - 0.01);
        (assign9620_e11742, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9620_e11744;
        locals.var_tmf1_dn0 = assign9620_e11744_d_n0;
        locals.var_tmf1_dn2 = assign9620_e11744_d_n2;

        let (assign9630_e11758, assign9630_e11758_d_n0, assign9630_e11758_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9630_e11754: f64 = (4.0 * locals.var_nfagat_i);
        let assign9630_e11756: f64 = (assign9630_e11754 * 0.01);
        (assign9630_e11756, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9630_e11758;
        locals.var_tmf2_dn0 = assign9630_e11758_d_n0;
        locals.var_tmf2_dn2 = assign9630_e11758_d_n2;

        let (assign9640_e11774, assign9640_e11774_d_n0, assign9640_e11774_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let (assign9640_e11772, assign9640_e11772_d_n0, assign9640_e11772_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9640_e11771: f64 = (-locals.var_tmf2);
                (assign9640_e11771, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9640_e11772, assign9640_e11772_d_n0, assign9640_e11772_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9640_e11774;
        locals.var_tmf2_dn0 = assign9640_e11774_d_n0;
        locals.var_tmf2_dn2 = assign9640_e11774_d_n2;

        let (assign9650_e11789, assign9650_e11789_d_n0, assign9650_e11789_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9650_e11784: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9650_e11786: f64 = (assign9650_e11784 + locals.var_tmf2);
        let assign9650_e11787: f64 = (assign9650_e11786).sqrt();
        (assign9650_e11787, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9650_e11787)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9650_e11787)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9650_e11789;
        locals.var_tmf2_dn0 = assign9650_e11789_d_n0;
        locals.var_tmf2_dn2 = assign9650_e11789_d_n2;

        let (assign9660_e11805, assign9660_e11805_d_n0, assign9660_e11805_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9660_e11801: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9660_e11802: f64 = (0.5 * assign9660_e11801);
        let assign9660_e11803: f64 = (locals.var_nfagat_i + assign9660_e11802);
        (assign9660_e11803, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign9660_e11805;
        locals.var_nj1_dn0 = assign9660_e11805_d_n0;
        locals.var_nj1_dn2 = assign9660_e11805_d_n2;

        let (assign9670_e11819, assign9670_e11819_d_n0, assign9670_e11819_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9670_e11815: f64 = (p.p85 - locals.var_nj0);
        let assign9670_e11817: f64 = (assign9670_e11815 - 0.01);
        (assign9670_e11817, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9670_e11819;
        locals.var_tmf1_dn0 = assign9670_e11819_d_n0;
        locals.var_tmf1_dn2 = assign9670_e11819_d_n2;

        let (assign9680_e11833, assign9680_e11833_d_n0, assign9680_e11833_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9680_e11829: f64 = (4.0 * p.p85);
        let assign9680_e11831: f64 = (assign9680_e11829 * 0.01);
        (assign9680_e11831, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9680_e11833;
        locals.var_tmf2_dn0 = assign9680_e11833_d_n0;
        locals.var_tmf2_dn2 = assign9680_e11833_d_n2;

        let (assign9690_e11849, assign9690_e11849_d_n0, assign9690_e11849_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let (assign9690_e11847, assign9690_e11847_d_n0, assign9690_e11847_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9690_e11846: f64 = (-locals.var_tmf2);
                (assign9690_e11846, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9690_e11847, assign9690_e11847_d_n0, assign9690_e11847_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9690_e11849;
        locals.var_tmf2_dn0 = assign9690_e11849_d_n0;
        locals.var_tmf2_dn2 = assign9690_e11849_d_n2;

        let (assign9700_e11864, assign9700_e11864_d_n0, assign9700_e11864_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9700_e11859: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9700_e11861: f64 = (assign9700_e11859 + locals.var_tmf2);
        let assign9700_e11862: f64 = (assign9700_e11861).sqrt();
        (assign9700_e11862, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9700_e11862)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9700_e11862)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9700_e11864;
        locals.var_tmf2_dn0 = assign9700_e11864_d_n0;
        locals.var_tmf2_dn2 = assign9700_e11864_d_n2;

        let (assign9710_e11880, assign9710_e11880_d_n0, assign9710_e11880_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9710_e11876: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9710_e11877: f64 = (0.5 * assign9710_e11876);
        let assign9710_e11878: f64 = (p.p85 - assign9710_e11877);
        (assign9710_e11878, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9710_e11880;
        locals.var_nj0_dn0 = assign9710_e11880_d_n0;
        locals.var_nj0_dn2 = assign9710_e11880_d_n2;

        let (assign9720_e11894, assign9720_e11894_d_n0, assign9720_e11894_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9720_e11890: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign9720_e11892: f64 = (assign9720_e11890 - 0.01);
        (assign9720_e11892, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9720_e11894;
        locals.var_tmf1_dn0 = assign9720_e11894_d_n0;
        locals.var_tmf1_dn2 = assign9720_e11894_d_n2;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9730_e11908, assign9730_e11908_d_n0, assign9730_e11908_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9730_e11904: f64 = (4.0 * locals.var_nfagat_i);
        let assign9730_e11906: f64 = (assign9730_e11904 * 0.01);
        (assign9730_e11906, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9730_e11908;
        locals.var_tmf2_dn0 = assign9730_e11908_d_n0;
        locals.var_tmf2_dn2 = assign9730_e11908_d_n2;

        let (assign9740_e11924, assign9740_e11924_d_n0, assign9740_e11924_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let (assign9740_e11922, assign9740_e11922_d_n0, assign9740_e11922_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9740_e11921: f64 = (-locals.var_tmf2);
                (assign9740_e11921, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9740_e11922, assign9740_e11922_d_n0, assign9740_e11922_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9740_e11924;
        locals.var_tmf2_dn0 = assign9740_e11924_d_n0;
        locals.var_tmf2_dn2 = assign9740_e11924_d_n2;

        let (assign9750_e11939, assign9750_e11939_d_n0, assign9750_e11939_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9750_e11934: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9750_e11936: f64 = (assign9750_e11934 + locals.var_tmf2);
        let assign9750_e11937: f64 = (assign9750_e11936).sqrt();
        (assign9750_e11937, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9750_e11937)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9750_e11937)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9750_e11939;
        locals.var_tmf2_dn0 = assign9750_e11939_d_n0;
        locals.var_tmf2_dn2 = assign9750_e11939_d_n2;

        let (assign9760_e11955, assign9760_e11955_d_n0, assign9760_e11955_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign9760_e11951: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9760_e11952: f64 = (0.5 * assign9760_e11951);
        let assign9760_e11953: f64 = (locals.var_nfagat_i + assign9760_e11952);
        (assign9760_e11953, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9760_e11955;
        locals.var_nj0_dn0 = assign9760_e11955_d_n0;
        locals.var_nj0_dn2 = assign9760_e11955_d_n2;

        let (assign9770_e11966, assign9770_e11966_d_n0, assign9770_e11966_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9770_e11966;
        locals.var_nj0_dn0 = assign9770_e11966_d_n0;
        locals.var_nj0_dn2 = assign9770_e11966_d_n2;

        let (assign9780_e11977, assign9780_e11977_d_n0, assign9780_e11977_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard163 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign9780_e11977;
        locals.var_nj1_dn0 = assign9780_e11977_d_n0;
        locals.var_nj1_dn2 = assign9780_e11977_d_n2;

        let (assign9850_e12227,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign9850_e12223: f64 = (locals.var_nin * locals.var_nin);
        let assign9850_e12225: f64 = (assign9850_e12223 / locals.var_ndibot_i);
        (assign9850_e12225,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign9850_e12227;

        let (assign9860_e12243,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign9860_e12236: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign9860_e12239: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign9860_e12240: f64 = (assign9860_e12239).ln();
        let assign9860_e12241: f64 = (assign9860_e12236 * assign9860_e12240);
        (assign9860_e12241,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign9860_e12243;

        let assign9870_e12246: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign9870_e12246;

        let (assign9880_e12263, assign9880_e12263_d_n0, assign9880_e12263_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9880_e12258: f64 = (locals.var_vmax - locals.var_vha1);
        let assign9880_e12259: f64 = (p.p86 * assign9880_e12258);
        let assign9880_e12261: f64 = (assign9880_e12259 + locals.var_nfabot_i);
        (assign9880_e12261, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign9880_e12263;
        locals.var_nja10_dn0 = assign9880_e12263_d_n0;
        locals.var_nja10_dn2 = assign9880_e12263_d_n2;

        let (assign9890_e12278, assign9890_e12278_d_n0, assign9890_e12278_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9890_e12275: f64 = (p.p86 * locals.var_vha1);
        let assign9890_e12276: f64 = (locals.var_nfabot_i - assign9890_e12275);
        (assign9890_e12276, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign9890_e12278;
        locals.var_nj0_dn0 = assign9890_e12278_d_n0;
        locals.var_nj0_dn2 = assign9890_e12278_d_n2;

        let (assign9900_e12293, assign9900_e12293_d_n0, assign9900_e12293_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9900_e12289: f64 = (p.p85 - locals.var_nja10);
        let assign9900_e12291: f64 = (assign9900_e12289 - 0.01);
        (assign9900_e12291, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9900_e12293;
        locals.var_tmf1_dn0 = assign9900_e12293_d_n0;
        locals.var_tmf1_dn2 = assign9900_e12293_d_n2;

        let (assign9910_e12308, assign9910_e12308_d_n0, assign9910_e12308_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9910_e12304: f64 = (4.0 * p.p85);
        let assign9910_e12306: f64 = (assign9910_e12304 * 0.01);
        (assign9910_e12306, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9910_e12308;
        locals.var_tmf2_dn0 = assign9910_e12308_d_n0;
        locals.var_tmf2_dn2 = assign9910_e12308_d_n2;

        let (assign9920_e12325, assign9920_e12325_d_n0, assign9920_e12325_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let (assign9920_e12323, assign9920_e12323_d_n0, assign9920_e12323_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9920_e12322: f64 = (-locals.var_tmf2);
                (assign9920_e12322, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9920_e12323, assign9920_e12323_d_n0, assign9920_e12323_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9920_e12325;
        locals.var_tmf2_dn0 = assign9920_e12325_d_n0;
        locals.var_tmf2_dn2 = assign9920_e12325_d_n2;

        let (assign9930_e12341, assign9930_e12341_d_n0, assign9930_e12341_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9930_e12336: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9930_e12338: f64 = (assign9930_e12336 + locals.var_tmf2);
        let assign9930_e12339: f64 = (assign9930_e12338).sqrt();
        (assign9930_e12339, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9930_e12339)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9930_e12339)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9930_e12341;
        locals.var_tmf2_dn0 = assign9930_e12341_d_n0;
        locals.var_tmf2_dn2 = assign9930_e12341_d_n2;

        let (assign9940_e12358, assign9940_e12358_d_n0, assign9940_e12358_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9940_e12354: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign9940_e12355: f64 = (1.0 + assign9940_e12354);
        let assign9940_e12356: f64 = (0.5 * assign9940_e12355);
        (assign9940_e12356, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign9940_e12358;
        locals.var_dfn_su_dn0 = assign9940_e12358_d_n0;
        locals.var_dfn_su_dn2 = assign9940_e12358_d_n2;

        let (assign9950_e12375, assign9950_e12375_d_n0, assign9950_e12375_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9950_e12371: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9950_e12372: f64 = (0.5 * assign9950_e12371);
        let assign9950_e12373: f64 = (p.p85 - assign9950_e12372);
        (assign9950_e12373, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign9950_e12375;
        locals.var_nja11_dn0 = assign9950_e12375_d_n0;
        locals.var_nja11_dn2 = assign9950_e12375_d_n2;

        let (assign9960_e12390, assign9960_e12390_d_n0, assign9960_e12390_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9960_e12386: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign9960_e12388: f64 = (assign9960_e12386 - 0.01);
        (assign9960_e12388, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign9960_e12390;
        locals.var_tmf1_dn0 = assign9960_e12390_d_n0;
        locals.var_tmf1_dn2 = assign9960_e12390_d_n2;

        let (assign9970_e12405, assign9970_e12405_d_n0, assign9970_e12405_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9970_e12401: f64 = (4.0 * locals.var_nfabot_i);
        let assign9970_e12403: f64 = (assign9970_e12401 * 0.01);
        (assign9970_e12403, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9970_e12405;
        locals.var_tmf2_dn0 = assign9970_e12405_d_n0;
        locals.var_tmf2_dn2 = assign9970_e12405_d_n2;

        let (assign9980_e12422, assign9980_e12422_d_n0, assign9980_e12422_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let (assign9980_e12420, assign9980_e12420_d_n0, assign9980_e12420_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign9980_e12419: f64 = (-locals.var_tmf2);
                (assign9980_e12419, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign9980_e12420, assign9980_e12420_d_n0, assign9980_e12420_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9980_e12422;
        locals.var_tmf2_dn0 = assign9980_e12422_d_n0;
        locals.var_tmf2_dn2 = assign9980_e12422_d_n2;

        let (assign9990_e12438, assign9990_e12438_d_n0, assign9990_e12438_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign9990_e12433: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9990_e12435: f64 = (assign9990_e12433 + locals.var_tmf2);
        let assign9990_e12436: f64 = (assign9990_e12435).sqrt();
        (assign9990_e12436, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9990_e12436)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9990_e12436)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign9990_e12438;
        locals.var_tmf2_dn0 = assign9990_e12438_d_n0;
        locals.var_tmf2_dn2 = assign9990_e12438_d_n2;

        let (assign10000_e12455, assign10000_e12455_d_n0, assign10000_e12455_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10000_e12451: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign10000_e12452: f64 = (1.0 + assign10000_e12451);
        let assign10000_e12453: f64 = (0.5 * assign10000_e12452);
        (assign10000_e12453, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign10000_e12455;
        locals.var_dfn_sl_dn0 = assign10000_e12455_d_n0;
        locals.var_dfn_sl_dn2 = assign10000_e12455_d_n2;

        let (assign10010_e12472, assign10010_e12472_d_n0, assign10010_e12472_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10010_e12468: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10010_e12469: f64 = (0.5 * assign10010_e12468);
        let assign10010_e12470: f64 = (locals.var_nfabot_i + assign10010_e12469);
        (assign10010_e12470, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign10010_e12472;
        locals.var_nj1_dn0 = assign10010_e12472_d_n0;
        locals.var_nj1_dn2 = assign10010_e12472_d_n2;

        let (assign10020_e12487, assign10020_e12487_d_n0, assign10020_e12487_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10020_e12483: f64 = (p.p85 - locals.var_nj0);
        let assign10020_e12485: f64 = (assign10020_e12483 - 0.01);
        (assign10020_e12485, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10020_e12487;
        locals.var_tmf1_dn0 = assign10020_e12487_d_n0;
        locals.var_tmf1_dn2 = assign10020_e12487_d_n2;

        let (assign10030_e12502, assign10030_e12502_d_n0, assign10030_e12502_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10030_e12498: f64 = (4.0 * p.p85);
        let assign10030_e12500: f64 = (assign10030_e12498 * 0.01);
        (assign10030_e12500, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10030_e12502;
        locals.var_tmf2_dn0 = assign10030_e12502_d_n0;
        locals.var_tmf2_dn2 = assign10030_e12502_d_n2;

        let (assign10040_e12519, assign10040_e12519_d_n0, assign10040_e12519_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let (assign10040_e12517, assign10040_e12517_d_n0, assign10040_e12517_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10040_e12516: f64 = (-locals.var_tmf2);
                (assign10040_e12516, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10040_e12517, assign10040_e12517_d_n0, assign10040_e12517_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10040_e12519;
        locals.var_tmf2_dn0 = assign10040_e12519_d_n0;
        locals.var_tmf2_dn2 = assign10040_e12519_d_n2;

        let (assign10050_e12535, assign10050_e12535_d_n0, assign10050_e12535_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10050_e12530: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10050_e12532: f64 = (assign10050_e12530 + locals.var_tmf2);
        let assign10050_e12533: f64 = (assign10050_e12532).sqrt();
        (assign10050_e12533, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10050_e12533)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10050_e12533)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10050_e12535;
        locals.var_tmf2_dn0 = assign10050_e12535_d_n0;
        locals.var_tmf2_dn2 = assign10050_e12535_d_n2;

        let (assign10060_e12552, assign10060_e12552_d_n0, assign10060_e12552_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10060_e12548: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10060_e12549: f64 = (0.5 * assign10060_e12548);
        let assign10060_e12550: f64 = (p.p85 - assign10060_e12549);
        (assign10060_e12550, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10060_e12552;
        locals.var_nj0_dn0 = assign10060_e12552_d_n0;
        locals.var_nj0_dn2 = assign10060_e12552_d_n2;

        let (assign10070_e12567, assign10070_e12567_d_n0, assign10070_e12567_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10070_e12563: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign10070_e12565: f64 = (assign10070_e12563 - 0.01);
        (assign10070_e12565, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10070_e12567;
        locals.var_tmf1_dn0 = assign10070_e12567_d_n0;
        locals.var_tmf1_dn2 = assign10070_e12567_d_n2;

        let (assign10080_e12582, assign10080_e12582_d_n0, assign10080_e12582_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10080_e12578: f64 = (4.0 * locals.var_nfabot_i);
        let assign10080_e12580: f64 = (assign10080_e12578 * 0.01);
        (assign10080_e12580, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10080_e12582;
        locals.var_tmf2_dn0 = assign10080_e12582_d_n0;
        locals.var_tmf2_dn2 = assign10080_e12582_d_n2;

        let (assign10090_e12599, assign10090_e12599_d_n0, assign10090_e12599_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let (assign10090_e12597, assign10090_e12597_d_n0, assign10090_e12597_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10090_e12596: f64 = (-locals.var_tmf2);
                (assign10090_e12596, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10090_e12597, assign10090_e12597_d_n0, assign10090_e12597_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10090_e12599;
        locals.var_tmf2_dn0 = assign10090_e12599_d_n0;
        locals.var_tmf2_dn2 = assign10090_e12599_d_n2;

        let (assign10100_e12615, assign10100_e12615_d_n0, assign10100_e12615_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10100_e12610: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10100_e12612: f64 = (assign10100_e12610 + locals.var_tmf2);
        let assign10100_e12613: f64 = (assign10100_e12612).sqrt();
        (assign10100_e12613, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10100_e12613)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10100_e12613)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10100_e12615;
        locals.var_tmf2_dn0 = assign10100_e12615_d_n0;
        locals.var_tmf2_dn2 = assign10100_e12615_d_n2;

        let (assign10110_e12632, assign10110_e12632_d_n0, assign10110_e12632_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10110_e12628: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10110_e12629: f64 = (0.5 * assign10110_e12628);
        let assign10110_e12630: f64 = (locals.var_nfabot_i + assign10110_e12629);
        (assign10110_e12630, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10110_e12632;
        locals.var_nj0_dn0 = assign10110_e12632_d_n0;
        locals.var_nj0_dn2 = assign10110_e12632_d_n2;

        let (assign10120_e12647, assign10120_e12647_d_n0, assign10120_e12647_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign10120_e12643: f64 = (p.p86 * locals.var_dfn_su);
        let assign10120_e12645: f64 = (assign10120_e12643 * locals.var_dfn_sl);
        (assign10120_e12645, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign10120_e12643 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign10120_e12643 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign10120_e12647;
        locals.var_dnj1_dv_dn0 = assign10120_e12647_d_n0;
        locals.var_dnj1_dv_dn2 = assign10120_e12647_d_n2;

        let (assign10130_e12659, assign10130_e12659_d_n0, assign10130_e12659_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10130_e12659;
        locals.var_nj0_dn0 = assign10130_e12659_d_n0;
        locals.var_nj0_dn2 = assign10130_e12659_d_n2;

        let (assign10140_e12671, assign10140_e12671_d_n0, assign10140_e12671_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign10140_e12671;
        locals.var_nj1_dn0 = assign10140_e12671_d_n0;
        locals.var_nj1_dn2 = assign10140_e12671_d_n2;

        let (assign10150_e12683, assign10150_e12683_d_n0, assign10150_e12683_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard166 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign10150_e12683;
        locals.var_dnj1_dv_dn0 = assign10150_e12683_d_n0;
        locals.var_dnj1_dv_dn2 = assign10150_e12683_d_n2;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign10160_e12687: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10160_e12691: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10160_e12692: f64 = (locals.var_vha1 * assign10160_e12691);
        let assign10160_e12695: f64 = (locals.var_nj0 * p.p85);
        let assign10160_e12696: f64 = (assign10160_e12692 / assign10160_e12695);
        let assign10160_e12697: f64 = (assign10160_e12687 + assign10160_e12696);
        let assign10160_e12698: f64 = (locals.var_phitdinv * assign10160_e12697);
        let assign10160_e12699: f64 = (assign10160_e12698).abs();
        let assign10160_e12701: f64 = if assign10160_e12699 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign10160_e12701;

        let (assign10170_e12727, assign10170_e12727_d_n0, assign10170_e12727_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard167 != 0.0)) {
        let assign10170_e12713: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10170_e12717: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10170_e12718: f64 = (locals.var_vha1 * assign10170_e12717);
        let assign10170_e12721: f64 = (locals.var_nj0 * p.p85);
        let assign10170_e12722: f64 = (assign10170_e12718 / assign10170_e12721);
        let assign10170_e12723: f64 = (assign10170_e12713 + assign10170_e12722);
        let assign10170_e12724: f64 = (locals.var_phitdinv * assign10170_e12723);
        let assign10170_e12725: f64 = (assign10170_e12724).exp();
        (assign10170_e12725, (assign10170_e12725 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign10170_e12721) - (assign10170_e12718 * (locals.var_nj0_dn0 * p.p85))) / (assign10170_e12721 * assign10170_e12721))))), (assign10170_e12725 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign10170_e12721) - (assign10170_e12718 * (locals.var_nj0_dn2 * p.p85))) / (assign10170_e12721 * assign10170_e12721))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign10170_e12727;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign10170_e12727_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign10170_e12727_d_n2;

        let assign10180_e12731: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10180_e12735: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10180_e12736: f64 = (locals.var_vha1 * assign10180_e12735);
        let assign10180_e12739: f64 = (locals.var_nj0 * p.p85);
        let assign10180_e12740: f64 = (assign10180_e12736 / assign10180_e12739);
        let assign10180_e12741: f64 = (assign10180_e12731 + assign10180_e12740);
        let assign10180_e12742: f64 = (locals.var_phitdinv * assign10180_e12741);
        let assign10180_e12744: f64 = (-230.25850929940458);
        let assign10180_e12745: f64 = if assign10180_e12742 < assign10180_e12744 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign10180_e12745;

        let (assign10190_e12826, assign10190_e12826_d_n0, assign10190_e12826_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard167 == 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign10190_e12760: f64 = (-230.25850929940458);
        let assign10190_e12764: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10190_e12768: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10190_e12769: f64 = (locals.var_vha1 * assign10190_e12768);
        let assign10190_e12772: f64 = (locals.var_nj0 * p.p85);
        let assign10190_e12773: f64 = (assign10190_e12769 / assign10190_e12772);
        let assign10190_e12774: f64 = (assign10190_e12764 + assign10190_e12773);
        let assign10190_e12775: f64 = (locals.var_phitdinv * assign10190_e12774);
        let assign10190_e12776: f64 = (assign10190_e12760 - assign10190_e12775);
        let assign10190_e12780: f64 = (-230.25850929940458);
        let assign10190_e12784: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10190_e12788: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10190_e12789: f64 = (locals.var_vha1 * assign10190_e12788);
        let assign10190_e12792: f64 = (locals.var_nj0 * p.p85);
        let assign10190_e12793: f64 = (assign10190_e12789 / assign10190_e12792);
        let assign10190_e12794: f64 = (assign10190_e12784 + assign10190_e12793);
        let assign10190_e12795: f64 = (locals.var_phitdinv * assign10190_e12794);
        let assign10190_e12796: f64 = (assign10190_e12780 - assign10190_e12795);
        let assign10190_e12799: f64 = (-230.25850929940458);
        let assign10190_e12803: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10190_e12807: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10190_e12808: f64 = (locals.var_vha1 * assign10190_e12807);
        let assign10190_e12811: f64 = (locals.var_nj0 * p.p85);
        let assign10190_e12812: f64 = (assign10190_e12808 / assign10190_e12811);
        let assign10190_e12813: f64 = (assign10190_e12803 + assign10190_e12812);
        let assign10190_e12814: f64 = (locals.var_phitdinv * assign10190_e12813);
        let assign10190_e12815: f64 = (assign10190_e12799 - assign10190_e12814);
        let assign10190_e12817: f64 = (assign10190_e12815 * 0.3333333333333333);
        let assign10190_e12818: f64 = (1.0 + assign10190_e12817);
        let assign10190_e12819: f64 = (assign10190_e12796 * assign10190_e12818);
        let assign10190_e12820: f64 = (0.5 * assign10190_e12819);
        let assign10190_e12821: f64 = (1.0 + assign10190_e12820);
        let assign10190_e12822: f64 = (assign10190_e12776 * assign10190_e12821);
        let assign10190_e12823: f64 = (1.0 + assign10190_e12822);
        let assign10190_e12824: f64 = (1e-100 / assign10190_e12823);
        (assign10190_e12824, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign10190_e12772) - (assign10190_e12769 * (locals.var_nj0_dn0 * p.p85))) / (assign10190_e12772 * assign10190_e12772))))) * assign10190_e12821) + (assign10190_e12776 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign10190_e12792) - (assign10190_e12789 * (locals.var_nj0_dn0 * p.p85))) / (assign10190_e12792 * assign10190_e12792))))) * assign10190_e12818) + (assign10190_e12796 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign10190_e12811) - (assign10190_e12808 * (locals.var_nj0_dn0 * p.p85))) / (assign10190_e12811 * assign10190_e12811))))) * 0.3333333333333333))))))) / (assign10190_e12823 * assign10190_e12823))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign10190_e12772) - (assign10190_e12769 * (locals.var_nj0_dn2 * p.p85))) / (assign10190_e12772 * assign10190_e12772))))) * assign10190_e12821) + (assign10190_e12776 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign10190_e12792) - (assign10190_e12789 * (locals.var_nj0_dn2 * p.p85))) / (assign10190_e12792 * assign10190_e12792))))) * assign10190_e12818) + (assign10190_e12796 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign10190_e12811) - (assign10190_e12808 * (locals.var_nj0_dn2 * p.p85))) / (assign10190_e12811 * assign10190_e12811))))) * 0.3333333333333333))))))) / (assign10190_e12823 * assign10190_e12823))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign10190_e12826;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign10190_e12826_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign10190_e12826_d_n2;

        let (assign10200_e12905, assign10200_e12905_d_n0, assign10200_e12905_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard167 == 0.0)) && (locals.var_guard168 == 0.0)) {
        let assign10200_e12844: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10200_e12848: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10200_e12849: f64 = (locals.var_vha1 * assign10200_e12848);
        let assign10200_e12852: f64 = (locals.var_nj0 * p.p85);
        let assign10200_e12853: f64 = (assign10200_e12849 / assign10200_e12852);
        let assign10200_e12854: f64 = (assign10200_e12844 + assign10200_e12853);
        let assign10200_e12855: f64 = (locals.var_phitdinv * assign10200_e12854);
        let assign10200_e12857: f64 = (assign10200_e12855 - 230.25850929940458);
        let assign10200_e12863: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10200_e12867: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10200_e12868: f64 = (locals.var_vha1 * assign10200_e12867);
        let assign10200_e12871: f64 = (locals.var_nj0 * p.p85);
        let assign10200_e12872: f64 = (assign10200_e12868 / assign10200_e12871);
        let assign10200_e12873: f64 = (assign10200_e12863 + assign10200_e12872);
        let assign10200_e12874: f64 = (locals.var_phitdinv * assign10200_e12873);
        let assign10200_e12876: f64 = (assign10200_e12874 - 230.25850929940458);
        let assign10200_e12881: f64 = (locals.var_vmax / locals.var_nj1);
        let assign10200_e12885: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign10200_e12886: f64 = (locals.var_vha1 * assign10200_e12885);
        let assign10200_e12889: f64 = (locals.var_nj0 * p.p85);
        let assign10200_e12890: f64 = (assign10200_e12886 / assign10200_e12889);
        let assign10200_e12891: f64 = (assign10200_e12881 + assign10200_e12890);
        let assign10200_e12892: f64 = (locals.var_phitdinv * assign10200_e12891);
        let assign10200_e12894: f64 = (assign10200_e12892 - 230.25850929940458);
        let assign10200_e12896: f64 = (assign10200_e12894 * 0.3333333333333333);
        let assign10200_e12897: f64 = (1.0 + assign10200_e12896);
        let assign10200_e12898: f64 = (assign10200_e12876 * assign10200_e12897);
        let assign10200_e12899: f64 = (0.5 * assign10200_e12898);
        let assign10200_e12900: f64 = (1.0 + assign10200_e12899);
        let assign10200_e12901: f64 = (assign10200_e12857 * assign10200_e12900);
        let assign10200_e12902: f64 = (1.0 + assign10200_e12901);
        let assign10200_e12903: f64 = (1e100 * assign10200_e12902);
        (assign10200_e12903, (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign10200_e12852) - (assign10200_e12849 * (locals.var_nj0_dn0 * p.p85))) / (assign10200_e12852 * assign10200_e12852)))) * assign10200_e12900) + (assign10200_e12857 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign10200_e12871) - (assign10200_e12868 * (locals.var_nj0_dn0 * p.p85))) / (assign10200_e12871 * assign10200_e12871)))) * assign10200_e12897) + (assign10200_e12876 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign10200_e12889) - (assign10200_e12886 * (locals.var_nj0_dn0 * p.p85))) / (assign10200_e12889 * assign10200_e12889)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign10200_e12852) - (assign10200_e12849 * (locals.var_nj0_dn2 * p.p85))) / (assign10200_e12852 * assign10200_e12852)))) * assign10200_e12900) + (assign10200_e12857 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign10200_e12871) - (assign10200_e12868 * (locals.var_nj0_dn2 * p.p85))) / (assign10200_e12871 * assign10200_e12871)))) * assign10200_e12897) + (assign10200_e12876 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign10200_e12889) - (assign10200_e12886 * (locals.var_nj0_dn2 * p.p85))) / (assign10200_e12889 * assign10200_e12889)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign10200_e12905;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign10200_e12905_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign10200_e12905_d_n2;

        let (assign10210_e12932, assign10210_e12932_d_n0, assign10210_e12932_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign10210_e12916: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign10210_e12917: f64 = (locals.var_nj1 - assign10210_e12916);
        let assign10210_e12920: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign10210_e12921: f64 = (assign10210_e12917 / assign10210_e12920);
        let assign10210_e12924: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign10210_e12927: f64 = (locals.var_nj0 * p.p85);
        let assign10210_e12928: f64 = (assign10210_e12924 / assign10210_e12927);
        let assign10210_e12929: f64 = (assign10210_e12921 + assign10210_e12928);
        let assign10210_e12930: f64 = (locals.var_phitdinv * assign10210_e12929);
        (assign10210_e12930, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign10210_e12920) - (assign10210_e12917 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign10210_e12920 * assign10210_e12920)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign10210_e12927) - (assign10210_e12924 * (locals.var_nj0_dn0 * p.p85))) / (assign10210_e12927 * assign10210_e12927)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign10210_e12920) - (assign10210_e12917 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign10210_e12920 * assign10210_e12920)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign10210_e12927) - (assign10210_e12924 * (locals.var_nj0_dn2 * p.p85))) / (assign10210_e12927 * assign10210_e12927)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign10210_e12932;
        locals.var_dvmax_over_phitd_dv_dn0 = assign10210_e12932_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign10210_e12932_d_n2;

        let (assign10220_e12949, assign10220_e12949_d_n0, assign10220_e12949_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign10220_e12942: f64 = (locals.var_v2 - locals.var_vmax);
        let assign10220_e12944: f64 = (assign10220_e12942 * locals.var_dvmax_over_phitd_dv);
        let assign10220_e12945: f64 = (1.0 + assign10220_e12944);
        let assign10220_e12947: f64 = (assign10220_e12945 * locals.var_exp_vmax_over_phitd_bot);
        (assign10220_e12947, (((assign10220_e12942 * locals.var_dvmax_over_phitd_dv_dn0) * locals.var_exp_vmax_over_phitd_bot) + (assign10220_e12945 * locals.var_exp_vmax_over_phitd_bot_dn0)), (((assign10220_e12942 * locals.var_dvmax_over_phitd_dv_dn2) * locals.var_exp_vmax_over_phitd_bot) + (assign10220_e12945 * locals.var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign10220_e12949;
        locals.var_idmultbot_dn0 = assign10220_e12949_d_n0;
        locals.var_idmultbot_dn2 = assign10220_e12949_d_n2;

        let (assign10230_e12962,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign10230_e12958: f64 = (locals.var_nin * locals.var_nin);
        let assign10230_e12960: f64 = (assign10230_e12958 / locals.var_ndisti_i);
        (assign10230_e12960,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign10230_e12962;

        let (assign10240_e12978,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign10240_e12971: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign10240_e12974: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign10240_e12975: f64 = (assign10240_e12974).ln();
        let assign10240_e12976: f64 = (assign10240_e12971 * assign10240_e12975);
        (assign10240_e12976,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign10240_e12978;

        let assign10250_e12981: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign10250_e12981;

        let (assign10260_e12998, assign10260_e12998_d_n0, assign10260_e12998_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10260_e12993: f64 = (locals.var_vmax - locals.var_vha1);
        let assign10260_e12994: f64 = (p.p86 * assign10260_e12993);
        let assign10260_e12996: f64 = (assign10260_e12994 + locals.var_nfasti_i);
        (assign10260_e12996, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign10260_e12998;
        locals.var_nja10_dn0 = assign10260_e12998_d_n0;
        locals.var_nja10_dn2 = assign10260_e12998_d_n2;

        let (assign10270_e13013, assign10270_e13013_d_n0, assign10270_e13013_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10270_e13010: f64 = (p.p86 * locals.var_vha1);
        let assign10270_e13011: f64 = (locals.var_nfasti_i - assign10270_e13010);
        (assign10270_e13011, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10270_e13013;
        locals.var_nj0_dn0 = assign10270_e13013_d_n0;
        locals.var_nj0_dn2 = assign10270_e13013_d_n2;

        let (assign10280_e13028, assign10280_e13028_d_n0, assign10280_e13028_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10280_e13024: f64 = (p.p85 - locals.var_nja10);
        let assign10280_e13026: f64 = (assign10280_e13024 - 0.01);
        (assign10280_e13026, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10280_e13028;
        locals.var_tmf1_dn0 = assign10280_e13028_d_n0;
        locals.var_tmf1_dn2 = assign10280_e13028_d_n2;

        let (assign10290_e13043, assign10290_e13043_d_n0, assign10290_e13043_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10290_e13039: f64 = (4.0 * p.p85);
        let assign10290_e13041: f64 = (assign10290_e13039 * 0.01);
        (assign10290_e13041, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10290_e13043;
        locals.var_tmf2_dn0 = assign10290_e13043_d_n0;
        locals.var_tmf2_dn2 = assign10290_e13043_d_n2;

        let (assign10300_e13060, assign10300_e13060_d_n0, assign10300_e13060_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let (assign10300_e13058, assign10300_e13058_d_n0, assign10300_e13058_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10300_e13057: f64 = (-locals.var_tmf2);
                (assign10300_e13057, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10300_e13058, assign10300_e13058_d_n0, assign10300_e13058_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10300_e13060;
        locals.var_tmf2_dn0 = assign10300_e13060_d_n0;
        locals.var_tmf2_dn2 = assign10300_e13060_d_n2;

        let (assign10310_e13076, assign10310_e13076_d_n0, assign10310_e13076_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10310_e13071: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10310_e13073: f64 = (assign10310_e13071 + locals.var_tmf2);
        let assign10310_e13074: f64 = (assign10310_e13073).sqrt();
        (assign10310_e13074, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10310_e13074)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10310_e13074)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10310_e13076;
        locals.var_tmf2_dn0 = assign10310_e13076_d_n0;
        locals.var_tmf2_dn2 = assign10310_e13076_d_n2;

        let (assign10320_e13093, assign10320_e13093_d_n0, assign10320_e13093_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10320_e13089: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign10320_e13090: f64 = (1.0 + assign10320_e13089);
        let assign10320_e13091: f64 = (0.5 * assign10320_e13090);
        (assign10320_e13091, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign10320_e13093;
        locals.var_dfn_su_dn0 = assign10320_e13093_d_n0;
        locals.var_dfn_su_dn2 = assign10320_e13093_d_n2;

        let (assign10330_e13110, assign10330_e13110_d_n0, assign10330_e13110_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10330_e13106: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10330_e13107: f64 = (0.5 * assign10330_e13106);
        let assign10330_e13108: f64 = (p.p85 - assign10330_e13107);
        (assign10330_e13108, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign10330_e13110;
        locals.var_nja11_dn0 = assign10330_e13110_d_n0;
        locals.var_nja11_dn2 = assign10330_e13110_d_n2;

        let (assign10340_e13125, assign10340_e13125_d_n0, assign10340_e13125_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10340_e13121: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign10340_e13123: f64 = (assign10340_e13121 - 0.01);
        (assign10340_e13123, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10340_e13125;
        locals.var_tmf1_dn0 = assign10340_e13125_d_n0;
        locals.var_tmf1_dn2 = assign10340_e13125_d_n2;

        let (assign10350_e13140, assign10350_e13140_d_n0, assign10350_e13140_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10350_e13136: f64 = (4.0 * locals.var_nfasti_i);
        let assign10350_e13138: f64 = (assign10350_e13136 * 0.01);
        (assign10350_e13138, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10350_e13140;
        locals.var_tmf2_dn0 = assign10350_e13140_d_n0;
        locals.var_tmf2_dn2 = assign10350_e13140_d_n2;

        let (assign10360_e13157, assign10360_e13157_d_n0, assign10360_e13157_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let (assign10360_e13155, assign10360_e13155_d_n0, assign10360_e13155_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10360_e13154: f64 = (-locals.var_tmf2);
                (assign10360_e13154, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10360_e13155, assign10360_e13155_d_n0, assign10360_e13155_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10360_e13157;
        locals.var_tmf2_dn0 = assign10360_e13157_d_n0;
        locals.var_tmf2_dn2 = assign10360_e13157_d_n2;

        let (assign10370_e13173, assign10370_e13173_d_n0, assign10370_e13173_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10370_e13168: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10370_e13170: f64 = (assign10370_e13168 + locals.var_tmf2);
        let assign10370_e13171: f64 = (assign10370_e13170).sqrt();
        (assign10370_e13171, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10370_e13171)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10370_e13171)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10370_e13173;
        locals.var_tmf2_dn0 = assign10370_e13173_d_n0;
        locals.var_tmf2_dn2 = assign10370_e13173_d_n2;

        let (assign10380_e13190, assign10380_e13190_d_n0, assign10380_e13190_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10380_e13186: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign10380_e13187: f64 = (1.0 + assign10380_e13186);
        let assign10380_e13188: f64 = (0.5 * assign10380_e13187);
        (assign10380_e13188, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign10380_e13190;
        locals.var_dfn_sl_dn0 = assign10380_e13190_d_n0;
        locals.var_dfn_sl_dn2 = assign10380_e13190_d_n2;

        let (assign10390_e13207, assign10390_e13207_d_n0, assign10390_e13207_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10390_e13203: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10390_e13204: f64 = (0.5 * assign10390_e13203);
        let assign10390_e13205: f64 = (locals.var_nfasti_i + assign10390_e13204);
        (assign10390_e13205, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign10390_e13207;
        locals.var_nj1_dn0 = assign10390_e13207_d_n0;
        locals.var_nj1_dn2 = assign10390_e13207_d_n2;

        let (assign10400_e13222, assign10400_e13222_d_n0, assign10400_e13222_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10400_e13218: f64 = (p.p85 - locals.var_nj0);
        let assign10400_e13220: f64 = (assign10400_e13218 - 0.01);
        (assign10400_e13220, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10400_e13222;
        locals.var_tmf1_dn0 = assign10400_e13222_d_n0;
        locals.var_tmf1_dn2 = assign10400_e13222_d_n2;

        let (assign10410_e13237, assign10410_e13237_d_n0, assign10410_e13237_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10410_e13233: f64 = (4.0 * p.p85);
        let assign10410_e13235: f64 = (assign10410_e13233 * 0.01);
        (assign10410_e13235, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10410_e13237;
        locals.var_tmf2_dn0 = assign10410_e13237_d_n0;
        locals.var_tmf2_dn2 = assign10410_e13237_d_n2;

        let (assign10420_e13254, assign10420_e13254_d_n0, assign10420_e13254_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let (assign10420_e13252, assign10420_e13252_d_n0, assign10420_e13252_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10420_e13251: f64 = (-locals.var_tmf2);
                (assign10420_e13251, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10420_e13252, assign10420_e13252_d_n0, assign10420_e13252_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10420_e13254;
        locals.var_tmf2_dn0 = assign10420_e13254_d_n0;
        locals.var_tmf2_dn2 = assign10420_e13254_d_n2;

        let (assign10430_e13270, assign10430_e13270_d_n0, assign10430_e13270_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10430_e13265: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10430_e13267: f64 = (assign10430_e13265 + locals.var_tmf2);
        let assign10430_e13268: f64 = (assign10430_e13267).sqrt();
        (assign10430_e13268, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10430_e13268)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10430_e13268)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10430_e13270;
        locals.var_tmf2_dn0 = assign10430_e13270_d_n0;
        locals.var_tmf2_dn2 = assign10430_e13270_d_n2;

        let (assign10440_e13287, assign10440_e13287_d_n0, assign10440_e13287_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10440_e13283: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10440_e13284: f64 = (0.5 * assign10440_e13283);
        let assign10440_e13285: f64 = (p.p85 - assign10440_e13284);
        (assign10440_e13285, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10440_e13287;
        locals.var_nj0_dn0 = assign10440_e13287_d_n0;
        locals.var_nj0_dn2 = assign10440_e13287_d_n2;

        let (assign10450_e13302, assign10450_e13302_d_n0, assign10450_e13302_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10450_e13298: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign10450_e13300: f64 = (assign10450_e13298 - 0.01);
        (assign10450_e13300, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10450_e13302;
        locals.var_tmf1_dn0 = assign10450_e13302_d_n0;
        locals.var_tmf1_dn2 = assign10450_e13302_d_n2;

        let (assign10460_e13317, assign10460_e13317_d_n0, assign10460_e13317_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10460_e13313: f64 = (4.0 * locals.var_nfasti_i);
        let assign10460_e13315: f64 = (assign10460_e13313 * 0.01);
        (assign10460_e13315, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10460_e13317;
        locals.var_tmf2_dn0 = assign10460_e13317_d_n0;
        locals.var_tmf2_dn2 = assign10460_e13317_d_n2;

        let (assign10470_e13334, assign10470_e13334_d_n0, assign10470_e13334_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let (assign10470_e13332, assign10470_e13332_d_n0, assign10470_e13332_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10470_e13331: f64 = (-locals.var_tmf2);
                (assign10470_e13331, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10470_e13332, assign10470_e13332_d_n0, assign10470_e13332_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10470_e13334;
        locals.var_tmf2_dn0 = assign10470_e13334_d_n0;
        locals.var_tmf2_dn2 = assign10470_e13334_d_n2;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10480_e13350, assign10480_e13350_d_n0, assign10480_e13350_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10480_e13345: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10480_e13347: f64 = (assign10480_e13345 + locals.var_tmf2);
        let assign10480_e13348: f64 = (assign10480_e13347).sqrt();
        (assign10480_e13348, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10480_e13348)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10480_e13348)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10480_e13350;
        locals.var_tmf2_dn0 = assign10480_e13350_d_n0;
        locals.var_tmf2_dn2 = assign10480_e13350_d_n2;

        let (assign10490_e13367, assign10490_e13367_d_n0, assign10490_e13367_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10490_e13363: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10490_e13364: f64 = (0.5 * assign10490_e13363);
        let assign10490_e13365: f64 = (locals.var_nfasti_i + assign10490_e13364);
        (assign10490_e13365, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10490_e13367;
        locals.var_nj0_dn0 = assign10490_e13367_d_n0;
        locals.var_nj0_dn2 = assign10490_e13367_d_n2;

        let (assign10500_e13382, assign10500_e13382_d_n0, assign10500_e13382_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign10500_e13378: f64 = (p.p86 * locals.var_dfn_su);
        let assign10500_e13380: f64 = (assign10500_e13378 * locals.var_dfn_sl);
        (assign10500_e13380, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign10500_e13378 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign10500_e13378 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign10500_e13382;
        locals.var_dnj1_dv_dn0 = assign10500_e13382_d_n0;
        locals.var_dnj1_dv_dn2 = assign10500_e13382_d_n2;

        let (assign10510_e13394, assign10510_e13394_d_n0, assign10510_e13394_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10510_e13394;
        locals.var_nj0_dn0 = assign10510_e13394_d_n0;
        locals.var_nj0_dn2 = assign10510_e13394_d_n2;

        let (assign10520_e13406, assign10520_e13406_d_n0, assign10520_e13406_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign10520_e13406;
        locals.var_nj1_dn0 = assign10520_e13406_d_n0;
        locals.var_nj1_dn2 = assign10520_e13406_d_n2;

        let (assign10530_e13418, assign10530_e13418_d_n0, assign10530_e13418_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard169 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign10530_e13418;
        locals.var_dnj1_dv_dn0 = assign10530_e13418_d_n0;
        locals.var_dnj1_dv_dn2 = assign10530_e13418_d_n2;

        let (assign10590_e13667, assign10590_e13667_d_n0, assign10590_e13667_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign10590_e13651: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign10590_e13652: f64 = (locals.var_nj1 - assign10590_e13651);
        let assign10590_e13655: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign10590_e13656: f64 = (assign10590_e13652 / assign10590_e13655);
        let assign10590_e13659: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign10590_e13662: f64 = (locals.var_nj0 * p.p85);
        let assign10590_e13663: f64 = (assign10590_e13659 / assign10590_e13662);
        let assign10590_e13664: f64 = (assign10590_e13656 + assign10590_e13663);
        let assign10590_e13665: f64 = (locals.var_phitdinv * assign10590_e13664);
        (assign10590_e13665, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign10590_e13655) - (assign10590_e13652 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign10590_e13655 * assign10590_e13655)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign10590_e13662) - (assign10590_e13659 * (locals.var_nj0_dn0 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign10590_e13655) - (assign10590_e13652 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign10590_e13655 * assign10590_e13655)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign10590_e13662) - (assign10590_e13659 * (locals.var_nj0_dn2 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign10590_e13667;
        locals.var_dvmax_over_phitd_dv_dn0 = assign10590_e13667_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign10590_e13667_d_n2;

        let (assign10610_e13697,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign10610_e13693: f64 = (locals.var_nin * locals.var_nin);
        let assign10610_e13695: f64 = (assign10610_e13693 / locals.var_ndigat_i);
        (assign10610_e13695,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign10610_e13697;

        let (assign10620_e13713,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign10620_e13706: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign10620_e13709: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign10620_e13710: f64 = (assign10620_e13709).ln();
        let assign10620_e13711: f64 = (assign10620_e13706 * assign10620_e13710);
        (assign10620_e13711,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign10620_e13713;

        let assign10630_e13716: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard172 = assign10630_e13716;

        let (assign10640_e13733, assign10640_e13733_d_n0, assign10640_e13733_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10640_e13728: f64 = (locals.var_vmax - locals.var_vha1);
        let assign10640_e13729: f64 = (p.p86 * assign10640_e13728);
        let assign10640_e13731: f64 = (assign10640_e13729 + locals.var_nfagat_i);
        (assign10640_e13731, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign10640_e13733;
        locals.var_nja10_dn0 = assign10640_e13733_d_n0;
        locals.var_nja10_dn2 = assign10640_e13733_d_n2;

        let (assign10650_e13748, assign10650_e13748_d_n0, assign10650_e13748_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10650_e13745: f64 = (p.p86 * locals.var_vha1);
        let assign10650_e13746: f64 = (locals.var_nfagat_i - assign10650_e13745);
        (assign10650_e13746, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10650_e13748;
        locals.var_nj0_dn0 = assign10650_e13748_d_n0;
        locals.var_nj0_dn2 = assign10650_e13748_d_n2;

        let (assign10660_e13763, assign10660_e13763_d_n0, assign10660_e13763_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10660_e13759: f64 = (p.p85 - locals.var_nja10);
        let assign10660_e13761: f64 = (assign10660_e13759 - 0.01);
        (assign10660_e13761, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10660_e13763;
        locals.var_tmf1_dn0 = assign10660_e13763_d_n0;
        locals.var_tmf1_dn2 = assign10660_e13763_d_n2;

        let (assign10670_e13778, assign10670_e13778_d_n0, assign10670_e13778_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10670_e13774: f64 = (4.0 * p.p85);
        let assign10670_e13776: f64 = (assign10670_e13774 * 0.01);
        (assign10670_e13776, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10670_e13778;
        locals.var_tmf2_dn0 = assign10670_e13778_d_n0;
        locals.var_tmf2_dn2 = assign10670_e13778_d_n2;

        let (assign10680_e13795, assign10680_e13795_d_n0, assign10680_e13795_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let (assign10680_e13793, assign10680_e13793_d_n0, assign10680_e13793_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10680_e13792: f64 = (-locals.var_tmf2);
                (assign10680_e13792, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10680_e13793, assign10680_e13793_d_n0, assign10680_e13793_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10680_e13795;
        locals.var_tmf2_dn0 = assign10680_e13795_d_n0;
        locals.var_tmf2_dn2 = assign10680_e13795_d_n2;

        let (assign10690_e13811, assign10690_e13811_d_n0, assign10690_e13811_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10690_e13806: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10690_e13808: f64 = (assign10690_e13806 + locals.var_tmf2);
        let assign10690_e13809: f64 = (assign10690_e13808).sqrt();
        (assign10690_e13809, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10690_e13809)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10690_e13809)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10690_e13811;
        locals.var_tmf2_dn0 = assign10690_e13811_d_n0;
        locals.var_tmf2_dn2 = assign10690_e13811_d_n2;

        let (assign10700_e13828, assign10700_e13828_d_n0, assign10700_e13828_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10700_e13824: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign10700_e13825: f64 = (1.0 + assign10700_e13824);
        let assign10700_e13826: f64 = (0.5 * assign10700_e13825);
        (assign10700_e13826, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign10700_e13828;
        locals.var_dfn_su_dn0 = assign10700_e13828_d_n0;
        locals.var_dfn_su_dn2 = assign10700_e13828_d_n2;

        let (assign10710_e13845, assign10710_e13845_d_n0, assign10710_e13845_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10710_e13841: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10710_e13842: f64 = (0.5 * assign10710_e13841);
        let assign10710_e13843: f64 = (p.p85 - assign10710_e13842);
        (assign10710_e13843, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign10710_e13845;
        locals.var_nja11_dn0 = assign10710_e13845_d_n0;
        locals.var_nja11_dn2 = assign10710_e13845_d_n2;

        let (assign10720_e13860, assign10720_e13860_d_n0, assign10720_e13860_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10720_e13856: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign10720_e13858: f64 = (assign10720_e13856 - 0.01);
        (assign10720_e13858, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10720_e13860;
        locals.var_tmf1_dn0 = assign10720_e13860_d_n0;
        locals.var_tmf1_dn2 = assign10720_e13860_d_n2;

        let (assign10730_e13875, assign10730_e13875_d_n0, assign10730_e13875_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10730_e13871: f64 = (4.0 * locals.var_nfagat_i);
        let assign10730_e13873: f64 = (assign10730_e13871 * 0.01);
        (assign10730_e13873, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10730_e13875;
        locals.var_tmf2_dn0 = assign10730_e13875_d_n0;
        locals.var_tmf2_dn2 = assign10730_e13875_d_n2;

        let (assign10740_e13892, assign10740_e13892_d_n0, assign10740_e13892_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let (assign10740_e13890, assign10740_e13890_d_n0, assign10740_e13890_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10740_e13889: f64 = (-locals.var_tmf2);
                (assign10740_e13889, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10740_e13890, assign10740_e13890_d_n0, assign10740_e13890_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10740_e13892;
        locals.var_tmf2_dn0 = assign10740_e13892_d_n0;
        locals.var_tmf2_dn2 = assign10740_e13892_d_n2;

        let (assign10750_e13908, assign10750_e13908_d_n0, assign10750_e13908_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10750_e13903: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10750_e13905: f64 = (assign10750_e13903 + locals.var_tmf2);
        let assign10750_e13906: f64 = (assign10750_e13905).sqrt();
        (assign10750_e13906, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10750_e13906)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10750_e13906)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10750_e13908;
        locals.var_tmf2_dn0 = assign10750_e13908_d_n0;
        locals.var_tmf2_dn2 = assign10750_e13908_d_n2;

        let (assign10760_e13925, assign10760_e13925_d_n0, assign10760_e13925_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10760_e13921: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign10760_e13922: f64 = (1.0 + assign10760_e13921);
        let assign10760_e13923: f64 = (0.5 * assign10760_e13922);
        (assign10760_e13923, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign10760_e13925;
        locals.var_dfn_sl_dn0 = assign10760_e13925_d_n0;
        locals.var_dfn_sl_dn2 = assign10760_e13925_d_n2;

        let (assign10770_e13942, assign10770_e13942_d_n0, assign10770_e13942_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10770_e13938: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10770_e13939: f64 = (0.5 * assign10770_e13938);
        let assign10770_e13940: f64 = (locals.var_nfagat_i + assign10770_e13939);
        (assign10770_e13940, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign10770_e13942;
        locals.var_nj1_dn0 = assign10770_e13942_d_n0;
        locals.var_nj1_dn2 = assign10770_e13942_d_n2;

        let (assign10780_e13957, assign10780_e13957_d_n0, assign10780_e13957_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10780_e13953: f64 = (p.p85 - locals.var_nj0);
        let assign10780_e13955: f64 = (assign10780_e13953 - 0.01);
        (assign10780_e13955, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10780_e13957;
        locals.var_tmf1_dn0 = assign10780_e13957_d_n0;
        locals.var_tmf1_dn2 = assign10780_e13957_d_n2;

        let (assign10790_e13972, assign10790_e13972_d_n0, assign10790_e13972_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10790_e13968: f64 = (4.0 * p.p85);
        let assign10790_e13970: f64 = (assign10790_e13968 * 0.01);
        (assign10790_e13970, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10790_e13972;
        locals.var_tmf2_dn0 = assign10790_e13972_d_n0;
        locals.var_tmf2_dn2 = assign10790_e13972_d_n2;

        let (assign10800_e13989, assign10800_e13989_d_n0, assign10800_e13989_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let (assign10800_e13987, assign10800_e13987_d_n0, assign10800_e13987_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10800_e13986: f64 = (-locals.var_tmf2);
                (assign10800_e13986, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10800_e13987, assign10800_e13987_d_n0, assign10800_e13987_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10800_e13989;
        locals.var_tmf2_dn0 = assign10800_e13989_d_n0;
        locals.var_tmf2_dn2 = assign10800_e13989_d_n2;

        let (assign10810_e14005, assign10810_e14005_d_n0, assign10810_e14005_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10810_e14000: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10810_e14002: f64 = (assign10810_e14000 + locals.var_tmf2);
        let assign10810_e14003: f64 = (assign10810_e14002).sqrt();
        (assign10810_e14003, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10810_e14003)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10810_e14003)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10810_e14005;
        locals.var_tmf2_dn0 = assign10810_e14005_d_n0;
        locals.var_tmf2_dn2 = assign10810_e14005_d_n2;

        let (assign10820_e14022, assign10820_e14022_d_n0, assign10820_e14022_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10820_e14018: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10820_e14019: f64 = (0.5 * assign10820_e14018);
        let assign10820_e14020: f64 = (p.p85 - assign10820_e14019);
        (assign10820_e14020, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10820_e14022;
        locals.var_nj0_dn0 = assign10820_e14022_d_n0;
        locals.var_nj0_dn2 = assign10820_e14022_d_n2;

        let (assign10830_e14037, assign10830_e14037_d_n0, assign10830_e14037_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10830_e14033: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign10830_e14035: f64 = (assign10830_e14033 - 0.01);
        (assign10830_e14035, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign10830_e14037;
        locals.var_tmf1_dn0 = assign10830_e14037_d_n0;
        locals.var_tmf1_dn2 = assign10830_e14037_d_n2;

        let (assign10840_e14052, assign10840_e14052_d_n0, assign10840_e14052_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10840_e14048: f64 = (4.0 * locals.var_nfagat_i);
        let assign10840_e14050: f64 = (assign10840_e14048 * 0.01);
        (assign10840_e14050, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10840_e14052;
        locals.var_tmf2_dn0 = assign10840_e14052_d_n0;
        locals.var_tmf2_dn2 = assign10840_e14052_d_n2;

        let (assign10850_e14069, assign10850_e14069_d_n0, assign10850_e14069_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let (assign10850_e14067, assign10850_e14067_d_n0, assign10850_e14067_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign10850_e14066: f64 = (-locals.var_tmf2);
                (assign10850_e14066, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign10850_e14067, assign10850_e14067_d_n0, assign10850_e14067_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10850_e14069;
        locals.var_tmf2_dn0 = assign10850_e14069_d_n0;
        locals.var_tmf2_dn2 = assign10850_e14069_d_n2;

        let (assign10860_e14085, assign10860_e14085_d_n0, assign10860_e14085_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10860_e14080: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign10860_e14082: f64 = (assign10860_e14080 + locals.var_tmf2);
        let assign10860_e14083: f64 = (assign10860_e14082).sqrt();
        (assign10860_e14083, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign10860_e14083)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign10860_e14083)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign10860_e14085;
        locals.var_tmf2_dn0 = assign10860_e14085_d_n0;
        locals.var_tmf2_dn2 = assign10860_e14085_d_n2;

        let (assign10870_e14102, assign10870_e14102_d_n0, assign10870_e14102_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10870_e14098: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign10870_e14099: f64 = (0.5 * assign10870_e14098);
        let assign10870_e14100: f64 = (locals.var_nfagat_i + assign10870_e14099);
        (assign10870_e14100, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10870_e14102;
        locals.var_nj0_dn0 = assign10870_e14102_d_n0;
        locals.var_nj0_dn2 = assign10870_e14102_d_n2;

        let (assign10880_e14117, assign10880_e14117_d_n0, assign10880_e14117_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign10880_e14113: f64 = (p.p86 * locals.var_dfn_su);
        let assign10880_e14115: f64 = (assign10880_e14113 * locals.var_dfn_sl);
        (assign10880_e14115, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign10880_e14113 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign10880_e14113 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign10880_e14117;
        locals.var_dnj1_dv_dn0 = assign10880_e14117_d_n0;
        locals.var_dnj1_dv_dn2 = assign10880_e14117_d_n2;

        let (assign10890_e14129, assign10890_e14129_d_n0, assign10890_e14129_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign10890_e14129;
        locals.var_nj0_dn0 = assign10890_e14129_d_n0;
        locals.var_nj0_dn2 = assign10890_e14129_d_n2;

        let (assign10900_e14141, assign10900_e14141_d_n0, assign10900_e14141_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign10900_e14141;
        locals.var_nj1_dn0 = assign10900_e14141_d_n0;
        locals.var_nj1_dn2 = assign10900_e14141_d_n2;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10910_e14153, assign10910_e14153_d_n0, assign10910_e14153_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard172 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign10910_e14153;
        locals.var_dnj1_dv_dn0 = assign10910_e14153_d_n0;
        locals.var_dnj1_dv_dn2 = assign10910_e14153_d_n2;

        let (assign10970_e14402, assign10970_e14402_d_n0, assign10970_e14402_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign10970_e14386: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign10970_e14387: f64 = (locals.var_nj1 - assign10970_e14386);
        let assign10970_e14390: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign10970_e14391: f64 = (assign10970_e14387 / assign10970_e14390);
        let assign10970_e14394: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign10970_e14397: f64 = (locals.var_nj0 * p.p85);
        let assign10970_e14398: f64 = (assign10970_e14394 / assign10970_e14397);
        let assign10970_e14399: f64 = (assign10970_e14391 + assign10970_e14398);
        let assign10970_e14400: f64 = (locals.var_phitdinv * assign10970_e14399);
        (assign10970_e14400, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign10970_e14390) - (assign10970_e14387 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign10970_e14390 * assign10970_e14390)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign10970_e14397) - (assign10970_e14394 * (locals.var_nj0_dn0 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign10970_e14390) - (assign10970_e14387 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign10970_e14390 * assign10970_e14390)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign10970_e14397) - (assign10970_e14394 * (locals.var_nj0_dn2 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign10970_e14402;
        locals.var_dvmax_over_phitd_dv_dn0 = assign10970_e14402_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign10970_e14402_d_n2;

        let (assign10990_e14427, assign10990_e14427_d_n0, assign10990_e14427_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard153 != 0.0)) {
        let assign10990_e14425: f64 = (locals.var_idmultbot - 1.0);
        (assign10990_e14425, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign10990_e14427;
        locals.var_idmultbot_dn0 = assign10990_e14427_d_n0;
        locals.var_idmultbot_dn2 = assign10990_e14427_d_n2;

        let (assign11100_e14600, assign11100_e14600_d_n0, assign11100_e14600_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard153 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign11100_e14600;
        locals.var_idmultbot_dn0 = assign11100_e14600_d_n0;
        locals.var_idmultbot_dn2 = assign11100_e14600_d_n2;

        let assign13630_e18150: f64 = if (!(((locals.var_ab_i == 0.0) && (locals.var_ls_i == 0.0)) && (locals.var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard230 = assign13630_e18150;

        let assign13710_e18222: f64 = if locals.var_v3 < locals.var_vmax { 1.0 } else { 0.0 };
        locals.var_guard231 = assign13710_e18222;

        let (assign13770_e18363,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) {
        let assign13770_e18359: f64 = (locals.var_nin * locals.var_nin);
        let assign13770_e18361: f64 = (assign13770_e18359 / locals.var_ndibot_i);
        (assign13770_e18361,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign13770_e18363;

        let (assign13780_e18378,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) {
        let assign13780_e18371: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign13780_e18374: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign13780_e18375: f64 = (assign13780_e18374).ln();
        let assign13780_e18376: f64 = (assign13780_e18371 * assign13780_e18375);
        (assign13780_e18376,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign13780_e18378;

        let assign13790_e18381: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard234 = assign13790_e18381;

        let (assign13800_e18397, assign13800_e18397_d_n0, assign13800_e18397_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13800_e18392: f64 = (locals.var_v3 - locals.var_vha1);
        let assign13800_e18393: f64 = (p.p86 * assign13800_e18392);
        let assign13800_e18395: f64 = (assign13800_e18393 + locals.var_nfabot_i);
        (assign13800_e18395, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign13800_e18397;
        locals.var_nja10_dn0 = assign13800_e18397_d_n0;
        locals.var_nja10_dn2 = assign13800_e18397_d_n2;

        let (assign13810_e18411, assign13810_e18411_d_n0, assign13810_e18411_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13810_e18408: f64 = (p.p86 * locals.var_vha1);
        let assign13810_e18409: f64 = (locals.var_nfabot_i - assign13810_e18408);
        (assign13810_e18409, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign13810_e18411;
        locals.var_nj0_dn0 = assign13810_e18411_d_n0;
        locals.var_nj0_dn2 = assign13810_e18411_d_n2;

        let (assign13820_e18425, assign13820_e18425_d_n0, assign13820_e18425_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13820_e18421: f64 = (p.p85 - locals.var_nja10);
        let assign13820_e18423: f64 = (assign13820_e18421 - 0.01);
        (assign13820_e18423, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign13820_e18425;
        locals.var_tmf1_dn0 = assign13820_e18425_d_n0;
        locals.var_tmf1_dn2 = assign13820_e18425_d_n2;

        let (assign13830_e18439, assign13830_e18439_d_n0, assign13830_e18439_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13830_e18435: f64 = (4.0 * p.p85);
        let assign13830_e18437: f64 = (assign13830_e18435 * 0.01);
        (assign13830_e18437, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13830_e18439;
        locals.var_tmf2_dn0 = assign13830_e18439_d_n0;
        locals.var_tmf2_dn2 = assign13830_e18439_d_n2;

        let (assign13840_e18455, assign13840_e18455_d_n0, assign13840_e18455_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let (assign13840_e18453, assign13840_e18453_d_n0, assign13840_e18453_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign13840_e18452: f64 = (-locals.var_tmf2);
                (assign13840_e18452, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign13840_e18453, assign13840_e18453_d_n0, assign13840_e18453_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13840_e18455;
        locals.var_tmf2_dn0 = assign13840_e18455_d_n0;
        locals.var_tmf2_dn2 = assign13840_e18455_d_n2;

        let (assign13850_e18470, assign13850_e18470_d_n0, assign13850_e18470_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13850_e18465: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13850_e18467: f64 = (assign13850_e18465 + locals.var_tmf2);
        let assign13850_e18468: f64 = (assign13850_e18467).sqrt();
        (assign13850_e18468, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13850_e18468)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13850_e18468)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13850_e18470;
        locals.var_tmf2_dn0 = assign13850_e18470_d_n0;
        locals.var_tmf2_dn2 = assign13850_e18470_d_n2;

        let (assign13860_e18486, assign13860_e18486_d_n0, assign13860_e18486_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13860_e18482: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13860_e18483: f64 = (0.5 * assign13860_e18482);
        let assign13860_e18484: f64 = (p.p85 - assign13860_e18483);
        (assign13860_e18484, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign13860_e18486;
        locals.var_nja11_dn0 = assign13860_e18486_d_n0;
        locals.var_nja11_dn2 = assign13860_e18486_d_n2;

        let (assign13870_e18500, assign13870_e18500_d_n0, assign13870_e18500_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13870_e18496: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign13870_e18498: f64 = (assign13870_e18496 - 0.01);
        (assign13870_e18498, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign13870_e18500;
        locals.var_tmf1_dn0 = assign13870_e18500_d_n0;
        locals.var_tmf1_dn2 = assign13870_e18500_d_n2;

        let (assign13880_e18514, assign13880_e18514_d_n0, assign13880_e18514_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13880_e18510: f64 = (4.0 * locals.var_nfabot_i);
        let assign13880_e18512: f64 = (assign13880_e18510 * 0.01);
        (assign13880_e18512, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13880_e18514;
        locals.var_tmf2_dn0 = assign13880_e18514_d_n0;
        locals.var_tmf2_dn2 = assign13880_e18514_d_n2;

        let (assign13890_e18530, assign13890_e18530_d_n0, assign13890_e18530_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let (assign13890_e18528, assign13890_e18528_d_n0, assign13890_e18528_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign13890_e18527: f64 = (-locals.var_tmf2);
                (assign13890_e18527, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign13890_e18528, assign13890_e18528_d_n0, assign13890_e18528_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13890_e18530;
        locals.var_tmf2_dn0 = assign13890_e18530_d_n0;
        locals.var_tmf2_dn2 = assign13890_e18530_d_n2;

        let (assign13900_e18545, assign13900_e18545_d_n0, assign13900_e18545_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13900_e18540: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13900_e18542: f64 = (assign13900_e18540 + locals.var_tmf2);
        let assign13900_e18543: f64 = (assign13900_e18542).sqrt();
        (assign13900_e18543, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13900_e18543)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13900_e18543)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13900_e18545;
        locals.var_tmf2_dn0 = assign13900_e18545_d_n0;
        locals.var_tmf2_dn2 = assign13900_e18545_d_n2;

        let (assign13910_e18561, assign13910_e18561_d_n0, assign13910_e18561_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13910_e18557: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13910_e18558: f64 = (0.5 * assign13910_e18557);
        let assign13910_e18559: f64 = (locals.var_nfabot_i + assign13910_e18558);
        (assign13910_e18559, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign13910_e18561;
        locals.var_nj1_dn0 = assign13910_e18561_d_n0;
        locals.var_nj1_dn2 = assign13910_e18561_d_n2;

        let (assign13920_e18575, assign13920_e18575_d_n0, assign13920_e18575_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13920_e18571: f64 = (p.p85 - locals.var_nj0);
        let assign13920_e18573: f64 = (assign13920_e18571 - 0.01);
        (assign13920_e18573, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign13920_e18575;
        locals.var_tmf1_dn0 = assign13920_e18575_d_n0;
        locals.var_tmf1_dn2 = assign13920_e18575_d_n2;

        let (assign13930_e18589, assign13930_e18589_d_n0, assign13930_e18589_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13930_e18585: f64 = (4.0 * p.p85);
        let assign13930_e18587: f64 = (assign13930_e18585 * 0.01);
        (assign13930_e18587, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13930_e18589;
        locals.var_tmf2_dn0 = assign13930_e18589_d_n0;
        locals.var_tmf2_dn2 = assign13930_e18589_d_n2;

        let (assign13940_e18605, assign13940_e18605_d_n0, assign13940_e18605_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let (assign13940_e18603, assign13940_e18603_d_n0, assign13940_e18603_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign13940_e18602: f64 = (-locals.var_tmf2);
                (assign13940_e18602, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign13940_e18603, assign13940_e18603_d_n0, assign13940_e18603_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13940_e18605;
        locals.var_tmf2_dn0 = assign13940_e18605_d_n0;
        locals.var_tmf2_dn2 = assign13940_e18605_d_n2;

        let (assign13950_e18620, assign13950_e18620_d_n0, assign13950_e18620_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13950_e18615: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13950_e18617: f64 = (assign13950_e18615 + locals.var_tmf2);
        let assign13950_e18618: f64 = (assign13950_e18617).sqrt();
        (assign13950_e18618, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13950_e18618)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13950_e18618)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13950_e18620;
        locals.var_tmf2_dn0 = assign13950_e18620_d_n0;
        locals.var_tmf2_dn2 = assign13950_e18620_d_n2;

        let (assign13960_e18636, assign13960_e18636_d_n0, assign13960_e18636_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13960_e18632: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13960_e18633: f64 = (0.5 * assign13960_e18632);
        let assign13960_e18634: f64 = (p.p85 - assign13960_e18633);
        (assign13960_e18634, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign13960_e18636;
        locals.var_nj0_dn0 = assign13960_e18636_d_n0;
        locals.var_nj0_dn2 = assign13960_e18636_d_n2;

        let (assign13970_e18650, assign13970_e18650_d_n0, assign13970_e18650_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13970_e18646: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign13970_e18648: f64 = (assign13970_e18646 - 0.01);
        (assign13970_e18648, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign13970_e18650;
        locals.var_tmf1_dn0 = assign13970_e18650_d_n0;
        locals.var_tmf1_dn2 = assign13970_e18650_d_n2;

        let (assign13980_e18664, assign13980_e18664_d_n0, assign13980_e18664_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign13980_e18660: f64 = (4.0 * locals.var_nfabot_i);
        let assign13980_e18662: f64 = (assign13980_e18660 * 0.01);
        (assign13980_e18662, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13980_e18664;
        locals.var_tmf2_dn0 = assign13980_e18664_d_n0;
        locals.var_tmf2_dn2 = assign13980_e18664_d_n2;

        let (assign13990_e18680, assign13990_e18680_d_n0, assign13990_e18680_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let (assign13990_e18678, assign13990_e18678_d_n0, assign13990_e18678_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign13990_e18677: f64 = (-locals.var_tmf2);
                (assign13990_e18677, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign13990_e18678, assign13990_e18678_d_n0, assign13990_e18678_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign13990_e18680;
        locals.var_tmf2_dn0 = assign13990_e18680_d_n0;
        locals.var_tmf2_dn2 = assign13990_e18680_d_n2;

        let (assign14000_e18695, assign14000_e18695_d_n0, assign14000_e18695_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign14000_e18690: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14000_e18692: f64 = (assign14000_e18690 + locals.var_tmf2);
        let assign14000_e18693: f64 = (assign14000_e18692).sqrt();
        (assign14000_e18693, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14000_e18693)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14000_e18693)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14000_e18695;
        locals.var_tmf2_dn0 = assign14000_e18695_d_n0;
        locals.var_tmf2_dn2 = assign14000_e18695_d_n2;

        let (assign14010_e18711, assign14010_e18711_d_n0, assign14010_e18711_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        let assign14010_e18707: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14010_e18708: f64 = (0.5 * assign14010_e18707);
        let assign14010_e18709: f64 = (locals.var_nfabot_i + assign14010_e18708);
        (assign14010_e18709, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14010_e18711;
        locals.var_nj0_dn0 = assign14010_e18711_d_n0;
        locals.var_nj0_dn2 = assign14010_e18711_d_n2;

        let (assign14020_e18722, assign14020_e18722_d_n0, assign14020_e18722_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14020_e18722;
        locals.var_nj0_dn0 = assign14020_e18722_d_n0;
        locals.var_nj0_dn2 = assign14020_e18722_d_n2;

        let (assign14030_e18733, assign14030_e18733_d_n0, assign14030_e18733_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign14030_e18733;
        locals.var_nj1_dn0 = assign14030_e18733_d_n0;
        locals.var_nj1_dn2 = assign14030_e18733_d_n2;

        let assign14040_e18737: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14040_e18741: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14040_e18742: f64 = (locals.var_vha1 * assign14040_e18741);
        let assign14040_e18745: f64 = (locals.var_nj0 * p.p85);
        let assign14040_e18746: f64 = (assign14040_e18742 / assign14040_e18745);
        let assign14040_e18747: f64 = (assign14040_e18737 + assign14040_e18746);
        let assign14040_e18748: f64 = (locals.var_phitdinv * assign14040_e18747);
        let assign14040_e18749: f64 = (assign14040_e18748).abs();
        let assign14040_e18751: f64 = if assign14040_e18749 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign14040_e18751;

        let (assign14050_e18776, assign14050_e18776_d_n0, assign14050_e18776_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard235 != 0.0)) {
        let assign14050_e18762: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14050_e18766: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14050_e18767: f64 = (locals.var_vha1 * assign14050_e18766);
        let assign14050_e18770: f64 = (locals.var_nj0 * p.p85);
        let assign14050_e18771: f64 = (assign14050_e18767 / assign14050_e18770);
        let assign14050_e18772: f64 = (assign14050_e18762 + assign14050_e18771);
        let assign14050_e18773: f64 = (locals.var_phitdinv * assign14050_e18772);
        let assign14050_e18774: f64 = (assign14050_e18773).exp();
        (assign14050_e18774, (assign14050_e18774 * (locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign14050_e18770) - (assign14050_e18767 * (locals.var_nj0_dn0 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign14050_e18770) - (assign14050_e18767 * (locals.var_nj0_dn2 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign14050_e18776;
        locals.var_idmultbot_dn0 = assign14050_e18776_d_n0;
        locals.var_idmultbot_dn2 = assign14050_e18776_d_n2;

        let assign14060_e18780: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14060_e18784: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14060_e18785: f64 = (locals.var_vha1 * assign14060_e18784);
        let assign14060_e18788: f64 = (locals.var_nj0 * p.p85);
        let assign14060_e18789: f64 = (assign14060_e18785 / assign14060_e18788);
        let assign14060_e18790: f64 = (assign14060_e18780 + assign14060_e18789);
        let assign14060_e18791: f64 = (locals.var_phitdinv * assign14060_e18790);
        let assign14060_e18793: f64 = (-230.25850929940458);
        let assign14060_e18794: f64 = if assign14060_e18791 < assign14060_e18793 { 1.0 } else { 0.0 };
        locals.var_guard236 = assign14060_e18794;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14070_e18874, assign14070_e18874_d_n0, assign14070_e18874_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard235 == 0.0)) && (locals.var_guard236 != 0.0)) {
        let assign14070_e18808: f64 = (-230.25850929940458);
        let assign14070_e18812: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14070_e18816: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14070_e18817: f64 = (locals.var_vha1 * assign14070_e18816);
        let assign14070_e18820: f64 = (locals.var_nj0 * p.p85);
        let assign14070_e18821: f64 = (assign14070_e18817 / assign14070_e18820);
        let assign14070_e18822: f64 = (assign14070_e18812 + assign14070_e18821);
        let assign14070_e18823: f64 = (locals.var_phitdinv * assign14070_e18822);
        let assign14070_e18824: f64 = (assign14070_e18808 - assign14070_e18823);
        let assign14070_e18828: f64 = (-230.25850929940458);
        let assign14070_e18832: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14070_e18836: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14070_e18837: f64 = (locals.var_vha1 * assign14070_e18836);
        let assign14070_e18840: f64 = (locals.var_nj0 * p.p85);
        let assign14070_e18841: f64 = (assign14070_e18837 / assign14070_e18840);
        let assign14070_e18842: f64 = (assign14070_e18832 + assign14070_e18841);
        let assign14070_e18843: f64 = (locals.var_phitdinv * assign14070_e18842);
        let assign14070_e18844: f64 = (assign14070_e18828 - assign14070_e18843);
        let assign14070_e18847: f64 = (-230.25850929940458);
        let assign14070_e18851: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14070_e18855: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14070_e18856: f64 = (locals.var_vha1 * assign14070_e18855);
        let assign14070_e18859: f64 = (locals.var_nj0 * p.p85);
        let assign14070_e18860: f64 = (assign14070_e18856 / assign14070_e18859);
        let assign14070_e18861: f64 = (assign14070_e18851 + assign14070_e18860);
        let assign14070_e18862: f64 = (locals.var_phitdinv * assign14070_e18861);
        let assign14070_e18863: f64 = (assign14070_e18847 - assign14070_e18862);
        let assign14070_e18865: f64 = (assign14070_e18863 * 0.3333333333333333);
        let assign14070_e18866: f64 = (1.0 + assign14070_e18865);
        let assign14070_e18867: f64 = (assign14070_e18844 * assign14070_e18866);
        let assign14070_e18868: f64 = (0.5 * assign14070_e18867);
        let assign14070_e18869: f64 = (1.0 + assign14070_e18868);
        let assign14070_e18870: f64 = (assign14070_e18824 * assign14070_e18869);
        let assign14070_e18871: f64 = (1.0 + assign14070_e18870);
        let assign14070_e18872: f64 = (1e-100 / assign14070_e18871);
        (assign14070_e18872, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign14070_e18820) - (assign14070_e18817 * (locals.var_nj0_dn0 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign14070_e18840) - (assign14070_e18837 * (locals.var_nj0_dn0 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign14070_e18859) - (assign14070_e18856 * (locals.var_nj0_dn0 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign14070_e18820) - (assign14070_e18817 * (locals.var_nj0_dn2 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign14070_e18840) - (assign14070_e18837 * (locals.var_nj0_dn2 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign14070_e18859) - (assign14070_e18856 * (locals.var_nj0_dn2 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign14070_e18874;
        locals.var_idmultbot_dn0 = assign14070_e18874_d_n0;
        locals.var_idmultbot_dn2 = assign14070_e18874_d_n2;

        let (assign14080_e18952, assign14080_e18952_d_n0, assign14080_e18952_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard235 == 0.0)) && (locals.var_guard236 == 0.0)) {
        let assign14080_e18891: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14080_e18895: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14080_e18896: f64 = (locals.var_vha1 * assign14080_e18895);
        let assign14080_e18899: f64 = (locals.var_nj0 * p.p85);
        let assign14080_e18900: f64 = (assign14080_e18896 / assign14080_e18899);
        let assign14080_e18901: f64 = (assign14080_e18891 + assign14080_e18900);
        let assign14080_e18902: f64 = (locals.var_phitdinv * assign14080_e18901);
        let assign14080_e18904: f64 = (assign14080_e18902 - 230.25850929940458);
        let assign14080_e18910: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14080_e18914: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14080_e18915: f64 = (locals.var_vha1 * assign14080_e18914);
        let assign14080_e18918: f64 = (locals.var_nj0 * p.p85);
        let assign14080_e18919: f64 = (assign14080_e18915 / assign14080_e18918);
        let assign14080_e18920: f64 = (assign14080_e18910 + assign14080_e18919);
        let assign14080_e18921: f64 = (locals.var_phitdinv * assign14080_e18920);
        let assign14080_e18923: f64 = (assign14080_e18921 - 230.25850929940458);
        let assign14080_e18928: f64 = (locals.var_v3 / locals.var_nj1);
        let assign14080_e18932: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign14080_e18933: f64 = (locals.var_vha1 * assign14080_e18932);
        let assign14080_e18936: f64 = (locals.var_nj0 * p.p85);
        let assign14080_e18937: f64 = (assign14080_e18933 / assign14080_e18936);
        let assign14080_e18938: f64 = (assign14080_e18928 + assign14080_e18937);
        let assign14080_e18939: f64 = (locals.var_phitdinv * assign14080_e18938);
        let assign14080_e18941: f64 = (assign14080_e18939 - 230.25850929940458);
        let assign14080_e18943: f64 = (assign14080_e18941 * 0.3333333333333333);
        let assign14080_e18944: f64 = (1.0 + assign14080_e18943);
        let assign14080_e18945: f64 = (assign14080_e18923 * assign14080_e18944);
        let assign14080_e18946: f64 = (0.5 * assign14080_e18945);
        let assign14080_e18947: f64 = (1.0 + assign14080_e18946);
        let assign14080_e18948: f64 = (assign14080_e18904 * assign14080_e18947);
        let assign14080_e18949: f64 = (1.0 + assign14080_e18948);
        let assign14080_e18950: f64 = (1e100 * assign14080_e18949);
        (assign14080_e18950, (1e100 * (((locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign14080_e18899) - (assign14080_e18896 * (locals.var_nj0_dn0 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign14080_e18918) - (assign14080_e18915 * (locals.var_nj0_dn0 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign14080_e18936) - (assign14080_e18933 * (locals.var_nj0_dn0 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign14080_e18899) - (assign14080_e18896 * (locals.var_nj0_dn2 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign14080_e18918) - (assign14080_e18915 * (locals.var_nj0_dn2 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((locals.var_phitdinv * ((-((locals.var_v3 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign14080_e18936) - (assign14080_e18933 * (locals.var_nj0_dn2 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign14080_e18952;
        locals.var_idmultbot_dn0 = assign14080_e18952_d_n0;
        locals.var_idmultbot_dn2 = assign14080_e18952_d_n2;

        let (assign14090_e18964,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) {
        let assign14090_e18960: f64 = (locals.var_nin * locals.var_nin);
        let assign14090_e18962: f64 = (assign14090_e18960 / locals.var_ndisti_i);
        (assign14090_e18962,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign14090_e18964;

        let (assign14100_e18979,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) {
        let assign14100_e18972: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign14100_e18975: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign14100_e18976: f64 = (assign14100_e18975).ln();
        let assign14100_e18977: f64 = (assign14100_e18972 * assign14100_e18976);
        (assign14100_e18977,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign14100_e18979;

        let assign14110_e18982: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard237 = assign14110_e18982;

        let (assign14120_e18998, assign14120_e18998_d_n0, assign14120_e18998_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14120_e18993: f64 = (locals.var_v3 - locals.var_vha1);
        let assign14120_e18994: f64 = (p.p86 * assign14120_e18993);
        let assign14120_e18996: f64 = (assign14120_e18994 + locals.var_nfasti_i);
        (assign14120_e18996, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign14120_e18998;
        locals.var_nja10_dn0 = assign14120_e18998_d_n0;
        locals.var_nja10_dn2 = assign14120_e18998_d_n2;

        let (assign14130_e19012, assign14130_e19012_d_n0, assign14130_e19012_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14130_e19009: f64 = (p.p86 * locals.var_vha1);
        let assign14130_e19010: f64 = (locals.var_nfasti_i - assign14130_e19009);
        (assign14130_e19010, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14130_e19012;
        locals.var_nj0_dn0 = assign14130_e19012_d_n0;
        locals.var_nj0_dn2 = assign14130_e19012_d_n2;

        let (assign14140_e19026, assign14140_e19026_d_n0, assign14140_e19026_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14140_e19022: f64 = (p.p85 - locals.var_nja10);
        let assign14140_e19024: f64 = (assign14140_e19022 - 0.01);
        (assign14140_e19024, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14140_e19026;
        locals.var_tmf1_dn0 = assign14140_e19026_d_n0;
        locals.var_tmf1_dn2 = assign14140_e19026_d_n2;

        let (assign14150_e19040, assign14150_e19040_d_n0, assign14150_e19040_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14150_e19036: f64 = (4.0 * p.p85);
        let assign14150_e19038: f64 = (assign14150_e19036 * 0.01);
        (assign14150_e19038, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14150_e19040;
        locals.var_tmf2_dn0 = assign14150_e19040_d_n0;
        locals.var_tmf2_dn2 = assign14150_e19040_d_n2;

        let (assign14160_e19056, assign14160_e19056_d_n0, assign14160_e19056_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let (assign14160_e19054, assign14160_e19054_d_n0, assign14160_e19054_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14160_e19053: f64 = (-locals.var_tmf2);
                (assign14160_e19053, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14160_e19054, assign14160_e19054_d_n0, assign14160_e19054_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14160_e19056;
        locals.var_tmf2_dn0 = assign14160_e19056_d_n0;
        locals.var_tmf2_dn2 = assign14160_e19056_d_n2;

        let (assign14170_e19071, assign14170_e19071_d_n0, assign14170_e19071_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14170_e19066: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14170_e19068: f64 = (assign14170_e19066 + locals.var_tmf2);
        let assign14170_e19069: f64 = (assign14170_e19068).sqrt();
        (assign14170_e19069, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14170_e19069)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14170_e19069)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14170_e19071;
        locals.var_tmf2_dn0 = assign14170_e19071_d_n0;
        locals.var_tmf2_dn2 = assign14170_e19071_d_n2;

        let (assign14180_e19087, assign14180_e19087_d_n0, assign14180_e19087_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14180_e19083: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14180_e19084: f64 = (0.5 * assign14180_e19083);
        let assign14180_e19085: f64 = (p.p85 - assign14180_e19084);
        (assign14180_e19085, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign14180_e19087;
        locals.var_nja11_dn0 = assign14180_e19087_d_n0;
        locals.var_nja11_dn2 = assign14180_e19087_d_n2;

        let (assign14190_e19101, assign14190_e19101_d_n0, assign14190_e19101_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14190_e19097: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign14190_e19099: f64 = (assign14190_e19097 - 0.01);
        (assign14190_e19099, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14190_e19101;
        locals.var_tmf1_dn0 = assign14190_e19101_d_n0;
        locals.var_tmf1_dn2 = assign14190_e19101_d_n2;

        let (assign14200_e19115, assign14200_e19115_d_n0, assign14200_e19115_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14200_e19111: f64 = (4.0 * locals.var_nfasti_i);
        let assign14200_e19113: f64 = (assign14200_e19111 * 0.01);
        (assign14200_e19113, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14200_e19115;
        locals.var_tmf2_dn0 = assign14200_e19115_d_n0;
        locals.var_tmf2_dn2 = assign14200_e19115_d_n2;

        let (assign14210_e19131, assign14210_e19131_d_n0, assign14210_e19131_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let (assign14210_e19129, assign14210_e19129_d_n0, assign14210_e19129_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14210_e19128: f64 = (-locals.var_tmf2);
                (assign14210_e19128, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14210_e19129, assign14210_e19129_d_n0, assign14210_e19129_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14210_e19131;
        locals.var_tmf2_dn0 = assign14210_e19131_d_n0;
        locals.var_tmf2_dn2 = assign14210_e19131_d_n2;

        let (assign14220_e19146, assign14220_e19146_d_n0, assign14220_e19146_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14220_e19141: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14220_e19143: f64 = (assign14220_e19141 + locals.var_tmf2);
        let assign14220_e19144: f64 = (assign14220_e19143).sqrt();
        (assign14220_e19144, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14220_e19144)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14220_e19144)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14220_e19146;
        locals.var_tmf2_dn0 = assign14220_e19146_d_n0;
        locals.var_tmf2_dn2 = assign14220_e19146_d_n2;

        let (assign14230_e19162, assign14230_e19162_d_n0, assign14230_e19162_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14230_e19158: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14230_e19159: f64 = (0.5 * assign14230_e19158);
        let assign14230_e19160: f64 = (locals.var_nfasti_i + assign14230_e19159);
        (assign14230_e19160, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign14230_e19162;
        locals.var_nj1_dn0 = assign14230_e19162_d_n0;
        locals.var_nj1_dn2 = assign14230_e19162_d_n2;

        let (assign14240_e19176, assign14240_e19176_d_n0, assign14240_e19176_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14240_e19172: f64 = (p.p85 - locals.var_nj0);
        let assign14240_e19174: f64 = (assign14240_e19172 - 0.01);
        (assign14240_e19174, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14240_e19176;
        locals.var_tmf1_dn0 = assign14240_e19176_d_n0;
        locals.var_tmf1_dn2 = assign14240_e19176_d_n2;

        let (assign14250_e19190, assign14250_e19190_d_n0, assign14250_e19190_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14250_e19186: f64 = (4.0 * p.p85);
        let assign14250_e19188: f64 = (assign14250_e19186 * 0.01);
        (assign14250_e19188, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14250_e19190;
        locals.var_tmf2_dn0 = assign14250_e19190_d_n0;
        locals.var_tmf2_dn2 = assign14250_e19190_d_n2;

        let (assign14260_e19206, assign14260_e19206_d_n0, assign14260_e19206_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let (assign14260_e19204, assign14260_e19204_d_n0, assign14260_e19204_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14260_e19203: f64 = (-locals.var_tmf2);
                (assign14260_e19203, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14260_e19204, assign14260_e19204_d_n0, assign14260_e19204_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14260_e19206;
        locals.var_tmf2_dn0 = assign14260_e19206_d_n0;
        locals.var_tmf2_dn2 = assign14260_e19206_d_n2;

        let (assign14270_e19221, assign14270_e19221_d_n0, assign14270_e19221_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14270_e19216: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14270_e19218: f64 = (assign14270_e19216 + locals.var_tmf2);
        let assign14270_e19219: f64 = (assign14270_e19218).sqrt();
        (assign14270_e19219, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14270_e19219)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14270_e19219)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14270_e19221;
        locals.var_tmf2_dn0 = assign14270_e19221_d_n0;
        locals.var_tmf2_dn2 = assign14270_e19221_d_n2;

        let (assign14280_e19237, assign14280_e19237_d_n0, assign14280_e19237_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14280_e19233: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14280_e19234: f64 = (0.5 * assign14280_e19233);
        let assign14280_e19235: f64 = (p.p85 - assign14280_e19234);
        (assign14280_e19235, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14280_e19237;
        locals.var_nj0_dn0 = assign14280_e19237_d_n0;
        locals.var_nj0_dn2 = assign14280_e19237_d_n2;

        let (assign14290_e19251, assign14290_e19251_d_n0, assign14290_e19251_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14290_e19247: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign14290_e19249: f64 = (assign14290_e19247 - 0.01);
        (assign14290_e19249, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14290_e19251;
        locals.var_tmf1_dn0 = assign14290_e19251_d_n0;
        locals.var_tmf1_dn2 = assign14290_e19251_d_n2;

        let (assign14300_e19265, assign14300_e19265_d_n0, assign14300_e19265_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14300_e19261: f64 = (4.0 * locals.var_nfasti_i);
        let assign14300_e19263: f64 = (assign14300_e19261 * 0.01);
        (assign14300_e19263, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14300_e19265;
        locals.var_tmf2_dn0 = assign14300_e19265_d_n0;
        locals.var_tmf2_dn2 = assign14300_e19265_d_n2;

        let (assign14310_e19281, assign14310_e19281_d_n0, assign14310_e19281_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let (assign14310_e19279, assign14310_e19279_d_n0, assign14310_e19279_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14310_e19278: f64 = (-locals.var_tmf2);
                (assign14310_e19278, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14310_e19279, assign14310_e19279_d_n0, assign14310_e19279_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14310_e19281;
        locals.var_tmf2_dn0 = assign14310_e19281_d_n0;
        locals.var_tmf2_dn2 = assign14310_e19281_d_n2;

        let (assign14320_e19296, assign14320_e19296_d_n0, assign14320_e19296_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14320_e19291: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14320_e19293: f64 = (assign14320_e19291 + locals.var_tmf2);
        let assign14320_e19294: f64 = (assign14320_e19293).sqrt();
        (assign14320_e19294, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14320_e19294)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14320_e19294)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14320_e19296;
        locals.var_tmf2_dn0 = assign14320_e19296_d_n0;
        locals.var_tmf2_dn2 = assign14320_e19296_d_n2;

        let (assign14330_e19312, assign14330_e19312_d_n0, assign14330_e19312_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 != 0.0)) {
        let assign14330_e19308: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14330_e19309: f64 = (0.5 * assign14330_e19308);
        let assign14330_e19310: f64 = (locals.var_nfasti_i + assign14330_e19309);
        (assign14330_e19310, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14330_e19312;
        locals.var_nj0_dn0 = assign14330_e19312_d_n0;
        locals.var_nj0_dn2 = assign14330_e19312_d_n2;

        let (assign14340_e19323, assign14340_e19323_d_n0, assign14340_e19323_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14340_e19323;
        locals.var_nj0_dn0 = assign14340_e19323_d_n0;
        locals.var_nj0_dn2 = assign14340_e19323_d_n2;

        let (assign14350_e19334, assign14350_e19334_d_n0, assign14350_e19334_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard237 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign14350_e19334;
        locals.var_nj1_dn0 = assign14350_e19334_d_n0;
        locals.var_nj1_dn2 = assign14350_e19334_d_n2;

        let (assign14410_e19565,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) {
        let assign14410_e19561: f64 = (locals.var_nin * locals.var_nin);
        let assign14410_e19563: f64 = (assign14410_e19561 / locals.var_ndigat_i);
        (assign14410_e19563,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign14410_e19565;

        let (assign14420_e19580,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) {
        let assign14420_e19573: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign14420_e19576: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign14420_e19577: f64 = (assign14420_e19576).ln();
        let assign14420_e19578: f64 = (assign14420_e19573 * assign14420_e19577);
        (assign14420_e19578,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign14420_e19580;

        let assign14430_e19583: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign14430_e19583;

        let (assign14440_e19599, assign14440_e19599_d_n0, assign14440_e19599_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14440_e19594: f64 = (locals.var_v3 - locals.var_vha1);
        let assign14440_e19595: f64 = (p.p86 * assign14440_e19594);
        let assign14440_e19597: f64 = (assign14440_e19595 + locals.var_nfagat_i);
        (assign14440_e19597, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign14440_e19599;
        locals.var_nja10_dn0 = assign14440_e19599_d_n0;
        locals.var_nja10_dn2 = assign14440_e19599_d_n2;

        let (assign14450_e19613, assign14450_e19613_d_n0, assign14450_e19613_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14450_e19610: f64 = (p.p86 * locals.var_vha1);
        let assign14450_e19611: f64 = (locals.var_nfagat_i - assign14450_e19610);
        (assign14450_e19611, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14450_e19613;
        locals.var_nj0_dn0 = assign14450_e19613_d_n0;
        locals.var_nj0_dn2 = assign14450_e19613_d_n2;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14460_e19627, assign14460_e19627_d_n0, assign14460_e19627_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14460_e19623: f64 = (p.p85 - locals.var_nja10);
        let assign14460_e19625: f64 = (assign14460_e19623 - 0.01);
        (assign14460_e19625, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14460_e19627;
        locals.var_tmf1_dn0 = assign14460_e19627_d_n0;
        locals.var_tmf1_dn2 = assign14460_e19627_d_n2;

        let (assign14470_e19641, assign14470_e19641_d_n0, assign14470_e19641_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14470_e19637: f64 = (4.0 * p.p85);
        let assign14470_e19639: f64 = (assign14470_e19637 * 0.01);
        (assign14470_e19639, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14470_e19641;
        locals.var_tmf2_dn0 = assign14470_e19641_d_n0;
        locals.var_tmf2_dn2 = assign14470_e19641_d_n2;

        let (assign14480_e19657, assign14480_e19657_d_n0, assign14480_e19657_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let (assign14480_e19655, assign14480_e19655_d_n0, assign14480_e19655_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14480_e19654: f64 = (-locals.var_tmf2);
                (assign14480_e19654, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14480_e19655, assign14480_e19655_d_n0, assign14480_e19655_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14480_e19657;
        locals.var_tmf2_dn0 = assign14480_e19657_d_n0;
        locals.var_tmf2_dn2 = assign14480_e19657_d_n2;

        let (assign14490_e19672, assign14490_e19672_d_n0, assign14490_e19672_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14490_e19667: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14490_e19669: f64 = (assign14490_e19667 + locals.var_tmf2);
        let assign14490_e19670: f64 = (assign14490_e19669).sqrt();
        (assign14490_e19670, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14490_e19670)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14490_e19670)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14490_e19672;
        locals.var_tmf2_dn0 = assign14490_e19672_d_n0;
        locals.var_tmf2_dn2 = assign14490_e19672_d_n2;

        let (assign14500_e19688, assign14500_e19688_d_n0, assign14500_e19688_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14500_e19684: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14500_e19685: f64 = (0.5 * assign14500_e19684);
        let assign14500_e19686: f64 = (p.p85 - assign14500_e19685);
        (assign14500_e19686, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign14500_e19688;
        locals.var_nja11_dn0 = assign14500_e19688_d_n0;
        locals.var_nja11_dn2 = assign14500_e19688_d_n2;

        let (assign14510_e19702, assign14510_e19702_d_n0, assign14510_e19702_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14510_e19698: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign14510_e19700: f64 = (assign14510_e19698 - 0.01);
        (assign14510_e19700, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14510_e19702;
        locals.var_tmf1_dn0 = assign14510_e19702_d_n0;
        locals.var_tmf1_dn2 = assign14510_e19702_d_n2;

        let (assign14520_e19716, assign14520_e19716_d_n0, assign14520_e19716_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14520_e19712: f64 = (4.0 * locals.var_nfagat_i);
        let assign14520_e19714: f64 = (assign14520_e19712 * 0.01);
        (assign14520_e19714, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14520_e19716;
        locals.var_tmf2_dn0 = assign14520_e19716_d_n0;
        locals.var_tmf2_dn2 = assign14520_e19716_d_n2;

        let (assign14530_e19732, assign14530_e19732_d_n0, assign14530_e19732_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let (assign14530_e19730, assign14530_e19730_d_n0, assign14530_e19730_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14530_e19729: f64 = (-locals.var_tmf2);
                (assign14530_e19729, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14530_e19730, assign14530_e19730_d_n0, assign14530_e19730_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14530_e19732;
        locals.var_tmf2_dn0 = assign14530_e19732_d_n0;
        locals.var_tmf2_dn2 = assign14530_e19732_d_n2;

        let (assign14540_e19747, assign14540_e19747_d_n0, assign14540_e19747_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14540_e19742: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14540_e19744: f64 = (assign14540_e19742 + locals.var_tmf2);
        let assign14540_e19745: f64 = (assign14540_e19744).sqrt();
        (assign14540_e19745, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14540_e19745)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14540_e19745)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14540_e19747;
        locals.var_tmf2_dn0 = assign14540_e19747_d_n0;
        locals.var_tmf2_dn2 = assign14540_e19747_d_n2;

        let (assign14550_e19763, assign14550_e19763_d_n0, assign14550_e19763_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14550_e19759: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14550_e19760: f64 = (0.5 * assign14550_e19759);
        let assign14550_e19761: f64 = (locals.var_nfagat_i + assign14550_e19760);
        (assign14550_e19761, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign14550_e19763;
        locals.var_nj1_dn0 = assign14550_e19763_d_n0;
        locals.var_nj1_dn2 = assign14550_e19763_d_n2;

        let (assign14560_e19777, assign14560_e19777_d_n0, assign14560_e19777_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14560_e19773: f64 = (p.p85 - locals.var_nj0);
        let assign14560_e19775: f64 = (assign14560_e19773 - 0.01);
        (assign14560_e19775, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14560_e19777;
        locals.var_tmf1_dn0 = assign14560_e19777_d_n0;
        locals.var_tmf1_dn2 = assign14560_e19777_d_n2;

        let (assign14570_e19791, assign14570_e19791_d_n0, assign14570_e19791_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14570_e19787: f64 = (4.0 * p.p85);
        let assign14570_e19789: f64 = (assign14570_e19787 * 0.01);
        (assign14570_e19789, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14570_e19791;
        locals.var_tmf2_dn0 = assign14570_e19791_d_n0;
        locals.var_tmf2_dn2 = assign14570_e19791_d_n2;

        let (assign14580_e19807, assign14580_e19807_d_n0, assign14580_e19807_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let (assign14580_e19805, assign14580_e19805_d_n0, assign14580_e19805_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14580_e19804: f64 = (-locals.var_tmf2);
                (assign14580_e19804, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14580_e19805, assign14580_e19805_d_n0, assign14580_e19805_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14580_e19807;
        locals.var_tmf2_dn0 = assign14580_e19807_d_n0;
        locals.var_tmf2_dn2 = assign14580_e19807_d_n2;

        let (assign14590_e19822, assign14590_e19822_d_n0, assign14590_e19822_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14590_e19817: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14590_e19819: f64 = (assign14590_e19817 + locals.var_tmf2);
        let assign14590_e19820: f64 = (assign14590_e19819).sqrt();
        (assign14590_e19820, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14590_e19820)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14590_e19820)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14590_e19822;
        locals.var_tmf2_dn0 = assign14590_e19822_d_n0;
        locals.var_tmf2_dn2 = assign14590_e19822_d_n2;

        let (assign14600_e19838, assign14600_e19838_d_n0, assign14600_e19838_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14600_e19834: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14600_e19835: f64 = (0.5 * assign14600_e19834);
        let assign14600_e19836: f64 = (p.p85 - assign14600_e19835);
        (assign14600_e19836, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14600_e19838;
        locals.var_nj0_dn0 = assign14600_e19838_d_n0;
        locals.var_nj0_dn2 = assign14600_e19838_d_n2;

        let (assign14610_e19852, assign14610_e19852_d_n0, assign14610_e19852_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14610_e19848: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign14610_e19850: f64 = (assign14610_e19848 - 0.01);
        (assign14610_e19850, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14610_e19852;
        locals.var_tmf1_dn0 = assign14610_e19852_d_n0;
        locals.var_tmf1_dn2 = assign14610_e19852_d_n2;

        let (assign14620_e19866, assign14620_e19866_d_n0, assign14620_e19866_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14620_e19862: f64 = (4.0 * locals.var_nfagat_i);
        let assign14620_e19864: f64 = (assign14620_e19862 * 0.01);
        (assign14620_e19864, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14620_e19866;
        locals.var_tmf2_dn0 = assign14620_e19866_d_n0;
        locals.var_tmf2_dn2 = assign14620_e19866_d_n2;

        let (assign14630_e19882, assign14630_e19882_d_n0, assign14630_e19882_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let (assign14630_e19880, assign14630_e19880_d_n0, assign14630_e19880_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14630_e19879: f64 = (-locals.var_tmf2);
                (assign14630_e19879, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14630_e19880, assign14630_e19880_d_n0, assign14630_e19880_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14630_e19882;
        locals.var_tmf2_dn0 = assign14630_e19882_d_n0;
        locals.var_tmf2_dn2 = assign14630_e19882_d_n2;

        let (assign14640_e19897, assign14640_e19897_d_n0, assign14640_e19897_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14640_e19892: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14640_e19894: f64 = (assign14640_e19892 + locals.var_tmf2);
        let assign14640_e19895: f64 = (assign14640_e19894).sqrt();
        (assign14640_e19895, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14640_e19895)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14640_e19895)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14640_e19897;
        locals.var_tmf2_dn0 = assign14640_e19897_d_n0;
        locals.var_tmf2_dn2 = assign14640_e19897_d_n2;

        let (assign14650_e19913, assign14650_e19913_d_n0, assign14650_e19913_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 != 0.0)) {
        let assign14650_e19909: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14650_e19910: f64 = (0.5 * assign14650_e19909);
        let assign14650_e19911: f64 = (locals.var_nfagat_i + assign14650_e19910);
        (assign14650_e19911, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14650_e19913;
        locals.var_nj0_dn0 = assign14650_e19913_d_n0;
        locals.var_nj0_dn2 = assign14650_e19913_d_n2;

        let (assign14660_e19924, assign14660_e19924_d_n0, assign14660_e19924_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14660_e19924;
        locals.var_nj0_dn0 = assign14660_e19924_d_n0;
        locals.var_nj0_dn2 = assign14660_e19924_d_n2;

        let (assign14670_e19935, assign14670_e19935_d_n0, assign14670_e19935_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) && (locals.var_guard240 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign14670_e19935;
        locals.var_nj1_dn0 = assign14670_e19935_d_n0;
        locals.var_nj1_dn2 = assign14670_e19935_d_n2;

        let (assign14740_e20185,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign14740_e20181: f64 = (locals.var_nin * locals.var_nin);
        let assign14740_e20183: f64 = (assign14740_e20181 / locals.var_ndibot_i);
        (assign14740_e20183,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign14740_e20185;

        let (assign14750_e20201,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign14750_e20194: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign14750_e20197: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign14750_e20198: f64 = (assign14750_e20197).ln();
        let assign14750_e20199: f64 = (assign14750_e20194 * assign14750_e20198);
        (assign14750_e20199,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign14750_e20201;

        let assign14760_e20204: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign14760_e20204;

        let (assign14770_e20221, assign14770_e20221_d_n0, assign14770_e20221_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14770_e20216: f64 = (locals.var_vmax - locals.var_vha1);
        let assign14770_e20217: f64 = (p.p86 * assign14770_e20216);
        let assign14770_e20219: f64 = (assign14770_e20217 + locals.var_nfabot_i);
        (assign14770_e20219, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign14770_e20221;
        locals.var_nja10_dn0 = assign14770_e20221_d_n0;
        locals.var_nja10_dn2 = assign14770_e20221_d_n2;

        let (assign14780_e20236, assign14780_e20236_d_n0, assign14780_e20236_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14780_e20233: f64 = (p.p86 * locals.var_vha1);
        let assign14780_e20234: f64 = (locals.var_nfabot_i - assign14780_e20233);
        (assign14780_e20234, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14780_e20236;
        locals.var_nj0_dn0 = assign14780_e20236_d_n0;
        locals.var_nj0_dn2 = assign14780_e20236_d_n2;

        let (assign14790_e20251, assign14790_e20251_d_n0, assign14790_e20251_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14790_e20247: f64 = (p.p85 - locals.var_nja10);
        let assign14790_e20249: f64 = (assign14790_e20247 - 0.01);
        (assign14790_e20249, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14790_e20251;
        locals.var_tmf1_dn0 = assign14790_e20251_d_n0;
        locals.var_tmf1_dn2 = assign14790_e20251_d_n2;

        let (assign14800_e20266, assign14800_e20266_d_n0, assign14800_e20266_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14800_e20262: f64 = (4.0 * p.p85);
        let assign14800_e20264: f64 = (assign14800_e20262 * 0.01);
        (assign14800_e20264, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14800_e20266;
        locals.var_tmf2_dn0 = assign14800_e20266_d_n0;
        locals.var_tmf2_dn2 = assign14800_e20266_d_n2;

        let (assign14810_e20283, assign14810_e20283_d_n0, assign14810_e20283_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let (assign14810_e20281, assign14810_e20281_d_n0, assign14810_e20281_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14810_e20280: f64 = (-locals.var_tmf2);
                (assign14810_e20280, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14810_e20281, assign14810_e20281_d_n0, assign14810_e20281_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14810_e20283;
        locals.var_tmf2_dn0 = assign14810_e20283_d_n0;
        locals.var_tmf2_dn2 = assign14810_e20283_d_n2;

        let (assign14820_e20299, assign14820_e20299_d_n0, assign14820_e20299_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14820_e20294: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14820_e20296: f64 = (assign14820_e20294 + locals.var_tmf2);
        let assign14820_e20297: f64 = (assign14820_e20296).sqrt();
        (assign14820_e20297, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14820_e20297)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14820_e20297)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14820_e20299;
        locals.var_tmf2_dn0 = assign14820_e20299_d_n0;
        locals.var_tmf2_dn2 = assign14820_e20299_d_n2;

        let (assign14830_e20316, assign14830_e20316_d_n0, assign14830_e20316_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14830_e20312: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14830_e20313: f64 = (1.0 + assign14830_e20312);
        let assign14830_e20314: f64 = (0.5 * assign14830_e20313);
        (assign14830_e20314, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign14830_e20316;
        locals.var_dfn_su_dn0 = assign14830_e20316_d_n0;
        locals.var_dfn_su_dn2 = assign14830_e20316_d_n2;

        let (assign14840_e20333, assign14840_e20333_d_n0, assign14840_e20333_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14840_e20329: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14840_e20330: f64 = (0.5 * assign14840_e20329);
        let assign14840_e20331: f64 = (p.p85 - assign14840_e20330);
        (assign14840_e20331, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign14840_e20333;
        locals.var_nja11_dn0 = assign14840_e20333_d_n0;
        locals.var_nja11_dn2 = assign14840_e20333_d_n2;

        let (assign14850_e20348, assign14850_e20348_d_n0, assign14850_e20348_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14850_e20344: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign14850_e20346: f64 = (assign14850_e20344 - 0.01);
        (assign14850_e20346, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14850_e20348;
        locals.var_tmf1_dn0 = assign14850_e20348_d_n0;
        locals.var_tmf1_dn2 = assign14850_e20348_d_n2;

        let (assign14860_e20363, assign14860_e20363_d_n0, assign14860_e20363_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14860_e20359: f64 = (4.0 * locals.var_nfabot_i);
        let assign14860_e20361: f64 = (assign14860_e20359 * 0.01);
        (assign14860_e20361, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14860_e20363;
        locals.var_tmf2_dn0 = assign14860_e20363_d_n0;
        locals.var_tmf2_dn2 = assign14860_e20363_d_n2;

        let (assign14870_e20380, assign14870_e20380_d_n0, assign14870_e20380_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let (assign14870_e20378, assign14870_e20378_d_n0, assign14870_e20378_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14870_e20377: f64 = (-locals.var_tmf2);
                (assign14870_e20377, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14870_e20378, assign14870_e20378_d_n0, assign14870_e20378_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14870_e20380;
        locals.var_tmf2_dn0 = assign14870_e20380_d_n0;
        locals.var_tmf2_dn2 = assign14870_e20380_d_n2;

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14880_e20396, assign14880_e20396_d_n0, assign14880_e20396_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14880_e20391: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14880_e20393: f64 = (assign14880_e20391 + locals.var_tmf2);
        let assign14880_e20394: f64 = (assign14880_e20393).sqrt();
        (assign14880_e20394, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14880_e20394)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14880_e20394)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14880_e20396;
        locals.var_tmf2_dn0 = assign14880_e20396_d_n0;
        locals.var_tmf2_dn2 = assign14880_e20396_d_n2;

        let (assign14890_e20413, assign14890_e20413_d_n0, assign14890_e20413_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14890_e20409: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14890_e20410: f64 = (1.0 + assign14890_e20409);
        let assign14890_e20411: f64 = (0.5 * assign14890_e20410);
        (assign14890_e20411, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign14890_e20413;
        locals.var_dfn_sl_dn0 = assign14890_e20413_d_n0;
        locals.var_dfn_sl_dn2 = assign14890_e20413_d_n2;

        let (assign14900_e20430, assign14900_e20430_d_n0, assign14900_e20430_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14900_e20426: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14900_e20427: f64 = (0.5 * assign14900_e20426);
        let assign14900_e20428: f64 = (locals.var_nfabot_i + assign14900_e20427);
        (assign14900_e20428, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign14900_e20430;
        locals.var_nj1_dn0 = assign14900_e20430_d_n0;
        locals.var_nj1_dn2 = assign14900_e20430_d_n2;

        let (assign14910_e20445, assign14910_e20445_d_n0, assign14910_e20445_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14910_e20441: f64 = (p.p85 - locals.var_nj0);
        let assign14910_e20443: f64 = (assign14910_e20441 - 0.01);
        (assign14910_e20443, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14910_e20445;
        locals.var_tmf1_dn0 = assign14910_e20445_d_n0;
        locals.var_tmf1_dn2 = assign14910_e20445_d_n2;

        let (assign14920_e20460, assign14920_e20460_d_n0, assign14920_e20460_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14920_e20456: f64 = (4.0 * p.p85);
        let assign14920_e20458: f64 = (assign14920_e20456 * 0.01);
        (assign14920_e20458, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14920_e20460;
        locals.var_tmf2_dn0 = assign14920_e20460_d_n0;
        locals.var_tmf2_dn2 = assign14920_e20460_d_n2;

        let (assign14930_e20477, assign14930_e20477_d_n0, assign14930_e20477_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let (assign14930_e20475, assign14930_e20475_d_n0, assign14930_e20475_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14930_e20474: f64 = (-locals.var_tmf2);
                (assign14930_e20474, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14930_e20475, assign14930_e20475_d_n0, assign14930_e20475_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14930_e20477;
        locals.var_tmf2_dn0 = assign14930_e20477_d_n0;
        locals.var_tmf2_dn2 = assign14930_e20477_d_n2;

        let (assign14940_e20493, assign14940_e20493_d_n0, assign14940_e20493_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14940_e20488: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14940_e20490: f64 = (assign14940_e20488 + locals.var_tmf2);
        let assign14940_e20491: f64 = (assign14940_e20490).sqrt();
        (assign14940_e20491, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14940_e20491)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14940_e20491)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14940_e20493;
        locals.var_tmf2_dn0 = assign14940_e20493_d_n0;
        locals.var_tmf2_dn2 = assign14940_e20493_d_n2;

        let (assign14950_e20510, assign14950_e20510_d_n0, assign14950_e20510_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14950_e20506: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14950_e20507: f64 = (0.5 * assign14950_e20506);
        let assign14950_e20508: f64 = (p.p85 - assign14950_e20507);
        (assign14950_e20508, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign14950_e20510;
        locals.var_nj0_dn0 = assign14950_e20510_d_n0;
        locals.var_nj0_dn2 = assign14950_e20510_d_n2;

        let (assign14960_e20525, assign14960_e20525_d_n0, assign14960_e20525_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14960_e20521: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign14960_e20523: f64 = (assign14960_e20521 - 0.01);
        (assign14960_e20523, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign14960_e20525;
        locals.var_tmf1_dn0 = assign14960_e20525_d_n0;
        locals.var_tmf1_dn2 = assign14960_e20525_d_n2;

        let (assign14970_e20540, assign14970_e20540_d_n0, assign14970_e20540_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14970_e20536: f64 = (4.0 * locals.var_nfabot_i);
        let assign14970_e20538: f64 = (assign14970_e20536 * 0.01);
        (assign14970_e20538, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14970_e20540;
        locals.var_tmf2_dn0 = assign14970_e20540_d_n0;
        locals.var_tmf2_dn2 = assign14970_e20540_d_n2;

        let (assign14980_e20557, assign14980_e20557_d_n0, assign14980_e20557_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let (assign14980_e20555, assign14980_e20555_d_n0, assign14980_e20555_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign14980_e20554: f64 = (-locals.var_tmf2);
                (assign14980_e20554, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign14980_e20555, assign14980_e20555_d_n0, assign14980_e20555_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14980_e20557;
        locals.var_tmf2_dn0 = assign14980_e20557_d_n0;
        locals.var_tmf2_dn2 = assign14980_e20557_d_n2;

        let (assign14990_e20573, assign14990_e20573_d_n0, assign14990_e20573_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign14990_e20568: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14990_e20570: f64 = (assign14990_e20568 + locals.var_tmf2);
        let assign14990_e20571: f64 = (assign14990_e20570).sqrt();
        (assign14990_e20571, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14990_e20571)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14990_e20571)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign14990_e20573;
        locals.var_tmf2_dn0 = assign14990_e20573_d_n0;
        locals.var_tmf2_dn2 = assign14990_e20573_d_n2;

        let (assign15000_e20590, assign15000_e20590_d_n0, assign15000_e20590_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign15000_e20586: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15000_e20587: f64 = (0.5 * assign15000_e20586);
        let assign15000_e20588: f64 = (locals.var_nfabot_i + assign15000_e20587);
        (assign15000_e20588, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15000_e20590;
        locals.var_nj0_dn0 = assign15000_e20590_d_n0;
        locals.var_nj0_dn2 = assign15000_e20590_d_n2;

        let (assign15010_e20605, assign15010_e20605_d_n0, assign15010_e20605_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign15010_e20601: f64 = (p.p86 * locals.var_dfn_su);
        let assign15010_e20603: f64 = (assign15010_e20601 * locals.var_dfn_sl);
        (assign15010_e20603, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign15010_e20601 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign15010_e20601 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign15010_e20605;
        locals.var_dnj1_dv_dn0 = assign15010_e20605_d_n0;
        locals.var_dnj1_dv_dn2 = assign15010_e20605_d_n2;

        let (assign15020_e20617, assign15020_e20617_d_n0, assign15020_e20617_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15020_e20617;
        locals.var_nj0_dn0 = assign15020_e20617_d_n0;
        locals.var_nj0_dn2 = assign15020_e20617_d_n2;

        let (assign15030_e20629, assign15030_e20629_d_n0, assign15030_e20629_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign15030_e20629;
        locals.var_nj1_dn0 = assign15030_e20629_d_n0;
        locals.var_nj1_dn2 = assign15030_e20629_d_n2;

        let (assign15040_e20641, assign15040_e20641_d_n0, assign15040_e20641_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard243 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign15040_e20641;
        locals.var_dnj1_dv_dn0 = assign15040_e20641_d_n0;
        locals.var_dnj1_dv_dn2 = assign15040_e20641_d_n2;

        let assign15050_e20645: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15050_e20649: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15050_e20650: f64 = (locals.var_vha1 * assign15050_e20649);
        let assign15050_e20653: f64 = (locals.var_nj0 * p.p85);
        let assign15050_e20654: f64 = (assign15050_e20650 / assign15050_e20653);
        let assign15050_e20655: f64 = (assign15050_e20645 + assign15050_e20654);
        let assign15050_e20656: f64 = (locals.var_phitdinv * assign15050_e20655);
        let assign15050_e20657: f64 = (assign15050_e20656).abs();
        let assign15050_e20659: f64 = if assign15050_e20657 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign15050_e20659;

        let (assign15060_e20685, assign15060_e20685_d_n0, assign15060_e20685_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard244 != 0.0)) {
        let assign15060_e20671: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15060_e20675: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15060_e20676: f64 = (locals.var_vha1 * assign15060_e20675);
        let assign15060_e20679: f64 = (locals.var_nj0 * p.p85);
        let assign15060_e20680: f64 = (assign15060_e20676 / assign15060_e20679);
        let assign15060_e20681: f64 = (assign15060_e20671 + assign15060_e20680);
        let assign15060_e20682: f64 = (locals.var_phitdinv * assign15060_e20681);
        let assign15060_e20683: f64 = (assign15060_e20682).exp();
        (assign15060_e20683, (assign15060_e20683 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign15060_e20679) - (assign15060_e20676 * (locals.var_nj0_dn0 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign15060_e20679) - (assign15060_e20676 * (locals.var_nj0_dn2 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign15060_e20685;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign15060_e20685_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign15060_e20685_d_n2;

        let assign15070_e20689: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15070_e20693: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15070_e20694: f64 = (locals.var_vha1 * assign15070_e20693);
        let assign15070_e20697: f64 = (locals.var_nj0 * p.p85);
        let assign15070_e20698: f64 = (assign15070_e20694 / assign15070_e20697);
        let assign15070_e20699: f64 = (assign15070_e20689 + assign15070_e20698);
        let assign15070_e20700: f64 = (locals.var_phitdinv * assign15070_e20699);
        let assign15070_e20702: f64 = (-230.25850929940458);
        let assign15070_e20703: f64 = if assign15070_e20700 < assign15070_e20702 { 1.0 } else { 0.0 };
        locals.var_guard245 = assign15070_e20703;

        let (assign15080_e20784, assign15080_e20784_d_n0, assign15080_e20784_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard244 == 0.0)) && (locals.var_guard245 != 0.0)) {
        let assign15080_e20718: f64 = (-230.25850929940458);
        let assign15080_e20722: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15080_e20726: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15080_e20727: f64 = (locals.var_vha1 * assign15080_e20726);
        let assign15080_e20730: f64 = (locals.var_nj0 * p.p85);
        let assign15080_e20731: f64 = (assign15080_e20727 / assign15080_e20730);
        let assign15080_e20732: f64 = (assign15080_e20722 + assign15080_e20731);
        let assign15080_e20733: f64 = (locals.var_phitdinv * assign15080_e20732);
        let assign15080_e20734: f64 = (assign15080_e20718 - assign15080_e20733);
        let assign15080_e20738: f64 = (-230.25850929940458);
        let assign15080_e20742: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15080_e20746: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15080_e20747: f64 = (locals.var_vha1 * assign15080_e20746);
        let assign15080_e20750: f64 = (locals.var_nj0 * p.p85);
        let assign15080_e20751: f64 = (assign15080_e20747 / assign15080_e20750);
        let assign15080_e20752: f64 = (assign15080_e20742 + assign15080_e20751);
        let assign15080_e20753: f64 = (locals.var_phitdinv * assign15080_e20752);
        let assign15080_e20754: f64 = (assign15080_e20738 - assign15080_e20753);
        let assign15080_e20757: f64 = (-230.25850929940458);
        let assign15080_e20761: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15080_e20765: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15080_e20766: f64 = (locals.var_vha1 * assign15080_e20765);
        let assign15080_e20769: f64 = (locals.var_nj0 * p.p85);
        let assign15080_e20770: f64 = (assign15080_e20766 / assign15080_e20769);
        let assign15080_e20771: f64 = (assign15080_e20761 + assign15080_e20770);
        let assign15080_e20772: f64 = (locals.var_phitdinv * assign15080_e20771);
        let assign15080_e20773: f64 = (assign15080_e20757 - assign15080_e20772);
        let assign15080_e20775: f64 = (assign15080_e20773 * 0.3333333333333333);
        let assign15080_e20776: f64 = (1.0 + assign15080_e20775);
        let assign15080_e20777: f64 = (assign15080_e20754 * assign15080_e20776);
        let assign15080_e20778: f64 = (0.5 * assign15080_e20777);
        let assign15080_e20779: f64 = (1.0 + assign15080_e20778);
        let assign15080_e20780: f64 = (assign15080_e20734 * assign15080_e20779);
        let assign15080_e20781: f64 = (1.0 + assign15080_e20780);
        let assign15080_e20782: f64 = (1e-100 / assign15080_e20781);
        (assign15080_e20782, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign15080_e20730) - (assign15080_e20727 * (locals.var_nj0_dn0 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign15080_e20750) - (assign15080_e20747 * (locals.var_nj0_dn0 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign15080_e20769) - (assign15080_e20766 * (locals.var_nj0_dn0 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign15080_e20730) - (assign15080_e20727 * (locals.var_nj0_dn2 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign15080_e20750) - (assign15080_e20747 * (locals.var_nj0_dn2 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign15080_e20769) - (assign15080_e20766 * (locals.var_nj0_dn2 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign15080_e20784;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign15080_e20784_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign15080_e20784_d_n2;

        let (assign15090_e20863, assign15090_e20863_d_n0, assign15090_e20863_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard244 == 0.0)) && (locals.var_guard245 == 0.0)) {
        let assign15090_e20802: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15090_e20806: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15090_e20807: f64 = (locals.var_vha1 * assign15090_e20806);
        let assign15090_e20810: f64 = (locals.var_nj0 * p.p85);
        let assign15090_e20811: f64 = (assign15090_e20807 / assign15090_e20810);
        let assign15090_e20812: f64 = (assign15090_e20802 + assign15090_e20811);
        let assign15090_e20813: f64 = (locals.var_phitdinv * assign15090_e20812);
        let assign15090_e20815: f64 = (assign15090_e20813 - 230.25850929940458);
        let assign15090_e20821: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15090_e20825: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15090_e20826: f64 = (locals.var_vha1 * assign15090_e20825);
        let assign15090_e20829: f64 = (locals.var_nj0 * p.p85);
        let assign15090_e20830: f64 = (assign15090_e20826 / assign15090_e20829);
        let assign15090_e20831: f64 = (assign15090_e20821 + assign15090_e20830);
        let assign15090_e20832: f64 = (locals.var_phitdinv * assign15090_e20831);
        let assign15090_e20834: f64 = (assign15090_e20832 - 230.25850929940458);
        let assign15090_e20839: f64 = (locals.var_vmax / locals.var_nj1);
        let assign15090_e20843: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign15090_e20844: f64 = (locals.var_vha1 * assign15090_e20843);
        let assign15090_e20847: f64 = (locals.var_nj0 * p.p85);
        let assign15090_e20848: f64 = (assign15090_e20844 / assign15090_e20847);
        let assign15090_e20849: f64 = (assign15090_e20839 + assign15090_e20848);
        let assign15090_e20850: f64 = (locals.var_phitdinv * assign15090_e20849);
        let assign15090_e20852: f64 = (assign15090_e20850 - 230.25850929940458);
        let assign15090_e20854: f64 = (assign15090_e20852 * 0.3333333333333333);
        let assign15090_e20855: f64 = (1.0 + assign15090_e20854);
        let assign15090_e20856: f64 = (assign15090_e20834 * assign15090_e20855);
        let assign15090_e20857: f64 = (0.5 * assign15090_e20856);
        let assign15090_e20858: f64 = (1.0 + assign15090_e20857);
        let assign15090_e20859: f64 = (assign15090_e20815 * assign15090_e20858);
        let assign15090_e20860: f64 = (1.0 + assign15090_e20859);
        let assign15090_e20861: f64 = (1e100 * assign15090_e20860);
        (assign15090_e20861, (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign15090_e20810) - (assign15090_e20807 * (locals.var_nj0_dn0 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign15090_e20829) - (assign15090_e20826 * (locals.var_nj0_dn0 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign15090_e20847) - (assign15090_e20844 * (locals.var_nj0_dn0 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign15090_e20810) - (assign15090_e20807 * (locals.var_nj0_dn2 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign15090_e20829) - (assign15090_e20826 * (locals.var_nj0_dn2 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign15090_e20847) - (assign15090_e20844 * (locals.var_nj0_dn2 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign15090_e20863;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign15090_e20863_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign15090_e20863_d_n2;

        let (assign15100_e20890, assign15100_e20890_d_n0, assign15100_e20890_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign15100_e20874: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign15100_e20875: f64 = (locals.var_nj1 - assign15100_e20874);
        let assign15100_e20878: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign15100_e20879: f64 = (assign15100_e20875 / assign15100_e20878);
        let assign15100_e20882: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign15100_e20885: f64 = (locals.var_nj0 * p.p85);
        let assign15100_e20886: f64 = (assign15100_e20882 / assign15100_e20885);
        let assign15100_e20887: f64 = (assign15100_e20879 + assign15100_e20886);
        let assign15100_e20888: f64 = (locals.var_phitdinv * assign15100_e20887);
        (assign15100_e20888, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign15100_e20878) - (assign15100_e20875 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign15100_e20878 * assign15100_e20878)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign15100_e20885) - (assign15100_e20882 * (locals.var_nj0_dn0 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign15100_e20878) - (assign15100_e20875 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign15100_e20878 * assign15100_e20878)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign15100_e20885) - (assign15100_e20882 * (locals.var_nj0_dn2 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign15100_e20890;
        locals.var_dvmax_over_phitd_dv_dn0 = assign15100_e20890_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign15100_e20890_d_n2;

        let (assign15110_e20907, assign15110_e20907_d_n0, assign15110_e20907_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign15110_e20900: f64 = (locals.var_v3 - locals.var_vmax);
        let assign15110_e20902: f64 = (assign15110_e20900 * locals.var_dvmax_over_phitd_dv);
        let assign15110_e20903: f64 = (1.0 + assign15110_e20902);
        let assign15110_e20905: f64 = (assign15110_e20903 * locals.var_exp_vmax_over_phitd_bot);
        (assign15110_e20905, (((assign15110_e20900 * locals.var_dvmax_over_phitd_dv_dn0) * locals.var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * locals.var_exp_vmax_over_phitd_bot_dn0)), (((assign15110_e20900 * locals.var_dvmax_over_phitd_dv_dn2) * locals.var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * locals.var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign15110_e20907;
        locals.var_idmultbot_dn0 = assign15110_e20907_d_n0;
        locals.var_idmultbot_dn2 = assign15110_e20907_d_n2;

        let (assign15120_e20920,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign15120_e20916: f64 = (locals.var_nin * locals.var_nin);
        let assign15120_e20918: f64 = (assign15120_e20916 / locals.var_ndisti_i);
        (assign15120_e20918,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign15120_e20920;

        let (assign15130_e20936,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign15130_e20929: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign15130_e20932: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign15130_e20933: f64 = (assign15130_e20932).ln();
        let assign15130_e20934: f64 = (assign15130_e20929 * assign15130_e20933);
        (assign15130_e20934,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign15130_e20936;

        let assign15140_e20939: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard246 = assign15140_e20939;

        let (assign15150_e20956, assign15150_e20956_d_n0, assign15150_e20956_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15150_e20951: f64 = (locals.var_vmax - locals.var_vha1);
        let assign15150_e20952: f64 = (p.p86 * assign15150_e20951);
        let assign15150_e20954: f64 = (assign15150_e20952 + locals.var_nfasti_i);
        (assign15150_e20954, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign15150_e20956;
        locals.var_nja10_dn0 = assign15150_e20956_d_n0;
        locals.var_nja10_dn2 = assign15150_e20956_d_n2;

        let (assign15160_e20971, assign15160_e20971_d_n0, assign15160_e20971_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15160_e20968: f64 = (p.p86 * locals.var_vha1);
        let assign15160_e20969: f64 = (locals.var_nfasti_i - assign15160_e20968);
        (assign15160_e20969, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15160_e20971;
        locals.var_nj0_dn0 = assign15160_e20971_d_n0;
        locals.var_nj0_dn2 = assign15160_e20971_d_n2;

        let (assign15170_e20986, assign15170_e20986_d_n0, assign15170_e20986_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15170_e20982: f64 = (p.p85 - locals.var_nja10);
        let assign15170_e20984: f64 = (assign15170_e20982 - 0.01);
        (assign15170_e20984, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign15170_e20986;
        locals.var_tmf1_dn0 = assign15170_e20986_d_n0;
        locals.var_tmf1_dn2 = assign15170_e20986_d_n2;

        let (assign15180_e21001, assign15180_e21001_d_n0, assign15180_e21001_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15180_e20997: f64 = (4.0 * p.p85);
        let assign15180_e20999: f64 = (assign15180_e20997 * 0.01);
        (assign15180_e20999, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15180_e21001;
        locals.var_tmf2_dn0 = assign15180_e21001_d_n0;
        locals.var_tmf2_dn2 = assign15180_e21001_d_n2;

        let (assign15190_e21018, assign15190_e21018_d_n0, assign15190_e21018_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let (assign15190_e21016, assign15190_e21016_d_n0, assign15190_e21016_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign15190_e21015: f64 = (-locals.var_tmf2);
                (assign15190_e21015, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign15190_e21016, assign15190_e21016_d_n0, assign15190_e21016_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15190_e21018;
        locals.var_tmf2_dn0 = assign15190_e21018_d_n0;
        locals.var_tmf2_dn2 = assign15190_e21018_d_n2;

        let (assign15200_e21034, assign15200_e21034_d_n0, assign15200_e21034_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15200_e21029: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15200_e21031: f64 = (assign15200_e21029 + locals.var_tmf2);
        let assign15200_e21032: f64 = (assign15200_e21031).sqrt();
        (assign15200_e21032, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15200_e21032)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15200_e21032)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15200_e21034;
        locals.var_tmf2_dn0 = assign15200_e21034_d_n0;
        locals.var_tmf2_dn2 = assign15200_e21034_d_n2;

    }
}

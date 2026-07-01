#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
        var_alphaav_slot: &mut f64,
        var_berfc_slot: &mut f64,
        var_cerfc_slot: &mut f64,
        var_chnl_type_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_deltaphigr_slot: &mut f64,
        var_epssi_slot: &mut f64,
        var_fstopbot_slot: &mut f64,
        var_fstopgat_slot: &mut f64,
        var_fstopsti_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_kbol_over_qele_slot: &mut f64,
        var_one_minus_pbot_slot: &mut f64,
        var_one_minus_pgat_slot: &mut f64,
        var_one_minus_pgat2nd_slot: &mut f64,
        var_one_minus_psti_slot: &mut f64,
        var_one_over_one_minus_pbot_slot: &mut f64,
        var_one_over_one_minus_pgat_slot: &mut f64,
        var_one_over_one_minus_psti_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_perfc_slot: &mut f64,
        var_pgat2nd_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phiggat2nd_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phigrbot_slot: &mut f64,
        var_phigrgat_slot: &mut f64,
        var_phigrgat2nd_slot: &mut f64,
        var_phigrsti_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_phitr_slot: &mut f64,
        var_phitrinv_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_slopebot_slot: &mut f64,
        var_slopegat_slot: &mut f64,
        var_slopegat_db0_slot: &mut f64,
        var_slopegat_db1_slot: &mut f64,
        var_slopegat_db2_slot: &mut f64,
        var_slopegat_db3_slot: &mut f64,
        var_slopegat_db4_slot: &mut f64,
        var_slopegat_db5_slot: &mut f64,
        var_slopegat_db6_slot: &mut f64,
        var_slopegat_dn0_slot: &mut f64,
        var_slopegat_dn1_slot: &mut f64,
        var_slopegat_dn10_slot: &mut f64,
        var_slopegat_dn11_slot: &mut f64,
        var_slopegat_dn12_slot: &mut f64,
        var_slopegat_dn2_slot: &mut f64,
        var_slopegat_dn3_slot: &mut f64,
        var_slopegat_dn4_slot: &mut f64,
        var_slopegat_dn5_slot: &mut f64,
        var_slopegat_dn6_slot: &mut f64,
        var_slopegat_dn7_slot: &mut f64,
        var_slopegat_dn8_slot: &mut f64,
        var_slopegat_dn9_slot: &mut f64,
        var_slopesti_slot: &mut f64,
        var_swgat2nd_slot: &mut f64,
        var_swjunexp_i_slot: &mut f64,
        var_tkr_slot: &mut f64,
        var_tkr_1_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirbotinv_slot: &mut f64,
        var_vbirgat2nd_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirgatinv_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbirstiinv_slot: &mut f64,
        var_vbrinvbot_slot: &mut f64,
        var_vbrinvgat_slot: &mut f64,
        var_vbrinvgat_db0_slot: &mut f64,
        var_vbrinvgat_db1_slot: &mut f64,
        var_vbrinvgat_db2_slot: &mut f64,
        var_vbrinvgat_db3_slot: &mut f64,
        var_vbrinvgat_db4_slot: &mut f64,
        var_vbrinvgat_db5_slot: &mut f64,
        var_vbrinvgat_db6_slot: &mut f64,
        var_vbrinvgat_dn0_slot: &mut f64,
        var_vbrinvgat_dn1_slot: &mut f64,
        var_vbrinvgat_dn10_slot: &mut f64,
        var_vbrinvgat_dn11_slot: &mut f64,
        var_vbrinvgat_dn12_slot: &mut f64,
        var_vbrinvgat_dn2_slot: &mut f64,
        var_vbrinvgat_dn3_slot: &mut f64,
        var_vbrinvgat_dn4_slot: &mut f64,
        var_vbrinvgat_dn5_slot: &mut f64,
        var_vbrinvgat_dn6_slot: &mut f64,
        var_vbrinvgat_dn7_slot: &mut f64,
        var_vbrinvgat_dn8_slot: &mut f64,
        var_vbrinvgat_dn9_slot: &mut f64,
        var_vbrinvsti_slot: &mut f64,
        var_wdepnulrbot_slot: &mut f64,
        var_wdepnulrgat_slot: &mut f64,
        var_wdepnulrinvbot_slot: &mut f64,
        var_wdepnulrinvgat_slot: &mut f64,
        var_wdepnulrinvsti_slot: &mut f64,
        var_wdepnulrsti_slot: &mut f64,
    ) {
        let mut var_alphaav: f64 = *var_alphaav_slot;
        let mut var_berfc: f64 = *var_berfc_slot;
        let mut var_cerfc: f64 = *var_cerfc_slot;
        let mut var_chnl_type: f64 = *var_chnl_type_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_deltaphigr: f64 = *var_deltaphigr_slot;
        let mut var_epssi: f64 = *var_epssi_slot;
        let mut var_fstopbot: f64 = *var_fstopbot_slot;
        let mut var_fstopgat: f64 = *var_fstopgat_slot;
        let mut var_fstopsti: f64 = *var_fstopsti_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_kbol_over_qele: f64 = *var_kbol_over_qele_slot;
        let mut var_one_minus_pbot: f64 = *var_one_minus_pbot_slot;
        let mut var_one_minus_pgat: f64 = *var_one_minus_pgat_slot;
        let mut var_one_minus_pgat2nd: f64 = *var_one_minus_pgat2nd_slot;
        let mut var_one_minus_psti: f64 = *var_one_minus_psti_slot;
        let mut var_one_over_one_minus_pbot: f64 = *var_one_over_one_minus_pbot_slot;
        let mut var_one_over_one_minus_pgat: f64 = *var_one_over_one_minus_pgat_slot;
        let mut var_one_over_one_minus_psti: f64 = *var_one_over_one_minus_psti_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_perfc: f64 = *var_perfc_slot;
        let mut var_pgat2nd: f64 = *var_pgat2nd_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phiggat2nd: f64 = *var_phiggat2nd_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phigrbot: f64 = *var_phigrbot_slot;
        let mut var_phigrgat: f64 = *var_phigrgat_slot;
        let mut var_phigrgat2nd: f64 = *var_phigrgat2nd_slot;
        let mut var_phigrsti: f64 = *var_phigrsti_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_phitr: f64 = *var_phitr_slot;
        let mut var_phitrinv: f64 = *var_phitrinv_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_slopebot: f64 = *var_slopebot_slot;
        let mut var_slopegat: f64 = *var_slopegat_slot;
        let mut var_slopegat_db0: f64 = *var_slopegat_db0_slot;
        let mut var_slopegat_db1: f64 = *var_slopegat_db1_slot;
        let mut var_slopegat_db2: f64 = *var_slopegat_db2_slot;
        let mut var_slopegat_db3: f64 = *var_slopegat_db3_slot;
        let mut var_slopegat_db4: f64 = *var_slopegat_db4_slot;
        let mut var_slopegat_db5: f64 = *var_slopegat_db5_slot;
        let mut var_slopegat_db6: f64 = *var_slopegat_db6_slot;
        let mut var_slopegat_dn0: f64 = *var_slopegat_dn0_slot;
        let mut var_slopegat_dn1: f64 = *var_slopegat_dn1_slot;
        let mut var_slopegat_dn10: f64 = *var_slopegat_dn10_slot;
        let mut var_slopegat_dn11: f64 = *var_slopegat_dn11_slot;
        let mut var_slopegat_dn12: f64 = *var_slopegat_dn12_slot;
        let mut var_slopegat_dn2: f64 = *var_slopegat_dn2_slot;
        let mut var_slopegat_dn3: f64 = *var_slopegat_dn3_slot;
        let mut var_slopegat_dn4: f64 = *var_slopegat_dn4_slot;
        let mut var_slopegat_dn5: f64 = *var_slopegat_dn5_slot;
        let mut var_slopegat_dn6: f64 = *var_slopegat_dn6_slot;
        let mut var_slopegat_dn7: f64 = *var_slopegat_dn7_slot;
        let mut var_slopegat_dn8: f64 = *var_slopegat_dn8_slot;
        let mut var_slopegat_dn9: f64 = *var_slopegat_dn9_slot;
        let mut var_slopesti: f64 = *var_slopesti_slot;
        let mut var_swgat2nd: f64 = *var_swgat2nd_slot;
        let mut var_swjunexp_i: f64 = *var_swjunexp_i_slot;
        let mut var_tkr: f64 = *var_tkr_slot;
        let mut var_tkr_1: f64 = *var_tkr_1_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirbotinv: f64 = *var_vbirbotinv_slot;
        let mut var_vbirgat2nd: f64 = *var_vbirgat2nd_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirgatinv: f64 = *var_vbirgatinv_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbirstiinv: f64 = *var_vbirstiinv_slot;
        let mut var_vbrinvbot: f64 = *var_vbrinvbot_slot;
        let mut var_vbrinvgat: f64 = *var_vbrinvgat_slot;
        let mut var_vbrinvgat_db0: f64 = *var_vbrinvgat_db0_slot;
        let mut var_vbrinvgat_db1: f64 = *var_vbrinvgat_db1_slot;
        let mut var_vbrinvgat_db2: f64 = *var_vbrinvgat_db2_slot;
        let mut var_vbrinvgat_db3: f64 = *var_vbrinvgat_db3_slot;
        let mut var_vbrinvgat_db4: f64 = *var_vbrinvgat_db4_slot;
        let mut var_vbrinvgat_db5: f64 = *var_vbrinvgat_db5_slot;
        let mut var_vbrinvgat_db6: f64 = *var_vbrinvgat_db6_slot;
        let mut var_vbrinvgat_dn0: f64 = *var_vbrinvgat_dn0_slot;
        let mut var_vbrinvgat_dn1: f64 = *var_vbrinvgat_dn1_slot;
        let mut var_vbrinvgat_dn10: f64 = *var_vbrinvgat_dn10_slot;
        let mut var_vbrinvgat_dn11: f64 = *var_vbrinvgat_dn11_slot;
        let mut var_vbrinvgat_dn12: f64 = *var_vbrinvgat_dn12_slot;
        let mut var_vbrinvgat_dn2: f64 = *var_vbrinvgat_dn2_slot;
        let mut var_vbrinvgat_dn3: f64 = *var_vbrinvgat_dn3_slot;
        let mut var_vbrinvgat_dn4: f64 = *var_vbrinvgat_dn4_slot;
        let mut var_vbrinvgat_dn5: f64 = *var_vbrinvgat_dn5_slot;
        let mut var_vbrinvgat_dn6: f64 = *var_vbrinvgat_dn6_slot;
        let mut var_vbrinvgat_dn7: f64 = *var_vbrinvgat_dn7_slot;
        let mut var_vbrinvgat_dn8: f64 = *var_vbrinvgat_dn8_slot;
        let mut var_vbrinvgat_dn9: f64 = *var_vbrinvgat_dn9_slot;
        let mut var_vbrinvsti: f64 = *var_vbrinvsti_slot;
        let mut var_wdepnulrbot: f64 = *var_wdepnulrbot_slot;
        let mut var_wdepnulrgat: f64 = *var_wdepnulrgat_slot;
        let mut var_wdepnulrinvbot: f64 = *var_wdepnulrinvbot_slot;
        let mut var_wdepnulrinvgat: f64 = *var_wdepnulrinvgat_slot;
        let mut var_wdepnulrinvsti: f64 = *var_wdepnulrinvsti_slot;
        let mut var_wdepnulrsti: f64 = *var_wdepnulrsti_slot;

        let assign00_e1484: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1 = assign00_e1484;

        let (assign10_e1489,) = {
    if (var_guard1 != 0.0) {
        let assign10_e1487: f64 = 1.0;
        (assign10_e1487,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign10_e1489;

        let (assign20_e1495,) = {
    if (var_guard1 == 0.0) {
        let assign20_e1493: f64 = (-1.0);
        (assign20_e1493,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign20_e1495;

        let assign30_e1498: f64 = (8.8541878176e-12 * 11.8);
        var_epssi = assign30_e1498;

        let assign40_e1501: f64 = (273.15 + p.p38);
        var_tkr = assign40_e1501;

        var_swjunexp_i = 0.0;

        let assign60_e1505: f64 = if p.p944 > 0.5 { 1.0 } else { 0.0 };
        var_guard2 = assign60_e1505;

        let (assign70_e1509,) = {
    if (var_guard2 != 0.0) {
        (1.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign70_e1509;

        let (assign80_e1514,) = {
    if (var_guard2 == 0.0) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign80_e1514;

        let assign90_e1517: f64 = (273.15 + p.p840);
        var_tkr_1 = assign90_e1517;

        let assign100_e1520: f64 = (1.3806505e-23 / 1.6021918e-19);
        var_kbol_over_qele = assign100_e1520;

        let assign110_e1523: f64 = (var_kbol_over_qele * var_tkr_1);
        var_phitr = assign110_e1523;

        let assign120_e1526: f64 = (1.0 / var_phitr);
        var_phitrinv = assign120_e1526;

        let assign130_e1529: f64 = (0.000702 * var_tkr_1);
        let assign130_e1531: f64 = (assign130_e1529 * var_tkr_1);
        let assign130_e1532: f64 = (-assign130_e1531);
        let assign130_e1535: f64 = (1108.0 + var_tkr_1);
        let assign130_e1536: f64 = (assign130_e1532 / assign130_e1535);
        var_deltaphigr = assign130_e1536;

        let assign140_e1539: f64 = (p.p851 + var_deltaphigr);
        var_phigrbot = assign140_e1539;

        let assign150_e1542: f64 = (p.p852 + var_deltaphigr);
        var_phigrsti = assign150_e1542;

        let assign160_e1545: f64 = (p.p853 + var_deltaphigr);
        var_phigrgat = assign160_e1545;

        let assign170_e1548: f64 = (1.0 - p.p848);
        var_one_minus_pbot = assign170_e1548;

        let assign180_e1551: f64 = (1.0 - p.p849);
        var_one_minus_psti = assign180_e1551;

        let assign190_e1554: f64 = (1.0 - p.p850);
        var_one_minus_pgat = assign190_e1554;

        let assign200_e1557: f64 = (1.0 / var_one_minus_pbot);
        var_one_over_one_minus_pbot = assign200_e1557;

        let assign210_e1560: f64 = (1.0 / var_one_minus_psti);
        var_one_over_one_minus_psti = assign210_e1560;

        let assign220_e1563: f64 = (1.0 / var_one_minus_pgat);
        var_one_over_one_minus_pgat = assign220_e1563;

        let assign230_e1566: f64 = (var_epssi / p.p842);
        var_wdepnulrbot = assign230_e1566;

        let assign240_e1569: f64 = (p.p860 * var_epssi);
        let assign240_e1571: f64 = (assign240_e1569 / p.p843);
        var_wdepnulrsti = assign240_e1571;

        let assign250_e1574: f64 = (p.p861 * var_epssi);
        let assign250_e1576: f64 = (assign250_e1574 / p.p844);
        var_wdepnulrgat = assign250_e1576;

        let assign260_e1579: f64 = (1.0 / var_wdepnulrbot);
        var_wdepnulrinvbot = assign260_e1579;

        let assign270_e1582: f64 = (1.0 / var_wdepnulrsti);
        var_wdepnulrinvsti = assign270_e1582;

        let assign280_e1585: f64 = (1.0 / var_wdepnulrgat);
        var_wdepnulrinvgat = assign280_e1585;

        let assign290_e1588: f64 = (1.0 / p.p845);
        var_vbirbotinv = assign290_e1588;

        let assign300_e1591: f64 = (1.0 / p.p846);
        var_vbirstiinv = assign300_e1591;

        let assign310_e1594: f64 = (1.0 / p.p847);
        var_vbirgatinv = assign310_e1594;

        let assign320_e1597: f64 = (1.772453850905516 * 0.29214664);
        var_perfc = assign320_e1597;

        let assign330_e1599: f64 = (-5.0);
        let assign330_e1601: f64 = (assign330_e1599 * 0.29214664);
        let assign330_e1603: f64 = (assign330_e1601 + 6.0);
        let assign330_e1606: f64 = (-2.0);
        let assign330_e1607: f64 = (var_perfc).powf(assign330_e1606);
        let assign330_e1608: f64 = (assign330_e1603 - assign330_e1607);
        let assign330_e1610: f64 = (assign330_e1608 / 3.0);
        var_berfc = assign330_e1610;

        let assign340_e1613: f64 = (1.0 - 0.29214664);
        let assign340_e1615: f64 = (assign340_e1613 - var_berfc);
        var_cerfc = assign340_e1615;

        let assign350_e1619: f64 = (1.0 / p.p841);
        let assign350_e1620: f64 = (1.0 - assign350_e1619);
        var_alphaav = assign350_e1620;

        let assign360_e1625: f64 = (var_alphaav).powf(p.p880);
        let assign360_e1626: f64 = (1.0 - assign360_e1625);
        let assign360_e1627: f64 = (1.0 / assign360_e1626);
        var_fstopbot = assign360_e1627;

        let assign370_e1632: f64 = (var_alphaav).powf(p.p881);
        let assign370_e1633: f64 = (1.0 - assign370_e1632);
        let assign370_e1634: f64 = (1.0 / assign370_e1633);
        var_fstopsti = assign370_e1634;

        let assign380_e1639: f64 = (var_alphaav).powf(p.p882);
        let assign380_e1640: f64 = (1.0 - assign380_e1639);
        let assign380_e1641: f64 = (1.0 / assign380_e1640);
        var_fstopgat = assign380_e1641;

        let assign390_e1644: f64 = (1.0 / p.p877);
        var_vbrinvbot = assign390_e1644;

        let assign400_e1647: f64 = (1.0 / p.p878);
        var_vbrinvsti = assign400_e1647;

        let assign410_e1650: f64 = (1.0 / p.p879);
        var_vbrinvgat = assign410_e1650;
        var_vbrinvgat_dn0 = 0.0;
        var_vbrinvgat_dn1 = 0.0;
        var_vbrinvgat_dn2 = 0.0;
        var_vbrinvgat_dn3 = 0.0;
        var_vbrinvgat_dn4 = 0.0;
        var_vbrinvgat_dn5 = 0.0;
        var_vbrinvgat_dn6 = 0.0;
        var_vbrinvgat_dn7 = 0.0;
        var_vbrinvgat_dn8 = 0.0;
        var_vbrinvgat_dn9 = 0.0;
        var_vbrinvgat_dn10 = 0.0;
        var_vbrinvgat_dn11 = 0.0;
        var_vbrinvgat_dn12 = 0.0;
        var_vbrinvgat_db0 = 0.0;
        var_vbrinvgat_db1 = 0.0;
        var_vbrinvgat_db2 = 0.0;
        var_vbrinvgat_db3 = 0.0;
        var_vbrinvgat_db4 = 0.0;
        var_vbrinvgat_db5 = 0.0;
        var_vbrinvgat_db6 = 0.0;

        let assign420_e1653: f64 = (var_fstopbot * var_fstopbot);
        let assign420_e1657: f64 = (p.p880 - 1.0);
        let assign420_e1658: f64 = (var_alphaav).powf(assign420_e1657);
        let assign420_e1659: f64 = (assign420_e1653 * assign420_e1658);
        let assign420_e1660: f64 = (-assign420_e1659);
        let assign420_e1662: f64 = (assign420_e1660 * p.p880);
        let assign420_e1664: f64 = (assign420_e1662 * var_vbrinvbot);
        var_slopebot = assign420_e1664;

        let assign430_e1667: f64 = (var_fstopsti * var_fstopsti);
        let assign430_e1671: f64 = (p.p881 - 1.0);
        let assign430_e1672: f64 = (var_alphaav).powf(assign430_e1671);
        let assign430_e1673: f64 = (assign430_e1667 * assign430_e1672);
        let assign430_e1674: f64 = (-assign430_e1673);
        let assign430_e1676: f64 = (assign430_e1674 * p.p881);
        let assign430_e1678: f64 = (assign430_e1676 * var_vbrinvsti);
        var_slopesti = assign430_e1678;

        let assign440_e1681: f64 = (var_fstopgat * var_fstopgat);
        let assign440_e1685: f64 = (p.p882 - 1.0);
        let assign440_e1686: f64 = (var_alphaav).powf(assign440_e1685);
        let assign440_e1687: f64 = (assign440_e1681 * assign440_e1686);
        let assign440_e1688: f64 = (-assign440_e1687);
        let assign440_e1690: f64 = (assign440_e1688 * p.p882);
        let assign440_e1692: f64 = (assign440_e1690 * var_vbrinvgat);
        var_slopegat = assign440_e1692;
        var_slopegat_dn0 = (assign440_e1690 * var_vbrinvgat_dn0);
        var_slopegat_dn1 = (assign440_e1690 * var_vbrinvgat_dn1);
        var_slopegat_dn2 = (assign440_e1690 * var_vbrinvgat_dn2);
        var_slopegat_dn3 = (assign440_e1690 * var_vbrinvgat_dn3);
        var_slopegat_dn4 = (assign440_e1690 * var_vbrinvgat_dn4);
        var_slopegat_dn5 = (assign440_e1690 * var_vbrinvgat_dn5);
        var_slopegat_dn6 = (assign440_e1690 * var_vbrinvgat_dn6);
        var_slopegat_dn7 = (assign440_e1690 * var_vbrinvgat_dn7);
        var_slopegat_dn8 = (assign440_e1690 * var_vbrinvgat_dn8);
        var_slopegat_dn9 = (assign440_e1690 * var_vbrinvgat_dn9);
        var_slopegat_dn10 = (assign440_e1690 * var_vbrinvgat_dn10);
        var_slopegat_dn11 = (assign440_e1690 * var_vbrinvgat_dn11);
        var_slopegat_dn12 = (assign440_e1690 * var_vbrinvgat_dn12);
        var_slopegat_db0 = (assign440_e1690 * var_vbrinvgat_db0);
        var_slopegat_db1 = (assign440_e1690 * var_vbrinvgat_db1);
        var_slopegat_db2 = (assign440_e1690 * var_vbrinvgat_db2);
        var_slopegat_db3 = (assign440_e1690 * var_vbrinvgat_db3);
        var_slopegat_db4 = (assign440_e1690 * var_vbrinvgat_db4);
        var_slopegat_db5 = (assign440_e1690 * var_vbrinvgat_db5);
        var_slopegat_db6 = (assign440_e1690 * var_vbrinvgat_db6);

        let assign450_e1707: f64 = if ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0)) { 1.0 } else { 0.0 };
        var_guard3 = assign450_e1707;

        let (assign460_e1711,) = {
    if (var_guard3 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign460_e1711;

        let (assign470_e1716,) = {
    if (var_guard3 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign470_e1716;

        let assign480_e1719: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard4 = assign480_e1719;

        if (s.v[984] != 0.0) {
            s.store_scalar(458, (if ((p.p844 * p.p883) > 1e-18) { (p.p844 * p.p883) } else { 1e-18 }));
        }

        let (assign500_e1745,) = {
    if (var_guard4 != 0.0) {
        let assign500_e1736: f64 = (p.p847 * p.p884);
        let (assign500_e1743,) = {
            if (assign500_e1736 > 0.05) {
                let assign500_e1741: f64 = (p.p847 * p.p884);
                (assign500_e1741,)
            } else {
                (0.05,)
            }
        };
        (assign500_e1743,)
    } else {
        (var_vbirgat2nd,)
    }
};
        var_vbirgat2nd = assign500_e1745;

        let (assign510_e1772,) = {
    if (var_guard4 != 0.0) {
        let assign510_e1749: f64 = (p.p850 * p.p885);
        let (assign510_e1756,) = {
            if (assign510_e1749 > 0.05) {
                let assign510_e1754: f64 = (p.p850 * p.p885);
                (assign510_e1754,)
            } else {
                (0.05,)
            }
        };
        let (assign510_e1770,) = {
            if (assign510_e1756 < 0.95) {
                let assign510_e1761: f64 = (p.p850 * p.p885);
                let (assign510_e1768,) = {
                    if (assign510_e1761 > 0.05) {
                        let assign510_e1766: f64 = (p.p850 * p.p885);
                        (assign510_e1766,)
                    } else {
                        (0.05,)
                    }
                };
                (assign510_e1768,)
            } else {
                (0.95,)
            }
        };
        (assign510_e1770,)
    } else {
        (var_pgat2nd,)
    }
};
        var_pgat2nd = assign510_e1772;

        let (assign520_e1778,) = {
    if (var_guard4 != 0.0) {
        let assign520_e1776: f64 = (p.p853 * p.p886);
        (assign520_e1776,)
    } else {
        (var_phiggat2nd,)
    }
};
        var_phiggat2nd = assign520_e1778;

        let (assign530_e1784,) = {
    if (var_guard4 != 0.0) {
        let assign530_e1782: f64 = (var_phiggat2nd + var_deltaphigr);
        (assign530_e1782,)
    } else {
        (var_phigrgat2nd,)
    }
};
        var_phigrgat2nd = assign530_e1784;

        let (assign540_e1790,) = {
    if (var_guard4 != 0.0) {
        let assign540_e1788: f64 = (1.0 - var_pgat2nd);
        (assign540_e1788,)
    } else {
        (var_one_minus_pgat2nd,)
    }
};
        var_one_minus_pgat2nd = assign540_e1790;

        if (s.v[984] != 0.0) {
            s.store_scalar(469, (1.0 / var_one_minus_pgat2nd));
        }

        let assign560_e1799: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign560_e1799;

        let (assign570_e1803,) = {
    if (var_guard5 != 0.0) {
        (p.p842,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign570_e1803;

        let (assign580_e1807,) = {
    if (var_guard5 != 0.0) {
        (p.p843,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign580_e1807;

        let (assign590_e1811,) = {
    if (var_guard5 != 0.0) {
        (p.p844,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign590_e1811;

        let (assign600_e1815,) = {
    if (var_guard5 != 0.0) {
        (p.p845,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign600_e1815;

        let (assign610_e1819,) = {
    if (var_guard5 != 0.0) {
        (p.p846,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign610_e1819;

        let (assign620_e1823,) = {
    if (var_guard5 != 0.0) {
        (p.p847,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign620_e1823;

        let (assign630_e1827,) = {
    if (var_guard5 != 0.0) {
        (p.p848,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign630_e1827;

        let (assign640_e1831,) = {
    if (var_guard5 != 0.0) {
        (p.p849,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign640_e1831;

        let (assign650_e1835,) = {
    if (var_guard5 != 0.0) {
        (p.p850,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign650_e1835;

        let (assign660_e1839,) = {
    if (var_guard5 != 0.0) {
        (p.p851,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign660_e1839;

        let (assign670_e1843,) = {
    if (var_guard5 != 0.0) {
        (p.p852,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign670_e1843;

        let (assign680_e1847,) = {
    if (var_guard5 != 0.0) {
        (p.p853,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign680_e1847;

        let (assign690_e1851,) = {
    if (var_guard5 != 0.0) {
        (p.p854,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign690_e1851;

        let (assign700_e1855,) = {
    if (var_guard5 != 0.0) {
        (p.p855,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign700_e1855;

        let (assign710_e1859,) = {
    if (var_guard5 != 0.0) {
        (p.p856,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign710_e1859;

        let (assign720_e1863,) = {
    if (var_guard5 != 0.0) {
        (p.p857,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign720_e1863;

        let (assign730_e1867,) = {
    if (var_guard5 != 0.0) {
        (p.p858,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign730_e1867;

        *var_alphaav_slot = var_alphaav;
        *var_berfc_slot = var_berfc;
        *var_cerfc_slot = var_cerfc;
        *var_chnl_type_slot = var_chnl_type;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_deltaphigr_slot = var_deltaphigr;
        *var_epssi_slot = var_epssi;
        *var_fstopbot_slot = var_fstopbot;
        *var_fstopgat_slot = var_fstopgat;
        *var_fstopsti_slot = var_fstopsti;
        *var_guard1_slot = var_guard1;
        *var_guard2_slot = var_guard2;
        *var_guard3_slot = var_guard3;
        *var_guard4_slot = var_guard4;
        *var_guard5_slot = var_guard5;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_kbol_over_qele_slot = var_kbol_over_qele;
        *var_one_minus_pbot_slot = var_one_minus_pbot;
        *var_one_minus_pgat_slot = var_one_minus_pgat;
        *var_one_minus_pgat2nd_slot = var_one_minus_pgat2nd;
        *var_one_minus_psti_slot = var_one_minus_psti;
        *var_one_over_one_minus_pbot_slot = var_one_over_one_minus_pbot;
        *var_one_over_one_minus_pgat_slot = var_one_over_one_minus_pgat;
        *var_one_over_one_minus_psti_slot = var_one_over_one_minus_psti;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_perfc_slot = var_perfc;
        *var_pgat2nd_slot = var_pgat2nd;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phiggat2nd_slot = var_phiggat2nd;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phigrbot_slot = var_phigrbot;
        *var_phigrgat_slot = var_phigrgat;
        *var_phigrgat2nd_slot = var_phigrgat2nd;
        *var_phigrsti_slot = var_phigrsti;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_phitr_slot = var_phitr;
        *var_phitrinv_slot = var_phitrinv;
        *var_pstid_i_slot = var_pstid_i;
        *var_slopebot_slot = var_slopebot;
        *var_slopegat_slot = var_slopegat;
        *var_slopegat_db0_slot = var_slopegat_db0;
        *var_slopegat_db1_slot = var_slopegat_db1;
        *var_slopegat_db2_slot = var_slopegat_db2;
        *var_slopegat_db3_slot = var_slopegat_db3;
        *var_slopegat_db4_slot = var_slopegat_db4;
        *var_slopegat_db5_slot = var_slopegat_db5;
        *var_slopegat_db6_slot = var_slopegat_db6;
        *var_slopegat_dn0_slot = var_slopegat_dn0;
        *var_slopegat_dn1_slot = var_slopegat_dn1;
        *var_slopegat_dn10_slot = var_slopegat_dn10;
        *var_slopegat_dn11_slot = var_slopegat_dn11;
        *var_slopegat_dn12_slot = var_slopegat_dn12;
        *var_slopegat_dn2_slot = var_slopegat_dn2;
        *var_slopegat_dn3_slot = var_slopegat_dn3;
        *var_slopegat_dn4_slot = var_slopegat_dn4;
        *var_slopegat_dn5_slot = var_slopegat_dn5;
        *var_slopegat_dn6_slot = var_slopegat_dn6;
        *var_slopegat_dn7_slot = var_slopegat_dn7;
        *var_slopegat_dn8_slot = var_slopegat_dn8;
        *var_slopegat_dn9_slot = var_slopegat_dn9;
        *var_slopesti_slot = var_slopesti;
        *var_swgat2nd_slot = var_swgat2nd;
        *var_swjunexp_i_slot = var_swjunexp_i;
        *var_tkr_slot = var_tkr;
        *var_tkr_1_slot = var_tkr_1;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirbotinv_slot = var_vbirbotinv;
        *var_vbirgat2nd_slot = var_vbirgat2nd;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirgatinv_slot = var_vbirgatinv;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbirstiinv_slot = var_vbirstiinv;
        *var_vbrinvbot_slot = var_vbrinvbot;
        *var_vbrinvgat_slot = var_vbrinvgat;
        *var_vbrinvgat_db0_slot = var_vbrinvgat_db0;
        *var_vbrinvgat_db1_slot = var_vbrinvgat_db1;
        *var_vbrinvgat_db2_slot = var_vbrinvgat_db2;
        *var_vbrinvgat_db3_slot = var_vbrinvgat_db3;
        *var_vbrinvgat_db4_slot = var_vbrinvgat_db4;
        *var_vbrinvgat_db5_slot = var_vbrinvgat_db5;
        *var_vbrinvgat_db6_slot = var_vbrinvgat_db6;
        *var_vbrinvgat_dn0_slot = var_vbrinvgat_dn0;
        *var_vbrinvgat_dn1_slot = var_vbrinvgat_dn1;
        *var_vbrinvgat_dn10_slot = var_vbrinvgat_dn10;
        *var_vbrinvgat_dn11_slot = var_vbrinvgat_dn11;
        *var_vbrinvgat_dn12_slot = var_vbrinvgat_dn12;
        *var_vbrinvgat_dn2_slot = var_vbrinvgat_dn2;
        *var_vbrinvgat_dn3_slot = var_vbrinvgat_dn3;
        *var_vbrinvgat_dn4_slot = var_vbrinvgat_dn4;
        *var_vbrinvgat_dn5_slot = var_vbrinvgat_dn5;
        *var_vbrinvgat_dn6_slot = var_vbrinvgat_dn6;
        *var_vbrinvgat_dn7_slot = var_vbrinvgat_dn7;
        *var_vbrinvgat_dn8_slot = var_vbrinvgat_dn8;
        *var_vbrinvgat_dn9_slot = var_vbrinvgat_dn9;
        *var_vbrinvsti_slot = var_vbrinvsti;
        *var_wdepnulrbot_slot = var_wdepnulrbot;
        *var_wdepnulrgat_slot = var_wdepnulrgat;
        *var_wdepnulrinvbot_slot = var_wdepnulrinvbot;
        *var_wdepnulrinvgat_slot = var_wdepnulrinvgat;
        *var_wdepnulrinvsti_slot = var_wdepnulrinvsti;
        *var_wdepnulrsti_slot = var_wdepnulrsti;
    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        var_guard5: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vjunrefd_i_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vjunrefd_i: f64 = *var_vjunrefd_i_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;

        let (assign740_e1871,) = {
    if (var_guard5 != 0.0) {
        (p.p859,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign740_e1871;

        let (assign750_e1875,) = {
    if (var_guard5 != 0.0) {
        (p.p860,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign750_e1875;

        let (assign760_e1879,) = {
    if (var_guard5 != 0.0) {
        (p.p861,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign760_e1879;

        let (assign770_e1883,) = {
    if (var_guard5 != 0.0) {
        (p.p862,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign770_e1883;

        let (assign780_e1887,) = {
    if (var_guard5 != 0.0) {
        (p.p863,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign780_e1887;

        let (assign790_e1891,) = {
    if (var_guard5 != 0.0) {
        (p.p864,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign790_e1891;

        let (assign800_e1895,) = {
    if (var_guard5 != 0.0) {
        (p.p865,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign800_e1895;

        let (assign810_e1899,) = {
    if (var_guard5 != 0.0) {
        (p.p866,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign810_e1899;

        let (assign820_e1903,) = {
    if (var_guard5 != 0.0) {
        (p.p867,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign820_e1903;

        let (assign830_e1907,) = {
    if (var_guard5 != 0.0) {
        (p.p868,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign830_e1907;

        let (assign840_e1911,) = {
    if (var_guard5 != 0.0) {
        (p.p869,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign840_e1911;

        let (assign850_e1915,) = {
    if (var_guard5 != 0.0) {
        (p.p870,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign850_e1915;

        let (assign860_e1919,) = {
    if (var_guard5 != 0.0) {
        (p.p871,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign860_e1919;

        let (assign870_e1923,) = {
    if (var_guard5 != 0.0) {
        (p.p872,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign870_e1923;

        let (assign880_e1927,) = {
    if (var_guard5 != 0.0) {
        (p.p873,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign880_e1927;

        let (assign890_e1931,) = {
    if (var_guard5 != 0.0) {
        (p.p874,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign890_e1931;

        let (assign900_e1935,) = {
    if (var_guard5 != 0.0) {
        (p.p875,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign900_e1935;

        let (assign910_e1939,) = {
    if (var_guard5 != 0.0) {
        (p.p876,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign910_e1939;

        let (assign920_e1943,) = {
    if (var_guard5 != 0.0) {
        (p.p877,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign920_e1943;

        let (assign930_e1947,) = {
    if (var_guard5 != 0.0) {
        (p.p878,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign930_e1947;

        let (assign940_e1951,) = {
    if (var_guard5 != 0.0) {
        (p.p879,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign940_e1951;

        let (assign950_e1955,) = {
    if (var_guard5 != 0.0) {
        (p.p880,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign950_e1955;

        let (assign960_e1959,) = {
    if (var_guard5 != 0.0) {
        (p.p881,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign960_e1959;

        let (assign970_e1963,) = {
    if (var_guard5 != 0.0) {
        (p.p882,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign970_e1963;

        let (assign980_e1967,) = {
    if (var_guard5 != 0.0) {
        (p.p945,)
    } else {
        (var_vjunrefd_i,)
    }
};
        var_vjunrefd_i = assign980_e1967;

        let (assign990_e1971,) = {
    if (var_guard5 != 0.0) {
        (p.p946,)
    } else {
        (s.v[554],)
    }
};
        s.store_scalar(554, assign990_e1971);

        let (assign1000_e1975,) = {
    if (var_guard5 != 0.0) {
        (p.p889,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1000_e1975;

        let (assign1010_e1979,) = {
    if (var_guard5 != 0.0) {
        (p.p890,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1010_e1979;

        let (assign1020_e1983,) = {
    if (var_guard5 != 0.0) {
        (p.p891,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1020_e1983;

        let (assign1030_e1987,) = {
    if (var_guard5 != 0.0) {
        (p.p892,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1030_e1987;

        let (assign1040_e1991,) = {
    if (var_guard5 != 0.0) {
        (p.p883,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1040_e1991;

        let (assign1050_e1995,) = {
    if (var_guard5 != 0.0) {
        (p.p884,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1050_e1995;

        let (assign1060_e1999,) = {
    if (var_guard5 != 0.0) {
        (p.p885,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1060_e1999;

        let (assign1070_e2003,) = {
    if (var_guard5 != 0.0) {
        (p.p886,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1070_e2003;

        let (assign1080_e2007,) = {
    if (var_guard5 != 0.0) {
        (p.p887,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1080_e2007;

        let (assign1090_e2011,) = {
    if (var_guard5 != 0.0) {
        (p.p888,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1090_e2011;

        let (assign1100_e2016,) = {
    if (var_guard5 == 0.0) {
        (p.p893,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign1100_e2016;

        let (assign1110_e2021,) = {
    if (var_guard5 == 0.0) {
        (p.p894,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign1110_e2021;

        let (assign1120_e2026,) = {
    if (var_guard5 == 0.0) {
        (p.p895,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign1120_e2026;

        let (assign1130_e2031,) = {
    if (var_guard5 == 0.0) {
        (p.p896,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign1130_e2031;

        let (assign1140_e2036,) = {
    if (var_guard5 == 0.0) {
        (p.p897,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign1140_e2036;

        let (assign1150_e2041,) = {
    if (var_guard5 == 0.0) {
        (p.p898,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign1150_e2041;

        let (assign1160_e2046,) = {
    if (var_guard5 == 0.0) {
        (p.p899,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign1160_e2046;

        let (assign1170_e2051,) = {
    if (var_guard5 == 0.0) {
        (p.p900,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign1170_e2051;

        let (assign1180_e2056,) = {
    if (var_guard5 == 0.0) {
        (p.p901,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign1180_e2056;

        let (assign1190_e2061,) = {
    if (var_guard5 == 0.0) {
        (p.p902,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign1190_e2061;

        let (assign1200_e2066,) = {
    if (var_guard5 == 0.0) {
        (p.p903,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign1200_e2066;

        let (assign1210_e2071,) = {
    if (var_guard5 == 0.0) {
        (p.p904,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign1210_e2071;

        let (assign1220_e2076,) = {
    if (var_guard5 == 0.0) {
        (p.p905,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign1220_e2076;

        let (assign1230_e2081,) = {
    if (var_guard5 == 0.0) {
        (p.p906,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign1230_e2081;

        let (assign1240_e2086,) = {
    if (var_guard5 == 0.0) {
        (p.p907,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign1240_e2086;

        let (assign1250_e2091,) = {
    if (var_guard5 == 0.0) {
        (p.p908,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign1250_e2091;

        let (assign1260_e2096,) = {
    if (var_guard5 == 0.0) {
        (p.p909,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign1260_e2096;

        let (assign1270_e2101,) = {
    if (var_guard5 == 0.0) {
        (p.p910,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign1270_e2101;

        let (assign1280_e2106,) = {
    if (var_guard5 == 0.0) {
        (p.p911,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign1280_e2106;

        let (assign1290_e2111,) = {
    if (var_guard5 == 0.0) {
        (p.p912,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign1290_e2111;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_pstid_i_slot = var_pstid_i;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vjunrefd_i_slot = var_vjunrefd_i;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjunstid_i_slot = var_xjunstid_i;
    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        var_alphaav: f64,
        var_cjorbotd_i: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigr: f64,
        var_epssi: f64,
        var_guard5: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggatd_i: f64,
        var_phigstid_i: f64,
        var_pstid_i: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_xjungatd_i: f64,
        var_xjunstid_i: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fstopbot_d_slot: &mut f64,
        var_fstopgat_d_slot: &mut f64,
        var_fstopsti_d_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_one_minus_pbot_d_slot: &mut f64,
        var_one_minus_pgat_d_slot: &mut f64,
        var_one_minus_psti_d_slot: &mut f64,
        var_one_over_one_minus_pbot_d_slot: &mut f64,
        var_one_over_one_minus_pgat_d_slot: &mut f64,
        var_one_over_one_minus_psti_d_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_phigrbot_d_slot: &mut f64,
        var_phigrgat_d_slot: &mut f64,
        var_phigrsti_d_slot: &mut f64,
        var_slopebot_d_slot: &mut f64,
        var_slopegat_d_slot: &mut f64,
        var_slopegat_d_db0_slot: &mut f64,
        var_slopegat_d_db1_slot: &mut f64,
        var_slopegat_d_db2_slot: &mut f64,
        var_slopegat_d_db3_slot: &mut f64,
        var_slopegat_d_db4_slot: &mut f64,
        var_slopegat_d_db5_slot: &mut f64,
        var_slopegat_d_db6_slot: &mut f64,
        var_slopegat_d_dn0_slot: &mut f64,
        var_slopegat_d_dn1_slot: &mut f64,
        var_slopegat_d_dn10_slot: &mut f64,
        var_slopegat_d_dn11_slot: &mut f64,
        var_slopegat_d_dn12_slot: &mut f64,
        var_slopegat_d_dn2_slot: &mut f64,
        var_slopegat_d_dn3_slot: &mut f64,
        var_slopegat_d_dn4_slot: &mut f64,
        var_slopegat_d_dn5_slot: &mut f64,
        var_slopegat_d_dn6_slot: &mut f64,
        var_slopegat_d_dn7_slot: &mut f64,
        var_slopegat_d_dn8_slot: &mut f64,
        var_slopegat_d_dn9_slot: &mut f64,
        var_slopesti_d_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_swgat2nd_d_slot: &mut f64,
        var_vbirbotinv_d_slot: &mut f64,
        var_vbirgat2nd_d_slot: &mut f64,
        var_vbirgatinv_d_slot: &mut f64,
        var_vbirstiinv_d_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrinvbot_d_slot: &mut f64,
        var_vbrinvgat_d_slot: &mut f64,
        var_vbrinvgat_d_db0_slot: &mut f64,
        var_vbrinvgat_d_db1_slot: &mut f64,
        var_vbrinvgat_d_db2_slot: &mut f64,
        var_vbrinvgat_d_db3_slot: &mut f64,
        var_vbrinvgat_d_db4_slot: &mut f64,
        var_vbrinvgat_d_db5_slot: &mut f64,
        var_vbrinvgat_d_db6_slot: &mut f64,
        var_vbrinvgat_d_dn0_slot: &mut f64,
        var_vbrinvgat_d_dn1_slot: &mut f64,
        var_vbrinvgat_d_dn10_slot: &mut f64,
        var_vbrinvgat_d_dn11_slot: &mut f64,
        var_vbrinvgat_d_dn12_slot: &mut f64,
        var_vbrinvgat_d_dn2_slot: &mut f64,
        var_vbrinvgat_d_dn3_slot: &mut f64,
        var_vbrinvgat_d_dn4_slot: &mut f64,
        var_vbrinvgat_d_dn5_slot: &mut f64,
        var_vbrinvgat_d_dn6_slot: &mut f64,
        var_vbrinvgat_d_dn7_slot: &mut f64,
        var_vbrinvgat_d_dn8_slot: &mut f64,
        var_vbrinvgat_d_dn9_slot: &mut f64,
        var_vbrinvsti_d_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vjunrefd_i_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_wdepnulrbot_d_slot: &mut f64,
        var_wdepnulrgat_d_slot: &mut f64,
        var_wdepnulrinvbot_d_slot: &mut f64,
        var_wdepnulrinvgat_d_slot: &mut f64,
        var_wdepnulrinvsti_d_slot: &mut f64,
        var_wdepnulrsti_d_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fstopbot_d: f64 = *var_fstopbot_d_slot;
        let mut var_fstopgat_d: f64 = *var_fstopgat_d_slot;
        let mut var_fstopsti_d: f64 = *var_fstopsti_d_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_one_minus_pbot_d: f64 = *var_one_minus_pbot_d_slot;
        let mut var_one_minus_pgat_d: f64 = *var_one_minus_pgat_d_slot;
        let mut var_one_minus_psti_d: f64 = *var_one_minus_psti_d_slot;
        let mut var_one_over_one_minus_pbot_d: f64 = *var_one_over_one_minus_pbot_d_slot;
        let mut var_one_over_one_minus_pgat_d: f64 = *var_one_over_one_minus_pgat_d_slot;
        let mut var_one_over_one_minus_psti_d: f64 = *var_one_over_one_minus_psti_d_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_phigrbot_d: f64 = *var_phigrbot_d_slot;
        let mut var_phigrgat_d: f64 = *var_phigrgat_d_slot;
        let mut var_phigrsti_d: f64 = *var_phigrsti_d_slot;
        let mut var_slopebot_d: f64 = *var_slopebot_d_slot;
        let mut var_slopegat_d: f64 = *var_slopegat_d_slot;
        let mut var_slopegat_d_db0: f64 = *var_slopegat_d_db0_slot;
        let mut var_slopegat_d_db1: f64 = *var_slopegat_d_db1_slot;
        let mut var_slopegat_d_db2: f64 = *var_slopegat_d_db2_slot;
        let mut var_slopegat_d_db3: f64 = *var_slopegat_d_db3_slot;
        let mut var_slopegat_d_db4: f64 = *var_slopegat_d_db4_slot;
        let mut var_slopegat_d_db5: f64 = *var_slopegat_d_db5_slot;
        let mut var_slopegat_d_db6: f64 = *var_slopegat_d_db6_slot;
        let mut var_slopegat_d_dn0: f64 = *var_slopegat_d_dn0_slot;
        let mut var_slopegat_d_dn1: f64 = *var_slopegat_d_dn1_slot;
        let mut var_slopegat_d_dn10: f64 = *var_slopegat_d_dn10_slot;
        let mut var_slopegat_d_dn11: f64 = *var_slopegat_d_dn11_slot;
        let mut var_slopegat_d_dn12: f64 = *var_slopegat_d_dn12_slot;
        let mut var_slopegat_d_dn2: f64 = *var_slopegat_d_dn2_slot;
        let mut var_slopegat_d_dn3: f64 = *var_slopegat_d_dn3_slot;
        let mut var_slopegat_d_dn4: f64 = *var_slopegat_d_dn4_slot;
        let mut var_slopegat_d_dn5: f64 = *var_slopegat_d_dn5_slot;
        let mut var_slopegat_d_dn6: f64 = *var_slopegat_d_dn6_slot;
        let mut var_slopegat_d_dn7: f64 = *var_slopegat_d_dn7_slot;
        let mut var_slopegat_d_dn8: f64 = *var_slopegat_d_dn8_slot;
        let mut var_slopegat_d_dn9: f64 = *var_slopegat_d_dn9_slot;
        let mut var_slopesti_d: f64 = *var_slopesti_d_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_swgat2nd_d: f64 = *var_swgat2nd_d_slot;
        let mut var_vbirbotinv_d: f64 = *var_vbirbotinv_d_slot;
        let mut var_vbirgat2nd_d: f64 = *var_vbirgat2nd_d_slot;
        let mut var_vbirgatinv_d: f64 = *var_vbirgatinv_d_slot;
        let mut var_vbirstiinv_d: f64 = *var_vbirstiinv_d_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrinvbot_d: f64 = *var_vbrinvbot_d_slot;
        let mut var_vbrinvgat_d: f64 = *var_vbrinvgat_d_slot;
        let mut var_vbrinvgat_d_db0: f64 = *var_vbrinvgat_d_db0_slot;
        let mut var_vbrinvgat_d_db1: f64 = *var_vbrinvgat_d_db1_slot;
        let mut var_vbrinvgat_d_db2: f64 = *var_vbrinvgat_d_db2_slot;
        let mut var_vbrinvgat_d_db3: f64 = *var_vbrinvgat_d_db3_slot;
        let mut var_vbrinvgat_d_db4: f64 = *var_vbrinvgat_d_db4_slot;
        let mut var_vbrinvgat_d_db5: f64 = *var_vbrinvgat_d_db5_slot;
        let mut var_vbrinvgat_d_db6: f64 = *var_vbrinvgat_d_db6_slot;
        let mut var_vbrinvgat_d_dn0: f64 = *var_vbrinvgat_d_dn0_slot;
        let mut var_vbrinvgat_d_dn1: f64 = *var_vbrinvgat_d_dn1_slot;
        let mut var_vbrinvgat_d_dn10: f64 = *var_vbrinvgat_d_dn10_slot;
        let mut var_vbrinvgat_d_dn11: f64 = *var_vbrinvgat_d_dn11_slot;
        let mut var_vbrinvgat_d_dn12: f64 = *var_vbrinvgat_d_dn12_slot;
        let mut var_vbrinvgat_d_dn2: f64 = *var_vbrinvgat_d_dn2_slot;
        let mut var_vbrinvgat_d_dn3: f64 = *var_vbrinvgat_d_dn3_slot;
        let mut var_vbrinvgat_d_dn4: f64 = *var_vbrinvgat_d_dn4_slot;
        let mut var_vbrinvgat_d_dn5: f64 = *var_vbrinvgat_d_dn5_slot;
        let mut var_vbrinvgat_d_dn6: f64 = *var_vbrinvgat_d_dn6_slot;
        let mut var_vbrinvgat_d_dn7: f64 = *var_vbrinvgat_d_dn7_slot;
        let mut var_vbrinvgat_d_dn8: f64 = *var_vbrinvgat_d_dn8_slot;
        let mut var_vbrinvgat_d_dn9: f64 = *var_vbrinvgat_d_dn9_slot;
        let mut var_vbrinvsti_d: f64 = *var_vbrinvsti_d_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vjunrefd_i: f64 = *var_vjunrefd_i_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_wdepnulrbot_d: f64 = *var_wdepnulrbot_d_slot;
        let mut var_wdepnulrgat_d: f64 = *var_wdepnulrgat_d_slot;
        let mut var_wdepnulrinvbot_d: f64 = *var_wdepnulrinvbot_d_slot;
        let mut var_wdepnulrinvgat_d: f64 = *var_wdepnulrinvgat_d_slot;
        let mut var_wdepnulrinvsti_d: f64 = *var_wdepnulrinvsti_d_slot;
        let mut var_wdepnulrsti_d: f64 = *var_wdepnulrsti_d_slot;

        let (assign1300_e2116,) = {
    if (var_guard5 == 0.0) {
        (p.p913,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign1300_e2116;

        let (assign1310_e2121,) = {
    if (var_guard5 == 0.0) {
        (p.p914,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign1310_e2121;

        let (assign1320_e2126,) = {
    if (var_guard5 == 0.0) {
        (p.p915,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign1320_e2126;

        let (assign1330_e2131,) = {
    if (var_guard5 == 0.0) {
        (p.p916,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign1330_e2131;

        let (assign1340_e2136,) = {
    if (var_guard5 == 0.0) {
        (p.p917,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign1340_e2136;

        let (assign1350_e2141,) = {
    if (var_guard5 == 0.0) {
        (p.p918,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign1350_e2141;

        let (assign1360_e2146,) = {
    if (var_guard5 == 0.0) {
        (p.p919,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign1360_e2146;

        let (assign1370_e2151,) = {
    if (var_guard5 == 0.0) {
        (p.p920,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign1370_e2151;

        let (assign1380_e2156,) = {
    if (var_guard5 == 0.0) {
        (p.p921,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign1380_e2156;

        let (assign1390_e2161,) = {
    if (var_guard5 == 0.0) {
        (p.p922,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1390_e2161;

        let (assign1400_e2166,) = {
    if (var_guard5 == 0.0) {
        (p.p923,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1400_e2166;

        let (assign1410_e2171,) = {
    if (var_guard5 == 0.0) {
        (p.p924,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1410_e2171;

        let (assign1420_e2176,) = {
    if (var_guard5 == 0.0) {
        (p.p925,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1420_e2176;

        let (assign1430_e2181,) = {
    if (var_guard5 == 0.0) {
        (p.p926,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1430_e2181;

        let (assign1440_e2186,) = {
    if (var_guard5 == 0.0) {
        (p.p927,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1440_e2186;

        let (assign1450_e2191,) = {
    if (var_guard5 == 0.0) {
        (p.p928,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1450_e2191;

        let (assign1460_e2196,) = {
    if (var_guard5 == 0.0) {
        (p.p929,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1460_e2196;

        let (assign1470_e2201,) = {
    if (var_guard5 == 0.0) {
        (p.p930,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1470_e2201;

        let (assign1480_e2206,) = {
    if (var_guard5 == 0.0) {
        (p.p931,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1480_e2206;

        let (assign1490_e2211,) = {
    if (var_guard5 == 0.0) {
        (p.p932,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1490_e2211;

        let (assign1500_e2216,) = {
    if (var_guard5 == 0.0) {
        (p.p933,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1500_e2216;

        let (assign1510_e2221,) = {
    if (var_guard5 == 0.0) {
        (p.p947,)
    } else {
        (var_vjunrefd_i,)
    }
};
        var_vjunrefd_i = assign1510_e2221;

        let (assign1520_e2226,) = {
    if (var_guard5 == 0.0) {
        (p.p948,)
    } else {
        (s.v[554],)
    }
};
        s.store_scalar(554, assign1520_e2226);

        let (assign1530_e2231,) = {
    if (var_guard5 == 0.0) {
        (p.p940,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1530_e2231;

        let (assign1540_e2236,) = {
    if (var_guard5 == 0.0) {
        (p.p941,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1540_e2236;

        let (assign1550_e2241,) = {
    if (var_guard5 == 0.0) {
        (p.p942,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1550_e2241;

        let (assign1560_e2246,) = {
    if (var_guard5 == 0.0) {
        (p.p943,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1560_e2246;

        let (assign1570_e2251,) = {
    if (var_guard5 == 0.0) {
        (p.p934,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1570_e2251;

        let (assign1580_e2256,) = {
    if (var_guard5 == 0.0) {
        (p.p935,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1580_e2256;

        let (assign1590_e2261,) = {
    if (var_guard5 == 0.0) {
        (p.p936,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1590_e2261;

        let (assign1600_e2266,) = {
    if (var_guard5 == 0.0) {
        (p.p937,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1600_e2266;

        let (assign1610_e2271,) = {
    if (var_guard5 == 0.0) {
        (p.p938,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1610_e2271;

        let (assign1620_e2276,) = {
    if (var_guard5 == 0.0) {
        (p.p939,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1620_e2276;

        let assign1630_e2279: f64 = (var_phigbotd_i + var_deltaphigr);
        var_phigrbot_d = assign1630_e2279;

        let assign1640_e2282: f64 = (var_phigstid_i + var_deltaphigr);
        var_phigrsti_d = assign1640_e2282;

        let assign1650_e2285: f64 = (var_phiggatd_i + var_deltaphigr);
        var_phigrgat_d = assign1650_e2285;

        let assign1660_e2288: f64 = (1.0 - var_pbotd_i);
        var_one_minus_pbot_d = assign1660_e2288;

        let assign1670_e2291: f64 = (1.0 - var_pstid_i);
        var_one_minus_psti_d = assign1670_e2291;

        let assign1680_e2294: f64 = (1.0 - var_pgatd_i);
        var_one_minus_pgat_d = assign1680_e2294;

        let assign1690_e2297: f64 = (1.0 / var_one_minus_pbot_d);
        var_one_over_one_minus_pbot_d = assign1690_e2297;

        let assign1700_e2300: f64 = (1.0 / var_one_minus_psti_d);
        var_one_over_one_minus_psti_d = assign1700_e2300;

        let assign1710_e2303: f64 = (1.0 / var_one_minus_pgat_d);
        var_one_over_one_minus_pgat_d = assign1710_e2303;

        let assign1720_e2306: f64 = (var_epssi / var_cjorbotd_i);
        var_wdepnulrbot_d = assign1720_e2306;

        let assign1730_e2309: f64 = (var_xjunstid_i * var_epssi);
        let assign1730_e2311: f64 = (assign1730_e2309 / var_cjorstid_i);
        var_wdepnulrsti_d = assign1730_e2311;

        let assign1740_e2314: f64 = (var_xjungatd_i * var_epssi);
        let assign1740_e2316: f64 = (assign1740_e2314 / var_cjorgatd_i);
        var_wdepnulrgat_d = assign1740_e2316;

        let assign1750_e2319: f64 = (1.0 / var_wdepnulrbot_d);
        var_wdepnulrinvbot_d = assign1750_e2319;

        let assign1760_e2322: f64 = (1.0 / var_wdepnulrsti_d);
        var_wdepnulrinvsti_d = assign1760_e2322;

        let assign1770_e2325: f64 = (1.0 / var_wdepnulrgat_d);
        var_wdepnulrinvgat_d = assign1770_e2325;

        let assign1780_e2328: f64 = (1.0 / var_vbirbotd_i);
        var_vbirbotinv_d = assign1780_e2328;

        let assign1790_e2331: f64 = (1.0 / var_vbirstid_i);
        var_vbirstiinv_d = assign1790_e2331;

        let assign1800_e2334: f64 = (1.0 / var_vbirgatd_i);
        var_vbirgatinv_d = assign1800_e2334;

        let assign1810_e2339: f64 = (var_alphaav).powf(var_pbrbotd_i);
        let assign1810_e2340: f64 = (1.0 - assign1810_e2339);
        let assign1810_e2341: f64 = (1.0 / assign1810_e2340);
        var_fstopbot_d = assign1810_e2341;

        let assign1820_e2346: f64 = (var_alphaav).powf(var_pbrstid_i);
        let assign1820_e2347: f64 = (1.0 - assign1820_e2346);
        let assign1820_e2348: f64 = (1.0 / assign1820_e2347);
        var_fstopsti_d = assign1820_e2348;

        let assign1830_e2353: f64 = (var_alphaav).powf(var_pbrgatd_i);
        let assign1830_e2354: f64 = (1.0 - assign1830_e2353);
        let assign1830_e2355: f64 = (1.0 / assign1830_e2354);
        var_fstopgat_d = assign1830_e2355;

        let assign1840_e2358: f64 = (1.0 / var_vbrbotd_i);
        var_vbrinvbot_d = assign1840_e2358;

        let assign1850_e2361: f64 = (1.0 / var_vbrstid_i);
        var_vbrinvsti_d = assign1850_e2361;

        let assign1860_e2364: f64 = (1.0 / var_vbrgatd_i);
        var_vbrinvgat_d = assign1860_e2364;
        var_vbrinvgat_d_dn0 = 0.0;
        var_vbrinvgat_d_dn1 = 0.0;
        var_vbrinvgat_d_dn2 = 0.0;
        var_vbrinvgat_d_dn3 = 0.0;
        var_vbrinvgat_d_dn4 = 0.0;
        var_vbrinvgat_d_dn5 = 0.0;
        var_vbrinvgat_d_dn6 = 0.0;
        var_vbrinvgat_d_dn7 = 0.0;
        var_vbrinvgat_d_dn8 = 0.0;
        var_vbrinvgat_d_dn9 = 0.0;
        var_vbrinvgat_d_dn10 = 0.0;
        var_vbrinvgat_d_dn11 = 0.0;
        var_vbrinvgat_d_dn12 = 0.0;
        var_vbrinvgat_d_db0 = 0.0;
        var_vbrinvgat_d_db1 = 0.0;
        var_vbrinvgat_d_db2 = 0.0;
        var_vbrinvgat_d_db3 = 0.0;
        var_vbrinvgat_d_db4 = 0.0;
        var_vbrinvgat_d_db5 = 0.0;
        var_vbrinvgat_d_db6 = 0.0;

        let assign1870_e2367: f64 = (var_fstopbot_d * var_fstopbot_d);
        let assign1870_e2371: f64 = (var_pbrbotd_i - 1.0);
        let assign1870_e2372: f64 = (var_alphaav).powf(assign1870_e2371);
        let assign1870_e2373: f64 = (assign1870_e2367 * assign1870_e2372);
        let assign1870_e2374: f64 = (-assign1870_e2373);
        let assign1870_e2376: f64 = (assign1870_e2374 * var_pbrbotd_i);
        let assign1870_e2378: f64 = (assign1870_e2376 * var_vbrinvbot_d);
        var_slopebot_d = assign1870_e2378;

        let assign1880_e2381: f64 = (var_fstopsti_d * var_fstopsti_d);
        let assign1880_e2385: f64 = (var_pbrstid_i - 1.0);
        let assign1880_e2386: f64 = (var_alphaav).powf(assign1880_e2385);
        let assign1880_e2387: f64 = (assign1880_e2381 * assign1880_e2386);
        let assign1880_e2388: f64 = (-assign1880_e2387);
        let assign1880_e2390: f64 = (assign1880_e2388 * var_pbrstid_i);
        let assign1880_e2392: f64 = (assign1880_e2390 * var_vbrinvsti_d);
        var_slopesti_d = assign1880_e2392;

        let assign1890_e2395: f64 = (var_fstopgat_d * var_fstopgat_d);
        let assign1890_e2399: f64 = (var_pbrgatd_i - 1.0);
        let assign1890_e2400: f64 = (var_alphaav).powf(assign1890_e2399);
        let assign1890_e2401: f64 = (assign1890_e2395 * assign1890_e2400);
        let assign1890_e2402: f64 = (-assign1890_e2401);
        let assign1890_e2404: f64 = (assign1890_e2402 * var_pbrgatd_i);
        let assign1890_e2406: f64 = (assign1890_e2404 * var_vbrinvgat_d);
        var_slopegat_d = assign1890_e2406;
        var_slopegat_d_dn0 = (assign1890_e2404 * var_vbrinvgat_d_dn0);
        var_slopegat_d_dn1 = (assign1890_e2404 * var_vbrinvgat_d_dn1);
        var_slopegat_d_dn2 = (assign1890_e2404 * var_vbrinvgat_d_dn2);
        var_slopegat_d_dn3 = (assign1890_e2404 * var_vbrinvgat_d_dn3);
        var_slopegat_d_dn4 = (assign1890_e2404 * var_vbrinvgat_d_dn4);
        var_slopegat_d_dn5 = (assign1890_e2404 * var_vbrinvgat_d_dn5);
        var_slopegat_d_dn6 = (assign1890_e2404 * var_vbrinvgat_d_dn6);
        var_slopegat_d_dn7 = (assign1890_e2404 * var_vbrinvgat_d_dn7);
        var_slopegat_d_dn8 = (assign1890_e2404 * var_vbrinvgat_d_dn8);
        var_slopegat_d_dn9 = (assign1890_e2404 * var_vbrinvgat_d_dn9);
        var_slopegat_d_dn10 = (assign1890_e2404 * var_vbrinvgat_d_dn10);
        var_slopegat_d_dn11 = (assign1890_e2404 * var_vbrinvgat_d_dn11);
        var_slopegat_d_dn12 = (assign1890_e2404 * var_vbrinvgat_d_dn12);
        var_slopegat_d_db0 = (assign1890_e2404 * var_vbrinvgat_d_db0);
        var_slopegat_d_db1 = (assign1890_e2404 * var_vbrinvgat_d_db1);
        var_slopegat_d_db2 = (assign1890_e2404 * var_vbrinvgat_d_db2);
        var_slopegat_d_db3 = (assign1890_e2404 * var_vbrinvgat_d_db3);
        var_slopegat_d_db4 = (assign1890_e2404 * var_vbrinvgat_d_db4);
        var_slopegat_d_db5 = (assign1890_e2404 * var_vbrinvgat_d_db5);
        var_slopegat_d_db6 = (assign1890_e2404 * var_vbrinvgat_d_db6);

        let assign1900_e2421: f64 = if ((((var_fcjorgat2d_i != 1.0) || (var_fvbirgat2d_i != 1.0)) || (var_fpgat2d_i != 1.0)) || (var_fphiggat2d_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard6 = assign1900_e2421;

        let (assign1910_e2425,) = {
    if (var_guard6 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign1910_e2425;

        let (assign1920_e2430,) = {
    if (var_guard6 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign1920_e2430;

        let assign1930_e2433: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard7 = assign1930_e2433;

        if (s.v[987] != 0.0) {
            s.store_scalar(621, (if ((s.v[508] * s.v[547]) > 1e-18) { (var_cjorgatd_i * var_fcjorgat2d_i) } else { 1e-18 }));
        }

        let (assign1950_e2459,) = {
    if (var_guard7 != 0.0) {
        let assign1950_e2450: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
        let (assign1950_e2457,) = {
            if (assign1950_e2450 > 0.05) {
                let assign1950_e2455: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
                (assign1950_e2455,)
            } else {
                (0.05,)
            }
        };
        (assign1950_e2457,)
    } else {
        (var_vbirgat2nd_d,)
    }
};
        var_vbirgat2nd_d = assign1950_e2459;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fstopbot_d_slot = var_fstopbot_d;
        *var_fstopgat_d_slot = var_fstopgat_d;
        *var_fstopsti_d_slot = var_fstopsti_d;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_guard6_slot = var_guard6;
        *var_guard7_slot = var_guard7;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_one_minus_pbot_d_slot = var_one_minus_pbot_d;
        *var_one_minus_pgat_d_slot = var_one_minus_pgat_d;
        *var_one_minus_psti_d_slot = var_one_minus_psti_d;
        *var_one_over_one_minus_pbot_d_slot = var_one_over_one_minus_pbot_d;
        *var_one_over_one_minus_pgat_d_slot = var_one_over_one_minus_pgat_d;
        *var_one_over_one_minus_psti_d_slot = var_one_over_one_minus_psti_d;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_phigrbot_d_slot = var_phigrbot_d;
        *var_phigrgat_d_slot = var_phigrgat_d;
        *var_phigrsti_d_slot = var_phigrsti_d;
        *var_slopebot_d_slot = var_slopebot_d;
        *var_slopegat_d_slot = var_slopegat_d;
        *var_slopegat_d_db0_slot = var_slopegat_d_db0;
        *var_slopegat_d_db1_slot = var_slopegat_d_db1;
        *var_slopegat_d_db2_slot = var_slopegat_d_db2;
        *var_slopegat_d_db3_slot = var_slopegat_d_db3;
        *var_slopegat_d_db4_slot = var_slopegat_d_db4;
        *var_slopegat_d_db5_slot = var_slopegat_d_db5;
        *var_slopegat_d_db6_slot = var_slopegat_d_db6;
        *var_slopegat_d_dn0_slot = var_slopegat_d_dn0;
        *var_slopegat_d_dn1_slot = var_slopegat_d_dn1;
        *var_slopegat_d_dn10_slot = var_slopegat_d_dn10;
        *var_slopegat_d_dn11_slot = var_slopegat_d_dn11;
        *var_slopegat_d_dn12_slot = var_slopegat_d_dn12;
        *var_slopegat_d_dn2_slot = var_slopegat_d_dn2;
        *var_slopegat_d_dn3_slot = var_slopegat_d_dn3;
        *var_slopegat_d_dn4_slot = var_slopegat_d_dn4;
        *var_slopegat_d_dn5_slot = var_slopegat_d_dn5;
        *var_slopegat_d_dn6_slot = var_slopegat_d_dn6;
        *var_slopegat_d_dn7_slot = var_slopegat_d_dn7;
        *var_slopegat_d_dn8_slot = var_slopegat_d_dn8;
        *var_slopegat_d_dn9_slot = var_slopegat_d_dn9;
        *var_slopesti_d_slot = var_slopesti_d;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_swgat2nd_d_slot = var_swgat2nd_d;
        *var_vbirbotinv_d_slot = var_vbirbotinv_d;
        *var_vbirgat2nd_d_slot = var_vbirgat2nd_d;
        *var_vbirgatinv_d_slot = var_vbirgatinv_d;
        *var_vbirstiinv_d_slot = var_vbirstiinv_d;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrinvbot_d_slot = var_vbrinvbot_d;
        *var_vbrinvgat_d_slot = var_vbrinvgat_d;
        *var_vbrinvgat_d_db0_slot = var_vbrinvgat_d_db0;
        *var_vbrinvgat_d_db1_slot = var_vbrinvgat_d_db1;
        *var_vbrinvgat_d_db2_slot = var_vbrinvgat_d_db2;
        *var_vbrinvgat_d_db3_slot = var_vbrinvgat_d_db3;
        *var_vbrinvgat_d_db4_slot = var_vbrinvgat_d_db4;
        *var_vbrinvgat_d_db5_slot = var_vbrinvgat_d_db5;
        *var_vbrinvgat_d_db6_slot = var_vbrinvgat_d_db6;
        *var_vbrinvgat_d_dn0_slot = var_vbrinvgat_d_dn0;
        *var_vbrinvgat_d_dn1_slot = var_vbrinvgat_d_dn1;
        *var_vbrinvgat_d_dn10_slot = var_vbrinvgat_d_dn10;
        *var_vbrinvgat_d_dn11_slot = var_vbrinvgat_d_dn11;
        *var_vbrinvgat_d_dn12_slot = var_vbrinvgat_d_dn12;
        *var_vbrinvgat_d_dn2_slot = var_vbrinvgat_d_dn2;
        *var_vbrinvgat_d_dn3_slot = var_vbrinvgat_d_dn3;
        *var_vbrinvgat_d_dn4_slot = var_vbrinvgat_d_dn4;
        *var_vbrinvgat_d_dn5_slot = var_vbrinvgat_d_dn5;
        *var_vbrinvgat_d_dn6_slot = var_vbrinvgat_d_dn6;
        *var_vbrinvgat_d_dn7_slot = var_vbrinvgat_d_dn7;
        *var_vbrinvgat_d_dn8_slot = var_vbrinvgat_d_dn8;
        *var_vbrinvgat_d_dn9_slot = var_vbrinvgat_d_dn9;
        *var_vbrinvsti_d_slot = var_vbrinvsti_d;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vjunrefd_i_slot = var_vjunrefd_i;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_wdepnulrbot_d_slot = var_wdepnulrbot_d;
        *var_wdepnulrgat_d_slot = var_wdepnulrgat_d;
        *var_wdepnulrinvbot_d_slot = var_wdepnulrinvbot_d;
        *var_wdepnulrinvgat_d_slot = var_wdepnulrinvgat_d;
        *var_wdepnulrinvsti_d_slot = var_wdepnulrinvsti_d;
        *var_wdepnulrsti_d_slot = var_wdepnulrsti_d;
    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        var_deltaphigr: f64,
        var_fpgat2d_i: f64,
        var_fphiggat2d_i: f64,
        var_guard7: f64,
        var_idsatrbotd_i: f64,
        var_idsatrgatd_i: f64,
        var_idsatrstid_i: f64,
        var_kbol_over_qele: f64,
        var_one_over_one_minus_pbot: f64,
        var_one_over_one_minus_pgat: f64,
        var_one_over_one_minus_psti: f64,
        var_pgat2nd: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggat2nd: f64,
        var_phiggatd_i: f64,
        var_phigrbot: f64,
        var_phigrbot_d: f64,
        var_phigrgat: f64,
        var_phigrgat2nd: f64,
        var_phigrgat_d: f64,
        var_phigrsti: f64,
        var_phigrsti_d: f64,
        var_phigstid_i: f64,
        var_phitrinv: f64,
        var_swgat2nd: f64,
        var_tkr: f64,
        var_tkr_1: f64,
        var_vbirbotd_i: f64,
        var_vbirgat2nd: f64,
        var_atatbot_slot: &mut f64,
        var_atatgat_slot: &mut f64,
        var_atatsti_slot: &mut f64,
        var_auxt_slot: &mut f64,
        var_btatpartbot_slot: &mut f64,
        var_btatpartgat_slot: &mut f64,
        var_btatpartsti_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_deltaebot_slot: &mut f64,
        var_deltaegat_slot: &mut f64,
        var_deltaesti_slot: &mut f64,
        var_deltaphigd_slot: &mut f64,
        var_fbbtbot_slot: &mut f64,
        var_fbbtgat_slot: &mut f64,
        var_fbbtgat_db0_slot: &mut f64,
        var_fbbtgat_db1_slot: &mut f64,
        var_fbbtgat_db2_slot: &mut f64,
        var_fbbtgat_db3_slot: &mut f64,
        var_fbbtgat_db4_slot: &mut f64,
        var_fbbtgat_db5_slot: &mut f64,
        var_fbbtgat_db6_slot: &mut f64,
        var_fbbtgat_dn0_slot: &mut f64,
        var_fbbtgat_dn1_slot: &mut f64,
        var_fbbtgat_dn10_slot: &mut f64,
        var_fbbtgat_dn11_slot: &mut f64,
        var_fbbtgat_dn12_slot: &mut f64,
        var_fbbtgat_dn2_slot: &mut f64,
        var_fbbtgat_dn3_slot: &mut f64,
        var_fbbtgat_dn4_slot: &mut f64,
        var_fbbtgat_dn5_slot: &mut f64,
        var_fbbtgat_dn6_slot: &mut f64,
        var_fbbtgat_dn7_slot: &mut f64,
        var_fbbtgat_dn8_slot: &mut f64,
        var_fbbtgat_dn9_slot: &mut f64,
        var_fbbtsti_slot: &mut f64,
        var_ftdbot_slot: &mut f64,
        var_ftdbot_d_slot: &mut f64,
        var_ftdgat_slot: &mut f64,
        var_ftdgat2nd_slot: &mut f64,
        var_ftdgat_d_slot: &mut f64,
        var_ftdsti_slot: &mut f64,
        var_ftdsti_d_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_idsatbot_slot: &mut f64,
        var_idsatbot_d_slot: &mut f64,
        var_idsatgat_slot: &mut f64,
        var_idsatgat_d_slot: &mut f64,
        var_idsatsti_slot: &mut f64,
        var_idsatsti_d_slot: &mut f64,
        var_inv_phita_slot: &mut f64,
        var_one_minus_pgat2nd_d_slot: &mut f64,
        var_pgat2nd_d_slot: &mut f64,
        var_phigdbot_slot: &mut f64,
        var_phigdbot_d_slot: &mut f64,
        var_phigdgat_slot: &mut f64,
        var_phigdgat2nd_slot: &mut f64,
        var_phigdgat_d_slot: &mut f64,
        var_phigdsti_slot: &mut f64,
        var_phigdsti_d_slot: &mut f64,
        var_phiggat2nd_d_slot: &mut f64,
        var_phigrgat2nd_d_slot: &mut f64,
        var_phita_slot: &mut f64,
        var_phitd_slot: &mut f64,
        var_phitdinv_slot: &mut f64,
        var_rta_slot: &mut f64,
        var_tka_slot: &mut f64,
        var_tkd_1_slot: &mut f64,
        var_ubibot_slot: &mut f64,
        var_ubibot_d_slot: &mut f64,
        var_ubigat_slot: &mut f64,
        var_ubigat2nd_slot: &mut f64,
        var_ubisti_slot: &mut f64,
        var_vbibot_slot: &mut f64,
        var_vbigat_slot: &mut f64,
        var_vbigat2nd_slot: &mut f64,
        var_vbiinvbot_slot: &mut f64,
        var_vbiinvgat_slot: &mut f64,
        var_vbiinvgat2nd_slot: &mut f64,
        var_vbiinvsti_slot: &mut f64,
        var_vbisti_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_atatbot: f64 = *var_atatbot_slot;
        let mut var_atatgat: f64 = *var_atatgat_slot;
        let mut var_atatsti: f64 = *var_atatsti_slot;
        let mut var_auxt: f64 = *var_auxt_slot;
        let mut var_btatpartbot: f64 = *var_btatpartbot_slot;
        let mut var_btatpartgat: f64 = *var_btatpartgat_slot;
        let mut var_btatpartsti: f64 = *var_btatpartsti_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_deltaebot: f64 = *var_deltaebot_slot;
        let mut var_deltaegat: f64 = *var_deltaegat_slot;
        let mut var_deltaesti: f64 = *var_deltaesti_slot;
        let mut var_deltaphigd: f64 = *var_deltaphigd_slot;
        let mut var_fbbtbot: f64 = *var_fbbtbot_slot;
        let mut var_fbbtgat: f64 = *var_fbbtgat_slot;
        let mut var_fbbtgat_db0: f64 = *var_fbbtgat_db0_slot;
        let mut var_fbbtgat_db1: f64 = *var_fbbtgat_db1_slot;
        let mut var_fbbtgat_db2: f64 = *var_fbbtgat_db2_slot;
        let mut var_fbbtgat_db3: f64 = *var_fbbtgat_db3_slot;
        let mut var_fbbtgat_db4: f64 = *var_fbbtgat_db4_slot;
        let mut var_fbbtgat_db5: f64 = *var_fbbtgat_db5_slot;
        let mut var_fbbtgat_db6: f64 = *var_fbbtgat_db6_slot;
        let mut var_fbbtgat_dn0: f64 = *var_fbbtgat_dn0_slot;
        let mut var_fbbtgat_dn1: f64 = *var_fbbtgat_dn1_slot;
        let mut var_fbbtgat_dn10: f64 = *var_fbbtgat_dn10_slot;
        let mut var_fbbtgat_dn11: f64 = *var_fbbtgat_dn11_slot;
        let mut var_fbbtgat_dn12: f64 = *var_fbbtgat_dn12_slot;
        let mut var_fbbtgat_dn2: f64 = *var_fbbtgat_dn2_slot;
        let mut var_fbbtgat_dn3: f64 = *var_fbbtgat_dn3_slot;
        let mut var_fbbtgat_dn4: f64 = *var_fbbtgat_dn4_slot;
        let mut var_fbbtgat_dn5: f64 = *var_fbbtgat_dn5_slot;
        let mut var_fbbtgat_dn6: f64 = *var_fbbtgat_dn6_slot;
        let mut var_fbbtgat_dn7: f64 = *var_fbbtgat_dn7_slot;
        let mut var_fbbtgat_dn8: f64 = *var_fbbtgat_dn8_slot;
        let mut var_fbbtgat_dn9: f64 = *var_fbbtgat_dn9_slot;
        let mut var_fbbtsti: f64 = *var_fbbtsti_slot;
        let mut var_ftdbot: f64 = *var_ftdbot_slot;
        let mut var_ftdbot_d: f64 = *var_ftdbot_d_slot;
        let mut var_ftdgat: f64 = *var_ftdgat_slot;
        let mut var_ftdgat2nd: f64 = *var_ftdgat2nd_slot;
        let mut var_ftdgat_d: f64 = *var_ftdgat_d_slot;
        let mut var_ftdsti: f64 = *var_ftdsti_slot;
        let mut var_ftdsti_d: f64 = *var_ftdsti_d_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_idsatbot: f64 = *var_idsatbot_slot;
        let mut var_idsatbot_d: f64 = *var_idsatbot_d_slot;
        let mut var_idsatgat: f64 = *var_idsatgat_slot;
        let mut var_idsatgat_d: f64 = *var_idsatgat_d_slot;
        let mut var_idsatsti: f64 = *var_idsatsti_slot;
        let mut var_idsatsti_d: f64 = *var_idsatsti_d_slot;
        let mut var_inv_phita: f64 = *var_inv_phita_slot;
        let mut var_one_minus_pgat2nd_d: f64 = *var_one_minus_pgat2nd_d_slot;
        let mut var_pgat2nd_d: f64 = *var_pgat2nd_d_slot;
        let mut var_phigdbot: f64 = *var_phigdbot_slot;
        let mut var_phigdbot_d: f64 = *var_phigdbot_d_slot;
        let mut var_phigdgat: f64 = *var_phigdgat_slot;
        let mut var_phigdgat2nd: f64 = *var_phigdgat2nd_slot;
        let mut var_phigdgat_d: f64 = *var_phigdgat_d_slot;
        let mut var_phigdsti: f64 = *var_phigdsti_slot;
        let mut var_phigdsti_d: f64 = *var_phigdsti_d_slot;
        let mut var_phiggat2nd_d: f64 = *var_phiggat2nd_d_slot;
        let mut var_phigrgat2nd_d: f64 = *var_phigrgat2nd_d_slot;
        let mut var_phita: f64 = *var_phita_slot;
        let mut var_phitd: f64 = *var_phitd_slot;
        let mut var_phitdinv: f64 = *var_phitdinv_slot;
        let mut var_rta: f64 = *var_rta_slot;
        let mut var_tka: f64 = *var_tka_slot;
        let mut var_tkd_1: f64 = *var_tkd_1_slot;
        let mut var_ubibot: f64 = *var_ubibot_slot;
        let mut var_ubibot_d: f64 = *var_ubibot_d_slot;
        let mut var_ubigat: f64 = *var_ubigat_slot;
        let mut var_ubigat2nd: f64 = *var_ubigat2nd_slot;
        let mut var_ubisti: f64 = *var_ubisti_slot;
        let mut var_vbibot: f64 = *var_vbibot_slot;
        let mut var_vbigat: f64 = *var_vbigat_slot;
        let mut var_vbigat2nd: f64 = *var_vbigat2nd_slot;
        let mut var_vbiinvbot: f64 = *var_vbiinvbot_slot;
        let mut var_vbiinvgat: f64 = *var_vbiinvgat_slot;
        let mut var_vbiinvgat2nd: f64 = *var_vbiinvgat2nd_slot;
        let mut var_vbiinvsti: f64 = *var_vbiinvsti_slot;
        let mut var_vbisti: f64 = *var_vbisti_slot;

        let (assign1960_e2486,) = {
    if (var_guard7 != 0.0) {
        let assign1960_e2463: f64 = (var_pgatd_i * var_fpgat2d_i);
        let (assign1960_e2470,) = {
            if (assign1960_e2463 > 0.05) {
                let assign1960_e2468: f64 = (var_pgatd_i * var_fpgat2d_i);
                (assign1960_e2468,)
            } else {
                (0.05,)
            }
        };
        let (assign1960_e2484,) = {
            if (assign1960_e2470 < 0.95) {
                let assign1960_e2475: f64 = (var_pgatd_i * var_fpgat2d_i);
                let (assign1960_e2482,) = {
                    if (assign1960_e2475 > 0.05) {
                        let assign1960_e2480: f64 = (var_pgatd_i * var_fpgat2d_i);
                        (assign1960_e2480,)
                    } else {
                        (0.05,)
                    }
                };
                (assign1960_e2482,)
            } else {
                (0.95,)
            }
        };
        (assign1960_e2484,)
    } else {
        (var_pgat2nd_d,)
    }
};
        var_pgat2nd_d = assign1960_e2486;

        let (assign1970_e2492,) = {
    if (var_guard7 != 0.0) {
        let assign1970_e2490: f64 = (var_phiggatd_i * var_fphiggat2d_i);
        (assign1970_e2490,)
    } else {
        (var_phiggat2nd_d,)
    }
};
        var_phiggat2nd_d = assign1970_e2492;

        let (assign1980_e2498,) = {
    if (var_guard7 != 0.0) {
        let assign1980_e2496: f64 = (var_phiggat2nd_d + var_deltaphigr);
        (assign1980_e2496,)
    } else {
        (var_phigrgat2nd_d,)
    }
};
        var_phigrgat2nd_d = assign1980_e2498;

        let (assign1990_e2504,) = {
    if (var_guard7 != 0.0) {
        let assign1990_e2502: f64 = (1.0 - var_pgat2nd_d);
        (assign1990_e2502,)
    } else {
        (var_one_minus_pgat2nd_d,)
    }
};
        var_one_minus_pgat2nd_d = assign1990_e2504;

        if (s.v[987] != 0.0) {
            s.store_scalar(632, (1.0 / var_one_minus_pgat2nd_d));
        }

        let assign2050_e2532: f64 = ctx_temp;
        let assign2050_e2534: f64 = (assign2050_e2532 + p.p55);
        let assign2050_e2536: f64 = (assign2050_e2534 + p.p35);
        var_tka = assign2050_e2536;

        let assign2060_e2539: f64 = (var_tka / var_tkr);
        var_rta = assign2060_e2539;

        let assign2070_e2542: f64 = (var_tka - var_tkr);
        var_delta = assign2070_e2542;

        let assign2080_e2545: f64 = (var_tka * 1.3806505e-23);
        let assign2080_e2547: f64 = (assign2080_e2545 / 1.6021918e-19);
        var_phita = assign2080_e2547;

        let assign2090_e2550: f64 = (1.0 / var_phita);
        var_inv_phita = assign2090_e2550;

        let assign2100_e2551: f64 = ctx_temp;
        let assign2100_e2553: f64 = (assign2100_e2551 + p.p55);
        let assign2100_e2555: f64 = (assign2100_e2553 + p.p35);
        let assign2100_e2558: f64 = (-250.0);
        let assign2100_e2559: f64 = (273.15 + assign2100_e2558);
        let assign2100_e2560: f64 = (assign2100_e2555).max(assign2100_e2559);
        var_tkd_1 = assign2100_e2560;

        let assign2110_e2563: f64 = (var_tkd_1 / var_tkr_1);
        var_auxt = assign2110_e2563;

        let assign2120_e2566: f64 = (var_kbol_over_qele * var_tkd_1);
        var_phitd = assign2120_e2566;

        let assign2130_e2569: f64 = (1.0 / var_phitd);
        var_phitdinv = assign2130_e2569;

        let assign2140_e2572: f64 = (0.000702 * var_tkd_1);
        let assign2140_e2574: f64 = (assign2140_e2572 * var_tkd_1);
        let assign2140_e2575: f64 = (-assign2140_e2574);
        let assign2140_e2578: f64 = (1108.0 + var_tkd_1);
        let assign2140_e2579: f64 = (assign2140_e2575 / assign2140_e2578);
        var_deltaphigd = assign2140_e2579;

        let assign2150_e2582: f64 = (p.p851 + var_deltaphigd);
        var_phigdbot = assign2150_e2582;

        let assign2160_e2585: f64 = (p.p852 + var_deltaphigd);
        var_phigdsti = assign2160_e2585;

        let assign2170_e2588: f64 = (p.p853 + var_deltaphigd);
        var_phigdgat = assign2170_e2588;

        let assign2180_e2591: f64 = (var_auxt).powf(1.5);
        let assign2180_e2595: f64 = (var_phigrbot * var_phitrinv);
        let assign2180_e2598: f64 = (var_phigdbot * var_phitdinv);
        let assign2180_e2599: f64 = (assign2180_e2595 - assign2180_e2598);
        let assign2180_e2600: f64 = (0.5 * assign2180_e2599);
        let assign2180_e2601: f64 = (assign2180_e2600).exp();
        let assign2180_e2602: f64 = (assign2180_e2591 * assign2180_e2601);
        var_ftdbot = assign2180_e2602;

        let assign2190_e2605: f64 = (var_auxt).powf(1.5);
        let assign2190_e2609: f64 = (var_phigrsti * var_phitrinv);
        let assign2190_e2612: f64 = (var_phigdsti * var_phitdinv);
        let assign2190_e2613: f64 = (assign2190_e2609 - assign2190_e2612);
        let assign2190_e2614: f64 = (0.5 * assign2190_e2613);
        let assign2190_e2615: f64 = (assign2190_e2614).exp();
        let assign2190_e2616: f64 = (assign2190_e2605 * assign2190_e2615);
        var_ftdsti = assign2190_e2616;

        let assign2200_e2619: f64 = (var_auxt).powf(1.5);
        let assign2200_e2623: f64 = (var_phigrgat * var_phitrinv);
        let assign2200_e2626: f64 = (var_phigdgat * var_phitdinv);
        let assign2200_e2627: f64 = (assign2200_e2623 - assign2200_e2626);
        let assign2200_e2628: f64 = (0.5 * assign2200_e2627);
        let assign2200_e2629: f64 = (assign2200_e2628).exp();
        let assign2200_e2630: f64 = (assign2200_e2619 * assign2200_e2629);
        var_ftdgat = assign2200_e2630;

        let assign2210_e2633: f64 = (p.p854 * var_ftdbot);
        let assign2210_e2635: f64 = (assign2210_e2633 * var_ftdbot);
        var_idsatbot = assign2210_e2635;

        let assign2220_e2638: f64 = (p.p855 * var_ftdsti);
        let assign2220_e2640: f64 = (assign2220_e2638 * var_ftdsti);
        var_idsatsti = assign2220_e2640;

        let assign2230_e2643: f64 = (p.p856 * var_ftdgat);
        let assign2230_e2645: f64 = (assign2230_e2643 * var_ftdgat);
        var_idsatgat = assign2230_e2645;

        let assign2240_e2648: f64 = (p.p845 * var_auxt);
        let assign2240_e2651: f64 = (2.0 * var_phitd);
        let assign2240_e2653: f64 = (var_ftdbot).ln();
        let assign2240_e2654: f64 = (assign2240_e2651 * assign2240_e2653);
        let assign2240_e2655: f64 = (assign2240_e2648 - assign2240_e2654);
        var_ubibot = assign2240_e2655;

        let assign2250_e2658: f64 = (p.p846 * var_auxt);
        let assign2250_e2661: f64 = (2.0 * var_phitd);
        let assign2250_e2663: f64 = (var_ftdsti).ln();
        let assign2250_e2664: f64 = (assign2250_e2661 * assign2250_e2663);
        let assign2250_e2665: f64 = (assign2250_e2658 - assign2250_e2664);
        var_ubisti = assign2250_e2665;

        let assign2260_e2668: f64 = (p.p847 * var_auxt);
        let assign2260_e2671: f64 = (2.0 * var_phitd);
        let assign2260_e2673: f64 = (var_ftdgat).ln();
        let assign2260_e2674: f64 = (assign2260_e2671 * assign2260_e2673);
        let assign2260_e2675: f64 = (assign2260_e2668 - assign2260_e2674);
        var_ubigat = assign2260_e2675;

        let assign2270_e2681: f64 = (0.05 - var_ubibot);
        let assign2270_e2683: f64 = (assign2270_e2681 * var_phitdinv);
        let assign2270_e2684: f64 = (assign2270_e2683).exp();
        let assign2270_e2685: f64 = (1.0 + assign2270_e2684);
        let assign2270_e2686: f64 = (assign2270_e2685).ln();
        let assign2270_e2687: f64 = (var_phitd * assign2270_e2686);
        let assign2270_e2688: f64 = (var_ubibot + assign2270_e2687);
        var_vbibot = assign2270_e2688;

        let assign2280_e2694: f64 = (0.05 - var_ubisti);
        let assign2280_e2696: f64 = (assign2280_e2694 * var_phitdinv);
        let assign2280_e2697: f64 = (assign2280_e2696).exp();
        let assign2280_e2698: f64 = (1.0 + assign2280_e2697);
        let assign2280_e2699: f64 = (assign2280_e2698).ln();
        let assign2280_e2700: f64 = (var_phitd * assign2280_e2699);
        let assign2280_e2701: f64 = (var_ubisti + assign2280_e2700);
        var_vbisti = assign2280_e2701;

        let assign2290_e2707: f64 = (0.05 - var_ubigat);
        let assign2290_e2709: f64 = (assign2290_e2707 * var_phitdinv);
        let assign2290_e2710: f64 = (assign2290_e2709).exp();
        let assign2290_e2711: f64 = (1.0 + assign2290_e2710);
        let assign2290_e2712: f64 = (assign2290_e2711).ln();
        let assign2290_e2713: f64 = (var_phitd * assign2290_e2712);
        let assign2290_e2714: f64 = (var_ubigat + assign2290_e2713);
        var_vbigat = assign2290_e2714;

        let assign2300_e2717: f64 = (1.0 / var_vbibot);
        var_vbiinvbot = assign2300_e2717;

        let assign2310_e2720: f64 = (1.0 / var_vbisti);
        var_vbiinvsti = assign2310_e2720;

        let assign2320_e2723: f64 = (1.0 / var_vbigat);
        var_vbiinvgat = assign2320_e2723;

        s.store_scalar(415, (p.p842 * (((p.p845 * var_vbiinvbot)) as f64).powf(p.p848)));

        s.store_scalar(416, (p.p843 * (((p.p846 * var_vbiinvsti)) as f64).powf(p.p849)));

        s.store_scalar(417, (p.p844 * (((p.p847 * var_vbiinvgat)) as f64).powf(p.p850)));

        s.store_scalar(418, ((s.v[415] * var_vbibot) * var_one_over_one_minus_pbot));

        s.store_scalar(419, ((s.v[416] * var_vbisti) * var_one_over_one_minus_psti));

        s.store_scalar(420, ((s.v[417] * var_vbigat) * var_one_over_one_minus_pgat));

        s.store_scalar(421, (2.0 * s.v[415]));

        s.store_scalar(422, (2.0 * s.v[416]));

        s.store_scalar(423, (2.0 * s.v[417]));

        let assign2420_e2771: f64 = (0.5 * var_phigdbot);
        let assign2420_e2773: f64 = (assign2420_e2771).max(var_phitd);
        var_deltaebot = assign2420_e2773;

        let assign2430_e2776: f64 = (0.5 * var_phigdsti);
        let assign2430_e2778: f64 = (assign2430_e2776).max(var_phitd);
        var_deltaesti = assign2430_e2778;

        let assign2440_e2781: f64 = (0.5 * var_phigdgat);
        let assign2440_e2783: f64 = (assign2440_e2781).max(var_phitd);
        var_deltaegat = assign2440_e2783;

        let assign2450_e2786: f64 = (var_deltaebot * var_phitdinv);
        var_atatbot = assign2450_e2786;

        let assign2460_e2789: f64 = (var_deltaesti * var_phitdinv);
        var_atatsti = assign2460_e2789;

        let assign2470_e2792: f64 = (var_deltaegat * var_phitdinv);
        var_atatgat = assign2470_e2792;

        let assign2480_e2795: f64 = (32.0 * p.p865);
        let assign2480_e2797: f64 = (assign2480_e2795 * 9.1093826e-31);
        let assign2480_e2799: f64 = (assign2480_e2797 * 1.6021918e-19);
        let assign2480_e2802: f64 = (var_deltaebot * var_deltaebot);
        let assign2480_e2804: f64 = (assign2480_e2802 * var_deltaebot);
        let assign2480_e2805: f64 = (assign2480_e2799 * assign2480_e2804);
        let assign2480_e2806: f64 = (assign2480_e2805).sqrt();
        let assign2480_e2809: f64 = (3.0 * 1.05457168e-34);
        let assign2480_e2810: f64 = (assign2480_e2806 / assign2480_e2809);
        var_btatpartbot = assign2480_e2810;

        let assign2490_e2813: f64 = (32.0 * p.p866);
        let assign2490_e2815: f64 = (assign2490_e2813 * 9.1093826e-31);
        let assign2490_e2817: f64 = (assign2490_e2815 * 1.6021918e-19);
        let assign2490_e2820: f64 = (var_deltaesti * var_deltaesti);
        let assign2490_e2822: f64 = (assign2490_e2820 * var_deltaesti);
        let assign2490_e2823: f64 = (assign2490_e2817 * assign2490_e2822);
        let assign2490_e2824: f64 = (assign2490_e2823).sqrt();
        let assign2490_e2827: f64 = (3.0 * 1.05457168e-34);
        let assign2490_e2828: f64 = (assign2490_e2824 / assign2490_e2827);
        var_btatpartsti = assign2490_e2828;

        let assign2500_e2831: f64 = (32.0 * p.p867);
        let assign2500_e2833: f64 = (assign2500_e2831 * 9.1093826e-31);
        let assign2500_e2835: f64 = (assign2500_e2833 * 1.6021918e-19);
        let assign2500_e2838: f64 = (var_deltaegat * var_deltaegat);
        let assign2500_e2840: f64 = (assign2500_e2838 * var_deltaegat);
        let assign2500_e2841: f64 = (assign2500_e2835 * assign2500_e2840);
        let assign2500_e2842: f64 = (assign2500_e2841).sqrt();
        let assign2500_e2845: f64 = (3.0 * 1.05457168e-34);
        let assign2500_e2846: f64 = (assign2500_e2842 / assign2500_e2845);
        var_btatpartgat = assign2500_e2846;

        let assign2510_e2852: f64 = (var_tkd_1 - var_tkr_1);
        let assign2510_e2853: f64 = (p.p874 * assign2510_e2852);
        let assign2510_e2854: f64 = (1.0 + assign2510_e2853);
        let assign2510_e2855: f64 = (p.p871 * assign2510_e2854);
        var_fbbtbot = assign2510_e2855;

        let assign2520_e2861: f64 = (var_tkd_1 - var_tkr_1);
        let assign2520_e2862: f64 = (p.p875 * assign2520_e2861);
        let assign2520_e2863: f64 = (1.0 + assign2520_e2862);
        let assign2520_e2864: f64 = (p.p872 * assign2520_e2863);
        var_fbbtsti = assign2520_e2864;

        let assign2530_e2870: f64 = (var_tkd_1 - var_tkr_1);
        let assign2530_e2871: f64 = (p.p876 * assign2530_e2870);
        let assign2530_e2872: f64 = (1.0 + assign2530_e2871);
        let assign2530_e2873: f64 = (p.p873 * assign2530_e2872);
        var_fbbtgat = assign2530_e2873;
        var_fbbtgat_dn0 = 0.0;
        var_fbbtgat_dn1 = 0.0;
        var_fbbtgat_dn2 = 0.0;
        var_fbbtgat_dn3 = 0.0;
        var_fbbtgat_dn4 = 0.0;
        var_fbbtgat_dn5 = 0.0;
        var_fbbtgat_dn6 = 0.0;
        var_fbbtgat_dn7 = 0.0;
        var_fbbtgat_dn8 = 0.0;
        var_fbbtgat_dn9 = 0.0;
        var_fbbtgat_dn10 = 0.0;
        var_fbbtgat_dn11 = 0.0;
        var_fbbtgat_dn12 = 0.0;
        var_fbbtgat_db0 = 0.0;
        var_fbbtgat_db1 = 0.0;
        var_fbbtgat_db2 = 0.0;
        var_fbbtgat_db3 = 0.0;
        var_fbbtgat_db4 = 0.0;
        var_fbbtgat_db5 = 0.0;
        var_fbbtgat_db6 = 0.0;

        let (assign2540_e2879,) = {
    if (var_fbbtbot > 0.0) {
        (var_fbbtbot,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot = assign2540_e2879;

        let (assign2550_e2885,) = {
    if (var_fbbtsti > 0.0) {
        (var_fbbtsti,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti = assign2550_e2885;

        let (assign2560_e2891, assign2560_e2891_d_n0, assign2560_e2891_d_n1, assign2560_e2891_d_n2, assign2560_e2891_d_n3, assign2560_e2891_d_n4, assign2560_e2891_d_n5, assign2560_e2891_d_n6, assign2560_e2891_d_n7, assign2560_e2891_d_n8, assign2560_e2891_d_n9, assign2560_e2891_d_n10, assign2560_e2891_d_n11, assign2560_e2891_d_n12, assign2560_e2891_d_b0, assign2560_e2891_d_b1, assign2560_e2891_d_b2, assign2560_e2891_d_b3, assign2560_e2891_d_b4, assign2560_e2891_d_b5, assign2560_e2891_d_b6,) = {
    if (var_fbbtgat > 0.0) {
        (var_fbbtgat, var_fbbtgat_dn0, var_fbbtgat_dn1, var_fbbtgat_dn2, var_fbbtgat_dn3, var_fbbtgat_dn4, var_fbbtgat_dn5, var_fbbtgat_dn6, var_fbbtgat_dn7, var_fbbtgat_dn8, var_fbbtgat_dn9, var_fbbtgat_dn10, var_fbbtgat_dn11, var_fbbtgat_dn12, var_fbbtgat_db0, var_fbbtgat_db1, var_fbbtgat_db2, var_fbbtgat_db3, var_fbbtgat_db4, var_fbbtgat_db5, var_fbbtgat_db6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat = assign2560_e2891;
        var_fbbtgat_dn0 = assign2560_e2891_d_n0;
        var_fbbtgat_dn1 = assign2560_e2891_d_n1;
        var_fbbtgat_dn2 = assign2560_e2891_d_n2;
        var_fbbtgat_dn3 = assign2560_e2891_d_n3;
        var_fbbtgat_dn4 = assign2560_e2891_d_n4;
        var_fbbtgat_dn5 = assign2560_e2891_d_n5;
        var_fbbtgat_dn6 = assign2560_e2891_d_n6;
        var_fbbtgat_dn7 = assign2560_e2891_d_n7;
        var_fbbtgat_dn8 = assign2560_e2891_d_n8;
        var_fbbtgat_dn9 = assign2560_e2891_d_n9;
        var_fbbtgat_dn10 = assign2560_e2891_d_n10;
        var_fbbtgat_dn11 = assign2560_e2891_d_n11;
        var_fbbtgat_dn12 = assign2560_e2891_d_n12;
        var_fbbtgat_db0 = assign2560_e2891_d_b0;
        var_fbbtgat_db1 = assign2560_e2891_d_b1;
        var_fbbtgat_db2 = assign2560_e2891_d_b2;
        var_fbbtgat_db3 = assign2560_e2891_d_b3;
        var_fbbtgat_db4 = assign2560_e2891_d_b4;
        var_fbbtgat_db5 = assign2560_e2891_d_b5;
        var_fbbtgat_db6 = assign2560_e2891_d_b6;

        let assign2570_e2894: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard27 = assign2570_e2894;

        let (assign2580_e2900,) = {
    if (var_guard27 != 0.0) {
        let assign2580_e2898: f64 = (var_phiggat2nd + var_deltaphigd);
        (assign2580_e2898,)
    } else {
        (var_phigdgat2nd,)
    }
};
        var_phigdgat2nd = assign2580_e2900;

        let (assign2590_e2917,) = {
    if (var_guard27 != 0.0) {
        let assign2590_e2904: f64 = (var_auxt).powf(1.5);
        let assign2590_e2908: f64 = (var_phigrgat2nd * var_phitrinv);
        let assign2590_e2911: f64 = (var_phigdgat2nd * var_phitdinv);
        let assign2590_e2912: f64 = (assign2590_e2908 - assign2590_e2911);
        let assign2590_e2913: f64 = (0.5 * assign2590_e2912);
        let assign2590_e2914: f64 = (assign2590_e2913).exp();
        let assign2590_e2915: f64 = (assign2590_e2904 * assign2590_e2914);
        (assign2590_e2915,)
    } else {
        (var_ftdgat2nd,)
    }
};
        var_ftdgat2nd = assign2590_e2917;

        let (assign2600_e2930,) = {
    if (var_guard27 != 0.0) {
        let assign2600_e2921: f64 = (var_vbirgat2nd * var_auxt);
        let assign2600_e2924: f64 = (2.0 * var_phitd);
        let assign2600_e2926: f64 = (var_ftdgat2nd).ln();
        let assign2600_e2927: f64 = (assign2600_e2924 * assign2600_e2926);
        let assign2600_e2928: f64 = (assign2600_e2921 - assign2600_e2927);
        (assign2600_e2928,)
    } else {
        (var_ubigat2nd,)
    }
};
        var_ubigat2nd = assign2600_e2930;

        let (assign2610_e2946,) = {
    if (var_guard27 != 0.0) {
        let assign2610_e2937: f64 = (0.05 - var_ubigat2nd);
        let assign2610_e2939: f64 = (assign2610_e2937 * var_phitdinv);
        let assign2610_e2940: f64 = (assign2610_e2939).exp();
        let assign2610_e2941: f64 = (1.0 + assign2610_e2940);
        let assign2610_e2942: f64 = (assign2610_e2941).ln();
        let assign2610_e2943: f64 = (var_phitd * assign2610_e2942);
        let assign2610_e2944: f64 = (var_ubigat2nd + assign2610_e2943);
        (assign2610_e2944,)
    } else {
        (var_vbigat2nd,)
    }
};
        var_vbigat2nd = assign2610_e2946;

        let (assign2620_e2952,) = {
    if (var_guard27 != 0.0) {
        let assign2620_e2950: f64 = (1.0 / var_vbigat2nd);
        (assign2620_e2950,)
    } else {
        (var_vbiinvgat2nd,)
    }
};
        var_vbiinvgat2nd = assign2620_e2952;

        if (s.v[1007] != 0.0) {
            s.store_scale(470, 458, (((var_vbirgat2nd * var_vbiinvgat2nd)) as f64).powf(var_pgat2nd));
            s.store_scaled_mul(471, 470, 469, var_vbigat2nd);
            s.store_scale(472, 470, 2.0);
        }

        let assign2660_e2979: f64 = (var_phigbotd_i + var_deltaphigd);
        var_phigdbot_d = assign2660_e2979;

        let assign2670_e2982: f64 = (var_phigstid_i + var_deltaphigd);
        var_phigdsti_d = assign2670_e2982;

        let assign2680_e2985: f64 = (var_phiggatd_i + var_deltaphigd);
        var_phigdgat_d = assign2680_e2985;

        let assign2690_e2988: f64 = (var_auxt).powf(1.5);
        let assign2690_e2992: f64 = (var_phigrbot_d * var_phitrinv);
        let assign2690_e2995: f64 = (var_phigdbot_d * var_phitdinv);
        let assign2690_e2996: f64 = (assign2690_e2992 - assign2690_e2995);
        let assign2690_e2997: f64 = (0.5 * assign2690_e2996);
        let assign2690_e2998: f64 = (assign2690_e2997).exp();
        let assign2690_e2999: f64 = (assign2690_e2988 * assign2690_e2998);
        var_ftdbot_d = assign2690_e2999;

        let assign2700_e3002: f64 = (var_auxt).powf(1.5);
        let assign2700_e3006: f64 = (var_phigrsti_d * var_phitrinv);
        let assign2700_e3009: f64 = (var_phigdsti_d * var_phitdinv);
        let assign2700_e3010: f64 = (assign2700_e3006 - assign2700_e3009);
        let assign2700_e3011: f64 = (0.5 * assign2700_e3010);
        let assign2700_e3012: f64 = (assign2700_e3011).exp();
        let assign2700_e3013: f64 = (assign2700_e3002 * assign2700_e3012);
        var_ftdsti_d = assign2700_e3013;

        let assign2710_e3016: f64 = (var_auxt).powf(1.5);
        let assign2710_e3020: f64 = (var_phigrgat_d * var_phitrinv);
        let assign2710_e3023: f64 = (var_phigdgat_d * var_phitdinv);
        let assign2710_e3024: f64 = (assign2710_e3020 - assign2710_e3023);
        let assign2710_e3025: f64 = (0.5 * assign2710_e3024);
        let assign2710_e3026: f64 = (assign2710_e3025).exp();
        let assign2710_e3027: f64 = (assign2710_e3016 * assign2710_e3026);
        var_ftdgat_d = assign2710_e3027;

        let assign2720_e3030: f64 = (var_idsatrbotd_i * var_ftdbot_d);
        let assign2720_e3032: f64 = (assign2720_e3030 * var_ftdbot_d);
        var_idsatbot_d = assign2720_e3032;

        let assign2730_e3035: f64 = (var_idsatrstid_i * var_ftdsti_d);
        let assign2730_e3037: f64 = (assign2730_e3035 * var_ftdsti_d);
        var_idsatsti_d = assign2730_e3037;

        let assign2740_e3040: f64 = (var_idsatrgatd_i * var_ftdgat_d);
        let assign2740_e3042: f64 = (assign2740_e3040 * var_ftdgat_d);
        var_idsatgat_d = assign2740_e3042;

        let assign2750_e3045: f64 = (var_vbirbotd_i * var_auxt);
        let assign2750_e3048: f64 = (2.0 * var_phitd);
        let assign2750_e3050: f64 = (var_ftdbot_d).ln();
        let assign2750_e3051: f64 = (assign2750_e3048 * assign2750_e3050);
        let assign2750_e3052: f64 = (assign2750_e3045 - assign2750_e3051);
        var_ubibot_d = assign2750_e3052;

        *var_atatbot_slot = var_atatbot;
        *var_atatgat_slot = var_atatgat;
        *var_atatsti_slot = var_atatsti;
        *var_auxt_slot = var_auxt;
        *var_btatpartbot_slot = var_btatpartbot;
        *var_btatpartgat_slot = var_btatpartgat;
        *var_btatpartsti_slot = var_btatpartsti;
        *var_delta_slot = var_delta;
        *var_deltaebot_slot = var_deltaebot;
        *var_deltaegat_slot = var_deltaegat;
        *var_deltaesti_slot = var_deltaesti;
        *var_deltaphigd_slot = var_deltaphigd;
        *var_fbbtbot_slot = var_fbbtbot;
        *var_fbbtgat_slot = var_fbbtgat;
        *var_fbbtgat_db0_slot = var_fbbtgat_db0;
        *var_fbbtgat_db1_slot = var_fbbtgat_db1;
        *var_fbbtgat_db2_slot = var_fbbtgat_db2;
        *var_fbbtgat_db3_slot = var_fbbtgat_db3;
        *var_fbbtgat_db4_slot = var_fbbtgat_db4;
        *var_fbbtgat_db5_slot = var_fbbtgat_db5;
        *var_fbbtgat_db6_slot = var_fbbtgat_db6;
        *var_fbbtgat_dn0_slot = var_fbbtgat_dn0;
        *var_fbbtgat_dn1_slot = var_fbbtgat_dn1;
        *var_fbbtgat_dn10_slot = var_fbbtgat_dn10;
        *var_fbbtgat_dn11_slot = var_fbbtgat_dn11;
        *var_fbbtgat_dn12_slot = var_fbbtgat_dn12;
        *var_fbbtgat_dn2_slot = var_fbbtgat_dn2;
        *var_fbbtgat_dn3_slot = var_fbbtgat_dn3;
        *var_fbbtgat_dn4_slot = var_fbbtgat_dn4;
        *var_fbbtgat_dn5_slot = var_fbbtgat_dn5;
        *var_fbbtgat_dn6_slot = var_fbbtgat_dn6;
        *var_fbbtgat_dn7_slot = var_fbbtgat_dn7;
        *var_fbbtgat_dn8_slot = var_fbbtgat_dn8;
        *var_fbbtgat_dn9_slot = var_fbbtgat_dn9;
        *var_fbbtsti_slot = var_fbbtsti;
        *var_ftdbot_slot = var_ftdbot;
        *var_ftdbot_d_slot = var_ftdbot_d;
        *var_ftdgat_slot = var_ftdgat;
        *var_ftdgat2nd_slot = var_ftdgat2nd;
        *var_ftdgat_d_slot = var_ftdgat_d;
        *var_ftdsti_slot = var_ftdsti;
        *var_ftdsti_d_slot = var_ftdsti_d;
        *var_guard27_slot = var_guard27;
        *var_idsatbot_slot = var_idsatbot;
        *var_idsatbot_d_slot = var_idsatbot_d;
        *var_idsatgat_slot = var_idsatgat;
        *var_idsatgat_d_slot = var_idsatgat_d;
        *var_idsatsti_slot = var_idsatsti;
        *var_idsatsti_d_slot = var_idsatsti_d;
        *var_inv_phita_slot = var_inv_phita;
        *var_one_minus_pgat2nd_d_slot = var_one_minus_pgat2nd_d;
        *var_pgat2nd_d_slot = var_pgat2nd_d;
        *var_phigdbot_slot = var_phigdbot;
        *var_phigdbot_d_slot = var_phigdbot_d;
        *var_phigdgat_slot = var_phigdgat;
        *var_phigdgat2nd_slot = var_phigdgat2nd;
        *var_phigdgat_d_slot = var_phigdgat_d;
        *var_phigdsti_slot = var_phigdsti;
        *var_phigdsti_d_slot = var_phigdsti_d;
        *var_phiggat2nd_d_slot = var_phiggat2nd_d;
        *var_phigrgat2nd_d_slot = var_phigrgat2nd_d;
        *var_phita_slot = var_phita;
        *var_phitd_slot = var_phitd;
        *var_phitdinv_slot = var_phitdinv;
        *var_rta_slot = var_rta;
        *var_tka_slot = var_tka;
        *var_tkd_1_slot = var_tkd_1;
        *var_ubibot_slot = var_ubibot;
        *var_ubibot_d_slot = var_ubibot_d;
        *var_ubigat_slot = var_ubigat;
        *var_ubigat2nd_slot = var_ubigat2nd;
        *var_ubisti_slot = var_ubisti;
        *var_vbibot_slot = var_vbibot;
        *var_vbigat_slot = var_vbigat;
        *var_vbigat2nd_slot = var_vbigat2nd;
        *var_vbiinvbot_slot = var_vbiinvbot;
        *var_vbiinvgat_slot = var_vbiinvgat;
        *var_vbiinvgat2nd_slot = var_vbiinvgat2nd;
        *var_vbiinvsti_slot = var_vbiinvsti;
        *var_vbisti_slot = var_vbisti;
    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        var_auxt: f64,
        var_cjorbotd_i: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigd: f64,
        var_fbbtrbotd_i: f64,
        var_fbbtrgatd_i: f64,
        var_fbbtrstid_i: f64,
        var_ftdgat_d: f64,
        var_ftdsti_d: f64,
        var_mefftatbotd_i: f64,
        var_mefftatgatd_i: f64,
        var_mefftatstid_i: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbotd_i: f64,
        var_pgat2nd_d: f64,
        var_pgatd_i: f64,
        var_phigdbot_d: f64,
        var_phigdgat_d: f64,
        var_phigdsti_d: f64,
        var_phiggat2nd_d: f64,
        var_phigrgat2nd_d: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitrinv: f64,
        var_pstid_i: f64,
        var_stfbbtbotd_i: f64,
        var_stfbbtgatd_i: f64,
        var_stfbbtstid_i: f64,
        var_swgat2nd_d: f64,
        var_tkd_1: f64,
        var_tkr_1: f64,
        var_ubibot_d: f64,
        var_vbirbotd_i: f64,
        var_vbirgat2nd_d: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_abdrain_i_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_ad_i_slot: &mut f64,
        var_as_i_slot: &mut f64,
        var_atatbot_d_slot: &mut f64,
        var_atatgat_d_slot: &mut f64,
        var_atatsti_d_slot: &mut f64,
        var_btatpartbot_d_slot: &mut f64,
        var_btatpartgat_d_slot: &mut f64,
        var_btatpartsti_d_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_deltaebot_d_slot: &mut f64,
        var_deltaegat_d_slot: &mut f64,
        var_deltaesti_d_slot: &mut f64,
        var_delwod_slot: &mut f64,
        var_fbbtbot_d_slot: &mut f64,
        var_fbbtgat_d_slot: &mut f64,
        var_fbbtgat_d_db0_slot: &mut f64,
        var_fbbtgat_d_db1_slot: &mut f64,
        var_fbbtgat_d_db2_slot: &mut f64,
        var_fbbtgat_d_db3_slot: &mut f64,
        var_fbbtgat_d_db4_slot: &mut f64,
        var_fbbtgat_d_db5_slot: &mut f64,
        var_fbbtgat_d_db6_slot: &mut f64,
        var_fbbtgat_d_dn0_slot: &mut f64,
        var_fbbtgat_d_dn1_slot: &mut f64,
        var_fbbtgat_d_dn10_slot: &mut f64,
        var_fbbtgat_d_dn11_slot: &mut f64,
        var_fbbtgat_d_dn12_slot: &mut f64,
        var_fbbtgat_d_dn2_slot: &mut f64,
        var_fbbtgat_d_dn3_slot: &mut f64,
        var_fbbtgat_d_dn4_slot: &mut f64,
        var_fbbtgat_d_dn5_slot: &mut f64,
        var_fbbtgat_d_dn6_slot: &mut f64,
        var_fbbtgat_d_dn7_slot: &mut f64,
        var_fbbtgat_d_dn8_slot: &mut f64,
        var_fbbtgat_d_dn9_slot: &mut f64,
        var_fbbtsti_d_slot: &mut f64,
        var_ftdgat2nd_d_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_iae_slot: &mut f64,
        var_iiae_slot: &mut f64,
        var_iiwe_slot: &mut f64,
        var_iiwecv_slot: &mut f64,
        var_il_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_ile2_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_jw_i_slot: &mut f64,
        var_l_i_slot: &mut f64,
        var_lcv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_ngcon_i_slot: &mut f64,
        var_pd_i_slot: &mut f64,
        var_phigdgat2nd_d_slot: &mut f64,
        var_ps_i_slot: &mut f64,
        var_sa_i_slot: &mut f64,
        var_sb_i_slot: &mut f64,
        var_sc_i_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_sd_i_slot: &mut f64,
        var_ubigat2nd_d_slot: &mut f64,
        var_ubigat_d_slot: &mut f64,
        var_ubisti_d_slot: &mut f64,
        var_vbibot_d_slot: &mut f64,
        var_vbigat2nd_d_slot: &mut f64,
        var_vbigat_d_slot: &mut f64,
        var_vbiinvbot_d_slot: &mut f64,
        var_vbiinvgat2nd_d_slot: &mut f64,
        var_vbiinvgat_d_slot: &mut f64,
        var_vbiinvsti_d_slot: &mut f64,
        var_vbisti_d_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_we_slot: &mut f64,
        var_wecv_slot: &mut f64,
        var_xgw_i_slot: &mut f64,
    ) {
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_ad_i: f64 = *var_ad_i_slot;
        let mut var_as_i: f64 = *var_as_i_slot;
        let mut var_atatbot_d: f64 = *var_atatbot_d_slot;
        let mut var_atatgat_d: f64 = *var_atatgat_d_slot;
        let mut var_atatsti_d: f64 = *var_atatsti_d_slot;
        let mut var_btatpartbot_d: f64 = *var_btatpartbot_d_slot;
        let mut var_btatpartgat_d: f64 = *var_btatpartgat_d_slot;
        let mut var_btatpartsti_d: f64 = *var_btatpartsti_d_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_deltaebot_d: f64 = *var_deltaebot_d_slot;
        let mut var_deltaegat_d: f64 = *var_deltaegat_d_slot;
        let mut var_deltaesti_d: f64 = *var_deltaesti_d_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
        let mut var_fbbtbot_d: f64 = *var_fbbtbot_d_slot;
        let mut var_fbbtgat_d: f64 = *var_fbbtgat_d_slot;
        let mut var_fbbtgat_d_db0: f64 = *var_fbbtgat_d_db0_slot;
        let mut var_fbbtgat_d_db1: f64 = *var_fbbtgat_d_db1_slot;
        let mut var_fbbtgat_d_db2: f64 = *var_fbbtgat_d_db2_slot;
        let mut var_fbbtgat_d_db3: f64 = *var_fbbtgat_d_db3_slot;
        let mut var_fbbtgat_d_db4: f64 = *var_fbbtgat_d_db4_slot;
        let mut var_fbbtgat_d_db5: f64 = *var_fbbtgat_d_db5_slot;
        let mut var_fbbtgat_d_db6: f64 = *var_fbbtgat_d_db6_slot;
        let mut var_fbbtgat_d_dn0: f64 = *var_fbbtgat_d_dn0_slot;
        let mut var_fbbtgat_d_dn1: f64 = *var_fbbtgat_d_dn1_slot;
        let mut var_fbbtgat_d_dn10: f64 = *var_fbbtgat_d_dn10_slot;
        let mut var_fbbtgat_d_dn11: f64 = *var_fbbtgat_d_dn11_slot;
        let mut var_fbbtgat_d_dn12: f64 = *var_fbbtgat_d_dn12_slot;
        let mut var_fbbtgat_d_dn2: f64 = *var_fbbtgat_d_dn2_slot;
        let mut var_fbbtgat_d_dn3: f64 = *var_fbbtgat_d_dn3_slot;
        let mut var_fbbtgat_d_dn4: f64 = *var_fbbtgat_d_dn4_slot;
        let mut var_fbbtgat_d_dn5: f64 = *var_fbbtgat_d_dn5_slot;
        let mut var_fbbtgat_d_dn6: f64 = *var_fbbtgat_d_dn6_slot;
        let mut var_fbbtgat_d_dn7: f64 = *var_fbbtgat_d_dn7_slot;
        let mut var_fbbtgat_d_dn8: f64 = *var_fbbtgat_d_dn8_slot;
        let mut var_fbbtgat_d_dn9: f64 = *var_fbbtgat_d_dn9_slot;
        let mut var_fbbtsti_d: f64 = *var_fbbtsti_d_slot;
        let mut var_ftdgat2nd_d: f64 = *var_ftdgat2nd_d_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_iiae: f64 = *var_iiae_slot;
        let mut var_iiwe: f64 = *var_iiwe_slot;
        let mut var_iiwecv: f64 = *var_iiwecv_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_ile2: f64 = *var_ile2_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_jw_i: f64 = *var_jw_i_slot;
        let mut var_l_i: f64 = *var_l_i_slot;
        let mut var_lcv: f64 = *var_lcv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_ngcon_i: f64 = *var_ngcon_i_slot;
        let mut var_pd_i: f64 = *var_pd_i_slot;
        let mut var_phigdgat2nd_d: f64 = *var_phigdgat2nd_d_slot;
        let mut var_ps_i: f64 = *var_ps_i_slot;
        let mut var_sa_i: f64 = *var_sa_i_slot;
        let mut var_sb_i: f64 = *var_sb_i_slot;
        let mut var_sc_i: f64 = *var_sc_i_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_sd_i: f64 = *var_sd_i_slot;
        let mut var_ubigat2nd_d: f64 = *var_ubigat2nd_d_slot;
        let mut var_ubigat_d: f64 = *var_ubigat_d_slot;
        let mut var_ubisti_d: f64 = *var_ubisti_d_slot;
        let mut var_vbibot_d: f64 = *var_vbibot_d_slot;
        let mut var_vbigat2nd_d: f64 = *var_vbigat2nd_d_slot;
        let mut var_vbigat_d: f64 = *var_vbigat_d_slot;
        let mut var_vbiinvbot_d: f64 = *var_vbiinvbot_d_slot;
        let mut var_vbiinvgat2nd_d: f64 = *var_vbiinvgat2nd_d_slot;
        let mut var_vbiinvgat_d: f64 = *var_vbiinvgat_d_slot;
        let mut var_vbiinvsti_d: f64 = *var_vbiinvsti_d_slot;
        let mut var_vbisti_d: f64 = *var_vbisti_d_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_xgw_i: f64 = *var_xgw_i_slot;

        let assign2760_e3055: f64 = (var_vbirstid_i * var_auxt);
        let assign2760_e3058: f64 = (2.0 * var_phitd);
        let assign2760_e3060: f64 = (var_ftdsti_d).ln();
        let assign2760_e3061: f64 = (assign2760_e3058 * assign2760_e3060);
        let assign2760_e3062: f64 = (assign2760_e3055 - assign2760_e3061);
        var_ubisti_d = assign2760_e3062;

        let assign2770_e3065: f64 = (var_vbirgatd_i * var_auxt);
        let assign2770_e3068: f64 = (2.0 * var_phitd);
        let assign2770_e3070: f64 = (var_ftdgat_d).ln();
        let assign2770_e3071: f64 = (assign2770_e3068 * assign2770_e3070);
        let assign2770_e3072: f64 = (assign2770_e3065 - assign2770_e3071);
        var_ubigat_d = assign2770_e3072;

        let assign2780_e3078: f64 = (0.05 - var_ubibot_d);
        let assign2780_e3080: f64 = (assign2780_e3078 * var_phitdinv);
        let assign2780_e3081: f64 = (assign2780_e3080).exp();
        let assign2780_e3082: f64 = (1.0 + assign2780_e3081);
        let assign2780_e3083: f64 = (assign2780_e3082).ln();
        let assign2780_e3084: f64 = (var_phitd * assign2780_e3083);
        let assign2780_e3085: f64 = (var_ubibot_d + assign2780_e3084);
        var_vbibot_d = assign2780_e3085;

        let assign2790_e3091: f64 = (0.05 - var_ubisti_d);
        let assign2790_e3093: f64 = (assign2790_e3091 * var_phitdinv);
        let assign2790_e3094: f64 = (assign2790_e3093).exp();
        let assign2790_e3095: f64 = (1.0 + assign2790_e3094);
        let assign2790_e3096: f64 = (assign2790_e3095).ln();
        let assign2790_e3097: f64 = (var_phitd * assign2790_e3096);
        let assign2790_e3098: f64 = (var_ubisti_d + assign2790_e3097);
        var_vbisti_d = assign2790_e3098;

        let assign2800_e3104: f64 = (0.05 - var_ubigat_d);
        let assign2800_e3106: f64 = (assign2800_e3104 * var_phitdinv);
        let assign2800_e3107: f64 = (assign2800_e3106).exp();
        let assign2800_e3108: f64 = (1.0 + assign2800_e3107);
        let assign2800_e3109: f64 = (assign2800_e3108).ln();
        let assign2800_e3110: f64 = (var_phitd * assign2800_e3109);
        let assign2800_e3111: f64 = (var_ubigat_d + assign2800_e3110);
        var_vbigat_d = assign2800_e3111;

        let assign2810_e3114: f64 = (1.0 / var_vbibot_d);
        var_vbiinvbot_d = assign2810_e3114;

        let assign2820_e3117: f64 = (1.0 / var_vbisti_d);
        var_vbiinvsti_d = assign2820_e3117;

        let assign2830_e3120: f64 = (1.0 / var_vbigat_d);
        var_vbiinvgat_d = assign2830_e3120;

        s.store_scalar(582, (var_cjorbotd_i * (((var_vbirbotd_i * var_vbiinvbot_d)) as f64).powf(var_pbotd_i)));

        s.store_scalar(583, (var_cjorstid_i * (((var_vbirstid_i * var_vbiinvsti_d)) as f64).powf(var_pstid_i)));

        s.store_scalar(584, (var_cjorgatd_i * (((var_vbirgatd_i * var_vbiinvgat_d)) as f64).powf(var_pgatd_i)));

        s.store_scalar(585, ((s.v[582] * var_vbibot_d) * var_one_over_one_minus_pbot_d));

        s.store_scalar(586, ((s.v[583] * var_vbisti_d) * var_one_over_one_minus_psti_d));

        s.store_scalar(587, ((s.v[584] * var_vbigat_d) * var_one_over_one_minus_pgat_d));

        s.store_scalar(588, (2.0 * s.v[582]));

        s.store_scalar(589, (2.0 * s.v[583]));

        s.store_scalar(590, (2.0 * s.v[584]));

        let assign2930_e3168: f64 = (0.5 * var_phigdbot_d);
        let assign2930_e3170: f64 = (assign2930_e3168).max(var_phitd);
        var_deltaebot_d = assign2930_e3170;

        let assign2940_e3173: f64 = (0.5 * var_phigdsti_d);
        let assign2940_e3175: f64 = (assign2940_e3173).max(var_phitd);
        var_deltaesti_d = assign2940_e3175;

        let assign2950_e3178: f64 = (0.5 * var_phigdgat_d);
        let assign2950_e3180: f64 = (assign2950_e3178).max(var_phitd);
        var_deltaegat_d = assign2950_e3180;

        let assign2960_e3183: f64 = (var_deltaebot_d * var_phitdinv);
        var_atatbot_d = assign2960_e3183;

        let assign2970_e3186: f64 = (var_deltaesti_d * var_phitdinv);
        var_atatsti_d = assign2970_e3186;

        let assign2980_e3189: f64 = (var_deltaegat_d * var_phitdinv);
        var_atatgat_d = assign2980_e3189;

        let assign2990_e3192: f64 = (32.0 * var_mefftatbotd_i);
        let assign2990_e3194: f64 = (assign2990_e3192 * 9.1093826e-31);
        let assign2990_e3196: f64 = (assign2990_e3194 * 1.6021918e-19);
        let assign2990_e3199: f64 = (var_deltaebot_d * var_deltaebot_d);
        let assign2990_e3201: f64 = (assign2990_e3199 * var_deltaebot_d);
        let assign2990_e3202: f64 = (assign2990_e3196 * assign2990_e3201);
        let assign2990_e3203: f64 = (assign2990_e3202).sqrt();
        let assign2990_e3206: f64 = (3.0 * 1.05457168e-34);
        let assign2990_e3207: f64 = (assign2990_e3203 / assign2990_e3206);
        var_btatpartbot_d = assign2990_e3207;

        let assign3000_e3210: f64 = (32.0 * var_mefftatstid_i);
        let assign3000_e3212: f64 = (assign3000_e3210 * 9.1093826e-31);
        let assign3000_e3214: f64 = (assign3000_e3212 * 1.6021918e-19);
        let assign3000_e3217: f64 = (var_deltaesti_d * var_deltaesti_d);
        let assign3000_e3219: f64 = (assign3000_e3217 * var_deltaesti_d);
        let assign3000_e3220: f64 = (assign3000_e3214 * assign3000_e3219);
        let assign3000_e3221: f64 = (assign3000_e3220).sqrt();
        let assign3000_e3224: f64 = (3.0 * 1.05457168e-34);
        let assign3000_e3225: f64 = (assign3000_e3221 / assign3000_e3224);
        var_btatpartsti_d = assign3000_e3225;

        let assign3010_e3228: f64 = (32.0 * var_mefftatgatd_i);
        let assign3010_e3230: f64 = (assign3010_e3228 * 9.1093826e-31);
        let assign3010_e3232: f64 = (assign3010_e3230 * 1.6021918e-19);
        let assign3010_e3235: f64 = (var_deltaegat_d * var_deltaegat_d);
        let assign3010_e3237: f64 = (assign3010_e3235 * var_deltaegat_d);
        let assign3010_e3238: f64 = (assign3010_e3232 * assign3010_e3237);
        let assign3010_e3239: f64 = (assign3010_e3238).sqrt();
        let assign3010_e3242: f64 = (3.0 * 1.05457168e-34);
        let assign3010_e3243: f64 = (assign3010_e3239 / assign3010_e3242);
        var_btatpartgat_d = assign3010_e3243;

        let assign3020_e3249: f64 = (var_tkd_1 - var_tkr_1);
        let assign3020_e3250: f64 = (var_stfbbtbotd_i * assign3020_e3249);
        let assign3020_e3251: f64 = (1.0 + assign3020_e3250);
        let assign3020_e3252: f64 = (var_fbbtrbotd_i * assign3020_e3251);
        var_fbbtbot_d = assign3020_e3252;

        let assign3030_e3258: f64 = (var_tkd_1 - var_tkr_1);
        let assign3030_e3259: f64 = (var_stfbbtstid_i * assign3030_e3258);
        let assign3030_e3260: f64 = (1.0 + assign3030_e3259);
        let assign3030_e3261: f64 = (var_fbbtrstid_i * assign3030_e3260);
        var_fbbtsti_d = assign3030_e3261;

        let assign3040_e3267: f64 = (var_tkd_1 - var_tkr_1);
        let assign3040_e3268: f64 = (var_stfbbtgatd_i * assign3040_e3267);
        let assign3040_e3269: f64 = (1.0 + assign3040_e3268);
        let assign3040_e3270: f64 = (var_fbbtrgatd_i * assign3040_e3269);
        var_fbbtgat_d = assign3040_e3270;
        var_fbbtgat_d_dn0 = 0.0;
        var_fbbtgat_d_dn1 = 0.0;
        var_fbbtgat_d_dn2 = 0.0;
        var_fbbtgat_d_dn3 = 0.0;
        var_fbbtgat_d_dn4 = 0.0;
        var_fbbtgat_d_dn5 = 0.0;
        var_fbbtgat_d_dn6 = 0.0;
        var_fbbtgat_d_dn7 = 0.0;
        var_fbbtgat_d_dn8 = 0.0;
        var_fbbtgat_d_dn9 = 0.0;
        var_fbbtgat_d_dn10 = 0.0;
        var_fbbtgat_d_dn11 = 0.0;
        var_fbbtgat_d_dn12 = 0.0;
        var_fbbtgat_d_db0 = 0.0;
        var_fbbtgat_d_db1 = 0.0;
        var_fbbtgat_d_db2 = 0.0;
        var_fbbtgat_d_db3 = 0.0;
        var_fbbtgat_d_db4 = 0.0;
        var_fbbtgat_d_db5 = 0.0;
        var_fbbtgat_d_db6 = 0.0;

        let (assign3050_e3276,) = {
    if (var_fbbtbot_d > 0.0) {
        (var_fbbtbot_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot_d = assign3050_e3276;

        let (assign3060_e3282,) = {
    if (var_fbbtsti_d > 0.0) {
        (var_fbbtsti_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti_d = assign3060_e3282;

        let (assign3070_e3288, assign3070_e3288_d_n0, assign3070_e3288_d_n1, assign3070_e3288_d_n2, assign3070_e3288_d_n3, assign3070_e3288_d_n4, assign3070_e3288_d_n5, assign3070_e3288_d_n6, assign3070_e3288_d_n7, assign3070_e3288_d_n8, assign3070_e3288_d_n9, assign3070_e3288_d_n10, assign3070_e3288_d_n11, assign3070_e3288_d_n12, assign3070_e3288_d_b0, assign3070_e3288_d_b1, assign3070_e3288_d_b2, assign3070_e3288_d_b3, assign3070_e3288_d_b4, assign3070_e3288_d_b5, assign3070_e3288_d_b6,) = {
    if (var_fbbtgat_d > 0.0) {
        (var_fbbtgat_d, var_fbbtgat_d_dn0, var_fbbtgat_d_dn1, var_fbbtgat_d_dn2, var_fbbtgat_d_dn3, var_fbbtgat_d_dn4, var_fbbtgat_d_dn5, var_fbbtgat_d_dn6, var_fbbtgat_d_dn7, var_fbbtgat_d_dn8, var_fbbtgat_d_dn9, var_fbbtgat_d_dn10, var_fbbtgat_d_dn11, var_fbbtgat_d_dn12, var_fbbtgat_d_db0, var_fbbtgat_d_db1, var_fbbtgat_d_db2, var_fbbtgat_d_db3, var_fbbtgat_d_db4, var_fbbtgat_d_db5, var_fbbtgat_d_db6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat_d = assign3070_e3288;
        var_fbbtgat_d_dn0 = assign3070_e3288_d_n0;
        var_fbbtgat_d_dn1 = assign3070_e3288_d_n1;
        var_fbbtgat_d_dn2 = assign3070_e3288_d_n2;
        var_fbbtgat_d_dn3 = assign3070_e3288_d_n3;
        var_fbbtgat_d_dn4 = assign3070_e3288_d_n4;
        var_fbbtgat_d_dn5 = assign3070_e3288_d_n5;
        var_fbbtgat_d_dn6 = assign3070_e3288_d_n6;
        var_fbbtgat_d_dn7 = assign3070_e3288_d_n7;
        var_fbbtgat_d_dn8 = assign3070_e3288_d_n8;
        var_fbbtgat_d_dn9 = assign3070_e3288_d_n9;
        var_fbbtgat_d_dn10 = assign3070_e3288_d_n10;
        var_fbbtgat_d_dn11 = assign3070_e3288_d_n11;
        var_fbbtgat_d_dn12 = assign3070_e3288_d_n12;
        var_fbbtgat_d_db0 = assign3070_e3288_d_b0;
        var_fbbtgat_d_db1 = assign3070_e3288_d_b1;
        var_fbbtgat_d_db2 = assign3070_e3288_d_b2;
        var_fbbtgat_d_db3 = assign3070_e3288_d_b3;
        var_fbbtgat_d_db4 = assign3070_e3288_d_b4;
        var_fbbtgat_d_db5 = assign3070_e3288_d_b5;
        var_fbbtgat_d_db6 = assign3070_e3288_d_b6;

        let assign3080_e3291: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard28 = assign3080_e3291;

        let (assign3090_e3297,) = {
    if (var_guard28 != 0.0) {
        let assign3090_e3295: f64 = (var_phiggat2nd_d + var_deltaphigd);
        (assign3090_e3295,)
    } else {
        (var_phigdgat2nd_d,)
    }
};
        var_phigdgat2nd_d = assign3090_e3297;

        let (assign3100_e3314,) = {
    if (var_guard28 != 0.0) {
        let assign3100_e3301: f64 = (var_auxt).powf(1.5);
        let assign3100_e3305: f64 = (var_phigrgat2nd_d * var_phitrinv);
        let assign3100_e3308: f64 = (var_phigdgat2nd_d * var_phitdinv);
        let assign3100_e3309: f64 = (assign3100_e3305 - assign3100_e3308);
        let assign3100_e3310: f64 = (0.5 * assign3100_e3309);
        let assign3100_e3311: f64 = (assign3100_e3310).exp();
        let assign3100_e3312: f64 = (assign3100_e3301 * assign3100_e3311);
        (assign3100_e3312,)
    } else {
        (var_ftdgat2nd_d,)
    }
};
        var_ftdgat2nd_d = assign3100_e3314;

        let (assign3110_e3327,) = {
    if (var_guard28 != 0.0) {
        let assign3110_e3318: f64 = (var_vbirgat2nd_d * var_auxt);
        let assign3110_e3321: f64 = (2.0 * var_phitd);
        let assign3110_e3323: f64 = (var_ftdgat2nd_d).ln();
        let assign3110_e3324: f64 = (assign3110_e3321 * assign3110_e3323);
        let assign3110_e3325: f64 = (assign3110_e3318 - assign3110_e3324);
        (assign3110_e3325,)
    } else {
        (var_ubigat2nd_d,)
    }
};
        var_ubigat2nd_d = assign3110_e3327;

        let (assign3120_e3343,) = {
    if (var_guard28 != 0.0) {
        let assign3120_e3334: f64 = (0.05 - var_ubigat2nd_d);
        let assign3120_e3336: f64 = (assign3120_e3334 * var_phitdinv);
        let assign3120_e3337: f64 = (assign3120_e3336).exp();
        let assign3120_e3338: f64 = (1.0 + assign3120_e3337);
        let assign3120_e3339: f64 = (assign3120_e3338).ln();
        let assign3120_e3340: f64 = (var_phitd * assign3120_e3339);
        let assign3120_e3341: f64 = (var_ubigat2nd_d + assign3120_e3340);
        (assign3120_e3341,)
    } else {
        (var_vbigat2nd_d,)
    }
};
        var_vbigat2nd_d = assign3120_e3343;

        let (assign3130_e3349,) = {
    if (var_guard28 != 0.0) {
        let assign3130_e3347: f64 = (1.0 / var_vbigat2nd_d);
        (assign3130_e3347,)
    } else {
        (var_vbiinvgat2nd_d,)
    }
};
        var_vbiinvgat2nd_d = assign3130_e3349;

        if (s.v[1008] != 0.0) {
            s.store_scale(633, 621, (((var_vbirgat2nd_d * var_vbiinvgat2nd_d)) as f64).powf(var_pgat2nd_d));
            s.store_scaled_mul(634, 633, 632, var_vbigat2nd_d);
            s.store_scale(635, 633, 2.0);
        }

        var_nf_i = 1.0;

        var_invnf = 1.0;

        var_le = 0.0;

        var_we = 0.0;

        var_l_i = p.p0;

        var_w_i = p.p1;

        var_sa_i = p.p2;

        var_sb_i = p.p3;

        var_sd_i = p.p4;

        var_sc_i = p.p8;

        var_xgw_i = p.p11;

        var_absource_i = p.p19;

        var_lssource_i = p.p20;

        var_lgsource_i = p.p21;

        var_abdrain_i = p.p22;

        var_lsdrain_i = p.p23;

        var_lgdrain_i = p.p24;

        var_as_i = p.p25;

        var_ps_i = p.p26;

        var_ad_i = p.p27;

        var_pd_i = p.p28;

        var_jw_i = p.p14;

        let assign3390_e3398: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard29 = assign3390_e3398;

        let (assign3400_e3407,) = {
    if (var_guard29 != 0.0) {
        let (assign3400_e3405,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3400_e3405,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3400_e3407;

        let (assign3410_e3414,) = {
    if (var_guard29 != 0.0) {
        let assign3410_e3411: f64 = (var_nf_i + 0.5);
        let assign3410_e3412: f64 = (assign3410_e3411).floor();
        (assign3410_e3412,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3410_e3414;

        let (assign3420_e3420,) = {
    if (var_guard29 != 0.0) {
        let assign3420_e3418: f64 = (1.0 / var_nf_i);
        (assign3420_e3418,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign3420_e3420;

        let assign3430_e3423: f64 = (var_w_i * var_invnf);
        let (assign3430_e3430,) = {
    if (assign3430_e3423 > 1e-9) {
        let assign3430_e3428: f64 = (var_w_i * var_invnf);
        (assign3430_e3428,)
    } else {
        (1e-9,)
    }
};
        var_w_i = assign3430_e3430;

        var_sca_i = p.p5;

        var_scb_i = p.p6;

        var_scc_i = p.p7;

        let (assign3470_e3439,) = {
    if (p.p10 < 1.5) {
        (1.0,)
    } else {
        (2.0,)
    }
};
        var_ngcon_i = assign3470_e3439;

        let assign3480_e3442: f64 = (1e-6 / var_l_i);
        var_il = assign3480_e3442;

        let assign3490_e3445: f64 = (1e-6 / var_w_i);
        var_iw = assign3490_e3445;

        let assign3500_e3450: f64 = (p.p190 * var_il);
        let assign3500_e3451: f64 = (1.0 + assign3500_e3450);
        let assign3500_e3452: f64 = (p.p189 * assign3500_e3451);
        let assign3500_e3456: f64 = (p.p191 * var_iw);
        let assign3500_e3457: f64 = (1.0 + assign3500_e3456);
        let assign3500_e3458: f64 = (assign3500_e3452 * assign3500_e3457);
        var_dellps = assign3500_e3458;

        let assign3510_e3463: f64 = (p.p194 * var_il);
        let assign3510_e3464: f64 = (1.0 + assign3510_e3463);
        let assign3510_e3465: f64 = (p.p193 * assign3510_e3464);
        let assign3510_e3469: f64 = (p.p195 * var_iw);
        let assign3510_e3470: f64 = (1.0 + assign3510_e3469);
        let assign3510_e3471: f64 = (assign3510_e3465 * assign3510_e3470);
        var_delwod = assign3510_e3471;

        let assign3520_e3474: f64 = (var_l_i + var_dellps);
        let assign3520_e3477: f64 = (2.0 * p.p192);
        let assign3520_e3478: f64 = (assign3520_e3474 - assign3520_e3477);
        let (assign3520_e3489,) = {
    if (assign3520_e3478 > 1e-9) {
        let assign3520_e3483: f64 = (var_l_i + var_dellps);
        let assign3520_e3486: f64 = (2.0 * p.p192);
        let assign3520_e3487: f64 = (assign3520_e3483 - assign3520_e3486);
        (assign3520_e3487,)
    } else {
        (1e-9,)
    }
};
        var_le = assign3520_e3489;

        let assign3530_e3492: f64 = (var_w_i + var_delwod);
        let assign3530_e3495: f64 = (2.0 * p.p196);
        let assign3530_e3496: f64 = (assign3530_e3492 - assign3530_e3495);
        let (assign3530_e3507,) = {
    if (assign3530_e3496 > 1e-9) {
        let assign3530_e3501: f64 = (var_w_i + var_delwod);
        let assign3530_e3504: f64 = (2.0 * p.p196);
        let assign3530_e3505: f64 = (assign3530_e3501 - assign3530_e3504);
        (assign3530_e3505,)
    } else {
        (1e-9,)
    }
};
        var_we = assign3530_e3507;

        let assign3540_e3510: f64 = (1e-6 / var_le);
        var_ile = assign3540_e3510;

        let assign3550_e3513: f64 = (var_ile * var_ile);
        var_ile2 = assign3550_e3513;

        let assign3560_e3516: f64 = (1e-6 / var_we);
        var_iwe = assign3560_e3516;

        let assign3570_e3519: f64 = (1.0 / var_iwe);
        var_iiwe = assign3570_e3519;

        let assign3580_e3522: f64 = (var_ile * var_iwe);
        var_iae = assign3580_e3522;

        let assign3590_e3525: f64 = (1.0 / var_iae);
        var_iiae = assign3590_e3525;

        s.store_scalar(320, (if ((((s.v[3] + s.v[310]) - (2.0 * p.p192)) + p.p197) > 1e-9) { (((var_l_i + var_dellps) - (2.0 * p.p192)) + p.p197) } else { 1e-9 }));

        let assign3610_e3550: f64 = (var_w_i + var_delwod);
        let assign3610_e3553: f64 = (2.0 * p.p196);
        let assign3610_e3554: f64 = (assign3610_e3550 - assign3610_e3553);
        let assign3610_e3556: f64 = (assign3610_e3554 + p.p198);
        let (assign3610_e3569,) = {
    if (assign3610_e3556 > 1e-9) {
        let assign3610_e3561: f64 = (var_w_i + var_delwod);
        let assign3610_e3564: f64 = (2.0 * p.p196);
        let assign3610_e3565: f64 = (assign3610_e3561 - assign3610_e3564);
        let assign3610_e3567: f64 = (assign3610_e3565 + p.p198);
        (assign3610_e3567,)
    } else {
        (1e-9,)
    }
};
        var_wecv = assign3610_e3569;

        let assign3620_e3572: f64 = (var_wecv / 1e-6);
        var_iiwecv = assign3620_e3572;

        let assign3630_e3575: f64 = (var_l_i + var_dellps);
        let assign3630_e3577: f64 = (assign3630_e3575 + p.p197);
        let (assign3630_e3586,) = {
    if (assign3630_e3577 > 1e-9) {
        let assign3630_e3582: f64 = (var_l_i + var_dellps);
        let assign3630_e3584: f64 = (assign3630_e3582 + p.p197);
        (assign3630_e3584,)
    } else {
        (1e-9,)
    }
};
        var_lcv = assign3630_e3586;

        *var_abdrain_i_slot = var_abdrain_i;
        *var_absource_i_slot = var_absource_i;
        *var_ad_i_slot = var_ad_i;
        *var_as_i_slot = var_as_i;
        *var_atatbot_d_slot = var_atatbot_d;
        *var_atatgat_d_slot = var_atatgat_d;
        *var_atatsti_d_slot = var_atatsti_d;
        *var_btatpartbot_d_slot = var_btatpartbot_d;
        *var_btatpartgat_d_slot = var_btatpartgat_d;
        *var_btatpartsti_d_slot = var_btatpartsti_d;
        *var_dellps_slot = var_dellps;
        *var_deltaebot_d_slot = var_deltaebot_d;
        *var_deltaegat_d_slot = var_deltaegat_d;
        *var_deltaesti_d_slot = var_deltaesti_d;
        *var_delwod_slot = var_delwod;
        *var_fbbtbot_d_slot = var_fbbtbot_d;
        *var_fbbtgat_d_slot = var_fbbtgat_d;
        *var_fbbtgat_d_db0_slot = var_fbbtgat_d_db0;
        *var_fbbtgat_d_db1_slot = var_fbbtgat_d_db1;
        *var_fbbtgat_d_db2_slot = var_fbbtgat_d_db2;
        *var_fbbtgat_d_db3_slot = var_fbbtgat_d_db3;
        *var_fbbtgat_d_db4_slot = var_fbbtgat_d_db4;
        *var_fbbtgat_d_db5_slot = var_fbbtgat_d_db5;
        *var_fbbtgat_d_db6_slot = var_fbbtgat_d_db6;
        *var_fbbtgat_d_dn0_slot = var_fbbtgat_d_dn0;
        *var_fbbtgat_d_dn1_slot = var_fbbtgat_d_dn1;
        *var_fbbtgat_d_dn10_slot = var_fbbtgat_d_dn10;
        *var_fbbtgat_d_dn11_slot = var_fbbtgat_d_dn11;
        *var_fbbtgat_d_dn12_slot = var_fbbtgat_d_dn12;
        *var_fbbtgat_d_dn2_slot = var_fbbtgat_d_dn2;
        *var_fbbtgat_d_dn3_slot = var_fbbtgat_d_dn3;
        *var_fbbtgat_d_dn4_slot = var_fbbtgat_d_dn4;
        *var_fbbtgat_d_dn5_slot = var_fbbtgat_d_dn5;
        *var_fbbtgat_d_dn6_slot = var_fbbtgat_d_dn6;
        *var_fbbtgat_d_dn7_slot = var_fbbtgat_d_dn7;
        *var_fbbtgat_d_dn8_slot = var_fbbtgat_d_dn8;
        *var_fbbtgat_d_dn9_slot = var_fbbtgat_d_dn9;
        *var_fbbtsti_d_slot = var_fbbtsti_d;
        *var_ftdgat2nd_d_slot = var_ftdgat2nd_d;
        *var_guard28_slot = var_guard28;
        *var_guard29_slot = var_guard29;
        *var_iae_slot = var_iae;
        *var_iiae_slot = var_iiae;
        *var_iiwe_slot = var_iiwe;
        *var_iiwecv_slot = var_iiwecv;
        *var_il_slot = var_il;
        *var_ile_slot = var_ile;
        *var_ile2_slot = var_ile2;
        *var_invnf_slot = var_invnf;
        *var_iw_slot = var_iw;
        *var_iwe_slot = var_iwe;
        *var_jw_i_slot = var_jw_i;
        *var_l_i_slot = var_l_i;
        *var_lcv_slot = var_lcv;
        *var_le_slot = var_le;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lssource_i_slot = var_lssource_i;
        *var_nf_i_slot = var_nf_i;
        *var_ngcon_i_slot = var_ngcon_i;
        *var_pd_i_slot = var_pd_i;
        *var_phigdgat2nd_d_slot = var_phigdgat2nd_d;
        *var_ps_i_slot = var_ps_i;
        *var_sa_i_slot = var_sa_i;
        *var_sb_i_slot = var_sb_i;
        *var_sc_i_slot = var_sc_i;
        *var_sca_i_slot = var_sca_i;
        *var_scb_i_slot = var_scb_i;
        *var_scc_i_slot = var_scc_i;
        *var_sd_i_slot = var_sd_i;
        *var_ubigat2nd_d_slot = var_ubigat2nd_d;
        *var_ubigat_d_slot = var_ubigat_d;
        *var_ubisti_d_slot = var_ubisti_d;
        *var_vbibot_d_slot = var_vbibot_d;
        *var_vbigat2nd_d_slot = var_vbigat2nd_d;
        *var_vbigat_d_slot = var_vbigat_d;
        *var_vbiinvbot_d_slot = var_vbiinvbot_d;
        *var_vbiinvgat2nd_d_slot = var_vbiinvgat2nd_d;
        *var_vbiinvgat_d_slot = var_vbiinvgat_d;
        *var_vbiinvsti_d_slot = var_vbiinvsti_d;
        *var_vbisti_d_slot = var_vbisti_d;
        *var_w_i_slot = var_w_i;
        *var_we_slot = var_we;
        *var_wecv_slot = var_wecv;
        *var_xgw_i_slot = var_xgw_i;
    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dellps: f64,
        var_delwod: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_l_i: f64,
        var_lcv: f64,
        var_w_i: f64,
        var_we: f64,
        var_xgw_i: f64,
        var_a1_p_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_iilcv_slot: &mut f64,
        var_iiwcv_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_l_f_slot: &mut f64,
        var_l_slif_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_npcke_slot: &mut f64,
        var_nsub0e_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_rbulk_p_slot: &mut f64,
        var_rde_p_slot: &mut f64,
        var_rg_p_slot: &mut f64,
        var_rjund_p_slot: &mut f64,
        var_rjuns_p_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rse_p_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rth_p_slot: &mut f64,
        var_rwell_p_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strth_p_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatt_p_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vp_p_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_w_f_slot: &mut f64,
        var_wcv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xgwe_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a2_p: f64 = *var_a2_p_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_iilcv: f64 = *var_iilcv_slot;
        let mut var_iiwcv: f64 = *var_iiwcv_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_l_f: f64 = *var_l_f_slot;
        let mut var_l_slif: f64 = *var_l_slif_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_npcke: f64 = *var_npcke_slot;
        let mut var_nsub0e: f64 = *var_nsub0e_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_rbulk_p: f64 = *var_rbulk_p_slot;
        let mut var_rde_p: f64 = *var_rde_p_slot;
        let mut var_rg_p: f64 = *var_rg_p_slot;
        let mut var_rjund_p: f64 = *var_rjund_p_slot;
        let mut var_rjuns_p: f64 = *var_rjuns_p_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rse_p: f64 = *var_rse_p_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rth_p: f64 = *var_rth_p_slot;
        let mut var_rwell_p: f64 = *var_rwell_p_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strth_p: f64 = *var_strth_p_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_w_f: f64 = *var_w_f_slot;
        let mut var_wcv: f64 = *var_wcv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xgwe: f64 = *var_xgwe_slot;

        let assign3640_e3589: f64 = (var_w_i + var_delwod);
        let assign3640_e3591: f64 = (assign3640_e3589 + p.p198);
        let (assign3640_e3600,) = {
    if (assign3640_e3591 > 1e-9) {
        let assign3640_e3596: f64 = (var_w_i + var_delwod);
        let assign3640_e3598: f64 = (assign3640_e3596 + p.p198);
        (assign3640_e3598,)
    } else {
        (1e-9,)
    }
};
        var_wcv = assign3640_e3600;

        let assign3650_e3603: f64 = (var_lcv / 1e-6);
        var_iilcv = assign3650_e3603;

        let assign3660_e3606: f64 = (var_wcv / 1e-6);
        var_iiwcv = assign3660_e3606;

        let assign3670_e3609: f64 = (var_l_i + var_dellps);
        let (assign3670_e3616,) = {
    if (assign3670_e3609 > 1e-9) {
        let assign3670_e3614: f64 = (var_l_i + var_dellps);
        (assign3670_e3614,)
    } else {
        (1e-9,)
    }
};
        var_l_f = assign3670_e3616;

        let assign3680_e3619: f64 = (var_l_f + p.p444);
        let (assign3680_e3626,) = {
    if (assign3680_e3619 > 1e-9) {
        let assign3680_e3624: f64 = (var_l_f + p.p444);
        (assign3680_e3624,)
    } else {
        (1e-9,)
    }
};
        var_l_slif = assign3680_e3626;

        let assign3690_e3629: f64 = (var_w_i + var_delwod);
        let (assign3690_e3636,) = {
    if (assign3690_e3629 > 1e-9) {
        let assign3690_e3634: f64 = (var_w_i + var_delwod);
        (assign3690_e3634,)
    } else {
        (1e-9,)
    }
};
        var_w_f = assign3690_e3636;

        let assign3700_e3640: f64 = (0.5 * var_delwod);
        let assign3700_e3641: f64 = (var_xgw_i - assign3700_e3640);
        let (assign3700_e3650,) = {
    if (assign3700_e3641 > 1e-9) {
        let assign3700_e3647: f64 = (0.5 * var_delwod);
        let assign3700_e3648: f64 = (var_xgw_i - assign3700_e3647);
        (assign3700_e3648,)
    } else {
        (1e-9,)
    }
};
        var_xgwe = assign3700_e3650;

        var_vfb_p = p.p56;

        var_stvfb_p = p.p57;

        var_st2vfb_p = p.p58;

        var_tox_p = p.p59;

        var_epsrox_p = p.p60;

        var_neff_p = p.p61;

        var_gfacnud_p = p.p62;

        var_vsbnud_p = p.p63;

        var_dvsbnud_p = p.p64;

        var_dphib_p = p.p65;

        var_np_p = p.p66;

        var_toxov_p = p.p67;

        var_toxovd_p = p.p68;

        var_nov_p = p.p69;

        var_novd_p = p.p70;

        var_ct_p = p.p71;

        var_ctg_p = p.p73;

        var_ctb_p = p.p72;

        var_stct_p = p.p74;

        var_psce_p = p.p78;

        var_psced_p = p.p80;

        var_psceb_p = p.p79;

        var_cf_p = p.p75;

        var_cfd_p = p.p77;

        var_cfb_p = p.p76;

        var_betn_p = p.p81;

        var_stbet_p = p.p82;

        var_mue_p = p.p83;

        var_stmue_p = p.p84;

        var_themu_p = p.p85;

        var_stthemu_p = p.p86;

        var_cs_p = p.p87;

        var_stcs_p = p.p88;

        var_thecs_p = p.p89;

        var_stthecs_p = p.p90;

        var_xcor_p = p.p91;

        var_stxcor_p = p.p92;

        var_feta_p = p.p93;

        var_rs_p = p.p94;

        var_strs_p = p.p95;

        var_rsb_p = p.p96;

        var_rsg_p = p.p97;

        var_thesat_p = p.p98;

        var_stthesat_p = p.p99;

        var_thesatb_p = p.p100;

        var_thesatg_p = p.p101;

        var_thesatt_p = p.p102;

        var_ax_p = p.p103;

        var_alp_p = p.p104;

        var_alp1_p = p.p105;

        var_alp2_p = p.p106;

        var_vp_p = p.p107;

        var_a1_p = p.p108;

        var_a2_p = p.p109;

        var_sta2_p = p.p110;

        var_a3_p = p.p111;

        var_a4_p = p.p112;

        var_imaxii_p = p.p113;

        var_gco_p = p.p114;

        var_iginv_p = p.p115;

        var_igov_p = p.p116;

        var_igovd_p = p.p117;

        var_stig_p = p.p118;

        var_gc2_p = p.p119;

        var_gc3_p = p.p120;

        var_gc2ov_p = p.p119;

        let assign4370_e3718: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4370_e3720: f64 = if assign4370_e3718 == 1.0 { 1.0 } else { 0.0 };
        var_guard30 = assign4370_e3720;

        let (assign4380_e3724,) = {
    if (var_guard30 != 0.0) {
        (p.p121,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign4380_e3724;

        var_gc3ov_p = p.p120;

        let assign4400_e3727: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4400_e3729: f64 = if assign4400_e3727 == 1.0 { 1.0 } else { 0.0 };
        var_guard31 = assign4400_e3729;

        let (assign4410_e3733,) = {
    if (var_guard31 != 0.0) {
        (p.p122,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign4410_e3733;

        var_gc2ovd_p = var_gc2ov_p;

        let assign4430_e3736: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4430_e3738: f64 = if assign4430_e3736 == 1.0 { 1.0 } else { 0.0 };
        var_guard32 = assign4430_e3738;

        let (assign4440_e3742,) = {
    if (var_guard32 != 0.0) {
        (p.p123,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign4440_e3742;

        var_gc3ovd_p = var_gc3ov_p;

        let assign4460_e3745: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4460_e3747: f64 = if assign4460_e3745 == 1.0 { 1.0 } else { 0.0 };
        var_guard33 = assign4460_e3747;

        let (assign4470_e3751,) = {
    if (var_guard33 != 0.0) {
        (p.p124,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign4470_e3751;

        var_chib_p = p.p125;

        var_agidl_p = p.p126;

        var_agidld_p = p.p127;

        var_bgidl_p = p.p128;

        var_bgidld_p = p.p129;

        var_stbgidl_p = p.p130;

        var_stbgidld_p = p.p131;

        var_cgidl_p = p.p132;

        var_cgidld_p = p.p133;

        s.store_scalar(118, p.p134);

        var_delvtac_p = p.p135;

        var_facneffac_p = p.p136;

        var_thesatac_p = p.p98;

        let assign4610_e3766: f64 = if param_given[137] { 1.0 } else { 0.0 };
        let assign4610_e3768: f64 = if assign4610_e3766 == 1.0 { 1.0 } else { 0.0 };
        var_guard34 = assign4610_e3768;

        let (assign4620_e3772,) = {
    if (var_guard34 != 0.0) {
        (p.p137,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign4620_e3772;

        var_axac_p = p.p103;

        let assign4640_e3775: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4640_e3777: f64 = if assign4640_e3775 == 1.0 { 1.0 } else { 0.0 };
        var_guard35 = assign4640_e3777;

        let (assign4650_e3781,) = {
    if (var_guard35 != 0.0) {
        (p.p138,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4650_e3781;

        var_alpac_p = p.p139;

        var_alp1ac_p = p.p140;

        var_cgov_p = p.p141;

        var_cgovd_p = p.p142;

        var_fcgovacc_p = p.p143;

        var_fcgovaccd_p = p.p144;

        var_cgovaccg_p = p.p145;

        var_cgbov_p = p.p146;

        var_cinr_p = p.p147;

        var_cinrd_p = p.p148;

        var_dvfbinr_p = p.p149;

        var_fcinrdep_p = p.p150;

        var_fcinracc_p = p.p151;

        var_axinr_p = p.p152;

        var_cfr_p = p.p153;

        var_cfrd_p = p.p154;

        s.store_scalar(139, p.p155);

        s.store_scalar(140, p.p156);

        var_vfbedge_p = p.p161;

        var_stvfbedge_p = p.p162;

        var_dphibedge_p = p.p163;

        var_neffedge_p = p.p164;

        var_ctedge_p = p.p165;

        var_betnedge_p = p.p166;

        var_stbetedge_p = p.p167;

        var_psceedge_p = p.p168;

        var_pscebedge_p = p.p169;

        var_pscededge_p = p.p170;

        var_cfedge_p = p.p171;

        var_cfdedge_p = p.p173;

        var_cfbedge_p = p.p172;

        var_rg_p = p.p179;

        var_rse_p = p.p180;

        var_rde_p = p.p181;

        var_rwell_p = p.p183;

        var_rbulk_p = p.p182;

        var_rjuns_p = p.p184;

        var_rjund_p = p.p185;

        var_rth_p = p.p186;

        var_cth_p = p.p187;

        var_strth_p = p.p188;

        let assign5160_e3834: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign5160_e3834;

        let (assign5170_e3852,) = {
    if (var_guard36 != 0.0) {
        let assign5170_e3840: f64 = (var_ile).powf(p.p201);
        let assign5170_e3841: f64 = (p.p200 * assign5170_e3840);
        let assign5170_e3842: f64 = (p.p199 + assign5170_e3841);
        let assign5170_e3845: f64 = (p.p202 * var_iwe);
        let assign5170_e3846: f64 = (assign5170_e3842 + assign5170_e3845);
        let assign5170_e3849: f64 = (p.p203 * var_iae);
        let assign5170_e3850: f64 = (assign5170_e3846 + assign5170_e3849);
        (assign5170_e3850,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign5170_e3852;

        let (assign5180_e3868,) = {
    if (var_guard36 != 0.0) {
        let assign5180_e3857: f64 = (p.p205 * var_ile);
        let assign5180_e3858: f64 = (p.p204 + assign5180_e3857);
        let assign5180_e3861: f64 = (p.p206 * var_iwe);
        let assign5180_e3862: f64 = (assign5180_e3858 + assign5180_e3861);
        let assign5180_e3865: f64 = (p.p207 * var_iae);
        let assign5180_e3866: f64 = (assign5180_e3862 + assign5180_e3865);
        (assign5180_e3866,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign5180_e3868;

        let (assign5190_e3872,) = {
    if (var_guard36 != 0.0) {
        (p.p208,)
    } else {
        (var_st2vfb_p,)
    }
};
        var_st2vfb_p = assign5190_e3872;

        let (assign5200_e3876,) = {
    if (var_guard36 != 0.0) {
        (p.p209,)
    } else {
        (var_tox_p,)
    }
};
        var_tox_p = assign5200_e3876;

        let (assign5210_e3880,) = {
    if (var_guard36 != 0.0) {
        (p.p210,)
    } else {
        (var_epsrox_p,)
    }
};
        var_epsrox_p = assign5210_e3880;

        let (assign5220_e3913,) = {
    if (var_guard36 != 0.0) {
        let assign5220_e3886: f64 = (p.p212 * var_iwe);
        let assign5220_e3890: f64 = (var_we / p.p213);
        let assign5220_e3891: f64 = (1.0 + assign5220_e3890);
        let assign5220_e3892: f64 = (assign5220_e3891).ln();
        let assign5220_e3893: f64 = (assign5220_e3886 * assign5220_e3892);
        let assign5220_e3894: f64 = (1.0 + assign5220_e3893);
        let (assign5220_e3910,) = {
            if (assign5220_e3894 > 0.001) {
                let assign5220_e3900: f64 = (p.p212 * var_iwe);
                let assign5220_e3904: f64 = (var_we / p.p213);
                let assign5220_e3905: f64 = (1.0 + assign5220_e3904);
                let assign5220_e3906: f64 = (assign5220_e3905).ln();
                let assign5220_e3907: f64 = (assign5220_e3900 * assign5220_e3906);
                let assign5220_e3908: f64 = (1.0 + assign5220_e3907);
                (assign5220_e3908,)
            } else {
                (0.001,)
            }
        };
        let assign5220_e3911: f64 = (p.p211 * assign5220_e3910);
        (assign5220_e3911,)
    } else {
        (var_nsub0e,)
    }
};
        var_nsub0e = assign5220_e3913;

        let (assign5230_e3946,) = {
    if (var_guard36 != 0.0) {
        let assign5230_e3919: f64 = (p.p215 * var_iwe);
        let assign5230_e3923: f64 = (var_we / p.p216);
        let assign5230_e3924: f64 = (1.0 + assign5230_e3923);
        let assign5230_e3925: f64 = (assign5230_e3924).ln();
        let assign5230_e3926: f64 = (assign5230_e3919 * assign5230_e3925);
        let assign5230_e3927: f64 = (1.0 + assign5230_e3926);
        let (assign5230_e3943,) = {
            if (assign5230_e3927 > 0.001) {
                let assign5230_e3933: f64 = (p.p215 * var_iwe);
                let assign5230_e3937: f64 = (var_we / p.p216);
                let assign5230_e3938: f64 = (1.0 + assign5230_e3937);
                let assign5230_e3939: f64 = (assign5230_e3938).ln();
                let assign5230_e3940: f64 = (assign5230_e3933 * assign5230_e3939);
                let assign5230_e3941: f64 = (1.0 + assign5230_e3940);
                (assign5230_e3941,)
            } else {
                (0.001,)
            }
        };
        let assign5230_e3944: f64 = (p.p214 * assign5230_e3943);
        (assign5230_e3944,)
    } else {
        (var_npcke,)
    }
};
        var_npcke = assign5230_e3946;

        *var_a1_p_slot = var_a1_p;
        *var_a2_p_slot = var_a2_p;
        *var_a3_p_slot = var_a3_p;
        *var_a4_p_slot = var_a4_p;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp_p_slot = var_alp_p;
        *var_alpac_p_slot = var_alpac_p;
        *var_ax_p_slot = var_ax_p;
        *var_axac_p_slot = var_axac_p;
        *var_axinr_p_slot = var_axinr_p;
        *var_betn_p_slot = var_betn_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_cf_p_slot = var_cf_p;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_chib_p_slot = var_chib_p;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cs_p_slot = var_cs_p;
        *var_ct_p_slot = var_ct_p;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctg_p_slot = var_ctg_p;
        *var_cth_p_slot = var_cth_p;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_feta_p_slot = var_feta_p;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gco_p_slot = var_gco_p;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_guard30_slot = var_guard30;
        *var_guard31_slot = var_guard31;
        *var_guard32_slot = var_guard32;
        *var_guard33_slot = var_guard33;
        *var_guard34_slot = var_guard34;
        *var_guard35_slot = var_guard35;
        *var_guard36_slot = var_guard36;
        *var_iginv_p_slot = var_iginv_p;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_iilcv_slot = var_iilcv;
        *var_iiwcv_slot = var_iiwcv;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_l_f_slot = var_l_f;
        *var_l_slif_slot = var_l_slif;
        *var_mue_p_slot = var_mue_p;
        *var_neff_p_slot = var_neff_p;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
        *var_npcke_slot = var_npcke;
        *var_nsub0e_slot = var_nsub0e;
        *var_psce_p_slot = var_psce_p;
        *var_psceb_p_slot = var_psceb_p;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_psced_p_slot = var_psced_p;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_rbulk_p_slot = var_rbulk_p;
        *var_rde_p_slot = var_rde_p;
        *var_rg_p_slot = var_rg_p;
        *var_rjund_p_slot = var_rjund_p;
        *var_rjuns_p_slot = var_rjuns_p;
        *var_rs_p_slot = var_rs_p;
        *var_rsb_p_slot = var_rsb_p;
        *var_rse_p_slot = var_rse_p;
        *var_rsg_p_slot = var_rsg_p;
        *var_rth_p_slot = var_rth_p;
        *var_rwell_p_slot = var_rwell_p;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_sta2_p_slot = var_sta2_p;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stcs_p_slot = var_stcs_p;
        *var_stct_p_slot = var_stct_p;
        *var_stig_p_slot = var_stig_p;
        *var_stmue_p_slot = var_stmue_p;
        *var_strs_p_slot = var_strs_p;
        *var_strth_p_slot = var_strth_p;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_thecs_p_slot = var_thecs_p;
        *var_themu_p_slot = var_themu_p;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_tox_p_slot = var_tox_p;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vp_p_slot = var_vp_p;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_w_f_slot = var_w_f;
        *var_wcv_slot = var_wcv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xgwe_slot = var_xgwe;
    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_npcke: f64,
        var_nsub0e: f64,
        var_we: f64,
        var_aa_slot: &mut f64,
        var_bb_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_fbet1e_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard38_slot: &mut f64,
        var_gwe_slot: &mut f64,
        var_lp1e_slot: &mut f64,
        var_lpcke_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
    ) {
        let mut var_aa: f64 = *var_aa_slot;
        let mut var_bb: f64 = *var_bb_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_fbet1e: f64 = *var_fbet1e_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard38: f64 = *var_guard38_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
        let mut var_lp1e: f64 = *var_lp1e_slot;
        let mut var_lpcke: f64 = *var_lpcke_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;

        let (assign5240_e3979,) = {
    if (var_guard36 != 0.0) {
        let assign5240_e3952: f64 = (p.p218 * var_iwe);
        let assign5240_e3956: f64 = (var_we / p.p216);
        let assign5240_e3957: f64 = (1.0 + assign5240_e3956);
        let assign5240_e3958: f64 = (assign5240_e3957).ln();
        let assign5240_e3959: f64 = (assign5240_e3952 * assign5240_e3958);
        let assign5240_e3960: f64 = (1.0 + assign5240_e3959);
        let (assign5240_e3976,) = {
            if (assign5240_e3960 > 0.001) {
                let assign5240_e3966: f64 = (p.p218 * var_iwe);
                let assign5240_e3970: f64 = (var_we / p.p216);
                let assign5240_e3971: f64 = (1.0 + assign5240_e3970);
                let assign5240_e3972: f64 = (assign5240_e3971).ln();
                let assign5240_e3973: f64 = (assign5240_e3966 * assign5240_e3972);
                let assign5240_e3974: f64 = (1.0 + assign5240_e3973);
                (assign5240_e3974,)
            } else {
                (0.001,)
            }
        };
        let assign5240_e3977: f64 = (p.p217 * assign5240_e3976);
        (assign5240_e3977,)
    } else {
        (var_lpcke,)
    }
};
        var_lpcke = assign5240_e3979;

        let assign5250_e3983: f64 = (2.0 * var_lpcke);
        let assign5250_e3984: f64 = if var_le > assign5250_e3983 { 1.0 } else { 0.0 };
        var_guard37 = assign5250_e3984;

        let (assign5260_e3990,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        (75000000000.0,)
    } else {
        (var_aa,)
    }
};
        var_aa = assign5260_e3990;

        let (assign5270_e4004,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5270_e3997: f64 = (0.5 * var_npcke);
        let assign5270_e3998: f64 = (var_nsub0e + assign5270_e3997);
        let assign5270_e3999: f64 = (assign5270_e3998).sqrt();
        let assign5270_e4001: f64 = (var_nsub0e).sqrt();
        let assign5270_e4002: f64 = (assign5270_e3999 - assign5270_e4001);
        (assign5270_e4002,)
    } else {
        (var_bb,)
    }
};
        var_bb = assign5270_e4004;

        let (assign5280_e4029,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5280_e4009: f64 = (var_nsub0e).sqrt();
        let assign5280_e4014: f64 = (2.0 * var_lpcke);
        let assign5280_e4016: f64 = (assign5280_e4014 / var_le);
        let assign5280_e4019: f64 = (var_bb / var_aa);
        let assign5280_e4020: f64 = (assign5280_e4019).exp();
        let assign5280_e4022: f64 = (assign5280_e4020 - 1.0);
        let assign5280_e4023: f64 = (assign5280_e4016 * assign5280_e4022);
        let assign5280_e4024: f64 = (1.0 + assign5280_e4023);
        let assign5280_e4025: f64 = (assign5280_e4024).ln();
        let assign5280_e4026: f64 = (var_aa * assign5280_e4025);
        let assign5280_e4027: f64 = (assign5280_e4009 + assign5280_e4026);
        (assign5280_e4027,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5280_e4029;

        let (assign5290_e4037,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5290_e4035: f64 = (var_nsub * var_nsub);
        (assign5290_e4035,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5290_e4037;

        let assign5300_e4040: f64 = if var_le >= var_lpcke { 1.0 } else { 0.0 };
        var_guard38 = assign5300_e4040;

        let (assign5310_e4055,) = {
    if (((var_guard36 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 != 0.0)) {
        let assign5310_e4050: f64 = (var_npcke * var_lpcke);
        let assign5310_e4052: f64 = (assign5310_e4050 / var_le);
        let assign5310_e4053: f64 = (var_nsub0e + assign5310_e4052);
        (assign5310_e4053,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5310_e4055;

        let (assign5320_e4073,) = {
    if (((var_guard36 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 == 0.0)) {
        let assign5320_e4068: f64 = (var_le / var_lpcke);
        let assign5320_e4069: f64 = (2.0 - assign5320_e4068);
        let assign5320_e4070: f64 = (var_npcke * assign5320_e4069);
        let assign5320_e4071: f64 = (var_nsub0e + assign5320_e4070);
        (assign5320_e4071,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5320_e4073;

        let (assign5330_e4087,) = {
    if (var_guard36 != 0.0) {
        let assign5330_e4079: f64 = (p.p219 * var_ile);
        let assign5330_e4080: f64 = (1.0 - assign5330_e4079);
        let assign5330_e4083: f64 = (p.p220 * var_ile2);
        let assign5330_e4084: f64 = (assign5330_e4080 - assign5330_e4083);
        let assign5330_e4085: f64 = (var_nsub * assign5330_e4084);
        (assign5330_e4085,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign5330_e4087;

        let (assign5340_e4105,) = {
    if (var_guard36 != 0.0) {
        let assign5340_e4093: f64 = (var_ile).powf(p.p223);
        let assign5340_e4094: f64 = (p.p222 * assign5340_e4093);
        let assign5340_e4095: f64 = (p.p221 + assign5340_e4094);
        let assign5340_e4098: f64 = (p.p224 * var_iwe);
        let assign5340_e4099: f64 = (assign5340_e4095 + assign5340_e4098);
        let assign5340_e4102: f64 = (p.p225 * var_iae);
        let assign5340_e4103: f64 = (assign5340_e4099 + assign5340_e4102);
        (assign5340_e4103,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign5340_e4105;

        let (assign5350_e4109,) = {
    if (var_guard36 != 0.0) {
        (p.p226,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign5350_e4109;

        let (assign5360_e4113,) = {
    if (var_guard36 != 0.0) {
        (p.p227,)
    } else {
        (var_dvsbnud_p,)
    }
};
        var_dvsbnud_p = assign5360_e4113;

        let (assign5370_e4131,) = {
    if (var_guard36 != 0.0) {
        let assign5370_e4119: f64 = (var_ile).powf(p.p230);
        let assign5370_e4120: f64 = (p.p229 * assign5370_e4119);
        let assign5370_e4121: f64 = (p.p228 + assign5370_e4120);
        let assign5370_e4124: f64 = (p.p231 * var_iwe);
        let assign5370_e4125: f64 = (assign5370_e4121 + assign5370_e4124);
        let assign5370_e4128: f64 = (p.p232 * var_iae);
        let assign5370_e4129: f64 = (assign5370_e4125 + assign5370_e4128);
        (assign5370_e4129,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign5370_e4131;

        let (assign5380_e4150,) = {
    if (var_guard36 != 0.0) {
        let assign5380_e4138: f64 = (p.p234 * var_ile);
        let assign5380_e4139: f64 = (1.0 + assign5380_e4138);
        let (assign5380_e4147,) = {
            if (1e-6 > assign5380_e4139) {
                (1e-6,)
            } else {
                let assign5380_e4145: f64 = (p.p234 * var_ile);
                let assign5380_e4146: f64 = (1.0 + assign5380_e4145);
                (assign5380_e4146,)
            }
        };
        let assign5380_e4148: f64 = (p.p233 * assign5380_e4147);
        (assign5380_e4148,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign5380_e4150;

        let (assign5390_e4154,) = {
    if (var_guard36 != 0.0) {
        (p.p235,)
    } else {
        (var_toxov_p,)
    }
};
        var_toxov_p = assign5390_e4154;

        let (assign5400_e4158,) = {
    if (var_guard36 != 0.0) {
        (p.p236,)
    } else {
        (var_toxovd_p,)
    }
};
        var_toxovd_p = assign5400_e4158;

        let (assign5410_e4162,) = {
    if (var_guard36 != 0.0) {
        (p.p239,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign5410_e4162;

        let (assign5420_e4166,) = {
    if (var_guard36 != 0.0) {
        (p.p240,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign5420_e4166;

        let (assign5430_e4188,) = {
    if (var_guard36 != 0.0) {
        let assign5430_e4172: f64 = (var_ile).powf(p.p243);
        let assign5430_e4173: f64 = (p.p242 * assign5430_e4172);
        let assign5430_e4174: f64 = (p.p241 + assign5430_e4173);
        let assign5430_e4178: f64 = (p.p244 * var_iwe);
        let assign5430_e4179: f64 = (1.0 + assign5430_e4178);
        let assign5430_e4180: f64 = (assign5430_e4174 * assign5430_e4179);
        let assign5430_e4184: f64 = (p.p245 * var_iae);
        let assign5430_e4185: f64 = (1.0 + assign5430_e4184);
        let assign5430_e4186: f64 = (assign5430_e4180 * assign5430_e4185);
        (assign5430_e4186,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign5430_e4188;

        let (assign5440_e4192,) = {
    if (var_guard36 != 0.0) {
        (p.p247,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign5440_e4192;

        let (assign5450_e4196,) = {
    if (var_guard36 != 0.0) {
        (p.p246,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign5450_e4196;

        let (assign5460_e4200,) = {
    if (var_guard36 != 0.0) {
        (p.p248,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign5460_e4200;

        let (assign5470_e4214,) = {
    if (var_guard36 != 0.0) {
        let assign5470_e4205: f64 = (var_ile).powf(p.p250);
        let assign5470_e4206: f64 = (p.p249 * assign5470_e4205);
        let assign5470_e4210: f64 = (p.p251 * var_iwe);
        let assign5470_e4211: f64 = (1.0 + assign5470_e4210);
        let assign5470_e4212: f64 = (assign5470_e4206 * assign5470_e4211);
        (assign5470_e4212,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign5470_e4214;

        let (assign5480_e4218,) = {
    if (var_guard36 != 0.0) {
        (p.p253,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign5480_e4218;

        let (assign5490_e4222,) = {
    if (var_guard36 != 0.0) {
        (p.p252,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign5490_e4222;

        let (assign5500_e4236,) = {
    if (var_guard36 != 0.0) {
        let assign5500_e4227: f64 = (var_ile).powf(p.p255);
        let assign5500_e4228: f64 = (p.p254 * assign5500_e4227);
        let assign5500_e4232: f64 = (p.p256 * var_iwe);
        let assign5500_e4233: f64 = (1.0 + assign5500_e4232);
        let assign5500_e4234: f64 = (assign5500_e4228 * assign5500_e4233);
        (assign5500_e4234,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign5500_e4236;

        let (assign5510_e4240,) = {
    if (var_guard36 != 0.0) {
        (p.p258,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign5510_e4240;

        let (assign5520_e4244,) = {
    if (var_guard36 != 0.0) {
        (p.p257,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign5520_e4244;

        let (assign5530_e4254,) = {
    if (var_guard36 != 0.0) {
        let assign5530_e4250: f64 = (p.p261 * var_iwe);
        let assign5530_e4251: f64 = (1.0 + assign5530_e4250);
        let assign5530_e4252: f64 = (p.p260 * assign5530_e4251);
        (assign5530_e4252,)
    } else {
        (var_fbet1e,)
    }
};
        var_fbet1e = assign5530_e4254;

        let (assign5540_e4273,) = {
    if (var_guard36 != 0.0) {
        let assign5540_e4260: f64 = (p.p263 * var_iwe);
        let assign5540_e4261: f64 = (1.0 + assign5540_e4260);
        let (assign5540_e4270,) = {
            if (assign5540_e4261 > 0.001) {
                let assign5540_e4267: f64 = (p.p263 * var_iwe);
                let assign5540_e4268: f64 = (1.0 + assign5540_e4267);
                (assign5540_e4268,)
            } else {
                (0.001,)
            }
        };
        let assign5540_e4271: f64 = (p.p262 * assign5540_e4270);
        (assign5540_e4271,)
    } else {
        (var_lp1e,)
    }
};
        var_lp1e = assign5540_e4273;

        let (assign5550_e4305,) = {
    if (var_guard36 != 0.0) {
        let assign5550_e4278: f64 = (var_fbet1e * var_lp1e);
        let assign5550_e4280: f64 = (assign5550_e4278 / var_le);
        let assign5550_e4283: f64 = (-var_le);
        let assign5550_e4285: f64 = (assign5550_e4283 / var_lp1e);
        let assign5550_e4286: f64 = (assign5550_e4285).exp();
        let assign5550_e4287: f64 = (1.0 - assign5550_e4286);
        let assign5550_e4288: f64 = (assign5550_e4280 * assign5550_e4287);
        let assign5550_e4289: f64 = (1.0 + assign5550_e4288);
        let assign5550_e4292: f64 = (p.p264 * p.p265);
        let assign5550_e4294: f64 = (assign5550_e4292 / var_le);
        let assign5550_e4297: f64 = (-var_le);
        let assign5550_e4299: f64 = (assign5550_e4297 / p.p265);
        let assign5550_e4300: f64 = (assign5550_e4299).exp();
        let assign5550_e4301: f64 = (1.0 - assign5550_e4300);
        let assign5550_e4302: f64 = (assign5550_e4294 * assign5550_e4301);
        let assign5550_e4303: f64 = (assign5550_e4289 + assign5550_e4302);
        (assign5550_e4303,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5550_e4305;

        let (assign5560_e4314,) = {
    if (var_guard36 != 0.0) {
        let (assign5560_e4312,) = {
            if (var_gpe > 1e-15) {
                (var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5560_e4312,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5560_e4314;

        let (assign5570_e4333,) = {
    if (var_guard36 != 0.0) {
        let assign5570_e4319: f64 = (p.p266 * var_iwe);
        let assign5570_e4320: f64 = (1.0 + assign5570_e4319);
        let assign5570_e4323: f64 = (p.p267 * var_iwe);
        let assign5570_e4327: f64 = (var_we / p.p268);
        let assign5570_e4328: f64 = (1.0 + assign5570_e4327);
        let assign5570_e4329: f64 = (assign5570_e4328).ln();
        let assign5570_e4330: f64 = (assign5570_e4323 * assign5570_e4329);
        let assign5570_e4331: f64 = (assign5570_e4320 + assign5570_e4330);
        (assign5570_e4331,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign5570_e4333;

        let (assign5580_e4345,) = {
    if (var_guard36 != 0.0) {
        let assign5580_e4337: f64 = (p.p259 * var_we);
        let assign5580_e4340: f64 = (var_gpe * var_le);
        let assign5580_e4341: f64 = (assign5580_e4337 / assign5580_e4340);
        let assign5580_e4343: f64 = (assign5580_e4341 * var_gwe);
        (assign5580_e4343,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign5580_e4345;

        let (assign5590_e4361,) = {
    if (var_guard36 != 0.0) {
        let assign5590_e4350: f64 = (p.p270 * var_ile);
        let assign5590_e4351: f64 = (p.p269 + assign5590_e4350);
        let assign5590_e4354: f64 = (p.p271 * var_iwe);
        let assign5590_e4355: f64 = (assign5590_e4351 + assign5590_e4354);
        let assign5590_e4358: f64 = (p.p272 * var_iae);
        let assign5590_e4359: f64 = (assign5590_e4355 + assign5590_e4358);
        (assign5590_e4359,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign5590_e4361;

        let (assign5600_e4371,) = {
    if (var_guard36 != 0.0) {
        let assign5600_e4367: f64 = (p.p274 * var_iwe);
        let assign5600_e4368: f64 = (1.0 + assign5600_e4367);
        let assign5600_e4369: f64 = (p.p273 * assign5600_e4368);
        (assign5600_e4369,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign5600_e4371;

        let (assign5610_e4375,) = {
    if (var_guard36 != 0.0) {
        (p.p275,)
    } else {
        (var_stmue_p,)
    }
};
        var_stmue_p = assign5610_e4375;

        let (assign5620_e4379,) = {
    if (var_guard36 != 0.0) {
        (p.p276,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign5620_e4379;

        let (assign5630_e4383,) = {
    if (var_guard36 != 0.0) {
        (p.p277,)
    } else {
        (var_stthemu_p,)
    }
};
        var_stthemu_p = assign5630_e4383;

        *var_aa_slot = var_aa;
        *var_bb_slot = var_bb;
        *var_betn_p_slot = var_betn_p;
        *var_cf_p_slot = var_cf_p;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfd_p_slot = var_cfd_p;
        *var_ct_p_slot = var_ct_p;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctg_p_slot = var_ctg_p;
        *var_dphib_p_slot = var_dphib_p;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_fbet1e_slot = var_fbet1e;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gpe_slot = var_gpe;
        *var_guard37_slot = var_guard37;
        *var_guard38_slot = var_guard38;
        *var_gwe_slot = var_gwe;
        *var_lp1e_slot = var_lp1e;
        *var_lpcke_slot = var_lpcke;
        *var_mue_p_slot = var_mue_p;
        *var_neff_p_slot = var_neff_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
        *var_nsub_slot = var_nsub;
        *var_psce_p_slot = var_psce_p;
        *var_psceb_p_slot = var_psceb_p;
        *var_psced_p_slot = var_psced_p;
        *var_stbet_p_slot = var_stbet_p;
        *var_stct_p_slot = var_stct_p;
        *var_stmue_p_slot = var_stmue_p;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_themu_p_slot = var_themu_p;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_vsbnud_p_slot = var_vsbnud_p;
    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_gpe: f64,
        var_guard36: f64,
        var_gwe: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_a1_p_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_guard39_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatt_p_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_vp_p_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a2_p: f64 = *var_a2_p_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_guard39: f64 = *var_guard39_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;

        let (assign5640_e4405,) = {
    if (var_guard36 != 0.0) {
        let assign5640_e4389: f64 = (var_ile).powf(p.p280);
        let assign5640_e4390: f64 = (p.p279 * assign5640_e4389);
        let assign5640_e4391: f64 = (p.p278 + assign5640_e4390);
        let assign5640_e4395: f64 = (p.p281 * var_iwe);
        let assign5640_e4396: f64 = (1.0 + assign5640_e4395);
        let assign5640_e4397: f64 = (assign5640_e4391 * assign5640_e4396);
        let assign5640_e4401: f64 = (p.p282 * var_iae);
        let assign5640_e4402: f64 = (1.0 + assign5640_e4401);
        let assign5640_e4403: f64 = (assign5640_e4397 * assign5640_e4402);
        (assign5640_e4403,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign5640_e4405;

        let (assign5650_e4409,) = {
    if (var_guard36 != 0.0) {
        (p.p283,)
    } else {
        (var_stcs_p,)
    }
};
        var_stcs_p = assign5650_e4409;

        let (assign5660_e4413,) = {
    if (var_guard36 != 0.0) {
        (p.p284,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign5660_e4413;

        let (assign5670_e4417,) = {
    if (var_guard36 != 0.0) {
        (p.p285,)
    } else {
        (var_stthecs_p,)
    }
};
        var_stthecs_p = assign5670_e4417;

        let (assign5680_e4439,) = {
    if (var_guard36 != 0.0) {
        let assign5680_e4423: f64 = (p.p287 * var_ile);
        let assign5680_e4424: f64 = (1.0 + assign5680_e4423);
        let assign5680_e4425: f64 = (p.p286 * assign5680_e4424);
        let assign5680_e4429: f64 = (p.p288 * var_iwe);
        let assign5680_e4430: f64 = (1.0 + assign5680_e4429);
        let assign5680_e4431: f64 = (assign5680_e4425 * assign5680_e4430);
        let assign5680_e4435: f64 = (p.p289 * var_iae);
        let assign5680_e4436: f64 = (1.0 + assign5680_e4435);
        let assign5680_e4437: f64 = (assign5680_e4431 * assign5680_e4436);
        (assign5680_e4437,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign5680_e4439;

        let (assign5690_e4443,) = {
    if (var_guard36 != 0.0) {
        (p.p290,)
    } else {
        (var_stxcor_p,)
    }
};
        var_stxcor_p = assign5690_e4443;

        let (assign5700_e4447,) = {
    if (var_guard36 != 0.0) {
        (p.p291,)
    } else {
        (var_feta_p,)
    }
};
        var_feta_p = assign5700_e4447;

        let (assign5710_e4459,) = {
    if (var_guard36 != 0.0) {
        let assign5710_e4451: f64 = (p.p292 * var_iwe);
        let assign5710_e4455: f64 = (p.p293 * var_iwe);
        let assign5710_e4456: f64 = (1.0 + assign5710_e4455);
        let assign5710_e4457: f64 = (assign5710_e4451 * assign5710_e4456);
        (assign5710_e4457,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign5710_e4459;

        let (assign5720_e4463,) = {
    if (var_guard36 != 0.0) {
        (p.p294,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign5720_e4463;

        let (assign5730_e4467,) = {
    if (var_guard36 != 0.0) {
        (p.p295,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign5730_e4467;

        let (assign5740_e4471,) = {
    if (var_guard36 != 0.0) {
        (p.p296,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign5740_e4471;

        let (assign5750_e4497,) = {
    if (var_guard36 != 0.0) {
        let assign5750_e4476: f64 = (p.p298 * var_gwe);
        let assign5750_e4478: f64 = (assign5750_e4476 / var_gpe);
        let assign5750_e4481: f64 = (var_ile).powf(p.p299);
        let assign5750_e4482: f64 = (assign5750_e4478 * assign5750_e4481);
        let assign5750_e4483: f64 = (p.p297 + assign5750_e4482);
        let assign5750_e4487: f64 = (p.p300 * var_iwe);
        let assign5750_e4488: f64 = (1.0 + assign5750_e4487);
        let assign5750_e4489: f64 = (assign5750_e4483 * assign5750_e4488);
        let assign5750_e4493: f64 = (p.p301 * var_iae);
        let assign5750_e4494: f64 = (1.0 + assign5750_e4493);
        let assign5750_e4495: f64 = (assign5750_e4489 * assign5750_e4494);
        (assign5750_e4495,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign5750_e4497;

        let (assign5760_e4513,) = {
    if (var_guard36 != 0.0) {
        let assign5760_e4502: f64 = (p.p303 * var_ile);
        let assign5760_e4503: f64 = (p.p302 + assign5760_e4502);
        let assign5760_e4506: f64 = (p.p304 * var_iwe);
        let assign5760_e4507: f64 = (assign5760_e4503 + assign5760_e4506);
        let assign5760_e4510: f64 = (p.p305 * var_iae);
        let assign5760_e4511: f64 = (assign5760_e4507 + assign5760_e4510);
        (assign5760_e4511,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign5760_e4513;

        let (assign5770_e4517,) = {
    if (var_guard36 != 0.0) {
        (p.p306,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign5770_e4517;

        let (assign5780_e4521,) = {
    if (var_guard36 != 0.0) {
        (p.p307,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign5780_e4521;

        let (assign5790_e4525,) = {
    if (var_guard36 != 0.0) {
        (p.p308,)
    } else {
        (var_thesatt_p,)
    }
};
        var_thesatt_p = assign5790_e4525;

        let (assign5800_e4535,) = {
    if (var_guard36 != 0.0) {
        let assign5800_e4531: f64 = (p.p310 * var_ile);
        let assign5800_e4532: f64 = (1.0 + assign5800_e4531);
        let assign5800_e4533: f64 = (p.p309 / assign5800_e4532);
        (assign5800_e4533,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign5800_e4535;

        let (assign5810_e4549,) = {
    if (var_guard36 != 0.0) {
        let assign5810_e4540: f64 = (var_ile).powf(p.p312);
        let assign5810_e4541: f64 = (p.p311 * assign5810_e4540);
        let assign5810_e4545: f64 = (p.p313 * var_iwe);
        let assign5810_e4546: f64 = (1.0 + assign5810_e4545);
        let assign5810_e4547: f64 = (assign5810_e4541 * assign5810_e4546);
        (assign5810_e4547,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign5810_e4549;

        let (assign5820_e4555,) = {
    if (var_guard36 != 0.0) {
        let assign5820_e4553: f64 = (var_ile).powf(p.p315);
        (assign5820_e4553,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign5820_e4555;

        let (assign5830_e4575,) = {
    if (var_guard36 != 0.0) {
        let assign5830_e4559: f64 = (p.p314 * var_tmpx);
        let assign5830_e4563: f64 = (p.p317 * var_iwe);
        let assign5830_e4564: f64 = (1.0 + assign5830_e4563);
        let assign5830_e4565: f64 = (assign5830_e4559 * assign5830_e4564);
        let assign5830_e4569: f64 = (p.p316 * var_ile);
        let assign5830_e4571: f64 = (assign5830_e4569 * var_tmpx);
        let assign5830_e4572: f64 = (1.0 + assign5830_e4571);
        let assign5830_e4573: f64 = (assign5830_e4565 / assign5830_e4572);
        (assign5830_e4573,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign5830_e4575;

        let (assign5840_e4581,) = {
    if (var_guard36 != 0.0) {
        let assign5840_e4579: f64 = (var_ile).powf(p.p319);
        (assign5840_e4579,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign5840_e4581;

        let (assign5850_e4601,) = {
    if (var_guard36 != 0.0) {
        let assign5850_e4585: f64 = (p.p318 * var_tmpx);
        let assign5850_e4589: f64 = (p.p321 * var_iwe);
        let assign5850_e4590: f64 = (1.0 + assign5850_e4589);
        let assign5850_e4591: f64 = (assign5850_e4585 * assign5850_e4590);
        let assign5850_e4595: f64 = (p.p320 * var_ile);
        let assign5850_e4597: f64 = (assign5850_e4595 * var_tmpx);
        let assign5850_e4598: f64 = (1.0 + assign5850_e4597);
        let assign5850_e4599: f64 = (assign5850_e4591 / assign5850_e4598);
        (assign5850_e4599,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign5850_e4601;

        let (assign5860_e4605,) = {
    if (var_guard36 != 0.0) {
        (p.p322,)
    } else {
        (var_vp_p,)
    }
};
        var_vp_p = assign5860_e4605;

        let (assign5870_e4621,) = {
    if (var_guard36 != 0.0) {
        let assign5870_e4611: f64 = (p.p324 * var_ile);
        let assign5870_e4612: f64 = (1.0 + assign5870_e4611);
        let assign5870_e4613: f64 = (p.p323 * assign5870_e4612);
        let assign5870_e4617: f64 = (p.p325 * var_iwe);
        let assign5870_e4618: f64 = (1.0 + assign5870_e4617);
        let assign5870_e4619: f64 = (assign5870_e4613 * assign5870_e4618);
        (assign5870_e4619,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign5870_e4621;

        let (assign5880_e4625,) = {
    if (var_guard36 != 0.0) {
        (p.p326,)
    } else {
        (var_a2_p,)
    }
};
        var_a2_p = assign5880_e4625;

        let (assign5890_e4629,) = {
    if (var_guard36 != 0.0) {
        (p.p327,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign5890_e4629;

        let (assign5900_e4645,) = {
    if (var_guard36 != 0.0) {
        let assign5900_e4635: f64 = (p.p329 * var_ile);
        let assign5900_e4636: f64 = (1.0 + assign5900_e4635);
        let assign5900_e4637: f64 = (p.p328 * assign5900_e4636);
        let assign5900_e4641: f64 = (p.p330 * var_iwe);
        let assign5900_e4642: f64 = (1.0 + assign5900_e4641);
        let assign5900_e4643: f64 = (assign5900_e4637 * assign5900_e4642);
        (assign5900_e4643,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign5900_e4645;

        let (assign5910_e4661,) = {
    if (var_guard36 != 0.0) {
        let assign5910_e4651: f64 = (p.p332 * var_ile);
        let assign5910_e4652: f64 = (1.0 + assign5910_e4651);
        let assign5910_e4653: f64 = (p.p331 * assign5910_e4652);
        let assign5910_e4657: f64 = (p.p333 * var_iwe);
        let assign5910_e4658: f64 = (1.0 + assign5910_e4657);
        let assign5910_e4659: f64 = (assign5910_e4653 * assign5910_e4658);
        (assign5910_e4659,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign5910_e4661;

        let (assign5920_e4665,) = {
    if (var_guard36 != 0.0) {
        (p.p334,)
    } else {
        (var_imaxii_p,)
    }
};
        var_imaxii_p = assign5920_e4665;

        let (assign5930_e4669,) = {
    if (var_guard36 != 0.0) {
        (p.p335,)
    } else {
        (var_gco_p,)
    }
};
        var_gco_p = assign5930_e4669;

        let (assign5940_e4675,) = {
    if (var_guard36 != 0.0) {
        let assign5940_e4673: f64 = (p.p336 / var_iae);
        (assign5940_e4673,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign5940_e4675;

        let (assign5950_e4685,) = {
    if (var_guard36 != 0.0) {
        let assign5950_e4679: f64 = (p.p337 * p.p237);
        let assign5950_e4682: f64 = (1e-6 * var_iwe);
        let assign5950_e4683: f64 = (assign5950_e4679 / assign5950_e4682);
        (assign5950_e4683,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign5950_e4685;

        let (assign5960_e4695,) = {
    if (var_guard36 != 0.0) {
        let assign5960_e4689: f64 = (p.p338 * p.p238);
        let assign5960_e4692: f64 = (1e-6 * var_iwe);
        let assign5960_e4693: f64 = (assign5960_e4689 / assign5960_e4692);
        (assign5960_e4693,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign5960_e4695;

        let (assign5970_e4699,) = {
    if (var_guard36 != 0.0) {
        (p.p339,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign5970_e4699;

        let (assign5980_e4703,) = {
    if (var_guard36 != 0.0) {
        (p.p340,)
    } else {
        (var_gc2_p,)
    }
};
        var_gc2_p = assign5980_e4703;

        let (assign5990_e4707,) = {
    if (var_guard36 != 0.0) {
        (p.p341,)
    } else {
        (var_gc3_p,)
    }
};
        var_gc3_p = assign5990_e4707;

        let (assign6000_e4711,) = {
    if (var_guard36 != 0.0) {
        (p.p340,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6000_e4711;

        let assign6010_e4713: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6010_e4715: f64 = if assign6010_e4713 == 1.0 { 1.0 } else { 0.0 };
        var_guard39 = assign6010_e4715;

        let (assign6020_e4721,) = {
    if ((var_guard36 != 0.0) && (var_guard39 != 0.0)) {
        (p.p342,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6020_e4721;

        let (assign6030_e4725,) = {
    if (var_guard36 != 0.0) {
        (p.p341,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6030_e4725;

        let assign6040_e4727: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6040_e4729: f64 = if assign6040_e4727 == 1.0 { 1.0 } else { 0.0 };
        var_guard40 = assign6040_e4729;

        let (assign6050_e4735,) = {
    if ((var_guard36 != 0.0) && (var_guard40 != 0.0)) {
        (p.p343,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6050_e4735;

        let (assign6060_e4739,) = {
    if (var_guard36 != 0.0) {
        (var_gc2ov_p,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6060_e4739;

        let assign6070_e4741: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6070_e4743: f64 = if assign6070_e4741 == 1.0 { 1.0 } else { 0.0 };
        var_guard41 = assign6070_e4743;

        let (assign6080_e4749,) = {
    if ((var_guard36 != 0.0) && (var_guard41 != 0.0)) {
        (p.p344,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6080_e4749;

        let (assign6090_e4753,) = {
    if (var_guard36 != 0.0) {
        (var_gc3ov_p,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6090_e4753;

        let assign6100_e4755: f64 = if param_given[345] { 1.0 } else { 0.0 };
        let assign6100_e4757: f64 = if assign6100_e4755 == 1.0 { 1.0 } else { 0.0 };
        var_guard42 = assign6100_e4757;

        let (assign6110_e4763,) = {
    if ((var_guard36 != 0.0) && (var_guard42 != 0.0)) {
        (p.p345,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6110_e4763;

        let (assign6120_e4767,) = {
    if (var_guard36 != 0.0) {
        (p.p346,)
    } else {
        (var_chib_p,)
    }
};
        var_chib_p = assign6120_e4767;

        *var_a1_p_slot = var_a1_p;
        *var_a2_p_slot = var_a2_p;
        *var_a3_p_slot = var_a3_p;
        *var_a4_p_slot = var_a4_p;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp_p_slot = var_alp_p;
        *var_ax_p_slot = var_ax_p;
        *var_chib_p_slot = var_chib_p;
        *var_cs_p_slot = var_cs_p;
        *var_feta_p_slot = var_feta_p;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gco_p_slot = var_gco_p;
        *var_guard39_slot = var_guard39;
        *var_guard40_slot = var_guard40;
        *var_guard41_slot = var_guard41;
        *var_guard42_slot = var_guard42;
        *var_iginv_p_slot = var_iginv_p;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_rs_p_slot = var_rs_p;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsg_p_slot = var_rsg_p;
        *var_sta2_p_slot = var_sta2_p;
        *var_stcs_p_slot = var_stcs_p;
        *var_stig_p_slot = var_stig_p;
        *var_strs_p_slot = var_strs_p;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_thecs_p_slot = var_thecs_p;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_tmpx_slot = var_tmpx;
        *var_vp_p_slot = var_vp_p;
        *var_xcor_p_slot = var_xcor_p;
    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_betn_p: f64,
        var_gpe: f64,
        var_guard36: f64,
        var_gwe: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_wecv: f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_alp1ac_p_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axacl_i_slot: &mut f64,
        var_axaco_i_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_we_edge_slot: &mut f64,
    ) {
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axacl_i: f64 = *var_axacl_i_slot;
        let mut var_axaco_i: f64 = *var_axaco_i_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;

        let (assign6130_e4777,) = {
    if (var_guard36 != 0.0) {
        let assign6130_e4771: f64 = (p.p347 * p.p237);
        let assign6130_e4774: f64 = (1e-6 * var_iwe);
        let assign6130_e4775: f64 = (assign6130_e4771 / assign6130_e4774);
        (assign6130_e4775,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign6130_e4777;

        let (assign6140_e4787,) = {
    if (var_guard36 != 0.0) {
        let assign6140_e4781: f64 = (p.p348 * p.p238);
        let assign6140_e4784: f64 = (1e-6 * var_iwe);
        let assign6140_e4785: f64 = (assign6140_e4781 / assign6140_e4784);
        (assign6140_e4785,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign6140_e4787;

        let (assign6150_e4791,) = {
    if (var_guard36 != 0.0) {
        (p.p349,)
    } else {
        (var_bgidl_p,)
    }
};
        var_bgidl_p = assign6150_e4791;

        let (assign6160_e4795,) = {
    if (var_guard36 != 0.0) {
        (p.p350,)
    } else {
        (var_bgidld_p,)
    }
};
        var_bgidld_p = assign6160_e4795;

        let (assign6170_e4799,) = {
    if (var_guard36 != 0.0) {
        (p.p351,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign6170_e4799;

        let (assign6180_e4803,) = {
    if (var_guard36 != 0.0) {
        (p.p352,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign6180_e4803;

        let (assign6190_e4807,) = {
    if (var_guard36 != 0.0) {
        (p.p353,)
    } else {
        (var_cgidl_p,)
    }
};
        var_cgidl_p = assign6190_e4807;

        let (assign6200_e4811,) = {
    if (var_guard36 != 0.0) {
        (p.p354,)
    } else {
        (var_cgidld_p,)
    }
};
        var_cgidld_p = assign6200_e4811;

        if (s.v[1016] != 0.0) {
            s.store_scalar(118, ((((8.8541878176e-12 * p.p210) * var_wecv) * s.v[320]) / p.p209));
        }

        let (assign6220_e4835,) = {
    if (var_guard36 != 0.0) {
        let assign6220_e4827: f64 = (8.8541878176e-12 * p.p210);
        let assign6220_e4829: f64 = (assign6220_e4827 * var_wecv);
        let assign6220_e4831: f64 = (assign6220_e4829 * p.p237);
        let assign6220_e4833: f64 = (assign6220_e4831 / p.p235);
        (assign6220_e4833,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign6220_e4835;

        let (assign6230_e4847,) = {
    if (var_guard36 != 0.0) {
        let assign6230_e4839: f64 = (8.8541878176e-12 * p.p210);
        let assign6230_e4841: f64 = (assign6230_e4839 * var_wecv);
        let assign6230_e4843: f64 = (assign6230_e4841 * p.p238);
        let assign6230_e4845: f64 = (assign6230_e4843 / p.p236);
        (assign6230_e4845,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign6230_e4847;

        let (assign6240_e4865,) = {
    if (var_guard36 != 0.0) {
        let assign6240_e4853: f64 = (var_ile).powf(p.p357);
        let assign6240_e4854: f64 = (p.p356 * assign6240_e4853);
        let assign6240_e4855: f64 = (p.p355 + assign6240_e4854);
        let assign6240_e4858: f64 = (p.p358 * var_iwe);
        let assign6240_e4859: f64 = (assign6240_e4855 + assign6240_e4858);
        let assign6240_e4862: f64 = (p.p359 * var_iae);
        let assign6240_e4863: f64 = (assign6240_e4859 + assign6240_e4862);
        (assign6240_e4863,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign6240_e4865;

        let (assign6250_e4881,) = {
    if (var_guard36 != 0.0) {
        let assign6250_e4870: f64 = (p.p361 * var_ile);
        let assign6250_e4871: f64 = (p.p360 + assign6250_e4870);
        let assign6250_e4874: f64 = (p.p362 * var_iwe);
        let assign6250_e4875: f64 = (assign6250_e4871 + assign6250_e4874);
        let assign6250_e4878: f64 = (p.p363 * var_iae);
        let assign6250_e4879: f64 = (assign6250_e4875 + assign6250_e4878);
        (assign6250_e4879,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign6250_e4881;

        let (assign6260_e4885,) = {
    if (var_guard36 != 0.0) {
        (p.p297,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6260_e4885;

        let assign6270_e4887: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6270_e4889: f64 = if assign6270_e4887 == 1.0 { 1.0 } else { 0.0 };
        var_guard43 = assign6270_e4889;

        let (assign6280_e4895,) = {
    if ((var_guard36 != 0.0) && (var_guard43 != 0.0)) {
        (p.p364,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6280_e4895;

        let (assign6290_e4899,) = {
    if (var_guard36 != 0.0) {
        (p.p298,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6290_e4899;

        let assign6300_e4901: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6300_e4903: f64 = if assign6300_e4901 == 1.0 { 1.0 } else { 0.0 };
        var_guard44 = assign6300_e4903;

        let (assign6310_e4909,) = {
    if ((var_guard36 != 0.0) && (var_guard44 != 0.0)) {
        (p.p365,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6310_e4909;

        let (assign6320_e4913,) = {
    if (var_guard36 != 0.0) {
        (p.p299,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6320_e4913;

        let assign6330_e4915: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6330_e4917: f64 = if assign6330_e4915 == 1.0 { 1.0 } else { 0.0 };
        var_guard45 = assign6330_e4917;

        let (assign6340_e4923,) = {
    if ((var_guard36 != 0.0) && (var_guard45 != 0.0)) {
        (p.p366,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6340_e4923;

        let (assign6350_e4927,) = {
    if (var_guard36 != 0.0) {
        (p.p300,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6350_e4927;

        let assign6360_e4929: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6360_e4931: f64 = if assign6360_e4929 == 1.0 { 1.0 } else { 0.0 };
        var_guard46 = assign6360_e4931;

        let (assign6370_e4937,) = {
    if ((var_guard36 != 0.0) && (var_guard46 != 0.0)) {
        (p.p367,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6370_e4937;

        let (assign6380_e4941,) = {
    if (var_guard36 != 0.0) {
        (p.p301,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6380_e4941;

        let assign6390_e4943: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6390_e4945: f64 = if assign6390_e4943 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign6390_e4945;

        let (assign6400_e4951,) = {
    if ((var_guard36 != 0.0) && (var_guard47 != 0.0)) {
        (p.p368,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6400_e4951;

        let (assign6410_e4977,) = {
    if (var_guard36 != 0.0) {
        let assign6410_e4956: f64 = (var_thesatacl_i * var_gwe);
        let assign6410_e4958: f64 = (assign6410_e4956 / var_gpe);
        let assign6410_e4961: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign6410_e4962: f64 = (assign6410_e4958 * assign6410_e4961);
        let assign6410_e4963: f64 = (var_thesataco_i + assign6410_e4962);
        let assign6410_e4967: f64 = (var_thesatacw_i * var_iwe);
        let assign6410_e4968: f64 = (1.0 + assign6410_e4967);
        let assign6410_e4969: f64 = (assign6410_e4963 * assign6410_e4968);
        let assign6410_e4973: f64 = (var_thesataclw_i * var_iae);
        let assign6410_e4974: f64 = (1.0 + assign6410_e4973);
        let assign6410_e4975: f64 = (assign6410_e4969 * assign6410_e4974);
        (assign6410_e4975,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign6410_e4977;

        let (assign6420_e4981,) = {
    if (var_guard36 != 0.0) {
        (p.p309,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6420_e4981;

        let assign6430_e4983: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6430_e4985: f64 = if assign6430_e4983 == 1.0 { 1.0 } else { 0.0 };
        var_guard48 = assign6430_e4985;

        let (assign6440_e4991,) = {
    if ((var_guard36 != 0.0) && (var_guard48 != 0.0)) {
        (p.p369,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6440_e4991;

        let (assign6450_e4995,) = {
    if (var_guard36 != 0.0) {
        (p.p310,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6450_e4995;

        let assign6460_e4997: f64 = if param_given[370] { 1.0 } else { 0.0 };
        let assign6460_e4999: f64 = if assign6460_e4997 == 1.0 { 1.0 } else { 0.0 };
        var_guard49 = assign6460_e4999;

        let (assign6470_e5005,) = {
    if ((var_guard36 != 0.0) && (var_guard49 != 0.0)) {
        (p.p370,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6470_e5005;

        let (assign6480_e5015,) = {
    if (var_guard36 != 0.0) {
        let assign6480_e5011: f64 = (var_axacl_i * var_ile);
        let assign6480_e5012: f64 = (1.0 + assign6480_e5011);
        let assign6480_e5013: f64 = (var_axaco_i / assign6480_e5012);
        (assign6480_e5013,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign6480_e5015;

        let (assign6490_e5029,) = {
    if (var_guard36 != 0.0) {
        let assign6490_e5020: f64 = (var_ile).powf(p.p372);
        let assign6490_e5021: f64 = (p.p371 * assign6490_e5020);
        let assign6490_e5025: f64 = (p.p373 * var_iwe);
        let assign6490_e5026: f64 = (1.0 + assign6490_e5025);
        let assign6490_e5027: f64 = (assign6490_e5021 * assign6490_e5026);
        (assign6490_e5027,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign6490_e5029;

        let (assign6500_e5035,) = {
    if (var_guard36 != 0.0) {
        let assign6500_e5033: f64 = (var_ile).powf(p.p375);
        (assign6500_e5033,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6500_e5035;

        let (assign6510_e5055,) = {
    if (var_guard36 != 0.0) {
        let assign6510_e5039: f64 = (p.p374 * var_tmpx);
        let assign6510_e5043: f64 = (p.p377 * var_iwe);
        let assign6510_e5044: f64 = (1.0 + assign6510_e5043);
        let assign6510_e5045: f64 = (assign6510_e5039 * assign6510_e5044);
        let assign6510_e5049: f64 = (p.p376 * var_ile);
        let assign6510_e5051: f64 = (assign6510_e5049 * var_tmpx);
        let assign6510_e5052: f64 = (1.0 + assign6510_e5051);
        let assign6510_e5053: f64 = (assign6510_e5045 / assign6510_e5052);
        (assign6510_e5053,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign6510_e5055;

        let (assign6520_e5059,) = {
    if (var_guard36 != 0.0) {
        (p.p378,)
    } else {
        (var_fcgovacc_p,)
    }
};
        var_fcgovacc_p = assign6520_e5059;

        let (assign6530_e5063,) = {
    if (var_guard36 != 0.0) {
        (p.p379,)
    } else {
        (var_fcgovaccd_p,)
    }
};
        var_fcgovaccd_p = assign6530_e5063;

        let (assign6540_e5067,) = {
    if (var_guard36 != 0.0) {
        (p.p380,)
    } else {
        (var_cgovaccg_p,)
    }
};
        var_cgovaccg_p = assign6540_e5067;

        let (assign6550_e5073,) = {
    if (var_guard36 != 0.0) {
        let assign6550_e5071: f64 = (p.p381 * var_iilcv);
        (assign6550_e5071,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign6550_e5073;

        let (assign6560_e5079,) = {
    if (var_guard36 != 0.0) {
        let assign6560_e5077: f64 = (p.p382 * var_iiwecv);
        (assign6560_e5077,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign6560_e5079;

        let (assign6570_e5085,) = {
    if (var_guard36 != 0.0) {
        let assign6570_e5083: f64 = (p.p383 * var_iiwecv);
        (assign6570_e5083,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign6570_e5085;

        let (assign6580_e5089,) = {
    if (var_guard36 != 0.0) {
        (p.p384,)
    } else {
        (var_dvfbinr_p,)
    }
};
        var_dvfbinr_p = assign6580_e5089;

        let (assign6590_e5093,) = {
    if (var_guard36 != 0.0) {
        (p.p385,)
    } else {
        (var_fcinrdep_p,)
    }
};
        var_fcinrdep_p = assign6590_e5093;

        let (assign6600_e5097,) = {
    if (var_guard36 != 0.0) {
        (p.p386,)
    } else {
        (var_fcinracc_p,)
    }
};
        var_fcinracc_p = assign6600_e5097;

        let (assign6610_e5101,) = {
    if (var_guard36 != 0.0) {
        (p.p387,)
    } else {
        (var_axinr_p,)
    }
};
        var_axinr_p = assign6610_e5101;

        let (assign6620_e5107,) = {
    if (var_guard36 != 0.0) {
        let assign6620_e5105: f64 = (p.p388 * var_iiwcv);
        (assign6620_e5105,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign6620_e5107;

        let (assign6630_e5113,) = {
    if (var_guard36 != 0.0) {
        let assign6630_e5111: f64 = (p.p389 * var_iiwcv);
        (assign6630_e5111,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign6630_e5113;

        let (assign6640_e5123,) = {
    if (var_guard36 != 0.0) {
        let assign6640_e5118: f64 = (2.0 * p.p396);
        let assign6640_e5120: f64 = (assign6640_e5118 / var_le);
        let assign6640_e5121: f64 = (1.0 - assign6640_e5120);
        (assign6640_e5121,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign6640_e5123;

        if (s.v[1016] != 0.0) {
            s.store_scalar(139, p.p390);
            s.store_scalar(140, ((((p.p391 * var_betn_p) * var_betn_p) * var_iwe) * var_iwe));
        }

        let (assign6730_e5194,) = {
    if (var_guard36 != 0.0) {
        let assign6730_e5188: f64 = (2.0 * p.p398);
        let assign6730_e5191: f64 = (p.p399 * var_we);
        let assign6730_e5192: f64 = (assign6730_e5188 + assign6730_e5191);
        (assign6730_e5192,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign6730_e5194;

        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alpac_p_slot = var_alpac_p;
        *var_axac_p_slot = var_axac_p;
        *var_axacl_i_slot = var_axacl_i;
        *var_axaco_i_slot = var_axaco_i;
        *var_axinr_p_slot = var_axinr_p;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_guard43_slot = var_guard43;
        *var_guard44_slot = var_guard44;
        *var_guard45_slot = var_guard45;
        *var_guard46_slot = var_guard46;
        *var_guard47_slot = var_guard47;
        *var_guard48_slot = var_guard48;
        *var_guard49_slot = var_guard49;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_temp0_slot = var_temp0;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesatacw_i_slot = var_thesatacw_i;
        *var_tmpx_slot = var_tmpx;
        *var_we_edge_slot = var_we_edge;
    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_l_f: f64,
        var_l_slif: f64,
        var_le: f64,
        var_nf_i: f64,
        var_ngcon_i: f64,
        var_w_f: f64,
        var_we_edge: f64,
        var_xgwe: f64,
        var_betnedge_p_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_deltarth_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gpe_edge_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard56_slot: &mut f64,
        var_kuowe_slot: &mut f64,
        var_kvthowe_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_rbulk_p_slot: &mut f64,
        var_rde_p_slot: &mut f64,
        var_rg_p_slot: &mut f64,
        var_rjund_p_slot: &mut f64,
        var_rjuns_p_slot: &mut f64,
        var_rse_p_slot: &mut f64,
        var_rsh_i_slot: &mut f64,
        var_rshd_i_slot: &mut f64,
        var_rth_p_slot: &mut f64,
        var_rwell_p_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_strth_p_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
    ) {
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_deltarth: f64 = *var_deltarth_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gpe_edge: f64 = *var_gpe_edge_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
        let mut var_kuowe: f64 = *var_kuowe_slot;
        let mut var_kvthowe: f64 = *var_kvthowe_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_rbulk_p: f64 = *var_rbulk_p_slot;
        let mut var_rde_p: f64 = *var_rde_p_slot;
        let mut var_rg_p: f64 = *var_rg_p_slot;
        let mut var_rjund_p: f64 = *var_rjund_p_slot;
        let mut var_rjuns_p: f64 = *var_rjuns_p_slot;
        let mut var_rse_p: f64 = *var_rse_p_slot;
        let mut var_rsh_i: f64 = *var_rsh_i_slot;
        let mut var_rshd_i: f64 = *var_rshd_i_slot;
        let mut var_rth_p: f64 = *var_rth_p_slot;
        let mut var_rwell_p: f64 = *var_rwell_p_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_strth_p: f64 = *var_strth_p_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;

        let (assign6760_e5210,) = {
    if (var_guard36 != 0.0) {
        (p.p400,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign6760_e5210;

        let (assign6770_e5226,) = {
    if (var_guard36 != 0.0) {
        let assign6770_e5215: f64 = (p.p402 * var_ile);
        let assign6770_e5216: f64 = (p.p401 + assign6770_e5215);
        let assign6770_e5219: f64 = (p.p403 * var_iwe);
        let assign6770_e5220: f64 = (assign6770_e5216 + assign6770_e5219);
        let assign6770_e5223: f64 = (p.p404 * var_iae);
        let assign6770_e5224: f64 = (assign6770_e5220 + assign6770_e5223);
        (assign6770_e5224,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign6770_e5226;

        let (assign6780_e5244,) = {
    if (var_guard36 != 0.0) {
        let assign6780_e5232: f64 = (var_ile).powf(p.p407);
        let assign6780_e5233: f64 = (p.p406 * assign6780_e5232);
        let assign6780_e5234: f64 = (p.p405 + assign6780_e5233);
        let assign6780_e5237: f64 = (p.p408 * var_iwe);
        let assign6780_e5238: f64 = (assign6780_e5234 + assign6780_e5237);
        let assign6780_e5241: f64 = (p.p409 * var_iae);
        let assign6780_e5242: f64 = (assign6780_e5238 + assign6780_e5241);
        (assign6780_e5242,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign6780_e5244;

        let (assign6790_e5268,) = {
    if (var_guard36 != 0.0) {
        let assign6790_e5251: f64 = (var_ile).powf(p.p412);
        let assign6790_e5252: f64 = (p.p411 * assign6790_e5251);
        let assign6790_e5253: f64 = (1.0 + assign6790_e5252);
        let assign6790_e5254: f64 = (p.p410 * assign6790_e5253);
        let assign6790_e5258: f64 = (p.p413 * var_iwe);
        let assign6790_e5259: f64 = (1.0 + assign6790_e5258);
        let assign6790_e5260: f64 = (assign6790_e5254 * assign6790_e5259);
        let assign6790_e5264: f64 = (p.p414 * var_iae);
        let assign6790_e5265: f64 = (1.0 + assign6790_e5264);
        let assign6790_e5266: f64 = (assign6790_e5260 * assign6790_e5265);
        (assign6790_e5266,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign6790_e5268;

        let (assign6800_e5278,) = {
    if (var_guard36 != 0.0) {
        let assign6800_e5274: f64 = (var_ile).powf(p.p417);
        let assign6800_e5275: f64 = (p.p416 * assign6800_e5274);
        let assign6800_e5276: f64 = (p.p415 + assign6800_e5275);
        (assign6800_e5276,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign6800_e5278;

        let (assign6810_e5296,) = {
    if (var_guard36 != 0.0) {
        let assign6810_e5283: f64 = (p.p418 * p.p419);
        let assign6810_e5285: f64 = (assign6810_e5283 / var_le);
        let assign6810_e5288: f64 = (-var_le);
        let assign6810_e5290: f64 = (assign6810_e5288 / p.p419);
        let assign6810_e5291: f64 = (assign6810_e5290).exp();
        let assign6810_e5292: f64 = (1.0 - assign6810_e5291);
        let assign6810_e5293: f64 = (assign6810_e5285 * assign6810_e5292);
        let assign6810_e5294: f64 = (1.0 + assign6810_e5293);
        (assign6810_e5294,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign6810_e5296;

        let (assign6820_e5305,) = {
    if (var_guard36 != 0.0) {
        let (assign6820_e5303,) = {
            if (var_gpe_edge > 1e-15) {
                (var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign6820_e5303,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign6820_e5305;

        let (assign6830_e5321,) = {
    if (var_guard36 != 0.0) {
        let assign6830_e5309: f64 = (p.p259 * var_we_edge);
        let assign6830_e5312: f64 = (var_gpe_edge * var_le);
        let assign6830_e5313: f64 = (assign6830_e5309 / assign6830_e5312);
        let assign6830_e5317: f64 = (p.p420 * var_iwe);
        let assign6830_e5318: f64 = (1.0 + assign6830_e5317);
        let assign6830_e5319: f64 = (assign6830_e5313 * assign6830_e5318);
        (assign6830_e5319,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign6830_e5321;

        let (assign6840_e5337,) = {
    if (var_guard36 != 0.0) {
        let assign6840_e5326: f64 = (p.p422 * var_ile);
        let assign6840_e5327: f64 = (p.p421 + assign6840_e5326);
        let assign6840_e5330: f64 = (p.p423 * var_iwe);
        let assign6840_e5331: f64 = (assign6840_e5327 + assign6840_e5330);
        let assign6840_e5334: f64 = (p.p424 * var_iae);
        let assign6840_e5335: f64 = (assign6840_e5331 + assign6840_e5334);
        (assign6840_e5335,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign6840_e5337;

        let (assign6850_e5351,) = {
    if (var_guard36 != 0.0) {
        let assign6850_e5342: f64 = (var_ile).powf(p.p426);
        let assign6850_e5343: f64 = (p.p425 * assign6850_e5342);
        let assign6850_e5347: f64 = (p.p427 * var_iwe);
        let assign6850_e5348: f64 = (1.0 + assign6850_e5347);
        let assign6850_e5349: f64 = (assign6850_e5343 * assign6850_e5348);
        (assign6850_e5349,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign6850_e5351;

        let (assign6860_e5355,) = {
    if (var_guard36 != 0.0) {
        (p.p428,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign6860_e5355;

        let (assign6870_e5359,) = {
    if (var_guard36 != 0.0) {
        (p.p429,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign6870_e5359;

        let (assign6880_e5373,) = {
    if (var_guard36 != 0.0) {
        let assign6880_e5364: f64 = (var_ile).powf(p.p431);
        let assign6880_e5365: f64 = (p.p430 * assign6880_e5364);
        let assign6880_e5369: f64 = (p.p432 * var_iwe);
        let assign6880_e5370: f64 = (1.0 + assign6880_e5369);
        let assign6880_e5371: f64 = (assign6880_e5365 * assign6880_e5370);
        (assign6880_e5371,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign6880_e5373;

        let (assign6890_e5377,) = {
    if (var_guard36 != 0.0) {
        (p.p434,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign6890_e5377;

        let (assign6900_e5381,) = {
    if (var_guard36 != 0.0) {
        (p.p433,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign6900_e5381;

        let (assign6960_e5423,) = {
    if (var_guard36 != 0.0) {
        let assign6960_e5412: f64 = (p.p832 * var_ile);
        let assign6960_e5413: f64 = (p.p831 + assign6960_e5412);
        let assign6960_e5416: f64 = (p.p833 * var_iwe);
        let assign6960_e5417: f64 = (assign6960_e5413 + assign6960_e5416);
        let assign6960_e5420: f64 = (p.p834 * var_iae);
        let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
        (assign6960_e5421,)
    } else {
        (var_kvthowe,)
    }
};
        var_kvthowe = assign6960_e5423;

        let (assign6970_e5439,) = {
    if (var_guard36 != 0.0) {
        let assign6970_e5428: f64 = (p.p836 * var_ile);
        let assign6970_e5429: f64 = (p.p835 + assign6970_e5428);
        let assign6970_e5432: f64 = (p.p837 * var_iwe);
        let assign6970_e5433: f64 = (assign6970_e5429 + assign6970_e5432);
        let assign6970_e5436: f64 = (p.p838 * var_iae);
        let assign6970_e5437: f64 = (assign6970_e5433 + assign6970_e5436);
        (assign6970_e5437,)
    } else {
        (var_kuowe,)
    }
};
        var_kuowe = assign6970_e5439;

        let (assign6980_e5467,) = {
    if (var_guard36 != 0.0) {
        let assign6980_e5444: f64 = (0.3333333333333333 * var_w_f);
        let assign6980_e5446: f64 = (assign6980_e5444 / var_ngcon_i);
        let assign6980_e5448: f64 = (assign6980_e5446 + var_xgwe);
        let assign6980_e5449: f64 = (p.p443 * assign6980_e5448);
        let assign6980_e5452: f64 = (var_ngcon_i * var_l_slif);
        let assign6980_e5453: f64 = (assign6980_e5449 / assign6980_e5452);
        let assign6980_e5456: f64 = (p.p441 + p.p442);
        let assign6980_e5459: f64 = (var_w_f * var_l_f);
        let assign6980_e5460: f64 = (assign6980_e5456 / assign6980_e5459);
        let assign6980_e5461: f64 = (assign6980_e5453 + assign6980_e5460);
        let assign6980_e5464: f64 = (var_nf_i * p.p440);
        let assign6980_e5465: f64 = (assign6980_e5461 + assign6980_e5464);
        (assign6980_e5465,)
    } else {
        (var_rg_p,)
    }
};
        var_rg_p = assign6980_e5467;

        let (assign6990_e5476,) = {
    if (var_guard36 != 0.0) {
        let (assign6990_e5474,) = {
            if (p.p445 > 0.0) {
                (p.p445,)
            } else {
                (0.0,)
            }
        };
        (assign6990_e5474,)
    } else {
        (var_rsh_i,)
    }
};
        var_rsh_i = assign6990_e5476;

        let (assign7000_e5485,) = {
    if (var_guard36 != 0.0) {
        let (assign7000_e5483,) = {
            if (p.p446 > 0.0) {
                (p.p446,)
            } else {
                (0.0,)
            }
        };
        (assign7000_e5483,)
    } else {
        (var_rshd_i,)
    }
};
        var_rshd_i = assign7000_e5485;

        let assign7010_e5488: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard50 = assign7010_e5488;

        let (assign7020_e5494,) = {
    if ((var_guard36 != 0.0) && (var_guard50 != 0.0)) {
        (var_rsh_i,)
    } else {
        (var_rshd_i,)
    }
};
        var_rshd_i = assign7020_e5494;

        let (assign7030_e5502,) = {
    if (var_guard36 != 0.0) {
        let assign7030_e5498: f64 = (var_nf_i * p.p12);
        let assign7030_e5500: f64 = (assign7030_e5498 * var_rsh_i);
        (assign7030_e5500,)
    } else {
        (var_rse_p,)
    }
};
        var_rse_p = assign7030_e5502;

        let (assign7040_e5510,) = {
    if (var_guard36 != 0.0) {
        let assign7040_e5506: f64 = (var_nf_i * p.p13);
        let assign7040_e5508: f64 = (assign7040_e5506 * var_rshd_i);
        (assign7040_e5508,)
    } else {
        (var_rde_p,)
    }
};
        var_rde_p = assign7040_e5510;

        let (assign7050_e5516,) = {
    if (var_guard36 != 0.0) {
        let assign7050_e5514: f64 = (var_nf_i * p.p448);
        (assign7050_e5514,)
    } else {
        (var_rwell_p,)
    }
};
        var_rwell_p = assign7050_e5516;

        let (assign7060_e5522,) = {
    if (var_guard36 != 0.0) {
        let assign7060_e5520: f64 = (var_nf_i * p.p447);
        (assign7060_e5520,)
    } else {
        (var_rbulk_p,)
    }
};
        var_rbulk_p = assign7060_e5522;

        let (assign7070_e5528,) = {
    if (var_guard36 != 0.0) {
        let assign7070_e5526: f64 = (var_nf_i * p.p449);
        (assign7070_e5526,)
    } else {
        (var_rjuns_p,)
    }
};
        var_rjuns_p = assign7070_e5528;

        let (assign7080_e5534,) = {
    if (var_guard36 != 0.0) {
        let assign7080_e5532: f64 = (var_nf_i * p.p450);
        (assign7080_e5532,)
    } else {
        (var_rjund_p,)
    }
};
        var_rjund_p = assign7080_e5534;

        let (assign7090_e5546,) = {
    if (var_guard36 != 0.0) {
        let assign7090_e5540: f64 = (p.p454 / var_ile);
        let assign7090_e5541: f64 = (1.0 + assign7090_e5540);
        let assign7090_e5543: f64 = (assign7090_e5541 / var_iwe);
        let assign7090_e5544: f64 = (p.p453 + assign7090_e5543);
        (assign7090_e5544,)
    } else {
        (var_deltarth,)
    }
};
        var_deltarth = assign7090_e5546;

        let (assign7100_e5555,) = {
    if (var_guard36 != 0.0) {
        let (assign7100_e5553,) = {
            if (var_deltarth > 1e-6) {
                (var_deltarth,)
            } else {
                (1e-6,)
            }
        };
        (assign7100_e5553,)
    } else {
        (var_deltarth,)
    }
};
        var_deltarth = assign7100_e5555;

        let (assign7110_e5563,) = {
    if (var_guard36 != 0.0) {
        let assign7110_e5560: f64 = (p.p452 / var_deltarth);
        let assign7110_e5561: f64 = (p.p451 + assign7110_e5560);
        (assign7110_e5561,)
    } else {
        (var_rth_p,)
    }
};
        var_rth_p = assign7110_e5563;

        let (assign7120_e5579,) = {
    if (var_guard36 != 0.0) {
        let assign7120_e5571: f64 = (p.p458 / var_ile);
        let assign7120_e5572: f64 = (1.0 + assign7120_e5571);
        let assign7120_e5573: f64 = (p.p457 + assign7120_e5572);
        let assign7120_e5574: f64 = (p.p456 * assign7120_e5573);
        let assign7120_e5576: f64 = (assign7120_e5574 / var_iwe);
        let assign7120_e5577: f64 = (p.p455 + assign7120_e5576);
        (assign7120_e5577,)
    } else {
        (var_cth_p,)
    }
};
        var_cth_p = assign7120_e5579;

        let (assign7130_e5583,) = {
    if (var_guard36 != 0.0) {
        (p.p459,)
    } else {
        (var_strth_p,)
    }
};
        var_strth_p = assign7130_e5583;

        let assign7140_e5602: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        var_guard51 = assign7140_e5602;

        let (assign7150_e5620,) = {
    if ((var_guard36 != 0.0) && (var_guard51 != 0.0)) {
        let assign7150_e5609: f64 = (p.p461 * var_ile);
        let assign7150_e5610: f64 = (p.p460 + assign7150_e5609);
        let assign7150_e5613: f64 = (p.p462 * var_iwe);
        let assign7150_e5614: f64 = (assign7150_e5610 + assign7150_e5613);
        let assign7150_e5617: f64 = (p.p463 * var_iae);
        let assign7150_e5618: f64 = (assign7150_e5614 + assign7150_e5617);
        (assign7150_e5618,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign7150_e5620;

        let assign7160_e5639: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        var_guard52 = assign7160_e5639;

        let (assign7170_e5657,) = {
    if ((var_guard36 != 0.0) && (var_guard52 != 0.0)) {
        let assign7170_e5646: f64 = (p.p465 * var_ile);
        let assign7170_e5647: f64 = (p.p464 + assign7170_e5646);
        let assign7170_e5650: f64 = (p.p466 * var_iwe);
        let assign7170_e5651: f64 = (assign7170_e5647 + assign7170_e5650);
        let assign7170_e5654: f64 = (p.p467 * var_iae);
        let assign7170_e5655: f64 = (assign7170_e5651 + assign7170_e5654);
        (assign7170_e5655,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign7170_e5657;

        let assign7180_e5676: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        var_guard53 = assign7180_e5676;

        let (assign7190_e5694,) = {
    if ((var_guard36 != 0.0) && (var_guard53 != 0.0)) {
        let assign7190_e5683: f64 = (p.p469 * var_ile);
        let assign7190_e5684: f64 = (p.p468 + assign7190_e5683);
        let assign7190_e5687: f64 = (p.p470 * var_iwe);
        let assign7190_e5688: f64 = (assign7190_e5684 + assign7190_e5687);
        let assign7190_e5691: f64 = (p.p471 * var_iae);
        let assign7190_e5692: f64 = (assign7190_e5688 + assign7190_e5691);
        (assign7190_e5692,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign7190_e5694;

        let assign7200_e5713: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        var_guard54 = assign7200_e5713;

        let (assign7210_e5731,) = {
    if ((var_guard36 != 0.0) && (var_guard54 != 0.0)) {
        let assign7210_e5720: f64 = (p.p473 * var_ile);
        let assign7210_e5721: f64 = (p.p472 + assign7210_e5720);
        let assign7210_e5724: f64 = (p.p474 * var_iwe);
        let assign7210_e5725: f64 = (assign7210_e5721 + assign7210_e5724);
        let assign7210_e5728: f64 = (p.p475 * var_iae);
        let assign7210_e5729: f64 = (assign7210_e5725 + assign7210_e5728);
        (assign7210_e5729,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign7210_e5731;

        let assign7220_e5750: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        var_guard55 = assign7220_e5750;

        let (assign7230_e5768,) = {
    if ((var_guard36 != 0.0) && (var_guard55 != 0.0)) {
        let assign7230_e5757: f64 = (p.p477 * var_ile);
        let assign7230_e5758: f64 = (p.p476 + assign7230_e5757);
        let assign7230_e5761: f64 = (p.p478 * var_iwe);
        let assign7230_e5762: f64 = (assign7230_e5758 + assign7230_e5761);
        let assign7230_e5765: f64 = (p.p479 * var_iae);
        let assign7230_e5766: f64 = (assign7230_e5762 + assign7230_e5765);
        (assign7230_e5766,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign7230_e5768;

        let assign7240_e5787: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        var_guard56 = assign7240_e5787;

        *var_betnedge_p_slot = var_betnedge_p;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_cth_p_slot = var_cth_p;
        *var_deltarth_slot = var_deltarth;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gpe_edge_slot = var_gpe_edge;
        *var_guard50_slot = var_guard50;
        *var_guard51_slot = var_guard51;
        *var_guard52_slot = var_guard52;
        *var_guard53_slot = var_guard53;
        *var_guard54_slot = var_guard54;
        *var_guard55_slot = var_guard55;
        *var_guard56_slot = var_guard56;
        *var_kuowe_slot = var_kuowe;
        *var_kvthowe_slot = var_kvthowe;
        *var_neff_p_slot = var_neff_p;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_rbulk_p_slot = var_rbulk_p;
        *var_rde_p_slot = var_rde_p;
        *var_rg_p_slot = var_rg_p;
        *var_rjund_p_slot = var_rjund_p;
        *var_rjuns_p_slot = var_rjuns_p;
        *var_rse_p_slot = var_rse_p;
        *var_rsh_i_slot = var_rsh_i;
        *var_rshd_i_slot = var_rshd_i;
        *var_rth_p_slot = var_rth_p;
        *var_rwell_p_slot = var_rwell_p;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_strth_p_slot = var_strth_p;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vsbnud_p_slot = var_vsbnud_p;
    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_guard56: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_betn_p_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard58_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard61_slot: &mut f64,
        var_guard62_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_guard65_slot: &mut f64,
        var_guard66_slot: &mut f64,
        var_guard67_slot: &mut f64,
        var_guard68_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard84_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
    ) {
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard58: f64 = *var_guard58_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard61: f64 = *var_guard61_slot;
        let mut var_guard62: f64 = *var_guard62_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_guard65: f64 = *var_guard65_slot;
        let mut var_guard66: f64 = *var_guard66_slot;
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;

        let (assign7250_e5805,) = {
    if ((var_guard36 != 0.0) && (var_guard56 != 0.0)) {
        let assign7250_e5794: f64 = (p.p481 * var_ile);
        let assign7250_e5795: f64 = (p.p480 + assign7250_e5794);
        let assign7250_e5798: f64 = (p.p482 * var_iwe);
        let assign7250_e5799: f64 = (assign7250_e5795 + assign7250_e5798);
        let assign7250_e5802: f64 = (p.p483 * var_iae);
        let assign7250_e5803: f64 = (assign7250_e5799 + assign7250_e5802);
        (assign7250_e5803,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign7250_e5805;

        let assign7260_e5824: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        var_guard57 = assign7260_e5824;

        let (assign7270_e5842,) = {
    if ((var_guard36 != 0.0) && (var_guard57 != 0.0)) {
        let assign7270_e5831: f64 = (p.p485 * var_ile);
        let assign7270_e5832: f64 = (p.p484 + assign7270_e5831);
        let assign7270_e5835: f64 = (p.p486 * var_iwe);
        let assign7270_e5836: f64 = (assign7270_e5832 + assign7270_e5835);
        let assign7270_e5839: f64 = (p.p487 * var_iae);
        let assign7270_e5840: f64 = (assign7270_e5836 + assign7270_e5839);
        (assign7270_e5840,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign7270_e5842;

        let assign7280_e5861: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        var_guard58 = assign7280_e5861;

        let (assign7290_e5879,) = {
    if ((var_guard36 != 0.0) && (var_guard58 != 0.0)) {
        let assign7290_e5868: f64 = (p.p489 * var_ile);
        let assign7290_e5869: f64 = (p.p488 + assign7290_e5868);
        let assign7290_e5872: f64 = (p.p490 * var_iwe);
        let assign7290_e5873: f64 = (assign7290_e5869 + assign7290_e5872);
        let assign7290_e5876: f64 = (p.p491 * var_iae);
        let assign7290_e5877: f64 = (assign7290_e5873 + assign7290_e5876);
        (assign7290_e5877,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign7290_e5879;

        let assign7300_e5898: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        var_guard59 = assign7300_e5898;

        let (assign7310_e5916,) = {
    if ((var_guard36 != 0.0) && (var_guard59 != 0.0)) {
        let assign7310_e5905: f64 = (p.p493 * var_ile);
        let assign7310_e5906: f64 = (p.p492 + assign7310_e5905);
        let assign7310_e5909: f64 = (p.p494 * var_iwe);
        let assign7310_e5910: f64 = (assign7310_e5906 + assign7310_e5909);
        let assign7310_e5913: f64 = (p.p495 * var_iae);
        let assign7310_e5914: f64 = (assign7310_e5910 + assign7310_e5913);
        (assign7310_e5914,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign7310_e5916;

        let assign7320_e5935: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        var_guard60 = assign7320_e5935;

        let (assign7330_e5953,) = {
    if ((var_guard36 != 0.0) && (var_guard60 != 0.0)) {
        let assign7330_e5942: f64 = (p.p497 * var_ile);
        let assign7330_e5943: f64 = (p.p496 + assign7330_e5942);
        let assign7330_e5946: f64 = (p.p498 * var_iwe);
        let assign7330_e5947: f64 = (assign7330_e5943 + assign7330_e5946);
        let assign7330_e5950: f64 = (p.p499 * var_iae);
        let assign7330_e5951: f64 = (assign7330_e5947 + assign7330_e5950);
        (assign7330_e5951,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign7330_e5953;

        let assign7340_e5972: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        var_guard61 = assign7340_e5972;

        let (assign7350_e5990,) = {
    if ((var_guard36 != 0.0) && (var_guard61 != 0.0)) {
        let assign7350_e5979: f64 = (p.p505 * var_ile);
        let assign7350_e5980: f64 = (p.p504 + assign7350_e5979);
        let assign7350_e5983: f64 = (p.p506 * var_iwe);
        let assign7350_e5984: f64 = (assign7350_e5980 + assign7350_e5983);
        let assign7350_e5987: f64 = (p.p507 * var_iae);
        let assign7350_e5988: f64 = (assign7350_e5984 + assign7350_e5987);
        (assign7350_e5988,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign7350_e5990;

        let assign7360_e6009: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        var_guard62 = assign7360_e6009;

        let (assign7370_e6027,) = {
    if ((var_guard36 != 0.0) && (var_guard62 != 0.0)) {
        let assign7370_e6016: f64 = (p.p501 * var_ile);
        let assign7370_e6017: f64 = (p.p500 + assign7370_e6016);
        let assign7370_e6020: f64 = (p.p502 * var_iwe);
        let assign7370_e6021: f64 = (assign7370_e6017 + assign7370_e6020);
        let assign7370_e6024: f64 = (p.p503 * var_iae);
        let assign7370_e6025: f64 = (assign7370_e6021 + assign7370_e6024);
        (assign7370_e6025,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign7370_e6027;

        let assign7380_e6046: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        var_guard63 = assign7380_e6046;

        let (assign7390_e6064,) = {
    if ((var_guard36 != 0.0) && (var_guard63 != 0.0)) {
        let assign7390_e6053: f64 = (p.p509 * var_ile);
        let assign7390_e6054: f64 = (p.p508 + assign7390_e6053);
        let assign7390_e6057: f64 = (p.p510 * var_iwe);
        let assign7390_e6058: f64 = (assign7390_e6054 + assign7390_e6057);
        let assign7390_e6061: f64 = (p.p511 * var_iae);
        let assign7390_e6062: f64 = (assign7390_e6058 + assign7390_e6061);
        (assign7390_e6062,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign7390_e6064;

        let assign7400_e6083: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        var_guard64 = assign7400_e6083;

        let (assign7410_e6103,) = {
    if ((var_guard36 != 0.0) && (var_guard64 != 0.0)) {
        let assign7410_e6091: f64 = (p.p513 * var_ile);
        let assign7410_e6092: f64 = (p.p512 + assign7410_e6091);
        let assign7410_e6095: f64 = (p.p514 * var_iwe);
        let assign7410_e6096: f64 = (assign7410_e6092 + assign7410_e6095);
        let assign7410_e6099: f64 = (p.p515 * var_iae);
        let assign7410_e6100: f64 = (assign7410_e6096 + assign7410_e6099);
        let assign7410_e6101: f64 = (var_ile2 * assign7410_e6100);
        (assign7410_e6101,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign7410_e6103;

        let assign7420_e6122: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        var_guard65 = assign7420_e6122;

        let (assign7430_e6140,) = {
    if ((var_guard36 != 0.0) && (var_guard65 != 0.0)) {
        let assign7430_e6129: f64 = (p.p521 * var_ile);
        let assign7430_e6130: f64 = (p.p520 + assign7430_e6129);
        let assign7430_e6133: f64 = (p.p522 * var_iwe);
        let assign7430_e6134: f64 = (assign7430_e6130 + assign7430_e6133);
        let assign7430_e6137: f64 = (p.p523 * var_iae);
        let assign7430_e6138: f64 = (assign7430_e6134 + assign7430_e6137);
        (assign7430_e6138,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign7430_e6140;

        let assign7440_e6159: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        var_guard66 = assign7440_e6159;

        let (assign7450_e6177,) = {
    if ((var_guard36 != 0.0) && (var_guard66 != 0.0)) {
        let assign7450_e6166: f64 = (p.p517 * var_ile);
        let assign7450_e6167: f64 = (p.p516 + assign7450_e6166);
        let assign7450_e6170: f64 = (p.p518 * var_iwe);
        let assign7450_e6171: f64 = (assign7450_e6167 + assign7450_e6170);
        let assign7450_e6174: f64 = (p.p519 * var_iae);
        let assign7450_e6175: f64 = (assign7450_e6171 + assign7450_e6174);
        (assign7450_e6175,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign7450_e6177;

        let assign7460_e6196: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        var_guard67 = assign7460_e6196;

        let (assign7470_e6216,) = {
    if ((var_guard36 != 0.0) && (var_guard67 != 0.0)) {
        let assign7470_e6204: f64 = (p.p525 * var_ile);
        let assign7470_e6205: f64 = (p.p524 + assign7470_e6204);
        let assign7470_e6208: f64 = (p.p526 * var_iwe);
        let assign7470_e6209: f64 = (assign7470_e6205 + assign7470_e6208);
        let assign7470_e6212: f64 = (p.p527 * var_iae);
        let assign7470_e6213: f64 = (assign7470_e6209 + assign7470_e6212);
        let assign7470_e6214: f64 = (var_ile2 * assign7470_e6213);
        (assign7470_e6214,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign7470_e6216;

        let assign7480_e6235: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        var_guard68 = assign7480_e6235;

        let (assign7490_e6253,) = {
    if ((var_guard36 != 0.0) && (var_guard68 != 0.0)) {
        let assign7490_e6242: f64 = (p.p533 * var_ile);
        let assign7490_e6243: f64 = (p.p532 + assign7490_e6242);
        let assign7490_e6246: f64 = (p.p534 * var_iwe);
        let assign7490_e6247: f64 = (assign7490_e6243 + assign7490_e6246);
        let assign7490_e6250: f64 = (p.p535 * var_iae);
        let assign7490_e6251: f64 = (assign7490_e6247 + assign7490_e6250);
        (assign7490_e6251,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign7490_e6253;

        let assign7500_e6272: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        var_guard69 = assign7500_e6272;

        let (assign7510_e6290,) = {
    if ((var_guard36 != 0.0) && (var_guard69 != 0.0)) {
        let assign7510_e6279: f64 = (p.p529 * var_ile);
        let assign7510_e6280: f64 = (p.p528 + assign7510_e6279);
        let assign7510_e6283: f64 = (p.p530 * var_iwe);
        let assign7510_e6284: f64 = (assign7510_e6280 + assign7510_e6283);
        let assign7510_e6287: f64 = (p.p531 * var_iae);
        let assign7510_e6288: f64 = (assign7510_e6284 + assign7510_e6287);
        (assign7510_e6288,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign7510_e6290;

        let assign7520_e6309: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        var_guard70 = assign7520_e6309;

        let (assign7530_e6331,) = {
    if ((var_guard36 != 0.0) && (var_guard70 != 0.0)) {
        let assign7530_e6315: f64 = (var_we / var_le);
        let assign7530_e6319: f64 = (p.p537 * var_ile);
        let assign7530_e6320: f64 = (p.p536 + assign7530_e6319);
        let assign7530_e6323: f64 = (p.p538 * var_iwe);
        let assign7530_e6324: f64 = (assign7530_e6320 + assign7530_e6323);
        let assign7530_e6327: f64 = (p.p539 * var_iae);
        let assign7530_e6328: f64 = (assign7530_e6324 + assign7530_e6327);
        let assign7530_e6329: f64 = (assign7530_e6315 * assign7530_e6328);
        (assign7530_e6329,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign7530_e6331;

        let assign7540_e6350: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        var_guard71 = assign7540_e6350;

        let (assign7550_e6368,) = {
    if ((var_guard36 != 0.0) && (var_guard71 != 0.0)) {
        let assign7550_e6357: f64 = (p.p541 * var_ile);
        let assign7550_e6358: f64 = (p.p540 + assign7550_e6357);
        let assign7550_e6361: f64 = (p.p542 * var_iwe);
        let assign7550_e6362: f64 = (assign7550_e6358 + assign7550_e6361);
        let assign7550_e6365: f64 = (p.p543 * var_iae);
        let assign7550_e6366: f64 = (assign7550_e6362 + assign7550_e6365);
        (assign7550_e6366,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign7550_e6368;

        let assign7560_e6387: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        var_guard72 = assign7560_e6387;

        let (assign7570_e6405,) = {
    if ((var_guard36 != 0.0) && (var_guard72 != 0.0)) {
        let assign7570_e6394: f64 = (p.p545 * var_ile);
        let assign7570_e6395: f64 = (p.p544 + assign7570_e6394);
        let assign7570_e6398: f64 = (p.p546 * var_iwe);
        let assign7570_e6399: f64 = (assign7570_e6395 + assign7570_e6398);
        let assign7570_e6402: f64 = (p.p547 * var_iae);
        let assign7570_e6403: f64 = (assign7570_e6399 + assign7570_e6402);
        (assign7570_e6403,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign7570_e6405;

        let assign7580_e6424: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        var_guard73 = assign7580_e6424;

        let (assign7590_e6442,) = {
    if ((var_guard36 != 0.0) && (var_guard73 != 0.0)) {
        let assign7590_e6431: f64 = (p.p549 * var_ile);
        let assign7590_e6432: f64 = (p.p548 + assign7590_e6431);
        let assign7590_e6435: f64 = (p.p550 * var_iwe);
        let assign7590_e6436: f64 = (assign7590_e6432 + assign7590_e6435);
        let assign7590_e6439: f64 = (p.p551 * var_iae);
        let assign7590_e6440: f64 = (assign7590_e6436 + assign7590_e6439);
        (assign7590_e6440,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign7590_e6442;

        let assign7600_e6461: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        var_guard74 = assign7600_e6461;

        let (assign7610_e6479,) = {
    if ((var_guard36 != 0.0) && (var_guard74 != 0.0)) {
        let assign7610_e6468: f64 = (p.p553 * var_ile);
        let assign7610_e6469: f64 = (p.p552 + assign7610_e6468);
        let assign7610_e6472: f64 = (p.p554 * var_iwe);
        let assign7610_e6473: f64 = (assign7610_e6469 + assign7610_e6472);
        let assign7610_e6476: f64 = (p.p555 * var_iae);
        let assign7610_e6477: f64 = (assign7610_e6473 + assign7610_e6476);
        (assign7610_e6477,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign7610_e6479;

        let assign7620_e6498: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        var_guard75 = assign7620_e6498;

        let (assign7630_e6516,) = {
    if ((var_guard36 != 0.0) && (var_guard75 != 0.0)) {
        let assign7630_e6505: f64 = (p.p557 * var_ile);
        let assign7630_e6506: f64 = (p.p556 + assign7630_e6505);
        let assign7630_e6509: f64 = (p.p558 * var_iwe);
        let assign7630_e6510: f64 = (assign7630_e6506 + assign7630_e6509);
        let assign7630_e6513: f64 = (p.p559 * var_iae);
        let assign7630_e6514: f64 = (assign7630_e6510 + assign7630_e6513);
        (assign7630_e6514,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign7630_e6516;

        let assign7640_e6535: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        var_guard76 = assign7640_e6535;

        let (assign7650_e6553,) = {
    if ((var_guard36 != 0.0) && (var_guard76 != 0.0)) {
        let assign7650_e6542: f64 = (p.p561 * var_ile);
        let assign7650_e6543: f64 = (p.p560 + assign7650_e6542);
        let assign7650_e6546: f64 = (p.p562 * var_iwe);
        let assign7650_e6547: f64 = (assign7650_e6543 + assign7650_e6546);
        let assign7650_e6550: f64 = (p.p563 * var_iae);
        let assign7650_e6551: f64 = (assign7650_e6547 + assign7650_e6550);
        (assign7650_e6551,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign7650_e6553;

        let assign7660_e6572: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        var_guard77 = assign7660_e6572;

        let (assign7670_e6592,) = {
    if ((var_guard36 != 0.0) && (var_guard77 != 0.0)) {
        let assign7670_e6580: f64 = (p.p565 * var_ile);
        let assign7670_e6581: f64 = (p.p564 + assign7670_e6580);
        let assign7670_e6584: f64 = (p.p566 * var_iwe);
        let assign7670_e6585: f64 = (assign7670_e6581 + assign7670_e6584);
        let assign7670_e6588: f64 = (p.p567 * var_iae);
        let assign7670_e6589: f64 = (assign7670_e6585 + assign7670_e6588);
        let assign7670_e6590: f64 = (var_iwe * assign7670_e6589);
        (assign7670_e6590,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign7670_e6592;

        let assign7680_e6611: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        var_guard78 = assign7680_e6611;

        let (assign7690_e6629,) = {
    if ((var_guard36 != 0.0) && (var_guard78 != 0.0)) {
        let assign7690_e6618: f64 = (p.p569 * var_ile);
        let assign7690_e6619: f64 = (p.p568 + assign7690_e6618);
        let assign7690_e6622: f64 = (p.p570 * var_iwe);
        let assign7690_e6623: f64 = (assign7690_e6619 + assign7690_e6622);
        let assign7690_e6626: f64 = (p.p571 * var_iae);
        let assign7690_e6627: f64 = (assign7690_e6623 + assign7690_e6626);
        (assign7690_e6627,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign7690_e6629;

        let assign7700_e6648: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        var_guard79 = assign7700_e6648;

        let (assign7710_e6666,) = {
    if ((var_guard36 != 0.0) && (var_guard79 != 0.0)) {
        let assign7710_e6655: f64 = (p.p573 * var_ile);
        let assign7710_e6656: f64 = (p.p572 + assign7710_e6655);
        let assign7710_e6659: f64 = (p.p574 * var_iwe);
        let assign7710_e6660: f64 = (assign7710_e6656 + assign7710_e6659);
        let assign7710_e6663: f64 = (p.p575 * var_iae);
        let assign7710_e6664: f64 = (assign7710_e6660 + assign7710_e6663);
        (assign7710_e6664,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign7710_e6666;

        let assign7720_e6685: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        var_guard80 = assign7720_e6685;

        let (assign7730_e6703,) = {
    if ((var_guard36 != 0.0) && (var_guard80 != 0.0)) {
        let assign7730_e6692: f64 = (p.p577 * var_ile);
        let assign7730_e6693: f64 = (p.p576 + assign7730_e6692);
        let assign7730_e6696: f64 = (p.p578 * var_iwe);
        let assign7730_e6697: f64 = (assign7730_e6693 + assign7730_e6696);
        let assign7730_e6700: f64 = (p.p579 * var_iae);
        let assign7730_e6701: f64 = (assign7730_e6697 + assign7730_e6700);
        (assign7730_e6701,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign7730_e6703;

        let assign7740_e6722: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        var_guard81 = assign7740_e6722;

        let (assign7750_e6742,) = {
    if ((var_guard36 != 0.0) && (var_guard81 != 0.0)) {
        let assign7750_e6730: f64 = (p.p581 * var_ile);
        let assign7750_e6731: f64 = (p.p580 + assign7750_e6730);
        let assign7750_e6734: f64 = (p.p582 * var_iwe);
        let assign7750_e6735: f64 = (assign7750_e6731 + assign7750_e6734);
        let assign7750_e6738: f64 = (p.p583 * var_iae);
        let assign7750_e6739: f64 = (assign7750_e6735 + assign7750_e6738);
        let assign7750_e6740: f64 = (var_ile * assign7750_e6739);
        (assign7750_e6740,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign7750_e6742;

        let assign7760_e6761: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        var_guard82 = assign7760_e6761;

        let (assign7770_e6779,) = {
    if ((var_guard36 != 0.0) && (var_guard82 != 0.0)) {
        let assign7770_e6768: f64 = (p.p585 * var_ile);
        let assign7770_e6769: f64 = (p.p584 + assign7770_e6768);
        let assign7770_e6772: f64 = (p.p586 * var_iwe);
        let assign7770_e6773: f64 = (assign7770_e6769 + assign7770_e6772);
        let assign7770_e6776: f64 = (p.p587 * var_iae);
        let assign7770_e6777: f64 = (assign7770_e6773 + assign7770_e6776);
        (assign7770_e6777,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign7770_e6779;

        let assign7780_e6798: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        var_guard83 = assign7780_e6798;

        let (assign7790_e6816,) = {
    if ((var_guard36 != 0.0) && (var_guard83 != 0.0)) {
        let assign7790_e6805: f64 = (p.p589 * var_ile);
        let assign7790_e6806: f64 = (p.p588 + assign7790_e6805);
        let assign7790_e6809: f64 = (p.p590 * var_iwe);
        let assign7790_e6810: f64 = (assign7790_e6806 + assign7790_e6809);
        let assign7790_e6813: f64 = (p.p591 * var_iae);
        let assign7790_e6814: f64 = (assign7790_e6810 + assign7790_e6813);
        (assign7790_e6814,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign7790_e6816;

        let assign7800_e6835: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        var_guard84 = assign7800_e6835;

        *var_betn_p_slot = var_betn_p;
        *var_cf_p_slot = var_cf_p;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfd_p_slot = var_cfd_p;
        *var_cs_p_slot = var_cs_p;
        *var_ct_p_slot = var_ct_p;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctg_p_slot = var_ctg_p;
        *var_dphib_p_slot = var_dphib_p;
        *var_guard57_slot = var_guard57;
        *var_guard58_slot = var_guard58;
        *var_guard59_slot = var_guard59;
        *var_guard60_slot = var_guard60;
        *var_guard61_slot = var_guard61;
        *var_guard62_slot = var_guard62;
        *var_guard63_slot = var_guard63;
        *var_guard64_slot = var_guard64;
        *var_guard65_slot = var_guard65;
        *var_guard66_slot = var_guard66;
        *var_guard67_slot = var_guard67;
        *var_guard68_slot = var_guard68;
        *var_guard69_slot = var_guard69;
        *var_guard70_slot = var_guard70;
        *var_guard71_slot = var_guard71;
        *var_guard72_slot = var_guard72;
        *var_guard73_slot = var_guard73;
        *var_guard74_slot = var_guard74;
        *var_guard75_slot = var_guard75;
        *var_guard76_slot = var_guard76;
        *var_guard77_slot = var_guard77;
        *var_guard78_slot = var_guard78;
        *var_guard79_slot = var_guard79;
        *var_guard80_slot = var_guard80;
        *var_guard81_slot = var_guard81;
        *var_guard82_slot = var_guard82;
        *var_guard83_slot = var_guard83;
        *var_guard84_slot = var_guard84;
        *var_mue_p_slot = var_mue_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
        *var_psce_p_slot = var_psce_p;
        *var_psceb_p_slot = var_psceb_p;
        *var_psced_p_slot = var_psced_p;
        *var_rs_p_slot = var_rs_p;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsg_p_slot = var_rsg_p;
        *var_stbet_p_slot = var_stbet_p;
        *var_stct_p_slot = var_stct_p;
        *var_strs_p_slot = var_strs_p;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_thecs_p_slot = var_thecs_p;
        *var_themu_p_slot = var_themu_p;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_xcor_p_slot = var_xcor_p;
    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_guard84: f64,
        var_iae: f64,
        var_iiae: f64,
        var_iiwe: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_a1_p_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_plparam_i_slot: &mut f64,
        var_plwparam_i_slot: &mut f64,
        var_poparam_i_slot: &mut f64,
        var_pwparam_i_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_plparam_i: f64 = *var_plparam_i_slot;
        let mut var_plwparam_i: f64 = *var_plwparam_i_slot;
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
        let mut var_pwparam_i: f64 = *var_pwparam_i_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;

        let (assign7810_e6853,) = {
    if ((var_guard36 != 0.0) && (var_guard84 != 0.0)) {
        let assign7810_e6842: f64 = (p.p593 * var_ile);
        let assign7810_e6843: f64 = (p.p592 + assign7810_e6842);
        let assign7810_e6846: f64 = (p.p594 * var_iwe);
        let assign7810_e6847: f64 = (assign7810_e6843 + assign7810_e6846);
        let assign7810_e6850: f64 = (p.p595 * var_iae);
        let assign7810_e6851: f64 = (assign7810_e6847 + assign7810_e6850);
        (assign7810_e6851,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign7810_e6853;

        let assign7820_e6872: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        var_guard85 = assign7820_e6872;

        let (assign7830_e6890,) = {
    if ((var_guard36 != 0.0) && (var_guard85 != 0.0)) {
        let assign7830_e6879: f64 = (p.p597 * var_ile);
        let assign7830_e6880: f64 = (p.p596 + assign7830_e6879);
        let assign7830_e6883: f64 = (p.p598 * var_iwe);
        let assign7830_e6884: f64 = (assign7830_e6880 + assign7830_e6883);
        let assign7830_e6887: f64 = (p.p599 * var_iae);
        let assign7830_e6888: f64 = (assign7830_e6884 + assign7830_e6887);
        (assign7830_e6888,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign7830_e6890;

        let assign7840_e6909: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        var_guard86 = assign7840_e6909;

        let (assign7850_e6929,) = {
    if ((var_guard36 != 0.0) && (var_guard86 != 0.0)) {
        let assign7850_e6917: f64 = (p.p601 * var_ile);
        let assign7850_e6918: f64 = (p.p600 + assign7850_e6917);
        let assign7850_e6921: f64 = (p.p602 * var_iwe);
        let assign7850_e6922: f64 = (assign7850_e6918 + assign7850_e6921);
        let assign7850_e6925: f64 = (p.p603 * var_iae);
        let assign7850_e6926: f64 = (assign7850_e6922 + assign7850_e6925);
        let assign7850_e6927: f64 = (var_ile * assign7850_e6926);
        (assign7850_e6927,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign7850_e6929;

        let assign7860_e6948: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        var_guard87 = assign7860_e6948;

        let (assign7870_e6966,) = {
    if ((var_guard36 != 0.0) && (var_guard87 != 0.0)) {
        let assign7870_e6955: f64 = (p.p605 * var_ile);
        let assign7870_e6956: f64 = (p.p604 + assign7870_e6955);
        let assign7870_e6959: f64 = (p.p606 * var_iwe);
        let assign7870_e6960: f64 = (assign7870_e6956 + assign7870_e6959);
        let assign7870_e6963: f64 = (p.p607 * var_iae);
        let assign7870_e6964: f64 = (assign7870_e6960 + assign7870_e6963);
        (assign7870_e6964,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign7870_e6966;

        let assign7880_e6985: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        var_guard88 = assign7880_e6985;

        let (assign7890_e7003,) = {
    if ((var_guard36 != 0.0) && (var_guard88 != 0.0)) {
        let assign7890_e6992: f64 = (p.p609 * var_ile);
        let assign7890_e6993: f64 = (p.p608 + assign7890_e6992);
        let assign7890_e6996: f64 = (p.p610 * var_iwe);
        let assign7890_e6997: f64 = (assign7890_e6993 + assign7890_e6996);
        let assign7890_e7000: f64 = (p.p611 * var_iae);
        let assign7890_e7001: f64 = (assign7890_e6997 + assign7890_e7000);
        (assign7890_e7001,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign7890_e7003;

        let assign7900_e7022: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        var_guard89 = assign7900_e7022;

        let (assign7910_e7040,) = {
    if ((var_guard36 != 0.0) && (var_guard89 != 0.0)) {
        let assign7910_e7029: f64 = (p.p613 * var_ile);
        let assign7910_e7030: f64 = (p.p612 + assign7910_e7029);
        let assign7910_e7033: f64 = (p.p614 * var_iwe);
        let assign7910_e7034: f64 = (assign7910_e7030 + assign7910_e7033);
        let assign7910_e7037: f64 = (p.p615 * var_iae);
        let assign7910_e7038: f64 = (assign7910_e7034 + assign7910_e7037);
        (assign7910_e7038,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign7910_e7040;

        let assign7920_e7059: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        var_guard90 = assign7920_e7059;

        let (assign7930_e7077,) = {
    if ((var_guard36 != 0.0) && (var_guard90 != 0.0)) {
        let assign7930_e7066: f64 = (p.p617 * var_ile);
        let assign7930_e7067: f64 = (p.p616 + assign7930_e7066);
        let assign7930_e7070: f64 = (p.p618 * var_iwe);
        let assign7930_e7071: f64 = (assign7930_e7067 + assign7930_e7070);
        let assign7930_e7074: f64 = (p.p619 * var_iae);
        let assign7930_e7075: f64 = (assign7930_e7071 + assign7930_e7074);
        (assign7930_e7075,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign7930_e7077;

        let assign7940_e7096: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        var_guard91 = assign7940_e7096;

        let (assign7950_e7114,) = {
    if ((var_guard36 != 0.0) && (var_guard91 != 0.0)) {
        let assign7950_e7103: f64 = (p.p621 * var_ile);
        let assign7950_e7104: f64 = (p.p620 + assign7950_e7103);
        let assign7950_e7107: f64 = (p.p622 * var_iwe);
        let assign7950_e7108: f64 = (assign7950_e7104 + assign7950_e7107);
        let assign7950_e7111: f64 = (p.p623 * var_iae);
        let assign7950_e7112: f64 = (assign7950_e7108 + assign7950_e7111);
        (assign7950_e7112,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign7950_e7114;

        let assign7960_e7133: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        var_guard92 = assign7960_e7133;

        let (assign7970_e7151,) = {
    if ((var_guard36 != 0.0) && (var_guard92 != 0.0)) {
        let assign7970_e7140: f64 = (p.p625 * var_ile);
        let assign7970_e7141: f64 = (p.p624 + assign7970_e7140);
        let assign7970_e7144: f64 = (p.p626 * var_iwe);
        let assign7970_e7145: f64 = (assign7970_e7141 + assign7970_e7144);
        let assign7970_e7148: f64 = (p.p627 * var_iae);
        let assign7970_e7149: f64 = (assign7970_e7145 + assign7970_e7148);
        (assign7970_e7149,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign7970_e7151;

        let assign7980_e7170: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        var_guard93 = assign7980_e7170;

        let (assign7990_e7190,) = {
    if ((var_guard36 != 0.0) && (var_guard93 != 0.0)) {
        let assign7990_e7178: f64 = (p.p629 * var_ile);
        let assign7990_e7179: f64 = (p.p628 + assign7990_e7178);
        let assign7990_e7182: f64 = (p.p630 * var_iwe);
        let assign7990_e7183: f64 = (assign7990_e7179 + assign7990_e7182);
        let assign7990_e7186: f64 = (p.p631 * var_iae);
        let assign7990_e7187: f64 = (assign7990_e7183 + assign7990_e7186);
        let assign7990_e7188: f64 = (var_iiae * assign7990_e7187);
        (assign7990_e7188,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign7990_e7190;

        let assign8000_e7209: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        var_guard94 = assign8000_e7209;

        let (assign8010_e7229,) = {
    if ((var_guard36 != 0.0) && (var_guard94 != 0.0)) {
        let assign8010_e7217: f64 = (p.p633 * var_ile);
        let assign8010_e7218: f64 = (p.p632 + assign8010_e7217);
        let assign8010_e7221: f64 = (p.p634 * var_iwe);
        let assign8010_e7222: f64 = (assign8010_e7218 + assign8010_e7221);
        let assign8010_e7225: f64 = (p.p635 * var_iae);
        let assign8010_e7226: f64 = (assign8010_e7222 + assign8010_e7225);
        let assign8010_e7227: f64 = (var_iiwe * assign8010_e7226);
        (assign8010_e7227,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign8010_e7229;

        let assign8020_e7248: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        var_guard95 = assign8020_e7248;

        let (assign8030_e7268,) = {
    if ((var_guard36 != 0.0) && (var_guard95 != 0.0)) {
        let assign8030_e7256: f64 = (p.p637 * var_ile);
        let assign8030_e7257: f64 = (p.p636 + assign8030_e7256);
        let assign8030_e7260: f64 = (p.p638 * var_iwe);
        let assign8030_e7261: f64 = (assign8030_e7257 + assign8030_e7260);
        let assign8030_e7264: f64 = (p.p639 * var_iae);
        let assign8030_e7265: f64 = (assign8030_e7261 + assign8030_e7264);
        let assign8030_e7266: f64 = (var_iiwe * assign8030_e7265);
        (assign8030_e7266,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign8030_e7268;

        let assign8040_e7287: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        var_guard96 = assign8040_e7287;

        let (assign8050_e7305,) = {
    if ((var_guard36 != 0.0) && (var_guard96 != 0.0)) {
        let assign8050_e7294: f64 = (p.p641 * var_ile);
        let assign8050_e7295: f64 = (p.p640 + assign8050_e7294);
        let assign8050_e7298: f64 = (p.p642 * var_iwe);
        let assign8050_e7299: f64 = (assign8050_e7295 + assign8050_e7298);
        let assign8050_e7302: f64 = (p.p643 * var_iae);
        let assign8050_e7303: f64 = (assign8050_e7299 + assign8050_e7302);
        (assign8050_e7303,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign8050_e7305;

        let assign8060_e7324: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        var_guard97 = assign8060_e7324;

        let (assign8070_e7344,) = {
    if ((var_guard36 != 0.0) && (var_guard97 != 0.0)) {
        let assign8070_e7332: f64 = (p.p645 * var_ile);
        let assign8070_e7333: f64 = (p.p644 + assign8070_e7332);
        let assign8070_e7336: f64 = (p.p646 * var_iwe);
        let assign8070_e7337: f64 = (assign8070_e7333 + assign8070_e7336);
        let assign8070_e7340: f64 = (p.p647 * var_iae);
        let assign8070_e7341: f64 = (assign8070_e7337 + assign8070_e7340);
        let assign8070_e7342: f64 = (var_iiwe * assign8070_e7341);
        (assign8070_e7342,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign8070_e7344;

        let assign8080_e7363: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        var_guard98 = assign8080_e7363;

        let (assign8090_e7383,) = {
    if ((var_guard36 != 0.0) && (var_guard98 != 0.0)) {
        let assign8090_e7371: f64 = (p.p649 * var_ile);
        let assign8090_e7372: f64 = (p.p648 + assign8090_e7371);
        let assign8090_e7375: f64 = (p.p650 * var_iwe);
        let assign8090_e7376: f64 = (assign8090_e7372 + assign8090_e7375);
        let assign8090_e7379: f64 = (p.p651 * var_iae);
        let assign8090_e7380: f64 = (assign8090_e7376 + assign8090_e7379);
        let assign8090_e7381: f64 = (var_iiwe * assign8090_e7380);
        (assign8090_e7381,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign8090_e7383;

        let assign8100_e7402: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        var_guard99 = assign8100_e7402;

        let (assign8110_e7420,) = {
    if ((var_guard36 != 0.0) && (var_guard99 != 0.0)) {
        let assign8110_e7409: f64 = (p.p653 * var_ile);
        let assign8110_e7410: f64 = (p.p652 + assign8110_e7409);
        let assign8110_e7413: f64 = (p.p654 * var_iwe);
        let assign8110_e7414: f64 = (assign8110_e7410 + assign8110_e7413);
        let assign8110_e7417: f64 = (p.p655 * var_iae);
        let assign8110_e7418: f64 = (assign8110_e7414 + assign8110_e7417);
        (assign8110_e7418,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign8110_e7420;

        let assign8120_e7439: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        var_guard100 = assign8120_e7439;

        let (assign8130_e7457,) = {
    if ((var_guard36 != 0.0) && (var_guard100 != 0.0)) {
        let assign8130_e7446: f64 = (p.p657 * var_ile);
        let assign8130_e7447: f64 = (p.p656 + assign8130_e7446);
        let assign8130_e7450: f64 = (p.p658 * var_iwe);
        let assign8130_e7451: f64 = (assign8130_e7447 + assign8130_e7450);
        let assign8130_e7454: f64 = (p.p659 * var_iae);
        let assign8130_e7455: f64 = (assign8130_e7451 + assign8130_e7454);
        (assign8130_e7455,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign8130_e7457;

        s.b[1081] = (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]);
        s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });

        if ((s.v[1016] != 0.0) && s.b[1081]) {
            s.store_scalar(118, (((var_iiwecv * s.v[320]) / 1e-6) * (((p.p660 + (p.p661 * var_ile)) + (p.p662 * var_iwe)) + (p.p663 * var_iae))));
        }

        let assign8160_e7519: f64 = if (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) { 1.0 } else { 0.0 };
        var_guard102 = assign8160_e7519;

        let (assign8170_e7537,) = {
    if ((var_guard36 != 0.0) && (var_guard102 != 0.0)) {
        let assign8170_e7526: f64 = (p.p665 * var_ile);
        let assign8170_e7527: f64 = (p.p664 + assign8170_e7526);
        let assign8170_e7530: f64 = (p.p666 * var_iwe);
        let assign8170_e7531: f64 = (assign8170_e7527 + assign8170_e7530);
        let assign8170_e7534: f64 = (p.p667 * var_iae);
        let assign8170_e7535: f64 = (assign8170_e7531 + assign8170_e7534);
        (assign8170_e7535,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign8170_e7537;

        let assign8180_e7556: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        var_guard103 = assign8180_e7556;

        let (assign8190_e7574,) = {
    if ((var_guard36 != 0.0) && (var_guard103 != 0.0)) {
        let assign8190_e7563: f64 = (p.p669 * var_ile);
        let assign8190_e7564: f64 = (p.p668 + assign8190_e7563);
        let assign8190_e7567: f64 = (p.p670 * var_iwe);
        let assign8190_e7568: f64 = (assign8190_e7564 + assign8190_e7567);
        let assign8190_e7571: f64 = (p.p671 * var_iae);
        let assign8190_e7572: f64 = (assign8190_e7568 + assign8190_e7571);
        (assign8190_e7572,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign8190_e7574;

        let assign8200_e7613: f64 = if (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        var_guard104 = assign8200_e7613;

        let (assign8210_e7619,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p580,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8210_e7619;

        let assign8220_e7621: f64 = if param_given[672] { 1.0 } else { 0.0 };
        let assign8220_e7623: f64 = if assign8220_e7621 == 1.0 { 1.0 } else { 0.0 };
        var_guard105 = assign8220_e7623;

        let (assign8230_e7631,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard105 != 0.0)) {
        (p.p672,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8230_e7631;

        let (assign8240_e7637,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p581,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8240_e7637;

        let assign8250_e7639: f64 = if param_given[673] { 1.0 } else { 0.0 };
        let assign8250_e7641: f64 = if assign8250_e7639 == 1.0 { 1.0 } else { 0.0 };
        var_guard106 = assign8250_e7641;

        let (assign8260_e7649,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard106 != 0.0)) {
        (p.p673,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8260_e7649;

        let (assign8270_e7655,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p582,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8270_e7655;

        let assign8280_e7657: f64 = if param_given[674] { 1.0 } else { 0.0 };
        let assign8280_e7659: f64 = if assign8280_e7657 == 1.0 { 1.0 } else { 0.0 };
        var_guard107 = assign8280_e7659;

        let (assign8290_e7667,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard107 != 0.0)) {
        (p.p674,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8290_e7667;

        let (assign8300_e7673,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p583,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8300_e7673;

        let assign8310_e7675: f64 = if param_given[675] { 1.0 } else { 0.0 };
        let assign8310_e7677: f64 = if assign8310_e7675 == 1.0 { 1.0 } else { 0.0 };
        var_guard108 = assign8310_e7677;

        let (assign8320_e7685,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard108 != 0.0)) {
        (p.p675,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8320_e7685;

        let (assign8330_e7705,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        let assign8330_e7693: f64 = (var_plparam_i * var_ile);
        let assign8330_e7694: f64 = (var_poparam_i + assign8330_e7693);
        let assign8330_e7697: f64 = (var_pwparam_i * var_iwe);
        let assign8330_e7698: f64 = (assign8330_e7694 + assign8330_e7697);
        let assign8330_e7701: f64 = (var_plwparam_i * var_iae);
        let assign8330_e7702: f64 = (assign8330_e7698 + assign8330_e7701);
        let assign8330_e7703: f64 = (var_ile * assign8330_e7702);
        (assign8330_e7703,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign8330_e7705;

        let assign8340_e7744: f64 = if (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        var_guard109 = assign8340_e7744;

        let (assign8350_e7750,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p596,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8350_e7750;

        let assign8360_e7752: f64 = if param_given[676] { 1.0 } else { 0.0 };
        let assign8360_e7754: f64 = if assign8360_e7752 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign8360_e7754;

        let (assign8370_e7762,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p676,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8370_e7762;

        let (assign8380_e7768,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p597,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8380_e7768;

        let assign8390_e7770: f64 = if param_given[677] { 1.0 } else { 0.0 };
        let assign8390_e7772: f64 = if assign8390_e7770 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign8390_e7772;

        let (assign8400_e7780,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p677,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8400_e7780;

        *var_a1_p_slot = var_a1_p;
        *var_a3_p_slot = var_a3_p;
        *var_a4_p_slot = var_a4_p;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp_p_slot = var_alp_p;
        *var_ax_p_slot = var_ax_p;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_guard100_slot = var_guard100;
        *var_guard102_slot = var_guard102;
        *var_guard103_slot = var_guard103;
        *var_guard104_slot = var_guard104;
        *var_guard105_slot = var_guard105;
        *var_guard106_slot = var_guard106;
        *var_guard107_slot = var_guard107;
        *var_guard108_slot = var_guard108;
        *var_guard109_slot = var_guard109;
        *var_guard110_slot = var_guard110;
        *var_guard111_slot = var_guard111;
        *var_guard85_slot = var_guard85;
        *var_guard86_slot = var_guard86;
        *var_guard87_slot = var_guard87;
        *var_guard88_slot = var_guard88;
        *var_guard89_slot = var_guard89;
        *var_guard90_slot = var_guard90;
        *var_guard91_slot = var_guard91;
        *var_guard92_slot = var_guard92;
        *var_guard93_slot = var_guard93;
        *var_guard94_slot = var_guard94;
        *var_guard95_slot = var_guard95;
        *var_guard96_slot = var_guard96;
        *var_guard97_slot = var_guard97;
        *var_guard98_slot = var_guard98;
        *var_guard99_slot = var_guard99;
        *var_iginv_p_slot = var_iginv_p;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_plparam_i_slot = var_plparam_i;
        *var_plwparam_i_slot = var_plwparam_i;
        *var_poparam_i_slot = var_poparam_i;
        *var_pwparam_i_slot = var_pwparam_i;
        *var_sta2_p_slot = var_sta2_p;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stig_p_slot = var_stig_p;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatg_p_slot = var_thesatg_p;
    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard109: f64,
        var_guard36: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_plparam_i: f64,
        var_poparam_i: f64,
        var_we_edge: f64,
        var_alp1ac_p_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_plwparam_i_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_pwparam_i_slot: &mut f64,
        var_rth_p_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
    ) {
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_plwparam_i: f64 = *var_plwparam_i_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_pwparam_i: f64 = *var_pwparam_i_slot;
        let mut var_rth_p: f64 = *var_rth_p_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;

        let (assign8410_e7786,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p598,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8410_e7786;

        let assign8420_e7788: f64 = if param_given[678] { 1.0 } else { 0.0 };
        let assign8420_e7790: f64 = if assign8420_e7788 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign8420_e7790;

        let (assign8430_e7798,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p678,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8430_e7798;

        let (assign8440_e7804,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p599,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8440_e7804;

        let assign8450_e7806: f64 = if param_given[679] { 1.0 } else { 0.0 };
        let assign8450_e7808: f64 = if assign8450_e7806 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign8450_e7808;

        let (assign8460_e7816,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p679,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8460_e7816;

        let (assign8470_e7836,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        let assign8470_e7824: f64 = (var_plparam_i * var_ile);
        let assign8470_e7825: f64 = (var_poparam_i + assign8470_e7824);
        let assign8470_e7828: f64 = (var_pwparam_i * var_iwe);
        let assign8470_e7829: f64 = (assign8470_e7825 + assign8470_e7828);
        let assign8470_e7832: f64 = (var_plwparam_i * var_iae);
        let assign8470_e7833: f64 = (assign8470_e7829 + assign8470_e7832);
        let assign8470_e7834: f64 = assign8470_e7833;
        (assign8470_e7834,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign8470_e7836;

        let assign8480_e7855: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        var_guard114 = assign8480_e7855;

        let (assign8490_e7875,) = {
    if ((var_guard36 != 0.0) && (var_guard114 != 0.0)) {
        let assign8490_e7863: f64 = (p.p681 * var_ile);
        let assign8490_e7864: f64 = (p.p680 + assign8490_e7863);
        let assign8490_e7867: f64 = (p.p682 * var_iwe);
        let assign8490_e7868: f64 = (assign8490_e7864 + assign8490_e7867);
        let assign8490_e7871: f64 = (p.p683 * var_iae);
        let assign8490_e7872: f64 = (assign8490_e7868 + assign8490_e7871);
        let assign8490_e7873: f64 = (var_ile * assign8490_e7872);
        (assign8490_e7873,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign8490_e7875;

        let assign8500_e7894: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        var_guard115 = assign8500_e7894;

        let (assign8510_e7914,) = {
    if ((var_guard36 != 0.0) && (var_guard115 != 0.0)) {
        let assign8510_e7902: f64 = (p.p685 * var_ile);
        let assign8510_e7903: f64 = (p.p684 + assign8510_e7902);
        let assign8510_e7906: f64 = (p.p686 * var_iwe);
        let assign8510_e7907: f64 = (assign8510_e7903 + assign8510_e7906);
        let assign8510_e7910: f64 = (p.p687 * var_iae);
        let assign8510_e7911: f64 = (assign8510_e7907 + assign8510_e7910);
        let assign8510_e7912: f64 = (var_ile * assign8510_e7911);
        (assign8510_e7912,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign8510_e7914;

        let assign8520_e7933: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        var_guard116 = assign8520_e7933;

        let (assign8530_e7953,) = {
    if ((var_guard36 != 0.0) && (var_guard116 != 0.0)) {
        let assign8530_e7941: f64 = (p.p689 * var_ile);
        let assign8530_e7942: f64 = (p.p688 + assign8530_e7941);
        let assign8530_e7945: f64 = (p.p690 * var_iwe);
        let assign8530_e7946: f64 = (assign8530_e7942 + assign8530_e7945);
        let assign8530_e7949: f64 = (p.p691 * var_iae);
        let assign8530_e7950: f64 = (assign8530_e7946 + assign8530_e7949);
        let assign8530_e7951: f64 = (var_iiwecv * assign8530_e7950);
        (assign8530_e7951,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign8530_e7953;

        let assign8540_e7972: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        var_guard117 = assign8540_e7972;

        let (assign8550_e7992,) = {
    if ((var_guard36 != 0.0) && (var_guard117 != 0.0)) {
        let assign8550_e7980: f64 = (p.p693 * var_ile);
        let assign8550_e7981: f64 = (p.p692 + assign8550_e7980);
        let assign8550_e7984: f64 = (p.p694 * var_iwe);
        let assign8550_e7985: f64 = (assign8550_e7981 + assign8550_e7984);
        let assign8550_e7988: f64 = (p.p695 * var_iae);
        let assign8550_e7989: f64 = (assign8550_e7985 + assign8550_e7988);
        let assign8550_e7990: f64 = (var_iiwecv * assign8550_e7989);
        (assign8550_e7990,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign8550_e7992;

        let assign8560_e8011: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };
        var_guard118 = assign8560_e8011;

        let (assign8570_e8031,) = {
    if ((var_guard36 != 0.0) && (var_guard118 != 0.0)) {
        let assign8570_e8019: f64 = (p.p697 * var_ile);
        let assign8570_e8020: f64 = (p.p696 + assign8570_e8019);
        let assign8570_e8023: f64 = (p.p698 * var_iwe);
        let assign8570_e8024: f64 = (assign8570_e8020 + assign8570_e8023);
        let assign8570_e8027: f64 = (p.p699 * var_iae);
        let assign8570_e8028: f64 = (assign8570_e8024 + assign8570_e8027);
        let assign8570_e8029: f64 = (var_iilcv * assign8570_e8028);
        (assign8570_e8029,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign8570_e8031;

        let assign8580_e8050: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };
        var_guard119 = assign8580_e8050;

        let (assign8590_e8070,) = {
    if ((var_guard36 != 0.0) && (var_guard119 != 0.0)) {
        let assign8590_e8058: f64 = (p.p701 * var_ile);
        let assign8590_e8059: f64 = (p.p700 + assign8590_e8058);
        let assign8590_e8062: f64 = (p.p702 * var_iwe);
        let assign8590_e8063: f64 = (assign8590_e8059 + assign8590_e8062);
        let assign8590_e8066: f64 = (p.p703 * var_iae);
        let assign8590_e8067: f64 = (assign8590_e8063 + assign8590_e8066);
        let assign8590_e8068: f64 = (var_iiwecv * assign8590_e8067);
        (assign8590_e8068,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign8590_e8070;

        let assign8600_e8089: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };
        var_guard120 = assign8600_e8089;

        let (assign8610_e8109,) = {
    if ((var_guard36 != 0.0) && (var_guard120 != 0.0)) {
        let assign8610_e8097: f64 = (p.p705 * var_ile);
        let assign8610_e8098: f64 = (p.p704 + assign8610_e8097);
        let assign8610_e8101: f64 = (p.p706 * var_iwe);
        let assign8610_e8102: f64 = (assign8610_e8098 + assign8610_e8101);
        let assign8610_e8105: f64 = (p.p707 * var_iae);
        let assign8610_e8106: f64 = (assign8610_e8102 + assign8610_e8105);
        let assign8610_e8107: f64 = (var_iiwecv * assign8610_e8106);
        (assign8610_e8107,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign8610_e8109;

        let assign8620_e8128: f64 = if (((param_given[708] || param_given[709]) || param_given[710]) || param_given[711]) { 1.0 } else { 0.0 };
        var_guard121 = assign8620_e8128;

        let (assign8630_e8148,) = {
    if ((var_guard36 != 0.0) && (var_guard121 != 0.0)) {
        let assign8630_e8136: f64 = (p.p709 * var_ile);
        let assign8630_e8137: f64 = (p.p708 + assign8630_e8136);
        let assign8630_e8140: f64 = (p.p710 * var_iwe);
        let assign8630_e8141: f64 = (assign8630_e8137 + assign8630_e8140);
        let assign8630_e8144: f64 = (p.p711 * var_iae);
        let assign8630_e8145: f64 = (assign8630_e8141 + assign8630_e8144);
        let assign8630_e8146: f64 = (var_iiwcv * assign8630_e8145);
        (assign8630_e8146,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign8630_e8148;

        let assign8640_e8167: f64 = if (((param_given[712] || param_given[713]) || param_given[714]) || param_given[715]) { 1.0 } else { 0.0 };
        var_guard122 = assign8640_e8167;

        let (assign8650_e8187,) = {
    if ((var_guard36 != 0.0) && (var_guard122 != 0.0)) {
        let assign8650_e8175: f64 = (p.p713 * var_ile);
        let assign8650_e8176: f64 = (p.p712 + assign8650_e8175);
        let assign8650_e8179: f64 = (p.p714 * var_iwe);
        let assign8650_e8180: f64 = (assign8650_e8176 + assign8650_e8179);
        let assign8650_e8183: f64 = (p.p715 * var_iae);
        let assign8650_e8184: f64 = (assign8650_e8180 + assign8650_e8183);
        let assign8650_e8185: f64 = (var_iiwcv * assign8650_e8184);
        (assign8650_e8185,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign8650_e8187;

        s.b[1103] = (((param_given[716] || param_given[717]) || param_given[718]) || param_given[719]);
        s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });

        if ((s.v[1016] != 0.0) && s.b[1103]) {
            s.store_scalar(140, (var_ile2 * (((p.p716 + (p.p717 * var_ile)) + (p.p718 * var_iwe)) + (p.p719 * var_iae))));
        }

        let assign8740_e8362: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        var_guard127 = assign8740_e8362;

        let (assign8750_e8380,) = {
    if ((var_guard36 != 0.0) && (var_guard127 != 0.0)) {
        let assign8750_e8369: f64 = (p.p733 * var_ile);
        let assign8750_e8370: f64 = (p.p732 + assign8750_e8369);
        let assign8750_e8373: f64 = (p.p734 * var_iwe);
        let assign8750_e8374: f64 = (assign8750_e8370 + assign8750_e8373);
        let assign8750_e8377: f64 = (p.p735 * var_iae);
        let assign8750_e8378: f64 = (assign8750_e8374 + assign8750_e8377);
        (assign8750_e8378,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign8750_e8380;

        let assign8760_e8399: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        var_guard128 = assign8760_e8399;

        let (assign8770_e8417,) = {
    if ((var_guard36 != 0.0) && (var_guard128 != 0.0)) {
        let assign8770_e8406: f64 = (p.p737 * var_ile);
        let assign8770_e8407: f64 = (p.p736 + assign8770_e8406);
        let assign8770_e8410: f64 = (p.p738 * var_iwe);
        let assign8770_e8411: f64 = (assign8770_e8407 + assign8770_e8410);
        let assign8770_e8414: f64 = (p.p739 * var_iae);
        let assign8770_e8415: f64 = (assign8770_e8411 + assign8770_e8414);
        (assign8770_e8415,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign8770_e8417;

        let assign8780_e8436: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        var_guard129 = assign8780_e8436;

        let (assign8790_e8454,) = {
    if ((var_guard36 != 0.0) && (var_guard129 != 0.0)) {
        let assign8790_e8443: f64 = (p.p741 * var_ile);
        let assign8790_e8444: f64 = (p.p740 + assign8790_e8443);
        let assign8790_e8447: f64 = (p.p742 * var_iwe);
        let assign8790_e8448: f64 = (assign8790_e8444 + assign8790_e8447);
        let assign8790_e8451: f64 = (p.p743 * var_iae);
        let assign8790_e8452: f64 = (assign8790_e8448 + assign8790_e8451);
        (assign8790_e8452,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign8790_e8454;

        let assign8800_e8473: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        var_guard130 = assign8800_e8473;

        let (assign8810_e8491,) = {
    if ((var_guard36 != 0.0) && (var_guard130 != 0.0)) {
        let assign8810_e8480: f64 = (p.p745 * var_ile);
        let assign8810_e8481: f64 = (p.p744 + assign8810_e8480);
        let assign8810_e8484: f64 = (p.p746 * var_iwe);
        let assign8810_e8485: f64 = (assign8810_e8481 + assign8810_e8484);
        let assign8810_e8488: f64 = (p.p747 * var_iae);
        let assign8810_e8489: f64 = (assign8810_e8485 + assign8810_e8488);
        (assign8810_e8489,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign8810_e8491;

        let assign8820_e8510: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        var_guard131 = assign8820_e8510;

        let (assign8830_e8528,) = {
    if ((var_guard36 != 0.0) && (var_guard131 != 0.0)) {
        let assign8830_e8517: f64 = (p.p749 * var_ile);
        let assign8830_e8518: f64 = (p.p748 + assign8830_e8517);
        let assign8830_e8521: f64 = (p.p750 * var_iwe);
        let assign8830_e8522: f64 = (assign8830_e8518 + assign8830_e8521);
        let assign8830_e8525: f64 = (p.p751 * var_iae);
        let assign8830_e8526: f64 = (assign8830_e8522 + assign8830_e8525);
        (assign8830_e8526,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign8830_e8528;

        let assign8840_e8547: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        var_guard132 = assign8840_e8547;

        let (assign8850_e8569,) = {
    if ((var_guard36 != 0.0) && (var_guard132 != 0.0)) {
        let assign8850_e8553: f64 = (var_we_edge / var_le);
        let assign8850_e8557: f64 = (p.p753 * var_ile);
        let assign8850_e8558: f64 = (p.p752 + assign8850_e8557);
        let assign8850_e8561: f64 = (p.p754 * var_iwe);
        let assign8850_e8562: f64 = (assign8850_e8558 + assign8850_e8561);
        let assign8850_e8565: f64 = (p.p755 * var_iae);
        let assign8850_e8566: f64 = (assign8850_e8562 + assign8850_e8565);
        let assign8850_e8567: f64 = (assign8850_e8553 * assign8850_e8566);
        (assign8850_e8567,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign8850_e8569;

        let assign8860_e8588: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        var_guard133 = assign8860_e8588;

        let (assign8870_e8606,) = {
    if ((var_guard36 != 0.0) && (var_guard133 != 0.0)) {
        let assign8870_e8595: f64 = (p.p757 * var_ile);
        let assign8870_e8596: f64 = (p.p756 + assign8870_e8595);
        let assign8870_e8599: f64 = (p.p758 * var_iwe);
        let assign8870_e8600: f64 = (assign8870_e8596 + assign8870_e8599);
        let assign8870_e8603: f64 = (p.p759 * var_iae);
        let assign8870_e8604: f64 = (assign8870_e8600 + assign8870_e8603);
        (assign8870_e8604,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign8870_e8606;

        let assign8880_e8625: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        var_guard134 = assign8880_e8625;

        let (assign8890_e8645,) = {
    if ((var_guard36 != 0.0) && (var_guard134 != 0.0)) {
        let assign8890_e8633: f64 = (p.p761 * var_ile);
        let assign8890_e8634: f64 = (p.p760 + assign8890_e8633);
        let assign8890_e8637: f64 = (p.p762 * var_iwe);
        let assign8890_e8638: f64 = (assign8890_e8634 + assign8890_e8637);
        let assign8890_e8641: f64 = (p.p763 * var_iae);
        let assign8890_e8642: f64 = (assign8890_e8638 + assign8890_e8641);
        let assign8890_e8643: f64 = (var_ile2 * assign8890_e8642);
        (assign8890_e8643,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign8890_e8645;

        let assign8900_e8664: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        var_guard135 = assign8900_e8664;

        let (assign8910_e8682,) = {
    if ((var_guard36 != 0.0) && (var_guard135 != 0.0)) {
        let assign8910_e8671: f64 = (p.p765 * var_ile);
        let assign8910_e8672: f64 = (p.p764 + assign8910_e8671);
        let assign8910_e8675: f64 = (p.p766 * var_iwe);
        let assign8910_e8676: f64 = (assign8910_e8672 + assign8910_e8675);
        let assign8910_e8679: f64 = (p.p767 * var_iae);
        let assign8910_e8680: f64 = (assign8910_e8676 + assign8910_e8679);
        (assign8910_e8680,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign8910_e8682;

        let assign8920_e8701: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        var_guard136 = assign8920_e8701;

        let (assign8930_e8719,) = {
    if ((var_guard36 != 0.0) && (var_guard136 != 0.0)) {
        let assign8930_e8708: f64 = (p.p769 * var_ile);
        let assign8930_e8709: f64 = (p.p768 + assign8930_e8708);
        let assign8930_e8712: f64 = (p.p770 * var_iwe);
        let assign8930_e8713: f64 = (assign8930_e8709 + assign8930_e8712);
        let assign8930_e8716: f64 = (p.p771 * var_iae);
        let assign8930_e8717: f64 = (assign8930_e8713 + assign8930_e8716);
        (assign8930_e8717,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign8930_e8719;

        let assign8940_e8738: f64 = if (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]) { 1.0 } else { 0.0 };
        var_guard137 = assign8940_e8738;

        let (assign8950_e8758,) = {
    if ((var_guard36 != 0.0) && (var_guard137 != 0.0)) {
        let assign8950_e8746: f64 = (p.p773 * var_ile);
        let assign8950_e8747: f64 = (p.p772 + assign8950_e8746);
        let assign8950_e8750: f64 = (p.p774 * var_iwe);
        let assign8950_e8751: f64 = (assign8950_e8747 + assign8950_e8750);
        let assign8950_e8754: f64 = (p.p775 * var_iae);
        let assign8950_e8755: f64 = (assign8950_e8751 + assign8950_e8754);
        let assign8950_e8756: f64 = (var_ile2 * assign8950_e8755);
        (assign8950_e8756,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign8950_e8758;

        let assign8960_e8777: f64 = if (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]) { 1.0 } else { 0.0 };
        var_guard138 = assign8960_e8777;

        let (assign8970_e8795,) = {
    if ((var_guard36 != 0.0) && (var_guard138 != 0.0)) {
        let assign8970_e8784: f64 = (p.p781 * var_ile);
        let assign8970_e8785: f64 = (p.p780 + assign8970_e8784);
        let assign8970_e8788: f64 = (p.p782 * var_iwe);
        let assign8970_e8789: f64 = (assign8970_e8785 + assign8970_e8788);
        let assign8970_e8792: f64 = (p.p783 * var_iae);
        let assign8970_e8793: f64 = (assign8970_e8789 + assign8970_e8792);
        (assign8970_e8793,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign8970_e8795;

        let assign8980_e8814: f64 = if (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]) { 1.0 } else { 0.0 };
        var_guard139 = assign8980_e8814;

        let (assign8990_e8832,) = {
    if ((var_guard36 != 0.0) && (var_guard139 != 0.0)) {
        let assign8990_e8821: f64 = (p.p777 * var_ile);
        let assign8990_e8822: f64 = (p.p776 + assign8990_e8821);
        let assign8990_e8825: f64 = (p.p778 * var_iwe);
        let assign8990_e8826: f64 = (assign8990_e8822 + assign8990_e8825);
        let assign8990_e8829: f64 = (p.p779 * var_iae);
        let assign8990_e8830: f64 = (assign8990_e8826 + assign8990_e8829);
        (assign8990_e8830,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign8990_e8832;

        let assign9060_e8968: f64 = if (((param_given[796] || param_given[797]) || param_given[798]) || param_given[799]) { 1.0 } else { 0.0 };
        var_guard143 = assign9060_e8968;

        let (assign9070_e8988,) = {
    if ((var_guard36 != 0.0) && (var_guard143 != 0.0)) {
        let assign9070_e8976: f64 = (p.p797 * var_ile);
        let assign9070_e8977: f64 = (p.p796 + assign9070_e8976);
        let assign9070_e8980: f64 = (p.p798 * var_iwe);
        let assign9070_e8981: f64 = (assign9070_e8977 + assign9070_e8980);
        let assign9070_e8984: f64 = (p.p799 * var_iae);
        let assign9070_e8985: f64 = (assign9070_e8981 + assign9070_e8984);
        let assign9070_e8986: f64 = (var_iae * assign9070_e8985);
        (assign9070_e8986,)
    } else {
        (var_rth_p,)
    }
};
        var_rth_p = assign9070_e8988;

        let assign9080_e9007: f64 = if (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]) { 1.0 } else { 0.0 };
        var_guard144 = assign9080_e9007;

        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alpac_p_slot = var_alpac_p;
        *var_axac_p_slot = var_axac_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_guard112_slot = var_guard112;
        *var_guard113_slot = var_guard113;
        *var_guard114_slot = var_guard114;
        *var_guard115_slot = var_guard115;
        *var_guard116_slot = var_guard116;
        *var_guard117_slot = var_guard117;
        *var_guard118_slot = var_guard118;
        *var_guard119_slot = var_guard119;
        *var_guard120_slot = var_guard120;
        *var_guard121_slot = var_guard121;
        *var_guard122_slot = var_guard122;
        *var_guard127_slot = var_guard127;
        *var_guard128_slot = var_guard128;
        *var_guard129_slot = var_guard129;
        *var_guard130_slot = var_guard130;
        *var_guard131_slot = var_guard131;
        *var_guard132_slot = var_guard132;
        *var_guard133_slot = var_guard133;
        *var_guard134_slot = var_guard134;
        *var_guard135_slot = var_guard135;
        *var_guard136_slot = var_guard136;
        *var_guard137_slot = var_guard137;
        *var_guard138_slot = var_guard138;
        *var_guard139_slot = var_guard139;
        *var_guard143_slot = var_guard143;
        *var_guard144_slot = var_guard144;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_plwparam_i_slot = var_plwparam_i;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_pwparam_i_slot = var_pwparam_i;
        *var_rth_p_slot = var_rth_p;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dellps: f64,
        var_delwod: f64,
        var_guard144: f64,
        var_guard36: f64,
        var_iae: f64,
        var_iiae: f64,
        var_ile: f64,
        var_invnf: f64,
        var_iwe: f64,
        var_l_i: f64,
        var_nf_i: f64,
        var_rta: f64,
        var_sa_i: f64,
        var_sb_i: f64,
        var_sc_i: f64,
        var_scb_i: f64,
        var_scc_i: f64,
        var_sd_i: f64,
        var_w_i: f64,
        var_betn_p_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_guard145_slot: &mut f64,
        var_guard146_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_kvsatac_i_slot: &mut f64,
        var_loop__slot: &mut f64,
        var_lx_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_strth_p_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp00_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_wx_slot: &mut f64,
    ) {
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_guard145: f64 = *var_guard145_slot;
        let mut var_guard146: f64 = *var_guard146_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_kvsatac_i: f64 = *var_kvsatac_i_slot;
        let mut var_loop_: f64 = *var_loop__slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_strth_p: f64 = *var_strth_p_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp00: f64 = *var_temp00_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_wx: f64 = *var_wx_slot;

        let (assign9090_e9027,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9090_e9015: f64 = (p.p801 * var_ile);
        let assign9090_e9016: f64 = (p.p800 + assign9090_e9015);
        let assign9090_e9019: f64 = (p.p802 * var_iwe);
        let assign9090_e9020: f64 = (assign9090_e9016 + assign9090_e9019);
        let assign9090_e9023: f64 = (p.p803 * var_iae);
        let assign9090_e9024: f64 = (assign9090_e9020 + assign9090_e9023);
        let assign9090_e9025: f64 = (var_iiae * assign9090_e9024);
        (assign9090_e9025,)
    } else {
        (var_cth_p,)
    }
};
        var_cth_p = assign9090_e9027;

        let assign9100_e9046: f64 = if (((param_given[804] || param_given[805]) || param_given[806]) || param_given[807]) { 1.0 } else { 0.0 };
        var_guard145 = assign9100_e9046;

        let (assign9110_e9064,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9110_e9053: f64 = (p.p805 * var_ile);
        let assign9110_e9054: f64 = (p.p804 + assign9110_e9053);
        let assign9110_e9057: f64 = (p.p806 * var_iwe);
        let assign9110_e9058: f64 = (assign9110_e9054 + assign9110_e9057);
        let assign9110_e9061: f64 = (p.p807 * var_iae);
        let assign9110_e9062: f64 = (assign9110_e9058 + assign9110_e9061);
        (assign9110_e9062,)
    } else {
        (var_strth_p,)
    }
};
        var_strth_p = assign9110_e9064;

        let (assign9120_e9068,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_tmpa,)
    }
};
        var_tmpa = assign9120_e9068;

        let (assign9130_e9072,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign9130_e9072;

        let (assign9140_e9076,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_loop_,)
    }
};
        var_loop_ = assign9140_e9076;

        let (assign9150_e9080,) = {
    if (var_guard36 != 0.0) {
        (p.p812,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9150_e9080;

        let assign9160_e9082: f64 = if param_given[813] { 1.0 } else { 0.0 };
        let assign9160_e9084: f64 = if assign9160_e9082 == 1.0 { 1.0 } else { 0.0 };
        var_guard146 = assign9160_e9084;

        let (assign9170_e9090,) = {
    if ((var_guard36 != 0.0) && (var_guard146 != 0.0)) {
        (p.p813,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9170_e9090;

        let assign9180_e9109: f64 = if (((var_sa_i > 0.0) && (var_sb_i > 0.0)) && ((var_nf_i == 1.0) || ((var_nf_i > 1.0) && (var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        var_guard147 = assign9180_e9109;

        let mut assign9190_loop_guard: usize = 0;
        while {
            let assign9190_cond_e9116: f64 = (var_nf_i - 0.5);
            let assign9190_cond_e9118: f64 = if (((var_guard36 != 0.0) && (var_guard147 != 0.0)) && (var_loop_ < assign9190_cond_e9116)) { 1.0 } else { 0.0 };
            assign9190_cond_e9118 != 0.0
        } {
            assign9190_loop_guard += 1;
            assert!(assign9190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9190_body0_e9138,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9190_body0_e9127: f64 = (0.5 * var_l_i);
        let assign9190_body0_e9128: f64 = (var_sa_i + assign9190_body0_e9127);
        let assign9190_body0_e9132: f64 = (var_sd_i + var_l_i);
        let assign9190_body0_e9133: f64 = (var_loop_ * assign9190_body0_e9132);
        let assign9190_body0_e9134: f64 = (assign9190_body0_e9128 + assign9190_body0_e9133);
        let assign9190_body0_e9135: f64 = (1.0 / assign9190_body0_e9134);
        let assign9190_body0_e9136: f64 = (var_tmpa + assign9190_body0_e9135);
        (assign9190_body0_e9136,)
    } else {
        (var_tmpa,)
    }
};
            var_tmpa = assign9190_body0_e9138;
            let (assign9190_body1_e9158,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9190_body1_e9147: f64 = (0.5 * var_l_i);
        let assign9190_body1_e9148: f64 = (var_sb_i + assign9190_body1_e9147);
        let assign9190_body1_e9152: f64 = (var_sd_i + var_l_i);
        let assign9190_body1_e9153: f64 = (var_loop_ * assign9190_body1_e9152);
        let assign9190_body1_e9154: f64 = (assign9190_body1_e9148 + assign9190_body1_e9153);
        let assign9190_body1_e9155: f64 = (1.0 / assign9190_body1_e9154);
        let assign9190_body1_e9156: f64 = (var_tmpb + assign9190_body1_e9155);
        (assign9190_body1_e9156,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign9190_body1_e9158;
            let (assign9190_body2_e9166,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9190_body2_e9164: f64 = (var_loop_ + 1.0);
        (assign9190_body2_e9164,)
    } else {
        (var_loop_,)
    }
};
            var_loop_ = assign9190_body2_e9166;
        }

        let (assign9200_e9174,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9200_e9172: f64 = (var_tmpa * var_invnf);
        (assign9200_e9172,)
    } else {
        (var_invsa,)
    }
};
        var_invsa = assign9200_e9174;

        let (assign9210_e9182,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9210_e9180: f64 = (var_tmpb * var_invnf);
        (assign9210_e9180,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign9210_e9182;

        let (assign9220_e9194,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9220_e9190: f64 = (0.5 * var_l_i);
        let assign9220_e9191: f64 = (p.p808 + assign9220_e9190);
        let assign9220_e9192: f64 = (1.0 / assign9220_e9191);
        (assign9220_e9192,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign9220_e9194;

        let (assign9230_e9206,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9230_e9202: f64 = (0.5 * var_l_i);
        let assign9230_e9203: f64 = (p.p809 + assign9230_e9202);
        let assign9230_e9204: f64 = (1.0 / assign9230_e9203);
        (assign9230_e9204,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign9230_e9206;

        let (assign9240_e9221,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9240_e9212: f64 = (var_l_i + var_dellps);
        let (assign9240_e9219,) = {
            if (assign9240_e9212 > 1e-9) {
                let assign9240_e9217: f64 = (var_l_i + var_dellps);
                (assign9240_e9217,)
            } else {
                (1e-9,)
            }
        };
        (assign9240_e9219,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign9240_e9221;

        let (assign9250_e9240,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9250_e9227: f64 = (var_w_i + var_delwod);
        let assign9250_e9229: f64 = (assign9250_e9227 + p.p810);
        let (assign9250_e9238,) = {
            if (assign9250_e9229 > 1e-9) {
                let assign9250_e9234: f64 = (var_w_i + var_delwod);
                let assign9250_e9236: f64 = (assign9250_e9234 + p.p810);
                (assign9250_e9236,)
            } else {
                (1e-9,)
            }
        };
        (assign9250_e9238,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign9250_e9240;

        let (assign9260_e9250,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9260_e9247: f64 = (var_lx).powf(p.p818);
        let assign9260_e9248: f64 = (1.0 / assign9260_e9247);
        (assign9260_e9248,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9260_e9250;

        let (assign9270_e9260,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9270_e9257: f64 = (var_wx).powf(p.p819);
        let assign9270_e9258: f64 = (1.0 / assign9270_e9257);
        (assign9270_e9258,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9270_e9260;

        let (assign9280_e9288,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9280_e9267: f64 = (p.p815 * var_templ);
        let assign9280_e9268: f64 = (1.0 + assign9280_e9267);
        let assign9280_e9271: f64 = (p.p816 * var_tempw);
        let assign9280_e9272: f64 = (assign9280_e9268 + assign9280_e9271);
        let assign9280_e9275: f64 = (p.p817 * var_templ);
        let assign9280_e9277: f64 = (assign9280_e9275 * var_tempw);
        let assign9280_e9278: f64 = (assign9280_e9272 + assign9280_e9277);
        let assign9280_e9283: f64 = (var_rta - 1.0);
        let assign9280_e9284: f64 = (p.p814 * assign9280_e9283);
        let assign9280_e9285: f64 = (1.0 + assign9280_e9284);
        let assign9280_e9286: f64 = (assign9280_e9278 * assign9280_e9285);
        (assign9280_e9286,)
    } else {
        (var_kstressu0,)
    }
};
        var_kstressu0 = assign9280_e9288;

        let (assign9290_e9300,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9290_e9295: f64 = (var_invsa + var_invsb);
        let assign9290_e9296: f64 = (p.p811 * assign9290_e9295);
        let assign9290_e9298: f64 = (assign9290_e9296 / var_kstressu0);
        (assign9290_e9298,)
    } else {
        (var_rhobeta,)
    }
};
        var_rhobeta = assign9290_e9300;

        let (assign9300_e9312,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9300_e9307: f64 = (var_invsaref + var_invsbref);
        let assign9300_e9308: f64 = (p.p811 * assign9300_e9307);
        let assign9300_e9310: f64 = (assign9300_e9308 / var_kstressu0);
        (assign9300_e9310,)
    } else {
        (var_rhobetaref,)
    }
};
        var_rhobetaref = assign9300_e9312;

        let (assign9310_e9322,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9310_e9319: f64 = (var_lx).powf(p.p824);
        let assign9310_e9320: f64 = (1.0 / assign9310_e9319);
        (assign9310_e9320,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9310_e9322;

        let (assign9320_e9332,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9320_e9329: f64 = (var_wx).powf(p.p825);
        let assign9320_e9330: f64 = (1.0 / assign9320_e9329);
        (assign9320_e9330,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9320_e9332;

        let (assign9330_e9352,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9330_e9339: f64 = (p.p821 * var_templ);
        let assign9330_e9340: f64 = (1.0 + assign9330_e9339);
        let assign9330_e9343: f64 = (p.p822 * var_tempw);
        let assign9330_e9344: f64 = (assign9330_e9340 + assign9330_e9343);
        let assign9330_e9347: f64 = (p.p823 * var_templ);
        let assign9330_e9349: f64 = (assign9330_e9347 * var_tempw);
        let assign9330_e9350: f64 = (assign9330_e9344 + assign9330_e9349);
        (assign9330_e9350,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign9330_e9352;

        let (assign9340_e9364,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9340_e9358: f64 = (var_invsa + var_invsb);
        let assign9340_e9360: f64 = (assign9340_e9358 - var_invsaref);
        let assign9340_e9362: f64 = (assign9340_e9360 - var_invsbref);
        (assign9340_e9362,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9340_e9364;

        let (assign9350_e9376,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9350_e9370: f64 = (1.0 + var_rhobeta);
        let assign9350_e9373: f64 = (1.0 + var_rhobetaref);
        let assign9350_e9374: f64 = (assign9350_e9370 / assign9350_e9373);
        (assign9350_e9374,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9350_e9376;

        let (assign9360_e9384,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9360_e9382: f64 = (var_betn_p * var_temp00);
        (assign9360_e9382,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9360_e9384;

        let (assign9370_e9404,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9370_e9390: f64 = (var_thesat_p * var_temp00);
        let assign9370_e9394: f64 = (p.p812 * var_rhobetaref);
        let assign9370_e9395: f64 = (1.0 + assign9370_e9394);
        let assign9370_e9396: f64 = (assign9370_e9390 * assign9370_e9395);
        let assign9370_e9400: f64 = (p.p812 * var_rhobeta);
        let assign9370_e9401: f64 = (1.0 + assign9370_e9400);
        let assign9370_e9402: f64 = (assign9370_e9396 / assign9370_e9401);
        (assign9370_e9402,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign9370_e9404;

        let (assign9380_e9424,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9380_e9410: f64 = (var_thesatac_p * var_temp00);
        let assign9380_e9414: f64 = (var_kvsatac_i * var_rhobetaref);
        let assign9380_e9415: f64 = (1.0 + assign9380_e9414);
        let assign9380_e9416: f64 = (assign9380_e9410 * assign9380_e9415);
        let assign9380_e9420: f64 = (var_kvsatac_i * var_rhobeta);
        let assign9380_e9421: f64 = (1.0 + assign9380_e9420);
        let assign9380_e9422: f64 = (assign9380_e9416 / assign9380_e9421);
        (assign9380_e9422,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign9380_e9424;

        let (assign9390_e9432,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9390_e9430: f64 = (var_betnedge_p * var_temp00);
        (assign9390_e9430,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9390_e9432;

        let (assign9400_e9442,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9400_e9438: f64 = (p.p820 * var_temp0);
        let assign9400_e9440: f64 = (assign9400_e9438 / var_kstressvth0);
        (assign9400_e9440,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9400_e9442;

        let (assign9410_e9450,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9410_e9448: f64 = (var_vfb_p + var_temp00);
        (assign9410_e9448,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9410_e9450;

        let (assign9420_e9458,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9420_e9456: f64 = (var_vfbedge_p + var_temp00);
        (assign9420_e9456,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9420_e9458;

        let (assign9430_e9470,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9430_e9464: f64 = (p.p826 * var_temp0);
        let assign9430_e9467: f64 = (var_kstressvth0).powf(p.p827);
        let assign9430_e9468: f64 = (assign9430_e9464 / assign9430_e9467);
        (assign9430_e9468,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9430_e9470;

        let (assign9440_e9478,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9440_e9476: f64 = (var_cf_p + var_temp00);
        (assign9440_e9476,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign9440_e9478;

        let (assign9450_e9486,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9450_e9484: f64 = (var_cfedge_p + var_temp00);
        (assign9450_e9484,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign9450_e9486;

        let assign9460_e9501: f64 = if ((((var_sca_i > 0.0) || (var_scb_i > 0.0)) || (var_scc_i > 0.0)) || (var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard148 = assign9460_e9501;

        let assign9470_e9512: f64 = if (((var_sca_i == 0.0) && (var_scb_i == 0.0)) && (var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard149 = assign9470_e9512;

        let (assign9480_e9522,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9480_e9520: f64 = (var_sc_i + var_w_i);
        (assign9480_e9520,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9480_e9522;

        let (assign9490_e9532,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9490_e9530: f64 = (1.0 / p.p828);
        (assign9490_e9530,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9490_e9532;

        let (assign9500_e9546,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9500_e9540: f64 = (p.p828 * p.p828);
        let assign9500_e9543: f64 = (var_sc_i * var_temp0);
        let assign9500_e9544: f64 = (assign9500_e9540 / assign9500_e9543);
        (assign9500_e9544,)
    } else {
        (var_sca_i,)
    }
};
        var_sca_i = assign9500_e9546;

        *var_betn_p_slot = var_betn_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cf_p_slot = var_cf_p;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cth_p_slot = var_cth_p;
        *var_guard145_slot = var_guard145;
        *var_guard146_slot = var_guard146;
        *var_guard147_slot = var_guard147;
        *var_guard148_slot = var_guard148;
        *var_guard149_slot = var_guard149;
        *var_invsa_slot = var_invsa;
        *var_invsaref_slot = var_invsaref;
        *var_invsb_slot = var_invsb;
        *var_invsbref_slot = var_invsbref;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_kvsatac_i_slot = var_kvsatac_i;
        *var_loop__slot = var_loop_;
        *var_lx_slot = var_lx;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_sca_i_slot = var_sca_i;
        *var_strth_p_slot = var_strth_p;
        *var_temp0_slot = var_temp0;
        *var_temp00_slot = var_temp00;
        *var_templ_slot = var_templ;
        *var_tempw_slot = var_tempw;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_tmpa_slot = var_tmpa;
        *var_tmpb_slot = var_tmpb;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_wx_slot = var_wx;
    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        var_alp1_p: f64,
        var_alp2_p: f64,
        var_alp_p: f64,
        var_ax_p: f64,
        var_cf_p: f64,
        var_cfb_p: f64,
        var_cfd_p: f64,
        var_cs_p: f64,
        var_ct_p: f64,
        var_ctb_p: f64,
        var_ctg_p: f64,
        var_dphib_p: f64,
        var_dvsbnud_p: f64,
        var_epsrox_p: f64,
        var_feta_p: f64,
        var_gfacnud_p: f64,
        var_guard148: f64,
        var_guard149: f64,
        var_guard36: f64,
        var_kuowe: f64,
        var_kvthowe: f64,
        var_mue_p: f64,
        var_neff_p: f64,
        var_nov_p: f64,
        var_novd_p: f64,
        var_np_p: f64,
        var_psce_p: f64,
        var_psceb_p: f64,
        var_psced_p: f64,
        var_rs_p: f64,
        var_rsb_p: f64,
        var_rsg_p: f64,
        var_sc_i: f64,
        var_sca_i: f64,
        var_st2vfb_p: f64,
        var_stbet_p: f64,
        var_stcs_p: f64,
        var_stct_p: f64,
        var_stmue_p: f64,
        var_strs_p: f64,
        var_stthecs_p: f64,
        var_stthemu_p: f64,
        var_stthesat_p: f64,
        var_stvfb_p: f64,
        var_stxcor_p: f64,
        var_temp00: f64,
        var_thecs_p: f64,
        var_themu_p: f64,
        var_thesat_p: f64,
        var_thesatb_p: f64,
        var_thesatg_p: f64,
        var_thesatt_p: f64,
        var_tox_p: f64,
        var_toxov_p: f64,
        var_toxovd_p: f64,
        var_vp_p: f64,
        var_vsbnud_p: f64,
        var_w_i: f64,
        var_xcor_p: f64,
        var_alp1_i_slot: &mut f64,
        var_alp2_i_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_ax_i_slot: &mut f64,
        var_betn_i_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_cf_i_slot: &mut f64,
        var_cfb_i_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cs_i_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_ctb_i_slot: &mut f64,
        var_ctg_i_slot: &mut f64,
        var_dphib_i_slot: &mut f64,
        var_dvsbnud_i_slot: &mut f64,
        var_epsrox_i_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_gfacnud_i_slot: &mut f64,
        var_mue_i_slot: &mut f64,
        var_neff_i_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_psce_i_slot: &mut f64,
        var_psceb_i_slot: &mut f64,
        var_psced_i_slot: &mut f64,
        var_rs_i_slot: &mut f64,
        var_rsb_i_slot: &mut f64,
        var_rsg_i_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_st2vfb_i_slot: &mut f64,
        var_stbet_i_slot: &mut f64,
        var_stcs_i_slot: &mut f64,
        var_stct_i_slot: &mut f64,
        var_stmue_i_slot: &mut f64,
        var_strs_i_slot: &mut f64,
        var_stthecs_i_slot: &mut f64,
        var_stthemu_i_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_thecs_i_slot: &mut f64,
        var_themu_i_slot: &mut f64,
        var_thesat_i_slot: &mut f64,
        var_thesatb_i_slot: &mut f64,
        var_thesatg_i_slot: &mut f64,
        var_thesatt_i_slot: &mut f64,
        var_tox_i_slot: &mut f64,
        var_toxov_i_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_vfb_i_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vsbnud_i_slot: &mut f64,
        var_xcor_i_slot: &mut f64,
    ) {
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp2_i: f64 = *var_alp2_i_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_betn_i: f64 = *var_betn_i_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cf_i: f64 = *var_cf_i_slot;
        let mut var_cfb_i: f64 = *var_cfb_i_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cs_i: f64 = *var_cs_i_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_ctb_i: f64 = *var_ctb_i_slot;
        let mut var_ctg_i: f64 = *var_ctg_i_slot;
        let mut var_dphib_i: f64 = *var_dphib_i_slot;
        let mut var_dvsbnud_i: f64 = *var_dvsbnud_i_slot;
        let mut var_epsrox_i: f64 = *var_epsrox_i_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_gfacnud_i: f64 = *var_gfacnud_i_slot;
        let mut var_mue_i: f64 = *var_mue_i_slot;
        let mut var_neff_i: f64 = *var_neff_i_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_psce_i: f64 = *var_psce_i_slot;
        let mut var_psceb_i: f64 = *var_psceb_i_slot;
        let mut var_psced_i: f64 = *var_psced_i_slot;
        let mut var_rs_i: f64 = *var_rs_i_slot;
        let mut var_rsb_i: f64 = *var_rsb_i_slot;
        let mut var_rsg_i: f64 = *var_rsg_i_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_st2vfb_i: f64 = *var_st2vfb_i_slot;
        let mut var_stbet_i: f64 = *var_stbet_i_slot;
        let mut var_stcs_i: f64 = *var_stcs_i_slot;
        let mut var_stct_i: f64 = *var_stct_i_slot;
        let mut var_stmue_i: f64 = *var_stmue_i_slot;
        let mut var_strs_i: f64 = *var_strs_i_slot;
        let mut var_stthecs_i: f64 = *var_stthecs_i_slot;
        let mut var_stthemu_i: f64 = *var_stthemu_i_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_thecs_i: f64 = *var_thecs_i_slot;
        let mut var_themu_i: f64 = *var_themu_i_slot;
        let mut var_thesat_i: f64 = *var_thesat_i_slot;
        let mut var_thesatb_i: f64 = *var_thesatb_i_slot;
        let mut var_thesatg_i: f64 = *var_thesatg_i_slot;
        let mut var_thesatt_i: f64 = *var_thesatt_i_slot;
        let mut var_tox_i: f64 = *var_tox_i_slot;
        let mut var_toxov_i: f64 = *var_toxov_i_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_vfb_i: f64 = *var_vfb_i_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vsbnud_i: f64 = *var_vsbnud_i_slot;
        let mut var_xcor_i: f64 = *var_xcor_i_slot;

        let (assign9510_e9586,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9510_e9554: f64 = (0.1 * var_sc_i);
        let assign9510_e9557: f64 = (0.01 * p.p828);
        let assign9510_e9558: f64 = (assign9510_e9554 + assign9510_e9557);
        let assign9510_e9560: f64 = (-10.0);
        let assign9510_e9562: f64 = (assign9510_e9560 * var_sc_i);
        let assign9510_e9564: f64 = (assign9510_e9562 * var_temp00);
        let assign9510_e9565: f64 = (assign9510_e9564).exp();
        let assign9510_e9566: f64 = (assign9510_e9558 * assign9510_e9565);
        let assign9510_e9569: f64 = (0.1 * var_temp0);
        let assign9510_e9572: f64 = (0.01 * p.p828);
        let assign9510_e9573: f64 = (assign9510_e9569 + assign9510_e9572);
        let assign9510_e9575: f64 = (-10.0);
        let assign9510_e9577: f64 = (assign9510_e9575 * var_temp0);
        let assign9510_e9579: f64 = (assign9510_e9577 * var_temp00);
        let assign9510_e9580: f64 = (assign9510_e9579).exp();
        let assign9510_e9581: f64 = (assign9510_e9573 * assign9510_e9580);
        let assign9510_e9582: f64 = (assign9510_e9566 - assign9510_e9581);
        let assign9510_e9584: f64 = (assign9510_e9582 / var_w_i);
        (assign9510_e9584,)
    } else {
        (var_scb_i,)
    }
};
        var_scb_i = assign9510_e9586;

        let (assign9520_e9626,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9520_e9594: f64 = (0.05 * var_sc_i);
        let assign9520_e9597: f64 = (0.0025 * p.p828);
        let assign9520_e9598: f64 = (assign9520_e9594 + assign9520_e9597);
        let assign9520_e9600: f64 = (-20.0);
        let assign9520_e9602: f64 = (assign9520_e9600 * var_sc_i);
        let assign9520_e9604: f64 = (assign9520_e9602 * var_temp00);
        let assign9520_e9605: f64 = (assign9520_e9604).exp();
        let assign9520_e9606: f64 = (assign9520_e9598 * assign9520_e9605);
        let assign9520_e9609: f64 = (0.05 * var_temp0);
        let assign9520_e9612: f64 = (0.0025 * p.p828);
        let assign9520_e9613: f64 = (assign9520_e9609 + assign9520_e9612);
        let assign9520_e9615: f64 = (-20.0);
        let assign9520_e9617: f64 = (assign9520_e9615 * var_temp0);
        let assign9520_e9619: f64 = (assign9520_e9617 * var_temp00);
        let assign9520_e9620: f64 = (assign9520_e9619).exp();
        let assign9520_e9621: f64 = (assign9520_e9613 * assign9520_e9620);
        let assign9520_e9622: f64 = (assign9520_e9606 - assign9520_e9621);
        let assign9520_e9624: f64 = (assign9520_e9622 / var_w_i);
        (assign9520_e9624,)
    } else {
        (var_scc_i,)
    }
};
        var_scc_i = assign9520_e9626;

        let (assign9530_e9640,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9530_e9633: f64 = (p.p829 * var_scb_i);
        let assign9530_e9634: f64 = (var_sca_i + assign9530_e9633);
        let assign9530_e9637: f64 = (p.p830 * var_scc_i);
        let assign9530_e9638: f64 = (assign9530_e9634 + assign9530_e9637);
        (assign9530_e9638,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9530_e9640;

        let (assign9540_e9650,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9540_e9647: f64 = (var_kvthowe * var_temp0);
        let assign9540_e9648: f64 = (var_vfb_p + assign9540_e9647);
        (assign9540_e9648,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9540_e9650;

        let (assign9550_e9662,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9550_e9658: f64 = (var_kuowe * var_temp0);
        let assign9550_e9659: f64 = (1.0 + assign9550_e9658);
        let assign9550_e9660: f64 = (var_betn_p * assign9550_e9659);
        (assign9550_e9660,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9550_e9662;

        let (assign9560_e9672,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9560_e9669: f64 = (var_kvthowe * var_temp0);
        let assign9560_e9670: f64 = (var_vfbedge_p + assign9560_e9669);
        (assign9560_e9670,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9560_e9672;

        let (assign9570_e9684,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9570_e9680: f64 = (var_kuowe * var_temp0);
        let assign9570_e9681: f64 = (1.0 + assign9570_e9680);
        let assign9570_e9682: f64 = (var_betnedge_p * assign9570_e9681);
        (assign9570_e9682,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9570_e9684;

        var_vfb_i = var_vfb_p;

        var_stvfb_i = var_stvfb_p;

        var_st2vfb_i = var_st2vfb_p;

        var_tox_i = var_tox_p;

        var_epsrox_i = var_epsrox_p;

        let (assign9630_e9700,) = {
    if (var_neff_p > 1e20) {
        let (assign9630_e9698,) = {
            if (var_neff_p < 1e26) {
                (var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9630_e9698,)
    } else {
        (1e20,)
    }
};
        var_neff_i = assign9630_e9700;

        let (assign9640_e9706,) = {
    if (var_gfacnud_p > 0.01) {
        (var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        var_gfacnud_i = assign9640_e9706;

        let (assign9650_e9712,) = {
    if (var_vsbnud_p > 0.0) {
        (var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        var_vsbnud_i = assign9650_e9712;

        var_dvsbnud_i = var_dvsbnud_p;

        var_dphib_i = var_dphib_p;

        let (assign9680_e9720,) = {
    if (var_np_p > 0.0) {
        (var_np_p,)
    } else {
        (0.0,)
    }
};
        var_np_i = assign9680_e9720;

        var_toxov_i = var_toxov_p;

        var_toxovd_i = var_toxovd_p;

        let (assign9710_e9733,) = {
    if (var_nov_p > 1e23) {
        let (assign9710_e9731,) = {
            if (var_nov_p < 1e27) {
                (var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9710_e9731,)
    } else {
        (1e23,)
    }
};
        var_nov_i = assign9710_e9733;

        let (assign9720_e9744,) = {
    if (var_novd_p > 1e23) {
        let (assign9720_e9742,) = {
            if (var_novd_p < 1e27) {
                (var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9720_e9742,)
    } else {
        (1e23,)
    }
};
        var_novd_i = assign9720_e9744;

        let (assign9730_e9750,) = {
    if (var_ct_p > 0.0) {
        (var_ct_p,)
    } else {
        (0.0,)
    }
};
        var_ct_i = assign9730_e9750;

        let (assign9740_e9761,) = {
    if (var_ctb_p > 0.0) {
        let (assign9740_e9759,) = {
            if (var_ctb_p < 0.5) {
                (var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9740_e9759,)
    } else {
        (0.0,)
    }
};
        var_ctb_i = assign9740_e9761;

        let (assign9750_e9772,) = {
    if (var_ctg_p > 0.0) {
        let (assign9750_e9770,) = {
            if (var_ctg_p < 1.0) {
                (var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9750_e9770,)
    } else {
        (0.0,)
    }
};
        var_ctg_i = assign9750_e9772;

        var_stct_i = var_stct_p;

        let (assign9770_e9779,) = {
    if (var_cf_p > 0.0) {
        (var_cf_p,)
    } else {
        (0.0,)
    }
};
        var_cf_i = assign9770_e9779;

        let (assign9780_e9790,) = {
    if (var_cfb_p > 0.0) {
        let (assign9780_e9788,) = {
            if (var_cfb_p < 1.0) {
                (var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9780_e9788,)
    } else {
        (0.0,)
    }
};
        var_cfb_i = assign9780_e9790;

        let (assign9790_e9796,) = {
    if (var_cfd_p > 0.0) {
        (var_cfd_p,)
    } else {
        (0.0,)
    }
};
        var_cfd_i = assign9790_e9796;

        let (assign9800_e9802,) = {
    if (var_psce_p > 0.0) {
        (var_psce_p,)
    } else {
        (0.0,)
    }
};
        var_psce_i = assign9800_e9802;

        let (assign9810_e9813,) = {
    if (var_psceb_p > 0.0) {
        let (assign9810_e9811,) = {
            if (var_psceb_p < 1.0) {
                (var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9810_e9811,)
    } else {
        (0.0,)
    }
};
        var_psceb_i = assign9810_e9813;

        let (assign9820_e9819,) = {
    if (var_psced_p > 0.0) {
        (var_psced_p,)
    } else {
        (0.0,)
    }
};
        var_psced_i = assign9820_e9819;

        let (assign9830_e9825,) = {
    if (var_betn_p > 0.0) {
        (var_betn_p,)
    } else {
        (0.0,)
    }
};
        var_betn_i = assign9830_e9825;

        var_stbet_i = var_stbet_p;

        let (assign9850_e9832,) = {
    if (var_mue_p > 0.0) {
        (var_mue_p,)
    } else {
        (0.0,)
    }
};
        var_mue_i = assign9850_e9832;

        var_stmue_i = var_stmue_p;

        let (assign9870_e9839,) = {
    if (var_themu_p > 0.0) {
        (var_themu_p,)
    } else {
        (0.0,)
    }
};
        var_themu_i = assign9870_e9839;

        var_stthemu_i = var_stthemu_p;

        let (assign9890_e9846,) = {
    if (var_cs_p > 0.0) {
        (var_cs_p,)
    } else {
        (0.0,)
    }
};
        var_cs_i = assign9890_e9846;

        var_stcs_i = var_stcs_p;

        let (assign9910_e9853,) = {
    if (var_thecs_p > 0.0) {
        (var_thecs_p,)
    } else {
        (0.0,)
    }
};
        var_thecs_i = assign9910_e9853;

        var_stthecs_i = var_stthecs_p;

        let (assign9930_e9860,) = {
    if (var_xcor_p > 0.0) {
        (var_xcor_p,)
    } else {
        (0.0,)
    }
};
        var_xcor_i = assign9930_e9860;

        var_stxcor_i = var_stxcor_p;

        var_feta_i = var_feta_p;

        let (assign9960_e9868,) = {
    if (var_rs_p > 0.0) {
        (var_rs_p,)
    } else {
        (0.0,)
    }
};
        var_rs_i = assign9960_e9868;

        var_strs_i = var_strs_p;

        let assign9980_e9872: f64 = (-0.5);
        let (assign9980_e9882,) = {
    if (var_rsb_p > assign9980_e9872) {
        let (assign9980_e9879,) = {
            if (var_rsb_p < 1.0) {
                (var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9980_e9879,)
    } else {
        let assign9980_e9881: f64 = (-0.5);
        (assign9980_e9881,)
    }
};
        var_rsb_i = assign9980_e9882;

        let assign9990_e9885: f64 = (-0.5);
        let (assign9990_e9890,) = {
    if (var_rsg_p > assign9990_e9885) {
        (var_rsg_p,)
    } else {
        let assign9990_e9889: f64 = (-0.5);
        (assign9990_e9889,)
    }
};
        var_rsg_i = assign9990_e9890;

        let (assign10000_e9896,) = {
    if (var_thesat_p > 0.0) {
        (var_thesat_p,)
    } else {
        (0.0,)
    }
};
        var_thesat_i = assign10000_e9896;

        var_stthesat_i = var_stthesat_p;

        let assign10020_e9900: f64 = (-0.5);
        let (assign10020_e9910,) = {
    if (var_thesatb_p > assign10020_e9900) {
        let (assign10020_e9907,) = {
            if (var_thesatb_p < 1.0) {
                (var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10020_e9907,)
    } else {
        let assign10020_e9909: f64 = (-0.5);
        (assign10020_e9909,)
    }
};
        var_thesatb_i = assign10020_e9910;

        let assign10030_e9913: f64 = (-0.5);
        let (assign10030_e9918,) = {
    if (var_thesatg_p > assign10030_e9913) {
        (var_thesatg_p,)
    } else {
        let assign10030_e9917: f64 = (-0.5);
        (assign10030_e9917,)
    }
};
        var_thesatg_i = assign10030_e9918;

        let (assign10040_e9924,) = {
    if (var_thesatt_p > 0.01) {
        (var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        var_thesatt_i = assign10040_e9924;

        let (assign10050_e9930,) = {
    if (var_ax_p > 2.0) {
        (var_ax_p,)
    } else {
        (2.0,)
    }
};
        var_ax_i = assign10050_e9930;

        let (assign10060_e9936,) = {
    if (var_alp_p > 0.0) {
        (var_alp_p,)
    } else {
        (0.0,)
    }
};
        var_alp_i = assign10060_e9936;

        let (assign10070_e9942,) = {
    if (var_alp1_p > 0.0) {
        (var_alp1_p,)
    } else {
        (0.0,)
    }
};
        var_alp1_i = assign10070_e9942;

        let (assign10080_e9948,) = {
    if (var_alp2_p > 0.0) {
        (var_alp2_p,)
    } else {
        (0.0,)
    }
};
        var_alp2_i = assign10080_e9948;

        var_vp_i = var_vp_p;

        *var_alp1_i_slot = var_alp1_i;
        *var_alp2_i_slot = var_alp2_i;
        *var_alp_i_slot = var_alp_i;
        *var_ax_i_slot = var_ax_i;
        *var_betn_i_slot = var_betn_i;
        *var_betn_p_slot = var_betn_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cf_i_slot = var_cf_i;
        *var_cfb_i_slot = var_cfb_i;
        *var_cfd_i_slot = var_cfd_i;
        *var_cs_i_slot = var_cs_i;
        *var_ct_i_slot = var_ct_i;
        *var_ctb_i_slot = var_ctb_i;
        *var_ctg_i_slot = var_ctg_i;
        *var_dphib_i_slot = var_dphib_i;
        *var_dvsbnud_i_slot = var_dvsbnud_i;
        *var_epsrox_i_slot = var_epsrox_i;
        *var_feta_i_slot = var_feta_i;
        *var_gfacnud_i_slot = var_gfacnud_i;
        *var_mue_i_slot = var_mue_i;
        *var_neff_i_slot = var_neff_i;
        *var_nov_i_slot = var_nov_i;
        *var_novd_i_slot = var_novd_i;
        *var_np_i_slot = var_np_i;
        *var_psce_i_slot = var_psce_i;
        *var_psceb_i_slot = var_psceb_i;
        *var_psced_i_slot = var_psced_i;
        *var_rs_i_slot = var_rs_i;
        *var_rsb_i_slot = var_rsb_i;
        *var_rsg_i_slot = var_rsg_i;
        *var_scb_i_slot = var_scb_i;
        *var_scc_i_slot = var_scc_i;
        *var_st2vfb_i_slot = var_st2vfb_i;
        *var_stbet_i_slot = var_stbet_i;
        *var_stcs_i_slot = var_stcs_i;
        *var_stct_i_slot = var_stct_i;
        *var_stmue_i_slot = var_stmue_i;
        *var_strs_i_slot = var_strs_i;
        *var_stthecs_i_slot = var_stthecs_i;
        *var_stthemu_i_slot = var_stthemu_i;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_temp0_slot = var_temp0;
        *var_thecs_i_slot = var_thecs_i;
        *var_themu_i_slot = var_themu_i;
        *var_thesat_i_slot = var_thesat_i;
        *var_thesatb_i_slot = var_thesatb_i;
        *var_thesatg_i_slot = var_thesatg_i;
        *var_thesatt_i_slot = var_thesatt_i;
        *var_tox_i_slot = var_tox_i;
        *var_toxov_i_slot = var_toxov_i;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_vfb_i_slot = var_vfb_i;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vp_i_slot = var_vp_i;
        *var_vsbnud_i_slot = var_vsbnud_i;
        *var_xcor_i_slot = var_xcor_i;
    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
        var_a1_p: f64,
        var_a2_p: f64,
        var_a3_p: f64,
        var_a4_p: f64,
        var_agidl_p: f64,
        var_agidld_p: f64,
        var_alp1ac_p: f64,
        var_alpac_p: f64,
        var_axac_p: f64,
        var_axinr_p: f64,
        var_betnedge_p: f64,
        var_bgidl_p: f64,
        var_bgidld_p: f64,
        var_cfbedge_p: f64,
        var_cfdedge_p: f64,
        var_cfedge_p: f64,
        var_cfr_p: f64,
        var_cfrd_p: f64,
        var_cgbov_p: f64,
        var_cgidl_p: f64,
        var_cgidld_p: f64,
        var_cgov_p: f64,
        var_cgovaccg_p: f64,
        var_cgovd_p: f64,
        var_chib_p: f64,
        var_cinr_p: f64,
        var_cinrd_p: f64,
        var_ctedge_p: f64,
        var_cth_p: f64,
        var_delvtac_p: f64,
        var_dphibedge_p: f64,
        var_dvfbinr_p: f64,
        var_facneffac_p: f64,
        var_fcgovacc_p: f64,
        var_fcgovaccd_p: f64,
        var_fcinracc_p: f64,
        var_fcinrdep_p: f64,
        var_gc2_p: f64,
        var_gc2ov_p: f64,
        var_gc2ovd_p: f64,
        var_gc3_p: f64,
        var_gc3ov_p: f64,
        var_gc3ovd_p: f64,
        var_gco_p: f64,
        var_iginv_p: f64,
        var_igov_p: f64,
        var_igovd_p: f64,
        var_imaxii_p: f64,
        var_neffedge_p: f64,
        var_nf_i: f64,
        var_nov_i: f64,
        var_pscebedge_p: f64,
        var_pscededge_p: f64,
        var_psceedge_p: f64,
        var_rbulk_p: f64,
        var_rde_p: f64,
        var_rg_p: f64,
        var_rjund_p: f64,
        var_rjuns_p: f64,
        var_rse_p: f64,
        var_rth_p: f64,
        var_rwell_p: f64,
        var_sta2_p: f64,
        var_stbetedge_p: f64,
        var_stbgidl_p: f64,
        var_stbgidld_p: f64,
        var_stig_p: f64,
        var_strth_p: f64,
        var_stvfbedge_p: f64,
        var_thesatac_p: f64,
        var_toxov_i: f64,
        var_vfbedge_p: f64,
        var_a1_i_slot: &mut f64,
        var_a2_i_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_a4_i_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_alp1ac_i_slot: &mut f64,
        var_alpac_i_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_axinr_i_slot: &mut f64,
        var_betnedge_i_slot: &mut f64,
        var_bgidl_i_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
        var_cfbedge_i_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cfedge_i_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgov_i_slot: &mut f64,
        var_cgovaccg_i_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_cinr_i_slot: &mut f64,
        var_cinrd_i_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_cth_i_slot: &mut f64,
        var_delvtac_i_slot: &mut f64,
        var_delvto_i_slot: &mut f64,
        var_delvtoedge_i_slot: &mut f64,
        var_dphibedge_i_slot: &mut f64,
        var_dvfbinr_i_slot: &mut f64,
        var_facneffac_i_slot: &mut f64,
        var_factuo_i_slot: &mut f64,
        var_factuoedge_i_slot: &mut f64,
        var_fcgovacc_i_slot: &mut f64,
        var_fcgovaccd_i_slot: &mut f64,
        var_fcinracc_i_slot: &mut f64,
        var_fcinrdep_i_slot: &mut f64,
        var_gc2_i_slot: &mut f64,
        var_gc2ov_i_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc3_i_slot: &mut f64,
        var_gc3ov_i_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_imaxii_i_slot: &mut f64,
        var_mult_inst_slot: &mut f64,
        var_neffedge_i_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_pscebedge_i_slot: &mut f64,
        var_pscededge_i_slot: &mut f64,
        var_psceedge_i_slot: &mut f64,
        var_rbulk_i_slot: &mut f64,
        var_rde_i_slot: &mut f64,
        var_rg_i_slot: &mut f64,
        var_rjund_i_slot: &mut f64,
        var_rjuns_i_slot: &mut f64,
        var_rse_i_slot: &mut f64,
        var_rth_i_slot: &mut f64,
        var_rwell_i_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_strth_i_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_thesatac_i_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_vfbedge_i_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a2_i: f64 = *var_a2_i_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_a4_i: f64 = *var_a4_i_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_alp1ac_i: f64 = *var_alp1ac_i_slot;
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_axinr_i: f64 = *var_axinr_i_slot;
        let mut var_betnedge_i: f64 = *var_betnedge_i_slot;
        let mut var_bgidl_i: f64 = *var_bgidl_i_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
        let mut var_cfbedge_i: f64 = *var_cfbedge_i_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cfedge_i: f64 = *var_cfedge_i_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgov_i: f64 = *var_cgov_i_slot;
        let mut var_cgovaccg_i: f64 = *var_cgovaccg_i_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_cinr_i: f64 = *var_cinr_i_slot;
        let mut var_cinrd_i: f64 = *var_cinrd_i_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_cth_i: f64 = *var_cth_i_slot;
        let mut var_delvtac_i: f64 = *var_delvtac_i_slot;
        let mut var_delvto_i: f64 = *var_delvto_i_slot;
        let mut var_delvtoedge_i: f64 = *var_delvtoedge_i_slot;
        let mut var_dphibedge_i: f64 = *var_dphibedge_i_slot;
        let mut var_dvfbinr_i: f64 = *var_dvfbinr_i_slot;
        let mut var_facneffac_i: f64 = *var_facneffac_i_slot;
        let mut var_factuo_i: f64 = *var_factuo_i_slot;
        let mut var_factuoedge_i: f64 = *var_factuoedge_i_slot;
        let mut var_fcgovacc_i: f64 = *var_fcgovacc_i_slot;
        let mut var_fcgovaccd_i: f64 = *var_fcgovaccd_i_slot;
        let mut var_fcinracc_i: f64 = *var_fcinracc_i_slot;
        let mut var_fcinrdep_i: f64 = *var_fcinrdep_i_slot;
        let mut var_gc2_i: f64 = *var_gc2_i_slot;
        let mut var_gc2ov_i: f64 = *var_gc2ov_i_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc3_i: f64 = *var_gc3_i_slot;
        let mut var_gc3ov_i: f64 = *var_gc3ov_i_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_imaxii_i: f64 = *var_imaxii_i_slot;
        let mut var_mult_inst: f64 = *var_mult_inst_slot;
        let mut var_neffedge_i: f64 = *var_neffedge_i_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_pscebedge_i: f64 = *var_pscebedge_i_slot;
        let mut var_pscededge_i: f64 = *var_pscededge_i_slot;
        let mut var_psceedge_i: f64 = *var_psceedge_i_slot;
        let mut var_rbulk_i: f64 = *var_rbulk_i_slot;
        let mut var_rde_i: f64 = *var_rde_i_slot;
        let mut var_rg_i: f64 = *var_rg_i_slot;
        let mut var_rjund_i: f64 = *var_rjund_i_slot;
        let mut var_rjuns_i: f64 = *var_rjuns_i_slot;
        let mut var_rse_i: f64 = *var_rse_i_slot;
        let mut var_rth_i: f64 = *var_rth_i_slot;
        let mut var_rwell_i: f64 = *var_rwell_i_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_strth_i: f64 = *var_strth_i_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_thesatac_i: f64 = *var_thesatac_i_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_vfbedge_i: f64 = *var_vfbedge_i_slot;

        let (assign10100_e9955,) = {
    if (var_a1_p > 0.0) {
        (var_a1_p,)
    } else {
        (0.0,)
    }
};
        var_a1_i = assign10100_e9955;

        var_a2_i = var_a2_p;

        var_sta2_i = var_sta2_p;

        let (assign10130_e9963,) = {
    if (var_a3_p > 0.0) {
        (var_a3_p,)
    } else {
        (0.0,)
    }
};
        var_a3_i = assign10130_e9963;

        let (assign10140_e9969,) = {
    if (var_a4_p > 0.0) {
        (var_a4_p,)
    } else {
        (0.0,)
    }
};
        var_a4_i = assign10140_e9969;

        let (assign10150_e9975,) = {
    if (var_imaxii_p > 1e-12) {
        (var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        var_imaxii_i = assign10150_e9975;

        var_gco_i = var_gco_p;

        let (assign10170_e9982,) = {
    if (var_iginv_p > 0.0) {
        (var_iginv_p,)
    } else {
        (0.0,)
    }
};
        var_iginv_i = assign10170_e9982;

        let (assign10180_e9988,) = {
    if (var_igov_p > 0.0) {
        (var_igov_p,)
    } else {
        (0.0,)
    }
};
        var_igov_i = assign10180_e9988;

        let (assign10190_e9994,) = {
    if (var_igovd_p > 0.0) {
        (var_igovd_p,)
    } else {
        (0.0,)
    }
};
        var_igovd_i = assign10190_e9994;

        var_stig_i = var_stig_p;

        var_gc2_i = var_gc2_p;

        var_gc3_i = var_gc3_p;

        var_gc2ov_i = var_gc2ov_p;

        var_gc3ov_i = var_gc3ov_p;

        var_gc2ovd_i = var_gc2ovd_p;

        var_gc3ovd_i = var_gc3ovd_p;

        var_chib_i = var_chib_p;

        let (assign10280_e10008,) = {
    if (var_agidl_p > 0.0) {
        (var_agidl_p,)
    } else {
        (0.0,)
    }
};
        var_agidl_i = assign10280_e10008;

        let (assign10290_e10014,) = {
    if (var_agidld_p > 0.0) {
        (var_agidld_p,)
    } else {
        (0.0,)
    }
};
        var_agidld_i = assign10290_e10014;

        var_bgidl_i = var_bgidl_p;

        var_bgidld_i = var_bgidld_p;

        var_stbgidl_i = var_stbgidl_p;

        var_stbgidld_i = var_stbgidld_p;

        var_cgidl_i = var_cgidl_p;

        var_cgidld_i = var_cgidld_p;

        if (s.v[118] > 0.0) {
            s.copy_ad(253, 118);
        } else {
            s.store_scalar(253, 0.0);
        }

        var_delvtac_i = var_delvtac_p;

        let (assign10380_e10033,) = {
    if (var_facneffac_p > 0.0) {
        (var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        var_facneffac_i = assign10380_e10033;

        let (assign10390_e10039,) = {
    if (var_thesatac_p > 0.0) {
        (var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        var_thesatac_i = assign10390_e10039;

        let (assign10400_e10045,) = {
    if (var_axac_p > 2.0) {
        (var_axac_p,)
    } else {
        (2.0,)
    }
};
        var_axac_i = assign10400_e10045;

        var_alpac_i = var_alpac_p;

        let (assign10420_e10052,) = {
    if (var_alp1ac_p > 0.0) {
        (var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        var_alp1ac_i = assign10420_e10052;

        let (assign10430_e10058,) = {
    if (var_cgov_p > 0.0) {
        (var_cgov_p,)
    } else {
        (0.0,)
    }
};
        var_cgov_i = assign10430_e10058;

        let (assign10440_e10064,) = {
    if (var_cgovd_p > 0.0) {
        (var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        var_cgovd_i = assign10440_e10064;

        var_fcgovacc_i = var_fcgovacc_p;

        var_fcgovaccd_i = var_fcgovaccd_p;

        var_cgovaccg_i = var_cgovaccg_p;

        let (assign10480_e10073,) = {
    if (var_cgbov_p > 0.0) {
        (var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        var_cgbov_i = assign10480_e10073;

        let (assign10490_e10079,) = {
    if (var_cinr_p > 0.0) {
        (var_cinr_p,)
    } else {
        (0.0,)
    }
};
        var_cinr_i = assign10490_e10079;

        let (assign10500_e10085,) = {
    if (var_cinrd_p > 0.0) {
        (var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        var_cinrd_i = assign10500_e10085;

        var_dvfbinr_i = var_dvfbinr_p;

        var_fcinrdep_i = var_fcinrdep_p;

        var_fcinracc_i = var_fcinracc_p;

        var_axinr_i = var_axinr_p;

        let (assign10550_e10095,) = {
    if (var_cfr_p > 0.0) {
        (var_cfr_p,)
    } else {
        (0.0,)
    }
};
        var_cfr_i = assign10550_e10095;

        let (assign10560_e10101,) = {
    if (var_cfrd_p > 0.0) {
        (var_cfrd_p,)
    } else {
        (0.0,)
    }
};
        var_cfrd_i = assign10560_e10101;

        s.copy_ad(274, 139);

        if (s.v[140] > 0.0) {
            s.copy_ad(275, 140);
        } else {
            s.store_scalar(275, 0.0);
        }

        var_vfbedge_i = var_vfbedge_p;

        var_stvfbedge_i = var_stvfbedge_p;

        var_dphibedge_i = var_dphibedge_p;

        let (assign10660_e10141,) = {
    if (var_neffedge_p > 1e20) {
        let (assign10660_e10139,) = {
            if (var_neffedge_p < 1e26) {
                (var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10660_e10139,)
    } else {
        (1e20,)
    }
};
        var_neffedge_i = assign10660_e10141;

        let (assign10670_e10147,) = {
    if (var_ctedge_p > 0.0) {
        (var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        var_ctedge_i = assign10670_e10147;

        let (assign10680_e10153,) = {
    if (var_betnedge_p > 0.0) {
        (var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        var_betnedge_i = assign10680_e10153;

        var_stbetedge_i = var_stbetedge_p;

        let (assign10700_e10160,) = {
    if (var_psceedge_p > 0.0) {
        (var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        var_psceedge_i = assign10700_e10160;

        let (assign10710_e10171,) = {
    if (var_pscebedge_p > 0.0) {
        let (assign10710_e10169,) = {
            if (var_pscebedge_p < 1.0) {
                (var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10710_e10169,)
    } else {
        (0.0,)
    }
};
        var_pscebedge_i = assign10710_e10171;

        let (assign10720_e10177,) = {
    if (var_pscededge_p > 0.0) {
        (var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        var_pscededge_i = assign10720_e10177;

        let (assign10730_e10183,) = {
    if (var_cfedge_p > 0.0) {
        (var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfedge_i = assign10730_e10183;

        let (assign10740_e10194,) = {
    if (var_cfbedge_p > 0.0) {
        let (assign10740_e10192,) = {
            if (var_cfbedge_p < 1.0) {
                (var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10740_e10192,)
    } else {
        (0.0,)
    }
};
        var_cfbedge_i = assign10740_e10194;

        let (assign10750_e10200,) = {
    if (var_cfdedge_p > 0.0) {
        (var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfdedge_i = assign10750_e10200;

        let (assign10810_e10226,) = {
    if (var_rg_p > 0.0) {
        (var_rg_p,)
    } else {
        (0.0,)
    }
};
        var_rg_i = assign10810_e10226;

        var_rse_i = var_rse_p;

        var_rde_i = var_rde_p;

        var_rbulk_i = var_rbulk_p;

        var_rjuns_i = var_rjuns_p;

        var_rjund_i = var_rjund_p;

        var_rwell_i = var_rwell_p;

        let (assign10880_e10238,) = {
    if (var_rth_p > 0.0001) {
        (var_rth_p,)
    } else {
        (0.0001,)
    }
};
        var_rth_i = assign10880_e10238;

        let (assign10890_e10244,) = {
    if (var_cth_p > 0.0) {
        (var_cth_p,)
    } else {
        (0.0,)
    }
};
        var_cth_i = assign10890_e10244;

        var_strth_i = var_strth_p;

        let assign10910_e10248: f64 = (p.p31 * var_nf_i);
        let (assign10910_e10255,) = {
    if (assign10910_e10248 > 0.0) {
        let assign10910_e10253: f64 = (p.p31 * var_nf_i);
        (assign10910_e10253,)
    } else {
        (0.0,)
    }
};
        var_mult_inst = assign10910_e10255;

        var_factuo_i = p.p16;

        var_delvto_i = p.p15;

        var_factuoedge_i = p.p18;

        var_delvtoedge_i = p.p17;

        let assign10960_e10262: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard150 = assign10960_e10262;

        let (assign10970_e10266,) = {
    if (var_guard150 != 0.0) {
        (var_toxov_i,)
    } else {
        (var_toxovd_i,)
    }
};
        var_toxovd_i = assign10970_e10266;

        let (assign10980_e10270,) = {
    if (var_guard150 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign10980_e10270;

        let (assign10990_e10274,) = {
    if (var_guard150 != 0.0) {
        (var_agidl_i,)
    } else {
        (var_agidld_i,)
    }
};
        var_agidld_i = assign10990_e10274;

        let (assign11000_e10278,) = {
    if (var_guard150 != 0.0) {
        (var_bgidl_i,)
    } else {
        (var_bgidld_i,)
    }
};
        var_bgidld_i = assign11000_e10278;

        let (assign11010_e10282,) = {
    if (var_guard150 != 0.0) {
        (var_stbgidl_i,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign11010_e10282;

        let (assign11020_e10286,) = {
    if (var_guard150 != 0.0) {
        (var_cgidl_i,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign11020_e10286;

        let (assign11030_e10290,) = {
    if (var_guard150 != 0.0) {
        (var_igov_i,)
    } else {
        (var_igovd_i,)
    }
};
        var_igovd_i = assign11030_e10290;

        let (assign11040_e10294,) = {
    if (var_guard150 != 0.0) {
        (var_gc2ov_i,)
    } else {
        (var_gc2ovd_i,)
    }
};
        var_gc2ovd_i = assign11040_e10294;

        let (assign11050_e10298,) = {
    if (var_guard150 != 0.0) {
        (var_gc3ov_i,)
    } else {
        (var_gc3ovd_i,)
    }
};
        var_gc3ovd_i = assign11050_e10298;

        let (assign11060_e10302,) = {
    if (var_guard150 != 0.0) {
        (var_cgov_i,)
    } else {
        (var_cgovd_i,)
    }
};
        var_cgovd_i = assign11060_e10302;

        *var_a1_i_slot = var_a1_i;
        *var_a2_i_slot = var_a2_i;
        *var_a3_i_slot = var_a3_i;
        *var_a4_i_slot = var_a4_i;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidld_i_slot = var_agidld_i;
        *var_alp1ac_i_slot = var_alp1ac_i;
        *var_alpac_i_slot = var_alpac_i;
        *var_axac_i_slot = var_axac_i;
        *var_axinr_i_slot = var_axinr_i;
        *var_betnedge_i_slot = var_betnedge_i;
        *var_bgidl_i_slot = var_bgidl_i;
        *var_bgidld_i_slot = var_bgidld_i;
        *var_cfbedge_i_slot = var_cfbedge_i;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cfedge_i_slot = var_cfedge_i;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgov_i_slot = var_cgov_i;
        *var_cgovaccg_i_slot = var_cgovaccg_i;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_chib_i_slot = var_chib_i;
        *var_cinr_i_slot = var_cinr_i;
        *var_cinrd_i_slot = var_cinrd_i;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_cth_i_slot = var_cth_i;
        *var_delvtac_i_slot = var_delvtac_i;
        *var_delvto_i_slot = var_delvto_i;
        *var_delvtoedge_i_slot = var_delvtoedge_i;
        *var_dphibedge_i_slot = var_dphibedge_i;
        *var_dvfbinr_i_slot = var_dvfbinr_i;
        *var_facneffac_i_slot = var_facneffac_i;
        *var_factuo_i_slot = var_factuo_i;
        *var_factuoedge_i_slot = var_factuoedge_i;
        *var_fcgovacc_i_slot = var_fcgovacc_i;
        *var_fcgovaccd_i_slot = var_fcgovaccd_i;
        *var_fcinracc_i_slot = var_fcinracc_i;
        *var_fcinrdep_i_slot = var_fcinrdep_i;
        *var_gc2_i_slot = var_gc2_i;
        *var_gc2ov_i_slot = var_gc2ov_i;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc3_i_slot = var_gc3_i;
        *var_gc3ov_i_slot = var_gc3ov_i;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gco_i_slot = var_gco_i;
        *var_guard150_slot = var_guard150;
        *var_iginv_i_slot = var_iginv_i;
        *var_igov_i_slot = var_igov_i;
        *var_igovd_i_slot = var_igovd_i;
        *var_imaxii_i_slot = var_imaxii_i;
        *var_mult_inst_slot = var_mult_inst;
        *var_neffedge_i_slot = var_neffedge_i;
        *var_novd_i_slot = var_novd_i;
        *var_pscebedge_i_slot = var_pscebedge_i;
        *var_pscededge_i_slot = var_pscededge_i;
        *var_psceedge_i_slot = var_psceedge_i;
        *var_rbulk_i_slot = var_rbulk_i;
        *var_rde_i_slot = var_rde_i;
        *var_rg_i_slot = var_rg_i;
        *var_rjund_i_slot = var_rjund_i;
        *var_rjuns_i_slot = var_rjuns_i;
        *var_rse_i_slot = var_rse_i;
        *var_rth_i_slot = var_rth_i;
        *var_rwell_i_slot = var_rwell_i;
        *var_sta2_i_slot = var_sta2_i;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stig_i_slot = var_stig_i;
        *var_strth_i_slot = var_strth_i;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_thesatac_i_slot = var_thesatac_i;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_vfbedge_i_slot = var_vfbedge_i;
    }
}

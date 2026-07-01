#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        var_alphaav_slot: &mut f64,
        var_berfc_slot: &mut f64,
        var_cerfc_slot: &mut f64,
        var_chnl_type_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorgat2nd_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
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
        var_one_over_one_minus_pgat2nd_slot: &mut f64,
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
        let mut var_cjorgat2nd: f64 = *var_cjorgat2nd_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
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
        let mut var_one_over_one_minus_pgat2nd: f64 = *var_one_over_one_minus_pgat2nd_slot;
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

        let assign00_e1445: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1 = assign00_e1445;

        let (assign10_e1450,) = {
    if (var_guard1 != 0.0) {
        let assign10_e1448: f64 = 1.0;
        (assign10_e1448,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign10_e1450;

        let (assign20_e1456,) = {
    if (var_guard1 == 0.0) {
        let assign20_e1454: f64 = (-1.0);
        (assign20_e1454,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign20_e1456;

        let assign30_e1459: f64 = (8.8541878176e-12 * 11.8);
        var_epssi = assign30_e1459;

        let assign40_e1462: f64 = (273.15 + p.p38);
        var_tkr = assign40_e1462;

        var_swjunexp_i = 0.0;

        let assign60_e1466: f64 = if p.p920 > 0.5 { 1.0 } else { 0.0 };
        var_guard2 = assign60_e1466;

        let (assign70_e1470,) = {
    if (var_guard2 != 0.0) {
        (1.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign70_e1470;

        let (assign80_e1475,) = {
    if (var_guard2 == 0.0) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign80_e1475;

        let assign90_e1478: f64 = (273.15 + p.p816);
        var_tkr_1 = assign90_e1478;

        let assign100_e1481: f64 = (1.3806505e-23 / 1.6021918e-19);
        var_kbol_over_qele = assign100_e1481;

        let assign110_e1484: f64 = (var_kbol_over_qele * var_tkr_1);
        var_phitr = assign110_e1484;

        let assign120_e1487: f64 = (1.0 / var_phitr);
        var_phitrinv = assign120_e1487;

        let assign130_e1490: f64 = (0.000702 * var_tkr_1);
        let assign130_e1492: f64 = (assign130_e1490 * var_tkr_1);
        let assign130_e1493: f64 = (-assign130_e1492);
        let assign130_e1496: f64 = (1108.0 + var_tkr_1);
        let assign130_e1497: f64 = (assign130_e1493 / assign130_e1496);
        var_deltaphigr = assign130_e1497;

        let assign140_e1500: f64 = (p.p827 + var_deltaphigr);
        var_phigrbot = assign140_e1500;

        let assign150_e1503: f64 = (p.p828 + var_deltaphigr);
        var_phigrsti = assign150_e1503;

        let assign160_e1506: f64 = (p.p829 + var_deltaphigr);
        var_phigrgat = assign160_e1506;

        let assign170_e1509: f64 = (1.0 - p.p824);
        var_one_minus_pbot = assign170_e1509;

        let assign180_e1512: f64 = (1.0 - p.p825);
        var_one_minus_psti = assign180_e1512;

        let assign190_e1515: f64 = (1.0 - p.p826);
        var_one_minus_pgat = assign190_e1515;

        let assign200_e1518: f64 = (1.0 / var_one_minus_pbot);
        var_one_over_one_minus_pbot = assign200_e1518;

        let assign210_e1521: f64 = (1.0 / var_one_minus_psti);
        var_one_over_one_minus_psti = assign210_e1521;

        let assign220_e1524: f64 = (1.0 / var_one_minus_pgat);
        var_one_over_one_minus_pgat = assign220_e1524;

        let assign230_e1527: f64 = (var_epssi / p.p818);
        var_wdepnulrbot = assign230_e1527;

        let assign240_e1530: f64 = (p.p836 * var_epssi);
        let assign240_e1532: f64 = (assign240_e1530 / p.p819);
        var_wdepnulrsti = assign240_e1532;

        let assign250_e1535: f64 = (p.p837 * var_epssi);
        let assign250_e1537: f64 = (assign250_e1535 / p.p820);
        var_wdepnulrgat = assign250_e1537;

        let assign260_e1540: f64 = (1.0 / var_wdepnulrbot);
        var_wdepnulrinvbot = assign260_e1540;

        let assign270_e1543: f64 = (1.0 / var_wdepnulrsti);
        var_wdepnulrinvsti = assign270_e1543;

        let assign280_e1546: f64 = (1.0 / var_wdepnulrgat);
        var_wdepnulrinvgat = assign280_e1546;

        let assign290_e1549: f64 = (1.0 / p.p821);
        var_vbirbotinv = assign290_e1549;

        let assign300_e1552: f64 = (1.0 / p.p822);
        var_vbirstiinv = assign300_e1552;

        let assign310_e1555: f64 = (1.0 / p.p823);
        var_vbirgatinv = assign310_e1555;

        let assign320_e1558: f64 = (1.772453850905516 * 0.29214664);
        var_perfc = assign320_e1558;

        let assign330_e1560: f64 = (-5.0);
        let assign330_e1562: f64 = (assign330_e1560 * 0.29214664);
        let assign330_e1564: f64 = (assign330_e1562 + 6.0);
        let assign330_e1567: f64 = (-2.0);
        let assign330_e1568: f64 = (var_perfc).powf(assign330_e1567);
        let assign330_e1569: f64 = (assign330_e1564 - assign330_e1568);
        let assign330_e1571: f64 = (assign330_e1569 / 3.0);
        var_berfc = assign330_e1571;

        let assign340_e1574: f64 = (1.0 - 0.29214664);
        let assign340_e1576: f64 = (assign340_e1574 - var_berfc);
        var_cerfc = assign340_e1576;

        let assign350_e1580: f64 = (1.0 / p.p817);
        let assign350_e1581: f64 = (1.0 - assign350_e1580);
        var_alphaav = assign350_e1581;

        let assign360_e1586: f64 = (var_alphaav).powf(p.p856);
        let assign360_e1587: f64 = (1.0 - assign360_e1586);
        let assign360_e1588: f64 = (1.0 / assign360_e1587);
        var_fstopbot = assign360_e1588;

        let assign370_e1593: f64 = (var_alphaav).powf(p.p857);
        let assign370_e1594: f64 = (1.0 - assign370_e1593);
        let assign370_e1595: f64 = (1.0 / assign370_e1594);
        var_fstopsti = assign370_e1595;

        let assign380_e1600: f64 = (var_alphaav).powf(p.p858);
        let assign380_e1601: f64 = (1.0 - assign380_e1600);
        let assign380_e1602: f64 = (1.0 / assign380_e1601);
        var_fstopgat = assign380_e1602;

        let assign390_e1605: f64 = (1.0 / p.p853);
        var_vbrinvbot = assign390_e1605;

        let assign400_e1608: f64 = (1.0 / p.p854);
        var_vbrinvsti = assign400_e1608;

        let assign410_e1611: f64 = (1.0 / p.p855);
        var_vbrinvgat = assign410_e1611;
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
        var_vbrinvgat_db0 = 0.0;
        var_vbrinvgat_db1 = 0.0;
        var_vbrinvgat_db2 = 0.0;
        var_vbrinvgat_db3 = 0.0;
        var_vbrinvgat_db4 = 0.0;
        var_vbrinvgat_db5 = 0.0;
        var_vbrinvgat_db6 = 0.0;

        let assign420_e1614: f64 = (var_fstopbot * var_fstopbot);
        let assign420_e1618: f64 = (p.p856 - 1.0);
        let assign420_e1619: f64 = (var_alphaav).powf(assign420_e1618);
        let assign420_e1620: f64 = (assign420_e1614 * assign420_e1619);
        let assign420_e1621: f64 = (-assign420_e1620);
        let assign420_e1623: f64 = (assign420_e1621 * p.p856);
        let assign420_e1625: f64 = (assign420_e1623 * var_vbrinvbot);
        var_slopebot = assign420_e1625;

        let assign430_e1628: f64 = (var_fstopsti * var_fstopsti);
        let assign430_e1632: f64 = (p.p857 - 1.0);
        let assign430_e1633: f64 = (var_alphaav).powf(assign430_e1632);
        let assign430_e1634: f64 = (assign430_e1628 * assign430_e1633);
        let assign430_e1635: f64 = (-assign430_e1634);
        let assign430_e1637: f64 = (assign430_e1635 * p.p857);
        let assign430_e1639: f64 = (assign430_e1637 * var_vbrinvsti);
        var_slopesti = assign430_e1639;

        let assign440_e1642: f64 = (var_fstopgat * var_fstopgat);
        let assign440_e1646: f64 = (p.p858 - 1.0);
        let assign440_e1647: f64 = (var_alphaav).powf(assign440_e1646);
        let assign440_e1648: f64 = (assign440_e1642 * assign440_e1647);
        let assign440_e1649: f64 = (-assign440_e1648);
        let assign440_e1651: f64 = (assign440_e1649 * p.p858);
        let assign440_e1653: f64 = (assign440_e1651 * var_vbrinvgat);
        var_slopegat = assign440_e1653;
        var_slopegat_dn0 = (assign440_e1651 * var_vbrinvgat_dn0);
        var_slopegat_dn1 = (assign440_e1651 * var_vbrinvgat_dn1);
        var_slopegat_dn2 = (assign440_e1651 * var_vbrinvgat_dn2);
        var_slopegat_dn3 = (assign440_e1651 * var_vbrinvgat_dn3);
        var_slopegat_dn4 = (assign440_e1651 * var_vbrinvgat_dn4);
        var_slopegat_dn5 = (assign440_e1651 * var_vbrinvgat_dn5);
        var_slopegat_dn6 = (assign440_e1651 * var_vbrinvgat_dn6);
        var_slopegat_dn7 = (assign440_e1651 * var_vbrinvgat_dn7);
        var_slopegat_dn8 = (assign440_e1651 * var_vbrinvgat_dn8);
        var_slopegat_dn9 = (assign440_e1651 * var_vbrinvgat_dn9);
        var_slopegat_dn10 = (assign440_e1651 * var_vbrinvgat_dn10);
        var_slopegat_dn11 = (assign440_e1651 * var_vbrinvgat_dn11);
        var_slopegat_db0 = (assign440_e1651 * var_vbrinvgat_db0);
        var_slopegat_db1 = (assign440_e1651 * var_vbrinvgat_db1);
        var_slopegat_db2 = (assign440_e1651 * var_vbrinvgat_db2);
        var_slopegat_db3 = (assign440_e1651 * var_vbrinvgat_db3);
        var_slopegat_db4 = (assign440_e1651 * var_vbrinvgat_db4);
        var_slopegat_db5 = (assign440_e1651 * var_vbrinvgat_db5);
        var_slopegat_db6 = (assign440_e1651 * var_vbrinvgat_db6);

        let assign450_e1668: f64 = if ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0)) { 1.0 } else { 0.0 };
        var_guard3 = assign450_e1668;

        let (assign460_e1672,) = {
    if (var_guard3 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign460_e1672;

        let (assign470_e1677,) = {
    if (var_guard3 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign470_e1677;

        let assign480_e1680: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard4 = assign480_e1680;

        let (assign490_e1693,) = {
    if (var_guard4 != 0.0) {
        let assign490_e1684: f64 = (p.p820 * p.p859);
        let (assign490_e1691,) = {
            if (assign490_e1684 > 1e-18) {
                let assign490_e1689: f64 = (p.p820 * p.p859);
                (assign490_e1689,)
            } else {
                (1e-18,)
            }
        };
        (assign490_e1691,)
    } else {
        (var_cjorgat2nd,)
    }
};
        var_cjorgat2nd = assign490_e1693;

        let (assign500_e1706,) = {
    if (var_guard4 != 0.0) {
        let assign500_e1697: f64 = (p.p823 * p.p860);
        let (assign500_e1704,) = {
            if (assign500_e1697 > 0.05) {
                let assign500_e1702: f64 = (p.p823 * p.p860);
                (assign500_e1702,)
            } else {
                (0.05,)
            }
        };
        (assign500_e1704,)
    } else {
        (var_vbirgat2nd,)
    }
};
        var_vbirgat2nd = assign500_e1706;

        let (assign510_e1733,) = {
    if (var_guard4 != 0.0) {
        let assign510_e1710: f64 = (p.p826 * p.p861);
        let (assign510_e1717,) = {
            if (assign510_e1710 > 0.05) {
                let assign510_e1715: f64 = (p.p826 * p.p861);
                (assign510_e1715,)
            } else {
                (0.05,)
            }
        };
        let (assign510_e1731,) = {
            if (assign510_e1717 < 0.95) {
                let assign510_e1722: f64 = (p.p826 * p.p861);
                let (assign510_e1729,) = {
                    if (assign510_e1722 > 0.05) {
                        let assign510_e1727: f64 = (p.p826 * p.p861);
                        (assign510_e1727,)
                    } else {
                        (0.05,)
                    }
                };
                (assign510_e1729,)
            } else {
                (0.95,)
            }
        };
        (assign510_e1731,)
    } else {
        (var_pgat2nd,)
    }
};
        var_pgat2nd = assign510_e1733;

        let (assign520_e1739,) = {
    if (var_guard4 != 0.0) {
        let assign520_e1737: f64 = (p.p829 * p.p862);
        (assign520_e1737,)
    } else {
        (var_phiggat2nd,)
    }
};
        var_phiggat2nd = assign520_e1739;

        let (assign530_e1745,) = {
    if (var_guard4 != 0.0) {
        let assign530_e1743: f64 = (var_phiggat2nd + var_deltaphigr);
        (assign530_e1743,)
    } else {
        (var_phigrgat2nd,)
    }
};
        var_phigrgat2nd = assign530_e1745;

        let (assign540_e1751,) = {
    if (var_guard4 != 0.0) {
        let assign540_e1749: f64 = (1.0 - var_pgat2nd);
        (assign540_e1749,)
    } else {
        (var_one_minus_pgat2nd,)
    }
};
        var_one_minus_pgat2nd = assign540_e1751;

        let (assign550_e1757,) = {
    if (var_guard4 != 0.0) {
        let assign550_e1755: f64 = (1.0 / var_one_minus_pgat2nd);
        (assign550_e1755,)
    } else {
        (var_one_over_one_minus_pgat2nd,)
    }
};
        var_one_over_one_minus_pgat2nd = assign550_e1757;

        let assign560_e1760: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign560_e1760;

        let (assign570_e1764,) = {
    if (var_guard5 != 0.0) {
        (p.p818,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign570_e1764;

        let (assign580_e1768,) = {
    if (var_guard5 != 0.0) {
        (p.p819,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign580_e1768;

        let (assign590_e1772,) = {
    if (var_guard5 != 0.0) {
        (p.p820,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign590_e1772;

        let (assign600_e1776,) = {
    if (var_guard5 != 0.0) {
        (p.p821,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign600_e1776;

        let (assign610_e1780,) = {
    if (var_guard5 != 0.0) {
        (p.p822,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign610_e1780;

        let (assign620_e1784,) = {
    if (var_guard5 != 0.0) {
        (p.p823,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign620_e1784;

        let (assign630_e1788,) = {
    if (var_guard5 != 0.0) {
        (p.p824,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign630_e1788;

        let (assign640_e1792,) = {
    if (var_guard5 != 0.0) {
        (p.p825,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign640_e1792;

        let (assign650_e1796,) = {
    if (var_guard5 != 0.0) {
        (p.p826,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign650_e1796;

        let (assign660_e1800,) = {
    if (var_guard5 != 0.0) {
        (p.p827,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign660_e1800;

        let (assign670_e1804,) = {
    if (var_guard5 != 0.0) {
        (p.p828,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign670_e1804;

        let (assign680_e1808,) = {
    if (var_guard5 != 0.0) {
        (p.p829,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign680_e1808;

        let (assign690_e1812,) = {
    if (var_guard5 != 0.0) {
        (p.p830,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign690_e1812;

        let (assign700_e1816,) = {
    if (var_guard5 != 0.0) {
        (p.p831,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign700_e1816;

        let (assign710_e1820,) = {
    if (var_guard5 != 0.0) {
        (p.p832,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign710_e1820;

        *var_alphaav_slot = var_alphaav;
        *var_berfc_slot = var_berfc;
        *var_cerfc_slot = var_cerfc;
        *var_chnl_type_slot = var_chnl_type;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorgat2nd_slot = var_cjorgat2nd;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorstid_i_slot = var_cjorstid_i;
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
        *var_one_over_one_minus_pgat2nd_slot = var_one_over_one_minus_pgat2nd;
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
        var_fjunqd_i_slot: &mut f64,
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
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
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

        let (assign720_e1824,) = {
    if (var_guard5 != 0.0) {
        (p.p833,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign720_e1824;

        let (assign730_e1828,) = {
    if (var_guard5 != 0.0) {
        (p.p834,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign730_e1828;

        let (assign740_e1832,) = {
    if (var_guard5 != 0.0) {
        (p.p835,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign740_e1832;

        let (assign750_e1836,) = {
    if (var_guard5 != 0.0) {
        (p.p836,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign750_e1836;

        let (assign760_e1840,) = {
    if (var_guard5 != 0.0) {
        (p.p837,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign760_e1840;

        let (assign770_e1844,) = {
    if (var_guard5 != 0.0) {
        (p.p838,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign770_e1844;

        let (assign780_e1848,) = {
    if (var_guard5 != 0.0) {
        (p.p839,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign780_e1848;

        let (assign790_e1852,) = {
    if (var_guard5 != 0.0) {
        (p.p840,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign790_e1852;

        let (assign800_e1856,) = {
    if (var_guard5 != 0.0) {
        (p.p841,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign800_e1856;

        let (assign810_e1860,) = {
    if (var_guard5 != 0.0) {
        (p.p842,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign810_e1860;

        let (assign820_e1864,) = {
    if (var_guard5 != 0.0) {
        (p.p843,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign820_e1864;

        let (assign830_e1868,) = {
    if (var_guard5 != 0.0) {
        (p.p844,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign830_e1868;

        let (assign840_e1872,) = {
    if (var_guard5 != 0.0) {
        (p.p845,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign840_e1872;

        let (assign850_e1876,) = {
    if (var_guard5 != 0.0) {
        (p.p846,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign850_e1876;

        let (assign860_e1880,) = {
    if (var_guard5 != 0.0) {
        (p.p847,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign860_e1880;

        let (assign870_e1884,) = {
    if (var_guard5 != 0.0) {
        (p.p848,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign870_e1884;

        let (assign880_e1888,) = {
    if (var_guard5 != 0.0) {
        (p.p849,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign880_e1888;

        let (assign890_e1892,) = {
    if (var_guard5 != 0.0) {
        (p.p850,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign890_e1892;

        let (assign900_e1896,) = {
    if (var_guard5 != 0.0) {
        (p.p851,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign900_e1896;

        let (assign910_e1900,) = {
    if (var_guard5 != 0.0) {
        (p.p852,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign910_e1900;

        let (assign920_e1904,) = {
    if (var_guard5 != 0.0) {
        (p.p853,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign920_e1904;

        let (assign930_e1908,) = {
    if (var_guard5 != 0.0) {
        (p.p854,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign930_e1908;

        let (assign940_e1912,) = {
    if (var_guard5 != 0.0) {
        (p.p855,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign940_e1912;

        let (assign950_e1916,) = {
    if (var_guard5 != 0.0) {
        (p.p856,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign950_e1916;

        let (assign960_e1920,) = {
    if (var_guard5 != 0.0) {
        (p.p857,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign960_e1920;

        let (assign970_e1924,) = {
    if (var_guard5 != 0.0) {
        (p.p858,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign970_e1924;

        let (assign980_e1928,) = {
    if (var_guard5 != 0.0) {
        (p.p921,)
    } else {
        (var_vjunrefd_i,)
    }
};
        var_vjunrefd_i = assign980_e1928;

        let (assign990_e1932,) = {
    if (var_guard5 != 0.0) {
        (p.p922,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign990_e1932;

        let (assign1000_e1936,) = {
    if (var_guard5 != 0.0) {
        (p.p865,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1000_e1936;

        let (assign1010_e1940,) = {
    if (var_guard5 != 0.0) {
        (p.p866,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1010_e1940;

        let (assign1020_e1944,) = {
    if (var_guard5 != 0.0) {
        (p.p867,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1020_e1944;

        let (assign1030_e1948,) = {
    if (var_guard5 != 0.0) {
        (p.p868,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1030_e1948;

        let (assign1040_e1952,) = {
    if (var_guard5 != 0.0) {
        (p.p859,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1040_e1952;

        let (assign1050_e1956,) = {
    if (var_guard5 != 0.0) {
        (p.p860,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1050_e1956;

        let (assign1060_e1960,) = {
    if (var_guard5 != 0.0) {
        (p.p861,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1060_e1960;

        let (assign1070_e1964,) = {
    if (var_guard5 != 0.0) {
        (p.p862,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1070_e1964;

        let (assign1080_e1968,) = {
    if (var_guard5 != 0.0) {
        (p.p863,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1080_e1968;

        let (assign1090_e1972,) = {
    if (var_guard5 != 0.0) {
        (p.p864,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1090_e1972;

        let (assign1100_e1977,) = {
    if (var_guard5 == 0.0) {
        (p.p869,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign1100_e1977;

        let (assign1110_e1982,) = {
    if (var_guard5 == 0.0) {
        (p.p870,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign1110_e1982;

        let (assign1120_e1987,) = {
    if (var_guard5 == 0.0) {
        (p.p871,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign1120_e1987;

        let (assign1130_e1992,) = {
    if (var_guard5 == 0.0) {
        (p.p872,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign1130_e1992;

        let (assign1140_e1997,) = {
    if (var_guard5 == 0.0) {
        (p.p873,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign1140_e1997;

        let (assign1150_e2002,) = {
    if (var_guard5 == 0.0) {
        (p.p874,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign1150_e2002;

        let (assign1160_e2007,) = {
    if (var_guard5 == 0.0) {
        (p.p875,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign1160_e2007;

        let (assign1170_e2012,) = {
    if (var_guard5 == 0.0) {
        (p.p876,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign1170_e2012;

        let (assign1180_e2017,) = {
    if (var_guard5 == 0.0) {
        (p.p877,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign1180_e2017;

        let (assign1190_e2022,) = {
    if (var_guard5 == 0.0) {
        (p.p878,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign1190_e2022;

        let (assign1200_e2027,) = {
    if (var_guard5 == 0.0) {
        (p.p879,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign1200_e2027;

        let (assign1210_e2032,) = {
    if (var_guard5 == 0.0) {
        (p.p880,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign1210_e2032;

        let (assign1220_e2037,) = {
    if (var_guard5 == 0.0) {
        (p.p881,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign1220_e2037;

        let (assign1230_e2042,) = {
    if (var_guard5 == 0.0) {
        (p.p882,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign1230_e2042;

        let (assign1240_e2047,) = {
    if (var_guard5 == 0.0) {
        (p.p883,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign1240_e2047;

        let (assign1250_e2052,) = {
    if (var_guard5 == 0.0) {
        (p.p884,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign1250_e2052;

        let (assign1260_e2057,) = {
    if (var_guard5 == 0.0) {
        (p.p885,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign1260_e2057;

        let (assign1270_e2062,) = {
    if (var_guard5 == 0.0) {
        (p.p886,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign1270_e2062;

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
        *var_fjunqd_i_slot = var_fjunqd_i;
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
        var_adbbtgatd_i_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cjorgat2nd_d_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
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
        let mut var_cjorgat2nd_d: f64 = *var_cjorgat2nd_d_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
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
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;

        let (assign1280_e2067,) = {
    if (var_guard5 == 0.0) {
        (p.p887,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign1280_e2067;

        let (assign1290_e2072,) = {
    if (var_guard5 == 0.0) {
        (p.p888,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign1290_e2072;

        let (assign1300_e2077,) = {
    if (var_guard5 == 0.0) {
        (p.p889,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign1300_e2077;

        let (assign1310_e2082,) = {
    if (var_guard5 == 0.0) {
        (p.p890,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign1310_e2082;

        let (assign1320_e2087,) = {
    if (var_guard5 == 0.0) {
        (p.p891,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign1320_e2087;

        let (assign1330_e2092,) = {
    if (var_guard5 == 0.0) {
        (p.p892,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign1330_e2092;

        let (assign1340_e2097,) = {
    if (var_guard5 == 0.0) {
        (p.p893,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign1340_e2097;

        let (assign1350_e2102,) = {
    if (var_guard5 == 0.0) {
        (p.p894,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign1350_e2102;

        let (assign1360_e2107,) = {
    if (var_guard5 == 0.0) {
        (p.p895,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign1360_e2107;

        let (assign1370_e2112,) = {
    if (var_guard5 == 0.0) {
        (p.p896,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign1370_e2112;

        let (assign1380_e2117,) = {
    if (var_guard5 == 0.0) {
        (p.p897,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign1380_e2117;

        let (assign1390_e2122,) = {
    if (var_guard5 == 0.0) {
        (p.p898,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1390_e2122;

        let (assign1400_e2127,) = {
    if (var_guard5 == 0.0) {
        (p.p899,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1400_e2127;

        let (assign1410_e2132,) = {
    if (var_guard5 == 0.0) {
        (p.p900,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1410_e2132;

        let (assign1420_e2137,) = {
    if (var_guard5 == 0.0) {
        (p.p901,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1420_e2137;

        let (assign1430_e2142,) = {
    if (var_guard5 == 0.0) {
        (p.p902,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1430_e2142;

        let (assign1440_e2147,) = {
    if (var_guard5 == 0.0) {
        (p.p903,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1440_e2147;

        let (assign1450_e2152,) = {
    if (var_guard5 == 0.0) {
        (p.p904,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1450_e2152;

        let (assign1460_e2157,) = {
    if (var_guard5 == 0.0) {
        (p.p905,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1460_e2157;

        let (assign1470_e2162,) = {
    if (var_guard5 == 0.0) {
        (p.p906,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1470_e2162;

        let (assign1480_e2167,) = {
    if (var_guard5 == 0.0) {
        (p.p907,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1480_e2167;

        let (assign1490_e2172,) = {
    if (var_guard5 == 0.0) {
        (p.p908,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1490_e2172;

        let (assign1500_e2177,) = {
    if (var_guard5 == 0.0) {
        (p.p909,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1500_e2177;

        let (assign1510_e2182,) = {
    if (var_guard5 == 0.0) {
        (p.p923,)
    } else {
        (var_vjunrefd_i,)
    }
};
        var_vjunrefd_i = assign1510_e2182;

        let (assign1520_e2187,) = {
    if (var_guard5 == 0.0) {
        (p.p924,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1520_e2187;

        let (assign1530_e2192,) = {
    if (var_guard5 == 0.0) {
        (p.p916,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1530_e2192;

        let (assign1540_e2197,) = {
    if (var_guard5 == 0.0) {
        (p.p917,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1540_e2197;

        let (assign1550_e2202,) = {
    if (var_guard5 == 0.0) {
        (p.p918,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1550_e2202;

        let (assign1560_e2207,) = {
    if (var_guard5 == 0.0) {
        (p.p919,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1560_e2207;

        let (assign1570_e2212,) = {
    if (var_guard5 == 0.0) {
        (p.p910,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1570_e2212;

        let (assign1580_e2217,) = {
    if (var_guard5 == 0.0) {
        (p.p911,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1580_e2217;

        let (assign1590_e2222,) = {
    if (var_guard5 == 0.0) {
        (p.p912,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1590_e2222;

        let (assign1600_e2227,) = {
    if (var_guard5 == 0.0) {
        (p.p913,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1600_e2227;

        let (assign1610_e2232,) = {
    if (var_guard5 == 0.0) {
        (p.p914,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1610_e2232;

        let (assign1620_e2237,) = {
    if (var_guard5 == 0.0) {
        (p.p915,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1620_e2237;

        let assign1630_e2240: f64 = (var_phigbotd_i + var_deltaphigr);
        var_phigrbot_d = assign1630_e2240;

        let assign1640_e2243: f64 = (var_phigstid_i + var_deltaphigr);
        var_phigrsti_d = assign1640_e2243;

        let assign1650_e2246: f64 = (var_phiggatd_i + var_deltaphigr);
        var_phigrgat_d = assign1650_e2246;

        let assign1660_e2249: f64 = (1.0 - var_pbotd_i);
        var_one_minus_pbot_d = assign1660_e2249;

        let assign1670_e2252: f64 = (1.0 - var_pstid_i);
        var_one_minus_psti_d = assign1670_e2252;

        let assign1680_e2255: f64 = (1.0 - var_pgatd_i);
        var_one_minus_pgat_d = assign1680_e2255;

        let assign1690_e2258: f64 = (1.0 / var_one_minus_pbot_d);
        var_one_over_one_minus_pbot_d = assign1690_e2258;

        let assign1700_e2261: f64 = (1.0 / var_one_minus_psti_d);
        var_one_over_one_minus_psti_d = assign1700_e2261;

        let assign1710_e2264: f64 = (1.0 / var_one_minus_pgat_d);
        var_one_over_one_minus_pgat_d = assign1710_e2264;

        let assign1720_e2267: f64 = (var_epssi / var_cjorbotd_i);
        var_wdepnulrbot_d = assign1720_e2267;

        let assign1730_e2270: f64 = (var_xjunstid_i * var_epssi);
        let assign1730_e2272: f64 = (assign1730_e2270 / var_cjorstid_i);
        var_wdepnulrsti_d = assign1730_e2272;

        let assign1740_e2275: f64 = (var_xjungatd_i * var_epssi);
        let assign1740_e2277: f64 = (assign1740_e2275 / var_cjorgatd_i);
        var_wdepnulrgat_d = assign1740_e2277;

        let assign1750_e2280: f64 = (1.0 / var_wdepnulrbot_d);
        var_wdepnulrinvbot_d = assign1750_e2280;

        let assign1760_e2283: f64 = (1.0 / var_wdepnulrsti_d);
        var_wdepnulrinvsti_d = assign1760_e2283;

        let assign1770_e2286: f64 = (1.0 / var_wdepnulrgat_d);
        var_wdepnulrinvgat_d = assign1770_e2286;

        let assign1780_e2289: f64 = (1.0 / var_vbirbotd_i);
        var_vbirbotinv_d = assign1780_e2289;

        let assign1790_e2292: f64 = (1.0 / var_vbirstid_i);
        var_vbirstiinv_d = assign1790_e2292;

        let assign1800_e2295: f64 = (1.0 / var_vbirgatd_i);
        var_vbirgatinv_d = assign1800_e2295;

        let assign1810_e2300: f64 = (var_alphaav).powf(var_pbrbotd_i);
        let assign1810_e2301: f64 = (1.0 - assign1810_e2300);
        let assign1810_e2302: f64 = (1.0 / assign1810_e2301);
        var_fstopbot_d = assign1810_e2302;

        let assign1820_e2307: f64 = (var_alphaav).powf(var_pbrstid_i);
        let assign1820_e2308: f64 = (1.0 - assign1820_e2307);
        let assign1820_e2309: f64 = (1.0 / assign1820_e2308);
        var_fstopsti_d = assign1820_e2309;

        let assign1830_e2314: f64 = (var_alphaav).powf(var_pbrgatd_i);
        let assign1830_e2315: f64 = (1.0 - assign1830_e2314);
        let assign1830_e2316: f64 = (1.0 / assign1830_e2315);
        var_fstopgat_d = assign1830_e2316;

        let assign1840_e2319: f64 = (1.0 / var_vbrbotd_i);
        var_vbrinvbot_d = assign1840_e2319;

        let assign1850_e2322: f64 = (1.0 / var_vbrstid_i);
        var_vbrinvsti_d = assign1850_e2322;

        let assign1860_e2325: f64 = (1.0 / var_vbrgatd_i);
        var_vbrinvgat_d = assign1860_e2325;
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
        var_vbrinvgat_d_db0 = 0.0;
        var_vbrinvgat_d_db1 = 0.0;
        var_vbrinvgat_d_db2 = 0.0;
        var_vbrinvgat_d_db3 = 0.0;
        var_vbrinvgat_d_db4 = 0.0;
        var_vbrinvgat_d_db5 = 0.0;
        var_vbrinvgat_d_db6 = 0.0;

        let assign1870_e2328: f64 = (var_fstopbot_d * var_fstopbot_d);
        let assign1870_e2332: f64 = (var_pbrbotd_i - 1.0);
        let assign1870_e2333: f64 = (var_alphaav).powf(assign1870_e2332);
        let assign1870_e2334: f64 = (assign1870_e2328 * assign1870_e2333);
        let assign1870_e2335: f64 = (-assign1870_e2334);
        let assign1870_e2337: f64 = (assign1870_e2335 * var_pbrbotd_i);
        let assign1870_e2339: f64 = (assign1870_e2337 * var_vbrinvbot_d);
        var_slopebot_d = assign1870_e2339;

        let assign1880_e2342: f64 = (var_fstopsti_d * var_fstopsti_d);
        let assign1880_e2346: f64 = (var_pbrstid_i - 1.0);
        let assign1880_e2347: f64 = (var_alphaav).powf(assign1880_e2346);
        let assign1880_e2348: f64 = (assign1880_e2342 * assign1880_e2347);
        let assign1880_e2349: f64 = (-assign1880_e2348);
        let assign1880_e2351: f64 = (assign1880_e2349 * var_pbrstid_i);
        let assign1880_e2353: f64 = (assign1880_e2351 * var_vbrinvsti_d);
        var_slopesti_d = assign1880_e2353;

        let assign1890_e2356: f64 = (var_fstopgat_d * var_fstopgat_d);
        let assign1890_e2360: f64 = (var_pbrgatd_i - 1.0);
        let assign1890_e2361: f64 = (var_alphaav).powf(assign1890_e2360);
        let assign1890_e2362: f64 = (assign1890_e2356 * assign1890_e2361);
        let assign1890_e2363: f64 = (-assign1890_e2362);
        let assign1890_e2365: f64 = (assign1890_e2363 * var_pbrgatd_i);
        let assign1890_e2367: f64 = (assign1890_e2365 * var_vbrinvgat_d);
        var_slopegat_d = assign1890_e2367;
        var_slopegat_d_dn0 = (assign1890_e2365 * var_vbrinvgat_d_dn0);
        var_slopegat_d_dn1 = (assign1890_e2365 * var_vbrinvgat_d_dn1);
        var_slopegat_d_dn2 = (assign1890_e2365 * var_vbrinvgat_d_dn2);
        var_slopegat_d_dn3 = (assign1890_e2365 * var_vbrinvgat_d_dn3);
        var_slopegat_d_dn4 = (assign1890_e2365 * var_vbrinvgat_d_dn4);
        var_slopegat_d_dn5 = (assign1890_e2365 * var_vbrinvgat_d_dn5);
        var_slopegat_d_dn6 = (assign1890_e2365 * var_vbrinvgat_d_dn6);
        var_slopegat_d_dn7 = (assign1890_e2365 * var_vbrinvgat_d_dn7);
        var_slopegat_d_dn8 = (assign1890_e2365 * var_vbrinvgat_d_dn8);
        var_slopegat_d_dn9 = (assign1890_e2365 * var_vbrinvgat_d_dn9);
        var_slopegat_d_dn10 = (assign1890_e2365 * var_vbrinvgat_d_dn10);
        var_slopegat_d_dn11 = (assign1890_e2365 * var_vbrinvgat_d_dn11);
        var_slopegat_d_db0 = (assign1890_e2365 * var_vbrinvgat_d_db0);
        var_slopegat_d_db1 = (assign1890_e2365 * var_vbrinvgat_d_db1);
        var_slopegat_d_db2 = (assign1890_e2365 * var_vbrinvgat_d_db2);
        var_slopegat_d_db3 = (assign1890_e2365 * var_vbrinvgat_d_db3);
        var_slopegat_d_db4 = (assign1890_e2365 * var_vbrinvgat_d_db4);
        var_slopegat_d_db5 = (assign1890_e2365 * var_vbrinvgat_d_db5);
        var_slopegat_d_db6 = (assign1890_e2365 * var_vbrinvgat_d_db6);

        let assign1900_e2382: f64 = if ((((var_fcjorgat2d_i != 1.0) || (var_fvbirgat2d_i != 1.0)) || (var_fpgat2d_i != 1.0)) || (var_fphiggat2d_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard6 = assign1900_e2382;

        let (assign1910_e2386,) = {
    if (var_guard6 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign1910_e2386;

        let (assign1920_e2391,) = {
    if (var_guard6 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign1920_e2391;

        let assign1930_e2394: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard7 = assign1930_e2394;

        let (assign1940_e2407,) = {
    if (var_guard7 != 0.0) {
        let assign1940_e2398: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
        let (assign1940_e2405,) = {
            if (assign1940_e2398 > 1e-18) {
                let assign1940_e2403: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
                (assign1940_e2403,)
            } else {
                (1e-18,)
            }
        };
        (assign1940_e2405,)
    } else {
        (var_cjorgat2nd_d,)
    }
};
        var_cjorgat2nd_d = assign1940_e2407;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cjorgat2nd_d_slot = var_cjorgat2nd_d;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fjunqd_i_slot = var_fjunqd_i;
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
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjunstid_i_slot = var_xjunstid_i;
    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_deltaphigr: f64,
        var_fpgat2d_i: f64,
        var_fphiggat2d_i: f64,
        var_fvbirgat2d_i: f64,
        var_guard7: f64,
        var_kbol_over_qele: f64,
        var_one_over_one_minus_pbot: f64,
        var_one_over_one_minus_pgat: f64,
        var_one_over_one_minus_psti: f64,
        var_pgatd_i: f64,
        var_phiggat2nd: f64,
        var_phiggatd_i: f64,
        var_phigrbot: f64,
        var_phigrgat: f64,
        var_phigrgat2nd: f64,
        var_phigrsti: f64,
        var_phitrinv: f64,
        var_swgat2nd: f64,
        var_tkr: f64,
        var_tkr_1: f64,
        var_vbirgat2nd: f64,
        var_vbirgatd_i: f64,
        var_atatbot_slot: &mut f64,
        var_atatgat_slot: &mut f64,
        var_atatsti_slot: &mut f64,
        var_auxt_slot: &mut f64,
        var_btatpartbot_slot: &mut f64,
        var_btatpartgat_slot: &mut f64,
        var_btatpartsti_slot: &mut f64,
        var_cjobot_slot: &mut f64,
        var_cjogat_slot: &mut f64,
        var_cjosti_slot: &mut f64,
        var_delt_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_deltaebot_slot: &mut f64,
        var_deltaegat_slot: &mut f64,
        var_deltaesti_slot: &mut f64,
        var_deltaphigd_slot: &mut f64,
        var_eg_slot: &mut f64,
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
        var_ftdgat_slot: &mut f64,
        var_ftdgat2nd_slot: &mut f64,
        var_ftdsti_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_idsatbot_slot: &mut f64,
        var_idsatgat_slot: &mut f64,
        var_idsatsti_slot: &mut f64,
        var_inv_phit_slot: &mut f64,
        var_inv_phita_slot: &mut f64,
        var_ln_rtn_slot: &mut f64,
        var_nt0_slot: &mut f64,
        var_one_minus_pgat2nd_d_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_d_slot: &mut f64,
        var_pgat2nd_d_slot: &mut f64,
        var_phibfac_slot: &mut f64,
        var_phigdbot_slot: &mut f64,
        var_phigdgat_slot: &mut f64,
        var_phigdgat2nd_slot: &mut f64,
        var_phigdsti_slot: &mut f64,
        var_phiggat2nd_d_slot: &mut f64,
        var_phigrgat2nd_d_slot: &mut f64,
        var_phit_slot: &mut f64,
        var_phita_slot: &mut f64,
        var_phitd_slot: &mut f64,
        var_phitdinv_slot: &mut f64,
        var_qpref2bot_slot: &mut f64,
        var_qpref2gat_slot: &mut f64,
        var_qpref2sti_slot: &mut f64,
        var_qprefbot_slot: &mut f64,
        var_qprefgat_slot: &mut f64,
        var_qprefsti_slot: &mut f64,
        var_rta_slot: &mut f64,
        var_rtn_slot: &mut f64,
        var_tka_slot: &mut f64,
        var_tkd_slot: &mut f64,
        var_tkd_1_slot: &mut f64,
        var_tkd_sq_slot: &mut f64,
        var_ubibot_slot: &mut f64,
        var_ubigat_slot: &mut f64,
        var_ubigat2nd_slot: &mut f64,
        var_ubisti_slot: &mut f64,
        var_vbibot_slot: &mut f64,
        var_vbigat_slot: &mut f64,
        var_vbiinvbot_slot: &mut f64,
        var_vbiinvgat_slot: &mut f64,
        var_vbiinvsti_slot: &mut f64,
        var_vbirgat2nd_d_slot: &mut f64,
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
        let mut var_cjobot: f64 = *var_cjobot_slot;
        let mut var_cjogat: f64 = *var_cjogat_slot;
        let mut var_cjosti: f64 = *var_cjosti_slot;
        let mut var_delt: f64 = *var_delt_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_deltaebot: f64 = *var_deltaebot_slot;
        let mut var_deltaegat: f64 = *var_deltaegat_slot;
        let mut var_deltaesti: f64 = *var_deltaesti_slot;
        let mut var_deltaphigd: f64 = *var_deltaphigd_slot;
        let mut var_eg: f64 = *var_eg_slot;
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
        let mut var_ftdgat: f64 = *var_ftdgat_slot;
        let mut var_ftdgat2nd: f64 = *var_ftdgat2nd_slot;
        let mut var_ftdsti: f64 = *var_ftdsti_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_idsatbot: f64 = *var_idsatbot_slot;
        let mut var_idsatgat: f64 = *var_idsatgat_slot;
        let mut var_idsatsti: f64 = *var_idsatsti_slot;
        let mut var_inv_phit: f64 = *var_inv_phit_slot;
        let mut var_inv_phita: f64 = *var_inv_phita_slot;
        let mut var_ln_rtn: f64 = *var_ln_rtn_slot;
        let mut var_nt0: f64 = *var_nt0_slot;
        let mut var_one_minus_pgat2nd_d: f64 = *var_one_minus_pgat2nd_d_slot;
        let mut var_one_over_one_minus_pgat2nd_d: f64 = *var_one_over_one_minus_pgat2nd_d_slot;
        let mut var_pgat2nd_d: f64 = *var_pgat2nd_d_slot;
        let mut var_phibfac: f64 = *var_phibfac_slot;
        let mut var_phigdbot: f64 = *var_phigdbot_slot;
        let mut var_phigdgat: f64 = *var_phigdgat_slot;
        let mut var_phigdgat2nd: f64 = *var_phigdgat2nd_slot;
        let mut var_phigdsti: f64 = *var_phigdsti_slot;
        let mut var_phiggat2nd_d: f64 = *var_phiggat2nd_d_slot;
        let mut var_phigrgat2nd_d: f64 = *var_phigrgat2nd_d_slot;
        let mut var_phit: f64 = *var_phit_slot;
        let mut var_phita: f64 = *var_phita_slot;
        let mut var_phitd: f64 = *var_phitd_slot;
        let mut var_phitdinv: f64 = *var_phitdinv_slot;
        let mut var_qpref2bot: f64 = *var_qpref2bot_slot;
        let mut var_qpref2gat: f64 = *var_qpref2gat_slot;
        let mut var_qpref2sti: f64 = *var_qpref2sti_slot;
        let mut var_qprefbot: f64 = *var_qprefbot_slot;
        let mut var_qprefgat: f64 = *var_qprefgat_slot;
        let mut var_qprefsti: f64 = *var_qprefsti_slot;
        let mut var_rta: f64 = *var_rta_slot;
        let mut var_rtn: f64 = *var_rtn_slot;
        let mut var_tka: f64 = *var_tka_slot;
        let mut var_tkd: f64 = *var_tkd_slot;
        let mut var_tkd_1: f64 = *var_tkd_1_slot;
        let mut var_tkd_sq: f64 = *var_tkd_sq_slot;
        let mut var_ubibot: f64 = *var_ubibot_slot;
        let mut var_ubigat: f64 = *var_ubigat_slot;
        let mut var_ubigat2nd: f64 = *var_ubigat2nd_slot;
        let mut var_ubisti: f64 = *var_ubisti_slot;
        let mut var_vbibot: f64 = *var_vbibot_slot;
        let mut var_vbigat: f64 = *var_vbigat_slot;
        let mut var_vbiinvbot: f64 = *var_vbiinvbot_slot;
        let mut var_vbiinvgat: f64 = *var_vbiinvgat_slot;
        let mut var_vbiinvsti: f64 = *var_vbiinvsti_slot;
        let mut var_vbirgat2nd_d: f64 = *var_vbirgat2nd_d_slot;
        let mut var_vbisti: f64 = *var_vbisti_slot;

        let (assign1950_e2420,) = {
    if (var_guard7 != 0.0) {
        let assign1950_e2411: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
        let (assign1950_e2418,) = {
            if (assign1950_e2411 > 0.05) {
                let assign1950_e2416: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
                (assign1950_e2416,)
            } else {
                (0.05,)
            }
        };
        (assign1950_e2418,)
    } else {
        (var_vbirgat2nd_d,)
    }
};
        var_vbirgat2nd_d = assign1950_e2420;

        let (assign1960_e2447,) = {
    if (var_guard7 != 0.0) {
        let assign1960_e2424: f64 = (var_pgatd_i * var_fpgat2d_i);
        let (assign1960_e2431,) = {
            if (assign1960_e2424 > 0.05) {
                let assign1960_e2429: f64 = (var_pgatd_i * var_fpgat2d_i);
                (assign1960_e2429,)
            } else {
                (0.05,)
            }
        };
        let (assign1960_e2445,) = {
            if (assign1960_e2431 < 0.95) {
                let assign1960_e2436: f64 = (var_pgatd_i * var_fpgat2d_i);
                let (assign1960_e2443,) = {
                    if (assign1960_e2436 > 0.05) {
                        let assign1960_e2441: f64 = (var_pgatd_i * var_fpgat2d_i);
                        (assign1960_e2441,)
                    } else {
                        (0.05,)
                    }
                };
                (assign1960_e2443,)
            } else {
                (0.95,)
            }
        };
        (assign1960_e2445,)
    } else {
        (var_pgat2nd_d,)
    }
};
        var_pgat2nd_d = assign1960_e2447;

        let (assign1970_e2453,) = {
    if (var_guard7 != 0.0) {
        let assign1970_e2451: f64 = (var_phiggatd_i * var_fphiggat2d_i);
        (assign1970_e2451,)
    } else {
        (var_phiggat2nd_d,)
    }
};
        var_phiggat2nd_d = assign1970_e2453;

        let (assign1980_e2459,) = {
    if (var_guard7 != 0.0) {
        let assign1980_e2457: f64 = (var_phiggat2nd_d + var_deltaphigr);
        (assign1980_e2457,)
    } else {
        (var_phigrgat2nd_d,)
    }
};
        var_phigrgat2nd_d = assign1980_e2459;

        let (assign1990_e2465,) = {
    if (var_guard7 != 0.0) {
        let assign1990_e2463: f64 = (1.0 - var_pgat2nd_d);
        (assign1990_e2463,)
    } else {
        (var_one_minus_pgat2nd_d,)
    }
};
        var_one_minus_pgat2nd_d = assign1990_e2465;

        let (assign2000_e2471,) = {
    if (var_guard7 != 0.0) {
        let assign2000_e2469: f64 = (1.0 / var_one_minus_pgat2nd_d);
        (assign2000_e2469,)
    } else {
        (var_one_over_one_minus_pgat2nd_d,)
    }
};
        var_one_over_one_minus_pgat2nd_d = assign2000_e2471;

        let assign2050_e2493: f64 = ctx_temp;
        let assign2050_e2495: f64 = (assign2050_e2493 + p.p55);
        let assign2050_e2497: f64 = (assign2050_e2495 + p.p35);
        var_tka = assign2050_e2497;

        let assign2060_e2500: f64 = (var_tka / var_tkr);
        var_rta = assign2060_e2500;

        let assign2070_e2503: f64 = (var_tka - var_tkr);
        var_delta = assign2070_e2503;

        let assign2080_e2506: f64 = (var_tka * 1.3806505e-23);
        let assign2080_e2508: f64 = (assign2080_e2506 / 1.6021918e-19);
        var_phita = assign2080_e2508;

        let assign2090_e2511: f64 = (1.0 / var_phita);
        var_inv_phita = assign2090_e2511;

        var_tkd = var_tka;

        let assign2110_e2515: f64 = (var_tkd * var_tkd);
        var_tkd_sq = assign2110_e2515;

        let assign2120_e2518: f64 = (var_tkd - var_tkr);
        var_delt = assign2120_e2518;

        let assign2130_e2521: f64 = (var_tkr / var_tkd);
        var_rtn = assign2130_e2521;

        let assign2140_e2523: f64 = (var_rtn).ln();
        var_ln_rtn = assign2140_e2523;

        let assign2150_e2526: f64 = (var_tkd * 1.3806505e-23);
        let assign2150_e2528: f64 = (assign2150_e2526 / 1.6021918e-19);
        var_phit = assign2150_e2528;

        let assign2160_e2531: f64 = (1.0 / var_phit);
        var_inv_phit = assign2160_e2531;

        let assign2170_e2535: f64 = (9.025e-5 * var_tkd);
        let assign2170_e2536: f64 = (1.179 - assign2170_e2535);
        let assign2170_e2539: f64 = (3.05e-7 * var_tkd_sq);
        let assign2170_e2540: f64 = (assign2170_e2536 - assign2170_e2539);
        var_eg = assign2170_e2540;

        let assign2180_e2544: f64 = (0.00045 * var_tkd);
        let assign2180_e2545: f64 = (1.045 + assign2180_e2544);
        let assign2180_e2549: f64 = (0.0014 * var_tkd);
        let assign2180_e2550: f64 = (0.523 + assign2180_e2549);
        let assign2180_e2553: f64 = (1.48e-6 * var_tkd_sq);
        let assign2180_e2554: f64 = (assign2180_e2550 - assign2180_e2553);
        let assign2180_e2555: f64 = (assign2180_e2545 * assign2180_e2554);
        let assign2180_e2557: f64 = (assign2180_e2555 * var_tkd_sq);
        let assign2180_e2559: f64 = (assign2180_e2557 / 90000.0);
        var_phibfac = assign2180_e2559;

        let (assign2190_e2565,) = {
    if (var_phibfac > 0.001) {
        (var_phibfac,)
    } else {
        (0.001,)
    }
};
        var_phibfac = assign2190_e2565;

        let assign2200_e2568: f64 = (4.0 * 1.3806505e-23);
        let assign2200_e2570: f64 = (assign2200_e2568 * var_tkd);
        var_nt0 = assign2200_e2570;

        let assign2210_e2571: f64 = ctx_temp;
        let assign2210_e2573: f64 = (assign2210_e2571 + p.p55);
        let assign2210_e2575: f64 = (assign2210_e2573 + p.p35);
        let assign2210_e2578: f64 = (-250.0);
        let assign2210_e2579: f64 = (273.15 + assign2210_e2578);
        let assign2210_e2580: f64 = (assign2210_e2575).max(assign2210_e2579);
        var_tkd_1 = assign2210_e2580;

        let assign2220_e2583: f64 = (var_tkd_1 / var_tkr_1);
        var_auxt = assign2220_e2583;

        let assign2230_e2586: f64 = (var_kbol_over_qele * var_tkd_1);
        var_phitd = assign2230_e2586;

        let assign2240_e2589: f64 = (1.0 / var_phitd);
        var_phitdinv = assign2240_e2589;

        let assign2250_e2592: f64 = (0.000702 * var_tkd_1);
        let assign2250_e2594: f64 = (assign2250_e2592 * var_tkd_1);
        let assign2250_e2595: f64 = (-assign2250_e2594);
        let assign2250_e2598: f64 = (1108.0 + var_tkd_1);
        let assign2250_e2599: f64 = (assign2250_e2595 / assign2250_e2598);
        var_deltaphigd = assign2250_e2599;

        let assign2260_e2602: f64 = (p.p827 + var_deltaphigd);
        var_phigdbot = assign2260_e2602;

        let assign2270_e2605: f64 = (p.p828 + var_deltaphigd);
        var_phigdsti = assign2270_e2605;

        let assign2280_e2608: f64 = (p.p829 + var_deltaphigd);
        var_phigdgat = assign2280_e2608;

        let assign2290_e2611: f64 = (var_auxt).powf(1.5);
        let assign2290_e2615: f64 = (var_phigrbot * var_phitrinv);
        let assign2290_e2618: f64 = (var_phigdbot * var_phitdinv);
        let assign2290_e2619: f64 = (assign2290_e2615 - assign2290_e2618);
        let assign2290_e2620: f64 = (0.5 * assign2290_e2619);
        let assign2290_e2621: f64 = (assign2290_e2620).exp();
        let assign2290_e2622: f64 = (assign2290_e2611 * assign2290_e2621);
        var_ftdbot = assign2290_e2622;

        let assign2300_e2625: f64 = (var_auxt).powf(1.5);
        let assign2300_e2629: f64 = (var_phigrsti * var_phitrinv);
        let assign2300_e2632: f64 = (var_phigdsti * var_phitdinv);
        let assign2300_e2633: f64 = (assign2300_e2629 - assign2300_e2632);
        let assign2300_e2634: f64 = (0.5 * assign2300_e2633);
        let assign2300_e2635: f64 = (assign2300_e2634).exp();
        let assign2300_e2636: f64 = (assign2300_e2625 * assign2300_e2635);
        var_ftdsti = assign2300_e2636;

        let assign2310_e2639: f64 = (var_auxt).powf(1.5);
        let assign2310_e2643: f64 = (var_phigrgat * var_phitrinv);
        let assign2310_e2646: f64 = (var_phigdgat * var_phitdinv);
        let assign2310_e2647: f64 = (assign2310_e2643 - assign2310_e2646);
        let assign2310_e2648: f64 = (0.5 * assign2310_e2647);
        let assign2310_e2649: f64 = (assign2310_e2648).exp();
        let assign2310_e2650: f64 = (assign2310_e2639 * assign2310_e2649);
        var_ftdgat = assign2310_e2650;

        let assign2320_e2653: f64 = (p.p830 * var_ftdbot);
        let assign2320_e2655: f64 = (assign2320_e2653 * var_ftdbot);
        var_idsatbot = assign2320_e2655;

        let assign2330_e2658: f64 = (p.p831 * var_ftdsti);
        let assign2330_e2660: f64 = (assign2330_e2658 * var_ftdsti);
        var_idsatsti = assign2330_e2660;

        let assign2340_e2663: f64 = (p.p832 * var_ftdgat);
        let assign2340_e2665: f64 = (assign2340_e2663 * var_ftdgat);
        var_idsatgat = assign2340_e2665;

        let assign2350_e2668: f64 = (p.p821 * var_auxt);
        let assign2350_e2671: f64 = (2.0 * var_phitd);
        let assign2350_e2673: f64 = (var_ftdbot).ln();
        let assign2350_e2674: f64 = (assign2350_e2671 * assign2350_e2673);
        let assign2350_e2675: f64 = (assign2350_e2668 - assign2350_e2674);
        var_ubibot = assign2350_e2675;

        let assign2360_e2678: f64 = (p.p822 * var_auxt);
        let assign2360_e2681: f64 = (2.0 * var_phitd);
        let assign2360_e2683: f64 = (var_ftdsti).ln();
        let assign2360_e2684: f64 = (assign2360_e2681 * assign2360_e2683);
        let assign2360_e2685: f64 = (assign2360_e2678 - assign2360_e2684);
        var_ubisti = assign2360_e2685;

        let assign2370_e2688: f64 = (p.p823 * var_auxt);
        let assign2370_e2691: f64 = (2.0 * var_phitd);
        let assign2370_e2693: f64 = (var_ftdgat).ln();
        let assign2370_e2694: f64 = (assign2370_e2691 * assign2370_e2693);
        let assign2370_e2695: f64 = (assign2370_e2688 - assign2370_e2694);
        var_ubigat = assign2370_e2695;

        let assign2380_e2701: f64 = (0.05 - var_ubibot);
        let assign2380_e2703: f64 = (assign2380_e2701 * var_phitdinv);
        let assign2380_e2704: f64 = (assign2380_e2703).exp();
        let assign2380_e2705: f64 = (1.0 + assign2380_e2704);
        let assign2380_e2706: f64 = (assign2380_e2705).ln();
        let assign2380_e2707: f64 = (var_phitd * assign2380_e2706);
        let assign2380_e2708: f64 = (var_ubibot + assign2380_e2707);
        var_vbibot = assign2380_e2708;

        let assign2390_e2714: f64 = (0.05 - var_ubisti);
        let assign2390_e2716: f64 = (assign2390_e2714 * var_phitdinv);
        let assign2390_e2717: f64 = (assign2390_e2716).exp();
        let assign2390_e2718: f64 = (1.0 + assign2390_e2717);
        let assign2390_e2719: f64 = (assign2390_e2718).ln();
        let assign2390_e2720: f64 = (var_phitd * assign2390_e2719);
        let assign2390_e2721: f64 = (var_ubisti + assign2390_e2720);
        var_vbisti = assign2390_e2721;

        let assign2400_e2727: f64 = (0.05 - var_ubigat);
        let assign2400_e2729: f64 = (assign2400_e2727 * var_phitdinv);
        let assign2400_e2730: f64 = (assign2400_e2729).exp();
        let assign2400_e2731: f64 = (1.0 + assign2400_e2730);
        let assign2400_e2732: f64 = (assign2400_e2731).ln();
        let assign2400_e2733: f64 = (var_phitd * assign2400_e2732);
        let assign2400_e2734: f64 = (var_ubigat + assign2400_e2733);
        var_vbigat = assign2400_e2734;

        let assign2410_e2737: f64 = (1.0 / var_vbibot);
        var_vbiinvbot = assign2410_e2737;

        let assign2420_e2740: f64 = (1.0 / var_vbisti);
        var_vbiinvsti = assign2420_e2740;

        let assign2430_e2743: f64 = (1.0 / var_vbigat);
        var_vbiinvgat = assign2430_e2743;

        let assign2440_e2747: f64 = (p.p821 * var_vbiinvbot);
        let assign2440_e2749: f64 = (assign2440_e2747).powf(p.p824);
        let assign2440_e2750: f64 = (p.p818 * assign2440_e2749);
        var_cjobot = assign2440_e2750;

        let assign2450_e2754: f64 = (p.p822 * var_vbiinvsti);
        let assign2450_e2756: f64 = (assign2450_e2754).powf(p.p825);
        let assign2450_e2757: f64 = (p.p819 * assign2450_e2756);
        var_cjosti = assign2450_e2757;

        let assign2460_e2761: f64 = (p.p823 * var_vbiinvgat);
        let assign2460_e2763: f64 = (assign2460_e2761).powf(p.p826);
        let assign2460_e2764: f64 = (p.p820 * assign2460_e2763);
        var_cjogat = assign2460_e2764;

        let assign2470_e2767: f64 = (var_cjobot * var_vbibot);
        let assign2470_e2769: f64 = (assign2470_e2767 * var_one_over_one_minus_pbot);
        var_qprefbot = assign2470_e2769;

        let assign2480_e2772: f64 = (var_cjosti * var_vbisti);
        let assign2480_e2774: f64 = (assign2480_e2772 * var_one_over_one_minus_psti);
        var_qprefsti = assign2480_e2774;

        let assign2490_e2777: f64 = (var_cjogat * var_vbigat);
        let assign2490_e2779: f64 = (assign2490_e2777 * var_one_over_one_minus_pgat);
        var_qprefgat = assign2490_e2779;

        let assign2500_e2782: f64 = (2.0 * var_cjobot);
        var_qpref2bot = assign2500_e2782;

        let assign2510_e2785: f64 = (2.0 * var_cjosti);
        var_qpref2sti = assign2510_e2785;

        let assign2520_e2788: f64 = (2.0 * var_cjogat);
        var_qpref2gat = assign2520_e2788;

        let assign2530_e2791: f64 = (0.5 * var_phigdbot);
        let assign2530_e2793: f64 = (assign2530_e2791).max(var_phitd);
        var_deltaebot = assign2530_e2793;

        let assign2540_e2796: f64 = (0.5 * var_phigdsti);
        let assign2540_e2798: f64 = (assign2540_e2796).max(var_phitd);
        var_deltaesti = assign2540_e2798;

        let assign2550_e2801: f64 = (0.5 * var_phigdgat);
        let assign2550_e2803: f64 = (assign2550_e2801).max(var_phitd);
        var_deltaegat = assign2550_e2803;

        let assign2560_e2806: f64 = (var_deltaebot * var_phitdinv);
        var_atatbot = assign2560_e2806;

        let assign2570_e2809: f64 = (var_deltaesti * var_phitdinv);
        var_atatsti = assign2570_e2809;

        let assign2580_e2812: f64 = (var_deltaegat * var_phitdinv);
        var_atatgat = assign2580_e2812;

        let assign2590_e2815: f64 = (32.0 * p.p841);
        let assign2590_e2817: f64 = (assign2590_e2815 * 9.1093826e-31);
        let assign2590_e2819: f64 = (assign2590_e2817 * 1.6021918e-19);
        let assign2590_e2822: f64 = (var_deltaebot * var_deltaebot);
        let assign2590_e2824: f64 = (assign2590_e2822 * var_deltaebot);
        let assign2590_e2825: f64 = (assign2590_e2819 * assign2590_e2824);
        let assign2590_e2826: f64 = (assign2590_e2825).sqrt();
        let assign2590_e2829: f64 = (3.0 * 1.05457168e-34);
        let assign2590_e2830: f64 = (assign2590_e2826 / assign2590_e2829);
        var_btatpartbot = assign2590_e2830;

        let assign2600_e2833: f64 = (32.0 * p.p842);
        let assign2600_e2835: f64 = (assign2600_e2833 * 9.1093826e-31);
        let assign2600_e2837: f64 = (assign2600_e2835 * 1.6021918e-19);
        let assign2600_e2840: f64 = (var_deltaesti * var_deltaesti);
        let assign2600_e2842: f64 = (assign2600_e2840 * var_deltaesti);
        let assign2600_e2843: f64 = (assign2600_e2837 * assign2600_e2842);
        let assign2600_e2844: f64 = (assign2600_e2843).sqrt();
        let assign2600_e2847: f64 = (3.0 * 1.05457168e-34);
        let assign2600_e2848: f64 = (assign2600_e2844 / assign2600_e2847);
        var_btatpartsti = assign2600_e2848;

        let assign2610_e2851: f64 = (32.0 * p.p843);
        let assign2610_e2853: f64 = (assign2610_e2851 * 9.1093826e-31);
        let assign2610_e2855: f64 = (assign2610_e2853 * 1.6021918e-19);
        let assign2610_e2858: f64 = (var_deltaegat * var_deltaegat);
        let assign2610_e2860: f64 = (assign2610_e2858 * var_deltaegat);
        let assign2610_e2861: f64 = (assign2610_e2855 * assign2610_e2860);
        let assign2610_e2862: f64 = (assign2610_e2861).sqrt();
        let assign2610_e2865: f64 = (3.0 * 1.05457168e-34);
        let assign2610_e2866: f64 = (assign2610_e2862 / assign2610_e2865);
        var_btatpartgat = assign2610_e2866;

        let assign2620_e2872: f64 = (var_tkd_1 - var_tkr_1);
        let assign2620_e2873: f64 = (p.p850 * assign2620_e2872);
        let assign2620_e2874: f64 = (1.0 + assign2620_e2873);
        let assign2620_e2875: f64 = (p.p847 * assign2620_e2874);
        var_fbbtbot = assign2620_e2875;

        let assign2630_e2881: f64 = (var_tkd_1 - var_tkr_1);
        let assign2630_e2882: f64 = (p.p851 * assign2630_e2881);
        let assign2630_e2883: f64 = (1.0 + assign2630_e2882);
        let assign2630_e2884: f64 = (p.p848 * assign2630_e2883);
        var_fbbtsti = assign2630_e2884;

        let assign2640_e2890: f64 = (var_tkd_1 - var_tkr_1);
        let assign2640_e2891: f64 = (p.p852 * assign2640_e2890);
        let assign2640_e2892: f64 = (1.0 + assign2640_e2891);
        let assign2640_e2893: f64 = (p.p849 * assign2640_e2892);
        var_fbbtgat = assign2640_e2893;
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
        var_fbbtgat_db0 = 0.0;
        var_fbbtgat_db1 = 0.0;
        var_fbbtgat_db2 = 0.0;
        var_fbbtgat_db3 = 0.0;
        var_fbbtgat_db4 = 0.0;
        var_fbbtgat_db5 = 0.0;
        var_fbbtgat_db6 = 0.0;

        let (assign2650_e2899,) = {
    if (var_fbbtbot > 0.0) {
        (var_fbbtbot,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot = assign2650_e2899;

        let (assign2660_e2905,) = {
    if (var_fbbtsti > 0.0) {
        (var_fbbtsti,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti = assign2660_e2905;

        let (assign2670_e2911, assign2670_e2911_d_n0, assign2670_e2911_d_n1, assign2670_e2911_d_n2, assign2670_e2911_d_n3, assign2670_e2911_d_n4, assign2670_e2911_d_n5, assign2670_e2911_d_n6, assign2670_e2911_d_n7, assign2670_e2911_d_n8, assign2670_e2911_d_n9, assign2670_e2911_d_n10, assign2670_e2911_d_n11, assign2670_e2911_d_b0, assign2670_e2911_d_b1, assign2670_e2911_d_b2, assign2670_e2911_d_b3, assign2670_e2911_d_b4, assign2670_e2911_d_b5, assign2670_e2911_d_b6,) = {
    if (var_fbbtgat > 0.0) {
        (var_fbbtgat, var_fbbtgat_dn0, var_fbbtgat_dn1, var_fbbtgat_dn2, var_fbbtgat_dn3, var_fbbtgat_dn4, var_fbbtgat_dn5, var_fbbtgat_dn6, var_fbbtgat_dn7, var_fbbtgat_dn8, var_fbbtgat_dn9, var_fbbtgat_dn10, var_fbbtgat_dn11, var_fbbtgat_db0, var_fbbtgat_db1, var_fbbtgat_db2, var_fbbtgat_db3, var_fbbtgat_db4, var_fbbtgat_db5, var_fbbtgat_db6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat = assign2670_e2911;
        var_fbbtgat_dn0 = assign2670_e2911_d_n0;
        var_fbbtgat_dn1 = assign2670_e2911_d_n1;
        var_fbbtgat_dn2 = assign2670_e2911_d_n2;
        var_fbbtgat_dn3 = assign2670_e2911_d_n3;
        var_fbbtgat_dn4 = assign2670_e2911_d_n4;
        var_fbbtgat_dn5 = assign2670_e2911_d_n5;
        var_fbbtgat_dn6 = assign2670_e2911_d_n6;
        var_fbbtgat_dn7 = assign2670_e2911_d_n7;
        var_fbbtgat_dn8 = assign2670_e2911_d_n8;
        var_fbbtgat_dn9 = assign2670_e2911_d_n9;
        var_fbbtgat_dn10 = assign2670_e2911_d_n10;
        var_fbbtgat_dn11 = assign2670_e2911_d_n11;
        var_fbbtgat_db0 = assign2670_e2911_d_b0;
        var_fbbtgat_db1 = assign2670_e2911_d_b1;
        var_fbbtgat_db2 = assign2670_e2911_d_b2;
        var_fbbtgat_db3 = assign2670_e2911_d_b3;
        var_fbbtgat_db4 = assign2670_e2911_d_b4;
        var_fbbtgat_db5 = assign2670_e2911_d_b5;
        var_fbbtgat_db6 = assign2670_e2911_d_b6;

        let assign2680_e2914: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard27 = assign2680_e2914;

        let (assign2690_e2920,) = {
    if (var_guard27 != 0.0) {
        let assign2690_e2918: f64 = (var_phiggat2nd + var_deltaphigd);
        (assign2690_e2918,)
    } else {
        (var_phigdgat2nd,)
    }
};
        var_phigdgat2nd = assign2690_e2920;

        let (assign2700_e2937,) = {
    if (var_guard27 != 0.0) {
        let assign2700_e2924: f64 = (var_auxt).powf(1.5);
        let assign2700_e2928: f64 = (var_phigrgat2nd * var_phitrinv);
        let assign2700_e2931: f64 = (var_phigdgat2nd * var_phitdinv);
        let assign2700_e2932: f64 = (assign2700_e2928 - assign2700_e2931);
        let assign2700_e2933: f64 = (0.5 * assign2700_e2932);
        let assign2700_e2934: f64 = (assign2700_e2933).exp();
        let assign2700_e2935: f64 = (assign2700_e2924 * assign2700_e2934);
        (assign2700_e2935,)
    } else {
        (var_ftdgat2nd,)
    }
};
        var_ftdgat2nd = assign2700_e2937;

        let (assign2710_e2950,) = {
    if (var_guard27 != 0.0) {
        let assign2710_e2941: f64 = (var_vbirgat2nd * var_auxt);
        let assign2710_e2944: f64 = (2.0 * var_phitd);
        let assign2710_e2946: f64 = (var_ftdgat2nd).ln();
        let assign2710_e2947: f64 = (assign2710_e2944 * assign2710_e2946);
        let assign2710_e2948: f64 = (assign2710_e2941 - assign2710_e2947);
        (assign2710_e2948,)
    } else {
        (var_ubigat2nd,)
    }
};
        var_ubigat2nd = assign2710_e2950;

        *var_atatbot_slot = var_atatbot;
        *var_atatgat_slot = var_atatgat;
        *var_atatsti_slot = var_atatsti;
        *var_auxt_slot = var_auxt;
        *var_btatpartbot_slot = var_btatpartbot;
        *var_btatpartgat_slot = var_btatpartgat;
        *var_btatpartsti_slot = var_btatpartsti;
        *var_cjobot_slot = var_cjobot;
        *var_cjogat_slot = var_cjogat;
        *var_cjosti_slot = var_cjosti;
        *var_delt_slot = var_delt;
        *var_delta_slot = var_delta;
        *var_deltaebot_slot = var_deltaebot;
        *var_deltaegat_slot = var_deltaegat;
        *var_deltaesti_slot = var_deltaesti;
        *var_deltaphigd_slot = var_deltaphigd;
        *var_eg_slot = var_eg;
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
        *var_ftdgat_slot = var_ftdgat;
        *var_ftdgat2nd_slot = var_ftdgat2nd;
        *var_ftdsti_slot = var_ftdsti;
        *var_guard27_slot = var_guard27;
        *var_idsatbot_slot = var_idsatbot;
        *var_idsatgat_slot = var_idsatgat;
        *var_idsatsti_slot = var_idsatsti;
        *var_inv_phit_slot = var_inv_phit;
        *var_inv_phita_slot = var_inv_phita;
        *var_ln_rtn_slot = var_ln_rtn;
        *var_nt0_slot = var_nt0;
        *var_one_minus_pgat2nd_d_slot = var_one_minus_pgat2nd_d;
        *var_one_over_one_minus_pgat2nd_d_slot = var_one_over_one_minus_pgat2nd_d;
        *var_pgat2nd_d_slot = var_pgat2nd_d;
        *var_phibfac_slot = var_phibfac;
        *var_phigdbot_slot = var_phigdbot;
        *var_phigdgat_slot = var_phigdgat;
        *var_phigdgat2nd_slot = var_phigdgat2nd;
        *var_phigdsti_slot = var_phigdsti;
        *var_phiggat2nd_d_slot = var_phiggat2nd_d;
        *var_phigrgat2nd_d_slot = var_phigrgat2nd_d;
        *var_phit_slot = var_phit;
        *var_phita_slot = var_phita;
        *var_phitd_slot = var_phitd;
        *var_phitdinv_slot = var_phitdinv;
        *var_qpref2bot_slot = var_qpref2bot;
        *var_qpref2gat_slot = var_qpref2gat;
        *var_qpref2sti_slot = var_qpref2sti;
        *var_qprefbot_slot = var_qprefbot;
        *var_qprefgat_slot = var_qprefgat;
        *var_qprefsti_slot = var_qprefsti;
        *var_rta_slot = var_rta;
        *var_rtn_slot = var_rtn;
        *var_tka_slot = var_tka;
        *var_tkd_slot = var_tkd;
        *var_tkd_1_slot = var_tkd_1;
        *var_tkd_sq_slot = var_tkd_sq;
        *var_ubibot_slot = var_ubibot;
        *var_ubigat_slot = var_ubigat;
        *var_ubigat2nd_slot = var_ubigat2nd;
        *var_ubisti_slot = var_ubisti;
        *var_vbibot_slot = var_vbibot;
        *var_vbigat_slot = var_vbigat;
        *var_vbiinvbot_slot = var_vbiinvbot;
        *var_vbiinvgat_slot = var_vbiinvgat;
        *var_vbiinvsti_slot = var_vbiinvsti;
        *var_vbirgat2nd_d_slot = var_vbirgat2nd_d;
        *var_vbisti_slot = var_vbisti;
    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        var_auxt: f64,
        var_cjorbotd_i: f64,
        var_cjorgat2nd: f64,
        var_cjorgat2nd_d: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigd: f64,
        var_fbbtrbotd_i: f64,
        var_fbbtrgatd_i: f64,
        var_fbbtrstid_i: f64,
        var_guard27: f64,
        var_idsatrbotd_i: f64,
        var_idsatrgatd_i: f64,
        var_idsatrstid_i: f64,
        var_mefftatbotd_i: f64,
        var_mefftatgatd_i: f64,
        var_mefftatstid_i: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_one_over_one_minus_pgat2nd: f64,
        var_one_over_one_minus_pgat2nd_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbotd_i: f64,
        var_pgat2nd: f64,
        var_pgat2nd_d: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggat2nd_d: f64,
        var_phiggatd_i: f64,
        var_phigrbot_d: f64,
        var_phigrgat2nd_d: f64,
        var_phigrgat_d: f64,
        var_phigrsti_d: f64,
        var_phigstid_i: f64,
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
        var_ubigat2nd: f64,
        var_vbirbotd_i: f64,
        var_vbirgat2nd: f64,
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
        var_cjobot_d_slot: &mut f64,
        var_cjogat2nd_slot: &mut f64,
        var_cjogat2nd_d_slot: &mut f64,
        var_cjogat_d_slot: &mut f64,
        var_cjosti_d_slot: &mut f64,
        var_deltaebot_d_slot: &mut f64,
        var_deltaegat_d_slot: &mut f64,
        var_deltaesti_d_slot: &mut f64,
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
        var_fbbtgat_d_dn2_slot: &mut f64,
        var_fbbtgat_d_dn3_slot: &mut f64,
        var_fbbtgat_d_dn4_slot: &mut f64,
        var_fbbtgat_d_dn5_slot: &mut f64,
        var_fbbtgat_d_dn6_slot: &mut f64,
        var_fbbtgat_d_dn7_slot: &mut f64,
        var_fbbtgat_d_dn8_slot: &mut f64,
        var_fbbtgat_d_dn9_slot: &mut f64,
        var_fbbtsti_d_slot: &mut f64,
        var_ftdbot_d_slot: &mut f64,
        var_ftdgat2nd_d_slot: &mut f64,
        var_ftdgat_d_slot: &mut f64,
        var_ftdsti_d_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_idsatbot_d_slot: &mut f64,
        var_idsatgat_d_slot: &mut f64,
        var_idsatsti_d_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_jw_i_slot: &mut f64,
        var_l_i_slot: &mut f64,
        var_le_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_pd_i_slot: &mut f64,
        var_phigdbot_d_slot: &mut f64,
        var_phigdgat2nd_d_slot: &mut f64,
        var_phigdgat_d_slot: &mut f64,
        var_phigdsti_d_slot: &mut f64,
        var_ps_i_slot: &mut f64,
        var_qpref2bot_d_slot: &mut f64,
        var_qpref2gat2nd_slot: &mut f64,
        var_qpref2gat2nd_d_slot: &mut f64,
        var_qpref2gat_d_slot: &mut f64,
        var_qpref2sti_d_slot: &mut f64,
        var_qprefbot_d_slot: &mut f64,
        var_qprefgat2nd_slot: &mut f64,
        var_qprefgat2nd_d_slot: &mut f64,
        var_qprefgat_d_slot: &mut f64,
        var_qprefsti_d_slot: &mut f64,
        var_sa_i_slot: &mut f64,
        var_sb_i_slot: &mut f64,
        var_sc_i_slot: &mut f64,
        var_sd_i_slot: &mut f64,
        var_ubibot_d_slot: &mut f64,
        var_ubigat2nd_d_slot: &mut f64,
        var_ubigat_d_slot: &mut f64,
        var_ubisti_d_slot: &mut f64,
        var_vbibot_d_slot: &mut f64,
        var_vbigat2nd_slot: &mut f64,
        var_vbigat2nd_d_slot: &mut f64,
        var_vbigat_d_slot: &mut f64,
        var_vbiinvbot_d_slot: &mut f64,
        var_vbiinvgat2nd_slot: &mut f64,
        var_vbiinvgat2nd_d_slot: &mut f64,
        var_vbiinvgat_d_slot: &mut f64,
        var_vbiinvsti_d_slot: &mut f64,
        var_vbisti_d_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_we_slot: &mut f64,
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
        let mut var_cjobot_d: f64 = *var_cjobot_d_slot;
        let mut var_cjogat2nd: f64 = *var_cjogat2nd_slot;
        let mut var_cjogat2nd_d: f64 = *var_cjogat2nd_d_slot;
        let mut var_cjogat_d: f64 = *var_cjogat_d_slot;
        let mut var_cjosti_d: f64 = *var_cjosti_d_slot;
        let mut var_deltaebot_d: f64 = *var_deltaebot_d_slot;
        let mut var_deltaegat_d: f64 = *var_deltaegat_d_slot;
        let mut var_deltaesti_d: f64 = *var_deltaesti_d_slot;
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
        let mut var_fbbtgat_d_dn2: f64 = *var_fbbtgat_d_dn2_slot;
        let mut var_fbbtgat_d_dn3: f64 = *var_fbbtgat_d_dn3_slot;
        let mut var_fbbtgat_d_dn4: f64 = *var_fbbtgat_d_dn4_slot;
        let mut var_fbbtgat_d_dn5: f64 = *var_fbbtgat_d_dn5_slot;
        let mut var_fbbtgat_d_dn6: f64 = *var_fbbtgat_d_dn6_slot;
        let mut var_fbbtgat_d_dn7: f64 = *var_fbbtgat_d_dn7_slot;
        let mut var_fbbtgat_d_dn8: f64 = *var_fbbtgat_d_dn8_slot;
        let mut var_fbbtgat_d_dn9: f64 = *var_fbbtgat_d_dn9_slot;
        let mut var_fbbtsti_d: f64 = *var_fbbtsti_d_slot;
        let mut var_ftdbot_d: f64 = *var_ftdbot_d_slot;
        let mut var_ftdgat2nd_d: f64 = *var_ftdgat2nd_d_slot;
        let mut var_ftdgat_d: f64 = *var_ftdgat_d_slot;
        let mut var_ftdsti_d: f64 = *var_ftdsti_d_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_idsatbot_d: f64 = *var_idsatbot_d_slot;
        let mut var_idsatgat_d: f64 = *var_idsatgat_d_slot;
        let mut var_idsatsti_d: f64 = *var_idsatsti_d_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_jw_i: f64 = *var_jw_i_slot;
        let mut var_l_i: f64 = *var_l_i_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_pd_i: f64 = *var_pd_i_slot;
        let mut var_phigdbot_d: f64 = *var_phigdbot_d_slot;
        let mut var_phigdgat2nd_d: f64 = *var_phigdgat2nd_d_slot;
        let mut var_phigdgat_d: f64 = *var_phigdgat_d_slot;
        let mut var_phigdsti_d: f64 = *var_phigdsti_d_slot;
        let mut var_ps_i: f64 = *var_ps_i_slot;
        let mut var_qpref2bot_d: f64 = *var_qpref2bot_d_slot;
        let mut var_qpref2gat2nd: f64 = *var_qpref2gat2nd_slot;
        let mut var_qpref2gat2nd_d: f64 = *var_qpref2gat2nd_d_slot;
        let mut var_qpref2gat_d: f64 = *var_qpref2gat_d_slot;
        let mut var_qpref2sti_d: f64 = *var_qpref2sti_d_slot;
        let mut var_qprefbot_d: f64 = *var_qprefbot_d_slot;
        let mut var_qprefgat2nd: f64 = *var_qprefgat2nd_slot;
        let mut var_qprefgat2nd_d: f64 = *var_qprefgat2nd_d_slot;
        let mut var_qprefgat_d: f64 = *var_qprefgat_d_slot;
        let mut var_qprefsti_d: f64 = *var_qprefsti_d_slot;
        let mut var_sa_i: f64 = *var_sa_i_slot;
        let mut var_sb_i: f64 = *var_sb_i_slot;
        let mut var_sc_i: f64 = *var_sc_i_slot;
        let mut var_sd_i: f64 = *var_sd_i_slot;
        let mut var_ubibot_d: f64 = *var_ubibot_d_slot;
        let mut var_ubigat2nd_d: f64 = *var_ubigat2nd_d_slot;
        let mut var_ubigat_d: f64 = *var_ubigat_d_slot;
        let mut var_ubisti_d: f64 = *var_ubisti_d_slot;
        let mut var_vbibot_d: f64 = *var_vbibot_d_slot;
        let mut var_vbigat2nd: f64 = *var_vbigat2nd_slot;
        let mut var_vbigat2nd_d: f64 = *var_vbigat2nd_d_slot;
        let mut var_vbigat_d: f64 = *var_vbigat_d_slot;
        let mut var_vbiinvbot_d: f64 = *var_vbiinvbot_d_slot;
        let mut var_vbiinvgat2nd: f64 = *var_vbiinvgat2nd_slot;
        let mut var_vbiinvgat2nd_d: f64 = *var_vbiinvgat2nd_d_slot;
        let mut var_vbiinvgat_d: f64 = *var_vbiinvgat_d_slot;
        let mut var_vbiinvsti_d: f64 = *var_vbiinvsti_d_slot;
        let mut var_vbisti_d: f64 = *var_vbisti_d_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_xgw_i: f64 = *var_xgw_i_slot;

        let (assign2720_e2966,) = {
    if (var_guard27 != 0.0) {
        let assign2720_e2957: f64 = (0.05 - var_ubigat2nd);
        let assign2720_e2959: f64 = (assign2720_e2957 * var_phitdinv);
        let assign2720_e2960: f64 = (assign2720_e2959).exp();
        let assign2720_e2961: f64 = (1.0 + assign2720_e2960);
        let assign2720_e2962: f64 = (assign2720_e2961).ln();
        let assign2720_e2963: f64 = (var_phitd * assign2720_e2962);
        let assign2720_e2964: f64 = (var_ubigat2nd + assign2720_e2963);
        (assign2720_e2964,)
    } else {
        (var_vbigat2nd,)
    }
};
        var_vbigat2nd = assign2720_e2966;

        let (assign2730_e2972,) = {
    if (var_guard27 != 0.0) {
        let assign2730_e2970: f64 = (1.0 / var_vbigat2nd);
        (assign2730_e2970,)
    } else {
        (var_vbiinvgat2nd,)
    }
};
        var_vbiinvgat2nd = assign2730_e2972;

        let (assign2740_e2982,) = {
    if (var_guard27 != 0.0) {
        let assign2740_e2977: f64 = (var_vbirgat2nd * var_vbiinvgat2nd);
        let assign2740_e2979: f64 = (assign2740_e2977).powf(var_pgat2nd);
        let assign2740_e2980: f64 = (var_cjorgat2nd * assign2740_e2979);
        (assign2740_e2980,)
    } else {
        (var_cjogat2nd,)
    }
};
        var_cjogat2nd = assign2740_e2982;

        let (assign2750_e2990,) = {
    if (var_guard27 != 0.0) {
        let assign2750_e2986: f64 = (var_cjogat2nd * var_vbigat2nd);
        let assign2750_e2988: f64 = (assign2750_e2986 * var_one_over_one_minus_pgat2nd);
        (assign2750_e2988,)
    } else {
        (var_qprefgat2nd,)
    }
};
        var_qprefgat2nd = assign2750_e2990;

        let (assign2760_e2996,) = {
    if (var_guard27 != 0.0) {
        let assign2760_e2994: f64 = (2.0 * var_cjogat2nd);
        (assign2760_e2994,)
    } else {
        (var_qpref2gat2nd,)
    }
};
        var_qpref2gat2nd = assign2760_e2996;

        let assign2770_e2999: f64 = (var_phigbotd_i + var_deltaphigd);
        var_phigdbot_d = assign2770_e2999;

        let assign2780_e3002: f64 = (var_phigstid_i + var_deltaphigd);
        var_phigdsti_d = assign2780_e3002;

        let assign2790_e3005: f64 = (var_phiggatd_i + var_deltaphigd);
        var_phigdgat_d = assign2790_e3005;

        let assign2800_e3008: f64 = (var_auxt).powf(1.5);
        let assign2800_e3012: f64 = (var_phigrbot_d * var_phitrinv);
        let assign2800_e3015: f64 = (var_phigdbot_d * var_phitdinv);
        let assign2800_e3016: f64 = (assign2800_e3012 - assign2800_e3015);
        let assign2800_e3017: f64 = (0.5 * assign2800_e3016);
        let assign2800_e3018: f64 = (assign2800_e3017).exp();
        let assign2800_e3019: f64 = (assign2800_e3008 * assign2800_e3018);
        var_ftdbot_d = assign2800_e3019;

        let assign2810_e3022: f64 = (var_auxt).powf(1.5);
        let assign2810_e3026: f64 = (var_phigrsti_d * var_phitrinv);
        let assign2810_e3029: f64 = (var_phigdsti_d * var_phitdinv);
        let assign2810_e3030: f64 = (assign2810_e3026 - assign2810_e3029);
        let assign2810_e3031: f64 = (0.5 * assign2810_e3030);
        let assign2810_e3032: f64 = (assign2810_e3031).exp();
        let assign2810_e3033: f64 = (assign2810_e3022 * assign2810_e3032);
        var_ftdsti_d = assign2810_e3033;

        let assign2820_e3036: f64 = (var_auxt).powf(1.5);
        let assign2820_e3040: f64 = (var_phigrgat_d * var_phitrinv);
        let assign2820_e3043: f64 = (var_phigdgat_d * var_phitdinv);
        let assign2820_e3044: f64 = (assign2820_e3040 - assign2820_e3043);
        let assign2820_e3045: f64 = (0.5 * assign2820_e3044);
        let assign2820_e3046: f64 = (assign2820_e3045).exp();
        let assign2820_e3047: f64 = (assign2820_e3036 * assign2820_e3046);
        var_ftdgat_d = assign2820_e3047;

        let assign2830_e3050: f64 = (var_idsatrbotd_i * var_ftdbot_d);
        let assign2830_e3052: f64 = (assign2830_e3050 * var_ftdbot_d);
        var_idsatbot_d = assign2830_e3052;

        let assign2840_e3055: f64 = (var_idsatrstid_i * var_ftdsti_d);
        let assign2840_e3057: f64 = (assign2840_e3055 * var_ftdsti_d);
        var_idsatsti_d = assign2840_e3057;

        let assign2850_e3060: f64 = (var_idsatrgatd_i * var_ftdgat_d);
        let assign2850_e3062: f64 = (assign2850_e3060 * var_ftdgat_d);
        var_idsatgat_d = assign2850_e3062;

        let assign2860_e3065: f64 = (var_vbirbotd_i * var_auxt);
        let assign2860_e3068: f64 = (2.0 * var_phitd);
        let assign2860_e3070: f64 = (var_ftdbot_d).ln();
        let assign2860_e3071: f64 = (assign2860_e3068 * assign2860_e3070);
        let assign2860_e3072: f64 = (assign2860_e3065 - assign2860_e3071);
        var_ubibot_d = assign2860_e3072;

        let assign2870_e3075: f64 = (var_vbirstid_i * var_auxt);
        let assign2870_e3078: f64 = (2.0 * var_phitd);
        let assign2870_e3080: f64 = (var_ftdsti_d).ln();
        let assign2870_e3081: f64 = (assign2870_e3078 * assign2870_e3080);
        let assign2870_e3082: f64 = (assign2870_e3075 - assign2870_e3081);
        var_ubisti_d = assign2870_e3082;

        let assign2880_e3085: f64 = (var_vbirgatd_i * var_auxt);
        let assign2880_e3088: f64 = (2.0 * var_phitd);
        let assign2880_e3090: f64 = (var_ftdgat_d).ln();
        let assign2880_e3091: f64 = (assign2880_e3088 * assign2880_e3090);
        let assign2880_e3092: f64 = (assign2880_e3085 - assign2880_e3091);
        var_ubigat_d = assign2880_e3092;

        let assign2890_e3098: f64 = (0.05 - var_ubibot_d);
        let assign2890_e3100: f64 = (assign2890_e3098 * var_phitdinv);
        let assign2890_e3101: f64 = (assign2890_e3100).exp();
        let assign2890_e3102: f64 = (1.0 + assign2890_e3101);
        let assign2890_e3103: f64 = (assign2890_e3102).ln();
        let assign2890_e3104: f64 = (var_phitd * assign2890_e3103);
        let assign2890_e3105: f64 = (var_ubibot_d + assign2890_e3104);
        var_vbibot_d = assign2890_e3105;

        let assign2900_e3111: f64 = (0.05 - var_ubisti_d);
        let assign2900_e3113: f64 = (assign2900_e3111 * var_phitdinv);
        let assign2900_e3114: f64 = (assign2900_e3113).exp();
        let assign2900_e3115: f64 = (1.0 + assign2900_e3114);
        let assign2900_e3116: f64 = (assign2900_e3115).ln();
        let assign2900_e3117: f64 = (var_phitd * assign2900_e3116);
        let assign2900_e3118: f64 = (var_ubisti_d + assign2900_e3117);
        var_vbisti_d = assign2900_e3118;

        let assign2910_e3124: f64 = (0.05 - var_ubigat_d);
        let assign2910_e3126: f64 = (assign2910_e3124 * var_phitdinv);
        let assign2910_e3127: f64 = (assign2910_e3126).exp();
        let assign2910_e3128: f64 = (1.0 + assign2910_e3127);
        let assign2910_e3129: f64 = (assign2910_e3128).ln();
        let assign2910_e3130: f64 = (var_phitd * assign2910_e3129);
        let assign2910_e3131: f64 = (var_ubigat_d + assign2910_e3130);
        var_vbigat_d = assign2910_e3131;

        let assign2920_e3134: f64 = (1.0 / var_vbibot_d);
        var_vbiinvbot_d = assign2920_e3134;

        let assign2930_e3137: f64 = (1.0 / var_vbisti_d);
        var_vbiinvsti_d = assign2930_e3137;

        let assign2940_e3140: f64 = (1.0 / var_vbigat_d);
        var_vbiinvgat_d = assign2940_e3140;

        let assign2950_e3144: f64 = (var_vbirbotd_i * var_vbiinvbot_d);
        let assign2950_e3146: f64 = (assign2950_e3144).powf(var_pbotd_i);
        let assign2950_e3147: f64 = (var_cjorbotd_i * assign2950_e3146);
        var_cjobot_d = assign2950_e3147;

        let assign2960_e3151: f64 = (var_vbirstid_i * var_vbiinvsti_d);
        let assign2960_e3153: f64 = (assign2960_e3151).powf(var_pstid_i);
        let assign2960_e3154: f64 = (var_cjorstid_i * assign2960_e3153);
        var_cjosti_d = assign2960_e3154;

        let assign2970_e3158: f64 = (var_vbirgatd_i * var_vbiinvgat_d);
        let assign2970_e3160: f64 = (assign2970_e3158).powf(var_pgatd_i);
        let assign2970_e3161: f64 = (var_cjorgatd_i * assign2970_e3160);
        var_cjogat_d = assign2970_e3161;

        let assign2980_e3164: f64 = (var_cjobot_d * var_vbibot_d);
        let assign2980_e3166: f64 = (assign2980_e3164 * var_one_over_one_minus_pbot_d);
        var_qprefbot_d = assign2980_e3166;

        let assign2990_e3169: f64 = (var_cjosti_d * var_vbisti_d);
        let assign2990_e3171: f64 = (assign2990_e3169 * var_one_over_one_minus_psti_d);
        var_qprefsti_d = assign2990_e3171;

        let assign3000_e3174: f64 = (var_cjogat_d * var_vbigat_d);
        let assign3000_e3176: f64 = (assign3000_e3174 * var_one_over_one_minus_pgat_d);
        var_qprefgat_d = assign3000_e3176;

        let assign3010_e3179: f64 = (2.0 * var_cjobot_d);
        var_qpref2bot_d = assign3010_e3179;

        let assign3020_e3182: f64 = (2.0 * var_cjosti_d);
        var_qpref2sti_d = assign3020_e3182;

        let assign3030_e3185: f64 = (2.0 * var_cjogat_d);
        var_qpref2gat_d = assign3030_e3185;

        let assign3040_e3188: f64 = (0.5 * var_phigdbot_d);
        let assign3040_e3190: f64 = (assign3040_e3188).max(var_phitd);
        var_deltaebot_d = assign3040_e3190;

        let assign3050_e3193: f64 = (0.5 * var_phigdsti_d);
        let assign3050_e3195: f64 = (assign3050_e3193).max(var_phitd);
        var_deltaesti_d = assign3050_e3195;

        let assign3060_e3198: f64 = (0.5 * var_phigdgat_d);
        let assign3060_e3200: f64 = (assign3060_e3198).max(var_phitd);
        var_deltaegat_d = assign3060_e3200;

        let assign3070_e3203: f64 = (var_deltaebot_d * var_phitdinv);
        var_atatbot_d = assign3070_e3203;

        let assign3080_e3206: f64 = (var_deltaesti_d * var_phitdinv);
        var_atatsti_d = assign3080_e3206;

        let assign3090_e3209: f64 = (var_deltaegat_d * var_phitdinv);
        var_atatgat_d = assign3090_e3209;

        let assign3100_e3212: f64 = (32.0 * var_mefftatbotd_i);
        let assign3100_e3214: f64 = (assign3100_e3212 * 9.1093826e-31);
        let assign3100_e3216: f64 = (assign3100_e3214 * 1.6021918e-19);
        let assign3100_e3219: f64 = (var_deltaebot_d * var_deltaebot_d);
        let assign3100_e3221: f64 = (assign3100_e3219 * var_deltaebot_d);
        let assign3100_e3222: f64 = (assign3100_e3216 * assign3100_e3221);
        let assign3100_e3223: f64 = (assign3100_e3222).sqrt();
        let assign3100_e3226: f64 = (3.0 * 1.05457168e-34);
        let assign3100_e3227: f64 = (assign3100_e3223 / assign3100_e3226);
        var_btatpartbot_d = assign3100_e3227;

        let assign3110_e3230: f64 = (32.0 * var_mefftatstid_i);
        let assign3110_e3232: f64 = (assign3110_e3230 * 9.1093826e-31);
        let assign3110_e3234: f64 = (assign3110_e3232 * 1.6021918e-19);
        let assign3110_e3237: f64 = (var_deltaesti_d * var_deltaesti_d);
        let assign3110_e3239: f64 = (assign3110_e3237 * var_deltaesti_d);
        let assign3110_e3240: f64 = (assign3110_e3234 * assign3110_e3239);
        let assign3110_e3241: f64 = (assign3110_e3240).sqrt();
        let assign3110_e3244: f64 = (3.0 * 1.05457168e-34);
        let assign3110_e3245: f64 = (assign3110_e3241 / assign3110_e3244);
        var_btatpartsti_d = assign3110_e3245;

        let assign3120_e3248: f64 = (32.0 * var_mefftatgatd_i);
        let assign3120_e3250: f64 = (assign3120_e3248 * 9.1093826e-31);
        let assign3120_e3252: f64 = (assign3120_e3250 * 1.6021918e-19);
        let assign3120_e3255: f64 = (var_deltaegat_d * var_deltaegat_d);
        let assign3120_e3257: f64 = (assign3120_e3255 * var_deltaegat_d);
        let assign3120_e3258: f64 = (assign3120_e3252 * assign3120_e3257);
        let assign3120_e3259: f64 = (assign3120_e3258).sqrt();
        let assign3120_e3262: f64 = (3.0 * 1.05457168e-34);
        let assign3120_e3263: f64 = (assign3120_e3259 / assign3120_e3262);
        var_btatpartgat_d = assign3120_e3263;

        let assign3130_e3269: f64 = (var_tkd_1 - var_tkr_1);
        let assign3130_e3270: f64 = (var_stfbbtbotd_i * assign3130_e3269);
        let assign3130_e3271: f64 = (1.0 + assign3130_e3270);
        let assign3130_e3272: f64 = (var_fbbtrbotd_i * assign3130_e3271);
        var_fbbtbot_d = assign3130_e3272;

        let assign3140_e3278: f64 = (var_tkd_1 - var_tkr_1);
        let assign3140_e3279: f64 = (var_stfbbtstid_i * assign3140_e3278);
        let assign3140_e3280: f64 = (1.0 + assign3140_e3279);
        let assign3140_e3281: f64 = (var_fbbtrstid_i * assign3140_e3280);
        var_fbbtsti_d = assign3140_e3281;

        let assign3150_e3287: f64 = (var_tkd_1 - var_tkr_1);
        let assign3150_e3288: f64 = (var_stfbbtgatd_i * assign3150_e3287);
        let assign3150_e3289: f64 = (1.0 + assign3150_e3288);
        let assign3150_e3290: f64 = (var_fbbtrgatd_i * assign3150_e3289);
        var_fbbtgat_d = assign3150_e3290;
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
        var_fbbtgat_d_db0 = 0.0;
        var_fbbtgat_d_db1 = 0.0;
        var_fbbtgat_d_db2 = 0.0;
        var_fbbtgat_d_db3 = 0.0;
        var_fbbtgat_d_db4 = 0.0;
        var_fbbtgat_d_db5 = 0.0;
        var_fbbtgat_d_db6 = 0.0;

        let (assign3160_e3296,) = {
    if (var_fbbtbot_d > 0.0) {
        (var_fbbtbot_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot_d = assign3160_e3296;

        let (assign3170_e3302,) = {
    if (var_fbbtsti_d > 0.0) {
        (var_fbbtsti_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti_d = assign3170_e3302;

        let (assign3180_e3308, assign3180_e3308_d_n0, assign3180_e3308_d_n1, assign3180_e3308_d_n2, assign3180_e3308_d_n3, assign3180_e3308_d_n4, assign3180_e3308_d_n5, assign3180_e3308_d_n6, assign3180_e3308_d_n7, assign3180_e3308_d_n8, assign3180_e3308_d_n9, assign3180_e3308_d_n10, assign3180_e3308_d_n11, assign3180_e3308_d_b0, assign3180_e3308_d_b1, assign3180_e3308_d_b2, assign3180_e3308_d_b3, assign3180_e3308_d_b4, assign3180_e3308_d_b5, assign3180_e3308_d_b6,) = {
    if (var_fbbtgat_d > 0.0) {
        (var_fbbtgat_d, var_fbbtgat_d_dn0, var_fbbtgat_d_dn1, var_fbbtgat_d_dn2, var_fbbtgat_d_dn3, var_fbbtgat_d_dn4, var_fbbtgat_d_dn5, var_fbbtgat_d_dn6, var_fbbtgat_d_dn7, var_fbbtgat_d_dn8, var_fbbtgat_d_dn9, var_fbbtgat_d_dn10, var_fbbtgat_d_dn11, var_fbbtgat_d_db0, var_fbbtgat_d_db1, var_fbbtgat_d_db2, var_fbbtgat_d_db3, var_fbbtgat_d_db4, var_fbbtgat_d_db5, var_fbbtgat_d_db6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat_d = assign3180_e3308;
        var_fbbtgat_d_dn0 = assign3180_e3308_d_n0;
        var_fbbtgat_d_dn1 = assign3180_e3308_d_n1;
        var_fbbtgat_d_dn2 = assign3180_e3308_d_n2;
        var_fbbtgat_d_dn3 = assign3180_e3308_d_n3;
        var_fbbtgat_d_dn4 = assign3180_e3308_d_n4;
        var_fbbtgat_d_dn5 = assign3180_e3308_d_n5;
        var_fbbtgat_d_dn6 = assign3180_e3308_d_n6;
        var_fbbtgat_d_dn7 = assign3180_e3308_d_n7;
        var_fbbtgat_d_dn8 = assign3180_e3308_d_n8;
        var_fbbtgat_d_dn9 = assign3180_e3308_d_n9;
        var_fbbtgat_d_dn10 = assign3180_e3308_d_n10;
        var_fbbtgat_d_dn11 = assign3180_e3308_d_n11;
        var_fbbtgat_d_db0 = assign3180_e3308_d_b0;
        var_fbbtgat_d_db1 = assign3180_e3308_d_b1;
        var_fbbtgat_d_db2 = assign3180_e3308_d_b2;
        var_fbbtgat_d_db3 = assign3180_e3308_d_b3;
        var_fbbtgat_d_db4 = assign3180_e3308_d_b4;
        var_fbbtgat_d_db5 = assign3180_e3308_d_b5;
        var_fbbtgat_d_db6 = assign3180_e3308_d_b6;

        let assign3190_e3311: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard28 = assign3190_e3311;

        let (assign3200_e3317,) = {
    if (var_guard28 != 0.0) {
        let assign3200_e3315: f64 = (var_phiggat2nd_d + var_deltaphigd);
        (assign3200_e3315,)
    } else {
        (var_phigdgat2nd_d,)
    }
};
        var_phigdgat2nd_d = assign3200_e3317;

        let (assign3210_e3334,) = {
    if (var_guard28 != 0.0) {
        let assign3210_e3321: f64 = (var_auxt).powf(1.5);
        let assign3210_e3325: f64 = (var_phigrgat2nd_d * var_phitrinv);
        let assign3210_e3328: f64 = (var_phigdgat2nd_d * var_phitdinv);
        let assign3210_e3329: f64 = (assign3210_e3325 - assign3210_e3328);
        let assign3210_e3330: f64 = (0.5 * assign3210_e3329);
        let assign3210_e3331: f64 = (assign3210_e3330).exp();
        let assign3210_e3332: f64 = (assign3210_e3321 * assign3210_e3331);
        (assign3210_e3332,)
    } else {
        (var_ftdgat2nd_d,)
    }
};
        var_ftdgat2nd_d = assign3210_e3334;

        let (assign3220_e3347,) = {
    if (var_guard28 != 0.0) {
        let assign3220_e3338: f64 = (var_vbirgat2nd_d * var_auxt);
        let assign3220_e3341: f64 = (2.0 * var_phitd);
        let assign3220_e3343: f64 = (var_ftdgat2nd_d).ln();
        let assign3220_e3344: f64 = (assign3220_e3341 * assign3220_e3343);
        let assign3220_e3345: f64 = (assign3220_e3338 - assign3220_e3344);
        (assign3220_e3345,)
    } else {
        (var_ubigat2nd_d,)
    }
};
        var_ubigat2nd_d = assign3220_e3347;

        let (assign3230_e3363,) = {
    if (var_guard28 != 0.0) {
        let assign3230_e3354: f64 = (0.05 - var_ubigat2nd_d);
        let assign3230_e3356: f64 = (assign3230_e3354 * var_phitdinv);
        let assign3230_e3357: f64 = (assign3230_e3356).exp();
        let assign3230_e3358: f64 = (1.0 + assign3230_e3357);
        let assign3230_e3359: f64 = (assign3230_e3358).ln();
        let assign3230_e3360: f64 = (var_phitd * assign3230_e3359);
        let assign3230_e3361: f64 = (var_ubigat2nd_d + assign3230_e3360);
        (assign3230_e3361,)
    } else {
        (var_vbigat2nd_d,)
    }
};
        var_vbigat2nd_d = assign3230_e3363;

        let (assign3240_e3369,) = {
    if (var_guard28 != 0.0) {
        let assign3240_e3367: f64 = (1.0 / var_vbigat2nd_d);
        (assign3240_e3367,)
    } else {
        (var_vbiinvgat2nd_d,)
    }
};
        var_vbiinvgat2nd_d = assign3240_e3369;

        let (assign3250_e3379,) = {
    if (var_guard28 != 0.0) {
        let assign3250_e3374: f64 = (var_vbirgat2nd_d * var_vbiinvgat2nd_d);
        let assign3250_e3376: f64 = (assign3250_e3374).powf(var_pgat2nd_d);
        let assign3250_e3377: f64 = (var_cjorgat2nd_d * assign3250_e3376);
        (assign3250_e3377,)
    } else {
        (var_cjogat2nd_d,)
    }
};
        var_cjogat2nd_d = assign3250_e3379;

        let (assign3260_e3387,) = {
    if (var_guard28 != 0.0) {
        let assign3260_e3383: f64 = (var_cjogat2nd_d * var_vbigat2nd_d);
        let assign3260_e3385: f64 = (assign3260_e3383 * var_one_over_one_minus_pgat2nd_d);
        (assign3260_e3385,)
    } else {
        (var_qprefgat2nd_d,)
    }
};
        var_qprefgat2nd_d = assign3260_e3387;

        let (assign3270_e3393,) = {
    if (var_guard28 != 0.0) {
        let assign3270_e3391: f64 = (2.0 * var_cjogat2nd_d);
        (assign3270_e3391,)
    } else {
        (var_qpref2gat2nd_d,)
    }
};
        var_qpref2gat2nd_d = assign3270_e3393;

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

        let assign3500_e3418: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard29 = assign3500_e3418;

        let (assign3510_e3427,) = {
    if (var_guard29 != 0.0) {
        let (assign3510_e3425,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3510_e3425,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3510_e3427;

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
        *var_cjobot_d_slot = var_cjobot_d;
        *var_cjogat2nd_slot = var_cjogat2nd;
        *var_cjogat2nd_d_slot = var_cjogat2nd_d;
        *var_cjogat_d_slot = var_cjogat_d;
        *var_cjosti_d_slot = var_cjosti_d;
        *var_deltaebot_d_slot = var_deltaebot_d;
        *var_deltaegat_d_slot = var_deltaegat_d;
        *var_deltaesti_d_slot = var_deltaesti_d;
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
        *var_fbbtgat_d_dn2_slot = var_fbbtgat_d_dn2;
        *var_fbbtgat_d_dn3_slot = var_fbbtgat_d_dn3;
        *var_fbbtgat_d_dn4_slot = var_fbbtgat_d_dn4;
        *var_fbbtgat_d_dn5_slot = var_fbbtgat_d_dn5;
        *var_fbbtgat_d_dn6_slot = var_fbbtgat_d_dn6;
        *var_fbbtgat_d_dn7_slot = var_fbbtgat_d_dn7;
        *var_fbbtgat_d_dn8_slot = var_fbbtgat_d_dn8;
        *var_fbbtgat_d_dn9_slot = var_fbbtgat_d_dn9;
        *var_fbbtsti_d_slot = var_fbbtsti_d;
        *var_ftdbot_d_slot = var_ftdbot_d;
        *var_ftdgat2nd_d_slot = var_ftdgat2nd_d;
        *var_ftdgat_d_slot = var_ftdgat_d;
        *var_ftdsti_d_slot = var_ftdsti_d;
        *var_guard28_slot = var_guard28;
        *var_guard29_slot = var_guard29;
        *var_idsatbot_d_slot = var_idsatbot_d;
        *var_idsatgat_d_slot = var_idsatgat_d;
        *var_idsatsti_d_slot = var_idsatsti_d;
        *var_invnf_slot = var_invnf;
        *var_jw_i_slot = var_jw_i;
        *var_l_i_slot = var_l_i;
        *var_le_slot = var_le;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lssource_i_slot = var_lssource_i;
        *var_nf_i_slot = var_nf_i;
        *var_pd_i_slot = var_pd_i;
        *var_phigdbot_d_slot = var_phigdbot_d;
        *var_phigdgat2nd_d_slot = var_phigdgat2nd_d;
        *var_phigdgat_d_slot = var_phigdgat_d;
        *var_phigdsti_d_slot = var_phigdsti_d;
        *var_ps_i_slot = var_ps_i;
        *var_qpref2bot_d_slot = var_qpref2bot_d;
        *var_qpref2gat2nd_slot = var_qpref2gat2nd;
        *var_qpref2gat2nd_d_slot = var_qpref2gat2nd_d;
        *var_qpref2gat_d_slot = var_qpref2gat_d;
        *var_qpref2sti_d_slot = var_qpref2sti_d;
        *var_qprefbot_d_slot = var_qprefbot_d;
        *var_qprefgat2nd_slot = var_qprefgat2nd;
        *var_qprefgat2nd_d_slot = var_qprefgat2nd_d;
        *var_qprefgat_d_slot = var_qprefgat_d;
        *var_qprefsti_d_slot = var_qprefsti_d;
        *var_sa_i_slot = var_sa_i;
        *var_sb_i_slot = var_sb_i;
        *var_sc_i_slot = var_sc_i;
        *var_sd_i_slot = var_sd_i;
        *var_ubibot_d_slot = var_ubibot_d;
        *var_ubigat2nd_d_slot = var_ubigat2nd_d;
        *var_ubigat_d_slot = var_ubigat_d;
        *var_ubisti_d_slot = var_ubisti_d;
        *var_vbibot_d_slot = var_vbibot_d;
        *var_vbigat2nd_slot = var_vbigat2nd;
        *var_vbigat2nd_d_slot = var_vbigat2nd_d;
        *var_vbigat_d_slot = var_vbigat_d;
        *var_vbiinvbot_d_slot = var_vbiinvbot_d;
        *var_vbiinvgat2nd_slot = var_vbiinvgat2nd;
        *var_vbiinvgat2nd_d_slot = var_vbiinvgat2nd_d;
        *var_vbiinvgat_d_slot = var_vbiinvgat_d;
        *var_vbiinvsti_d_slot = var_vbiinvsti_d;
        *var_vbisti_d_slot = var_vbisti_d;
        *var_w_i_slot = var_w_i;
        *var_we_slot = var_we;
        *var_xgw_i_slot = var_xgw_i;
    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard29: f64,
        var_l_i: f64,
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
        var_bgidl_p_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
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
        var_cox_p_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delwod_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fntexc_p_slot: &mut f64,
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
        var_iae_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_iiae_slot: &mut f64,
        var_iilcv_slot: &mut f64,
        var_iiwcv_slot: &mut f64,
        var_iiwe_slot: &mut f64,
        var_iiwecv_slot: &mut f64,
        var_il_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_ile2_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_l_f_slot: &mut f64,
        var_l_slif_slot: &mut f64,
        var_lcv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_lecv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_ngcon_i_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
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
        var_vp_p_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_w_f_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_wcv_slot: &mut f64,
        var_we_slot: &mut f64,
        var_wecv_slot: &mut f64,
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
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
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
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fntexc_p: f64 = *var_fntexc_p_slot;
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
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_iiae: f64 = *var_iiae_slot;
        let mut var_iilcv: f64 = *var_iilcv_slot;
        let mut var_iiwcv: f64 = *var_iiwcv_slot;
        let mut var_iiwe: f64 = *var_iiwe_slot;
        let mut var_iiwecv: f64 = *var_iiwecv_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_ile2: f64 = *var_ile2_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_l_f: f64 = *var_l_f_slot;
        let mut var_l_slif: f64 = *var_l_slif_slot;
        let mut var_lcv: f64 = *var_lcv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_lecv: f64 = *var_lecv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_ngcon_i: f64 = *var_ngcon_i_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
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
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_w_f: f64 = *var_w_f_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_wcv: f64 = *var_wcv_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xgwe: f64 = *var_xgwe_slot;

        let (assign3520_e3434,) = {
    if (var_guard29 != 0.0) {
        let assign3520_e3431: f64 = (var_nf_i + 0.5);
        let assign3520_e3432: f64 = (assign3520_e3431).floor();
        (assign3520_e3432,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3520_e3434;

        let (assign3530_e3440,) = {
    if (var_guard29 != 0.0) {
        let assign3530_e3438: f64 = (1.0 / var_nf_i);
        (assign3530_e3438,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign3530_e3440;

        let assign3540_e3443: f64 = (var_w_i * var_invnf);
        let (assign3540_e3450,) = {
    if (assign3540_e3443 > 1e-9) {
        let assign3540_e3448: f64 = (var_w_i * var_invnf);
        (assign3540_e3448,)
    } else {
        (1e-9,)
    }
};
        var_w_i = assign3540_e3450;

        var_sca_i = p.p5;

        var_scb_i = p.p6;

        var_scc_i = p.p7;

        let (assign3580_e3459,) = {
    if (p.p10 < 1.5) {
        (1.0,)
    } else {
        (2.0,)
    }
};
        var_ngcon_i = assign3580_e3459;

        let assign3590_e3462: f64 = (1e-6 / var_l_i);
        var_il = assign3590_e3462;

        let assign3600_e3465: f64 = (1e-6 / var_w_i);
        var_iw = assign3600_e3465;

        let assign3610_e3470: f64 = (p.p187 * var_il);
        let assign3610_e3471: f64 = (1.0 + assign3610_e3470);
        let assign3610_e3472: f64 = (p.p186 * assign3610_e3471);
        let assign3610_e3476: f64 = (p.p188 * var_iw);
        let assign3610_e3477: f64 = (1.0 + assign3610_e3476);
        let assign3610_e3478: f64 = (assign3610_e3472 * assign3610_e3477);
        var_dellps = assign3610_e3478;

        let assign3620_e3483: f64 = (p.p191 * var_il);
        let assign3620_e3484: f64 = (1.0 + assign3620_e3483);
        let assign3620_e3485: f64 = (p.p190 * assign3620_e3484);
        let assign3620_e3489: f64 = (p.p192 * var_iw);
        let assign3620_e3490: f64 = (1.0 + assign3620_e3489);
        let assign3620_e3491: f64 = (assign3620_e3485 * assign3620_e3490);
        var_delwod = assign3620_e3491;

        let assign3630_e3494: f64 = (var_l_i + var_dellps);
        let assign3630_e3497: f64 = (2.0 * p.p189);
        let assign3630_e3498: f64 = (assign3630_e3494 - assign3630_e3497);
        let (assign3630_e3509,) = {
    if (assign3630_e3498 > 1e-9) {
        let assign3630_e3503: f64 = (var_l_i + var_dellps);
        let assign3630_e3506: f64 = (2.0 * p.p189);
        let assign3630_e3507: f64 = (assign3630_e3503 - assign3630_e3506);
        (assign3630_e3507,)
    } else {
        (1e-9,)
    }
};
        var_le = assign3630_e3509;

        let assign3640_e3512: f64 = (var_w_i + var_delwod);
        let assign3640_e3515: f64 = (2.0 * p.p193);
        let assign3640_e3516: f64 = (assign3640_e3512 - assign3640_e3515);
        let (assign3640_e3527,) = {
    if (assign3640_e3516 > 1e-9) {
        let assign3640_e3521: f64 = (var_w_i + var_delwod);
        let assign3640_e3524: f64 = (2.0 * p.p193);
        let assign3640_e3525: f64 = (assign3640_e3521 - assign3640_e3524);
        (assign3640_e3525,)
    } else {
        (1e-9,)
    }
};
        var_we = assign3640_e3527;

        let assign3650_e3530: f64 = (1e-6 / var_le);
        var_ile = assign3650_e3530;

        let assign3660_e3533: f64 = (var_ile * var_ile);
        var_ile2 = assign3660_e3533;

        let assign3670_e3536: f64 = (1e-6 / var_we);
        var_iwe = assign3670_e3536;

        let assign3680_e3539: f64 = (1.0 / var_iwe);
        var_iiwe = assign3680_e3539;

        let assign3690_e3542: f64 = (var_ile * var_iwe);
        var_iae = assign3690_e3542;

        let assign3700_e3545: f64 = (1.0 / var_iae);
        var_iiae = assign3700_e3545;

        let assign3710_e3548: f64 = (var_l_i + var_dellps);
        let assign3710_e3551: f64 = (2.0 * p.p189);
        let assign3710_e3552: f64 = (assign3710_e3548 - assign3710_e3551);
        let assign3710_e3554: f64 = (assign3710_e3552 + p.p194);
        let (assign3710_e3567,) = {
    if (assign3710_e3554 > 1e-9) {
        let assign3710_e3559: f64 = (var_l_i + var_dellps);
        let assign3710_e3562: f64 = (2.0 * p.p189);
        let assign3710_e3563: f64 = (assign3710_e3559 - assign3710_e3562);
        let assign3710_e3565: f64 = (assign3710_e3563 + p.p194);
        (assign3710_e3565,)
    } else {
        (1e-9,)
    }
};
        var_lecv = assign3710_e3567;

        let assign3720_e3570: f64 = (var_w_i + var_delwod);
        let assign3720_e3573: f64 = (2.0 * p.p193);
        let assign3720_e3574: f64 = (assign3720_e3570 - assign3720_e3573);
        let assign3720_e3576: f64 = (assign3720_e3574 + p.p195);
        let (assign3720_e3589,) = {
    if (assign3720_e3576 > 1e-9) {
        let assign3720_e3581: f64 = (var_w_i + var_delwod);
        let assign3720_e3584: f64 = (2.0 * p.p193);
        let assign3720_e3585: f64 = (assign3720_e3581 - assign3720_e3584);
        let assign3720_e3587: f64 = (assign3720_e3585 + p.p195);
        (assign3720_e3587,)
    } else {
        (1e-9,)
    }
};
        var_wecv = assign3720_e3589;

        let assign3730_e3592: f64 = (var_wecv / 1e-6);
        var_iiwecv = assign3730_e3592;

        let assign3740_e3595: f64 = (var_l_i + var_dellps);
        let assign3740_e3597: f64 = (assign3740_e3595 + p.p194);
        let (assign3740_e3606,) = {
    if (assign3740_e3597 > 1e-9) {
        let assign3740_e3602: f64 = (var_l_i + var_dellps);
        let assign3740_e3604: f64 = (assign3740_e3602 + p.p194);
        (assign3740_e3604,)
    } else {
        (1e-9,)
    }
};
        var_lcv = assign3740_e3606;

        let assign3750_e3609: f64 = (var_w_i + var_delwod);
        let assign3750_e3611: f64 = (assign3750_e3609 + p.p195);
        let (assign3750_e3620,) = {
    if (assign3750_e3611 > 1e-9) {
        let assign3750_e3616: f64 = (var_w_i + var_delwod);
        let assign3750_e3618: f64 = (assign3750_e3616 + p.p195);
        (assign3750_e3618,)
    } else {
        (1e-9,)
    }
};
        var_wcv = assign3750_e3620;

        let assign3760_e3623: f64 = (var_lcv / 1e-6);
        var_iilcv = assign3760_e3623;

        let assign3770_e3626: f64 = (var_wcv / 1e-6);
        var_iiwcv = assign3770_e3626;

        let assign3780_e3629: f64 = (var_l_i + var_dellps);
        let (assign3780_e3636,) = {
    if (assign3780_e3629 > 1e-9) {
        let assign3780_e3634: f64 = (var_l_i + var_dellps);
        (assign3780_e3634,)
    } else {
        (1e-9,)
    }
};
        var_l_f = assign3780_e3636;

        let assign3790_e3639: f64 = (var_l_f + p.p441);
        let (assign3790_e3646,) = {
    if (assign3790_e3639 > 1e-9) {
        let assign3790_e3644: f64 = (var_l_f + p.p441);
        (assign3790_e3644,)
    } else {
        (1e-9,)
    }
};
        var_l_slif = assign3790_e3646;

        let assign3800_e3649: f64 = (var_w_i + var_delwod);
        let (assign3800_e3656,) = {
    if (assign3800_e3649 > 1e-9) {
        let assign3800_e3654: f64 = (var_w_i + var_delwod);
        (assign3800_e3654,)
    } else {
        (1e-9,)
    }
};
        var_w_f = assign3800_e3656;

        let assign3810_e3660: f64 = (0.5 * var_delwod);
        let assign3810_e3661: f64 = (var_xgw_i - assign3810_e3660);
        let (assign3810_e3670,) = {
    if (assign3810_e3661 > 1e-9) {
        let assign3810_e3667: f64 = (0.5 * var_delwod);
        let assign3810_e3668: f64 = (var_xgw_i - assign3810_e3667);
        (assign3810_e3668,)
    } else {
        (1e-9,)
    }
};
        var_xgwe = assign3810_e3670;

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

        let assign4480_e3738: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4480_e3740: f64 = if assign4480_e3738 == 1.0 { 1.0 } else { 0.0 };
        var_guard30 = assign4480_e3740;

        let (assign4490_e3744,) = {
    if (var_guard30 != 0.0) {
        (p.p121,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign4490_e3744;

        var_gc3ov_p = p.p120;

        let assign4510_e3747: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4510_e3749: f64 = if assign4510_e3747 == 1.0 { 1.0 } else { 0.0 };
        var_guard31 = assign4510_e3749;

        let (assign4520_e3753,) = {
    if (var_guard31 != 0.0) {
        (p.p122,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign4520_e3753;

        var_gc2ovd_p = var_gc2ov_p;

        let assign4540_e3756: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4540_e3758: f64 = if assign4540_e3756 == 1.0 { 1.0 } else { 0.0 };
        var_guard32 = assign4540_e3758;

        let (assign4550_e3762,) = {
    if (var_guard32 != 0.0) {
        (p.p123,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign4550_e3762;

        var_gc3ovd_p = var_gc3ov_p;

        let assign4570_e3765: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4570_e3767: f64 = if assign4570_e3765 == 1.0 { 1.0 } else { 0.0 };
        var_guard33 = assign4570_e3767;

        let (assign4580_e3771,) = {
    if (var_guard33 != 0.0) {
        (p.p124,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign4580_e3771;

        var_chib_p = p.p125;

        var_agidl_p = p.p126;

        var_agidld_p = p.p127;

        var_bgidl_p = p.p128;

        var_bgidld_p = p.p129;

        var_stbgidl_p = p.p130;

        var_stbgidld_p = p.p131;

        var_cgidl_p = p.p132;

        var_cgidld_p = p.p133;

        var_cox_p = p.p134;

        var_delvtac_p = p.p135;

        var_facneffac_p = p.p136;

        var_thesatac_p = p.p98;

        let assign4720_e3786: f64 = if param_given[137] { 1.0 } else { 0.0 };
        let assign4720_e3788: f64 = if assign4720_e3786 == 1.0 { 1.0 } else { 0.0 };
        var_guard34 = assign4720_e3788;

        let (assign4730_e3792,) = {
    if (var_guard34 != 0.0) {
        (p.p137,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign4730_e3792;

        var_axac_p = p.p103;

        let assign4750_e3795: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4750_e3797: f64 = if assign4750_e3795 == 1.0 { 1.0 } else { 0.0 };
        var_guard35 = assign4750_e3797;

        let (assign4760_e3801,) = {
    if (var_guard35 != 0.0) {
        (p.p138,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4760_e3801;

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

        var_fnt_p = p.p155;

        var_fntexc_p = p.p156;

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
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_cf_p_slot = var_cf_p;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfd_p_slot = var_cfd_p;
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
        *var_cox_p_slot = var_cox_p;
        *var_cs_p_slot = var_cs_p;
        *var_ct_p_slot = var_ct_p;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctg_p_slot = var_ctg_p;
        *var_dellps_slot = var_dellps;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delwod_slot = var_delwod;
        *var_dphib_p_slot = var_dphib_p;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_feta_p_slot = var_feta_p;
        *var_fnt_p_slot = var_fnt_p;
        *var_fntexc_p_slot = var_fntexc_p;
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
        *var_iae_slot = var_iae;
        *var_iginv_p_slot = var_iginv_p;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_iiae_slot = var_iiae;
        *var_iilcv_slot = var_iilcv;
        *var_iiwcv_slot = var_iiwcv;
        *var_iiwe_slot = var_iiwe;
        *var_iiwecv_slot = var_iiwecv;
        *var_il_slot = var_il;
        *var_ile_slot = var_ile;
        *var_ile2_slot = var_ile2;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_invnf_slot = var_invnf;
        *var_iw_slot = var_iw;
        *var_iwe_slot = var_iwe;
        *var_l_f_slot = var_l_f;
        *var_l_slif_slot = var_l_slif;
        *var_lcv_slot = var_lcv;
        *var_le_slot = var_le;
        *var_lecv_slot = var_lecv;
        *var_mue_p_slot = var_mue_p;
        *var_neff_p_slot = var_neff_p;
        *var_nf_i_slot = var_nf_i;
        *var_ngcon_i_slot = var_ngcon_i;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
        *var_psce_p_slot = var_psce_p;
        *var_psceb_p_slot = var_psceb_p;
        *var_psced_p_slot = var_psced_p;
        *var_rs_p_slot = var_rs_p;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsg_p_slot = var_rsg_p;
        *var_sca_i_slot = var_sca_i;
        *var_scb_i_slot = var_scb_i;
        *var_scc_i_slot = var_scc_i;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_sta2_p_slot = var_sta2_p;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stcs_p_slot = var_stcs_p;
        *var_stct_p_slot = var_stct_p;
        *var_stig_p_slot = var_stig_p;
        *var_stmue_p_slot = var_stmue_p;
        *var_strs_p_slot = var_strs_p;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stvfb_p_slot = var_stvfb_p;
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
        *var_vp_p_slot = var_vp_p;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_w_f_slot = var_w_f;
        *var_w_i_slot = var_w_i;
        *var_wcv_slot = var_wcv;
        *var_we_slot = var_we;
        *var_wecv_slot = var_wecv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xgwe_slot = var_xgwe;
    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_aa_slot: &mut f64,
        var_bb_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard38_slot: &mut f64,
        var_lpcke_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_npcke_slot: &mut f64,
        var_nsub_slot: &mut f64,
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
        var_rse_p_slot: &mut f64,
        var_rwell_p_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
    ) {
        let mut var_aa: f64 = *var_aa_slot;
        let mut var_bb: f64 = *var_bb_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard38: f64 = *var_guard38_slot;
        let mut var_lpcke: f64 = *var_lpcke_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_npcke: f64 = *var_npcke_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
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
        let mut var_rse_p: f64 = *var_rse_p_slot;
        let mut var_rwell_p: f64 = *var_rwell_p_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;

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

        let assign5240_e3851: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign5240_e3851;

        let (assign5250_e3869,) = {
    if (var_guard36 != 0.0) {
        let assign5250_e3857: f64 = (var_ile).powf(p.p198);
        let assign5250_e3858: f64 = (p.p197 * assign5250_e3857);
        let assign5250_e3859: f64 = (p.p196 + assign5250_e3858);
        let assign5250_e3862: f64 = (p.p199 * var_iwe);
        let assign5250_e3863: f64 = (assign5250_e3859 + assign5250_e3862);
        let assign5250_e3866: f64 = (p.p200 * var_iae);
        let assign5250_e3867: f64 = (assign5250_e3863 + assign5250_e3866);
        (assign5250_e3867,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign5250_e3869;

        let (assign5260_e3885,) = {
    if (var_guard36 != 0.0) {
        let assign5260_e3874: f64 = (p.p202 * var_ile);
        let assign5260_e3875: f64 = (p.p201 + assign5260_e3874);
        let assign5260_e3878: f64 = (p.p203 * var_iwe);
        let assign5260_e3879: f64 = (assign5260_e3875 + assign5260_e3878);
        let assign5260_e3882: f64 = (p.p204 * var_iae);
        let assign5260_e3883: f64 = (assign5260_e3879 + assign5260_e3882);
        (assign5260_e3883,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign5260_e3885;

        let (assign5270_e3889,) = {
    if (var_guard36 != 0.0) {
        (p.p205,)
    } else {
        (var_st2vfb_p,)
    }
};
        var_st2vfb_p = assign5270_e3889;

        let (assign5280_e3893,) = {
    if (var_guard36 != 0.0) {
        (p.p206,)
    } else {
        (var_tox_p,)
    }
};
        var_tox_p = assign5280_e3893;

        let (assign5290_e3897,) = {
    if (var_guard36 != 0.0) {
        (p.p207,)
    } else {
        (var_epsrox_p,)
    }
};
        var_epsrox_p = assign5290_e3897;

        let (assign5300_e3930,) = {
    if (var_guard36 != 0.0) {
        let assign5300_e3903: f64 = (p.p209 * var_iwe);
        let assign5300_e3907: f64 = (var_we / p.p210);
        let assign5300_e3908: f64 = (1.0 + assign5300_e3907);
        let assign5300_e3909: f64 = (assign5300_e3908).ln();
        let assign5300_e3910: f64 = (assign5300_e3903 * assign5300_e3909);
        let assign5300_e3911: f64 = (1.0 + assign5300_e3910);
        let (assign5300_e3927,) = {
            if (assign5300_e3911 > 0.001) {
                let assign5300_e3917: f64 = (p.p209 * var_iwe);
                let assign5300_e3921: f64 = (var_we / p.p210);
                let assign5300_e3922: f64 = (1.0 + assign5300_e3921);
                let assign5300_e3923: f64 = (assign5300_e3922).ln();
                let assign5300_e3924: f64 = (assign5300_e3917 * assign5300_e3923);
                let assign5300_e3925: f64 = (1.0 + assign5300_e3924);
                (assign5300_e3925,)
            } else {
                (0.001,)
            }
        };
        let assign5300_e3928: f64 = (p.p208 * assign5300_e3927);
        (assign5300_e3928,)
    } else {
        (var_nsub0e,)
    }
};
        var_nsub0e = assign5300_e3930;

        let (assign5310_e3963,) = {
    if (var_guard36 != 0.0) {
        let assign5310_e3936: f64 = (p.p212 * var_iwe);
        let assign5310_e3940: f64 = (var_we / p.p213);
        let assign5310_e3941: f64 = (1.0 + assign5310_e3940);
        let assign5310_e3942: f64 = (assign5310_e3941).ln();
        let assign5310_e3943: f64 = (assign5310_e3936 * assign5310_e3942);
        let assign5310_e3944: f64 = (1.0 + assign5310_e3943);
        let (assign5310_e3960,) = {
            if (assign5310_e3944 > 0.001) {
                let assign5310_e3950: f64 = (p.p212 * var_iwe);
                let assign5310_e3954: f64 = (var_we / p.p213);
                let assign5310_e3955: f64 = (1.0 + assign5310_e3954);
                let assign5310_e3956: f64 = (assign5310_e3955).ln();
                let assign5310_e3957: f64 = (assign5310_e3950 * assign5310_e3956);
                let assign5310_e3958: f64 = (1.0 + assign5310_e3957);
                (assign5310_e3958,)
            } else {
                (0.001,)
            }
        };
        let assign5310_e3961: f64 = (p.p211 * assign5310_e3960);
        (assign5310_e3961,)
    } else {
        (var_npcke,)
    }
};
        var_npcke = assign5310_e3963;

        let (assign5320_e3996,) = {
    if (var_guard36 != 0.0) {
        let assign5320_e3969: f64 = (p.p215 * var_iwe);
        let assign5320_e3973: f64 = (var_we / p.p213);
        let assign5320_e3974: f64 = (1.0 + assign5320_e3973);
        let assign5320_e3975: f64 = (assign5320_e3974).ln();
        let assign5320_e3976: f64 = (assign5320_e3969 * assign5320_e3975);
        let assign5320_e3977: f64 = (1.0 + assign5320_e3976);
        let (assign5320_e3993,) = {
            if (assign5320_e3977 > 0.001) {
                let assign5320_e3983: f64 = (p.p215 * var_iwe);
                let assign5320_e3987: f64 = (var_we / p.p213);
                let assign5320_e3988: f64 = (1.0 + assign5320_e3987);
                let assign5320_e3989: f64 = (assign5320_e3988).ln();
                let assign5320_e3990: f64 = (assign5320_e3983 * assign5320_e3989);
                let assign5320_e3991: f64 = (1.0 + assign5320_e3990);
                (assign5320_e3991,)
            } else {
                (0.001,)
            }
        };
        let assign5320_e3994: f64 = (p.p214 * assign5320_e3993);
        (assign5320_e3994,)
    } else {
        (var_lpcke,)
    }
};
        var_lpcke = assign5320_e3996;

        let assign5330_e4000: f64 = (2.0 * var_lpcke);
        let assign5330_e4001: f64 = if var_le > assign5330_e4000 { 1.0 } else { 0.0 };
        var_guard37 = assign5330_e4001;

        let (assign5340_e4007,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        (75000000000.0,)
    } else {
        (var_aa,)
    }
};
        var_aa = assign5340_e4007;

        let (assign5350_e4021,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5350_e4014: f64 = (0.5 * var_npcke);
        let assign5350_e4015: f64 = (var_nsub0e + assign5350_e4014);
        let assign5350_e4016: f64 = (assign5350_e4015).sqrt();
        let assign5350_e4018: f64 = (var_nsub0e).sqrt();
        let assign5350_e4019: f64 = (assign5350_e4016 - assign5350_e4018);
        (assign5350_e4019,)
    } else {
        (var_bb,)
    }
};
        var_bb = assign5350_e4021;

        let (assign5360_e4046,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5360_e4026: f64 = (var_nsub0e).sqrt();
        let assign5360_e4031: f64 = (2.0 * var_lpcke);
        let assign5360_e4033: f64 = (assign5360_e4031 / var_le);
        let assign5360_e4036: f64 = (var_bb / var_aa);
        let assign5360_e4037: f64 = (assign5360_e4036).exp();
        let assign5360_e4039: f64 = (assign5360_e4037 - 1.0);
        let assign5360_e4040: f64 = (assign5360_e4033 * assign5360_e4039);
        let assign5360_e4041: f64 = (1.0 + assign5360_e4040);
        let assign5360_e4042: f64 = (assign5360_e4041).ln();
        let assign5360_e4043: f64 = (var_aa * assign5360_e4042);
        let assign5360_e4044: f64 = (assign5360_e4026 + assign5360_e4043);
        (assign5360_e4044,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5360_e4046;

        let (assign5370_e4054,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5370_e4052: f64 = (var_nsub * var_nsub);
        (assign5370_e4052,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5370_e4054;

        let assign5380_e4057: f64 = if var_le >= var_lpcke { 1.0 } else { 0.0 };
        var_guard38 = assign5380_e4057;

        let (assign5390_e4072,) = {
    if (((var_guard36 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 != 0.0)) {
        let assign5390_e4067: f64 = (var_npcke * var_lpcke);
        let assign5390_e4069: f64 = (assign5390_e4067 / var_le);
        let assign5390_e4070: f64 = (var_nsub0e + assign5390_e4069);
        (assign5390_e4070,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5390_e4072;

        let (assign5400_e4090,) = {
    if (((var_guard36 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 == 0.0)) {
        let assign5400_e4085: f64 = (var_le / var_lpcke);
        let assign5400_e4086: f64 = (2.0 - assign5400_e4085);
        let assign5400_e4087: f64 = (var_npcke * assign5400_e4086);
        let assign5400_e4088: f64 = (var_nsub0e + assign5400_e4087);
        (assign5400_e4088,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5400_e4090;

        let (assign5410_e4104,) = {
    if (var_guard36 != 0.0) {
        let assign5410_e4096: f64 = (p.p216 * var_ile);
        let assign5410_e4097: f64 = (1.0 - assign5410_e4096);
        let assign5410_e4100: f64 = (p.p217 * var_ile2);
        let assign5410_e4101: f64 = (assign5410_e4097 - assign5410_e4100);
        let assign5410_e4102: f64 = (var_nsub * assign5410_e4101);
        (assign5410_e4102,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign5410_e4104;

        let (assign5420_e4122,) = {
    if (var_guard36 != 0.0) {
        let assign5420_e4110: f64 = (var_ile).powf(p.p220);
        let assign5420_e4111: f64 = (p.p219 * assign5420_e4110);
        let assign5420_e4112: f64 = (p.p218 + assign5420_e4111);
        let assign5420_e4115: f64 = (p.p221 * var_iwe);
        let assign5420_e4116: f64 = (assign5420_e4112 + assign5420_e4115);
        let assign5420_e4119: f64 = (p.p222 * var_iae);
        let assign5420_e4120: f64 = (assign5420_e4116 + assign5420_e4119);
        (assign5420_e4120,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign5420_e4122;

        let (assign5430_e4126,) = {
    if (var_guard36 != 0.0) {
        (p.p223,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign5430_e4126;

        let (assign5440_e4130,) = {
    if (var_guard36 != 0.0) {
        (p.p224,)
    } else {
        (var_dvsbnud_p,)
    }
};
        var_dvsbnud_p = assign5440_e4130;

        let (assign5450_e4148,) = {
    if (var_guard36 != 0.0) {
        let assign5450_e4136: f64 = (var_ile).powf(p.p227);
        let assign5450_e4137: f64 = (p.p226 * assign5450_e4136);
        let assign5450_e4138: f64 = (p.p225 + assign5450_e4137);
        let assign5450_e4141: f64 = (p.p228 * var_iwe);
        let assign5450_e4142: f64 = (assign5450_e4138 + assign5450_e4141);
        let assign5450_e4145: f64 = (p.p229 * var_iae);
        let assign5450_e4146: f64 = (assign5450_e4142 + assign5450_e4145);
        (assign5450_e4146,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign5450_e4148;

        let (assign5460_e4167,) = {
    if (var_guard36 != 0.0) {
        let assign5460_e4155: f64 = (p.p231 * var_ile);
        let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
        let (assign5460_e4164,) = {
            if (1e-6 > assign5460_e4156) {
                (1e-6,)
            } else {
                let assign5460_e4162: f64 = (p.p231 * var_ile);
                let assign5460_e4163: f64 = (1.0 + assign5460_e4162);
                (assign5460_e4163,)
            }
        };
        let assign5460_e4165: f64 = (p.p230 * assign5460_e4164);
        (assign5460_e4165,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign5460_e4167;

        let (assign5470_e4171,) = {
    if (var_guard36 != 0.0) {
        (p.p232,)
    } else {
        (var_toxov_p,)
    }
};
        var_toxov_p = assign5470_e4171;

        let (assign5480_e4175,) = {
    if (var_guard36 != 0.0) {
        (p.p233,)
    } else {
        (var_toxovd_p,)
    }
};
        var_toxovd_p = assign5480_e4175;

        let (assign5490_e4179,) = {
    if (var_guard36 != 0.0) {
        (p.p236,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign5490_e4179;

        let (assign5500_e4183,) = {
    if (var_guard36 != 0.0) {
        (p.p237,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign5500_e4183;

        let (assign5510_e4205,) = {
    if (var_guard36 != 0.0) {
        let assign5510_e4189: f64 = (var_ile).powf(p.p240);
        let assign5510_e4190: f64 = (p.p239 * assign5510_e4189);
        let assign5510_e4191: f64 = (p.p238 + assign5510_e4190);
        let assign5510_e4195: f64 = (p.p241 * var_iwe);
        let assign5510_e4196: f64 = (1.0 + assign5510_e4195);
        let assign5510_e4197: f64 = (assign5510_e4191 * assign5510_e4196);
        let assign5510_e4201: f64 = (p.p242 * var_iae);
        let assign5510_e4202: f64 = (1.0 + assign5510_e4201);
        let assign5510_e4203: f64 = (assign5510_e4197 * assign5510_e4202);
        (assign5510_e4203,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign5510_e4205;

        let (assign5520_e4209,) = {
    if (var_guard36 != 0.0) {
        (p.p244,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign5520_e4209;

        let (assign5530_e4213,) = {
    if (var_guard36 != 0.0) {
        (p.p243,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign5530_e4213;

        let (assign5540_e4217,) = {
    if (var_guard36 != 0.0) {
        (p.p245,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign5540_e4217;

        let (assign5550_e4231,) = {
    if (var_guard36 != 0.0) {
        let assign5550_e4222: f64 = (var_ile).powf(p.p247);
        let assign5550_e4223: f64 = (p.p246 * assign5550_e4222);
        let assign5550_e4227: f64 = (p.p248 * var_iwe);
        let assign5550_e4228: f64 = (1.0 + assign5550_e4227);
        let assign5550_e4229: f64 = (assign5550_e4223 * assign5550_e4228);
        (assign5550_e4229,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign5550_e4231;

        let (assign5560_e4235,) = {
    if (var_guard36 != 0.0) {
        (p.p250,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign5560_e4235;

        let (assign5570_e4239,) = {
    if (var_guard36 != 0.0) {
        (p.p249,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign5570_e4239;

        let (assign5580_e4253,) = {
    if (var_guard36 != 0.0) {
        let assign5580_e4244: f64 = (var_ile).powf(p.p252);
        let assign5580_e4245: f64 = (p.p251 * assign5580_e4244);
        let assign5580_e4249: f64 = (p.p253 * var_iwe);
        let assign5580_e4250: f64 = (1.0 + assign5580_e4249);
        let assign5580_e4251: f64 = (assign5580_e4245 * assign5580_e4250);
        (assign5580_e4251,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign5580_e4253;

        let (assign5590_e4257,) = {
    if (var_guard36 != 0.0) {
        (p.p255,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign5590_e4257;

        let (assign5600_e4261,) = {
    if (var_guard36 != 0.0) {
        (p.p254,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign5600_e4261;

        *var_aa_slot = var_aa;
        *var_bb_slot = var_bb;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cf_p_slot = var_cf_p;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_ct_p_slot = var_ct_p;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctg_p_slot = var_ctg_p;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_guard36_slot = var_guard36;
        *var_guard37_slot = var_guard37;
        *var_guard38_slot = var_guard38;
        *var_lpcke_slot = var_lpcke;
        *var_neff_p_slot = var_neff_p;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
        *var_npcke_slot = var_npcke;
        *var_nsub_slot = var_nsub;
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
        *var_rse_p_slot = var_rse_p;
        *var_rwell_p_slot = var_rwell_p;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stct_p_slot = var_stct_p;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_tox_p_slot = var_tox_p;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vsbnud_p_slot = var_vsbnud_p;
    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_a1_p_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_fbet1e_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_gwe_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_lp1e_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_themu_p_slot: &mut f64,
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
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_fbet1e: f64 = *var_fbet1e_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_lp1e: f64 = *var_lp1e_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;

        let (assign5610_e4271,) = {
    if (var_guard36 != 0.0) {
        let assign5610_e4267: f64 = (p.p258 * var_iwe);
        let assign5610_e4268: f64 = (1.0 + assign5610_e4267);
        let assign5610_e4269: f64 = (p.p257 * assign5610_e4268);
        (assign5610_e4269,)
    } else {
        (var_fbet1e,)
    }
};
        var_fbet1e = assign5610_e4271;

        let (assign5620_e4290,) = {
    if (var_guard36 != 0.0) {
        let assign5620_e4277: f64 = (p.p260 * var_iwe);
        let assign5620_e4278: f64 = (1.0 + assign5620_e4277);
        let (assign5620_e4287,) = {
            if (assign5620_e4278 > 0.001) {
                let assign5620_e4284: f64 = (p.p260 * var_iwe);
                let assign5620_e4285: f64 = (1.0 + assign5620_e4284);
                (assign5620_e4285,)
            } else {
                (0.001,)
            }
        };
        let assign5620_e4288: f64 = (p.p259 * assign5620_e4287);
        (assign5620_e4288,)
    } else {
        (var_lp1e,)
    }
};
        var_lp1e = assign5620_e4290;

        let (assign5630_e4322,) = {
    if (var_guard36 != 0.0) {
        let assign5630_e4295: f64 = (var_fbet1e * var_lp1e);
        let assign5630_e4297: f64 = (assign5630_e4295 / var_le);
        let assign5630_e4300: f64 = (-var_le);
        let assign5630_e4302: f64 = (assign5630_e4300 / var_lp1e);
        let assign5630_e4303: f64 = (assign5630_e4302).exp();
        let assign5630_e4304: f64 = (1.0 - assign5630_e4303);
        let assign5630_e4305: f64 = (assign5630_e4297 * assign5630_e4304);
        let assign5630_e4306: f64 = (1.0 + assign5630_e4305);
        let assign5630_e4309: f64 = (p.p261 * p.p262);
        let assign5630_e4311: f64 = (assign5630_e4309 / var_le);
        let assign5630_e4314: f64 = (-var_le);
        let assign5630_e4316: f64 = (assign5630_e4314 / p.p262);
        let assign5630_e4317: f64 = (assign5630_e4316).exp();
        let assign5630_e4318: f64 = (1.0 - assign5630_e4317);
        let assign5630_e4319: f64 = (assign5630_e4311 * assign5630_e4318);
        let assign5630_e4320: f64 = (assign5630_e4306 + assign5630_e4319);
        (assign5630_e4320,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5630_e4322;

        let (assign5640_e4331,) = {
    if (var_guard36 != 0.0) {
        let (assign5640_e4329,) = {
            if (var_gpe > 1e-15) {
                (var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5640_e4329,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5640_e4331;

        let (assign5650_e4350,) = {
    if (var_guard36 != 0.0) {
        let assign5650_e4336: f64 = (p.p263 * var_iwe);
        let assign5650_e4337: f64 = (1.0 + assign5650_e4336);
        let assign5650_e4340: f64 = (p.p264 * var_iwe);
        let assign5650_e4344: f64 = (var_we / p.p265);
        let assign5650_e4345: f64 = (1.0 + assign5650_e4344);
        let assign5650_e4346: f64 = (assign5650_e4345).ln();
        let assign5650_e4347: f64 = (assign5650_e4340 * assign5650_e4346);
        let assign5650_e4348: f64 = (assign5650_e4337 + assign5650_e4347);
        (assign5650_e4348,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign5650_e4350;

        let (assign5660_e4362,) = {
    if (var_guard36 != 0.0) {
        let assign5660_e4354: f64 = (p.p256 * var_we);
        let assign5660_e4357: f64 = (var_gpe * var_le);
        let assign5660_e4358: f64 = (assign5660_e4354 / assign5660_e4357);
        let assign5660_e4360: f64 = (assign5660_e4358 * var_gwe);
        (assign5660_e4360,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign5660_e4362;

        let (assign5670_e4378,) = {
    if (var_guard36 != 0.0) {
        let assign5670_e4367: f64 = (p.p267 * var_ile);
        let assign5670_e4368: f64 = (p.p266 + assign5670_e4367);
        let assign5670_e4371: f64 = (p.p268 * var_iwe);
        let assign5670_e4372: f64 = (assign5670_e4368 + assign5670_e4371);
        let assign5670_e4375: f64 = (p.p269 * var_iae);
        let assign5670_e4376: f64 = (assign5670_e4372 + assign5670_e4375);
        (assign5670_e4376,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign5670_e4378;

        let (assign5680_e4388,) = {
    if (var_guard36 != 0.0) {
        let assign5680_e4384: f64 = (p.p271 * var_iwe);
        let assign5680_e4385: f64 = (1.0 + assign5680_e4384);
        let assign5680_e4386: f64 = (p.p270 * assign5680_e4385);
        (assign5680_e4386,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign5680_e4388;

        let (assign5690_e4392,) = {
    if (var_guard36 != 0.0) {
        (p.p272,)
    } else {
        (var_stmue_p,)
    }
};
        var_stmue_p = assign5690_e4392;

        let (assign5700_e4396,) = {
    if (var_guard36 != 0.0) {
        (p.p273,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign5700_e4396;

        let (assign5710_e4400,) = {
    if (var_guard36 != 0.0) {
        (p.p274,)
    } else {
        (var_stthemu_p,)
    }
};
        var_stthemu_p = assign5710_e4400;

        let (assign5720_e4422,) = {
    if (var_guard36 != 0.0) {
        let assign5720_e4406: f64 = (var_ile).powf(p.p277);
        let assign5720_e4407: f64 = (p.p276 * assign5720_e4406);
        let assign5720_e4408: f64 = (p.p275 + assign5720_e4407);
        let assign5720_e4412: f64 = (p.p278 * var_iwe);
        let assign5720_e4413: f64 = (1.0 + assign5720_e4412);
        let assign5720_e4414: f64 = (assign5720_e4408 * assign5720_e4413);
        let assign5720_e4418: f64 = (p.p279 * var_iae);
        let assign5720_e4419: f64 = (1.0 + assign5720_e4418);
        let assign5720_e4420: f64 = (assign5720_e4414 * assign5720_e4419);
        (assign5720_e4420,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign5720_e4422;

        let (assign5730_e4426,) = {
    if (var_guard36 != 0.0) {
        (p.p280,)
    } else {
        (var_stcs_p,)
    }
};
        var_stcs_p = assign5730_e4426;

        let (assign5740_e4430,) = {
    if (var_guard36 != 0.0) {
        (p.p281,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign5740_e4430;

        let (assign5750_e4434,) = {
    if (var_guard36 != 0.0) {
        (p.p282,)
    } else {
        (var_stthecs_p,)
    }
};
        var_stthecs_p = assign5750_e4434;

        let (assign5760_e4456,) = {
    if (var_guard36 != 0.0) {
        let assign5760_e4440: f64 = (p.p284 * var_ile);
        let assign5760_e4441: f64 = (1.0 + assign5760_e4440);
        let assign5760_e4442: f64 = (p.p283 * assign5760_e4441);
        let assign5760_e4446: f64 = (p.p285 * var_iwe);
        let assign5760_e4447: f64 = (1.0 + assign5760_e4446);
        let assign5760_e4448: f64 = (assign5760_e4442 * assign5760_e4447);
        let assign5760_e4452: f64 = (p.p286 * var_iae);
        let assign5760_e4453: f64 = (1.0 + assign5760_e4452);
        let assign5760_e4454: f64 = (assign5760_e4448 * assign5760_e4453);
        (assign5760_e4454,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign5760_e4456;

        let (assign5770_e4460,) = {
    if (var_guard36 != 0.0) {
        (p.p287,)
    } else {
        (var_stxcor_p,)
    }
};
        var_stxcor_p = assign5770_e4460;

        let (assign5780_e4464,) = {
    if (var_guard36 != 0.0) {
        (p.p288,)
    } else {
        (var_feta_p,)
    }
};
        var_feta_p = assign5780_e4464;

        let (assign5790_e4476,) = {
    if (var_guard36 != 0.0) {
        let assign5790_e4468: f64 = (p.p289 * var_iwe);
        let assign5790_e4472: f64 = (p.p290 * var_iwe);
        let assign5790_e4473: f64 = (1.0 + assign5790_e4472);
        let assign5790_e4474: f64 = (assign5790_e4468 * assign5790_e4473);
        (assign5790_e4474,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign5790_e4476;

        let (assign5800_e4480,) = {
    if (var_guard36 != 0.0) {
        (p.p291,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign5800_e4480;

        let (assign5810_e4484,) = {
    if (var_guard36 != 0.0) {
        (p.p292,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign5810_e4484;

        let (assign5820_e4488,) = {
    if (var_guard36 != 0.0) {
        (p.p293,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign5820_e4488;

        let (assign5830_e4514,) = {
    if (var_guard36 != 0.0) {
        let assign5830_e4493: f64 = (p.p295 * var_gwe);
        let assign5830_e4495: f64 = (assign5830_e4493 / var_gpe);
        let assign5830_e4498: f64 = (var_ile).powf(p.p296);
        let assign5830_e4499: f64 = (assign5830_e4495 * assign5830_e4498);
        let assign5830_e4500: f64 = (p.p294 + assign5830_e4499);
        let assign5830_e4504: f64 = (p.p297 * var_iwe);
        let assign5830_e4505: f64 = (1.0 + assign5830_e4504);
        let assign5830_e4506: f64 = (assign5830_e4500 * assign5830_e4505);
        let assign5830_e4510: f64 = (p.p298 * var_iae);
        let assign5830_e4511: f64 = (1.0 + assign5830_e4510);
        let assign5830_e4512: f64 = (assign5830_e4506 * assign5830_e4511);
        (assign5830_e4512,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign5830_e4514;

        let (assign5840_e4530,) = {
    if (var_guard36 != 0.0) {
        let assign5840_e4519: f64 = (p.p300 * var_ile);
        let assign5840_e4520: f64 = (p.p299 + assign5840_e4519);
        let assign5840_e4523: f64 = (p.p301 * var_iwe);
        let assign5840_e4524: f64 = (assign5840_e4520 + assign5840_e4523);
        let assign5840_e4527: f64 = (p.p302 * var_iae);
        let assign5840_e4528: f64 = (assign5840_e4524 + assign5840_e4527);
        (assign5840_e4528,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign5840_e4530;

        let (assign5850_e4534,) = {
    if (var_guard36 != 0.0) {
        (p.p303,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign5850_e4534;

        let (assign5860_e4538,) = {
    if (var_guard36 != 0.0) {
        (p.p304,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign5860_e4538;

        let (assign5870_e4542,) = {
    if (var_guard36 != 0.0) {
        (p.p305,)
    } else {
        (var_thesatt_p,)
    }
};
        var_thesatt_p = assign5870_e4542;

        let (assign5880_e4552,) = {
    if (var_guard36 != 0.0) {
        let assign5880_e4548: f64 = (p.p307 * var_ile);
        let assign5880_e4549: f64 = (1.0 + assign5880_e4548);
        let assign5880_e4550: f64 = (p.p306 / assign5880_e4549);
        (assign5880_e4550,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign5880_e4552;

        let (assign5890_e4566,) = {
    if (var_guard36 != 0.0) {
        let assign5890_e4557: f64 = (var_ile).powf(p.p309);
        let assign5890_e4558: f64 = (p.p308 * assign5890_e4557);
        let assign5890_e4562: f64 = (p.p310 * var_iwe);
        let assign5890_e4563: f64 = (1.0 + assign5890_e4562);
        let assign5890_e4564: f64 = (assign5890_e4558 * assign5890_e4563);
        (assign5890_e4564,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign5890_e4566;

        let (assign5900_e4572,) = {
    if (var_guard36 != 0.0) {
        let assign5900_e4570: f64 = (var_ile).powf(p.p312);
        (assign5900_e4570,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign5900_e4572;

        let (assign5910_e4592,) = {
    if (var_guard36 != 0.0) {
        let assign5910_e4576: f64 = (p.p311 * var_tmpx);
        let assign5910_e4580: f64 = (p.p314 * var_iwe);
        let assign5910_e4581: f64 = (1.0 + assign5910_e4580);
        let assign5910_e4582: f64 = (assign5910_e4576 * assign5910_e4581);
        let assign5910_e4586: f64 = (p.p313 * var_ile);
        let assign5910_e4588: f64 = (assign5910_e4586 * var_tmpx);
        let assign5910_e4589: f64 = (1.0 + assign5910_e4588);
        let assign5910_e4590: f64 = (assign5910_e4582 / assign5910_e4589);
        (assign5910_e4590,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign5910_e4592;

        let (assign5920_e4598,) = {
    if (var_guard36 != 0.0) {
        let assign5920_e4596: f64 = (var_ile).powf(p.p316);
        (assign5920_e4596,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign5920_e4598;

        let (assign5930_e4618,) = {
    if (var_guard36 != 0.0) {
        let assign5930_e4602: f64 = (p.p315 * var_tmpx);
        let assign5930_e4606: f64 = (p.p318 * var_iwe);
        let assign5930_e4607: f64 = (1.0 + assign5930_e4606);
        let assign5930_e4608: f64 = (assign5930_e4602 * assign5930_e4607);
        let assign5930_e4612: f64 = (p.p317 * var_ile);
        let assign5930_e4614: f64 = (assign5930_e4612 * var_tmpx);
        let assign5930_e4615: f64 = (1.0 + assign5930_e4614);
        let assign5930_e4616: f64 = (assign5930_e4608 / assign5930_e4615);
        (assign5930_e4616,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign5930_e4618;

        let (assign5940_e4622,) = {
    if (var_guard36 != 0.0) {
        (p.p319,)
    } else {
        (var_vp_p,)
    }
};
        var_vp_p = assign5940_e4622;

        let (assign5950_e4638,) = {
    if (var_guard36 != 0.0) {
        let assign5950_e4628: f64 = (p.p321 * var_ile);
        let assign5950_e4629: f64 = (1.0 + assign5950_e4628);
        let assign5950_e4630: f64 = (p.p320 * assign5950_e4629);
        let assign5950_e4634: f64 = (p.p322 * var_iwe);
        let assign5950_e4635: f64 = (1.0 + assign5950_e4634);
        let assign5950_e4636: f64 = (assign5950_e4630 * assign5950_e4635);
        (assign5950_e4636,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign5950_e4638;

        let (assign5960_e4642,) = {
    if (var_guard36 != 0.0) {
        (p.p323,)
    } else {
        (var_a2_p,)
    }
};
        var_a2_p = assign5960_e4642;

        let (assign5970_e4646,) = {
    if (var_guard36 != 0.0) {
        (p.p324,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign5970_e4646;

        let (assign5980_e4662,) = {
    if (var_guard36 != 0.0) {
        let assign5980_e4652: f64 = (p.p326 * var_ile);
        let assign5980_e4653: f64 = (1.0 + assign5980_e4652);
        let assign5980_e4654: f64 = (p.p325 * assign5980_e4653);
        let assign5980_e4658: f64 = (p.p327 * var_iwe);
        let assign5980_e4659: f64 = (1.0 + assign5980_e4658);
        let assign5980_e4660: f64 = (assign5980_e4654 * assign5980_e4659);
        (assign5980_e4660,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign5980_e4662;

        let (assign5990_e4678,) = {
    if (var_guard36 != 0.0) {
        let assign5990_e4668: f64 = (p.p329 * var_ile);
        let assign5990_e4669: f64 = (1.0 + assign5990_e4668);
        let assign5990_e4670: f64 = (p.p328 * assign5990_e4669);
        let assign5990_e4674: f64 = (p.p330 * var_iwe);
        let assign5990_e4675: f64 = (1.0 + assign5990_e4674);
        let assign5990_e4676: f64 = (assign5990_e4670 * assign5990_e4675);
        (assign5990_e4676,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign5990_e4678;

        let (assign6000_e4682,) = {
    if (var_guard36 != 0.0) {
        (p.p331,)
    } else {
        (var_imaxii_p,)
    }
};
        var_imaxii_p = assign6000_e4682;

        let (assign6010_e4686,) = {
    if (var_guard36 != 0.0) {
        (p.p332,)
    } else {
        (var_gco_p,)
    }
};
        var_gco_p = assign6010_e4686;

        *var_a1_p_slot = var_a1_p;
        *var_a2_p_slot = var_a2_p;
        *var_a3_p_slot = var_a3_p;
        *var_a4_p_slot = var_a4_p;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp_p_slot = var_alp_p;
        *var_ax_p_slot = var_ax_p;
        *var_betn_p_slot = var_betn_p;
        *var_cs_p_slot = var_cs_p;
        *var_fbet1e_slot = var_fbet1e;
        *var_feta_p_slot = var_feta_p;
        *var_gco_p_slot = var_gco_p;
        *var_gpe_slot = var_gpe;
        *var_gwe_slot = var_gwe;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_lp1e_slot = var_lp1e;
        *var_mue_p_slot = var_mue_p;
        *var_rs_p_slot = var_rs_p;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsg_p_slot = var_rsg_p;
        *var_sta2_p_slot = var_sta2_p;
        *var_stbet_p_slot = var_stbet_p;
        *var_stcs_p_slot = var_stcs_p;
        *var_stmue_p_slot = var_stmue_p;
        *var_strs_p_slot = var_strs_p;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_thecs_p_slot = var_thecs_p;
        *var_themu_p_slot = var_themu_p;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_tmpx_slot = var_tmpx;
        *var_vp_p_slot = var_vp_p;
        *var_xcor_p_slot = var_xcor_p;
    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_gpe: f64,
        var_guard36: f64,
        var_gwe: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_wecv: f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axacl_i_slot: &mut f64,
        var_axaco_i_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_guard39_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
    ) {
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axacl_i: f64 = *var_axacl_i_slot;
        let mut var_axaco_i: f64 = *var_axaco_i_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_guard39: f64 = *var_guard39_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;

        let (assign6020_e4692,) = {
    if (var_guard36 != 0.0) {
        let assign6020_e4690: f64 = (p.p333 / var_iae);
        (assign6020_e4690,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign6020_e4692;

        let (assign6030_e4702,) = {
    if (var_guard36 != 0.0) {
        let assign6030_e4696: f64 = (p.p334 * p.p234);
        let assign6030_e4699: f64 = (1e-6 * var_iwe);
        let assign6030_e4700: f64 = (assign6030_e4696 / assign6030_e4699);
        (assign6030_e4700,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign6030_e4702;

        let (assign6040_e4712,) = {
    if (var_guard36 != 0.0) {
        let assign6040_e4706: f64 = (p.p335 * p.p235);
        let assign6040_e4709: f64 = (1e-6 * var_iwe);
        let assign6040_e4710: f64 = (assign6040_e4706 / assign6040_e4709);
        (assign6040_e4710,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign6040_e4712;

        let (assign6050_e4716,) = {
    if (var_guard36 != 0.0) {
        (p.p336,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign6050_e4716;

        let (assign6060_e4720,) = {
    if (var_guard36 != 0.0) {
        (p.p337,)
    } else {
        (var_gc2_p,)
    }
};
        var_gc2_p = assign6060_e4720;

        let (assign6070_e4724,) = {
    if (var_guard36 != 0.0) {
        (p.p338,)
    } else {
        (var_gc3_p,)
    }
};
        var_gc3_p = assign6070_e4724;

        let (assign6080_e4728,) = {
    if (var_guard36 != 0.0) {
        (p.p337,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6080_e4728;

        let assign6090_e4730: f64 = if param_given[339] { 1.0 } else { 0.0 };
        let assign6090_e4732: f64 = if assign6090_e4730 == 1.0 { 1.0 } else { 0.0 };
        var_guard39 = assign6090_e4732;

        let (assign6100_e4738,) = {
    if ((var_guard36 != 0.0) && (var_guard39 != 0.0)) {
        (p.p339,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6100_e4738;

        let (assign6110_e4742,) = {
    if (var_guard36 != 0.0) {
        (p.p338,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6110_e4742;

        let assign6120_e4744: f64 = if param_given[340] { 1.0 } else { 0.0 };
        let assign6120_e4746: f64 = if assign6120_e4744 == 1.0 { 1.0 } else { 0.0 };
        var_guard40 = assign6120_e4746;

        let (assign6130_e4752,) = {
    if ((var_guard36 != 0.0) && (var_guard40 != 0.0)) {
        (p.p340,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6130_e4752;

        let (assign6140_e4756,) = {
    if (var_guard36 != 0.0) {
        (var_gc2ov_p,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6140_e4756;

        let assign6150_e4758: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6150_e4760: f64 = if assign6150_e4758 == 1.0 { 1.0 } else { 0.0 };
        var_guard41 = assign6150_e4760;

        let (assign6160_e4766,) = {
    if ((var_guard36 != 0.0) && (var_guard41 != 0.0)) {
        (p.p341,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6160_e4766;

        let (assign6170_e4770,) = {
    if (var_guard36 != 0.0) {
        (var_gc3ov_p,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6170_e4770;

        let assign6180_e4772: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6180_e4774: f64 = if assign6180_e4772 == 1.0 { 1.0 } else { 0.0 };
        var_guard42 = assign6180_e4774;

        let (assign6190_e4780,) = {
    if ((var_guard36 != 0.0) && (var_guard42 != 0.0)) {
        (p.p342,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6190_e4780;

        let (assign6200_e4784,) = {
    if (var_guard36 != 0.0) {
        (p.p343,)
    } else {
        (var_chib_p,)
    }
};
        var_chib_p = assign6200_e4784;

        let (assign6210_e4794,) = {
    if (var_guard36 != 0.0) {
        let assign6210_e4788: f64 = (p.p344 * p.p234);
        let assign6210_e4791: f64 = (1e-6 * var_iwe);
        let assign6210_e4792: f64 = (assign6210_e4788 / assign6210_e4791);
        (assign6210_e4792,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign6210_e4794;

        let (assign6220_e4804,) = {
    if (var_guard36 != 0.0) {
        let assign6220_e4798: f64 = (p.p345 * p.p235);
        let assign6220_e4801: f64 = (1e-6 * var_iwe);
        let assign6220_e4802: f64 = (assign6220_e4798 / assign6220_e4801);
        (assign6220_e4802,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign6220_e4804;

        let (assign6230_e4808,) = {
    if (var_guard36 != 0.0) {
        (p.p346,)
    } else {
        (var_bgidl_p,)
    }
};
        var_bgidl_p = assign6230_e4808;

        let (assign6240_e4812,) = {
    if (var_guard36 != 0.0) {
        (p.p347,)
    } else {
        (var_bgidld_p,)
    }
};
        var_bgidld_p = assign6240_e4812;

        let (assign6250_e4816,) = {
    if (var_guard36 != 0.0) {
        (p.p348,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign6250_e4816;

        let (assign6260_e4820,) = {
    if (var_guard36 != 0.0) {
        (p.p349,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign6260_e4820;

        let (assign6270_e4824,) = {
    if (var_guard36 != 0.0) {
        (p.p350,)
    } else {
        (var_cgidl_p,)
    }
};
        var_cgidl_p = assign6270_e4824;

        let (assign6280_e4828,) = {
    if (var_guard36 != 0.0) {
        (p.p351,)
    } else {
        (var_cgidld_p,)
    }
};
        var_cgidld_p = assign6280_e4828;

        let (assign6290_e4840,) = {
    if (var_guard36 != 0.0) {
        let assign6290_e4832: f64 = (8.8541878176e-12 * p.p207);
        let assign6290_e4834: f64 = (assign6290_e4832 * var_wecv);
        let assign6290_e4836: f64 = (assign6290_e4834 * var_lecv);
        let assign6290_e4838: f64 = (assign6290_e4836 / p.p206);
        (assign6290_e4838,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign6290_e4840;

        let (assign6300_e4852,) = {
    if (var_guard36 != 0.0) {
        let assign6300_e4844: f64 = (8.8541878176e-12 * p.p207);
        let assign6300_e4846: f64 = (assign6300_e4844 * var_wecv);
        let assign6300_e4848: f64 = (assign6300_e4846 * p.p234);
        let assign6300_e4850: f64 = (assign6300_e4848 / p.p232);
        (assign6300_e4850,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign6300_e4852;

        let (assign6310_e4864,) = {
    if (var_guard36 != 0.0) {
        let assign6310_e4856: f64 = (8.8541878176e-12 * p.p207);
        let assign6310_e4858: f64 = (assign6310_e4856 * var_wecv);
        let assign6310_e4860: f64 = (assign6310_e4858 * p.p235);
        let assign6310_e4862: f64 = (assign6310_e4860 / p.p233);
        (assign6310_e4862,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign6310_e4864;

        let (assign6320_e4882,) = {
    if (var_guard36 != 0.0) {
        let assign6320_e4870: f64 = (var_ile).powf(p.p354);
        let assign6320_e4871: f64 = (p.p353 * assign6320_e4870);
        let assign6320_e4872: f64 = (p.p352 + assign6320_e4871);
        let assign6320_e4875: f64 = (p.p355 * var_iwe);
        let assign6320_e4876: f64 = (assign6320_e4872 + assign6320_e4875);
        let assign6320_e4879: f64 = (p.p356 * var_iae);
        let assign6320_e4880: f64 = (assign6320_e4876 + assign6320_e4879);
        (assign6320_e4880,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign6320_e4882;

        let (assign6330_e4898,) = {
    if (var_guard36 != 0.0) {
        let assign6330_e4887: f64 = (p.p358 * var_ile);
        let assign6330_e4888: f64 = (p.p357 + assign6330_e4887);
        let assign6330_e4891: f64 = (p.p359 * var_iwe);
        let assign6330_e4892: f64 = (assign6330_e4888 + assign6330_e4891);
        let assign6330_e4895: f64 = (p.p360 * var_iae);
        let assign6330_e4896: f64 = (assign6330_e4892 + assign6330_e4895);
        (assign6330_e4896,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign6330_e4898;

        let (assign6340_e4902,) = {
    if (var_guard36 != 0.0) {
        (p.p294,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6340_e4902;

        let assign6350_e4904: f64 = if param_given[361] { 1.0 } else { 0.0 };
        let assign6350_e4906: f64 = if assign6350_e4904 == 1.0 { 1.0 } else { 0.0 };
        var_guard43 = assign6350_e4906;

        let (assign6360_e4912,) = {
    if ((var_guard36 != 0.0) && (var_guard43 != 0.0)) {
        (p.p361,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6360_e4912;

        let (assign6370_e4916,) = {
    if (var_guard36 != 0.0) {
        (p.p295,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6370_e4916;

        let assign6380_e4918: f64 = if param_given[362] { 1.0 } else { 0.0 };
        let assign6380_e4920: f64 = if assign6380_e4918 == 1.0 { 1.0 } else { 0.0 };
        var_guard44 = assign6380_e4920;

        let (assign6390_e4926,) = {
    if ((var_guard36 != 0.0) && (var_guard44 != 0.0)) {
        (p.p362,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6390_e4926;

        let (assign6400_e4930,) = {
    if (var_guard36 != 0.0) {
        (p.p296,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6400_e4930;

        let assign6410_e4932: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6410_e4934: f64 = if assign6410_e4932 == 1.0 { 1.0 } else { 0.0 };
        var_guard45 = assign6410_e4934;

        let (assign6420_e4940,) = {
    if ((var_guard36 != 0.0) && (var_guard45 != 0.0)) {
        (p.p363,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6420_e4940;

        let (assign6430_e4944,) = {
    if (var_guard36 != 0.0) {
        (p.p297,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6430_e4944;

        let assign6440_e4946: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6440_e4948: f64 = if assign6440_e4946 == 1.0 { 1.0 } else { 0.0 };
        var_guard46 = assign6440_e4948;

        let (assign6450_e4954,) = {
    if ((var_guard36 != 0.0) && (var_guard46 != 0.0)) {
        (p.p364,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6450_e4954;

        let (assign6460_e4958,) = {
    if (var_guard36 != 0.0) {
        (p.p298,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6460_e4958;

        let assign6470_e4960: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6470_e4962: f64 = if assign6470_e4960 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign6470_e4962;

        let (assign6480_e4968,) = {
    if ((var_guard36 != 0.0) && (var_guard47 != 0.0)) {
        (p.p365,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6480_e4968;

        let (assign6490_e4994,) = {
    if (var_guard36 != 0.0) {
        let assign6490_e4973: f64 = (var_thesatacl_i * var_gwe);
        let assign6490_e4975: f64 = (assign6490_e4973 / var_gpe);
        let assign6490_e4978: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign6490_e4979: f64 = (assign6490_e4975 * assign6490_e4978);
        let assign6490_e4980: f64 = (var_thesataco_i + assign6490_e4979);
        let assign6490_e4984: f64 = (var_thesatacw_i * var_iwe);
        let assign6490_e4985: f64 = (1.0 + assign6490_e4984);
        let assign6490_e4986: f64 = (assign6490_e4980 * assign6490_e4985);
        let assign6490_e4990: f64 = (var_thesataclw_i * var_iae);
        let assign6490_e4991: f64 = (1.0 + assign6490_e4990);
        let assign6490_e4992: f64 = (assign6490_e4986 * assign6490_e4991);
        (assign6490_e4992,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign6490_e4994;

        let (assign6500_e4998,) = {
    if (var_guard36 != 0.0) {
        (p.p306,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6500_e4998;

        let assign6510_e5000: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6510_e5002: f64 = if assign6510_e5000 == 1.0 { 1.0 } else { 0.0 };
        var_guard48 = assign6510_e5002;

        let (assign6520_e5008,) = {
    if ((var_guard36 != 0.0) && (var_guard48 != 0.0)) {
        (p.p366,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6520_e5008;

        let (assign6530_e5012,) = {
    if (var_guard36 != 0.0) {
        (p.p307,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6530_e5012;

        let assign6540_e5014: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6540_e5016: f64 = if assign6540_e5014 == 1.0 { 1.0 } else { 0.0 };
        var_guard49 = assign6540_e5016;

        let (assign6550_e5022,) = {
    if ((var_guard36 != 0.0) && (var_guard49 != 0.0)) {
        (p.p367,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6550_e5022;

        let (assign6560_e5032,) = {
    if (var_guard36 != 0.0) {
        let assign6560_e5028: f64 = (var_axacl_i * var_ile);
        let assign6560_e5029: f64 = (1.0 + assign6560_e5028);
        let assign6560_e5030: f64 = (var_axaco_i / assign6560_e5029);
        (assign6560_e5030,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign6560_e5032;

        let (assign6570_e5046,) = {
    if (var_guard36 != 0.0) {
        let assign6570_e5037: f64 = (var_ile).powf(p.p369);
        let assign6570_e5038: f64 = (p.p368 * assign6570_e5037);
        let assign6570_e5042: f64 = (p.p370 * var_iwe);
        let assign6570_e5043: f64 = (1.0 + assign6570_e5042);
        let assign6570_e5044: f64 = (assign6570_e5038 * assign6570_e5043);
        (assign6570_e5044,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign6570_e5046;

        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_alpac_p_slot = var_alpac_p;
        *var_axac_p_slot = var_axac_p;
        *var_axacl_i_slot = var_axacl_i;
        *var_axaco_i_slot = var_axaco_i;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_chib_p_slot = var_chib_p;
        *var_cox_p_slot = var_cox_p;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_guard39_slot = var_guard39;
        *var_guard40_slot = var_guard40;
        *var_guard41_slot = var_guard41;
        *var_guard42_slot = var_guard42;
        *var_guard43_slot = var_guard43;
        *var_guard44_slot = var_guard44;
        *var_guard45_slot = var_guard45;
        *var_guard46_slot = var_guard46;
        *var_guard47_slot = var_guard47;
        *var_guard48_slot = var_guard48;
        *var_guard49_slot = var_guard49;
        *var_iginv_p_slot = var_iginv_p;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stig_p_slot = var_stig_p;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesatacw_i_slot = var_thesatacw_i;
    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        var_betn_p: f64,
        var_guard36: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_l_f: f64,
        var_l_slif: f64,
        var_le: f64,
        var_nf_i: f64,
        var_ngcon_i: f64,
        var_w_f: f64,
        var_we: f64,
        var_xgwe: f64,
        var_alp1ac_p_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fntexc_p_slot: &mut f64,
        var_gpe_edge_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_kuowe_slot: &mut f64,
        var_kvthowe_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_rde_p_slot: &mut f64,
        var_rg_p_slot: &mut f64,
        var_rse_p_slot: &mut f64,
        var_rsh_i_slot: &mut f64,
        var_rshd_i_slot: &mut f64,
        var_rwell_p_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_we_edge_slot: &mut f64,
    ) {
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fntexc_p: f64 = *var_fntexc_p_slot;
        let mut var_gpe_edge: f64 = *var_gpe_edge_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_kuowe: f64 = *var_kuowe_slot;
        let mut var_kvthowe: f64 = *var_kvthowe_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_rde_p: f64 = *var_rde_p_slot;
        let mut var_rg_p: f64 = *var_rg_p_slot;
        let mut var_rse_p: f64 = *var_rse_p_slot;
        let mut var_rsh_i: f64 = *var_rsh_i_slot;
        let mut var_rshd_i: f64 = *var_rshd_i_slot;
        let mut var_rwell_p: f64 = *var_rwell_p_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;

        let (assign6580_e5052,) = {
    if (var_guard36 != 0.0) {
        let assign6580_e5050: f64 = (var_ile).powf(p.p372);
        (assign6580_e5050,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6580_e5052;

        let (assign6590_e5072,) = {
    if (var_guard36 != 0.0) {
        let assign6590_e5056: f64 = (p.p371 * var_tmpx);
        let assign6590_e5060: f64 = (p.p374 * var_iwe);
        let assign6590_e5061: f64 = (1.0 + assign6590_e5060);
        let assign6590_e5062: f64 = (assign6590_e5056 * assign6590_e5061);
        let assign6590_e5066: f64 = (p.p373 * var_ile);
        let assign6590_e5068: f64 = (assign6590_e5066 * var_tmpx);
        let assign6590_e5069: f64 = (1.0 + assign6590_e5068);
        let assign6590_e5070: f64 = (assign6590_e5062 / assign6590_e5069);
        (assign6590_e5070,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign6590_e5072;

        let (assign6600_e5076,) = {
    if (var_guard36 != 0.0) {
        (p.p375,)
    } else {
        (var_fcgovacc_p,)
    }
};
        var_fcgovacc_p = assign6600_e5076;

        let (assign6610_e5080,) = {
    if (var_guard36 != 0.0) {
        (p.p376,)
    } else {
        (var_fcgovaccd_p,)
    }
};
        var_fcgovaccd_p = assign6610_e5080;

        let (assign6620_e5084,) = {
    if (var_guard36 != 0.0) {
        (p.p377,)
    } else {
        (var_cgovaccg_p,)
    }
};
        var_cgovaccg_p = assign6620_e5084;

        let (assign6630_e5090,) = {
    if (var_guard36 != 0.0) {
        let assign6630_e5088: f64 = (p.p378 * var_iilcv);
        (assign6630_e5088,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign6630_e5090;

        let (assign6640_e5096,) = {
    if (var_guard36 != 0.0) {
        let assign6640_e5094: f64 = (p.p379 * var_iiwecv);
        (assign6640_e5094,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign6640_e5096;

        let (assign6650_e5102,) = {
    if (var_guard36 != 0.0) {
        let assign6650_e5100: f64 = (p.p380 * var_iiwecv);
        (assign6650_e5100,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign6650_e5102;

        let (assign6660_e5106,) = {
    if (var_guard36 != 0.0) {
        (p.p381,)
    } else {
        (var_dvfbinr_p,)
    }
};
        var_dvfbinr_p = assign6660_e5106;

        let (assign6670_e5110,) = {
    if (var_guard36 != 0.0) {
        (p.p382,)
    } else {
        (var_fcinrdep_p,)
    }
};
        var_fcinrdep_p = assign6670_e5110;

        let (assign6680_e5114,) = {
    if (var_guard36 != 0.0) {
        (p.p383,)
    } else {
        (var_fcinracc_p,)
    }
};
        var_fcinracc_p = assign6680_e5114;

        let (assign6690_e5118,) = {
    if (var_guard36 != 0.0) {
        (p.p384,)
    } else {
        (var_axinr_p,)
    }
};
        var_axinr_p = assign6690_e5118;

        let (assign6700_e5124,) = {
    if (var_guard36 != 0.0) {
        let assign6700_e5122: f64 = (p.p385 * var_iiwcv);
        (assign6700_e5122,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign6700_e5124;

        let (assign6710_e5130,) = {
    if (var_guard36 != 0.0) {
        let assign6710_e5128: f64 = (p.p386 * var_iiwcv);
        (assign6710_e5128,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign6710_e5130;

        let (assign6720_e5140,) = {
    if (var_guard36 != 0.0) {
        let assign6720_e5135: f64 = (2.0 * p.p393);
        let assign6720_e5137: f64 = (assign6720_e5135 / var_le);
        let assign6720_e5138: f64 = (1.0 - assign6720_e5137);
        (assign6720_e5138,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign6720_e5140;

        let (assign6750_e5161,) = {
    if (var_guard36 != 0.0) {
        (p.p387,)
    } else {
        (var_fnt_p,)
    }
};
        var_fnt_p = assign6750_e5161;

        let (assign6760_e5173,) = {
    if (var_guard36 != 0.0) {
        let assign6760_e5165: f64 = (p.p388 * var_betn_p);
        let assign6760_e5167: f64 = (assign6760_e5165 * var_betn_p);
        let assign6760_e5169: f64 = (assign6760_e5167 * var_iwe);
        let assign6760_e5171: f64 = (assign6760_e5169 * var_iwe);
        (assign6760_e5171,)
    } else {
        (var_fntexc_p,)
    }
};
        var_fntexc_p = assign6760_e5173;

        let (assign6810_e5211,) = {
    if (var_guard36 != 0.0) {
        let assign6810_e5205: f64 = (2.0 * p.p395);
        let assign6810_e5208: f64 = (p.p396 * var_we);
        let assign6810_e5209: f64 = (assign6810_e5205 + assign6810_e5208);
        (assign6810_e5209,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign6810_e5211;

        let (assign6840_e5227,) = {
    if (var_guard36 != 0.0) {
        (p.p397,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign6840_e5227;

        let (assign6850_e5243,) = {
    if (var_guard36 != 0.0) {
        let assign6850_e5232: f64 = (p.p399 * var_ile);
        let assign6850_e5233: f64 = (p.p398 + assign6850_e5232);
        let assign6850_e5236: f64 = (p.p400 * var_iwe);
        let assign6850_e5237: f64 = (assign6850_e5233 + assign6850_e5236);
        let assign6850_e5240: f64 = (p.p401 * var_iae);
        let assign6850_e5241: f64 = (assign6850_e5237 + assign6850_e5240);
        (assign6850_e5241,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign6850_e5243;

        let (assign6860_e5261,) = {
    if (var_guard36 != 0.0) {
        let assign6860_e5249: f64 = (var_ile).powf(p.p404);
        let assign6860_e5250: f64 = (p.p403 * assign6860_e5249);
        let assign6860_e5251: f64 = (p.p402 + assign6860_e5250);
        let assign6860_e5254: f64 = (p.p405 * var_iwe);
        let assign6860_e5255: f64 = (assign6860_e5251 + assign6860_e5254);
        let assign6860_e5258: f64 = (p.p406 * var_iae);
        let assign6860_e5259: f64 = (assign6860_e5255 + assign6860_e5258);
        (assign6860_e5259,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign6860_e5261;

        let (assign6870_e5285,) = {
    if (var_guard36 != 0.0) {
        let assign6870_e5268: f64 = (var_ile).powf(p.p409);
        let assign6870_e5269: f64 = (p.p408 * assign6870_e5268);
        let assign6870_e5270: f64 = (1.0 + assign6870_e5269);
        let assign6870_e5271: f64 = (p.p407 * assign6870_e5270);
        let assign6870_e5275: f64 = (p.p410 * var_iwe);
        let assign6870_e5276: f64 = (1.0 + assign6870_e5275);
        let assign6870_e5277: f64 = (assign6870_e5271 * assign6870_e5276);
        let assign6870_e5281: f64 = (p.p411 * var_iae);
        let assign6870_e5282: f64 = (1.0 + assign6870_e5281);
        let assign6870_e5283: f64 = (assign6870_e5277 * assign6870_e5282);
        (assign6870_e5283,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign6870_e5285;

        let (assign6880_e5295,) = {
    if (var_guard36 != 0.0) {
        let assign6880_e5291: f64 = (var_ile).powf(p.p414);
        let assign6880_e5292: f64 = (p.p413 * assign6880_e5291);
        let assign6880_e5293: f64 = (p.p412 + assign6880_e5292);
        (assign6880_e5293,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign6880_e5295;

        let (assign6890_e5313,) = {
    if (var_guard36 != 0.0) {
        let assign6890_e5300: f64 = (p.p415 * p.p416);
        let assign6890_e5302: f64 = (assign6890_e5300 / var_le);
        let assign6890_e5305: f64 = (-var_le);
        let assign6890_e5307: f64 = (assign6890_e5305 / p.p416);
        let assign6890_e5308: f64 = (assign6890_e5307).exp();
        let assign6890_e5309: f64 = (1.0 - assign6890_e5308);
        let assign6890_e5310: f64 = (assign6890_e5302 * assign6890_e5309);
        let assign6890_e5311: f64 = (1.0 + assign6890_e5310);
        (assign6890_e5311,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign6890_e5313;

        let (assign6900_e5322,) = {
    if (var_guard36 != 0.0) {
        let (assign6900_e5320,) = {
            if (var_gpe_edge > 1e-15) {
                (var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign6900_e5320,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign6900_e5322;

        let (assign6910_e5338,) = {
    if (var_guard36 != 0.0) {
        let assign6910_e5326: f64 = (p.p256 * var_we_edge);
        let assign6910_e5329: f64 = (var_gpe_edge * var_le);
        let assign6910_e5330: f64 = (assign6910_e5326 / assign6910_e5329);
        let assign6910_e5334: f64 = (p.p417 * var_iwe);
        let assign6910_e5335: f64 = (1.0 + assign6910_e5334);
        let assign6910_e5336: f64 = (assign6910_e5330 * assign6910_e5335);
        (assign6910_e5336,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign6910_e5338;

        let (assign6920_e5354,) = {
    if (var_guard36 != 0.0) {
        let assign6920_e5343: f64 = (p.p419 * var_ile);
        let assign6920_e5344: f64 = (p.p418 + assign6920_e5343);
        let assign6920_e5347: f64 = (p.p420 * var_iwe);
        let assign6920_e5348: f64 = (assign6920_e5344 + assign6920_e5347);
        let assign6920_e5351: f64 = (p.p421 * var_iae);
        let assign6920_e5352: f64 = (assign6920_e5348 + assign6920_e5351);
        (assign6920_e5352,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign6920_e5354;

        let (assign6930_e5368,) = {
    if (var_guard36 != 0.0) {
        let assign6930_e5359: f64 = (var_ile).powf(p.p423);
        let assign6930_e5360: f64 = (p.p422 * assign6930_e5359);
        let assign6930_e5364: f64 = (p.p424 * var_iwe);
        let assign6930_e5365: f64 = (1.0 + assign6930_e5364);
        let assign6930_e5366: f64 = (assign6930_e5360 * assign6930_e5365);
        (assign6930_e5366,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign6930_e5368;

        let (assign6940_e5372,) = {
    if (var_guard36 != 0.0) {
        (p.p425,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign6940_e5372;

        let (assign6950_e5376,) = {
    if (var_guard36 != 0.0) {
        (p.p426,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign6950_e5376;

        let (assign6960_e5390,) = {
    if (var_guard36 != 0.0) {
        let assign6960_e5381: f64 = (var_ile).powf(p.p428);
        let assign6960_e5382: f64 = (p.p427 * assign6960_e5381);
        let assign6960_e5386: f64 = (p.p429 * var_iwe);
        let assign6960_e5387: f64 = (1.0 + assign6960_e5386);
        let assign6960_e5388: f64 = (assign6960_e5382 * assign6960_e5387);
        (assign6960_e5388,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign6960_e5390;

        let (assign6970_e5394,) = {
    if (var_guard36 != 0.0) {
        (p.p431,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign6970_e5394;

        let (assign6980_e5398,) = {
    if (var_guard36 != 0.0) {
        (p.p430,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign6980_e5398;

        let (assign7040_e5440,) = {
    if (var_guard36 != 0.0) {
        let assign7040_e5429: f64 = (p.p808 * var_ile);
        let assign7040_e5430: f64 = (p.p807 + assign7040_e5429);
        let assign7040_e5433: f64 = (p.p809 * var_iwe);
        let assign7040_e5434: f64 = (assign7040_e5430 + assign7040_e5433);
        let assign7040_e5437: f64 = (p.p810 * var_iae);
        let assign7040_e5438: f64 = (assign7040_e5434 + assign7040_e5437);
        (assign7040_e5438,)
    } else {
        (var_kvthowe,)
    }
};
        var_kvthowe = assign7040_e5440;

        let (assign7050_e5456,) = {
    if (var_guard36 != 0.0) {
        let assign7050_e5445: f64 = (p.p812 * var_ile);
        let assign7050_e5446: f64 = (p.p811 + assign7050_e5445);
        let assign7050_e5449: f64 = (p.p813 * var_iwe);
        let assign7050_e5450: f64 = (assign7050_e5446 + assign7050_e5449);
        let assign7050_e5453: f64 = (p.p814 * var_iae);
        let assign7050_e5454: f64 = (assign7050_e5450 + assign7050_e5453);
        (assign7050_e5454,)
    } else {
        (var_kuowe,)
    }
};
        var_kuowe = assign7050_e5456;

        let (assign7060_e5484,) = {
    if (var_guard36 != 0.0) {
        let assign7060_e5461: f64 = (0.3333333333333333 * var_w_f);
        let assign7060_e5463: f64 = (assign7060_e5461 / var_ngcon_i);
        let assign7060_e5465: f64 = (assign7060_e5463 + var_xgwe);
        let assign7060_e5466: f64 = (p.p440 * assign7060_e5465);
        let assign7060_e5469: f64 = (var_ngcon_i * var_l_slif);
        let assign7060_e5470: f64 = (assign7060_e5466 / assign7060_e5469);
        let assign7060_e5473: f64 = (p.p438 + p.p439);
        let assign7060_e5476: f64 = (var_w_f * var_l_f);
        let assign7060_e5477: f64 = (assign7060_e5473 / assign7060_e5476);
        let assign7060_e5478: f64 = (assign7060_e5470 + assign7060_e5477);
        let assign7060_e5481: f64 = (var_nf_i * p.p437);
        let assign7060_e5482: f64 = (assign7060_e5478 + assign7060_e5481);
        (assign7060_e5482,)
    } else {
        (var_rg_p,)
    }
};
        var_rg_p = assign7060_e5484;

        let (assign7070_e5493,) = {
    if (var_guard36 != 0.0) {
        let (assign7070_e5491,) = {
            if (p.p442 > 0.0) {
                (p.p442,)
            } else {
                (0.0,)
            }
        };
        (assign7070_e5491,)
    } else {
        (var_rsh_i,)
    }
};
        var_rsh_i = assign7070_e5493;

        let (assign7080_e5502,) = {
    if (var_guard36 != 0.0) {
        let (assign7080_e5500,) = {
            if (p.p443 > 0.0) {
                (p.p443,)
            } else {
                (0.0,)
            }
        };
        (assign7080_e5500,)
    } else {
        (var_rshd_i,)
    }
};
        var_rshd_i = assign7080_e5502;

        let assign7090_e5505: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard50 = assign7090_e5505;

        let (assign7100_e5511,) = {
    if ((var_guard36 != 0.0) && (var_guard50 != 0.0)) {
        (var_rsh_i,)
    } else {
        (var_rshd_i,)
    }
};
        var_rshd_i = assign7100_e5511;

        let (assign7110_e5519,) = {
    if (var_guard36 != 0.0) {
        let assign7110_e5515: f64 = (var_nf_i * p.p12);
        let assign7110_e5517: f64 = (assign7110_e5515 * var_rsh_i);
        (assign7110_e5517,)
    } else {
        (var_rse_p,)
    }
};
        var_rse_p = assign7110_e5519;

        let (assign7120_e5527,) = {
    if (var_guard36 != 0.0) {
        let assign7120_e5523: f64 = (var_nf_i * p.p13);
        let assign7120_e5525: f64 = (assign7120_e5523 * var_rshd_i);
        (assign7120_e5525,)
    } else {
        (var_rde_p,)
    }
};
        var_rde_p = assign7120_e5527;

        let (assign7130_e5533,) = {
    if (var_guard36 != 0.0) {
        let assign7130_e5531: f64 = (var_nf_i * p.p445);
        (assign7130_e5531,)
    } else {
        (var_rwell_p,)
    }
};
        var_rwell_p = assign7130_e5533;

        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_axinr_p_slot = var_axinr_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_fnt_p_slot = var_fnt_p;
        *var_fntexc_p_slot = var_fntexc_p;
        *var_gpe_edge_slot = var_gpe_edge;
        *var_guard50_slot = var_guard50;
        *var_kuowe_slot = var_kuowe;
        *var_kvthowe_slot = var_kvthowe;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_rde_p_slot = var_rde_p;
        *var_rg_p_slot = var_rg_p;
        *var_rse_p_slot = var_rse_p;
        *var_rsh_i_slot = var_rsh_i;
        *var_rshd_i_slot = var_rshd_i;
        *var_rwell_p_slot = var_rwell_p;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_temp0_slot = var_temp0;
        *var_tmpx_slot = var_tmpx;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_we_edge_slot = var_we_edge;
    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_nf_i: f64,
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
        var_gfacnud_p_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard56_slot: &mut f64,
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
        var_mue_p_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_rbulk_p_slot: &mut f64,
        var_rjund_p_slot: &mut f64,
        var_rjuns_p_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
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
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
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
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_rbulk_p: f64 = *var_rbulk_p_slot;
        let mut var_rjund_p: f64 = *var_rjund_p_slot;
        let mut var_rjuns_p: f64 = *var_rjuns_p_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;

        let (assign7140_e5539,) = {
    if (var_guard36 != 0.0) {
        let assign7140_e5537: f64 = (var_nf_i * p.p444);
        (assign7140_e5537,)
    } else {
        (var_rbulk_p,)
    }
};
        var_rbulk_p = assign7140_e5539;

        let (assign7150_e5545,) = {
    if (var_guard36 != 0.0) {
        let assign7150_e5543: f64 = (var_nf_i * p.p446);
        (assign7150_e5543,)
    } else {
        (var_rjuns_p,)
    }
};
        var_rjuns_p = assign7150_e5545;

        let (assign7160_e5551,) = {
    if (var_guard36 != 0.0) {
        let assign7160_e5549: f64 = (var_nf_i * p.p447);
        (assign7160_e5549,)
    } else {
        (var_rjund_p,)
    }
};
        var_rjund_p = assign7160_e5551;

        let assign7170_e5570: f64 = if (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]) { 1.0 } else { 0.0 };
        var_guard51 = assign7170_e5570;

        let (assign7180_e5588,) = {
    if ((var_guard36 != 0.0) && (var_guard51 != 0.0)) {
        let assign7180_e5577: f64 = (p.p449 * var_ile);
        let assign7180_e5578: f64 = (p.p448 + assign7180_e5577);
        let assign7180_e5581: f64 = (p.p450 * var_iwe);
        let assign7180_e5582: f64 = (assign7180_e5578 + assign7180_e5581);
        let assign7180_e5585: f64 = (p.p451 * var_iae);
        let assign7180_e5586: f64 = (assign7180_e5582 + assign7180_e5585);
        (assign7180_e5586,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign7180_e5588;

        let assign7190_e5607: f64 = if (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]) { 1.0 } else { 0.0 };
        var_guard52 = assign7190_e5607;

        let (assign7200_e5625,) = {
    if ((var_guard36 != 0.0) && (var_guard52 != 0.0)) {
        let assign7200_e5614: f64 = (p.p453 * var_ile);
        let assign7200_e5615: f64 = (p.p452 + assign7200_e5614);
        let assign7200_e5618: f64 = (p.p454 * var_iwe);
        let assign7200_e5619: f64 = (assign7200_e5615 + assign7200_e5618);
        let assign7200_e5622: f64 = (p.p455 * var_iae);
        let assign7200_e5623: f64 = (assign7200_e5619 + assign7200_e5622);
        (assign7200_e5623,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign7200_e5625;

        let assign7210_e5644: f64 = if (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]) { 1.0 } else { 0.0 };
        var_guard53 = assign7210_e5644;

        let (assign7220_e5662,) = {
    if ((var_guard36 != 0.0) && (var_guard53 != 0.0)) {
        let assign7220_e5651: f64 = (p.p457 * var_ile);
        let assign7220_e5652: f64 = (p.p456 + assign7220_e5651);
        let assign7220_e5655: f64 = (p.p458 * var_iwe);
        let assign7220_e5656: f64 = (assign7220_e5652 + assign7220_e5655);
        let assign7220_e5659: f64 = (p.p459 * var_iae);
        let assign7220_e5660: f64 = (assign7220_e5656 + assign7220_e5659);
        (assign7220_e5660,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign7220_e5662;

        let assign7230_e5681: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        var_guard54 = assign7230_e5681;

        let (assign7240_e5699,) = {
    if ((var_guard36 != 0.0) && (var_guard54 != 0.0)) {
        let assign7240_e5688: f64 = (p.p461 * var_ile);
        let assign7240_e5689: f64 = (p.p460 + assign7240_e5688);
        let assign7240_e5692: f64 = (p.p462 * var_iwe);
        let assign7240_e5693: f64 = (assign7240_e5689 + assign7240_e5692);
        let assign7240_e5696: f64 = (p.p463 * var_iae);
        let assign7240_e5697: f64 = (assign7240_e5693 + assign7240_e5696);
        (assign7240_e5697,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign7240_e5699;

        let assign7250_e5718: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        var_guard55 = assign7250_e5718;

        let (assign7260_e5736,) = {
    if ((var_guard36 != 0.0) && (var_guard55 != 0.0)) {
        let assign7260_e5725: f64 = (p.p465 * var_ile);
        let assign7260_e5726: f64 = (p.p464 + assign7260_e5725);
        let assign7260_e5729: f64 = (p.p466 * var_iwe);
        let assign7260_e5730: f64 = (assign7260_e5726 + assign7260_e5729);
        let assign7260_e5733: f64 = (p.p467 * var_iae);
        let assign7260_e5734: f64 = (assign7260_e5730 + assign7260_e5733);
        (assign7260_e5734,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign7260_e5736;

        let assign7270_e5755: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        var_guard56 = assign7270_e5755;

        let (assign7280_e5773,) = {
    if ((var_guard36 != 0.0) && (var_guard56 != 0.0)) {
        let assign7280_e5762: f64 = (p.p469 * var_ile);
        let assign7280_e5763: f64 = (p.p468 + assign7280_e5762);
        let assign7280_e5766: f64 = (p.p470 * var_iwe);
        let assign7280_e5767: f64 = (assign7280_e5763 + assign7280_e5766);
        let assign7280_e5770: f64 = (p.p471 * var_iae);
        let assign7280_e5771: f64 = (assign7280_e5767 + assign7280_e5770);
        (assign7280_e5771,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign7280_e5773;

        let assign7290_e5792: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        var_guard57 = assign7290_e5792;

        let (assign7300_e5810,) = {
    if ((var_guard36 != 0.0) && (var_guard57 != 0.0)) {
        let assign7300_e5799: f64 = (p.p473 * var_ile);
        let assign7300_e5800: f64 = (p.p472 + assign7300_e5799);
        let assign7300_e5803: f64 = (p.p474 * var_iwe);
        let assign7300_e5804: f64 = (assign7300_e5800 + assign7300_e5803);
        let assign7300_e5807: f64 = (p.p475 * var_iae);
        let assign7300_e5808: f64 = (assign7300_e5804 + assign7300_e5807);
        (assign7300_e5808,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign7300_e5810;

        let assign7310_e5829: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        var_guard58 = assign7310_e5829;

        let (assign7320_e5847,) = {
    if ((var_guard36 != 0.0) && (var_guard58 != 0.0)) {
        let assign7320_e5836: f64 = (p.p477 * var_ile);
        let assign7320_e5837: f64 = (p.p476 + assign7320_e5836);
        let assign7320_e5840: f64 = (p.p478 * var_iwe);
        let assign7320_e5841: f64 = (assign7320_e5837 + assign7320_e5840);
        let assign7320_e5844: f64 = (p.p479 * var_iae);
        let assign7320_e5845: f64 = (assign7320_e5841 + assign7320_e5844);
        (assign7320_e5845,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign7320_e5847;

        let assign7330_e5866: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        var_guard59 = assign7330_e5866;

        let (assign7340_e5884,) = {
    if ((var_guard36 != 0.0) && (var_guard59 != 0.0)) {
        let assign7340_e5873: f64 = (p.p481 * var_ile);
        let assign7340_e5874: f64 = (p.p480 + assign7340_e5873);
        let assign7340_e5877: f64 = (p.p482 * var_iwe);
        let assign7340_e5878: f64 = (assign7340_e5874 + assign7340_e5877);
        let assign7340_e5881: f64 = (p.p483 * var_iae);
        let assign7340_e5882: f64 = (assign7340_e5878 + assign7340_e5881);
        (assign7340_e5882,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign7340_e5884;

        let assign7350_e5903: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        var_guard60 = assign7350_e5903;

        let (assign7360_e5921,) = {
    if ((var_guard36 != 0.0) && (var_guard60 != 0.0)) {
        let assign7360_e5910: f64 = (p.p485 * var_ile);
        let assign7360_e5911: f64 = (p.p484 + assign7360_e5910);
        let assign7360_e5914: f64 = (p.p486 * var_iwe);
        let assign7360_e5915: f64 = (assign7360_e5911 + assign7360_e5914);
        let assign7360_e5918: f64 = (p.p487 * var_iae);
        let assign7360_e5919: f64 = (assign7360_e5915 + assign7360_e5918);
        (assign7360_e5919,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign7360_e5921;

        let assign7370_e5940: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        var_guard61 = assign7370_e5940;

        let (assign7380_e5958,) = {
    if ((var_guard36 != 0.0) && (var_guard61 != 0.0)) {
        let assign7380_e5947: f64 = (p.p493 * var_ile);
        let assign7380_e5948: f64 = (p.p492 + assign7380_e5947);
        let assign7380_e5951: f64 = (p.p494 * var_iwe);
        let assign7380_e5952: f64 = (assign7380_e5948 + assign7380_e5951);
        let assign7380_e5955: f64 = (p.p495 * var_iae);
        let assign7380_e5956: f64 = (assign7380_e5952 + assign7380_e5955);
        (assign7380_e5956,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign7380_e5958;

        let assign7390_e5977: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        var_guard62 = assign7390_e5977;

        let (assign7400_e5995,) = {
    if ((var_guard36 != 0.0) && (var_guard62 != 0.0)) {
        let assign7400_e5984: f64 = (p.p489 * var_ile);
        let assign7400_e5985: f64 = (p.p488 + assign7400_e5984);
        let assign7400_e5988: f64 = (p.p490 * var_iwe);
        let assign7400_e5989: f64 = (assign7400_e5985 + assign7400_e5988);
        let assign7400_e5992: f64 = (p.p491 * var_iae);
        let assign7400_e5993: f64 = (assign7400_e5989 + assign7400_e5992);
        (assign7400_e5993,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign7400_e5995;

        let assign7410_e6014: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        var_guard63 = assign7410_e6014;

        let (assign7420_e6032,) = {
    if ((var_guard36 != 0.0) && (var_guard63 != 0.0)) {
        let assign7420_e6021: f64 = (p.p497 * var_ile);
        let assign7420_e6022: f64 = (p.p496 + assign7420_e6021);
        let assign7420_e6025: f64 = (p.p498 * var_iwe);
        let assign7420_e6026: f64 = (assign7420_e6022 + assign7420_e6025);
        let assign7420_e6029: f64 = (p.p499 * var_iae);
        let assign7420_e6030: f64 = (assign7420_e6026 + assign7420_e6029);
        (assign7420_e6030,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign7420_e6032;

        let assign7430_e6051: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        var_guard64 = assign7430_e6051;

        let (assign7440_e6071,) = {
    if ((var_guard36 != 0.0) && (var_guard64 != 0.0)) {
        let assign7440_e6059: f64 = (p.p501 * var_ile);
        let assign7440_e6060: f64 = (p.p500 + assign7440_e6059);
        let assign7440_e6063: f64 = (p.p502 * var_iwe);
        let assign7440_e6064: f64 = (assign7440_e6060 + assign7440_e6063);
        let assign7440_e6067: f64 = (p.p503 * var_iae);
        let assign7440_e6068: f64 = (assign7440_e6064 + assign7440_e6067);
        let assign7440_e6069: f64 = (var_ile2 * assign7440_e6068);
        (assign7440_e6069,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign7440_e6071;

        let assign7450_e6090: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        var_guard65 = assign7450_e6090;

        let (assign7460_e6108,) = {
    if ((var_guard36 != 0.0) && (var_guard65 != 0.0)) {
        let assign7460_e6097: f64 = (p.p509 * var_ile);
        let assign7460_e6098: f64 = (p.p508 + assign7460_e6097);
        let assign7460_e6101: f64 = (p.p510 * var_iwe);
        let assign7460_e6102: f64 = (assign7460_e6098 + assign7460_e6101);
        let assign7460_e6105: f64 = (p.p511 * var_iae);
        let assign7460_e6106: f64 = (assign7460_e6102 + assign7460_e6105);
        (assign7460_e6106,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign7460_e6108;

        let assign7470_e6127: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        var_guard66 = assign7470_e6127;

        let (assign7480_e6145,) = {
    if ((var_guard36 != 0.0) && (var_guard66 != 0.0)) {
        let assign7480_e6134: f64 = (p.p505 * var_ile);
        let assign7480_e6135: f64 = (p.p504 + assign7480_e6134);
        let assign7480_e6138: f64 = (p.p506 * var_iwe);
        let assign7480_e6139: f64 = (assign7480_e6135 + assign7480_e6138);
        let assign7480_e6142: f64 = (p.p507 * var_iae);
        let assign7480_e6143: f64 = (assign7480_e6139 + assign7480_e6142);
        (assign7480_e6143,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign7480_e6145;

        let assign7490_e6164: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        var_guard67 = assign7490_e6164;

        let (assign7500_e6184,) = {
    if ((var_guard36 != 0.0) && (var_guard67 != 0.0)) {
        let assign7500_e6172: f64 = (p.p513 * var_ile);
        let assign7500_e6173: f64 = (p.p512 + assign7500_e6172);
        let assign7500_e6176: f64 = (p.p514 * var_iwe);
        let assign7500_e6177: f64 = (assign7500_e6173 + assign7500_e6176);
        let assign7500_e6180: f64 = (p.p515 * var_iae);
        let assign7500_e6181: f64 = (assign7500_e6177 + assign7500_e6180);
        let assign7500_e6182: f64 = (var_ile2 * assign7500_e6181);
        (assign7500_e6182,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign7500_e6184;

        let assign7510_e6203: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        var_guard68 = assign7510_e6203;

        let (assign7520_e6221,) = {
    if ((var_guard36 != 0.0) && (var_guard68 != 0.0)) {
        let assign7520_e6210: f64 = (p.p521 * var_ile);
        let assign7520_e6211: f64 = (p.p520 + assign7520_e6210);
        let assign7520_e6214: f64 = (p.p522 * var_iwe);
        let assign7520_e6215: f64 = (assign7520_e6211 + assign7520_e6214);
        let assign7520_e6218: f64 = (p.p523 * var_iae);
        let assign7520_e6219: f64 = (assign7520_e6215 + assign7520_e6218);
        (assign7520_e6219,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign7520_e6221;

        let assign7530_e6240: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        var_guard69 = assign7530_e6240;

        let (assign7540_e6258,) = {
    if ((var_guard36 != 0.0) && (var_guard69 != 0.0)) {
        let assign7540_e6247: f64 = (p.p517 * var_ile);
        let assign7540_e6248: f64 = (p.p516 + assign7540_e6247);
        let assign7540_e6251: f64 = (p.p518 * var_iwe);
        let assign7540_e6252: f64 = (assign7540_e6248 + assign7540_e6251);
        let assign7540_e6255: f64 = (p.p519 * var_iae);
        let assign7540_e6256: f64 = (assign7540_e6252 + assign7540_e6255);
        (assign7540_e6256,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign7540_e6258;

        let assign7550_e6277: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        var_guard70 = assign7550_e6277;

        let (assign7560_e6299,) = {
    if ((var_guard36 != 0.0) && (var_guard70 != 0.0)) {
        let assign7560_e6283: f64 = (var_we / var_le);
        let assign7560_e6287: f64 = (p.p525 * var_ile);
        let assign7560_e6288: f64 = (p.p524 + assign7560_e6287);
        let assign7560_e6291: f64 = (p.p526 * var_iwe);
        let assign7560_e6292: f64 = (assign7560_e6288 + assign7560_e6291);
        let assign7560_e6295: f64 = (p.p527 * var_iae);
        let assign7560_e6296: f64 = (assign7560_e6292 + assign7560_e6295);
        let assign7560_e6297: f64 = (assign7560_e6283 * assign7560_e6296);
        (assign7560_e6297,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign7560_e6299;

        let assign7570_e6318: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        var_guard71 = assign7570_e6318;

        let (assign7580_e6336,) = {
    if ((var_guard36 != 0.0) && (var_guard71 != 0.0)) {
        let assign7580_e6325: f64 = (p.p529 * var_ile);
        let assign7580_e6326: f64 = (p.p528 + assign7580_e6325);
        let assign7580_e6329: f64 = (p.p530 * var_iwe);
        let assign7580_e6330: f64 = (assign7580_e6326 + assign7580_e6329);
        let assign7580_e6333: f64 = (p.p531 * var_iae);
        let assign7580_e6334: f64 = (assign7580_e6330 + assign7580_e6333);
        (assign7580_e6334,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign7580_e6336;

        let assign7590_e6355: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        var_guard72 = assign7590_e6355;

        let (assign7600_e6373,) = {
    if ((var_guard36 != 0.0) && (var_guard72 != 0.0)) {
        let assign7600_e6362: f64 = (p.p533 * var_ile);
        let assign7600_e6363: f64 = (p.p532 + assign7600_e6362);
        let assign7600_e6366: f64 = (p.p534 * var_iwe);
        let assign7600_e6367: f64 = (assign7600_e6363 + assign7600_e6366);
        let assign7600_e6370: f64 = (p.p535 * var_iae);
        let assign7600_e6371: f64 = (assign7600_e6367 + assign7600_e6370);
        (assign7600_e6371,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign7600_e6373;

        let assign7610_e6392: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        var_guard73 = assign7610_e6392;

        let (assign7620_e6410,) = {
    if ((var_guard36 != 0.0) && (var_guard73 != 0.0)) {
        let assign7620_e6399: f64 = (p.p537 * var_ile);
        let assign7620_e6400: f64 = (p.p536 + assign7620_e6399);
        let assign7620_e6403: f64 = (p.p538 * var_iwe);
        let assign7620_e6404: f64 = (assign7620_e6400 + assign7620_e6403);
        let assign7620_e6407: f64 = (p.p539 * var_iae);
        let assign7620_e6408: f64 = (assign7620_e6404 + assign7620_e6407);
        (assign7620_e6408,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign7620_e6410;

        let assign7630_e6429: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        var_guard74 = assign7630_e6429;

        let (assign7640_e6447,) = {
    if ((var_guard36 != 0.0) && (var_guard74 != 0.0)) {
        let assign7640_e6436: f64 = (p.p541 * var_ile);
        let assign7640_e6437: f64 = (p.p540 + assign7640_e6436);
        let assign7640_e6440: f64 = (p.p542 * var_iwe);
        let assign7640_e6441: f64 = (assign7640_e6437 + assign7640_e6440);
        let assign7640_e6444: f64 = (p.p543 * var_iae);
        let assign7640_e6445: f64 = (assign7640_e6441 + assign7640_e6444);
        (assign7640_e6445,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign7640_e6447;

        let assign7650_e6466: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        var_guard75 = assign7650_e6466;

        let (assign7660_e6484,) = {
    if ((var_guard36 != 0.0) && (var_guard75 != 0.0)) {
        let assign7660_e6473: f64 = (p.p545 * var_ile);
        let assign7660_e6474: f64 = (p.p544 + assign7660_e6473);
        let assign7660_e6477: f64 = (p.p546 * var_iwe);
        let assign7660_e6478: f64 = (assign7660_e6474 + assign7660_e6477);
        let assign7660_e6481: f64 = (p.p547 * var_iae);
        let assign7660_e6482: f64 = (assign7660_e6478 + assign7660_e6481);
        (assign7660_e6482,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign7660_e6484;

        let assign7670_e6503: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        var_guard76 = assign7670_e6503;

        let (assign7680_e6521,) = {
    if ((var_guard36 != 0.0) && (var_guard76 != 0.0)) {
        let assign7680_e6510: f64 = (p.p549 * var_ile);
        let assign7680_e6511: f64 = (p.p548 + assign7680_e6510);
        let assign7680_e6514: f64 = (p.p550 * var_iwe);
        let assign7680_e6515: f64 = (assign7680_e6511 + assign7680_e6514);
        let assign7680_e6518: f64 = (p.p551 * var_iae);
        let assign7680_e6519: f64 = (assign7680_e6515 + assign7680_e6518);
        (assign7680_e6519,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign7680_e6521;

        let assign7690_e6540: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        var_guard77 = assign7690_e6540;

        *var_betn_p_slot = var_betn_p;
        *var_cf_p_slot = var_cf_p;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfd_p_slot = var_cfd_p;
        *var_cs_p_slot = var_cs_p;
        *var_ct_p_slot = var_ct_p;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctg_p_slot = var_ctg_p;
        *var_dphib_p_slot = var_dphib_p;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_guard51_slot = var_guard51;
        *var_guard52_slot = var_guard52;
        *var_guard53_slot = var_guard53;
        *var_guard54_slot = var_guard54;
        *var_guard55_slot = var_guard55;
        *var_guard56_slot = var_guard56;
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
        *var_mue_p_slot = var_mue_p;
        *var_neff_p_slot = var_neff_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
        *var_psce_p_slot = var_psce_p;
        *var_psceb_p_slot = var_psceb_p;
        *var_psced_p_slot = var_psced_p;
        *var_rbulk_p_slot = var_rbulk_p;
        *var_rjund_p_slot = var_rjund_p;
        *var_rjuns_p_slot = var_rjuns_p;
        *var_stbet_p_slot = var_stbet_p;
        *var_stct_p_slot = var_stct_p;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_thecs_p_slot = var_thecs_p;
        *var_themu_p_slot = var_themu_p;
        *var_vfb_p_slot = var_vfb_p;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_xcor_p_slot = var_xcor_p;
    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_guard77: f64,
        var_iae: f64,
        var_iiae: f64,
        var_iiwe: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_a1_p_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard84_slot: &mut f64,
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
        var_poparam_i_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
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
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
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
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;

        let (assign7700_e6560,) = {
    if ((var_guard36 != 0.0) && (var_guard77 != 0.0)) {
        let assign7700_e6548: f64 = (p.p553 * var_ile);
        let assign7700_e6549: f64 = (p.p552 + assign7700_e6548);
        let assign7700_e6552: f64 = (p.p554 * var_iwe);
        let assign7700_e6553: f64 = (assign7700_e6549 + assign7700_e6552);
        let assign7700_e6556: f64 = (p.p555 * var_iae);
        let assign7700_e6557: f64 = (assign7700_e6553 + assign7700_e6556);
        let assign7700_e6558: f64 = (var_iwe * assign7700_e6557);
        (assign7700_e6558,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign7700_e6560;

        let assign7710_e6579: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        var_guard78 = assign7710_e6579;

        let (assign7720_e6597,) = {
    if ((var_guard36 != 0.0) && (var_guard78 != 0.0)) {
        let assign7720_e6586: f64 = (p.p557 * var_ile);
        let assign7720_e6587: f64 = (p.p556 + assign7720_e6586);
        let assign7720_e6590: f64 = (p.p558 * var_iwe);
        let assign7720_e6591: f64 = (assign7720_e6587 + assign7720_e6590);
        let assign7720_e6594: f64 = (p.p559 * var_iae);
        let assign7720_e6595: f64 = (assign7720_e6591 + assign7720_e6594);
        (assign7720_e6595,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign7720_e6597;

        let assign7730_e6616: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        var_guard79 = assign7730_e6616;

        let (assign7740_e6634,) = {
    if ((var_guard36 != 0.0) && (var_guard79 != 0.0)) {
        let assign7740_e6623: f64 = (p.p561 * var_ile);
        let assign7740_e6624: f64 = (p.p560 + assign7740_e6623);
        let assign7740_e6627: f64 = (p.p562 * var_iwe);
        let assign7740_e6628: f64 = (assign7740_e6624 + assign7740_e6627);
        let assign7740_e6631: f64 = (p.p563 * var_iae);
        let assign7740_e6632: f64 = (assign7740_e6628 + assign7740_e6631);
        (assign7740_e6632,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign7740_e6634;

        let assign7750_e6653: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        var_guard80 = assign7750_e6653;

        let (assign7760_e6671,) = {
    if ((var_guard36 != 0.0) && (var_guard80 != 0.0)) {
        let assign7760_e6660: f64 = (p.p565 * var_ile);
        let assign7760_e6661: f64 = (p.p564 + assign7760_e6660);
        let assign7760_e6664: f64 = (p.p566 * var_iwe);
        let assign7760_e6665: f64 = (assign7760_e6661 + assign7760_e6664);
        let assign7760_e6668: f64 = (p.p567 * var_iae);
        let assign7760_e6669: f64 = (assign7760_e6665 + assign7760_e6668);
        (assign7760_e6669,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign7760_e6671;

        let assign7770_e6690: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        var_guard81 = assign7770_e6690;

        let (assign7780_e6710,) = {
    if ((var_guard36 != 0.0) && (var_guard81 != 0.0)) {
        let assign7780_e6698: f64 = (p.p569 * var_ile);
        let assign7780_e6699: f64 = (p.p568 + assign7780_e6698);
        let assign7780_e6702: f64 = (p.p570 * var_iwe);
        let assign7780_e6703: f64 = (assign7780_e6699 + assign7780_e6702);
        let assign7780_e6706: f64 = (p.p571 * var_iae);
        let assign7780_e6707: f64 = (assign7780_e6703 + assign7780_e6706);
        let assign7780_e6708: f64 = (var_ile * assign7780_e6707);
        (assign7780_e6708,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign7780_e6710;

        let assign7790_e6729: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        var_guard82 = assign7790_e6729;

        let (assign7800_e6747,) = {
    if ((var_guard36 != 0.0) && (var_guard82 != 0.0)) {
        let assign7800_e6736: f64 = (p.p573 * var_ile);
        let assign7800_e6737: f64 = (p.p572 + assign7800_e6736);
        let assign7800_e6740: f64 = (p.p574 * var_iwe);
        let assign7800_e6741: f64 = (assign7800_e6737 + assign7800_e6740);
        let assign7800_e6744: f64 = (p.p575 * var_iae);
        let assign7800_e6745: f64 = (assign7800_e6741 + assign7800_e6744);
        (assign7800_e6745,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign7800_e6747;

        let assign7810_e6766: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        var_guard83 = assign7810_e6766;

        let (assign7820_e6784,) = {
    if ((var_guard36 != 0.0) && (var_guard83 != 0.0)) {
        let assign7820_e6773: f64 = (p.p577 * var_ile);
        let assign7820_e6774: f64 = (p.p576 + assign7820_e6773);
        let assign7820_e6777: f64 = (p.p578 * var_iwe);
        let assign7820_e6778: f64 = (assign7820_e6774 + assign7820_e6777);
        let assign7820_e6781: f64 = (p.p579 * var_iae);
        let assign7820_e6782: f64 = (assign7820_e6778 + assign7820_e6781);
        (assign7820_e6782,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign7820_e6784;

        let assign7830_e6803: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        var_guard84 = assign7830_e6803;

        let (assign7840_e6821,) = {
    if ((var_guard36 != 0.0) && (var_guard84 != 0.0)) {
        let assign7840_e6810: f64 = (p.p581 * var_ile);
        let assign7840_e6811: f64 = (p.p580 + assign7840_e6810);
        let assign7840_e6814: f64 = (p.p582 * var_iwe);
        let assign7840_e6815: f64 = (assign7840_e6811 + assign7840_e6814);
        let assign7840_e6818: f64 = (p.p583 * var_iae);
        let assign7840_e6819: f64 = (assign7840_e6815 + assign7840_e6818);
        (assign7840_e6819,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign7840_e6821;

        let assign7850_e6840: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        var_guard85 = assign7850_e6840;

        let (assign7860_e6858,) = {
    if ((var_guard36 != 0.0) && (var_guard85 != 0.0)) {
        let assign7860_e6847: f64 = (p.p585 * var_ile);
        let assign7860_e6848: f64 = (p.p584 + assign7860_e6847);
        let assign7860_e6851: f64 = (p.p586 * var_iwe);
        let assign7860_e6852: f64 = (assign7860_e6848 + assign7860_e6851);
        let assign7860_e6855: f64 = (p.p587 * var_iae);
        let assign7860_e6856: f64 = (assign7860_e6852 + assign7860_e6855);
        (assign7860_e6856,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign7860_e6858;

        let assign7870_e6877: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        var_guard86 = assign7870_e6877;

        let (assign7880_e6897,) = {
    if ((var_guard36 != 0.0) && (var_guard86 != 0.0)) {
        let assign7880_e6885: f64 = (p.p589 * var_ile);
        let assign7880_e6886: f64 = (p.p588 + assign7880_e6885);
        let assign7880_e6889: f64 = (p.p590 * var_iwe);
        let assign7880_e6890: f64 = (assign7880_e6886 + assign7880_e6889);
        let assign7880_e6893: f64 = (p.p591 * var_iae);
        let assign7880_e6894: f64 = (assign7880_e6890 + assign7880_e6893);
        let assign7880_e6895: f64 = (var_ile * assign7880_e6894);
        (assign7880_e6895,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign7880_e6897;

        let assign7890_e6916: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        var_guard87 = assign7890_e6916;

        let (assign7900_e6934,) = {
    if ((var_guard36 != 0.0) && (var_guard87 != 0.0)) {
        let assign7900_e6923: f64 = (p.p593 * var_ile);
        let assign7900_e6924: f64 = (p.p592 + assign7900_e6923);
        let assign7900_e6927: f64 = (p.p594 * var_iwe);
        let assign7900_e6928: f64 = (assign7900_e6924 + assign7900_e6927);
        let assign7900_e6931: f64 = (p.p595 * var_iae);
        let assign7900_e6932: f64 = (assign7900_e6928 + assign7900_e6931);
        (assign7900_e6932,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign7900_e6934;

        let assign7910_e6953: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        var_guard88 = assign7910_e6953;

        let (assign7920_e6971,) = {
    if ((var_guard36 != 0.0) && (var_guard88 != 0.0)) {
        let assign7920_e6960: f64 = (p.p597 * var_ile);
        let assign7920_e6961: f64 = (p.p596 + assign7920_e6960);
        let assign7920_e6964: f64 = (p.p598 * var_iwe);
        let assign7920_e6965: f64 = (assign7920_e6961 + assign7920_e6964);
        let assign7920_e6968: f64 = (p.p599 * var_iae);
        let assign7920_e6969: f64 = (assign7920_e6965 + assign7920_e6968);
        (assign7920_e6969,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign7920_e6971;

        let assign7930_e6990: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        var_guard89 = assign7930_e6990;

        let (assign7940_e7008,) = {
    if ((var_guard36 != 0.0) && (var_guard89 != 0.0)) {
        let assign7940_e6997: f64 = (p.p601 * var_ile);
        let assign7940_e6998: f64 = (p.p600 + assign7940_e6997);
        let assign7940_e7001: f64 = (p.p602 * var_iwe);
        let assign7940_e7002: f64 = (assign7940_e6998 + assign7940_e7001);
        let assign7940_e7005: f64 = (p.p603 * var_iae);
        let assign7940_e7006: f64 = (assign7940_e7002 + assign7940_e7005);
        (assign7940_e7006,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign7940_e7008;

        let assign7950_e7027: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        var_guard90 = assign7950_e7027;

        let (assign7960_e7045,) = {
    if ((var_guard36 != 0.0) && (var_guard90 != 0.0)) {
        let assign7960_e7034: f64 = (p.p605 * var_ile);
        let assign7960_e7035: f64 = (p.p604 + assign7960_e7034);
        let assign7960_e7038: f64 = (p.p606 * var_iwe);
        let assign7960_e7039: f64 = (assign7960_e7035 + assign7960_e7038);
        let assign7960_e7042: f64 = (p.p607 * var_iae);
        let assign7960_e7043: f64 = (assign7960_e7039 + assign7960_e7042);
        (assign7960_e7043,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign7960_e7045;

        let assign7970_e7064: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        var_guard91 = assign7970_e7064;

        let (assign7980_e7082,) = {
    if ((var_guard36 != 0.0) && (var_guard91 != 0.0)) {
        let assign7980_e7071: f64 = (p.p609 * var_ile);
        let assign7980_e7072: f64 = (p.p608 + assign7980_e7071);
        let assign7980_e7075: f64 = (p.p610 * var_iwe);
        let assign7980_e7076: f64 = (assign7980_e7072 + assign7980_e7075);
        let assign7980_e7079: f64 = (p.p611 * var_iae);
        let assign7980_e7080: f64 = (assign7980_e7076 + assign7980_e7079);
        (assign7980_e7080,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign7980_e7082;

        let assign7990_e7101: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        var_guard92 = assign7990_e7101;

        let (assign8000_e7119,) = {
    if ((var_guard36 != 0.0) && (var_guard92 != 0.0)) {
        let assign8000_e7108: f64 = (p.p613 * var_ile);
        let assign8000_e7109: f64 = (p.p612 + assign8000_e7108);
        let assign8000_e7112: f64 = (p.p614 * var_iwe);
        let assign8000_e7113: f64 = (assign8000_e7109 + assign8000_e7112);
        let assign8000_e7116: f64 = (p.p615 * var_iae);
        let assign8000_e7117: f64 = (assign8000_e7113 + assign8000_e7116);
        (assign8000_e7117,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign8000_e7119;

        let assign8010_e7138: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        var_guard93 = assign8010_e7138;

        let (assign8020_e7158,) = {
    if ((var_guard36 != 0.0) && (var_guard93 != 0.0)) {
        let assign8020_e7146: f64 = (p.p617 * var_ile);
        let assign8020_e7147: f64 = (p.p616 + assign8020_e7146);
        let assign8020_e7150: f64 = (p.p618 * var_iwe);
        let assign8020_e7151: f64 = (assign8020_e7147 + assign8020_e7150);
        let assign8020_e7154: f64 = (p.p619 * var_iae);
        let assign8020_e7155: f64 = (assign8020_e7151 + assign8020_e7154);
        let assign8020_e7156: f64 = (var_iiae * assign8020_e7155);
        (assign8020_e7156,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign8020_e7158;

        let assign8030_e7177: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        var_guard94 = assign8030_e7177;

        let (assign8040_e7197,) = {
    if ((var_guard36 != 0.0) && (var_guard94 != 0.0)) {
        let assign8040_e7185: f64 = (p.p621 * var_ile);
        let assign8040_e7186: f64 = (p.p620 + assign8040_e7185);
        let assign8040_e7189: f64 = (p.p622 * var_iwe);
        let assign8040_e7190: f64 = (assign8040_e7186 + assign8040_e7189);
        let assign8040_e7193: f64 = (p.p623 * var_iae);
        let assign8040_e7194: f64 = (assign8040_e7190 + assign8040_e7193);
        let assign8040_e7195: f64 = (var_iiwe * assign8040_e7194);
        (assign8040_e7195,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign8040_e7197;

        let assign8050_e7216: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        var_guard95 = assign8050_e7216;

        let (assign8060_e7236,) = {
    if ((var_guard36 != 0.0) && (var_guard95 != 0.0)) {
        let assign8060_e7224: f64 = (p.p625 * var_ile);
        let assign8060_e7225: f64 = (p.p624 + assign8060_e7224);
        let assign8060_e7228: f64 = (p.p626 * var_iwe);
        let assign8060_e7229: f64 = (assign8060_e7225 + assign8060_e7228);
        let assign8060_e7232: f64 = (p.p627 * var_iae);
        let assign8060_e7233: f64 = (assign8060_e7229 + assign8060_e7232);
        let assign8060_e7234: f64 = (var_iiwe * assign8060_e7233);
        (assign8060_e7234,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign8060_e7236;

        let assign8070_e7255: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        var_guard96 = assign8070_e7255;

        let (assign8080_e7273,) = {
    if ((var_guard36 != 0.0) && (var_guard96 != 0.0)) {
        let assign8080_e7262: f64 = (p.p629 * var_ile);
        let assign8080_e7263: f64 = (p.p628 + assign8080_e7262);
        let assign8080_e7266: f64 = (p.p630 * var_iwe);
        let assign8080_e7267: f64 = (assign8080_e7263 + assign8080_e7266);
        let assign8080_e7270: f64 = (p.p631 * var_iae);
        let assign8080_e7271: f64 = (assign8080_e7267 + assign8080_e7270);
        (assign8080_e7271,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign8080_e7273;

        let assign8090_e7292: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        var_guard97 = assign8090_e7292;

        let (assign8100_e7312,) = {
    if ((var_guard36 != 0.0) && (var_guard97 != 0.0)) {
        let assign8100_e7300: f64 = (p.p633 * var_ile);
        let assign8100_e7301: f64 = (p.p632 + assign8100_e7300);
        let assign8100_e7304: f64 = (p.p634 * var_iwe);
        let assign8100_e7305: f64 = (assign8100_e7301 + assign8100_e7304);
        let assign8100_e7308: f64 = (p.p635 * var_iae);
        let assign8100_e7309: f64 = (assign8100_e7305 + assign8100_e7308);
        let assign8100_e7310: f64 = (var_iiwe * assign8100_e7309);
        (assign8100_e7310,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign8100_e7312;

        let assign8110_e7331: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        var_guard98 = assign8110_e7331;

        let (assign8120_e7351,) = {
    if ((var_guard36 != 0.0) && (var_guard98 != 0.0)) {
        let assign8120_e7339: f64 = (p.p637 * var_ile);
        let assign8120_e7340: f64 = (p.p636 + assign8120_e7339);
        let assign8120_e7343: f64 = (p.p638 * var_iwe);
        let assign8120_e7344: f64 = (assign8120_e7340 + assign8120_e7343);
        let assign8120_e7347: f64 = (p.p639 * var_iae);
        let assign8120_e7348: f64 = (assign8120_e7344 + assign8120_e7347);
        let assign8120_e7349: f64 = (var_iiwe * assign8120_e7348);
        (assign8120_e7349,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign8120_e7351;

        let assign8130_e7370: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        var_guard99 = assign8130_e7370;

        let (assign8140_e7388,) = {
    if ((var_guard36 != 0.0) && (var_guard99 != 0.0)) {
        let assign8140_e7377: f64 = (p.p641 * var_ile);
        let assign8140_e7378: f64 = (p.p640 + assign8140_e7377);
        let assign8140_e7381: f64 = (p.p642 * var_iwe);
        let assign8140_e7382: f64 = (assign8140_e7378 + assign8140_e7381);
        let assign8140_e7385: f64 = (p.p643 * var_iae);
        let assign8140_e7386: f64 = (assign8140_e7382 + assign8140_e7385);
        (assign8140_e7386,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign8140_e7388;

        let assign8150_e7407: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        var_guard100 = assign8150_e7407;

        let (assign8160_e7425,) = {
    if ((var_guard36 != 0.0) && (var_guard100 != 0.0)) {
        let assign8160_e7414: f64 = (p.p645 * var_ile);
        let assign8160_e7415: f64 = (p.p644 + assign8160_e7414);
        let assign8160_e7418: f64 = (p.p646 * var_iwe);
        let assign8160_e7419: f64 = (assign8160_e7415 + assign8160_e7418);
        let assign8160_e7422: f64 = (p.p647 * var_iae);
        let assign8160_e7423: f64 = (assign8160_e7419 + assign8160_e7422);
        (assign8160_e7423,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign8160_e7425;

        let assign8170_e7444: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        var_guard101 = assign8170_e7444;

        let (assign8180_e7468,) = {
    if ((var_guard36 != 0.0) && (var_guard101 != 0.0)) {
        let assign8180_e7450: f64 = (var_iiwecv * var_lecv);
        let assign8180_e7452: f64 = (assign8180_e7450 / 1e-6);
        let assign8180_e7456: f64 = (p.p649 * var_ile);
        let assign8180_e7457: f64 = (p.p648 + assign8180_e7456);
        let assign8180_e7460: f64 = (p.p650 * var_iwe);
        let assign8180_e7461: f64 = (assign8180_e7457 + assign8180_e7460);
        let assign8180_e7464: f64 = (p.p651 * var_iae);
        let assign8180_e7465: f64 = (assign8180_e7461 + assign8180_e7464);
        let assign8180_e7466: f64 = (assign8180_e7452 * assign8180_e7465);
        (assign8180_e7466,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign8180_e7468;

        let assign8190_e7487: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        var_guard102 = assign8190_e7487;

        let (assign8200_e7505,) = {
    if ((var_guard36 != 0.0) && (var_guard102 != 0.0)) {
        let assign8200_e7494: f64 = (p.p653 * var_ile);
        let assign8200_e7495: f64 = (p.p652 + assign8200_e7494);
        let assign8200_e7498: f64 = (p.p654 * var_iwe);
        let assign8200_e7499: f64 = (assign8200_e7495 + assign8200_e7498);
        let assign8200_e7502: f64 = (p.p655 * var_iae);
        let assign8200_e7503: f64 = (assign8200_e7499 + assign8200_e7502);
        (assign8200_e7503,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign8200_e7505;

        let assign8210_e7524: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        var_guard103 = assign8210_e7524;

        let (assign8220_e7542,) = {
    if ((var_guard36 != 0.0) && (var_guard103 != 0.0)) {
        let assign8220_e7531: f64 = (p.p657 * var_ile);
        let assign8220_e7532: f64 = (p.p656 + assign8220_e7531);
        let assign8220_e7535: f64 = (p.p658 * var_iwe);
        let assign8220_e7536: f64 = (assign8220_e7532 + assign8220_e7535);
        let assign8220_e7539: f64 = (p.p659 * var_iae);
        let assign8220_e7540: f64 = (assign8220_e7536 + assign8220_e7539);
        (assign8220_e7540,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign8220_e7542;

        let assign8230_e7581: f64 = if (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        var_guard104 = assign8230_e7581;

        let (assign8240_e7587,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p568,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8240_e7587;

        let assign8250_e7589: f64 = if param_given[660] { 1.0 } else { 0.0 };
        let assign8250_e7591: f64 = if assign8250_e7589 == 1.0 { 1.0 } else { 0.0 };
        var_guard105 = assign8250_e7591;

        *var_a1_p_slot = var_a1_p;
        *var_a3_p_slot = var_a3_p;
        *var_a4_p_slot = var_a4_p;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp_p_slot = var_alp_p;
        *var_ax_p_slot = var_ax_p;
        *var_cox_p_slot = var_cox_p;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_guard100_slot = var_guard100;
        *var_guard101_slot = var_guard101;
        *var_guard102_slot = var_guard102;
        *var_guard103_slot = var_guard103;
        *var_guard104_slot = var_guard104;
        *var_guard105_slot = var_guard105;
        *var_guard78_slot = var_guard78;
        *var_guard79_slot = var_guard79;
        *var_guard80_slot = var_guard80;
        *var_guard81_slot = var_guard81;
        *var_guard82_slot = var_guard82;
        *var_guard83_slot = var_guard83;
        *var_guard84_slot = var_guard84;
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
        *var_poparam_i_slot = var_poparam_i;
        *var_rs_p_slot = var_rs_p;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsg_p_slot = var_rsg_p;
        *var_sta2_p_slot = var_sta2_p;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stig_p_slot = var_stig_p;
        *var_strs_p_slot = var_strs_p;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatg_p_slot = var_thesatg_p;
    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard104: f64,
        var_guard105: f64,
        var_guard36: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we_edge: f64,
        var_alp1ac_p_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_fntexc_p_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard111_slot: &mut f64,
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
        var_guard123_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_plparam_i_slot: &mut f64,
        var_plwparam_i_slot: &mut f64,
        var_poparam_i_slot: &mut f64,
        var_pwparam_i_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
    ) {
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_fntexc_p: f64 = *var_fntexc_p_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
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
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_plparam_i: f64 = *var_plparam_i_slot;
        let mut var_plwparam_i: f64 = *var_plwparam_i_slot;
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
        let mut var_pwparam_i: f64 = *var_pwparam_i_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;

        let (assign8260_e7599,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard105 != 0.0)) {
        (p.p660,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8260_e7599;

        let (assign8270_e7605,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p569,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8270_e7605;

        let assign8280_e7607: f64 = if param_given[661] { 1.0 } else { 0.0 };
        let assign8280_e7609: f64 = if assign8280_e7607 == 1.0 { 1.0 } else { 0.0 };
        var_guard106 = assign8280_e7609;

        let (assign8290_e7617,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard106 != 0.0)) {
        (p.p661,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8290_e7617;

        let (assign8300_e7623,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p570,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8300_e7623;

        let assign8310_e7625: f64 = if param_given[662] { 1.0 } else { 0.0 };
        let assign8310_e7627: f64 = if assign8310_e7625 == 1.0 { 1.0 } else { 0.0 };
        var_guard107 = assign8310_e7627;

        let (assign8320_e7635,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard107 != 0.0)) {
        (p.p662,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8320_e7635;

        let (assign8330_e7641,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p571,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8330_e7641;

        let assign8340_e7643: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8340_e7645: f64 = if assign8340_e7643 == 1.0 { 1.0 } else { 0.0 };
        var_guard108 = assign8340_e7645;

        let (assign8350_e7653,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard108 != 0.0)) {
        (p.p663,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8350_e7653;

        let (assign8360_e7673,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        let assign8360_e7661: f64 = (var_plparam_i * var_ile);
        let assign8360_e7662: f64 = (var_poparam_i + assign8360_e7661);
        let assign8360_e7665: f64 = (var_pwparam_i * var_iwe);
        let assign8360_e7666: f64 = (assign8360_e7662 + assign8360_e7665);
        let assign8360_e7669: f64 = (var_plwparam_i * var_iae);
        let assign8360_e7670: f64 = (assign8360_e7666 + assign8360_e7669);
        let assign8360_e7671: f64 = (var_ile * assign8360_e7670);
        (assign8360_e7671,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign8360_e7673;

        let assign8370_e7712: f64 = if (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        var_guard109 = assign8370_e7712;

        let (assign8380_e7718,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p584,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8380_e7718;

        let assign8390_e7720: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8390_e7722: f64 = if assign8390_e7720 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign8390_e7722;

        let (assign8400_e7730,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p664,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8400_e7730;

        let (assign8410_e7736,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p585,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8410_e7736;

        let assign8420_e7738: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8420_e7740: f64 = if assign8420_e7738 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign8420_e7740;

        let (assign8430_e7748,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p665,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8430_e7748;

        let (assign8440_e7754,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p586,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8440_e7754;

        let assign8450_e7756: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8450_e7758: f64 = if assign8450_e7756 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign8450_e7758;

        let (assign8460_e7766,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p666,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8460_e7766;

        let (assign8470_e7772,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p587,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8470_e7772;

        let assign8480_e7774: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8480_e7776: f64 = if assign8480_e7774 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign8480_e7776;

        let (assign8490_e7784,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p667,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8490_e7784;

        let (assign8500_e7804,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        let assign8500_e7792: f64 = (var_plparam_i * var_ile);
        let assign8500_e7793: f64 = (var_poparam_i + assign8500_e7792);
        let assign8500_e7796: f64 = (var_pwparam_i * var_iwe);
        let assign8500_e7797: f64 = (assign8500_e7793 + assign8500_e7796);
        let assign8500_e7800: f64 = (var_plwparam_i * var_iae);
        let assign8500_e7801: f64 = (assign8500_e7797 + assign8500_e7800);
        let assign8500_e7802: f64 = assign8500_e7801;
        (assign8500_e7802,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign8500_e7804;

        let assign8510_e7823: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        var_guard114 = assign8510_e7823;

        let (assign8520_e7843,) = {
    if ((var_guard36 != 0.0) && (var_guard114 != 0.0)) {
        let assign8520_e7831: f64 = (p.p669 * var_ile);
        let assign8520_e7832: f64 = (p.p668 + assign8520_e7831);
        let assign8520_e7835: f64 = (p.p670 * var_iwe);
        let assign8520_e7836: f64 = (assign8520_e7832 + assign8520_e7835);
        let assign8520_e7839: f64 = (p.p671 * var_iae);
        let assign8520_e7840: f64 = (assign8520_e7836 + assign8520_e7839);
        let assign8520_e7841: f64 = (var_ile * assign8520_e7840);
        (assign8520_e7841,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign8520_e7843;

        let assign8530_e7862: f64 = if (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) { 1.0 } else { 0.0 };
        var_guard115 = assign8530_e7862;

        let (assign8540_e7882,) = {
    if ((var_guard36 != 0.0) && (var_guard115 != 0.0)) {
        let assign8540_e7870: f64 = (p.p673 * var_ile);
        let assign8540_e7871: f64 = (p.p672 + assign8540_e7870);
        let assign8540_e7874: f64 = (p.p674 * var_iwe);
        let assign8540_e7875: f64 = (assign8540_e7871 + assign8540_e7874);
        let assign8540_e7878: f64 = (p.p675 * var_iae);
        let assign8540_e7879: f64 = (assign8540_e7875 + assign8540_e7878);
        let assign8540_e7880: f64 = (var_ile * assign8540_e7879);
        (assign8540_e7880,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign8540_e7882;

        let assign8550_e7901: f64 = if (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) { 1.0 } else { 0.0 };
        var_guard116 = assign8550_e7901;

        let (assign8560_e7921,) = {
    if ((var_guard36 != 0.0) && (var_guard116 != 0.0)) {
        let assign8560_e7909: f64 = (p.p677 * var_ile);
        let assign8560_e7910: f64 = (p.p676 + assign8560_e7909);
        let assign8560_e7913: f64 = (p.p678 * var_iwe);
        let assign8560_e7914: f64 = (assign8560_e7910 + assign8560_e7913);
        let assign8560_e7917: f64 = (p.p679 * var_iae);
        let assign8560_e7918: f64 = (assign8560_e7914 + assign8560_e7917);
        let assign8560_e7919: f64 = (var_iiwecv * assign8560_e7918);
        (assign8560_e7919,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign8560_e7921;

        let assign8570_e7940: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        var_guard117 = assign8570_e7940;

        let (assign8580_e7960,) = {
    if ((var_guard36 != 0.0) && (var_guard117 != 0.0)) {
        let assign8580_e7948: f64 = (p.p681 * var_ile);
        let assign8580_e7949: f64 = (p.p680 + assign8580_e7948);
        let assign8580_e7952: f64 = (p.p682 * var_iwe);
        let assign8580_e7953: f64 = (assign8580_e7949 + assign8580_e7952);
        let assign8580_e7956: f64 = (p.p683 * var_iae);
        let assign8580_e7957: f64 = (assign8580_e7953 + assign8580_e7956);
        let assign8580_e7958: f64 = (var_iiwecv * assign8580_e7957);
        (assign8580_e7958,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign8580_e7960;

        let assign8590_e7979: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        var_guard118 = assign8590_e7979;

        let (assign8600_e7999,) = {
    if ((var_guard36 != 0.0) && (var_guard118 != 0.0)) {
        let assign8600_e7987: f64 = (p.p685 * var_ile);
        let assign8600_e7988: f64 = (p.p684 + assign8600_e7987);
        let assign8600_e7991: f64 = (p.p686 * var_iwe);
        let assign8600_e7992: f64 = (assign8600_e7988 + assign8600_e7991);
        let assign8600_e7995: f64 = (p.p687 * var_iae);
        let assign8600_e7996: f64 = (assign8600_e7992 + assign8600_e7995);
        let assign8600_e7997: f64 = (var_iilcv * assign8600_e7996);
        (assign8600_e7997,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign8600_e7999;

        let assign8610_e8018: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        var_guard119 = assign8610_e8018;

        let (assign8620_e8038,) = {
    if ((var_guard36 != 0.0) && (var_guard119 != 0.0)) {
        let assign8620_e8026: f64 = (p.p689 * var_ile);
        let assign8620_e8027: f64 = (p.p688 + assign8620_e8026);
        let assign8620_e8030: f64 = (p.p690 * var_iwe);
        let assign8620_e8031: f64 = (assign8620_e8027 + assign8620_e8030);
        let assign8620_e8034: f64 = (p.p691 * var_iae);
        let assign8620_e8035: f64 = (assign8620_e8031 + assign8620_e8034);
        let assign8620_e8036: f64 = (var_iiwecv * assign8620_e8035);
        (assign8620_e8036,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign8620_e8038;

        let assign8630_e8057: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        var_guard120 = assign8630_e8057;

        let (assign8640_e8077,) = {
    if ((var_guard36 != 0.0) && (var_guard120 != 0.0)) {
        let assign8640_e8065: f64 = (p.p693 * var_ile);
        let assign8640_e8066: f64 = (p.p692 + assign8640_e8065);
        let assign8640_e8069: f64 = (p.p694 * var_iwe);
        let assign8640_e8070: f64 = (assign8640_e8066 + assign8640_e8069);
        let assign8640_e8073: f64 = (p.p695 * var_iae);
        let assign8640_e8074: f64 = (assign8640_e8070 + assign8640_e8073);
        let assign8640_e8075: f64 = (var_iiwecv * assign8640_e8074);
        (assign8640_e8075,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign8640_e8077;

        let assign8650_e8096: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };
        var_guard121 = assign8650_e8096;

        let (assign8660_e8116,) = {
    if ((var_guard36 != 0.0) && (var_guard121 != 0.0)) {
        let assign8660_e8104: f64 = (p.p697 * var_ile);
        let assign8660_e8105: f64 = (p.p696 + assign8660_e8104);
        let assign8660_e8108: f64 = (p.p698 * var_iwe);
        let assign8660_e8109: f64 = (assign8660_e8105 + assign8660_e8108);
        let assign8660_e8112: f64 = (p.p699 * var_iae);
        let assign8660_e8113: f64 = (assign8660_e8109 + assign8660_e8112);
        let assign8660_e8114: f64 = (var_iiwcv * assign8660_e8113);
        (assign8660_e8114,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign8660_e8116;

        let assign8670_e8135: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };
        var_guard122 = assign8670_e8135;

        let (assign8680_e8155,) = {
    if ((var_guard36 != 0.0) && (var_guard122 != 0.0)) {
        let assign8680_e8143: f64 = (p.p701 * var_ile);
        let assign8680_e8144: f64 = (p.p700 + assign8680_e8143);
        let assign8680_e8147: f64 = (p.p702 * var_iwe);
        let assign8680_e8148: f64 = (assign8680_e8144 + assign8680_e8147);
        let assign8680_e8151: f64 = (p.p703 * var_iae);
        let assign8680_e8152: f64 = (assign8680_e8148 + assign8680_e8151);
        let assign8680_e8153: f64 = (var_iiwcv * assign8680_e8152);
        (assign8680_e8153,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign8680_e8155;

        let assign8690_e8174: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };
        var_guard123 = assign8690_e8174;

        let (assign8700_e8194,) = {
    if ((var_guard36 != 0.0) && (var_guard123 != 0.0)) {
        let assign8700_e8182: f64 = (p.p705 * var_ile);
        let assign8700_e8183: f64 = (p.p704 + assign8700_e8182);
        let assign8700_e8186: f64 = (p.p706 * var_iwe);
        let assign8700_e8187: f64 = (assign8700_e8183 + assign8700_e8186);
        let assign8700_e8190: f64 = (p.p707 * var_iae);
        let assign8700_e8191: f64 = (assign8700_e8187 + assign8700_e8190);
        let assign8700_e8192: f64 = (var_ile2 * assign8700_e8191);
        (assign8700_e8192,)
    } else {
        (var_fntexc_p,)
    }
};
        var_fntexc_p = assign8700_e8194;

        let assign8770_e8330: f64 = if (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]) { 1.0 } else { 0.0 };
        var_guard127 = assign8770_e8330;

        let (assign8780_e8348,) = {
    if ((var_guard36 != 0.0) && (var_guard127 != 0.0)) {
        let assign8780_e8337: f64 = (p.p721 * var_ile);
        let assign8780_e8338: f64 = (p.p720 + assign8780_e8337);
        let assign8780_e8341: f64 = (p.p722 * var_iwe);
        let assign8780_e8342: f64 = (assign8780_e8338 + assign8780_e8341);
        let assign8780_e8345: f64 = (p.p723 * var_iae);
        let assign8780_e8346: f64 = (assign8780_e8342 + assign8780_e8345);
        (assign8780_e8346,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign8780_e8348;

        let assign8790_e8367: f64 = if (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]) { 1.0 } else { 0.0 };
        var_guard128 = assign8790_e8367;

        let (assign8800_e8385,) = {
    if ((var_guard36 != 0.0) && (var_guard128 != 0.0)) {
        let assign8800_e8374: f64 = (p.p725 * var_ile);
        let assign8800_e8375: f64 = (p.p724 + assign8800_e8374);
        let assign8800_e8378: f64 = (p.p726 * var_iwe);
        let assign8800_e8379: f64 = (assign8800_e8375 + assign8800_e8378);
        let assign8800_e8382: f64 = (p.p727 * var_iae);
        let assign8800_e8383: f64 = (assign8800_e8379 + assign8800_e8382);
        (assign8800_e8383,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign8800_e8385;

        let assign8810_e8404: f64 = if (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]) { 1.0 } else { 0.0 };
        var_guard129 = assign8810_e8404;

        let (assign8820_e8422,) = {
    if ((var_guard36 != 0.0) && (var_guard129 != 0.0)) {
        let assign8820_e8411: f64 = (p.p729 * var_ile);
        let assign8820_e8412: f64 = (p.p728 + assign8820_e8411);
        let assign8820_e8415: f64 = (p.p730 * var_iwe);
        let assign8820_e8416: f64 = (assign8820_e8412 + assign8820_e8415);
        let assign8820_e8419: f64 = (p.p731 * var_iae);
        let assign8820_e8420: f64 = (assign8820_e8416 + assign8820_e8419);
        (assign8820_e8420,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign8820_e8422;

        let assign8830_e8441: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        var_guard130 = assign8830_e8441;

        let (assign8840_e8459,) = {
    if ((var_guard36 != 0.0) && (var_guard130 != 0.0)) {
        let assign8840_e8448: f64 = (p.p733 * var_ile);
        let assign8840_e8449: f64 = (p.p732 + assign8840_e8448);
        let assign8840_e8452: f64 = (p.p734 * var_iwe);
        let assign8840_e8453: f64 = (assign8840_e8449 + assign8840_e8452);
        let assign8840_e8456: f64 = (p.p735 * var_iae);
        let assign8840_e8457: f64 = (assign8840_e8453 + assign8840_e8456);
        (assign8840_e8457,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign8840_e8459;

        let assign8850_e8478: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        var_guard131 = assign8850_e8478;

        let (assign8860_e8496,) = {
    if ((var_guard36 != 0.0) && (var_guard131 != 0.0)) {
        let assign8860_e8485: f64 = (p.p737 * var_ile);
        let assign8860_e8486: f64 = (p.p736 + assign8860_e8485);
        let assign8860_e8489: f64 = (p.p738 * var_iwe);
        let assign8860_e8490: f64 = (assign8860_e8486 + assign8860_e8489);
        let assign8860_e8493: f64 = (p.p739 * var_iae);
        let assign8860_e8494: f64 = (assign8860_e8490 + assign8860_e8493);
        (assign8860_e8494,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign8860_e8496;

        let assign8870_e8515: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        var_guard132 = assign8870_e8515;

        let (assign8880_e8537,) = {
    if ((var_guard36 != 0.0) && (var_guard132 != 0.0)) {
        let assign8880_e8521: f64 = (var_we_edge / var_le);
        let assign8880_e8525: f64 = (p.p741 * var_ile);
        let assign8880_e8526: f64 = (p.p740 + assign8880_e8525);
        let assign8880_e8529: f64 = (p.p742 * var_iwe);
        let assign8880_e8530: f64 = (assign8880_e8526 + assign8880_e8529);
        let assign8880_e8533: f64 = (p.p743 * var_iae);
        let assign8880_e8534: f64 = (assign8880_e8530 + assign8880_e8533);
        let assign8880_e8535: f64 = (assign8880_e8521 * assign8880_e8534);
        (assign8880_e8535,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign8880_e8537;

        let assign8890_e8556: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        var_guard133 = assign8890_e8556;

        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alpac_p_slot = var_alpac_p;
        *var_axac_p_slot = var_axac_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_fntexc_p_slot = var_fntexc_p;
        *var_guard106_slot = var_guard106;
        *var_guard107_slot = var_guard107;
        *var_guard108_slot = var_guard108;
        *var_guard109_slot = var_guard109;
        *var_guard110_slot = var_guard110;
        *var_guard111_slot = var_guard111;
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
        *var_guard123_slot = var_guard123;
        *var_guard127_slot = var_guard127;
        *var_guard128_slot = var_guard128;
        *var_guard129_slot = var_guard129;
        *var_guard130_slot = var_guard130;
        *var_guard131_slot = var_guard131;
        *var_guard132_slot = var_guard132;
        *var_guard133_slot = var_guard133;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_plparam_i_slot = var_plparam_i;
        *var_plwparam_i_slot = var_plwparam_i;
        *var_poparam_i_slot = var_poparam_i;
        *var_pwparam_i_slot = var_pwparam_i;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dellps: f64,
        var_delwod: f64,
        var_guard133: f64,
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_invnf: f64,
        var_iwe: f64,
        var_l_i: f64,
        var_nf_i: f64,
        var_rta: f64,
        var_sa_i: f64,
        var_sb_i: f64,
        var_sd_i: f64,
        var_w_i: f64,
        var_betn_p_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_kvsatac_i_slot: &mut f64,
        var_loop__slot: &mut f64,
        var_lx_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp00_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_wx_slot: &mut f64,
    ) {
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_kvsatac_i: f64 = *var_kvsatac_i_slot;
        let mut var_loop_: f64 = *var_loop__slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp00: f64 = *var_temp00_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_wx: f64 = *var_wx_slot;

        let (assign8900_e8574,) = {
    if ((var_guard36 != 0.0) && (var_guard133 != 0.0)) {
        let assign8900_e8563: f64 = (p.p745 * var_ile);
        let assign8900_e8564: f64 = (p.p744 + assign8900_e8563);
        let assign8900_e8567: f64 = (p.p746 * var_iwe);
        let assign8900_e8568: f64 = (assign8900_e8564 + assign8900_e8567);
        let assign8900_e8571: f64 = (p.p747 * var_iae);
        let assign8900_e8572: f64 = (assign8900_e8568 + assign8900_e8571);
        (assign8900_e8572,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign8900_e8574;

        let assign8910_e8593: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        var_guard134 = assign8910_e8593;

        let (assign8920_e8613,) = {
    if ((var_guard36 != 0.0) && (var_guard134 != 0.0)) {
        let assign8920_e8601: f64 = (p.p749 * var_ile);
        let assign8920_e8602: f64 = (p.p748 + assign8920_e8601);
        let assign8920_e8605: f64 = (p.p750 * var_iwe);
        let assign8920_e8606: f64 = (assign8920_e8602 + assign8920_e8605);
        let assign8920_e8609: f64 = (p.p751 * var_iae);
        let assign8920_e8610: f64 = (assign8920_e8606 + assign8920_e8609);
        let assign8920_e8611: f64 = (var_ile2 * assign8920_e8610);
        (assign8920_e8611,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign8920_e8613;

        let assign8930_e8632: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        var_guard135 = assign8930_e8632;

        let (assign8940_e8650,) = {
    if ((var_guard36 != 0.0) && (var_guard135 != 0.0)) {
        let assign8940_e8639: f64 = (p.p753 * var_ile);
        let assign8940_e8640: f64 = (p.p752 + assign8940_e8639);
        let assign8940_e8643: f64 = (p.p754 * var_iwe);
        let assign8940_e8644: f64 = (assign8940_e8640 + assign8940_e8643);
        let assign8940_e8647: f64 = (p.p755 * var_iae);
        let assign8940_e8648: f64 = (assign8940_e8644 + assign8940_e8647);
        (assign8940_e8648,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign8940_e8650;

        let assign8950_e8669: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        var_guard136 = assign8950_e8669;

        let (assign8960_e8687,) = {
    if ((var_guard36 != 0.0) && (var_guard136 != 0.0)) {
        let assign8960_e8676: f64 = (p.p757 * var_ile);
        let assign8960_e8677: f64 = (p.p756 + assign8960_e8676);
        let assign8960_e8680: f64 = (p.p758 * var_iwe);
        let assign8960_e8681: f64 = (assign8960_e8677 + assign8960_e8680);
        let assign8960_e8684: f64 = (p.p759 * var_iae);
        let assign8960_e8685: f64 = (assign8960_e8681 + assign8960_e8684);
        (assign8960_e8685,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign8960_e8687;

        let assign8970_e8706: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        var_guard137 = assign8970_e8706;

        let (assign8980_e8726,) = {
    if ((var_guard36 != 0.0) && (var_guard137 != 0.0)) {
        let assign8980_e8714: f64 = (p.p761 * var_ile);
        let assign8980_e8715: f64 = (p.p760 + assign8980_e8714);
        let assign8980_e8718: f64 = (p.p762 * var_iwe);
        let assign8980_e8719: f64 = (assign8980_e8715 + assign8980_e8718);
        let assign8980_e8722: f64 = (p.p763 * var_iae);
        let assign8980_e8723: f64 = (assign8980_e8719 + assign8980_e8722);
        let assign8980_e8724: f64 = (var_ile2 * assign8980_e8723);
        (assign8980_e8724,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign8980_e8726;

        let assign8990_e8745: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        var_guard138 = assign8990_e8745;

        let (assign9000_e8763,) = {
    if ((var_guard36 != 0.0) && (var_guard138 != 0.0)) {
        let assign9000_e8752: f64 = (p.p769 * var_ile);
        let assign9000_e8753: f64 = (p.p768 + assign9000_e8752);
        let assign9000_e8756: f64 = (p.p770 * var_iwe);
        let assign9000_e8757: f64 = (assign9000_e8753 + assign9000_e8756);
        let assign9000_e8760: f64 = (p.p771 * var_iae);
        let assign9000_e8761: f64 = (assign9000_e8757 + assign9000_e8760);
        (assign9000_e8761,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign9000_e8763;

        let assign9010_e8782: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        var_guard139 = assign9010_e8782;

        let (assign9020_e8800,) = {
    if ((var_guard36 != 0.0) && (var_guard139 != 0.0)) {
        let assign9020_e8789: f64 = (p.p765 * var_ile);
        let assign9020_e8790: f64 = (p.p764 + assign9020_e8789);
        let assign9020_e8793: f64 = (p.p766 * var_iwe);
        let assign9020_e8794: f64 = (assign9020_e8790 + assign9020_e8793);
        let assign9020_e8797: f64 = (p.p767 * var_iae);
        let assign9020_e8798: f64 = (assign9020_e8794 + assign9020_e8797);
        (assign9020_e8798,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign9020_e8800;

        let (assign9090_e8921,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_tmpa,)
    }
};
        var_tmpa = assign9090_e8921;

        let (assign9100_e8925,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign9100_e8925;

        let (assign9110_e8929,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_loop_,)
    }
};
        var_loop_ = assign9110_e8929;

        let (assign9120_e8933,) = {
    if (var_guard36 != 0.0) {
        (p.p788,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9120_e8933;

        let assign9130_e8935: f64 = if param_given[789] { 1.0 } else { 0.0 };
        let assign9130_e8937: f64 = if assign9130_e8935 == 1.0 { 1.0 } else { 0.0 };
        var_guard143 = assign9130_e8937;

        let (assign9140_e8943,) = {
    if ((var_guard36 != 0.0) && (var_guard143 != 0.0)) {
        (p.p789,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9140_e8943;

        let assign9150_e8962: f64 = if (((var_sa_i > 0.0) && (var_sb_i > 0.0)) && ((var_nf_i == 1.0) || ((var_nf_i > 1.0) && (var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        var_guard144 = assign9150_e8962;

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (var_nf_i - 0.5);
            let assign9160_cond_e8971: f64 = if (((var_guard36 != 0.0) && (var_guard144 != 0.0)) && (var_loop_ < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9160_body0_e8991,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9160_body0_e8980: f64 = (0.5 * var_l_i);
        let assign9160_body0_e8981: f64 = (var_sa_i + assign9160_body0_e8980);
        let assign9160_body0_e8985: f64 = (var_sd_i + var_l_i);
        let assign9160_body0_e8986: f64 = (var_loop_ * assign9160_body0_e8985);
        let assign9160_body0_e8987: f64 = (assign9160_body0_e8981 + assign9160_body0_e8986);
        let assign9160_body0_e8988: f64 = (1.0 / assign9160_body0_e8987);
        let assign9160_body0_e8989: f64 = (var_tmpa + assign9160_body0_e8988);
        (assign9160_body0_e8989,)
    } else {
        (var_tmpa,)
    }
};
            var_tmpa = assign9160_body0_e8991;
            let (assign9160_body1_e9011,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9160_body1_e9000: f64 = (0.5 * var_l_i);
        let assign9160_body1_e9001: f64 = (var_sb_i + assign9160_body1_e9000);
        let assign9160_body1_e9005: f64 = (var_sd_i + var_l_i);
        let assign9160_body1_e9006: f64 = (var_loop_ * assign9160_body1_e9005);
        let assign9160_body1_e9007: f64 = (assign9160_body1_e9001 + assign9160_body1_e9006);
        let assign9160_body1_e9008: f64 = (1.0 / assign9160_body1_e9007);
        let assign9160_body1_e9009: f64 = (var_tmpb + assign9160_body1_e9008);
        (assign9160_body1_e9009,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign9160_body1_e9011;
            let (assign9160_body2_e9019,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9160_body2_e9017: f64 = (var_loop_ + 1.0);
        (assign9160_body2_e9017,)
    } else {
        (var_loop_,)
    }
};
            var_loop_ = assign9160_body2_e9019;
        }

        let (assign9170_e9027,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9170_e9025: f64 = (var_tmpa * var_invnf);
        (assign9170_e9025,)
    } else {
        (var_invsa,)
    }
};
        var_invsa = assign9170_e9027;

        let (assign9180_e9035,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9180_e9033: f64 = (var_tmpb * var_invnf);
        (assign9180_e9033,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign9180_e9035;

        let (assign9190_e9047,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9190_e9043: f64 = (0.5 * var_l_i);
        let assign9190_e9044: f64 = (p.p784 + assign9190_e9043);
        let assign9190_e9045: f64 = (1.0 / assign9190_e9044);
        (assign9190_e9045,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign9190_e9047;

        let (assign9200_e9059,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9200_e9055: f64 = (0.5 * var_l_i);
        let assign9200_e9056: f64 = (p.p785 + assign9200_e9055);
        let assign9200_e9057: f64 = (1.0 / assign9200_e9056);
        (assign9200_e9057,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign9200_e9059;

        let (assign9210_e9074,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9210_e9065: f64 = (var_l_i + var_dellps);
        let (assign9210_e9072,) = {
            if (assign9210_e9065 > 1e-9) {
                let assign9210_e9070: f64 = (var_l_i + var_dellps);
                (assign9210_e9070,)
            } else {
                (1e-9,)
            }
        };
        (assign9210_e9072,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign9210_e9074;

        let (assign9220_e9093,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9220_e9080: f64 = (var_w_i + var_delwod);
        let assign9220_e9082: f64 = (assign9220_e9080 + p.p786);
        let (assign9220_e9091,) = {
            if (assign9220_e9082 > 1e-9) {
                let assign9220_e9087: f64 = (var_w_i + var_delwod);
                let assign9220_e9089: f64 = (assign9220_e9087 + p.p786);
                (assign9220_e9089,)
            } else {
                (1e-9,)
            }
        };
        (assign9220_e9091,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign9220_e9093;

        let (assign9230_e9103,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9230_e9100: f64 = (var_lx).powf(p.p794);
        let assign9230_e9101: f64 = (1.0 / assign9230_e9100);
        (assign9230_e9101,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9230_e9103;

        let (assign9240_e9113,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9240_e9110: f64 = (var_wx).powf(p.p795);
        let assign9240_e9111: f64 = (1.0 / assign9240_e9110);
        (assign9240_e9111,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9240_e9113;

        let (assign9250_e9141,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9250_e9120: f64 = (p.p791 * var_templ);
        let assign9250_e9121: f64 = (1.0 + assign9250_e9120);
        let assign9250_e9124: f64 = (p.p792 * var_tempw);
        let assign9250_e9125: f64 = (assign9250_e9121 + assign9250_e9124);
        let assign9250_e9128: f64 = (p.p793 * var_templ);
        let assign9250_e9130: f64 = (assign9250_e9128 * var_tempw);
        let assign9250_e9131: f64 = (assign9250_e9125 + assign9250_e9130);
        let assign9250_e9136: f64 = (var_rta - 1.0);
        let assign9250_e9137: f64 = (p.p790 * assign9250_e9136);
        let assign9250_e9138: f64 = (1.0 + assign9250_e9137);
        let assign9250_e9139: f64 = (assign9250_e9131 * assign9250_e9138);
        (assign9250_e9139,)
    } else {
        (var_kstressu0,)
    }
};
        var_kstressu0 = assign9250_e9141;

        let (assign9260_e9153,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9260_e9148: f64 = (var_invsa + var_invsb);
        let assign9260_e9149: f64 = (p.p787 * assign9260_e9148);
        let assign9260_e9151: f64 = (assign9260_e9149 / var_kstressu0);
        (assign9260_e9151,)
    } else {
        (var_rhobeta,)
    }
};
        var_rhobeta = assign9260_e9153;

        let (assign9270_e9165,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9270_e9160: f64 = (var_invsaref + var_invsbref);
        let assign9270_e9161: f64 = (p.p787 * assign9270_e9160);
        let assign9270_e9163: f64 = (assign9270_e9161 / var_kstressu0);
        (assign9270_e9163,)
    } else {
        (var_rhobetaref,)
    }
};
        var_rhobetaref = assign9270_e9165;

        let (assign9280_e9175,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9280_e9172: f64 = (var_lx).powf(p.p800);
        let assign9280_e9173: f64 = (1.0 / assign9280_e9172);
        (assign9280_e9173,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9280_e9175;

        let (assign9290_e9185,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9290_e9182: f64 = (var_wx).powf(p.p801);
        let assign9290_e9183: f64 = (1.0 / assign9290_e9182);
        (assign9290_e9183,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9290_e9185;

        let (assign9300_e9205,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9300_e9192: f64 = (p.p797 * var_templ);
        let assign9300_e9193: f64 = (1.0 + assign9300_e9192);
        let assign9300_e9196: f64 = (p.p798 * var_tempw);
        let assign9300_e9197: f64 = (assign9300_e9193 + assign9300_e9196);
        let assign9300_e9200: f64 = (p.p799 * var_templ);
        let assign9300_e9202: f64 = (assign9300_e9200 * var_tempw);
        let assign9300_e9203: f64 = (assign9300_e9197 + assign9300_e9202);
        (assign9300_e9203,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign9300_e9205;

        let (assign9310_e9217,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9310_e9211: f64 = (var_invsa + var_invsb);
        let assign9310_e9213: f64 = (assign9310_e9211 - var_invsaref);
        let assign9310_e9215: f64 = (assign9310_e9213 - var_invsbref);
        (assign9310_e9215,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9310_e9217;

        let (assign9320_e9229,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9320_e9223: f64 = (1.0 + var_rhobeta);
        let assign9320_e9226: f64 = (1.0 + var_rhobetaref);
        let assign9320_e9227: f64 = (assign9320_e9223 / assign9320_e9226);
        (assign9320_e9227,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9320_e9229;

        let (assign9330_e9237,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9330_e9235: f64 = (var_betn_p * var_temp00);
        (assign9330_e9235,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9330_e9237;

        let (assign9340_e9257,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9340_e9243: f64 = (var_thesat_p * var_temp00);
        let assign9340_e9247: f64 = (p.p788 * var_rhobetaref);
        let assign9340_e9248: f64 = (1.0 + assign9340_e9247);
        let assign9340_e9249: f64 = (assign9340_e9243 * assign9340_e9248);
        let assign9340_e9253: f64 = (p.p788 * var_rhobeta);
        let assign9340_e9254: f64 = (1.0 + assign9340_e9253);
        let assign9340_e9255: f64 = (assign9340_e9249 / assign9340_e9254);
        (assign9340_e9255,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign9340_e9257;

        let (assign9350_e9277,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9350_e9263: f64 = (var_thesatac_p * var_temp00);
        let assign9350_e9267: f64 = (var_kvsatac_i * var_rhobetaref);
        let assign9350_e9268: f64 = (1.0 + assign9350_e9267);
        let assign9350_e9269: f64 = (assign9350_e9263 * assign9350_e9268);
        let assign9350_e9273: f64 = (var_kvsatac_i * var_rhobeta);
        let assign9350_e9274: f64 = (1.0 + assign9350_e9273);
        let assign9350_e9275: f64 = (assign9350_e9269 / assign9350_e9274);
        (assign9350_e9275,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign9350_e9277;

        let (assign9360_e9285,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9360_e9283: f64 = (var_betnedge_p * var_temp00);
        (assign9360_e9283,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9360_e9285;

        let (assign9370_e9295,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9370_e9291: f64 = (p.p796 * var_temp0);
        let assign9370_e9293: f64 = (assign9370_e9291 / var_kstressvth0);
        (assign9370_e9293,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9370_e9295;

        let (assign9380_e9303,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9380_e9301: f64 = (var_vfb_p + var_temp00);
        (assign9380_e9301,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9380_e9303;

        *var_betn_p_slot = var_betn_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_guard134_slot = var_guard134;
        *var_guard135_slot = var_guard135;
        *var_guard136_slot = var_guard136;
        *var_guard137_slot = var_guard137;
        *var_guard138_slot = var_guard138;
        *var_guard139_slot = var_guard139;
        *var_guard143_slot = var_guard143;
        *var_guard144_slot = var_guard144;
        *var_invsa_slot = var_invsa;
        *var_invsaref_slot = var_invsaref;
        *var_invsb_slot = var_invsb;
        *var_invsbref_slot = var_invsbref;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_kvsatac_i_slot = var_kvsatac_i;
        *var_loop__slot = var_loop_;
        *var_lx_slot = var_lx;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_temp0_slot = var_temp0;
        *var_temp00_slot = var_temp00;
        *var_templ_slot = var_templ;
        *var_tempw_slot = var_tempw;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_tmpa_slot = var_tmpa;
        *var_tmpb_slot = var_tmpb;
        *var_vfb_p_slot = var_vfb_p;
        *var_wx_slot = var_wx;
    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
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
        var_guard144: f64,
        var_guard36: f64,
        var_kstressvth0: f64,
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
        var_st2vfb_p: f64,
        var_stbet_p: f64,
        var_stcs_p: f64,
        var_stct_p: f64,
        var_stmue_p: f64,
        var_strs_p: f64,
        var_stthecs_p: f64,
        var_stthemu_p: f64,
        var_stvfb_p: f64,
        var_stxcor_p: f64,
        var_thecs_p: f64,
        var_themu_p: f64,
        var_tox_p: f64,
        var_toxov_p: f64,
        var_toxovd_p: f64,
        var_vsbnud_p: f64,
        var_w_i: f64,
        var_xcor_p: f64,
        var_betn_i_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_cf_i_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cfb_i_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cs_i_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_ctb_i_slot: &mut f64,
        var_ctg_i_slot: &mut f64,
        var_dphib_i_slot: &mut f64,
        var_dvsbnud_i_slot: &mut f64,
        var_epsrox_i_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_gfacnud_i_slot: &mut f64,
        var_guard145_slot: &mut f64,
        var_guard146_slot: &mut f64,
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
        var_sca_i_slot: &mut f64,
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
        var_stvfb_i_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp00_slot: &mut f64,
        var_thecs_i_slot: &mut f64,
        var_themu_i_slot: &mut f64,
        var_tox_i_slot: &mut f64,
        var_toxov_i_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_vfb_i_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vsbnud_i_slot: &mut f64,
        var_xcor_i_slot: &mut f64,
    ) {
        let mut var_betn_i: f64 = *var_betn_i_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_cf_i: f64 = *var_cf_i_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cfb_i: f64 = *var_cfb_i_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cs_i: f64 = *var_cs_i_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_ctb_i: f64 = *var_ctb_i_slot;
        let mut var_ctg_i: f64 = *var_ctg_i_slot;
        let mut var_dphib_i: f64 = *var_dphib_i_slot;
        let mut var_dvsbnud_i: f64 = *var_dvsbnud_i_slot;
        let mut var_epsrox_i: f64 = *var_epsrox_i_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_gfacnud_i: f64 = *var_gfacnud_i_slot;
        let mut var_guard145: f64 = *var_guard145_slot;
        let mut var_guard146: f64 = *var_guard146_slot;
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
        let mut var_sca_i: f64 = *var_sca_i_slot;
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
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp00: f64 = *var_temp00_slot;
        let mut var_thecs_i: f64 = *var_thecs_i_slot;
        let mut var_themu_i: f64 = *var_themu_i_slot;
        let mut var_tox_i: f64 = *var_tox_i_slot;
        let mut var_toxov_i: f64 = *var_toxov_i_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_vfb_i: f64 = *var_vfb_i_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vsbnud_i: f64 = *var_vsbnud_i_slot;
        let mut var_xcor_i: f64 = *var_xcor_i_slot;

        let (assign9390_e9311,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9390_e9309: f64 = (var_vfbedge_p + var_temp00);
        (assign9390_e9309,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9390_e9311;

        let (assign9400_e9323,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9400_e9317: f64 = (p.p802 * var_temp0);
        let assign9400_e9320: f64 = (var_kstressvth0).powf(p.p803);
        let assign9400_e9321: f64 = (assign9400_e9317 / assign9400_e9320);
        (assign9400_e9321,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9400_e9323;

        let (assign9410_e9331,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9410_e9329: f64 = (var_cf_p + var_temp00);
        (assign9410_e9329,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign9410_e9331;

        let (assign9420_e9339,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9420_e9337: f64 = (var_cfedge_p + var_temp00);
        (assign9420_e9337,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign9420_e9339;

        let assign9430_e9354: f64 = if ((((var_sca_i > 0.0) || (var_scb_i > 0.0)) || (var_scc_i > 0.0)) || (var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard145 = assign9430_e9354;

        let assign9440_e9365: f64 = if (((var_sca_i == 0.0) && (var_scb_i == 0.0)) && (var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard146 = assign9440_e9365;

        let (assign9450_e9375,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9450_e9373: f64 = (var_sc_i + var_w_i);
        (assign9450_e9373,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9450_e9375;

        let (assign9460_e9385,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9460_e9383: f64 = (1.0 / p.p804);
        (assign9460_e9383,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9460_e9385;

        let (assign9470_e9399,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9470_e9393: f64 = (p.p804 * p.p804);
        let assign9470_e9396: f64 = (var_sc_i * var_temp0);
        let assign9470_e9397: f64 = (assign9470_e9393 / assign9470_e9396);
        (assign9470_e9397,)
    } else {
        (var_sca_i,)
    }
};
        var_sca_i = assign9470_e9399;

        let (assign9480_e9439,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9480_e9407: f64 = (0.1 * var_sc_i);
        let assign9480_e9410: f64 = (0.01 * p.p804);
        let assign9480_e9411: f64 = (assign9480_e9407 + assign9480_e9410);
        let assign9480_e9413: f64 = (-10.0);
        let assign9480_e9415: f64 = (assign9480_e9413 * var_sc_i);
        let assign9480_e9417: f64 = (assign9480_e9415 * var_temp00);
        let assign9480_e9418: f64 = (assign9480_e9417).exp();
        let assign9480_e9419: f64 = (assign9480_e9411 * assign9480_e9418);
        let assign9480_e9422: f64 = (0.1 * var_temp0);
        let assign9480_e9425: f64 = (0.01 * p.p804);
        let assign9480_e9426: f64 = (assign9480_e9422 + assign9480_e9425);
        let assign9480_e9428: f64 = (-10.0);
        let assign9480_e9430: f64 = (assign9480_e9428 * var_temp0);
        let assign9480_e9432: f64 = (assign9480_e9430 * var_temp00);
        let assign9480_e9433: f64 = (assign9480_e9432).exp();
        let assign9480_e9434: f64 = (assign9480_e9426 * assign9480_e9433);
        let assign9480_e9435: f64 = (assign9480_e9419 - assign9480_e9434);
        let assign9480_e9437: f64 = (assign9480_e9435 / var_w_i);
        (assign9480_e9437,)
    } else {
        (var_scb_i,)
    }
};
        var_scb_i = assign9480_e9439;

        let (assign9490_e9479,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9490_e9447: f64 = (0.05 * var_sc_i);
        let assign9490_e9450: f64 = (0.0025 * p.p804);
        let assign9490_e9451: f64 = (assign9490_e9447 + assign9490_e9450);
        let assign9490_e9453: f64 = (-20.0);
        let assign9490_e9455: f64 = (assign9490_e9453 * var_sc_i);
        let assign9490_e9457: f64 = (assign9490_e9455 * var_temp00);
        let assign9490_e9458: f64 = (assign9490_e9457).exp();
        let assign9490_e9459: f64 = (assign9490_e9451 * assign9490_e9458);
        let assign9490_e9462: f64 = (0.05 * var_temp0);
        let assign9490_e9465: f64 = (0.0025 * p.p804);
        let assign9490_e9466: f64 = (assign9490_e9462 + assign9490_e9465);
        let assign9490_e9468: f64 = (-20.0);
        let assign9490_e9470: f64 = (assign9490_e9468 * var_temp0);
        let assign9490_e9472: f64 = (assign9490_e9470 * var_temp00);
        let assign9490_e9473: f64 = (assign9490_e9472).exp();
        let assign9490_e9474: f64 = (assign9490_e9466 * assign9490_e9473);
        let assign9490_e9475: f64 = (assign9490_e9459 - assign9490_e9474);
        let assign9490_e9477: f64 = (assign9490_e9475 / var_w_i);
        (assign9490_e9477,)
    } else {
        (var_scc_i,)
    }
};
        var_scc_i = assign9490_e9479;

        let (assign9500_e9493,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9500_e9486: f64 = (p.p805 * var_scb_i);
        let assign9500_e9487: f64 = (var_sca_i + assign9500_e9486);
        let assign9500_e9490: f64 = (p.p806 * var_scc_i);
        let assign9500_e9491: f64 = (assign9500_e9487 + assign9500_e9490);
        (assign9500_e9491,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9500_e9493;

        let (assign9510_e9503,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9510_e9500: f64 = (var_kvthowe * var_temp0);
        let assign9510_e9501: f64 = (var_vfb_p + assign9510_e9500);
        (assign9510_e9501,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9510_e9503;

        let (assign9520_e9515,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9520_e9511: f64 = (var_kuowe * var_temp0);
        let assign9520_e9512: f64 = (1.0 + assign9520_e9511);
        let assign9520_e9513: f64 = (var_betn_p * assign9520_e9512);
        (assign9520_e9513,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9520_e9515;

        let (assign9530_e9525,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9530_e9522: f64 = (var_kvthowe * var_temp0);
        let assign9530_e9523: f64 = (var_vfbedge_p + assign9530_e9522);
        (assign9530_e9523,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9530_e9525;

        let (assign9540_e9537,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9540_e9533: f64 = (var_kuowe * var_temp0);
        let assign9540_e9534: f64 = (1.0 + assign9540_e9533);
        let assign9540_e9535: f64 = (var_betnedge_p * assign9540_e9534);
        (assign9540_e9535,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9540_e9537;

        var_vfb_i = var_vfb_p;

        var_stvfb_i = var_stvfb_p;

        var_st2vfb_i = var_st2vfb_p;

        var_tox_i = var_tox_p;

        var_epsrox_i = var_epsrox_p;

        let (assign9600_e9553,) = {
    if (var_neff_p > 1e20) {
        let (assign9600_e9551,) = {
            if (var_neff_p < 1e26) {
                (var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9600_e9551,)
    } else {
        (1e20,)
    }
};
        var_neff_i = assign9600_e9553;

        let (assign9610_e9559,) = {
    if (var_gfacnud_p > 0.01) {
        (var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        var_gfacnud_i = assign9610_e9559;

        let (assign9620_e9565,) = {
    if (var_vsbnud_p > 0.0) {
        (var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        var_vsbnud_i = assign9620_e9565;

        var_dvsbnud_i = var_dvsbnud_p;

        var_dphib_i = var_dphib_p;

        let (assign9650_e9573,) = {
    if (var_np_p > 0.0) {
        (var_np_p,)
    } else {
        (0.0,)
    }
};
        var_np_i = assign9650_e9573;

        var_toxov_i = var_toxov_p;

        var_toxovd_i = var_toxovd_p;

        let (assign9680_e9586,) = {
    if (var_nov_p > 1e23) {
        let (assign9680_e9584,) = {
            if (var_nov_p < 1e27) {
                (var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9680_e9584,)
    } else {
        (1e23,)
    }
};
        var_nov_i = assign9680_e9586;

        let (assign9690_e9597,) = {
    if (var_novd_p > 1e23) {
        let (assign9690_e9595,) = {
            if (var_novd_p < 1e27) {
                (var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9690_e9595,)
    } else {
        (1e23,)
    }
};
        var_novd_i = assign9690_e9597;

        let (assign9700_e9603,) = {
    if (var_ct_p > 0.0) {
        (var_ct_p,)
    } else {
        (0.0,)
    }
};
        var_ct_i = assign9700_e9603;

        let (assign9710_e9614,) = {
    if (var_ctb_p > 0.0) {
        let (assign9710_e9612,) = {
            if (var_ctb_p < 0.5) {
                (var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9710_e9612,)
    } else {
        (0.0,)
    }
};
        var_ctb_i = assign9710_e9614;

        let (assign9720_e9625,) = {
    if (var_ctg_p > 0.0) {
        let (assign9720_e9623,) = {
            if (var_ctg_p < 1.0) {
                (var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9720_e9623,)
    } else {
        (0.0,)
    }
};
        var_ctg_i = assign9720_e9625;

        var_stct_i = var_stct_p;

        let (assign9740_e9632,) = {
    if (var_cf_p > 0.0) {
        (var_cf_p,)
    } else {
        (0.0,)
    }
};
        var_cf_i = assign9740_e9632;

        let (assign9750_e9643,) = {
    if (var_cfb_p > 0.0) {
        let (assign9750_e9641,) = {
            if (var_cfb_p < 1.0) {
                (var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9750_e9641,)
    } else {
        (0.0,)
    }
};
        var_cfb_i = assign9750_e9643;

        let (assign9760_e9649,) = {
    if (var_cfd_p > 0.0) {
        (var_cfd_p,)
    } else {
        (0.0,)
    }
};
        var_cfd_i = assign9760_e9649;

        let (assign9770_e9655,) = {
    if (var_psce_p > 0.0) {
        (var_psce_p,)
    } else {
        (0.0,)
    }
};
        var_psce_i = assign9770_e9655;

        let (assign9780_e9666,) = {
    if (var_psceb_p > 0.0) {
        let (assign9780_e9664,) = {
            if (var_psceb_p < 1.0) {
                (var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9780_e9664,)
    } else {
        (0.0,)
    }
};
        var_psceb_i = assign9780_e9666;

        let (assign9790_e9672,) = {
    if (var_psced_p > 0.0) {
        (var_psced_p,)
    } else {
        (0.0,)
    }
};
        var_psced_i = assign9790_e9672;

        let (assign9800_e9678,) = {
    if (var_betn_p > 0.0) {
        (var_betn_p,)
    } else {
        (0.0,)
    }
};
        var_betn_i = assign9800_e9678;

        var_stbet_i = var_stbet_p;

        let (assign9820_e9685,) = {
    if (var_mue_p > 0.0) {
        (var_mue_p,)
    } else {
        (0.0,)
    }
};
        var_mue_i = assign9820_e9685;

        var_stmue_i = var_stmue_p;

        let (assign9840_e9692,) = {
    if (var_themu_p > 0.0) {
        (var_themu_p,)
    } else {
        (0.0,)
    }
};
        var_themu_i = assign9840_e9692;

        var_stthemu_i = var_stthemu_p;

        let (assign9860_e9699,) = {
    if (var_cs_p > 0.0) {
        (var_cs_p,)
    } else {
        (0.0,)
    }
};
        var_cs_i = assign9860_e9699;

        var_stcs_i = var_stcs_p;

        let (assign9880_e9706,) = {
    if (var_thecs_p > 0.0) {
        (var_thecs_p,)
    } else {
        (0.0,)
    }
};
        var_thecs_i = assign9880_e9706;

        var_stthecs_i = var_stthecs_p;

        let (assign9900_e9713,) = {
    if (var_xcor_p > 0.0) {
        (var_xcor_p,)
    } else {
        (0.0,)
    }
};
        var_xcor_i = assign9900_e9713;

        var_stxcor_i = var_stxcor_p;

        var_feta_i = var_feta_p;

        let (assign9930_e9721,) = {
    if (var_rs_p > 0.0) {
        (var_rs_p,)
    } else {
        (0.0,)
    }
};
        var_rs_i = assign9930_e9721;

        var_strs_i = var_strs_p;

        let assign9950_e9725: f64 = (-0.5);
        let (assign9950_e9735,) = {
    if (var_rsb_p > assign9950_e9725) {
        let (assign9950_e9732,) = {
            if (var_rsb_p < 1.0) {
                (var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9950_e9732,)
    } else {
        let assign9950_e9734: f64 = (-0.5);
        (assign9950_e9734,)
    }
};
        var_rsb_i = assign9950_e9735;

        let assign9960_e9738: f64 = (-0.5);
        let (assign9960_e9743,) = {
    if (var_rsg_p > assign9960_e9738) {
        (var_rsg_p,)
    } else {
        let assign9960_e9742: f64 = (-0.5);
        (assign9960_e9742,)
    }
};
        var_rsg_i = assign9960_e9743;

        *var_betn_i_slot = var_betn_i;
        *var_betn_p_slot = var_betn_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cf_i_slot = var_cf_i;
        *var_cf_p_slot = var_cf_p;
        *var_cfb_i_slot = var_cfb_i;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cs_i_slot = var_cs_i;
        *var_ct_i_slot = var_ct_i;
        *var_ctb_i_slot = var_ctb_i;
        *var_ctg_i_slot = var_ctg_i;
        *var_dphib_i_slot = var_dphib_i;
        *var_dvsbnud_i_slot = var_dvsbnud_i;
        *var_epsrox_i_slot = var_epsrox_i;
        *var_feta_i_slot = var_feta_i;
        *var_gfacnud_i_slot = var_gfacnud_i;
        *var_guard145_slot = var_guard145;
        *var_guard146_slot = var_guard146;
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
        *var_sca_i_slot = var_sca_i;
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
        *var_stvfb_i_slot = var_stvfb_i;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_temp0_slot = var_temp0;
        *var_temp00_slot = var_temp00;
        *var_thecs_i_slot = var_thecs_i;
        *var_themu_i_slot = var_themu_i;
        *var_tox_i_slot = var_tox_i;
        *var_toxov_i_slot = var_toxov_i;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_vfb_i_slot = var_vfb_i;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vsbnud_i_slot = var_vsbnud_i;
        *var_xcor_i_slot = var_xcor_i;
    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        var_a1_p: f64,
        var_a2_p: f64,
        var_a3_p: f64,
        var_a4_p: f64,
        var_agidl_p: f64,
        var_agidld_p: f64,
        var_alp1_p: f64,
        var_alp1ac_p: f64,
        var_alp2_p: f64,
        var_alp_p: f64,
        var_alpac_p: f64,
        var_ax_p: f64,
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
        var_cox_p: f64,
        var_ctedge_p: f64,
        var_delvtac_p: f64,
        var_dphibedge_p: f64,
        var_dvfbinr_p: f64,
        var_facneffac_p: f64,
        var_fcgovacc_p: f64,
        var_fcgovaccd_p: f64,
        var_fcinracc_p: f64,
        var_fcinrdep_p: f64,
        var_fnt_p: f64,
        var_fntexc_p: f64,
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
        var_rwell_p: f64,
        var_sta2_p: f64,
        var_stbetedge_p: f64,
        var_stbgidl_p: f64,
        var_stbgidld_p: f64,
        var_stig_p: f64,
        var_stthesat_p: f64,
        var_stvfbedge_p: f64,
        var_thesat_p: f64,
        var_thesatac_p: f64,
        var_thesatb_p: f64,
        var_thesatg_p: f64,
        var_thesatt_p: f64,
        var_toxov_i: f64,
        var_vfbedge_p: f64,
        var_vp_p: f64,
        var_a1_i_slot: &mut f64,
        var_a2_i_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_a4_i_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_alp1_i_slot: &mut f64,
        var_alp1ac_i_slot: &mut f64,
        var_alp2_i_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alpac_i_slot: &mut f64,
        var_ax_i_slot: &mut f64,
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
        var_cox_i_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
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
        var_fnt_i_slot: &mut f64,
        var_fntexc_i_slot: &mut f64,
        var_gc2_i_slot: &mut f64,
        var_gc2ov_i_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc3_i_slot: &mut f64,
        var_gc3ov_i_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_guard147_slot: &mut f64,
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
        var_rwell_i_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_thesat_i_slot: &mut f64,
        var_thesatac_i_slot: &mut f64,
        var_thesatb_i_slot: &mut f64,
        var_thesatg_i_slot: &mut f64,
        var_thesatt_i_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_vfbedge_i_slot: &mut f64,
        var_vp_i_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a2_i: f64 = *var_a2_i_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_a4_i: f64 = *var_a4_i_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp1ac_i: f64 = *var_alp1ac_i_slot;
        let mut var_alp2_i: f64 = *var_alp2_i_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
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
        let mut var_cox_i: f64 = *var_cox_i_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
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
        let mut var_fnt_i: f64 = *var_fnt_i_slot;
        let mut var_fntexc_i: f64 = *var_fntexc_i_slot;
        let mut var_gc2_i: f64 = *var_gc2_i_slot;
        let mut var_gc2ov_i: f64 = *var_gc2ov_i_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc3_i: f64 = *var_gc3_i_slot;
        let mut var_gc3ov_i: f64 = *var_gc3ov_i_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
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
        let mut var_rwell_i: f64 = *var_rwell_i_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_thesat_i: f64 = *var_thesat_i_slot;
        let mut var_thesatac_i: f64 = *var_thesatac_i_slot;
        let mut var_thesatb_i: f64 = *var_thesatb_i_slot;
        let mut var_thesatg_i: f64 = *var_thesatg_i_slot;
        let mut var_thesatt_i: f64 = *var_thesatt_i_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_vfbedge_i: f64 = *var_vfbedge_i_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;

        let (assign9970_e9749,) = {
    if (var_thesat_p > 0.0) {
        (var_thesat_p,)
    } else {
        (0.0,)
    }
};
        var_thesat_i = assign9970_e9749;

        var_stthesat_i = var_stthesat_p;

        let assign9990_e9753: f64 = (-0.5);
        let (assign9990_e9763,) = {
    if (var_thesatb_p > assign9990_e9753) {
        let (assign9990_e9760,) = {
            if (var_thesatb_p < 1.0) {
                (var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9990_e9760,)
    } else {
        let assign9990_e9762: f64 = (-0.5);
        (assign9990_e9762,)
    }
};
        var_thesatb_i = assign9990_e9763;

        let assign10000_e9766: f64 = (-0.5);
        let (assign10000_e9771,) = {
    if (var_thesatg_p > assign10000_e9766) {
        (var_thesatg_p,)
    } else {
        let assign10000_e9770: f64 = (-0.5);
        (assign10000_e9770,)
    }
};
        var_thesatg_i = assign10000_e9771;

        let (assign10010_e9777,) = {
    if (var_thesatt_p > 0.01) {
        (var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        var_thesatt_i = assign10010_e9777;

        let (assign10020_e9783,) = {
    if (var_ax_p > 2.0) {
        (var_ax_p,)
    } else {
        (2.0,)
    }
};
        var_ax_i = assign10020_e9783;

        let (assign10030_e9789,) = {
    if (var_alp_p > 0.0) {
        (var_alp_p,)
    } else {
        (0.0,)
    }
};
        var_alp_i = assign10030_e9789;

        let (assign10040_e9795,) = {
    if (var_alp1_p > 0.0) {
        (var_alp1_p,)
    } else {
        (0.0,)
    }
};
        var_alp1_i = assign10040_e9795;

        let (assign10050_e9801,) = {
    if (var_alp2_p > 0.0) {
        (var_alp2_p,)
    } else {
        (0.0,)
    }
};
        var_alp2_i = assign10050_e9801;

        var_vp_i = var_vp_p;

        let (assign10070_e9808,) = {
    if (var_a1_p > 0.0) {
        (var_a1_p,)
    } else {
        (0.0,)
    }
};
        var_a1_i = assign10070_e9808;

        var_a2_i = var_a2_p;

        var_sta2_i = var_sta2_p;

        let (assign10100_e9816,) = {
    if (var_a3_p > 0.0) {
        (var_a3_p,)
    } else {
        (0.0,)
    }
};
        var_a3_i = assign10100_e9816;

        let (assign10110_e9822,) = {
    if (var_a4_p > 0.0) {
        (var_a4_p,)
    } else {
        (0.0,)
    }
};
        var_a4_i = assign10110_e9822;

        let (assign10120_e9828,) = {
    if (var_imaxii_p > 1e-12) {
        (var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        var_imaxii_i = assign10120_e9828;

        var_gco_i = var_gco_p;

        let (assign10140_e9835,) = {
    if (var_iginv_p > 0.0) {
        (var_iginv_p,)
    } else {
        (0.0,)
    }
};
        var_iginv_i = assign10140_e9835;

        let (assign10150_e9841,) = {
    if (var_igov_p > 0.0) {
        (var_igov_p,)
    } else {
        (0.0,)
    }
};
        var_igov_i = assign10150_e9841;

        let (assign10160_e9847,) = {
    if (var_igovd_p > 0.0) {
        (var_igovd_p,)
    } else {
        (0.0,)
    }
};
        var_igovd_i = assign10160_e9847;

        var_stig_i = var_stig_p;

        var_gc2_i = var_gc2_p;

        var_gc3_i = var_gc3_p;

        var_gc2ov_i = var_gc2ov_p;

        var_gc3ov_i = var_gc3ov_p;

        var_gc2ovd_i = var_gc2ovd_p;

        var_gc3ovd_i = var_gc3ovd_p;

        var_chib_i = var_chib_p;

        let (assign10250_e9861,) = {
    if (var_agidl_p > 0.0) {
        (var_agidl_p,)
    } else {
        (0.0,)
    }
};
        var_agidl_i = assign10250_e9861;

        let (assign10260_e9867,) = {
    if (var_agidld_p > 0.0) {
        (var_agidld_p,)
    } else {
        (0.0,)
    }
};
        var_agidld_i = assign10260_e9867;

        var_bgidl_i = var_bgidl_p;

        var_bgidld_i = var_bgidld_p;

        var_stbgidl_i = var_stbgidl_p;

        var_stbgidld_i = var_stbgidld_p;

        var_cgidl_i = var_cgidl_p;

        var_cgidld_i = var_cgidld_p;

        let (assign10330_e9879,) = {
    if (var_cox_p > 0.0) {
        (var_cox_p,)
    } else {
        (0.0,)
    }
};
        var_cox_i = assign10330_e9879;

        var_delvtac_i = var_delvtac_p;

        let (assign10350_e9886,) = {
    if (var_facneffac_p > 0.0) {
        (var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        var_facneffac_i = assign10350_e9886;

        let (assign10360_e9892,) = {
    if (var_thesatac_p > 0.0) {
        (var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        var_thesatac_i = assign10360_e9892;

        let (assign10370_e9898,) = {
    if (var_axac_p > 2.0) {
        (var_axac_p,)
    } else {
        (2.0,)
    }
};
        var_axac_i = assign10370_e9898;

        var_alpac_i = var_alpac_p;

        let (assign10390_e9905,) = {
    if (var_alp1ac_p > 0.0) {
        (var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        var_alp1ac_i = assign10390_e9905;

        let (assign10400_e9911,) = {
    if (var_cgov_p > 0.0) {
        (var_cgov_p,)
    } else {
        (0.0,)
    }
};
        var_cgov_i = assign10400_e9911;

        let (assign10410_e9917,) = {
    if (var_cgovd_p > 0.0) {
        (var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        var_cgovd_i = assign10410_e9917;

        var_fcgovacc_i = var_fcgovacc_p;

        var_fcgovaccd_i = var_fcgovaccd_p;

        var_cgovaccg_i = var_cgovaccg_p;

        let (assign10450_e9926,) = {
    if (var_cgbov_p > 0.0) {
        (var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        var_cgbov_i = assign10450_e9926;

        let (assign10460_e9932,) = {
    if (var_cinr_p > 0.0) {
        (var_cinr_p,)
    } else {
        (0.0,)
    }
};
        var_cinr_i = assign10460_e9932;

        let (assign10470_e9938,) = {
    if (var_cinrd_p > 0.0) {
        (var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        var_cinrd_i = assign10470_e9938;

        var_dvfbinr_i = var_dvfbinr_p;

        var_fcinrdep_i = var_fcinrdep_p;

        var_fcinracc_i = var_fcinracc_p;

        var_axinr_i = var_axinr_p;

        let (assign10520_e9948,) = {
    if (var_cfr_p > 0.0) {
        (var_cfr_p,)
    } else {
        (0.0,)
    }
};
        var_cfr_i = assign10520_e9948;

        let (assign10530_e9954,) = {
    if (var_cfrd_p > 0.0) {
        (var_cfrd_p,)
    } else {
        (0.0,)
    }
};
        var_cfrd_i = assign10530_e9954;

        var_fnt_i = var_fnt_p;

        let (assign10550_e9961,) = {
    if (var_fntexc_p > 0.0) {
        (var_fntexc_p,)
    } else {
        (0.0,)
    }
};
        var_fntexc_i = assign10550_e9961;

        var_vfbedge_i = var_vfbedge_p;

        var_stvfbedge_i = var_stvfbedge_p;

        var_dphibedge_i = var_dphibedge_p;

        let (assign10630_e9994,) = {
    if (var_neffedge_p > 1e20) {
        let (assign10630_e9992,) = {
            if (var_neffedge_p < 1e26) {
                (var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10630_e9992,)
    } else {
        (1e20,)
    }
};
        var_neffedge_i = assign10630_e9994;

        let (assign10640_e10000,) = {
    if (var_ctedge_p > 0.0) {
        (var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        var_ctedge_i = assign10640_e10000;

        let (assign10650_e10006,) = {
    if (var_betnedge_p > 0.0) {
        (var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        var_betnedge_i = assign10650_e10006;

        var_stbetedge_i = var_stbetedge_p;

        let (assign10670_e10013,) = {
    if (var_psceedge_p > 0.0) {
        (var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        var_psceedge_i = assign10670_e10013;

        let (assign10680_e10024,) = {
    if (var_pscebedge_p > 0.0) {
        let (assign10680_e10022,) = {
            if (var_pscebedge_p < 1.0) {
                (var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10680_e10022,)
    } else {
        (0.0,)
    }
};
        var_pscebedge_i = assign10680_e10024;

        let (assign10690_e10030,) = {
    if (var_pscededge_p > 0.0) {
        (var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        var_pscededge_i = assign10690_e10030;

        let (assign10700_e10036,) = {
    if (var_cfedge_p > 0.0) {
        (var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfedge_i = assign10700_e10036;

        let (assign10710_e10047,) = {
    if (var_cfbedge_p > 0.0) {
        let (assign10710_e10045,) = {
            if (var_cfbedge_p < 1.0) {
                (var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10710_e10045,)
    } else {
        (0.0,)
    }
};
        var_cfbedge_i = assign10710_e10047;

        let (assign10720_e10053,) = {
    if (var_cfdedge_p > 0.0) {
        (var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfdedge_i = assign10720_e10053;

        let (assign10780_e10079,) = {
    if (var_rg_p > 0.0) {
        (var_rg_p,)
    } else {
        (0.0,)
    }
};
        var_rg_i = assign10780_e10079;

        var_rse_i = var_rse_p;

        var_rde_i = var_rde_p;

        var_rbulk_i = var_rbulk_p;

        var_rjuns_i = var_rjuns_p;

        var_rjund_i = var_rjund_p;

        var_rwell_i = var_rwell_p;

        let assign10850_e10088: f64 = (p.p31 * var_nf_i);
        let (assign10850_e10095,) = {
    if (assign10850_e10088 > 0.0) {
        let assign10850_e10093: f64 = (p.p31 * var_nf_i);
        (assign10850_e10093,)
    } else {
        (0.0,)
    }
};
        var_mult_inst = assign10850_e10095;

        var_factuo_i = p.p16;

        var_delvto_i = p.p15;

        var_factuoedge_i = p.p18;

        var_delvtoedge_i = p.p17;

        let assign10900_e10102: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard147 = assign10900_e10102;

        let (assign10910_e10106,) = {
    if (var_guard147 != 0.0) {
        (var_toxov_i,)
    } else {
        (var_toxovd_i,)
    }
};
        var_toxovd_i = assign10910_e10106;

        let (assign10920_e10110,) = {
    if (var_guard147 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign10920_e10110;

        *var_a1_i_slot = var_a1_i;
        *var_a2_i_slot = var_a2_i;
        *var_a3_i_slot = var_a3_i;
        *var_a4_i_slot = var_a4_i;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidld_i_slot = var_agidld_i;
        *var_alp1_i_slot = var_alp1_i;
        *var_alp1ac_i_slot = var_alp1ac_i;
        *var_alp2_i_slot = var_alp2_i;
        *var_alp_i_slot = var_alp_i;
        *var_alpac_i_slot = var_alpac_i;
        *var_ax_i_slot = var_ax_i;
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
        *var_cox_i_slot = var_cox_i;
        *var_ctedge_i_slot = var_ctedge_i;
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
        *var_fnt_i_slot = var_fnt_i;
        *var_fntexc_i_slot = var_fntexc_i;
        *var_gc2_i_slot = var_gc2_i;
        *var_gc2ov_i_slot = var_gc2ov_i;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc3_i_slot = var_gc3_i;
        *var_gc3ov_i_slot = var_gc3ov_i;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gco_i_slot = var_gco_i;
        *var_guard147_slot = var_guard147;
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
        *var_rwell_i_slot = var_rwell_i;
        *var_sta2_i_slot = var_sta2_i;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stig_i_slot = var_stig_i;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_thesat_i_slot = var_thesat_i;
        *var_thesatac_i_slot = var_thesatac_i;
        *var_thesatb_i_slot = var_thesatb_i;
        *var_thesatg_i_slot = var_thesatg_i;
        *var_thesatt_i_slot = var_thesatt_i;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_vfbedge_i_slot = var_vfbedge_i;
        *var_vp_i_slot = var_vp_i;
    }
}

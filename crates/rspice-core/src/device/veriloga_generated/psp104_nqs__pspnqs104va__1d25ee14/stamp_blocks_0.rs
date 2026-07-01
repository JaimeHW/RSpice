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
        var_guard10_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard9_slot: &mut f64,
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
        var_phigrbot_slot: &mut f64,
        var_phigrgat_slot: &mut f64,
        var_phigrgat2nd_slot: &mut f64,
        var_phigrsti_slot: &mut f64,
        var_phitr_slot: &mut f64,
        var_phitrinv_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_slopebot_slot: &mut f64,
        var_slopegat_slot: &mut f64,
        var_slopegat_dn5_slot: &mut f64,
        var_slopegat_dn6_slot: &mut f64,
        var_slopegat_dn7_slot: &mut f64,
        var_slopegat_dn8_slot: &mut f64,
        var_slopesti_slot: &mut f64,
        var_swgat2nd_slot: &mut f64,
        var_swjunexp_i_slot: &mut f64,
        var_swnqs_i_slot: &mut f64,
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
        var_vbrinvgat_dn5_slot: &mut f64,
        var_vbrinvgat_dn6_slot: &mut f64,
        var_vbrinvgat_dn7_slot: &mut f64,
        var_vbrinvgat_dn8_slot: &mut f64,
        var_vbrinvsti_slot: &mut f64,
        var_vnorm_slot: &mut f64,
        var_vnorm_inv_slot: &mut f64,
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
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
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
        let mut var_phigrbot: f64 = *var_phigrbot_slot;
        let mut var_phigrgat: f64 = *var_phigrgat_slot;
        let mut var_phigrgat2nd: f64 = *var_phigrgat2nd_slot;
        let mut var_phigrsti: f64 = *var_phigrsti_slot;
        let mut var_phitr: f64 = *var_phitr_slot;
        let mut var_phitrinv: f64 = *var_phitrinv_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_slopebot: f64 = *var_slopebot_slot;
        let mut var_slopegat: f64 = *var_slopegat_slot;
        let mut var_slopegat_dn5: f64 = *var_slopegat_dn5_slot;
        let mut var_slopegat_dn6: f64 = *var_slopegat_dn6_slot;
        let mut var_slopegat_dn7: f64 = *var_slopegat_dn7_slot;
        let mut var_slopegat_dn8: f64 = *var_slopegat_dn8_slot;
        let mut var_slopesti: f64 = *var_slopesti_slot;
        let mut var_swgat2nd: f64 = *var_swgat2nd_slot;
        let mut var_swjunexp_i: f64 = *var_swjunexp_i_slot;
        let mut var_swnqs_i: f64 = *var_swnqs_i_slot;
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
        let mut var_vbrinvgat_dn5: f64 = *var_vbrinvgat_dn5_slot;
        let mut var_vbrinvgat_dn6: f64 = *var_vbrinvgat_dn6_slot;
        let mut var_vbrinvgat_dn7: f64 = *var_vbrinvgat_dn7_slot;
        let mut var_vbrinvgat_dn8: f64 = *var_vbrinvgat_dn8_slot;
        let mut var_vbrinvsti: f64 = *var_vbrinvsti_slot;
        let mut var_vnorm: f64 = *var_vnorm_slot;
        let mut var_vnorm_inv: f64 = *var_vnorm_inv_slot;
        let mut var_wdepnulrbot: f64 = *var_wdepnulrbot_slot;
        let mut var_wdepnulrgat: f64 = *var_wdepnulrgat_slot;
        let mut var_wdepnulrinvbot: f64 = *var_wdepnulrinvbot_slot;
        let mut var_wdepnulrinvgat: f64 = *var_wdepnulrinvgat_slot;
        let mut var_wdepnulrinvsti: f64 = *var_wdepnulrinvsti_slot;
        let mut var_wdepnulrsti: f64 = *var_wdepnulrsti_slot;

        let assign00_e1569: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1 = assign00_e1569;

        let (assign10_e1574,) = {
    if (var_guard1 != 0.0) {
        let assign10_e1572: f64 = 1.0;
        (assign10_e1572,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign10_e1574;

        let (assign20_e1580,) = {
    if (var_guard1 == 0.0) {
        let assign20_e1578: f64 = (-1.0);
        (assign20_e1578,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign20_e1580;

        let assign30_e1583: f64 = (8.8541878176e-12 * 11.8);
        var_epssi = assign30_e1583;

        let assign40_e1586: f64 = if p.p51 < 0.5 { 1.0 } else { 0.0 };
        var_guard2 = assign40_e1586;

        let (assign50_e1590,) = {
    if (var_guard2 != 0.0) {
        (0.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign50_e1590;

        let assign60_e1593: f64 = if p.p51 < 1.5 { 1.0 } else { 0.0 };
        var_guard3 = assign60_e1593;

        let (assign70_e1600,) = {
    if ((var_guard2 == 0.0) && (var_guard3 != 0.0)) {
        (1.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign70_e1600;

        let assign80_e1603: f64 = if p.p51 < 2.5 { 1.0 } else { 0.0 };
        var_guard4 = assign80_e1603;

        let (assign90_e1613,) = {
    if (((var_guard2 == 0.0) && (var_guard3 == 0.0)) && (var_guard4 != 0.0)) {
        (2.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign90_e1613;

        let assign100_e1616: f64 = if p.p51 < 4.0 { 1.0 } else { 0.0 };
        var_guard5 = assign100_e1616;

        let (assign110_e1629,) = {
    if ((((var_guard2 == 0.0) && (var_guard3 == 0.0)) && (var_guard4 == 0.0)) && (var_guard5 != 0.0)) {
        (3.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign110_e1629;

        let assign120_e1632: f64 = if p.p51 < 7.0 { 1.0 } else { 0.0 };
        var_guard6 = assign120_e1632;

        let (assign130_e1648,) = {
    if (((((var_guard2 == 0.0) && (var_guard3 == 0.0)) && (var_guard4 == 0.0)) && (var_guard5 == 0.0)) && (var_guard6 != 0.0)) {
        (5.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign130_e1648;

        let (assign140_e1665,) = {
    if (((((var_guard2 == 0.0) && (var_guard3 == 0.0)) && (var_guard4 == 0.0)) && (var_guard5 == 0.0)) && (var_guard6 == 0.0)) {
        (9.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign140_e1665;

        var_vnorm = 10.0;

        let assign170_e1670: f64 = (1.0 / var_vnorm);
        var_vnorm_inv = assign170_e1670;

        let assign180_e1673: f64 = (273.15 + p.p38);
        var_tkr = assign180_e1673;

        var_swjunexp_i = 0.0;

        let assign200_e1677: f64 = if p.p927 > 0.5 { 1.0 } else { 0.0 };
        var_guard7 = assign200_e1677;

        let (assign210_e1681,) = {
    if (var_guard7 != 0.0) {
        (1.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign210_e1681;

        let (assign220_e1686,) = {
    if (var_guard7 == 0.0) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign220_e1686;

        let assign230_e1689: f64 = (273.15 + p.p823);
        var_tkr_1 = assign230_e1689;

        let assign240_e1692: f64 = (1.3806505e-23 / 1.6021918e-19);
        var_kbol_over_qele = assign240_e1692;

        let assign250_e1695: f64 = (var_kbol_over_qele * var_tkr_1);
        var_phitr = assign250_e1695;

        let assign260_e1698: f64 = (1.0 / var_phitr);
        var_phitrinv = assign260_e1698;

        let assign270_e1701: f64 = (0.000702 * var_tkr_1);
        let assign270_e1703: f64 = (assign270_e1701 * var_tkr_1);
        let assign270_e1704: f64 = (-assign270_e1703);
        let assign270_e1707: f64 = (1108.0 + var_tkr_1);
        let assign270_e1708: f64 = (assign270_e1704 / assign270_e1707);
        var_deltaphigr = assign270_e1708;

        let assign280_e1711: f64 = (p.p834 + var_deltaphigr);
        var_phigrbot = assign280_e1711;

        let assign290_e1714: f64 = (p.p835 + var_deltaphigr);
        var_phigrsti = assign290_e1714;

        let assign300_e1717: f64 = (p.p836 + var_deltaphigr);
        var_phigrgat = assign300_e1717;

        let assign310_e1720: f64 = (1.0 - p.p831);
        var_one_minus_pbot = assign310_e1720;

        let assign320_e1723: f64 = (1.0 - p.p832);
        var_one_minus_psti = assign320_e1723;

        let assign330_e1726: f64 = (1.0 - p.p833);
        var_one_minus_pgat = assign330_e1726;

        let assign340_e1729: f64 = (1.0 / var_one_minus_pbot);
        var_one_over_one_minus_pbot = assign340_e1729;

        let assign350_e1732: f64 = (1.0 / var_one_minus_psti);
        var_one_over_one_minus_psti = assign350_e1732;

        let assign360_e1735: f64 = (1.0 / var_one_minus_pgat);
        var_one_over_one_minus_pgat = assign360_e1735;

        let assign370_e1738: f64 = (var_epssi / p.p825);
        var_wdepnulrbot = assign370_e1738;

        let assign380_e1741: f64 = (p.p843 * var_epssi);
        let assign380_e1743: f64 = (assign380_e1741 / p.p826);
        var_wdepnulrsti = assign380_e1743;

        let assign390_e1746: f64 = (p.p844 * var_epssi);
        let assign390_e1748: f64 = (assign390_e1746 / p.p827);
        var_wdepnulrgat = assign390_e1748;

        let assign400_e1751: f64 = (1.0 / var_wdepnulrbot);
        var_wdepnulrinvbot = assign400_e1751;

        let assign410_e1754: f64 = (1.0 / var_wdepnulrsti);
        var_wdepnulrinvsti = assign410_e1754;

        let assign420_e1757: f64 = (1.0 / var_wdepnulrgat);
        var_wdepnulrinvgat = assign420_e1757;

        let assign430_e1760: f64 = (1.0 / p.p828);
        var_vbirbotinv = assign430_e1760;

        let assign440_e1763: f64 = (1.0 / p.p829);
        var_vbirstiinv = assign440_e1763;

        let assign450_e1766: f64 = (1.0 / p.p830);
        var_vbirgatinv = assign450_e1766;

        let assign460_e1769: f64 = (1.772453850905516 * 0.29214664);
        var_perfc = assign460_e1769;

        let assign470_e1771: f64 = (-5.0);
        let assign470_e1773: f64 = (assign470_e1771 * 0.29214664);
        let assign470_e1775: f64 = (assign470_e1773 + 6.0);
        let assign470_e1778: f64 = (-2.0);
        let assign470_e1779: f64 = (var_perfc).powf(assign470_e1778);
        let assign470_e1780: f64 = (assign470_e1775 - assign470_e1779);
        let assign470_e1782: f64 = (assign470_e1780 / 3.0);
        var_berfc = assign470_e1782;

        let assign480_e1785: f64 = (1.0 - 0.29214664);
        let assign480_e1787: f64 = (assign480_e1785 - var_berfc);
        var_cerfc = assign480_e1787;

        let assign490_e1791: f64 = (1.0 / p.p824);
        let assign490_e1792: f64 = (1.0 - assign490_e1791);
        var_alphaav = assign490_e1792;

        let assign500_e1797: f64 = (var_alphaav).powf(p.p863);
        let assign500_e1798: f64 = (1.0 - assign500_e1797);
        let assign500_e1799: f64 = (1.0 / assign500_e1798);
        var_fstopbot = assign500_e1799;

        let assign510_e1804: f64 = (var_alphaav).powf(p.p864);
        let assign510_e1805: f64 = (1.0 - assign510_e1804);
        let assign510_e1806: f64 = (1.0 / assign510_e1805);
        var_fstopsti = assign510_e1806;

        let assign520_e1811: f64 = (var_alphaav).powf(p.p865);
        let assign520_e1812: f64 = (1.0 - assign520_e1811);
        let assign520_e1813: f64 = (1.0 / assign520_e1812);
        var_fstopgat = assign520_e1813;

        let assign530_e1816: f64 = (1.0 / p.p860);
        var_vbrinvbot = assign530_e1816;

        let assign540_e1819: f64 = (1.0 / p.p861);
        var_vbrinvsti = assign540_e1819;

        let assign550_e1822: f64 = (1.0 / p.p862);
        var_vbrinvgat = assign550_e1822;
        var_vbrinvgat_dn5 = 0.0;
        var_vbrinvgat_dn6 = 0.0;
        var_vbrinvgat_dn7 = 0.0;
        var_vbrinvgat_dn8 = 0.0;

        let assign560_e1825: f64 = (var_fstopbot * var_fstopbot);
        let assign560_e1829: f64 = (p.p863 - 1.0);
        let assign560_e1830: f64 = (var_alphaav).powf(assign560_e1829);
        let assign560_e1831: f64 = (assign560_e1825 * assign560_e1830);
        let assign560_e1832: f64 = (-assign560_e1831);
        let assign560_e1834: f64 = (assign560_e1832 * p.p863);
        let assign560_e1836: f64 = (assign560_e1834 * var_vbrinvbot);
        var_slopebot = assign560_e1836;

        let assign570_e1839: f64 = (var_fstopsti * var_fstopsti);
        let assign570_e1843: f64 = (p.p864 - 1.0);
        let assign570_e1844: f64 = (var_alphaav).powf(assign570_e1843);
        let assign570_e1845: f64 = (assign570_e1839 * assign570_e1844);
        let assign570_e1846: f64 = (-assign570_e1845);
        let assign570_e1848: f64 = (assign570_e1846 * p.p864);
        let assign570_e1850: f64 = (assign570_e1848 * var_vbrinvsti);
        var_slopesti = assign570_e1850;

        let assign580_e1853: f64 = (var_fstopgat * var_fstopgat);
        let assign580_e1857: f64 = (p.p865 - 1.0);
        let assign580_e1858: f64 = (var_alphaav).powf(assign580_e1857);
        let assign580_e1859: f64 = (assign580_e1853 * assign580_e1858);
        let assign580_e1860: f64 = (-assign580_e1859);
        let assign580_e1862: f64 = (assign580_e1860 * p.p865);
        let assign580_e1864: f64 = (assign580_e1862 * var_vbrinvgat);
        var_slopegat = assign580_e1864;
        var_slopegat_dn5 = (assign580_e1862 * var_vbrinvgat_dn5);
        var_slopegat_dn6 = (assign580_e1862 * var_vbrinvgat_dn6);
        var_slopegat_dn7 = (assign580_e1862 * var_vbrinvgat_dn7);
        var_slopegat_dn8 = (assign580_e1862 * var_vbrinvgat_dn8);

        let assign590_e1879: f64 = if ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0)) { 1.0 } else { 0.0 };
        var_guard8 = assign590_e1879;

        let (assign600_e1883,) = {
    if (var_guard8 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign600_e1883;

        let (assign610_e1888,) = {
    if (var_guard8 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign610_e1888;

        let assign620_e1891: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard9 = assign620_e1891;

        let (assign630_e1904,) = {
    if (var_guard9 != 0.0) {
        let assign630_e1895: f64 = (p.p827 * p.p866);
        let (assign630_e1902,) = {
            if (assign630_e1895 > 1e-18) {
                let assign630_e1900: f64 = (p.p827 * p.p866);
                (assign630_e1900,)
            } else {
                (1e-18,)
            }
        };
        (assign630_e1902,)
    } else {
        (var_cjorgat2nd,)
    }
};
        var_cjorgat2nd = assign630_e1904;

        let (assign640_e1917,) = {
    if (var_guard9 != 0.0) {
        let assign640_e1908: f64 = (p.p830 * p.p867);
        let (assign640_e1915,) = {
            if (assign640_e1908 > 0.05) {
                let assign640_e1913: f64 = (p.p830 * p.p867);
                (assign640_e1913,)
            } else {
                (0.05,)
            }
        };
        (assign640_e1915,)
    } else {
        (var_vbirgat2nd,)
    }
};
        var_vbirgat2nd = assign640_e1917;

        let (assign650_e1944,) = {
    if (var_guard9 != 0.0) {
        let assign650_e1921: f64 = (p.p833 * p.p868);
        let (assign650_e1928,) = {
            if (assign650_e1921 > 0.05) {
                let assign650_e1926: f64 = (p.p833 * p.p868);
                (assign650_e1926,)
            } else {
                (0.05,)
            }
        };
        let (assign650_e1942,) = {
            if (assign650_e1928 < 0.95) {
                let assign650_e1933: f64 = (p.p833 * p.p868);
                let (assign650_e1940,) = {
                    if (assign650_e1933 > 0.05) {
                        let assign650_e1938: f64 = (p.p833 * p.p868);
                        (assign650_e1938,)
                    } else {
                        (0.05,)
                    }
                };
                (assign650_e1940,)
            } else {
                (0.95,)
            }
        };
        (assign650_e1942,)
    } else {
        (var_pgat2nd,)
    }
};
        var_pgat2nd = assign650_e1944;

        let (assign660_e1950,) = {
    if (var_guard9 != 0.0) {
        let assign660_e1948: f64 = (p.p836 * p.p869);
        (assign660_e1948,)
    } else {
        (var_phiggat2nd,)
    }
};
        var_phiggat2nd = assign660_e1950;

        let (assign670_e1956,) = {
    if (var_guard9 != 0.0) {
        let assign670_e1954: f64 = (var_phiggat2nd + var_deltaphigr);
        (assign670_e1954,)
    } else {
        (var_phigrgat2nd,)
    }
};
        var_phigrgat2nd = assign670_e1956;

        let (assign680_e1962,) = {
    if (var_guard9 != 0.0) {
        let assign680_e1960: f64 = (1.0 - var_pgat2nd);
        (assign680_e1960,)
    } else {
        (var_one_minus_pgat2nd,)
    }
};
        var_one_minus_pgat2nd = assign680_e1962;

        let (assign690_e1968,) = {
    if (var_guard9 != 0.0) {
        let assign690_e1966: f64 = (1.0 / var_one_minus_pgat2nd);
        (assign690_e1966,)
    } else {
        (var_one_over_one_minus_pgat2nd,)
    }
};
        var_one_over_one_minus_pgat2nd = assign690_e1968;

        let assign700_e1971: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard10 = assign700_e1971;

        let (assign710_e1975,) = {
    if (var_guard10 != 0.0) {
        (p.p825,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign710_e1975;

        let (assign720_e1979,) = {
    if (var_guard10 != 0.0) {
        (p.p826,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign720_e1979;

        let (assign730_e1983,) = {
    if (var_guard10 != 0.0) {
        (p.p827,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign730_e1983;

        let (assign740_e1987,) = {
    if (var_guard10 != 0.0) {
        (p.p828,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign740_e1987;

        let (assign750_e1991,) = {
    if (var_guard10 != 0.0) {
        (p.p829,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign750_e1991;

        let (assign760_e1995,) = {
    if (var_guard10 != 0.0) {
        (p.p830,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign760_e1995;

        let (assign770_e1999,) = {
    if (var_guard10 != 0.0) {
        (p.p831,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign770_e1999;

        let (assign780_e2003,) = {
    if (var_guard10 != 0.0) {
        (p.p832,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign780_e2003;

        let (assign790_e2007,) = {
    if (var_guard10 != 0.0) {
        (p.p833,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign790_e2007;

        let (assign800_e2011,) = {
    if (var_guard10 != 0.0) {
        (p.p834,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign800_e2011;

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
        *var_guard10_slot = var_guard10;
        *var_guard2_slot = var_guard2;
        *var_guard3_slot = var_guard3;
        *var_guard4_slot = var_guard4;
        *var_guard5_slot = var_guard5;
        *var_guard6_slot = var_guard6;
        *var_guard7_slot = var_guard7;
        *var_guard8_slot = var_guard8;
        *var_guard9_slot = var_guard9;
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
        *var_phigrbot_slot = var_phigrbot;
        *var_phigrgat_slot = var_phigrgat;
        *var_phigrgat2nd_slot = var_phigrgat2nd;
        *var_phigrsti_slot = var_phigrsti;
        *var_phitr_slot = var_phitr;
        *var_phitrinv_slot = var_phitrinv;
        *var_pstid_i_slot = var_pstid_i;
        *var_slopebot_slot = var_slopebot;
        *var_slopegat_slot = var_slopegat;
        *var_slopegat_dn5_slot = var_slopegat_dn5;
        *var_slopegat_dn6_slot = var_slopegat_dn6;
        *var_slopegat_dn7_slot = var_slopegat_dn7;
        *var_slopegat_dn8_slot = var_slopegat_dn8;
        *var_slopesti_slot = var_slopesti;
        *var_swgat2nd_slot = var_swgat2nd;
        *var_swjunexp_i_slot = var_swjunexp_i;
        *var_swnqs_i_slot = var_swnqs_i;
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
        *var_vbrinvgat_dn5_slot = var_vbrinvgat_dn5;
        *var_vbrinvgat_dn6_slot = var_vbrinvgat_dn6;
        *var_vbrinvgat_dn7_slot = var_vbrinvgat_dn7;
        *var_vbrinvgat_dn8_slot = var_vbrinvgat_dn8;
        *var_vbrinvsti_slot = var_vbrinvsti;
        *var_vnorm_slot = var_vnorm;
        *var_vnorm_inv_slot = var_vnorm_inv;
        *var_wdepnulrbot_slot = var_wdepnulrbot;
        *var_wdepnulrgat_slot = var_wdepnulrgat;
        *var_wdepnulrinvbot_slot = var_wdepnulrinvbot;
        *var_wdepnulrinvgat_slot = var_wdepnulrinvgat;
        *var_wdepnulrinvsti_slot = var_wdepnulrinvsti;
        *var_wdepnulrsti_slot = var_wdepnulrsti;
    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        var_guard10: f64,
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

        let (assign810_e2015,) = {
    if (var_guard10 != 0.0) {
        (p.p835,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign810_e2015;

        let (assign820_e2019,) = {
    if (var_guard10 != 0.0) {
        (p.p836,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign820_e2019;

        let (assign830_e2023,) = {
    if (var_guard10 != 0.0) {
        (p.p837,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign830_e2023;

        let (assign840_e2027,) = {
    if (var_guard10 != 0.0) {
        (p.p838,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign840_e2027;

        let (assign850_e2031,) = {
    if (var_guard10 != 0.0) {
        (p.p839,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign850_e2031;

        let (assign860_e2035,) = {
    if (var_guard10 != 0.0) {
        (p.p840,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign860_e2035;

        let (assign870_e2039,) = {
    if (var_guard10 != 0.0) {
        (p.p841,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign870_e2039;

        let (assign880_e2043,) = {
    if (var_guard10 != 0.0) {
        (p.p842,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign880_e2043;

        let (assign890_e2047,) = {
    if (var_guard10 != 0.0) {
        (p.p843,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign890_e2047;

        let (assign900_e2051,) = {
    if (var_guard10 != 0.0) {
        (p.p844,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign900_e2051;

        let (assign910_e2055,) = {
    if (var_guard10 != 0.0) {
        (p.p845,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign910_e2055;

        let (assign920_e2059,) = {
    if (var_guard10 != 0.0) {
        (p.p846,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign920_e2059;

        let (assign930_e2063,) = {
    if (var_guard10 != 0.0) {
        (p.p847,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign930_e2063;

        let (assign940_e2067,) = {
    if (var_guard10 != 0.0) {
        (p.p848,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign940_e2067;

        let (assign950_e2071,) = {
    if (var_guard10 != 0.0) {
        (p.p849,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign950_e2071;

        let (assign960_e2075,) = {
    if (var_guard10 != 0.0) {
        (p.p850,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign960_e2075;

        let (assign970_e2079,) = {
    if (var_guard10 != 0.0) {
        (p.p851,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign970_e2079;

        let (assign980_e2083,) = {
    if (var_guard10 != 0.0) {
        (p.p852,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign980_e2083;

        let (assign990_e2087,) = {
    if (var_guard10 != 0.0) {
        (p.p853,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign990_e2087;

        let (assign1000_e2091,) = {
    if (var_guard10 != 0.0) {
        (p.p854,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1000_e2091;

        let (assign1010_e2095,) = {
    if (var_guard10 != 0.0) {
        (p.p855,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1010_e2095;

        let (assign1020_e2099,) = {
    if (var_guard10 != 0.0) {
        (p.p856,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1020_e2099;

        let (assign1030_e2103,) = {
    if (var_guard10 != 0.0) {
        (p.p857,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1030_e2103;

        let (assign1040_e2107,) = {
    if (var_guard10 != 0.0) {
        (p.p858,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1040_e2107;

        let (assign1050_e2111,) = {
    if (var_guard10 != 0.0) {
        (p.p859,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1050_e2111;

        let (assign1060_e2115,) = {
    if (var_guard10 != 0.0) {
        (p.p860,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1060_e2115;

        let (assign1070_e2119,) = {
    if (var_guard10 != 0.0) {
        (p.p861,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1070_e2119;

        let (assign1080_e2123,) = {
    if (var_guard10 != 0.0) {
        (p.p862,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1080_e2123;

        let (assign1090_e2127,) = {
    if (var_guard10 != 0.0) {
        (p.p863,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1090_e2127;

        let (assign1100_e2131,) = {
    if (var_guard10 != 0.0) {
        (p.p864,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1100_e2131;

        let (assign1110_e2135,) = {
    if (var_guard10 != 0.0) {
        (p.p865,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1110_e2135;

        let (assign1120_e2139,) = {
    if (var_guard10 != 0.0) {
        (p.p928,)
    } else {
        (var_vjunrefd_i,)
    }
};
        var_vjunrefd_i = assign1120_e2139;

        let (assign1130_e2143,) = {
    if (var_guard10 != 0.0) {
        (p.p929,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1130_e2143;

        let (assign1140_e2147,) = {
    if (var_guard10 != 0.0) {
        (p.p872,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1140_e2147;

        let (assign1150_e2151,) = {
    if (var_guard10 != 0.0) {
        (p.p873,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1150_e2151;

        let (assign1160_e2155,) = {
    if (var_guard10 != 0.0) {
        (p.p874,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1160_e2155;

        let (assign1170_e2159,) = {
    if (var_guard10 != 0.0) {
        (p.p875,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1170_e2159;

        let (assign1180_e2163,) = {
    if (var_guard10 != 0.0) {
        (p.p866,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1180_e2163;

        let (assign1190_e2167,) = {
    if (var_guard10 != 0.0) {
        (p.p867,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1190_e2167;

        let (assign1200_e2171,) = {
    if (var_guard10 != 0.0) {
        (p.p868,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1200_e2171;

        let (assign1210_e2175,) = {
    if (var_guard10 != 0.0) {
        (p.p869,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1210_e2175;

        let (assign1220_e2179,) = {
    if (var_guard10 != 0.0) {
        (p.p870,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1220_e2179;

        let (assign1230_e2183,) = {
    if (var_guard10 != 0.0) {
        (p.p871,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1230_e2183;

        let (assign1240_e2188,) = {
    if (var_guard10 == 0.0) {
        (p.p876,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign1240_e2188;

        let (assign1250_e2193,) = {
    if (var_guard10 == 0.0) {
        (p.p877,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign1250_e2193;

        let (assign1260_e2198,) = {
    if (var_guard10 == 0.0) {
        (p.p878,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign1260_e2198;

        let (assign1270_e2203,) = {
    if (var_guard10 == 0.0) {
        (p.p879,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign1270_e2203;

        let (assign1280_e2208,) = {
    if (var_guard10 == 0.0) {
        (p.p880,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign1280_e2208;

        let (assign1290_e2213,) = {
    if (var_guard10 == 0.0) {
        (p.p881,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign1290_e2213;

        let (assign1300_e2218,) = {
    if (var_guard10 == 0.0) {
        (p.p882,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign1300_e2218;

        let (assign1310_e2223,) = {
    if (var_guard10 == 0.0) {
        (p.p883,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign1310_e2223;

        let (assign1320_e2228,) = {
    if (var_guard10 == 0.0) {
        (p.p884,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign1320_e2228;

        let (assign1330_e2233,) = {
    if (var_guard10 == 0.0) {
        (p.p885,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign1330_e2233;

        let (assign1340_e2238,) = {
    if (var_guard10 == 0.0) {
        (p.p886,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign1340_e2238;

        let (assign1350_e2243,) = {
    if (var_guard10 == 0.0) {
        (p.p887,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign1350_e2243;

        let (assign1360_e2248,) = {
    if (var_guard10 == 0.0) {
        (p.p888,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign1360_e2248;

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
        var_guard10: f64,
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
        var_fstopbot_d_slot: &mut f64,
        var_fstopgat_d_slot: &mut f64,
        var_fstopsti_d_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
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
        var_slopegat_d_dn5_slot: &mut f64,
        var_slopegat_d_dn6_slot: &mut f64,
        var_slopegat_d_dn7_slot: &mut f64,
        var_slopegat_d_dn8_slot: &mut f64,
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
        var_vbrinvgat_d_dn5_slot: &mut f64,
        var_vbrinvgat_d_dn6_slot: &mut f64,
        var_vbrinvgat_d_dn7_slot: &mut f64,
        var_vbrinvgat_d_dn8_slot: &mut f64,
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
        let mut var_fstopbot_d: f64 = *var_fstopbot_d_slot;
        let mut var_fstopgat_d: f64 = *var_fstopgat_d_slot;
        let mut var_fstopsti_d: f64 = *var_fstopsti_d_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
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
        let mut var_slopegat_d_dn5: f64 = *var_slopegat_d_dn5_slot;
        let mut var_slopegat_d_dn6: f64 = *var_slopegat_d_dn6_slot;
        let mut var_slopegat_d_dn7: f64 = *var_slopegat_d_dn7_slot;
        let mut var_slopegat_d_dn8: f64 = *var_slopegat_d_dn8_slot;
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
        let mut var_vbrinvgat_d_dn5: f64 = *var_vbrinvgat_d_dn5_slot;
        let mut var_vbrinvgat_d_dn6: f64 = *var_vbrinvgat_d_dn6_slot;
        let mut var_vbrinvgat_d_dn7: f64 = *var_vbrinvgat_d_dn7_slot;
        let mut var_vbrinvgat_d_dn8: f64 = *var_vbrinvgat_d_dn8_slot;
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

        let (assign1370_e2253,) = {
    if (var_guard10 == 0.0) {
        (p.p889,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign1370_e2253;

        let (assign1380_e2258,) = {
    if (var_guard10 == 0.0) {
        (p.p890,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign1380_e2258;

        let (assign1390_e2263,) = {
    if (var_guard10 == 0.0) {
        (p.p891,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign1390_e2263;

        let (assign1400_e2268,) = {
    if (var_guard10 == 0.0) {
        (p.p892,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign1400_e2268;

        let (assign1410_e2273,) = {
    if (var_guard10 == 0.0) {
        (p.p893,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign1410_e2273;

        let (assign1420_e2278,) = {
    if (var_guard10 == 0.0) {
        (p.p894,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign1420_e2278;

        let (assign1430_e2283,) = {
    if (var_guard10 == 0.0) {
        (p.p895,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign1430_e2283;

        let (assign1440_e2288,) = {
    if (var_guard10 == 0.0) {
        (p.p896,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign1440_e2288;

        let (assign1450_e2293,) = {
    if (var_guard10 == 0.0) {
        (p.p897,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign1450_e2293;

        let (assign1460_e2298,) = {
    if (var_guard10 == 0.0) {
        (p.p898,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign1460_e2298;

        let (assign1470_e2303,) = {
    if (var_guard10 == 0.0) {
        (p.p899,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign1470_e2303;

        let (assign1480_e2308,) = {
    if (var_guard10 == 0.0) {
        (p.p900,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign1480_e2308;

        let (assign1490_e2313,) = {
    if (var_guard10 == 0.0) {
        (p.p901,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign1490_e2313;

        let (assign1500_e2318,) = {
    if (var_guard10 == 0.0) {
        (p.p902,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign1500_e2318;

        let (assign1510_e2323,) = {
    if (var_guard10 == 0.0) {
        (p.p903,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign1510_e2323;

        let (assign1520_e2328,) = {
    if (var_guard10 == 0.0) {
        (p.p904,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign1520_e2328;

        let (assign1530_e2333,) = {
    if (var_guard10 == 0.0) {
        (p.p905,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1530_e2333;

        let (assign1540_e2338,) = {
    if (var_guard10 == 0.0) {
        (p.p906,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1540_e2338;

        let (assign1550_e2343,) = {
    if (var_guard10 == 0.0) {
        (p.p907,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1550_e2343;

        let (assign1560_e2348,) = {
    if (var_guard10 == 0.0) {
        (p.p908,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1560_e2348;

        let (assign1570_e2353,) = {
    if (var_guard10 == 0.0) {
        (p.p909,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1570_e2353;

        let (assign1580_e2358,) = {
    if (var_guard10 == 0.0) {
        (p.p910,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1580_e2358;

        let (assign1590_e2363,) = {
    if (var_guard10 == 0.0) {
        (p.p911,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1590_e2363;

        let (assign1600_e2368,) = {
    if (var_guard10 == 0.0) {
        (p.p912,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1600_e2368;

        let (assign1610_e2373,) = {
    if (var_guard10 == 0.0) {
        (p.p913,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1610_e2373;

        let (assign1620_e2378,) = {
    if (var_guard10 == 0.0) {
        (p.p914,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1620_e2378;

        let (assign1630_e2383,) = {
    if (var_guard10 == 0.0) {
        (p.p915,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1630_e2383;

        let (assign1640_e2388,) = {
    if (var_guard10 == 0.0) {
        (p.p916,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1640_e2388;

        let (assign1650_e2393,) = {
    if (var_guard10 == 0.0) {
        (p.p930,)
    } else {
        (var_vjunrefd_i,)
    }
};
        var_vjunrefd_i = assign1650_e2393;

        let (assign1660_e2398,) = {
    if (var_guard10 == 0.0) {
        (p.p931,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1660_e2398;

        let (assign1670_e2403,) = {
    if (var_guard10 == 0.0) {
        (p.p923,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1670_e2403;

        let (assign1680_e2408,) = {
    if (var_guard10 == 0.0) {
        (p.p924,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1680_e2408;

        let (assign1690_e2413,) = {
    if (var_guard10 == 0.0) {
        (p.p925,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1690_e2413;

        let (assign1700_e2418,) = {
    if (var_guard10 == 0.0) {
        (p.p926,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1700_e2418;

        let (assign1710_e2423,) = {
    if (var_guard10 == 0.0) {
        (p.p917,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1710_e2423;

        let (assign1720_e2428,) = {
    if (var_guard10 == 0.0) {
        (p.p918,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1720_e2428;

        let (assign1730_e2433,) = {
    if (var_guard10 == 0.0) {
        (p.p919,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1730_e2433;

        let (assign1740_e2438,) = {
    if (var_guard10 == 0.0) {
        (p.p920,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1740_e2438;

        let (assign1750_e2443,) = {
    if (var_guard10 == 0.0) {
        (p.p921,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1750_e2443;

        let (assign1760_e2448,) = {
    if (var_guard10 == 0.0) {
        (p.p922,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1760_e2448;

        let assign1770_e2451: f64 = (var_phigbotd_i + var_deltaphigr);
        var_phigrbot_d = assign1770_e2451;

        let assign1780_e2454: f64 = (var_phigstid_i + var_deltaphigr);
        var_phigrsti_d = assign1780_e2454;

        let assign1790_e2457: f64 = (var_phiggatd_i + var_deltaphigr);
        var_phigrgat_d = assign1790_e2457;

        let assign1800_e2460: f64 = (1.0 - var_pbotd_i);
        var_one_minus_pbot_d = assign1800_e2460;

        let assign1810_e2463: f64 = (1.0 - var_pstid_i);
        var_one_minus_psti_d = assign1810_e2463;

        let assign1820_e2466: f64 = (1.0 - var_pgatd_i);
        var_one_minus_pgat_d = assign1820_e2466;

        let assign1830_e2469: f64 = (1.0 / var_one_minus_pbot_d);
        var_one_over_one_minus_pbot_d = assign1830_e2469;

        let assign1840_e2472: f64 = (1.0 / var_one_minus_psti_d);
        var_one_over_one_minus_psti_d = assign1840_e2472;

        let assign1850_e2475: f64 = (1.0 / var_one_minus_pgat_d);
        var_one_over_one_minus_pgat_d = assign1850_e2475;

        let assign1860_e2478: f64 = (var_epssi / var_cjorbotd_i);
        var_wdepnulrbot_d = assign1860_e2478;

        let assign1870_e2481: f64 = (var_xjunstid_i * var_epssi);
        let assign1870_e2483: f64 = (assign1870_e2481 / var_cjorstid_i);
        var_wdepnulrsti_d = assign1870_e2483;

        let assign1880_e2486: f64 = (var_xjungatd_i * var_epssi);
        let assign1880_e2488: f64 = (assign1880_e2486 / var_cjorgatd_i);
        var_wdepnulrgat_d = assign1880_e2488;

        let assign1890_e2491: f64 = (1.0 / var_wdepnulrbot_d);
        var_wdepnulrinvbot_d = assign1890_e2491;

        let assign1900_e2494: f64 = (1.0 / var_wdepnulrsti_d);
        var_wdepnulrinvsti_d = assign1900_e2494;

        let assign1910_e2497: f64 = (1.0 / var_wdepnulrgat_d);
        var_wdepnulrinvgat_d = assign1910_e2497;

        let assign1920_e2500: f64 = (1.0 / var_vbirbotd_i);
        var_vbirbotinv_d = assign1920_e2500;

        let assign1930_e2503: f64 = (1.0 / var_vbirstid_i);
        var_vbirstiinv_d = assign1930_e2503;

        let assign1940_e2506: f64 = (1.0 / var_vbirgatd_i);
        var_vbirgatinv_d = assign1940_e2506;

        let assign1950_e2511: f64 = (var_alphaav).powf(var_pbrbotd_i);
        let assign1950_e2512: f64 = (1.0 - assign1950_e2511);
        let assign1950_e2513: f64 = (1.0 / assign1950_e2512);
        var_fstopbot_d = assign1950_e2513;

        let assign1960_e2518: f64 = (var_alphaav).powf(var_pbrstid_i);
        let assign1960_e2519: f64 = (1.0 - assign1960_e2518);
        let assign1960_e2520: f64 = (1.0 / assign1960_e2519);
        var_fstopsti_d = assign1960_e2520;

        let assign1970_e2525: f64 = (var_alphaav).powf(var_pbrgatd_i);
        let assign1970_e2526: f64 = (1.0 - assign1970_e2525);
        let assign1970_e2527: f64 = (1.0 / assign1970_e2526);
        var_fstopgat_d = assign1970_e2527;

        let assign1980_e2530: f64 = (1.0 / var_vbrbotd_i);
        var_vbrinvbot_d = assign1980_e2530;

        let assign1990_e2533: f64 = (1.0 / var_vbrstid_i);
        var_vbrinvsti_d = assign1990_e2533;

        let assign2000_e2536: f64 = (1.0 / var_vbrgatd_i);
        var_vbrinvgat_d = assign2000_e2536;
        var_vbrinvgat_d_dn5 = 0.0;
        var_vbrinvgat_d_dn6 = 0.0;
        var_vbrinvgat_d_dn7 = 0.0;
        var_vbrinvgat_d_dn8 = 0.0;

        let assign2010_e2539: f64 = (var_fstopbot_d * var_fstopbot_d);
        let assign2010_e2543: f64 = (var_pbrbotd_i - 1.0);
        let assign2010_e2544: f64 = (var_alphaav).powf(assign2010_e2543);
        let assign2010_e2545: f64 = (assign2010_e2539 * assign2010_e2544);
        let assign2010_e2546: f64 = (-assign2010_e2545);
        let assign2010_e2548: f64 = (assign2010_e2546 * var_pbrbotd_i);
        let assign2010_e2550: f64 = (assign2010_e2548 * var_vbrinvbot_d);
        var_slopebot_d = assign2010_e2550;

        let assign2020_e2553: f64 = (var_fstopsti_d * var_fstopsti_d);
        let assign2020_e2557: f64 = (var_pbrstid_i - 1.0);
        let assign2020_e2558: f64 = (var_alphaav).powf(assign2020_e2557);
        let assign2020_e2559: f64 = (assign2020_e2553 * assign2020_e2558);
        let assign2020_e2560: f64 = (-assign2020_e2559);
        let assign2020_e2562: f64 = (assign2020_e2560 * var_pbrstid_i);
        let assign2020_e2564: f64 = (assign2020_e2562 * var_vbrinvsti_d);
        var_slopesti_d = assign2020_e2564;

        let assign2030_e2567: f64 = (var_fstopgat_d * var_fstopgat_d);
        let assign2030_e2571: f64 = (var_pbrgatd_i - 1.0);
        let assign2030_e2572: f64 = (var_alphaav).powf(assign2030_e2571);
        let assign2030_e2573: f64 = (assign2030_e2567 * assign2030_e2572);
        let assign2030_e2574: f64 = (-assign2030_e2573);
        let assign2030_e2576: f64 = (assign2030_e2574 * var_pbrgatd_i);
        let assign2030_e2578: f64 = (assign2030_e2576 * var_vbrinvgat_d);
        var_slopegat_d = assign2030_e2578;
        var_slopegat_d_dn5 = (assign2030_e2576 * var_vbrinvgat_d_dn5);
        var_slopegat_d_dn6 = (assign2030_e2576 * var_vbrinvgat_d_dn6);
        var_slopegat_d_dn7 = (assign2030_e2576 * var_vbrinvgat_d_dn7);
        var_slopegat_d_dn8 = (assign2030_e2576 * var_vbrinvgat_d_dn8);

        let assign2040_e2593: f64 = if ((((var_fcjorgat2d_i != 1.0) || (var_fvbirgat2d_i != 1.0)) || (var_fpgat2d_i != 1.0)) || (var_fphiggat2d_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard11 = assign2040_e2593;

        let (assign2050_e2597,) = {
    if (var_guard11 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign2050_e2597;

        let (assign2060_e2602,) = {
    if (var_guard11 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign2060_e2602;

        let assign2070_e2605: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard12 = assign2070_e2605;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
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
        *var_fstopbot_d_slot = var_fstopbot_d;
        *var_fstopgat_d_slot = var_fstopgat_d;
        *var_fstopsti_d_slot = var_fstopsti_d;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_guard11_slot = var_guard11;
        *var_guard12_slot = var_guard12;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
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
        *var_slopegat_d_dn5_slot = var_slopegat_d_dn5;
        *var_slopegat_d_dn6_slot = var_slopegat_d_dn6;
        *var_slopegat_d_dn7_slot = var_slopegat_d_dn7;
        *var_slopegat_d_dn8_slot = var_slopegat_d_dn8;
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
        *var_vbrinvgat_d_dn5_slot = var_vbrinvgat_d_dn5;
        *var_vbrinvgat_d_dn6_slot = var_vbrinvgat_d_dn6;
        *var_vbrinvgat_d_dn7_slot = var_vbrinvgat_d_dn7;
        *var_vbrinvgat_d_dn8_slot = var_vbrinvgat_d_dn8;
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
        var_cjorgatd_i: f64,
        var_deltaphigr: f64,
        var_fcjorgat2d_i: f64,
        var_fpgat2d_i: f64,
        var_fphiggat2d_i: f64,
        var_fvbirgat2d_i: f64,
        var_guard12: f64,
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
        var_cjorgat2nd_d_slot: &mut f64,
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
        var_fbbtgat_dn5_slot: &mut f64,
        var_fbbtgat_dn6_slot: &mut f64,
        var_fbbtgat_dn7_slot: &mut f64,
        var_fbbtgat_dn8_slot: &mut f64,
        var_fbbtsti_slot: &mut f64,
        var_ftdbot_slot: &mut f64,
        var_ftdgat_slot: &mut f64,
        var_ftdgat2nd_slot: &mut f64,
        var_ftdsti_slot: &mut f64,
        var_guard32_slot: &mut f64,
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
        var_vbigat2nd_slot: &mut f64,
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
        let mut var_cjorgat2nd_d: f64 = *var_cjorgat2nd_d_slot;
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
        let mut var_fbbtgat_dn5: f64 = *var_fbbtgat_dn5_slot;
        let mut var_fbbtgat_dn6: f64 = *var_fbbtgat_dn6_slot;
        let mut var_fbbtgat_dn7: f64 = *var_fbbtgat_dn7_slot;
        let mut var_fbbtgat_dn8: f64 = *var_fbbtgat_dn8_slot;
        let mut var_fbbtsti: f64 = *var_fbbtsti_slot;
        let mut var_ftdbot: f64 = *var_ftdbot_slot;
        let mut var_ftdgat: f64 = *var_ftdgat_slot;
        let mut var_ftdgat2nd: f64 = *var_ftdgat2nd_slot;
        let mut var_ftdsti: f64 = *var_ftdsti_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
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
        let mut var_vbigat2nd: f64 = *var_vbigat2nd_slot;
        let mut var_vbiinvbot: f64 = *var_vbiinvbot_slot;
        let mut var_vbiinvgat: f64 = *var_vbiinvgat_slot;
        let mut var_vbiinvsti: f64 = *var_vbiinvsti_slot;
        let mut var_vbirgat2nd_d: f64 = *var_vbirgat2nd_d_slot;
        let mut var_vbisti: f64 = *var_vbisti_slot;

        let (assign2080_e2618,) = {
    if (var_guard12 != 0.0) {
        let assign2080_e2609: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
        let (assign2080_e2616,) = {
            if (assign2080_e2609 > 1e-18) {
                let assign2080_e2614: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
                (assign2080_e2614,)
            } else {
                (1e-18,)
            }
        };
        (assign2080_e2616,)
    } else {
        (var_cjorgat2nd_d,)
    }
};
        var_cjorgat2nd_d = assign2080_e2618;

        let (assign2090_e2631,) = {
    if (var_guard12 != 0.0) {
        let assign2090_e2622: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
        let (assign2090_e2629,) = {
            if (assign2090_e2622 > 0.05) {
                let assign2090_e2627: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
                (assign2090_e2627,)
            } else {
                (0.05,)
            }
        };
        (assign2090_e2629,)
    } else {
        (var_vbirgat2nd_d,)
    }
};
        var_vbirgat2nd_d = assign2090_e2631;

        let (assign2100_e2658,) = {
    if (var_guard12 != 0.0) {
        let assign2100_e2635: f64 = (var_pgatd_i * var_fpgat2d_i);
        let (assign2100_e2642,) = {
            if (assign2100_e2635 > 0.05) {
                let assign2100_e2640: f64 = (var_pgatd_i * var_fpgat2d_i);
                (assign2100_e2640,)
            } else {
                (0.05,)
            }
        };
        let (assign2100_e2656,) = {
            if (assign2100_e2642 < 0.95) {
                let assign2100_e2647: f64 = (var_pgatd_i * var_fpgat2d_i);
                let (assign2100_e2654,) = {
                    if (assign2100_e2647 > 0.05) {
                        let assign2100_e2652: f64 = (var_pgatd_i * var_fpgat2d_i);
                        (assign2100_e2652,)
                    } else {
                        (0.05,)
                    }
                };
                (assign2100_e2654,)
            } else {
                (0.95,)
            }
        };
        (assign2100_e2656,)
    } else {
        (var_pgat2nd_d,)
    }
};
        var_pgat2nd_d = assign2100_e2658;

        let (assign2110_e2664,) = {
    if (var_guard12 != 0.0) {
        let assign2110_e2662: f64 = (var_phiggatd_i * var_fphiggat2d_i);
        (assign2110_e2662,)
    } else {
        (var_phiggat2nd_d,)
    }
};
        var_phiggat2nd_d = assign2110_e2664;

        let (assign2120_e2670,) = {
    if (var_guard12 != 0.0) {
        let assign2120_e2668: f64 = (var_phiggat2nd_d + var_deltaphigr);
        (assign2120_e2668,)
    } else {
        (var_phigrgat2nd_d,)
    }
};
        var_phigrgat2nd_d = assign2120_e2670;

        let (assign2130_e2676,) = {
    if (var_guard12 != 0.0) {
        let assign2130_e2674: f64 = (1.0 - var_pgat2nd_d);
        (assign2130_e2674,)
    } else {
        (var_one_minus_pgat2nd_d,)
    }
};
        var_one_minus_pgat2nd_d = assign2130_e2676;

        let (assign2140_e2682,) = {
    if (var_guard12 != 0.0) {
        let assign2140_e2680: f64 = (1.0 / var_one_minus_pgat2nd_d);
        (assign2140_e2680,)
    } else {
        (var_one_over_one_minus_pgat2nd_d,)
    }
};
        var_one_over_one_minus_pgat2nd_d = assign2140_e2682;

        let assign2190_e2704: f64 = ctx_temp;
        let assign2190_e2706: f64 = (assign2190_e2704 + p.p56);
        let assign2190_e2708: f64 = (assign2190_e2706 + p.p35);
        var_tka = assign2190_e2708;

        let assign2200_e2711: f64 = (var_tka / var_tkr);
        var_rta = assign2200_e2711;

        let assign2210_e2714: f64 = (var_tka - var_tkr);
        var_delta = assign2210_e2714;

        let assign2220_e2717: f64 = (var_tka * 1.3806505e-23);
        let assign2220_e2719: f64 = (assign2220_e2717 / 1.6021918e-19);
        var_phita = assign2220_e2719;

        let assign2230_e2722: f64 = (1.0 / var_phita);
        var_inv_phita = assign2230_e2722;

        var_tkd = var_tka;

        let assign2250_e2726: f64 = (var_tkd * var_tkd);
        var_tkd_sq = assign2250_e2726;

        let assign2260_e2729: f64 = (var_tkd - var_tkr);
        var_delt = assign2260_e2729;

        let assign2270_e2732: f64 = (var_tkr / var_tkd);
        var_rtn = assign2270_e2732;

        let assign2280_e2734: f64 = (var_rtn).ln();
        var_ln_rtn = assign2280_e2734;

        let assign2290_e2737: f64 = (var_tkd * 1.3806505e-23);
        let assign2290_e2739: f64 = (assign2290_e2737 / 1.6021918e-19);
        var_phit = assign2290_e2739;

        let assign2300_e2742: f64 = (1.0 / var_phit);
        var_inv_phit = assign2300_e2742;

        let assign2310_e2746: f64 = (9.025e-5 * var_tkd);
        let assign2310_e2747: f64 = (1.179 - assign2310_e2746);
        let assign2310_e2750: f64 = (3.05e-7 * var_tkd_sq);
        let assign2310_e2751: f64 = (assign2310_e2747 - assign2310_e2750);
        var_eg = assign2310_e2751;

        let assign2320_e2755: f64 = (0.00045 * var_tkd);
        let assign2320_e2756: f64 = (1.045 + assign2320_e2755);
        let assign2320_e2760: f64 = (0.0014 * var_tkd);
        let assign2320_e2761: f64 = (0.523 + assign2320_e2760);
        let assign2320_e2764: f64 = (1.48e-6 * var_tkd_sq);
        let assign2320_e2765: f64 = (assign2320_e2761 - assign2320_e2764);
        let assign2320_e2766: f64 = (assign2320_e2756 * assign2320_e2765);
        let assign2320_e2768: f64 = (assign2320_e2766 * var_tkd_sq);
        let assign2320_e2770: f64 = (assign2320_e2768 / 90000.0);
        var_phibfac = assign2320_e2770;

        let (assign2330_e2776,) = {
    if (var_phibfac > 0.001) {
        (var_phibfac,)
    } else {
        (0.001,)
    }
};
        var_phibfac = assign2330_e2776;

        let assign2340_e2779: f64 = (4.0 * 1.3806505e-23);
        let assign2340_e2781: f64 = (assign2340_e2779 * var_tkd);
        var_nt0 = assign2340_e2781;

        let assign2350_e2782: f64 = ctx_temp;
        let assign2350_e2784: f64 = (assign2350_e2782 + p.p56);
        let assign2350_e2786: f64 = (assign2350_e2784 + p.p35);
        let assign2350_e2789: f64 = (-250.0);
        let assign2350_e2790: f64 = (273.15 + assign2350_e2789);
        let assign2350_e2791: f64 = (assign2350_e2786).max(assign2350_e2790);
        var_tkd_1 = assign2350_e2791;

        let assign2360_e2794: f64 = (var_tkd_1 / var_tkr_1);
        var_auxt = assign2360_e2794;

        let assign2370_e2797: f64 = (var_kbol_over_qele * var_tkd_1);
        var_phitd = assign2370_e2797;

        let assign2380_e2800: f64 = (1.0 / var_phitd);
        var_phitdinv = assign2380_e2800;

        let assign2390_e2803: f64 = (0.000702 * var_tkd_1);
        let assign2390_e2805: f64 = (assign2390_e2803 * var_tkd_1);
        let assign2390_e2806: f64 = (-assign2390_e2805);
        let assign2390_e2809: f64 = (1108.0 + var_tkd_1);
        let assign2390_e2810: f64 = (assign2390_e2806 / assign2390_e2809);
        var_deltaphigd = assign2390_e2810;

        let assign2400_e2813: f64 = (p.p834 + var_deltaphigd);
        var_phigdbot = assign2400_e2813;

        let assign2410_e2816: f64 = (p.p835 + var_deltaphigd);
        var_phigdsti = assign2410_e2816;

        let assign2420_e2819: f64 = (p.p836 + var_deltaphigd);
        var_phigdgat = assign2420_e2819;

        let assign2430_e2822: f64 = (var_auxt).powf(1.5);
        let assign2430_e2826: f64 = (var_phigrbot * var_phitrinv);
        let assign2430_e2829: f64 = (var_phigdbot * var_phitdinv);
        let assign2430_e2830: f64 = (assign2430_e2826 - assign2430_e2829);
        let assign2430_e2831: f64 = (0.5 * assign2430_e2830);
        let assign2430_e2832: f64 = (assign2430_e2831).exp();
        let assign2430_e2833: f64 = (assign2430_e2822 * assign2430_e2832);
        var_ftdbot = assign2430_e2833;

        let assign2440_e2836: f64 = (var_auxt).powf(1.5);
        let assign2440_e2840: f64 = (var_phigrsti * var_phitrinv);
        let assign2440_e2843: f64 = (var_phigdsti * var_phitdinv);
        let assign2440_e2844: f64 = (assign2440_e2840 - assign2440_e2843);
        let assign2440_e2845: f64 = (0.5 * assign2440_e2844);
        let assign2440_e2846: f64 = (assign2440_e2845).exp();
        let assign2440_e2847: f64 = (assign2440_e2836 * assign2440_e2846);
        var_ftdsti = assign2440_e2847;

        let assign2450_e2850: f64 = (var_auxt).powf(1.5);
        let assign2450_e2854: f64 = (var_phigrgat * var_phitrinv);
        let assign2450_e2857: f64 = (var_phigdgat * var_phitdinv);
        let assign2450_e2858: f64 = (assign2450_e2854 - assign2450_e2857);
        let assign2450_e2859: f64 = (0.5 * assign2450_e2858);
        let assign2450_e2860: f64 = (assign2450_e2859).exp();
        let assign2450_e2861: f64 = (assign2450_e2850 * assign2450_e2860);
        var_ftdgat = assign2450_e2861;

        let assign2460_e2864: f64 = (p.p837 * var_ftdbot);
        let assign2460_e2866: f64 = (assign2460_e2864 * var_ftdbot);
        var_idsatbot = assign2460_e2866;

        let assign2470_e2869: f64 = (p.p838 * var_ftdsti);
        let assign2470_e2871: f64 = (assign2470_e2869 * var_ftdsti);
        var_idsatsti = assign2470_e2871;

        let assign2480_e2874: f64 = (p.p839 * var_ftdgat);
        let assign2480_e2876: f64 = (assign2480_e2874 * var_ftdgat);
        var_idsatgat = assign2480_e2876;

        let assign2490_e2879: f64 = (p.p828 * var_auxt);
        let assign2490_e2882: f64 = (2.0 * var_phitd);
        let assign2490_e2884: f64 = (var_ftdbot).ln();
        let assign2490_e2885: f64 = (assign2490_e2882 * assign2490_e2884);
        let assign2490_e2886: f64 = (assign2490_e2879 - assign2490_e2885);
        var_ubibot = assign2490_e2886;

        let assign2500_e2889: f64 = (p.p829 * var_auxt);
        let assign2500_e2892: f64 = (2.0 * var_phitd);
        let assign2500_e2894: f64 = (var_ftdsti).ln();
        let assign2500_e2895: f64 = (assign2500_e2892 * assign2500_e2894);
        let assign2500_e2896: f64 = (assign2500_e2889 - assign2500_e2895);
        var_ubisti = assign2500_e2896;

        let assign2510_e2899: f64 = (p.p830 * var_auxt);
        let assign2510_e2902: f64 = (2.0 * var_phitd);
        let assign2510_e2904: f64 = (var_ftdgat).ln();
        let assign2510_e2905: f64 = (assign2510_e2902 * assign2510_e2904);
        let assign2510_e2906: f64 = (assign2510_e2899 - assign2510_e2905);
        var_ubigat = assign2510_e2906;

        let assign2520_e2912: f64 = (0.05 - var_ubibot);
        let assign2520_e2914: f64 = (assign2520_e2912 * var_phitdinv);
        let assign2520_e2915: f64 = (assign2520_e2914).exp();
        let assign2520_e2916: f64 = (1.0 + assign2520_e2915);
        let assign2520_e2917: f64 = (assign2520_e2916).ln();
        let assign2520_e2918: f64 = (var_phitd * assign2520_e2917);
        let assign2520_e2919: f64 = (var_ubibot + assign2520_e2918);
        var_vbibot = assign2520_e2919;

        let assign2530_e2925: f64 = (0.05 - var_ubisti);
        let assign2530_e2927: f64 = (assign2530_e2925 * var_phitdinv);
        let assign2530_e2928: f64 = (assign2530_e2927).exp();
        let assign2530_e2929: f64 = (1.0 + assign2530_e2928);
        let assign2530_e2930: f64 = (assign2530_e2929).ln();
        let assign2530_e2931: f64 = (var_phitd * assign2530_e2930);
        let assign2530_e2932: f64 = (var_ubisti + assign2530_e2931);
        var_vbisti = assign2530_e2932;

        let assign2540_e2938: f64 = (0.05 - var_ubigat);
        let assign2540_e2940: f64 = (assign2540_e2938 * var_phitdinv);
        let assign2540_e2941: f64 = (assign2540_e2940).exp();
        let assign2540_e2942: f64 = (1.0 + assign2540_e2941);
        let assign2540_e2943: f64 = (assign2540_e2942).ln();
        let assign2540_e2944: f64 = (var_phitd * assign2540_e2943);
        let assign2540_e2945: f64 = (var_ubigat + assign2540_e2944);
        var_vbigat = assign2540_e2945;

        let assign2550_e2948: f64 = (1.0 / var_vbibot);
        var_vbiinvbot = assign2550_e2948;

        let assign2560_e2951: f64 = (1.0 / var_vbisti);
        var_vbiinvsti = assign2560_e2951;

        let assign2570_e2954: f64 = (1.0 / var_vbigat);
        var_vbiinvgat = assign2570_e2954;

        let assign2580_e2958: f64 = (p.p828 * var_vbiinvbot);
        let assign2580_e2960: f64 = (assign2580_e2958).powf(p.p831);
        let assign2580_e2961: f64 = (p.p825 * assign2580_e2960);
        var_cjobot = assign2580_e2961;

        let assign2590_e2965: f64 = (p.p829 * var_vbiinvsti);
        let assign2590_e2967: f64 = (assign2590_e2965).powf(p.p832);
        let assign2590_e2968: f64 = (p.p826 * assign2590_e2967);
        var_cjosti = assign2590_e2968;

        let assign2600_e2972: f64 = (p.p830 * var_vbiinvgat);
        let assign2600_e2974: f64 = (assign2600_e2972).powf(p.p833);
        let assign2600_e2975: f64 = (p.p827 * assign2600_e2974);
        var_cjogat = assign2600_e2975;

        let assign2610_e2978: f64 = (var_cjobot * var_vbibot);
        let assign2610_e2980: f64 = (assign2610_e2978 * var_one_over_one_minus_pbot);
        var_qprefbot = assign2610_e2980;

        let assign2620_e2983: f64 = (var_cjosti * var_vbisti);
        let assign2620_e2985: f64 = (assign2620_e2983 * var_one_over_one_minus_psti);
        var_qprefsti = assign2620_e2985;

        let assign2630_e2988: f64 = (var_cjogat * var_vbigat);
        let assign2630_e2990: f64 = (assign2630_e2988 * var_one_over_one_minus_pgat);
        var_qprefgat = assign2630_e2990;

        let assign2640_e2993: f64 = (2.0 * var_cjobot);
        var_qpref2bot = assign2640_e2993;

        let assign2650_e2996: f64 = (2.0 * var_cjosti);
        var_qpref2sti = assign2650_e2996;

        let assign2660_e2999: f64 = (2.0 * var_cjogat);
        var_qpref2gat = assign2660_e2999;

        let assign2670_e3002: f64 = (0.5 * var_phigdbot);
        let assign2670_e3004: f64 = (assign2670_e3002).max(var_phitd);
        var_deltaebot = assign2670_e3004;

        let assign2680_e3007: f64 = (0.5 * var_phigdsti);
        let assign2680_e3009: f64 = (assign2680_e3007).max(var_phitd);
        var_deltaesti = assign2680_e3009;

        let assign2690_e3012: f64 = (0.5 * var_phigdgat);
        let assign2690_e3014: f64 = (assign2690_e3012).max(var_phitd);
        var_deltaegat = assign2690_e3014;

        let assign2700_e3017: f64 = (var_deltaebot * var_phitdinv);
        var_atatbot = assign2700_e3017;

        let assign2710_e3020: f64 = (var_deltaesti * var_phitdinv);
        var_atatsti = assign2710_e3020;

        let assign2720_e3023: f64 = (var_deltaegat * var_phitdinv);
        var_atatgat = assign2720_e3023;

        let assign2730_e3026: f64 = (32.0 * p.p848);
        let assign2730_e3028: f64 = (assign2730_e3026 * 9.1093826e-31);
        let assign2730_e3030: f64 = (assign2730_e3028 * 1.6021918e-19);
        let assign2730_e3033: f64 = (var_deltaebot * var_deltaebot);
        let assign2730_e3035: f64 = (assign2730_e3033 * var_deltaebot);
        let assign2730_e3036: f64 = (assign2730_e3030 * assign2730_e3035);
        let assign2730_e3037: f64 = (assign2730_e3036).sqrt();
        let assign2730_e3040: f64 = (3.0 * 1.05457168e-34);
        let assign2730_e3041: f64 = (assign2730_e3037 / assign2730_e3040);
        var_btatpartbot = assign2730_e3041;

        let assign2740_e3044: f64 = (32.0 * p.p849);
        let assign2740_e3046: f64 = (assign2740_e3044 * 9.1093826e-31);
        let assign2740_e3048: f64 = (assign2740_e3046 * 1.6021918e-19);
        let assign2740_e3051: f64 = (var_deltaesti * var_deltaesti);
        let assign2740_e3053: f64 = (assign2740_e3051 * var_deltaesti);
        let assign2740_e3054: f64 = (assign2740_e3048 * assign2740_e3053);
        let assign2740_e3055: f64 = (assign2740_e3054).sqrt();
        let assign2740_e3058: f64 = (3.0 * 1.05457168e-34);
        let assign2740_e3059: f64 = (assign2740_e3055 / assign2740_e3058);
        var_btatpartsti = assign2740_e3059;

        let assign2750_e3062: f64 = (32.0 * p.p850);
        let assign2750_e3064: f64 = (assign2750_e3062 * 9.1093826e-31);
        let assign2750_e3066: f64 = (assign2750_e3064 * 1.6021918e-19);
        let assign2750_e3069: f64 = (var_deltaegat * var_deltaegat);
        let assign2750_e3071: f64 = (assign2750_e3069 * var_deltaegat);
        let assign2750_e3072: f64 = (assign2750_e3066 * assign2750_e3071);
        let assign2750_e3073: f64 = (assign2750_e3072).sqrt();
        let assign2750_e3076: f64 = (3.0 * 1.05457168e-34);
        let assign2750_e3077: f64 = (assign2750_e3073 / assign2750_e3076);
        var_btatpartgat = assign2750_e3077;

        let assign2760_e3083: f64 = (var_tkd_1 - var_tkr_1);
        let assign2760_e3084: f64 = (p.p857 * assign2760_e3083);
        let assign2760_e3085: f64 = (1.0 + assign2760_e3084);
        let assign2760_e3086: f64 = (p.p854 * assign2760_e3085);
        var_fbbtbot = assign2760_e3086;

        let assign2770_e3092: f64 = (var_tkd_1 - var_tkr_1);
        let assign2770_e3093: f64 = (p.p858 * assign2770_e3092);
        let assign2770_e3094: f64 = (1.0 + assign2770_e3093);
        let assign2770_e3095: f64 = (p.p855 * assign2770_e3094);
        var_fbbtsti = assign2770_e3095;

        let assign2780_e3101: f64 = (var_tkd_1 - var_tkr_1);
        let assign2780_e3102: f64 = (p.p859 * assign2780_e3101);
        let assign2780_e3103: f64 = (1.0 + assign2780_e3102);
        let assign2780_e3104: f64 = (p.p856 * assign2780_e3103);
        var_fbbtgat = assign2780_e3104;
        var_fbbtgat_dn5 = 0.0;
        var_fbbtgat_dn6 = 0.0;
        var_fbbtgat_dn7 = 0.0;
        var_fbbtgat_dn8 = 0.0;

        let (assign2790_e3110,) = {
    if (var_fbbtbot > 0.0) {
        (var_fbbtbot,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot = assign2790_e3110;

        let (assign2800_e3116,) = {
    if (var_fbbtsti > 0.0) {
        (var_fbbtsti,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti = assign2800_e3116;

        let (assign2810_e3122, assign2810_e3122_d_n5, assign2810_e3122_d_n6, assign2810_e3122_d_n7, assign2810_e3122_d_n8,) = {
    if (var_fbbtgat > 0.0) {
        (var_fbbtgat, var_fbbtgat_dn5, var_fbbtgat_dn6, var_fbbtgat_dn7, var_fbbtgat_dn8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat = assign2810_e3122;
        var_fbbtgat_dn5 = assign2810_e3122_d_n5;
        var_fbbtgat_dn6 = assign2810_e3122_d_n6;
        var_fbbtgat_dn7 = assign2810_e3122_d_n7;
        var_fbbtgat_dn8 = assign2810_e3122_d_n8;

        let assign2820_e3125: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard32 = assign2820_e3125;

        let (assign2830_e3131,) = {
    if (var_guard32 != 0.0) {
        let assign2830_e3129: f64 = (var_phiggat2nd + var_deltaphigd);
        (assign2830_e3129,)
    } else {
        (var_phigdgat2nd,)
    }
};
        var_phigdgat2nd = assign2830_e3131;

        let (assign2840_e3148,) = {
    if (var_guard32 != 0.0) {
        let assign2840_e3135: f64 = (var_auxt).powf(1.5);
        let assign2840_e3139: f64 = (var_phigrgat2nd * var_phitrinv);
        let assign2840_e3142: f64 = (var_phigdgat2nd * var_phitdinv);
        let assign2840_e3143: f64 = (assign2840_e3139 - assign2840_e3142);
        let assign2840_e3144: f64 = (0.5 * assign2840_e3143);
        let assign2840_e3145: f64 = (assign2840_e3144).exp();
        let assign2840_e3146: f64 = (assign2840_e3135 * assign2840_e3145);
        (assign2840_e3146,)
    } else {
        (var_ftdgat2nd,)
    }
};
        var_ftdgat2nd = assign2840_e3148;

        let (assign2850_e3161,) = {
    if (var_guard32 != 0.0) {
        let assign2850_e3152: f64 = (var_vbirgat2nd * var_auxt);
        let assign2850_e3155: f64 = (2.0 * var_phitd);
        let assign2850_e3157: f64 = (var_ftdgat2nd).ln();
        let assign2850_e3158: f64 = (assign2850_e3155 * assign2850_e3157);
        let assign2850_e3159: f64 = (assign2850_e3152 - assign2850_e3158);
        (assign2850_e3159,)
    } else {
        (var_ubigat2nd,)
    }
};
        var_ubigat2nd = assign2850_e3161;

        let (assign2860_e3177,) = {
    if (var_guard32 != 0.0) {
        let assign2860_e3168: f64 = (0.05 - var_ubigat2nd);
        let assign2860_e3170: f64 = (assign2860_e3168 * var_phitdinv);
        let assign2860_e3171: f64 = (assign2860_e3170).exp();
        let assign2860_e3172: f64 = (1.0 + assign2860_e3171);
        let assign2860_e3173: f64 = (assign2860_e3172).ln();
        let assign2860_e3174: f64 = (var_phitd * assign2860_e3173);
        let assign2860_e3175: f64 = (var_ubigat2nd + assign2860_e3174);
        (assign2860_e3175,)
    } else {
        (var_vbigat2nd,)
    }
};
        var_vbigat2nd = assign2860_e3177;

        *var_atatbot_slot = var_atatbot;
        *var_atatgat_slot = var_atatgat;
        *var_atatsti_slot = var_atatsti;
        *var_auxt_slot = var_auxt;
        *var_btatpartbot_slot = var_btatpartbot;
        *var_btatpartgat_slot = var_btatpartgat;
        *var_btatpartsti_slot = var_btatpartsti;
        *var_cjobot_slot = var_cjobot;
        *var_cjogat_slot = var_cjogat;
        *var_cjorgat2nd_d_slot = var_cjorgat2nd_d;
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
        *var_fbbtgat_dn5_slot = var_fbbtgat_dn5;
        *var_fbbtgat_dn6_slot = var_fbbtgat_dn6;
        *var_fbbtgat_dn7_slot = var_fbbtgat_dn7;
        *var_fbbtgat_dn8_slot = var_fbbtgat_dn8;
        *var_fbbtsti_slot = var_fbbtsti;
        *var_ftdbot_slot = var_ftdbot;
        *var_ftdgat_slot = var_ftdgat;
        *var_ftdgat2nd_slot = var_ftdgat2nd;
        *var_ftdsti_slot = var_ftdsti;
        *var_guard32_slot = var_guard32;
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
        *var_vbigat2nd_slot = var_vbigat2nd;
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
        var_guard32: f64,
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
        var_vbigat2nd: f64,
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
        var_fbbtgat_d_dn5_slot: &mut f64,
        var_fbbtgat_d_dn6_slot: &mut f64,
        var_fbbtgat_d_dn7_slot: &mut f64,
        var_fbbtgat_d_dn8_slot: &mut f64,
        var_fbbtsti_d_slot: &mut f64,
        var_ftdbot_d_slot: &mut f64,
        var_ftdgat2nd_d_slot: &mut f64,
        var_ftdgat_d_slot: &mut f64,
        var_ftdsti_d_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_idsatbot_d_slot: &mut f64,
        var_idsatgat_d_slot: &mut f64,
        var_idsatsti_d_slot: &mut f64,
        var_il_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_jw_i_slot: &mut f64,
        var_l_i_slot: &mut f64,
        var_le_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_ngcon_i_slot: &mut f64,
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
        var_sca_i_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_sd_i_slot: &mut f64,
        var_ubibot_d_slot: &mut f64,
        var_ubigat2nd_d_slot: &mut f64,
        var_ubigat_d_slot: &mut f64,
        var_ubisti_d_slot: &mut f64,
        var_vbibot_d_slot: &mut f64,
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
        let mut var_fbbtgat_d_dn5: f64 = *var_fbbtgat_d_dn5_slot;
        let mut var_fbbtgat_d_dn6: f64 = *var_fbbtgat_d_dn6_slot;
        let mut var_fbbtgat_d_dn7: f64 = *var_fbbtgat_d_dn7_slot;
        let mut var_fbbtgat_d_dn8: f64 = *var_fbbtgat_d_dn8_slot;
        let mut var_fbbtsti_d: f64 = *var_fbbtsti_d_slot;
        let mut var_ftdbot_d: f64 = *var_ftdbot_d_slot;
        let mut var_ftdgat2nd_d: f64 = *var_ftdgat2nd_d_slot;
        let mut var_ftdgat_d: f64 = *var_ftdgat_d_slot;
        let mut var_ftdsti_d: f64 = *var_ftdsti_d_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_idsatbot_d: f64 = *var_idsatbot_d_slot;
        let mut var_idsatgat_d: f64 = *var_idsatgat_d_slot;
        let mut var_idsatsti_d: f64 = *var_idsatsti_d_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_jw_i: f64 = *var_jw_i_slot;
        let mut var_l_i: f64 = *var_l_i_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_ngcon_i: f64 = *var_ngcon_i_slot;
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
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_sd_i: f64 = *var_sd_i_slot;
        let mut var_ubibot_d: f64 = *var_ubibot_d_slot;
        let mut var_ubigat2nd_d: f64 = *var_ubigat2nd_d_slot;
        let mut var_ubigat_d: f64 = *var_ubigat_d_slot;
        let mut var_ubisti_d: f64 = *var_ubisti_d_slot;
        let mut var_vbibot_d: f64 = *var_vbibot_d_slot;
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

        let (assign2870_e3183,) = {
    if (var_guard32 != 0.0) {
        let assign2870_e3181: f64 = (1.0 / var_vbigat2nd);
        (assign2870_e3181,)
    } else {
        (var_vbiinvgat2nd,)
    }
};
        var_vbiinvgat2nd = assign2870_e3183;

        let (assign2880_e3193,) = {
    if (var_guard32 != 0.0) {
        let assign2880_e3188: f64 = (var_vbirgat2nd * var_vbiinvgat2nd);
        let assign2880_e3190: f64 = (assign2880_e3188).powf(var_pgat2nd);
        let assign2880_e3191: f64 = (var_cjorgat2nd * assign2880_e3190);
        (assign2880_e3191,)
    } else {
        (var_cjogat2nd,)
    }
};
        var_cjogat2nd = assign2880_e3193;

        let (assign2890_e3201,) = {
    if (var_guard32 != 0.0) {
        let assign2890_e3197: f64 = (var_cjogat2nd * var_vbigat2nd);
        let assign2890_e3199: f64 = (assign2890_e3197 * var_one_over_one_minus_pgat2nd);
        (assign2890_e3199,)
    } else {
        (var_qprefgat2nd,)
    }
};
        var_qprefgat2nd = assign2890_e3201;

        let (assign2900_e3207,) = {
    if (var_guard32 != 0.0) {
        let assign2900_e3205: f64 = (2.0 * var_cjogat2nd);
        (assign2900_e3205,)
    } else {
        (var_qpref2gat2nd,)
    }
};
        var_qpref2gat2nd = assign2900_e3207;

        let assign2910_e3210: f64 = (var_phigbotd_i + var_deltaphigd);
        var_phigdbot_d = assign2910_e3210;

        let assign2920_e3213: f64 = (var_phigstid_i + var_deltaphigd);
        var_phigdsti_d = assign2920_e3213;

        let assign2930_e3216: f64 = (var_phiggatd_i + var_deltaphigd);
        var_phigdgat_d = assign2930_e3216;

        let assign2940_e3219: f64 = (var_auxt).powf(1.5);
        let assign2940_e3223: f64 = (var_phigrbot_d * var_phitrinv);
        let assign2940_e3226: f64 = (var_phigdbot_d * var_phitdinv);
        let assign2940_e3227: f64 = (assign2940_e3223 - assign2940_e3226);
        let assign2940_e3228: f64 = (0.5 * assign2940_e3227);
        let assign2940_e3229: f64 = (assign2940_e3228).exp();
        let assign2940_e3230: f64 = (assign2940_e3219 * assign2940_e3229);
        var_ftdbot_d = assign2940_e3230;

        let assign2950_e3233: f64 = (var_auxt).powf(1.5);
        let assign2950_e3237: f64 = (var_phigrsti_d * var_phitrinv);
        let assign2950_e3240: f64 = (var_phigdsti_d * var_phitdinv);
        let assign2950_e3241: f64 = (assign2950_e3237 - assign2950_e3240);
        let assign2950_e3242: f64 = (0.5 * assign2950_e3241);
        let assign2950_e3243: f64 = (assign2950_e3242).exp();
        let assign2950_e3244: f64 = (assign2950_e3233 * assign2950_e3243);
        var_ftdsti_d = assign2950_e3244;

        let assign2960_e3247: f64 = (var_auxt).powf(1.5);
        let assign2960_e3251: f64 = (var_phigrgat_d * var_phitrinv);
        let assign2960_e3254: f64 = (var_phigdgat_d * var_phitdinv);
        let assign2960_e3255: f64 = (assign2960_e3251 - assign2960_e3254);
        let assign2960_e3256: f64 = (0.5 * assign2960_e3255);
        let assign2960_e3257: f64 = (assign2960_e3256).exp();
        let assign2960_e3258: f64 = (assign2960_e3247 * assign2960_e3257);
        var_ftdgat_d = assign2960_e3258;

        let assign2970_e3261: f64 = (var_idsatrbotd_i * var_ftdbot_d);
        let assign2970_e3263: f64 = (assign2970_e3261 * var_ftdbot_d);
        var_idsatbot_d = assign2970_e3263;

        let assign2980_e3266: f64 = (var_idsatrstid_i * var_ftdsti_d);
        let assign2980_e3268: f64 = (assign2980_e3266 * var_ftdsti_d);
        var_idsatsti_d = assign2980_e3268;

        let assign2990_e3271: f64 = (var_idsatrgatd_i * var_ftdgat_d);
        let assign2990_e3273: f64 = (assign2990_e3271 * var_ftdgat_d);
        var_idsatgat_d = assign2990_e3273;

        let assign3000_e3276: f64 = (var_vbirbotd_i * var_auxt);
        let assign3000_e3279: f64 = (2.0 * var_phitd);
        let assign3000_e3281: f64 = (var_ftdbot_d).ln();
        let assign3000_e3282: f64 = (assign3000_e3279 * assign3000_e3281);
        let assign3000_e3283: f64 = (assign3000_e3276 - assign3000_e3282);
        var_ubibot_d = assign3000_e3283;

        let assign3010_e3286: f64 = (var_vbirstid_i * var_auxt);
        let assign3010_e3289: f64 = (2.0 * var_phitd);
        let assign3010_e3291: f64 = (var_ftdsti_d).ln();
        let assign3010_e3292: f64 = (assign3010_e3289 * assign3010_e3291);
        let assign3010_e3293: f64 = (assign3010_e3286 - assign3010_e3292);
        var_ubisti_d = assign3010_e3293;

        let assign3020_e3296: f64 = (var_vbirgatd_i * var_auxt);
        let assign3020_e3299: f64 = (2.0 * var_phitd);
        let assign3020_e3301: f64 = (var_ftdgat_d).ln();
        let assign3020_e3302: f64 = (assign3020_e3299 * assign3020_e3301);
        let assign3020_e3303: f64 = (assign3020_e3296 - assign3020_e3302);
        var_ubigat_d = assign3020_e3303;

        let assign3030_e3309: f64 = (0.05 - var_ubibot_d);
        let assign3030_e3311: f64 = (assign3030_e3309 * var_phitdinv);
        let assign3030_e3312: f64 = (assign3030_e3311).exp();
        let assign3030_e3313: f64 = (1.0 + assign3030_e3312);
        let assign3030_e3314: f64 = (assign3030_e3313).ln();
        let assign3030_e3315: f64 = (var_phitd * assign3030_e3314);
        let assign3030_e3316: f64 = (var_ubibot_d + assign3030_e3315);
        var_vbibot_d = assign3030_e3316;

        let assign3040_e3322: f64 = (0.05 - var_ubisti_d);
        let assign3040_e3324: f64 = (assign3040_e3322 * var_phitdinv);
        let assign3040_e3325: f64 = (assign3040_e3324).exp();
        let assign3040_e3326: f64 = (1.0 + assign3040_e3325);
        let assign3040_e3327: f64 = (assign3040_e3326).ln();
        let assign3040_e3328: f64 = (var_phitd * assign3040_e3327);
        let assign3040_e3329: f64 = (var_ubisti_d + assign3040_e3328);
        var_vbisti_d = assign3040_e3329;

        let assign3050_e3335: f64 = (0.05 - var_ubigat_d);
        let assign3050_e3337: f64 = (assign3050_e3335 * var_phitdinv);
        let assign3050_e3338: f64 = (assign3050_e3337).exp();
        let assign3050_e3339: f64 = (1.0 + assign3050_e3338);
        let assign3050_e3340: f64 = (assign3050_e3339).ln();
        let assign3050_e3341: f64 = (var_phitd * assign3050_e3340);
        let assign3050_e3342: f64 = (var_ubigat_d + assign3050_e3341);
        var_vbigat_d = assign3050_e3342;

        let assign3060_e3345: f64 = (1.0 / var_vbibot_d);
        var_vbiinvbot_d = assign3060_e3345;

        let assign3070_e3348: f64 = (1.0 / var_vbisti_d);
        var_vbiinvsti_d = assign3070_e3348;

        let assign3080_e3351: f64 = (1.0 / var_vbigat_d);
        var_vbiinvgat_d = assign3080_e3351;

        let assign3090_e3355: f64 = (var_vbirbotd_i * var_vbiinvbot_d);
        let assign3090_e3357: f64 = (assign3090_e3355).powf(var_pbotd_i);
        let assign3090_e3358: f64 = (var_cjorbotd_i * assign3090_e3357);
        var_cjobot_d = assign3090_e3358;

        let assign3100_e3362: f64 = (var_vbirstid_i * var_vbiinvsti_d);
        let assign3100_e3364: f64 = (assign3100_e3362).powf(var_pstid_i);
        let assign3100_e3365: f64 = (var_cjorstid_i * assign3100_e3364);
        var_cjosti_d = assign3100_e3365;

        let assign3110_e3369: f64 = (var_vbirgatd_i * var_vbiinvgat_d);
        let assign3110_e3371: f64 = (assign3110_e3369).powf(var_pgatd_i);
        let assign3110_e3372: f64 = (var_cjorgatd_i * assign3110_e3371);
        var_cjogat_d = assign3110_e3372;

        let assign3120_e3375: f64 = (var_cjobot_d * var_vbibot_d);
        let assign3120_e3377: f64 = (assign3120_e3375 * var_one_over_one_minus_pbot_d);
        var_qprefbot_d = assign3120_e3377;

        let assign3130_e3380: f64 = (var_cjosti_d * var_vbisti_d);
        let assign3130_e3382: f64 = (assign3130_e3380 * var_one_over_one_minus_psti_d);
        var_qprefsti_d = assign3130_e3382;

        let assign3140_e3385: f64 = (var_cjogat_d * var_vbigat_d);
        let assign3140_e3387: f64 = (assign3140_e3385 * var_one_over_one_minus_pgat_d);
        var_qprefgat_d = assign3140_e3387;

        let assign3150_e3390: f64 = (2.0 * var_cjobot_d);
        var_qpref2bot_d = assign3150_e3390;

        let assign3160_e3393: f64 = (2.0 * var_cjosti_d);
        var_qpref2sti_d = assign3160_e3393;

        let assign3170_e3396: f64 = (2.0 * var_cjogat_d);
        var_qpref2gat_d = assign3170_e3396;

        let assign3180_e3399: f64 = (0.5 * var_phigdbot_d);
        let assign3180_e3401: f64 = (assign3180_e3399).max(var_phitd);
        var_deltaebot_d = assign3180_e3401;

        let assign3190_e3404: f64 = (0.5 * var_phigdsti_d);
        let assign3190_e3406: f64 = (assign3190_e3404).max(var_phitd);
        var_deltaesti_d = assign3190_e3406;

        let assign3200_e3409: f64 = (0.5 * var_phigdgat_d);
        let assign3200_e3411: f64 = (assign3200_e3409).max(var_phitd);
        var_deltaegat_d = assign3200_e3411;

        let assign3210_e3414: f64 = (var_deltaebot_d * var_phitdinv);
        var_atatbot_d = assign3210_e3414;

        let assign3220_e3417: f64 = (var_deltaesti_d * var_phitdinv);
        var_atatsti_d = assign3220_e3417;

        let assign3230_e3420: f64 = (var_deltaegat_d * var_phitdinv);
        var_atatgat_d = assign3230_e3420;

        let assign3240_e3423: f64 = (32.0 * var_mefftatbotd_i);
        let assign3240_e3425: f64 = (assign3240_e3423 * 9.1093826e-31);
        let assign3240_e3427: f64 = (assign3240_e3425 * 1.6021918e-19);
        let assign3240_e3430: f64 = (var_deltaebot_d * var_deltaebot_d);
        let assign3240_e3432: f64 = (assign3240_e3430 * var_deltaebot_d);
        let assign3240_e3433: f64 = (assign3240_e3427 * assign3240_e3432);
        let assign3240_e3434: f64 = (assign3240_e3433).sqrt();
        let assign3240_e3437: f64 = (3.0 * 1.05457168e-34);
        let assign3240_e3438: f64 = (assign3240_e3434 / assign3240_e3437);
        var_btatpartbot_d = assign3240_e3438;

        let assign3250_e3441: f64 = (32.0 * var_mefftatstid_i);
        let assign3250_e3443: f64 = (assign3250_e3441 * 9.1093826e-31);
        let assign3250_e3445: f64 = (assign3250_e3443 * 1.6021918e-19);
        let assign3250_e3448: f64 = (var_deltaesti_d * var_deltaesti_d);
        let assign3250_e3450: f64 = (assign3250_e3448 * var_deltaesti_d);
        let assign3250_e3451: f64 = (assign3250_e3445 * assign3250_e3450);
        let assign3250_e3452: f64 = (assign3250_e3451).sqrt();
        let assign3250_e3455: f64 = (3.0 * 1.05457168e-34);
        let assign3250_e3456: f64 = (assign3250_e3452 / assign3250_e3455);
        var_btatpartsti_d = assign3250_e3456;

        let assign3260_e3459: f64 = (32.0 * var_mefftatgatd_i);
        let assign3260_e3461: f64 = (assign3260_e3459 * 9.1093826e-31);
        let assign3260_e3463: f64 = (assign3260_e3461 * 1.6021918e-19);
        let assign3260_e3466: f64 = (var_deltaegat_d * var_deltaegat_d);
        let assign3260_e3468: f64 = (assign3260_e3466 * var_deltaegat_d);
        let assign3260_e3469: f64 = (assign3260_e3463 * assign3260_e3468);
        let assign3260_e3470: f64 = (assign3260_e3469).sqrt();
        let assign3260_e3473: f64 = (3.0 * 1.05457168e-34);
        let assign3260_e3474: f64 = (assign3260_e3470 / assign3260_e3473);
        var_btatpartgat_d = assign3260_e3474;

        let assign3270_e3480: f64 = (var_tkd_1 - var_tkr_1);
        let assign3270_e3481: f64 = (var_stfbbtbotd_i * assign3270_e3480);
        let assign3270_e3482: f64 = (1.0 + assign3270_e3481);
        let assign3270_e3483: f64 = (var_fbbtrbotd_i * assign3270_e3482);
        var_fbbtbot_d = assign3270_e3483;

        let assign3280_e3489: f64 = (var_tkd_1 - var_tkr_1);
        let assign3280_e3490: f64 = (var_stfbbtstid_i * assign3280_e3489);
        let assign3280_e3491: f64 = (1.0 + assign3280_e3490);
        let assign3280_e3492: f64 = (var_fbbtrstid_i * assign3280_e3491);
        var_fbbtsti_d = assign3280_e3492;

        let assign3290_e3498: f64 = (var_tkd_1 - var_tkr_1);
        let assign3290_e3499: f64 = (var_stfbbtgatd_i * assign3290_e3498);
        let assign3290_e3500: f64 = (1.0 + assign3290_e3499);
        let assign3290_e3501: f64 = (var_fbbtrgatd_i * assign3290_e3500);
        var_fbbtgat_d = assign3290_e3501;
        var_fbbtgat_d_dn5 = 0.0;
        var_fbbtgat_d_dn6 = 0.0;
        var_fbbtgat_d_dn7 = 0.0;
        var_fbbtgat_d_dn8 = 0.0;

        let (assign3300_e3507,) = {
    if (var_fbbtbot_d > 0.0) {
        (var_fbbtbot_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot_d = assign3300_e3507;

        let (assign3310_e3513,) = {
    if (var_fbbtsti_d > 0.0) {
        (var_fbbtsti_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti_d = assign3310_e3513;

        let (assign3320_e3519, assign3320_e3519_d_n5, assign3320_e3519_d_n6, assign3320_e3519_d_n7, assign3320_e3519_d_n8,) = {
    if (var_fbbtgat_d > 0.0) {
        (var_fbbtgat_d, var_fbbtgat_d_dn5, var_fbbtgat_d_dn6, var_fbbtgat_d_dn7, var_fbbtgat_d_dn8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat_d = assign3320_e3519;
        var_fbbtgat_d_dn5 = assign3320_e3519_d_n5;
        var_fbbtgat_d_dn6 = assign3320_e3519_d_n6;
        var_fbbtgat_d_dn7 = assign3320_e3519_d_n7;
        var_fbbtgat_d_dn8 = assign3320_e3519_d_n8;

        let assign3330_e3522: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3330_e3522;

        let (assign3340_e3528,) = {
    if (var_guard33 != 0.0) {
        let assign3340_e3526: f64 = (var_phiggat2nd_d + var_deltaphigd);
        (assign3340_e3526,)
    } else {
        (var_phigdgat2nd_d,)
    }
};
        var_phigdgat2nd_d = assign3340_e3528;

        let (assign3350_e3545,) = {
    if (var_guard33 != 0.0) {
        let assign3350_e3532: f64 = (var_auxt).powf(1.5);
        let assign3350_e3536: f64 = (var_phigrgat2nd_d * var_phitrinv);
        let assign3350_e3539: f64 = (var_phigdgat2nd_d * var_phitdinv);
        let assign3350_e3540: f64 = (assign3350_e3536 - assign3350_e3539);
        let assign3350_e3541: f64 = (0.5 * assign3350_e3540);
        let assign3350_e3542: f64 = (assign3350_e3541).exp();
        let assign3350_e3543: f64 = (assign3350_e3532 * assign3350_e3542);
        (assign3350_e3543,)
    } else {
        (var_ftdgat2nd_d,)
    }
};
        var_ftdgat2nd_d = assign3350_e3545;

        let (assign3360_e3558,) = {
    if (var_guard33 != 0.0) {
        let assign3360_e3549: f64 = (var_vbirgat2nd_d * var_auxt);
        let assign3360_e3552: f64 = (2.0 * var_phitd);
        let assign3360_e3554: f64 = (var_ftdgat2nd_d).ln();
        let assign3360_e3555: f64 = (assign3360_e3552 * assign3360_e3554);
        let assign3360_e3556: f64 = (assign3360_e3549 - assign3360_e3555);
        (assign3360_e3556,)
    } else {
        (var_ubigat2nd_d,)
    }
};
        var_ubigat2nd_d = assign3360_e3558;

        let (assign3370_e3574,) = {
    if (var_guard33 != 0.0) {
        let assign3370_e3565: f64 = (0.05 - var_ubigat2nd_d);
        let assign3370_e3567: f64 = (assign3370_e3565 * var_phitdinv);
        let assign3370_e3568: f64 = (assign3370_e3567).exp();
        let assign3370_e3569: f64 = (1.0 + assign3370_e3568);
        let assign3370_e3570: f64 = (assign3370_e3569).ln();
        let assign3370_e3571: f64 = (var_phitd * assign3370_e3570);
        let assign3370_e3572: f64 = (var_ubigat2nd_d + assign3370_e3571);
        (assign3370_e3572,)
    } else {
        (var_vbigat2nd_d,)
    }
};
        var_vbigat2nd_d = assign3370_e3574;

        let (assign3380_e3580,) = {
    if (var_guard33 != 0.0) {
        let assign3380_e3578: f64 = (1.0 / var_vbigat2nd_d);
        (assign3380_e3578,)
    } else {
        (var_vbiinvgat2nd_d,)
    }
};
        var_vbiinvgat2nd_d = assign3380_e3580;

        let (assign3390_e3590,) = {
    if (var_guard33 != 0.0) {
        let assign3390_e3585: f64 = (var_vbirgat2nd_d * var_vbiinvgat2nd_d);
        let assign3390_e3587: f64 = (assign3390_e3585).powf(var_pgat2nd_d);
        let assign3390_e3588: f64 = (var_cjorgat2nd_d * assign3390_e3587);
        (assign3390_e3588,)
    } else {
        (var_cjogat2nd_d,)
    }
};
        var_cjogat2nd_d = assign3390_e3590;

        let (assign3400_e3598,) = {
    if (var_guard33 != 0.0) {
        let assign3400_e3594: f64 = (var_cjogat2nd_d * var_vbigat2nd_d);
        let assign3400_e3596: f64 = (assign3400_e3594 * var_one_over_one_minus_pgat2nd_d);
        (assign3400_e3596,)
    } else {
        (var_qprefgat2nd_d,)
    }
};
        var_qprefgat2nd_d = assign3400_e3598;

        let (assign3410_e3604,) = {
    if (var_guard33 != 0.0) {
        let assign3410_e3602: f64 = (2.0 * var_cjogat2nd_d);
        (assign3410_e3602,)
    } else {
        (var_qpref2gat2nd_d,)
    }
};
        var_qpref2gat2nd_d = assign3410_e3604;

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

        let assign3640_e3629: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard34 = assign3640_e3629;

        let (assign3650_e3638,) = {
    if (var_guard34 != 0.0) {
        let (assign3650_e3636,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3650_e3636,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3650_e3638;

        let (assign3660_e3645,) = {
    if (var_guard34 != 0.0) {
        let assign3660_e3642: f64 = (var_nf_i + 0.5);
        let assign3660_e3643: f64 = (assign3660_e3642).floor();
        (assign3660_e3643,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3660_e3645;

        let (assign3670_e3651,) = {
    if (var_guard34 != 0.0) {
        let assign3670_e3649: f64 = (1.0 / var_nf_i);
        (assign3670_e3649,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign3670_e3651;

        let assign3680_e3654: f64 = (var_w_i * var_invnf);
        let (assign3680_e3661,) = {
    if (assign3680_e3654 > 1e-9) {
        let assign3680_e3659: f64 = (var_w_i * var_invnf);
        (assign3680_e3659,)
    } else {
        (1e-9,)
    }
};
        var_w_i = assign3680_e3661;

        var_sca_i = p.p5;

        var_scb_i = p.p6;

        var_scc_i = p.p7;

        let (assign3720_e3670,) = {
    if (p.p10 < 1.5) {
        (1.0,)
    } else {
        (2.0,)
    }
};
        var_ngcon_i = assign3720_e3670;

        let assign3730_e3673: f64 = (1e-6 / var_l_i);
        var_il = assign3730_e3673;

        let assign3740_e3676: f64 = (1e-6 / var_w_i);
        var_iw = assign3740_e3676;

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
        *var_fbbtgat_d_dn5_slot = var_fbbtgat_d_dn5;
        *var_fbbtgat_d_dn6_slot = var_fbbtgat_d_dn6;
        *var_fbbtgat_d_dn7_slot = var_fbbtgat_d_dn7;
        *var_fbbtgat_d_dn8_slot = var_fbbtgat_d_dn8;
        *var_fbbtsti_d_slot = var_fbbtsti_d;
        *var_ftdbot_d_slot = var_ftdbot_d;
        *var_ftdgat2nd_d_slot = var_ftdgat2nd_d;
        *var_ftdgat_d_slot = var_ftdgat_d;
        *var_ftdsti_d_slot = var_ftdsti_d;
        *var_guard33_slot = var_guard33;
        *var_guard34_slot = var_guard34;
        *var_idsatbot_d_slot = var_idsatbot_d;
        *var_idsatgat_d_slot = var_idsatgat_d;
        *var_idsatsti_d_slot = var_idsatsti_d;
        *var_il_slot = var_il;
        *var_invnf_slot = var_invnf;
        *var_iw_slot = var_iw;
        *var_jw_i_slot = var_jw_i;
        *var_l_i_slot = var_l_i;
        *var_le_slot = var_le;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lssource_i_slot = var_lssource_i;
        *var_nf_i_slot = var_nf_i;
        *var_ngcon_i_slot = var_ngcon_i;
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
        *var_sca_i_slot = var_sca_i;
        *var_scb_i_slot = var_scb_i;
        *var_scc_i_slot = var_scc_i;
        *var_sd_i_slot = var_sd_i;
        *var_ubibot_d_slot = var_ubibot_d;
        *var_ubigat2nd_d_slot = var_ubigat2nd_d;
        *var_ubigat_d_slot = var_ubigat_d;
        *var_ubisti_d_slot = var_ubisti_d;
        *var_vbibot_d_slot = var_vbibot_d;
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
        var_il: f64,
        var_iw: f64,
        var_l_i: f64,
        var_w_i: f64,
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
        var_cox_p_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delwod_slot: &mut f64,
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
        var_guard35_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard38_slot: &mut f64,
        var_guard39_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_iae_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_iiae_slot: &mut f64,
        var_iilcv_slot: &mut f64,
        var_iiwcv_slot: &mut f64,
        var_iiwe_slot: &mut f64,
        var_iiwecv_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_ile2_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_l_f_slot: &mut f64,
        var_l_slif_slot: &mut f64,
        var_lcv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_lecv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_munqs_p_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
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
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
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
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard38: f64 = *var_guard38_slot;
        let mut var_guard39: f64 = *var_guard39_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_iiae: f64 = *var_iiae_slot;
        let mut var_iilcv: f64 = *var_iilcv_slot;
        let mut var_iiwcv: f64 = *var_iiwcv_slot;
        let mut var_iiwe: f64 = *var_iiwe_slot;
        let mut var_iiwecv: f64 = *var_iiwecv_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_ile2: f64 = *var_ile2_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_l_f: f64 = *var_l_f_slot;
        let mut var_l_slif: f64 = *var_l_slif_slot;
        let mut var_lcv: f64 = *var_lcv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_lecv: f64 = *var_lecv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_munqs_p: f64 = *var_munqs_p_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
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
        let mut var_we: f64 = *var_we_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xgwe: f64 = *var_xgwe_slot;

        let assign3750_e3681: f64 = (p.p189 * var_il);
        let assign3750_e3682: f64 = (1.0 + assign3750_e3681);
        let assign3750_e3683: f64 = (p.p188 * assign3750_e3682);
        let assign3750_e3687: f64 = (p.p190 * var_iw);
        let assign3750_e3688: f64 = (1.0 + assign3750_e3687);
        let assign3750_e3689: f64 = (assign3750_e3683 * assign3750_e3688);
        var_dellps = assign3750_e3689;

        let assign3760_e3694: f64 = (p.p193 * var_il);
        let assign3760_e3695: f64 = (1.0 + assign3760_e3694);
        let assign3760_e3696: f64 = (p.p192 * assign3760_e3695);
        let assign3760_e3700: f64 = (p.p194 * var_iw);
        let assign3760_e3701: f64 = (1.0 + assign3760_e3700);
        let assign3760_e3702: f64 = (assign3760_e3696 * assign3760_e3701);
        var_delwod = assign3760_e3702;

        let assign3770_e3705: f64 = (var_l_i + var_dellps);
        let assign3770_e3708: f64 = (2.0 * p.p191);
        let assign3770_e3709: f64 = (assign3770_e3705 - assign3770_e3708);
        let (assign3770_e3720,) = {
    if (assign3770_e3709 > 1e-9) {
        let assign3770_e3714: f64 = (var_l_i + var_dellps);
        let assign3770_e3717: f64 = (2.0 * p.p191);
        let assign3770_e3718: f64 = (assign3770_e3714 - assign3770_e3717);
        (assign3770_e3718,)
    } else {
        (1e-9,)
    }
};
        var_le = assign3770_e3720;

        let assign3780_e3723: f64 = (var_w_i + var_delwod);
        let assign3780_e3726: f64 = (2.0 * p.p195);
        let assign3780_e3727: f64 = (assign3780_e3723 - assign3780_e3726);
        let (assign3780_e3738,) = {
    if (assign3780_e3727 > 1e-9) {
        let assign3780_e3732: f64 = (var_w_i + var_delwod);
        let assign3780_e3735: f64 = (2.0 * p.p195);
        let assign3780_e3736: f64 = (assign3780_e3732 - assign3780_e3735);
        (assign3780_e3736,)
    } else {
        (1e-9,)
    }
};
        var_we = assign3780_e3738;

        let assign3790_e3741: f64 = (1e-6 / var_le);
        var_ile = assign3790_e3741;

        let assign3800_e3744: f64 = (var_ile * var_ile);
        var_ile2 = assign3800_e3744;

        let assign3810_e3747: f64 = (1e-6 / var_we);
        var_iwe = assign3810_e3747;

        let assign3820_e3750: f64 = (1.0 / var_iwe);
        var_iiwe = assign3820_e3750;

        let assign3830_e3753: f64 = (var_ile * var_iwe);
        var_iae = assign3830_e3753;

        let assign3840_e3756: f64 = (1.0 / var_iae);
        var_iiae = assign3840_e3756;

        let assign3850_e3759: f64 = (var_l_i + var_dellps);
        let assign3850_e3762: f64 = (2.0 * p.p191);
        let assign3850_e3763: f64 = (assign3850_e3759 - assign3850_e3762);
        let assign3850_e3765: f64 = (assign3850_e3763 + p.p196);
        let (assign3850_e3778,) = {
    if (assign3850_e3765 > 1e-9) {
        let assign3850_e3770: f64 = (var_l_i + var_dellps);
        let assign3850_e3773: f64 = (2.0 * p.p191);
        let assign3850_e3774: f64 = (assign3850_e3770 - assign3850_e3773);
        let assign3850_e3776: f64 = (assign3850_e3774 + p.p196);
        (assign3850_e3776,)
    } else {
        (1e-9,)
    }
};
        var_lecv = assign3850_e3778;

        let assign3860_e3781: f64 = (var_w_i + var_delwod);
        let assign3860_e3784: f64 = (2.0 * p.p195);
        let assign3860_e3785: f64 = (assign3860_e3781 - assign3860_e3784);
        let assign3860_e3787: f64 = (assign3860_e3785 + p.p197);
        let (assign3860_e3800,) = {
    if (assign3860_e3787 > 1e-9) {
        let assign3860_e3792: f64 = (var_w_i + var_delwod);
        let assign3860_e3795: f64 = (2.0 * p.p195);
        let assign3860_e3796: f64 = (assign3860_e3792 - assign3860_e3795);
        let assign3860_e3798: f64 = (assign3860_e3796 + p.p197);
        (assign3860_e3798,)
    } else {
        (1e-9,)
    }
};
        var_wecv = assign3860_e3800;

        let assign3870_e3803: f64 = (var_wecv / 1e-6);
        var_iiwecv = assign3870_e3803;

        let assign3880_e3806: f64 = (var_l_i + var_dellps);
        let assign3880_e3808: f64 = (assign3880_e3806 + p.p196);
        let (assign3880_e3817,) = {
    if (assign3880_e3808 > 1e-9) {
        let assign3880_e3813: f64 = (var_l_i + var_dellps);
        let assign3880_e3815: f64 = (assign3880_e3813 + p.p196);
        (assign3880_e3815,)
    } else {
        (1e-9,)
    }
};
        var_lcv = assign3880_e3817;

        let assign3890_e3820: f64 = (var_w_i + var_delwod);
        let assign3890_e3822: f64 = (assign3890_e3820 + p.p197);
        let (assign3890_e3831,) = {
    if (assign3890_e3822 > 1e-9) {
        let assign3890_e3827: f64 = (var_w_i + var_delwod);
        let assign3890_e3829: f64 = (assign3890_e3827 + p.p197);
        (assign3890_e3829,)
    } else {
        (1e-9,)
    }
};
        var_wcv = assign3890_e3831;

        let assign3900_e3834: f64 = (var_lcv / 1e-6);
        var_iilcv = assign3900_e3834;

        let assign3910_e3837: f64 = (var_wcv / 1e-6);
        var_iiwcv = assign3910_e3837;

        let assign3920_e3840: f64 = (var_l_i + var_dellps);
        let (assign3920_e3847,) = {
    if (assign3920_e3840 > 1e-9) {
        let assign3920_e3845: f64 = (var_l_i + var_dellps);
        (assign3920_e3845,)
    } else {
        (1e-9,)
    }
};
        var_l_f = assign3920_e3847;

        let assign3930_e3850: f64 = (var_l_f + p.p443);
        let (assign3930_e3857,) = {
    if (assign3930_e3850 > 1e-9) {
        let assign3930_e3855: f64 = (var_l_f + p.p443);
        (assign3930_e3855,)
    } else {
        (1e-9,)
    }
};
        var_l_slif = assign3930_e3857;

        let assign3940_e3860: f64 = (var_w_i + var_delwod);
        let (assign3940_e3867,) = {
    if (assign3940_e3860 > 1e-9) {
        let assign3940_e3865: f64 = (var_w_i + var_delwod);
        (assign3940_e3865,)
    } else {
        (1e-9,)
    }
};
        var_w_f = assign3940_e3867;

        let assign3950_e3871: f64 = (0.5 * var_delwod);
        let assign3950_e3872: f64 = (var_xgw_i - assign3950_e3871);
        let (assign3950_e3881,) = {
    if (assign3950_e3872 > 1e-9) {
        let assign3950_e3878: f64 = (0.5 * var_delwod);
        let assign3950_e3879: f64 = (var_xgw_i - assign3950_e3878);
        (assign3950_e3879,)
    } else {
        (1e-9,)
    }
};
        var_xgwe = assign3950_e3881;

        var_vfb_p = p.p57;

        var_stvfb_p = p.p58;

        var_st2vfb_p = p.p59;

        var_tox_p = p.p60;

        var_epsrox_p = p.p61;

        var_neff_p = p.p62;

        var_gfacnud_p = p.p63;

        var_vsbnud_p = p.p64;

        var_dvsbnud_p = p.p65;

        var_dphib_p = p.p66;

        var_np_p = p.p67;

        var_toxov_p = p.p68;

        var_toxovd_p = p.p69;

        var_nov_p = p.p70;

        var_novd_p = p.p71;

        var_ct_p = p.p72;

        var_ctg_p = p.p74;

        var_ctb_p = p.p73;

        var_stct_p = p.p75;

        var_psce_p = p.p79;

        var_psced_p = p.p81;

        var_psceb_p = p.p80;

        var_cf_p = p.p76;

        var_cfd_p = p.p78;

        var_cfb_p = p.p77;

        var_betn_p = p.p82;

        var_stbet_p = p.p83;

        var_mue_p = p.p84;

        var_stmue_p = p.p85;

        var_themu_p = p.p86;

        var_stthemu_p = p.p87;

        var_cs_p = p.p88;

        var_stcs_p = p.p89;

        var_thecs_p = p.p90;

        var_stthecs_p = p.p91;

        var_xcor_p = p.p92;

        var_stxcor_p = p.p93;

        var_feta_p = p.p94;

        var_rs_p = p.p95;

        var_strs_p = p.p96;

        var_rsb_p = p.p97;

        var_rsg_p = p.p98;

        var_thesat_p = p.p99;

        var_stthesat_p = p.p100;

        var_thesatb_p = p.p101;

        var_thesatg_p = p.p102;

        var_thesatt_p = p.p103;

        var_ax_p = p.p104;

        var_alp_p = p.p105;

        var_alp1_p = p.p106;

        var_alp2_p = p.p107;

        var_vp_p = p.p108;

        var_a1_p = p.p109;

        var_a2_p = p.p110;

        var_sta2_p = p.p111;

        var_a3_p = p.p112;

        var_a4_p = p.p113;

        var_imaxii_p = p.p114;

        var_gco_p = p.p115;

        var_iginv_p = p.p116;

        var_igov_p = p.p117;

        var_igovd_p = p.p118;

        var_stig_p = p.p119;

        var_gc2_p = p.p120;

        var_gc3_p = p.p121;

        var_gc2ov_p = p.p120;

        let assign4620_e3949: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4620_e3951: f64 = if assign4620_e3949 == 1.0 { 1.0 } else { 0.0 };
        var_guard35 = assign4620_e3951;

        let (assign4630_e3955,) = {
    if (var_guard35 != 0.0) {
        (p.p122,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign4630_e3955;

        var_gc3ov_p = p.p121;

        let assign4650_e3958: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4650_e3960: f64 = if assign4650_e3958 == 1.0 { 1.0 } else { 0.0 };
        var_guard36 = assign4650_e3960;

        let (assign4660_e3964,) = {
    if (var_guard36 != 0.0) {
        (p.p123,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign4660_e3964;

        var_gc2ovd_p = var_gc2ov_p;

        let assign4680_e3967: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4680_e3969: f64 = if assign4680_e3967 == 1.0 { 1.0 } else { 0.0 };
        var_guard37 = assign4680_e3969;

        let (assign4690_e3973,) = {
    if (var_guard37 != 0.0) {
        (p.p124,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign4690_e3973;

        var_gc3ovd_p = var_gc3ov_p;

        let assign4710_e3976: f64 = if param_given[125] { 1.0 } else { 0.0 };
        let assign4710_e3978: f64 = if assign4710_e3976 == 1.0 { 1.0 } else { 0.0 };
        var_guard38 = assign4710_e3978;

        let (assign4720_e3982,) = {
    if (var_guard38 != 0.0) {
        (p.p125,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign4720_e3982;

        var_chib_p = p.p126;

        var_agidl_p = p.p127;

        var_agidld_p = p.p128;

        var_bgidl_p = p.p129;

        var_bgidld_p = p.p130;

        var_stbgidl_p = p.p131;

        var_stbgidld_p = p.p132;

        var_cgidl_p = p.p133;

        var_cgidld_p = p.p134;

        var_cox_p = p.p135;

        var_delvtac_p = p.p136;

        var_facneffac_p = p.p137;

        var_thesatac_p = p.p99;

        let assign4860_e3997: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4860_e3999: f64 = if assign4860_e3997 == 1.0 { 1.0 } else { 0.0 };
        var_guard39 = assign4860_e3999;

        let (assign4870_e4003,) = {
    if (var_guard39 != 0.0) {
        (p.p138,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign4870_e4003;

        var_axac_p = p.p104;

        let assign4890_e4006: f64 = if param_given[139] { 1.0 } else { 0.0 };
        let assign4890_e4008: f64 = if assign4890_e4006 == 1.0 { 1.0 } else { 0.0 };
        var_guard40 = assign4890_e4008;

        let (assign4900_e4012,) = {
    if (var_guard40 != 0.0) {
        (p.p139,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4900_e4012;

        var_alpac_p = p.p140;

        var_alp1ac_p = p.p141;

        var_cgov_p = p.p142;

        var_cgovd_p = p.p143;

        var_fcgovacc_p = p.p144;

        var_fcgovaccd_p = p.p145;

        var_cgovaccg_p = p.p146;

        var_cgbov_p = p.p147;

        var_cinr_p = p.p148;

        var_cinrd_p = p.p149;

        var_dvfbinr_p = p.p150;

        var_fcinrdep_p = p.p151;

        var_fcinracc_p = p.p152;

        var_axinr_p = p.p153;

        var_cfr_p = p.p154;

        var_cfrd_p = p.p155;

        var_fnt_p = p.p156;

        var_fntexc_p = p.p157;

        var_vfbedge_p = p.p162;

        var_stvfbedge_p = p.p163;

        var_dphibedge_p = p.p164;

        var_neffedge_p = p.p165;

        var_ctedge_p = p.p166;

        var_betnedge_p = p.p167;

        var_stbetedge_p = p.p168;

        var_psceedge_p = p.p169;

        var_pscebedge_p = p.p170;

        var_pscededge_p = p.p171;

        var_cfedge_p = p.p172;

        var_cfdedge_p = p.p174;

        var_cfbedge_p = p.p173;

        var_rg_p = p.p180;

        var_rse_p = p.p181;

        var_rde_p = p.p182;

        var_rwell_p = p.p184;

        var_rbulk_p = p.p183;

        var_rjuns_p = p.p185;

        var_rjund_p = p.p186;

        var_munqs_p = p.p187;

        let assign5390_e4063: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard41 = assign5390_e4063;

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
        *var_cox_p_slot = var_cox_p;
        *var_cs_p_slot = var_cs_p;
        *var_ct_p_slot = var_ct_p;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctg_p_slot = var_ctg_p;
        *var_dellps_slot = var_dellps;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delwod_slot = var_delwod;
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
        *var_guard35_slot = var_guard35;
        *var_guard36_slot = var_guard36;
        *var_guard37_slot = var_guard37;
        *var_guard38_slot = var_guard38;
        *var_guard39_slot = var_guard39;
        *var_guard40_slot = var_guard40;
        *var_guard41_slot = var_guard41;
        *var_iae_slot = var_iae;
        *var_iginv_p_slot = var_iginv_p;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_iiae_slot = var_iiae;
        *var_iilcv_slot = var_iilcv;
        *var_iiwcv_slot = var_iiwcv;
        *var_iiwe_slot = var_iiwe;
        *var_iiwecv_slot = var_iiwecv;
        *var_ile_slot = var_ile;
        *var_ile2_slot = var_ile2;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_iwe_slot = var_iwe;
        *var_l_f_slot = var_l_f;
        *var_l_slif_slot = var_l_slif;
        *var_lcv_slot = var_lcv;
        *var_le_slot = var_le;
        *var_lecv_slot = var_lecv;
        *var_mue_p_slot = var_mue_p;
        *var_munqs_p_slot = var_munqs_p;
        *var_neff_p_slot = var_neff_p;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
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
        *var_we_slot = var_we;
        *var_wecv_slot = var_wecv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xgwe_slot = var_xgwe;
    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        var_guard41: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_aa_slot: &mut f64,
        var_bb_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_fbet1e_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_lp1e_slot: &mut f64,
        var_lpcke_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_npcke_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub0e_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
    ) {
        let mut var_aa: f64 = *var_aa_slot;
        let mut var_bb: f64 = *var_bb_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_fbet1e: f64 = *var_fbet1e_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_lp1e: f64 = *var_lp1e_slot;
        let mut var_lpcke: f64 = *var_lpcke_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_npcke: f64 = *var_npcke_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub0e: f64 = *var_nsub0e_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;

        let (assign5400_e4081,) = {
    if (var_guard41 != 0.0) {
        let assign5400_e4069: f64 = (var_ile).powf(p.p200);
        let assign5400_e4070: f64 = (p.p199 * assign5400_e4069);
        let assign5400_e4071: f64 = (p.p198 + assign5400_e4070);
        let assign5400_e4074: f64 = (p.p201 * var_iwe);
        let assign5400_e4075: f64 = (assign5400_e4071 + assign5400_e4074);
        let assign5400_e4078: f64 = (p.p202 * var_iae);
        let assign5400_e4079: f64 = (assign5400_e4075 + assign5400_e4078);
        (assign5400_e4079,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign5400_e4081;

        let (assign5410_e4097,) = {
    if (var_guard41 != 0.0) {
        let assign5410_e4086: f64 = (p.p204 * var_ile);
        let assign5410_e4087: f64 = (p.p203 + assign5410_e4086);
        let assign5410_e4090: f64 = (p.p205 * var_iwe);
        let assign5410_e4091: f64 = (assign5410_e4087 + assign5410_e4090);
        let assign5410_e4094: f64 = (p.p206 * var_iae);
        let assign5410_e4095: f64 = (assign5410_e4091 + assign5410_e4094);
        (assign5410_e4095,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign5410_e4097;

        let (assign5420_e4101,) = {
    if (var_guard41 != 0.0) {
        (p.p207,)
    } else {
        (var_st2vfb_p,)
    }
};
        var_st2vfb_p = assign5420_e4101;

        let (assign5430_e4105,) = {
    if (var_guard41 != 0.0) {
        (p.p208,)
    } else {
        (var_tox_p,)
    }
};
        var_tox_p = assign5430_e4105;

        let (assign5440_e4109,) = {
    if (var_guard41 != 0.0) {
        (p.p209,)
    } else {
        (var_epsrox_p,)
    }
};
        var_epsrox_p = assign5440_e4109;

        let (assign5450_e4142,) = {
    if (var_guard41 != 0.0) {
        let assign5450_e4115: f64 = (p.p211 * var_iwe);
        let assign5450_e4119: f64 = (var_we / p.p212);
        let assign5450_e4120: f64 = (1.0 + assign5450_e4119);
        let assign5450_e4121: f64 = (assign5450_e4120).ln();
        let assign5450_e4122: f64 = (assign5450_e4115 * assign5450_e4121);
        let assign5450_e4123: f64 = (1.0 + assign5450_e4122);
        let (assign5450_e4139,) = {
            if (assign5450_e4123 > 0.001) {
                let assign5450_e4129: f64 = (p.p211 * var_iwe);
                let assign5450_e4133: f64 = (var_we / p.p212);
                let assign5450_e4134: f64 = (1.0 + assign5450_e4133);
                let assign5450_e4135: f64 = (assign5450_e4134).ln();
                let assign5450_e4136: f64 = (assign5450_e4129 * assign5450_e4135);
                let assign5450_e4137: f64 = (1.0 + assign5450_e4136);
                (assign5450_e4137,)
            } else {
                (0.001,)
            }
        };
        let assign5450_e4140: f64 = (p.p210 * assign5450_e4139);
        (assign5450_e4140,)
    } else {
        (var_nsub0e,)
    }
};
        var_nsub0e = assign5450_e4142;

        let (assign5460_e4175,) = {
    if (var_guard41 != 0.0) {
        let assign5460_e4148: f64 = (p.p214 * var_iwe);
        let assign5460_e4152: f64 = (var_we / p.p215);
        let assign5460_e4153: f64 = (1.0 + assign5460_e4152);
        let assign5460_e4154: f64 = (assign5460_e4153).ln();
        let assign5460_e4155: f64 = (assign5460_e4148 * assign5460_e4154);
        let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
        let (assign5460_e4172,) = {
            if (assign5460_e4156 > 0.001) {
                let assign5460_e4162: f64 = (p.p214 * var_iwe);
                let assign5460_e4166: f64 = (var_we / p.p215);
                let assign5460_e4167: f64 = (1.0 + assign5460_e4166);
                let assign5460_e4168: f64 = (assign5460_e4167).ln();
                let assign5460_e4169: f64 = (assign5460_e4162 * assign5460_e4168);
                let assign5460_e4170: f64 = (1.0 + assign5460_e4169);
                (assign5460_e4170,)
            } else {
                (0.001,)
            }
        };
        let assign5460_e4173: f64 = (p.p213 * assign5460_e4172);
        (assign5460_e4173,)
    } else {
        (var_npcke,)
    }
};
        var_npcke = assign5460_e4175;

        let (assign5470_e4208,) = {
    if (var_guard41 != 0.0) {
        let assign5470_e4181: f64 = (p.p217 * var_iwe);
        let assign5470_e4185: f64 = (var_we / p.p215);
        let assign5470_e4186: f64 = (1.0 + assign5470_e4185);
        let assign5470_e4187: f64 = (assign5470_e4186).ln();
        let assign5470_e4188: f64 = (assign5470_e4181 * assign5470_e4187);
        let assign5470_e4189: f64 = (1.0 + assign5470_e4188);
        let (assign5470_e4205,) = {
            if (assign5470_e4189 > 0.001) {
                let assign5470_e4195: f64 = (p.p217 * var_iwe);
                let assign5470_e4199: f64 = (var_we / p.p215);
                let assign5470_e4200: f64 = (1.0 + assign5470_e4199);
                let assign5470_e4201: f64 = (assign5470_e4200).ln();
                let assign5470_e4202: f64 = (assign5470_e4195 * assign5470_e4201);
                let assign5470_e4203: f64 = (1.0 + assign5470_e4202);
                (assign5470_e4203,)
            } else {
                (0.001,)
            }
        };
        let assign5470_e4206: f64 = (p.p216 * assign5470_e4205);
        (assign5470_e4206,)
    } else {
        (var_lpcke,)
    }
};
        var_lpcke = assign5470_e4208;

        let assign5480_e4212: f64 = (2.0 * var_lpcke);
        let assign5480_e4213: f64 = if var_le > assign5480_e4212 { 1.0 } else { 0.0 };
        var_guard42 = assign5480_e4213;

        let (assign5490_e4219,) = {
    if ((var_guard41 != 0.0) && (var_guard42 != 0.0)) {
        (75000000000.0,)
    } else {
        (var_aa,)
    }
};
        var_aa = assign5490_e4219;

        let (assign5500_e4233,) = {
    if ((var_guard41 != 0.0) && (var_guard42 != 0.0)) {
        let assign5500_e4226: f64 = (0.5 * var_npcke);
        let assign5500_e4227: f64 = (var_nsub0e + assign5500_e4226);
        let assign5500_e4228: f64 = (assign5500_e4227).sqrt();
        let assign5500_e4230: f64 = (var_nsub0e).sqrt();
        let assign5500_e4231: f64 = (assign5500_e4228 - assign5500_e4230);
        (assign5500_e4231,)
    } else {
        (var_bb,)
    }
};
        var_bb = assign5500_e4233;

        let (assign5510_e4258,) = {
    if ((var_guard41 != 0.0) && (var_guard42 != 0.0)) {
        let assign5510_e4238: f64 = (var_nsub0e).sqrt();
        let assign5510_e4243: f64 = (2.0 * var_lpcke);
        let assign5510_e4245: f64 = (assign5510_e4243 / var_le);
        let assign5510_e4248: f64 = (var_bb / var_aa);
        let assign5510_e4249: f64 = (assign5510_e4248).exp();
        let assign5510_e4251: f64 = (assign5510_e4249 - 1.0);
        let assign5510_e4252: f64 = (assign5510_e4245 * assign5510_e4251);
        let assign5510_e4253: f64 = (1.0 + assign5510_e4252);
        let assign5510_e4254: f64 = (assign5510_e4253).ln();
        let assign5510_e4255: f64 = (var_aa * assign5510_e4254);
        let assign5510_e4256: f64 = (assign5510_e4238 + assign5510_e4255);
        (assign5510_e4256,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5510_e4258;

        let (assign5520_e4266,) = {
    if ((var_guard41 != 0.0) && (var_guard42 != 0.0)) {
        let assign5520_e4264: f64 = (var_nsub * var_nsub);
        (assign5520_e4264,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5520_e4266;

        let assign5530_e4269: f64 = if var_le >= var_lpcke { 1.0 } else { 0.0 };
        var_guard43 = assign5530_e4269;

        let (assign5540_e4284,) = {
    if (((var_guard41 != 0.0) && (var_guard42 == 0.0)) && (var_guard43 != 0.0)) {
        let assign5540_e4279: f64 = (var_npcke * var_lpcke);
        let assign5540_e4281: f64 = (assign5540_e4279 / var_le);
        let assign5540_e4282: f64 = (var_nsub0e + assign5540_e4281);
        (assign5540_e4282,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5540_e4284;

        let (assign5550_e4302,) = {
    if (((var_guard41 != 0.0) && (var_guard42 == 0.0)) && (var_guard43 == 0.0)) {
        let assign5550_e4297: f64 = (var_le / var_lpcke);
        let assign5550_e4298: f64 = (2.0 - assign5550_e4297);
        let assign5550_e4299: f64 = (var_npcke * assign5550_e4298);
        let assign5550_e4300: f64 = (var_nsub0e + assign5550_e4299);
        (assign5550_e4300,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5550_e4302;

        let (assign5560_e4316,) = {
    if (var_guard41 != 0.0) {
        let assign5560_e4308: f64 = (p.p218 * var_ile);
        let assign5560_e4309: f64 = (1.0 - assign5560_e4308);
        let assign5560_e4312: f64 = (p.p219 * var_ile2);
        let assign5560_e4313: f64 = (assign5560_e4309 - assign5560_e4312);
        let assign5560_e4314: f64 = (var_nsub * assign5560_e4313);
        (assign5560_e4314,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign5560_e4316;

        let (assign5570_e4334,) = {
    if (var_guard41 != 0.0) {
        let assign5570_e4322: f64 = (var_ile).powf(p.p222);
        let assign5570_e4323: f64 = (p.p221 * assign5570_e4322);
        let assign5570_e4324: f64 = (p.p220 + assign5570_e4323);
        let assign5570_e4327: f64 = (p.p223 * var_iwe);
        let assign5570_e4328: f64 = (assign5570_e4324 + assign5570_e4327);
        let assign5570_e4331: f64 = (p.p224 * var_iae);
        let assign5570_e4332: f64 = (assign5570_e4328 + assign5570_e4331);
        (assign5570_e4332,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign5570_e4334;

        let (assign5580_e4338,) = {
    if (var_guard41 != 0.0) {
        (p.p225,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign5580_e4338;

        let (assign5590_e4342,) = {
    if (var_guard41 != 0.0) {
        (p.p226,)
    } else {
        (var_dvsbnud_p,)
    }
};
        var_dvsbnud_p = assign5590_e4342;

        let (assign5600_e4360,) = {
    if (var_guard41 != 0.0) {
        let assign5600_e4348: f64 = (var_ile).powf(p.p229);
        let assign5600_e4349: f64 = (p.p228 * assign5600_e4348);
        let assign5600_e4350: f64 = (p.p227 + assign5600_e4349);
        let assign5600_e4353: f64 = (p.p230 * var_iwe);
        let assign5600_e4354: f64 = (assign5600_e4350 + assign5600_e4353);
        let assign5600_e4357: f64 = (p.p231 * var_iae);
        let assign5600_e4358: f64 = (assign5600_e4354 + assign5600_e4357);
        (assign5600_e4358,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign5600_e4360;

        let (assign5610_e4379,) = {
    if (var_guard41 != 0.0) {
        let assign5610_e4367: f64 = (p.p233 * var_ile);
        let assign5610_e4368: f64 = (1.0 + assign5610_e4367);
        let (assign5610_e4376,) = {
            if (1e-6 > assign5610_e4368) {
                (1e-6,)
            } else {
                let assign5610_e4374: f64 = (p.p233 * var_ile);
                let assign5610_e4375: f64 = (1.0 + assign5610_e4374);
                (assign5610_e4375,)
            }
        };
        let assign5610_e4377: f64 = (p.p232 * assign5610_e4376);
        (assign5610_e4377,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign5610_e4379;

        let (assign5620_e4383,) = {
    if (var_guard41 != 0.0) {
        (p.p234,)
    } else {
        (var_toxov_p,)
    }
};
        var_toxov_p = assign5620_e4383;

        let (assign5630_e4387,) = {
    if (var_guard41 != 0.0) {
        (p.p235,)
    } else {
        (var_toxovd_p,)
    }
};
        var_toxovd_p = assign5630_e4387;

        let (assign5640_e4391,) = {
    if (var_guard41 != 0.0) {
        (p.p238,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign5640_e4391;

        let (assign5650_e4395,) = {
    if (var_guard41 != 0.0) {
        (p.p239,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign5650_e4395;

        let (assign5660_e4417,) = {
    if (var_guard41 != 0.0) {
        let assign5660_e4401: f64 = (var_ile).powf(p.p242);
        let assign5660_e4402: f64 = (p.p241 * assign5660_e4401);
        let assign5660_e4403: f64 = (p.p240 + assign5660_e4402);
        let assign5660_e4407: f64 = (p.p243 * var_iwe);
        let assign5660_e4408: f64 = (1.0 + assign5660_e4407);
        let assign5660_e4409: f64 = (assign5660_e4403 * assign5660_e4408);
        let assign5660_e4413: f64 = (p.p244 * var_iae);
        let assign5660_e4414: f64 = (1.0 + assign5660_e4413);
        let assign5660_e4415: f64 = (assign5660_e4409 * assign5660_e4414);
        (assign5660_e4415,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign5660_e4417;

        let (assign5670_e4421,) = {
    if (var_guard41 != 0.0) {
        (p.p246,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign5670_e4421;

        let (assign5680_e4425,) = {
    if (var_guard41 != 0.0) {
        (p.p245,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign5680_e4425;

        let (assign5690_e4429,) = {
    if (var_guard41 != 0.0) {
        (p.p247,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign5690_e4429;

        let (assign5700_e4443,) = {
    if (var_guard41 != 0.0) {
        let assign5700_e4434: f64 = (var_ile).powf(p.p249);
        let assign5700_e4435: f64 = (p.p248 * assign5700_e4434);
        let assign5700_e4439: f64 = (p.p250 * var_iwe);
        let assign5700_e4440: f64 = (1.0 + assign5700_e4439);
        let assign5700_e4441: f64 = (assign5700_e4435 * assign5700_e4440);
        (assign5700_e4441,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign5700_e4443;

        let (assign5710_e4447,) = {
    if (var_guard41 != 0.0) {
        (p.p252,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign5710_e4447;

        let (assign5720_e4451,) = {
    if (var_guard41 != 0.0) {
        (p.p251,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign5720_e4451;

        let (assign5730_e4465,) = {
    if (var_guard41 != 0.0) {
        let assign5730_e4456: f64 = (var_ile).powf(p.p254);
        let assign5730_e4457: f64 = (p.p253 * assign5730_e4456);
        let assign5730_e4461: f64 = (p.p255 * var_iwe);
        let assign5730_e4462: f64 = (1.0 + assign5730_e4461);
        let assign5730_e4463: f64 = (assign5730_e4457 * assign5730_e4462);
        (assign5730_e4463,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign5730_e4465;

        let (assign5740_e4469,) = {
    if (var_guard41 != 0.0) {
        (p.p257,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign5740_e4469;

        let (assign5750_e4473,) = {
    if (var_guard41 != 0.0) {
        (p.p256,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign5750_e4473;

        let (assign5760_e4483,) = {
    if (var_guard41 != 0.0) {
        let assign5760_e4479: f64 = (p.p260 * var_iwe);
        let assign5760_e4480: f64 = (1.0 + assign5760_e4479);
        let assign5760_e4481: f64 = (p.p259 * assign5760_e4480);
        (assign5760_e4481,)
    } else {
        (var_fbet1e,)
    }
};
        var_fbet1e = assign5760_e4483;

        let (assign5770_e4502,) = {
    if (var_guard41 != 0.0) {
        let assign5770_e4489: f64 = (p.p262 * var_iwe);
        let assign5770_e4490: f64 = (1.0 + assign5770_e4489);
        let (assign5770_e4499,) = {
            if (assign5770_e4490 > 0.001) {
                let assign5770_e4496: f64 = (p.p262 * var_iwe);
                let assign5770_e4497: f64 = (1.0 + assign5770_e4496);
                (assign5770_e4497,)
            } else {
                (0.001,)
            }
        };
        let assign5770_e4500: f64 = (p.p261 * assign5770_e4499);
        (assign5770_e4500,)
    } else {
        (var_lp1e,)
    }
};
        var_lp1e = assign5770_e4502;

        *var_aa_slot = var_aa;
        *var_bb_slot = var_bb;
        *var_cf_p_slot = var_cf_p;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfd_p_slot = var_cfd_p;
        *var_ct_p_slot = var_ct_p;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctg_p_slot = var_ctg_p;
        *var_dphib_p_slot = var_dphib_p;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_fbet1e_slot = var_fbet1e;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_guard42_slot = var_guard42;
        *var_guard43_slot = var_guard43;
        *var_lp1e_slot = var_lp1e;
        *var_lpcke_slot = var_lpcke;
        *var_neff_p_slot = var_neff_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
        *var_npcke_slot = var_npcke;
        *var_nsub_slot = var_nsub;
        *var_nsub0e_slot = var_nsub0e;
        *var_psce_p_slot = var_psce_p;
        *var_psceb_p_slot = var_psceb_p;
        *var_psced_p_slot = var_psced_p;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_stct_p_slot = var_stct_p;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_tox_p_slot = var_tox_p;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_vfb_p_slot = var_vfb_p;
        *var_vsbnud_p_slot = var_vsbnud_p;
    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        var_fbet1e: f64,
        var_guard41: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_lp1e: f64,
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
        var_feta_p_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_gwe_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
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
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
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

        let (assign5780_e4534,) = {
    if (var_guard41 != 0.0) {
        let assign5780_e4507: f64 = (var_fbet1e * var_lp1e);
        let assign5780_e4509: f64 = (assign5780_e4507 / var_le);
        let assign5780_e4512: f64 = (-var_le);
        let assign5780_e4514: f64 = (assign5780_e4512 / var_lp1e);
        let assign5780_e4515: f64 = (assign5780_e4514).exp();
        let assign5780_e4516: f64 = (1.0 - assign5780_e4515);
        let assign5780_e4517: f64 = (assign5780_e4509 * assign5780_e4516);
        let assign5780_e4518: f64 = (1.0 + assign5780_e4517);
        let assign5780_e4521: f64 = (p.p263 * p.p264);
        let assign5780_e4523: f64 = (assign5780_e4521 / var_le);
        let assign5780_e4526: f64 = (-var_le);
        let assign5780_e4528: f64 = (assign5780_e4526 / p.p264);
        let assign5780_e4529: f64 = (assign5780_e4528).exp();
        let assign5780_e4530: f64 = (1.0 - assign5780_e4529);
        let assign5780_e4531: f64 = (assign5780_e4523 * assign5780_e4530);
        let assign5780_e4532: f64 = (assign5780_e4518 + assign5780_e4531);
        (assign5780_e4532,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5780_e4534;

        let (assign5790_e4543,) = {
    if (var_guard41 != 0.0) {
        let (assign5790_e4541,) = {
            if (var_gpe > 1e-15) {
                (var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5790_e4541,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5790_e4543;

        let (assign5800_e4562,) = {
    if (var_guard41 != 0.0) {
        let assign5800_e4548: f64 = (p.p265 * var_iwe);
        let assign5800_e4549: f64 = (1.0 + assign5800_e4548);
        let assign5800_e4552: f64 = (p.p266 * var_iwe);
        let assign5800_e4556: f64 = (var_we / p.p267);
        let assign5800_e4557: f64 = (1.0 + assign5800_e4556);
        let assign5800_e4558: f64 = (assign5800_e4557).ln();
        let assign5800_e4559: f64 = (assign5800_e4552 * assign5800_e4558);
        let assign5800_e4560: f64 = (assign5800_e4549 + assign5800_e4559);
        (assign5800_e4560,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign5800_e4562;

        let (assign5810_e4574,) = {
    if (var_guard41 != 0.0) {
        let assign5810_e4566: f64 = (p.p258 * var_we);
        let assign5810_e4569: f64 = (var_gpe * var_le);
        let assign5810_e4570: f64 = (assign5810_e4566 / assign5810_e4569);
        let assign5810_e4572: f64 = (assign5810_e4570 * var_gwe);
        (assign5810_e4572,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign5810_e4574;

        let (assign5820_e4590,) = {
    if (var_guard41 != 0.0) {
        let assign5820_e4579: f64 = (p.p269 * var_ile);
        let assign5820_e4580: f64 = (p.p268 + assign5820_e4579);
        let assign5820_e4583: f64 = (p.p270 * var_iwe);
        let assign5820_e4584: f64 = (assign5820_e4580 + assign5820_e4583);
        let assign5820_e4587: f64 = (p.p271 * var_iae);
        let assign5820_e4588: f64 = (assign5820_e4584 + assign5820_e4587);
        (assign5820_e4588,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign5820_e4590;

        let (assign5830_e4600,) = {
    if (var_guard41 != 0.0) {
        let assign5830_e4596: f64 = (p.p273 * var_iwe);
        let assign5830_e4597: f64 = (1.0 + assign5830_e4596);
        let assign5830_e4598: f64 = (p.p272 * assign5830_e4597);
        (assign5830_e4598,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign5830_e4600;

        let (assign5840_e4604,) = {
    if (var_guard41 != 0.0) {
        (p.p274,)
    } else {
        (var_stmue_p,)
    }
};
        var_stmue_p = assign5840_e4604;

        let (assign5850_e4608,) = {
    if (var_guard41 != 0.0) {
        (p.p275,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign5850_e4608;

        let (assign5860_e4612,) = {
    if (var_guard41 != 0.0) {
        (p.p276,)
    } else {
        (var_stthemu_p,)
    }
};
        var_stthemu_p = assign5860_e4612;

        let (assign5870_e4634,) = {
    if (var_guard41 != 0.0) {
        let assign5870_e4618: f64 = (var_ile).powf(p.p279);
        let assign5870_e4619: f64 = (p.p278 * assign5870_e4618);
        let assign5870_e4620: f64 = (p.p277 + assign5870_e4619);
        let assign5870_e4624: f64 = (p.p280 * var_iwe);
        let assign5870_e4625: f64 = (1.0 + assign5870_e4624);
        let assign5870_e4626: f64 = (assign5870_e4620 * assign5870_e4625);
        let assign5870_e4630: f64 = (p.p281 * var_iae);
        let assign5870_e4631: f64 = (1.0 + assign5870_e4630);
        let assign5870_e4632: f64 = (assign5870_e4626 * assign5870_e4631);
        (assign5870_e4632,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign5870_e4634;

        let (assign5880_e4638,) = {
    if (var_guard41 != 0.0) {
        (p.p282,)
    } else {
        (var_stcs_p,)
    }
};
        var_stcs_p = assign5880_e4638;

        let (assign5890_e4642,) = {
    if (var_guard41 != 0.0) {
        (p.p283,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign5890_e4642;

        let (assign5900_e4646,) = {
    if (var_guard41 != 0.0) {
        (p.p284,)
    } else {
        (var_stthecs_p,)
    }
};
        var_stthecs_p = assign5900_e4646;

        let (assign5910_e4668,) = {
    if (var_guard41 != 0.0) {
        let assign5910_e4652: f64 = (p.p286 * var_ile);
        let assign5910_e4653: f64 = (1.0 + assign5910_e4652);
        let assign5910_e4654: f64 = (p.p285 * assign5910_e4653);
        let assign5910_e4658: f64 = (p.p287 * var_iwe);
        let assign5910_e4659: f64 = (1.0 + assign5910_e4658);
        let assign5910_e4660: f64 = (assign5910_e4654 * assign5910_e4659);
        let assign5910_e4664: f64 = (p.p288 * var_iae);
        let assign5910_e4665: f64 = (1.0 + assign5910_e4664);
        let assign5910_e4666: f64 = (assign5910_e4660 * assign5910_e4665);
        (assign5910_e4666,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign5910_e4668;

        let (assign5920_e4672,) = {
    if (var_guard41 != 0.0) {
        (p.p289,)
    } else {
        (var_stxcor_p,)
    }
};
        var_stxcor_p = assign5920_e4672;

        let (assign5930_e4676,) = {
    if (var_guard41 != 0.0) {
        (p.p290,)
    } else {
        (var_feta_p,)
    }
};
        var_feta_p = assign5930_e4676;

        let (assign5940_e4688,) = {
    if (var_guard41 != 0.0) {
        let assign5940_e4680: f64 = (p.p291 * var_iwe);
        let assign5940_e4684: f64 = (p.p292 * var_iwe);
        let assign5940_e4685: f64 = (1.0 + assign5940_e4684);
        let assign5940_e4686: f64 = (assign5940_e4680 * assign5940_e4685);
        (assign5940_e4686,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign5940_e4688;

        let (assign5950_e4692,) = {
    if (var_guard41 != 0.0) {
        (p.p293,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign5950_e4692;

        let (assign5960_e4696,) = {
    if (var_guard41 != 0.0) {
        (p.p294,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign5960_e4696;

        let (assign5970_e4700,) = {
    if (var_guard41 != 0.0) {
        (p.p295,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign5970_e4700;

        let (assign5980_e4726,) = {
    if (var_guard41 != 0.0) {
        let assign5980_e4705: f64 = (p.p297 * var_gwe);
        let assign5980_e4707: f64 = (assign5980_e4705 / var_gpe);
        let assign5980_e4710: f64 = (var_ile).powf(p.p298);
        let assign5980_e4711: f64 = (assign5980_e4707 * assign5980_e4710);
        let assign5980_e4712: f64 = (p.p296 + assign5980_e4711);
        let assign5980_e4716: f64 = (p.p299 * var_iwe);
        let assign5980_e4717: f64 = (1.0 + assign5980_e4716);
        let assign5980_e4718: f64 = (assign5980_e4712 * assign5980_e4717);
        let assign5980_e4722: f64 = (p.p300 * var_iae);
        let assign5980_e4723: f64 = (1.0 + assign5980_e4722);
        let assign5980_e4724: f64 = (assign5980_e4718 * assign5980_e4723);
        (assign5980_e4724,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign5980_e4726;

        let (assign5990_e4742,) = {
    if (var_guard41 != 0.0) {
        let assign5990_e4731: f64 = (p.p302 * var_ile);
        let assign5990_e4732: f64 = (p.p301 + assign5990_e4731);
        let assign5990_e4735: f64 = (p.p303 * var_iwe);
        let assign5990_e4736: f64 = (assign5990_e4732 + assign5990_e4735);
        let assign5990_e4739: f64 = (p.p304 * var_iae);
        let assign5990_e4740: f64 = (assign5990_e4736 + assign5990_e4739);
        (assign5990_e4740,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign5990_e4742;

        let (assign6000_e4746,) = {
    if (var_guard41 != 0.0) {
        (p.p305,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign6000_e4746;

        let (assign6010_e4750,) = {
    if (var_guard41 != 0.0) {
        (p.p306,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign6010_e4750;

        let (assign6020_e4754,) = {
    if (var_guard41 != 0.0) {
        (p.p307,)
    } else {
        (var_thesatt_p,)
    }
};
        var_thesatt_p = assign6020_e4754;

        let (assign6030_e4764,) = {
    if (var_guard41 != 0.0) {
        let assign6030_e4760: f64 = (p.p309 * var_ile);
        let assign6030_e4761: f64 = (1.0 + assign6030_e4760);
        let assign6030_e4762: f64 = (p.p308 / assign6030_e4761);
        (assign6030_e4762,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign6030_e4764;

        let (assign6040_e4778,) = {
    if (var_guard41 != 0.0) {
        let assign6040_e4769: f64 = (var_ile).powf(p.p311);
        let assign6040_e4770: f64 = (p.p310 * assign6040_e4769);
        let assign6040_e4774: f64 = (p.p312 * var_iwe);
        let assign6040_e4775: f64 = (1.0 + assign6040_e4774);
        let assign6040_e4776: f64 = (assign6040_e4770 * assign6040_e4775);
        (assign6040_e4776,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign6040_e4778;

        let (assign6050_e4784,) = {
    if (var_guard41 != 0.0) {
        let assign6050_e4782: f64 = (var_ile).powf(p.p314);
        (assign6050_e4782,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6050_e4784;

        let (assign6060_e4804,) = {
    if (var_guard41 != 0.0) {
        let assign6060_e4788: f64 = (p.p313 * var_tmpx);
        let assign6060_e4792: f64 = (p.p316 * var_iwe);
        let assign6060_e4793: f64 = (1.0 + assign6060_e4792);
        let assign6060_e4794: f64 = (assign6060_e4788 * assign6060_e4793);
        let assign6060_e4798: f64 = (p.p315 * var_ile);
        let assign6060_e4800: f64 = (assign6060_e4798 * var_tmpx);
        let assign6060_e4801: f64 = (1.0 + assign6060_e4800);
        let assign6060_e4802: f64 = (assign6060_e4794 / assign6060_e4801);
        (assign6060_e4802,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign6060_e4804;

        let (assign6070_e4810,) = {
    if (var_guard41 != 0.0) {
        let assign6070_e4808: f64 = (var_ile).powf(p.p318);
        (assign6070_e4808,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6070_e4810;

        let (assign6080_e4830,) = {
    if (var_guard41 != 0.0) {
        let assign6080_e4814: f64 = (p.p317 * var_tmpx);
        let assign6080_e4818: f64 = (p.p320 * var_iwe);
        let assign6080_e4819: f64 = (1.0 + assign6080_e4818);
        let assign6080_e4820: f64 = (assign6080_e4814 * assign6080_e4819);
        let assign6080_e4824: f64 = (p.p319 * var_ile);
        let assign6080_e4826: f64 = (assign6080_e4824 * var_tmpx);
        let assign6080_e4827: f64 = (1.0 + assign6080_e4826);
        let assign6080_e4828: f64 = (assign6080_e4820 / assign6080_e4827);
        (assign6080_e4828,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign6080_e4830;

        let (assign6090_e4834,) = {
    if (var_guard41 != 0.0) {
        (p.p321,)
    } else {
        (var_vp_p,)
    }
};
        var_vp_p = assign6090_e4834;

        let (assign6100_e4850,) = {
    if (var_guard41 != 0.0) {
        let assign6100_e4840: f64 = (p.p323 * var_ile);
        let assign6100_e4841: f64 = (1.0 + assign6100_e4840);
        let assign6100_e4842: f64 = (p.p322 * assign6100_e4841);
        let assign6100_e4846: f64 = (p.p324 * var_iwe);
        let assign6100_e4847: f64 = (1.0 + assign6100_e4846);
        let assign6100_e4848: f64 = (assign6100_e4842 * assign6100_e4847);
        (assign6100_e4848,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign6100_e4850;

        let (assign6110_e4854,) = {
    if (var_guard41 != 0.0) {
        (p.p325,)
    } else {
        (var_a2_p,)
    }
};
        var_a2_p = assign6110_e4854;

        let (assign6120_e4858,) = {
    if (var_guard41 != 0.0) {
        (p.p326,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign6120_e4858;

        let (assign6130_e4874,) = {
    if (var_guard41 != 0.0) {
        let assign6130_e4864: f64 = (p.p328 * var_ile);
        let assign6130_e4865: f64 = (1.0 + assign6130_e4864);
        let assign6130_e4866: f64 = (p.p327 * assign6130_e4865);
        let assign6130_e4870: f64 = (p.p329 * var_iwe);
        let assign6130_e4871: f64 = (1.0 + assign6130_e4870);
        let assign6130_e4872: f64 = (assign6130_e4866 * assign6130_e4871);
        (assign6130_e4872,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign6130_e4874;

        let (assign6140_e4890,) = {
    if (var_guard41 != 0.0) {
        let assign6140_e4880: f64 = (p.p331 * var_ile);
        let assign6140_e4881: f64 = (1.0 + assign6140_e4880);
        let assign6140_e4882: f64 = (p.p330 * assign6140_e4881);
        let assign6140_e4886: f64 = (p.p332 * var_iwe);
        let assign6140_e4887: f64 = (1.0 + assign6140_e4886);
        let assign6140_e4888: f64 = (assign6140_e4882 * assign6140_e4887);
        (assign6140_e4888,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign6140_e4890;

        let (assign6150_e4894,) = {
    if (var_guard41 != 0.0) {
        (p.p333,)
    } else {
        (var_imaxii_p,)
    }
};
        var_imaxii_p = assign6150_e4894;

        let (assign6160_e4898,) = {
    if (var_guard41 != 0.0) {
        (p.p334,)
    } else {
        (var_gco_p,)
    }
};
        var_gco_p = assign6160_e4898;

        let (assign6170_e4904,) = {
    if (var_guard41 != 0.0) {
        let assign6170_e4902: f64 = (p.p335 / var_iae);
        (assign6170_e4902,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign6170_e4904;

        let (assign6180_e4914,) = {
    if (var_guard41 != 0.0) {
        let assign6180_e4908: f64 = (p.p336 * p.p236);
        let assign6180_e4911: f64 = (1e-6 * var_iwe);
        let assign6180_e4912: f64 = (assign6180_e4908 / assign6180_e4911);
        (assign6180_e4912,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign6180_e4914;

        let (assign6190_e4924,) = {
    if (var_guard41 != 0.0) {
        let assign6190_e4918: f64 = (p.p337 * p.p237);
        let assign6190_e4921: f64 = (1e-6 * var_iwe);
        let assign6190_e4922: f64 = (assign6190_e4918 / assign6190_e4921);
        (assign6190_e4922,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign6190_e4924;

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
        *var_feta_p_slot = var_feta_p;
        *var_gco_p_slot = var_gco_p;
        *var_gpe_slot = var_gpe;
        *var_gwe_slot = var_gwe;
        *var_iginv_p_slot = var_iginv_p;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_imaxii_p_slot = var_imaxii_p;
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
        var_guard41: f64,
        var_gwe: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_wecv: f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_alp1ac_p_slot: &mut f64,
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
        var_fcgovacc_p_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
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
        var_stbgidl_p_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
        var_tmpx_slot: &mut f64,
    ) {
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
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
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
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
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;

        let (assign6200_e4928,) = {
    if (var_guard41 != 0.0) {
        (p.p338,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign6200_e4928;

        let (assign6210_e4932,) = {
    if (var_guard41 != 0.0) {
        (p.p339,)
    } else {
        (var_gc2_p,)
    }
};
        var_gc2_p = assign6210_e4932;

        let (assign6220_e4936,) = {
    if (var_guard41 != 0.0) {
        (p.p340,)
    } else {
        (var_gc3_p,)
    }
};
        var_gc3_p = assign6220_e4936;

        let (assign6230_e4940,) = {
    if (var_guard41 != 0.0) {
        (p.p339,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6230_e4940;

        let assign6240_e4942: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6240_e4944: f64 = if assign6240_e4942 == 1.0 { 1.0 } else { 0.0 };
        var_guard44 = assign6240_e4944;

        let (assign6250_e4950,) = {
    if ((var_guard41 != 0.0) && (var_guard44 != 0.0)) {
        (p.p341,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6250_e4950;

        let (assign6260_e4954,) = {
    if (var_guard41 != 0.0) {
        (p.p340,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6260_e4954;

        let assign6270_e4956: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6270_e4958: f64 = if assign6270_e4956 == 1.0 { 1.0 } else { 0.0 };
        var_guard45 = assign6270_e4958;

        let (assign6280_e4964,) = {
    if ((var_guard41 != 0.0) && (var_guard45 != 0.0)) {
        (p.p342,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6280_e4964;

        let (assign6290_e4968,) = {
    if (var_guard41 != 0.0) {
        (var_gc2ov_p,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6290_e4968;

        let assign6300_e4970: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6300_e4972: f64 = if assign6300_e4970 == 1.0 { 1.0 } else { 0.0 };
        var_guard46 = assign6300_e4972;

        let (assign6310_e4978,) = {
    if ((var_guard41 != 0.0) && (var_guard46 != 0.0)) {
        (p.p343,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6310_e4978;

        let (assign6320_e4982,) = {
    if (var_guard41 != 0.0) {
        (var_gc3ov_p,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6320_e4982;

        let assign6330_e4984: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6330_e4986: f64 = if assign6330_e4984 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign6330_e4986;

        let (assign6340_e4992,) = {
    if ((var_guard41 != 0.0) && (var_guard47 != 0.0)) {
        (p.p344,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6340_e4992;

        let (assign6350_e4996,) = {
    if (var_guard41 != 0.0) {
        (p.p345,)
    } else {
        (var_chib_p,)
    }
};
        var_chib_p = assign6350_e4996;

        let (assign6360_e5006,) = {
    if (var_guard41 != 0.0) {
        let assign6360_e5000: f64 = (p.p346 * p.p236);
        let assign6360_e5003: f64 = (1e-6 * var_iwe);
        let assign6360_e5004: f64 = (assign6360_e5000 / assign6360_e5003);
        (assign6360_e5004,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign6360_e5006;

        let (assign6370_e5016,) = {
    if (var_guard41 != 0.0) {
        let assign6370_e5010: f64 = (p.p347 * p.p237);
        let assign6370_e5013: f64 = (1e-6 * var_iwe);
        let assign6370_e5014: f64 = (assign6370_e5010 / assign6370_e5013);
        (assign6370_e5014,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign6370_e5016;

        let (assign6380_e5020,) = {
    if (var_guard41 != 0.0) {
        (p.p348,)
    } else {
        (var_bgidl_p,)
    }
};
        var_bgidl_p = assign6380_e5020;

        let (assign6390_e5024,) = {
    if (var_guard41 != 0.0) {
        (p.p349,)
    } else {
        (var_bgidld_p,)
    }
};
        var_bgidld_p = assign6390_e5024;

        let (assign6400_e5028,) = {
    if (var_guard41 != 0.0) {
        (p.p350,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign6400_e5028;

        let (assign6410_e5032,) = {
    if (var_guard41 != 0.0) {
        (p.p351,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign6410_e5032;

        let (assign6420_e5036,) = {
    if (var_guard41 != 0.0) {
        (p.p352,)
    } else {
        (var_cgidl_p,)
    }
};
        var_cgidl_p = assign6420_e5036;

        let (assign6430_e5040,) = {
    if (var_guard41 != 0.0) {
        (p.p353,)
    } else {
        (var_cgidld_p,)
    }
};
        var_cgidld_p = assign6430_e5040;

        let (assign6440_e5052,) = {
    if (var_guard41 != 0.0) {
        let assign6440_e5044: f64 = (8.8541878176e-12 * p.p209);
        let assign6440_e5046: f64 = (assign6440_e5044 * var_wecv);
        let assign6440_e5048: f64 = (assign6440_e5046 * var_lecv);
        let assign6440_e5050: f64 = (assign6440_e5048 / p.p208);
        (assign6440_e5050,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign6440_e5052;

        let (assign6450_e5064,) = {
    if (var_guard41 != 0.0) {
        let assign6450_e5056: f64 = (8.8541878176e-12 * p.p209);
        let assign6450_e5058: f64 = (assign6450_e5056 * var_wecv);
        let assign6450_e5060: f64 = (assign6450_e5058 * p.p236);
        let assign6450_e5062: f64 = (assign6450_e5060 / p.p234);
        (assign6450_e5062,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign6450_e5064;

        let (assign6460_e5076,) = {
    if (var_guard41 != 0.0) {
        let assign6460_e5068: f64 = (8.8541878176e-12 * p.p209);
        let assign6460_e5070: f64 = (assign6460_e5068 * var_wecv);
        let assign6460_e5072: f64 = (assign6460_e5070 * p.p237);
        let assign6460_e5074: f64 = (assign6460_e5072 / p.p235);
        (assign6460_e5074,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign6460_e5076;

        let (assign6470_e5094,) = {
    if (var_guard41 != 0.0) {
        let assign6470_e5082: f64 = (var_ile).powf(p.p356);
        let assign6470_e5083: f64 = (p.p355 * assign6470_e5082);
        let assign6470_e5084: f64 = (p.p354 + assign6470_e5083);
        let assign6470_e5087: f64 = (p.p357 * var_iwe);
        let assign6470_e5088: f64 = (assign6470_e5084 + assign6470_e5087);
        let assign6470_e5091: f64 = (p.p358 * var_iae);
        let assign6470_e5092: f64 = (assign6470_e5088 + assign6470_e5091);
        (assign6470_e5092,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign6470_e5094;

        let (assign6480_e5110,) = {
    if (var_guard41 != 0.0) {
        let assign6480_e5099: f64 = (p.p360 * var_ile);
        let assign6480_e5100: f64 = (p.p359 + assign6480_e5099);
        let assign6480_e5103: f64 = (p.p361 * var_iwe);
        let assign6480_e5104: f64 = (assign6480_e5100 + assign6480_e5103);
        let assign6480_e5107: f64 = (p.p362 * var_iae);
        let assign6480_e5108: f64 = (assign6480_e5104 + assign6480_e5107);
        (assign6480_e5108,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign6480_e5110;

        let (assign6490_e5114,) = {
    if (var_guard41 != 0.0) {
        (p.p296,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6490_e5114;

        let assign6500_e5116: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6500_e5118: f64 = if assign6500_e5116 == 1.0 { 1.0 } else { 0.0 };
        var_guard48 = assign6500_e5118;

        let (assign6510_e5124,) = {
    if ((var_guard41 != 0.0) && (var_guard48 != 0.0)) {
        (p.p363,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6510_e5124;

        let (assign6520_e5128,) = {
    if (var_guard41 != 0.0) {
        (p.p297,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6520_e5128;

        let assign6530_e5130: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6530_e5132: f64 = if assign6530_e5130 == 1.0 { 1.0 } else { 0.0 };
        var_guard49 = assign6530_e5132;

        let (assign6540_e5138,) = {
    if ((var_guard41 != 0.0) && (var_guard49 != 0.0)) {
        (p.p364,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6540_e5138;

        let (assign6550_e5142,) = {
    if (var_guard41 != 0.0) {
        (p.p298,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6550_e5142;

        let assign6560_e5144: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6560_e5146: f64 = if assign6560_e5144 == 1.0 { 1.0 } else { 0.0 };
        var_guard50 = assign6560_e5146;

        let (assign6570_e5152,) = {
    if ((var_guard41 != 0.0) && (var_guard50 != 0.0)) {
        (p.p365,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6570_e5152;

        let (assign6580_e5156,) = {
    if (var_guard41 != 0.0) {
        (p.p299,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6580_e5156;

        let assign6590_e5158: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6590_e5160: f64 = if assign6590_e5158 == 1.0 { 1.0 } else { 0.0 };
        var_guard51 = assign6590_e5160;

        let (assign6600_e5166,) = {
    if ((var_guard41 != 0.0) && (var_guard51 != 0.0)) {
        (p.p366,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6600_e5166;

        let (assign6610_e5170,) = {
    if (var_guard41 != 0.0) {
        (p.p300,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6610_e5170;

        let assign6620_e5172: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6620_e5174: f64 = if assign6620_e5172 == 1.0 { 1.0 } else { 0.0 };
        var_guard52 = assign6620_e5174;

        let (assign6630_e5180,) = {
    if ((var_guard41 != 0.0) && (var_guard52 != 0.0)) {
        (p.p367,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6630_e5180;

        let (assign6640_e5206,) = {
    if (var_guard41 != 0.0) {
        let assign6640_e5185: f64 = (var_thesatacl_i * var_gwe);
        let assign6640_e5187: f64 = (assign6640_e5185 / var_gpe);
        let assign6640_e5190: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign6640_e5191: f64 = (assign6640_e5187 * assign6640_e5190);
        let assign6640_e5192: f64 = (var_thesataco_i + assign6640_e5191);
        let assign6640_e5196: f64 = (var_thesatacw_i * var_iwe);
        let assign6640_e5197: f64 = (1.0 + assign6640_e5196);
        let assign6640_e5198: f64 = (assign6640_e5192 * assign6640_e5197);
        let assign6640_e5202: f64 = (var_thesataclw_i * var_iae);
        let assign6640_e5203: f64 = (1.0 + assign6640_e5202);
        let assign6640_e5204: f64 = (assign6640_e5198 * assign6640_e5203);
        (assign6640_e5204,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign6640_e5206;

        let (assign6650_e5210,) = {
    if (var_guard41 != 0.0) {
        (p.p308,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6650_e5210;

        let assign6660_e5212: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6660_e5214: f64 = if assign6660_e5212 == 1.0 { 1.0 } else { 0.0 };
        var_guard53 = assign6660_e5214;

        let (assign6670_e5220,) = {
    if ((var_guard41 != 0.0) && (var_guard53 != 0.0)) {
        (p.p368,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6670_e5220;

        let (assign6680_e5224,) = {
    if (var_guard41 != 0.0) {
        (p.p309,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6680_e5224;

        let assign6690_e5226: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6690_e5228: f64 = if assign6690_e5226 == 1.0 { 1.0 } else { 0.0 };
        var_guard54 = assign6690_e5228;

        let (assign6700_e5234,) = {
    if ((var_guard41 != 0.0) && (var_guard54 != 0.0)) {
        (p.p369,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6700_e5234;

        let (assign6710_e5244,) = {
    if (var_guard41 != 0.0) {
        let assign6710_e5240: f64 = (var_axacl_i * var_ile);
        let assign6710_e5241: f64 = (1.0 + assign6710_e5240);
        let assign6710_e5242: f64 = (var_axaco_i / assign6710_e5241);
        (assign6710_e5242,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign6710_e5244;

        let (assign6720_e5258,) = {
    if (var_guard41 != 0.0) {
        let assign6720_e5249: f64 = (var_ile).powf(p.p371);
        let assign6720_e5250: f64 = (p.p370 * assign6720_e5249);
        let assign6720_e5254: f64 = (p.p372 * var_iwe);
        let assign6720_e5255: f64 = (1.0 + assign6720_e5254);
        let assign6720_e5256: f64 = (assign6720_e5250 * assign6720_e5255);
        (assign6720_e5256,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign6720_e5258;

        let (assign6730_e5264,) = {
    if (var_guard41 != 0.0) {
        let assign6730_e5262: f64 = (var_ile).powf(p.p374);
        (assign6730_e5262,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6730_e5264;

        let (assign6740_e5284,) = {
    if (var_guard41 != 0.0) {
        let assign6740_e5268: f64 = (p.p373 * var_tmpx);
        let assign6740_e5272: f64 = (p.p376 * var_iwe);
        let assign6740_e5273: f64 = (1.0 + assign6740_e5272);
        let assign6740_e5274: f64 = (assign6740_e5268 * assign6740_e5273);
        let assign6740_e5278: f64 = (p.p375 * var_ile);
        let assign6740_e5280: f64 = (assign6740_e5278 * var_tmpx);
        let assign6740_e5281: f64 = (1.0 + assign6740_e5280);
        let assign6740_e5282: f64 = (assign6740_e5274 / assign6740_e5281);
        (assign6740_e5282,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign6740_e5284;

        let (assign6750_e5288,) = {
    if (var_guard41 != 0.0) {
        (p.p377,)
    } else {
        (var_fcgovacc_p,)
    }
};
        var_fcgovacc_p = assign6750_e5288;

        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_alp1ac_p_slot = var_alp1ac_p;
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
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
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
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stig_p_slot = var_stig_p;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesatacw_i_slot = var_thesatacw_i;
        *var_tmpx_slot = var_tmpx;
    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        var_betn_p: f64,
        var_guard41: f64,
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
        var_fcgovaccd_p_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fntexc_p_slot: &mut f64,
        var_gpe_edge_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_kuowe_slot: &mut f64,
        var_kvthowe_slot: &mut f64,
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
        var_rwell_p_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_we_edge_slot: &mut f64,
    ) {
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
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fntexc_p: f64 = *var_fntexc_p_slot;
        let mut var_gpe_edge: f64 = *var_gpe_edge_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_kuowe: f64 = *var_kuowe_slot;
        let mut var_kvthowe: f64 = *var_kvthowe_slot;
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
        let mut var_rwell_p: f64 = *var_rwell_p_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;

        let (assign6760_e5292,) = {
    if (var_guard41 != 0.0) {
        (p.p378,)
    } else {
        (var_fcgovaccd_p,)
    }
};
        var_fcgovaccd_p = assign6760_e5292;

        let (assign6770_e5296,) = {
    if (var_guard41 != 0.0) {
        (p.p379,)
    } else {
        (var_cgovaccg_p,)
    }
};
        var_cgovaccg_p = assign6770_e5296;

        let (assign6780_e5302,) = {
    if (var_guard41 != 0.0) {
        let assign6780_e5300: f64 = (p.p380 * var_iilcv);
        (assign6780_e5300,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign6780_e5302;

        let (assign6790_e5308,) = {
    if (var_guard41 != 0.0) {
        let assign6790_e5306: f64 = (p.p381 * var_iiwecv);
        (assign6790_e5306,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign6790_e5308;

        let (assign6800_e5314,) = {
    if (var_guard41 != 0.0) {
        let assign6800_e5312: f64 = (p.p382 * var_iiwecv);
        (assign6800_e5312,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign6800_e5314;

        let (assign6810_e5318,) = {
    if (var_guard41 != 0.0) {
        (p.p383,)
    } else {
        (var_dvfbinr_p,)
    }
};
        var_dvfbinr_p = assign6810_e5318;

        let (assign6820_e5322,) = {
    if (var_guard41 != 0.0) {
        (p.p384,)
    } else {
        (var_fcinrdep_p,)
    }
};
        var_fcinrdep_p = assign6820_e5322;

        let (assign6830_e5326,) = {
    if (var_guard41 != 0.0) {
        (p.p385,)
    } else {
        (var_fcinracc_p,)
    }
};
        var_fcinracc_p = assign6830_e5326;

        let (assign6840_e5330,) = {
    if (var_guard41 != 0.0) {
        (p.p386,)
    } else {
        (var_axinr_p,)
    }
};
        var_axinr_p = assign6840_e5330;

        let (assign6850_e5336,) = {
    if (var_guard41 != 0.0) {
        let assign6850_e5334: f64 = (p.p387 * var_iiwcv);
        (assign6850_e5334,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign6850_e5336;

        let (assign6860_e5342,) = {
    if (var_guard41 != 0.0) {
        let assign6860_e5340: f64 = (p.p388 * var_iiwcv);
        (assign6860_e5340,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign6860_e5342;

        let (assign6870_e5352,) = {
    if (var_guard41 != 0.0) {
        let assign6870_e5347: f64 = (2.0 * p.p395);
        let assign6870_e5349: f64 = (assign6870_e5347 / var_le);
        let assign6870_e5350: f64 = (1.0 - assign6870_e5349);
        (assign6870_e5350,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign6870_e5352;

        let (assign6900_e5373,) = {
    if (var_guard41 != 0.0) {
        (p.p389,)
    } else {
        (var_fnt_p,)
    }
};
        var_fnt_p = assign6900_e5373;

        let (assign6910_e5385,) = {
    if (var_guard41 != 0.0) {
        let assign6910_e5377: f64 = (p.p390 * var_betn_p);
        let assign6910_e5379: f64 = (assign6910_e5377 * var_betn_p);
        let assign6910_e5381: f64 = (assign6910_e5379 * var_iwe);
        let assign6910_e5383: f64 = (assign6910_e5381 * var_iwe);
        (assign6910_e5383,)
    } else {
        (var_fntexc_p,)
    }
};
        var_fntexc_p = assign6910_e5385;

        let (assign6960_e5423,) = {
    if (var_guard41 != 0.0) {
        let assign6960_e5417: f64 = (2.0 * p.p397);
        let assign6960_e5420: f64 = (p.p398 * var_we);
        let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
        (assign6960_e5421,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign6960_e5423;

        let (assign6990_e5439,) = {
    if (var_guard41 != 0.0) {
        (p.p399,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign6990_e5439;

        let (assign7000_e5455,) = {
    if (var_guard41 != 0.0) {
        let assign7000_e5444: f64 = (p.p401 * var_ile);
        let assign7000_e5445: f64 = (p.p400 + assign7000_e5444);
        let assign7000_e5448: f64 = (p.p402 * var_iwe);
        let assign7000_e5449: f64 = (assign7000_e5445 + assign7000_e5448);
        let assign7000_e5452: f64 = (p.p403 * var_iae);
        let assign7000_e5453: f64 = (assign7000_e5449 + assign7000_e5452);
        (assign7000_e5453,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign7000_e5455;

        let (assign7010_e5473,) = {
    if (var_guard41 != 0.0) {
        let assign7010_e5461: f64 = (var_ile).powf(p.p406);
        let assign7010_e5462: f64 = (p.p405 * assign7010_e5461);
        let assign7010_e5463: f64 = (p.p404 + assign7010_e5462);
        let assign7010_e5466: f64 = (p.p407 * var_iwe);
        let assign7010_e5467: f64 = (assign7010_e5463 + assign7010_e5466);
        let assign7010_e5470: f64 = (p.p408 * var_iae);
        let assign7010_e5471: f64 = (assign7010_e5467 + assign7010_e5470);
        (assign7010_e5471,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign7010_e5473;

        let (assign7020_e5497,) = {
    if (var_guard41 != 0.0) {
        let assign7020_e5480: f64 = (var_ile).powf(p.p411);
        let assign7020_e5481: f64 = (p.p410 * assign7020_e5480);
        let assign7020_e5482: f64 = (1.0 + assign7020_e5481);
        let assign7020_e5483: f64 = (p.p409 * assign7020_e5482);
        let assign7020_e5487: f64 = (p.p412 * var_iwe);
        let assign7020_e5488: f64 = (1.0 + assign7020_e5487);
        let assign7020_e5489: f64 = (assign7020_e5483 * assign7020_e5488);
        let assign7020_e5493: f64 = (p.p413 * var_iae);
        let assign7020_e5494: f64 = (1.0 + assign7020_e5493);
        let assign7020_e5495: f64 = (assign7020_e5489 * assign7020_e5494);
        (assign7020_e5495,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign7020_e5497;

        let (assign7030_e5507,) = {
    if (var_guard41 != 0.0) {
        let assign7030_e5503: f64 = (var_ile).powf(p.p416);
        let assign7030_e5504: f64 = (p.p415 * assign7030_e5503);
        let assign7030_e5505: f64 = (p.p414 + assign7030_e5504);
        (assign7030_e5505,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign7030_e5507;

        let (assign7040_e5525,) = {
    if (var_guard41 != 0.0) {
        let assign7040_e5512: f64 = (p.p417 * p.p418);
        let assign7040_e5514: f64 = (assign7040_e5512 / var_le);
        let assign7040_e5517: f64 = (-var_le);
        let assign7040_e5519: f64 = (assign7040_e5517 / p.p418);
        let assign7040_e5520: f64 = (assign7040_e5519).exp();
        let assign7040_e5521: f64 = (1.0 - assign7040_e5520);
        let assign7040_e5522: f64 = (assign7040_e5514 * assign7040_e5521);
        let assign7040_e5523: f64 = (1.0 + assign7040_e5522);
        (assign7040_e5523,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign7040_e5525;

        let (assign7050_e5534,) = {
    if (var_guard41 != 0.0) {
        let (assign7050_e5532,) = {
            if (var_gpe_edge > 1e-15) {
                (var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign7050_e5532,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign7050_e5534;

        let (assign7060_e5550,) = {
    if (var_guard41 != 0.0) {
        let assign7060_e5538: f64 = (p.p258 * var_we_edge);
        let assign7060_e5541: f64 = (var_gpe_edge * var_le);
        let assign7060_e5542: f64 = (assign7060_e5538 / assign7060_e5541);
        let assign7060_e5546: f64 = (p.p419 * var_iwe);
        let assign7060_e5547: f64 = (1.0 + assign7060_e5546);
        let assign7060_e5548: f64 = (assign7060_e5542 * assign7060_e5547);
        (assign7060_e5548,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign7060_e5550;

        let (assign7070_e5566,) = {
    if (var_guard41 != 0.0) {
        let assign7070_e5555: f64 = (p.p421 * var_ile);
        let assign7070_e5556: f64 = (p.p420 + assign7070_e5555);
        let assign7070_e5559: f64 = (p.p422 * var_iwe);
        let assign7070_e5560: f64 = (assign7070_e5556 + assign7070_e5559);
        let assign7070_e5563: f64 = (p.p423 * var_iae);
        let assign7070_e5564: f64 = (assign7070_e5560 + assign7070_e5563);
        (assign7070_e5564,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign7070_e5566;

        let (assign7080_e5580,) = {
    if (var_guard41 != 0.0) {
        let assign7080_e5571: f64 = (var_ile).powf(p.p425);
        let assign7080_e5572: f64 = (p.p424 * assign7080_e5571);
        let assign7080_e5576: f64 = (p.p426 * var_iwe);
        let assign7080_e5577: f64 = (1.0 + assign7080_e5576);
        let assign7080_e5578: f64 = (assign7080_e5572 * assign7080_e5577);
        (assign7080_e5578,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign7080_e5580;

        let (assign7090_e5584,) = {
    if (var_guard41 != 0.0) {
        (p.p427,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign7090_e5584;

        let (assign7100_e5588,) = {
    if (var_guard41 != 0.0) {
        (p.p428,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign7100_e5588;

        let (assign7110_e5602,) = {
    if (var_guard41 != 0.0) {
        let assign7110_e5593: f64 = (var_ile).powf(p.p430);
        let assign7110_e5594: f64 = (p.p429 * assign7110_e5593);
        let assign7110_e5598: f64 = (p.p431 * var_iwe);
        let assign7110_e5599: f64 = (1.0 + assign7110_e5598);
        let assign7110_e5600: f64 = (assign7110_e5594 * assign7110_e5599);
        (assign7110_e5600,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign7110_e5602;

        let (assign7120_e5606,) = {
    if (var_guard41 != 0.0) {
        (p.p433,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign7120_e5606;

        let (assign7130_e5610,) = {
    if (var_guard41 != 0.0) {
        (p.p432,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign7130_e5610;

        let (assign7190_e5652,) = {
    if (var_guard41 != 0.0) {
        let assign7190_e5641: f64 = (p.p815 * var_ile);
        let assign7190_e5642: f64 = (p.p814 + assign7190_e5641);
        let assign7190_e5645: f64 = (p.p816 * var_iwe);
        let assign7190_e5646: f64 = (assign7190_e5642 + assign7190_e5645);
        let assign7190_e5649: f64 = (p.p817 * var_iae);
        let assign7190_e5650: f64 = (assign7190_e5646 + assign7190_e5649);
        (assign7190_e5650,)
    } else {
        (var_kvthowe,)
    }
};
        var_kvthowe = assign7190_e5652;

        let (assign7200_e5668,) = {
    if (var_guard41 != 0.0) {
        let assign7200_e5657: f64 = (p.p819 * var_ile);
        let assign7200_e5658: f64 = (p.p818 + assign7200_e5657);
        let assign7200_e5661: f64 = (p.p820 * var_iwe);
        let assign7200_e5662: f64 = (assign7200_e5658 + assign7200_e5661);
        let assign7200_e5665: f64 = (p.p821 * var_iae);
        let assign7200_e5666: f64 = (assign7200_e5662 + assign7200_e5665);
        (assign7200_e5666,)
    } else {
        (var_kuowe,)
    }
};
        var_kuowe = assign7200_e5668;

        let (assign7210_e5696,) = {
    if (var_guard41 != 0.0) {
        let assign7210_e5673: f64 = (0.3333333333333333 * var_w_f);
        let assign7210_e5675: f64 = (assign7210_e5673 / var_ngcon_i);
        let assign7210_e5677: f64 = (assign7210_e5675 + var_xgwe);
        let assign7210_e5678: f64 = (p.p442 * assign7210_e5677);
        let assign7210_e5681: f64 = (var_ngcon_i * var_l_slif);
        let assign7210_e5682: f64 = (assign7210_e5678 / assign7210_e5681);
        let assign7210_e5685: f64 = (p.p440 + p.p441);
        let assign7210_e5688: f64 = (var_w_f * var_l_f);
        let assign7210_e5689: f64 = (assign7210_e5685 / assign7210_e5688);
        let assign7210_e5690: f64 = (assign7210_e5682 + assign7210_e5689);
        let assign7210_e5693: f64 = (var_nf_i * p.p439);
        let assign7210_e5694: f64 = (assign7210_e5690 + assign7210_e5693);
        (assign7210_e5694,)
    } else {
        (var_rg_p,)
    }
};
        var_rg_p = assign7210_e5696;

        let (assign7220_e5705,) = {
    if (var_guard41 != 0.0) {
        let (assign7220_e5703,) = {
            if (p.p444 > 0.0) {
                (p.p444,)
            } else {
                (0.0,)
            }
        };
        (assign7220_e5703,)
    } else {
        (var_rsh_i,)
    }
};
        var_rsh_i = assign7220_e5705;

        let (assign7230_e5714,) = {
    if (var_guard41 != 0.0) {
        let (assign7230_e5712,) = {
            if (p.p445 > 0.0) {
                (p.p445,)
            } else {
                (0.0,)
            }
        };
        (assign7230_e5712,)
    } else {
        (var_rshd_i,)
    }
};
        var_rshd_i = assign7230_e5714;

        let assign7240_e5717: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard55 = assign7240_e5717;

        let (assign7250_e5723,) = {
    if ((var_guard41 != 0.0) && (var_guard55 != 0.0)) {
        (var_rsh_i,)
    } else {
        (var_rshd_i,)
    }
};
        var_rshd_i = assign7250_e5723;

        let (assign7260_e5731,) = {
    if (var_guard41 != 0.0) {
        let assign7260_e5727: f64 = (var_nf_i * p.p12);
        let assign7260_e5729: f64 = (assign7260_e5727 * var_rsh_i);
        (assign7260_e5729,)
    } else {
        (var_rse_p,)
    }
};
        var_rse_p = assign7260_e5731;

        let (assign7270_e5739,) = {
    if (var_guard41 != 0.0) {
        let assign7270_e5735: f64 = (var_nf_i * p.p13);
        let assign7270_e5737: f64 = (assign7270_e5735 * var_rshd_i);
        (assign7270_e5737,)
    } else {
        (var_rde_p,)
    }
};
        var_rde_p = assign7270_e5739;

        let (assign7280_e5745,) = {
    if (var_guard41 != 0.0) {
        let assign7280_e5743: f64 = (var_nf_i * p.p447);
        (assign7280_e5743,)
    } else {
        (var_rwell_p,)
    }
};
        var_rwell_p = assign7280_e5745;

        let (assign7290_e5751,) = {
    if (var_guard41 != 0.0) {
        let assign7290_e5749: f64 = (var_nf_i * p.p446);
        (assign7290_e5749,)
    } else {
        (var_rbulk_p,)
    }
};
        var_rbulk_p = assign7290_e5751;

        let (assign7300_e5757,) = {
    if (var_guard41 != 0.0) {
        let assign7300_e5755: f64 = (var_nf_i * p.p448);
        (assign7300_e5755,)
    } else {
        (var_rjuns_p,)
    }
};
        var_rjuns_p = assign7300_e5757;

        let (assign7310_e5763,) = {
    if (var_guard41 != 0.0) {
        let assign7310_e5761: f64 = (var_nf_i * p.p449);
        (assign7310_e5761,)
    } else {
        (var_rjund_p,)
    }
};
        var_rjund_p = assign7310_e5763;

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
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_fnt_p_slot = var_fnt_p;
        *var_fntexc_p_slot = var_fntexc_p;
        *var_gpe_edge_slot = var_gpe_edge;
        *var_guard55_slot = var_guard55;
        *var_kuowe_slot = var_kuowe;
        *var_kvthowe_slot = var_kvthowe;
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
        *var_rwell_p_slot = var_rwell_p;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_temp0_slot = var_temp0;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_we_edge_slot = var_we_edge;
    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard41: f64,
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
        var_gfacnud_p_slot: &mut f64,
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
        var_guard78_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_munqs_p_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_rs_p_slot: &mut f64,
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
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_munqs_p: f64 = *var_munqs_p_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;

        let (assign7320_e5767,) = {
    if (var_guard41 != 0.0) {
        (p.p450,)
    } else {
        (var_munqs_p,)
    }
};
        var_munqs_p = assign7320_e5767;

        let assign7330_e5786: f64 = if (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]) { 1.0 } else { 0.0 };
        var_guard56 = assign7330_e5786;

        let (assign7340_e5804,) = {
    if ((var_guard41 != 0.0) && (var_guard56 != 0.0)) {
        let assign7340_e5793: f64 = (p.p452 * var_ile);
        let assign7340_e5794: f64 = (p.p451 + assign7340_e5793);
        let assign7340_e5797: f64 = (p.p453 * var_iwe);
        let assign7340_e5798: f64 = (assign7340_e5794 + assign7340_e5797);
        let assign7340_e5801: f64 = (p.p454 * var_iae);
        let assign7340_e5802: f64 = (assign7340_e5798 + assign7340_e5801);
        (assign7340_e5802,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign7340_e5804;

        let assign7350_e5823: f64 = if (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]) { 1.0 } else { 0.0 };
        var_guard57 = assign7350_e5823;

        let (assign7360_e5841,) = {
    if ((var_guard41 != 0.0) && (var_guard57 != 0.0)) {
        let assign7360_e5830: f64 = (p.p456 * var_ile);
        let assign7360_e5831: f64 = (p.p455 + assign7360_e5830);
        let assign7360_e5834: f64 = (p.p457 * var_iwe);
        let assign7360_e5835: f64 = (assign7360_e5831 + assign7360_e5834);
        let assign7360_e5838: f64 = (p.p458 * var_iae);
        let assign7360_e5839: f64 = (assign7360_e5835 + assign7360_e5838);
        (assign7360_e5839,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign7360_e5841;

        let assign7370_e5860: f64 = if (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]) { 1.0 } else { 0.0 };
        var_guard58 = assign7370_e5860;

        let (assign7380_e5878,) = {
    if ((var_guard41 != 0.0) && (var_guard58 != 0.0)) {
        let assign7380_e5867: f64 = (p.p460 * var_ile);
        let assign7380_e5868: f64 = (p.p459 + assign7380_e5867);
        let assign7380_e5871: f64 = (p.p461 * var_iwe);
        let assign7380_e5872: f64 = (assign7380_e5868 + assign7380_e5871);
        let assign7380_e5875: f64 = (p.p462 * var_iae);
        let assign7380_e5876: f64 = (assign7380_e5872 + assign7380_e5875);
        (assign7380_e5876,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign7380_e5878;

        let assign7390_e5897: f64 = if (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]) { 1.0 } else { 0.0 };
        var_guard59 = assign7390_e5897;

        let (assign7400_e5915,) = {
    if ((var_guard41 != 0.0) && (var_guard59 != 0.0)) {
        let assign7400_e5904: f64 = (p.p464 * var_ile);
        let assign7400_e5905: f64 = (p.p463 + assign7400_e5904);
        let assign7400_e5908: f64 = (p.p465 * var_iwe);
        let assign7400_e5909: f64 = (assign7400_e5905 + assign7400_e5908);
        let assign7400_e5912: f64 = (p.p466 * var_iae);
        let assign7400_e5913: f64 = (assign7400_e5909 + assign7400_e5912);
        (assign7400_e5913,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign7400_e5915;

        let assign7410_e5934: f64 = if (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]) { 1.0 } else { 0.0 };
        var_guard60 = assign7410_e5934;

        let (assign7420_e5952,) = {
    if ((var_guard41 != 0.0) && (var_guard60 != 0.0)) {
        let assign7420_e5941: f64 = (p.p468 * var_ile);
        let assign7420_e5942: f64 = (p.p467 + assign7420_e5941);
        let assign7420_e5945: f64 = (p.p469 * var_iwe);
        let assign7420_e5946: f64 = (assign7420_e5942 + assign7420_e5945);
        let assign7420_e5949: f64 = (p.p470 * var_iae);
        let assign7420_e5950: f64 = (assign7420_e5946 + assign7420_e5949);
        (assign7420_e5950,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign7420_e5952;

        let assign7430_e5971: f64 = if (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]) { 1.0 } else { 0.0 };
        var_guard61 = assign7430_e5971;

        let (assign7440_e5989,) = {
    if ((var_guard41 != 0.0) && (var_guard61 != 0.0)) {
        let assign7440_e5978: f64 = (p.p472 * var_ile);
        let assign7440_e5979: f64 = (p.p471 + assign7440_e5978);
        let assign7440_e5982: f64 = (p.p473 * var_iwe);
        let assign7440_e5983: f64 = (assign7440_e5979 + assign7440_e5982);
        let assign7440_e5986: f64 = (p.p474 * var_iae);
        let assign7440_e5987: f64 = (assign7440_e5983 + assign7440_e5986);
        (assign7440_e5987,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign7440_e5989;

        let assign7450_e6008: f64 = if (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]) { 1.0 } else { 0.0 };
        var_guard62 = assign7450_e6008;

        let (assign7460_e6026,) = {
    if ((var_guard41 != 0.0) && (var_guard62 != 0.0)) {
        let assign7460_e6015: f64 = (p.p476 * var_ile);
        let assign7460_e6016: f64 = (p.p475 + assign7460_e6015);
        let assign7460_e6019: f64 = (p.p477 * var_iwe);
        let assign7460_e6020: f64 = (assign7460_e6016 + assign7460_e6019);
        let assign7460_e6023: f64 = (p.p478 * var_iae);
        let assign7460_e6024: f64 = (assign7460_e6020 + assign7460_e6023);
        (assign7460_e6024,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign7460_e6026;

        let assign7470_e6045: f64 = if (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]) { 1.0 } else { 0.0 };
        var_guard63 = assign7470_e6045;

        let (assign7480_e6063,) = {
    if ((var_guard41 != 0.0) && (var_guard63 != 0.0)) {
        let assign7480_e6052: f64 = (p.p480 * var_ile);
        let assign7480_e6053: f64 = (p.p479 + assign7480_e6052);
        let assign7480_e6056: f64 = (p.p481 * var_iwe);
        let assign7480_e6057: f64 = (assign7480_e6053 + assign7480_e6056);
        let assign7480_e6060: f64 = (p.p482 * var_iae);
        let assign7480_e6061: f64 = (assign7480_e6057 + assign7480_e6060);
        (assign7480_e6061,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign7480_e6063;

        let assign7490_e6082: f64 = if (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]) { 1.0 } else { 0.0 };
        var_guard64 = assign7490_e6082;

        let (assign7500_e6100,) = {
    if ((var_guard41 != 0.0) && (var_guard64 != 0.0)) {
        let assign7500_e6089: f64 = (p.p484 * var_ile);
        let assign7500_e6090: f64 = (p.p483 + assign7500_e6089);
        let assign7500_e6093: f64 = (p.p485 * var_iwe);
        let assign7500_e6094: f64 = (assign7500_e6090 + assign7500_e6093);
        let assign7500_e6097: f64 = (p.p486 * var_iae);
        let assign7500_e6098: f64 = (assign7500_e6094 + assign7500_e6097);
        (assign7500_e6098,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign7500_e6100;

        let assign7510_e6119: f64 = if (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]) { 1.0 } else { 0.0 };
        var_guard65 = assign7510_e6119;

        let (assign7520_e6137,) = {
    if ((var_guard41 != 0.0) && (var_guard65 != 0.0)) {
        let assign7520_e6126: f64 = (p.p488 * var_ile);
        let assign7520_e6127: f64 = (p.p487 + assign7520_e6126);
        let assign7520_e6130: f64 = (p.p489 * var_iwe);
        let assign7520_e6131: f64 = (assign7520_e6127 + assign7520_e6130);
        let assign7520_e6134: f64 = (p.p490 * var_iae);
        let assign7520_e6135: f64 = (assign7520_e6131 + assign7520_e6134);
        (assign7520_e6135,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign7520_e6137;

        let assign7530_e6156: f64 = if (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]) { 1.0 } else { 0.0 };
        var_guard66 = assign7530_e6156;

        let (assign7540_e6174,) = {
    if ((var_guard41 != 0.0) && (var_guard66 != 0.0)) {
        let assign7540_e6163: f64 = (p.p496 * var_ile);
        let assign7540_e6164: f64 = (p.p495 + assign7540_e6163);
        let assign7540_e6167: f64 = (p.p497 * var_iwe);
        let assign7540_e6168: f64 = (assign7540_e6164 + assign7540_e6167);
        let assign7540_e6171: f64 = (p.p498 * var_iae);
        let assign7540_e6172: f64 = (assign7540_e6168 + assign7540_e6171);
        (assign7540_e6172,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign7540_e6174;

        let assign7550_e6193: f64 = if (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]) { 1.0 } else { 0.0 };
        var_guard67 = assign7550_e6193;

        let (assign7560_e6211,) = {
    if ((var_guard41 != 0.0) && (var_guard67 != 0.0)) {
        let assign7560_e6200: f64 = (p.p492 * var_ile);
        let assign7560_e6201: f64 = (p.p491 + assign7560_e6200);
        let assign7560_e6204: f64 = (p.p493 * var_iwe);
        let assign7560_e6205: f64 = (assign7560_e6201 + assign7560_e6204);
        let assign7560_e6208: f64 = (p.p494 * var_iae);
        let assign7560_e6209: f64 = (assign7560_e6205 + assign7560_e6208);
        (assign7560_e6209,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign7560_e6211;

        let assign7570_e6230: f64 = if (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]) { 1.0 } else { 0.0 };
        var_guard68 = assign7570_e6230;

        let (assign7580_e6248,) = {
    if ((var_guard41 != 0.0) && (var_guard68 != 0.0)) {
        let assign7580_e6237: f64 = (p.p500 * var_ile);
        let assign7580_e6238: f64 = (p.p499 + assign7580_e6237);
        let assign7580_e6241: f64 = (p.p501 * var_iwe);
        let assign7580_e6242: f64 = (assign7580_e6238 + assign7580_e6241);
        let assign7580_e6245: f64 = (p.p502 * var_iae);
        let assign7580_e6246: f64 = (assign7580_e6242 + assign7580_e6245);
        (assign7580_e6246,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign7580_e6248;

        let assign7590_e6267: f64 = if (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]) { 1.0 } else { 0.0 };
        var_guard69 = assign7590_e6267;

        let (assign7600_e6287,) = {
    if ((var_guard41 != 0.0) && (var_guard69 != 0.0)) {
        let assign7600_e6275: f64 = (p.p504 * var_ile);
        let assign7600_e6276: f64 = (p.p503 + assign7600_e6275);
        let assign7600_e6279: f64 = (p.p505 * var_iwe);
        let assign7600_e6280: f64 = (assign7600_e6276 + assign7600_e6279);
        let assign7600_e6283: f64 = (p.p506 * var_iae);
        let assign7600_e6284: f64 = (assign7600_e6280 + assign7600_e6283);
        let assign7600_e6285: f64 = (var_ile2 * assign7600_e6284);
        (assign7600_e6285,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign7600_e6287;

        let assign7610_e6306: f64 = if (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]) { 1.0 } else { 0.0 };
        var_guard70 = assign7610_e6306;

        let (assign7620_e6324,) = {
    if ((var_guard41 != 0.0) && (var_guard70 != 0.0)) {
        let assign7620_e6313: f64 = (p.p512 * var_ile);
        let assign7620_e6314: f64 = (p.p511 + assign7620_e6313);
        let assign7620_e6317: f64 = (p.p513 * var_iwe);
        let assign7620_e6318: f64 = (assign7620_e6314 + assign7620_e6317);
        let assign7620_e6321: f64 = (p.p514 * var_iae);
        let assign7620_e6322: f64 = (assign7620_e6318 + assign7620_e6321);
        (assign7620_e6322,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign7620_e6324;

        let assign7630_e6343: f64 = if (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]) { 1.0 } else { 0.0 };
        var_guard71 = assign7630_e6343;

        let (assign7640_e6361,) = {
    if ((var_guard41 != 0.0) && (var_guard71 != 0.0)) {
        let assign7640_e6350: f64 = (p.p508 * var_ile);
        let assign7640_e6351: f64 = (p.p507 + assign7640_e6350);
        let assign7640_e6354: f64 = (p.p509 * var_iwe);
        let assign7640_e6355: f64 = (assign7640_e6351 + assign7640_e6354);
        let assign7640_e6358: f64 = (p.p510 * var_iae);
        let assign7640_e6359: f64 = (assign7640_e6355 + assign7640_e6358);
        (assign7640_e6359,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign7640_e6361;

        let assign7650_e6380: f64 = if (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]) { 1.0 } else { 0.0 };
        var_guard72 = assign7650_e6380;

        let (assign7660_e6400,) = {
    if ((var_guard41 != 0.0) && (var_guard72 != 0.0)) {
        let assign7660_e6388: f64 = (p.p516 * var_ile);
        let assign7660_e6389: f64 = (p.p515 + assign7660_e6388);
        let assign7660_e6392: f64 = (p.p517 * var_iwe);
        let assign7660_e6393: f64 = (assign7660_e6389 + assign7660_e6392);
        let assign7660_e6396: f64 = (p.p518 * var_iae);
        let assign7660_e6397: f64 = (assign7660_e6393 + assign7660_e6396);
        let assign7660_e6398: f64 = (var_ile2 * assign7660_e6397);
        (assign7660_e6398,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign7660_e6400;

        let assign7670_e6419: f64 = if (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]) { 1.0 } else { 0.0 };
        var_guard73 = assign7670_e6419;

        let (assign7680_e6437,) = {
    if ((var_guard41 != 0.0) && (var_guard73 != 0.0)) {
        let assign7680_e6426: f64 = (p.p524 * var_ile);
        let assign7680_e6427: f64 = (p.p523 + assign7680_e6426);
        let assign7680_e6430: f64 = (p.p525 * var_iwe);
        let assign7680_e6431: f64 = (assign7680_e6427 + assign7680_e6430);
        let assign7680_e6434: f64 = (p.p526 * var_iae);
        let assign7680_e6435: f64 = (assign7680_e6431 + assign7680_e6434);
        (assign7680_e6435,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign7680_e6437;

        let assign7690_e6456: f64 = if (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]) { 1.0 } else { 0.0 };
        var_guard74 = assign7690_e6456;

        let (assign7700_e6474,) = {
    if ((var_guard41 != 0.0) && (var_guard74 != 0.0)) {
        let assign7700_e6463: f64 = (p.p520 * var_ile);
        let assign7700_e6464: f64 = (p.p519 + assign7700_e6463);
        let assign7700_e6467: f64 = (p.p521 * var_iwe);
        let assign7700_e6468: f64 = (assign7700_e6464 + assign7700_e6467);
        let assign7700_e6471: f64 = (p.p522 * var_iae);
        let assign7700_e6472: f64 = (assign7700_e6468 + assign7700_e6471);
        (assign7700_e6472,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign7700_e6474;

        let assign7710_e6493: f64 = if (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]) { 1.0 } else { 0.0 };
        var_guard75 = assign7710_e6493;

        let (assign7720_e6515,) = {
    if ((var_guard41 != 0.0) && (var_guard75 != 0.0)) {
        let assign7720_e6499: f64 = (var_we / var_le);
        let assign7720_e6503: f64 = (p.p528 * var_ile);
        let assign7720_e6504: f64 = (p.p527 + assign7720_e6503);
        let assign7720_e6507: f64 = (p.p529 * var_iwe);
        let assign7720_e6508: f64 = (assign7720_e6504 + assign7720_e6507);
        let assign7720_e6511: f64 = (p.p530 * var_iae);
        let assign7720_e6512: f64 = (assign7720_e6508 + assign7720_e6511);
        let assign7720_e6513: f64 = (assign7720_e6499 * assign7720_e6512);
        (assign7720_e6513,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign7720_e6515;

        let assign7730_e6534: f64 = if (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]) { 1.0 } else { 0.0 };
        var_guard76 = assign7730_e6534;

        let (assign7740_e6552,) = {
    if ((var_guard41 != 0.0) && (var_guard76 != 0.0)) {
        let assign7740_e6541: f64 = (p.p532 * var_ile);
        let assign7740_e6542: f64 = (p.p531 + assign7740_e6541);
        let assign7740_e6545: f64 = (p.p533 * var_iwe);
        let assign7740_e6546: f64 = (assign7740_e6542 + assign7740_e6545);
        let assign7740_e6549: f64 = (p.p534 * var_iae);
        let assign7740_e6550: f64 = (assign7740_e6546 + assign7740_e6549);
        (assign7740_e6550,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign7740_e6552;

        let assign7750_e6571: f64 = if (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]) { 1.0 } else { 0.0 };
        var_guard77 = assign7750_e6571;

        let (assign7760_e6589,) = {
    if ((var_guard41 != 0.0) && (var_guard77 != 0.0)) {
        let assign7760_e6578: f64 = (p.p536 * var_ile);
        let assign7760_e6579: f64 = (p.p535 + assign7760_e6578);
        let assign7760_e6582: f64 = (p.p537 * var_iwe);
        let assign7760_e6583: f64 = (assign7760_e6579 + assign7760_e6582);
        let assign7760_e6586: f64 = (p.p538 * var_iae);
        let assign7760_e6587: f64 = (assign7760_e6583 + assign7760_e6586);
        (assign7760_e6587,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign7760_e6589;

        let assign7770_e6608: f64 = if (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]) { 1.0 } else { 0.0 };
        var_guard78 = assign7770_e6608;

        let (assign7780_e6626,) = {
    if ((var_guard41 != 0.0) && (var_guard78 != 0.0)) {
        let assign7780_e6615: f64 = (p.p540 * var_ile);
        let assign7780_e6616: f64 = (p.p539 + assign7780_e6615);
        let assign7780_e6619: f64 = (p.p541 * var_iwe);
        let assign7780_e6620: f64 = (assign7780_e6616 + assign7780_e6619);
        let assign7780_e6623: f64 = (p.p542 * var_iae);
        let assign7780_e6624: f64 = (assign7780_e6620 + assign7780_e6623);
        (assign7780_e6624,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign7780_e6626;

        let assign7790_e6645: f64 = if (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]) { 1.0 } else { 0.0 };
        var_guard79 = assign7790_e6645;

        let (assign7800_e6663,) = {
    if ((var_guard41 != 0.0) && (var_guard79 != 0.0)) {
        let assign7800_e6652: f64 = (p.p544 * var_ile);
        let assign7800_e6653: f64 = (p.p543 + assign7800_e6652);
        let assign7800_e6656: f64 = (p.p545 * var_iwe);
        let assign7800_e6657: f64 = (assign7800_e6653 + assign7800_e6656);
        let assign7800_e6660: f64 = (p.p546 * var_iae);
        let assign7800_e6661: f64 = (assign7800_e6657 + assign7800_e6660);
        (assign7800_e6661,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign7800_e6663;

        let assign7810_e6682: f64 = if (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]) { 1.0 } else { 0.0 };
        var_guard80 = assign7810_e6682;

        let (assign7820_e6700,) = {
    if ((var_guard41 != 0.0) && (var_guard80 != 0.0)) {
        let assign7820_e6689: f64 = (p.p548 * var_ile);
        let assign7820_e6690: f64 = (p.p547 + assign7820_e6689);
        let assign7820_e6693: f64 = (p.p549 * var_iwe);
        let assign7820_e6694: f64 = (assign7820_e6690 + assign7820_e6693);
        let assign7820_e6697: f64 = (p.p550 * var_iae);
        let assign7820_e6698: f64 = (assign7820_e6694 + assign7820_e6697);
        (assign7820_e6698,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign7820_e6700;

        let assign7830_e6719: f64 = if (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]) { 1.0 } else { 0.0 };
        var_guard81 = assign7830_e6719;

        let (assign7840_e6737,) = {
    if ((var_guard41 != 0.0) && (var_guard81 != 0.0)) {
        let assign7840_e6726: f64 = (p.p552 * var_ile);
        let assign7840_e6727: f64 = (p.p551 + assign7840_e6726);
        let assign7840_e6730: f64 = (p.p553 * var_iwe);
        let assign7840_e6731: f64 = (assign7840_e6727 + assign7840_e6730);
        let assign7840_e6734: f64 = (p.p554 * var_iae);
        let assign7840_e6735: f64 = (assign7840_e6731 + assign7840_e6734);
        (assign7840_e6735,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign7840_e6737;

        let assign7850_e6756: f64 = if (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]) { 1.0 } else { 0.0 };
        var_guard82 = assign7850_e6756;

        let (assign7860_e6776,) = {
    if ((var_guard41 != 0.0) && (var_guard82 != 0.0)) {
        let assign7860_e6764: f64 = (p.p556 * var_ile);
        let assign7860_e6765: f64 = (p.p555 + assign7860_e6764);
        let assign7860_e6768: f64 = (p.p557 * var_iwe);
        let assign7860_e6769: f64 = (assign7860_e6765 + assign7860_e6768);
        let assign7860_e6772: f64 = (p.p558 * var_iae);
        let assign7860_e6773: f64 = (assign7860_e6769 + assign7860_e6772);
        let assign7860_e6774: f64 = (var_iwe * assign7860_e6773);
        (assign7860_e6774,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign7860_e6776;

        let assign7870_e6795: f64 = if (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]) { 1.0 } else { 0.0 };
        var_guard83 = assign7870_e6795;

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
        *var_guard78_slot = var_guard78;
        *var_guard79_slot = var_guard79;
        *var_guard80_slot = var_guard80;
        *var_guard81_slot = var_guard81;
        *var_guard82_slot = var_guard82;
        *var_guard83_slot = var_guard83;
        *var_mue_p_slot = var_mue_p;
        *var_munqs_p_slot = var_munqs_p;
        *var_neff_p_slot = var_neff_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_np_p_slot = var_np_p;
        *var_psce_p_slot = var_psce_p;
        *var_psceb_p_slot = var_psceb_p;
        *var_psced_p_slot = var_psced_p;
        *var_rs_p_slot = var_rs_p;
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
        var_guard41: f64,
        var_guard83: f64,
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
        var_guard106_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard110_slot: &mut f64,
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
        var_plparam_i_slot: &mut f64,
        var_poparam_i_slot: &mut f64,
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
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
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
        let mut var_plparam_i: f64 = *var_plparam_i_slot;
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
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

        let (assign7880_e6813,) = {
    if ((var_guard41 != 0.0) && (var_guard83 != 0.0)) {
        let assign7880_e6802: f64 = (p.p560 * var_ile);
        let assign7880_e6803: f64 = (p.p559 + assign7880_e6802);
        let assign7880_e6806: f64 = (p.p561 * var_iwe);
        let assign7880_e6807: f64 = (assign7880_e6803 + assign7880_e6806);
        let assign7880_e6810: f64 = (p.p562 * var_iae);
        let assign7880_e6811: f64 = (assign7880_e6807 + assign7880_e6810);
        (assign7880_e6811,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign7880_e6813;

        let assign7890_e6832: f64 = if (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]) { 1.0 } else { 0.0 };
        var_guard84 = assign7890_e6832;

        let (assign7900_e6850,) = {
    if ((var_guard41 != 0.0) && (var_guard84 != 0.0)) {
        let assign7900_e6839: f64 = (p.p564 * var_ile);
        let assign7900_e6840: f64 = (p.p563 + assign7900_e6839);
        let assign7900_e6843: f64 = (p.p565 * var_iwe);
        let assign7900_e6844: f64 = (assign7900_e6840 + assign7900_e6843);
        let assign7900_e6847: f64 = (p.p566 * var_iae);
        let assign7900_e6848: f64 = (assign7900_e6844 + assign7900_e6847);
        (assign7900_e6848,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign7900_e6850;

        let assign7910_e6869: f64 = if (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]) { 1.0 } else { 0.0 };
        var_guard85 = assign7910_e6869;

        let (assign7920_e6887,) = {
    if ((var_guard41 != 0.0) && (var_guard85 != 0.0)) {
        let assign7920_e6876: f64 = (p.p568 * var_ile);
        let assign7920_e6877: f64 = (p.p567 + assign7920_e6876);
        let assign7920_e6880: f64 = (p.p569 * var_iwe);
        let assign7920_e6881: f64 = (assign7920_e6877 + assign7920_e6880);
        let assign7920_e6884: f64 = (p.p570 * var_iae);
        let assign7920_e6885: f64 = (assign7920_e6881 + assign7920_e6884);
        (assign7920_e6885,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign7920_e6887;

        let assign7930_e6906: f64 = if (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        var_guard86 = assign7930_e6906;

        let (assign7940_e6926,) = {
    if ((var_guard41 != 0.0) && (var_guard86 != 0.0)) {
        let assign7940_e6914: f64 = (p.p572 * var_ile);
        let assign7940_e6915: f64 = (p.p571 + assign7940_e6914);
        let assign7940_e6918: f64 = (p.p573 * var_iwe);
        let assign7940_e6919: f64 = (assign7940_e6915 + assign7940_e6918);
        let assign7940_e6922: f64 = (p.p574 * var_iae);
        let assign7940_e6923: f64 = (assign7940_e6919 + assign7940_e6922);
        let assign7940_e6924: f64 = (var_ile * assign7940_e6923);
        (assign7940_e6924,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign7940_e6926;

        let assign7950_e6945: f64 = if (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]) { 1.0 } else { 0.0 };
        var_guard87 = assign7950_e6945;

        let (assign7960_e6963,) = {
    if ((var_guard41 != 0.0) && (var_guard87 != 0.0)) {
        let assign7960_e6952: f64 = (p.p576 * var_ile);
        let assign7960_e6953: f64 = (p.p575 + assign7960_e6952);
        let assign7960_e6956: f64 = (p.p577 * var_iwe);
        let assign7960_e6957: f64 = (assign7960_e6953 + assign7960_e6956);
        let assign7960_e6960: f64 = (p.p578 * var_iae);
        let assign7960_e6961: f64 = (assign7960_e6957 + assign7960_e6960);
        (assign7960_e6961,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign7960_e6963;

        let assign7970_e6982: f64 = if (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]) { 1.0 } else { 0.0 };
        var_guard88 = assign7970_e6982;

        let (assign7980_e7000,) = {
    if ((var_guard41 != 0.0) && (var_guard88 != 0.0)) {
        let assign7980_e6989: f64 = (p.p580 * var_ile);
        let assign7980_e6990: f64 = (p.p579 + assign7980_e6989);
        let assign7980_e6993: f64 = (p.p581 * var_iwe);
        let assign7980_e6994: f64 = (assign7980_e6990 + assign7980_e6993);
        let assign7980_e6997: f64 = (p.p582 * var_iae);
        let assign7980_e6998: f64 = (assign7980_e6994 + assign7980_e6997);
        (assign7980_e6998,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign7980_e7000;

        let assign7990_e7019: f64 = if (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]) { 1.0 } else { 0.0 };
        var_guard89 = assign7990_e7019;

        let (assign8000_e7037,) = {
    if ((var_guard41 != 0.0) && (var_guard89 != 0.0)) {
        let assign8000_e7026: f64 = (p.p584 * var_ile);
        let assign8000_e7027: f64 = (p.p583 + assign8000_e7026);
        let assign8000_e7030: f64 = (p.p585 * var_iwe);
        let assign8000_e7031: f64 = (assign8000_e7027 + assign8000_e7030);
        let assign8000_e7034: f64 = (p.p586 * var_iae);
        let assign8000_e7035: f64 = (assign8000_e7031 + assign8000_e7034);
        (assign8000_e7035,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign8000_e7037;

        let assign8010_e7056: f64 = if (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        var_guard90 = assign8010_e7056;

        let (assign8020_e7074,) = {
    if ((var_guard41 != 0.0) && (var_guard90 != 0.0)) {
        let assign8020_e7063: f64 = (p.p588 * var_ile);
        let assign8020_e7064: f64 = (p.p587 + assign8020_e7063);
        let assign8020_e7067: f64 = (p.p589 * var_iwe);
        let assign8020_e7068: f64 = (assign8020_e7064 + assign8020_e7067);
        let assign8020_e7071: f64 = (p.p590 * var_iae);
        let assign8020_e7072: f64 = (assign8020_e7068 + assign8020_e7071);
        (assign8020_e7072,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign8020_e7074;

        let assign8030_e7093: f64 = if (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]) { 1.0 } else { 0.0 };
        var_guard91 = assign8030_e7093;

        let (assign8040_e7113,) = {
    if ((var_guard41 != 0.0) && (var_guard91 != 0.0)) {
        let assign8040_e7101: f64 = (p.p592 * var_ile);
        let assign8040_e7102: f64 = (p.p591 + assign8040_e7101);
        let assign8040_e7105: f64 = (p.p593 * var_iwe);
        let assign8040_e7106: f64 = (assign8040_e7102 + assign8040_e7105);
        let assign8040_e7109: f64 = (p.p594 * var_iae);
        let assign8040_e7110: f64 = (assign8040_e7106 + assign8040_e7109);
        let assign8040_e7111: f64 = (var_ile * assign8040_e7110);
        (assign8040_e7111,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign8040_e7113;

        let assign8050_e7132: f64 = if (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]) { 1.0 } else { 0.0 };
        var_guard92 = assign8050_e7132;

        let (assign8060_e7150,) = {
    if ((var_guard41 != 0.0) && (var_guard92 != 0.0)) {
        let assign8060_e7139: f64 = (p.p596 * var_ile);
        let assign8060_e7140: f64 = (p.p595 + assign8060_e7139);
        let assign8060_e7143: f64 = (p.p597 * var_iwe);
        let assign8060_e7144: f64 = (assign8060_e7140 + assign8060_e7143);
        let assign8060_e7147: f64 = (p.p598 * var_iae);
        let assign8060_e7148: f64 = (assign8060_e7144 + assign8060_e7147);
        (assign8060_e7148,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign8060_e7150;

        let assign8070_e7169: f64 = if (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]) { 1.0 } else { 0.0 };
        var_guard93 = assign8070_e7169;

        let (assign8080_e7187,) = {
    if ((var_guard41 != 0.0) && (var_guard93 != 0.0)) {
        let assign8080_e7176: f64 = (p.p600 * var_ile);
        let assign8080_e7177: f64 = (p.p599 + assign8080_e7176);
        let assign8080_e7180: f64 = (p.p601 * var_iwe);
        let assign8080_e7181: f64 = (assign8080_e7177 + assign8080_e7180);
        let assign8080_e7184: f64 = (p.p602 * var_iae);
        let assign8080_e7185: f64 = (assign8080_e7181 + assign8080_e7184);
        (assign8080_e7185,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign8080_e7187;

        let assign8090_e7206: f64 = if (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]) { 1.0 } else { 0.0 };
        var_guard94 = assign8090_e7206;

        let (assign8100_e7224,) = {
    if ((var_guard41 != 0.0) && (var_guard94 != 0.0)) {
        let assign8100_e7213: f64 = (p.p604 * var_ile);
        let assign8100_e7214: f64 = (p.p603 + assign8100_e7213);
        let assign8100_e7217: f64 = (p.p605 * var_iwe);
        let assign8100_e7218: f64 = (assign8100_e7214 + assign8100_e7217);
        let assign8100_e7221: f64 = (p.p606 * var_iae);
        let assign8100_e7222: f64 = (assign8100_e7218 + assign8100_e7221);
        (assign8100_e7222,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign8100_e7224;

        let assign8110_e7243: f64 = if (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]) { 1.0 } else { 0.0 };
        var_guard95 = assign8110_e7243;

        let (assign8120_e7261,) = {
    if ((var_guard41 != 0.0) && (var_guard95 != 0.0)) {
        let assign8120_e7250: f64 = (p.p608 * var_ile);
        let assign8120_e7251: f64 = (p.p607 + assign8120_e7250);
        let assign8120_e7254: f64 = (p.p609 * var_iwe);
        let assign8120_e7255: f64 = (assign8120_e7251 + assign8120_e7254);
        let assign8120_e7258: f64 = (p.p610 * var_iae);
        let assign8120_e7259: f64 = (assign8120_e7255 + assign8120_e7258);
        (assign8120_e7259,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign8120_e7261;

        let assign8130_e7280: f64 = if (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]) { 1.0 } else { 0.0 };
        var_guard96 = assign8130_e7280;

        let (assign8140_e7298,) = {
    if ((var_guard41 != 0.0) && (var_guard96 != 0.0)) {
        let assign8140_e7287: f64 = (p.p612 * var_ile);
        let assign8140_e7288: f64 = (p.p611 + assign8140_e7287);
        let assign8140_e7291: f64 = (p.p613 * var_iwe);
        let assign8140_e7292: f64 = (assign8140_e7288 + assign8140_e7291);
        let assign8140_e7295: f64 = (p.p614 * var_iae);
        let assign8140_e7296: f64 = (assign8140_e7292 + assign8140_e7295);
        (assign8140_e7296,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign8140_e7298;

        let assign8150_e7317: f64 = if (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]) { 1.0 } else { 0.0 };
        var_guard97 = assign8150_e7317;

        let (assign8160_e7335,) = {
    if ((var_guard41 != 0.0) && (var_guard97 != 0.0)) {
        let assign8160_e7324: f64 = (p.p616 * var_ile);
        let assign8160_e7325: f64 = (p.p615 + assign8160_e7324);
        let assign8160_e7328: f64 = (p.p617 * var_iwe);
        let assign8160_e7329: f64 = (assign8160_e7325 + assign8160_e7328);
        let assign8160_e7332: f64 = (p.p618 * var_iae);
        let assign8160_e7333: f64 = (assign8160_e7329 + assign8160_e7332);
        (assign8160_e7333,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign8160_e7335;

        let assign8170_e7354: f64 = if (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]) { 1.0 } else { 0.0 };
        var_guard98 = assign8170_e7354;

        let (assign8180_e7374,) = {
    if ((var_guard41 != 0.0) && (var_guard98 != 0.0)) {
        let assign8180_e7362: f64 = (p.p620 * var_ile);
        let assign8180_e7363: f64 = (p.p619 + assign8180_e7362);
        let assign8180_e7366: f64 = (p.p621 * var_iwe);
        let assign8180_e7367: f64 = (assign8180_e7363 + assign8180_e7366);
        let assign8180_e7370: f64 = (p.p622 * var_iae);
        let assign8180_e7371: f64 = (assign8180_e7367 + assign8180_e7370);
        let assign8180_e7372: f64 = (var_iiae * assign8180_e7371);
        (assign8180_e7372,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign8180_e7374;

        let assign8190_e7393: f64 = if (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]) { 1.0 } else { 0.0 };
        var_guard99 = assign8190_e7393;

        let (assign8200_e7413,) = {
    if ((var_guard41 != 0.0) && (var_guard99 != 0.0)) {
        let assign8200_e7401: f64 = (p.p624 * var_ile);
        let assign8200_e7402: f64 = (p.p623 + assign8200_e7401);
        let assign8200_e7405: f64 = (p.p625 * var_iwe);
        let assign8200_e7406: f64 = (assign8200_e7402 + assign8200_e7405);
        let assign8200_e7409: f64 = (p.p626 * var_iae);
        let assign8200_e7410: f64 = (assign8200_e7406 + assign8200_e7409);
        let assign8200_e7411: f64 = (var_iiwe * assign8200_e7410);
        (assign8200_e7411,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign8200_e7413;

        let assign8210_e7432: f64 = if (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]) { 1.0 } else { 0.0 };
        var_guard100 = assign8210_e7432;

        let (assign8220_e7452,) = {
    if ((var_guard41 != 0.0) && (var_guard100 != 0.0)) {
        let assign8220_e7440: f64 = (p.p628 * var_ile);
        let assign8220_e7441: f64 = (p.p627 + assign8220_e7440);
        let assign8220_e7444: f64 = (p.p629 * var_iwe);
        let assign8220_e7445: f64 = (assign8220_e7441 + assign8220_e7444);
        let assign8220_e7448: f64 = (p.p630 * var_iae);
        let assign8220_e7449: f64 = (assign8220_e7445 + assign8220_e7448);
        let assign8220_e7450: f64 = (var_iiwe * assign8220_e7449);
        (assign8220_e7450,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign8220_e7452;

        let assign8230_e7471: f64 = if (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]) { 1.0 } else { 0.0 };
        var_guard101 = assign8230_e7471;

        let (assign8240_e7489,) = {
    if ((var_guard41 != 0.0) && (var_guard101 != 0.0)) {
        let assign8240_e7478: f64 = (p.p632 * var_ile);
        let assign8240_e7479: f64 = (p.p631 + assign8240_e7478);
        let assign8240_e7482: f64 = (p.p633 * var_iwe);
        let assign8240_e7483: f64 = (assign8240_e7479 + assign8240_e7482);
        let assign8240_e7486: f64 = (p.p634 * var_iae);
        let assign8240_e7487: f64 = (assign8240_e7483 + assign8240_e7486);
        (assign8240_e7487,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign8240_e7489;

        let assign8250_e7508: f64 = if (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]) { 1.0 } else { 0.0 };
        var_guard102 = assign8250_e7508;

        let (assign8260_e7528,) = {
    if ((var_guard41 != 0.0) && (var_guard102 != 0.0)) {
        let assign8260_e7516: f64 = (p.p636 * var_ile);
        let assign8260_e7517: f64 = (p.p635 + assign8260_e7516);
        let assign8260_e7520: f64 = (p.p637 * var_iwe);
        let assign8260_e7521: f64 = (assign8260_e7517 + assign8260_e7520);
        let assign8260_e7524: f64 = (p.p638 * var_iae);
        let assign8260_e7525: f64 = (assign8260_e7521 + assign8260_e7524);
        let assign8260_e7526: f64 = (var_iiwe * assign8260_e7525);
        (assign8260_e7526,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign8260_e7528;

        let assign8270_e7547: f64 = if (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]) { 1.0 } else { 0.0 };
        var_guard103 = assign8270_e7547;

        let (assign8280_e7567,) = {
    if ((var_guard41 != 0.0) && (var_guard103 != 0.0)) {
        let assign8280_e7555: f64 = (p.p640 * var_ile);
        let assign8280_e7556: f64 = (p.p639 + assign8280_e7555);
        let assign8280_e7559: f64 = (p.p641 * var_iwe);
        let assign8280_e7560: f64 = (assign8280_e7556 + assign8280_e7559);
        let assign8280_e7563: f64 = (p.p642 * var_iae);
        let assign8280_e7564: f64 = (assign8280_e7560 + assign8280_e7563);
        let assign8280_e7565: f64 = (var_iiwe * assign8280_e7564);
        (assign8280_e7565,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign8280_e7567;

        let assign8290_e7586: f64 = if (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]) { 1.0 } else { 0.0 };
        var_guard104 = assign8290_e7586;

        let (assign8300_e7604,) = {
    if ((var_guard41 != 0.0) && (var_guard104 != 0.0)) {
        let assign8300_e7593: f64 = (p.p644 * var_ile);
        let assign8300_e7594: f64 = (p.p643 + assign8300_e7593);
        let assign8300_e7597: f64 = (p.p645 * var_iwe);
        let assign8300_e7598: f64 = (assign8300_e7594 + assign8300_e7597);
        let assign8300_e7601: f64 = (p.p646 * var_iae);
        let assign8300_e7602: f64 = (assign8300_e7598 + assign8300_e7601);
        (assign8300_e7602,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign8300_e7604;

        let assign8310_e7623: f64 = if (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]) { 1.0 } else { 0.0 };
        var_guard105 = assign8310_e7623;

        let (assign8320_e7641,) = {
    if ((var_guard41 != 0.0) && (var_guard105 != 0.0)) {
        let assign8320_e7630: f64 = (p.p648 * var_ile);
        let assign8320_e7631: f64 = (p.p647 + assign8320_e7630);
        let assign8320_e7634: f64 = (p.p649 * var_iwe);
        let assign8320_e7635: f64 = (assign8320_e7631 + assign8320_e7634);
        let assign8320_e7638: f64 = (p.p650 * var_iae);
        let assign8320_e7639: f64 = (assign8320_e7635 + assign8320_e7638);
        (assign8320_e7639,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign8320_e7641;

        let assign8330_e7660: f64 = if (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]) { 1.0 } else { 0.0 };
        var_guard106 = assign8330_e7660;

        let (assign8340_e7684,) = {
    if ((var_guard41 != 0.0) && (var_guard106 != 0.0)) {
        let assign8340_e7666: f64 = (var_iiwecv * var_lecv);
        let assign8340_e7668: f64 = (assign8340_e7666 / 1e-6);
        let assign8340_e7672: f64 = (p.p652 * var_ile);
        let assign8340_e7673: f64 = (p.p651 + assign8340_e7672);
        let assign8340_e7676: f64 = (p.p653 * var_iwe);
        let assign8340_e7677: f64 = (assign8340_e7673 + assign8340_e7676);
        let assign8340_e7680: f64 = (p.p654 * var_iae);
        let assign8340_e7681: f64 = (assign8340_e7677 + assign8340_e7680);
        let assign8340_e7682: f64 = (assign8340_e7668 * assign8340_e7681);
        (assign8340_e7682,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign8340_e7684;

        let assign8350_e7703: f64 = if (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]) { 1.0 } else { 0.0 };
        var_guard107 = assign8350_e7703;

        let (assign8360_e7721,) = {
    if ((var_guard41 != 0.0) && (var_guard107 != 0.0)) {
        let assign8360_e7710: f64 = (p.p656 * var_ile);
        let assign8360_e7711: f64 = (p.p655 + assign8360_e7710);
        let assign8360_e7714: f64 = (p.p657 * var_iwe);
        let assign8360_e7715: f64 = (assign8360_e7711 + assign8360_e7714);
        let assign8360_e7718: f64 = (p.p658 * var_iae);
        let assign8360_e7719: f64 = (assign8360_e7715 + assign8360_e7718);
        (assign8360_e7719,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign8360_e7721;

        let assign8370_e7740: f64 = if (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]) { 1.0 } else { 0.0 };
        var_guard108 = assign8370_e7740;

        let (assign8380_e7758,) = {
    if ((var_guard41 != 0.0) && (var_guard108 != 0.0)) {
        let assign8380_e7747: f64 = (p.p660 * var_ile);
        let assign8380_e7748: f64 = (p.p659 + assign8380_e7747);
        let assign8380_e7751: f64 = (p.p661 * var_iwe);
        let assign8380_e7752: f64 = (assign8380_e7748 + assign8380_e7751);
        let assign8380_e7755: f64 = (p.p662 * var_iae);
        let assign8380_e7756: f64 = (assign8380_e7752 + assign8380_e7755);
        (assign8380_e7756,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign8380_e7758;

        let assign8390_e7797: f64 = if (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        var_guard109 = assign8390_e7797;

        let (assign8400_e7803,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        (p.p571,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8400_e7803;

        let assign8410_e7805: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8410_e7807: f64 = if assign8410_e7805 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign8410_e7807;

        let (assign8420_e7815,) = {
    if (((var_guard41 != 0.0) && (var_guard109 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p663,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8420_e7815;

        let (assign8430_e7821,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        (p.p572,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8430_e7821;

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
        *var_guard106_slot = var_guard106;
        *var_guard107_slot = var_guard107;
        *var_guard108_slot = var_guard108;
        *var_guard109_slot = var_guard109;
        *var_guard110_slot = var_guard110;
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
        *var_plparam_i_slot = var_plparam_i;
        *var_poparam_i_slot = var_poparam_i;
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
        var_guard109: f64,
        var_guard41: f64,
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
        var_guard124_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_plparam_i_slot: &mut f64,
        var_plwparam_i_slot: &mut f64,
        var_poparam_i_slot: &mut f64,
        var_pwparam_i_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
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
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_plparam_i: f64 = *var_plparam_i_slot;
        let mut var_plwparam_i: f64 = *var_plwparam_i_slot;
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
        let mut var_pwparam_i: f64 = *var_pwparam_i_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;

        let assign8440_e7823: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8440_e7825: f64 = if assign8440_e7823 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign8440_e7825;

        let (assign8450_e7833,) = {
    if (((var_guard41 != 0.0) && (var_guard109 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p664,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8450_e7833;

        let (assign8460_e7839,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        (p.p573,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8460_e7839;

        let assign8470_e7841: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8470_e7843: f64 = if assign8470_e7841 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign8470_e7843;

        let (assign8480_e7851,) = {
    if (((var_guard41 != 0.0) && (var_guard109 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p665,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8480_e7851;

        let (assign8490_e7857,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        (p.p574,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8490_e7857;

        let assign8500_e7859: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8500_e7861: f64 = if assign8500_e7859 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign8500_e7861;

        let (assign8510_e7869,) = {
    if (((var_guard41 != 0.0) && (var_guard109 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p666,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8510_e7869;

        let (assign8520_e7889,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        let assign8520_e7877: f64 = (var_plparam_i * var_ile);
        let assign8520_e7878: f64 = (var_poparam_i + assign8520_e7877);
        let assign8520_e7881: f64 = (var_pwparam_i * var_iwe);
        let assign8520_e7882: f64 = (assign8520_e7878 + assign8520_e7881);
        let assign8520_e7885: f64 = (var_plwparam_i * var_iae);
        let assign8520_e7886: f64 = (assign8520_e7882 + assign8520_e7885);
        let assign8520_e7887: f64 = (var_ile * assign8520_e7886);
        (assign8520_e7887,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign8520_e7889;

        let assign8530_e7928: f64 = if (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        var_guard114 = assign8530_e7928;

        let (assign8540_e7934,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        (p.p587,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8540_e7934;

        let assign8550_e7936: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8550_e7938: f64 = if assign8550_e7936 == 1.0 { 1.0 } else { 0.0 };
        var_guard115 = assign8550_e7938;

        let (assign8560_e7946,) = {
    if (((var_guard41 != 0.0) && (var_guard114 != 0.0)) && (var_guard115 != 0.0)) {
        (p.p667,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8560_e7946;

        let (assign8570_e7952,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        (p.p588,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8570_e7952;

        let assign8580_e7954: f64 = if param_given[668] { 1.0 } else { 0.0 };
        let assign8580_e7956: f64 = if assign8580_e7954 == 1.0 { 1.0 } else { 0.0 };
        var_guard116 = assign8580_e7956;

        let (assign8590_e7964,) = {
    if (((var_guard41 != 0.0) && (var_guard114 != 0.0)) && (var_guard116 != 0.0)) {
        (p.p668,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8590_e7964;

        let (assign8600_e7970,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        (p.p589,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8600_e7970;

        let assign8610_e7972: f64 = if param_given[669] { 1.0 } else { 0.0 };
        let assign8610_e7974: f64 = if assign8610_e7972 == 1.0 { 1.0 } else { 0.0 };
        var_guard117 = assign8610_e7974;

        let (assign8620_e7982,) = {
    if (((var_guard41 != 0.0) && (var_guard114 != 0.0)) && (var_guard117 != 0.0)) {
        (p.p669,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8620_e7982;

        let (assign8630_e7988,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        (p.p590,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8630_e7988;

        let assign8640_e7990: f64 = if param_given[670] { 1.0 } else { 0.0 };
        let assign8640_e7992: f64 = if assign8640_e7990 == 1.0 { 1.0 } else { 0.0 };
        var_guard118 = assign8640_e7992;

        let (assign8650_e8000,) = {
    if (((var_guard41 != 0.0) && (var_guard114 != 0.0)) && (var_guard118 != 0.0)) {
        (p.p670,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8650_e8000;

        let (assign8660_e8020,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        let assign8660_e8008: f64 = (var_plparam_i * var_ile);
        let assign8660_e8009: f64 = (var_poparam_i + assign8660_e8008);
        let assign8660_e8012: f64 = (var_pwparam_i * var_iwe);
        let assign8660_e8013: f64 = (assign8660_e8009 + assign8660_e8012);
        let assign8660_e8016: f64 = (var_plwparam_i * var_iae);
        let assign8660_e8017: f64 = (assign8660_e8013 + assign8660_e8016);
        let assign8660_e8018: f64 = assign8660_e8017;
        (assign8660_e8018,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign8660_e8020;

        let assign8670_e8039: f64 = if (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]) { 1.0 } else { 0.0 };
        var_guard119 = assign8670_e8039;

        let (assign8680_e8059,) = {
    if ((var_guard41 != 0.0) && (var_guard119 != 0.0)) {
        let assign8680_e8047: f64 = (p.p672 * var_ile);
        let assign8680_e8048: f64 = (p.p671 + assign8680_e8047);
        let assign8680_e8051: f64 = (p.p673 * var_iwe);
        let assign8680_e8052: f64 = (assign8680_e8048 + assign8680_e8051);
        let assign8680_e8055: f64 = (p.p674 * var_iae);
        let assign8680_e8056: f64 = (assign8680_e8052 + assign8680_e8055);
        let assign8680_e8057: f64 = (var_ile * assign8680_e8056);
        (assign8680_e8057,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign8680_e8059;

        let assign8690_e8078: f64 = if (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]) { 1.0 } else { 0.0 };
        var_guard120 = assign8690_e8078;

        let (assign8700_e8098,) = {
    if ((var_guard41 != 0.0) && (var_guard120 != 0.0)) {
        let assign8700_e8086: f64 = (p.p676 * var_ile);
        let assign8700_e8087: f64 = (p.p675 + assign8700_e8086);
        let assign8700_e8090: f64 = (p.p677 * var_iwe);
        let assign8700_e8091: f64 = (assign8700_e8087 + assign8700_e8090);
        let assign8700_e8094: f64 = (p.p678 * var_iae);
        let assign8700_e8095: f64 = (assign8700_e8091 + assign8700_e8094);
        let assign8700_e8096: f64 = (var_ile * assign8700_e8095);
        (assign8700_e8096,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign8700_e8098;

        let assign8710_e8117: f64 = if (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]) { 1.0 } else { 0.0 };
        var_guard121 = assign8710_e8117;

        let (assign8720_e8137,) = {
    if ((var_guard41 != 0.0) && (var_guard121 != 0.0)) {
        let assign8720_e8125: f64 = (p.p680 * var_ile);
        let assign8720_e8126: f64 = (p.p679 + assign8720_e8125);
        let assign8720_e8129: f64 = (p.p681 * var_iwe);
        let assign8720_e8130: f64 = (assign8720_e8126 + assign8720_e8129);
        let assign8720_e8133: f64 = (p.p682 * var_iae);
        let assign8720_e8134: f64 = (assign8720_e8130 + assign8720_e8133);
        let assign8720_e8135: f64 = (var_iiwecv * assign8720_e8134);
        (assign8720_e8135,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign8720_e8137;

        let assign8730_e8156: f64 = if (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]) { 1.0 } else { 0.0 };
        var_guard122 = assign8730_e8156;

        let (assign8740_e8176,) = {
    if ((var_guard41 != 0.0) && (var_guard122 != 0.0)) {
        let assign8740_e8164: f64 = (p.p684 * var_ile);
        let assign8740_e8165: f64 = (p.p683 + assign8740_e8164);
        let assign8740_e8168: f64 = (p.p685 * var_iwe);
        let assign8740_e8169: f64 = (assign8740_e8165 + assign8740_e8168);
        let assign8740_e8172: f64 = (p.p686 * var_iae);
        let assign8740_e8173: f64 = (assign8740_e8169 + assign8740_e8172);
        let assign8740_e8174: f64 = (var_iiwecv * assign8740_e8173);
        (assign8740_e8174,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign8740_e8176;

        let assign8750_e8195: f64 = if (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]) { 1.0 } else { 0.0 };
        var_guard123 = assign8750_e8195;

        let (assign8760_e8215,) = {
    if ((var_guard41 != 0.0) && (var_guard123 != 0.0)) {
        let assign8760_e8203: f64 = (p.p688 * var_ile);
        let assign8760_e8204: f64 = (p.p687 + assign8760_e8203);
        let assign8760_e8207: f64 = (p.p689 * var_iwe);
        let assign8760_e8208: f64 = (assign8760_e8204 + assign8760_e8207);
        let assign8760_e8211: f64 = (p.p690 * var_iae);
        let assign8760_e8212: f64 = (assign8760_e8208 + assign8760_e8211);
        let assign8760_e8213: f64 = (var_iilcv * assign8760_e8212);
        (assign8760_e8213,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign8760_e8215;

        let assign8770_e8234: f64 = if (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]) { 1.0 } else { 0.0 };
        var_guard124 = assign8770_e8234;

        let (assign8780_e8254,) = {
    if ((var_guard41 != 0.0) && (var_guard124 != 0.0)) {
        let assign8780_e8242: f64 = (p.p692 * var_ile);
        let assign8780_e8243: f64 = (p.p691 + assign8780_e8242);
        let assign8780_e8246: f64 = (p.p693 * var_iwe);
        let assign8780_e8247: f64 = (assign8780_e8243 + assign8780_e8246);
        let assign8780_e8250: f64 = (p.p694 * var_iae);
        let assign8780_e8251: f64 = (assign8780_e8247 + assign8780_e8250);
        let assign8780_e8252: f64 = (var_iiwecv * assign8780_e8251);
        (assign8780_e8252,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign8780_e8254;

        let assign8790_e8273: f64 = if (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]) { 1.0 } else { 0.0 };
        var_guard125 = assign8790_e8273;

        let (assign8800_e8293,) = {
    if ((var_guard41 != 0.0) && (var_guard125 != 0.0)) {
        let assign8800_e8281: f64 = (p.p696 * var_ile);
        let assign8800_e8282: f64 = (p.p695 + assign8800_e8281);
        let assign8800_e8285: f64 = (p.p697 * var_iwe);
        let assign8800_e8286: f64 = (assign8800_e8282 + assign8800_e8285);
        let assign8800_e8289: f64 = (p.p698 * var_iae);
        let assign8800_e8290: f64 = (assign8800_e8286 + assign8800_e8289);
        let assign8800_e8291: f64 = (var_iiwecv * assign8800_e8290);
        (assign8800_e8291,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign8800_e8293;

        let assign8810_e8312: f64 = if (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]) { 1.0 } else { 0.0 };
        var_guard126 = assign8810_e8312;

        let (assign8820_e8332,) = {
    if ((var_guard41 != 0.0) && (var_guard126 != 0.0)) {
        let assign8820_e8320: f64 = (p.p700 * var_ile);
        let assign8820_e8321: f64 = (p.p699 + assign8820_e8320);
        let assign8820_e8324: f64 = (p.p701 * var_iwe);
        let assign8820_e8325: f64 = (assign8820_e8321 + assign8820_e8324);
        let assign8820_e8328: f64 = (p.p702 * var_iae);
        let assign8820_e8329: f64 = (assign8820_e8325 + assign8820_e8328);
        let assign8820_e8330: f64 = (var_iiwcv * assign8820_e8329);
        (assign8820_e8330,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign8820_e8332;

        let assign8830_e8351: f64 = if (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]) { 1.0 } else { 0.0 };
        var_guard127 = assign8830_e8351;

        let (assign8840_e8371,) = {
    if ((var_guard41 != 0.0) && (var_guard127 != 0.0)) {
        let assign8840_e8359: f64 = (p.p704 * var_ile);
        let assign8840_e8360: f64 = (p.p703 + assign8840_e8359);
        let assign8840_e8363: f64 = (p.p705 * var_iwe);
        let assign8840_e8364: f64 = (assign8840_e8360 + assign8840_e8363);
        let assign8840_e8367: f64 = (p.p706 * var_iae);
        let assign8840_e8368: f64 = (assign8840_e8364 + assign8840_e8367);
        let assign8840_e8369: f64 = (var_iiwcv * assign8840_e8368);
        (assign8840_e8369,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign8840_e8371;

        let assign8850_e8390: f64 = if (((param_given[707] || param_given[708]) || param_given[709]) || param_given[710]) { 1.0 } else { 0.0 };
        var_guard128 = assign8850_e8390;

        let (assign8860_e8410,) = {
    if ((var_guard41 != 0.0) && (var_guard128 != 0.0)) {
        let assign8860_e8398: f64 = (p.p708 * var_ile);
        let assign8860_e8399: f64 = (p.p707 + assign8860_e8398);
        let assign8860_e8402: f64 = (p.p709 * var_iwe);
        let assign8860_e8403: f64 = (assign8860_e8399 + assign8860_e8402);
        let assign8860_e8406: f64 = (p.p710 * var_iae);
        let assign8860_e8407: f64 = (assign8860_e8403 + assign8860_e8406);
        let assign8860_e8408: f64 = (var_ile2 * assign8860_e8407);
        (assign8860_e8408,)
    } else {
        (var_fntexc_p,)
    }
};
        var_fntexc_p = assign8860_e8410;

        let assign8930_e8546: f64 = if (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]) { 1.0 } else { 0.0 };
        var_guard132 = assign8930_e8546;

        let (assign8940_e8564,) = {
    if ((var_guard41 != 0.0) && (var_guard132 != 0.0)) {
        let assign8940_e8553: f64 = (p.p724 * var_ile);
        let assign8940_e8554: f64 = (p.p723 + assign8940_e8553);
        let assign8940_e8557: f64 = (p.p725 * var_iwe);
        let assign8940_e8558: f64 = (assign8940_e8554 + assign8940_e8557);
        let assign8940_e8561: f64 = (p.p726 * var_iae);
        let assign8940_e8562: f64 = (assign8940_e8558 + assign8940_e8561);
        (assign8940_e8562,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign8940_e8564;

        let assign8950_e8583: f64 = if (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]) { 1.0 } else { 0.0 };
        var_guard133 = assign8950_e8583;

        let (assign8960_e8601,) = {
    if ((var_guard41 != 0.0) && (var_guard133 != 0.0)) {
        let assign8960_e8590: f64 = (p.p728 * var_ile);
        let assign8960_e8591: f64 = (p.p727 + assign8960_e8590);
        let assign8960_e8594: f64 = (p.p729 * var_iwe);
        let assign8960_e8595: f64 = (assign8960_e8591 + assign8960_e8594);
        let assign8960_e8598: f64 = (p.p730 * var_iae);
        let assign8960_e8599: f64 = (assign8960_e8595 + assign8960_e8598);
        (assign8960_e8599,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign8960_e8601;

        let assign8970_e8620: f64 = if (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]) { 1.0 } else { 0.0 };
        var_guard134 = assign8970_e8620;

        let (assign8980_e8638,) = {
    if ((var_guard41 != 0.0) && (var_guard134 != 0.0)) {
        let assign8980_e8627: f64 = (p.p732 * var_ile);
        let assign8980_e8628: f64 = (p.p731 + assign8980_e8627);
        let assign8980_e8631: f64 = (p.p733 * var_iwe);
        let assign8980_e8632: f64 = (assign8980_e8628 + assign8980_e8631);
        let assign8980_e8635: f64 = (p.p734 * var_iae);
        let assign8980_e8636: f64 = (assign8980_e8632 + assign8980_e8635);
        (assign8980_e8636,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign8980_e8638;

        let assign8990_e8657: f64 = if (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]) { 1.0 } else { 0.0 };
        var_guard135 = assign8990_e8657;

        let (assign9000_e8675,) = {
    if ((var_guard41 != 0.0) && (var_guard135 != 0.0)) {
        let assign9000_e8664: f64 = (p.p736 * var_ile);
        let assign9000_e8665: f64 = (p.p735 + assign9000_e8664);
        let assign9000_e8668: f64 = (p.p737 * var_iwe);
        let assign9000_e8669: f64 = (assign9000_e8665 + assign9000_e8668);
        let assign9000_e8672: f64 = (p.p738 * var_iae);
        let assign9000_e8673: f64 = (assign9000_e8669 + assign9000_e8672);
        (assign9000_e8673,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign9000_e8675;

        let assign9010_e8694: f64 = if (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]) { 1.0 } else { 0.0 };
        var_guard136 = assign9010_e8694;

        let (assign9020_e8712,) = {
    if ((var_guard41 != 0.0) && (var_guard136 != 0.0)) {
        let assign9020_e8701: f64 = (p.p740 * var_ile);
        let assign9020_e8702: f64 = (p.p739 + assign9020_e8701);
        let assign9020_e8705: f64 = (p.p741 * var_iwe);
        let assign9020_e8706: f64 = (assign9020_e8702 + assign9020_e8705);
        let assign9020_e8709: f64 = (p.p742 * var_iae);
        let assign9020_e8710: f64 = (assign9020_e8706 + assign9020_e8709);
        (assign9020_e8710,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign9020_e8712;

        let assign9030_e8731: f64 = if (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]) { 1.0 } else { 0.0 };
        var_guard137 = assign9030_e8731;

        let (assign9040_e8753,) = {
    if ((var_guard41 != 0.0) && (var_guard137 != 0.0)) {
        let assign9040_e8737: f64 = (var_we_edge / var_le);
        let assign9040_e8741: f64 = (p.p744 * var_ile);
        let assign9040_e8742: f64 = (p.p743 + assign9040_e8741);
        let assign9040_e8745: f64 = (p.p745 * var_iwe);
        let assign9040_e8746: f64 = (assign9040_e8742 + assign9040_e8745);
        let assign9040_e8749: f64 = (p.p746 * var_iae);
        let assign9040_e8750: f64 = (assign9040_e8746 + assign9040_e8749);
        let assign9040_e8751: f64 = (assign9040_e8737 * assign9040_e8750);
        (assign9040_e8751,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9040_e8753;

        let assign9050_e8772: f64 = if (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]) { 1.0 } else { 0.0 };
        var_guard138 = assign9050_e8772;

        let (assign9060_e8790,) = {
    if ((var_guard41 != 0.0) && (var_guard138 != 0.0)) {
        let assign9060_e8779: f64 = (p.p748 * var_ile);
        let assign9060_e8780: f64 = (p.p747 + assign9060_e8779);
        let assign9060_e8783: f64 = (p.p749 * var_iwe);
        let assign9060_e8784: f64 = (assign9060_e8780 + assign9060_e8783);
        let assign9060_e8787: f64 = (p.p750 * var_iae);
        let assign9060_e8788: f64 = (assign9060_e8784 + assign9060_e8787);
        (assign9060_e8788,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign9060_e8790;

        let assign9070_e8809: f64 = if (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]) { 1.0 } else { 0.0 };
        var_guard139 = assign9070_e8809;

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
        *var_guard124_slot = var_guard124;
        *var_guard125_slot = var_guard125;
        *var_guard126_slot = var_guard126;
        *var_guard127_slot = var_guard127;
        *var_guard128_slot = var_guard128;
        *var_guard132_slot = var_guard132;
        *var_guard133_slot = var_guard133;
        *var_guard134_slot = var_guard134;
        *var_guard135_slot = var_guard135;
        *var_guard136_slot = var_guard136;
        *var_guard137_slot = var_guard137;
        *var_guard138_slot = var_guard138;
        *var_guard139_slot = var_guard139;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_plparam_i_slot = var_plparam_i;
        *var_plwparam_i_slot = var_plwparam_i;
        *var_poparam_i_slot = var_poparam_i;
        *var_pwparam_i_slot = var_pwparam_i;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_vfbedge_p_slot = var_vfbedge_p;
    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dellps: f64,
        var_delwod: f64,
        var_guard139: f64,
        var_guard41: f64,
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
        var_guard140_slot: &mut f64,
        var_guard141_slot: &mut f64,
        var_guard142_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_kvsatac_i_slot: &mut f64,
        var_loop__slot: &mut f64,
        var_lx_slot: &mut f64,
        var_munqs_p_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
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
        let mut var_guard140: f64 = *var_guard140_slot;
        let mut var_guard141: f64 = *var_guard141_slot;
        let mut var_guard142: f64 = *var_guard142_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_kvsatac_i: f64 = *var_kvsatac_i_slot;
        let mut var_loop_: f64 = *var_loop__slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_munqs_p: f64 = *var_munqs_p_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
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

        let (assign9080_e8829,) = {
    if ((var_guard41 != 0.0) && (var_guard139 != 0.0)) {
        let assign9080_e8817: f64 = (p.p752 * var_ile);
        let assign9080_e8818: f64 = (p.p751 + assign9080_e8817);
        let assign9080_e8821: f64 = (p.p753 * var_iwe);
        let assign9080_e8822: f64 = (assign9080_e8818 + assign9080_e8821);
        let assign9080_e8825: f64 = (p.p754 * var_iae);
        let assign9080_e8826: f64 = (assign9080_e8822 + assign9080_e8825);
        let assign9080_e8827: f64 = (var_ile2 * assign9080_e8826);
        (assign9080_e8827,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign9080_e8829;

        let assign9090_e8848: f64 = if (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]) { 1.0 } else { 0.0 };
        var_guard140 = assign9090_e8848;

        let (assign9100_e8866,) = {
    if ((var_guard41 != 0.0) && (var_guard140 != 0.0)) {
        let assign9100_e8855: f64 = (p.p756 * var_ile);
        let assign9100_e8856: f64 = (p.p755 + assign9100_e8855);
        let assign9100_e8859: f64 = (p.p757 * var_iwe);
        let assign9100_e8860: f64 = (assign9100_e8856 + assign9100_e8859);
        let assign9100_e8863: f64 = (p.p758 * var_iae);
        let assign9100_e8864: f64 = (assign9100_e8860 + assign9100_e8863);
        (assign9100_e8864,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign9100_e8866;

        let assign9110_e8885: f64 = if (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]) { 1.0 } else { 0.0 };
        var_guard141 = assign9110_e8885;

        let (assign9120_e8903,) = {
    if ((var_guard41 != 0.0) && (var_guard141 != 0.0)) {
        let assign9120_e8892: f64 = (p.p760 * var_ile);
        let assign9120_e8893: f64 = (p.p759 + assign9120_e8892);
        let assign9120_e8896: f64 = (p.p761 * var_iwe);
        let assign9120_e8897: f64 = (assign9120_e8893 + assign9120_e8896);
        let assign9120_e8900: f64 = (p.p762 * var_iae);
        let assign9120_e8901: f64 = (assign9120_e8897 + assign9120_e8900);
        (assign9120_e8901,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign9120_e8903;

        let assign9130_e8922: f64 = if (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]) { 1.0 } else { 0.0 };
        var_guard142 = assign9130_e8922;

        let (assign9140_e8942,) = {
    if ((var_guard41 != 0.0) && (var_guard142 != 0.0)) {
        let assign9140_e8930: f64 = (p.p764 * var_ile);
        let assign9140_e8931: f64 = (p.p763 + assign9140_e8930);
        let assign9140_e8934: f64 = (p.p765 * var_iwe);
        let assign9140_e8935: f64 = (assign9140_e8931 + assign9140_e8934);
        let assign9140_e8938: f64 = (p.p766 * var_iae);
        let assign9140_e8939: f64 = (assign9140_e8935 + assign9140_e8938);
        let assign9140_e8940: f64 = (var_ile2 * assign9140_e8939);
        (assign9140_e8940,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign9140_e8942;

        let assign9150_e8961: f64 = if (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]) { 1.0 } else { 0.0 };
        var_guard143 = assign9150_e8961;

        let (assign9160_e8979,) = {
    if ((var_guard41 != 0.0) && (var_guard143 != 0.0)) {
        let assign9160_e8968: f64 = (p.p772 * var_ile);
        let assign9160_e8969: f64 = (p.p771 + assign9160_e8968);
        let assign9160_e8972: f64 = (p.p773 * var_iwe);
        let assign9160_e8973: f64 = (assign9160_e8969 + assign9160_e8972);
        let assign9160_e8976: f64 = (p.p774 * var_iae);
        let assign9160_e8977: f64 = (assign9160_e8973 + assign9160_e8976);
        (assign9160_e8977,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign9160_e8979;

        let assign9170_e8998: f64 = if (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]) { 1.0 } else { 0.0 };
        var_guard144 = assign9170_e8998;

        let (assign9180_e9016,) = {
    if ((var_guard41 != 0.0) && (var_guard144 != 0.0)) {
        let assign9180_e9005: f64 = (p.p768 * var_ile);
        let assign9180_e9006: f64 = (p.p767 + assign9180_e9005);
        let assign9180_e9009: f64 = (p.p769 * var_iwe);
        let assign9180_e9010: f64 = (assign9180_e9006 + assign9180_e9009);
        let assign9180_e9013: f64 = (p.p770 * var_iae);
        let assign9180_e9014: f64 = (assign9180_e9010 + assign9180_e9013);
        (assign9180_e9014,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign9180_e9016;

        let assign9250_e9152: f64 = if (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]) { 1.0 } else { 0.0 };
        var_guard148 = assign9250_e9152;

        let (assign9260_e9170,) = {
    if ((var_guard41 != 0.0) && (var_guard148 != 0.0)) {
        let assign9260_e9159: f64 = (p.p788 * var_ile);
        let assign9260_e9160: f64 = (p.p787 + assign9260_e9159);
        let assign9260_e9163: f64 = (p.p789 * var_iwe);
        let assign9260_e9164: f64 = (assign9260_e9160 + assign9260_e9163);
        let assign9260_e9167: f64 = (p.p790 * var_iae);
        let assign9260_e9168: f64 = (assign9260_e9164 + assign9260_e9167);
        (assign9260_e9168,)
    } else {
        (var_munqs_p,)
    }
};
        var_munqs_p = assign9260_e9170;

        let (assign9270_e9174,) = {
    if (var_guard41 != 0.0) {
        (0.0,)
    } else {
        (var_tmpa,)
    }
};
        var_tmpa = assign9270_e9174;

        let (assign9280_e9178,) = {
    if (var_guard41 != 0.0) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign9280_e9178;

        let (assign9290_e9182,) = {
    if (var_guard41 != 0.0) {
        (0.0,)
    } else {
        (var_loop_,)
    }
};
        var_loop_ = assign9290_e9182;

        let (assign9300_e9186,) = {
    if (var_guard41 != 0.0) {
        (p.p795,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9300_e9186;

        let assign9310_e9188: f64 = if param_given[796] { 1.0 } else { 0.0 };
        let assign9310_e9190: f64 = if assign9310_e9188 == 1.0 { 1.0 } else { 0.0 };
        var_guard149 = assign9310_e9190;

        let (assign9320_e9196,) = {
    if ((var_guard41 != 0.0) && (var_guard149 != 0.0)) {
        (p.p796,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9320_e9196;

        let assign9330_e9215: f64 = if (((var_sa_i > 0.0) && (var_sb_i > 0.0)) && ((var_nf_i == 1.0) || ((var_nf_i > 1.0) && (var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        var_guard150 = assign9330_e9215;

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (var_nf_i - 0.5);
            let assign9340_cond_e9224: f64 = if (((var_guard41 != 0.0) && (var_guard150 != 0.0)) && (var_loop_ < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9340_body0_e9244,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9340_body0_e9233: f64 = (0.5 * var_l_i);
        let assign9340_body0_e9234: f64 = (var_sa_i + assign9340_body0_e9233);
        let assign9340_body0_e9238: f64 = (var_sd_i + var_l_i);
        let assign9340_body0_e9239: f64 = (var_loop_ * assign9340_body0_e9238);
        let assign9340_body0_e9240: f64 = (assign9340_body0_e9234 + assign9340_body0_e9239);
        let assign9340_body0_e9241: f64 = (1.0 / assign9340_body0_e9240);
        let assign9340_body0_e9242: f64 = (var_tmpa + assign9340_body0_e9241);
        (assign9340_body0_e9242,)
    } else {
        (var_tmpa,)
    }
};
            var_tmpa = assign9340_body0_e9244;
            let (assign9340_body1_e9264,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9340_body1_e9253: f64 = (0.5 * var_l_i);
        let assign9340_body1_e9254: f64 = (var_sb_i + assign9340_body1_e9253);
        let assign9340_body1_e9258: f64 = (var_sd_i + var_l_i);
        let assign9340_body1_e9259: f64 = (var_loop_ * assign9340_body1_e9258);
        let assign9340_body1_e9260: f64 = (assign9340_body1_e9254 + assign9340_body1_e9259);
        let assign9340_body1_e9261: f64 = (1.0 / assign9340_body1_e9260);
        let assign9340_body1_e9262: f64 = (var_tmpb + assign9340_body1_e9261);
        (assign9340_body1_e9262,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign9340_body1_e9264;
            let (assign9340_body2_e9272,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9340_body2_e9270: f64 = (var_loop_ + 1.0);
        (assign9340_body2_e9270,)
    } else {
        (var_loop_,)
    }
};
            var_loop_ = assign9340_body2_e9272;
        }

        let (assign9350_e9280,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9350_e9278: f64 = (var_tmpa * var_invnf);
        (assign9350_e9278,)
    } else {
        (var_invsa,)
    }
};
        var_invsa = assign9350_e9280;

        let (assign9360_e9288,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9360_e9286: f64 = (var_tmpb * var_invnf);
        (assign9360_e9286,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign9360_e9288;

        let (assign9370_e9300,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9370_e9296: f64 = (0.5 * var_l_i);
        let assign9370_e9297: f64 = (p.p791 + assign9370_e9296);
        let assign9370_e9298: f64 = (1.0 / assign9370_e9297);
        (assign9370_e9298,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign9370_e9300;

        let (assign9380_e9312,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9380_e9308: f64 = (0.5 * var_l_i);
        let assign9380_e9309: f64 = (p.p792 + assign9380_e9308);
        let assign9380_e9310: f64 = (1.0 / assign9380_e9309);
        (assign9380_e9310,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign9380_e9312;

        let (assign9390_e9327,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9390_e9318: f64 = (var_l_i + var_dellps);
        let (assign9390_e9325,) = {
            if (assign9390_e9318 > 1e-9) {
                let assign9390_e9323: f64 = (var_l_i + var_dellps);
                (assign9390_e9323,)
            } else {
                (1e-9,)
            }
        };
        (assign9390_e9325,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign9390_e9327;

        let (assign9400_e9346,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9400_e9333: f64 = (var_w_i + var_delwod);
        let assign9400_e9335: f64 = (assign9400_e9333 + p.p793);
        let (assign9400_e9344,) = {
            if (assign9400_e9335 > 1e-9) {
                let assign9400_e9340: f64 = (var_w_i + var_delwod);
                let assign9400_e9342: f64 = (assign9400_e9340 + p.p793);
                (assign9400_e9342,)
            } else {
                (1e-9,)
            }
        };
        (assign9400_e9344,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign9400_e9346;

        let (assign9410_e9356,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9410_e9353: f64 = (var_lx).powf(p.p801);
        let assign9410_e9354: f64 = (1.0 / assign9410_e9353);
        (assign9410_e9354,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9410_e9356;

        let (assign9420_e9366,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9420_e9363: f64 = (var_wx).powf(p.p802);
        let assign9420_e9364: f64 = (1.0 / assign9420_e9363);
        (assign9420_e9364,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9420_e9366;

        let (assign9430_e9394,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9430_e9373: f64 = (p.p798 * var_templ);
        let assign9430_e9374: f64 = (1.0 + assign9430_e9373);
        let assign9430_e9377: f64 = (p.p799 * var_tempw);
        let assign9430_e9378: f64 = (assign9430_e9374 + assign9430_e9377);
        let assign9430_e9381: f64 = (p.p800 * var_templ);
        let assign9430_e9383: f64 = (assign9430_e9381 * var_tempw);
        let assign9430_e9384: f64 = (assign9430_e9378 + assign9430_e9383);
        let assign9430_e9389: f64 = (var_rta - 1.0);
        let assign9430_e9390: f64 = (p.p797 * assign9430_e9389);
        let assign9430_e9391: f64 = (1.0 + assign9430_e9390);
        let assign9430_e9392: f64 = (assign9430_e9384 * assign9430_e9391);
        (assign9430_e9392,)
    } else {
        (var_kstressu0,)
    }
};
        var_kstressu0 = assign9430_e9394;

        let (assign9440_e9406,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9440_e9401: f64 = (var_invsa + var_invsb);
        let assign9440_e9402: f64 = (p.p794 * assign9440_e9401);
        let assign9440_e9404: f64 = (assign9440_e9402 / var_kstressu0);
        (assign9440_e9404,)
    } else {
        (var_rhobeta,)
    }
};
        var_rhobeta = assign9440_e9406;

        let (assign9450_e9418,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9450_e9413: f64 = (var_invsaref + var_invsbref);
        let assign9450_e9414: f64 = (p.p794 * assign9450_e9413);
        let assign9450_e9416: f64 = (assign9450_e9414 / var_kstressu0);
        (assign9450_e9416,)
    } else {
        (var_rhobetaref,)
    }
};
        var_rhobetaref = assign9450_e9418;

        let (assign9460_e9428,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9460_e9425: f64 = (var_lx).powf(p.p807);
        let assign9460_e9426: f64 = (1.0 / assign9460_e9425);
        (assign9460_e9426,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9460_e9428;

        let (assign9470_e9438,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9470_e9435: f64 = (var_wx).powf(p.p808);
        let assign9470_e9436: f64 = (1.0 / assign9470_e9435);
        (assign9470_e9436,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9470_e9438;

        let (assign9480_e9458,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9480_e9445: f64 = (p.p804 * var_templ);
        let assign9480_e9446: f64 = (1.0 + assign9480_e9445);
        let assign9480_e9449: f64 = (p.p805 * var_tempw);
        let assign9480_e9450: f64 = (assign9480_e9446 + assign9480_e9449);
        let assign9480_e9453: f64 = (p.p806 * var_templ);
        let assign9480_e9455: f64 = (assign9480_e9453 * var_tempw);
        let assign9480_e9456: f64 = (assign9480_e9450 + assign9480_e9455);
        (assign9480_e9456,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign9480_e9458;

        let (assign9490_e9470,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9490_e9464: f64 = (var_invsa + var_invsb);
        let assign9490_e9466: f64 = (assign9490_e9464 - var_invsaref);
        let assign9490_e9468: f64 = (assign9490_e9466 - var_invsbref);
        (assign9490_e9468,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9490_e9470;

        let (assign9500_e9482,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9500_e9476: f64 = (1.0 + var_rhobeta);
        let assign9500_e9479: f64 = (1.0 + var_rhobetaref);
        let assign9500_e9480: f64 = (assign9500_e9476 / assign9500_e9479);
        (assign9500_e9480,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9500_e9482;

        let (assign9510_e9490,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9510_e9488: f64 = (var_betn_p * var_temp00);
        (assign9510_e9488,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9510_e9490;

        let (assign9520_e9510,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9520_e9496: f64 = (var_thesat_p * var_temp00);
        let assign9520_e9500: f64 = (p.p795 * var_rhobetaref);
        let assign9520_e9501: f64 = (1.0 + assign9520_e9500);
        let assign9520_e9502: f64 = (assign9520_e9496 * assign9520_e9501);
        let assign9520_e9506: f64 = (p.p795 * var_rhobeta);
        let assign9520_e9507: f64 = (1.0 + assign9520_e9506);
        let assign9520_e9508: f64 = (assign9520_e9502 / assign9520_e9507);
        (assign9520_e9508,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign9520_e9510;

        let (assign9530_e9530,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9530_e9516: f64 = (var_thesatac_p * var_temp00);
        let assign9530_e9520: f64 = (var_kvsatac_i * var_rhobetaref);
        let assign9530_e9521: f64 = (1.0 + assign9530_e9520);
        let assign9530_e9522: f64 = (assign9530_e9516 * assign9530_e9521);
        let assign9530_e9526: f64 = (var_kvsatac_i * var_rhobeta);
        let assign9530_e9527: f64 = (1.0 + assign9530_e9526);
        let assign9530_e9528: f64 = (assign9530_e9522 / assign9530_e9527);
        (assign9530_e9528,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign9530_e9530;

        let (assign9540_e9538,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9540_e9536: f64 = (var_betnedge_p * var_temp00);
        (assign9540_e9536,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9540_e9538;

        let (assign9550_e9548,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9550_e9544: f64 = (p.p803 * var_temp0);
        let assign9550_e9546: f64 = (assign9550_e9544 / var_kstressvth0);
        (assign9550_e9546,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9550_e9548;

        let (assign9560_e9556,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9560_e9554: f64 = (var_vfb_p + var_temp00);
        (assign9560_e9554,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9560_e9556;

        *var_betn_p_slot = var_betn_p;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_guard140_slot = var_guard140;
        *var_guard141_slot = var_guard141;
        *var_guard142_slot = var_guard142;
        *var_guard143_slot = var_guard143;
        *var_guard144_slot = var_guard144;
        *var_guard148_slot = var_guard148;
        *var_guard149_slot = var_guard149;
        *var_guard150_slot = var_guard150;
        *var_invsa_slot = var_invsa;
        *var_invsaref_slot = var_invsaref;
        *var_invsb_slot = var_invsb;
        *var_invsbref_slot = var_invsbref;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_kvsatac_i_slot = var_kvsatac_i;
        *var_loop__slot = var_loop_;
        *var_lx_slot = var_lx;
        *var_munqs_p_slot = var_munqs_p;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobetaref_slot = var_rhobetaref;
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
        var_guard150: f64,
        var_guard41: f64,
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
        var_guard151_slot: &mut f64,
        var_guard152_slot: &mut f64,
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
        let mut var_guard151: f64 = *var_guard151_slot;
        let mut var_guard152: f64 = *var_guard152_slot;
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

        let (assign9570_e9564,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9570_e9562: f64 = (var_vfbedge_p + var_temp00);
        (assign9570_e9562,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9570_e9564;

        let (assign9580_e9576,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9580_e9570: f64 = (p.p809 * var_temp0);
        let assign9580_e9573: f64 = (var_kstressvth0).powf(p.p810);
        let assign9580_e9574: f64 = (assign9580_e9570 / assign9580_e9573);
        (assign9580_e9574,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9580_e9576;

        let (assign9590_e9584,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9590_e9582: f64 = (var_cf_p + var_temp00);
        (assign9590_e9582,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign9590_e9584;

        let (assign9600_e9592,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9600_e9590: f64 = (var_cfedge_p + var_temp00);
        (assign9600_e9590,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign9600_e9592;

        let assign9610_e9607: f64 = if ((((var_sca_i > 0.0) || (var_scb_i > 0.0)) || (var_scc_i > 0.0)) || (var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard151 = assign9610_e9607;

        let assign9620_e9618: f64 = if (((var_sca_i == 0.0) && (var_scb_i == 0.0)) && (var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard152 = assign9620_e9618;

        let (assign9630_e9628,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9630_e9626: f64 = (var_sc_i + var_w_i);
        (assign9630_e9626,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9630_e9628;

        let (assign9640_e9638,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9640_e9636: f64 = (1.0 / p.p811);
        (assign9640_e9636,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9640_e9638;

        let (assign9650_e9652,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9650_e9646: f64 = (p.p811 * p.p811);
        let assign9650_e9649: f64 = (var_sc_i * var_temp0);
        let assign9650_e9650: f64 = (assign9650_e9646 / assign9650_e9649);
        (assign9650_e9650,)
    } else {
        (var_sca_i,)
    }
};
        var_sca_i = assign9650_e9652;

        let (assign9660_e9692,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9660_e9660: f64 = (0.1 * var_sc_i);
        let assign9660_e9663: f64 = (0.01 * p.p811);
        let assign9660_e9664: f64 = (assign9660_e9660 + assign9660_e9663);
        let assign9660_e9666: f64 = (-10.0);
        let assign9660_e9668: f64 = (assign9660_e9666 * var_sc_i);
        let assign9660_e9670: f64 = (assign9660_e9668 * var_temp00);
        let assign9660_e9671: f64 = (assign9660_e9670).exp();
        let assign9660_e9672: f64 = (assign9660_e9664 * assign9660_e9671);
        let assign9660_e9675: f64 = (0.1 * var_temp0);
        let assign9660_e9678: f64 = (0.01 * p.p811);
        let assign9660_e9679: f64 = (assign9660_e9675 + assign9660_e9678);
        let assign9660_e9681: f64 = (-10.0);
        let assign9660_e9683: f64 = (assign9660_e9681 * var_temp0);
        let assign9660_e9685: f64 = (assign9660_e9683 * var_temp00);
        let assign9660_e9686: f64 = (assign9660_e9685).exp();
        let assign9660_e9687: f64 = (assign9660_e9679 * assign9660_e9686);
        let assign9660_e9688: f64 = (assign9660_e9672 - assign9660_e9687);
        let assign9660_e9690: f64 = (assign9660_e9688 / var_w_i);
        (assign9660_e9690,)
    } else {
        (var_scb_i,)
    }
};
        var_scb_i = assign9660_e9692;

        let (assign9670_e9732,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9670_e9700: f64 = (0.05 * var_sc_i);
        let assign9670_e9703: f64 = (0.0025 * p.p811);
        let assign9670_e9704: f64 = (assign9670_e9700 + assign9670_e9703);
        let assign9670_e9706: f64 = (-20.0);
        let assign9670_e9708: f64 = (assign9670_e9706 * var_sc_i);
        let assign9670_e9710: f64 = (assign9670_e9708 * var_temp00);
        let assign9670_e9711: f64 = (assign9670_e9710).exp();
        let assign9670_e9712: f64 = (assign9670_e9704 * assign9670_e9711);
        let assign9670_e9715: f64 = (0.05 * var_temp0);
        let assign9670_e9718: f64 = (0.0025 * p.p811);
        let assign9670_e9719: f64 = (assign9670_e9715 + assign9670_e9718);
        let assign9670_e9721: f64 = (-20.0);
        let assign9670_e9723: f64 = (assign9670_e9721 * var_temp0);
        let assign9670_e9725: f64 = (assign9670_e9723 * var_temp00);
        let assign9670_e9726: f64 = (assign9670_e9725).exp();
        let assign9670_e9727: f64 = (assign9670_e9719 * assign9670_e9726);
        let assign9670_e9728: f64 = (assign9670_e9712 - assign9670_e9727);
        let assign9670_e9730: f64 = (assign9670_e9728 / var_w_i);
        (assign9670_e9730,)
    } else {
        (var_scc_i,)
    }
};
        var_scc_i = assign9670_e9732;

        let (assign9680_e9746,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9680_e9739: f64 = (p.p812 * var_scb_i);
        let assign9680_e9740: f64 = (var_sca_i + assign9680_e9739);
        let assign9680_e9743: f64 = (p.p813 * var_scc_i);
        let assign9680_e9744: f64 = (assign9680_e9740 + assign9680_e9743);
        (assign9680_e9744,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9680_e9746;

        let (assign9690_e9756,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9690_e9753: f64 = (var_kvthowe * var_temp0);
        let assign9690_e9754: f64 = (var_vfb_p + assign9690_e9753);
        (assign9690_e9754,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9690_e9756;

        let (assign9700_e9768,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9700_e9764: f64 = (var_kuowe * var_temp0);
        let assign9700_e9765: f64 = (1.0 + assign9700_e9764);
        let assign9700_e9766: f64 = (var_betn_p * assign9700_e9765);
        (assign9700_e9766,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9700_e9768;

        let (assign9710_e9778,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9710_e9775: f64 = (var_kvthowe * var_temp0);
        let assign9710_e9776: f64 = (var_vfbedge_p + assign9710_e9775);
        (assign9710_e9776,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9710_e9778;

        let (assign9720_e9790,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9720_e9786: f64 = (var_kuowe * var_temp0);
        let assign9720_e9787: f64 = (1.0 + assign9720_e9786);
        let assign9720_e9788: f64 = (var_betnedge_p * assign9720_e9787);
        (assign9720_e9788,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9720_e9790;

        var_vfb_i = var_vfb_p;

        var_stvfb_i = var_stvfb_p;

        var_st2vfb_i = var_st2vfb_p;

        var_tox_i = var_tox_p;

        var_epsrox_i = var_epsrox_p;

        let (assign9780_e9806,) = {
    if (var_neff_p > 1e20) {
        let (assign9780_e9804,) = {
            if (var_neff_p < 1e26) {
                (var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9780_e9804,)
    } else {
        (1e20,)
    }
};
        var_neff_i = assign9780_e9806;

        let (assign9790_e9812,) = {
    if (var_gfacnud_p > 0.01) {
        (var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        var_gfacnud_i = assign9790_e9812;

        let (assign9800_e9818,) = {
    if (var_vsbnud_p > 0.0) {
        (var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        var_vsbnud_i = assign9800_e9818;

        var_dvsbnud_i = var_dvsbnud_p;

        var_dphib_i = var_dphib_p;

        let (assign9830_e9826,) = {
    if (var_np_p > 0.0) {
        (var_np_p,)
    } else {
        (0.0,)
    }
};
        var_np_i = assign9830_e9826;

        var_toxov_i = var_toxov_p;

        var_toxovd_i = var_toxovd_p;

        let (assign9860_e9839,) = {
    if (var_nov_p > 1e23) {
        let (assign9860_e9837,) = {
            if (var_nov_p < 1e27) {
                (var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9860_e9837,)
    } else {
        (1e23,)
    }
};
        var_nov_i = assign9860_e9839;

        let (assign9870_e9850,) = {
    if (var_novd_p > 1e23) {
        let (assign9870_e9848,) = {
            if (var_novd_p < 1e27) {
                (var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9870_e9848,)
    } else {
        (1e23,)
    }
};
        var_novd_i = assign9870_e9850;

        let (assign9880_e9856,) = {
    if (var_ct_p > 0.0) {
        (var_ct_p,)
    } else {
        (0.0,)
    }
};
        var_ct_i = assign9880_e9856;

        let (assign9890_e9867,) = {
    if (var_ctb_p > 0.0) {
        let (assign9890_e9865,) = {
            if (var_ctb_p < 0.5) {
                (var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9890_e9865,)
    } else {
        (0.0,)
    }
};
        var_ctb_i = assign9890_e9867;

        let (assign9900_e9878,) = {
    if (var_ctg_p > 0.0) {
        let (assign9900_e9876,) = {
            if (var_ctg_p < 1.0) {
                (var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9900_e9876,)
    } else {
        (0.0,)
    }
};
        var_ctg_i = assign9900_e9878;

        var_stct_i = var_stct_p;

        let (assign9920_e9885,) = {
    if (var_cf_p > 0.0) {
        (var_cf_p,)
    } else {
        (0.0,)
    }
};
        var_cf_i = assign9920_e9885;

        let (assign9930_e9896,) = {
    if (var_cfb_p > 0.0) {
        let (assign9930_e9894,) = {
            if (var_cfb_p < 1.0) {
                (var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9930_e9894,)
    } else {
        (0.0,)
    }
};
        var_cfb_i = assign9930_e9896;

        let (assign9940_e9902,) = {
    if (var_cfd_p > 0.0) {
        (var_cfd_p,)
    } else {
        (0.0,)
    }
};
        var_cfd_i = assign9940_e9902;

        let (assign9950_e9908,) = {
    if (var_psce_p > 0.0) {
        (var_psce_p,)
    } else {
        (0.0,)
    }
};
        var_psce_i = assign9950_e9908;

        let (assign9960_e9919,) = {
    if (var_psceb_p > 0.0) {
        let (assign9960_e9917,) = {
            if (var_psceb_p < 1.0) {
                (var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9960_e9917,)
    } else {
        (0.0,)
    }
};
        var_psceb_i = assign9960_e9919;

        let (assign9970_e9925,) = {
    if (var_psced_p > 0.0) {
        (var_psced_p,)
    } else {
        (0.0,)
    }
};
        var_psced_i = assign9970_e9925;

        let (assign9980_e9931,) = {
    if (var_betn_p > 0.0) {
        (var_betn_p,)
    } else {
        (0.0,)
    }
};
        var_betn_i = assign9980_e9931;

        var_stbet_i = var_stbet_p;

        let (assign10000_e9938,) = {
    if (var_mue_p > 0.0) {
        (var_mue_p,)
    } else {
        (0.0,)
    }
};
        var_mue_i = assign10000_e9938;

        var_stmue_i = var_stmue_p;

        let (assign10020_e9945,) = {
    if (var_themu_p > 0.0) {
        (var_themu_p,)
    } else {
        (0.0,)
    }
};
        var_themu_i = assign10020_e9945;

        var_stthemu_i = var_stthemu_p;

        let (assign10040_e9952,) = {
    if (var_cs_p > 0.0) {
        (var_cs_p,)
    } else {
        (0.0,)
    }
};
        var_cs_i = assign10040_e9952;

        var_stcs_i = var_stcs_p;

        let (assign10060_e9959,) = {
    if (var_thecs_p > 0.0) {
        (var_thecs_p,)
    } else {
        (0.0,)
    }
};
        var_thecs_i = assign10060_e9959;

        var_stthecs_i = var_stthecs_p;

        let (assign10080_e9966,) = {
    if (var_xcor_p > 0.0) {
        (var_xcor_p,)
    } else {
        (0.0,)
    }
};
        var_xcor_i = assign10080_e9966;

        var_stxcor_i = var_stxcor_p;

        var_feta_i = var_feta_p;

        let (assign10110_e9974,) = {
    if (var_rs_p > 0.0) {
        (var_rs_p,)
    } else {
        (0.0,)
    }
};
        var_rs_i = assign10110_e9974;

        var_strs_i = var_strs_p;

        let assign10130_e9978: f64 = (-0.5);
        let (assign10130_e9988,) = {
    if (var_rsb_p > assign10130_e9978) {
        let (assign10130_e9985,) = {
            if (var_rsb_p < 1.0) {
                (var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10130_e9985,)
    } else {
        let assign10130_e9987: f64 = (-0.5);
        (assign10130_e9987,)
    }
};
        var_rsb_i = assign10130_e9988;

        let assign10140_e9991: f64 = (-0.5);
        let (assign10140_e9996,) = {
    if (var_rsg_p > assign10140_e9991) {
        (var_rsg_p,)
    } else {
        let assign10140_e9995: f64 = (-0.5);
        (assign10140_e9995,)
    }
};
        var_rsg_i = assign10140_e9996;

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
        *var_guard151_slot = var_guard151;
        *var_guard152_slot = var_guard152;
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
        var_munqs_p: f64,
        var_neffedge_p: f64,
        var_nf_i: f64,
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
        var_guard153_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_imaxii_i_slot: &mut f64,
        var_mult_inst_slot: &mut f64,
        var_munqs_i_slot: &mut f64,
        var_neffedge_i_slot: &mut f64,
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
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_imaxii_i: f64 = *var_imaxii_i_slot;
        let mut var_mult_inst: f64 = *var_mult_inst_slot;
        let mut var_munqs_i: f64 = *var_munqs_i_slot;
        let mut var_neffedge_i: f64 = *var_neffedge_i_slot;
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

        let (assign10150_e10002,) = {
    if (var_thesat_p > 0.0) {
        (var_thesat_p,)
    } else {
        (0.0,)
    }
};
        var_thesat_i = assign10150_e10002;

        var_stthesat_i = var_stthesat_p;

        let assign10170_e10006: f64 = (-0.5);
        let (assign10170_e10016,) = {
    if (var_thesatb_p > assign10170_e10006) {
        let (assign10170_e10013,) = {
            if (var_thesatb_p < 1.0) {
                (var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10170_e10013,)
    } else {
        let assign10170_e10015: f64 = (-0.5);
        (assign10170_e10015,)
    }
};
        var_thesatb_i = assign10170_e10016;

        let assign10180_e10019: f64 = (-0.5);
        let (assign10180_e10024,) = {
    if (var_thesatg_p > assign10180_e10019) {
        (var_thesatg_p,)
    } else {
        let assign10180_e10023: f64 = (-0.5);
        (assign10180_e10023,)
    }
};
        var_thesatg_i = assign10180_e10024;

        let (assign10190_e10030,) = {
    if (var_thesatt_p > 0.01) {
        (var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        var_thesatt_i = assign10190_e10030;

        let (assign10200_e10036,) = {
    if (var_ax_p > 2.0) {
        (var_ax_p,)
    } else {
        (2.0,)
    }
};
        var_ax_i = assign10200_e10036;

        let (assign10210_e10042,) = {
    if (var_alp_p > 0.0) {
        (var_alp_p,)
    } else {
        (0.0,)
    }
};
        var_alp_i = assign10210_e10042;

        let (assign10220_e10048,) = {
    if (var_alp1_p > 0.0) {
        (var_alp1_p,)
    } else {
        (0.0,)
    }
};
        var_alp1_i = assign10220_e10048;

        let (assign10230_e10054,) = {
    if (var_alp2_p > 0.0) {
        (var_alp2_p,)
    } else {
        (0.0,)
    }
};
        var_alp2_i = assign10230_e10054;

        var_vp_i = var_vp_p;

        let (assign10250_e10061,) = {
    if (var_a1_p > 0.0) {
        (var_a1_p,)
    } else {
        (0.0,)
    }
};
        var_a1_i = assign10250_e10061;

        var_a2_i = var_a2_p;

        var_sta2_i = var_sta2_p;

        let (assign10280_e10069,) = {
    if (var_a3_p > 0.0) {
        (var_a3_p,)
    } else {
        (0.0,)
    }
};
        var_a3_i = assign10280_e10069;

        let (assign10290_e10075,) = {
    if (var_a4_p > 0.0) {
        (var_a4_p,)
    } else {
        (0.0,)
    }
};
        var_a4_i = assign10290_e10075;

        let (assign10300_e10081,) = {
    if (var_imaxii_p > 1e-12) {
        (var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        var_imaxii_i = assign10300_e10081;

        var_gco_i = var_gco_p;

        let (assign10320_e10088,) = {
    if (var_iginv_p > 0.0) {
        (var_iginv_p,)
    } else {
        (0.0,)
    }
};
        var_iginv_i = assign10320_e10088;

        let (assign10330_e10094,) = {
    if (var_igov_p > 0.0) {
        (var_igov_p,)
    } else {
        (0.0,)
    }
};
        var_igov_i = assign10330_e10094;

        let (assign10340_e10100,) = {
    if (var_igovd_p > 0.0) {
        (var_igovd_p,)
    } else {
        (0.0,)
    }
};
        var_igovd_i = assign10340_e10100;

        var_stig_i = var_stig_p;

        var_gc2_i = var_gc2_p;

        var_gc3_i = var_gc3_p;

        var_gc2ov_i = var_gc2ov_p;

        var_gc3ov_i = var_gc3ov_p;

        var_gc2ovd_i = var_gc2ovd_p;

        var_gc3ovd_i = var_gc3ovd_p;

        var_chib_i = var_chib_p;

        let (assign10430_e10114,) = {
    if (var_agidl_p > 0.0) {
        (var_agidl_p,)
    } else {
        (0.0,)
    }
};
        var_agidl_i = assign10430_e10114;

        let (assign10440_e10120,) = {
    if (var_agidld_p > 0.0) {
        (var_agidld_p,)
    } else {
        (0.0,)
    }
};
        var_agidld_i = assign10440_e10120;

        var_bgidl_i = var_bgidl_p;

        var_bgidld_i = var_bgidld_p;

        var_stbgidl_i = var_stbgidl_p;

        var_stbgidld_i = var_stbgidld_p;

        var_cgidl_i = var_cgidl_p;

        var_cgidld_i = var_cgidld_p;

        let (assign10510_e10132,) = {
    if (var_cox_p > 0.0) {
        (var_cox_p,)
    } else {
        (0.0,)
    }
};
        var_cox_i = assign10510_e10132;

        var_delvtac_i = var_delvtac_p;

        let (assign10530_e10139,) = {
    if (var_facneffac_p > 0.0) {
        (var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        var_facneffac_i = assign10530_e10139;

        let (assign10540_e10145,) = {
    if (var_thesatac_p > 0.0) {
        (var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        var_thesatac_i = assign10540_e10145;

        let (assign10550_e10151,) = {
    if (var_axac_p > 2.0) {
        (var_axac_p,)
    } else {
        (2.0,)
    }
};
        var_axac_i = assign10550_e10151;

        var_alpac_i = var_alpac_p;

        let (assign10570_e10158,) = {
    if (var_alp1ac_p > 0.0) {
        (var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        var_alp1ac_i = assign10570_e10158;

        let (assign10580_e10164,) = {
    if (var_cgov_p > 0.0) {
        (var_cgov_p,)
    } else {
        (0.0,)
    }
};
        var_cgov_i = assign10580_e10164;

        let (assign10590_e10170,) = {
    if (var_cgovd_p > 0.0) {
        (var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        var_cgovd_i = assign10590_e10170;

        var_fcgovacc_i = var_fcgovacc_p;

        var_fcgovaccd_i = var_fcgovaccd_p;

        var_cgovaccg_i = var_cgovaccg_p;

        let (assign10630_e10179,) = {
    if (var_cgbov_p > 0.0) {
        (var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        var_cgbov_i = assign10630_e10179;

        let (assign10640_e10185,) = {
    if (var_cinr_p > 0.0) {
        (var_cinr_p,)
    } else {
        (0.0,)
    }
};
        var_cinr_i = assign10640_e10185;

        let (assign10650_e10191,) = {
    if (var_cinrd_p > 0.0) {
        (var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        var_cinrd_i = assign10650_e10191;

        var_dvfbinr_i = var_dvfbinr_p;

        var_fcinrdep_i = var_fcinrdep_p;

        var_fcinracc_i = var_fcinracc_p;

        var_axinr_i = var_axinr_p;

        let (assign10700_e10201,) = {
    if (var_cfr_p > 0.0) {
        (var_cfr_p,)
    } else {
        (0.0,)
    }
};
        var_cfr_i = assign10700_e10201;

        let (assign10710_e10207,) = {
    if (var_cfrd_p > 0.0) {
        (var_cfrd_p,)
    } else {
        (0.0,)
    }
};
        var_cfrd_i = assign10710_e10207;

        var_fnt_i = var_fnt_p;

        let (assign10730_e10214,) = {
    if (var_fntexc_p > 0.0) {
        (var_fntexc_p,)
    } else {
        (0.0,)
    }
};
        var_fntexc_i = assign10730_e10214;

        var_vfbedge_i = var_vfbedge_p;

        var_stvfbedge_i = var_stvfbedge_p;

        var_dphibedge_i = var_dphibedge_p;

        let (assign10810_e10247,) = {
    if (var_neffedge_p > 1e20) {
        let (assign10810_e10245,) = {
            if (var_neffedge_p < 1e26) {
                (var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10810_e10245,)
    } else {
        (1e20,)
    }
};
        var_neffedge_i = assign10810_e10247;

        let (assign10820_e10253,) = {
    if (var_ctedge_p > 0.0) {
        (var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        var_ctedge_i = assign10820_e10253;

        let (assign10830_e10259,) = {
    if (var_betnedge_p > 0.0) {
        (var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        var_betnedge_i = assign10830_e10259;

        var_stbetedge_i = var_stbetedge_p;

        let (assign10850_e10266,) = {
    if (var_psceedge_p > 0.0) {
        (var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        var_psceedge_i = assign10850_e10266;

        let (assign10860_e10277,) = {
    if (var_pscebedge_p > 0.0) {
        let (assign10860_e10275,) = {
            if (var_pscebedge_p < 1.0) {
                (var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10860_e10275,)
    } else {
        (0.0,)
    }
};
        var_pscebedge_i = assign10860_e10277;

        let (assign10870_e10283,) = {
    if (var_pscededge_p > 0.0) {
        (var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        var_pscededge_i = assign10870_e10283;

        let (assign10880_e10289,) = {
    if (var_cfedge_p > 0.0) {
        (var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfedge_i = assign10880_e10289;

        let (assign10890_e10300,) = {
    if (var_cfbedge_p > 0.0) {
        let (assign10890_e10298,) = {
            if (var_cfbedge_p < 1.0) {
                (var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10890_e10298,)
    } else {
        (0.0,)
    }
};
        var_cfbedge_i = assign10890_e10300;

        let (assign10900_e10306,) = {
    if (var_cfdedge_p > 0.0) {
        (var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfdedge_i = assign10900_e10306;

        let (assign10960_e10332,) = {
    if (var_rg_p > 0.0) {
        (var_rg_p,)
    } else {
        (0.0,)
    }
};
        var_rg_i = assign10960_e10332;

        var_rse_i = var_rse_p;

        var_rde_i = var_rde_p;

        var_rbulk_i = var_rbulk_p;

        var_rjuns_i = var_rjuns_p;

        var_rjund_i = var_rjund_p;

        var_rwell_i = var_rwell_p;

        let assign11030_e10341: f64 = (p.p31 * var_nf_i);
        let (assign11030_e10348,) = {
    if (assign11030_e10341 > 0.0) {
        let assign11030_e10346: f64 = (p.p31 * var_nf_i);
        (assign11030_e10346,)
    } else {
        (0.0,)
    }
};
        var_mult_inst = assign11030_e10348;

        var_factuo_i = p.p16;

        var_delvto_i = p.p15;

        var_factuoedge_i = p.p18;

        var_delvtoedge_i = p.p17;

        let (assign11080_e10358,) = {
    if (var_munqs_p > 0.0) {
        (var_munqs_p,)
    } else {
        (0.0,)
    }
};
        var_munqs_i = assign11080_e10358;

        let assign11090_e10361: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard153 = assign11090_e10361;

        let (assign11100_e10365,) = {
    if (var_guard153 != 0.0) {
        (var_toxov_i,)
    } else {
        (var_toxovd_i,)
    }
};
        var_toxovd_i = assign11100_e10365;

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
        *var_guard153_slot = var_guard153;
        *var_iginv_i_slot = var_iginv_i;
        *var_igov_i_slot = var_igov_i;
        *var_igovd_i_slot = var_igovd_i;
        *var_imaxii_i_slot = var_imaxii_i;
        *var_mult_inst_slot = var_mult_inst;
        *var_munqs_i_slot = var_munqs_i;
        *var_neffedge_i_slot = var_neffedge_i;
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

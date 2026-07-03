#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign00_e1569: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e1569;
        if (locals.var_guard1 != 0.0) {
            let assign10_e1572: f64 = 1.0;
            locals.var_chnl_type = assign10_e1572;
        }
        if (locals.var_guard1 == 0.0) {
            let assign20_e1578: f64 = (-1.0);
            locals.var_chnl_type = assign20_e1578;
        }
        let assign30_e1583: f64 = (8.8541878176e-12 * 11.8);
        locals.var_epssi = assign30_e1583;
        let assign40_e1586: f64 = if p.p51 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign40_e1586;
        if (locals.var_guard2 != 0.0) {
            locals.var_swnqs_i = 0.0;
        }
        let assign60_e1593: f64 = if p.p51 < 1.5 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign60_e1593;
        if ((locals.var_guard2 == 0.0) && (locals.var_guard3 != 0.0)) {
            locals.var_swnqs_i = 1.0;
        }
        let assign80_e1603: f64 = if p.p51 < 2.5 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign80_e1603;
        if (((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 != 0.0)) {
            locals.var_swnqs_i = 2.0;
        }
        let assign100_e1616: f64 = if p.p51 < 4.0 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign100_e1616;
        if ((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 != 0.0)) {
            locals.var_swnqs_i = 3.0;
        }
        let assign120_e1632: f64 = if p.p51 < 7.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign120_e1632;
        if (((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 == 0.0)) && (locals.var_guard6 != 0.0)) {
            locals.var_swnqs_i = 5.0;
        }
        if (((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 == 0.0)) && (locals.var_guard6 == 0.0)) {
            locals.var_swnqs_i = 9.0;
        }
        locals.var_r_nqs = 1000.0;
        locals.var_vnorm = 10.0;
        let assign170_e1670: f64 = (1.0 / locals.var_vnorm);
        locals.var_vnorm_inv = assign170_e1670;
        let assign180_e1673: f64 = (273.15 + p.p38);
        locals.var_tkr = assign180_e1673;
        locals.var_swjunexp_i = 0.0;
        let assign200_e1677: f64 = if p.p927 > 0.5 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign200_e1677;
        if (locals.var_guard7 != 0.0) {
            locals.var_swjunexp_i = 1.0;
        }
        if (locals.var_guard7 == 0.0) {
            locals.var_swjunexp_i = 0.0;
        }
        let assign230_e1689: f64 = (273.15 + p.p823);
        locals.var_tkr_1 = assign230_e1689;
        let assign240_e1692: f64 = (1.3806505e-23 / 1.6021918e-19);
        locals.var_kbol_over_qele = assign240_e1692;
        let assign250_e1695: f64 = (locals.var_kbol_over_qele * locals.var_tkr_1);
        locals.var_phitr = assign250_e1695;
        let assign260_e1698: f64 = (1.0 / locals.var_phitr);
        locals.var_phitrinv = assign260_e1698;
        let assign270_e1701: f64 = (0.000702 * locals.var_tkr_1);
        let assign270_e1703: f64 = (assign270_e1701 * locals.var_tkr_1);
        let assign270_e1704: f64 = (-assign270_e1703);
        let assign270_e1707: f64 = (1108.0 + locals.var_tkr_1);
        let assign270_e1708: f64 = (assign270_e1704 / assign270_e1707);
        locals.var_deltaphigr = assign270_e1708;
        let assign280_e1711: f64 = (p.p834 + locals.var_deltaphigr);
        locals.var_phigrbot = assign280_e1711;
        let assign290_e1714: f64 = (p.p835 + locals.var_deltaphigr);
        locals.var_phigrsti = assign290_e1714;
        let assign300_e1717: f64 = (p.p836 + locals.var_deltaphigr);
        locals.var_phigrgat = assign300_e1717;
        let assign310_e1720: f64 = (1.0 - p.p831);
        locals.var_one_minus_pbot = assign310_e1720;
        let assign320_e1723: f64 = (1.0 - p.p832);
        locals.var_one_minus_psti = assign320_e1723;
        let assign330_e1726: f64 = (1.0 - p.p833);
        locals.var_one_minus_pgat = assign330_e1726;
        let assign340_e1729: f64 = (1.0 / locals.var_one_minus_pbot);
        locals.var_one_over_one_minus_pbot = assign340_e1729;
        let assign350_e1732: f64 = (1.0 / locals.var_one_minus_psti);
        locals.var_one_over_one_minus_psti = assign350_e1732;
        let assign360_e1735: f64 = (1.0 / locals.var_one_minus_pgat);
        locals.var_one_over_one_minus_pgat = assign360_e1735;
        let assign370_e1738: f64 = (locals.var_epssi / p.p825);
        locals.var_wdepnulrbot = assign370_e1738;
        let assign380_e1741: f64 = (p.p843 * locals.var_epssi);
        let assign380_e1743: f64 = (assign380_e1741 / p.p826);
        locals.var_wdepnulrsti = assign380_e1743;
        let assign390_e1746: f64 = (p.p844 * locals.var_epssi);
        let assign390_e1748: f64 = (assign390_e1746 / p.p827);
        locals.var_wdepnulrgat = assign390_e1748;
        let assign400_e1751: f64 = (1.0 / locals.var_wdepnulrbot);
        locals.var_wdepnulrinvbot = assign400_e1751;
        let assign410_e1754: f64 = (1.0 / locals.var_wdepnulrsti);
        locals.var_wdepnulrinvsti = assign410_e1754;
        let assign420_e1757: f64 = (1.0 / locals.var_wdepnulrgat);
        locals.var_wdepnulrinvgat = assign420_e1757;
        let assign430_e1760: f64 = (1.0 / p.p828);
        locals.var_vbirbotinv = assign430_e1760;
        let assign440_e1763: f64 = (1.0 / p.p829);
        locals.var_vbirstiinv = assign440_e1763;
        let assign450_e1766: f64 = (1.0 / p.p830);
        locals.var_vbirgatinv = assign450_e1766;
        let assign460_e1769: f64 = (1.772453850905516 * 0.29214664);
        locals.var_perfc = assign460_e1769;
        let assign470_e1771: f64 = (-5.0);
        let assign470_e1773: f64 = (assign470_e1771 * 0.29214664);
        let assign470_e1775: f64 = (assign470_e1773 + 6.0);
        let assign470_e1778: f64 = (-2.0);
        let assign470_e1779: f64 = (locals.var_perfc).powf(assign470_e1778);
        let assign470_e1780: f64 = (assign470_e1775 - assign470_e1779);
        let assign470_e1782: f64 = (assign470_e1780 / 3.0);
        locals.var_berfc = assign470_e1782;
        let assign480_e1785: f64 = (1.0 - 0.29214664);
        let assign480_e1787: f64 = (assign480_e1785 - locals.var_berfc);
        locals.var_cerfc = assign480_e1787;
        let assign490_e1791: f64 = (1.0 / p.p824);
        let assign490_e1792: f64 = (1.0 - assign490_e1791);
        locals.var_alphaav = assign490_e1792;
        let assign500_e1797: f64 = (locals.var_alphaav).powf(p.p863);
        let assign500_e1798: f64 = (1.0 - assign500_e1797);
        let assign500_e1799: f64 = (1.0 / assign500_e1798);
        locals.var_fstopbot = assign500_e1799;
        let assign510_e1804: f64 = (locals.var_alphaav).powf(p.p864);
        let assign510_e1805: f64 = (1.0 - assign510_e1804);
        let assign510_e1806: f64 = (1.0 / assign510_e1805);
        locals.var_fstopsti = assign510_e1806;
        let assign520_e1811: f64 = (locals.var_alphaav).powf(p.p865);
        let assign520_e1812: f64 = (1.0 - assign520_e1811);
        let assign520_e1813: f64 = (1.0 / assign520_e1812);
        locals.var_fstopgat = assign520_e1813;
        let assign530_e1816: f64 = (1.0 / p.p860);
        locals.var_vbrinvbot = assign530_e1816;
        let assign540_e1819: f64 = (1.0 / p.p861);
        locals.var_vbrinvsti = assign540_e1819;
        let assign550_e1822: f64 = (1.0 / p.p862);
        (locals.var_vbrinvgat, locals.var_vbrinvgat_dn5, locals.var_vbrinvgat_dn6, locals.var_vbrinvgat_dn7, locals.var_vbrinvgat_dn8, ) = (assign550_e1822, 0.0, 0.0, 0.0, 0.0, );
        let assign560_e1825: f64 = (locals.var_fstopbot * locals.var_fstopbot);
        let assign560_e1829: f64 = (p.p863 - 1.0);
        let assign560_e1830: f64 = (locals.var_alphaav).powf(assign560_e1829);
        let assign560_e1831: f64 = (assign560_e1825 * assign560_e1830);
        let assign560_e1832: f64 = (-assign560_e1831);
        let assign560_e1834: f64 = (assign560_e1832 * p.p863);
        let assign560_e1836: f64 = (assign560_e1834 * locals.var_vbrinvbot);
        locals.var_slopebot = assign560_e1836;
        let assign570_e1839: f64 = (locals.var_fstopsti * locals.var_fstopsti);
        let assign570_e1843: f64 = (p.p864 - 1.0);
        let assign570_e1844: f64 = (locals.var_alphaav).powf(assign570_e1843);
        let assign570_e1845: f64 = (assign570_e1839 * assign570_e1844);
        let assign570_e1846: f64 = (-assign570_e1845);
        let assign570_e1848: f64 = (assign570_e1846 * p.p864);
        let assign570_e1850: f64 = (assign570_e1848 * locals.var_vbrinvsti);
        locals.var_slopesti = assign570_e1850;
        let assign580_e1853: f64 = (locals.var_fstopgat * locals.var_fstopgat);
        let assign580_e1857: f64 = (p.p865 - 1.0);
        let assign580_e1858: f64 = (locals.var_alphaav).powf(assign580_e1857);
        let assign580_e1859: f64 = (assign580_e1853 * assign580_e1858);
        let assign580_e1860: f64 = (-assign580_e1859);
        let assign580_e1862: f64 = (assign580_e1860 * p.p865);
        let assign580_e1864: f64 = (assign580_e1862 * locals.var_vbrinvgat);
        (locals.var_slopegat, locals.var_slopegat_dn5, locals.var_slopegat_dn6, locals.var_slopegat_dn7, locals.var_slopegat_dn8, ) = (assign580_e1864, (assign580_e1862 * locals.var_vbrinvgat_dn5), (assign580_e1862 * locals.var_vbrinvgat_dn6), (assign580_e1862 * locals.var_vbrinvgat_dn7), (assign580_e1862 * locals.var_vbrinvgat_dn8), );
        let assign590_e1879: f64 = if ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard8 = assign590_e1879;
        if (locals.var_guard8 != 0.0) {
            locals.var_swgat2nd = 1.0;
        }
        if (locals.var_guard8 == 0.0) {
            locals.var_swgat2nd = 0.0;
        }
        let assign620_e1891: f64 = if locals.var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign620_e1891;
        if (locals.var_guard9 != 0.0) {
            let assign630_e1895: f64 = (p.p827 * p.p866);
            let (assign630_e1902,) = {
    if (assign630_e1895 > 1e-18) {
        let assign630_e1900: f64 = (p.p827 * p.p866);
        (assign630_e1900,)
    } else {
        (1e-18,)
    }
};
            locals.var_cjorgat2nd = assign630_e1902;
        }
        if (locals.var_guard9 != 0.0) {
            let assign640_e1908: f64 = (p.p830 * p.p867);
            let (assign640_e1915,) = {
    if (assign640_e1908 > 0.05) {
        let assign640_e1913: f64 = (p.p830 * p.p867);
        (assign640_e1913,)
    } else {
        (0.05,)
    }
};
            locals.var_vbirgat2nd = assign640_e1915;
        }
        if (locals.var_guard9 != 0.0) {
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
            locals.var_pgat2nd = assign650_e1942;
        }
        if (locals.var_guard9 != 0.0) {
            let assign660_e1948: f64 = (p.p836 * p.p869);
            locals.var_phiggat2nd = assign660_e1948;
        }
        if (locals.var_guard9 != 0.0) {
            let assign670_e1954: f64 = (locals.var_phiggat2nd + locals.var_deltaphigr);
            locals.var_phigrgat2nd = assign670_e1954;
        }
        if (locals.var_guard9 != 0.0) {
            let assign680_e1960: f64 = (1.0 - locals.var_pgat2nd);
            locals.var_one_minus_pgat2nd = assign680_e1960;
        }
        if (locals.var_guard9 != 0.0) {
            let assign690_e1966: f64 = (1.0 / locals.var_one_minus_pgat2nd);
            locals.var_one_over_one_minus_pgat2nd = assign690_e1966;
        }
        let assign700_e1971: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard10 = assign700_e1971;
        if (locals.var_guard10 != 0.0) {
            locals.var_cjorbotd_i = p.p825;
            locals.var_cjorstid_i = p.p826;
            locals.var_cjorgatd_i = p.p827;
            locals.var_vbirbotd_i = p.p828;
            locals.var_vbirstid_i = p.p829;
            locals.var_vbirgatd_i = p.p830;
            locals.var_pbotd_i = p.p831;
            locals.var_pstid_i = p.p832;
            locals.var_pgatd_i = p.p833;
            locals.var_phigbotd_i = p.p834;
            locals.var_phigstid_i = p.p835;
            locals.var_phiggatd_i = p.p836;
            locals.var_idsatrbotd_i = p.p837;
            locals.var_idsatrstid_i = p.p838;
            locals.var_idsatrgatd_i = p.p839;
            locals.var_csrhbotd_i = p.p840;
            locals.var_csrhstid_i = p.p841;
            locals.var_csrhgatd_i = p.p842;
            locals.var_xjunstid_i = p.p843;
            locals.var_xjungatd_i = p.p844;
            locals.var_ctatbotd_i = p.p845;
            locals.var_ctatstid_i = p.p846;
            locals.var_ctatgatd_i = p.p847;
            locals.var_mefftatbotd_i = p.p848;
            locals.var_mefftatstid_i = p.p849;
            locals.var_mefftatgatd_i = p.p850;
            locals.var_cbbtbotd_i = p.p851;
            locals.var_cbbtstid_i = p.p852;
            locals.var_cbbtgatd_i = p.p853;
            locals.var_fbbtrbotd_i = p.p854;
            locals.var_fbbtrstid_i = p.p855;
            locals.var_fbbtrgatd_i = p.p856;
            locals.var_stfbbtbotd_i = p.p857;
            locals.var_stfbbtstid_i = p.p858;
            locals.var_stfbbtgatd_i = p.p859;
            locals.var_vbrbotd_i = p.p860;
            locals.var_vbrstid_i = p.p861;
            locals.var_vbrgatd_i = p.p862;
            locals.var_pbrbotd_i = p.p863;
            locals.var_pbrstid_i = p.p864;
            locals.var_pbrgatd_i = p.p865;
            locals.var_vjunrefd_i = p.p928;
            locals.var_fjunqd_i = p.p929;
            locals.var_advbrgatd_i = p.p872;
            locals.var_bdvbrgatd_i = p.p873;
            locals.var_adbbtgatd_i = p.p874;
            locals.var_bdbbtgatd_i = p.p875;
            locals.var_fcjorgat2d_i = p.p866;
            locals.var_fvbirgat2d_i = p.p867;
        }
    }
    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        if (locals.var_guard10 != 0.0) {
            locals.var_fpgat2d_i = p.p868;
            locals.var_fphiggat2d_i = p.p869;
            locals.var_vtrgatd_i = p.p870;
            locals.var_anugatd_i = p.p871;
        }
        if (locals.var_guard10 == 0.0) {
            locals.var_cjorbotd_i = p.p876;
            locals.var_cjorstid_i = p.p877;
            locals.var_cjorgatd_i = p.p878;
            locals.var_vbirbotd_i = p.p879;
            locals.var_vbirstid_i = p.p880;
            locals.var_vbirgatd_i = p.p881;
            locals.var_pbotd_i = p.p882;
            locals.var_pstid_i = p.p883;
            locals.var_pgatd_i = p.p884;
            locals.var_phigbotd_i = p.p885;
            locals.var_phigstid_i = p.p886;
            locals.var_phiggatd_i = p.p887;
            locals.var_idsatrbotd_i = p.p888;
            locals.var_idsatrstid_i = p.p889;
            locals.var_idsatrgatd_i = p.p890;
            locals.var_csrhbotd_i = p.p891;
            locals.var_csrhstid_i = p.p892;
            locals.var_csrhgatd_i = p.p893;
            locals.var_xjunstid_i = p.p894;
            locals.var_xjungatd_i = p.p895;
            locals.var_ctatbotd_i = p.p896;
            locals.var_ctatstid_i = p.p897;
            locals.var_ctatgatd_i = p.p898;
            locals.var_mefftatbotd_i = p.p899;
            locals.var_mefftatstid_i = p.p900;
            locals.var_mefftatgatd_i = p.p901;
            locals.var_cbbtbotd_i = p.p902;
            locals.var_cbbtstid_i = p.p903;
            locals.var_cbbtgatd_i = p.p904;
            locals.var_fbbtrbotd_i = p.p905;
            locals.var_fbbtrstid_i = p.p906;
            locals.var_fbbtrgatd_i = p.p907;
            locals.var_stfbbtbotd_i = p.p908;
            locals.var_stfbbtstid_i = p.p909;
            locals.var_stfbbtgatd_i = p.p910;
            locals.var_vbrbotd_i = p.p911;
            locals.var_vbrstid_i = p.p912;
            locals.var_vbrgatd_i = p.p913;
            locals.var_pbrbotd_i = p.p914;
            locals.var_pbrstid_i = p.p915;
            locals.var_pbrgatd_i = p.p916;
            locals.var_vjunrefd_i = p.p930;
            locals.var_fjunqd_i = p.p931;
            locals.var_advbrgatd_i = p.p923;
            locals.var_bdvbrgatd_i = p.p924;
            locals.var_adbbtgatd_i = p.p925;
            locals.var_bdbbtgatd_i = p.p926;
            locals.var_fcjorgat2d_i = p.p917;
            locals.var_fvbirgat2d_i = p.p918;
            locals.var_fpgat2d_i = p.p919;
            locals.var_fphiggat2d_i = p.p920;
            locals.var_vtrgatd_i = p.p921;
            locals.var_anugatd_i = p.p922;
        }
        let assign1770_e2451: f64 = (locals.var_phigbotd_i + locals.var_deltaphigr);
        locals.var_phigrbot_d = assign1770_e2451;
        let assign1780_e2454: f64 = (locals.var_phigstid_i + locals.var_deltaphigr);
        locals.var_phigrsti_d = assign1780_e2454;
        let assign1790_e2457: f64 = (locals.var_phiggatd_i + locals.var_deltaphigr);
        locals.var_phigrgat_d = assign1790_e2457;
        let assign1800_e2460: f64 = (1.0 - locals.var_pbotd_i);
        locals.var_one_minus_pbot_d = assign1800_e2460;
        let assign1810_e2463: f64 = (1.0 - locals.var_pstid_i);
        locals.var_one_minus_psti_d = assign1810_e2463;
        let assign1820_e2466: f64 = (1.0 - locals.var_pgatd_i);
        locals.var_one_minus_pgat_d = assign1820_e2466;
        let assign1830_e2469: f64 = (1.0 / locals.var_one_minus_pbot_d);
        locals.var_one_over_one_minus_pbot_d = assign1830_e2469;
        let assign1840_e2472: f64 = (1.0 / locals.var_one_minus_psti_d);
        locals.var_one_over_one_minus_psti_d = assign1840_e2472;
        let assign1850_e2475: f64 = (1.0 / locals.var_one_minus_pgat_d);
        locals.var_one_over_one_minus_pgat_d = assign1850_e2475;
        let assign1860_e2478: f64 = (locals.var_epssi / locals.var_cjorbotd_i);
        locals.var_wdepnulrbot_d = assign1860_e2478;
        let assign1870_e2481: f64 = (locals.var_xjunstid_i * locals.var_epssi);
        let assign1870_e2483: f64 = (assign1870_e2481 / locals.var_cjorstid_i);
        locals.var_wdepnulrsti_d = assign1870_e2483;
        let assign1880_e2486: f64 = (locals.var_xjungatd_i * locals.var_epssi);
        let assign1880_e2488: f64 = (assign1880_e2486 / locals.var_cjorgatd_i);
        locals.var_wdepnulrgat_d = assign1880_e2488;
        let assign1890_e2491: f64 = (1.0 / locals.var_wdepnulrbot_d);
        locals.var_wdepnulrinvbot_d = assign1890_e2491;
        let assign1900_e2494: f64 = (1.0 / locals.var_wdepnulrsti_d);
        locals.var_wdepnulrinvsti_d = assign1900_e2494;
        let assign1910_e2497: f64 = (1.0 / locals.var_wdepnulrgat_d);
        locals.var_wdepnulrinvgat_d = assign1910_e2497;
        let assign1920_e2500: f64 = (1.0 / locals.var_vbirbotd_i);
        locals.var_vbirbotinv_d = assign1920_e2500;
        let assign1930_e2503: f64 = (1.0 / locals.var_vbirstid_i);
        locals.var_vbirstiinv_d = assign1930_e2503;
        let assign1940_e2506: f64 = (1.0 / locals.var_vbirgatd_i);
        locals.var_vbirgatinv_d = assign1940_e2506;
        let assign1950_e2511: f64 = (locals.var_alphaav).powf(locals.var_pbrbotd_i);
        let assign1950_e2512: f64 = (1.0 - assign1950_e2511);
        let assign1950_e2513: f64 = (1.0 / assign1950_e2512);
        locals.var_fstopbot_d = assign1950_e2513;
        let assign1960_e2518: f64 = (locals.var_alphaav).powf(locals.var_pbrstid_i);
        let assign1960_e2519: f64 = (1.0 - assign1960_e2518);
        let assign1960_e2520: f64 = (1.0 / assign1960_e2519);
        locals.var_fstopsti_d = assign1960_e2520;
        let assign1970_e2525: f64 = (locals.var_alphaav).powf(locals.var_pbrgatd_i);
        let assign1970_e2526: f64 = (1.0 - assign1970_e2525);
        let assign1970_e2527: f64 = (1.0 / assign1970_e2526);
        locals.var_fstopgat_d = assign1970_e2527;
        let assign1980_e2530: f64 = (1.0 / locals.var_vbrbotd_i);
        locals.var_vbrinvbot_d = assign1980_e2530;
        let assign1990_e2533: f64 = (1.0 / locals.var_vbrstid_i);
        locals.var_vbrinvsti_d = assign1990_e2533;
        let assign2000_e2536: f64 = (1.0 / locals.var_vbrgatd_i);
        (locals.var_vbrinvgat_d, locals.var_vbrinvgat_d_dn5, locals.var_vbrinvgat_d_dn6, locals.var_vbrinvgat_d_dn7, locals.var_vbrinvgat_d_dn8, ) = (assign2000_e2536, 0.0, 0.0, 0.0, 0.0, );
        let assign2010_e2539: f64 = (locals.var_fstopbot_d * locals.var_fstopbot_d);
        let assign2010_e2543: f64 = (locals.var_pbrbotd_i - 1.0);
        let assign2010_e2544: f64 = (locals.var_alphaav).powf(assign2010_e2543);
        let assign2010_e2545: f64 = (assign2010_e2539 * assign2010_e2544);
        let assign2010_e2546: f64 = (-assign2010_e2545);
        let assign2010_e2548: f64 = (assign2010_e2546 * locals.var_pbrbotd_i);
        let assign2010_e2550: f64 = (assign2010_e2548 * locals.var_vbrinvbot_d);
        locals.var_slopebot_d = assign2010_e2550;
        let assign2020_e2553: f64 = (locals.var_fstopsti_d * locals.var_fstopsti_d);
        let assign2020_e2557: f64 = (locals.var_pbrstid_i - 1.0);
        let assign2020_e2558: f64 = (locals.var_alphaav).powf(assign2020_e2557);
        let assign2020_e2559: f64 = (assign2020_e2553 * assign2020_e2558);
        let assign2020_e2560: f64 = (-assign2020_e2559);
        let assign2020_e2562: f64 = (assign2020_e2560 * locals.var_pbrstid_i);
        let assign2020_e2564: f64 = (assign2020_e2562 * locals.var_vbrinvsti_d);
        locals.var_slopesti_d = assign2020_e2564;
        let assign2030_e2567: f64 = (locals.var_fstopgat_d * locals.var_fstopgat_d);
        let assign2030_e2571: f64 = (locals.var_pbrgatd_i - 1.0);
        let assign2030_e2572: f64 = (locals.var_alphaav).powf(assign2030_e2571);
        let assign2030_e2573: f64 = (assign2030_e2567 * assign2030_e2572);
        let assign2030_e2574: f64 = (-assign2030_e2573);
        let assign2030_e2576: f64 = (assign2030_e2574 * locals.var_pbrgatd_i);
        let assign2030_e2578: f64 = (assign2030_e2576 * locals.var_vbrinvgat_d);
        (locals.var_slopegat_d, locals.var_slopegat_d_dn5, locals.var_slopegat_d_dn6, locals.var_slopegat_d_dn7, locals.var_slopegat_d_dn8, ) = (assign2030_e2578, (assign2030_e2576 * locals.var_vbrinvgat_d_dn5), (assign2030_e2576 * locals.var_vbrinvgat_d_dn6), (assign2030_e2576 * locals.var_vbrinvgat_d_dn7), (assign2030_e2576 * locals.var_vbrinvgat_d_dn8), );
        let assign2040_e2593: f64 = if ((((locals.var_fcjorgat2d_i != 1.0) || (locals.var_fvbirgat2d_i != 1.0)) || (locals.var_fpgat2d_i != 1.0)) || (locals.var_fphiggat2d_i != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard11 = assign2040_e2593;
        if (locals.var_guard11 != 0.0) {
            locals.var_swgat2nd_d = 1.0;
        }
        if (locals.var_guard11 == 0.0) {
            locals.var_swgat2nd_d = 0.0;
        }
        let assign2070_e2605: f64 = if locals.var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign2070_e2605;
        if (locals.var_guard12 != 0.0) {
            let assign2080_e2609: f64 = (locals.var_cjorgatd_i * locals.var_fcjorgat2d_i);
            let (assign2080_e2616,) = {
    if (assign2080_e2609 > 1e-18) {
        let assign2080_e2614: f64 = (locals.var_cjorgatd_i * locals.var_fcjorgat2d_i);
        (assign2080_e2614,)
    } else {
        (1e-18,)
    }
};
            locals.var_cjorgat2nd_d = assign2080_e2616;
        }
        if (locals.var_guard12 != 0.0) {
            let assign2090_e2622: f64 = (locals.var_vbirgatd_i * locals.var_fvbirgat2d_i);
            let (assign2090_e2629,) = {
    if (assign2090_e2622 > 0.05) {
        let assign2090_e2627: f64 = (locals.var_vbirgatd_i * locals.var_fvbirgat2d_i);
        (assign2090_e2627,)
    } else {
        (0.05,)
    }
};
            locals.var_vbirgat2nd_d = assign2090_e2629;
        }
        if (locals.var_guard12 != 0.0) {
            let assign2100_e2635: f64 = (locals.var_pgatd_i * locals.var_fpgat2d_i);
            let (assign2100_e2642,) = {
    if (assign2100_e2635 > 0.05) {
        let assign2100_e2640: f64 = (locals.var_pgatd_i * locals.var_fpgat2d_i);
        (assign2100_e2640,)
    } else {
        (0.05,)
    }
};
            let (assign2100_e2656,) = {
    if (assign2100_e2642 < 0.95) {
        let assign2100_e2647: f64 = (locals.var_pgatd_i * locals.var_fpgat2d_i);
        let (assign2100_e2654,) = {
            if (assign2100_e2647 > 0.05) {
                let assign2100_e2652: f64 = (locals.var_pgatd_i * locals.var_fpgat2d_i);
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
            locals.var_pgat2nd_d = assign2100_e2656;
        }
        if (locals.var_guard12 != 0.0) {
            let assign2110_e2662: f64 = (locals.var_phiggatd_i * locals.var_fphiggat2d_i);
            locals.var_phiggat2nd_d = assign2110_e2662;
        }
        if (locals.var_guard12 != 0.0) {
            let assign2120_e2668: f64 = (locals.var_phiggat2nd_d + locals.var_deltaphigr);
            locals.var_phigrgat2nd_d = assign2120_e2668;
        }
        if (locals.var_guard12 != 0.0) {
            let assign2130_e2674: f64 = (1.0 - locals.var_pgat2nd_d);
            locals.var_one_minus_pgat2nd_d = assign2130_e2674;
        }
        if (locals.var_guard12 != 0.0) {
            let assign2140_e2680: f64 = (1.0 / locals.var_one_minus_pgat2nd_d);
            locals.var_one_over_one_minus_pgat2nd_d = assign2140_e2680;
        }
        let assign2150_e2685: f64 = 0.0;
        locals.var_gmin = assign2150_e2685;
        let assign2190_e2704: f64 = ctx_temp;
        let assign2190_e2706: f64 = (assign2190_e2704 + p.p56);
        let assign2190_e2708: f64 = (assign2190_e2706 + p.p35);
        locals.var_tka = assign2190_e2708;
        let assign2200_e2711: f64 = (locals.var_tka / locals.var_tkr);
        locals.var_rta = assign2200_e2711;
        let assign2210_e2714: f64 = (locals.var_tka - locals.var_tkr);
        locals.var_delta = assign2210_e2714;
        let assign2220_e2717: f64 = (locals.var_tka * 1.3806505e-23);
        let assign2220_e2719: f64 = (assign2220_e2717 / 1.6021918e-19);
        locals.var_phita = assign2220_e2719;
        let assign2230_e2722: f64 = (1.0 / locals.var_phita);
        locals.var_inv_phita = assign2230_e2722;
        locals.var_tkd = locals.var_tka;
        let assign2250_e2726: f64 = (locals.var_tkd * locals.var_tkd);
        locals.var_tkd_sq = assign2250_e2726;
        let assign2260_e2729: f64 = (locals.var_tkd - locals.var_tkr);
        locals.var_delt = assign2260_e2729;
        let assign2270_e2732: f64 = (locals.var_tkr / locals.var_tkd);
        locals.var_rtn = assign2270_e2732;
        let assign2280_e2734: f64 = (locals.var_rtn).ln();
        locals.var_ln_rtn = assign2280_e2734;
        let assign2290_e2737: f64 = (locals.var_tkd * 1.3806505e-23);
        let assign2290_e2739: f64 = (assign2290_e2737 / 1.6021918e-19);
        locals.var_phit = assign2290_e2739;
        let assign2300_e2742: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign2300_e2742;
        let assign2310_e2746: f64 = (9.025e-5 * locals.var_tkd);
        let assign2310_e2747: f64 = (1.179 - assign2310_e2746);
        let assign2310_e2750: f64 = (3.05e-7 * locals.var_tkd_sq);
        let assign2310_e2751: f64 = (assign2310_e2747 - assign2310_e2750);
        locals.var_eg = assign2310_e2751;
        let assign2320_e2755: f64 = (0.00045 * locals.var_tkd);
        let assign2320_e2756: f64 = (1.045 + assign2320_e2755);
        let assign2320_e2760: f64 = (0.0014 * locals.var_tkd);
        let assign2320_e2761: f64 = (0.523 + assign2320_e2760);
        let assign2320_e2764: f64 = (1.48e-6 * locals.var_tkd_sq);
        let assign2320_e2765: f64 = (assign2320_e2761 - assign2320_e2764);
        let assign2320_e2766: f64 = (assign2320_e2756 * assign2320_e2765);
        let assign2320_e2768: f64 = (assign2320_e2766 * locals.var_tkd_sq);
        let assign2320_e2770: f64 = (assign2320_e2768 / 90000.0);
        locals.var_phibfac = assign2320_e2770;
        if (!(locals.var_phibfac > 0.001)) {
            locals.var_phibfac = 0.001;
        }
        let assign2340_e2779: f64 = (4.0 * 1.3806505e-23);
        let assign2340_e2781: f64 = (assign2340_e2779 * locals.var_tkd);
        locals.var_nt0 = assign2340_e2781;
        let assign2350_e2782: f64 = ctx_temp;
        let assign2350_e2784: f64 = (assign2350_e2782 + p.p56);
        let assign2350_e2786: f64 = (assign2350_e2784 + p.p35);
        let assign2350_e2789: f64 = (-250.0);
        let assign2350_e2790: f64 = (273.15 + assign2350_e2789);
        let assign2350_e2791: f64 = (assign2350_e2786).max(assign2350_e2790);
        locals.var_tkd_1 = assign2350_e2791;
        let assign2360_e2794: f64 = (locals.var_tkd_1 / locals.var_tkr_1);
        locals.var_auxt = assign2360_e2794;
        let assign2370_e2797: f64 = (locals.var_kbol_over_qele * locals.var_tkd_1);
        locals.var_phitd = assign2370_e2797;
        let assign2380_e2800: f64 = (1.0 / locals.var_phitd);
        locals.var_phitdinv = assign2380_e2800;
    }
    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2390_e2803: f64 = (0.000702 * locals.var_tkd_1);
        let assign2390_e2805: f64 = (assign2390_e2803 * locals.var_tkd_1);
        let assign2390_e2806: f64 = (-assign2390_e2805);
        let assign2390_e2809: f64 = (1108.0 + locals.var_tkd_1);
        let assign2390_e2810: f64 = (assign2390_e2806 / assign2390_e2809);
        locals.var_deltaphigd = assign2390_e2810;
        let assign2400_e2813: f64 = (p.p834 + locals.var_deltaphigd);
        locals.var_phigdbot = assign2400_e2813;
        let assign2410_e2816: f64 = (p.p835 + locals.var_deltaphigd);
        locals.var_phigdsti = assign2410_e2816;
        let assign2420_e2819: f64 = (p.p836 + locals.var_deltaphigd);
        locals.var_phigdgat = assign2420_e2819;
        let assign2430_e2822: f64 = (locals.var_auxt).powf(1.5);
        let assign2430_e2826: f64 = (locals.var_phigrbot * locals.var_phitrinv);
        let assign2430_e2829: f64 = (locals.var_phigdbot * locals.var_phitdinv);
        let assign2430_e2830: f64 = (assign2430_e2826 - assign2430_e2829);
        let assign2430_e2831: f64 = (0.5 * assign2430_e2830);
        let assign2430_e2832: f64 = (assign2430_e2831).exp();
        let assign2430_e2833: f64 = (assign2430_e2822 * assign2430_e2832);
        locals.var_ftdbot = assign2430_e2833;
        let assign2440_e2836: f64 = (locals.var_auxt).powf(1.5);
        let assign2440_e2840: f64 = (locals.var_phigrsti * locals.var_phitrinv);
        let assign2440_e2843: f64 = (locals.var_phigdsti * locals.var_phitdinv);
        let assign2440_e2844: f64 = (assign2440_e2840 - assign2440_e2843);
        let assign2440_e2845: f64 = (0.5 * assign2440_e2844);
        let assign2440_e2846: f64 = (assign2440_e2845).exp();
        let assign2440_e2847: f64 = (assign2440_e2836 * assign2440_e2846);
        locals.var_ftdsti = assign2440_e2847;
        let assign2450_e2850: f64 = (locals.var_auxt).powf(1.5);
        let assign2450_e2854: f64 = (locals.var_phigrgat * locals.var_phitrinv);
        let assign2450_e2857: f64 = (locals.var_phigdgat * locals.var_phitdinv);
        let assign2450_e2858: f64 = (assign2450_e2854 - assign2450_e2857);
        let assign2450_e2859: f64 = (0.5 * assign2450_e2858);
        let assign2450_e2860: f64 = (assign2450_e2859).exp();
        let assign2450_e2861: f64 = (assign2450_e2850 * assign2450_e2860);
        locals.var_ftdgat = assign2450_e2861;
        let assign2460_e2864: f64 = (p.p837 * locals.var_ftdbot);
        let assign2460_e2866: f64 = (assign2460_e2864 * locals.var_ftdbot);
        locals.var_idsatbot = assign2460_e2866;
        let assign2470_e2869: f64 = (p.p838 * locals.var_ftdsti);
        let assign2470_e2871: f64 = (assign2470_e2869 * locals.var_ftdsti);
        locals.var_idsatsti = assign2470_e2871;
        let assign2480_e2874: f64 = (p.p839 * locals.var_ftdgat);
        let assign2480_e2876: f64 = (assign2480_e2874 * locals.var_ftdgat);
        locals.var_idsatgat = assign2480_e2876;
        let assign2490_e2879: f64 = (p.p828 * locals.var_auxt);
        let assign2490_e2882: f64 = (2.0 * locals.var_phitd);
        let assign2490_e2884: f64 = (locals.var_ftdbot).ln();
        let assign2490_e2885: f64 = (assign2490_e2882 * assign2490_e2884);
        let assign2490_e2886: f64 = (assign2490_e2879 - assign2490_e2885);
        locals.var_ubibot = assign2490_e2886;
        let assign2500_e2889: f64 = (p.p829 * locals.var_auxt);
        let assign2500_e2892: f64 = (2.0 * locals.var_phitd);
        let assign2500_e2894: f64 = (locals.var_ftdsti).ln();
        let assign2500_e2895: f64 = (assign2500_e2892 * assign2500_e2894);
        let assign2500_e2896: f64 = (assign2500_e2889 - assign2500_e2895);
        locals.var_ubisti = assign2500_e2896;
        let assign2510_e2899: f64 = (p.p830 * locals.var_auxt);
        let assign2510_e2902: f64 = (2.0 * locals.var_phitd);
        let assign2510_e2904: f64 = (locals.var_ftdgat).ln();
        let assign2510_e2905: f64 = (assign2510_e2902 * assign2510_e2904);
        let assign2510_e2906: f64 = (assign2510_e2899 - assign2510_e2905);
        locals.var_ubigat = assign2510_e2906;
        let assign2520_e2912: f64 = (0.05 - locals.var_ubibot);
        let assign2520_e2914: f64 = (assign2520_e2912 * locals.var_phitdinv);
        let assign2520_e2915: f64 = (assign2520_e2914).exp();
        let assign2520_e2916: f64 = (1.0 + assign2520_e2915);
        let assign2520_e2917: f64 = (assign2520_e2916).ln();
        let assign2520_e2918: f64 = (locals.var_phitd * assign2520_e2917);
        let assign2520_e2919: f64 = (locals.var_ubibot + assign2520_e2918);
        locals.var_vbibot = assign2520_e2919;
        let assign2530_e2925: f64 = (0.05 - locals.var_ubisti);
        let assign2530_e2927: f64 = (assign2530_e2925 * locals.var_phitdinv);
        let assign2530_e2928: f64 = (assign2530_e2927).exp();
        let assign2530_e2929: f64 = (1.0 + assign2530_e2928);
        let assign2530_e2930: f64 = (assign2530_e2929).ln();
        let assign2530_e2931: f64 = (locals.var_phitd * assign2530_e2930);
        let assign2530_e2932: f64 = (locals.var_ubisti + assign2530_e2931);
        locals.var_vbisti = assign2530_e2932;
        let assign2540_e2938: f64 = (0.05 - locals.var_ubigat);
        let assign2540_e2940: f64 = (assign2540_e2938 * locals.var_phitdinv);
        let assign2540_e2941: f64 = (assign2540_e2940).exp();
        let assign2540_e2942: f64 = (1.0 + assign2540_e2941);
        let assign2540_e2943: f64 = (assign2540_e2942).ln();
        let assign2540_e2944: f64 = (locals.var_phitd * assign2540_e2943);
        let assign2540_e2945: f64 = (locals.var_ubigat + assign2540_e2944);
        locals.var_vbigat = assign2540_e2945;
        let assign2550_e2948: f64 = (1.0 / locals.var_vbibot);
        locals.var_vbiinvbot = assign2550_e2948;
        let assign2560_e2951: f64 = (1.0 / locals.var_vbisti);
        locals.var_vbiinvsti = assign2560_e2951;
        let assign2570_e2954: f64 = (1.0 / locals.var_vbigat);
        locals.var_vbiinvgat = assign2570_e2954;
        let assign2580_e2958: f64 = (p.p828 * locals.var_vbiinvbot);
        let assign2580_e2960: f64 = (assign2580_e2958).powf(p.p831);
        let assign2580_e2961: f64 = (p.p825 * assign2580_e2960);
        locals.var_cjobot = assign2580_e2961;
        let assign2590_e2965: f64 = (p.p829 * locals.var_vbiinvsti);
        let assign2590_e2967: f64 = (assign2590_e2965).powf(p.p832);
        let assign2590_e2968: f64 = (p.p826 * assign2590_e2967);
        locals.var_cjosti = assign2590_e2968;
        let assign2600_e2972: f64 = (p.p830 * locals.var_vbiinvgat);
        let assign2600_e2974: f64 = (assign2600_e2972).powf(p.p833);
        let assign2600_e2975: f64 = (p.p827 * assign2600_e2974);
        locals.var_cjogat = assign2600_e2975;
        let assign2610_e2978: f64 = (locals.var_cjobot * locals.var_vbibot);
        let assign2610_e2980: f64 = (assign2610_e2978 * locals.var_one_over_one_minus_pbot);
        locals.var_qprefbot = assign2610_e2980;
        let assign2620_e2983: f64 = (locals.var_cjosti * locals.var_vbisti);
        let assign2620_e2985: f64 = (assign2620_e2983 * locals.var_one_over_one_minus_psti);
        locals.var_qprefsti = assign2620_e2985;
        let assign2630_e2988: f64 = (locals.var_cjogat * locals.var_vbigat);
        let assign2630_e2990: f64 = (assign2630_e2988 * locals.var_one_over_one_minus_pgat);
        locals.var_qprefgat = assign2630_e2990;
        let assign2640_e2993: f64 = (2.0 * locals.var_cjobot);
        locals.var_qpref2bot = assign2640_e2993;
        let assign2650_e2996: f64 = (2.0 * locals.var_cjosti);
        locals.var_qpref2sti = assign2650_e2996;
        let assign2660_e2999: f64 = (2.0 * locals.var_cjogat);
        locals.var_qpref2gat = assign2660_e2999;
        let assign2670_e3002: f64 = (0.5 * locals.var_phigdbot);
        let assign2670_e3004: f64 = (assign2670_e3002).max(locals.var_phitd);
        locals.var_deltaebot = assign2670_e3004;
        let assign2680_e3007: f64 = (0.5 * locals.var_phigdsti);
        let assign2680_e3009: f64 = (assign2680_e3007).max(locals.var_phitd);
        locals.var_deltaesti = assign2680_e3009;
        let assign2690_e3012: f64 = (0.5 * locals.var_phigdgat);
        let assign2690_e3014: f64 = (assign2690_e3012).max(locals.var_phitd);
        locals.var_deltaegat = assign2690_e3014;
        let assign2700_e3017: f64 = (locals.var_deltaebot * locals.var_phitdinv);
        locals.var_atatbot = assign2700_e3017;
        let assign2710_e3020: f64 = (locals.var_deltaesti * locals.var_phitdinv);
        locals.var_atatsti = assign2710_e3020;
        let assign2720_e3023: f64 = (locals.var_deltaegat * locals.var_phitdinv);
        locals.var_atatgat = assign2720_e3023;
        let assign2730_e3026: f64 = (32.0 * p.p848);
        let assign2730_e3028: f64 = (assign2730_e3026 * 9.1093826e-31);
        let assign2730_e3030: f64 = (assign2730_e3028 * 1.6021918e-19);
        let assign2730_e3033: f64 = (locals.var_deltaebot * locals.var_deltaebot);
        let assign2730_e3035: f64 = (assign2730_e3033 * locals.var_deltaebot);
        let assign2730_e3036: f64 = (assign2730_e3030 * assign2730_e3035);
        let assign2730_e3037: f64 = (assign2730_e3036).sqrt();
        let assign2730_e3040: f64 = (3.0 * 1.05457168e-34);
        let assign2730_e3041: f64 = (assign2730_e3037 / assign2730_e3040);
        locals.var_btatpartbot = assign2730_e3041;
        let assign2740_e3044: f64 = (32.0 * p.p849);
        let assign2740_e3046: f64 = (assign2740_e3044 * 9.1093826e-31);
        let assign2740_e3048: f64 = (assign2740_e3046 * 1.6021918e-19);
        let assign2740_e3051: f64 = (locals.var_deltaesti * locals.var_deltaesti);
        let assign2740_e3053: f64 = (assign2740_e3051 * locals.var_deltaesti);
        let assign2740_e3054: f64 = (assign2740_e3048 * assign2740_e3053);
        let assign2740_e3055: f64 = (assign2740_e3054).sqrt();
        let assign2740_e3058: f64 = (3.0 * 1.05457168e-34);
        let assign2740_e3059: f64 = (assign2740_e3055 / assign2740_e3058);
        locals.var_btatpartsti = assign2740_e3059;
        let assign2750_e3062: f64 = (32.0 * p.p850);
        let assign2750_e3064: f64 = (assign2750_e3062 * 9.1093826e-31);
        let assign2750_e3066: f64 = (assign2750_e3064 * 1.6021918e-19);
        let assign2750_e3069: f64 = (locals.var_deltaegat * locals.var_deltaegat);
        let assign2750_e3071: f64 = (assign2750_e3069 * locals.var_deltaegat);
        let assign2750_e3072: f64 = (assign2750_e3066 * assign2750_e3071);
        let assign2750_e3073: f64 = (assign2750_e3072).sqrt();
        let assign2750_e3076: f64 = (3.0 * 1.05457168e-34);
        let assign2750_e3077: f64 = (assign2750_e3073 / assign2750_e3076);
        locals.var_btatpartgat = assign2750_e3077;
        let assign2760_e3083: f64 = (locals.var_tkd_1 - locals.var_tkr_1);
        let assign2760_e3084: f64 = (p.p857 * assign2760_e3083);
        let assign2760_e3085: f64 = (1.0 + assign2760_e3084);
        let assign2760_e3086: f64 = (p.p854 * assign2760_e3085);
        locals.var_fbbtbot = assign2760_e3086;
        let assign2770_e3092: f64 = (locals.var_tkd_1 - locals.var_tkr_1);
        let assign2770_e3093: f64 = (p.p858 * assign2770_e3092);
        let assign2770_e3094: f64 = (1.0 + assign2770_e3093);
        let assign2770_e3095: f64 = (p.p855 * assign2770_e3094);
        locals.var_fbbtsti = assign2770_e3095;
        let assign2780_e3101: f64 = (locals.var_tkd_1 - locals.var_tkr_1);
        let assign2780_e3102: f64 = (p.p859 * assign2780_e3101);
        let assign2780_e3103: f64 = (1.0 + assign2780_e3102);
        let assign2780_e3104: f64 = (p.p856 * assign2780_e3103);
        (locals.var_fbbtgat, locals.var_fbbtgat_dn5, locals.var_fbbtgat_dn6, locals.var_fbbtgat_dn7, locals.var_fbbtgat_dn8, ) = (assign2780_e3104, 0.0, 0.0, 0.0, 0.0, );
        if (!(locals.var_fbbtbot > 0.0)) {
            locals.var_fbbtbot = 0.0;
        }
        if (!(locals.var_fbbtsti > 0.0)) {
            locals.var_fbbtsti = 0.0;
        }
        if (!(locals.var_fbbtgat > 0.0)) {
            (locals.var_fbbtgat, locals.var_fbbtgat_dn5, locals.var_fbbtgat_dn6, locals.var_fbbtgat_dn7, locals.var_fbbtgat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign2820_e3125: f64 = if locals.var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign2820_e3125;
        if (locals.var_guard32 != 0.0) {
            let assign2830_e3129: f64 = (locals.var_phiggat2nd + locals.var_deltaphigd);
            locals.var_phigdgat2nd = assign2830_e3129;
        }
        if (locals.var_guard32 != 0.0) {
            let assign2840_e3135: f64 = (locals.var_auxt).powf(1.5);
            let assign2840_e3139: f64 = (locals.var_phigrgat2nd * locals.var_phitrinv);
            let assign2840_e3142: f64 = (locals.var_phigdgat2nd * locals.var_phitdinv);
            let assign2840_e3143: f64 = (assign2840_e3139 - assign2840_e3142);
            let assign2840_e3144: f64 = (0.5 * assign2840_e3143);
            let assign2840_e3145: f64 = (assign2840_e3144).exp();
            let assign2840_e3146: f64 = (assign2840_e3135 * assign2840_e3145);
            locals.var_ftdgat2nd = assign2840_e3146;
        }
        if (locals.var_guard32 != 0.0) {
            let assign2850_e3152: f64 = (locals.var_vbirgat2nd * locals.var_auxt);
            let assign2850_e3155: f64 = (2.0 * locals.var_phitd);
            let assign2850_e3157: f64 = (locals.var_ftdgat2nd).ln();
            let assign2850_e3158: f64 = (assign2850_e3155 * assign2850_e3157);
            let assign2850_e3159: f64 = (assign2850_e3152 - assign2850_e3158);
            locals.var_ubigat2nd = assign2850_e3159;
        }
        if (locals.var_guard32 != 0.0) {
            let assign2860_e3168: f64 = (0.05 - locals.var_ubigat2nd);
            let assign2860_e3170: f64 = (assign2860_e3168 * locals.var_phitdinv);
            let assign2860_e3171: f64 = (assign2860_e3170).exp();
            let assign2860_e3172: f64 = (1.0 + assign2860_e3171);
            let assign2860_e3173: f64 = (assign2860_e3172).ln();
            let assign2860_e3174: f64 = (locals.var_phitd * assign2860_e3173);
            let assign2860_e3175: f64 = (locals.var_ubigat2nd + assign2860_e3174);
            locals.var_vbigat2nd = assign2860_e3175;
        }
        if (locals.var_guard32 != 0.0) {
            let assign2870_e3181: f64 = (1.0 / locals.var_vbigat2nd);
            locals.var_vbiinvgat2nd = assign2870_e3181;
        }
        if (locals.var_guard32 != 0.0) {
            let assign2880_e3188: f64 = (locals.var_vbirgat2nd * locals.var_vbiinvgat2nd);
            let assign2880_e3190: f64 = (assign2880_e3188).powf(locals.var_pgat2nd);
            let assign2880_e3191: f64 = (locals.var_cjorgat2nd * assign2880_e3190);
            locals.var_cjogat2nd = assign2880_e3191;
        }
        if (locals.var_guard32 != 0.0) {
            let assign2890_e3197: f64 = (locals.var_cjogat2nd * locals.var_vbigat2nd);
            let assign2890_e3199: f64 = (assign2890_e3197 * locals.var_one_over_one_minus_pgat2nd);
            locals.var_qprefgat2nd = assign2890_e3199;
        }
        if (locals.var_guard32 != 0.0) {
            let assign2900_e3205: f64 = (2.0 * locals.var_cjogat2nd);
            locals.var_qpref2gat2nd = assign2900_e3205;
        }
        let assign2910_e3210: f64 = (locals.var_phigbotd_i + locals.var_deltaphigd);
        locals.var_phigdbot_d = assign2910_e3210;
        let assign2920_e3213: f64 = (locals.var_phigstid_i + locals.var_deltaphigd);
        locals.var_phigdsti_d = assign2920_e3213;
        let assign2930_e3216: f64 = (locals.var_phiggatd_i + locals.var_deltaphigd);
        locals.var_phigdgat_d = assign2930_e3216;
        let assign2940_e3219: f64 = (locals.var_auxt).powf(1.5);
        let assign2940_e3223: f64 = (locals.var_phigrbot_d * locals.var_phitrinv);
        let assign2940_e3226: f64 = (locals.var_phigdbot_d * locals.var_phitdinv);
        let assign2940_e3227: f64 = (assign2940_e3223 - assign2940_e3226);
        let assign2940_e3228: f64 = (0.5 * assign2940_e3227);
        let assign2940_e3229: f64 = (assign2940_e3228).exp();
        let assign2940_e3230: f64 = (assign2940_e3219 * assign2940_e3229);
        locals.var_ftdbot_d = assign2940_e3230;
        let assign2950_e3233: f64 = (locals.var_auxt).powf(1.5);
        let assign2950_e3237: f64 = (locals.var_phigrsti_d * locals.var_phitrinv);
        let assign2950_e3240: f64 = (locals.var_phigdsti_d * locals.var_phitdinv);
        let assign2950_e3241: f64 = (assign2950_e3237 - assign2950_e3240);
        let assign2950_e3242: f64 = (0.5 * assign2950_e3241);
        let assign2950_e3243: f64 = (assign2950_e3242).exp();
        let assign2950_e3244: f64 = (assign2950_e3233 * assign2950_e3243);
        locals.var_ftdsti_d = assign2950_e3244;
        let assign2960_e3247: f64 = (locals.var_auxt).powf(1.5);
        let assign2960_e3251: f64 = (locals.var_phigrgat_d * locals.var_phitrinv);
        let assign2960_e3254: f64 = (locals.var_phigdgat_d * locals.var_phitdinv);
        let assign2960_e3255: f64 = (assign2960_e3251 - assign2960_e3254);
        let assign2960_e3256: f64 = (0.5 * assign2960_e3255);
        let assign2960_e3257: f64 = (assign2960_e3256).exp();
        let assign2960_e3258: f64 = (assign2960_e3247 * assign2960_e3257);
        locals.var_ftdgat_d = assign2960_e3258;
        let assign2970_e3261: f64 = (locals.var_idsatrbotd_i * locals.var_ftdbot_d);
        let assign2970_e3263: f64 = (assign2970_e3261 * locals.var_ftdbot_d);
        locals.var_idsatbot_d = assign2970_e3263;
        let assign2980_e3266: f64 = (locals.var_idsatrstid_i * locals.var_ftdsti_d);
        let assign2980_e3268: f64 = (assign2980_e3266 * locals.var_ftdsti_d);
        locals.var_idsatsti_d = assign2980_e3268;
        let assign2990_e3271: f64 = (locals.var_idsatrgatd_i * locals.var_ftdgat_d);
        let assign2990_e3273: f64 = (assign2990_e3271 * locals.var_ftdgat_d);
        locals.var_idsatgat_d = assign2990_e3273;
        let assign3000_e3276: f64 = (locals.var_vbirbotd_i * locals.var_auxt);
        let assign3000_e3279: f64 = (2.0 * locals.var_phitd);
        let assign3000_e3281: f64 = (locals.var_ftdbot_d).ln();
        let assign3000_e3282: f64 = (assign3000_e3279 * assign3000_e3281);
        let assign3000_e3283: f64 = (assign3000_e3276 - assign3000_e3282);
        locals.var_ubibot_d = assign3000_e3283;
        let assign3010_e3286: f64 = (locals.var_vbirstid_i * locals.var_auxt);
        let assign3010_e3289: f64 = (2.0 * locals.var_phitd);
        let assign3010_e3291: f64 = (locals.var_ftdsti_d).ln();
        let assign3010_e3292: f64 = (assign3010_e3289 * assign3010_e3291);
        let assign3010_e3293: f64 = (assign3010_e3286 - assign3010_e3292);
        locals.var_ubisti_d = assign3010_e3293;
        let assign3020_e3296: f64 = (locals.var_vbirgatd_i * locals.var_auxt);
        let assign3020_e3299: f64 = (2.0 * locals.var_phitd);
        let assign3020_e3301: f64 = (locals.var_ftdgat_d).ln();
        let assign3020_e3302: f64 = (assign3020_e3299 * assign3020_e3301);
        let assign3020_e3303: f64 = (assign3020_e3296 - assign3020_e3302);
        locals.var_ubigat_d = assign3020_e3303;
        let assign3030_e3309: f64 = (0.05 - locals.var_ubibot_d);
        let assign3030_e3311: f64 = (assign3030_e3309 * locals.var_phitdinv);
        let assign3030_e3312: f64 = (assign3030_e3311).exp();
        let assign3030_e3313: f64 = (1.0 + assign3030_e3312);
        let assign3030_e3314: f64 = (assign3030_e3313).ln();
        let assign3030_e3315: f64 = (locals.var_phitd * assign3030_e3314);
        let assign3030_e3316: f64 = (locals.var_ubibot_d + assign3030_e3315);
        locals.var_vbibot_d = assign3030_e3316;
        let assign3040_e3322: f64 = (0.05 - locals.var_ubisti_d);
        let assign3040_e3324: f64 = (assign3040_e3322 * locals.var_phitdinv);
        let assign3040_e3325: f64 = (assign3040_e3324).exp();
        let assign3040_e3326: f64 = (1.0 + assign3040_e3325);
        let assign3040_e3327: f64 = (assign3040_e3326).ln();
        let assign3040_e3328: f64 = (locals.var_phitd * assign3040_e3327);
        let assign3040_e3329: f64 = (locals.var_ubisti_d + assign3040_e3328);
        locals.var_vbisti_d = assign3040_e3329;
        let assign3050_e3335: f64 = (0.05 - locals.var_ubigat_d);
        let assign3050_e3337: f64 = (assign3050_e3335 * locals.var_phitdinv);
        let assign3050_e3338: f64 = (assign3050_e3337).exp();
        let assign3050_e3339: f64 = (1.0 + assign3050_e3338);
        let assign3050_e3340: f64 = (assign3050_e3339).ln();
        let assign3050_e3341: f64 = (locals.var_phitd * assign3050_e3340);
        let assign3050_e3342: f64 = (locals.var_ubigat_d + assign3050_e3341);
        locals.var_vbigat_d = assign3050_e3342;
        let assign3060_e3345: f64 = (1.0 / locals.var_vbibot_d);
        locals.var_vbiinvbot_d = assign3060_e3345;
        let assign3070_e3348: f64 = (1.0 / locals.var_vbisti_d);
        locals.var_vbiinvsti_d = assign3070_e3348;
        let assign3080_e3351: f64 = (1.0 / locals.var_vbigat_d);
        locals.var_vbiinvgat_d = assign3080_e3351;
        let assign3090_e3355: f64 = (locals.var_vbirbotd_i * locals.var_vbiinvbot_d);
        let assign3090_e3357: f64 = (assign3090_e3355).powf(locals.var_pbotd_i);
        let assign3090_e3358: f64 = (locals.var_cjorbotd_i * assign3090_e3357);
        locals.var_cjobot_d = assign3090_e3358;
        let assign3100_e3362: f64 = (locals.var_vbirstid_i * locals.var_vbiinvsti_d);
        let assign3100_e3364: f64 = (assign3100_e3362).powf(locals.var_pstid_i);
        let assign3100_e3365: f64 = (locals.var_cjorstid_i * assign3100_e3364);
        locals.var_cjosti_d = assign3100_e3365;
        let assign3110_e3369: f64 = (locals.var_vbirgatd_i * locals.var_vbiinvgat_d);
        let assign3110_e3371: f64 = (assign3110_e3369).powf(locals.var_pgatd_i);
        let assign3110_e3372: f64 = (locals.var_cjorgatd_i * assign3110_e3371);
        locals.var_cjogat_d = assign3110_e3372;
        let assign3120_e3375: f64 = (locals.var_cjobot_d * locals.var_vbibot_d);
        let assign3120_e3377: f64 = (assign3120_e3375 * locals.var_one_over_one_minus_pbot_d);
        locals.var_qprefbot_d = assign3120_e3377;
        let assign3130_e3380: f64 = (locals.var_cjosti_d * locals.var_vbisti_d);
        let assign3130_e3382: f64 = (assign3130_e3380 * locals.var_one_over_one_minus_psti_d);
        locals.var_qprefsti_d = assign3130_e3382;
        let assign3140_e3385: f64 = (locals.var_cjogat_d * locals.var_vbigat_d);
        let assign3140_e3387: f64 = (assign3140_e3385 * locals.var_one_over_one_minus_pgat_d);
        locals.var_qprefgat_d = assign3140_e3387;
        let assign3150_e3390: f64 = (2.0 * locals.var_cjobot_d);
        locals.var_qpref2bot_d = assign3150_e3390;
        let assign3160_e3393: f64 = (2.0 * locals.var_cjosti_d);
        locals.var_qpref2sti_d = assign3160_e3393;
        let assign3170_e3396: f64 = (2.0 * locals.var_cjogat_d);
        locals.var_qpref2gat_d = assign3170_e3396;
        let assign3180_e3399: f64 = (0.5 * locals.var_phigdbot_d);
        let assign3180_e3401: f64 = (assign3180_e3399).max(locals.var_phitd);
        locals.var_deltaebot_d = assign3180_e3401;
        let assign3190_e3404: f64 = (0.5 * locals.var_phigdsti_d);
        let assign3190_e3406: f64 = (assign3190_e3404).max(locals.var_phitd);
        locals.var_deltaesti_d = assign3190_e3406;
        let assign3200_e3409: f64 = (0.5 * locals.var_phigdgat_d);
        let assign3200_e3411: f64 = (assign3200_e3409).max(locals.var_phitd);
        locals.var_deltaegat_d = assign3200_e3411;
        let assign3210_e3414: f64 = (locals.var_deltaebot_d * locals.var_phitdinv);
        locals.var_atatbot_d = assign3210_e3414;
        let assign3220_e3417: f64 = (locals.var_deltaesti_d * locals.var_phitdinv);
        locals.var_atatsti_d = assign3220_e3417;
        let assign3230_e3420: f64 = (locals.var_deltaegat_d * locals.var_phitdinv);
        locals.var_atatgat_d = assign3230_e3420;
        let assign3240_e3423: f64 = (32.0 * locals.var_mefftatbotd_i);
        let assign3240_e3425: f64 = (assign3240_e3423 * 9.1093826e-31);
        let assign3240_e3427: f64 = (assign3240_e3425 * 1.6021918e-19);
        let assign3240_e3430: f64 = (locals.var_deltaebot_d * locals.var_deltaebot_d);
        let assign3240_e3432: f64 = (assign3240_e3430 * locals.var_deltaebot_d);
        let assign3240_e3433: f64 = (assign3240_e3427 * assign3240_e3432);
        let assign3240_e3434: f64 = (assign3240_e3433).sqrt();
        let assign3240_e3437: f64 = (3.0 * 1.05457168e-34);
        let assign3240_e3438: f64 = (assign3240_e3434 / assign3240_e3437);
        locals.var_btatpartbot_d = assign3240_e3438;
        let assign3250_e3441: f64 = (32.0 * locals.var_mefftatstid_i);
        let assign3250_e3443: f64 = (assign3250_e3441 * 9.1093826e-31);
        let assign3250_e3445: f64 = (assign3250_e3443 * 1.6021918e-19);
        let assign3250_e3448: f64 = (locals.var_deltaesti_d * locals.var_deltaesti_d);
        let assign3250_e3450: f64 = (assign3250_e3448 * locals.var_deltaesti_d);
        let assign3250_e3451: f64 = (assign3250_e3445 * assign3250_e3450);
        let assign3250_e3452: f64 = (assign3250_e3451).sqrt();
        let assign3250_e3455: f64 = (3.0 * 1.05457168e-34);
        let assign3250_e3456: f64 = (assign3250_e3452 / assign3250_e3455);
        locals.var_btatpartsti_d = assign3250_e3456;
        let assign3260_e3459: f64 = (32.0 * locals.var_mefftatgatd_i);
        let assign3260_e3461: f64 = (assign3260_e3459 * 9.1093826e-31);
        let assign3260_e3463: f64 = (assign3260_e3461 * 1.6021918e-19);
        let assign3260_e3466: f64 = (locals.var_deltaegat_d * locals.var_deltaegat_d);
        let assign3260_e3468: f64 = (assign3260_e3466 * locals.var_deltaegat_d);
        let assign3260_e3469: f64 = (assign3260_e3463 * assign3260_e3468);
        let assign3260_e3470: f64 = (assign3260_e3469).sqrt();
        let assign3260_e3473: f64 = (3.0 * 1.05457168e-34);
        let assign3260_e3474: f64 = (assign3260_e3470 / assign3260_e3473);
        locals.var_btatpartgat_d = assign3260_e3474;
        let assign3270_e3480: f64 = (locals.var_tkd_1 - locals.var_tkr_1);
        let assign3270_e3481: f64 = (locals.var_stfbbtbotd_i * assign3270_e3480);
        let assign3270_e3482: f64 = (1.0 + assign3270_e3481);
        let assign3270_e3483: f64 = (locals.var_fbbtrbotd_i * assign3270_e3482);
        locals.var_fbbtbot_d = assign3270_e3483;
        let assign3280_e3489: f64 = (locals.var_tkd_1 - locals.var_tkr_1);
        let assign3280_e3490: f64 = (locals.var_stfbbtstid_i * assign3280_e3489);
        let assign3280_e3491: f64 = (1.0 + assign3280_e3490);
        let assign3280_e3492: f64 = (locals.var_fbbtrstid_i * assign3280_e3491);
        locals.var_fbbtsti_d = assign3280_e3492;
        let assign3290_e3498: f64 = (locals.var_tkd_1 - locals.var_tkr_1);
        let assign3290_e3499: f64 = (locals.var_stfbbtgatd_i * assign3290_e3498);
        let assign3290_e3500: f64 = (1.0 + assign3290_e3499);
        let assign3290_e3501: f64 = (locals.var_fbbtrgatd_i * assign3290_e3500);
        (locals.var_fbbtgat_d, locals.var_fbbtgat_d_dn5, locals.var_fbbtgat_d_dn6, locals.var_fbbtgat_d_dn7, locals.var_fbbtgat_d_dn8, ) = (assign3290_e3501, 0.0, 0.0, 0.0, 0.0, );
        if (!(locals.var_fbbtbot_d > 0.0)) {
            locals.var_fbbtbot_d = 0.0;
        }
    }
    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (!(locals.var_fbbtsti_d > 0.0)) {
            locals.var_fbbtsti_d = 0.0;
        }
        if (!(locals.var_fbbtgat_d > 0.0)) {
            (locals.var_fbbtgat_d, locals.var_fbbtgat_d_dn5, locals.var_fbbtgat_d_dn6, locals.var_fbbtgat_d_dn7, locals.var_fbbtgat_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign3330_e3522: f64 = if locals.var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3330_e3522;
        if (locals.var_guard33 != 0.0) {
            let assign3340_e3526: f64 = (locals.var_phiggat2nd_d + locals.var_deltaphigd);
            locals.var_phigdgat2nd_d = assign3340_e3526;
        }
        if (locals.var_guard33 != 0.0) {
            let assign3350_e3532: f64 = (locals.var_auxt).powf(1.5);
            let assign3350_e3536: f64 = (locals.var_phigrgat2nd_d * locals.var_phitrinv);
            let assign3350_e3539: f64 = (locals.var_phigdgat2nd_d * locals.var_phitdinv);
            let assign3350_e3540: f64 = (assign3350_e3536 - assign3350_e3539);
            let assign3350_e3541: f64 = (0.5 * assign3350_e3540);
            let assign3350_e3542: f64 = (assign3350_e3541).exp();
            let assign3350_e3543: f64 = (assign3350_e3532 * assign3350_e3542);
            locals.var_ftdgat2nd_d = assign3350_e3543;
        }
        if (locals.var_guard33 != 0.0) {
            let assign3360_e3549: f64 = (locals.var_vbirgat2nd_d * locals.var_auxt);
            let assign3360_e3552: f64 = (2.0 * locals.var_phitd);
            let assign3360_e3554: f64 = (locals.var_ftdgat2nd_d).ln();
            let assign3360_e3555: f64 = (assign3360_e3552 * assign3360_e3554);
            let assign3360_e3556: f64 = (assign3360_e3549 - assign3360_e3555);
            locals.var_ubigat2nd_d = assign3360_e3556;
        }
        if (locals.var_guard33 != 0.0) {
            let assign3370_e3565: f64 = (0.05 - locals.var_ubigat2nd_d);
            let assign3370_e3567: f64 = (assign3370_e3565 * locals.var_phitdinv);
            let assign3370_e3568: f64 = (assign3370_e3567).exp();
            let assign3370_e3569: f64 = (1.0 + assign3370_e3568);
            let assign3370_e3570: f64 = (assign3370_e3569).ln();
            let assign3370_e3571: f64 = (locals.var_phitd * assign3370_e3570);
            let assign3370_e3572: f64 = (locals.var_ubigat2nd_d + assign3370_e3571);
            locals.var_vbigat2nd_d = assign3370_e3572;
        }
        if (locals.var_guard33 != 0.0) {
            let assign3380_e3578: f64 = (1.0 / locals.var_vbigat2nd_d);
            locals.var_vbiinvgat2nd_d = assign3380_e3578;
        }
        if (locals.var_guard33 != 0.0) {
            let assign3390_e3585: f64 = (locals.var_vbirgat2nd_d * locals.var_vbiinvgat2nd_d);
            let assign3390_e3587: f64 = (assign3390_e3585).powf(locals.var_pgat2nd_d);
            let assign3390_e3588: f64 = (locals.var_cjorgat2nd_d * assign3390_e3587);
            locals.var_cjogat2nd_d = assign3390_e3588;
        }
        if (locals.var_guard33 != 0.0) {
            let assign3400_e3594: f64 = (locals.var_cjogat2nd_d * locals.var_vbigat2nd_d);
            let assign3400_e3596: f64 = (assign3400_e3594 * locals.var_one_over_one_minus_pgat2nd_d);
            locals.var_qprefgat2nd_d = assign3400_e3596;
        }
        if (locals.var_guard33 != 0.0) {
            let assign3410_e3602: f64 = (2.0 * locals.var_cjogat2nd_d);
            locals.var_qpref2gat2nd_d = assign3410_e3602;
        }
        locals.var_nf_i = 1.0;
        locals.var_invnf = 1.0;
        locals.var_le = 0.0;
        locals.var_we = 0.0;
        locals.var_l_i = p.p0;
        locals.var_w_i = p.p1;
        locals.var_sa_i = p.p2;
        locals.var_sb_i = p.p3;
        locals.var_sd_i = p.p4;
        locals.var_sc_i = p.p8;
        locals.var_xgw_i = p.p11;
        locals.var_absource_i = p.p19;
        locals.var_lssource_i = p.p20;
        locals.var_lgsource_i = p.p21;
        locals.var_abdrain_i = p.p22;
        locals.var_lsdrain_i = p.p23;
        locals.var_lgdrain_i = p.p24;
        locals.var_as_i = p.p25;
        locals.var_ps_i = p.p26;
        locals.var_ad_i = p.p27;
        locals.var_pd_i = p.p28;
        locals.var_jw_i = p.p14;
        let assign3640_e3629: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3640_e3629;
        if (locals.var_guard34 != 0.0) {
            let (assign3650_e3636,) = {
    if (p.p9 > 1.0) {
        (p.p9,)
    } else {
        (1.0,)
    }
};
            locals.var_nf_i = assign3650_e3636;
        }
        if (locals.var_guard34 != 0.0) {
            let assign3660_e3642: f64 = (locals.var_nf_i + 0.5);
            let assign3660_e3643: f64 = (assign3660_e3642).floor();
            locals.var_nf_i = assign3660_e3643;
        }
        if (locals.var_guard34 != 0.0) {
            let assign3670_e3649: f64 = (1.0 / locals.var_nf_i);
            locals.var_invnf = assign3670_e3649;
        }
        let assign3680_e3654: f64 = (locals.var_w_i * locals.var_invnf);
        let (assign3680_e3661,) = {
    if (assign3680_e3654 > 1e-9) {
        let assign3680_e3659: f64 = (locals.var_w_i * locals.var_invnf);
        (assign3680_e3659,)
    } else {
        (1e-9,)
    }
};
        locals.var_w_i = assign3680_e3661;
        locals.var_sca_i = p.p5;
        locals.var_scb_i = p.p6;
        locals.var_scc_i = p.p7;
        let (assign3720_e3670,) = {
    if (p.p10 < 1.5) {
        (1.0,)
    } else {
        (2.0,)
    }
};
        locals.var_ngcon_i = assign3720_e3670;
        let assign3730_e3673: f64 = (1e-6 / locals.var_l_i);
        locals.var_il = assign3730_e3673;
        let assign3740_e3676: f64 = (1e-6 / locals.var_w_i);
        locals.var_iw = assign3740_e3676;
        let assign3750_e3681: f64 = (p.p189 * locals.var_il);
        let assign3750_e3682: f64 = (1.0 + assign3750_e3681);
        let assign3750_e3683: f64 = (p.p188 * assign3750_e3682);
        let assign3750_e3687: f64 = (p.p190 * locals.var_iw);
        let assign3750_e3688: f64 = (1.0 + assign3750_e3687);
        let assign3750_e3689: f64 = (assign3750_e3683 * assign3750_e3688);
        locals.var_dellps = assign3750_e3689;
        let assign3760_e3694: f64 = (p.p193 * locals.var_il);
        let assign3760_e3695: f64 = (1.0 + assign3760_e3694);
        let assign3760_e3696: f64 = (p.p192 * assign3760_e3695);
        let assign3760_e3700: f64 = (p.p194 * locals.var_iw);
        let assign3760_e3701: f64 = (1.0 + assign3760_e3700);
        let assign3760_e3702: f64 = (assign3760_e3696 * assign3760_e3701);
        locals.var_delwod = assign3760_e3702;
        let assign3770_e3705: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3770_e3708: f64 = (2.0 * p.p191);
        let assign3770_e3709: f64 = (assign3770_e3705 - assign3770_e3708);
        let (assign3770_e3720,) = {
    if (assign3770_e3709 > 1e-9) {
        let assign3770_e3714: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3770_e3717: f64 = (2.0 * p.p191);
        let assign3770_e3718: f64 = (assign3770_e3714 - assign3770_e3717);
        (assign3770_e3718,)
    } else {
        (1e-9,)
    }
};
        locals.var_le = assign3770_e3720;
        let assign3780_e3723: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3780_e3726: f64 = (2.0 * p.p195);
        let assign3780_e3727: f64 = (assign3780_e3723 - assign3780_e3726);
        let (assign3780_e3738,) = {
    if (assign3780_e3727 > 1e-9) {
        let assign3780_e3732: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3780_e3735: f64 = (2.0 * p.p195);
        let assign3780_e3736: f64 = (assign3780_e3732 - assign3780_e3735);
        (assign3780_e3736,)
    } else {
        (1e-9,)
    }
};
        locals.var_we = assign3780_e3738;
        let assign3790_e3741: f64 = (1e-6 / locals.var_le);
        locals.var_ile = assign3790_e3741;
        let assign3800_e3744: f64 = (locals.var_ile * locals.var_ile);
        locals.var_ile2 = assign3800_e3744;
        let assign3810_e3747: f64 = (1e-6 / locals.var_we);
        locals.var_iwe = assign3810_e3747;
        let assign3820_e3750: f64 = (1.0 / locals.var_iwe);
        locals.var_iiwe = assign3820_e3750;
        let assign3830_e3753: f64 = (locals.var_ile * locals.var_iwe);
        locals.var_iae = assign3830_e3753;
        let assign3840_e3756: f64 = (1.0 / locals.var_iae);
        locals.var_iiae = assign3840_e3756;
        let assign3850_e3759: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3850_e3762: f64 = (2.0 * p.p191);
        let assign3850_e3763: f64 = (assign3850_e3759 - assign3850_e3762);
        let assign3850_e3765: f64 = (assign3850_e3763 + p.p196);
        let (assign3850_e3778,) = {
    if (assign3850_e3765 > 1e-9) {
        let assign3850_e3770: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3850_e3773: f64 = (2.0 * p.p191);
        let assign3850_e3774: f64 = (assign3850_e3770 - assign3850_e3773);
        let assign3850_e3776: f64 = (assign3850_e3774 + p.p196);
        (assign3850_e3776,)
    } else {
        (1e-9,)
    }
};
        locals.var_lecv = assign3850_e3778;
        let assign3860_e3781: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3860_e3784: f64 = (2.0 * p.p195);
        let assign3860_e3785: f64 = (assign3860_e3781 - assign3860_e3784);
        let assign3860_e3787: f64 = (assign3860_e3785 + p.p197);
        let (assign3860_e3800,) = {
    if (assign3860_e3787 > 1e-9) {
        let assign3860_e3792: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3860_e3795: f64 = (2.0 * p.p195);
        let assign3860_e3796: f64 = (assign3860_e3792 - assign3860_e3795);
        let assign3860_e3798: f64 = (assign3860_e3796 + p.p197);
        (assign3860_e3798,)
    } else {
        (1e-9,)
    }
};
        locals.var_wecv = assign3860_e3800;
        let assign3870_e3803: f64 = (locals.var_wecv / 1e-6);
        locals.var_iiwecv = assign3870_e3803;
        let assign3880_e3806: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3880_e3808: f64 = (assign3880_e3806 + p.p196);
        let (assign3880_e3817,) = {
    if (assign3880_e3808 > 1e-9) {
        let assign3880_e3813: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3880_e3815: f64 = (assign3880_e3813 + p.p196);
        (assign3880_e3815,)
    } else {
        (1e-9,)
    }
};
        locals.var_lcv = assign3880_e3817;
        let assign3890_e3820: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3890_e3822: f64 = (assign3890_e3820 + p.p197);
        let (assign3890_e3831,) = {
    if (assign3890_e3822 > 1e-9) {
        let assign3890_e3827: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3890_e3829: f64 = (assign3890_e3827 + p.p197);
        (assign3890_e3829,)
    } else {
        (1e-9,)
    }
};
        locals.var_wcv = assign3890_e3831;
        let assign3900_e3834: f64 = (locals.var_lcv / 1e-6);
        locals.var_iilcv = assign3900_e3834;
        let assign3910_e3837: f64 = (locals.var_wcv / 1e-6);
        locals.var_iiwcv = assign3910_e3837;
        let assign3920_e3840: f64 = (locals.var_l_i + locals.var_dellps);
        let (assign3920_e3847,) = {
    if (assign3920_e3840 > 1e-9) {
        let assign3920_e3845: f64 = (locals.var_l_i + locals.var_dellps);
        (assign3920_e3845,)
    } else {
        (1e-9,)
    }
};
        locals.var_l_f = assign3920_e3847;
        let assign3930_e3850: f64 = (locals.var_l_f + p.p443);
        let (assign3930_e3857,) = {
    if (assign3930_e3850 > 1e-9) {
        let assign3930_e3855: f64 = (locals.var_l_f + p.p443);
        (assign3930_e3855,)
    } else {
        (1e-9,)
    }
};
        locals.var_l_slif = assign3930_e3857;
        let assign3940_e3860: f64 = (locals.var_w_i + locals.var_delwod);
        let (assign3940_e3867,) = {
    if (assign3940_e3860 > 1e-9) {
        let assign3940_e3865: f64 = (locals.var_w_i + locals.var_delwod);
        (assign3940_e3865,)
    } else {
        (1e-9,)
    }
};
        locals.var_w_f = assign3940_e3867;
        let assign3950_e3871: f64 = (0.5 * locals.var_delwod);
        let assign3950_e3872: f64 = (locals.var_xgw_i - assign3950_e3871);
        let (assign3950_e3881,) = {
    if (assign3950_e3872 > 1e-9) {
        let assign3950_e3878: f64 = (0.5 * locals.var_delwod);
        let assign3950_e3879: f64 = (locals.var_xgw_i - assign3950_e3878);
        (assign3950_e3879,)
    } else {
        (1e-9,)
    }
};
        locals.var_xgwe = assign3950_e3881;
        locals.var_vfb_p = p.p57;
        locals.var_stvfb_p = p.p58;
        locals.var_st2vfb_p = p.p59;
        locals.var_tox_p = p.p60;
        locals.var_epsrox_p = p.p61;
        locals.var_neff_p = p.p62;
        locals.var_gfacnud_p = p.p63;
        locals.var_vsbnud_p = p.p64;
        locals.var_dvsbnud_p = p.p65;
        locals.var_dphib_p = p.p66;
        locals.var_np_p = p.p67;
        locals.var_toxov_p = p.p68;
        locals.var_toxovd_p = p.p69;
        locals.var_nov_p = p.p70;
        locals.var_novd_p = p.p71;
        locals.var_ct_p = p.p72;
        locals.var_ctg_p = p.p74;
        locals.var_ctb_p = p.p73;
        locals.var_stct_p = p.p75;
        locals.var_psce_p = p.p79;
        locals.var_psced_p = p.p81;
        locals.var_psceb_p = p.p80;
        locals.var_cf_p = p.p76;
        locals.var_cfd_p = p.p78;
        locals.var_cfb_p = p.p77;
        locals.var_betn_p = p.p82;
        locals.var_stbet_p = p.p83;
        locals.var_mue_p = p.p84;
        locals.var_stmue_p = p.p85;
        locals.var_themu_p = p.p86;
        locals.var_stthemu_p = p.p87;
        locals.var_cs_p = p.p88;
        locals.var_stcs_p = p.p89;
        locals.var_thecs_p = p.p90;
        locals.var_stthecs_p = p.p91;
        locals.var_xcor_p = p.p92;
        locals.var_stxcor_p = p.p93;
        locals.var_feta_p = p.p94;
        locals.var_rs_p = p.p95;
        locals.var_strs_p = p.p96;
        locals.var_rsb_p = p.p97;
        locals.var_rsg_p = p.p98;
        locals.var_thesat_p = p.p99;
        locals.var_stthesat_p = p.p100;
        locals.var_thesatb_p = p.p101;
        locals.var_thesatg_p = p.p102;
        locals.var_thesatt_p = p.p103;
        locals.var_ax_p = p.p104;
        locals.var_alp_p = p.p105;
        locals.var_alp1_p = p.p106;
        locals.var_alp2_p = p.p107;
        locals.var_vp_p = p.p108;
        locals.var_a1_p = p.p109;
        locals.var_a2_p = p.p110;
        locals.var_sta2_p = p.p111;
        locals.var_a3_p = p.p112;
        locals.var_a4_p = p.p113;
        locals.var_imaxii_p = p.p114;
        locals.var_gco_p = p.p115;
        locals.var_iginv_p = p.p116;
        locals.var_igov_p = p.p117;
        locals.var_igovd_p = p.p118;
        locals.var_stig_p = p.p119;
        locals.var_gc2_p = p.p120;
        locals.var_gc3_p = p.p121;
        locals.var_gc2ov_p = p.p120;
        let assign4620_e3949: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4620_e3951: f64 = if assign4620_e3949 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign4620_e3951;
        if (locals.var_guard35 != 0.0) {
            locals.var_gc2ov_p = p.p122;
        }
        locals.var_gc3ov_p = p.p121;
        let assign4650_e3958: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4650_e3960: f64 = if assign4650_e3958 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign4650_e3960;
        if (locals.var_guard36 != 0.0) {
            locals.var_gc3ov_p = p.p123;
        }
        locals.var_gc2ovd_p = locals.var_gc2ov_p;
        let assign4680_e3967: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4680_e3969: f64 = if assign4680_e3967 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign4680_e3969;
        if (locals.var_guard37 != 0.0) {
            locals.var_gc2ovd_p = p.p124;
        }
        locals.var_gc3ovd_p = locals.var_gc3ov_p;
    }
    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign4710_e3976: f64 = if param_given[125] { 1.0 } else { 0.0 };
        let assign4710_e3978: f64 = if assign4710_e3976 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign4710_e3978;
        if (locals.var_guard38 != 0.0) {
            locals.var_gc3ovd_p = p.p125;
        }
        locals.var_chib_p = p.p126;
        locals.var_agidl_p = p.p127;
        locals.var_agidld_p = p.p128;
        locals.var_bgidl_p = p.p129;
        locals.var_bgidld_p = p.p130;
        locals.var_stbgidl_p = p.p131;
        locals.var_stbgidld_p = p.p132;
        locals.var_cgidl_p = p.p133;
        locals.var_cgidld_p = p.p134;
        locals.var_cox_p = p.p135;
        locals.var_delvtac_p = p.p136;
        locals.var_facneffac_p = p.p137;
        locals.var_thesatac_p = p.p99;
        let assign4860_e3997: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4860_e3999: f64 = if assign4860_e3997 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign4860_e3999;
        if (locals.var_guard39 != 0.0) {
            locals.var_thesatac_p = p.p138;
        }
        locals.var_axac_p = p.p104;
        let assign4890_e4006: f64 = if param_given[139] { 1.0 } else { 0.0 };
        let assign4890_e4008: f64 = if assign4890_e4006 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign4890_e4008;
        if (locals.var_guard40 != 0.0) {
            locals.var_axac_p = p.p139;
        }
        locals.var_alpac_p = p.p140;
        locals.var_alp1ac_p = p.p141;
        locals.var_cgov_p = p.p142;
        locals.var_cgovd_p = p.p143;
        locals.var_fcgovacc_p = p.p144;
        locals.var_fcgovaccd_p = p.p145;
        locals.var_cgovaccg_p = p.p146;
        locals.var_cgbov_p = p.p147;
        locals.var_cinr_p = p.p148;
        locals.var_cinrd_p = p.p149;
        locals.var_dvfbinr_p = p.p150;
        locals.var_fcinrdep_p = p.p151;
        locals.var_fcinracc_p = p.p152;
        locals.var_axinr_p = p.p153;
        locals.var_cfr_p = p.p154;
        locals.var_cfrd_p = p.p155;
        locals.var_fnt_p = p.p156;
        locals.var_fntexc_p = p.p157;
        locals.var_vfbedge_p = p.p162;
        locals.var_stvfbedge_p = p.p163;
        locals.var_dphibedge_p = p.p164;
        locals.var_neffedge_p = p.p165;
        locals.var_ctedge_p = p.p166;
        locals.var_betnedge_p = p.p167;
        locals.var_stbetedge_p = p.p168;
        locals.var_psceedge_p = p.p169;
        locals.var_pscebedge_p = p.p170;
        locals.var_pscededge_p = p.p171;
        locals.var_cfedge_p = p.p172;
        locals.var_cfdedge_p = p.p174;
        locals.var_cfbedge_p = p.p173;
        locals.var_rg_p = p.p180;
        locals.var_rse_p = p.p181;
        locals.var_rde_p = p.p182;
        locals.var_rwell_p = p.p184;
        locals.var_rbulk_p = p.p183;
        locals.var_rjuns_p = p.p185;
        locals.var_rjund_p = p.p186;
        locals.var_munqs_p = p.p187;
        let assign5390_e4063: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign5390_e4063;
        if (locals.var_guard41 != 0.0) {
            let assign5400_e4069: f64 = (locals.var_ile).powf(p.p200);
            let assign5400_e4070: f64 = (p.p199 * assign5400_e4069);
            let assign5400_e4071: f64 = (p.p198 + assign5400_e4070);
            let assign5400_e4074: f64 = (p.p201 * locals.var_iwe);
            let assign5400_e4075: f64 = (assign5400_e4071 + assign5400_e4074);
            let assign5400_e4078: f64 = (p.p202 * locals.var_iae);
            let assign5400_e4079: f64 = (assign5400_e4075 + assign5400_e4078);
            locals.var_vfb_p = assign5400_e4079;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5410_e4086: f64 = (p.p204 * locals.var_ile);
            let assign5410_e4087: f64 = (p.p203 + assign5410_e4086);
            let assign5410_e4090: f64 = (p.p205 * locals.var_iwe);
            let assign5410_e4091: f64 = (assign5410_e4087 + assign5410_e4090);
            let assign5410_e4094: f64 = (p.p206 * locals.var_iae);
            let assign5410_e4095: f64 = (assign5410_e4091 + assign5410_e4094);
            locals.var_stvfb_p = assign5410_e4095;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_st2vfb_p = p.p207;
            locals.var_tox_p = p.p208;
            locals.var_epsrox_p = p.p209;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5450_e4115: f64 = (p.p211 * locals.var_iwe);
            let assign5450_e4119: f64 = (locals.var_we / p.p212);
            let assign5450_e4120: f64 = (1.0 + assign5450_e4119);
            let assign5450_e4121: f64 = (assign5450_e4120).ln();
            let assign5450_e4122: f64 = (assign5450_e4115 * assign5450_e4121);
            let assign5450_e4123: f64 = (1.0 + assign5450_e4122);
            let (assign5450_e4139,) = {
    if (assign5450_e4123 > 0.001) {
        let assign5450_e4129: f64 = (p.p211 * locals.var_iwe);
        let assign5450_e4133: f64 = (locals.var_we / p.p212);
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
            locals.var_nsub0e = assign5450_e4140;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5460_e4148: f64 = (p.p214 * locals.var_iwe);
            let assign5460_e4152: f64 = (locals.var_we / p.p215);
            let assign5460_e4153: f64 = (1.0 + assign5460_e4152);
            let assign5460_e4154: f64 = (assign5460_e4153).ln();
            let assign5460_e4155: f64 = (assign5460_e4148 * assign5460_e4154);
            let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
            let (assign5460_e4172,) = {
    if (assign5460_e4156 > 0.001) {
        let assign5460_e4162: f64 = (p.p214 * locals.var_iwe);
        let assign5460_e4166: f64 = (locals.var_we / p.p215);
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
            locals.var_npcke = assign5460_e4173;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5470_e4181: f64 = (p.p217 * locals.var_iwe);
            let assign5470_e4185: f64 = (locals.var_we / p.p215);
            let assign5470_e4186: f64 = (1.0 + assign5470_e4185);
            let assign5470_e4187: f64 = (assign5470_e4186).ln();
            let assign5470_e4188: f64 = (assign5470_e4181 * assign5470_e4187);
            let assign5470_e4189: f64 = (1.0 + assign5470_e4188);
            let (assign5470_e4205,) = {
    if (assign5470_e4189 > 0.001) {
        let assign5470_e4195: f64 = (p.p217 * locals.var_iwe);
        let assign5470_e4199: f64 = (locals.var_we / p.p215);
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
            locals.var_lpcke = assign5470_e4206;
        }
        let assign5480_e4212: f64 = (2.0 * locals.var_lpcke);
        let assign5480_e4213: f64 = if locals.var_le > assign5480_e4212 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign5480_e4213;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
            locals.var_aa = 75000000000.0;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
            let assign5500_e4226: f64 = (0.5 * locals.var_npcke);
            let assign5500_e4227: f64 = (locals.var_nsub0e + assign5500_e4226);
            let assign5500_e4228: f64 = (assign5500_e4227).sqrt();
            let assign5500_e4230: f64 = (locals.var_nsub0e).sqrt();
            let assign5500_e4231: f64 = (assign5500_e4228 - assign5500_e4230);
            locals.var_bb = assign5500_e4231;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
            let assign5510_e4238: f64 = (locals.var_nsub0e).sqrt();
            let assign5510_e4243: f64 = (2.0 * locals.var_lpcke);
            let assign5510_e4245: f64 = (assign5510_e4243 / locals.var_le);
            let assign5510_e4248: f64 = (locals.var_bb / locals.var_aa);
            let assign5510_e4249: f64 = (assign5510_e4248).exp();
            let assign5510_e4251: f64 = (assign5510_e4249 - 1.0);
            let assign5510_e4252: f64 = (assign5510_e4245 * assign5510_e4251);
            let assign5510_e4253: f64 = (1.0 + assign5510_e4252);
            let assign5510_e4254: f64 = (assign5510_e4253).ln();
            let assign5510_e4255: f64 = (locals.var_aa * assign5510_e4254);
            let assign5510_e4256: f64 = (assign5510_e4238 + assign5510_e4255);
            locals.var_nsub = assign5510_e4256;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
            let assign5520_e4264: f64 = (locals.var_nsub * locals.var_nsub);
            locals.var_nsub = assign5520_e4264;
        }
        let assign5530_e4269: f64 = if locals.var_le >= locals.var_lpcke { 1.0 } else { 0.0 };
        locals.var_guard43 = assign5530_e4269;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 != 0.0)) {
            let assign5540_e4279: f64 = (locals.var_npcke * locals.var_lpcke);
            let assign5540_e4281: f64 = (assign5540_e4279 / locals.var_le);
            let assign5540_e4282: f64 = (locals.var_nsub0e + assign5540_e4281);
            locals.var_nsub = assign5540_e4282;
        }
        if (((locals.var_guard41 != 0.0) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 == 0.0)) {
            let assign5550_e4297: f64 = (locals.var_le / locals.var_lpcke);
            let assign5550_e4298: f64 = (2.0 - assign5550_e4297);
            let assign5550_e4299: f64 = (locals.var_npcke * assign5550_e4298);
            let assign5550_e4300: f64 = (locals.var_nsub0e + assign5550_e4299);
            locals.var_nsub = assign5550_e4300;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5560_e4308: f64 = (p.p218 * locals.var_ile);
            let assign5560_e4309: f64 = (1.0 - assign5560_e4308);
            let assign5560_e4312: f64 = (p.p219 * locals.var_ile2);
            let assign5560_e4313: f64 = (assign5560_e4309 - assign5560_e4312);
            let assign5560_e4314: f64 = (locals.var_nsub * assign5560_e4313);
            locals.var_neff_p = assign5560_e4314;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5570_e4322: f64 = (locals.var_ile).powf(p.p222);
            let assign5570_e4323: f64 = (p.p221 * assign5570_e4322);
            let assign5570_e4324: f64 = (p.p220 + assign5570_e4323);
            let assign5570_e4327: f64 = (p.p223 * locals.var_iwe);
            let assign5570_e4328: f64 = (assign5570_e4324 + assign5570_e4327);
            let assign5570_e4331: f64 = (p.p224 * locals.var_iae);
            let assign5570_e4332: f64 = (assign5570_e4328 + assign5570_e4331);
            locals.var_gfacnud_p = assign5570_e4332;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_vsbnud_p = p.p225;
            locals.var_dvsbnud_p = p.p226;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5600_e4348: f64 = (locals.var_ile).powf(p.p229);
            let assign5600_e4349: f64 = (p.p228 * assign5600_e4348);
            let assign5600_e4350: f64 = (p.p227 + assign5600_e4349);
            let assign5600_e4353: f64 = (p.p230 * locals.var_iwe);
            let assign5600_e4354: f64 = (assign5600_e4350 + assign5600_e4353);
            let assign5600_e4357: f64 = (p.p231 * locals.var_iae);
            let assign5600_e4358: f64 = (assign5600_e4354 + assign5600_e4357);
            locals.var_dphib_p = assign5600_e4358;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5610_e4367: f64 = (p.p233 * locals.var_ile);
            let assign5610_e4368: f64 = (1.0 + assign5610_e4367);
            let (assign5610_e4376,) = {
    if (1e-6 > assign5610_e4368) {
        (1e-6,)
    } else {
        let assign5610_e4374: f64 = (p.p233 * locals.var_ile);
        let assign5610_e4375: f64 = (1.0 + assign5610_e4374);
        (assign5610_e4375,)
    }
};
            let assign5610_e4377: f64 = (p.p232 * assign5610_e4376);
            locals.var_np_p = assign5610_e4377;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_toxov_p = p.p234;
            locals.var_toxovd_p = p.p235;
            locals.var_nov_p = p.p238;
            locals.var_novd_p = p.p239;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5660_e4401: f64 = (locals.var_ile).powf(p.p242);
            let assign5660_e4402: f64 = (p.p241 * assign5660_e4401);
            let assign5660_e4403: f64 = (p.p240 + assign5660_e4402);
            let assign5660_e4407: f64 = (p.p243 * locals.var_iwe);
            let assign5660_e4408: f64 = (1.0 + assign5660_e4407);
            let assign5660_e4409: f64 = (assign5660_e4403 * assign5660_e4408);
            let assign5660_e4413: f64 = (p.p244 * locals.var_iae);
            let assign5660_e4414: f64 = (1.0 + assign5660_e4413);
            let assign5660_e4415: f64 = (assign5660_e4409 * assign5660_e4414);
            locals.var_ct_p = assign5660_e4415;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_ctg_p = p.p246;
            locals.var_ctb_p = p.p245;
            locals.var_stct_p = p.p247;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5700_e4434: f64 = (locals.var_ile).powf(p.p249);
            let assign5700_e4435: f64 = (p.p248 * assign5700_e4434);
            let assign5700_e4439: f64 = (p.p250 * locals.var_iwe);
            let assign5700_e4440: f64 = (1.0 + assign5700_e4439);
            let assign5700_e4441: f64 = (assign5700_e4435 * assign5700_e4440);
            locals.var_cf_p = assign5700_e4441;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_cfd_p = p.p252;
            locals.var_cfb_p = p.p251;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5730_e4456: f64 = (locals.var_ile).powf(p.p254);
            let assign5730_e4457: f64 = (p.p253 * assign5730_e4456);
            let assign5730_e4461: f64 = (p.p255 * locals.var_iwe);
            let assign5730_e4462: f64 = (1.0 + assign5730_e4461);
            let assign5730_e4463: f64 = (assign5730_e4457 * assign5730_e4462);
            locals.var_psce_p = assign5730_e4463;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_psced_p = p.p257;
            locals.var_psceb_p = p.p256;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5760_e4479: f64 = (p.p260 * locals.var_iwe);
            let assign5760_e4480: f64 = (1.0 + assign5760_e4479);
            let assign5760_e4481: f64 = (p.p259 * assign5760_e4480);
            locals.var_fbet1e = assign5760_e4481;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5770_e4489: f64 = (p.p262 * locals.var_iwe);
            let assign5770_e4490: f64 = (1.0 + assign5770_e4489);
            let (assign5770_e4499,) = {
    if (assign5770_e4490 > 0.001) {
        let assign5770_e4496: f64 = (p.p262 * locals.var_iwe);
        let assign5770_e4497: f64 = (1.0 + assign5770_e4496);
        (assign5770_e4497,)
    } else {
        (0.001,)
    }
};
            let assign5770_e4500: f64 = (p.p261 * assign5770_e4499);
            locals.var_lp1e = assign5770_e4500;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5780_e4507: f64 = (locals.var_fbet1e * locals.var_lp1e);
            let assign5780_e4509: f64 = (assign5780_e4507 / locals.var_le);
            let assign5780_e4512: f64 = (-locals.var_le);
            let assign5780_e4514: f64 = (assign5780_e4512 / locals.var_lp1e);
            let assign5780_e4515: f64 = (assign5780_e4514).exp();
            let assign5780_e4516: f64 = (1.0 - assign5780_e4515);
            let assign5780_e4517: f64 = (assign5780_e4509 * assign5780_e4516);
            let assign5780_e4518: f64 = (1.0 + assign5780_e4517);
            let assign5780_e4521: f64 = (p.p263 * p.p264);
            let assign5780_e4523: f64 = (assign5780_e4521 / locals.var_le);
            let assign5780_e4526: f64 = (-locals.var_le);
            let assign5780_e4528: f64 = (assign5780_e4526 / p.p264);
            let assign5780_e4529: f64 = (assign5780_e4528).exp();
            let assign5780_e4530: f64 = (1.0 - assign5780_e4529);
            let assign5780_e4531: f64 = (assign5780_e4523 * assign5780_e4530);
            let assign5780_e4532: f64 = (assign5780_e4518 + assign5780_e4531);
            locals.var_gpe = assign5780_e4532;
        }
        if (locals.var_guard41 != 0.0) {
            let (assign5790_e4541,) = {
    if (locals.var_gpe > 1e-15) {
        (locals.var_gpe,)
    } else {
        (1e-15,)
    }
};
            locals.var_gpe = assign5790_e4541;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5800_e4548: f64 = (p.p265 * locals.var_iwe);
            let assign5800_e4549: f64 = (1.0 + assign5800_e4548);
            let assign5800_e4552: f64 = (p.p266 * locals.var_iwe);
            let assign5800_e4556: f64 = (locals.var_we / p.p267);
            let assign5800_e4557: f64 = (1.0 + assign5800_e4556);
            let assign5800_e4558: f64 = (assign5800_e4557).ln();
            let assign5800_e4559: f64 = (assign5800_e4552 * assign5800_e4558);
            let assign5800_e4560: f64 = (assign5800_e4549 + assign5800_e4559);
            locals.var_gwe = assign5800_e4560;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5810_e4566: f64 = (p.p258 * locals.var_we);
            let assign5810_e4569: f64 = (locals.var_gpe * locals.var_le);
            let assign5810_e4570: f64 = (assign5810_e4566 / assign5810_e4569);
            let assign5810_e4572: f64 = (assign5810_e4570 * locals.var_gwe);
            locals.var_betn_p = assign5810_e4572;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5820_e4579: f64 = (p.p269 * locals.var_ile);
            let assign5820_e4580: f64 = (p.p268 + assign5820_e4579);
            let assign5820_e4583: f64 = (p.p270 * locals.var_iwe);
            let assign5820_e4584: f64 = (assign5820_e4580 + assign5820_e4583);
            let assign5820_e4587: f64 = (p.p271 * locals.var_iae);
            let assign5820_e4588: f64 = (assign5820_e4584 + assign5820_e4587);
            locals.var_stbet_p = assign5820_e4588;
        }
    }
    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard41 != 0.0) {
            let assign5830_e4596: f64 = (p.p273 * locals.var_iwe);
            let assign5830_e4597: f64 = (1.0 + assign5830_e4596);
            let assign5830_e4598: f64 = (p.p272 * assign5830_e4597);
            locals.var_mue_p = assign5830_e4598;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_stmue_p = p.p274;
            locals.var_themu_p = p.p275;
            locals.var_stthemu_p = p.p276;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5870_e4618: f64 = (locals.var_ile).powf(p.p279);
            let assign5870_e4619: f64 = (p.p278 * assign5870_e4618);
            let assign5870_e4620: f64 = (p.p277 + assign5870_e4619);
            let assign5870_e4624: f64 = (p.p280 * locals.var_iwe);
            let assign5870_e4625: f64 = (1.0 + assign5870_e4624);
            let assign5870_e4626: f64 = (assign5870_e4620 * assign5870_e4625);
            let assign5870_e4630: f64 = (p.p281 * locals.var_iae);
            let assign5870_e4631: f64 = (1.0 + assign5870_e4630);
            let assign5870_e4632: f64 = (assign5870_e4626 * assign5870_e4631);
            locals.var_cs_p = assign5870_e4632;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_stcs_p = p.p282;
            locals.var_thecs_p = p.p283;
            locals.var_stthecs_p = p.p284;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5910_e4652: f64 = (p.p286 * locals.var_ile);
            let assign5910_e4653: f64 = (1.0 + assign5910_e4652);
            let assign5910_e4654: f64 = (p.p285 * assign5910_e4653);
            let assign5910_e4658: f64 = (p.p287 * locals.var_iwe);
            let assign5910_e4659: f64 = (1.0 + assign5910_e4658);
            let assign5910_e4660: f64 = (assign5910_e4654 * assign5910_e4659);
            let assign5910_e4664: f64 = (p.p288 * locals.var_iae);
            let assign5910_e4665: f64 = (1.0 + assign5910_e4664);
            let assign5910_e4666: f64 = (assign5910_e4660 * assign5910_e4665);
            locals.var_xcor_p = assign5910_e4666;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_stxcor_p = p.p289;
            locals.var_feta_p = p.p290;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5940_e4680: f64 = (p.p291 * locals.var_iwe);
            let assign5940_e4684: f64 = (p.p292 * locals.var_iwe);
            let assign5940_e4685: f64 = (1.0 + assign5940_e4684);
            let assign5940_e4686: f64 = (assign5940_e4680 * assign5940_e4685);
            locals.var_rs_p = assign5940_e4686;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_strs_p = p.p293;
            locals.var_rsb_p = p.p294;
            locals.var_rsg_p = p.p295;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5980_e4705: f64 = (p.p297 * locals.var_gwe);
            let assign5980_e4707: f64 = (assign5980_e4705 / locals.var_gpe);
            let assign5980_e4710: f64 = (locals.var_ile).powf(p.p298);
            let assign5980_e4711: f64 = (assign5980_e4707 * assign5980_e4710);
            let assign5980_e4712: f64 = (p.p296 + assign5980_e4711);
            let assign5980_e4716: f64 = (p.p299 * locals.var_iwe);
            let assign5980_e4717: f64 = (1.0 + assign5980_e4716);
            let assign5980_e4718: f64 = (assign5980_e4712 * assign5980_e4717);
            let assign5980_e4722: f64 = (p.p300 * locals.var_iae);
            let assign5980_e4723: f64 = (1.0 + assign5980_e4722);
            let assign5980_e4724: f64 = (assign5980_e4718 * assign5980_e4723);
            locals.var_thesat_p = assign5980_e4724;
        }
        if (locals.var_guard41 != 0.0) {
            let assign5990_e4731: f64 = (p.p302 * locals.var_ile);
            let assign5990_e4732: f64 = (p.p301 + assign5990_e4731);
            let assign5990_e4735: f64 = (p.p303 * locals.var_iwe);
            let assign5990_e4736: f64 = (assign5990_e4732 + assign5990_e4735);
            let assign5990_e4739: f64 = (p.p304 * locals.var_iae);
            let assign5990_e4740: f64 = (assign5990_e4736 + assign5990_e4739);
            locals.var_stthesat_p = assign5990_e4740;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_thesatb_p = p.p305;
            locals.var_thesatg_p = p.p306;
            locals.var_thesatt_p = p.p307;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6030_e4760: f64 = (p.p309 * locals.var_ile);
            let assign6030_e4761: f64 = (1.0 + assign6030_e4760);
            let assign6030_e4762: f64 = (p.p308 / assign6030_e4761);
            locals.var_ax_p = assign6030_e4762;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6040_e4769: f64 = (locals.var_ile).powf(p.p311);
            let assign6040_e4770: f64 = (p.p310 * assign6040_e4769);
            let assign6040_e4774: f64 = (p.p312 * locals.var_iwe);
            let assign6040_e4775: f64 = (1.0 + assign6040_e4774);
            let assign6040_e4776: f64 = (assign6040_e4770 * assign6040_e4775);
            locals.var_alp_p = assign6040_e4776;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6050_e4782: f64 = (locals.var_ile).powf(p.p314);
            locals.var_tmpx = assign6050_e4782;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6060_e4788: f64 = (p.p313 * locals.var_tmpx);
            let assign6060_e4792: f64 = (p.p316 * locals.var_iwe);
            let assign6060_e4793: f64 = (1.0 + assign6060_e4792);
            let assign6060_e4794: f64 = (assign6060_e4788 * assign6060_e4793);
            let assign6060_e4798: f64 = (p.p315 * locals.var_ile);
            let assign6060_e4800: f64 = (assign6060_e4798 * locals.var_tmpx);
            let assign6060_e4801: f64 = (1.0 + assign6060_e4800);
            let assign6060_e4802: f64 = (assign6060_e4794 / assign6060_e4801);
            locals.var_alp1_p = assign6060_e4802;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6070_e4808: f64 = (locals.var_ile).powf(p.p318);
            locals.var_tmpx = assign6070_e4808;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6080_e4814: f64 = (p.p317 * locals.var_tmpx);
            let assign6080_e4818: f64 = (p.p320 * locals.var_iwe);
            let assign6080_e4819: f64 = (1.0 + assign6080_e4818);
            let assign6080_e4820: f64 = (assign6080_e4814 * assign6080_e4819);
            let assign6080_e4824: f64 = (p.p319 * locals.var_ile);
            let assign6080_e4826: f64 = (assign6080_e4824 * locals.var_tmpx);
            let assign6080_e4827: f64 = (1.0 + assign6080_e4826);
            let assign6080_e4828: f64 = (assign6080_e4820 / assign6080_e4827);
            locals.var_alp2_p = assign6080_e4828;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_vp_p = p.p321;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6100_e4840: f64 = (p.p323 * locals.var_ile);
            let assign6100_e4841: f64 = (1.0 + assign6100_e4840);
            let assign6100_e4842: f64 = (p.p322 * assign6100_e4841);
            let assign6100_e4846: f64 = (p.p324 * locals.var_iwe);
            let assign6100_e4847: f64 = (1.0 + assign6100_e4846);
            let assign6100_e4848: f64 = (assign6100_e4842 * assign6100_e4847);
            locals.var_a1_p = assign6100_e4848;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_a2_p = p.p325;
            locals.var_sta2_p = p.p326;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6130_e4864: f64 = (p.p328 * locals.var_ile);
            let assign6130_e4865: f64 = (1.0 + assign6130_e4864);
            let assign6130_e4866: f64 = (p.p327 * assign6130_e4865);
            let assign6130_e4870: f64 = (p.p329 * locals.var_iwe);
            let assign6130_e4871: f64 = (1.0 + assign6130_e4870);
            let assign6130_e4872: f64 = (assign6130_e4866 * assign6130_e4871);
            locals.var_a3_p = assign6130_e4872;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6140_e4880: f64 = (p.p331 * locals.var_ile);
            let assign6140_e4881: f64 = (1.0 + assign6140_e4880);
            let assign6140_e4882: f64 = (p.p330 * assign6140_e4881);
            let assign6140_e4886: f64 = (p.p332 * locals.var_iwe);
            let assign6140_e4887: f64 = (1.0 + assign6140_e4886);
            let assign6140_e4888: f64 = (assign6140_e4882 * assign6140_e4887);
            locals.var_a4_p = assign6140_e4888;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_imaxii_p = p.p333;
            locals.var_gco_p = p.p334;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6170_e4902: f64 = (p.p335 / locals.var_iae);
            locals.var_iginv_p = assign6170_e4902;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6180_e4908: f64 = (p.p336 * p.p236);
            let assign6180_e4911: f64 = (1e-6 * locals.var_iwe);
            let assign6180_e4912: f64 = (assign6180_e4908 / assign6180_e4911);
            locals.var_igov_p = assign6180_e4912;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6190_e4918: f64 = (p.p337 * p.p237);
            let assign6190_e4921: f64 = (1e-6 * locals.var_iwe);
            let assign6190_e4922: f64 = (assign6190_e4918 / assign6190_e4921);
            locals.var_igovd_p = assign6190_e4922;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_stig_p = p.p338;
            locals.var_gc2_p = p.p339;
            locals.var_gc3_p = p.p340;
            locals.var_gc2ov_p = p.p339;
        }
        let assign6240_e4942: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6240_e4944: f64 = if assign6240_e4942 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign6240_e4944;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard44 != 0.0)) {
            locals.var_gc2ov_p = p.p341;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_gc3ov_p = p.p340;
        }
        let assign6270_e4956: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6270_e4958: f64 = if assign6270_e4956 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign6270_e4958;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard45 != 0.0)) {
            locals.var_gc3ov_p = p.p342;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_gc2ovd_p = locals.var_gc2ov_p;
        }
        let assign6300_e4970: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6300_e4972: f64 = if assign6300_e4970 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign6300_e4972;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard46 != 0.0)) {
            locals.var_gc2ovd_p = p.p343;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_gc3ovd_p = locals.var_gc3ov_p;
        }
        let assign6330_e4984: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6330_e4986: f64 = if assign6330_e4984 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign6330_e4986;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard47 != 0.0)) {
            locals.var_gc3ovd_p = p.p344;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_chib_p = p.p345;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6360_e5000: f64 = (p.p346 * p.p236);
            let assign6360_e5003: f64 = (1e-6 * locals.var_iwe);
            let assign6360_e5004: f64 = (assign6360_e5000 / assign6360_e5003);
            locals.var_agidl_p = assign6360_e5004;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6370_e5010: f64 = (p.p347 * p.p237);
            let assign6370_e5013: f64 = (1e-6 * locals.var_iwe);
            let assign6370_e5014: f64 = (assign6370_e5010 / assign6370_e5013);
            locals.var_agidld_p = assign6370_e5014;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_bgidl_p = p.p348;
            locals.var_bgidld_p = p.p349;
            locals.var_stbgidl_p = p.p350;
            locals.var_stbgidld_p = p.p351;
            locals.var_cgidl_p = p.p352;
            locals.var_cgidld_p = p.p353;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6440_e5044: f64 = (8.8541878176e-12 * p.p209);
            let assign6440_e5046: f64 = (assign6440_e5044 * locals.var_wecv);
            let assign6440_e5048: f64 = (assign6440_e5046 * locals.var_lecv);
            let assign6440_e5050: f64 = (assign6440_e5048 / p.p208);
            locals.var_cox_p = assign6440_e5050;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6450_e5056: f64 = (8.8541878176e-12 * p.p209);
            let assign6450_e5058: f64 = (assign6450_e5056 * locals.var_wecv);
            let assign6450_e5060: f64 = (assign6450_e5058 * p.p236);
            let assign6450_e5062: f64 = (assign6450_e5060 / p.p234);
            locals.var_cgov_p = assign6450_e5062;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6460_e5068: f64 = (8.8541878176e-12 * p.p209);
            let assign6460_e5070: f64 = (assign6460_e5068 * locals.var_wecv);
            let assign6460_e5072: f64 = (assign6460_e5070 * p.p237);
            let assign6460_e5074: f64 = (assign6460_e5072 / p.p235);
            locals.var_cgovd_p = assign6460_e5074;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6470_e5082: f64 = (locals.var_ile).powf(p.p356);
            let assign6470_e5083: f64 = (p.p355 * assign6470_e5082);
            let assign6470_e5084: f64 = (p.p354 + assign6470_e5083);
            let assign6470_e5087: f64 = (p.p357 * locals.var_iwe);
            let assign6470_e5088: f64 = (assign6470_e5084 + assign6470_e5087);
            let assign6470_e5091: f64 = (p.p358 * locals.var_iae);
            let assign6470_e5092: f64 = (assign6470_e5088 + assign6470_e5091);
            locals.var_delvtac_p = assign6470_e5092;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6480_e5099: f64 = (p.p360 * locals.var_ile);
            let assign6480_e5100: f64 = (p.p359 + assign6480_e5099);
            let assign6480_e5103: f64 = (p.p361 * locals.var_iwe);
            let assign6480_e5104: f64 = (assign6480_e5100 + assign6480_e5103);
            let assign6480_e5107: f64 = (p.p362 * locals.var_iae);
            let assign6480_e5108: f64 = (assign6480_e5104 + assign6480_e5107);
            locals.var_facneffac_p = assign6480_e5108;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_thesataco_i = p.p296;
        }
        let assign6500_e5116: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6500_e5118: f64 = if assign6500_e5116 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign6500_e5118;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard48 != 0.0)) {
            locals.var_thesataco_i = p.p363;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_thesatacl_i = p.p297;
        }
        let assign6530_e5130: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6530_e5132: f64 = if assign6530_e5130 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign6530_e5132;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard49 != 0.0)) {
            locals.var_thesatacl_i = p.p364;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_thesataclexp_i = p.p298;
        }
        let assign6560_e5144: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6560_e5146: f64 = if assign6560_e5144 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign6560_e5146;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard50 != 0.0)) {
            locals.var_thesataclexp_i = p.p365;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_thesatacw_i = p.p299;
        }
        let assign6590_e5158: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6590_e5160: f64 = if assign6590_e5158 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign6590_e5160;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard51 != 0.0)) {
            locals.var_thesatacw_i = p.p366;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_thesataclw_i = p.p300;
        }
        let assign6620_e5172: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6620_e5174: f64 = if assign6620_e5172 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign6620_e5174;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard52 != 0.0)) {
            locals.var_thesataclw_i = p.p367;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6640_e5185: f64 = (locals.var_thesatacl_i * locals.var_gwe);
            let assign6640_e5187: f64 = (assign6640_e5185 / locals.var_gpe);
            let assign6640_e5190: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
            let assign6640_e5191: f64 = (assign6640_e5187 * assign6640_e5190);
            let assign6640_e5192: f64 = (locals.var_thesataco_i + assign6640_e5191);
            let assign6640_e5196: f64 = (locals.var_thesatacw_i * locals.var_iwe);
            let assign6640_e5197: f64 = (1.0 + assign6640_e5196);
            let assign6640_e5198: f64 = (assign6640_e5192 * assign6640_e5197);
            let assign6640_e5202: f64 = (locals.var_thesataclw_i * locals.var_iae);
            let assign6640_e5203: f64 = (1.0 + assign6640_e5202);
            let assign6640_e5204: f64 = (assign6640_e5198 * assign6640_e5203);
            locals.var_thesatac_p = assign6640_e5204;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_axaco_i = p.p308;
        }
        let assign6660_e5212: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6660_e5214: f64 = if assign6660_e5212 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign6660_e5214;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard53 != 0.0)) {
            locals.var_axaco_i = p.p368;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_axacl_i = p.p309;
        }
        let assign6690_e5226: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6690_e5228: f64 = if assign6690_e5226 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign6690_e5228;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard54 != 0.0)) {
            locals.var_axacl_i = p.p369;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6710_e5240: f64 = (locals.var_axacl_i * locals.var_ile);
            let assign6710_e5241: f64 = (1.0 + assign6710_e5240);
            let assign6710_e5242: f64 = (locals.var_axaco_i / assign6710_e5241);
            locals.var_axac_p = assign6710_e5242;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6720_e5249: f64 = (locals.var_ile).powf(p.p371);
            let assign6720_e5250: f64 = (p.p370 * assign6720_e5249);
            let assign6720_e5254: f64 = (p.p372 * locals.var_iwe);
            let assign6720_e5255: f64 = (1.0 + assign6720_e5254);
            let assign6720_e5256: f64 = (assign6720_e5250 * assign6720_e5255);
            locals.var_alpac_p = assign6720_e5256;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6730_e5262: f64 = (locals.var_ile).powf(p.p374);
            locals.var_tmpx = assign6730_e5262;
        }
    }
    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard41 != 0.0) {
            let assign6740_e5268: f64 = (p.p373 * locals.var_tmpx);
            let assign6740_e5272: f64 = (p.p376 * locals.var_iwe);
            let assign6740_e5273: f64 = (1.0 + assign6740_e5272);
            let assign6740_e5274: f64 = (assign6740_e5268 * assign6740_e5273);
            let assign6740_e5278: f64 = (p.p375 * locals.var_ile);
            let assign6740_e5280: f64 = (assign6740_e5278 * locals.var_tmpx);
            let assign6740_e5281: f64 = (1.0 + assign6740_e5280);
            let assign6740_e5282: f64 = (assign6740_e5274 / assign6740_e5281);
            locals.var_alp1ac_p = assign6740_e5282;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_fcgovacc_p = p.p377;
            locals.var_fcgovaccd_p = p.p378;
            locals.var_cgovaccg_p = p.p379;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6780_e5300: f64 = (p.p380 * locals.var_iilcv);
            locals.var_cgbov_p = assign6780_e5300;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6790_e5306: f64 = (p.p381 * locals.var_iiwecv);
            locals.var_cinr_p = assign6790_e5306;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6800_e5312: f64 = (p.p382 * locals.var_iiwecv);
            locals.var_cinrd_p = assign6800_e5312;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_dvfbinr_p = p.p383;
            locals.var_fcinrdep_p = p.p384;
            locals.var_fcinracc_p = p.p385;
            locals.var_axinr_p = p.p386;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6850_e5334: f64 = (p.p387 * locals.var_iiwcv);
            locals.var_cfr_p = assign6850_e5334;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6860_e5340: f64 = (p.p388 * locals.var_iiwcv);
            locals.var_cfrd_p = assign6860_e5340;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6870_e5347: f64 = (2.0 * p.p395);
            let assign6870_e5349: f64 = (assign6870_e5347 / locals.var_le);
            let assign6870_e5350: f64 = (1.0 - assign6870_e5349);
            locals.var_temp0 = assign6870_e5350;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_fnt_p = p.p389;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6910_e5377: f64 = (p.p390 * locals.var_betn_p);
            let assign6910_e5379: f64 = (assign6910_e5377 * locals.var_betn_p);
            let assign6910_e5381: f64 = (assign6910_e5379 * locals.var_iwe);
            let assign6910_e5383: f64 = (assign6910_e5381 * locals.var_iwe);
            locals.var_fntexc_p = assign6910_e5383;
        }
        if (locals.var_guard41 != 0.0) {
            let assign6960_e5417: f64 = (2.0 * p.p397);
            let assign6960_e5420: f64 = (p.p398 * locals.var_we);
            let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
            locals.var_we_edge = assign6960_e5421;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_vfbedge_p = p.p399;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7000_e5444: f64 = (p.p401 * locals.var_ile);
            let assign7000_e5445: f64 = (p.p400 + assign7000_e5444);
            let assign7000_e5448: f64 = (p.p402 * locals.var_iwe);
            let assign7000_e5449: f64 = (assign7000_e5445 + assign7000_e5448);
            let assign7000_e5452: f64 = (p.p403 * locals.var_iae);
            let assign7000_e5453: f64 = (assign7000_e5449 + assign7000_e5452);
            locals.var_stvfbedge_p = assign7000_e5453;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7010_e5461: f64 = (locals.var_ile).powf(p.p406);
            let assign7010_e5462: f64 = (p.p405 * assign7010_e5461);
            let assign7010_e5463: f64 = (p.p404 + assign7010_e5462);
            let assign7010_e5466: f64 = (p.p407 * locals.var_iwe);
            let assign7010_e5467: f64 = (assign7010_e5463 + assign7010_e5466);
            let assign7010_e5470: f64 = (p.p408 * locals.var_iae);
            let assign7010_e5471: f64 = (assign7010_e5467 + assign7010_e5470);
            locals.var_dphibedge_p = assign7010_e5471;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7020_e5480: f64 = (locals.var_ile).powf(p.p411);
            let assign7020_e5481: f64 = (p.p410 * assign7020_e5480);
            let assign7020_e5482: f64 = (1.0 + assign7020_e5481);
            let assign7020_e5483: f64 = (p.p409 * assign7020_e5482);
            let assign7020_e5487: f64 = (p.p412 * locals.var_iwe);
            let assign7020_e5488: f64 = (1.0 + assign7020_e5487);
            let assign7020_e5489: f64 = (assign7020_e5483 * assign7020_e5488);
            let assign7020_e5493: f64 = (p.p413 * locals.var_iae);
            let assign7020_e5494: f64 = (1.0 + assign7020_e5493);
            let assign7020_e5495: f64 = (assign7020_e5489 * assign7020_e5494);
            locals.var_neffedge_p = assign7020_e5495;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7030_e5503: f64 = (locals.var_ile).powf(p.p416);
            let assign7030_e5504: f64 = (p.p415 * assign7030_e5503);
            let assign7030_e5505: f64 = (p.p414 + assign7030_e5504);
            locals.var_ctedge_p = assign7030_e5505;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7040_e5512: f64 = (p.p417 * p.p418);
            let assign7040_e5514: f64 = (assign7040_e5512 / locals.var_le);
            let assign7040_e5517: f64 = (-locals.var_le);
            let assign7040_e5519: f64 = (assign7040_e5517 / p.p418);
            let assign7040_e5520: f64 = (assign7040_e5519).exp();
            let assign7040_e5521: f64 = (1.0 - assign7040_e5520);
            let assign7040_e5522: f64 = (assign7040_e5514 * assign7040_e5521);
            let assign7040_e5523: f64 = (1.0 + assign7040_e5522);
            locals.var_gpe_edge = assign7040_e5523;
        }
        if (locals.var_guard41 != 0.0) {
            let (assign7050_e5532,) = {
    if (locals.var_gpe_edge > 1e-15) {
        (locals.var_gpe_edge,)
    } else {
        (1e-15,)
    }
};
            locals.var_gpe_edge = assign7050_e5532;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7060_e5538: f64 = (p.p258 * locals.var_we_edge);
            let assign7060_e5541: f64 = (locals.var_gpe_edge * locals.var_le);
            let assign7060_e5542: f64 = (assign7060_e5538 / assign7060_e5541);
            let assign7060_e5546: f64 = (p.p419 * locals.var_iwe);
            let assign7060_e5547: f64 = (1.0 + assign7060_e5546);
            let assign7060_e5548: f64 = (assign7060_e5542 * assign7060_e5547);
            locals.var_betnedge_p = assign7060_e5548;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7070_e5555: f64 = (p.p421 * locals.var_ile);
            let assign7070_e5556: f64 = (p.p420 + assign7070_e5555);
            let assign7070_e5559: f64 = (p.p422 * locals.var_iwe);
            let assign7070_e5560: f64 = (assign7070_e5556 + assign7070_e5559);
            let assign7070_e5563: f64 = (p.p423 * locals.var_iae);
            let assign7070_e5564: f64 = (assign7070_e5560 + assign7070_e5563);
            locals.var_stbetedge_p = assign7070_e5564;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7080_e5571: f64 = (locals.var_ile).powf(p.p425);
            let assign7080_e5572: f64 = (p.p424 * assign7080_e5571);
            let assign7080_e5576: f64 = (p.p426 * locals.var_iwe);
            let assign7080_e5577: f64 = (1.0 + assign7080_e5576);
            let assign7080_e5578: f64 = (assign7080_e5572 * assign7080_e5577);
            locals.var_psceedge_p = assign7080_e5578;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_pscebedge_p = p.p427;
            locals.var_pscededge_p = p.p428;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7110_e5593: f64 = (locals.var_ile).powf(p.p430);
            let assign7110_e5594: f64 = (p.p429 * assign7110_e5593);
            let assign7110_e5598: f64 = (p.p431 * locals.var_iwe);
            let assign7110_e5599: f64 = (1.0 + assign7110_e5598);
            let assign7110_e5600: f64 = (assign7110_e5594 * assign7110_e5599);
            locals.var_cfedge_p = assign7110_e5600;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_cfdedge_p = p.p433;
            locals.var_cfbedge_p = p.p432;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7190_e5641: f64 = (p.p815 * locals.var_ile);
            let assign7190_e5642: f64 = (p.p814 + assign7190_e5641);
            let assign7190_e5645: f64 = (p.p816 * locals.var_iwe);
            let assign7190_e5646: f64 = (assign7190_e5642 + assign7190_e5645);
            let assign7190_e5649: f64 = (p.p817 * locals.var_iae);
            let assign7190_e5650: f64 = (assign7190_e5646 + assign7190_e5649);
            locals.var_kvthowe = assign7190_e5650;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7200_e5657: f64 = (p.p819 * locals.var_ile);
            let assign7200_e5658: f64 = (p.p818 + assign7200_e5657);
            let assign7200_e5661: f64 = (p.p820 * locals.var_iwe);
            let assign7200_e5662: f64 = (assign7200_e5658 + assign7200_e5661);
            let assign7200_e5665: f64 = (p.p821 * locals.var_iae);
            let assign7200_e5666: f64 = (assign7200_e5662 + assign7200_e5665);
            locals.var_kuowe = assign7200_e5666;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7210_e5673: f64 = (0.3333333333333333 * locals.var_w_f);
            let assign7210_e5675: f64 = (assign7210_e5673 / locals.var_ngcon_i);
            let assign7210_e5677: f64 = (assign7210_e5675 + locals.var_xgwe);
            let assign7210_e5678: f64 = (p.p442 * assign7210_e5677);
            let assign7210_e5681: f64 = (locals.var_ngcon_i * locals.var_l_slif);
            let assign7210_e5682: f64 = (assign7210_e5678 / assign7210_e5681);
            let assign7210_e5685: f64 = (p.p440 + p.p441);
            let assign7210_e5688: f64 = (locals.var_w_f * locals.var_l_f);
            let assign7210_e5689: f64 = (assign7210_e5685 / assign7210_e5688);
            let assign7210_e5690: f64 = (assign7210_e5682 + assign7210_e5689);
            let assign7210_e5693: f64 = (locals.var_nf_i * p.p439);
            let assign7210_e5694: f64 = (assign7210_e5690 + assign7210_e5693);
            locals.var_rg_p = assign7210_e5694;
        }
        if (locals.var_guard41 != 0.0) {
            let (assign7220_e5703,) = {
    if (p.p444 > 0.0) {
        (p.p444,)
    } else {
        (0.0,)
    }
};
            locals.var_rsh_i = assign7220_e5703;
        }
        if (locals.var_guard41 != 0.0) {
            let (assign7230_e5712,) = {
    if (p.p445 > 0.0) {
        (p.p445,)
    } else {
        (0.0,)
    }
};
            locals.var_rshd_i = assign7230_e5712;
        }
        let assign7240_e5717: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign7240_e5717;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard55 != 0.0)) {
            locals.var_rshd_i = locals.var_rsh_i;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7260_e5727: f64 = (locals.var_nf_i * p.p12);
            let assign7260_e5729: f64 = (assign7260_e5727 * locals.var_rsh_i);
            locals.var_rse_p = assign7260_e5729;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7270_e5735: f64 = (locals.var_nf_i * p.p13);
            let assign7270_e5737: f64 = (assign7270_e5735 * locals.var_rshd_i);
            locals.var_rde_p = assign7270_e5737;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7280_e5743: f64 = (locals.var_nf_i * p.p447);
            locals.var_rwell_p = assign7280_e5743;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7290_e5749: f64 = (locals.var_nf_i * p.p446);
            locals.var_rbulk_p = assign7290_e5749;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7300_e5755: f64 = (locals.var_nf_i * p.p448);
            locals.var_rjuns_p = assign7300_e5755;
        }
        if (locals.var_guard41 != 0.0) {
            let assign7310_e5761: f64 = (locals.var_nf_i * p.p449);
            locals.var_rjund_p = assign7310_e5761;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_munqs_p = p.p450;
        }
        let assign7330_e5786: f64 = if (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]) { 1.0 } else { 0.0 };
        locals.var_guard56 = assign7330_e5786;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard56 != 0.0)) {
            let assign7340_e5793: f64 = (p.p452 * locals.var_ile);
            let assign7340_e5794: f64 = (p.p451 + assign7340_e5793);
            let assign7340_e5797: f64 = (p.p453 * locals.var_iwe);
            let assign7340_e5798: f64 = (assign7340_e5794 + assign7340_e5797);
            let assign7340_e5801: f64 = (p.p454 * locals.var_iae);
            let assign7340_e5802: f64 = (assign7340_e5798 + assign7340_e5801);
            locals.var_vfb_p = assign7340_e5802;
        }
        let assign7350_e5823: f64 = if (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign7350_e5823;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard57 != 0.0)) {
            let assign7360_e5830: f64 = (p.p456 * locals.var_ile);
            let assign7360_e5831: f64 = (p.p455 + assign7360_e5830);
            let assign7360_e5834: f64 = (p.p457 * locals.var_iwe);
            let assign7360_e5835: f64 = (assign7360_e5831 + assign7360_e5834);
            let assign7360_e5838: f64 = (p.p458 * locals.var_iae);
            let assign7360_e5839: f64 = (assign7360_e5835 + assign7360_e5838);
            locals.var_stvfb_p = assign7360_e5839;
        }
        let assign7370_e5860: f64 = if (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign7370_e5860;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard58 != 0.0)) {
            let assign7380_e5867: f64 = (p.p460 * locals.var_ile);
            let assign7380_e5868: f64 = (p.p459 + assign7380_e5867);
            let assign7380_e5871: f64 = (p.p461 * locals.var_iwe);
            let assign7380_e5872: f64 = (assign7380_e5868 + assign7380_e5871);
            let assign7380_e5875: f64 = (p.p462 * locals.var_iae);
            let assign7380_e5876: f64 = (assign7380_e5872 + assign7380_e5875);
            locals.var_neff_p = assign7380_e5876;
        }
        let assign7390_e5897: f64 = if (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign7390_e5897;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard59 != 0.0)) {
            let assign7400_e5904: f64 = (p.p464 * locals.var_ile);
            let assign7400_e5905: f64 = (p.p463 + assign7400_e5904);
            let assign7400_e5908: f64 = (p.p465 * locals.var_iwe);
            let assign7400_e5909: f64 = (assign7400_e5905 + assign7400_e5908);
            let assign7400_e5912: f64 = (p.p466 * locals.var_iae);
            let assign7400_e5913: f64 = (assign7400_e5909 + assign7400_e5912);
            locals.var_gfacnud_p = assign7400_e5913;
        }
        let assign7410_e5934: f64 = if (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]) { 1.0 } else { 0.0 };
        locals.var_guard60 = assign7410_e5934;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard60 != 0.0)) {
            let assign7420_e5941: f64 = (p.p468 * locals.var_ile);
            let assign7420_e5942: f64 = (p.p467 + assign7420_e5941);
            let assign7420_e5945: f64 = (p.p469 * locals.var_iwe);
            let assign7420_e5946: f64 = (assign7420_e5942 + assign7420_e5945);
            let assign7420_e5949: f64 = (p.p470 * locals.var_iae);
            let assign7420_e5950: f64 = (assign7420_e5946 + assign7420_e5949);
            locals.var_vsbnud_p = assign7420_e5950;
        }
        let assign7430_e5971: f64 = if (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]) { 1.0 } else { 0.0 };
        locals.var_guard61 = assign7430_e5971;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard61 != 0.0)) {
            let assign7440_e5978: f64 = (p.p472 * locals.var_ile);
            let assign7440_e5979: f64 = (p.p471 + assign7440_e5978);
            let assign7440_e5982: f64 = (p.p473 * locals.var_iwe);
            let assign7440_e5983: f64 = (assign7440_e5979 + assign7440_e5982);
            let assign7440_e5986: f64 = (p.p474 * locals.var_iae);
            let assign7440_e5987: f64 = (assign7440_e5983 + assign7440_e5986);
            locals.var_dphib_p = assign7440_e5987;
        }
        let assign7450_e6008: f64 = if (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]) { 1.0 } else { 0.0 };
        locals.var_guard62 = assign7450_e6008;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard62 != 0.0)) {
            let assign7460_e6015: f64 = (p.p476 * locals.var_ile);
            let assign7460_e6016: f64 = (p.p475 + assign7460_e6015);
            let assign7460_e6019: f64 = (p.p477 * locals.var_iwe);
            let assign7460_e6020: f64 = (assign7460_e6016 + assign7460_e6019);
            let assign7460_e6023: f64 = (p.p478 * locals.var_iae);
            let assign7460_e6024: f64 = (assign7460_e6020 + assign7460_e6023);
            locals.var_np_p = assign7460_e6024;
        }
        let assign7470_e6045: f64 = if (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]) { 1.0 } else { 0.0 };
        locals.var_guard63 = assign7470_e6045;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard63 != 0.0)) {
            let assign7480_e6052: f64 = (p.p480 * locals.var_ile);
            let assign7480_e6053: f64 = (p.p479 + assign7480_e6052);
            let assign7480_e6056: f64 = (p.p481 * locals.var_iwe);
            let assign7480_e6057: f64 = (assign7480_e6053 + assign7480_e6056);
            let assign7480_e6060: f64 = (p.p482 * locals.var_iae);
            let assign7480_e6061: f64 = (assign7480_e6057 + assign7480_e6060);
            locals.var_nov_p = assign7480_e6061;
        }
        let assign7490_e6082: f64 = if (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]) { 1.0 } else { 0.0 };
        locals.var_guard64 = assign7490_e6082;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard64 != 0.0)) {
            let assign7500_e6089: f64 = (p.p484 * locals.var_ile);
            let assign7500_e6090: f64 = (p.p483 + assign7500_e6089);
            let assign7500_e6093: f64 = (p.p485 * locals.var_iwe);
            let assign7500_e6094: f64 = (assign7500_e6090 + assign7500_e6093);
            let assign7500_e6097: f64 = (p.p486 * locals.var_iae);
            let assign7500_e6098: f64 = (assign7500_e6094 + assign7500_e6097);
            locals.var_novd_p = assign7500_e6098;
        }
        let assign7510_e6119: f64 = if (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]) { 1.0 } else { 0.0 };
        locals.var_guard65 = assign7510_e6119;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard65 != 0.0)) {
            let assign7520_e6126: f64 = (p.p488 * locals.var_ile);
            let assign7520_e6127: f64 = (p.p487 + assign7520_e6126);
            let assign7520_e6130: f64 = (p.p489 * locals.var_iwe);
            let assign7520_e6131: f64 = (assign7520_e6127 + assign7520_e6130);
            let assign7520_e6134: f64 = (p.p490 * locals.var_iae);
            let assign7520_e6135: f64 = (assign7520_e6131 + assign7520_e6134);
            locals.var_ct_p = assign7520_e6135;
        }
        let assign7530_e6156: f64 = if (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]) { 1.0 } else { 0.0 };
        locals.var_guard66 = assign7530_e6156;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard66 != 0.0)) {
            let assign7540_e6163: f64 = (p.p496 * locals.var_ile);
            let assign7540_e6164: f64 = (p.p495 + assign7540_e6163);
            let assign7540_e6167: f64 = (p.p497 * locals.var_iwe);
            let assign7540_e6168: f64 = (assign7540_e6164 + assign7540_e6167);
            let assign7540_e6171: f64 = (p.p498 * locals.var_iae);
            let assign7540_e6172: f64 = (assign7540_e6168 + assign7540_e6171);
            locals.var_ctg_p = assign7540_e6172;
        }
        let assign7550_e6193: f64 = if (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7550_e6193;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard67 != 0.0)) {
            let assign7560_e6200: f64 = (p.p492 * locals.var_ile);
            let assign7560_e6201: f64 = (p.p491 + assign7560_e6200);
            let assign7560_e6204: f64 = (p.p493 * locals.var_iwe);
            let assign7560_e6205: f64 = (assign7560_e6201 + assign7560_e6204);
            let assign7560_e6208: f64 = (p.p494 * locals.var_iae);
            let assign7560_e6209: f64 = (assign7560_e6205 + assign7560_e6208);
            locals.var_ctb_p = assign7560_e6209;
        }
        let assign7570_e6230: f64 = if (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7570_e6230;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard68 != 0.0)) {
            let assign7580_e6237: f64 = (p.p500 * locals.var_ile);
            let assign7580_e6238: f64 = (p.p499 + assign7580_e6237);
            let assign7580_e6241: f64 = (p.p501 * locals.var_iwe);
            let assign7580_e6242: f64 = (assign7580_e6238 + assign7580_e6241);
            let assign7580_e6245: f64 = (p.p502 * locals.var_iae);
            let assign7580_e6246: f64 = (assign7580_e6242 + assign7580_e6245);
            locals.var_stct_p = assign7580_e6246;
        }
        let assign7590_e6267: f64 = if (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7590_e6267;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard69 != 0.0)) {
            let assign7600_e6275: f64 = (p.p504 * locals.var_ile);
            let assign7600_e6276: f64 = (p.p503 + assign7600_e6275);
            let assign7600_e6279: f64 = (p.p505 * locals.var_iwe);
            let assign7600_e6280: f64 = (assign7600_e6276 + assign7600_e6279);
            let assign7600_e6283: f64 = (p.p506 * locals.var_iae);
            let assign7600_e6284: f64 = (assign7600_e6280 + assign7600_e6283);
            let assign7600_e6285: f64 = (locals.var_ile2 * assign7600_e6284);
            locals.var_cf_p = assign7600_e6285;
        }
        let assign7610_e6306: f64 = if (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7610_e6306;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard70 != 0.0)) {
            let assign7620_e6313: f64 = (p.p512 * locals.var_ile);
            let assign7620_e6314: f64 = (p.p511 + assign7620_e6313);
            let assign7620_e6317: f64 = (p.p513 * locals.var_iwe);
            let assign7620_e6318: f64 = (assign7620_e6314 + assign7620_e6317);
            let assign7620_e6321: f64 = (p.p514 * locals.var_iae);
            let assign7620_e6322: f64 = (assign7620_e6318 + assign7620_e6321);
            locals.var_cfd_p = assign7620_e6322;
        }
    }
    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign7630_e6343: f64 = if (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7630_e6343;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard71 != 0.0)) {
            let assign7640_e6350: f64 = (p.p508 * locals.var_ile);
            let assign7640_e6351: f64 = (p.p507 + assign7640_e6350);
            let assign7640_e6354: f64 = (p.p509 * locals.var_iwe);
            let assign7640_e6355: f64 = (assign7640_e6351 + assign7640_e6354);
            let assign7640_e6358: f64 = (p.p510 * locals.var_iae);
            let assign7640_e6359: f64 = (assign7640_e6355 + assign7640_e6358);
            locals.var_cfb_p = assign7640_e6359;
        }
        let assign7650_e6380: f64 = if (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7650_e6380;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard72 != 0.0)) {
            let assign7660_e6388: f64 = (p.p516 * locals.var_ile);
            let assign7660_e6389: f64 = (p.p515 + assign7660_e6388);
            let assign7660_e6392: f64 = (p.p517 * locals.var_iwe);
            let assign7660_e6393: f64 = (assign7660_e6389 + assign7660_e6392);
            let assign7660_e6396: f64 = (p.p518 * locals.var_iae);
            let assign7660_e6397: f64 = (assign7660_e6393 + assign7660_e6396);
            let assign7660_e6398: f64 = (locals.var_ile2 * assign7660_e6397);
            locals.var_psce_p = assign7660_e6398;
        }
        let assign7670_e6419: f64 = if (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7670_e6419;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard73 != 0.0)) {
            let assign7680_e6426: f64 = (p.p524 * locals.var_ile);
            let assign7680_e6427: f64 = (p.p523 + assign7680_e6426);
            let assign7680_e6430: f64 = (p.p525 * locals.var_iwe);
            let assign7680_e6431: f64 = (assign7680_e6427 + assign7680_e6430);
            let assign7680_e6434: f64 = (p.p526 * locals.var_iae);
            let assign7680_e6435: f64 = (assign7680_e6431 + assign7680_e6434);
            locals.var_psced_p = assign7680_e6435;
        }
        let assign7690_e6456: f64 = if (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7690_e6456;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard74 != 0.0)) {
            let assign7700_e6463: f64 = (p.p520 * locals.var_ile);
            let assign7700_e6464: f64 = (p.p519 + assign7700_e6463);
            let assign7700_e6467: f64 = (p.p521 * locals.var_iwe);
            let assign7700_e6468: f64 = (assign7700_e6464 + assign7700_e6467);
            let assign7700_e6471: f64 = (p.p522 * locals.var_iae);
            let assign7700_e6472: f64 = (assign7700_e6468 + assign7700_e6471);
            locals.var_psceb_p = assign7700_e6472;
        }
        let assign7710_e6493: f64 = if (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7710_e6493;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard75 != 0.0)) {
            let assign7720_e6499: f64 = (locals.var_we / locals.var_le);
            let assign7720_e6503: f64 = (p.p528 * locals.var_ile);
            let assign7720_e6504: f64 = (p.p527 + assign7720_e6503);
            let assign7720_e6507: f64 = (p.p529 * locals.var_iwe);
            let assign7720_e6508: f64 = (assign7720_e6504 + assign7720_e6507);
            let assign7720_e6511: f64 = (p.p530 * locals.var_iae);
            let assign7720_e6512: f64 = (assign7720_e6508 + assign7720_e6511);
            let assign7720_e6513: f64 = (assign7720_e6499 * assign7720_e6512);
            locals.var_betn_p = assign7720_e6513;
        }
        let assign7730_e6534: f64 = if (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7730_e6534;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard76 != 0.0)) {
            let assign7740_e6541: f64 = (p.p532 * locals.var_ile);
            let assign7740_e6542: f64 = (p.p531 + assign7740_e6541);
            let assign7740_e6545: f64 = (p.p533 * locals.var_iwe);
            let assign7740_e6546: f64 = (assign7740_e6542 + assign7740_e6545);
            let assign7740_e6549: f64 = (p.p534 * locals.var_iae);
            let assign7740_e6550: f64 = (assign7740_e6546 + assign7740_e6549);
            locals.var_stbet_p = assign7740_e6550;
        }
        let assign7750_e6571: f64 = if (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign7750_e6571;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard77 != 0.0)) {
            let assign7760_e6578: f64 = (p.p536 * locals.var_ile);
            let assign7760_e6579: f64 = (p.p535 + assign7760_e6578);
            let assign7760_e6582: f64 = (p.p537 * locals.var_iwe);
            let assign7760_e6583: f64 = (assign7760_e6579 + assign7760_e6582);
            let assign7760_e6586: f64 = (p.p538 * locals.var_iae);
            let assign7760_e6587: f64 = (assign7760_e6583 + assign7760_e6586);
            locals.var_mue_p = assign7760_e6587;
        }
        let assign7770_e6608: f64 = if (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign7770_e6608;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard78 != 0.0)) {
            let assign7780_e6615: f64 = (p.p540 * locals.var_ile);
            let assign7780_e6616: f64 = (p.p539 + assign7780_e6615);
            let assign7780_e6619: f64 = (p.p541 * locals.var_iwe);
            let assign7780_e6620: f64 = (assign7780_e6616 + assign7780_e6619);
            let assign7780_e6623: f64 = (p.p542 * locals.var_iae);
            let assign7780_e6624: f64 = (assign7780_e6620 + assign7780_e6623);
            locals.var_themu_p = assign7780_e6624;
        }
        let assign7790_e6645: f64 = if (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign7790_e6645;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard79 != 0.0)) {
            let assign7800_e6652: f64 = (p.p544 * locals.var_ile);
            let assign7800_e6653: f64 = (p.p543 + assign7800_e6652);
            let assign7800_e6656: f64 = (p.p545 * locals.var_iwe);
            let assign7800_e6657: f64 = (assign7800_e6653 + assign7800_e6656);
            let assign7800_e6660: f64 = (p.p546 * locals.var_iae);
            let assign7800_e6661: f64 = (assign7800_e6657 + assign7800_e6660);
            locals.var_cs_p = assign7800_e6661;
        }
        let assign7810_e6682: f64 = if (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]) { 1.0 } else { 0.0 };
        locals.var_guard80 = assign7810_e6682;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard80 != 0.0)) {
            let assign7820_e6689: f64 = (p.p548 * locals.var_ile);
            let assign7820_e6690: f64 = (p.p547 + assign7820_e6689);
            let assign7820_e6693: f64 = (p.p549 * locals.var_iwe);
            let assign7820_e6694: f64 = (assign7820_e6690 + assign7820_e6693);
            let assign7820_e6697: f64 = (p.p550 * locals.var_iae);
            let assign7820_e6698: f64 = (assign7820_e6694 + assign7820_e6697);
            locals.var_thecs_p = assign7820_e6698;
        }
        let assign7830_e6719: f64 = if (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]) { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7830_e6719;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard81 != 0.0)) {
            let assign7840_e6726: f64 = (p.p552 * locals.var_ile);
            let assign7840_e6727: f64 = (p.p551 + assign7840_e6726);
            let assign7840_e6730: f64 = (p.p553 * locals.var_iwe);
            let assign7840_e6731: f64 = (assign7840_e6727 + assign7840_e6730);
            let assign7840_e6734: f64 = (p.p554 * locals.var_iae);
            let assign7840_e6735: f64 = (assign7840_e6731 + assign7840_e6734);
            locals.var_xcor_p = assign7840_e6735;
        }
        let assign7850_e6756: f64 = if (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7850_e6756;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard82 != 0.0)) {
            let assign7860_e6764: f64 = (p.p556 * locals.var_ile);
            let assign7860_e6765: f64 = (p.p555 + assign7860_e6764);
            let assign7860_e6768: f64 = (p.p557 * locals.var_iwe);
            let assign7860_e6769: f64 = (assign7860_e6765 + assign7860_e6768);
            let assign7860_e6772: f64 = (p.p558 * locals.var_iae);
            let assign7860_e6773: f64 = (assign7860_e6769 + assign7860_e6772);
            let assign7860_e6774: f64 = (locals.var_iwe * assign7860_e6773);
            locals.var_rs_p = assign7860_e6774;
        }
        let assign7870_e6795: f64 = if (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7870_e6795;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard83 != 0.0)) {
            let assign7880_e6802: f64 = (p.p560 * locals.var_ile);
            let assign7880_e6803: f64 = (p.p559 + assign7880_e6802);
            let assign7880_e6806: f64 = (p.p561 * locals.var_iwe);
            let assign7880_e6807: f64 = (assign7880_e6803 + assign7880_e6806);
            let assign7880_e6810: f64 = (p.p562 * locals.var_iae);
            let assign7880_e6811: f64 = (assign7880_e6807 + assign7880_e6810);
            locals.var_strs_p = assign7880_e6811;
        }
        let assign7890_e6832: f64 = if (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]) { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7890_e6832;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard84 != 0.0)) {
            let assign7900_e6839: f64 = (p.p564 * locals.var_ile);
            let assign7900_e6840: f64 = (p.p563 + assign7900_e6839);
            let assign7900_e6843: f64 = (p.p565 * locals.var_iwe);
            let assign7900_e6844: f64 = (assign7900_e6840 + assign7900_e6843);
            let assign7900_e6847: f64 = (p.p566 * locals.var_iae);
            let assign7900_e6848: f64 = (assign7900_e6844 + assign7900_e6847);
            locals.var_rsb_p = assign7900_e6848;
        }
        let assign7910_e6869: f64 = if (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]) { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7910_e6869;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard85 != 0.0)) {
            let assign7920_e6876: f64 = (p.p568 * locals.var_ile);
            let assign7920_e6877: f64 = (p.p567 + assign7920_e6876);
            let assign7920_e6880: f64 = (p.p569 * locals.var_iwe);
            let assign7920_e6881: f64 = (assign7920_e6877 + assign7920_e6880);
            let assign7920_e6884: f64 = (p.p570 * locals.var_iae);
            let assign7920_e6885: f64 = (assign7920_e6881 + assign7920_e6884);
            locals.var_rsg_p = assign7920_e6885;
        }
        let assign7930_e6906: f64 = if (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7930_e6906;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard86 != 0.0)) {
            let assign7940_e6914: f64 = (p.p572 * locals.var_ile);
            let assign7940_e6915: f64 = (p.p571 + assign7940_e6914);
            let assign7940_e6918: f64 = (p.p573 * locals.var_iwe);
            let assign7940_e6919: f64 = (assign7940_e6915 + assign7940_e6918);
            let assign7940_e6922: f64 = (p.p574 * locals.var_iae);
            let assign7940_e6923: f64 = (assign7940_e6919 + assign7940_e6922);
            let assign7940_e6924: f64 = (locals.var_ile * assign7940_e6923);
            locals.var_thesat_p = assign7940_e6924;
        }
        let assign7950_e6945: f64 = if (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]) { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7950_e6945;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard87 != 0.0)) {
            let assign7960_e6952: f64 = (p.p576 * locals.var_ile);
            let assign7960_e6953: f64 = (p.p575 + assign7960_e6952);
            let assign7960_e6956: f64 = (p.p577 * locals.var_iwe);
            let assign7960_e6957: f64 = (assign7960_e6953 + assign7960_e6956);
            let assign7960_e6960: f64 = (p.p578 * locals.var_iae);
            let assign7960_e6961: f64 = (assign7960_e6957 + assign7960_e6960);
            locals.var_stthesat_p = assign7960_e6961;
        }
        let assign7970_e6982: f64 = if (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]) { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7970_e6982;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard88 != 0.0)) {
            let assign7980_e6989: f64 = (p.p580 * locals.var_ile);
            let assign7980_e6990: f64 = (p.p579 + assign7980_e6989);
            let assign7980_e6993: f64 = (p.p581 * locals.var_iwe);
            let assign7980_e6994: f64 = (assign7980_e6990 + assign7980_e6993);
            let assign7980_e6997: f64 = (p.p582 * locals.var_iae);
            let assign7980_e6998: f64 = (assign7980_e6994 + assign7980_e6997);
            locals.var_thesatb_p = assign7980_e6998;
        }
        let assign7990_e7019: f64 = if (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7990_e7019;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard89 != 0.0)) {
            let assign8000_e7026: f64 = (p.p584 * locals.var_ile);
            let assign8000_e7027: f64 = (p.p583 + assign8000_e7026);
            let assign8000_e7030: f64 = (p.p585 * locals.var_iwe);
            let assign8000_e7031: f64 = (assign8000_e7027 + assign8000_e7030);
            let assign8000_e7034: f64 = (p.p586 * locals.var_iae);
            let assign8000_e7035: f64 = (assign8000_e7031 + assign8000_e7034);
            locals.var_thesatg_p = assign8000_e7035;
        }
        let assign8010_e7056: f64 = if (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        locals.var_guard90 = assign8010_e7056;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard90 != 0.0)) {
            let assign8020_e7063: f64 = (p.p588 * locals.var_ile);
            let assign8020_e7064: f64 = (p.p587 + assign8020_e7063);
            let assign8020_e7067: f64 = (p.p589 * locals.var_iwe);
            let assign8020_e7068: f64 = (assign8020_e7064 + assign8020_e7067);
            let assign8020_e7071: f64 = (p.p590 * locals.var_iae);
            let assign8020_e7072: f64 = (assign8020_e7068 + assign8020_e7071);
            locals.var_ax_p = assign8020_e7072;
        }
        let assign8030_e7093: f64 = if (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]) { 1.0 } else { 0.0 };
        locals.var_guard91 = assign8030_e7093;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard91 != 0.0)) {
            let assign8040_e7101: f64 = (p.p592 * locals.var_ile);
            let assign8040_e7102: f64 = (p.p591 + assign8040_e7101);
            let assign8040_e7105: f64 = (p.p593 * locals.var_iwe);
            let assign8040_e7106: f64 = (assign8040_e7102 + assign8040_e7105);
            let assign8040_e7109: f64 = (p.p594 * locals.var_iae);
            let assign8040_e7110: f64 = (assign8040_e7106 + assign8040_e7109);
            let assign8040_e7111: f64 = (locals.var_ile * assign8040_e7110);
            locals.var_alp_p = assign8040_e7111;
        }
        let assign8050_e7132: f64 = if (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign8050_e7132;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard92 != 0.0)) {
            let assign8060_e7139: f64 = (p.p596 * locals.var_ile);
            let assign8060_e7140: f64 = (p.p595 + assign8060_e7139);
            let assign8060_e7143: f64 = (p.p597 * locals.var_iwe);
            let assign8060_e7144: f64 = (assign8060_e7140 + assign8060_e7143);
            let assign8060_e7147: f64 = (p.p598 * locals.var_iae);
            let assign8060_e7148: f64 = (assign8060_e7144 + assign8060_e7147);
            locals.var_alp1_p = assign8060_e7148;
        }
        let assign8070_e7169: f64 = if (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]) { 1.0 } else { 0.0 };
        locals.var_guard93 = assign8070_e7169;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard93 != 0.0)) {
            let assign8080_e7176: f64 = (p.p600 * locals.var_ile);
            let assign8080_e7177: f64 = (p.p599 + assign8080_e7176);
            let assign8080_e7180: f64 = (p.p601 * locals.var_iwe);
            let assign8080_e7181: f64 = (assign8080_e7177 + assign8080_e7180);
            let assign8080_e7184: f64 = (p.p602 * locals.var_iae);
            let assign8080_e7185: f64 = (assign8080_e7181 + assign8080_e7184);
            locals.var_alp2_p = assign8080_e7185;
        }
        let assign8090_e7206: f64 = if (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]) { 1.0 } else { 0.0 };
        locals.var_guard94 = assign8090_e7206;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard94 != 0.0)) {
            let assign8100_e7213: f64 = (p.p604 * locals.var_ile);
            let assign8100_e7214: f64 = (p.p603 + assign8100_e7213);
            let assign8100_e7217: f64 = (p.p605 * locals.var_iwe);
            let assign8100_e7218: f64 = (assign8100_e7214 + assign8100_e7217);
            let assign8100_e7221: f64 = (p.p606 * locals.var_iae);
            let assign8100_e7222: f64 = (assign8100_e7218 + assign8100_e7221);
            locals.var_a1_p = assign8100_e7222;
        }
        let assign8110_e7243: f64 = if (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]) { 1.0 } else { 0.0 };
        locals.var_guard95 = assign8110_e7243;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard95 != 0.0)) {
            let assign8120_e7250: f64 = (p.p608 * locals.var_ile);
            let assign8120_e7251: f64 = (p.p607 + assign8120_e7250);
            let assign8120_e7254: f64 = (p.p609 * locals.var_iwe);
            let assign8120_e7255: f64 = (assign8120_e7251 + assign8120_e7254);
            let assign8120_e7258: f64 = (p.p610 * locals.var_iae);
            let assign8120_e7259: f64 = (assign8120_e7255 + assign8120_e7258);
            locals.var_sta2_p = assign8120_e7259;
        }
        let assign8130_e7280: f64 = if (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]) { 1.0 } else { 0.0 };
        locals.var_guard96 = assign8130_e7280;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard96 != 0.0)) {
            let assign8140_e7287: f64 = (p.p612 * locals.var_ile);
            let assign8140_e7288: f64 = (p.p611 + assign8140_e7287);
            let assign8140_e7291: f64 = (p.p613 * locals.var_iwe);
            let assign8140_e7292: f64 = (assign8140_e7288 + assign8140_e7291);
            let assign8140_e7295: f64 = (p.p614 * locals.var_iae);
            let assign8140_e7296: f64 = (assign8140_e7292 + assign8140_e7295);
            locals.var_a3_p = assign8140_e7296;
        }
        let assign8150_e7317: f64 = if (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]) { 1.0 } else { 0.0 };
        locals.var_guard97 = assign8150_e7317;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard97 != 0.0)) {
            let assign8160_e7324: f64 = (p.p616 * locals.var_ile);
            let assign8160_e7325: f64 = (p.p615 + assign8160_e7324);
            let assign8160_e7328: f64 = (p.p617 * locals.var_iwe);
            let assign8160_e7329: f64 = (assign8160_e7325 + assign8160_e7328);
            let assign8160_e7332: f64 = (p.p618 * locals.var_iae);
            let assign8160_e7333: f64 = (assign8160_e7329 + assign8160_e7332);
            locals.var_a4_p = assign8160_e7333;
        }
        let assign8170_e7354: f64 = if (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8170_e7354;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard98 != 0.0)) {
            let assign8180_e7362: f64 = (p.p620 * locals.var_ile);
            let assign8180_e7363: f64 = (p.p619 + assign8180_e7362);
            let assign8180_e7366: f64 = (p.p621 * locals.var_iwe);
            let assign8180_e7367: f64 = (assign8180_e7363 + assign8180_e7366);
            let assign8180_e7370: f64 = (p.p622 * locals.var_iae);
            let assign8180_e7371: f64 = (assign8180_e7367 + assign8180_e7370);
            let assign8180_e7372: f64 = (locals.var_iiae * assign8180_e7371);
            locals.var_iginv_p = assign8180_e7372;
        }
        let assign8190_e7393: f64 = if (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8190_e7393;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard99 != 0.0)) {
            let assign8200_e7401: f64 = (p.p624 * locals.var_ile);
            let assign8200_e7402: f64 = (p.p623 + assign8200_e7401);
            let assign8200_e7405: f64 = (p.p625 * locals.var_iwe);
            let assign8200_e7406: f64 = (assign8200_e7402 + assign8200_e7405);
            let assign8200_e7409: f64 = (p.p626 * locals.var_iae);
            let assign8200_e7410: f64 = (assign8200_e7406 + assign8200_e7409);
            let assign8200_e7411: f64 = (locals.var_iiwe * assign8200_e7410);
            locals.var_igov_p = assign8200_e7411;
        }
        let assign8210_e7432: f64 = if (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8210_e7432;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard100 != 0.0)) {
            let assign8220_e7440: f64 = (p.p628 * locals.var_ile);
            let assign8220_e7441: f64 = (p.p627 + assign8220_e7440);
            let assign8220_e7444: f64 = (p.p629 * locals.var_iwe);
            let assign8220_e7445: f64 = (assign8220_e7441 + assign8220_e7444);
            let assign8220_e7448: f64 = (p.p630 * locals.var_iae);
            let assign8220_e7449: f64 = (assign8220_e7445 + assign8220_e7448);
            let assign8220_e7450: f64 = (locals.var_iiwe * assign8220_e7449);
            locals.var_igovd_p = assign8220_e7450;
        }
        let assign8230_e7471: f64 = if (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]) { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8230_e7471;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard101 != 0.0)) {
            let assign8240_e7478: f64 = (p.p632 * locals.var_ile);
            let assign8240_e7479: f64 = (p.p631 + assign8240_e7478);
            let assign8240_e7482: f64 = (p.p633 * locals.var_iwe);
            let assign8240_e7483: f64 = (assign8240_e7479 + assign8240_e7482);
            let assign8240_e7486: f64 = (p.p634 * locals.var_iae);
            let assign8240_e7487: f64 = (assign8240_e7483 + assign8240_e7486);
            locals.var_stig_p = assign8240_e7487;
        }
        let assign8250_e7508: f64 = if (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8250_e7508;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard102 != 0.0)) {
            let assign8260_e7516: f64 = (p.p636 * locals.var_ile);
            let assign8260_e7517: f64 = (p.p635 + assign8260_e7516);
            let assign8260_e7520: f64 = (p.p637 * locals.var_iwe);
            let assign8260_e7521: f64 = (assign8260_e7517 + assign8260_e7520);
            let assign8260_e7524: f64 = (p.p638 * locals.var_iae);
            let assign8260_e7525: f64 = (assign8260_e7521 + assign8260_e7524);
            let assign8260_e7526: f64 = (locals.var_iiwe * assign8260_e7525);
            locals.var_agidl_p = assign8260_e7526;
        }
        let assign8270_e7547: f64 = if (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8270_e7547;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard103 != 0.0)) {
            let assign8280_e7555: f64 = (p.p640 * locals.var_ile);
            let assign8280_e7556: f64 = (p.p639 + assign8280_e7555);
            let assign8280_e7559: f64 = (p.p641 * locals.var_iwe);
            let assign8280_e7560: f64 = (assign8280_e7556 + assign8280_e7559);
            let assign8280_e7563: f64 = (p.p642 * locals.var_iae);
            let assign8280_e7564: f64 = (assign8280_e7560 + assign8280_e7563);
            let assign8280_e7565: f64 = (locals.var_iiwe * assign8280_e7564);
            locals.var_agidld_p = assign8280_e7565;
        }
        let assign8290_e7586: f64 = if (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8290_e7586;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard104 != 0.0)) {
            let assign8300_e7593: f64 = (p.p644 * locals.var_ile);
            let assign8300_e7594: f64 = (p.p643 + assign8300_e7593);
            let assign8300_e7597: f64 = (p.p645 * locals.var_iwe);
            let assign8300_e7598: f64 = (assign8300_e7594 + assign8300_e7597);
            let assign8300_e7601: f64 = (p.p646 * locals.var_iae);
            let assign8300_e7602: f64 = (assign8300_e7598 + assign8300_e7601);
            locals.var_stbgidl_p = assign8300_e7602;
        }
        let assign8310_e7623: f64 = if (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]) { 1.0 } else { 0.0 };
        locals.var_guard105 = assign8310_e7623;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard105 != 0.0)) {
            let assign8320_e7630: f64 = (p.p648 * locals.var_ile);
            let assign8320_e7631: f64 = (p.p647 + assign8320_e7630);
            let assign8320_e7634: f64 = (p.p649 * locals.var_iwe);
            let assign8320_e7635: f64 = (assign8320_e7631 + assign8320_e7634);
            let assign8320_e7638: f64 = (p.p650 * locals.var_iae);
            let assign8320_e7639: f64 = (assign8320_e7635 + assign8320_e7638);
            locals.var_stbgidld_p = assign8320_e7639;
        }
        let assign8330_e7660: f64 = if (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]) { 1.0 } else { 0.0 };
        locals.var_guard106 = assign8330_e7660;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard106 != 0.0)) {
            let assign8340_e7666: f64 = (locals.var_iiwecv * locals.var_lecv);
            let assign8340_e7668: f64 = (assign8340_e7666 / 1e-6);
            let assign8340_e7672: f64 = (p.p652 * locals.var_ile);
            let assign8340_e7673: f64 = (p.p651 + assign8340_e7672);
            let assign8340_e7676: f64 = (p.p653 * locals.var_iwe);
            let assign8340_e7677: f64 = (assign8340_e7673 + assign8340_e7676);
            let assign8340_e7680: f64 = (p.p654 * locals.var_iae);
            let assign8340_e7681: f64 = (assign8340_e7677 + assign8340_e7680);
            let assign8340_e7682: f64 = (assign8340_e7668 * assign8340_e7681);
            locals.var_cox_p = assign8340_e7682;
        }
        let assign8350_e7703: f64 = if (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]) { 1.0 } else { 0.0 };
        locals.var_guard107 = assign8350_e7703;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard107 != 0.0)) {
            let assign8360_e7710: f64 = (p.p656 * locals.var_ile);
            let assign8360_e7711: f64 = (p.p655 + assign8360_e7710);
            let assign8360_e7714: f64 = (p.p657 * locals.var_iwe);
            let assign8360_e7715: f64 = (assign8360_e7711 + assign8360_e7714);
            let assign8360_e7718: f64 = (p.p658 * locals.var_iae);
            let assign8360_e7719: f64 = (assign8360_e7715 + assign8360_e7718);
            locals.var_delvtac_p = assign8360_e7719;
        }
        let assign8370_e7740: f64 = if (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]) { 1.0 } else { 0.0 };
        locals.var_guard108 = assign8370_e7740;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard108 != 0.0)) {
            let assign8380_e7747: f64 = (p.p660 * locals.var_ile);
            let assign8380_e7748: f64 = (p.p659 + assign8380_e7747);
            let assign8380_e7751: f64 = (p.p661 * locals.var_iwe);
            let assign8380_e7752: f64 = (assign8380_e7748 + assign8380_e7751);
            let assign8380_e7755: f64 = (p.p662 * locals.var_iae);
            let assign8380_e7756: f64 = (assign8380_e7752 + assign8380_e7755);
            locals.var_facneffac_p = assign8380_e7756;
        }
        let assign8390_e7797: f64 = if (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign8390_e7797;
    }
    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
            locals.var_poparam_i = p.p571;
        }
        let assign8410_e7805: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8410_e7807: f64 = if assign8410_e7805 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign8410_e7807;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
            locals.var_poparam_i = p.p663;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
            locals.var_plparam_i = p.p572;
        }
        let assign8440_e7823: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8440_e7825: f64 = if assign8440_e7823 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign8440_e7825;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
            locals.var_plparam_i = p.p664;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
            locals.var_pwparam_i = p.p573;
        }
        let assign8470_e7841: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8470_e7843: f64 = if assign8470_e7841 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign8470_e7843;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard112 != 0.0)) {
            locals.var_pwparam_i = p.p665;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
            locals.var_plwparam_i = p.p574;
        }
        let assign8500_e7859: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8500_e7861: f64 = if assign8500_e7859 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8500_e7861;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
            locals.var_plwparam_i = p.p666;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
            let assign8520_e7877: f64 = (locals.var_plparam_i * locals.var_ile);
            let assign8520_e7878: f64 = (locals.var_poparam_i + assign8520_e7877);
            let assign8520_e7881: f64 = (locals.var_pwparam_i * locals.var_iwe);
            let assign8520_e7882: f64 = (assign8520_e7878 + assign8520_e7881);
            let assign8520_e7885: f64 = (locals.var_plwparam_i * locals.var_iae);
            let assign8520_e7886: f64 = (assign8520_e7882 + assign8520_e7885);
            let assign8520_e7887: f64 = (locals.var_ile * assign8520_e7886);
            locals.var_thesatac_p = assign8520_e7887;
        }
        let assign8530_e7928: f64 = if (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8530_e7928;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
            locals.var_poparam_i = p.p587;
        }
        let assign8550_e7936: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8550_e7938: f64 = if assign8550_e7936 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8550_e7938;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard115 != 0.0)) {
            locals.var_poparam_i = p.p667;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
            locals.var_plparam_i = p.p588;
        }
        let assign8580_e7954: f64 = if param_given[668] { 1.0 } else { 0.0 };
        let assign8580_e7956: f64 = if assign8580_e7954 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8580_e7956;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 != 0.0)) {
            locals.var_plparam_i = p.p668;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
            locals.var_pwparam_i = p.p589;
        }
        let assign8610_e7972: f64 = if param_given[669] { 1.0 } else { 0.0 };
        let assign8610_e7974: f64 = if assign8610_e7972 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8610_e7974;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard117 != 0.0)) {
            locals.var_pwparam_i = p.p669;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
            locals.var_plwparam_i = p.p590;
        }
        let assign8640_e7990: f64 = if param_given[670] { 1.0 } else { 0.0 };
        let assign8640_e7992: f64 = if assign8640_e7990 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8640_e7992;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard118 != 0.0)) {
            locals.var_plwparam_i = p.p670;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
            let assign8660_e8008: f64 = (locals.var_plparam_i * locals.var_ile);
            let assign8660_e8009: f64 = (locals.var_poparam_i + assign8660_e8008);
            let assign8660_e8012: f64 = (locals.var_pwparam_i * locals.var_iwe);
            let assign8660_e8013: f64 = (assign8660_e8009 + assign8660_e8012);
            let assign8660_e8016: f64 = (locals.var_plwparam_i * locals.var_iae);
            let assign8660_e8017: f64 = (assign8660_e8013 + assign8660_e8016);
            let assign8660_e8018: f64 = assign8660_e8017;
            locals.var_axac_p = assign8660_e8018;
        }
        let assign8670_e8039: f64 = if (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign8670_e8039;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard119 != 0.0)) {
            let assign8680_e8047: f64 = (p.p672 * locals.var_ile);
            let assign8680_e8048: f64 = (p.p671 + assign8680_e8047);
            let assign8680_e8051: f64 = (p.p673 * locals.var_iwe);
            let assign8680_e8052: f64 = (assign8680_e8048 + assign8680_e8051);
            let assign8680_e8055: f64 = (p.p674 * locals.var_iae);
            let assign8680_e8056: f64 = (assign8680_e8052 + assign8680_e8055);
            let assign8680_e8057: f64 = (locals.var_ile * assign8680_e8056);
            locals.var_alpac_p = assign8680_e8057;
        }
        let assign8690_e8078: f64 = if (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign8690_e8078;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard120 != 0.0)) {
            let assign8700_e8086: f64 = (p.p676 * locals.var_ile);
            let assign8700_e8087: f64 = (p.p675 + assign8700_e8086);
            let assign8700_e8090: f64 = (p.p677 * locals.var_iwe);
            let assign8700_e8091: f64 = (assign8700_e8087 + assign8700_e8090);
            let assign8700_e8094: f64 = (p.p678 * locals.var_iae);
            let assign8700_e8095: f64 = (assign8700_e8091 + assign8700_e8094);
            let assign8700_e8096: f64 = (locals.var_ile * assign8700_e8095);
            locals.var_alp1ac_p = assign8700_e8096;
        }
        let assign8710_e8117: f64 = if (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]) { 1.0 } else { 0.0 };
        locals.var_guard121 = assign8710_e8117;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard121 != 0.0)) {
            let assign8720_e8125: f64 = (p.p680 * locals.var_ile);
            let assign8720_e8126: f64 = (p.p679 + assign8720_e8125);
            let assign8720_e8129: f64 = (p.p681 * locals.var_iwe);
            let assign8720_e8130: f64 = (assign8720_e8126 + assign8720_e8129);
            let assign8720_e8133: f64 = (p.p682 * locals.var_iae);
            let assign8720_e8134: f64 = (assign8720_e8130 + assign8720_e8133);
            let assign8720_e8135: f64 = (locals.var_iiwecv * assign8720_e8134);
            locals.var_cgov_p = assign8720_e8135;
        }
        let assign8730_e8156: f64 = if (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]) { 1.0 } else { 0.0 };
        locals.var_guard122 = assign8730_e8156;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard122 != 0.0)) {
            let assign8740_e8164: f64 = (p.p684 * locals.var_ile);
            let assign8740_e8165: f64 = (p.p683 + assign8740_e8164);
            let assign8740_e8168: f64 = (p.p685 * locals.var_iwe);
            let assign8740_e8169: f64 = (assign8740_e8165 + assign8740_e8168);
            let assign8740_e8172: f64 = (p.p686 * locals.var_iae);
            let assign8740_e8173: f64 = (assign8740_e8169 + assign8740_e8172);
            let assign8740_e8174: f64 = (locals.var_iiwecv * assign8740_e8173);
            locals.var_cgovd_p = assign8740_e8174;
        }
        let assign8750_e8195: f64 = if (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]) { 1.0 } else { 0.0 };
        locals.var_guard123 = assign8750_e8195;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard123 != 0.0)) {
            let assign8760_e8203: f64 = (p.p688 * locals.var_ile);
            let assign8760_e8204: f64 = (p.p687 + assign8760_e8203);
            let assign8760_e8207: f64 = (p.p689 * locals.var_iwe);
            let assign8760_e8208: f64 = (assign8760_e8204 + assign8760_e8207);
            let assign8760_e8211: f64 = (p.p690 * locals.var_iae);
            let assign8760_e8212: f64 = (assign8760_e8208 + assign8760_e8211);
            let assign8760_e8213: f64 = (locals.var_iilcv * assign8760_e8212);
            locals.var_cgbov_p = assign8760_e8213;
        }
        let assign8770_e8234: f64 = if (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]) { 1.0 } else { 0.0 };
        locals.var_guard124 = assign8770_e8234;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard124 != 0.0)) {
            let assign8780_e8242: f64 = (p.p692 * locals.var_ile);
            let assign8780_e8243: f64 = (p.p691 + assign8780_e8242);
            let assign8780_e8246: f64 = (p.p693 * locals.var_iwe);
            let assign8780_e8247: f64 = (assign8780_e8243 + assign8780_e8246);
            let assign8780_e8250: f64 = (p.p694 * locals.var_iae);
            let assign8780_e8251: f64 = (assign8780_e8247 + assign8780_e8250);
            let assign8780_e8252: f64 = (locals.var_iiwecv * assign8780_e8251);
            locals.var_cinr_p = assign8780_e8252;
        }
        let assign8790_e8273: f64 = if (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]) { 1.0 } else { 0.0 };
        locals.var_guard125 = assign8790_e8273;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard125 != 0.0)) {
            let assign8800_e8281: f64 = (p.p696 * locals.var_ile);
            let assign8800_e8282: f64 = (p.p695 + assign8800_e8281);
            let assign8800_e8285: f64 = (p.p697 * locals.var_iwe);
            let assign8800_e8286: f64 = (assign8800_e8282 + assign8800_e8285);
            let assign8800_e8289: f64 = (p.p698 * locals.var_iae);
            let assign8800_e8290: f64 = (assign8800_e8286 + assign8800_e8289);
            let assign8800_e8291: f64 = (locals.var_iiwecv * assign8800_e8290);
            locals.var_cinrd_p = assign8800_e8291;
        }
        let assign8810_e8312: f64 = if (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]) { 1.0 } else { 0.0 };
        locals.var_guard126 = assign8810_e8312;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard126 != 0.0)) {
            let assign8820_e8320: f64 = (p.p700 * locals.var_ile);
            let assign8820_e8321: f64 = (p.p699 + assign8820_e8320);
            let assign8820_e8324: f64 = (p.p701 * locals.var_iwe);
            let assign8820_e8325: f64 = (assign8820_e8321 + assign8820_e8324);
            let assign8820_e8328: f64 = (p.p702 * locals.var_iae);
            let assign8820_e8329: f64 = (assign8820_e8325 + assign8820_e8328);
            let assign8820_e8330: f64 = (locals.var_iiwcv * assign8820_e8329);
            locals.var_cfr_p = assign8820_e8330;
        }
        let assign8830_e8351: f64 = if (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign8830_e8351;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard127 != 0.0)) {
            let assign8840_e8359: f64 = (p.p704 * locals.var_ile);
            let assign8840_e8360: f64 = (p.p703 + assign8840_e8359);
            let assign8840_e8363: f64 = (p.p705 * locals.var_iwe);
            let assign8840_e8364: f64 = (assign8840_e8360 + assign8840_e8363);
            let assign8840_e8367: f64 = (p.p706 * locals.var_iae);
            let assign8840_e8368: f64 = (assign8840_e8364 + assign8840_e8367);
            let assign8840_e8369: f64 = (locals.var_iiwcv * assign8840_e8368);
            locals.var_cfrd_p = assign8840_e8369;
        }
        let assign8850_e8390: f64 = if (((param_given[707] || param_given[708]) || param_given[709]) || param_given[710]) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign8850_e8390;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard128 != 0.0)) {
            let assign8860_e8398: f64 = (p.p708 * locals.var_ile);
            let assign8860_e8399: f64 = (p.p707 + assign8860_e8398);
            let assign8860_e8402: f64 = (p.p709 * locals.var_iwe);
            let assign8860_e8403: f64 = (assign8860_e8399 + assign8860_e8402);
            let assign8860_e8406: f64 = (p.p710 * locals.var_iae);
            let assign8860_e8407: f64 = (assign8860_e8403 + assign8860_e8406);
            let assign8860_e8408: f64 = (locals.var_ile2 * assign8860_e8407);
            locals.var_fntexc_p = assign8860_e8408;
        }
        let assign8930_e8546: f64 = if (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8930_e8546;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard132 != 0.0)) {
            let assign8940_e8553: f64 = (p.p724 * locals.var_ile);
            let assign8940_e8554: f64 = (p.p723 + assign8940_e8553);
            let assign8940_e8557: f64 = (p.p725 * locals.var_iwe);
            let assign8940_e8558: f64 = (assign8940_e8554 + assign8940_e8557);
            let assign8940_e8561: f64 = (p.p726 * locals.var_iae);
            let assign8940_e8562: f64 = (assign8940_e8558 + assign8940_e8561);
            locals.var_vfbedge_p = assign8940_e8562;
        }
        let assign8950_e8583: f64 = if (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8950_e8583;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard133 != 0.0)) {
            let assign8960_e8590: f64 = (p.p728 * locals.var_ile);
            let assign8960_e8591: f64 = (p.p727 + assign8960_e8590);
            let assign8960_e8594: f64 = (p.p729 * locals.var_iwe);
            let assign8960_e8595: f64 = (assign8960_e8591 + assign8960_e8594);
            let assign8960_e8598: f64 = (p.p730 * locals.var_iae);
            let assign8960_e8599: f64 = (assign8960_e8595 + assign8960_e8598);
            locals.var_stvfbedge_p = assign8960_e8599;
        }
        let assign8970_e8620: f64 = if (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8970_e8620;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard134 != 0.0)) {
            let assign8980_e8627: f64 = (p.p732 * locals.var_ile);
            let assign8980_e8628: f64 = (p.p731 + assign8980_e8627);
            let assign8980_e8631: f64 = (p.p733 * locals.var_iwe);
            let assign8980_e8632: f64 = (assign8980_e8628 + assign8980_e8631);
            let assign8980_e8635: f64 = (p.p734 * locals.var_iae);
            let assign8980_e8636: f64 = (assign8980_e8632 + assign8980_e8635);
            locals.var_dphibedge_p = assign8980_e8636;
        }
        let assign8990_e8657: f64 = if (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]) { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8990_e8657;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard135 != 0.0)) {
            let assign9000_e8664: f64 = (p.p736 * locals.var_ile);
            let assign9000_e8665: f64 = (p.p735 + assign9000_e8664);
            let assign9000_e8668: f64 = (p.p737 * locals.var_iwe);
            let assign9000_e8669: f64 = (assign9000_e8665 + assign9000_e8668);
            let assign9000_e8672: f64 = (p.p738 * locals.var_iae);
            let assign9000_e8673: f64 = (assign9000_e8669 + assign9000_e8672);
            locals.var_neffedge_p = assign9000_e8673;
        }
        let assign9010_e8694: f64 = if (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign9010_e8694;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard136 != 0.0)) {
            let assign9020_e8701: f64 = (p.p740 * locals.var_ile);
            let assign9020_e8702: f64 = (p.p739 + assign9020_e8701);
            let assign9020_e8705: f64 = (p.p741 * locals.var_iwe);
            let assign9020_e8706: f64 = (assign9020_e8702 + assign9020_e8705);
            let assign9020_e8709: f64 = (p.p742 * locals.var_iae);
            let assign9020_e8710: f64 = (assign9020_e8706 + assign9020_e8709);
            locals.var_ctedge_p = assign9020_e8710;
        }
        let assign9030_e8731: f64 = if (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign9030_e8731;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard137 != 0.0)) {
            let assign9040_e8737: f64 = (locals.var_we_edge / locals.var_le);
            let assign9040_e8741: f64 = (p.p744 * locals.var_ile);
            let assign9040_e8742: f64 = (p.p743 + assign9040_e8741);
            let assign9040_e8745: f64 = (p.p745 * locals.var_iwe);
            let assign9040_e8746: f64 = (assign9040_e8742 + assign9040_e8745);
            let assign9040_e8749: f64 = (p.p746 * locals.var_iae);
            let assign9040_e8750: f64 = (assign9040_e8746 + assign9040_e8749);
            let assign9040_e8751: f64 = (assign9040_e8737 * assign9040_e8750);
            locals.var_betnedge_p = assign9040_e8751;
        }
        let assign9050_e8772: f64 = if (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign9050_e8772;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard138 != 0.0)) {
            let assign9060_e8779: f64 = (p.p748 * locals.var_ile);
            let assign9060_e8780: f64 = (p.p747 + assign9060_e8779);
            let assign9060_e8783: f64 = (p.p749 * locals.var_iwe);
            let assign9060_e8784: f64 = (assign9060_e8780 + assign9060_e8783);
            let assign9060_e8787: f64 = (p.p750 * locals.var_iae);
            let assign9060_e8788: f64 = (assign9060_e8784 + assign9060_e8787);
            locals.var_stbetedge_p = assign9060_e8788;
        }
        let assign9070_e8809: f64 = if (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign9070_e8809;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard139 != 0.0)) {
            let assign9080_e8817: f64 = (p.p752 * locals.var_ile);
            let assign9080_e8818: f64 = (p.p751 + assign9080_e8817);
            let assign9080_e8821: f64 = (p.p753 * locals.var_iwe);
            let assign9080_e8822: f64 = (assign9080_e8818 + assign9080_e8821);
            let assign9080_e8825: f64 = (p.p754 * locals.var_iae);
            let assign9080_e8826: f64 = (assign9080_e8822 + assign9080_e8825);
            let assign9080_e8827: f64 = (locals.var_ile2 * assign9080_e8826);
            locals.var_psceedge_p = assign9080_e8827;
        }
        let assign9090_e8848: f64 = if (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]) { 1.0 } else { 0.0 };
        locals.var_guard140 = assign9090_e8848;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard140 != 0.0)) {
            let assign9100_e8855: f64 = (p.p756 * locals.var_ile);
            let assign9100_e8856: f64 = (p.p755 + assign9100_e8855);
            let assign9100_e8859: f64 = (p.p757 * locals.var_iwe);
            let assign9100_e8860: f64 = (assign9100_e8856 + assign9100_e8859);
            let assign9100_e8863: f64 = (p.p758 * locals.var_iae);
            let assign9100_e8864: f64 = (assign9100_e8860 + assign9100_e8863);
            locals.var_pscebedge_p = assign9100_e8864;
        }
        let assign9110_e8885: f64 = if (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]) { 1.0 } else { 0.0 };
        locals.var_guard141 = assign9110_e8885;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard141 != 0.0)) {
            let assign9120_e8892: f64 = (p.p760 * locals.var_ile);
            let assign9120_e8893: f64 = (p.p759 + assign9120_e8892);
            let assign9120_e8896: f64 = (p.p761 * locals.var_iwe);
            let assign9120_e8897: f64 = (assign9120_e8893 + assign9120_e8896);
            let assign9120_e8900: f64 = (p.p762 * locals.var_iae);
            let assign9120_e8901: f64 = (assign9120_e8897 + assign9120_e8900);
            locals.var_pscededge_p = assign9120_e8901;
        }
        let assign9130_e8922: f64 = if (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]) { 1.0 } else { 0.0 };
        locals.var_guard142 = assign9130_e8922;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard142 != 0.0)) {
            let assign9140_e8930: f64 = (p.p764 * locals.var_ile);
            let assign9140_e8931: f64 = (p.p763 + assign9140_e8930);
            let assign9140_e8934: f64 = (p.p765 * locals.var_iwe);
            let assign9140_e8935: f64 = (assign9140_e8931 + assign9140_e8934);
            let assign9140_e8938: f64 = (p.p766 * locals.var_iae);
            let assign9140_e8939: f64 = (assign9140_e8935 + assign9140_e8938);
            let assign9140_e8940: f64 = (locals.var_ile2 * assign9140_e8939);
            locals.var_cfedge_p = assign9140_e8940;
        }
        let assign9150_e8961: f64 = if (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]) { 1.0 } else { 0.0 };
        locals.var_guard143 = assign9150_e8961;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard143 != 0.0)) {
            let assign9160_e8968: f64 = (p.p772 * locals.var_ile);
            let assign9160_e8969: f64 = (p.p771 + assign9160_e8968);
            let assign9160_e8972: f64 = (p.p773 * locals.var_iwe);
            let assign9160_e8973: f64 = (assign9160_e8969 + assign9160_e8972);
            let assign9160_e8976: f64 = (p.p774 * locals.var_iae);
            let assign9160_e8977: f64 = (assign9160_e8973 + assign9160_e8976);
            locals.var_cfdedge_p = assign9160_e8977;
        }
        let assign9170_e8998: f64 = if (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign9170_e8998;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9180_e9005: f64 = (p.p768 * locals.var_ile);
            let assign9180_e9006: f64 = (p.p767 + assign9180_e9005);
            let assign9180_e9009: f64 = (p.p769 * locals.var_iwe);
            let assign9180_e9010: f64 = (assign9180_e9006 + assign9180_e9009);
            let assign9180_e9013: f64 = (p.p770 * locals.var_iae);
            let assign9180_e9014: f64 = (assign9180_e9010 + assign9180_e9013);
            locals.var_cfbedge_p = assign9180_e9014;
        }
        let assign9250_e9152: f64 = if (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign9250_e9152;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard148 != 0.0)) {
            let assign9260_e9159: f64 = (p.p788 * locals.var_ile);
            let assign9260_e9160: f64 = (p.p787 + assign9260_e9159);
            let assign9260_e9163: f64 = (p.p789 * locals.var_iwe);
            let assign9260_e9164: f64 = (assign9260_e9160 + assign9260_e9163);
            let assign9260_e9167: f64 = (p.p790 * locals.var_iae);
            let assign9260_e9168: f64 = (assign9260_e9164 + assign9260_e9167);
            locals.var_munqs_p = assign9260_e9168;
        }
        if (locals.var_guard41 != 0.0) {
            locals.var_tmpa = 0.0;
            locals.var_tmpb = 0.0;
            locals.var_loop_ = 0.0;
            locals.var_kvsatac_i = p.p795;
        }
        let assign9310_e9188: f64 = if param_given[796] { 1.0 } else { 0.0 };
        let assign9310_e9190: f64 = if assign9310_e9188 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign9310_e9190;
        if ((locals.var_guard41 != 0.0) && (locals.var_guard149 != 0.0)) {
            locals.var_kvsatac_i = p.p796;
        }
        let assign9330_e9215: f64 = if (((locals.var_sa_i > 0.0) && (locals.var_sb_i > 0.0)) && ((locals.var_nf_i == 1.0) || ((locals.var_nf_i > 1.0) && (locals.var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign9330_e9215;
        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (locals.var_nf_i - 0.5);
            let assign9340_cond_e9224: f64 = if (((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_loop_ < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
                let assign9340_body0_e9233: f64 = (0.5 * locals.var_l_i);
                let assign9340_body0_e9234: f64 = (locals.var_sa_i + assign9340_body0_e9233);
                let assign9340_body0_e9238: f64 = (locals.var_sd_i + locals.var_l_i);
                let assign9340_body0_e9239: f64 = (locals.var_loop_ * assign9340_body0_e9238);
                let assign9340_body0_e9240: f64 = (assign9340_body0_e9234 + assign9340_body0_e9239);
                let assign9340_body0_e9241: f64 = (1.0 / assign9340_body0_e9240);
                let assign9340_body0_e9242: f64 = (locals.var_tmpa + assign9340_body0_e9241);
                locals.var_tmpa = assign9340_body0_e9242;
            }
            if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
                let assign9340_body1_e9253: f64 = (0.5 * locals.var_l_i);
                let assign9340_body1_e9254: f64 = (locals.var_sb_i + assign9340_body1_e9253);
                let assign9340_body1_e9258: f64 = (locals.var_sd_i + locals.var_l_i);
                let assign9340_body1_e9259: f64 = (locals.var_loop_ * assign9340_body1_e9258);
                let assign9340_body1_e9260: f64 = (assign9340_body1_e9254 + assign9340_body1_e9259);
                let assign9340_body1_e9261: f64 = (1.0 / assign9340_body1_e9260);
                let assign9340_body1_e9262: f64 = (locals.var_tmpb + assign9340_body1_e9261);
                locals.var_tmpb = assign9340_body1_e9262;
            }
            if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
                let assign9340_body2_e9270: f64 = (locals.var_loop_ + 1.0);
                locals.var_loop_ = assign9340_body2_e9270;
            }
        }
    }
    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9350_e9278: f64 = (locals.var_tmpa * locals.var_invnf);
            locals.var_invsa = assign9350_e9278;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9360_e9286: f64 = (locals.var_tmpb * locals.var_invnf);
            locals.var_invsb = assign9360_e9286;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9370_e9296: f64 = (0.5 * locals.var_l_i);
            let assign9370_e9297: f64 = (p.p791 + assign9370_e9296);
            let assign9370_e9298: f64 = (1.0 / assign9370_e9297);
            locals.var_invsaref = assign9370_e9298;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9380_e9308: f64 = (0.5 * locals.var_l_i);
            let assign9380_e9309: f64 = (p.p792 + assign9380_e9308);
            let assign9380_e9310: f64 = (1.0 / assign9380_e9309);
            locals.var_invsbref = assign9380_e9310;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9390_e9318: f64 = (locals.var_l_i + locals.var_dellps);
            let (assign9390_e9325,) = {
    if (assign9390_e9318 > 1e-9) {
        let assign9390_e9323: f64 = (locals.var_l_i + locals.var_dellps);
        (assign9390_e9323,)
    } else {
        (1e-9,)
    }
};
            locals.var_lx = assign9390_e9325;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9400_e9333: f64 = (locals.var_w_i + locals.var_delwod);
            let assign9400_e9335: f64 = (assign9400_e9333 + p.p793);
            let (assign9400_e9344,) = {
    if (assign9400_e9335 > 1e-9) {
        let assign9400_e9340: f64 = (locals.var_w_i + locals.var_delwod);
        let assign9400_e9342: f64 = (assign9400_e9340 + p.p793);
        (assign9400_e9342,)
    } else {
        (1e-9,)
    }
};
            locals.var_wx = assign9400_e9344;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9410_e9353: f64 = (locals.var_lx).powf(p.p801);
            let assign9410_e9354: f64 = (1.0 / assign9410_e9353);
            locals.var_templ = assign9410_e9354;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9420_e9363: f64 = (locals.var_wx).powf(p.p802);
            let assign9420_e9364: f64 = (1.0 / assign9420_e9363);
            locals.var_tempw = assign9420_e9364;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9430_e9373: f64 = (p.p798 * locals.var_templ);
            let assign9430_e9374: f64 = (1.0 + assign9430_e9373);
            let assign9430_e9377: f64 = (p.p799 * locals.var_tempw);
            let assign9430_e9378: f64 = (assign9430_e9374 + assign9430_e9377);
            let assign9430_e9381: f64 = (p.p800 * locals.var_templ);
            let assign9430_e9383: f64 = (assign9430_e9381 * locals.var_tempw);
            let assign9430_e9384: f64 = (assign9430_e9378 + assign9430_e9383);
            let assign9430_e9389: f64 = (locals.var_rta - 1.0);
            let assign9430_e9390: f64 = (p.p797 * assign9430_e9389);
            let assign9430_e9391: f64 = (1.0 + assign9430_e9390);
            let assign9430_e9392: f64 = (assign9430_e9384 * assign9430_e9391);
            locals.var_kstressu0 = assign9430_e9392;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9440_e9401: f64 = (locals.var_invsa + locals.var_invsb);
            let assign9440_e9402: f64 = (p.p794 * assign9440_e9401);
            let assign9440_e9404: f64 = (assign9440_e9402 / locals.var_kstressu0);
            locals.var_rhobeta = assign9440_e9404;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9450_e9413: f64 = (locals.var_invsaref + locals.var_invsbref);
            let assign9450_e9414: f64 = (p.p794 * assign9450_e9413);
            let assign9450_e9416: f64 = (assign9450_e9414 / locals.var_kstressu0);
            locals.var_rhobetaref = assign9450_e9416;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9460_e9425: f64 = (locals.var_lx).powf(p.p807);
            let assign9460_e9426: f64 = (1.0 / assign9460_e9425);
            locals.var_templ = assign9460_e9426;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9470_e9435: f64 = (locals.var_wx).powf(p.p808);
            let assign9470_e9436: f64 = (1.0 / assign9470_e9435);
            locals.var_tempw = assign9470_e9436;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9480_e9445: f64 = (p.p804 * locals.var_templ);
            let assign9480_e9446: f64 = (1.0 + assign9480_e9445);
            let assign9480_e9449: f64 = (p.p805 * locals.var_tempw);
            let assign9480_e9450: f64 = (assign9480_e9446 + assign9480_e9449);
            let assign9480_e9453: f64 = (p.p806 * locals.var_templ);
            let assign9480_e9455: f64 = (assign9480_e9453 * locals.var_tempw);
            let assign9480_e9456: f64 = (assign9480_e9450 + assign9480_e9455);
            locals.var_kstressvth0 = assign9480_e9456;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9490_e9464: f64 = (locals.var_invsa + locals.var_invsb);
            let assign9490_e9466: f64 = (assign9490_e9464 - locals.var_invsaref);
            let assign9490_e9468: f64 = (assign9490_e9466 - locals.var_invsbref);
            locals.var_temp0 = assign9490_e9468;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9500_e9476: f64 = (1.0 + locals.var_rhobeta);
            let assign9500_e9479: f64 = (1.0 + locals.var_rhobetaref);
            let assign9500_e9480: f64 = (assign9500_e9476 / assign9500_e9479);
            locals.var_temp00 = assign9500_e9480;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9510_e9488: f64 = (locals.var_betn_p * locals.var_temp00);
            locals.var_betn_p = assign9510_e9488;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9520_e9496: f64 = (locals.var_thesat_p * locals.var_temp00);
            let assign9520_e9500: f64 = (p.p795 * locals.var_rhobetaref);
            let assign9520_e9501: f64 = (1.0 + assign9520_e9500);
            let assign9520_e9502: f64 = (assign9520_e9496 * assign9520_e9501);
            let assign9520_e9506: f64 = (p.p795 * locals.var_rhobeta);
            let assign9520_e9507: f64 = (1.0 + assign9520_e9506);
            let assign9520_e9508: f64 = (assign9520_e9502 / assign9520_e9507);
            locals.var_thesat_p = assign9520_e9508;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9530_e9516: f64 = (locals.var_thesatac_p * locals.var_temp00);
            let assign9530_e9520: f64 = (locals.var_kvsatac_i * locals.var_rhobetaref);
            let assign9530_e9521: f64 = (1.0 + assign9530_e9520);
            let assign9530_e9522: f64 = (assign9530_e9516 * assign9530_e9521);
            let assign9530_e9526: f64 = (locals.var_kvsatac_i * locals.var_rhobeta);
            let assign9530_e9527: f64 = (1.0 + assign9530_e9526);
            let assign9530_e9528: f64 = (assign9530_e9522 / assign9530_e9527);
            locals.var_thesatac_p = assign9530_e9528;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9540_e9536: f64 = (locals.var_betnedge_p * locals.var_temp00);
            locals.var_betnedge_p = assign9540_e9536;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9550_e9544: f64 = (p.p803 * locals.var_temp0);
            let assign9550_e9546: f64 = (assign9550_e9544 / locals.var_kstressvth0);
            locals.var_temp00 = assign9550_e9546;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9560_e9554: f64 = (locals.var_vfb_p + locals.var_temp00);
            locals.var_vfb_p = assign9560_e9554;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9570_e9562: f64 = (locals.var_vfbedge_p + locals.var_temp00);
            locals.var_vfbedge_p = assign9570_e9562;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9580_e9570: f64 = (p.p809 * locals.var_temp0);
            let assign9580_e9573: f64 = (locals.var_kstressvth0).powf(p.p810);
            let assign9580_e9574: f64 = (assign9580_e9570 / assign9580_e9573);
            locals.var_temp00 = assign9580_e9574;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9590_e9582: f64 = (locals.var_cf_p + locals.var_temp00);
            locals.var_cf_p = assign9590_e9582;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
            let assign9600_e9590: f64 = (locals.var_cfedge_p + locals.var_temp00);
            locals.var_cfedge_p = assign9600_e9590;
        }
        let assign9610_e9607: f64 = if ((((locals.var_sca_i > 0.0) || (locals.var_scb_i > 0.0)) || (locals.var_scc_i > 0.0)) || (locals.var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard151 = assign9610_e9607;
        let assign9620_e9618: f64 = if (((locals.var_sca_i == 0.0) && (locals.var_scb_i == 0.0)) && (locals.var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard152 = assign9620_e9618;
        if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
            let assign9630_e9626: f64 = (locals.var_sc_i + locals.var_w_i);
            locals.var_temp0 = assign9630_e9626;
        }
        if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
            let assign9640_e9636: f64 = (1.0 / p.p811);
            locals.var_temp00 = assign9640_e9636;
        }
        if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
            let assign9650_e9646: f64 = (p.p811 * p.p811);
            let assign9650_e9649: f64 = (locals.var_sc_i * locals.var_temp0);
            let assign9650_e9650: f64 = (assign9650_e9646 / assign9650_e9649);
            locals.var_sca_i = assign9650_e9650;
        }
        if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
            let assign9660_e9660: f64 = (0.1 * locals.var_sc_i);
            let assign9660_e9663: f64 = (0.01 * p.p811);
            let assign9660_e9664: f64 = (assign9660_e9660 + assign9660_e9663);
            let assign9660_e9666: f64 = (-10.0);
            let assign9660_e9668: f64 = (assign9660_e9666 * locals.var_sc_i);
            let assign9660_e9670: f64 = (assign9660_e9668 * locals.var_temp00);
            let assign9660_e9671: f64 = (assign9660_e9670).exp();
            let assign9660_e9672: f64 = (assign9660_e9664 * assign9660_e9671);
            let assign9660_e9675: f64 = (0.1 * locals.var_temp0);
            let assign9660_e9678: f64 = (0.01 * p.p811);
            let assign9660_e9679: f64 = (assign9660_e9675 + assign9660_e9678);
            let assign9660_e9681: f64 = (-10.0);
            let assign9660_e9683: f64 = (assign9660_e9681 * locals.var_temp0);
            let assign9660_e9685: f64 = (assign9660_e9683 * locals.var_temp00);
            let assign9660_e9686: f64 = (assign9660_e9685).exp();
            let assign9660_e9687: f64 = (assign9660_e9679 * assign9660_e9686);
            let assign9660_e9688: f64 = (assign9660_e9672 - assign9660_e9687);
            let assign9660_e9690: f64 = (assign9660_e9688 / locals.var_w_i);
            locals.var_scb_i = assign9660_e9690;
        }
        if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
            let assign9670_e9700: f64 = (0.05 * locals.var_sc_i);
            let assign9670_e9703: f64 = (0.0025 * p.p811);
            let assign9670_e9704: f64 = (assign9670_e9700 + assign9670_e9703);
            let assign9670_e9706: f64 = (-20.0);
            let assign9670_e9708: f64 = (assign9670_e9706 * locals.var_sc_i);
            let assign9670_e9710: f64 = (assign9670_e9708 * locals.var_temp00);
            let assign9670_e9711: f64 = (assign9670_e9710).exp();
            let assign9670_e9712: f64 = (assign9670_e9704 * assign9670_e9711);
            let assign9670_e9715: f64 = (0.05 * locals.var_temp0);
            let assign9670_e9718: f64 = (0.0025 * p.p811);
            let assign9670_e9719: f64 = (assign9670_e9715 + assign9670_e9718);
            let assign9670_e9721: f64 = (-20.0);
            let assign9670_e9723: f64 = (assign9670_e9721 * locals.var_temp0);
            let assign9670_e9725: f64 = (assign9670_e9723 * locals.var_temp00);
            let assign9670_e9726: f64 = (assign9670_e9725).exp();
            let assign9670_e9727: f64 = (assign9670_e9719 * assign9670_e9726);
            let assign9670_e9728: f64 = (assign9670_e9712 - assign9670_e9727);
            let assign9670_e9730: f64 = (assign9670_e9728 / locals.var_w_i);
            locals.var_scc_i = assign9670_e9730;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
            let assign9680_e9739: f64 = (p.p812 * locals.var_scb_i);
            let assign9680_e9740: f64 = (locals.var_sca_i + assign9680_e9739);
            let assign9680_e9743: f64 = (p.p813 * locals.var_scc_i);
            let assign9680_e9744: f64 = (assign9680_e9740 + assign9680_e9743);
            locals.var_temp0 = assign9680_e9744;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
            let assign9690_e9753: f64 = (locals.var_kvthowe * locals.var_temp0);
            let assign9690_e9754: f64 = (locals.var_vfb_p + assign9690_e9753);
            locals.var_vfb_p = assign9690_e9754;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
            let assign9700_e9764: f64 = (locals.var_kuowe * locals.var_temp0);
            let assign9700_e9765: f64 = (1.0 + assign9700_e9764);
            let assign9700_e9766: f64 = (locals.var_betn_p * assign9700_e9765);
            locals.var_betn_p = assign9700_e9766;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
            let assign9710_e9775: f64 = (locals.var_kvthowe * locals.var_temp0);
            let assign9710_e9776: f64 = (locals.var_vfbedge_p + assign9710_e9775);
            locals.var_vfbedge_p = assign9710_e9776;
        }
        if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
            let assign9720_e9786: f64 = (locals.var_kuowe * locals.var_temp0);
            let assign9720_e9787: f64 = (1.0 + assign9720_e9786);
            let assign9720_e9788: f64 = (locals.var_betnedge_p * assign9720_e9787);
            locals.var_betnedge_p = assign9720_e9788;
        }
        locals.var_vfb_i = locals.var_vfb_p;
        locals.var_stvfb_i = locals.var_stvfb_p;
        locals.var_st2vfb_i = locals.var_st2vfb_p;
        locals.var_tox_i = locals.var_tox_p;
        locals.var_epsrox_i = locals.var_epsrox_p;
        let (assign9780_e9806,) = {
    if (locals.var_neff_p > 1e20) {
        let (assign9780_e9804,) = {
            if (locals.var_neff_p < 1e26) {
                (locals.var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9780_e9804,)
    } else {
        (1e20,)
    }
};
        locals.var_neff_i = assign9780_e9806;
        let (assign9790_e9812,) = {
    if (locals.var_gfacnud_p > 0.01) {
        (locals.var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        locals.var_gfacnud_i = assign9790_e9812;
        let (assign9800_e9818,) = {
    if (locals.var_vsbnud_p > 0.0) {
        (locals.var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        locals.var_vsbnud_i = assign9800_e9818;
        locals.var_dvsbnud_i = locals.var_dvsbnud_p;
        locals.var_dphib_i = locals.var_dphib_p;
        let (assign9830_e9826,) = {
    if (locals.var_np_p > 0.0) {
        (locals.var_np_p,)
    } else {
        (0.0,)
    }
};
        locals.var_np_i = assign9830_e9826;
        locals.var_toxov_i = locals.var_toxov_p;
        locals.var_toxovd_i = locals.var_toxovd_p;
        let (assign9860_e9839,) = {
    if (locals.var_nov_p > 1e23) {
        let (assign9860_e9837,) = {
            if (locals.var_nov_p < 1e27) {
                (locals.var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9860_e9837,)
    } else {
        (1e23,)
    }
};
        locals.var_nov_i = assign9860_e9839;
        let (assign9870_e9850,) = {
    if (locals.var_novd_p > 1e23) {
        let (assign9870_e9848,) = {
            if (locals.var_novd_p < 1e27) {
                (locals.var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9870_e9848,)
    } else {
        (1e23,)
    }
};
        locals.var_novd_i = assign9870_e9850;
        let (assign9880_e9856,) = {
    if (locals.var_ct_p > 0.0) {
        (locals.var_ct_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ct_i = assign9880_e9856;
        let (assign9890_e9867,) = {
    if (locals.var_ctb_p > 0.0) {
        let (assign9890_e9865,) = {
            if (locals.var_ctb_p < 0.5) {
                (locals.var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9890_e9865,)
    } else {
        (0.0,)
    }
};
        locals.var_ctb_i = assign9890_e9867;
        let (assign9900_e9878,) = {
    if (locals.var_ctg_p > 0.0) {
        let (assign9900_e9876,) = {
            if (locals.var_ctg_p < 1.0) {
                (locals.var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9900_e9876,)
    } else {
        (0.0,)
    }
};
        locals.var_ctg_i = assign9900_e9878;
        locals.var_stct_i = locals.var_stct_p;
        let (assign9920_e9885,) = {
    if (locals.var_cf_p > 0.0) {
        (locals.var_cf_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cf_i = assign9920_e9885;
        let (assign9930_e9896,) = {
    if (locals.var_cfb_p > 0.0) {
        let (assign9930_e9894,) = {
            if (locals.var_cfb_p < 1.0) {
                (locals.var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9930_e9894,)
    } else {
        (0.0,)
    }
};
        locals.var_cfb_i = assign9930_e9896;
        let (assign9940_e9902,) = {
    if (locals.var_cfd_p > 0.0) {
        (locals.var_cfd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfd_i = assign9940_e9902;
        let (assign9950_e9908,) = {
    if (locals.var_psce_p > 0.0) {
        (locals.var_psce_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psce_i = assign9950_e9908;
        let (assign9960_e9919,) = {
    if (locals.var_psceb_p > 0.0) {
        let (assign9960_e9917,) = {
            if (locals.var_psceb_p < 1.0) {
                (locals.var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9960_e9917,)
    } else {
        (0.0,)
    }
};
        locals.var_psceb_i = assign9960_e9919;
        let (assign9970_e9925,) = {
    if (locals.var_psced_p > 0.0) {
        (locals.var_psced_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psced_i = assign9970_e9925;
        let (assign9980_e9931,) = {
    if (locals.var_betn_p > 0.0) {
        (locals.var_betn_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betn_i = assign9980_e9931;
        locals.var_stbet_i = locals.var_stbet_p;
    }
    pub(super) fn stamp_transient_block_10(
        locals: &mut StampLocals,
    ) {
        let (assign10000_e9938,) = {
    if (locals.var_mue_p > 0.0) {
        (locals.var_mue_p,)
    } else {
        (0.0,)
    }
};
        locals.var_mue_i = assign10000_e9938;
        locals.var_stmue_i = locals.var_stmue_p;
        let (assign10020_e9945,) = {
    if (locals.var_themu_p > 0.0) {
        (locals.var_themu_p,)
    } else {
        (0.0,)
    }
};
        locals.var_themu_i = assign10020_e9945;
        locals.var_stthemu_i = locals.var_stthemu_p;
        let (assign10040_e9952,) = {
    if (locals.var_cs_p > 0.0) {
        (locals.var_cs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cs_i = assign10040_e9952;
        locals.var_stcs_i = locals.var_stcs_p;
        let (assign10060_e9959,) = {
    if (locals.var_thecs_p > 0.0) {
        (locals.var_thecs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thecs_i = assign10060_e9959;
        locals.var_stthecs_i = locals.var_stthecs_p;
        let (assign10080_e9966,) = {
    if (locals.var_xcor_p > 0.0) {
        (locals.var_xcor_p,)
    } else {
        (0.0,)
    }
};
        locals.var_xcor_i = assign10080_e9966;
        locals.var_stxcor_i = locals.var_stxcor_p;
        locals.var_feta_i = locals.var_feta_p;
        let (assign10110_e9974,) = {
    if (locals.var_rs_p > 0.0) {
        (locals.var_rs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rs_i = assign10110_e9974;
        locals.var_strs_i = locals.var_strs_p;
        let assign10130_e9978: f64 = (-0.5);
        let (assign10130_e9988,) = {
    if (locals.var_rsb_p > assign10130_e9978) {
        let (assign10130_e9985,) = {
            if (locals.var_rsb_p < 1.0) {
                (locals.var_rsb_p,)
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
        locals.var_rsb_i = assign10130_e9988;
        let assign10140_e9991: f64 = (-0.5);
        let (assign10140_e9996,) = {
    if (locals.var_rsg_p > assign10140_e9991) {
        (locals.var_rsg_p,)
    } else {
        let assign10140_e9995: f64 = (-0.5);
        (assign10140_e9995,)
    }
};
        locals.var_rsg_i = assign10140_e9996;
        let (assign10150_e10002,) = {
    if (locals.var_thesat_p > 0.0) {
        (locals.var_thesat_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesat_i = assign10150_e10002;
        locals.var_stthesat_i = locals.var_stthesat_p;
        let assign10170_e10006: f64 = (-0.5);
        let (assign10170_e10016,) = {
    if (locals.var_thesatb_p > assign10170_e10006) {
        let (assign10170_e10013,) = {
            if (locals.var_thesatb_p < 1.0) {
                (locals.var_thesatb_p,)
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
        locals.var_thesatb_i = assign10170_e10016;
        let assign10180_e10019: f64 = (-0.5);
        let (assign10180_e10024,) = {
    if (locals.var_thesatg_p > assign10180_e10019) {
        (locals.var_thesatg_p,)
    } else {
        let assign10180_e10023: f64 = (-0.5);
        (assign10180_e10023,)
    }
};
        locals.var_thesatg_i = assign10180_e10024;
        let (assign10190_e10030,) = {
    if (locals.var_thesatt_p > 0.01) {
        (locals.var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        locals.var_thesatt_i = assign10190_e10030;
        let (assign10200_e10036,) = {
    if (locals.var_ax_p > 2.0) {
        (locals.var_ax_p,)
    } else {
        (2.0,)
    }
};
        locals.var_ax_i = assign10200_e10036;
        let (assign10210_e10042,) = {
    if (locals.var_alp_p > 0.0) {
        (locals.var_alp_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp_i = assign10210_e10042;
        let (assign10220_e10048,) = {
    if (locals.var_alp1_p > 0.0) {
        (locals.var_alp1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1_i = assign10220_e10048;
        let (assign10230_e10054,) = {
    if (locals.var_alp2_p > 0.0) {
        (locals.var_alp2_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp2_i = assign10230_e10054;
        locals.var_vp_i = locals.var_vp_p;
        let (assign10250_e10061,) = {
    if (locals.var_a1_p > 0.0) {
        (locals.var_a1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a1_i = assign10250_e10061;
        locals.var_a2_i = locals.var_a2_p;
        locals.var_sta2_i = locals.var_sta2_p;
        let (assign10280_e10069,) = {
    if (locals.var_a3_p > 0.0) {
        (locals.var_a3_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a3_i = assign10280_e10069;
        let (assign10290_e10075,) = {
    if (locals.var_a4_p > 0.0) {
        (locals.var_a4_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a4_i = assign10290_e10075;
        let (assign10300_e10081,) = {
    if (locals.var_imaxii_p > 1e-12) {
        (locals.var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        locals.var_imaxii_i = assign10300_e10081;
        locals.var_gco_i = locals.var_gco_p;
        let (assign10320_e10088,) = {
    if (locals.var_iginv_p > 0.0) {
        (locals.var_iginv_p,)
    } else {
        (0.0,)
    }
};
        locals.var_iginv_i = assign10320_e10088;
        let (assign10330_e10094,) = {
    if (locals.var_igov_p > 0.0) {
        (locals.var_igov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igov_i = assign10330_e10094;
        let (assign10340_e10100,) = {
    if (locals.var_igovd_p > 0.0) {
        (locals.var_igovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igovd_i = assign10340_e10100;
        locals.var_stig_i = locals.var_stig_p;
        locals.var_gc2_i = locals.var_gc2_p;
        locals.var_gc3_i = locals.var_gc3_p;
        locals.var_gc2ov_i = locals.var_gc2ov_p;
        locals.var_gc3ov_i = locals.var_gc3ov_p;
        locals.var_gc2ovd_i = locals.var_gc2ovd_p;
        locals.var_gc3ovd_i = locals.var_gc3ovd_p;
        locals.var_chib_i = locals.var_chib_p;
        let (assign10430_e10114,) = {
    if (locals.var_agidl_p > 0.0) {
        (locals.var_agidl_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidl_i = assign10430_e10114;
        let (assign10440_e10120,) = {
    if (locals.var_agidld_p > 0.0) {
        (locals.var_agidld_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidld_i = assign10440_e10120;
        locals.var_bgidl_i = locals.var_bgidl_p;
        locals.var_bgidld_i = locals.var_bgidld_p;
        locals.var_stbgidl_i = locals.var_stbgidl_p;
        locals.var_stbgidld_i = locals.var_stbgidld_p;
        locals.var_cgidl_i = locals.var_cgidl_p;
        locals.var_cgidld_i = locals.var_cgidld_p;
        let (assign10510_e10132,) = {
    if (locals.var_cox_p > 0.0) {
        (locals.var_cox_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cox_i = assign10510_e10132;
        locals.var_delvtac_i = locals.var_delvtac_p;
        let (assign10530_e10139,) = {
    if (locals.var_facneffac_p > 0.0) {
        (locals.var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_facneffac_i = assign10530_e10139;
        let (assign10540_e10145,) = {
    if (locals.var_thesatac_p > 0.0) {
        (locals.var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesatac_i = assign10540_e10145;
        let (assign10550_e10151,) = {
    if (locals.var_axac_p > 2.0) {
        (locals.var_axac_p,)
    } else {
        (2.0,)
    }
};
        locals.var_axac_i = assign10550_e10151;
        locals.var_alpac_i = locals.var_alpac_p;
        let (assign10570_e10158,) = {
    if (locals.var_alp1ac_p > 0.0) {
        (locals.var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1ac_i = assign10570_e10158;
        let (assign10580_e10164,) = {
    if (locals.var_cgov_p > 0.0) {
        (locals.var_cgov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgov_i = assign10580_e10164;
        let (assign10590_e10170,) = {
    if (locals.var_cgovd_p > 0.0) {
        (locals.var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgovd_i = assign10590_e10170;
        locals.var_fcgovacc_i = locals.var_fcgovacc_p;
        locals.var_fcgovaccd_i = locals.var_fcgovaccd_p;
        locals.var_cgovaccg_i = locals.var_cgovaccg_p;
        let (assign10630_e10179,) = {
    if (locals.var_cgbov_p > 0.0) {
        (locals.var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgbov_i = assign10630_e10179;
        let (assign10640_e10185,) = {
    if (locals.var_cinr_p > 0.0) {
        (locals.var_cinr_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinr_i = assign10640_e10185;
        let (assign10650_e10191,) = {
    if (locals.var_cinrd_p > 0.0) {
        (locals.var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinrd_i = assign10650_e10191;
        locals.var_dvfbinr_i = locals.var_dvfbinr_p;
        locals.var_fcinrdep_i = locals.var_fcinrdep_p;
        locals.var_fcinracc_i = locals.var_fcinracc_p;
        locals.var_axinr_i = locals.var_axinr_p;
        let (assign10700_e10201,) = {
    if (locals.var_cfr_p > 0.0) {
        (locals.var_cfr_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfr_i = assign10700_e10201;
        let (assign10710_e10207,) = {
    if (locals.var_cfrd_p > 0.0) {
        (locals.var_cfrd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfrd_i = assign10710_e10207;
        locals.var_fnt_i = locals.var_fnt_p;
        let (assign10730_e10214,) = {
    if (locals.var_fntexc_p > 0.0) {
        (locals.var_fntexc_p,)
    } else {
        (0.0,)
    }
};
        locals.var_fntexc_i = assign10730_e10214;
        locals.var_vfbedge_i = locals.var_vfbedge_p;
        locals.var_stvfbedge_i = locals.var_stvfbedge_p;
        locals.var_dphibedge_i = locals.var_dphibedge_p;
        let (assign10810_e10247,) = {
    if (locals.var_neffedge_p > 1e20) {
        let (assign10810_e10245,) = {
            if (locals.var_neffedge_p < 1e26) {
                (locals.var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10810_e10245,)
    } else {
        (1e20,)
    }
};
        locals.var_neffedge_i = assign10810_e10247;
        let (assign10820_e10253,) = {
    if (locals.var_ctedge_p > 0.0) {
        (locals.var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ctedge_i = assign10820_e10253;
        let (assign10830_e10259,) = {
    if (locals.var_betnedge_p > 0.0) {
        (locals.var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betnedge_i = assign10830_e10259;
        locals.var_stbetedge_i = locals.var_stbetedge_p;
        let (assign10850_e10266,) = {
    if (locals.var_psceedge_p > 0.0) {
        (locals.var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psceedge_i = assign10850_e10266;
        let (assign10860_e10277,) = {
    if (locals.var_pscebedge_p > 0.0) {
        let (assign10860_e10275,) = {
            if (locals.var_pscebedge_p < 1.0) {
                (locals.var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10860_e10275,)
    } else {
        (0.0,)
    }
};
        locals.var_pscebedge_i = assign10860_e10277;
        let (assign10870_e10283,) = {
    if (locals.var_pscededge_p > 0.0) {
        (locals.var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_pscededge_i = assign10870_e10283;
    }
    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10880_e10289,) = {
    if (locals.var_cfedge_p > 0.0) {
        (locals.var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfedge_i = assign10880_e10289;
        let (assign10890_e10300,) = {
    if (locals.var_cfbedge_p > 0.0) {
        let (assign10890_e10298,) = {
            if (locals.var_cfbedge_p < 1.0) {
                (locals.var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10890_e10298,)
    } else {
        (0.0,)
    }
};
        locals.var_cfbedge_i = assign10890_e10300;
        let (assign10900_e10306,) = {
    if (locals.var_cfdedge_p > 0.0) {
        (locals.var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfdedge_i = assign10900_e10306;
        let (assign10960_e10332,) = {
    if (locals.var_rg_p > 0.0) {
        (locals.var_rg_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rg_i = assign10960_e10332;
        locals.var_rse_i = locals.var_rse_p;
        locals.var_rde_i = locals.var_rde_p;
        locals.var_rbulk_i = locals.var_rbulk_p;
        locals.var_rjuns_i = locals.var_rjuns_p;
        locals.var_rjund_i = locals.var_rjund_p;
        locals.var_rwell_i = locals.var_rwell_p;
        let assign11030_e10341: f64 = (p.p31 * locals.var_nf_i);
        let (assign11030_e10348,) = {
    if (assign11030_e10341 > 0.0) {
        let assign11030_e10346: f64 = (p.p31 * locals.var_nf_i);
        (assign11030_e10346,)
    } else {
        (0.0,)
    }
};
        locals.var_mult_inst = assign11030_e10348;
        locals.var_factuo_i = p.p16;
        locals.var_delvto_i = p.p15;
        locals.var_factuoedge_i = p.p18;
        locals.var_delvtoedge_i = p.p17;
        let (assign11080_e10358,) = {
    if (locals.var_munqs_p > 0.0) {
        (locals.var_munqs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_munqs_i = assign11080_e10358;
        let assign11090_e10361: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign11090_e10361;
        if (locals.var_guard153 != 0.0) {
            locals.var_toxovd_i = locals.var_toxov_i;
            locals.var_novd_i = locals.var_nov_i;
            locals.var_agidld_i = locals.var_agidl_i;
            locals.var_bgidld_i = locals.var_bgidl_i;
            locals.var_stbgidld_i = locals.var_stbgidl_i;
            locals.var_cgidld_i = locals.var_cgidl_i;
            locals.var_igovd_i = locals.var_igov_i;
            locals.var_gc2ovd_i = locals.var_gc2ov_i;
            locals.var_gc3ovd_i = locals.var_gc3ov_i;
            locals.var_cgovd_i = locals.var_cgov_i;
            locals.var_fcgovaccd_i = locals.var_fcgovacc_i;
            locals.var_cinrd_i = locals.var_cinr_i;
            locals.var_cfrd_i = locals.var_cfr_i;
        }
        let assign11230_e10416: f64 = (8.8541878176e-12 * locals.var_epsrox_i);
        locals.var_epsox = assign11230_e10416;
        let assign11240_e10419: f64 = (locals.var_epsox / locals.var_tox_i);
        locals.var_coxprime = assign11240_e10419;
        let assign11250_e10422: f64 = (locals.var_tox_i * locals.var_tox_i);
        locals.var_tox_sq = assign11250_e10422;
        let assign11260_e10425: f64 = (locals.var_coxprime / 1.6021918e-19);
        locals.var_cox_over_q = assign11260_e10425;
        let assign11270_e10428: f64 = (locals.var_facneffac_i * locals.var_neff_i);
        locals.var_neffac_i = assign11270_e10428;
        let (assign11280_e10439,) = {
    if (locals.var_neffac_i > 1e20) {
        let (assign11280_e10437,) = {
            if (locals.var_neffac_i < 1e26) {
                (locals.var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11280_e10437,)
    } else {
        (1e20,)
    }
};
        locals.var_neffac_i = assign11280_e10439;
        locals.var_qq = 0.0;
        let assign11300_e10443: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign11300_e10443;
        if (locals.var_guard154 != 0.0) {
            let assign11310_e10447: f64 = (0.4 * 5.951993);
            let assign11310_e10449: f64 = (assign11310_e10447 * p.p52);
            let assign11310_e10452: f64 = (locals.var_coxprime).powf(0.6666666666666666);
            let assign11310_e10453: f64 = (assign11310_e10449 * assign11310_e10452);
            locals.var_qq = assign11310_e10453;
        }
        let assign11320_e10458: f64 = (-1.0);
        let assign11320_e10459: f64 = if locals.var_chnl_type == assign11320_e10458 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign11320_e10459;
        if ((locals.var_guard154 != 0.0) && (locals.var_guard155 != 0.0)) {
            let assign11330_e10465: f64 = (7.448711 / 5.951993);
            let assign11330_e10467: f64 = (assign11330_e10465 * locals.var_qq);
            locals.var_qq = assign11330_e10467;
        }
        let assign11340_e10472: f64 = (1e-8 * locals.var_coxprime);
        let assign11340_e10474: f64 = (assign11340_e10472 / locals.var_epssi);
        locals.var_e_eff0 = assign11340_e10474;
        let assign11350_e10477: f64 = (0.5 * locals.var_feta_i);
        locals.var_eta_mu = assign11350_e10477;
        locals.var_eta_mu1 = 0.5;
        let assign11370_e10481: f64 = (-1.0);
        let assign11370_e10482: f64 = if locals.var_chnl_type == assign11370_e10481 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign11370_e10482;
        if (locals.var_guard156 != 0.0) {
            let assign11380_e10486: f64 = (0.3333333333333333 * locals.var_feta_i);
            locals.var_eta_mu = assign11380_e10486;
        }
        if (locals.var_guard156 != 0.0) {
            locals.var_eta_mu1 = 0.3333333333333333;
        }
        let assign11400_e10495: f64 = (-2.0);
        let assign11400_e10497: f64 = (assign11400_e10495 / locals.var_ax_i);
        let assign11400_e10499: f64 = (assign11400_e10497 + 1.0);
        let assign11400_e10500: f64 = (2.0_f64).powf(assign11400_e10499);
        let assign11400_e10502: f64 = (assign11400_e10500 - 1.0);
        locals.var_temp = assign11400_e10502;
        let assign11410_e10505: f64 = (locals.var_temp - 1.0);
        let assign11410_e10508: f64 = (locals.var_temp - 1.0);
        let assign11410_e10509: f64 = (assign11410_e10505 * assign11410_e10508);
        let assign11410_e10512: f64 = (4.0 * locals.var_temp);
        let (assign11410_e10519,) = {
    if (assign11410_e10512 > 0.0001) {
        let assign11410_e10517: f64 = (4.0 * locals.var_temp);
        (assign11410_e10517,)
    } else {
        (0.0001,)
    }
};
        let assign11410_e10520: f64 = (assign11410_e10509 / assign11410_e10519);
        locals.var_ar = assign11410_e10520;
        let assign11420_e10523: f64 = (-2.0);
        let assign11420_e10525: f64 = (assign11420_e10523 / locals.var_axac_i);
        let assign11420_e10527: f64 = (assign11420_e10525 + 1.0);
        let assign11420_e10528: f64 = (2.0_f64).powf(assign11420_e10527);
        let assign11420_e10530: f64 = (assign11420_e10528 - 1.0);
        locals.var_temp = assign11420_e10530;
        let assign11430_e10533: f64 = (locals.var_temp - 1.0);
        let assign11430_e10536: f64 = (locals.var_temp - 1.0);
        let assign11430_e10537: f64 = (assign11430_e10533 * assign11430_e10536);
        let assign11430_e10540: f64 = (4.0 * locals.var_temp);
        let (assign11430_e10547,) = {
    if (assign11430_e10540 > 0.0001) {
        let assign11430_e10545: f64 = (4.0 * locals.var_temp);
        (assign11430_e10545,)
    } else {
        (0.0001,)
    }
};
        let assign11430_e10548: f64 = (assign11430_e10537 / assign11430_e10547);
        locals.var_arac = assign11430_e10548;
        let assign11440_e10551: f64 = (1.0 / locals.var_vp_i);
        locals.var_inv_vp = assign11440_e10551;
        let assign11450_e10554: f64 = (locals.var_epsox / locals.var_toxov_i);
        locals.var_coxovprime = assign11450_e10554;
        let assign11460_e10557: f64 = (locals.var_epsox / locals.var_toxovd_i);
        locals.var_coxovprime_d = assign11460_e10557;
        let assign11470_e10560: f64 = (2.0 * 1.6021918e-19);
        let assign11470_e10562: f64 = (assign11470_e10560 * locals.var_nov_i);
        let assign11470_e10564: f64 = (assign11470_e10562 * locals.var_epssi);
        let assign11470_e10566: f64 = (assign11470_e10564 * locals.var_inv_phita);
        let assign11470_e10567: f64 = (assign11470_e10566).sqrt();
        let assign11470_e10569: f64 = (assign11470_e10567 / locals.var_coxovprime);
        locals.var_gov_s = assign11470_e10569;
        let assign11480_e10572: f64 = (2.0 * 1.6021918e-19);
        let assign11480_e10574: f64 = (assign11480_e10572 * locals.var_novd_i);
        let assign11480_e10576: f64 = (assign11480_e10574 * locals.var_epssi);
        let assign11480_e10578: f64 = (assign11480_e10576 * locals.var_inv_phita);
        let assign11480_e10579: f64 = (assign11480_e10578).sqrt();
        let assign11480_e10581: f64 = (assign11480_e10579 / locals.var_coxovprime_d);
        locals.var_gov_d = assign11480_e10581;
        let assign11490_e10584: f64 = (locals.var_gov_s * locals.var_gov_s);
        locals.var_gov2_s = assign11490_e10584;
        let assign11500_e10587: f64 = (locals.var_gov_d * locals.var_gov_d);
        locals.var_gov2_d = assign11500_e10587;
        let assign11510_e10590: f64 = (locals.var_cgovaccg_i * 0.005);
        let assign11510_e10592: f64 = (assign11510_e10590 * locals.var_inv_phita);
        let assign11510_e10593: f64 = (assign11510_e10592).exp();
        let assign11510_e10595: f64 = (assign11510_e10593 - 1.0);
        let assign11510_e10596: f64 = (assign11510_e10595).ln();
        let assign11510_e10598: f64 = (assign11510_e10596 / locals.var_cgovaccg_i);
        let assign11510_e10601: f64 = (0.005 * locals.var_inv_phita);
        let assign11510_e10602: f64 = (assign11510_e10601).exp();
        let assign11510_e10604: f64 = (assign11510_e10602 - 1.0);
        let assign11510_e10605: f64 = (assign11510_e10604).ln();
        let assign11510_e10606: f64 = (assign11510_e10598 - assign11510_e10605);
        locals.var_dxgb_ov_th = assign11510_e10606;
        let assign11520_e10609: f64 = (0.5 * locals.var_gov_s);
        let assign11520_e10610: f64 = (assign11520_e10609).ln();
        let assign11520_e10612: f64 = (assign11520_e10610 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_s = assign11520_e10612;
        let assign11530_e10615: f64 = (0.5 * locals.var_gov_d);
        let assign11530_e10616: f64 = (assign11530_e10615).ln();
        let assign11530_e10618: f64 = (assign11530_e10616 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_d = assign11530_e10618;
        let assign11540_e10621: f64 = (1.0 / locals.var_gov_s);
        locals.var_inv_gov = assign11540_e10621;
        let assign11550_e10624: f64 = (3.1 * locals.var_gov_s);
        let assign11550_e10626: f64 = (assign11550_e10624 + 8.5);
        locals.var_sp_ov_eps = assign11550_e10626;
        let assign11560_e10629: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_s = assign11560_e10629;
        let assign11570_e10632: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11570_e10632;
        let assign11580_e10635: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign11580_e10635;
        if (locals.var_guard157 != 0.0) {
            let assign11590_e10639: f64 = (64.0 * locals.var_inv_gov);
            locals.var_sp_ov_a_s = assign11590_e10639;
        }
        let assign11600_e10644: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign11600_e10644;
        if ((locals.var_guard157 == 0.0) && (locals.var_guard158 != 0.0)) {
            let assign11610_e10651: f64 = (22.0 * locals.var_inv_gov);
            let assign11610_e10653: f64 = (assign11610_e10651 + 3.0);
            locals.var_sp_ov_a_s = assign11610_e10653;
        }
        let assign11620_e10658: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign11620_e10658;
        if (((locals.var_guard157 == 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
            let assign11630_e10667: f64 = (-7.2);
            let assign11630_e10669: f64 = (assign11630_e10667 * locals.var_inv_gov);
            let assign11630_e10671: f64 = (assign11630_e10669 + 15.5);
            locals.var_sp_ov_a_s = assign11630_e10671;
        }
        if (((locals.var_guard157 == 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) {
            locals.var_sp_ov_a_s = locals.var_gov_s;
        }
        let assign11650_e10688: f64 = (locals.var_gov2_s * 0.5);
        let assign11650_e10689: f64 = (locals.var_sp_ov_delta + assign11650_e10688);
        let assign11650_e10694: f64 = (locals.var_gov2_s * 0.25);
        let assign11650_e10695: f64 = (locals.var_sp_ov_delta + assign11650_e10694);
        let assign11650_e10697: f64 = (assign11650_e10695 + locals.var_sp_ov_a_s);
        let assign11650_e10698: f64 = (assign11650_e10697).sqrt();
        let assign11650_e10699: f64 = (locals.var_gov_s * assign11650_e10698);
        let assign11650_e10700: f64 = (assign11650_e10689 - assign11650_e10699);
        locals.var_sp_ov_delta1_s = assign11650_e10700;
        let assign11660_e10703: f64 = (1.0 / locals.var_gov_d);
        locals.var_inv_gov = assign11660_e10703;
        let assign11670_e10706: f64 = (3.1 * locals.var_gov_d);
        let assign11670_e10708: f64 = (assign11670_e10706 + 8.5);
        locals.var_sp_ov_eps = assign11670_e10708;
        let assign11680_e10711: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_d = assign11680_e10711;
        let assign11690_e10714: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11690_e10714;
        let assign11700_e10717: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign11700_e10717;
        if (locals.var_guard160 != 0.0) {
            let assign11710_e10721: f64 = (64.0 * locals.var_inv_gov);
            locals.var_sp_ov_a_d = assign11710_e10721;
        }
        let assign11720_e10726: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign11720_e10726;
        if ((locals.var_guard160 == 0.0) && (locals.var_guard161 != 0.0)) {
            let assign11730_e10733: f64 = (22.0 * locals.var_inv_gov);
            let assign11730_e10735: f64 = (assign11730_e10733 + 3.0);
            locals.var_sp_ov_a_d = assign11730_e10735;
        }
        let assign11740_e10740: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign11740_e10740;
        if (((locals.var_guard160 == 0.0) && (locals.var_guard161 == 0.0)) && (locals.var_guard162 != 0.0)) {
            let assign11750_e10749: f64 = (-7.2);
            let assign11750_e10751: f64 = (assign11750_e10749 * locals.var_inv_gov);
            let assign11750_e10753: f64 = (assign11750_e10751 + 15.5);
            locals.var_sp_ov_a_d = assign11750_e10753;
        }
        if (((locals.var_guard160 == 0.0) && (locals.var_guard161 == 0.0)) && (locals.var_guard162 == 0.0)) {
            locals.var_sp_ov_a_d = locals.var_gov_d;
        }
        let assign11770_e10770: f64 = (locals.var_gov2_d * 0.5);
        let assign11770_e10771: f64 = (locals.var_sp_ov_delta + assign11770_e10770);
        let assign11770_e10776: f64 = (locals.var_gov2_d * 0.25);
        let assign11770_e10777: f64 = (locals.var_sp_ov_delta + assign11770_e10776);
        let assign11770_e10779: f64 = (assign11770_e10777 + locals.var_sp_ov_a_d);
        let assign11770_e10780: f64 = (assign11770_e10779).sqrt();
        let assign11770_e10781: f64 = (locals.var_gov_d * assign11770_e10780);
        let assign11770_e10782: f64 = (assign11770_e10771 - assign11770_e10781);
        locals.var_sp_ov_delta1_d = assign11770_e10782;
        let assign11780_e10785: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign11780_e10788: f64 = (2.0 * locals.var_phit);
        let assign11780_e10792: f64 = (-0.75);
        let assign11780_e10793: f64 = (locals.var_phibfac).powf(assign11780_e10792);
        let assign11780_e10794: f64 = (locals.var_neff_i * assign11780_e10793);
        let assign11780_e10796: f64 = (assign11780_e10794 * 4e-26);
        let assign11780_e10797: f64 = (assign11780_e10796).ln();
        let assign11780_e10798: f64 = (assign11780_e10788 * assign11780_e10797);
        let assign11780_e10799: f64 = (assign11780_e10785 + assign11780_e10798);
        locals.var_phib_dc = assign11780_e10799;
        if (!(locals.var_phib_dc > 0.05)) {
            locals.var_phib_dc = 0.05;
        }
        let assign11800_e10808: f64 = (2.0 * 1.6021918e-19);
        let assign11800_e10810: f64 = (assign11800_e10808 * locals.var_neff_i);
        let assign11800_e10812: f64 = (assign11800_e10810 * locals.var_epssi);
        let assign11800_e10814: f64 = (assign11800_e10812 * locals.var_inv_phit);
        let assign11800_e10815: f64 = (assign11800_e10814).sqrt();
        let assign11800_e10817: f64 = (assign11800_e10815 / locals.var_coxprime);
        locals.var_g_0_dc = assign11800_e10817;
        locals.var_kp = 0.0;
        locals.var_np = 0.0;
        let assign11830_e10822: f64 = if locals.var_np_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign11830_e10822;
        if (locals.var_guard163 != 0.0) {
            let assign11840_e10826: f64 = (80000000.0 / locals.var_tox_sq);
            locals.var_arg2max = assign11840_e10826;
        }
        if (locals.var_guard163 != 0.0) {
            let (assign11850_e10835,) = {
    if (locals.var_np_i > locals.var_arg2max) {
        (locals.var_np_i,)
    } else {
        (locals.var_arg2max,)
    }
};
            locals.var_np = assign11850_e10835;
        }
        if (locals.var_guard163 != 0.0) {
            let (assign11860_e10844,) = {
    if (5e24 > locals.var_np) {
        (5e24,)
    } else {
        (locals.var_np,)
    }
};
            locals.var_np = assign11860_e10844;
        }
        if (locals.var_guard163 != 0.0) {
            let assign11870_e10850: f64 = (2.0 * locals.var_coxprime);
            let assign11870_e10852: f64 = (assign11870_e10850 * locals.var_coxprime);
            let assign11870_e10854: f64 = (assign11870_e10852 * locals.var_phit);
            let assign11870_e10857: f64 = (1.6021918e-19 * locals.var_np);
            let assign11870_e10859: f64 = (assign11870_e10857 * locals.var_epssi);
            let assign11870_e10860: f64 = (assign11870_e10854 / assign11870_e10859);
            locals.var_kp = assign11870_e10860;
        }
        let assign11880_e10865: f64 = (100.0 * locals.var_phit);
        let assign11880_e10867: f64 = (assign11880_e10865 * locals.var_phit);
        locals.var_qlim2 = assign11880_e10867;
        let assign11890_e10870: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard164 = assign11890_e10870;
        if (locals.var_guard164 != 0.0) {
            let assign11900_e10874: f64 = (locals.var_phit * locals.var_g_0_dc);
            let assign11900_e10876: f64 = (assign11900_e10874 * locals.var_g_0_dc);
            let assign11900_e10878: f64 = (assign11900_e10876 * locals.var_phib_dc);
            let assign11900_e10879: f64 = (assign11900_e10878).sqrt();
            locals.var_qb0 = assign11900_e10879;
        }
        if (locals.var_guard164 != 0.0) {
            let assign11910_e10885: f64 = (0.75 * locals.var_qq);
            let assign11910_e10888: f64 = (locals.var_qb0).powf(0.6666666666666666);
            let assign11910_e10889: f64 = (assign11910_e10885 * assign11910_e10888);
            locals.var_dphibq = assign11910_e10889;
        }
    }
    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard164 != 0.0) {
            let assign11920_e10895: f64 = (locals.var_phib_dc + locals.var_dphibq);
            locals.var_phib_dc = assign11920_e10895;
        }
        if (locals.var_guard164 != 0.0) {
            let assign11930_e10903: f64 = (2.0 * 0.6666666666666666);
            let assign11930_e10905: f64 = (assign11930_e10903 * locals.var_dphibq);
            let assign11930_e10907: f64 = (assign11930_e10905 / locals.var_qb0);
            let assign11930_e10908: f64 = (1.0 + assign11930_e10907);
            let assign11930_e10909: f64 = (locals.var_g_0_dc * assign11930_e10908);
            locals.var_g_0_dc = assign11930_e10909;
        }
        let assign11940_e10913: f64 = (locals.var_phib_dc).sqrt();
        locals.var_sqrt_phib_dc = assign11940_e10913;
        let assign11950_e10916: f64 = (0.95 * locals.var_phib_dc);
        locals.var_phix_dc = assign11950_e10916;
        let assign11960_e10919: f64 = (0.0025 * locals.var_phib_dc);
        let assign11960_e10921: f64 = (assign11960_e10919 * locals.var_phib_dc);
        locals.var_aphi_dc = assign11960_e10921;
        locals.var_bphi_dc = locals.var_aphi_dc;
        let assign11980_e10925: f64 = (locals.var_bphi_dc).sqrt();
        let assign11980_e10926: f64 = (0.5 * assign11980_e10925);
        locals.var_phix2 = assign11980_e10926;
        let assign11990_e10930: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11990_e10932: f64 = assign11990_e10930;
        let assign11990_e10935: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11990_e10937: f64 = assign11990_e10935;
        let assign11990_e10940: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11990_e10942: f64 = assign11990_e10940;
        let assign11990_e10943: f64 = (assign11990_e10937 * assign11990_e10942);
        let assign11990_e10945: f64 = (assign11990_e10943 + locals.var_aphi_dc);
        let assign11990_e10946: f64 = (assign11990_e10945).sqrt();
        let assign11990_e10947: f64 = (assign11990_e10932 - assign11990_e10946);
        let assign11990_e10948: f64 = (0.5 * assign11990_e10947);
        locals.var_phix1_dc = assign11990_e10948;
        let assign12000_e10952: f64 = (locals.var_phib_dc + locals.var_eg);
        let assign12000_e10953: f64 = (0.5 * assign12000_e10952);
        locals.var_alpha_b = assign12000_e10953;
        let assign12010_e10956: f64 = (locals.var_vsbnud_i + locals.var_phib_dc);
        let assign12010_e10957: f64 = (assign12010_e10956).sqrt();
        let assign12010_e10959: f64 = (assign12010_e10957 - locals.var_sqrt_phib_dc);
        locals.var_us1 = assign12010_e10959;
        let assign12020_e10962: f64 = (locals.var_vsbnud_i + locals.var_dvsbnud_i);
        let assign12020_e10964: f64 = (assign12020_e10962 + locals.var_phib_dc);
        let assign12020_e10965: f64 = (assign12020_e10964).sqrt();
        let assign12020_e10967: f64 = (assign12020_e10965 - locals.var_sqrt_phib_dc);
        let assign12020_e10969: f64 = (assign12020_e10967 - locals.var_us1);
        locals.var_us21 = assign12020_e10969;
        let assign12030_e10972: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign12030_e10974: f64 = (assign12030_e10972 + locals.var_delvtac_i);
        let assign12030_e10977: f64 = (2.0 * locals.var_phit);
        let assign12030_e10981: f64 = (-0.75);
        let assign12030_e10982: f64 = (locals.var_phibfac).powf(assign12030_e10981);
        let assign12030_e10983: f64 = (locals.var_neffac_i * assign12030_e10982);
        let assign12030_e10985: f64 = (assign12030_e10983 * 4e-26);
        let assign12030_e10986: f64 = (assign12030_e10985).ln();
        let assign12030_e10987: f64 = (assign12030_e10977 * assign12030_e10986);
        let assign12030_e10988: f64 = (assign12030_e10974 + assign12030_e10987);
        locals.var_phib_ac = assign12030_e10988;
        if (!(locals.var_phib_ac > 0.05)) {
            locals.var_phib_ac = 0.05;
        }
        let assign12050_e10997: f64 = (2.0 * 1.6021918e-19);
        let assign12050_e10999: f64 = (assign12050_e10997 * locals.var_neffac_i);
        let assign12050_e11001: f64 = (assign12050_e10999 * locals.var_epssi);
        let assign12050_e11003: f64 = (assign12050_e11001 * locals.var_inv_phit);
        let assign12050_e11004: f64 = (assign12050_e11003).sqrt();
        let assign12050_e11006: f64 = (assign12050_e11004 / locals.var_coxprime);
        locals.var_g_0_ac = assign12050_e11006;
        let assign12060_e11009: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign12060_e11009;
        if (locals.var_guard165 != 0.0) {
            let assign12070_e11013: f64 = (locals.var_phit * locals.var_g_0_ac);
            let assign12070_e11015: f64 = (assign12070_e11013 * locals.var_g_0_ac);
            let assign12070_e11017: f64 = (assign12070_e11015 * locals.var_phib_ac);
            let assign12070_e11018: f64 = (assign12070_e11017).sqrt();
            locals.var_qb0 = assign12070_e11018;
        }
        if (locals.var_guard165 != 0.0) {
            let assign12080_e11024: f64 = (0.75 * locals.var_qq);
            let assign12080_e11027: f64 = (locals.var_qb0).powf(0.6666666666666666);
            let assign12080_e11028: f64 = (assign12080_e11024 * assign12080_e11027);
            locals.var_dphibq = assign12080_e11028;
        }
        if (locals.var_guard165 != 0.0) {
            let assign12090_e11034: f64 = (locals.var_phib_ac + locals.var_dphibq);
            locals.var_phib_ac = assign12090_e11034;
        }
        if (locals.var_guard165 != 0.0) {
            let assign12100_e11042: f64 = (2.0 * 0.6666666666666666);
            let assign12100_e11044: f64 = (assign12100_e11042 * locals.var_dphibq);
            let assign12100_e11046: f64 = (assign12100_e11044 / locals.var_qb0);
            let assign12100_e11047: f64 = (1.0 + assign12100_e11046);
            let assign12100_e11048: f64 = (locals.var_g_0_ac * assign12100_e11047);
            locals.var_g_0_ac = assign12100_e11048;
        }
        let assign12110_e11053: f64 = (0.95 * locals.var_phib_ac);
        locals.var_phix_ac = assign12110_e11053;
        let assign12120_e11056: f64 = (0.0025 * locals.var_phib_ac);
        let assign12120_e11058: f64 = (assign12120_e11056 * locals.var_phib_ac);
        locals.var_aphi_ac = assign12120_e11058;
        locals.var_bphi_ac = locals.var_aphi_ac;
        let assign12140_e11062: f64 = (locals.var_bphi_ac).sqrt();
        let assign12140_e11063: f64 = (0.5 * assign12140_e11062);
        locals.var_phix2 = assign12140_e11063;
        let assign12150_e11067: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign12150_e11069: f64 = assign12150_e11067;
        let assign12150_e11072: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign12150_e11074: f64 = assign12150_e11072;
        let assign12150_e11077: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign12150_e11079: f64 = assign12150_e11077;
        let assign12150_e11080: f64 = (assign12150_e11074 * assign12150_e11079);
        let assign12150_e11082: f64 = (assign12150_e11080 + locals.var_aphi_ac);
        let assign12150_e11083: f64 = (assign12150_e11082).sqrt();
        let assign12150_e11084: f64 = (assign12150_e11069 - assign12150_e11083);
        let assign12150_e11085: f64 = (0.5 * assign12150_e11084);
        locals.var_phix1_ac = assign12150_e11085;
        let assign12160_e11089: f64 = (locals.var_stvfb_i * locals.var_delt);
        let assign12160_e11093: f64 = (locals.var_st2vfb_i * locals.var_delt);
        let assign12160_e11094: f64 = (1.0 + assign12160_e11093);
        let assign12160_e11095: f64 = (assign12160_e11089 * assign12160_e11094);
        let assign12160_e11096: f64 = (locals.var_vfb_i + assign12160_e11095);
        let assign12160_e11098: f64 = (assign12160_e11096 + locals.var_delvto_i);
        locals.var_vfb_t = assign12160_e11098;
        let assign12170_e11101: f64 = (locals.var_stct_i * locals.var_ln_rtn);
        let assign12170_e11102: f64 = (assign12170_e11101).exp();
        locals.var_tf_ct = assign12170_e11102;
        let assign12180_e11105: f64 = (locals.var_ct_i * locals.var_tf_ct);
        locals.var_ct_t = assign12180_e11105;
        let assign12190_e11108: f64 = (locals.var_ctg_i / locals.var_rtn);
        locals.var_ctg_t = assign12190_e11108;
        let assign12200_e11111: f64 = (locals.var_stbet_i * locals.var_ln_rtn);
        let assign12200_e11112: f64 = (assign12200_e11111).exp();
        locals.var_tf_bet = assign12200_e11112;
        let assign12210_e11115: f64 = (locals.var_betn_i * locals.var_tf_bet);
        locals.var_betn_t = assign12210_e11115;
        let assign12220_e11118: f64 = (locals.var_factuo_i * locals.var_betn_t);
        let assign12220_e11120: f64 = (assign12220_e11118 * locals.var_coxprime);
        locals.var_bet_i = assign12220_e11120;
        let assign12230_e11124: f64 = (locals.var_stthemu_i * locals.var_ln_rtn);
        let assign12230_e11125: f64 = (assign12230_e11124).exp();
        let assign12230_e11126: f64 = (locals.var_themu_i * assign12230_e11125);
        locals.var_themu_t = assign12230_e11126;
        let assign12240_e11129: f64 = (locals.var_stmue_i * locals.var_ln_rtn);
        let assign12240_e11130: f64 = (assign12240_e11129).exp();
        locals.var_tf_mue = assign12240_e11130;
        let assign12250_e11133: f64 = (locals.var_mue_i * locals.var_tf_mue);
        locals.var_mue_t = assign12250_e11133;
        let assign12260_e11137: f64 = (locals.var_stthecs_i * locals.var_ln_rtn);
        let assign12260_e11138: f64 = (assign12260_e11137).exp();
        let assign12260_e11139: f64 = (locals.var_thecs_i * assign12260_e11138);
        locals.var_thecs_t = assign12260_e11139;
        let assign12270_e11142: f64 = (locals.var_stcs_i * locals.var_ln_rtn);
        let assign12270_e11143: f64 = (assign12270_e11142).exp();
        locals.var_tf_cs = assign12270_e11143;
        let assign12280_e11146: f64 = (locals.var_cs_i * locals.var_tf_cs);
        locals.var_cs_t = assign12280_e11146;
        let assign12290_e11149: f64 = (locals.var_stxcor_i * locals.var_ln_rtn);
        let assign12290_e11150: f64 = (assign12290_e11149).exp();
        locals.var_tf_xcor = assign12290_e11150;
        let assign12300_e11153: f64 = (locals.var_xcor_i * locals.var_tf_xcor);
        locals.var_xcor_t = assign12300_e11153;
        let assign12310_e11156: f64 = (locals.var_strs_i * locals.var_ln_rtn);
        let assign12310_e11157: f64 = (assign12310_e11156).exp();
        locals.var_tf_ther = assign12310_e11157;
        let assign12320_e11160: f64 = (locals.var_rs_i * locals.var_tf_ther);
        locals.var_rs_t = assign12320_e11160;
        let assign12330_e11163: f64 = (2.0 * locals.var_bet_i);
        let assign12330_e11165: f64 = (assign12330_e11163 * locals.var_rs_t);
        locals.var_ther_i = assign12330_e11165;
        let assign12340_e11168: f64 = (locals.var_stthesat_i * locals.var_ln_rtn);
        let assign12340_e11169: f64 = (assign12340_e11168).exp();
        locals.var_tf_thesat = assign12340_e11169;
        let assign12350_e11172: f64 = (locals.var_thesat_i * locals.var_tf_thesat);
        locals.var_thesat_t = assign12350_e11172;
        let assign12360_e11175: f64 = (locals.var_thesatac_i * locals.var_tf_thesat);
        locals.var_thesatac_t = assign12360_e11175;
        let assign12370_e11178: f64 = (-locals.var_sta2_i);
        let assign12370_e11180: f64 = (assign12370_e11178 * locals.var_ln_rtn);
        let assign12370_e11181: f64 = (assign12370_e11180).exp();
        let assign12370_e11182: f64 = (locals.var_a2_i * assign12370_e11181);
        locals.var_a2_t = assign12370_e11182;
        let assign12380_e11185: f64 = (locals.var_fnt_i * 4.0);
        let assign12380_e11187: f64 = (assign12380_e11185 * 1.3806505e-23);
        let assign12380_e11189: f64 = (assign12380_e11187 * locals.var_tkd);
        locals.var_nt = assign12380_e11189;
        let assign12400_e11203: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard166 = assign12400_e11203;
        if (locals.var_guard166 != 0.0) {
            let assign12410_e11208: f64 = (locals.var_stvfbedge_i * locals.var_delt);
            let assign12410_e11209: f64 = (locals.var_vfbedge_i + assign12410_e11208);
            let assign12410_e11211: f64 = (assign12410_e11209 + locals.var_delvtoedge_i);
            locals.var_vfbedge_t = assign12410_e11211;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12420_e11217: f64 = (locals.var_stbetedge_i * locals.var_ln_rtn);
            let assign12420_e11218: f64 = (assign12420_e11217).exp();
            locals.var_tf_betedge = assign12420_e11218;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12430_e11224: f64 = (locals.var_betnedge_i * locals.var_tf_betedge);
            locals.var_betnedge_t = assign12430_e11224;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12440_e11230: f64 = (locals.var_factuoedge_i * locals.var_betnedge_t);
            let assign12440_e11232: f64 = (assign12440_e11230 * locals.var_coxprime);
            locals.var_betedge_i = assign12440_e11232;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12450_e11240: f64 = (locals.var_ctedge_i * locals.var_rtn);
            let assign12450_e11241: f64 = (1.0 + assign12450_e11240);
            let assign12450_e11242: f64 = (locals.var_phit * assign12450_e11241);
            locals.var_phit0edge = assign12450_e11242;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12460_e11248: f64 = (locals.var_eg + locals.var_dphibedge_i);
            let assign12460_e11251: f64 = (2.0 * locals.var_phit0edge);
            let assign12460_e11255: f64 = (-0.75);
            let assign12460_e11256: f64 = (locals.var_phibfac).powf(assign12460_e11255);
            let assign12460_e11257: f64 = (locals.var_neffedge_i * assign12460_e11256);
            let assign12460_e11259: f64 = (assign12460_e11257 * 4e-26);
            let assign12460_e11260: f64 = (assign12460_e11259).ln();
            let assign12460_e11261: f64 = (assign12460_e11251 * assign12460_e11260);
            let assign12460_e11262: f64 = (assign12460_e11248 + assign12460_e11261);
            locals.var_phibedge = assign12460_e11262;
        }
        if (locals.var_guard166 != 0.0) {
            let (assign12470_e11271,) = {
    if (locals.var_phibedge > 0.05) {
        (locals.var_phibedge,)
    } else {
        (0.05,)
    }
};
            locals.var_phibedge = assign12470_e11271;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12480_e11277: f64 = (2.0 * 1.6021918e-19);
            let assign12480_e11279: f64 = (assign12480_e11277 * locals.var_neffedge_i);
            let assign12480_e11281: f64 = (assign12480_e11279 * locals.var_epssi);
            let assign12480_e11283: f64 = (assign12480_e11281 * locals.var_inv_phit);
            let assign12480_e11284: f64 = (assign12480_e11283).sqrt();
            let assign12480_e11286: f64 = (assign12480_e11284 / locals.var_coxprime);
            locals.var_gfedge = assign12480_e11286;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12490_e11292: f64 = (locals.var_gfedge * locals.var_gfedge);
            locals.var_gfedge2 = assign12490_e11292;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12500_e11297: f64 = (locals.var_gfedge2).ln();
            locals.var_lngfedge2 = assign12500_e11297;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12510_e11303: f64 = (0.95 * locals.var_phibedge);
            locals.var_phixedge = assign12510_e11303;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12520_e11309: f64 = (0.0025 * locals.var_phibedge);
            let assign12520_e11311: f64 = (assign12520_e11309 * locals.var_phibedge);
            locals.var_aphiedge = assign12520_e11311;
        }
        if (locals.var_guard166 != 0.0) {
            locals.var_bphiedge = locals.var_aphiedge;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12540_e11321: f64 = (locals.var_bphiedge).sqrt();
            let assign12540_e11322: f64 = (0.5 * assign12540_e11321);
            locals.var_phix2edge = assign12540_e11322;
        }
        if (locals.var_guard166 != 0.0) {
            let assign12550_e11329: f64 = (locals.var_phixedge - locals.var_phix2edge);
            let assign12550_e11331: f64 = assign12550_e11329;
            let assign12550_e11334: f64 = (locals.var_phixedge - locals.var_phix2edge);
            let assign12550_e11336: f64 = assign12550_e11334;
            let assign12550_e11339: f64 = (locals.var_phixedge - locals.var_phix2edge);
            let assign12550_e11341: f64 = assign12550_e11339;
            let assign12550_e11342: f64 = (assign12550_e11336 * assign12550_e11341);
            let assign12550_e11344: f64 = (assign12550_e11342 + locals.var_aphiedge);
            let assign12550_e11345: f64 = (assign12550_e11344).sqrt();
            let assign12550_e11346: f64 = (assign12550_e11331 - assign12550_e11345);
            let assign12550_e11347: f64 = (0.5 * assign12550_e11346);
            locals.var_phix1edge = assign12550_e11347;
        }
        if (locals.var_guard166 == 0.0) {
            locals.var_vfbedge_t = 0.0;
            locals.var_tf_betedge = 1.0;
            locals.var_betnedge_t = 0.0;
            locals.var_betedge_i = 0.0;
            locals.var_phit0edge = locals.var_phit;
            locals.var_phibedge = 0.0;
            locals.var_gfedge = 1.0;
            locals.var_gfedge2 = 1.0;
            locals.var_lngfedge2 = 0.0;
            locals.var_phixedge = 0.0;
            locals.var_aphiedge = 0.0;
            locals.var_bphiedge = 0.0;
            locals.var_phix2edge = 0.0;
            locals.var_phix1edge = 0.0;
        }
        let assign12740_e11452: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign12740_e11452;
        let assign12750_e11455: f64 = (4.0 * 0.3333333333333333);
        let assign12750_e11458: f64 = (2.0 * 1.6021918e-19);
        let assign12750_e11460: f64 = (assign12750_e11458 * 9.1093826e-31);
        let assign12750_e11462: f64 = (assign12750_e11460 * locals.var_chib_i);
        let assign12750_e11463: f64 = (assign12750_e11462).sqrt();
        let assign12750_e11464: f64 = (assign12750_e11455 * assign12750_e11463);
        let assign12750_e11466: f64 = (assign12750_e11464 / 1.05457168e-34);
        locals.var_b_fact = assign12750_e11466;
        let assign12760_e11469: f64 = (locals.var_b_fact * locals.var_tox_i);
        locals.var_bch = assign12760_e11469;
        let assign12770_e11472: f64 = (locals.var_b_fact * locals.var_toxov_i);
        locals.var_bov = assign12770_e11472;
        let assign12780_e11475: f64 = (locals.var_b_fact * locals.var_toxovd_i);
        locals.var_bov_d = assign12780_e11475;
        locals.var_gcq = 0.0;
        let assign12800_e11479: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign12800_e11479;
        if (locals.var_guard167 != 0.0) {
            let assign12810_e11482: f64 = (-0.495);
            let assign12810_e11484: f64 = (assign12810_e11482 * locals.var_gc2_i);
            let assign12810_e11486: f64 = (assign12810_e11484 / locals.var_gc3_i);
            locals.var_gcq = assign12810_e11486;
        }
        locals.var_gcqov = 0.0;
        let assign12830_e11492: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign12830_e11492;
        if (locals.var_guard168 != 0.0) {
            let assign12840_e11495: f64 = (-0.495);
            let assign12840_e11497: f64 = (assign12840_e11495 * locals.var_gc2ov_i);
            let assign12840_e11499: f64 = (assign12840_e11497 / locals.var_gc3ov_i);
            locals.var_gcqov = assign12840_e11499;
        }
        let assign12850_e11504: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign12850_e11504;
        if (locals.var_guard169 != 0.0) {
            let assign12860_e11507: f64 = (-0.495);
            let assign12860_e11509: f64 = (assign12860_e11507 * locals.var_gc2ovd_i);
            let assign12860_e11511: f64 = (assign12860_e11509 / locals.var_gc3ovd_i);
            locals.var_gcqovd = assign12860_e11511;
        }
        let assign12870_e11516: f64 = (locals.var_rta).powf(locals.var_stig_i);
        locals.var_tf_ig = assign12870_e11516;
        let assign12880_e11519: f64 = (locals.var_iginv_i * locals.var_tf_ig);
        locals.var_iginv_i = assign12880_e11519;
        let assign12890_e11522: f64 = (locals.var_igov_i * locals.var_tf_ig);
        locals.var_igov_i = assign12890_e11522;
        let assign12900_e11525: f64 = (locals.var_igovd_i * locals.var_tf_ig);
        locals.var_igovd_i = assign12900_e11525;
        let assign12910_e11528: f64 = (locals.var_agidl_i * 4e-18);
        let assign12910_e11531: f64 = (locals.var_toxov_i * locals.var_toxov_i);
        let assign12910_e11532: f64 = (assign12910_e11528 / assign12910_e11531);
        locals.var_agidls = assign12910_e11532;
        let assign12920_e11535: f64 = (locals.var_agidld_i * 4e-18);
        let assign12920_e11538: f64 = (locals.var_toxovd_i * locals.var_toxovd_i);
        let assign12920_e11539: f64 = (assign12920_e11535 / assign12920_e11538);
        locals.var_agidlds = assign12920_e11539;
        let assign12930_e11543: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign12930_e11544: f64 = (1.0 + assign12930_e11543);
        let (assign12930_e11553,) = {
    if (assign12930_e11544 > 0.0) {
        let assign12930_e11550: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign12930_e11551: f64 = (1.0 + assign12930_e11550);
        (assign12930_e11551,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign12930_e11553;
        let assign12940_e11556: f64 = (locals.var_bgidl_i * locals.var_b_fact);
        locals.var_bgidl_t = assign12940_e11556;
        let assign12950_e11559: f64 = (locals.var_bgidl_t * locals.var_toxov_i);
        let assign12950_e11561: f64 = (assign12950_e11559 * 500000000.0);
        locals.var_bgidls = assign12950_e11561;
    }
    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign12960_e11565: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign12960_e11566: f64 = (1.0 + assign12960_e11565);
        let (assign12960_e11575,) = {
    if (assign12960_e11566 > 0.0) {
        let assign12960_e11572: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign12960_e11573: f64 = (1.0 + assign12960_e11572);
        (assign12960_e11573,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign12960_e11575;
        let assign12970_e11578: f64 = (locals.var_bgidld_i * locals.var_b_fact);
        locals.var_bgidld_t = assign12970_e11578;
        let assign12980_e11581: f64 = (locals.var_bgidld_t * locals.var_toxovd_i);
        let assign12980_e11583: f64 = (assign12980_e11581 * 500000000.0);
        locals.var_bgidlds = assign12980_e11583;
        locals.var_vinr_max = 0.0;
        let assign13000_e11587: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign13000_e11587;
        if (locals.var_guard170 != 0.0) {
            let assign13010_e11591: f64 = (0.75 / locals.var_fcinracc_i);
            locals.var_vinr_max = assign13010_e11591;
        }
        let assign13020_e11596: f64 = (locals.var_axinr_i * locals.var_axinr_i);
        locals.var_ainr = assign13020_e11596;
        let assign13030_e11599: f64 = (9.1093826e-31 * 1000000000.0);
        let assign13030_e11601: f64 = (assign13030_e11599 * locals.var_fntexc_i);
        locals.var_fac_exc = assign13030_e11601;
        let assign13040_e11604: f64 = if locals.var_rg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard171 = assign13040_e11604;
        if (locals.var_guard171 != 0.0) {
            let assign13050_e11608: f64 = (1.0 / locals.var_rg_i);
            locals.var_ggate = assign13050_e11608;
        }
        if (locals.var_guard171 == 0.0) {
            locals.var_ggate = 0.0;
        }
        let assign13070_e11618: f64 = if locals.var_rse_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard172 = assign13070_e11618;
        if (locals.var_guard172 != 0.0) {
            let assign13080_e11622: f64 = (1.0 / locals.var_rse_i);
            locals.var_gsource = assign13080_e11622;
        }
        if (locals.var_guard172 == 0.0) {
            locals.var_gsource = 0.0;
        }
        let assign13100_e11632: f64 = if locals.var_rde_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard173 = assign13100_e11632;
        if (locals.var_guard173 != 0.0) {
            let assign13110_e11636: f64 = (1.0 / locals.var_rde_i);
            locals.var_gdrain = assign13110_e11636;
        }
        if (locals.var_guard173 == 0.0) {
            locals.var_gdrain = 0.0;
        }
        let assign13130_e11646: f64 = if locals.var_rbulk_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign13130_e11646;
        if (locals.var_guard174 != 0.0) {
            let assign13140_e11650: f64 = (1.0 / locals.var_rbulk_i);
            locals.var_gbulk = assign13140_e11650;
        }
        if (locals.var_guard174 == 0.0) {
            locals.var_gbulk = 0.0;
        }
        let assign13160_e11660: f64 = if locals.var_rjuns_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard175 = assign13160_e11660;
        if (locals.var_guard175 != 0.0) {
            let assign13170_e11664: f64 = (1.0 / locals.var_rjuns_i);
            locals.var_gjuns = assign13170_e11664;
        }
        if (locals.var_guard175 == 0.0) {
            locals.var_gjuns = 0.0;
        }
        let assign13190_e11674: f64 = if locals.var_rjund_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard176 = assign13190_e11674;
        if (locals.var_guard176 != 0.0) {
            let assign13200_e11678: f64 = (1.0 / locals.var_rjund_i);
            locals.var_gjund = assign13200_e11678;
        }
        if (locals.var_guard176 == 0.0) {
            locals.var_gjund = 0.0;
        }
        let assign13220_e11688: f64 = if locals.var_rwell_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard177 = assign13220_e11688;
        if (locals.var_guard177 != 0.0) {
            let assign13230_e11692: f64 = (1.0 / locals.var_rwell_i);
            locals.var_gwell = assign13230_e11692;
        }
        if (locals.var_guard177 == 0.0) {
            locals.var_gwell = 0.0;
        }
        let assign13250_e11702: f64 = (locals.var_absource_i * locals.var_invnf);
        locals.var_abs_i = assign13250_e11702;
        let assign13260_e11705: f64 = (locals.var_lssource_i * locals.var_invnf);
        locals.var_lss_i = assign13260_e11705;
        let assign13270_e11708: f64 = (locals.var_lgsource_i * locals.var_invnf);
        locals.var_lgs_i = assign13270_e11708;
        let assign13280_e11711: f64 = (locals.var_abdrain_i * locals.var_invnf);
        locals.var_abd_i = assign13280_e11711;
        let assign13290_e11714: f64 = (locals.var_lsdrain_i * locals.var_invnf);
        locals.var_lsd_i = assign13290_e11714;
        let assign13300_e11717: f64 = (locals.var_lgdrain_i * locals.var_invnf);
        locals.var_lgd_i = assign13300_e11717;
        locals.var_jwcorr = 0.0;
        let assign13320_e11721: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard178 = assign13320_e11721;
        if (locals.var_guard178 != 0.0) {
            locals.var_jwcorr = 1.0;
        }
        locals.var_jww = locals.var_we;
        let assign13350_e11729: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard179 = assign13350_e11729;
        if (locals.var_guard179 != 0.0) {
            let (assign13360_e11736,) = {
    if (locals.var_jw_i > 0.0) {
        (locals.var_jw_i,)
    } else {
        (0.0,)
    }
};
            locals.var_jww = assign13360_e11736;
        }
        let assign13370_e11745: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard180 = assign13370_e11745;
        if (locals.var_guard180 != 0.0) {
            let assign13380_e11749: f64 = (locals.var_as_i * locals.var_invnf);
            locals.var_abs_i = assign13380_e11749;
        }
        if (locals.var_guard180 != 0.0) {
            let assign13390_e11755: f64 = (locals.var_ps_i * locals.var_invnf);
            let assign13390_e11758: f64 = (locals.var_jwcorr * locals.var_jww);
            let assign13390_e11759: f64 = (assign13390_e11755 - assign13390_e11758);
            locals.var_lss_i = assign13390_e11759;
        }
        if (locals.var_guard180 != 0.0) {
            locals.var_lgs_i = locals.var_jww;
        }
        if (locals.var_guard180 != 0.0) {
            let assign13410_e11769: f64 = (locals.var_ad_i * locals.var_invnf);
            locals.var_abd_i = assign13410_e11769;
        }
        if (locals.var_guard180 != 0.0) {
            let assign13420_e11775: f64 = (locals.var_pd_i * locals.var_invnf);
            let assign13420_e11778: f64 = (locals.var_jwcorr * locals.var_jww);
            let assign13420_e11779: f64 = (assign13420_e11775 - assign13420_e11778);
            locals.var_lsd_i = assign13420_e11779;
        }
        if (locals.var_guard180 != 0.0) {
            locals.var_lgd_i = locals.var_jww;
        }
        let assign13440_e11796: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard181 = assign13440_e11796;
        if (locals.var_guard181 != 0.0) {
            let (assign13450_e11803,) = {
    if (locals.var_abs_i > 0.0) {
        (locals.var_abs_i,)
    } else {
        (0.0,)
    }
};
            locals.var_absource_i = assign13450_e11803;
        }
        if (locals.var_guard181 != 0.0) {
            let (assign13460_e11812,) = {
    if (locals.var_lss_i > 0.0) {
        (locals.var_lss_i,)
    } else {
        (0.0,)
    }
};
            locals.var_lssource_i = assign13460_e11812;
        }
        if (locals.var_guard181 != 0.0) {
            let (assign13470_e11821,) = {
    if (locals.var_lgs_i > 0.0) {
        (locals.var_lgs_i,)
    } else {
        (0.0,)
    }
};
            locals.var_lgsource_i = assign13470_e11821;
        }
        if (locals.var_guard181 != 0.0) {
            let (assign13480_e11830,) = {
    if (locals.var_abd_i > 0.0) {
        (locals.var_abd_i,)
    } else {
        (0.0,)
    }
};
            locals.var_abdrain_i = assign13480_e11830;
        }
        if (locals.var_guard181 != 0.0) {
            let (assign13490_e11839,) = {
    if (locals.var_lsd_i > 0.0) {
        (locals.var_lsd_i,)
    } else {
        (0.0,)
    }
};
            locals.var_lsdrain_i = assign13490_e11839;
        }
        if (locals.var_guard181 != 0.0) {
            let (assign13500_e11848,) = {
    if (locals.var_lgd_i > 0.0) {
        (locals.var_lgd_i,)
    } else {
        (0.0,)
    }
};
            locals.var_lgdrain_i = assign13500_e11848;
        }
        if (locals.var_guard181 == 0.0) {
            locals.var_absource_i = 0.0;
            locals.var_lssource_i = 0.0;
            locals.var_lgsource_i = 0.0;
            locals.var_abdrain_i = 0.0;
            locals.var_lsdrain_i = 0.0;
            locals.var_lgdrain_i = 0.0;
        }
        locals.var_vbimin_s = 0.0;
        locals.var_vbimin_d = 0.0;
        locals.var_vfmin_s = 0.0;
        locals.var_vfmin_d = 0.0;
        locals.var_vch_s = 0.0;
        locals.var_vch_d = 0.0;
        locals.var_vbbtlim_s = 0.0;
        locals.var_vbbtlim_d = 0.0;
        locals.var_vmax_s = 0.0;
        locals.var_vmax_d = 0.0;
        locals.var_exp_vmax_over_phitd_s = 0.0;
        locals.var_exp_vmax_over_phitd_d = 0.0;
        locals.var_isatfor1_s = 0.0;
        locals.var_isatfor1_d = 0.0;
        locals.var_mfor1_s = 1.0;
        locals.var_mfor1_d = 1.0;
        (locals.var_isatfor2_s, locals.var_isatfor2_s_dn5, locals.var_isatfor2_s_dn6, locals.var_isatfor2_s_dn7, locals.var_isatfor2_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_isatfor2_d, locals.var_isatfor2_d_dn5, locals.var_isatfor2_d_dn6, locals.var_isatfor2_d_dn7, locals.var_isatfor2_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_mfor2_s, locals.var_mfor2_s_dn5, locals.var_mfor2_s_dn6, locals.var_mfor2_s_dn7, locals.var_mfor2_s_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_mfor2_d, locals.var_mfor2_d_dn5, locals.var_mfor2_d_dn6, locals.var_mfor2_d_dn7, locals.var_mfor2_d_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_isatrev_s, locals.var_isatrev_s_dn5, locals.var_isatrev_s_dn6, locals.var_isatrev_s_dn7, locals.var_isatrev_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_isatrev_d, locals.var_isatrev_d_dn5, locals.var_isatrev_d_dn6, locals.var_isatrev_d_dn7, locals.var_isatrev_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_mrev_s, locals.var_mrev_s_dn5, locals.var_mrev_s_dn6, locals.var_mrev_s_dn7, locals.var_mrev_s_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_mrev_d, locals.var_mrev_d_dn5, locals.var_mrev_d_dn6, locals.var_mrev_d_dn7, locals.var_mrev_d_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_m0flag_s = 0.0;
        locals.var_m0flag_d = 0.0;
        locals.var_xhighf1_s = 0.0;
        locals.var_xhighf1_d = 0.0;
        locals.var_expxhf1_s = 0.0;
        locals.var_expxhf1_d = 0.0;
        (locals.var_xhighf2_s, locals.var_xhighf2_s_dn5, locals.var_xhighf2_s_dn6, locals.var_xhighf2_s_dn7, locals.var_xhighf2_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_xhighf2_d, locals.var_xhighf2_d_dn5, locals.var_xhighf2_d_dn6, locals.var_xhighf2_d_dn7, locals.var_xhighf2_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_expxhf2_s, locals.var_expxhf2_s_dn5, locals.var_expxhf2_s_dn6, locals.var_expxhf2_s_dn7, locals.var_expxhf2_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_expxhf2_d, locals.var_expxhf2_d_dn5, locals.var_expxhf2_d_dn6, locals.var_expxhf2_d_dn7, locals.var_expxhf2_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_xhighr_s, locals.var_xhighr_s_dn5, locals.var_xhighr_s_dn6, locals.var_xhighr_s_dn7, locals.var_xhighr_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_xhighr_d, locals.var_xhighr_d_dn5, locals.var_xhighr_d_dn6, locals.var_xhighr_d_dn7, locals.var_xhighr_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_expxhr_s, locals.var_expxhr_s_dn5, locals.var_expxhr_s_dn6, locals.var_expxhr_s_dn7, locals.var_expxhr_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_expxhr_d, locals.var_expxhr_d_dn5, locals.var_expxhr_d_dn6, locals.var_expxhr_d_dn7, locals.var_expxhr_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_zflagbot_s = 1.0;
        locals.var_zflagbot_d = 1.0;
        locals.var_zflagsti_s = 1.0;
        locals.var_zflagsti_d = 1.0;
        locals.var_zflaggat_s = 1.0;
        locals.var_zflaggat_d = 1.0;
        (locals.var_m0_rev, locals.var_m0_rev_dn5, locals.var_m0_rev_dn6, locals.var_m0_rev_dn7, locals.var_m0_rev_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_mcor_rev, locals.var_mcor_rev_dn5, locals.var_mcor_rev_dn6, locals.var_mcor_rev_dn7, locals.var_mcor_rev_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_i1_cor, locals.var_i1_cor_dn5, locals.var_i1_cor_dn6, locals.var_i1_cor_dn7, locals.var_i1_cor_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_i2_cor, locals.var_i2_cor_dn5, locals.var_i2_cor_dn6, locals.var_i2_cor_dn7, locals.var_i2_cor_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_i3_cor, locals.var_i3_cor_dn5, locals.var_i3_cor_dn6, locals.var_i3_cor_dn7, locals.var_i3_cor_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_i4_cor, locals.var_i4_cor_dn5, locals.var_i4_cor_dn6, locals.var_i4_cor_dn7, locals.var_i4_cor_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_i5_cor, locals.var_i5_cor_dn5, locals.var_i5_cor_dn6, locals.var_i5_cor_dn7, locals.var_i5_cor_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_tt0 = 0.0;
        (locals.var_tt1, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        (locals.var_tt2, locals.var_tt2_dn5, locals.var_tt2_dn6, locals.var_tt2_dn7, locals.var_tt2_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_zfrac = 0.0;
        (locals.var_alphaje, locals.var_alphaje_dn5, locals.var_alphaje_dn6, locals.var_alphaje_dn7, locals.var_alphaje_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        let assign14130_e11939: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard182 = assign14130_e11939;
        let assign14140_e11942: f64 = (locals.var_idsatbot * locals.var_absource_i);
        let assign14140_e11944: f64 = if assign14140_e11942 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard183 = assign14140_e11944;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard183 != 0.0)) {
            let assign14150_e11952: f64 = (locals.var_idsatbot * locals.var_absource_i);
            let assign14150_e11953: f64 = (p.p822 / assign14150_e11952);
            let assign14150_e11955: f64 = (assign14150_e11953 + 1.0);
            let assign14150_e11956: f64 = (assign14150_e11955).ln();
            let assign14150_e11957: f64 = (locals.var_phitd * assign14150_e11956);
            locals.var_vmaxbot = assign14150_e11957;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard183 == 0.0)) {
            locals.var_vmaxbot = 100000000.0;
        }
        let assign14170_e11969: f64 = (locals.var_idsatsti * locals.var_lssource_i);
        let assign14170_e11971: f64 = if assign14170_e11969 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard184 = assign14170_e11971;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard184 != 0.0)) {
            let assign14180_e11979: f64 = (locals.var_idsatsti * locals.var_lssource_i);
            let assign14180_e11980: f64 = (p.p822 / assign14180_e11979);
            let assign14180_e11982: f64 = (assign14180_e11980 + 1.0);
            let assign14180_e11983: f64 = (assign14180_e11982).ln();
            let assign14180_e11984: f64 = (locals.var_phitd * assign14180_e11983);
            locals.var_vmaxsti = assign14180_e11984;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard184 == 0.0)) {
            locals.var_vmaxsti = 100000000.0;
        }
        let assign14200_e11996: f64 = (locals.var_idsatgat * locals.var_lgsource_i);
        let assign14200_e11998: f64 = if assign14200_e11996 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign14200_e11998;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard185 != 0.0)) {
            let assign14210_e12006: f64 = (locals.var_idsatgat * locals.var_lgsource_i);
            let assign14210_e12007: f64 = (p.p822 / assign14210_e12006);
            let assign14210_e12009: f64 = (assign14210_e12007 + 1.0);
            let assign14210_e12010: f64 = (assign14210_e12009).ln();
            let assign14210_e12011: f64 = (locals.var_phitd * assign14210_e12010);
            locals.var_vmaxgat = assign14210_e12011;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard185 == 0.0)) {
            locals.var_vmaxgat = 100000000.0;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14230_e12024: f64 = (locals.var_vmaxbot).min(locals.var_vmaxsti);
            let assign14230_e12026: f64 = (assign14230_e12024).min(locals.var_vmaxgat);
            locals.var_vmax_s = assign14230_e12026;
        }
        let assign14240_e12031: f64 = (locals.var_vmax_s * locals.var_phitdinv);
        let assign14240_e12032: f64 = (assign14240_e12031).abs();
        let assign14240_e12034: f64 = if assign14240_e12032 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard186 = assign14240_e12034;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard186 != 0.0)) {
            let assign14250_e12040: f64 = (locals.var_vmax_s * locals.var_phitdinv);
            let assign14250_e12041: f64 = (assign14250_e12040).exp();
            locals.var_exp_vmax_over_phitd_s = assign14250_e12041;
        }
        let assign14260_e12046: f64 = (locals.var_vmax_s * locals.var_phitdinv);
        let assign14260_e12048: f64 = if assign14260_e12046 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard187 = assign14260_e12048;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard186 == 0.0)) && (locals.var_guard187 != 0.0)) {
            let assign14270_e12058: f64 = (-230.25850929940458);
            let assign14270_e12061: f64 = (locals.var_vmax_s * locals.var_phitdinv);
            let assign14270_e12062: f64 = (assign14270_e12058 - assign14270_e12061);
            let assign14270_e12066: f64 = (-230.25850929940458);
            let assign14270_e12069: f64 = (locals.var_vmax_s * locals.var_phitdinv);
            let assign14270_e12070: f64 = (assign14270_e12066 - assign14270_e12069);
            let assign14270_e12073: f64 = (-230.25850929940458);
            let assign14270_e12076: f64 = (locals.var_vmax_s * locals.var_phitdinv);
            let assign14270_e12077: f64 = (assign14270_e12073 - assign14270_e12076);
            let assign14270_e12079: f64 = (assign14270_e12077 * 0.3333333333333333);
            let assign14270_e12080: f64 = (1.0 + assign14270_e12079);
            let assign14270_e12081: f64 = (assign14270_e12070 * assign14270_e12080);
            let assign14270_e12082: f64 = (0.5 * assign14270_e12081);
            let assign14270_e12083: f64 = (1.0 + assign14270_e12082);
            let assign14270_e12084: f64 = (assign14270_e12062 * assign14270_e12083);
            let assign14270_e12085: f64 = (1.0 + assign14270_e12084);
            let assign14270_e12086: f64 = (1e-100 / assign14270_e12085);
            locals.var_exp_vmax_over_phitd_s = assign14270_e12086;
        }
    }
    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard182 != 0.0) && (locals.var_guard186 == 0.0)) && (locals.var_guard187 == 0.0)) {
            let assign14280_e12100: f64 = (locals.var_vmax_s * locals.var_phitdinv);
            let assign14280_e12102: f64 = (assign14280_e12100 - 230.25850929940458);
            let assign14280_e12107: f64 = (locals.var_vmax_s * locals.var_phitdinv);
            let assign14280_e12109: f64 = (assign14280_e12107 - 230.25850929940458);
            let assign14280_e12113: f64 = (locals.var_vmax_s * locals.var_phitdinv);
            let assign14280_e12115: f64 = (assign14280_e12113 - 230.25850929940458);
            let assign14280_e12117: f64 = (assign14280_e12115 * 0.3333333333333333);
            let assign14280_e12118: f64 = (1.0 + assign14280_e12117);
            let assign14280_e12119: f64 = (assign14280_e12109 * assign14280_e12118);
            let assign14280_e12120: f64 = (0.5 * assign14280_e12119);
            let assign14280_e12121: f64 = (1.0 + assign14280_e12120);
            let assign14280_e12122: f64 = (assign14280_e12102 * assign14280_e12121);
            let assign14280_e12123: f64 = (1.0 + assign14280_e12122);
            let assign14280_e12124: f64 = (1e100 * assign14280_e12123);
            locals.var_exp_vmax_over_phitd_s = assign14280_e12124;
        }
        if (locals.var_guard182 != 0.0) {
            locals.var_vbibot2 = locals.var_vbibot;
            locals.var_vbisti2 = locals.var_vbisti;
            locals.var_vbigat2 = locals.var_vbigat;
            locals.var_pbot2 = p.p831;
            locals.var_psti2 = p.p832;
            locals.var_pgat2 = p.p833;
            locals.var_vbibot2r = p.p828;
            locals.var_vbisti2r = p.p829;
            locals.var_vbigat2r = p.p830;
        }
        let assign14380_e12165: f64 = if locals.var_absource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign14380_e12165;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard188 != 0.0)) {
            let assign14390_e12171: f64 = (locals.var_vbisti + locals.var_vbigat);
            locals.var_vbibot2 = assign14390_e12171;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard188 != 0.0)) {
            let assign14400_e12180: f64 = (p.p832).min(p.p833);
            let assign14400_e12181: f64 = (0.9 * assign14400_e12180);
            locals.var_pbot2 = assign14400_e12181;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard188 != 0.0)) {
            let assign14410_e12189: f64 = (p.p829 + p.p830);
            locals.var_vbibot2r = assign14410_e12189;
        }
        let assign14420_e12194: f64 = if locals.var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign14420_e12194;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard189 != 0.0)) {
            let assign14430_e12200: f64 = (locals.var_vbibot + locals.var_vbigat);
            locals.var_vbisti2 = assign14430_e12200;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard189 != 0.0)) {
            let assign14440_e12209: f64 = (p.p831).min(p.p833);
            let assign14440_e12210: f64 = (0.9 * assign14440_e12209);
            locals.var_psti2 = assign14440_e12210;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard189 != 0.0)) {
            let assign14450_e12218: f64 = (p.p828 + p.p830);
            locals.var_vbisti2r = assign14450_e12218;
        }
        let assign14460_e12223: f64 = if locals.var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign14460_e12223;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard190 != 0.0)) {
            let assign14470_e12229: f64 = (locals.var_vbibot + locals.var_vbisti);
            locals.var_vbigat2 = assign14470_e12229;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard190 != 0.0)) {
            let assign14480_e12238: f64 = (p.p831).min(p.p832);
            let assign14480_e12239: f64 = (0.9 * assign14480_e12238);
            locals.var_pgat2 = assign14480_e12239;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard190 != 0.0)) {
            let assign14490_e12247: f64 = (p.p828 + p.p829);
            locals.var_vbigat2r = assign14490_e12247;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14500_e12253: f64 = (locals.var_vbibot2).min(locals.var_vbisti2);
            let assign14500_e12255: f64 = (assign14500_e12253).min(locals.var_vbigat2);
            locals.var_vbimin_s = assign14500_e12255;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14510_e12261: f64 = (locals.var_vbimin_s * 0.1);
            locals.var_vch_s = assign14510_e12261;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14520_e12267: f64 = (locals.var_pbot2).max(locals.var_psti2);
            let assign14520_e12269: f64 = (assign14520_e12267).max(locals.var_pgat2);
            locals.var_pmax = assign14520_e12269;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14530_e12277: f64 = (-1.0);
            let assign14530_e12279: f64 = (assign14530_e12277 / locals.var_pmax);
            let assign14530_e12280: f64 = (2.0_f64).powf(assign14530_e12279);
            let assign14530_e12281: f64 = (1.0 - assign14530_e12280);
            let assign14530_e12282: f64 = (locals.var_vbimin_s * assign14530_e12281);
            locals.var_vfmin_s = assign14530_e12282;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14540_e12288: f64 = (locals.var_vbibot2r).min(locals.var_vbisti2r);
            let assign14540_e12290: f64 = (assign14540_e12288).min(locals.var_vbigat2r);
            let assign14540_e12292: f64 = (assign14540_e12290 - 0.05);
            locals.var_vbbtlim_s = assign14540_e12292;
        }
        let assign14550_e12297: f64 = (locals.var_idsatbot_d * locals.var_abdrain_i);
        let assign14550_e12299: f64 = if assign14550_e12297 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign14550_e12299;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard191 != 0.0)) {
            let assign14560_e12307: f64 = (locals.var_idsatbot_d * locals.var_abdrain_i);
            let assign14560_e12308: f64 = (p.p822 / assign14560_e12307);
            let assign14560_e12310: f64 = (assign14560_e12308 + 1.0);
            let assign14560_e12311: f64 = (assign14560_e12310).ln();
            let assign14560_e12312: f64 = (locals.var_phitd * assign14560_e12311);
            locals.var_vmaxbot = assign14560_e12312;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard191 == 0.0)) {
            locals.var_vmaxbot = 100000000.0;
        }
        let assign14580_e12324: f64 = (locals.var_idsatsti_d * locals.var_lsdrain_i);
        let assign14580_e12326: f64 = if assign14580_e12324 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign14580_e12326;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard192 != 0.0)) {
            let assign14590_e12334: f64 = (locals.var_idsatsti_d * locals.var_lsdrain_i);
            let assign14590_e12335: f64 = (p.p822 / assign14590_e12334);
            let assign14590_e12337: f64 = (assign14590_e12335 + 1.0);
            let assign14590_e12338: f64 = (assign14590_e12337).ln();
            let assign14590_e12339: f64 = (locals.var_phitd * assign14590_e12338);
            locals.var_vmaxsti = assign14590_e12339;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard192 == 0.0)) {
            locals.var_vmaxsti = 100000000.0;
        }
        let assign14610_e12351: f64 = (locals.var_idsatgat_d * locals.var_lgdrain_i);
        let assign14610_e12353: f64 = if assign14610_e12351 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign14610_e12353;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard193 != 0.0)) {
            let assign14620_e12361: f64 = (locals.var_idsatgat_d * locals.var_lgdrain_i);
            let assign14620_e12362: f64 = (p.p822 / assign14620_e12361);
            let assign14620_e12364: f64 = (assign14620_e12362 + 1.0);
            let assign14620_e12365: f64 = (assign14620_e12364).ln();
            let assign14620_e12366: f64 = (locals.var_phitd * assign14620_e12365);
            locals.var_vmaxgat = assign14620_e12366;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard193 == 0.0)) {
            locals.var_vmaxgat = 100000000.0;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14640_e12379: f64 = (locals.var_vmaxbot).min(locals.var_vmaxsti);
            let assign14640_e12381: f64 = (assign14640_e12379).min(locals.var_vmaxgat);
            locals.var_vmax_d = assign14640_e12381;
        }
        let assign14650_e12386: f64 = (locals.var_vmax_d * locals.var_phitdinv);
        let assign14650_e12387: f64 = (assign14650_e12386).abs();
        let assign14650_e12389: f64 = if assign14650_e12387 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign14650_e12389;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard194 != 0.0)) {
            let assign14660_e12395: f64 = (locals.var_vmax_d * locals.var_phitdinv);
            let assign14660_e12396: f64 = (assign14660_e12395).exp();
            locals.var_exp_vmax_over_phitd_d = assign14660_e12396;
        }
        let assign14670_e12401: f64 = (locals.var_vmax_d * locals.var_phitdinv);
        let assign14670_e12403: f64 = if assign14670_e12401 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard195 = assign14670_e12403;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard194 == 0.0)) && (locals.var_guard195 != 0.0)) {
            let assign14680_e12413: f64 = (-230.25850929940458);
            let assign14680_e12416: f64 = (locals.var_vmax_d * locals.var_phitdinv);
            let assign14680_e12417: f64 = (assign14680_e12413 - assign14680_e12416);
            let assign14680_e12421: f64 = (-230.25850929940458);
            let assign14680_e12424: f64 = (locals.var_vmax_d * locals.var_phitdinv);
            let assign14680_e12425: f64 = (assign14680_e12421 - assign14680_e12424);
            let assign14680_e12428: f64 = (-230.25850929940458);
            let assign14680_e12431: f64 = (locals.var_vmax_d * locals.var_phitdinv);
            let assign14680_e12432: f64 = (assign14680_e12428 - assign14680_e12431);
            let assign14680_e12434: f64 = (assign14680_e12432 * 0.3333333333333333);
            let assign14680_e12435: f64 = (1.0 + assign14680_e12434);
            let assign14680_e12436: f64 = (assign14680_e12425 * assign14680_e12435);
            let assign14680_e12437: f64 = (0.5 * assign14680_e12436);
            let assign14680_e12438: f64 = (1.0 + assign14680_e12437);
            let assign14680_e12439: f64 = (assign14680_e12417 * assign14680_e12438);
            let assign14680_e12440: f64 = (1.0 + assign14680_e12439);
            let assign14680_e12441: f64 = (1e-100 / assign14680_e12440);
            locals.var_exp_vmax_over_phitd_d = assign14680_e12441;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard194 == 0.0)) && (locals.var_guard195 == 0.0)) {
            let assign14690_e12455: f64 = (locals.var_vmax_d * locals.var_phitdinv);
            let assign14690_e12457: f64 = (assign14690_e12455 - 230.25850929940458);
            let assign14690_e12462: f64 = (locals.var_vmax_d * locals.var_phitdinv);
            let assign14690_e12464: f64 = (assign14690_e12462 - 230.25850929940458);
            let assign14690_e12468: f64 = (locals.var_vmax_d * locals.var_phitdinv);
            let assign14690_e12470: f64 = (assign14690_e12468 - 230.25850929940458);
            let assign14690_e12472: f64 = (assign14690_e12470 * 0.3333333333333333);
            let assign14690_e12473: f64 = (1.0 + assign14690_e12472);
            let assign14690_e12474: f64 = (assign14690_e12464 * assign14690_e12473);
            let assign14690_e12475: f64 = (0.5 * assign14690_e12474);
            let assign14690_e12476: f64 = (1.0 + assign14690_e12475);
            let assign14690_e12477: f64 = (assign14690_e12457 * assign14690_e12476);
            let assign14690_e12478: f64 = (1.0 + assign14690_e12477);
            let assign14690_e12479: f64 = (1e100 * assign14690_e12478);
            locals.var_exp_vmax_over_phitd_d = assign14690_e12479;
        }
        if (locals.var_guard182 != 0.0) {
            locals.var_vbibot2 = locals.var_vbibot_d;
            locals.var_vbisti2 = locals.var_vbisti_d;
            locals.var_vbigat2 = locals.var_vbigat_d;
            locals.var_pbot2 = locals.var_pbotd_i;
            locals.var_psti2 = locals.var_pstid_i;
            locals.var_pgat2 = locals.var_pgatd_i;
            locals.var_vbibot2r = locals.var_vbirbotd_i;
            locals.var_vbisti2r = locals.var_vbirstid_i;
            locals.var_vbigat2r = locals.var_vbirgatd_i;
        }
        let assign14790_e12520: f64 = if locals.var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard196 = assign14790_e12520;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard196 != 0.0)) {
            let assign14800_e12526: f64 = (locals.var_vbisti_d + locals.var_vbigat_d);
            locals.var_vbibot2 = assign14800_e12526;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard196 != 0.0)) {
            let assign14810_e12535: f64 = (locals.var_pstid_i).min(locals.var_pgatd_i);
            let assign14810_e12536: f64 = (0.9 * assign14810_e12535);
            locals.var_pbot2 = assign14810_e12536;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard196 != 0.0)) {
            let assign14820_e12544: f64 = (locals.var_vbirstid_i + locals.var_vbirgatd_i);
            locals.var_vbibot2r = assign14820_e12544;
        }
        let assign14830_e12549: f64 = if locals.var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign14830_e12549;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard197 != 0.0)) {
            let assign14840_e12555: f64 = (locals.var_vbibot_d + locals.var_vbigat_d);
            locals.var_vbisti2 = assign14840_e12555;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard197 != 0.0)) {
            let assign14850_e12564: f64 = (locals.var_pbotd_i).min(locals.var_pgatd_i);
            let assign14850_e12565: f64 = (0.9 * assign14850_e12564);
            locals.var_psti2 = assign14850_e12565;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard197 != 0.0)) {
            let assign14860_e12573: f64 = (locals.var_vbirbotd_i + locals.var_vbirgatd_i);
            locals.var_vbisti2r = assign14860_e12573;
        }
        let assign14870_e12578: f64 = if locals.var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard198 = assign14870_e12578;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard198 != 0.0)) {
            let assign14880_e12584: f64 = (locals.var_vbibot_d + locals.var_vbisti_d);
            locals.var_vbigat2 = assign14880_e12584;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard198 != 0.0)) {
            let assign14890_e12593: f64 = (locals.var_pbotd_i).min(locals.var_pstid_i);
            let assign14890_e12594: f64 = (0.9 * assign14890_e12593);
            locals.var_pgat2 = assign14890_e12594;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard198 != 0.0)) {
            let assign14900_e12602: f64 = (locals.var_vbirbotd_i + locals.var_vbirstid_i);
            locals.var_vbigat2r = assign14900_e12602;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14910_e12608: f64 = (locals.var_vbibot2).min(locals.var_vbisti2);
            let assign14910_e12610: f64 = (assign14910_e12608).min(locals.var_vbigat2);
            locals.var_vbimin_d = assign14910_e12610;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14920_e12616: f64 = (locals.var_vbimin_d * 0.1);
            locals.var_vch_d = assign14920_e12616;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14930_e12622: f64 = (locals.var_pbot2).max(locals.var_psti2);
            let assign14930_e12624: f64 = (assign14930_e12622).max(locals.var_pgat2);
            locals.var_pmax = assign14930_e12624;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14940_e12632: f64 = (-1.0);
            let assign14940_e12634: f64 = (assign14940_e12632 / locals.var_pmax);
            let assign14940_e12635: f64 = (2.0_f64).powf(assign14940_e12634);
            let assign14940_e12636: f64 = (1.0 - assign14940_e12635);
            let assign14940_e12637: f64 = (locals.var_vbimin_d * assign14940_e12636);
            locals.var_vfmin_d = assign14940_e12637;
        }
        if (locals.var_guard182 != 0.0) {
            let assign14950_e12643: f64 = (locals.var_vbibot2r).min(locals.var_vbisti2r);
            let assign14950_e12645: f64 = (assign14950_e12643).min(locals.var_vbigat2r);
            let assign14950_e12647: f64 = (assign14950_e12645 - 0.05);
            locals.var_vbbtlim_d = assign14950_e12647;
        }
        let assign14960_e12652: f64 = if locals.var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard199 = assign14960_e12652;
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_idmult = 0.0;
            locals.var_z = 0.0;
            locals.var_zinv = 0.0;
            locals.var_two_psistar = 0.0;
            locals.var_vjlim = 0.0;
            locals.var_vjsrh = 0.0;
            locals.var_vbbt = 0.0;
            locals.var_vav = 0.0;
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_id__blk219 = 0.0;
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vbi_minus_vjsrh = 0.0;
            locals.var_wsrhstep = 0.0;
            locals.var_dwsrh = 0.0;
            locals.var_wsrh = 0.0;
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
    }
    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_fracna = 0.4;
            locals.var_fracnb = 0.65;
            locals.var_fraci = 0.8;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign15450_e12945: f64 = (-locals.var_fracna);
            let assign15450_e12947: f64 = (assign15450_e12945 * p.p928);
            locals.var_v1 = assign15450_e12947;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign15460_e12954: f64 = (-locals.var_fracnb);
            let assign15460_e12956: f64 = (assign15460_e12954 * p.p928);
            locals.var_v2 = assign15460_e12956;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign15470_e12963: f64 = (-locals.var_fraci);
            let assign15470_e12965: f64 = (assign15470_e12963 * p.p928);
            locals.var_v3 = assign15470_e12965;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_v4 = 0.1;
            locals.var_v5 = 0.2;
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign15520_e13003: f64 = if (!(((locals.var_absource_i == 0.0) && (locals.var_lssource_i == 0.0)) && (locals.var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard248 = assign15520_e13003;
        let assign15600_e13089: f64 = if locals.var_v1 < locals.var_vmax_s { 1.0 } else { 0.0 };
        locals.var_guard249 = assign15600_e13089;
        let assign15610_e13091: f64 = (-0.5);
        let assign15610_e13094: f64 = (locals.var_v1 * locals.var_phitdinv);
        let assign15610_e13095: f64 = (assign15610_e13091 * assign15610_e13094);
        let assign15610_e13096: f64 = (assign15610_e13095).abs();
        let assign15610_e13098: f64 = if assign15610_e13096 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign15610_e13098;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 != 0.0)) {
            let assign15620_e13109: f64 = (-0.5);
            let assign15620_e13112: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign15620_e13113: f64 = (assign15620_e13109 * assign15620_e13112);
            let assign15620_e13114: f64 = (assign15620_e13113).exp();
            locals.var_z = assign15620_e13114;
        }
        let assign15630_e13118: f64 = (-0.5);
        let assign15630_e13121: f64 = (locals.var_v1 * locals.var_phitdinv);
        let assign15630_e13122: f64 = (assign15630_e13118 * assign15630_e13121);
        let assign15630_e13124: f64 = if assign15630_e13122 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign15630_e13124;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard251 != 0.0)) {
            let assign15640_e13140: f64 = (-230.25850929940458);
            let assign15640_e13142: f64 = (-0.5);
            let assign15640_e13145: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign15640_e13146: f64 = (assign15640_e13142 * assign15640_e13145);
            let assign15640_e13147: f64 = (assign15640_e13140 - assign15640_e13146);
            let assign15640_e13151: f64 = (-230.25850929940458);
            let assign15640_e13153: f64 = (-0.5);
            let assign15640_e13156: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign15640_e13157: f64 = (assign15640_e13153 * assign15640_e13156);
            let assign15640_e13158: f64 = (assign15640_e13151 - assign15640_e13157);
            let assign15640_e13161: f64 = (-230.25850929940458);
            let assign15640_e13163: f64 = (-0.5);
            let assign15640_e13166: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign15640_e13167: f64 = (assign15640_e13163 * assign15640_e13166);
            let assign15640_e13168: f64 = (assign15640_e13161 - assign15640_e13167);
            let assign15640_e13170: f64 = (assign15640_e13168 * 0.3333333333333333);
            let assign15640_e13171: f64 = (1.0 + assign15640_e13170);
            let assign15640_e13172: f64 = (assign15640_e13158 * assign15640_e13171);
            let assign15640_e13173: f64 = (0.5 * assign15640_e13172);
            let assign15640_e13174: f64 = (1.0 + assign15640_e13173);
            let assign15640_e13175: f64 = (assign15640_e13147 * assign15640_e13174);
            let assign15640_e13176: f64 = (1.0 + assign15640_e13175);
            let assign15640_e13177: f64 = (1e-100 / assign15640_e13176);
            locals.var_z = assign15640_e13177;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard251 == 0.0)) {
            let assign15650_e13196: f64 = (-0.5);
            let assign15650_e13199: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign15650_e13200: f64 = (assign15650_e13196 * assign15650_e13199);
            let assign15650_e13202: f64 = (assign15650_e13200 - 230.25850929940458);
            let assign15650_e13206: f64 = (-0.5);
            let assign15650_e13209: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign15650_e13210: f64 = (assign15650_e13206 * assign15650_e13209);
            let assign15650_e13212: f64 = (assign15650_e13210 - 230.25850929940458);
            let assign15650_e13215: f64 = (-0.5);
            let assign15650_e13218: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign15650_e13219: f64 = (assign15650_e13215 * assign15650_e13218);
            let assign15650_e13221: f64 = (assign15650_e13219 - 230.25850929940458);
            let assign15650_e13223: f64 = (assign15650_e13221 * 0.3333333333333333);
            let assign15650_e13224: f64 = (1.0 + assign15650_e13223);
            let assign15650_e13225: f64 = (assign15650_e13212 * assign15650_e13224);
            let assign15650_e13226: f64 = (0.5 * assign15650_e13225);
            let assign15650_e13227: f64 = (1.0 + assign15650_e13226);
            let assign15650_e13228: f64 = (assign15650_e13202 * assign15650_e13227);
            let assign15650_e13229: f64 = (1.0 + assign15650_e13228);
            let assign15650_e13230: f64 = (1e100 * assign15650_e13229);
            locals.var_z = assign15650_e13230;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
            let assign15660_e13242: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign15660_e13242;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
            let assign15670_e13254: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign15670_e13254;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) {
            let assign15680_e13268: f64 = (locals.var_v1 - locals.var_vmax_s);
            let assign15680_e13270: f64 = (assign15680_e13268 * locals.var_phitdinv);
            let assign15680_e13271: f64 = (1.0 + assign15680_e13270);
            let assign15680_e13273: f64 = (assign15680_e13271 * locals.var_exp_vmax_over_phitd_s);
            locals.var_idmult = assign15680_e13273;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) {
            let assign15690_e13285: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign15690_e13285;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) {
            let assign15700_e13298: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign15700_e13298;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) {
            let assign15710_e13308: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign15710_e13308;
        }
        let assign15720_e13313: f64 = if locals.var_v1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign15720_e13313;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard252 != 0.0)) {
            let assign15730_e13325: f64 = (2.0 + locals.var_z);
            let assign15730_e13328: f64 = (locals.var_z + 1.0);
            let assign15730_e13331: f64 = (locals.var_z + 3.0);
            let assign15730_e13332: f64 = (assign15730_e13328 * assign15730_e13331);
            let assign15730_e13333: f64 = (assign15730_e13332).sqrt();
            let assign15730_e13334: f64 = (assign15730_e13325 + assign15730_e13333);
            let assign15730_e13335: f64 = (assign15730_e13334).ln();
            let assign15730_e13336: f64 = (locals.var_phitd * assign15730_e13335);
            let assign15730_e13337: f64 = (2.0 * assign15730_e13336);
            locals.var_two_psistar = assign15730_e13337;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard252 == 0.0)) {
            let assign15740_e13349: f64 = (-locals.var_v1);
            let assign15740_e13354: f64 = (2.0 * locals.var_zinv);
            let assign15740_e13356: f64 = (assign15740_e13354 + 1.0);
            let assign15740_e13359: f64 = (1.0 + locals.var_zinv);
            let assign15740_e13363: f64 = (3.0 * locals.var_zinv);
            let assign15740_e13364: f64 = (1.0 + assign15740_e13363);
            let assign15740_e13365: f64 = (assign15740_e13359 * assign15740_e13364);
            let assign15740_e13366: f64 = (assign15740_e13365).sqrt();
            let assign15740_e13367: f64 = (assign15740_e13356 + assign15740_e13366);
            let assign15740_e13368: f64 = (assign15740_e13367).ln();
            let assign15740_e13369: f64 = (locals.var_phitd * assign15740_e13368);
            let assign15740_e13370: f64 = (2.0 * assign15740_e13369);
            let assign15740_e13371: f64 = (assign15740_e13349 + assign15740_e13370);
            locals.var_two_psistar = assign15740_e13371;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) {
            let assign15750_e13381: f64 = (locals.var_vbimin_s - locals.var_two_psistar);
            locals.var_vjlim = assign15750_e13381;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) {
            let assign15760_e13392: f64 = (locals.var_v1 + locals.var_vjlim);
            let assign15760_e13395: f64 = (locals.var_v1 - locals.var_vjlim);
            let assign15760_e13398: f64 = (locals.var_v1 - locals.var_vjlim);
            let assign15760_e13399: f64 = (assign15760_e13395 * assign15760_e13398);
            let assign15760_e13402: f64 = (4.0 * locals.var_phitd);
            let assign15760_e13404: f64 = (assign15760_e13402 * locals.var_phitd);
            let assign15760_e13405: f64 = (assign15760_e13399 + assign15760_e13404);
            let assign15760_e13406: f64 = (assign15760_e13405).sqrt();
            let assign15760_e13407: f64 = (assign15760_e13392 - assign15760_e13406);
            let assign15760_e13408: f64 = (0.5 * assign15760_e13407);
            locals.var_vjsrh = assign15760_e13408;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) {
            let assign15770_e13419: f64 = (locals.var_v1 + locals.var_vbbtlim_s);
            let assign15770_e13422: f64 = (locals.var_v1 - locals.var_vbbtlim_s);
            let assign15770_e13425: f64 = (locals.var_v1 - locals.var_vbbtlim_s);
            let assign15770_e13426: f64 = (assign15770_e13422 * assign15770_e13425);
            let assign15770_e13429: f64 = (4.0 * locals.var_phitr);
            let assign15770_e13431: f64 = (assign15770_e13429 * locals.var_phitr);
            let assign15770_e13432: f64 = (assign15770_e13426 + assign15770_e13431);
            let assign15770_e13433: f64 = (assign15770_e13432).sqrt();
            let assign15770_e13434: f64 = (assign15770_e13419 - assign15770_e13433);
            let assign15770_e13435: f64 = (0.5 * assign15770_e13434);
            locals.var_vbbt = assign15770_e13435;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard248 != 0.0)) {
            let assign15780_e13446: f64 = locals.var_v1;
            let assign15780_e13449: f64 = locals.var_v1;
            let assign15780_e13452: f64 = locals.var_v1;
            let assign15780_e13453: f64 = (assign15780_e13449 * assign15780_e13452);
            let assign15780_e13456: f64 = (4.0 * 1e-6);
            let assign15780_e13458: f64 = (assign15780_e13456 * 1e-6);
            let assign15780_e13459: f64 = (assign15780_e13453 + assign15780_e13458);
            let assign15780_e13460: f64 = (assign15780_e13459).sqrt();
            let assign15780_e13461: f64 = (assign15780_e13446 - assign15780_e13460);
            let assign15780_e13462: f64 = (0.5 * assign15780_e13461);
            locals.var_vav = assign15780_e13462;
        }
        let assign15790_e13467: f64 = if locals.var_absource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign15790_e13467;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) {
            let assign15810_e13484: f64 = (locals.var_idsatbot * locals.var_idmult);
            locals.var_id__blk219 = assign15810_e13484;
        }
        let assign15820_e13493: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard254 = assign15820_e13493;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) {
            let assign15840_e13516: f64 = (locals.var_vbibot - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign15840_e13516;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) {
            let assign15850_e13532: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign15850_e13533: f64 = (1.0 - assign15850_e13532);
            let assign15850_e13534: f64 = (assign15850_e13533).sqrt();
            let assign15850_e13535: f64 = (1.0 - assign15850_e13534);
            locals.var_wsrhstep = assign15850_e13535;
        }
        let assign15860_e13540: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign15860_e13540;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) && (locals.var_guard255 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) && (locals.var_guard255 == 0.0)) {
            let assign15880_e13569: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign15880_e13571: f64 = (locals.var_wsrhstep).ln();
            let assign15880_e13572: f64 = (assign15880_e13569 * assign15880_e13571);
            let assign15880_e13575: f64 = (1.0 - locals.var_wsrhstep);
            let assign15880_e13576: f64 = (assign15880_e13572 / assign15880_e13575);
            let assign15880_e13578: f64 = (assign15880_e13576 + locals.var_wsrhstep);
            let assign15880_e13582: f64 = (2.0 * p.p831);
            let assign15880_e13583: f64 = (1.0 - assign15880_e13582);
            let assign15880_e13584: f64 = (assign15880_e13578 * assign15880_e13583);
            locals.var_dwsrh = assign15880_e13584;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) {
            let assign15890_e13598: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign15890_e13598;
        }
        let assign15900_e13603: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign15900_e13603;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) && (locals.var_guard256 != 0.0)) {
            let assign15910_e13617: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign15910_e13618: f64 = (assign15910_e13617).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign15910_e13618, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) && (locals.var_guard256 == 0.0)) {
            let assign15920_e13635: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign15920_e13637: f64 = (assign15920_e13635).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign15920_e13637, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) {
            let assign15930_e13651: f64 = (locals.var_wdepnulrbot * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign15930_e13651, (locals.var_wdepnulrbot * locals.var_tmp_dn5), (locals.var_wdepnulrbot * locals.var_tmp_dn6), (locals.var_wdepnulrbot * locals.var_tmp_dn7), (locals.var_wdepnulrbot * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) {
            let assign15940_e13666: f64 = (locals.var_zinv - 1.0);
            let assign15940_e13668: f64 = (assign15940_e13666 * locals.var_wdep);
            let assign15940_e13669: f64 = (locals.var_ftdbot * assign15940_e13668);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign15940_e13669, (locals.var_ftdbot * (assign15940_e13666 * locals.var_wdep_dn5)), (locals.var_ftdbot * (assign15940_e13666 * locals.var_wdep_dn6)), (locals.var_ftdbot * (assign15940_e13666 * locals.var_wdep_dn7)), (locals.var_ftdbot * (assign15940_e13666 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) {
            let assign15950_e13684: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign15950_e13685: f64 = (p.p840 * assign15950_e13684);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign15950_e13685, (p.p840 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign15960_e13690: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign15960_e13690;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign15980_e13714: f64 = (locals.var_wdep * locals.var_one_minus_pbot);
            let assign15980_e13716: f64 = (assign15980_e13714 / locals.var_vbi_minus_vjsrh);
            let assign15980_e13717: f64 = (locals.var_btatpartbot * assign15980_e13716);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign15980_e13717, (locals.var_btatpartbot * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign15990_e13731: f64 = (0.666666666666667 * locals.var_atatbot);
            let assign15990_e13733: f64 = (assign15990_e13731 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign15990_e13733, (-((assign15990_e13731 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign15990_e13731 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign15990_e13731 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign15990_e13731 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16000_e13747: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign16000_e13747, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16010_e13761: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign16010_e13764: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign16010_e13766: f64 = (assign16010_e13764 + 1.0);
            let assign16010_e13767: f64 = (assign16010_e13761 / assign16010_e13766);
            let assign16010_e13768: f64 = (assign16010_e13767).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign16010_e13768, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign16010_e13766) - (assign16010_e13761 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign16010_e13766 * assign16010_e13766)) / (2.0 * assign16010_e13768)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign16010_e13766) - (assign16010_e13761 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign16010_e13766 * assign16010_e13766)) / (2.0 * assign16010_e13768)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign16010_e13766) - (assign16010_e13761 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign16010_e13766 * assign16010_e13766)) / (2.0 * assign16010_e13768)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign16010_e13766) - (assign16010_e13761 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign16010_e13766 * assign16010_e13766)) / (2.0 * assign16010_e13768)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16020_e13781: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign16020_e13781, (locals.var_umax_dn5 / (2.0 * assign16020_e13781)), (locals.var_umax_dn6 / (2.0 * assign16020_e13781)), (locals.var_umax_dn7 / (2.0 * assign16020_e13781)), (locals.var_umax_dn8 / (2.0 * assign16020_e13781)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16030_e13795: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign16030_e13795, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign16040_e13799: f64 = (-p.p831);
        let assign16040_e13801: f64 = (assign16040_e13799 * locals.var_one_over_one_minus_pbot);
        let assign16040_e13803: f64 = (-1.0);
        let assign16040_e13804: f64 = if assign16040_e13801 == assign16040_e13803 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign16040_e13804;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard258 != 0.0)) {
            let assign16050_e13820: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign16050_e13821: f64 = (1.0 + assign16050_e13820);
            let assign16050_e13822: f64 = (1.0 / assign16050_e13821);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign16050_e13822, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign16050_e13821 * assign16050_e13821))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign16050_e13821 * assign16050_e13821))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign16050_e13821 * assign16050_e13821))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign16050_e13821 * assign16050_e13821))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard258 == 0.0)) {
            let assign16060_e13840: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign16060_e13841: f64 = (1.0 + assign16060_e13840);
            let assign16060_e13843: f64 = (-p.p831);
            let assign16060_e13845: f64 = (assign16060_e13843 * locals.var_one_over_one_minus_pbot);
            let assign16060_e13846: f64 = (assign16060_e13841).powf(assign16060_e13845);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign16060_e13846, if 0.0 == 0.0 && ((assign16060_e13845) as f64).is_finite() && ((assign16060_e13845) as f64).fract() == 0.0 { if assign16060_e13845 == 0.0 { 0.0 } else { (assign16060_e13845 * ((assign16060_e13841).powf(assign16060_e13845 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign16060_e13846 * (assign16060_e13845 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign16060_e13841))) }, if 0.0 == 0.0 && ((assign16060_e13845) as f64).is_finite() && ((assign16060_e13845) as f64).fract() == 0.0 { if assign16060_e13845 == 0.0 { 0.0 } else { (assign16060_e13845 * ((assign16060_e13841).powf(assign16060_e13845 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign16060_e13846 * (assign16060_e13845 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign16060_e13841))) }, if 0.0 == 0.0 && ((assign16060_e13845) as f64).is_finite() && ((assign16060_e13845) as f64).fract() == 0.0 { if assign16060_e13845 == 0.0 { 0.0 } else { (assign16060_e13845 * ((assign16060_e13841).powf(assign16060_e13845 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign16060_e13846 * (assign16060_e13845 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign16060_e13841))) }, if 0.0 == 0.0 && ((assign16060_e13845) as f64).is_finite() && ((assign16060_e13845) as f64).fract() == 0.0 { if assign16060_e13845 == 0.0 { 0.0 } else { (assign16060_e13845 * ((assign16060_e13841).powf(assign16060_e13845 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign16060_e13846 * (assign16060_e13845 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign16060_e13841))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16070_e13860: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign16070_e13863: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign16070_e13864: f64 = (assign16070_e13860 / assign16070_e13863);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign16070_e13864, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign16070_e13863) - (assign16070_e13860 * locals.var_wgamma_dn5)) / (assign16070_e13863 * assign16070_e13863)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign16070_e13863) - (assign16070_e13860 * locals.var_wgamma_dn6)) / (assign16070_e13863 * assign16070_e13863)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign16070_e13863) - (assign16070_e13860 * locals.var_wgamma_dn7)) / (assign16070_e13863 * assign16070_e13863)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign16070_e13863) - (assign16070_e13860 * locals.var_wgamma_dn8)) / (assign16070_e13863 * assign16070_e13863)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16080_e13879: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign16080_e13880: f64 = (0.375 * assign16080_e13879);
            let assign16080_e13881: f64 = (assign16080_e13880).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign16080_e13881, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign16080_e13881)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign16080_e13881)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign16080_e13881)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign16080_e13881)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16090_e13896: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign16090_e13897: f64 = (2.0 * assign16090_e13896);
            let assign16090_e13899: f64 = (assign16090_e13897 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign16090_e13899, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16100_e13913: f64 = (locals.var_atatbot * locals.var_twoatatoverthreebtat);
            let assign16100_e13915: f64 = (assign16100_e13913 * locals.var_sqrtumax);
            let assign16100_e13918: f64 = (locals.var_atatbot * locals.var_umax);
            let assign16100_e13919: f64 = (assign16100_e13915 - assign16100_e13918);
            let assign16100_e13923: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign16100_e13924: f64 = (0.5 * assign16100_e13923);
            let assign16100_e13925: f64 = (assign16100_e13919 + assign16100_e13924);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign16100_e13925, (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign16100_e13913 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign16100_e13913 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign16100_e13913 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign16100_e13913 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16110_e13939: f64 = (locals.var_ltat - 1.0);
            let assign16110_e13941: f64 = (assign16110_e13939 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign16110_e13941, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign16110_e13939 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign16110_e13939 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign16110_e13939 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign16110_e13939 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16120_e13955: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign16120_e13955, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign16130_e13960: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign16130_e13960;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard259 != 0.0)) {
            let assign16140_e13976: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign16140_e13977: f64 = (1.0 + assign16140_e13976);
            let assign16140_e13978: f64 = (1.0 / assign16140_e13977);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign16140_e13978, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign16140_e13977 * assign16140_e13977))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign16140_e13977 * assign16140_e13977))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign16140_e13977 * assign16140_e13977))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign16140_e13977 * assign16140_e13977))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard259 == 0.0)) {
            let assign16150_e13997: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign16150_e13998: f64 = (1.0 - assign16150_e13997);
            let assign16150_e13999: f64 = (1.0 / assign16150_e13998);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign16150_e13999, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign16150_e13998 * assign16150_e13998))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign16150_e13998 * assign16150_e13998))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign16150_e13998 * assign16150_e13998))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign16150_e13998 * assign16150_e13998))), );
        }
        let assign16160_e14003: f64 = (-locals.var_ysq);
        let assign16160_e14005: f64 = (assign16160_e14003 + locals.var_mtat);
        let assign16160_e14007: f64 = (-230.25850929940458);
        let assign16160_e14008: f64 = if assign16160_e14005 > assign16160_e14007 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign16160_e14008;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard260 != 0.0)) {
            let assign16170_e14021: f64 = (-locals.var_ysq);
            let assign16170_e14023: f64 = (assign16170_e14021 + locals.var_mtat);
            let assign16170_e14024: f64 = (assign16170_e14023).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16170_e14024, (assign16170_e14024 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign16170_e14024 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign16170_e14024 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign16170_e14024 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
    }
}

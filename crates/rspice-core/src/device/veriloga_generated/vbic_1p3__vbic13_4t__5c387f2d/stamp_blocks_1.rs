#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let assign1470_e1831: f64 = (locals.var_vtv / locals.var_rt);
        let assign1470_e1832: f64 = (2.0 * assign1470_e1831);
        let assign1470_e1835: f64 = (0.5 * p.p50);
        let assign1470_e1837: f64 = (assign1470_e1835 * locals.var_rt);
        let assign1470_e1839: f64 = (assign1470_e1837 / locals.var_vtv);
        let assign1470_e1840: f64 = (assign1470_e1839).exp();
        let assign1470_e1842: f64 = (-0.5);
        let assign1470_e1844: f64 = (assign1470_e1842 * p.p50);
        let assign1470_e1846: f64 = (assign1470_e1844 * locals.var_rt);
        let assign1470_e1848: f64 = (assign1470_e1846 / locals.var_vtv);
        let assign1470_e1849: f64 = (assign1470_e1848).exp();
        let assign1470_e1850: f64 = (assign1470_e1840 - assign1470_e1849);
        let assign1470_e1851: f64 = (assign1470_e1850).ln();
        let assign1470_e1852: f64 = (assign1470_e1832 * assign1470_e1851);
        locals.var_psiio__blk39 = assign1470_e1852;
        locals.var_psiio__blk39_dn4 = (((2.0 * (((locals.var_vtv_dn4 * locals.var_rt) - (locals.var_vtv * locals.var_rt_dn4)) / (locals.var_rt * locals.var_rt))) * assign1470_e1851) + (assign1470_e1832 * (((assign1470_e1840 * ((((assign1470_e1835 * locals.var_rt_dn4) * locals.var_vtv) - (assign1470_e1837 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))) - (assign1470_e1849 * ((((assign1470_e1844 * locals.var_rt_dn4) * locals.var_vtv) - (assign1470_e1846 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / assign1470_e1850)));
        locals.var_psiio__blk39_rv = 0.0;

        let assign1480_e1855: f64 = (locals.var_psiio__blk39 * locals.var_rt);
        let assign1480_e1858: f64 = (3.0 * locals.var_vtv);
        let assign1480_e1860: f64 = (locals.var_rt).ln();
        let assign1480_e1861: f64 = (assign1480_e1858 * assign1480_e1860);
        let assign1480_e1862: f64 = (assign1480_e1855 - assign1480_e1861);
        let assign1480_e1866: f64 = (locals.var_rt - 1.0);
        let assign1480_e1867: f64 = (p.p116 * assign1480_e1866);
        let assign1480_e1868: f64 = (assign1480_e1862 - assign1480_e1867);
        locals.var_psiin__blk40 = assign1480_e1868;
        locals.var_psiin__blk40_dn4 = ((((locals.var_psiio__blk39_dn4 * locals.var_rt) + (locals.var_psiio__blk39 * locals.var_rt_dn4)) - (((3.0 * locals.var_vtv_dn4) * assign1480_e1860) + (assign1480_e1858 * (locals.var_rt_dn4 / locals.var_rt)))) - (p.p116 * locals.var_rt_dn4));
        locals.var_psiin__blk40_rv = 0.0;

        let assign1490_e1872: f64 = (2.0 * locals.var_vtv);
        let assign1490_e1878: f64 = (-locals.var_psiin__blk40);
        let assign1490_e1880: f64 = (assign1490_e1878 / locals.var_vtv);
        let assign1490_e1881: f64 = (assign1490_e1880).exp();
        let assign1490_e1882: f64 = (4.0 * assign1490_e1881);
        let assign1490_e1883: f64 = (1.0 + assign1490_e1882);
        let assign1490_e1884: f64 = (assign1490_e1883).sqrt();
        let assign1490_e1885: f64 = (1.0 + assign1490_e1884);
        let assign1490_e1886: f64 = (0.5 * assign1490_e1885);
        let assign1490_e1887: f64 = (assign1490_e1886).ln();
        let assign1490_e1888: f64 = (assign1490_e1872 * assign1490_e1887);
        let assign1490_e1889: f64 = (locals.var_psiin__blk40 + assign1490_e1888);
        locals.var_ps_t = assign1490_e1889;
        locals.var_ps_t_dn4 = (locals.var_psiin__blk40_dn4 + (((2.0 * locals.var_vtv_dn4) * assign1490_e1887) + (assign1490_e1872 * ((0.5 * ((4.0 * (assign1490_e1881 * ((((-locals.var_psiin__blk40_dn4) * locals.var_vtv) - (assign1490_e1878 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / (2.0 * assign1490_e1884))) / assign1490_e1886))));
        locals.var_ps_t_rv = 0.0;

        let assign1500_e1893: f64 = (p.p37 / locals.var_pe_t);
        let assign1500_e1895: f64 = (assign1500_e1893).powf(p.p38);
        let assign1500_e1896: f64 = (p.p36 * assign1500_e1895);
        locals.var_cje_t = assign1500_e1896;
        locals.var_cje_t_dn4 = (p.p36 * if 0.0 == 0.0 && ((p.p38) as f64).is_finite() && ((p.p38) as f64).fract() == 0.0 { if p.p38 == 0.0 { 0.0 } else { (p.p38 * ((assign1500_e1893).powf(p.p38 - 1.0) * (-((p.p37 * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign1500_e1895 * (p.p38 * ((-((p.p37 * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t))) / assign1500_e1893))) });
        locals.var_cje_t_rv = 0.0;

        let assign1510_e1900: f64 = (p.p42 / locals.var_pc_t);
        let assign1510_e1902: f64 = (assign1510_e1900).powf(p.p43);
        let assign1510_e1903: f64 = (p.p41 * assign1510_e1902);
        locals.var_cjc_t = assign1510_e1903;
        locals.var_cjc_t_dn4 = (p.p41 * if 0.0 == 0.0 && ((p.p43) as f64).is_finite() && ((p.p43) as f64).fract() == 0.0 { if p.p43 == 0.0 { 0.0 } else { (p.p43 * ((assign1510_e1900).powf(p.p43 - 1.0) * (-((p.p42 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign1510_e1902 * (p.p43 * ((-((p.p42 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign1510_e1900))) });
        locals.var_cjc_t_rv = 0.0;

        let assign1520_e1907: f64 = (p.p42 / locals.var_pc_t);
        let assign1520_e1909: f64 = (assign1520_e1907).powf(p.p43);
        let assign1520_e1910: f64 = (p.p48 * assign1520_e1909);
        locals.var_cjep_t = assign1520_e1910;
        locals.var_cjep_t_dn4 = (p.p48 * if 0.0 == 0.0 && ((p.p43) as f64).is_finite() && ((p.p43) as f64).fract() == 0.0 { if p.p43 == 0.0 { 0.0 } else { (p.p43 * ((assign1520_e1907).powf(p.p43 - 1.0) * (-((p.p42 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign1520_e1909 * (p.p43 * ((-((p.p42 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign1520_e1907))) });
        locals.var_cjep_t_rv = 0.0;

        let assign1530_e1914: f64 = (p.p50 / locals.var_ps_t);
        let assign1530_e1916: f64 = (assign1530_e1914).powf(p.p51);
        let assign1530_e1917: f64 = (p.p49 * assign1530_e1916);
        locals.var_cjcp_t = assign1530_e1917;
        locals.var_cjcp_t_dn4 = (p.p49 * if 0.0 == 0.0 && ((p.p51) as f64).is_finite() && ((p.p51) as f64).fract() == 0.0 { if p.p51 == 0.0 { 0.0 } else { (p.p51 * ((assign1530_e1914).powf(p.p51 - 1.0) * (-((p.p50 * locals.var_ps_t_dn4) / (locals.var_ps_t * locals.var_ps_t))))) } } else { (assign1530_e1916 * (p.p51 * ((-((p.p50 * locals.var_ps_t_dn4) / (locals.var_ps_t * locals.var_ps_t))) / assign1530_e1914))) });
        locals.var_cjcp_t_rv = 0.0;

        let assign1540_e1921: f64 = (locals.var_rt).powf(p.p122);
        let assign1540_e1922: f64 = (p.p19 * assign1540_e1921);
        let assign1540_e1924: f64 = (-p.p113);
        let assign1540_e1927: f64 = (1.0 - locals.var_rt);
        let assign1540_e1928: f64 = (assign1540_e1924 * assign1540_e1927);
        let assign1540_e1930: f64 = (assign1540_e1928 / locals.var_vtv);
        let assign1540_e1931: f64 = (assign1540_e1930).exp();
        let assign1540_e1932: f64 = (assign1540_e1922 * assign1540_e1931);
        locals.var_gamm_t = assign1540_e1932;
        locals.var_gamm_t_dn4 = (((p.p19 * if 0.0 == 0.0 && ((p.p122) as f64).is_finite() && ((p.p122) as f64).fract() == 0.0 { if p.p122 == 0.0 { 0.0 } else { (p.p122 * ((locals.var_rt).powf(p.p122 - 1.0) * locals.var_rt_dn4)) } } else { (assign1540_e1921 * (p.p122 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1540_e1931) + (assign1540_e1922 * (assign1540_e1931 * ((((assign1540_e1924 * (-locals.var_rt_dn4)) * locals.var_vtv) - (assign1540_e1928 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))));
        locals.var_gamm_t_rv = 0.0;

        let assign1570_e1949: f64 = (locals.var_dt * p.p130);
        let assign1570_e1950: f64 = (1.0 + assign1570_e1949);
        let assign1570_e1951: f64 = (p.p70 * assign1570_e1950);
        locals.var_vef_t = assign1570_e1951;
        locals.var_vef_t_dn4 = (p.p70 * (locals.var_dt_dn4 * p.p130));
        locals.var_vef_t_rv = 0.0;

        let assign1580_e1956: f64 = (locals.var_dt * p.p131);
        let assign1580_e1957: f64 = (1.0 + assign1580_e1956);
        let assign1580_e1958: f64 = (p.p71 * assign1580_e1957);
        locals.var_ver_t = assign1580_e1958;
        locals.var_ver_t_dn4 = (p.p71 * (locals.var_dt_dn4 * p.p131));
        locals.var_ver_t_rv = 0.0;

        let (assign1670_e2030, assign1670_e2030_d_n4,) = {
    if (locals.var_vef_t > 0.0) {
        let assign1670_e2028: f64 = (1.0 / locals.var_vef_t);
        (assign1670_e2028, (-(locals.var_vef_t_dn4 / (locals.var_vef_t * locals.var_vef_t))),)
    } else {
        (0.0, 0.0,)
    }
};
        locals.var_ivef = assign1670_e2030;
        locals.var_ivef_dn4 = assign1670_e2030_d_n4;
        locals.var_ivef_rv = 0.0;

        let (assign1680_e2038, assign1680_e2038_d_n4,) = {
    if (locals.var_ver_t > 0.0) {
        let assign1680_e2036: f64 = (1.0 / locals.var_ver_t);
        (assign1680_e2036, (-(locals.var_ver_t_dn4 / (locals.var_ver_t * locals.var_ver_t))),)
    } else {
        (0.0, 0.0,)
    }
};
        locals.var_iver = assign1680_e2038;
        locals.var_iver_dn4 = assign1680_e2038_d_n4;
        locals.var_iver_rv = 0.0;

        let (assign1690_e2046, assign1690_e2046_d_n4,) = {
    if (locals.var_ikf_t > 0.0) {
        let assign1690_e2044: f64 = (1.0 / locals.var_ikf_t);
        (assign1690_e2044, (-(locals.var_ikf_t_dn4 / (locals.var_ikf_t * locals.var_ikf_t))),)
    } else {
        (0.0, 0.0,)
    }
};
        locals.var_iikf = assign1690_e2046;
        locals.var_iikf_dn4 = assign1690_e2046_d_n4;
        locals.var_iikf_rv = 0.0;

        let assign1710_e2057: f64 = (locals.var_vbictype * (nv8 - nv9));
        locals.var_vbei = assign1710_e2057;
        locals.var_vbei_dn8 = locals.var_vbictype;
        locals.var_vbei_dn9 = (-locals.var_vbictype);
        locals.var_vbei_rv = 0.0;

        let assign1720_e2060: f64 = (locals.var_vbictype * (nv7 - nv9));
        locals.var_vbex = assign1720_e2060;
        locals.var_vbex_dn7 = locals.var_vbictype;
        locals.var_vbex_dn9 = (-locals.var_vbictype);
        locals.var_vbex_rv = 0.0;

        let assign1730_e2063: f64 = (locals.var_vbictype * (nv8 - nv6));
        locals.var_vbci = assign1730_e2063;
        locals.var_vbci_dn6 = (-locals.var_vbictype);
        locals.var_vbci_dn8 = locals.var_vbictype;
        locals.var_vbci_rv = 0.0;

        let assign1740_e2066: f64 = (locals.var_vbictype * (nv8 - nv5));
        locals.var_vbcx = assign1740_e2066;
        locals.var_vbcx_dn5 = (-locals.var_vbictype);
        locals.var_vbcx_dn8 = locals.var_vbictype;
        locals.var_vbcx_rv = 0.0;

        let assign1760_e2072: f64 = (locals.var_vbictype * (nv7 - nv10));
        locals.var_vbep = assign1760_e2072;
        locals.var_vbep_dn7 = locals.var_vbictype;
        locals.var_vbep_dn10 = (-locals.var_vbictype);
        locals.var_vbep_rv = 0.0;

        let assign1860_e2088: f64 = (locals.var_vbictype * (nv11 - nv10));
        locals.var_vbcp = assign1860_e2088;
        locals.var_vbcp_dn10 = (-locals.var_vbictype);
        locals.var_vbcp_dn11 = locals.var_vbictype;
        locals.var_vbcp_rv = 0.0;

        let assign1910_e2096: f64 = (-locals.var_pe_t);
        let assign1910_e2098: f64 = (assign1910_e2096 * p.p34);
        locals.var_dv0 = assign1910_e2098;
        locals.var_dv0_dn4 = ((-locals.var_pe_t_dn4) * p.p34);
        locals.var_dv0_rv = 0.0;

        let assign1920_e2101: f64 = if p.p39 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign1920_e2101;
        locals.var_guard52_rv = 0.0;

        let (assign1930_e2107, assign1930_e2107_d_n4, assign1930_e2107_d_n8, assign1930_e2107_d_n9,) = {
    if (locals.var_guard52 != 0.0) {
        let assign1930_e2105: f64 = (locals.var_vbei + locals.var_dv0);
        (assign1930_e2105, locals.var_dv0_dn4, locals.var_vbei_dn8, locals.var_vbei_dn9,)
    } else {
        (locals.var_dvh, locals.var_dvh_dn4, locals.var_dvh_dn8, locals.var_dvh_dn9,)
    }
};
        locals.var_dvh = assign1930_e2107;
        locals.var_dvh_dn4 = assign1930_e2107_d_n4;
        locals.var_dvh_dn8 = assign1930_e2107_d_n8;
        locals.var_dvh_dn9 = assign1930_e2107_d_n9;
        locals.var_dvh_rv = 0.0;

        let assign1940_e2110: f64 = if locals.var_dvh > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign1940_e2110;
        locals.var_guard53_rv = 0.0;

        let (assign1950_e2121,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign1950_e2116: f64 = (1.0 - p.p34);
        let assign1950_e2118: f64 = (-p.p38);
        let assign1950_e2119: f64 = (assign1950_e2116).powf(assign1950_e2118);
        (assign1950_e2119,)
    } else {
        (locals.var_pwq,)
    }
};
        locals.var_pwq = assign1950_e2121;
        locals.var_pwq_rv = 0.0;

        let (assign1960_e2139, assign1960_e2139_d_n4, assign1960_e2139_d_n8, assign1960_e2139_d_n9,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign1960_e2130: f64 = (1.0 - p.p34);
        let assign1960_e2131: f64 = (locals.var_pwq * assign1960_e2130);
        let assign1960_e2132: f64 = (1.0 - assign1960_e2131);
        let assign1960_e2133: f64 = (locals.var_pe_t * assign1960_e2132);
        let assign1960_e2136: f64 = (1.0 - p.p38);
        let assign1960_e2137: f64 = (assign1960_e2133 / assign1960_e2136);
        (assign1960_e2137, ((locals.var_pe_t_dn4 * assign1960_e2132) / assign1960_e2136), 0.0, 0.0,)
    } else {
        (locals.var_qlo, locals.var_qlo_dn4, locals.var_qlo_dn8, locals.var_qlo_dn9,)
    }
};
        locals.var_qlo = assign1960_e2139;
        locals.var_qlo_dn4 = assign1960_e2139_d_n4;
        locals.var_qlo_dn8 = assign1960_e2139_d_n8;
        locals.var_qlo_dn9 = assign1960_e2139_d_n9;
        locals.var_qlo_rv = 0.0;

        let (assign1970_e2161, assign1970_e2161_d_n4, assign1970_e2161_d_n8, assign1970_e2161_d_n9,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign1970_e2147: f64 = (0.5 * p.p38);
        let assign1970_e2149: f64 = (assign1970_e2147 * locals.var_dvh);
        let assign1970_e2153: f64 = (1.0 - p.p34);
        let assign1970_e2154: f64 = (locals.var_pe_t * assign1970_e2153);
        let assign1970_e2155: f64 = (assign1970_e2149 / assign1970_e2154);
        let assign1970_e2156: f64 = (1.0 + assign1970_e2155);
        let assign1970_e2157: f64 = (locals.var_dvh * assign1970_e2156);
        let assign1970_e2159: f64 = (assign1970_e2157 * locals.var_pwq);
        (assign1970_e2159, (((locals.var_dvh_dn4 * assign1970_e2156) + (locals.var_dvh * ((((assign1970_e2147 * locals.var_dvh_dn4) * assign1970_e2154) - (assign1970_e2149 * (locals.var_pe_t_dn4 * assign1970_e2153))) / (assign1970_e2154 * assign1970_e2154)))) * locals.var_pwq), (((locals.var_dvh_dn8 * assign1970_e2156) + (locals.var_dvh * ((assign1970_e2147 * locals.var_dvh_dn8) / assign1970_e2154))) * locals.var_pwq), (((locals.var_dvh_dn9 * assign1970_e2156) + (locals.var_dvh * ((assign1970_e2147 * locals.var_dvh_dn9) / assign1970_e2154))) * locals.var_pwq),)
    } else {
        (locals.var_qhi, locals.var_qhi_dn4, locals.var_qhi_dn8, locals.var_qhi_dn9,)
    }
};
        locals.var_qhi = assign1970_e2161;
        locals.var_qhi_dn4 = assign1970_e2161_d_n4;
        locals.var_qhi_dn8 = assign1970_e2161_d_n8;
        locals.var_qhi_dn9 = assign1970_e2161_d_n9;
        locals.var_qhi_rv = 0.0;

        let (assign1980_e2184, assign1980_e2184_d_n4, assign1980_e2184_d_n8, assign1980_e2184_d_n9,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 == 0.0)) {
        let assign1980_e2171: f64 = (locals.var_vbei / locals.var_pe_t);
        let assign1980_e2172: f64 = (1.0 - assign1980_e2171);
        let assign1980_e2175: f64 = (1.0 - p.p38);
        let assign1980_e2176: f64 = (assign1980_e2172).powf(assign1980_e2175);
        let assign1980_e2177: f64 = (1.0 - assign1980_e2176);
        let assign1980_e2178: f64 = (locals.var_pe_t * assign1980_e2177);
        let assign1980_e2181: f64 = (1.0 - p.p38);
        let assign1980_e2182: f64 = (assign1980_e2178 / assign1980_e2181);
        (assign1980_e2182, (((locals.var_pe_t_dn4 * assign1980_e2177) + (locals.var_pe_t * (-if 0.0 == 0.0 && ((assign1980_e2175) as f64).is_finite() && ((assign1980_e2175) as f64).fract() == 0.0 { if assign1980_e2175 == 0.0 { 0.0 } else { (assign1980_e2175 * ((assign1980_e2172).powf(assign1980_e2175 - 1.0) * (-(-((locals.var_vbei * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t)))))) } } else { (assign1980_e2176 * (assign1980_e2175 * ((-(-((locals.var_vbei * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t)))) / assign1980_e2172))) }))) / assign1980_e2181), ((locals.var_pe_t * (-if 0.0 == 0.0 && ((assign1980_e2175) as f64).is_finite() && ((assign1980_e2175) as f64).fract() == 0.0 { if assign1980_e2175 == 0.0 { 0.0 } else { (assign1980_e2175 * ((assign1980_e2172).powf(assign1980_e2175 - 1.0) * (-(locals.var_vbei_dn8 / locals.var_pe_t)))) } } else { (assign1980_e2176 * (assign1980_e2175 * ((-(locals.var_vbei_dn8 / locals.var_pe_t)) / assign1980_e2172))) })) / assign1980_e2181), ((locals.var_pe_t * (-if 0.0 == 0.0 && ((assign1980_e2175) as f64).is_finite() && ((assign1980_e2175) as f64).fract() == 0.0 { if assign1980_e2175 == 0.0 { 0.0 } else { (assign1980_e2175 * ((assign1980_e2172).powf(assign1980_e2175 - 1.0) * (-(locals.var_vbei_dn9 / locals.var_pe_t)))) } } else { (assign1980_e2176 * (assign1980_e2175 * ((-(locals.var_vbei_dn9 / locals.var_pe_t)) / assign1980_e2172))) })) / assign1980_e2181),)
    } else {
        (locals.var_qlo, locals.var_qlo_dn4, locals.var_qlo_dn8, locals.var_qlo_dn9,)
    }
};
        locals.var_qlo = assign1980_e2184;
        locals.var_qlo_dn4 = assign1980_e2184_d_n4;
        locals.var_qlo_dn8 = assign1980_e2184_d_n8;
        locals.var_qlo_dn9 = assign1980_e2184_d_n9;
        locals.var_qlo_rv = 0.0;

        let (assign1990_e2191, assign1990_e2191_d_n4, assign1990_e2191_d_n8, assign1990_e2191_d_n9,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi, locals.var_qhi_dn4, locals.var_qhi_dn8, locals.var_qhi_dn9,)
    }
};
        locals.var_qhi = assign1990_e2191;
        locals.var_qhi_dn4 = assign1990_e2191_d_n4;
        locals.var_qhi_dn8 = assign1990_e2191_d_n8;
        locals.var_qhi_dn9 = assign1990_e2191_d_n9;
        locals.var_qhi_rv = 0.0;

        let (assign2000_e2197, assign2000_e2197_d_n4, assign2000_e2197_d_n8, assign2000_e2197_d_n9,) = {
    if (locals.var_guard52 != 0.0) {
        let assign2000_e2195: f64 = (locals.var_qlo + locals.var_qhi);
        (assign2000_e2195, (locals.var_qlo_dn4 + locals.var_qhi_dn4), (locals.var_qlo_dn8 + locals.var_qhi_dn8), (locals.var_qlo_dn9 + locals.var_qhi_dn9),)
    } else {
        (locals.var_qdbe, locals.var_qdbe_dn4, locals.var_qdbe_dn8, locals.var_qdbe_dn9,)
    }
};
        locals.var_qdbe = assign2000_e2197;
        locals.var_qdbe_dn4 = assign2000_e2197_d_n4;
        locals.var_qdbe_dn8 = assign2000_e2197_d_n8;
        locals.var_qdbe_dn9 = assign2000_e2197_d_n9;
        locals.var_qdbe_rv = 0.0;

        let (assign2010_e2211, assign2010_e2211_d_n4,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2010_e2202: f64 = (locals.var_dv0 * locals.var_dv0);
        let assign2010_e2205: f64 = (4.0 * p.p39);
        let assign2010_e2207: f64 = (assign2010_e2205 * p.p39);
        let assign2010_e2208: f64 = (assign2010_e2202 + assign2010_e2207);
        let assign2010_e2209: f64 = (assign2010_e2208).sqrt();
        (assign2010_e2209, (((locals.var_dv0_dn4 * locals.var_dv0) + (locals.var_dv0 * locals.var_dv0_dn4)) / (2.0 * assign2010_e2209)),)
    } else {
        (locals.var_mv0, locals.var_mv0_dn4,)
    }
};
        locals.var_mv0 = assign2010_e2211;
        locals.var_mv0_dn4 = assign2010_e2211_d_n4;
        locals.var_mv0_rv = 0.0;

        let (assign2020_e2221, assign2020_e2221_d_n4,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2020_e2215: f64 = (-0.5);
        let assign2020_e2218: f64 = (locals.var_dv0 + locals.var_mv0);
        let assign2020_e2219: f64 = (assign2020_e2215 * assign2020_e2218);
        (assign2020_e2219, (assign2020_e2215 * (locals.var_dv0_dn4 + locals.var_mv0_dn4)),)
    } else {
        (locals.var_vl0, locals.var_vl0_dn4,)
    }
};
        locals.var_vl0 = assign2020_e2221;
        locals.var_vl0_dn4 = assign2020_e2221_d_n4;
        locals.var_vl0_rv = 0.0;

        let (assign2030_e2241, assign2030_e2241_d_n4,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2030_e2225: f64 = (-locals.var_pe_t);
        let assign2030_e2229: f64 = (locals.var_vl0 / locals.var_pe_t);
        let assign2030_e2230: f64 = (1.0 - assign2030_e2229);
        let assign2030_e2233: f64 = (1.0 - p.p38);
        let assign2030_e2234: f64 = (assign2030_e2230).powf(assign2030_e2233);
        let assign2030_e2235: f64 = (assign2030_e2225 * assign2030_e2234);
        let assign2030_e2238: f64 = (1.0 - p.p38);
        let assign2030_e2239: f64 = (assign2030_e2235 / assign2030_e2238);
        (assign2030_e2239, ((((-locals.var_pe_t_dn4) * assign2030_e2234) + (assign2030_e2225 * if 0.0 == 0.0 && ((assign2030_e2233) as f64).is_finite() && ((assign2030_e2233) as f64).fract() == 0.0 { if assign2030_e2233 == 0.0 { 0.0 } else { (assign2030_e2233 * ((assign2030_e2230).powf(assign2030_e2233 - 1.0) * (-(((locals.var_vl0_dn4 * locals.var_pe_t) - (locals.var_vl0 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign2030_e2234 * (assign2030_e2233 * ((-(((locals.var_vl0_dn4 * locals.var_pe_t) - (locals.var_vl0 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))) / assign2030_e2230))) })) / assign2030_e2238),)
    } else {
        (locals.var_q0, locals.var_q0_dn4,)
    }
};
        locals.var_q0 = assign2030_e2241;
        locals.var_q0_dn4 = assign2030_e2241_d_n4;
        locals.var_q0_rv = 0.0;

        let (assign2040_e2248, assign2040_e2248_d_n4, assign2040_e2248_d_n8, assign2040_e2248_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2040_e2246: f64 = (locals.var_vbei + locals.var_dv0);
        (assign2040_e2246, locals.var_dv0_dn4, locals.var_vbei_dn8, locals.var_vbei_dn9,)
    } else {
        (locals.var_dv, locals.var_dv_dn4, locals.var_dv_dn8, locals.var_dv_dn9,)
    }
};
        locals.var_dv = assign2040_e2248;
        locals.var_dv_dn4 = assign2040_e2248_d_n4;
        locals.var_dv_dn8 = assign2040_e2248_d_n8;
        locals.var_dv_dn9 = assign2040_e2248_d_n9;
        locals.var_dv_rv = 0.0;

        let (assign2050_e2262, assign2050_e2262_d_n4, assign2050_e2262_d_n8, assign2050_e2262_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2050_e2253: f64 = (locals.var_dv * locals.var_dv);
        let assign2050_e2256: f64 = (4.0 * p.p39);
        let assign2050_e2258: f64 = (assign2050_e2256 * p.p39);
        let assign2050_e2259: f64 = (assign2050_e2253 + assign2050_e2258);
        let assign2050_e2260: f64 = (assign2050_e2259).sqrt();
        (assign2050_e2260, (((locals.var_dv_dn4 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn4)) / (2.0 * assign2050_e2260)), (((locals.var_dv_dn8 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn8)) / (2.0 * assign2050_e2260)), (((locals.var_dv_dn9 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn9)) / (2.0 * assign2050_e2260)),)
    } else {
        (locals.var_mv, locals.var_mv_dn4, locals.var_mv_dn8, locals.var_mv_dn9,)
    }
};
        locals.var_mv = assign2050_e2262;
        locals.var_mv_dn4 = assign2050_e2262_d_n4;
        locals.var_mv_dn8 = assign2050_e2262_d_n8;
        locals.var_mv_dn9 = assign2050_e2262_d_n9;
        locals.var_mv_rv = 0.0;

        let (assign2060_e2273, assign2060_e2273_d_n4, assign2060_e2273_d_n8, assign2060_e2273_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2060_e2268: f64 = (locals.var_dv - locals.var_mv);
        let assign2060_e2269: f64 = (0.5 * assign2060_e2268);
        let assign2060_e2271: f64 = (assign2060_e2269 - locals.var_dv0);
        (assign2060_e2271, ((0.5 * (locals.var_dv_dn4 - locals.var_mv_dn4)) - locals.var_dv0_dn4), (0.5 * (locals.var_dv_dn8 - locals.var_mv_dn8)), (0.5 * (locals.var_dv_dn9 - locals.var_mv_dn9)),)
    } else {
        (locals.var_vl, locals.var_vl_dn4, locals.var_vl_dn8, locals.var_vl_dn9,)
    }
};
        locals.var_vl = assign2060_e2273;
        locals.var_vl_dn4 = assign2060_e2273_d_n4;
        locals.var_vl_dn8 = assign2060_e2273_d_n8;
        locals.var_vl_dn9 = assign2060_e2273_d_n9;
        locals.var_vl_rv = 0.0;

        let (assign2070_e2293, assign2070_e2293_d_n4, assign2070_e2293_d_n8, assign2070_e2293_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2070_e2277: f64 = (-locals.var_pe_t);
        let assign2070_e2281: f64 = (locals.var_vl / locals.var_pe_t);
        let assign2070_e2282: f64 = (1.0 - assign2070_e2281);
        let assign2070_e2285: f64 = (1.0 - p.p38);
        let assign2070_e2286: f64 = (assign2070_e2282).powf(assign2070_e2285);
        let assign2070_e2287: f64 = (assign2070_e2277 * assign2070_e2286);
        let assign2070_e2290: f64 = (1.0 - p.p38);
        let assign2070_e2291: f64 = (assign2070_e2287 / assign2070_e2290);
        (assign2070_e2291, ((((-locals.var_pe_t_dn4) * assign2070_e2286) + (assign2070_e2277 * if 0.0 == 0.0 && ((assign2070_e2285) as f64).is_finite() && ((assign2070_e2285) as f64).fract() == 0.0 { if assign2070_e2285 == 0.0 { 0.0 } else { (assign2070_e2285 * ((assign2070_e2282).powf(assign2070_e2285 - 1.0) * (-(((locals.var_vl_dn4 * locals.var_pe_t) - (locals.var_vl * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign2070_e2286 * (assign2070_e2285 * ((-(((locals.var_vl_dn4 * locals.var_pe_t) - (locals.var_vl * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))) / assign2070_e2282))) })) / assign2070_e2290), ((assign2070_e2277 * if 0.0 == 0.0 && ((assign2070_e2285) as f64).is_finite() && ((assign2070_e2285) as f64).fract() == 0.0 { if assign2070_e2285 == 0.0 { 0.0 } else { (assign2070_e2285 * ((assign2070_e2282).powf(assign2070_e2285 - 1.0) * (-(locals.var_vl_dn8 / locals.var_pe_t)))) } } else { (assign2070_e2286 * (assign2070_e2285 * ((-(locals.var_vl_dn8 / locals.var_pe_t)) / assign2070_e2282))) }) / assign2070_e2290), ((assign2070_e2277 * if 0.0 == 0.0 && ((assign2070_e2285) as f64).is_finite() && ((assign2070_e2285) as f64).fract() == 0.0 { if assign2070_e2285 == 0.0 { 0.0 } else { (assign2070_e2285 * ((assign2070_e2282).powf(assign2070_e2285 - 1.0) * (-(locals.var_vl_dn9 / locals.var_pe_t)))) } } else { (assign2070_e2286 * (assign2070_e2285 * ((-(locals.var_vl_dn9 / locals.var_pe_t)) / assign2070_e2282))) }) / assign2070_e2290),)
    } else {
        (locals.var_qlo, locals.var_qlo_dn4, locals.var_qlo_dn8, locals.var_qlo_dn9,)
    }
};
        locals.var_qlo = assign2070_e2293;
        locals.var_qlo_dn4 = assign2070_e2293_d_n4;
        locals.var_qlo_dn8 = assign2070_e2293_d_n8;
        locals.var_qlo_dn9 = assign2070_e2293_d_n9;
        locals.var_qlo_rv = 0.0;

        let (assign2080_e2331, assign2080_e2331_d_n4, assign2080_e2331_d_n8, assign2080_e2331_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2080_e2299: f64 = (1.0 - p.p34);
        let assign2080_e2301: f64 = (-p.p38);
        let assign2080_e2302: f64 = (assign2080_e2299).powf(assign2080_e2301);
        let assign2080_e2305: f64 = (locals.var_vbei - locals.var_vl);
        let assign2080_e2307: f64 = (assign2080_e2305 + locals.var_vl0);
        let assign2080_e2308: f64 = (assign2080_e2302 * assign2080_e2307);
        let assign2080_e2312: f64 = (0.5 * p.p38);
        let assign2080_e2315: f64 = (locals.var_vbei - locals.var_vl);
        let assign2080_e2317: f64 = (assign2080_e2315 + locals.var_vl0);
        let assign2080_e2318: f64 = (assign2080_e2312 * assign2080_e2317);
        let assign2080_e2322: f64 = (1.0 - p.p34);
        let assign2080_e2323: f64 = (locals.var_pe_t * assign2080_e2322);
        let assign2080_e2324: f64 = (assign2080_e2318 / assign2080_e2323);
        let assign2080_e2325: f64 = (1.0 + assign2080_e2324);
        let assign2080_e2326: f64 = (assign2080_e2308 * assign2080_e2325);
        let assign2080_e2327: f64 = (locals.var_qlo + assign2080_e2326);
        let assign2080_e2329: f64 = (assign2080_e2327 - locals.var_q0);
        (assign2080_e2329, ((locals.var_qlo_dn4 + (((assign2080_e2302 * ((-locals.var_vl_dn4) + locals.var_vl0_dn4)) * assign2080_e2325) + (assign2080_e2308 * ((((assign2080_e2312 * ((-locals.var_vl_dn4) + locals.var_vl0_dn4)) * assign2080_e2323) - (assign2080_e2318 * (locals.var_pe_t_dn4 * assign2080_e2322))) / (assign2080_e2323 * assign2080_e2323))))) - locals.var_q0_dn4), (locals.var_qlo_dn8 + (((assign2080_e2302 * (locals.var_vbei_dn8 - locals.var_vl_dn8)) * assign2080_e2325) + (assign2080_e2308 * ((assign2080_e2312 * (locals.var_vbei_dn8 - locals.var_vl_dn8)) / assign2080_e2323)))), (locals.var_qlo_dn9 + (((assign2080_e2302 * (locals.var_vbei_dn9 - locals.var_vl_dn9)) * assign2080_e2325) + (assign2080_e2308 * ((assign2080_e2312 * (locals.var_vbei_dn9 - locals.var_vl_dn9)) / assign2080_e2323)))),)
    } else {
        (locals.var_qdbe, locals.var_qdbe_dn4, locals.var_qdbe_dn8, locals.var_qdbe_dn9,)
    }
};
        locals.var_qdbe = assign2080_e2331;
        locals.var_qdbe_dn4 = assign2080_e2331_d_n4;
        locals.var_qdbe_dn8 = assign2080_e2331_d_n8;
        locals.var_qdbe_dn9 = assign2080_e2331_d_n9;
        locals.var_qdbe_rv = 0.0;

        let assign2090_e2333: f64 = (-locals.var_pc_t);
        let assign2090_e2335: f64 = (assign2090_e2333 * p.p34);
        locals.var_dv0__blk54 = assign2090_e2335;
        locals.var_dv0__blk54_dn4 = ((-locals.var_pc_t_dn4) * p.p34);
        locals.var_dv0__blk54_rv = 0.0;

        let assign2100_e2338: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign2100_e2338;
        locals.var_guard75_rv = 0.0;

        let (assign2110_e2344, assign2110_e2344_d_n4, assign2110_e2344_d_n6, assign2110_e2344_d_n8,) = {
    if (locals.var_guard75 != 0.0) {
        let assign2110_e2342: f64 = (locals.var_vbci + locals.var_dv0__blk54);
        (assign2110_e2342, locals.var_dv0__blk54_dn4, locals.var_vbci_dn6, locals.var_vbci_dn8,)
    } else {
        (locals.var_dvh__blk55, locals.var_dvh__blk55_dn4, locals.var_dvh__blk55_dn6, locals.var_dvh__blk55_dn8,)
    }
};
        locals.var_dvh__blk55 = assign2110_e2344;
        locals.var_dvh__blk55_dn4 = assign2110_e2344_d_n4;
        locals.var_dvh__blk55_dn6 = assign2110_e2344_d_n6;
        locals.var_dvh__blk55_dn8 = assign2110_e2344_d_n8;
        locals.var_dvh__blk55_rv = 0.0;

        let assign2120_e2347: f64 = if locals.var_dvh__blk55 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign2120_e2347;
        locals.var_guard76_rv = 0.0;

        let (assign2130_e2360,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign2130_e2353: f64 = (1.0 - p.p34);
        let assign2130_e2355: f64 = (-1.0);
        let assign2130_e2357: f64 = (assign2130_e2355 - p.p43);
        let assign2130_e2358: f64 = (assign2130_e2353).powf(assign2130_e2357);
        (assign2130_e2358,)
    } else {
        (locals.var_pwq__blk56,)
    }
};
        locals.var_pwq__blk56 = assign2130_e2360;
        locals.var_pwq__blk56_rv = 0.0;

        let (assign2140_e2382, assign2140_e2382_d_n4, assign2140_e2382_d_n6, assign2140_e2382_d_n8,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign2140_e2369: f64 = (1.0 - p.p34);
        let assign2140_e2370: f64 = (locals.var_pwq__blk56 * assign2140_e2369);
        let assign2140_e2373: f64 = (1.0 - p.p34);
        let assign2140_e2374: f64 = (assign2140_e2370 * assign2140_e2373);
        let assign2140_e2375: f64 = (1.0 - assign2140_e2374);
        let assign2140_e2376: f64 = (locals.var_pc_t * assign2140_e2375);
        let assign2140_e2379: f64 = (1.0 - p.p43);
        let assign2140_e2380: f64 = (assign2140_e2376 / assign2140_e2379);
        (assign2140_e2380, ((locals.var_pc_t_dn4 * assign2140_e2375) / assign2140_e2379), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2140_e2382;
        locals.var_qlo__blk57_dn4 = assign2140_e2382_d_n4;
        locals.var_qlo__blk57_dn6 = assign2140_e2382_d_n6;
        locals.var_qlo__blk57_dn8 = assign2140_e2382_d_n8;
        locals.var_qlo__blk57_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2150_e2402, assign2150_e2402_d_n4, assign2150_e2402_d_n6, assign2150_e2402_d_n8,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign2150_e2389: f64 = (1.0 - p.p34);
        let assign2150_e2392: f64 = (0.5 * p.p43);
        let assign2150_e2394: f64 = (assign2150_e2392 * locals.var_dvh__blk55);
        let assign2150_e2396: f64 = (assign2150_e2394 / locals.var_pc_t);
        let assign2150_e2397: f64 = (assign2150_e2389 + assign2150_e2396);
        let assign2150_e2398: f64 = (locals.var_dvh__blk55 * assign2150_e2397);
        let assign2150_e2400: f64 = (assign2150_e2398 * locals.var_pwq__blk56);
        (assign2150_e2400, (((locals.var_dvh__blk55_dn4 * assign2150_e2397) + (locals.var_dvh__blk55 * ((((assign2150_e2392 * locals.var_dvh__blk55_dn4) * locals.var_pc_t) - (assign2150_e2394 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)))) * locals.var_pwq__blk56), (((locals.var_dvh__blk55_dn6 * assign2150_e2397) + (locals.var_dvh__blk55 * ((assign2150_e2392 * locals.var_dvh__blk55_dn6) / locals.var_pc_t))) * locals.var_pwq__blk56), (((locals.var_dvh__blk55_dn8 * assign2150_e2397) + (locals.var_dvh__blk55 * ((assign2150_e2392 * locals.var_dvh__blk55_dn8) / locals.var_pc_t))) * locals.var_pwq__blk56),)
    } else {
        (locals.var_qhi__blk58, locals.var_qhi__blk58_dn4, locals.var_qhi__blk58_dn6, locals.var_qhi__blk58_dn8,)
    }
};
        locals.var_qhi__blk58 = assign2150_e2402;
        locals.var_qhi__blk58_dn4 = assign2150_e2402_d_n4;
        locals.var_qhi__blk58_dn6 = assign2150_e2402_d_n6;
        locals.var_qhi__blk58_dn8 = assign2150_e2402_d_n8;
        locals.var_qhi__blk58_rv = 0.0;

        let assign2160_e2408: f64 = (-p.p45);
        let assign2160_e2410: f64 = if ((p.p45 > 0.0) && (locals.var_vbci < assign2160_e2408)) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign2160_e2410;
        locals.var_guard77_rv = 0.0;

        let (assign2170_e2449, assign2170_e2449_d_n4, assign2170_e2449_d_n6, assign2170_e2449_d_n8,) = {
    if (((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign2170_e2422: f64 = (p.p45 / locals.var_pc_t);
        let assign2170_e2423: f64 = (1.0 + assign2170_e2422);
        let assign2170_e2426: f64 = (1.0 - p.p43);
        let assign2170_e2427: f64 = (assign2170_e2423).powf(assign2170_e2426);
        let assign2170_e2431: f64 = (1.0 - p.p43);
        let assign2170_e2434: f64 = (locals.var_vbci + p.p45);
        let assign2170_e2435: f64 = (assign2170_e2431 * assign2170_e2434);
        let assign2170_e2438: f64 = (locals.var_pc_t + p.p45);
        let assign2170_e2439: f64 = (assign2170_e2435 / assign2170_e2438);
        let assign2170_e2440: f64 = (1.0 - assign2170_e2439);
        let assign2170_e2441: f64 = (assign2170_e2427 * assign2170_e2440);
        let assign2170_e2442: f64 = (1.0 - assign2170_e2441);
        let assign2170_e2443: f64 = (locals.var_pc_t * assign2170_e2442);
        let assign2170_e2446: f64 = (1.0 - p.p43);
        let assign2170_e2447: f64 = (assign2170_e2443 / assign2170_e2446);
        (assign2170_e2447, (((locals.var_pc_t_dn4 * assign2170_e2442) + (locals.var_pc_t * (-((if 0.0 == 0.0 && ((assign2170_e2426) as f64).is_finite() && ((assign2170_e2426) as f64).fract() == 0.0 { if assign2170_e2426 == 0.0 { 0.0 } else { (assign2170_e2426 * ((assign2170_e2423).powf(assign2170_e2426 - 1.0) * (-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2170_e2427 * (assign2170_e2426 * ((-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign2170_e2423))) } * assign2170_e2440) + (assign2170_e2427 * (-(-((assign2170_e2435 * locals.var_pc_t_dn4) / (assign2170_e2438 * assign2170_e2438))))))))) / assign2170_e2446), ((locals.var_pc_t * (-(assign2170_e2427 * (-((assign2170_e2431 * locals.var_vbci_dn6) / assign2170_e2438))))) / assign2170_e2446), ((locals.var_pc_t * (-(assign2170_e2427 * (-((assign2170_e2431 * locals.var_vbci_dn8) / assign2170_e2438))))) / assign2170_e2446),)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2170_e2449;
        locals.var_qlo__blk57_dn4 = assign2170_e2449_d_n4;
        locals.var_qlo__blk57_dn6 = assign2170_e2449_d_n6;
        locals.var_qlo__blk57_dn8 = assign2170_e2449_d_n8;
        locals.var_qlo__blk57_rv = 0.0;

        let (assign2180_e2475, assign2180_e2475_d_n4, assign2180_e2475_d_n6, assign2180_e2475_d_n8,) = {
    if (((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign2180_e2462: f64 = (locals.var_vbci / locals.var_pc_t);
        let assign2180_e2463: f64 = (1.0 - assign2180_e2462);
        let assign2180_e2466: f64 = (1.0 - p.p43);
        let assign2180_e2467: f64 = (assign2180_e2463).powf(assign2180_e2466);
        let assign2180_e2468: f64 = (1.0 - assign2180_e2467);
        let assign2180_e2469: f64 = (locals.var_pc_t * assign2180_e2468);
        let assign2180_e2472: f64 = (1.0 - p.p43);
        let assign2180_e2473: f64 = (assign2180_e2469 / assign2180_e2472);
        (assign2180_e2473, (((locals.var_pc_t_dn4 * assign2180_e2468) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2180_e2466) as f64).is_finite() && ((assign2180_e2466) as f64).fract() == 0.0 { if assign2180_e2466 == 0.0 { 0.0 } else { (assign2180_e2466 * ((assign2180_e2463).powf(assign2180_e2466 - 1.0) * (-(-((locals.var_vbci * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t)))))) } } else { (assign2180_e2467 * (assign2180_e2466 * ((-(-((locals.var_vbci * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t)))) / assign2180_e2463))) }))) / assign2180_e2472), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2180_e2466) as f64).is_finite() && ((assign2180_e2466) as f64).fract() == 0.0 { if assign2180_e2466 == 0.0 { 0.0 } else { (assign2180_e2466 * ((assign2180_e2463).powf(assign2180_e2466 - 1.0) * (-(locals.var_vbci_dn6 / locals.var_pc_t)))) } } else { (assign2180_e2467 * (assign2180_e2466 * ((-(locals.var_vbci_dn6 / locals.var_pc_t)) / assign2180_e2463))) })) / assign2180_e2472), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2180_e2466) as f64).is_finite() && ((assign2180_e2466) as f64).fract() == 0.0 { if assign2180_e2466 == 0.0 { 0.0 } else { (assign2180_e2466 * ((assign2180_e2463).powf(assign2180_e2466 - 1.0) * (-(locals.var_vbci_dn8 / locals.var_pc_t)))) } } else { (assign2180_e2467 * (assign2180_e2466 * ((-(locals.var_vbci_dn8 / locals.var_pc_t)) / assign2180_e2463))) })) / assign2180_e2472),)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2180_e2475;
        locals.var_qlo__blk57_dn4 = assign2180_e2475_d_n4;
        locals.var_qlo__blk57_dn6 = assign2180_e2475_d_n6;
        locals.var_qlo__blk57_dn8 = assign2180_e2475_d_n8;
        locals.var_qlo__blk57_rv = 0.0;

        let (assign2190_e2482, assign2190_e2482_d_n4, assign2190_e2482_d_n6, assign2190_e2482_d_n8,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk58, locals.var_qhi__blk58_dn4, locals.var_qhi__blk58_dn6, locals.var_qhi__blk58_dn8,)
    }
};
        locals.var_qhi__blk58 = assign2190_e2482;
        locals.var_qhi__blk58_dn4 = assign2190_e2482_d_n4;
        locals.var_qhi__blk58_dn6 = assign2190_e2482_d_n6;
        locals.var_qhi__blk58_dn8 = assign2190_e2482_d_n8;
        locals.var_qhi__blk58_rv = 0.0;

        let (assign2200_e2488, assign2200_e2488_d_n4, assign2200_e2488_d_n6, assign2200_e2488_d_n8,) = {
    if (locals.var_guard75 != 0.0) {
        let assign2200_e2486: f64 = (locals.var_qlo__blk57 + locals.var_qhi__blk58);
        (assign2200_e2486, (locals.var_qlo__blk57_dn4 + locals.var_qhi__blk58_dn4), (locals.var_qlo__blk57_dn6 + locals.var_qhi__blk58_dn6), (locals.var_qlo__blk57_dn8 + locals.var_qhi__blk58_dn8),)
    } else {
        (locals.var_qdbc, locals.var_qdbc_dn4, locals.var_qdbc_dn6, locals.var_qdbc_dn8,)
    }
};
        locals.var_qdbc = assign2200_e2488;
        locals.var_qdbc_dn4 = assign2200_e2488_d_n4;
        locals.var_qdbc_dn6 = assign2200_e2488_d_n6;
        locals.var_qdbc_dn8 = assign2200_e2488_d_n8;
        locals.var_qdbc_rv = 0.0;

        let assign2210_e2495: f64 = if ((p.p45 > 0.0) && (p.p46 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign2210_e2495;
        locals.var_guard78_rv = 0.0;

        let (assign2220_e2508, assign2220_e2508_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2220_e2502: f64 = (p.p45 + locals.var_dv0__blk54);
        let assign2220_e2505: f64 = (p.p45 - locals.var_dv0__blk54);
        let assign2220_e2506: f64 = (assign2220_e2502 / assign2220_e2505);
        (assign2220_e2506, (((locals.var_dv0__blk54_dn4 * assign2220_e2505) - (assign2220_e2502 * (-locals.var_dv0__blk54_dn4))) / (assign2220_e2505 * assign2220_e2505)),)
    } else {
        (locals.var_vn0, locals.var_vn0_dn4,)
    }
};
        locals.var_vn0 = assign2220_e2508;
        locals.var_vn0_dn4 = assign2220_e2508_d_n4;
        locals.var_vn0_rv = 0.0;

        let (assign2230_e2547, assign2230_e2547_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2230_e2515: f64 = (2.0 * locals.var_vn0);
        let assign2230_e2518: f64 = (locals.var_vn0 - 1.0);
        let assign2230_e2521: f64 = (locals.var_vn0 - 1.0);
        let assign2230_e2522: f64 = (assign2230_e2518 * assign2230_e2521);
        let assign2230_e2525: f64 = (4.0 * p.p44);
        let assign2230_e2527: f64 = (assign2230_e2525 * p.p44);
        let assign2230_e2528: f64 = (assign2230_e2522 + assign2230_e2527);
        let assign2230_e2529: f64 = (assign2230_e2528).sqrt();
        let assign2230_e2532: f64 = (locals.var_vn0 + 1.0);
        let assign2230_e2535: f64 = (locals.var_vn0 + 1.0);
        let assign2230_e2536: f64 = (assign2230_e2532 * assign2230_e2535);
        let assign2230_e2539: f64 = (4.0 * p.p46);
        let assign2230_e2541: f64 = (assign2230_e2539 * p.p46);
        let assign2230_e2542: f64 = (assign2230_e2536 + assign2230_e2541);
        let assign2230_e2543: f64 = (assign2230_e2542).sqrt();
        let assign2230_e2544: f64 = (assign2230_e2529 + assign2230_e2543);
        let assign2230_e2545: f64 = (assign2230_e2515 / assign2230_e2544);
        (assign2230_e2545, ((((2.0 * locals.var_vn0_dn4) * assign2230_e2544) - (assign2230_e2515 * ((((locals.var_vn0_dn4 * assign2230_e2521) + (assign2230_e2518 * locals.var_vn0_dn4)) / (2.0 * assign2230_e2529)) + (((locals.var_vn0_dn4 * assign2230_e2535) + (assign2230_e2532 * locals.var_vn0_dn4)) / (2.0 * assign2230_e2543))))) / (assign2230_e2544 * assign2230_e2544)),)
    } else {
        (locals.var_vnl0, locals.var_vnl0_dn4,)
    }
};
        locals.var_vnl0 = assign2230_e2547;
        locals.var_vnl0_dn4 = assign2230_e2547_d_n4;
        locals.var_vnl0_rv = 0.0;

        let (assign2240_e2564, assign2240_e2564_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2240_e2556: f64 = (p.p45 - locals.var_dv0__blk54);
        let assign2240_e2557: f64 = (locals.var_vnl0 * assign2240_e2556);
        let assign2240_e2559: f64 = (assign2240_e2557 - p.p45);
        let assign2240_e2561: f64 = (assign2240_e2559 - locals.var_dv0__blk54);
        let assign2240_e2562: f64 = (0.5 * assign2240_e2561);
        (assign2240_e2562, (0.5 * (((locals.var_vnl0_dn4 * assign2240_e2556) + (locals.var_vnl0 * (-locals.var_dv0__blk54_dn4))) - locals.var_dv0__blk54_dn4)),)
    } else {
        (locals.var_vl0__blk61, locals.var_vl0__blk61_dn4,)
    }
};
        locals.var_vl0__blk61 = assign2240_e2564;
        locals.var_vl0__blk61_dn4 = assign2240_e2564_d_n4;
        locals.var_vl0__blk61_rv = 0.0;

        let (assign2250_e2587, assign2250_e2587_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2250_e2574: f64 = (locals.var_vl0__blk61 / locals.var_pc_t);
        let assign2250_e2575: f64 = (1.0 - assign2250_e2574);
        let assign2250_e2578: f64 = (1.0 - p.p43);
        let assign2250_e2579: f64 = (assign2250_e2575).powf(assign2250_e2578);
        let assign2250_e2580: f64 = (1.0 - assign2250_e2579);
        let assign2250_e2581: f64 = (locals.var_pc_t * assign2250_e2580);
        let assign2250_e2584: f64 = (1.0 - p.p43);
        let assign2250_e2585: f64 = (assign2250_e2581 / assign2250_e2584);
        (assign2250_e2585, (((locals.var_pc_t_dn4 * assign2250_e2580) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2250_e2578) as f64).is_finite() && ((assign2250_e2578) as f64).fract() == 0.0 { if assign2250_e2578 == 0.0 { 0.0 } else { (assign2250_e2578 * ((assign2250_e2575).powf(assign2250_e2578 - 1.0) * (-(((locals.var_vl0__blk61_dn4 * locals.var_pc_t) - (locals.var_vl0__blk61 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2250_e2579 * (assign2250_e2578 * ((-(((locals.var_vl0__blk61_dn4 * locals.var_pc_t) - (locals.var_vl0__blk61 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign2250_e2575))) }))) / assign2250_e2584),)
    } else {
        (locals.var_qlo0, locals.var_qlo0_dn4,)
    }
};
        locals.var_qlo0 = assign2250_e2587;
        locals.var_qlo0_dn4 = assign2250_e2587_d_n4;
        locals.var_qlo0_rv = 0.0;

        let (assign2260_e2604, assign2260_e2604_d_n4, assign2260_e2604_d_n6, assign2260_e2604_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2260_e2594: f64 = (2.0 * locals.var_vbci);
        let assign2260_e2596: f64 = (assign2260_e2594 + p.p45);
        let assign2260_e2598: f64 = (assign2260_e2596 + locals.var_dv0__blk54);
        let assign2260_e2601: f64 = (p.p45 - locals.var_dv0__blk54);
        let assign2260_e2602: f64 = (assign2260_e2598 / assign2260_e2601);
        (assign2260_e2602, (((locals.var_dv0__blk54_dn4 * assign2260_e2601) - (assign2260_e2598 * (-locals.var_dv0__blk54_dn4))) / (assign2260_e2601 * assign2260_e2601)), ((2.0 * locals.var_vbci_dn6) / assign2260_e2601), ((2.0 * locals.var_vbci_dn8) / assign2260_e2601),)
    } else {
        (locals.var_vn, locals.var_vn_dn4, locals.var_vn_dn6, locals.var_vn_dn8,)
    }
};
        locals.var_vn = assign2260_e2604;
        locals.var_vn_dn4 = assign2260_e2604_d_n4;
        locals.var_vn_dn6 = assign2260_e2604_d_n6;
        locals.var_vn_dn8 = assign2260_e2604_d_n8;
        locals.var_vn_rv = 0.0;

        let (assign2270_e2643, assign2270_e2643_d_n4, assign2270_e2643_d_n6, assign2270_e2643_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2270_e2611: f64 = (2.0 * locals.var_vn);
        let assign2270_e2614: f64 = (locals.var_vn - 1.0);
        let assign2270_e2617: f64 = (locals.var_vn - 1.0);
        let assign2270_e2618: f64 = (assign2270_e2614 * assign2270_e2617);
        let assign2270_e2621: f64 = (4.0 * p.p44);
        let assign2270_e2623: f64 = (assign2270_e2621 * p.p44);
        let assign2270_e2624: f64 = (assign2270_e2618 + assign2270_e2623);
        let assign2270_e2625: f64 = (assign2270_e2624).sqrt();
        let assign2270_e2628: f64 = (locals.var_vn + 1.0);
        let assign2270_e2631: f64 = (locals.var_vn + 1.0);
        let assign2270_e2632: f64 = (assign2270_e2628 * assign2270_e2631);
        let assign2270_e2635: f64 = (4.0 * p.p46);
        let assign2270_e2637: f64 = (assign2270_e2635 * p.p46);
        let assign2270_e2638: f64 = (assign2270_e2632 + assign2270_e2637);
        let assign2270_e2639: f64 = (assign2270_e2638).sqrt();
        let assign2270_e2640: f64 = (assign2270_e2625 + assign2270_e2639);
        let assign2270_e2641: f64 = (assign2270_e2611 / assign2270_e2640);
        (assign2270_e2641, ((((2.0 * locals.var_vn_dn4) * assign2270_e2640) - (assign2270_e2611 * ((((locals.var_vn_dn4 * assign2270_e2617) + (assign2270_e2614 * locals.var_vn_dn4)) / (2.0 * assign2270_e2625)) + (((locals.var_vn_dn4 * assign2270_e2631) + (assign2270_e2628 * locals.var_vn_dn4)) / (2.0 * assign2270_e2639))))) / (assign2270_e2640 * assign2270_e2640)), ((((2.0 * locals.var_vn_dn6) * assign2270_e2640) - (assign2270_e2611 * ((((locals.var_vn_dn6 * assign2270_e2617) + (assign2270_e2614 * locals.var_vn_dn6)) / (2.0 * assign2270_e2625)) + (((locals.var_vn_dn6 * assign2270_e2631) + (assign2270_e2628 * locals.var_vn_dn6)) / (2.0 * assign2270_e2639))))) / (assign2270_e2640 * assign2270_e2640)), ((((2.0 * locals.var_vn_dn8) * assign2270_e2640) - (assign2270_e2611 * ((((locals.var_vn_dn8 * assign2270_e2617) + (assign2270_e2614 * locals.var_vn_dn8)) / (2.0 * assign2270_e2625)) + (((locals.var_vn_dn8 * assign2270_e2631) + (assign2270_e2628 * locals.var_vn_dn8)) / (2.0 * assign2270_e2639))))) / (assign2270_e2640 * assign2270_e2640)),)
    } else {
        (locals.var_vnl, locals.var_vnl_dn4, locals.var_vnl_dn6, locals.var_vnl_dn8,)
    }
};
        locals.var_vnl = assign2270_e2643;
        locals.var_vnl_dn4 = assign2270_e2643_d_n4;
        locals.var_vnl_dn6 = assign2270_e2643_d_n6;
        locals.var_vnl_dn8 = assign2270_e2643_d_n8;
        locals.var_vnl_rv = 0.0;

        let (assign2280_e2660, assign2280_e2660_d_n4, assign2280_e2660_d_n6, assign2280_e2660_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2280_e2652: f64 = (p.p45 - locals.var_dv0__blk54);
        let assign2280_e2653: f64 = (locals.var_vnl * assign2280_e2652);
        let assign2280_e2655: f64 = (assign2280_e2653 - p.p45);
        let assign2280_e2657: f64 = (assign2280_e2655 - locals.var_dv0__blk54);
        let assign2280_e2658: f64 = (0.5 * assign2280_e2657);
        (assign2280_e2658, (0.5 * (((locals.var_vnl_dn4 * assign2280_e2652) + (locals.var_vnl * (-locals.var_dv0__blk54_dn4))) - locals.var_dv0__blk54_dn4)), (0.5 * (locals.var_vnl_dn6 * assign2280_e2652)), (0.5 * (locals.var_vnl_dn8 * assign2280_e2652)),)
    } else {
        (locals.var_vl__blk65, locals.var_vl__blk65_dn4, locals.var_vl__blk65_dn6, locals.var_vl__blk65_dn8,)
    }
};
        locals.var_vl__blk65 = assign2280_e2660;
        locals.var_vl__blk65_dn4 = assign2280_e2660_d_n4;
        locals.var_vl__blk65_dn6 = assign2280_e2660_d_n6;
        locals.var_vl__blk65_dn8 = assign2280_e2660_d_n8;
        locals.var_vl__blk65_rv = 0.0;

        let (assign2290_e2683, assign2290_e2683_d_n4, assign2290_e2683_d_n6, assign2290_e2683_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2290_e2670: f64 = (locals.var_vl__blk65 / locals.var_pc_t);
        let assign2290_e2671: f64 = (1.0 - assign2290_e2670);
        let assign2290_e2674: f64 = (1.0 - p.p43);
        let assign2290_e2675: f64 = (assign2290_e2671).powf(assign2290_e2674);
        let assign2290_e2676: f64 = (1.0 - assign2290_e2675);
        let assign2290_e2677: f64 = (locals.var_pc_t * assign2290_e2676);
        let assign2290_e2680: f64 = (1.0 - p.p43);
        let assign2290_e2681: f64 = (assign2290_e2677 / assign2290_e2680);
        (assign2290_e2681, (((locals.var_pc_t_dn4 * assign2290_e2676) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2290_e2674) as f64).is_finite() && ((assign2290_e2674) as f64).fract() == 0.0 { if assign2290_e2674 == 0.0 { 0.0 } else { (assign2290_e2674 * ((assign2290_e2671).powf(assign2290_e2674 - 1.0) * (-(((locals.var_vl__blk65_dn4 * locals.var_pc_t) - (locals.var_vl__blk65 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2290_e2675 * (assign2290_e2674 * ((-(((locals.var_vl__blk65_dn4 * locals.var_pc_t) - (locals.var_vl__blk65 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign2290_e2671))) }))) / assign2290_e2680), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2290_e2674) as f64).is_finite() && ((assign2290_e2674) as f64).fract() == 0.0 { if assign2290_e2674 == 0.0 { 0.0 } else { (assign2290_e2674 * ((assign2290_e2671).powf(assign2290_e2674 - 1.0) * (-(locals.var_vl__blk65_dn6 / locals.var_pc_t)))) } } else { (assign2290_e2675 * (assign2290_e2674 * ((-(locals.var_vl__blk65_dn6 / locals.var_pc_t)) / assign2290_e2671))) })) / assign2290_e2680), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2290_e2674) as f64).is_finite() && ((assign2290_e2674) as f64).fract() == 0.0 { if assign2290_e2674 == 0.0 { 0.0 } else { (assign2290_e2674 * ((assign2290_e2671).powf(assign2290_e2674 - 1.0) * (-(locals.var_vl__blk65_dn8 / locals.var_pc_t)))) } } else { (assign2290_e2675 * (assign2290_e2674 * ((-(locals.var_vl__blk65_dn8 / locals.var_pc_t)) / assign2290_e2671))) })) / assign2290_e2680),)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2290_e2683;
        locals.var_qlo__blk57_dn4 = assign2290_e2683_d_n4;
        locals.var_qlo__blk57_dn6 = assign2290_e2683_d_n6;
        locals.var_qlo__blk57_dn8 = assign2290_e2683_d_n8;
        locals.var_qlo__blk57_rv = 0.0;

        let (assign2300_e2694, assign2300_e2694_d_n4, assign2300_e2694_d_n6, assign2300_e2694_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2300_e2691: f64 = (locals.var_vnl + 1.0);
        let assign2300_e2692: f64 = (0.5 * assign2300_e2691);
        (assign2300_e2692, (0.5 * locals.var_vnl_dn4), (0.5 * locals.var_vnl_dn6), (0.5 * locals.var_vnl_dn8),)
    } else {
        (locals.var_sel, locals.var_sel_dn4, locals.var_sel_dn6, locals.var_sel_dn8,)
    }
};
        locals.var_sel = assign2300_e2694;
        locals.var_sel_dn4 = assign2300_e2694_d_n4;
        locals.var_sel_dn6 = assign2300_e2694_d_n6;
        locals.var_sel_dn8 = assign2300_e2694_d_n8;
        locals.var_sel_rv = 0.0;

        let (assign2310_e2708, assign2310_e2708_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2310_e2702: f64 = (p.p45 / locals.var_pc_t);
        let assign2310_e2703: f64 = (1.0 + assign2310_e2702);
        let assign2310_e2705: f64 = (-p.p43);
        let assign2310_e2706: f64 = (assign2310_e2703).powf(assign2310_e2705);
        (assign2310_e2706, if 0.0 == 0.0 && ((assign2310_e2705) as f64).is_finite() && ((assign2310_e2705) as f64).fract() == 0.0 { if assign2310_e2705 == 0.0 { 0.0 } else { (assign2310_e2705 * ((assign2310_e2703).powf(assign2310_e2705 - 1.0) * (-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2310_e2706 * (assign2310_e2705 * ((-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign2310_e2703))) },)
    } else {
        (locals.var_crt, locals.var_crt_dn4,)
    }
};
        locals.var_crt = assign2310_e2708;
        locals.var_crt_dn4 = assign2310_e2708_d_n4;
        locals.var_crt_rv = 0.0;

        let (assign2320_e2722, assign2320_e2722_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2320_e2716: f64 = (locals.var_dv0__blk54 / locals.var_pc_t);
        let assign2320_e2717: f64 = (1.0 + assign2320_e2716);
        let assign2320_e2719: f64 = (-p.p43);
        let assign2320_e2720: f64 = (assign2320_e2717).powf(assign2320_e2719);
        (assign2320_e2720, if 0.0 == 0.0 && ((assign2320_e2719) as f64).is_finite() && ((assign2320_e2719) as f64).fract() == 0.0 { if assign2320_e2719 == 0.0 { 0.0 } else { (assign2320_e2719 * ((assign2320_e2717).powf(assign2320_e2719 - 1.0) * (((locals.var_dv0__blk54_dn4 * locals.var_pc_t) - (locals.var_dv0__blk54 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)))) } } else { (assign2320_e2720 * (assign2320_e2719 * ((((locals.var_dv0__blk54_dn4 * locals.var_pc_t) - (locals.var_dv0__blk54 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)) / assign2320_e2717))) },)
    } else {
        (locals.var_cmx, locals.var_cmx_dn4,)
    }
};
        locals.var_cmx = assign2320_e2722;
        locals.var_cmx_dn4 = assign2320_e2722_d_n4;
        locals.var_cmx_rv = 0.0;

        let (assign2330_e2737, assign2330_e2737_d_n4, assign2330_e2737_d_n6, assign2330_e2737_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2330_e2729: f64 = (1.0 - locals.var_sel);
        let assign2330_e2731: f64 = (assign2330_e2729 * locals.var_crt);
        let assign2330_e2734: f64 = (locals.var_sel * locals.var_cmx);
        let assign2330_e2735: f64 = (assign2330_e2731 + assign2330_e2734);
        (assign2330_e2735, ((((-locals.var_sel_dn4) * locals.var_crt) + (assign2330_e2729 * locals.var_crt_dn4)) + ((locals.var_sel_dn4 * locals.var_cmx) + (locals.var_sel * locals.var_cmx_dn4))), (((-locals.var_sel_dn6) * locals.var_crt) + (locals.var_sel_dn6 * locals.var_cmx)), (((-locals.var_sel_dn8) * locals.var_crt) + (locals.var_sel_dn8 * locals.var_cmx)),)
    } else {
        (locals.var_cl, locals.var_cl_dn4, locals.var_cl_dn6, locals.var_cl_dn8,)
    }
};
        locals.var_cl = assign2330_e2737;
        locals.var_cl_dn4 = assign2330_e2737_d_n4;
        locals.var_cl_dn6 = assign2330_e2737_d_n6;
        locals.var_cl_dn8 = assign2330_e2737_d_n8;
        locals.var_cl_rv = 0.0;

        let (assign2340_e2750, assign2340_e2750_d_n4, assign2340_e2750_d_n6, assign2340_e2750_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2340_e2744: f64 = (locals.var_vbci - locals.var_vl__blk65);
        let assign2340_e2746: f64 = (assign2340_e2744 + locals.var_vl0__blk61);
        let assign2340_e2748: f64 = (assign2340_e2746 * locals.var_cl);
        (assign2340_e2748, ((((-locals.var_vl__blk65_dn4) + locals.var_vl0__blk61_dn4) * locals.var_cl) + (assign2340_e2746 * locals.var_cl_dn4)), (((locals.var_vbci_dn6 - locals.var_vl__blk65_dn6) * locals.var_cl) + (assign2340_e2746 * locals.var_cl_dn6)), (((locals.var_vbci_dn8 - locals.var_vl__blk65_dn8) * locals.var_cl) + (assign2340_e2746 * locals.var_cl_dn8)),)
    } else {
        (locals.var_ql, locals.var_ql_dn4, locals.var_ql_dn6, locals.var_ql_dn8,)
    }
};
        locals.var_ql = assign2340_e2750;
        locals.var_ql_dn4 = assign2340_e2750_d_n4;
        locals.var_ql_dn6 = assign2340_e2750_d_n6;
        locals.var_ql_dn8 = assign2340_e2750_d_n8;
        locals.var_ql_rv = 0.0;

        let (assign2350_e2761, assign2350_e2761_d_n4, assign2350_e2761_d_n6, assign2350_e2761_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2350_e2757: f64 = (locals.var_ql + locals.var_qlo__blk57);
        let assign2350_e2759: f64 = (assign2350_e2757 - locals.var_qlo0);
        (assign2350_e2759, ((locals.var_ql_dn4 + locals.var_qlo__blk57_dn4) - locals.var_qlo0_dn4), (locals.var_ql_dn6 + locals.var_qlo__blk57_dn6), (locals.var_ql_dn8 + locals.var_qlo__blk57_dn8),)
    } else {
        (locals.var_qdbc, locals.var_qdbc_dn4, locals.var_qdbc_dn6, locals.var_qdbc_dn8,)
    }
};
        locals.var_qdbc = assign2350_e2761;
        locals.var_qdbc_dn4 = assign2350_e2761_d_n4;
        locals.var_qdbc_dn6 = assign2350_e2761_d_n6;
        locals.var_qdbc_dn8 = assign2350_e2761_d_n8;
        locals.var_qdbc_rv = 0.0;

        let (assign2360_e2778, assign2360_e2778_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2360_e2769: f64 = (locals.var_dv0__blk54 * locals.var_dv0__blk54);
        let assign2360_e2772: f64 = (4.0 * p.p44);
        let assign2360_e2774: f64 = (assign2360_e2772 * p.p44);
        let assign2360_e2775: f64 = (assign2360_e2769 + assign2360_e2774);
        let assign2360_e2776: f64 = (assign2360_e2775).sqrt();
        (assign2360_e2776, (((locals.var_dv0__blk54_dn4 * locals.var_dv0__blk54) + (locals.var_dv0__blk54 * locals.var_dv0__blk54_dn4)) / (2.0 * assign2360_e2776)),)
    } else {
        (locals.var_mv0__blk71, locals.var_mv0__blk71_dn4,)
    }
};
        locals.var_mv0__blk71 = assign2360_e2778;
        locals.var_mv0__blk71_dn4 = assign2360_e2778_d_n4;
        locals.var_mv0__blk71_rv = 0.0;

        let (assign2370_e2791, assign2370_e2791_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2370_e2785: f64 = (-0.5);
        let assign2370_e2788: f64 = (locals.var_dv0__blk54 + locals.var_mv0__blk71);
        let assign2370_e2789: f64 = (assign2370_e2785 * assign2370_e2788);
        (assign2370_e2789, (assign2370_e2785 * (locals.var_dv0__blk54_dn4 + locals.var_mv0__blk71_dn4)),)
    } else {
        (locals.var_vl0__blk61, locals.var_vl0__blk61_dn4,)
    }
};
        locals.var_vl0__blk61 = assign2370_e2791;
        locals.var_vl0__blk61_dn4 = assign2370_e2791_d_n4;
        locals.var_vl0__blk61_rv = 0.0;

        let (assign2380_e2814, assign2380_e2814_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2380_e2798: f64 = (-locals.var_pc_t);
        let assign2380_e2802: f64 = (locals.var_vl0__blk61 / locals.var_pc_t);
        let assign2380_e2803: f64 = (1.0 - assign2380_e2802);
        let assign2380_e2806: f64 = (1.0 - p.p43);
        let assign2380_e2807: f64 = (assign2380_e2803).powf(assign2380_e2806);
        let assign2380_e2808: f64 = (assign2380_e2798 * assign2380_e2807);
        let assign2380_e2811: f64 = (1.0 - p.p43);
        let assign2380_e2812: f64 = (assign2380_e2808 / assign2380_e2811);
        (assign2380_e2812, ((((-locals.var_pc_t_dn4) * assign2380_e2807) + (assign2380_e2798 * if 0.0 == 0.0 && ((assign2380_e2806) as f64).is_finite() && ((assign2380_e2806) as f64).fract() == 0.0 { if assign2380_e2806 == 0.0 { 0.0 } else { (assign2380_e2806 * ((assign2380_e2803).powf(assign2380_e2806 - 1.0) * (-(((locals.var_vl0__blk61_dn4 * locals.var_pc_t) - (locals.var_vl0__blk61 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2380_e2807 * (assign2380_e2806 * ((-(((locals.var_vl0__blk61_dn4 * locals.var_pc_t) - (locals.var_vl0__blk61 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign2380_e2803))) })) / assign2380_e2811),)
    } else {
        (locals.var_q0__blk72, locals.var_q0__blk72_dn4,)
    }
};
        locals.var_q0__blk72 = assign2380_e2814;
        locals.var_q0__blk72_dn4 = assign2380_e2814_d_n4;
        locals.var_q0__blk72_rv = 0.0;

        let (assign2390_e2824, assign2390_e2824_d_n4, assign2390_e2824_d_n6, assign2390_e2824_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2390_e2822: f64 = (locals.var_vbci + locals.var_dv0__blk54);
        (assign2390_e2822, locals.var_dv0__blk54_dn4, locals.var_vbci_dn6, locals.var_vbci_dn8,)
    } else {
        (locals.var_dv__blk73, locals.var_dv__blk73_dn4, locals.var_dv__blk73_dn6, locals.var_dv__blk73_dn8,)
    }
};
        locals.var_dv__blk73 = assign2390_e2824;
        locals.var_dv__blk73_dn4 = assign2390_e2824_d_n4;
        locals.var_dv__blk73_dn6 = assign2390_e2824_d_n6;
        locals.var_dv__blk73_dn8 = assign2390_e2824_d_n8;
        locals.var_dv__blk73_rv = 0.0;

        let (assign2400_e2841, assign2400_e2841_d_n4, assign2400_e2841_d_n6, assign2400_e2841_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2400_e2832: f64 = (locals.var_dv__blk73 * locals.var_dv__blk73);
        let assign2400_e2835: f64 = (4.0 * p.p44);
        let assign2400_e2837: f64 = (assign2400_e2835 * p.p44);
        let assign2400_e2838: f64 = (assign2400_e2832 + assign2400_e2837);
        let assign2400_e2839: f64 = (assign2400_e2838).sqrt();
        (assign2400_e2839, (((locals.var_dv__blk73_dn4 * locals.var_dv__blk73) + (locals.var_dv__blk73 * locals.var_dv__blk73_dn4)) / (2.0 * assign2400_e2839)), (((locals.var_dv__blk73_dn6 * locals.var_dv__blk73) + (locals.var_dv__blk73 * locals.var_dv__blk73_dn6)) / (2.0 * assign2400_e2839)), (((locals.var_dv__blk73_dn8 * locals.var_dv__blk73) + (locals.var_dv__blk73 * locals.var_dv__blk73_dn8)) / (2.0 * assign2400_e2839)),)
    } else {
        (locals.var_mv__blk74, locals.var_mv__blk74_dn4, locals.var_mv__blk74_dn6, locals.var_mv__blk74_dn8,)
    }
};
        locals.var_mv__blk74 = assign2400_e2841;
        locals.var_mv__blk74_dn4 = assign2400_e2841_d_n4;
        locals.var_mv__blk74_dn6 = assign2400_e2841_d_n6;
        locals.var_mv__blk74_dn8 = assign2400_e2841_d_n8;
        locals.var_mv__blk74_rv = 0.0;

        let (assign2410_e2855, assign2410_e2855_d_n4, assign2410_e2855_d_n6, assign2410_e2855_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2410_e2850: f64 = (locals.var_dv__blk73 - locals.var_mv__blk74);
        let assign2410_e2851: f64 = (0.5 * assign2410_e2850);
        let assign2410_e2853: f64 = (assign2410_e2851 - locals.var_dv0__blk54);
        (assign2410_e2853, ((0.5 * (locals.var_dv__blk73_dn4 - locals.var_mv__blk74_dn4)) - locals.var_dv0__blk54_dn4), (0.5 * (locals.var_dv__blk73_dn6 - locals.var_mv__blk74_dn6)), (0.5 * (locals.var_dv__blk73_dn8 - locals.var_mv__blk74_dn8)),)
    } else {
        (locals.var_vl__blk65, locals.var_vl__blk65_dn4, locals.var_vl__blk65_dn6, locals.var_vl__blk65_dn8,)
    }
};
        locals.var_vl__blk65 = assign2410_e2855;
        locals.var_vl__blk65_dn4 = assign2410_e2855_d_n4;
        locals.var_vl__blk65_dn6 = assign2410_e2855_d_n6;
        locals.var_vl__blk65_dn8 = assign2410_e2855_d_n8;
        locals.var_vl__blk65_rv = 0.0;

        let (assign2420_e2878, assign2420_e2878_d_n4, assign2420_e2878_d_n6, assign2420_e2878_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2420_e2862: f64 = (-locals.var_pc_t);
        let assign2420_e2866: f64 = (locals.var_vl__blk65 / locals.var_pc_t);
        let assign2420_e2867: f64 = (1.0 - assign2420_e2866);
        let assign2420_e2870: f64 = (1.0 - p.p43);
        let assign2420_e2871: f64 = (assign2420_e2867).powf(assign2420_e2870);
        let assign2420_e2872: f64 = (assign2420_e2862 * assign2420_e2871);
        let assign2420_e2875: f64 = (1.0 - p.p43);
        let assign2420_e2876: f64 = (assign2420_e2872 / assign2420_e2875);
        (assign2420_e2876, ((((-locals.var_pc_t_dn4) * assign2420_e2871) + (assign2420_e2862 * if 0.0 == 0.0 && ((assign2420_e2870) as f64).is_finite() && ((assign2420_e2870) as f64).fract() == 0.0 { if assign2420_e2870 == 0.0 { 0.0 } else { (assign2420_e2870 * ((assign2420_e2867).powf(assign2420_e2870 - 1.0) * (-(((locals.var_vl__blk65_dn4 * locals.var_pc_t) - (locals.var_vl__blk65 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2420_e2871 * (assign2420_e2870 * ((-(((locals.var_vl__blk65_dn4 * locals.var_pc_t) - (locals.var_vl__blk65 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign2420_e2867))) })) / assign2420_e2875), ((assign2420_e2862 * if 0.0 == 0.0 && ((assign2420_e2870) as f64).is_finite() && ((assign2420_e2870) as f64).fract() == 0.0 { if assign2420_e2870 == 0.0 { 0.0 } else { (assign2420_e2870 * ((assign2420_e2867).powf(assign2420_e2870 - 1.0) * (-(locals.var_vl__blk65_dn6 / locals.var_pc_t)))) } } else { (assign2420_e2871 * (assign2420_e2870 * ((-(locals.var_vl__blk65_dn6 / locals.var_pc_t)) / assign2420_e2867))) }) / assign2420_e2875), ((assign2420_e2862 * if 0.0 == 0.0 && ((assign2420_e2870) as f64).is_finite() && ((assign2420_e2870) as f64).fract() == 0.0 { if assign2420_e2870 == 0.0 { 0.0 } else { (assign2420_e2870 * ((assign2420_e2867).powf(assign2420_e2870 - 1.0) * (-(locals.var_vl__blk65_dn8 / locals.var_pc_t)))) } } else { (assign2420_e2871 * (assign2420_e2870 * ((-(locals.var_vl__blk65_dn8 / locals.var_pc_t)) / assign2420_e2867))) }) / assign2420_e2875),)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2420_e2878;
        locals.var_qlo__blk57_dn4 = assign2420_e2878_d_n4;
        locals.var_qlo__blk57_dn6 = assign2420_e2878_d_n6;
        locals.var_qlo__blk57_dn8 = assign2420_e2878_d_n8;
        locals.var_qlo__blk57_rv = 0.0;

        let (assign2430_e2901, assign2430_e2901_d_n4, assign2430_e2901_d_n6, assign2430_e2901_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2430_e2887: f64 = (1.0 - p.p34);
        let assign2430_e2889: f64 = (-p.p43);
        let assign2430_e2890: f64 = (assign2430_e2887).powf(assign2430_e2889);
        let assign2430_e2893: f64 = (locals.var_vbci - locals.var_vl__blk65);
        let assign2430_e2895: f64 = (assign2430_e2893 + locals.var_vl0__blk61);
        let assign2430_e2896: f64 = (assign2430_e2890 * assign2430_e2895);
        let assign2430_e2897: f64 = (locals.var_qlo__blk57 + assign2430_e2896);
        let assign2430_e2899: f64 = (assign2430_e2897 - locals.var_q0__blk72);
        (assign2430_e2899, ((locals.var_qlo__blk57_dn4 + (assign2430_e2890 * ((-locals.var_vl__blk65_dn4) + locals.var_vl0__blk61_dn4))) - locals.var_q0__blk72_dn4), (locals.var_qlo__blk57_dn6 + (assign2430_e2890 * (locals.var_vbci_dn6 - locals.var_vl__blk65_dn6))), (locals.var_qlo__blk57_dn8 + (assign2430_e2890 * (locals.var_vbci_dn8 - locals.var_vl__blk65_dn8))),)
    } else {
        (locals.var_qdbc, locals.var_qdbc_dn4, locals.var_qdbc_dn6, locals.var_qdbc_dn8,)
    }
};
        locals.var_qdbc = assign2430_e2901;
        locals.var_qdbc_dn4 = assign2430_e2901_d_n4;
        locals.var_qdbc_dn6 = assign2430_e2901_d_n6;
        locals.var_qdbc_dn8 = assign2430_e2901_d_n8;
        locals.var_qdbc_rv = 0.0;

        let assign2440_e2905: f64 = (locals.var_nf_t * locals.var_vtv);
        let assign2440_e2906: f64 = (1.0 / assign2440_e2905);
        locals.var_afac = assign2440_e2906;
        locals.var_afac_dn4 = (-(((locals.var_nf_t_dn4 * locals.var_vtv) + (locals.var_nf_t * locals.var_vtv_dn4)) / (assign2440_e2905 * assign2440_e2905)));
        locals.var_afac_rv = 0.0;

        let assign2450_e2909: f64 = if locals.var_vbei < locals.var_maxvifi { 1.0 } else { 0.0 };
        locals.var_guard79 = assign2450_e2909;
        locals.var_guard79_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2460_e2916, assign2460_e2916_d_n4, assign2460_e2916_d_n5, assign2460_e2916_d_n6, assign2460_e2916_d_n7, assign2460_e2916_d_n8, assign2460_e2916_d_n9, assign2460_e2916_d_n10, assign2460_e2916_d_n11,) = {
    if (locals.var_guard79 != 0.0) {
        let assign2460_e2913: f64 = (locals.var_vbei * locals.var_afac);
        let assign2460_e2914: f64 = (assign2460_e2913).exp();
        (assign2460_e2914, (assign2460_e2914 * (locals.var_vbei * locals.var_afac_dn4)), 0.0, 0.0, 0.0, (assign2460_e2914 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2460_e2914 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2460_e2916;
        locals.var_expi_dn4 = assign2460_e2916_d_n4;
        locals.var_expi_dn5 = assign2460_e2916_d_n5;
        locals.var_expi_dn6 = assign2460_e2916_d_n6;
        locals.var_expi_dn7 = assign2460_e2916_d_n7;
        locals.var_expi_dn8 = assign2460_e2916_d_n8;
        locals.var_expi_dn9 = assign2460_e2916_d_n9;
        locals.var_expi_dn10 = assign2460_e2916_d_n10;
        locals.var_expi_dn11 = assign2460_e2916_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign2470_e2932, assign2470_e2932_d_n4, assign2470_e2932_d_n5, assign2470_e2932_d_n6, assign2470_e2932_d_n7, assign2470_e2932_d_n8, assign2470_e2932_d_n9, assign2470_e2932_d_n10, assign2470_e2932_d_n11,) = {
    if (locals.var_guard79 == 0.0) {
        let assign2470_e2921: f64 = (locals.var_maxvifi * locals.var_afac);
        let assign2470_e2922: f64 = (assign2470_e2921).exp();
        let assign2470_e2926: f64 = (locals.var_vbei - locals.var_maxvifi);
        let assign2470_e2928: f64 = (assign2470_e2926 * locals.var_afac);
        let assign2470_e2929: f64 = (1.0 + assign2470_e2928);
        let assign2470_e2930: f64 = (assign2470_e2922 * assign2470_e2929);
        (assign2470_e2930, (((assign2470_e2922 * ((locals.var_maxvifi_dn4 * locals.var_afac) + (locals.var_maxvifi * locals.var_afac_dn4))) * assign2470_e2929) + (assign2470_e2922 * (((-locals.var_maxvifi_dn4) * locals.var_afac) + (assign2470_e2926 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign2470_e2922 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2470_e2922 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2470_e2932;
        locals.var_expi_dn4 = assign2470_e2932_d_n4;
        locals.var_expi_dn5 = assign2470_e2932_d_n5;
        locals.var_expi_dn6 = assign2470_e2932_d_n6;
        locals.var_expi_dn7 = assign2470_e2932_d_n7;
        locals.var_expi_dn8 = assign2470_e2932_d_n8;
        locals.var_expi_dn9 = assign2470_e2932_d_n9;
        locals.var_expi_dn10 = assign2470_e2932_d_n10;
        locals.var_expi_dn11 = assign2470_e2932_d_n11;
        locals.var_expi_rv = 0.0;

        let assign2480_e2936: f64 = (locals.var_expi - 1.0);
        let assign2480_e2937: f64 = (locals.var_is_t * assign2480_e2936);
        locals.var_ifi = assign2480_e2937;
        locals.var_ifi_dn4 = ((locals.var_is_t_dn4 * assign2480_e2936) + (locals.var_is_t * locals.var_expi_dn4));
        locals.var_ifi_dn5 = (locals.var_is_t * locals.var_expi_dn5);
        locals.var_ifi_dn6 = (locals.var_is_t * locals.var_expi_dn6);
        locals.var_ifi_dn7 = (locals.var_is_t * locals.var_expi_dn7);
        locals.var_ifi_dn8 = (locals.var_is_t * locals.var_expi_dn8);
        locals.var_ifi_dn9 = (locals.var_is_t * locals.var_expi_dn9);
        locals.var_ifi_dn10 = (locals.var_is_t * locals.var_expi_dn10);
        locals.var_ifi_dn11 = (locals.var_is_t * locals.var_expi_dn11);
        locals.var_ifi_rv = 0.0;

        let assign2490_e2941: f64 = (locals.var_nr_t * locals.var_vtv);
        let assign2490_e2942: f64 = (1.0 / assign2490_e2941);
        locals.var_afac = assign2490_e2942;
        locals.var_afac_dn4 = (-(((locals.var_nr_t_dn4 * locals.var_vtv) + (locals.var_nr_t * locals.var_vtv_dn4)) / (assign2490_e2941 * assign2490_e2941)));
        locals.var_afac_rv = 0.0;

        let assign2500_e2945: f64 = if locals.var_vbci < locals.var_maxviri { 1.0 } else { 0.0 };
        locals.var_guard80 = assign2500_e2945;
        locals.var_guard80_rv = 0.0;

        let (assign2510_e2952, assign2510_e2952_d_n4, assign2510_e2952_d_n5, assign2510_e2952_d_n6, assign2510_e2952_d_n7, assign2510_e2952_d_n8, assign2510_e2952_d_n9, assign2510_e2952_d_n10, assign2510_e2952_d_n11,) = {
    if (locals.var_guard80 != 0.0) {
        let assign2510_e2949: f64 = (locals.var_vbci * locals.var_afac);
        let assign2510_e2950: f64 = (assign2510_e2949).exp();
        (assign2510_e2950, (assign2510_e2950 * (locals.var_vbci * locals.var_afac_dn4)), 0.0, (assign2510_e2950 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign2510_e2950 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2510_e2952;
        locals.var_expi_dn4 = assign2510_e2952_d_n4;
        locals.var_expi_dn5 = assign2510_e2952_d_n5;
        locals.var_expi_dn6 = assign2510_e2952_d_n6;
        locals.var_expi_dn7 = assign2510_e2952_d_n7;
        locals.var_expi_dn8 = assign2510_e2952_d_n8;
        locals.var_expi_dn9 = assign2510_e2952_d_n9;
        locals.var_expi_dn10 = assign2510_e2952_d_n10;
        locals.var_expi_dn11 = assign2510_e2952_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign2520_e2968, assign2520_e2968_d_n4, assign2520_e2968_d_n5, assign2520_e2968_d_n6, assign2520_e2968_d_n7, assign2520_e2968_d_n8, assign2520_e2968_d_n9, assign2520_e2968_d_n10, assign2520_e2968_d_n11,) = {
    if (locals.var_guard80 == 0.0) {
        let assign2520_e2957: f64 = (locals.var_maxviri * locals.var_afac);
        let assign2520_e2958: f64 = (assign2520_e2957).exp();
        let assign2520_e2962: f64 = (locals.var_vbci - locals.var_maxviri);
        let assign2520_e2964: f64 = (assign2520_e2962 * locals.var_afac);
        let assign2520_e2965: f64 = (1.0 + assign2520_e2964);
        let assign2520_e2966: f64 = (assign2520_e2958 * assign2520_e2965);
        (assign2520_e2966, (((assign2520_e2958 * ((locals.var_maxviri_dn4 * locals.var_afac) + (locals.var_maxviri * locals.var_afac_dn4))) * assign2520_e2965) + (assign2520_e2958 * (((-locals.var_maxviri_dn4) * locals.var_afac) + (assign2520_e2962 * locals.var_afac_dn4)))), 0.0, (assign2520_e2958 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign2520_e2958 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2520_e2968;
        locals.var_expi_dn4 = assign2520_e2968_d_n4;
        locals.var_expi_dn5 = assign2520_e2968_d_n5;
        locals.var_expi_dn6 = assign2520_e2968_d_n6;
        locals.var_expi_dn7 = assign2520_e2968_d_n7;
        locals.var_expi_dn8 = assign2520_e2968_d_n8;
        locals.var_expi_dn9 = assign2520_e2968_d_n9;
        locals.var_expi_dn10 = assign2520_e2968_d_n10;
        locals.var_expi_dn11 = assign2520_e2968_d_n11;
        locals.var_expi_rv = 0.0;

        let assign2530_e2971: f64 = (locals.var_is_t * locals.var_isrr_t);
        let assign2530_e2974: f64 = (locals.var_expi - 1.0);
        let assign2530_e2975: f64 = (assign2530_e2971 * assign2530_e2974);
        locals.var_iri = assign2530_e2975;
        locals.var_iri_dn4 = ((((locals.var_is_t_dn4 * locals.var_isrr_t) + (locals.var_is_t * locals.var_isrr_t_dn4)) * assign2530_e2974) + (assign2530_e2971 * locals.var_expi_dn4));
        locals.var_iri_dn5 = (assign2530_e2971 * locals.var_expi_dn5);
        locals.var_iri_dn6 = (assign2530_e2971 * locals.var_expi_dn6);
        locals.var_iri_dn7 = (assign2530_e2971 * locals.var_expi_dn7);
        locals.var_iri_dn8 = (assign2530_e2971 * locals.var_expi_dn8);
        locals.var_iri_dn9 = (assign2530_e2971 * locals.var_expi_dn9);
        locals.var_iri_dn10 = (assign2530_e2971 * locals.var_expi_dn10);
        locals.var_iri_dn11 = (assign2530_e2971 * locals.var_expi_dn11);
        locals.var_iri_rv = 0.0;

        let assign2540_e2979: f64 = (locals.var_qdbe * locals.var_iver);
        let assign2540_e2980: f64 = (1.0 + assign2540_e2979);
        let assign2540_e2983: f64 = (locals.var_qdbc * locals.var_ivef);
        let assign2540_e2984: f64 = (assign2540_e2980 + assign2540_e2983);
        let assign2540_e2986: f64 = (assign2540_e2984 - 0.0001);
        locals.var_q1z = assign2540_e2986;
        locals.var_q1z_dn4 = (((locals.var_qdbe_dn4 * locals.var_iver) + (locals.var_qdbe * locals.var_iver_dn4)) + ((locals.var_qdbc_dn4 * locals.var_ivef) + (locals.var_qdbc * locals.var_ivef_dn4)));
        locals.var_q1z_dn6 = (locals.var_qdbc_dn6 * locals.var_ivef);
        locals.var_q1z_dn8 = ((locals.var_qdbe_dn8 * locals.var_iver) + (locals.var_qdbc_dn8 * locals.var_ivef));
        locals.var_q1z_dn9 = (locals.var_qdbe_dn9 * locals.var_iver);
        locals.var_q1z_rv = 0.0;

        let assign2550_e2990: f64 = (locals.var_q1z * locals.var_q1z);
        let assign2550_e2992: f64 = (assign2550_e2990 + 1e-8);
        let assign2550_e2993: f64 = (assign2550_e2992).sqrt();
        let assign2550_e2995: f64 = (assign2550_e2993 + locals.var_q1z);
        let assign2550_e2996: f64 = (0.5 * assign2550_e2995);
        let assign2550_e2998: f64 = (assign2550_e2996 + 0.0001);
        locals.var_q1 = assign2550_e2998;
        locals.var_q1_dn4 = (0.5 * ((((locals.var_q1z_dn4 * locals.var_q1z) + (locals.var_q1z * locals.var_q1z_dn4)) / (2.0 * assign2550_e2993)) + locals.var_q1z_dn4));
        locals.var_q1_dn6 = (0.5 * ((((locals.var_q1z_dn6 * locals.var_q1z) + (locals.var_q1z * locals.var_q1z_dn6)) / (2.0 * assign2550_e2993)) + locals.var_q1z_dn6));
        locals.var_q1_dn8 = (0.5 * ((((locals.var_q1z_dn8 * locals.var_q1z) + (locals.var_q1z * locals.var_q1z_dn8)) / (2.0 * assign2550_e2993)) + locals.var_q1z_dn8));
        locals.var_q1_dn9 = (0.5 * ((((locals.var_q1z_dn9 * locals.var_q1z) + (locals.var_q1z * locals.var_q1z_dn9)) / (2.0 * assign2550_e2993)) + locals.var_q1z_dn9));
        locals.var_q1_rv = 0.0;

        let assign2560_e3001: f64 = (locals.var_ifi * locals.var_iikf);
        let assign2560_e3004: f64 = (locals.var_iri * locals.var_iikr);
        let assign2560_e3005: f64 = (assign2560_e3001 + assign2560_e3004);
        locals.var_q2 = assign2560_e3005;
        locals.var_q2_dn4 = (((locals.var_ifi_dn4 * locals.var_iikf) + (locals.var_ifi * locals.var_iikf_dn4)) + (locals.var_iri_dn4 * locals.var_iikr));
        locals.var_q2_dn5 = ((locals.var_ifi_dn5 * locals.var_iikf) + (locals.var_iri_dn5 * locals.var_iikr));
        locals.var_q2_dn6 = ((locals.var_ifi_dn6 * locals.var_iikf) + (locals.var_iri_dn6 * locals.var_iikr));
        locals.var_q2_dn7 = ((locals.var_ifi_dn7 * locals.var_iikf) + (locals.var_iri_dn7 * locals.var_iikr));
        locals.var_q2_dn8 = ((locals.var_ifi_dn8 * locals.var_iikf) + (locals.var_iri_dn8 * locals.var_iikr));
        locals.var_q2_dn9 = ((locals.var_ifi_dn9 * locals.var_iikf) + (locals.var_iri_dn9 * locals.var_iikr));
        locals.var_q2_dn10 = ((locals.var_ifi_dn10 * locals.var_iikf) + (locals.var_iri_dn10 * locals.var_iikr));
        locals.var_q2_dn11 = ((locals.var_ifi_dn11 * locals.var_iikf) + (locals.var_iri_dn11 * locals.var_iikr));
        locals.var_q2_rv = 0.0;

        let assign2570_e3008: f64 = if p.p30 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign2570_e3008;
        locals.var_guard81_rv = 0.0;

        let (assign2580_e3020, assign2580_e3020_d_n4, assign2580_e3020_d_n5, assign2580_e3020_d_n6, assign2580_e3020_d_n7, assign2580_e3020_d_n8, assign2580_e3020_d_n9, assign2580_e3020_d_n10, assign2580_e3020_d_n11,) = {
    if (locals.var_guard81 != 0.0) {
        let assign2580_e3013: f64 = (1.0 / p.p73);
        let assign2580_e3014: f64 = (locals.var_q1).powf(assign2580_e3013);
        let assign2580_e3017: f64 = (4.0 * locals.var_q2);
        let assign2580_e3018: f64 = (assign2580_e3014 + assign2580_e3017);
        (assign2580_e3018, (if 0.0 == 0.0 && ((assign2580_e3013) as f64).is_finite() && ((assign2580_e3013) as f64).fract() == 0.0 { if assign2580_e3013 == 0.0 { 0.0 } else { (assign2580_e3013 * ((locals.var_q1).powf(assign2580_e3013 - 1.0) * locals.var_q1_dn4)) } } else { (assign2580_e3014 * (assign2580_e3013 * (locals.var_q1_dn4 / locals.var_q1))) } + (4.0 * locals.var_q2_dn4)), (4.0 * locals.var_q2_dn5), (if 0.0 == 0.0 && ((assign2580_e3013) as f64).is_finite() && ((assign2580_e3013) as f64).fract() == 0.0 { if assign2580_e3013 == 0.0 { 0.0 } else { (assign2580_e3013 * ((locals.var_q1).powf(assign2580_e3013 - 1.0) * locals.var_q1_dn6)) } } else { (assign2580_e3014 * (assign2580_e3013 * (locals.var_q1_dn6 / locals.var_q1))) } + (4.0 * locals.var_q2_dn6)), (4.0 * locals.var_q2_dn7), (if 0.0 == 0.0 && ((assign2580_e3013) as f64).is_finite() && ((assign2580_e3013) as f64).fract() == 0.0 { if assign2580_e3013 == 0.0 { 0.0 } else { (assign2580_e3013 * ((locals.var_q1).powf(assign2580_e3013 - 1.0) * locals.var_q1_dn8)) } } else { (assign2580_e3014 * (assign2580_e3013 * (locals.var_q1_dn8 / locals.var_q1))) } + (4.0 * locals.var_q2_dn8)), (if 0.0 == 0.0 && ((assign2580_e3013) as f64).is_finite() && ((assign2580_e3013) as f64).fract() == 0.0 { if assign2580_e3013 == 0.0 { 0.0 } else { (assign2580_e3013 * ((locals.var_q1).powf(assign2580_e3013 - 1.0) * locals.var_q1_dn9)) } } else { (assign2580_e3014 * (assign2580_e3013 * (locals.var_q1_dn9 / locals.var_q1))) } + (4.0 * locals.var_q2_dn9)), (4.0 * locals.var_q2_dn10), (4.0 * locals.var_q2_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign2580_e3020;
        locals.var_arg_dn4 = assign2580_e3020_d_n4;
        locals.var_arg_dn5 = assign2580_e3020_d_n5;
        locals.var_arg_dn6 = assign2580_e3020_d_n6;
        locals.var_arg_dn7 = assign2580_e3020_d_n7;
        locals.var_arg_dn8 = assign2580_e3020_d_n8;
        locals.var_arg_dn9 = assign2580_e3020_d_n9;
        locals.var_arg_dn10 = assign2580_e3020_d_n10;
        locals.var_arg_dn11 = assign2580_e3020_d_n11;
        locals.var_arg_rv = 0.0;

        let assign2590_e3023: f64 = if locals.var_arg > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign2590_e3023;
        locals.var_guard82_rv = 0.0;

        let (assign2600_e3035, assign2600_e3035_d_n4, assign2600_e3035_d_n5, assign2600_e3035_d_n6, assign2600_e3035_d_n7, assign2600_e3035_d_n8, assign2600_e3035_d_n9, assign2600_e3035_d_n10, assign2600_e3035_d_n11,) = {
    if ((locals.var_guard81 != 0.0) && (locals.var_guard82 != 0.0)) {
        let assign2600_e3031: f64 = (locals.var_arg).powf(p.p73);
        let assign2600_e3032: f64 = (locals.var_q1 + assign2600_e3031);
        let assign2600_e3033: f64 = (0.5 * assign2600_e3032);
        (assign2600_e3033, (0.5 * (locals.var_q1_dn4 + if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn4)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn4 / locals.var_arg))) })), (0.5 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn5)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn5 / locals.var_arg))) }), (0.5 * (locals.var_q1_dn6 + if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn6)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn6 / locals.var_arg))) })), (0.5 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn7)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn7 / locals.var_arg))) }), (0.5 * (locals.var_q1_dn8 + if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn8)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn8 / locals.var_arg))) })), (0.5 * (locals.var_q1_dn9 + if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn9)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn9 / locals.var_arg))) })), (0.5 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn10)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn10 / locals.var_arg))) }), (0.5 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn11)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn11 / locals.var_arg))) }),)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign2600_e3035;
        locals.var_qb_dn4 = assign2600_e3035_d_n4;
        locals.var_qb_dn5 = assign2600_e3035_d_n5;
        locals.var_qb_dn6 = assign2600_e3035_d_n6;
        locals.var_qb_dn7 = assign2600_e3035_d_n7;
        locals.var_qb_dn8 = assign2600_e3035_d_n8;
        locals.var_qb_dn9 = assign2600_e3035_d_n9;
        locals.var_qb_dn10 = assign2600_e3035_d_n10;
        locals.var_qb_dn11 = assign2600_e3035_d_n11;
        locals.var_qb_rv = 0.0;

        let (assign2610_e3048, assign2610_e3048_d_n4, assign2610_e3048_d_n5, assign2610_e3048_d_n6, assign2610_e3048_d_n7, assign2610_e3048_d_n8, assign2610_e3048_d_n9, assign2610_e3048_d_n10, assign2610_e3048_d_n11,) = {
    if ((locals.var_guard81 != 0.0) && (locals.var_guard82 == 0.0)) {
        let assign2610_e3044: f64 = (1e-8_f64).powf(p.p73);
        let assign2610_e3045: f64 = (locals.var_q1 + assign2610_e3044);
        let assign2610_e3046: f64 = (0.5 * assign2610_e3045);
        (assign2610_e3046, (0.5 * locals.var_q1_dn4), 0.0, (0.5 * locals.var_q1_dn6), 0.0, (0.5 * locals.var_q1_dn8), (0.5 * locals.var_q1_dn9), 0.0, 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign2610_e3048;
        locals.var_qb_dn4 = assign2610_e3048_d_n4;
        locals.var_qb_dn5 = assign2610_e3048_d_n5;
        locals.var_qb_dn6 = assign2610_e3048_d_n6;
        locals.var_qb_dn7 = assign2610_e3048_d_n7;
        locals.var_qb_dn8 = assign2610_e3048_d_n8;
        locals.var_qb_dn9 = assign2610_e3048_d_n9;
        locals.var_qb_dn10 = assign2610_e3048_d_n10;
        locals.var_qb_dn11 = assign2610_e3048_d_n11;
        locals.var_qb_rv = 0.0;

        let (assign2620_e3057, assign2620_e3057_d_n4, assign2620_e3057_d_n5, assign2620_e3057_d_n6, assign2620_e3057_d_n7, assign2620_e3057_d_n8, assign2620_e3057_d_n9, assign2620_e3057_d_n10, assign2620_e3057_d_n11,) = {
    if (locals.var_guard81 == 0.0) {
        let assign2620_e3054: f64 = (4.0 * locals.var_q2);
        let assign2620_e3055: f64 = (1.0 + assign2620_e3054);
        (assign2620_e3055, (4.0 * locals.var_q2_dn4), (4.0 * locals.var_q2_dn5), (4.0 * locals.var_q2_dn6), (4.0 * locals.var_q2_dn7), (4.0 * locals.var_q2_dn8), (4.0 * locals.var_q2_dn9), (4.0 * locals.var_q2_dn10), (4.0 * locals.var_q2_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign2620_e3057;
        locals.var_arg_dn4 = assign2620_e3057_d_n4;
        locals.var_arg_dn5 = assign2620_e3057_d_n5;
        locals.var_arg_dn6 = assign2620_e3057_d_n6;
        locals.var_arg_dn7 = assign2620_e3057_d_n7;
        locals.var_arg_dn8 = assign2620_e3057_d_n8;
        locals.var_arg_dn9 = assign2620_e3057_d_n9;
        locals.var_arg_dn10 = assign2620_e3057_d_n10;
        locals.var_arg_dn11 = assign2620_e3057_d_n11;
        locals.var_arg_rv = 0.0;

        let assign2630_e3060: f64 = if locals.var_arg > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign2630_e3060;
        locals.var_guard83_rv = 0.0;

        let (assign2640_e3075, assign2640_e3075_d_n4, assign2640_e3075_d_n5, assign2640_e3075_d_n6, assign2640_e3075_d_n7, assign2640_e3075_d_n8, assign2640_e3075_d_n9, assign2640_e3075_d_n10, assign2640_e3075_d_n11,) = {
    if ((locals.var_guard81 == 0.0) && (locals.var_guard83 != 0.0)) {
        let assign2640_e3067: f64 = (0.5 * locals.var_q1);
        let assign2640_e3071: f64 = (locals.var_arg).powf(p.p73);
        let assign2640_e3072: f64 = (1.0 + assign2640_e3071);
        let assign2640_e3073: f64 = (assign2640_e3067 * assign2640_e3072);
        (assign2640_e3073, (((0.5 * locals.var_q1_dn4) * assign2640_e3072) + (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn4)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn4 / locals.var_arg))) })), (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn5)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn5 / locals.var_arg))) }), (((0.5 * locals.var_q1_dn6) * assign2640_e3072) + (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn6)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn6 / locals.var_arg))) })), (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn7)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn7 / locals.var_arg))) }), (((0.5 * locals.var_q1_dn8) * assign2640_e3072) + (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn8)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn8 / locals.var_arg))) })), (((0.5 * locals.var_q1_dn9) * assign2640_e3072) + (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn9)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn9 / locals.var_arg))) })), (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn10)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn10 / locals.var_arg))) }), (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn11)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn11 / locals.var_arg))) }),)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign2640_e3075;
        locals.var_qb_dn4 = assign2640_e3075_d_n4;
        locals.var_qb_dn5 = assign2640_e3075_d_n5;
        locals.var_qb_dn6 = assign2640_e3075_d_n6;
        locals.var_qb_dn7 = assign2640_e3075_d_n7;
        locals.var_qb_dn8 = assign2640_e3075_d_n8;
        locals.var_qb_dn9 = assign2640_e3075_d_n9;
        locals.var_qb_dn10 = assign2640_e3075_d_n10;
        locals.var_qb_dn11 = assign2640_e3075_d_n11;
        locals.var_qb_rv = 0.0;

        let (assign2650_e3091, assign2650_e3091_d_n4, assign2650_e3091_d_n5, assign2650_e3091_d_n6, assign2650_e3091_d_n7, assign2650_e3091_d_n8, assign2650_e3091_d_n9, assign2650_e3091_d_n10, assign2650_e3091_d_n11,) = {
    if ((locals.var_guard81 == 0.0) && (locals.var_guard83 == 0.0)) {
        let assign2650_e3083: f64 = (0.5 * locals.var_q1);
        let assign2650_e3087: f64 = (1e-8_f64).powf(p.p73);
        let assign2650_e3088: f64 = (1.0 + assign2650_e3087);
        let assign2650_e3089: f64 = (assign2650_e3083 * assign2650_e3088);
        (assign2650_e3089, ((0.5 * locals.var_q1_dn4) * assign2650_e3088), 0.0, ((0.5 * locals.var_q1_dn6) * assign2650_e3088), 0.0, ((0.5 * locals.var_q1_dn8) * assign2650_e3088), ((0.5 * locals.var_q1_dn9) * assign2650_e3088), 0.0, 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign2650_e3091;
        locals.var_qb_dn4 = assign2650_e3091_d_n4;
        locals.var_qb_dn5 = assign2650_e3091_d_n5;
        locals.var_qb_dn6 = assign2650_e3091_d_n6;
        locals.var_qb_dn7 = assign2650_e3091_d_n7;
        locals.var_qb_dn8 = assign2650_e3091_d_n8;
        locals.var_qb_dn9 = assign2650_e3091_d_n9;
        locals.var_qb_dn10 = assign2650_e3091_d_n10;
        locals.var_qb_dn11 = assign2650_e3091_d_n11;
        locals.var_qb_rv = 0.0;

        let assign2690_e3101: f64 = if p.p31 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign2690_e3101;
        locals.var_guard84_rv = 0.0;

        let (assign2700_e3109, assign2700_e3109_d_n4,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2700_e3106: f64 = (p.p33 * locals.var_vtv);
        let assign2700_e3107: f64 = (1.0 / assign2700_e3106);
        (assign2700_e3107, (-((p.p33 * locals.var_vtv_dn4) / (assign2700_e3106 * assign2700_e3106))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign2700_e3109;
        locals.var_afac_dn4 = assign2700_e3109_d_n4;
        locals.var_afac_rv = 0.0;

        let assign2710_e3112: f64 = if locals.var_vbep < locals.var_maxvip { 1.0 } else { 0.0 };
        locals.var_guard85 = assign2710_e3112;
        locals.var_guard85_rv = 0.0;

        let (assign2720_e3121, assign2720_e3121_d_n4, assign2720_e3121_d_n5, assign2720_e3121_d_n6, assign2720_e3121_d_n7, assign2720_e3121_d_n8, assign2720_e3121_d_n9, assign2720_e3121_d_n10, assign2720_e3121_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign2720_e3118: f64 = (locals.var_vbep * locals.var_afac);
        let assign2720_e3119: f64 = (assign2720_e3118).exp();
        (assign2720_e3119, (assign2720_e3119 * (locals.var_vbep * locals.var_afac_dn4)), 0.0, 0.0, (assign2720_e3119 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign2720_e3119 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2720_e3121;
        locals.var_expi_dn4 = assign2720_e3121_d_n4;
        locals.var_expi_dn5 = assign2720_e3121_d_n5;
        locals.var_expi_dn6 = assign2720_e3121_d_n6;
        locals.var_expi_dn7 = assign2720_e3121_d_n7;
        locals.var_expi_dn8 = assign2720_e3121_d_n8;
        locals.var_expi_dn9 = assign2720_e3121_d_n9;
        locals.var_expi_dn10 = assign2720_e3121_d_n10;
        locals.var_expi_dn11 = assign2720_e3121_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign2730_e3139, assign2730_e3139_d_n4, assign2730_e3139_d_n5, assign2730_e3139_d_n6, assign2730_e3139_d_n7, assign2730_e3139_d_n8, assign2730_e3139_d_n9, assign2730_e3139_d_n10, assign2730_e3139_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard85 == 0.0)) {
        let assign2730_e3128: f64 = (locals.var_maxvip * locals.var_afac);
        let assign2730_e3129: f64 = (assign2730_e3128).exp();
        let assign2730_e3133: f64 = (locals.var_vbep - locals.var_maxvip);
        let assign2730_e3135: f64 = (assign2730_e3133 * locals.var_afac);
        let assign2730_e3136: f64 = (1.0 + assign2730_e3135);
        let assign2730_e3137: f64 = (assign2730_e3129 * assign2730_e3136);
        (assign2730_e3137, (((assign2730_e3129 * ((locals.var_maxvip_dn4 * locals.var_afac) + (locals.var_maxvip * locals.var_afac_dn4))) * assign2730_e3136) + (assign2730_e3129 * (((-locals.var_maxvip_dn4) * locals.var_afac) + (assign2730_e3133 * locals.var_afac_dn4)))), 0.0, 0.0, (assign2730_e3129 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign2730_e3129 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2730_e3139;
        locals.var_expi_dn4 = assign2730_e3139_d_n4;
        locals.var_expi_dn5 = assign2730_e3139_d_n5;
        locals.var_expi_dn6 = assign2730_e3139_d_n6;
        locals.var_expi_dn7 = assign2730_e3139_d_n7;
        locals.var_expi_dn8 = assign2730_e3139_d_n8;
        locals.var_expi_dn9 = assign2730_e3139_d_n9;
        locals.var_expi_dn10 = assign2730_e3139_d_n10;
        locals.var_expi_dn11 = assign2730_e3139_d_n11;
        locals.var_expi_rv = 0.0;

        let assign2740_e3142: f64 = if locals.var_vbci < locals.var_maxvip { 1.0 } else { 0.0 };
        locals.var_guard86 = assign2740_e3142;
        locals.var_guard86_rv = 0.0;

        let (assign2750_e3151, assign2750_e3151_d_n4, assign2750_e3151_d_n5, assign2750_e3151_d_n6, assign2750_e3151_d_n7, assign2750_e3151_d_n8, assign2750_e3151_d_n9, assign2750_e3151_d_n10, assign2750_e3151_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign2750_e3148: f64 = (locals.var_vbci * locals.var_afac);
        let assign2750_e3149: f64 = (assign2750_e3148).exp();
        (assign2750_e3149, (assign2750_e3149 * (locals.var_vbci * locals.var_afac_dn4)), 0.0, (assign2750_e3149 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign2750_e3149 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign2750_e3151;
        locals.var_expx_dn4 = assign2750_e3151_d_n4;
        locals.var_expx_dn5 = assign2750_e3151_d_n5;
        locals.var_expx_dn6 = assign2750_e3151_d_n6;
        locals.var_expx_dn7 = assign2750_e3151_d_n7;
        locals.var_expx_dn8 = assign2750_e3151_d_n8;
        locals.var_expx_dn9 = assign2750_e3151_d_n9;
        locals.var_expx_dn10 = assign2750_e3151_d_n10;
        locals.var_expx_dn11 = assign2750_e3151_d_n11;
        locals.var_expx_rv = 0.0;

        let (assign2760_e3169, assign2760_e3169_d_n4, assign2760_e3169_d_n5, assign2760_e3169_d_n6, assign2760_e3169_d_n7, assign2760_e3169_d_n8, assign2760_e3169_d_n9, assign2760_e3169_d_n10, assign2760_e3169_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard86 == 0.0)) {
        let assign2760_e3158: f64 = (locals.var_maxvip * locals.var_afac);
        let assign2760_e3159: f64 = (assign2760_e3158).exp();
        let assign2760_e3163: f64 = (locals.var_vbci - locals.var_maxvip);
        let assign2760_e3165: f64 = (assign2760_e3163 * locals.var_afac);
        let assign2760_e3166: f64 = (1.0 + assign2760_e3165);
        let assign2760_e3167: f64 = (assign2760_e3159 * assign2760_e3166);
        (assign2760_e3167, (((assign2760_e3159 * ((locals.var_maxvip_dn4 * locals.var_afac) + (locals.var_maxvip * locals.var_afac_dn4))) * assign2760_e3166) + (assign2760_e3159 * (((-locals.var_maxvip_dn4) * locals.var_afac) + (assign2760_e3163 * locals.var_afac_dn4)))), 0.0, (assign2760_e3159 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign2760_e3159 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign2760_e3169;
        locals.var_expx_dn4 = assign2760_e3169_d_n4;
        locals.var_expx_dn5 = assign2760_e3169_d_n5;
        locals.var_expx_dn6 = assign2760_e3169_d_n6;
        locals.var_expx_dn7 = assign2760_e3169_d_n7;
        locals.var_expx_dn8 = assign2760_e3169_d_n8;
        locals.var_expx_dn9 = assign2760_e3169_d_n9;
        locals.var_expx_dn10 = assign2760_e3169_d_n10;
        locals.var_expx_dn11 = assign2760_e3169_d_n11;
        locals.var_expx_rv = 0.0;

        let (assign2770_e3185, assign2770_e3185_d_n4, assign2770_e3185_d_n5, assign2770_e3185_d_n6, assign2770_e3185_d_n7, assign2770_e3185_d_n8, assign2770_e3185_d_n9, assign2770_e3185_d_n10, assign2770_e3185_d_n11,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2770_e3174: f64 = (p.p32 * locals.var_expi);
        let assign2770_e3177: f64 = (1.0 - p.p32);
        let assign2770_e3179: f64 = (assign2770_e3177 * locals.var_expx);
        let assign2770_e3180: f64 = (assign2770_e3174 + assign2770_e3179);
        let assign2770_e3182: f64 = (assign2770_e3180 - 1.0);
        let assign2770_e3183: f64 = (locals.var_isp_t * assign2770_e3182);
        (assign2770_e3183, ((locals.var_isp_t_dn4 * assign2770_e3182) + (locals.var_isp_t * ((p.p32 * locals.var_expi_dn4) + (assign2770_e3177 * locals.var_expx_dn4)))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn5) + (assign2770_e3177 * locals.var_expx_dn5))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn6) + (assign2770_e3177 * locals.var_expx_dn6))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn7) + (assign2770_e3177 * locals.var_expx_dn7))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn8) + (assign2770_e3177 * locals.var_expx_dn8))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn9) + (assign2770_e3177 * locals.var_expx_dn9))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn10) + (assign2770_e3177 * locals.var_expx_dn10))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn11) + (assign2770_e3177 * locals.var_expx_dn11))),)
    } else {
        (locals.var_ifp, locals.var_ifp_dn4, locals.var_ifp_dn5, locals.var_ifp_dn6, locals.var_ifp_dn7, locals.var_ifp_dn8, locals.var_ifp_dn9, locals.var_ifp_dn10, locals.var_ifp_dn11,)
    }
};
        locals.var_ifp = assign2770_e3185;
        locals.var_ifp_dn4 = assign2770_e3185_d_n4;
        locals.var_ifp_dn5 = assign2770_e3185_d_n5;
        locals.var_ifp_dn6 = assign2770_e3185_d_n6;
        locals.var_ifp_dn7 = assign2770_e3185_d_n7;
        locals.var_ifp_dn8 = assign2770_e3185_d_n8;
        locals.var_ifp_dn9 = assign2770_e3185_d_n9;
        locals.var_ifp_dn10 = assign2770_e3185_d_n10;
        locals.var_ifp_dn11 = assign2770_e3185_d_n11;
        locals.var_ifp_rv = 0.0;

        let (assign2780_e3191, assign2780_e3191_d_n4, assign2780_e3191_d_n5, assign2780_e3191_d_n6, assign2780_e3191_d_n7, assign2780_e3191_d_n8, assign2780_e3191_d_n9, assign2780_e3191_d_n10, assign2780_e3191_d_n11,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2780_e3189: f64 = (locals.var_ifp * locals.var_iikp);
        (assign2780_e3189, (locals.var_ifp_dn4 * locals.var_iikp), (locals.var_ifp_dn5 * locals.var_iikp), (locals.var_ifp_dn6 * locals.var_iikp), (locals.var_ifp_dn7 * locals.var_iikp), (locals.var_ifp_dn8 * locals.var_iikp), (locals.var_ifp_dn9 * locals.var_iikp), (locals.var_ifp_dn10 * locals.var_iikp), (locals.var_ifp_dn11 * locals.var_iikp),)
    } else {
        (locals.var_q2p, locals.var_q2p_dn4, locals.var_q2p_dn5, locals.var_q2p_dn6, locals.var_q2p_dn7, locals.var_q2p_dn8, locals.var_q2p_dn9, locals.var_q2p_dn10, locals.var_q2p_dn11,)
    }
};
        locals.var_q2p = assign2780_e3191;
        locals.var_q2p_dn4 = assign2780_e3191_d_n4;
        locals.var_q2p_dn5 = assign2780_e3191_d_n5;
        locals.var_q2p_dn6 = assign2780_e3191_d_n6;
        locals.var_q2p_dn7 = assign2780_e3191_d_n7;
        locals.var_q2p_dn8 = assign2780_e3191_d_n8;
        locals.var_q2p_dn9 = assign2780_e3191_d_n9;
        locals.var_q2p_dn10 = assign2780_e3191_d_n10;
        locals.var_q2p_dn11 = assign2780_e3191_d_n11;
        locals.var_q2p_rv = 0.0;

        let (assign2790_e3199, assign2790_e3199_d_n4, assign2790_e3199_d_n5, assign2790_e3199_d_n6, assign2790_e3199_d_n7, assign2790_e3199_d_n8, assign2790_e3199_d_n9, assign2790_e3199_d_n10, assign2790_e3199_d_n11,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2790_e3196: f64 = (4.0 * locals.var_q2p);
        let assign2790_e3197: f64 = (1.0 + assign2790_e3196);
        (assign2790_e3197, (4.0 * locals.var_q2p_dn4), (4.0 * locals.var_q2p_dn5), (4.0 * locals.var_q2p_dn6), (4.0 * locals.var_q2p_dn7), (4.0 * locals.var_q2p_dn8), (4.0 * locals.var_q2p_dn9), (4.0 * locals.var_q2p_dn10), (4.0 * locals.var_q2p_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign2790_e3199;
        locals.var_arg_dn4 = assign2790_e3199_d_n4;
        locals.var_arg_dn5 = assign2790_e3199_d_n5;
        locals.var_arg_dn6 = assign2790_e3199_d_n6;
        locals.var_arg_dn7 = assign2790_e3199_d_n7;
        locals.var_arg_dn8 = assign2790_e3199_d_n8;
        locals.var_arg_dn9 = assign2790_e3199_d_n9;
        locals.var_arg_dn10 = assign2790_e3199_d_n10;
        locals.var_arg_dn11 = assign2790_e3199_d_n11;
        locals.var_arg_rv = 0.0;

        let assign2830_e3228: f64 = if locals.var_vbcp < locals.var_maxvip { 1.0 } else { 0.0 };
        locals.var_guard88 = assign2830_e3228;
        locals.var_guard88_rv = 0.0;

        let (assign2840_e3237, assign2840_e3237_d_n4, assign2840_e3237_d_n5, assign2840_e3237_d_n6, assign2840_e3237_d_n7, assign2840_e3237_d_n8, assign2840_e3237_d_n9, assign2840_e3237_d_n10, assign2840_e3237_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard88 != 0.0)) {
        let assign2840_e3234: f64 = (locals.var_vbcp * locals.var_afac);
        let assign2840_e3235: f64 = (assign2840_e3234).exp();
        (assign2840_e3235, (assign2840_e3235 * (locals.var_vbcp * locals.var_afac_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0, (assign2840_e3235 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign2840_e3235 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2840_e3237;
        locals.var_expi_dn4 = assign2840_e3237_d_n4;
        locals.var_expi_dn5 = assign2840_e3237_d_n5;
        locals.var_expi_dn6 = assign2840_e3237_d_n6;
        locals.var_expi_dn7 = assign2840_e3237_d_n7;
        locals.var_expi_dn8 = assign2840_e3237_d_n8;
        locals.var_expi_dn9 = assign2840_e3237_d_n9;
        locals.var_expi_dn10 = assign2840_e3237_d_n10;
        locals.var_expi_dn11 = assign2840_e3237_d_n11;
        locals.var_expi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2850_e3255, assign2850_e3255_d_n4, assign2850_e3255_d_n5, assign2850_e3255_d_n6, assign2850_e3255_d_n7, assign2850_e3255_d_n8, assign2850_e3255_d_n9, assign2850_e3255_d_n10, assign2850_e3255_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard88 == 0.0)) {
        let assign2850_e3244: f64 = (locals.var_maxvip * locals.var_afac);
        let assign2850_e3245: f64 = (assign2850_e3244).exp();
        let assign2850_e3249: f64 = (locals.var_vbcp - locals.var_maxvip);
        let assign2850_e3251: f64 = (assign2850_e3249 * locals.var_afac);
        let assign2850_e3252: f64 = (1.0 + assign2850_e3251);
        let assign2850_e3253: f64 = (assign2850_e3245 * assign2850_e3252);
        (assign2850_e3253, (((assign2850_e3245 * ((locals.var_maxvip_dn4 * locals.var_afac) + (locals.var_maxvip * locals.var_afac_dn4))) * assign2850_e3252) + (assign2850_e3245 * (((-locals.var_maxvip_dn4) * locals.var_afac) + (assign2850_e3249 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, 0.0, 0.0, (assign2850_e3245 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign2850_e3245 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2850_e3255;
        locals.var_expi_dn4 = assign2850_e3255_d_n4;
        locals.var_expi_dn5 = assign2850_e3255_d_n5;
        locals.var_expi_dn6 = assign2850_e3255_d_n6;
        locals.var_expi_dn7 = assign2850_e3255_d_n7;
        locals.var_expi_dn8 = assign2850_e3255_d_n8;
        locals.var_expi_dn9 = assign2850_e3255_d_n9;
        locals.var_expi_dn10 = assign2850_e3255_d_n10;
        locals.var_expi_dn11 = assign2850_e3255_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign2880_e3276, assign2880_e3276_d_n4, assign2880_e3276_d_n5, assign2880_e3276_d_n6, assign2880_e3276_d_n7, assign2880_e3276_d_n8, assign2880_e3276_d_n9, assign2880_e3276_d_n10, assign2880_e3276_d_n11,) = {
    if (locals.var_guard84 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ifp, locals.var_ifp_dn4, locals.var_ifp_dn5, locals.var_ifp_dn6, locals.var_ifp_dn7, locals.var_ifp_dn8, locals.var_ifp_dn9, locals.var_ifp_dn10, locals.var_ifp_dn11,)
    }
};
        locals.var_ifp = assign2880_e3276;
        locals.var_ifp_dn4 = assign2880_e3276_d_n4;
        locals.var_ifp_dn5 = assign2880_e3276_d_n5;
        locals.var_ifp_dn6 = assign2880_e3276_d_n6;
        locals.var_ifp_dn7 = assign2880_e3276_d_n7;
        locals.var_ifp_dn8 = assign2880_e3276_d_n8;
        locals.var_ifp_dn9 = assign2880_e3276_d_n9;
        locals.var_ifp_dn10 = assign2880_e3276_d_n10;
        locals.var_ifp_dn11 = assign2880_e3276_d_n11;
        locals.var_ifp_rv = 0.0;

        let assign2910_e3289: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign2910_e3289;
        locals.var_guard89_rv = 0.0;

        let (assign2920_e3297, assign2920_e3297_d_n4,) = {
    if (locals.var_guard89 != 0.0) {
        let assign2920_e3294: f64 = (p.p56 * locals.var_vtv);
        let assign2920_e3295: f64 = (1.0 / assign2920_e3294);
        (assign2920_e3295, (-((p.p56 * locals.var_vtv_dn4) / (assign2920_e3294 * assign2920_e3294))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign2920_e3297;
        locals.var_afac_dn4 = assign2920_e3297_d_n4;
        locals.var_afac_rv = 0.0;

        let assign2930_e3300: f64 = if locals.var_vbei < locals.var_maxvibei { 1.0 } else { 0.0 };
        locals.var_guard90 = assign2930_e3300;
        locals.var_guard90_rv = 0.0;

        let (assign2940_e3309, assign2940_e3309_d_n4, assign2940_e3309_d_n5, assign2940_e3309_d_n6, assign2940_e3309_d_n7, assign2940_e3309_d_n8, assign2940_e3309_d_n9, assign2940_e3309_d_n10, assign2940_e3309_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign2940_e3306: f64 = (locals.var_vbei * locals.var_afac);
        let assign2940_e3307: f64 = (assign2940_e3306).exp();
        (assign2940_e3307, (assign2940_e3307 * (locals.var_vbei * locals.var_afac_dn4)), 0.0, 0.0, 0.0, (assign2940_e3307 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2940_e3307 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2940_e3309;
        locals.var_expi_dn4 = assign2940_e3309_d_n4;
        locals.var_expi_dn5 = assign2940_e3309_d_n5;
        locals.var_expi_dn6 = assign2940_e3309_d_n6;
        locals.var_expi_dn7 = assign2940_e3309_d_n7;
        locals.var_expi_dn8 = assign2940_e3309_d_n8;
        locals.var_expi_dn9 = assign2940_e3309_d_n9;
        locals.var_expi_dn10 = assign2940_e3309_d_n10;
        locals.var_expi_dn11 = assign2940_e3309_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign2950_e3327, assign2950_e3327_d_n4, assign2950_e3327_d_n5, assign2950_e3327_d_n6, assign2950_e3327_d_n7, assign2950_e3327_d_n8, assign2950_e3327_d_n9, assign2950_e3327_d_n10, assign2950_e3327_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard90 == 0.0)) {
        let assign2950_e3316: f64 = (locals.var_maxvibei * locals.var_afac);
        let assign2950_e3317: f64 = (assign2950_e3316).exp();
        let assign2950_e3321: f64 = (locals.var_vbei - locals.var_maxvibei);
        let assign2950_e3323: f64 = (assign2950_e3321 * locals.var_afac);
        let assign2950_e3324: f64 = (1.0 + assign2950_e3323);
        let assign2950_e3325: f64 = (assign2950_e3317 * assign2950_e3324);
        (assign2950_e3325, (((assign2950_e3317 * ((locals.var_maxvibei_dn4 * locals.var_afac) + (locals.var_maxvibei * locals.var_afac_dn4))) * assign2950_e3324) + (assign2950_e3317 * (((-locals.var_maxvibei_dn4) * locals.var_afac) + (assign2950_e3321 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign2950_e3317 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2950_e3317 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2950_e3327;
        locals.var_expi_dn4 = assign2950_e3327_d_n4;
        locals.var_expi_dn5 = assign2950_e3327_d_n5;
        locals.var_expi_dn6 = assign2950_e3327_d_n6;
        locals.var_expi_dn7 = assign2950_e3327_d_n7;
        locals.var_expi_dn8 = assign2950_e3327_d_n8;
        locals.var_expi_dn9 = assign2950_e3327_d_n9;
        locals.var_expi_dn10 = assign2950_e3327_d_n10;
        locals.var_expi_dn11 = assign2950_e3327_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign2960_e3335, assign2960_e3335_d_n4,) = {
    if (locals.var_guard89 != 0.0) {
        let assign2960_e3332: f64 = (p.p59 * locals.var_vtv);
        let assign2960_e3333: f64 = (1.0 / assign2960_e3332);
        (assign2960_e3333, (-((p.p59 * locals.var_vtv_dn4) / (assign2960_e3332 * assign2960_e3332))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign2960_e3335;
        locals.var_afac_dn4 = assign2960_e3335_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3030_e3412: f64 = if p.p88 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign3030_e3412;
        locals.var_guard93_rv = 0.0;

        let (assign3040_e3421, assign3040_e3421_d_n4, assign3040_e3421_d_n8, assign3040_e3421_d_n9,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign3040_e3417: f64 = (-locals.var_vbbe_t);
        let assign3040_e3419: f64 = (assign3040_e3417 - locals.var_vbei);
        (assign3040_e3419, (-locals.var_vbbe_t_dn4), (-locals.var_vbei_dn8), (-locals.var_vbei_dn9),)
    } else {
        (locals.var_bvbe, locals.var_bvbe_dn4, locals.var_bvbe_dn8, locals.var_bvbe_dn9,)
    }
};
        locals.var_bvbe = assign3040_e3421;
        locals.var_bvbe_dn4 = assign3040_e3421_d_n4;
        locals.var_bvbe_dn8 = assign3040_e3421_d_n8;
        locals.var_bvbe_dn9 = assign3040_e3421_d_n9;
        locals.var_bvbe_rv = 0.0;

        let (assign3050_e3431, assign3050_e3431_d_n4,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign3050_e3428: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign3050_e3429: f64 = (1.0 / assign3050_e3428);
        (assign3050_e3429, (-(((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)) / (assign3050_e3428 * assign3050_e3428))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3050_e3431;
        locals.var_afac_dn4 = assign3050_e3431_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3060_e3434: f64 = if locals.var_bvbe < locals.var_maxvibbe { 1.0 } else { 0.0 };
        locals.var_guard94 = assign3060_e3434;
        locals.var_guard94_rv = 0.0;

        let (assign3070_e3445, assign3070_e3445_d_n4, assign3070_e3445_d_n5, assign3070_e3445_d_n6, assign3070_e3445_d_n7, assign3070_e3445_d_n8, assign3070_e3445_d_n9, assign3070_e3445_d_n10, assign3070_e3445_d_n11,) = {
    if (((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign3070_e3442: f64 = (locals.var_bvbe * locals.var_afac);
        let assign3070_e3443: f64 = (assign3070_e3442).exp();
        (assign3070_e3443, (assign3070_e3443 * ((locals.var_bvbe_dn4 * locals.var_afac) + (locals.var_bvbe * locals.var_afac_dn4))), 0.0, 0.0, 0.0, (assign3070_e3443 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3070_e3443 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3070_e3445;
        locals.var_expx_dn4 = assign3070_e3445_d_n4;
        locals.var_expx_dn5 = assign3070_e3445_d_n5;
        locals.var_expx_dn6 = assign3070_e3445_d_n6;
        locals.var_expx_dn7 = assign3070_e3445_d_n7;
        locals.var_expx_dn8 = assign3070_e3445_d_n8;
        locals.var_expx_dn9 = assign3070_e3445_d_n9;
        locals.var_expx_dn10 = assign3070_e3445_d_n10;
        locals.var_expx_dn11 = assign3070_e3445_d_n11;
        locals.var_expx_rv = 0.0;

        let (assign3080_e3465, assign3080_e3465_d_n4, assign3080_e3465_d_n5, assign3080_e3465_d_n6, assign3080_e3465_d_n7, assign3080_e3465_d_n8, assign3080_e3465_d_n9, assign3080_e3465_d_n10, assign3080_e3465_d_n11,) = {
    if (((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 == 0.0)) {
        let assign3080_e3454: f64 = (locals.var_maxvibbe * locals.var_afac);
        let assign3080_e3455: f64 = (assign3080_e3454).exp();
        let assign3080_e3459: f64 = (locals.var_bvbe - locals.var_maxvibbe);
        let assign3080_e3461: f64 = (assign3080_e3459 * locals.var_afac);
        let assign3080_e3462: f64 = (1.0 + assign3080_e3461);
        let assign3080_e3463: f64 = (assign3080_e3455 * assign3080_e3462);
        (assign3080_e3463, (((assign3080_e3455 * ((locals.var_maxvibbe_dn4 * locals.var_afac) + (locals.var_maxvibbe * locals.var_afac_dn4))) * assign3080_e3462) + (assign3080_e3455 * (((locals.var_bvbe_dn4 - locals.var_maxvibbe_dn4) * locals.var_afac) + (assign3080_e3459 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3080_e3455 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3080_e3455 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3080_e3465;
        locals.var_expx_dn4 = assign3080_e3465_d_n4;
        locals.var_expx_dn5 = assign3080_e3465_d_n5;
        locals.var_expx_dn6 = assign3080_e3465_d_n6;
        locals.var_expx_dn7 = assign3080_e3465_d_n7;
        locals.var_expx_dn8 = assign3080_e3465_d_n8;
        locals.var_expx_dn9 = assign3080_e3465_d_n9;
        locals.var_expx_dn10 = assign3080_e3465_d_n10;
        locals.var_expx_dn11 = assign3080_e3465_d_n11;
        locals.var_expx_rv = 0.0;

        let assign3110_e3484: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign3110_e3484;
        locals.var_guard95_rv = 0.0;

        let (assign3130_e3502, assign3130_e3502_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) {
        let assign3130_e3499: f64 = (p.p56 * locals.var_vtv);
        let assign3130_e3500: f64 = (1.0 / assign3130_e3499);
        (assign3130_e3500, (-((p.p56 * locals.var_vtv_dn4) / (assign3130_e3499 * assign3130_e3499))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3130_e3502;
        locals.var_afac_dn4 = assign3130_e3502_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3140_e3505: f64 = if locals.var_vbex < locals.var_maxvibei { 1.0 } else { 0.0 };
        locals.var_guard96 = assign3140_e3505;
        locals.var_guard96_rv = 0.0;

        let (assign3150_e3517, assign3150_e3517_d_n4, assign3150_e3517_d_n5, assign3150_e3517_d_n6, assign3150_e3517_d_n7, assign3150_e3517_d_n8, assign3150_e3517_d_n9, assign3150_e3517_d_n10, assign3150_e3517_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign3150_e3514: f64 = (locals.var_vbex * locals.var_afac);
        let assign3150_e3515: f64 = (assign3150_e3514).exp();
        (assign3150_e3515, (assign3150_e3515 * (locals.var_vbex * locals.var_afac_dn4)), 0.0, 0.0, (assign3150_e3515 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3150_e3515 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3150_e3517;
        locals.var_expi_dn4 = assign3150_e3517_d_n4;
        locals.var_expi_dn5 = assign3150_e3517_d_n5;
        locals.var_expi_dn6 = assign3150_e3517_d_n6;
        locals.var_expi_dn7 = assign3150_e3517_d_n7;
        locals.var_expi_dn8 = assign3150_e3517_d_n8;
        locals.var_expi_dn9 = assign3150_e3517_d_n9;
        locals.var_expi_dn10 = assign3150_e3517_d_n10;
        locals.var_expi_dn11 = assign3150_e3517_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3160_e3538, assign3160_e3538_d_n4, assign3160_e3538_d_n5, assign3160_e3538_d_n6, assign3160_e3538_d_n7, assign3160_e3538_d_n8, assign3160_e3538_d_n9, assign3160_e3538_d_n10, assign3160_e3538_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 == 0.0)) {
        let assign3160_e3527: f64 = (locals.var_maxvibei * locals.var_afac);
        let assign3160_e3528: f64 = (assign3160_e3527).exp();
        let assign3160_e3532: f64 = (locals.var_vbex - locals.var_maxvibei);
        let assign3160_e3534: f64 = (assign3160_e3532 * locals.var_afac);
        let assign3160_e3535: f64 = (1.0 + assign3160_e3534);
        let assign3160_e3536: f64 = (assign3160_e3528 * assign3160_e3535);
        (assign3160_e3536, (((assign3160_e3528 * ((locals.var_maxvibei_dn4 * locals.var_afac) + (locals.var_maxvibei * locals.var_afac_dn4))) * assign3160_e3535) + (assign3160_e3528 * (((-locals.var_maxvibei_dn4) * locals.var_afac) + (assign3160_e3532 * locals.var_afac_dn4)))), 0.0, 0.0, (assign3160_e3528 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3160_e3528 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3160_e3538;
        locals.var_expi_dn4 = assign3160_e3538_d_n4;
        locals.var_expi_dn5 = assign3160_e3538_d_n5;
        locals.var_expi_dn6 = assign3160_e3538_d_n6;
        locals.var_expi_dn7 = assign3160_e3538_d_n7;
        locals.var_expi_dn8 = assign3160_e3538_d_n8;
        locals.var_expi_dn9 = assign3160_e3538_d_n9;
        locals.var_expi_dn10 = assign3160_e3538_d_n10;
        locals.var_expi_dn11 = assign3160_e3538_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3170_e3549, assign3170_e3549_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) {
        let assign3170_e3546: f64 = (p.p59 * locals.var_vtv);
        let assign3170_e3547: f64 = (1.0 / assign3170_e3546);
        (assign3170_e3547, (-((p.p59 * locals.var_vtv_dn4) / (assign3170_e3546 * assign3170_e3546))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3170_e3549;
        locals.var_afac_dn4 = assign3170_e3549_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3220_e3605: f64 = if p.p88 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign3220_e3605;
        locals.var_guard98_rv = 0.0;

        let (assign3230_e3617, assign3230_e3617_d_n4, assign3230_e3617_d_n8, assign3230_e3617_d_n9,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign3230_e3613: f64 = (-locals.var_vbbe_t);
        let assign3230_e3615: f64 = (assign3230_e3613 - locals.var_vbei);
        (assign3230_e3615, (-locals.var_vbbe_t_dn4), (-locals.var_vbei_dn8), (-locals.var_vbei_dn9),)
    } else {
        (locals.var_bvbe, locals.var_bvbe_dn4, locals.var_bvbe_dn8, locals.var_bvbe_dn9,)
    }
};
        locals.var_bvbe = assign3230_e3617;
        locals.var_bvbe_dn4 = assign3230_e3617_d_n4;
        locals.var_bvbe_dn8 = assign3230_e3617_d_n8;
        locals.var_bvbe_dn9 = assign3230_e3617_d_n9;
        locals.var_bvbe_rv = 0.0;

        let (assign3240_e3630, assign3240_e3630_d_n4,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign3240_e3627: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign3240_e3628: f64 = (1.0 / assign3240_e3627);
        (assign3240_e3628, (-(((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)) / (assign3240_e3627 * assign3240_e3627))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3240_e3630;
        locals.var_afac_dn4 = assign3240_e3630_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3250_e3633: f64 = if locals.var_bvbe < locals.var_maxvibbe { 1.0 } else { 0.0 };
        locals.var_guard99 = assign3250_e3633;
        locals.var_guard99_rv = 0.0;

        let (assign3260_e3647, assign3260_e3647_d_n4, assign3260_e3647_d_n5, assign3260_e3647_d_n6, assign3260_e3647_d_n7, assign3260_e3647_d_n8, assign3260_e3647_d_n9, assign3260_e3647_d_n10, assign3260_e3647_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
        let assign3260_e3644: f64 = (locals.var_bvbe * locals.var_afac);
        let assign3260_e3645: f64 = (assign3260_e3644).exp();
        (assign3260_e3645, (assign3260_e3645 * ((locals.var_bvbe_dn4 * locals.var_afac) + (locals.var_bvbe * locals.var_afac_dn4))), 0.0, 0.0, 0.0, (assign3260_e3645 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3260_e3645 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3260_e3647;
        locals.var_expx_dn4 = assign3260_e3647_d_n4;
        locals.var_expx_dn5 = assign3260_e3647_d_n5;
        locals.var_expx_dn6 = assign3260_e3647_d_n6;
        locals.var_expx_dn7 = assign3260_e3647_d_n7;
        locals.var_expx_dn8 = assign3260_e3647_d_n8;
        locals.var_expx_dn9 = assign3260_e3647_d_n9;
        locals.var_expx_dn10 = assign3260_e3647_d_n10;
        locals.var_expx_dn11 = assign3260_e3647_d_n11;
        locals.var_expx_rv = 0.0;

        let (assign3270_e3670, assign3270_e3670_d_n4, assign3270_e3670_d_n5, assign3270_e3670_d_n6, assign3270_e3670_d_n7, assign3270_e3670_d_n8, assign3270_e3670_d_n9, assign3270_e3670_d_n10, assign3270_e3670_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign3270_e3659: f64 = (locals.var_maxvibbe * locals.var_afac);
        let assign3270_e3660: f64 = (assign3270_e3659).exp();
        let assign3270_e3664: f64 = (locals.var_bvbe - locals.var_maxvibbe);
        let assign3270_e3666: f64 = (assign3270_e3664 * locals.var_afac);
        let assign3270_e3667: f64 = (1.0 + assign3270_e3666);
        let assign3270_e3668: f64 = (assign3270_e3660 * assign3270_e3667);
        (assign3270_e3668, (((assign3270_e3660 * ((locals.var_maxvibbe_dn4 * locals.var_afac) + (locals.var_maxvibbe * locals.var_afac_dn4))) * assign3270_e3667) + (assign3270_e3660 * (((locals.var_bvbe_dn4 - locals.var_maxvibbe_dn4) * locals.var_afac) + (assign3270_e3664 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3270_e3660 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3270_e3660 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3270_e3670;
        locals.var_expx_dn4 = assign3270_e3670_d_n4;
        locals.var_expx_dn5 = assign3270_e3670_d_n5;
        locals.var_expx_dn6 = assign3270_e3670_d_n6;
        locals.var_expx_dn7 = assign3270_e3670_d_n7;
        locals.var_expx_dn8 = assign3270_e3670_d_n8;
        locals.var_expx_dn9 = assign3270_e3670_d_n9;
        locals.var_expx_dn10 = assign3270_e3670_d_n10;
        locals.var_expx_dn11 = assign3270_e3670_d_n11;
        locals.var_expx_rv = 0.0;

        let (assign3290_e3697, assign3290_e3697_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3290_e3694: f64 = (p.p56 * locals.var_vtv);
        let assign3290_e3695: f64 = (1.0 / assign3290_e3694);
        (assign3290_e3695, (-((p.p56 * locals.var_vtv_dn4) / (assign3290_e3694 * assign3290_e3694))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3290_e3697;
        locals.var_afac_dn4 = assign3290_e3697_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3300_e3700: f64 = if locals.var_vbei < locals.var_maxvibei { 1.0 } else { 0.0 };
        locals.var_guard100 = assign3300_e3700;
        locals.var_guard100_rv = 0.0;

        let (assign3310_e3713, assign3310_e3713_d_n4, assign3310_e3713_d_n5, assign3310_e3713_d_n6, assign3310_e3713_d_n7, assign3310_e3713_d_n8, assign3310_e3713_d_n9, assign3310_e3713_d_n10, assign3310_e3713_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard100 != 0.0)) {
        let assign3310_e3710: f64 = (locals.var_vbei * locals.var_afac);
        let assign3310_e3711: f64 = (assign3310_e3710).exp();
        (assign3310_e3711, (assign3310_e3711 * (locals.var_vbei * locals.var_afac_dn4)), 0.0, 0.0, 0.0, (assign3310_e3711 * (locals.var_vbei_dn8 * locals.var_afac)), (assign3310_e3711 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3310_e3713;
        locals.var_expi_dn4 = assign3310_e3713_d_n4;
        locals.var_expi_dn5 = assign3310_e3713_d_n5;
        locals.var_expi_dn6 = assign3310_e3713_d_n6;
        locals.var_expi_dn7 = assign3310_e3713_d_n7;
        locals.var_expi_dn8 = assign3310_e3713_d_n8;
        locals.var_expi_dn9 = assign3310_e3713_d_n9;
        locals.var_expi_dn10 = assign3310_e3713_d_n10;
        locals.var_expi_dn11 = assign3310_e3713_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3320_e3735, assign3320_e3735_d_n4, assign3320_e3735_d_n5, assign3320_e3735_d_n6, assign3320_e3735_d_n7, assign3320_e3735_d_n8, assign3320_e3735_d_n9, assign3320_e3735_d_n10, assign3320_e3735_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign3320_e3724: f64 = (locals.var_maxvibei * locals.var_afac);
        let assign3320_e3725: f64 = (assign3320_e3724).exp();
        let assign3320_e3729: f64 = (locals.var_vbei - locals.var_maxvibei);
        let assign3320_e3731: f64 = (assign3320_e3729 * locals.var_afac);
        let assign3320_e3732: f64 = (1.0 + assign3320_e3731);
        let assign3320_e3733: f64 = (assign3320_e3725 * assign3320_e3732);
        (assign3320_e3733, (((assign3320_e3725 * ((locals.var_maxvibei_dn4 * locals.var_afac) + (locals.var_maxvibei * locals.var_afac_dn4))) * assign3320_e3732) + (assign3320_e3725 * (((-locals.var_maxvibei_dn4) * locals.var_afac) + (assign3320_e3729 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3320_e3725 * (locals.var_vbei_dn8 * locals.var_afac)), (assign3320_e3725 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3320_e3735;
        locals.var_expi_dn4 = assign3320_e3735_d_n4;
        locals.var_expi_dn5 = assign3320_e3735_d_n5;
        locals.var_expi_dn6 = assign3320_e3735_d_n6;
        locals.var_expi_dn7 = assign3320_e3735_d_n7;
        locals.var_expi_dn8 = assign3320_e3735_d_n8;
        locals.var_expi_dn9 = assign3320_e3735_d_n9;
        locals.var_expi_dn10 = assign3320_e3735_d_n10;
        locals.var_expi_dn11 = assign3320_e3735_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3330_e3747, assign3330_e3747_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3330_e3744: f64 = (p.p59 * locals.var_vtv);
        let assign3330_e3745: f64 = (1.0 / assign3330_e3744);
        (assign3330_e3745, (-((p.p59 * locals.var_vtv_dn4) / (assign3330_e3744 * assign3330_e3744))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3330_e3747;
        locals.var_afac_dn4 = assign3330_e3747_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3400_e3844: f64 = if p.p88 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign3400_e3844;
        locals.var_guard103_rv = 0.0;

        let (assign3410_e3857, assign3410_e3857_d_n4, assign3410_e3857_d_n8, assign3410_e3857_d_n9,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign3410_e3853: f64 = (-locals.var_vbbe_t);
        let assign3410_e3855: f64 = (assign3410_e3853 - locals.var_vbei);
        (assign3410_e3855, (-locals.var_vbbe_t_dn4), (-locals.var_vbei_dn8), (-locals.var_vbei_dn9),)
    } else {
        (locals.var_bvbe, locals.var_bvbe_dn4, locals.var_bvbe_dn8, locals.var_bvbe_dn9,)
    }
};
        locals.var_bvbe = assign3410_e3857;
        locals.var_bvbe_dn4 = assign3410_e3857_d_n4;
        locals.var_bvbe_dn8 = assign3410_e3857_d_n8;
        locals.var_bvbe_dn9 = assign3410_e3857_d_n9;
        locals.var_bvbe_rv = 0.0;

        let (assign3420_e3871, assign3420_e3871_d_n4,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign3420_e3868: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign3420_e3869: f64 = (1.0 / assign3420_e3868);
        (assign3420_e3869, (-(((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)) / (assign3420_e3868 * assign3420_e3868))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3420_e3871;
        locals.var_afac_dn4 = assign3420_e3871_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3430_e3874: f64 = if locals.var_bvbe < locals.var_maxvibbe { 1.0 } else { 0.0 };
        locals.var_guard104 = assign3430_e3874;
        locals.var_guard104_rv = 0.0;

        let (assign3440_e3889, assign3440_e3889_d_n4, assign3440_e3889_d_n5, assign3440_e3889_d_n6, assign3440_e3889_d_n7, assign3440_e3889_d_n8, assign3440_e3889_d_n9, assign3440_e3889_d_n10, assign3440_e3889_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 != 0.0)) {
        let assign3440_e3886: f64 = (locals.var_bvbe * locals.var_afac);
        let assign3440_e3887: f64 = (assign3440_e3886).exp();
        (assign3440_e3887, (assign3440_e3887 * ((locals.var_bvbe_dn4 * locals.var_afac) + (locals.var_bvbe * locals.var_afac_dn4))), 0.0, 0.0, 0.0, (assign3440_e3887 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3440_e3887 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3440_e3889;
        locals.var_expx_dn4 = assign3440_e3889_d_n4;
        locals.var_expx_dn5 = assign3440_e3889_d_n5;
        locals.var_expx_dn6 = assign3440_e3889_d_n6;
        locals.var_expx_dn7 = assign3440_e3889_d_n7;
        locals.var_expx_dn8 = assign3440_e3889_d_n8;
        locals.var_expx_dn9 = assign3440_e3889_d_n9;
        locals.var_expx_dn10 = assign3440_e3889_d_n10;
        locals.var_expx_dn11 = assign3440_e3889_d_n11;
        locals.var_expx_rv = 0.0;

        let (assign3450_e3913, assign3450_e3913_d_n4, assign3450_e3913_d_n5, assign3450_e3913_d_n6, assign3450_e3913_d_n7, assign3450_e3913_d_n8, assign3450_e3913_d_n9, assign3450_e3913_d_n10, assign3450_e3913_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign3450_e3902: f64 = (locals.var_maxvibbe * locals.var_afac);
        let assign3450_e3903: f64 = (assign3450_e3902).exp();
        let assign3450_e3907: f64 = (locals.var_bvbe - locals.var_maxvibbe);
        let assign3450_e3909: f64 = (assign3450_e3907 * locals.var_afac);
        let assign3450_e3910: f64 = (1.0 + assign3450_e3909);
        let assign3450_e3911: f64 = (assign3450_e3903 * assign3450_e3910);
        (assign3450_e3911, (((assign3450_e3903 * ((locals.var_maxvibbe_dn4 * locals.var_afac) + (locals.var_maxvibbe * locals.var_afac_dn4))) * assign3450_e3910) + (assign3450_e3903 * (((locals.var_bvbe_dn4 - locals.var_maxvibbe_dn4) * locals.var_afac) + (assign3450_e3907 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3450_e3903 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3450_e3903 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3450_e3913;
        locals.var_expx_dn4 = assign3450_e3913_d_n4;
        locals.var_expx_dn5 = assign3450_e3913_d_n5;
        locals.var_expx_dn6 = assign3450_e3913_d_n6;
        locals.var_expx_dn7 = assign3450_e3913_d_n7;
        locals.var_expx_dn8 = assign3450_e3913_d_n8;
        locals.var_expx_dn9 = assign3450_e3913_d_n9;
        locals.var_expx_dn10 = assign3450_e3913_d_n10;
        locals.var_expx_dn11 = assign3450_e3913_d_n11;
        locals.var_expx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3470_e3943, assign3470_e3943_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3470_e3940: f64 = (p.p56 * locals.var_vtv);
        let assign3470_e3941: f64 = (1.0 / assign3470_e3940);
        (assign3470_e3941, (-((p.p56 * locals.var_vtv_dn4) / (assign3470_e3940 * assign3470_e3940))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3470_e3943;
        locals.var_afac_dn4 = assign3470_e3943_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3480_e3946: f64 = if locals.var_vbex < locals.var_maxvibei { 1.0 } else { 0.0 };
        locals.var_guard105 = assign3480_e3946;
        locals.var_guard105_rv = 0.0;

        let (assign3490_e3959, assign3490_e3959_d_n4, assign3490_e3959_d_n5, assign3490_e3959_d_n6, assign3490_e3959_d_n7, assign3490_e3959_d_n8, assign3490_e3959_d_n9, assign3490_e3959_d_n10, assign3490_e3959_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard105 != 0.0)) {
        let assign3490_e3956: f64 = (locals.var_vbex * locals.var_afac);
        let assign3490_e3957: f64 = (assign3490_e3956).exp();
        (assign3490_e3957, (assign3490_e3957 * (locals.var_vbex * locals.var_afac_dn4)), 0.0, 0.0, (assign3490_e3957 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3490_e3957 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3490_e3959;
        locals.var_expi_dn4 = assign3490_e3959_d_n4;
        locals.var_expi_dn5 = assign3490_e3959_d_n5;
        locals.var_expi_dn6 = assign3490_e3959_d_n6;
        locals.var_expi_dn7 = assign3490_e3959_d_n7;
        locals.var_expi_dn8 = assign3490_e3959_d_n8;
        locals.var_expi_dn9 = assign3490_e3959_d_n9;
        locals.var_expi_dn10 = assign3490_e3959_d_n10;
        locals.var_expi_dn11 = assign3490_e3959_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3500_e3981, assign3500_e3981_d_n4, assign3500_e3981_d_n5, assign3500_e3981_d_n6, assign3500_e3981_d_n7, assign3500_e3981_d_n8, assign3500_e3981_d_n9, assign3500_e3981_d_n10, assign3500_e3981_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard105 == 0.0)) {
        let assign3500_e3970: f64 = (locals.var_maxvibei * locals.var_afac);
        let assign3500_e3971: f64 = (assign3500_e3970).exp();
        let assign3500_e3975: f64 = (locals.var_vbex - locals.var_maxvibei);
        let assign3500_e3977: f64 = (assign3500_e3975 * locals.var_afac);
        let assign3500_e3978: f64 = (1.0 + assign3500_e3977);
        let assign3500_e3979: f64 = (assign3500_e3971 * assign3500_e3978);
        (assign3500_e3979, (((assign3500_e3971 * ((locals.var_maxvibei_dn4 * locals.var_afac) + (locals.var_maxvibei * locals.var_afac_dn4))) * assign3500_e3978) + (assign3500_e3971 * (((-locals.var_maxvibei_dn4) * locals.var_afac) + (assign3500_e3975 * locals.var_afac_dn4)))), 0.0, 0.0, (assign3500_e3971 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3500_e3971 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3500_e3981;
        locals.var_expi_dn4 = assign3500_e3981_d_n4;
        locals.var_expi_dn5 = assign3500_e3981_d_n5;
        locals.var_expi_dn6 = assign3500_e3981_d_n6;
        locals.var_expi_dn7 = assign3500_e3981_d_n7;
        locals.var_expi_dn8 = assign3500_e3981_d_n8;
        locals.var_expi_dn9 = assign3500_e3981_d_n9;
        locals.var_expi_dn10 = assign3500_e3981_d_n10;
        locals.var_expi_dn11 = assign3500_e3981_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3510_e3993, assign3510_e3993_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3510_e3990: f64 = (p.p59 * locals.var_vtv);
        let assign3510_e3991: f64 = (1.0 / assign3510_e3990);
        (assign3510_e3991, (-((p.p59 * locals.var_vtv_dn4) / (assign3510_e3990 * assign3510_e3990))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3510_e3993;
        locals.var_afac_dn4 = assign3510_e3993_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3560_e4056: f64 = if p.p88 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign3560_e4056;
        locals.var_guard107_rv = 0.0;

        let (assign3570_e4069, assign3570_e4069_d_n4, assign3570_e4069_d_n8, assign3570_e4069_d_n9,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) {
        let assign3570_e4065: f64 = (-locals.var_vbbe_t);
        let assign3570_e4067: f64 = (assign3570_e4065 - locals.var_vbei);
        (assign3570_e4067, (-locals.var_vbbe_t_dn4), (-locals.var_vbei_dn8), (-locals.var_vbei_dn9),)
    } else {
        (locals.var_bvbe, locals.var_bvbe_dn4, locals.var_bvbe_dn8, locals.var_bvbe_dn9,)
    }
};
        locals.var_bvbe = assign3570_e4069;
        locals.var_bvbe_dn4 = assign3570_e4069_d_n4;
        locals.var_bvbe_dn8 = assign3570_e4069_d_n8;
        locals.var_bvbe_dn9 = assign3570_e4069_d_n9;
        locals.var_bvbe_rv = 0.0;

        let (assign3580_e4083, assign3580_e4083_d_n4,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) {
        let assign3580_e4080: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign3580_e4081: f64 = (1.0 / assign3580_e4080);
        (assign3580_e4081, (-(((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)) / (assign3580_e4080 * assign3580_e4080))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3580_e4083;
        locals.var_afac_dn4 = assign3580_e4083_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3590_e4086: f64 = if locals.var_bvbe < locals.var_maxvibbe { 1.0 } else { 0.0 };
        locals.var_guard108 = assign3590_e4086;
        locals.var_guard108_rv = 0.0;

        let (assign3600_e4101, assign3600_e4101_d_n4, assign3600_e4101_d_n5, assign3600_e4101_d_n6, assign3600_e4101_d_n7, assign3600_e4101_d_n8, assign3600_e4101_d_n9, assign3600_e4101_d_n10, assign3600_e4101_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) {
        let assign3600_e4098: f64 = (locals.var_bvbe * locals.var_afac);
        let assign3600_e4099: f64 = (assign3600_e4098).exp();
        (assign3600_e4099, (assign3600_e4099 * ((locals.var_bvbe_dn4 * locals.var_afac) + (locals.var_bvbe * locals.var_afac_dn4))), 0.0, 0.0, 0.0, (assign3600_e4099 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3600_e4099 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3600_e4101;
        locals.var_expx_dn4 = assign3600_e4101_d_n4;
        locals.var_expx_dn5 = assign3600_e4101_d_n5;
        locals.var_expx_dn6 = assign3600_e4101_d_n6;
        locals.var_expx_dn7 = assign3600_e4101_d_n7;
        locals.var_expx_dn8 = assign3600_e4101_d_n8;
        locals.var_expx_dn9 = assign3600_e4101_d_n9;
        locals.var_expx_dn10 = assign3600_e4101_d_n10;
        locals.var_expx_dn11 = assign3600_e4101_d_n11;
        locals.var_expx_rv = 0.0;

        let (assign3610_e4125, assign3610_e4125_d_n4, assign3610_e4125_d_n5, assign3610_e4125_d_n6, assign3610_e4125_d_n7, assign3610_e4125_d_n8, assign3610_e4125_d_n9, assign3610_e4125_d_n10, assign3610_e4125_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 == 0.0)) {
        let assign3610_e4114: f64 = (locals.var_maxvibbe * locals.var_afac);
        let assign3610_e4115: f64 = (assign3610_e4114).exp();
        let assign3610_e4119: f64 = (locals.var_bvbe - locals.var_maxvibbe);
        let assign3610_e4121: f64 = (assign3610_e4119 * locals.var_afac);
        let assign3610_e4122: f64 = (1.0 + assign3610_e4121);
        let assign3610_e4123: f64 = (assign3610_e4115 * assign3610_e4122);
        (assign3610_e4123, (((assign3610_e4115 * ((locals.var_maxvibbe_dn4 * locals.var_afac) + (locals.var_maxvibbe * locals.var_afac_dn4))) * assign3610_e4122) + (assign3610_e4115 * (((locals.var_bvbe_dn4 - locals.var_maxvibbe_dn4) * locals.var_afac) + (assign3610_e4119 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3610_e4115 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3610_e4115 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3610_e4125;
        locals.var_expx_dn4 = assign3610_e4125_d_n4;
        locals.var_expx_dn5 = assign3610_e4125_d_n5;
        locals.var_expx_dn6 = assign3610_e4125_d_n6;
        locals.var_expx_dn7 = assign3610_e4125_d_n7;
        locals.var_expx_dn8 = assign3610_e4125_d_n8;
        locals.var_expx_dn9 = assign3610_e4125_d_n9;
        locals.var_expx_dn10 = assign3610_e4125_d_n10;
        locals.var_expx_dn11 = assign3610_e4125_d_n11;
        locals.var_expx_rv = 0.0;

        let assign3630_e4149: f64 = (p.p61 * locals.var_vtv);
        let assign3630_e4150: f64 = (1.0 / assign3630_e4149);
        locals.var_afac = assign3630_e4150;
        locals.var_afac_dn4 = (-((p.p61 * locals.var_vtv_dn4) / (assign3630_e4149 * assign3630_e4149)));
        locals.var_afac_rv = 0.0;

        let assign3640_e4153: f64 = if locals.var_vbci < locals.var_maxvibci { 1.0 } else { 0.0 };
        locals.var_guard109 = assign3640_e4153;
        locals.var_guard109_rv = 0.0;

        let (assign3650_e4160, assign3650_e4160_d_n4, assign3650_e4160_d_n5, assign3650_e4160_d_n6, assign3650_e4160_d_n7, assign3650_e4160_d_n8, assign3650_e4160_d_n9, assign3650_e4160_d_n10, assign3650_e4160_d_n11,) = {
    if (locals.var_guard109 != 0.0) {
        let assign3650_e4157: f64 = (locals.var_vbci * locals.var_afac);
        let assign3650_e4158: f64 = (assign3650_e4157).exp();
        (assign3650_e4158, (assign3650_e4158 * (locals.var_vbci * locals.var_afac_dn4)), 0.0, (assign3650_e4158 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign3650_e4158 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3650_e4160;
        locals.var_expi_dn4 = assign3650_e4160_d_n4;
        locals.var_expi_dn5 = assign3650_e4160_d_n5;
        locals.var_expi_dn6 = assign3650_e4160_d_n6;
        locals.var_expi_dn7 = assign3650_e4160_d_n7;
        locals.var_expi_dn8 = assign3650_e4160_d_n8;
        locals.var_expi_dn9 = assign3650_e4160_d_n9;
        locals.var_expi_dn10 = assign3650_e4160_d_n10;
        locals.var_expi_dn11 = assign3650_e4160_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3660_e4176, assign3660_e4176_d_n4, assign3660_e4176_d_n5, assign3660_e4176_d_n6, assign3660_e4176_d_n7, assign3660_e4176_d_n8, assign3660_e4176_d_n9, assign3660_e4176_d_n10, assign3660_e4176_d_n11,) = {
    if (locals.var_guard109 == 0.0) {
        let assign3660_e4165: f64 = (locals.var_maxvibci * locals.var_afac);
        let assign3660_e4166: f64 = (assign3660_e4165).exp();
        let assign3660_e4170: f64 = (locals.var_vbci - locals.var_maxvibci);
        let assign3660_e4172: f64 = (assign3660_e4170 * locals.var_afac);
        let assign3660_e4173: f64 = (1.0 + assign3660_e4172);
        let assign3660_e4174: f64 = (assign3660_e4166 * assign3660_e4173);
        (assign3660_e4174, (((assign3660_e4166 * ((locals.var_maxvibci_dn4 * locals.var_afac) + (locals.var_maxvibci * locals.var_afac_dn4))) * assign3660_e4173) + (assign3660_e4166 * (((-locals.var_maxvibci_dn4) * locals.var_afac) + (assign3660_e4170 * locals.var_afac_dn4)))), 0.0, (assign3660_e4166 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign3660_e4166 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3660_e4176;
        locals.var_expi_dn4 = assign3660_e4176_d_n4;
        locals.var_expi_dn5 = assign3660_e4176_d_n5;
        locals.var_expi_dn6 = assign3660_e4176_d_n6;
        locals.var_expi_dn7 = assign3660_e4176_d_n7;
        locals.var_expi_dn8 = assign3660_e4176_d_n8;
        locals.var_expi_dn9 = assign3660_e4176_d_n9;
        locals.var_expi_dn10 = assign3660_e4176_d_n10;
        locals.var_expi_dn11 = assign3660_e4176_d_n11;
        locals.var_expi_rv = 0.0;

        let assign3670_e4180: f64 = (p.p63 * locals.var_vtv);
        let assign3670_e4181: f64 = (1.0 / assign3670_e4180);
        locals.var_afac = assign3670_e4181;
        locals.var_afac_dn4 = (-((p.p63 * locals.var_vtv_dn4) / (assign3670_e4180 * assign3670_e4180)));
        locals.var_afac_rv = 0.0;

        let assign3720_e4225: f64 = if ((p.p64 > 0.0) || (p.p65 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard111 = assign3720_e4225;
        locals.var_guard111_rv = 0.0;

        let (assign3730_e4233, assign3730_e4233_d_n4,) = {
    if (locals.var_guard111 != 0.0) {
        let assign3730_e4230: f64 = (p.p61 * locals.var_vtv);
        let assign3730_e4231: f64 = (1.0 / assign3730_e4230);
        (assign3730_e4231, (-((p.p61 * locals.var_vtv_dn4) / (assign3730_e4230 * assign3730_e4230))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3730_e4233;
        locals.var_afac_dn4 = assign3730_e4233_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3740_e4236: f64 = if locals.var_vbep < locals.var_maxvibeip { 1.0 } else { 0.0 };
        locals.var_guard112 = assign3740_e4236;
        locals.var_guard112_rv = 0.0;

        let (assign3750_e4245, assign3750_e4245_d_n4, assign3750_e4245_d_n5, assign3750_e4245_d_n6, assign3750_e4245_d_n7, assign3750_e4245_d_n8, assign3750_e4245_d_n9, assign3750_e4245_d_n10, assign3750_e4245_d_n11,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard112 != 0.0)) {
        let assign3750_e4242: f64 = (locals.var_vbep * locals.var_afac);
        let assign3750_e4243: f64 = (assign3750_e4242).exp();
        (assign3750_e4243, (assign3750_e4243 * (locals.var_vbep * locals.var_afac_dn4)), 0.0, 0.0, (assign3750_e4243 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign3750_e4243 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3750_e4245;
        locals.var_expi_dn4 = assign3750_e4245_d_n4;
        locals.var_expi_dn5 = assign3750_e4245_d_n5;
        locals.var_expi_dn6 = assign3750_e4245_d_n6;
        locals.var_expi_dn7 = assign3750_e4245_d_n7;
        locals.var_expi_dn8 = assign3750_e4245_d_n8;
        locals.var_expi_dn9 = assign3750_e4245_d_n9;
        locals.var_expi_dn10 = assign3750_e4245_d_n10;
        locals.var_expi_dn11 = assign3750_e4245_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3760_e4263, assign3760_e4263_d_n4, assign3760_e4263_d_n5, assign3760_e4263_d_n6, assign3760_e4263_d_n7, assign3760_e4263_d_n8, assign3760_e4263_d_n9, assign3760_e4263_d_n10, assign3760_e4263_d_n11,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard112 == 0.0)) {
        let assign3760_e4252: f64 = (locals.var_maxvibeip * locals.var_afac);
        let assign3760_e4253: f64 = (assign3760_e4252).exp();
        let assign3760_e4257: f64 = (locals.var_vbep - locals.var_maxvibeip);
        let assign3760_e4259: f64 = (assign3760_e4257 * locals.var_afac);
        let assign3760_e4260: f64 = (1.0 + assign3760_e4259);
        let assign3760_e4261: f64 = (assign3760_e4253 * assign3760_e4260);
        (assign3760_e4261, (((assign3760_e4253 * ((locals.var_maxvibeip_dn4 * locals.var_afac) + (locals.var_maxvibeip * locals.var_afac_dn4))) * assign3760_e4260) + (assign3760_e4253 * (((-locals.var_maxvibeip_dn4) * locals.var_afac) + (assign3760_e4257 * locals.var_afac_dn4)))), 0.0, 0.0, (assign3760_e4253 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign3760_e4253 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3760_e4263;
        locals.var_expi_dn4 = assign3760_e4263_d_n4;
        locals.var_expi_dn5 = assign3760_e4263_d_n5;
        locals.var_expi_dn6 = assign3760_e4263_d_n6;
        locals.var_expi_dn7 = assign3760_e4263_d_n7;
        locals.var_expi_dn8 = assign3760_e4263_d_n8;
        locals.var_expi_dn9 = assign3760_e4263_d_n9;
        locals.var_expi_dn10 = assign3760_e4263_d_n10;
        locals.var_expi_dn11 = assign3760_e4263_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3770_e4271, assign3770_e4271_d_n4,) = {
    if (locals.var_guard111 != 0.0) {
        let assign3770_e4268: f64 = (p.p63 * locals.var_vtv);
        let assign3770_e4269: f64 = (1.0 / assign3770_e4268);
        (assign3770_e4269, (-((p.p63 * locals.var_vtv_dn4) / (assign3770_e4268 * assign3770_e4268))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3770_e4271;
        locals.var_afac_dn4 = assign3770_e4271_d_n4;
        locals.var_afac_rv = 0.0;

        let assign3830_e4323: f64 = (locals.var_vbci / locals.var_vtv);
        locals.var_arg = assign3830_e4323;
        locals.var_arg_dn4 = (-((locals.var_vbci * locals.var_vtv_dn4) / (locals.var_vtv * locals.var_vtv)));
        locals.var_arg_dn5 = 0.0;
        locals.var_arg_dn6 = (locals.var_vbci_dn6 / locals.var_vtv);
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = (locals.var_vbci_dn8 / locals.var_vtv);
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;
        locals.var_arg_rv = 0.0;

        let assign3840_e4326: f64 = if locals.var_arg < locals.var_vmaxexp { 1.0 } else { 0.0 };
        locals.var_guard114 = assign3840_e4326;
        locals.var_guard114_rv = 0.0;

        let (assign3850_e4331, assign3850_e4331_d_n4, assign3850_e4331_d_n5, assign3850_e4331_d_n6, assign3850_e4331_d_n7, assign3850_e4331_d_n8, assign3850_e4331_d_n9, assign3850_e4331_d_n10, assign3850_e4331_d_n11,) = {
    if (locals.var_guard114 != 0.0) {
        let assign3850_e4329: f64 = (locals.var_arg).exp();
        (assign3850_e4329, (assign3850_e4329 * locals.var_arg_dn4), (assign3850_e4329 * locals.var_arg_dn5), (assign3850_e4329 * locals.var_arg_dn6), (assign3850_e4329 * locals.var_arg_dn7), (assign3850_e4329 * locals.var_arg_dn8), (assign3850_e4329 * locals.var_arg_dn9), (assign3850_e4329 * locals.var_arg_dn10), (assign3850_e4329 * locals.var_arg_dn11),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3850_e4331;
        locals.var_expi_dn4 = assign3850_e4331_d_n4;
        locals.var_expi_dn5 = assign3850_e4331_d_n5;
        locals.var_expi_dn6 = assign3850_e4331_d_n6;
        locals.var_expi_dn7 = assign3850_e4331_d_n7;
        locals.var_expi_dn8 = assign3850_e4331_d_n8;
        locals.var_expi_dn9 = assign3850_e4331_d_n9;
        locals.var_expi_dn10 = assign3850_e4331_d_n10;
        locals.var_expi_dn11 = assign3850_e4331_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign3860_e4343, assign3860_e4343_d_n4, assign3860_e4343_d_n5, assign3860_e4343_d_n6, assign3860_e4343_d_n7, assign3860_e4343_d_n8, assign3860_e4343_d_n9, assign3860_e4343_d_n10, assign3860_e4343_d_n11,) = {
    if (locals.var_guard114 == 0.0) {
        let assign3860_e4335: f64 = (locals.var_vmaxexp).exp();
        let assign3860_e4339: f64 = (locals.var_arg - locals.var_vmaxexp);
        let assign3860_e4340: f64 = (1.0 + assign3860_e4339);
        let assign3860_e4341: f64 = (assign3860_e4335 * assign3860_e4340);
        (assign3860_e4341, (assign3860_e4335 * locals.var_arg_dn4), (assign3860_e4335 * locals.var_arg_dn5), (assign3860_e4335 * locals.var_arg_dn6), (assign3860_e4335 * locals.var_arg_dn7), (assign3860_e4335 * locals.var_arg_dn8), (assign3860_e4335 * locals.var_arg_dn9), (assign3860_e4335 * locals.var_arg_dn10), (assign3860_e4335 * locals.var_arg_dn11),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3860_e4343;
        locals.var_expi_dn4 = assign3860_e4343_d_n4;
        locals.var_expi_dn5 = assign3860_e4343_d_n5;
        locals.var_expi_dn6 = assign3860_e4343_d_n6;
        locals.var_expi_dn7 = assign3860_e4343_d_n7;
        locals.var_expi_dn8 = assign3860_e4343_d_n8;
        locals.var_expi_dn9 = assign3860_e4343_d_n9;
        locals.var_expi_dn10 = assign3860_e4343_d_n10;
        locals.var_expi_dn11 = assign3860_e4343_d_n11;
        locals.var_expi_rv = 0.0;

        let assign3870_e4346: f64 = (locals.var_vbcx / locals.var_vtv);
        locals.var_arg = assign3870_e4346;
        locals.var_arg_dn4 = (-((locals.var_vbcx * locals.var_vtv_dn4) / (locals.var_vtv * locals.var_vtv)));
        locals.var_arg_dn5 = (locals.var_vbcx_dn5 / locals.var_vtv);
        locals.var_arg_dn6 = 0.0;
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = (locals.var_vbcx_dn8 / locals.var_vtv);
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;
        locals.var_arg_rv = 0.0;

        let assign3880_e4349: f64 = if locals.var_arg < locals.var_vmaxexp { 1.0 } else { 0.0 };
        locals.var_guard115 = assign3880_e4349;
        locals.var_guard115_rv = 0.0;

        let (assign3890_e4354, assign3890_e4354_d_n4, assign3890_e4354_d_n5, assign3890_e4354_d_n6, assign3890_e4354_d_n7, assign3890_e4354_d_n8, assign3890_e4354_d_n9, assign3890_e4354_d_n10, assign3890_e4354_d_n11,) = {
    if (locals.var_guard115 != 0.0) {
        let assign3890_e4352: f64 = (locals.var_arg).exp();
        (assign3890_e4352, (assign3890_e4352 * locals.var_arg_dn4), (assign3890_e4352 * locals.var_arg_dn5), (assign3890_e4352 * locals.var_arg_dn6), (assign3890_e4352 * locals.var_arg_dn7), (assign3890_e4352 * locals.var_arg_dn8), (assign3890_e4352 * locals.var_arg_dn9), (assign3890_e4352 * locals.var_arg_dn10), (assign3890_e4352 * locals.var_arg_dn11),)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3890_e4354;
        locals.var_expx_dn4 = assign3890_e4354_d_n4;
        locals.var_expx_dn5 = assign3890_e4354_d_n5;
        locals.var_expx_dn6 = assign3890_e4354_d_n6;
        locals.var_expx_dn7 = assign3890_e4354_d_n7;
        locals.var_expx_dn8 = assign3890_e4354_d_n8;
        locals.var_expx_dn9 = assign3890_e4354_d_n9;
        locals.var_expx_dn10 = assign3890_e4354_d_n10;
        locals.var_expx_dn11 = assign3890_e4354_d_n11;
        locals.var_expx_rv = 0.0;

        let (assign3900_e4366, assign3900_e4366_d_n4, assign3900_e4366_d_n5, assign3900_e4366_d_n6, assign3900_e4366_d_n7, assign3900_e4366_d_n8, assign3900_e4366_d_n9, assign3900_e4366_d_n10, assign3900_e4366_d_n11,) = {
    if (locals.var_guard115 == 0.0) {
        let assign3900_e4358: f64 = (locals.var_vmaxexp).exp();
        let assign3900_e4362: f64 = (locals.var_arg - locals.var_vmaxexp);
        let assign3900_e4363: f64 = (1.0 + assign3900_e4362);
        let assign3900_e4364: f64 = (assign3900_e4358 * assign3900_e4363);
        (assign3900_e4364, (assign3900_e4358 * locals.var_arg_dn4), (assign3900_e4358 * locals.var_arg_dn5), (assign3900_e4358 * locals.var_arg_dn6), (assign3900_e4358 * locals.var_arg_dn7), (assign3900_e4358 * locals.var_arg_dn8), (assign3900_e4358 * locals.var_arg_dn9), (assign3900_e4358 * locals.var_arg_dn10), (assign3900_e4358 * locals.var_arg_dn11),)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3900_e4366;
        locals.var_expx_dn4 = assign3900_e4366_d_n4;
        locals.var_expx_dn5 = assign3900_e4366_d_n5;
        locals.var_expx_dn6 = assign3900_e4366_d_n6;
        locals.var_expx_dn7 = assign3900_e4366_d_n7;
        locals.var_expx_dn8 = assign3900_e4366_d_n8;
        locals.var_expx_dn9 = assign3900_e4366_d_n9;
        locals.var_expx_dn10 = assign3900_e4366_d_n10;
        locals.var_expx_dn11 = assign3900_e4366_d_n11;
        locals.var_expx_rv = 0.0;

        let assign3910_e4370: f64 = (locals.var_gamm_t * locals.var_expi);
        let assign3910_e4371: f64 = (1.0 + assign3910_e4370);
        let assign3910_e4372: f64 = (assign3910_e4371).sqrt();
        locals.var_kbci = assign3910_e4372;
        locals.var_kbci_dn4 = (((locals.var_gamm_t_dn4 * locals.var_expi) + (locals.var_gamm_t * locals.var_expi_dn4)) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn5 = ((locals.var_gamm_t * locals.var_expi_dn5) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn6 = ((locals.var_gamm_t * locals.var_expi_dn6) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn7 = ((locals.var_gamm_t * locals.var_expi_dn7) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn8 = ((locals.var_gamm_t * locals.var_expi_dn8) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn9 = ((locals.var_gamm_t * locals.var_expi_dn9) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn10 = ((locals.var_gamm_t * locals.var_expi_dn10) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn11 = ((locals.var_gamm_t * locals.var_expi_dn11) / (2.0 * assign3910_e4372));
        locals.var_kbci_rv = 0.0;

        let assign3920_e4376: f64 = (locals.var_gamm_t * locals.var_expx);
        let assign3920_e4377: f64 = (1.0 + assign3920_e4376);
        let assign3920_e4378: f64 = (assign3920_e4377).sqrt();
        locals.var_kbcx = assign3920_e4378;
        locals.var_kbcx_dn4 = (((locals.var_gamm_t_dn4 * locals.var_expx) + (locals.var_gamm_t * locals.var_expx_dn4)) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn5 = ((locals.var_gamm_t * locals.var_expx_dn5) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn6 = ((locals.var_gamm_t * locals.var_expx_dn6) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn7 = ((locals.var_gamm_t * locals.var_expx_dn7) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn8 = ((locals.var_gamm_t * locals.var_expx_dn8) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn9 = ((locals.var_gamm_t * locals.var_expx_dn9) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn10 = ((locals.var_gamm_t * locals.var_expx_dn10) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn11 = ((locals.var_gamm_t * locals.var_expx_dn11) / (2.0 * assign3920_e4378));
        locals.var_kbcx_rv = 0.0;

        let assign4340_e4745: f64 = if ((p.p66 > 0.0) || (p.p68 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign4340_e4745;
        locals.var_guard132_rv = 0.0;

        let (assign4350_e4753, assign4350_e4753_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign4350_e4750: f64 = (p.p67 * locals.var_vtv);
        let assign4350_e4751: f64 = (1.0 / assign4350_e4750);
        (assign4350_e4751, (-((p.p67 * locals.var_vtv_dn4) / (assign4350_e4750 * assign4350_e4750))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign4350_e4753;
        locals.var_afac_dn4 = assign4350_e4753_d_n4;
        locals.var_afac_rv = 0.0;

        let assign4360_e4756: f64 = if locals.var_vbcp < locals.var_maxvibcip { 1.0 } else { 0.0 };
        locals.var_guard133 = assign4360_e4756;
        locals.var_guard133_rv = 0.0;

        let (assign4370_e4765, assign4370_e4765_d_n4, assign4370_e4765_d_n5, assign4370_e4765_d_n6, assign4370_e4765_d_n7, assign4370_e4765_d_n8, assign4370_e4765_d_n9, assign4370_e4765_d_n10, assign4370_e4765_d_n11,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard133 != 0.0)) {
        let assign4370_e4762: f64 = (locals.var_vbcp * locals.var_afac);
        let assign4370_e4763: f64 = (assign4370_e4762).exp();
        (assign4370_e4763, (assign4370_e4763 * (locals.var_vbcp * locals.var_afac_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0, (assign4370_e4763 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign4370_e4763 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign4370_e4765;
        locals.var_expi_dn4 = assign4370_e4765_d_n4;
        locals.var_expi_dn5 = assign4370_e4765_d_n5;
        locals.var_expi_dn6 = assign4370_e4765_d_n6;
        locals.var_expi_dn7 = assign4370_e4765_d_n7;
        locals.var_expi_dn8 = assign4370_e4765_d_n8;
        locals.var_expi_dn9 = assign4370_e4765_d_n9;
        locals.var_expi_dn10 = assign4370_e4765_d_n10;
        locals.var_expi_dn11 = assign4370_e4765_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign4380_e4783, assign4380_e4783_d_n4, assign4380_e4783_d_n5, assign4380_e4783_d_n6, assign4380_e4783_d_n7, assign4380_e4783_d_n8, assign4380_e4783_d_n9, assign4380_e4783_d_n10, assign4380_e4783_d_n11,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard133 == 0.0)) {
        let assign4380_e4772: f64 = (locals.var_maxvibcip * locals.var_afac);
        let assign4380_e4773: f64 = (assign4380_e4772).exp();
        let assign4380_e4777: f64 = (locals.var_vbcp - locals.var_maxvibcip);
        let assign4380_e4779: f64 = (assign4380_e4777 * locals.var_afac);
        let assign4380_e4780: f64 = (1.0 + assign4380_e4779);
        let assign4380_e4781: f64 = (assign4380_e4773 * assign4380_e4780);
        (assign4380_e4781, (((assign4380_e4773 * ((locals.var_maxvibcip_dn4 * locals.var_afac) + (locals.var_maxvibcip * locals.var_afac_dn4))) * assign4380_e4780) + (assign4380_e4773 * (((-locals.var_maxvibcip_dn4) * locals.var_afac) + (assign4380_e4777 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, 0.0, 0.0, (assign4380_e4773 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign4380_e4773 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign4380_e4783;
        locals.var_expi_dn4 = assign4380_e4783_d_n4;
        locals.var_expi_dn5 = assign4380_e4783_d_n5;
        locals.var_expi_dn6 = assign4380_e4783_d_n6;
        locals.var_expi_dn7 = assign4380_e4783_d_n7;
        locals.var_expi_dn8 = assign4380_e4783_d_n8;
        locals.var_expi_dn9 = assign4380_e4783_d_n9;
        locals.var_expi_dn10 = assign4380_e4783_d_n10;
        locals.var_expi_dn11 = assign4380_e4783_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign4390_e4791, assign4390_e4791_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign4390_e4788: f64 = (p.p69 * locals.var_vtv);
        let assign4390_e4789: f64 = (1.0 / assign4390_e4788);
        (assign4390_e4789, (-((p.p69 * locals.var_vtv_dn4) / (assign4390_e4788 * assign4390_e4788))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign4390_e4791;
        locals.var_afac_dn4 = assign4390_e4791_d_n4;
        locals.var_afac_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4750_e5022: f64 = if p.p49 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard135 = assign4750_e5022;
        locals.var_guard135_rv = 0.0;

        let (assign4760_e5029, assign4760_e5029_d_n4,) = {
    if (locals.var_guard135 != 0.0) {
        let assign4760_e5025: f64 = (-locals.var_ps_t);
        let assign4760_e5027: f64 = (assign4760_e5025 * p.p34);
        (assign4760_e5027, ((-locals.var_ps_t_dn4) * p.p34),)
    } else {
        (locals.var_dv0__blk136, locals.var_dv0__blk136_dn4,)
    }
};
        locals.var_dv0__blk136 = assign4760_e5029;
        locals.var_dv0__blk136_dn4 = assign4760_e5029_d_n4;
        locals.var_dv0__blk136_rv = 0.0;

        let assign4770_e5032: f64 = if p.p52 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign4770_e5032;
        locals.var_guard147_rv = 0.0;

        let (assign4780_e5040, assign4780_e5040_d_n4, assign4780_e5040_d_n10, assign4780_e5040_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign4780_e5038: f64 = (locals.var_vbcp + locals.var_dv0__blk136);
        (assign4780_e5038, locals.var_dv0__blk136_dn4, locals.var_vbcp_dn10, locals.var_vbcp_dn11,)
    } else {
        (locals.var_dvh__blk137, locals.var_dvh__blk137_dn4, locals.var_dvh__blk137_dn10, locals.var_dvh__blk137_dn11,)
    }
};
        locals.var_dvh__blk137 = assign4780_e5040;
        locals.var_dvh__blk137_dn4 = assign4780_e5040_d_n4;
        locals.var_dvh__blk137_dn10 = assign4780_e5040_d_n10;
        locals.var_dvh__blk137_dn11 = assign4780_e5040_d_n11;
        locals.var_dvh__blk137_rv = 0.0;

        let assign4790_e5043: f64 = if locals.var_dvh__blk137 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign4790_e5043;
        locals.var_guard148_rv = 0.0;

        let (assign4800_e5056,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 != 0.0)) {
        let assign4800_e5051: f64 = (1.0 - p.p34);
        let assign4800_e5053: f64 = (-p.p51);
        let assign4800_e5054: f64 = (assign4800_e5051).powf(assign4800_e5053);
        (assign4800_e5054,)
    } else {
        (locals.var_pwq__blk138,)
    }
};
        locals.var_pwq__blk138 = assign4800_e5056;
        locals.var_pwq__blk138_rv = 0.0;

        let (assign4810_e5076, assign4810_e5076_d_n4, assign4810_e5076_d_n10, assign4810_e5076_d_n11,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 != 0.0)) {
        let assign4810_e5067: f64 = (1.0 - p.p34);
        let assign4810_e5068: f64 = (locals.var_pwq__blk138 * assign4810_e5067);
        let assign4810_e5069: f64 = (1.0 - assign4810_e5068);
        let assign4810_e5070: f64 = (locals.var_ps_t * assign4810_e5069);
        let assign4810_e5073: f64 = (1.0 - p.p51);
        let assign4810_e5074: f64 = (assign4810_e5070 / assign4810_e5073);
        (assign4810_e5074, ((locals.var_ps_t_dn4 * assign4810_e5069) / assign4810_e5073), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk139, locals.var_qlo__blk139_dn4, locals.var_qlo__blk139_dn10, locals.var_qlo__blk139_dn11,)
    }
};
        locals.var_qlo__blk139 = assign4810_e5076;
        locals.var_qlo__blk139_dn4 = assign4810_e5076_d_n4;
        locals.var_qlo__blk139_dn10 = assign4810_e5076_d_n10;
        locals.var_qlo__blk139_dn11 = assign4810_e5076_d_n11;
        locals.var_qlo__blk139_rv = 0.0;

        let (assign4820_e5100, assign4820_e5100_d_n4, assign4820_e5100_d_n10, assign4820_e5100_d_n11,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 != 0.0)) {
        let assign4820_e5086: f64 = (0.5 * p.p51);
        let assign4820_e5088: f64 = (assign4820_e5086 * locals.var_dvh__blk137);
        let assign4820_e5092: f64 = (1.0 - p.p34);
        let assign4820_e5093: f64 = (locals.var_ps_t * assign4820_e5092);
        let assign4820_e5094: f64 = (assign4820_e5088 / assign4820_e5093);
        let assign4820_e5095: f64 = (1.0 + assign4820_e5094);
        let assign4820_e5096: f64 = (locals.var_dvh__blk137 * assign4820_e5095);
        let assign4820_e5098: f64 = (assign4820_e5096 * locals.var_pwq__blk138);
        (assign4820_e5098, (((locals.var_dvh__blk137_dn4 * assign4820_e5095) + (locals.var_dvh__blk137 * ((((assign4820_e5086 * locals.var_dvh__blk137_dn4) * assign4820_e5093) - (assign4820_e5088 * (locals.var_ps_t_dn4 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * locals.var_pwq__blk138), (((locals.var_dvh__blk137_dn10 * assign4820_e5095) + (locals.var_dvh__blk137 * ((assign4820_e5086 * locals.var_dvh__blk137_dn10) / assign4820_e5093))) * locals.var_pwq__blk138), (((locals.var_dvh__blk137_dn11 * assign4820_e5095) + (locals.var_dvh__blk137 * ((assign4820_e5086 * locals.var_dvh__blk137_dn11) / assign4820_e5093))) * locals.var_pwq__blk138),)
    } else {
        (locals.var_qhi__blk140, locals.var_qhi__blk140_dn4, locals.var_qhi__blk140_dn10, locals.var_qhi__blk140_dn11,)
    }
};
        locals.var_qhi__blk140 = assign4820_e5100;
        locals.var_qhi__blk140_dn4 = assign4820_e5100_d_n4;
        locals.var_qhi__blk140_dn10 = assign4820_e5100_d_n10;
        locals.var_qhi__blk140_dn11 = assign4820_e5100_d_n11;
        locals.var_qhi__blk140_rv = 0.0;

        let (assign4830_e5125, assign4830_e5125_d_n4, assign4830_e5125_d_n10, assign4830_e5125_d_n11,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign4830_e5112: f64 = (locals.var_vbcp / locals.var_ps_t);
        let assign4830_e5113: f64 = (1.0 - assign4830_e5112);
        let assign4830_e5116: f64 = (1.0 - p.p51);
        let assign4830_e5117: f64 = (assign4830_e5113).powf(assign4830_e5116);
        let assign4830_e5118: f64 = (1.0 - assign4830_e5117);
        let assign4830_e5119: f64 = (locals.var_ps_t * assign4830_e5118);
        let assign4830_e5122: f64 = (1.0 - p.p51);
        let assign4830_e5123: f64 = (assign4830_e5119 / assign4830_e5122);
        (assign4830_e5123, (((locals.var_ps_t_dn4 * assign4830_e5118) + (locals.var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(-((locals.var_vbcp * locals.var_ps_t_dn4) / (locals.var_ps_t * locals.var_ps_t)))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(-((locals.var_vbcp * locals.var_ps_t_dn4) / (locals.var_ps_t * locals.var_ps_t)))) / assign4830_e5113))) }))) / assign4830_e5122), ((locals.var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(locals.var_vbcp_dn10 / locals.var_ps_t)))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(locals.var_vbcp_dn10 / locals.var_ps_t)) / assign4830_e5113))) })) / assign4830_e5122), ((locals.var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(locals.var_vbcp_dn11 / locals.var_ps_t)))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(locals.var_vbcp_dn11 / locals.var_ps_t)) / assign4830_e5113))) })) / assign4830_e5122),)
    } else {
        (locals.var_qlo__blk139, locals.var_qlo__blk139_dn4, locals.var_qlo__blk139_dn10, locals.var_qlo__blk139_dn11,)
    }
};
        locals.var_qlo__blk139 = assign4830_e5125;
        locals.var_qlo__blk139_dn4 = assign4830_e5125_d_n4;
        locals.var_qlo__blk139_dn10 = assign4830_e5125_d_n10;
        locals.var_qlo__blk139_dn11 = assign4830_e5125_d_n11;
        locals.var_qlo__blk139_rv = 0.0;

        let (assign4840_e5134, assign4840_e5134_d_n4, assign4840_e5134_d_n10, assign4840_e5134_d_n11,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk140, locals.var_qhi__blk140_dn4, locals.var_qhi__blk140_dn10, locals.var_qhi__blk140_dn11,)
    }
};
        locals.var_qhi__blk140 = assign4840_e5134;
        locals.var_qhi__blk140_dn4 = assign4840_e5134_d_n4;
        locals.var_qhi__blk140_dn10 = assign4840_e5134_d_n10;
        locals.var_qhi__blk140_dn11 = assign4840_e5134_d_n11;
        locals.var_qhi__blk140_rv = 0.0;

        let (assign4850_e5142, assign4850_e5142_d_n4, assign4850_e5142_d_n10, assign4850_e5142_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign4850_e5140: f64 = (locals.var_qlo__blk139 + locals.var_qhi__blk140);
        (assign4850_e5140, (locals.var_qlo__blk139_dn4 + locals.var_qhi__blk140_dn4), (locals.var_qlo__blk139_dn10 + locals.var_qhi__blk140_dn10), (locals.var_qlo__blk139_dn11 + locals.var_qhi__blk140_dn11),)
    } else {
        (locals.var_qdbcp, locals.var_qdbcp_dn4, locals.var_qdbcp_dn10, locals.var_qdbcp_dn11,)
    }
};
        locals.var_qdbcp = assign4850_e5142;
        locals.var_qdbcp_dn4 = assign4850_e5142_d_n4;
        locals.var_qdbcp_dn10 = assign4850_e5142_d_n10;
        locals.var_qdbcp_dn11 = assign4850_e5142_d_n11;
        locals.var_qdbcp_rv = 0.0;

        let (assign4860_e5158, assign4860_e5158_d_n4,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4860_e5149: f64 = (locals.var_dv0__blk136 * locals.var_dv0__blk136);
        let assign4860_e5152: f64 = (4.0 * p.p52);
        let assign4860_e5154: f64 = (assign4860_e5152 * p.p52);
        let assign4860_e5155: f64 = (assign4860_e5149 + assign4860_e5154);
        let assign4860_e5156: f64 = (assign4860_e5155).sqrt();
        (assign4860_e5156, (((locals.var_dv0__blk136_dn4 * locals.var_dv0__blk136) + (locals.var_dv0__blk136 * locals.var_dv0__blk136_dn4)) / (2.0 * assign4860_e5156)),)
    } else {
        (locals.var_mv0__blk141, locals.var_mv0__blk141_dn4,)
    }
};
        locals.var_mv0__blk141 = assign4860_e5158;
        locals.var_mv0__blk141_dn4 = assign4860_e5158_d_n4;
        locals.var_mv0__blk141_rv = 0.0;

        let (assign4870_e5170, assign4870_e5170_d_n4,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4870_e5164: f64 = (-0.5);
        let assign4870_e5167: f64 = (locals.var_dv0__blk136 + locals.var_mv0__blk141);
        let assign4870_e5168: f64 = (assign4870_e5164 * assign4870_e5167);
        (assign4870_e5168, (assign4870_e5164 * (locals.var_dv0__blk136_dn4 + locals.var_mv0__blk141_dn4)),)
    } else {
        (locals.var_vl0__blk142, locals.var_vl0__blk142_dn4,)
    }
};
        locals.var_vl0__blk142 = assign4870_e5170;
        locals.var_vl0__blk142_dn4 = assign4870_e5170_d_n4;
        locals.var_vl0__blk142_rv = 0.0;

        let (assign4880_e5192, assign4880_e5192_d_n4,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4880_e5176: f64 = (-locals.var_ps_t);
        let assign4880_e5180: f64 = (locals.var_vl0__blk142 / locals.var_ps_t);
        let assign4880_e5181: f64 = (1.0 - assign4880_e5180);
        let assign4880_e5184: f64 = (1.0 - p.p51);
        let assign4880_e5185: f64 = (assign4880_e5181).powf(assign4880_e5184);
        let assign4880_e5186: f64 = (assign4880_e5176 * assign4880_e5185);
        let assign4880_e5189: f64 = (1.0 - p.p51);
        let assign4880_e5190: f64 = (assign4880_e5186 / assign4880_e5189);
        (assign4880_e5190, ((((-locals.var_ps_t_dn4) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((locals.var_vl0__blk142_dn4 * locals.var_ps_t) - (locals.var_vl0__blk142 * locals.var_ps_t_dn4)) / (locals.var_ps_t * locals.var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((locals.var_vl0__blk142_dn4 * locals.var_ps_t) - (locals.var_vl0__blk142 * locals.var_ps_t_dn4)) / (locals.var_ps_t * locals.var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189),)
    } else {
        (locals.var_q0__blk143, locals.var_q0__blk143_dn4,)
    }
};
        locals.var_q0__blk143 = assign4880_e5192;
        locals.var_q0__blk143_dn4 = assign4880_e5192_d_n4;
        locals.var_q0__blk143_rv = 0.0;

        let (assign4890_e5201, assign4890_e5201_d_n4, assign4890_e5201_d_n10, assign4890_e5201_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4890_e5199: f64 = (locals.var_vbcp + locals.var_dv0__blk136);
        (assign4890_e5199, locals.var_dv0__blk136_dn4, locals.var_vbcp_dn10, locals.var_vbcp_dn11,)
    } else {
        (locals.var_dv__blk144, locals.var_dv__blk144_dn4, locals.var_dv__blk144_dn10, locals.var_dv__blk144_dn11,)
    }
};
        locals.var_dv__blk144 = assign4890_e5201;
        locals.var_dv__blk144_dn4 = assign4890_e5201_d_n4;
        locals.var_dv__blk144_dn10 = assign4890_e5201_d_n10;
        locals.var_dv__blk144_dn11 = assign4890_e5201_d_n11;
        locals.var_dv__blk144_rv = 0.0;

        let (assign4900_e5217, assign4900_e5217_d_n4, assign4900_e5217_d_n10, assign4900_e5217_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4900_e5208: f64 = (locals.var_dv__blk144 * locals.var_dv__blk144);
        let assign4900_e5211: f64 = (4.0 * p.p52);
        let assign4900_e5213: f64 = (assign4900_e5211 * p.p52);
        let assign4900_e5214: f64 = (assign4900_e5208 + assign4900_e5213);
        let assign4900_e5215: f64 = (assign4900_e5214).sqrt();
        (assign4900_e5215, (((locals.var_dv__blk144_dn4 * locals.var_dv__blk144) + (locals.var_dv__blk144 * locals.var_dv__blk144_dn4)) / (2.0 * assign4900_e5215)), (((locals.var_dv__blk144_dn10 * locals.var_dv__blk144) + (locals.var_dv__blk144 * locals.var_dv__blk144_dn10)) / (2.0 * assign4900_e5215)), (((locals.var_dv__blk144_dn11 * locals.var_dv__blk144) + (locals.var_dv__blk144 * locals.var_dv__blk144_dn11)) / (2.0 * assign4900_e5215)),)
    } else {
        (locals.var_mv__blk145, locals.var_mv__blk145_dn4, locals.var_mv__blk145_dn10, locals.var_mv__blk145_dn11,)
    }
};
        locals.var_mv__blk145 = assign4900_e5217;
        locals.var_mv__blk145_dn4 = assign4900_e5217_d_n4;
        locals.var_mv__blk145_dn10 = assign4900_e5217_d_n10;
        locals.var_mv__blk145_dn11 = assign4900_e5217_d_n11;
        locals.var_mv__blk145_rv = 0.0;

        let (assign4910_e5230, assign4910_e5230_d_n4, assign4910_e5230_d_n10, assign4910_e5230_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4910_e5225: f64 = (locals.var_dv__blk144 - locals.var_mv__blk145);
        let assign4910_e5226: f64 = (0.5 * assign4910_e5225);
        let assign4910_e5228: f64 = (assign4910_e5226 - locals.var_dv0__blk136);
        (assign4910_e5228, ((0.5 * (locals.var_dv__blk144_dn4 - locals.var_mv__blk145_dn4)) - locals.var_dv0__blk136_dn4), (0.5 * (locals.var_dv__blk144_dn10 - locals.var_mv__blk145_dn10)), (0.5 * (locals.var_dv__blk144_dn11 - locals.var_mv__blk145_dn11)),)
    } else {
        (locals.var_vl__blk146, locals.var_vl__blk146_dn4, locals.var_vl__blk146_dn10, locals.var_vl__blk146_dn11,)
    }
};
        locals.var_vl__blk146 = assign4910_e5230;
        locals.var_vl__blk146_dn4 = assign4910_e5230_d_n4;
        locals.var_vl__blk146_dn10 = assign4910_e5230_d_n10;
        locals.var_vl__blk146_dn11 = assign4910_e5230_d_n11;
        locals.var_vl__blk146_rv = 0.0;

        let (assign4920_e5252, assign4920_e5252_d_n4, assign4920_e5252_d_n10, assign4920_e5252_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4920_e5236: f64 = (-locals.var_ps_t);
        let assign4920_e5240: f64 = (locals.var_vl__blk146 / locals.var_ps_t);
        let assign4920_e5241: f64 = (1.0 - assign4920_e5240);
        let assign4920_e5244: f64 = (1.0 - p.p51);
        let assign4920_e5245: f64 = (assign4920_e5241).powf(assign4920_e5244);
        let assign4920_e5246: f64 = (assign4920_e5236 * assign4920_e5245);
        let assign4920_e5249: f64 = (1.0 - p.p51);
        let assign4920_e5250: f64 = (assign4920_e5246 / assign4920_e5249);
        (assign4920_e5250, ((((-locals.var_ps_t_dn4) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((locals.var_vl__blk146_dn4 * locals.var_ps_t) - (locals.var_vl__blk146 * locals.var_ps_t_dn4)) / (locals.var_ps_t * locals.var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((locals.var_vl__blk146_dn4 * locals.var_ps_t) - (locals.var_vl__blk146 * locals.var_ps_t_dn4)) / (locals.var_ps_t * locals.var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(locals.var_vl__blk146_dn10 / locals.var_ps_t)))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(locals.var_vl__blk146_dn10 / locals.var_ps_t)) / assign4920_e5241))) }) / assign4920_e5249), ((assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(locals.var_vl__blk146_dn11 / locals.var_ps_t)))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(locals.var_vl__blk146_dn11 / locals.var_ps_t)) / assign4920_e5241))) }) / assign4920_e5249),)
    } else {
        (locals.var_qlo__blk139, locals.var_qlo__blk139_dn4, locals.var_qlo__blk139_dn10, locals.var_qlo__blk139_dn11,)
    }
};
        locals.var_qlo__blk139 = assign4920_e5252;
        locals.var_qlo__blk139_dn4 = assign4920_e5252_d_n4;
        locals.var_qlo__blk139_dn10 = assign4920_e5252_d_n10;
        locals.var_qlo__blk139_dn11 = assign4920_e5252_d_n11;
        locals.var_qlo__blk139_rv = 0.0;

        let (assign4930_e5292, assign4930_e5292_d_n4, assign4930_e5292_d_n10, assign4930_e5292_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4930_e5260: f64 = (1.0 - p.p34);
        let assign4930_e5262: f64 = (-p.p51);
        let assign4930_e5263: f64 = (assign4930_e5260).powf(assign4930_e5262);
        let assign4930_e5266: f64 = (locals.var_vbcp - locals.var_vl__blk146);
        let assign4930_e5268: f64 = (assign4930_e5266 + locals.var_vl0__blk142);
        let assign4930_e5269: f64 = (assign4930_e5263 * assign4930_e5268);
        let assign4930_e5273: f64 = (0.5 * p.p51);
        let assign4930_e5276: f64 = (locals.var_vbcp - locals.var_vl__blk146);
        let assign4930_e5278: f64 = (assign4930_e5276 + locals.var_vl0__blk142);
        let assign4930_e5279: f64 = (assign4930_e5273 * assign4930_e5278);
        let assign4930_e5283: f64 = (1.0 - p.p34);
        let assign4930_e5284: f64 = (locals.var_ps_t * assign4930_e5283);
        let assign4930_e5285: f64 = (assign4930_e5279 / assign4930_e5284);
        let assign4930_e5286: f64 = (1.0 + assign4930_e5285);
        let assign4930_e5287: f64 = (assign4930_e5269 * assign4930_e5286);
        let assign4930_e5288: f64 = (locals.var_qlo__blk139 + assign4930_e5287);
        let assign4930_e5290: f64 = (assign4930_e5288 - locals.var_q0__blk143);
        (assign4930_e5290, ((locals.var_qlo__blk139_dn4 + (((assign4930_e5263 * ((-locals.var_vl__blk146_dn4) + locals.var_vl0__blk142_dn4)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((-locals.var_vl__blk146_dn4) + locals.var_vl0__blk142_dn4)) * assign4930_e5284) - (assign4930_e5279 * (locals.var_ps_t_dn4 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - locals.var_q0__blk143_dn4), (locals.var_qlo__blk139_dn10 + (((assign4930_e5263 * (locals.var_vbcp_dn10 - locals.var_vl__blk146_dn10)) * assign4930_e5286) + (assign4930_e5269 * ((assign4930_e5273 * (locals.var_vbcp_dn10 - locals.var_vl__blk146_dn10)) / assign4930_e5284)))), (locals.var_qlo__blk139_dn11 + (((assign4930_e5263 * (locals.var_vbcp_dn11 - locals.var_vl__blk146_dn11)) * assign4930_e5286) + (assign4930_e5269 * ((assign4930_e5273 * (locals.var_vbcp_dn11 - locals.var_vl__blk146_dn11)) / assign4930_e5284)))),)
    } else {
        (locals.var_qdbcp, locals.var_qdbcp_dn4, locals.var_qdbcp_dn10, locals.var_qdbcp_dn11,)
    }
};
        locals.var_qdbcp = assign4930_e5292;
        locals.var_qdbcp_dn4 = assign4930_e5292_d_n4;
        locals.var_qdbcp_dn10 = assign4930_e5292_d_n10;
        locals.var_qdbcp_dn11 = assign4930_e5292_d_n11;
        locals.var_qdbcp_rv = 0.0;

        let (assign4940_e5297, assign4940_e5297_d_n4, assign4940_e5297_d_n10, assign4940_e5297_d_n11,) = {
    if (locals.var_guard135 == 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdbcp, locals.var_qdbcp_dn4, locals.var_qdbcp_dn10, locals.var_qdbcp_dn11,)
    }
};
        locals.var_qdbcp = assign4940_e5297;
        locals.var_qdbcp_dn4 = assign4940_e5297_d_n4;
        locals.var_qdbcp_dn10 = assign4940_e5297_d_n10;
        locals.var_qdbcp_dn11 = assign4940_e5297_d_n11;
        locals.var_qdbcp_rv = 0.0;

        let assign4950_e5299: f64 = (-locals.var_pe_t);
        let assign4950_e5301: f64 = (assign4950_e5299 * p.p34);
        locals.var_dv0__blk149 = assign4950_e5301;
        locals.var_dv0__blk149_dn4 = ((-locals.var_pe_t_dn4) * p.p34);
        locals.var_dv0__blk149_rv = 0.0;

        let assign4960_e5304: f64 = if p.p39 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign4960_e5304;
        locals.var_guard160_rv = 0.0;

        let (assign4970_e5310, assign4970_e5310_d_n4, assign4970_e5310_d_n7, assign4970_e5310_d_n9,) = {
    if (locals.var_guard160 != 0.0) {
        let assign4970_e5308: f64 = (locals.var_vbex + locals.var_dv0__blk149);
        (assign4970_e5308, locals.var_dv0__blk149_dn4, locals.var_vbex_dn7, locals.var_vbex_dn9,)
    } else {
        (locals.var_dvh__blk150, locals.var_dvh__blk150_dn4, locals.var_dvh__blk150_dn7, locals.var_dvh__blk150_dn9,)
    }
};
        locals.var_dvh__blk150 = assign4970_e5310;
        locals.var_dvh__blk150_dn4 = assign4970_e5310_d_n4;
        locals.var_dvh__blk150_dn7 = assign4970_e5310_d_n7;
        locals.var_dvh__blk150_dn9 = assign4970_e5310_d_n9;
        locals.var_dvh__blk150_rv = 0.0;

        let assign4980_e5313: f64 = if locals.var_dvh__blk150 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign4980_e5313;
        locals.var_guard161_rv = 0.0;

        let (assign4990_e5324,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 != 0.0)) {
        let assign4990_e5319: f64 = (1.0 - p.p34);
        let assign4990_e5321: f64 = (-p.p38);
        let assign4990_e5322: f64 = (assign4990_e5319).powf(assign4990_e5321);
        (assign4990_e5322,)
    } else {
        (locals.var_pwq__blk151,)
    }
};
        locals.var_pwq__blk151 = assign4990_e5324;
        locals.var_pwq__blk151_rv = 0.0;

        let (assign5000_e5342, assign5000_e5342_d_n4, assign5000_e5342_d_n7, assign5000_e5342_d_n9,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 != 0.0)) {
        let assign5000_e5333: f64 = (1.0 - p.p34);
        let assign5000_e5334: f64 = (locals.var_pwq__blk151 * assign5000_e5333);
        let assign5000_e5335: f64 = (1.0 - assign5000_e5334);
        let assign5000_e5336: f64 = (locals.var_pe_t * assign5000_e5335);
        let assign5000_e5339: f64 = (1.0 - p.p38);
        let assign5000_e5340: f64 = (assign5000_e5336 / assign5000_e5339);
        (assign5000_e5340, ((locals.var_pe_t_dn4 * assign5000_e5335) / assign5000_e5339), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk152, locals.var_qlo__blk152_dn4, locals.var_qlo__blk152_dn7, locals.var_qlo__blk152_dn9,)
    }
};
        locals.var_qlo__blk152 = assign5000_e5342;
        locals.var_qlo__blk152_dn4 = assign5000_e5342_d_n4;
        locals.var_qlo__blk152_dn7 = assign5000_e5342_d_n7;
        locals.var_qlo__blk152_dn9 = assign5000_e5342_d_n9;
        locals.var_qlo__blk152_rv = 0.0;

        let (assign5010_e5364, assign5010_e5364_d_n4, assign5010_e5364_d_n7, assign5010_e5364_d_n9,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 != 0.0)) {
        let assign5010_e5350: f64 = (0.5 * p.p38);
        let assign5010_e5352: f64 = (assign5010_e5350 * locals.var_dvh__blk150);
        let assign5010_e5356: f64 = (1.0 - p.p34);
        let assign5010_e5357: f64 = (locals.var_pe_t * assign5010_e5356);
        let assign5010_e5358: f64 = (assign5010_e5352 / assign5010_e5357);
        let assign5010_e5359: f64 = (1.0 + assign5010_e5358);
        let assign5010_e5360: f64 = (locals.var_dvh__blk150 * assign5010_e5359);
        let assign5010_e5362: f64 = (assign5010_e5360 * locals.var_pwq__blk151);
        (assign5010_e5362, (((locals.var_dvh__blk150_dn4 * assign5010_e5359) + (locals.var_dvh__blk150 * ((((assign5010_e5350 * locals.var_dvh__blk150_dn4) * assign5010_e5357) - (assign5010_e5352 * (locals.var_pe_t_dn4 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * locals.var_pwq__blk151), (((locals.var_dvh__blk150_dn7 * assign5010_e5359) + (locals.var_dvh__blk150 * ((assign5010_e5350 * locals.var_dvh__blk150_dn7) / assign5010_e5357))) * locals.var_pwq__blk151), (((locals.var_dvh__blk150_dn9 * assign5010_e5359) + (locals.var_dvh__blk150 * ((assign5010_e5350 * locals.var_dvh__blk150_dn9) / assign5010_e5357))) * locals.var_pwq__blk151),)
    } else {
        (locals.var_qhi__blk153, locals.var_qhi__blk153_dn4, locals.var_qhi__blk153_dn7, locals.var_qhi__blk153_dn9,)
    }
};
        locals.var_qhi__blk153 = assign5010_e5364;
        locals.var_qhi__blk153_dn4 = assign5010_e5364_d_n4;
        locals.var_qhi__blk153_dn7 = assign5010_e5364_d_n7;
        locals.var_qhi__blk153_dn9 = assign5010_e5364_d_n9;
        locals.var_qhi__blk153_rv = 0.0;

        let (assign5020_e5387, assign5020_e5387_d_n4, assign5020_e5387_d_n7, assign5020_e5387_d_n9,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 == 0.0)) {
        let assign5020_e5374: f64 = (locals.var_vbex / locals.var_pe_t);
        let assign5020_e5375: f64 = (1.0 - assign5020_e5374);
        let assign5020_e5378: f64 = (1.0 - p.p38);
        let assign5020_e5379: f64 = (assign5020_e5375).powf(assign5020_e5378);
        let assign5020_e5380: f64 = (1.0 - assign5020_e5379);
        let assign5020_e5381: f64 = (locals.var_pe_t * assign5020_e5380);
        let assign5020_e5384: f64 = (1.0 - p.p38);
        let assign5020_e5385: f64 = (assign5020_e5381 / assign5020_e5384);
        (assign5020_e5385, (((locals.var_pe_t_dn4 * assign5020_e5380) + (locals.var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(-((locals.var_vbex * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t)))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(-((locals.var_vbex * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t)))) / assign5020_e5375))) }))) / assign5020_e5384), ((locals.var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(locals.var_vbex_dn7 / locals.var_pe_t)))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(locals.var_vbex_dn7 / locals.var_pe_t)) / assign5020_e5375))) })) / assign5020_e5384), ((locals.var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(locals.var_vbex_dn9 / locals.var_pe_t)))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(locals.var_vbex_dn9 / locals.var_pe_t)) / assign5020_e5375))) })) / assign5020_e5384),)
    } else {
        (locals.var_qlo__blk152, locals.var_qlo__blk152_dn4, locals.var_qlo__blk152_dn7, locals.var_qlo__blk152_dn9,)
    }
};
        locals.var_qlo__blk152 = assign5020_e5387;
        locals.var_qlo__blk152_dn4 = assign5020_e5387_d_n4;
        locals.var_qlo__blk152_dn7 = assign5020_e5387_d_n7;
        locals.var_qlo__blk152_dn9 = assign5020_e5387_d_n9;
        locals.var_qlo__blk152_rv = 0.0;

        let (assign5030_e5394, assign5030_e5394_d_n4, assign5030_e5394_d_n7, assign5030_e5394_d_n9,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk153, locals.var_qhi__blk153_dn4, locals.var_qhi__blk153_dn7, locals.var_qhi__blk153_dn9,)
    }
};
        locals.var_qhi__blk153 = assign5030_e5394;
        locals.var_qhi__blk153_dn4 = assign5030_e5394_d_n4;
        locals.var_qhi__blk153_dn7 = assign5030_e5394_d_n7;
        locals.var_qhi__blk153_dn9 = assign5030_e5394_d_n9;
        locals.var_qhi__blk153_rv = 0.0;

        let (assign5040_e5400, assign5040_e5400_d_n4, assign5040_e5400_d_n7, assign5040_e5400_d_n9,) = {
    if (locals.var_guard160 != 0.0) {
        let assign5040_e5398: f64 = (locals.var_qlo__blk152 + locals.var_qhi__blk153);
        (assign5040_e5398, (locals.var_qlo__blk152_dn4 + locals.var_qhi__blk153_dn4), (locals.var_qlo__blk152_dn7 + locals.var_qhi__blk153_dn7), (locals.var_qlo__blk152_dn9 + locals.var_qhi__blk153_dn9),)
    } else {
        (locals.var_qdbex, locals.var_qdbex_dn4, locals.var_qdbex_dn7, locals.var_qdbex_dn9,)
    }
};
        locals.var_qdbex = assign5040_e5400;
        locals.var_qdbex_dn4 = assign5040_e5400_d_n4;
        locals.var_qdbex_dn7 = assign5040_e5400_d_n7;
        locals.var_qdbex_dn9 = assign5040_e5400_d_n9;
        locals.var_qdbex_rv = 0.0;

        let (assign5050_e5414, assign5050_e5414_d_n4,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5050_e5405: f64 = (locals.var_dv0__blk149 * locals.var_dv0__blk149);
        let assign5050_e5408: f64 = (4.0 * p.p39);
        let assign5050_e5410: f64 = (assign5050_e5408 * p.p39);
        let assign5050_e5411: f64 = (assign5050_e5405 + assign5050_e5410);
        let assign5050_e5412: f64 = (assign5050_e5411).sqrt();
        (assign5050_e5412, (((locals.var_dv0__blk149_dn4 * locals.var_dv0__blk149) + (locals.var_dv0__blk149 * locals.var_dv0__blk149_dn4)) / (2.0 * assign5050_e5412)),)
    } else {
        (locals.var_mv0__blk154, locals.var_mv0__blk154_dn4,)
    }
};
        locals.var_mv0__blk154 = assign5050_e5414;
        locals.var_mv0__blk154_dn4 = assign5050_e5414_d_n4;
        locals.var_mv0__blk154_rv = 0.0;

        let (assign5060_e5424, assign5060_e5424_d_n4,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5060_e5418: f64 = (-0.5);
        let assign5060_e5421: f64 = (locals.var_dv0__blk149 + locals.var_mv0__blk154);
        let assign5060_e5422: f64 = (assign5060_e5418 * assign5060_e5421);
        (assign5060_e5422, (assign5060_e5418 * (locals.var_dv0__blk149_dn4 + locals.var_mv0__blk154_dn4)),)
    } else {
        (locals.var_vl0__blk155, locals.var_vl0__blk155_dn4,)
    }
};
        locals.var_vl0__blk155 = assign5060_e5424;
        locals.var_vl0__blk155_dn4 = assign5060_e5424_d_n4;
        locals.var_vl0__blk155_rv = 0.0;

        let (assign5070_e5444, assign5070_e5444_d_n4,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5070_e5428: f64 = (-locals.var_pe_t);
        let assign5070_e5432: f64 = (locals.var_vl0__blk155 / locals.var_pe_t);
        let assign5070_e5433: f64 = (1.0 - assign5070_e5432);
        let assign5070_e5436: f64 = (1.0 - p.p38);
        let assign5070_e5437: f64 = (assign5070_e5433).powf(assign5070_e5436);
        let assign5070_e5438: f64 = (assign5070_e5428 * assign5070_e5437);
        let assign5070_e5441: f64 = (1.0 - p.p38);
        let assign5070_e5442: f64 = (assign5070_e5438 / assign5070_e5441);
        (assign5070_e5442, ((((-locals.var_pe_t_dn4) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((locals.var_vl0__blk155_dn4 * locals.var_pe_t) - (locals.var_vl0__blk155 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((locals.var_vl0__blk155_dn4 * locals.var_pe_t) - (locals.var_vl0__blk155 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441),)
    } else {
        (locals.var_q0__blk156, locals.var_q0__blk156_dn4,)
    }
};
        locals.var_q0__blk156 = assign5070_e5444;
        locals.var_q0__blk156_dn4 = assign5070_e5444_d_n4;
        locals.var_q0__blk156_rv = 0.0;

        let (assign5080_e5451, assign5080_e5451_d_n4, assign5080_e5451_d_n7, assign5080_e5451_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5080_e5449: f64 = (locals.var_vbex + locals.var_dv0__blk149);
        (assign5080_e5449, locals.var_dv0__blk149_dn4, locals.var_vbex_dn7, locals.var_vbex_dn9,)
    } else {
        (locals.var_dv__blk157, locals.var_dv__blk157_dn4, locals.var_dv__blk157_dn7, locals.var_dv__blk157_dn9,)
    }
};
        locals.var_dv__blk157 = assign5080_e5451;
        locals.var_dv__blk157_dn4 = assign5080_e5451_d_n4;
        locals.var_dv__blk157_dn7 = assign5080_e5451_d_n7;
        locals.var_dv__blk157_dn9 = assign5080_e5451_d_n9;
        locals.var_dv__blk157_rv = 0.0;

        let (assign5090_e5465, assign5090_e5465_d_n4, assign5090_e5465_d_n7, assign5090_e5465_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5090_e5456: f64 = (locals.var_dv__blk157 * locals.var_dv__blk157);
        let assign5090_e5459: f64 = (4.0 * p.p39);
        let assign5090_e5461: f64 = (assign5090_e5459 * p.p39);
        let assign5090_e5462: f64 = (assign5090_e5456 + assign5090_e5461);
        let assign5090_e5463: f64 = (assign5090_e5462).sqrt();
        (assign5090_e5463, (((locals.var_dv__blk157_dn4 * locals.var_dv__blk157) + (locals.var_dv__blk157 * locals.var_dv__blk157_dn4)) / (2.0 * assign5090_e5463)), (((locals.var_dv__blk157_dn7 * locals.var_dv__blk157) + (locals.var_dv__blk157 * locals.var_dv__blk157_dn7)) / (2.0 * assign5090_e5463)), (((locals.var_dv__blk157_dn9 * locals.var_dv__blk157) + (locals.var_dv__blk157 * locals.var_dv__blk157_dn9)) / (2.0 * assign5090_e5463)),)
    } else {
        (locals.var_mv__blk158, locals.var_mv__blk158_dn4, locals.var_mv__blk158_dn7, locals.var_mv__blk158_dn9,)
    }
};
        locals.var_mv__blk158 = assign5090_e5465;
        locals.var_mv__blk158_dn4 = assign5090_e5465_d_n4;
        locals.var_mv__blk158_dn7 = assign5090_e5465_d_n7;
        locals.var_mv__blk158_dn9 = assign5090_e5465_d_n9;
        locals.var_mv__blk158_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5100_e5476, assign5100_e5476_d_n4, assign5100_e5476_d_n7, assign5100_e5476_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5100_e5471: f64 = (locals.var_dv__blk157 - locals.var_mv__blk158);
        let assign5100_e5472: f64 = (0.5 * assign5100_e5471);
        let assign5100_e5474: f64 = (assign5100_e5472 - locals.var_dv0__blk149);
        (assign5100_e5474, ((0.5 * (locals.var_dv__blk157_dn4 - locals.var_mv__blk158_dn4)) - locals.var_dv0__blk149_dn4), (0.5 * (locals.var_dv__blk157_dn7 - locals.var_mv__blk158_dn7)), (0.5 * (locals.var_dv__blk157_dn9 - locals.var_mv__blk158_dn9)),)
    } else {
        (locals.var_vl__blk159, locals.var_vl__blk159_dn4, locals.var_vl__blk159_dn7, locals.var_vl__blk159_dn9,)
    }
};
        locals.var_vl__blk159 = assign5100_e5476;
        locals.var_vl__blk159_dn4 = assign5100_e5476_d_n4;
        locals.var_vl__blk159_dn7 = assign5100_e5476_d_n7;
        locals.var_vl__blk159_dn9 = assign5100_e5476_d_n9;
        locals.var_vl__blk159_rv = 0.0;

        let (assign5110_e5496, assign5110_e5496_d_n4, assign5110_e5496_d_n7, assign5110_e5496_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5110_e5480: f64 = (-locals.var_pe_t);
        let assign5110_e5484: f64 = (locals.var_vl__blk159 / locals.var_pe_t);
        let assign5110_e5485: f64 = (1.0 - assign5110_e5484);
        let assign5110_e5488: f64 = (1.0 - p.p38);
        let assign5110_e5489: f64 = (assign5110_e5485).powf(assign5110_e5488);
        let assign5110_e5490: f64 = (assign5110_e5480 * assign5110_e5489);
        let assign5110_e5493: f64 = (1.0 - p.p38);
        let assign5110_e5494: f64 = (assign5110_e5490 / assign5110_e5493);
        (assign5110_e5494, ((((-locals.var_pe_t_dn4) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((locals.var_vl__blk159_dn4 * locals.var_pe_t) - (locals.var_vl__blk159 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((locals.var_vl__blk159_dn4 * locals.var_pe_t) - (locals.var_vl__blk159 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(locals.var_vl__blk159_dn7 / locals.var_pe_t)))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(locals.var_vl__blk159_dn7 / locals.var_pe_t)) / assign5110_e5485))) }) / assign5110_e5493), ((assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(locals.var_vl__blk159_dn9 / locals.var_pe_t)))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(locals.var_vl__blk159_dn9 / locals.var_pe_t)) / assign5110_e5485))) }) / assign5110_e5493),)
    } else {
        (locals.var_qlo__blk152, locals.var_qlo__blk152_dn4, locals.var_qlo__blk152_dn7, locals.var_qlo__blk152_dn9,)
    }
};
        locals.var_qlo__blk152 = assign5110_e5496;
        locals.var_qlo__blk152_dn4 = assign5110_e5496_d_n4;
        locals.var_qlo__blk152_dn7 = assign5110_e5496_d_n7;
        locals.var_qlo__blk152_dn9 = assign5110_e5496_d_n9;
        locals.var_qlo__blk152_rv = 0.0;

        let (assign5120_e5534, assign5120_e5534_d_n4, assign5120_e5534_d_n7, assign5120_e5534_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5120_e5502: f64 = (1.0 - p.p34);
        let assign5120_e5504: f64 = (-p.p38);
        let assign5120_e5505: f64 = (assign5120_e5502).powf(assign5120_e5504);
        let assign5120_e5508: f64 = (locals.var_vbex - locals.var_vl__blk159);
        let assign5120_e5510: f64 = (assign5120_e5508 + locals.var_vl0__blk155);
        let assign5120_e5511: f64 = (assign5120_e5505 * assign5120_e5510);
        let assign5120_e5515: f64 = (0.5 * p.p38);
        let assign5120_e5518: f64 = (locals.var_vbex - locals.var_vl__blk159);
        let assign5120_e5520: f64 = (assign5120_e5518 + locals.var_vl0__blk155);
        let assign5120_e5521: f64 = (assign5120_e5515 * assign5120_e5520);
        let assign5120_e5525: f64 = (1.0 - p.p34);
        let assign5120_e5526: f64 = (locals.var_pe_t * assign5120_e5525);
        let assign5120_e5527: f64 = (assign5120_e5521 / assign5120_e5526);
        let assign5120_e5528: f64 = (1.0 + assign5120_e5527);
        let assign5120_e5529: f64 = (assign5120_e5511 * assign5120_e5528);
        let assign5120_e5530: f64 = (locals.var_qlo__blk152 + assign5120_e5529);
        let assign5120_e5532: f64 = (assign5120_e5530 - locals.var_q0__blk156);
        (assign5120_e5532, ((locals.var_qlo__blk152_dn4 + (((assign5120_e5505 * ((-locals.var_vl__blk159_dn4) + locals.var_vl0__blk155_dn4)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((-locals.var_vl__blk159_dn4) + locals.var_vl0__blk155_dn4)) * assign5120_e5526) - (assign5120_e5521 * (locals.var_pe_t_dn4 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - locals.var_q0__blk156_dn4), (locals.var_qlo__blk152_dn7 + (((assign5120_e5505 * (locals.var_vbex_dn7 - locals.var_vl__blk159_dn7)) * assign5120_e5528) + (assign5120_e5511 * ((assign5120_e5515 * (locals.var_vbex_dn7 - locals.var_vl__blk159_dn7)) / assign5120_e5526)))), (locals.var_qlo__blk152_dn9 + (((assign5120_e5505 * (locals.var_vbex_dn9 - locals.var_vl__blk159_dn9)) * assign5120_e5528) + (assign5120_e5511 * ((assign5120_e5515 * (locals.var_vbex_dn9 - locals.var_vl__blk159_dn9)) / assign5120_e5526)))),)
    } else {
        (locals.var_qdbex, locals.var_qdbex_dn4, locals.var_qdbex_dn7, locals.var_qdbex_dn9,)
    }
};
        locals.var_qdbex = assign5120_e5534;
        locals.var_qdbex_dn4 = assign5120_e5534_d_n4;
        locals.var_qdbex_dn7 = assign5120_e5534_d_n7;
        locals.var_qdbex_dn9 = assign5120_e5534_d_n9;
        locals.var_qdbex_rv = 0.0;

        let assign5130_e5536: f64 = (-locals.var_pc_t);
        let assign5130_e5538: f64 = (assign5130_e5536 * p.p34);
        locals.var_dv0__blk162 = assign5130_e5538;
        locals.var_dv0__blk162_dn4 = ((-locals.var_pc_t_dn4) * p.p34);
        locals.var_dv0__blk162_rv = 0.0;

        let assign5140_e5541: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard183 = assign5140_e5541;
        locals.var_guard183_rv = 0.0;

        let (assign5150_e5547, assign5150_e5547_d_n4, assign5150_e5547_d_n7, assign5150_e5547_d_n10,) = {
    if (locals.var_guard183 != 0.0) {
        let assign5150_e5545: f64 = (locals.var_vbep + locals.var_dv0__blk162);
        (assign5150_e5545, locals.var_dv0__blk162_dn4, locals.var_vbep_dn7, locals.var_vbep_dn10,)
    } else {
        (locals.var_dvh__blk163, locals.var_dvh__blk163_dn4, locals.var_dvh__blk163_dn7, locals.var_dvh__blk163_dn10,)
    }
};
        locals.var_dvh__blk163 = assign5150_e5547;
        locals.var_dvh__blk163_dn4 = assign5150_e5547_d_n4;
        locals.var_dvh__blk163_dn7 = assign5150_e5547_d_n7;
        locals.var_dvh__blk163_dn10 = assign5150_e5547_d_n10;
        locals.var_dvh__blk163_rv = 0.0;

        let assign5160_e5550: f64 = if locals.var_dvh__blk163 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard184 = assign5160_e5550;
        locals.var_guard184_rv = 0.0;

        let (assign5170_e5563,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 != 0.0)) {
        let assign5170_e5556: f64 = (1.0 - p.p34);
        let assign5170_e5558: f64 = (-1.0);
        let assign5170_e5560: f64 = (assign5170_e5558 - p.p43);
        let assign5170_e5561: f64 = (assign5170_e5556).powf(assign5170_e5560);
        (assign5170_e5561,)
    } else {
        (locals.var_pwq__blk164,)
    }
};
        locals.var_pwq__blk164 = assign5170_e5563;
        locals.var_pwq__blk164_rv = 0.0;

        let (assign5180_e5585, assign5180_e5585_d_n4, assign5180_e5585_d_n7, assign5180_e5585_d_n10,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 != 0.0)) {
        let assign5180_e5572: f64 = (1.0 - p.p34);
        let assign5180_e5573: f64 = (locals.var_pwq__blk164 * assign5180_e5572);
        let assign5180_e5576: f64 = (1.0 - p.p34);
        let assign5180_e5577: f64 = (assign5180_e5573 * assign5180_e5576);
        let assign5180_e5578: f64 = (1.0 - assign5180_e5577);
        let assign5180_e5579: f64 = (locals.var_pc_t * assign5180_e5578);
        let assign5180_e5582: f64 = (1.0 - p.p43);
        let assign5180_e5583: f64 = (assign5180_e5579 / assign5180_e5582);
        (assign5180_e5583, ((locals.var_pc_t_dn4 * assign5180_e5578) / assign5180_e5582), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5180_e5585;
        locals.var_qlo__blk165_dn4 = assign5180_e5585_d_n4;
        locals.var_qlo__blk165_dn7 = assign5180_e5585_d_n7;
        locals.var_qlo__blk165_dn10 = assign5180_e5585_d_n10;
        locals.var_qlo__blk165_rv = 0.0;

        let (assign5190_e5605, assign5190_e5605_d_n4, assign5190_e5605_d_n7, assign5190_e5605_d_n10,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 != 0.0)) {
        let assign5190_e5592: f64 = (1.0 - p.p34);
        let assign5190_e5595: f64 = (0.5 * p.p43);
        let assign5190_e5597: f64 = (assign5190_e5595 * locals.var_dvh__blk163);
        let assign5190_e5599: f64 = (assign5190_e5597 / locals.var_pc_t);
        let assign5190_e5600: f64 = (assign5190_e5592 + assign5190_e5599);
        let assign5190_e5601: f64 = (locals.var_dvh__blk163 * assign5190_e5600);
        let assign5190_e5603: f64 = (assign5190_e5601 * locals.var_pwq__blk164);
        (assign5190_e5603, (((locals.var_dvh__blk163_dn4 * assign5190_e5600) + (locals.var_dvh__blk163 * ((((assign5190_e5595 * locals.var_dvh__blk163_dn4) * locals.var_pc_t) - (assign5190_e5597 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)))) * locals.var_pwq__blk164), (((locals.var_dvh__blk163_dn7 * assign5190_e5600) + (locals.var_dvh__blk163 * ((assign5190_e5595 * locals.var_dvh__blk163_dn7) / locals.var_pc_t))) * locals.var_pwq__blk164), (((locals.var_dvh__blk163_dn10 * assign5190_e5600) + (locals.var_dvh__blk163 * ((assign5190_e5595 * locals.var_dvh__blk163_dn10) / locals.var_pc_t))) * locals.var_pwq__blk164),)
    } else {
        (locals.var_qhi__blk166, locals.var_qhi__blk166_dn4, locals.var_qhi__blk166_dn7, locals.var_qhi__blk166_dn10,)
    }
};
        locals.var_qhi__blk166 = assign5190_e5605;
        locals.var_qhi__blk166_dn4 = assign5190_e5605_d_n4;
        locals.var_qhi__blk166_dn7 = assign5190_e5605_d_n7;
        locals.var_qhi__blk166_dn10 = assign5190_e5605_d_n10;
        locals.var_qhi__blk166_rv = 0.0;

        let assign5200_e5611: f64 = (-p.p45);
        let assign5200_e5613: f64 = if ((p.p45 > 0.0) && (locals.var_vbep < assign5200_e5611)) { 1.0 } else { 0.0 };
        locals.var_guard185 = assign5200_e5613;
        locals.var_guard185_rv = 0.0;

        let (assign5210_e5652, assign5210_e5652_d_n4, assign5210_e5652_d_n7, assign5210_e5652_d_n10,) = {
    if (((locals.var_guard183 != 0.0) && (locals.var_guard184 == 0.0)) && (locals.var_guard185 != 0.0)) {
        let assign5210_e5625: f64 = (p.p45 / locals.var_pc_t);
        let assign5210_e5626: f64 = (1.0 + assign5210_e5625);
        let assign5210_e5629: f64 = (1.0 - p.p43);
        let assign5210_e5630: f64 = (assign5210_e5626).powf(assign5210_e5629);
        let assign5210_e5634: f64 = (1.0 - p.p43);
        let assign5210_e5637: f64 = (locals.var_vbep + p.p45);
        let assign5210_e5638: f64 = (assign5210_e5634 * assign5210_e5637);
        let assign5210_e5641: f64 = (locals.var_pc_t + p.p45);
        let assign5210_e5642: f64 = (assign5210_e5638 / assign5210_e5641);
        let assign5210_e5643: f64 = (1.0 - assign5210_e5642);
        let assign5210_e5644: f64 = (assign5210_e5630 * assign5210_e5643);
        let assign5210_e5645: f64 = (1.0 - assign5210_e5644);
        let assign5210_e5646: f64 = (locals.var_pc_t * assign5210_e5645);
        let assign5210_e5649: f64 = (1.0 - p.p43);
        let assign5210_e5650: f64 = (assign5210_e5646 / assign5210_e5649);
        (assign5210_e5650, (((locals.var_pc_t_dn4 * assign5210_e5645) + (locals.var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-(-((assign5210_e5638 * locals.var_pc_t_dn4) / (assign5210_e5641 * assign5210_e5641))))))))) / assign5210_e5649), ((locals.var_pc_t * (-(assign5210_e5630 * (-((assign5210_e5634 * locals.var_vbep_dn7) / assign5210_e5641))))) / assign5210_e5649), ((locals.var_pc_t * (-(assign5210_e5630 * (-((assign5210_e5634 * locals.var_vbep_dn10) / assign5210_e5641))))) / assign5210_e5649),)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5210_e5652;
        locals.var_qlo__blk165_dn4 = assign5210_e5652_d_n4;
        locals.var_qlo__blk165_dn7 = assign5210_e5652_d_n7;
        locals.var_qlo__blk165_dn10 = assign5210_e5652_d_n10;
        locals.var_qlo__blk165_rv = 0.0;

        let (assign5220_e5678, assign5220_e5678_d_n4, assign5220_e5678_d_n7, assign5220_e5678_d_n10,) = {
    if (((locals.var_guard183 != 0.0) && (locals.var_guard184 == 0.0)) && (locals.var_guard185 == 0.0)) {
        let assign5220_e5665: f64 = (locals.var_vbep / locals.var_pc_t);
        let assign5220_e5666: f64 = (1.0 - assign5220_e5665);
        let assign5220_e5669: f64 = (1.0 - p.p43);
        let assign5220_e5670: f64 = (assign5220_e5666).powf(assign5220_e5669);
        let assign5220_e5671: f64 = (1.0 - assign5220_e5670);
        let assign5220_e5672: f64 = (locals.var_pc_t * assign5220_e5671);
        let assign5220_e5675: f64 = (1.0 - p.p43);
        let assign5220_e5676: f64 = (assign5220_e5672 / assign5220_e5675);
        (assign5220_e5676, (((locals.var_pc_t_dn4 * assign5220_e5671) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(-((locals.var_vbep * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t)))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(-((locals.var_vbep * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t)))) / assign5220_e5666))) }))) / assign5220_e5675), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(locals.var_vbep_dn7 / locals.var_pc_t)))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(locals.var_vbep_dn7 / locals.var_pc_t)) / assign5220_e5666))) })) / assign5220_e5675), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(locals.var_vbep_dn10 / locals.var_pc_t)))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(locals.var_vbep_dn10 / locals.var_pc_t)) / assign5220_e5666))) })) / assign5220_e5675),)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5220_e5678;
        locals.var_qlo__blk165_dn4 = assign5220_e5678_d_n4;
        locals.var_qlo__blk165_dn7 = assign5220_e5678_d_n7;
        locals.var_qlo__blk165_dn10 = assign5220_e5678_d_n10;
        locals.var_qlo__blk165_rv = 0.0;

        let (assign5230_e5685, assign5230_e5685_d_n4, assign5230_e5685_d_n7, assign5230_e5685_d_n10,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk166, locals.var_qhi__blk166_dn4, locals.var_qhi__blk166_dn7, locals.var_qhi__blk166_dn10,)
    }
};
        locals.var_qhi__blk166 = assign5230_e5685;
        locals.var_qhi__blk166_dn4 = assign5230_e5685_d_n4;
        locals.var_qhi__blk166_dn7 = assign5230_e5685_d_n7;
        locals.var_qhi__blk166_dn10 = assign5230_e5685_d_n10;
        locals.var_qhi__blk166_rv = 0.0;

        let (assign5240_e5691, assign5240_e5691_d_n4, assign5240_e5691_d_n7, assign5240_e5691_d_n10,) = {
    if (locals.var_guard183 != 0.0) {
        let assign5240_e5689: f64 = (locals.var_qlo__blk165 + locals.var_qhi__blk166);
        (assign5240_e5689, (locals.var_qlo__blk165_dn4 + locals.var_qhi__blk166_dn4), (locals.var_qlo__blk165_dn7 + locals.var_qhi__blk166_dn7), (locals.var_qlo__blk165_dn10 + locals.var_qhi__blk166_dn10),)
    } else {
        (locals.var_qdbep, locals.var_qdbep_dn4, locals.var_qdbep_dn7, locals.var_qdbep_dn10,)
    }
};
        locals.var_qdbep = assign5240_e5691;
        locals.var_qdbep_dn4 = assign5240_e5691_d_n4;
        locals.var_qdbep_dn7 = assign5240_e5691_d_n7;
        locals.var_qdbep_dn10 = assign5240_e5691_d_n10;
        locals.var_qdbep_rv = 0.0;

        let assign5250_e5698: f64 = if ((p.p45 > 0.0) && (p.p46 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard186 = assign5250_e5698;
        locals.var_guard186_rv = 0.0;

        let (assign5260_e5711, assign5260_e5711_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5260_e5705: f64 = (p.p45 + locals.var_dv0__blk162);
        let assign5260_e5708: f64 = (p.p45 - locals.var_dv0__blk162);
        let assign5260_e5709: f64 = (assign5260_e5705 / assign5260_e5708);
        (assign5260_e5709, (((locals.var_dv0__blk162_dn4 * assign5260_e5708) - (assign5260_e5705 * (-locals.var_dv0__blk162_dn4))) / (assign5260_e5708 * assign5260_e5708)),)
    } else {
        (locals.var_vn0__blk167, locals.var_vn0__blk167_dn4,)
    }
};
        locals.var_vn0__blk167 = assign5260_e5711;
        locals.var_vn0__blk167_dn4 = assign5260_e5711_d_n4;
        locals.var_vn0__blk167_rv = 0.0;

        let (assign5270_e5750, assign5270_e5750_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5270_e5718: f64 = (2.0 * locals.var_vn0__blk167);
        let assign5270_e5721: f64 = (locals.var_vn0__blk167 - 1.0);
        let assign5270_e5724: f64 = (locals.var_vn0__blk167 - 1.0);
        let assign5270_e5725: f64 = (assign5270_e5721 * assign5270_e5724);
        let assign5270_e5728: f64 = (4.0 * p.p44);
        let assign5270_e5730: f64 = (assign5270_e5728 * p.p44);
        let assign5270_e5731: f64 = (assign5270_e5725 + assign5270_e5730);
        let assign5270_e5732: f64 = (assign5270_e5731).sqrt();
        let assign5270_e5735: f64 = (locals.var_vn0__blk167 + 1.0);
        let assign5270_e5738: f64 = (locals.var_vn0__blk167 + 1.0);
        let assign5270_e5739: f64 = (assign5270_e5735 * assign5270_e5738);
        let assign5270_e5742: f64 = (4.0 * p.p46);
        let assign5270_e5744: f64 = (assign5270_e5742 * p.p46);
        let assign5270_e5745: f64 = (assign5270_e5739 + assign5270_e5744);
        let assign5270_e5746: f64 = (assign5270_e5745).sqrt();
        let assign5270_e5747: f64 = (assign5270_e5732 + assign5270_e5746);
        let assign5270_e5748: f64 = (assign5270_e5718 / assign5270_e5747);
        (assign5270_e5748, ((((2.0 * locals.var_vn0__blk167_dn4) * assign5270_e5747) - (assign5270_e5718 * ((((locals.var_vn0__blk167_dn4 * assign5270_e5724) + (assign5270_e5721 * locals.var_vn0__blk167_dn4)) / (2.0 * assign5270_e5732)) + (((locals.var_vn0__blk167_dn4 * assign5270_e5738) + (assign5270_e5735 * locals.var_vn0__blk167_dn4)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)),)
    } else {
        (locals.var_vnl0__blk168, locals.var_vnl0__blk168_dn4,)
    }
};
        locals.var_vnl0__blk168 = assign5270_e5750;
        locals.var_vnl0__blk168_dn4 = assign5270_e5750_d_n4;
        locals.var_vnl0__blk168_rv = 0.0;

        let (assign5280_e5767, assign5280_e5767_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5280_e5759: f64 = (p.p45 - locals.var_dv0__blk162);
        let assign5280_e5760: f64 = (locals.var_vnl0__blk168 * assign5280_e5759);
        let assign5280_e5762: f64 = (assign5280_e5760 - p.p45);
        let assign5280_e5764: f64 = (assign5280_e5762 - locals.var_dv0__blk162);
        let assign5280_e5765: f64 = (0.5 * assign5280_e5764);
        (assign5280_e5765, (0.5 * (((locals.var_vnl0__blk168_dn4 * assign5280_e5759) + (locals.var_vnl0__blk168 * (-locals.var_dv0__blk162_dn4))) - locals.var_dv0__blk162_dn4)),)
    } else {
        (locals.var_vl0__blk169, locals.var_vl0__blk169_dn4,)
    }
};
        locals.var_vl0__blk169 = assign5280_e5767;
        locals.var_vl0__blk169_dn4 = assign5280_e5767_d_n4;
        locals.var_vl0__blk169_rv = 0.0;

        let (assign5290_e5790, assign5290_e5790_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5290_e5777: f64 = (locals.var_vl0__blk169 / locals.var_pc_t);
        let assign5290_e5778: f64 = (1.0 - assign5290_e5777);
        let assign5290_e5781: f64 = (1.0 - p.p43);
        let assign5290_e5782: f64 = (assign5290_e5778).powf(assign5290_e5781);
        let assign5290_e5783: f64 = (1.0 - assign5290_e5782);
        let assign5290_e5784: f64 = (locals.var_pc_t * assign5290_e5783);
        let assign5290_e5787: f64 = (1.0 - p.p43);
        let assign5290_e5788: f64 = (assign5290_e5784 / assign5290_e5787);
        (assign5290_e5788, (((locals.var_pc_t_dn4 * assign5290_e5783) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((locals.var_vl0__blk169_dn4 * locals.var_pc_t) - (locals.var_vl0__blk169 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((locals.var_vl0__blk169_dn4 * locals.var_pc_t) - (locals.var_vl0__blk169 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787),)
    } else {
        (locals.var_qlo0__blk170, locals.var_qlo0__blk170_dn4,)
    }
};
        locals.var_qlo0__blk170 = assign5290_e5790;
        locals.var_qlo0__blk170_dn4 = assign5290_e5790_d_n4;
        locals.var_qlo0__blk170_rv = 0.0;

        let (assign5300_e5807, assign5300_e5807_d_n4, assign5300_e5807_d_n7, assign5300_e5807_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5300_e5797: f64 = (2.0 * locals.var_vbep);
        let assign5300_e5799: f64 = (assign5300_e5797 + p.p45);
        let assign5300_e5801: f64 = (assign5300_e5799 + locals.var_dv0__blk162);
        let assign5300_e5804: f64 = (p.p45 - locals.var_dv0__blk162);
        let assign5300_e5805: f64 = (assign5300_e5801 / assign5300_e5804);
        (assign5300_e5805, (((locals.var_dv0__blk162_dn4 * assign5300_e5804) - (assign5300_e5801 * (-locals.var_dv0__blk162_dn4))) / (assign5300_e5804 * assign5300_e5804)), ((2.0 * locals.var_vbep_dn7) / assign5300_e5804), ((2.0 * locals.var_vbep_dn10) / assign5300_e5804),)
    } else {
        (locals.var_vn__blk171, locals.var_vn__blk171_dn4, locals.var_vn__blk171_dn7, locals.var_vn__blk171_dn10,)
    }
};
        locals.var_vn__blk171 = assign5300_e5807;
        locals.var_vn__blk171_dn4 = assign5300_e5807_d_n4;
        locals.var_vn__blk171_dn7 = assign5300_e5807_d_n7;
        locals.var_vn__blk171_dn10 = assign5300_e5807_d_n10;
        locals.var_vn__blk171_rv = 0.0;

        let (assign5310_e5846, assign5310_e5846_d_n4, assign5310_e5846_d_n7, assign5310_e5846_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5310_e5814: f64 = (2.0 * locals.var_vn__blk171);
        let assign5310_e5817: f64 = (locals.var_vn__blk171 - 1.0);
        let assign5310_e5820: f64 = (locals.var_vn__blk171 - 1.0);
        let assign5310_e5821: f64 = (assign5310_e5817 * assign5310_e5820);
        let assign5310_e5824: f64 = (4.0 * p.p44);
        let assign5310_e5826: f64 = (assign5310_e5824 * p.p44);
        let assign5310_e5827: f64 = (assign5310_e5821 + assign5310_e5826);
        let assign5310_e5828: f64 = (assign5310_e5827).sqrt();
        let assign5310_e5831: f64 = (locals.var_vn__blk171 + 1.0);
        let assign5310_e5834: f64 = (locals.var_vn__blk171 + 1.0);
        let assign5310_e5835: f64 = (assign5310_e5831 * assign5310_e5834);
        let assign5310_e5838: f64 = (4.0 * p.p46);
        let assign5310_e5840: f64 = (assign5310_e5838 * p.p46);
        let assign5310_e5841: f64 = (assign5310_e5835 + assign5310_e5840);
        let assign5310_e5842: f64 = (assign5310_e5841).sqrt();
        let assign5310_e5843: f64 = (assign5310_e5828 + assign5310_e5842);
        let assign5310_e5844: f64 = (assign5310_e5814 / assign5310_e5843);
        (assign5310_e5844, ((((2.0 * locals.var_vn__blk171_dn4) * assign5310_e5843) - (assign5310_e5814 * ((((locals.var_vn__blk171_dn4 * assign5310_e5820) + (assign5310_e5817 * locals.var_vn__blk171_dn4)) / (2.0 * assign5310_e5828)) + (((locals.var_vn__blk171_dn4 * assign5310_e5834) + (assign5310_e5831 * locals.var_vn__blk171_dn4)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * locals.var_vn__blk171_dn7) * assign5310_e5843) - (assign5310_e5814 * ((((locals.var_vn__blk171_dn7 * assign5310_e5820) + (assign5310_e5817 * locals.var_vn__blk171_dn7)) / (2.0 * assign5310_e5828)) + (((locals.var_vn__blk171_dn7 * assign5310_e5834) + (assign5310_e5831 * locals.var_vn__blk171_dn7)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * locals.var_vn__blk171_dn10) * assign5310_e5843) - (assign5310_e5814 * ((((locals.var_vn__blk171_dn10 * assign5310_e5820) + (assign5310_e5817 * locals.var_vn__blk171_dn10)) / (2.0 * assign5310_e5828)) + (((locals.var_vn__blk171_dn10 * assign5310_e5834) + (assign5310_e5831 * locals.var_vn__blk171_dn10)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)),)
    } else {
        (locals.var_vnl__blk172, locals.var_vnl__blk172_dn4, locals.var_vnl__blk172_dn7, locals.var_vnl__blk172_dn10,)
    }
};
        locals.var_vnl__blk172 = assign5310_e5846;
        locals.var_vnl__blk172_dn4 = assign5310_e5846_d_n4;
        locals.var_vnl__blk172_dn7 = assign5310_e5846_d_n7;
        locals.var_vnl__blk172_dn10 = assign5310_e5846_d_n10;
        locals.var_vnl__blk172_rv = 0.0;

        let (assign5320_e5863, assign5320_e5863_d_n4, assign5320_e5863_d_n7, assign5320_e5863_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5320_e5855: f64 = (p.p45 - locals.var_dv0__blk162);
        let assign5320_e5856: f64 = (locals.var_vnl__blk172 * assign5320_e5855);
        let assign5320_e5858: f64 = (assign5320_e5856 - p.p45);
        let assign5320_e5860: f64 = (assign5320_e5858 - locals.var_dv0__blk162);
        let assign5320_e5861: f64 = (0.5 * assign5320_e5860);
        (assign5320_e5861, (0.5 * (((locals.var_vnl__blk172_dn4 * assign5320_e5855) + (locals.var_vnl__blk172 * (-locals.var_dv0__blk162_dn4))) - locals.var_dv0__blk162_dn4)), (0.5 * (locals.var_vnl__blk172_dn7 * assign5320_e5855)), (0.5 * (locals.var_vnl__blk172_dn10 * assign5320_e5855)),)
    } else {
        (locals.var_vl__blk173, locals.var_vl__blk173_dn4, locals.var_vl__blk173_dn7, locals.var_vl__blk173_dn10,)
    }
};
        locals.var_vl__blk173 = assign5320_e5863;
        locals.var_vl__blk173_dn4 = assign5320_e5863_d_n4;
        locals.var_vl__blk173_dn7 = assign5320_e5863_d_n7;
        locals.var_vl__blk173_dn10 = assign5320_e5863_d_n10;
        locals.var_vl__blk173_rv = 0.0;

        let (assign5330_e5886, assign5330_e5886_d_n4, assign5330_e5886_d_n7, assign5330_e5886_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5330_e5873: f64 = (locals.var_vl__blk173 / locals.var_pc_t);
        let assign5330_e5874: f64 = (1.0 - assign5330_e5873);
        let assign5330_e5877: f64 = (1.0 - p.p43);
        let assign5330_e5878: f64 = (assign5330_e5874).powf(assign5330_e5877);
        let assign5330_e5879: f64 = (1.0 - assign5330_e5878);
        let assign5330_e5880: f64 = (locals.var_pc_t * assign5330_e5879);
        let assign5330_e5883: f64 = (1.0 - p.p43);
        let assign5330_e5884: f64 = (assign5330_e5880 / assign5330_e5883);
        (assign5330_e5884, (((locals.var_pc_t_dn4 * assign5330_e5879) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((locals.var_vl__blk173_dn4 * locals.var_pc_t) - (locals.var_vl__blk173 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((locals.var_vl__blk173_dn4 * locals.var_pc_t) - (locals.var_vl__blk173 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(locals.var_vl__blk173_dn7 / locals.var_pc_t)))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(locals.var_vl__blk173_dn7 / locals.var_pc_t)) / assign5330_e5874))) })) / assign5330_e5883), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(locals.var_vl__blk173_dn10 / locals.var_pc_t)))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(locals.var_vl__blk173_dn10 / locals.var_pc_t)) / assign5330_e5874))) })) / assign5330_e5883),)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5330_e5886;
        locals.var_qlo__blk165_dn4 = assign5330_e5886_d_n4;
        locals.var_qlo__blk165_dn7 = assign5330_e5886_d_n7;
        locals.var_qlo__blk165_dn10 = assign5330_e5886_d_n10;
        locals.var_qlo__blk165_rv = 0.0;

        let (assign5340_e5897, assign5340_e5897_d_n4, assign5340_e5897_d_n7, assign5340_e5897_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5340_e5894: f64 = (locals.var_vnl__blk172 + 1.0);
        let assign5340_e5895: f64 = (0.5 * assign5340_e5894);
        (assign5340_e5895, (0.5 * locals.var_vnl__blk172_dn4), (0.5 * locals.var_vnl__blk172_dn7), (0.5 * locals.var_vnl__blk172_dn10),)
    } else {
        (locals.var_sel__blk174, locals.var_sel__blk174_dn4, locals.var_sel__blk174_dn7, locals.var_sel__blk174_dn10,)
    }
};
        locals.var_sel__blk174 = assign5340_e5897;
        locals.var_sel__blk174_dn4 = assign5340_e5897_d_n4;
        locals.var_sel__blk174_dn7 = assign5340_e5897_d_n7;
        locals.var_sel__blk174_dn10 = assign5340_e5897_d_n10;
        locals.var_sel__blk174_rv = 0.0;

        let (assign5350_e5911, assign5350_e5911_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5350_e5905: f64 = (p.p45 / locals.var_pc_t);
        let assign5350_e5906: f64 = (1.0 + assign5350_e5905);
        let assign5350_e5908: f64 = (-p.p43);
        let assign5350_e5909: f64 = (assign5350_e5906).powf(assign5350_e5908);
        (assign5350_e5909, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign5350_e5906))) },)
    } else {
        (locals.var_crt__blk175, locals.var_crt__blk175_dn4,)
    }
};
        locals.var_crt__blk175 = assign5350_e5911;
        locals.var_crt__blk175_dn4 = assign5350_e5911_d_n4;
        locals.var_crt__blk175_rv = 0.0;

        let (assign5360_e5925, assign5360_e5925_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5360_e5919: f64 = (locals.var_dv0__blk162 / locals.var_pc_t);
        let assign5360_e5920: f64 = (1.0 + assign5360_e5919);
        let assign5360_e5922: f64 = (-p.p43);
        let assign5360_e5923: f64 = (assign5360_e5920).powf(assign5360_e5922);
        (assign5360_e5923, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((locals.var_dv0__blk162_dn4 * locals.var_pc_t) - (locals.var_dv0__blk162 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((locals.var_dv0__blk162_dn4 * locals.var_pc_t) - (locals.var_dv0__blk162 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)) / assign5360_e5920))) },)
    } else {
        (locals.var_cmx__blk176, locals.var_cmx__blk176_dn4,)
    }
};
        locals.var_cmx__blk176 = assign5360_e5925;
        locals.var_cmx__blk176_dn4 = assign5360_e5925_d_n4;
        locals.var_cmx__blk176_rv = 0.0;

        let (assign5370_e5940, assign5370_e5940_d_n4, assign5370_e5940_d_n7, assign5370_e5940_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5370_e5932: f64 = (1.0 - locals.var_sel__blk174);
        let assign5370_e5934: f64 = (assign5370_e5932 * locals.var_crt__blk175);
        let assign5370_e5937: f64 = (locals.var_sel__blk174 * locals.var_cmx__blk176);
        let assign5370_e5938: f64 = (assign5370_e5934 + assign5370_e5937);
        (assign5370_e5938, ((((-locals.var_sel__blk174_dn4) * locals.var_crt__blk175) + (assign5370_e5932 * locals.var_crt__blk175_dn4)) + ((locals.var_sel__blk174_dn4 * locals.var_cmx__blk176) + (locals.var_sel__blk174 * locals.var_cmx__blk176_dn4))), (((-locals.var_sel__blk174_dn7) * locals.var_crt__blk175) + (locals.var_sel__blk174_dn7 * locals.var_cmx__blk176)), (((-locals.var_sel__blk174_dn10) * locals.var_crt__blk175) + (locals.var_sel__blk174_dn10 * locals.var_cmx__blk176)),)
    } else {
        (locals.var_cl__blk177, locals.var_cl__blk177_dn4, locals.var_cl__blk177_dn7, locals.var_cl__blk177_dn10,)
    }
};
        locals.var_cl__blk177 = assign5370_e5940;
        locals.var_cl__blk177_dn4 = assign5370_e5940_d_n4;
        locals.var_cl__blk177_dn7 = assign5370_e5940_d_n7;
        locals.var_cl__blk177_dn10 = assign5370_e5940_d_n10;
        locals.var_cl__blk177_rv = 0.0;

        let (assign5380_e5953, assign5380_e5953_d_n4, assign5380_e5953_d_n7, assign5380_e5953_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5380_e5947: f64 = (locals.var_vbep - locals.var_vl__blk173);
        let assign5380_e5949: f64 = (assign5380_e5947 + locals.var_vl0__blk169);
        let assign5380_e5951: f64 = (assign5380_e5949 * locals.var_cl__blk177);
        (assign5380_e5951, ((((-locals.var_vl__blk173_dn4) + locals.var_vl0__blk169_dn4) * locals.var_cl__blk177) + (assign5380_e5949 * locals.var_cl__blk177_dn4)), (((locals.var_vbep_dn7 - locals.var_vl__blk173_dn7) * locals.var_cl__blk177) + (assign5380_e5949 * locals.var_cl__blk177_dn7)), (((locals.var_vbep_dn10 - locals.var_vl__blk173_dn10) * locals.var_cl__blk177) + (assign5380_e5949 * locals.var_cl__blk177_dn10)),)
    } else {
        (locals.var_ql__blk178, locals.var_ql__blk178_dn4, locals.var_ql__blk178_dn7, locals.var_ql__blk178_dn10,)
    }
};
        locals.var_ql__blk178 = assign5380_e5953;
        locals.var_ql__blk178_dn4 = assign5380_e5953_d_n4;
        locals.var_ql__blk178_dn7 = assign5380_e5953_d_n7;
        locals.var_ql__blk178_dn10 = assign5380_e5953_d_n10;
        locals.var_ql__blk178_rv = 0.0;

        let (assign5390_e5964, assign5390_e5964_d_n4, assign5390_e5964_d_n7, assign5390_e5964_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5390_e5960: f64 = (locals.var_ql__blk178 + locals.var_qlo__blk165);
        let assign5390_e5962: f64 = (assign5390_e5960 - locals.var_qlo0__blk170);
        (assign5390_e5962, ((locals.var_ql__blk178_dn4 + locals.var_qlo__blk165_dn4) - locals.var_qlo0__blk170_dn4), (locals.var_ql__blk178_dn7 + locals.var_qlo__blk165_dn7), (locals.var_ql__blk178_dn10 + locals.var_qlo__blk165_dn10),)
    } else {
        (locals.var_qdbep, locals.var_qdbep_dn4, locals.var_qdbep_dn7, locals.var_qdbep_dn10,)
    }
};
        locals.var_qdbep = assign5390_e5964;
        locals.var_qdbep_dn4 = assign5390_e5964_d_n4;
        locals.var_qdbep_dn7 = assign5390_e5964_d_n7;
        locals.var_qdbep_dn10 = assign5390_e5964_d_n10;
        locals.var_qdbep_rv = 0.0;

        let (assign5400_e5981, assign5400_e5981_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5400_e5972: f64 = (locals.var_dv0__blk162 * locals.var_dv0__blk162);
        let assign5400_e5975: f64 = (4.0 * p.p44);
        let assign5400_e5977: f64 = (assign5400_e5975 * p.p44);
        let assign5400_e5978: f64 = (assign5400_e5972 + assign5400_e5977);
        let assign5400_e5979: f64 = (assign5400_e5978).sqrt();
        (assign5400_e5979, (((locals.var_dv0__blk162_dn4 * locals.var_dv0__blk162) + (locals.var_dv0__blk162 * locals.var_dv0__blk162_dn4)) / (2.0 * assign5400_e5979)),)
    } else {
        (locals.var_mv0__blk179, locals.var_mv0__blk179_dn4,)
    }
};
        locals.var_mv0__blk179 = assign5400_e5981;
        locals.var_mv0__blk179_dn4 = assign5400_e5981_d_n4;
        locals.var_mv0__blk179_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5410_e5994, assign5410_e5994_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5410_e5988: f64 = (-0.5);
        let assign5410_e5991: f64 = (locals.var_dv0__blk162 + locals.var_mv0__blk179);
        let assign5410_e5992: f64 = (assign5410_e5988 * assign5410_e5991);
        (assign5410_e5992, (assign5410_e5988 * (locals.var_dv0__blk162_dn4 + locals.var_mv0__blk179_dn4)),)
    } else {
        (locals.var_vl0__blk169, locals.var_vl0__blk169_dn4,)
    }
};
        locals.var_vl0__blk169 = assign5410_e5994;
        locals.var_vl0__blk169_dn4 = assign5410_e5994_d_n4;
        locals.var_vl0__blk169_rv = 0.0;

        let (assign5420_e6017, assign5420_e6017_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5420_e6001: f64 = (-locals.var_pc_t);
        let assign5420_e6005: f64 = (locals.var_vl0__blk169 / locals.var_pc_t);
        let assign5420_e6006: f64 = (1.0 - assign5420_e6005);
        let assign5420_e6009: f64 = (1.0 - p.p43);
        let assign5420_e6010: f64 = (assign5420_e6006).powf(assign5420_e6009);
        let assign5420_e6011: f64 = (assign5420_e6001 * assign5420_e6010);
        let assign5420_e6014: f64 = (1.0 - p.p43);
        let assign5420_e6015: f64 = (assign5420_e6011 / assign5420_e6014);
        (assign5420_e6015, ((((-locals.var_pc_t_dn4) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((locals.var_vl0__blk169_dn4 * locals.var_pc_t) - (locals.var_vl0__blk169 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((locals.var_vl0__blk169_dn4 * locals.var_pc_t) - (locals.var_vl0__blk169 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014),)
    } else {
        (locals.var_q0__blk180, locals.var_q0__blk180_dn4,)
    }
};
        locals.var_q0__blk180 = assign5420_e6017;
        locals.var_q0__blk180_dn4 = assign5420_e6017_d_n4;
        locals.var_q0__blk180_rv = 0.0;

        let (assign5430_e6027, assign5430_e6027_d_n4, assign5430_e6027_d_n7, assign5430_e6027_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5430_e6025: f64 = (locals.var_vbep + locals.var_dv0__blk162);
        (assign5430_e6025, locals.var_dv0__blk162_dn4, locals.var_vbep_dn7, locals.var_vbep_dn10,)
    } else {
        (locals.var_dv__blk181, locals.var_dv__blk181_dn4, locals.var_dv__blk181_dn7, locals.var_dv__blk181_dn10,)
    }
};
        locals.var_dv__blk181 = assign5430_e6027;
        locals.var_dv__blk181_dn4 = assign5430_e6027_d_n4;
        locals.var_dv__blk181_dn7 = assign5430_e6027_d_n7;
        locals.var_dv__blk181_dn10 = assign5430_e6027_d_n10;
        locals.var_dv__blk181_rv = 0.0;

        let (assign5440_e6044, assign5440_e6044_d_n4, assign5440_e6044_d_n7, assign5440_e6044_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5440_e6035: f64 = (locals.var_dv__blk181 * locals.var_dv__blk181);
        let assign5440_e6038: f64 = (4.0 * p.p44);
        let assign5440_e6040: f64 = (assign5440_e6038 * p.p44);
        let assign5440_e6041: f64 = (assign5440_e6035 + assign5440_e6040);
        let assign5440_e6042: f64 = (assign5440_e6041).sqrt();
        (assign5440_e6042, (((locals.var_dv__blk181_dn4 * locals.var_dv__blk181) + (locals.var_dv__blk181 * locals.var_dv__blk181_dn4)) / (2.0 * assign5440_e6042)), (((locals.var_dv__blk181_dn7 * locals.var_dv__blk181) + (locals.var_dv__blk181 * locals.var_dv__blk181_dn7)) / (2.0 * assign5440_e6042)), (((locals.var_dv__blk181_dn10 * locals.var_dv__blk181) + (locals.var_dv__blk181 * locals.var_dv__blk181_dn10)) / (2.0 * assign5440_e6042)),)
    } else {
        (locals.var_mv__blk182, locals.var_mv__blk182_dn4, locals.var_mv__blk182_dn7, locals.var_mv__blk182_dn10,)
    }
};
        locals.var_mv__blk182 = assign5440_e6044;
        locals.var_mv__blk182_dn4 = assign5440_e6044_d_n4;
        locals.var_mv__blk182_dn7 = assign5440_e6044_d_n7;
        locals.var_mv__blk182_dn10 = assign5440_e6044_d_n10;
        locals.var_mv__blk182_rv = 0.0;

        let (assign5450_e6058, assign5450_e6058_d_n4, assign5450_e6058_d_n7, assign5450_e6058_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5450_e6053: f64 = (locals.var_dv__blk181 - locals.var_mv__blk182);
        let assign5450_e6054: f64 = (0.5 * assign5450_e6053);
        let assign5450_e6056: f64 = (assign5450_e6054 - locals.var_dv0__blk162);
        (assign5450_e6056, ((0.5 * (locals.var_dv__blk181_dn4 - locals.var_mv__blk182_dn4)) - locals.var_dv0__blk162_dn4), (0.5 * (locals.var_dv__blk181_dn7 - locals.var_mv__blk182_dn7)), (0.5 * (locals.var_dv__blk181_dn10 - locals.var_mv__blk182_dn10)),)
    } else {
        (locals.var_vl__blk173, locals.var_vl__blk173_dn4, locals.var_vl__blk173_dn7, locals.var_vl__blk173_dn10,)
    }
};
        locals.var_vl__blk173 = assign5450_e6058;
        locals.var_vl__blk173_dn4 = assign5450_e6058_d_n4;
        locals.var_vl__blk173_dn7 = assign5450_e6058_d_n7;
        locals.var_vl__blk173_dn10 = assign5450_e6058_d_n10;
        locals.var_vl__blk173_rv = 0.0;

        let (assign5460_e6081, assign5460_e6081_d_n4, assign5460_e6081_d_n7, assign5460_e6081_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5460_e6065: f64 = (-locals.var_pc_t);
        let assign5460_e6069: f64 = (locals.var_vl__blk173 / locals.var_pc_t);
        let assign5460_e6070: f64 = (1.0 - assign5460_e6069);
        let assign5460_e6073: f64 = (1.0 - p.p43);
        let assign5460_e6074: f64 = (assign5460_e6070).powf(assign5460_e6073);
        let assign5460_e6075: f64 = (assign5460_e6065 * assign5460_e6074);
        let assign5460_e6078: f64 = (1.0 - p.p43);
        let assign5460_e6079: f64 = (assign5460_e6075 / assign5460_e6078);
        (assign5460_e6079, ((((-locals.var_pc_t_dn4) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((locals.var_vl__blk173_dn4 * locals.var_pc_t) - (locals.var_vl__blk173 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((locals.var_vl__blk173_dn4 * locals.var_pc_t) - (locals.var_vl__blk173 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(locals.var_vl__blk173_dn7 / locals.var_pc_t)))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(locals.var_vl__blk173_dn7 / locals.var_pc_t)) / assign5460_e6070))) }) / assign5460_e6078), ((assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(locals.var_vl__blk173_dn10 / locals.var_pc_t)))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(locals.var_vl__blk173_dn10 / locals.var_pc_t)) / assign5460_e6070))) }) / assign5460_e6078),)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5460_e6081;
        locals.var_qlo__blk165_dn4 = assign5460_e6081_d_n4;
        locals.var_qlo__blk165_dn7 = assign5460_e6081_d_n7;
        locals.var_qlo__blk165_dn10 = assign5460_e6081_d_n10;
        locals.var_qlo__blk165_rv = 0.0;

        let (assign5470_e6104, assign5470_e6104_d_n4, assign5470_e6104_d_n7, assign5470_e6104_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5470_e6090: f64 = (1.0 - p.p34);
        let assign5470_e6092: f64 = (-p.p43);
        let assign5470_e6093: f64 = (assign5470_e6090).powf(assign5470_e6092);
        let assign5470_e6096: f64 = (locals.var_vbep - locals.var_vl__blk173);
        let assign5470_e6098: f64 = (assign5470_e6096 + locals.var_vl0__blk169);
        let assign5470_e6099: f64 = (assign5470_e6093 * assign5470_e6098);
        let assign5470_e6100: f64 = (locals.var_qlo__blk165 + assign5470_e6099);
        let assign5470_e6102: f64 = (assign5470_e6100 - locals.var_q0__blk180);
        (assign5470_e6102, ((locals.var_qlo__blk165_dn4 + (assign5470_e6093 * ((-locals.var_vl__blk173_dn4) + locals.var_vl0__blk169_dn4))) - locals.var_q0__blk180_dn4), (locals.var_qlo__blk165_dn7 + (assign5470_e6093 * (locals.var_vbep_dn7 - locals.var_vl__blk173_dn7))), (locals.var_qlo__blk165_dn10 + (assign5470_e6093 * (locals.var_vbep_dn10 - locals.var_vl__blk173_dn10))),)
    } else {
        (locals.var_qdbep, locals.var_qdbep_dn4, locals.var_qdbep_dn7, locals.var_qdbep_dn10,)
    }
};
        locals.var_qdbep = assign5470_e6104;
        locals.var_qdbep_dn4 = assign5470_e6104_d_n4;
        locals.var_qdbep_dn7 = assign5470_e6104_d_n7;
        locals.var_qdbep_dn10 = assign5470_e6104_d_n10;
        locals.var_qdbep_rv = 0.0;

        let (assign5480_e6110,) = {
    if (locals.var_ifi > 0.0) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        locals.var_sgif = assign5480_e6110;
        locals.var_sgif_rv = 0.0;

        let assign5490_e6113: f64 = (locals.var_ifi * locals.var_sgif);
        let assign5490_e6115: f64 = (assign5490_e6113 * locals.var_iitf);
        locals.var_rif = assign5490_e6115;
        locals.var_rif_dn4 = ((locals.var_ifi_dn4 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn5 = ((locals.var_ifi_dn5 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn6 = ((locals.var_ifi_dn6 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn7 = ((locals.var_ifi_dn7 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn8 = ((locals.var_ifi_dn8 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn9 = ((locals.var_ifi_dn9 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn10 = ((locals.var_ifi_dn10 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn11 = ((locals.var_ifi_dn11 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_rv = 0.0;

        let assign5500_e6119: f64 = (locals.var_rif + 1.0);
        let assign5500_e6120: f64 = (locals.var_rif / assign5500_e6119);
        locals.var_mif = assign5500_e6120;
        locals.var_mif_dn4 = (((locals.var_rif_dn4 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn4)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn5 = (((locals.var_rif_dn5 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn5)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn6 = (((locals.var_rif_dn6 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn6)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn7 = (((locals.var_rif_dn7 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn7)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn8 = (((locals.var_rif_dn8 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn8)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn9 = (((locals.var_rif_dn9 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn9)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn10 = (((locals.var_rif_dn10 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn10)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn11 = (((locals.var_rif_dn11 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn11)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_rv = 0.0;

        let assign5510_e6123: f64 = (locals.var_vbci * locals.var_ivtf);
        let assign5510_e6125: f64 = (assign5510_e6123 / 1.44);
        locals.var_arg = assign5510_e6125;
        locals.var_arg_dn4 = 0.0;
        locals.var_arg_dn5 = 0.0;
        locals.var_arg_dn6 = ((locals.var_vbci_dn6 * locals.var_ivtf) / 1.44);
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = ((locals.var_vbci_dn8 * locals.var_ivtf) / 1.44);
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;
        locals.var_arg_rv = 0.0;

        let assign5520_e6128: f64 = if locals.var_arg < locals.var_vmaxexp { 1.0 } else { 0.0 };
        locals.var_guard187 = assign5520_e6128;
        locals.var_guard187_rv = 0.0;

        let (assign5530_e6133, assign5530_e6133_d_n4, assign5530_e6133_d_n5, assign5530_e6133_d_n6, assign5530_e6133_d_n7, assign5530_e6133_d_n8, assign5530_e6133_d_n9, assign5530_e6133_d_n10, assign5530_e6133_d_n11,) = {
    if (locals.var_guard187 != 0.0) {
        let assign5530_e6131: f64 = (locals.var_arg).exp();
        (assign5530_e6131, (assign5530_e6131 * locals.var_arg_dn4), (assign5530_e6131 * locals.var_arg_dn5), (assign5530_e6131 * locals.var_arg_dn6), (assign5530_e6131 * locals.var_arg_dn7), (assign5530_e6131 * locals.var_arg_dn8), (assign5530_e6131 * locals.var_arg_dn9), (assign5530_e6131 * locals.var_arg_dn10), (assign5530_e6131 * locals.var_arg_dn11),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign5530_e6133;
        locals.var_expi_dn4 = assign5530_e6133_d_n4;
        locals.var_expi_dn5 = assign5530_e6133_d_n5;
        locals.var_expi_dn6 = assign5530_e6133_d_n6;
        locals.var_expi_dn7 = assign5530_e6133_d_n7;
        locals.var_expi_dn8 = assign5530_e6133_d_n8;
        locals.var_expi_dn9 = assign5530_e6133_d_n9;
        locals.var_expi_dn10 = assign5530_e6133_d_n10;
        locals.var_expi_dn11 = assign5530_e6133_d_n11;
        locals.var_expi_rv = 0.0;

        let (assign5540_e6145, assign5540_e6145_d_n4, assign5540_e6145_d_n5, assign5540_e6145_d_n6, assign5540_e6145_d_n7, assign5540_e6145_d_n8, assign5540_e6145_d_n9, assign5540_e6145_d_n10, assign5540_e6145_d_n11,) = {
    if (locals.var_guard187 == 0.0) {
        let assign5540_e6137: f64 = (locals.var_vmaxexp).exp();
        let assign5540_e6141: f64 = (locals.var_arg - locals.var_vmaxexp);
        let assign5540_e6142: f64 = (1.0 + assign5540_e6141);
        let assign5540_e6143: f64 = (assign5540_e6137 * assign5540_e6142);
        (assign5540_e6143, (assign5540_e6137 * locals.var_arg_dn4), (assign5540_e6137 * locals.var_arg_dn5), (assign5540_e6137 * locals.var_arg_dn6), (assign5540_e6137 * locals.var_arg_dn7), (assign5540_e6137 * locals.var_arg_dn8), (assign5540_e6137 * locals.var_arg_dn9), (assign5540_e6137 * locals.var_arg_dn10), (assign5540_e6137 * locals.var_arg_dn11),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign5540_e6145;
        locals.var_expi_dn4 = assign5540_e6145_d_n4;
        locals.var_expi_dn5 = assign5540_e6145_d_n5;
        locals.var_expi_dn6 = assign5540_e6145_d_n6;
        locals.var_expi_dn7 = assign5540_e6145_d_n7;
        locals.var_expi_dn8 = assign5540_e6145_d_n8;
        locals.var_expi_dn9 = assign5540_e6145_d_n9;
        locals.var_expi_dn10 = assign5540_e6145_d_n10;
        locals.var_expi_dn11 = assign5540_e6145_d_n11;
        locals.var_expi_rv = 0.0;

        let assign5550_e6150: f64 = (p.p77 * locals.var_q1);
        let assign5550_e6151: f64 = (1.0 + assign5550_e6150);
        let assign5550_e6152: f64 = (p.p76 * assign5550_e6151);
        let assign5550_e6156: f64 = (p.p78 * locals.var_expi);
        let assign5550_e6160: f64 = (locals.var_mif * locals.var_mif);
        let assign5550_e6161: f64 = (locals.var_sltf + assign5550_e6160);
        let assign5550_e6162: f64 = (assign5550_e6156 * assign5550_e6161);
        let assign5550_e6164: f64 = (assign5550_e6162 * locals.var_sgif);
        let assign5550_e6165: f64 = (1.0 + assign5550_e6164);
        let assign5550_e6166: f64 = (assign5550_e6152 * assign5550_e6165);
        locals.var_tff = assign5550_e6166;
        locals.var_tff_dn4 = (((p.p76 * (p.p77 * locals.var_q1_dn4)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn4) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn4 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn4)))) * locals.var_sgif)));
        locals.var_tff_dn5 = (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn5) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn5 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn5)))) * locals.var_sgif));
        locals.var_tff_dn6 = (((p.p76 * (p.p77 * locals.var_q1_dn6)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn6) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn6 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn6)))) * locals.var_sgif)));
        locals.var_tff_dn7 = (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn7) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn7 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn7)))) * locals.var_sgif));
        locals.var_tff_dn8 = (((p.p76 * (p.p77 * locals.var_q1_dn8)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn8) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn8 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn8)))) * locals.var_sgif)));
        locals.var_tff_dn9 = (((p.p76 * (p.p77 * locals.var_q1_dn9)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn9) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn9 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn9)))) * locals.var_sgif)));
        locals.var_tff_dn10 = (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn10) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn10 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn10)))) * locals.var_sgif));
        locals.var_tff_dn11 = (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn11) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn11 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn11)))) * locals.var_sgif));
        locals.var_tff_rv = 0.0;

        let assign5560_e6169: f64 = (locals.var_cje_t * locals.var_qdbe);
        let assign5560_e6171: f64 = (assign5560_e6169 * p.p55);
        let assign5560_e6174: f64 = (locals.var_tff * locals.var_ifi);
        let assign5560_e6176: f64 = (assign5560_e6174 / locals.var_qb);
        let assign5560_e6177: f64 = (assign5560_e6171 + assign5560_e6176);
        locals.var_qbe = assign5560_e6177;
        locals.var_qbe_dn4 = ((((locals.var_cje_t_dn4 * locals.var_qdbe) + (locals.var_cje_t * locals.var_qdbe_dn4)) * p.p55) + (((((locals.var_tff_dn4 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn4)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn4)) / (locals.var_qb * locals.var_qb)));
        locals.var_qbe_dn5 = (((((locals.var_tff_dn5 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn5)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn5)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_dn6 = (((((locals.var_tff_dn6 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn6)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn6)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_dn7 = (((((locals.var_tff_dn7 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn7)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn7)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_dn8 = (((locals.var_cje_t * locals.var_qdbe_dn8) * p.p55) + (((((locals.var_tff_dn8 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn8)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn8)) / (locals.var_qb * locals.var_qb)));
        locals.var_qbe_dn9 = (((locals.var_cje_t * locals.var_qdbe_dn9) * p.p55) + (((((locals.var_tff_dn9 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn9)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn9)) / (locals.var_qb * locals.var_qb)));
        locals.var_qbe_dn10 = (((((locals.var_tff_dn10 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn10)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn10)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_dn11 = (((((locals.var_tff_dn11 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn11)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn11)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_rv = 0.0;

        let assign5570_e6180: f64 = (locals.var_cje_t * locals.var_qdbex);
        let assign5570_e6183: f64 = (1.0 - p.p55);
        let assign5570_e6184: f64 = (assign5570_e6180 * assign5570_e6183);
        locals.var_qbex = assign5570_e6184;
        locals.var_qbex_dn4 = (((locals.var_cje_t_dn4 * locals.var_qdbex) + (locals.var_cje_t * locals.var_qdbex_dn4)) * assign5570_e6183);
        locals.var_qbex_dn7 = ((locals.var_cje_t * locals.var_qdbex_dn7) * assign5570_e6183);
        locals.var_qbex_dn9 = ((locals.var_cje_t * locals.var_qdbex_dn9) * assign5570_e6183);
        locals.var_qbex_rv = 0.0;

        let assign5580_e6187: f64 = (locals.var_cjc_t * locals.var_qdbc);
        let assign5580_e6190: f64 = (p.p81 * locals.var_iri);
        let assign5580_e6191: f64 = (assign5580_e6187 + assign5580_e6190);
        let assign5580_e6194: f64 = (p.p47 * locals.var_kbci);
        let assign5580_e6195: f64 = (assign5580_e6191 + assign5580_e6194);
        locals.var_qbc = assign5580_e6195;
        locals.var_qbc_dn4 = ((((locals.var_cjc_t_dn4 * locals.var_qdbc) + (locals.var_cjc_t * locals.var_qdbc_dn4)) + (p.p81 * locals.var_iri_dn4)) + (p.p47 * locals.var_kbci_dn4));
        locals.var_qbc_dn5 = ((p.p81 * locals.var_iri_dn5) + (p.p47 * locals.var_kbci_dn5));
        locals.var_qbc_dn6 = (((locals.var_cjc_t * locals.var_qdbc_dn6) + (p.p81 * locals.var_iri_dn6)) + (p.p47 * locals.var_kbci_dn6));
        locals.var_qbc_dn7 = ((p.p81 * locals.var_iri_dn7) + (p.p47 * locals.var_kbci_dn7));
        locals.var_qbc_dn8 = (((locals.var_cjc_t * locals.var_qdbc_dn8) + (p.p81 * locals.var_iri_dn8)) + (p.p47 * locals.var_kbci_dn8));
        locals.var_qbc_dn9 = ((p.p81 * locals.var_iri_dn9) + (p.p47 * locals.var_kbci_dn9));
        locals.var_qbc_dn10 = ((p.p81 * locals.var_iri_dn10) + (p.p47 * locals.var_kbci_dn10));
        locals.var_qbc_dn11 = ((p.p81 * locals.var_iri_dn11) + (p.p47 * locals.var_kbci_dn11));
        locals.var_qbc_rv = 0.0;

        let assign5590_e6198: f64 = (p.p47 * locals.var_kbcx);
        locals.var_qbcx = assign5590_e6198;
        locals.var_qbcx_dn4 = (p.p47 * locals.var_kbcx_dn4);
        locals.var_qbcx_dn5 = (p.p47 * locals.var_kbcx_dn5);
        locals.var_qbcx_dn6 = (p.p47 * locals.var_kbcx_dn6);
        locals.var_qbcx_dn7 = (p.p47 * locals.var_kbcx_dn7);
        locals.var_qbcx_dn8 = (p.p47 * locals.var_kbcx_dn8);
        locals.var_qbcx_dn9 = (p.p47 * locals.var_kbcx_dn9);
        locals.var_qbcx_dn10 = (p.p47 * locals.var_kbcx_dn10);
        locals.var_qbcx_dn11 = (p.p47 * locals.var_kbcx_dn11);
        locals.var_qbcx_rv = 0.0;

        let assign5600_e6201: f64 = (locals.var_cjep_t * locals.var_qdbep);
        let assign5600_e6204: f64 = (p.p81 * locals.var_ifp);
        let assign5600_e6205: f64 = (assign5600_e6201 + assign5600_e6204);
        locals.var_qbep = assign5600_e6205;
        locals.var_qbep_dn4 = (((locals.var_cjep_t_dn4 * locals.var_qdbep) + (locals.var_cjep_t * locals.var_qdbep_dn4)) + (p.p81 * locals.var_ifp_dn4));
        locals.var_qbep_dn5 = (p.p81 * locals.var_ifp_dn5);
        locals.var_qbep_dn6 = (p.p81 * locals.var_ifp_dn6);
        locals.var_qbep_dn7 = ((locals.var_cjep_t * locals.var_qdbep_dn7) + (p.p81 * locals.var_ifp_dn7));
        locals.var_qbep_dn8 = (p.p81 * locals.var_ifp_dn8);
        locals.var_qbep_dn9 = (p.p81 * locals.var_ifp_dn9);
        locals.var_qbep_dn10 = ((locals.var_cjep_t * locals.var_qdbep_dn10) + (p.p81 * locals.var_ifp_dn10));
        locals.var_qbep_dn11 = (p.p81 * locals.var_ifp_dn11);
        locals.var_qbep_rv = 0.0;

        let assign5610_e6208: f64 = (locals.var_cjcp_t * locals.var_qdbcp);
        let assign5610_e6211: f64 = (p.p53 * locals.var_vbcp);
        let assign5610_e6212: f64 = (assign5610_e6208 + assign5610_e6211);
        locals.var_qbcp = assign5610_e6212;
        locals.var_qbcp_dn4 = ((locals.var_cjcp_t_dn4 * locals.var_qdbcp) + (locals.var_cjcp_t * locals.var_qdbcp_dn4));
        locals.var_qbcp_dn10 = ((locals.var_cjcp_t * locals.var_qdbcp_dn10) + (p.p53 * locals.var_vbcp_dn10));
        locals.var_qbcp_dn11 = ((locals.var_cjcp_t * locals.var_qdbcp_dn11) + (p.p53 * locals.var_vbcp_dn11));
        locals.var_qbcp_rv = 0.0;

        let assign5640_e6221: f64 = (locals.var_dt_et * p.p102);
        locals.var_qcth = assign5640_e6221;
        locals.var_qcth_dn4 = (locals.var_dt_et_dn4 * p.p102);
        locals.var_qcth_rv = 0.0;

        let assign5670_e6232: f64 = locals.var_vbictype;
        let assign5670_e6234: f64 = (assign5670_e6232 * locals.var_qbe);
        locals.var_qbe = assign5670_e6234;
        locals.var_qbe_dn4 = (assign5670_e6232 * locals.var_qbe_dn4);
        locals.var_qbe_dn5 = (assign5670_e6232 * locals.var_qbe_dn5);
        locals.var_qbe_dn6 = (assign5670_e6232 * locals.var_qbe_dn6);
        locals.var_qbe_dn7 = (assign5670_e6232 * locals.var_qbe_dn7);
        locals.var_qbe_dn8 = (assign5670_e6232 * locals.var_qbe_dn8);
        locals.var_qbe_dn9 = (assign5670_e6232 * locals.var_qbe_dn9);
        locals.var_qbe_dn10 = (assign5670_e6232 * locals.var_qbe_dn10);
        locals.var_qbe_dn11 = (assign5670_e6232 * locals.var_qbe_dn11);
        locals.var_qbe_rv = 0.0;

        let assign5680_e6237: f64 = locals.var_vbictype;
        let assign5680_e6239: f64 = (assign5680_e6237 * locals.var_qbex);
        locals.var_qbex = assign5680_e6239;
        locals.var_qbex_dn4 = (assign5680_e6237 * locals.var_qbex_dn4);
        locals.var_qbex_dn7 = (assign5680_e6237 * locals.var_qbex_dn7);
        locals.var_qbex_dn9 = (assign5680_e6237 * locals.var_qbex_dn9);
        locals.var_qbex_rv = 0.0;

        let assign5690_e6242: f64 = locals.var_vbictype;
        let assign5690_e6244: f64 = (assign5690_e6242 * locals.var_qbc);
        locals.var_qbc = assign5690_e6244;
        locals.var_qbc_dn4 = (assign5690_e6242 * locals.var_qbc_dn4);
        locals.var_qbc_dn5 = (assign5690_e6242 * locals.var_qbc_dn5);
        locals.var_qbc_dn6 = (assign5690_e6242 * locals.var_qbc_dn6);
        locals.var_qbc_dn7 = (assign5690_e6242 * locals.var_qbc_dn7);
        locals.var_qbc_dn8 = (assign5690_e6242 * locals.var_qbc_dn8);
        locals.var_qbc_dn9 = (assign5690_e6242 * locals.var_qbc_dn9);
        locals.var_qbc_dn10 = (assign5690_e6242 * locals.var_qbc_dn10);
        locals.var_qbc_dn11 = (assign5690_e6242 * locals.var_qbc_dn11);
        locals.var_qbc_rv = 0.0;

        let assign5700_e6247: f64 = locals.var_vbictype;
        let assign5700_e6249: f64 = (assign5700_e6247 * locals.var_qbcx);
        locals.var_qbcx = assign5700_e6249;
        locals.var_qbcx_dn4 = (assign5700_e6247 * locals.var_qbcx_dn4);
        locals.var_qbcx_dn5 = (assign5700_e6247 * locals.var_qbcx_dn5);
        locals.var_qbcx_dn6 = (assign5700_e6247 * locals.var_qbcx_dn6);
        locals.var_qbcx_dn7 = (assign5700_e6247 * locals.var_qbcx_dn7);
        locals.var_qbcx_dn8 = (assign5700_e6247 * locals.var_qbcx_dn8);
        locals.var_qbcx_dn9 = (assign5700_e6247 * locals.var_qbcx_dn9);
        locals.var_qbcx_dn10 = (assign5700_e6247 * locals.var_qbcx_dn10);
        locals.var_qbcx_dn11 = (assign5700_e6247 * locals.var_qbcx_dn11);
        locals.var_qbcx_rv = 0.0;

        let assign5710_e6252: f64 = locals.var_vbictype;
        let assign5710_e6254: f64 = (assign5710_e6252 * locals.var_qbep);
        locals.var_qbep = assign5710_e6254;
        locals.var_qbep_dn4 = (assign5710_e6252 * locals.var_qbep_dn4);
        locals.var_qbep_dn5 = (assign5710_e6252 * locals.var_qbep_dn5);
        locals.var_qbep_dn6 = (assign5710_e6252 * locals.var_qbep_dn6);
        locals.var_qbep_dn7 = (assign5710_e6252 * locals.var_qbep_dn7);
        locals.var_qbep_dn8 = (assign5710_e6252 * locals.var_qbep_dn8);
        locals.var_qbep_dn9 = (assign5710_e6252 * locals.var_qbep_dn9);
        locals.var_qbep_dn10 = (assign5710_e6252 * locals.var_qbep_dn10);
        locals.var_qbep_dn11 = (assign5710_e6252 * locals.var_qbep_dn11);
        locals.var_qbep_rv = 0.0;

        let assign5740_e6263: f64 = locals.var_vbictype;
        let assign5740_e6265: f64 = (assign5740_e6263 * locals.var_qbcp);
        locals.var_qbcp = assign5740_e6265;
        locals.var_qbcp_dn4 = (assign5740_e6263 * locals.var_qbcp_dn4);
        locals.var_qbcp_dn10 = (assign5740_e6263 * locals.var_qbcp_dn10);
        locals.var_qbcp_dn11 = (assign5740_e6263 * locals.var_qbcp_dn11);
        locals.var_qbcp_rv = 0.0;

        let assign5750_e6268: f64 = locals.var_qcth;
        locals.var_qcth = assign5750_e6268;
        locals.var_qcth_dn4 = locals.var_qcth_dn4;
        locals.var_qcth_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let eq0_value: f64 = locals.var_ibe;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_ibe_dn4), multiplicity * (locals.var_ibe_dn5), multiplicity * (locals.var_ibe_dn6), multiplicity * (locals.var_ibe_dn7), multiplicity * (locals.var_ibe_dn8), multiplicity * (locals.var_ibe_dn9), multiplicity * (locals.var_ibe_dn10), multiplicity * (locals.var_ibe_dn11)],
            [],
            [],
            1.0,
        );
        let eq1_value: f64 = locals.var_ibex;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq1_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_ibex_dn4), multiplicity * (locals.var_ibex_dn5), multiplicity * (locals.var_ibex_dn6), multiplicity * (locals.var_ibex_dn7), multiplicity * (locals.var_ibex_dn8), multiplicity * (locals.var_ibex_dn9), multiplicity * (locals.var_ibex_dn10), multiplicity * (locals.var_ibex_dn11)],
            [],
            [],
            1.0,
        );
        let eq3_value: f64 = locals.var_itzr;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_itzr_dn4), multiplicity * (locals.var_itzr_dn5), multiplicity * (locals.var_itzr_dn6), multiplicity * (locals.var_itzr_dn7), multiplicity * (locals.var_itzr_dn8), multiplicity * (locals.var_itzr_dn9), multiplicity * (locals.var_itzr_dn10), multiplicity * (locals.var_itzr_dn11)],
            [],
            [],
            1.0,
        );
        let eq4_value: f64 = locals.var_ibc;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq4_value),
            [4, 5, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (locals.var_ibc_dn4), multiplicity * (locals.var_ibc_dn5), multiplicity * (locals.var_ibc_dn6), multiplicity * (locals.var_ibc_dn7), multiplicity * (locals.var_ibc_dn8), multiplicity * (locals.var_ibc_dn9), multiplicity * (locals.var_ibc_dn10), multiplicity * (locals.var_ibc_dn11), multiplicity * (locals.var_ibc_dn13)],
            [],
            [],
            1.0,
        );
        let eq5_value: f64 = locals.var_igcx;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq5_value),
            [0, 4, 5, 6, 7, 8],
            [multiplicity * (locals.var_igcx_dn0), multiplicity * (locals.var_igcx_dn4), multiplicity * (locals.var_igcx_dn5), multiplicity * (locals.var_igcx_dn6), multiplicity * (locals.var_igcx_dn7), multiplicity * (locals.var_igcx_dn8)],
            [],
            [],
            1.0,
        );
        let eq6_value: f64 = locals.var_ibep;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq6_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_ibep_dn4), multiplicity * (locals.var_ibep_dn5), multiplicity * (locals.var_ibep_dn6), multiplicity * (locals.var_ibep_dn7), multiplicity * (locals.var_ibep_dn8), multiplicity * (locals.var_ibep_dn9), multiplicity * (locals.var_ibep_dn10), multiplicity * (locals.var_ibep_dn11)],
            [],
            [],
            1.0,
        );
        let eq7_value: f64 = locals.var_ircx;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (eq7_value),
            0,
            multiplicity * (locals.var_ircx_dn0),
            4,
            multiplicity * (locals.var_ircx_dn4),
            5,
            multiplicity * (locals.var_ircx_dn5),
        );
        let eq8_value: f64 = locals.var_irci;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq8_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_irci_dn4), multiplicity * (locals.var_irci_dn5), multiplicity * (locals.var_irci_dn6), multiplicity * (locals.var_irci_dn7), multiplicity * (locals.var_irci_dn8), multiplicity * (locals.var_irci_dn9), multiplicity * (locals.var_irci_dn10), multiplicity * (locals.var_irci_dn11)],
            [],
            [],
            1.0,
        );
        let eq9_value: f64 = locals.var_irbx;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (eq9_value),
            1,
            multiplicity * (locals.var_irbx_dn1),
            4,
            multiplicity * (locals.var_irbx_dn4),
            7,
            multiplicity * (locals.var_irbx_dn7),
        );
        let eq10_value: f64 = locals.var_irbi;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq10_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_irbi_dn4), multiplicity * (locals.var_irbi_dn5), multiplicity * (locals.var_irbi_dn6), multiplicity * (locals.var_irbi_dn7), multiplicity * (locals.var_irbi_dn8), multiplicity * (locals.var_irbi_dn9), multiplicity * (locals.var_irbi_dn10), multiplicity * (locals.var_irbi_dn11)],
            [],
            [],
            1.0,
        );
        let eq11_value: f64 = locals.var_ire;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (eq11_value),
            2,
            multiplicity * (locals.var_ire_dn2),
            4,
            multiplicity * (locals.var_ire_dn4),
            9,
            multiplicity * (locals.var_ire_dn9),
        );
        let eq12_value: f64 = locals.var_irbp;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(10),
            Some(5),
            multiplicity * (eq12_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_irbp_dn4), multiplicity * (locals.var_irbp_dn5), multiplicity * (locals.var_irbp_dn6), multiplicity * (locals.var_irbp_dn7), multiplicity * (locals.var_irbp_dn8), multiplicity * (locals.var_irbp_dn9), multiplicity * (locals.var_irbp_dn10), multiplicity * (locals.var_irbp_dn11)],
            [],
            [],
            1.0,
        );
        let eq13_value: f64 = locals.var_ibcp;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(10),
            multiplicity * (eq13_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_ibcp_dn4), multiplicity * (locals.var_ibcp_dn5), multiplicity * (locals.var_ibcp_dn6), multiplicity * (locals.var_ibcp_dn7), multiplicity * (locals.var_ibcp_dn8), multiplicity * (locals.var_ibcp_dn9), multiplicity * (locals.var_ibcp_dn10), multiplicity * (locals.var_ibcp_dn11)],
            [],
            [],
            1.0,
        );
        let eq14_value: f64 = locals.var_iccp;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(11),
            multiplicity * (eq14_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (locals.var_iccp_dn4), multiplicity * (locals.var_iccp_dn5), multiplicity * (locals.var_iccp_dn6), multiplicity * (locals.var_iccp_dn7), multiplicity * (locals.var_iccp_dn8), multiplicity * (locals.var_iccp_dn9), multiplicity * (locals.var_iccp_dn10), multiplicity * (locals.var_iccp_dn11)],
            [],
            [],
            1.0,
        );
        let eq15_value: f64 = locals.var_irs;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (eq15_value),
            3,
            multiplicity * (locals.var_irs_dn3),
            4,
            multiplicity * (locals.var_irs_dn4),
            11,
            multiplicity * (locals.var_irs_dn11),
        );
        let eq16_value: f64 = locals.var_ixf1;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(12),
            None,
            multiplicity * (eq16_value),
            [4, 5, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (locals.var_ixf1_dn4), multiplicity * (locals.var_ixf1_dn5), multiplicity * (locals.var_ixf1_dn6), multiplicity * (locals.var_ixf1_dn7), multiplicity * (locals.var_ixf1_dn8), multiplicity * (locals.var_ixf1_dn9), multiplicity * (locals.var_ixf1_dn10), multiplicity * (locals.var_ixf1_dn11), multiplicity * (locals.var_ixf1_dn13)],
            [],
            [],
            1.0,
        );
        let eq18_value: f64 = locals.var_irth;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq18_value),
            4,
            multiplicity * (locals.var_irth_dn4),
        );
        let eq19_value: f64 = locals.var_ith;
        let eq19_node_derivative_indices: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13];
        let eq19_node_derivatives: [f64; 13] = [locals.var_ith_dn0, locals.var_ith_dn1, locals.var_ith_dn2, locals.var_ith_dn3, locals.var_ith_dn4, locals.var_ith_dn5, locals.var_ith_dn6, locals.var_ith_dn7, locals.var_ith_dn8, locals.var_ith_dn9, locals.var_ith_dn10, locals.var_ith_dn11, locals.var_ith_dn13];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e159: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qbe);
        let eq20_value: f64 = eq20_e159;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq20_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((locals.var_qbe_dn4 * ddt_scale)), multiplicity * ((locals.var_qbe_dn5 * ddt_scale)), multiplicity * ((locals.var_qbe_dn6 * ddt_scale)), multiplicity * ((locals.var_qbe_dn7 * ddt_scale)), multiplicity * ((locals.var_qbe_dn8 * ddt_scale)), multiplicity * ((locals.var_qbe_dn9 * ddt_scale)), multiplicity * ((locals.var_qbe_dn10 * ddt_scale)), multiplicity * ((locals.var_qbe_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq21_e161: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qbex);
        let eq21_value: f64 = eq21_e161;
        stamper.stamp_current_node3_local(
            Some(7),
            Some(9),
            multiplicity * (eq21_value),
            4,
            multiplicity * ((locals.var_qbex_dn4 * ddt_scale)),
            7,
            multiplicity * ((locals.var_qbex_dn7 * ddt_scale)),
            9,
            multiplicity * ((locals.var_qbex_dn9 * ddt_scale)),
        );
        let eq22_e163: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qbc);
        let eq22_value: f64 = eq22_e163;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq22_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((locals.var_qbc_dn4 * ddt_scale)), multiplicity * ((locals.var_qbc_dn5 * ddt_scale)), multiplicity * ((locals.var_qbc_dn6 * ddt_scale)), multiplicity * ((locals.var_qbc_dn7 * ddt_scale)), multiplicity * ((locals.var_qbc_dn8 * ddt_scale)), multiplicity * ((locals.var_qbc_dn9 * ddt_scale)), multiplicity * ((locals.var_qbc_dn10 * ddt_scale)), multiplicity * ((locals.var_qbc_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq23_e165: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qbcx);
        let eq23_value: f64 = eq23_e165;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq23_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((locals.var_qbcx_dn4 * ddt_scale)), multiplicity * ((locals.var_qbcx_dn5 * ddt_scale)), multiplicity * ((locals.var_qbcx_dn6 * ddt_scale)), multiplicity * ((locals.var_qbcx_dn7 * ddt_scale)), multiplicity * ((locals.var_qbcx_dn8 * ddt_scale)), multiplicity * ((locals.var_qbcx_dn9 * ddt_scale)), multiplicity * ((locals.var_qbcx_dn10 * ddt_scale)), multiplicity * ((locals.var_qbcx_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq24_e167: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qbep);
        let eq24_value: f64 = eq24_e167;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq24_value),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((locals.var_qbep_dn4 * ddt_scale)), multiplicity * ((locals.var_qbep_dn5 * ddt_scale)), multiplicity * ((locals.var_qbep_dn6 * ddt_scale)), multiplicity * ((locals.var_qbep_dn7 * ddt_scale)), multiplicity * ((locals.var_qbep_dn8 * ddt_scale)), multiplicity * ((locals.var_qbep_dn9 * ddt_scale)), multiplicity * ((locals.var_qbep_dn10 * ddt_scale)), multiplicity * ((locals.var_qbep_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq27_e173: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbcp);
        let eq27_value: f64 = eq27_e173;
        stamper.stamp_current_node3_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            4,
            multiplicity * ((locals.var_qbcp_dn4 * ddt_scale)),
            10,
            multiplicity * ((locals.var_qbcp_dn10 * ddt_scale)),
            11,
            multiplicity * ((locals.var_qbcp_dn11 * ddt_scale)),
        );
        let eq30_e179: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, locals.var_qcth);
        let eq30_value: f64 = eq30_e179;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq30_value),
            4,
            multiplicity * ((locals.var_qcth_dn4 * ddt_scale)),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let eq20_e159_q: f64 = locals.var_qbe;
        let eq20_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10, locals.var_qbe_dn11, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq21_e161_q: f64 = locals.var_qbex;
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes[4],
            multiplicity * (locals.var_qbex_dn4),
            nodes[7],
            multiplicity * (locals.var_qbex_dn7),
            nodes[9],
            multiplicity * (locals.var_qbex_dn9),
        );
        let eq22_e163_q: f64 = locals.var_qbc;
        let eq22_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10, locals.var_qbc_dn11, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq23_e165_q: f64 = locals.var_qbcx;
        let eq23_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, locals.var_qbcx_dn4, locals.var_qbcx_dn5, locals.var_qbcx_dn6, locals.var_qbcx_dn7, locals.var_qbcx_dn8, locals.var_qbcx_dn9, locals.var_qbcx_dn10, locals.var_qbcx_dn11, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq24_e167_q: f64 = locals.var_qbep;
        let eq24_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, locals.var_qbep_dn4, locals.var_qbep_dn5, locals.var_qbep_dn6, locals.var_qbep_dn7, locals.var_qbep_dn8, locals.var_qbep_dn9, locals.var_qbep_dn10, locals.var_qbep_dn11, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq27_e173_q: f64 = locals.var_qbcp;
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[10]),
            nodes[4],
            multiplicity * (locals.var_qbcp_dn4),
            nodes[10],
            multiplicity * (locals.var_qbcp_dn10),
            nodes[11],
            multiplicity * (locals.var_qbcp_dn11),
        );
        let eq30_e179_q: f64 = locals.var_qcth;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (locals.var_qcth_dn4),
        );
    }
}
